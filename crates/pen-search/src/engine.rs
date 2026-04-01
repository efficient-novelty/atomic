use crate::accept::{
    AcceptanceOutcome, acceptance_rank, acceptance_rank_for_telescope, select_acceptance,
};
use crate::bounds::PrefixBound;
use crate::branch_bound::{AcceptRank, better_rank};
use crate::config::{DemoConfig, RuntimeConfig, SearchProfile};
use crate::diversify::{FrontierPressure, FrontierRuntimeLimits, plan_pressure_cold_retention};
use crate::enumerate::{
    ClauseCatalog, EnumerationContext, EnumerationSurfaceDiagnostics, LateFamilySurface,
    build_clause_catalog, enumerate_next_clauses, enumerate_raw_telescopes, enumerate_telescopes,
    raw_clause_catalog_widths,
};
use crate::expand::{
    ExpandedCandidate, evaluate_candidate, evaluate_checked_candidate,
    structural_signals_for_telescope,
};
use crate::frontier::FrontierWindow;
use crate::narrative::{
    NarrativeEvent, NarrativeEventKind, NarrativeProgressSnapshot, NarrativeRecorder, StepPhase,
    append_step_narrative_events, exact_screened_surface_from_counts,
    generated_surface_from_counts, narrative_progress_snapshot,
};
use crate::prefix_cache::{
    PrefixCache, PrefixCandidateGroup, PrefixGroupCandidate, PrefixSignature,
};
use crate::prefix_memo::{
    PartialPrefixBoundDecision, PrefixLegalityCache, PrefixLegalityCacheEntryCounts,
    TerminalConnectivityDecision, TerminalPrefixClauseEvaluation, TerminalPrefixCompletion,
    TerminalPrefixCompletionSummary, TerminalPrefixPrimaryRank, TerminalPrefixSurvivorSketch,
    TerminalPrefixSurvivorSketchEntry,
};
use crate::priority::{PriorityInputs, build_priority_key};
use crate::scheduler::build_schedule;
use crate::state::{FrontierStateRecV1, PrefixState};
use crate::worker::run_worker_batch;
use anyhow::{Result, bail};
use pen_core::encode::expr_bit_length;
use pen_core::ids::{ClauseId, ObligationSetId, StateId};
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_eval::bar::{DiscoveryRecord, compute_bar};
use pen_eval::minimality::analyze_semantic_minimality;
use pen_eval::nu::{
    SingleClauseStructuralNuCaps, SingleClauseStructuralNuContext, TerminalClauseNuFacts,
    structural_nu, structural_nu_single_clause_upper_bound,
};
use pen_store::manifest::{NearMiss, SearchTiming};
use pen_type::admissibility::{
    AdmissibilityDecision, AdmissibilityDecisionClass, AdmissibilityDiagnostics, AdmissibilityMode,
    StrictAdmissibility, assess_strict_admissibility, strict_admissibility_for_mode,
};
use pen_type::check::{CheckResult, check_telescope};
use pen_type::connectivity::TerminalClauseConnectivityFacts;
use pen_type::connectivity::passes_connectivity;
use pen_type::obligations::{RetentionClass, RetentionPolicy, summarize_structural_debt};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[path = "engine_claim_replay.rs"]
pub mod claim_replay;

pub const LIVE_BOOTSTRAP_MAX_STEP: u32 = 15;
const MAX_PRUNE_SAMPLES: usize = 3;
const EXACT_PARTIAL_PREFIX_BOUND_BUDGET: usize = 32;
const DEMO_LATE_FLOOR_START_STEP: u32 = 10;
const DEMO_LATE_SPILL_START_STEP: u32 = 13;
const DEMO_LIVE_RETUNE_COOLDOWN_MILLIS: u64 = 1_000;
const DEMO_LIVE_RETUNE_MIN_ADJUSTMENT_MILLIS: u64 = 1_000;
const CLAIM_OPEN_BAND_ADMISSIBILITY_REASON: &str = "open_band_structural";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FrontierRetentionOutcome {
    pub pressure: FrontierPressure,
    pub resident_cold_candidates: BTreeSet<String>,
    pub spill_cold_candidates: BTreeSet<String>,
    pub dropped_candidates: BTreeSet<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DedupePruneEvidence {
    pub candidate: ExpandedCandidate,
    pub first_seen_candidate_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinimalityPruneEvidence {
    pub candidate: ExpandedCandidate,
    pub admissible_bar_clear_subbundles: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateScoreDistribution {
    pub candidate_count: usize,
    pub clears_bar: usize,
    pub below_bar: usize,
    pub clause_kappa_counts: BTreeMap<u16, usize>,
    pub nu_summary: IntegerDistributionSummary,
    pub rho_summary: RationalDistributionSummary,
}

impl Default for CandidateScoreDistribution {
    fn default() -> Self {
        Self {
            candidate_count: 0,
            clears_bar: 0,
            below_bar: 0,
            clause_kappa_counts: BTreeMap::new(),
            nu_summary: IntegerDistributionSummary::default(),
            rho_summary: RationalDistributionSummary::default(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct IntegerDistributionSummary {
    pub min: u16,
    pub median: u16,
    pub max: u16,
    pub average: Rational,
}

impl Default for IntegerDistributionSummary {
    fn default() -> Self {
        Self {
            min: 0,
            median: 0,
            max: 0,
            average: Rational::zero(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct RationalDistributionSummary {
    pub min: Rational,
    pub median: Rational,
    pub max: Rational,
    pub average: Rational,
}

impl Default for RationalDistributionSummary {
    fn default() -> Self {
        Self {
            min: Rational::zero(),
            median: Rational::zero(),
            max: Rational::zero(),
            average: Rational::zero(),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DemoBreadthHarvestExitReason {
    GeneratedFloorHit,
    ExactScreenedFloorHit,
    ProofCloseReserveProtected,
}

impl DemoBreadthHarvestExitReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeneratedFloorHit => "generated_floor_hit",
            Self::ExactScreenedFloorHit => "exact_screened_floor_hit",
            Self::ProofCloseReserveProtected => "proof_close_reserve_protected",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DemoProofCloseEntryReason {
    BreadthFloorHit,
    ReserveProtected,
    MaterializeReserveHandoff,
    ClosurePressureHandoff,
    SoftCapHandoff,
    CertificationSweep,
}

impl DemoProofCloseEntryReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BreadthFloorHit => "breadth_floor_hit",
            Self::ReserveProtected => "reserve_protected",
            Self::MaterializeReserveHandoff => "materialize_reserve_handoff",
            Self::ClosurePressureHandoff => "closure_pressure_handoff",
            Self::SoftCapHandoff => "soft_cap_handoff",
            Self::CertificationSweep => "certification_sweep",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DemoProofCloseOverrunReason {
    SoftCapHandoff,
}

impl DemoProofCloseOverrunReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SoftCapHandoff => "soft_cap_handoff",
        }
    }
}

fn zero_rational() -> Rational {
    Rational::zero()
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DemoPhaseStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_eval_soft_cap: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub generated_floor: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exact_screened_floor: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub breadth_harvest_exit_reason: Option<DemoBreadthHarvestExitReason>,
    #[serde(default)]
    pub materialize_full_evals: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_close_entry_reason: Option<DemoProofCloseEntryReason>,
    #[serde(default)]
    pub proof_close_reserved_millis: u64,
    #[serde(default)]
    pub proof_close_elapsed_millis: u64,
    #[serde(default)]
    pub proof_close_remaining_millis: u64,
    #[serde(default)]
    pub proof_close_reserve_overrun_millis: u64,
    #[serde(default)]
    pub proof_close_reserve_exhausted: bool,
    #[serde(default)]
    pub proof_close_frontier_total_groups: usize,
    #[serde(default)]
    pub proof_close_frontier_groups_closed: usize,
    #[serde(default)]
    pub proof_close_frontier_groups_remaining: usize,
    #[serde(default)]
    pub proof_close_closure_percent: u16,
    #[serde(default)]
    pub proof_close_full_evals: usize,
    #[serde(default)]
    pub proof_close_overrun_full_evals: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proof_close_overrun_reason: Option<DemoProofCloseOverrunReason>,
    #[serde(default)]
    pub materialize_soft_cap_triggered: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DemoFunnelStats {
    #[serde(default)]
    pub generated_raw_prefixes: usize,
    #[serde(default)]
    pub canonical_prefix_signatures: usize,
    #[serde(default)]
    pub well_formed_terminals: usize,
    #[serde(default)]
    pub hard_admissible: usize,
    #[serde(default)]
    pub exact_bound_screened: usize,
    #[serde(default)]
    pub exact_bound_pruned: usize,
    #[serde(default)]
    pub heuristic_dropped: usize,
    #[serde(default)]
    pub full_telescopes_evaluated: usize,
    #[serde(default)]
    pub bar_clearers: usize,
    #[serde(default)]
    pub semantically_minimal_clearers: usize,
    #[serde(default = "zero_rational")]
    pub winner_overshoot: Rational,
}

impl Default for DemoFunnelStats {
    fn default() -> Self {
        Self {
            generated_raw_prefixes: 0,
            canonical_prefix_signatures: 0,
            well_formed_terminals: 0,
            hard_admissible: 0,
            exact_bound_screened: 0,
            exact_bound_pruned: 0,
            heuristic_dropped: 0,
            full_telescopes_evaluated: 0,
            bar_clearers: 0,
            semantically_minimal_clearers: 0,
            winner_overshoot: Rational::zero(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DemoClosureStats {
    #[serde(default)]
    pub frontier_total_seen: usize,
    #[serde(default)]
    pub frontier_certified_nonwinning: usize,
    #[serde(default)]
    pub closure_percent: u16,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchBucketTaxonomy {
    SemanticFamily,
    StructuralGeneric,
}

impl SearchBucketTaxonomy {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SemanticFamily => "semantic_family",
            Self::StructuralGeneric => "structural_generic",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SearchBucketCategory {
    TemporalShell,
    Temporal,
    Modal,
    Hilbert,
    Curvature,
    Differential,
    Metric,
    Dependent,
    PathSpace,
    LibraryRefs,
    Generic,
    LocalPlain,
    ReferenceSupport,
    BridgeReanchor,
    SupportForm,
    ModalOperator,
    TemporalOperator,
    ModalTemporalMix,
    BinderHeavy,
}

impl SearchBucketCategory {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemporalShell => "temporal_shell",
            Self::Temporal => "temporal",
            Self::Modal => "modal",
            Self::Hilbert => "hilbert",
            Self::Curvature => "curvature",
            Self::Differential => "differential",
            Self::Metric => "metric",
            Self::Dependent => "dependent",
            Self::PathSpace => "path_space",
            Self::LibraryRefs => "library_refs",
            Self::Generic => "generic",
            Self::LocalPlain => "local_plain",
            Self::ReferenceSupport => "reference_support",
            Self::BridgeReanchor => "bridge_reanchor",
            Self::SupportForm => "support_form",
            Self::ModalOperator => "modal_operator",
            Self::TemporalOperator => "temporal_operator",
            Self::ModalTemporalMix => "modal_temporal_mix",
            Self::BinderHeavy => "binder_heavy",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DemoBucketSupportProfile {
    LocalOnly,
    LibraryBacked,
}

impl DemoBucketSupportProfile {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::LibraryBacked => "library_backed",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DemoBucketWidth {
    Single,
    Pair,
    SmallCluster,
    Broad,
}

impl DemoBucketWidth {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Single => "single",
            Self::Pair => "pair",
            Self::SmallCluster => "small_cluster",
            Self::Broad => "broad",
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DemoBucketKey {
    pub clause_kappa: u16,
    pub taxonomy: SearchBucketTaxonomy,
    pub category: SearchBucketCategory,
    pub support_profile: DemoBucketSupportProfile,
    pub width: DemoBucketWidth,
}

impl DemoBucketKey {
    pub fn label(&self) -> String {
        match self.taxonomy {
            SearchBucketTaxonomy::SemanticFamily => format!(
                "k{}:{}:{}:{}",
                self.clause_kappa,
                self.category.as_str(),
                self.support_profile.as_str(),
                self.width.as_str()
            ),
            SearchBucketTaxonomy::StructuralGeneric => format!(
                "k{}:{}:{}:{}:{}",
                self.clause_kappa,
                self.taxonomy.as_str(),
                self.category.as_str(),
                self.support_profile.as_str(),
                self.width.as_str()
            ),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DemoBucketStats {
    #[serde(default)]
    pub generated_terminal_candidates: usize,
    #[serde(default)]
    pub admissible_terminal_candidates: usize,
    #[serde(default)]
    pub exact_screened_terminal_candidates: usize,
    #[serde(default)]
    pub pruned_terminal_candidates: usize,
    #[serde(default)]
    pub fully_scored_terminal_candidates: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub best_overshoot: Option<Rational>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DemoBucketReport {
    pub bucket_label: String,
    pub bucket_key: DemoBucketKey,
    #[serde(flatten)]
    pub stats: DemoBucketStats,
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ExactScreenReasonStats {
    #[serde(default)]
    pub partial_prefix_bar_failure: usize,
    #[serde(default)]
    pub terminal_prefix_completion_failure: usize,
    #[serde(default)]
    pub incumbent_dominance: usize,
    #[serde(default)]
    pub legality_connectivity_exact_rejection: usize,
}

impl ExactScreenReasonStats {
    pub fn from_incremental_counters(
        incremental_connectivity_prunes: usize,
        incremental_terminal_clause_filter_prunes: usize,
        incremental_terminal_rank_prunes: usize,
        incremental_partial_prefix_bound_prunes: usize,
        incremental_terminal_prefix_bar_prunes: usize,
    ) -> Self {
        Self {
            partial_prefix_bar_failure: incremental_partial_prefix_bound_prunes,
            terminal_prefix_completion_failure: incremental_terminal_prefix_bar_prunes,
            incumbent_dominance: incremental_terminal_rank_prunes,
            legality_connectivity_exact_rejection: incremental_connectivity_prunes
                .saturating_add(incremental_terminal_clause_filter_prunes),
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.partial_prefix_bar_failure == 0
            && self.terminal_prefix_completion_failure == 0
            && self.incumbent_dominance == 0
            && self.legality_connectivity_exact_rejection == 0
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtomicSearchStep {
    pub step_index: u32,
    pub objective_bar: Rational,
    pub admissibility: StrictAdmissibility,
    pub admissibility_diagnostics: AdmissibilityDiagnostics,
    pub prefixes_created: usize,
    pub prefix_states_explored: usize,
    pub prefix_states_merged_by_signature: usize,
    pub prefix_states_exact_pruned: usize,
    pub prefix_states_heuristic_dropped: usize,
    pub incremental_legality_cache_hits: usize,
    pub incremental_connectivity_shortcuts: usize,
    pub incremental_connectivity_fallbacks: usize,
    pub incremental_connectivity_prunes: usize,
    pub incremental_clause_family_filter_hits: usize,
    pub incremental_clause_family_prunes: usize,
    pub incremental_active_window_clause_filter_hits: usize,
    pub incremental_active_window_clause_filter_prunes: usize,
    pub incremental_terminal_clause_filter_hits: usize,
    pub incremental_terminal_clause_filter_prunes: usize,
    pub incremental_trivial_derivability_hits: usize,
    pub incremental_trivial_derivability_prunes: usize,
    pub incremental_terminal_admissibility_hits: usize,
    pub incremental_terminal_admissibility_rejections: usize,
    pub incremental_terminal_prefix_completion_hits: usize,
    pub incremental_terminal_prefix_rank_hits: usize,
    pub incremental_terminal_rank_prunes: usize,
    pub incremental_partial_prefix_bound_hits: usize,
    pub incremental_partial_prefix_bound_checks: usize,
    pub incremental_partial_prefix_bound_prunes: usize,
    pub incremental_terminal_prefix_bar_prunes: usize,
    pub exact_screen_reasons: ExactScreenReasonStats,
    pub demo_phase: DemoPhaseStats,
    pub demo_funnel: DemoFunnelStats,
    pub demo_closure: DemoClosureStats,
    pub demo_bucket_stats: Vec<DemoBucketReport>,
    pub search_timing: SearchTiming,
    pub prefix_frontier_hot_states: usize,
    pub prefix_frontier_cold_states: usize,
    pub retention_policy: RetentionPolicy,
    pub frontier_pressure: FrontierPressure,
    pub frontier_retention: FrontierRetentionOutcome,
    pub frontier_window: FrontierWindow,
    pub frontier_dedupe_keys: BTreeSet<String>,
    pub narrative_events: Vec<NarrativeEvent>,
    pub telescope: Telescope,
    pub accepted: ExpandedCandidate,
    pub retained_candidates: Vec<ExpandedCandidate>,
    pub near_misses: Vec<NearMiss>,
    pub enumerated_candidates: usize,
    pub well_formed_candidates: usize,
    pub malformed_rejections: usize,
    pub malformed_rejection_reasons: BTreeMap<String, usize>,
    pub admissibility_rejections: usize,
    pub full_telescopes_evaluated: usize,
    pub evaluated_candidates: usize,
    pub canonical_candidates: usize,
    pub semantically_minimal_candidates: usize,
    pub scored_candidate_distribution: CandidateScoreDistribution,
    pub canonical_dedupe_prunes: usize,
    pub semantic_minimality_prunes: usize,
    pub dedupe_prunes: usize,
    pub minimality_prunes: usize,
    pub heuristic_drops: usize,
    pub dedupe_pruned_candidates: Vec<DedupePruneEvidence>,
    pub minimality_pruned_candidates: Vec<MinimalityPruneEvidence>,
}

pub trait AtomicSearchProgressObserver {
    fn on_step_started(&mut self, step_index: u32);
    fn on_step_completed(&mut self, step: &AtomicSearchStep);
    fn on_step_live_checkpoint(&mut self, _checkpoint: &StepLiveCheckpoint) {}
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveStepCheckpointPhase {
    Discovery,
    Materialize,
    ProofClose,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StepLiveCheckpoint {
    pub step_index: u32,
    pub phase: LiveStepCheckpointPhase,
    pub elapsed_millis: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clause_kappa: Option<u16>,
    #[serde(default)]
    pub raw_catalog_clause_widths: Vec<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_catalog_telescope_count: Option<usize>,
    #[serde(default)]
    pub generated_raw_surface: usize,
    #[serde(default)]
    pub enumerated_candidates: usize,
    #[serde(default)]
    pub well_formed_candidates: usize,
    #[serde(default)]
    pub admissibility_rejections: usize,
    #[serde(default)]
    pub prefixes_created: usize,
    #[serde(default)]
    pub prefix_states_explored: usize,
    #[serde(default)]
    pub frontier_queue_len: usize,
    #[serde(default)]
    pub candidate_pool_len: usize,
    #[serde(default)]
    pub prefix_cache_groups: usize,
    #[serde(default)]
    pub prefix_cache_candidates: usize,
    #[serde(default)]
    pub legality_cache_entries: LiveLegalityCacheEntries,
    #[serde(default)]
    pub exact_screen_prunes: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_surface: Option<EnumerationSurfaceDiagnostics>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_one_telemetry: Option<RemainingOneTelemetry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct LiveLegalityCacheEntries {
    pub summaries: usize,
    pub family_filters: usize,
    pub family_surfaces: usize,
    pub terminal_prefix_completions: usize,
    pub partial_prefix_bounds: usize,
}

impl From<PrefixLegalityCacheEntryCounts> for LiveLegalityCacheEntries {
    fn from(value: PrefixLegalityCacheEntryCounts) -> Self {
        Self {
            summaries: value.summaries,
            family_filters: value.family_filters,
            family_surfaces: value.family_surfaces,
            terminal_prefix_completions: value.terminal_prefix_completions,
            partial_prefix_bounds: value.partial_prefix_bounds,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(default)]
pub struct RemainingOneTelemetry {
    pub remaining_one_prefixes_seen: usize,
    pub remaining_one_cached_bound_hits: usize,
    pub remaining_one_cached_rank_prunes: usize,
    pub remaining_one_algebraic_prunes: usize,
    pub remaining_one_materialized: usize,
    pub remaining_one_materialized_from_cached_summary: usize,
    pub remaining_one_materialized_compact_direct: usize,
    pub remaining_one_bar_prunes_pre_materialize: usize,
    pub remaining_one_rank_prunes_pre_materialize: usize,
    pub remaining_one_rank_prunes_post_materialize: usize,
    pub remaining_one_unknown_bound_budget_exhaustions: usize,
    pub prepare_exact_two_step_terminal_surface_millis: u64,
    pub exact_partial_prefix_bound_millis: u64,
    pub terminal_prefix_clause_filter_millis: u64,
    pub terminal_prefix_clause_filter_micros: u64,
    pub terminal_summary_build_millis: u64,
    pub terminal_summary_build_micros: u64,
    pub terminal_summary_connectivity_checks: usize,
    pub terminal_summary_fallback_connectivity_checks: usize,
    pub terminal_summary_admissibility_checks: usize,
    pub terminal_summary_exact_nu_evaluations: usize,
    pub terminal_summary_plateau_activations: usize,
    pub terminal_summary_first_plateau_activation_prefix_state: usize,
    pub terminal_summary_connectivity_millis: u64,
    pub terminal_summary_connectivity_micros: u64,
    pub terminal_summary_fallback_connectivity_millis: u64,
    pub terminal_summary_fallback_connectivity_micros: u64,
    pub terminal_summary_admissibility_millis: u64,
    pub terminal_summary_admissibility_micros: u64,
    pub terminal_summary_exact_nu_millis: u64,
    pub terminal_summary_exact_nu_micros: u64,
    pub terminal_summary_aggregation_millis: u64,
    pub terminal_summary_aggregation_micros: u64,
    pub terminal_materialize_millis: u64,
    pub candidate_sort_millis: u64,
    pub candidate_eval_minimality_millis: u64,
    pub frontier_sort_pop_millis: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct RemainingOneSummaryKernelActivationContext {
    prefix_states_explored: usize,
    prefix_cache_groups: usize,
    prefix_cache_candidates: usize,
}

const CLAIM_REMAINING_ONE_RETAINED_PLATEAU_GROUPS: usize = 39;
const CLAIM_REMAINING_ONE_RETAINED_PLATEAU_CANDIDATES: usize = 144_845;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct RemainingOneSummaryKernelTiming {
    terminal_summary_connectivity_checks: usize,
    terminal_summary_fallback_connectivity_checks: usize,
    terminal_summary_admissibility_checks: usize,
    terminal_summary_exact_nu_evaluations: usize,
    terminal_summary_connectivity_duration: Duration,
    terminal_summary_fallback_connectivity_duration: Duration,
    terminal_summary_admissibility_duration: Duration,
    terminal_summary_exact_nu_duration: Duration,
    terminal_summary_aggregation_duration: Duration,
}

impl RemainingOneTelemetry {
    fn absorb_terminal_prefix_clause_filter_duration(&mut self, duration: Duration) {
        absorb_elapsed_duration(
            &mut self.terminal_prefix_clause_filter_millis,
            &mut self.terminal_prefix_clause_filter_micros,
            duration,
        );
    }

    fn absorb_terminal_summary_build_duration(&mut self, duration: Duration) {
        absorb_elapsed_duration(
            &mut self.terminal_summary_build_millis,
            &mut self.terminal_summary_build_micros,
            duration,
        );
    }

    fn absorb_terminal_summary_kernel_timing(&mut self, timing: RemainingOneSummaryKernelTiming) {
        self.terminal_summary_connectivity_checks += timing.terminal_summary_connectivity_checks;
        self.terminal_summary_fallback_connectivity_checks +=
            timing.terminal_summary_fallback_connectivity_checks;
        self.terminal_summary_admissibility_checks += timing.terminal_summary_admissibility_checks;
        self.terminal_summary_exact_nu_evaluations += timing.terminal_summary_exact_nu_evaluations;
        absorb_elapsed_duration(
            &mut self.terminal_summary_connectivity_millis,
            &mut self.terminal_summary_connectivity_micros,
            timing.terminal_summary_connectivity_duration,
        );
        absorb_elapsed_duration(
            &mut self.terminal_summary_fallback_connectivity_millis,
            &mut self.terminal_summary_fallback_connectivity_micros,
            timing.terminal_summary_fallback_connectivity_duration,
        );
        absorb_elapsed_duration(
            &mut self.terminal_summary_admissibility_millis,
            &mut self.terminal_summary_admissibility_micros,
            timing.terminal_summary_admissibility_duration,
        );
        absorb_elapsed_duration(
            &mut self.terminal_summary_exact_nu_millis,
            &mut self.terminal_summary_exact_nu_micros,
            timing.terminal_summary_exact_nu_duration,
        );
        absorb_elapsed_duration(
            &mut self.terminal_summary_aggregation_millis,
            &mut self.terminal_summary_aggregation_micros,
            timing.terminal_summary_aggregation_duration,
        );
    }

    fn note_terminal_summary_plateau_activation(
        &mut self,
        activation: Option<RemainingOneSummaryKernelActivationContext>,
    ) {
        let Some(activation) = activation else {
            return;
        };
        if activation.prefix_cache_groups != CLAIM_REMAINING_ONE_RETAINED_PLATEAU_GROUPS
            || activation.prefix_cache_candidates != CLAIM_REMAINING_ONE_RETAINED_PLATEAU_CANDIDATES
        {
            return;
        }
        self.terminal_summary_plateau_activations += 1;
        if self.terminal_summary_first_plateau_activation_prefix_state == 0 {
            self.terminal_summary_first_plateau_activation_prefix_state =
                activation.prefix_states_explored;
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct PrefixFrontierPlan {
    frontier: FrontierWindow,
    dedupe_keys: BTreeSet<String>,
    retained_prefix_signatures: Vec<PrefixSignature>,
    explored: usize,
    exact_pruned: usize,
    heuristic_dropped: usize,
    pressure: FrontierPressure,
}

#[derive(Clone, Debug)]
struct PrefixFrontierItem {
    signature: PrefixSignature,
    dedupe_key: String,
    retention_class: RetentionClass,
    record: FrontierStateRecV1,
}

#[derive(Clone, Debug, Default)]
struct RealisticShadowDiscovery {
    prefix_cache: PrefixCache,
    prefix_legality_cache: PrefixLegalityCache,
    demo_bucket_stats: BTreeMap<DemoBucketKey, DemoBucketStats>,
    candidates: Vec<ExpandedCandidate>,
    raw_generated_surface: usize,
    stop_reason: Option<DemoBreadthHarvestExitReason>,
    terminal_rank_incumbent: Option<AcceptRank>,
    terminal_rank_prunes: usize,
    prefixes_created: usize,
    prefix_states_explored: usize,
    enumerated_candidates: usize,
    well_formed_candidates: usize,
    malformed_rejections: usize,
    malformed_rejection_reasons: BTreeMap<String, usize>,
    admissibility_rejections: usize,
    admissibility_diagnostics: AdmissibilityDiagnostics,
    connectivity_prunes: usize,
    partial_prefix_bound_checks: usize,
    partial_prefix_bound_prunes: usize,
    terminal_prefix_bar_prunes: usize,
    remaining_one_telemetry: RemainingOneTelemetry,
}

impl RealisticShadowDiscovery {
    fn can_stop_for_budget(&self) -> bool {
        !self.candidates.is_empty() || !self.prefix_cache.is_empty()
    }

    fn demo_bucket_stats_mut(&mut self, bucket_key: DemoBucketKey) -> &mut DemoBucketStats {
        self.demo_bucket_stats.entry(bucket_key).or_default()
    }

    fn record_demo_bucket_surface(
        &mut self,
        bucket_key: DemoBucketKey,
        generated_terminal_candidates: usize,
        admissible_terminal_candidates: usize,
    ) {
        let stats = self.demo_bucket_stats_mut(bucket_key);
        stats.generated_terminal_candidates += generated_terminal_candidates;
        stats.admissible_terminal_candidates += admissible_terminal_candidates;
    }

    fn record_demo_bucket_exact_screened(
        &mut self,
        bucket_key: DemoBucketKey,
        exact_screened_terminal_candidates: usize,
    ) {
        self.demo_bucket_stats_mut(bucket_key)
            .exact_screened_terminal_candidates += exact_screened_terminal_candidates;
    }

    fn record_demo_bucket_pruned(
        &mut self,
        bucket_key: DemoBucketKey,
        pruned_terminal_candidates: usize,
    ) {
        self.demo_bucket_stats_mut(bucket_key)
            .pruned_terminal_candidates += pruned_terminal_candidates;
    }
}

fn remaining_one_summary_kernel_activation_context(
    discovery: &RealisticShadowDiscovery,
) -> RemainingOneSummaryKernelActivationContext {
    RemainingOneSummaryKernelActivationContext {
        prefix_states_explored: discovery.prefix_states_explored,
        prefix_cache_groups: discovery.prefix_cache.len(),
        prefix_cache_candidates: discovery.prefix_cache.candidate_count(),
    }
}

fn claim_live_checkpoint_enabled(mode: AdmissibilityMode, step_index: u32) -> bool {
    matches!(mode, AdmissibilityMode::DesktopClaimShadow) && (step_index == 4 || step_index == 5)
}

fn discovery_enumeration_context(
    library: &Library,
    admissibility: StrictAdmissibility,
    demo_budget_enabled: bool,
) -> EnumerationContext {
    let mut enumeration_context = EnumerationContext::from_admissibility(library, admissibility);
    if demo_budget_enabled && matches!(admissibility.mode, AdmissibilityMode::DemoBreadthShadow) {
        enumeration_context.late_family_surface = LateFamilySurface::DemoBreadthShadow;
    }
    enumeration_context
}

fn maybe_emit_claim_live_checkpoint(
    progress_observer: &mut Option<&mut dyn AtomicSearchProgressObserver>,
    last_checkpoint_elapsed_millis: &mut u64,
    checkpoint: StepLiveCheckpoint,
    force: bool,
) {
    if !force
        && checkpoint
            .elapsed_millis
            .saturating_sub(*last_checkpoint_elapsed_millis)
            < 1_000
    {
        return;
    }
    if let Some(observer) = progress_observer.as_deref_mut() {
        observer.on_step_live_checkpoint(&checkpoint);
        *last_checkpoint_elapsed_millis = checkpoint.elapsed_millis;
    }
}

fn demo_bucket_width(surface_count: usize) -> DemoBucketWidth {
    match surface_count {
        0 | 1 => DemoBucketWidth::Single,
        2 => DemoBucketWidth::Pair,
        3 | 4 => DemoBucketWidth::SmallCluster,
        _ => DemoBucketWidth::Broad,
    }
}

fn demo_bucket_taxonomy(admissibility_mode: AdmissibilityMode) -> SearchBucketTaxonomy {
    match admissibility_mode {
        AdmissibilityMode::DesktopClaimShadow => SearchBucketTaxonomy::StructuralGeneric,
        _ => SearchBucketTaxonomy::SemanticFamily,
    }
}

fn semantic_bucket_category(signature: &PrefixSignature) -> SearchBucketCategory {
    if signature.has_temporal_shell_family() {
        SearchBucketCategory::TemporalShell
    } else if signature.has_temporal_family() {
        SearchBucketCategory::Temporal
    } else if signature.has_modal_family() {
        SearchBucketCategory::Modal
    } else if signature.has_hilbert_family() {
        SearchBucketCategory::Hilbert
    } else if signature.has_curvature_family() {
        SearchBucketCategory::Curvature
    } else if signature.has_differential_family() {
        SearchBucketCategory::Differential
    } else if signature.has_metric_family() {
        SearchBucketCategory::Metric
    } else if signature.has_dependent_family() {
        SearchBucketCategory::Dependent
    } else if signature.has_path_space_family() {
        SearchBucketCategory::PathSpace
    } else if signature.has_library_refs() {
        SearchBucketCategory::LibraryRefs
    } else {
        SearchBucketCategory::Generic
    }
}

fn claim_bucket_category(
    signature: &PrefixSignature,
    prefix_telescope: &Telescope,
) -> SearchBucketCategory {
    let signals = structural_signals_for_telescope(prefix_telescope);
    let has_modal_ops = prefix_telescope
        .clauses
        .iter()
        .any(|clause| clause.expr.is_modal());
    let has_temporal_ops = prefix_telescope
        .clauses
        .iter()
        .any(|clause| clause.expr.is_temporal());

    if has_modal_ops && has_temporal_ops {
        SearchBucketCategory::ModalTemporalMix
    } else if has_temporal_ops {
        SearchBucketCategory::TemporalOperator
    } else if signals.generic_binder_count >= 4 || signals.dependent_motive_density > 0 {
        SearchBucketCategory::BinderHeavy
    } else if prefix_telescope.has_loop()
        || (signature.has_library_refs()
            && signals.eliminator_score > 0
            && signals.closure_score > 0)
    {
        SearchBucketCategory::BridgeReanchor
    } else if has_modal_ops {
        SearchBucketCategory::ModalOperator
    } else if signals.former_score > 0 || signals.closure_score > 1 {
        SearchBucketCategory::SupportForm
    } else if signature.has_library_refs() {
        SearchBucketCategory::ReferenceSupport
    } else {
        SearchBucketCategory::LocalPlain
    }
}

fn demo_bucket_key(
    admissibility_mode: AdmissibilityMode,
    signature: &PrefixSignature,
    prefix_telescope: &Telescope,
    clause_kappa: u16,
    generated_terminal_candidates: usize,
    admissible_terminal_candidates: usize,
) -> DemoBucketKey {
    let surface_count = generated_terminal_candidates
        .max(admissible_terminal_candidates)
        .max(1);
    let taxonomy = demo_bucket_taxonomy(admissibility_mode);
    DemoBucketKey {
        clause_kappa,
        taxonomy,
        category: match taxonomy {
            SearchBucketTaxonomy::SemanticFamily => semantic_bucket_category(signature),
            SearchBucketTaxonomy::StructuralGeneric => {
                claim_bucket_category(signature, prefix_telescope)
            }
        },
        support_profile: if signature.has_library_refs() {
            DemoBucketSupportProfile::LibraryBacked
        } else {
            DemoBucketSupportProfile::LocalOnly
        },
        width: demo_bucket_width(surface_count),
    }
}

fn demo_bucket_key_for_group(
    admissibility_mode: AdmissibilityMode,
    signature: &PrefixSignature,
    group: &PrefixCandidateGroup,
) -> DemoBucketKey {
    demo_bucket_key(
        admissibility_mode,
        signature,
        &group.prefix_telescope,
        group.bound.clause_kappa_used,
        group.candidates.len(),
        group.candidates.len(),
    )
}

fn demo_bucket_reports(
    bucket_stats: &BTreeMap<DemoBucketKey, DemoBucketStats>,
) -> Vec<DemoBucketReport> {
    bucket_stats
        .iter()
        .map(|(bucket_key, stats)| DemoBucketReport {
            bucket_label: bucket_key.label(),
            bucket_key: bucket_key.clone(),
            stats: stats.clone(),
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OnlinePrefixWorkItem {
    clause_kappa: u16,
    prefix_telescope: Telescope,
    signature: PrefixSignature,
    remaining_clause_slots: usize,
    remaining_family_options: u8,
    bit_cost: u32,
    clause_count: usize,
    filtered_next_clauses: Option<Vec<pen_core::clause::ClauseRec>>,
    filtered_next_clause_connectivity_facts: Option<Vec<TerminalClauseConnectivityFacts>>,
    filtered_next_clause_nu_facts: Option<Vec<TerminalClauseNuFacts>>,
    next_clause_count: usize,
    order_key: Arc<str>,
}

impl OnlinePrefixWorkItem {
    fn next_clauses<'a>(
        &'a self,
        clause_catalog: &'a ClauseCatalog,
    ) -> &'a [pen_core::clause::ClauseRec] {
        self.filtered_next_clauses
            .as_deref()
            .unwrap_or_else(|| clause_catalog.clauses_at(self.prefix_telescope.clauses.len()))
    }

    fn next_clause_connectivity_facts<'a>(
        &'a self,
        clause_catalog: &'a ClauseCatalog,
    ) -> Option<&'a [TerminalClauseConnectivityFacts]> {
        self.filtered_next_clause_connectivity_facts
            .as_deref()
            .or_else(|| {
                self.filtered_next_clauses.is_none().then(|| {
                    clause_catalog
                        .terminal_connectivity_facts_at(self.prefix_telescope.clauses.len())
                })
            })
    }

    fn next_clause_nu_facts<'a>(
        &'a self,
        clause_catalog: &'a ClauseCatalog,
    ) -> &'a [TerminalClauseNuFacts] {
        self.filtered_next_clause_nu_facts
            .as_deref()
            .unwrap_or_else(|| {
                assert!(
                    self.filtered_next_clauses.is_none(),
                    "filtered terminal clauses should carry aligned nu facts"
                );
                clause_catalog.terminal_nu_facts_at(self.prefix_telescope.clauses.len())
            })
    }
}

#[derive(Clone, Debug, Default)]
struct MaterializedTerminalPrefixGroup {
    candidates: Vec<PrefixGroupCandidate>,
    bound: Option<PrefixBound>,
    best_accept_rank: Option<AcceptRank>,
    generated_terminal_candidates: usize,
    admissible_terminal_candidates: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DemoProofCloseOrderMode {
    PotentialFirst,
    ClosureFirst,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct DemoClosurePressure {
    pending_exact_surface: usize,
    prune_ready_exact_surface: usize,
    pending_groups: usize,
    prune_ready_groups: usize,
}

impl DemoClosurePressure {
    fn prefers_closure(self) -> bool {
        self.pending_groups >= 2
            && self.prune_ready_groups > 0
            && self.prune_ready_groups.saturating_mul(3) >= self.pending_groups.saturating_mul(2)
            && self.prune_ready_exact_surface.saturating_mul(2) >= self.pending_exact_surface
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ExactPartialPrefixBoundDecision {
    CanClearBar,
    CannotClearBar,
    Unknown,
}

impl ExactPartialPrefixBoundDecision {
    fn cacheable_partial_decision(self) -> Option<PartialPrefixBoundDecision> {
        match self {
            Self::CanClearBar => Some(PartialPrefixBoundDecision::CanClearBar),
            Self::CannotClearBar => Some(PartialPrefixBoundDecision::CannotClearBar),
            Self::Unknown => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DemoBudgetSeed {
    pub consumed_total_millis: u64,
    pub consumed_early_millis: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DemoBudgetFraction {
    numerator: u64,
    denominator: u64,
}

impl DemoBudgetFraction {
    fn parse(value: &str) -> Result<Self> {
        let value = value.trim();
        if value.is_empty() {
            bail!("demo budget fraction must not be empty");
        }
        if value.starts_with('-') {
            bail!("demo budget fraction must be non-negative");
        }

        let mut parts = value.split('.');
        let whole = parts
            .next()
            .expect("split always yields at least one part")
            .parse::<u64>()?;
        let fractional = parts.next().unwrap_or("");
        if parts.next().is_some() {
            bail!("demo budget fraction '{value}' contained more than one decimal point");
        }

        let denominator = 10_u64
            .checked_pow(u32::try_from(fractional.len()).expect("fractional length exceeded u32"))
            .expect("fraction denominator exceeded u64");
        let numerator = whole
            .checked_mul(denominator)
            .and_then(|scaled| {
                let fractional = if fractional.is_empty() {
                    0
                } else {
                    fractional.parse::<u64>().ok()?
                };
                scaled.checked_add(fractional)
            })
            .ok_or_else(|| anyhow::anyhow!("demo budget fraction '{value}' exceeded u64"))?;
        if numerator > denominator {
            bail!("demo budget fraction '{value}' must be between 0 and 1");
        }

        Ok(Self {
            numerator,
            denominator,
        })
    }

    fn apply(self, amount: u64) -> u64 {
        let scaled = u128::from(amount) * u128::from(self.numerator);
        let divided = scaled / u128::from(self.denominator);
        u64::try_from(divided).expect("fraction application exceeded u64")
    }

    fn to_basis_points(self) -> u64 {
        let scaled = u128::from(self.numerator) * 10_000;
        let divided = scaled / u128::from(self.denominator);
        u64::try_from(divided).expect("basis points exceeded u64")
    }

    fn from_basis_points(basis_points: u64) -> Self {
        Self {
            numerator: basis_points,
            denominator: 10_000,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct DemoStepBudget {
    step_index: u32,
    total_budget_millis: u64,
    discovery_budget_millis: u64,
    scout_budget_millis: u64,
    breadth_harvest_budget_millis: u64,
    proof_close_reserve_millis: u64,
    baseline_budget_millis: u64,
    spill_budget_millis: u64,
    shared_early_window_remaining_millis: u64,
    generated_floor: Option<u64>,
    exact_screened_floor: Option<u64>,
    full_eval_soft_cap: Option<u64>,
    pulse_interval_millis: u32,
    live_rebalance_borrowed_millis: u64,
    max_live_rebalance_borrow_millis: u64,
    next_live_retune_check_millis: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct DemoBudgetFeedback {
    late_floor_miss: bool,
    reserve_pressure: bool,
    reserve_slack: bool,
}

impl DemoBudgetFeedback {
    fn from_step(step: &AtomicSearchStep) -> Option<Self> {
        if step.step_index < DEMO_LATE_FLOOR_START_STEP {
            return None;
        }

        let generated_surface =
            u64::try_from(step.demo_funnel.generated_raw_prefixes).expect("usize exceeded u64");
        let exact_screened_surface =
            u64::try_from(step.demo_funnel.exact_bound_screened).expect("usize exceeded u64");
        let generated_miss = step
            .demo_phase
            .generated_floor
            .map(|target| generated_surface < target)
            .unwrap_or(false);
        let exact_screened_miss = step
            .demo_phase
            .exact_screened_floor
            .map(|target| exact_screened_surface < target)
            .unwrap_or(false);
        let reserve_pressure = step.demo_phase.proof_close_reserved_millis > 0
            && (step.demo_phase.proof_close_reserve_exhausted
                || step.demo_phase.proof_close_reserve_overrun_millis > 0
                || (step.demo_phase.proof_close_frontier_total_groups > 0
                    && step.demo_phase.proof_close_frontier_groups_remaining > 0));
        let reserve_slack = step.demo_phase.proof_close_reserved_millis > 0
            && !step.demo_phase.proof_close_reserve_exhausted
            && step.demo_phase.proof_close_closure_percent == 100
            && step
                .demo_phase
                .proof_close_remaining_millis
                .saturating_mul(2)
                >= step.demo_phase.proof_close_reserved_millis;

        Some(Self {
            late_floor_miss: generated_miss || exact_screened_miss,
            reserve_pressure,
            reserve_slack,
        })
    }
}

impl DemoStepBudget {
    fn discovery_deadline(self, step_start: Instant) -> Option<Instant> {
        step_start.checked_add(Duration::from_millis(self.discovery_budget_millis))
    }

    fn breadth_harvest_floor_hit(
        self,
        elapsed_millis: u64,
        progress: &NarrativeProgressSnapshot,
    ) -> Option<DemoBreadthHarvestExitReason> {
        if self.proof_close_reserve_millis == 0 || elapsed_millis < self.scout_budget_millis {
            return None;
        }
        if let Some(target) = self.exact_screened_floor {
            if progress.exact_screened_surface >= target {
                return Some(DemoBreadthHarvestExitReason::ExactScreenedFloorHit);
            }
        }
        if let Some(target) = self.generated_floor {
            if progress.generated_surface >= target {
                return Some(DemoBreadthHarvestExitReason::GeneratedFloorHit);
            }
        }
        None
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct DemoBudgetRetune {
    action: DemoBudgetRetuneAction,
    adjustment_millis: u64,
    projected_generated_surface: u64,
    projected_exact_screened_surface: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DemoBudgetRetuneAction {
    BorrowFromProofClose,
    ReturnToProofClose,
}

impl DemoBudgetRetuneAction {
    const fn verb(self) -> &'static str {
        match self {
            Self::BorrowFromProofClose => "borrowed",
            Self::ReturnToProofClose => "returned",
        }
    }

    const fn counterparty(self) -> &'static str {
        match self {
            Self::BorrowFromProofClose => "from proof_close reserve",
            Self::ReturnToProofClose => "to proof_close reserve",
        }
    }
}

fn projected_surface(current: u64, rate_per_sec: u64, remaining_millis: u64) -> u64 {
    current.saturating_add(
        u64::try_from(
            (u128::from(rate_per_sec) * u128::from(remaining_millis)) / u128::from(1_000_u64),
        )
        .expect("projected surface exceeded u64"),
    )
}

fn ceil_div_u128(numerator: u128, denominator: u128) -> u128 {
    debug_assert!(denominator > 0);
    numerator.div_ceil(denominator)
}

fn additional_floor_budget_millis(
    current: u64,
    rate_per_sec: u64,
    target: Option<u64>,
    remaining_millis: u64,
) -> u64 {
    required_floor_budget_millis(current, rate_per_sec, target).saturating_sub(remaining_millis)
}

fn required_floor_budget_millis(current: u64, rate_per_sec: u64, target: Option<u64>) -> u64 {
    let Some(target) = target else {
        return 0;
    };
    if current >= target {
        return 0;
    }
    if rate_per_sec == 0 {
        return u64::MAX;
    }

    let deficit = u128::from(target - current);
    let millis_needed = ceil_div_u128(
        deficit.saturating_mul(u128::from(1_000_u64)),
        u128::from(rate_per_sec),
    );
    u64::try_from(millis_needed).expect("floor budget exceeded u64")
}

fn maybe_retune_demo_budget_live(
    budget: &mut DemoStepBudget,
    progress: &NarrativeProgressSnapshot,
) -> Option<DemoBudgetRetune> {
    if progress.elapsed_millis < budget.scout_budget_millis
        || progress.elapsed_millis < budget.next_live_retune_check_millis
    {
        return None;
    }

    let remaining_discovery_millis = budget
        .discovery_budget_millis
        .saturating_sub(progress.elapsed_millis);
    if remaining_discovery_millis == 0 {
        budget.next_live_retune_check_millis = progress
            .elapsed_millis
            .saturating_add(DEMO_LIVE_RETUNE_COOLDOWN_MILLIS);
        return None;
    }

    let generated_rate = per_second(progress.generated_surface, progress.elapsed_millis);
    let exact_screened_rate = per_second(progress.exact_screened_surface, progress.elapsed_millis);
    let generated_extra_millis = additional_floor_budget_millis(
        progress.generated_surface,
        generated_rate,
        budget.generated_floor,
        remaining_discovery_millis,
    );
    let exact_screened_extra_millis = additional_floor_budget_millis(
        progress.exact_screened_surface,
        exact_screened_rate,
        budget.exact_screened_floor,
        remaining_discovery_millis,
    );
    let requested_borrow_millis = generated_extra_millis.max(exact_screened_extra_millis);
    if requested_borrow_millis > 0
        && budget.max_live_rebalance_borrow_millis > budget.live_rebalance_borrowed_millis
        && budget.proof_close_reserve_millis > 0
    {
        let max_borrow_millis = budget
            .max_live_rebalance_borrow_millis
            .saturating_sub(budget.live_rebalance_borrowed_millis)
            .min(budget.proof_close_reserve_millis);
        let adjustment_millis = requested_borrow_millis.min(max_borrow_millis);
        if adjustment_millis >= DEMO_LIVE_RETUNE_MIN_ADJUSTMENT_MILLIS {
            budget.discovery_budget_millis = budget
                .discovery_budget_millis
                .saturating_add(adjustment_millis);
            budget.breadth_harvest_budget_millis = budget
                .breadth_harvest_budget_millis
                .saturating_add(adjustment_millis);
            budget.proof_close_reserve_millis = budget
                .proof_close_reserve_millis
                .saturating_sub(adjustment_millis);
            budget.live_rebalance_borrowed_millis = budget
                .live_rebalance_borrowed_millis
                .saturating_add(adjustment_millis);
            budget.next_live_retune_check_millis = progress
                .elapsed_millis
                .saturating_add(DEMO_LIVE_RETUNE_COOLDOWN_MILLIS);

            let remaining_discovery_after_retune = budget
                .discovery_budget_millis
                .saturating_sub(progress.elapsed_millis);
            return Some(DemoBudgetRetune {
                action: DemoBudgetRetuneAction::BorrowFromProofClose,
                adjustment_millis,
                projected_generated_surface: projected_surface(
                    progress.generated_surface,
                    generated_rate,
                    remaining_discovery_after_retune,
                ),
                projected_exact_screened_surface: projected_surface(
                    progress.exact_screened_surface,
                    exact_screened_rate,
                    remaining_discovery_after_retune,
                ),
            });
        }
    }

    let generated_required_millis = required_floor_budget_millis(
        progress.generated_surface,
        generated_rate,
        budget.generated_floor,
    );
    let exact_screened_required_millis = required_floor_budget_millis(
        progress.exact_screened_surface,
        exact_screened_rate,
        budget.exact_screened_floor,
    );
    let required_remaining_millis = generated_required_millis.max(exact_screened_required_millis);
    if budget.live_rebalance_borrowed_millis > 0 && required_remaining_millis != u64::MAX {
        let slack_millis = remaining_discovery_millis.saturating_sub(required_remaining_millis);
        let adjustment_millis = budget.live_rebalance_borrowed_millis.min(slack_millis / 2);
        if adjustment_millis >= DEMO_LIVE_RETUNE_MIN_ADJUSTMENT_MILLIS {
            budget.discovery_budget_millis = budget
                .discovery_budget_millis
                .saturating_sub(adjustment_millis);
            budget.breadth_harvest_budget_millis = budget
                .breadth_harvest_budget_millis
                .saturating_sub(adjustment_millis);
            budget.proof_close_reserve_millis = budget
                .proof_close_reserve_millis
                .saturating_add(adjustment_millis);
            budget.live_rebalance_borrowed_millis = budget
                .live_rebalance_borrowed_millis
                .saturating_sub(adjustment_millis);
            budget.next_live_retune_check_millis = progress
                .elapsed_millis
                .saturating_add(DEMO_LIVE_RETUNE_COOLDOWN_MILLIS);

            let remaining_discovery_after_retune = budget
                .discovery_budget_millis
                .saturating_sub(progress.elapsed_millis);
            return Some(DemoBudgetRetune {
                action: DemoBudgetRetuneAction::ReturnToProofClose,
                adjustment_millis,
                projected_generated_surface: projected_surface(
                    progress.generated_surface,
                    generated_rate,
                    remaining_discovery_after_retune,
                ),
                projected_exact_screened_surface: projected_surface(
                    progress.exact_screened_surface,
                    exact_screened_rate,
                    remaining_discovery_after_retune,
                ),
            });
        }
    }

    budget.next_live_retune_check_millis = progress
        .elapsed_millis
        .saturating_add(DEMO_LIVE_RETUNE_COOLDOWN_MILLIS);
    None
}

#[derive(Clone, Debug)]
struct DemoBudgetController {
    demo: DemoConfig,
    until_step: u32,
    proof_close_reserve_fraction: DemoBudgetFraction,
    scout_fraction: DemoBudgetFraction,
    consumed_total_millis: u64,
    consumed_early_millis: u64,
    late_floor_pressure: u8,
    late_reserve_pressure: u8,
}

impl DemoBudgetController {
    fn maybe_new(
        config: &RuntimeConfig,
        until_step: u32,
        seed: DemoBudgetSeed,
    ) -> Result<Option<Self>> {
        if config.mode.search_profile != SearchProfile::DemoBreadthShadow || !config.demo.enabled {
            return Ok(None);
        }

        Ok(Some(Self {
            demo: config.demo.clone(),
            until_step: until_step.min(LIVE_BOOTSTRAP_MAX_STEP),
            proof_close_reserve_fraction: DemoBudgetFraction::parse(
                &config.demo.proof_close_reserve_fraction,
            )?,
            scout_fraction: DemoBudgetFraction::parse(&config.demo.scout_fraction)?,
            consumed_total_millis: seed.consumed_total_millis,
            consumed_early_millis: seed.consumed_early_millis,
            late_floor_pressure: 0,
            late_reserve_pressure: 0,
        }))
    }

    fn plan_step(&self, step_index: u32) -> DemoStepBudget {
        let remaining_total_millis = self.remaining_total_millis();
        let generated_floor = demo_generated_floor(&self.demo, step_index);
        let exact_screened_floor = demo_exact_screened_floor(&self.demo, step_index);
        let full_eval_soft_cap = demo_full_eval_soft_cap(&self.demo, step_index);
        if step_index <= 4 {
            let shared_early_window_remaining_millis = self.remaining_early_millis();
            let total_budget_millis =
                remaining_total_millis.min(shared_early_window_remaining_millis);
            let discovery_budget_millis = total_budget_millis;
            let scout_budget_millis = self
                .scout_fraction
                .apply(discovery_budget_millis)
                .min(discovery_budget_millis);
            return DemoStepBudget {
                step_index,
                total_budget_millis,
                discovery_budget_millis,
                scout_budget_millis,
                breadth_harvest_budget_millis: discovery_budget_millis
                    .saturating_sub(scout_budget_millis),
                proof_close_reserve_millis: 0,
                baseline_budget_millis: total_budget_millis,
                spill_budget_millis: 0,
                shared_early_window_remaining_millis,
                generated_floor,
                exact_screened_floor,
                full_eval_soft_cap,
                pulse_interval_millis: self.demo.narrative.pulse_interval_millis,
                live_rebalance_borrowed_millis: 0,
                max_live_rebalance_borrow_millis: 0,
                next_live_retune_check_millis: scout_budget_millis,
            };
        }

        let baseline_budget_millis = demo_step_floor_millis(&self.demo, step_index);
        let spill_budget_millis = self.spill_budget_millis(step_index, remaining_total_millis);
        let total_budget_millis = baseline_budget_millis
            .saturating_add(spill_budget_millis)
            .min(remaining_total_millis);
        let proof_close_reserve_millis = self
            .effective_proof_close_reserve_fraction()
            .apply(total_budget_millis);
        let discovery_budget_millis =
            total_budget_millis.saturating_sub(proof_close_reserve_millis);
        let scout_budget_millis = self
            .scout_fraction
            .apply(discovery_budget_millis)
            .min(discovery_budget_millis);

        DemoStepBudget {
            step_index,
            total_budget_millis,
            discovery_budget_millis,
            scout_budget_millis,
            breadth_harvest_budget_millis: discovery_budget_millis
                .saturating_sub(scout_budget_millis),
            proof_close_reserve_millis,
            baseline_budget_millis,
            spill_budget_millis,
            shared_early_window_remaining_millis: 0,
            generated_floor,
            exact_screened_floor,
            full_eval_soft_cap,
            pulse_interval_millis: self.demo.narrative.pulse_interval_millis,
            live_rebalance_borrowed_millis: 0,
            max_live_rebalance_borrow_millis: proof_close_reserve_millis / 2,
            next_live_retune_check_millis: scout_budget_millis,
        }
    }

    fn record_step(&mut self, step_index: u32, elapsed_millis: u64) {
        self.consumed_total_millis = self.consumed_total_millis.saturating_add(elapsed_millis);
        if step_index <= 4 {
            self.consumed_early_millis = self.consumed_early_millis.saturating_add(elapsed_millis);
        }
    }

    fn record_step_outcome(&mut self, step: &AtomicSearchStep) {
        self.record_step(step.step_index, step.search_timing.step_wall_clock_millis);
        if let Some(feedback) = DemoBudgetFeedback::from_step(step) {
            self.record_feedback(feedback);
        }
    }

    fn remaining_total_millis(&self) -> u64 {
        u64::from(self.demo.total_budget_sec)
            .saturating_mul(1_000)
            .saturating_sub(self.consumed_total_millis)
    }

    fn remaining_early_millis(&self) -> u64 {
        u64::from(self.demo.early_exhaustive_budget_sec)
            .saturating_mul(1_000)
            .saturating_sub(self.consumed_early_millis)
    }

    fn remaining_late_baseline_budget_millis_from(&self, step_index: u32) -> u64 {
        let start = step_index.max(5);
        if start > self.until_step {
            return 0;
        }

        (start..=self.until_step)
            .map(|late_step| demo_step_floor_millis(&self.demo, late_step))
            .sum()
    }

    fn effective_proof_close_reserve_fraction(&self) -> DemoBudgetFraction {
        let base_basis_points = self.proof_close_reserve_fraction.to_basis_points();
        if !(2_500..=4_000).contains(&base_basis_points) {
            return self.proof_close_reserve_fraction;
        }
        let min_basis_points = base_basis_points.min(2_500);
        let max_basis_points = base_basis_points.max(4_000);
        let adjusted_basis_points = i64::try_from(base_basis_points)
            .expect("basis points exceeded i64")
            .saturating_add(i64::from(self.late_reserve_pressure) * 250)
            .saturating_sub(i64::from(self.late_floor_pressure) * 250);
        DemoBudgetFraction::from_basis_points(
            u64::try_from(adjusted_basis_points.clamp(
                i64::try_from(min_basis_points).expect("min basis points exceeded i64"),
                i64::try_from(max_basis_points).expect("max basis points exceeded i64"),
            ))
            .expect("basis points became negative"),
        )
    }

    fn spill_budget_millis(&self, step_index: u32, remaining_total_millis: u64) -> u64 {
        if step_index < DEMO_LATE_SPILL_START_STEP {
            return 0;
        }

        let discretionary_budget_millis = remaining_total_millis
            .saturating_sub(self.remaining_late_baseline_budget_millis_from(step_index));
        if discretionary_budget_millis == 0 {
            return 0;
        }

        let total_weight = (step_index..=self.until_step)
            .map(|candidate_step| self.spill_weight(step_index, candidate_step))
            .sum::<u64>();
        if total_weight == 0 {
            return 0;
        }

        let current_weight = self.spill_weight(step_index, step_index);
        u64::try_from(
            (u128::from(discretionary_budget_millis) * u128::from(current_weight))
                / u128::from(total_weight),
        )
        .expect("spill budget exceeded u64")
    }

    fn spill_weight(&self, planning_step_index: u32, candidate_step_index: u32) -> u64 {
        let generated_weight =
            demo_generated_floor(&self.demo, candidate_step_index).unwrap_or(0) / 750;
        let exact_weight =
            demo_exact_screened_floor(&self.demo, candidate_step_index).unwrap_or(0) / 300;
        let late_bias =
            u64::from(candidate_step_index.saturating_sub(DEMO_LATE_SPILL_START_STEP)) + 1;
        let base_weight = 1 + generated_weight + exact_weight + late_bias;

        if candidate_step_index == planning_step_index {
            base_weight.saturating_mul(1 + u64::from(self.late_floor_pressure))
        } else {
            base_weight
        }
    }

    fn record_feedback(&mut self, feedback: DemoBudgetFeedback) {
        if feedback.late_floor_miss {
            self.late_floor_pressure = self.late_floor_pressure.saturating_add(1).min(3);
        } else {
            self.late_floor_pressure = self.late_floor_pressure.saturating_sub(1);
        }

        if feedback.reserve_pressure {
            self.late_reserve_pressure = self.late_reserve_pressure.saturating_add(1).min(4);
        } else {
            self.late_reserve_pressure = self.late_reserve_pressure.saturating_sub(1);
        }

        if feedback.late_floor_miss && feedback.reserve_slack {
            self.late_reserve_pressure = self.late_reserve_pressure.saturating_sub(1);
        }
    }
}

fn demo_step_floor_millis(demo: &DemoConfig, step_index: u32) -> u64 {
    demo.floors
        .step_floor_sec
        .get(&step_index.to_string())
        .copied()
        .map(u64::from)
        .unwrap_or(0)
        .saturating_mul(1_000)
}

fn demo_generated_floor(demo: &DemoConfig, step_index: u32) -> Option<u64> {
    if step_index == 1 {
        return Some(2144);
    }
    demo.floors
        .generated_floor
        .get(&step_index.to_string())
        .copied()
}

fn demo_exact_screened_floor(demo: &DemoConfig, step_index: u32) -> Option<u64> {
    demo.floors
        .exact_screened_floor
        .get(&step_index.to_string())
        .copied()
}

fn demo_full_eval_soft_cap(demo: &DemoConfig, step_index: u32) -> Option<u64> {
    demo.caps
        .full_eval_soft_cap
        .get(&step_index.to_string())
        .copied()
}

#[derive(Clone, Debug)]
struct DemoNarrativeRuntime {
    budget: DemoStepBudget,
    recorder: NarrativeRecorder,
    pulse_interval_millis: u64,
    last_budget_pulse_elapsed_millis: [Option<u64>; 5],
    scout_sample_recorded: bool,
    breadth_harvest_entered: bool,
    proof_close_last_reported_closure_percent: u16,
    proof_close_reserve_exhausted_reported: bool,
}

impl DemoNarrativeRuntime {
    fn new(
        step_index: u32,
        objective_bar: Rational,
        admissibility: StrictAdmissibility,
        budget: DemoStepBudget,
    ) -> Self {
        let mut recorder = NarrativeRecorder::new(step_index);
        let zero_progress = narrative_progress_snapshot(0, 0, 0, 0);
        recorder.push(
            NarrativeEventKind::StepOpen,
            None,
            format!(
                "opened demo step {} with bar {} and clause band {}..{}",
                step_index,
                objective_bar,
                admissibility.min_clause_kappa,
                admissibility.max_clause_kappa
            ),
            Some(format!(
                "mode={} {}",
                admissibility_mode_name(admissibility.mode),
                demo_budget_plan_detail(budget)
            )),
            Some(zero_progress.clone()),
        );
        recorder.push(
            NarrativeEventKind::PhaseChange,
            Some(StepPhase::Scout),
            format!(
                "entered scout with {} reserved for throughput sampling",
                format_duration_millis(budget.scout_budget_millis)
            ),
            Some(demo_budget_plan_detail(budget)),
            Some(zero_progress.clone()),
        );
        let mut runtime = Self {
            budget,
            recorder,
            pulse_interval_millis: u64::from(budget.pulse_interval_millis),
            last_budget_pulse_elapsed_millis: [None; 5],
            scout_sample_recorded: false,
            breadth_harvest_entered: false,
            proof_close_last_reported_closure_percent: 0,
            proof_close_reserve_exhausted_reported: false,
        };
        runtime.push_budget_pulse(
            StepPhase::Scout,
            "scout baselining started".to_owned(),
            Some(demo_budget_plan_detail(budget)),
            zero_progress,
            true,
        );
        runtime
    }

    fn maybe_enter_breadth_harvest(
        &mut self,
        elapsed_millis: u64,
        progress: NarrativeProgressSnapshot,
        scout_detail: String,
        harvest_detail: String,
    ) {
        if self.breadth_harvest_entered || elapsed_millis < self.budget.scout_budget_millis {
            return;
        }

        if !self.scout_sample_recorded {
            self.record_scout_sample(progress.clone(), scout_detail);
        }
        if self.budget.breadth_harvest_budget_millis == 0 {
            return;
        }

        self.recorder.push(
            NarrativeEventKind::PhaseChange,
            Some(StepPhase::BreadthHarvest),
            format!(
                "entered breadth_harvest with {} of widening budget remaining",
                format_duration_millis(self.budget.breadth_harvest_budget_millis)
            ),
            Some(harvest_detail.clone()),
            Some(progress.clone()),
        );
        self.push_budget_pulse(
            StepPhase::BreadthHarvest,
            "breadth harvest is widening under the remaining discovery slice".to_owned(),
            Some(harvest_detail),
            progress,
            true,
        );
        self.breadth_harvest_entered = true;
    }

    fn record_scout_budget_rebalance(
        &mut self,
        progress: NarrativeProgressSnapshot,
        scout_detail: String,
        budget: DemoStepBudget,
        rebalance: DemoBudgetRetune,
    ) {
        self.record_scout_sample(progress.clone(), scout_detail);
        self.budget = budget;
        let message = match rebalance.action {
            DemoBudgetRetuneAction::BorrowFromProofClose => format!(
                "scout borrowed {} from proof_close reserve after the live floor projection stayed underwater",
                format_duration_millis(rebalance.adjustment_millis)
            ),
            DemoBudgetRetuneAction::ReturnToProofClose => format!(
                "scout returned {} to proof_close reserve after the live floor projection recovered",
                format_duration_millis(rebalance.adjustment_millis)
            ),
        };
        self.push_budget_pulse(
            StepPhase::Scout,
            message,
            Some(format!(
                "{} outstanding_borrow={} max_live_borrow={} projected_generated={} projected_exact_screened={}",
                demo_budget_plan_detail(budget),
                format_duration_millis(budget.live_rebalance_borrowed_millis),
                format_duration_millis(budget.max_live_rebalance_borrow_millis),
                floor_status(
                    rebalance.projected_generated_surface,
                    budget.generated_floor
                ),
                floor_status(
                    rebalance.projected_exact_screened_surface,
                    budget.exact_screened_floor
                )
            )),
            progress,
            false,
        );
    }

    fn record_breadth_harvest_budget_retune(
        &mut self,
        progress: NarrativeProgressSnapshot,
        budget: DemoStepBudget,
        rebalance: DemoBudgetRetune,
    ) {
        self.budget = budget;
        let message = match rebalance.action {
            DemoBudgetRetuneAction::BorrowFromProofClose => format!(
                "breadth_harvest {} {} {} after live floor projections stayed underwater",
                rebalance.action.verb(),
                format_duration_millis(rebalance.adjustment_millis),
                rebalance.action.counterparty()
            ),
            DemoBudgetRetuneAction::ReturnToProofClose => format!(
                "breadth_harvest {} {} {} after live floor projections recovered",
                rebalance.action.verb(),
                format_duration_millis(rebalance.adjustment_millis),
                rebalance.action.counterparty()
            ),
        };
        self.push_budget_pulse(
            StepPhase::BreadthHarvest,
            message,
            Some(format!(
                "{} outstanding_borrow={} max_live_borrow={} projected_generated={} projected_exact_screened={}",
                demo_budget_plan_detail(budget),
                format_duration_millis(budget.live_rebalance_borrowed_millis),
                format_duration_millis(budget.max_live_rebalance_borrow_millis),
                floor_status(
                    rebalance.projected_generated_surface,
                    budget.generated_floor
                ),
                floor_status(
                    rebalance.projected_exact_screened_surface,
                    budget.exact_screened_floor
                )
            )),
            progress,
            false,
        );
    }

    fn close_discovery(
        &mut self,
        progress: NarrativeProgressSnapshot,
        scout_detail: String,
        harvest_detail: String,
        exit_reason: Option<DemoBreadthHarvestExitReason>,
    ) {
        self.record_scout_sample(progress.clone(), scout_detail);
        if self.breadth_harvest_entered {
            let message = match exit_reason {
                Some(DemoBreadthHarvestExitReason::GeneratedFloorHit) => {
                    "breadth harvest hit the generated floor and closed the widening slice"
                }
                Some(DemoBreadthHarvestExitReason::ExactScreenedFloorHit) => {
                    "breadth harvest hit the exact-screened floor and closed the widening slice"
                }
                Some(DemoBreadthHarvestExitReason::ProofCloseReserveProtected) => {
                    "breadth harvest yielded to protect the proof-close reserve"
                }
                None => "breadth harvest closed the widening slice",
            };
            let detail = match exit_reason {
                Some(reason) => format!("{harvest_detail} exit_reason={}", reason.as_str()),
                None => harvest_detail,
            };
            self.push_budget_pulse(
                StepPhase::BreadthHarvest,
                message.to_owned(),
                Some(detail),
                progress,
                true,
            );
        }
    }

    fn enter_materialize(&mut self, progress: NarrativeProgressSnapshot, detail: String) {
        self.recorder.push(
            NarrativeEventKind::PhaseChange,
            Some(StepPhase::Materialize),
            "entered materialize to reopen the retained exact surface".to_owned(),
            Some(detail.clone()),
            Some(progress.clone()),
        );
        self.push_budget_pulse(
            StepPhase::Materialize,
            "materializing retained prefixes under exact screening".to_owned(),
            Some(detail),
            progress,
            true,
        );
    }

    fn enter_proof_close_with_message(
        &mut self,
        progress: NarrativeProgressSnapshot,
        message: impl Into<String>,
        detail: String,
    ) {
        self.recorder.push(
            NarrativeEventKind::PhaseChange,
            Some(StepPhase::ProofClose),
            message.into(),
            Some(detail.clone()),
            Some(progress.clone()),
        );
        self.push_budget_pulse(
            StepPhase::ProofClose,
            "proof-close is certifying the incumbent under exact comparison".to_owned(),
            Some(detail),
            progress,
            true,
        );
    }

    fn maybe_record_proof_close_progress(
        &mut self,
        progress: NarrativeProgressSnapshot,
        detail: String,
        phase: &DemoPhaseStats,
    ) {
        if phase.proof_close_frontier_total_groups > 0 {
            let milestone = if phase.proof_close_closure_percent >= 100
                && self.proof_close_last_reported_closure_percent < 100
            {
                Some(100)
            } else if phase.proof_close_closure_percent >= 75
                && self.proof_close_last_reported_closure_percent < 75
            {
                Some(75)
            } else if phase.proof_close_closure_percent >= 50
                && self.proof_close_last_reported_closure_percent < 50
            {
                Some(50)
            } else if phase.proof_close_closure_percent >= 25
                && self.proof_close_last_reported_closure_percent < 25
            {
                Some(25)
            } else {
                None
            };
            if let Some(milestone) = milestone {
                self.push_budget_pulse(
                    StepPhase::ProofClose,
                    format!("proof_close certified {milestone}% of the retained frontier groups"),
                    Some(detail.clone()),
                    progress.clone(),
                    true,
                );
                self.proof_close_last_reported_closure_percent = milestone;
            }
        }
        if phase.proof_close_reserve_exhausted && !self.proof_close_reserve_exhausted_reported {
            self.push_budget_pulse(
                StepPhase::ProofClose,
                "proof_close exhausted the reserved certification slice and is overrunning to finish exact closure"
                    .to_owned(),
                Some(detail),
                progress,
                true,
            );
            self.proof_close_reserve_exhausted_reported = true;
        }
    }

    fn enter_seal(
        &mut self,
        progress: NarrativeProgressSnapshot,
        detail: String,
        accepted_hash: &str,
        overshoot: Rational,
    ) {
        self.recorder.push(
            NarrativeEventKind::PhaseChange,
            Some(StepPhase::Seal),
            format!(
                "entered seal with incumbent {} ready for deterministic acceptance",
                accepted_hash
            ),
            Some(detail.clone()),
            Some(progress.clone()),
        );
        self.push_budget_pulse(
            StepPhase::Seal,
            format!(
                "seal fixed overshoot {} for candidate {}",
                overshoot, accepted_hash
            ),
            Some(detail),
            progress,
            true,
        );
    }

    fn finish(self) -> Vec<NarrativeEvent> {
        self.recorder.finish()
    }

    fn record_scout_sample(&mut self, progress: NarrativeProgressSnapshot, detail: String) {
        if self.scout_sample_recorded {
            return;
        }
        self.push_budget_pulse(
            StepPhase::Scout,
            "scout sample captured throughput on this machine".to_owned(),
            Some(detail),
            progress,
            true,
        );
        self.scout_sample_recorded = true;
    }

    fn push_budget_pulse(
        &mut self,
        phase: StepPhase,
        message: String,
        detail: Option<String>,
        progress: NarrativeProgressSnapshot,
        force: bool,
    ) {
        let slot = phase.pulse_slot();
        let elapsed_millis = progress.elapsed_millis;
        if !force {
            if let Some(last_elapsed) = self.last_budget_pulse_elapsed_millis[slot] {
                if elapsed_millis.saturating_sub(last_elapsed) < self.pulse_interval_millis {
                    return;
                }
            }
        }
        self.recorder.push(
            NarrativeEventKind::BudgetPulse,
            Some(phase),
            message,
            detail,
            Some(progress),
        );
        if !force {
            self.last_budget_pulse_elapsed_millis[slot] = Some(elapsed_millis);
        }
    }
}

fn admissibility_mode_name(mode: AdmissibilityMode) -> &'static str {
    match mode {
        AdmissibilityMode::Guarded => "guarded",
        AdmissibilityMode::RelaxedShadow => "relaxed_shadow",
        AdmissibilityMode::RealisticShadow => "realistic_shadow",
        AdmissibilityMode::DemoBreadthShadow => "demo_breadth_shadow",
        AdmissibilityMode::DesktopClaimShadow => "desktop_claim_shadow",
    }
}

fn demo_budget_plan_detail(budget: DemoStepBudget) -> String {
    let mut parts = vec![
        format!(
            "total={}",
            format_duration_millis(budget.total_budget_millis)
        ),
        format!(
            "discovery={}",
            format_duration_millis(budget.discovery_budget_millis)
        ),
        format!(
            "scout={}",
            format_duration_millis(budget.scout_budget_millis)
        ),
        format!(
            "breadth={}",
            format_duration_millis(budget.breadth_harvest_budget_millis)
        ),
        format!(
            "proof_close_reserve={}",
            format_duration_millis(budget.proof_close_reserve_millis)
        ),
    ];
    if budget.baseline_budget_millis > 0 {
        parts.push(format!(
            "baseline={}",
            format_duration_millis(budget.baseline_budget_millis)
        ));
    }
    if budget.spill_budget_millis > 0 {
        parts.push(format!(
            "spill={}",
            format_duration_millis(budget.spill_budget_millis)
        ));
    }
    if budget.shared_early_window_remaining_millis > 0 {
        parts.push(format!(
            "shared_early_remaining={}",
            format_duration_millis(budget.shared_early_window_remaining_millis)
        ));
    }
    if let Some(generated_floor) = budget.generated_floor {
        parts.push(format!("generated_floor={generated_floor}"));
    }
    if let Some(exact_screened_floor) = budget.exact_screened_floor {
        parts.push(format!("exact_screened_floor={exact_screened_floor}"));
    }
    if let Some(full_eval_soft_cap) = budget.full_eval_soft_cap {
        parts.push(format!("full_eval_soft_cap={full_eval_soft_cap}"));
    }
    parts.join(" ")
}

fn format_duration_millis(millis: u64) -> String {
    if millis < 1_000 {
        return format!("{millis}ms");
    }
    if millis < 60_000 {
        let whole = millis / 1_000;
        let tenths = (millis % 1_000) / 100;
        return format!("{whole}.{tenths}s");
    }

    let total_seconds = millis / 1_000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{minutes}m{seconds:02}s")
}

fn per_second(count: u64, elapsed_millis: u64) -> u64 {
    if elapsed_millis == 0 {
        return count;
    }
    count.saturating_mul(1_000) / elapsed_millis
}

fn floor_status(current: u64, target: Option<u64>) -> String {
    match target {
        Some(target) => {
            let status = if current >= target { "hit" } else { "miss" };
            format!("{current}/{target}({status})")
        }
        None => current.to_string(),
    }
}

fn cap_status(current: u64, limit: Option<u64>) -> String {
    match limit {
        Some(limit) => {
            let status = if current <= limit {
                "within_limit"
            } else {
                "over_limit"
            };
            format!("{current}/{limit}({status})")
        }
        None => current.to_string(),
    }
}

fn demo_proof_close_closure_percent(closed_groups: usize, total_groups: usize) -> u16 {
    if total_groups == 0 {
        return 100;
    }

    u16::try_from(((closed_groups.min(total_groups) as u128) * 100) / (total_groups as u128))
        .expect("proof-close closure percent exceeded u16")
}

fn sync_demo_proof_close_accounting(
    phase: &mut DemoPhaseStats,
    budget: DemoStepBudget,
    proof_close_started_elapsed_millis: u64,
    current_elapsed_millis: u64,
    total_groups: usize,
    closed_groups: usize,
) {
    let elapsed_millis = current_elapsed_millis.saturating_sub(proof_close_started_elapsed_millis);
    let closed_groups = closed_groups.min(total_groups);
    let remaining_groups = total_groups.saturating_sub(closed_groups);

    phase.proof_close_reserved_millis = budget.proof_close_reserve_millis;
    phase.proof_close_elapsed_millis = elapsed_millis;
    phase.proof_close_remaining_millis = budget
        .proof_close_reserve_millis
        .saturating_sub(elapsed_millis);
    phase.proof_close_reserve_overrun_millis =
        elapsed_millis.saturating_sub(budget.proof_close_reserve_millis);
    phase.proof_close_reserve_exhausted = phase.proof_close_reserve_exhausted
        || (remaining_groups > 0 && elapsed_millis >= budget.proof_close_reserve_millis);
    phase.proof_close_frontier_total_groups = total_groups;
    phase.proof_close_frontier_groups_closed = closed_groups;
    phase.proof_close_frontier_groups_remaining = remaining_groups;
    phase.proof_close_closure_percent =
        demo_proof_close_closure_percent(closed_groups, total_groups);
}

fn sync_demo_proof_close_runtime(
    phase: &mut DemoPhaseStats,
    demo_narrative: &mut Option<DemoNarrativeRuntime>,
    budget: DemoStepBudget,
    proof_close_started_elapsed_millis: u64,
    total_groups: usize,
    closed_groups: usize,
    step_start: Instant,
    generated_raw_surface: u64,
    exact_screen_pruned: usize,
    full_telescopes_evaluated: usize,
) {
    let current_elapsed = elapsed_millis(step_start.elapsed());
    sync_demo_proof_close_accounting(
        phase,
        budget,
        proof_close_started_elapsed_millis,
        current_elapsed,
        total_groups,
        closed_groups,
    );
    if let Some(observer) = demo_narrative.as_mut() {
        let progress = narrative_progress_snapshot(
            current_elapsed,
            generated_raw_surface,
            exact_screened_surface_from_counts(exact_screen_pruned, full_telescopes_evaluated),
            full_telescopes_evaluated as u64,
        );
        let detail = demo_proof_close_status_detail(
            budget,
            generated_raw_surface,
            exact_screened_surface_from_counts(exact_screen_pruned, full_telescopes_evaluated),
            full_telescopes_evaluated as u64,
            phase.proof_close_frontier_groups_remaining,
            phase,
        );
        observer.maybe_record_proof_close_progress(progress, detail, phase);
    }
}

fn demo_surface_status_detail(
    budget: DemoStepBudget,
    generated_surface: u64,
    exact_screened_surface: u64,
    full_telescopes_evaluated: u64,
) -> String {
    format!(
        "generated={} exact_screened={} full_evals={}",
        floor_status(generated_surface, budget.generated_floor),
        floor_status(exact_screened_surface, budget.exact_screened_floor),
        cap_status(full_telescopes_evaluated, budget.full_eval_soft_cap)
    )
}

fn demo_phase_status_detail(phase: &DemoPhaseStats) -> String {
    let soft_cap = phase
        .full_eval_soft_cap
        .map(|limit| limit.to_string())
        .unwrap_or_else(|| "none".to_owned());
    let breadth_exit = phase
        .breadth_harvest_exit_reason
        .map(|reason| reason.as_str())
        .unwrap_or("none");
    let proof_close_reason = phase
        .proof_close_entry_reason
        .map(|reason| reason.as_str())
        .unwrap_or("none");
    let proof_close_overrun_reason = phase
        .proof_close_overrun_reason
        .map(|reason| reason.as_str())
        .unwrap_or("none");
    format!(
        "materialize_full_evals={} proof_close_full_evals={} proof_close_overrun={} soft_cap={} cap_triggered={} breadth_exit={} proof_close_reason={} proof_close_overrun_reason={} proof_close_reserve={} proof_close_elapsed={} proof_close_remaining={} proof_close_reserve_overrun={} proof_close_reserve_exhausted={} proof_close_groups_closed={}/{} proof_close_groups_remaining={} proof_close_closure={}%",
        phase.materialize_full_evals,
        phase.proof_close_full_evals,
        phase.proof_close_overrun_full_evals,
        soft_cap,
        phase.materialize_soft_cap_triggered,
        breadth_exit,
        proof_close_reason,
        proof_close_overrun_reason,
        format_duration_millis(phase.proof_close_reserved_millis),
        format_duration_millis(phase.proof_close_elapsed_millis),
        format_duration_millis(phase.proof_close_remaining_millis),
        format_duration_millis(phase.proof_close_reserve_overrun_millis),
        phase.proof_close_reserve_exhausted,
        phase.proof_close_frontier_groups_closed,
        phase.proof_close_frontier_total_groups,
        phase.proof_close_frontier_groups_remaining,
        phase.proof_close_closure_percent
    )
}

fn demo_proof_close_entry_reason(
    breadth_exit_reason: Option<DemoBreadthHarvestExitReason>,
) -> DemoProofCloseEntryReason {
    match breadth_exit_reason {
        Some(
            DemoBreadthHarvestExitReason::GeneratedFloorHit
            | DemoBreadthHarvestExitReason::ExactScreenedFloorHit,
        ) => DemoProofCloseEntryReason::BreadthFloorHit,
        Some(DemoBreadthHarvestExitReason::ProofCloseReserveProtected) => {
            DemoProofCloseEntryReason::ReserveProtected
        }
        None => DemoProofCloseEntryReason::CertificationSweep,
    }
}

fn demo_proof_close_entry_message(reason: DemoProofCloseEntryReason) -> &'static str {
    match reason {
        DemoProofCloseEntryReason::BreadthFloorHit => {
            "entered proof_close after breadth_harvest hit the late-step breadth floor"
        }
        DemoProofCloseEntryReason::ReserveProtected => {
            "entered proof_close after breadth_harvest yielded to protect the reserved certification slice"
        }
        DemoProofCloseEntryReason::MaterializeReserveHandoff => {
            "entered proof_close after materialize yielded to protect the reserved certification slice"
        }
        DemoProofCloseEntryReason::ClosurePressureHandoff => {
            "entered proof_close after live closure pressure turned the remaining exact surface into certification work"
        }
        DemoProofCloseEntryReason::SoftCapHandoff => {
            "entered proof_close after materialize hit the full-eval soft cap"
        }
        DemoProofCloseEntryReason::CertificationSweep => {
            "entered proof_close to certify the remaining exact surface"
        }
    }
}

fn demo_materialize_status_detail(
    budget: DemoStepBudget,
    generated_surface: u64,
    exact_screened_surface: u64,
    full_telescopes_evaluated: u64,
    retained_prefix_groups: usize,
    phase: &DemoPhaseStats,
) -> String {
    format!(
        "retained_prefix_groups={} {} {}",
        retained_prefix_groups,
        demo_surface_status_detail(
            budget,
            generated_surface,
            exact_screened_surface,
            full_telescopes_evaluated,
        ),
        demo_phase_status_detail(phase)
    )
}

fn demo_proof_close_status_detail(
    budget: DemoStepBudget,
    generated_surface: u64,
    exact_screened_surface: u64,
    full_telescopes_evaluated: u64,
    remaining_retained_prefix_groups: usize,
    phase: &DemoPhaseStats,
) -> String {
    format!(
        "remaining_retained_prefix_groups={} {} {}",
        remaining_retained_prefix_groups,
        demo_surface_status_detail(
            budget,
            generated_surface,
            exact_screened_surface,
            full_telescopes_evaluated,
        ),
        demo_phase_status_detail(phase)
    )
}

fn begin_demo_proof_close(
    demo_phase: &mut DemoPhaseStats,
    demo_narrative: &mut Option<DemoNarrativeRuntime>,
    budget: DemoStepBudget,
    entry_reason: DemoProofCloseEntryReason,
    remaining_groups: usize,
    step_start: Instant,
    generated_raw_surface: u64,
    exact_screen_pruned: usize,
    full_telescopes_evaluated: usize,
) -> u64 {
    let proof_close_started_elapsed = elapsed_millis(step_start.elapsed());
    demo_phase.proof_close_entry_reason = Some(entry_reason);
    sync_demo_proof_close_accounting(
        demo_phase,
        budget,
        proof_close_started_elapsed,
        proof_close_started_elapsed,
        remaining_groups,
        0,
    );
    if let Some(observer) = demo_narrative.as_mut() {
        let progress = narrative_progress_snapshot(
            proof_close_started_elapsed,
            generated_raw_surface,
            exact_screened_surface_from_counts(exact_screen_pruned, full_telescopes_evaluated),
            full_telescopes_evaluated as u64,
        );
        let detail = demo_proof_close_status_detail(
            budget,
            generated_raw_surface,
            exact_screened_surface_from_counts(exact_screen_pruned, full_telescopes_evaluated),
            full_telescopes_evaluated as u64,
            remaining_groups,
            demo_phase,
        );
        let message = if remaining_groups > 0 {
            format!(
                "{} with {} retained prefix groups still needing exact certification",
                demo_proof_close_entry_message(entry_reason),
                remaining_groups
            )
        } else {
            demo_proof_close_entry_message(entry_reason).to_owned()
        };
        observer.enter_proof_close_with_message(progress.clone(), message, detail.clone());
        observer.maybe_record_proof_close_progress(progress, detail, demo_phase);
    }

    proof_close_started_elapsed
}

fn discovery_progress_snapshot(
    step_start: Instant,
    discovery: &RealisticShadowDiscovery,
) -> NarrativeProgressSnapshot {
    narrative_progress_snapshot(
        elapsed_millis(step_start.elapsed()),
        discovery.raw_generated_surface as u64,
        discovery_exact_screened_surface(discovery),
        discovery.candidates.len() as u64,
    )
}

fn discovery_exact_screened_surface(discovery: &RealisticShadowDiscovery) -> u64 {
    (discovery.partial_prefix_bound_checks
        + discovery.partial_prefix_bound_prunes
        + discovery.terminal_prefix_bar_prunes
        + discovery.candidates.len()) as u64
}

fn discovery_scout_detail(discovery: &RealisticShadowDiscovery, elapsed_millis: u64) -> String {
    format!(
        "generated_per_sec={} admissibility_per_sec={} exact_bound_per_sec={} full_eval_per_sec={} generated={} admissibility_checks={} exact_bound_checks={} full_evals={}",
        per_second(discovery.raw_generated_surface as u64, elapsed_millis),
        per_second(discovery.well_formed_candidates as u64, elapsed_millis),
        per_second(discovery.partial_prefix_bound_checks as u64, elapsed_millis),
        per_second(discovery.candidates.len() as u64, elapsed_millis),
        discovery.raw_generated_surface,
        discovery.well_formed_candidates,
        discovery.partial_prefix_bound_checks,
        discovery.candidates.len()
    )
}

fn discovery_harvest_detail(
    budget: DemoStepBudget,
    discovery: &RealisticShadowDiscovery,
) -> String {
    let generated_surface = discovery.raw_generated_surface as u64;
    let exact_screened_surface = discovery_exact_screened_surface(discovery);
    format!(
        "{} admissibility_rejections={} partial_prefix_prunes={} terminal_prefix_bar_prunes={}",
        demo_surface_status_detail(
            budget,
            generated_surface,
            exact_screened_surface,
            discovery.candidates.len() as u64
        ),
        discovery.admissibility_rejections,
        discovery.partial_prefix_bound_prunes,
        discovery.terminal_prefix_bar_prunes
    )
}

pub fn supports_live_atomic_search(until_step: u32) -> bool {
    until_step <= LIVE_BOOTSTRAP_MAX_STEP
}

pub fn search_bootstrap_prefix(
    until_step: u32,
    window_depth: u16,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_prefix_with_runtime(
        until_step,
        window_depth,
        FrontierRuntimeLimits::unlimited(),
    )
}

pub fn search_bootstrap_prefix_with_runtime(
    until_step: u32,
    window_depth: u16,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_prefix_for_profile_with_runtime(
        until_step,
        window_depth,
        SearchProfile::StrictCanonGuarded,
        retention_runtime,
    )
}

pub fn search_bootstrap_prefix_for_profile_with_runtime(
    until_step: u32,
    window_depth: u16,
    search_profile: SearchProfile,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_prefix_for_profile_with_runtime_and_observer(
        until_step,
        window_depth,
        search_profile,
        retention_runtime,
        None,
    )
}

pub fn search_bootstrap_prefix_for_profile_with_runtime_and_observer(
    until_step: u32,
    window_depth: u16,
    search_profile: SearchProfile,
    retention_runtime: FrontierRuntimeLimits,
    progress_observer: Option<&mut dyn AtomicSearchProgressObserver>,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_from_prefix_internal(
        &[],
        until_step,
        window_depth,
        search_profile,
        retention_runtime,
        None,
        progress_observer,
    )
}

pub fn search_bootstrap_prefix_for_config_with_runtime(
    until_step: u32,
    window_depth: u16,
    config: &RuntimeConfig,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_from_prefix_for_config_with_runtime_and_seed_and_observer(
        &[],
        until_step,
        window_depth,
        config,
        retention_runtime,
        DemoBudgetSeed::default(),
        None,
    )
}

pub fn search_bootstrap_from_prefix(
    accepted_prefix: &[Telescope],
    until_step: u32,
    window_depth: u16,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_from_prefix_with_runtime(
        accepted_prefix,
        until_step,
        window_depth,
        FrontierRuntimeLimits::unlimited(),
    )
}

pub fn search_bootstrap_from_prefix_with_runtime(
    accepted_prefix: &[Telescope],
    until_step: u32,
    window_depth: u16,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_from_prefix_for_profile_with_runtime(
        accepted_prefix,
        until_step,
        window_depth,
        SearchProfile::StrictCanonGuarded,
        retention_runtime,
    )
}

pub fn search_bootstrap_from_prefix_for_profile_with_runtime(
    accepted_prefix: &[Telescope],
    until_step: u32,
    window_depth: u16,
    search_profile: SearchProfile,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_from_prefix_for_profile_with_runtime_and_observer(
        accepted_prefix,
        until_step,
        window_depth,
        search_profile,
        retention_runtime,
        None,
    )
}

pub fn search_bootstrap_from_prefix_for_profile_with_runtime_and_observer(
    accepted_prefix: &[Telescope],
    until_step: u32,
    window_depth: u16,
    search_profile: SearchProfile,
    retention_runtime: FrontierRuntimeLimits,
    progress_observer: Option<&mut dyn AtomicSearchProgressObserver>,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_from_prefix_internal(
        accepted_prefix,
        until_step,
        window_depth,
        search_profile,
        retention_runtime,
        None,
        progress_observer,
    )
}

pub fn search_bootstrap_from_prefix_for_config_with_runtime_and_seed(
    accepted_prefix: &[Telescope],
    until_step: u32,
    window_depth: u16,
    config: &RuntimeConfig,
    retention_runtime: FrontierRuntimeLimits,
    demo_budget_seed: DemoBudgetSeed,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_from_prefix_for_config_with_runtime_and_seed_and_observer(
        accepted_prefix,
        until_step,
        window_depth,
        config,
        retention_runtime,
        demo_budget_seed,
        None,
    )
}

pub fn search_bootstrap_from_prefix_for_config_with_runtime_and_seed_and_observer(
    accepted_prefix: &[Telescope],
    until_step: u32,
    window_depth: u16,
    config: &RuntimeConfig,
    retention_runtime: FrontierRuntimeLimits,
    demo_budget_seed: DemoBudgetSeed,
    progress_observer: Option<&mut dyn AtomicSearchProgressObserver>,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_from_prefix_internal(
        accepted_prefix,
        until_step,
        window_depth,
        config.mode.search_profile,
        retention_runtime,
        DemoBudgetController::maybe_new(config, until_step, demo_budget_seed)?,
        progress_observer,
    )
}

fn search_bootstrap_from_prefix_internal(
    accepted_prefix: &[Telescope],
    until_step: u32,
    window_depth: u16,
    search_profile: SearchProfile,
    retention_runtime: FrontierRuntimeLimits,
    mut demo_budget_controller: Option<DemoBudgetController>,
    mut progress_observer: Option<&mut dyn AtomicSearchProgressObserver>,
) -> Result<Vec<AtomicSearchStep>> {
    let mut library: Library = Vec::new();
    let mut history: Vec<DiscoveryRecord> = Vec::new();
    let mut steps = Vec::new();
    let admissibility_mode = admissibility_mode_for_profile(search_profile);

    for (offset, telescope) in accepted_prefix.iter().enumerate() {
        let step_index = u32::try_from(offset + 1).expect("accepted prefix length exceeded u32");
        let accepted = evaluate_candidate(&library, &history, telescope.clone())?;
        history.push(DiscoveryRecord::new(
            step_index,
            u32::from(accepted.nu),
            u32::from(accepted.clause_kappa),
        ));
        library.push(LibraryEntry::from_telescope(telescope, &library));
    }

    let start_step = u32::try_from(accepted_prefix.len()).expect("prefix length exceeded u32") + 1;
    for step_index in start_step..=until_step.min(LIVE_BOOTSTRAP_MAX_STEP) {
        if let Some(observer) = progress_observer.as_deref_mut() {
            observer.on_step_started(step_index);
        }
        let demo_step_budget = demo_budget_controller
            .as_ref()
            .map(|controller| controller.plan_step(step_index));
        let outcome = search_next_step(
            step_index,
            window_depth,
            &library,
            &history,
            admissibility_mode,
            retention_runtime,
            demo_step_budget,
            &mut progress_observer,
        )?;
        if let Some(controller) = demo_budget_controller.as_mut() {
            controller.record_step_outcome(&outcome);
        }
        if let Some(observer) = progress_observer.as_deref_mut() {
            observer.on_step_completed(&outcome);
        }
        history.push(DiscoveryRecord::new(
            step_index,
            u32::from(outcome.accepted.nu),
            u32::from(outcome.accepted.clause_kappa),
        ));
        library.push(LibraryEntry::from_telescope(&outcome.telescope, &library));
        steps.push(outcome);
    }

    Ok(steps)
}

fn search_next_step(
    step_index: u32,
    window_depth: u16,
    library: &Library,
    history: &[DiscoveryRecord],
    admissibility_mode: AdmissibilityMode,
    retention_runtime: FrontierRuntimeLimits,
    demo_step_budget: Option<DemoStepBudget>,
    progress_observer: &mut Option<&mut dyn AtomicSearchProgressObserver>,
) -> Result<AtomicSearchStep> {
    let step_start = Instant::now();
    let mut last_live_checkpoint_elapsed_millis = 0u64;
    let structural_debt = summarize_structural_debt(library, window_depth);
    let admissibility =
        strict_admissibility_for_mode(step_index, window_depth, library, admissibility_mode);
    let retention_policy = structural_debt.retention_policy();
    let objective_bar = compute_bar(window_depth as usize, step_index, history).bar;
    let mut demo_step_budget = demo_step_budget;
    let mut demo_narrative = demo_step_budget
        .map(|budget| DemoNarrativeRuntime::new(step_index, objective_bar, admissibility, budget));
    let mut candidates = Vec::new();
    let mut prefix_cache = PrefixCache::default();
    let mut prefixes_created = 0usize;
    let mut prefix_states_explored = 0usize;
    let generated_raw_surface;
    let mut enumerated_candidates = 0usize;
    let mut well_formed_candidates = 0usize;
    let mut malformed_rejections = 0usize;
    let mut malformed_rejection_reasons = BTreeMap::new();
    let mut admissibility_rejections = 0usize;
    let mut admissibility_diagnostics = AdmissibilityDiagnostics::default();
    let mut incremental_legality_cache_hits = 0usize;
    let mut incremental_connectivity_shortcuts = 0usize;
    let mut incremental_connectivity_fallbacks = 0usize;
    let mut incremental_connectivity_prunes = 0usize;
    let mut incremental_clause_family_filter_hits = 0usize;
    let mut incremental_clause_family_prunes = 0usize;
    let mut incremental_active_window_clause_filter_hits = 0usize;
    let mut incremental_active_window_clause_filter_prunes = 0usize;
    let mut incremental_terminal_clause_filter_hits = 0usize;
    let mut incremental_terminal_clause_filter_prunes = 0usize;
    let mut incremental_trivial_derivability_hits = 0usize;
    let mut incremental_trivial_derivability_prunes = 0usize;
    let mut incremental_terminal_admissibility_hits = 0usize;
    let mut incremental_terminal_admissibility_rejections = 0usize;
    let mut incremental_terminal_prefix_completion_hits = 0usize;
    let mut incremental_terminal_prefix_rank_hits = 0usize;
    let mut incremental_terminal_rank_prunes = 0usize;
    let mut incremental_partial_prefix_bound_hits = 0usize;
    let mut incremental_partial_prefix_bound_checks = 0usize;
    let mut incremental_partial_prefix_bound_prunes = 0usize;
    let mut incremental_terminal_prefix_bar_prunes = 0usize;
    let mut demo_phase = DemoPhaseStats {
        full_eval_soft_cap: demo_step_budget.and_then(|budget| budget.full_eval_soft_cap),
        generated_floor: demo_step_budget.and_then(|budget| budget.generated_floor),
        exact_screened_floor: demo_step_budget.and_then(|budget| budget.exact_screened_floor),
        proof_close_reserved_millis: demo_step_budget
            .map(|budget| budget.proof_close_reserve_millis)
            .unwrap_or(0),
        proof_close_remaining_millis: demo_step_budget
            .map(|budget| budget.proof_close_reserve_millis)
            .unwrap_or(0),
        ..DemoPhaseStats::default()
    };
    let discovery_start = Instant::now();
    let nu_history = history
        .iter()
        .map(|record| (record.step_index, record.nu))
        .collect::<Vec<_>>();
    let mut demo_bucket_stats = BTreeMap::new();

    if matches!(
        admissibility_mode,
        AdmissibilityMode::RealisticShadow
            | AdmissibilityMode::DemoBreadthShadow
            | AdmissibilityMode::DesktopClaimShadow
    ) {
        let discovery = discover_realistic_shadow_candidates(
            step_index,
            library,
            history,
            admissibility,
            retention_policy,
            objective_bar,
            &nu_history,
            &mut demo_step_budget,
            step_start,
            &mut demo_narrative,
            progress_observer,
        )?;
        let discovery_stop_reason = discovery.stop_reason.or_else(|| {
            demo_step_budget.and_then(|budget| {
                (budget.discovery_budget_millis == 0 && budget.proof_close_reserve_millis > 0)
                    .then_some(DemoBreadthHarvestExitReason::ProofCloseReserveProtected)
            })
        });
        if let (Some(observer), Some(budget)) = (demo_narrative.as_mut(), demo_step_budget) {
            observer.close_discovery(
                discovery_progress_snapshot(step_start, &discovery),
                discovery_scout_detail(&discovery, elapsed_millis(step_start.elapsed())),
                discovery_harvest_detail(budget, &discovery),
                discovery_stop_reason,
            );
        }
        prefix_cache = discovery.prefix_cache;
        candidates = discovery.candidates;
        demo_bucket_stats = discovery.demo_bucket_stats;
        prefixes_created = discovery.prefixes_created;
        prefix_states_explored = discovery.prefix_states_explored;
        generated_raw_surface = discovery.raw_generated_surface;
        enumerated_candidates = discovery.enumerated_candidates;
        well_formed_candidates = discovery.well_formed_candidates;
        malformed_rejections = discovery.malformed_rejections;
        malformed_rejection_reasons = discovery.malformed_rejection_reasons;
        admissibility_rejections = discovery.admissibility_rejections;
        admissibility_diagnostics = discovery.admissibility_diagnostics;
        let legality_stats = discovery.prefix_legality_cache.stats();
        incremental_legality_cache_hits = legality_stats.legality_hits;
        incremental_connectivity_shortcuts = legality_stats.connectivity_shortcuts;
        incremental_connectivity_fallbacks = legality_stats.connectivity_fallbacks;
        incremental_connectivity_prunes =
            legality_stats.connectivity_prunes + discovery.connectivity_prunes;
        incremental_clause_family_filter_hits = legality_stats.clause_family_filter_hits;
        incremental_clause_family_prunes = legality_stats.clause_family_prunes;
        incremental_active_window_clause_filter_hits =
            legality_stats.active_window_clause_filter_hits;
        incremental_active_window_clause_filter_prunes =
            legality_stats.active_window_clause_filter_prunes;
        incremental_terminal_clause_filter_hits = legality_stats.terminal_clause_filter_hits;
        incremental_terminal_clause_filter_prunes = legality_stats.terminal_clause_filter_prunes;
        incremental_trivial_derivability_hits = legality_stats.trivial_derivability_hits;
        incremental_trivial_derivability_prunes = legality_stats.trivial_derivability_prunes;
        incremental_terminal_admissibility_hits = legality_stats.terminal_admissibility_hits;
        incremental_terminal_admissibility_rejections =
            legality_stats.terminal_admissibility_rejections;
        incremental_terminal_prefix_completion_hits =
            legality_stats.terminal_prefix_completion_hits;
        incremental_terminal_prefix_rank_hits = legality_stats.terminal_prefix_rank_hits;
        incremental_terminal_rank_prunes = discovery.terminal_rank_prunes;
        incremental_partial_prefix_bound_hits = legality_stats.partial_prefix_bound_hits;
        incremental_partial_prefix_bound_checks = discovery.partial_prefix_bound_checks;
        incremental_partial_prefix_bound_prunes = discovery.partial_prefix_bound_prunes;
        incremental_terminal_prefix_bar_prunes = discovery.terminal_prefix_bar_prunes;
        demo_phase.breadth_harvest_exit_reason = discovery_stop_reason;
    } else {
        for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
            let telescopes = enumerate_telescopes(
                library,
                EnumerationContext::from_admissibility(library, admissibility),
                clause_kappa,
            );
            enumerated_candidates += telescopes.len();

            for telescope in telescopes {
                match check_telescope(library, &telescope) {
                    CheckResult::Ok => {
                        well_formed_candidates += 1;
                    }
                    CheckResult::Err(error) => {
                        malformed_rejections += 1;
                        *malformed_rejection_reasons
                            .entry(error.kind_label().to_owned())
                            .or_insert(0) += 1;
                        continue;
                    }
                }
                let admissibility_decision =
                    assess_strict_admissibility(step_index, library, &telescope, admissibility);
                admissibility_diagnostics.record(&admissibility_decision);
                if !admissibility_decision.is_admitted() {
                    admissibility_rejections += 1;
                    continue;
                }
                let candidate = evaluate_checked_candidate(library, history, telescope)?;
                candidates.push(candidate);
            }
        }
        generated_raw_surface = usize::try_from(generated_surface_from_counts(
            prefixes_created,
            enumerated_candidates,
        ))
        .expect("generated surface exceeded usize");
        if let (Some(observer), Some(budget)) = (demo_narrative.as_mut(), demo_step_budget) {
            observer.close_discovery(
                narrative_progress_snapshot(
                    elapsed_millis(step_start.elapsed()),
                    generated_raw_surface as u64,
                    exact_screened_surface_from_counts(0, candidates.len()),
                    candidates.len() as u64,
                ),
                format!(
                    "generated_per_sec={} admissibility_per_sec={} exact_bound_per_sec=0 full_eval_per_sec={} generated={} admissibility_checks={} exact_bound_checks=0 full_evals={}",
                    per_second(
                        generated_raw_surface as u64,
                        elapsed_millis(step_start.elapsed())
                    ),
                    per_second(well_formed_candidates as u64, elapsed_millis(step_start.elapsed())),
                    per_second(candidates.len() as u64, elapsed_millis(step_start.elapsed())),
                    generated_raw_surface,
                    well_formed_candidates,
                    candidates.len()
                ),
                demo_surface_status_detail(
                    budget,
                    generated_raw_surface as u64,
                    exact_screened_surface_from_counts(0, candidates.len()),
                    candidates.len() as u64,
                ),
                None,
            );
        }
    }
    if let Some(budget) = demo_step_budget {
        demo_phase.full_eval_soft_cap = budget.full_eval_soft_cap;
        demo_phase.generated_floor = budget.generated_floor;
        demo_phase.exact_screened_floor = budget.exact_screened_floor;
        demo_phase.proof_close_reserved_millis = budget.proof_close_reserve_millis;
        demo_phase.proof_close_remaining_millis = budget.proof_close_reserve_millis;
    }
    let candidate_discovery_wall_clock_millis = elapsed_millis(discovery_start.elapsed());

    let prefix_frontier_planning_start = Instant::now();
    let prefix_frontier = if matches!(
        admissibility_mode,
        AdmissibilityMode::RealisticShadow
            | AdmissibilityMode::DemoBreadthShadow
            | AdmissibilityMode::DesktopClaimShadow
    ) {
        build_prefix_frontier_plan(
            prefix_states_explored,
            step_index,
            objective_bar,
            retention_policy,
            retention_runtime,
            &prefix_cache,
        )
    } else {
        PrefixFrontierPlan::default()
    };
    let prefix_frontier_planning_wall_clock_millis =
        elapsed_millis(prefix_frontier_planning_start.elapsed());
    let prefix_states_exact_pruned = prefix_frontier.exact_pruned
        + incremental_clause_family_prunes
        + incremental_partial_prefix_bound_prunes
        + incremental_terminal_prefix_bar_prunes;
    let retained_prefix_groups = prefix_frontier.retained_prefix_signatures.len();
    if claim_live_checkpoint_enabled(admissibility.mode, step_index) {
        maybe_emit_claim_live_checkpoint(
            progress_observer,
            &mut last_live_checkpoint_elapsed_millis,
            StepLiveCheckpoint {
                step_index,
                phase: LiveStepCheckpointPhase::Materialize,
                elapsed_millis: elapsed_millis(step_start.elapsed()),
                clause_kappa: None,
                raw_catalog_clause_widths: Vec::new(),
                raw_catalog_telescope_count: None,
                generated_raw_surface,
                enumerated_candidates,
                well_formed_candidates,
                admissibility_rejections,
                prefixes_created,
                prefix_states_explored,
                frontier_queue_len: retained_prefix_groups,
                candidate_pool_len: candidates.len(),
                prefix_cache_groups: prefix_cache.len(),
                prefix_cache_candidates: prefix_cache.candidate_count(),
                legality_cache_entries: LiveLegalityCacheEntries::default(),
                exact_screen_prunes: prefix_states_exact_pruned
                    + incremental_connectivity_prunes
                    + incremental_terminal_clause_filter_prunes
                    + incremental_terminal_rank_prunes,
                claim_surface: None,
                remaining_one_telemetry: None,
                note: Some("claim_materialize_entry".to_owned()),
            },
            true,
        );
    }
    if let (Some(observer), Some(budget)) = (demo_narrative.as_mut(), demo_step_budget) {
        observer.enter_materialize(
            narrative_progress_snapshot(
                elapsed_millis(step_start.elapsed()),
                generated_raw_surface as u64,
                late_demo_exact_screened_surface(
                    prefix_states_exact_pruned,
                    incremental_connectivity_prunes,
                    incremental_terminal_clause_filter_prunes,
                    incremental_terminal_rank_prunes,
                    candidates.len(),
                    budget.exact_screened_floor,
                ),
                candidates.len() as u64,
            ),
            demo_materialize_status_detail(
                budget,
                generated_raw_surface as u64,
                late_demo_exact_screened_surface(
                    prefix_states_exact_pruned,
                    incremental_connectivity_prunes,
                    incremental_terminal_clause_filter_prunes,
                    incremental_terminal_rank_prunes,
                    candidates.len(),
                    budget.exact_screened_floor,
                ),
                candidates.len() as u64,
                retained_prefix_groups,
                &demo_phase,
            ),
        );
    }
    if demo_step_budget.is_some() {
        demo_phase.materialize_full_evals = candidates.len();
    }

    let mut demo_proof_close_entered = false;
    let mut demo_proof_close_started_elapsed_millis = None::<u64>;
    let mut demo_proof_close_total_groups = 0usize;
    let mut demo_proof_close_closed_groups = 0usize;
    if matches!(
        admissibility_mode,
        AdmissibilityMode::RealisticShadow
            | AdmissibilityMode::DemoBreadthShadow
            | AdmissibilityMode::DesktopClaimShadow
    ) {
        let mut incumbent_terminal_rank = None;
        let mut pending_group_signatures = prefix_frontier.retained_prefix_signatures.clone();
        while !pending_group_signatures.is_empty() {
            if claim_live_checkpoint_enabled(admissibility.mode, step_index) {
                maybe_emit_claim_live_checkpoint(
                    progress_observer,
                    &mut last_live_checkpoint_elapsed_millis,
                    StepLiveCheckpoint {
                        step_index,
                        phase: LiveStepCheckpointPhase::ProofClose,
                        elapsed_millis: elapsed_millis(step_start.elapsed()),
                        clause_kappa: None,
                        raw_catalog_clause_widths: Vec::new(),
                        raw_catalog_telescope_count: None,
                        generated_raw_surface,
                        enumerated_candidates,
                        well_formed_candidates,
                        admissibility_rejections,
                        prefixes_created,
                        prefix_states_explored,
                        frontier_queue_len: pending_group_signatures.len(),
                        candidate_pool_len: candidates.len(),
                        prefix_cache_groups: prefix_cache.len(),
                        prefix_cache_candidates: prefix_cache.candidate_count(),
                        legality_cache_entries: LiveLegalityCacheEntries::default(),
                        exact_screen_prunes: prefix_states_exact_pruned
                            + incremental_connectivity_prunes
                            + incremental_terminal_clause_filter_prunes
                            + incremental_terminal_rank_prunes,
                        claim_surface: None,
                        remaining_one_telemetry: None,
                        note: Some("claim_proof_close_progress".to_owned()),
                    },
                    false,
                );
            }
            let remaining_groups = pending_group_signatures.len();
            if !demo_proof_close_entered {
                if let Some(budget) = demo_step_budget {
                    if let Some(proof_close_entry_reason) =
                        demo_materialize_to_proof_close_handoff_reason(
                            budget,
                            &pending_group_signatures,
                            &prefix_cache,
                            incumbent_terminal_rank.as_ref(),
                            demo_phase.materialize_full_evals,
                        )
                    {
                        demo_proof_close_started_elapsed_millis = Some(begin_demo_proof_close(
                            &mut demo_phase,
                            &mut demo_narrative,
                            budget,
                            proof_close_entry_reason,
                            remaining_groups,
                            step_start,
                            generated_raw_surface as u64,
                            late_demo_exact_screen_pruned(
                                prefix_states_exact_pruned,
                                incremental_connectivity_prunes,
                                incremental_terminal_clause_filter_prunes,
                                incremental_terminal_rank_prunes,
                                budget.exact_screened_floor,
                            ),
                            candidates.len(),
                        ));
                        demo_proof_close_total_groups = remaining_groups;
                        demo_proof_close_closed_groups = 0;
                        demo_proof_close_entered = true;
                    }
                }
            }
            let remaining_reserve_millis = if demo_proof_close_entered {
                demo_phase.proof_close_remaining_millis
            } else {
                demo_step_budget
                    .map(|budget| budget.proof_close_reserve_millis)
                    .unwrap_or(0)
            };
            let group_index = if demo_step_budget.is_some() {
                select_demo_proof_close_group_index(
                    &pending_group_signatures,
                    &prefix_cache,
                    admissibility.mode,
                    &demo_bucket_stats,
                    incumbent_terminal_rank.as_ref(),
                    remaining_reserve_millis,
                )
                .unwrap_or(0)
            } else {
                0
            };
            let signature = pending_group_signatures.remove(group_index);
            let Some(group) = load_terminal_prefix_group_for_proof_close(
                &mut prefix_cache,
                &signature,
                admissibility.mode,
            ) else {
                if demo_proof_close_entered {
                    demo_proof_close_closed_groups += 1;
                    if let (Some(proof_close_started_elapsed), Some(budget)) =
                        (demo_proof_close_started_elapsed_millis, demo_step_budget)
                    {
                        sync_demo_proof_close_runtime(
                            &mut demo_phase,
                            &mut demo_narrative,
                            budget,
                            proof_close_started_elapsed,
                            demo_proof_close_total_groups,
                            demo_proof_close_closed_groups,
                            step_start,
                            generated_raw_surface as u64,
                            late_demo_exact_screen_pruned(
                                prefix_states_exact_pruned,
                                incremental_connectivity_prunes,
                                incremental_terminal_clause_filter_prunes,
                                incremental_terminal_rank_prunes,
                                budget.exact_screened_floor,
                            ),
                            candidates.len(),
                        );
                    }
                }
                continue;
            };
            let bucket_key = demo_bucket_key_for_group(admissibility.mode, &signature, &group);
            if let (Some(group_best_rank), Some(incumbent_rank)) =
                (&group.best_accept_rank, &incumbent_terminal_rank)
            {
                if !better_rank(group_best_rank, incumbent_rank) {
                    incremental_terminal_rank_prunes += group.candidates.len();
                    demo_bucket_stats
                        .entry(bucket_key.clone())
                        .or_default()
                        .pruned_terminal_candidates += group.candidates.len();
                    if demo_proof_close_entered {
                        demo_proof_close_closed_groups += 1;
                        if let (Some(proof_close_started_elapsed), Some(budget)) =
                            (demo_proof_close_started_elapsed_millis, demo_step_budget)
                        {
                            sync_demo_proof_close_runtime(
                                &mut demo_phase,
                                &mut demo_narrative,
                                budget,
                                proof_close_started_elapsed,
                                demo_proof_close_total_groups,
                                demo_proof_close_closed_groups,
                                step_start,
                                generated_raw_surface as u64,
                                late_demo_exact_screen_pruned(
                                    prefix_states_exact_pruned,
                                    incremental_connectivity_prunes,
                                    incremental_terminal_clause_filter_prunes,
                                    incremental_terminal_rank_prunes,
                                    budget.exact_screened_floor,
                                ),
                                candidates.len(),
                            );
                        }
                    }
                    continue;
                }
            }

            let mut group_candidates = group.candidates;
            if demo_step_budget.is_some() {
                sort_terminal_prefix_group_candidates_for_certification(&mut group_candidates);
            } else {
                group_candidates.sort_by_key(|candidate| {
                    serde_json::to_string(&candidate.telescope).expect("serialize")
                });
            }
            for group_candidate in group_candidates {
                if !demo_proof_close_entered {
                    if let Some(full_eval_soft_cap) = demo_phase.full_eval_soft_cap {
                        if u64::try_from(candidates.len()).expect("candidate count exceeded u64")
                            >= full_eval_soft_cap
                        {
                            demo_phase.materialize_soft_cap_triggered = true;
                            demo_phase.materialize_full_evals = candidates.len();
                            demo_proof_close_started_elapsed_millis =
                                demo_step_budget.map(|budget| {
                                    begin_demo_proof_close(
                                        &mut demo_phase,
                                        &mut demo_narrative,
                                        budget,
                                        DemoProofCloseEntryReason::SoftCapHandoff,
                                        remaining_groups,
                                        step_start,
                                        generated_raw_surface as u64,
                                        late_demo_exact_screen_pruned(
                                            prefix_states_exact_pruned,
                                            incremental_connectivity_prunes,
                                            incremental_terminal_clause_filter_prunes,
                                            incremental_terminal_rank_prunes,
                                            budget.exact_screened_floor,
                                        ),
                                        candidates.len(),
                                    )
                                });
                            demo_proof_close_total_groups = remaining_groups;
                            demo_proof_close_closed_groups = 0;
                            demo_proof_close_entered = true;
                        }
                    }
                }
                if let (Some(candidate_rank), Some(incumbent_rank)) =
                    (&group_candidate.accept_rank, &incumbent_terminal_rank)
                {
                    if !better_rank(candidate_rank, incumbent_rank) {
                        incremental_terminal_rank_prunes += 1;
                        demo_bucket_stats
                            .entry(bucket_key.clone())
                            .or_default()
                            .pruned_terminal_candidates += 1;
                        continue;
                    }
                }

                let candidate = match group_candidate.evaluated_candidate.clone() {
                    Some(candidate) => candidate,
                    None => evaluate_checked_candidate(
                        library,
                        history,
                        group_candidate.telescope.clone(),
                    )?,
                };
                if let Some(rank) = acceptance_rank(objective_bar, &candidate) {
                    let witness = analyze_semantic_minimality(
                        step_index,
                        objective_bar,
                        admissibility,
                        &candidate.telescope,
                        library,
                        &nu_history,
                    );
                    let better_than_incumbent = incumbent_terminal_rank
                        .as_ref()
                        .map(|current| better_rank(&rank, current))
                        .unwrap_or(true);
                    if witness.is_minimal() && better_than_incumbent {
                        incumbent_terminal_rank = Some(rank);
                    }
                }
                let bucket_stats = demo_bucket_stats.entry(bucket_key.clone()).or_default();
                bucket_stats.fully_scored_terminal_candidates += 1;
                if candidate.rho >= objective_bar {
                    let overshoot = candidate.rho - objective_bar;
                    match bucket_stats.best_overshoot {
                        Some(current) if current <= overshoot => {}
                        _ => bucket_stats.best_overshoot = Some(overshoot),
                    }
                }
                candidates.push(candidate);
                if demo_step_budget.is_some() {
                    if demo_proof_close_entered {
                        demo_phase.proof_close_full_evals += 1;
                        if demo_phase.materialize_soft_cap_triggered {
                            demo_phase.proof_close_overrun_full_evals += 1;
                            demo_phase
                                .proof_close_overrun_reason
                                .get_or_insert(DemoProofCloseOverrunReason::SoftCapHandoff);
                        }
                    } else {
                        demo_phase.materialize_full_evals = candidates.len();
                    }
                }
            }
            if demo_proof_close_entered {
                demo_proof_close_closed_groups += 1;
                if let (Some(proof_close_started_elapsed), Some(budget)) =
                    (demo_proof_close_started_elapsed_millis, demo_step_budget)
                {
                    sync_demo_proof_close_runtime(
                        &mut demo_phase,
                        &mut demo_narrative,
                        budget,
                        proof_close_started_elapsed,
                        demo_proof_close_total_groups,
                        demo_proof_close_closed_groups,
                        step_start,
                        generated_raw_surface as u64,
                        late_demo_exact_screen_pruned(
                            prefix_states_exact_pruned,
                            incremental_connectivity_prunes,
                            incremental_terminal_clause_filter_prunes,
                            incremental_terminal_rank_prunes,
                            budget.exact_screened_floor,
                        ),
                        candidates.len(),
                    );
                }
            }
        }
    }

    if candidates.is_empty() {
        bail!("no atomic candidates were generated for step {step_index}");
    }
    if demo_step_budget.is_some() && !demo_proof_close_entered {
        demo_phase.materialize_full_evals = candidates.len();
        let proof_close_entry_reason =
            demo_proof_close_entry_reason(demo_phase.breadth_harvest_exit_reason);
        if let Some(budget) = demo_step_budget {
            begin_demo_proof_close(
                &mut demo_phase,
                &mut demo_narrative,
                budget,
                proof_close_entry_reason,
                0,
                step_start,
                generated_raw_surface as u64,
                late_demo_exact_screen_pruned(
                    prefix_states_exact_pruned,
                    incremental_connectivity_prunes,
                    incremental_terminal_clause_filter_prunes,
                    incremental_terminal_rank_prunes,
                    budget.exact_screened_floor,
                ),
                candidates.len(),
            );
        }
    }
    let selection_start = Instant::now();
    let evaluated_candidates = candidates.len();
    let full_telescopes_evaluated = evaluated_candidates;
    let scored_candidate_distribution = candidate_score_distribution(&candidates, objective_bar);

    let mut seen_canonical = BTreeMap::new();
    let mut deduped = Vec::new();
    let mut dedupe_pruned_candidates = Vec::new();
    for candidate in candidates {
        if let Some(first_seen_candidate_hash) =
            seen_canonical.get(&candidate.canonical_hash).cloned()
        {
            if dedupe_pruned_candidates.len() < MAX_PRUNE_SAMPLES {
                dedupe_pruned_candidates.push(DedupePruneEvidence {
                    candidate,
                    first_seen_candidate_hash,
                });
            }
            continue;
        }
        seen_canonical.insert(
            candidate.canonical_hash.clone(),
            candidate.candidate_hash.clone(),
        );
        deduped.push(candidate);
    }
    let dedupe_prunes = evaluated_candidates.saturating_sub(deduped.len());
    let canonical_dedupe_prunes = dedupe_prunes;
    let deduped_len = deduped.len();
    let canonical_candidates = deduped_len;
    let mut minimal = Vec::new();
    let mut minimality_pruned_candidates = Vec::new();
    for candidate in deduped {
        let witness = analyze_semantic_minimality(
            step_index,
            objective_bar,
            admissibility,
            &candidate.telescope,
            library,
            &nu_history,
        );
        if witness.is_minimal() {
            minimal.push(candidate);
        } else if minimality_pruned_candidates.len() < MAX_PRUNE_SAMPLES {
            minimality_pruned_candidates.push(MinimalityPruneEvidence {
                candidate: candidate.clone(),
                admissible_bar_clear_subbundles: witness.admissible_bar_clear_subbundles.len(),
            });
        }
    }
    let minimality_prunes = deduped_len.saturating_sub(minimal.len());
    let semantic_minimality_prunes = minimality_prunes;
    // Exact acceptance still competes over the full semantically minimal pool.
    let retained = minimal;
    let semantically_minimal_candidates = retained.len();
    if retained.is_empty() {
        bail!("no semantically minimal candidates survived for step {step_index}");
    }
    let acceptance = select_acceptance(objective_bar, &retained)
        .ok_or_else(|| anyhow::anyhow!("no candidate cleared the bar at step {step_index}"))?;
    let frontier_retention = build_frontier_retention(
        objective_bar,
        retention_policy,
        &retained,
        retention_runtime,
    );
    if let (Some(observer), Some(budget)) = (demo_narrative.as_mut(), demo_step_budget) {
        let bar_clearers = retained
            .iter()
            .filter(|candidate| candidate.rho >= objective_bar)
            .count();
        observer.enter_seal(
            narrative_progress_snapshot(
                elapsed_millis(step_start.elapsed()),
                generated_raw_surface as u64,
                late_demo_exact_screened_surface(
                    prefix_states_exact_pruned,
                    incremental_connectivity_prunes,
                    incremental_terminal_clause_filter_prunes,
                    incremental_terminal_rank_prunes,
                    full_telescopes_evaluated,
                    budget.exact_screened_floor,
                ),
                full_telescopes_evaluated as u64,
            ),
            format!(
                "bar_clearers={} near_misses={} {}",
                bar_clearers,
                acceptance.near_misses.len(),
                demo_surface_status_detail(
                    budget,
                    generated_raw_surface as u64,
                    late_demo_exact_screened_surface(
                        prefix_states_exact_pruned,
                        incremental_connectivity_prunes,
                        incremental_terminal_clause_filter_prunes,
                        incremental_terminal_rank_prunes,
                        full_telescopes_evaluated,
                        budget.exact_screened_floor,
                    ),
                    full_telescopes_evaluated as u64
                )
            ),
            &acceptance.accepted.candidate_hash,
            acceptance.accepted.rho - objective_bar,
        );
    }
    let heuristic_drops = frontier_retention.dropped_candidates.len();
    let search_timing = SearchTiming {
        step_wall_clock_millis: elapsed_millis(step_start.elapsed()),
        candidate_discovery_wall_clock_millis,
        prefix_frontier_planning_wall_clock_millis,
        selection_wall_clock_millis: elapsed_millis(selection_start.elapsed()),
    };
    let narrative_events = demo_narrative
        .map(DemoNarrativeRuntime::finish)
        .unwrap_or_default();

    build_step_result(
        step_index,
        objective_bar,
        admissibility,
        admissibility_diagnostics,
        generated_raw_surface,
        prefixes_created,
        prefix_frontier.explored,
        prefix_cache.stats().merged_by_signature,
        prefix_states_exact_pruned,
        prefix_frontier.heuristic_dropped,
        incremental_legality_cache_hits,
        incremental_connectivity_shortcuts,
        incremental_connectivity_fallbacks,
        incremental_connectivity_prunes,
        incremental_clause_family_filter_hits,
        incremental_clause_family_prunes,
        incremental_active_window_clause_filter_hits,
        incremental_active_window_clause_filter_prunes,
        incremental_terminal_clause_filter_hits,
        incremental_terminal_clause_filter_prunes,
        incremental_trivial_derivability_hits,
        incremental_trivial_derivability_prunes,
        incremental_terminal_admissibility_hits,
        incremental_terminal_admissibility_rejections,
        incremental_terminal_prefix_completion_hits,
        incremental_terminal_prefix_rank_hits,
        incremental_terminal_rank_prunes,
        incremental_partial_prefix_bound_hits,
        incremental_partial_prefix_bound_checks,
        incremental_partial_prefix_bound_prunes,
        incremental_terminal_prefix_bar_prunes,
        ExactScreenReasonStats::from_incremental_counters(
            incremental_connectivity_prunes,
            incremental_terminal_clause_filter_prunes,
            incremental_terminal_rank_prunes,
            incremental_partial_prefix_bound_prunes,
            incremental_terminal_prefix_bar_prunes,
        ),
        demo_phase,
        demo_bucket_stats,
        search_timing,
        prefix_frontier.frontier.hot.len(),
        prefix_frontier.frontier.cold.len(),
        retention_policy,
        if matches!(
            admissibility_mode,
            AdmissibilityMode::RealisticShadow
                | AdmissibilityMode::DemoBreadthShadow
                | AdmissibilityMode::DesktopClaimShadow
        ) {
            prefix_frontier.pressure
        } else {
            frontier_retention.pressure
        },
        frontier_retention,
        prefix_frontier.frontier,
        prefix_frontier.dedupe_keys,
        acceptance,
        enumerated_candidates,
        well_formed_candidates,
        malformed_rejections,
        malformed_rejection_reasons,
        admissibility_rejections,
        full_telescopes_evaluated,
        evaluated_candidates,
        canonical_candidates,
        semantically_minimal_candidates,
        scored_candidate_distribution,
        canonical_dedupe_prunes,
        semantic_minimality_prunes,
        dedupe_prunes,
        minimality_prunes,
        heuristic_drops,
        dedupe_pruned_candidates,
        minimality_pruned_candidates,
        &retained,
        narrative_events,
    )
}

fn build_step_result(
    step_index: u32,
    objective_bar: Rational,
    admissibility: StrictAdmissibility,
    admissibility_diagnostics: AdmissibilityDiagnostics,
    generated_raw_surface: usize,
    prefixes_created: usize,
    prefix_states_explored: usize,
    prefix_states_merged_by_signature: usize,
    prefix_states_exact_pruned: usize,
    prefix_states_heuristic_dropped: usize,
    incremental_legality_cache_hits: usize,
    incremental_connectivity_shortcuts: usize,
    incremental_connectivity_fallbacks: usize,
    incremental_connectivity_prunes: usize,
    incremental_clause_family_filter_hits: usize,
    incremental_clause_family_prunes: usize,
    incremental_active_window_clause_filter_hits: usize,
    incremental_active_window_clause_filter_prunes: usize,
    incremental_terminal_clause_filter_hits: usize,
    incremental_terminal_clause_filter_prunes: usize,
    incremental_trivial_derivability_hits: usize,
    incremental_trivial_derivability_prunes: usize,
    incremental_terminal_admissibility_hits: usize,
    incremental_terminal_admissibility_rejections: usize,
    incremental_terminal_prefix_completion_hits: usize,
    incremental_terminal_prefix_rank_hits: usize,
    incremental_terminal_rank_prunes: usize,
    incremental_partial_prefix_bound_hits: usize,
    incremental_partial_prefix_bound_checks: usize,
    incremental_partial_prefix_bound_prunes: usize,
    incremental_terminal_prefix_bar_prunes: usize,
    exact_screen_reasons: ExactScreenReasonStats,
    demo_phase: DemoPhaseStats,
    demo_bucket_stats: BTreeMap<DemoBucketKey, DemoBucketStats>,
    search_timing: SearchTiming,
    prefix_frontier_hot_states: usize,
    prefix_frontier_cold_states: usize,
    retention_policy: RetentionPolicy,
    frontier_pressure: FrontierPressure,
    frontier_retention: FrontierRetentionOutcome,
    frontier_window: FrontierWindow,
    frontier_dedupe_keys: BTreeSet<String>,
    acceptance: AcceptanceOutcome,
    enumerated_candidates: usize,
    well_formed_candidates: usize,
    malformed_rejections: usize,
    malformed_rejection_reasons: BTreeMap<String, usize>,
    admissibility_rejections: usize,
    full_telescopes_evaluated: usize,
    evaluated_candidates: usize,
    canonical_candidates: usize,
    semantically_minimal_candidates: usize,
    scored_candidate_distribution: CandidateScoreDistribution,
    canonical_dedupe_prunes: usize,
    semantic_minimality_prunes: usize,
    dedupe_prunes: usize,
    minimality_prunes: usize,
    heuristic_drops: usize,
    dedupe_pruned_candidates: Vec<DedupePruneEvidence>,
    minimality_pruned_candidates: Vec<MinimalityPruneEvidence>,
    retained: &[ExpandedCandidate],
    narrative_events: Vec<NarrativeEvent>,
) -> Result<AtomicSearchStep> {
    let accepted = retained
        .iter()
        .find(|candidate| candidate.candidate_hash == acceptance.accepted.candidate_hash)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("accepted candidate vanished during selection"))?;
    let exact_screen_pruned = late_demo_exact_screen_pruned(
        prefix_states_exact_pruned,
        incremental_connectivity_prunes,
        incremental_terminal_clause_filter_prunes,
        incremental_terminal_rank_prunes,
        demo_phase.exact_screened_floor,
    );
    let demo_funnel = build_demo_funnel_stats(
        objective_bar,
        generated_raw_surface,
        prefixes_created,
        enumerated_candidates,
        well_formed_candidates,
        admissibility_rejections,
        exact_screen_pruned,
        prefix_states_heuristic_dropped,
        heuristic_drops,
        full_telescopes_evaluated,
        &scored_candidate_distribution,
        retained,
        accepted.rho - objective_bar,
    );
    let demo_closure = build_demo_closure_stats(&demo_funnel);

    let mut step = AtomicSearchStep {
        step_index,
        objective_bar,
        admissibility,
        admissibility_diagnostics,
        prefixes_created,
        prefix_states_explored,
        prefix_states_merged_by_signature,
        prefix_states_exact_pruned,
        prefix_states_heuristic_dropped,
        incremental_legality_cache_hits,
        incremental_connectivity_shortcuts,
        incremental_connectivity_fallbacks,
        incremental_connectivity_prunes,
        incremental_clause_family_filter_hits,
        incremental_clause_family_prunes,
        incremental_active_window_clause_filter_hits,
        incremental_active_window_clause_filter_prunes,
        incremental_terminal_clause_filter_hits,
        incremental_terminal_clause_filter_prunes,
        incremental_trivial_derivability_hits,
        incremental_trivial_derivability_prunes,
        incremental_terminal_admissibility_hits,
        incremental_terminal_admissibility_rejections,
        incremental_terminal_prefix_completion_hits,
        incremental_terminal_prefix_rank_hits,
        incremental_terminal_rank_prunes,
        incremental_partial_prefix_bound_hits,
        incremental_partial_prefix_bound_checks,
        incremental_partial_prefix_bound_prunes,
        incremental_terminal_prefix_bar_prunes,
        exact_screen_reasons,
        demo_phase,
        demo_funnel,
        demo_closure,
        demo_bucket_stats: demo_bucket_reports(&demo_bucket_stats),
        search_timing,
        prefix_frontier_hot_states,
        prefix_frontier_cold_states,
        retention_policy,
        frontier_pressure,
        frontier_retention,
        frontier_window,
        frontier_dedupe_keys,
        narrative_events: Vec::new(),
        telescope: accepted.telescope.clone(),
        accepted,
        retained_candidates: retained.to_vec(),
        near_misses: acceptance.near_misses,
        enumerated_candidates,
        well_formed_candidates,
        malformed_rejections,
        malformed_rejection_reasons,
        admissibility_rejections,
        full_telescopes_evaluated,
        evaluated_candidates,
        canonical_candidates,
        semantically_minimal_candidates,
        scored_candidate_distribution,
        canonical_dedupe_prunes,
        semantic_minimality_prunes,
        dedupe_prunes,
        minimality_prunes,
        heuristic_drops,
        dedupe_pruned_candidates,
        minimality_pruned_candidates,
    };
    step.narrative_events = append_step_narrative_events(&step, narrative_events);
    Ok(step)
}

fn build_demo_funnel_stats(
    objective_bar: Rational,
    generated_raw_surface: usize,
    prefixes_created: usize,
    enumerated_candidates: usize,
    well_formed_candidates: usize,
    admissibility_rejections: usize,
    exact_screen_pruned: usize,
    prefix_states_heuristic_dropped: usize,
    heuristic_drops: usize,
    full_telescopes_evaluated: usize,
    scored_candidate_distribution: &CandidateScoreDistribution,
    retained: &[ExpandedCandidate],
    winner_overshoot: Rational,
) -> DemoFunnelStats {
    DemoFunnelStats {
        generated_raw_prefixes: generated_raw_surface,
        // Prefix creation is already signature-gated on the live path, so the
        // currently honest canonical-prefix surface is the stored unique
        // created-prefix count for prefix search and the enumerated count for
        // early exhaustive steps.
        canonical_prefix_signatures: if prefixes_created > 0 {
            prefixes_created
        } else {
            enumerated_candidates
        },
        well_formed_terminals: well_formed_candidates,
        hard_admissible: well_formed_candidates.saturating_sub(admissibility_rejections),
        exact_bound_screened: exact_screen_pruned.saturating_add(full_telescopes_evaluated),
        exact_bound_pruned: exact_screen_pruned,
        heuristic_dropped: prefix_states_heuristic_dropped.saturating_add(heuristic_drops),
        full_telescopes_evaluated,
        bar_clearers: scored_candidate_distribution.clears_bar,
        semantically_minimal_clearers: retained
            .iter()
            .filter(|candidate| candidate.rho >= objective_bar)
            .count(),
        winner_overshoot,
    }
}

fn late_demo_exact_screen_pruned(
    prefix_states_exact_pruned: usize,
    incremental_connectivity_prunes: usize,
    incremental_terminal_clause_filter_prunes: usize,
    incremental_terminal_rank_prunes: usize,
    exact_screened_floor: Option<u64>,
) -> usize {
    if exact_screened_floor.is_some() {
        prefix_states_exact_pruned
            .saturating_add(incremental_connectivity_prunes)
            .saturating_add(incremental_terminal_clause_filter_prunes)
            .saturating_add(incremental_terminal_rank_prunes)
    } else {
        prefix_states_exact_pruned
    }
}

fn late_demo_exact_screened_surface(
    prefix_states_exact_pruned: usize,
    incremental_connectivity_prunes: usize,
    incremental_terminal_clause_filter_prunes: usize,
    incremental_terminal_rank_prunes: usize,
    full_telescopes_evaluated: usize,
    exact_screened_floor: Option<u64>,
) -> u64 {
    exact_screened_surface_from_counts(
        late_demo_exact_screen_pruned(
            prefix_states_exact_pruned,
            incremental_connectivity_prunes,
            incremental_terminal_clause_filter_prunes,
            incremental_terminal_rank_prunes,
            exact_screened_floor,
        ),
        full_telescopes_evaluated,
    )
}

fn build_demo_closure_stats(funnel: &DemoFunnelStats) -> DemoClosureStats {
    let frontier_total_seen = funnel.exact_bound_screened;
    let frontier_certified_nonwinning = funnel.exact_bound_pruned;
    let closure_percent = if frontier_total_seen == 0 {
        0
    } else {
        u16::try_from(
            (u128::from(u64::try_from(frontier_certified_nonwinning).expect("usize exceeded u64"))
                * 100)
                / u128::from(u64::try_from(frontier_total_seen).expect("usize exceeded u64")),
        )
        .expect("closure percent exceeded u16")
    };

    DemoClosureStats {
        frontier_total_seen,
        frontier_certified_nonwinning,
        closure_percent,
    }
}

fn admissibility_mode_for_profile(search_profile: SearchProfile) -> AdmissibilityMode {
    match search_profile {
        SearchProfile::StrictCanonGuarded | SearchProfile::Unknown => AdmissibilityMode::Guarded,
        SearchProfile::RelaxedShadow => AdmissibilityMode::RelaxedShadow,
        SearchProfile::DesktopClaimShadow => AdmissibilityMode::DesktopClaimShadow,
        SearchProfile::RealisticFrontierShadow | SearchProfile::DemoBreadthShadow => {
            AdmissibilityMode::RealisticShadow
        }
    }
}

fn build_frontier_retention(
    objective_bar: Rational,
    retention_policy: RetentionPolicy,
    retained: &[ExpandedCandidate],
    retention_runtime: FrontierRuntimeLimits,
) -> FrontierRetentionOutcome {
    let hot_count = retained
        .iter()
        .filter(|candidate| candidate.rho >= objective_bar)
        .count();
    let mut below_bar = retained
        .iter()
        .filter(|candidate| candidate.rho < objective_bar)
        .cloned()
        .collect::<Vec<_>>();
    below_bar.sort_by_key(|candidate| {
        let retention_class = retention_policy.classify(candidate.retention_signals());
        (
            retention_class.priority_rank(),
            objective_bar - candidate.rho,
            candidate.clause_kappa,
            std::cmp::Reverse(candidate.nu),
            candidate.canonical_hash.clone(),
        )
    });

    let plan = plan_pressure_cold_retention(
        below_bar,
        hot_count,
        retention_policy,
        retention_runtime,
        |candidate| retention_policy.classify(candidate.retention_signals()),
    );

    FrontierRetentionOutcome {
        pressure: plan.pressure,
        resident_cold_candidates: plan
            .resident
            .iter()
            .map(|candidate| candidate.candidate_hash.clone())
            .collect(),
        spill_cold_candidates: plan
            .spill
            .iter()
            .map(|candidate| candidate.candidate_hash.clone())
            .collect(),
        dropped_candidates: plan
            .dropped
            .iter()
            .map(|candidate| candidate.candidate_hash.clone())
            .collect(),
    }
}

fn discover_realistic_shadow_candidates(
    step_index: u32,
    library: &Library,
    history: &[DiscoveryRecord],
    admissibility: StrictAdmissibility,
    retention_policy: RetentionPolicy,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    demo_step_budget: &mut Option<DemoStepBudget>,
    step_start: Instant,
    demo_narrative: &mut Option<DemoNarrativeRuntime>,
    progress_observer: &mut Option<&mut dyn AtomicSearchProgressObserver>,
) -> Result<RealisticShadowDiscovery> {
    if demo_step_budget.is_some() && step_index <= 4 {
        return discover_demo_early_exhaustive_candidates(
            step_index,
            library,
            history,
            admissibility,
            step_start,
            demo_step_budget,
            demo_narrative,
            progress_observer,
        );
    }

    let mut discovery = RealisticShadowDiscovery::default();
    let enumeration_context =
        discovery_enumeration_context(library, admissibility, demo_step_budget.is_some());
    let mut last_checkpoint_elapsed_millis = 0u64;
    let claim_surface = claim_live_checkpoint_enabled(admissibility.mode, step_index)
        .then(|| enumeration_context.surface_diagnostics());

    for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
        if claim_replay::capture_should_stop() {
            break;
        }
        if demo_discovery_budget_exhausted(
            demo_step_budget,
            &mut discovery,
            step_start,
            demo_narrative.as_mut(),
        ) {
            break;
        }
        if clause_kappa <= 1 {
            let telescopes = enumerate_telescopes(library, enumeration_context, clause_kappa);
            discovery.raw_generated_surface += telescopes.len();
            discovery.enumerated_candidates += telescopes.len();
            for telescope in telescopes {
                if demo_discovery_budget_exhausted(
                    demo_step_budget,
                    &mut discovery,
                    step_start,
                    demo_narrative.as_mut(),
                ) {
                    break;
                }
                discovery.well_formed_candidates += 1;
                let admissibility_decision =
                    assess_strict_admissibility(step_index, library, &telescope, admissibility);
                discovery
                    .admissibility_diagnostics
                    .record(&admissibility_decision);
                if !admissibility_decision.is_admitted() {
                    discovery.admissibility_rejections += 1;
                    continue;
                }
                discovery
                    .candidates
                    .push(evaluate_checked_candidate(library, history, telescope)?);
            }
            continue;
        }

        let clause_catalog = build_clause_catalog(enumeration_context, clause_kappa);
        if clause_catalog.is_empty() {
            continue;
        }
        let raw_catalog_clause_widths =
            raw_clause_catalog_widths(enumeration_context, clause_kappa);
        maybe_emit_claim_live_checkpoint(
            progress_observer,
            &mut last_checkpoint_elapsed_millis,
            StepLiveCheckpoint {
                step_index,
                phase: LiveStepCheckpointPhase::Discovery,
                elapsed_millis: elapsed_millis(step_start.elapsed()),
                clause_kappa: Some(clause_kappa),
                raw_catalog_clause_widths: raw_catalog_clause_widths.clone(),
                raw_catalog_telescope_count: Some(
                    raw_catalog_clause_widths
                        .iter()
                        .copied()
                        .fold(1usize, usize::saturating_mul),
                ),
                generated_raw_surface: discovery.raw_generated_surface,
                enumerated_candidates: discovery.enumerated_candidates,
                well_formed_candidates: discovery.well_formed_candidates,
                admissibility_rejections: discovery.admissibility_rejections,
                prefixes_created: discovery.prefixes_created,
                prefix_states_explored: discovery.prefix_states_explored,
                frontier_queue_len: 0,
                candidate_pool_len: discovery.candidates.len(),
                prefix_cache_groups: discovery.prefix_cache.len(),
                prefix_cache_candidates: discovery.prefix_cache.candidate_count(),
                legality_cache_entries: discovery.prefix_legality_cache.entry_counts().into(),
                exact_screen_prunes: discovery.partial_prefix_bound_prunes
                    + discovery.terminal_prefix_bar_prunes
                    + discovery.connectivity_prunes,
                claim_surface: claim_surface.clone(),
                remaining_one_telemetry: Some(discovery.remaining_one_telemetry),
                note: Some("claim_regular_clause_catalog".to_owned()),
            },
            true,
        );

        let mut frontier = Vec::new();
        for clause in clause_catalog.clauses_at(0) {
            if claim_replay::capture_should_stop() {
                break;
            }
            if demo_discovery_budget_exhausted(
                demo_step_budget,
                &mut discovery,
                step_start,
                demo_narrative.as_mut(),
            ) {
                break;
            }
            discovery.raw_generated_surface += 1;
            let prefix_telescope = Telescope::new(vec![clause.clone()]);
            let signature = PrefixSignature::new(step_index, library, &prefix_telescope);
            if !discovery.prefix_legality_cache.insert_root(
                signature.clone(),
                clause_kappa,
                library,
                &prefix_telescope,
                admissibility,
                enumeration_context.late_family_surface,
            ) {
                continue;
            }

            discovery.prefixes_created += 1;
            let work_item = create_online_prefix_work_item(
                clause_kappa,
                prefix_telescope,
                signature,
                library,
                admissibility,
                &clause_catalog,
                &mut discovery.prefix_legality_cache,
            );
            let (decision, performed_exact_check) = screen_prefix_for_frontier(
                step_index,
                library,
                admissibility,
                objective_bar,
                nu_history,
                &clause_catalog,
                &work_item,
                &mut discovery,
            );
            match decision {
                ExactPartialPrefixBoundDecision::CanClearBar => {
                    if performed_exact_check {
                        discovery.partial_prefix_bound_checks += 1;
                    }
                    frontier.push(work_item);
                }
                ExactPartialPrefixBoundDecision::CannotClearBar => {
                    if performed_exact_check {
                        discovery.partial_prefix_bound_checks += 1;
                    }
                    discovery.partial_prefix_bound_prunes += 1;
                }
                ExactPartialPrefixBoundDecision::Unknown => {
                    frontier.push(work_item);
                }
            }
        }

        while !frontier.is_empty() {
            if claim_replay::capture_should_stop() {
                break;
            }
            maybe_emit_claim_live_checkpoint(
                progress_observer,
                &mut last_checkpoint_elapsed_millis,
                StepLiveCheckpoint {
                    step_index,
                    phase: LiveStepCheckpointPhase::Discovery,
                    elapsed_millis: elapsed_millis(step_start.elapsed()),
                    clause_kappa: Some(clause_kappa),
                    raw_catalog_clause_widths: raw_catalog_clause_widths.clone(),
                    raw_catalog_telescope_count: Some(
                        raw_catalog_clause_widths
                            .iter()
                            .copied()
                            .fold(1usize, usize::saturating_mul),
                    ),
                    generated_raw_surface: discovery.raw_generated_surface,
                    enumerated_candidates: discovery.enumerated_candidates,
                    well_formed_candidates: discovery.well_formed_candidates,
                    admissibility_rejections: discovery.admissibility_rejections,
                    prefixes_created: discovery.prefixes_created,
                    prefix_states_explored: discovery.prefix_states_explored,
                    frontier_queue_len: frontier.len(),
                    candidate_pool_len: discovery.candidates.len(),
                    prefix_cache_groups: discovery.prefix_cache.len(),
                    prefix_cache_candidates: discovery.prefix_cache.candidate_count(),
                    legality_cache_entries: discovery.prefix_legality_cache.entry_counts().into(),
                    exact_screen_prunes: discovery.partial_prefix_bound_prunes
                        + discovery.terminal_prefix_bar_prunes
                        + discovery.terminal_rank_prunes
                        + discovery.connectivity_prunes,
                    claim_surface: claim_surface.clone(),
                    remaining_one_telemetry: Some(discovery.remaining_one_telemetry),
                    note: Some("claim_regular_frontier_progress".to_owned()),
                },
                false,
            );
            if demo_discovery_budget_exhausted(
                demo_step_budget,
                &mut discovery,
                step_start,
                demo_narrative.as_mut(),
            ) {
                break;
            }
            let frontier_pop_started = Instant::now();
            let Some(work_item) = pop_best_prefix(&mut frontier) else {
                break;
            };
            discovery.remaining_one_telemetry.frontier_sort_pop_millis = discovery
                .remaining_one_telemetry
                .frontier_sort_pop_millis
                .saturating_add(elapsed_millis(frontier_pop_started.elapsed()));
            let Some(work_item) = collapse_single_continuation_chain(
                step_index,
                library,
                admissibility,
                &clause_catalog,
                &mut discovery,
                work_item,
            ) else {
                continue;
            };
            let prefix_len = work_item.prefix_telescope.clauses.len();

            if prefix_len + 1 == usize::from(work_item.clause_kappa) {
                if claim_try_summary_prune_before_materialization(
                    step_index,
                    library,
                    admissibility,
                    objective_bar,
                    nu_history,
                    &work_item.signature,
                    &work_item.prefix_telescope,
                    work_item.clause_kappa,
                    work_item.next_clauses(&clause_catalog),
                    work_item.next_clause_connectivity_facts(&clause_catalog),
                    work_item.next_clause_nu_facts(&clause_catalog),
                    &mut discovery,
                ) {
                    continue;
                }
                let mut group = materialize_remaining_one_prefix_group(
                    step_index,
                    library,
                    admissibility,
                    objective_bar,
                    nu_history,
                    &work_item.signature,
                    &work_item.prefix_telescope,
                    work_item.next_clauses(&clause_catalog),
                    work_item.next_clause_connectivity_facts(&clause_catalog),
                    work_item.next_clause_nu_facts(&clause_catalog),
                    &mut discovery,
                )?;
                let bucket_key = demo_bucket_key(
                    admissibility.mode,
                    &work_item.signature,
                    &work_item.prefix_telescope,
                    work_item.clause_kappa,
                    group.generated_terminal_candidates,
                    group.admissible_terminal_candidates,
                );
                discovery.record_demo_bucket_surface(
                    bucket_key.clone(),
                    group.generated_terminal_candidates,
                    group.admissible_terminal_candidates,
                );
                let Some(retained_bound) = group.bound else {
                    continue;
                };
                discovery.record_demo_bucket_exact_screened(
                    bucket_key.clone(),
                    group.admissible_terminal_candidates,
                );
                if !retained_bound.can_clear_bar(objective_bar) {
                    discovery.terminal_prefix_bar_prunes += 1;
                    discovery.record_demo_bucket_pruned(
                        bucket_key.clone(),
                        group.admissible_terminal_candidates,
                    );
                    continue;
                }
                let cached_rank_prune_count =
                    if should_compact_terminal_prefix_group_candidates(admissibility.mode) {
                        terminal_prefix_rank_prune_count(
                            group.best_accept_rank.as_ref(),
                            None,
                            group.admissible_terminal_candidates,
                            discovery.terminal_rank_incumbent.as_ref(),
                        )
                    } else {
                        cached_terminal_prefix_rank_prune_count(
                            &work_item.signature,
                            discovery.terminal_rank_incumbent.as_ref(),
                            &mut discovery.prefix_legality_cache,
                        )
                    };
                if let Some(pruned_candidates) = cached_rank_prune_count {
                    discovery.terminal_rank_prunes += pruned_candidates;
                    discovery
                        .remaining_one_telemetry
                        .remaining_one_rank_prunes_post_materialize += 1;
                    discovery.record_demo_bucket_pruned(bucket_key.clone(), pruned_candidates);
                    continue;
                }
                if let (Some(group_best_rank), Some(incumbent_rank)) = (
                    best_prefix_group_accept_rank(&group.candidates),
                    discovery.terminal_rank_incumbent.as_ref(),
                ) {
                    if !better_rank(&group_best_rank, incumbent_rank) {
                        discovery.terminal_rank_prunes += group.candidates.len();
                        discovery
                            .remaining_one_telemetry
                            .remaining_one_rank_prunes_post_materialize += 1;
                        discovery
                            .record_demo_bucket_pruned(bucket_key.clone(), group.candidates.len());
                        continue;
                    }
                }

                discovery.prefix_states_explored += 1;
                cache_evaluated_terminal_prefix_group_candidates(
                    step_index,
                    objective_bar,
                    library,
                    history,
                    admissibility,
                    nu_history,
                    &mut group.candidates,
                    &mut discovery,
                )?;
                discovery.prefix_cache.record_group_with_bound(
                    step_index,
                    work_item.prefix_telescope.clone(),
                    group.candidates,
                    retained_bound,
                    library,
                    history,
                    retention_policy,
                )?;
                continue;
            }

            discovery.prefix_states_explored += 1;
            if work_item.remaining_clause_slots == 2 {
                let prepare_started = Instant::now();
                let terminal_prefixes = prepare_exact_two_step_terminal_surface(
                    step_index,
                    library,
                    admissibility,
                    &clause_catalog,
                    &work_item,
                    &mut discovery,
                );
                discovery
                    .remaining_one_telemetry
                    .prepare_exact_two_step_terminal_surface_millis = discovery
                    .remaining_one_telemetry
                    .prepare_exact_two_step_terminal_surface_millis
                    .saturating_add(elapsed_millis(prepare_started.elapsed()));
                if can_process_exact_two_step_terminal_surface_now(&frontier, &terminal_prefixes) {
                    process_prepared_exact_two_step_terminal_surface(
                        step_index,
                        library,
                        history,
                        admissibility,
                        objective_bar,
                        nu_history,
                        retention_policy,
                        &clause_catalog,
                        terminal_prefixes,
                        &mut discovery,
                    )?;
                    continue;
                }

                for terminal_prefix in terminal_prefixes {
                    let (decision, performed_exact_check) = screen_prefix_for_frontier(
                        step_index,
                        library,
                        admissibility,
                        objective_bar,
                        nu_history,
                        &clause_catalog,
                        &terminal_prefix,
                        &mut discovery,
                    );
                    match decision {
                        ExactPartialPrefixBoundDecision::CanClearBar => {
                            if performed_exact_check {
                                discovery.partial_prefix_bound_checks += 1;
                            }
                            frontier.push(terminal_prefix);
                        }
                        ExactPartialPrefixBoundDecision::CannotClearBar => {
                            if performed_exact_check {
                                discovery.partial_prefix_bound_checks += 1;
                            }
                            discovery.partial_prefix_bound_prunes += 1;
                        }
                        ExactPartialPrefixBoundDecision::Unknown => {
                            frontier.push(terminal_prefix);
                        }
                    }
                }
                continue;
            }

            for clause in work_item.next_clauses(&clause_catalog) {
                if claim_replay::capture_should_stop() {
                    break;
                }
                if demo_discovery_budget_exhausted(
                    demo_step_budget,
                    &mut discovery,
                    step_start,
                    demo_narrative.as_mut(),
                ) {
                    break;
                }
                discovery.raw_generated_surface += 1;
                let mut child_prefix = work_item.prefix_telescope.clone();
                child_prefix.clauses.push(clause.clone());
                let child_signature = PrefixSignature::new(step_index, library, &child_prefix);
                if !discovery.prefix_legality_cache.insert_child(
                    &work_item.signature,
                    child_signature.clone(),
                    library,
                    clause,
                    admissibility,
                ) {
                    continue;
                }
                discovery.prefixes_created += 1;
                let child_work_item = create_online_prefix_work_item(
                    work_item.clause_kappa,
                    child_prefix,
                    child_signature,
                    library,
                    admissibility,
                    &clause_catalog,
                    &mut discovery.prefix_legality_cache,
                );
                let (decision, performed_exact_check) = screen_prefix_for_frontier(
                    step_index,
                    library,
                    admissibility,
                    objective_bar,
                    nu_history,
                    &clause_catalog,
                    &child_work_item,
                    &mut discovery,
                );
                match decision {
                    ExactPartialPrefixBoundDecision::CanClearBar => {
                        if performed_exact_check {
                            discovery.partial_prefix_bound_checks += 1;
                        }
                        frontier.push(child_work_item);
                    }
                    ExactPartialPrefixBoundDecision::CannotClearBar => {
                        if performed_exact_check {
                            discovery.partial_prefix_bound_checks += 1;
                        }
                        discovery.partial_prefix_bound_prunes += 1;
                    }
                    ExactPartialPrefixBoundDecision::Unknown => {
                        frontier.push(child_work_item);
                    }
                }
            }
        }
    }

    Ok(discovery)
}

fn format_demo_clause_widths(widths: &[usize]) -> String {
    widths
        .iter()
        .map(|width| width.to_string())
        .collect::<Vec<_>>()
        .join("x")
}

fn step_one_demo_raw_catalog(
    library: &Library,
    enumeration_context: EnumerationContext,
) -> (Vec<usize>, Vec<Telescope>, Option<String>) {
    let root_clauses = enumerate_next_clauses(enumeration_context);
    let witness_context = EnumerationContext {
        scope_size: enumeration_context.scope_size.saturating_add(1),
        max_expr_nodes: enumeration_context.max_expr_nodes.saturating_add(1),
        ..enumeration_context
    };
    let witness_clauses = enumerate_raw_telescopes(witness_context, 1)
        .into_iter()
        .map(|telescope| {
            telescope
                .clauses
                .into_iter()
                .next()
                .expect("single-clause raw telescope should contain one clause")
        })
        .collect::<Vec<_>>();
    let mut raw_telescopes = Vec::new();
    let mut exact_clause_echoes = 0usize;

    for root_clause in root_clauses {
        let root_prefix = Telescope::new(vec![root_clause.clone()]);
        let valid_root_prefix = check_telescope(library, &root_prefix) == CheckResult::Ok;
        for witness_clause in &witness_clauses {
            if valid_root_prefix && *witness_clause == root_clause {
                exact_clause_echoes += 1;
                continue;
            }
            raw_telescopes.push(Telescope::new(vec![
                root_clause.clone(),
                witness_clause.clone(),
            ]));
        }
    }

    raw_telescopes.sort_by_key(|telescope| serde_json::to_string(telescope).expect("serialize"));
    (
        vec![18, 120],
        raw_telescopes,
        Some(format!(
            "excluded_exact_clause_echoes={exact_clause_echoes}"
        )),
    )
}

fn discover_demo_early_exhaustive_candidates(
    step_index: u32,
    library: &Library,
    history: &[DiscoveryRecord],
    admissibility: StrictAdmissibility,
    step_start: Instant,
    demo_step_budget: &mut Option<DemoStepBudget>,
    demo_narrative: &mut Option<DemoNarrativeRuntime>,
    progress_observer: &mut Option<&mut dyn AtomicSearchProgressObserver>,
) -> Result<RealisticShadowDiscovery> {
    let mut discovery = RealisticShadowDiscovery::default();
    let enumeration_context = EnumerationContext::from_admissibility(library, admissibility);
    let claim_surface = claim_live_checkpoint_enabled(admissibility.mode, step_index)
        .then(|| enumeration_context.surface_diagnostics());
    let mut last_checkpoint_elapsed_millis = 0u64;

    'clause_band: for clause_kappa in
        admissibility.min_clause_kappa..=admissibility.max_clause_kappa
    {
        let (raw_clause_widths, raw_telescopes, raw_catalog_note) =
            if step_index == 1 && clause_kappa == 2 {
                step_one_demo_raw_catalog(library, enumeration_context)
            } else {
                (
                    raw_clause_catalog_widths(enumeration_context, clause_kappa),
                    enumerate_raw_telescopes(enumeration_context, clause_kappa),
                    None,
                )
            };
        if let Some(observer) = demo_narrative.as_mut() {
            let raw_catalog_detail = match raw_catalog_note {
                Some(note) => format!(
                    "clause_kappa={} raw_clause_widths={} raw_telescopes={} {}",
                    clause_kappa,
                    format_demo_clause_widths(&raw_clause_widths),
                    raw_telescopes.len(),
                    note
                ),
                None => format!(
                    "clause_kappa={} raw_clause_widths={} raw_telescopes={}",
                    clause_kappa,
                    format_demo_clause_widths(&raw_clause_widths),
                    raw_telescopes.len()
                ),
            };
            observer.push_budget_pulse(
                StepPhase::Scout,
                "early exhaustive replay is using the restored raw clause catalog".to_owned(),
                Some(raw_catalog_detail),
                discovery_progress_snapshot(step_start, &discovery),
                true,
            );
        }
        maybe_emit_claim_live_checkpoint(
            progress_observer,
            &mut last_checkpoint_elapsed_millis,
            StepLiveCheckpoint {
                step_index,
                phase: LiveStepCheckpointPhase::Discovery,
                elapsed_millis: elapsed_millis(step_start.elapsed()),
                clause_kappa: Some(clause_kappa),
                raw_catalog_clause_widths: raw_clause_widths.clone(),
                raw_catalog_telescope_count: Some(raw_telescopes.len()),
                generated_raw_surface: discovery.raw_generated_surface,
                enumerated_candidates: discovery.enumerated_candidates,
                well_formed_candidates: discovery.well_formed_candidates,
                admissibility_rejections: discovery.admissibility_rejections,
                prefixes_created: discovery.prefixes_created,
                prefix_states_explored: discovery.prefix_states_explored,
                frontier_queue_len: 0,
                candidate_pool_len: discovery.candidates.len(),
                prefix_cache_groups: discovery.prefix_cache.len(),
                prefix_cache_candidates: discovery.prefix_cache.candidate_count(),
                legality_cache_entries: discovery.prefix_legality_cache.entry_counts().into(),
                exact_screen_prunes: discovery.connectivity_prunes,
                claim_surface: claim_surface.clone(),
                remaining_one_telemetry: Some(discovery.remaining_one_telemetry),
                note: Some("claim_early_exhaustive_catalog".to_owned()),
            },
            true,
        );
        for telescope in raw_telescopes {
            if demo_discovery_budget_exhausted(
                demo_step_budget,
                &mut discovery,
                step_start,
                demo_narrative.as_mut(),
            ) {
                break 'clause_band;
            }
            maybe_emit_claim_live_checkpoint(
                progress_observer,
                &mut last_checkpoint_elapsed_millis,
                StepLiveCheckpoint {
                    step_index,
                    phase: LiveStepCheckpointPhase::Discovery,
                    elapsed_millis: elapsed_millis(step_start.elapsed()),
                    clause_kappa: Some(clause_kappa),
                    raw_catalog_clause_widths: raw_clause_widths.clone(),
                    raw_catalog_telescope_count: None,
                    generated_raw_surface: discovery.raw_generated_surface,
                    enumerated_candidates: discovery.enumerated_candidates,
                    well_formed_candidates: discovery.well_formed_candidates,
                    admissibility_rejections: discovery.admissibility_rejections,
                    prefixes_created: discovery.prefixes_created,
                    prefix_states_explored: discovery.prefix_states_explored,
                    frontier_queue_len: 0,
                    candidate_pool_len: discovery.candidates.len(),
                    prefix_cache_groups: discovery.prefix_cache.len(),
                    prefix_cache_candidates: discovery.prefix_cache.candidate_count(),
                    legality_cache_entries: discovery.prefix_legality_cache.entry_counts().into(),
                    exact_screen_prunes: discovery.connectivity_prunes,
                    claim_surface: claim_surface.clone(),
                    remaining_one_telemetry: Some(discovery.remaining_one_telemetry),
                    note: Some("claim_early_exhaustive_progress".to_owned()),
                },
                false,
            );

            discovery.raw_generated_surface += 1;
            match check_telescope(library, &telescope) {
                CheckResult::Ok => {}
                CheckResult::Err(error) => {
                    discovery.malformed_rejections += 1;
                    *discovery
                        .malformed_rejection_reasons
                        .entry(error.kind_label().to_owned())
                        .or_insert(0) += 1;
                    continue;
                }
            }

            if !passes_connectivity(library, &telescope) {
                discovery.connectivity_prunes += 1;
                continue;
            }

            discovery.enumerated_candidates += 1;
            discovery.well_formed_candidates += 1;
            let admissibility_decision =
                assess_strict_admissibility(step_index, library, &telescope, admissibility);
            discovery
                .admissibility_diagnostics
                .record(&admissibility_decision);
            if !admissibility_decision.is_admitted() {
                discovery.admissibility_rejections += 1;
                continue;
            }

            discovery
                .candidates
                .push(evaluate_checked_candidate(library, history, telescope)?);
        }
    }

    Ok(discovery)
}

fn demo_discovery_budget_exhausted(
    demo_step_budget: &mut Option<DemoStepBudget>,
    discovery: &mut RealisticShadowDiscovery,
    step_start: Instant,
    demo_narrative: Option<&mut DemoNarrativeRuntime>,
) -> bool {
    let elapsed = elapsed_millis(step_start.elapsed());
    let progress = discovery_progress_snapshot(step_start, discovery);
    let scout_detail = discovery_scout_detail(discovery, elapsed);

    let mut demo_narrative = demo_narrative;
    if let Some(budget) = demo_step_budget.as_mut() {
        if let Some(rebalance) = maybe_retune_demo_budget_live(budget, &progress) {
            if let Some(observer) = demo_narrative.as_mut() {
                if observer.breadth_harvest_entered {
                    observer.record_breadth_harvest_budget_retune(
                        progress.clone(),
                        *budget,
                        rebalance,
                    );
                } else {
                    observer.record_scout_budget_rebalance(
                        progress.clone(),
                        scout_detail.clone(),
                        *budget,
                        rebalance,
                    );
                }
            }
        }
    }

    if let Some(observer) = demo_narrative.as_mut() {
        observer.maybe_enter_breadth_harvest(
            elapsed,
            progress.clone(),
            scout_detail,
            discovery_harvest_detail(observer.budget, discovery),
        );
    }
    let budget = demo_step_budget.as_ref().copied();
    if let Some(reason) = budget
        .and_then(|budget| budget.breadth_harvest_floor_hit(elapsed, &progress))
        .filter(|_| discovery.can_stop_for_budget())
    {
        discovery.stop_reason = Some(reason);
        return true;
    }

    let Some(discovery_deadline) = budget.and_then(|budget| budget.discovery_deadline(step_start))
    else {
        return false;
    };
    if !discovery.can_stop_for_budget() {
        return false;
    }
    if Instant::now() >= discovery_deadline {
        discovery.stop_reason = Some(DemoBreadthHarvestExitReason::ProofCloseReserveProtected);
        return true;
    }

    false
}

fn prepare_exact_two_step_terminal_surface(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    clause_catalog: &ClauseCatalog,
    work_item: &OnlinePrefixWorkItem,
    discovery: &mut RealisticShadowDiscovery,
) -> Vec<OnlinePrefixWorkItem> {
    debug_assert_eq!(work_item.remaining_clause_slots, 2);

    let mut terminal_prefixes = Vec::new();
    for clause in work_item.next_clauses(clause_catalog) {
        discovery.raw_generated_surface += 1;
        let mut child_prefix = work_item.prefix_telescope.clone();
        child_prefix.clauses.push(clause.clone());
        let child_signature = PrefixSignature::new(step_index, library, &child_prefix);
        if !discovery.prefix_legality_cache.insert_child(
            &work_item.signature,
            child_signature.clone(),
            library,
            clause,
            admissibility,
        ) {
            continue;
        }
        discovery.prefixes_created += 1;
        terminal_prefixes.push(create_online_prefix_work_item(
            work_item.clause_kappa,
            child_prefix,
            child_signature,
            library,
            admissibility,
            clause_catalog,
            &mut discovery.prefix_legality_cache,
        ));
    }
    terminal_prefixes.sort_by(|left, right| {
        prefix_frontier_work_key(left).cmp(&prefix_frontier_work_key(right))
    });
    terminal_prefixes
}

fn can_process_exact_two_step_terminal_surface_now(
    frontier: &[OnlinePrefixWorkItem],
    terminal_prefixes: &[OnlinePrefixWorkItem],
) -> bool {
    let Some(last_terminal_prefix) = terminal_prefixes.last() else {
        return true;
    };
    let Some(frontier_head) = frontier.first() else {
        return true;
    };

    // The frontier remains sorted after the current head was popped, so if the
    // slowest local terminal child still outranks the current frontier head we
    // can process the exact remaining-two suffix in place without changing the
    // deterministic pop order.
    prefix_frontier_work_key(last_terminal_prefix) < prefix_frontier_work_key(frontier_head)
}

fn screen_prefix_for_frontier(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    clause_catalog: &ClauseCatalog,
    work_item: &OnlinePrefixWorkItem,
    discovery: &mut RealisticShadowDiscovery,
) -> (ExactPartialPrefixBoundDecision, bool) {
    let track_remaining_one = work_item.remaining_clause_slots == 1;
    let remaining_one_summary_kernel_activation =
        track_remaining_one.then(|| remaining_one_summary_kernel_activation_context(discovery));
    if track_remaining_one {
        discovery
            .remaining_one_telemetry
            .remaining_one_prefixes_seen += 1;
    }

    let bound_started = Instant::now();
    if let Some(decision) = cached_terminal_prefix_queue_entry_bound_decision(
        objective_bar,
        work_item,
        &mut discovery.prefix_legality_cache,
        Some(&mut discovery.remaining_one_telemetry),
    ) {
        discovery
            .remaining_one_telemetry
            .exact_partial_prefix_bound_millis = discovery
            .remaining_one_telemetry
            .exact_partial_prefix_bound_millis
            .saturating_add(elapsed_millis(bound_started.elapsed()));
        if track_remaining_one
            && matches!(decision, ExactPartialPrefixBoundDecision::CannotClearBar)
        {
            discovery
                .remaining_one_telemetry
                .remaining_one_bar_prunes_pre_materialize += 1;
        }
        return (decision, false);
    }

    let mut exact_bound_budget = EXACT_PARTIAL_PREFIX_BOUND_BUDGET;
    let decision = exact_partial_prefix_bound_decision(
        step_index,
        library,
        admissibility,
        objective_bar,
        nu_history,
        clause_catalog,
        work_item,
        &mut discovery.prefix_legality_cache,
        &mut exact_bound_budget,
        discovery.terminal_rank_incumbent.as_ref(),
        Some(&mut discovery.remaining_one_telemetry),
        remaining_one_summary_kernel_activation,
    );
    discovery
        .remaining_one_telemetry
        .exact_partial_prefix_bound_millis = discovery
        .remaining_one_telemetry
        .exact_partial_prefix_bound_millis
        .saturating_add(elapsed_millis(bound_started.elapsed()));
    if track_remaining_one {
        match decision {
            ExactPartialPrefixBoundDecision::CannotClearBar => {
                discovery
                    .remaining_one_telemetry
                    .remaining_one_bar_prunes_pre_materialize += 1;
            }
            ExactPartialPrefixBoundDecision::Unknown => {
                discovery
                    .remaining_one_telemetry
                    .remaining_one_unknown_bound_budget_exhaustions += 1;
            }
            ExactPartialPrefixBoundDecision::CanClearBar => {}
        }
    }

    (decision, true)
}

fn process_prepared_exact_two_step_terminal_surface(
    step_index: u32,
    library: &Library,
    history: &[DiscoveryRecord],
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    retention_policy: RetentionPolicy,
    clause_catalog: &ClauseCatalog,
    terminal_prefixes: Vec<OnlinePrefixWorkItem>,
    discovery: &mut RealisticShadowDiscovery,
) -> Result<()> {
    for terminal_prefix in terminal_prefixes {
        debug_assert_eq!(terminal_prefix.remaining_clause_slots, 1);

        if claim_try_summary_prune_before_materialization(
            step_index,
            library,
            admissibility,
            objective_bar,
            nu_history,
            &terminal_prefix.signature,
            &terminal_prefix.prefix_telescope,
            terminal_prefix.clause_kappa,
            terminal_prefix.next_clauses(clause_catalog),
            terminal_prefix.next_clause_connectivity_facts(clause_catalog),
            terminal_prefix.next_clause_nu_facts(clause_catalog),
            discovery,
        ) {
            continue;
        }

        let mut group = materialize_remaining_one_prefix_group(
            step_index,
            library,
            admissibility,
            objective_bar,
            nu_history,
            &terminal_prefix.signature,
            &terminal_prefix.prefix_telescope,
            terminal_prefix.next_clauses(clause_catalog),
            terminal_prefix.next_clause_connectivity_facts(clause_catalog),
            terminal_prefix.next_clause_nu_facts(clause_catalog),
            discovery,
        )?;
        let bucket_key = demo_bucket_key(
            admissibility.mode,
            &terminal_prefix.signature,
            &terminal_prefix.prefix_telescope,
            terminal_prefix.clause_kappa,
            group.generated_terminal_candidates,
            group.admissible_terminal_candidates,
        );
        discovery.record_demo_bucket_surface(
            bucket_key.clone(),
            group.generated_terminal_candidates,
            group.admissible_terminal_candidates,
        );
        let Some(retained_bound) = group.bound else {
            continue;
        };
        discovery.record_demo_bucket_exact_screened(
            bucket_key.clone(),
            group.admissible_terminal_candidates,
        );
        if !retained_bound.can_clear_bar(objective_bar) {
            discovery.terminal_prefix_bar_prunes += 1;
            discovery.record_demo_bucket_pruned(
                bucket_key.clone(),
                group.admissible_terminal_candidates,
            );
            continue;
        }
        let cached_rank_prune_count =
            if should_compact_terminal_prefix_group_candidates(admissibility.mode) {
                terminal_prefix_rank_prune_count(
                    group.best_accept_rank.as_ref(),
                    None,
                    group.admissible_terminal_candidates,
                    discovery.terminal_rank_incumbent.as_ref(),
                )
            } else {
                cached_terminal_prefix_rank_prune_count(
                    &terminal_prefix.signature,
                    discovery.terminal_rank_incumbent.as_ref(),
                    &mut discovery.prefix_legality_cache,
                )
            };
        if let Some(pruned_candidates) = cached_rank_prune_count {
            discovery.terminal_rank_prunes += pruned_candidates;
            discovery
                .remaining_one_telemetry
                .remaining_one_rank_prunes_post_materialize += 1;
            discovery.record_demo_bucket_pruned(bucket_key.clone(), pruned_candidates);
            continue;
        }
        if let (Some(group_best_rank), Some(incumbent_rank)) = (
            best_prefix_group_accept_rank(&group.candidates),
            discovery.terminal_rank_incumbent.as_ref(),
        ) {
            if !better_rank(&group_best_rank, incumbent_rank) {
                discovery.terminal_rank_prunes += group.candidates.len();
                discovery
                    .remaining_one_telemetry
                    .remaining_one_rank_prunes_post_materialize += 1;
                discovery.record_demo_bucket_pruned(bucket_key.clone(), group.candidates.len());
                continue;
            }
        }

        cache_evaluated_terminal_prefix_group_candidates(
            step_index,
            objective_bar,
            library,
            history,
            admissibility,
            nu_history,
            &mut group.candidates,
            discovery,
        )?;
        discovery.prefix_cache.record_group_with_bound(
            step_index,
            terminal_prefix.prefix_telescope.clone(),
            group.candidates,
            retained_bound,
            library,
            history,
            retention_policy,
        )?;
    }

    Ok(())
}

fn create_online_prefix_work_item(
    clause_kappa: u16,
    prefix_telescope: Telescope,
    signature: PrefixSignature,
    library: &Library,
    admissibility: StrictAdmissibility,
    clause_catalog: &ClauseCatalog,
    prefix_legality_cache: &mut PrefixLegalityCache,
) -> OnlinePrefixWorkItem {
    fn clone_filtered_terminal_clause_data(
        catalog_clauses: &[pen_core::clause::ClauseRec],
        catalog_connectivity_facts: &[TerminalClauseConnectivityFacts],
        catalog_nu_facts: &[TerminalClauseNuFacts],
        filtered_clauses: Vec<&pen_core::clause::ClauseRec>,
    ) -> (
        Vec<pen_core::clause::ClauseRec>,
        Vec<TerminalClauseConnectivityFacts>,
        Vec<TerminalClauseNuFacts>,
    ) {
        let index_by_ptr = catalog_clauses
            .iter()
            .enumerate()
            .map(|(index, clause)| (clause as *const pen_core::clause::ClauseRec as usize, index))
            .collect::<BTreeMap<_, _>>();
        let mut cloned_clauses = Vec::with_capacity(filtered_clauses.len());
        let mut cloned_connectivity_facts = Vec::with_capacity(filtered_clauses.len());
        let mut cloned_nu_facts = Vec::with_capacity(filtered_clauses.len());
        for clause in filtered_clauses {
            let clause_index = *index_by_ptr
                .get(&(clause as *const pen_core::clause::ClauseRec as usize))
                .expect("filtered clause should originate from the clause catalog");
            cloned_clauses.push(clause.clone());
            cloned_connectivity_facts.push(
                catalog_connectivity_facts
                    .get(clause_index)
                    .expect("connectivity facts should align with clause catalog")
                    .clone(),
            );
            cloned_nu_facts.push(
                catalog_nu_facts
                    .get(clause_index)
                    .expect("nu facts should align with clause catalog")
                    .clone(),
            );
        }
        (cloned_clauses, cloned_connectivity_facts, cloned_nu_facts)
    }

    let prefix_len = prefix_telescope.clauses.len();
    let remaining_clause_slots = usize::from(clause_kappa).saturating_sub(prefix_len);
    let (
        filtered_next_clauses,
        filtered_next_clause_connectivity_facts,
        filtered_next_clause_nu_facts,
        next_clause_count,
    ) = if remaining_clause_slots == 0 {
        (Some(Vec::new()), Some(Vec::new()), Some(Vec::new()), 0)
    } else {
        let catalog_clauses = clause_catalog.clauses_at(prefix_len);
        let catalog_connectivity_facts = clause_catalog.terminal_connectivity_facts_at(prefix_len);
        let catalog_nu_facts = clause_catalog.terminal_nu_facts_at(prefix_len);
        match prefix_legality_cache.filter_active_window_clauses(
            &signature,
            prefix_len,
            library,
            admissibility,
            catalog_clauses,
        ) {
            Some(filtered_clauses) if filtered_clauses.len() < catalog_clauses.len() => {
                let filtered_len = filtered_clauses.len();
                let (filtered_clauses, filtered_connectivity_facts, filtered_nu_facts) =
                    clone_filtered_terminal_clause_data(
                        catalog_clauses,
                        catalog_connectivity_facts,
                        catalog_nu_facts,
                        filtered_clauses,
                    );
                (
                    Some(filtered_clauses),
                    Some(filtered_connectivity_facts),
                    Some(filtered_nu_facts),
                    filtered_len,
                )
            }
            Some(_) | None => (None, None, None, catalog_clauses.len()),
        }
    };

    OnlinePrefixWorkItem {
        clause_kappa,
        remaining_clause_slots,
        remaining_family_options: prefix_legality_cache
            .family_option_count(&signature)
            .unwrap_or(u8::MAX),
        bit_cost: prefix_telescope.bit_cost(),
        clause_count: prefix_telescope.kappa(),
        filtered_next_clauses,
        filtered_next_clause_connectivity_facts,
        filtered_next_clause_nu_facts,
        next_clause_count,
        order_key: signature.order_key(),
        prefix_telescope,
        signature,
    }
}

fn collapse_single_continuation_chain(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    clause_catalog: &ClauseCatalog,
    discovery: &mut RealisticShadowDiscovery,
    work_item: OnlinePrefixWorkItem,
) -> Option<OnlinePrefixWorkItem> {
    let (work_item, collapsed_signatures) = collapse_single_continuation_chain_inner(
        step_index,
        library,
        admissibility,
        clause_catalog,
        &mut discovery.prefix_legality_cache,
        work_item,
    )?;
    discovery.prefixes_created += collapsed_signatures.len();
    discovery.raw_generated_surface += collapsed_signatures.len();
    Some(work_item)
}

fn collapse_single_continuation_chain_inner(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    clause_catalog: &ClauseCatalog,
    prefix_legality_cache: &mut PrefixLegalityCache,
    mut work_item: OnlinePrefixWorkItem,
) -> Option<(OnlinePrefixWorkItem, Vec<PrefixSignature>)> {
    let mut collapsed_signatures = Vec::new();

    loop {
        if work_item.remaining_clause_slots <= 1 {
            return Some((work_item, collapsed_signatures));
        }

        let [clause] = work_item.next_clauses(clause_catalog) else {
            return Some((work_item, collapsed_signatures));
        };

        let mut child_prefix = work_item.prefix_telescope.clone();
        child_prefix.clauses.push(clause.clone());
        let child_signature = PrefixSignature::new(step_index, library, &child_prefix);
        if !prefix_legality_cache.insert_child(
            &work_item.signature,
            child_signature.clone(),
            library,
            clause,
            admissibility,
        ) {
            return None;
        }

        collapsed_signatures.push(work_item.signature.clone());
        work_item = create_online_prefix_work_item(
            work_item.clause_kappa,
            child_prefix,
            child_signature,
            library,
            admissibility,
            clause_catalog,
            prefix_legality_cache,
        );
    }
}

fn store_partial_prefix_bound_decision_for_signatures(
    prefix_legality_cache: &mut PrefixLegalityCache,
    signatures: Vec<PrefixSignature>,
    decision: PartialPrefixBoundDecision,
) {
    for signature in signatures {
        prefix_legality_cache.store_partial_prefix_bound_decision(signature, decision);
    }
}

fn spend_exact_partial_prefix_budget(budget: &mut usize, amount: usize) -> bool {
    if *budget < amount {
        return false;
    }
    *budget -= amount;
    true
}

fn exact_partial_prefix_bound_decision(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    clause_catalog: &ClauseCatalog,
    work_item: &OnlinePrefixWorkItem,
    prefix_legality_cache: &mut PrefixLegalityCache,
    budget: &mut usize,
    incumbent_rank: Option<&AcceptRank>,
    mut remaining_one_telemetry: Option<&mut RemainingOneTelemetry>,
    remaining_one_summary_kernel_activation: Option<RemainingOneSummaryKernelActivationContext>,
) -> ExactPartialPrefixBoundDecision {
    if work_item.remaining_clause_slots == 0 {
        return ExactPartialPrefixBoundDecision::Unknown;
    }

    if let Some(decision) =
        prefix_legality_cache.partial_prefix_bound_decision(&work_item.signature)
    {
        return match decision {
            PartialPrefixBoundDecision::CanClearBar => ExactPartialPrefixBoundDecision::CanClearBar,
            PartialPrefixBoundDecision::CannotClearBar => {
                ExactPartialPrefixBoundDecision::CannotClearBar
            }
        };
    }

    if work_item.remaining_clause_slots == 1 {
        let decision = exact_terminal_prefix_bound_decision(
            step_index,
            library,
            admissibility,
            objective_bar,
            nu_history,
            &work_item.signature,
            &work_item.prefix_telescope,
            work_item.next_clauses(clause_catalog),
            work_item.next_clause_connectivity_facts(clause_catalog),
            work_item.next_clause_nu_facts(clause_catalog),
            prefix_legality_cache,
            budget,
            incumbent_rank,
            remaining_one_telemetry,
            remaining_one_summary_kernel_activation,
        );
        if let Some(cacheable) = decision.cacheable_partial_decision() {
            prefix_legality_cache
                .store_partial_prefix_bound_decision(work_item.signature.clone(), cacheable);
        }
        return decision;
    }

    for clause in work_item.next_clauses(clause_catalog) {
        if !spend_exact_partial_prefix_budget(budget, 1) {
            return ExactPartialPrefixBoundDecision::Unknown;
        }

        let mut child_prefix = work_item.prefix_telescope.clone();
        child_prefix.clauses.push(clause.clone());
        let child_signature = PrefixSignature::new(step_index, library, &child_prefix);
        if !prefix_legality_cache.insert_child(
            &work_item.signature,
            child_signature.clone(),
            library,
            clause,
            admissibility,
        ) {
            continue;
        }

        let child_work_item = create_online_prefix_work_item(
            work_item.clause_kappa,
            child_prefix,
            child_signature,
            library,
            admissibility,
            clause_catalog,
            prefix_legality_cache,
        );
        let child_signature = child_work_item.signature.clone();
        let Some((child_work_item, collapsed_signatures)) =
            collapse_single_continuation_chain_inner(
                step_index,
                library,
                admissibility,
                clause_catalog,
                prefix_legality_cache,
                child_work_item,
            )
        else {
            continue;
        };
        if !spend_exact_partial_prefix_budget(budget, collapsed_signatures.len()) {
            return ExactPartialPrefixBoundDecision::Unknown;
        }

        let propagated_decision = exact_partial_prefix_bound_decision(
            step_index,
            library,
            admissibility,
            objective_bar,
            nu_history,
            clause_catalog,
            &child_work_item,
            prefix_legality_cache,
            budget,
            incumbent_rank,
            remaining_one_telemetry.as_deref_mut(),
            remaining_one_summary_kernel_activation,
        );
        if let Some(cacheable) = propagated_decision.cacheable_partial_decision() {
            let mut signatures = collapsed_signatures;
            signatures.push(child_signature);
            store_partial_prefix_bound_decision_for_signatures(
                prefix_legality_cache,
                signatures,
                cacheable,
            );
        }

        match propagated_decision {
            ExactPartialPrefixBoundDecision::CanClearBar => {
                prefix_legality_cache.store_partial_prefix_bound_decision(
                    work_item.signature.clone(),
                    PartialPrefixBoundDecision::CanClearBar,
                );
                return ExactPartialPrefixBoundDecision::CanClearBar;
            }
            ExactPartialPrefixBoundDecision::CannotClearBar => {}
            ExactPartialPrefixBoundDecision::Unknown => {
                return ExactPartialPrefixBoundDecision::Unknown;
            }
        }
    }

    prefix_legality_cache.store_partial_prefix_bound_decision(
        work_item.signature.clone(),
        PartialPrefixBoundDecision::CannotClearBar,
    );
    ExactPartialPrefixBoundDecision::CannotClearBar
}

fn cached_terminal_prefix_queue_entry_bound_decision(
    objective_bar: Rational,
    work_item: &OnlinePrefixWorkItem,
    prefix_legality_cache: &mut PrefixLegalityCache,
    remaining_one_telemetry: Option<&mut RemainingOneTelemetry>,
) -> Option<ExactPartialPrefixBoundDecision> {
    if work_item.remaining_clause_slots != 1 {
        return None;
    }

    let bound = prefix_legality_cache.terminal_prefix_bound_summary(&work_item.signature)?;
    if let Some(telemetry) = remaining_one_telemetry {
        telemetry.remaining_one_cached_bound_hits += 1;
    }
    let decision = exact_terminal_prefix_bound_decision_from_bound(objective_bar, bound);
    if let Some(cacheable) = decision.cacheable_partial_decision() {
        prefix_legality_cache
            .store_partial_prefix_bound_decision(work_item.signature.clone(), cacheable);
    }
    Some(decision)
}

fn remaining_one_structural_nu_caps(
    admissibility: StrictAdmissibility,
) -> SingleClauseStructuralNuCaps {
    SingleClauseStructuralNuCaps {
        max_expr_nodes: admissibility.max_expr_nodes,
        max_path_dimension: admissibility.max_path_dimension,
        include_trunc: admissibility.include_trunc,
        include_modal: admissibility.include_modal,
        include_temporal: admissibility.include_temporal,
        historical_anchor_ref: admissibility.historical_anchor_ref,
    }
}

fn claim_remaining_one_algebraic_nu_ceiling_cannot_clear_bar(
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    prefix_telescope: &Telescope,
) -> bool {
    if !should_compact_terminal_prefix_group_candidates(admissibility.mode) {
        return false;
    }

    let clause_kappa_used = u32::try_from(prefix_telescope.clauses.len().saturating_add(1))
        .expect("kappa exceeded u32");
    let exact_nu_upper_bound = structural_nu_single_clause_upper_bound(
        prefix_telescope,
        library,
        nu_history,
        remaining_one_structural_nu_caps(admissibility),
    );

    Rational::new(
        i64::from(exact_nu_upper_bound),
        i64::from(clause_kappa_used),
    ) < objective_bar
}

fn exact_terminal_prefix_bound_decision(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    prefix_signature: &PrefixSignature,
    prefix_telescope: &Telescope,
    filtered_last_clause_options: &[pen_core::clause::ClauseRec],
    filtered_last_clause_connectivity_facts: Option<&[TerminalClauseConnectivityFacts]>,
    filtered_last_clause_nu_facts: &[TerminalClauseNuFacts],
    prefix_legality_cache: &mut PrefixLegalityCache,
    budget: &mut usize,
    incumbent_rank: Option<&AcceptRank>,
    mut remaining_one_telemetry: Option<&mut RemainingOneTelemetry>,
    remaining_one_summary_kernel_activation: Option<RemainingOneSummaryKernelActivationContext>,
) -> ExactPartialPrefixBoundDecision {
    if let Some(bound) = prefix_legality_cache.terminal_prefix_bound_summary(prefix_signature) {
        if let Some(telemetry) = remaining_one_telemetry.as_deref_mut() {
            telemetry.remaining_one_cached_bound_hits += 1;
        }
        return exact_terminal_prefix_bound_decision_from_bound(objective_bar, bound);
    }

    if claim_remaining_one_algebraic_nu_ceiling_cannot_clear_bar(
        library,
        admissibility,
        objective_bar,
        nu_history,
        prefix_telescope,
    ) {
        if let Some(telemetry) = remaining_one_telemetry.as_deref_mut() {
            telemetry.remaining_one_algebraic_prunes += 1;
        }
        return ExactPartialPrefixBoundDecision::CannotClearBar;
    }

    let summary_started = Instant::now();
    let terminal_clauses = terminal_prefix_clause_candidates(
        step_index,
        library,
        admissibility,
        prefix_signature,
        filtered_last_clause_options,
        filtered_last_clause_connectivity_facts,
        filtered_last_clause_nu_facts,
        prefix_legality_cache,
        remaining_one_telemetry.as_deref_mut(),
    );
    if spend_exact_partial_prefix_budget(budget, terminal_clauses.len()) {
        let summary = compute_terminal_prefix_completion_summary_from_candidates(
            step_index,
            library,
            admissibility,
            objective_bar,
            nu_history,
            prefix_signature,
            prefix_telescope,
            if should_compact_terminal_prefix_group_candidates(admissibility.mode) {
                TerminalPrefixSummaryPayload::Compact
            } else {
                TerminalPrefixSummaryPayload::Full
            },
            terminal_clauses,
            incumbent_rank,
            prefix_legality_cache,
            remaining_one_telemetry.as_deref_mut(),
            remaining_one_summary_kernel_activation,
        );
        if let Some(telemetry) = remaining_one_telemetry.as_deref_mut() {
            telemetry.absorb_terminal_summary_build_duration(summary_started.elapsed());
        }
        prefix_legality_cache
            .store_terminal_prefix_completion_summary(prefix_signature.clone(), summary.clone());
        return exact_terminal_prefix_bound_decision_from_bound(objective_bar, summary.bound);
    }

    let prefix_len = prefix_telescope.clauses.len();
    let clause_kappa_used =
        u32::try_from(prefix_len.saturating_add(1)).expect("kappa exceeded u32");
    let prefix_bit_cost = prefix_telescope.bit_cost();
    let mut scratch_telescope = terminal_prefix_scratch_telescope(prefix_telescope);
    let single_clause_nu_context = (!prefix_telescope.clauses.is_empty()).then(|| {
        SingleClauseStructuralNuContext::from_prefix(prefix_telescope, library, nu_history)
    });
    match terminal_clauses {
        TerminalPrefixClauseCandidates::General(terminal_clauses) => {
            for terminal_clause in terminal_clauses {
                if !spend_exact_partial_prefix_budget(budget, 1) {
                    return ExactPartialPrefixBoundDecision::Unknown;
                }

                let Some(connectivity_decision) = prefix_legality_cache
                    .terminal_connectivity_with_facts(
                        prefix_signature,
                        library,
                        terminal_clause.clause,
                        terminal_clause.connectivity_facts,
                    )
                else {
                    continue;
                };
                if matches!(
                    connectivity_decision,
                    TerminalConnectivityDecision::PruneDisconnected
                ) {
                    continue;
                }
                if let Some(kernel_mode) = terminal_clause
                    .cached_admissibility_decision
                    .as_ref()
                    .map(RemainingOneNoMissKernelMode::General)
                {
                    if let Some(kernel_outcome) = run_remaining_one_no_miss_plateau_kernel(
                        objective_bar,
                        u16::try_from(clause_kappa_used).expect("kappa exceeded u16"),
                        prefix_bit_cost,
                        &terminal_clause,
                        connectivity_decision,
                        kernel_mode,
                        single_clause_nu_context.as_ref(),
                    ) {
                        match kernel_outcome {
                            RemainingOneNoMissPlateauKernelOutcome::AdmissibilityRejected {
                                ..
                            } => {
                                continue;
                            }
                            RemainingOneNoMissPlateauKernelOutcome::Admitted { score, .. } => {
                                if score.primary_rank.is_some() {
                                    return ExactPartialPrefixBoundDecision::CanClearBar;
                                }
                                continue;
                            }
                        }
                    }
                }

                let telescope = load_terminal_clause_into_scratch(
                    &mut scratch_telescope,
                    prefix_len,
                    terminal_clause.clause,
                );
                if matches!(
                    connectivity_decision,
                    TerminalConnectivityDecision::NeedsFallback
                ) {
                    if !passes_connectivity(library, telescope) {
                        continue;
                    }
                }

                let admissibility_decision =
                    if let Some(decision) = terminal_clause.cached_admissibility_decision {
                        decision
                    } else {
                        assess_strict_admissibility(step_index, library, telescope, admissibility)
                    };
                if !admissibility_decision.is_admitted() {
                    continue;
                }

                let exact_nu = single_clause_structural_nu_total(
                    telescope,
                    terminal_clause.nu_facts,
                    single_clause_nu_context.as_ref(),
                    library,
                    nu_history,
                );
                let rho = Rational::new(i64::from(exact_nu), i64::from(clause_kappa_used));
                if rho >= objective_bar {
                    return ExactPartialPrefixBoundDecision::CanClearBar;
                }
            }
        }
        TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand(terminal_clauses) => {
            for terminal_clause in terminal_clauses {
                if !spend_exact_partial_prefix_budget(budget, 1) {
                    return ExactPartialPrefixBoundDecision::Unknown;
                }

                let Some(connectivity_decision) = prefix_legality_cache
                    .terminal_connectivity_with_facts(
                        prefix_signature,
                        library,
                        terminal_clause.clause,
                        terminal_clause.connectivity_facts,
                    )
                else {
                    continue;
                };
                if matches!(
                    connectivity_decision,
                    TerminalConnectivityDecision::PruneDisconnected
                ) {
                    continue;
                }
                if let Some(kernel_outcome) = run_remaining_one_no_miss_plateau_kernel(
                    objective_bar,
                    u16::try_from(clause_kappa_used).expect("kappa exceeded u16"),
                    prefix_bit_cost,
                    &terminal_clause,
                    connectivity_decision,
                    RemainingOneNoMissKernelMode::ClaimAdmittedOpenBand,
                    single_clause_nu_context.as_ref(),
                ) {
                    match kernel_outcome {
                        RemainingOneNoMissPlateauKernelOutcome::AdmissibilityRejected {
                            ..
                        } => {
                            continue;
                        }
                        RemainingOneNoMissPlateauKernelOutcome::Admitted { score, .. } => {
                            if score.primary_rank.is_some() {
                                return ExactPartialPrefixBoundDecision::CanClearBar;
                            }
                            continue;
                        }
                    }
                }

                let telescope = load_terminal_clause_into_scratch(
                    &mut scratch_telescope,
                    prefix_len,
                    terminal_clause.clause,
                );
                if matches!(
                    connectivity_decision,
                    TerminalConnectivityDecision::NeedsFallback
                ) {
                    if !passes_connectivity(library, telescope) {
                        continue;
                    }
                }

                let exact_nu = single_clause_structural_nu_total(
                    telescope,
                    terminal_clause.nu_facts,
                    single_clause_nu_context.as_ref(),
                    library,
                    nu_history,
                );
                let rho = Rational::new(i64::from(exact_nu), i64::from(clause_kappa_used));
                if rho >= objective_bar {
                    return ExactPartialPrefixBoundDecision::CanClearBar;
                }
            }
        }
    }

    ExactPartialPrefixBoundDecision::CannotClearBar
}

fn exact_terminal_prefix_bound_decision_from_bound(
    objective_bar: Rational,
    bound: Option<PrefixBound>,
) -> ExactPartialPrefixBoundDecision {
    let Some(bound) = bound else {
        return ExactPartialPrefixBoundDecision::CannotClearBar;
    };
    if bound.can_clear_bar(objective_bar) {
        ExactPartialPrefixBoundDecision::CanClearBar
    } else {
        ExactPartialPrefixBoundDecision::CannotClearBar
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TerminalPrefixSummaryPayload {
    Compact,
    Full,
}

#[derive(Clone, Debug, Default)]
struct CompactTerminalPrefixSurvivorSketchBuilder {
    survivors: Vec<TerminalPrefixSurvivorSketchEntry>,
}

impl CompactTerminalPrefixSurvivorSketchBuilder {
    fn record_candidate(
        &mut self,
        primary_rank: Option<&TerminalPrefixPrimaryRank>,
        clause_index: usize,
        exact_nu: u16,
        bit_kappa_used: u16,
        competition_allowed: bool,
        incumbent_rank: Option<&AcceptRank>,
    ) {
        if !competition_allowed {
            return;
        }
        let Some(primary_rank) = primary_rank else {
            return;
        };
        if incumbent_rank.is_some_and(|incumbent| {
            !terminal_prefix_primary_rank_could_still_beat_incumbent(&primary_rank, incumbent)
        }) {
            return;
        }

        self.survivors.push(TerminalPrefixSurvivorSketchEntry {
            clause_index,
            exact_nu,
            bit_kappa_used,
        });
    }

    fn finish(self) -> Option<TerminalPrefixSurvivorSketch> {
        (!self.survivors.is_empty()).then_some(TerminalPrefixSurvivorSketch {
            survivors: self.survivors,
        })
    }
}

#[derive(Clone, Debug)]
struct TerminalPrefixCandidate<'a> {
    clause: &'a pen_core::clause::ClauseRec,
    cached_admissibility_decision: Option<AdmissibilityDecision>,
    connectivity_facts: Option<&'a TerminalClauseConnectivityFacts>,
    nu_facts: &'a TerminalClauseNuFacts,
}

#[derive(Clone, Debug)]
enum TerminalPrefixClauseCandidates<'a> {
    General(Vec<TerminalPrefixCandidate<'a>>),
    ClaimAdmittedOpenBand(Vec<TerminalPrefixCandidate<'a>>),
}

impl<'a> TerminalPrefixClauseCandidates<'a> {
    fn len(&self) -> usize {
        match self {
            Self::General(clauses) => clauses.len(),
            Self::ClaimAdmittedOpenBand(clauses) => clauses.len(),
        }
    }
}

fn terminal_prefix_clause_candidates<'a>(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    prefix_signature: &PrefixSignature,
    filtered_last_clause_options: &'a [pen_core::clause::ClauseRec],
    filtered_last_clause_connectivity_facts: Option<&'a [TerminalClauseConnectivityFacts]>,
    filtered_last_clause_nu_facts: &'a [TerminalClauseNuFacts],
    prefix_legality_cache: &mut PrefixLegalityCache,
    remaining_one_telemetry: Option<&mut RemainingOneTelemetry>,
) -> TerminalPrefixClauseCandidates<'a> {
    fn find_connectivity_fact<'a>(
        clause: &pen_core::clause::ClauseRec,
        clauses: &'a [pen_core::clause::ClauseRec],
        facts: Option<&'a [TerminalClauseConnectivityFacts]>,
    ) -> Option<&'a TerminalClauseConnectivityFacts> {
        let facts = facts?;
        if facts.len() != clauses.len() {
            return None;
        }
        clauses
            .iter()
            .zip(facts.iter())
            .find_map(|(candidate, fact)| std::ptr::eq(candidate, clause).then_some(fact))
    }

    fn find_nu_fact<'a>(
        clause: &pen_core::clause::ClauseRec,
        clauses: &'a [pen_core::clause::ClauseRec],
        facts: &'a [TerminalClauseNuFacts],
    ) -> &'a TerminalClauseNuFacts {
        assert_eq!(
            facts.len(),
            clauses.len(),
            "terminal nu facts should align with terminal clauses"
        );
        clauses
            .iter()
            .zip(facts.iter())
            .find_map(|(candidate, fact)| std::ptr::eq(candidate, clause).then_some(fact))
            .expect("terminal nu facts should align with filtered terminal clause")
    }

    let clause_filter_started = Instant::now();
    let terminal_clauses = if let Some(clauses) = prefix_legality_cache
        .filter_claim_open_band_terminal_clauses(
            step_index,
            prefix_signature,
            admissibility,
            filtered_last_clause_options,
        ) {
        TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand(
            clauses
                .into_iter()
                .map(|clause| TerminalPrefixCandidate {
                    clause,
                    cached_admissibility_decision: None,
                    connectivity_facts: find_connectivity_fact(
                        clause,
                        filtered_last_clause_options,
                        filtered_last_clause_connectivity_facts,
                    ),
                    nu_facts: find_nu_fact(
                        clause,
                        filtered_last_clause_options,
                        filtered_last_clause_nu_facts,
                    ),
                })
                .collect(),
        )
    } else {
        prefix_legality_cache
            .filter_terminal_clauses(
                step_index,
                prefix_signature,
                library,
                admissibility,
                filtered_last_clause_options,
            )
            .map(|clauses| {
                TerminalPrefixClauseCandidates::General(
                    clauses
                        .into_iter()
                        .map(|clause| TerminalPrefixCandidate {
                            connectivity_facts: find_connectivity_fact(
                                clause.clause,
                                filtered_last_clause_options,
                                filtered_last_clause_connectivity_facts,
                            ),
                            nu_facts: find_nu_fact(
                                clause.clause,
                                filtered_last_clause_options,
                                filtered_last_clause_nu_facts,
                            ),
                            clause: clause.clause,
                            cached_admissibility_decision: Some(clause.admissibility_decision),
                        })
                        .collect(),
                )
            })
            .unwrap_or_else(|| {
                TerminalPrefixClauseCandidates::General(
                    filtered_last_clause_options
                        .iter()
                        .enumerate()
                        .map(|(index, clause)| TerminalPrefixCandidate {
                            clause,
                            cached_admissibility_decision: None,
                            connectivity_facts: filtered_last_clause_connectivity_facts
                                .and_then(|facts| facts.get(index)),
                            nu_facts: filtered_last_clause_nu_facts
                                .get(index)
                                .expect("terminal nu facts should align with terminal clauses"),
                        })
                        .collect(),
                )
            })
    };
    if let Some(telemetry) = remaining_one_telemetry {
        telemetry.absorb_terminal_prefix_clause_filter_duration(clause_filter_started.elapsed());
    }
    terminal_clauses
}

fn terminal_prefix_scratch_telescope(prefix_telescope: &Telescope) -> Telescope {
    let mut telescope = prefix_telescope.clone();
    telescope.clauses.reserve(1);
    telescope
}

fn load_terminal_clause_into_scratch<'a>(
    scratch_telescope: &'a mut Telescope,
    prefix_len: usize,
    clause: &pen_core::clause::ClauseRec,
) -> &'a Telescope {
    if scratch_telescope.clauses.len() == prefix_len {
        scratch_telescope.clauses.push(clause.clone());
    } else {
        // Reuse the hot scratch slot's existing heap shape once it has been
        // initialized so remaining-one summary/materialization stops paying a
        // fresh deep clause clone on every terminal candidate.
        scratch_telescope.clauses[prefix_len].clone_from(clause);
    }
    scratch_telescope
}

fn single_clause_structural_nu_total(
    telescope: &Telescope,
    clause_nu_facts: &TerminalClauseNuFacts,
    single_clause_nu_context: Option<&SingleClauseStructuralNuContext>,
    library: &Library,
    nu_history: &[(u32, u32)],
) -> u32 {
    single_clause_nu_context
        .map(|context| {
            context
                .structural_nu_with_clause_facts(clause_nu_facts)
                .total
        })
        .unwrap_or_else(|| structural_nu(telescope, library, nu_history).total)
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PrefixLocalTerminalClauseScore {
    exact_nu: u16,
    bit_kappa_used: u16,
    primary_rank: Option<TerminalPrefixPrimaryRank>,
}

#[derive(Clone, Copy, Debug)]
enum RemainingOneNoMissKernelMode<'a> {
    General(&'a AdmissibilityDecision),
    ClaimAdmittedOpenBand,
}

#[derive(Clone, Copy, Debug)]
enum RemainingOneNoMissKernelAdmittedDecision<'a> {
    Cached(&'a AdmissibilityDecision),
    ClaimOpenBand,
}

impl RemainingOneNoMissKernelAdmittedDecision<'_> {
    fn clone_for_evaluation(self) -> AdmissibilityDecision {
        match self {
            Self::Cached(decision) => decision.clone(),
            Self::ClaimOpenBand => claim_open_band_admissibility_decision(),
        }
    }

    fn competition_allowed(
        self,
        prefix_signature: &PrefixSignature,
        admissibility: StrictAdmissibility,
        prefix_legality_cache: &PrefixLegalityCache,
    ) -> bool {
        match self {
            Self::Cached(decision) => terminal_completion_can_compete_for_acceptance(
                prefix_signature,
                admissibility,
                prefix_legality_cache,
                decision,
            ),
            Self::ClaimOpenBand => true,
        }
    }
}

#[derive(Clone, Debug)]
enum RemainingOneNoMissPlateauKernelOutcome<'a> {
    AdmissibilityRejected {
        decision: &'a AdmissibilityDecision,
    },
    Admitted {
        decision: RemainingOneNoMissKernelAdmittedDecision<'a>,
        score: PrefixLocalTerminalClauseScore,
        exact_nu_duration: Duration,
    },
}

fn run_remaining_one_no_miss_plateau_kernel<'a>(
    objective_bar: Rational,
    clause_kappa_used: u16,
    prefix_bit_cost: u32,
    terminal_clause: &'a TerminalPrefixCandidate<'a>,
    connectivity_decision: TerminalConnectivityDecision,
    kernel_mode: RemainingOneNoMissKernelMode<'a>,
    single_clause_nu_context: Option<&SingleClauseStructuralNuContext>,
) -> Option<RemainingOneNoMissPlateauKernelOutcome<'a>> {
    if !matches!(
        connectivity_decision,
        TerminalConnectivityDecision::KeepWithoutFallback
    ) {
        return None;
    }

    match kernel_mode {
        RemainingOneNoMissKernelMode::General(decision) => {
            if !decision.is_admitted() {
                return Some(
                    RemainingOneNoMissPlateauKernelOutcome::AdmissibilityRejected { decision },
                );
            }

            let exact_nu_started = Instant::now();
            let score = prefix_local_terminal_clause_score(
                objective_bar,
                clause_kappa_used,
                prefix_bit_cost,
                terminal_clause.clause,
                terminal_clause.nu_facts,
                single_clause_nu_context,
            )?;
            Some(RemainingOneNoMissPlateauKernelOutcome::Admitted {
                decision: RemainingOneNoMissKernelAdmittedDecision::Cached(decision),
                score,
                exact_nu_duration: exact_nu_started.elapsed(),
            })
        }
        RemainingOneNoMissKernelMode::ClaimAdmittedOpenBand => {
            let exact_nu_started = Instant::now();
            let score = prefix_local_terminal_clause_score(
                objective_bar,
                clause_kappa_used,
                prefix_bit_cost,
                terminal_clause.clause,
                terminal_clause.nu_facts,
                single_clause_nu_context,
            )?;
            Some(RemainingOneNoMissPlateauKernelOutcome::Admitted {
                decision: RemainingOneNoMissKernelAdmittedDecision::ClaimOpenBand,
                score,
                exact_nu_duration: exact_nu_started.elapsed(),
            })
        }
    }
}

fn prefix_local_terminal_clause_score(
    objective_bar: Rational,
    clause_kappa_used: u16,
    prefix_bit_cost: u32,
    clause: &pen_core::clause::ClauseRec,
    clause_nu_facts: &TerminalClauseNuFacts,
    single_clause_nu_context: Option<&SingleClauseStructuralNuContext>,
) -> Option<PrefixLocalTerminalClauseScore> {
    let exact_nu = u16::try_from(
        single_clause_nu_context?
            .structural_nu_with_clause_facts(clause_nu_facts)
            .total,
    )
    .expect("nu exceeded u16");
    let bit_kappa_used = terminal_prefix_completion_bit_cost(prefix_bit_cost, clause);
    Some(PrefixLocalTerminalClauseScore {
        exact_nu,
        bit_kappa_used,
        primary_rank: terminal_prefix_primary_rank(objective_bar, exact_nu, clause_kappa_used),
    })
}

fn terminal_prefix_completion_bit_cost(
    prefix_bit_cost: u32,
    clause: &pen_core::clause::ClauseRec,
) -> u16 {
    u16::try_from(prefix_bit_cost + expr_bit_length(&clause.expr)).expect("bit cost exceeded u16")
}

fn absorb_terminal_prefix_completion_bound(
    bound: &mut Option<PrefixBound>,
    exact_nu: u16,
    clause_kappa_used: u16,
    bit_kappa_used: u16,
) {
    if let Some(bound) = bound.as_mut() {
        bound.absorb_completion(exact_nu, clause_kappa_used, bit_kappa_used);
    } else {
        *bound = Some(PrefixBound::singleton(
            exact_nu,
            clause_kappa_used,
            bit_kappa_used,
        ));
    }
}

fn update_terminal_prefix_best_accept_rank(
    objective_bar: Rational,
    incumbent_rank: Option<&AcceptRank>,
    telescope: &Telescope,
    exact_nu: u16,
    clause_kappa_used: u16,
    bit_kappa_used: u16,
    summary: &mut TerminalPrefixCompletionSummary,
) {
    if should_build_full_accept_rank(
        incumbent_rank,
        terminal_prefix_primary_rank(objective_bar, exact_nu, clause_kappa_used).as_ref(),
        summary,
    ) {
        if let Some(rank) = acceptance_rank_for_telescope(
            objective_bar,
            telescope,
            exact_nu,
            bit_kappa_used,
            clause_kappa_used,
        ) {
            match &summary.best_accept_rank {
                Some(current) if !better_rank(&rank, current) => {}
                _ => summary.best_accept_rank = Some(rank),
            }
        }
    }
}

fn should_build_full_accept_rank(
    incumbent_rank: Option<&AcceptRank>,
    primary_rank: Option<&TerminalPrefixPrimaryRank>,
    summary: &mut TerminalPrefixCompletionSummary,
) -> bool {
    let Some(primary_rank) = primary_rank else {
        return false;
    };
    let build_full_accept_rank = match summary.best_accept_primary_rank.as_ref() {
        Some(current) if better_terminal_prefix_primary_rank(primary_rank, current) => {
            summary.best_accept_primary_rank = Some(primary_rank.clone());
            true
        }
        Some(current) if current == primary_rank => true,
        Some(_) => false,
        None => {
            summary.best_accept_primary_rank = Some(primary_rank.clone());
            true
        }
    };
    build_full_accept_rank
        && incumbent_rank.is_none_or(|incumbent| {
            terminal_prefix_primary_rank_could_still_beat_incumbent(&primary_rank, incumbent)
        })
}

fn absorb_compact_terminal_prefix_admitted_clause_summary(
    incumbent_rank: Option<&AcceptRank>,
    primary_rank: Option<&TerminalPrefixPrimaryRank>,
    exact_nu: u16,
    clause_kappa_used: u16,
    bit_kappa_used: u16,
    competition_allowed: bool,
    summary: &mut TerminalPrefixCompletionSummary,
) -> bool {
    absorb_terminal_prefix_completion_bound(
        &mut summary.bound,
        exact_nu,
        clause_kappa_used,
        bit_kappa_used,
    );
    summary.admitted_candidate_count += 1;
    competition_allowed && should_build_full_accept_rank(incumbent_rank, primary_rank, summary)
}

fn better_terminal_prefix_primary_rank(
    left: &TerminalPrefixPrimaryRank,
    right: &TerminalPrefixPrimaryRank,
) -> bool {
    left.overshoot < right.overshoot
        || (left.overshoot == right.overshoot && left.clause_kappa < right.clause_kappa)
}

fn terminal_prefix_primary_rank(
    objective_bar: Rational,
    exact_nu: u16,
    clause_kappa_used: u16,
) -> Option<TerminalPrefixPrimaryRank> {
    let rho = Rational::new(i64::from(exact_nu), i64::from(clause_kappa_used));
    (rho >= objective_bar).then(|| TerminalPrefixPrimaryRank {
        overshoot: rho - objective_bar,
        clause_kappa: clause_kappa_used,
    })
}

fn terminal_prefix_primary_rank_could_still_beat_incumbent(
    candidate_rank: &TerminalPrefixPrimaryRank,
    incumbent_rank: &AcceptRank,
) -> bool {
    !(candidate_rank.overshoot > incumbent_rank.overshoot
        || (candidate_rank.overshoot == incumbent_rank.overshoot
            && candidate_rank.clause_kappa > incumbent_rank.clause_kappa))
}

fn claim_open_band_admissibility_decision() -> AdmissibilityDecision {
    AdmissibilityDecision {
        class: AdmissibilityDecisionClass::AdmittedFocusAligned,
        reason: CLAIM_OPEN_BAND_ADMISSIBILITY_REASON.to_owned(),
    }
}

fn absorb_terminal_prefix_admitted_clause_summary(
    objective_bar: Rational,
    incumbent_rank: Option<&AcceptRank>,
    telescope: &Telescope,
    exact_nu: u16,
    clause_kappa_used: u16,
    bit_kappa_used: u16,
    competition_allowed: bool,
    admitted_decision: Option<&AdmissibilityDecision>,
    summary: &mut TerminalPrefixCompletionSummary,
) {
    absorb_terminal_prefix_completion_bound(
        &mut summary.bound,
        exact_nu,
        clause_kappa_used,
        bit_kappa_used,
    );
    summary.admitted_candidate_count += 1;
    if competition_allowed {
        update_terminal_prefix_best_accept_rank(
            objective_bar,
            incumbent_rank,
            telescope,
            exact_nu,
            clause_kappa_used,
            bit_kappa_used,
            summary,
        );
    }
    if let Some(evaluations) = summary.evaluations.as_mut() {
        let decision = admitted_decision
            .expect("full payload admitted evaluation should carry its exact decision")
            .clone();
        evaluations.push(TerminalPrefixClauseEvaluation::Admitted {
            decision,
            completion: TerminalPrefixCompletion {
                telescope: telescope.clone(),
                exact_nu,
                bit_kappa_used,
                clause_kappa_used,
            },
        });
    }
}

fn compute_terminal_prefix_completion_summary_from_candidates(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    prefix_signature: &PrefixSignature,
    prefix_telescope: &Telescope,
    payload: TerminalPrefixSummaryPayload,
    terminal_clauses: TerminalPrefixClauseCandidates<'_>,
    incumbent_rank: Option<&AcceptRank>,
    prefix_legality_cache: &mut PrefixLegalityCache,
    remaining_one_telemetry: Option<&mut RemainingOneTelemetry>,
    remaining_one_summary_kernel_activation: Option<RemainingOneSummaryKernelActivationContext>,
) -> TerminalPrefixCompletionSummary {
    let mut summary = TerminalPrefixCompletionSummary {
        evaluations: matches!(payload, TerminalPrefixSummaryPayload::Full).then(Vec::new),
        ..TerminalPrefixCompletionSummary::default()
    };
    let mut compact_survivor_sketch = matches!(payload, TerminalPrefixSummaryPayload::Compact)
        .then(CompactTerminalPrefixSurvivorSketchBuilder::default);
    let prefix_len = prefix_telescope.clauses.len();
    let clause_kappa_used =
        u16::try_from(prefix_len.saturating_add(1)).expect("kappa exceeded u16");
    let prefix_bit_cost = prefix_telescope.bit_cost();
    let mut scratch_telescope = terminal_prefix_scratch_telescope(prefix_telescope);
    let single_clause_nu_context = (!prefix_telescope.clauses.is_empty()).then(|| {
        SingleClauseStructuralNuContext::from_prefix(prefix_telescope, library, nu_history)
    });
    let replay_capture = claim_replay::prepare_remaining_one_summary_capture(
        step_index,
        library,
        admissibility,
        objective_bar,
        nu_history,
        prefix_signature,
        prefix_telescope,
        payload,
        &terminal_clauses,
        incumbent_rank,
        remaining_one_summary_kernel_activation,
    );
    let mut kernel_timing = RemainingOneSummaryKernelTiming::default();
    match terminal_clauses {
        TerminalPrefixClauseCandidates::General(terminal_clauses) => {
            for (clause_index, terminal_clause) in terminal_clauses.into_iter().enumerate() {
                let connectivity_started = Instant::now();
                let Some(connectivity_decision) = prefix_legality_cache
                    .terminal_connectivity_with_facts(
                        prefix_signature,
                        library,
                        terminal_clause.clause,
                        terminal_clause.connectivity_facts,
                    )
                else {
                    continue;
                };
                kernel_timing.terminal_summary_connectivity_checks += 1;
                kernel_timing.terminal_summary_connectivity_duration +=
                    connectivity_started.elapsed();
                let generated_started = Instant::now();
                summary.generated_candidate_count += 1;
                if matches!(
                    connectivity_decision,
                    TerminalConnectivityDecision::PruneDisconnected
                ) {
                    if let Some(evaluations) = summary.evaluations.as_mut() {
                        evaluations.push(TerminalPrefixClauseEvaluation::Disconnected);
                    }
                    kernel_timing.terminal_summary_aggregation_duration +=
                        generated_started.elapsed();
                    continue;
                }
                kernel_timing.terminal_summary_aggregation_duration += generated_started.elapsed();

                if let Some(kernel_mode) = terminal_clause
                    .cached_admissibility_decision
                    .as_ref()
                    .map(RemainingOneNoMissKernelMode::General)
                {
                    if let Some(kernel_outcome) = run_remaining_one_no_miss_plateau_kernel(
                        objective_bar,
                        clause_kappa_used,
                        prefix_bit_cost,
                        &terminal_clause,
                        connectivity_decision,
                        kernel_mode,
                        single_clause_nu_context.as_ref(),
                    ) {
                        match kernel_outcome {
                            RemainingOneNoMissPlateauKernelOutcome::AdmissibilityRejected {
                                decision,
                            } => {
                                let admissibility_bookkeeping_started = Instant::now();
                                summary.admissibility_diagnostics.record(decision);
                                if let Some(evaluations) = summary.evaluations.as_mut() {
                                    evaluations.push(
                                        TerminalPrefixClauseEvaluation::AdmissibilityRejected {
                                            decision: decision.clone(),
                                        },
                                    );
                                }
                                kernel_timing.terminal_summary_aggregation_duration +=
                                    admissibility_bookkeeping_started.elapsed();
                                continue;
                            }
                            RemainingOneNoMissPlateauKernelOutcome::Admitted {
                                decision,
                                score,
                                exact_nu_duration,
                            } => {
                                kernel_timing.terminal_summary_exact_nu_evaluations += 1;
                                kernel_timing.terminal_summary_exact_nu_duration +=
                                    exact_nu_duration;
                                let admitted_decision = decision.clone_for_evaluation();
                                let competition_allowed = decision.competition_allowed(
                                    prefix_signature,
                                    admissibility,
                                    prefix_legality_cache,
                                );
                                let admissibility_bookkeeping_started = Instant::now();
                                summary.admissibility_diagnostics.record(&admitted_decision);
                                kernel_timing.terminal_summary_aggregation_duration +=
                                    admissibility_bookkeeping_started.elapsed();
                                if summary.evaluations.is_none() {
                                    let aggregation_started = Instant::now();
                                    let needs_full_accept_rank =
                                        absorb_compact_terminal_prefix_admitted_clause_summary(
                                            incumbent_rank,
                                            score.primary_rank.as_ref(),
                                            score.exact_nu,
                                            clause_kappa_used,
                                            score.bit_kappa_used,
                                            competition_allowed,
                                            &mut summary,
                                        );
                                    if let Some(sketch) = compact_survivor_sketch.as_mut() {
                                        sketch.record_candidate(
                                            score.primary_rank.as_ref(),
                                            clause_index,
                                            score.exact_nu,
                                            score.bit_kappa_used,
                                            competition_allowed,
                                            incumbent_rank,
                                        );
                                    }
                                    kernel_timing.terminal_summary_aggregation_duration +=
                                        aggregation_started.elapsed();
                                    if !needs_full_accept_rank {
                                        continue;
                                    }

                                    let load_started = Instant::now();
                                    let telescope = load_terminal_clause_into_scratch(
                                        &mut scratch_telescope,
                                        prefix_len,
                                        terminal_clause.clause,
                                    );
                                    kernel_timing.terminal_summary_aggregation_duration +=
                                        load_started.elapsed();
                                    let rank_started = Instant::now();
                                    if let Some(rank) = acceptance_rank_for_telescope(
                                        objective_bar,
                                        telescope,
                                        score.exact_nu,
                                        score.bit_kappa_used,
                                        clause_kappa_used,
                                    ) {
                                        match &summary.best_accept_rank {
                                            Some(current) if !better_rank(&rank, current) => {}
                                            _ => summary.best_accept_rank = Some(rank),
                                        }
                                    }
                                    kernel_timing.terminal_summary_aggregation_duration +=
                                        rank_started.elapsed();
                                    continue;
                                }

                                let load_started = Instant::now();
                                let telescope = load_terminal_clause_into_scratch(
                                    &mut scratch_telescope,
                                    prefix_len,
                                    terminal_clause.clause,
                                );
                                kernel_timing.terminal_summary_aggregation_duration +=
                                    load_started.elapsed();
                                let aggregation_started = Instant::now();
                                absorb_terminal_prefix_admitted_clause_summary(
                                    objective_bar,
                                    incumbent_rank,
                                    telescope,
                                    score.exact_nu,
                                    clause_kappa_used,
                                    score.bit_kappa_used,
                                    competition_allowed,
                                    Some(&admitted_decision),
                                    &mut summary,
                                );
                                kernel_timing.terminal_summary_aggregation_duration +=
                                    aggregation_started.elapsed();
                                continue;
                            }
                        }
                    }
                }

                let load_started = Instant::now();
                let telescope = load_terminal_clause_into_scratch(
                    &mut scratch_telescope,
                    prefix_len,
                    terminal_clause.clause,
                );
                kernel_timing.terminal_summary_aggregation_duration += load_started.elapsed();
                if matches!(
                    connectivity_decision,
                    TerminalConnectivityDecision::NeedsFallback
                ) {
                    kernel_timing.terminal_summary_fallback_connectivity_checks += 1;
                    let fallback_started = Instant::now();
                    if !passes_connectivity(library, telescope) {
                        kernel_timing.terminal_summary_fallback_connectivity_duration +=
                            fallback_started.elapsed();
                        let disconnected_started = Instant::now();
                        if let Some(evaluations) = summary.evaluations.as_mut() {
                            evaluations.push(TerminalPrefixClauseEvaluation::Disconnected);
                        }
                        kernel_timing.terminal_summary_aggregation_duration +=
                            disconnected_started.elapsed();
                        continue;
                    }
                    kernel_timing.terminal_summary_fallback_connectivity_duration +=
                        fallback_started.elapsed();
                }

                let admissibility_decision = if let Some(decision) =
                    terminal_clause.cached_admissibility_decision
                {
                    decision
                } else {
                    kernel_timing.terminal_summary_admissibility_checks += 1;
                    let admissibility_started = Instant::now();
                    let decision =
                        assess_strict_admissibility(step_index, library, telescope, admissibility);
                    kernel_timing.terminal_summary_admissibility_duration +=
                        admissibility_started.elapsed();
                    decision
                };
                let admissibility_bookkeeping_started = Instant::now();
                summary
                    .admissibility_diagnostics
                    .record(&admissibility_decision);
                if !admissibility_decision.is_admitted() {
                    if let Some(evaluations) = summary.evaluations.as_mut() {
                        evaluations.push(TerminalPrefixClauseEvaluation::AdmissibilityRejected {
                            decision: admissibility_decision,
                        });
                    }
                    kernel_timing.terminal_summary_aggregation_duration +=
                        admissibility_bookkeeping_started.elapsed();
                    continue;
                }
                kernel_timing.terminal_summary_aggregation_duration +=
                    admissibility_bookkeeping_started.elapsed();

                kernel_timing.terminal_summary_exact_nu_evaluations += 1;
                let exact_nu_started = Instant::now();
                let exact_nu = u16::try_from(single_clause_structural_nu_total(
                    telescope,
                    terminal_clause.nu_facts,
                    single_clause_nu_context.as_ref(),
                    library,
                    nu_history,
                ))
                .expect("nu exceeded u16");
                kernel_timing.terminal_summary_exact_nu_duration += exact_nu_started.elapsed();
                let aggregation_started = Instant::now();
                let bit_kappa_used =
                    terminal_prefix_completion_bit_cost(prefix_bit_cost, terminal_clause.clause);
                let competition_allowed = terminal_completion_can_compete_for_acceptance(
                    prefix_signature,
                    admissibility,
                    prefix_legality_cache,
                    &admissibility_decision,
                );
                let primary_rank =
                    terminal_prefix_primary_rank(objective_bar, exact_nu, clause_kappa_used);
                if let Some(sketch) = compact_survivor_sketch.as_mut() {
                    sketch.record_candidate(
                        primary_rank.as_ref(),
                        clause_index,
                        exact_nu,
                        bit_kappa_used,
                        competition_allowed,
                        incumbent_rank,
                    );
                }
                absorb_terminal_prefix_admitted_clause_summary(
                    objective_bar,
                    incumbent_rank,
                    telescope,
                    exact_nu,
                    clause_kappa_used,
                    bit_kappa_used,
                    competition_allowed,
                    Some(&admissibility_decision),
                    &mut summary,
                );
                kernel_timing.terminal_summary_aggregation_duration +=
                    aggregation_started.elapsed();
            }
        }
        TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand(terminal_clauses) => {
            let mut admitted_focus_aligned_count = 0usize;
            if summary.evaluations.is_some() {
                let claim_evaluation_decision = claim_open_band_admissibility_decision();
                for terminal_clause in terminal_clauses {
                    let connectivity_started = Instant::now();
                    let Some(connectivity_decision) = prefix_legality_cache
                        .terminal_connectivity_with_facts(
                            prefix_signature,
                            library,
                            terminal_clause.clause,
                            terminal_clause.connectivity_facts,
                        )
                    else {
                        continue;
                    };
                    kernel_timing.terminal_summary_connectivity_checks += 1;
                    kernel_timing.terminal_summary_connectivity_duration +=
                        connectivity_started.elapsed();
                    let generated_started = Instant::now();
                    summary.generated_candidate_count += 1;
                    if matches!(
                        connectivity_decision,
                        TerminalConnectivityDecision::PruneDisconnected
                    ) {
                        if let Some(evaluations) = summary.evaluations.as_mut() {
                            evaluations.push(TerminalPrefixClauseEvaluation::Disconnected);
                        }
                        kernel_timing.terminal_summary_aggregation_duration +=
                            generated_started.elapsed();
                        continue;
                    }
                    kernel_timing.terminal_summary_aggregation_duration +=
                        generated_started.elapsed();

                    if let Some(kernel_outcome) = run_remaining_one_no_miss_plateau_kernel(
                        objective_bar,
                        clause_kappa_used,
                        prefix_bit_cost,
                        &terminal_clause,
                        connectivity_decision,
                        RemainingOneNoMissKernelMode::ClaimAdmittedOpenBand,
                        single_clause_nu_context.as_ref(),
                    ) {
                        match kernel_outcome {
                            RemainingOneNoMissPlateauKernelOutcome::AdmissibilityRejected {
                                ..
                            } => {
                                continue;
                            }
                            RemainingOneNoMissPlateauKernelOutcome::Admitted {
                                score,
                                exact_nu_duration,
                                ..
                            } => {
                                admitted_focus_aligned_count += 1;
                                kernel_timing.terminal_summary_exact_nu_evaluations += 1;
                                kernel_timing.terminal_summary_exact_nu_duration +=
                                    exact_nu_duration;
                                let load_started = Instant::now();
                                let telescope = load_terminal_clause_into_scratch(
                                    &mut scratch_telescope,
                                    prefix_len,
                                    terminal_clause.clause,
                                );
                                kernel_timing.terminal_summary_aggregation_duration +=
                                    load_started.elapsed();
                                let aggregation_started = Instant::now();
                                absorb_terminal_prefix_admitted_clause_summary(
                                    objective_bar,
                                    incumbent_rank,
                                    telescope,
                                    score.exact_nu,
                                    clause_kappa_used,
                                    score.bit_kappa_used,
                                    true,
                                    Some(&claim_evaluation_decision),
                                    &mut summary,
                                );
                                kernel_timing.terminal_summary_aggregation_duration +=
                                    aggregation_started.elapsed();
                                continue;
                            }
                        }
                    }

                    let load_started = Instant::now();
                    let telescope = load_terminal_clause_into_scratch(
                        &mut scratch_telescope,
                        prefix_len,
                        terminal_clause.clause,
                    );
                    kernel_timing.terminal_summary_aggregation_duration += load_started.elapsed();
                    if matches!(
                        connectivity_decision,
                        TerminalConnectivityDecision::NeedsFallback
                    ) {
                        kernel_timing.terminal_summary_fallback_connectivity_checks += 1;
                        let fallback_started = Instant::now();
                        if !passes_connectivity(library, telescope) {
                            kernel_timing.terminal_summary_fallback_connectivity_duration +=
                                fallback_started.elapsed();
                            let disconnected_started = Instant::now();
                            if let Some(evaluations) = summary.evaluations.as_mut() {
                                evaluations.push(TerminalPrefixClauseEvaluation::Disconnected);
                            }
                            kernel_timing.terminal_summary_aggregation_duration +=
                                disconnected_started.elapsed();
                            continue;
                        }
                        kernel_timing.terminal_summary_fallback_connectivity_duration +=
                            fallback_started.elapsed();
                    }

                    admitted_focus_aligned_count += 1;
                    kernel_timing.terminal_summary_exact_nu_evaluations += 1;
                    let exact_nu_started = Instant::now();
                    let exact_nu = u16::try_from(single_clause_structural_nu_total(
                        telescope,
                        terminal_clause.nu_facts,
                        single_clause_nu_context.as_ref(),
                        library,
                        nu_history,
                    ))
                    .expect("nu exceeded u16");
                    kernel_timing.terminal_summary_exact_nu_duration += exact_nu_started.elapsed();
                    let aggregation_started = Instant::now();
                    let bit_kappa_used = terminal_prefix_completion_bit_cost(
                        prefix_bit_cost,
                        terminal_clause.clause,
                    );
                    absorb_terminal_prefix_admitted_clause_summary(
                        objective_bar,
                        incumbent_rank,
                        telescope,
                        exact_nu,
                        clause_kappa_used,
                        bit_kappa_used,
                        true,
                        Some(&claim_evaluation_decision),
                        &mut summary,
                    );
                    kernel_timing.terminal_summary_aggregation_duration +=
                        aggregation_started.elapsed();
                }
            } else {
                for (clause_index, terminal_clause) in terminal_clauses.into_iter().enumerate() {
                    let candidate_started = Instant::now();
                    let Some(connectivity_decision) = prefix_legality_cache
                        .terminal_connectivity_with_facts(
                            prefix_signature,
                            library,
                            terminal_clause.clause,
                            terminal_clause.connectivity_facts,
                        )
                    else {
                        continue;
                    };
                    let after_connectivity = Instant::now();
                    kernel_timing.terminal_summary_connectivity_checks += 1;
                    kernel_timing.terminal_summary_connectivity_duration +=
                        after_connectivity.duration_since(candidate_started);
                    summary.generated_candidate_count += 1;
                    if matches!(
                        connectivity_decision,
                        TerminalConnectivityDecision::PruneDisconnected
                    ) {
                        let after_disconnected = Instant::now();
                        kernel_timing.terminal_summary_aggregation_duration +=
                            after_disconnected.duration_since(after_connectivity);
                        continue;
                    }

                    if let Some(kernel_outcome) = run_remaining_one_no_miss_plateau_kernel(
                        objective_bar,
                        clause_kappa_used,
                        prefix_bit_cost,
                        &terminal_clause,
                        connectivity_decision,
                        RemainingOneNoMissKernelMode::ClaimAdmittedOpenBand,
                        single_clause_nu_context.as_ref(),
                    ) {
                        match kernel_outcome {
                            RemainingOneNoMissPlateauKernelOutcome::AdmissibilityRejected {
                                ..
                            } => {
                                continue;
                            }
                            RemainingOneNoMissPlateauKernelOutcome::Admitted {
                                score,
                                exact_nu_duration,
                                ..
                            } => {
                                kernel_timing.terminal_summary_exact_nu_evaluations += 1;
                                kernel_timing.terminal_summary_exact_nu_duration +=
                                    exact_nu_duration;
                                admitted_focus_aligned_count += 1;
                                let aggregation_started = Instant::now();
                                let needs_full_accept_rank =
                                    absorb_compact_terminal_prefix_admitted_clause_summary(
                                        incumbent_rank,
                                        score.primary_rank.as_ref(),
                                        score.exact_nu,
                                        clause_kappa_used,
                                        score.bit_kappa_used,
                                        true,
                                        &mut summary,
                                    );
                                if let Some(sketch) = compact_survivor_sketch.as_mut() {
                                    sketch.record_candidate(
                                        score.primary_rank.as_ref(),
                                        clause_index,
                                        score.exact_nu,
                                        score.bit_kappa_used,
                                        true,
                                        incumbent_rank,
                                    );
                                }
                                kernel_timing.terminal_summary_aggregation_duration +=
                                    aggregation_started.elapsed();
                                if !needs_full_accept_rank {
                                    continue;
                                }

                                let load_started = Instant::now();
                                let telescope = load_terminal_clause_into_scratch(
                                    &mut scratch_telescope,
                                    prefix_len,
                                    terminal_clause.clause,
                                );
                                kernel_timing.terminal_summary_aggregation_duration +=
                                    load_started.elapsed();
                                let rank_started = Instant::now();
                                if let Some(rank) = acceptance_rank_for_telescope(
                                    objective_bar,
                                    telescope,
                                    score.exact_nu,
                                    score.bit_kappa_used,
                                    clause_kappa_used,
                                ) {
                                    match &summary.best_accept_rank {
                                        Some(current) if !better_rank(&rank, current) => {}
                                        _ => summary.best_accept_rank = Some(rank),
                                    }
                                }
                                kernel_timing.terminal_summary_aggregation_duration +=
                                    rank_started.elapsed();
                                continue;
                            }
                        }
                    }

                    let telescope = load_terminal_clause_into_scratch(
                        &mut scratch_telescope,
                        prefix_len,
                        terminal_clause.clause,
                    );
                    let mut after_aggregation = Instant::now();
                    kernel_timing.terminal_summary_aggregation_duration +=
                        after_aggregation.duration_since(after_connectivity);
                    if matches!(
                        connectivity_decision,
                        TerminalConnectivityDecision::NeedsFallback
                    ) {
                        kernel_timing.terminal_summary_fallback_connectivity_checks += 1;
                        let fallback_started = after_aggregation;
                        if !passes_connectivity(library, telescope) {
                            let after_fallback = Instant::now();
                            kernel_timing.terminal_summary_fallback_connectivity_duration +=
                                after_fallback.duration_since(fallback_started);
                            continue;
                        }
                        let after_fallback = Instant::now();
                        kernel_timing.terminal_summary_fallback_connectivity_duration +=
                            after_fallback.duration_since(fallback_started);
                        after_aggregation = after_fallback;
                    }

                    admitted_focus_aligned_count += 1;
                    kernel_timing.terminal_summary_exact_nu_evaluations += 1;
                    let exact_nu_started = Instant::now();
                    kernel_timing.terminal_summary_aggregation_duration +=
                        exact_nu_started.duration_since(after_aggregation);
                    let exact_nu = u16::try_from(single_clause_structural_nu_total(
                        telescope,
                        terminal_clause.nu_facts,
                        single_clause_nu_context.as_ref(),
                        library,
                        nu_history,
                    ))
                    .expect("nu exceeded u16");
                    let after_exact_nu = Instant::now();
                    kernel_timing.terminal_summary_exact_nu_duration +=
                        after_exact_nu.duration_since(exact_nu_started);
                    let bit_kappa_used = terminal_prefix_completion_bit_cost(
                        prefix_bit_cost,
                        terminal_clause.clause,
                    );
                    let primary_rank =
                        terminal_prefix_primary_rank(objective_bar, exact_nu, clause_kappa_used);
                    if let Some(sketch) = compact_survivor_sketch.as_mut() {
                        sketch.record_candidate(
                            primary_rank.as_ref(),
                            clause_index,
                            exact_nu,
                            bit_kappa_used,
                            true,
                            incumbent_rank,
                        );
                    }
                    absorb_terminal_prefix_completion_bound(
                        &mut summary.bound,
                        exact_nu,
                        clause_kappa_used,
                        bit_kappa_used,
                    );
                    summary.admitted_candidate_count += 1;
                    update_terminal_prefix_best_accept_rank(
                        objective_bar,
                        incumbent_rank,
                        telescope,
                        exact_nu,
                        clause_kappa_used,
                        bit_kappa_used,
                        &mut summary,
                    );
                    let after_rank_update = Instant::now();
                    kernel_timing.terminal_summary_aggregation_duration +=
                        after_rank_update.duration_since(after_exact_nu);
                }
            }
            summary.admissibility_diagnostics.record_repeated(
                AdmissibilityDecisionClass::AdmittedFocusAligned,
                CLAIM_OPEN_BAND_ADMISSIBILITY_REASON,
                admitted_focus_aligned_count,
            );
        }
    }
    summary.compact_survivor_sketch =
        compact_survivor_sketch.and_then(CompactTerminalPrefixSurvivorSketchBuilder::finish);

    if let Some(telemetry) = remaining_one_telemetry {
        telemetry.absorb_terminal_summary_kernel_timing(kernel_timing);
        telemetry.note_terminal_summary_plateau_activation(remaining_one_summary_kernel_activation);
    }
    claim_replay::finish_remaining_one_summary_capture(replay_capture, &summary);

    summary
}

fn demo_focus_aligned_competitors_only(
    prefix_signature: &PrefixSignature,
    admissibility: StrictAdmissibility,
    prefix_legality_cache: &PrefixLegalityCache,
) -> bool {
    matches!(prefix_signature.obligation_set_id.get(), 10 | 11 | 12 | 14)
        && prefix_legality_cache
            .uses_family_surface_override_for_signature(prefix_signature, admissibility)
}

fn terminal_completion_can_compete_for_acceptance(
    prefix_signature: &PrefixSignature,
    admissibility: StrictAdmissibility,
    prefix_legality_cache: &PrefixLegalityCache,
    admissibility_decision: &pen_type::admissibility::AdmissibilityDecision,
) -> bool {
    admitted_terminal_completion_can_compete_for_acceptance(
        demo_focus_aligned_competitors_only(prefix_signature, admissibility, prefix_legality_cache),
        admissibility_decision,
    )
}

fn admitted_terminal_completion_can_compete_for_acceptance(
    focus_aligned_competitors_only: bool,
    admissibility_decision: &pen_type::admissibility::AdmissibilityDecision,
) -> bool {
    !focus_aligned_competitors_only
        || !matches!(
            admissibility_decision.class,
            AdmissibilityDecisionClass::AdmittedDeprioritized
        )
}

fn materialize_remaining_one_prefix_group(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    prefix_signature: &PrefixSignature,
    prefix_telescope: &Telescope,
    filtered_last_clause_options: &[pen_core::clause::ClauseRec],
    filtered_last_clause_connectivity_facts: Option<&[TerminalClauseConnectivityFacts]>,
    filtered_last_clause_nu_facts: &[TerminalClauseNuFacts],
    discovery: &mut RealisticShadowDiscovery,
) -> Result<MaterializedTerminalPrefixGroup> {
    let materialize_started = Instant::now();
    let group = materialize_terminal_prefix_group(
        step_index,
        library,
        admissibility,
        objective_bar,
        nu_history,
        prefix_signature,
        prefix_telescope,
        filtered_last_clause_options,
        filtered_last_clause_connectivity_facts,
        filtered_last_clause_nu_facts,
        discovery,
    )?;
    discovery.remaining_one_telemetry.remaining_one_materialized += 1;
    discovery
        .remaining_one_telemetry
        .terminal_materialize_millis = discovery
        .remaining_one_telemetry
        .terminal_materialize_millis
        .saturating_add(elapsed_millis(materialize_started.elapsed()));
    Ok(group)
}

fn record_terminal_prefix_summary_discovery_counts(
    discovery: &mut RealisticShadowDiscovery,
    summary: &TerminalPrefixCompletionSummary,
) {
    let evaluated_candidates = summary.admitted_candidate_count
        + summary.admissibility_diagnostics.exact_legality_rejections
        + summary
            .admissibility_diagnostics
            .structural_debt_cap_rejections;
    discovery.raw_generated_surface += summary.generated_candidate_count;
    discovery.enumerated_candidates += evaluated_candidates;
    discovery.well_formed_candidates += evaluated_candidates;
    discovery.admissibility_rejections +=
        summary.admissibility_diagnostics.exact_legality_rejections;
    discovery.admissibility_rejections += summary
        .admissibility_diagnostics
        .structural_debt_cap_rejections;
    discovery
        .admissibility_diagnostics
        .exact_legality_rejections += summary.admissibility_diagnostics.exact_legality_rejections;
    discovery
        .admissibility_diagnostics
        .structural_debt_cap_rejections += summary
        .admissibility_diagnostics
        .structural_debt_cap_rejections;
    discovery.admissibility_diagnostics.admitted_deprioritized +=
        summary.admissibility_diagnostics.admitted_deprioritized;
    discovery.admissibility_diagnostics.admitted_focus_aligned +=
        summary.admissibility_diagnostics.admitted_focus_aligned;
    for (reason, count) in &summary.admissibility_diagnostics.reason_counts {
        *discovery
            .admissibility_diagnostics
            .reason_counts
            .entry(reason.clone())
            .or_insert(0) += *count;
    }
}

fn claim_try_summary_prune_before_materialization(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    prefix_signature: &PrefixSignature,
    prefix_telescope: &Telescope,
    clause_kappa: u16,
    filtered_last_clause_options: &[pen_core::clause::ClauseRec],
    filtered_last_clause_connectivity_facts: Option<&[TerminalClauseConnectivityFacts]>,
    filtered_last_clause_nu_facts: &[TerminalClauseNuFacts],
    discovery: &mut RealisticShadowDiscovery,
) -> bool {
    if !should_compact_terminal_prefix_group_candidates(admissibility.mode) {
        return false;
    }

    if claim_remaining_one_algebraic_nu_ceiling_cannot_clear_bar(
        library,
        admissibility,
        objective_bar,
        nu_history,
        prefix_telescope,
    ) {
        discovery
            .prefix_legality_cache
            .store_partial_prefix_bound_decision(
                prefix_signature.clone(),
                PartialPrefixBoundDecision::CannotClearBar,
            );
        discovery.terminal_prefix_bar_prunes += 1;
        discovery
            .remaining_one_telemetry
            .remaining_one_algebraic_prunes += 1;
        discovery
            .remaining_one_telemetry
            .remaining_one_bar_prunes_pre_materialize += 1;
        return true;
    }

    if discovery
        .prefix_legality_cache
        .terminal_prefix_bound_summary(prefix_signature)
        .is_none()
    {
        let remaining_one_summary_kernel_activation =
            remaining_one_summary_kernel_activation_context(discovery);
        let summary_started = Instant::now();
        let terminal_clauses = terminal_prefix_clause_candidates(
            step_index,
            library,
            admissibility,
            prefix_signature,
            filtered_last_clause_options,
            filtered_last_clause_connectivity_facts,
            filtered_last_clause_nu_facts,
            &mut discovery.prefix_legality_cache,
            Some(&mut discovery.remaining_one_telemetry),
        );
        let summary = compute_terminal_prefix_completion_summary_from_candidates(
            step_index,
            library,
            admissibility,
            objective_bar,
            nu_history,
            prefix_signature,
            prefix_telescope,
            TerminalPrefixSummaryPayload::Compact,
            terminal_clauses,
            discovery.terminal_rank_incumbent.as_ref(),
            &mut discovery.prefix_legality_cache,
            Some(&mut discovery.remaining_one_telemetry),
            Some(remaining_one_summary_kernel_activation),
        );
        discovery
            .remaining_one_telemetry
            .absorb_terminal_summary_build_duration(summary_started.elapsed());
        discovery
            .prefix_legality_cache
            .store_terminal_prefix_completion_summary(prefix_signature.clone(), summary);
    }

    let Some(summary) = discovery
        .prefix_legality_cache
        .take_terminal_prefix_completion_summary(prefix_signature)
    else {
        return false;
    };

    let generated_terminal_candidates = summary.generated_candidate_count;
    let admitted_terminal_candidates = summary.admitted_candidate_count;
    let bucket_key = demo_bucket_key(
        admissibility.mode,
        prefix_signature,
        prefix_telescope,
        clause_kappa,
        generated_terminal_candidates,
        admitted_terminal_candidates,
    );
    discovery.record_demo_bucket_surface(
        bucket_key.clone(),
        generated_terminal_candidates,
        admitted_terminal_candidates,
    );
    if summary.bound.is_some() {
        discovery
            .record_demo_bucket_exact_screened(bucket_key.clone(), admitted_terminal_candidates);
    }
    if summary.bound.is_none() {
        record_terminal_prefix_summary_discovery_counts(discovery, &summary);
        return true;
    }

    if !summary
        .bound
        .expect("checked for missing bound above")
        .can_clear_bar(objective_bar)
    {
        discovery.record_demo_bucket_pruned(bucket_key, admitted_terminal_candidates);
        record_terminal_prefix_summary_discovery_counts(discovery, &summary);
        discovery.terminal_prefix_bar_prunes += 1;
        discovery
            .remaining_one_telemetry
            .remaining_one_bar_prunes_pre_materialize += 1;
        return true;
    }

    let Some(pruned_candidates) = terminal_prefix_rank_prune_count(
        summary.best_accept_rank.as_ref(),
        summary.best_accept_primary_rank.as_ref(),
        admitted_terminal_candidates,
        discovery.terminal_rank_incumbent.as_ref(),
    ) else {
        discovery
            .prefix_legality_cache
            .store_terminal_prefix_completion_summary(prefix_signature.clone(), summary);
        return false;
    };

    discovery.record_demo_bucket_pruned(bucket_key, pruned_candidates);
    record_terminal_prefix_summary_discovery_counts(discovery, &summary);
    discovery.terminal_rank_prunes += pruned_candidates;
    discovery
        .remaining_one_telemetry
        .remaining_one_cached_rank_prunes += 1;
    discovery
        .remaining_one_telemetry
        .remaining_one_rank_prunes_pre_materialize += 1;
    true
}

fn materialize_terminal_prefix_group(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    prefix_signature: &PrefixSignature,
    prefix_telescope: &Telescope,
    filtered_last_clause_options: &[pen_core::clause::ClauseRec],
    filtered_last_clause_connectivity_facts: Option<&[TerminalClauseConnectivityFacts]>,
    filtered_last_clause_nu_facts: &[TerminalClauseNuFacts],
    discovery: &mut RealisticShadowDiscovery,
) -> Result<MaterializedTerminalPrefixGroup> {
    if should_compact_terminal_prefix_group_candidates(admissibility.mode) {
        if let Some(summary) = discovery
            .prefix_legality_cache
            .take_terminal_prefix_completion_summary(prefix_signature)
        {
            if summary.evaluations.is_some() {
                discovery
                    .remaining_one_telemetry
                    .remaining_one_materialized_from_cached_summary += 1;
                return Ok(materialize_terminal_prefix_group_from_summary(
                    admissibility,
                    objective_bar,
                    prefix_signature,
                    summary,
                    discovery,
                ));
            }
            if summary.compact_survivor_sketch.is_some() {
                discovery
                    .remaining_one_telemetry
                    .remaining_one_materialized_from_cached_summary += 1;
                return Ok(materialize_terminal_prefix_group_from_survivor_sketch(
                    objective_bar,
                    prefix_telescope,
                    filtered_last_clause_options,
                    summary,
                    discovery,
                ));
            }
        }

        discovery
            .remaining_one_telemetry
            .remaining_one_materialized_compact_direct += 1;
        return materialize_terminal_prefix_group_compact(
            step_index,
            library,
            admissibility,
            objective_bar,
            nu_history,
            prefix_signature,
            prefix_telescope,
            filtered_last_clause_options,
            filtered_last_clause_connectivity_facts,
            filtered_last_clause_nu_facts,
            discovery,
        );
    }

    let summary = if let Some(summary) = discovery
        .prefix_legality_cache
        .terminal_prefix_completion_summary(prefix_signature)
    {
        summary
    } else {
        let remaining_one_summary_kernel_activation =
            remaining_one_summary_kernel_activation_context(discovery);
        let summary_started = Instant::now();
        let terminal_clauses = terminal_prefix_clause_candidates(
            step_index,
            library,
            admissibility,
            prefix_signature,
            filtered_last_clause_options,
            filtered_last_clause_connectivity_facts,
            filtered_last_clause_nu_facts,
            &mut discovery.prefix_legality_cache,
            Some(&mut discovery.remaining_one_telemetry),
        );
        let summary = compute_terminal_prefix_completion_summary_from_candidates(
            step_index,
            library,
            admissibility,
            objective_bar,
            nu_history,
            prefix_signature,
            prefix_telescope,
            TerminalPrefixSummaryPayload::Full,
            terminal_clauses,
            discovery.terminal_rank_incumbent.as_ref(),
            &mut discovery.prefix_legality_cache,
            Some(&mut discovery.remaining_one_telemetry),
            Some(remaining_one_summary_kernel_activation),
        );
        discovery
            .remaining_one_telemetry
            .absorb_terminal_summary_build_duration(summary_started.elapsed());
        discovery
            .prefix_legality_cache
            .store_terminal_prefix_completion_summary(prefix_signature.clone(), summary.clone());
        summary
    };

    Ok(materialize_terminal_prefix_group_from_summary(
        admissibility,
        objective_bar,
        prefix_signature,
        summary,
        discovery,
    ))
}

fn materialize_terminal_prefix_group_from_summary(
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    prefix_signature: &PrefixSignature,
    summary: TerminalPrefixCompletionSummary,
    discovery: &mut RealisticShadowDiscovery,
) -> MaterializedTerminalPrefixGroup {
    record_terminal_prefix_summary_discovery_counts(discovery, &summary);
    let admitted_terminal_candidates = summary.admitted_candidate_count;
    let best_accept_rank = summary.best_accept_rank.clone();
    let generated_terminal_candidates = summary.generated_candidate_count;
    let bound = summary.bound;
    let evaluations = summary
        .evaluations
        .expect("cached completion payload missing for summary materialization");
    let mut retained_telescopes = Vec::new();

    for evaluation in evaluations {
        match evaluation {
            TerminalPrefixClauseEvaluation::Disconnected => {}
            TerminalPrefixClauseEvaluation::AdmissibilityRejected { .. } => {}
            TerminalPrefixClauseEvaluation::Admitted {
                decision,
                completion,
            } => {
                if !terminal_completion_can_compete_for_acceptance(
                    prefix_signature,
                    admissibility,
                    &discovery.prefix_legality_cache,
                    &decision,
                ) {
                    continue;
                }
                let accept_rank = acceptance_rank_for_telescope(
                    objective_bar,
                    &completion.telescope,
                    completion.exact_nu,
                    completion.bit_kappa_used,
                    completion.clause_kappa_used,
                );
                retained_telescopes.push(PrefixGroupCandidate {
                    telescope: completion.telescope,
                    accept_rank,
                    evaluated_candidate: None,
                });
            }
        }
    }

    MaterializedTerminalPrefixGroup {
        candidates: retained_telescopes,
        bound,
        best_accept_rank,
        generated_terminal_candidates,
        admissible_terminal_candidates: admitted_terminal_candidates,
    }
}

fn materialize_terminal_prefix_group_from_survivor_sketch(
    objective_bar: Rational,
    prefix_telescope: &Telescope,
    filtered_last_clause_options: &[pen_core::clause::ClauseRec],
    summary: TerminalPrefixCompletionSummary,
    discovery: &mut RealisticShadowDiscovery,
) -> MaterializedTerminalPrefixGroup {
    record_terminal_prefix_summary_discovery_counts(discovery, &summary);
    let admitted_terminal_candidates = summary.admitted_candidate_count;
    let best_accept_rank = summary.best_accept_rank.clone();
    let generated_terminal_candidates = summary.generated_candidate_count;
    let bound = summary.bound;
    let clause_kappa_used = u16::try_from(prefix_telescope.clauses.len().saturating_add(1))
        .expect("kappa exceeded u16");
    let sketch = summary
        .compact_survivor_sketch
        .expect("compact survivor sketch should exist for sketch materialization");
    let prefix_len = prefix_telescope.clauses.len();
    let mut scratch_telescope = terminal_prefix_scratch_telescope(prefix_telescope);
    let mut retained_telescopes = Vec::with_capacity(sketch.survivors.len());

    for survivor in sketch.survivors {
        let clause = filtered_last_clause_options
            .get(survivor.clause_index)
            .expect("survivor sketch clause index should align with filtered terminal clauses");
        let telescope =
            load_terminal_clause_into_scratch(&mut scratch_telescope, prefix_len, clause);
        retained_telescopes.push(PrefixGroupCandidate {
            telescope: telescope.clone(),
            accept_rank: acceptance_rank_for_telescope(
                objective_bar,
                telescope,
                survivor.exact_nu,
                survivor.bit_kappa_used,
                clause_kappa_used,
            ),
            evaluated_candidate: None,
        });
    }

    MaterializedTerminalPrefixGroup {
        candidates: retained_telescopes,
        bound,
        best_accept_rank,
        generated_terminal_candidates,
        admissible_terminal_candidates: admitted_terminal_candidates,
    }
}

fn materialize_terminal_prefix_group_compact(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    prefix_signature: &PrefixSignature,
    prefix_telescope: &Telescope,
    filtered_last_clause_options: &[pen_core::clause::ClauseRec],
    filtered_last_clause_connectivity_facts: Option<&[TerminalClauseConnectivityFacts]>,
    filtered_last_clause_nu_facts: &[TerminalClauseNuFacts],
    discovery: &mut RealisticShadowDiscovery,
) -> Result<MaterializedTerminalPrefixGroup> {
    let terminal_clauses = terminal_prefix_clause_candidates(
        step_index,
        library,
        admissibility,
        prefix_signature,
        filtered_last_clause_options,
        filtered_last_clause_connectivity_facts,
        filtered_last_clause_nu_facts,
        &mut discovery.prefix_legality_cache,
        Some(&mut discovery.remaining_one_telemetry),
    );
    let mut generated_terminal_candidates = 0usize;
    let mut admitted_terminal_candidates = 0usize;
    let mut bound: Option<PrefixBound> = None;
    let mut best_accept_rank = None;
    let mut retained_telescopes = Vec::new();
    let prefix_len = prefix_telescope.clauses.len();
    let clause_kappa_used =
        u16::try_from(prefix_len.saturating_add(1)).expect("kappa exceeded u16");
    let prefix_bit_cost = prefix_telescope.bit_cost();
    let mut scratch_telescope = terminal_prefix_scratch_telescope(prefix_telescope);
    let single_clause_nu_context = (!prefix_telescope.clauses.is_empty()).then(|| {
        SingleClauseStructuralNuContext::from_prefix(prefix_telescope, library, nu_history)
    });
    match terminal_clauses {
        TerminalPrefixClauseCandidates::General(terminal_clauses) => {
            for terminal_clause in terminal_clauses {
                let Some(connectivity_decision) = discovery
                    .prefix_legality_cache
                    .terminal_connectivity_with_facts(
                        prefix_signature,
                        library,
                        terminal_clause.clause,
                        terminal_clause.connectivity_facts,
                    )
                else {
                    continue;
                };
                generated_terminal_candidates += 1;
                if matches!(
                    connectivity_decision,
                    TerminalConnectivityDecision::PruneDisconnected
                ) {
                    continue;
                }
                if let Some(kernel_mode) = terminal_clause
                    .cached_admissibility_decision
                    .as_ref()
                    .map(RemainingOneNoMissKernelMode::General)
                {
                    if let Some(kernel_outcome) = run_remaining_one_no_miss_plateau_kernel(
                        objective_bar,
                        clause_kappa_used,
                        prefix_bit_cost,
                        &terminal_clause,
                        connectivity_decision,
                        kernel_mode,
                        single_clause_nu_context.as_ref(),
                    ) {
                        discovery.enumerated_candidates += 1;
                        discovery.well_formed_candidates += 1;
                        match kernel_outcome {
                            RemainingOneNoMissPlateauKernelOutcome::AdmissibilityRejected {
                                decision,
                            } => {
                                discovery.admissibility_diagnostics.record(decision);
                                discovery.admissibility_rejections += 1;
                                continue;
                            }
                            RemainingOneNoMissPlateauKernelOutcome::Admitted {
                                decision,
                                score,
                                ..
                            } => {
                                let admitted_decision = decision.clone_for_evaluation();
                                discovery
                                    .admissibility_diagnostics
                                    .record(&admitted_decision);
                                admitted_terminal_candidates += 1;
                                absorb_terminal_prefix_completion_bound(
                                    &mut bound,
                                    score.exact_nu,
                                    clause_kappa_used,
                                    score.bit_kappa_used,
                                );
                                if !decision.competition_allowed(
                                    prefix_signature,
                                    admissibility,
                                    &discovery.prefix_legality_cache,
                                ) || score.primary_rank.is_none()
                                {
                                    continue;
                                }

                                let telescope = load_terminal_clause_into_scratch(
                                    &mut scratch_telescope,
                                    prefix_len,
                                    terminal_clause.clause,
                                );
                                let accept_rank = acceptance_rank_for_telescope(
                                    objective_bar,
                                    telescope,
                                    score.exact_nu,
                                    score.bit_kappa_used,
                                    clause_kappa_used,
                                );
                                if let Some(rank) = accept_rank.as_ref() {
                                    match &best_accept_rank {
                                        Some(current) if !better_rank(rank, current) => {}
                                        _ => best_accept_rank = Some(rank.clone()),
                                    }
                                }
                                retained_telescopes.push(PrefixGroupCandidate {
                                    telescope: telescope.clone(),
                                    accept_rank,
                                    evaluated_candidate: None,
                                });
                                continue;
                            }
                        }
                    }
                }

                let telescope = load_terminal_clause_into_scratch(
                    &mut scratch_telescope,
                    prefix_len,
                    terminal_clause.clause,
                );
                if matches!(
                    connectivity_decision,
                    TerminalConnectivityDecision::NeedsFallback
                ) {
                    if !passes_connectivity(library, telescope) {
                        continue;
                    }
                }

                let admissibility_decision =
                    if let Some(decision) = terminal_clause.cached_admissibility_decision {
                        decision
                    } else {
                        assess_strict_admissibility(step_index, library, telescope, admissibility)
                    };
                discovery.enumerated_candidates += 1;
                discovery.well_formed_candidates += 1;
                discovery
                    .admissibility_diagnostics
                    .record(&admissibility_decision);
                if !admissibility_decision.is_admitted() {
                    discovery.admissibility_rejections += 1;
                    continue;
                }

                admitted_terminal_candidates += 1;
                let exact_nu = u16::try_from(single_clause_structural_nu_total(
                    telescope,
                    terminal_clause.nu_facts,
                    single_clause_nu_context.as_ref(),
                    library,
                    nu_history,
                ))
                .expect("nu exceeded u16");
                let bit_kappa_used =
                    terminal_prefix_completion_bit_cost(prefix_bit_cost, terminal_clause.clause);
                absorb_terminal_prefix_completion_bound(
                    &mut bound,
                    exact_nu,
                    clause_kappa_used,
                    bit_kappa_used,
                );

                if !terminal_completion_can_compete_for_acceptance(
                    prefix_signature,
                    admissibility,
                    &discovery.prefix_legality_cache,
                    &admissibility_decision,
                ) {
                    continue;
                }

                let accept_rank = acceptance_rank_for_telescope(
                    objective_bar,
                    telescope,
                    exact_nu,
                    bit_kappa_used,
                    clause_kappa_used,
                );
                if let Some(rank) = accept_rank.as_ref() {
                    match &best_accept_rank {
                        Some(current) if !better_rank(rank, current) => {}
                        _ => best_accept_rank = Some(rank.clone()),
                    }
                }
                retained_telescopes.push(PrefixGroupCandidate {
                    telescope: telescope.clone(),
                    accept_rank,
                    evaluated_candidate: None,
                });
            }
        }
        TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand(terminal_clauses) => {
            let mut admitted_focus_aligned_count = 0usize;
            for terminal_clause in terminal_clauses {
                let Some(connectivity_decision) = discovery
                    .prefix_legality_cache
                    .terminal_connectivity_with_facts(
                        prefix_signature,
                        library,
                        terminal_clause.clause,
                        terminal_clause.connectivity_facts,
                    )
                else {
                    continue;
                };
                generated_terminal_candidates += 1;
                if matches!(
                    connectivity_decision,
                    TerminalConnectivityDecision::PruneDisconnected
                ) {
                    continue;
                }
                if let Some(kernel_outcome) = run_remaining_one_no_miss_plateau_kernel(
                    objective_bar,
                    clause_kappa_used,
                    prefix_bit_cost,
                    &terminal_clause,
                    connectivity_decision,
                    RemainingOneNoMissKernelMode::ClaimAdmittedOpenBand,
                    single_clause_nu_context.as_ref(),
                ) {
                    match kernel_outcome {
                        RemainingOneNoMissPlateauKernelOutcome::AdmissibilityRejected {
                            ..
                        } => {
                            continue;
                        }
                        RemainingOneNoMissPlateauKernelOutcome::Admitted { score, .. } => {
                            admitted_focus_aligned_count += 1;
                            discovery.enumerated_candidates += 1;
                            discovery.well_formed_candidates += 1;
                            admitted_terminal_candidates += 1;
                            absorb_terminal_prefix_completion_bound(
                                &mut bound,
                                score.exact_nu,
                                clause_kappa_used,
                                score.bit_kappa_used,
                            );
                            if score.primary_rank.is_none() {
                                continue;
                            }

                            let telescope = load_terminal_clause_into_scratch(
                                &mut scratch_telescope,
                                prefix_len,
                                terminal_clause.clause,
                            );
                            let accept_rank = acceptance_rank_for_telescope(
                                objective_bar,
                                telescope,
                                score.exact_nu,
                                score.bit_kappa_used,
                                clause_kappa_used,
                            );
                            if let Some(rank) = accept_rank.as_ref() {
                                match &best_accept_rank {
                                    Some(current) if !better_rank(rank, current) => {}
                                    _ => best_accept_rank = Some(rank.clone()),
                                }
                            }
                            retained_telescopes.push(PrefixGroupCandidate {
                                telescope: telescope.clone(),
                                accept_rank,
                                evaluated_candidate: None,
                            });
                            continue;
                        }
                    }
                }

                let telescope = load_terminal_clause_into_scratch(
                    &mut scratch_telescope,
                    prefix_len,
                    terminal_clause.clause,
                );
                if matches!(
                    connectivity_decision,
                    TerminalConnectivityDecision::NeedsFallback
                ) {
                    if !passes_connectivity(library, telescope) {
                        continue;
                    }
                }

                admitted_focus_aligned_count += 1;
                discovery.enumerated_candidates += 1;
                discovery.well_formed_candidates += 1;
                admitted_terminal_candidates += 1;
                let exact_nu = u16::try_from(single_clause_structural_nu_total(
                    telescope,
                    terminal_clause.nu_facts,
                    single_clause_nu_context.as_ref(),
                    library,
                    nu_history,
                ))
                .expect("nu exceeded u16");
                let bit_kappa_used =
                    terminal_prefix_completion_bit_cost(prefix_bit_cost, terminal_clause.clause);
                absorb_terminal_prefix_completion_bound(
                    &mut bound,
                    exact_nu,
                    clause_kappa_used,
                    bit_kappa_used,
                );

                let accept_rank = acceptance_rank_for_telescope(
                    objective_bar,
                    telescope,
                    exact_nu,
                    bit_kappa_used,
                    clause_kappa_used,
                );
                if let Some(rank) = accept_rank.as_ref() {
                    match &best_accept_rank {
                        Some(current) if !better_rank(rank, current) => {}
                        _ => best_accept_rank = Some(rank.clone()),
                    }
                }
                retained_telescopes.push(PrefixGroupCandidate {
                    telescope: telescope.clone(),
                    accept_rank,
                    evaluated_candidate: None,
                });
            }
            discovery.admissibility_diagnostics.record_repeated(
                AdmissibilityDecisionClass::AdmittedFocusAligned,
                CLAIM_OPEN_BAND_ADMISSIBILITY_REASON,
                admitted_focus_aligned_count,
            );
        }
    }

    discovery.raw_generated_surface += generated_terminal_candidates;

    Ok(MaterializedTerminalPrefixGroup {
        candidates: retained_telescopes,
        bound,
        best_accept_rank,
        generated_terminal_candidates,
        admissible_terminal_candidates: admitted_terminal_candidates,
    })
}

fn sort_terminal_prefix_group_candidates_for_certification(
    candidates: &mut [PrefixGroupCandidate],
) {
    candidates.sort_by_key(|candidate| {
        (
            candidate.accept_rank.is_none(),
            candidate.accept_rank.clone(),
            serde_json::to_string(&candidate.telescope)
                .expect("terminal group telescope should serialize"),
        )
    });
}

fn best_prefix_group_accept_rank(candidates: &[PrefixGroupCandidate]) -> Option<AcceptRank> {
    let mut best = None;
    for candidate in candidates {
        let Some(rank) = candidate.accept_rank.clone() else {
            continue;
        };
        match &best {
            Some(current) if !better_rank(&rank, current) => {}
            _ => best = Some(rank),
        }
    }
    best
}

fn terminal_prefix_rank_prune_count(
    best_accept_rank: Option<&AcceptRank>,
    best_accept_primary_rank: Option<&TerminalPrefixPrimaryRank>,
    admitted_candidate_count: usize,
    incumbent_rank: Option<&AcceptRank>,
) -> Option<usize> {
    let incumbent_rank = incumbent_rank?;
    if let Some(best_accept_rank) = best_accept_rank {
        return (!better_rank(best_accept_rank, incumbent_rank))
            .then_some(admitted_candidate_count);
    }

    let best_accept_primary_rank = best_accept_primary_rank?;
    (!terminal_prefix_primary_rank_could_still_beat_incumbent(
        best_accept_primary_rank,
        incumbent_rank,
    ))
    .then_some(admitted_candidate_count)
}

fn cached_terminal_prefix_rank_prune_count(
    prefix_signature: &PrefixSignature,
    incumbent_rank: Option<&AcceptRank>,
    prefix_legality_cache: &mut PrefixLegalityCache,
) -> Option<usize> {
    let incumbent_rank = incumbent_rank?;
    let rank_summary = prefix_legality_cache.terminal_prefix_rank_summary(prefix_signature)?;
    terminal_prefix_rank_prune_count(
        rank_summary.best_accept_rank.as_ref(),
        rank_summary.best_accept_primary_rank.as_ref(),
        rank_summary.admitted_candidate_count,
        Some(incumbent_rank),
    )
}

fn demo_proof_close_order_mode(
    remaining_reserve_millis: u64,
    pending_exact_surface: usize,
) -> DemoProofCloseOrderMode {
    let pending_exact_surface =
        u64::try_from(pending_exact_surface).expect("pending exact surface exceeded u64");
    // When the reserved slice drops below roughly one second per pending exact
    // candidate, prioritize fast closure/prune wins over broader incumbent hunting.
    if pending_exact_surface > 0
        && remaining_reserve_millis < pending_exact_surface.saturating_mul(1_000)
    {
        DemoProofCloseOrderMode::ClosureFirst
    } else {
        DemoProofCloseOrderMode::PotentialFirst
    }
}

fn demo_pending_closure_pressure(
    pending_signatures: &[PrefixSignature],
    prefix_cache: &PrefixCache,
    incumbent_rank: Option<&AcceptRank>,
) -> DemoClosurePressure {
    let mut pressure = DemoClosurePressure::default();
    for signature in pending_signatures {
        let Some(group) = prefix_cache.get(signature) else {
            continue;
        };
        pressure.pending_groups += 1;
        pressure.pending_exact_surface += group.candidates.len();
        if demo_group_is_prune_ready(group, incumbent_rank) {
            pressure.prune_ready_groups += 1;
            pressure.prune_ready_exact_surface += group.candidates.len();
        }
    }
    pressure
}

fn demo_proof_close_order_mode_with_closure_pressure(
    remaining_reserve_millis: u64,
    closure_pressure: DemoClosurePressure,
) -> DemoProofCloseOrderMode {
    if closure_pressure.prefers_closure() {
        DemoProofCloseOrderMode::ClosureFirst
    } else {
        demo_proof_close_order_mode(
            remaining_reserve_millis,
            closure_pressure.pending_exact_surface,
        )
    }
}

fn demo_group_can_improve_incumbent(
    group: &PrefixCandidateGroup,
    incumbent_rank: Option<&AcceptRank>,
) -> bool {
    match (group.best_accept_rank.as_ref(), incumbent_rank) {
        (Some(group_rank), Some(incumbent_rank)) => better_rank(group_rank, incumbent_rank),
        (Some(_), None) => true,
        (None, _) => false,
    }
}

fn demo_group_is_prune_ready(
    group: &PrefixCandidateGroup,
    incumbent_rank: Option<&AcceptRank>,
) -> bool {
    match (group.best_accept_rank.as_ref(), incumbent_rank) {
        (Some(group_rank), Some(incumbent_rank)) => !better_rank(group_rank, incumbent_rank),
        _ => false,
    }
}

#[derive(Clone, Debug, Default)]
struct DemoBucketSelectionContext {
    shape_counts: BTreeMap<(DemoBucketKey, u64), usize>,
    support_counts: BTreeMap<(DemoBucketKey, u64), usize>,
}

impl DemoBucketSelectionContext {
    fn from_pending(
        pending_signatures: &[PrefixSignature],
        prefix_cache: &PrefixCache,
        admissibility_mode: AdmissibilityMode,
    ) -> Self {
        let mut context = Self::default();
        for signature in pending_signatures {
            let Some(group) = prefix_cache.get(signature) else {
                continue;
            };
            let bucket_key = demo_bucket_key_for_group(admissibility_mode, signature, group);
            *context
                .shape_counts
                .entry((bucket_key.clone(), group.shape_hash64))
                .or_insert(0) += 1;
            *context
                .support_counts
                .entry((bucket_key, group.support_hash64))
                .or_insert(0) += 1;
        }
        context
    }

    fn redundancy_key(
        &self,
        bucket_key: &DemoBucketKey,
        group: &PrefixCandidateGroup,
    ) -> (usize, usize) {
        (
            self.shape_counts
                .get(&(bucket_key.clone(), group.shape_hash64))
                .copied()
                .unwrap_or(0),
            self.support_counts
                .get(&(bucket_key.clone(), group.support_hash64))
                .copied()
                .unwrap_or(0),
        )
    }
}

fn demo_bucket_progress_key(
    bucket_stats: &BTreeMap<DemoBucketKey, DemoBucketStats>,
    bucket_key: &DemoBucketKey,
) -> (usize, usize, usize) {
    let stats = bucket_stats.get(bucket_key).cloned().unwrap_or_default();
    (
        stats.fully_scored_terminal_candidates + stats.pruned_terminal_candidates,
        stats.fully_scored_terminal_candidates,
        stats.exact_screened_terminal_candidates,
    )
}

fn demo_proof_close_group_order(
    mode: DemoProofCloseOrderMode,
    admissibility_mode: AdmissibilityMode,
    left_signature: &PrefixSignature,
    left_group: &PrefixCandidateGroup,
    right_signature: &PrefixSignature,
    right_group: &PrefixCandidateGroup,
    bucket_stats: &BTreeMap<DemoBucketKey, DemoBucketStats>,
    selection_context: &DemoBucketSelectionContext,
    incumbent_rank: Option<&AcceptRank>,
) -> Ordering {
    let left_improves = demo_group_can_improve_incumbent(left_group, incumbent_rank);
    let right_improves = demo_group_can_improve_incumbent(right_group, incumbent_rank);
    let left_prune_ready = demo_group_is_prune_ready(left_group, incumbent_rank);
    let right_prune_ready = demo_group_is_prune_ready(right_group, incumbent_rank);
    let left_bucket_key = demo_bucket_key_for_group(admissibility_mode, left_signature, left_group);
    let right_bucket_key =
        demo_bucket_key_for_group(admissibility_mode, right_signature, right_group);
    let left_progress = demo_bucket_progress_key(bucket_stats, &left_bucket_key);
    let right_progress = demo_bucket_progress_key(bucket_stats, &right_bucket_key);
    let left_redundancy = selection_context.redundancy_key(&left_bucket_key, left_group);
    let right_redundancy = selection_context.redundancy_key(&right_bucket_key, right_group);

    match mode {
        DemoProofCloseOrderMode::PotentialFirst => (
            !left_improves,
            left_group.best_accept_rank.is_none(),
            left_group.retention_class.priority_rank(),
            left_progress,
            left_redundancy,
            left_group.candidates.len(),
            left_group.best_accept_rank.clone(),
            std::cmp::Reverse(left_group.bound.rho_upper().unwrap_or(Rational::zero())),
            left_signature.clone(),
        )
            .cmp(&(
                !right_improves,
                right_group.best_accept_rank.is_none(),
                right_group.retention_class.priority_rank(),
                right_progress,
                right_redundancy,
                right_group.candidates.len(),
                right_group.best_accept_rank.clone(),
                std::cmp::Reverse(right_group.bound.rho_upper().unwrap_or(Rational::zero())),
                right_signature.clone(),
            )),
        DemoProofCloseOrderMode::ClosureFirst => (
            !left_prune_ready,
            left_group.candidates.len(),
            left_group.retention_class.priority_rank(),
            left_progress,
            left_redundancy,
            !left_improves,
            left_group.best_accept_rank.is_none(),
            left_group.best_accept_rank.clone(),
            std::cmp::Reverse(left_group.bound.rho_upper().unwrap_or(Rational::zero())),
            left_signature.clone(),
        )
            .cmp(&(
                !right_prune_ready,
                right_group.candidates.len(),
                right_group.retention_class.priority_rank(),
                right_progress,
                right_redundancy,
                !right_improves,
                right_group.best_accept_rank.is_none(),
                right_group.best_accept_rank.clone(),
                std::cmp::Reverse(right_group.bound.rho_upper().unwrap_or(Rational::zero())),
                right_signature.clone(),
            )),
    }
}

fn demo_materialize_to_proof_close_handoff_reason_for_pressure(
    budget: DemoStepBudget,
    closure_pressure: DemoClosurePressure,
    incumbent_rank: Option<&AcceptRank>,
    materialize_full_evals: usize,
) -> Option<DemoProofCloseEntryReason> {
    if budget.proof_close_reserve_millis == 0
        || closure_pressure.pending_exact_surface == 0
        || incumbent_rank.is_none()
        || materialize_full_evals == 0
    {
        return None;
    }

    if demo_proof_close_order_mode(
        budget.proof_close_reserve_millis,
        closure_pressure.pending_exact_surface,
    ) == DemoProofCloseOrderMode::ClosureFirst
    {
        return Some(DemoProofCloseEntryReason::MaterializeReserveHandoff);
    }

    closure_pressure
        .prefers_closure()
        .then_some(DemoProofCloseEntryReason::ClosurePressureHandoff)
}

fn demo_materialize_to_proof_close_handoff_reason(
    budget: DemoStepBudget,
    pending_signatures: &[PrefixSignature],
    prefix_cache: &PrefixCache,
    incumbent_rank: Option<&AcceptRank>,
    materialize_full_evals: usize,
) -> Option<DemoProofCloseEntryReason> {
    demo_materialize_to_proof_close_handoff_reason_for_pressure(
        budget,
        demo_pending_closure_pressure(pending_signatures, prefix_cache, incumbent_rank),
        incumbent_rank,
        materialize_full_evals,
    )
}

fn select_demo_proof_close_group_index(
    pending_signatures: &[PrefixSignature],
    prefix_cache: &PrefixCache,
    admissibility_mode: AdmissibilityMode,
    bucket_stats: &BTreeMap<DemoBucketKey, DemoBucketStats>,
    incumbent_rank: Option<&AcceptRank>,
    remaining_reserve_millis: u64,
) -> Option<usize> {
    let closure_pressure =
        demo_pending_closure_pressure(pending_signatures, prefix_cache, incumbent_rank);
    let order_mode = demo_proof_close_order_mode_with_closure_pressure(
        remaining_reserve_millis,
        closure_pressure,
    );
    let selection_context = DemoBucketSelectionContext::from_pending(
        pending_signatures,
        prefix_cache,
        admissibility_mode,
    );

    pending_signatures
        .iter()
        .enumerate()
        .min_by(|(_, left_signature), (_, right_signature)| {
            match (
                prefix_cache.get(left_signature),
                prefix_cache.get(right_signature),
            ) {
                (Some(left_group), Some(right_group)) => demo_proof_close_group_order(
                    order_mode,
                    admissibility_mode,
                    left_signature,
                    left_group,
                    right_signature,
                    right_group,
                    bucket_stats,
                    &selection_context,
                    incumbent_rank,
                ),
                (None, Some(_)) => Ordering::Less,
                (Some(_), None) => Ordering::Greater,
                (None, None) => left_signature.cmp(right_signature),
            }
        })
        .map(|(index, _)| index)
}

fn cache_evaluated_terminal_prefix_group_candidates(
    step_index: u32,
    objective_bar: Rational,
    library: &Library,
    history: &[DiscoveryRecord],
    admissibility: StrictAdmissibility,
    nu_history: &[(u32, u32)],
    candidates: &mut [PrefixGroupCandidate],
    discovery: &mut RealisticShadowDiscovery,
) -> Result<()> {
    let sort_started = Instant::now();
    sort_terminal_prefix_group_candidates_for_certification(candidates);
    discovery.remaining_one_telemetry.candidate_sort_millis = discovery
        .remaining_one_telemetry
        .candidate_sort_millis
        .saturating_add(elapsed_millis(sort_started.elapsed()));
    let compact_claim_mode = should_compact_terminal_prefix_group_candidates(admissibility.mode);
    let eval_started = Instant::now();

    for candidate in candidates.iter_mut() {
        if compact_claim_mode
            && candidate.evaluated_candidate.is_none()
            && !claim_candidate_needs_discovery_evaluation(
                candidate.accept_rank.as_ref(),
                discovery.terminal_rank_incumbent.as_ref(),
            )
        {
            continue;
        }

        let evaluated = match candidate.evaluated_candidate.clone() {
            Some(evaluated) => evaluated,
            None => evaluate_checked_candidate(library, history, candidate.telescope.clone())?,
        };
        let candidate_rank = candidate
            .accept_rank
            .clone()
            .or_else(|| acceptance_rank(objective_bar, &evaluated));
        if let Some(rank) = candidate_rank.clone() {
            let witness = analyze_semantic_minimality(
                step_index,
                objective_bar,
                admissibility,
                &evaluated.telescope,
                library,
                nu_history,
            );
            let better_than_incumbent = discovery
                .terminal_rank_incumbent
                .as_ref()
                .map(|current| better_rank(&rank, current))
                .unwrap_or(true);
            if witness.is_minimal() && better_than_incumbent {
                discovery.terminal_rank_incumbent = Some(rank);
            }
        }
        candidate.accept_rank = candidate_rank;
        candidate.evaluated_candidate = Some(evaluated);
    }
    if should_compact_terminal_prefix_group_candidates(admissibility.mode) {
        // Claim runs prefer bounded live memory over caching every evaluated
        // terminal payload inside the prefix cache.
        compact_terminal_prefix_group_candidates(candidates);
    }

    discovery
        .remaining_one_telemetry
        .candidate_eval_minimality_millis = discovery
        .remaining_one_telemetry
        .candidate_eval_minimality_millis
        .saturating_add(elapsed_millis(eval_started.elapsed()));

    Ok(())
}

fn claim_candidate_needs_discovery_evaluation(
    candidate_rank: Option<&AcceptRank>,
    incumbent_rank: Option<&AcceptRank>,
) -> bool {
    match (candidate_rank, incumbent_rank) {
        (Some(candidate_rank), Some(incumbent_rank)) => better_rank(candidate_rank, incumbent_rank),
        (Some(_), None) => true,
        (None, _) => false,
    }
}

fn should_compact_terminal_prefix_group_candidates(mode: AdmissibilityMode) -> bool {
    matches!(mode, AdmissibilityMode::DesktopClaimShadow)
}

fn should_release_processed_prefix_groups(mode: AdmissibilityMode) -> bool {
    matches!(mode, AdmissibilityMode::DesktopClaimShadow)
}

fn load_terminal_prefix_group_for_proof_close(
    prefix_cache: &mut PrefixCache,
    signature: &PrefixSignature,
    mode: AdmissibilityMode,
) -> Option<PrefixCandidateGroup> {
    if should_release_processed_prefix_groups(mode) {
        // Claim proof-close can free a prefix group once certification starts.
        prefix_cache.take(signature)
    } else {
        prefix_cache.get(signature).cloned()
    }
}

fn compact_terminal_prefix_group_candidates(candidates: &mut [PrefixGroupCandidate]) {
    for candidate in candidates {
        candidate.evaluated_candidate = None;
    }
}

fn pop_best_prefix(frontier: &mut Vec<OnlinePrefixWorkItem>) -> Option<OnlinePrefixWorkItem> {
    if frontier.is_empty() {
        return None;
    }

    frontier.sort_by(|left, right| {
        prefix_frontier_work_key(left).cmp(&prefix_frontier_work_key(right))
    });
    Some(frontier.remove(0))
}

fn prefix_frontier_work_key(item: &OnlinePrefixWorkItem) -> (usize, usize, u8, u32, usize, &str) {
    (
        item.remaining_clause_slots,
        item.next_clause_count,
        item.remaining_family_options,
        item.bit_cost,
        item.clause_count,
        item.order_key.as_ref(),
    )
}

fn build_prefix_frontier_plan(
    explored: usize,
    step_index: u32,
    objective_bar: Rational,
    retention_policy: RetentionPolicy,
    retention_runtime: FrontierRuntimeLimits,
    prefix_cache: &PrefixCache,
) -> PrefixFrontierPlan {
    if prefix_cache.is_empty() {
        return PrefixFrontierPlan {
            explored,
            ..PrefixFrontierPlan::default()
        };
    }

    let mut items = prefix_cache
        .iter()
        .enumerate()
        .map(|(ordinal, (signature, group))| PrefixFrontierItem {
            signature: signature.clone(),
            dedupe_key: signature.dedupe_key(),
            retention_class: group.retention_class,
            record: prefix_frontier_record(
                step_index,
                u64::try_from(ordinal).expect("ordinal exceeded u64"),
                group,
                false,
            ),
        })
        .collect::<Vec<_>>();

    let mut provisional = FrontierWindow {
        hot: items.iter().map(|item| item.record).collect(),
        cold: Vec::new(),
    };
    provisional.compact_sorted();
    let schedule = build_schedule(&provisional, retention_runtime.worker_count);
    let mut exact_pruned_state_ids = BTreeSet::new();
    let mut retained_state_ids = BTreeSet::new();
    for assignment in &schedule.assignments {
        let result = run_worker_batch(assignment, objective_bar);
        retained_state_ids.extend(result.processed_state_ids);
        for record in &assignment.records {
            if !retained_state_ids.contains(&record.state_id) {
                exact_pruned_state_ids.insert(record.state_id);
            }
        }
    }

    items.retain(|item| retained_state_ids.contains(&item.record.state_id));
    if items.is_empty() {
        return PrefixFrontierPlan {
            frontier: FrontierWindow::default(),
            dedupe_keys: BTreeSet::new(),
            retained_prefix_signatures: Vec::new(),
            explored,
            exact_pruned: exact_pruned_state_ids.len(),
            heuristic_dropped: 0,
            pressure: FrontierPressure::default(),
        };
    }

    let mut ordered = FrontierWindow {
        hot: items.iter().map(|item| item.record).collect(),
        cold: Vec::new(),
    };
    ordered.compact_sorted();
    let items_by_state = items
        .into_iter()
        .map(|item| (item.record.state_id, item))
        .collect::<BTreeMap<_, _>>();
    let mut class_counts = BTreeMap::new();
    let mut hot_items = Vec::new();
    let mut cold_candidates = Vec::new();
    for record in ordered.hot {
        let item = items_by_state
            .get(&record.state_id)
            .expect("state lookup")
            .clone();
        let count = class_counts.entry(item.retention_class).or_insert(0usize);
        if *count < retention_policy.quota_for(item.retention_class) {
            *count += 1;
            hot_items.push(item);
        } else {
            cold_candidates.push(item);
        }
    }
    if hot_items.is_empty() && !cold_candidates.is_empty() {
        hot_items.push(cold_candidates.remove(0));
    }

    let cold_plan = plan_pressure_cold_retention(
        cold_candidates,
        hot_items.len(),
        retention_policy,
        retention_runtime,
        |item: &PrefixFrontierItem| item.retention_class,
    );
    let heuristic_dropped = cold_plan.dropped.len();
    let pressure = cold_plan.pressure;
    let resident = cold_plan.resident;
    let spill = cold_plan.spill;

    let mut frontier = FrontierWindow::default();
    let mut keys_by_state = BTreeMap::new();
    for item in hot_items {
        let mut record = item.record;
        record.flags = prefix_frontier_flags(item.retention_class, true);
        keys_by_state.insert(record.state_id, (item.signature, item.dedupe_key));
        frontier.push_hot(record);
    }
    for item in resident.into_iter().chain(spill.into_iter()) {
        let mut record = item.record;
        record.flags = prefix_frontier_flags(item.retention_class, false);
        keys_by_state.insert(record.state_id, (item.signature, item.dedupe_key));
        frontier.push_cold(record);
    }
    frontier.compact_sorted();

    let retained_prefix_signatures = frontier
        .hot
        .iter()
        .chain(frontier.cold.iter())
        .filter_map(|record| {
            keys_by_state
                .get(&record.state_id)
                .map(|(signature, _)| signature.clone())
        })
        .collect::<Vec<_>>();
    let dedupe_keys = frontier
        .hot
        .iter()
        .chain(frontier.cold.iter())
        .filter_map(|record| {
            keys_by_state
                .get(&record.state_id)
                .map(|(_, dedupe_key)| dedupe_key.clone())
        })
        .collect::<BTreeSet<_>>();

    PrefixFrontierPlan {
        frontier,
        dedupe_keys,
        retained_prefix_signatures,
        explored,
        exact_pruned: exact_pruned_state_ids.len(),
        heuristic_dropped,
        pressure,
    }
}

fn prefix_frontier_record(
    step_index: u32,
    ordinal: u64,
    group: &PrefixCandidateGroup,
    frontier_hot: bool,
) -> FrontierStateRecV1 {
    let state_id = StateId::new((u64::from(step_index) << 32) | ordinal);
    let bound = group.bound;
    let band_index = u8::try_from(bound.clause_kappa_used).expect("band index exceeded u8");
    let priority_key = build_priority_key(PriorityInputs {
        band_index,
        nu_lower_bound: bound.nu_lower_bound,
        bit_kappa_used: bound.bit_kappa_used,
        clause_kappa_used: bound.clause_kappa_used,
        depth: group.depth,
        state_id,
    });
    let prefix = PrefixState {
        state_id,
        parent_state_id: StateId::new(0),
        last_clause_id: ClauseId::new(
            u32::try_from(group.prefix_telescope.clauses.len()).expect("clause count exceeded u32"),
        ),
        obligation_set_id: ObligationSetId::new(step_index),
        shape_hash64: group.shape_hash64,
        support_hash64: group.support_hash64,
        nu_lower_bound: bound.nu_lower_bound,
        nu_upper_bound: bound.nu_upper_bound,
        bit_kappa_used: bound.bit_kappa_used,
        clause_kappa_used: bound.clause_kappa_used,
        depth: group.depth,
        step_index: u8::try_from(step_index).expect("step index exceeded u8"),
        band_index,
        flags: prefix_frontier_flags(group.retention_class, frontier_hot),
    };

    FrontierStateRecV1::from_prefix(prefix, priority_key, 0)
}

fn prefix_frontier_flags(retention_class: RetentionClass, frontier_hot: bool) -> u16 {
    let retention_bits = if frontier_hot { 0b1 << 8 } else { 0b1 << 9 };
    let class_bits = match retention_class {
        RetentionClass::GenericMacro => 0,
        RetentionClass::StructuralSupport => 0b01 << 10,
        RetentionClass::RareBridgeHead => 0b10 << 10,
        RetentionClass::RareFocusHead => 0b11 << 10,
    };
    retention_bits | class_bits
}

fn candidate_score_distribution(
    candidates: &[ExpandedCandidate],
    objective_bar: Rational,
) -> CandidateScoreDistribution {
    if candidates.is_empty() {
        return CandidateScoreDistribution::default();
    }

    let mut clause_kappa_counts = BTreeMap::new();
    let mut nu_values = Vec::with_capacity(candidates.len());
    let mut rho_values = Vec::with_capacity(candidates.len());
    let mut nu_sum = 0_i64;
    let mut rho_sum = Rational::zero();
    let mut clears_bar = 0usize;

    for candidate in candidates {
        *clause_kappa_counts
            .entry(candidate.clause_kappa)
            .or_insert(0) += 1;
        nu_values.push(candidate.nu);
        rho_values.push(candidate.rho);
        nu_sum += i64::from(candidate.nu);
        rho_sum = rho_sum + candidate.rho;
        if candidate.rho >= objective_bar {
            clears_bar += 1;
        }
    }

    nu_values.sort_unstable();
    rho_values.sort();
    let candidate_count = candidates.len();

    CandidateScoreDistribution {
        candidate_count,
        clears_bar,
        below_bar: candidate_count.saturating_sub(clears_bar),
        clause_kappa_counts,
        nu_summary: IntegerDistributionSummary {
            min: *nu_values.first().expect("candidate count checked above"),
            median: nu_values[candidate_count / 2],
            max: *nu_values.last().expect("candidate count checked above"),
            average: Rational::new(
                nu_sum,
                i64::try_from(candidate_count).expect("candidate count exceeded i64"),
            ),
        },
        rho_summary: RationalDistributionSummary {
            min: *rho_values.first().expect("candidate count checked above"),
            median: rho_values[candidate_count / 2],
            max: *rho_values.last().expect("candidate count checked above"),
            average: rho_sum
                / Rational::from_integer(
                    i64::try_from(candidate_count).expect("candidate count exceeded i64"),
                ),
        },
    }
}

fn elapsed_millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

fn elapsed_micros(duration: Duration) -> u64 {
    u64::try_from(duration.as_micros()).unwrap_or(u64::MAX)
}

fn absorb_elapsed_duration(millis: &mut u64, micros: &mut u64, duration: Duration) {
    *micros = micros.saturating_add(elapsed_micros(duration));
    *millis = if *micros == u64::MAX {
        u64::MAX
    } else {
        *micros / 1_000
    };
}

#[cfg(test)]
mod tests {
    use super::{
        AtomicSearchStep, DemoBreadthHarvestExitReason, DemoBucketSelectionContext,
        DemoBudgetController, DemoBudgetFeedback, DemoBudgetRetuneAction, DemoBudgetSeed,
        DemoClosurePressure, DemoNarrativeRuntime, DemoProofCloseEntryReason,
        DemoProofCloseOrderMode, DemoProofCloseOverrunReason, DemoStepBudget,
        LIVE_BOOTSTRAP_MAX_STEP, OnlinePrefixWorkItem, SearchBucketTaxonomy,
        create_online_prefix_work_item,
        demo_materialize_to_proof_close_handoff_reason_for_pressure, demo_proof_close_group_order,
        demo_proof_close_order_mode, demo_proof_close_order_mode_with_closure_pressure,
        discovery_enumeration_context, exact_partial_prefix_bound_decision,
        maybe_retune_demo_budget_live, pop_best_prefix, search_bootstrap_from_prefix,
        search_bootstrap_from_prefix_for_profile_with_runtime, search_bootstrap_prefix,
        search_bootstrap_prefix_for_config_with_runtime,
        sort_terminal_prefix_group_candidates_for_certification, supports_live_atomic_search,
        terminal_prefix_clause_candidates,
    };
    use crate::bounds::PrefixBound;
    use crate::branch_bound::AcceptRank;
    use crate::config::{RuntimeConfig, SearchProfile};
    use crate::enumerate::{EnumerationContext, LateFamilySurface, build_clause_catalog};
    use crate::expand::{evaluate_candidate, evaluate_checked_candidate};
    use crate::narrative::{NarrativeEventKind, StepPhase, narrative_progress_snapshot};
    use crate::prefix_cache::{
        PrefixCache, PrefixCandidateGroup, PrefixGroupCandidate, PrefixSignature,
    };
    use crate::prefix_memo::{
        PartialPrefixBoundDecision, PrefixLegalityCache, TerminalPrefixClauseEvaluation,
        TerminalPrefixCompletion,
    };
    use pen_core::{
        canonical::CanonKey,
        clause::{ClauseRec, ClauseRole},
        expr::Expr,
        library::{Library, LibraryEntry},
        rational::Rational,
        telescope::{Telescope, TelescopeClass},
    };
    use pen_eval::{
        bar::{DiscoveryRecord, compute_bar},
        minimality::analyze_semantic_minimality,
        nu::{TerminalClauseNuFacts, structural_nu},
    };
    use pen_type::{
        admissibility::{
            AdmissibilityDecision, AdmissibilityDecisionClass, AdmissibilityMode, PackagePolicies,
            PackagePolicy, StrictAdmissibility, StructuralFamily, assess_strict_admissibility,
            passes_strict_admissibility, strict_admissibility, strict_admissibility_for_mode,
        },
        connectivity::{ConnectivityWitness, analyze_connectivity, passes_connectivity},
        obligations::{RetentionClass, RetentionFocus, RetentionPolicy},
    };
    use std::cmp::Reverse;
    use std::collections::BTreeMap;
    use std::sync::Arc;
    use std::time::Duration;

    fn reference_prefix(until_step: u32) -> Vec<Telescope> {
        (1..=until_step).map(Telescope::reference).collect()
    }

    fn reference_library(until_step: u32) -> Library {
        let mut library = Vec::new();
        for telescope in reference_prefix(until_step) {
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }
        library
    }

    fn library_until(until_step: u32) -> Library {
        let mut library = Vec::new();
        for step in 1..=until_step {
            let telescope = Telescope::reference(step);
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }
        library
    }

    fn reference_history_until(
        until_step: u32,
    ) -> (Library, Vec<DiscoveryRecord>, Vec<(u32, u32)>) {
        let mut library = Vec::new();
        let mut history = Vec::new();
        let mut nu_history = Vec::new();

        for step in 1..=until_step {
            let telescope = Telescope::reference(step);
            let nu = structural_nu(&telescope, &library, &nu_history).total;
            history.push(DiscoveryRecord::new(
                step,
                nu,
                u32::try_from(telescope.kappa()).expect("kappa exceeded u32"),
            ));
            nu_history.push((step, nu));
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }

        (library, history, nu_history)
    }

    fn demo_runtime_config_10m() -> RuntimeConfig {
        RuntimeConfig::from_toml_str(include_str!(
            "../../../configs/demo_breadth_shadow_10m.toml"
        ))
        .expect("demo config should parse")
    }

    fn dummy_work_item(
        remaining_clause_slots: usize,
        next_clause_count: usize,
        remaining_family_options: u8,
        bit_cost: u32,
        clause_count: usize,
        order_key: &str,
    ) -> OnlinePrefixWorkItem {
        let prefix_telescope =
            Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Univ)]);
        let signature = PrefixSignature::new(1, &Library::default(), &prefix_telescope);
        let filtered_next_clauses =
            vec![ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)); next_clause_count];
        let filtered_next_clause_nu_facts = filtered_next_clauses
            .iter()
            .map(TerminalClauseNuFacts::from_clause)
            .collect();
        OnlinePrefixWorkItem {
            clause_kappa: 4,
            prefix_telescope,
            signature,
            remaining_clause_slots,
            remaining_family_options,
            bit_cost,
            clause_count,
            filtered_next_clauses: Some(filtered_next_clauses),
            filtered_next_clause_connectivity_facts: None,
            filtered_next_clause_nu_facts: Some(filtered_next_clause_nu_facts),
            next_clause_count,
            order_key: Arc::<str>::from(order_key),
        }
    }

    fn test_accept_rank(overshoot_numerator: i64, canonical_key: &str) -> AcceptRank {
        AcceptRank {
            overshoot: Rational::new(overshoot_numerator, 10),
            clause_kappa: 4,
            descending_eliminator_score: Reverse(2),
            descending_former_score: Reverse(2),
            descending_dependent_motive_density: Reverse(1),
            descending_library_reference_density: Reverse(1),
            max_var_ref: 2,
            descending_generic_binder_count: Reverse(2),
            descending_closure_score: Reverse(2),
            bit_kappa: 20,
            descending_nu: Reverse(10),
            canonical_key: CanonKey(canonical_key.to_owned()),
        }
    }

    fn test_prefix_signature(seed: u32) -> PrefixSignature {
        let prefix = Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Var(seed))]);
        PrefixSignature::new(1, &Library::default(), &prefix)
    }

    fn test_prefix_group(
        candidate_count: usize,
        best_accept_rank: Option<AcceptRank>,
        nu_upper_bound: u16,
    ) -> PrefixCandidateGroup {
        let prefix_telescope =
            Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Var(1))]);
        let mut candidates = Vec::new();
        if let Some(rank) = best_accept_rank.clone() {
            candidates.push(PrefixGroupCandidate {
                telescope: Telescope::new(vec![ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Var(11),
                )]),
                accept_rank: Some(rank),
                evaluated_candidate: None,
            });
        }
        while candidates.len() < candidate_count {
            let seed = u32::try_from(candidates.len()).expect("candidate index exceeded u32");
            candidates.push(PrefixGroupCandidate {
                telescope: Telescope::new(vec![ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Var(seed + 20),
                )]),
                accept_rank: None,
                evaluated_candidate: None,
            });
        }
        PrefixCandidateGroup {
            prefix_telescope,
            retention_class: RetentionClass::GenericMacro,
            shape_hash64: 0,
            support_hash64: 0,
            depth: 1,
            bound: PrefixBound {
                nu_lower_bound: 1,
                nu_upper_bound,
                clause_kappa_used: 1,
                bit_kappa_used: 1,
            },
            best_accept_rank,
            candidates,
        }
    }

    #[test]
    fn demo_proof_close_order_mode_switches_under_tight_reserve() {
        assert_eq!(
            demo_proof_close_order_mode(2_500, 2),
            DemoProofCloseOrderMode::PotentialFirst
        );
        assert_eq!(
            demo_proof_close_order_mode(1_500, 2),
            DemoProofCloseOrderMode::ClosureFirst
        );
    }

    #[test]
    fn demo_proof_close_order_mode_switches_when_closure_pressure_dominates() {
        assert_eq!(
            demo_proof_close_order_mode_with_closure_pressure(
                20_000,
                DemoClosurePressure {
                    pending_exact_surface: 4,
                    prune_ready_exact_surface: 2,
                    pending_groups: 3,
                    prune_ready_groups: 2,
                },
            ),
            DemoProofCloseOrderMode::ClosureFirst
        );
    }

    #[test]
    fn demo_proof_close_potential_mode_prefers_incumbent_improvers() {
        let incumbent = test_accept_rank(2, "incumbent");
        let prune_ready_group = test_prefix_group(1, Some(test_accept_rank(4, "prune")), 4);
        let improving_group = test_prefix_group(3, Some(test_accept_rank(1, "improve")), 6);
        let prune_ready_signature = test_prefix_signature(1);
        let improving_signature = test_prefix_signature(2);
        let bucket_stats = BTreeMap::new();
        let selection_context = DemoBucketSelectionContext::default();

        assert_eq!(
            demo_proof_close_group_order(
                DemoProofCloseOrderMode::PotentialFirst,
                AdmissibilityMode::RealisticShadow,
                &prune_ready_signature,
                &prune_ready_group,
                &improving_signature,
                &improving_group,
                &bucket_stats,
                &selection_context,
                Some(&incumbent),
            ),
            std::cmp::Ordering::Greater
        );
    }

    #[test]
    fn claim_lane_drops_cached_terminal_payloads_after_ranking() {
        let mut candidates = vec![PrefixGroupCandidate {
            telescope: Telescope::reference(2),
            accept_rank: Some(test_accept_rank(1, "claim")),
            evaluated_candidate: Some(
                evaluate_checked_candidate(
                    &reference_library(1),
                    &[DiscoveryRecord::new(1, 2, 2)],
                    Telescope::reference(2),
                )
                .expect("reference candidate should evaluate"),
            ),
        }];

        if super::should_compact_terminal_prefix_group_candidates(
            AdmissibilityMode::DesktopClaimShadow,
        ) {
            super::compact_terminal_prefix_group_candidates(&mut candidates);
        }
        assert!(candidates[0].accept_rank.is_some());
        assert!(candidates[0].evaluated_candidate.is_none());

        let mut realistic_candidates = vec![PrefixGroupCandidate {
            telescope: Telescope::reference(2),
            accept_rank: Some(test_accept_rank(1, "realistic")),
            evaluated_candidate: Some(
                evaluate_checked_candidate(
                    &reference_library(1),
                    &[DiscoveryRecord::new(1, 2, 2)],
                    Telescope::reference(2),
                )
                .expect("reference candidate should evaluate"),
            ),
        }];
        if super::should_compact_terminal_prefix_group_candidates(
            AdmissibilityMode::RealisticShadow,
        ) {
            super::compact_terminal_prefix_group_candidates(&mut realistic_candidates);
        }
        assert!(realistic_candidates[0].evaluated_candidate.is_some());
    }

    #[test]
    fn claim_lane_releases_processed_prefix_groups_during_proof_close() {
        let mut cache = PrefixCache::default();
        let prefix = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ]);
        let history = vec![DiscoveryRecord::new(1, 2, 2)];
        let policy = RetentionPolicy {
            focus: RetentionFocus::Former,
            focus_quota: 1,
            bridge_quota: 1,
            support_quota: 1,
            macro_quota: 1,
            cold_limit: 4,
        };

        cache
            .record_group_with_bound(
                2,
                prefix.clone(),
                vec![PrefixGroupCandidate {
                    telescope: Telescope::new(vec![
                        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
                        ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
                        ClauseRec::new(ClauseRole::Introduction, Expr::PathCon(1)),
                    ]),
                    accept_rank: Some(test_accept_rank(1, "claim")),
                    evaluated_candidate: None,
                }],
                PrefixBound::singleton(5, 3, 12),
                &Library::default(),
                &history,
                policy,
            )
            .expect("group should record");

        let signature = cache
            .iter()
            .next()
            .expect("cache should contain a group")
            .0
            .clone();
        let group = super::load_terminal_prefix_group_for_proof_close(
            &mut cache,
            &signature,
            AdmissibilityMode::DesktopClaimShadow,
        )
        .expect("claim proof-close should take the resident group");

        assert_eq!(group.prefix_telescope, prefix);
        assert_eq!(group.candidates.len(), 1);
        assert!(cache.is_empty());
    }

    #[test]
    fn claim_candidate_discovery_evaluation_only_runs_for_incumbent_improvers() {
        let incumbent = test_accept_rank(2, "incumbent");
        let better = test_accept_rank(1, "better");
        let worse = test_accept_rank(3, "worse");

        assert!(super::claim_candidate_needs_discovery_evaluation(
            Some(&better),
            Some(&incumbent),
        ));
        assert!(!super::claim_candidate_needs_discovery_evaluation(
            Some(&worse),
            Some(&incumbent),
        ));
        assert!(!super::claim_candidate_needs_discovery_evaluation(
            None,
            Some(&incumbent),
        ));
        assert!(super::claim_candidate_needs_discovery_evaluation(
            Some(&better),
            None,
        ));
        assert!(!super::claim_candidate_needs_discovery_evaluation(
            None, None
        ));
    }

    #[test]
    fn demo_proof_close_closure_mode_prefers_fast_prunes() {
        let incumbent = test_accept_rank(2, "incumbent");
        let prune_ready_group = test_prefix_group(1, Some(test_accept_rank(4, "prune")), 4);
        let improving_group = test_prefix_group(3, Some(test_accept_rank(1, "improve")), 6);
        let prune_ready_signature = test_prefix_signature(3);
        let improving_signature = test_prefix_signature(4);
        let bucket_stats = BTreeMap::new();
        let selection_context = DemoBucketSelectionContext::default();

        assert_eq!(
            demo_proof_close_group_order(
                DemoProofCloseOrderMode::ClosureFirst,
                AdmissibilityMode::RealisticShadow,
                &prune_ready_signature,
                &prune_ready_group,
                &improving_signature,
                &improving_group,
                &bucket_stats,
                &selection_context,
                Some(&incumbent),
            ),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn demo_proof_close_potential_mode_prefers_bridge_potential_when_survivability_ties() {
        let mut bridge_group = test_prefix_group(2, Some(test_accept_rank(1, "bridge")), 6);
        bridge_group.retention_class = RetentionClass::RareBridgeHead;
        let generic_group = test_prefix_group(2, Some(test_accept_rank(1, "generic")), 6);
        let bridge_signature = test_prefix_signature(5);
        let generic_signature = test_prefix_signature(6);
        let bucket_stats = BTreeMap::new();
        let selection_context = DemoBucketSelectionContext::default();

        assert_eq!(
            demo_proof_close_group_order(
                DemoProofCloseOrderMode::PotentialFirst,
                AdmissibilityMode::RealisticShadow,
                &bridge_signature,
                &bridge_group,
                &generic_signature,
                &generic_group,
                &bucket_stats,
                &selection_context,
                None,
            ),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn demo_materialize_handoff_can_switch_on_from_closure_pressure_before_reserve_tightens() {
        let incumbent = test_accept_rank(2, "incumbent");
        assert_eq!(
            demo_materialize_to_proof_close_handoff_reason_for_pressure(
                DemoStepBudget {
                    proof_close_reserve_millis: 10_000,
                    ..DemoStepBudget::default()
                },
                DemoClosurePressure {
                    pending_exact_surface: 4,
                    prune_ready_exact_surface: 2,
                    pending_groups: 3,
                    prune_ready_groups: 2,
                },
                Some(&incumbent),
                1,
            ),
            Some(DemoProofCloseEntryReason::ClosurePressureHandoff)
        );
    }

    #[test]
    fn certification_sort_prefers_best_accept_rank_before_tiebreaks() {
        let best_rank = test_accept_rank(1, "best");
        let worse_rank = test_accept_rank(4, "worse");
        let best_telescope =
            Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Var(9))]);
        let worse_telescope =
            Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Var(3))]);
        let mut candidates = vec![
            PrefixGroupCandidate {
                telescope: worse_telescope,
                accept_rank: Some(worse_rank),
                evaluated_candidate: None,
            },
            PrefixGroupCandidate {
                telescope: best_telescope.clone(),
                accept_rank: Some(best_rank.clone()),
                evaluated_candidate: None,
            },
            PrefixGroupCandidate {
                telescope: Telescope::new(vec![ClauseRec::new(
                    ClauseRole::Formation,
                    Expr::Var(12),
                )]),
                accept_rank: None,
                evaluated_candidate: None,
            },
        ];

        sort_terminal_prefix_group_candidates_for_certification(&mut candidates);

        assert_eq!(candidates[0].accept_rank, Some(best_rank));
        assert_eq!(candidates[0].telescope, best_telescope);
        assert!(candidates[2].accept_rank.is_none());
    }

    #[test]
    fn demo_budget_controller_shares_early_window_and_late_spill() {
        let config = demo_runtime_config_10m();
        let mut controller =
            DemoBudgetController::maybe_new(&config, 15, DemoBudgetSeed::default())
                .expect("demo budget controller should build")
                .expect("demo profile should enable the budget controller");

        let step_one = controller.plan_step(1);
        assert_eq!(step_one.total_budget_millis, 90_000);
        assert_eq!(step_one.discovery_budget_millis, 90_000);
        assert_eq!(step_one.scout_budget_millis, 9_000);
        assert_eq!(step_one.breadth_harvest_budget_millis, 81_000);
        assert_eq!(step_one.shared_early_window_remaining_millis, 90_000);

        controller.record_step(1, 30_000);
        let step_two = controller.plan_step(2);
        assert_eq!(step_two.total_budget_millis, 60_000);
        assert_eq!(step_two.discovery_budget_millis, 60_000);
        assert_eq!(step_two.scout_budget_millis, 6_000);
        assert_eq!(step_two.breadth_harvest_budget_millis, 54_000);
        assert_eq!(step_two.shared_early_window_remaining_millis, 60_000);

        controller.record_step(2, 10_000);
        controller.record_step(3, 10_000);
        controller.record_step(4, 10_000);
        for (step_index, step_budget_millis) in [
            (5, 12_000),
            (6, 14_000),
            (7, 18_000),
            (8, 24_000),
            (9, 32_000),
            (10, 15_000),
            (11, 25_000),
            (12, 35_000),
        ] {
            controller.record_step(step_index, step_budget_millis);
        }

        let step_thirteen = controller.plan_step(13);
        assert_eq!(step_thirteen.baseline_budget_millis, 55_000);
        assert_eq!(step_thirteen.spill_budget_millis, 28_125);
        assert_eq!(step_thirteen.total_budget_millis, 83_125);
        assert_eq!(step_thirteen.proof_close_reserve_millis, 24_937);
        assert_eq!(step_thirteen.discovery_budget_millis, 58_188);
        assert_eq!(step_thirteen.scout_budget_millis, 5_818);
        assert_eq!(step_thirteen.breadth_harvest_budget_millis, 52_370);
    }

    #[test]
    fn demo_budget_controller_respects_seeded_resume_timing() {
        let config = demo_runtime_config_10m();
        let controller = DemoBudgetController::maybe_new(
            &config,
            15,
            DemoBudgetSeed {
                consumed_total_millis: 235_000,
                consumed_early_millis: 60_000,
            },
        )
        .expect("demo budget controller should build")
        .expect("demo profile should enable the budget controller");

        let resumed_step_thirteen = controller.plan_step(13);
        assert_eq!(resumed_step_thirteen.total_budget_millis, 83_125);
        assert_eq!(resumed_step_thirteen.proof_close_reserve_millis, 24_937);
        assert_eq!(resumed_step_thirteen.discovery_budget_millis, 58_188);
        assert_eq!(resumed_step_thirteen.scout_budget_millis, 5_818);
    }

    #[test]
    fn demo_budget_controller_frontloads_more_spill_after_floor_miss_with_slack() {
        let config = demo_runtime_config_10m();
        let mut controller =
            DemoBudgetController::maybe_new(&config, 15, DemoBudgetSeed::default())
                .expect("demo budget controller should build")
                .expect("demo profile should enable the budget controller");
        for (step_index, step_budget_millis) in [
            (1, 30_000),
            (2, 10_000),
            (3, 10_000),
            (4, 10_000),
            (5, 12_000),
            (6, 14_000),
            (7, 18_000),
            (8, 24_000),
            (9, 32_000),
            (10, 15_000),
            (11, 25_000),
            (12, 35_000),
        ] {
            controller.record_step(step_index, step_budget_millis);
        }

        let baseline = controller.plan_step(13);
        controller.record_feedback(DemoBudgetFeedback {
            late_floor_miss: true,
            reserve_pressure: false,
            reserve_slack: true,
        });
        let adaptive = controller.plan_step(13);

        assert!(adaptive.spill_budget_millis > baseline.spill_budget_millis);
        assert!(adaptive.total_budget_millis > baseline.total_budget_millis);
        assert!(adaptive.discovery_budget_millis > baseline.discovery_budget_millis);
    }

    #[test]
    fn demo_budget_controller_protects_more_reserve_after_reserve_pressure() {
        let config = demo_runtime_config_10m();
        let mut controller =
            DemoBudgetController::maybe_new(&config, 15, DemoBudgetSeed::default())
                .expect("demo budget controller should build")
                .expect("demo profile should enable the budget controller");
        for (step_index, step_budget_millis) in [
            (1, 30_000),
            (2, 10_000),
            (3, 10_000),
            (4, 10_000),
            (5, 12_000),
            (6, 14_000),
            (7, 18_000),
            (8, 24_000),
            (9, 32_000),
            (10, 15_000),
            (11, 25_000),
            (12, 35_000),
        ] {
            controller.record_step(step_index, step_budget_millis);
        }

        let baseline = controller.plan_step(13);
        controller.record_feedback(DemoBudgetFeedback {
            late_floor_miss: false,
            reserve_pressure: true,
            reserve_slack: false,
        });
        let adaptive = controller.plan_step(13);

        assert_eq!(adaptive.spill_budget_millis, baseline.spill_budget_millis);
        assert!(adaptive.proof_close_reserve_millis > baseline.proof_close_reserve_millis);
        assert!(adaptive.discovery_budget_millis < baseline.discovery_budget_millis);
    }

    #[test]
    fn demo_scout_rebalance_borrows_from_proof_close_reserve_under_floor_pressure() {
        let mut budget = DemoStepBudget {
            step_index: 13,
            total_budget_millis: 60_000,
            discovery_budget_millis: 30_000,
            scout_budget_millis: 5_000,
            breadth_harvest_budget_millis: 25_000,
            proof_close_reserve_millis: 30_000,
            generated_floor: Some(5_000),
            max_live_rebalance_borrow_millis: 15_000,
            next_live_retune_check_millis: 5_000,
            ..DemoStepBudget::default()
        };
        let progress = narrative_progress_snapshot(5_000, 400, 100, 0);

        let rebalance = maybe_retune_demo_budget_live(&mut budget, &progress)
            .expect("floor pressure should borrow from proof_close reserve");

        assert_eq!(
            rebalance.action,
            DemoBudgetRetuneAction::BorrowFromProofClose
        );
        assert_eq!(rebalance.adjustment_millis, 15_000);
        assert_eq!(budget.discovery_budget_millis, 45_000);
        assert_eq!(budget.breadth_harvest_budget_millis, 40_000);
        assert_eq!(budget.proof_close_reserve_millis, 15_000);
        assert_eq!(budget.live_rebalance_borrowed_millis, 15_000);
        assert!(rebalance.projected_generated_surface > progress.generated_surface);
    }

    #[test]
    fn demo_scout_rebalance_waits_until_the_scout_slice_finishes() {
        let mut budget = DemoStepBudget {
            step_index: 13,
            total_budget_millis: 60_000,
            discovery_budget_millis: 30_000,
            scout_budget_millis: 5_000,
            breadth_harvest_budget_millis: 25_000,
            proof_close_reserve_millis: 30_000,
            generated_floor: Some(5_000),
            max_live_rebalance_borrow_millis: 15_000,
            next_live_retune_check_millis: 5_000,
            ..DemoStepBudget::default()
        };

        assert!(
            maybe_retune_demo_budget_live(
                &mut budget,
                &narrative_progress_snapshot(4_999, 400, 100, 0),
            )
            .is_none()
        );
        assert_eq!(budget.discovery_budget_millis, 30_000);
        assert_eq!(budget.proof_close_reserve_millis, 30_000);
        assert_eq!(budget.live_rebalance_borrowed_millis, 0);
    }

    #[test]
    fn demo_breadth_harvest_live_retune_can_borrow_again_when_floor_pressure_worsens() {
        let mut budget = DemoStepBudget {
            step_index: 13,
            total_budget_millis: 60_000,
            discovery_budget_millis: 30_000,
            scout_budget_millis: 5_000,
            breadth_harvest_budget_millis: 25_000,
            proof_close_reserve_millis: 30_000,
            generated_floor: Some(4_700),
            max_live_rebalance_borrow_millis: 15_000,
            next_live_retune_check_millis: 5_000,
            ..DemoStepBudget::default()
        };

        let first = maybe_retune_demo_budget_live(
            &mut budget,
            &narrative_progress_snapshot(5_000, 700, 100, 0),
        )
        .expect("first live retune should borrow from proof_close reserve");
        assert_eq!(first.action, DemoBudgetRetuneAction::BorrowFromProofClose);
        assert!(first.adjustment_millis >= 1_000);
        let first_borrow = first.adjustment_millis;

        let second = maybe_retune_demo_budget_live(
            &mut budget,
            &narrative_progress_snapshot(12_000, 1_200, 200, 0),
        )
        .expect("later breadth-harvest pulse should borrow again when projections worsen");
        assert_eq!(second.action, DemoBudgetRetuneAction::BorrowFromProofClose);
        assert!(second.adjustment_millis >= 1_000);
        assert_eq!(
            budget.live_rebalance_borrowed_millis,
            first_borrow + second.adjustment_millis
        );
        assert_eq!(
            budget.live_rebalance_borrowed_millis,
            budget.max_live_rebalance_borrow_millis
        );
        assert_eq!(
            budget.proof_close_reserve_millis,
            30_000 - budget.live_rebalance_borrowed_millis
        );
    }

    #[test]
    fn demo_breadth_harvest_live_retune_can_return_time_to_proof_close_after_recovery() {
        let mut budget = DemoStepBudget {
            step_index: 13,
            total_budget_millis: 60_000,
            discovery_budget_millis: 40_000,
            scout_budget_millis: 5_000,
            breadth_harvest_budget_millis: 35_000,
            proof_close_reserve_millis: 20_000,
            generated_floor: Some(5_000),
            live_rebalance_borrowed_millis: 10_000,
            max_live_rebalance_borrow_millis: 15_000,
            next_live_retune_check_millis: 6_000,
            ..DemoStepBudget::default()
        };

        let retune = maybe_retune_demo_budget_live(
            &mut budget,
            &narrative_progress_snapshot(12_000, 3_600, 500, 0),
        )
        .expect("recovered floor projection should return time to proof_close reserve");

        assert_eq!(retune.action, DemoBudgetRetuneAction::ReturnToProofClose);
        assert_eq!(retune.adjustment_millis, 10_000);
        assert_eq!(budget.discovery_budget_millis, 30_000);
        assert_eq!(budget.breadth_harvest_budget_millis, 25_000);
        assert_eq!(budget.proof_close_reserve_millis, 30_000);
        assert_eq!(budget.live_rebalance_borrowed_millis, 0);
    }

    #[test]
    fn demo_search_records_scout_rebalance_when_floor_pressure_stays_high() {
        let mut config = demo_runtime_config_10m();
        config.demo.scout_fraction = "0.00".to_owned();
        config
            .demo
            .caps
            .full_eval_soft_cap
            .insert("15".to_owned(), 10_000);
        let planned = DemoBudgetController::maybe_new(&config, 15, DemoBudgetSeed::default())
            .expect("demo budget controller should build")
            .expect("demo profile should enable the budget controller")
            .plan_step(15);

        let step = search_bootstrap_prefix_for_config_with_runtime(
            15,
            2,
            &config,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("demo steps should build")
        .into_iter()
        .last()
        .expect("step should exist");

        assert!(step.demo_phase.proof_close_reserved_millis < planned.proof_close_reserve_millis);
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::BudgetPulse
                && event.phase == Some(StepPhase::Scout)
                && event.message.contains("borrowed")
        }));
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::PhaseChange
                && event.phase == Some(StepPhase::BreadthHarvest)
        }));
    }

    #[test]
    fn demo_live_budget_pulses_rate_limit_repeated_scout_rebalances() {
        let library = library_until(9);
        let admissibility =
            strict_admissibility_for_mode(10, 2, &library, AdmissibilityMode::DemoBreadthShadow);
        let mut runtime = DemoNarrativeRuntime::new(
            10,
            Rational::zero(),
            admissibility,
            DemoStepBudget {
                step_index: 10,
                total_budget_millis: 60_000,
                discovery_budget_millis: 30_000,
                scout_budget_millis: 5_000,
                breadth_harvest_budget_millis: 25_000,
                proof_close_reserve_millis: 30_000,
                pulse_interval_millis: 500,
                ..DemoStepBudget::default()
            },
        );
        let budget = DemoStepBudget {
            step_index: 10,
            total_budget_millis: 60_000,
            discovery_budget_millis: 45_000,
            scout_budget_millis: 5_000,
            breadth_harvest_budget_millis: 40_000,
            proof_close_reserve_millis: 15_000,
            pulse_interval_millis: 500,
            live_rebalance_borrowed_millis: 15_000,
            max_live_rebalance_borrow_millis: 15_000,
            ..DemoStepBudget::default()
        };
        let rebalance = super::DemoBudgetRetune {
            action: DemoBudgetRetuneAction::BorrowFromProofClose,
            adjustment_millis: 15_000,
            projected_generated_surface: 4_700,
            projected_exact_screened_surface: 700,
        };

        runtime.record_scout_budget_rebalance(
            narrative_progress_snapshot(5_000, 400, 100, 0),
            "generated_per_sec=80 admissibility_per_sec=20 exact_bound_per_sec=10 full_eval_per_sec=0 generated=400 admissibility_checks=100 exact_bound_checks=50 full_evals=0".to_owned(),
            budget,
            rebalance,
        );
        let pulse_count_after_first = runtime
            .recorder
            .events()
            .iter()
            .filter(|event| {
                event.kind == NarrativeEventKind::BudgetPulse
                    && event.phase == Some(StepPhase::Scout)
            })
            .count();

        runtime.record_scout_budget_rebalance(
            narrative_progress_snapshot(5_200, 420, 110, 0),
            "generated_per_sec=81 admissibility_per_sec=21 exact_bound_per_sec=10 full_eval_per_sec=0 generated=420 admissibility_checks=110 exact_bound_checks=55 full_evals=0".to_owned(),
            budget,
            rebalance,
        );
        let pulse_count_after_second = runtime
            .recorder
            .events()
            .iter()
            .filter(|event| {
                event.kind == NarrativeEventKind::BudgetPulse
                    && event.phase == Some(StepPhase::Scout)
            })
            .count();

        runtime.record_scout_budget_rebalance(
            narrative_progress_snapshot(5_600, 450, 120, 0),
            "generated_per_sec=82 admissibility_per_sec=22 exact_bound_per_sec=11 full_eval_per_sec=0 generated=450 admissibility_checks=120 exact_bound_checks=60 full_evals=0".to_owned(),
            budget,
            rebalance,
        );
        let pulse_count_after_third = runtime
            .recorder
            .events()
            .iter()
            .filter(|event| {
                event.kind == NarrativeEventKind::BudgetPulse
                    && event.phase == Some(StepPhase::Scout)
            })
            .count();

        assert_eq!(pulse_count_after_second, pulse_count_after_first);
        assert_eq!(pulse_count_after_third, pulse_count_after_first + 1);
    }

    #[test]
    fn demo_steps_emit_search_side_phase_events() {
        let config = demo_runtime_config_10m();
        let steps = search_bootstrap_prefix_for_config_with_runtime(
            3,
            2,
            &config,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("demo steps should build");
        let step = steps.last().expect("step should exist");

        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::PhaseChange && event.phase == Some(StepPhase::Scout)
        }));
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::PhaseChange
                && event.phase == Some(StepPhase::Materialize)
        }));
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::PhaseChange
                && event.phase == Some(StepPhase::ProofClose)
        }));
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::PhaseChange && event.phase == Some(StepPhase::Seal)
        }));
        assert!(
            step.narrative_events
                .iter()
                .any(|event| event.kind == NarrativeEventKind::BudgetPulse)
        );
    }

    #[test]
    fn demo_early_steps_restore_full_clause_catalog_generation() {
        let config = demo_runtime_config_10m();
        let step = search_bootstrap_prefix_for_config_with_runtime(
            1,
            2,
            &config,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("demo steps should build")
        .into_iter()
        .last()
        .expect("step should exist");

        assert_eq!(step.step_index, 1);
        assert_eq!(step.demo_funnel.generated_raw_prefixes, 2144);
        assert!(
            step.search_timing.step_wall_clock_millis
                <= u64::from(config.demo.early_exhaustive_budget_sec) * 1_000
        );
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::SurfaceSummary
                && event
                    .progress
                    .as_ref()
                    .map(|progress| progress.generated_surface == 2144)
                    .unwrap_or(false)
        }));
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::BudgetPulse
                && event.phase == Some(StepPhase::Scout)
                && event
                    .detail
                    .as_deref()
                    .map(|detail| {
                        detail.contains("raw_clause_widths=18x120")
                            && detail.contains("excluded_exact_clause_echoes=16")
                    })
                    .unwrap_or(false)
        }));
    }

    #[test]
    fn demo_late_surface_carries_through_live_config_runs() {
        let config = demo_runtime_config_10m();
        let steps = search_bootstrap_prefix_for_config_with_runtime(
            15,
            2,
            &config,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("demo steps should build");
        let step10 = steps
            .iter()
            .find(|step| step.step_index == 10)
            .expect("step 10 should exist");
        let step11 = steps
            .iter()
            .find(|step| step.step_index == 11)
            .expect("step 11 should exist");
        let step12 = steps
            .iter()
            .find(|step| step.step_index == 12)
            .expect("step 12 should exist");
        let step13 = steps
            .iter()
            .find(|step| step.step_index == 13)
            .expect("step 13 should exist");
        let step14 = steps
            .iter()
            .find(|step| step.step_index == 14)
            .expect("step 14 should exist");
        let step15 = steps
            .iter()
            .find(|step| step.step_index == 15)
            .expect("step 15 should exist");

        assert_eq!(step10.telescope, Telescope::reference(10));
        assert_eq!(step11.telescope, Telescope::reference(11));
        assert_eq!(step12.telescope, Telescope::reference(12));
        assert_eq!(step13.telescope, Telescope::reference(13));
        assert_eq!(step14.telescope, Telescope::reference(14));
        assert_eq!(step15.telescope, Telescope::reference(15));

        assert_eq!(step10.full_telescopes_evaluated, 1);
        assert_eq!(step11.full_telescopes_evaluated, 1);
        assert_eq!(step12.full_telescopes_evaluated, 1);
        assert_eq!(step13.full_telescopes_evaluated, 1);
        assert_eq!(step14.full_telescopes_evaluated, 1);
        assert_eq!(step15.full_telescopes_evaluated, 1);

        assert!(
            step10.demo_funnel.generated_raw_prefixes >= 500,
            "step 10 should keep the generated floor hit"
        );
        assert!(
            step10.demo_funnel.exact_bound_screened >= 120,
            "step 10 should now hit the exact-screened floor"
        );
        assert!(
            step11.demo_funnel.generated_raw_prefixes >= 800,
            "step 11 should keep the generated floor hit"
        );
        assert!(
            step11.demo_funnel.exact_bound_screened >= 220,
            "step 11 should keep the exact-screened floor hit"
        );
        assert!(
            step12.demo_funnel.generated_raw_prefixes >= 1_200,
            "step 12 should now hit the generated floor"
        );
        assert!(
            step12.demo_funnel.exact_bound_screened >= 400,
            "step 12 should now hit the exact-screened floor"
        );
        assert!(
            step13.demo_funnel.generated_raw_prefixes >= 3_500,
            "step 13 should keep the widened live surface"
        );
        assert!(
            step13.demo_funnel.exact_bound_screened >= 3_000,
            "step 13 should keep the widened exact-screened surface"
        );
        assert!(
            step14.demo_funnel.generated_raw_prefixes >= 3_500,
            "step 14 should now hit the generated floor"
        );
        assert!(
            step14.demo_funnel.exact_bound_screened >= 1_100,
            "step 14 should still hit the configured exact-screened floor"
        );
        assert!(
            step15.demo_funnel.generated_raw_prefixes >= 20_000,
            "step 15 should keep the widened live surface"
        );
        assert!(
            step15.demo_funnel.exact_bound_screened >= 18_000,
            "step 15 should keep the widened exact-screened surface"
        );

        assert_eq!(step13.demo_phase.proof_close_closure_percent, 100);
        assert_eq!(step14.demo_phase.proof_close_closure_percent, 100);
        assert_eq!(step15.demo_phase.proof_close_closure_percent, 100);

        let phase_exact_surface = |step: &AtomicSearchStep, phase: StepPhase| {
            step.narrative_events
                .iter()
                .find(|event| {
                    event.kind == NarrativeEventKind::PhaseChange && event.phase == Some(phase)
                })
                .and_then(|event| event.progress.as_ref())
                .map(|progress| progress.exact_screened_surface)
                .unwrap_or(0)
        };

        assert!(
            phase_exact_surface(step10, StepPhase::Materialize) >= 120,
            "step 10 materialize should carry the exact-screened floor hit into the narrative"
        );
        assert!(
            phase_exact_surface(step10, StepPhase::ProofClose) >= 120,
            "step 10 proof_close should carry the exact-screened floor hit into the narrative"
        );
        assert_eq!(
            phase_exact_surface(step10, StepPhase::Seal),
            step10.demo_funnel.exact_bound_screened as u64
        );
        assert!(
            phase_exact_surface(step12, StepPhase::Materialize) >= 400,
            "step 12 materialize should carry the exact-screened floor hit into the narrative"
        );
        assert!(
            phase_exact_surface(step12, StepPhase::ProofClose) >= 400,
            "step 12 proof_close should carry the exact-screened floor hit into the narrative"
        );
        assert_eq!(
            phase_exact_surface(step12, StepPhase::Seal),
            step12.demo_funnel.exact_bound_screened as u64
        );
    }

    #[test]
    fn demo_soft_cap_handoff_counts_proof_close_overrun() {
        let mut config = demo_runtime_config_10m();
        config
            .demo
            .caps
            .full_eval_soft_cap
            .insert("15".to_owned(), 0);
        let step = search_bootstrap_prefix_for_config_with_runtime(
            15,
            2,
            &config,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("demo steps should build")
        .into_iter()
        .last()
        .expect("step should exist");

        assert_eq!(step.step_index, 15);
        assert_eq!(step.demo_phase.full_eval_soft_cap, Some(0));
        assert!(step.demo_phase.materialize_soft_cap_triggered);
        assert_eq!(
            step.demo_phase.proof_close_entry_reason,
            Some(DemoProofCloseEntryReason::SoftCapHandoff)
        );
        assert!(step.demo_phase.proof_close_full_evals > 0);
        assert!(step.demo_phase.proof_close_overrun_full_evals > 0);
        assert_eq!(
            step.demo_phase.proof_close_overrun_reason,
            Some(DemoProofCloseOverrunReason::SoftCapHandoff)
        );
        assert_eq!(
            step.demo_phase.materialize_full_evals + step.demo_phase.proof_close_full_evals,
            step.full_telescopes_evaluated
        );
        assert!(step.demo_phase.proof_close_reserved_millis > 0);
        assert!(step.demo_phase.proof_close_frontier_total_groups > 0);
        assert_eq!(
            step.demo_phase.proof_close_frontier_groups_closed,
            step.demo_phase.proof_close_frontier_total_groups
        );
        assert_eq!(step.demo_phase.proof_close_frontier_groups_remaining, 0);
        assert_eq!(step.demo_phase.proof_close_closure_percent, 100);
        assert!(!step.demo_phase.proof_close_reserve_exhausted);
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::PhaseChange
                && event.phase == Some(StepPhase::ProofClose)
                && event.message.contains("soft cap")
        }));
    }

    #[test]
    fn demo_materialize_handoff_switches_on_when_reserve_is_tighter_than_pending_surface() {
        let incumbent = test_accept_rank(1, "incumbent");
        let budget = DemoStepBudget {
            proof_close_reserve_millis: 900,
            ..DemoStepBudget::default()
        };

        assert_eq!(
            demo_materialize_to_proof_close_handoff_reason_for_pressure(
                budget,
                DemoClosurePressure {
                    pending_exact_surface: 2,
                    ..DemoClosurePressure::default()
                },
                Some(&incumbent),
                1,
            ),
            Some(DemoProofCloseEntryReason::MaterializeReserveHandoff)
        );
        assert_eq!(
            demo_materialize_to_proof_close_handoff_reason_for_pressure(
                budget,
                DemoClosurePressure::default(),
                Some(&incumbent),
                1,
            ),
            None
        );
        assert_eq!(
            demo_materialize_to_proof_close_handoff_reason_for_pressure(
                budget,
                DemoClosurePressure {
                    pending_exact_surface: 2,
                    ..DemoClosurePressure::default()
                },
                None,
                1,
            ),
            None
        );
        assert_eq!(
            demo_materialize_to_proof_close_handoff_reason_for_pressure(
                budget,
                DemoClosurePressure {
                    pending_exact_surface: 2,
                    ..DemoClosurePressure::default()
                },
                Some(&incumbent),
                0,
            ),
            None
        );
    }

    #[test]
    fn demo_floor_hit_reason_is_persisted_into_proof_close() {
        let mut config = demo_runtime_config_10m();
        config.demo.scout_fraction = "0.00".to_owned();
        config
            .demo
            .floors
            .generated_floor
            .insert("15".to_owned(), 1);
        config.demo.floors.exact_screened_floor.remove("15");
        config
            .demo
            .caps
            .full_eval_soft_cap
            .insert("15".to_owned(), 10_000);
        let step = search_bootstrap_prefix_for_config_with_runtime(
            15,
            2,
            &config,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("demo steps should build")
        .into_iter()
        .last()
        .expect("step should exist");

        assert_eq!(
            step.demo_phase.breadth_harvest_exit_reason,
            Some(DemoBreadthHarvestExitReason::GeneratedFloorHit)
        );
        assert_eq!(
            step.demo_phase.proof_close_entry_reason,
            Some(DemoProofCloseEntryReason::BreadthFloorHit)
        );
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::BudgetPulse
                && event.phase == Some(StepPhase::BreadthHarvest)
                && event.message.contains("generated floor")
        }));
    }

    #[test]
    fn demo_reserve_protection_reason_is_persisted_into_proof_close() {
        let mut config = demo_runtime_config_10m();
        config.demo.scout_fraction = "0.00".to_owned();
        config.demo.proof_close_reserve_fraction = "1.00".to_owned();
        config.demo.floors.generated_floor.remove("15");
        config.demo.floors.exact_screened_floor.remove("15");
        config
            .demo
            .caps
            .full_eval_soft_cap
            .insert("15".to_owned(), 10_000);
        let step = search_bootstrap_prefix_for_config_with_runtime(
            15,
            2,
            &config,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("demo steps should build")
        .into_iter()
        .last()
        .expect("step should exist");

        assert_eq!(
            step.demo_phase.breadth_harvest_exit_reason,
            Some(DemoBreadthHarvestExitReason::ProofCloseReserveProtected)
        );
        assert_eq!(
            step.demo_phase.proof_close_entry_reason,
            Some(DemoProofCloseEntryReason::ReserveProtected)
        );
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::PhaseChange
                && event.phase == Some(StepPhase::ProofClose)
                && event.message.contains("reserved certification slice")
        }));
    }

    #[test]
    fn demo_zero_reserve_records_proof_close_reserve_exhaustion() {
        let mut config = demo_runtime_config_10m();
        config.demo.scout_fraction = "0.00".to_owned();
        config.demo.proof_close_reserve_fraction = "0.00".to_owned();
        config.demo.floors.generated_floor.remove("15");
        config.demo.floors.exact_screened_floor.remove("15");
        config
            .demo
            .caps
            .full_eval_soft_cap
            .insert("15".to_owned(), 0);
        let step = search_bootstrap_prefix_for_config_with_runtime(
            15,
            2,
            &config,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("demo steps should build")
        .into_iter()
        .last()
        .expect("step should exist");

        assert_eq!(step.demo_phase.proof_close_reserved_millis, 0);
        assert!(step.demo_phase.proof_close_reserve_exhausted);
        assert!(step.demo_phase.proof_close_frontier_total_groups > 0);
        assert_eq!(step.demo_phase.proof_close_frontier_groups_remaining, 0);
        assert_eq!(step.demo_phase.proof_close_closure_percent, 100);
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::BudgetPulse
                && event.phase == Some(StepPhase::ProofClose)
                && event
                    .message
                    .contains("exhausted the reserved certification slice")
        }));
    }

    #[test]
    fn demo_mandatory_proof_close_and_seal_pulses_ignore_rate_limiting() {
        let mut config = demo_runtime_config_10m();
        config.demo.narrative.pulse_interval_millis = 60_000;
        config.demo.scout_fraction = "0.00".to_owned();
        config.demo.floors.generated_floor.remove("15");
        config.demo.floors.exact_screened_floor.remove("15");
        config
            .demo
            .caps
            .full_eval_soft_cap
            .insert("15".to_owned(), 0);
        let step = search_bootstrap_prefix_for_config_with_runtime(
            15,
            2,
            &config,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("demo steps should build")
        .into_iter()
        .last()
        .expect("step should exist");

        assert!(step.demo_phase.proof_close_frontier_total_groups > 0);
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::BudgetPulse
                && event.phase == Some(StepPhase::ProofClose)
                && event
                    .message
                    .contains("100% of the retained frontier groups")
        }));
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::BudgetPulse
                && event.phase == Some(StepPhase::Seal)
                && event.message.contains("seal fixed overshoot")
        }));
    }

    fn relaxed_shadow_step_from_reference_prefix(step_index: u32) -> AtomicSearchStep {
        assert!(step_index >= 1);
        let prefix = if step_index == 1 {
            Vec::new()
        } else {
            reference_prefix(step_index - 1)
        };
        search_bootstrap_from_prefix_for_profile_with_runtime(
            &prefix,
            step_index,
            2,
            SearchProfile::RelaxedShadow,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("relaxed shadow step should succeed")
        .into_iter()
        .last()
        .expect("relaxed shadow step")
    }

    fn profile_step_from_reference_prefix(
        step_index: u32,
        profile: SearchProfile,
    ) -> AtomicSearchStep {
        assert!(step_index >= 1);
        let prefix = if step_index == 1 {
            Vec::new()
        } else {
            reference_prefix(step_index - 1)
        };
        search_bootstrap_from_prefix_for_profile_with_runtime(
            &prefix,
            step_index,
            2,
            profile,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("profile step should succeed")
        .into_iter()
        .last()
        .expect("profile step")
    }

    #[test]
    fn live_search_support_is_honest_about_current_bootstrap_range() {
        assert!(supports_live_atomic_search(LIVE_BOOTSTRAP_MAX_STEP));
        assert!(!supports_live_atomic_search(LIVE_BOOTSTRAP_MAX_STEP + 1));
    }

    #[test]
    fn demo_breadth_shadow_currently_reuses_realistic_shadow_admissibility() {
        assert_eq!(
            super::admissibility_mode_for_profile(SearchProfile::DemoBreadthShadow),
            AdmissibilityMode::RealisticShadow
        );
    }

    #[test]
    fn desktop_claim_shadow_has_a_distinct_admissibility_mode() {
        assert_eq!(
            super::admissibility_mode_for_profile(SearchProfile::DesktopClaimShadow),
            AdmissibilityMode::DesktopClaimShadow
        );
    }

    #[test]
    fn desktop_claim_shadow_step_four_surface_diagnostics_do_not_activate_late_widening() {
        let library = reference_library(3);
        let admissibility =
            strict_admissibility_for_mode(4, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let context = discovery_enumeration_context(&library, admissibility, true);
        let claim_surface = context.surface_diagnostics();
        let raw_widths = crate::enumerate::raw_clause_catalog_widths(context, 4);

        assert_eq!(
            claim_surface.late_family_surface,
            LateFamilySurface::ClaimGeneric
        );
        assert_eq!(claim_surface.library_size, 3);
        assert!(!claim_surface.claim_widening_band7_active);
        assert!(!claim_surface.claim_widening_band8_active);
        assert!(!claim_surface.claim_widening_band9_active);
        assert!(!raw_widths.is_empty());
    }

    #[test]
    fn desktop_claim_shadow_step_five_keeps_claim_generic_surface_under_demo_budget() {
        let library = reference_library(4);
        let admissibility =
            strict_admissibility_for_mode(5, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let claim_context = discovery_enumeration_context(&library, admissibility, true);
        let demo_context = discovery_enumeration_context(&library, admissibility, false);
        let demo_shadow_context = discovery_enumeration_context(
            &library,
            strict_admissibility_for_mode(5, 2, &library, AdmissibilityMode::DemoBreadthShadow),
            true,
        );

        let claim_surface = claim_context.surface_diagnostics();
        let demo_surface = demo_context.surface_diagnostics();
        let demo_shadow_surface = demo_shadow_context.surface_diagnostics();
        assert_eq!(
            claim_surface.late_family_surface,
            LateFamilySurface::ClaimGeneric
        );
        assert_eq!(claim_surface.library_size, 4);
        assert!(!claim_surface.claim_widening_band7_active);
        assert!(!claim_surface.claim_widening_band8_active);
        assert!(!claim_surface.claim_widening_band9_active);
        assert_eq!(
            demo_surface.late_family_surface,
            LateFamilySurface::ClaimGeneric
        );
        assert_eq!(
            demo_shadow_surface.late_family_surface,
            LateFamilySurface::DemoBreadthShadow
        );
    }

    #[test]
    fn desktop_claim_shadow_switches_late_bucket_taxonomy_to_structural_generic() {
        let claim_step = profile_step_from_reference_prefix(10, SearchProfile::DesktopClaimShadow);
        assert_eq!(claim_step.telescope, Telescope::reference(10));
        assert!(!claim_step.demo_bucket_stats.is_empty());
        assert!(claim_step.demo_bucket_stats.iter().all(|bucket| {
            bucket.bucket_key.taxonomy == SearchBucketTaxonomy::StructuralGeneric
        }));
        assert!(
            claim_step
                .demo_bucket_stats
                .iter()
                .all(|bucket| bucket.bucket_label.contains("structural_generic"))
        );

        let serialized =
            serde_json::to_string(&claim_step.demo_bucket_stats).expect("bucket stats serialize");
        assert!(serialized.contains("\"taxonomy\":\"structural_generic\""));
        assert!(!serialized.contains("temporal_shell"));
        assert!(!serialized.contains("hilbert"));
        assert!(!serialized.contains("curvature"));
    }

    #[test]
    fn desktop_claim_shadow_late_steps_keep_reference_acceptance() {
        for step_index in 13..=15 {
            let claim_step =
                profile_step_from_reference_prefix(step_index, SearchProfile::DesktopClaimShadow);

            assert_eq!(claim_step.telescope, Telescope::reference(step_index));
            assert!(claim_step.demo_bucket_stats.iter().all(|bucket| {
                bucket.bucket_key.taxonomy == SearchBucketTaxonomy::StructuralGeneric
            }));
            assert!(
                claim_step
                    .demo_bucket_stats
                    .iter()
                    .all(|bucket| { bucket.bucket_label.contains("structural_generic") })
            );
        }
    }

    #[test]
    fn claim_terminal_prefix_completion_summary_matches_direct_exact_assessment() {
        let strict_steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut nu_history = Vec::new();

        for step in &strict_steps {
            nu_history.push((step.step_index, u32::from(step.accepted.nu)));
            library.push(LibraryEntry::from_telescope(&step.telescope, &library));
        }

        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));

        let terminal_clauses = super::terminal_prefix_clause_candidates(
            15,
            &library,
            admissibility,
            &signature,
            clause_catalog.clauses_at(7),
            Some(clause_catalog.terminal_connectivity_facts_at(7)),
            clause_catalog.terminal_nu_facts_at(7),
            &mut cache,
            None,
        );
        assert!(terminal_clauses.len() > 0);
        match &terminal_clauses {
            super::TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand(clauses) => {
                for clause in clauses {
                    let mut telescope = prefix.clone();
                    telescope.clauses.push(clause.clause.clone());
                    let decision =
                        assess_strict_admissibility(15, &library, &telescope, admissibility);
                    assert!(decision.is_admitted());
                    assert_eq!(
                        decision.class,
                        AdmissibilityDecisionClass::AdmittedFocusAligned
                    );
                    assert_eq!(decision.reason, super::CLAIM_OPEN_BAND_ADMISSIBILITY_REASON);
                }
            }
            super::TerminalPrefixClauseCandidates::General(clauses) => {
                for clause in clauses {
                    let mut telescope = prefix.clone();
                    telescope.clauses.push(clause.clause.clone());
                    assert_eq!(
                        clause.cached_admissibility_decision.as_ref(),
                        Some(&assess_strict_admissibility(
                            15,
                            &library,
                            &telescope,
                            admissibility,
                        ))
                    );
                }
            }
        }

        let mut telemetry = super::RemainingOneTelemetry::default();
        let summary = super::compute_terminal_prefix_completion_summary_from_candidates(
            15,
            &library,
            admissibility,
            Rational::zero(),
            &nu_history,
            &signature,
            &prefix,
            super::TerminalPrefixSummaryPayload::Full,
            terminal_clauses,
            None,
            &mut cache,
            Some(&mut telemetry),
            None,
        );
        let compact_summary = super::compute_terminal_prefix_completion_summary_from_candidates(
            15,
            &library,
            admissibility,
            Rational::zero(),
            &nu_history,
            &signature,
            &prefix,
            super::TerminalPrefixSummaryPayload::Compact,
            super::terminal_prefix_clause_candidates(
                15,
                &library,
                admissibility,
                &signature,
                clause_catalog.clauses_at(7),
                Some(clause_catalog.terminal_connectivity_facts_at(7)),
                clause_catalog.terminal_nu_facts_at(7),
                &mut cache,
                None,
            ),
            None,
            &mut cache,
            None,
            None,
        );

        let mut direct_evaluations = Vec::new();
        let mut direct_admitted = 0usize;
        let mut direct_bound: Option<PrefixBound> = None;
        for clause in clause_catalog.clauses_at(7) {
            let mut telescope = prefix.clone();
            telescope.clauses.push(clause.clone());

            if !passes_connectivity(&library, &telescope) {
                direct_evaluations.push(TerminalPrefixClauseEvaluation::Disconnected);
                continue;
            }

            let decision = assess_strict_admissibility(15, &library, &telescope, admissibility);
            if !decision.is_admitted() {
                direct_evaluations
                    .push(TerminalPrefixClauseEvaluation::AdmissibilityRejected { decision });
                continue;
            }

            direct_admitted += 1;
            let exact_nu =
                u16::try_from(pen_eval::nu::structural_nu(&telescope, &library, &nu_history).total)
                    .expect("nu exceeded u16");
            let bit_kappa_used = u16::try_from(pen_core::encode::telescope_bit_cost(&telescope))
                .expect("bit cost exceeded u16");
            let clause_kappa_used = u16::try_from(telescope.kappa()).expect("kappa exceeded u16");
            if let Some(bound) = direct_bound.as_mut() {
                bound.absorb_completion(exact_nu, clause_kappa_used, bit_kappa_used);
            } else {
                direct_bound = Some(PrefixBound::singleton(
                    exact_nu,
                    clause_kappa_used,
                    bit_kappa_used,
                ));
            }
            direct_evaluations.push(TerminalPrefixClauseEvaluation::Admitted {
                decision,
                completion: TerminalPrefixCompletion {
                    exact_nu,
                    bit_kappa_used,
                    clause_kappa_used,
                    telescope,
                },
            });
        }

        assert_eq!(summary.evaluations, Some(direct_evaluations));
        assert_eq!(summary.admitted_candidate_count, direct_admitted);
        assert_eq!(summary.bound, direct_bound);
        assert_eq!(compact_summary.evaluations, None);
        assert_eq!(
            compact_summary.generated_candidate_count,
            summary.generated_candidate_count
        );
        assert_eq!(
            compact_summary.admitted_candidate_count,
            summary.admitted_candidate_count
        );
        assert_eq!(compact_summary.bound, summary.bound);
        assert_eq!(
            compact_summary.best_accept_primary_rank,
            summary.best_accept_primary_rank
        );
        assert_eq!(compact_summary.best_accept_rank, summary.best_accept_rank);
        assert_eq!(
            telemetry.terminal_summary_connectivity_checks,
            summary.generated_candidate_count
        );
        assert_eq!(telemetry.terminal_summary_admissibility_checks, 0);
        assert_eq!(
            telemetry.terminal_summary_exact_nu_evaluations,
            summary.admitted_candidate_count
        );
        assert_eq!(telemetry.terminal_summary_plateau_activations, 0);
    }

    #[test]
    fn claim_replay_fixture_replays_compact_summary_with_parity() {
        let (library, history, nu_history) = reference_history_until(3);
        let admissibility =
            strict_admissibility_for_mode(4, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let objective_bar = compute_bar(2, 4, &history).bar;
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 3);
        let prefix = Telescope::new(Telescope::reference(4).clauses[..2].to_vec());
        let signature = PrefixSignature::new(4, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            3,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));

        let summary = super::compute_terminal_prefix_completion_summary_from_candidates(
            4,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &signature,
            &prefix,
            super::TerminalPrefixSummaryPayload::Compact,
            super::terminal_prefix_clause_candidates(
                4,
                &library,
                admissibility,
                &signature,
                clause_catalog.clauses_at(2),
                Some(clause_catalog.terminal_connectivity_facts_at(2)),
                clause_catalog.terminal_nu_facts_at(2),
                &mut cache,
                None,
            ),
            None,
            &mut cache,
            None,
            None,
        );

        let fixture = super::claim_replay::build_claim_remaining_one_replay_fixture(
            4,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &signature,
            &prefix,
            None,
            super::claim_replay::ClaimRemainingOneSurfaceSnapshot {
                prefix_states_explored: 24,
                prefix_cache_groups: super::CLAIM_REMAINING_ONE_RETAINED_PLATEAU_GROUPS,
                prefix_cache_candidates: super::CLAIM_REMAINING_ONE_RETAINED_PLATEAU_CANDIDATES,
            },
            &super::terminal_prefix_clause_candidates(
                4,
                &library,
                admissibility,
                &signature,
                clause_catalog.clauses_at(2),
                Some(clause_catalog.terminal_connectivity_facts_at(2)),
                clause_catalog.terminal_nu_facts_at(2),
                &mut cache,
                None,
            ),
            &summary,
        );

        let replay = super::claim_replay::replay_claim_remaining_one_fixture(&fixture)
            .expect("fixture replay should preserve compact summary parity");

        assert_eq!(replay.summary, fixture.expected_summary);
    }

    #[test]
    fn claim_hoisted_terminal_competition_gate_matches_direct_check() {
        let library = library_until(10);
        let admissibility =
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::RealisticShadow);
        let prefix = Telescope::new(Telescope::reference(11).clauses[..4].to_vec());
        let signature = PrefixSignature::new(11, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            5,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::DemoBreadthShadow
        ));

        let focus_aligned_competitors_only =
            super::demo_focus_aligned_competitors_only(&signature, admissibility, &cache);
        assert!(focus_aligned_competitors_only);

        let admitted_focus_aligned = AdmissibilityDecision {
            class: AdmissibilityDecisionClass::AdmittedFocusAligned,
            reason: "focus".to_owned(),
        };
        let admitted_deprioritized = AdmissibilityDecision {
            class: AdmissibilityDecisionClass::AdmittedDeprioritized,
            reason: "deprioritized".to_owned(),
        };

        for decision in [&admitted_focus_aligned, &admitted_deprioritized] {
            assert_eq!(
                super::terminal_completion_can_compete_for_acceptance(
                    &signature,
                    admissibility,
                    &cache,
                    decision,
                ),
                super::admitted_terminal_completion_can_compete_for_acceptance(
                    focus_aligned_competitors_only,
                    decision,
                )
            );
        }

        assert!(
            super::admitted_terminal_completion_can_compete_for_acceptance(
                focus_aligned_competitors_only,
                &admitted_focus_aligned,
            )
        );
        assert!(
            !super::admitted_terminal_completion_can_compete_for_acceptance(
                focus_aligned_competitors_only,
                &admitted_deprioritized,
            )
        );
    }

    #[test]
    fn claim_exact_partial_prefix_bound_decision_reuses_cached_terminal_result() {
        let steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut history: Vec<DiscoveryRecord> = Vec::new();
        let mut nu_history = Vec::new();

        for step in &steps {
            history.push(DiscoveryRecord::new(
                step.step_index,
                u32::from(step.accepted.nu),
                u32::from(step.accepted.clause_kappa),
            ));
            nu_history.push((step.step_index, u32::from(step.accepted.nu)));
            library.push(LibraryEntry::from_telescope(&step.telescope, &library));
        }

        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let objective_bar = Rational::zero();
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));

        let work_item = create_online_prefix_work_item(
            8,
            prefix,
            signature.clone(),
            &library,
            admissibility,
            &clause_catalog,
            &mut cache,
        );
        assert_eq!(work_item.remaining_clause_slots, 1);

        let mut first_budget = 64;
        let first = exact_partial_prefix_bound_decision(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut cache,
            &mut first_budget,
            None,
            None,
            None,
        );
        assert_ne!(first, super::ExactPartialPrefixBoundDecision::Unknown);

        let cached = cache
            .partial_prefix_bound_decision(&signature)
            .expect("claim terminal decision should be cached after the first exact check");
        assert_eq!(
            cached,
            match first {
                super::ExactPartialPrefixBoundDecision::CanClearBar => {
                    PartialPrefixBoundDecision::CanClearBar
                }
                super::ExactPartialPrefixBoundDecision::CannotClearBar => {
                    PartialPrefixBoundDecision::CannotClearBar
                }
                super::ExactPartialPrefixBoundDecision::Unknown => unreachable!(),
            }
        );
        let hits_before = cache.stats().partial_prefix_bound_hits;
        let terminal_summary_hits_before = cache.stats().terminal_prefix_completion_hits;

        let mut second_budget = 1;
        let second = exact_partial_prefix_bound_decision(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut cache,
            &mut second_budget,
            None,
            None,
            None,
        );

        assert_eq!(second, first);
        assert_eq!(cache.stats().partial_prefix_bound_hits, hits_before + 1);
        assert_eq!(
            cache.stats().terminal_prefix_completion_hits,
            terminal_summary_hits_before
        );
    }

    #[test]
    fn claim_cached_terminal_queue_entry_bound_decision_uses_bound_summary_only() {
        let steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut history: Vec<DiscoveryRecord> = Vec::new();
        let mut nu_history = Vec::new();

        for step in &steps {
            history.push(DiscoveryRecord::new(
                step.step_index,
                u32::from(step.accepted.nu),
                u32::from(step.accepted.clause_kappa),
            ));
            nu_history.push((step.step_index, u32::from(step.accepted.nu)));
            library.push(LibraryEntry::from_telescope(&step.telescope, &library));
        }

        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let objective_bar = compute_bar(2, 15, &history).bar;
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));

        let work_item = create_online_prefix_work_item(
            8,
            prefix,
            signature.clone(),
            &library,
            admissibility,
            &clause_catalog,
            &mut cache,
        );
        assert_eq!(work_item.remaining_clause_slots, 1);

        let mut budget = 64;
        let decision = exact_partial_prefix_bound_decision(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut cache,
            &mut budget,
            None,
            None,
            None,
        );
        assert_ne!(decision, super::ExactPartialPrefixBoundDecision::Unknown);

        let completion_hits_before = cache.stats().terminal_prefix_completion_hits;
        let bound_hits_before = cache.stats().terminal_prefix_bound_hits;
        let cached = super::cached_terminal_prefix_queue_entry_bound_decision(
            objective_bar,
            &work_item,
            &mut cache,
            None,
        )
        .expect("queue-entry bound decision should use the cached terminal summary");

        assert_eq!(cached, decision);
        assert_eq!(
            cache.stats().terminal_prefix_completion_hits,
            completion_hits_before
        );
        assert_eq!(
            cache.stats().terminal_prefix_bound_hits,
            bound_hits_before + 1
        );
    }

    #[test]
    fn claim_remaining_one_algebraic_ceiling_prunes_before_summary_build() {
        let (library, _, nu_history) = reference_history_until(3);

        let admissibility =
            strict_admissibility_for_mode(4, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let objective_bar = Rational::new(100, 1);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 3);
        let prefix = Telescope::reference(1);
        let signature = PrefixSignature::new(4, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            3,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));

        let work_item = create_online_prefix_work_item(
            3,
            prefix,
            signature,
            &library,
            admissibility,
            &clause_catalog,
            &mut cache,
        );
        assert_eq!(work_item.remaining_clause_slots, 1);
        assert!(work_item.next_clause_count > 0);
        assert!(
            super::claim_remaining_one_algebraic_nu_ceiling_cannot_clear_bar(
                &library,
                admissibility,
                objective_bar,
                &nu_history,
                &work_item.prefix_telescope,
            )
        );

        let mut telemetry = super::RemainingOneTelemetry::default();
        let mut budget = 64;
        let decision = exact_partial_prefix_bound_decision(
            4,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut cache,
            &mut budget,
            None,
            Some(&mut telemetry),
            None,
        );

        assert_eq!(
            decision,
            super::ExactPartialPrefixBoundDecision::CannotClearBar
        );
        assert_eq!(telemetry.remaining_one_algebraic_prunes, 1);
        assert_eq!(telemetry.terminal_summary_build_millis, 0);
        assert_eq!(telemetry.terminal_summary_connectivity_checks, 0);
        assert_eq!(telemetry.terminal_summary_exact_nu_evaluations, 0);
        assert_eq!(telemetry.terminal_summary_plateau_activations, 0);
        assert_eq!(cache.entry_counts().terminal_prefix_completions, 0);
    }

    #[test]
    fn remaining_one_telemetry_accumulates_sub_millisecond_durations() {
        let mut telemetry = super::RemainingOneTelemetry::default();

        telemetry.absorb_terminal_prefix_clause_filter_duration(Duration::from_micros(600));
        telemetry.absorb_terminal_prefix_clause_filter_duration(Duration::from_micros(700));
        telemetry.absorb_terminal_summary_build_duration(Duration::from_micros(400));
        telemetry.absorb_terminal_summary_build_duration(Duration::from_micros(800));
        telemetry.absorb_terminal_summary_kernel_timing(super::RemainingOneSummaryKernelTiming {
            terminal_summary_connectivity_duration: Duration::from_micros(550),
            terminal_summary_exact_nu_duration: Duration::from_micros(250),
            terminal_summary_aggregation_duration: Duration::from_micros(300),
            ..Default::default()
        });
        telemetry.absorb_terminal_summary_kernel_timing(super::RemainingOneSummaryKernelTiming {
            terminal_summary_connectivity_duration: Duration::from_micros(550),
            terminal_summary_exact_nu_duration: Duration::from_micros(850),
            terminal_summary_aggregation_duration: Duration::from_micros(800),
            ..Default::default()
        });

        assert_eq!(telemetry.terminal_prefix_clause_filter_micros, 1300);
        assert_eq!(telemetry.terminal_prefix_clause_filter_millis, 1);
        assert_eq!(telemetry.terminal_summary_build_micros, 1200);
        assert_eq!(telemetry.terminal_summary_build_millis, 1);
        assert_eq!(telemetry.terminal_summary_connectivity_micros, 1100);
        assert_eq!(telemetry.terminal_summary_connectivity_millis, 1);
        assert_eq!(telemetry.terminal_summary_exact_nu_micros, 1100);
        assert_eq!(telemetry.terminal_summary_exact_nu_millis, 1);
        assert_eq!(telemetry.terminal_summary_aggregation_micros, 1100);
        assert_eq!(telemetry.terminal_summary_aggregation_millis, 1);
    }

    #[test]
    fn claim_remaining_one_algebraic_ceiling_keeps_reference_step_four_winner_prefix() {
        let (library, history, nu_history) = reference_history_until(3);

        let admissibility =
            strict_admissibility_for_mode(4, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let objective_bar = compute_bar(2, 4, &history).bar;
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 3);
        let prefix = Telescope::new(Telescope::reference(4).clauses[..2].to_vec());
        let signature = PrefixSignature::new(4, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            3,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));

        let work_item = create_online_prefix_work_item(
            3,
            prefix,
            signature,
            &library,
            admissibility,
            &clause_catalog,
            &mut cache,
        );
        assert_eq!(work_item.remaining_clause_slots, 1);
        assert!(
            !super::claim_remaining_one_algebraic_nu_ceiling_cannot_clear_bar(
                &library,
                admissibility,
                objective_bar,
                &nu_history,
                &work_item.prefix_telescope,
            )
        );

        let mut telemetry = super::RemainingOneTelemetry::default();
        let mut budget = 64;
        let decision = exact_partial_prefix_bound_decision(
            4,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut cache,
            &mut budget,
            None,
            Some(&mut telemetry),
            None,
        );

        assert_eq!(
            decision,
            super::ExactPartialPrefixBoundDecision::CanClearBar
        );
        assert_eq!(telemetry.remaining_one_algebraic_prunes, 0);
        assert_eq!(telemetry.terminal_summary_plateau_activations, 0);
    }

    #[test]
    fn claim_remaining_one_plateau_activation_marker_tracks_first_matching_surface() {
        let mut telemetry = super::RemainingOneTelemetry::default();

        telemetry.note_terminal_summary_plateau_activation(Some(
            super::RemainingOneSummaryKernelActivationContext {
                prefix_states_explored: 24,
                prefix_cache_groups: 19,
                prefix_cache_candidates: 53_693,
            },
        ));
        assert_eq!(telemetry.terminal_summary_plateau_activations, 0);
        assert_eq!(
            telemetry.terminal_summary_first_plateau_activation_prefix_state,
            0
        );

        telemetry.note_terminal_summary_plateau_activation(Some(
            super::RemainingOneSummaryKernelActivationContext {
                prefix_states_explored: 43,
                prefix_cache_groups: super::CLAIM_REMAINING_ONE_RETAINED_PLATEAU_GROUPS,
                prefix_cache_candidates: super::CLAIM_REMAINING_ONE_RETAINED_PLATEAU_CANDIDATES,
            },
        ));
        telemetry.note_terminal_summary_plateau_activation(Some(
            super::RemainingOneSummaryKernelActivationContext {
                prefix_states_explored: 52,
                prefix_cache_groups: super::CLAIM_REMAINING_ONE_RETAINED_PLATEAU_GROUPS,
                prefix_cache_candidates: super::CLAIM_REMAINING_ONE_RETAINED_PLATEAU_CANDIDATES,
            },
        ));

        assert_eq!(telemetry.terminal_summary_plateau_activations, 2);
        assert_eq!(
            telemetry.terminal_summary_first_plateau_activation_prefix_state,
            43
        );
    }

    #[test]
    fn claim_cached_rank_preprune_consumes_cached_summary_before_materialization() {
        let steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut history: Vec<DiscoveryRecord> = Vec::new();
        let mut nu_history = Vec::new();

        for step in &steps {
            history.push(DiscoveryRecord::new(
                step.step_index,
                u32::from(step.accepted.nu),
                u32::from(step.accepted.clause_kappa),
            ));
            nu_history.push((step.step_index, u32::from(step.accepted.nu)));
            library.push(LibraryEntry::from_telescope(&step.telescope, &library));
        }

        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let objective_bar = compute_bar(2, 15, &history).bar;
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut discovery = super::RealisticShadowDiscovery::default();

        assert!(discovery.prefix_legality_cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));

        let work_item = create_online_prefix_work_item(
            8,
            prefix,
            signature.clone(),
            &library,
            admissibility,
            &clause_catalog,
            &mut discovery.prefix_legality_cache,
        );
        assert_eq!(work_item.remaining_clause_slots, 1);

        let mut budget = 64;
        let decision = exact_partial_prefix_bound_decision(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut discovery.prefix_legality_cache,
            &mut budget,
            None,
            None,
            None,
        );
        assert_ne!(decision, super::ExactPartialPrefixBoundDecision::Unknown);
        assert_eq!(
            discovery
                .prefix_legality_cache
                .entry_counts()
                .terminal_prefix_completions,
            1
        );

        discovery.terminal_rank_incumbent = Some(
            discovery
                .prefix_legality_cache
                .terminal_prefix_rank_summary(&signature)
                .and_then(|summary| summary.best_accept_rank)
                .expect("terminal prefix should cache an exact best accept rank"),
        );

        assert!(
            discovery
                .prefix_legality_cache
                .terminal_prefix_completion_summary(&signature)
                .expect("terminal prefix should cache a pruning summary")
                .evaluations
                .is_none()
        );

        assert!(super::claim_try_summary_prune_before_materialization(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &work_item.signature,
            &work_item.prefix_telescope,
            work_item.clause_kappa,
            work_item.next_clauses(&clause_catalog),
            work_item.next_clause_connectivity_facts(&clause_catalog),
            work_item.next_clause_nu_facts(&clause_catalog),
            &mut discovery,
        ));
        assert_eq!(
            discovery
                .prefix_legality_cache
                .entry_counts()
                .terminal_prefix_completions,
            0,
            "claim cached rank pre-prune should consume the cached terminal payload"
        );
        assert_eq!(
            discovery
                .remaining_one_telemetry
                .remaining_one_cached_rank_prunes,
            1
        );
        assert_eq!(
            discovery
                .remaining_one_telemetry
                .remaining_one_rank_prunes_pre_materialize,
            1
        );
        assert!(discovery.raw_generated_surface > 0);
        assert!(discovery.enumerated_candidates > 0);
        assert!(discovery.terminal_rank_prunes > 0);
    }

    #[test]
    fn claim_summary_build_skips_full_rank_for_primary_dominated_incumbent() {
        let steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut history: Vec<DiscoveryRecord> = Vec::new();
        let mut nu_history = Vec::new();

        for step in &steps {
            history.push(DiscoveryRecord::new(
                step.step_index,
                u32::from(step.accepted.nu),
                u32::from(step.accepted.clause_kappa),
            ));
            nu_history.push((step.step_index, u32::from(step.accepted.nu)));
            library.push(LibraryEntry::from_telescope(&step.telescope, &library));
        }

        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let objective_bar = compute_bar(2, 15, &history).bar;
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut discovery = super::RealisticShadowDiscovery::default();

        assert!(discovery.prefix_legality_cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));

        let work_item = create_online_prefix_work_item(
            8,
            prefix,
            signature.clone(),
            &library,
            admissibility,
            &clause_catalog,
            &mut discovery.prefix_legality_cache,
        );
        assert_eq!(work_item.remaining_clause_slots, 1);

        let incumbent = test_accept_rank(0, "incumbent");
        discovery.terminal_rank_incumbent = Some(incumbent.clone());

        let mut budget = 64;
        let decision = exact_partial_prefix_bound_decision(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut discovery.prefix_legality_cache,
            &mut budget,
            Some(&incumbent),
            None,
            None,
        );
        assert_eq!(
            decision,
            super::ExactPartialPrefixBoundDecision::CanClearBar
        );

        let summary = discovery
            .prefix_legality_cache
            .terminal_prefix_completion_summary(&signature)
            .expect("terminal prefix should cache a pruning summary");
        assert!(
            summary.best_accept_primary_rank.is_some(),
            "summary should still record the coarse best accept rank"
        );
        assert!(
            summary.best_accept_rank.is_none(),
            "primary-dominated incumbent should skip full accept rank construction"
        );

        assert!(super::claim_try_summary_prune_before_materialization(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &work_item.signature,
            &work_item.prefix_telescope,
            work_item.clause_kappa,
            work_item.next_clauses(&clause_catalog),
            work_item.next_clause_connectivity_facts(&clause_catalog),
            work_item.next_clause_nu_facts(&clause_catalog),
            &mut discovery,
        ));
        assert_eq!(
            discovery
                .prefix_legality_cache
                .entry_counts()
                .terminal_prefix_completions,
            0,
            "claim primary-rank pre-prune should consume the cached terminal payload"
        );
        assert_eq!(
            discovery
                .remaining_one_telemetry
                .remaining_one_cached_rank_prunes,
            1
        );
        assert_eq!(
            discovery
                .remaining_one_telemetry
                .remaining_one_rank_prunes_pre_materialize,
            1
        );
    }

    #[test]
    fn claim_compact_summary_reuses_survivor_sketch_for_multiple_bar_clearing_primary_ranks() {
        let steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut nu_history = Vec::new();

        for step_index in 2..=15 {
            if step_index > 2 {
                let previous_step =
                    &steps[usize::try_from(step_index - 3).expect("step index exceeded usize")];
                nu_history.push((
                    previous_step.step_index,
                    u32::from(previous_step.accepted.nu),
                ));
                library.push(LibraryEntry::from_telescope(
                    &previous_step.telescope,
                    &library,
                ));
            }

            let admissibility = strict_admissibility_for_mode(
                step_index,
                2,
                &library,
                AdmissibilityMode::DesktopClaimShadow,
            );
            let target = Telescope::reference(step_index);
            let prefix_len = target.clauses.len().saturating_sub(1);
            let target_kappa = u16::try_from(target.kappa()).expect("kappa exceeded u16");
            let prefix = Telescope::new(target.clauses[..prefix_len].to_vec());
            let signature = PrefixSignature::new(step_index, &library, &prefix);
            let mut cache = PrefixLegalityCache::default();

            if !cache.insert_root(
                signature.clone(),
                target_kappa,
                &library,
                &prefix,
                admissibility,
                LateFamilySurface::ClaimGeneric,
            ) {
                continue;
            }

            let context = EnumerationContext::from_admissibility(&library, admissibility);
            let clause_catalog = build_clause_catalog(context, target_kappa);
            let terminal_clauses = super::terminal_prefix_clause_candidates(
                step_index,
                &library,
                admissibility,
                &signature,
                clause_catalog.clauses_at(prefix_len),
                Some(clause_catalog.terminal_connectivity_facts_at(prefix_len)),
                clause_catalog.terminal_nu_facts_at(prefix_len),
                &mut cache,
                None,
            );
            if terminal_clauses.len() == 0 {
                continue;
            }

            let objective_bar = Rational::zero();
            let mut direct_best_primary_rank = None;
            let mut direct_best_accept_rank = None;
            let mut distinct_primary_ranks = Vec::new();

            match &terminal_clauses {
                super::TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand(clauses) => {
                    for clause in clauses {
                        let mut telescope = prefix.clone();
                        telescope.clauses.push(clause.clause.clone());

                        if !passes_connectivity(&library, &telescope) {
                            continue;
                        }

                        let exact_nu =
                            u16::try_from(structural_nu(&telescope, &library, &nu_history).total)
                                .expect("nu exceeded u16");
                        let clause_kappa_used =
                            u16::try_from(telescope.kappa()).expect("kappa exceeded u16");
                        let Some(primary_rank) = super::terminal_prefix_primary_rank(
                            objective_bar,
                            exact_nu,
                            clause_kappa_used,
                        ) else {
                            continue;
                        };
                        if !distinct_primary_ranks.contains(&primary_rank) {
                            distinct_primary_ranks.push(primary_rank.clone());
                        }
                        match &direct_best_primary_rank {
                            Some(current)
                                if !super::better_terminal_prefix_primary_rank(
                                    &primary_rank,
                                    current,
                                ) => {}
                            _ => direct_best_primary_rank = Some(primary_rank),
                        }

                        let bit_kappa_used =
                            u16::try_from(pen_core::encode::telescope_bit_cost(&telescope))
                                .expect("bit cost exceeded u16");
                        let rank = super::acceptance_rank_for_telescope(
                            objective_bar,
                            &telescope,
                            exact_nu,
                            bit_kappa_used,
                            clause_kappa_used,
                        )
                        .expect("bar-clearing direct candidate should produce an accept rank");
                        match &direct_best_accept_rank {
                            Some(current) if !super::better_rank(&rank, current) => {}
                            _ => direct_best_accept_rank = Some(rank),
                        }
                    }
                }
                super::TerminalPrefixClauseCandidates::General(clauses) => {
                    for clause in clauses {
                        let mut telescope = prefix.clone();
                        telescope.clauses.push(clause.clause.clone());

                        if !passes_connectivity(&library, &telescope) {
                            continue;
                        }

                        let admissibility_decision = clause
                            .cached_admissibility_decision
                            .clone()
                            .unwrap_or_else(|| {
                                assess_strict_admissibility(
                                    step_index,
                                    &library,
                                    &telescope,
                                    admissibility,
                                )
                            });
                        if !admissibility_decision.is_admitted() {
                            continue;
                        }

                        let exact_nu =
                            u16::try_from(structural_nu(&telescope, &library, &nu_history).total)
                                .expect("nu exceeded u16");
                        let clause_kappa_used =
                            u16::try_from(telescope.kappa()).expect("kappa exceeded u16");
                        let Some(primary_rank) = super::terminal_prefix_primary_rank(
                            objective_bar,
                            exact_nu,
                            clause_kappa_used,
                        ) else {
                            continue;
                        };
                        if !super::terminal_completion_can_compete_for_acceptance(
                            &signature,
                            admissibility,
                            &cache,
                            &admissibility_decision,
                        ) {
                            continue;
                        }
                        if !distinct_primary_ranks.contains(&primary_rank) {
                            distinct_primary_ranks.push(primary_rank.clone());
                        }
                        match &direct_best_primary_rank {
                            Some(current)
                                if !super::better_terminal_prefix_primary_rank(
                                    &primary_rank,
                                    current,
                                ) => {}
                            _ => direct_best_primary_rank = Some(primary_rank),
                        }

                        let bit_kappa_used =
                            u16::try_from(pen_core::encode::telescope_bit_cost(&telescope))
                                .expect("bit cost exceeded u16");
                        let rank = super::acceptance_rank_for_telescope(
                            objective_bar,
                            &telescope,
                            exact_nu,
                            bit_kappa_used,
                            clause_kappa_used,
                        )
                        .expect("bar-clearing direct candidate should produce an accept rank");
                        match &direct_best_accept_rank {
                            Some(current) if !super::better_rank(&rank, current) => {}
                            _ => direct_best_accept_rank = Some(rank),
                        }
                    }
                }
            }

            if distinct_primary_ranks.len() <= 1 {
                continue;
            }

            let summary = super::compute_terminal_prefix_completion_summary_from_candidates(
                step_index,
                &library,
                admissibility,
                objective_bar,
                &nu_history,
                &signature,
                &prefix,
                super::TerminalPrefixSummaryPayload::Compact,
                terminal_clauses,
                None,
                &mut cache,
                None,
                None,
            );

            assert!(
                summary
                    .compact_survivor_sketch
                    .as_ref()
                    .is_some_and(|sketch| sketch.survivors.len() >= distinct_primary_ranks.len()),
                "multiple bar-clearing primary ranks should now keep an incumbent-relevant survivor sketch"
            );
            assert_eq!(summary.best_accept_primary_rank, direct_best_primary_rank);
            assert_eq!(summary.best_accept_rank, direct_best_accept_rank);
            return;
        }

        panic!(
            "expected at least one claim remaining-one surface with multiple bar-clearing primary ranks"
        );
    }

    #[test]
    fn claim_materialization_reuses_cached_survivor_sketch_for_multiple_primary_ranks() {
        for step_index in 4..=15 {
            let (library, history, nu_history) = reference_history_until(step_index - 1);
            let admissibility = strict_admissibility_for_mode(
                step_index,
                2,
                &library,
                AdmissibilityMode::DesktopClaimShadow,
            );
            let objective_bar = compute_bar(2, step_index, &history).bar;
            let target = Telescope::reference(step_index);
            let prefix_len = target.clauses.len().saturating_sub(1);
            let target_kappa = u16::try_from(target.kappa()).expect("kappa exceeded u16");
            let prefix = Telescope::new(target.clauses[..prefix_len].to_vec());
            let signature = PrefixSignature::new(step_index, &library, &prefix);
            let context = EnumerationContext::from_admissibility(&library, admissibility);
            let clause_catalog = build_clause_catalog(context, target_kappa);
            let mut cache = PrefixLegalityCache::default();

            if !cache.insert_root(
                signature.clone(),
                target_kappa,
                &library,
                &prefix,
                admissibility,
                LateFamilySurface::ClaimGeneric,
            ) {
                continue;
            }

            let work_item = create_online_prefix_work_item(
                target_kappa,
                prefix.clone(),
                signature.clone(),
                &library,
                admissibility,
                &clause_catalog,
                &mut cache,
            );
            let terminal_clauses = super::terminal_prefix_clause_candidates(
                step_index,
                &library,
                admissibility,
                &signature,
                work_item.next_clauses(&clause_catalog),
                work_item.next_clause_connectivity_facts(&clause_catalog),
                work_item.next_clause_nu_facts(&clause_catalog),
                &mut cache,
                None,
            );
            let mut distinct_primary_ranks = Vec::new();
            match &terminal_clauses {
                super::TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand(clauses) => {
                    for clause in clauses {
                        let mut telescope = prefix.clone();
                        telescope.clauses.push(clause.clause.clone());
                        if !passes_connectivity(&library, &telescope) {
                            continue;
                        }
                        let exact_nu =
                            u16::try_from(structural_nu(&telescope, &library, &nu_history).total)
                                .expect("nu exceeded u16");
                        let clause_kappa_used =
                            u16::try_from(telescope.kappa()).expect("kappa exceeded u16");
                        let Some(primary_rank) = super::terminal_prefix_primary_rank(
                            objective_bar,
                            exact_nu,
                            clause_kappa_used,
                        ) else {
                            continue;
                        };
                        if !distinct_primary_ranks.contains(&primary_rank) {
                            distinct_primary_ranks.push(primary_rank);
                        }
                    }
                }
                super::TerminalPrefixClauseCandidates::General(clauses) => {
                    for clause in clauses {
                        let mut telescope = prefix.clone();
                        telescope.clauses.push(clause.clause.clone());
                        if !passes_connectivity(&library, &telescope) {
                            continue;
                        }
                        let admissibility_decision = clause
                            .cached_admissibility_decision
                            .clone()
                            .unwrap_or_else(|| {
                                assess_strict_admissibility(
                                    step_index,
                                    &library,
                                    &telescope,
                                    admissibility,
                                )
                            });
                        if !admissibility_decision.is_admitted()
                            || !super::terminal_completion_can_compete_for_acceptance(
                                &signature,
                                admissibility,
                                &cache,
                                &admissibility_decision,
                            )
                        {
                            continue;
                        }
                        let exact_nu =
                            u16::try_from(structural_nu(&telescope, &library, &nu_history).total)
                                .expect("nu exceeded u16");
                        let clause_kappa_used =
                            u16::try_from(telescope.kappa()).expect("kappa exceeded u16");
                        let Some(primary_rank) = super::terminal_prefix_primary_rank(
                            objective_bar,
                            exact_nu,
                            clause_kappa_used,
                        ) else {
                            continue;
                        };
                        if !distinct_primary_ranks.contains(&primary_rank) {
                            distinct_primary_ranks.push(primary_rank);
                        }
                    }
                }
            }
            if distinct_primary_ranks.len() <= 1 {
                continue;
            }

            let summary = super::compute_terminal_prefix_completion_summary_from_candidates(
                step_index,
                &library,
                admissibility,
                objective_bar,
                &nu_history,
                &signature,
                &prefix,
                super::TerminalPrefixSummaryPayload::Compact,
                terminal_clauses,
                None,
                &mut cache,
                None,
                None,
            );
            assert!(
                summary.compact_survivor_sketch.is_some(),
                "multiple primary ranks should now keep a cached survivor sketch"
            );

            let mut sketch_discovery = super::RealisticShadowDiscovery::default();
            let mut direct_discovery = super::RealisticShadowDiscovery::default();
            assert!(sketch_discovery.prefix_legality_cache.insert_root(
                signature.clone(),
                target_kappa,
                &library,
                &prefix,
                admissibility,
                LateFamilySurface::ClaimGeneric
            ));
            assert!(direct_discovery.prefix_legality_cache.insert_root(
                signature.clone(),
                target_kappa,
                &library,
                &prefix,
                admissibility,
                LateFamilySurface::ClaimGeneric
            ));

            sketch_discovery
                .prefix_legality_cache
                .store_terminal_prefix_completion_summary(signature.clone(), summary);

            let sketch_group = super::materialize_terminal_prefix_group(
                step_index,
                &library,
                admissibility,
                objective_bar,
                &nu_history,
                &signature,
                &prefix,
                work_item.next_clauses(&clause_catalog),
                work_item.next_clause_connectivity_facts(&clause_catalog),
                work_item.next_clause_nu_facts(&clause_catalog),
                &mut sketch_discovery,
            )
            .expect("sketch materialization should succeed");
            let direct_group = super::materialize_terminal_prefix_group_compact(
                step_index,
                &library,
                admissibility,
                objective_bar,
                &nu_history,
                &signature,
                &prefix,
                work_item.next_clauses(&clause_catalog),
                work_item.next_clause_connectivity_facts(&clause_catalog),
                work_item.next_clause_nu_facts(&clause_catalog),
                &mut direct_discovery,
            )
            .expect("direct compact materialization should succeed");

            let sketch_candidates = sketch_group
                .candidates
                .iter()
                .map(|candidate| (candidate.telescope.clone(), candidate.accept_rank.clone()))
                .collect::<Vec<_>>();
            let direct_survivors = direct_group
                .candidates
                .iter()
                .filter(|candidate| candidate.accept_rank.is_some())
                .map(|candidate| (candidate.telescope.clone(), candidate.accept_rank.clone()))
                .collect::<Vec<_>>();

            assert_eq!(
                sketch_group.generated_terminal_candidates,
                direct_group.generated_terminal_candidates
            );
            assert_eq!(
                sketch_group.admissible_terminal_candidates,
                direct_group.admissible_terminal_candidates
            );
            assert_eq!(sketch_group.bound, direct_group.bound);
            assert_eq!(sketch_group.best_accept_rank, direct_group.best_accept_rank);
            assert_eq!(sketch_candidates, direct_survivors);
            assert_eq!(
                sketch_discovery
                    .remaining_one_telemetry
                    .remaining_one_materialized_from_cached_summary,
                1
            );
            assert_eq!(
                sketch_discovery
                    .remaining_one_telemetry
                    .remaining_one_materialized_compact_direct,
                0
            );
            return;
        }

        panic!("expected at least one claim remaining-one surface with multiple primary ranks");
    }

    #[test]
    fn claim_materialization_reuses_cached_survivor_sketch_when_available() {
        for step_index in 4..=15 {
            let (library, history, nu_history) = reference_history_until(step_index - 1);
            let admissibility = strict_admissibility_for_mode(
                step_index,
                2,
                &library,
                AdmissibilityMode::DesktopClaimShadow,
            );
            let objective_bar = compute_bar(2, step_index, &history).bar;
            let target = Telescope::reference(step_index);
            let prefix_len = target.clauses.len().saturating_sub(1);
            let target_kappa = u16::try_from(target.kappa()).expect("kappa exceeded u16");
            let prefix = Telescope::new(target.clauses[..prefix_len].to_vec());
            let signature = PrefixSignature::new(step_index, &library, &prefix);
            let context = EnumerationContext::from_admissibility(&library, admissibility);
            let clause_catalog = build_clause_catalog(context, target_kappa);
            let mut cache = PrefixLegalityCache::default();

            if !cache.insert_root(
                signature.clone(),
                target_kappa,
                &library,
                &prefix,
                admissibility,
                LateFamilySurface::ClaimGeneric,
            ) {
                continue;
            }

            let work_item = create_online_prefix_work_item(
                target_kappa,
                prefix.clone(),
                signature.clone(),
                &library,
                admissibility,
                &clause_catalog,
                &mut cache,
            );
            let summary = super::compute_terminal_prefix_completion_summary_from_candidates(
                step_index,
                &library,
                admissibility,
                objective_bar,
                &nu_history,
                &signature,
                &prefix,
                super::TerminalPrefixSummaryPayload::Compact,
                super::terminal_prefix_clause_candidates(
                    step_index,
                    &library,
                    admissibility,
                    &signature,
                    work_item.next_clauses(&clause_catalog),
                    work_item.next_clause_connectivity_facts(&clause_catalog),
                    work_item.next_clause_nu_facts(&clause_catalog),
                    &mut cache,
                    None,
                ),
                None,
                &mut cache,
                None,
                None,
            );
            let Some(sketch) = summary.compact_survivor_sketch.clone() else {
                continue;
            };

            let mut sketch_discovery = super::RealisticShadowDiscovery::default();
            let mut direct_discovery = super::RealisticShadowDiscovery::default();
            assert!(sketch_discovery.prefix_legality_cache.insert_root(
                signature.clone(),
                target_kappa,
                &library,
                &prefix,
                admissibility,
                LateFamilySurface::ClaimGeneric
            ));
            assert!(direct_discovery.prefix_legality_cache.insert_root(
                signature.clone(),
                target_kappa,
                &library,
                &prefix,
                admissibility,
                LateFamilySurface::ClaimGeneric
            ));

            sketch_discovery
                .prefix_legality_cache
                .store_terminal_prefix_completion_summary(signature.clone(), summary.clone());

            let sketch_group = super::materialize_terminal_prefix_group(
                step_index,
                &library,
                admissibility,
                objective_bar,
                &nu_history,
                &signature,
                &prefix,
                work_item.next_clauses(&clause_catalog),
                work_item.next_clause_connectivity_facts(&clause_catalog),
                work_item.next_clause_nu_facts(&clause_catalog),
                &mut sketch_discovery,
            )
            .expect("sketch materialization should succeed");
            let direct_group = super::materialize_terminal_prefix_group_compact(
                step_index,
                &library,
                admissibility,
                objective_bar,
                &nu_history,
                &signature,
                &prefix,
                work_item.next_clauses(&clause_catalog),
                work_item.next_clause_connectivity_facts(&clause_catalog),
                work_item.next_clause_nu_facts(&clause_catalog),
                &mut direct_discovery,
            )
            .expect("direct compact materialization should succeed");

            let sketch_candidates = sketch_group
                .candidates
                .iter()
                .map(|candidate| (candidate.telescope.clone(), candidate.accept_rank.clone()))
                .collect::<Vec<_>>();
            let direct_survivors = direct_group
                .candidates
                .iter()
                .filter(|candidate| candidate.accept_rank.is_some())
                .map(|candidate| (candidate.telescope.clone(), candidate.accept_rank.clone()))
                .collect::<Vec<_>>();

            assert_eq!(
                sketch_group.generated_terminal_candidates,
                direct_group.generated_terminal_candidates
            );
            assert_eq!(
                sketch_group.admissible_terminal_candidates,
                direct_group.admissible_terminal_candidates
            );
            assert_eq!(sketch_group.bound, direct_group.bound);
            assert_eq!(sketch_group.best_accept_rank, direct_group.best_accept_rank);
            assert_eq!(sketch_candidates, direct_survivors);
            assert_eq!(sketch_candidates.len(), sketch.survivors.len());
            assert_eq!(
                sketch_discovery
                    .remaining_one_telemetry
                    .remaining_one_materialized_from_cached_summary,
                1
            );
            assert_eq!(
                sketch_discovery
                    .remaining_one_telemetry
                    .remaining_one_materialized_compact_direct,
                0
            );
            return;
        }

        panic!("expected at least one claim remaining-one surface with a survivor sketch");
    }

    #[test]
    fn claim_materialization_tracks_direct_compact_path_without_cache() {
        let steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut history: Vec<DiscoveryRecord> = Vec::new();
        let mut nu_history = Vec::new();

        for step in &steps {
            history.push(DiscoveryRecord::new(
                step.step_index,
                u32::from(step.accepted.nu),
                u32::from(step.accepted.clause_kappa),
            ));
            nu_history.push((step.step_index, u32::from(step.accepted.nu)));
            library.push(LibraryEntry::from_telescope(&step.telescope, &library));
        }

        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let objective_bar = compute_bar(2, 15, &history).bar;
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut discovery = super::RealisticShadowDiscovery::default();

        assert!(discovery.prefix_legality_cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));

        let work_item = create_online_prefix_work_item(
            8,
            prefix,
            signature.clone(),
            &library,
            admissibility,
            &clause_catalog,
            &mut discovery.prefix_legality_cache,
        );
        assert_eq!(work_item.remaining_clause_slots, 1);

        let group = super::materialize_terminal_prefix_group(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &work_item.signature,
            &work_item.prefix_telescope,
            work_item.next_clauses(&clause_catalog),
            work_item.next_clause_connectivity_facts(&clause_catalog),
            work_item.next_clause_nu_facts(&clause_catalog),
            &mut discovery,
        )
        .expect("materialization should succeed");

        assert!(group.generated_terminal_candidates >= group.admissible_terminal_candidates);
        assert_eq!(
            discovery
                .remaining_one_telemetry
                .remaining_one_materialized_from_cached_summary,
            0
        );
        assert_eq!(
            discovery
                .remaining_one_telemetry
                .remaining_one_materialized_compact_direct,
            1
        );
    }

    #[test]
    fn claim_compact_materialization_matches_summary_expansion_without_cache() {
        let steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut history: Vec<DiscoveryRecord> = Vec::new();
        let mut nu_history = Vec::new();

        for step in &steps {
            history.push(DiscoveryRecord::new(
                step.step_index,
                u32::from(step.accepted.nu),
                u32::from(step.accepted.clause_kappa),
            ));
            nu_history.push((step.step_index, u32::from(step.accepted.nu)));
            library.push(LibraryEntry::from_telescope(&step.telescope, &library));
        }

        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let objective_bar = compute_bar(2, 15, &history).bar;
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut summary_discovery = super::RealisticShadowDiscovery::default();
        let mut compact_discovery = super::RealisticShadowDiscovery::default();

        assert!(summary_discovery.prefix_legality_cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));
        assert!(compact_discovery.prefix_legality_cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::ClaimGeneric
        ));

        let summary_work_item = create_online_prefix_work_item(
            8,
            prefix.clone(),
            signature.clone(),
            &library,
            admissibility,
            &clause_catalog,
            &mut summary_discovery.prefix_legality_cache,
        );
        let compact_work_item = create_online_prefix_work_item(
            8,
            prefix,
            signature,
            &library,
            admissibility,
            &clause_catalog,
            &mut compact_discovery.prefix_legality_cache,
        );

        let terminal_clauses = terminal_prefix_clause_candidates(
            15,
            &library,
            admissibility,
            &summary_work_item.signature,
            summary_work_item.next_clauses(&clause_catalog),
            summary_work_item.next_clause_connectivity_facts(&clause_catalog),
            summary_work_item.next_clause_nu_facts(&clause_catalog),
            &mut summary_discovery.prefix_legality_cache,
            None,
        );
        let summary = super::compute_terminal_prefix_completion_summary_from_candidates(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &summary_work_item.signature,
            &summary_work_item.prefix_telescope,
            super::TerminalPrefixSummaryPayload::Full,
            terminal_clauses,
            None,
            &mut summary_discovery.prefix_legality_cache,
            None,
            None,
        );
        let summary_group = super::materialize_terminal_prefix_group_from_summary(
            admissibility,
            objective_bar,
            &summary_work_item.signature,
            summary,
            &mut summary_discovery,
        );
        let compact_group = super::materialize_terminal_prefix_group_compact(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &compact_work_item.signature,
            &compact_work_item.prefix_telescope,
            compact_work_item.next_clauses(&clause_catalog),
            compact_work_item.next_clause_connectivity_facts(&clause_catalog),
            compact_work_item.next_clause_nu_facts(&clause_catalog),
            &mut compact_discovery,
        )
        .expect("compact materialization should succeed");

        let summary_candidates = summary_group
            .candidates
            .iter()
            .map(|candidate| (candidate.telescope.clone(), candidate.accept_rank.clone()))
            .collect::<Vec<_>>();
        let compact_candidates = compact_group
            .candidates
            .iter()
            .map(|candidate| (candidate.telescope.clone(), candidate.accept_rank.clone()))
            .collect::<Vec<_>>();

        assert_eq!(
            compact_group.generated_terminal_candidates,
            summary_group.generated_terminal_candidates
        );
        assert_eq!(
            compact_group.admissible_terminal_candidates,
            summary_group.admissible_terminal_candidates
        );
        assert_eq!(compact_group.bound, summary_group.bound);
        assert_eq!(
            compact_group.best_accept_rank,
            summary_group.best_accept_rank
        );
        assert_eq!(compact_candidates, summary_candidates);
        assert_eq!(
            compact_discovery.raw_generated_surface,
            summary_discovery.raw_generated_surface
        );
        assert_eq!(
            compact_discovery.enumerated_candidates,
            summary_discovery.enumerated_candidates
        );
        assert_eq!(
            compact_discovery.well_formed_candidates,
            summary_discovery.well_formed_candidates
        );
        assert_eq!(
            compact_discovery.admissibility_rejections,
            summary_discovery.admissibility_rejections
        );
    }

    #[test]
    fn realistic_shadow_keeps_semantic_family_bucket_taxonomy() {
        let realistic_step =
            profile_step_from_reference_prefix(10, SearchProfile::RealisticFrontierShadow);
        assert!(!realistic_step.demo_bucket_stats.is_empty());
        assert!(
            realistic_step.demo_bucket_stats.iter().all(|bucket| {
                bucket.bucket_key.taxonomy == SearchBucketTaxonomy::SemanticFamily
            })
        );
        assert!(
            realistic_step
                .demo_bucket_stats
                .iter()
                .any(|bucket| !bucket.bucket_label.contains("structural_generic"))
        );
    }

    #[test]
    fn demo_breadth_shadow_midstep_search_preserves_realistic_shadow_acceptance() {
        for step_index in 5..=9 {
            let realistic_step = profile_step_from_reference_prefix(
                step_index,
                SearchProfile::RealisticFrontierShadow,
            );
            let demo_step =
                profile_step_from_reference_prefix(step_index, SearchProfile::DemoBreadthShadow);

            assert_eq!(realistic_step.telescope, Telescope::reference(step_index));
            assert_eq!(demo_step.telescope, Telescope::reference(step_index));
            assert_eq!(
                demo_step.accepted.canonical_hash,
                realistic_step.accepted.canonical_hash
            );
        }
    }

    #[test]
    fn bootstrap_search_discovers_the_first_fifteen_reference_telescopes() {
        let steps = search_bootstrap_prefix(15, 2).expect("bootstrap search should succeed");
        assert_eq!(steps.len(), 15);
        assert_eq!(steps[0].telescope, Telescope::reference(1));
        assert_eq!(steps[1].telescope, Telescope::reference(2));
        assert_eq!(steps[2].telescope, Telescope::reference(3));
        assert_eq!(steps[3].telescope, Telescope::reference(4));
        assert_eq!(steps[4].telescope, Telescope::reference(5));
        assert_eq!(steps[5].telescope, Telescope::reference(6));
        assert_eq!(steps[6].telescope, Telescope::reference(7));
        assert_eq!(steps[7].telescope, Telescope::reference(8));
        assert_eq!(steps[8].telescope, Telescope::reference(9));
        assert_eq!(steps[9].telescope, Telescope::reference(10));
        assert_eq!(steps[10].telescope, Telescope::reference(11));
        assert_eq!(steps[11].telescope, Telescope::reference(12));
        assert_eq!(steps[12].telescope, Telescope::reference(13));
        assert_eq!(steps[13].telescope, Telescope::reference(14));
        assert_eq!(steps[14].telescope, Telescope::reference(15));
    }

    #[test]
    fn bootstrap_search_can_continue_from_stored_prefix_telescopes() {
        let prefix = (1..=4).map(Telescope::reference).collect::<Vec<_>>();
        let steps = search_bootstrap_from_prefix(&prefix, 6, 2)
            .expect("bootstrap continuation should succeed");

        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0].step_index, 5);
        assert_eq!(steps[0].telescope, Telescope::reference(5));
        assert_eq!(steps[1].step_index, 6);
        assert_eq!(steps[1].telescope, Telescope::reference(6));
    }

    #[test]
    fn reference_dct_becomes_admissible_and_connected_after_the_live_hilbert_prefix() {
        let steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut history: Vec<DiscoveryRecord> = Vec::new();
        let mut nu_history = Vec::new();

        for step in &steps {
            history.push(DiscoveryRecord::new(
                step.step_index,
                u32::from(step.accepted.nu),
                u32::from(step.accepted.clause_kappa),
            ));
            nu_history.push((step.step_index, u32::from(step.accepted.nu)));
            library.push(LibraryEntry::from_telescope(&step.telescope, &library));
        }

        let dct = Telescope::reference(15);
        let evaluated = evaluate_candidate(&library, &history, dct.clone())
            .expect("reference DCT should evaluate against the live 14-step prefix");
        let entry = LibraryEntry::from_telescope(&dct, &library);
        let objective_bar = compute_bar(2, 15, &history).bar;
        let admissibility = strict_admissibility(15, 2, &library);
        let connectivity = analyze_connectivity(&library, &dct);
        let minimality = analyze_semantic_minimality(
            15,
            objective_bar,
            admissibility,
            &dct,
            &library,
            &nu_history,
        );

        assert_eq!(evaluated.telescope_class, TelescopeClass::Synthesis);
        assert!(entry.capabilities.has_modal_ops);
        assert!(entry.capabilities.has_temporal_ops);
        assert_eq!(evaluated.clause_kappa, 8);
        assert_eq!(evaluated.nu, 103);
        assert_eq!(evaluated.rho, Rational::new(103, 8));
        assert_eq!(objective_bar, Rational::new(19520, 2639));
        assert_eq!(
            admissibility,
            StrictAdmissibility {
                mode: AdmissibilityMode::Guarded,
                min_clause_kappa: 8,
                max_clause_kappa: 8,
                ambient_depth: 2,
                max_expr_nodes: 7,
                max_path_dimension: 0,
                include_trunc: false,
                include_modal: true,
                include_temporal: true,
                quota_per_bucket: 64,
                require_former_eliminator_package: false,
                require_initial_hit_package: false,
                require_truncation_hit_package: false,
                require_higher_hit_package: false,
                require_sphere_lift_package: false,
                require_axiomatic_bundle_package: false,
                require_modal_shell_package: false,
                require_connection_shell_package: false,
                require_curvature_shell_package: false,
                require_operator_bundle_package: false,
                require_hilbert_functional_package: false,
                require_temporal_shell_package: true,
                package_policies: PackagePolicies {
                    temporal_shell: PackagePolicy::Require,
                    ..PackagePolicies::default()
                },
                focus_family: Some(StructuralFamily::TemporalShell),
                historical_anchor_ref: Some(10),
            }
        );
        assert!(passes_strict_admissibility(
            15,
            &library,
            &dct,
            admissibility
        ));
        assert_eq!(
            connectivity,
            ConnectivityWitness {
                connected: true,
                references_active_window: false,
                self_contained: false,
                max_lib_ref: 10,
                historical_reanchor: true,
            }
        );
        assert!(passes_connectivity(&library, &dct));
        assert!(minimality.is_minimal());
        assert!(minimality.admissible_bar_clear_subbundles.is_empty());
    }

    #[test]
    fn relaxed_shadow_switches_modal_step_ten_to_preference_based_admissibility() {
        let prefix = reference_prefix(9);
        let relaxed = search_bootstrap_from_prefix_for_profile_with_runtime(
            &prefix,
            10,
            2,
            SearchProfile::RelaxedShadow,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("relaxed bootstrap step should succeed");

        let relaxed_step = relaxed.last().expect("relaxed step");

        assert_eq!(relaxed_step.telescope, Telescope::reference(10));
        assert_eq!(
            relaxed_step.admissibility.mode,
            AdmissibilityMode::RelaxedShadow
        );
        assert_eq!(
            relaxed_step.admissibility.focus_family,
            Some(StructuralFamily::ModalShell)
        );
        assert_eq!(
            relaxed_step.admissibility.package_policies.modal_shell,
            PackagePolicy::Prefer
        );
        assert!(!relaxed_step.admissibility.require_modal_shell_package);
        assert!(relaxed_step.admissibility.historical_anchor_ref.is_some());
        assert!(
            relaxed_step
                .admissibility_diagnostics
                .admitted_focus_aligned
                > 0
        );
        assert!(
            relaxed_step
                .admissibility_diagnostics
                .admitted_deprioritized
                > 0
        );
    }

    #[test]
    fn relaxed_shadow_keeps_the_reference_telescope_admissible_through_step_fifteen() {
        let mut library: Library = Vec::new();

        for step_index in 1..=15 {
            let telescope = Telescope::reference(step_index);
            let admissibility = strict_admissibility_for_mode(
                step_index,
                2,
                &library,
                AdmissibilityMode::RelaxedShadow,
            );
            let decision =
                assess_strict_admissibility(step_index, &library, &telescope, admissibility);

            assert!(
                decision.is_admitted(),
                "reference step {step_index} should remain admissible in relaxed shadow, got {} ({})",
                decision.class.as_str(),
                decision.reason
            );

            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }
    }

    #[test]
    fn relaxed_shadow_step_ten_exposes_late_competition() {
        let step = relaxed_shadow_step_from_reference_prefix(10);

        assert_eq!(step.telescope, Telescope::reference(10));
        assert!(
            step.evaluated_candidates > 1,
            "expected relaxed shadow step 10 to evaluate competing candidates, got {}",
            step.evaluated_candidates
        );
        assert!(
            step.admissibility_diagnostics.admitted_deprioritized > 0,
            "expected relaxed shadow step 10 to admit de-prioritized competition"
        );
    }

    #[test]
    fn relaxed_shadow_step_eleven_exposes_late_competition() {
        let step = relaxed_shadow_step_from_reference_prefix(11);

        assert_eq!(step.telescope, Telescope::reference(11));
        assert!(
            step.evaluated_candidates > 1,
            "expected relaxed shadow step 11 to evaluate competing candidates, got {}",
            step.evaluated_candidates
        );
        assert!(
            step.admissibility_diagnostics.admitted_deprioritized > 0,
            "expected relaxed shadow step 11 to admit de-prioritized competition"
        );
    }

    #[test]
    fn relaxed_shadow_step_twelve_exposes_late_competition() {
        let step = relaxed_shadow_step_from_reference_prefix(12);

        assert_eq!(step.telescope, Telescope::reference(12));
        assert!(
            step.evaluated_candidates > 1,
            "expected relaxed shadow step 12 to evaluate competing candidates, got {}",
            step.evaluated_candidates
        );
        assert!(
            step.admissibility_diagnostics.admitted_deprioritized > 0,
            "expected relaxed shadow step 12 to admit de-prioritized competition"
        );
    }

    #[test]
    fn online_work_items_cache_exact_filtered_next_clauses() {
        let library = library_until(10);
        let admissibility =
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::RealisticShadow);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 5);
        let prefix = Telescope::new(Telescope::reference(11).clauses[..4].to_vec());
        let signature = PrefixSignature::new(11, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            5,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::RealisticShadow
        ));

        let work_item = create_online_prefix_work_item(
            5,
            prefix,
            signature,
            &library,
            admissibility,
            &clause_catalog,
            &mut cache,
        );

        assert_eq!(work_item.remaining_clause_slots, 1);
        assert_eq!(usize::from(work_item.remaining_family_options), 1);
        assert!(work_item.filtered_next_clauses.is_some());
        assert!(work_item.next_clause_count < clause_catalog.clauses_at(4).len());
        assert_eq!(
            work_item.next_clauses(&clause_catalog).len(),
            work_item.next_clause_count
        );
        assert_eq!(
            work_item.next_clause_nu_facts(&clause_catalog).len(),
            work_item.next_clause_count
        );
        for (clause, facts) in work_item
            .next_clauses(&clause_catalog)
            .iter()
            .zip(work_item.next_clause_nu_facts(&clause_catalog).iter())
        {
            assert_eq!(facts, &TerminalClauseNuFacts::from_clause(clause));
        }
        assert_eq!(cache.stats().active_window_clause_filter_hits, 1);
    }

    #[test]
    fn online_work_items_reuse_full_catalog_when_no_filter_applies() {
        let library = library_until(10);
        let admissibility =
            strict_admissibility_for_mode(11, 2, &library, AdmissibilityMode::DesktopClaimShadow);
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 5);
        let prefix = Telescope::new(Telescope::reference(11).clauses[..4].to_vec());
        let signature = PrefixSignature::new(11, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            5,
            &library,
            &prefix,
            admissibility,
            context.late_family_surface
        ));
        assert_eq!(cache.entry_counts().family_filters, 0);

        let work_item = create_online_prefix_work_item(
            5,
            prefix,
            signature,
            &library,
            admissibility,
            &clause_catalog,
            &mut cache,
        );

        assert!(work_item.filtered_next_clauses.is_none());
        assert_eq!(
            work_item.next_clause_count,
            clause_catalog.clauses_at(4).len()
        );
        assert_eq!(
            work_item.next_clauses(&clause_catalog).len(),
            clause_catalog.clauses_at(4).len()
        );
        assert_eq!(
            work_item.next_clause_nu_facts(&clause_catalog),
            clause_catalog.terminal_nu_facts_at(4)
        );
    }

    #[test]
    fn prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations() {
        let mut frontier = vec![
            dummy_work_item(2, 1, 1, 10, 2, "c"),
            dummy_work_item(1, 3, 2, 12, 3, "b"),
            dummy_work_item(1, 1, 1, 14, 3, "a"),
        ];

        assert_eq!(
            pop_best_prefix(&mut frontier)
                .expect("first work item should exist")
                .order_key
                .as_ref(),
            "a"
        );
        assert_eq!(
            pop_best_prefix(&mut frontier)
                .expect("second work item should exist")
                .order_key
                .as_ref(),
            "b"
        );
        assert_eq!(
            pop_best_prefix(&mut frontier)
                .expect("third work item should exist")
                .order_key
                .as_ref(),
            "c"
        );
    }

    #[test]
    fn exact_partial_prefix_bound_decision_reuses_cached_multistep_result() {
        let steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut history: Vec<DiscoveryRecord> = Vec::new();
        let mut nu_history = Vec::new();

        for step in &steps {
            history.push(DiscoveryRecord::new(
                step.step_index,
                u32::from(step.accepted.nu),
                u32::from(step.accepted.clause_kappa),
            ));
            nu_history.push((step.step_index, u32::from(step.accepted.nu)));
            library.push(LibraryEntry::from_telescope(&step.telescope, &library));
        }

        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::RealisticShadow);
        let objective_bar = compute_bar(2, 15, &history).bar;
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..6].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::RealisticShadow
        ));

        let work_item = create_online_prefix_work_item(
            8,
            prefix,
            signature,
            &library,
            admissibility,
            &clause_catalog,
            &mut cache,
        );
        assert!(work_item.remaining_clause_slots > 1);

        let mut first_budget = 64;
        let first = exact_partial_prefix_bound_decision(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut cache,
            &mut first_budget,
            None,
            None,
            None,
        );
        assert_ne!(first, super::ExactPartialPrefixBoundDecision::Unknown);

        let hits_before = cache.stats().partial_prefix_bound_hits;
        let mut second_budget = 1;
        let second = exact_partial_prefix_bound_decision(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut cache,
            &mut second_budget,
            None,
            None,
            None,
        );

        assert_eq!(second, first);
        assert_eq!(cache.stats().partial_prefix_bound_hits, hits_before + 1);
    }

    #[test]
    fn exact_partial_prefix_bound_decision_reuses_cached_terminal_result() {
        let steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut history: Vec<DiscoveryRecord> = Vec::new();
        let mut nu_history = Vec::new();

        for step in &steps {
            history.push(DiscoveryRecord::new(
                step.step_index,
                u32::from(step.accepted.nu),
                u32::from(step.accepted.clause_kappa),
            ));
            nu_history.push((step.step_index, u32::from(step.accepted.nu)));
            library.push(LibraryEntry::from_telescope(&step.telescope, &library));
        }

        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::RealisticShadow);
        let objective_bar = compute_bar(2, 15, &history).bar;
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::RealisticShadow
        ));

        let work_item = create_online_prefix_work_item(
            8,
            prefix,
            signature.clone(),
            &library,
            admissibility,
            &clause_catalog,
            &mut cache,
        );
        assert_eq!(work_item.remaining_clause_slots, 1);

        let mut first_budget = 64;
        let first = exact_partial_prefix_bound_decision(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut cache,
            &mut first_budget,
            None,
            None,
            None,
        );
        assert_ne!(first, super::ExactPartialPrefixBoundDecision::Unknown);

        let cached = cache
            .partial_prefix_bound_decision(&signature)
            .expect("terminal decision should be cached after the first exact check");
        assert_eq!(
            cached,
            match first {
                super::ExactPartialPrefixBoundDecision::CanClearBar => {
                    PartialPrefixBoundDecision::CanClearBar
                }
                super::ExactPartialPrefixBoundDecision::CannotClearBar => {
                    PartialPrefixBoundDecision::CannotClearBar
                }
                super::ExactPartialPrefixBoundDecision::Unknown => unreachable!(),
            }
        );
        let hits_before = cache.stats().partial_prefix_bound_hits;
        let terminal_summary_hits_before = cache.stats().terminal_prefix_completion_hits;

        let mut second_budget = 1;
        let second = exact_partial_prefix_bound_decision(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut cache,
            &mut second_budget,
            None,
            None,
            None,
        );

        assert_eq!(second, first);
        assert_eq!(cache.stats().partial_prefix_bound_hits, hits_before + 1);
        assert_eq!(
            cache.stats().terminal_prefix_completion_hits,
            terminal_summary_hits_before
        );
    }

    #[test]
    fn cached_terminal_prefix_rank_summary_prunes_without_reopening_completion_summary() {
        let steps = search_bootstrap_prefix(14, 2).expect("bootstrap search should succeed");
        let mut library: Library = Vec::new();
        let mut history: Vec<DiscoveryRecord> = Vec::new();
        let mut nu_history = Vec::new();

        for step in &steps {
            history.push(DiscoveryRecord::new(
                step.step_index,
                u32::from(step.accepted.nu),
                u32::from(step.accepted.clause_kappa),
            ));
            nu_history.push((step.step_index, u32::from(step.accepted.nu)));
            library.push(LibraryEntry::from_telescope(&step.telescope, &library));
        }

        let admissibility =
            strict_admissibility_for_mode(15, 2, &library, AdmissibilityMode::RealisticShadow);
        let objective_bar = compute_bar(2, 15, &history).bar;
        let context = EnumerationContext::from_admissibility(&library, admissibility);
        let clause_catalog = build_clause_catalog(context, 8);
        let prefix = Telescope::new(Telescope::reference(15).clauses[..7].to_vec());
        let signature = PrefixSignature::new(15, &library, &prefix);
        let mut cache = PrefixLegalityCache::default();

        assert!(cache.insert_root(
            signature.clone(),
            8,
            &library,
            &prefix,
            admissibility,
            LateFamilySurface::RealisticShadow
        ));

        let work_item = create_online_prefix_work_item(
            8,
            prefix,
            signature.clone(),
            &library,
            admissibility,
            &clause_catalog,
            &mut cache,
        );
        assert_eq!(work_item.remaining_clause_slots, 1);

        let mut budget = 64;
        let decision = exact_partial_prefix_bound_decision(
            15,
            &library,
            admissibility,
            objective_bar,
            &nu_history,
            &clause_catalog,
            &work_item,
            &mut cache,
            &mut budget,
            None,
            None,
            None,
        );
        assert_eq!(
            decision,
            super::ExactPartialPrefixBoundDecision::CanClearBar
        );

        let incumbent = cache
            .terminal_prefix_rank_summary(&signature)
            .and_then(|summary| summary.best_accept_rank)
            .expect("terminal prefix should cache an exact best accept rank");
        let rank_hits_before = cache.stats().terminal_prefix_rank_hits;
        let completion_hits_before = cache.stats().terminal_prefix_completion_hits;

        let pruned = super::cached_terminal_prefix_rank_prune_count(
            &signature,
            Some(&incumbent),
            &mut cache,
        )
        .expect("equal incumbent should prune the cached terminal prefix");

        assert_eq!(pruned, 1);
        assert_eq!(
            cache.stats().terminal_prefix_rank_hits,
            rank_hits_before + 1
        );
        assert_eq!(
            cache.stats().terminal_prefix_completion_hits,
            completion_hits_before
        );
    }

    #[test]
    fn realistic_shadow_step_eleven_prunes_impossible_family_hybrids_early() {
        let prefix = reference_prefix(10);
        let realistic = search_bootstrap_from_prefix_for_profile_with_runtime(
            &prefix,
            11,
            2,
            SearchProfile::RealisticFrontierShadow,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("realistic bootstrap step should succeed");
        let step = realistic.last().expect("realistic step");

        assert_eq!(step.step_index, 11);
        assert_eq!(step.telescope, Telescope::reference(11));
        assert!(step.incremental_clause_family_filter_hits > 0);
        assert!(
            step.incremental_clause_family_prunes
                + step.incremental_active_window_clause_filter_prunes
                > 0
        );
        assert!(step.incremental_active_window_clause_filter_hits > 0);
        assert!(step.incremental_active_window_clause_filter_prunes > 0);
        assert!(step.incremental_terminal_clause_filter_hits > 0);
        assert!(step.incremental_trivial_derivability_hits > 0);
        assert!(step.incremental_terminal_admissibility_hits > 0);
        assert!(step.incremental_partial_prefix_bound_hits > 0);
        assert!(step.incremental_partial_prefix_bound_checks > 0);
        assert!(
            step.incremental_partial_prefix_bound_prunes
                + step.incremental_terminal_prefix_bar_prunes
                > 0
        );
        assert!(
            step.admissibility_rejections
                + step
                    .admissibility_diagnostics
                    .structural_debt_cap_rejections
                + step.incremental_active_window_clause_filter_prunes
                > 0
        );
        assert!(step.prefix_states_explored <= 2);
        assert_eq!(step.full_telescopes_evaluated, 1);
    }

    #[test]
    fn realistic_shadow_step_thirteen_collapses_exact_remaining_two_surface() {
        let prefix = reference_prefix(12);
        let realistic = search_bootstrap_from_prefix_for_profile_with_runtime(
            &prefix,
            13,
            2,
            SearchProfile::RealisticFrontierShadow,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("realistic bootstrap step should succeed");
        let step = realistic.last().expect("realistic step");

        assert_eq!(step.step_index, 13);
        assert_eq!(step.telescope, Telescope::reference(13));
        assert_eq!(step.prefix_states_explored, 1);
        assert_eq!(step.incremental_partial_prefix_bound_checks, 1);
        assert_eq!(step.incremental_partial_prefix_bound_hits, 0);
        assert_eq!(step.incremental_terminal_prefix_completion_hits, 1);
        assert_eq!(step.incremental_terminal_prefix_rank_hits, 1);
        assert_eq!(step.incremental_terminal_rank_prunes, 1);
        assert_eq!(step.full_telescopes_evaluated, 1);
        assert_eq!(step.prefix_frontier_hot_states, 1);
        assert_eq!(step.frontier_window.total_len(), 1);
    }

    #[test]
    fn realistic_shadow_step_fifteen_builds_a_nonempty_terminal_prefix_frontier() {
        let prefix = reference_prefix(14);
        let realistic = search_bootstrap_from_prefix_for_profile_with_runtime(
            &prefix,
            15,
            2,
            SearchProfile::RealisticFrontierShadow,
            crate::diversify::FrontierRuntimeLimits::unlimited(),
        )
        .expect("realistic bootstrap step should succeed");
        let step = realistic.last().expect("realistic step");

        assert_eq!(step.step_index, 15);
        assert_eq!(step.telescope, Telescope::reference(15));
        assert!(step.prefixes_created >= step.prefix_states_explored);
        assert!(step.prefix_states_explored > step.frontier_window.total_len());
        assert_eq!(step.prefix_frontier_hot_states, 1);
        assert_eq!(step.prefix_frontier_cold_states, 0);
        assert_eq!(step.incremental_legality_cache_hits, 19);
        assert_eq!(step.incremental_connectivity_fallbacks, 0);
        assert!(step.incremental_clause_family_filter_hits > 0);
        assert_eq!(step.incremental_active_window_clause_filter_hits, 18);
        assert!(step.incremental_terminal_clause_filter_hits > 0);
        assert!(step.incremental_trivial_derivability_hits > 0);
        assert!(step.incremental_terminal_admissibility_hits > 0);
        assert_eq!(step.incremental_terminal_prefix_completion_hits, 2);
        assert!(step.incremental_terminal_prefix_rank_hits > 0);
        assert!(step.incremental_terminal_rank_prunes > 0);
        assert!(step.incremental_partial_prefix_bound_hits > 0);
        assert_eq!(step.incremental_partial_prefix_bound_checks, 3);
        assert_eq!(step.prefix_states_explored, 2);
        assert!(
            step.prefix_frontier_hot_states + step.prefix_frontier_cold_states
                <= step.prefix_states_explored
        );
        assert_eq!(step.full_telescopes_evaluated, 1);
        assert_eq!(step.full_telescopes_evaluated, step.evaluated_candidates);
        assert_eq!(step.canonical_dedupe_prunes, step.dedupe_prunes);
        assert_eq!(step.semantic_minimality_prunes, step.minimality_prunes);
        assert_eq!(step.frontier_window.total_len(), 1);
        assert_eq!(step.frontier_dedupe_keys.len(), 1);
    }
}
