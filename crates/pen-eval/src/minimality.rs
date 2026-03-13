use crate::scc::{ClauseIx, terminal_clause_sccs, terminal_scc_sub_bundles};
use pen_core::telescope::Telescope;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MinimalityWitness {
    pub terminal_components: Vec<Vec<ClauseIx>>,
    pub detachable_subbundles: Vec<Telescope>,
}

impl MinimalityWitness {
    pub fn is_minimal(&self) -> bool {
        self.detachable_subbundles.is_empty()
    }
}

pub fn analyze_minimality(telescope: &Telescope) -> MinimalityWitness {
    MinimalityWitness {
        terminal_components: terminal_clause_sccs(telescope),
        detachable_subbundles: terminal_scc_sub_bundles(telescope),
    }
}

pub fn is_structurally_minimal(telescope: &Telescope) -> bool {
    analyze_minimality(telescope).is_minimal()
}

#[cfg(test)]
mod tests {
    use super::{analyze_minimality, is_structurally_minimal};
    use pen_core::telescope::Telescope;

    #[test]
    fn pi_reference_telescope_is_not_minimal_under_terminal_amputation() {
        let witness = analyze_minimality(&Telescope::reference(4));
        assert!(!witness.is_minimal());
        assert_eq!(witness.terminal_components, vec![vec![0]]);
        assert!(!is_structurally_minimal(&Telescope::reference(4)));
    }
}
