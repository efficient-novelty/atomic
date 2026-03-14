use crate::scc::{ClauseIx, terminal_clause_sccs, terminal_scc_sub_bundles};
use crate::{bar::compute_rho, nu::compute_native_nu};
use pen_core::{library::Library, rational::Rational, telescope::Telescope};
use pen_type::{
    admissibility::{StrictAdmissibility, passes_strict_admissibility},
    check::{CheckResult, check_telescope},
};

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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SemanticMinimalityWitness {
    pub structural: MinimalityWitness,
    pub admissible_bar_clear_subbundles: Vec<Telescope>,
}

impl SemanticMinimalityWitness {
    pub fn is_minimal(&self) -> bool {
        self.admissible_bar_clear_subbundles.is_empty()
    }
}

pub fn analyze_semantic_minimality(
    step_index: u32,
    objective_bar: Rational,
    admissibility: StrictAdmissibility,
    telescope: &Telescope,
    library: &Library,
    history: &[(u32, u32)],
) -> SemanticMinimalityWitness {
    let structural = analyze_minimality(telescope);
    let admissible_bar_clear_subbundles = structural
        .detachable_subbundles
        .iter()
        .filter(|candidate| check_telescope(library, candidate) == CheckResult::Ok)
        .filter(|candidate| {
            passes_strict_admissibility(step_index, library, candidate, admissibility)
        })
        .filter(|candidate| {
            let native = compute_native_nu(candidate, library, history);
            let clause_kappa = u32::try_from(candidate.kappa()).expect("kappa should fit in u32");
            if clause_kappa == 0 {
                return false;
            }

            compute_rho(native.total, clause_kappa)
                .map(|rho| rho >= objective_bar)
                .unwrap_or(false)
        })
        .cloned()
        .collect();

    SemanticMinimalityWitness {
        structural,
        admissible_bar_clear_subbundles,
    }
}

#[cfg(test)]
mod tests {
    use super::{analyze_minimality, analyze_semantic_minimality, is_structurally_minimal};
    use pen_core::{
        library::{Library, LibraryEntry},
        rational::Rational,
        telescope::Telescope,
    };
    use pen_type::admissibility::strict_admissibility;

    #[test]
    fn pi_reference_telescope_is_not_minimal_under_terminal_amputation() {
        let witness = analyze_minimality(&Telescope::reference(4));
        assert!(!witness.is_minimal());
        assert_eq!(witness.terminal_components, vec![vec![0]]);
        assert!(!is_structurally_minimal(&Telescope::reference(4)));
    }

    #[test]
    fn pi_reference_telescope_is_semantically_minimal_for_step_four() {
        let mut library: Library = Vec::new();
        let mut history = Vec::new();
        for step in 1..=3 {
            let telescope = Telescope::reference(step);
            let result = crate::nu::structural_nu(&telescope, &library, &history);
            library.push(LibraryEntry::from_telescope(&telescope, &library));
            history.push((step, result.total));
        }

        let admissibility = strict_admissibility(4, 2, &library);
        let witness = analyze_semantic_minimality(
            4,
            Rational::new(3, 2),
            admissibility,
            &Telescope::reference(4),
            &library,
            &history,
        );
        assert_eq!(witness.structural.terminal_components, vec![vec![0]]);
        assert!(witness.is_minimal());
    }
}
