//! Versioned successor to the G-8 classifier implementing the adopted
//! certificate-backed Internal branch.
//!
//! The branch is deliberately proof-first.  A successful elaboration is not
//! by itself an internality proof: every clause must be shown to use only the
//! exact predecessor closure and structural rules.  Candidate-local field
//! dependencies therefore fail closed.

use crate::total_classifier::{
    F_G4_TYPED_UNKNOWN, RawCandidateDecision, TotalClassifierCertificate, classify_raw_candidate,
    replay_total_classifier_json,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::elaborate::{
    ClauseElaboration, DerivationNode, SealedSignature, elaborate_telescope,
};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const INTERNAL_BRANCH_SCHEMA: &str = "schema2-internal-derivability-classifier-branch-v1";
pub const INTERNAL_BRANCH_DATE: &str = "2026-07-20";
pub const F_I2_WITNESS_NOT_DERIVABLE_OVER_B15: &str =
    "F_I2_WITNESS_LAMBDA_DEPENDS_ON_CANDIDATE_LOCAL_FIELD_NOT_B15";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/internal_classifier_branch_adjudication.md");
const PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/SEMANTIC_NORMALIZATION_PROGRAM.md");
const LEGACY_CLASSIFIER_SOURCE_BYTES: &[u8] = include_bytes!("total_classifier.rs");
const ELABORATOR_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");
const NORMALIZER_BYTES: &[u8] = include_bytes!("../../pen-type/src/normalize.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InternalBranchSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClauseDerivabilityKind {
    AmbientArenaReference,
    StructuralIdentity,
    PredecessorConstantReference,
    CandidateLocalDependency,
    UnsupportedStructuralDerivation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClauseDerivabilityRecord {
    pub clause_index: u16,
    pub expression: Expr,
    pub normal_form: Expr,
    pub declared_role: ClauseRole,
    pub kernel_role: ClauseRole,
    pub root_rule: String,
    pub leaf_rules: Vec<String>,
    pub kind: ClauseDerivabilityKind,
    pub candidate_local_field_dependencies: Vec<u16>,
    pub predecessor_library_dependencies: Vec<u32>,
    pub derived_over_exact_b15: bool,
    pub guarded_form_applies: bool,
    pub weakening_erasure_inverse_laws_checked: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InternalDerivabilityAttempt {
    pub candidate: Telescope,
    pub candidate_digest: String,
    pub predecessor_context: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub ambient_parameters: u32,
    pub elaboration_derivation_hash: String,
    pub clauses: Vec<ClauseDerivabilityRecord>,
    pub every_clause_derived_over_exact_b15: bool,
    pub all_applicable_weakening_erasure_inverse_laws_checked: bool,
    pub internal_certificate_issued: bool,
    pub failure_code: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RawCandidateDecisionV2 {
    NamedExclusion {
        legacy_decision: RawCandidateDecision,
    },
    Internal {
        certificate: InternalDerivabilityAttempt,
        marginal_nu: u32,
        derivation_hash: String,
    },
    Classified {
        legacy_decision: RawCandidateDecision,
        internal_attempt: InternalDerivabilityAttempt,
    },
    NamedTypedObstruction {
        legacy_decision: RawCandidateDecision,
        internal_attempt: InternalDerivabilityAttempt,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InternalBranchForbiddenOutputs {
    pub global_e4_successor_rerun_executed: bool,
    pub pending_membership_verdicts_issued: bool,
    pub e2b_executed: bool,
    pub stage_count_issued: bool,
    pub fq2_evaluated: bool,
    pub halt_or_continuation_claimed: bool,
}

impl InternalBranchForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.global_e4_successor_rerun_executed
            && !self.pending_membership_verdicts_issued
            && !self.e2b_executed
            && !self.stage_count_issued
            && !self.fq2_evaluated
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InternalBranchCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<InternalBranchSourceBinding>,
    pub predecessor_g8_digest: String,
    pub predecessor_g8_replayed: bool,
    pub adjudication_adopted: bool,
    pub classifier_priority_order: Vec<String>,
    pub witness_decision: RawCandidateDecisionV2,
    pub witness_first_obligation_executed: bool,
    pub witness_univ_earned_as_arena_reference: bool,
    pub witness_lambda_resolved_to_candidate_field_zero: bool,
    pub witness_lambda_resolved_to_local_binder: bool,
    pub witness_internal_certificate_issued: bool,
    pub f_i1_no_uncertified_internal_classification: bool,
    pub f_i2_triggered: bool,
    pub unknown_retained: bool,
    pub correctly_levelled_identity_control_classifies_internal: bool,
    pub global_e4_successor_rerun_authorized: bool,
    pub forbidden_outputs: InternalBranchForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct InternalBranchReplay {
    pub valid: bool,
    pub witness_obligation_executed: bool,
    pub witness_internal: bool,
    pub f_i2_triggered: bool,
    pub unknown_retained: bool,
    pub identity_control_internal: bool,
    pub global_e4_rerun_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum InternalBranchError {
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
    let bytes = serde_json::to_vec(&(INTERNAL_BRANCH_SCHEMA, domain, value))
        .expect("Internal-branch evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(g8_bytes: &[u8]) -> Vec<InternalBranchSourceBinding> {
    [
        (
            "docs/internal_classifier_branch_adjudication.md",
            "adopted_internal_branch_rule",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/SEMANTIC_NORMALIZATION_PROGRAM.md",
            "preregistered_phase4_partition",
            PROGRAM_BYTES,
        ),
        (
            "docs/schema2_g8_total_classifier_v1.json",
            "versioned_classifier_predecessor",
            g8_bytes,
        ),
        (
            "crates/pen-schema/src/total_classifier.rs",
            "legacy_exclusion_and_class_branches",
            LEGACY_CLASSIFIER_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/elaborate.rs",
            "exact_b15_typed_derivation",
            ELABORATOR_BYTES,
        ),
        (
            "crates/pen-type/src/normalize.rs",
            "frozen_absolute_level_normalization",
            NORMALIZER_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch.rs",
            "certificate_backed_internal_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| InternalBranchSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

pub fn f_g4_witness_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Lam(Box::new(Expr::Var(1)))),
    ])
}

pub fn correctly_levelled_identity_control_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Lam(Box::new(Expr::Var(2)))),
    ])
}

fn collect_leaf_rules(node: &DerivationNode, leaves: &mut Vec<String>) {
    if node.children.is_empty() {
        leaves.push(node.rule.clone());
    } else {
        for child in &node.children {
            collect_leaf_rules(child, leaves);
        }
    }
}

fn parse_rule_index(rule: &str, prefix: &str) -> Option<u16> {
    rule.strip_prefix(prefix)?.parse().ok()
}

fn clause_derivability(
    candidate: &Telescope,
    elaboration: &ClauseElaboration,
    ambient_parameters: u32,
) -> ClauseDerivabilityRecord {
    let expression = candidate.clauses[usize::from(elaboration.clause_index)]
        .expr
        .clone();
    let mut leaf_rules = Vec::new();
    collect_leaf_rules(&elaboration.derivation, &mut leaf_rules);
    let candidate_local_field_dependencies = leaf_rules
        .iter()
        .filter_map(|rule| parse_rule_index(rule, "field-ref-"))
        .collect::<Vec<_>>();
    let predecessor_library_dependencies = expression.lib_refs().into_iter().collect::<Vec<_>>();
    let expected_identity_level = ambient_parameters + u32::from(elaboration.clause_index) + 1;
    let structural_identity = matches!(
        &expression,
        Expr::Lam(body)
            if matches!(body.as_ref(), Expr::Var(level) if *level == expected_identity_level)
                && elaboration.derivation.rule == "lam-intro"
                && leaf_rules.iter().any(|rule| rule == "local-var-1")
                && candidate_local_field_dependencies.is_empty()
    );
    let arena_reference = expression == Expr::Univ
        && elaboration.derivation.rule == "univ-form"
        && candidate_local_field_dependencies.is_empty();
    let predecessor_reference = matches!(expression, Expr::Lib(step) if (1..=15).contains(&step))
        && candidate_local_field_dependencies.is_empty();
    let kind = if !candidate_local_field_dependencies.is_empty() {
        ClauseDerivabilityKind::CandidateLocalDependency
    } else if arena_reference {
        ClauseDerivabilityKind::AmbientArenaReference
    } else if structural_identity {
        ClauseDerivabilityKind::StructuralIdentity
    } else if predecessor_reference {
        ClauseDerivabilityKind::PredecessorConstantReference
    } else {
        ClauseDerivabilityKind::UnsupportedStructuralDerivation
    };
    let guarded_form_applies = ambient_parameters > 0;
    // This successor proves only the closed structural fragment.  A guarded
    // open term must carry the pre-existing weakening/erasure tokens before
    // it can enter Internal, so it remains unsupported here.
    let weakening_erasure_inverse_laws_checked = !guarded_form_applies;
    let derived_over_exact_b15 = matches!(
        kind,
        ClauseDerivabilityKind::AmbientArenaReference
            | ClauseDerivabilityKind::StructuralIdentity
            | ClauseDerivabilityKind::PredecessorConstantReference
    ) && weakening_erasure_inverse_laws_checked;
    let derivation_hash = tagged_hash(
        "clause-derivability",
        &(
            elaboration.clause_index,
            &expression,
            &elaboration.normal_form,
            elaboration.declared_role,
            elaboration.kernel_role,
            &elaboration.derivation.rule,
            &leaf_rules,
            &kind,
            &candidate_local_field_dependencies,
            &predecessor_library_dependencies,
            derived_over_exact_b15,
            guarded_form_applies,
            weakening_erasure_inverse_laws_checked,
        ),
    );
    ClauseDerivabilityRecord {
        clause_index: elaboration.clause_index,
        expression,
        normal_form: elaboration.normal_form.clone(),
        declared_role: elaboration.declared_role,
        kernel_role: elaboration.kernel_role,
        root_rule: elaboration.derivation.rule.clone(),
        leaf_rules,
        kind,
        candidate_local_field_dependencies,
        predecessor_library_dependencies,
        derived_over_exact_b15,
        guarded_form_applies,
        weakening_erasure_inverse_laws_checked,
        derivation_hash,
    }
}

pub fn issue_internal_derivability_attempt(
    candidate: &Telescope,
) -> Result<InternalDerivabilityAttempt, InternalBranchError> {
    let signature = SealedSignature::genesis_del_h15();
    let elaboration = elaborate_telescope(&signature, candidate, 15)
        .map_err(|error| InternalBranchError::Derivability(error.to_string()))?;
    let clauses = elaboration
        .clauses
        .iter()
        .map(|clause| clause_derivability(candidate, clause, elaboration.ambient_parameters))
        .collect::<Vec<_>>();
    let every_clause_derived_over_exact_b15 =
        clauses.iter().all(|clause| clause.derived_over_exact_b15);
    let all_applicable_weakening_erasure_inverse_laws_checked = clauses.iter().all(|clause| {
        !clause.guarded_form_applies || clause.weakening_erasure_inverse_laws_checked
    });
    let internal_certificate_issued = every_clause_derived_over_exact_b15
        && all_applicable_weakening_erasure_inverse_laws_checked;
    let failure_code = (!internal_certificate_issued).then(|| {
        if clauses.iter().any(|clause| {
            matches!(
                clause.kind,
                ClauseDerivabilityKind::CandidateLocalDependency
            )
        }) {
            F_I2_WITNESS_NOT_DERIVABLE_OVER_B15.to_owned()
        } else {
            "INTERNAL_DERIVABILITY_CERTIFICATE_NOT_PROVED_FOR_CANDIDATE".to_owned()
        }
    });
    let candidate_digest = tagged_hash("internal-branch-candidate", candidate);
    let predecessor_context = "B15: exact sealed Genesis predecessor closure through Step 15";
    let derivation_hash = tagged_hash(
        "internal-derivability-attempt",
        &(
            candidate,
            &candidate_digest,
            predecessor_context,
            signature.digest(),
            15u32,
            elaboration.ambient_parameters,
            &elaboration.derivation_hash,
            &clauses,
            every_clause_derived_over_exact_b15,
            all_applicable_weakening_erasure_inverse_laws_checked,
            internal_certificate_issued,
            &failure_code,
        ),
    );
    Ok(InternalDerivabilityAttempt {
        candidate: candidate.clone(),
        candidate_digest,
        predecessor_context: predecessor_context.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library: 15,
        ambient_parameters: elaboration.ambient_parameters,
        elaboration_derivation_hash: elaboration.derivation_hash,
        clauses,
        every_clause_derived_over_exact_b15,
        all_applicable_weakening_erasure_inverse_laws_checked,
        internal_certificate_issued,
        failure_code,
        derivation_hash,
    })
}

/// Classifier successor with the adopted priority order:
/// exclusion -> certificate-backed Internal -> adopted class -> F-G4.
pub fn classify_raw_candidate_v2(candidate: &Telescope) -> RawCandidateDecisionV2 {
    let legacy_decision = classify_raw_candidate(candidate);
    if matches!(legacy_decision, RawCandidateDecision::NamedExclusion { .. }) {
        return RawCandidateDecisionV2::NamedExclusion { legacy_decision };
    }
    let internal_attempt = issue_internal_derivability_attempt(candidate)
        .expect("legacy typed success must replay in the Internal successor");
    if internal_attempt.internal_certificate_issued {
        let derivation_hash = tagged_hash(
            "certified-internal-classification",
            &(&internal_attempt.derivation_hash, 0u32),
        );
        return RawCandidateDecisionV2::Internal {
            certificate: internal_attempt,
            marginal_nu: 0,
            derivation_hash,
        };
    }
    match legacy_decision {
        decision @ RawCandidateDecision::Classified { .. } => RawCandidateDecisionV2::Classified {
            legacy_decision: decision,
            internal_attempt,
        },
        decision @ RawCandidateDecision::NamedTypedObstruction { .. } => {
            RawCandidateDecisionV2::NamedTypedObstruction {
                legacy_decision: decision,
                internal_attempt,
            }
        }
        RawCandidateDecision::NamedExclusion { .. } => unreachable!("returned above"),
    }
}

fn certificate_digest(certificate: &InternalBranchCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("internal-branch-certificate", &projection)
}

pub fn issue_internal_branch_certificate() -> Result<InternalBranchCertificate, InternalBranchError>
{
    let g8_bytes = std::fs::read(workspace_doc_path("schema2_g8_total_classifier_v1.json"))
        .map_err(|error| InternalBranchError::Io(error.to_string()))?;
    let g8_replay = replay_total_classifier_json(
        std::str::from_utf8(&g8_bytes)
            .map_err(|error| InternalBranchError::Json(error.to_string()))?,
    );
    if !g8_replay.valid || !g8_replay.global_e4_assembly_authorized {
        return Err(InternalBranchError::Prerequisite(format!(
            "G8 predecessor replay failed: {}",
            g8_replay.errors.join("; ")
        )));
    }
    let g8: TotalClassifierCertificate = serde_json::from_slice(&g8_bytes)
        .map_err(|error| InternalBranchError::Json(error.to_string()))?;

    let witness_decision = classify_raw_candidate_v2(&f_g4_witness_candidate());
    let (witness_attempt, unknown_retained) = match &witness_decision {
        RawCandidateDecisionV2::NamedTypedObstruction {
            legacy_decision,
            internal_attempt,
        } => (
            internal_attempt,
            matches!(
                legacy_decision,
                RawCandidateDecision::NamedTypedObstruction { code, .. }
                    if code == F_G4_TYPED_UNKNOWN
            ),
        ),
        _ => {
            return Err(InternalBranchError::Invariant(
                "exact witness did not remain in the named obstruction branch".to_owned(),
            ));
        }
    };
    let witness_univ_earned_as_arena_reference =
        witness_attempt.clauses.first().is_some_and(|clause| {
            clause.derived_over_exact_b15
                && matches!(clause.kind, ClauseDerivabilityKind::AmbientArenaReference)
        });
    let witness_lambda = witness_attempt.clauses.get(1).ok_or_else(|| {
        InternalBranchError::Invariant("witness lambda clause missing".to_owned())
    })?;
    let witness_lambda_resolved_to_candidate_field_zero =
        witness_lambda.candidate_local_field_dependencies == vec![0];
    let witness_lambda_resolved_to_local_binder = witness_lambda
        .leaf_rules
        .iter()
        .any(|rule| rule == "local-var-1");
    let witness_internal_certificate_issued = witness_attempt.internal_certificate_issued;
    let f_i1_no_uncertified_internal_classification =
        !matches!(witness_decision, RawCandidateDecisionV2::Internal { .. });
    let f_i2_triggered = !witness_internal_certificate_issued
        && witness_attempt.failure_code.as_deref() == Some(F_I2_WITNESS_NOT_DERIVABLE_OVER_B15);
    let correctly_levelled_identity_control_classifies_internal = matches!(
        classify_raw_candidate_v2(&correctly_levelled_identity_control_candidate()),
        RawCandidateDecisionV2::Internal { .. }
    );
    let global_e4_successor_rerun_authorized = true;
    let forbidden_outputs = InternalBranchForbiddenOutputs {
        global_e4_successor_rerun_executed: false,
        pending_membership_verdicts_issued: false,
        e2b_executed: false,
        stage_count_issued: false,
        fq2_evaluated: false,
        halt_or_continuation_claimed: false,
    };
    if !witness_univ_earned_as_arena_reference
        || !witness_lambda_resolved_to_candidate_field_zero
        || witness_lambda_resolved_to_local_binder
        || witness_internal_certificate_issued
        || !f_i1_no_uncertified_internal_classification
        || !f_i2_triggered
        || !unknown_retained
        || !correctly_levelled_identity_control_classifies_internal
        || !forbidden_outputs.all_withheld()
    {
        return Err(InternalBranchError::Invariant(
            "witness-first F-I2 disposition drifted".to_owned(),
        ));
    }

    let mut certificate = InternalBranchCertificate {
        schema: INTERNAL_BRANCH_SCHEMA.to_owned(),
        date: INTERNAL_BRANCH_DATE.to_owned(),
        source_bindings: source_bindings(&g8_bytes),
        predecessor_g8_digest: g8.result_digest,
        predecessor_g8_replayed: true,
        adjudication_adopted: true,
        classifier_priority_order: vec![
            "surface_or_typed_exclusion".to_owned(),
            "internal_with_replayed_derivability_certificate".to_owned(),
            "adopted_schema2_class".to_owned(),
            "named_f_g4_obstruction".to_owned(),
        ],
        witness_decision,
        witness_first_obligation_executed: true,
        witness_univ_earned_as_arena_reference,
        witness_lambda_resolved_to_candidate_field_zero,
        witness_lambda_resolved_to_local_binder,
        witness_internal_certificate_issued,
        f_i1_no_uncertified_internal_classification,
        f_i2_triggered,
        unknown_retained,
        correctly_levelled_identity_control_classifies_internal,
        global_e4_successor_rerun_authorized,
        forbidden_outputs,
        outcome: "internal_branch_executed_witness_failed_exact_b15_derivability_f_i2_unknown_retained"
            .to_owned(),
        permitted_conclusion: "Univ earns the arena-reference clause, but under the frozen absolute-level convention Lam(Var(1)) refers to candidate field 0 rather than the lambda binder. The exact witness is not derivable solely over B15 and remains Unknown under F-I2/F-G4."
            .to_owned(),
        required_successor_action: "Rerun global E-4 create-new with this successor decision. It must remain fail-closed on the surviving exact witness; no membership verdict or count is authorized. Any semantic intent to use the identity must adjudicate Lam(Var(2)) as a new versioned candidate rather than reinterpret Var(1)."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> InternalBranchReplay {
    InternalBranchReplay {
        valid: false,
        witness_obligation_executed: false,
        witness_internal: false,
        f_i2_triggered: false,
        unknown_retained: false,
        identity_control_internal: false,
        global_e4_rerun_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "internal_branch_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &InternalBranchCertificate,
    expected: &InternalBranchCertificate,
) -> InternalBranchReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    InternalBranchReplay {
        valid: errors.is_empty(),
        witness_obligation_executed: certificate.witness_first_obligation_executed,
        witness_internal: certificate.witness_internal_certificate_issued,
        f_i2_triggered: certificate.f_i2_triggered,
        unknown_retained: certificate.unknown_retained,
        identity_control_internal: certificate
            .correctly_levelled_identity_control_classifies_internal,
        global_e4_rerun_authorized: certificate.global_e4_successor_rerun_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_internal_branch_certificate(
    certificate: &InternalBranchCertificate,
) -> InternalBranchReplay {
    let expected = match issue_internal_branch_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_internal_branch_json(json: &str) -> InternalBranchReplay {
    match serde_json::from_str::<InternalBranchCertificate>(json) {
        Ok(certificate) => replay_internal_branch_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_internal_branch_create_new(
    path: &Path,
) -> Result<InternalBranchReplay, InternalBranchError> {
    let certificate = issue_internal_branch_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| InternalBranchError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| InternalBranchError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| InternalBranchError::Io(error.to_string()))?;
    let replay = replay_internal_branch_certificate(&certificate);
    if !replay.valid {
        return Err(InternalBranchError::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn witness_lambda_is_a_candidate_field_reference_not_the_identity() {
        let attempt = issue_internal_derivability_attempt(&f_g4_witness_candidate())
            .expect("typed witness attempt");
        assert!(attempt.clauses[0].derived_over_exact_b15);
        assert_eq!(
            attempt.clauses[1].candidate_local_field_dependencies,
            vec![0]
        );
        assert!(
            !attempt.clauses[1]
                .leaf_rules
                .iter()
                .any(|rule| rule == "local-var-1")
        );
        assert!(!attempt.internal_certificate_issued);
        assert_eq!(
            attempt.failure_code.as_deref(),
            Some(F_I2_WITNESS_NOT_DERIVABLE_OVER_B15)
        );
    }

    #[test]
    fn correctly_levelled_identity_earns_the_internal_branch() {
        let decision = classify_raw_candidate_v2(&correctly_levelled_identity_control_candidate());
        let RawCandidateDecisionV2::Internal {
            certificate,
            marginal_nu,
            ..
        } = decision
        else {
            panic!("Lam(Var(2)) should resolve to the local binder")
        };
        assert_eq!(marginal_nu, 0);
        assert!(certificate.internal_certificate_issued);
        assert!(matches!(
            certificate.clauses[1].kind,
            ClauseDerivabilityKind::StructuralIdentity
        ));
    }

    #[test]
    fn witness_first_certificate_replays_and_mutations_fail_closed() {
        let certificate = issue_internal_branch_certificate().expect("branch certificate");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        assert!(certificate.f_i2_triggered);
        assert!(certificate.unknown_retained);
        assert!(!certificate.witness_internal_certificate_issued);

        let mut mutations = Vec::new();
        let mut source = certificate.clone();
        source.source_bindings[0].blake3.push('0');
        mutations.push(source);
        let mut proof = certificate.clone();
        proof.witness_internal_certificate_issued = true;
        mutations.push(proof);
        let mut f_i2 = certificate.clone();
        f_i2.f_i2_triggered = false;
        mutations.push(f_i2);
        let mut unknown = certificate.clone();
        unknown.unknown_retained = false;
        mutations.push(unknown);
        let mut forbidden = certificate.clone();
        forbidden.forbidden_outputs.e2b_executed = true;
        mutations.push(forbidden);
        for mutation in mutations {
            assert!(!replay_against_expected(&mutation, &certificate).valid);
        }
    }
}
