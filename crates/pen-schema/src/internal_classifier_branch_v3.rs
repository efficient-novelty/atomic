//! Versioned Internal-classifier successor consuming replayable guarded
//! weakening/erasure evidence for the one-ambient-parameter identity family.

use crate::internal_classifier_branch_v2::{
    InductiveInternalityCertificate, RawCandidateDecisionV3, classify_raw_candidate_v3,
    replay_inductive_internality_json,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::SealedSignature;
use pen_type::guarded_internality::{
    GuardedClauseWeakeningErasureProjection, issue_guarded_clause_weakening_erasure_token,
    replay_guarded_clause_weakening_erasure_projection,
    replay_guarded_clause_weakening_erasure_token,
};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GUARDED_IDENTITY_INTERNALITY_SCHEMA: &str = "schema2-guarded-identity-internality-v1";
pub const GUARDED_IDENTITY_INTERNALITY_DATE: &str = "2026-07-20";

const PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/SEMANTIC_NORMALIZATION_PROGRAM.md");
const V3_RESULT_BYTES: &[u8] = include_bytes!("../../../docs/SCHEMA2_GLOBAL_E4_V3_RESULT.md");
const EXISTING_THEOREM_BYTES: &[u8] = include_bytes!("../../pen-eval/src/internality.rs");
const TOKEN_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/guarded_internality.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v2.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v3.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardedSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardedIdentityInternalCertificate {
    pub base_candidate: Telescope,
    pub guarded_candidate: Telescope,
    pub signature_digest: String,
    pub visible_library: u32,
    pub predecessor_decision: RawCandidateDecisionV3,
    pub base_decision: RawCandidateDecisionV3,
    pub clause_evidence: Vec<GuardedClauseWeakeningErasureProjection>,
    pub clause_indices_complete: bool,
    pub every_projection_replayed: bool,
    pub every_inserted_ambient_unused: bool,
    pub every_inverse_law_checked: bool,
    pub base_is_internal_identity: bool,
    pub guarded_identity_is_weakening_image: bool,
    pub internal_certificate_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RawCandidateDecisionV4 {
    NamedExclusion {
        predecessor_decision: RawCandidateDecisionV3,
    },
    InternalPredecessor {
        predecessor_decision: RawCandidateDecisionV3,
    },
    InternalGuarded {
        certificate: GuardedIdentityInternalCertificate,
        marginal_nu: u32,
        derivation_hash: String,
    },
    Classified {
        predecessor_decision: RawCandidateDecisionV3,
    },
    NamedTypedObstruction {
        predecessor_decision: RawCandidateDecisionV3,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardedIdentityForbiddenOutputs {
    pub global_e4_rerun_executed: bool,
    pub pending_membership_verdicts_issued: bool,
    pub e2b_executed: bool,
    pub stage_count_issued: bool,
    pub fq2_evaluated: bool,
    pub halt_or_continuation_claimed: bool,
}

impl GuardedIdentityForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.global_e4_rerun_executed
            && !self.pending_membership_verdicts_issued
            && !self.e2b_executed
            && !self.stage_count_issued
            && !self.fq2_evaluated
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardedIdentityInternalityCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GuardedSourceBinding>,
    pub predecessor_inductive_digest: String,
    pub predecessor_inductive_replayed: bool,
    pub existing_guarded_theorem_source_bound: bool,
    pub opaque_token_public_fields_exposed: bool,
    pub guarded_predecessor_was_unknown: bool,
    pub guarded_successor_decision: RawCandidateDecisionV4,
    pub guarded_now_internal: bool,
    pub clause_evidence_count: usize,
    pub all_clause_tokens_replayed: bool,
    pub both_inverse_laws_checked_per_clause: bool,
    pub marginal_nu: u32,
    pub global_e4_rerun_authorized: bool,
    pub forbidden_outputs: GuardedIdentityForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardedIdentityInternalityReplay {
    pub valid: bool,
    pub guarded_internal: bool,
    pub clause_evidence_count: usize,
    pub all_tokens_replayed: bool,
    pub both_inverse_laws_per_clause: bool,
    pub marginal_nu: u32,
    pub global_e4_rerun_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GuardedIdentityInternalityError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("token failed: {0}")]
    Token(String),
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
    let bytes = serde_json::to_vec(&(GUARDED_IDENTITY_INTERNALITY_SCHEMA, domain, value))
        .expect("guarded identity evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(predecessor_bytes: &[u8]) -> Vec<GuardedSourceBinding> {
    [
        (
            "docs/SEMANTIC_NORMALIZATION_PROGRAM.md",
            "guarded_internality_obligation",
            PROGRAM_BYTES,
        ),
        (
            "docs/SCHEMA2_GLOBAL_E4_V3_RESULT.md",
            "exact_one_ambient_identity_gap",
            V3_RESULT_BYTES,
        ),
        (
            "docs/schema2_inductive_internality_v1.json",
            "inductive_classifier_predecessor",
            predecessor_bytes,
        ),
        (
            "crates/pen-eval/src/internality.rs",
            "existing_guarded_weakening_erasure_theorem",
            EXISTING_THEOREM_BYTES,
        ),
        (
            "crates/pen-type/src/guarded_internality.rs",
            "opaque_per_clause_token_export",
            TOKEN_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v2.rs",
            "inductive_internal_predecessor",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v3.rs",
            "guarded_identity_classifier_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| GuardedSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

pub fn base_identity_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Lam(Box::new(Expr::Var(2)))),
    ])
}

pub fn one_ambient_identity_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Lam(Box::new(Expr::Var(3)))),
    ])
}

fn issue_guarded_identity_internal_certificate()
-> Result<GuardedIdentityInternalCertificate, GuardedIdentityInternalityError> {
    let signature = SealedSignature::genesis_del_h15();
    let base_candidate = base_identity_candidate();
    let guarded_candidate = one_ambient_identity_candidate();
    let predecessor_decision = classify_raw_candidate_v3(&guarded_candidate);
    if !matches!(
        predecessor_decision,
        RawCandidateDecisionV3::NamedTypedObstruction { .. }
    ) {
        return Err(GuardedIdentityInternalityError::Invariant(
            "guarded predecessor is not the registered Unknown".to_owned(),
        ));
    }
    let base_decision = classify_raw_candidate_v3(&base_candidate);
    let base_is_internal_identity = matches!(
        &base_decision,
        RawCandidateDecisionV3::Internal { certificate, .. }
            if certificate.clauses.get(1).is_some_and(|clause| {
                clause.identity_not_projection && clause.field_dependencies.is_empty()
            })
    );

    let mut clause_evidence = Vec::new();
    for clause_index in 0..u16::try_from(guarded_candidate.kappa()).expect("kappa fits u16") {
        let token = issue_guarded_clause_weakening_erasure_token(
            &signature,
            &base_candidate,
            &guarded_candidate,
            15,
            clause_index,
        )
        .map_err(|error| GuardedIdentityInternalityError::Token(error.to_string()))?;
        replay_guarded_clause_weakening_erasure_token(
            &signature,
            &base_candidate,
            &guarded_candidate,
            15,
            &token,
        )
        .map_err(|error| GuardedIdentityInternalityError::Token(error.to_string()))?;
        let projection = token.projection().clone();
        replay_guarded_clause_weakening_erasure_projection(
            &signature,
            &base_candidate,
            &guarded_candidate,
            15,
            &projection,
        )
        .map_err(|error| GuardedIdentityInternalityError::Token(error.to_string()))?;
        clause_evidence.push(projection);
    }
    let clause_indices_complete = clause_evidence
        .iter()
        .map(|evidence| evidence.clause_index)
        .eq(0..u16::try_from(guarded_candidate.kappa()).expect("kappa fits u16"));
    let every_projection_replayed = true;
    let every_inserted_ambient_unused = clause_evidence
        .iter()
        .all(|evidence| evidence.inserted_ambient_is_unused);
    let every_inverse_law_checked = clause_evidence
        .iter()
        .all(|evidence| evidence.both_inverse_laws_checked);
    let guarded_identity_is_weakening_image = clause_evidence.iter().all(|evidence| {
        evidence.weakening_matches_guarded.equal && evidence.erasure_matches_base.equal
    });
    let internal_certificate_issued = base_is_internal_identity
        && clause_indices_complete
        && every_projection_replayed
        && every_inserted_ambient_unused
        && every_inverse_law_checked
        && guarded_identity_is_weakening_image;
    let marginal_nu = 0;
    if !internal_certificate_issued {
        return Err(GuardedIdentityInternalityError::Invariant(
            "guarded identity certificate incomplete".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "guarded-identity-internal-certificate",
        &(
            &base_candidate,
            &guarded_candidate,
            signature.digest(),
            15u32,
            &predecessor_decision,
            &base_decision,
            &clause_evidence,
            clause_indices_complete,
            every_projection_replayed,
            every_inserted_ambient_unused,
            every_inverse_law_checked,
            base_is_internal_identity,
            guarded_identity_is_weakening_image,
            internal_certificate_issued,
            marginal_nu,
        ),
    );
    Ok(GuardedIdentityInternalCertificate {
        base_candidate,
        guarded_candidate,
        signature_digest: signature.digest().to_owned(),
        visible_library: 15,
        predecessor_decision,
        base_decision,
        clause_evidence,
        clause_indices_complete,
        every_projection_replayed,
        every_inserted_ambient_unused,
        every_inverse_law_checked,
        base_is_internal_identity,
        guarded_identity_is_weakening_image,
        internal_certificate_issued,
        marginal_nu,
        derivation_hash,
    })
}

pub fn classify_raw_candidate_v4(candidate: &Telescope) -> RawCandidateDecisionV4 {
    let predecessor_decision = classify_raw_candidate_v3(candidate);
    if candidate == &one_ambient_identity_candidate()
        && matches!(
            predecessor_decision,
            RawCandidateDecisionV3::NamedTypedObstruction { .. }
        )
    {
        let certificate = issue_guarded_identity_internal_certificate()
            .expect("registered guarded identity evidence must issue");
        let marginal_nu = certificate.marginal_nu;
        let derivation_hash = tagged_hash(
            "guarded-internal-classification",
            &(&certificate.derivation_hash, marginal_nu),
        );
        return RawCandidateDecisionV4::InternalGuarded {
            certificate,
            marginal_nu,
            derivation_hash,
        };
    }
    match predecessor_decision {
        decision @ RawCandidateDecisionV3::NamedExclusion { .. } => {
            RawCandidateDecisionV4::NamedExclusion {
                predecessor_decision: decision,
            }
        }
        decision @ RawCandidateDecisionV3::Internal { .. } => {
            RawCandidateDecisionV4::InternalPredecessor {
                predecessor_decision: decision,
            }
        }
        decision @ RawCandidateDecisionV3::Classified { .. } => {
            RawCandidateDecisionV4::Classified {
                predecessor_decision: decision,
            }
        }
        decision @ RawCandidateDecisionV3::NamedTypedObstruction { .. } => {
            RawCandidateDecisionV4::NamedTypedObstruction {
                predecessor_decision: decision,
            }
        }
    }
}

fn certificate_digest(certificate: &GuardedIdentityInternalityCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("guarded-identity-internality-certificate", &projection)
}

pub fn issue_guarded_identity_internality_certificate()
-> Result<GuardedIdentityInternalityCertificate, GuardedIdentityInternalityError> {
    let predecessor_bytes =
        std::fs::read(workspace_doc_path("schema2_inductive_internality_v1.json"))
            .map_err(|error| GuardedIdentityInternalityError::Io(error.to_string()))?;
    let predecessor_replay = replay_inductive_internality_json(
        std::str::from_utf8(&predecessor_bytes)
            .map_err(|error| GuardedIdentityInternalityError::Json(error.to_string()))?,
    );
    if !predecessor_replay.valid
        || !predecessor_replay.identity_control_internal
        || !predecessor_replay.identity_control_identity
    {
        return Err(GuardedIdentityInternalityError::Prerequisite(format!(
            "inductive predecessor replay failed: {}",
            predecessor_replay.errors.join("; ")
        )));
    }
    let predecessor: InductiveInternalityCertificate =
        serde_json::from_slice(&predecessor_bytes)
            .map_err(|error| GuardedIdentityInternalityError::Json(error.to_string()))?;

    let guarded_predecessor_was_unknown = matches!(
        classify_raw_candidate_v3(&one_ambient_identity_candidate()),
        RawCandidateDecisionV3::NamedTypedObstruction { .. }
    );
    let guarded_successor_decision = classify_raw_candidate_v4(&one_ambient_identity_candidate());
    let (
        guarded_now_internal,
        clause_evidence_count,
        all_clause_tokens_replayed,
        both_inverse_laws_checked_per_clause,
        marginal_nu,
    ) = match &guarded_successor_decision {
        RawCandidateDecisionV4::InternalGuarded {
            certificate,
            marginal_nu,
            ..
        } => (
            certificate.internal_certificate_issued,
            certificate.clause_evidence.len(),
            certificate.every_projection_replayed,
            certificate.every_inverse_law_checked,
            *marginal_nu,
        ),
        _ => (false, 0, false, false, u32::MAX),
    };
    let global_e4_rerun_authorized = true;
    let forbidden_outputs = GuardedIdentityForbiddenOutputs {
        global_e4_rerun_executed: false,
        pending_membership_verdicts_issued: false,
        e2b_executed: false,
        stage_count_issued: false,
        fq2_evaluated: false,
        halt_or_continuation_claimed: false,
    };
    if !guarded_predecessor_was_unknown
        || !guarded_now_internal
        || clause_evidence_count != 2
        || !all_clause_tokens_replayed
        || !both_inverse_laws_checked_per_clause
        || marginal_nu != 0
        || !forbidden_outputs.all_withheld()
    {
        return Err(GuardedIdentityInternalityError::Invariant(
            "guarded identity successor disposition drifted".to_owned(),
        ));
    }

    let mut certificate = GuardedIdentityInternalityCertificate {
        schema: GUARDED_IDENTITY_INTERNALITY_SCHEMA.to_owned(),
        date: GUARDED_IDENTITY_INTERNALITY_DATE.to_owned(),
        source_bindings: source_bindings(&predecessor_bytes),
        predecessor_inductive_digest: predecessor.result_digest,
        predecessor_inductive_replayed: true,
        existing_guarded_theorem_source_bound: true,
        opaque_token_public_fields_exposed: false,
        guarded_predecessor_was_unknown,
        guarded_successor_decision,
        guarded_now_internal,
        clause_evidence_count,
        all_clause_tokens_replayed,
        both_inverse_laws_checked_per_clause,
        marginal_nu,
        global_e4_rerun_authorized,
        forbidden_outputs,
        outcome: "guarded_one_ambient_identity_internal_with_two_replayed_inverse_tokens"
            .to_owned(),
        permitted_conclusion: "[Univ,Lam(Var(3))] is the one-ambient weakening image of the certified identity [Univ,Lam(Var(2))]. Both clauses carry replayed erasure-after-weakening and weakening-after-erasure witnesses, so the guarded candidate earns Internal with nu=0 and global E-4 may rerun create-new."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GuardedIdentityInternalityReplay {
    GuardedIdentityInternalityReplay {
        valid: false,
        guarded_internal: false,
        clause_evidence_count: 0,
        all_tokens_replayed: false,
        both_inverse_laws_per_clause: false,
        marginal_nu: u32::MAX,
        global_e4_rerun_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "guarded_identity_internality_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &GuardedIdentityInternalityCertificate,
    expected: &GuardedIdentityInternalityCertificate,
) -> GuardedIdentityInternalityReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GuardedIdentityInternalityReplay {
        valid: errors.is_empty(),
        guarded_internal: certificate.guarded_now_internal,
        clause_evidence_count: certificate.clause_evidence_count,
        all_tokens_replayed: certificate.all_clause_tokens_replayed,
        both_inverse_laws_per_clause: certificate.both_inverse_laws_checked_per_clause,
        marginal_nu: certificate.marginal_nu,
        global_e4_rerun_authorized: certificate.global_e4_rerun_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_guarded_identity_internality_certificate(
    certificate: &GuardedIdentityInternalityCertificate,
) -> GuardedIdentityInternalityReplay {
    let expected = match issue_guarded_identity_internality_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_guarded_identity_internality_json(json: &str) -> GuardedIdentityInternalityReplay {
    match serde_json::from_str::<GuardedIdentityInternalityCertificate>(json) {
        Ok(certificate) => replay_guarded_identity_internality_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_guarded_identity_internality_create_new(
    path: &Path,
) -> Result<GuardedIdentityInternalityReplay, GuardedIdentityInternalityError> {
    let certificate = issue_guarded_identity_internality_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GuardedIdentityInternalityError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GuardedIdentityInternalityError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GuardedIdentityInternalityError::Io(error.to_string()))?;
    let replay = replay_guarded_identity_internality_certificate(&certificate);
    if !replay.valid {
        return Err(GuardedIdentityInternalityError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_candidate_clauses_export_replayable_inverse_evidence() {
        let certificate = issue_guarded_identity_internal_certificate().expect("certificate");
        assert_eq!(certificate.clause_evidence.len(), 2);
        assert!(certificate.every_projection_replayed);
        assert!(certificate.every_inverse_law_checked);
        assert!(certificate.guarded_identity_is_weakening_image);
        assert!(certificate.internal_certificate_issued);
        assert_eq!(certificate.marginal_nu, 0);
    }

    #[test]
    fn guarded_candidate_moves_from_unknown_to_internal() {
        let candidate = one_ambient_identity_candidate();
        assert!(matches!(
            classify_raw_candidate_v3(&candidate),
            RawCandidateDecisionV3::NamedTypedObstruction { .. }
        ));
        assert!(matches!(
            classify_raw_candidate_v4(&candidate),
            RawCandidateDecisionV4::InternalGuarded { marginal_nu: 0, .. }
        ));
    }

    #[test]
    fn successor_replay_and_mutations_fail_closed() {
        let certificate = issue_guarded_identity_internality_certificate().expect("certificate");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut evidence = certificate.clone();
        evidence.both_inverse_laws_checked_per_clause = false;
        mutations.push(evidence);
        let mut internal = certificate.clone();
        internal.guarded_now_internal = false;
        mutations.push(internal);
        let mut token_seal = certificate.clone();
        token_seal.opaque_token_public_fields_exposed = true;
        mutations.push(token_seal);
        let mut forbidden = certificate.clone();
        forbidden.forbidden_outputs.e2b_executed = true;
        mutations.push(forbidden);
        for mutation in mutations {
            assert!(!replay_against_expected(&mutation, &certificate).valid);
        }
    }
}
