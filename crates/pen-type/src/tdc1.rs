//! Minimal typed surface for the TDC-1 formed-path experiment.
//!
//! This module deliberately separates two facts which the legacy evaluator
//! conflates:
//!
//! 1. the shallow telescope `[App(Univ, Lib(window)), PathCon(d)]` elaborates;
//! 2. the `1 + d^2` L1 index set has typed cubical realizers.
//!
//! The first fact is checked by the existing kernel.  The second is not
//! silently inferred from a `PathCon` atom: kernel v1 has no interval,
//! `coe`, `hcom`, motive, or constructor-computation syntax.  TDC-1 can
//! therefore enumerate and canonically index the requested beta/Kan sites,
//! but a site remains an explicit proof obligation until a later cubical
//! extension supplies a replayable realization token.

use crate::elaborate::{
    ClauseFailure, KernelTy, SealedSignature, TelescopeElaboration, elaborate_telescope,
};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

pub const TDC1_FRAGMENT_VERSION: &str = "tdc1-formed-path-fragment-v1";

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PathSchemaKey {
    Beta,
    Kan { principal: u32, probe: u32 },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PathSchemaJudgement {
    ConstructorComputation,
    FiberAutomorphism,
    AutomorphismNaturality,
}

/// Canonical *index presentation* of one L1 site.  This is not represented as
/// an `Expr`: doing so would pretend the shallow AST contains cubical terms it
/// does not contain.  `owner_normal_form` binds the site to the typed
/// formation, and ordered direction indices preserve the transpose
/// distinction required by the frozen L1 statement.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PathSchemaNormalForm {
    pub owner_normal_form: Expr,
    pub path_dimension: u32,
    pub key: PathSchemaKey,
    pub judgement: PathSchemaJudgement,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PathSchemaObligation {
    pub normal_form: PathSchemaNormalForm,
    pub normal_form_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FormedPathTyping {
    pub subject_hash: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub kappa: u16,
    pub formation_clause: u16,
    pub formation_normal_form: Expr,
    pub path_clause: u16,
    pub dimension: u32,
    pub elaboration_derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum FormedPathError {
    #[error("surface elaboration failed: {0}")]
    Elaboration(ClauseFailure),
    #[error("formed-path surface must contain exactly one path declaration")]
    PathDeclarationArity,
    #[error("formed-path surface has no coarse-free typed formation")]
    NoTypedFormation,
    #[error("TDC-1 requires exactly two clauses")]
    NotTwoClausePackage,
    #[error("TDC-1 formation is not App(Univ, Lib({expected_window}))")]
    NotWindowFormation { expected_window: u32 },
    #[error("TDC-1 requires PathCon(4), found PathCon({found})")]
    NotDimensionFour { found: u32 },
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CubicalPrimitive {
    ConstructorEliminatorComputation,
    Coe,
    HcomNaturality,
}

#[derive(Clone, Debug, Deserialize, Error, Eq, PartialEq, Serialize)]
#[error("kernel v1 cannot realize {key:?}: missing primitive {missing:?}")]
pub struct PathSchemaRealizationGap {
    pub key: PathSchemaKey,
    pub missing: CubicalPrimitive,
    pub required_scope_increment: String,
}

fn obligation_digest(normal_form: &PathSchemaNormalForm) -> String {
    let payload = (TDC1_FRAGMENT_VERSION, normal_form);
    format!(
        "blake3:{}",
        blake3_hex(&serde_json::to_vec(&payload).expect("TDC-1 normal form serializes"))
    )
}

/// Elaborate any single-path package and retain the first coarse-free typed
/// formation.  Historical regression uses this broader entry point.
pub fn elaborate_formed_path(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
) -> Result<(FormedPathTyping, TelescopeElaboration), FormedPathError> {
    let elaboration = elaborate_telescope(signature, telescope, visible_library)
        .map_err(FormedPathError::Elaboration)?;
    let paths: Vec<(u16, u32)> = telescope
        .clauses
        .iter()
        .enumerate()
        .filter_map(|(index, clause)| match clause.expr {
            Expr::PathCon(dimension) => Some((index as u16, dimension)),
            _ => None,
        })
        .collect();
    let [(path_clause, dimension)] = paths.as_slice() else {
        return Err(FormedPathError::PathDeclarationArity);
    };
    let formation = elaboration
        .clauses
        .iter()
        .find(|clause| {
            clause.kernel_role == ClauseRole::Formation
                && clause.kernel_ty == KernelTy::Type
                && clause.coarse_assumptions == 0
        })
        .ok_or(FormedPathError::NoTypedFormation)?;
    let typing = FormedPathTyping {
        subject_hash: elaboration.subject_hash.clone(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        kappa: telescope.kappa() as u16,
        formation_clause: formation.clause_index,
        formation_normal_form: formation.normal_form.clone(),
        path_clause: *path_clause,
        dimension: *dimension,
        elaboration_derivation_hash: elaboration.derivation_hash.clone(),
    };
    Ok((typing, elaboration))
}

/// Strict TDC-1 surface check: the exact registered two-clause d=4 package
/// formed over the newest visible library entry.
pub fn elaborate_tdc1_package(
    signature: &SealedSignature,
    telescope: &Telescope,
    visible_library: u32,
) -> Result<(FormedPathTyping, TelescopeElaboration), FormedPathError> {
    if telescope.kappa() != 2 {
        return Err(FormedPathError::NotTwoClausePackage);
    }
    let expected = Expr::App(Box::new(Expr::Univ), Box::new(Expr::Lib(visible_library)));
    if telescope.clauses.first().map(|clause| &clause.expr) != Some(&expected) {
        return Err(FormedPathError::NotWindowFormation {
            expected_window: visible_library,
        });
    }
    let (typing, elaboration) = elaborate_formed_path(signature, telescope, visible_library)?;
    if typing.dimension != 4 {
        return Err(FormedPathError::NotDimensionFour {
            found: typing.dimension,
        });
    }
    Ok((typing, elaboration))
}

/// Enumerate beta plus the ordered d-by-d Kan matrix and produce stable
/// canonical index presentations.  Exact cardinality and uniqueness are
/// checked here; the semantic schema/basis isomorphism is intentionally not
/// claimed.
pub fn enumerate_path_schema_obligations(typing: &FormedPathTyping) -> Vec<PathSchemaObligation> {
    let mut keys = Vec::with_capacity(1 + (typing.dimension * typing.dimension) as usize);
    keys.push(PathSchemaKey::Beta);
    for principal in 0..typing.dimension {
        for probe in 0..typing.dimension {
            keys.push(PathSchemaKey::Kan { principal, probe });
        }
    }
    let unique: BTreeSet<_> = keys.iter().cloned().collect();
    assert_eq!(unique.len(), keys.len(), "generated L1 indices are unique");
    assert_eq!(
        keys.len(),
        1 + (typing.dimension * typing.dimension) as usize,
        "generated L1 cardinality is exact"
    );
    keys.into_iter()
        .map(|key| {
            let judgement = match key {
                PathSchemaKey::Beta => PathSchemaJudgement::ConstructorComputation,
                PathSchemaKey::Kan { principal, probe } if principal == probe => {
                    PathSchemaJudgement::FiberAutomorphism
                }
                PathSchemaKey::Kan { .. } => PathSchemaJudgement::AutomorphismNaturality,
            };
            let normal_form = PathSchemaNormalForm {
                owner_normal_form: typing.formation_normal_form.clone(),
                path_dimension: typing.dimension,
                key,
                judgement,
            };
            PathSchemaObligation {
                normal_form_digest: obligation_digest(&normal_form),
                normal_form,
            }
        })
        .collect()
}

/// The current fragment has no constructor that could carry the requested
/// cubical judgement.  Return the exact missing primitive per site instead
/// of manufacturing a token from the finite index.
pub fn realize_path_schema(
    obligation: &PathSchemaObligation,
) -> Result<(), PathSchemaRealizationGap> {
    let missing = match obligation.normal_form.judgement {
        PathSchemaJudgement::ConstructorComputation => {
            CubicalPrimitive::ConstructorEliminatorComputation
        }
        PathSchemaJudgement::FiberAutomorphism => CubicalPrimitive::Coe,
        PathSchemaJudgement::AutomorphismNaturality => CubicalPrimitive::HcomNaturality,
    };
    Err(PathSchemaRealizationGap {
        key: obligation.normal_form.key.clone(),
        missing,
        required_scope_increment: "extend the typed kernel with interval/cofibration contexts, coe and hcom typing/computation, a motive-typed constructor eliminator, and a replayable schema/basis realization token".to_owned(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::ClauseRec;

    fn candidate(dimension: u32) -> Telescope {
        Telescope::new(vec![
            ClauseRec::new(
                ClauseRole::Formation,
                Expr::App(Box::new(Expr::Univ), Box::new(Expr::Lib(15))),
            ),
            ClauseRec::new(ClauseRole::PathAttach, Expr::PathCon(dimension)),
        ])
    }

    #[test]
    fn registered_surface_elaborates_without_claiming_cubical_realizers() {
        let signature = SealedSignature::genesis_del_h15();
        let (typing, _) = elaborate_tdc1_package(&signature, &candidate(4), 15)
            .expect("registered surface elaborates");
        let obligations = enumerate_path_schema_obligations(&typing);
        assert_eq!(obligations.len(), 17);
        assert_eq!(
            obligations
                .iter()
                .map(|site| &site.normal_form_digest)
                .collect::<BTreeSet<_>>()
                .len(),
            17
        );
        assert!(
            obligations
                .iter()
                .all(|site| realize_path_schema(site).is_err())
        );
    }

    #[test]
    fn strict_surface_rejects_dimension_drift() {
        let signature = SealedSignature::genesis_del_h15();
        assert_eq!(
            elaborate_tdc1_package(&signature, &candidate(3), 15).unwrap_err(),
            FormedPathError::NotDimensionFour { found: 3 }
        );
    }
}
