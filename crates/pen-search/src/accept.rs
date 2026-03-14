use crate::branch_bound::AcceptRank;
use crate::expand::ExpandedCandidate;
use pen_core::rational::Rational;
use pen_store::manifest::{AcceptedCandidate, NearMiss};
use std::cmp::Reverse;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptanceOutcome {
    pub accepted: AcceptedCandidate,
    pub near_misses: Vec<NearMiss>,
}

pub fn select_acceptance(
    bar: Rational,
    candidates: &[ExpandedCandidate],
) -> Option<AcceptanceOutcome> {
    let best = candidates
        .iter()
        .filter_map(|candidate| acceptance_rank(bar, candidate).map(|rank| (candidate, rank)))
        .min_by(|(_, left), (_, right)| left.cmp(right))?
        .0;

    let overshoot = best.rho - bar;
    let accepted = AcceptedCandidate {
        candidate_hash: best.candidate_hash.clone(),
        canonical_hash: best.canonical_hash.clone(),
        bit_kappa: best.bit_kappa,
        clause_kappa: best.clause_kappa,
        nu: best.nu,
        rho: best.rho,
        overshoot,
        shape_fingerprint: best.shape_fingerprint.clone(),
        support_fingerprint: best.support_fingerprint.clone(),
    };

    let near_misses = collect_near_misses(bar, candidates, &best.canonical_hash);

    Some(AcceptanceOutcome {
        accepted,
        near_misses,
    })
}

fn acceptance_rank(bar: Rational, candidate: &ExpandedCandidate) -> Option<AcceptRank> {
    (candidate.rho >= bar).then(|| AcceptRank {
        overshoot: candidate.rho - bar,
        clause_kappa: candidate.clause_kappa,
        descending_eliminator_score: Reverse(candidate.signals.eliminator_score),
        descending_former_score: Reverse(candidate.signals.former_score),
        descending_dependent_motive_density: Reverse(candidate.signals.dependent_motive_density),
        descending_library_reference_density: Reverse(candidate.signals.library_reference_density),
        descending_generic_binder_count: Reverse(candidate.signals.generic_binder_count),
        descending_closure_score: Reverse(candidate.signals.closure_score),
        max_var_ref: candidate
            .telescope
            .var_refs()
            .iter()
            .next_back()
            .copied()
            .unwrap_or(0),
        bit_kappa: candidate.bit_kappa,
        descending_nu: Reverse(candidate.nu),
        canonical_key: candidate.canonical_key.clone(),
    })
}

fn collect_near_misses(
    bar: Rational,
    candidates: &[ExpandedCandidate],
    selected_hash: &str,
) -> Vec<NearMiss> {
    let mut rows: Vec<(bool, Rational, &ExpandedCandidate)> = candidates
        .iter()
        .filter(|candidate| candidate.canonical_hash != selected_hash)
        .map(|candidate| {
            if candidate.rho >= bar {
                (true, candidate.rho - bar, candidate)
            } else {
                (false, bar - candidate.rho, candidate)
            }
        })
        .collect();

    rows.sort_by_key(|(clears_bar, delta, candidate)| {
        (
            !clears_bar,
            *delta,
            candidate.clause_kappa,
            Reverse(candidate.nu),
            candidate.canonical_key.clone(),
        )
    });

    rows.into_iter()
        .take(3)
        .map(|(clears_bar, _, candidate)| NearMiss {
            candidate_hash: candidate.candidate_hash.clone(),
            canonical_hash: candidate.canonical_hash.clone(),
            bit_kappa: candidate.bit_kappa,
            clause_kappa: candidate.clause_kappa,
            nu: candidate.nu,
            status: if clears_bar {
                "bar_clear_higher_overshoot".to_owned()
            } else {
                "below_bar".to_owned()
            },
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::select_acceptance;
    use crate::expand::evaluate_candidate;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::rational::Rational;
    use pen_core::telescope::Telescope;
    use pen_eval::bar::DiscoveryRecord;

    fn replay_prefix(last_step: u32) -> (Library, Vec<DiscoveryRecord>) {
        let mut library = Vec::new();
        let mut history = Vec::new();

        for step in 1..=last_step {
            let telescope = Telescope::reference(step);
            let evaluated = evaluate_candidate(&library, &history, telescope.clone())
                .expect("reference telescope should evaluate");
            history.push(DiscoveryRecord::new(
                step,
                u32::from(evaluated.nu),
                u32::from(evaluated.clause_kappa),
            ));
            library.push(LibraryEntry::from_telescope(&telescope, &library));
        }

        (library, history)
    }

    #[test]
    fn acceptance_prefers_minimal_positive_overshoot() {
        let (library, history) = replay_prefix(8);
        let hopf = evaluate_candidate(&library, &history, Telescope::reference(9))
            .expect("hopf should evaluate");
        let sphere = evaluate_candidate(&library, &history, Telescope::reference(8))
            .expect("sphere should evaluate");
        let bar = Rational::new(4, 1);

        let outcome = select_acceptance(bar, &[hopf.clone(), sphere]).expect("winner");
        assert_eq!(outcome.accepted.nu, hopf.nu);
        assert_eq!(outcome.accepted.overshoot, Rational::new(1, 4));
        assert_eq!(outcome.near_misses[0].status, "below_bar");
    }
}
