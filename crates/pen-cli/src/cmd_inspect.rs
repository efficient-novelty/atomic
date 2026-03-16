use crate::cli::InspectArgs;
use crate::cmd_run::current_search_compat;
use crate::report::{
    LateStepClaimStatus, StepReport, load_step_reports, render_debug_report,
    render_replay_ablation, render_standard_report, summarize_prune_reports,
};
use anyhow::{Context, Result, bail};
use pen_search::resume::decide_resume;
use pen_store::frontier::read_frontier_manifest;
use pen_store::manifest::{FrontierManifestV1, RunManifestV1};
use pen_store::spill::read_frontier_cache_blob;
use std::fs;
use std::path::{Path, PathBuf};

pub fn inspect(args: InspectArgs) -> Result<String> {
    let path = absolute(args.path)?;

    if path.is_dir() && path.join("run.json").exists() {
        return inspect_run_dir(&path);
    }
    if path.file_name().and_then(|name| name.to_str()) == Some("run.json") {
        return inspect_run_dir(
            path.parent()
                .context("run.json must have a parent directory")?,
        );
    }
    if path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with("-summary.json"))
    {
        let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let step: StepReport = serde_json::from_str(&text).context("parse step summary")?;
        let late_step_claim = if step.canon_evidence.late_step_claim.status
            == LateStepClaimStatus::NotApplicable
        {
            String::new()
        } else {
            format!(
                "\nlate_step_claim: {} step {} {} nu={} matches_current_acceptance={}",
                match step.canon_evidence.late_step_claim.status {
                    LateStepClaimStatus::NotApplicable => "not_applicable",
                    LateStepClaimStatus::ExecutableCanon => "executable_canon",
                    LateStepClaimStatus::HistoricalProvenanceOnly => "historical_provenance_only",
                },
                step.canon_evidence.late_step_claim.adopted_step,
                step.canon_evidence.late_step_claim.adopted_label,
                step.canon_evidence.late_step_claim.adopted_nu,
                step.canon_evidence.late_step_claim.matches_accepted
            )
        };
        let frontier_details = if step.frontier_pressure
            == pen_search::diversify::FrontierPressure::default()
        {
            String::new()
        } else {
            format!(
                "\nretention_focus: {}\nfrontier_pressure: state={} action={} rss_bytes={} hot_bytes={} cold_bytes={} dedupe_bytes={} requested_cold={} retained_cold={} resident_cold={} spill_backed={} dropped={}",
                match step.frontier_policy.focus {
                    pen_type::obligations::RetentionFocus::OpenBand => "open_band",
                    pen_type::obligations::RetentionFocus::Former => "former",
                    pen_type::obligations::RetentionFocus::Hit => "hit",
                    pen_type::obligations::RetentionFocus::Axiomatic => "axiomatic",
                    pen_type::obligations::RetentionFocus::Modal => "modal",
                    pen_type::obligations::RetentionFocus::Temporal => "temporal",
                },
                step.frontier_pressure.governor_state.as_str(),
                step.frontier_pressure.pressure_action.as_str(),
                step.frontier_pressure.rss_bytes,
                step.frontier_pressure.hot_frontier_bytes,
                step.frontier_pressure.cold_frontier_bytes,
                step.frontier_pressure.dedupe_bytes,
                step.frontier_pressure.requested_cold_limit,
                step.frontier_pressure.retained_cold_limit,
                step.frontier_pressure.resident_cold_limit,
                step.frontier_pressure.spill_backed_cold_records,
                step.frontier_pressure.dropped_cold_records
            )
        };
        let replay_ablation = render_replay_ablation(&step.replay_ablation)
            .map(|ablation| format!("\nreplay_ablation: {ablation}"))
            .unwrap_or_default();
        let timing_details = if step.provenance == crate::report::StepProvenance::ReferenceReplay
            && step.search_stats.search_timing == pen_store::manifest::SearchTiming::default()
        {
            String::new()
        } else {
            format!(
                "\ntiming: step_ms={} discovery_ms={} frontier_plan_ms={} selection_ms={}",
                step.search_stats.search_timing.step_wall_clock_millis,
                step.search_stats
                    .search_timing
                    .candidate_discovery_wall_clock_millis,
                step.search_stats
                    .search_timing
                    .prefix_frontier_planning_wall_clock_millis,
                step.search_stats.search_timing.selection_wall_clock_millis
            )
        };
        let prune_summary = if step.prune_reports.is_empty() {
            String::new()
        } else {
            format!(
                "\nprune_samples: {}",
                summarize_prune_reports(&step.prune_reports)
            )
        };
        let prefix_frontier = if step.search_stats.prefix_states_explored == 0 {
            String::new()
        } else {
            let prefix_memo = if step.search_stats.incremental_legality_cache_hits == 0
                && step.search_stats.incremental_connectivity_shortcuts == 0
                && step.search_stats.incremental_connectivity_fallbacks == 0
                && step.search_stats.incremental_connectivity_prunes == 0
                && step.search_stats.incremental_clause_family_filter_hits == 0
                && step.search_stats.incremental_clause_family_prunes == 0
                && step
                    .search_stats
                    .incremental_active_window_clause_filter_hits
                    == 0
                && step
                    .search_stats
                    .incremental_active_window_clause_filter_prunes
                    == 0
                && step.search_stats.incremental_terminal_clause_filter_hits == 0
                && step.search_stats.incremental_terminal_clause_filter_prunes == 0
                && step.search_stats.incremental_trivial_derivability_hits == 0
                && step.search_stats.incremental_trivial_derivability_prunes == 0
                && step.search_stats.incremental_terminal_admissibility_hits == 0
                && step
                    .search_stats
                    .incremental_terminal_admissibility_rejections
                    == 0
                && step
                    .search_stats
                    .incremental_terminal_prefix_completion_hits
                    == 0
                && step.search_stats.incremental_partial_prefix_bound_checks == 0
                && step.search_stats.incremental_partial_prefix_bound_prunes == 0
                && step.search_stats.incremental_terminal_prefix_bar_prunes == 0
            {
                String::new()
            } else {
                format!(
                    "\nprefix_memo: legality_hits={} connectivity_shortcuts={} connectivity_fallbacks={} connectivity_prunes={} clause_family_hits={} clause_family_prunes={} active_window_filter_hits={} active_window_filter_prunes={} terminal_clause_filter_hits={} terminal_clause_filter_prunes={} trivial_derivability_hits={} trivial_derivability_prunes={} terminal_admissibility_hits={} terminal_admissibility_rejections={} terminal_prefix_completion_hits={} partial_prefix_bound_checks={} partial_prefix_bound_prunes={} terminal_prefix_bar_prunes={}",
                    step.search_stats.incremental_legality_cache_hits,
                    step.search_stats.incremental_connectivity_shortcuts,
                    step.search_stats.incremental_connectivity_fallbacks,
                    step.search_stats.incremental_connectivity_prunes,
                    step.search_stats.incremental_clause_family_filter_hits,
                    step.search_stats.incremental_clause_family_prunes,
                    step.search_stats
                        .incremental_active_window_clause_filter_hits,
                    step.search_stats
                        .incremental_active_window_clause_filter_prunes,
                    step.search_stats.incremental_terminal_clause_filter_hits,
                    step.search_stats.incremental_terminal_clause_filter_prunes,
                    step.search_stats.incremental_trivial_derivability_hits,
                    step.search_stats.incremental_trivial_derivability_prunes,
                    step.search_stats.incremental_terminal_admissibility_hits,
                    step.search_stats
                        .incremental_terminal_admissibility_rejections,
                    step.search_stats
                        .incremental_terminal_prefix_completion_hits,
                    step.search_stats.incremental_partial_prefix_bound_checks,
                    step.search_stats.incremental_partial_prefix_bound_prunes,
                    step.search_stats.incremental_terminal_prefix_bar_prunes
                )
            };
            format!(
                "\nprefix_frontier: created={} explored={} merged={} exact_pruned={} heuristic_dropped={} hot={} cold={}{}",
                step.search_stats.prefixes_created,
                step.search_stats.prefix_states_explored,
                step.search_stats.prefix_states_merged_by_signature,
                step.search_stats.prefix_states_exact_pruned,
                step.search_stats.prefix_states_heuristic_dropped,
                step.search_stats.prefix_frontier_hot_states,
                step.search_stats.prefix_frontier_cold_states,
                prefix_memo
            )
        };
        return Ok(format!(
            "step {} ({})\nsearch_profile: {}\nnu: {}\nkappa: {}\ncharged_kappa: {}\nrho: {}\nbar: {}\nbar_distance: {}\ncandidate: {}\ncanonical: {}{}{}{}{}{}{}",
            step.step_index,
            step.label,
            step.search_profile.as_str(),
            step.accepted.nu,
            step.accepted.clause_kappa,
            step.canon_evidence.charged_clause_kappa,
            step.accepted.rho,
            step.objective_bar,
            step.canon_evidence.bar_distance,
            step.accepted.candidate_hash,
            step.accepted.canonical_hash,
            late_step_claim,
            replay_ablation,
            timing_details,
            prune_summary,
            prefix_frontier,
            frontier_details,
        ));
    }
    if path.file_name().and_then(|name| name.to_str()) == Some("frontier.manifest.json") {
        let frontier: FrontierManifestV1 = read_frontier_manifest(&path)?;
        let decision = decide_resume(&current_search_compat(), &frontier);
        let frontier_dir = path
            .parent()
            .context("frontier manifest must have a parent directory")?;
        let cache_blob = read_frontier_cache_blob(frontier_dir, &frontier.files.cache_blob)?;
        let cache_lines = cache_blob
            .map(|cache| {
                format!(
                    "\nfrontier_epoch: {}\nworker_count: {}\npriority_heads: {:?}\nspill_generation: {}\ngovernor_state: {}\npressure_action: {}\nresident_cold_records: {}\nspilled_cold_records: {}\ncache_blob: {}",
                    frontier.frontier_epoch,
                    frontier.scheduler.worker_count,
                    frontier.scheduler.priority_heads,
                    frontier.scheduler.spill_generation,
                    cache.governor_state.as_str(),
                    cache.pressure_action.as_str(),
                    cache.resident_cold_records,
                    cache.spilled_cold_records,
                    frontier.files.cache_blob
                )
            })
            .unwrap_or_else(|| {
                format!(
                    "\nfrontier_epoch: {}\nworker_count: {}\npriority_heads: {:?}\nspill_generation: {}",
                    frontier.frontier_epoch,
                    frontier.scheduler.worker_count,
                    frontier.scheduler.priority_heads,
                    frontier.scheduler.spill_generation
                )
            });
        return Ok(format!(
            "frontier step {} band {}\nprefix_created: {}\nprefix_explored: {}\nprefix_merged: {}\nprefix_exact_pruned: {}\nprefix_heuristic_dropped: {}\nlegality_hits: {}\nconnectivity_shortcuts: {}\nconnectivity_fallbacks: {}\nconnectivity_prunes: {}\nclause_family_hits: {}\nclause_family_prunes: {}\nactive_window_filter_hits: {}\nactive_window_filter_prunes: {}\nterminal_clause_filter_hits: {}\nterminal_clause_filter_prunes: {}\ntrivial_derivability_hits: {}\ntrivial_derivability_prunes: {}\nterminal_admissibility_hits: {}\nterminal_admissibility_rejections: {}\nterminal_prefix_completion_hits: {}\npartial_prefix_bound_checks: {}\npartial_prefix_bound_prunes: {}\nterminal_prefix_bar_prunes: {}\nmemory_snapshot: rss_bytes={} hot_frontier_bytes={} interner_bytes={} dedupe_bytes={} cache_bytes={}\nhot_states: {}\ncold_states: {}\ndedupe_keys: {}\nresume_decision: {:?}{}",
            frontier.step_index,
            frontier.band_index,
            frontier.counts.prefixes_created,
            frontier.counts.prefix_states_explored,
            frontier.counts.prefix_states_merged_by_signature,
            frontier.counts.prefix_states_exact_pruned,
            frontier.counts.prefix_states_heuristic_dropped,
            frontier.counts.incremental_legality_cache_hits,
            frontier.counts.incremental_connectivity_shortcuts,
            frontier.counts.incremental_connectivity_fallbacks,
            frontier.counts.incremental_connectivity_prunes,
            frontier.counts.incremental_clause_family_filter_hits,
            frontier.counts.incremental_clause_family_prunes,
            frontier.counts.incremental_active_window_clause_filter_hits,
            frontier
                .counts
                .incremental_active_window_clause_filter_prunes,
            frontier.counts.incremental_terminal_clause_filter_hits,
            frontier.counts.incremental_terminal_clause_filter_prunes,
            frontier.counts.incremental_trivial_derivability_hits,
            frontier.counts.incremental_trivial_derivability_prunes,
            frontier.counts.incremental_terminal_admissibility_hits,
            frontier
                .counts
                .incremental_terminal_admissibility_rejections,
            frontier.counts.incremental_terminal_prefix_completion_hits,
            frontier.counts.incremental_partial_prefix_bound_checks,
            frontier.counts.incremental_partial_prefix_bound_prunes,
            frontier.counts.incremental_terminal_prefix_bar_prunes,
            frontier.memory_snapshot.rss_bytes,
            frontier.memory_snapshot.hot_frontier_bytes,
            frontier.memory_snapshot.interner_bytes,
            frontier.memory_snapshot.dedupe_bytes,
            frontier.memory_snapshot.cache_bytes,
            frontier.counts.hot_states,
            frontier.counts.cold_states,
            frontier.counts.dedupe_keys,
            decision,
            cache_lines
        ));
    }

    bail!("unsupported inspect target: {}", path.display())
}

fn inspect_run_dir(run_dir: &Path) -> Result<String> {
    let manifest_path = run_dir.join("run.json");
    let run_text = fs::read_to_string(&manifest_path)
        .with_context(|| format!("read {}", manifest_path.display()))?;
    let manifest: RunManifestV1 = serde_json::from_str(&run_text).context("parse run manifest")?;
    let steps = load_step_reports(run_dir)?;

    Ok(format!(
        "{}\n\n{}",
        render_standard_report(&manifest.run_id, &steps),
        render_debug_report(&manifest.run_id, &steps)
    ))
}

fn absolute(path: PathBuf) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}
