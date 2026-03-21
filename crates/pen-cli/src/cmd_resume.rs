use crate::cli::ResumeArgs;
use crate::cmd_run::{
    build_run_manifest, current_search_compat, frontier_runtime_limits, now_utc,
    resolved_worker_count, terminal_narrative_config, write_run_artifacts,
};
use crate::output::{OutputStyle, render_run_output};
use crate::progress::TerminalStepProgress;
use crate::report::{
    GeneratedSteps, StepGenerationMode, StepProgressObserver, StepProvenance, StepReport,
    annotate_search_profile, extend_steps_from_reports_with_config_and_runtime_and_progress,
    load_step_reports, reevaluate_steps_from_reports_with_config_and_runtime_and_progress,
};
use anyhow::{Context, Result, bail};
use pen_search::config::RuntimeConfig;
use pen_search::frontier::FrontierWindow;
use pen_search::resume::{
    ResumeDecision, checkpoint_compat_from_resume, decide_resume, decide_step_resume,
};
use pen_search::scheduler::build_schedule;
use pen_search::state::FrontierStateRecV1;
use pen_store::checkpoint::{read_step_checkpoint, step_checkpoint_path};
use pen_store::frontier::{
    FrontierGeneration, base_step_checkpoint_path, load_latest_frontier_generation,
};
use pen_store::manifest::RunManifestV1;
use std::fs;
use std::path::{Path, PathBuf};

pub fn resume(args: ResumeArgs) -> Result<String> {
    let run_dir = absolute(args.run_dir)?;
    let manifest_path = run_dir.join("run.json");
    let run_text = fs::read_to_string(&manifest_path)
        .with_context(|| format!("read {}", manifest_path.display()))?;
    let manifest: RunManifestV1 = serde_json::from_str(&run_text).context("parse run manifest")?;

    let config_path = run_dir.join("config.toml");
    let config_text = fs::read_to_string(&config_path)
        .with_context(|| format!("read {}", config_path.display()))?;
    let config = RuntimeConfig::from_toml_str(&config_text).context("parse runtime config")?;
    let target = args.until_step.unwrap_or(config.search.until_step);

    if manifest.position.completed_step >= target {
        let mut steps = load_step_reports(&run_dir)?;
        annotate_search_profile(&mut steps, config.mode.search_profile);
        steps.truncate(target as usize);
        return Ok(render_run_output(
            OutputStyle::from_debug(args.debug),
            &manifest.run_id,
            &steps,
            terminal_narrative_config(&config, args.narrative),
        ));
    }

    let mut existing_steps = load_step_reports(&run_dir)?;
    annotate_search_profile(&mut existing_steps, config.mode.search_profile);
    let mut progress = TerminalStepProgress::stderr(target);
    let PreparedResume {
        generated: GeneratedSteps { mode, mut steps },
        worker_count,
    } = prepare_resume(
        &run_dir,
        &manifest,
        &config,
        target,
        &existing_steps,
        progress
            .as_mut()
            .map(|observer| observer as &mut dyn StepProgressObserver),
    )?;
    annotate_search_profile(&mut steps, config.mode.search_profile);
    let updated = now_utc()?;
    let new_manifest = build_run_manifest(
        &manifest.run_id,
        &config_text,
        &steps,
        &config,
        worker_count,
        manifest.created_utc,
        updated,
    )?;

    write_run_artifacts(
        &run_dir,
        &config_text,
        &new_manifest,
        &steps,
        worker_count,
        &config,
        mode.as_str(),
        config.mode.search_profile.as_str(),
    )?;
    Ok(render_run_output(
        OutputStyle::from_debug(args.debug),
        &manifest.run_id,
        &steps,
        terminal_narrative_config(&config, args.narrative),
    ))
}

struct PreparedResume {
    generated: GeneratedSteps,
    worker_count: u16,
}

fn prepare_resume(
    run_dir: &Path,
    manifest: &RunManifestV1,
    config: &RuntimeConfig,
    target: u32,
    existing_steps: &[StepReport],
    progress: Option<&mut dyn StepProgressObserver>,
) -> Result<PreparedResume> {
    let current_compat = current_search_compat();
    let latest_step_decision = latest_step_resume_decision(run_dir, manifest)?;
    let frontier = latest_frontier_generation(run_dir, manifest)?;
    let worker_count = resolved_worker_count(config);

    match frontier {
        Some(frontier) => match decide_resume(&current_compat, &frontier.manifest) {
            ResumeDecision::FrontierCheckpoint => {
                validate_frontier_checkpoint(run_dir, manifest, existing_steps, &frontier)?;
                let frontier_worker_count = frontier.manifest.scheduler.worker_count.max(1);
                let mut generated = match progress {
                    Some(observer) => {
                        extend_steps_from_reports_with_config_and_runtime_and_progress(
                            existing_steps,
                            target,
                            config,
                            frontier_runtime_limits(config, frontier_worker_count),
                            Some(observer),
                        )?
                    }
                    None => extend_steps_from_reports_with_config_and_runtime_and_progress(
                        existing_steps,
                        target,
                        config,
                        frontier_runtime_limits(config, frontier_worker_count),
                        None,
                    )?,
                };
                for step in generated.steps.iter_mut().skip(existing_steps.len()) {
                    step.provenance = StepProvenance::FrontierCheckpointResume;
                }
                generated.mode = StepGenerationMode::FrontierCheckpointResume;
                Ok(PreparedResume {
                    generated,
                    worker_count: frontier_worker_count,
                })
            }
            ResumeDecision::StepCheckpoint => prepare_step_resume(
                latest_step_decision,
                existing_steps,
                config,
                target,
                worker_count,
                progress,
            ),
            ResumeDecision::StepCheckpointReevaluate => match latest_step_decision {
                ResumeDecision::MigrationRequired => bail!(
                    "migration required: stored step checkpoint is not compatible with the current AST/type schema"
                ),
                ResumeDecision::FrontierCheckpoint
                | ResumeDecision::StepCheckpoint
                | ResumeDecision::StepCheckpointReevaluate => Ok(PreparedResume {
                    generated: match progress {
                        Some(observer) => {
                            reevaluate_steps_from_reports_with_config_and_runtime_and_progress(
                                existing_steps,
                                target,
                                config,
                                frontier_runtime_limits(config, worker_count),
                                Some(observer),
                            )?
                        }
                        None => reevaluate_steps_from_reports_with_config_and_runtime_and_progress(
                            existing_steps,
                            target,
                            config,
                            frontier_runtime_limits(config, worker_count),
                            None,
                        )?,
                    },
                    worker_count,
                }),
            },
            ResumeDecision::MigrationRequired => bail!(
                "migration required: stored frontier artifacts are not compatible with the current AST/type schema"
            ),
        },
        None => prepare_step_resume(
            latest_step_decision,
            existing_steps,
            config,
            target,
            worker_count,
            progress,
        ),
    }
}

fn prepare_step_resume(
    decision: ResumeDecision,
    existing_steps: &[StepReport],
    config: &RuntimeConfig,
    target: u32,
    worker_count: u16,
    progress: Option<&mut dyn StepProgressObserver>,
) -> Result<PreparedResume> {
    match decision {
        ResumeDecision::FrontierCheckpoint | ResumeDecision::StepCheckpoint => {
            let mut generated = extend_steps_from_reports_with_config_and_runtime_and_progress(
                existing_steps,
                target,
                config,
                frontier_runtime_limits(config, worker_count),
                progress,
            )?;
            generated.mode = StepGenerationMode::StepCheckpointResume;
            Ok(PreparedResume {
                generated,
                worker_count,
            })
        }
        ResumeDecision::StepCheckpointReevaluate => Ok(PreparedResume {
            generated: reevaluate_steps_from_reports_with_config_and_runtime_and_progress(
                existing_steps,
                target,
                config,
                frontier_runtime_limits(config, worker_count),
                progress,
            )?,
            worker_count,
        }),
        ResumeDecision::MigrationRequired => bail!(
            "migration required: stored step checkpoint is not compatible with the current AST/type schema"
        ),
    }
}

fn latest_step_resume_decision(run_dir: &Path, manifest: &RunManifestV1) -> Result<ResumeDecision> {
    if manifest.position.completed_step == 0 {
        return Ok(ResumeDecision::StepCheckpoint);
    }

    let step_path = step_checkpoint_path(run_dir, manifest.position.completed_step);
    let checkpoint = read_step_checkpoint(&step_path)?;
    if checkpoint.run_id != manifest.run_id {
        bail!(
            "step checkpoint {} belonged to run {}, expected {}",
            step_path.display(),
            checkpoint.run_id,
            manifest.run_id
        );
    }
    if checkpoint.step_index != manifest.position.completed_step {
        bail!(
            "step checkpoint {} stored step {}, expected {}",
            step_path.display(),
            checkpoint.step_index,
            manifest.position.completed_step
        );
    }

    Ok(decide_step_resume(
        &current_search_compat().checkpoint_compat(),
        &checkpoint.compat,
    ))
}

fn latest_frontier_generation(
    run_dir: &Path,
    manifest: &RunManifestV1,
) -> Result<Option<FrontierGeneration<{ FrontierStateRecV1::BYTE_LEN }>>> {
    if manifest.position.completed_step < 4 || manifest.position.frontier_epoch == 0 {
        return Ok(None);
    }

    load_latest_frontier_generation::<{ FrontierStateRecV1::BYTE_LEN }>(
        run_dir,
        manifest.position.completed_step,
        manifest.position.active_band,
    )
}

fn validate_frontier_checkpoint(
    run_dir: &Path,
    manifest: &RunManifestV1,
    existing_steps: &[StepReport],
    frontier: &FrontierGeneration<{ FrontierStateRecV1::BYTE_LEN }>,
) -> Result<()> {
    if frontier.manifest.run_id != manifest.run_id {
        bail!(
            "frontier manifest {} belonged to run {}, expected {}",
            frontier.frontier_dir.display(),
            frontier.manifest.run_id,
            manifest.run_id
        );
    }
    if frontier.manifest.step_index != manifest.position.completed_step {
        bail!(
            "frontier manifest {} stored step {}, expected {}",
            frontier.frontier_dir.display(),
            frontier.manifest.step_index,
            manifest.position.completed_step
        );
    }
    if frontier.manifest.band_index != manifest.position.active_band {
        bail!(
            "frontier manifest {} stored band {}, expected {}",
            frontier.frontier_dir.display(),
            frontier.manifest.band_index,
            manifest.position.active_band
        );
    }
    if frontier.manifest.frontier_epoch != manifest.position.frontier_epoch {
        bail!(
            "frontier manifest {} stored epoch {}, expected {}",
            frontier.frontier_dir.display(),
            frontier.manifest.frontier_epoch,
            manifest.position.frontier_epoch
        );
    }

    let Some(last_step) = existing_steps.last() else {
        bail!("frontier checkpoint existed without any stored step reports");
    };
    if last_step.step_index != frontier.manifest.step_index {
        bail!(
            "latest step report ended at {}, but frontier manifest ended at {}",
            last_step.step_index,
            frontier.manifest.step_index
        );
    }
    if u32::from(last_step.accepted.clause_kappa) != frontier.manifest.band_index {
        bail!(
            "latest step report ended at band {}, but frontier manifest stored band {}",
            last_step.accepted.clause_kappa,
            frontier.manifest.band_index
        );
    }

    let base_step_path = base_step_checkpoint_path(&frontier.frontier_dir, &frontier.manifest);
    let checkpoint = read_step_checkpoint(&base_step_path)?;
    if checkpoint.run_id != manifest.run_id
        || checkpoint.step_index != manifest.position.completed_step
    {
        bail!(
            "frontier base step checkpoint {} did not match run {} step {}",
            base_step_path.display(),
            manifest.run_id,
            manifest.position.completed_step
        );
    }
    let frontier_checkpoint_compat =
        checkpoint_compat_from_resume(&frontier.manifest.resume_compatible);
    if checkpoint.compat != frontier_checkpoint_compat {
        bail!(
            "frontier checkpoint {} and base step checkpoint {} disagreed about compatibility",
            frontier.frontier_dir.display(),
            base_step_path.display()
        );
    }

    let hot = frontier
        .hot_records
        .iter()
        .copied()
        .map(FrontierStateRecV1::from_le_bytes)
        .collect::<Vec<_>>();
    let cold = frontier
        .cold_records
        .iter()
        .copied()
        .map(FrontierStateRecV1::from_le_bytes)
        .collect::<Vec<_>>();
    let mut expected = FrontierWindow {
        hot: hot.clone(),
        cold: cold.clone(),
    };
    expected.compact_sorted();
    if expected.hot != hot || expected.cold != cold {
        bail!(
            "frontier checkpoint {} was not persisted in deterministic frontier order",
            frontier.frontier_dir.display()
        );
    }

    let expected_priority_heads =
        expected.priority_heads(frontier.manifest.scheduler.priority_heads.len());
    if frontier.manifest.scheduler.priority_heads != expected_priority_heads {
        bail!(
            "frontier checkpoint {} stored priority heads {:?}, expected {:?}",
            frontier.frontier_dir.display(),
            frontier.manifest.scheduler.priority_heads,
            expected_priority_heads
        );
    }

    let worker_count = frontier.manifest.scheduler.worker_count.max(1);
    let schedule = build_schedule(&expected, worker_count);
    if schedule.assignments.len() as u16 != worker_count {
        bail!(
            "frontier checkpoint {} stored worker count {}, but schedule rebuilt {} assignments",
            frontier.frontier_dir.display(),
            worker_count,
            schedule.assignments.len()
        );
    }
    if expected
        .hot
        .iter()
        .chain(expected.cold.iter())
        .any(|record| record.worker_hint >= worker_count)
    {
        bail!(
            "frontier checkpoint {} contained worker hints beyond worker count {}",
            frontier.frontier_dir.display(),
            worker_count
        );
    }

    let expected_step_path = step_checkpoint_path(run_dir, manifest.position.completed_step);
    if normalize_path(&expected_step_path)? != normalize_path(&base_step_path)? {
        bail!(
            "frontier checkpoint {} pointed at {}, expected {}",
            frontier.frontier_dir.display(),
            base_step_path.display(),
            expected_step_path.display()
        );
    }

    Ok(())
}

fn normalize_path(path: &Path) -> Result<PathBuf> {
    fs::canonicalize(path).with_context(|| format!("canonicalize {}", path.display()))
}

fn absolute(path: PathBuf) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

#[cfg(test)]
mod tests {
    use super::resume;
    use crate::cli::{ResumeArgs, RunArgs};
    use crate::cmd_run::run;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-cli-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    #[test]
    fn resume_command_extends_reference_replay() {
        let root = temp_dir("resume");
        run(RunArgs {
            config: "configs/debug.toml".into(),
            root: root.clone(),
            run_id: Some("resume-run".to_owned()),
            until_step: Some(2),
            debug: false,
            narrative: false,
        })
        .expect("initial run should succeed");

        let output = resume(ResumeArgs {
            run_dir: root.join("resume-run"),
            until_step: Some(4),
            debug: false,
            narrative: false,
        })
        .expect("resume should succeed");

        assert!(output.contains("completed_step: 4"));
        fs::remove_dir_all(root).ok();
    }
}
