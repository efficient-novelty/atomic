//! Proof-strength successor for the one wrapped surface left open by E-4 v9.
//!
//! The v9 probe classifier is retained only as an archival predecessor.  This
//! module does not accept its fabricated `String` prior certificates.  It
//! replays the exact declaration, audits each clause for live ambient use,
//! and gives every live clause a candidate-specific
//! [`VerifiedClosureDerivationV2`].  Clauses with no live ambient use are
//! recorded outside this successor's support; in particular, no certificate
//! for the unused `Formation: Univ` clause is fabricated merely to populate a
//! prior map.  This theorem-program version deliberately withholds the
//! candidate-level `Internal` verdict while motive-v2 has open F-M1 cases.

use crate::internal_classifier_branch::{
    ClauseDerivabilityKind, InternalDerivabilityAttempt, issue_internal_derivability_attempt,
};
use crate::internal_classifier_branch_v7::declared_context_control_candidate;
use crate::internal_classifier_branch_v8::{
    AmbientWrappedCandidate, AmbientWrapperError, validate_wrapped_candidate,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::contextual_internality::{
    AmbientContextDeclarationProjection, ContextualInternalityError,
    ContextualTypedStructureProjection, issue_ambient_context_declaration_token,
    issue_contextual_typed_structure_token,
};
use pen_type::elaborate::SealedSignature;
use pen_type::motive_parametric_coherence::ClosureRuleKind;
use pen_type::motive_parametric_coherence_v2::{
    CLOSURE_RULE_INVENTORY_V2, ClosureRuleEvidenceV2, MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION,
    VerifiedClosureDerivationV2, issue_contextual_closure_derivation_v2,
    replay_verified_closure_derivation_v2,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub const WRAPPED_PROOF_STRENGTH_CLASSIFIER_VERSION: &str =
    "ambient-wrapped-proof-strength-classifier-v11";
pub const WRAPPED_V11_TYPED_EXCLUSION: &str = "WRAPPED_V11_EXACT_TYPED_EXCLUSION";
pub const WRAPPED_V11_EXACT_EVIDENCE_GAP: &str = "WRAPPED_V11_EXACT_CONTEXTUAL_EVIDENCE_NOT_PROVED";
pub const WRAPPED_V11_OUTSIDE_SUCCESSOR_SURFACE: &str =
    "WRAPPED_V11_OUTSIDE_FROZEN_V9_SURVIVOR_SURFACE";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EliminatorProofStatusV11 {
    TheoremProgramBlockedByIssuableSourceCounterexamples,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EmptyAssignmentFiberObservationV11 {
    VacuityDoesNotEstablishNonemptyFibers,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProbeDispositionV11 {
    RegressionOnlyNeverEvidence,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextualAuditDispositionV11 {
    NoLiveAmbientUse,
    IllTypedAmbientUse { parameter: u32 },
    UndeclaredAmbientParameter { parameter: u32 },
}

/// The exact boundary of the current eliminator theorem program.  The six
/// branches exist and exact specializations can replay, but the public issuer
/// domain still contains known F-M1 counterexamples.  Consequently this
/// record is evidence *against* promoting the program to a universal theorem.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EliminatorTheoremProgramV11 {
    pub eliminator_version: String,
    pub eliminator_function: String,
    pub replay_function: String,
    pub closure_rule_inventory: Vec<ClosureRuleKind>,
    pub source_rule: ClosureRuleKind,
    pub proof_status: EliminatorProofStatusV11,
    pub empty_assignment_fiber: EmptyAssignmentFiberObservationV11,
    pub probe_disposition: ProbeDispositionV11,
    pub motive_filter: Option<String>,
    pub known_counterexample_classes: Vec<String>,
    pub universal_specialization_claim_authorized: bool,
    pub contract_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedNonliveClauseV11 {
    pub clause_index: u16,
    pub exact_prefix: Telescope,
    pub contextual_audit_disposition: ContextualAuditDispositionV11,
    pub contextual_audit_error: String,
    pub outside_live_contextual_support: bool,
    pub predecessor_internal_evidence: InternalDerivabilityAttempt,
    pub predecessor_evidence_reissued: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactContextualClauseV11 {
    pub clause_index: u16,
    pub typed_structure: ContextualTypedStructureProjection,
    pub exact_closure_derivation: VerifiedClosureDerivationV2,
    pub exact_used_prior_clause_indices: Vec<u16>,
    pub source_replayed: bool,
    pub eliminator_theorem_program: EliminatorTheoremProgramV11,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProofStrengthWrappedLiveSourceCertificateV11 {
    pub version: String,
    pub candidate: AmbientWrappedCandidate,
    pub declaration: AmbientContextDeclarationProjection,
    pub certified_nonlive_clauses: Vec<CertifiedNonliveClauseV11>,
    pub exact_contextual_clauses: Vec<ExactContextualClauseV11>,
    pub clause_partition: Vec<u16>,
    pub exact_live_support: BTreeMap<u16, Vec<u16>>,
    pub no_bare_prior_hash_accepted: bool,
    pub live_contextual_clause_coverage_complete: bool,
    pub at_least_one_live_contextual_clause: bool,
    pub every_clause_has_exact_source_evidence: bool,
    pub candidate_level_internality_authorized: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WrappedCandidateDecisionV11 {
    OutsideFrozenSuccessorSurface {
        code: String,
        candidate: AmbientWrappedCandidate,
    },
    NamedTypedExclusion {
        code: String,
        clause_index: u16,
        disposition: ContextualAuditDispositionV11,
        error: String,
        declaration: AmbientContextDeclarationProjection,
    },
    ExactLiveContextualSourceEstablished {
        certificate: ProofStrengthWrappedLiveSourceCertificateV11,
        marginal_nu: u32,
        derivation_hash: String,
    },
    NamedTypedObstruction {
        code: String,
        clause_index: u16,
        reason: String,
        declaration: AmbientContextDeclarationProjection,
    },
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(WRAPPED_PROOF_STRENGTH_CLASSIFIER_VERSION, domain, value))
        .expect("proof-strength wrapped classifier evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn exact_prefix(candidate: &Telescope, clause_index: u16) -> Option<Telescope> {
    let end = usize::from(clause_index).checked_add(1)?;
    Some(Telescope::new(candidate.clauses.get(..end)?.to_vec()))
}

fn eliminator_theorem_program(source_rule: ClosureRuleKind) -> EliminatorTheoremProgramV11 {
    let eliminator_version = MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION.to_owned();
    let eliminator_function =
        "pen_type::motive_parametric_coherence_v2::specialize_verified_closure_derivation_v2"
            .to_owned();
    let replay_function =
        "pen_type::motive_parametric_coherence_v2::replay_specialized_closure_derivation_v2"
            .to_owned();
    let closure_rule_inventory = CLOSURE_RULE_INVENTORY_V2.to_vec();
    let proof_status =
        EliminatorProofStatusV11::TheoremProgramBlockedByIssuableSourceCounterexamples;
    let empty_assignment_fiber =
        EmptyAssignmentFiberObservationV11::VacuityDoesNotEstablishNonemptyFibers;
    let probe_disposition = ProbeDispositionV11::RegressionOnlyNeverEvidence;
    let motive_filter = None;
    let known_counterexample_classes = vec![
        "contextual_with_prior_coarse_motive_vs_dereferenced_or_beta_motive".to_owned(),
        "incompatible_shared_ambient_declarations".to_owned(),
        "prior_field_projection_to_formation".to_owned(),
        "guarded_weakening_copies_unshifted_field_dependent_inferred_motive".to_owned(),
    ];
    let universal_specialization_claim_authorized = false;
    let contract_hash = tagged_hash(
        "universal-eliminator-contract",
        &(
            &eliminator_version,
            &eliminator_function,
            &replay_function,
            &closure_rule_inventory,
            source_rule,
            &proof_status,
            &empty_assignment_fiber,
            &probe_disposition,
            &motive_filter,
            &known_counterexample_classes,
            universal_specialization_claim_authorized,
        ),
    );
    EliminatorTheoremProgramV11 {
        eliminator_version,
        eliminator_function,
        replay_function,
        closure_rule_inventory,
        source_rule,
        proof_status,
        empty_assignment_fiber,
        probe_disposition,
        motive_filter,
        known_counterexample_classes,
        universal_specialization_claim_authorized,
        contract_hash,
    }
}

fn issue_live_source_certificate(
    candidate: &AmbientWrappedCandidate,
    declaration: AmbientContextDeclarationProjection,
) -> Result<ProofStrengthWrappedLiveSourceCertificateV11, WrappedCandidateDecisionV11> {
    let signature = SealedSignature::genesis_del_h15();
    let declaration_token = issue_ambient_context_declaration_token(
        &signature,
        &candidate.clauses,
        15,
        candidate.ambient.clone(),
    )
    .expect("validated wrapped declaration reissues exactly");
    debug_assert_eq!(declaration_token.projection(), &declaration);

    // This surface has no live field references.  Keeping the map typed and
    // empty is the important distinction from v9's fabricated string hash.
    let exact_prior_derivations = BTreeMap::new();
    let exact_prior_hashes = BTreeMap::new();
    let mut certified_nonlive_clauses = Vec::new();
    let mut exact_contextual_clauses = Vec::new();

    for clause_index in 0..u16::try_from(candidate.clauses.kappa()).expect("kappa fits u16") {
        match issue_contextual_typed_structure_token(
            &signature,
            &candidate.clauses,
            15,
            clause_index,
            &declaration_token,
            &exact_prior_hashes,
        ) {
            Ok(typed_structure) => {
                let closure = match issue_contextual_closure_derivation_v2(
                    &signature,
                    &candidate.clauses,
                    15,
                    clause_index,
                    &declaration_token,
                    &exact_prior_derivations,
                ) {
                    Ok(closure) => closure,
                    Err(error) => {
                        return Err(WrappedCandidateDecisionV11::NamedTypedObstruction {
                            code: WRAPPED_V11_EXACT_EVIDENCE_GAP.to_owned(),
                            clause_index,
                            reason: error.to_string(),
                            declaration,
                        });
                    }
                };
                if let Err(error) =
                    replay_verified_closure_derivation_v2(&signature, closure.projection())
                {
                    return Err(WrappedCandidateDecisionV11::NamedTypedObstruction {
                        code: WRAPPED_V11_EXACT_EVIDENCE_GAP.to_owned(),
                        clause_index,
                        reason: error.to_string(),
                        declaration,
                    });
                }
                if closure.projection().evidence
                    != (ClosureRuleEvidenceV2::Contextual {
                        typed_structure: typed_structure.projection().clone(),
                        prior_derivations: BTreeMap::new(),
                    })
                {
                    return Err(WrappedCandidateDecisionV11::NamedTypedObstruction {
                        code: WRAPPED_V11_EXACT_EVIDENCE_GAP.to_owned(),
                        clause_index,
                        reason: "v2 contextual relation did not retain the exact typed structure and empty used-field support".to_owned(),
                        declaration,
                    });
                }
                let exact_used_prior_clause_indices = match &closure.projection().evidence {
                    ClosureRuleEvidenceV2::Contextual {
                        prior_derivations, ..
                    } => prior_derivations.keys().copied().collect(),
                    _ => unreachable!("exact equality above selected Contextual evidence"),
                };
                let source_replayed = true;
                let marginal_nu = 0;
                let eliminator_theorem_program =
                    eliminator_theorem_program(closure.projection().rule);
                let derivation_hash = tagged_hash(
                    "exact-contextual-clause",
                    &(
                        clause_index,
                        typed_structure.projection(),
                        closure.projection(),
                        &exact_used_prior_clause_indices,
                        source_replayed,
                        &eliminator_theorem_program,
                        marginal_nu,
                    ),
                );
                exact_contextual_clauses.push(ExactContextualClauseV11 {
                    clause_index,
                    typed_structure: typed_structure.projection().clone(),
                    exact_closure_derivation: closure.projection().clone(),
                    exact_used_prior_clause_indices,
                    source_replayed,
                    eliminator_theorem_program,
                    marginal_nu,
                    derivation_hash,
                });
            }
            Err(error @ ContextualInternalityError::NoLiveAmbientUse) => {
                let Some(prefix) = exact_prefix(&candidate.clauses, clause_index) else {
                    return Err(WrappedCandidateDecisionV11::NamedTypedObstruction {
                        code: WRAPPED_V11_EXACT_EVIDENCE_GAP.to_owned(),
                        clause_index,
                        reason: "exact nonlive prefix was unavailable".to_owned(),
                        declaration,
                    });
                };
                let contextual_audit_disposition = ContextualAuditDispositionV11::NoLiveAmbientUse;
                let contextual_audit_error = error.to_string();
                let outside_live_contextual_support = true;
                let predecessor_internal_evidence = match issue_internal_derivability_attempt(
                    &prefix,
                ) {
                    Ok(evidence)
                        if evidence.candidate == prefix
                            && evidence.internal_certificate_issued
                            && evidence.clauses.get(usize::from(clause_index)).is_some_and(
                                |clause| {
                                    clause.expression
                                        == candidate.clauses.clauses[usize::from(clause_index)].expr
                                        && clause.derived_over_exact_b15
                                        && matches!(
                                            clause.kind,
                                            ClauseDerivabilityKind::AmbientArenaReference
                                        )
                                },
                            ) =>
                    {
                        evidence
                    }
                    Ok(_) => {
                        return Err(WrappedCandidateDecisionV11::NamedTypedObstruction {
                                code: WRAPPED_V11_EXACT_EVIDENCE_GAP.to_owned(),
                                clause_index,
                                reason: "exact nonlive prefix did not reissue inherited Internal arena-reference evidence".to_owned(),
                                declaration,
                            });
                    }
                    Err(error) => {
                        return Err(WrappedCandidateDecisionV11::NamedTypedObstruction {
                            code: WRAPPED_V11_EXACT_EVIDENCE_GAP.to_owned(),
                            clause_index,
                            reason: error.to_string(),
                            declaration,
                        });
                    }
                };
                let predecessor_evidence_reissued = issue_internal_derivability_attempt(&prefix)
                    .is_ok_and(|reissued| reissued == predecessor_internal_evidence);
                let derivation_hash = tagged_hash(
                    "certified-nonlive-clause",
                    &(
                        clause_index,
                        &prefix,
                        &contextual_audit_disposition,
                        &contextual_audit_error,
                        outside_live_contextual_support,
                        &predecessor_internal_evidence,
                        predecessor_evidence_reissued,
                    ),
                );
                certified_nonlive_clauses.push(CertifiedNonliveClauseV11 {
                    clause_index,
                    exact_prefix: prefix,
                    contextual_audit_disposition,
                    contextual_audit_error,
                    outside_live_contextual_support,
                    predecessor_internal_evidence,
                    predecessor_evidence_reissued,
                    derivation_hash,
                });
            }
            Err(error @ ContextualInternalityError::IllTypedAmbientUse { .. })
            | Err(error @ ContextualInternalityError::UndeclaredAmbientParameter { .. }) => {
                let disposition = match &error {
                    ContextualInternalityError::IllTypedAmbientUse { parameter } => {
                        ContextualAuditDispositionV11::IllTypedAmbientUse {
                            parameter: *parameter,
                        }
                    }
                    ContextualInternalityError::UndeclaredAmbientParameter { parameter } => {
                        ContextualAuditDispositionV11::UndeclaredAmbientParameter {
                            parameter: *parameter,
                        }
                    }
                    _ => unreachable!("matched exact typed-exclusion errors"),
                };
                return Err(WrappedCandidateDecisionV11::NamedTypedExclusion {
                    code: WRAPPED_V11_TYPED_EXCLUSION.to_owned(),
                    clause_index,
                    disposition,
                    error: error.to_string(),
                    declaration,
                });
            }
            Err(error) => {
                return Err(WrappedCandidateDecisionV11::NamedTypedObstruction {
                    code: WRAPPED_V11_EXACT_EVIDENCE_GAP.to_owned(),
                    clause_index,
                    reason: error.to_string(),
                    declaration,
                });
            }
        }
    }

    let mut clause_partition = certified_nonlive_clauses
        .iter()
        .map(|record| record.clause_index)
        .chain(
            exact_contextual_clauses
                .iter()
                .map(|record| record.clause_index),
        )
        .collect::<Vec<_>>();
    clause_partition.sort_unstable();
    let expected_partition =
        (0..u16::try_from(candidate.clauses.kappa()).expect("kappa fits u16")).collect::<Vec<_>>();
    let exact_live_support = exact_contextual_clauses
        .iter()
        .map(|record| {
            (
                record.clause_index,
                record.exact_used_prior_clause_indices.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let no_bare_prior_hash_accepted = exact_live_support.values().all(Vec::is_empty);
    let live_contextual_clause_coverage_complete = clause_partition == expected_partition;
    let at_least_one_live_contextual_clause = !exact_contextual_clauses.is_empty();
    let every_clause_has_exact_source_evidence = certified_nonlive_clauses
        .iter()
        .all(|record| record.predecessor_evidence_reissued)
        && exact_contextual_clauses
            .iter()
            .all(|record| record.source_replayed);
    let candidate_level_internality_authorized = false;
    let marginal_nu = 0;
    if !no_bare_prior_hash_accepted
        || !live_contextual_clause_coverage_complete
        || !at_least_one_live_contextual_clause
    {
        return Err(WrappedCandidateDecisionV11::NamedTypedObstruction {
            code: WRAPPED_V11_EXACT_EVIDENCE_GAP.to_owned(),
            clause_index: u16::MAX,
            reason:
                "proof-strength clause partition is incomplete or contains unearned prior support"
                    .to_owned(),
            declaration,
        });
    }
    let mut certificate = ProofStrengthWrappedLiveSourceCertificateV11 {
        version: WRAPPED_PROOF_STRENGTH_CLASSIFIER_VERSION.to_owned(),
        candidate: candidate.clone(),
        declaration,
        certified_nonlive_clauses,
        exact_contextual_clauses,
        clause_partition,
        exact_live_support,
        no_bare_prior_hash_accepted,
        live_contextual_clause_coverage_complete,
        at_least_one_live_contextual_clause,
        every_clause_has_exact_source_evidence,
        candidate_level_internality_authorized,
        marginal_nu,
        derivation_hash: String::new(),
    };
    certificate.derivation_hash = tagged_hash("proof-strength-wrapped-live-source", &certificate);
    Ok(certificate)
}

pub fn classify_wrapped_candidate_v11(
    candidate: &AmbientWrappedCandidate,
) -> Result<WrappedCandidateDecisionV11, AmbientWrapperError> {
    let declaration = validate_wrapped_candidate(candidate)?;
    if candidate.clauses != declared_context_control_candidate() {
        return Ok(WrappedCandidateDecisionV11::OutsideFrozenSuccessorSurface {
            code: WRAPPED_V11_OUTSIDE_SUCCESSOR_SURFACE.to_owned(),
            candidate: candidate.clone(),
        });
    }
    Ok(
        match issue_live_source_certificate(candidate, declaration) {
            Ok(certificate) => {
                let marginal_nu = certificate.marginal_nu;
                let derivation_hash = tagged_hash(
                    "v11-exact-contextual-internal",
                    &(&certificate, marginal_nu),
                );
                WrappedCandidateDecisionV11::ExactLiveContextualSourceEstablished {
                    certificate,
                    marginal_nu,
                    derivation_hash,
                }
            }
            Err(decision) => decision,
        },
    )
}

pub fn replay_wrapped_candidate_decision_v11(
    candidate: &AmbientWrappedCandidate,
    projection: &WrappedCandidateDecisionV11,
) -> Result<(), AmbientWrapperError> {
    let expected = classify_wrapped_candidate_v11(candidate)?;
    if expected == *projection {
        Ok(())
    } else {
        Err(AmbientWrapperError::Declaration(
            "proof-strength v11 decision replay mismatch".to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::internal_classifier_branch_v8::wrap_candidate;
    use pen_core::expr::Expr;
    use pen_type::contextual_internality::ContextualMotive;

    fn exact_v9_survivor() -> AmbientWrappedCandidate {
        wrap_candidate(
            declared_context_control_candidate(),
            vec![ContextualMotive::Function {
                domain: Box::new(ContextualMotive::Type),
                codomain: Box::new(ContextualMotive::Element(Expr::Univ)),
            }],
        )
        .expect("exact v9 survivor")
    }

    #[test]
    fn exact_v9_body_earns_contextual_source_with_empty_used_field_support() {
        let candidate = exact_v9_survivor();
        let decision = classify_wrapped_candidate_v11(&candidate).expect("classify");
        let WrappedCandidateDecisionV11::ExactLiveContextualSourceEstablished {
            certificate,
            marginal_nu,
            ..
        } = &decision
        else {
            panic!("exact v9 survivor must establish its exact live source: {decision:?}")
        };
        assert_eq!(certificate.certified_nonlive_clauses.len(), 1);
        let clause_zero = &certificate.certified_nonlive_clauses[0];
        assert!(clause_zero.predecessor_evidence_reissued);
        assert!(
            clause_zero
                .predecessor_internal_evidence
                .internal_certificate_issued
        );
        assert!(matches!(
            clause_zero.predecessor_internal_evidence.clauses[0].kind,
            ClauseDerivabilityKind::AmbientArenaReference
        ));
        assert_eq!(
            clause_zero.predecessor_internal_evidence.clauses[0].root_rule,
            "univ-form"
        );
        assert_eq!(certificate.exact_contextual_clauses.len(), 1);
        assert_eq!(certificate.clause_partition, vec![0, 1]);
        assert_eq!(certificate.exact_live_support.get(&1), Some(&Vec::new()));
        assert!(certificate.no_bare_prior_hash_accepted);
        assert!(certificate.every_clause_has_exact_source_evidence);
        assert!(!certificate.candidate_level_internality_authorized);
        assert_eq!(*marginal_nu, 0);
        replay_wrapped_candidate_decision_v11(&candidate, &decision).expect("replay");
    }

    #[test]
    fn incompatible_motive_is_an_exact_typed_exclusion() {
        let candidate = wrap_candidate(
            declared_context_control_candidate(),
            vec![ContextualMotive::Type],
        )
        .expect("declaration is formable even though its use is ill typed");
        assert!(matches!(
            classify_wrapped_candidate_v11(&candidate).expect("classify"),
            WrappedCandidateDecisionV11::NamedTypedExclusion {
                disposition: ContextualAuditDispositionV11::IllTypedAmbientUse { parameter: 1 },
                ..
            }
        ));
    }

    #[test]
    fn decision_and_exact_source_mutations_fail_closed() {
        let candidate = exact_v9_survivor();
        let decision = classify_wrapped_candidate_v11(&candidate).expect("classify");
        let mut mutation = decision.clone();
        let WrappedCandidateDecisionV11::ExactLiveContextualSourceEstablished {
            certificate, ..
        } = &mut mutation
        else {
            unreachable!()
        };
        certificate.exact_contextual_clauses[0]
            .exact_closure_derivation
            .expression = Expr::Univ;
        assert!(replay_wrapped_candidate_decision_v11(&candidate, &mutation).is_err());

        let mut inherited_mutation = decision.clone();
        let WrappedCandidateDecisionV11::ExactLiveContextualSourceEstablished {
            certificate, ..
        } = &mut inherited_mutation
        else {
            unreachable!()
        };
        certificate.certified_nonlive_clauses[0]
            .predecessor_internal_evidence
            .clauses[0]
            .derived_over_exact_b15 = false;
        assert!(replay_wrapped_candidate_decision_v11(&candidate, &inherited_mutation).is_err());

        let signature = SealedSignature::genesis_del_h15();
        let declaration = issue_ambient_context_declaration_token(
            &signature,
            &candidate.clauses,
            15,
            candidate.ambient.clone(),
        )
        .expect("declaration");
        let WrappedCandidateDecisionV11::ExactLiveContextualSourceEstablished {
            certificate, ..
        } = decision
        else {
            unreachable!()
        };
        let source = certificate.exact_contextual_clauses[0]
            .exact_closure_derivation
            .clone();
        let fabricated = BTreeMap::from([(0, source)]);
        assert!(
            issue_contextual_closure_derivation_v2(
                &signature,
                &candidate.clauses,
                15,
                1,
                &declaration,
                &fabricated,
            )
            .is_err()
        );
    }
}
