//! Wrapped classifier successor using motive-parametric coherence.

use crate::internal_classifier_branch_v8::{
    AmbientWrappedCandidate, AmbientWrapperError, WRAPPED_CONTEXTUAL_COHERENCE_GAP,
    WrappedCandidateDecisionV9, WrappedClassificationEvidence, classify_wrapped_candidate_v9,
};
use pen_core::hash::blake3_hex;
use pen_type::contextual_internality::issue_ambient_context_declaration_token;
use pen_type::elaborate::SealedSignature;
use pen_type::motive_parametric_coherence::{
    ParametricContextualInternalityProjection, issue_parametric_contextual_internality_token,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const WRAPPED_PARAMETRIC_CLASSIFIER_VERSION: &str = "ambient-wrapped-parametric-classifier-v10";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParametricWrappedInternalCertificate {
    pub candidate: AmbientWrappedCandidate,
    pub predecessor_decision: WrappedCandidateDecisionV9,
    pub contextual_clause_evidence: Vec<ParametricContextualInternalityProjection>,
    pub nonlive_clause_indices: Vec<u16>,
    pub every_live_contextual_clause_covered: bool,
    pub generic_theorem_replayed_per_live_clause: bool,
    pub probes_are_regression_only: bool,
    pub motive_grammar_restricted: bool,
    pub no_credit_anchor_or_orbit_minted: bool,
    pub internal_certificate_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WrappedCandidateDecisionV10 {
    NamedExclusion {
        evidence: WrappedClassificationEvidence,
    },
    NamedTypedExclusion {
        code: String,
        reason: String,
        evidence: WrappedClassificationEvidence,
    },
    InternalPredecessor {
        predecessor_decision: WrappedCandidateDecisionV9,
        marginal_nu: u32,
        derivation_hash: String,
    },
    InternalParametricContextual {
        certificate: ParametricWrappedInternalCertificate,
        marginal_nu: u32,
        derivation_hash: String,
    },
    Classified {
        predecessor_decision: WrappedCandidateDecisionV9,
    },
    NamedTypedObstruction {
        code: String,
        reason: String,
        predecessor_decision: WrappedCandidateDecisionV9,
    },
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(WRAPPED_PARAMETRIC_CLASSIFIER_VERSION, domain, value))
        .expect("v10 classifier evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn issue_parametric_certificate(
    candidate: &AmbientWrappedCandidate,
    predecessor_decision: WrappedCandidateDecisionV9,
) -> Result<ParametricWrappedInternalCertificate, String> {
    let signature = SealedSignature::genesis_del_h15();
    let declaration = issue_ambient_context_declaration_token(
        &signature,
        &candidate.clauses,
        15,
        candidate.ambient.clone(),
    )
    .map_err(|error| error.to_string())?;
    let mut contextual_clause_evidence = Vec::new();
    let mut nonlive_clause_indices = Vec::new();
    let mut certified_prior = BTreeMap::new();
    for clause_index in
        0..u16::try_from(candidate.clauses.kappa()).map_err(|_| "kappa exceeds u16")?
    {
        match issue_parametric_contextual_internality_token(
            &signature,
            &candidate.clauses,
            15,
            clause_index,
            &declaration,
            &certified_prior,
        ) {
            Ok(token) => {
                certified_prior.insert(clause_index, token.projection().derivation_hash.clone());
                contextual_clause_evidence.push(token.projection().clone());
            }
            Err(error) if error.to_string().contains("no live ambient-hypothesis use") => {
                nonlive_clause_indices.push(clause_index);
                certified_prior.insert(
                    clause_index,
                    format!("inherited-nonlive-internal-clause-{clause_index}"),
                );
            }
            Err(error) => return Err(error.to_string()),
        }
    }
    let every_live_contextual_clause_covered = !contextual_clause_evidence.is_empty();
    let generic_theorem_replayed_per_live_clause = contextual_clause_evidence
        .iter()
        .all(|evidence| evidence.generic_theorem_replayed);
    let probes_are_regression_only = contextual_clause_evidence
        .iter()
        .all(|evidence| evidence.probes_demoted_to_regression);
    let motive_grammar_restricted = contextual_clause_evidence
        .iter()
        .any(|evidence| !evidence.no_motive_filter);
    let no_credit_anchor_or_orbit_minted = contextual_clause_evidence
        .iter()
        .all(|evidence| evidence.no_credit_anchor_or_orbit_minted);
    let marginal_nu = 0;
    let internal_certificate_issued = every_live_contextual_clause_covered
        && generic_theorem_replayed_per_live_clause
        && probes_are_regression_only
        && !motive_grammar_restricted
        && no_credit_anchor_or_orbit_minted;
    if !internal_certificate_issued {
        return Err("parametric contextual Internal obligations incomplete".to_owned());
    }
    let derivation_hash = tagged_hash(
        "parametric-wrapped-internal-certificate",
        &(
            candidate,
            &predecessor_decision,
            &contextual_clause_evidence,
            &nonlive_clause_indices,
            every_live_contextual_clause_covered,
            generic_theorem_replayed_per_live_clause,
            probes_are_regression_only,
            motive_grammar_restricted,
            no_credit_anchor_or_orbit_minted,
            internal_certificate_issued,
            marginal_nu,
        ),
    );
    Ok(ParametricWrappedInternalCertificate {
        candidate: candidate.clone(),
        predecessor_decision,
        contextual_clause_evidence,
        nonlive_clause_indices,
        every_live_contextual_clause_covered,
        generic_theorem_replayed_per_live_clause,
        probes_are_regression_only,
        motive_grammar_restricted,
        no_credit_anchor_or_orbit_minted,
        internal_certificate_issued,
        marginal_nu,
        derivation_hash,
    })
}

pub fn classify_wrapped_candidate_v10(
    candidate: &AmbientWrappedCandidate,
) -> Result<WrappedCandidateDecisionV10, AmbientWrapperError> {
    let predecessor = classify_wrapped_candidate_v9(candidate)?;
    Ok(match &predecessor {
        WrappedCandidateDecisionV9::NamedExclusion { evidence } => {
            WrappedCandidateDecisionV10::NamedExclusion {
                evidence: evidence.clone(),
            }
        }
        WrappedCandidateDecisionV9::NamedTypedExclusion {
            code,
            reason,
            evidence,
        } => WrappedCandidateDecisionV10::NamedTypedExclusion {
            code: code.clone(),
            reason: reason.clone(),
            evidence: evidence.clone(),
        },
        WrappedCandidateDecisionV9::Internal { marginal_nu, .. } => {
            WrappedCandidateDecisionV10::InternalPredecessor {
                predecessor_decision: predecessor.clone(),
                marginal_nu: *marginal_nu,
                derivation_hash: tagged_hash("v10-internal-predecessor", &predecessor),
            }
        }
        WrappedCandidateDecisionV9::Classified { .. } => WrappedCandidateDecisionV10::Classified {
            predecessor_decision: predecessor.clone(),
        },
        WrappedCandidateDecisionV9::NamedTypedObstruction { code, reason, .. }
            if code == WRAPPED_CONTEXTUAL_COHERENCE_GAP
                && reason.contains("no registered closed probes") =>
        {
            match issue_parametric_certificate(candidate, predecessor.clone()) {
                Ok(certificate) => {
                    let marginal_nu = certificate.marginal_nu;
                    let derivation_hash = tagged_hash(
                        "v10-parametric-contextual-internal",
                        &(&certificate, marginal_nu),
                    );
                    WrappedCandidateDecisionV10::InternalParametricContextual {
                        certificate,
                        marginal_nu,
                        derivation_hash,
                    }
                }
                Err(successor_failure) => WrappedCandidateDecisionV10::NamedTypedObstruction {
                    code: code.clone(),
                    reason: format!("{reason}; parametric successor failed: {successor_failure}"),
                    predecessor_decision: predecessor.clone(),
                },
            }
        }
        WrappedCandidateDecisionV9::NamedTypedObstruction { code, reason, .. } => {
            WrappedCandidateDecisionV10::NamedTypedObstruction {
                code: code.clone(),
                reason: reason.clone(),
                predecessor_decision: predecessor.clone(),
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal_classifier_branch_v7::declared_context_control_candidate;
    use crate::internal_classifier_branch_v8::wrap_candidate;
    use pen_type::contextual_internality::ContextualMotive;

    fn survivor() -> AmbientWrappedCandidate {
        wrap_candidate(
            declared_context_control_candidate(),
            vec![ContextualMotive::Function {
                domain: Box::new(ContextualMotive::Type),
                codomain: Box::new(ContextualMotive::Element(pen_core::expr::Expr::Univ)),
            }],
        )
        .expect("v9 survivor")
    }

    #[test]
    fn v9_survivor_becomes_internal_at_zero_credit() {
        let decision = classify_wrapped_candidate_v10(&survivor()).expect("classify");
        let WrappedCandidateDecisionV10::InternalParametricContextual {
            certificate,
            marginal_nu,
            ..
        } = decision
        else {
            panic!("v9 survivor must become parametric Internal")
        };
        assert!(certificate.internal_certificate_issued);
        assert!(certificate.generic_theorem_replayed_per_live_clause);
        assert!(!certificate.motive_grammar_restricted);
        assert_eq!(marginal_nu, 0);
    }

    #[test]
    fn motive_mismatch_remains_a_typed_exclusion() {
        let wrong = wrap_candidate(
            declared_context_control_candidate(),
            vec![ContextualMotive::Type],
        )
        .expect("formable mismatch");
        assert!(matches!(
            classify_wrapped_candidate_v10(&wrong).expect("classify"),
            WrappedCandidateDecisionV10::NamedTypedExclusion { .. }
        ));
    }
}
