//! Versioned `Internal` classifier successor for the adopted transparent
//! ambient-former closure.
//!
//! Clauses are processed left-to-right.  A clause either inherits an earned
//! inductive certificate or replays the transparent term-constructor tree
//! against the hashes of already certified clauses.  The rule is zero-credit
//! and preserves the charged/fresh-head/guarded firewalls verbatim.

use crate::internal_classifier_branch_v2::RawCandidateDecisionV3;
use crate::internal_classifier_branch_v3::RawCandidateDecisionV4;
use crate::internal_classifier_branch_v4::{
    RawCandidateDecisionV5, STRUCTURAL_LAMBDA_INTERNALITY_SCHEMA,
    StructuralLambdaInternalityCertificate, classify_raw_candidate_v5,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::ambient_former_internality::{
    AmbientFormerClosureProjection, AmbientFormerInternalityError, TransparentFormer,
    issue_ambient_former_closure_token, registered_transparent_formers,
    replay_ambient_former_closure_projection, replay_ambient_former_closure_token,
};
use pen_type::elaborate::SealedSignature;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const AMBIENT_FORMER_INTERNALITY_SCHEMA: &str = "schema2-ambient-former-closure-internality-v1";
pub const AMBIENT_FORMER_INTERNALITY_DATE: &str = "2026-07-21";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/ambient_former_closure_adjudication.md");
const V5_RESULT_BYTES: &[u8] = include_bytes!("../../../docs/SCHEMA2_GLOBAL_E4_V5_RESULT.md");
const TOKEN_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/ambient_former_internality.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v4.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v5.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientFormerSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientFormerInheritedClause {
    pub clause_index: u16,
    pub predecessor_clause_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientFormerInternalCertificate {
    pub candidate: Telescope,
    pub signature_digest: String,
    pub visible_library: u32,
    pub predecessor_decision: RawCandidateDecisionV5,
    pub inherited_internal_clauses: Vec<AmbientFormerInheritedClause>,
    pub constructor_evidence: Vec<AmbientFormerClosureProjection>,
    pub final_certified_clause_hashes: BTreeMap<u16, String>,
    pub every_clause_covered_exactly_once: bool,
    pub ordered_prefix_discipline_replayed: bool,
    pub every_constructor_projection_replayed: bool,
    pub every_premise_internal: bool,
    pub typed_results_preserved: bool,
    pub full_provenance_retained: bool,
    pub charged_and_fresh_heads_excluded: bool,
    pub standing_orbit_exception_preserved: bool,
    pub internal_certificate_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RawCandidateDecisionV6 {
    NamedExclusion {
        predecessor_decision: RawCandidateDecisionV5,
    },
    InternalPredecessor {
        predecessor_decision: RawCandidateDecisionV5,
    },
    InternalAmbientFormer {
        certificate: AmbientFormerInternalCertificate,
        marginal_nu: u32,
        derivation_hash: String,
    },
    Classified {
        predecessor_decision: RawCandidateDecisionV5,
    },
    NamedTypedObstruction {
        predecessor_decision: RawCandidateDecisionV5,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransparentFormerObligationRecord {
    pub former: TransparentFormer,
    pub witness: Telescope,
    pub evidence: AmbientFormerClosureProjection,
    pub target_occurs_in_replayed_tree: bool,
    pub typed_result_preserved: bool,
    pub full_provenance_retained: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientFormerFalsifierRecord {
    pub pathcon_rejected_as_charged: bool,
    pub candidate_declared_formation_rejected: bool,
    pub uncertified_premise_rejected: bool,
    pub candidate_fresh_head_rejected: bool,
    pub guarded_candidate_rejected_without_inverse_evidence: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientFormerForbiddenOutputs {
    pub global_e4_v6_executed: bool,
    pub pending_membership_verdicts_issued: bool,
    pub e2b_executed: bool,
    pub stage_count_issued: bool,
    pub fq2_evaluated: bool,
    pub halt_or_continuation_claimed: bool,
}

impl AmbientFormerForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.global_e4_v6_executed
            && !self.pending_membership_verdicts_issued
            && !self.e2b_executed
            && !self.stage_count_issued
            && !self.fq2_evaluated
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientFormerInternalityCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<AmbientFormerSourceBinding>,
    pub predecessor_structural_digest: String,
    pub predecessor_structural_replayed: bool,
    pub adjudication_option_a_adopted: bool,
    pub registered_transparent_formers: Vec<TransparentFormer>,
    pub former_obligations: Vec<TransparentFormerObligationRecord>,
    pub former_inventory_complete: bool,
    pub falsifiers: AmbientFormerFalsifierRecord,
    pub opaque_constructor_token_public_fields_exposed: bool,
    pub application_predecessor_was_unknown: bool,
    pub application_successor_decision: RawCandidateDecisionV6,
    pub application_now_internal: bool,
    pub inherited_internal_clause_count: usize,
    pub constructor_evidence_count: usize,
    pub application_stuckness_accepted_as_computation_fact: bool,
    pub all_constructor_evidence_replayed: bool,
    pub marginal_nu: u32,
    pub global_e4_v6_authorized: bool,
    pub forbidden_outputs: AmbientFormerForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientFormerInternalityReplay {
    pub valid: bool,
    pub former_inventory_complete: bool,
    pub former_obligation_count: usize,
    pub falsifiers_replayed: bool,
    pub application_internal: bool,
    pub application_constructor_replayed: bool,
    pub marginal_nu: u32,
    pub global_e4_v6_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum AmbientFormerInternalityCertificateError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("constructor failed: {0}")]
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
    let bytes = serde_json::to_vec(&(AMBIENT_FORMER_INTERNALITY_SCHEMA, domain, value))
        .expect("ambient-former classifier evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(structural_bytes: &[u8]) -> Vec<AmbientFormerSourceBinding> {
    [
        (
            "docs/ambient_former_closure_adjudication.md",
            "adopted_option_a_partition",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/SCHEMA2_GLOBAL_E4_V5_RESULT.md",
            "exact_application_body_gap",
            V5_RESULT_BYTES,
        ),
        (
            "docs/schema2_structural_lambda_internality_v1.json",
            "classifier_predecessor_artifact",
            structural_bytes,
        ),
        (
            "crates/pen-type/src/ambient_former_internality.rs",
            "term_level_transparent_former_tokens",
            TOKEN_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v4.rs",
            "structural_lambda_classifier_predecessor",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v5.rs",
            "ambient_former_classifier_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| AmbientFormerSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

pub fn application_body_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Lib(14)),
                Box::new(Expr::Lib(15)),
            ))),
        ),
    ])
}

pub fn candidate_fresh_head_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(1)),
                Box::new(Expr::Lib(15)),
            ))),
        ),
    ])
}

fn issue_ambient_former_internal_certificate(
    candidate: &Telescope,
    predecessor_decision: RawCandidateDecisionV5,
) -> Result<AmbientFormerInternalCertificate, AmbientFormerInternalityCertificateError> {
    let RawCandidateDecisionV5::NamedTypedObstruction {
        predecessor_decision:
            RawCandidateDecisionV4::NamedTypedObstruction {
                predecessor_decision:
                    RawCandidateDecisionV3::NamedTypedObstruction {
                        internal_attempt, ..
                    },
            },
    } = &predecessor_decision
    else {
        return Err(AmbientFormerInternalityCertificateError::Constructor(
            "predecessor does not expose the inductive Internal attempt".to_owned(),
        ));
    };

    let signature = SealedSignature::genesis_del_h15();
    let mut inherited_internal_clauses = Vec::new();
    let mut constructor_evidence = Vec::new();
    let mut certified_hashes = BTreeMap::<u16, String>::new();
    for clause in &internal_attempt.clauses {
        if clause.earned_internal {
            inherited_internal_clauses.push(AmbientFormerInheritedClause {
                clause_index: clause.clause_index,
                predecessor_clause_derivation_hash: clause.derivation_hash.clone(),
            });
            certified_hashes.insert(clause.clause_index, clause.derivation_hash.clone());
            continue;
        }
        let token = issue_ambient_former_closure_token(
            &signature,
            candidate,
            15,
            clause.clause_index,
            &certified_hashes,
        )
        .map_err(|error| {
            AmbientFormerInternalityCertificateError::Constructor(error.to_string())
        })?;
        replay_ambient_former_closure_token(&signature, candidate, 15, &token).map_err(
            |error| AmbientFormerInternalityCertificateError::Constructor(error.to_string()),
        )?;
        let projection = token.projection().clone();
        replay_ambient_former_closure_projection(&signature, candidate, 15, &projection).map_err(
            |error| AmbientFormerInternalityCertificateError::Constructor(error.to_string()),
        )?;
        certified_hashes.insert(clause.clause_index, projection.derivation_hash.clone());
        constructor_evidence.push(projection);
    }

    let every_clause_covered_exactly_once = certified_hashes.len() == candidate.kappa()
        && certified_hashes
            .keys()
            .copied()
            .eq(0..u16::try_from(candidate.kappa()).expect("kappa fits u16"));
    let ordered_prefix_discipline_replayed = constructor_evidence.iter().all(|evidence| {
        evidence.every_dependency_strictly_prior
            && evidence
                .certified_prior_clauses
                .keys()
                .all(|dependency| *dependency < evidence.clause_index)
    });
    let every_constructor_projection_replayed = !constructor_evidence.is_empty();
    let every_premise_internal = constructor_evidence
        .iter()
        .all(|evidence| evidence.every_premise_replayed_internal);
    let typed_results_preserved = constructor_evidence
        .iter()
        .all(|evidence| evidence.typed_result_preserved);
    let full_provenance_retained = constructor_evidence
        .iter()
        .all(|evidence| evidence.full_provenance_retained);
    let charged_and_fresh_heads_excluded = constructor_evidence.iter().all(|evidence| {
        evidence.charged_path_constructor_excluded && evidence.term_evidence.no_candidate_fresh_head
    });
    let standing_orbit_exception_preserved = constructor_evidence
        .iter()
        .all(|evidence| evidence.standing_orbit_exception_preserved);
    let internal_certificate_issued = every_clause_covered_exactly_once
        && ordered_prefix_discipline_replayed
        && every_constructor_projection_replayed
        && every_premise_internal
        && typed_results_preserved
        && full_provenance_retained
        && charged_and_fresh_heads_excluded
        && standing_orbit_exception_preserved;
    let marginal_nu = 0;
    if !internal_certificate_issued {
        return Err(AmbientFormerInternalityCertificateError::Invariant(
            "ambient-former candidate coverage is incomplete".to_owned(),
        ));
    }
    let derivation_subject = (
        candidate,
        signature.digest(),
        15u32,
        &predecessor_decision,
        &inherited_internal_clauses,
        &constructor_evidence,
        &certified_hashes,
    );
    let derivation_checks = (
        every_clause_covered_exactly_once,
        ordered_prefix_discipline_replayed,
        every_constructor_projection_replayed,
        every_premise_internal,
        typed_results_preserved,
        full_provenance_retained,
        charged_and_fresh_heads_excluded,
        standing_orbit_exception_preserved,
        internal_certificate_issued,
        marginal_nu,
    );
    let derivation_hash = tagged_hash(
        "ambient-former-internal-certificate",
        &(derivation_subject, derivation_checks),
    );
    Ok(AmbientFormerInternalCertificate {
        candidate: candidate.clone(),
        signature_digest: signature.digest().to_owned(),
        visible_library: 15,
        predecessor_decision,
        inherited_internal_clauses,
        constructor_evidence,
        final_certified_clause_hashes: certified_hashes,
        every_clause_covered_exactly_once,
        ordered_prefix_discipline_replayed,
        every_constructor_projection_replayed,
        every_premise_internal,
        typed_results_preserved,
        full_provenance_retained,
        charged_and_fresh_heads_excluded,
        standing_orbit_exception_preserved,
        internal_certificate_issued,
        marginal_nu,
        derivation_hash,
    })
}

pub fn classify_raw_candidate_v6(candidate: &Telescope) -> RawCandidateDecisionV6 {
    let predecessor_decision = classify_raw_candidate_v5(candidate);
    if matches!(
        predecessor_decision,
        RawCandidateDecisionV5::NamedTypedObstruction { .. }
    ) {
        if let Ok(certificate) =
            issue_ambient_former_internal_certificate(candidate, predecessor_decision.clone())
        {
            let marginal_nu = certificate.marginal_nu;
            let derivation_hash = tagged_hash(
                "ambient-former-internal-classification",
                &(&certificate.derivation_hash, marginal_nu),
            );
            return RawCandidateDecisionV6::InternalAmbientFormer {
                certificate,
                marginal_nu,
                derivation_hash,
            };
        }
    }
    match predecessor_decision {
        decision @ RawCandidateDecisionV5::NamedExclusion { .. } => {
            RawCandidateDecisionV6::NamedExclusion {
                predecessor_decision: decision,
            }
        }
        decision @ (RawCandidateDecisionV5::InternalPredecessor { .. }
        | RawCandidateDecisionV5::InternalStructuralLambda { .. }) => {
            RawCandidateDecisionV6::InternalPredecessor {
                predecessor_decision: decision,
            }
        }
        decision @ RawCandidateDecisionV5::Classified { .. } => {
            RawCandidateDecisionV6::Classified {
                predecessor_decision: decision,
            }
        }
        decision @ RawCandidateDecisionV5::NamedTypedObstruction { .. } => {
            RawCandidateDecisionV6::NamedTypedObstruction {
                predecessor_decision: decision,
            }
        }
    }
}

fn witness_for_former(former: TransparentFormer) -> Telescope {
    let body = match former {
        TransparentFormer::AmbientUniverse => Expr::Univ,
        TransparentFormer::VariableReference => Expr::Var(2),
        TransparentFormer::SealedLibraryConstant => Expr::Lib(14),
        TransparentFormer::LambdaIntroduction => Expr::Lam(Box::new(Expr::Univ)),
        TransparentFormer::Application => {
            Expr::App(Box::new(Expr::Lib(14)), Box::new(Expr::Lib(15)))
        }
        TransparentFormer::PiFormation => Expr::Pi(Box::new(Expr::Univ), Box::new(Expr::Univ)),
        TransparentFormer::SigmaFormation => {
            Expr::Sigma(Box::new(Expr::Univ), Box::new(Expr::Univ))
        }
        TransparentFormer::IdentityFormation => Expr::Id(
            Box::new(Expr::Univ),
            Box::new(Expr::Lib(14)),
            Box::new(Expr::Lib(15)),
        ),
        TransparentFormer::ReflexivityIntroduction => Expr::Refl(Box::new(Expr::Lib(14))),
        TransparentFormer::SuspensionFormation => Expr::Susp(Box::new(Expr::Lib(14))),
        TransparentFormer::TruncationFormation => Expr::Trunc(Box::new(Expr::Lib(14))),
        TransparentFormer::FlatFormation => Expr::Flat(Box::new(Expr::Lib(14))),
        TransparentFormer::SharpFormation => Expr::Sharp(Box::new(Expr::Lib(14))),
        TransparentFormer::DiscreteFormation => Expr::Disc(Box::new(Expr::Lib(14))),
        TransparentFormer::ShapeFormation => Expr::Shape(Box::new(Expr::Lib(14))),
        TransparentFormer::NextFormation => Expr::Next(Box::new(Expr::Lib(14))),
        TransparentFormer::EventuallyFormation => Expr::Eventually(Box::new(Expr::Lib(14))),
    };
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Lam(Box::new(body))),
    ])
}

fn tree_contains_former(
    evidence: &pen_type::ambient_former_internality::AmbientInternalTermProjection,
    former: TransparentFormer,
) -> bool {
    evidence.former == former
        || evidence
            .premises
            .iter()
            .any(|premise| tree_contains_former(premise, former))
}

fn issue_former_obligations()
-> Result<Vec<TransparentFormerObligationRecord>, AmbientFormerInternalityCertificateError> {
    let signature = SealedSignature::genesis_del_h15();
    let certified = BTreeMap::from([(0, "blake3:obligation-field-zero".to_owned())]);
    registered_transparent_formers()
        .into_iter()
        .map(|former| {
            let witness = witness_for_former(former);
            let token = issue_ambient_former_closure_token(&signature, &witness, 15, 1, &certified)
                .map_err(|error| {
                    AmbientFormerInternalityCertificateError::Constructor(error.to_string())
                })?;
            replay_ambient_former_closure_token(&signature, &witness, 15, &token).map_err(
                |error| AmbientFormerInternalityCertificateError::Constructor(error.to_string()),
            )?;
            let evidence = token.projection().clone();
            let target_occurs_in_replayed_tree =
                tree_contains_former(&evidence.term_evidence, former);
            let typed_result_preserved = evidence.typed_result_preserved;
            let full_provenance_retained = evidence.full_provenance_retained;
            let marginal_nu = evidence.marginal_nu;
            let derivation_hash = tagged_hash(
                "transparent-former-obligation",
                &(
                    former,
                    &witness,
                    &evidence,
                    target_occurs_in_replayed_tree,
                    typed_result_preserved,
                    full_provenance_retained,
                    marginal_nu,
                ),
            );
            Ok(TransparentFormerObligationRecord {
                former,
                witness,
                evidence,
                target_occurs_in_replayed_tree,
                typed_result_preserved,
                full_provenance_retained,
                marginal_nu,
                derivation_hash,
            })
        })
        .collect()
}

fn issue_falsifiers() -> AmbientFormerFalsifierRecord {
    let signature = SealedSignature::genesis_del_h15();
    let certified = BTreeMap::from([(0, "blake3:falsifier-field-zero".to_owned())]);
    let introduction = |expr| {
        Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(ClauseRole::Introduction, expr),
        ])
    };
    let pathcon_rejected_as_charged = matches!(
        issue_ambient_former_closure_token(
            &signature,
            &introduction(Expr::Lam(Box::new(Expr::PathCon(1)))),
            15,
            1,
            &certified,
        ),
        Err(AmbientFormerInternalityError::ChargedPathConstructor)
    );
    let formation = Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(ClauseRole::Formation, Expr::Flat(Box::new(Expr::Lib(15)))),
    ]);
    let candidate_declared_formation_rejected = matches!(
        issue_ambient_former_closure_token(&signature, &formation, 15, 1, &certified),
        Err(AmbientFormerInternalityError::CandidateDeclaredFormation { .. })
    );
    let field_premise = introduction(Expr::Lam(Box::new(Expr::Refl(Box::new(Expr::Var(1))))));
    let uncertified_premise_rejected = matches!(
        issue_ambient_former_closure_token(&signature, &field_premise, 15, 1, &BTreeMap::new(),),
        Err(AmbientFormerInternalityError::UncertifiedCandidateField { .. })
    );
    let candidate_fresh_head_rejected = matches!(
        issue_ambient_former_closure_token(
            &signature,
            &candidate_fresh_head_candidate(),
            15,
            1,
            &certified,
        ),
        Err(AmbientFormerInternalityError::CandidateFreshApplicationHead)
    );
    let guarded = introduction(Expr::Lam(Box::new(Expr::Var(3))));
    let guarded_candidate_rejected_without_inverse_evidence = matches!(
        issue_ambient_former_closure_token(&signature, &guarded, 15, 1, &certified),
        Err(AmbientFormerInternalityError::GuardedCandidate { ambient: 1 })
    );
    let derivation_hash = tagged_hash(
        "ambient-former-falsifiers",
        &(
            pathcon_rejected_as_charged,
            candidate_declared_formation_rejected,
            uncertified_premise_rejected,
            candidate_fresh_head_rejected,
            guarded_candidate_rejected_without_inverse_evidence,
        ),
    );
    AmbientFormerFalsifierRecord {
        pathcon_rejected_as_charged,
        candidate_declared_formation_rejected,
        uncertified_premise_rejected,
        candidate_fresh_head_rejected,
        guarded_candidate_rejected_without_inverse_evidence,
        derivation_hash,
    }
}

fn certificate_digest(certificate: &AmbientFormerInternalityCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("ambient-former-internality-certificate", &projection)
}

fn sealed_structural_predecessor_valid(
    certificate: &StructuralLambdaInternalityCertificate,
) -> bool {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let digest_bytes = serde_json::to_vec(&(
        STRUCTURAL_LAMBDA_INTERNALITY_SCHEMA,
        "structural-lambda-internality-certificate",
        &projection,
    ))
    .expect("sealed structural predecessor serializes");
    let expected_digest = format!("blake3:{}", blake3_hex(&digest_bytes));
    certificate.schema == STRUCTURAL_LAMBDA_INTERNALITY_SCHEMA
        && certificate.result_digest == expected_digest
        && certificate.predecessor_guarded_replayed
        && certificate.global_e4_v4_obligation_source_bound
        && !certificate.opaque_constructor_token_public_fields_exposed
        && certificate.structural_predecessor_was_unknown
        && matches!(
            certificate.structural_successor_decision,
            RawCandidateDecisionV5::InternalStructuralLambda { .. }
        )
        && certificate.structural_now_internal
        && certificate.inherited_internal_clause_count == 1
        && certificate.constructor_evidence_count == 1
        && certificate.all_constructor_evidence_replayed
        && certificate.marginal_nu == 0
        && certificate.global_e4_rerun_authorized
        && !certificate.forbidden_outputs.global_e4_rerun_executed
        && !certificate
            .forbidden_outputs
            .pending_membership_verdicts_issued
        && !certificate.forbidden_outputs.e2b_executed
        && !certificate.forbidden_outputs.stage_count_issued
        && !certificate.forbidden_outputs.fq2_evaluated
        && !certificate.forbidden_outputs.halt_or_continuation_claimed
}

pub fn issue_ambient_former_internality_certificate()
-> Result<AmbientFormerInternalityCertificate, AmbientFormerInternalityCertificateError> {
    let structural_bytes = std::fs::read(workspace_doc_path(
        "schema2_structural_lambda_internality_v1.json",
    ))
    .map_err(|error| AmbientFormerInternalityCertificateError::Io(error.to_string()))?;
    let structural: StructuralLambdaInternalityCertificate =
        serde_json::from_slice(&structural_bytes)
            .map_err(|error| AmbientFormerInternalityCertificateError::Json(error.to_string()))?;
    if !sealed_structural_predecessor_valid(&structural) {
        return Err(AmbientFormerInternalityCertificateError::Prerequisite(
            "sealed structural predecessor digest or required invariants failed".to_owned(),
        ));
    }

    let registered_transparent_formers = registered_transparent_formers();
    let former_obligations = issue_former_obligations()?;
    let former_inventory_complete = former_obligations.len()
        == registered_transparent_formers.len()
        && former_obligations
            .iter()
            .map(|record| record.former)
            .eq(registered_transparent_formers.iter().copied())
        && former_obligations.iter().all(|record| {
            record.target_occurs_in_replayed_tree
                && record.typed_result_preserved
                && record.full_provenance_retained
                && record.marginal_nu == 0
        });
    let falsifiers = issue_falsifiers();
    let falsifiers_replayed = falsifiers.pathcon_rejected_as_charged
        && falsifiers.candidate_declared_formation_rejected
        && falsifiers.uncertified_premise_rejected
        && falsifiers.candidate_fresh_head_rejected
        && falsifiers.guarded_candidate_rejected_without_inverse_evidence;

    let candidate = application_body_candidate();
    let application_predecessor_was_unknown = matches!(
        classify_raw_candidate_v5(&candidate),
        RawCandidateDecisionV5::NamedTypedObstruction { .. }
    );
    let application_successor_decision = classify_raw_candidate_v6(&candidate);
    let (
        application_now_internal,
        inherited_internal_clause_count,
        constructor_evidence_count,
        application_stuckness_accepted_as_computation_fact,
        all_constructor_evidence_replayed,
        marginal_nu,
    ) = match &application_successor_decision {
        RawCandidateDecisionV6::InternalAmbientFormer {
            certificate,
            marginal_nu,
            ..
        } => {
            let application_stuckness_accepted_as_computation_fact = certificate
                .constructor_evidence
                .iter()
                .any(|evidence| tree_has_rule(&evidence.term_evidence, "app-stuck"));
            (
                certificate.internal_certificate_issued,
                certificate.inherited_internal_clauses.len(),
                certificate.constructor_evidence.len(),
                application_stuckness_accepted_as_computation_fact,
                certificate.every_constructor_projection_replayed
                    && certificate.every_premise_internal
                    && certificate.typed_results_preserved
                    && certificate.full_provenance_retained,
                *marginal_nu,
            )
        }
        _ => (false, 0, 0, false, false, u32::MAX),
    };
    let forbidden_outputs = AmbientFormerForbiddenOutputs {
        global_e4_v6_executed: false,
        pending_membership_verdicts_issued: false,
        e2b_executed: false,
        stage_count_issued: false,
        fq2_evaluated: false,
        halt_or_continuation_claimed: false,
    };
    if !former_inventory_complete
        || !falsifiers_replayed
        || !application_predecessor_was_unknown
        || !application_now_internal
        || inherited_internal_clause_count != 1
        || constructor_evidence_count != 1
        || !application_stuckness_accepted_as_computation_fact
        || !all_constructor_evidence_replayed
        || marginal_nu != 0
        || !forbidden_outputs.all_withheld()
    {
        return Err(AmbientFormerInternalityCertificateError::Invariant(
            "ambient-former successor disposition drifted".to_owned(),
        ));
    }

    let mut certificate = AmbientFormerInternalityCertificate {
        schema: AMBIENT_FORMER_INTERNALITY_SCHEMA.to_owned(),
        date: AMBIENT_FORMER_INTERNALITY_DATE.to_owned(),
        source_bindings: source_bindings(&structural_bytes),
        predecessor_structural_digest: structural.result_digest,
        predecessor_structural_replayed: true,
        adjudication_option_a_adopted: true,
        registered_transparent_formers,
        former_obligations,
        former_inventory_complete,
        falsifiers,
        opaque_constructor_token_public_fields_exposed: false,
        application_predecessor_was_unknown,
        application_successor_decision,
        application_now_internal,
        inherited_internal_clause_count,
        constructor_evidence_count,
        application_stuckness_accepted_as_computation_fact,
        all_constructor_evidence_replayed,
        marginal_nu,
        global_e4_v6_authorized: true,
        forbidden_outputs,
        outcome: "transparent_ambient_formers_internal_at_term_level_application_witness_resolved"
            .to_owned(),
        permitted_conclusion: "Every registered transparent former has a replayed term-level witness with typed normalization, complete sealed provenance, and nu=0. [Univ,Lam(App(Lib(14),Lib(15)))] now earns Internal even though its application is stuck; candidate-fresh heads, charged PathCon, candidate formation, uncertified premises, and unguarded ambient parameters remain excluded. Global E-4 v6 may run create-new."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn tree_has_rule(
    evidence: &pen_type::ambient_former_internality::AmbientInternalTermProjection,
    rule: &str,
) -> bool {
    evidence.derivation_rule == rule
        || evidence
            .premises
            .iter()
            .any(|premise| tree_has_rule(premise, rule))
}

fn failed_replay(error: impl Into<String>) -> AmbientFormerInternalityReplay {
    AmbientFormerInternalityReplay {
        valid: false,
        former_inventory_complete: false,
        former_obligation_count: 0,
        falsifiers_replayed: false,
        application_internal: false,
        application_constructor_replayed: false,
        marginal_nu: u32::MAX,
        global_e4_v6_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "ambient_former_internality_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &AmbientFormerInternalityCertificate,
    expected: &AmbientFormerInternalityCertificate,
) -> AmbientFormerInternalityReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    let falsifiers_replayed = certificate.falsifiers.pathcon_rejected_as_charged
        && certificate.falsifiers.candidate_declared_formation_rejected
        && certificate.falsifiers.uncertified_premise_rejected
        && certificate.falsifiers.candidate_fresh_head_rejected
        && certificate
            .falsifiers
            .guarded_candidate_rejected_without_inverse_evidence;
    AmbientFormerInternalityReplay {
        valid: errors.is_empty(),
        former_inventory_complete: certificate.former_inventory_complete,
        former_obligation_count: certificate.former_obligations.len(),
        falsifiers_replayed,
        application_internal: certificate.application_now_internal,
        application_constructor_replayed: certificate.all_constructor_evidence_replayed,
        marginal_nu: certificate.marginal_nu,
        global_e4_v6_authorized: certificate.global_e4_v6_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_ambient_former_internality_certificate(
    certificate: &AmbientFormerInternalityCertificate,
) -> AmbientFormerInternalityReplay {
    let expected = match issue_ambient_former_internality_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_ambient_former_internality_json(json: &str) -> AmbientFormerInternalityReplay {
    match serde_json::from_str::<AmbientFormerInternalityCertificate>(json) {
        Ok(certificate) => replay_ambient_former_internality_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_ambient_former_internality_create_new(
    path: &Path,
) -> Result<AmbientFormerInternalityReplay, AmbientFormerInternalityCertificateError> {
    let certificate = issue_ambient_former_internality_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| AmbientFormerInternalityCertificateError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| AmbientFormerInternalityCertificateError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| AmbientFormerInternalityCertificateError::Io(error.to_string()))?;
    let replay = replay_ambient_former_internality_certificate(&certificate);
    if !replay.valid {
        return Err(AmbientFormerInternalityCertificateError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_witness_earns_zero_credit_internal() {
        let decision = classify_raw_candidate_v6(&application_body_candidate());
        let RawCandidateDecisionV6::InternalAmbientFormer {
            certificate,
            marginal_nu,
            ..
        } = decision
        else {
            panic!("application witness must enter ambient Internal")
        };
        assert_eq!(certificate.inherited_internal_clauses.len(), 1);
        assert_eq!(certificate.constructor_evidence.len(), 1);
        assert!(certificate.every_premise_internal);
        assert!(certificate.typed_results_preserved);
        assert!(certificate.full_provenance_retained);
        assert_eq!(marginal_nu, 0);
    }

    #[test]
    fn former_matrix_and_falsifiers_are_complete() {
        let certificate = issue_ambient_former_internality_certificate().expect("certificate");
        assert!(certificate.former_inventory_complete);
        assert_eq!(
            certificate.former_obligations.len(),
            registered_transparent_formers().len()
        );
        assert!(certificate.falsifiers.pathcon_rejected_as_charged);
        assert!(certificate.falsifiers.candidate_declared_formation_rejected);
        assert!(certificate.falsifiers.uncertified_premise_rejected);
        assert!(certificate.falsifiers.candidate_fresh_head_rejected);
    }

    #[test]
    fn candidate_fresh_head_remains_named_unknown() {
        assert!(matches!(
            classify_raw_candidate_v6(&candidate_fresh_head_candidate()),
            RawCandidateDecisionV6::NamedTypedObstruction { .. }
        ));
    }

    #[test]
    fn certificate_replays_and_mutations_fail_closed() {
        let certificate = issue_ambient_former_internality_certificate().expect("certificate");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut inventory = certificate.clone();
        inventory.former_inventory_complete = false;
        mutations.push(inventory);
        let mut credit = certificate.clone();
        credit.marginal_nu = 1;
        mutations.push(credit);
        let mut fresh = certificate.clone();
        fresh.falsifiers.candidate_fresh_head_rejected = false;
        mutations.push(fresh);
        let mut forbidden = certificate.clone();
        forbidden.forbidden_outputs.e2b_executed = true;
        mutations.push(forbidden);
        for mutation in mutations {
            assert!(!replay_against_expected(&mutation, &certificate).valid);
        }
    }
}
