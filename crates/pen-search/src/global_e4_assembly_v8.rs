//! Global E-4 v8 create-new rerun after adopting contextual internality.
//!
//! The rerun first corrects the v7 witness using the frozen absolute-level
//! derivation.  It then tests an exact raw candidate with a genuine ambient
//! head.  The explicit contextual judgment is replayable, but the raw
//! `Telescope` catalog does not carry its motive declaration, so the global
//! classifier must retain a named Unknown and stop under F-A5.

use crate::enumerate::{
    EnumerationContext, LateFamilySurface, RawSurfaceMembership, assess_raw_surface_membership,
};
use crate::global_e4_assembly_v7::{
    GLOBAL_E4_V7_SCHEMA, GlobalE4V7Certificate, replay_global_e4_v7_json,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_schema::internal_classifier_branch_v6::RawCandidateDecisionV7;
use pen_schema::internal_classifier_branch_v7::{
    CONTEXTUAL_INTERNALITY_SCHEMA, ContextualInternalityCertificate, RawCandidateDecisionV8,
    classify_raw_candidate_v8, classify_raw_candidate_v8_with_declaration,
    declared_context_control_candidate, declared_context_control_motives,
    replay_contextual_internality_json, v7_guarded_application_candidate,
};
use pen_type::contextual_internality::issue_ambient_context_declaration_token;
use pen_type::elaborate::{SealedSignature, elaborate_telescope};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GLOBAL_E4_V8_SCHEMA: &str = "schema2-global-e4-assembly-v8";
pub const GLOBAL_E4_V8_DATE: &str = "2026-07-21";
pub const RAW_CONTEXT_DECLARATION_GAP: &str = "RAW_TELESCOPE_AMBIENT_MOTIVE_DECLARATION_MISSING";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/contextual_internality_adjudication.md");
const ENUMERATOR_BYTES: &[u8] = include_bytes!("enumerate.rs");
const TOKEN_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/contextual_internality.rs");
const CLASSIFIER_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/internal_classifier_branch_v7.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v7.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v8.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V8SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedV7WitnessRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub predecessor_decision: RawCandidateDecisionV7,
    pub successor_decision: RawCandidateDecisionV8,
    pub exact_raw_catalog_member: bool,
    pub inferred_ambient_arity: u32,
    pub application_head_derivation_rule: String,
    pub ambient_use_count: usize,
    pub application_head_is_local_binder: bool,
    pub erased_base_internal: bool,
    pub per_clause_inverse_laws_replayed: bool,
    pub classified_internal: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LiveContextUnknownRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub raw_decision_without_declaration: RawCandidateDecisionV8,
    pub decision_with_explicit_declaration: RawCandidateDecisionV8,
    pub exact_raw_catalog_member: bool,
    pub inferred_ambient_arity: u32,
    pub ambient_head_derivation_rule: String,
    pub local_force_derivation_rule: String,
    pub genuine_live_ambient_use: bool,
    pub explicit_contextual_derivation_replays: bool,
    pub raw_telescope_has_motive_field: bool,
    pub named_gap: String,
    pub f_a5_retained: bool,
    pub f_g4_retained: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V8ForbiddenOutputs {
    pub five_pending_membership_verdicts_issued: bool,
    pub independent_family_tokens_issued: bool,
    pub e2b_executed: bool,
    pub historical_scores_recomputed: bool,
    pub fq2_evaluated: bool,
    pub agent_a_certified_totals_executed: bool,
    pub e5_guard_rail_f1_executed: bool,
    pub classifier_bridge_executed: bool,
    pub fork_executed: bool,
    pub halt_or_continuation_claimed: bool,
}

impl GlobalE4V8ForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.five_pending_membership_verdicts_issued
            && !self.independent_family_tokens_issued
            && !self.e2b_executed
            && !self.historical_scores_recomputed
            && !self.fq2_evaluated
            && !self.agent_a_certified_totals_executed
            && !self.e5_guard_rail_f1_executed
            && !self.classifier_bridge_executed
            && !self.fork_executed
            && !self.halt_or_continuation_claimed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V8Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GlobalE4V8SourceBinding>,
    pub predecessor_global_e4_v7_digest: String,
    pub predecessor_global_e4_v7_replayed: bool,
    pub contextual_internality_digest: String,
    pub contextual_internality_replayed: bool,
    pub global_e4_v8_executed: bool,
    pub predecessor_v7_witness_resolved: ResolvedV7WitnessRecord,
    pub next_live_context_unknown: LiveContextUnknownRecord,
    pub unknown_survivor_count_lower_bound: usize,
    pub fail_fast_catalog_scan_terminated: bool,
    pub no_unknown_survives: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub five_pending_memberships_now_authorized: bool,
    pub forbidden_outputs: GlobalE4V8ForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V8Replay {
    pub valid: bool,
    pub rerun_executed: bool,
    pub v7_witness_resolved: bool,
    pub next_exact_unknown_found: bool,
    pub contextual_rule_replays_with_declaration: bool,
    pub raw_context_declaration_gap: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub pending_memberships_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GlobalE4V8Error {
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
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V8_SCHEMA, domain, value))
        .expect("global E-4 v8 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(
    predecessor_bytes: &[u8],
    contextual_bytes: &[u8],
) -> Vec<GlobalE4V8SourceBinding> {
    [
        (
            "docs/schema2_global_e4_assembly_v7.json",
            "guarded_application_predecessor",
            predecessor_bytes,
        ),
        (
            "docs/schema2_contextual_internality_v1.json",
            "contextual_internality_successor",
            contextual_bytes,
        ),
        (
            "docs/contextual_internality_adjudication.md",
            "adopted_rule_and_f_a5",
            ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "exact_raw_catalog_membership",
            ENUMERATOR_BYTES,
        ),
        (
            "crates/pen-type/src/contextual_internality.rs",
            "motive_typed_contextual_token",
            TOKEN_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v7.rs",
            "contextual_classifier_and_raw_disconnect",
            CLASSIFIER_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v7.rs",
            "predecessor_fail_fast_assembly",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v8.rs",
            "live_context_fail_fast_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| GlobalE4V8SourceBinding {
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

fn sealed_v7_predecessor_valid(certificate: &GlobalE4V7Certificate, replay_valid: bool) -> bool {
    certificate.schema == GLOBAL_E4_V7_SCHEMA
        && replay_valid
        && certificate.global_e4_v7_executed
        && certificate
            .next_guarded_application_unknown
            .exact_raw_catalog_member
        && certificate.unknown_survivor_count_lower_bound >= 1
        && !certificate.no_unknown_survives
        && !certificate.class_exhaustion_proved
        && !certificate.global_e4_complete
        && !certificate.five_pending_memberships_now_authorized
}

fn issue_resolved_v7_witness() -> Result<ResolvedV7WitnessRecord, GlobalE4V8Error> {
    let candidate = v7_guarded_application_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let predecessor_decision =
        pen_schema::internal_classifier_branch_v6::classify_raw_candidate_v7(&candidate);
    let successor_decision = classify_raw_candidate_v8(&candidate);
    let signature = SealedSignature::genesis_del_h15();
    let elaboration = elaborate_telescope(&signature, &candidate, 15)
        .map_err(|error| GlobalE4V8Error::Invariant(error.to_string()))?;
    let application_head_derivation_rule = elaboration.clauses[1]
        .derivation
        .children
        .first()
        .and_then(|lambda_body| lambda_body.children.first())
        .map(|node| node.rule.clone())
        .unwrap_or_default();
    let inferred_ambient_arity = elaboration.ambient_parameters;
    let ambient_use_count =
        usize::from(application_head_derivation_rule.starts_with("ambient-param-"));
    let application_head_is_local_binder = application_head_derivation_rule == "local-var-1";
    let (erased_base_internal, per_clause_inverse_laws_replayed, classified_internal, marginal_nu) =
        match &successor_decision {
            RawCandidateDecisionV8::InternalGuardedClosure {
                certificate,
                marginal_nu,
                ..
            } => (
                certificate.base_candidate_internal,
                certificate.both_inverse_laws_replayed_per_clause,
                certificate.internal_certificate_issued,
                *marginal_nu,
            ),
            _ => (false, false, false, u32::MAX),
        };
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    if !matches!(
        predecessor_decision,
        RawCandidateDecisionV7::NamedTypedObstruction { .. }
    ) || !exact_raw_catalog_member
        || inferred_ambient_arity != 1
        || ambient_use_count != 0
        || !application_head_is_local_binder
        || !erased_base_internal
        || !per_clause_inverse_laws_replayed
        || !classified_internal
        || marginal_nu != 0
    {
        return Err(GlobalE4V8Error::Invariant(format!(
            "v7 witness correction failed: exact={exact_raw_catalog_member}, ambient={inferred_ambient_arity}, head={application_head_derivation_rule:?}, uses={ambient_use_count}, internal={classified_internal}, membership_rejections={:?}",
            raw_surface_membership.rejection_reasons,
        )));
    }
    let derivation_hash = tagged_hash(
        "resolved-v7-witness",
        &(
            &candidate,
            &raw_surface_membership,
            &predecessor_decision,
            &successor_decision,
            exact_raw_catalog_member,
            inferred_ambient_arity,
            &application_head_derivation_rule,
            ambient_use_count,
            application_head_is_local_binder,
            erased_base_internal,
            per_clause_inverse_laws_replayed,
            classified_internal,
            marginal_nu,
        ),
    );
    Ok(ResolvedV7WitnessRecord {
        candidate,
        raw_surface_membership,
        predecessor_decision,
        successor_decision,
        exact_raw_catalog_member,
        inferred_ambient_arity,
        application_head_derivation_rule,
        ambient_use_count,
        application_head_is_local_binder,
        erased_base_internal,
        per_clause_inverse_laws_replayed,
        classified_internal,
        marginal_nu,
        derivation_hash,
    })
}

fn find_rule(derivation: &pen_type::elaborate::DerivationNode, prefix: &str) -> Option<String> {
    if derivation.rule.starts_with(prefix) {
        return Some(derivation.rule.clone());
    }
    derivation
        .children
        .iter()
        .find_map(|child| find_rule(child, prefix))
}

fn issue_live_context_unknown() -> Result<LiveContextUnknownRecord, GlobalE4V8Error> {
    let candidate = declared_context_control_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let raw_decision_without_declaration = classify_raw_candidate_v8(&candidate);
    let signature = SealedSignature::genesis_del_h15();
    let declaration = issue_ambient_context_declaration_token(
        &signature,
        &candidate,
        15,
        declared_context_control_motives(),
    )
    .map_err(|error| GlobalE4V8Error::Invariant(error.to_string()))?;
    let decision_with_explicit_declaration =
        classify_raw_candidate_v8_with_declaration(&candidate, &declaration);
    let elaboration = elaborate_telescope(&signature, &candidate, 15)
        .map_err(|error| GlobalE4V8Error::Invariant(error.to_string()))?;
    let ambient_head_derivation_rule =
        find_rule(&elaboration.clauses[1].derivation, "ambient-param-").unwrap_or_default();
    let local_force_derivation_rule =
        find_rule(&elaboration.clauses[1].derivation, "local-var-").unwrap_or_default();
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    let inferred_ambient_arity = elaboration.ambient_parameters;
    let genuine_live_ambient_use = ambient_head_derivation_rule == "ambient-param-1";
    let explicit_contextual_derivation_replays = matches!(
        decision_with_explicit_declaration,
        RawCandidateDecisionV8::InternalContextual { .. }
    );
    let raw_telescope_has_motive_field = false;
    let named_gap = match &raw_decision_without_declaration {
        RawCandidateDecisionV8::NamedTypedObstruction {
            contextual_failure, ..
        } if contextual_failure.contains("no ambient-motive declaration") => {
            RAW_CONTEXT_DECLARATION_GAP.to_owned()
        }
        _ => String::new(),
    };
    let f_a5_retained = named_gap == RAW_CONTEXT_DECLARATION_GAP;
    let f_g4_retained = matches!(
        raw_decision_without_declaration,
        RawCandidateDecisionV8::NamedTypedObstruction { .. }
    );
    if !exact_raw_catalog_member
        || inferred_ambient_arity != 1
        || !genuine_live_ambient_use
        || local_force_derivation_rule != "local-var-1"
        || !explicit_contextual_derivation_replays
        || raw_telescope_has_motive_field
        || !f_a5_retained
        || !f_g4_retained
    {
        return Err(GlobalE4V8Error::Invariant(format!(
            "live-context disposition drifted: exact={exact_raw_catalog_member}, ambient={inferred_ambient_arity}, ambient_rule={ambient_head_derivation_rule:?}, local_rule={local_force_derivation_rule:?}, explicit={explicit_contextual_derivation_replays}, gap={named_gap:?}, membership_rejections={:?}, decision={raw_decision_without_declaration:?}",
            raw_surface_membership.rejection_reasons,
        )));
    }
    let derivation_hash = tagged_hash(
        "next-live-context-unknown",
        &(
            &candidate,
            &raw_surface_membership,
            &raw_decision_without_declaration,
            &decision_with_explicit_declaration,
            exact_raw_catalog_member,
            inferred_ambient_arity,
            &ambient_head_derivation_rule,
            &local_force_derivation_rule,
            genuine_live_ambient_use,
            explicit_contextual_derivation_replays,
            raw_telescope_has_motive_field,
            &named_gap,
            f_a5_retained,
            f_g4_retained,
        ),
    );
    Ok(LiveContextUnknownRecord {
        candidate,
        raw_surface_membership,
        raw_decision_without_declaration,
        decision_with_explicit_declaration,
        exact_raw_catalog_member,
        inferred_ambient_arity,
        ambient_head_derivation_rule,
        local_force_derivation_rule,
        genuine_live_ambient_use,
        explicit_contextual_derivation_replays,
        raw_telescope_has_motive_field,
        named_gap,
        f_a5_retained,
        f_g4_retained,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &GlobalE4V8Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("global-e4-v8-certificate", &projection)
}

pub fn issue_global_e4_v8_certificate() -> Result<GlobalE4V8Certificate, GlobalE4V8Error> {
    let predecessor_bytes = std::fs::read(workspace_doc_path("schema2_global_e4_assembly_v7.json"))
        .map_err(|error| GlobalE4V8Error::Io(error.to_string()))?;
    let predecessor_json = std::str::from_utf8(&predecessor_bytes)
        .map_err(|error| GlobalE4V8Error::Json(error.to_string()))?;
    let predecessor_replay = replay_global_e4_v7_json(predecessor_json);
    let predecessor: GlobalE4V7Certificate = serde_json::from_slice(&predecessor_bytes)
        .map_err(|error| GlobalE4V8Error::Json(error.to_string()))?;
    if !sealed_v7_predecessor_valid(&predecessor, predecessor_replay.valid) {
        return Err(GlobalE4V8Error::Prerequisite(
            "sealed global E-4 v7 replay or invariants failed".to_owned(),
        ));
    }

    let contextual_bytes =
        std::fs::read(workspace_doc_path("schema2_contextual_internality_v1.json"))
            .map_err(|error| GlobalE4V8Error::Io(error.to_string()))?;
    let contextual_json = std::str::from_utf8(&contextual_bytes)
        .map_err(|error| GlobalE4V8Error::Json(error.to_string()))?;
    let contextual_replay = replay_contextual_internality_json(contextual_json);
    if !contextual_replay.valid
        || !contextual_replay.v7_witness_resolved
        || !contextual_replay.declared_context_control_internal
        || !contextual_replay.motive_typing_replayed
        || !contextual_replay.instantiation_coherence_replayed
        || !contextual_replay.raw_surface_declaration_disconnect_exposed
        || !contextual_replay.falsifiers_replayed
        || contextual_replay.marginal_nu != 0
        || !contextual_replay.global_e4_v8_authorized
        || !contextual_replay.forbidden_outputs_withheld
    {
        return Err(GlobalE4V8Error::Prerequisite(format!(
            "contextual Internal replay failed: {}",
            contextual_replay.errors.join("; ")
        )));
    }
    let contextual: ContextualInternalityCertificate = serde_json::from_slice(&contextual_bytes)
        .map_err(|error| GlobalE4V8Error::Json(error.to_string()))?;
    if contextual.schema != CONTEXTUAL_INTERNALITY_SCHEMA {
        return Err(GlobalE4V8Error::Prerequisite(
            "contextual Internal schema mismatch".to_owned(),
        ));
    }

    let predecessor_v7_witness_resolved = issue_resolved_v7_witness()?;
    let next_live_context_unknown = issue_live_context_unknown()?;
    let forbidden_outputs = GlobalE4V8ForbiddenOutputs {
        five_pending_membership_verdicts_issued: false,
        independent_family_tokens_issued: false,
        e2b_executed: false,
        historical_scores_recomputed: false,
        fq2_evaluated: false,
        agent_a_certified_totals_executed: false,
        e5_guard_rail_f1_executed: false,
        classifier_bridge_executed: false,
        fork_executed: false,
        halt_or_continuation_claimed: false,
    };
    let unknown_survivor_count_lower_bound = 1;
    let fail_fast_catalog_scan_terminated = true;
    let no_unknown_survives = false;
    let class_exhaustion_proved = false;
    let global_e4_complete = false;
    let five_pending_memberships_now_authorized = false;
    if !predecessor_v7_witness_resolved.classified_internal
        || !next_live_context_unknown.f_a5_retained
        || !next_live_context_unknown.f_g4_retained
        || no_unknown_survives
        || class_exhaustion_proved
        || global_e4_complete
        || five_pending_memberships_now_authorized
        || !forbidden_outputs.all_withheld()
    {
        return Err(GlobalE4V8Error::Invariant(
            "v8 fail-fast firewall drifted".to_owned(),
        ));
    }
    let mut certificate = GlobalE4V8Certificate {
        schema: GLOBAL_E4_V8_SCHEMA.to_owned(),
        date: GLOBAL_E4_V8_DATE.to_owned(),
        source_bindings: source_bindings(&predecessor_bytes, &contextual_bytes),
        predecessor_global_e4_v7_digest: predecessor.result_digest,
        predecessor_global_e4_v7_replayed: true,
        contextual_internality_digest: contextual.result_digest,
        contextual_internality_replayed: true,
        global_e4_v8_executed: true,
        predecessor_v7_witness_resolved,
        next_live_context_unknown,
        unknown_survivor_count_lower_bound,
        fail_fast_catalog_scan_terminated,
        no_unknown_survives,
        class_exhaustion_proved,
        global_e4_complete,
        five_pending_memberships_now_authorized,
        forbidden_outputs,
        outcome: "global_e4_v8_resolves_v7_by_inverse_next_raw_context_declaration_unknown_blocks_exhaustion"
            .to_owned(),
        permitted_conclusion: "The sealed v7 witness earns Internal at nu=0, but its Var(3) head is local-var-1 and the proof is guarded weakening/erasure. The adopted contextual rule separately replays on an explicitly declared live context. An exact raw live-context candidate remains Unknown because the raw Telescope surface has no field carrying Gamma's motive."
            .to_owned(),
        required_successor_action: "Continue F-A5. Introduce a versioned raw-candidate wrapper whose ambient telescope stores ordered B15-formable motives, bind enumeration and classifier replay to that wrapper, and rerun v9. Do not infer or synthesize motives from a desired verdict."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GlobalE4V8Replay {
    GlobalE4V8Replay {
        valid: false,
        rerun_executed: false,
        v7_witness_resolved: false,
        next_exact_unknown_found: false,
        contextual_rule_replays_with_declaration: false,
        raw_context_declaration_gap: false,
        class_exhaustion_proved: false,
        global_e4_complete: false,
        pending_memberships_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "global_e4_v8_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &GlobalE4V8Certificate,
    expected: &GlobalE4V8Certificate,
) -> GlobalE4V8Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GlobalE4V8Replay {
        valid: errors.is_empty(),
        rerun_executed: certificate.global_e4_v8_executed,
        v7_witness_resolved: certificate
            .predecessor_v7_witness_resolved
            .classified_internal,
        next_exact_unknown_found: certificate
            .next_live_context_unknown
            .exact_raw_catalog_member
            && certificate.next_live_context_unknown.f_g4_retained,
        contextual_rule_replays_with_declaration: certificate
            .next_live_context_unknown
            .explicit_contextual_derivation_replays,
        raw_context_declaration_gap: certificate.next_live_context_unknown.named_gap
            == RAW_CONTEXT_DECLARATION_GAP,
        class_exhaustion_proved: certificate.class_exhaustion_proved,
        global_e4_complete: certificate.global_e4_complete,
        pending_memberships_authorized: certificate.five_pending_memberships_now_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_global_e4_v8_certificate(certificate: &GlobalE4V8Certificate) -> GlobalE4V8Replay {
    let expected = match issue_global_e4_v8_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_global_e4_v8_json(json: &str) -> GlobalE4V8Replay {
    match serde_json::from_str::<GlobalE4V8Certificate>(json) {
        Ok(certificate) => replay_global_e4_v8_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_global_e4_v8_create_new(path: &Path) -> Result<GlobalE4V8Replay, GlobalE4V8Error> {
    let certificate = issue_global_e4_v8_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GlobalE4V8Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GlobalE4V8Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GlobalE4V8Error::Io(error.to_string()))?;
    let replay = replay_global_e4_v8_certificate(&certificate);
    if !replay.valid {
        return Err(GlobalE4V8Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v7_witness_is_corrected_and_resolved() {
        let record = issue_resolved_v7_witness().expect("resolved v7 witness");
        assert!(record.exact_raw_catalog_member);
        assert_eq!(record.application_head_derivation_rule, "local-var-1");
        assert_eq!(record.ambient_use_count, 0);
        assert!(record.per_clause_inverse_laws_replayed);
        assert!(record.classified_internal);
        assert_eq!(record.marginal_nu, 0);
    }

    #[test]
    fn exact_live_context_candidate_exposes_raw_declaration_gap() {
        let record = issue_live_context_unknown().expect("live context Unknown");
        assert!(record.exact_raw_catalog_member);
        assert_eq!(record.ambient_head_derivation_rule, "ambient-param-1");
        assert_eq!(record.local_force_derivation_rule, "local-var-1");
        assert!(record.genuine_live_ambient_use);
        assert!(record.explicit_contextual_derivation_replays);
        assert!(!record.raw_telescope_has_motive_field);
        assert_eq!(record.named_gap, RAW_CONTEXT_DECLARATION_GAP);
        assert!(record.f_a5_retained);
        assert!(record.f_g4_retained);
    }

    #[test]
    fn v8_replay_and_mutations_fail_closed() {
        let certificate = issue_global_e4_v8_certificate().expect("v8 assembly");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut resolved = certificate.clone();
        resolved.predecessor_v7_witness_resolved.classified_internal = false;
        mutations.push(resolved);
        let mut gap = certificate.clone();
        gap.next_live_context_unknown.named_gap.clear();
        mutations.push(gap);
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
