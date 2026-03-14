use crate::cli::RunArgs;
use crate::output::{OutputStyle, render_run_output};
use crate::report::{GeneratedSteps, StepReport, generate_steps, write_step_reports};
use anyhow::{Context, Result, bail};
use pen_core::hash::blake3_hex;
use pen_core::library::{LibrarySnapshot, LibrarySnapshotEntry};
use pen_search::config::RuntimeConfig;
use pen_search::resume::CurrentCompat;
use pen_store::layout::{FRONTIER_RECORD_LAYOUT_ID, SCHEMA_VERSION_V1};
use pen_store::manifest::{
    BitBand, CheckpointCompat, ConfigFingerprint, HostInfo, NearMiss, RunArtifacts, RunCompat,
    RunManifestV1, RunPosition, RunStatus, StepCheckpointV1, StepObjective, StepStats,
};
use pen_store::telemetry::TelemetryEventV1;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

pub fn run(args: RunArgs) -> Result<String> {
    let config_path = absolute_from_repo(&args.config)?;
    let config_text = fs::read_to_string(&config_path)
        .with_context(|| format!("read {}", config_path.display()))?;
    let config = RuntimeConfig::from_toml_str(&config_text).context("parse runtime config")?;
    let run_id = args.run_id.unwrap_or(default_run_id()?);
    let run_dir = absolute_from_repo(&args.root)?.join(&run_id);
    if run_dir.exists() {
        bail!("run directory already exists: {}", run_dir.display());
    }

    let until_step = args.until_step.unwrap_or(config.search.until_step);
    let GeneratedSteps { mode, steps } = generate_steps(until_step, config.objective.window_depth)?;
    let created_utc = now_utc()?;
    let updated_utc = created_utc.clone();
    let manifest = build_run_manifest(&run_id, &config_text, &steps, created_utc, updated_utc);

    write_run_artifacts(
        &run_dir,
        &config_text,
        &manifest,
        &steps,
        config.objective.window_depth,
        mode.as_str(),
    )?;
    Ok(render_run_output(
        OutputStyle::from_debug(args.debug),
        &run_id,
        &steps,
    ))
}

pub(crate) fn current_run_compat() -> RunCompat {
    RunCompat {
        ast_schema_hash: tagged_hash("ast-schema-v1"),
        type_rules_hash: tagged_hash("type-rules-v1"),
        evaluator_hash: tagged_hash("evaluator-v1"),
        search_semantics_hash: tagged_hash("search-semantics-v1"),
        store_schema_hash: tagged_hash("store-schema-v1"),
    }
}

pub(crate) fn current_search_compat() -> CurrentCompat {
    CurrentCompat {
        ast_schema_hash: tagged_hash("ast-schema-v1"),
        type_rules_hash: tagged_hash("type-rules-v1"),
        evaluator_hash: tagged_hash("evaluator-v1"),
        search_semantics_hash: tagged_hash("search-semantics-v1"),
        record_layout_id: FRONTIER_RECORD_LAYOUT_ID.to_owned(),
    }
}

pub(crate) fn checkpoint_compat() -> CheckpointCompat {
    let compat = current_run_compat();
    CheckpointCompat {
        ast_schema_hash: compat.ast_schema_hash,
        type_rules_hash: compat.type_rules_hash,
        evaluator_hash: compat.evaluator_hash,
        search_semantics_hash: compat.search_semantics_hash,
    }
}

pub(crate) fn write_run_artifacts(
    run_dir: &Path,
    config_text: &str,
    manifest: &RunManifestV1,
    steps: &[StepReport],
    window_depth: u16,
    mode: &str,
) -> Result<()> {
    fs::create_dir_all(run_dir).with_context(|| format!("create {}", run_dir.display()))?;
    fs::create_dir_all(run_dir.join("reports").join("steps"))?;
    fs::create_dir_all(run_dir.join("checkpoints").join("steps"))?;
    fs::create_dir_all(run_dir.join("checkpoints").join("frontier"))?;

    fs::write(run_dir.join("config.toml"), config_text)
        .with_context(|| format!("write {}", run_dir.join("config.toml").display()))?;
    fs::write(run_dir.join("meta.sqlite3"), [])
        .with_context(|| format!("write {}", run_dir.join("meta.sqlite3").display()))?;

    write_json_file(&run_dir.join("run.json"), manifest)?;
    write_step_reports(run_dir, steps)?;
    write_step_checkpoints(run_dir, manifest, steps, window_depth)?;
    write_telemetry(run_dir, manifest, steps, mode)?;

    fs::write(
        run_dir.join("reports").join("latest.txt"),
        crate::report::render_standard_report(&manifest.run_id, steps),
    )?;
    fs::write(
        run_dir.join("reports").join("latest.debug.txt"),
        crate::report::render_debug_report(&manifest.run_id, steps),
    )?;

    Ok(())
}

pub(crate) fn build_run_manifest(
    run_id: &str,
    config_text: &str,
    steps: &[StepReport],
    created_utc: String,
    updated_utc: String,
) -> RunManifestV1 {
    let completed_step = steps.last().map(|step| step.step_index).unwrap_or(0);
    RunManifestV1 {
        schema_version: SCHEMA_VERSION_V1,
        run_id: run_id.to_owned(),
        status: RunStatus::Completed,
        created_utc,
        updated_utc,
        workspace_version: env!("CARGO_PKG_VERSION").to_owned(),
        compat: current_run_compat(),
        host: host_info(),
        config: ConfigFingerprint {
            path: "config.toml".to_owned(),
            sha256: sha256_hex(config_text.as_bytes()),
        },
        position: RunPosition {
            completed_step,
            active_step: completed_step.saturating_add(1),
            active_band: 0,
            frontier_epoch: 0,
        },
        artifacts: RunArtifacts {
            telemetry: "telemetry.ndjson".to_owned(),
            reports_dir: "reports".to_owned(),
            checkpoints_dir: "checkpoints".to_owned(),
        },
    }
}

fn write_step_checkpoints(
    run_dir: &Path,
    manifest: &RunManifestV1,
    steps: &[StepReport],
    window_depth: u16,
) -> Result<()> {
    for (index, step) in steps.iter().enumerate() {
        let checkpoint = StepCheckpointV1 {
            schema_version: SCHEMA_VERSION_V1,
            run_id: manifest.run_id.clone(),
            step_index: step.step_index,
            accepted_utc: manifest.updated_utc.clone(),
            compat: checkpoint_compat(),
            objective: StepObjective {
                bar: step.objective_bar,
                exact_clause_kappa: step.accepted.clause_kappa,
                bit_band: BitBand {
                    min: step.accepted.bit_kappa,
                    max: step.accepted.bit_kappa,
                },
            },
            accepted: step.accepted.clone(),
            library_snapshot: library_snapshot(&steps[..index], window_depth),
            near_misses: Vec::<NearMiss>::new(),
            stats: StepStats {
                frontier_scanned: 1,
                typed_prefixes: 1,
                sound_prunes: 0,
                heuristic_drops: 0,
            },
        };
        write_json_file(
            &run_dir
                .join("checkpoints")
                .join("steps")
                .join(format!("step-{:02}.json", step.step_index)),
            &checkpoint,
        )?;
    }
    Ok(())
}

fn library_snapshot(previous_steps: &[StepReport], window_depth: u16) -> LibrarySnapshot {
    let depth = usize::from(window_depth.max(1));
    let start = previous_steps.len().saturating_sub(depth);
    LibrarySnapshot {
        window_depth,
        entries: previous_steps[start..]
            .iter()
            .map(|step| LibrarySnapshotEntry {
                step: step.step_index,
                candidate_hash: step.accepted.candidate_hash.clone(),
                telescope: step.telescope.clone(),
            })
            .collect(),
    }
}

fn write_telemetry(
    run_dir: &Path,
    manifest: &RunManifestV1,
    steps: &[StepReport],
    mode: &str,
) -> Result<()> {
    let mut lines = Vec::new();
    lines.push(serde_json::to_string(&TelemetryEventV1 {
        schema_version: SCHEMA_VERSION_V1,
        run_id: manifest.run_id.clone(),
        event: "run_started".to_owned(),
        step_index: None,
        payload: json!({
            "created_utc": manifest.created_utc,
            "mode": mode,
        }),
    })?);

    for step in steps {
        lines.push(serde_json::to_string(&TelemetryEventV1 {
            schema_version: SCHEMA_VERSION_V1,
            run_id: manifest.run_id.clone(),
            event: "step_accepted".to_owned(),
            step_index: Some(step.step_index),
            payload: json!({
                "label": step.label,
                "nu": step.accepted.nu,
                "kappa": step.accepted.clause_kappa,
                "rho": step.accepted.rho.to_string(),
                "bar": step.objective_bar.to_string(),
                "candidate_hash": step.accepted.candidate_hash,
            }),
        })?);
    }

    lines.push(serde_json::to_string(&TelemetryEventV1 {
        schema_version: SCHEMA_VERSION_V1,
        run_id: manifest.run_id.clone(),
        event: "run_completed".to_owned(),
        step_index: manifest.position.completed_step.checked_add(1),
        payload: json!({
            "completed_step": manifest.position.completed_step,
            "status": "completed",
        }),
    })?);

    fs::write(
        run_dir.join("telemetry.ndjson"),
        format!("{}\n", lines.join("\n")),
    )?;
    Ok(())
}

fn write_json_file<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{json}\n")).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

fn absolute_from_repo(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("workspace root")
            .join(path))
    }
}

fn default_run_id() -> Result<String> {
    let timestamp = OffsetDateTime::now_utc().unix_timestamp();
    Ok(format!("reference-{timestamp}"))
}

pub(crate) fn now_utc() -> Result<String> {
    OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .context("format current UTC timestamp")
}

fn tagged_hash(tag: &str) -> String {
    format!("blake3:{}", blake3_hex(tag.as_bytes()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

fn host_info() -> HostInfo {
    let logical_cpus = std::thread::available_parallelism()
        .map(|count| count.get() as u32)
        .unwrap_or(1);
    HostInfo {
        os: std::env::consts::OS.to_owned(),
        arch: std::env::consts::ARCH.to_owned(),
        logical_cpus,
        ram_bytes: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::cli::RunArgs;
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
    fn run_command_writes_frozen_manifest_and_reports() {
        let root = temp_dir("run");
        let output = run(RunArgs {
            config: "configs/debug.toml".into(),
            root: root.clone(),
            run_id: Some("test-run".to_owned()),
            until_step: Some(3),
            debug: false,
        })
        .expect("run command should succeed");

        assert!(output.contains("completed_step: 3"));
        assert!(root.join("test-run").join("run.json").exists());
        assert!(
            root.join("test-run")
                .join("reports")
                .join("steps")
                .join("step-03-summary.json")
                .exists()
        );

        fs::remove_dir_all(root).ok();
    }
}
