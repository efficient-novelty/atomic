//! Versioned `Internal` classifier successor for certified-field dereference.
//!
//! The predecessor decision and its complete inductive clause records remain
//! in the certificate.  For each unsupported clause we first replay the
//! unchanged ambient-former closure; only a remaining certified-field use is
//! routed through the versioned dereference token.

use crate::internal_classifier_branch_v2::{InductiveClauseRecord, RawCandidateDecisionV3};
use crate::internal_classifier_branch_v3::RawCandidateDecisionV4;
use crate::internal_classifier_branch_v4::RawCandidateDecisionV5;
use crate::internal_classifier_branch_v5::{
    AmbientFormerInternalityCertificate, RawCandidateDecisionV6, candidate_fresh_head_candidate,
    classify_raw_candidate_v6, replay_ambient_former_internality_json,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::ambient_former_internality::{
    AmbientFormerClosureProjection, issue_ambient_former_closure_token,
    replay_ambient_former_closure_projection, replay_ambient_former_closure_token,
};
use pen_type::certified_field_dereference::{
    CertifiedFieldDereferenceError, CertifiedFieldDereferenceProjection, CertifiedPriorField,
    issue_certified_field_dereference_token, replay_certified_field_dereference_projection,
    replay_certified_field_dereference_token,
};
use pen_type::elaborate::SealedSignature;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const CERTIFIED_FIELD_INTERNALITY_SCHEMA: &str =
    "schema2-certified-field-dereference-internality-v1";
pub const CERTIFIED_FIELD_INTERNALITY_DATE: &str = "2026-07-21";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/certified_field_dereference_adjudication.md");
const V6_RESULT_BYTES: &[u8] = include_bytes!("../../../docs/SCHEMA2_GLOBAL_E4_V6_RESULT.md");
const V6_ASSEMBLY_BYTES: &[u8] = include_bytes!("../../../docs/schema2_global_e4_assembly_v6.json");
const TOKEN_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/certified_field_dereference.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v5.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v6.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedFieldSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CertifiedFieldClauseEvidence {
    Ambient(AmbientFormerClosureProjection),
    Dereference(CertifiedFieldDereferenceProjection),
}

impl CertifiedFieldClauseEvidence {
    fn clause_index(&self) -> u16 {
        match self {
            Self::Ambient(evidence) => evidence.clause_index,
            Self::Dereference(evidence) => evidence.clause_index,
        }
    }

    fn derivation_hash(&self) -> &str {
        match self {
            Self::Ambient(evidence) => &evidence.derivation_hash,
            Self::Dereference(evidence) => &evidence.derivation_hash,
        }
    }

    fn marginal_nu(&self) -> u32 {
        match self {
            Self::Ambient(evidence) => evidence.marginal_nu,
            Self::Dereference(evidence) => evidence.marginal_nu,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedFieldInternalCertificate {
    pub candidate: Telescope,
    pub signature_digest: String,
    pub visible_library: u32,
    pub predecessor_decision: RawCandidateDecisionV6,
    pub inherited_internal_clauses: Vec<InductiveClauseRecord>,
    pub constructor_evidence: Vec<CertifiedFieldClauseEvidence>,
    pub final_certified_clause_hashes: BTreeMap<u16, String>,
    pub every_clause_covered_exactly_once: bool,
    pub complete_referent_internal_derivations_retained: bool,
    pub ordered_prefix_discipline_replayed: bool,
    pub every_constructor_projection_replayed: bool,
    pub every_dereference_semantically_invisible: bool,
    pub every_typed_result_preserved: bool,
    pub pathcon_remains_charged: bool,
    pub guarded_inverse_law_requirement_unchanged: bool,
    pub no_credit_anchor_or_family_minted: bool,
    pub internal_certificate_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RawCandidateDecisionV7 {
    NamedExclusion {
        predecessor_decision: RawCandidateDecisionV6,
    },
    InternalPredecessor {
        predecessor_decision: RawCandidateDecisionV6,
    },
    InternalCertifiedField {
        certificate: CertifiedFieldInternalCertificate,
        marginal_nu: u32,
        derivation_hash: String,
    },
    Classified {
        predecessor_decision: RawCandidateDecisionV6,
    },
    NamedTypedObstruction {
        predecessor_decision: RawCandidateDecisionV6,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedFieldFalsifierRecord {
    pub uncertified_reference_rejected: bool,
    pub forward_or_cyclic_reference_rejected: bool,
    pub projection_mutation_rejected: bool,
    pub guarded_candidate_rejected_without_inverse_evidence: bool,
    pub pathcon_rejected_as_charged: bool,
    pub zero_credit_preserved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedFieldForbiddenOutputs {
    pub global_e4_v7_executed: bool,
    pub pending_membership_verdicts_issued: bool,
    pub e2b_executed: bool,
    pub stage_count_issued: bool,
    pub fq2_evaluated: bool,
    pub halt_or_continuation_claimed: bool,
}

impl CertifiedFieldForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.global_e4_v7_executed
            && !self.pending_membership_verdicts_issued
            && !self.e2b_executed
            && !self.stage_count_issued
            && !self.fq2_evaluated
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedFieldInternalityCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<CertifiedFieldSourceBinding>,
    pub predecessor_ambient_digest: String,
    pub predecessor_ambient_replayed: bool,
    pub global_e4_v6_gap_source_bound: bool,
    pub adopted_dereference_rule_v1: bool,
    pub opaque_token_public_fields_exposed: bool,
    pub predecessor_was_candidate_fresh_head_unknown: bool,
    pub successor_decision: RawCandidateDecisionV7,
    pub candidate_now_internal: bool,
    pub inherited_internal_clause_count: usize,
    pub ambient_evidence_count: usize,
    pub dereference_evidence_count: usize,
    pub complete_referent_derivation_replayed: bool,
    pub direct_form: Telescope,
    pub direct_judgment_identical_after_certified_refinement: bool,
    pub marginal_nu: u32,
    pub falsifiers: CertifiedFieldFalsifierRecord,
    pub global_e4_v7_authorized: bool,
    pub forbidden_outputs: CertifiedFieldForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedFieldInternalityReplay {
    pub valid: bool,
    pub candidate_internal: bool,
    pub dereference_replayed: bool,
    pub direct_judgment_identical: bool,
    pub falsifiers_replayed: bool,
    pub marginal_nu: u32,
    pub global_e4_v7_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum CertifiedFieldInternalityError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("constructor evidence failed: {0}")]
    Constructor(String),
    #[error("invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn workspace_doc_path(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs")
        .join(name)
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CERTIFIED_FIELD_INTERNALITY_SCHEMA, domain, value))
        .expect("certified-field Internal evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(ambient_bytes: &[u8]) -> Vec<CertifiedFieldSourceBinding> {
    [
        (
            "docs/certified_field_dereference_adjudication.md",
            "adopted_rule_and_falsifiers",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/SCHEMA2_GLOBAL_E4_V6_RESULT.md",
            "exact_candidate_fresh_head_gap",
            V6_RESULT_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v6.json",
            "sealed_global_e4_v6_predecessor",
            V6_ASSEMBLY_BYTES,
        ),
        (
            "docs/schema2_ambient_former_internality_v1.json",
            "ambient_former_predecessor_artifact",
            ambient_bytes,
        ),
        (
            "crates/pen-type/src/certified_field_dereference.rs",
            "opaque_dereference_replay_token",
            TOKEN_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v5.rs",
            "ambient_former_classifier_predecessor",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v6.rs",
            "certified_field_classifier_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| CertifiedFieldSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn global_v6_gap_is_exactly_bound() -> bool {
    let Ok(value) = serde_json::from_slice::<serde_json::Value>(V6_ASSEMBLY_BYTES) else {
        return false;
    };
    value.get("schema").and_then(serde_json::Value::as_str) == Some("schema2-global-e4-assembly-v6")
        && value
            .pointer("/next_candidate_fresh_head_unknown/exact_raw_catalog_member")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        && value
            .pointer(
                "/next_candidate_fresh_head_unknown/f_a3_candidate_fresh_head_firewall_replayed",
            )
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        && value
            .get("class_exhaustion_proved")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
        && value
            .get("global_e4_complete")
            .and_then(serde_json::Value::as_bool)
            == Some(false)
}

fn predecessor_internal_attempt(
    decision: &RawCandidateDecisionV6,
) -> Option<&crate::internal_classifier_branch_v2::InductiveInternalityAttempt> {
    let RawCandidateDecisionV6::NamedTypedObstruction {
        predecessor_decision:
            RawCandidateDecisionV5::NamedTypedObstruction {
                predecessor_decision:
                    RawCandidateDecisionV4::NamedTypedObstruction {
                        predecessor_decision:
                            RawCandidateDecisionV3::NamedTypedObstruction {
                                internal_attempt, ..
                            },
                    },
            },
    } = decision
    else {
        return None;
    };
    Some(internal_attempt)
}

fn issue_certified_field_internal_certificate(
    candidate: &Telescope,
    predecessor_decision: RawCandidateDecisionV6,
) -> Result<CertifiedFieldInternalCertificate, CertifiedFieldInternalityError> {
    let internal_attempt =
        predecessor_internal_attempt(&predecessor_decision).ok_or_else(|| {
            CertifiedFieldInternalityError::Constructor(
                "predecessor does not expose the inductive Internal attempt".to_owned(),
            )
        })?;
    let signature = SealedSignature::genesis_del_h15();
    let mut inherited_internal_clauses = Vec::new();
    let mut constructor_evidence = Vec::new();
    let mut certified_hashes = BTreeMap::<u16, String>::new();
    let mut certified_fields = BTreeMap::<u16, CertifiedPriorField>::new();

    for clause in &internal_attempt.clauses {
        if clause.earned_internal {
            inherited_internal_clauses.push(clause.clone());
            certified_hashes.insert(clause.clause_index, clause.derivation_hash.clone());
            certified_fields.insert(
                clause.clause_index,
                CertifiedPriorField {
                    clause_index: clause.clause_index,
                    certificate_hash: clause.derivation_hash.clone(),
                    referent_expression: clause.expression.clone(),
                },
            );
            continue;
        }

        let evidence = match issue_ambient_former_closure_token(
            &signature,
            candidate,
            15,
            clause.clause_index,
            &certified_hashes,
        ) {
            Ok(token) => {
                replay_ambient_former_closure_token(&signature, candidate, 15, &token).map_err(
                    |error| CertifiedFieldInternalityError::Constructor(error.to_string()),
                )?;
                let projection = token.projection().clone();
                replay_ambient_former_closure_projection(&signature, candidate, 15, &projection)
                    .map_err(|error| {
                        CertifiedFieldInternalityError::Constructor(error.to_string())
                    })?;
                CertifiedFieldClauseEvidence::Ambient(projection)
            }
            Err(_) => {
                let dependencies = clause
                    .field_dependencies
                    .iter()
                    .filter_map(|dependency| {
                        certified_fields
                            .get(dependency)
                            .cloned()
                            .map(|evidence| (*dependency, evidence))
                    })
                    .collect::<BTreeMap<_, _>>();
                let token = issue_certified_field_dereference_token(
                    &signature,
                    candidate,
                    15,
                    clause.clause_index,
                    &dependencies,
                )
                .map_err(|error| CertifiedFieldInternalityError::Constructor(error.to_string()))?;
                replay_certified_field_dereference_token(&signature, candidate, 15, &token)
                    .map_err(|error| {
                        CertifiedFieldInternalityError::Constructor(error.to_string())
                    })?;
                let projection = token.projection().clone();
                replay_certified_field_dereference_projection(
                    &signature,
                    candidate,
                    15,
                    &projection,
                )
                .map_err(|error| CertifiedFieldInternalityError::Constructor(error.to_string()))?;
                CertifiedFieldClauseEvidence::Dereference(projection)
            }
        };
        let derivation_hash = evidence.derivation_hash().to_owned();
        certified_hashes.insert(clause.clause_index, derivation_hash.clone());
        certified_fields.insert(
            clause.clause_index,
            CertifiedPriorField {
                clause_index: clause.clause_index,
                certificate_hash: derivation_hash,
                referent_expression: clause.expression.clone(),
            },
        );
        constructor_evidence.push(evidence);
    }

    let every_clause_covered_exactly_once = certified_hashes.len() == candidate.kappa()
        && certified_hashes
            .keys()
            .copied()
            .eq(0..u16::try_from(candidate.kappa()).expect("kappa fits u16"));
    let complete_referent_internal_derivations_retained = !inherited_internal_clauses.is_empty()
        && inherited_internal_clauses.iter().all(|clause| {
            clause.earned_internal
                && certified_hashes.get(&clause.clause_index) == Some(&clause.derivation_hash)
        });
    let ordered_prefix_discipline_replayed = constructor_evidence.iter().all(|evidence| {
        evidence.clause_index() < u16::try_from(candidate.kappa()).expect("kappa fits u16")
            && match evidence {
                CertifiedFieldClauseEvidence::Ambient(evidence) => evidence
                    .certified_prior_clauses
                    .keys()
                    .all(|dependency| *dependency < evidence.clause_index),
                CertifiedFieldClauseEvidence::Dereference(evidence) => evidence
                    .certified_prior_fields
                    .keys()
                    .all(|dependency| *dependency < evidence.clause_index),
            }
    });
    let every_constructor_projection_replayed = !constructor_evidence.is_empty();
    let every_dereference_semantically_invisible = constructor_evidence.iter().all(|evidence| {
        !matches!(evidence, CertifiedFieldClauseEvidence::Dereference(projection) if !projection.dereference_semantically_invisible)
    });
    let every_typed_result_preserved = constructor_evidence.iter().all(|evidence| match evidence {
        CertifiedFieldClauseEvidence::Ambient(evidence) => evidence.typed_result_preserved,
        CertifiedFieldClauseEvidence::Dereference(evidence) => {
            evidence.direct_closure.typed_result_preserved
                && evidence.certified_kernel_type_refinement_valid
                && evidence.normal_form_identical_to_direct_form
        }
    });
    let pathcon_remains_charged = constructor_evidence.iter().all(|evidence| match evidence {
        CertifiedFieldClauseEvidence::Ambient(evidence) => {
            evidence.charged_path_constructor_excluded
        }
        CertifiedFieldClauseEvidence::Dereference(evidence) => evidence.pathcon_remains_charged,
    });
    let guarded_inverse_law_requirement_unchanged = constructor_evidence.iter().all(|evidence| {
        !matches!(evidence, CertifiedFieldClauseEvidence::Dereference(projection) if !projection.guarded_inverse_law_requirement_unchanged)
    });
    let no_credit_anchor_or_family_minted = constructor_evidence.iter().all(|evidence| {
        evidence.marginal_nu() == 0
            && !matches!(evidence, CertifiedFieldClauseEvidence::Dereference(projection) if !projection.no_credit_anchor_or_family_minted)
    });
    let internal_certificate_issued = every_clause_covered_exactly_once
        && complete_referent_internal_derivations_retained
        && ordered_prefix_discipline_replayed
        && every_constructor_projection_replayed
        && every_dereference_semantically_invisible
        && every_typed_result_preserved
        && pathcon_remains_charged
        && guarded_inverse_law_requirement_unchanged
        && no_credit_anchor_or_family_minted;
    let marginal_nu = 0;
    if !internal_certificate_issued {
        return Err(CertifiedFieldInternalityError::Invariant(
            "certified-field candidate coverage is incomplete".to_owned(),
        ));
    }
    let subject = (
        candidate,
        signature.digest(),
        &predecessor_decision,
        &inherited_internal_clauses,
        &constructor_evidence,
        &certified_hashes,
    );
    let checks = (
        every_clause_covered_exactly_once,
        complete_referent_internal_derivations_retained,
        ordered_prefix_discipline_replayed,
        every_constructor_projection_replayed,
        every_dereference_semantically_invisible,
        every_typed_result_preserved,
        pathcon_remains_charged,
        guarded_inverse_law_requirement_unchanged,
        no_credit_anchor_or_family_minted,
        internal_certificate_issued,
        marginal_nu,
    );
    let derivation_hash = tagged_hash("certified-field-internal-certificate", &(subject, checks));
    Ok(CertifiedFieldInternalCertificate {
        candidate: candidate.clone(),
        signature_digest: signature.digest().to_owned(),
        visible_library: 15,
        predecessor_decision,
        inherited_internal_clauses,
        constructor_evidence,
        final_certified_clause_hashes: certified_hashes,
        every_clause_covered_exactly_once,
        complete_referent_internal_derivations_retained,
        ordered_prefix_discipline_replayed,
        every_constructor_projection_replayed,
        every_dereference_semantically_invisible,
        every_typed_result_preserved,
        pathcon_remains_charged,
        guarded_inverse_law_requirement_unchanged,
        no_credit_anchor_or_family_minted,
        internal_certificate_issued,
        marginal_nu,
        derivation_hash,
    })
}

pub fn classify_raw_candidate_v7(candidate: &Telescope) -> RawCandidateDecisionV7 {
    let predecessor_decision = classify_raw_candidate_v6(candidate);
    if matches!(
        predecessor_decision,
        RawCandidateDecisionV6::NamedTypedObstruction { .. }
    ) {
        if let Ok(certificate) =
            issue_certified_field_internal_certificate(candidate, predecessor_decision.clone())
        {
            let marginal_nu = certificate.marginal_nu;
            let derivation_hash = tagged_hash(
                "certified-field-internal-classification",
                &(&certificate.derivation_hash, marginal_nu),
            );
            return RawCandidateDecisionV7::InternalCertifiedField {
                certificate,
                marginal_nu,
                derivation_hash,
            };
        }
    }
    match predecessor_decision {
        decision @ RawCandidateDecisionV6::NamedExclusion { .. } => {
            RawCandidateDecisionV7::NamedExclusion {
                predecessor_decision: decision,
            }
        }
        decision @ (RawCandidateDecisionV6::InternalPredecessor { .. }
        | RawCandidateDecisionV6::InternalAmbientFormer { .. }) => {
            RawCandidateDecisionV7::InternalPredecessor {
                predecessor_decision: decision,
            }
        }
        decision @ RawCandidateDecisionV6::Classified { .. } => {
            RawCandidateDecisionV7::Classified {
                predecessor_decision: decision,
            }
        }
        decision @ RawCandidateDecisionV6::NamedTypedObstruction { .. } => {
            RawCandidateDecisionV7::NamedTypedObstruction {
                predecessor_decision: decision,
            }
        }
    }
}

fn direct_form_from_decision(decision: &RawCandidateDecisionV7) -> Option<Telescope> {
    let RawCandidateDecisionV7::InternalCertifiedField { certificate, .. } = decision else {
        return None;
    };
    certificate
        .constructor_evidence
        .iter()
        .find_map(|evidence| {
            let CertifiedFieldClauseEvidence::Dereference(evidence) = evidence else {
                return None;
            };
            Some(evidence.direct_candidate.clone())
        })
}

fn issue_falsifiers() -> CertifiedFieldFalsifierRecord {
    let signature = SealedSignature::genesis_del_h15();
    let candidate = candidate_fresh_head_candidate();
    let uncertified_reference_rejected = matches!(
        issue_certified_field_dereference_token(&signature, &candidate, 15, 1, &BTreeMap::new()),
        Err(CertifiedFieldDereferenceError::UncertifiedField { .. })
    );
    let forward = BTreeMap::from([(
        1,
        CertifiedPriorField {
            clause_index: 1,
            certificate_hash: "forged".to_owned(),
            referent_expression: candidate.clauses[1].expr.clone(),
        },
    )]);
    let forward_or_cyclic_reference_rejected = matches!(
        issue_certified_field_dereference_token(&signature, &candidate, 15, 1, &forward),
        Err(CertifiedFieldDereferenceError::ForwardOrCyclic { .. })
    );
    let certified = BTreeMap::from([(
        0,
        CertifiedPriorField {
            clause_index: 0,
            certificate_hash: "earned-internal-clause-0".to_owned(),
            referent_expression: candidate.clauses[0].expr.clone(),
        },
    )]);
    let token = issue_certified_field_dereference_token(&signature, &candidate, 15, 1, &certified)
        .expect("control dereference");
    let mut mutated = token.projection().clone();
    mutated.marginal_nu = 1;
    let projection_mutation_rejected = matches!(
        replay_certified_field_dereference_projection(&signature, &candidate, 15, &mutated,),
        Err(CertifiedFieldDereferenceError::ReplayMismatch)
    );
    let guarded = Telescope::new(vec![
        pen_core::clause::ClauseRec::new(
            pen_core::clause::ClauseRole::Formation,
            pen_core::expr::Expr::Univ,
        ),
        pen_core::clause::ClauseRec::new(
            pen_core::clause::ClauseRole::Introduction,
            pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Var(3)),
                Box::new(pen_core::expr::Expr::Var(1)),
            ))),
        ),
    ]);
    let guarded_candidate_rejected_without_inverse_evidence = matches!(
        issue_certified_field_dereference_token(&signature, &guarded, 15, 1, &certified),
        Err(CertifiedFieldDereferenceError::GuardedCandidate { .. })
    );
    let pathcon = Telescope::new(vec![
        pen_core::clause::ClauseRec::new(
            pen_core::clause::ClauseRole::Formation,
            pen_core::expr::Expr::Univ,
        ),
        pen_core::clause::ClauseRec::new(
            pen_core::clause::ClauseRole::Introduction,
            pen_core::expr::Expr::Lam(Box::new(pen_core::expr::Expr::App(
                Box::new(pen_core::expr::Expr::Var(1)),
                Box::new(pen_core::expr::Expr::PathCon(1)),
            ))),
        ),
    ]);
    let pathcon_rejected_as_charged = matches!(
        issue_certified_field_dereference_token(&signature, &pathcon, 15, 1, &certified),
        Err(CertifiedFieldDereferenceError::DirectClosure(message)) if message.contains("PathCon")
    );
    let zero_credit_preserved =
        token.marginal_nu() == 0 && token.projection().no_credit_anchor_or_family_minted;
    let derivation_hash = tagged_hash(
        "certified-field-falsifiers",
        &(
            uncertified_reference_rejected,
            forward_or_cyclic_reference_rejected,
            projection_mutation_rejected,
            guarded_candidate_rejected_without_inverse_evidence,
            pathcon_rejected_as_charged,
            zero_credit_preserved,
        ),
    );
    CertifiedFieldFalsifierRecord {
        uncertified_reference_rejected,
        forward_or_cyclic_reference_rejected,
        projection_mutation_rejected,
        guarded_candidate_rejected_without_inverse_evidence,
        pathcon_rejected_as_charged,
        zero_credit_preserved,
        derivation_hash,
    }
}

fn certificate_digest(certificate: &CertifiedFieldInternalityCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certified-field-internality-certificate", &projection)
}

pub fn issue_certified_field_internality_certificate()
-> Result<CertifiedFieldInternalityCertificate, CertifiedFieldInternalityError> {
    let ambient_bytes = std::fs::read(workspace_doc_path(
        "schema2_ambient_former_internality_v1.json",
    ))
    .map_err(|error| CertifiedFieldInternalityError::Io(error.to_string()))?;
    let ambient_replay = replay_ambient_former_internality_json(
        std::str::from_utf8(&ambient_bytes)
            .map_err(|error| CertifiedFieldInternalityError::Json(error.to_string()))?,
    );
    if !ambient_replay.valid || !ambient_replay.application_internal {
        return Err(CertifiedFieldInternalityError::Prerequisite(format!(
            "ambient predecessor replay failed: {}",
            ambient_replay.errors.join("; ")
        )));
    }
    let ambient: AmbientFormerInternalityCertificate = serde_json::from_slice(&ambient_bytes)
        .map_err(|error| CertifiedFieldInternalityError::Json(error.to_string()))?;
    if !global_v6_gap_is_exactly_bound() {
        return Err(CertifiedFieldInternalityError::Prerequisite(
            "global E-4 v6 gap binding failed".to_owned(),
        ));
    }

    let candidate = candidate_fresh_head_candidate();
    let predecessor_was_candidate_fresh_head_unknown = matches!(
        classify_raw_candidate_v6(&candidate),
        RawCandidateDecisionV6::NamedTypedObstruction { .. }
    );
    let successor_decision = classify_raw_candidate_v7(&candidate);
    let (
        candidate_now_internal,
        inherited_internal_clause_count,
        ambient_evidence_count,
        dereference_evidence_count,
        complete_referent_derivation_replayed,
        direct_judgment_identical_after_certified_refinement,
        marginal_nu,
    ) = match &successor_decision {
        RawCandidateDecisionV7::InternalCertifiedField {
            certificate,
            marginal_nu,
            ..
        } => (
            certificate.internal_certificate_issued,
            certificate.inherited_internal_clauses.len(),
            certificate
                .constructor_evidence
                .iter()
                .filter(|evidence| matches!(evidence, CertifiedFieldClauseEvidence::Ambient(_)))
                .count(),
            certificate
                .constructor_evidence
                .iter()
                .filter(|evidence| matches!(evidence, CertifiedFieldClauseEvidence::Dereference(_)))
                .count(),
            certificate.complete_referent_internal_derivations_retained,
            certificate.every_dereference_semantically_invisible,
            *marginal_nu,
        ),
        _ => (false, 0, 0, 0, false, false, u32::MAX),
    };
    let direct_form = direct_form_from_decision(&successor_decision).ok_or_else(|| {
        CertifiedFieldInternalityError::Invariant("dereference direct form absent".to_owned())
    })?;
    let falsifiers = issue_falsifiers();
    let falsifiers_replayed = falsifiers.uncertified_reference_rejected
        && falsifiers.forward_or_cyclic_reference_rejected
        && falsifiers.projection_mutation_rejected
        && falsifiers.guarded_candidate_rejected_without_inverse_evidence
        && falsifiers.pathcon_rejected_as_charged
        && falsifiers.zero_credit_preserved;
    let forbidden_outputs = CertifiedFieldForbiddenOutputs {
        global_e4_v7_executed: false,
        pending_membership_verdicts_issued: false,
        e2b_executed: false,
        stage_count_issued: false,
        fq2_evaluated: false,
        halt_or_continuation_claimed: false,
    };
    if !predecessor_was_candidate_fresh_head_unknown
        || !candidate_now_internal
        || inherited_internal_clause_count != 1
        || ambient_evidence_count != 0
        || dereference_evidence_count != 1
        || !complete_referent_derivation_replayed
        || !direct_judgment_identical_after_certified_refinement
        || marginal_nu != 0
        || !falsifiers_replayed
        || !forbidden_outputs.all_withheld()
    {
        return Err(CertifiedFieldInternalityError::Invariant(
            "certified-field successor disposition drifted".to_owned(),
        ));
    }

    let mut certificate = CertifiedFieldInternalityCertificate {
        schema: CERTIFIED_FIELD_INTERNALITY_SCHEMA.to_owned(),
        date: CERTIFIED_FIELD_INTERNALITY_DATE.to_owned(),
        source_bindings: source_bindings(&ambient_bytes),
        predecessor_ambient_digest: ambient.result_digest,
        predecessor_ambient_replayed: true,
        global_e4_v6_gap_source_bound: true,
        adopted_dereference_rule_v1: true,
        opaque_token_public_fields_exposed: false,
        predecessor_was_candidate_fresh_head_unknown,
        successor_decision,
        candidate_now_internal,
        inherited_internal_clause_count,
        ambient_evidence_count,
        dereference_evidence_count,
        complete_referent_derivation_replayed,
        direct_form,
        direct_judgment_identical_after_certified_refinement,
        marginal_nu,
        falsifiers,
        global_e4_v7_authorized: true,
        forbidden_outputs,
        outcome: "certified_field_head_dereferences_to_direct_internal_form_at_zero_credit"
            .to_owned(),
        permitted_conclusion: "The exact [Univ,Lam(App(Var(1),Lib(15)))] witness replays clause 0's complete earned Internal derivation, dereferences to [Univ,Lam(App(Univ,Lib(15)))], and has the identical refined direct judgment. It earns Internal with nu=0; uncertified, forward/cyclic, guarded-without-inverses, and PathCon cases remain excluded. Global E-4 v7 may run create-new."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> CertifiedFieldInternalityReplay {
    CertifiedFieldInternalityReplay {
        valid: false,
        candidate_internal: false,
        dereference_replayed: false,
        direct_judgment_identical: false,
        falsifiers_replayed: false,
        marginal_nu: u32::MAX,
        global_e4_v7_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "certified_field_internality_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &CertifiedFieldInternalityCertificate,
    expected: &CertifiedFieldInternalityCertificate,
) -> CertifiedFieldInternalityReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    let falsifiers_replayed = certificate.falsifiers.uncertified_reference_rejected
        && certificate.falsifiers.forward_or_cyclic_reference_rejected
        && certificate.falsifiers.projection_mutation_rejected
        && certificate
            .falsifiers
            .guarded_candidate_rejected_without_inverse_evidence
        && certificate.falsifiers.pathcon_rejected_as_charged
        && certificate.falsifiers.zero_credit_preserved;
    CertifiedFieldInternalityReplay {
        valid: errors.is_empty(),
        candidate_internal: certificate.candidate_now_internal,
        dereference_replayed: certificate.dereference_evidence_count == 1
            && certificate.complete_referent_derivation_replayed,
        direct_judgment_identical: certificate.direct_judgment_identical_after_certified_refinement,
        falsifiers_replayed,
        marginal_nu: certificate.marginal_nu,
        global_e4_v7_authorized: certificate.global_e4_v7_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_certified_field_internality_certificate(
    certificate: &CertifiedFieldInternalityCertificate,
) -> CertifiedFieldInternalityReplay {
    let expected = match issue_certified_field_internality_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_certified_field_internality_json(json: &str) -> CertifiedFieldInternalityReplay {
    match serde_json::from_str::<CertifiedFieldInternalityCertificate>(json) {
        Ok(certificate) => replay_certified_field_internality_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_certified_field_internality_create_new(
    path: &Path,
) -> Result<CertifiedFieldInternalityReplay, CertifiedFieldInternalityError> {
    let certificate = issue_certified_field_internality_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| CertifiedFieldInternalityError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| CertifiedFieldInternalityError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| CertifiedFieldInternalityError::Io(error.to_string()))?;
    let replay = replay_certified_field_internality_certificate(&certificate);
    if !replay.valid {
        return Err(CertifiedFieldInternalityError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn candidate_fresh_head_earns_internal_by_dereference() {
        let decision = classify_raw_candidate_v7(&candidate_fresh_head_candidate());
        let RawCandidateDecisionV7::InternalCertifiedField {
            certificate,
            marginal_nu,
            ..
        } = decision
        else {
            panic!("candidate-fresh certified head must enter Internal")
        };
        assert_eq!(certificate.inherited_internal_clauses.len(), 1);
        assert_eq!(certificate.constructor_evidence.len(), 1);
        assert!(matches!(
            certificate.constructor_evidence[0],
            CertifiedFieldClauseEvidence::Dereference(_)
        ));
        assert!(certificate.every_dereference_semantically_invisible);
        assert_eq!(marginal_nu, 0);
    }

    #[test]
    fn falsifiers_preserve_f_a3_guarded_and_pathcon_firewalls() {
        let falsifiers = issue_falsifiers();
        assert!(falsifiers.uncertified_reference_rejected);
        assert!(falsifiers.forward_or_cyclic_reference_rejected);
        assert!(falsifiers.projection_mutation_rejected);
        assert!(falsifiers.guarded_candidate_rejected_without_inverse_evidence);
        assert!(falsifiers.pathcon_rejected_as_charged);
        assert!(falsifiers.zero_credit_preserved);
    }

    #[test]
    fn certificate_replays_and_mutations_fail_closed() {
        let certificate = issue_certified_field_internality_certificate().expect("certificate");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut direct = certificate.clone();
        direct.direct_judgment_identical_after_certified_refinement = false;
        mutations.push(direct);
        let mut credit = certificate.clone();
        credit.marginal_nu = 1;
        mutations.push(credit);
        let mut forward = certificate.clone();
        forward.falsifiers.forward_or_cyclic_reference_rejected = false;
        mutations.push(forward);
        let mut forbidden = certificate.clone();
        forbidden.forbidden_outputs.e2b_executed = true;
        mutations.push(forbidden);
        for mutation in mutations {
            assert!(!replay_against_expected(&mutation, &certificate).valid);
        }
    }
}
