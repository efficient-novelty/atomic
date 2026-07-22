//! Create-new global E-4 successor assembly after adoption of the
//! certificate-backed Internal classifier branch.
//!
//! The assembly remains fail-fast: one exact raw-catalog member that fails
//! Internal derivability and remains in F-G4 is enough to prevent exhaustion.

use crate::enumerate::{
    EnumerationContext, LateFamilySurface, RawSurfaceMembership, assess_raw_surface_membership,
};
use crate::global_e4_assembly::{GlobalE4AssemblyCertificate, replay_global_e4_assembly_json};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_schema::internal_classifier_branch::{
    F_I2_WITNESS_NOT_DERIVABLE_OVER_B15, InternalBranchCertificate, RawCandidateDecisionV2,
    classify_raw_candidate_v2, f_g4_witness_candidate, replay_internal_branch_json,
};
use pen_schema::total_classifier::{F_G4_TYPED_UNKNOWN, RawCandidateDecision};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const GLOBAL_E4_V2_SCHEMA: &str = "schema2-global-e4-assembly-v2";
pub const GLOBAL_E4_V2_DATE: &str = "2026-07-20";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/internal_classifier_branch_adjudication.md");
const ENUMERATOR_BYTES: &[u8] = include_bytes!("enumerate.rs");
const INTERNAL_BRANCH_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-schema/src/internal_classifier_branch.rs");
const PREDECESSOR_ASSEMBLY_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("global_e4_assembly_v2.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UnknownSurvivorRecord {
    pub candidate: Telescope,
    pub raw_surface_membership: RawSurfaceMembership,
    pub internal_successor_decision: RawCandidateDecisionV2,
    pub exact_raw_catalog_member: bool,
    pub internal_certificate_issued: bool,
    pub f_i2_triggered: bool,
    pub f_g4_retained: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V2ForbiddenOutputs {
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

impl GlobalE4V2ForbiddenOutputs {
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
pub struct GlobalE4V2Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<GlobalE4V2SourceBinding>,
    pub predecessor_global_e4_digest: String,
    pub predecessor_global_e4_replayed: bool,
    pub internal_branch_digest: String,
    pub internal_branch_replayed: bool,
    pub internal_branch_priority_adopted: bool,
    pub global_e4_successor_rerun_executed: bool,
    pub unknown_survivors: Vec<UnknownSurvivorRecord>,
    pub unknown_survivor_count_lower_bound: u32,
    pub fail_fast_catalog_scan_terminated: bool,
    pub no_unknown_survives: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub five_pending_memberships_now_authorized: bool,
    pub forbidden_outputs: GlobalE4V2ForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GlobalE4V2Replay {
    pub valid: bool,
    pub successor_rerun_executed: bool,
    pub exact_unknown_survivor: bool,
    pub f_i2_triggered: bool,
    pub f_g4_retained: bool,
    pub class_exhaustion_proved: bool,
    pub global_e4_complete: bool,
    pub pending_memberships_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum GlobalE4V2Error {
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
    let bytes = serde_json::to_vec(&(GLOBAL_E4_V2_SCHEMA, domain, value))
        .expect("global E-4 v2 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings(
    predecessor_bytes: &[u8],
    internal_bytes: &[u8],
) -> Vec<GlobalE4V2SourceBinding> {
    [
        (
            "docs/internal_classifier_branch_adjudication.md",
            "adopted_successor_and_sequence",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v1.json",
            "failed_predecessor_assembly",
            predecessor_bytes,
        ),
        (
            "docs/schema2_internal_classifier_branch_v1.json",
            "witness_first_internal_decision",
            internal_bytes,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "exact_raw_catalog_membership",
            ENUMERATOR_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch.rs",
            "certificate_backed_internal_classifier",
            INTERNAL_BRANCH_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly.rs",
            "predecessor_global_assembly",
            PREDECESSOR_ASSEMBLY_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/global_e4_assembly_v2.rs",
            "fail_fast_successor_assembly",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| GlobalE4V2SourceBinding {
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

fn issue_unknown_survivor() -> Result<UnknownSurvivorRecord, GlobalE4V2Error> {
    let candidate = f_g4_witness_candidate();
    let raw_surface_membership = assess_raw_surface_membership(frozen_raw_context(), &candidate);
    let internal_successor_decision = classify_raw_candidate_v2(&candidate);
    let (internal_certificate_issued, f_i2_triggered, f_g4_retained) =
        match &internal_successor_decision {
            RawCandidateDecisionV2::NamedTypedObstruction {
                legacy_decision,
                internal_attempt,
            } => (
                internal_attempt.internal_certificate_issued,
                internal_attempt.failure_code.as_deref()
                    == Some(F_I2_WITNESS_NOT_DERIVABLE_OVER_B15),
                matches!(
                    legacy_decision,
                    RawCandidateDecision::NamedTypedObstruction { code, .. }
                        if code == F_G4_TYPED_UNKNOWN
                ),
            ),
            _ => (false, false, false),
        };
    let exact_raw_catalog_member = raw_surface_membership.is_member;
    if !exact_raw_catalog_member || internal_certificate_issued || !f_i2_triggered || !f_g4_retained
    {
        return Err(GlobalE4V2Error::Invariant(
            "exact witness no longer survives Internal -> F-G4".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "unknown-survivor",
        &(
            &candidate,
            &raw_surface_membership,
            &internal_successor_decision,
            exact_raw_catalog_member,
            internal_certificate_issued,
            f_i2_triggered,
            f_g4_retained,
        ),
    );
    Ok(UnknownSurvivorRecord {
        candidate,
        raw_surface_membership,
        internal_successor_decision,
        exact_raw_catalog_member,
        internal_certificate_issued,
        f_i2_triggered,
        f_g4_retained,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &GlobalE4V2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("global-e4-v2-certificate", &projection)
}

pub fn issue_global_e4_v2_certificate() -> Result<GlobalE4V2Certificate, GlobalE4V2Error> {
    let predecessor_bytes = std::fs::read(workspace_doc_path("schema2_global_e4_assembly_v1.json"))
        .map_err(|error| GlobalE4V2Error::Io(error.to_string()))?;
    let predecessor_replay = replay_global_e4_assembly_json(
        std::str::from_utf8(&predecessor_bytes)
            .map_err(|error| GlobalE4V2Error::Json(error.to_string()))?,
    );
    if !predecessor_replay.valid
        || !predecessor_replay.f_g4_triggered
        || predecessor_replay.global_e4_complete
    {
        return Err(GlobalE4V2Error::Prerequisite(format!(
            "predecessor global E-4 replay failed: {}",
            predecessor_replay.errors.join("; ")
        )));
    }
    let predecessor: GlobalE4AssemblyCertificate = serde_json::from_slice(&predecessor_bytes)
        .map_err(|error| GlobalE4V2Error::Json(error.to_string()))?;

    let internal_bytes = std::fs::read(workspace_doc_path(
        "schema2_internal_classifier_branch_v1.json",
    ))
    .map_err(|error| GlobalE4V2Error::Io(error.to_string()))?;
    let internal_replay = replay_internal_branch_json(
        std::str::from_utf8(&internal_bytes)
            .map_err(|error| GlobalE4V2Error::Json(error.to_string()))?,
    );
    if !internal_replay.valid
        || !internal_replay.global_e4_rerun_authorized
        || !internal_replay.f_i2_triggered
        || !internal_replay.unknown_retained
    {
        return Err(GlobalE4V2Error::Prerequisite(format!(
            "Internal-branch replay failed: {}",
            internal_replay.errors.join("; ")
        )));
    }
    let internal: InternalBranchCertificate = serde_json::from_slice(&internal_bytes)
        .map_err(|error| GlobalE4V2Error::Json(error.to_string()))?;

    let survivor = issue_unknown_survivor()?;
    let unknown_survivors = vec![survivor];
    let unknown_survivor_count_lower_bound = 1;
    let fail_fast_catalog_scan_terminated = true;
    let no_unknown_survives = false;
    let class_exhaustion_proved = false;
    let global_e4_complete = false;
    let five_pending_memberships_now_authorized = false;
    let forbidden_outputs = GlobalE4V2ForbiddenOutputs {
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
    if unknown_survivors.is_empty()
        || no_unknown_survives
        || class_exhaustion_proved
        || global_e4_complete
        || five_pending_memberships_now_authorized
        || !forbidden_outputs.all_withheld()
    {
        return Err(GlobalE4V2Error::Invariant(
            "F-I2/F-G4 successor firewall drifted".to_owned(),
        ));
    }

    let mut certificate = GlobalE4V2Certificate {
        schema: GLOBAL_E4_V2_SCHEMA.to_owned(),
        date: GLOBAL_E4_V2_DATE.to_owned(),
        source_bindings: source_bindings(&predecessor_bytes, &internal_bytes),
        predecessor_global_e4_digest: predecessor.result_digest,
        predecessor_global_e4_replayed: true,
        internal_branch_digest: internal.result_digest,
        internal_branch_replayed: true,
        internal_branch_priority_adopted: true,
        global_e4_successor_rerun_executed: true,
        unknown_survivors,
        unknown_survivor_count_lower_bound,
        fail_fast_catalog_scan_terminated,
        no_unknown_survives,
        class_exhaustion_proved,
        global_e4_complete,
        five_pending_memberships_now_authorized,
        forbidden_outputs,
        outcome: "global_e4_v2_executed_exact_f_i2_f_g4_survivor_exhaustion_not_proved"
            .to_owned(),
        permitted_conclusion: "The adopted Internal branch was executed before the class branch. The exact raw witness failed its B15 derivability obligation and survived as named Unknown, so global E-4 exhaustion remains false."
            .to_owned(),
        required_successor_action: "Stop before the five membership verdicts. Adjudicate the binding-level mismatch explicitly: retain Lam(Var(1)) and reopen the grammar, or adopt a versioned corrected identity candidate Lam(Var(2)); do not reinterpret the frozen absolute-level term in place."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> GlobalE4V2Replay {
    GlobalE4V2Replay {
        valid: false,
        successor_rerun_executed: false,
        exact_unknown_survivor: false,
        f_i2_triggered: false,
        f_g4_retained: false,
        class_exhaustion_proved: false,
        global_e4_complete: false,
        pending_memberships_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "global_e4_v2_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &GlobalE4V2Certificate,
    expected: &GlobalE4V2Certificate,
) -> GlobalE4V2Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    let survivor = certificate.unknown_survivors.first();
    GlobalE4V2Replay {
        valid: errors.is_empty(),
        successor_rerun_executed: certificate.global_e4_successor_rerun_executed,
        exact_unknown_survivor: survivor.is_some_and(|record| record.exact_raw_catalog_member),
        f_i2_triggered: survivor.is_some_and(|record| record.f_i2_triggered),
        f_g4_retained: survivor.is_some_and(|record| record.f_g4_retained),
        class_exhaustion_proved: certificate.class_exhaustion_proved,
        global_e4_complete: certificate.global_e4_complete,
        pending_memberships_authorized: certificate.five_pending_memberships_now_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_global_e4_v2_certificate(certificate: &GlobalE4V2Certificate) -> GlobalE4V2Replay {
    let expected = match issue_global_e4_v2_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_global_e4_v2_json(json: &str) -> GlobalE4V2Replay {
    match serde_json::from_str::<GlobalE4V2Certificate>(json) {
        Ok(certificate) => replay_global_e4_v2_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_global_e4_v2_create_new(path: &Path) -> Result<GlobalE4V2Replay, GlobalE4V2Error> {
    let certificate = issue_global_e4_v2_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| GlobalE4V2Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| GlobalE4V2Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| GlobalE4V2Error::Io(error.to_string()))?;
    let replay = replay_global_e4_v2_certificate(&certificate);
    if !replay.valid {
        return Err(GlobalE4V2Error::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_witness_survives_internal_and_fires_f_g4_again() {
        let survivor = issue_unknown_survivor().expect("survivor");
        assert!(survivor.exact_raw_catalog_member);
        assert!(!survivor.internal_certificate_issued);
        assert!(survivor.f_i2_triggered);
        assert!(survivor.f_g4_retained);
    }

    #[test]
    fn successor_assembly_executes_but_does_not_unlock_dominoes() {
        let certificate = issue_global_e4_v2_certificate().expect("v2 assembly");
        assert!(certificate.global_e4_successor_rerun_executed);
        assert_eq!(certificate.unknown_survivor_count_lower_bound, 1);
        assert!(!certificate.class_exhaustion_proved);
        assert!(!certificate.global_e4_complete);
        assert!(!certificate.five_pending_memberships_now_authorized);
        assert!(certificate.forbidden_outputs.all_withheld());
    }

    #[test]
    fn successor_replay_and_mutations_fail_closed() {
        let certificate = issue_global_e4_v2_certificate().expect("v2 assembly");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut survivor = certificate.clone();
        survivor.unknown_survivors[0].f_i2_triggered = false;
        mutations.push(survivor);
        let mut exhaustion = certificate.clone();
        exhaustion.class_exhaustion_proved = true;
        mutations.push(exhaustion);
        let mut authorization = certificate.clone();
        authorization.five_pending_memberships_now_authorized = true;
        mutations.push(authorization);
        let mut forbidden = certificate.clone();
        forbidden.forbidden_outputs.e2b_executed = true;
        mutations.push(forbidden);
        for mutation in mutations {
            assert!(!replay_against_expected(&mutation, &certificate).valid);
        }
    }
}
