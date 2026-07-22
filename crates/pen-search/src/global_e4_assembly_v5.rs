//! Global E-4 create-new rerun after adding zero-credit, constructor-replayable
//! structural lambda closure.

use crate::enumerate::{
    EnumerationContext, LateFamilySurface, RawSurfaceMembership, assess_raw_surface_membership,
};
use crate::global_e4_assembly_v4::{GlobalE4V4Certificate, replay_global_e4_v4_json};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_schema::internal_classifier_branch_v2::RawCandidateDecisionV3;
use pen_schema::internal_classifier_branch_v3::RawCandidateDecisionV4;
use pen_schema::internal_classifier_branch_v4::{
    RawCandidateDecisionV5, StructuralLambdaInternalityCertificate, classify_raw_candidate_v5,
    replay_structural_lambda_internality_json, structural_constant_universe_candidate,
};
use pen_schema::total_classifier::{F_G4_TYPED_UNKNOWN, RawCandidateDecision};
use pen_type::elaborate::SealedSignature;
use pen_type::structural_internality::issue_structural_lambda_closure_token;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GLOBAL_E4_V5_SCHEMA: &str = "schema2-global-e4-assembly-v5";
pub const GLOBAL_E4_V5_DATE: &str = "2026-07-20";
pub const APPLICATION_BODY_GAP: &str = "INDUCTIVE_TELESCOPE_INTERNALITY_NOT_PROVED_FOR_CANDIDATE";

const V4_RESULT_BYTES: &[u8] = include_bytes!("../../../docs/SCHEMA2_GLOBAL_E4_V4_RESULT.md");
const ENUMERATOR_BYTES: &[u8] = include_bytes!("enumerate.rs");
const STRUCTURAL_TOKEN_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/structural_internality.rs");
const CLASSIFIER_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/internal_classifier_branch_v4.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v4.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v5.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V5SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedStructuralLambdaRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub decision: RawCandidateDecisionV5,
    pub exact_raw_catalog_member: bool,
    pub classified_internal: bool,
    pub inherited_internal_clause_count: usize,
    pub constructor_evidence_count: usize,
    pub body_already_internal: bool,
    pub lambda_constructor_replayed: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationBodyUnknownRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub decision: RawCandidateDecisionV5,
    pub exact_raw_catalog_member: bool,
    pub ambient_parameters: u32,
    pub constructor_token_refused: bool,
    pub constructor_refusal: String,
    pub named_internal_gap: String,
    pub f_g4_retained: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V5ForbiddenOutputs {
    pub five_pending_membership_verdicts_issued: bool,
    pub independent_family_tokens_issued: bool,
    pub e2b_executed: bool,
    pub historical_scores_recomputed: bool,
    pub fq2_evaluated: bool,
    pub agent_a_f_t1_discharge_executed: bool,
    pub e5_guard_rail_f1_executed: bool,
    pub classifier_bridge_or_fork_executed: bool,
    pub halt_or_continuation_claimed: bool,
}

impl GlobalE4V5ForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.five_pending_membership_verdicts_issued
            && !self.independent_family_tokens_issued
            && !self.e2b_executed
            && !self.historical_scores_recomputed
            && !self.fq2_evaluated
            && !self.agent_a_f_t1_discharge_executed
            && !self.e5_guard_rail_f1_executed
            && !self.classifier_bridge_or_fork_executed
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V5Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GlobalE4V5SourceBinding>,
    pub predecessor_global_e4_v4_digest: String,
    pub predecessor_global_e4_v4_replayed: bool,
    pub structural_lambda_internality_digest: String,
    pub structural_lambda_internality_replayed: bool,
    pub global_e4_v5_executed: bool,
    pub predecessor_structural_unknown_resolved: ResolvedStructuralLambdaRecord,
    pub next_application_body_unknown: ApplicationBodyUnknownRecord,
    pub unknown_survivor_count_lower_bound: usize,
    pub fail_fast_catalog_scan_terminated: bool,
    pub no_unknown_survives: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub five_pending_memberships_now_authorized: bool,
    pub forbidden_outputs: GlobalE4V5ForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V5Replay {
    pub valid: bool,
    pub rerun_executed: bool,
    pub structural_unknown_resolved: bool,
    pub next_exact_unknown_found: bool,
    pub application_body_gap: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub pending_memberships_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GlobalE4V5Error {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
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
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V5_SCHEMA, domain, value))
        .expect("global E-4 v5 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(
    predecessor_bytes: &[u8],
    structural_bytes: &[u8],
) -> Vec<GlobalE4V5SourceBinding> {
    [
        (
            "docs/SCHEMA2_GLOBAL_E4_V4_RESULT.md",
            "predecessor_structural_gap",
            V4_RESULT_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v4.json",
            "failed_predecessor_assembly",
            predecessor_bytes,
        ),
        (
            "docs/schema2_structural_lambda_internality_v1.json",
            "zero_credit_constructor_successor",
            structural_bytes,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "exact_raw_catalog_membership",
            ENUMERATOR_BYTES,
        ),
        (
            "crates/pen-type/src/structural_internality.rs",
            "constructor_replay_and_refusal",
            STRUCTURAL_TOKEN_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v4.rs",
            "structural_lambda_internal_classifier",
            CLASSIFIER_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v4.rs",
            "predecessor_fail_fast_assembly",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v5.rs",
            "application_body_fail_fast_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| GlobalE4V5SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn frozen_raw_context() -> EnumerationContext {
    EnumerationContext {
        library_size: 15,
        scope_size: 2,
        max_path_dimension: 1,
        include_trunc: false,
        include_modal: true,
        include_temporal: true,
        include_linear_exponential: false,
        max_expr_nodes: 6,
        require_former_eliminator_clauses: false,
        require_initial_hit_clauses: false,
        require_truncation_hit_clauses: false,
        require_higher_hit_clauses: false,
        require_sphere_lift_clauses: false,
        require_axiomatic_bundle_clauses: false,
        require_modal_shell_clauses: false,
        require_connection_shell_clauses: false,
        require_curvature_shell_clauses: false,
        require_operator_bundle_clauses: false,
        require_hilbert_functional_clauses: false,
        require_temporal_shell_clauses: false,
        historical_anchor_ref: None,
        late_family_surface: LateFamilySurface::None,
    }
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

fn issue_resolved_structural_lambda() -> Result<ResolvedStructuralLambdaRecord, GlobalE4V5Error> {
    let candidate = structural_constant_universe_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let decision = classify_raw_candidate_v5(&candidate);
    let (
        classified_internal,
        inherited_internal_clause_count,
        constructor_evidence_count,
        body_already_internal,
        lambda_constructor_replayed,
        marginal_nu,
    ) = match &decision {
        RawCandidateDecisionV5::InternalStructuralLambda {
            certificate,
            marginal_nu,
            ..
        } => (
            certificate.internal_certificate_issued,
            certificate.inherited_internal_clauses.len(),
            certificate.constructor_evidence.len(),
            certificate.every_body_already_internal,
            certificate.every_lambda_constructor_replayed,
            *marginal_nu,
        ),
        _ => (false, 0, 0, false, false, u32::MAX),
    };
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    if !exact_raw_catalog_member
        || !classified_internal
        || inherited_internal_clause_count != 1
        || constructor_evidence_count != 1
        || !body_already_internal
        || !lambda_constructor_replayed
        || marginal_nu != 0
    {
        return Err(GlobalE4V5Error::Invariant(
            "structural lambda predecessor Unknown was not resolved".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "resolved-structural-lambda",
        &(
            &candidate,
            &raw_surface_membership,
            &decision,
            exact_raw_catalog_member,
            classified_internal,
            inherited_internal_clause_count,
            constructor_evidence_count,
            body_already_internal,
            lambda_constructor_replayed,
            marginal_nu,
        ),
    );
    Ok(ResolvedStructuralLambdaRecord {
        candidate,
        raw_surface_membership,
        decision,
        exact_raw_catalog_member,
        classified_internal,
        inherited_internal_clause_count,
        constructor_evidence_count,
        body_already_internal,
        lambda_constructor_replayed,
        marginal_nu,
        derivation_hash,
    })
}

fn issue_next_application_body_unknown() -> Result<ApplicationBodyUnknownRecord, GlobalE4V5Error> {
    let candidate = application_body_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let decision = classify_raw_candidate_v5(&candidate);
    let (ambient_parameters, named_internal_gap, f_g4_retained) = match &decision {
        RawCandidateDecisionV5::NamedTypedObstruction {
            predecessor_decision:
                RawCandidateDecisionV4::NamedTypedObstruction {
                    predecessor_decision:
                        RawCandidateDecisionV3::NamedTypedObstruction {
                            legacy_decision,
                            internal_attempt,
                        },
                },
        } => (
            0,
            internal_attempt.failure_code.clone().unwrap_or_default(),
            matches!(
                legacy_decision,
                RawCandidateDecision::NamedTypedObstruction { code, .. }
                    if code == F_G4_TYPED_UNKNOWN
            ),
        ),
        _ => (u32::MAX, String::new(), false),
    };
    let signature = SealedSignature::genesis_del_h15();
    let constructor_refusal = issue_structural_lambda_closure_token(&signature, &candidate, 15, 1)
        .expect_err("application bodies must not receive structural lambda tokens")
        .to_string();
    let constructor_token_refused = constructor_refusal.contains("unsupported structural body");
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    if !exact_raw_catalog_member
        || ambient_parameters != 0
        || !constructor_token_refused
        || named_internal_gap != APPLICATION_BODY_GAP
        || !f_g4_retained
    {
        return Err(GlobalE4V5Error::Invariant(format!(
            "application-body candidate disposition drifted: exact={exact_raw_catalog_member}, ambient={ambient_parameters}, token_refused={constructor_token_refused}, gap={named_internal_gap:?}, f_g4={f_g4_retained}, membership_rejections={:?}, decision={decision:?}",
            raw_surface_membership.rejection_reasons,
        )));
    }
    let derivation_hash = tagged_hash(
        "next-application-body-unknown",
        &(
            &candidate,
            &raw_surface_membership,
            &decision,
            exact_raw_catalog_member,
            ambient_parameters,
            constructor_token_refused,
            &constructor_refusal,
            &named_internal_gap,
            f_g4_retained,
        ),
    );
    Ok(ApplicationBodyUnknownRecord {
        candidate,
        raw_surface_membership,
        decision,
        exact_raw_catalog_member,
        ambient_parameters,
        constructor_token_refused,
        constructor_refusal,
        named_internal_gap,
        f_g4_retained,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &GlobalE4V5Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("global-e4-v5-certificate", &projection)
}

pub fn issue_global_e4_v5_certificate() -> Result<GlobalE4V5Certificate, GlobalE4V5Error> {
    let predecessor_bytes = std::fs::read(workspace_doc_path("schema2_global_e4_assembly_v4.json"))
        .map_err(|error| GlobalE4V5Error::Io(error.to_string()))?;
    let predecessor_replay = replay_global_e4_v4_json(
        std::str::from_utf8(&predecessor_bytes)
            .map_err(|error| GlobalE4V5Error::Json(error.to_string()))?,
    );
    if !predecessor_replay.valid
        || !predecessor_replay.structural_lambda_gap
        || predecessor_replay.global_e4_complete
    {
        return Err(GlobalE4V5Error::Prerequisite(format!(
            "global E-4 v4 replay failed: {}",
            predecessor_replay.errors.join("; ")
        )));
    }
    let predecessor: GlobalE4V4Certificate = serde_json::from_slice(&predecessor_bytes)
        .map_err(|error| GlobalE4V5Error::Json(error.to_string()))?;

    let structural_bytes = std::fs::read(workspace_doc_path(
        "schema2_structural_lambda_internality_v1.json",
    ))
    .map_err(|error| GlobalE4V5Error::Io(error.to_string()))?;
    let structural_replay = replay_structural_lambda_internality_json(
        std::str::from_utf8(&structural_bytes)
            .map_err(|error| GlobalE4V5Error::Json(error.to_string()))?,
    );
    if !structural_replay.valid
        || !structural_replay.structural_internal
        || !structural_replay.body_already_internal
        || !structural_replay.constructor_replayed
        || structural_replay.marginal_nu != 0
        || !structural_replay.global_e4_rerun_authorized
    {
        return Err(GlobalE4V5Error::Prerequisite(format!(
            "structural lambda replay failed: {}",
            structural_replay.errors.join("; ")
        )));
    }
    let structural: StructuralLambdaInternalityCertificate =
        serde_json::from_slice(&structural_bytes)
            .map_err(|error| GlobalE4V5Error::Json(error.to_string()))?;

    let predecessor_structural_unknown_resolved = issue_resolved_structural_lambda()?;
    let next_application_body_unknown = issue_next_application_body_unknown()?;
    let unknown_survivor_count_lower_bound = 1;
    let fail_fast_catalog_scan_terminated = true;
    let no_unknown_survives = false;
    let class_exhaustion_proved = false;
    let global_e4_complete = false;
    let five_pending_memberships_now_authorized = false;
    let forbidden_outputs = GlobalE4V5ForbiddenOutputs {
        five_pending_membership_verdicts_issued: false,
        independent_family_tokens_issued: false,
        e2b_executed: false,
        historical_scores_recomputed: false,
        fq2_evaluated: false,
        agent_a_f_t1_discharge_executed: false,
        e5_guard_rail_f1_executed: false,
        classifier_bridge_or_fork_executed: false,
        halt_or_continuation_claimed: false,
    };
    if !predecessor_structural_unknown_resolved.classified_internal
        || !next_application_body_unknown.f_g4_retained
        || no_unknown_survives
        || class_exhaustion_proved
        || global_e4_complete
        || five_pending_memberships_now_authorized
        || !forbidden_outputs.all_withheld()
    {
        return Err(GlobalE4V5Error::Invariant(
            "application-body Unknown firewall drifted".to_owned(),
        ));
    }

    let mut certificate = GlobalE4V5Certificate {
        schema: GLOBAL_E4_V5_SCHEMA.to_owned(),
        date: GLOBAL_E4_V5_DATE.to_owned(),
        source_bindings: source_bindings(&predecessor_bytes, &structural_bytes),
        predecessor_global_e4_v4_digest: predecessor.result_digest,
        predecessor_global_e4_v4_replayed: true,
        structural_lambda_internality_digest: structural.result_digest,
        structural_lambda_internality_replayed: true,
        global_e4_v5_executed: true,
        predecessor_structural_unknown_resolved,
        next_application_body_unknown,
        unknown_survivor_count_lower_bound,
        fail_fast_catalog_scan_terminated,
        no_unknown_survives,
        class_exhaustion_proved,
        global_e4_complete,
        five_pending_memberships_now_authorized,
        forbidden_outputs,
        outcome: "global_e4_v5_resolved_structural_lambda_next_application_body_unknown_blocks_exhaustion"
            .to_owned(),
        permitted_conclusion: "The exact [Univ,Lam(Univ)] witness now earns Internal by replayed lambda closure with nu=0. The exact [Univ,Lam(App(Lib(14),Lib(15)))] witness remains Unknown because an application body has no structural Internal constructor proof."
            .to_owned(),
        required_successor_action: "Decide whether application is an Internal structural constructor. If adopted, require replayable function and argument premises plus an application-congruence/naturality rule, preserve typed result formation, and award no credit unless an independently exported demand orbit is produced; then rerun global E-4 create-new."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GlobalE4V5Replay {
    GlobalE4V5Replay {
        valid: false,
        rerun_executed: false,
        structural_unknown_resolved: false,
        next_exact_unknown_found: false,
        application_body_gap: false,
        class_exhaustion_proved: false,
        global_e4_complete: false,
        pending_memberships_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "global_e4_v5_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &GlobalE4V5Certificate,
    expected: &GlobalE4V5Certificate,
) -> GlobalE4V5Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GlobalE4V5Replay {
        valid: errors.is_empty(),
        rerun_executed: certificate.global_e4_v5_executed,
        structural_unknown_resolved: certificate
            .predecessor_structural_unknown_resolved
            .classified_internal,
        next_exact_unknown_found: certificate
            .next_application_body_unknown
            .exact_raw_catalog_member
            && certificate.next_application_body_unknown.f_g4_retained,
        application_body_gap: certificate
            .next_application_body_unknown
            .constructor_token_refused,
        class_exhaustion_proved: certificate.class_exhaustion_proved,
        global_e4_complete: certificate.global_e4_complete,
        pending_memberships_authorized: certificate.five_pending_memberships_now_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_global_e4_v5_certificate(certificate: &GlobalE4V5Certificate) -> GlobalE4V5Replay {
    let expected = match issue_global_e4_v5_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_global_e4_v5_json(json: &str) -> GlobalE4V5Replay {
    match serde_json::from_str::<GlobalE4V5Certificate>(json) {
        Ok(certificate) => replay_global_e4_v5_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_global_e4_v5_create_new(path: &Path) -> Result<GlobalE4V5Replay, GlobalE4V5Error> {
    let certificate = issue_global_e4_v5_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GlobalE4V5Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GlobalE4V5Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GlobalE4V5Error::Io(error.to_string()))?;
    let replay = replay_global_e4_v5_certificate(&certificate);
    if !replay.valid {
        return Err(GlobalE4V5Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structural_lambda_is_resolved_at_zero_credit() {
        let resolved = issue_resolved_structural_lambda().expect("resolved structural lambda");
        assert!(resolved.exact_raw_catalog_member);
        assert!(resolved.classified_internal);
        assert_eq!(resolved.inherited_internal_clause_count, 1);
        assert_eq!(resolved.constructor_evidence_count, 1);
        assert!(resolved.body_already_internal);
        assert!(resolved.lambda_constructor_replayed);
        assert_eq!(resolved.marginal_nu, 0);
    }

    #[test]
    fn application_body_is_exact_and_remains_unknown() {
        let unknown = issue_next_application_body_unknown().expect("application body Unknown");
        assert!(unknown.exact_raw_catalog_member);
        assert_eq!(unknown.ambient_parameters, 0);
        assert!(unknown.constructor_token_refused);
        assert_eq!(unknown.named_internal_gap, APPLICATION_BODY_GAP);
        assert!(unknown.f_g4_retained);
    }

    #[test]
    fn v5_replay_and_mutations_fail_closed() {
        let certificate = issue_global_e4_v5_certificate().expect("v5 assembly");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut resolved = certificate.clone();
        resolved
            .predecessor_structural_unknown_resolved
            .classified_internal = false;
        mutations.push(resolved);
        let mut survivor = certificate.clone();
        survivor.next_application_body_unknown.f_g4_retained = false;
        mutations.push(survivor);
        let mut exhaustion = certificate.clone();
        exhaustion.class_exhaustion_proved = true;
        mutations.push(exhaustion);
        let mut forbidden = certificate.clone();
        forbidden.forbidden_outputs.e2b_executed = true;
        mutations.push(forbidden);
        for mutation in mutations {
            assert!(!replay_against_expected(&mutation, &certificate).valid);
        }
    }
}
