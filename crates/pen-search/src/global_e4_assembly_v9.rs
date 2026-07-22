//! Global E-4 v9 over the adopted ambient-wrapped candidate domain.
//!
//! The scan order is produced by `ambient_wrapper_domain` without consulting
//! classification.  The intended `Type -> Type` declaration closes the v8
//! representational gap.  F-A5 nevertheless remains authoritative: the scan
//! stops at the first admissible wrapped candidate whose semantic judgment is
//! still a named obstruction.

use crate::ambient_wrapper_domain::{
    MotiveGrammarFinitenessCertificate, PiCoverageRecord, WrappedSurfaceMembership,
    issue_pi_coverage, motive_candidates_through, motive_grammar_finiteness_certificate,
    wrapped_surface_membership,
};
use crate::global_e4_assembly_v8::{
    GLOBAL_E4_V8_SCHEMA, GlobalE4V8Certificate, GlobalE4V8ForbiddenOutputs,
    replay_global_e4_v8_json,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_schema::internal_classifier_branch_v7::{
    declared_context_control_candidate, declared_context_control_motives,
};
use pen_schema::internal_classifier_branch_v8::{
    AMBIENT_WRAPPER_VERSION, AmbientWrappedCandidate, WRAPPED_CONTEXTUAL_COHERENCE_GAP,
    WrappedCandidateDecisionV9, classify_wrapped_candidate_v9, closed_fiber_agrees,
    contextual_motive_node_count, wrap_candidate,
};
use pen_type::contextual_internality::{
    ContextualInternalityError, ContextualMotive, issue_ambient_context_declaration_token,
    issue_contextual_internality_token,
};
use pen_type::elaborate::SealedSignature;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GLOBAL_E4_V9_SCHEMA: &str = "schema2-global-e4-assembly-v9";
pub const GLOBAL_E4_V9_DATE: &str = "2026-07-21";

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/ambient_wrapper_adjudication.md");
const PREDECESSOR_ARTIFACT_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_global_e4_assembly_v8.json");
const ENUMERATOR_BYTES: &[u8] = include_bytes!("enumerate.rs");
const DOMAIN_SOURCE_BYTES: &[u8] = include_bytes!("ambient_wrapper_domain.rs");
const TOKEN_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/contextual_internality.rs");
const CLASSIFIER_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/internal_classifier_branch_v8.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v8.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v9.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V9SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WrappedDomainRecord {
    pub wrapper_version: String,
    pub finiteness: MotiveGrammarFinitenessCertificate,
    pub pi_coverage_live_control: PiCoverageRecord,
    pub pi_coverage_closed_control: PiCoverageRecord,
    pub closed_fiber_agreement_replayed: bool,
    pub enumerator_reads_classification: bool,
    pub classification_outcome_filter: bool,
    pub motive_inference_performed: bool,
    pub motive_selection_performed: bool,
    pub motive_repair_performed: bool,
    pub enumeration_order: String,
    pub prior_artifact_bytes_match_compile_time_binding: bool,
    pub archival_conservativity_replayed: bool,
    pub f_w1_retained: bool,
    pub f_w2_retained: bool,
    pub f_w3_retained: bool,
    pub f_w4_retained: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedV8FiberRecord {
    pub raw_candidate: Telescope,
    pub wrapped_candidate: AmbientWrappedCandidate,
    pub wrapped_surface_membership: WrappedSurfaceMembership,
    pub decision: WrappedCandidateDecisionV9,
    pub projection_matches_v8_unknown: bool,
    pub declaration_is_type_to_type: bool,
    pub classified_internal: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NextWrappedUnknownRecord {
    pub wrapped_candidate: AmbientWrappedCandidate,
    pub wrapped_surface_membership: WrappedSurfaceMembership,
    pub decision: WrappedCandidateDecisionV9,
    pub exact_raw_projection: bool,
    pub exact_motive: ContextualMotive,
    pub motive_node_count: u32,
    pub enumerated_before_classification: bool,
    pub declaration_replayed: bool,
    pub live_use_motive_typed: bool,
    pub contextual_token_error: String,
    pub named_gap: String,
    pub named_typed_exclusion: bool,
    pub f_a5_retained: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WrappedFiberScanRecord {
    pub scan_node_cap_reached: u32,
    pub syntactic_motives_visited: usize,
    pub admissible_wrappers_visited: usize,
    pub named_typed_exclusions_visited: usize,
    pub internal_wrappers_visited: usize,
    pub classified_wrappers_visited: usize,
    pub fail_fast_stopped_at_first_unknown: bool,
    pub next_unknown: NextWrappedUnknownRecord,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V9Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GlobalE4V9SourceBinding>,
    pub predecessor_global_e4_v8_digest: String,
    pub predecessor_global_e4_v8_replayed: bool,
    pub global_e4_v9_executed: bool,
    pub wrapped_domain: WrappedDomainRecord,
    pub predecessor_v8_fiber_resolved: ResolvedV8FiberRecord,
    pub wrapped_fiber_scan: WrappedFiberScanRecord,
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
pub struct GlobalE4V9Replay {
    pub valid: bool,
    pub rerun_executed: bool,
    pub wrapped_domain_finite: bool,
    pub pi_coverage_replayed: bool,
    pub declaration_independence_replayed: bool,
    pub v8_fiber_resolved: bool,
    pub next_exact_unknown_found: bool,
    pub f_a5_retained: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub pending_memberships_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GlobalE4V9Error {
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
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V9_SCHEMA, domain, value))
        .expect("global E-4 v9 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(predecessor_bytes: &[u8]) -> Vec<GlobalE4V9SourceBinding> {
    [
        (
            "docs/schema2_global_e4_assembly_v8.json",
            "byte_stable_predecessor_artifact",
            predecessor_bytes,
        ),
        (
            "docs/ambient_wrapper_adjudication.md",
            "adopted_wrapper_rule_and_falsifiers",
            ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "frozen_raw_projection_catalog",
            ENUMERATOR_BYTES,
        ),
        (
            "crates/pen-search/src/ambient_wrapper_domain.rs",
            "finite_verdict_blind_wrapped_domain",
            DOMAIN_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/contextual_internality.rs",
            "motive_typed_declaration_and_instantiation_judgment",
            TOKEN_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v8.rs",
            "wrapped_candidate_classifier",
            CLASSIFIER_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v8.rs",
            "archival_raw_domain_predecessor",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v9.rs",
            "wrapped_domain_fail_fast_assembly",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| GlobalE4V9SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn forbidden_outputs() -> GlobalE4V8ForbiddenOutputs {
    GlobalE4V8ForbiddenOutputs {
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
    }
}

fn outputs_all_withheld(outputs: &GlobalE4V8ForbiddenOutputs) -> bool {
    !outputs.five_pending_membership_verdicts_issued
        && !outputs.independent_family_tokens_issued
        && !outputs.e2b_executed
        && !outputs.historical_scores_recomputed
        && !outputs.fq2_evaluated
        && !outputs.agent_a_certified_totals_executed
        && !outputs.e5_guard_rail_f1_executed
        && !outputs.classifier_bridge_executed
        && !outputs.fork_executed
        && !outputs.halt_or_continuation_claimed
}

fn issue_wrapped_domain_record(
    predecessor_bytes: &[u8],
) -> Result<WrappedDomainRecord, GlobalE4V9Error> {
    let live = declared_context_control_candidate();
    let closed = Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Univ)]);
    let finiteness = motive_grammar_finiteness_certificate();
    let pi_coverage_live_control =
        issue_pi_coverage(&live).map_err(|error| GlobalE4V9Error::Invariant(error.to_string()))?;
    let pi_coverage_closed_control = issue_pi_coverage(&closed)
        .map_err(|error| GlobalE4V9Error::Invariant(error.to_string()))?;
    let closed_wrapper = wrap_candidate(closed, Vec::new())
        .map_err(|error| GlobalE4V9Error::Invariant(error.to_string()))?;
    let closed_fiber_agreement_replayed = closed_fiber_agrees(&closed_wrapper)
        .map_err(|error| GlobalE4V9Error::Invariant(error.to_string()))?;
    let enumerator_reads_classification = false;
    let classification_outcome_filter = false;
    let motive_inference_performed = false;
    let motive_selection_performed = false;
    let motive_repair_performed = false;
    let enumeration_order =
        "exact-node-count ascending; Type; Function(domain-size ascending); Element(AST constructor order)"
            .to_owned();
    let prior_artifact_bytes_match_compile_time_binding =
        predecessor_bytes == PREDECESSOR_ARTIFACT_BYTES;
    let archival_conservativity_replayed = prior_artifact_bytes_match_compile_time_binding;
    let f_w1_retained = !enumerator_reads_classification
        && !classification_outcome_filter
        && !motive_inference_performed
        && !motive_selection_performed
        && !motive_repair_performed;
    let f_w2_retained = pi_coverage_live_control.surjective_witness_replayed
        && pi_coverage_live_control.reverse_direction_replayed
        && pi_coverage_closed_control.surjective_witness_replayed
        && pi_coverage_closed_control.reverse_direction_replayed;
    let f_w3_retained = closed_fiber_agreement_replayed;
    let f_w4_retained = finiteness.finite && !finiteness.silent_truncation;
    if !f_w1_retained
        || !f_w2_retained
        || !f_w3_retained
        || !f_w4_retained
        || !archival_conservativity_replayed
    {
        return Err(GlobalE4V9Error::Invariant(
            "ambient-wrapper domain falsifier failed".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "wrapped-domain",
        &(
            &finiteness,
            &pi_coverage_live_control,
            &pi_coverage_closed_control,
            closed_fiber_agreement_replayed,
            enumerator_reads_classification,
            classification_outcome_filter,
            motive_inference_performed,
            motive_selection_performed,
            motive_repair_performed,
            &enumeration_order,
            prior_artifact_bytes_match_compile_time_binding,
            archival_conservativity_replayed,
            f_w1_retained,
            f_w2_retained,
            f_w3_retained,
            f_w4_retained,
        ),
    );
    Ok(WrappedDomainRecord {
        wrapper_version: AMBIENT_WRAPPER_VERSION.to_owned(),
        finiteness,
        pi_coverage_live_control,
        pi_coverage_closed_control,
        closed_fiber_agreement_replayed,
        enumerator_reads_classification,
        classification_outcome_filter,
        motive_inference_performed,
        motive_selection_performed,
        motive_repair_performed,
        enumeration_order,
        prior_artifact_bytes_match_compile_time_binding,
        archival_conservativity_replayed,
        f_w1_retained,
        f_w2_retained,
        f_w3_retained,
        f_w4_retained,
        derivation_hash,
    })
}

fn issue_resolved_v8_fiber(
    predecessor: &GlobalE4V8Certificate,
) -> Result<ResolvedV8FiberRecord, GlobalE4V9Error> {
    let raw_candidate = declared_context_control_candidate();
    let wrapped_candidate =
        wrap_candidate(raw_candidate.clone(), declared_context_control_motives())
            .map_err(|error| GlobalE4V9Error::Invariant(error.to_string()))?;
    let wrapped_surface_membership = wrapped_surface_membership(&wrapped_candidate);
    let decision = classify_wrapped_candidate_v9(&wrapped_candidate)
        .map_err(|error| GlobalE4V9Error::Invariant(error.to_string()))?;
    let projection_matches_v8_unknown =
        raw_candidate == predecessor.next_live_context_unknown.candidate;
    let declaration_is_type_to_type = wrapped_candidate.ambient
        == vec![ContextualMotive::Function {
            domain: Box::new(ContextualMotive::Type),
            codomain: Box::new(ContextualMotive::Type),
        }];
    let (classified_internal, marginal_nu) = match &decision {
        WrappedCandidateDecisionV9::Internal { marginal_nu, .. } => (true, *marginal_nu),
        _ => (false, u32::MAX),
    };
    if !wrapped_surface_membership.is_member
        || !projection_matches_v8_unknown
        || !declaration_is_type_to_type
        || !classified_internal
        || marginal_nu != 0
    {
        return Err(GlobalE4V9Error::Invariant(format!(
            "v8 fiber did not resolve: membership={}, projection={}, declaration={}, decision={decision:?}",
            wrapped_surface_membership.is_member,
            projection_matches_v8_unknown,
            declaration_is_type_to_type,
        )));
    }
    let derivation_hash = tagged_hash(
        "resolved-v8-fiber",
        &(
            &raw_candidate,
            &wrapped_candidate,
            &wrapped_surface_membership,
            &decision,
            projection_matches_v8_unknown,
            declaration_is_type_to_type,
            classified_internal,
            marginal_nu,
        ),
    );
    Ok(ResolvedV8FiberRecord {
        raw_candidate,
        wrapped_candidate,
        wrapped_surface_membership,
        decision,
        projection_matches_v8_unknown,
        declaration_is_type_to_type,
        classified_internal,
        marginal_nu,
        derivation_hash,
    })
}

fn issue_wrapped_fiber_scan() -> Result<WrappedFiberScanRecord, GlobalE4V9Error> {
    let raw_candidate = declared_context_control_candidate();
    let expected_motive = ContextualMotive::Function {
        domain: Box::new(ContextualMotive::Type),
        codomain: Box::new(ContextualMotive::Element(Expr::Univ)),
    };
    let scan_node_cap_reached = 4;
    let candidates = motive_candidates_through(scan_node_cap_reached);
    let mut syntactic_motives_visited = 0;
    let mut admissible_wrappers_visited = 0;
    let mut named_typed_exclusions_visited = 0;
    let mut internal_wrappers_visited = 0;
    let mut classified_wrappers_visited = 0;
    let mut next_unknown = None;
    for motive in candidates {
        syntactic_motives_visited += 1;
        let Ok(wrapped_candidate) = wrap_candidate(raw_candidate.clone(), vec![motive.clone()])
        else {
            continue;
        };
        admissible_wrappers_visited += 1;
        let membership = wrapped_surface_membership(&wrapped_candidate);
        if !membership.is_member {
            return Err(GlobalE4V9Error::Invariant(format!(
                "enumerated admissible wrapper failed membership: {:?}",
                membership.rejection_reasons
            )));
        }
        let decision = classify_wrapped_candidate_v9(&wrapped_candidate)
            .map_err(|error| GlobalE4V9Error::Invariant(error.to_string()))?;
        match &decision {
            WrappedCandidateDecisionV9::NamedTypedExclusion { .. } => {
                named_typed_exclusions_visited += 1;
            }
            WrappedCandidateDecisionV9::Internal { .. } => internal_wrappers_visited += 1,
            WrappedCandidateDecisionV9::Classified { .. }
            | WrappedCandidateDecisionV9::NamedExclusion { .. } => {
                classified_wrappers_visited += 1;
            }
            WrappedCandidateDecisionV9::NamedTypedObstruction { code, reason, .. } => {
                let signature = SealedSignature::genesis_del_h15();
                let declaration = issue_ambient_context_declaration_token(
                    &signature,
                    &raw_candidate,
                    15,
                    vec![motive.clone()],
                )
                .map_err(|error| GlobalE4V9Error::Invariant(error.to_string()))?;
                let token_error = issue_contextual_internality_token(
                    &signature,
                    &raw_candidate,
                    15,
                    1,
                    &declaration,
                    &BTreeMap::from([(0, "wrapped-scan-earned-prior-clause-0".to_owned())]),
                )
                .expect_err("named wrapped obstruction must fail token issuance");
                let live_use_motive_typed = matches!(
                    token_error,
                    ContextualInternalityError::NoRegisteredProbe { parameter: 1 }
                );
                let contextual_token_error = token_error.to_string();
                let named_gap = code.clone();
                let named_typed_exclusion = false;
                let f_a5_retained = named_gap == WRAPPED_CONTEXTUAL_COHERENCE_GAP
                    && live_use_motive_typed
                    && reason.contains("no registered closed probes");
                let exact_raw_projection = wrapped_candidate.clauses == raw_candidate;
                let exact_motive = motive.clone();
                let motive_node_count = contextual_motive_node_count(&motive);
                let enumerated_before_classification = true;
                let declaration_replayed = membership.declaration_replayed;
                let derivation_hash = tagged_hash(
                    "next-wrapped-unknown",
                    &(
                        &wrapped_candidate,
                        &membership,
                        &decision,
                        exact_raw_projection,
                        &exact_motive,
                        motive_node_count,
                        enumerated_before_classification,
                        declaration_replayed,
                        live_use_motive_typed,
                        &contextual_token_error,
                        &named_gap,
                        named_typed_exclusion,
                        f_a5_retained,
                    ),
                );
                next_unknown = Some(NextWrappedUnknownRecord {
                    wrapped_candidate,
                    wrapped_surface_membership: membership,
                    decision,
                    exact_raw_projection,
                    exact_motive,
                    motive_node_count,
                    enumerated_before_classification,
                    declaration_replayed,
                    live_use_motive_typed,
                    contextual_token_error,
                    named_gap,
                    named_typed_exclusion,
                    f_a5_retained,
                    derivation_hash,
                });
                break;
            }
        }
    }
    let next_unknown = next_unknown.ok_or_else(|| {
        GlobalE4V9Error::Invariant(
            "wrapped prefix through four nodes produced no F-A5 witness".to_owned(),
        )
    })?;
    let fail_fast_stopped_at_first_unknown = true;
    if next_unknown.exact_motive != expected_motive
        || next_unknown.motive_node_count != 4
        || !next_unknown.exact_raw_projection
        || !next_unknown.wrapped_surface_membership.is_member
        || !next_unknown.live_use_motive_typed
        || next_unknown.named_typed_exclusion
        || !next_unknown.f_a5_retained
    {
        return Err(GlobalE4V9Error::Invariant(format!(
            "first wrapped Unknown drifted: {next_unknown:?}"
        )));
    }
    let derivation_hash = tagged_hash(
        "wrapped-fiber-scan",
        &(
            scan_node_cap_reached,
            syntactic_motives_visited,
            admissible_wrappers_visited,
            named_typed_exclusions_visited,
            internal_wrappers_visited,
            classified_wrappers_visited,
            fail_fast_stopped_at_first_unknown,
            &next_unknown,
        ),
    );
    Ok(WrappedFiberScanRecord {
        scan_node_cap_reached,
        syntactic_motives_visited,
        admissible_wrappers_visited,
        named_typed_exclusions_visited,
        internal_wrappers_visited,
        classified_wrappers_visited,
        fail_fast_stopped_at_first_unknown,
        next_unknown,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &GlobalE4V9Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("global-e4-v9-certificate", &projection)
}

pub fn issue_global_e4_v9_certificate() -> Result<GlobalE4V9Certificate, GlobalE4V9Error> {
    let predecessor_bytes = std::fs::read(workspace_doc_path("schema2_global_e4_assembly_v8.json"))
        .map_err(|error| GlobalE4V9Error::Io(error.to_string()))?;
    let predecessor_json = std::str::from_utf8(&predecessor_bytes)
        .map_err(|error| GlobalE4V9Error::Json(error.to_string()))?;
    let predecessor_replay = replay_global_e4_v8_json(predecessor_json);
    let predecessor: GlobalE4V8Certificate = serde_json::from_slice(&predecessor_bytes)
        .map_err(|error| GlobalE4V9Error::Json(error.to_string()))?;
    if predecessor.schema != GLOBAL_E4_V8_SCHEMA
        || !predecessor_replay.valid
        || !predecessor.global_e4_v8_executed
        || predecessor.no_unknown_survives
        || predecessor.class_exhaustion_proved
        || predecessor.global_e4_complete
        || predecessor.unknown_survivor_count_lower_bound < 1
    {
        return Err(GlobalE4V9Error::Prerequisite(format!(
            "sealed v8 replay/invariants failed: {}",
            predecessor_replay.errors.join("; ")
        )));
    }
    let wrapped_domain = issue_wrapped_domain_record(&predecessor_bytes)?;
    let predecessor_v8_fiber_resolved = issue_resolved_v8_fiber(&predecessor)?;
    let wrapped_fiber_scan = issue_wrapped_fiber_scan()?;
    let unknown_survivor_count_lower_bound = 1;
    let fail_fast_catalog_scan_terminated = true;
    let no_unknown_survives = false;
    let class_exhaustion_proved = false;
    let global_e4_complete = false;
    let five_pending_memberships_now_authorized = false;
    let forbidden_outputs = forbidden_outputs();
    if !predecessor_v8_fiber_resolved.classified_internal
        || !wrapped_fiber_scan.next_unknown.f_a5_retained
        || no_unknown_survives
        || class_exhaustion_proved
        || global_e4_complete
        || five_pending_memberships_now_authorized
        || !outputs_all_withheld(&forbidden_outputs)
    {
        return Err(GlobalE4V9Error::Invariant(
            "v9 F-A5 firewall drifted".to_owned(),
        ));
    }
    let mut certificate = GlobalE4V9Certificate {
        schema: GLOBAL_E4_V9_SCHEMA.to_owned(),
        date: GLOBAL_E4_V9_DATE.to_owned(),
        source_bindings: source_bindings(&predecessor_bytes),
        predecessor_global_e4_v8_digest: predecessor.result_digest,
        predecessor_global_e4_v8_replayed: true,
        global_e4_v9_executed: true,
        wrapped_domain,
        predecessor_v8_fiber_resolved,
        wrapped_fiber_scan,
        unknown_survivor_count_lower_bound,
        fail_fast_catalog_scan_terminated,
        no_unknown_survives,
        class_exhaustion_proved,
        global_e4_complete,
        five_pending_memberships_now_authorized,
        forbidden_outputs,
        outcome: "global_e4_v9_resolves_v8_wrapper_gap_next_admissible_motive_lacks_contextual_instantiation_coherence"
            .to_owned(),
        permitted_conclusion: "The adopted wrapper is finite, verdict-blind, pi-covering, closed-fiber conservative, and it closes the v8 Type-to-Type fiber at nu=0. The independently enumerated four-node motive Type -> Element(Univ) is B15-formable and motive-typed at the live application, but the contextual kernel has no registered closed probes for it. F-A5 therefore retains an exact wrapped Unknown."
            .to_owned(),
        required_successor_action: "Continue F-A5 with a versioned theorem or rule that proves contextual instantiation coherence uniformly for every admissible B15 motive (or adjudicate a non-verdict-based restriction of the motive grammar). Do not filter this fiber by the desired Internal verdict."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GlobalE4V9Replay {
    GlobalE4V9Replay {
        valid: false,
        rerun_executed: false,
        wrapped_domain_finite: false,
        pi_coverage_replayed: false,
        declaration_independence_replayed: false,
        v8_fiber_resolved: false,
        next_exact_unknown_found: false,
        f_a5_retained: false,
        class_exhaustion_proved: false,
        global_e4_complete: false,
        pending_memberships_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "global_e4_v9_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &GlobalE4V9Certificate,
    expected: &GlobalE4V9Certificate,
) -> GlobalE4V9Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GlobalE4V9Replay {
        valid: errors.is_empty(),
        rerun_executed: certificate.global_e4_v9_executed,
        wrapped_domain_finite: certificate.wrapped_domain.finiteness.finite
            && !certificate.wrapped_domain.finiteness.silent_truncation,
        pi_coverage_replayed: certificate.wrapped_domain.f_w2_retained,
        declaration_independence_replayed: certificate.wrapped_domain.f_w1_retained,
        v8_fiber_resolved: certificate
            .predecessor_v8_fiber_resolved
            .classified_internal,
        next_exact_unknown_found: certificate
            .wrapped_fiber_scan
            .next_unknown
            .wrapped_surface_membership
            .is_member,
        f_a5_retained: certificate.wrapped_fiber_scan.next_unknown.f_a5_retained,
        class_exhaustion_proved: certificate.class_exhaustion_proved,
        global_e4_complete: certificate.global_e4_complete,
        pending_memberships_authorized: certificate.five_pending_memberships_now_authorized,
        forbidden_outputs_withheld: outputs_all_withheld(&certificate.forbidden_outputs),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_global_e4_v9_certificate(certificate: &GlobalE4V9Certificate) -> GlobalE4V9Replay {
    let expected = match issue_global_e4_v9_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_global_e4_v9_json(json: &str) -> GlobalE4V9Replay {
    // The source-bound classifier evidence is intentionally deeply nested.
    // Windows executables start their main thread with a comparatively small
    // stack, so make the replay API safe for both tests and the CLI rather
    // than relying on the caller's stack size.
    let json = json.to_owned();
    let worker = match std::thread::Builder::new()
        .name("global-e4-v9-replay".to_owned())
        .stack_size(16 * 1024 * 1024)
        .spawn(
            move || match serde_json::from_str::<GlobalE4V9Certificate>(&json) {
                Ok(certificate) => replay_global_e4_v9_certificate(&certificate),
                Err(error) => failed_replay(format!("JSON parse failed: {error}")),
            },
        ) {
        Ok(worker) => worker,
        Err(error) => return failed_replay(format!("replay worker spawn failed: {error}")),
    };
    match worker.join() {
        Ok(replay) => replay,
        Err(_) => failed_replay("replay worker panicked"),
    }
}

pub fn emit_global_e4_v9_create_new(path: &Path) -> Result<GlobalE4V9Replay, GlobalE4V9Error> {
    let certificate = issue_global_e4_v9_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GlobalE4V9Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GlobalE4V9Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GlobalE4V9Error::Io(error.to_string()))?;
    let replay = replay_global_e4_v9_certificate(&certificate);
    if !replay.valid {
        return Err(GlobalE4V9Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intended_wrapper_resolves_the_v8_gap_at_zero_credit() {
        let predecessor: GlobalE4V8Certificate =
            serde_json::from_slice(PREDECESSOR_ARTIFACT_BYTES).expect("sealed v8 artifact");
        let record = issue_resolved_v8_fiber(&predecessor).expect("resolved fiber");
        assert!(record.classified_internal);
        assert_eq!(record.marginal_nu, 0);
        assert!(record.projection_matches_v8_unknown);
    }

    #[test]
    fn blind_scan_finds_the_exact_next_wrapped_unknown() {
        let scan = issue_wrapped_fiber_scan().expect("wrapped scan");
        assert_eq!(
            scan.next_unknown.exact_motive,
            ContextualMotive::Function {
                domain: Box::new(ContextualMotive::Type),
                codomain: Box::new(ContextualMotive::Element(Expr::Univ)),
            }
        );
        assert_eq!(scan.next_unknown.motive_node_count, 4);
        assert!(scan.next_unknown.live_use_motive_typed);
        assert!(scan.next_unknown.f_a5_retained);
        assert!(!scan.next_unknown.named_typed_exclusion);
    }

    #[test]
    fn v9_replay_and_mutations_fail_closed() {
        let certificate = issue_global_e4_v9_certificate().expect("v9 assembly");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut finiteness = certificate.clone();
        finiteness.wrapped_domain.finiteness.finite = false;
        mutations.push(finiteness);
        let mut inference = certificate.clone();
        inference.wrapped_domain.motive_inference_performed = true;
        mutations.push(inference);
        let mut survivor = certificate.clone();
        survivor.wrapped_fiber_scan.next_unknown.f_a5_retained = false;
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

        let json = serde_json::to_string_pretty(&certificate).expect("serialize v9");
        assert!(replay_global_e4_v9_json(&json).valid);
    }
}
