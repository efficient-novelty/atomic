//! Versioned inductive-telescope successor to the first Internal classifier.
//!
//! Later clauses may use candidate-local fields only through an ordered
//! prefix of clauses that have already earned Internal certificates.  The
//! frozen distinction between field projection and local-binder identity is
//! retained in the proof object.

use crate::internal_classifier_branch::{
    ClauseDerivabilityKind, InternalBranchCertificate, InternalDerivabilityAttempt,
    RawCandidateDecisionV2, classify_raw_candidate_v2,
    correctly_levelled_identity_control_candidate, f_g4_witness_candidate,
    issue_internal_derivability_attempt, replay_internal_branch_json,
};
use crate::total_classifier::{RawCandidateDecision, classify_raw_candidate};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const INDUCTIVE_INTERNALITY_SCHEMA: &str = "schema2-inductive-telescope-internality-v1";
pub const INDUCTIVE_INTERNALITY_DATE: &str = "2026-07-20";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/inductive_telescope_internality_adjudication.md");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch.rs");
const ELABORATOR_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v2.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InductiveSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InductiveClauseKind {
    DirectAmbientArenaReference,
    DirectStructuralIdentity,
    DirectPredecessorConstantReference,
    InductiveConstantProjection,
    Unsupported,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InductiveClauseRecord {
    pub clause_index: u16,
    pub expression: Expr,
    pub root_rule: String,
    pub leaf_rules: Vec<String>,
    pub predecessor_kind: ClauseDerivabilityKind,
    pub kind: InductiveClauseKind,
    pub field_dependencies: Vec<u16>,
    pub certified_internal_prefix_before: Vec<u16>,
    pub dependency_certificate_hashes: Vec<String>,
    pub dependencies_are_strictly_prior: bool,
    pub every_dependency_already_internal: bool,
    pub projection_not_identity: bool,
    pub identity_not_projection: bool,
    pub guarded_inverse_laws_satisfied: bool,
    pub earned_internal: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InductiveInternalityAttempt {
    pub candidate: Telescope,
    pub predecessor_attempt_hash: String,
    pub predecessor_signature_digest: String,
    pub clauses: Vec<InductiveClauseRecord>,
    pub certified_internal_prefix: Vec<u16>,
    pub every_clause_earned_internal: bool,
    pub no_forward_missing_or_cyclic_dependency: bool,
    pub projection_identity_separation_checked: bool,
    pub all_guarded_inverse_laws_satisfied: bool,
    pub internal_certificate_issued: bool,
    pub failure_code: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RawCandidateDecisionV3 {
    NamedExclusion {
        legacy_decision: RawCandidateDecision,
    },
    Internal {
        certificate: InductiveInternalityAttempt,
        marginal_nu: u32,
        derivation_hash: String,
    },
    Classified {
        legacy_decision: RawCandidateDecision,
        internal_attempt: InductiveInternalityAttempt,
    },
    NamedTypedObstruction {
        legacy_decision: RawCandidateDecision,
        internal_attempt: InductiveInternalityAttempt,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InductiveForbiddenOutputs {
    pub global_e4_rerun_executed: bool,
    pub pending_membership_verdicts_issued: bool,
    pub e2b_executed: bool,
    pub stage_count_issued: bool,
    pub fq2_evaluated: bool,
    pub halt_or_continuation_claimed: bool,
}

impl InductiveForbiddenOutputs {
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
pub struct InductiveInternalityCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<InductiveSourceBinding>,
    pub predecessor_internal_branch_digest: String,
    pub predecessor_internal_branch_replayed: bool,
    pub predecessor_f_i2_result_preserved: bool,
    pub adopted_ordered_dependency_rule: bool,
    pub witness_predecessor_decision_was_unknown: bool,
    pub witness_successor_decision: RawCandidateDecisionV3,
    pub witness_now_internal: bool,
    pub witness_univ_earned_before_projection: bool,
    pub witness_lambda_is_constant_projection: bool,
    pub witness_lambda_is_not_identity: bool,
    pub identity_control_decision: RawCandidateDecisionV3,
    pub identity_control_is_internal: bool,
    pub identity_control_is_identity: bool,
    pub identity_control_is_not_projection: bool,
    pub f_it1_ordered_prefix_checked: bool,
    pub f_it3_projection_identity_separated: bool,
    pub global_e4_rerun_authorized: bool,
    pub forbidden_outputs: InductiveForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InductiveInternalityReplay {
    pub valid: bool,
    pub witness_internal: bool,
    pub witness_projection: bool,
    pub witness_not_identity: bool,
    pub identity_control_internal: bool,
    pub identity_control_identity: bool,
    pub ordered_prefix_checked: bool,
    pub global_e4_rerun_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum InductiveInternalityError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("derivability failed: {0}")]
    Derivability(String),
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
    let bytes = serde_json::to_vec(&(INDUCTIVE_INTERNALITY_SCHEMA, domain, value))
        .expect("inductive-internality evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(predecessor_bytes: &[u8]) -> Vec<InductiveSourceBinding> {
    [
        (
            "docs/inductive_telescope_internality_adjudication.md",
            "adopted_ordered_dependency_rule",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/schema2_internal_classifier_branch_v1.json",
            "archival_f_i2_predecessor",
            predecessor_bytes,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch.rs",
            "strict_direct_b15_predecessor",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/elaborate.rs",
            "frozen_field_and_local_resolution",
            ELABORATOR_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v2.rs",
            "inductive_telescope_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| InductiveSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn direct_kind(kind: &ClauseDerivabilityKind) -> Option<InductiveClauseKind> {
    match kind {
        ClauseDerivabilityKind::AmbientArenaReference => {
            Some(InductiveClauseKind::DirectAmbientArenaReference)
        }
        ClauseDerivabilityKind::StructuralIdentity => {
            Some(InductiveClauseKind::DirectStructuralIdentity)
        }
        ClauseDerivabilityKind::PredecessorConstantReference => {
            Some(InductiveClauseKind::DirectPredecessorConstantReference)
        }
        ClauseDerivabilityKind::CandidateLocalDependency
        | ClauseDerivabilityKind::UnsupportedStructuralDerivation => None,
    }
}

fn issue_from_predecessor_attempt(
    predecessor: InternalDerivabilityAttempt,
) -> InductiveInternalityAttempt {
    let mut certified_hashes = BTreeMap::<u16, String>::new();
    let mut clauses = Vec::with_capacity(predecessor.clauses.len());

    for clause in &predecessor.clauses {
        let certified_internal_prefix_before = certified_hashes.keys().copied().collect::<Vec<_>>();
        let dependencies_are_strictly_prior = clause
            .candidate_local_field_dependencies
            .iter()
            .all(|dependency| *dependency < clause.clause_index);
        let every_dependency_already_internal = clause
            .candidate_local_field_dependencies
            .iter()
            .all(|dependency| certified_hashes.contains_key(dependency));
        let dependency_certificate_hashes = clause
            .candidate_local_field_dependencies
            .iter()
            .filter_map(|dependency| certified_hashes.get(dependency).cloned())
            .collect::<Vec<_>>();
        let direct = direct_kind(&clause.kind);
        let is_field_only_lambda = clause.root_rule == "lam-intro"
            && !clause.leaf_rules.is_empty()
            && clause
                .leaf_rules
                .iter()
                .all(|rule| rule.starts_with("field-ref-"));
        let inductive_projection = direct.is_none()
            && matches!(
                clause.kind,
                ClauseDerivabilityKind::CandidateLocalDependency
            )
            && is_field_only_lambda
            && dependencies_are_strictly_prior
            && every_dependency_already_internal
            && !clause.guarded_form_applies;
        let kind = if let Some(kind) = direct {
            kind
        } else if inductive_projection {
            InductiveClauseKind::InductiveConstantProjection
        } else {
            InductiveClauseKind::Unsupported
        };
        let projection_not_identity =
            matches!(kind, InductiveClauseKind::InductiveConstantProjection)
                && clause
                    .leaf_rules
                    .iter()
                    .all(|rule| rule.starts_with("field-ref-"))
                && !clause
                    .leaf_rules
                    .iter()
                    .any(|rule| rule.starts_with("local-var-"));
        let identity_not_projection = matches!(kind, InductiveClauseKind::DirectStructuralIdentity)
            && clause.leaf_rules.iter().any(|rule| rule == "local-var-1")
            && clause.candidate_local_field_dependencies.is_empty();
        let guarded_inverse_laws_satisfied =
            !clause.guarded_form_applies || clause.weakening_erasure_inverse_laws_checked;
        let earned_internal = (clause.derived_over_exact_b15 || inductive_projection)
            && guarded_inverse_laws_satisfied;
        let derivation_hash = tagged_hash(
            "ordered-clause-internality",
            &(
                clause.clause_index,
                &clause.expression,
                &clause.root_rule,
                &clause.leaf_rules,
                &clause.kind,
                &kind,
                &clause.candidate_local_field_dependencies,
                &certified_internal_prefix_before,
                &dependency_certificate_hashes,
                dependencies_are_strictly_prior,
                every_dependency_already_internal,
                projection_not_identity,
                identity_not_projection,
                guarded_inverse_laws_satisfied,
                earned_internal,
            ),
        );
        let record = InductiveClauseRecord {
            clause_index: clause.clause_index,
            expression: clause.expression.clone(),
            root_rule: clause.root_rule.clone(),
            leaf_rules: clause.leaf_rules.clone(),
            predecessor_kind: clause.kind.clone(),
            kind,
            field_dependencies: clause.candidate_local_field_dependencies.clone(),
            certified_internal_prefix_before,
            dependency_certificate_hashes,
            dependencies_are_strictly_prior,
            every_dependency_already_internal,
            projection_not_identity,
            identity_not_projection,
            guarded_inverse_laws_satisfied,
            earned_internal,
            derivation_hash: derivation_hash.clone(),
        };
        if earned_internal {
            certified_hashes.insert(clause.clause_index, derivation_hash);
        }
        clauses.push(record);
    }

    let certified_internal_prefix = certified_hashes.keys().copied().collect::<Vec<_>>();
    let every_clause_earned_internal = clauses.iter().all(|clause| clause.earned_internal);
    let no_forward_missing_or_cyclic_dependency = clauses.iter().all(|clause| {
        clause.dependencies_are_strictly_prior && clause.every_dependency_already_internal
    });
    let projection_identity_separation_checked = clauses.iter().all(|clause| match clause.kind {
        InductiveClauseKind::InductiveConstantProjection => clause.projection_not_identity,
        InductiveClauseKind::DirectStructuralIdentity => clause.identity_not_projection,
        _ => true,
    });
    let all_guarded_inverse_laws_satisfied = clauses
        .iter()
        .all(|clause| clause.guarded_inverse_laws_satisfied);
    let internal_certificate_issued = every_clause_earned_internal
        && no_forward_missing_or_cyclic_dependency
        && projection_identity_separation_checked
        && all_guarded_inverse_laws_satisfied;
    let failure_code = (!internal_certificate_issued).then(|| {
        if clauses
            .iter()
            .any(|clause| !clause.guarded_inverse_laws_satisfied)
        {
            "INDUCTIVE_INTERNALITY_GUARDED_WEAKENING_ERASURE_INVERSE_LAWS_MISSING".to_owned()
        } else {
            "INDUCTIVE_TELESCOPE_INTERNALITY_NOT_PROVED_FOR_CANDIDATE".to_owned()
        }
    });
    let derivation_hash = tagged_hash(
        "inductive-telescope-internality",
        &(
            &predecessor.candidate,
            &predecessor.derivation_hash,
            &predecessor.signature_digest,
            &clauses,
            &certified_internal_prefix,
            every_clause_earned_internal,
            no_forward_missing_or_cyclic_dependency,
            projection_identity_separation_checked,
            all_guarded_inverse_laws_satisfied,
            internal_certificate_issued,
            &failure_code,
        ),
    );
    InductiveInternalityAttempt {
        candidate: predecessor.candidate,
        predecessor_attempt_hash: predecessor.derivation_hash,
        predecessor_signature_digest: predecessor.signature_digest,
        clauses,
        certified_internal_prefix,
        every_clause_earned_internal,
        no_forward_missing_or_cyclic_dependency,
        projection_identity_separation_checked,
        all_guarded_inverse_laws_satisfied,
        internal_certificate_issued,
        failure_code,
        derivation_hash,
    }
}

pub fn issue_inductive_internality_attempt(
    candidate: &Telescope,
) -> Result<InductiveInternalityAttempt, InductiveInternalityError> {
    let predecessor = issue_internal_derivability_attempt(candidate)
        .map_err(|error| InductiveInternalityError::Derivability(error.to_string()))?;
    Ok(issue_from_predecessor_attempt(predecessor))
}

pub fn classify_raw_candidate_v3(candidate: &Telescope) -> RawCandidateDecisionV3 {
    let legacy_decision = classify_raw_candidate(candidate);
    if matches!(legacy_decision, RawCandidateDecision::NamedExclusion { .. }) {
        return RawCandidateDecisionV3::NamedExclusion { legacy_decision };
    }
    let internal_attempt = issue_inductive_internality_attempt(candidate)
        .expect("legacy typed success must replay in the inductive successor");
    if internal_attempt.internal_certificate_issued {
        let derivation_hash = tagged_hash(
            "certified-inductive-internal-classification",
            &(&internal_attempt.derivation_hash, 0u32),
        );
        return RawCandidateDecisionV3::Internal {
            certificate: internal_attempt,
            marginal_nu: 0,
            derivation_hash,
        };
    }
    match legacy_decision {
        decision @ RawCandidateDecision::Classified { .. } => RawCandidateDecisionV3::Classified {
            legacy_decision: decision,
            internal_attempt,
        },
        decision @ RawCandidateDecision::NamedTypedObstruction { .. } => {
            RawCandidateDecisionV3::NamedTypedObstruction {
                legacy_decision: decision,
                internal_attempt,
            }
        }
        RawCandidateDecision::NamedExclusion { .. } => unreachable!("returned above"),
    }
}

fn internal_certificate(decision: &RawCandidateDecisionV3) -> Option<&InductiveInternalityAttempt> {
    match decision {
        RawCandidateDecisionV3::Internal { certificate, .. } => Some(certificate),
        _ => None,
    }
}

fn certificate_digest(certificate: &InductiveInternalityCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("inductive-internality-certificate", &projection)
}

pub fn issue_inductive_internality_certificate()
-> Result<InductiveInternalityCertificate, InductiveInternalityError> {
    let predecessor_bytes = std::fs::read(workspace_doc_path(
        "schema2_internal_classifier_branch_v1.json",
    ))
    .map_err(|error| InductiveInternalityError::Io(error.to_string()))?;
    let predecessor_replay = replay_internal_branch_json(
        std::str::from_utf8(&predecessor_bytes)
            .map_err(|error| InductiveInternalityError::Json(error.to_string()))?,
    );
    if !predecessor_replay.valid
        || !predecessor_replay.f_i2_triggered
        || !predecessor_replay.unknown_retained
    {
        return Err(InductiveInternalityError::Prerequisite(format!(
            "strict Internal predecessor failed: {}",
            predecessor_replay.errors.join("; ")
        )));
    }
    let predecessor: InternalBranchCertificate = serde_json::from_slice(&predecessor_bytes)
        .map_err(|error| InductiveInternalityError::Json(error.to_string()))?;
    let predecessor_decision = classify_raw_candidate_v2(&f_g4_witness_candidate());
    let witness_predecessor_decision_was_unknown = matches!(
        predecessor_decision,
        RawCandidateDecisionV2::NamedTypedObstruction { .. }
    );

    let witness_successor_decision = classify_raw_candidate_v3(&f_g4_witness_candidate());
    let witness_certificate =
        internal_certificate(&witness_successor_decision).ok_or_else(|| {
            InductiveInternalityError::Invariant(
                "witness did not enter inductive Internal".to_owned(),
            )
        })?;
    let witness_now_internal = true;
    let witness_univ_earned_before_projection = witness_certificate
        .clauses
        .first()
        .is_some_and(|clause| clause.earned_internal)
        && witness_certificate
            .clauses
            .get(1)
            .is_some_and(|clause| clause.certified_internal_prefix_before == vec![0]);
    let witness_lambda = witness_certificate
        .clauses
        .get(1)
        .ok_or_else(|| InductiveInternalityError::Invariant("witness lambda missing".to_owned()))?;
    let witness_lambda_is_constant_projection = matches!(
        witness_lambda.kind,
        InductiveClauseKind::InductiveConstantProjection
    ) && witness_lambda.field_dependencies == vec![0]
        && witness_lambda.dependency_certificate_hashes.len() == 1;
    let witness_lambda_is_not_identity = witness_lambda.projection_not_identity;

    let identity_control_decision =
        classify_raw_candidate_v3(&correctly_levelled_identity_control_candidate());
    let identity_certificate =
        internal_certificate(&identity_control_decision).ok_or_else(|| {
            InductiveInternalityError::Invariant("identity control not Internal".to_owned())
        })?;
    let identity_control_is_internal = true;
    let identity_clause = identity_certificate.clauses.get(1).ok_or_else(|| {
        InductiveInternalityError::Invariant("identity clause missing".to_owned())
    })?;
    let identity_control_is_identity = matches!(
        identity_clause.kind,
        InductiveClauseKind::DirectStructuralIdentity
    ) && identity_clause.identity_not_projection;
    let identity_control_is_not_projection = identity_clause.field_dependencies.is_empty()
        && !matches!(
            identity_clause.kind,
            InductiveClauseKind::InductiveConstantProjection
        );
    let f_it1_ordered_prefix_checked = witness_certificate.no_forward_missing_or_cyclic_dependency;
    let f_it3_projection_identity_separated = witness_certificate
        .projection_identity_separation_checked
        && identity_certificate.projection_identity_separation_checked
        && witness_lambda_is_not_identity
        && identity_control_is_not_projection;
    let global_e4_rerun_authorized = true;
    let forbidden_outputs = InductiveForbiddenOutputs {
        global_e4_rerun_executed: false,
        pending_membership_verdicts_issued: false,
        e2b_executed: false,
        stage_count_issued: false,
        fq2_evaluated: false,
        halt_or_continuation_claimed: false,
    };
    if !witness_predecessor_decision_was_unknown
        || !witness_now_internal
        || !witness_univ_earned_before_projection
        || !witness_lambda_is_constant_projection
        || !witness_lambda_is_not_identity
        || !identity_control_is_internal
        || !identity_control_is_identity
        || !identity_control_is_not_projection
        || !f_it1_ordered_prefix_checked
        || !f_it3_projection_identity_separated
        || !forbidden_outputs.all_withheld()
    {
        return Err(InductiveInternalityError::Invariant(
            "inductive witness/identity separation drifted".to_owned(),
        ));
    }

    let mut certificate = InductiveInternalityCertificate {
        schema: INDUCTIVE_INTERNALITY_SCHEMA.to_owned(),
        date: INDUCTIVE_INTERNALITY_DATE.to_owned(),
        source_bindings: source_bindings(&predecessor_bytes),
        predecessor_internal_branch_digest: predecessor.result_digest,
        predecessor_internal_branch_replayed: true,
        predecessor_f_i2_result_preserved: true,
        adopted_ordered_dependency_rule: true,
        witness_predecessor_decision_was_unknown,
        witness_successor_decision,
        witness_now_internal,
        witness_univ_earned_before_projection,
        witness_lambda_is_constant_projection,
        witness_lambda_is_not_identity,
        identity_control_decision,
        identity_control_is_internal,
        identity_control_is_identity,
        identity_control_is_not_projection,
        f_it1_ordered_prefix_checked,
        f_it3_projection_identity_separated,
        global_e4_rerun_authorized,
        forbidden_outputs,
        outcome: "inductive_telescope_internality_witness_internal_as_projection_identity_control_distinct"
            .to_owned(),
        permitted_conclusion: "The exact witness now earns Internal left-to-right: Univ first, then Lam(Var(1)) as a constant projection through certified field 0. Lam(Var(2)) independently remains structural identity. This resolves the predecessor witness only and authorizes a create-new global E-4 rerun."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> InductiveInternalityReplay {
    InductiveInternalityReplay {
        valid: false,
        witness_internal: false,
        witness_projection: false,
        witness_not_identity: false,
        identity_control_internal: false,
        identity_control_identity: false,
        ordered_prefix_checked: false,
        global_e4_rerun_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "inductive_internality_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &InductiveInternalityCertificate,
    expected: &InductiveInternalityCertificate,
) -> InductiveInternalityReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    InductiveInternalityReplay {
        valid: errors.is_empty(),
        witness_internal: certificate.witness_now_internal,
        witness_projection: certificate.witness_lambda_is_constant_projection,
        witness_not_identity: certificate.witness_lambda_is_not_identity,
        identity_control_internal: certificate.identity_control_is_internal,
        identity_control_identity: certificate.identity_control_is_identity,
        ordered_prefix_checked: certificate.f_it1_ordered_prefix_checked,
        global_e4_rerun_authorized: certificate.global_e4_rerun_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_inductive_internality_certificate(
    certificate: &InductiveInternalityCertificate,
) -> InductiveInternalityReplay {
    let expected = match issue_inductive_internality_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_inductive_internality_json(json: &str) -> InductiveInternalityReplay {
    match serde_json::from_str::<InductiveInternalityCertificate>(json) {
        Ok(certificate) => replay_inductive_internality_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_inductive_internality_create_new(
    path: &Path,
) -> Result<InductiveInternalityReplay, InductiveInternalityError> {
    let certificate = issue_inductive_internality_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| InductiveInternalityError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| InductiveInternalityError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| InductiveInternalityError::Io(error.to_string()))?;
    let replay = replay_inductive_internality_certificate(&certificate);
    if !replay.valid {
        return Err(InductiveInternalityError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn witness_projection_waits_for_clause_zero_internality() {
        let attempt = issue_inductive_internality_attempt(&f_g4_witness_candidate())
            .expect("inductive attempt");
        assert!(attempt.internal_certificate_issued);
        assert_eq!(attempt.certified_internal_prefix, vec![0, 1]);
        let projection = &attempt.clauses[1];
        assert_eq!(projection.certified_internal_prefix_before, vec![0]);
        assert_eq!(projection.field_dependencies, vec![0]);
        assert_eq!(projection.dependency_certificate_hashes.len(), 1);
        assert!(projection.every_dependency_already_internal);
        assert!(projection.projection_not_identity);
    }

    #[test]
    fn identity_control_remains_identity_not_projection() {
        let attempt =
            issue_inductive_internality_attempt(&correctly_levelled_identity_control_candidate())
                .expect("identity attempt");
        let identity = &attempt.clauses[1];
        assert!(matches!(
            identity.kind,
            InductiveClauseKind::DirectStructuralIdentity
        ));
        assert!(identity.identity_not_projection);
        assert!(identity.field_dependencies.is_empty());
    }

    #[test]
    fn successor_certificate_replays_and_mutations_fail_closed() {
        let certificate = issue_inductive_internality_certificate().expect("certificate");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut order = certificate.clone();
        order.f_it1_ordered_prefix_checked = false;
        mutations.push(order);
        let mut identity = certificate.clone();
        identity.witness_lambda_is_not_identity = false;
        mutations.push(identity);
        let mut projection = certificate.clone();
        projection.witness_lambda_is_constant_projection = false;
        mutations.push(projection);
        let mut forbidden = certificate.clone();
        forbidden.forbidden_outputs.e2b_executed = true;
        mutations.push(forbidden);
        for mutation in mutations {
            assert!(!replay_against_expected(&mutation, &certificate).valid);
        }
    }
}
