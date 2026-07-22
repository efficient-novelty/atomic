//! Global E-4 create-new rerun after transparent ambient-former closure.
//!
//! The rerun is deliberately fail-fast.  It first replays the exact v5
//! application witness into `Internal`, then exhibits the next exact raw
//! member whose application head is a candidate field.  Option A / F-A3
//! excludes that fresh head, so v6 records a genuine next rung and withholds
//! exhaustion and every downstream output.

use crate::enumerate::{
    EnumerationContext, LateFamilySurface, RawSurfaceMembership, assess_raw_surface_membership,
};
use crate::global_e4_assembly_v5::{GLOBAL_E4_V5_SCHEMA, GlobalE4V5Certificate};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_schema::internal_classifier_branch_v2::RawCandidateDecisionV3;
use pen_schema::internal_classifier_branch_v3::RawCandidateDecisionV4;
use pen_schema::internal_classifier_branch_v4::RawCandidateDecisionV5;
use pen_schema::internal_classifier_branch_v5::{
    AmbientFormerInternalityCertificate, RawCandidateDecisionV6, application_body_candidate,
    candidate_fresh_head_candidate, classify_raw_candidate_v6,
    replay_ambient_former_internality_json,
};
use pen_schema::total_classifier::{F_G4_TYPED_UNKNOWN, RawCandidateDecision};
use pen_type::ambient_former_internality::{
    AmbientFormerInternalityError, issue_ambient_former_closure_token,
};
use pen_type::elaborate::SealedSignature;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GLOBAL_E4_V6_SCHEMA: &str = "schema2-global-e4-assembly-v6";
pub const GLOBAL_E4_V6_DATE: &str = "2026-07-21";
pub const CANDIDATE_FRESH_HEAD_GAP: &str =
    "INDUCTIVE_TELESCOPE_INTERNALITY_NOT_PROVED_FOR_CANDIDATE";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/ambient_former_closure_adjudication.md");
const ENUMERATOR_BYTES: &[u8] = include_bytes!("enumerate.rs");
const TOKEN_BYTES: &[u8] = include_bytes!("../../pen-type/src/ambient_former_internality.rs");
const CLASSIFIER_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/internal_classifier_branch_v5.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v5.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v6.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V6SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedAmbientApplicationRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub decision: RawCandidateDecisionV6,
    pub exact_raw_catalog_member: bool,
    pub classified_internal: bool,
    pub inherited_internal_clause_count: usize,
    pub constructor_evidence_count: usize,
    pub application_stuckness_replayed: bool,
    pub every_premise_internal: bool,
    pub typed_result_preserved: bool,
    pub full_provenance_retained: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CandidateFreshHeadUnknownRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub decision: RawCandidateDecisionV6,
    pub exact_raw_catalog_member: bool,
    pub ambient_parameters: u32,
    pub prior_clause_certificate_supplied: bool,
    pub ambient_closure_refused: bool,
    pub ambient_closure_refusal: String,
    pub f_a3_candidate_fresh_head_firewall_replayed: bool,
    pub named_internal_gap: String,
    pub f_g4_retained: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V6ForbiddenOutputs {
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

impl GlobalE4V6ForbiddenOutputs {
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
pub struct GlobalE4V6Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GlobalE4V6SourceBinding>,
    pub predecessor_global_e4_v5_digest: String,
    pub predecessor_global_e4_v5_replayed: bool,
    pub ambient_former_internality_digest: String,
    pub ambient_former_internality_replayed: bool,
    pub global_e4_v6_executed: bool,
    pub predecessor_application_unknown_resolved: ResolvedAmbientApplicationRecord,
    pub next_candidate_fresh_head_unknown: CandidateFreshHeadUnknownRecord,
    pub unknown_survivor_count_lower_bound: usize,
    pub fail_fast_catalog_scan_terminated: bool,
    pub no_unknown_survives: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub five_pending_memberships_now_authorized: bool,
    pub forbidden_outputs: GlobalE4V6ForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V6Replay {
    pub valid: bool,
    pub rerun_executed: bool,
    pub application_unknown_resolved: bool,
    pub next_exact_unknown_found: bool,
    pub candidate_fresh_head_gap: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub pending_memberships_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GlobalE4V6Error {
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
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V6_SCHEMA, domain, value))
        .expect("global E-4 v6 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(predecessor_bytes: &[u8], ambient_bytes: &[u8]) -> Vec<GlobalE4V6SourceBinding> {
    [
        (
            "docs/schema2_global_e4_assembly_v5.json",
            "application_body_unknown_predecessor",
            predecessor_bytes,
        ),
        (
            "docs/schema2_ambient_former_internality_v1.json",
            "transparent_former_term_level_successor",
            ambient_bytes,
        ),
        (
            "docs/ambient_former_closure_adjudication.md",
            "option_a_and_f_a3_firewall",
            ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "exact_raw_catalog_membership",
            ENUMERATOR_BYTES,
        ),
        (
            "crates/pen-type/src/ambient_former_internality.rs",
            "term_constructor_replay_and_fresh_head_refusal",
            TOKEN_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v5.rs",
            "ambient_former_internal_classifier",
            CLASSIFIER_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v5.rs",
            "predecessor_fail_fast_assembly",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v6.rs",
            "candidate_fresh_head_fail_fast_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| GlobalE4V6SourceBinding {
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

fn sealed_v5_predecessor_valid(certificate: &GlobalE4V5Certificate) -> bool {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V5_SCHEMA, "global-e4-v5-certificate", &projection))
        .expect("sealed v5 predecessor serializes");
    let expected_digest = format!("blake3:{}", blake3_hex(&bytes));
    certificate.schema == GLOBAL_E4_V5_SCHEMA
        && certificate.result_digest == expected_digest
        && certificate.global_e4_v5_executed
        && certificate
            .next_application_body_unknown
            .exact_raw_catalog_member
        && certificate.next_application_body_unknown.f_g4_retained
        && certificate.unknown_survivor_count_lower_bound >= 1
        && certificate.fail_fast_catalog_scan_terminated
        && !certificate.no_unknown_survives
        && !certificate.class_exhaustion_proved
        && !certificate.global_e4_complete
        && !certificate.five_pending_memberships_now_authorized
        && !certificate
            .forbidden_outputs
            .five_pending_membership_verdicts_issued
        && !certificate
            .forbidden_outputs
            .independent_family_tokens_issued
        && !certificate.forbidden_outputs.e2b_executed
        && !certificate.forbidden_outputs.historical_scores_recomputed
        && !certificate.forbidden_outputs.fq2_evaluated
        && !certificate
            .forbidden_outputs
            .agent_a_f_t1_discharge_executed
        && !certificate.forbidden_outputs.e5_guard_rail_f1_executed
        && !certificate
            .forbidden_outputs
            .classifier_bridge_or_fork_executed
        && !certificate.forbidden_outputs.halt_or_continuation_claimed
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

fn issue_resolved_application() -> Result<ResolvedAmbientApplicationRecord, GlobalE4V6Error> {
    let candidate = application_body_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let decision = classify_raw_candidate_v6(&candidate);
    let (
        classified_internal,
        inherited_internal_clause_count,
        constructor_evidence_count,
        application_stuckness_replayed,
        every_premise_internal,
        typed_result_preserved,
        full_provenance_retained,
        marginal_nu,
    ) = match &decision {
        RawCandidateDecisionV6::InternalAmbientFormer {
            certificate,
            marginal_nu,
            ..
        } => (
            certificate.internal_certificate_issued,
            certificate.inherited_internal_clauses.len(),
            certificate.constructor_evidence.len(),
            certificate
                .constructor_evidence
                .iter()
                .any(|evidence| tree_has_rule(&evidence.term_evidence, "app-stuck")),
            certificate.every_premise_internal,
            certificate.typed_results_preserved,
            certificate.full_provenance_retained,
            *marginal_nu,
        ),
        _ => (false, 0, 0, false, false, false, false, u32::MAX),
    };
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    if !exact_raw_catalog_member
        || !classified_internal
        || inherited_internal_clause_count != 1
        || constructor_evidence_count != 1
        || !application_stuckness_replayed
        || !every_premise_internal
        || !typed_result_preserved
        || !full_provenance_retained
        || marginal_nu != 0
    {
        return Err(GlobalE4V6Error::Invariant(
            "v5 application Unknown was not resolved by term-level closure".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "resolved-ambient-application",
        &(
            &candidate,
            &raw_surface_membership,
            &decision,
            exact_raw_catalog_member,
            classified_internal,
            inherited_internal_clause_count,
            constructor_evidence_count,
            application_stuckness_replayed,
            every_premise_internal,
            typed_result_preserved,
            full_provenance_retained,
            marginal_nu,
        ),
    );
    Ok(ResolvedAmbientApplicationRecord {
        candidate,
        raw_surface_membership,
        decision,
        exact_raw_catalog_member,
        classified_internal,
        inherited_internal_clause_count,
        constructor_evidence_count,
        application_stuckness_replayed,
        every_premise_internal,
        typed_result_preserved,
        full_provenance_retained,
        marginal_nu,
        derivation_hash,
    })
}

fn issue_next_candidate_fresh_head_unknown()
-> Result<CandidateFreshHeadUnknownRecord, GlobalE4V6Error> {
    let candidate = candidate_fresh_head_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let decision = classify_raw_candidate_v6(&candidate);
    let (ambient_parameters, named_internal_gap, f_g4_retained) = match &decision {
        RawCandidateDecisionV6::NamedTypedObstruction {
            predecessor_decision:
                RawCandidateDecisionV5::NamedTypedObstruction {
                    predecessor_decision:
                        RawCandidateDecisionV4::NamedTypedObstruction {
                            predecessor_decision:
                                RawCandidateDecisionV3::NamedTypedObstruction {
                                    legacy_decision,
                                    internal_attempt,
                                },
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
    let mut certified = BTreeMap::new();
    certified.insert(0, "sealed-internal-clause-0".to_owned());
    let refusal = issue_ambient_former_closure_token(&signature, &candidate, 15, 1, &certified)
        .expect_err("candidate-fresh application head must be refused");
    let f_a3_candidate_fresh_head_firewall_replayed =
        refusal == AmbientFormerInternalityError::CandidateFreshApplicationHead;
    let ambient_closure_refusal = refusal.to_string();
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    let prior_clause_certificate_supplied = certified.contains_key(&0);
    let ambient_closure_refused = f_a3_candidate_fresh_head_firewall_replayed;
    if !exact_raw_catalog_member
        || ambient_parameters != 0
        || !prior_clause_certificate_supplied
        || !ambient_closure_refused
        || named_internal_gap != CANDIDATE_FRESH_HEAD_GAP
        || !f_g4_retained
    {
        return Err(GlobalE4V6Error::Invariant(format!(
            "candidate-fresh-head disposition drifted: exact={exact_raw_catalog_member}, ambient={ambient_parameters}, prior={prior_clause_certificate_supplied}, refused={ambient_closure_refused}, gap={named_internal_gap:?}, f_g4={f_g4_retained}, membership_rejections={:?}, decision={decision:?}",
            raw_surface_membership.rejection_reasons,
        )));
    }
    let derivation_hash = tagged_hash(
        "next-candidate-fresh-head-unknown",
        &(
            &candidate,
            &raw_surface_membership,
            &decision,
            exact_raw_catalog_member,
            ambient_parameters,
            prior_clause_certificate_supplied,
            ambient_closure_refused,
            &ambient_closure_refusal,
            f_a3_candidate_fresh_head_firewall_replayed,
            &named_internal_gap,
            f_g4_retained,
        ),
    );
    Ok(CandidateFreshHeadUnknownRecord {
        candidate,
        raw_surface_membership,
        decision,
        exact_raw_catalog_member,
        ambient_parameters,
        prior_clause_certificate_supplied,
        ambient_closure_refused,
        ambient_closure_refusal,
        f_a3_candidate_fresh_head_firewall_replayed,
        named_internal_gap,
        f_g4_retained,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &GlobalE4V6Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("global-e4-v6-certificate", &projection)
}

pub fn issue_global_e4_v6_certificate() -> Result<GlobalE4V6Certificate, GlobalE4V6Error> {
    let predecessor_bytes = std::fs::read(workspace_doc_path("schema2_global_e4_assembly_v5.json"))
        .map_err(|error| GlobalE4V6Error::Io(error.to_string()))?;
    let predecessor: GlobalE4V5Certificate = serde_json::from_slice(&predecessor_bytes)
        .map_err(|error| GlobalE4V6Error::Json(error.to_string()))?;
    if !sealed_v5_predecessor_valid(&predecessor) {
        return Err(GlobalE4V6Error::Prerequisite(
            "sealed global E-4 v5 digest or required invariants failed".to_owned(),
        ));
    }

    let ambient_bytes = std::fs::read(workspace_doc_path(
        "schema2_ambient_former_internality_v1.json",
    ))
    .map_err(|error| GlobalE4V6Error::Io(error.to_string()))?;
    let ambient_replay = replay_ambient_former_internality_json(
        std::str::from_utf8(&ambient_bytes)
            .map_err(|error| GlobalE4V6Error::Json(error.to_string()))?,
    );
    if !ambient_replay.valid
        || !ambient_replay.former_inventory_complete
        || !ambient_replay.falsifiers_replayed
        || !ambient_replay.application_internal
        || !ambient_replay.application_constructor_replayed
        || ambient_replay.marginal_nu != 0
        || !ambient_replay.global_e4_v6_authorized
        || !ambient_replay.forbidden_outputs_withheld
    {
        return Err(GlobalE4V6Error::Prerequisite(format!(
            "ambient former internality replay failed: {}",
            ambient_replay.errors.join("; ")
        )));
    }
    let ambient: AmbientFormerInternalityCertificate = serde_json::from_slice(&ambient_bytes)
        .map_err(|error| GlobalE4V6Error::Json(error.to_string()))?;

    let predecessor_application_unknown_resolved = issue_resolved_application()?;
    let next_candidate_fresh_head_unknown = issue_next_candidate_fresh_head_unknown()?;
    let forbidden_outputs = GlobalE4V6ForbiddenOutputs {
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
    let unknown_survivor_count_lower_bound = 1;
    let fail_fast_catalog_scan_terminated = true;
    let no_unknown_survives = false;
    let class_exhaustion_proved = false;
    let global_e4_complete = false;
    let five_pending_memberships_now_authorized = false;
    if !predecessor_application_unknown_resolved.classified_internal
        || !next_candidate_fresh_head_unknown.f_a3_candidate_fresh_head_firewall_replayed
        || !next_candidate_fresh_head_unknown.f_g4_retained
        || no_unknown_survives
        || class_exhaustion_proved
        || global_e4_complete
        || five_pending_memberships_now_authorized
        || !forbidden_outputs.all_withheld()
    {
        return Err(GlobalE4V6Error::Invariant(
            "candidate-fresh-head Unknown firewall drifted".to_owned(),
        ));
    }

    let mut certificate = GlobalE4V6Certificate {
        schema: GLOBAL_E4_V6_SCHEMA.to_owned(),
        date: GLOBAL_E4_V6_DATE.to_owned(),
        source_bindings: source_bindings(&predecessor_bytes, &ambient_bytes),
        predecessor_global_e4_v5_digest: predecessor.result_digest,
        predecessor_global_e4_v5_replayed: true,
        ambient_former_internality_digest: ambient.result_digest,
        ambient_former_internality_replayed: true,
        global_e4_v6_executed: true,
        predecessor_application_unknown_resolved,
        next_candidate_fresh_head_unknown,
        unknown_survivor_count_lower_bound,
        fail_fast_catalog_scan_terminated,
        no_unknown_survives,
        class_exhaustion_proved,
        global_e4_complete,
        five_pending_memberships_now_authorized,
        forbidden_outputs,
        outcome: "global_e4_v6_resolved_application_next_candidate_fresh_head_unknown_blocks_exhaustion"
            .to_owned(),
        permitted_conclusion: "The exact [Univ,Lam(App(Lib(14),Lib(15)))] witness now earns Internal by replayed transparent-former closure with nu=0. The exact [Univ,Lam(App(Var(1),Lib(15)))] witness remains Unknown: even with clause 0 certified Internal, F-A3 excludes a candidate-fresh application head."
            .to_owned(),
        required_successor_action: "Continue the F-A5 ladder honestly. Keep the adopted F-A3 exclusion intact and adjudicate candidate-field-headed application as a distinct Internal rule, or leave it Unknown; only a replayable successor may authorize another global E-4 create-new rerun."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GlobalE4V6Replay {
    GlobalE4V6Replay {
        valid: false,
        rerun_executed: false,
        application_unknown_resolved: false,
        next_exact_unknown_found: false,
        candidate_fresh_head_gap: false,
        class_exhaustion_proved: false,
        global_e4_complete: false,
        pending_memberships_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "global_e4_v6_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &GlobalE4V6Certificate,
    expected: &GlobalE4V6Certificate,
) -> GlobalE4V6Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GlobalE4V6Replay {
        valid: errors.is_empty(),
        rerun_executed: certificate.global_e4_v6_executed,
        application_unknown_resolved: certificate
            .predecessor_application_unknown_resolved
            .classified_internal,
        next_exact_unknown_found: certificate
            .next_candidate_fresh_head_unknown
            .exact_raw_catalog_member
            && certificate.next_candidate_fresh_head_unknown.f_g4_retained,
        candidate_fresh_head_gap: certificate
            .next_candidate_fresh_head_unknown
            .f_a3_candidate_fresh_head_firewall_replayed,
        class_exhaustion_proved: certificate.class_exhaustion_proved,
        global_e4_complete: certificate.global_e4_complete,
        pending_memberships_authorized: certificate.five_pending_memberships_now_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_global_e4_v6_certificate(certificate: &GlobalE4V6Certificate) -> GlobalE4V6Replay {
    let expected = match issue_global_e4_v6_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_global_e4_v6_json(json: &str) -> GlobalE4V6Replay {
    match serde_json::from_str::<GlobalE4V6Certificate>(json) {
        Ok(certificate) => replay_global_e4_v6_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_global_e4_v6_create_new(path: &Path) -> Result<GlobalE4V6Replay, GlobalE4V6Error> {
    let certificate = issue_global_e4_v6_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GlobalE4V6Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GlobalE4V6Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GlobalE4V6Error::Io(error.to_string()))?;
    let replay = replay_global_e4_v6_certificate(&certificate);
    if !replay.valid {
        return Err(GlobalE4V6Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn application_witness_is_resolved_at_zero_credit() {
        let resolved = issue_resolved_application().expect("resolved application");
        assert!(resolved.exact_raw_catalog_member);
        assert!(resolved.classified_internal);
        assert!(resolved.application_stuckness_replayed);
        assert!(resolved.every_premise_internal);
        assert_eq!(resolved.marginal_nu, 0);
    }

    #[test]
    fn candidate_fresh_head_is_exact_and_remains_unknown() {
        let unknown =
            issue_next_candidate_fresh_head_unknown().expect("candidate-fresh-head Unknown");
        assert!(unknown.exact_raw_catalog_member);
        assert!(unknown.prior_clause_certificate_supplied);
        assert!(unknown.ambient_closure_refused);
        assert!(unknown.f_a3_candidate_fresh_head_firewall_replayed);
        assert_eq!(unknown.named_internal_gap, CANDIDATE_FRESH_HEAD_GAP);
        assert!(unknown.f_g4_retained);
    }

    #[test]
    fn v6_replay_and_mutations_fail_closed() {
        let certificate = issue_global_e4_v6_certificate().expect("v6 assembly");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut resolved = certificate.clone();
        resolved
            .predecessor_application_unknown_resolved
            .classified_internal = false;
        mutations.push(resolved);
        let mut survivor = certificate.clone();
        survivor
            .next_candidate_fresh_head_unknown
            .f_a3_candidate_fresh_head_firewall_replayed = false;
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
