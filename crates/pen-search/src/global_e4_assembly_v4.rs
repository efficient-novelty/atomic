//! Global E-4 create-new rerun after exporting guarded per-clause inverse
//! evidence for the one-ambient-parameter identity family.

use crate::enumerate::{
    EnumerationContext, LateFamilySurface, RawSurfaceMembership, assess_raw_surface_membership,
};
use crate::global_e4_assembly_v3::{GlobalE4V3Certificate, replay_global_e4_v3_json};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_schema::internal_classifier_branch_v2::RawCandidateDecisionV3;
use pen_schema::internal_classifier_branch_v3::{
    GuardedIdentityInternalityCertificate, RawCandidateDecisionV4, classify_raw_candidate_v4,
    one_ambient_identity_candidate, replay_guarded_identity_internality_json,
};
use pen_schema::total_classifier::{F_G4_TYPED_UNKNOWN, RawCandidateDecision};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GLOBAL_E4_V4_SCHEMA: &str = "schema2-global-e4-assembly-v4";
pub const GLOBAL_E4_V4_DATE: &str = "2026-07-20";
pub const STRUCTURAL_LAMBDA_GAP: &str = "INDUCTIVE_TELESCOPE_INTERNALITY_NOT_PROVED_FOR_CANDIDATE";

const V3_RESULT_BYTES: &[u8] = include_bytes!("../../../docs/SCHEMA2_GLOBAL_E4_V3_RESULT.md");
const ENUMERATOR_BYTES: &[u8] = include_bytes!("enumerate.rs");
const GUARDED_TOKEN_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/guarded_internality.rs");
const CLASSIFIER_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/internal_classifier_branch_v3.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v3.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v4.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V4SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResolvedGuardedIdentityRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub decision: RawCandidateDecisionV4,
    pub exact_raw_catalog_member: bool,
    pub classified_internal: bool,
    pub clause_evidence_count: usize,
    pub both_inverse_laws_checked_per_clause: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralUnknownRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub decision: RawCandidateDecisionV4,
    pub exact_raw_catalog_member: bool,
    pub ambient_parameters: u32,
    pub internal_certificate_issued: bool,
    pub named_internal_gap: String,
    pub f_g4_retained: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V4ForbiddenOutputs {
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

impl GlobalE4V4ForbiddenOutputs {
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
pub struct GlobalE4V4Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GlobalE4V4SourceBinding>,
    pub predecessor_global_e4_v3_digest: String,
    pub predecessor_global_e4_v3_replayed: bool,
    pub guarded_identity_internality_digest: String,
    pub guarded_identity_internality_replayed: bool,
    pub global_e4_v4_executed: bool,
    pub predecessor_guarded_unknown_resolved: ResolvedGuardedIdentityRecord,
    pub next_structural_unknown: StructuralUnknownRecord,
    pub unknown_survivor_count_lower_bound: u32,
    pub fail_fast_catalog_scan_terminated: bool,
    pub no_unknown_survives: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub five_pending_memberships_now_authorized: bool,
    pub forbidden_outputs: GlobalE4V4ForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V4Replay {
    pub valid: bool,
    pub rerun_executed: bool,
    pub guarded_unknown_resolved: bool,
    pub next_exact_unknown_found: bool,
    pub structural_lambda_gap: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub pending_memberships_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GlobalE4V4Error {
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
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V4_SCHEMA, domain, value))
        .expect("global E-4 v4 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(predecessor_bytes: &[u8], guarded_bytes: &[u8]) -> Vec<GlobalE4V4SourceBinding> {
    [
        (
            "docs/SCHEMA2_GLOBAL_E4_V3_RESULT.md",
            "predecessor_guarded_gap",
            V3_RESULT_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v3.json",
            "failed_predecessor_assembly",
            predecessor_bytes,
        ),
        (
            "docs/schema2_guarded_identity_internality_v1.json",
            "guarded_inverse_successor",
            guarded_bytes,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "exact_raw_catalog_membership",
            ENUMERATOR_BYTES,
        ),
        (
            "crates/pen-type/src/guarded_internality.rs",
            "opaque_per_clause_inverse_tokens",
            GUARDED_TOKEN_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v3.rs",
            "guarded_internal_classifier",
            CLASSIFIER_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v3.rs",
            "predecessor_fail_fast_assembly",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v4.rs",
            "structural_fail_fast_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| GlobalE4V4SourceBinding {
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

pub fn structural_constant_universe_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(ClauseRole::Introduction, Expr::Lam(Box::new(Expr::Univ))),
    ])
}

fn issue_resolved_guarded_identity() -> Result<ResolvedGuardedIdentityRecord, GlobalE4V4Error> {
    let candidate = one_ambient_identity_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let decision = classify_raw_candidate_v4(&candidate);
    let (
        classified_internal,
        clause_evidence_count,
        both_inverse_laws_checked_per_clause,
        marginal_nu,
    ) = match &decision {
        RawCandidateDecisionV4::InternalGuarded {
            certificate,
            marginal_nu,
            ..
        } => (
            certificate.internal_certificate_issued,
            certificate.clause_evidence.len(),
            certificate.every_inverse_law_checked,
            *marginal_nu,
        ),
        _ => (false, 0, false, u32::MAX),
    };
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    if !exact_raw_catalog_member
        || !classified_internal
        || clause_evidence_count != 2
        || !both_inverse_laws_checked_per_clause
        || marginal_nu != 0
    {
        return Err(GlobalE4V4Error::Invariant(
            "guarded predecessor Unknown was not resolved".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "resolved-guarded-identity",
        &(
            &candidate,
            &raw_surface_membership,
            &decision,
            exact_raw_catalog_member,
            classified_internal,
            clause_evidence_count,
            both_inverse_laws_checked_per_clause,
            marginal_nu,
        ),
    );
    Ok(ResolvedGuardedIdentityRecord {
        candidate,
        raw_surface_membership,
        decision,
        exact_raw_catalog_member,
        classified_internal,
        clause_evidence_count,
        both_inverse_laws_checked_per_clause,
        marginal_nu,
        derivation_hash,
    })
}

fn issue_next_structural_unknown() -> Result<StructuralUnknownRecord, GlobalE4V4Error> {
    let candidate = structural_constant_universe_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let decision = classify_raw_candidate_v4(&candidate);
    let (ambient_parameters, internal_certificate_issued, named_internal_gap, f_g4_retained) =
        match &decision {
            RawCandidateDecisionV4::NamedTypedObstruction {
                predecessor_decision:
                    RawCandidateDecisionV3::NamedTypedObstruction {
                        legacy_decision,
                        internal_attempt,
                    },
            } => (
                0,
                internal_attempt.internal_certificate_issued,
                internal_attempt.failure_code.clone().unwrap_or_default(),
                matches!(
                    legacy_decision,
                    RawCandidateDecision::NamedTypedObstruction { code, .. }
                        if code == F_G4_TYPED_UNKNOWN
                ),
            ),
            _ => (u32::MAX, false, String::new(), false),
        };
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    if !exact_raw_catalog_member
        || ambient_parameters != 0
        || internal_certificate_issued
        || named_internal_gap != STRUCTURAL_LAMBDA_GAP
        || !f_g4_retained
    {
        return Err(GlobalE4V4Error::Invariant(
            "structural constant lambda did not remain exact Unknown".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "next-structural-unknown",
        &(
            &candidate,
            &raw_surface_membership,
            &decision,
            exact_raw_catalog_member,
            ambient_parameters,
            internal_certificate_issued,
            &named_internal_gap,
            f_g4_retained,
        ),
    );
    Ok(StructuralUnknownRecord {
        candidate,
        raw_surface_membership,
        decision,
        exact_raw_catalog_member,
        ambient_parameters,
        internal_certificate_issued,
        named_internal_gap,
        f_g4_retained,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &GlobalE4V4Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("global-e4-v4-certificate", &projection)
}

pub fn issue_global_e4_v4_certificate() -> Result<GlobalE4V4Certificate, GlobalE4V4Error> {
    let predecessor_bytes = std::fs::read(workspace_doc_path("schema2_global_e4_assembly_v3.json"))
        .map_err(|error| GlobalE4V4Error::Io(error.to_string()))?;
    let predecessor_replay = replay_global_e4_v3_json(
        std::str::from_utf8(&predecessor_bytes)
            .map_err(|error| GlobalE4V4Error::Json(error.to_string()))?,
    );
    if !predecessor_replay.valid
        || !predecessor_replay.guarded_inverse_gap
        || predecessor_replay.global_e4_complete
    {
        return Err(GlobalE4V4Error::Prerequisite(format!(
            "global E-4 v3 replay failed: {}",
            predecessor_replay.errors.join("; ")
        )));
    }
    let predecessor: GlobalE4V3Certificate = serde_json::from_slice(&predecessor_bytes)
        .map_err(|error| GlobalE4V4Error::Json(error.to_string()))?;

    let guarded_bytes = std::fs::read(workspace_doc_path(
        "schema2_guarded_identity_internality_v1.json",
    ))
    .map_err(|error| GlobalE4V4Error::Io(error.to_string()))?;
    let guarded_replay = replay_guarded_identity_internality_json(
        std::str::from_utf8(&guarded_bytes)
            .map_err(|error| GlobalE4V4Error::Json(error.to_string()))?,
    );
    if !guarded_replay.valid
        || !guarded_replay.guarded_internal
        || !guarded_replay.both_inverse_laws_per_clause
        || !guarded_replay.global_e4_rerun_authorized
    {
        return Err(GlobalE4V4Error::Prerequisite(format!(
            "guarded identity replay failed: {}",
            guarded_replay.errors.join("; ")
        )));
    }
    let guarded: GuardedIdentityInternalityCertificate = serde_json::from_slice(&guarded_bytes)
        .map_err(|error| GlobalE4V4Error::Json(error.to_string()))?;

    let predecessor_guarded_unknown_resolved = issue_resolved_guarded_identity()?;
    let next_structural_unknown = issue_next_structural_unknown()?;
    let unknown_survivor_count_lower_bound = 1;
    let fail_fast_catalog_scan_terminated = true;
    let no_unknown_survives = false;
    let class_exhaustion_proved = false;
    let global_e4_complete = false;
    let five_pending_memberships_now_authorized = false;
    let forbidden_outputs = GlobalE4V4ForbiddenOutputs {
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
    if !predecessor_guarded_unknown_resolved.classified_internal
        || !next_structural_unknown.f_g4_retained
        || no_unknown_survives
        || class_exhaustion_proved
        || global_e4_complete
        || five_pending_memberships_now_authorized
        || !forbidden_outputs.all_withheld()
    {
        return Err(GlobalE4V4Error::Invariant(
            "structural Unknown firewall drifted".to_owned(),
        ));
    }

    let mut certificate = GlobalE4V4Certificate {
        schema: GLOBAL_E4_V4_SCHEMA.to_owned(),
        date: GLOBAL_E4_V4_DATE.to_owned(),
        source_bindings: source_bindings(&predecessor_bytes, &guarded_bytes),
        predecessor_global_e4_v3_digest: predecessor.result_digest,
        predecessor_global_e4_v3_replayed: true,
        guarded_identity_internality_digest: guarded.result_digest,
        guarded_identity_internality_replayed: true,
        global_e4_v4_executed: true,
        predecessor_guarded_unknown_resolved,
        next_structural_unknown,
        unknown_survivor_count_lower_bound,
        fail_fast_catalog_scan_terminated,
        no_unknown_survives,
        class_exhaustion_proved,
        global_e4_complete,
        five_pending_memberships_now_authorized,
        forbidden_outputs,
        outcome: "global_e4_v4_resolved_guarded_identity_next_structural_lambda_unknown_blocks_exhaustion"
            .to_owned(),
        permitted_conclusion: "The one-ambient identity now earns Internal from two replayed per-clause inverse tokens. The exact closed candidate [Univ,Lam(Univ)] remains outside the current direct, inductive-projection, and guarded-identity branches, so F-G4 still prevents exhaustion."
            .to_owned(),
        required_successor_action: "Prove a structural-closure Internal rule for lambda introduction over already Internal structural bodies, beginning with the constant universe body, with constructor-level replay and no new marginal credit; then rerun global E-4 create-new."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GlobalE4V4Replay {
    GlobalE4V4Replay {
        valid: false,
        rerun_executed: false,
        guarded_unknown_resolved: false,
        next_exact_unknown_found: false,
        structural_lambda_gap: false,
        class_exhaustion_proved: false,
        global_e4_complete: false,
        pending_memberships_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "global_e4_v4_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &GlobalE4V4Certificate,
    expected: &GlobalE4V4Certificate,
) -> GlobalE4V4Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    GlobalE4V4Replay {
        valid: errors.is_empty(),
        rerun_executed: certificate.global_e4_v4_executed,
        guarded_unknown_resolved: certificate
            .predecessor_guarded_unknown_resolved
            .classified_internal,
        next_exact_unknown_found: certificate.next_structural_unknown.exact_raw_catalog_member
            && certificate.next_structural_unknown.f_g4_retained,
        structural_lambda_gap: certificate.next_structural_unknown.named_internal_gap
            == STRUCTURAL_LAMBDA_GAP,
        class_exhaustion_proved: certificate.class_exhaustion_proved,
        global_e4_complete: certificate.global_e4_complete,
        pending_memberships_authorized: certificate.five_pending_memberships_now_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_global_e4_v4_certificate(certificate: &GlobalE4V4Certificate) -> GlobalE4V4Replay {
    let expected = match issue_global_e4_v4_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_global_e4_v4_json(json: &str) -> GlobalE4V4Replay {
    match serde_json::from_str::<GlobalE4V4Certificate>(json) {
        Ok(certificate) => replay_global_e4_v4_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_global_e4_v4_create_new(path: &Path) -> Result<GlobalE4V4Replay, GlobalE4V4Error> {
    let certificate = issue_global_e4_v4_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GlobalE4V4Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GlobalE4V4Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GlobalE4V4Error::Io(error.to_string()))?;
    let replay = replay_global_e4_v4_certificate(&certificate);
    if !replay.valid {
        return Err(GlobalE4V4Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn guarded_identity_is_resolved_with_two_clause_tokens() {
        let resolved = issue_resolved_guarded_identity().expect("resolved guarded identity");
        assert!(resolved.exact_raw_catalog_member);
        assert!(resolved.classified_internal);
        assert_eq!(resolved.clause_evidence_count, 2);
        assert!(resolved.both_inverse_laws_checked_per_clause);
        assert_eq!(resolved.marginal_nu, 0);
    }

    #[test]
    fn structural_constant_lambda_is_exact_and_still_unknown() {
        let unknown = issue_next_structural_unknown().expect("structural unknown");
        assert!(unknown.exact_raw_catalog_member);
        assert_eq!(unknown.ambient_parameters, 0);
        assert!(!unknown.internal_certificate_issued);
        assert_eq!(unknown.named_internal_gap, STRUCTURAL_LAMBDA_GAP);
        assert!(unknown.f_g4_retained);
    }

    #[test]
    fn v4_replay_and_mutations_fail_closed() {
        let certificate = issue_global_e4_v4_certificate().expect("v4 assembly");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut resolved = certificate.clone();
        resolved
            .predecessor_guarded_unknown_resolved
            .classified_internal = false;
        mutations.push(resolved);
        let mut survivor = certificate.clone();
        survivor.next_structural_unknown.f_g4_retained = false;
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
