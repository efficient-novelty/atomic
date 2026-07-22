//! Global E-4 create-new rerun after certified-field dereference.
//!
//! The assembly resolves the exact v6 survivor, then fail-fast checks a
//! genuinely ambient-dependent application head.  Dereference cannot erase
//! or certify an ambient parameter, and the existing guarded theorem applies
//! only to a weakening image whose inserted parameter is unused.

use crate::enumerate::{
    EnumerationContext, LateFamilySurface, RawSurfaceMembership, assess_raw_surface_membership,
};
use crate::global_e4_assembly_v6::{GLOBAL_E4_V6_SCHEMA, GlobalE4V6Certificate};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_schema::internal_classifier_branch_v2::RawCandidateDecisionV3;
use pen_schema::internal_classifier_branch_v3::{
    RawCandidateDecisionV4, one_ambient_identity_candidate,
};
use pen_schema::internal_classifier_branch_v4::RawCandidateDecisionV5;
use pen_schema::internal_classifier_branch_v5::{
    RawCandidateDecisionV6, candidate_fresh_head_candidate,
};
use pen_schema::internal_classifier_branch_v6::{
    CERTIFIED_FIELD_INTERNALITY_SCHEMA, CertifiedFieldClauseEvidence,
    CertifiedFieldInternalityCertificate, RawCandidateDecisionV7, classify_raw_candidate_v7,
    replay_certified_field_internality_json,
};
use pen_schema::total_classifier::{F_G4_TYPED_UNKNOWN, RawCandidateDecision};
use pen_type::ambient_former_internality::{
    AmbientFormerInternalityError, issue_ambient_former_closure_token,
};
use pen_type::certified_field_dereference::{
    CertifiedFieldDereferenceError, issue_certified_field_dereference_token,
};
use pen_type::elaborate::SealedSignature;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GLOBAL_E4_V7_SCHEMA: &str = "schema2-global-e4-assembly-v7";
pub const GLOBAL_E4_V7_DATE: &str = "2026-07-21";
pub const GUARDED_DEPENDENCY_GAP: &str =
    "INDUCTIVE_INTERNALITY_GUARDED_WEAKENING_ERASURE_INVERSE_LAWS_MISSING";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/certified_field_dereference_adjudication.md");
const ENUMERATOR_BYTES: &[u8] = include_bytes!("enumerate.rs");
const DEREFERENCE_TOKEN_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/certified_field_dereference.rs");
const CLASSIFIER_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/internal_classifier_branch_v6.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v6.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v7.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V7SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedCertifiedFieldRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub decision: RawCandidateDecisionV7,
    pub exact_raw_catalog_member: bool,
    pub classified_internal: bool,
    pub complete_referent_derivation_replayed: bool,
    pub dereference_evidence_count: usize,
    pub direct_judgment_identical: bool,
    pub no_credit_minted: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardedApplicationUnknownRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub decision: RawCandidateDecisionV7,
    pub exact_raw_catalog_member: bool,
    pub ambient_parameters: u32,
    pub ambient_parameter_occurs_in_body: bool,
    pub registered_guarded_identity_is_distinct: bool,
    pub ambient_closure_refused: bool,
    pub ambient_closure_refusal: String,
    pub dereference_closure_refused: bool,
    pub dereference_closure_refusal: String,
    pub guarded_inverse_laws_missing: bool,
    pub named_internal_gap: String,
    pub f_g4_retained: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V7ForbiddenOutputs {
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

impl GlobalE4V7ForbiddenOutputs {
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
pub struct GlobalE4V7Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GlobalE4V7SourceBinding>,
    pub predecessor_global_e4_v6_digest: String,
    pub predecessor_global_e4_v6_replayed: bool,
    pub certified_field_internality_digest: String,
    pub certified_field_internality_replayed: bool,
    pub global_e4_v7_executed: bool,
    pub predecessor_certified_field_unknown_resolved: ResolvedCertifiedFieldRecord,
    pub next_guarded_application_unknown: GuardedApplicationUnknownRecord,
    pub unknown_survivor_count_lower_bound: usize,
    pub fail_fast_catalog_scan_terminated: bool,
    pub no_unknown_survives: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub five_pending_memberships_now_authorized: bool,
    pub forbidden_outputs: GlobalE4V7ForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V7Replay {
    pub valid: bool,
    pub rerun_executed: bool,
    pub certified_field_unknown_resolved: bool,
    pub next_exact_unknown_found: bool,
    pub guarded_dependency_gap: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub pending_memberships_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GlobalE4V7Error {
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
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V7_SCHEMA, domain, value))
        .expect("global E-4 v7 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(
    predecessor_bytes: &[u8],
    dereference_bytes: &[u8],
) -> Vec<GlobalE4V7SourceBinding> {
    [
        (
            "docs/schema2_global_e4_assembly_v6.json",
            "candidate_fresh_head_unknown_predecessor",
            predecessor_bytes,
        ),
        (
            "docs/schema2_certified_field_dereference_v1.json",
            "certified_field_successor",
            dereference_bytes,
        ),
        (
            "docs/certified_field_dereference_adjudication.md",
            "adopted_rule_and_standing_guarded_firewall",
            ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "exact_raw_catalog_membership",
            ENUMERATOR_BYTES,
        ),
        (
            "crates/pen-type/src/certified_field_dereference.rs",
            "dereference_replay_and_guarded_refusal",
            DEREFERENCE_TOKEN_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v6.rs",
            "certified_field_internal_classifier",
            CLASSIFIER_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v6.rs",
            "predecessor_fail_fast_assembly",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v7.rs",
            "guarded_dependency_fail_fast_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| GlobalE4V7SourceBinding {
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

fn sealed_v6_predecessor_valid(certificate: &GlobalE4V6Certificate) -> bool {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V6_SCHEMA, "global-e4-v6-certificate", &projection))
        .expect("sealed v6 predecessor serializes");
    let expected_digest = format!("blake3:{}", blake3_hex(&bytes));
    certificate.schema == GLOBAL_E4_V6_SCHEMA
        && certificate.result_digest == expected_digest
        && certificate.global_e4_v6_executed
        && certificate
            .next_candidate_fresh_head_unknown
            .exact_raw_catalog_member
        && certificate
            .next_candidate_fresh_head_unknown
            .f_a3_candidate_fresh_head_firewall_replayed
        && certificate.unknown_survivor_count_lower_bound >= 1
        && !certificate.no_unknown_survives
        && !certificate.class_exhaustion_proved
        && !certificate.global_e4_complete
        && !certificate.five_pending_memberships_now_authorized
        && certificate.forbidden_outputs.all_withheld_for_successor()
}

trait V6ForbiddenReplay {
    fn all_withheld_for_successor(&self) -> bool;
}

impl V6ForbiddenReplay for crate::global_e4_assembly_v6::GlobalE4V6ForbiddenOutputs {
    fn all_withheld_for_successor(&self) -> bool {
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

pub fn guarded_application_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(3)),
                Box::new(Expr::Lib(15)),
            ))),
        ),
    ])
}

fn issue_resolved_certified_field() -> Result<ResolvedCertifiedFieldRecord, GlobalE4V7Error> {
    let candidate = candidate_fresh_head_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let decision = classify_raw_candidate_v7(&candidate);
    let (
        classified_internal,
        complete_referent_derivation_replayed,
        dereference_evidence_count,
        direct_judgment_identical,
        no_credit_minted,
        marginal_nu,
    ) = match &decision {
        RawCandidateDecisionV7::InternalCertifiedField {
            certificate,
            marginal_nu,
            ..
        } => (
            certificate.internal_certificate_issued,
            certificate.complete_referent_internal_derivations_retained,
            certificate
                .constructor_evidence
                .iter()
                .filter(|evidence| matches!(evidence, CertifiedFieldClauseEvidence::Dereference(_)))
                .count(),
            certificate.every_dereference_semantically_invisible,
            certificate.no_credit_anchor_or_family_minted,
            *marginal_nu,
        ),
        _ => (false, false, 0, false, false, u32::MAX),
    };
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    if !exact_raw_catalog_member
        || !classified_internal
        || !complete_referent_derivation_replayed
        || dereference_evidence_count != 1
        || !direct_judgment_identical
        || !no_credit_minted
        || marginal_nu != 0
    {
        return Err(GlobalE4V7Error::Invariant(
            "v6 certified-field Unknown was not resolved".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "resolved-certified-field",
        &(
            &candidate,
            &raw_surface_membership,
            &decision,
            exact_raw_catalog_member,
            classified_internal,
            complete_referent_derivation_replayed,
            dereference_evidence_count,
            direct_judgment_identical,
            no_credit_minted,
            marginal_nu,
        ),
    );
    Ok(ResolvedCertifiedFieldRecord {
        candidate,
        raw_surface_membership,
        decision,
        exact_raw_catalog_member,
        classified_internal,
        complete_referent_derivation_replayed,
        dereference_evidence_count,
        direct_judgment_identical,
        no_credit_minted,
        marginal_nu,
        derivation_hash,
    })
}

fn issue_next_guarded_application_unknown()
-> Result<GuardedApplicationUnknownRecord, GlobalE4V7Error> {
    let candidate = guarded_application_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let decision = classify_raw_candidate_v7(&candidate);
    let (ambient_parameters, named_internal_gap, f_g4_retained) = match &decision {
        RawCandidateDecisionV7::NamedTypedObstruction {
            predecessor_decision:
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
                },
        } => (
            1,
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
    let ambient_refusal =
        issue_ambient_former_closure_token(&signature, &candidate, 15, 1, &BTreeMap::new())
            .expect_err("genuinely guarded application must not enter closed ambient closure");
    let ambient_closure_refused =
        ambient_refusal == AmbientFormerInternalityError::GuardedCandidate { ambient: 1 };
    let ambient_closure_refusal = ambient_refusal.to_string();
    let dereference_refusal =
        issue_certified_field_dereference_token(&signature, &candidate, 15, 1, &BTreeMap::new())
            .expect_err("dereference must preserve the guarded inverse-law requirement");
    let dereference_closure_refused =
        dereference_refusal == CertifiedFieldDereferenceError::GuardedCandidate { ambient: 1 };
    let dereference_closure_refusal = dereference_refusal.to_string();
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    let ambient_parameter_occurs_in_body = true;
    let registered_guarded_identity_is_distinct = candidate != one_ambient_identity_candidate();
    let guarded_inverse_laws_missing = named_internal_gap == GUARDED_DEPENDENCY_GAP;
    if !exact_raw_catalog_member
        || ambient_parameters != 1
        || !ambient_parameter_occurs_in_body
        || !registered_guarded_identity_is_distinct
        || !ambient_closure_refused
        || !dereference_closure_refused
        || !guarded_inverse_laws_missing
        || !f_g4_retained
    {
        return Err(GlobalE4V7Error::Invariant(format!(
            "guarded-application disposition drifted: exact={exact_raw_catalog_member}, ambient={ambient_parameters}, distinct={registered_guarded_identity_is_distinct}, ambient_refused={ambient_closure_refused}, deref_refused={dereference_closure_refused}, gap={named_internal_gap:?}, f_g4={f_g4_retained}, membership_rejections={:?}, decision={decision:?}",
            raw_surface_membership.rejection_reasons,
        )));
    }
    let derivation_hash = tagged_hash(
        "next-guarded-application-unknown",
        &(
            &candidate,
            &raw_surface_membership,
            &decision,
            exact_raw_catalog_member,
            ambient_parameters,
            ambient_parameter_occurs_in_body,
            registered_guarded_identity_is_distinct,
            ambient_closure_refused,
            &ambient_closure_refusal,
            dereference_closure_refused,
            &dereference_closure_refusal,
            guarded_inverse_laws_missing,
            &named_internal_gap,
            f_g4_retained,
        ),
    );
    Ok(GuardedApplicationUnknownRecord {
        candidate,
        raw_surface_membership,
        decision,
        exact_raw_catalog_member,
        ambient_parameters,
        ambient_parameter_occurs_in_body,
        registered_guarded_identity_is_distinct,
        ambient_closure_refused,
        ambient_closure_refusal,
        dereference_closure_refused,
        dereference_closure_refusal,
        guarded_inverse_laws_missing,
        named_internal_gap,
        f_g4_retained,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &GlobalE4V7Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("global-e4-v7-certificate", &projection)
}

pub fn issue_global_e4_v7_certificate() -> Result<GlobalE4V7Certificate, GlobalE4V7Error> {
    let predecessor_bytes = std::fs::read(workspace_doc_path("schema2_global_e4_assembly_v6.json"))
        .map_err(|error| GlobalE4V7Error::Io(error.to_string()))?;
    let predecessor: GlobalE4V6Certificate = serde_json::from_slice(&predecessor_bytes)
        .map_err(|error| GlobalE4V7Error::Json(error.to_string()))?;
    if !sealed_v6_predecessor_valid(&predecessor) {
        return Err(GlobalE4V7Error::Prerequisite(
            "sealed global E-4 v6 digest or required invariants failed".to_owned(),
        ));
    }

    let dereference_bytes = std::fs::read(workspace_doc_path(
        "schema2_certified_field_dereference_v1.json",
    ))
    .map_err(|error| GlobalE4V7Error::Io(error.to_string()))?;
    let dereference_replay = replay_certified_field_internality_json(
        std::str::from_utf8(&dereference_bytes)
            .map_err(|error| GlobalE4V7Error::Json(error.to_string()))?,
    );
    if !dereference_replay.valid
        || !dereference_replay.candidate_internal
        || !dereference_replay.dereference_replayed
        || !dereference_replay.direct_judgment_identical
        || !dereference_replay.falsifiers_replayed
        || dereference_replay.marginal_nu != 0
        || !dereference_replay.global_e4_v7_authorized
        || !dereference_replay.forbidden_outputs_withheld
    {
        return Err(GlobalE4V7Error::Prerequisite(format!(
            "certified-field internality replay failed: {}",
            dereference_replay.errors.join("; ")
        )));
    }
    let dereference: CertifiedFieldInternalityCertificate =
        serde_json::from_slice(&dereference_bytes)
            .map_err(|error| GlobalE4V7Error::Json(error.to_string()))?;
    if dereference.schema != CERTIFIED_FIELD_INTERNALITY_SCHEMA {
        return Err(GlobalE4V7Error::Prerequisite(
            "certified-field schema mismatch".to_owned(),
        ));
    }

    let predecessor_certified_field_unknown_resolved = issue_resolved_certified_field()?;
    let next_guarded_application_unknown = issue_next_guarded_application_unknown()?;
    let forbidden_outputs = GlobalE4V7ForbiddenOutputs {
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
    if !predecessor_certified_field_unknown_resolved.classified_internal
        || !next_guarded_application_unknown.guarded_inverse_laws_missing
        || !next_guarded_application_unknown.f_g4_retained
        || no_unknown_survives
        || class_exhaustion_proved
        || global_e4_complete
        || five_pending_memberships_now_authorized
        || !forbidden_outputs.all_withheld()
    {
        return Err(GlobalE4V7Error::Invariant(
            "guarded application Unknown firewall drifted".to_owned(),
        ));
    }

    let mut certificate = GlobalE4V7Certificate {
        schema: GLOBAL_E4_V7_SCHEMA.to_owned(),
        date: GLOBAL_E4_V7_DATE.to_owned(),
        source_bindings: source_bindings(&predecessor_bytes, &dereference_bytes),
        predecessor_global_e4_v6_digest: predecessor.result_digest,
        predecessor_global_e4_v6_replayed: true,
        certified_field_internality_digest: dereference.result_digest,
        certified_field_internality_replayed: true,
        global_e4_v7_executed: true,
        predecessor_certified_field_unknown_resolved,
        next_guarded_application_unknown,
        unknown_survivor_count_lower_bound,
        fail_fast_catalog_scan_terminated,
        no_unknown_survives,
        class_exhaustion_proved,
        global_e4_complete,
        five_pending_memberships_now_authorized,
        forbidden_outputs,
        outcome: "global_e4_v7_resolved_certified_field_next_guarded_dependency_unknown_blocks_exhaustion"
            .to_owned(),
        permitted_conclusion: "The exact certified-field-head witness now earns Internal by semantically invisible dereference with nu=0. The exact [Univ,Lam(App(Var(3),Lib(15)))] witness remains Unknown: Var(3) is a live ambient dependency, not a certified candidate field and not the unused parameter of the registered guarded identity weakening image."
            .to_owned(),
        required_successor_action: "Continue F-A5 honestly. Decide whether genuinely ambient-dependent families belong to D(B15). A positive successor needs a new guarded theorem with motive-sensitive weakening/erasure or another explicit inverse principle; certified-field dereference itself cannot discharge this obligation. Then rerun global E-4 create-new."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GlobalE4V7Replay {
    GlobalE4V7Replay {
        valid: false,
        rerun_executed: false,
        certified_field_unknown_resolved: false,
        next_exact_unknown_found: false,
        guarded_dependency_gap: false,
        class_exhaustion_proved: false,
        global_e4_complete: false,
        pending_memberships_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "global_e4_v7_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &GlobalE4V7Certificate,
    expected: &GlobalE4V7Certificate,
) -> GlobalE4V7Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GlobalE4V7Replay {
        valid: errors.is_empty(),
        rerun_executed: certificate.global_e4_v7_executed,
        certified_field_unknown_resolved: certificate
            .predecessor_certified_field_unknown_resolved
            .classified_internal,
        next_exact_unknown_found: certificate
            .next_guarded_application_unknown
            .exact_raw_catalog_member
            && certificate.next_guarded_application_unknown.f_g4_retained,
        guarded_dependency_gap: certificate
            .next_guarded_application_unknown
            .guarded_inverse_laws_missing,
        class_exhaustion_proved: certificate.class_exhaustion_proved,
        global_e4_complete: certificate.global_e4_complete,
        pending_memberships_authorized: certificate.five_pending_memberships_now_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_global_e4_v7_certificate(certificate: &GlobalE4V7Certificate) -> GlobalE4V7Replay {
    let expected = match issue_global_e4_v7_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_global_e4_v7_json(json: &str) -> GlobalE4V7Replay {
    match serde_json::from_str::<GlobalE4V7Certificate>(json) {
        Ok(certificate) => replay_global_e4_v7_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_global_e4_v7_create_new(path: &Path) -> Result<GlobalE4V7Replay, GlobalE4V7Error> {
    let certificate = issue_global_e4_v7_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GlobalE4V7Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GlobalE4V7Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GlobalE4V7Error::Io(error.to_string()))?;
    let replay = replay_global_e4_v7_certificate(&certificate);
    if !replay.valid {
        return Err(GlobalE4V7Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn certified_field_witness_is_resolved_at_zero_credit() {
        let resolved = issue_resolved_certified_field().expect("resolved certified field");
        assert!(resolved.exact_raw_catalog_member);
        assert!(resolved.classified_internal);
        assert!(resolved.complete_referent_derivation_replayed);
        assert!(resolved.direct_judgment_identical);
        assert_eq!(resolved.marginal_nu, 0);
    }

    #[test]
    fn guarded_application_is_exact_and_remains_unknown() {
        let unknown = issue_next_guarded_application_unknown().expect("guarded Unknown");
        assert!(unknown.exact_raw_catalog_member);
        assert_eq!(unknown.ambient_parameters, 1);
        assert!(unknown.ambient_parameter_occurs_in_body);
        assert!(unknown.registered_guarded_identity_is_distinct);
        assert!(unknown.ambient_closure_refused);
        assert!(unknown.dereference_closure_refused);
        assert!(unknown.guarded_inverse_laws_missing);
        assert!(unknown.f_g4_retained);
    }

    #[test]
    fn v7_replay_and_mutations_fail_closed() {
        let certificate = issue_global_e4_v7_certificate().expect("v7 assembly");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut resolved = certificate.clone();
        resolved
            .predecessor_certified_field_unknown_resolved
            .classified_internal = false;
        mutations.push(resolved);
        let mut survivor = certificate.clone();
        survivor
            .next_guarded_application_unknown
            .guarded_inverse_laws_missing = false;
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
