use crate::cli::RunArgs;
use crate::narrative::{
    NarrativeOutputConfig, write_step_narrative_artifact, write_step_narrative_artifacts,
};
use crate::output::{OutputStyle, render_run_output};
use crate::progress::TerminalStepProgress;
use crate::report::{
    GeneratedSteps, StepGenerationMode, StepProgressObserver, StepReport, annotate_search_profile,
    annotate_single_step_replay_ablation, generate_steps_with_config_and_runtime_and_progress,
    stored_exact_screen_reasons, stored_prune_class_stats, write_step_report, write_step_reports,
};
use anyhow::{Context, Result, bail};
use pen_core::hash::blake3_hex;
use pen_core::ids::{ClauseId, ObligationSetId, StateId};
use pen_core::library::{LibrarySnapshot, LibrarySnapshotEntry};
use pen_search::config::{RuntimeConfig, SearchProfile, WorkerSetting};
use pen_search::diversify::{FrontierPressure, FrontierRuntimeLimits};
use pen_search::engine::supports_live_atomic_search;
use pen_search::frontier::FrontierWindow;
use pen_search::priority::{PriorityInputs, build_priority_key};
use pen_search::resume::CurrentCompat;
use pen_search::scheduler::build_schedule;
use pen_search::state::{FrontierStateRecV1, PrefixState};
use pen_store::frontier::{frontier_checkpoint_dir, write_frontier_manifest};
use pen_store::layout::{FRONTIER_RECORD_LAYOUT_ID, SCHEMA_VERSION_V1};
use pen_store::manifest::{
    BitBand, BuildInfo, CheckpointCompat, ConfigFingerprint, FrontierManifestV1, HostInfo,
    NearMiss, ResumeCompatible, RunArtifacts, RunCompat, RunManifestV1, RunPosition, RunStatus,
    RuntimeInfo, SearchPolicyInfo, StepCheckpointV1, StepObjective, StepStats,
};
use pen_store::memory::GovernorConfig;
use pen_store::spill::{FrontierRuntimeInput, SpillConfig, persist_frontier_runtime};
use pen_store::sqlite::{FrontierGenerationRow, MetadataDb};
use pen_store::telemetry::TelemetryEventV1;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::process::Command;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System, get_current_pid};
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
    let worker_count = resolved_worker_count(&config);
    let created_utc = now_utc()?;
    let manifest_base = build_run_manifest_base(
        &run_id,
        &config_text,
        &config,
        worker_count,
        created_utc.clone(),
    )?;
    let writer = RunArtifactWriter::start(
        &run_dir,
        &config_text,
        manifest_base,
        &config,
        worker_count,
        planned_run_mode(until_step).as_str(),
        config.mode.search_profile.as_str(),
        created_utc.clone(),
        Vec::new(),
        None,
    )?;
    let mut observer = RunStepObserver::new(writer, until_step);
    let generated = generate_steps_with_config_and_runtime_and_progress(
        until_step,
        &config,
        frontier_runtime_limits(&config, worker_count),
        Some(&mut observer as &mut dyn StepProgressObserver),
    );
    let mut writer = observer.into_writer();
    let GeneratedSteps { mode, mut steps } = match generated {
        Ok(generated) => generated,
        Err(error) => return Err(finalize_failed_run(writer, error)),
    };
    if let Some(error) = writer.take_error() {
        return Err(finalize_failed_run(writer, error));
    }

    annotate_search_profile(&mut steps, config.mode.search_profile);
    writer.finalize_success(mode.as_str())?;
    Ok(render_run_output(
        OutputStyle::from_debug(args.debug),
        &run_id,
        &steps,
        terminal_narrative_config(&config, args.narrative),
    ))
}

pub(crate) fn terminal_narrative_config<'a>(
    config: &'a RuntimeConfig,
    requested: bool,
) -> Option<NarrativeOutputConfig<'a>> {
    if requested && config.mode.search_profile.supports_narrative_artifacts() && config.demo.enabled
    {
        Some(NarrativeOutputConfig::from_runtime(config))
    } else {
        None
    }
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

#[derive(Clone)]
pub(crate) struct RunManifestBase {
    pub run_id: String,
    pub created_utc: String,
    pub workspace_version: String,
    pub compat: RunCompat,
    pub host: HostInfo,
    pub runtime: RuntimeInfo,
    pub config: ConfigFingerprint,
    pub search_policy: SearchPolicyInfo,
    pub build: BuildInfo,
    pub artifacts: RunArtifacts,
}

pub(crate) struct RunArtifactWriter<'a> {
    run_dir: PathBuf,
    manifest_base: RunManifestBase,
    config: &'a RuntimeConfig,
    worker_count: u16,
    provenance_override: Option<(crate::report::StepProvenance, u32)>,
    search_profile: String,
    metadata: MetadataDb,
    persisted_steps: Vec<StepReport>,
    error: Option<anyhow::Error>,
}

pub(crate) struct RunStepObserver<'a> {
    terminal: Option<TerminalStepProgress<std::io::Stderr>>,
    writer: RunArtifactWriter<'a>,
}

impl<'a> RunStepObserver<'a> {
    pub(crate) fn new(writer: RunArtifactWriter<'a>, until_step: u32) -> Self {
        Self {
            terminal: TerminalStepProgress::stderr(until_step),
            writer,
        }
    }

    pub(crate) fn into_writer(self) -> RunArtifactWriter<'a> {
        self.writer
    }
}

impl StepProgressObserver for RunStepObserver<'_> {
    fn on_step_started(&mut self, step_index: u32, label: &str) {
        if let Some(terminal) = self.terminal.as_mut() {
            terminal.on_step_started(step_index, label);
        }
    }

    fn on_step_completed(&mut self, step: &StepReport) {
        if let Some(terminal) = self.terminal.as_mut() {
            terminal.on_step_completed(step);
        }
        self.writer.on_step_completed(step);
    }
}

#[allow(dead_code)]
pub(crate) fn write_run_artifacts(
    run_dir: &Path,
    config_text: &str,
    manifest: &RunManifestV1,
    steps: &[StepReport],
    worker_count: u16,
    config: &RuntimeConfig,
    mode: &str,
    search_profile: &str,
) -> Result<()> {
    let metadata = prepare_run_directory(run_dir, config_text)?;

    write_json_file(&run_dir.join("run.json"), manifest)?;
    write_step_reports(run_dir, steps)?;
    write_step_narrative_artifacts(run_dir, steps, config)?;
    write_step_checkpoints(run_dir, manifest, steps, config.objective.window_depth)?;
    write_frontier_snapshots(run_dir, manifest, steps, worker_count, config, &metadata)?;
    write_telemetry(run_dir, manifest, steps, mode, search_profile)?;
    write_latest_reports(run_dir, &manifest.run_id, steps)?;
    Ok(())
}

pub(crate) fn build_run_manifest_base(
    run_id: &str,
    config_text: &str,
    config: &RuntimeConfig,
    worker_count: u16,
    created_utc: String,
) -> Result<RunManifestBase> {
    let workspace_root = workspace_root();
    Ok(RunManifestBase {
        run_id: run_id.to_owned(),
        created_utc,
        workspace_version: env!("CARGO_PKG_VERSION").to_owned(),
        compat: current_run_compat(),
        host: host_info(),
        runtime: RuntimeInfo {
            resolved_worker_count: worker_count,
        },
        config: ConfigFingerprint {
            path: "config.toml".to_owned(),
            sha256: sha256_hex(config_text.as_bytes()),
        },
        search_policy: search_policy_info(config.mode.search_profile),
        build: build_info(&workspace_root)?,
        artifacts: RunArtifacts {
            telemetry: "telemetry.ndjson".to_owned(),
            reports_dir: "reports".to_owned(),
            checkpoints_dir: "checkpoints".to_owned(),
        },
    })
}

pub(crate) fn build_run_manifest(
    manifest_base: &RunManifestBase,
    steps: &[StepReport],
    updated_utc: String,
    status: RunStatus,
    failure_note: String,
) -> RunManifestV1 {
    let completed_step = steps.last().map(|step| step.step_index).unwrap_or(0);
    let active_band = steps
        .last()
        .map(|step| u32::from(step.accepted.clause_kappa))
        .unwrap_or(0);
    let frontier_epoch = steps
        .iter()
        .filter(|step| step.step_index >= 4)
        .count()
        .try_into()
        .expect("frontier epoch exceeded u32");
    RunManifestV1 {
        schema_version: SCHEMA_VERSION_V1,
        run_id: manifest_base.run_id.clone(),
        status,
        failure_note,
        created_utc: manifest_base.created_utc.clone(),
        updated_utc,
        workspace_version: manifest_base.workspace_version.clone(),
        compat: manifest_base.compat.clone(),
        host: manifest_base.host.clone(),
        runtime: manifest_base.runtime.clone(),
        config: manifest_base.config.clone(),
        search_policy: manifest_base.search_policy.clone(),
        build: manifest_base.build.clone(),
        position: RunPosition {
            completed_step,
            active_step: completed_step.saturating_add(1),
            active_band,
            frontier_epoch,
        },
        artifacts: manifest_base.artifacts.clone(),
    }
}

impl<'a> RunArtifactWriter<'a> {
    pub(crate) fn start(
        run_dir: &Path,
        config_text: &str,
        manifest_base: RunManifestBase,
        config: &'a RuntimeConfig,
        worker_count: u16,
        mode: &str,
        search_profile: &str,
        initial_updated_utc: String,
        persisted_steps: Vec<StepReport>,
        provenance_override: Option<(crate::report::StepProvenance, u32)>,
    ) -> Result<Self> {
        let metadata = prepare_run_directory(run_dir, config_text)?;
        let writer = Self {
            run_dir: run_dir.to_path_buf(),
            manifest_base,
            config,
            worker_count,
            provenance_override,
            search_profile: search_profile.to_owned(),
            metadata,
            persisted_steps,
            error: None,
        };
        let manifest =
            writer.build_manifest(initial_updated_utc, RunStatus::Running, String::new());
        write_json_file(&writer.run_dir.join("run.json"), &manifest)?;
        write_latest_reports(&writer.run_dir, &manifest.run_id, &writer.persisted_steps)?;
        append_telemetry_event(
            &writer.run_dir,
            &run_started_event(&manifest, mode, search_profile),
        )?;
        Ok(writer)
    }

    fn on_step_completed(&mut self, step: &StepReport) {
        if self.error.is_some() {
            return;
        }
        if let Err(error) = self.persist_step(step) {
            self.error = Some(error);
        }
    }

    pub(crate) fn take_error(&mut self) -> Option<anyhow::Error> {
        self.error.take()
    }

    pub(crate) fn finalize_success(&mut self, mode: &str) -> Result<()> {
        let manifest = self.current_manifest(RunStatus::Completed, String::new())?;
        write_json_file(&self.run_dir.join("run.json"), &manifest)?;
        write_latest_reports(&self.run_dir, &manifest.run_id, &self.persisted_steps)?;
        write_telemetry(
            &self.run_dir,
            &manifest,
            &self.persisted_steps,
            mode,
            &self.search_profile,
        )?;
        Ok(())
    }

    fn finalize_failed(&mut self, error: &anyhow::Error) -> Result<()> {
        let failure_note = render_failure_note(error);
        let manifest = self.current_manifest(RunStatus::Failed, failure_note)?;
        write_json_file(&self.run_dir.join("run.json"), &manifest)?;
        write_latest_reports(&self.run_dir, &manifest.run_id, &self.persisted_steps)?;
        append_telemetry_event(&self.run_dir, &run_status_event(&manifest))?;
        Ok(())
    }

    fn persist_step(&mut self, step: &StepReport) -> Result<()> {
        let mut persisted = step.clone();
        persisted.search_profile = self.config.mode.search_profile;
        if self.config.mode.search_profile == SearchProfile::DesktopClaimShadow {
            annotate_observed_process_rss(&mut persisted.frontier_pressure);
        }
        if let Some((provenance, min_step_index)) = self.provenance_override {
            if persisted.step_index >= min_step_index {
                persisted.provenance = provenance;
            }
        }
        annotate_single_step_replay_ablation(&mut persisted, self.config.objective.window_depth)?;
        let index = self.upsert_step(persisted.clone())?;
        let updated_utc = now_utc()?;
        let manifest = self.build_manifest(updated_utc, RunStatus::Running, String::new());
        write_json_file(&self.run_dir.join("run.json"), &manifest)?;
        write_step_report(&self.run_dir, &persisted)?;
        write_step_narrative_artifact(&self.run_dir, &persisted, self.config)?;
        write_step_checkpoint(
            &self.run_dir,
            &manifest,
            &self.persisted_steps[..index],
            &persisted,
            self.config.objective.window_depth,
        )?;
        write_frontier_snapshot(
            &self.run_dir,
            &manifest,
            &persisted,
            frontier_epoch_for_prefix(&self.persisted_steps[..=index]),
            self.worker_count,
            self.config,
            &self.metadata,
        )?;
        append_telemetry_event(&self.run_dir, &step_accepted_event(&manifest, &persisted))?;
        write_latest_reports(&self.run_dir, &manifest.run_id, &self.persisted_steps)?;
        Ok(())
    }

    fn upsert_step(&mut self, step: StepReport) -> Result<usize> {
        let index =
            usize::try_from(step.step_index.saturating_sub(1)).expect("step index exceeded usize");
        if index < self.persisted_steps.len() {
            self.persisted_steps[index] = step;
            return Ok(index);
        }
        if index == self.persisted_steps.len() {
            self.persisted_steps.push(step);
            return Ok(index);
        }
        bail!(
            "step persistence skipped from {} to {}",
            self.persisted_steps.len() + 1,
            index + 1
        );
    }

    fn current_manifest(&self, status: RunStatus, failure_note: String) -> Result<RunManifestV1> {
        Ok(self.build_manifest(now_utc()?, status, failure_note))
    }

    fn build_manifest(
        &self,
        updated_utc: String,
        status: RunStatus,
        failure_note: String,
    ) -> RunManifestV1 {
        build_run_manifest(
            &self.manifest_base,
            &self.persisted_steps,
            updated_utc,
            status,
            failure_note,
        )
    }
}

pub(crate) fn finalize_failed_run(
    mut writer: RunArtifactWriter<'_>,
    error: anyhow::Error,
) -> anyhow::Error {
    match writer.finalize_failed(&error) {
        Ok(()) => error,
        Err(persist_error) => error.context(format!(
            "also failed to persist failed-run artifacts: {persist_error:#}"
        )),
    }
}

fn planned_run_mode(until_step: u32) -> StepGenerationMode {
    if supports_live_atomic_search(until_step) {
        StepGenerationMode::AtomicBootstrapSearch
    } else {
        StepGenerationMode::ReferenceReplay
    }
}

fn prepare_run_directory(run_dir: &Path, config_text: &str) -> Result<MetadataDb> {
    fs::create_dir_all(run_dir).with_context(|| format!("create {}", run_dir.display()))?;
    fs::create_dir_all(run_dir.join("reports").join("steps"))?;
    fs::create_dir_all(run_dir.join("checkpoints").join("steps"))?;
    fs::create_dir_all(run_dir.join("checkpoints").join("frontier"))?;
    fs::write(run_dir.join("config.toml"), config_text)
        .with_context(|| format!("write {}", run_dir.join("config.toml").display()))?;
    MetadataDb::open(&run_dir.join("meta.sqlite3"))
}

fn write_latest_reports(run_dir: &Path, run_id: &str, steps: &[StepReport]) -> Result<()> {
    fs::create_dir_all(run_dir.join("reports"))?;
    fs::write(
        run_dir.join("reports").join("latest.txt"),
        crate::report::render_standard_report(run_id, steps),
    )?;
    fs::write(
        run_dir.join("reports").join("latest.debug.txt"),
        crate::report::render_debug_report(run_id, steps),
    )?;
    Ok(())
}

fn render_failure_note(error: &anyhow::Error) -> String {
    error
        .chain()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(": ")
}

fn frontier_epoch_for_prefix(steps: &[StepReport]) -> u32 {
    steps
        .iter()
        .filter(|step| step.step_index >= 4)
        .count()
        .try_into()
        .expect("frontier epoch exceeded u32")
}

#[allow(dead_code)]
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
            near_misses: checkpoint_near_misses(step),
            stats: checkpoint_stats(step),
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

fn write_step_checkpoint(
    run_dir: &Path,
    manifest: &RunManifestV1,
    previous_steps: &[StepReport],
    step: &StepReport,
    window_depth: u16,
) -> Result<()> {
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
        library_snapshot: library_snapshot(previous_steps, window_depth),
        near_misses: checkpoint_near_misses(step),
        stats: checkpoint_stats(step),
    };
    write_json_file(
        &run_dir
            .join("checkpoints")
            .join("steps")
            .join(format!("step-{:02}.json", step.step_index)),
        &checkpoint,
    )
}

fn checkpoint_near_misses(step: &StepReport) -> Vec<NearMiss> {
    if step.step_index >= 4 {
        step.near_misses.clone()
    } else {
        Vec::new()
    }
}

fn checkpoint_stats(step: &StepReport) -> StepStats {
    if step.step_index >= 4 {
        if step.search_stats.prefix_states_explored > 0 {
            return StepStats {
                frontier_scanned: step.search_stats.prefix_states_explored as u64,
                typed_prefixes: (step.search_stats.prefix_frontier_hot_states
                    + step.search_stats.prefix_frontier_cold_states)
                    as u64,
                sound_prunes: step.search_stats.prefix_states_exact_pruned as u64,
                heuristic_drops: step.search_stats.prefix_states_heuristic_dropped as u64,
            };
        }
        return StepStats {
            frontier_scanned: step.search_stats.evaluated_candidates as u64,
            typed_prefixes: step
                .search_stats
                .evaluated_candidates
                .saturating_sub(step.search_stats.dedupe_prunes) as u64,
            sound_prunes: step.search_stats.minimality_prunes as u64,
            heuristic_drops: step.search_stats.frontier_drops as u64,
        };
    }

    StepStats {
        frontier_scanned: 1,
        typed_prefixes: 1,
        sound_prunes: 0,
        heuristic_drops: 0,
    }
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
    search_profile: &str,
) -> Result<()> {
    let mut lines = Vec::with_capacity(steps.len() + 2);
    lines.push(serde_json::to_string(&run_started_event(
        manifest,
        mode,
        search_profile,
    ))?);
    for step in steps {
        lines.push(serde_json::to_string(&step_accepted_event(manifest, step))?);
    }
    lines.push(serde_json::to_string(&run_status_event(manifest))?);
    fs::write(
        run_dir.join("telemetry.ndjson"),
        format!("{}\n", lines.join("\n")),
    )?;
    Ok(())
}

fn append_telemetry_event(run_dir: &Path, event: &TelemetryEventV1) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(run_dir.join("telemetry.ndjson"))
        .with_context(|| format!("open {}", run_dir.join("telemetry.ndjson").display()))?;
    use std::io::Write;
    writeln!(file, "{}", serde_json::to_string(event)?).context("append telemetry event")?;
    Ok(())
}

fn run_started_event(
    manifest: &RunManifestV1,
    mode: &str,
    search_profile: &str,
) -> TelemetryEventV1 {
    TelemetryEventV1 {
        schema_version: SCHEMA_VERSION_V1,
        run_id: manifest.run_id.clone(),
        event: "run_started".to_owned(),
        step_index: None,
        payload: json!({
            "created_utc": manifest.created_utc,
            "mode": mode,
            "search_profile": search_profile,
            "search_policy": {
                "guidance_style": manifest.search_policy.guidance_style,
                "late_expansion_policy": manifest.search_policy.late_expansion_policy,
                "bucket_policy": manifest.search_policy.bucket_policy,
            },
        }),
    }
}

fn step_accepted_event(manifest: &RunManifestV1, step: &StepReport) -> TelemetryEventV1 {
    let late_step_claim = match step.canon_evidence.late_step_claim.status {
        crate::report::LateStepClaimStatus::NotApplicable => serde_json::Value::Null,
        crate::report::LateStepClaimStatus::ExecutableCanon => json!({
            "status": "executable_canon",
            "adopted_step": step.canon_evidence.late_step_claim.adopted_step,
            "adopted_label": step.canon_evidence.late_step_claim.adopted_label,
            "adopted_nu": step.canon_evidence.late_step_claim.adopted_nu,
            "matches_accepted": step.canon_evidence.late_step_claim.matches_accepted,
        }),
        crate::report::LateStepClaimStatus::HistoricalProvenanceOnly => json!({
            "status": "historical_provenance_only",
            "adopted_step": step.canon_evidence.late_step_claim.adopted_step,
            "adopted_label": step.canon_evidence.late_step_claim.adopted_label,
            "adopted_nu": step.canon_evidence.late_step_claim.adopted_nu,
            "matches_accepted": step.canon_evidence.late_step_claim.matches_accepted,
        }),
    };
    let replay_ablation = json!({
        "status": step.replay_ablation.status.as_str(),
        "reference_candidate_hash": step.replay_ablation.reference_candidate_hash,
        "reference_canonical_hash": step.replay_ablation.reference_canonical_hash,
        "rho_delta": step.replay_ablation.rho_delta.to_string(),
        "objective_bar_delta": step.replay_ablation.objective_bar_delta.to_string(),
        "overshoot_delta": step.replay_ablation.overshoot_delta.to_string(),
        "nu_delta": step.replay_ablation.nu_delta,
        "clause_kappa_delta": step.replay_ablation.clause_kappa_delta,
    });
    let prune_sample_counts = json!({
        "quotient_dedupe": step.prune_reports.iter().filter(|report| {
            report.prune_class == crate::report::PruneReportClass::QuotientDedupe
        }).count(),
        "sound_minimality": step.prune_reports.iter().filter(|report| {
            report.prune_class == crate::report::PruneReportClass::SoundMinimality
        }).count(),
        "heuristic_shaping": step.prune_reports.iter().filter(|report| {
            report.prune_class == crate::report::PruneReportClass::HeuristicShaping
        }).count(),
    });
    let prune_class_totals = stored_prune_class_stats(step);
    let exact_screen_reason_totals = stored_exact_screen_reasons(step);

    TelemetryEventV1 {
        schema_version: SCHEMA_VERSION_V1,
        run_id: manifest.run_id.clone(),
        event: "step_accepted".to_owned(),
        step_index: Some(step.step_index),
        payload: json!({
            "label": step.label,
            "nu": step.accepted.nu,
            "kappa": step.accepted.clause_kappa,
            "charged_kappa": step.canon_evidence.charged_clause_kappa,
            "rho": step.accepted.rho.to_string(),
            "bar": step.objective_bar.to_string(),
            "bar_distance": step.canon_evidence.bar_distance.to_string(),
            "candidate_hash": step.accepted.candidate_hash,
            "late_step_claim": late_step_claim,
            "replay_ablation": replay_ablation,
            "prune_classes": {
                "quotient_dedupe": prune_class_totals.quotient_dedupe,
                "sound_minimality": prune_class_totals.sound_minimality,
                "heuristic_shaping": prune_class_totals.heuristic_shaping,
            },
            "exact_screen_reasons": {
                "partial_prefix_bar_failure": exact_screen_reason_totals.partial_prefix_bar_failure,
                "terminal_prefix_completion_failure": exact_screen_reason_totals.terminal_prefix_completion_failure,
                "incumbent_dominance": exact_screen_reason_totals.incumbent_dominance,
                "legality_connectivity_exact_rejection": exact_screen_reason_totals.legality_connectivity_exact_rejection,
            },
            "prune_samples": prune_sample_counts,
            "frontier_pressure": {
                "governor_state": step.frontier_pressure.governor_state.as_str(),
                "pressure_action": step.frontier_pressure.pressure_action.as_str(),
                "rss_bytes": step.frontier_pressure.rss_bytes,
                "observed_process_rss_bytes": step.frontier_pressure.observed_process_rss_bytes,
                "rss_gap_bytes": step.frontier_pressure.rss_gap_bytes,
                "hot_frontier_bytes": step.frontier_pressure.hot_frontier_bytes,
                "cold_frontier_bytes": step.frontier_pressure.cold_frontier_bytes,
                "dedupe_bytes": step.frontier_pressure.dedupe_bytes,
                "requested_cold_limit": step.frontier_pressure.requested_cold_limit,
                "retained_cold_limit": step.frontier_pressure.retained_cold_limit,
                "resident_cold_limit": step.frontier_pressure.resident_cold_limit,
                "spill_backed_cold_records": step.frontier_pressure.spill_backed_cold_records,
                "dropped_cold_records": step.frontier_pressure.dropped_cold_records,
            },
            "search_timing": {
                "step_wall_clock_millis": step.search_stats.search_timing.step_wall_clock_millis,
                "candidate_discovery_wall_clock_millis": step.search_stats.search_timing.candidate_discovery_wall_clock_millis,
                "prefix_frontier_planning_wall_clock_millis": step.search_stats.search_timing.prefix_frontier_planning_wall_clock_millis,
                "selection_wall_clock_millis": step.search_stats.search_timing.selection_wall_clock_millis,
            },
        }),
    }
}

fn run_status_event(manifest: &RunManifestV1) -> TelemetryEventV1 {
    let event = match manifest.status {
        RunStatus::Completed => "run_completed",
        RunStatus::Failed => "run_failed",
        RunStatus::Paused => "run_paused",
        RunStatus::Running => "run_running",
    };
    let mut payload = serde_json::Map::from_iter([
        (
            "completed_step".to_owned(),
            serde_json::Value::from(manifest.position.completed_step),
        ),
        (
            "status".to_owned(),
            serde_json::Value::from(manifest.status.as_str()),
        ),
    ]);
    if !manifest.failure_note.is_empty() {
        payload.insert(
            "failure_note".to_owned(),
            serde_json::Value::from(manifest.failure_note.clone()),
        );
    }
    TelemetryEventV1 {
        schema_version: SCHEMA_VERSION_V1,
        run_id: manifest.run_id.clone(),
        event: event.to_owned(),
        step_index: manifest.position.completed_step.checked_add(1),
        payload: serde_json::Value::Object(payload),
    }
}

fn write_json_file<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(value)?;
    fs::write(path, format!("{json}\n")).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}

pub(crate) fn search_policy_info(search_profile: SearchProfile) -> SearchPolicyInfo {
    match search_profile {
        SearchProfile::Unknown | SearchProfile::StrictCanonGuarded => SearchPolicyInfo {
            guidance_style: "legacy_family_guided".to_owned(),
            late_expansion_policy: "guarded_exact".to_owned(),
            bucket_policy: "guarded_structural".to_owned(),
        },
        SearchProfile::RelaxedShadow => SearchPolicyInfo {
            guidance_style: "legacy_family_guided".to_owned(),
            late_expansion_policy: "relaxed_shadow".to_owned(),
            bucket_policy: "semantic_family_runtime_local".to_owned(),
        },
        SearchProfile::RealisticFrontierShadow => SearchPolicyInfo {
            guidance_style: "legacy_family_guided".to_owned(),
            late_expansion_policy: "realistic_shadow".to_owned(),
            bucket_policy: "semantic_family_runtime_local".to_owned(),
        },
        SearchProfile::DemoBreadthShadow => SearchPolicyInfo {
            guidance_style: "legacy_family_guided".to_owned(),
            late_expansion_policy: "demo_breadth_shadow".to_owned(),
            bucket_policy: "semantic_family_runtime_local".to_owned(),
        },
        SearchProfile::DesktopClaimShadow => SearchPolicyInfo {
            guidance_style: "claim_debt_guided".to_owned(),
            late_expansion_policy: "claim_generic".to_owned(),
            bucket_policy: "structural_generic".to_owned(),
        },
    }
}

fn absolute_from_repo(path: &Path) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(workspace_root().join(path))
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
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
    let mut system = System::new_all();
    system.refresh_all();
    let cpu_model = system
        .cpus()
        .first()
        .map(|cpu| cpu.brand().trim().to_owned())
        .filter(|brand| !brand.is_empty())
        .unwrap_or_else(|| "unknown".to_owned());
    let physical_core_count = System::physical_core_count().unwrap_or(logical_cpus as usize) as u32;
    HostInfo {
        os: std::env::consts::OS.to_owned(),
        arch: std::env::consts::ARCH.to_owned(),
        logical_cpus,
        ram_bytes: system.total_memory(),
        cpu_model,
        physical_core_count,
    }
}

fn build_info(workspace_root: &Path) -> Result<BuildInfo> {
    let cargo_lock_path = workspace_root.join("Cargo.lock");
    let binary_path = std::env::current_exe().context("locate current executable")?;
    let cargo_lock_sha256 = sha256_hex(
        &fs::read(&cargo_lock_path)
            .with_context(|| format!("read {}", cargo_lock_path.display()))?,
    );
    let binary_sha256 = sha256_hex(
        &fs::read(&binary_path).with_context(|| format!("read {}", binary_path.display()))?,
    );

    Ok(BuildInfo {
        profile: option_env!("PEN_BUILD_PROFILE")
            .unwrap_or("unknown")
            .to_owned(),
        target_triple: option_env!("PEN_TARGET_TRIPLE")
            .unwrap_or("unknown")
            .to_owned(),
        target_cpu: option_env!("PEN_TARGET_CPU")
            .unwrap_or("default")
            .to_owned(),
        git_commit_sha: git_stdout(workspace_root, &["rev-parse", "HEAD"])?,
        dirty_tree: git_dirty_tree(workspace_root)?,
        cargo_lock_sha256,
        binary_sha256,
    })
}

fn git_stdout(workspace_root: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .current_dir(workspace_root)
        .output()
        .with_context(|| format!("run git {}", args.join(" ")))?;
    if !output.status.success() {
        bail!(
            "git {} failed with status {:?}",
            args.join(" "),
            output.status.code()
        );
    }

    let stdout = String::from_utf8(output.stdout).context("decode git stdout")?;
    let trimmed = stdout.trim();
    if trimmed.is_empty() {
        bail!("git {} returned empty stdout", args.join(" "));
    }
    Ok(trimmed.to_owned())
}

fn git_dirty_tree(workspace_root: &Path) -> Result<bool> {
    let output = Command::new("git")
        .args(["status", "--porcelain", "--untracked-files=no"])
        .current_dir(workspace_root)
        .output()
        .context("run git status --porcelain --untracked-files=no")?;
    if !output.status.success() {
        bail!("git status --porcelain --untracked-files=no failed");
    }

    let stdout = String::from_utf8(output.stdout).context("decode git status stdout")?;
    Ok(!stdout.trim().is_empty())
}

pub(crate) fn resolved_worker_count(config: &RuntimeConfig) -> u16 {
    let logical_cpus = std::thread::available_parallelism()
        .map(|count| count.get() as u16)
        .unwrap_or(1);
    let mut system = System::new();
    system.refresh_memory();
    resolve_worker_count_for_host(config, logical_cpus, system.total_memory())
}

pub(crate) fn frontier_runtime_limits(
    config: &RuntimeConfig,
    worker_count: u16,
) -> FrontierRuntimeLimits {
    FrontierRuntimeLimits {
        worker_count,
        governor: governor_config(config),
        spill: spill_config(config),
        record_bytes: FrontierStateRecV1::BYTE_LEN as u64,
        dedupe_bytes_per_record: 32,
        worker_scratch_bytes: worker_scratch_bytes(worker_count, config),
        checkpoint_bytes: checkpoint_buffer_bytes(config),
        spill_buffer_bytes: spill_buffer_bytes(config),
    }
}

#[allow(dead_code)]
fn write_frontier_snapshots(
    run_dir: &Path,
    run_manifest: &RunManifestV1,
    steps: &[StepReport],
    worker_count: u16,
    config: &RuntimeConfig,
    metadata: &MetadataDb,
) -> Result<()> {
    for (frontier_epoch, step) in steps.iter().filter(|step| step.step_index >= 4).enumerate() {
        write_frontier_snapshot(
            run_dir,
            run_manifest,
            step,
            u32::try_from(frontier_epoch + 1).expect("frontier epoch exceeded u32"),
            worker_count,
            config,
            metadata,
        )?;
    }

    Ok(())
}

fn write_frontier_snapshot(
    run_dir: &Path,
    run_manifest: &RunManifestV1,
    step: &StepReport,
    frontier_epoch: u32,
    worker_count: u16,
    config: &RuntimeConfig,
    metadata: &MetadataDb,
) -> Result<()> {
    if step.step_index < 4 {
        return Ok(());
    }

    let band_index = u32::from(step.accepted.clause_kappa);
    let frontier_dir = frontier_checkpoint_dir(run_dir, step.step_index, band_index);
    let (mut window, dedupe_keys) = frontier_window_for_step(step, worker_count);
    if window.total_len() == 0 {
        return Ok(());
    }
    let schedule = build_schedule(&window, worker_count);
    apply_schedule_to_window(&mut window, &schedule);
    let frontier_worker_count = schedule.assignments.len() as u16;

    let hot_records = window
        .hot
        .iter()
        .map(|record| record.to_le_bytes())
        .collect::<Vec<_>>();
    let cold_records = window
        .cold
        .iter()
        .map(|record| record.to_le_bytes())
        .collect::<Vec<_>>();
    let compat = current_search_compat();
    let runtime = persist_frontier_runtime(
        &frontier_dir,
        &FrontierRuntimeInput {
            frontier_epoch,
            hot_records,
            cold_records,
            dedupe_keys: dedupe_keys.into_iter().collect(),
            prefixes_created: step.search_stats.prefixes_created as u64,
            prefix_states_explored: step.search_stats.prefix_states_explored as u64,
            prefix_states_merged_by_signature: step.search_stats.prefix_states_merged_by_signature
                as u64,
            prefix_states_exact_pruned: step.search_stats.prefix_states_exact_pruned as u64,
            prefix_states_heuristic_dropped: step.search_stats.prefix_states_heuristic_dropped
                as u64,
            incremental_legality_cache_hits: step.search_stats.incremental_legality_cache_hits
                as u64,
            incremental_connectivity_shortcuts: step.search_stats.incremental_connectivity_shortcuts
                as u64,
            incremental_connectivity_fallbacks: step.search_stats.incremental_connectivity_fallbacks
                as u64,
            incremental_connectivity_prunes: step.search_stats.incremental_connectivity_prunes
                as u64,
            incremental_clause_family_filter_hits: step
                .search_stats
                .incremental_clause_family_filter_hits
                as u64,
            incremental_clause_family_prunes: step.search_stats.incremental_clause_family_prunes
                as u64,
            incremental_active_window_clause_filter_hits: step
                .search_stats
                .incremental_active_window_clause_filter_hits
                as u64,
            incremental_active_window_clause_filter_prunes: step
                .search_stats
                .incremental_active_window_clause_filter_prunes
                as u64,
            incremental_terminal_clause_filter_hits: step
                .search_stats
                .incremental_terminal_clause_filter_hits
                as u64,
            incremental_terminal_clause_filter_prunes: step
                .search_stats
                .incremental_terminal_clause_filter_prunes
                as u64,
            incremental_trivial_derivability_hits: step
                .search_stats
                .incremental_trivial_derivability_hits
                as u64,
            incremental_trivial_derivability_prunes: step
                .search_stats
                .incremental_trivial_derivability_prunes
                as u64,
            incremental_terminal_admissibility_hits: step
                .search_stats
                .incremental_terminal_admissibility_hits
                as u64,
            incremental_terminal_admissibility_rejections: step
                .search_stats
                .incremental_terminal_admissibility_rejections
                as u64,
            incremental_terminal_prefix_completion_hits: step
                .search_stats
                .incremental_terminal_prefix_completion_hits
                as u64,
            incremental_terminal_prefix_rank_hits: step
                .search_stats
                .incremental_terminal_prefix_rank_hits
                as u64,
            incremental_terminal_rank_prunes: step.search_stats.incremental_terminal_rank_prunes
                as u64,
            incremental_partial_prefix_bound_hits: step
                .search_stats
                .incremental_partial_prefix_bound_hits
                as u64,
            incremental_partial_prefix_bound_checks: step
                .search_stats
                .incremental_partial_prefix_bound_checks
                as u64,
            incremental_partial_prefix_bound_prunes: step
                .search_stats
                .incremental_partial_prefix_bound_prunes
                as u64,
            incremental_terminal_prefix_bar_prunes: step
                .search_stats
                .incremental_terminal_prefix_bar_prunes
                as u64,
            worker_count: frontier_worker_count,
            priority_heads: window.priority_heads(8),
            interner_bytes: 0,
            worker_scratch_bytes: worker_scratch_bytes(frontier_worker_count, config),
            checkpoint_bytes: checkpoint_buffer_bytes(config),
            spill_buffer_bytes: spill_buffer_bytes(config),
        },
        governor_config(config),
        spill_config(config),
    )?;
    let frontier_manifest = FrontierManifestV1 {
        schema_version: SCHEMA_VERSION_V1,
        run_id: run_manifest.run_id.clone(),
        step_index: step.step_index,
        band_index,
        frontier_epoch,
        base_step_checkpoint: format!("../../../steps/step-{:02}.json", step.step_index),
        resume_compatible: ResumeCompatible {
            ast_schema_hash: compat.ast_schema_hash,
            type_rules_hash: compat.type_rules_hash,
            evaluator_hash: compat.evaluator_hash,
            search_semantics_hash: compat.search_semantics_hash,
            record_layout_id: compat.record_layout_id,
        },
        counts: runtime.counts,
        files: runtime.files,
        memory_snapshot: runtime.memory_snapshot,
        scheduler: runtime.scheduler,
    };
    write_frontier_manifest(&frontier_dir, &frontier_manifest)?;
    metadata.record_frontier_generation(&FrontierGenerationRow {
        run_id: run_manifest.run_id.clone(),
        step_index: step.step_index,
        band_index,
        frontier_epoch,
        worker_count: frontier_manifest.scheduler.worker_count,
        spill_generation: frontier_manifest.scheduler.spill_generation,
        hot_states: frontier_manifest.counts.hot_states,
        cold_states: frontier_manifest.counts.cold_states,
        governor_state: runtime.governor_state.as_str().to_owned(),
        pressure_action: runtime.pressure_action.as_str().to_owned(),
        rss_bytes: frontier_manifest.memory_snapshot.rss_bytes,
    })?;
    Ok(())
}

fn governor_config(config: &RuntimeConfig) -> GovernorConfig {
    GovernorConfig {
        green_limit_bytes: gib_to_bytes((config.memory.target_rss_gib - 1.0).max(0.0)),
        yellow_limit_bytes: gib_to_bytes(config.memory.soft_rss_gib),
        orange_limit_bytes: gib_to_bytes(config.memory.pressure_rss_gib),
        red_limit_bytes: gib_to_bytes(config.memory.emergency_rss_gib),
        hard_limit_bytes: gib_to_bytes(config.memory.hard_rss_gib),
    }
}

fn spill_config(config: &RuntimeConfig) -> SpillConfig {
    let checkpoint_bytes = checkpoint_buffer_bytes(config);
    let spill_bytes = spill_buffer_bytes(config);
    let record_len = FrontierStateRecV1::BYTE_LEN as u64;
    SpillConfig {
        max_records_per_shard: ((checkpoint_bytes.max(record_len) / 4) / record_len).clamp(1, 2048)
            as usize,
        max_dedupe_keys_per_segment: (checkpoint_bytes / 32).clamp(1, 4096) as usize,
        resident_cold_records: (spill_bytes / record_len).clamp(1, 4096) as usize,
    }
}

fn worker_scratch_bytes(worker_count: u16, config: &RuntimeConfig) -> u64 {
    u64::from(worker_count.max(1)) * u64::from(config.memory.worker_arena_mib) * 1024 * 1024
}

fn checkpoint_buffer_bytes(config: &RuntimeConfig) -> u64 {
    gib_to_bytes(config.memory.checkpoint_buffers_gib)
}

fn spill_buffer_bytes(config: &RuntimeConfig) -> u64 {
    gib_to_bytes(config.memory.spill_buffers_gib)
}

fn resolve_worker_count_for_host(
    config: &RuntimeConfig,
    logical_cpus: u16,
    host_ram_bytes: u64,
) -> u16 {
    let cpu_based_workers = config.search.resolve_workers(logical_cpus);
    if config.search.workers != WorkerSetting::Auto
        || config.mode.search_profile != SearchProfile::DesktopClaimShadow
        || host_ram_bytes == 0
    {
        return cpu_based_workers;
    }

    let arena_bytes = u64::from(config.memory.worker_arena_mib.max(1)) * 1024 * 1024;
    let host_claim_budget =
        host_ram_bytes.saturating_sub(gib_to_bytes(config.memory.reserve_for_os_gib));
    let steady_target = gib_to_bytes(config.memory.target_rss_gib).min(host_claim_budget);
    let soft_target = gib_to_bytes(config.memory.soft_rss_gib).min(host_claim_budget);
    let worker_headroom = soft_target.saturating_sub(steady_target);
    let memory_based_workers = u16::try_from((worker_headroom / arena_bytes).max(1))
        .unwrap_or(u16::MAX)
        .min(config.search.max_workers.max(1));

    cpu_based_workers.min(memory_based_workers.max(1))
}

fn annotate_observed_process_rss(frontier_pressure: &mut FrontierPressure) {
    if frontier_pressure.rss_bytes == 0 {
        return;
    }
    let Some(observed_process_rss_bytes) = current_process_rss_bytes() else {
        return;
    };
    frontier_pressure.observed_process_rss_bytes = observed_process_rss_bytes;
    frontier_pressure.rss_gap_bytes =
        signed_byte_delta(observed_process_rss_bytes, frontier_pressure.rss_bytes);
}

fn current_process_rss_bytes() -> Option<u64> {
    let pid = get_current_pid().ok()?;
    let mut system = System::new();
    system.refresh_processes_specifics(
        ProcessesToUpdate::Some(&[pid]),
        false,
        ProcessRefreshKind::nothing().with_memory(),
    );
    system.process(pid).map(|process| process.memory())
}

fn signed_byte_delta(observed_bytes: u64, accounted_bytes: u64) -> i64 {
    if observed_bytes >= accounted_bytes {
        i64::try_from(observed_bytes - accounted_bytes).unwrap_or(i64::MAX)
    } else {
        -i64::try_from(accounted_bytes - observed_bytes).unwrap_or(i64::MAX)
    }
}

fn gib_to_bytes(value: f64) -> u64 {
    (value * 1024_f64 * 1024_f64 * 1024_f64).round() as u64
}

fn frontier_window_for_step(
    step: &StepReport,
    worker_count: u16,
) -> (FrontierWindow, BTreeSet<String>) {
    if !step.prefix_frontier.is_empty() {
        let mut frontier = FrontierWindow {
            hot: step
                .prefix_frontier
                .hot_states
                .iter()
                .map(|state| state.to_record())
                .collect(),
            cold: step
                .prefix_frontier
                .cold_states
                .iter()
                .map(|state| state.to_record())
                .collect(),
        };
        frontier.compact_sorted();
        return (
            frontier,
            step.prefix_frontier
                .dedupe_keys
                .iter()
                .cloned()
                .collect::<BTreeSet<_>>(),
        );
    }

    let mut frontier = FrontierWindow::default();
    let mut dedupe_keys = BTreeSet::new();

    for (ordinal, candidate) in step.candidate_reports.iter().enumerate() {
        if matches!(
            candidate.frontier_retention,
            crate::report::FrontierRetention::NotRecorded
                | crate::report::FrontierRetention::Dropped
        ) {
            continue;
        }

        dedupe_keys.insert(candidate.canonical_hash.clone());
        let record = frontier_record(step.step_index, ordinal as u64, candidate, worker_count);
        match candidate.frontier_retention {
            crate::report::FrontierRetention::Hot => frontier.push_hot(record),
            crate::report::FrontierRetention::ColdResident
            | crate::report::FrontierRetention::SpillBacked => frontier.push_cold(record),
            crate::report::FrontierRetention::NotRecorded
            | crate::report::FrontierRetention::Dropped => {}
        }
    }

    frontier.compact_sorted();
    (frontier, dedupe_keys)
}

fn frontier_record(
    step_index: u32,
    ordinal: u64,
    candidate: &crate::report::CandidateReport,
    worker_count: u16,
) -> FrontierStateRecV1 {
    let state_id = StateId::new((u64::from(step_index) << 32) | ordinal);
    let clause_count = u16::try_from(candidate.clauses.len()).expect("clause count exceeded u16");
    let depth = clause_count.max(u16::try_from(candidate.library_refs.len()).unwrap_or(u16::MAX));
    let band_index = u8::try_from(candidate.clause_kappa).expect("band index exceeded u8");
    let priority_key = build_priority_key(PriorityInputs {
        band_index,
        nu_lower_bound: candidate.nu,
        bit_kappa_used: candidate.bit_kappa,
        clause_kappa_used: candidate.clause_kappa,
        depth,
        state_id,
    });
    let prefix = PrefixState {
        state_id,
        parent_state_id: StateId::new(0),
        last_clause_id: ClauseId::new(u32::try_from(candidate.clauses.len()).unwrap_or(u32::MAX)),
        obligation_set_id: ObligationSetId::new(step_index),
        shape_hash64: truncated_hash64(candidate.candidate_hash.as_bytes()),
        support_hash64: truncated_hash64(candidate.canonical_hash.as_bytes()),
        nu_lower_bound: candidate.nu,
        nu_upper_bound: candidate.nu,
        bit_kappa_used: candidate.bit_kappa,
        clause_kappa_used: candidate.clause_kappa,
        depth,
        step_index: u8::try_from(step_index).expect("step index exceeded u8"),
        band_index,
        flags: frontier_flags(candidate),
    };

    FrontierStateRecV1::from_prefix(
        prefix,
        priority_key,
        u16::try_from(ordinal % u64::from(worker_count.max(1))).expect("worker hint exceeded u16"),
    )
}

fn apply_schedule_to_window(
    window: &mut FrontierWindow,
    schedule: &pen_search::scheduler::SchedulerPlan,
) {
    let assignments = schedule
        .assignments
        .iter()
        .flat_map(|assignment| {
            assignment
                .records
                .iter()
                .map(move |record| (record.state_id, assignment.worker_id))
        })
        .collect::<std::collections::BTreeMap<_, _>>();

    for record in window.hot.iter_mut().chain(window.cold.iter_mut()) {
        if let Some(worker_id) = assignments.get(&record.state_id) {
            record.worker_hint = *worker_id;
        }
    }
}

fn frontier_flags(candidate: &crate::report::CandidateReport) -> u16 {
    let status_bits = match candidate.status {
        crate::report::CandidateStatus::AcceptedMinimalOvershoot => 0b0001,
        crate::report::CandidateStatus::ClearsBarHigherOvershoot => 0b0010,
        crate::report::CandidateStatus::BelowBar => 0b0100,
    };
    let retention_bits = match candidate.frontier_retention {
        crate::report::FrontierRetention::Hot => 0b1 << 8,
        crate::report::FrontierRetention::ColdResident
        | crate::report::FrontierRetention::SpillBacked => 0b1 << 9,
        crate::report::FrontierRetention::Dropped
        | crate::report::FrontierRetention::NotRecorded => 0,
    };
    let class_bits = match candidate.retention_class {
        pen_type::obligations::RetentionClass::GenericMacro => 0,
        pen_type::obligations::RetentionClass::StructuralSupport => 0b01 << 10,
        pen_type::obligations::RetentionClass::RareBridgeHead => 0b10 << 10,
        pen_type::obligations::RetentionClass::RareFocusHead => 0b11 << 10,
    };
    status_bits | retention_bits | class_bits
}

fn truncated_hash64(bytes: &[u8]) -> u64 {
    let hash = blake3_hex(bytes);
    u64::from_str_radix(&hash[..16], 16).expect("blake3 prefix should parse")
}

#[cfg(test)]
mod tests {
    use super::{RunArtifactWriter, build_run_manifest_base, planned_run_mode, run};
    use crate::cli::RunArgs;
    use crate::report::generate_steps_with_config_and_runtime;
    use pen_search::diversify::FrontierRuntimeLimits;
    use pen_store::frontier::frontier_checkpoint_dir;
    use pen_store::manifest::{RunManifestV1, RunStatus};
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
            narrative: false,
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

    #[test]
    fn demo_run_writes_narrative_and_event_artifacts() {
        let root = temp_dir("demo-run");
        run(RunArgs {
            config: "configs/demo_breadth_shadow_10m.toml".into(),
            root: root.clone(),
            run_id: Some("demo-test-run".to_owned()),
            until_step: Some(3),
            debug: false,
            narrative: false,
        })
        .expect("demo run should succeed");

        let run_dir = root.join("demo-test-run").join("reports").join("steps");
        assert!(run_dir.join("step-03-narrative.txt").exists());
        assert!(run_dir.join("step-03-events.ndjson").exists());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn claim_run_writes_policy_metadata_and_claim_narrative() {
        let root = temp_dir("claim-run");
        let output = run(RunArgs {
            config: "configs/desktop_claim_shadow_smoke.toml".into(),
            root: root.clone(),
            run_id: Some("claim-test-run".to_owned()),
            until_step: Some(3),
            debug: false,
            narrative: true,
        })
        .expect("claim run should succeed");

        assert!(output.contains("claim narrative"));

        let run_dir = root.join("claim-test-run");
        let manifest: RunManifestV1 = serde_json::from_str(
            &fs::read_to_string(run_dir.join("run.json")).expect("run manifest should exist"),
        )
        .expect("run manifest should parse");
        assert_eq!(manifest.search_policy.guidance_style, "claim_debt_guided");
        assert_eq!(
            manifest.search_policy.late_expansion_policy,
            "claim_generic"
        );
        assert_eq!(manifest.search_policy.bucket_policy, "structural_generic");
        assert!(manifest.host.ram_bytes > 0);
        assert!(manifest.host.physical_core_count > 0);
        assert!(!manifest.host.cpu_model.is_empty());
        assert!(manifest.runtime.resolved_worker_count > 0);
        assert!(!manifest.build.profile.is_empty());
        assert!(!manifest.build.target_triple.is_empty());
        assert!(!manifest.build.target_cpu.is_empty());
        assert!(!manifest.build.git_commit_sha.is_empty());
        assert!(!manifest.build.cargo_lock_sha256.is_empty());
        assert!(!manifest.build.binary_sha256.is_empty());

        let telemetry =
            fs::read_to_string(run_dir.join("telemetry.ndjson")).expect("telemetry should exist");
        assert!(telemetry.contains("\"search_profile\":\"desktop_claim_shadow\""));
        assert!(telemetry.contains("\"guidance_style\":\"claim_debt_guided\""));
        assert!(telemetry.contains("\"late_expansion_policy\":\"claim_generic\""));
        assert!(telemetry.contains("\"bucket_policy\":\"structural_generic\""));
        assert!(telemetry.contains("\"exact_screen_reasons\""));

        let steps_dir = run_dir.join("reports").join("steps");
        assert!(steps_dir.join("step-03-narrative.txt").exists());
        assert!(steps_dir.join("step-03-events.ndjson").exists());

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn claim_auto_worker_resolution_uses_memory_headroom() {
        let config_text = include_str!("../../../configs/desktop_claim_shadow_1h.toml");
        let config = pen_search::config::RuntimeConfig::from_toml_str(config_text)
            .expect("config should parse");

        assert_eq!(
            super::resolve_worker_count_for_host(&config, 16, super::gib_to_bytes(16.0)),
            8
        );
    }

    #[test]
    fn non_claim_auto_worker_resolution_stays_cpu_bound() {
        let config_text = include_str!("../../../configs/strict_canon_guarded.toml");
        let config = pen_search::config::RuntimeConfig::from_toml_str(config_text)
            .expect("config should parse");

        assert_eq!(
            super::resolve_worker_count_for_host(&config, 16, super::gib_to_bytes(16.0)),
            12
        );
    }

    #[test]
    fn signed_byte_delta_tracks_observed_gap_direction() {
        assert_eq!(super::signed_byte_delta(1_536, 1_024), 512);
        assert_eq!(super::signed_byte_delta(1_024, 1_536), -512);
    }

    #[test]
    fn failed_partial_claim_run_still_leaves_manifest_and_narrative_artifacts() {
        let root = temp_dir("partial-claim-failure");
        let run_dir = root.join("partial-claim");
        let config_text = include_str!("../../../configs/desktop_claim_shadow_smoke.toml");
        let config = pen_search::config::RuntimeConfig::from_toml_str(config_text)
            .expect("claim config should parse");
        let worker_count = super::resolved_worker_count(&config);
        let manifest_base = build_run_manifest_base(
            "partial-claim",
            config_text,
            &config,
            worker_count,
            super::now_utc().expect("timestamp should render"),
        )
        .expect("manifest base should build");
        let mut writer = RunArtifactWriter::start(
            &run_dir,
            config_text,
            manifest_base,
            &config,
            worker_count,
            planned_run_mode(4).as_str(),
            config.mode.search_profile.as_str(),
            super::now_utc().expect("timestamp should render"),
            Vec::new(),
            None,
        )
        .expect("writer should initialize");
        let generated =
            generate_steps_with_config_and_runtime(3, &config, FrontierRuntimeLimits::unlimited())
                .expect("claim steps should generate");
        for step in &generated.steps {
            writer.persist_step(step).expect("step should persist");
        }
        writer
            .finalize_failed(&anyhow::anyhow!("simulated allocator abort"))
            .expect("failure status should persist");

        let manifest: RunManifestV1 = serde_json::from_str(
            &fs::read_to_string(run_dir.join("run.json")).expect("manifest should exist"),
        )
        .expect("manifest should parse");
        assert_eq!(manifest.status, RunStatus::Failed);
        assert!(manifest.failure_note.contains("simulated allocator abort"));
        assert_eq!(manifest.position.completed_step, 3);
        assert_eq!(manifest.position.frontier_epoch, 0);

        let steps_dir = run_dir.join("reports").join("steps");
        assert!(steps_dir.join("step-03-summary.json").exists());
        assert!(steps_dir.join("step-03-narrative.txt").exists());
        assert!(steps_dir.join("step-03-events.ndjson").exists());
        assert!(
            run_dir
                .join("checkpoints")
                .join("steps")
                .join("step-03.json")
                .exists()
        );

        let latest =
            fs::read_to_string(run_dir.join("reports").join("latest.txt")).expect("latest report");
        assert!(latest.contains("completed_step: 3"));

        let telemetry =
            fs::read_to_string(run_dir.join("telemetry.ndjson")).expect("telemetry should exist");
        assert!(telemetry.contains("\"event\":\"run_started\""));
        assert!(telemetry.contains("\"event\":\"step_accepted\""));
        assert!(telemetry.contains("\"event\":\"run_failed\""));

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn failed_partial_late_run_still_leaves_frontier_snapshot() {
        let root = temp_dir("partial-frontier-failure");
        let run_dir = root.join("partial-frontier");
        let config_text = include_str!("../../../configs/debug.toml");
        let config =
            pen_search::config::RuntimeConfig::from_toml_str(config_text).expect("debug config");
        let worker_count = super::resolved_worker_count(&config);
        let manifest_base = build_run_manifest_base(
            "partial-frontier",
            config_text,
            &config,
            worker_count,
            super::now_utc().expect("timestamp should render"),
        )
        .expect("manifest base should build");
        let mut writer = RunArtifactWriter::start(
            &run_dir,
            config_text,
            manifest_base,
            &config,
            worker_count,
            planned_run_mode(4).as_str(),
            config.mode.search_profile.as_str(),
            super::now_utc().expect("timestamp should render"),
            Vec::new(),
            None,
        )
        .expect("writer should initialize");
        let generated =
            generate_steps_with_config_and_runtime(4, &config, FrontierRuntimeLimits::unlimited())
                .expect("debug steps should generate");
        for step in &generated.steps {
            writer.persist_step(step).expect("step should persist");
        }
        writer
            .finalize_failed(&anyhow::anyhow!("simulated allocator abort"))
            .expect("failure status should persist");

        let manifest: RunManifestV1 = serde_json::from_str(
            &fs::read_to_string(run_dir.join("run.json")).expect("manifest should exist"),
        )
        .expect("manifest should parse");
        assert_eq!(manifest.status, RunStatus::Failed);
        assert_eq!(manifest.position.completed_step, 4);
        assert_eq!(manifest.position.frontier_epoch, 1);

        let band_index = u32::from(generated.steps[3].accepted.clause_kappa);
        assert!(
            frontier_checkpoint_dir(&run_dir, 4, band_index)
                .join("frontier.manifest.json")
                .exists()
        );

        fs::remove_dir_all(root).ok();
    }
}
