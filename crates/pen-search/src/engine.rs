use crate::accept::{AcceptanceOutcome, select_acceptance};
use crate::bounds::PrefixBound;
use crate::config::SearchProfile;
use crate::diversify::{FrontierPressure, FrontierRuntimeLimits, plan_pressure_cold_retention};
use crate::enumerate::{EnumerationContext, build_clause_catalog, enumerate_telescopes};
use crate::expand::{ExpandedCandidate, evaluate_candidate, evaluate_checked_candidate};
use crate::frontier::FrontierWindow;
use crate::prefix_cache::{PrefixCache, PrefixCandidateGroup, PrefixSignature};
use crate::prefix_memo::{PrefixLegalityCache, TerminalConnectivityDecision};
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
    pub incremental_trivial_derivability_hits: usize,
    pub incremental_trivial_derivability_prunes: usize,
    pub incremental_terminal_admissibility_hits: usize,
    pub incremental_terminal_admissibility_rejections: usize,
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
    prefixes_created: usize,
    prefix_states_explored: usize,
    enumerated_candidates: usize,
    well_formed_candidates: usize,
    malformed_rejections: usize,
    malformed_rejection_reasons: BTreeMap<String, usize>,
    admissibility_rejections: usize,
    admissibility_diagnostics: AdmissibilityDiagnostics,
    terminal_prefix_bar_prunes: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct OnlinePrefixWorkItem {
    clause_kappa: u16,
    prefix_telescope: Telescope,
    signature: PrefixSignature,
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
    let mut incremental_trivial_derivability_hits = 0usize;
    let mut incremental_trivial_derivability_prunes = 0usize;
    let mut incremental_terminal_admissibility_hits = 0usize;
    let mut incremental_terminal_admissibility_rejections = 0usize;
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
        incremental_trivial_derivability_hits = legality_stats.trivial_derivability_hits;
        incremental_trivial_derivability_prunes = legality_stats.trivial_derivability_prunes;
        incremental_terminal_admissibility_hits = legality_stats.terminal_admissibility_hits;
        incremental_terminal_admissibility_rejections =
            legality_stats.terminal_admissibility_rejections;
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
        + incremental_terminal_prefix_bar_prunes;

    if admissibility_mode == AdmissibilityMode::RealisticShadow {
        for signature in &prefix_frontier.retained_prefix_signatures {
            let Some(group) = prefix_cache.get(signature) else {
                continue;
            };
            let mut group_telescopes = group.telescopes.clone();
            group_telescopes
                .sort_by_key(|telescope| serde_json::to_string(telescope).expect("serialize"));
            for telescope in group_telescopes {
                let candidate = evaluate_checked_candidate(library, history, telescope)?;
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
        incremental_trivial_derivability_hits,
        incremental_trivial_derivability_prunes,
        incremental_terminal_admissibility_hits,
        incremental_terminal_admissibility_rejections,
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
    incremental_trivial_derivability_hits: usize,
    incremental_trivial_derivability_prunes: usize,
    incremental_terminal_admissibility_hits: usize,
    incremental_terminal_admissibility_rejections: usize,
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
        incremental_trivial_derivability_hits,
        incremental_trivial_derivability_prunes,
        incremental_terminal_admissibility_hits,
        incremental_terminal_admissibility_rejections,
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

        let mut frontier = clause_catalog
            .clauses_at(0)
            .iter()
            .filter_map(|clause| {
                let prefix_telescope = Telescope::new(vec![clause.clone()]);
                let signature = PrefixSignature::new(step_index, library, &prefix_telescope);
                discovery
                    .prefix_legality_cache
                    .insert_root(
                        signature.clone(),
                        clause_kappa,
                        library,
                        &prefix_telescope,
                        admissibility,
                    )
                    .then_some(OnlinePrefixWorkItem {
                        clause_kappa,
                        prefix_telescope,
                        signature,
                    })
            })
            .collect::<Vec<_>>();
        discovery.prefixes_created += frontier.len();

        while let Some(work_item) = pop_best_prefix(&mut frontier) {
            discovery.prefix_states_explored += 1;
            let prefix_len = work_item.prefix_telescope.clauses.len();

            if prefix_len + 1 == usize::from(work_item.clause_kappa) {
                record_terminal_prefix_group(
                    step_index,
                    library,
                    history,
                    admissibility,
                    retention_policy,
                    objective_bar,
                    nu_history,
                    &work_item.signature,
                    &work_item.prefix_telescope,
                    clause_catalog.clauses_at(prefix_len),
                    &mut discovery,
                )?;
                continue;
            }

            let filtered_clauses = discovery
                .prefix_legality_cache
                .filter_active_window_clauses(
                    &work_item.signature,
                    prefix_len,
                    library,
                    admissibility,
                    clause_catalog.clauses_at(prefix_len),
                )
                .unwrap_or_else(|| clause_catalog.clauses_at(prefix_len).iter().collect());

            for clause in filtered_clauses {
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
                frontier.push(OnlinePrefixWorkItem {
                    clause_kappa: work_item.clause_kappa,
                    prefix_telescope: child_prefix,
                    signature: child_signature,
                });
            }
        }
    }

    Ok(discovery)
}

fn record_terminal_prefix_group(
    step_index: u32,
    library: &Library,
    history: &[DiscoveryRecord],
    admissibility: StrictAdmissibility,
    retention_policy: RetentionPolicy,
    objective_bar: Rational,
    nu_history: &[(u32, u32)],
    prefix_signature: &PrefixSignature,
    prefix_telescope: &Telescope,
    last_clause_options: &[pen_core::clause::ClauseRec],
    discovery: &mut RealisticShadowDiscovery,
) -> Result<()> {
    let filtered_clauses = discovery
        .prefix_legality_cache
        .filter_active_window_clauses(
            prefix_signature,
            prefix_telescope.clauses.len(),
            library,
            admissibility,
            last_clause_options,
        )
        .unwrap_or_else(|| last_clause_options.iter().collect());
    let mut retained_telescopes = Vec::new();
    let mut retained_bound: Option<PrefixBound> = None;

    for clause in filtered_clauses {
        let Some(connectivity_decision) = discovery.prefix_legality_cache.terminal_connectivity(
            prefix_signature,
            library,
            clause,
        ) else {
            continue;
        };
        if matches!(
            connectivity_decision,
            TerminalConnectivityDecision::PruneDisconnected
        ) {
            continue;
        }

        let cached_admissibility_decision = discovery.prefix_legality_cache.terminal_admissibility(
            step_index,
            prefix_signature,
            library,
            clause,
            admissibility,
        );
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

        discovery.enumerated_candidates += 1;
        discovery.well_formed_candidates += 1;
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
        discovery
            .admissibility_diagnostics
            .record(&admissibility_decision);
        if !admissibility_decision.is_admitted() {
            discovery.admissibility_rejections += 1;
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
        if let Some(bound) = retained_bound.as_mut() {
            bound.absorb_bound(completion_bound);
        } else {
            retained_bound = Some(completion_bound);
        }
        retained_telescopes.push(telescope);
    }

    let Some(retained_bound) = retained_bound else {
        return Ok(());
    };
    if !retained_bound.can_clear_bar(objective_bar) {
        discovery.terminal_prefix_bar_prunes += 1;
        return Ok(());
    }

    discovery.prefix_cache.record_group_with_bound(
        step_index,
        prefix_telescope.clone(),
        retained_telescopes,
        retained_bound,
        library,
        history,
        retention_policy,
    )?;

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

fn prefix_frontier_work_key(item: &OnlinePrefixWorkItem) -> (u16, u32, usize, String) {
    (
        item.clause_kappa,
        item.prefix_telescope.bit_cost(),
        item.prefix_telescope.kappa(),
        serde_json::to_string(&item.prefix_telescope).expect("prefix telescope should serialize"),
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
        AtomicSearchStep, LIVE_BOOTSTRAP_MAX_STEP, search_bootstrap_from_prefix,
        search_bootstrap_from_prefix_for_profile_with_runtime, search_bootstrap_prefix,
        supports_live_atomic_search,
    };
    use crate::config::SearchProfile;
    use crate::expand::evaluate_candidate;
    use pen_core::{
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
        assert!(step.incremental_trivial_derivability_hits > 0);
        assert!(step.incremental_terminal_admissibility_hits > 0);
        assert!(step.incremental_terminal_prefix_bar_prunes > 0);
        assert!(
            step.admissibility_rejections
                + step
                    .admissibility_diagnostics
                    .structural_debt_cap_rejections
                + step.incremental_active_window_clause_filter_prunes
                > 0
        );
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
        assert_eq!(step.prefix_frontier_hot_states, 2);
        assert_eq!(step.prefix_frontier_cold_states, 0);
        assert!(step.incremental_legality_cache_hits > 0);
        assert!(step.incremental_connectivity_fallbacks > 0);
        assert!(step.incremental_clause_family_filter_hits > 0);
        assert!(step.incremental_active_window_clause_filter_hits > 0);
        assert!(step.incremental_trivial_derivability_hits > 0);
        assert!(step.incremental_terminal_admissibility_hits > 0);
        assert!(
            step.prefix_frontier_hot_states + step.prefix_frontier_cold_states
                <= step.prefix_states_explored
        );
        assert_eq!(step.full_telescopes_evaluated, step.evaluated_candidates);
        assert_eq!(step.canonical_dedupe_prunes, step.dedupe_prunes);
        assert_eq!(step.semantic_minimality_prunes, step.minimality_prunes);
        assert_eq!(step.frontier_window.total_len(), 2);
        assert_eq!(step.frontier_dedupe_keys.len(), 2);
    }
}
