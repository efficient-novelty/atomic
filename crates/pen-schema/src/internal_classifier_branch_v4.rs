//! Versioned `Internal` classifier successor for structural lambda closure.
//!
//! A candidate clause may inherit `Internal` from the inductive predecessor,
//! or it may earn it by replaying the kernel lambda constructor over a body
//! that already has structural `Internal` evidence.  Constructor closure is
//! zero-credit and closed-only; it cannot discharge guarded, field-dependent,
//! application, or other unsupported clauses.

use crate::internal_classifier_branch_v3::{
    GuardedIdentityInternalityCertificate, RawCandidateDecisionV4, classify_raw_candidate_v4,
    replay_guarded_identity_internality_json,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::SealedSignature;
use pen_type::structural_internality::{
    StructuralInternalRule, StructuralLambdaClosureProjection,
    issue_structural_lambda_closure_token, replay_structural_lambda_closure_projection,
    replay_structural_lambda_closure_token,
};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const STRUCTURAL_LAMBDA_INTERNALITY_SCHEMA: &str = "schema2-structural-lambda-internality-v1";
pub const STRUCTURAL_LAMBDA_INTERNALITY_DATE: &str = "2026-07-20";

const V4_RESULT_BYTES: &[u8] = include_bytes!("../../../docs/SCHEMA2_GLOBAL_E4_V4_RESULT.md");
const TOKEN_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/structural_internality.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v3.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v4.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralLambdaSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InheritedInternalClause {
    pub clause_index: u16,
    pub predecessor_clause_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralLambdaInternalCertificate {
    pub candidate: Telescope,
    pub signature_digest: String,
    pub visible_library: u32,
    pub predecessor_decision: RawCandidateDecisionV4,
    pub inherited_internal_clauses: Vec<InheritedInternalClause>,
    pub constructor_evidence: Vec<StructuralLambdaClosureProjection>,
    pub inherited_clause_indices_complete: bool,
    pub constructor_clause_indices_complete: bool,
    pub every_clause_covered_exactly_once: bool,
    pub every_constructor_projection_replayed: bool,
    pub every_body_already_internal: bool,
    pub every_lambda_constructor_replayed: bool,
    pub closed_candidate: bool,
    pub internal_certificate_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RawCandidateDecisionV5 {
    NamedExclusion {
        predecessor_decision: RawCandidateDecisionV4,
    },
    InternalPredecessor {
        predecessor_decision: RawCandidateDecisionV4,
    },
    InternalStructuralLambda {
        certificate: StructuralLambdaInternalCertificate,
        marginal_nu: u32,
        derivation_hash: String,
    },
    Classified {
        predecessor_decision: RawCandidateDecisionV4,
    },
    NamedTypedObstruction {
        predecessor_decision: RawCandidateDecisionV4,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralLambdaForbiddenOutputs {
    pub global_e4_rerun_executed: bool,
    pub pending_membership_verdicts_issued: bool,
    pub e2b_executed: bool,
    pub stage_count_issued: bool,
    pub fq2_evaluated: bool,
    pub halt_or_continuation_claimed: bool,
}

impl StructuralLambdaForbiddenOutputs {
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
pub struct StructuralLambdaInternalityCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<StructuralLambdaSourceBinding>,
    pub predecessor_guarded_digest: String,
    pub predecessor_guarded_replayed: bool,
    pub global_e4_v4_obligation_source_bound: bool,
    pub opaque_constructor_token_public_fields_exposed: bool,
    pub structural_predecessor_was_unknown: bool,
    pub structural_successor_decision: RawCandidateDecisionV5,
    pub structural_now_internal: bool,
    pub inherited_internal_clause_count: usize,
    pub constructor_evidence_count: usize,
    pub constructor_clause_index: u16,
    pub body_rule: StructuralInternalRule,
    pub all_constructor_evidence_replayed: bool,
    pub marginal_nu: u32,
    pub global_e4_rerun_authorized: bool,
    pub forbidden_outputs: StructuralLambdaForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralLambdaInternalityReplay {
    pub valid: bool,
    pub structural_internal: bool,
    pub inherited_internal_clause_count: usize,
    pub constructor_evidence_count: usize,
    pub body_already_internal: bool,
    pub constructor_replayed: bool,
    pub marginal_nu: u32,
    pub global_e4_rerun_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum StructuralLambdaInternalityError {
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
    let bytes = serde_json::to_vec(&(STRUCTURAL_LAMBDA_INTERNALITY_SCHEMA, domain, value))
        .expect("structural lambda evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(guarded_bytes: &[u8]) -> Vec<StructuralLambdaSourceBinding> {
    [
        (
            "docs/SCHEMA2_GLOBAL_E4_V4_RESULT.md",
            "exact_structural_lambda_gap",
            V4_RESULT_BYTES,
        ),
        (
            "docs/schema2_guarded_identity_internality_v1.json",
            "classifier_predecessor_artifact",
            guarded_bytes,
        ),
        (
            "crates/pen-type/src/structural_internality.rs",
            "opaque_constructor_replay_token",
            TOKEN_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v3.rs",
            "guarded_internal_classifier_predecessor",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v4.rs",
            "zero_credit_structural_lambda_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| StructuralLambdaSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

pub fn structural_constant_universe_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Lam(Box::new(Expr::Univ))),
    ])
}

fn issue_structural_lambda_internal_certificate(
    candidate: &Telescope,
    predecessor_decision: RawCandidateDecisionV4,
) -> Result<StructuralLambdaInternalCertificate, StructuralLambdaInternalityError> {
    let RawCandidateDecisionV4::NamedTypedObstruction {
        predecessor_decision: inductive_predecessor,
    } = &predecessor_decision
    else {
        return Err(StructuralLambdaInternalityError::Constructor(
            "predecessor is not an unresolved typed candidate".to_owned(),
        ));
    };
    let crate::internal_classifier_branch_v2::RawCandidateDecisionV3::NamedTypedObstruction {
        internal_attempt,
        ..
    } = inductive_predecessor
    else {
        return Err(StructuralLambdaInternalityError::Constructor(
            "predecessor does not expose an inductive Internal attempt".to_owned(),
        ));
    };

    let signature = SealedSignature::genesis_del_h15();
    let mut inherited_internal_clauses = Vec::new();
    let mut constructor_evidence = Vec::new();
    for clause in &internal_attempt.clauses {
        if clause.earned_internal {
            inherited_internal_clauses.push(InheritedInternalClause {
                clause_index: clause.clause_index,
                predecessor_clause_derivation_hash: clause.derivation_hash.clone(),
            });
            continue;
        }
        let token =
            issue_structural_lambda_closure_token(&signature, candidate, 15, clause.clause_index)
                .map_err(|error| StructuralLambdaInternalityError::Constructor(error.to_string()))?;
        replay_structural_lambda_closure_token(&signature, candidate, 15, &token)
            .map_err(|error| StructuralLambdaInternalityError::Constructor(error.to_string()))?;
        let projection = token.projection().clone();
        replay_structural_lambda_closure_projection(&signature, candidate, 15, &projection)
            .map_err(|error| StructuralLambdaInternalityError::Constructor(error.to_string()))?;
        constructor_evidence.push(projection);
    }

    let inherited_clause_indices_complete = inherited_internal_clauses.iter().all(|record| {
        internal_attempt.clauses.iter().any(|clause| {
            clause.clause_index == record.clause_index
                && clause.earned_internal
                && clause.derivation_hash == record.predecessor_clause_derivation_hash
        })
    });
    let constructor_clause_indices_complete = constructor_evidence.iter().all(|evidence| {
        internal_attempt
            .clauses
            .iter()
            .any(|clause| clause.clause_index == evidence.clause_index && !clause.earned_internal)
    });
    let mut covered = inherited_internal_clauses
        .iter()
        .map(|record| record.clause_index)
        .chain(
            constructor_evidence
                .iter()
                .map(|record| record.clause_index),
        )
        .collect::<Vec<_>>();
    covered.sort_unstable();
    let every_clause_covered_exactly_once = covered
        == (0..u16::try_from(candidate.kappa()).expect("kappa fits u16")).collect::<Vec<_>>();
    let every_constructor_projection_replayed = !constructor_evidence.is_empty();
    let every_body_already_internal = constructor_evidence
        .iter()
        .all(|evidence| evidence.body_already_internal);
    let every_lambda_constructor_replayed = constructor_evidence
        .iter()
        .all(|evidence| evidence.lambda_constructor_replayed);
    let closed_candidate = constructor_evidence
        .iter()
        .all(|evidence| evidence.closed_candidate);
    let internal_certificate_issued = inherited_clause_indices_complete
        && constructor_clause_indices_complete
        && every_clause_covered_exactly_once
        && every_constructor_projection_replayed
        && every_body_already_internal
        && every_lambda_constructor_replayed
        && closed_candidate;
    let marginal_nu = 0;
    if !internal_certificate_issued {
        return Err(StructuralLambdaInternalityError::Invariant(
            "structural lambda clause coverage is incomplete".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "structural-lambda-internal-certificate",
        &(
            candidate,
            signature.digest(),
            15u32,
            &predecessor_decision,
            &inherited_internal_clauses,
            &constructor_evidence,
            inherited_clause_indices_complete,
            constructor_clause_indices_complete,
            every_clause_covered_exactly_once,
            every_constructor_projection_replayed,
            every_body_already_internal,
            every_lambda_constructor_replayed,
            closed_candidate,
            internal_certificate_issued,
            marginal_nu,
        ),
    );
    Ok(StructuralLambdaInternalCertificate {
        candidate: candidate.clone(),
        signature_digest: signature.digest().to_owned(),
        visible_library: 15,
        predecessor_decision,
        inherited_internal_clauses,
        constructor_evidence,
        inherited_clause_indices_complete,
        constructor_clause_indices_complete,
        every_clause_covered_exactly_once,
        every_constructor_projection_replayed,
        every_body_already_internal,
        every_lambda_constructor_replayed,
        closed_candidate,
        internal_certificate_issued,
        marginal_nu,
        derivation_hash,
    })
}

pub fn classify_raw_candidate_v5(candidate: &Telescope) -> RawCandidateDecisionV5 {
    let predecessor_decision = classify_raw_candidate_v4(candidate);
    if matches!(
        predecessor_decision,
        RawCandidateDecisionV4::NamedTypedObstruction { .. }
    ) {
        if let Ok(certificate) =
            issue_structural_lambda_internal_certificate(candidate, predecessor_decision.clone())
        {
            let marginal_nu = certificate.marginal_nu;
            let derivation_hash = tagged_hash(
                "structural-lambda-internal-classification",
                &(&certificate.derivation_hash, marginal_nu),
            );
            return RawCandidateDecisionV5::InternalStructuralLambda {
                certificate,
                marginal_nu,
                derivation_hash,
            };
        }
    }
    match predecessor_decision {
        decision @ RawCandidateDecisionV4::NamedExclusion { .. } => {
            RawCandidateDecisionV5::NamedExclusion {
                predecessor_decision: decision,
            }
        }
        decision @ (RawCandidateDecisionV4::InternalPredecessor { .. }
        | RawCandidateDecisionV4::InternalGuarded { .. }) => {
            RawCandidateDecisionV5::InternalPredecessor {
                predecessor_decision: decision,
            }
        }
        decision @ RawCandidateDecisionV4::Classified { .. } => {
            RawCandidateDecisionV5::Classified {
                predecessor_decision: decision,
            }
        }
        decision @ RawCandidateDecisionV4::NamedTypedObstruction { .. } => {
            RawCandidateDecisionV5::NamedTypedObstruction {
                predecessor_decision: decision,
            }
        }
    }
}

fn certificate_digest(certificate: &StructuralLambdaInternalityCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("structural-lambda-internality-certificate", &projection)
}

pub fn issue_structural_lambda_internality_certificate()
-> Result<StructuralLambdaInternalityCertificate, StructuralLambdaInternalityError> {
    let guarded_bytes = std::fs::read(workspace_doc_path(
        "schema2_guarded_identity_internality_v1.json",
    ))
    .map_err(|error| StructuralLambdaInternalityError::Io(error.to_string()))?;
    let guarded_replay = replay_guarded_identity_internality_json(
        std::str::from_utf8(&guarded_bytes)
            .map_err(|error| StructuralLambdaInternalityError::Json(error.to_string()))?,
    );
    if !guarded_replay.valid || !guarded_replay.guarded_internal {
        return Err(StructuralLambdaInternalityError::Prerequisite(format!(
            "guarded predecessor replay failed: {}",
            guarded_replay.errors.join("; ")
        )));
    }
    let guarded: GuardedIdentityInternalityCertificate = serde_json::from_slice(&guarded_bytes)
        .map_err(|error| StructuralLambdaInternalityError::Json(error.to_string()))?;

    let candidate = structural_constant_universe_candidate();
    let structural_predecessor_was_unknown = matches!(
        classify_raw_candidate_v4(&candidate),
        RawCandidateDecisionV4::NamedTypedObstruction { .. }
    );
    let structural_successor_decision = classify_raw_candidate_v5(&candidate);
    let (
        structural_now_internal,
        inherited_internal_clause_count,
        constructor_evidence_count,
        constructor_clause_index,
        body_rule,
        all_constructor_evidence_replayed,
        marginal_nu,
    ) = match &structural_successor_decision {
        RawCandidateDecisionV5::InternalStructuralLambda {
            certificate,
            marginal_nu,
            ..
        } => {
            let evidence = certificate.constructor_evidence.first().ok_or_else(|| {
                StructuralLambdaInternalityError::Invariant(
                    "constructor evidence is empty".to_owned(),
                )
            })?;
            (
                certificate.internal_certificate_issued,
                certificate.inherited_internal_clauses.len(),
                certificate.constructor_evidence.len(),
                evidence.clause_index,
                evidence.body_internal_evidence.rule.clone(),
                certificate.every_constructor_projection_replayed
                    && certificate.every_body_already_internal
                    && certificate.every_lambda_constructor_replayed,
                *marginal_nu,
            )
        }
        _ => (
            false,
            0,
            0,
            u16::MAX,
            StructuralInternalRule::LambdaIntroduction,
            false,
            u32::MAX,
        ),
    };
    let global_e4_rerun_authorized = true;
    let forbidden_outputs = StructuralLambdaForbiddenOutputs {
        global_e4_rerun_executed: false,
        pending_membership_verdicts_issued: false,
        e2b_executed: false,
        stage_count_issued: false,
        fq2_evaluated: false,
        halt_or_continuation_claimed: false,
    };
    if !structural_predecessor_was_unknown
        || !structural_now_internal
        || inherited_internal_clause_count != 1
        || constructor_evidence_count != 1
        || constructor_clause_index != 1
        || body_rule != StructuralInternalRule::AmbientUniverse
        || !all_constructor_evidence_replayed
        || marginal_nu != 0
        || !forbidden_outputs.all_withheld()
    {
        return Err(StructuralLambdaInternalityError::Invariant(
            "structural lambda successor disposition drifted".to_owned(),
        ));
    }

    let mut certificate = StructuralLambdaInternalityCertificate {
        schema: STRUCTURAL_LAMBDA_INTERNALITY_SCHEMA.to_owned(),
        date: STRUCTURAL_LAMBDA_INTERNALITY_DATE.to_owned(),
        source_bindings: source_bindings(&guarded_bytes),
        predecessor_guarded_digest: guarded.result_digest,
        predecessor_guarded_replayed: true,
        global_e4_v4_obligation_source_bound: true,
        opaque_constructor_token_public_fields_exposed: false,
        structural_predecessor_was_unknown,
        structural_successor_decision,
        structural_now_internal,
        inherited_internal_clause_count,
        constructor_evidence_count,
        constructor_clause_index,
        body_rule,
        all_constructor_evidence_replayed,
        marginal_nu,
        global_e4_rerun_authorized,
        forbidden_outputs,
        outcome: "constant_universe_lambda_internal_by_replayed_zero_credit_constructor"
            .to_owned(),
        permitted_conclusion: "[Univ,Lam(Univ)] earns Internal: clause 0 inherits its existing Internal certificate, the Univ body replays its structural base proof, and clause 1 replays lambda introduction over that body. The constructor adds nu=0 and authorizes global E-4 create-new."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> StructuralLambdaInternalityReplay {
    StructuralLambdaInternalityReplay {
        valid: false,
        structural_internal: false,
        inherited_internal_clause_count: 0,
        constructor_evidence_count: 0,
        body_already_internal: false,
        constructor_replayed: false,
        marginal_nu: u32::MAX,
        global_e4_rerun_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "structural_lambda_internality_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &StructuralLambdaInternalityCertificate,
    expected: &StructuralLambdaInternalityCertificate,
) -> StructuralLambdaInternalityReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    let (body_already_internal, constructor_replayed) =
        match &certificate.structural_successor_decision {
            RawCandidateDecisionV5::InternalStructuralLambda { certificate, .. } => (
                certificate.every_body_already_internal,
                certificate.every_lambda_constructor_replayed,
            ),
            _ => (false, false),
        };
    StructuralLambdaInternalityReplay {
        valid: errors.is_empty(),
        structural_internal: certificate.structural_now_internal,
        inherited_internal_clause_count: certificate.inherited_internal_clause_count,
        constructor_evidence_count: certificate.constructor_evidence_count,
        body_already_internal,
        constructor_replayed,
        marginal_nu: certificate.marginal_nu,
        global_e4_rerun_authorized: certificate.global_e4_rerun_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_structural_lambda_internality_certificate(
    certificate: &StructuralLambdaInternalityCertificate,
) -> StructuralLambdaInternalityReplay {
    let expected = match issue_structural_lambda_internality_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_structural_lambda_internality_json(json: &str) -> StructuralLambdaInternalityReplay {
    match serde_json::from_str::<StructuralLambdaInternalityCertificate>(json) {
        Ok(certificate) => replay_structural_lambda_internality_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_structural_lambda_internality_create_new(
    path: &Path,
) -> Result<StructuralLambdaInternalityReplay, StructuralLambdaInternalityError> {
    let certificate = issue_structural_lambda_internality_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| StructuralLambdaInternalityError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| StructuralLambdaInternalityError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| StructuralLambdaInternalityError::Io(error.to_string()))?;
    let replay = replay_structural_lambda_internality_certificate(&certificate);
    if !replay.valid {
        return Err(StructuralLambdaInternalityError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structural_constant_lambda_inherits_then_closes() {
        let candidate = structural_constant_universe_candidate();
        let decision = classify_raw_candidate_v5(&candidate);
        let RawCandidateDecisionV5::InternalStructuralLambda {
            certificate,
            marginal_nu,
            ..
        } = decision
        else {
            panic!("structural lambda must be Internal")
        };
        assert_eq!(certificate.inherited_internal_clauses.len(), 1);
        assert_eq!(certificate.inherited_internal_clauses[0].clause_index, 0);
        assert_eq!(certificate.constructor_evidence.len(), 1);
        assert_eq!(certificate.constructor_evidence[0].clause_index, 1);
        assert!(certificate.every_body_already_internal);
        assert!(certificate.every_lambda_constructor_replayed);
        assert_eq!(marginal_nu, 0);
    }

    #[test]
    fn nested_lambda_uses_the_same_recursive_constructor() {
        let candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::Lam(Box::new(Expr::Univ)))),
            ),
        ]);
        assert!(matches!(
            classify_raw_candidate_v5(&candidate),
            RawCandidateDecisionV5::InternalStructuralLambda { marginal_nu: 0, .. }
        ));
    }

    #[test]
    fn unsupported_application_body_does_not_enter_structural_internal() {
        let candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Lib(1)),
                    Box::new(Expr::Lib(2)),
                ))),
            ),
        ]);
        assert!(!matches!(
            classify_raw_candidate_v5(&candidate),
            RawCandidateDecisionV5::InternalStructuralLambda { .. }
        ));
    }

    #[test]
    fn certificate_replays_and_mutations_fail_closed() {
        let certificate = issue_structural_lambda_internality_certificate().expect("certificate");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut credit = certificate.clone();
        credit.marginal_nu = 1;
        mutations.push(credit);
        let mut replayed = certificate.clone();
        replayed.all_constructor_evidence_replayed = false;
        mutations.push(replayed);
        let mut inherited = certificate.clone();
        inherited.inherited_internal_clause_count = 0;
        mutations.push(inherited);
        let mut forbidden = certificate.clone();
        forbidden.forbidden_outputs.e2b_executed = true;
        mutations.push(forbidden);
        for mutation in mutations {
            assert!(!replay_against_expected(&mutation, &certificate).valid);
        }
    }
}
