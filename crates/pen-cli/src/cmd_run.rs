use crate::cli::RunArgs;
use crate::output::{OutputStyle, render_run_output};
use crate::report::{
    GeneratedSteps, StepReport, annotate_search_profile, generate_steps_for_profile_with_runtime,
    write_step_reports,
};
use anyhow::{Context, Result, bail};
use pen_core::hash::blake3_hex;
use pen_core::ids::{ClauseId, ObligationSetId, StateId};
use pen_core::library::{LibrarySnapshot, LibrarySnapshotEntry};
use pen_search::config::RuntimeConfig;
use pen_search::diversify::FrontierRuntimeLimits;
use pen_search::frontier::FrontierWindow;
use pen_search::priority::{PriorityInputs, build_priority_key};
use pen_search::resume::CurrentCompat;
use pen_search::scheduler::build_schedule;
use pen_search::state::{FrontierStateRecV1, PrefixState};
use pen_store::frontier::{frontier_checkpoint_dir, write_frontier_manifest};
use pen_store::layout::{FRONTIER_RECORD_LAYOUT_ID, SCHEMA_VERSION_V1};
use pen_store::manifest::{
    BitBand, CheckpointCompat, ConfigFingerprint, FrontierManifestV1, HostInfo, NearMiss,
    ResumeCompatible, RunArtifacts, RunCompat, RunManifestV1, RunPosition, RunStatus,
    StepCheckpointV1, StepObjective, StepStats,
};
use pen_store::memory::GovernorConfig;
use pen_store::spill::{FrontierRuntimeInput, SpillConfig, persist_frontier_runtime};
use pen_store::sqlite::{FrontierGenerationRow, MetadataDb};
use pen_store::telemetry::TelemetryEventV1;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
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
    let worker_count = resolved_worker_count(&config);
    let GeneratedSteps { mode, mut steps } = generate_steps_for_profile_with_runtime(
        until_step,
        config.objective.window_depth,
        config.mode.search_profile,
        frontier_runtime_limits(&config, worker_count),
    )?;
    annotate_search_profile(&mut steps, config.mode.search_profile);
    let created_utc = now_utc()?;
    let updated_utc = created_utc.clone();
    let manifest = build_run_manifest(&run_id, &config_text, &steps, created_utc, updated_utc);

    write_run_artifacts(
        &run_dir,
        &config_text,
        &manifest,
        &steps,
        worker_count,
        &config,
        mode.as_str(),
        config.mode.search_profile.as_str(),
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
    worker_count: u16,
    config: &RuntimeConfig,
    mode: &str,
    search_profile: &str,
) -> Result<()> {
    fs::create_dir_all(run_dir).with_context(|| format!("create {}", run_dir.display()))?;
    fs::create_dir_all(run_dir.join("reports").join("steps"))?;
    fs::create_dir_all(run_dir.join("checkpoints").join("steps"))?;
    fs::create_dir_all(run_dir.join("checkpoints").join("frontier"))?;

    fs::write(run_dir.join("config.toml"), config_text)
        .with_context(|| format!("write {}", run_dir.join("config.toml").display()))?;
    let metadata = MetadataDb::open(&run_dir.join("meta.sqlite3"))?;

    write_json_file(&run_dir.join("run.json"), manifest)?;
    write_step_reports(run_dir, steps)?;
    write_step_checkpoints(run_dir, manifest, steps, config.objective.window_depth)?;
    write_frontier_snapshots(run_dir, manifest, steps, worker_count, config, &metadata)?;
    write_telemetry(run_dir, manifest, steps, mode, search_profile)?;

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
            active_band,
            frontier_epoch,
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
    let mut lines = Vec::new();
    lines.push(serde_json::to_string(&TelemetryEventV1 {
        schema_version: SCHEMA_VERSION_V1,
        run_id: manifest.run_id.clone(),
        event: "run_started".to_owned(),
        step_index: None,
        payload: json!({
            "created_utc": manifest.created_utc,
            "mode": mode,
            "search_profile": search_profile,
        }),
    })?);

    for step in steps {
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
        lines.push(serde_json::to_string(&TelemetryEventV1 {
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
                "prune_samples": prune_sample_counts,
                "frontier_pressure": {
                    "governor_state": step.frontier_pressure.governor_state.as_str(),
                    "pressure_action": step.frontier_pressure.pressure_action.as_str(),
                    "rss_bytes": step.frontier_pressure.rss_bytes,
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

pub(crate) fn resolved_worker_count(config: &RuntimeConfig) -> u16 {
    let logical_cpus = std::thread::available_parallelism()
        .map(|count| count.get() as u16)
        .unwrap_or(1);
    config.search.resolve_workers(logical_cpus)
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

fn write_frontier_snapshots(
    run_dir: &Path,
    run_manifest: &RunManifestV1,
    steps: &[StepReport],
    worker_count: u16,
    config: &RuntimeConfig,
    metadata: &MetadataDb,
) -> Result<()> {
    for (frontier_epoch, step) in steps.iter().filter(|step| step.step_index >= 4).enumerate() {
        let band_index = u32::from(step.accepted.clause_kappa);
        let frontier_epoch =
            u32::try_from(frontier_epoch + 1).expect("frontier epoch exceeded u32");
        let frontier_dir = frontier_checkpoint_dir(run_dir, step.step_index, band_index);
        let (mut window, dedupe_keys) = frontier_window_for_step(step, worker_count);
        if window.total_len() == 0 {
            continue;
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
                prefix_states_merged_by_signature: step
                    .search_stats
                    .prefix_states_merged_by_signature
                    as u64,
                prefix_states_exact_pruned: step.search_stats.prefix_states_exact_pruned as u64,
                prefix_states_heuristic_dropped: step.search_stats.prefix_states_heuristic_dropped
                    as u64,
                incremental_legality_cache_hits: step.search_stats.incremental_legality_cache_hits
                    as u64,
                incremental_connectivity_shortcuts: step
                    .search_stats
                    .incremental_connectivity_shortcuts
                    as u64,
                incremental_connectivity_fallbacks: step
                    .search_stats
                    .incremental_connectivity_fallbacks
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
    }

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
