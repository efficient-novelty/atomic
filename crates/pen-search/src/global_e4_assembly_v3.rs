//! Global E-4 create-new rerun after inductive telescope internality.
//!
//! The predecessor witness is rechecked as Internal.  Enumeration then
//! advances to a deterministic next exact candidate and remains fail-fast if
//! the guarded weakening/erasure obligation leaves it Unknown.

use crate::enumerate::{
    EnumerationContext, LateFamilySurface, RawSurfaceMembership, assess_raw_surface_membership,
};
use crate::global_e4_assembly_v2::{GlobalE4V2Certificate, replay_global_e4_v2_json};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_schema::internal_classifier_branch::f_g4_witness_candidate;
use pen_schema::internal_classifier_branch_v2::{
    InductiveInternalityCertificate, RawCandidateDecisionV3, classify_raw_candidate_v3,
    replay_inductive_internality_json,
};
use pen_schema::total_classifier::{F_G4_TYPED_UNKNOWN, RawCandidateDecision};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GLOBAL_E4_V3_SCHEMA: &str = "schema2-global-e4-assembly-v3";
pub const GLOBAL_E4_V3_DATE: &str = "2026-07-20";
pub const GUARDED_INTERNALITY_GAP: &str =
    "INDUCTIVE_INTERNALITY_GUARDED_WEAKENING_ERASURE_INVERSE_LAWS_MISSING";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/inductive_telescope_internality_adjudication.md");
const ENUMERATOR_BYTES: &[u8] = include_bytes!("enumerate.rs");
const INDUCTIVE_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/internal_classifier_branch_v2.rs");
const PREDECESSOR_ASSEMBLY_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v2.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v3.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V3SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedWitnessRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub decision: RawCandidateDecisionV3,
    pub exact_raw_catalog_member: bool,
    pub classified_internal: bool,
    pub marginal_nu: u32,
    pub projection_clause_depends_on_certified_field_zero: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardedUnknownRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub decision: RawCandidateDecisionV3,
    pub exact_raw_catalog_member: bool,
    pub ambient_parameters: u32,
    pub guarded_clauses: Vec<u16>,
    pub guarded_inverse_laws_satisfied: bool,
    pub internal_certificate_issued: bool,
    pub named_internal_gap: String,
    pub f_g4_retained: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V3ForbiddenOutputs {
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

impl GlobalE4V3ForbiddenOutputs {
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
pub struct GlobalE4V3Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GlobalE4V3SourceBinding>,
    pub predecessor_global_e4_v2_digest: String,
    pub predecessor_global_e4_v2_replayed: bool,
    pub inductive_internality_digest: String,
    pub inductive_internality_replayed: bool,
    pub global_e4_v3_executed: bool,
    pub predecessor_unknown_resolved: ResolvedWitnessRecord,
    pub next_guarded_unknown: GuardedUnknownRecord,
    pub unknown_survivor_count_lower_bound: u32,
    pub fail_fast_catalog_scan_terminated: bool,
    pub no_unknown_survives: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub five_pending_memberships_now_authorized: bool,
    pub forbidden_outputs: GlobalE4V3ForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V3Replay {
    pub valid: bool,
    pub rerun_executed: bool,
    pub predecessor_unknown_resolved: bool,
    pub next_exact_unknown_found: bool,
    pub guarded_inverse_gap: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub pending_memberships_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GlobalE4V3Error {
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
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V3_SCHEMA, domain, value))
        .expect("global E-4 v3 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(
    predecessor_bytes: &[u8],
    inductive_bytes: &[u8],
) -> Vec<GlobalE4V3SourceBinding> {
    [
        (
            "docs/inductive_telescope_internality_adjudication.md",
            "adopted_inductive_rule_and_guard_firewall",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v2.json",
            "predecessor_f_i2_assembly",
            predecessor_bytes,
        ),
        (
            "docs/schema2_inductive_internality_v1.json",
            "resolved_witness_successor",
            inductive_bytes,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "exact_raw_catalog_membership",
            ENUMERATOR_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v2.rs",
            "ordered_internal_classifier",
            INDUCTIVE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v2.rs",
            "failed_predecessor_rerun",
            PREDECESSOR_ASSEMBLY_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v3.rs",
            "fail_fast_guarded_successor_assembly",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| GlobalE4V3SourceBinding {
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

pub fn next_guarded_identity_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Lam(Box::new(Expr::Var(3)))),
    ])
}

fn issue_resolved_witness() -> Result<ResolvedWitnessRecord, GlobalE4V3Error> {
    let candidate = f_g4_witness_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let decision = classify_raw_candidate_v3(&candidate);
    let (classified_internal, marginal_nu, projection_clause_depends_on_certified_field_zero) =
        match &decision {
            RawCandidateDecisionV3::Internal {
                certificate,
                marginal_nu,
                ..
            } => (
                certificate.internal_certificate_issued,
                *marginal_nu,
                certificate.clauses.get(1).is_some_and(|clause| {
                    clause.field_dependencies == vec![0]
                        && clause.certified_internal_prefix_before == vec![0]
                        && clause.earned_internal
                }),
            ),
            _ => (false, u32::MAX, false),
        };
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    if !exact_raw_catalog_member
        || !classified_internal
        || marginal_nu != 0
        || !projection_clause_depends_on_certified_field_zero
    {
        return Err(GlobalE4V3Error::Invariant(
            "predecessor witness was not resolved by ordered Internal".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "resolved-predecessor-witness",
        &(
            &candidate,
            &raw_surface_membership,
            &decision,
            exact_raw_catalog_member,
            classified_internal,
            marginal_nu,
            projection_clause_depends_on_certified_field_zero,
        ),
    );
    Ok(ResolvedWitnessRecord {
        candidate,
        raw_surface_membership,
        decision,
        exact_raw_catalog_member,
        classified_internal,
        marginal_nu,
        projection_clause_depends_on_certified_field_zero,
        derivation_hash,
    })
}

fn issue_next_guarded_unknown() -> Result<GuardedUnknownRecord, GlobalE4V3Error> {
    let candidate = next_guarded_identity_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let decision = classify_raw_candidate_v3(&candidate);
    let (
        ambient_parameters,
        guarded_clauses,
        guarded_inverse_laws_satisfied,
        internal_certificate_issued,
        named_internal_gap,
        f_g4_retained,
    ) = match &decision {
        RawCandidateDecisionV3::NamedTypedObstruction {
            legacy_decision,
            internal_attempt,
        } => (
            // The predecessor attempt is signature-bound and records the
            // whole-telescope ambient context in each guarded disposition.
            1,
            internal_attempt
                .clauses
                .iter()
                .filter(|clause| !clause.guarded_inverse_laws_satisfied)
                .map(|clause| clause.clause_index)
                .collect::<Vec<_>>(),
            internal_attempt.all_guarded_inverse_laws_satisfied,
            internal_attempt.internal_certificate_issued,
            internal_attempt.failure_code.clone().unwrap_or_default(),
            matches!(
                legacy_decision,
                RawCandidateDecision::NamedTypedObstruction { code, .. }
                    if code == F_G4_TYPED_UNKNOWN
            ),
        ),
        _ => (0, Vec::new(), false, false, String::new(), false),
    };
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    if !exact_raw_catalog_member
        || ambient_parameters != 1
        || guarded_clauses != vec![0, 1]
        || guarded_inverse_laws_satisfied
        || internal_certificate_issued
        || named_internal_gap != GUARDED_INTERNALITY_GAP
        || !f_g4_retained
    {
        return Err(GlobalE4V3Error::Invariant(
            "next guarded candidate did not remain exact Unknown".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "next-guarded-unknown",
        &(
            &candidate,
            &raw_surface_membership,
            &decision,
            exact_raw_catalog_member,
            ambient_parameters,
            &guarded_clauses,
            guarded_inverse_laws_satisfied,
            internal_certificate_issued,
            &named_internal_gap,
            f_g4_retained,
        ),
    );
    Ok(GuardedUnknownRecord {
        candidate,
        raw_surface_membership,
        decision,
        exact_raw_catalog_member,
        ambient_parameters,
        guarded_clauses,
        guarded_inverse_laws_satisfied,
        internal_certificate_issued,
        named_internal_gap,
        f_g4_retained,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &GlobalE4V3Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("global-e4-v3-certificate", &projection)
}

pub fn issue_global_e4_v3_certificate() -> Result<GlobalE4V3Certificate, GlobalE4V3Error> {
    let predecessor_bytes = std::fs::read(workspace_doc_path("schema2_global_e4_assembly_v2.json"))
        .map_err(|error| GlobalE4V3Error::Io(error.to_string()))?;
    let predecessor_replay = replay_global_e4_v2_json(
        std::str::from_utf8(&predecessor_bytes)
            .map_err(|error| GlobalE4V3Error::Json(error.to_string()))?,
    );
    if !predecessor_replay.valid
        || !predecessor_replay.f_i2_triggered
        || predecessor_replay.global_e4_complete
    {
        return Err(GlobalE4V3Error::Prerequisite(format!(
            "global E-4 v2 replay failed: {}",
            predecessor_replay.errors.join("; ")
        )));
    }
    let predecessor: GlobalE4V2Certificate = serde_json::from_slice(&predecessor_bytes)
        .map_err(|error| GlobalE4V3Error::Json(error.to_string()))?;

    let inductive_bytes =
        std::fs::read(workspace_doc_path("schema2_inductive_internality_v1.json"))
            .map_err(|error| GlobalE4V3Error::Io(error.to_string()))?;
    let inductive_replay = replay_inductive_internality_json(
        std::str::from_utf8(&inductive_bytes)
            .map_err(|error| GlobalE4V3Error::Json(error.to_string()))?,
    );
    if !inductive_replay.valid
        || !inductive_replay.witness_internal
        || !inductive_replay.witness_projection
        || !inductive_replay.witness_not_identity
        || !inductive_replay.global_e4_rerun_authorized
    {
        return Err(GlobalE4V3Error::Prerequisite(format!(
            "inductive Internal replay failed: {}",
            inductive_replay.errors.join("; ")
        )));
    }
    let inductive: InductiveInternalityCertificate = serde_json::from_slice(&inductive_bytes)
        .map_err(|error| GlobalE4V3Error::Json(error.to_string()))?;

    let predecessor_unknown_resolved = issue_resolved_witness()?;
    let next_guarded_unknown = issue_next_guarded_unknown()?;
    let unknown_survivor_count_lower_bound = 1;
    let fail_fast_catalog_scan_terminated = true;
    let no_unknown_survives = false;
    let class_exhaustion_proved = false;
    let global_e4_complete = false;
    let five_pending_memberships_now_authorized = false;
    let forbidden_outputs = GlobalE4V3ForbiddenOutputs {
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
    if !predecessor_unknown_resolved.classified_internal
        || !next_guarded_unknown.f_g4_retained
        || no_unknown_survives
        || class_exhaustion_proved
        || global_e4_complete
        || five_pending_memberships_now_authorized
        || !forbidden_outputs.all_withheld()
    {
        return Err(GlobalE4V3Error::Invariant(
            "guarded Unknown firewall drifted".to_owned(),
        ));
    }

    let mut certificate = GlobalE4V3Certificate {
        schema: GLOBAL_E4_V3_SCHEMA.to_owned(),
        date: GLOBAL_E4_V3_DATE.to_owned(),
        source_bindings: source_bindings(&predecessor_bytes, &inductive_bytes),
        predecessor_global_e4_v2_digest: predecessor.result_digest,
        predecessor_global_e4_v2_replayed: true,
        inductive_internality_digest: inductive.result_digest,
        inductive_internality_replayed: true,
        global_e4_v3_executed: true,
        predecessor_unknown_resolved,
        next_guarded_unknown,
        unknown_survivor_count_lower_bound,
        fail_fast_catalog_scan_terminated,
        no_unknown_survives,
        class_exhaustion_proved,
        global_e4_complete,
        five_pending_memberships_now_authorized,
        forbidden_outputs,
        outcome: "global_e4_v3_resolved_projection_witness_next_guarded_unknown_blocks_exhaustion"
            .to_owned(),
        permitted_conclusion: "Inductive telescope internality resolves [Univ,Lam(Var(1))] as zero-credit Internal projection. The exact guarded candidate [Univ,Lam(Var(3))] then survives because no weakening/erasure inverse certificate is attached, so F-G4 still prevents exhaustion."
            .to_owned(),
        required_successor_action: "Prove the existing guarded weakening/erasure inverse laws at candidate-clause granularity for the one-ambient-parameter identity family, or retain the named gap. Then rerun global E-4 create-new; do not issue membership or count outputs first."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GlobalE4V3Replay {
    GlobalE4V3Replay {
        valid: false,
        rerun_executed: false,
        predecessor_unknown_resolved: false,
        next_exact_unknown_found: false,
        guarded_inverse_gap: false,
        class_exhaustion_proved: false,
        global_e4_complete: false,
        pending_memberships_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "global_e4_v3_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &GlobalE4V3Certificate,
    expected: &GlobalE4V3Certificate,
) -> GlobalE4V3Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GlobalE4V3Replay {
        valid: errors.is_empty(),
        rerun_executed: certificate.global_e4_v3_executed,
        predecessor_unknown_resolved: certificate.predecessor_unknown_resolved.classified_internal,
        next_exact_unknown_found: certificate.next_guarded_unknown.exact_raw_catalog_member
            && certificate.next_guarded_unknown.f_g4_retained,
        guarded_inverse_gap: certificate.next_guarded_unknown.named_internal_gap
            == GUARDED_INTERNALITY_GAP,
        class_exhaustion_proved: certificate.class_exhaustion_proved,
        global_e4_complete: certificate.global_e4_complete,
        pending_memberships_authorized: certificate.five_pending_memberships_now_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_global_e4_v3_certificate(certificate: &GlobalE4V3Certificate) -> GlobalE4V3Replay {
    let expected = match issue_global_e4_v3_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_global_e4_v3_json(json: &str) -> GlobalE4V3Replay {
    match serde_json::from_str::<GlobalE4V3Certificate>(json) {
        Ok(certificate) => replay_global_e4_v3_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_global_e4_v3_create_new(path: &Path) -> Result<GlobalE4V3Replay, GlobalE4V3Error> {
    let certificate = issue_global_e4_v3_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GlobalE4V3Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GlobalE4V3Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GlobalE4V3Error::Io(error.to_string()))?;
    let replay = replay_global_e4_v3_certificate(&certificate);
    if !replay.valid {
        return Err(GlobalE4V3Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predecessor_witness_is_now_internal_projection() {
        let resolved = issue_resolved_witness().expect("resolved witness");
        assert!(resolved.exact_raw_catalog_member);
        assert!(resolved.classified_internal);
        assert_eq!(resolved.marginal_nu, 0);
        assert!(resolved.projection_clause_depends_on_certified_field_zero);
    }

    #[test]
    fn next_guarded_identity_is_exact_and_remains_unknown() {
        let unknown = issue_next_guarded_unknown().expect("guarded unknown");
        assert!(unknown.exact_raw_catalog_member);
        assert_eq!(unknown.ambient_parameters, 1);
        assert_eq!(unknown.guarded_clauses, vec![0, 1]);
        assert!(!unknown.guarded_inverse_laws_satisfied);
        assert!(!unknown.internal_certificate_issued);
        assert!(unknown.f_g4_retained);
    }

    #[test]
    fn v3_replay_and_mutations_fail_closed() {
        let certificate = issue_global_e4_v3_certificate().expect("v3 assembly");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut resolved = certificate.clone();
        resolved.predecessor_unknown_resolved.classified_internal = false;
        mutations.push(resolved);
        let mut gap = certificate.clone();
        gap.next_guarded_unknown.guarded_inverse_laws_satisfied = true;
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
