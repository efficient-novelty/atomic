//! Replayable E-5 successor rerun over the naturality/orbit transport theorem.
//!
//! The shared transport closes the old direct-J3 quotient defect and constructs
//! the pointwise chronological specializations.  It does not define the action
//! demanded by the 17 unary seeds, nor an intrinsically typed output for the
//! historical structural-completion snapshots.  This certificate records that
//! exact mixed result and refuses to turn an undefined output into an F1
//! verdict, semantic `O(16)`, or any downstream authorization.

use crate::naturality_orbit_transport::{
    A3_STRUCTURAL_COMPLETION_OUTPUT_GAP, A3_UNARY_ACTION_OUTPUT_GAP, A3PendingOutput,
    NaturalityOrbitTransportCertificate, issue_naturality_orbit_transport_certificate,
    replay_naturality_orbit_transport_certificate,
};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const E5_A3_SUCCESSOR_V2_SCHEMA: &str = "schema2-e5-a3-successor-v2";
pub const E5_A3_SUCCESSOR_V2_DATE: &str = "2026-07-21";

const TRANSPORT_SOURCE_BYTES: &[u8] = include_bytes!("naturality_orbit_transport.rs");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const PROTOCOL_BYTES: &[u8] = include_bytes!("../../../docs/tie_resolution_protocol.md");
const THEOREM_BYTES: &[u8] = include_bytes!("../../../docs/t1_result.md");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("e5_a3_successor_v2.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3SuccessorV2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3ChronologicalTransportAudit {
    pub transport_schema: String,
    pub transport_result_digest: String,
    pub transport_replay_valid: bool,
    pub grammar_archive_digest_valid: bool,
    pub grammar_live_replay_valid: bool,
    pub grammar_live_replay_errors: Vec<String>,
    pub j3_predecessor_archive_digest_valid: bool,
    pub j3_predecessor_live_replay_valid: bool,
    pub j3_predecessor_live_replay_errors: Vec<String>,
    pub frozen_surface_drift_bound_explicitly: bool,
    pub direct_instance_count: usize,
    pub direct_natural_family_count: usize,
    pub direct_regression_64_to_8_proved: bool,
    pub pointwise_instance_count: usize,
    pub pointwise_natural_family_count: usize,
    pub pointwise_joined_to_direct_j3_families: bool,
    pub all_chronological_outputs_constructed_and_kernel_typed: bool,
    pub uniform_specializations_not_multiplied: bool,
    pub naturality_orbit_transport_boundary_complete: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3RemainingOutputAudit {
    pub unary_seed_count: usize,
    pub unary_registration_join_count: usize,
    pub unary_action_outputs_defined: bool,
    pub unary_pending_outputs: Vec<A3PendingOutput>,
    pub structural_completion_outputs_defined: bool,
    pub structural_completion_pending_outputs: Vec<A3PendingOutput>,
    pub source_reflexivity_used_as_action_output: bool,
    pub later_historical_answer_imported_as_completion_output: bool,
    pub undefined_outputs_defaulted_to_singleton_orbits: bool,
    pub full_a3_output_grammar_complete: bool,
    pub named_gaps: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3SuccessorV2F1Audit {
    pub full_a3_output_grammar_required: bool,
    pub full_a3_output_grammar_available: bool,
    pub f1_executable: bool,
    pub f1_executed: bool,
    pub demanded_but_underdetermined_instance: Option<String>,
    pub f1_triggered: bool,
    pub f1_excluded: bool,
    pub semantic_o16_empty: Option<bool>,
    pub theorem12_full_instance_granularity_proved: bool,
    pub theorem12_refuted: bool,
    pub disposition: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3SuccessorV2Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<E5A3SuccessorV2SourceBinding>,
    pub chronological_transport: E5A3ChronologicalTransportAudit,
    pub remaining_outputs: E5A3RemainingOutputAudit,
    pub f1: E5A3SuccessorV2F1Audit,
    pub e5_successor_rerun_attempted: bool,
    pub e5_complete: bool,
    pub semantic_o16_certificate_issued: bool,
    pub t_bf2_authorized: bool,
    pub t_bf2_executed: bool,
    pub bridge_authorized: bool,
    pub bridge_executed: bool,
    pub bar_free_adoption_authorized: bool,
    pub bar_free_law_adopted: bool,
    pub halt_claim_issued: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3SuccessorV2Replay {
    pub valid: bool,
    pub direct_regression_64_to_8_proved: bool,
    pub pointwise_joined_to_eight_families: bool,
    pub unary_pending_count: usize,
    pub structural_completion_pending_count: usize,
    pub full_a3_output_grammar_complete: bool,
    pub f1_executable: bool,
    pub f1_executed: bool,
    pub semantic_o16_empty: Option<bool>,
    pub e5_complete: bool,
    pub t_bf2_authorized: bool,
    pub bridge_authorized: bool,
    pub bar_free_adoption_authorized: bool,
    pub halt_claim_issued: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum E5A3SuccessorV2Error {
    #[error("transport prerequisite failed: {0}")]
    Transport(String),
    #[error("E-5 successor invariant failed: {0}")]
    Invariant(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(E5_A3_SUCCESSOR_V2_SCHEMA, domain, value))
        .expect("E-5 A3 successor v2 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<E5A3SuccessorV2SourceBinding> {
    [
        (
            "crates/pen-search/src/naturality_orbit_transport.rs",
            "shared_typed_naturality_and_orbit_transport_theorem",
            TRANSPORT_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "historical_A3_candidate_grammar_and_fail_closed_boundary",
            A3_SOURCE_BYTES,
        ),
        (
            "docs/tie_resolution_protocol.md",
            "adopted_build_and_rerun_sequence",
            PROTOCOL_BYTES,
        ),
        (
            "docs/t1_result.md",
            "Theorem_12_and_instance_granularity_boundary",
            THEOREM_BYTES,
        ),
        (
            "crates/pen-search/src/e5_a3_successor_v2.rs",
            "this_create_new_fail_closed_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| E5A3SuccessorV2SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn certificate_digest(certificate: &E5A3SuccessorV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn validate_transport(
    transport: &NaturalityOrbitTransportCertificate,
) -> Result<(), E5A3SuccessorV2Error> {
    let replay = replay_naturality_orbit_transport_certificate(transport);
    if !replay.valid {
        return Err(E5A3SuccessorV2Error::Transport(replay.errors.join("; ")));
    }
    let a3 = &transport.a3;
    if a3.direct_instance_count != 64
        || !a3.grammar_archive_digest_valid
        || !a3.j3_predecessor_archive_digest_valid
        || a3.direct_natural_family_count != 8
        || !a3.j3_regression_64_to_8
        || a3.pointwise_instance_count != 8
        || a3.pointwise_natural_family_count != 8
        || !a3.pointwise_instances_join_existing_j3_families
        || !a3.chronological_outputs_constructed_and_kernel_typed
    {
        return Err(E5A3SuccessorV2Error::Invariant(
            "transport does not carry the certified direct 64-to-8 and pointwise 8-to-same-8 theorem"
                .to_owned(),
        ));
    }
    if a3.unary_seed_count != 17
        || a3.unary_registration_join_count != 17
        || a3.unary_pending_outputs.len() != 17
        || !a3
            .unary_pending_outputs
            .iter()
            .all(|row| row.gap_id == A3_UNARY_ACTION_OUTPUT_GAP)
    {
        return Err(E5A3SuccessorV2Error::Invariant(
            "the 17 registered unary sources are not preserved as exactly 17 undefined action outputs"
                .to_owned(),
        ));
    }
    if a3.structural_completion_pending_outputs.is_empty()
        || !a3
            .structural_completion_pending_outputs
            .iter()
            .all(|row| row.gap_id == A3_STRUCTURAL_COMPLETION_OUTPUT_GAP)
    {
        return Err(E5A3SuccessorV2Error::Invariant(
            "historical structural-completion obligations were absent or promoted without typed outputs"
                .to_owned(),
        ));
    }
    if a3.full_a3_output_grammar_complete
        || a3.f1_executable
        || a3.semantic_o16_decided
        || transport.bridge_authorized
        || transport.bar_free_adoption_authorized
        || transport.halt_claim_issued
    {
        return Err(E5A3SuccessorV2Error::Invariant(
            "transport improperly promoted an incomplete A3 output grammar downstream".to_owned(),
        ));
    }
    Ok(())
}

pub fn issue_e5_a3_successor_v2_certificate()
-> Result<E5A3SuccessorV2Certificate, E5A3SuccessorV2Error> {
    let transport = issue_naturality_orbit_transport_certificate()
        .map_err(|error| E5A3SuccessorV2Error::Transport(error.to_string()))?;
    validate_transport(&transport)?;
    let a3 = &transport.a3;

    let naturality_orbit_transport_boundary_complete = a3.j3_regression_64_to_8
        && a3.pointwise_instances_join_existing_j3_families
        && a3.chronological_outputs_constructed_and_kernel_typed;
    let chronological_hash = tagged_hash(
        "chronological-transport",
        &(
            (
                &transport.result_digest,
                a3.grammar_archive_digest_valid,
                a3.grammar_live_replay_valid,
                &a3.grammar_live_replay_errors,
                a3.j3_predecessor_archive_digest_valid,
                a3.j3_predecessor_live_replay_valid,
                &a3.j3_predecessor_live_replay_errors,
                a3.frozen_surface_drift_bound_explicitly,
            ),
            (
                a3.direct_instance_count,
                a3.direct_natural_family_count,
                a3.j3_regression_64_to_8,
                a3.pointwise_instance_count,
                a3.pointwise_natural_family_count,
                a3.pointwise_instances_join_existing_j3_families,
                a3.chronological_outputs_constructed_and_kernel_typed,
                a3.remaining_non_j3_seeds_never_defaulted_to_singletons,
                naturality_orbit_transport_boundary_complete,
            ),
        ),
    );
    let chronological_transport = E5A3ChronologicalTransportAudit {
        transport_schema: transport.schema.clone(),
        transport_result_digest: transport.result_digest.clone(),
        transport_replay_valid: true,
        grammar_archive_digest_valid: a3.grammar_archive_digest_valid,
        grammar_live_replay_valid: a3.grammar_live_replay_valid,
        grammar_live_replay_errors: a3.grammar_live_replay_errors.clone(),
        j3_predecessor_archive_digest_valid: a3.j3_predecessor_archive_digest_valid,
        j3_predecessor_live_replay_valid: a3.j3_predecessor_live_replay_valid,
        j3_predecessor_live_replay_errors: a3.j3_predecessor_live_replay_errors.clone(),
        frozen_surface_drift_bound_explicitly: a3.frozen_surface_drift_bound_explicitly,
        direct_instance_count: a3.direct_instance_count,
        direct_natural_family_count: a3.direct_natural_family_count,
        direct_regression_64_to_8_proved: a3.j3_regression_64_to_8,
        pointwise_instance_count: a3.pointwise_instance_count,
        pointwise_natural_family_count: a3.pointwise_natural_family_count,
        pointwise_joined_to_direct_j3_families: a3.pointwise_instances_join_existing_j3_families,
        all_chronological_outputs_constructed_and_kernel_typed: a3
            .chronological_outputs_constructed_and_kernel_typed,
        uniform_specializations_not_multiplied: a3.j3_regression_64_to_8
            && a3.pointwise_instances_join_existing_j3_families,
        naturality_orbit_transport_boundary_complete,
        derivation_hash: chronological_hash,
    };

    let named_gaps = vec![
        A3_STRUCTURAL_COMPLETION_OUTPUT_GAP.to_owned(),
        A3_UNARY_ACTION_OUTPUT_GAP.to_owned(),
    ];
    let remaining_hash = tagged_hash(
        "remaining-output-boundary",
        &(
            a3.unary_seed_count,
            a3.unary_registration_join_count,
            &a3.unary_pending_outputs,
            &a3.structural_completion_pending_outputs,
            &named_gaps,
        ),
    );
    let remaining_outputs = E5A3RemainingOutputAudit {
        unary_seed_count: a3.unary_seed_count,
        unary_registration_join_count: a3.unary_registration_join_count,
        unary_action_outputs_defined: false,
        unary_pending_outputs: a3.unary_pending_outputs.clone(),
        structural_completion_outputs_defined: false,
        structural_completion_pending_outputs: a3.structural_completion_pending_outputs.clone(),
        source_reflexivity_used_as_action_output: false,
        later_historical_answer_imported_as_completion_output: false,
        undefined_outputs_defaulted_to_singleton_orbits: false,
        full_a3_output_grammar_complete: false,
        named_gaps,
        derivation_hash: remaining_hash,
    };

    let f1_disposition = "unexecuted_and_undecided: chronological transport is complete, but full A3 is not a typed output grammar until the unary action and structural-completion output terms are defined".to_owned();
    let f1_hash = tagged_hash(
        "f1-disposition",
        &(
            true,
            false,
            false,
            false,
            Option::<String>::None,
            false,
            false,
            Option::<bool>::None,
            false,
            false,
            &f1_disposition,
            &remaining_outputs.derivation_hash,
        ),
    );
    let f1 = E5A3SuccessorV2F1Audit {
        full_a3_output_grammar_required: true,
        full_a3_output_grammar_available: false,
        f1_executable: false,
        f1_executed: false,
        demanded_but_underdetermined_instance: None,
        f1_triggered: false,
        f1_excluded: false,
        semantic_o16_empty: None,
        theorem12_full_instance_granularity_proved: false,
        theorem12_refuted: false,
        disposition: f1_disposition,
        derivation_hash: f1_hash,
    };

    let mut certificate = E5A3SuccessorV2Certificate {
        schema: E5_A3_SUCCESSOR_V2_SCHEMA.to_owned(),
        date: E5_A3_SUCCESSOR_V2_DATE.to_owned(),
        source_bindings: source_bindings(),
        chronological_transport,
        remaining_outputs,
        f1,
        e5_successor_rerun_attempted: true,
        e5_complete: false,
        semantic_o16_certificate_issued: false,
        t_bf2_authorized: false,
        t_bf2_executed: false,
        bridge_authorized: false,
        bridge_executed: false,
        bar_free_adoption_authorized: false,
        bar_free_law_adopted: false,
        halt_claim_issued: false,
        outcome: "chronological_transport_proved_full_a3_stopped_on_undefined_unary_and_structural_outputs".to_owned(),
        permitted_conclusion: "The E-5 successor now earns the direct 64-to-8 J3 quotient and constructs the eight pointwise substitutions as members of those same eight typed natural families. The 17 unary sources are registered but their action outputs are undefined, and the historical structural snapshots still do not determine intrinsically typed completion outputs. F1 therefore has not run, semantic O(16) has no value, and neither success nor refutation of Theorem 12 follows.".to_owned(),
        required_successor_action: "Define and kernel-type the unary action outputs and historical structural-completion outputs without source reflexivity or retrospective answer import; extend the orbit transport over those terms; then rerun E-5. Do not execute T-BF2, the bridge, bar-free adoption, or a halt certificate from this artifact.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> E5A3SuccessorV2Replay {
    E5A3SuccessorV2Replay {
        valid: false,
        direct_regression_64_to_8_proved: false,
        pointwise_joined_to_eight_families: false,
        unary_pending_count: 0,
        structural_completion_pending_count: 0,
        full_a3_output_grammar_complete: false,
        f1_executable: false,
        f1_executed: false,
        semantic_o16_empty: None,
        e5_complete: false,
        t_bf2_authorized: false,
        bridge_authorized: false,
        bar_free_adoption_authorized: false,
        halt_claim_issued: false,
        outcome: "e5_a3_successor_v2_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_e5_a3_successor_v2_certificate(
    certificate: &E5A3SuccessorV2Certificate,
) -> E5A3SuccessorV2Replay {
    let expected = match issue_e5_a3_successor_v2_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != &expected {
        errors.push("certificate differs from independent definition replay".to_owned());
    }
    E5A3SuccessorV2Replay {
        valid: errors.is_empty(),
        direct_regression_64_to_8_proved: certificate
            .chronological_transport
            .direct_regression_64_to_8_proved,
        pointwise_joined_to_eight_families: certificate
            .chronological_transport
            .pointwise_joined_to_direct_j3_families,
        unary_pending_count: certificate.remaining_outputs.unary_pending_outputs.len(),
        structural_completion_pending_count: certificate
            .remaining_outputs
            .structural_completion_pending_outputs
            .len(),
        full_a3_output_grammar_complete: certificate
            .remaining_outputs
            .full_a3_output_grammar_complete,
        f1_executable: certificate.f1.f1_executable,
        f1_executed: certificate.f1.f1_executed,
        semantic_o16_empty: certificate.f1.semantic_o16_empty,
        e5_complete: certificate.e5_complete,
        t_bf2_authorized: certificate.t_bf2_authorized,
        bridge_authorized: certificate.bridge_authorized,
        bar_free_adoption_authorized: certificate.bar_free_adoption_authorized,
        halt_claim_issued: certificate.halt_claim_issued,
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_e5_a3_successor_v2_json(json: &str) -> E5A3SuccessorV2Replay {
    match serde_json::from_str::<E5A3SuccessorV2Certificate>(json) {
        Ok(certificate) => replay_e5_a3_successor_v2_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_e5_a3_successor_v2_create_new(
    path: &Path,
) -> Result<E5A3SuccessorV2Replay, E5A3SuccessorV2Error> {
    let certificate = issue_e5_a3_successor_v2_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| E5A3SuccessorV2Error::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| E5A3SuccessorV2Error::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| E5A3SuccessorV2Error::Io(error.to_string()))?;
    let replay = replay_e5_a3_successor_v2_certificate(&certificate);
    if !replay.valid {
        return Err(E5A3SuccessorV2Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn certificate() -> E5A3SuccessorV2Certificate {
        static CERTIFICATE: OnceLock<E5A3SuccessorV2Certificate> = OnceLock::new();
        CERTIFICATE
            .get_or_init(|| issue_e5_a3_successor_v2_certificate().unwrap())
            .clone()
    }

    #[test]
    fn chronological_surface_now_passes_the_64_to_8_and_pointwise_regressions() {
        let certificate = certificate();
        assert_eq!(
            certificate.chronological_transport.direct_instance_count,
            64
        );
        assert_eq!(
            certificate
                .chronological_transport
                .direct_natural_family_count,
            8
        );
        assert!(
            certificate
                .chronological_transport
                .direct_regression_64_to_8_proved
        );
        assert_eq!(
            certificate.chronological_transport.pointwise_instance_count,
            8
        );
        assert!(
            certificate
                .chronological_transport
                .pointwise_joined_to_direct_j3_families
        );
        assert!(
            certificate
                .chronological_transport
                .naturality_orbit_transport_boundary_complete
        );
    }

    #[test]
    fn undefined_outputs_leave_f1_and_semantic_o16_undecided() {
        let certificate = certificate();
        assert_eq!(certificate.remaining_outputs.unary_seed_count, 17);
        assert_eq!(
            certificate.remaining_outputs.unary_pending_outputs.len(),
            17
        );
        assert!(
            !certificate
                .remaining_outputs
                .structural_completion_pending_outputs
                .is_empty()
        );
        assert!(
            !certificate
                .remaining_outputs
                .full_a3_output_grammar_complete
        );
        assert!(!certificate.f1.f1_executable);
        assert!(!certificate.f1.f1_executed);
        assert!(!certificate.f1.f1_triggered);
        assert!(!certificate.f1.f1_excluded);
        assert_eq!(certificate.f1.semantic_o16_empty, None);
        assert!(!certificate.e5_complete);
        assert!(!certificate.t_bf2_authorized);
        assert!(!certificate.bridge_authorized);
        assert!(!certificate.bar_free_adoption_authorized);
        assert!(!certificate.halt_claim_issued);
    }

    #[test]
    fn replay_rejects_a_redigested_false_promotion() {
        let mut forged = certificate();
        forged.remaining_outputs.full_a3_output_grammar_complete = true;
        forged.f1.full_a3_output_grammar_available = true;
        forged.f1.f1_executable = true;
        forged.f1.f1_executed = true;
        forged.f1.f1_excluded = true;
        forged.f1.semantic_o16_empty = Some(true);
        forged.e5_complete = true;
        forged.semantic_o16_certificate_issued = true;
        forged.t_bf2_authorized = true;
        forged.bridge_authorized = true;
        forged.bar_free_adoption_authorized = true;
        forged.halt_claim_issued = true;
        forged.result_digest = certificate_digest(&forged);
        assert!(!replay_e5_a3_successor_v2_certificate(&forged).valid);
    }
}
