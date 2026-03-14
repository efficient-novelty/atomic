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

pub const LIVE_BOOTSTRAP_MAX_STEP: u32 = 11;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtomicSearchStep {
    pub step_index: u32,
    pub objective_bar: Rational,
    pub admissibility: StrictAdmissibility,
    pub telescope: Telescope,
    pub accepted: ExpandedCandidate,
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
        near_misses: acceptance.near_misses,
        evaluated_candidates,
        dedupe_prunes,
        heuristic_drops,
    })
}

#[cfg(test)]
mod tests {
    use super::{LIVE_BOOTSTRAP_MAX_STEP, search_bootstrap_prefix, supports_live_atomic_search};
    use pen_core::telescope::Telescope;

    #[test]
    fn live_search_support_is_honest_about_current_bootstrap_range() {
        assert!(supports_live_atomic_search(LIVE_BOOTSTRAP_MAX_STEP));
        assert!(!supports_live_atomic_search(LIVE_BOOTSTRAP_MAX_STEP + 1));
    }

    #[test]
    fn bootstrap_search_discovers_the_first_eleven_reference_telescopes() {
        let steps = search_bootstrap_prefix(11, 2).expect("bootstrap search should succeed");
        assert_eq!(steps.len(), 11);
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
    }
}
