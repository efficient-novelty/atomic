use crate::engine::AtomicSearchStep;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrativeEventKind {
    StepOpen,
    SurfaceSummary,
    PrefixSummary,
    MemoSummary,
    FrontierSummary,
    AcceptanceSummary,
    StepComplete,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct NarrativeProgressSnapshot {
    pub elapsed_millis: u64,
    pub generated_surface: u64,
    pub exact_screened_surface: u64,
    pub full_telescopes_evaluated: u64,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NarrativeEvent {
    pub step_index: u32,
    pub ordinal: u16,
    pub kind: NarrativeEventKind,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<NarrativeProgressSnapshot>,
}

pub fn build_step_narrative_events(step: &AtomicSearchStep) -> Vec<NarrativeEvent> {
    let progress = step_progress(step);
    let mut ordinal = 0u16;
    let mut push_event = |events: &mut Vec<NarrativeEvent>,
                          kind: NarrativeEventKind,
                          message: String,
                          detail: Option<String>| {
        ordinal = ordinal.saturating_add(1);
        events.push(NarrativeEvent {
            step_index: step.step_index,
            ordinal,
            kind,
            message,
            detail,
            progress: Some(progress.clone()),
        });
    };

    let mut events = Vec::new();
    push_event(
        &mut events,
        NarrativeEventKind::StepOpen,
        format!(
            "opened step {} with bar {} and clause band {}..{}",
            step.step_index,
            step.objective_bar,
            step.admissibility.min_clause_kappa,
            step.admissibility.max_clause_kappa
        ),
        Some(format!(
            "mode={} exact_legality_rejections={} structural_cap_rejections={} admitted_focus_aligned={} admitted_deprioritized={}",
            admissibility_mode_label(step),
            step.admissibility_diagnostics.exact_legality_rejections,
            step.admissibility_diagnostics
                .structural_debt_cap_rejections,
            step.admissibility_diagnostics.admitted_focus_aligned,
            step.admissibility_diagnostics.admitted_deprioritized
        )),
    );
    push_event(
        &mut events,
        NarrativeEventKind::SurfaceSummary,
        format!(
            "generated {} structural candidates; {} were well formed and {} were rejected by admissibility",
            progress.generated_surface, step.well_formed_candidates, step.admissibility_rejections
        ),
        Some(format!(
            "malformed={} full_evaluated={} canonical={} semantically_minimal={}",
            step.malformed_rejections,
            step.full_telescopes_evaluated,
            step.canonical_candidates,
            step.semantically_minimal_candidates
        )),
    );
    if step.prefix_states_explored > 0 {
        push_event(
            &mut events,
            NarrativeEventKind::PrefixSummary,
            format!(
                "prefix frontier created={} explored={} merged={} exact_pruned={} heuristic_dropped={}",
                step.prefixes_created,
                step.prefix_states_explored,
                step.prefix_states_merged_by_signature,
                step.prefix_states_exact_pruned,
                step.prefix_states_heuristic_dropped
            ),
            Some(format!(
                "hot_states={} cold_states={} frontier_window={}",
                step.prefix_frontier_hot_states,
                step.prefix_frontier_cold_states,
                step.frontier_window.total_len()
            )),
        );
    }
    if uses_prefix_memo(step) {
        push_event(
            &mut events,
            NarrativeEventKind::MemoSummary,
            format!(
                "memo path hits: legality={} active_window={} terminal_completion={} partial_bounds={}",
                step.incremental_legality_cache_hits,
                step.incremental_active_window_clause_filter_hits,
                step.incremental_terminal_prefix_completion_hits,
                step.incremental_partial_prefix_bound_hits
            ),
            Some(format!(
                "connectivity_shortcuts={} clause_family_prunes={} terminal_rank_prunes={} terminal_prefix_bar_prunes={}",
                step.incremental_connectivity_shortcuts,
                step.incremental_clause_family_prunes
                    + step.incremental_active_window_clause_filter_prunes,
                step.incremental_terminal_rank_prunes,
                step.incremental_terminal_prefix_bar_prunes
            )),
        );
    }
    push_event(
        &mut events,
        NarrativeEventKind::FrontierSummary,
        format!(
            "retained {} candidates with frontier hot={} cold={} dropped={}",
            step.retained_candidates.len(),
            step.frontier_window.hot.len(),
            step.frontier_window.cold.len(),
            step.frontier_retention.dropped_candidates.len()
        ),
        Some(format!(
            "pressure_state={} pressure_action={} resident_cold={} spill_backed={}",
            step.frontier_pressure.governor_state.as_str(),
            step.frontier_pressure.pressure_action.as_str(),
            step.frontier_retention.resident_cold_candidates.len(),
            step.frontier_retention.spill_cold_candidates.len()
        )),
    );
    let bar_clearers = step
        .retained_candidates
        .iter()
        .filter(|candidate| candidate.rho >= step.objective_bar)
        .count();
    push_event(
        &mut events,
        NarrativeEventKind::AcceptanceSummary,
        format!(
            "accepted {} with overshoot {} (nu={} kappa={})",
            step.accepted.candidate_hash,
            step.accepted.rho - step.objective_bar,
            step.accepted.nu,
            step.accepted.clause_kappa
        ),
        Some(format!(
            "bar_clearers={} near_misses={} candidate_hash={} canonical_hash={}",
            bar_clearers,
            step.near_misses.len(),
            step.accepted.candidate_hash,
            step.accepted.canonical_hash
        )),
    );
    push_event(
        &mut events,
        NarrativeEventKind::StepComplete,
        format!(
            "completed step {} in {} ms",
            step.step_index, step.search_timing.step_wall_clock_millis
        ),
        Some(format!(
            "discovery_ms={} frontier_plan_ms={} selection_ms={}",
            step.search_timing.candidate_discovery_wall_clock_millis,
            step.search_timing
                .prefix_frontier_planning_wall_clock_millis,
            step.search_timing.selection_wall_clock_millis
        )),
    );

    events
}

fn step_progress(step: &AtomicSearchStep) -> NarrativeProgressSnapshot {
    NarrativeProgressSnapshot {
        elapsed_millis: step.search_timing.step_wall_clock_millis,
        generated_surface: if step.prefix_states_explored > 0 {
            step.prefixes_created.max(step.enumerated_candidates) as u64
        } else {
            step.enumerated_candidates as u64
        },
        exact_screened_surface: (step.prefix_states_exact_pruned + step.full_telescopes_evaluated)
            as u64,
        full_telescopes_evaluated: step.full_telescopes_evaluated as u64,
    }
}

fn admissibility_mode_label(step: &AtomicSearchStep) -> &'static str {
    match step.admissibility.mode {
        pen_type::admissibility::AdmissibilityMode::Guarded => "guarded",
        pen_type::admissibility::AdmissibilityMode::RelaxedShadow => "relaxed_shadow",
        pen_type::admissibility::AdmissibilityMode::RealisticShadow => "realistic_shadow",
    }
}

fn uses_prefix_memo(step: &AtomicSearchStep) -> bool {
    step.incremental_legality_cache_hits > 0
        || step.incremental_connectivity_shortcuts > 0
        || step.incremental_connectivity_fallbacks > 0
        || step.incremental_connectivity_prunes > 0
        || step.incremental_clause_family_filter_hits > 0
        || step.incremental_clause_family_prunes > 0
        || step.incremental_active_window_clause_filter_hits > 0
        || step.incremental_active_window_clause_filter_prunes > 0
        || step.incremental_terminal_clause_filter_hits > 0
        || step.incremental_terminal_clause_filter_prunes > 0
        || step.incremental_trivial_derivability_hits > 0
        || step.incremental_trivial_derivability_prunes > 0
        || step.incremental_terminal_admissibility_hits > 0
        || step.incremental_terminal_admissibility_rejections > 0
        || step.incremental_terminal_prefix_completion_hits > 0
        || step.incremental_terminal_prefix_rank_hits > 0
        || step.incremental_terminal_rank_prunes > 0
        || step.incremental_partial_prefix_bound_hits > 0
        || step.incremental_partial_prefix_bound_checks > 0
        || step.incremental_partial_prefix_bound_prunes > 0
        || step.incremental_terminal_prefix_bar_prunes > 0
}
