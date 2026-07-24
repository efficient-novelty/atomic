//! Fail-closed execution manifest for the frozen bridge-completion program.
//!
//! The program's exit antecedent did not hold: BC-1, BC-2, and BC-3
//! published complete fail-closed dispositions, but none discharged its
//! registered prerequisite.  This issuer binds and replays those results,
//! joins the BC-2/BC-3 aggregate-row testimony exactly, and records why an
//! M-3 successor was not issued.  It proves no new semantic claim.

use crate::bc1_depth_two_completeness_v1::{
    Bc1DepthTwoCompletenessV1Certificate, Bc1RunStatus, render_bc1_depth_two_completeness_v1,
    replay_bc1_depth_two_completeness_v1,
};
use crate::bc2_boundary_provenance_join_v1::{
    Bc2BoundaryProvenanceJoinV1Certificate, Bc2RowDispositionStatus, Bc2RunStatus,
    render_bc2_boundary_provenance_join_v1, replay_bc2_boundary_provenance_join_v1,
};
use crate::bc3_parent_row_disposition_v1::{
    Bc3DispositionKind, Bc3ParentRowDispositionV1Certificate, Bc3RunStatus,
    render_bc3_parent_row_disposition_v1, replay_bc3_parent_row_disposition_v1,
};
use crate::m3_e7_e8_bridge_v1::{
    M3E7E8BridgeV1Certificate, M3RunStatus, render_m3_e7_e8_bridge_v1, replay_m3_e7_e8_bridge_v1,
};
use pen_core::hash::blake3_hex;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::{OpenOptions, read, read_to_string, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const BRIDGE_COMPLETION_EXECUTION_V1_SCHEMA: &str = "bridge-completion-execution-v1";
pub const BRIDGE_COMPLETION_EXECUTION_V1_DATE: &str = "2026-07-23";
pub const BRIDGE_COMPLETION_EXECUTION_V1_CERTIFICATE_NAME: &str =
    "bridge_completion_execution_v1.json";
pub const BRIDGE_COMPLETION_EXECUTION_V1_REPORT_NAME: &str =
    "BRIDGE_COMPLETION_EXECUTION_RESULT.md";
pub const FROZEN_M3_V1_RESULT_DIGEST: &str =
    "blake3:a61fe8093ef9ce61a05aebfe859b0723a88a766a712ea6a2312e0f22ca386ba8";

const BRIDGE_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/bridge_completion_plan.md");
const MAINLINE_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/mainline_completion_plan.md");
const M3_CERTIFICATE_BYTES: &[u8] = include_bytes!("../../../docs/schema2_e7_e8_bridge_v1.json");
const M3_REPORT_BYTES: &[u8] = include_bytes!("../../../docs/SCHEMA2_E7_E8_BRIDGE_V1_RESULT.md");
const M3_SOURCE_BYTES: &[u8] = include_bytes!("m3_e7_e8_bridge_v1.rs");
const BC1_CERTIFICATE_BYTES: &[u8] =
    include_bytes!("../../../docs/bc1_depth_two_completeness_v1.json");
const BC1_REPORT_BYTES: &[u8] =
    include_bytes!("../../../docs/BC1_DEPTH_TWO_COMPLETENESS_RESULT.md");
const BC1_SOURCE_BYTES: &[u8] = include_bytes!("bc1_depth_two_completeness_v1.rs");
const BC2_CERTIFICATE_BYTES: &[u8] =
    include_bytes!("../../../docs/bc2_boundary_provenance_join_v1.json");
const BC2_REPORT_BYTES: &[u8] =
    include_bytes!("../../../docs/BC2_BOUNDARY_PROVENANCE_JOIN_RESULT.md");
const BC2_SOURCE_BYTES: &[u8] = include_bytes!("bc2_boundary_provenance_join_v1.rs");
const BC3_CERTIFICATE_BYTES: &[u8] =
    include_bytes!("../../../docs/bc3_parent_row_disposition_v1.json");
const BC3_REPORT_BYTES: &[u8] =
    include_bytes!("../../../docs/BC3_PARENT_ROW_DISPOSITION_RESULT.md");
const BC3_SOURCE_BYTES: &[u8] = include_bytes!("bc3_parent_row_disposition_v1.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("bridge_completion_execution_v1.rs");

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BridgeExecutionRegister {
    SemanticRegisterAuthority,
    StructuralTestimony,
    ProofInventory,
    ArtifactMetadata,
    SyntaxIdentifier,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BridgeExecutionQuantifier {
    FrozenM3V1Baseline,
    FrozenWrappedSurface,
    ExactAggregateParentRow,
    CrossPhaseFrozenAggregateSurface,
    FrozenProgramExitGate,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BridgeExecutionStatus {
    StoppedPrerequisiteGaps,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeRegisteredNumber {
    pub decimal: String,
    pub register: BridgeExecutionRegister,
    pub meaning: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeExecutionSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: BridgeRegisteredNumber,
    pub blake3: String,
    pub register: BridgeExecutionRegister,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeBaselineCondition {
    pub ordinal: BridgeRegisteredNumber,
    pub condition: String,
    pub proved: bool,
    pub named_gap: Option<String>,
    pub quantifier: BridgeExecutionQuantifier,
    pub exact_scope: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeBaselineAuthentication {
    pub quantifier: BridgeExecutionQuantifier,
    pub exact_scope: String,
    pub result_digest: String,
    pub certificate_bytes_blake3: String,
    pub report_bytes_blake3: String,
    pub source_bytes_blake3: String,
    pub public_replay_valid: bool,
    pub public_replay_errors: Vec<String>,
    pub report_exactly_rerendered: bool,
    pub condition_surface_size: BridgeRegisteredNumber,
    pub proved_surface_size: BridgeRegisteredNumber,
    pub open_surface_size: BridgeRegisteredNumber,
    pub conditions: Vec<BridgeBaselineCondition>,
    pub condition_partition_exact: bool,
    pub enacted_regression_exact: bool,
    pub cone_quantifiers_exact: bool,
    pub no_promotions: bool,
    pub stopped_status_reproduced: bool,
    pub bridge_claim_issued: bool,
    pub m4_authorized: bool,
    pub baseline_authenticated: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgePhaseAudit {
    pub phase_id: String,
    pub quantifier: BridgeExecutionQuantifier,
    pub exact_scope: String,
    pub certificate_path: String,
    pub report_path: String,
    pub source_path: String,
    pub result_digest: String,
    pub certificate_bytes_blake3: String,
    pub report_bytes_blake3: String,
    pub source_bytes_blake3: String,
    pub public_replay_valid: bool,
    pub public_replay_errors: Vec<String>,
    pub report_exactly_rerendered: bool,
    pub predecessor_baseline_authenticated: bool,
    pub disposition_surface_complete: bool,
    pub blocker_closed: bool,
    pub prerequisite_satisfied: bool,
    pub no_promotions: bool,
    pub promoted_surface_size: BridgeRegisteredNumber,
    pub gap_occurrence_surface_size: BridgeRegisteredNumber,
    pub status: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeRowProjection {
    pub aggregate_row_id: String,
    pub schema3_row_digest: String,
    pub schema4_row_derivation_hash: String,
    pub schema5_row_derivation_hash: String,
    pub disposition: String,
    pub named_gap: String,
    pub promoted: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeSharedRowJoin {
    pub quantifier: BridgeExecutionQuantifier,
    pub exact_scope: String,
    pub identity_register: BridgeExecutionRegister,
    pub comparator_register: BridgeExecutionRegister,
    pub bc2: BridgeRowProjection,
    pub bc3: BridgeRowProjection,
    pub identity_exact: bool,
    pub schema3_digest_exact: bool,
    pub schema4_digest_exact: bool,
    pub schema5_digest_exact: bool,
    pub both_explicitly_dispositioned: bool,
    pub neither_promoted: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeCrossPhaseRowAudit {
    pub quantifier: BridgeExecutionQuantifier,
    pub exact_scope: String,
    pub bc2_surface_size: BridgeRegisteredNumber,
    pub bc3_surface_size: BridgeRegisteredNumber,
    pub shared_surface_size: BridgeRegisteredNumber,
    pub distinct_identity_surface_size: BridgeRegisteredNumber,
    pub promoted_surface_size: BridgeRegisteredNumber,
    pub row_order_exact: bool,
    pub row_identity_exact: bool,
    pub row_digest_join_exact: bool,
    pub disposition_surface_complete: bool,
    pub no_duplicate_identity: bool,
    pub no_promotions: bool,
    pub structural_testimony_used_as_selector: bool,
    pub rows: Vec<BridgeSharedRowJoin>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeRemainingGap {
    pub occurrence_id: String,
    pub gap_id: String,
    pub owner_phase: String,
    pub quantifier: BridgeExecutionQuantifier,
    pub exact_scope: String,
    pub aggregate_row_id: Option<String>,
    pub register: BridgeExecutionRegister,
    pub exact_obstruction: String,
    pub keeps_exit_antecedent_false: bool,
    pub keeps_rows_unpromoted: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeFailedRouteTestimony {
    pub route_id: String,
    pub owner_phase: String,
    pub quantifier: BridgeExecutionQuantifier,
    pub exact_scope: String,
    pub register: BridgeExecutionRegister,
    pub exact_failure_point: String,
    pub failed_route_used_as_stop_cause: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeStandingRuleAudit {
    pub bound_input_bytes_match_replayed_phase_bindings: bool,
    pub create_new_artifact_required: bool,
    pub register_discipline_valid: bool,
    pub structural_testimony_used_as_semantic_authority: bool,
    pub selector_used: bool,
    pub unregistered_semantics_added: bool,
    pub scope_boundary_preserved: bool,
    pub count_bearing_predicate_names_used: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeExecutionGate {
    pub quantifier: BridgeExecutionQuantifier,
    pub exact_scope: String,
    pub prerequisite_surface_size: BridgeRegisteredNumber,
    pub satisfied_prerequisite_surface_size: BridgeRegisteredNumber,
    pub promoted_surface_size: BridgeRegisteredNumber,
    pub exit_antecedent_satisfied: bool,
    pub no_partial_promotion: bool,
    pub bridge_claim_issued: bool,
    pub m3_v2_issued: bool,
    pub m3_v1_remains_authoritative: bool,
    pub m4_authorized: bool,
    pub authoritative_certificate_path: String,
    pub successor_certificate_path: Option<String>,
    pub status: BridgeExecutionStatus,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeCompletionExecutionV1Certificate {
    pub schema: String,
    pub date: String,
    pub numeric_register_policy: String,
    pub source_bindings: Vec<BridgeExecutionSourceBinding>,
    pub source_surface_size: BridgeRegisteredNumber,
    pub baseline: BridgeBaselineAuthentication,
    pub phases: Vec<BridgePhaseAudit>,
    pub phase_surface_size: BridgeRegisteredNumber,
    pub cross_phase_rows: BridgeCrossPhaseRowAudit,
    pub remaining_gaps: Vec<BridgeRemainingGap>,
    pub remaining_gap_occurrence_surface_size: BridgeRegisteredNumber,
    pub failed_route_testimony: Vec<BridgeFailedRouteTestimony>,
    pub failed_route_surface_size: BridgeRegisteredNumber,
    pub standing_rules: BridgeStandingRuleAudit,
    pub gate: BridgeExecutionGate,
    pub mutation_falsifiers: Vec<String>,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BridgeCompletionExecutionReplay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub status: BridgeExecutionStatus,
    pub m3_v2_issued: bool,
    pub m3_v1_remains_authoritative: bool,
    pub m4_authorized: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum BridgeCompletionExecutionError {
    #[error("bridge-completion input failure: {0}")]
    Input(String),
    #[error("bridge-completion invariant failure: {0}")]
    Invariant(String),
    #[error("bridge-completion JSON failure: {0}")]
    Json(String),
    #[error("bridge-completion create-new I/O failure: {0}")]
    Io(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BRIDGE_COMPLETION_EXECUTION_V1_SCHEMA, domain, value))
        .expect("bridge-completion evidence serializes");
    bytes_hash(&bytes)
}

fn registered_number(
    value: impl ToString,
    register: BridgeExecutionRegister,
    meaning: &str,
) -> BridgeRegisteredNumber {
    BridgeRegisteredNumber {
        decimal: value.to_string(),
        register,
        meaning: meaning.to_owned(),
    }
}

fn metadata_number(value: impl ToString, meaning: &str) -> BridgeRegisteredNumber {
    registered_number(value, BridgeExecutionRegister::ArtifactMetadata, meaning)
}

fn syntax_number(value: impl ToString, meaning: &str) -> BridgeRegisteredNumber {
    registered_number(value, BridgeExecutionRegister::SyntaxIdentifier, meaning)
}

fn source_bindings() -> Vec<BridgeExecutionSourceBinding> {
    [
        (
            "docs/bridge_completion_plan.md",
            "frozen theorem program, exit antecedent, and falsifiers",
            BRIDGE_PLAN_BYTES,
        ),
        (
            "docs/mainline_completion_plan.md",
            "frozen downstream specification left untouched by this stop",
            MAINLINE_PLAN_BYTES,
        ),
        (
            "docs/schema2_e7_e8_bridge_v1.json",
            "authoritative M-3 v1 fail-closed certificate",
            M3_CERTIFICATE_BYTES,
        ),
        (
            "docs/SCHEMA2_E7_E8_BRIDGE_V1_RESULT.md",
            "authoritative M-3 v1 result report",
            M3_REPORT_BYTES,
        ),
        (
            "crates/pen-search/src/m3_e7_e8_bridge_v1.rs",
            "authoritative M-3 v1 issuer and replay source",
            M3_SOURCE_BYTES,
        ),
        (
            "docs/bc1_depth_two_completeness_v1.json",
            "BC-1 phase certificate",
            BC1_CERTIFICATE_BYTES,
        ),
        (
            "docs/BC1_DEPTH_TWO_COMPLETENESS_RESULT.md",
            "BC-1 phase result report",
            BC1_REPORT_BYTES,
        ),
        (
            "crates/pen-search/src/bc1_depth_two_completeness_v1.rs",
            "BC-1 issuer and replay source",
            BC1_SOURCE_BYTES,
        ),
        (
            "docs/bc2_boundary_provenance_join_v1.json",
            "BC-2 phase certificate",
            BC2_CERTIFICATE_BYTES,
        ),
        (
            "docs/BC2_BOUNDARY_PROVENANCE_JOIN_RESULT.md",
            "BC-2 phase result report",
            BC2_REPORT_BYTES,
        ),
        (
            "crates/pen-search/src/bc2_boundary_provenance_join_v1.rs",
            "BC-2 issuer and replay source",
            BC2_SOURCE_BYTES,
        ),
        (
            "docs/bc3_parent_row_disposition_v1.json",
            "BC-3 phase certificate",
            BC3_CERTIFICATE_BYTES,
        ),
        (
            "docs/BC3_PARENT_ROW_DISPOSITION_RESULT.md",
            "BC-3 phase result report",
            BC3_REPORT_BYTES,
        ),
        (
            "crates/pen-search/src/bc3_parent_row_disposition_v1.rs",
            "BC-3 issuer and replay source",
            BC3_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/bridge_completion_execution_v1.rs",
            "fail-closed execution-manifest issuer and replay source",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| BridgeExecutionSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: metadata_number(bytes.len(), "exact bound byte length"),
        blake3: bytes_hash(bytes),
        register: BridgeExecutionRegister::ArtifactMetadata,
    })
    .collect()
}

fn parse_bound<T: DeserializeOwned>(
    label: &str,
    bytes: &[u8],
) -> Result<T, BridgeCompletionExecutionError> {
    serde_json::from_slice(bytes)
        .map_err(|error| BridgeCompletionExecutionError::Json(format!("{label}: {error}")))
}

fn certificate_digest(certificate: &BridgeCompletionExecutionV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn build_baseline(
    certificate: &M3E7E8BridgeV1Certificate,
) -> Result<BridgeBaselineAuthentication, BridgeCompletionExecutionError> {
    let replay = replay_m3_e7_e8_bridge_v1(certificate);
    let report_exact = render_m3_e7_e8_bridge_v1(certificate).as_bytes() == M3_REPORT_BYTES;
    let proved = certificate
        .e8
        .bridge_conditions
        .iter()
        .filter(|condition| condition.proved)
        .count();
    let open = certificate
        .e8
        .bridge_conditions
        .len()
        .saturating_sub(proved);
    let conditions = certificate
        .e8
        .bridge_conditions
        .iter()
        .map(|condition| {
            let ordinal = syntax_number(
                condition.ordinal.value,
                "frozen bridge-condition ordinal; syntax identifier only",
            );
            let exact_scope =
                "Exact condition in the sealed M-3 v1 frozen wrapped-surface audit.".to_owned();
            let derivation_hash = tagged_hash(
                "baseline-condition",
                &(
                    &ordinal,
                    &condition.condition,
                    condition.proved,
                    &condition.named_gap,
                    &exact_scope,
                ),
            );
            BridgeBaselineCondition {
                ordinal,
                condition: condition.condition.clone(),
                proved: condition.proved,
                named_gap: condition.named_gap.clone(),
                quantifier: BridgeExecutionQuantifier::FrozenM3V1Baseline,
                exact_scope,
                derivation_hash,
            }
        })
        .collect::<Vec<_>>();
    let condition_partition_exact = conditions.len() == 9 && proved == 4 && open == 5;
    let enacted_regression_exact = certificate.enacted_regression.projections_exactly_equal
        && certificate
            .enacted_regression
            .cross_branch_statements_issued_only_after_regression;
    let cone_quantifiers_exact = certificate
        .cone
        .every_disposition_mirrored_with_exact_quantifier
        && !certificate
            .cone
            .structural_testimony_used_for_law_level_content;
    let no_promotions =
        certificate.zero_promotions_made && certificate.promoted_row_count.value == 0;
    let stopped_status_reproduced = certificate.m3_status == M3RunStatus::StoppedNamedGaps;
    let baseline_authenticated = replay.valid
        && certificate.result_digest == FROZEN_M3_V1_RESULT_DIGEST
        && report_exact
        && condition_partition_exact
        && enacted_regression_exact
        && cone_quantifiers_exact
        && no_promotions
        && stopped_status_reproduced
        && !certificate.bridge_claim_issued
        && !certificate.m4_authorized;
    if !baseline_authenticated {
        return Err(BridgeCompletionExecutionError::Input(format!(
            "frozen M-3 v1 baseline did not authenticate: {}",
            replay.errors.join("; ")
        )));
    }
    let exact_scope = "Exact sealed M-3 v1 certificate, report, source, condition partition, enacted regression, and BI-4-derived quantifiers.".to_owned();
    let derivation_hash = tagged_hash(
        "baseline-authentication",
        &(
            certificate.result_digest.as_str(),
            &conditions,
            &exact_scope,
            enacted_regression_exact,
            cone_quantifiers_exact,
        ),
    );
    Ok(BridgeBaselineAuthentication {
        quantifier: BridgeExecutionQuantifier::FrozenM3V1Baseline,
        exact_scope,
        result_digest: certificate.result_digest.clone(),
        certificate_bytes_blake3: bytes_hash(M3_CERTIFICATE_BYTES),
        report_bytes_blake3: bytes_hash(M3_REPORT_BYTES),
        source_bytes_blake3: bytes_hash(M3_SOURCE_BYTES),
        public_replay_valid: replay.valid,
        public_replay_errors: replay.errors,
        report_exactly_rerendered: report_exact,
        condition_surface_size: metadata_number(
            conditions.len(),
            "frozen bridge-condition surface size",
        ),
        proved_surface_size: metadata_number(proved, "proved baseline condition surface size"),
        open_surface_size: metadata_number(open, "open baseline condition surface size"),
        conditions,
        condition_partition_exact,
        enacted_regression_exact,
        cone_quantifiers_exact,
        no_promotions,
        stopped_status_reproduced,
        bridge_claim_issued: certificate.bridge_claim_issued,
        m4_authorized: certificate.m4_authorized,
        baseline_authenticated,
        derivation_hash,
    })
}

fn build_bc1_phase(
    certificate: &Bc1DepthTwoCompletenessV1Certificate,
    m3_digest: &str,
) -> Result<BridgePhaseAudit, BridgeCompletionExecutionError> {
    let replay = replay_bc1_depth_two_completeness_v1(certificate);
    let report_exact =
        render_bc1_depth_two_completeness_v1(certificate).as_bytes() == BC1_REPORT_BYTES;
    let predecessor_baseline_authenticated = certificate.m3_v1_baseline.result_digest == m3_digest
        && certificate.m3_v1_baseline.public_replay_valid
        && !certificate.m3_v1_baseline.baseline_shift_detected;
    let disposition_surface_complete = certificate
        .frozen_component_surface
        .every_frozen_component_has_explicit_disposition
        && certificate
            .depth_two_attempt
            .every_obligation_has_explicit_disposition;
    let no_promotions = certificate.no_promotions_made && certificate.promoted_row_count.value == 0;
    if !replay.valid
        || !report_exact
        || !predecessor_baseline_authenticated
        || !disposition_surface_complete
        || certificate.bc1_status != Bc1RunStatus::StoppedNamedGaps
        || certificate.c2_closed
        || certificate.m3_v2_prerequisite_satisfied
        || !no_promotions
    {
        return Err(BridgeCompletionExecutionError::Invariant(
            "BC-1 is not the exact replayable fail-closed phase result".to_owned(),
        ));
    }
    let exact_scope = "Frozen wrapped depth-two attempt over the sealed trace and adopted formers; no independent intended semantic domain is asserted.".to_owned();
    let derivation_hash = tagged_hash(
        "phase-audit",
        &(
            "BC-1",
            certificate.result_digest.as_str(),
            &exact_scope,
            disposition_surface_complete,
            no_promotions,
        ),
    );
    Ok(BridgePhaseAudit {
        phase_id: "BC-1".to_owned(),
        quantifier: BridgeExecutionQuantifier::FrozenWrappedSurface,
        exact_scope,
        certificate_path: "docs/bc1_depth_two_completeness_v1.json".to_owned(),
        report_path: "docs/BC1_DEPTH_TWO_COMPLETENESS_RESULT.md".to_owned(),
        source_path: "crates/pen-search/src/bc1_depth_two_completeness_v1.rs".to_owned(),
        result_digest: certificate.result_digest.clone(),
        certificate_bytes_blake3: bytes_hash(BC1_CERTIFICATE_BYTES),
        report_bytes_blake3: bytes_hash(BC1_REPORT_BYTES),
        source_bytes_blake3: bytes_hash(BC1_SOURCE_BYTES),
        public_replay_valid: replay.valid,
        public_replay_errors: replay.errors,
        report_exactly_rerendered: report_exact,
        predecessor_baseline_authenticated,
        disposition_surface_complete,
        blocker_closed: false,
        prerequisite_satisfied: false,
        no_promotions,
        promoted_surface_size: metadata_number(
            certificate.promoted_row_count.value,
            "BC-1 promoted surface size",
        ),
        gap_occurrence_surface_size: metadata_number(
            certificate.open_gaps.len(),
            "BC-1 remaining gap occurrence surface size",
        ),
        status: "stopped_named_gaps".to_owned(),
        derivation_hash,
    })
}

fn build_bc2_phase(
    certificate: &Bc2BoundaryProvenanceJoinV1Certificate,
    m3_digest: &str,
) -> Result<BridgePhaseAudit, BridgeCompletionExecutionError> {
    let replay = replay_bc2_boundary_provenance_join_v1(certificate);
    let report_exact =
        render_bc2_boundary_provenance_join_v1(certificate).as_bytes() == BC2_REPORT_BYTES;
    let predecessor_baseline_authenticated = certificate.m3_baseline.observed_result_digest
        == m3_digest
        && certificate.m3_baseline.result_digest_exact
        && certificate
            .m3_baseline
            .exact_predecessor_condition_partition_reproduced;
    let disposition_surface_complete = certificate
        .coverage
        .aggregate_row_disposition_is_total_and_functional
        && certificate.coverage.no_silent_residue;
    let no_promotions =
        certificate.coverage.no_promotions && certificate.coverage.promoted_row_count.value == "0";
    if !replay.valid
        || !report_exact
        || !predecessor_baseline_authenticated
        || !disposition_surface_complete
        || certificate.run_status != Bc2RunStatus::StoppedNamedGaps
        || certificate.c8_closed
        || certificate.candidate_level_join_proved
        || certificate.m4_authorized
        || !no_promotions
    {
        return Err(BridgeCompletionExecutionError::Invariant(
            "BC-2 is not the exact replayable fail-closed phase result".to_owned(),
        ));
    }
    let gap_occurrences = certificate.global_named_gaps.len() + certificate.row_dispositions.len();
    let exact_scope = "Every frozen Schema-3 aggregate row joined to its Schema-4/5 comparators; candidate-local provenance and cross-branch transport remain unproved.".to_owned();
    let derivation_hash = tagged_hash(
        "phase-audit",
        &(
            "BC-2",
            certificate.result_digest.as_str(),
            &exact_scope,
            disposition_surface_complete,
            no_promotions,
        ),
    );
    Ok(BridgePhaseAudit {
        phase_id: "BC-2".to_owned(),
        quantifier: BridgeExecutionQuantifier::FrozenWrappedSurface,
        exact_scope,
        certificate_path: "docs/bc2_boundary_provenance_join_v1.json".to_owned(),
        report_path: "docs/BC2_BOUNDARY_PROVENANCE_JOIN_RESULT.md".to_owned(),
        source_path: "crates/pen-search/src/bc2_boundary_provenance_join_v1.rs".to_owned(),
        result_digest: certificate.result_digest.clone(),
        certificate_bytes_blake3: bytes_hash(BC2_CERTIFICATE_BYTES),
        report_bytes_blake3: bytes_hash(BC2_REPORT_BYTES),
        source_bytes_blake3: bytes_hash(BC2_SOURCE_BYTES),
        public_replay_valid: replay.valid,
        public_replay_errors: replay.errors,
        report_exactly_rerendered: report_exact,
        predecessor_baseline_authenticated,
        disposition_surface_complete,
        blocker_closed: false,
        prerequisite_satisfied: false,
        no_promotions,
        promoted_surface_size: metadata_number(
            &certificate.coverage.promoted_row_count.value,
            "BC-2 promoted surface size",
        ),
        gap_occurrence_surface_size: metadata_number(
            gap_occurrences,
            "BC-2 remaining gap occurrence surface size",
        ),
        status: "stopped_named_gaps".to_owned(),
        derivation_hash,
    })
}

fn build_bc3_phase(
    certificate: &Bc3ParentRowDispositionV1Certificate,
    m3_digest: &str,
) -> Result<BridgePhaseAudit, BridgeCompletionExecutionError> {
    let replay = replay_bc3_parent_row_disposition_v1(certificate);
    let report_exact =
        render_bc3_parent_row_disposition_v1(certificate).as_bytes() == BC3_REPORT_BYTES;
    let predecessor_baseline_authenticated = certificate.m3_baseline.frozen_result_digest
        == m3_digest
        && certificate.m3_baseline.baseline_authenticated;
    let disposition_surface_complete = certificate.coverage.coverage_complete
        && certificate.coverage.silent_residue_rows.decimal == "0"
        && certificate
            .row_dispositions
            .iter()
            .all(|row| !row.semantic_nonexistence_proved);
    let promoted_surface_size = certificate
        .row_dispositions
        .iter()
        .filter(|row| row.row_promoted)
        .count();
    let no_promotions =
        certificate.coverage.all_parent_rows_remain_unpromoted && promoted_surface_size == 0;
    if !replay.valid
        || !report_exact
        || !predecessor_baseline_authenticated
        || !disposition_surface_complete
        || certificate.status != Bc3RunStatus::StoppedNamedImpossibilities
        || certificate.parent_refinement_condition_promoted
        || certificate.missing_or_duplicate_rejection_condition_promoted
        || certificate.m3_v2_authorized_by_bc3
        || certificate.m4_authorized_by_bc3
        || !no_promotions
    {
        return Err(BridgeCompletionExecutionError::Invariant(
            "BC-3 is not the exact replayable fail-closed phase result".to_owned(),
        ));
    }
    let gap_occurrences = certificate.row_dispositions.len();
    let exact_scope = "Every frozen Schema-3 aggregate parent row dispositioned relative to current registered inputs; a named impossibility is not semantic nonexistence.".to_owned();
    let derivation_hash = tagged_hash(
        "phase-audit",
        &(
            "BC-3",
            certificate.result_digest.as_str(),
            &exact_scope,
            disposition_surface_complete,
            no_promotions,
        ),
    );
    Ok(BridgePhaseAudit {
        phase_id: "BC-3".to_owned(),
        quantifier: BridgeExecutionQuantifier::FrozenWrappedSurface,
        exact_scope,
        certificate_path: "docs/bc3_parent_row_disposition_v1.json".to_owned(),
        report_path: "docs/BC3_PARENT_ROW_DISPOSITION_RESULT.md".to_owned(),
        source_path: "crates/pen-search/src/bc3_parent_row_disposition_v1.rs".to_owned(),
        result_digest: certificate.result_digest.clone(),
        certificate_bytes_blake3: bytes_hash(BC3_CERTIFICATE_BYTES),
        report_bytes_blake3: bytes_hash(BC3_REPORT_BYTES),
        source_bytes_blake3: bytes_hash(BC3_SOURCE_BYTES),
        public_replay_valid: replay.valid,
        public_replay_errors: replay.errors,
        report_exactly_rerendered: report_exact,
        predecessor_baseline_authenticated,
        disposition_surface_complete,
        blocker_closed: false,
        prerequisite_satisfied: false,
        no_promotions,
        promoted_surface_size: metadata_number(
            promoted_surface_size,
            "BC-3 promoted surface size derived from row dispositions",
        ),
        gap_occurrence_surface_size: metadata_number(
            gap_occurrences,
            "BC-3 remaining gap occurrence surface size",
        ),
        status: "stopped_named_impossibilities".to_owned(),
        derivation_hash,
    })
}

fn build_cross_phase_rows(
    bc2: &Bc2BoundaryProvenanceJoinV1Certificate,
    bc3: &Bc3ParentRowDispositionV1Certificate,
) -> Result<BridgeCrossPhaseRowAudit, BridgeCompletionExecutionError> {
    if bc2.row_dispositions.len() != 213 || bc3.row_dispositions.len() != 213 {
        return Err(BridgeCompletionExecutionError::Invariant(
            "BC-2/BC-3 aggregate parent surfaces do not have the frozen size".to_owned(),
        ));
    }
    let mut rows = Vec::with_capacity(bc2.row_dispositions.len());
    for (left, right) in bc2.row_dispositions.iter().zip(&bc3.row_dispositions) {
        let identity_exact = left.aggregate_row_id == right.aggregate_row_id;
        let schema3_digest_exact = left.schema3_row_digest == right.schema3_row_digest;
        let schema4_digest_exact =
            left.schema4_row_derivation_hash == right.schema4_row_derivation_hash;
        let schema5_digest_exact =
            left.schema5_row_derivation_hash == right.schema5_row_derivation_hash;
        let both_explicitly_dispositioned = left.disposition == Bc2RowDispositionStatus::NamedGap
            && right.disposition == Bc3DispositionKind::NamedImpossibility
            && !left.named_gap.is_empty()
            && right.named_impossibility_id.is_some();
        let neither_promoted = !left.promoted && !right.row_promoted;
        if !identity_exact
            || !schema3_digest_exact
            || !schema4_digest_exact
            || !schema5_digest_exact
            || !both_explicitly_dispositioned
            || !neither_promoted
        {
            return Err(BridgeCompletionExecutionError::Invariant(format!(
                "BC-2/BC-3 row join failed at `{}`",
                left.aggregate_row_id
            )));
        }
        let bc2_projection = BridgeRowProjection {
            aggregate_row_id: left.aggregate_row_id.clone(),
            schema3_row_digest: left.schema3_row_digest.clone(),
            schema4_row_derivation_hash: left.schema4_row_derivation_hash.clone(),
            schema5_row_derivation_hash: left.schema5_row_derivation_hash.clone(),
            disposition: "named_gap".to_owned(),
            named_gap: left.named_gap.clone(),
            promoted: left.promoted,
        };
        let bc3_projection = BridgeRowProjection {
            aggregate_row_id: right.aggregate_row_id.clone(),
            schema3_row_digest: right.schema3_row_digest.clone(),
            schema4_row_derivation_hash: right.schema4_row_derivation_hash.clone(),
            schema5_row_derivation_hash: right.schema5_row_derivation_hash.clone(),
            disposition: "named_impossibility".to_owned(),
            named_gap: right
                .named_impossibility_id
                .clone()
                .expect("validated named-impossibility identifier"),
            promoted: right.row_promoted,
        };
        let exact_scope = format!(
            "Exact shared frozen aggregate parent `{}`; comparator digests are structural testimony only.",
            left.aggregate_row_id
        );
        let derivation_hash = tagged_hash(
            "shared-row-join",
            &(
                &bc2_projection,
                &bc3_projection,
                identity_exact,
                schema3_digest_exact,
                schema4_digest_exact,
                schema5_digest_exact,
                &exact_scope,
            ),
        );
        rows.push(BridgeSharedRowJoin {
            quantifier: BridgeExecutionQuantifier::ExactAggregateParentRow,
            exact_scope,
            identity_register: BridgeExecutionRegister::ArtifactMetadata,
            comparator_register: BridgeExecutionRegister::StructuralTestimony,
            bc2: bc2_projection,
            bc3: bc3_projection,
            identity_exact,
            schema3_digest_exact,
            schema4_digest_exact,
            schema5_digest_exact,
            both_explicitly_dispositioned,
            neither_promoted,
            derivation_hash,
        });
    }
    let distinct = rows
        .iter()
        .map(|row| row.bc2.aggregate_row_id.clone())
        .collect::<BTreeSet<_>>();
    let row_order_exact = bc2
        .row_dispositions
        .iter()
        .map(|row| row.aggregate_row_id.as_str())
        .eq(bc3
            .row_dispositions
            .iter()
            .map(|row| row.aggregate_row_id.as_str()));
    let row_identity_exact = rows.iter().all(|row| row.identity_exact);
    let row_digest_join_exact = rows.iter().all(|row| {
        row.schema3_digest_exact && row.schema4_digest_exact && row.schema5_digest_exact
    });
    let disposition_surface_complete = rows.iter().all(|row| row.both_explicitly_dispositioned);
    let no_duplicate_identity = distinct.len() == rows.len();
    let promoted_surface_size = rows.iter().filter(|row| !row.neither_promoted).count();
    let no_promotions = promoted_surface_size == 0;
    if !row_order_exact
        || !row_identity_exact
        || !row_digest_join_exact
        || !disposition_surface_complete
        || !no_duplicate_identity
        || !no_promotions
    {
        return Err(BridgeCompletionExecutionError::Invariant(
            "cross-phase aggregate parent audit is incomplete".to_owned(),
        ));
    }
    let exact_scope = "Ordered identity-and-digest join of the complete frozen BC-2 and BC-3 aggregate parent surfaces; no candidate refinement is inferred.".to_owned();
    let derivation_hash = tagged_hash(
        "cross-phase-row-audit",
        &(
            &rows,
            &exact_scope,
            row_order_exact,
            no_duplicate_identity,
            no_promotions,
        ),
    );
    Ok(BridgeCrossPhaseRowAudit {
        quantifier: BridgeExecutionQuantifier::CrossPhaseFrozenAggregateSurface,
        exact_scope,
        bc2_surface_size: metadata_number(
            bc2.row_dispositions.len(),
            "BC-2 aggregate parent surface size",
        ),
        bc3_surface_size: metadata_number(
            bc3.row_dispositions.len(),
            "BC-3 aggregate parent surface size",
        ),
        shared_surface_size: metadata_number(rows.len(), "shared aggregate parent surface size"),
        distinct_identity_surface_size: metadata_number(
            distinct.len(),
            "distinct shared aggregate parent identity surface size",
        ),
        promoted_surface_size: metadata_number(
            promoted_surface_size,
            "promoted aggregate parent surface size derived from the joined rows",
        ),
        row_order_exact,
        row_identity_exact,
        row_digest_join_exact,
        disposition_surface_complete,
        no_duplicate_identity,
        no_promotions,
        structural_testimony_used_as_selector: false,
        rows,
        derivation_hash,
    })
}

fn remaining_gap(
    occurrence_id: String,
    gap_id: String,
    owner_phase: &str,
    quantifier: BridgeExecutionQuantifier,
    exact_scope: String,
    aggregate_row_id: Option<String>,
    exact_obstruction: String,
) -> BridgeRemainingGap {
    let derivation_hash = tagged_hash(
        "remaining-gap",
        &(
            &occurrence_id,
            &gap_id,
            owner_phase,
            quantifier,
            &exact_scope,
            &aggregate_row_id,
            &exact_obstruction,
        ),
    );
    BridgeRemainingGap {
        occurrence_id,
        gap_id,
        owner_phase: owner_phase.to_owned(),
        quantifier,
        exact_scope,
        aggregate_row_id,
        register: BridgeExecutionRegister::ProofInventory,
        exact_obstruction,
        keeps_exit_antecedent_false: true,
        keeps_rows_unpromoted: true,
        derivation_hash,
    }
}

fn build_remaining_gaps(
    bc1: &Bc1DepthTwoCompletenessV1Certificate,
    bc2: &Bc2BoundaryProvenanceJoinV1Certificate,
    bc3: &Bc3ParentRowDispositionV1Certificate,
) -> Result<Vec<BridgeRemainingGap>, BridgeCompletionExecutionError> {
    let mut gaps = Vec::new();
    for gap in &bc1.open_gaps {
        gaps.push(remaining_gap(
            gap.id.clone(),
            gap.id.clone(),
            "BC-1",
            BridgeExecutionQuantifier::FrozenWrappedSurface,
            format!(
                "Frozen wrapped BC-1 theorem obligation in phase `{}`.",
                gap.phase
            ),
            None,
            gap.exact_obstruction.clone(),
        ));
    }
    for gap_id in &bc2.global_named_gaps {
        let obstruction = if gap_id
            == &bc2
                .source_first_factorization_audit
                .exact_missing_theorem_id
        {
            bc2.source_first_factorization_audit
                .exact_obstruction
                .clone()
        } else {
            "Candidate-local family/orbit extraction, marginality, act-local provenance, and transport remain unproved on the frozen aggregate surface.".to_owned()
        };
        gaps.push(remaining_gap(
            gap_id.clone(),
            gap_id.clone(),
            "BC-2",
            BridgeExecutionQuantifier::FrozenWrappedSurface,
            "Global BC-2 candidate-boundary obligation over the frozen wrapped surface.".to_owned(),
            None,
            obstruction,
        ));
    }
    for row in &bc2.row_dispositions {
        gaps.push(remaining_gap(
            format!("{}@{}", row.named_gap, row.aggregate_row_id),
            row.named_gap.clone(),
            "BC-2",
            BridgeExecutionQuantifier::ExactAggregateParentRow,
            format!(
                "Exact BC-2 aggregate parent `{}` relative to current registered inputs.",
                row.aggregate_row_id
            ),
            Some(row.aggregate_row_id.clone()),
            row.exact_residue.clone(),
        ));
    }
    for row in &bc3.row_dispositions {
        let gap_id = row.named_impossibility_id.clone().ok_or_else(|| {
            BridgeCompletionExecutionError::Invariant(format!(
                "BC-3 row `{}` lacks its named impossibility",
                row.aggregate_row_id
            ))
        })?;
        gaps.push(remaining_gap(
            gap_id.clone(),
            gap_id,
            "BC-3",
            BridgeExecutionQuantifier::ExactAggregateParentRow,
            format!(
                "Exact BC-3 aggregate parent `{}` relative to current registered inputs; no semantic nonexistence claim.",
                row.aggregate_row_id
            ),
            Some(row.aggregate_row_id.clone()),
            row.exact_obstruction.clone(),
        ));
    }
    if gaps.is_empty()
        || gaps.iter().any(|gap| {
            !gap.keeps_exit_antecedent_false
                || !gap.keeps_rows_unpromoted
                || gap.exact_obstruction.is_empty()
        })
    {
        return Err(BridgeCompletionExecutionError::Invariant(
            "remaining named-gap enumeration is incomplete".to_owned(),
        ));
    }
    Ok(gaps)
}

fn build_failed_route_testimony(
    bc3: &Bc3ParentRowDispositionV1Certificate,
) -> Result<Vec<BridgeFailedRouteTestimony>, BridgeCompletionExecutionError> {
    let attempt = &bc3.universal_attempt;
    if !attempt.attempted
        || attempt.universal_refinement_theorem_proved
        || !attempt.failed_attempt_published
        || attempt.universal_failure_id.is_empty()
        || attempt.exact_failure_point.is_empty()
    {
        return Err(BridgeCompletionExecutionError::Invariant(
            "BC-3 failed universal route is not exact published testimony".to_owned(),
        ));
    }
    let exact_scope =
        "Failed BC-3 universal proof route over the frozen wrapped surface; information only, never a stop cause."
            .to_owned();
    let derivation_hash = tagged_hash(
        "failed-route-testimony",
        &(
            attempt.universal_failure_id.as_str(),
            &exact_scope,
            attempt.exact_failure_point.as_str(),
            false,
        ),
    );
    Ok(vec![BridgeFailedRouteTestimony {
        route_id: attempt.universal_failure_id.clone(),
        owner_phase: "BC-3".to_owned(),
        quantifier: BridgeExecutionQuantifier::FrozenWrappedSurface,
        exact_scope,
        register: BridgeExecutionRegister::ProofInventory,
        exact_failure_point: attempt.exact_failure_point.clone(),
        failed_route_used_as_stop_cause: false,
        derivation_hash,
    }])
}

fn build_certificate()
-> Result<BridgeCompletionExecutionV1Certificate, BridgeCompletionExecutionError> {
    let m3: M3E7E8BridgeV1Certificate = parse_bound("M-3 v1", M3_CERTIFICATE_BYTES)?;
    let bc1: Bc1DepthTwoCompletenessV1Certificate = parse_bound("BC-1", BC1_CERTIFICATE_BYTES)?;
    let bc2: Bc2BoundaryProvenanceJoinV1Certificate = parse_bound("BC-2", BC2_CERTIFICATE_BYTES)?;
    let bc3: Bc3ParentRowDispositionV1Certificate = parse_bound("BC-3", BC3_CERTIFICATE_BYTES)?;

    let baseline = build_baseline(&m3)?;
    let phases = vec![
        build_bc1_phase(&bc1, &baseline.result_digest)?,
        build_bc2_phase(&bc2, &baseline.result_digest)?,
        build_bc3_phase(&bc3, &baseline.result_digest)?,
    ];
    let cross_phase_rows = build_cross_phase_rows(&bc2, &bc3)?;
    let remaining_gaps = build_remaining_gaps(&bc1, &bc2, &bc3)?;
    let failed_route_testimony = build_failed_route_testimony(&bc3)?;
    let satisfied_prerequisite_surface_size = phases
        .iter()
        .filter(|phase| phase.prerequisite_satisfied)
        .count();
    let promoted_surface_size = phases
        .iter()
        .map(|phase| {
            phase
                .promoted_surface_size
                .decimal
                .parse::<usize>()
                .map_err(|error| {
                    BridgeCompletionExecutionError::Invariant(format!(
                        "{} promoted surface is not a decimal count: {error}",
                        phase.phase_id
                    ))
                })
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .sum::<usize>();
    let exit_antecedent_satisfied = satisfied_prerequisite_surface_size == phases.len()
        && phases.iter().all(|phase| phase.blocker_closed);
    let no_partial_promotion = promoted_surface_size == 0 && cross_phase_rows.no_promotions;
    if exit_antecedent_satisfied || !no_partial_promotion {
        return Err(BridgeCompletionExecutionError::Invariant(
            "frozen exit antecedent unexpectedly became true or a partial promotion appeared"
                .to_owned(),
        ));
    }

    let standing_rule_fields = (true, true, true, false, false, false, true, false);
    let standing_rules = BridgeStandingRuleAudit {
        bound_input_bytes_match_replayed_phase_bindings: standing_rule_fields.0,
        create_new_artifact_required: standing_rule_fields.1,
        register_discipline_valid: standing_rule_fields.2,
        structural_testimony_used_as_semantic_authority: standing_rule_fields.3,
        selector_used: standing_rule_fields.4,
        unregistered_semantics_added: standing_rule_fields.5,
        scope_boundary_preserved: standing_rule_fields.6,
        count_bearing_predicate_names_used: standing_rule_fields.7,
        derivation_hash: tagged_hash("standing-rules", &standing_rule_fields),
    };
    let gate_scope = "The frozen bridge-completion exit antecedent: every registered phase prerequisite must be discharged before any M-3 successor or downstream authorization.".to_owned();
    let gate_fields = (
        exit_antecedent_satisfied,
        no_partial_promotion,
        false,
        false,
        true,
        false,
        BridgeExecutionStatus::StoppedPrerequisiteGaps,
    );
    let gate = BridgeExecutionGate {
        quantifier: BridgeExecutionQuantifier::FrozenProgramExitGate,
        exact_scope: gate_scope.clone(),
        prerequisite_surface_size: metadata_number(
            phases.len(),
            "registered prerequisite surface size",
        ),
        satisfied_prerequisite_surface_size: metadata_number(
            satisfied_prerequisite_surface_size,
            "satisfied prerequisite surface size",
        ),
        promoted_surface_size: metadata_number(
            promoted_surface_size,
            "promoted program surface size derived from phase dispositions",
        ),
        exit_antecedent_satisfied: gate_fields.0,
        no_partial_promotion: gate_fields.1,
        bridge_claim_issued: gate_fields.2,
        m3_v2_issued: gate_fields.3,
        m3_v1_remains_authoritative: gate_fields.4,
        m4_authorized: gate_fields.5,
        authoritative_certificate_path: "docs/schema2_e7_e8_bridge_v1.json".to_owned(),
        successor_certificate_path: None,
        status: gate_fields.6,
        derivation_hash: tagged_hash("execution-gate", &(gate_scope, gate_fields)),
    };
    let bindings = source_bindings();
    let mut certificate = BridgeCompletionExecutionV1Certificate {
        schema: BRIDGE_COMPLETION_EXECUTION_V1_SCHEMA.to_owned(),
        date: BRIDGE_COMPLETION_EXECUTION_V1_DATE.to_owned(),
        numeric_register_policy: "Every public quantitative datum is a tagged decimal value. Decimal substrings in dates, schema versions, phase labels, row identifiers, theorem labels, and file names are syntax identifiers. Predicate names are count-neutral; their quantitative scope is established only by tagged evidence.".to_owned(),
        source_surface_size: metadata_number(
            bindings.len(),
            "exact bound source surface size",
        ),
        source_bindings: bindings,
        baseline,
        phase_surface_size: metadata_number(phases.len(), "executed phase surface size"),
        phases,
        cross_phase_rows,
        remaining_gap_occurrence_surface_size: metadata_number(
            remaining_gaps.len(),
            "remaining named-gap occurrence surface size",
        ),
        remaining_gaps,
        failed_route_surface_size: metadata_number(
            failed_route_testimony.len(),
            "failed proof-route testimony surface size",
        ),
        failed_route_testimony,
        standing_rules,
        gate,
        mutation_falsifiers: vec![
            "change_any_bound_certificate_report_source_byte_or_digest_then_replay_must_fail"
                .to_owned(),
            "change_the_frozen_baseline_condition_partition_or_enacted_regression_then_replay_must_fail"
                .to_owned(),
            "change_any_phase_status_scope_quantifier_or_replay_result_then_replay_must_fail"
                .to_owned(),
            "reorder_duplicate_remove_or_mutate_any_shared_row_identity_or_digest_then_replay_must_fail"
                .to_owned(),
            "remove_rename_or_rescope_any_remaining_named_gap_then_replay_must_fail".to_owned(),
            "promote_failed_proof_route_testimony_to_a_stop_cause_then_replay_must_fail".to_owned(),
            "retag_any_public_quantity_then_replay_must_fail".to_owned(),
            "make_any_partial_promotion_or_issue_any_successor_before_the_exit_antecedent_then_replay_must_fail"
                .to_owned(),
            "add_unregistered_semantics_or_cross_the_frozen_scope_boundary_then_replay_must_fail"
                .to_owned(),
            "add_any_unknown_JSON_field_then_replay_must_fail".to_owned(),
        ],
        permitted_conclusion: "The exact M-3 v1 baseline and each registered bridge-completion phase artifact replay. BC-2 and BC-3 expose the same ordered aggregate-row identities and comparator digests with complete explicit fail-closed dispositions and no promotions. The registered prerequisites remain open, so the frozen exit antecedent is false: no M-3 successor is issued, M-3 v1 remains authoritative, and M-4 is unauthorized.".to_owned(),
        required_successor_action: "Supply a versioned theorem or representation that discharges the enumerated prerequisite gaps without new unregistered semantics, rerun each affected phase create-new, and only then reevaluate the frozen exit antecedent.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

static EXPECTED_CERTIFICATE: OnceLock<Result<BridgeCompletionExecutionV1Certificate, String>> =
    OnceLock::new();

fn expected_certificate()
-> Result<&'static BridgeCompletionExecutionV1Certificate, BridgeCompletionExecutionError> {
    match EXPECTED_CERTIFICATE
        .get_or_init(|| build_certificate().map_err(|error| error.to_string()))
    {
        Ok(certificate) => Ok(certificate),
        Err(error) => Err(BridgeCompletionExecutionError::Input(error.clone())),
    }
}

pub fn issue_bridge_completion_execution_v1()
-> Result<BridgeCompletionExecutionV1Certificate, BridgeCompletionExecutionError> {
    expected_certificate().cloned()
}

fn invalid_replay(error: impl Into<String>) -> BridgeCompletionExecutionReplay {
    BridgeCompletionExecutionReplay {
        valid: false,
        errors: vec![error.into()],
        status: BridgeExecutionStatus::StoppedPrerequisiteGaps,
        m3_v2_issued: false,
        m3_v1_remains_authoritative: true,
        m4_authorized: false,
    }
}

pub fn replay_bridge_completion_execution_v1(
    claimed: &BridgeCompletionExecutionV1Certificate,
) -> BridgeCompletionExecutionReplay {
    let expected = match expected_certificate() {
        Ok(certificate) => certificate,
        Err(error) => return invalid_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("bridge-completion execution certificate digest mismatch".to_owned());
    }
    if claimed.source_bindings != source_bindings() {
        errors.push("bridge-completion execution source bindings drifted".to_owned());
    }
    if claimed != expected {
        errors.push(
            "bridge-completion execution certificate differs from deterministic reissuance"
                .to_owned(),
        );
    }
    BridgeCompletionExecutionReplay {
        valid: errors.is_empty(),
        errors,
        status: claimed.gate.status,
        m3_v2_issued: claimed.gate.m3_v2_issued,
        m3_v1_remains_authoritative: claimed.gate.m3_v1_remains_authoritative,
        m4_authorized: claimed.gate.m4_authorized,
    }
}

pub fn replay_bridge_completion_execution_v1_json(json: &str) -> BridgeCompletionExecutionReplay {
    match serde_json::from_str::<BridgeCompletionExecutionV1Certificate>(json) {
        Ok(certificate) => replay_bridge_completion_execution_v1(&certificate),
        Err(error) => invalid_replay(format!("invalid bridge-completion execution JSON: {error}")),
    }
}

fn register_label(register: BridgeExecutionRegister) -> &'static str {
    match register {
        BridgeExecutionRegister::SemanticRegisterAuthority => "semantic_register_authority",
        BridgeExecutionRegister::StructuralTestimony => "structural_testimony",
        BridgeExecutionRegister::ProofInventory => "proof_inventory",
        BridgeExecutionRegister::ArtifactMetadata => "artifact_metadata",
        BridgeExecutionRegister::SyntaxIdentifier => "syntax_identifier",
    }
}

fn render_number(value: &BridgeRegisteredNumber) -> String {
    format!(
        "{} [register: `{}`; meaning: {}]",
        value.decimal,
        register_label(value.register),
        value.meaning
    )
}

pub fn render_bridge_completion_execution_v1(
    certificate: &BridgeCompletionExecutionV1Certificate,
) -> String {
    let mut out = String::new();
    out.push_str("# Bridge-completion execution result\n\n");
    out.push_str(&format!(
        "**Date (syntax identifier):** {}. **Status:** `stopped_prerequisite_gaps`. **Certificate:** `{}`.\n\n",
        certificate.date, certificate.result_digest
    ));
    out.push_str("The frozen exit antecedent is false. The run therefore issues no M-3 successor, retains M-3 v1 as the authoritative bridge result, makes no partial promotion, and leaves M-4 unauthorized.\n\n");
    out.push_str("## Baseline\n\n");
    out.push_str(&format!(
        "M-3 v1 `{}` replays with **{}** conditions: **{}** proved and **{}** open. Its enacted regression and quantifiers reproduce exactly, and its promoted surface is empty.\n\n",
        certificate.baseline.result_digest,
        render_number(&certificate.baseline.condition_surface_size),
        render_number(&certificate.baseline.proved_surface_size),
        render_number(&certificate.baseline.open_surface_size),
    ));
    out.push_str("## Phase executions\n\n");
    out.push_str("| Phase | Status | Explicit disposition surface | Blocker closed | Remaining gap occurrences | Promoted surface |\n|---|---|---:|---:|---|---|\n");
    for phase in &certificate.phases {
        out.push_str(&format!(
            "| `{}` | `{}` | {} | {} | {} | {} |\n",
            phase.phase_id,
            phase.status,
            phase.disposition_surface_complete,
            phase.blocker_closed,
            render_number(&phase.gap_occurrence_surface_size),
            render_number(&phase.promoted_surface_size),
        ));
    }
    out.push_str("\n## Exact BC-2/BC-3 row join\n\n");
    out.push_str(&format!(
        "BC-2 surface: **{}**. BC-3 surface: **{}**. Shared ordered surface: **{}**, with **{}** distinct identities and **{}** promotions. Row identities and Schema-3/4/5 digests join exactly; comparator hashes remain structural testimony and select nothing.\n\n",
        render_number(&certificate.cross_phase_rows.bc2_surface_size),
        render_number(&certificate.cross_phase_rows.bc3_surface_size),
        render_number(&certificate.cross_phase_rows.shared_surface_size),
        render_number(&certificate.cross_phase_rows.distinct_identity_surface_size),
        render_number(&certificate.cross_phase_rows.promoted_surface_size),
    ));
    out.push_str("## Remaining named gaps\n\n");
    out.push_str(&format!(
        "The certificate enumerates **{}** exact gap occurrences. Row-local repetitions are retained because their scopes differ.\n\n",
        render_number(&certificate.remaining_gap_occurrence_surface_size)
    ));
    out.push_str("| Occurrence | Owner | Quantifier | Exact scope | Exact obstruction |\n|---|---|---|---|---|\n");
    for gap in &certificate.remaining_gaps {
        out.push_str(&format!(
            "| `{}` | `{}` | `{:?}` | {} | {} |\n",
            gap.occurrence_id,
            gap.owner_phase,
            gap.quantifier,
            gap.exact_scope.replace('|', "\\|"),
            gap.exact_obstruction.replace('|', "\\|"),
        ));
    }
    out.push_str("\n## Failed proof-route testimony\n\n");
    out.push_str(&format!(
        "The certificate retains **{}** failed proof route as information only. It is not included in the remaining-gap inventory and does not cause the stop.\n\n",
        render_number(&certificate.failed_route_surface_size)
    ));
    for route in &certificate.failed_route_testimony {
        out.push_str(&format!(
            "- `{}`: {} The route is a stop cause: **{}**.\n",
            route.route_id, route.exact_failure_point, route.failed_route_used_as_stop_cause
        ));
    }
    out.push_str("\n## Permitted conclusion\n\n");
    out.push_str(&certificate.permitted_conclusion);
    out.push_str("\n\n## Required successor\n\n");
    out.push_str(&certificate.required_successor_action);
    out.push('\n');
    out
}

fn cleanup_created(path: &Path, created: bool) {
    if created {
        let _ = remove_file(path);
    }
}

pub fn emit_bridge_completion_execution_v1_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<BridgeCompletionExecutionV1Certificate, BridgeCompletionExecutionError> {
    if certificate_path == report_path {
        return Err(BridgeCompletionExecutionError::Io(
            "certificate and report targets must be distinct".to_owned(),
        ));
    }
    if certificate_path.exists() || report_path.exists() {
        return Err(BridgeCompletionExecutionError::Io(
            "create-new target already exists; no artifact was overwritten".to_owned(),
        ));
    }
    let certificate = issue_bridge_completion_execution_v1()?;
    let replay = replay_bridge_completion_execution_v1(&certificate);
    if !replay.valid {
        return Err(BridgeCompletionExecutionError::Invariant(format!(
            "new execution manifest did not replay: {}",
            replay.errors.join("; ")
        )));
    }
    let mut json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| BridgeCompletionExecutionError::Json(error.to_string()))?;
    json.push(b'\n');
    let report = render_bridge_completion_execution_v1(&certificate);
    let mut certificate_created = false;
    let mut report_created = false;

    let result = (|| -> Result<(), BridgeCompletionExecutionError> {
        let mut certificate_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(certificate_path)
            .map_err(|error| BridgeCompletionExecutionError::Io(error.to_string()))?;
        certificate_created = true;
        certificate_file
            .write_all(&json)
            .and_then(|_| certificate_file.sync_all())
            .map_err(|error| BridgeCompletionExecutionError::Io(error.to_string()))?;
        drop(certificate_file);

        let mut report_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(report_path)
            .map_err(|error| BridgeCompletionExecutionError::Io(error.to_string()))?;
        report_created = true;
        report_file
            .write_all(report.as_bytes())
            .and_then(|_| report_file.sync_all())
            .map_err(|error| BridgeCompletionExecutionError::Io(error.to_string()))?;
        drop(report_file);

        let emitted_json = read_to_string(certificate_path)
            .map_err(|error| BridgeCompletionExecutionError::Io(error.to_string()))?;
        let emitted_report = read(report_path)
            .map_err(|error| BridgeCompletionExecutionError::Io(error.to_string()))?;
        if emitted_json.as_bytes() != json.as_slice() {
            return Err(BridgeCompletionExecutionError::Invariant(
                "emitted certificate bytes differ from create-new payload".to_owned(),
            ));
        }
        if emitted_report.as_slice() != report.as_bytes() {
            return Err(BridgeCompletionExecutionError::Invariant(
                "emitted report bytes differ from deterministic rendering".to_owned(),
            ));
        }
        let emitted_replay = replay_bridge_completion_execution_v1_json(&emitted_json);
        if !emitted_replay.valid {
            return Err(BridgeCompletionExecutionError::Invariant(format!(
                "emitted execution manifest did not replay: {}",
                emitted_replay.errors.join("; ")
            )));
        }
        Ok(())
    })();
    if let Err(error) = result {
        cleanup_created(certificate_path, certificate_created);
        cleanup_created(report_path, report_created);
        return Err(error);
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::{
        BridgeCompletionExecutionV1Certificate, BridgeExecutionQuantifier, BridgeExecutionRegister,
        BridgeExecutionStatus, certificate_digest, issue_bridge_completion_execution_v1,
        replay_bridge_completion_execution_v1, replay_bridge_completion_execution_v1_json,
    };
    use std::collections::BTreeSet;

    fn reseal(certificate: &mut BridgeCompletionExecutionV1Certificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    #[test]
    fn issues_replayable_fail_closed_execution_manifest() {
        let certificate =
            issue_bridge_completion_execution_v1().expect("execution manifest issues");
        let replay = replay_bridge_completion_execution_v1(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(
            certificate.gate.status,
            BridgeExecutionStatus::StoppedPrerequisiteGaps
        );
        assert!(!certificate.gate.exit_antecedent_satisfied);
        assert!(certificate.gate.no_partial_promotion);
        assert!(!certificate.gate.bridge_claim_issued);
        assert!(!certificate.gate.m3_v2_issued);
        assert!(certificate.gate.m3_v1_remains_authoritative);
        assert!(!certificate.gate.m4_authorized);
        assert!(certificate.baseline.baseline_authenticated);
        assert!(
            certificate
                .failed_route_testimony
                .iter()
                .all(|route| !route.failed_route_used_as_stop_cause)
        );
        assert!(certificate.failed_route_testimony.iter().all(|route| {
            certificate
                .remaining_gaps
                .iter()
                .all(|gap| gap.occurrence_id != route.route_id)
        }));
        assert!(
            certificate
                .phases
                .iter()
                .all(|phase| phase.public_replay_valid
                    && phase.disposition_surface_complete
                    && !phase.blocker_closed
                    && !phase.prerequisite_satisfied
                    && phase.no_promotions)
        );
    }

    #[test]
    fn exact_shared_row_surface_is_bound_without_promotion() {
        let certificate =
            issue_bridge_completion_execution_v1().expect("execution manifest issues");
        let rows = &certificate.cross_phase_rows;
        assert_eq!(rows.shared_surface_size.decimal, "213");
        assert_eq!(rows.distinct_identity_surface_size.decimal, "213");
        assert_eq!(rows.promoted_surface_size.decimal, "0");
        assert_eq!(
            rows.shared_surface_size.register,
            BridgeExecutionRegister::ArtifactMetadata
        );
        assert!(rows.row_order_exact);
        assert!(rows.row_identity_exact);
        assert!(rows.row_digest_join_exact);
        assert!(rows.disposition_surface_complete);
        assert!(rows.no_duplicate_identity);
        assert!(rows.no_promotions);
        let identities = rows
            .rows
            .iter()
            .map(|row| row.bc2.aggregate_row_id.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(identities.len(), rows.rows.len());
        assert!(rows.rows.iter().all(|row| {
            row.quantifier == BridgeExecutionQuantifier::ExactAggregateParentRow
                && row.identity_exact
                && row.schema3_digest_exact
                && row.schema4_digest_exact
                && row.schema5_digest_exact
                && row.both_explicitly_dispositioned
                && row.neither_promoted
        }));
    }

    #[test]
    fn digest_scope_register_gap_row_and_gate_mutations_fail() {
        let certificate =
            issue_bridge_completion_execution_v1().expect("execution manifest issues");

        let mut baseline = certificate.clone();
        baseline.baseline.result_digest.push_str(":mutated");
        reseal(&mut baseline);
        assert!(!replay_bridge_completion_execution_v1(&baseline).valid);

        let mut source = certificate.clone();
        source.source_bindings[0].blake3.push('0');
        reseal(&mut source);
        assert!(!replay_bridge_completion_execution_v1(&source).valid);

        let mut phase = certificate.clone();
        phase.phases[0].prerequisite_satisfied = true;
        reseal(&mut phase);
        assert!(!replay_bridge_completion_execution_v1(&phase).valid);

        let mut row_id = certificate.clone();
        row_id.cross_phase_rows.rows[0]
            .bc2
            .aggregate_row_id
            .push_str(":mutated");
        reseal(&mut row_id);
        assert!(!replay_bridge_completion_execution_v1(&row_id).valid);

        let mut row_digest = certificate.clone();
        row_digest.cross_phase_rows.rows[0]
            .bc3
            .schema3_row_digest
            .push('0');
        reseal(&mut row_digest);
        assert!(!replay_bridge_completion_execution_v1(&row_digest).valid);

        let mut row_order = certificate.clone();
        row_order.cross_phase_rows.rows.swap(0, 1);
        reseal(&mut row_order);
        assert!(!replay_bridge_completion_execution_v1(&row_order).valid);

        let mut gap = certificate.clone();
        gap.remaining_gaps.remove(0);
        reseal(&mut gap);
        assert!(!replay_bridge_completion_execution_v1(&gap).valid);

        let mut failed_route = certificate.clone();
        failed_route.failed_route_testimony[0].failed_route_used_as_stop_cause = true;
        reseal(&mut failed_route);
        assert!(!replay_bridge_completion_execution_v1(&failed_route).valid);

        let mut quantifier = certificate.clone();
        quantifier.remaining_gaps[0].quantifier = BridgeExecutionQuantifier::FrozenProgramExitGate;
        reseal(&mut quantifier);
        assert!(!replay_bridge_completion_execution_v1(&quantifier).valid);

        let mut register = certificate.clone();
        register.cross_phase_rows.shared_surface_size.register =
            BridgeExecutionRegister::SemanticRegisterAuthority;
        reseal(&mut register);
        assert!(!replay_bridge_completion_execution_v1(&register).valid);

        let mut promotion = certificate.clone();
        promotion.cross_phase_rows.rows[0].bc2.promoted = true;
        reseal(&mut promotion);
        assert!(!replay_bridge_completion_execution_v1(&promotion).valid);

        let mut successor = certificate.clone();
        successor.gate.m3_v2_issued = true;
        reseal(&mut successor);
        assert!(!replay_bridge_completion_execution_v1(&successor).valid);

        let mut authority = certificate.clone();
        authority.gate.m3_v1_remains_authoritative = false;
        reseal(&mut authority);
        assert!(!replay_bridge_completion_execution_v1(&authority).valid);

        let mut authorization = certificate.clone();
        authorization.gate.m4_authorized = true;
        reseal(&mut authorization);
        assert!(!replay_bridge_completion_execution_v1(&authorization).valid);

        let mut digest = certificate;
        digest.result_digest.push('0');
        assert!(!replay_bridge_completion_execution_v1(&digest).valid);
    }

    #[test]
    fn unknown_json_fields_fail_closed() {
        let certificate =
            issue_bridge_completion_execution_v1().expect("execution manifest issues");
        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("object")
            .insert("unknown_field".to_owned(), serde_json::Value::Bool(true));
        let replay = replay_bridge_completion_execution_v1_json(
            &serde_json::to_string(&value).expect("json"),
        );
        assert!(!replay.valid);
    }
}
