//! Applicability audit for the P5 record formula.
//!
//! The Telescopic Elimination theorem does not apply to every telescope that
//! the structural classifier calls `Axiomatic`.  In particular, its imported
//! interfaces must have a unique member whose prior dependency closure
//! contains every other direct import.  The evaluator historically replaced
//! that premise with "take the referenced entry having maximum historical
//! nu", which is not equivalent when two imports are incomparable.
//!
//! This module checks the graph premise only.  It deliberately does not claim
//! to decide the theorem's other semantic premise: that the clauses form a
//! Minimal Complete API of constructively irreducible operations.

use pen_core::telescope::Telescope;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use thiserror::Error;

/// The direct-import graph of an accepted telescope history.
///
/// An edge `a -> b` means that accepted entry `a` directly imports entry `b`.
/// Reachability therefore points from a later interface to the earlier
/// interfaces whose derivation surface it subsumes.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ImportDag {
    direct_imports: BTreeMap<u32, BTreeSet<u32>>,
}

impl ImportDag {
    /// Reconstruct the graph from the accepted telescopes themselves.  This
    /// cannot be reconstructed from `LibraryEntry`, which stores only the
    /// number of references and intentionally erases their identities.
    pub fn from_history(history: &[Telescope]) -> Self {
        let direct_imports = history
            .iter()
            .enumerate()
            .map(|(offset, telescope)| {
                let step = u32::try_from(offset + 1).expect("history length fits u32");
                (step, telescope.lib_refs())
            })
            .collect();
        Self { direct_imports }
    }

    /// The fixed Genesis reference prefix, used only by the Genesis audit.
    pub fn genesis_prefix(last_step: u32) -> Self {
        let history = (1..=last_step)
            .map(Telescope::reference)
            .collect::<Vec<_>>();
        Self::from_history(&history)
    }

    pub fn direct_imports(&self, step: u32) -> Option<&BTreeSet<u32>> {
        self.direct_imports.get(&step)
    }

    /// Reflexive transitive dependency closure of one accepted entry.
    pub fn reachable_from(&self, source: u32) -> BTreeSet<u32> {
        let mut reachable = BTreeSet::new();
        let mut queue = VecDeque::from([source]);

        while let Some(current) = queue.pop_front() {
            if !reachable.insert(current) {
                continue;
            }
            if let Some(imports) = self.direct_imports.get(&current) {
                queue.extend(imports.iter().copied());
            }
        }

        reachable
    }

    pub fn reaches(&self, source: u32, target: u32) -> bool {
        self.reachable_from(source).contains(&target)
    }
}

/// The graph-theoretic premise of the P5 record theorem for one candidate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct P5ImportAudit {
    /// Distinct interfaces directly imported by the candidate.
    pub direct_imports: Vec<u32>,
    /// Direct imports whose prior dependency closure contains every direct
    /// import.  P5 requires this list to contain exactly one member.
    pub dominant_imports: Vec<u32>,
    pub unique_dominant_import: Option<u32>,
    pub unique_dominant_import_holds: bool,
}

impl P5ImportAudit {
    pub fn check(candidate: &Telescope, graph: &ImportDag) -> Self {
        let direct_imports = candidate.lib_refs().into_iter().collect::<Vec<_>>();
        let dominant_imports = direct_imports
            .iter()
            .copied()
            .filter(|source| {
                direct_imports
                    .iter()
                    .copied()
                    .all(|target| graph.reaches(*source, target))
            })
            .collect::<Vec<_>>();
        let unique_dominant_import = match dominant_imports.as_slice() {
            [only] => Some(*only),
            _ => None,
        };

        Self {
            direct_imports,
            dominant_imports,
            unique_dominant_import,
            unique_dominant_import_holds: unique_dominant_import.is_some(),
        }
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum P5RecordError {
    #[error(
        "P5 requires one reachability-dominant direct import; direct imports {direct_imports:?} have dominant set {dominant_imports:?}"
    )]
    NoUniqueDominantImport {
        direct_imports: Vec<u32>,
        dominant_imports: Vec<u32>,
    },
    #[error("P5 dominant import L{step} has no nu entry in the supplied history")]
    MissingNuHistory { step: u32 },
}

/// Evaluate the P5 record equation only after checking its unique-dominant-
/// import hypothesis.
///
/// Success is still conditional on the caller separately supplying the
/// theorem's semantic Minimal Complete API / constructive-irreducibility
/// evidence.  This function checks the graph premise; it does not manufacture
/// that evidence from the shallow MBTT AST.
pub fn conditional_p5_nu_c(
    candidate: &Telescope,
    graph: &ImportDag,
    nu_history: &[(u32, u32)],
) -> Result<u32, P5RecordError> {
    let audit = P5ImportAudit::check(candidate, graph);
    let Some(dominant) = audit.unique_dominant_import else {
        return Err(P5RecordError::NoUniqueDominantImport {
            direct_imports: audit.direct_imports,
            dominant_imports: audit.dominant_imports,
        });
    };
    let inherited = nu_history
        .iter()
        .find_map(|(step, nu)| (*step == dominant).then_some(*nu))
        .ok_or(P5RecordError::MissingNuHistory { step: dominant })?;
    let kappa = u32::try_from(candidate.kappa()).expect("candidate kappa fits u32");
    let bridge_count = u32::try_from(audit.direct_imports.len())
        .expect("reference count fits u32")
        .saturating_sub(1);

    Ok(inherited + kappa + bridge_count)
}

#[cfg(test)]
mod tests {
    use super::{ImportDag, P5ImportAudit, P5RecordError, conditional_p5_nu_c};
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::telescope::Telescope;

    fn survivor() -> Telescope {
        Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Pi(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::Sigma(Box::new(Expr::Var(1)), Box::new(Expr::Var(1))),
            ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Var(1))),
            ),
        ])
    }

    #[test]
    fn genesis_dependency_closures_make_fourteen_and_fifteen_incomparable() {
        let graph = ImportDag::genesis_prefix(15);

        assert_eq!(
            graph.reachable_from(14),
            [10, 11, 12, 13, 14].into_iter().collect()
        );
        assert_eq!(graph.reachable_from(15), [10, 15].into_iter().collect());
        assert!(!graph.reaches(14, 15));
        assert!(!graph.reaches(15, 14));
    }

    #[test]
    fn reference_p5_packages_have_the_required_dominant_import() {
        let graph = ImportDag::genesis_prefix(15);
        let step_13_telescope = Telescope::reference(13);
        let step_14_telescope = Telescope::reference(14);
        let step_13 = P5ImportAudit::check(&step_13_telescope, &graph);
        let step_14 = P5ImportAudit::check(&step_14_telescope, &graph);

        assert_eq!(step_13.direct_imports, vec![11, 12]);
        assert_eq!(step_13.unique_dominant_import, Some(12));
        assert_eq!(step_14.direct_imports, vec![11, 12, 13]);
        assert_eq!(step_14.unique_dominant_import, Some(13));
        assert_eq!(
            conditional_p5_nu_c(&step_13_telescope, &graph, &[(12, 34)]),
            Ok(42)
        );
        assert_eq!(
            conditional_p5_nu_c(&step_14_telescope, &graph, &[(13, 46)]),
            Ok(57)
        );
    }

    #[test]
    fn survivor_is_outside_the_p5_unique_maximum_domain() {
        let graph = ImportDag::genesis_prefix(15);
        let candidate = survivor();
        let audit = P5ImportAudit::check(&candidate, &graph);

        assert_eq!(audit.direct_imports, vec![14, 15]);
        assert!(audit.dominant_imports.is_empty());
        assert!(!audit.unique_dominant_import_holds);

        let error = conditional_p5_nu_c(&candidate, &graph, &[(14, 83), (15, 103)])
            .expect_err("the stated P5 theorem has no L_max for this candidate");
        assert_eq!(
            error,
            P5RecordError::NoUniqueDominantImport {
                direct_imports: vec![14, 15],
                dominant_imports: vec![],
            }
        );
    }
}
