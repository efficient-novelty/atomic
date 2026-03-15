use crate::state::{FrontierStateRecV1, PrefixState};
use pen_core::rational::Rational;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PrefixBound {
    pub nu_lower_bound: u16,
    pub nu_upper_bound: u16,
    pub clause_kappa_used: u16,
    pub bit_kappa_used: u16,
}

impl PrefixBound {
    pub fn singleton(nu: u16, clause_kappa_used: u16, bit_kappa_used: u16) -> Self {
        Self {
            nu_lower_bound: nu,
            nu_upper_bound: nu,
            clause_kappa_used,
            bit_kappa_used,
        }
    }

    pub fn absorb_completion(
        &mut self,
        exact_nu: u16,
        clause_kappa_used: u16,
        bit_kappa_used: u16,
    ) {
        self.nu_lower_bound = self.nu_lower_bound.min(exact_nu);
        self.nu_upper_bound = self.nu_upper_bound.max(exact_nu);
        self.clause_kappa_used = self.clause_kappa_used.min(clause_kappa_used);
        self.bit_kappa_used = self.bit_kappa_used.min(bit_kappa_used);
    }

    pub fn absorb_bound(&mut self, other: Self) {
        self.nu_lower_bound = self.nu_lower_bound.min(other.nu_lower_bound);
        self.nu_upper_bound = self.nu_upper_bound.max(other.nu_upper_bound);
        self.clause_kappa_used = self.clause_kappa_used.min(other.clause_kappa_used);
        self.bit_kappa_used = self.bit_kappa_used.min(other.bit_kappa_used);
    }

    pub fn rho_lower(self) -> Option<Rational> {
        (self.clause_kappa_used != 0).then(|| {
            Rational::new(
                i64::from(self.nu_lower_bound),
                i64::from(self.clause_kappa_used),
            )
        })
    }

    pub fn rho_upper(self) -> Option<Rational> {
        (self.clause_kappa_used != 0).then(|| {
            Rational::new(
                i64::from(self.nu_upper_bound),
                i64::from(self.clause_kappa_used),
            )
        })
    }

    pub fn can_clear_bar(self, bar: Rational) -> bool {
        self.rho_upper().is_some_and(|rho_upper| rho_upper >= bar)
    }
}

impl From<PrefixState> for PrefixBound {
    fn from(prefix: PrefixState) -> Self {
        Self {
            nu_lower_bound: prefix.nu_lower_bound,
            nu_upper_bound: prefix.nu_upper_bound,
            clause_kappa_used: prefix.clause_kappa_used,
            bit_kappa_used: prefix.bit_kappa_used,
        }
    }
}

impl From<FrontierStateRecV1> for PrefixBound {
    fn from(record: FrontierStateRecV1) -> Self {
        Self {
            nu_lower_bound: record.nu_lower_bound,
            nu_upper_bound: record.nu_upper_bound,
            clause_kappa_used: record.clause_kappa_used,
            bit_kappa_used: record.bit_kappa_used,
        }
    }
}

impl From<&FrontierStateRecV1> for PrefixBound {
    fn from(record: &FrontierStateRecV1) -> Self {
        (*record).into()
    }
}

#[cfg(test)]
mod tests {
    use super::PrefixBound;
    use pen_core::rational::Rational;

    #[test]
    fn prefix_bound_tracks_exact_extrema_and_rho_bounds() {
        let mut bound = PrefixBound::singleton(17, 4, 80);
        bound.absorb_completion(21, 5, 76);
        bound.absorb_completion(15, 3, 82);

        assert_eq!(
            bound,
            PrefixBound {
                nu_lower_bound: 15,
                nu_upper_bound: 21,
                clause_kappa_used: 3,
                bit_kappa_used: 76,
            }
        );
        assert_eq!(bound.rho_lower(), Some(Rational::new(15, 3)));
        assert_eq!(bound.rho_upper(), Some(Rational::new(21, 3)));
        assert!(bound.can_clear_bar(Rational::new(7, 1)));
    }

    #[test]
    fn zero_clause_kappa_never_clears_the_bar() {
        let bound = PrefixBound::singleton(10, 0, 80);

        assert_eq!(bound.rho_lower(), None);
        assert_eq!(bound.rho_upper(), None);
        assert!(!bound.can_clear_bar(Rational::new(1, 1)));
    }

    #[test]
    fn absorb_bound_merges_precomputed_extrema() {
        let mut bound = PrefixBound::singleton(18, 5, 88);
        bound.absorb_bound(PrefixBound {
            nu_lower_bound: 12,
            nu_upper_bound: 22,
            clause_kappa_used: 4,
            bit_kappa_used: 80,
        });

        assert_eq!(
            bound,
            PrefixBound {
                nu_lower_bound: 12,
                nu_upper_bound: 22,
                clause_kappa_used: 4,
                bit_kappa_used: 80,
            }
        );
    }
}
