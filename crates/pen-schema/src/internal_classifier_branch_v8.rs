//! Classifier over the versioned ambient-telescope candidate wrapper.
//!
//! The wrapper is judgment data: motives are supplied before classification,
//! hashed with the frozen clause payload, and consumed without inference or
//! repair.  A declaration/use mismatch is a named typed exclusion.  A
//! well-typed declaration whose contextual specialization has not been
//! proved remains a named typed obstruction under F-A5.

use crate::internal_classifier_branch_v7::{
    RawCandidateDecisionV8, classify_raw_candidate_v8, classify_raw_candidate_v8_with_declaration,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::contextual_internality::{
    AmbientContextDeclarationProjection, ContextualInternalityError, ContextualMotive,
    issue_ambient_context_declaration_token, issue_contextual_internality_token,
};
use pen_type::elaborate::{SealedSignature, elaborate_telescope};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

pub const AMBIENT_WRAPPER_VERSION: &str = "ambient-telescope-candidate-wrapper-v1";
pub const AMBIENT_MOTIVE_NODE_CAP: u32 = 6;
pub const WRAPPED_MOTIVE_TYPE_MISMATCH: &str = "WRAPPED_AMBIENT_MOTIVE_USE_TYPE_MISMATCH";
pub const WRAPPED_CONTEXTUAL_COHERENCE_GAP: &str =
    "WRAPPED_CONTEXTUAL_INSTANTIATION_COHERENCE_NOT_PROVED";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientWrappedCandidate {
    pub version: String,
    pub clauses: Telescope,
    pub ambient: Vec<ContextualMotive>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WrappedClassificationEvidence {
    pub wrapper_hash: String,
    pub ambient_arity: u32,
    pub motive_node_counts: Vec<u32>,
    pub declaration: AmbientContextDeclarationProjection,
    pub declaration_supplied_before_classification: bool,
    pub motive_inference_performed: bool,
    pub motive_repair_performed: bool,
    pub predecessor_decision: RawCandidateDecisionV8,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WrappedCandidateDecisionV9 {
    NamedExclusion {
        evidence: WrappedClassificationEvidence,
    },
    NamedTypedExclusion {
        code: String,
        reason: String,
        evidence: WrappedClassificationEvidence,
    },
    Internal {
        evidence: WrappedClassificationEvidence,
        contextual_decision: RawCandidateDecisionV8,
        marginal_nu: u32,
        derivation_hash: String,
    },
    Classified {
        evidence: WrappedClassificationEvidence,
        contextual_decision: RawCandidateDecisionV8,
    },
    NamedTypedObstruction {
        code: String,
        reason: String,
        evidence: WrappedClassificationEvidence,
        contextual_decision: RawCandidateDecisionV8,
    },
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum AmbientWrapperError {
    #[error("wrapper version mismatch")]
    VersionMismatch,
    #[error("wrapped clauses failed frozen elaboration: {0}")]
    Elaboration(String),
    #[error("wrapped ambient vector length {declared} differs from inferred arity {inferred}")]
    AmbientArity { declared: u32, inferred: u32 },
    #[error("ambient motive {parameter} exceeds the frozen six-node cap ({nodes})")]
    MotiveCap { parameter: u32, nodes: u32 },
    #[error("ambient motive declaration failed: {0}")]
    Declaration(String),
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(AMBIENT_WRAPPER_VERSION, domain, value))
        .expect("wrapped classifier evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

pub fn contextual_motive_node_count(motive: &ContextualMotive) -> u32 {
    match motive {
        ContextualMotive::Type | ContextualMotive::Neutral => 1,
        ContextualMotive::Element(expr) => 1 + expression_node_count(expr),
        ContextualMotive::Function { domain, codomain } => {
            1 + contextual_motive_node_count(domain) + contextual_motive_node_count(codomain)
        }
    }
}

pub fn expression_node_count(expression: &pen_core::expr::Expr) -> u32 {
    use pen_core::expr::Expr;
    match expression {
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => 1,
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => 1 + expression_node_count(inner),
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            1 + expression_node_count(left) + expression_node_count(right)
        }
        Expr::Id(ty, left, right) => {
            1 + expression_node_count(ty)
                + expression_node_count(left)
                + expression_node_count(right)
        }
    }
}

pub fn wrapped_candidate_hash(candidate: &AmbientWrappedCandidate) -> String {
    tagged_hash("ambient-wrapped-candidate", candidate)
}

pub fn validate_wrapped_candidate(
    candidate: &AmbientWrappedCandidate,
) -> Result<AmbientContextDeclarationProjection, AmbientWrapperError> {
    if candidate.version != AMBIENT_WRAPPER_VERSION {
        return Err(AmbientWrapperError::VersionMismatch);
    }
    let signature = SealedSignature::genesis_del_h15();
    let elaboration = elaborate_telescope(&signature, &candidate.clauses, 15)
        .map_err(|error| AmbientWrapperError::Elaboration(error.to_string()))?;
    let declared = candidate.ambient.len() as u32;
    if declared != elaboration.ambient_parameters {
        return Err(AmbientWrapperError::AmbientArity {
            declared,
            inferred: elaboration.ambient_parameters,
        });
    }
    for (index, motive) in candidate.ambient.iter().enumerate() {
        let nodes = contextual_motive_node_count(motive);
        if nodes > AMBIENT_MOTIVE_NODE_CAP {
            return Err(AmbientWrapperError::MotiveCap {
                parameter: index as u32 + 1,
                nodes,
            });
        }
    }
    issue_ambient_context_declaration_token(
        &signature,
        &candidate.clauses,
        15,
        candidate.ambient.clone(),
    )
    .map(|token| token.projection().clone())
    .map_err(|error| AmbientWrapperError::Declaration(error.to_string()))
}

pub fn wrap_candidate(
    clauses: Telescope,
    ambient: Vec<ContextualMotive>,
) -> Result<AmbientWrappedCandidate, AmbientWrapperError> {
    let candidate = AmbientWrappedCandidate {
        version: AMBIENT_WRAPPER_VERSION.to_owned(),
        clauses,
        ambient,
    };
    validate_wrapped_candidate(&candidate)?;
    Ok(candidate)
}

fn decision_is_internal(decision: &RawCandidateDecisionV8) -> Option<u32> {
    match decision {
        RawCandidateDecisionV8::InternalPredecessor { .. } => Some(0),
        RawCandidateDecisionV8::InternalGuardedClosure { marginal_nu, .. }
        | RawCandidateDecisionV8::InternalContextual { marginal_nu, .. } => Some(*marginal_nu),
        _ => None,
    }
}

fn contextual_failure(
    candidate: &AmbientWrappedCandidate,
    declaration: &AmbientContextDeclarationProjection,
) -> Option<ContextualInternalityError> {
    if candidate.ambient.is_empty() {
        return None;
    }
    let signature = SealedSignature::genesis_del_h15();
    let declaration_token = issue_ambient_context_declaration_token(
        &signature,
        &candidate.clauses,
        15,
        candidate.ambient.clone(),
    )
    .expect("validated declaration reissues");
    debug_assert_eq!(declaration_token.projection(), declaration);
    let mut certified = BTreeMap::new();
    for clause_index in 0..u16::try_from(candidate.clauses.kappa()).expect("kappa fits u16") {
        for prior in 0..clause_index {
            certified
                .entry(prior)
                .or_insert_with(|| format!("wrapped-prior-clause-{prior}"));
        }
        match issue_contextual_internality_token(
            &signature,
            &candidate.clauses,
            15,
            clause_index,
            &declaration_token,
            &certified,
        ) {
            Ok(token) => {
                certified.insert(clause_index, token.projection().derivation_hash.clone());
            }
            Err(ContextualInternalityError::NoLiveAmbientUse) => {}
            Err(error) => return Some(error),
        }
    }
    None
}

fn evidence(
    candidate: &AmbientWrappedCandidate,
    declaration: AmbientContextDeclarationProjection,
    predecessor_decision: RawCandidateDecisionV8,
) -> WrappedClassificationEvidence {
    let wrapper_hash = wrapped_candidate_hash(candidate);
    let ambient_arity = candidate.ambient.len() as u32;
    let motive_node_counts = candidate
        .ambient
        .iter()
        .map(contextual_motive_node_count)
        .collect::<Vec<_>>();
    let declaration_supplied_before_classification = true;
    let motive_inference_performed = false;
    let motive_repair_performed = false;
    let derivation_hash = tagged_hash(
        "wrapped-classification-evidence",
        &(
            &wrapper_hash,
            ambient_arity,
            &motive_node_counts,
            &declaration,
            declaration_supplied_before_classification,
            motive_inference_performed,
            motive_repair_performed,
            &predecessor_decision,
        ),
    );
    WrappedClassificationEvidence {
        wrapper_hash,
        ambient_arity,
        motive_node_counts,
        declaration,
        declaration_supplied_before_classification,
        motive_inference_performed,
        motive_repair_performed,
        predecessor_decision,
        derivation_hash,
    }
}

pub fn classify_wrapped_candidate_v9(
    candidate: &AmbientWrappedCandidate,
) -> Result<WrappedCandidateDecisionV9, AmbientWrapperError> {
    let declaration = validate_wrapped_candidate(candidate)?;
    let predecessor_decision = classify_raw_candidate_v8(&candidate.clauses);
    let contextual_decision = if candidate.ambient.is_empty() {
        predecessor_decision.clone()
    } else {
        let signature = SealedSignature::genesis_del_h15();
        let declaration_token = issue_ambient_context_declaration_token(
            &signature,
            &candidate.clauses,
            15,
            candidate.ambient.clone(),
        )
        .expect("validated declaration reissues");
        classify_raw_candidate_v8_with_declaration(&candidate.clauses, &declaration_token)
    };
    let evidence = evidence(candidate, declaration.clone(), predecessor_decision);

    match &contextual_decision {
        RawCandidateDecisionV8::NamedExclusion { .. } => {
            Ok(WrappedCandidateDecisionV9::NamedExclusion { evidence })
        }
        RawCandidateDecisionV8::Classified { .. } => Ok(WrappedCandidateDecisionV9::Classified {
            evidence,
            contextual_decision,
        }),
        decision if decision_is_internal(decision).is_some() => {
            let marginal_nu = decision_is_internal(decision).expect("matched Internal");
            let derivation_hash = tagged_hash(
                "wrapped-internal-classification",
                &(&evidence.derivation_hash, &contextual_decision, marginal_nu),
            );
            Ok(WrappedCandidateDecisionV9::Internal {
                evidence,
                contextual_decision,
                marginal_nu,
                derivation_hash,
            })
        }
        RawCandidateDecisionV8::NamedTypedObstruction { .. } => {
            match contextual_failure(candidate, &declaration) {
                Some(error @ ContextualInternalityError::IllTypedAmbientUse { .. })
                | Some(error @ ContextualInternalityError::UndeclaredAmbientParameter { .. }) => {
                    Ok(WrappedCandidateDecisionV9::NamedTypedExclusion {
                        code: WRAPPED_MOTIVE_TYPE_MISMATCH.to_owned(),
                        reason: error.to_string(),
                        evidence,
                    })
                }
                Some(error) => Ok(WrappedCandidateDecisionV9::NamedTypedObstruction {
                    code: WRAPPED_CONTEXTUAL_COHERENCE_GAP.to_owned(),
                    reason: error.to_string(),
                    evidence,
                    contextual_decision,
                }),
                None => Ok(WrappedCandidateDecisionV9::NamedTypedObstruction {
                    code: WRAPPED_CONTEXTUAL_COHERENCE_GAP.to_owned(),
                    reason: "well-typed wrapped candidate retained the predecessor obstruction"
                        .to_owned(),
                    evidence,
                    contextual_decision,
                }),
            }
        }
        RawCandidateDecisionV8::InternalPredecessor { .. }
        | RawCandidateDecisionV8::InternalGuardedClosure { .. }
        | RawCandidateDecisionV8::InternalContextual { .. } => unreachable!("handled above"),
    }
}

pub fn closed_fiber_agrees(
    candidate: &AmbientWrappedCandidate,
) -> Result<bool, AmbientWrapperError> {
    if !candidate.ambient.is_empty() {
        return Ok(false);
    }
    let frozen = classify_raw_candidate_v8(&candidate.clauses);
    let wrapped = classify_wrapped_candidate_v9(candidate)?;
    Ok(match (frozen, wrapped) {
        (
            RawCandidateDecisionV8::NamedExclusion { .. },
            WrappedCandidateDecisionV9::NamedExclusion { .. },
        )
        | (
            RawCandidateDecisionV8::Classified { .. },
            WrappedCandidateDecisionV9::Classified { .. },
        )
        | (
            RawCandidateDecisionV8::InternalPredecessor { .. },
            WrappedCandidateDecisionV9::Internal { .. },
        )
        | (
            RawCandidateDecisionV8::InternalGuardedClosure { .. },
            WrappedCandidateDecisionV9::Internal { .. },
        )
        | (
            RawCandidateDecisionV8::InternalContextual { .. },
            WrappedCandidateDecisionV9::Internal { .. },
        )
        | (
            RawCandidateDecisionV8::NamedTypedObstruction { .. },
            WrappedCandidateDecisionV9::NamedTypedObstruction { .. },
        ) => true,
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal_classifier_branch_v7::declared_context_control_candidate;
    use pen_core::expr::Expr;

    fn function(codomain: ContextualMotive) -> ContextualMotive {
        ContextualMotive::Function {
            domain: Box::new(ContextualMotive::Type),
            codomain: Box::new(codomain),
        }
    }

    #[test]
    fn declarations_are_consumed_without_inference_or_repair() {
        let clauses = declared_context_control_candidate();
        let good = wrap_candidate(clauses.clone(), vec![function(ContextualMotive::Type)])
            .expect("good wrapper");
        let decision = classify_wrapped_candidate_v9(&good).expect("classify");
        let WrappedCandidateDecisionV9::Internal {
            evidence,
            marginal_nu,
            ..
        } = decision
        else {
            panic!("good declaration must be Internal")
        };
        assert!(evidence.declaration_supplied_before_classification);
        assert!(!evidence.motive_inference_performed);
        assert!(!evidence.motive_repair_performed);
        assert_eq!(marginal_nu, 0);

        let wrong = wrap_candidate(clauses, vec![ContextualMotive::Type]).expect("wrong wrapper");
        assert!(matches!(
            classify_wrapped_candidate_v9(&wrong).expect("classify wrong"),
            WrappedCandidateDecisionV9::NamedTypedExclusion { ref code, .. }
                if code == WRAPPED_MOTIVE_TYPE_MISMATCH
        ));
    }

    #[test]
    fn well_typed_unprobed_motive_remains_named_unknown() {
        let candidate = wrap_candidate(
            declared_context_control_candidate(),
            vec![function(ContextualMotive::Element(Expr::Univ))],
        )
        .expect("wrapped candidate");
        let decision = classify_wrapped_candidate_v9(&candidate).expect("classify");
        assert!(matches!(
            decision,
            WrappedCandidateDecisionV9::NamedTypedObstruction { ref code, ref reason, .. }
                if code == WRAPPED_CONTEXTUAL_COHERENCE_GAP
                    && reason.contains("no registered closed probes")
        ));
    }

    #[test]
    fn closed_fiber_preserves_frozen_decision() {
        let closed = Telescope::new(vec![pen_core::clause::ClauseRec::new(
            pen_core::clause::ClauseRole::Formation,
            Expr::Univ,
        )]);
        let wrapped = wrap_candidate(closed, Vec::new()).expect("closed wrapper");
        assert!(closed_fiber_agrees(&wrapped).expect("agreement"));
    }
}
