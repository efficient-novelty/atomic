use crate::accept::{
    AcceptanceOutcome, acceptance_rank, acceptance_rank_for_telescope, select_acceptance,
};
use crate::bounds::PrefixBound;
use crate::branch_bound::{AcceptRank, better_rank};
use crate::config::SearchProfile;
use crate::diversify::{FrontierPressure, FrontierRuntimeLimits, plan_pressure_cold_retention};
use crate::enumerate::{
    ClauseCatalog, EnumerationContext, build_clause_catalog, enumerate_telescopes,
};
use crate::expand::{ExpandedCandidate, evaluate_candidate, evaluate_checked_candidate};
use crate::frontier::FrontierWindow;
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
    pub incremental_terminal_rank_prunes: usize,
    pub incremental_partial_prefix_bound_hits: usize,
    pub incremental_partial_prefix_bound_checks: usize,
    pub incremental_partial_prefix_bound_prunes: usize,
    pub incremental_terminal_prefix_bar_prunes: usize,
    pub search_timing: SearchTiming,
    pub prefix_frontier_hot_states: usize,
    pub prefix_frontier_cold_states: usize,
    pub retention_policy: RetentionPolicy,
    pub frontier_pressure: FrontierPressure,
    pub frontier_retention: FrontierRetentionOutcome,
    pub frontier_window: FrontierWindow,
    pub frontier_dedupe_keys: BTreeSet<String>,
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
    search_bootstrap_from_prefix_for_profile_with_runtime(
        &[],
        until_step,
        window_depth,
        search_profile,
        retention_runtime,
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
        let outcome = search_next_step(
            step_index,
            window_depth,
            &library,
            &history,
            admissibility_mode,
            retention_runtime,
        )?;
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
) -> Result<AtomicSearchStep> {
    let step_start = Instant::now();
    let structural_debt = summarize_structural_debt(library, window_depth);
    let admissibility =
        strict_admissibility_for_mode(step_index, window_depth, library, admissibility_mode);
    let retention_policy = structural_debt.retention_policy();
    let objective_bar = compute_bar(window_depth as usize, step_index, history).bar;
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
    let mut incremental_terminal_rank_prunes = 0usize;
    let mut incremental_partial_prefix_bound_hits = 0usize;
    let mut incremental_partial_prefix_bound_checks = 0usize;
    let mut incremental_partial_prefix_bound_prunes = 0usize;
    let mut incremental_terminal_prefix_bar_prunes = 0usize;
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
        )?;
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

    if admissibility_mode == AdmissibilityMode::RealisticShadow {
        let mut incumbent_terminal_rank = None;
        for signature in &prefix_frontier.retained_prefix_signatures {
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
            }
        }
    }

    if candidates.is_empty() {
        bail!("no atomic candidates were generated for step {step_index}");
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
    let heuristic_drops = frontier_retention.dropped_candidates.len();
    let search_timing = SearchTiming {
        step_wall_clock_millis: elapsed_millis(step_start.elapsed()),
        candidate_discovery_wall_clock_millis,
        prefix_frontier_planning_wall_clock_millis,
        selection_wall_clock_millis: elapsed_millis(selection_start.elapsed()),
    };

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
        incremental_terminal_rank_prunes,
        incremental_partial_prefix_bound_hits,
        incremental_partial_prefix_bound_checks,
        incremental_partial_prefix_bound_prunes,
        incremental_terminal_prefix_bar_prunes,
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
    incremental_terminal_rank_prunes: usize,
    incremental_partial_prefix_bound_hits: usize,
    incremental_partial_prefix_bound_checks: usize,
    incremental_partial_prefix_bound_prunes: usize,
    incremental_terminal_prefix_bar_prunes: usize,
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
) -> Result<AtomicSearchStep> {
    let accepted = retained
        .iter()
        .find(|candidate| candidate.candidate_hash == acceptance.accepted.candidate_hash)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("accepted candidate vanished during selection"))?;

    Ok(AtomicSearchStep {
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
        incremental_terminal_rank_prunes,
        incremental_partial_prefix_bound_hits,
        incremental_partial_prefix_bound_checks,
        incremental_partial_prefix_bound_prunes,
        incremental_terminal_prefix_bar_prunes,
        search_timing,
        prefix_frontier_hot_states,
        prefix_frontier_cold_states,
        retention_policy,
        frontier_pressure,
        frontier_retention,
        frontier_window,
        frontier_dedupe_keys,
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
    })
}

fn admissibility_mode_for_profile(search_profile: SearchProfile) -> AdmissibilityMode {
    match search_profile {
        SearchProfile::StrictCanonGuarded | SearchProfile::Unknown => AdmissibilityMode::Guarded,
        SearchProfile::RelaxedShadow => AdmissibilityMode::RelaxedShadow,
        SearchProfile::RealisticFrontierShadow => AdmissibilityMode::RealisticShadow,
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
) -> Result<RealisticShadowDiscovery> {
    let mut discovery = RealisticShadowDiscovery::default();
    let enumeration_context = EnumerationContext::from_admissibility(library, admissibility);

    for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
        if clause_kappa <= 1 {
            let telescopes = enumerate_telescopes(library, enumeration_context, clause_kappa);
            discovery.enumerated_candidates += telescopes.len();
            for telescope in telescopes {
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

        while let Some(work_item) = pop_best_prefix(&mut frontier) {
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
                let child_work_item = create_online_prefix_work_item(
                    work_item.clause_kappa,
                    child_prefix,
                    child_signature,
                    library,
                    admissibility,
                    &clause_catalog,
                    &mut discovery.prefix_legality_cache,
                );
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
    let (work_item, collapsed_prefixes) = collapse_single_continuation_chain_inner(
        step_index,
        library,
        admissibility,
        clause_catalog,
        &mut discovery.prefix_legality_cache,
        work_item,
    )?;
    discovery.prefixes_created += collapsed_prefixes;
    Some(work_item)
}

fn collapse_single_continuation_chain_inner(
    step_index: u32,
    library: &Library,
    admissibility: StrictAdmissibility,
    clause_catalog: &ClauseCatalog,
    prefix_legality_cache: &mut PrefixLegalityCache,
    mut work_item: OnlinePrefixWorkItem,
) -> Option<(OnlinePrefixWorkItem, usize)> {
    let mut collapsed_prefixes = 0usize;

    loop {
        if work_item.remaining_clause_slots <= 1 {
            return Some((work_item, collapsed_prefixes));
        }

        let [clause] = work_item.next_clauses.as_slice() else {
            return Some((work_item, collapsed_prefixes));
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

        collapsed_prefixes += 1;
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

    if work_item.remaining_clause_slots == 1 {
        return exact_terminal_prefix_bound_decision(
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
        let Some((child_work_item, collapsed_prefixes)) = collapse_single_continuation_chain_inner(
            step_index,
            library,
            admissibility,
            clause_catalog,
            prefix_legality_cache,
            child_work_item,
        ) else {
            continue;
        };
        if !spend_exact_partial_prefix_budget(budget, collapsed_prefixes) {
            return ExactPartialPrefixBoundDecision::Unknown;
        }

        match exact_partial_prefix_bound_decision(
            step_index,
            library,
            admissibility,
            objective_bar,
            nu_history,
            clause_catalog,
            &child_work_item,
            prefix_legality_cache,
            budget,
        ) {
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
    let TerminalPrefixCompletionSummary { evaluations, bound } = summary;
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
        AtomicSearchStep, LIVE_BOOTSTRAP_MAX_STEP, OnlinePrefixWorkItem,
        create_online_prefix_work_item, exact_partial_prefix_bound_decision, pop_best_prefix,
        search_bootstrap_from_prefix, search_bootstrap_from_prefix_for_profile_with_runtime,
        search_bootstrap_prefix, supports_live_atomic_search,
    };
    use crate::config::SearchProfile;
    use crate::enumerate::{EnumerationContext, build_clause_catalog};
    use crate::expand::evaluate_candidate;
    use crate::prefix_cache::PrefixSignature;
    use crate::prefix_memo::PrefixLegalityCache;
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
        assert!(step.incremental_legality_cache_hits > 0);
        assert_eq!(step.incremental_connectivity_fallbacks, 0);
        assert!(step.incremental_clause_family_filter_hits > 0);
        assert!(step.incremental_active_window_clause_filter_hits > 0);
        assert!(step.incremental_terminal_clause_filter_hits > 0);
        assert!(step.incremental_trivial_derivability_hits > 0);
        assert!(step.incremental_terminal_admissibility_hits > 0);
        assert!(step.incremental_terminal_prefix_completion_hits > 0);
        assert!(step.incremental_terminal_rank_prunes > 0);
        assert!(step.incremental_partial_prefix_bound_checks > 0);
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
