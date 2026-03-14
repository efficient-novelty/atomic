use pen_core::canonical::CanonKey;
use pen_core::rational::Rational;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PruneClass {
    SoundImpossible,
    QuotientDedupe,
    HeuristicShaping,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BranchDecision {
    Keep,
    Prune(PruneClass),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AcceptRank {
    pub overshoot: Rational,
    pub clause_kappa: u16,
    pub descending_eliminator_score: std::cmp::Reverse<u16>,
    pub descending_former_score: std::cmp::Reverse<u16>,
    pub descending_dependent_motive_density: std::cmp::Reverse<u16>,
    pub descending_library_reference_density: std::cmp::Reverse<u16>,
    pub max_var_ref: u32,
    pub descending_generic_binder_count: std::cmp::Reverse<u16>,
    pub descending_closure_score: std::cmp::Reverse<u16>,
    pub bit_kappa: u16,
    pub descending_nu: std::cmp::Reverse<u16>,
    pub canonical_key: CanonKey,
}

pub fn sound_prune_by_bar(
    nu_upper_bound: u16,
    clause_kappa_used: u16,
    bar: Rational,
) -> BranchDecision {
    if clause_kappa_used == 0 {
        return BranchDecision::Prune(PruneClass::SoundImpossible);
    }

    let rho_upper = Rational::new(i64::from(nu_upper_bound), i64::from(clause_kappa_used));
    if rho_upper < bar {
        BranchDecision::Prune(PruneClass::SoundImpossible)
    } else {
        BranchDecision::Keep
    }
}

pub fn better_rank(left: &AcceptRank, right: &AcceptRank) -> bool {
    left < right
}

#[cfg(test)]
mod tests {
    use super::{AcceptRank, BranchDecision, PruneClass, better_rank, sound_prune_by_bar};
    use pen_core::canonical::CanonKey;
    use pen_core::rational::Rational;
    use std::cmp::Reverse;

    #[test]
    fn upper_bound_prune_is_exact() {
        assert_eq!(
            sound_prune_by_bar(10, 4, Rational::new(3, 1)),
            BranchDecision::Prune(PruneClass::SoundImpossible)
        );
        assert_eq!(
            sound_prune_by_bar(17, 4, Rational::new(4, 1)),
            BranchDecision::Keep
        );
    }

    #[test]
    fn accept_rank_prefers_lower_overshoot_then_lower_kappa() {
        let lower = AcceptRank {
            overshoot: Rational::new(1, 10),
            clause_kappa: 4,
            descending_eliminator_score: Reverse(3),
            descending_former_score: Reverse(2),
            descending_dependent_motive_density: Reverse(2),
            descending_library_reference_density: Reverse(1),
            descending_generic_binder_count: Reverse(4),
            descending_closure_score: Reverse(3),
            max_var_ref: 3,
            bit_kappa: 78,
            descending_nu: Reverse(17),
            canonical_key: CanonKey("a".to_owned()),
        };
        let higher = AcceptRank {
            overshoot: Rational::new(2, 10),
            clause_kappa: 3,
            descending_eliminator_score: Reverse(1),
            descending_former_score: Reverse(1),
            descending_dependent_motive_density: Reverse(1),
            descending_library_reference_density: Reverse(0),
            descending_generic_binder_count: Reverse(2),
            descending_closure_score: Reverse(1),
            max_var_ref: 4,
            bit_kappa: 70,
            descending_nu: Reverse(18),
            canonical_key: CanonKey("b".to_owned()),
        };

        assert!(better_rank(&lower, &higher));
    }
}
