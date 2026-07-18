//! SH-1 T3: the provisional, declared capacity oracle.
//!
//! This module is intentionally separate from the ordinary admissibility
//! implementation.  Oracle v0 is an experimental gate: its table entries are
//! declarations copied from SH-1c, not computations performed by the typed
//! kernel.  A positive result is therefore only *provisional admissibility*.
//! The trust-boundary record below is part of every gate decision so that an
//! artifact cannot silently present the table as derived evidence.

use pen_core::expr::Expr;
use pen_core::telescope::Telescope;
use serde::Serialize;
use std::collections::BTreeSet;

pub const SH1_CAPACITY_ORACLE_V0_ID: &str = "sh1.capacity-oracle.declared.v0";

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CapacityMultiplicity {
    InfiniteCyclic,
    Finite(u32),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DeclaredCapacityExport {
    pub export_id: &'static str,
    /// A `PathCon(d)` asks for a nonzero export in pi_(d-1).
    pub path_dimension: u32,
    pub homotopy_degree: u32,
    pub sealed_stratum: &'static str,
    pub homotopy_group: &'static str,
    pub multiplicity: CapacityMultiplicity,
    pub asserted_nontrivial: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CapacityOracleTrustBoundary {
    pub oracle_id: &'static str,
    pub epistemic_status: &'static str,
    pub declaration_source: &'static str,
    pub bar_used_to_construct_table: bool,
    pub verifies_dimension_level_nonzero_export_only: bool,
    pub verifies_candidate_host_binding: bool,
    pub verifies_homotopy_computation: bool,
    pub typed_kernel_required_to_earn_table: bool,
    pub warning: &'static str,
}

impl CapacityOracleTrustBoundary {
    pub const fn v0() -> Self {
        Self {
            oracle_id: SH1_CAPACITY_ORACLE_V0_ID,
            epistemic_status: "declared_not_derived",
            declaration_source: "docs/staircase_hypothesis.md#SH-1c",
            bar_used_to_construct_table: false,
            verifies_dimension_level_nonzero_export_only: true,
            verifies_candidate_host_binding: false,
            verifies_homotopy_computation: false,
            typed_kernel_required_to_earn_table: true,
            warning: "v0 is a declared experimental oracle; a typed kernel must derive both the exports and the candidate-to-class attachment before capacity is earned",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DeclaredCapacityOracleV0 {
    pub trust_boundary: CapacityOracleTrustBoundary,
    pub exports: Vec<DeclaredCapacityExport>,
}

impl Default for DeclaredCapacityOracleV0 {
    fn default() -> Self {
        Self::declared()
    }
}

impl DeclaredCapacityOracleV0 {
    /// Constructs the table declared by SH-1c.  This function performs no
    /// topological computation.
    pub fn declared() -> Self {
        Self {
            trust_boundary: CapacityOracleTrustBoundary::v0(),
            exports: vec![
                DeclaredCapacityExport {
                    export_id: "sh1c.pi3.s2.z",
                    path_dimension: 4,
                    homotopy_degree: 3,
                    sealed_stratum: "S2",
                    homotopy_group: "Z",
                    multiplicity: CapacityMultiplicity::InfiniteCyclic,
                    asserted_nontrivial: true,
                },
                DeclaredCapacityExport {
                    export_id: "sh1c.pi3.s3.z",
                    path_dimension: 4,
                    homotopy_degree: 3,
                    sealed_stratum: "S3",
                    homotopy_group: "Z",
                    multiplicity: CapacityMultiplicity::InfiniteCyclic,
                    asserted_nontrivial: true,
                },
                DeclaredCapacityExport {
                    export_id: "sh1c.pi4.s3.z_mod_2",
                    path_dimension: 5,
                    homotopy_degree: 4,
                    sealed_stratum: "S3",
                    homotopy_group: "Z/2",
                    multiplicity: CapacityMultiplicity::Finite(2),
                    asserted_nontrivial: true,
                },
                DeclaredCapacityExport {
                    export_id: "sh1c.pi5.s3.z_mod_2",
                    path_dimension: 6,
                    homotopy_degree: 5,
                    sealed_stratum: "S3",
                    homotopy_group: "Z/2",
                    multiplicity: CapacityMultiplicity::Finite(2),
                    asserted_nontrivial: true,
                },
                DeclaredCapacityExport {
                    export_id: "sh1c.pi6.s3.z_mod_12",
                    path_dimension: 7,
                    homotopy_degree: 6,
                    sealed_stratum: "S3",
                    homotopy_group: "Z/12",
                    multiplicity: CapacityMultiplicity::Finite(12),
                    asserted_nontrivial: true,
                },
            ],
        }
    }

    pub fn exports_for_dimension(&self, dimension: u32) -> Vec<&DeclaredCapacityExport> {
        self.exports
            .iter()
            .filter(|export| {
                export.path_dimension == dimension
                    && export.homotopy_degree.checked_add(1) == Some(dimension)
                    && export.asserted_nontrivial
            })
            .collect()
    }

    pub fn licenses_dimension(&self, dimension: u32) -> bool {
        !self.exports_for_dimension(dimension).is_empty()
    }

    pub fn licensed_dimensions(&self) -> Vec<u32> {
        self.exports
            .iter()
            .filter(|export| export.asserted_nontrivial)
            .map(|export| export.path_dimension)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PathConCapacityGateClass {
    NotApplicable,
    ProvisionallyAdmittedByDeclaredCapacity,
    RejectedNoDeclaredCapacity,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PathConCapacityGateDecision {
    pub oracle_id: &'static str,
    pub class: PathConCapacityGateClass,
    pub admitted: bool,
    pub path_dimensions: Vec<u32>,
    pub licensed_dimensions: Vec<u32>,
    pub rejected_dimensions: Vec<u32>,
    pub matched_export_ids: Vec<&'static str>,
    /// Always false in v0: `Expr::PathCon(d)` contains no host/class witness.
    pub candidate_host_binding_verified: bool,
    pub trust_boundary: CapacityOracleTrustBoundary,
}

impl PathConCapacityGateDecision {
    pub const fn is_admitted(&self) -> bool {
        self.admitted
    }
}

/// Experimental SH-1 pre-admissibility gate for every `PathCon(d)` occurring
/// in a candidate.  Ordinary non-path candidates pass as `NotApplicable`.
/// A mixed candidate is rejected if even one path dimension lacks a declared
/// nonzero export.
pub fn assess_pathcon_capacity_admissibility(
    oracle: &DeclaredCapacityOracleV0,
    telescope: &Telescope,
) -> PathConCapacityGateDecision {
    let mut dimensions = BTreeSet::new();
    for clause in &telescope.clauses {
        collect_path_dimensions(&clause.expr, &mut dimensions);
    }
    let path_dimensions: Vec<_> = dimensions.into_iter().collect();

    let rejected_dimensions: Vec<_> = path_dimensions
        .iter()
        .copied()
        .filter(|dimension| !oracle.licenses_dimension(*dimension))
        .collect();
    let matched_export_ids = path_dimensions
        .iter()
        .flat_map(|dimension| oracle.exports_for_dimension(*dimension))
        .map(|export| export.export_id)
        .collect();

    let (class, admitted) = if path_dimensions.is_empty() {
        (PathConCapacityGateClass::NotApplicable, true)
    } else if rejected_dimensions.is_empty() {
        (
            PathConCapacityGateClass::ProvisionallyAdmittedByDeclaredCapacity,
            true,
        )
    } else {
        (PathConCapacityGateClass::RejectedNoDeclaredCapacity, false)
    };

    PathConCapacityGateDecision {
        oracle_id: SH1_CAPACITY_ORACLE_V0_ID,
        class,
        admitted,
        path_dimensions,
        licensed_dimensions: oracle.licensed_dimensions(),
        rejected_dimensions,
        matched_export_ids,
        candidate_host_binding_verified: false,
        trust_boundary: oracle.trust_boundary.clone(),
    }
}

pub fn passes_pathcon_capacity_admissibility(
    oracle: &DeclaredCapacityOracleV0,
    telescope: &Telescope,
) -> bool {
    assess_pathcon_capacity_admissibility(oracle, telescope).is_admitted()
}

fn collect_path_dimensions(expr: &Expr, dimensions: &mut BTreeSet<u32>) {
    match expr {
        Expr::PathCon(dimension) => {
            dimensions.insert(*dimension);
        }
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            collect_path_dimensions(left, dimensions);
            collect_path_dimensions(right, dimensions);
        }
        Expr::Id(ty, left, right) => {
            collect_path_dimensions(ty, dimensions);
            collect_path_dimensions(left, dimensions);
            collect_path_dimensions(right, dimensions);
        }
        Expr::Lam(body)
        | Expr::Refl(body)
        | Expr::Susp(body)
        | Expr::Trunc(body)
        | Expr::Flat(body)
        | Expr::Sharp(body)
        | Expr::Disc(body)
        | Expr::Shape(body)
        | Expr::Next(body)
        | Expr::Eventually(body)
        | Expr::Bang(body)
        | Expr::WhyNot(body) => collect_path_dimensions(body, dimensions),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) => {}
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CapacityMultiplicity, DeclaredCapacityOracleV0, PathConCapacityGateClass,
        assess_pathcon_capacity_admissibility, passes_pathcon_capacity_admissibility,
    };
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::telescope::Telescope;

    fn path_candidate(dimension: u32) -> Telescope {
        Telescope::new(vec![ClauseRec::new(
            ClauseRole::PathAttach,
            Expr::PathCon(dimension),
        )])
    }

    #[test]
    fn declared_table_matches_sh1c_and_discloses_its_trust_boundary() {
        let oracle = DeclaredCapacityOracleV0::declared();

        assert_eq!(oracle.licensed_dimensions(), vec![4, 5, 6, 7]);
        assert_eq!(oracle.exports_for_dimension(4).len(), 2);
        assert_eq!(
            oracle.exports_for_dimension(7)[0].multiplicity,
            CapacityMultiplicity::Finite(12)
        );
        assert_eq!(
            oracle.trust_boundary.epistemic_status,
            "declared_not_derived"
        );
        assert!(!oracle.trust_boundary.bar_used_to_construct_table);
        assert!(!oracle.trust_boundary.verifies_candidate_host_binding);
        assert!(!oracle.trust_boundary.verifies_homotopy_computation);
        assert!(oracle.trust_boundary.typed_kernel_required_to_earn_table);
    }

    #[test]
    fn pathcon_gate_admits_only_declared_dimensions() {
        let oracle = DeclaredCapacityOracleV0::declared();

        for dimension in 4..=7 {
            let decision =
                assess_pathcon_capacity_admissibility(&oracle, &path_candidate(dimension));
            assert_eq!(
                decision.class,
                PathConCapacityGateClass::ProvisionallyAdmittedByDeclaredCapacity
            );
            assert!(decision.is_admitted());
            assert!(!decision.candidate_host_binding_verified);
        }

        for dimension in [1, 2, 3, 8] {
            let decision =
                assess_pathcon_capacity_admissibility(&oracle, &path_candidate(dimension));
            assert_eq!(
                decision.class,
                PathConCapacityGateClass::RejectedNoDeclaredCapacity
            );
            assert_eq!(decision.rejected_dimensions, vec![dimension]);
            assert!(!decision.is_admitted());
        }
    }

    #[test]
    fn nested_and_mixed_path_dimensions_cannot_bypass_the_gate() {
        let oracle = DeclaredCapacityOracleV0::declared();
        let nested_mixed = Telescope::new(vec![ClauseRec::new(
            ClauseRole::Elimination,
            Expr::App(Box::new(Expr::PathCon(4)), Box::new(Expr::PathCon(8))),
        )]);
        let decision = assess_pathcon_capacity_admissibility(&oracle, &nested_mixed);

        assert_eq!(decision.path_dimensions, vec![4, 8]);
        assert_eq!(decision.rejected_dimensions, vec![8]);
        assert!(!decision.is_admitted());
    }

    #[test]
    fn ordinary_non_path_candidates_are_outside_the_gate() {
        let oracle = DeclaredCapacityOracleV0::declared();
        let telescope = Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Univ)]);
        let decision = assess_pathcon_capacity_admissibility(&oracle, &telescope);

        assert_eq!(decision.class, PathConCapacityGateClass::NotApplicable);
        assert!(passes_pathcon_capacity_admissibility(&oracle, &telescope));
    }
}
