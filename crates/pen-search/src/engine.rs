use crate::accept::{AcceptanceOutcome, select_acceptance};
use crate::dedupe::dedupe_stable_by;
use crate::enumerate::{EnumerationContext, enumerate_telescopes};
use crate::expand::{ExpandedCandidate, evaluate_candidate};
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

pub const LIVE_BOOTSTRAP_MAX_STEP: u32 = 15;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtomicSearchStep {
    pub step_index: u32,
    pub objective_bar: Rational,
    pub admissibility: StrictAdmissibility,
    pub telescope: Telescope,
    pub accepted: ExpandedCandidate,
    pub retained_candidates: Vec<ExpandedCandidate>,
    pub near_misses: Vec<NearMiss>,
    pub evaluated_candidates: usize,
    pub dedupe_prunes: usize,
    pub heuristic_drops: usize,
}

pub fn supports_live_atomic_search(until_step: u32) -> bool {
    until_step <= LIVE_BOOTSTRAP_MAX_STEP
}

pub fn search_bootstrap_prefix(
    until_step: u32,
    window_depth: u16,
) -> Result<Vec<AtomicSearchStep>> {
    let mut library: Library = Vec::new();
    let mut history: Vec<DiscoveryRecord> = Vec::new();
    let mut steps = Vec::new();

    for step_index in 1..=until_step.min(LIVE_BOOTSTRAP_MAX_STEP) {
        let outcome = search_next_step(step_index, window_depth, &library, &history)?;
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
) -> Result<AtomicSearchStep> {
    let admissibility = strict_admissibility(step_index, window_depth, library);
    let objective_bar = compute_bar(window_depth as usize, step_index, history).bar;
    let mut candidates = Vec::new();
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

        for telescope in telescopes {
            if !passes_strict_admissibility(step_index, library, &telescope, admissibility) {
                continue;
            }
            let candidate = evaluate_candidate(library, history, telescope)?;
            candidates.push(candidate);
        }
    }

    if candidates.is_empty() {
        bail!("no atomic candidates were generated for step {step_index}");
    }
    let evaluated_candidates = candidates.len();

    let (deduped, dedupe_prunes, _) =
        dedupe_stable_by(candidates, |candidate| candidate.canonical_key.clone());
    let minimal = deduped
        .into_iter()
        .filter(|candidate| {
            analyze_semantic_minimality(
                step_index,
                objective_bar,
                admissibility,
                &candidate.telescope,
                library,
                &nu_history,
            )
            .is_minimal()
        })
        .collect::<Vec<_>>();
    let minimality_prunes = evaluated_candidates
        .saturating_sub(dedupe_prunes)
        .saturating_sub(minimal.len());
    // The bounded bootstrap path already evaluates a complete candidate pool, so
    // shaping it before exact acceptance can only hide real winners.
    let retained = minimal;
    let heuristic_drops = 0;
    if retained.is_empty() {
        bail!("no semantically minimal candidates survived for step {step_index}");
    }
    let acceptance = select_acceptance(objective_bar, &retained)
        .ok_or_else(|| anyhow::anyhow!("no candidate cleared the bar at step {step_index}"))?;

    build_step_result(
        step_index,
        objective_bar,
        admissibility,
        acceptance,
        evaluated_candidates,
        dedupe_prunes,
        heuristic_drops + minimality_prunes,
        &retained,
    )
}

fn build_step_result(
    step_index: u32,
    objective_bar: Rational,
    admissibility: StrictAdmissibility,
    acceptance: AcceptanceOutcome,
    evaluated_candidates: usize,
    dedupe_prunes: usize,
    heuristic_drops: usize,
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
        telescope: accepted.telescope.clone(),
        accepted,
        retained_candidates: retained.to_vec(),
        near_misses: acceptance.near_misses,
        evaluated_candidates,
        dedupe_prunes,
        heuristic_drops,
    })
}

#[cfg(test)]
mod tests {
    use super::{LIVE_BOOTSTRAP_MAX_STEP, search_bootstrap_prefix, supports_live_atomic_search};
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
            StrictAdmissibility, passes_strict_admissibility, strict_admissibility,
        },
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
        assert!(passes_strict_admissibility(15, &library, &dct, admissibility));
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
