use crate::accept::{
    AcceptanceOutcome, acceptance_rank, acceptance_rank_for_telescope, select_acceptance,
};
use crate::bounds::PrefixBound;
use crate::branch_bound::{AcceptRank, better_rank};
use crate::config::{DemoConfig, RuntimeConfig, SearchProfile};
use crate::diversify::{FrontierPressure, FrontierRuntimeLimits, plan_pressure_cold_retention};
use crate::enumerate::{
    ClauseCatalog, EnumerationContext, build_clause_catalog, enumerate_telescopes,
};
use crate::expand::{ExpandedCandidate, evaluate_candidate, evaluate_checked_candidate};
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
    PartialPrefixBoundDecision, PrefixLegalityCache, TerminalConnectivityDecision,
    TerminalPrefixClauseEvaluation, TerminalPrefixCompletion, TerminalPrefixCompletionSummary,
};
use crate::priority::{PriorityInputs, build_priority_key};
use crate::scheduler::build_schedule;
use crate::state::{FrontierStateRecV1, PrefixState};
use crate::worker::run_worker_batch;
use anyhow::{Result, bail};
use pen_core::encode::telescope_bit_cost;
use pen_core::ids::{ClauseId, ObligationSetId, StateId};
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_eval::bar::{DiscoveryRecord, compute_bar};
use pen_eval::minimality::analyze_semantic_minimality;
use pen_eval::nu::structural_nu;
use pen_store::manifest::{NearMiss, SearchTiming};
use pen_type::admissibility::{
    AdmissibilityDiagnostics, AdmissibilityMode, StrictAdmissibility, assess_strict_admissibility,
    strict_admissibility_for_mode,
};
use pen_type::check::{CheckResult, check_telescope};
use pen_type::connectivity::passes_connectivity;
use pen_type::obligations::{RetentionClass, RetentionPolicy, summarize_structural_debt};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::time::{Duration, Instant};

pub const LIVE_BOOTSTRAP_MAX_STEP: u32 = 15;
const MAX_PRUNE_SAMPLES: usize = 3;
const EXACT_PARTIAL_PREFIX_BOUND_BUDGET: usize = 32;
const DEMO_LATE_SPILL_START_STEP: u32 = 13;

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

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct DemoPhaseStats {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub full_eval_soft_cap: Option<u64>,
    #[serde(default)]
    pub materialize_full_evals: usize,
    #[serde(default)]
    pub proof_close_full_evals: usize,
    #[serde(default)]
    pub proof_close_overrun_full_evals: usize,
    #[serde(default)]
    pub materialize_soft_cap_triggered: bool,
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
    pub demo_phase: DemoPhaseStats,
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
    candidates: Vec<ExpandedCandidate>,
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
    partial_prefix_bound_checks: usize,
    partial_prefix_bound_prunes: usize,
    terminal_prefix_bar_prunes: usize,
}

impl RealisticShadowDiscovery {
    fn can_stop_for_budget(&self) -> bool {
        !self.candidates.is_empty() || !self.prefix_cache.is_empty()
    }
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
    next_clauses: Vec<pen_core::clause::ClauseRec>,
    order_key: String,
}

#[derive(Clone, Debug, Default)]
struct MaterializedTerminalPrefixGroup {
    candidates: Vec<PrefixGroupCandidate>,
    bound: Option<PrefixBound>,
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
}

impl DemoStepBudget {
    fn discovery_deadline(self, step_start: Instant) -> Option<Instant> {
        step_start.checked_add(Duration::from_millis(self.discovery_budget_millis))
    }
}

#[derive(Clone, Debug)]
struct DemoBudgetController {
    demo: DemoConfig,
    until_step: u32,
    proof_close_reserve_fraction: DemoBudgetFraction,
    scout_fraction: DemoBudgetFraction,
    consumed_total_millis: u64,
    consumed_early_millis: u64,
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
            };
        }

        let baseline_budget_millis = demo_step_floor_millis(&self.demo, step_index);
        let spill_budget_millis = if step_index >= DEMO_LATE_SPILL_START_STEP {
            let discretionary_budget_millis = remaining_total_millis
                .saturating_sub(self.remaining_late_baseline_budget_millis_from(step_index));
            discretionary_budget_millis / self.remaining_spill_steps(step_index)
        } else {
            0
        };
        let total_budget_millis = baseline_budget_millis
            .saturating_add(spill_budget_millis)
            .min(remaining_total_millis);
        let proof_close_reserve_millis =
            self.proof_close_reserve_fraction.apply(total_budget_millis);
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
        }
    }

    fn record_step(&mut self, step_index: u32, elapsed_millis: u64) {
        self.consumed_total_millis = self.consumed_total_millis.saturating_add(elapsed_millis);
        if step_index <= 4 {
            self.consumed_early_millis = self.consumed_early_millis.saturating_add(elapsed_millis);
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

    fn remaining_spill_steps(&self, step_index: u32) -> u64 {
        let start = step_index.max(DEMO_LATE_SPILL_START_STEP);
        if start > self.until_step {
            return 1;
        }
        u64::from(self.until_step - start + 1)
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
    scout_sample_recorded: bool,
    breadth_harvest_entered: bool,
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
        recorder.push(
            NarrativeEventKind::BudgetPulse,
            Some(StepPhase::Scout),
            "scout baselining started".to_owned(),
            Some(demo_budget_plan_detail(budget)),
            Some(zero_progress),
        );
        Self {
            budget,
            recorder,
            scout_sample_recorded: false,
            breadth_harvest_entered: false,
        }
    }

    fn maybe_enter_breadth_harvest(
        &mut self,
        elapsed_millis: u64,
        progress: NarrativeProgressSnapshot,
        scout_detail: String,
        harvest_detail: String,
    ) {
        if self.scout_sample_recorded || elapsed_millis < self.budget.scout_budget_millis {
            return;
        }

        self.record_scout_sample(progress.clone(), scout_detail);
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
        self.recorder.push(
            NarrativeEventKind::BudgetPulse,
            Some(StepPhase::BreadthHarvest),
            "breadth harvest is widening under the remaining discovery slice".to_owned(),
            Some(harvest_detail),
            Some(progress),
        );
        self.breadth_harvest_entered = true;
    }

    fn close_discovery(
        &mut self,
        progress: NarrativeProgressSnapshot,
        scout_detail: String,
        harvest_detail: String,
    ) {
        self.record_scout_sample(progress.clone(), scout_detail);
        if self.breadth_harvest_entered {
            self.recorder.push(
                NarrativeEventKind::BudgetPulse,
                Some(StepPhase::BreadthHarvest),
                "breadth harvest closed the widening slice".to_owned(),
                Some(harvest_detail),
                Some(progress),
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
        self.recorder.push(
            NarrativeEventKind::BudgetPulse,
            Some(StepPhase::Materialize),
            "materializing retained prefixes under exact screening".to_owned(),
            Some(detail),
            Some(progress),
        );
    }

    fn enter_proof_close(&mut self, progress: NarrativeProgressSnapshot, detail: String) {
        self.enter_proof_close_with_message(
            progress,
            "entered proof_close with the reserved certification slice",
            detail,
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
        self.recorder.push(
            NarrativeEventKind::BudgetPulse,
            Some(StepPhase::ProofClose),
            "proof-close is certifying the incumbent under exact comparison".to_owned(),
            Some(detail),
            Some(progress),
        );
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
        self.recorder.push(
            NarrativeEventKind::BudgetPulse,
            Some(StepPhase::Seal),
            format!(
                "seal fixed overshoot {} for candidate {}",
                overshoot, accepted_hash
            ),
            Some(detail),
            Some(progress),
        );
    }

    fn finish(self) -> Vec<NarrativeEvent> {
        self.recorder.finish()
    }

    fn record_scout_sample(&mut self, progress: NarrativeProgressSnapshot, detail: String) {
        if self.scout_sample_recorded {
            return;
        }
        self.recorder.push(
            NarrativeEventKind::BudgetPulse,
            Some(StepPhase::Scout),
            "scout sample captured throughput on this machine".to_owned(),
            Some(detail),
            Some(progress),
        );
        self.scout_sample_recorded = true;
    }
}

fn admissibility_mode_name(mode: AdmissibilityMode) -> &'static str {
    match mode {
        AdmissibilityMode::Guarded => "guarded",
        AdmissibilityMode::RelaxedShadow => "relaxed_shadow",
        AdmissibilityMode::RealisticShadow => "realistic_shadow",
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
    format!(
        "materialize_full_evals={} proof_close_full_evals={} proof_close_overrun={} soft_cap={} cap_triggered={}",
        phase.materialize_full_evals,
        phase.proof_close_full_evals,
        phase.proof_close_overrun_full_evals,
        soft_cap,
        phase.materialize_soft_cap_triggered
    )
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
        "proof_close_reserve={} remaining_retained_prefix_groups={} {} {}",
        format_duration_millis(budget.proof_close_reserve_millis),
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

fn discovery_progress_snapshot(
    step_start: Instant,
    discovery: &RealisticShadowDiscovery,
) -> NarrativeProgressSnapshot {
    narrative_progress_snapshot(
        elapsed_millis(step_start.elapsed()),
        generated_surface_from_counts(discovery.prefixes_created, discovery.enumerated_candidates),
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
        per_second(
            generated_surface_from_counts(
                discovery.prefixes_created,
                discovery.enumerated_candidates
            ),
            elapsed_millis
        ),
        per_second(discovery.well_formed_candidates as u64, elapsed_millis),
        per_second(discovery.partial_prefix_bound_checks as u64, elapsed_millis),
        per_second(discovery.candidates.len() as u64, elapsed_millis),
        generated_surface_from_counts(discovery.prefixes_created, discovery.enumerated_candidates),
        discovery.well_formed_candidates,
        discovery.partial_prefix_bound_checks,
        discovery.candidates.len()
    )
}

fn discovery_harvest_detail(
    budget: DemoStepBudget,
    discovery: &RealisticShadowDiscovery,
) -> String {
    let generated_surface =
        generated_surface_from_counts(discovery.prefixes_created, discovery.enumerated_candidates);
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
    search_bootstrap_from_prefix_internal(
        &[],
        until_step,
        window_depth,
        search_profile,
        retention_runtime,
        None,
    )
}

pub fn search_bootstrap_prefix_for_config_with_runtime(
    until_step: u32,
    window_depth: u16,
    config: &RuntimeConfig,
    retention_runtime: FrontierRuntimeLimits,
) -> Result<Vec<AtomicSearchStep>> {
    search_bootstrap_from_prefix_for_config_with_runtime_and_seed(
        &[],
        until_step,
        window_depth,
        config,
        retention_runtime,
        DemoBudgetSeed::default(),
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
    search_bootstrap_from_prefix_internal(
        accepted_prefix,
        until_step,
        window_depth,
        search_profile,
        retention_runtime,
        None,
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
    search_bootstrap_from_prefix_internal(
        accepted_prefix,
        until_step,
        window_depth,
        config.mode.search_profile,
        retention_runtime,
        DemoBudgetController::maybe_new(config, until_step, demo_budget_seed)?,
    )
}

fn search_bootstrap_from_prefix_internal(
    accepted_prefix: &[Telescope],
    until_step: u32,
    window_depth: u16,
    search_profile: SearchProfile,
    retention_runtime: FrontierRuntimeLimits,
    mut demo_budget_controller: Option<DemoBudgetController>,
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
        )?;
        if let Some(controller) = demo_budget_controller.as_mut() {
            controller.record_step(step_index, outcome.search_timing.step_wall_clock_millis);
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
) -> Result<AtomicSearchStep> {
    let step_start = Instant::now();
    let structural_debt = summarize_structural_debt(library, window_depth);
    let admissibility =
        strict_admissibility_for_mode(step_index, window_depth, library, admissibility_mode);
    let retention_policy = structural_debt.retention_policy();
    let objective_bar = compute_bar(window_depth as usize, step_index, history).bar;
    let mut demo_narrative = demo_step_budget
        .map(|budget| DemoNarrativeRuntime::new(step_index, objective_bar, admissibility, budget));
    let mut candidates = Vec::new();
    let mut prefix_cache = PrefixCache::default();
    let mut prefixes_created = 0usize;
    let mut prefix_states_explored = 0usize;
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
        ..DemoPhaseStats::default()
    };
    let discovery_start = Instant::now();
    let nu_history = history
        .iter()
        .map(|record| (record.step_index, record.nu))
        .collect::<Vec<_>>();

    if admissibility_mode == AdmissibilityMode::RealisticShadow {
        let discovery = discover_realistic_shadow_candidates(
            step_index,
            library,
            history,
            admissibility,
            retention_policy,
            objective_bar,
            &nu_history,
            demo_step_budget.and_then(|budget| budget.discovery_deadline(step_start)),
            step_start,
            &mut demo_narrative,
        )?;
        if let (Some(observer), Some(budget)) = (demo_narrative.as_mut(), demo_step_budget) {
            observer.close_discovery(
                discovery_progress_snapshot(step_start, &discovery),
                discovery_scout_detail(&discovery, elapsed_millis(step_start.elapsed())),
                discovery_harvest_detail(budget, &discovery),
            );
        }
        prefix_cache = discovery.prefix_cache;
        candidates = discovery.candidates;
        prefixes_created = discovery.prefixes_created;
        prefix_states_explored = discovery.prefix_states_explored;
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
        incremental_connectivity_prunes = legality_stats.connectivity_prunes;
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
        if let (Some(observer), Some(budget)) = (demo_narrative.as_mut(), demo_step_budget) {
            observer.close_discovery(
                narrative_progress_snapshot(
                    elapsed_millis(step_start.elapsed()),
                    generated_surface_from_counts(prefixes_created, enumerated_candidates),
                    exact_screened_surface_from_counts(0, candidates.len()),
                    candidates.len() as u64,
                ),
                format!(
                    "generated_per_sec={} admissibility_per_sec={} exact_bound_per_sec=0 full_eval_per_sec={} generated={} admissibility_checks={} exact_bound_checks=0 full_evals={}",
                    per_second(
                        generated_surface_from_counts(prefixes_created, enumerated_candidates),
                        elapsed_millis(step_start.elapsed())
                    ),
                    per_second(well_formed_candidates as u64, elapsed_millis(step_start.elapsed())),
                    per_second(candidates.len() as u64, elapsed_millis(step_start.elapsed())),
                    generated_surface_from_counts(prefixes_created, enumerated_candidates),
                    well_formed_candidates,
                    candidates.len()
                ),
                demo_surface_status_detail(
                    budget,
                    generated_surface_from_counts(prefixes_created, enumerated_candidates),
                    exact_screened_surface_from_counts(0, candidates.len()),
                    candidates.len() as u64,
                ),
            );
        }
    }
    let candidate_discovery_wall_clock_millis = elapsed_millis(discovery_start.elapsed());

    let prefix_frontier_planning_start = Instant::now();
    let prefix_frontier = if admissibility_mode == AdmissibilityMode::RealisticShadow {
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
    if let (Some(observer), Some(budget)) = (demo_narrative.as_mut(), demo_step_budget) {
        observer.enter_materialize(
            narrative_progress_snapshot(
                elapsed_millis(step_start.elapsed()),
                generated_surface_from_counts(prefixes_created, enumerated_candidates),
                exact_screened_surface_from_counts(prefix_states_exact_pruned, candidates.len()),
                candidates.len() as u64,
            ),
            demo_materialize_status_detail(
                budget,
                generated_surface_from_counts(prefixes_created, enumerated_candidates),
                exact_screened_surface_from_counts(prefix_states_exact_pruned, candidates.len()),
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
    if admissibility_mode == AdmissibilityMode::RealisticShadow {
        let mut incumbent_terminal_rank = None;
        for (group_index, signature) in prefix_frontier
            .retained_prefix_signatures
            .iter()
            .enumerate()
        {
            let Some(group) = prefix_cache.get(signature) else {
                continue;
            };
            if let (Some(group_best_rank), Some(incumbent_rank)) =
                (&group.best_accept_rank, &incumbent_terminal_rank)
            {
                if !better_rank(group_best_rank, incumbent_rank) {
                    incremental_terminal_rank_prunes += group.candidates.len();
                    continue;
                }
            }

            let mut group_candidates = group.candidates.clone();
            group_candidates.sort_by_key(|candidate| {
                serde_json::to_string(&candidate.telescope).expect("serialize")
            });
            for group_candidate in group_candidates {
                if !demo_proof_close_entered {
                    if let Some(full_eval_soft_cap) = demo_phase.full_eval_soft_cap {
                        if u64::try_from(candidates.len()).expect("candidate count exceeded u64")
                            >= full_eval_soft_cap
                        {
                            demo_phase.materialize_soft_cap_triggered = true;
                            demo_phase.materialize_full_evals = candidates.len();
                            if let (Some(observer), Some(budget)) =
                                (demo_narrative.as_mut(), demo_step_budget)
                            {
                                observer.enter_proof_close_with_message(
                                    narrative_progress_snapshot(
                                        elapsed_millis(step_start.elapsed()),
                                        generated_surface_from_counts(
                                            prefixes_created,
                                            enumerated_candidates,
                                        ),
                                        exact_screened_surface_from_counts(
                                            prefix_states_exact_pruned,
                                            candidates.len(),
                                        ),
                                        candidates.len() as u64,
                                    ),
                                    format!(
                                        "entered proof_close after materialize hit the full-eval soft cap with {} retained prefix groups still needing exact certification",
                                        retained_prefix_groups.saturating_sub(group_index)
                                    ),
                                    demo_proof_close_status_detail(
                                        budget,
                                        generated_surface_from_counts(
                                            prefixes_created,
                                            enumerated_candidates,
                                        ),
                                        exact_screened_surface_from_counts(
                                            prefix_states_exact_pruned,
                                            candidates.len(),
                                        ),
                                        candidates.len() as u64,
                                        retained_prefix_groups.saturating_sub(group_index),
                                        &demo_phase,
                                    ),
                                );
                            }
                            demo_proof_close_entered = true;
                        }
                    }
                }
                if let (Some(candidate_rank), Some(incumbent_rank)) =
                    (&group_candidate.accept_rank, &incumbent_terminal_rank)
                {
                    if !better_rank(candidate_rank, incumbent_rank) {
                        incremental_terminal_rank_prunes += 1;
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
                candidates.push(candidate);
                if demo_step_budget.is_some() {
                    if demo_proof_close_entered {
                        demo_phase.proof_close_full_evals += 1;
                        if demo_phase.materialize_soft_cap_triggered {
                            demo_phase.proof_close_overrun_full_evals += 1;
                        }
                    } else {
                        demo_phase.materialize_full_evals = candidates.len();
                    }
                }
            }
        }
    }

    if candidates.is_empty() {
        bail!("no atomic candidates were generated for step {step_index}");
    }
    if demo_step_budget.is_some() && !demo_proof_close_entered {
        demo_phase.materialize_full_evals = candidates.len();
        if let (Some(observer), Some(budget)) = (demo_narrative.as_mut(), demo_step_budget) {
            observer.enter_proof_close(
                narrative_progress_snapshot(
                    elapsed_millis(step_start.elapsed()),
                    generated_surface_from_counts(prefixes_created, enumerated_candidates),
                    exact_screened_surface_from_counts(
                        prefix_states_exact_pruned,
                        candidates.len(),
                    ),
                    candidates.len() as u64,
                ),
                demo_proof_close_status_detail(
                    budget,
                    generated_surface_from_counts(prefixes_created, enumerated_candidates),
                    exact_screened_surface_from_counts(
                        prefix_states_exact_pruned,
                        candidates.len(),
                    ),
                    candidates.len() as u64,
                    0,
                    &demo_phase,
                ),
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
                generated_surface_from_counts(prefixes_created, enumerated_candidates),
                exact_screened_surface_from_counts(
                    prefix_states_exact_pruned,
                    full_telescopes_evaluated,
                ),
                full_telescopes_evaluated as u64,
            ),
            format!(
                "bar_clearers={} near_misses={} {}",
                bar_clearers,
                acceptance.near_misses.len(),
                demo_surface_status_detail(
                    budget,
                    generated_surface_from_counts(prefixes_created, enumerated_candidates),
                    exact_screened_surface_from_counts(
                        prefix_states_exact_pruned,
                        full_telescopes_evaluated,
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
        demo_phase,
        search_timing,
        prefix_frontier.frontier.hot.len(),
        prefix_frontier.frontier.cold.len(),
        retention_policy,
        if admissibility_mode == AdmissibilityMode::RealisticShadow {
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
    demo_phase: DemoPhaseStats,
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
        demo_phase,
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

fn admissibility_mode_for_profile(search_profile: SearchProfile) -> AdmissibilityMode {
    match search_profile {
        SearchProfile::StrictCanonGuarded | SearchProfile::Unknown => AdmissibilityMode::Guarded,
        SearchProfile::RelaxedShadow => AdmissibilityMode::RelaxedShadow,
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
    discovery_deadline: Option<Instant>,
    step_start: Instant,
    demo_narrative: &mut Option<DemoNarrativeRuntime>,
) -> Result<RealisticShadowDiscovery> {
    let mut discovery = RealisticShadowDiscovery::default();
    let enumeration_context = EnumerationContext::from_admissibility(library, admissibility);

    for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
        if demo_discovery_budget_exhausted(
            discovery_deadline,
            &discovery,
            step_start,
            demo_narrative.as_mut(),
        ) {
            break;
        }
        if clause_kappa <= 1 {
            let telescopes = enumerate_telescopes(library, enumeration_context, clause_kappa);
            discovery.enumerated_candidates += telescopes.len();
            for telescope in telescopes {
                if demo_discovery_budget_exhausted(
                    discovery_deadline,
                    &discovery,
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

        let mut frontier = Vec::new();
        for clause in clause_catalog.clauses_at(0) {
            if demo_discovery_budget_exhausted(
                discovery_deadline,
                &discovery,
                step_start,
                demo_narrative.as_mut(),
            ) {
                break;
            }
            let prefix_telescope = Telescope::new(vec![clause.clone()]);
            let signature = PrefixSignature::new(step_index, library, &prefix_telescope);
            if !discovery.prefix_legality_cache.insert_root(
                signature.clone(),
                clause_kappa,
                library,
                &prefix_telescope,
                admissibility,
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
            if let Some(decision) = cached_terminal_prefix_queue_entry_bound_decision(
                objective_bar,
                &work_item,
                &mut discovery.prefix_legality_cache,
            ) {
                match decision {
                    ExactPartialPrefixBoundDecision::CanClearBar => frontier.push(work_item),
                    ExactPartialPrefixBoundDecision::CannotClearBar => {
                        discovery.partial_prefix_bound_prunes += 1;
                    }
                    ExactPartialPrefixBoundDecision::Unknown => frontier.push(work_item),
                }
                continue;
            }
            let mut exact_bound_budget = EXACT_PARTIAL_PREFIX_BOUND_BUDGET;
            match exact_partial_prefix_bound_decision(
                step_index,
                library,
                admissibility,
                objective_bar,
                nu_history,
                &clause_catalog,
                &work_item,
                &mut discovery.prefix_legality_cache,
                &mut exact_bound_budget,
            ) {
                ExactPartialPrefixBoundDecision::CanClearBar => {
                    discovery.partial_prefix_bound_checks += 1;
                    frontier.push(work_item);
                }
                ExactPartialPrefixBoundDecision::CannotClearBar => {
                    discovery.partial_prefix_bound_checks += 1;
                    discovery.partial_prefix_bound_prunes += 1;
                }
                ExactPartialPrefixBoundDecision::Unknown => {
                    frontier.push(work_item);
                }
            }
        }

        while !frontier.is_empty() {
            if demo_discovery_budget_exhausted(
                discovery_deadline,
                &discovery,
                step_start,
                demo_narrative.as_mut(),
            ) {
                break;
            }
            let Some(work_item) = pop_best_prefix(&mut frontier) else {
                break;
            };
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
                let mut group = materialize_terminal_prefix_group(
                    step_index,
                    library,
                    admissibility,
                    objective_bar,
                    nu_history,
                    &work_item.signature,
                    &work_item.prefix_telescope,
                    &work_item.next_clauses,
                    &mut discovery,
                )?;
                let Some(retained_bound) = group.bound else {
                    continue;
                };
                if !retained_bound.can_clear_bar(objective_bar) {
                    discovery.terminal_prefix_bar_prunes += 1;
                    continue;
                }
                if let Some(pruned_candidates) = cached_terminal_prefix_rank_prune_count(
                    &work_item.signature,
                    discovery.terminal_rank_incumbent.as_ref(),
                    &mut discovery.prefix_legality_cache,
                ) {
                    discovery.terminal_rank_prunes += pruned_candidates;
                    continue;
                }
                if let (Some(group_best_rank), Some(incumbent_rank)) = (
                    best_prefix_group_accept_rank(&group.candidates),
                    discovery.terminal_rank_incumbent.as_ref(),
                ) {
                    if !better_rank(&group_best_rank, incumbent_rank) {
                        discovery.terminal_rank_prunes += group.candidates.len();
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
                let terminal_prefixes = prepare_exact_two_step_terminal_surface(
                    step_index,
                    library,
                    admissibility,
                    &clause_catalog,
                    &work_item,
                    &mut discovery,
                );
                if can_process_exact_two_step_terminal_surface_now(&frontier, &terminal_prefixes) {
                    process_prepared_exact_two_step_terminal_surface(
                        step_index,
                        library,
                        history,
                        admissibility,
                        objective_bar,
                        nu_history,
                        retention_policy,
                        terminal_prefixes,
                        &mut discovery,
                    )?;
                    continue;
                }

                for terminal_prefix in terminal_prefixes {
                    let mut exact_bound_budget = EXACT_PARTIAL_PREFIX_BOUND_BUDGET;
                    match exact_partial_prefix_bound_decision(
                        step_index,
                        library,
                        admissibility,
                        objective_bar,
                        nu_history,
                        &clause_catalog,
                        &terminal_prefix,
                        &mut discovery.prefix_legality_cache,
                        &mut exact_bound_budget,
                    ) {
                        ExactPartialPrefixBoundDecision::CanClearBar => {
                            discovery.partial_prefix_bound_checks += 1;
                            frontier.push(terminal_prefix);
                        }
                        ExactPartialPrefixBoundDecision::CannotClearBar => {
                            discovery.partial_prefix_bound_checks += 1;
                            discovery.partial_prefix_bound_prunes += 1;
                        }
                        ExactPartialPrefixBoundDecision::Unknown => {
                            frontier.push(terminal_prefix);
                        }
                    }
                }
                continue;
            }

            for clause in &work_item.next_clauses {
                if demo_discovery_budget_exhausted(
                    discovery_deadline,
                    &discovery,
                    step_start,
                    demo_narrative.as_mut(),
                ) {
                    break;
                }
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
                if let Some(decision) = cached_terminal_prefix_queue_entry_bound_decision(
                    objective_bar,
                    &child_work_item,
                    &mut discovery.prefix_legality_cache,
                ) {
                    match decision {
                        ExactPartialPrefixBoundDecision::CanClearBar => {
                            frontier.push(child_work_item)
                        }
                        ExactPartialPrefixBoundDecision::CannotClearBar => {
                            discovery.partial_prefix_bound_prunes += 1;
                        }
                        ExactPartialPrefixBoundDecision::Unknown => frontier.push(child_work_item),
                    }
                    continue;
                }
                let mut exact_bound_budget = EXACT_PARTIAL_PREFIX_BOUND_BUDGET;
                match exact_partial_prefix_bound_decision(
                    step_index,
                    library,
                    admissibility,
                    objective_bar,
                    nu_history,
                    &clause_catalog,
                    &child_work_item,
                    &mut discovery.prefix_legality_cache,
                    &mut exact_bound_budget,
                ) {
                    ExactPartialPrefixBoundDecision::CanClearBar => {
                        discovery.partial_prefix_bound_checks += 1;
                        frontier.push(child_work_item);
                    }
                    ExactPartialPrefixBoundDecision::CannotClearBar => {
                        discovery.partial_prefix_bound_checks += 1;
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

fn demo_discovery_budget_exhausted(
    discovery_deadline: Option<Instant>,
    discovery: &RealisticShadowDiscovery,
    step_start: Instant,
    demo_narrative: Option<&mut DemoNarrativeRuntime>,
) -> bool {
    if let Some(observer) = demo_narrative {
        let elapsed = elapsed_millis(step_start.elapsed());
        let progress = discovery_progress_snapshot(step_start, discovery);
        observer.maybe_enter_breadth_harvest(
            elapsed,
            progress,
            discovery_scout_detail(discovery, elapsed),
            discovery_harvest_detail(observer.budget, discovery),
        );
    }

    let Some(discovery_deadline) = discovery_deadline else {
        return false;
    };
    if !discovery.can_stop_for_budget() {
        return false;
    }

    Instant::now() >= discovery_deadline
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
    for clause in &work_item.next_clauses {
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

fn process_prepared_exact_two_step_terminal_surface(
    step_index: u32,
    library: &Library,
    history: &[DiscoveryRecord],
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    retention_policy: RetentionPolicy,
    terminal_prefixes: Vec<OnlinePrefixWorkItem>,
    discovery: &mut RealisticShadowDiscovery,
) -> Result<()> {
    for terminal_prefix in terminal_prefixes {
        debug_assert_eq!(terminal_prefix.remaining_clause_slots, 1);

        let mut group = materialize_terminal_prefix_group(
            step_index,
            library,
            admissibility,
            objective_bar,
            nu_history,
            &terminal_prefix.signature,
            &terminal_prefix.prefix_telescope,
            &terminal_prefix.next_clauses,
            discovery,
        )?;
        let Some(retained_bound) = group.bound else {
            continue;
        };
        if !retained_bound.can_clear_bar(objective_bar) {
            discovery.terminal_prefix_bar_prunes += 1;
            continue;
        }
        if let Some(pruned_candidates) = cached_terminal_prefix_rank_prune_count(
            &terminal_prefix.signature,
            discovery.terminal_rank_incumbent.as_ref(),
            &mut discovery.prefix_legality_cache,
        ) {
            discovery.terminal_rank_prunes += pruned_candidates;
            continue;
        }
        if let (Some(group_best_rank), Some(incumbent_rank)) = (
            best_prefix_group_accept_rank(&group.candidates),
            discovery.terminal_rank_incumbent.as_ref(),
        ) {
            if !better_rank(&group_best_rank, incumbent_rank) {
                discovery.terminal_rank_prunes += group.candidates.len();
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
    let prefix_len = prefix_telescope.clauses.len();
    let remaining_clause_slots = usize::from(clause_kappa).saturating_sub(prefix_len);
    let next_clauses = if remaining_clause_slots == 0 {
        Vec::new()
    } else {
        prefix_legality_cache
            .filter_active_window_clauses(
                &signature,
                prefix_len,
                library,
                admissibility,
                clause_catalog.clauses_at(prefix_len),
            )
            .map(|clauses| clauses.into_iter().cloned().collect())
            .unwrap_or_else(|| clause_catalog.clauses_at(prefix_len).to_vec())
    };

    OnlinePrefixWorkItem {
        clause_kappa,
        remaining_clause_slots,
        remaining_family_options: prefix_legality_cache
            .family_option_count(&signature)
            .unwrap_or(u8::MAX),
        bit_cost: prefix_telescope.bit_cost(),
        clause_count: prefix_telescope.kappa(),
        next_clauses,
        order_key: serde_json::to_string(&prefix_telescope)
            .expect("prefix telescope should serialize"),
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

        let [clause] = work_item.next_clauses.as_slice() else {
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
            &work_item.next_clauses,
            prefix_legality_cache,
            budget,
        );
        if let Some(cacheable) = decision.cacheable_partial_decision() {
            prefix_legality_cache
                .store_partial_prefix_bound_decision(work_item.signature.clone(), cacheable);
        }
        return decision;
    }

    for clause in &work_item.next_clauses {
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
) -> Option<ExactPartialPrefixBoundDecision> {
    if work_item.remaining_clause_slots != 1 {
        return None;
    }

    let summary = prefix_legality_cache.terminal_prefix_completion_summary(&work_item.signature)?;
    let decision = exact_terminal_prefix_completion_summary_decision(objective_bar, &summary);
    if let Some(cacheable) = decision.cacheable_partial_decision() {
        prefix_legality_cache
            .store_partial_prefix_bound_decision(work_item.signature.clone(), cacheable);
    }
    Some(decision)
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
    prefix_legality_cache: &mut PrefixLegalityCache,
    budget: &mut usize,
) -> ExactPartialPrefixBoundDecision {
    if let Some(summary) =
        prefix_legality_cache.terminal_prefix_completion_summary(prefix_signature)
    {
        return exact_terminal_prefix_completion_summary_decision(objective_bar, &summary);
    }

    let terminal_clauses = terminal_prefix_clause_candidates(
        step_index,
        library,
        admissibility,
        prefix_signature,
        filtered_last_clause_options,
        prefix_legality_cache,
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
            terminal_clauses,
            prefix_legality_cache,
        );
        prefix_legality_cache
            .store_terminal_prefix_completion_summary(prefix_signature.clone(), summary.clone());
        return exact_terminal_prefix_completion_summary_decision(objective_bar, &summary);
    }

    for (clause, cached_admissibility_decision) in terminal_clauses {
        if !spend_exact_partial_prefix_budget(budget, 1) {
            return ExactPartialPrefixBoundDecision::Unknown;
        }

        let Some(connectivity_decision) =
            prefix_legality_cache.terminal_connectivity(prefix_signature, library, clause)
        else {
            continue;
        };
        if matches!(
            connectivity_decision,
            TerminalConnectivityDecision::PruneDisconnected
        ) {
            continue;
        }

        let mut telescope = None;
        if matches!(
            connectivity_decision,
            TerminalConnectivityDecision::NeedsFallback
        ) {
            let mut fallback_telescope = prefix_telescope.clone();
            fallback_telescope.clauses.push(clause.clone());
            if !passes_connectivity(library, &fallback_telescope) {
                continue;
            }
            telescope = Some(fallback_telescope);
        }

        let admissibility_decision = if let Some(decision) = cached_admissibility_decision {
            decision
        } else {
            let fallback_telescope = telescope.get_or_insert_with(|| {
                let mut telescope = prefix_telescope.clone();
                telescope.clauses.push(clause.clone());
                telescope
            });
            assess_strict_admissibility(step_index, library, fallback_telescope, admissibility)
        };
        if !admissibility_decision.is_admitted() {
            continue;
        }

        let telescope = telescope.unwrap_or_else(|| {
            let mut telescope = prefix_telescope.clone();
            telescope.clauses.push(clause.clone());
            telescope
        });
        let exact_nu = structural_nu(&telescope, library, nu_history).total;
        let clause_kappa_used = u32::try_from(telescope.kappa()).expect("kappa exceeded u32");
        let rho = Rational::new(i64::from(exact_nu), i64::from(clause_kappa_used));
        if rho >= objective_bar {
            return ExactPartialPrefixBoundDecision::CanClearBar;
        }
    }

    ExactPartialPrefixBoundDecision::CannotClearBar
}

fn exact_terminal_prefix_completion_summary_decision(
    objective_bar: Rational,
    summary: &TerminalPrefixCompletionSummary,
) -> ExactPartialPrefixBoundDecision {
    let Some(bound) = summary.bound else {
        return ExactPartialPrefixBoundDecision::CannotClearBar;
    };
    if bound.can_clear_bar(objective_bar) {
        ExactPartialPrefixBoundDecision::CanClearBar
    } else {
        ExactPartialPrefixBoundDecision::CannotClearBar
    }
}

fn terminal_prefix_clause_candidates<'a>(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    prefix_signature: &PrefixSignature,
    filtered_last_clause_options: &'a [pen_core::clause::ClauseRec],
    prefix_legality_cache: &mut PrefixLegalityCache,
) -> Vec<(
    &'a pen_core::clause::ClauseRec,
    Option<pen_type::admissibility::AdmissibilityDecision>,
)> {
    prefix_legality_cache
        .filter_terminal_clauses(
            step_index,
            prefix_signature,
            library,
            admissibility,
            &filtered_last_clause_options.iter().collect::<Vec<_>>(),
        )
        .map(|clauses| {
            clauses
                .into_iter()
                .map(|clause| (clause.clause, Some(clause.admissibility_decision)))
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| {
            filtered_last_clause_options
                .iter()
                .map(|clause| (clause, None))
                .collect::<Vec<_>>()
        })
}

fn compute_terminal_prefix_completion_summary_from_candidates(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    prefix_signature: &PrefixSignature,
    prefix_telescope: &Telescope,
    terminal_clauses: Vec<(
        &pen_core::clause::ClauseRec,
        Option<pen_type::admissibility::AdmissibilityDecision>,
    )>,
    prefix_legality_cache: &mut PrefixLegalityCache,
) -> TerminalPrefixCompletionSummary {
    let mut summary = TerminalPrefixCompletionSummary::default();

    for (clause, cached_admissibility_decision) in terminal_clauses {
        let Some(connectivity_decision) =
            prefix_legality_cache.terminal_connectivity(prefix_signature, library, clause)
        else {
            continue;
        };
        if matches!(
            connectivity_decision,
            TerminalConnectivityDecision::PruneDisconnected
        ) {
            summary
                .evaluations
                .push(TerminalPrefixClauseEvaluation::Disconnected);
            continue;
        }

        let mut telescope = None;
        if matches!(
            connectivity_decision,
            TerminalConnectivityDecision::NeedsFallback
        ) {
            let mut fallback_telescope = prefix_telescope.clone();
            fallback_telescope.clauses.push(clause.clone());
            if !passes_connectivity(library, &fallback_telescope) {
                summary
                    .evaluations
                    .push(TerminalPrefixClauseEvaluation::Disconnected);
                continue;
            }
            telescope = Some(fallback_telescope);
        }

        let admissibility_decision = if let Some(decision) = cached_admissibility_decision {
            decision
        } else {
            let fallback_telescope = telescope.get_or_insert_with(|| {
                let mut telescope = prefix_telescope.clone();
                telescope.clauses.push(clause.clone());
                telescope
            });
            assess_strict_admissibility(step_index, library, fallback_telescope, admissibility)
        };
        if !admissibility_decision.is_admitted() {
            summary
                .evaluations
                .push(TerminalPrefixClauseEvaluation::AdmissibilityRejected {
                    decision: admissibility_decision,
                });
            continue;
        }

        let telescope = telescope.unwrap_or_else(|| {
            let mut telescope = prefix_telescope.clone();
            telescope.clauses.push(clause.clone());
            telescope
        });
        let exact_nu = u16::try_from(structural_nu(&telescope, library, nu_history).total)
            .expect("nu exceeded u16");
        let bit_kappa_used =
            u16::try_from(telescope_bit_cost(&telescope)).expect("bit cost exceeded u16");
        let clause_kappa_used = u16::try_from(telescope.kappa()).expect("kappa exceeded u16");
        let completion_bound = PrefixBound::singleton(exact_nu, clause_kappa_used, bit_kappa_used);
        if let Some(bound) = summary.bound.as_mut() {
            bound.absorb_bound(completion_bound);
        } else {
            summary.bound = Some(completion_bound);
        }
        summary.admitted_candidate_count += 1;
        if let Some(rank) = acceptance_rank_for_telescope(
            objective_bar,
            &telescope,
            exact_nu,
            bit_kappa_used,
            clause_kappa_used,
        ) {
            match &summary.best_accept_rank {
                Some(current) if !better_rank(&rank, current) => {}
                _ => summary.best_accept_rank = Some(rank),
            }
        }
        summary
            .evaluations
            .push(TerminalPrefixClauseEvaluation::Admitted {
                decision: admissibility_decision,
                completion: TerminalPrefixCompletion {
                    telescope,
                    exact_nu,
                    bit_kappa_used,
                    clause_kappa_used,
                },
            });
    }

    summary
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
    discovery: &mut RealisticShadowDiscovery,
) -> Result<MaterializedTerminalPrefixGroup> {
    let summary = if let Some(summary) = discovery
        .prefix_legality_cache
        .terminal_prefix_completion_summary(prefix_signature)
    {
        summary
    } else {
        let terminal_clauses = terminal_prefix_clause_candidates(
            step_index,
            library,
            admissibility,
            prefix_signature,
            filtered_last_clause_options,
            &mut discovery.prefix_legality_cache,
        );
        let summary = compute_terminal_prefix_completion_summary_from_candidates(
            step_index,
            library,
            admissibility,
            objective_bar,
            nu_history,
            prefix_signature,
            prefix_telescope,
            terminal_clauses,
            &mut discovery.prefix_legality_cache,
        );
        discovery
            .prefix_legality_cache
            .store_terminal_prefix_completion_summary(prefix_signature.clone(), summary.clone());
        summary
    };
    let TerminalPrefixCompletionSummary {
        evaluations, bound, ..
    } = summary;
    let mut retained_telescopes = Vec::new();

    for evaluation in evaluations {
        match evaluation {
            TerminalPrefixClauseEvaluation::Disconnected => {}
            TerminalPrefixClauseEvaluation::AdmissibilityRejected { decision } => {
                discovery.enumerated_candidates += 1;
                discovery.well_formed_candidates += 1;
                discovery.admissibility_diagnostics.record(&decision);
                discovery.admissibility_rejections += 1;
            }
            TerminalPrefixClauseEvaluation::Admitted {
                decision,
                completion,
            } => {
                discovery.enumerated_candidates += 1;
                discovery.well_formed_candidates += 1;
                discovery.admissibility_diagnostics.record(&decision);
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

    Ok(MaterializedTerminalPrefixGroup {
        candidates: retained_telescopes,
        bound,
    })
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

fn cached_terminal_prefix_rank_prune_count(
    prefix_signature: &PrefixSignature,
    incumbent_rank: Option<&AcceptRank>,
    prefix_legality_cache: &mut PrefixLegalityCache,
) -> Option<usize> {
    let incumbent_rank = incumbent_rank?;
    let rank_summary = prefix_legality_cache.terminal_prefix_rank_summary(prefix_signature)?;
    let best_rank = rank_summary.best_accept_rank.as_ref()?;
    (!better_rank(best_rank, incumbent_rank)).then_some(rank_summary.admitted_candidate_count)
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
    candidates.sort_by_key(|candidate| {
        serde_json::to_string(&candidate.telescope)
            .expect("terminal group telescope should serialize")
    });

    for candidate in candidates {
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

    Ok(())
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
        item.next_clauses.len(),
        item.remaining_family_options,
        item.bit_cost,
        item.clause_count,
        item.order_key.as_str(),
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

#[cfg(test)]
mod tests {
    use super::{
        AtomicSearchStep, DemoBudgetController, DemoBudgetSeed, LIVE_BOOTSTRAP_MAX_STEP,
        OnlinePrefixWorkItem, create_online_prefix_work_item, exact_partial_prefix_bound_decision,
        pop_best_prefix, search_bootstrap_from_prefix,
        search_bootstrap_from_prefix_for_profile_with_runtime, search_bootstrap_prefix,
        search_bootstrap_prefix_for_config_with_runtime, supports_live_atomic_search,
    };
    use crate::config::{RuntimeConfig, SearchProfile};
    use crate::enumerate::{EnumerationContext, build_clause_catalog};
    use crate::expand::evaluate_candidate;
    use crate::narrative::{NarrativeEventKind, StepPhase};
    use crate::prefix_cache::PrefixSignature;
    use crate::prefix_memo::{PartialPrefixBoundDecision, PrefixLegalityCache};
    use pen_core::{
        clause::{ClauseRec, ClauseRole},
        expr::Expr,
        library::{Library, LibraryEntry},
        rational::Rational,
        telescope::{Telescope, TelescopeClass},
    };
    use pen_eval::{
        bar::{DiscoveryRecord, compute_bar},
        minimality::analyze_semantic_minimality,
    };
    use pen_type::{
        admissibility::{
            AdmissibilityMode, PackagePolicies, PackagePolicy, StrictAdmissibility,
            StructuralFamily, assess_strict_admissibility, passes_strict_admissibility,
            strict_admissibility, strict_admissibility_for_mode,
        },
        connectivity::{ConnectivityWitness, analyze_connectivity, passes_connectivity},
    };

    fn reference_prefix(until_step: u32) -> Vec<Telescope> {
        (1..=until_step).map(Telescope::reference).collect()
    }

    fn library_until(until_step: u32) -> Library {
        let mut library = Vec::new();
        for step in 1..=until_step {
            let telescope = Telescope::reference(step);
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }
        library
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
        OnlinePrefixWorkItem {
            clause_kappa: 4,
            prefix_telescope,
            signature,
            remaining_clause_slots,
            remaining_family_options,
            bit_cost,
            clause_count,
            next_clauses: vec![
                ClauseRec::new(ClauseRole::Introduction, Expr::Var(1));
                next_clause_count
            ],
            order_key: order_key.to_owned(),
        }
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
        assert_eq!(step_thirteen.spill_budget_millis, 50_000);
        assert_eq!(step_thirteen.total_budget_millis, 105_000);
        assert_eq!(step_thirteen.proof_close_reserve_millis, 31_500);
        assert_eq!(step_thirteen.discovery_budget_millis, 73_500);
        assert_eq!(step_thirteen.scout_budget_millis, 7_350);
        assert_eq!(step_thirteen.breadth_harvest_budget_millis, 66_150);
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
        assert_eq!(resumed_step_thirteen.total_budget_millis, 105_000);
        assert_eq!(resumed_step_thirteen.proof_close_reserve_millis, 31_500);
        assert_eq!(resumed_step_thirteen.discovery_budget_millis, 73_500);
        assert_eq!(resumed_step_thirteen.scout_budget_millis, 7_350);
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
        assert!(step.demo_phase.proof_close_full_evals > 0);
        assert!(step.demo_phase.proof_close_overrun_full_evals > 0);
        assert_eq!(
            step.demo_phase.materialize_full_evals + step.demo_phase.proof_close_full_evals,
            step.full_telescopes_evaluated
        );
        assert!(step.narrative_events.iter().any(|event| {
            event.kind == NarrativeEventKind::PhaseChange
                && event.phase == Some(StepPhase::ProofClose)
                && event.message.contains("soft cap")
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

        assert!(cache.insert_root(signature.clone(), 5, &library, &prefix, admissibility));

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
        assert!(!work_item.next_clauses.is_empty());
        assert!(work_item.next_clauses.len() < clause_catalog.clauses_at(4).len());
        assert_eq!(cache.stats().active_window_clause_filter_hits, 1);
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
                .order_key,
            "a"
        );
        assert_eq!(
            pop_best_prefix(&mut frontier)
                .expect("second work item should exist")
                .order_key,
            "b"
        );
        assert_eq!(
            pop_best_prefix(&mut frontier)
                .expect("third work item should exist")
                .order_key,
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

        assert!(cache.insert_root(signature.clone(), 8, &library, &prefix, admissibility));

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

        assert!(cache.insert_root(signature.clone(), 8, &library, &prefix, admissibility));

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

        assert!(cache.insert_root(signature.clone(), 8, &library, &prefix, admissibility));

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
