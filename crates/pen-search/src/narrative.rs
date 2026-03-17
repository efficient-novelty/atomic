use crate::engine::AtomicSearchStep;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StepPhase {
    Scout,
    BreadthHarvest,
    Materialize,
    ProofClose,
    Seal,
}

impl StepPhase {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Scout => "scout",
            Self::BreadthHarvest => "breadth_harvest",
            Self::Materialize => "materialize",
            Self::ProofClose => "proof_close",
            Self::Seal => "seal",
        }
    }

    pub const fn pulse_slot(self) -> usize {
        match self {
            Self::Scout => 0,
            Self::BreadthHarvest => 1,
            Self::Materialize => 2,
            Self::ProofClose => 3,
            Self::Seal => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrativeEventKind {
    StepOpen,
    PhaseChange,
    BudgetPulse,
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

pub fn narrative_progress_snapshot(
    elapsed_millis: u64,
    generated_surface: u64,
    exact_screened_surface: u64,
    full_telescopes_evaluated: u64,
) -> NarrativeProgressSnapshot {
    NarrativeProgressSnapshot {
        elapsed_millis,
        generated_surface,
        exact_screened_surface,
        full_telescopes_evaluated,
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct NarrativeEvent {
    pub step_index: u32,
    pub ordinal: u16,
    pub kind: NarrativeEventKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phase: Option<StepPhase>,
    pub message: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<NarrativeProgressSnapshot>,
}

#[derive(Clone, Debug, Default)]
pub struct NarrativeRecorder {
    step_index: u32,
    ordinal: u16,
    events: Vec<NarrativeEvent>,
}

impl NarrativeRecorder {
    pub fn new(step_index: u32) -> Self {
        Self {
            step_index,
            ordinal: 0,
            events: Vec::new(),
        }
    }

    pub fn from_existing(step_index: u32, events: Vec<NarrativeEvent>) -> Self {
        let ordinal = events.last().map(|event| event.ordinal).unwrap_or(0);
        Self {
            step_index,
            ordinal,
            events,
        }
    }

    pub fn has_kind(&self, kind: NarrativeEventKind) -> bool {
        self.events.iter().any(|event| event.kind == kind)
    }

    pub fn push(
        &mut self,
        kind: NarrativeEventKind,
        phase: Option<StepPhase>,
        message: impl Into<String>,
        detail: Option<String>,
        progress: Option<NarrativeProgressSnapshot>,
    ) {
        self.ordinal = self.ordinal.saturating_add(1);
        self.events.push(NarrativeEvent {
            step_index: self.step_index,
            ordinal: self.ordinal,
            kind,
            phase,
            message: message.into(),
            detail,
            progress,
        });
    }

    pub fn finish(self) -> Vec<NarrativeEvent> {
        self.events
    }

    pub fn events(&self) -> &[NarrativeEvent] {
        &self.events
    }
}

pub fn build_step_narrative_events(step: &AtomicSearchStep) -> Vec<NarrativeEvent> {
    append_step_narrative_events(step, Vec::new())
}

pub fn append_step_narrative_events(
    step: &AtomicSearchStep,
    seed_events: Vec<NarrativeEvent>,
) -> Vec<NarrativeEvent> {
    let progress = step_progress(step);
    let mut recorder = NarrativeRecorder::from_existing(step.step_index, seed_events);

    if !recorder.has_kind(NarrativeEventKind::StepOpen) {
        recorder.push(
            NarrativeEventKind::StepOpen,
            None,
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
                step.admissibility_diagnostics.structural_debt_cap_rejections,
                step.admissibility_diagnostics.admitted_focus_aligned,
                step.admissibility_diagnostics.admitted_deprioritized
            )),
            Some(progress.clone()),
        );
    }

    recorder.push(
        NarrativeEventKind::SurfaceSummary,
        None,
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
        Some(progress.clone()),
    );
    if step.prefix_states_explored > 0 {
        recorder.push(
            NarrativeEventKind::PrefixSummary,
            None,
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
            Some(progress.clone()),
        );
    }
    if uses_prefix_memo(step) {
        recorder.push(
            NarrativeEventKind::MemoSummary,
            None,
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
            Some(progress.clone()),
        );
    }
    recorder.push(
        NarrativeEventKind::FrontierSummary,
        None,
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
        Some(progress.clone()),
    );
    let bar_clearers = step
        .retained_candidates
        .iter()
        .filter(|candidate| candidate.rho >= step.objective_bar)
        .count();
    recorder.push(
        NarrativeEventKind::AcceptanceSummary,
        Some(StepPhase::Seal),
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
        Some(progress.clone()),
    );
    recorder.push(
        NarrativeEventKind::StepComplete,
        Some(StepPhase::Seal),
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
        Some(progress),
    );

    recorder.finish()
}

pub(crate) fn generated_surface_from_counts(
    prefixes_created: usize,
    enumerated_candidates: usize,
) -> u64 {
    prefixes_created.max(enumerated_candidates) as u64
}

pub(crate) fn exact_screened_surface_from_counts(
    prefix_states_exact_pruned: usize,
    full_telescopes_evaluated: usize,
) -> u64 {
    (prefix_states_exact_pruned + full_telescopes_evaluated) as u64
}

fn step_progress(step: &AtomicSearchStep) -> NarrativeProgressSnapshot {
    narrative_progress_snapshot(
        step.search_timing.step_wall_clock_millis,
        generated_surface_from_counts(step.prefixes_created, step.enumerated_candidates),
        exact_screened_surface_from_counts(
            step.prefix_states_exact_pruned,
            step.full_telescopes_evaluated,
        ),
        step.full_telescopes_evaluated as u64,
    )
}

fn admissibility_mode_label(step: &AtomicSearchStep) -> &'static str {
    match step.admissibility.mode {
        pen_type::admissibility::AdmissibilityMode::Guarded => "guarded",
        pen_type::admissibility::AdmissibilityMode::RelaxedShadow => "relaxed_shadow",
        pen_type::admissibility::AdmissibilityMode::RealisticShadow => "realistic_shadow",
        pen_type::admissibility::AdmissibilityMode::DemoBreadthShadow => "demo_breadth_shadow",
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
