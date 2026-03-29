use crate::branch_bound::AcceptRank;
use crate::expand::{
    ExpandedCandidate, StructuralSignals, structural_signals_for_expr,
    structural_signals_for_telescope,
};
use pen_core::canonical::canonical_key_telescope;
use pen_core::clause::ClauseRec;
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_store::manifest::{AcceptedCandidate, NearMiss};
use std::cmp::Reverse;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AcceptRankPrefixContext {
    structural_signals: StructuralSignals,
    library_refs: BTreeSet<u32>,
    var_refs: BTreeSet<u32>,
    max_var_ref: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TerminalClauseAcceptRankMetadata {
    structural_signals: StructuralSignals,
    library_refs: BTreeSet<u32>,
    var_refs: BTreeSet<u32>,
    max_var_ref: u32,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) struct AcceptRankNumericFields {
    pub overshoot: Rational,
    pub clause_kappa: u16,
    pub descending_eliminator_score: Reverse<u16>,
    pub descending_former_score: Reverse<u16>,
    pub descending_dependent_motive_density: Reverse<u16>,
    pub descending_library_reference_density: Reverse<u16>,
    pub max_var_ref: u32,
    pub descending_generic_binder_count: Reverse<u16>,
    pub descending_closure_score: Reverse<u16>,
    pub bit_kappa: u16,
    pub descending_nu: Reverse<u16>,
}

pub(crate) fn accept_rank_prefix_context(prefix_telescope: &Telescope) -> AcceptRankPrefixContext {
    let library_refs = prefix_telescope.lib_refs();
    let var_refs = prefix_telescope.var_refs();
    let max_var_ref = var_refs.iter().next_back().copied().unwrap_or(0);
    AcceptRankPrefixContext {
        structural_signals: structural_signals_for_telescope(prefix_telescope),
        library_refs,
        var_refs,
        max_var_ref,
    }
}

pub(crate) fn terminal_clause_accept_rank_metadata(
    clause: &ClauseRec,
) -> TerminalClauseAcceptRankMetadata {
    let library_refs = clause.expr.lib_refs();
    let var_refs = clause.expr.var_refs();
    let max_var_ref = var_refs.iter().next_back().copied().unwrap_or(0);
    TerminalClauseAcceptRankMetadata {
        structural_signals: structural_signals_for_expr(&clause.expr),
        library_refs,
        var_refs,
        max_var_ref,
    }
}

pub(crate) fn acceptance_rank_numeric_fields(
    bar: Rational,
    exact_nu: u16,
    bit_kappa: u16,
    clause_kappa: u16,
    prefix_context: &AcceptRankPrefixContext,
    clause_metadata: &TerminalClauseAcceptRankMetadata,
) -> Option<AcceptRankNumericFields> {
    let rho = Rational::new(i64::from(exact_nu), i64::from(clause_kappa));
    if rho < bar {
        return None;
    }

    let prefix_signals = &prefix_context.structural_signals;
    let clause_signals = &clause_metadata.structural_signals;
    let library_reference_density = u16::try_from(
        prefix_context
            .library_refs
            .union(&clause_metadata.library_refs)
            .count(),
    )
    .expect("library ref count exceeded u16");
    let closure_score = u16::try_from(
        prefix_context
            .var_refs
            .union(&clause_metadata.var_refs)
            .count(),
    )
    .expect("closure score exceeded u16");

    Some(AcceptRankNumericFields {
        overshoot: rho - bar,
        clause_kappa,
        descending_eliminator_score: Reverse(
            prefix_signals
                .eliminator_score
                .saturating_add(clause_signals.eliminator_score),
        ),
        descending_former_score: Reverse(
            prefix_signals
                .former_score
                .saturating_add(clause_signals.former_score),
        ),
        descending_dependent_motive_density: Reverse(
            prefix_signals
                .dependent_motive_density
                .saturating_add(clause_signals.dependent_motive_density),
        ),
        descending_library_reference_density: Reverse(library_reference_density),
        max_var_ref: prefix_context.max_var_ref.max(clause_metadata.max_var_ref),
        descending_generic_binder_count: Reverse(
            prefix_signals
                .generic_binder_count
                .saturating_add(clause_signals.generic_binder_count),
        ),
        descending_closure_score: Reverse(closure_score),
        bit_kappa,
        descending_nu: Reverse(exact_nu),
    })
}

pub(crate) fn finalize_acceptance_rank(
    numeric_fields: AcceptRankNumericFields,
    telescope: &Telescope,
) -> AcceptRank {
    AcceptRank {
        overshoot: numeric_fields.overshoot,
        clause_kappa: numeric_fields.clause_kappa,
        descending_eliminator_score: numeric_fields.descending_eliminator_score,
        descending_former_score: numeric_fields.descending_former_score,
        descending_dependent_motive_density: numeric_fields.descending_dependent_motive_density,
        descending_library_reference_density: numeric_fields.descending_library_reference_density,
        max_var_ref: numeric_fields.max_var_ref,
        descending_generic_binder_count: numeric_fields.descending_generic_binder_count,
        descending_closure_score: numeric_fields.descending_closure_score,
        bit_kappa: numeric_fields.bit_kappa,
        descending_nu: numeric_fields.descending_nu,
        canonical_key: canonical_key_telescope(telescope),
    }
}

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

pub(crate) fn acceptance_rank(bar: Rational, candidate: &ExpandedCandidate) -> Option<AcceptRank> {
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

pub(crate) fn acceptance_rank_for_telescope(
    bar: Rational,
    telescope: &Telescope,
    exact_nu: u16,
    bit_kappa: u16,
    clause_kappa: u16,
) -> Option<AcceptRank> {
    let rho = Rational::new(i64::from(exact_nu), i64::from(clause_kappa));
    if rho < bar {
        return None;
    }

    let signals = structural_signals_for_telescope(telescope);
    Some(AcceptRank {
        overshoot: rho - bar,
        clause_kappa,
        descending_eliminator_score: Reverse(signals.eliminator_score),
        descending_former_score: Reverse(signals.former_score),
        descending_dependent_motive_density: Reverse(signals.dependent_motive_density),
        descending_library_reference_density: Reverse(signals.library_reference_density),
        descending_generic_binder_count: Reverse(signals.generic_binder_count),
        descending_closure_score: Reverse(signals.closure_score),
        max_var_ref: telescope
            .var_refs()
            .iter()
            .next_back()
            .copied()
            .unwrap_or(0),
        bit_kappa,
        descending_nu: Reverse(exact_nu),
        canonical_key: canonical_key_telescope(telescope),
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
    use super::{
        accept_rank_prefix_context, acceptance_rank_for_telescope, acceptance_rank_numeric_fields,
        finalize_acceptance_rank, select_acceptance, terminal_clause_accept_rank_metadata,
    };
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

    #[test]
    fn claim_admitted_only_rank_metadata_matches_full_accept_rank() {
        let (library, history) = replay_prefix(14);
        let bar = Rational::zero();
        let target = Telescope::reference(15);
        let prefix = Telescope::new(target.clauses[..target.clauses.len() - 1].to_vec());
        let clause = target
            .clauses
            .last()
            .expect("reference target should have a terminal clause")
            .clone();
        let exact_nu = u16::try_from(
            evaluate_candidate(&library, &history, target.clone())
                .expect("reference target should evaluate")
                .nu,
        )
        .expect("nu should fit in u16");
        let bit_kappa =
            u16::try_from(target.bit_cost()).expect("reference target bit cost should fit in u16");
        let clause_kappa =
            u16::try_from(target.kappa()).expect("reference target kappa should fit in u16");

        let prefix_context = accept_rank_prefix_context(&prefix);
        let clause_metadata = terminal_clause_accept_rank_metadata(&clause);
        let numeric_fields = acceptance_rank_numeric_fields(
            bar,
            exact_nu,
            bit_kappa,
            clause_kappa,
            &prefix_context,
            &clause_metadata,
        )
        .expect("reference target should clear zero bar");
        let lazy_rank = finalize_acceptance_rank(numeric_fields, &target);
        let direct_rank =
            acceptance_rank_for_telescope(bar, &target, exact_nu, bit_kappa, clause_kappa)
                .expect("reference target should produce an accept rank");

        assert_eq!(lazy_rank, direct_rank);
    }
}
