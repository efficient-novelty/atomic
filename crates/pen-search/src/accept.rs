use crate::branch_bound::AcceptRank;
use crate::expand::{ExpandedCandidate, StructuralSignals, structural_signals_for_telescope};
use pen_core::canonical::{
    CanonKey, canonical_encoded_expr, canonical_encoded_telescope, canonical_key_from_encoded,
    canonical_key_telescope,
};
use pen_core::clause::ClauseRec;
use pen_core::rational::Rational;
use pen_core::telescope::Telescope;
use pen_eval::nu::TerminalClauseNuFacts;
use pen_store::manifest::{AcceptedCandidate, NearMiss};
use std::cmp::Reverse;
use std::collections::BTreeSet;

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrefixLocalAcceptRankContext {
    prefix_signals: StructuralSignals,
    prefix_lib_refs: BTreeSet<u32>,
    prefix_var_refs: BTreeSet<u32>,
    prefix_max_var_ref: u32,
    prefix_canonical_encoding: String,
}

pub(crate) fn acceptance_rank_context_for_prefix(
    prefix_telescope: &Telescope,
) -> PrefixLocalAcceptRankContext {
    let prefix_var_refs = prefix_telescope.var_refs();
    PrefixLocalAcceptRankContext {
        prefix_signals: structural_signals_for_telescope(prefix_telescope),
        prefix_lib_refs: prefix_telescope.lib_refs(),
        prefix_max_var_ref: prefix_var_refs.iter().next_back().copied().unwrap_or(0),
        prefix_var_refs,
        prefix_canonical_encoding: canonical_encoded_telescope(prefix_telescope),
    }
}

pub(crate) fn acceptance_rank_for_prefix_clause(
    bar: Rational,
    prefix_context: &PrefixLocalAcceptRankContext,
    terminal_clause: &ClauseRec,
    clause_nu_facts: &TerminalClauseNuFacts,
    exact_nu: u16,
    bit_kappa: u16,
    clause_kappa: u16,
) -> Option<AcceptRank> {
    let rho = Rational::new(i64::from(exact_nu), i64::from(clause_kappa));
    if rho < bar {
        return None;
    }

    let clause_signals = clause_nu_facts.acceptance_signals();
    let signals = StructuralSignals {
        eliminator_score: prefix_context
            .prefix_signals
            .eliminator_score
            .saturating_add(clause_signals.eliminator_score),
        former_score: prefix_context
            .prefix_signals
            .former_score
            .saturating_add(clause_signals.former_score),
        dependent_motive_density: prefix_context
            .prefix_signals
            .dependent_motive_density
            .saturating_add(clause_signals.dependent_motive_density),
        library_reference_density: u16::try_from(sorted_union_count(
            prefix_context.prefix_lib_refs.iter().copied(),
            clause_nu_facts.lib_refs().iter().copied(),
        ))
        .expect("library ref count exceeded u16"),
        generic_binder_count: prefix_context
            .prefix_signals
            .generic_binder_count
            .saturating_add(clause_signals.generic_binder_count),
        closure_score: u16::try_from(sorted_union_count(
            prefix_context.prefix_var_refs.iter().copied(),
            clause_nu_facts.var_refs().iter().copied(),
        ))
        .expect("closure score exceeded u16"),
    };
    let max_var_ref = prefix_context
        .prefix_max_var_ref
        .max(clause_nu_facts.max_var_ref());
    let canonical_key = canonical_key_for_prefix_clause(
        prefix_context.prefix_canonical_encoding.as_str(),
        terminal_clause,
    );

    Some(AcceptRank {
        overshoot: rho - bar,
        clause_kappa,
        descending_eliminator_score: Reverse(signals.eliminator_score),
        descending_former_score: Reverse(signals.former_score),
        descending_dependent_motive_density: Reverse(signals.dependent_motive_density),
        descending_library_reference_density: Reverse(signals.library_reference_density),
        descending_generic_binder_count: Reverse(signals.generic_binder_count),
        descending_closure_score: Reverse(signals.closure_score),
        max_var_ref,
        bit_kappa,
        descending_nu: Reverse(exact_nu),
        canonical_key,
    })
}

fn canonical_key_for_prefix_clause(prefix_encoding: &str, terminal_clause: &ClauseRec) -> CanonKey {
    let clause_encoding = canonical_encoded_expr(&terminal_clause.expr);
    let mut encoded =
        String::with_capacity(prefix_encoding.len().saturating_add(clause_encoding.len()));
    encoded.push_str(prefix_encoding);
    encoded.push_str(&clause_encoding);
    canonical_key_from_encoded(&encoded)
}

fn sorted_union_count<I, J>(left: I, right: J) -> usize
where
    I: IntoIterator<Item = u32>,
    J: IntoIterator<Item = u32>,
{
    let mut left = left.into_iter().peekable();
    let mut right = right.into_iter().peekable();
    let mut count = 0usize;

    loop {
        match (left.peek(), right.peek()) {
            (Some(left_value), Some(right_value)) => {
                if left_value < right_value {
                    left.next();
                } else if right_value < left_value {
                    right.next();
                } else {
                    left.next();
                    right.next();
                }
                count += 1;
            }
            (Some(_), None) => {
                count += left.count();
                break;
            }
            (None, Some(_)) => {
                count += right.count();
                break;
            }
            (None, None) => break,
        }
    }

    count
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
        acceptance_rank_context_for_prefix, acceptance_rank_for_prefix_clause,
        acceptance_rank_for_telescope, select_acceptance,
    };
    use crate::expand::evaluate_candidate;
    use pen_core::encode::telescope_bit_cost;
    use pen_core::library::{Library, LibraryEntry};
    use pen_core::rational::Rational;
    use pen_core::telescope::Telescope;
    use pen_eval::bar::DiscoveryRecord;
    use pen_eval::nu::TerminalClauseNuFacts;

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
    fn prefix_clause_acceptance_rank_matches_full_telescope_rank() {
        for step in 4..=15 {
            let (library, history) = replay_prefix(step - 1);
            let telescope = Telescope::reference(step);
            let evaluated = evaluate_candidate(&library, &history, telescope.clone())
                .expect("reference telescope should evaluate");
            let clause_kappa = u16::try_from(telescope.kappa()).expect("kappa exceeded u16");
            let bit_kappa =
                u16::try_from(telescope_bit_cost(&telescope)).expect("bit cost exceeded u16");
            let prefix = Telescope::new(telescope.clauses[..telescope.clauses.len() - 1].to_vec());
            let terminal_clause = telescope
                .clauses
                .last()
                .expect("reference telescope should have a final clause");
            let prefix_context = acceptance_rank_context_for_prefix(&prefix);
            let objective_bar = Rational::zero();

            assert_eq!(
                acceptance_rank_for_prefix_clause(
                    objective_bar,
                    &prefix_context,
                    terminal_clause,
                    &TerminalClauseNuFacts::from_clause(terminal_clause),
                    evaluated.nu,
                    bit_kappa,
                    clause_kappa,
                ),
                acceptance_rank_for_telescope(
                    objective_bar,
                    &telescope,
                    evaluated.nu,
                    bit_kappa,
                    clause_kappa,
                )
            );
        }
    }
}
