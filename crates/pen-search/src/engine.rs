use crate::accept::{AcceptanceOutcome, select_acceptance};
use crate::diversify::{FrontierPressure, FrontierRuntimeLimits, plan_pressure_cold_retention};
use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use crate::expand::{ExpandedCandidate, evaluate_candidate, evaluate_checked_candidate};
use anyhow::{Result, bail};
use pen_core::library::{Library, LibraryEntry};
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_eval::bar::{DiscoveryRecord, compute_bar};
use pen_eval::minimality::analyze_semantic_minimality;
use pen_store::manifest::NearMiss;
use pen_type::admissibility::{
    StrictAdmissibility, passes_strict_admissibility, strict_admissibility,
};
use pen_type::check::{CheckResult, check_telescope};
use pen_type::obligations::{RetentionPolicy, summarize_structural_debt};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

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
    pub retention_policy: RetentionPolicy,
    pub frontier_retention: FrontierRetentionOutcome,
    pub telescope: Telescope,
    pub accepted: ExpandedCandidate,
    pub retained_candidates: Vec<ExpandedCandidate>,
    pub near_misses: Vec<NearMiss>,
    pub enumerated_candidates: usize,
    pub well_formed_candidates: usize,
    pub malformed_rejections: usize,
    pub malformed_rejection_reasons: BTreeMap<String, usize>,
    pub admissibility_rejections: usize,
    pub evaluated_candidates: usize,
    pub canonical_candidates: usize,
    pub semantically_minimal_candidates: usize,
    pub scored_candidate_distribution: CandidateScoreDistribution,
    pub dedupe_prunes: usize,
    pub minimality_prunes: usize,
    pub heuristic_drops: usize,
    pub dedupe_pruned_candidates: Vec<DedupePruneEvidence>,
    pub minimality_pruned_candidates: Vec<MinimalityPruneEvidence>,
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
    search_bootstrap_from_prefix_with_runtime(&[], until_step, window_depth, retention_runtime)
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
    let mut library: Library = Vec::new();
    let mut history: Vec<DiscoveryRecord> = Vec::new();
    let mut steps = Vec::new();

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
    retention_runtime: FrontierRuntimeLimits,
) -> Result<AtomicSearchStep> {
    let structural_debt = summarize_structural_debt(library, window_depth);
    let admissibility = strict_admissibility(step_index, window_depth, library);
    let retention_policy = structural_debt.retention_policy();
    let objective_bar = compute_bar(window_depth as usize, step_index, history).bar;
    let mut candidates = Vec::new();
    let mut enumerated_candidates = 0usize;
    let mut well_formed_candidates = 0usize;
    let mut malformed_rejections = 0usize;
    let mut malformed_rejection_reasons = BTreeMap::new();
    let mut admissibility_rejections = 0usize;
    let nu_history = history
        .iter()
        .map(|record| (record.step_index, record.nu))
        .collect::<Vec<_>>();

    for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa {
        let telescopes = enumerate_telescopes(
            library,
            EnumerationContext {
                library_size: library.len() as u32,
                scope_size: admissibility.ambient_depth,
                max_path_dimension: admissibility.max_path_dimension,
                include_trunc: admissibility.include_trunc,
                include_modal: admissibility.include_modal,
                include_temporal: admissibility.include_temporal,
                max_expr_nodes: admissibility.max_expr_nodes,
                require_former_eliminator_clauses: admissibility.require_former_eliminator_package,
                require_initial_hit_clauses: admissibility.require_initial_hit_package,
                require_truncation_hit_clauses: admissibility.require_truncation_hit_package,
                require_higher_hit_clauses: admissibility.require_higher_hit_package,
                require_sphere_lift_clauses: admissibility.require_sphere_lift_package,
                require_axiomatic_bundle_clauses: admissibility.require_axiomatic_bundle_package,
                require_modal_shell_clauses: admissibility.require_modal_shell_package,
                require_connection_shell_clauses: admissibility.require_connection_shell_package,
                require_curvature_shell_clauses: admissibility.require_curvature_shell_package,
                require_operator_bundle_clauses: admissibility.require_operator_bundle_package,
                require_hilbert_functional_clauses: admissibility
                    .require_hilbert_functional_package,
                require_temporal_shell_clauses: admissibility.require_temporal_shell_package,
                historical_anchor_ref: admissibility.historical_anchor_ref,
            },
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
            if !passes_strict_admissibility(step_index, library, &telescope, admissibility) {
                admissibility_rejections += 1;
                continue;
            }
            let candidate = evaluate_checked_candidate(library, history, telescope)?;
            candidates.push(candidate);
        }
    }

    if candidates.is_empty() {
        bail!("no atomic candidates were generated for step {step_index}");
    }
    let evaluated_candidates = candidates.len();
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

    build_step_result(
        step_index,
        objective_bar,
        admissibility,
        retention_policy,
        frontier_retention,
        acceptance,
        enumerated_candidates,
        well_formed_candidates,
        malformed_rejections,
        malformed_rejection_reasons,
        admissibility_rejections,
        evaluated_candidates,
        canonical_candidates,
        semantically_minimal_candidates,
        scored_candidate_distribution,
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
    retention_policy: RetentionPolicy,
    frontier_retention: FrontierRetentionOutcome,
    acceptance: AcceptanceOutcome,
    enumerated_candidates: usize,
    well_formed_candidates: usize,
    malformed_rejections: usize,
    malformed_rejection_reasons: BTreeMap<String, usize>,
    admissibility_rejections: usize,
    evaluated_candidates: usize,
    canonical_candidates: usize,
    semantically_minimal_candidates: usize,
    scored_candidate_distribution: CandidateScoreDistribution,
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
        retention_policy,
        frontier_retention,
        telescope: accepted.telescope.clone(),
        accepted,
        retained_candidates: retained.to_vec(),
        near_misses: acceptance.near_misses,
        enumerated_candidates,
        well_formed_candidates,
        malformed_rejections,
        malformed_rejection_reasons,
        admissibility_rejections,
        evaluated_candidates,
        canonical_candidates,
        semantically_minimal_candidates,
        scored_candidate_distribution,
        dedupe_prunes,
        minimality_prunes,
        heuristic_drops,
        dedupe_pruned_candidates,
        minimality_pruned_candidates,
    })
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

#[cfg(test)]
mod tests {
    use super::{
        LIVE_BOOTSTRAP_MAX_STEP, search_bootstrap_from_prefix, search_bootstrap_prefix,
        supports_live_atomic_search,
    };
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
        admissibility::{StrictAdmissibility, passes_strict_admissibility, strict_admissibility},
        connectivity::{ConnectivityWitness, analyze_connectivity, passes_connectivity},
    };

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
}
