//! BC-2 candidate-boundary provenance join.
//!
//! The frozen Schema-3 surface is a dynamic-programming quotient.  Its rows
//! retain coarse classifier/proxy/support/route coordinates and a
//! multiplicity, but no candidate telescope.  This checker audits both the
//! archived surface and the source-first reconstruction route.  The latter
//! also stops: the IP-1 DP key omits the normalized-family, quotient, anchor,
//! A3-membership, predecessor-surface, and cubical-decision coordinates used
//! by the adopted act-local issuer, and no theorem proves that provenance
//! factors through that key.
//!
//! Consequently every aggregate row is explicitly dispositioned as a named
//! gap.  The existing Schema-4/5 joins are retained only as regression
//! comparators.  No provenance token, row promotion, bridge claim, or M-4
//! authorization is issued.

use crate::act_local_semantic_provenance_v5::{
    V5PrefixLocalIssuanceCapability, issue_v5_prefix_local_rule_authority,
    replay_v5_prefix_local_rule_authority,
};
use crate::candidate_join::CandidateJoinV4Certificate;
use crate::candidate_join_v5::CandidateJoinV5Certificate;
use crate::m3_e7_e8_bridge_v1::{
    M3ClaimStatus, M3E7E8BridgeV1Certificate, M3Register, M3RunStatus,
};
use crate::semantic_nu_transport_maps_v2::{
    NumericRegister, SEMANTIC_NU_TRANSPORT_MAPS_V2_SCHEMA, SEMANTIC_NU_TRANSPORT_SCOPE_BOUNDARY,
    SemanticNuTransportMapsV2Certificate,
};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{OpenOptions, read_to_string, remove_file};
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const BC2_BOUNDARY_PROVENANCE_JOIN_V1_SCHEMA: &str =
    "bc2-candidate-boundary-provenance-join-v1";
pub const BC2_BOUNDARY_PROVENANCE_JOIN_V1_DATE: &str = "2026-07-23";
pub const BC2_BOUNDARY_PROVENANCE_JOIN_V1_CERTIFICATE_NAME: &str =
    "bc2_boundary_provenance_join_v1.json";
pub const BC2_BOUNDARY_PROVENANCE_JOIN_V1_REPORT_NAME: &str =
    "BC2_BOUNDARY_PROVENANCE_JOIN_RESULT.md";
pub const BC2_GLOBAL_GAP: &str = "C8_CANDIDATE_BOUNDARY_PROVENANCE_JOIN";
pub const BC2_FACTORIZATION_GAP: &str = "BC2_IP1_DP_KEY_PROVENANCE_CONGRUENCE_NOT_PROVED";
pub const BC2_ROW_GAP: &str = "BC2_SCHEMA3_AGGREGATE_KEY_NOT_PROVED_PROVENANCE_CONGRUENT";

const EXPECTED_M3_V1_RESULT_DIGEST: &str =
    "blake3:a61fe8093ef9ce61a05aebfe859b0723a88a766a712ea6a2312e0f22ca386ba8";
const EXPECTED_M1_RESULT_DIGEST: &str =
    "blake3:b9d84d8927e4d1e369e6cf9904664e5f0a9ecc1cdf9e3e92644133cae14e804c";
const EXPECTED_M3_E5_DRIFT: &str = "prerequisite failed: stable A3 exhaustiveness certificate did not replay: certificate differs from definition replay";

const BRIDGE_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/bridge_completion_plan.md");
const MAINLINE_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/mainline_completion_plan.md");
const ACT_LOCAL_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/bi0_prerequisites_adjudication.md");
const NU_REGISTER_BYTES: &[u8] = include_bytes!("../../../docs/nu_register_adjudication.md");
const M1_CERTIFICATE_BYTES: &[u8] =
    include_bytes!("../../../docs/semantic_nu_transport_maps_v2.json");
const M3_CERTIFICATE_BYTES: &[u8] = include_bytes!("../../../docs/schema2_e7_e8_bridge_v1.json");
const SCHEMA3_BYTES: &[u8] = include_bytes!("../../../docs/ip1_candidate_verdict_join_v3.json");
const SCHEMA4_BYTES: &[u8] = include_bytes!("../../../docs/ip1_candidate_verdict_join_v4.json");
const SCHEMA5_BYTES: &[u8] =
    include_bytes!("../../../docs/ip1_candidate_verdict_join_v5_element_overlay.json");
const IP1_SOURCE_BYTES: &[u8] = include_bytes!("ip1_candidate_join.rs");
const ACT_LOCAL_SOURCE_BYTES: &[u8] = include_bytes!("act_local_semantic_provenance_v5.rs");
const M1_SOURCE_BYTES: &[u8] = include_bytes!("semantic_nu_transport_maps_v2.rs");
const M3_SOURCE_BYTES: &[u8] = include_bytes!("m3_e7_e8_bridge_v1.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("bc2_boundary_provenance_join_v1.rs");

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc2Register {
    SemanticRegisterAuthority,
    SemanticFamilyAuthority,
    StructuralTestimony,
    ProofInventory,
    ArtifactMetadata,
    SyntaxIdentifier,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc2Quantifier {
    FrozenWrappedSurface,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc2RowDispositionStatus {
    NamedGap,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc2RunStatus {
    StoppedNamedGaps,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2RegisteredValue {
    pub value: String,
    pub register: Bc2Register,
    pub meaning: String,
}

fn registered(value: impl ToString, register: Bc2Register, meaning: &str) -> Bc2RegisteredValue {
    Bc2RegisteredValue {
        value: value.to_string(),
        register,
        meaning: meaning.to_owned(),
    }
}

fn metadata(value: impl ToString, meaning: &str) -> Bc2RegisteredValue {
    registered(value, Bc2Register::ArtifactMetadata, meaning)
}

fn syntax(value: impl ToString, meaning: &str) -> Bc2RegisteredValue {
    registered(value, Bc2Register::SyntaxIdentifier, meaning)
}

fn structural(value: impl ToString, meaning: &str) -> Bc2RegisteredValue {
    registered(value, Bc2Register::StructuralTestimony, meaning)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: Bc2RegisteredValue,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2M3BaselineAuthentication {
    pub expected_result_digest: String,
    pub observed_result_digest: String,
    pub result_digest_exact: bool,
    pub self_digest_valid: bool,
    pub source_binding_bytes_exact: bool,
    pub stopped_named_gaps: bool,
    pub predecessor_condition_count: Bc2RegisteredValue,
    pub proved_predecessor_condition_count: Bc2RegisteredValue,
    pub open_predecessor_condition_count: Bc2RegisteredValue,
    pub exact_predecessor_condition_partition_reproduced: bool,
    pub exact_predecessor_condition_ordinals: Vec<Bc2RegisteredValue>,
    pub enacted_regression_projections_exactly_equal: bool,
    pub enacted_regression_complete: bool,
    pub registered_e5_drift_disclosed_exactly: bool,
    pub registered_e5_drift_repaired_or_concealed: bool,
    pub exact_schema3_row_surface: bool,
    pub no_predecessor_promotions: bool,
    pub predecessor_m4_authorized: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2ActLocalAuthorityAudit {
    pub rule_authority_derivation_hash: String,
    pub rule_authority_replay_valid: bool,
    pub semantic_family_is_unit_of_credit: bool,
    pub uniform_instances_require_independent_export: bool,
    pub support_hypotheses_mint_no_credit: bool,
    pub no_silent_residue_required: bool,
    pub source_document_read: bool,
    pub structural_scalar_read: bool,
    pub desired_vector_read: bool,
    pub candidate_and_exact_prefix_capability_required: bool,
    pub predecessor_semantic_surface_capability_required: bool,
    pub cubical_decision_surface_capability_required: bool,
    pub archive_join_is_authority: bool,
    pub archive_join_is_regression_comparator_only: bool,
    pub same_act_same_prefix_deterministic_replay_rule_bound: bool,
    pub no_enacted_future_input_permitted: bool,
    pub no_value_bar_hash_or_enumeration_order_selector: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2TransportMapReference {
    pub relation_id: String,
    pub source_object_id: String,
    pub target_object_id: String,
    pub map_derivation_hash: String,
    pub typed_total_map: bool,
    pub typed_total_inverse: bool,
    pub identity_composition_coherent: bool,
    pub semantic_family_nu_invariant: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2OrderObstructionReference {
    pub fixed_coordinate: String,
    pub left_candidate_hash: String,
    pub right_candidate_hash: String,
    pub left_semantic_nu: Bc2RegisteredValue,
    pub right_semantic_nu: Bc2RegisteredValue,
    pub semantic_nu_distinct: bool,
    pub no_accepted_typed_family_bijection_exists: bool,
    pub kept_separate_by_bc2: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2M1TransportAudit {
    pub expected_result_digest: String,
    pub observed_result_digest: String,
    pub result_digest_exact: bool,
    pub self_digest_valid: bool,
    pub scope_boundary_id: String,
    pub scope_boundary_exact: bool,
    pub exported_map_count: Bc2RegisteredValue,
    pub maps: Vec<Bc2TransportMapReference>,
    pub all_maps_typed_total_inverse_coherent_and_nu_invariant: bool,
    pub order_obstruction_count: Bc2RegisteredValue,
    pub order_obstructions: Vec<Bc2OrderObstructionReference>,
    pub registered_order_obstruction_inventory_preserved: bool,
    pub cross_obstruction_merge_attempted: bool,
    pub no_new_equivalence_semantics_assumed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2SourceFirstFactorizationAudit {
    pub ip1_source_bound: bool,
    pub act_local_source_bound: bool,
    pub source_first_route_audited: bool,
    pub ip1_dynamic_program_reconstructs_aggregate_rows: bool,
    pub aggregate_rows_require_candidate_refinement: bool,
    pub ip1_dp_key_carries_candidate_telescope: bool,
    pub ip1_dp_key_carries_normalized_family_identity: bool,
    pub ip1_dp_key_carries_role_declaration_identity: bool,
    pub ip1_dp_key_carries_unified_equivalence_closure: bool,
    pub ip1_dp_key_carries_anchor_relation: bool,
    pub ip1_dp_key_carries_a3_orbit_membership: bool,
    pub ip1_dp_key_carries_predecessor_semantic_surface: bool,
    pub ip1_dp_key_carries_cubical_decision_surface: bool,
    pub act_local_token_binds_candidate_hash: bool,
    pub act_local_token_binds_exact_prefix_digest: bool,
    pub act_local_issuer_is_cross_clause_and_prefix_sensitive: bool,
    pub registered_factorization_theorem_marker_found: bool,
    pub provenance_factorization_through_ip1_dp_key_proved: bool,
    pub source_first_symbolic_join_proved: bool,
    pub exact_missing_theorem_id: String,
    pub exact_obstruction: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2RowDisposition {
    pub aggregate_row_id: String,
    pub stratum_index: Bc2RegisteredValue,
    pub row_index: Bc2RegisteredValue,
    pub kappa: Bc2RegisteredValue,
    pub class: String,
    pub clause_local_extraction_proxy: String,
    pub direct_support: Bc2RegisteredValue,
    pub amplification_route: String,
    pub parent_multiplicity: Bc2RegisteredValue,
    pub schema3_row_digest: String,
    pub schema4_row_derivation_hash: String,
    pub schema5_row_derivation_hash: String,
    pub archive_comparator_matched: bool,
    pub archive_comparator_is_authority: bool,
    pub candidate_payload_present: bool,
    pub telescope_payload_present: bool,
    pub exact_prefix_digest_present: bool,
    pub candidate_and_prefix_input_complete: bool,
    pub natural_families_extracted: bool,
    pub semantic_demand_orbits_extracted: bool,
    pub marginality_decided: bool,
    pub candidate_local_provenance_token_issued: bool,
    pub branch_transport_replay_proved: bool,
    pub m1_transport_instantiated_for_candidate_families: bool,
    pub order_obstruction_separation_preserved: bool,
    pub disposition: Bc2RowDispositionStatus,
    pub named_gap: String,
    pub exact_residue: String,
    pub promoted: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2CoverageAudit {
    pub quantifier: Bc2Quantifier,
    pub schema3_stratum_count: Bc2RegisteredValue,
    pub schema3_row_count: Bc2RegisteredValue,
    pub explicit_row_disposition_count: Bc2RegisteredValue,
    pub act_local_resolved_row_count: Bc2RegisteredValue,
    pub named_gap_row_count: Bc2RegisteredValue,
    pub silent_residue_count: Bc2RegisteredValue,
    pub promoted_row_count: Bc2RegisteredValue,
    pub exact_row_order_and_identity: bool,
    pub aggregate_row_disposition_is_total_and_functional: bool,
    pub every_row_archive_join_is_testimony_only: bool,
    pub every_order_obstruction_kept_separate: bool,
    pub every_gap_keeps_row_unpromoted: bool,
    pub all_rows_act_locally_resolved: bool,
    pub c8_closed: bool,
    pub candidate_level_join_proved: bool,
    pub candidate_provenance_and_branch_transport_conditions_promotable: bool,
    pub branch_specific_candidate_prefixes_bound: bool,
    pub cone_level_candidate_claim_issued: bool,
    pub same_act_same_prefix_cross_branch_transport_is_open_obligation: bool,
    pub no_silent_residue: bool,
    pub no_promotions: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2BoundaryProvenanceJoinV1Certificate {
    pub schema: String,
    pub date: String,
    pub numeric_register_policy: String,
    pub source_bindings: Vec<Bc2SourceBinding>,
    pub m3_baseline: Bc2M3BaselineAuthentication,
    pub act_local_authority: Bc2ActLocalAuthorityAudit,
    pub m1_transport: Bc2M1TransportAudit,
    pub source_first_factorization_audit: Bc2SourceFirstFactorizationAudit,
    pub row_dispositions: Vec<Bc2RowDisposition>,
    pub coverage: Bc2CoverageAudit,
    pub global_named_gaps: Vec<String>,
    pub global_named_gap_count: Bc2RegisteredValue,
    pub archive_comparators_are_testimony_only: bool,
    pub desired_verdict_count_bar_hash_or_enumeration_order_used_as_selector: bool,
    pub run_status: Bc2RunStatus,
    pub c8_closed: bool,
    pub candidate_level_join_proved: bool,
    pub bridge_claim_issued: bool,
    pub m4_authorized: bool,
    pub mutation_falsifiers: Vec<String>,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc2Replay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub run_status: Bc2RunStatus,
    pub row_disposition_count: Bc2RegisteredValue,
    pub named_gap_row_count: Bc2RegisteredValue,
    pub c8_closed: bool,
    pub candidate_level_join_proved: bool,
    pub m4_authorized: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Bc2Error {
    #[error("BC-2 prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("BC-2 invariant failed: {0}")]
    Invariant(String),
    #[error("BC-2 JSON failed: {0}")]
    Json(String),
    #[error("BC-2 create-new I/O failed: {0}")]
    Io(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Schema3Projection {
    schema_version: u32,
    signature_digest: String,
    closure_digest: String,
    orbit_derivation_hash: String,
    strata: Vec<Schema3StratumProjection>,
    digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Schema3StratumProjection {
    kappa: u16,
    rows: Vec<Schema3RowProjection>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
struct Schema3RowProjection {
    kappa: u16,
    class: String,
    clause_local_extraction_proxy: String,
    direct_support: u8,
    amplification_route: String,
    count: String,
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BC2_BOUNDARY_PROVENANCE_JOIN_V1_SCHEMA, domain, value))
        .expect("BC-2 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn candidate_join_v4_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(4u32, domain, value)).expect("schema-4 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn m3_certificate_hash(certificate: &M3E7E8BridgeV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let bytes = serde_json::to_vec(&("mainline-m3-e7-e8-bridge-v1", "certificate", &projection))
        .expect("M-3 projection serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn m1_certificate_hash(certificate: &SemanticNuTransportMapsV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    let bytes = serde_json::to_vec(&(
        SEMANTIC_NU_TRANSPORT_MAPS_V2_SCHEMA,
        "certificate",
        &projection,
    ))
    .expect("M-1 projection serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn certificate_hash(certificate: &Bc2BoundaryProvenanceJoinV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn source_bindings() -> Vec<Bc2SourceBinding> {
    [
        (
            "docs/bridge_completion_plan.md",
            "frozen BC-2 mission and falsifiers",
            BRIDGE_PLAN_BYTES,
        ),
        (
            "docs/mainline_completion_plan.md",
            "frozen M-3/M-4 ordering and stop semantics",
            MAINLINE_PLAN_BYTES,
        ),
        (
            "docs/bi0_prerequisites_adjudication.md",
            "adopted act-local authority and F-AL2",
            ACT_LOCAL_ADJUDICATION_BYTES,
        ),
        (
            "docs/nu_register_adjudication.md",
            "semantic authority and structural testimony register split",
            NU_REGISTER_BYTES,
        ),
        (
            "docs/semantic_nu_transport_maps_v2.json",
            "sealed M-1 typed transport maps and order obstructions",
            M1_CERTIFICATE_BYTES,
        ),
        (
            "docs/schema2_e7_e8_bridge_v1.json",
            "sealed M-3 v1 fail-closed baseline",
            M3_CERTIFICATE_BYTES,
        ),
        (
            "docs/ip1_candidate_verdict_join_v3.json",
            "frozen Schema-3 aggregate parent surface",
            SCHEMA3_BYTES,
        ),
        (
            "docs/ip1_candidate_verdict_join_v4.json",
            "Schema-4 archive comparator",
            SCHEMA4_BYTES,
        ),
        (
            "docs/ip1_candidate_verdict_join_v5_element_overlay.json",
            "Schema-5 archive comparator",
            SCHEMA5_BYTES,
        ),
        (
            "crates/pen-search/src/ip1_candidate_join.rs",
            "source-first IP-1 dynamic-program reconstruction",
            IP1_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/act_local_semantic_provenance_v5.rs",
            "candidate-and-prefix-only provenance authority",
            ACT_LOCAL_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/semantic_nu_transport_maps_v2.rs",
            "M-1 map/inverse/coherence issuer",
            M1_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/m3_e7_e8_bridge_v1.rs",
            "M-3 baseline issuer",
            M3_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/bc2_boundary_provenance_join_v1.rs",
            "BC-2 fail-closed issuer and replay",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Bc2SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: metadata(bytes.len(), "bound input byte length"),
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn parse_u128(value: &str, label: &str) -> Result<u128, Bc2Error> {
    value.parse::<u128>().map_err(|error| {
        Bc2Error::Prerequisite(format!("{label} is not a canonical u128: {error}"))
    })
}

fn authenticate_m3(
    certificate: &M3E7E8BridgeV1Certificate,
) -> Result<Bc2M3BaselineAuthentication, Bc2Error> {
    let self_digest_valid = certificate.result_digest == m3_certificate_hash(certificate);
    let result_digest_exact = certificate.result_digest == EXPECTED_M3_V1_RESULT_DIGEST;
    let expected_source_bytes = BTreeMap::from([
        ("docs/ip1_candidate_verdict_join_v3.json", SCHEMA3_BYTES),
        ("docs/ip1_candidate_verdict_join_v4.json", SCHEMA4_BYTES),
        (
            "docs/ip1_candidate_verdict_join_v5_element_overlay.json",
            SCHEMA5_BYTES,
        ),
        (
            "crates/pen-search/src/m3_e7_e8_bridge_v1.rs",
            M3_SOURCE_BYTES,
        ),
    ]);
    let source_binding_bytes_exact = expected_source_bytes.iter().all(|(path, bytes)| {
        certificate.source_bindings.iter().any(|binding| {
            binding.path == *path
                && binding.byte_length.value == bytes.len()
                && binding.byte_length.register == M3Register::ArtifactMetadata
                && binding.register == M3Register::ArtifactMetadata
                && binding.blake3 == bytes_hash(bytes)
        })
    });
    let proved_conditions = certificate
        .e8
        .bridge_conditions
        .iter()
        .filter(|condition| condition.proved)
        .map(|condition| condition.ordinal.value)
        .collect::<Vec<_>>();
    let predecessor_condition_count = certificate.e8.bridge_conditions.len();
    let proved_predecessor_condition_count = proved_conditions.len();
    let open_predecessor_condition_count =
        predecessor_condition_count.saturating_sub(proved_predecessor_condition_count);
    let exact_predecessor_condition_partition_reproduced = proved_conditions == [1, 2, 7, 9]
        && predecessor_condition_count == 9
        && certificate.e8.proved_bridge_condition_count.value == proved_predecessor_condition_count;
    let exact_predecessor_condition_ordinals = proved_conditions
        .iter()
        .map(|ordinal| syntax(*ordinal, "proved M-3 v1 bridge-condition ordinal"))
        .collect::<Vec<_>>();
    let projection = &certificate.enacted_regression.e5_projection;
    let enacted_regression_complete = certificate.enacted_regression.projections_exactly_equal
        && certificate
            .enacted_regression
            .cross_branch_statements_issued_only_after_regression
        && certificate.enacted_regression.e5_frozen_self_digest_valid
        && certificate.enacted_regression.enacted_bi2_self_digest_valid
        && certificate
            .enacted_regression
            .enacted_bi2_root_exactly_joins_bi4
        && projection.exact_d_partition
        && projection.semantic_o16_empty
        && projection.f1_executed
        && projection.f1_excluded
        && projection.theorem12_full_instance_granularity
        && projection.focus_projection_reproduces_ladder
        && projection.stage_three_wrinkle_reproduced
        && projection.e5_complete;
    let registered_e5_drift_disclosed_exactly =
        !certificate.enacted_regression.e5_native_replay_valid
            && certificate
                .enacted_regression
                .frozen_testimony_fallback_used
            && certificate
                .enacted_regression
                .fallback_limited_to_exact_digest_bound_frozen_e5
            && certificate.enacted_regression.e5_native_replay_errors
                == [EXPECTED_M3_E5_DRIFT.to_owned()];
    let registered_e5_drift_repaired_or_concealed =
        certificate.enacted_regression.drift_repaired_inside_m3
            || certificate.enacted_regression.drift_concealed_inside_m3;
    let stopped_named_gaps = certificate.m3_status == M3RunStatus::StoppedNamedGaps
        && !certificate.bridge_claim_issued
        && !certificate.m4_authorized
        && certificate.open_gaps.len() == 3
        && certificate
            .open_gaps
            .iter()
            .all(|gap| gap.keeps_rows_unpromoted);
    let exact_schema3_row_surface = certificate.e8.schema3_strata.value == 3
        && certificate.e8.schema3_aggregate_rows.value == 213
        && certificate.e8.every_schema3_row_is_aggregate_bucket
        && certificate.e8.schema4_all_rows_structurally_joined
        && certificate.e8.schema5_public_replay_valid
        && certificate.e8.schema5_no_row_promoted;
    let no_predecessor_promotions = certificate.zero_promotions_made
        && certificate.promoted_row_count.value == 0
        && certificate
            .cone
            .claims
            .iter()
            .all(|claim| claim.status == M3ClaimStatus::Certified);
    if !self_digest_valid
        || !result_digest_exact
        || !source_binding_bytes_exact
        || !exact_predecessor_condition_partition_reproduced
        || !enacted_regression_complete
        || !registered_e5_drift_disclosed_exactly
        || registered_e5_drift_repaired_or_concealed
        || !stopped_named_gaps
        || !exact_schema3_row_surface
        || !no_predecessor_promotions
    {
        return Err(Bc2Error::Prerequisite(
            "frozen M-3 v1 baseline failed exact authentication".to_owned(),
        ));
    }
    let fields = (
        (
            EXPECTED_M3_V1_RESULT_DIGEST,
            &certificate.result_digest,
            result_digest_exact,
            self_digest_valid,
            source_binding_bytes_exact,
            stopped_named_gaps,
        ),
        (
            predecessor_condition_count,
            proved_predecessor_condition_count,
            open_predecessor_condition_count,
            exact_predecessor_condition_partition_reproduced,
            &exact_predecessor_condition_ordinals,
        ),
        (
            certificate.enacted_regression.projections_exactly_equal,
            enacted_regression_complete,
            registered_e5_drift_disclosed_exactly,
            registered_e5_drift_repaired_or_concealed,
            exact_schema3_row_surface,
            no_predecessor_promotions,
            certificate.m4_authorized,
        ),
    );
    let derivation_hash = tagged_hash("m3-baseline-authentication", &fields);
    Ok(Bc2M3BaselineAuthentication {
        expected_result_digest: EXPECTED_M3_V1_RESULT_DIGEST.to_owned(),
        observed_result_digest: certificate.result_digest.clone(),
        result_digest_exact,
        self_digest_valid,
        source_binding_bytes_exact,
        stopped_named_gaps,
        predecessor_condition_count: metadata(
            predecessor_condition_count,
            "authenticated M-3 v1 bridge-condition inventory",
        ),
        proved_predecessor_condition_count: metadata(
            proved_predecessor_condition_count,
            "authenticated proved M-3 v1 bridge-condition inventory",
        ),
        open_predecessor_condition_count: metadata(
            open_predecessor_condition_count,
            "authenticated open M-3 v1 bridge-condition inventory",
        ),
        exact_predecessor_condition_partition_reproduced,
        exact_predecessor_condition_ordinals,
        enacted_regression_projections_exactly_equal: certificate
            .enacted_regression
            .projections_exactly_equal,
        enacted_regression_complete,
        registered_e5_drift_disclosed_exactly,
        registered_e5_drift_repaired_or_concealed,
        exact_schema3_row_surface,
        no_predecessor_promotions,
        predecessor_m4_authorized: certificate.m4_authorized,
        derivation_hash,
    })
}

fn audit_act_local_authority() -> Result<Bc2ActLocalAuthorityAudit, Bc2Error> {
    let adjudication = std::str::from_utf8(ACT_LOCAL_ADJUDICATION_BYTES)
        .map_err(|error| Bc2Error::Prerequisite(error.to_string()))?;
    if !adjudication.contains("act-local-provenance-v1")
        || !adjudication.contains("F-AL2")
        || !adjudication.contains("candidate and its sealed prefix alone")
        || !adjudication.contains("regression references")
    {
        return Err(Bc2Error::Prerequisite(
            "act-local/F-AL2 markers failed exact source replay".to_owned(),
        ));
    }
    let authority = issue_v5_prefix_local_rule_authority();
    let rule_authority_replay_valid = replay_v5_prefix_local_rule_authority(&authority);
    let required_capabilities = [
        V5PrefixLocalIssuanceCapability::CandidateAndExactPrefix,
        V5PrefixLocalIssuanceCapability::PredecessorSemanticSurface,
        V5PrefixLocalIssuanceCapability::CubicalDecisionSurface,
    ];
    let candidate_and_exact_prefix_capability_required =
        required_capabilities.contains(&V5PrefixLocalIssuanceCapability::CandidateAndExactPrefix);
    let predecessor_semantic_surface_capability_required = required_capabilities
        .contains(&V5PrefixLocalIssuanceCapability::PredecessorSemanticSurface);
    let cubical_decision_surface_capability_required =
        required_capabilities.contains(&V5PrefixLocalIssuanceCapability::CubicalDecisionSurface);
    let same_act_same_prefix_deterministic_replay_rule_bound = ACT_LOCAL_SOURCE_BYTES
        .windows(b"candidate-prefix-binding".len())
        .any(|window| window == b"candidate-prefix-binding")
        && ACT_LOCAL_SOURCE_BYTES
            .windows(b"differs from deterministic reissuance".len())
            .any(|window| window == b"differs from deterministic reissuance");
    let fields = (
        (
            &authority.derivation_hash,
            rule_authority_replay_valid,
            authority.semantic_family_is_unit_of_credit,
            authority.uniform_instances_require_independent_export,
            authority.support_hypotheses_mint_no_credit,
            authority.zero_silent_residue_required,
            authority.source_document_read,
            authority.structural_scalar_read,
            authority.desired_vector_read,
        ),
        (
            candidate_and_exact_prefix_capability_required,
            predecessor_semantic_surface_capability_required,
            cubical_decision_surface_capability_required,
            false,
            true,
            same_act_same_prefix_deterministic_replay_rule_bound,
            true,
            false,
        ),
    );
    if !rule_authority_replay_valid
        || !authority.semantic_family_is_unit_of_credit
        || !authority.uniform_instances_require_independent_export
        || !authority.support_hypotheses_mint_no_credit
        || !authority.zero_silent_residue_required
        || authority.source_document_read
        || authority.structural_scalar_read
        || authority.desired_vector_read
        || !same_act_same_prefix_deterministic_replay_rule_bound
    {
        return Err(Bc2Error::Prerequisite(
            "act-local rule authority failed replay or F-AL2 hygiene".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash("act-local-authority-audit", &fields);
    Ok(Bc2ActLocalAuthorityAudit {
        rule_authority_derivation_hash: authority.derivation_hash,
        rule_authority_replay_valid,
        semantic_family_is_unit_of_credit: authority.semantic_family_is_unit_of_credit,
        uniform_instances_require_independent_export: authority
            .uniform_instances_require_independent_export,
        support_hypotheses_mint_no_credit: authority.support_hypotheses_mint_no_credit,
        no_silent_residue_required: authority.zero_silent_residue_required,
        source_document_read: authority.source_document_read,
        structural_scalar_read: authority.structural_scalar_read,
        desired_vector_read: authority.desired_vector_read,
        candidate_and_exact_prefix_capability_required,
        predecessor_semantic_surface_capability_required,
        cubical_decision_surface_capability_required,
        archive_join_is_authority: false,
        archive_join_is_regression_comparator_only: true,
        same_act_same_prefix_deterministic_replay_rule_bound,
        no_enacted_future_input_permitted: true,
        no_value_bar_hash_or_enumeration_order_selector: true,
        derivation_hash,
    })
}

fn audit_m1(
    certificate: &SemanticNuTransportMapsV2Certificate,
) -> Result<Bc2M1TransportAudit, Bc2Error> {
    let result_digest_exact = certificate.result_digest == EXPECTED_M1_RESULT_DIGEST;
    let self_digest_valid = certificate.result_digest == m1_certificate_hash(certificate);
    let scope_boundary_exact =
        certificate.scope_boundary_id == SEMANTIC_NU_TRANSPORT_SCOPE_BOUNDARY;
    if certificate.exported_map_count.register != NumericRegister::ProofInventory
        || certificate.order_axis_obstruction_count.register != NumericRegister::ProofInventory
        || certificate.exported_map_count.value != 5
        || certificate.order_axis_obstruction_count.value != 2
    {
        return Err(Bc2Error::Prerequisite(
            "M-1 registered map/obstruction inventory changed".to_owned(),
        ));
    }
    let maps = certificate
        .typed_transport_maps
        .iter()
        .map(|map| Bc2TransportMapReference {
            relation_id: map.relation_id.clone(),
            source_object_id: map.source_object_id.clone(),
            target_object_id: map.target_object_id.clone(),
            map_derivation_hash: map.derivation_hash.clone(),
            typed_total_map: map.forward_is_typed_total_function,
            typed_total_inverse: map.inverse_is_typed_total_function,
            identity_composition_coherent: map.maps_are_two_sided_inverses
                && map.source_identity_coherence
                && map.target_identity_coherence,
            semantic_family_nu_invariant: map.semantic_nu_invariant,
        })
        .collect::<Vec<_>>();
    let all_maps_typed_total_inverse_coherent_and_nu_invariant = maps.iter().all(|map| {
        map.typed_total_map
            && map.typed_total_inverse
            && map.identity_composition_coherent
            && map.semantic_family_nu_invariant
    });
    let order_obstructions = certificate
        .order_axis_obstructions
        .iter()
        .map(|obstruction| {
            if obstruction.left_semantic_nu.register != NumericRegister::SemanticFamilyNu
                || obstruction.right_semantic_nu.register != NumericRegister::SemanticFamilyNu
            {
                return Err(Bc2Error::Prerequisite(
                    "M-1 obstruction semantic nu is in the wrong register".to_owned(),
                ));
            }
            Ok(Bc2OrderObstructionReference {
                fixed_coordinate: obstruction.fixed_coordinate.clone(),
                left_candidate_hash: obstruction.left_candidate_hash.clone(),
                right_candidate_hash: obstruction.right_candidate_hash.clone(),
                left_semantic_nu: registered(
                    obstruction.left_semantic_nu.value,
                    Bc2Register::SemanticFamilyAuthority,
                    "left semantic-family nu in certified order obstruction",
                ),
                right_semantic_nu: registered(
                    obstruction.right_semantic_nu.value,
                    Bc2Register::SemanticFamilyAuthority,
                    "right semantic-family nu in certified order obstruction",
                ),
                semantic_nu_distinct: obstruction.semantic_nu_distinct,
                no_accepted_typed_family_bijection_exists: obstruction
                    .no_accepted_typed_family_bijection_exists,
                kept_separate_by_bc2: true,
                derivation_hash: obstruction.derivation_hash.clone(),
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let registered_order_obstruction_inventory_preserved = order_obstructions.len() == 2
        && order_obstructions.iter().all(|obstruction| {
            obstruction.semantic_nu_distinct
                && obstruction.no_accepted_typed_family_bijection_exists
                && obstruction.kept_separate_by_bc2
        });
    if !result_digest_exact
        || !self_digest_valid
        || !scope_boundary_exact
        || !all_maps_typed_total_inverse_coherent_and_nu_invariant
        || !registered_order_obstruction_inventory_preserved
        || !certificate.no_new_equivalence_semantics_assumed
        || certificate.desired_verdict_count_bar_hash_or_enumeration_order_used_as_selector
    {
        return Err(Bc2Error::Prerequisite(
            "sealed M-1 transport theorem failed BC-2 authentication".to_owned(),
        ));
    }
    let fields = (
        EXPECTED_M1_RESULT_DIGEST,
        &certificate.result_digest,
        result_digest_exact,
        self_digest_valid,
        &certificate.scope_boundary_id,
        scope_boundary_exact,
        &maps,
        all_maps_typed_total_inverse_coherent_and_nu_invariant,
        &order_obstructions,
        registered_order_obstruction_inventory_preserved,
        false,
        certificate.no_new_equivalence_semantics_assumed,
    );
    let derivation_hash = tagged_hash("m1-transport-audit", &fields);
    Ok(Bc2M1TransportAudit {
        expected_result_digest: EXPECTED_M1_RESULT_DIGEST.to_owned(),
        observed_result_digest: certificate.result_digest.clone(),
        result_digest_exact,
        self_digest_valid,
        scope_boundary_id: certificate.scope_boundary_id.clone(),
        scope_boundary_exact,
        exported_map_count: metadata(maps.len(), "typed family-class transport maps consumed"),
        maps,
        all_maps_typed_total_inverse_coherent_and_nu_invariant,
        order_obstruction_count: metadata(
            order_obstructions.len(),
            "certified order-axis no-transport obstructions consumed",
        ),
        order_obstructions,
        registered_order_obstruction_inventory_preserved,
        cross_obstruction_merge_attempted: false,
        no_new_equivalence_semantics_assumed: certificate.no_new_equivalence_semantics_assumed,
        derivation_hash,
    })
}

fn audit_source_first_factorization(
    schema3: &Schema3Projection,
) -> Result<Bc2SourceFirstFactorizationAudit, Bc2Error> {
    let ip1 = std::str::from_utf8(IP1_SOURCE_BYTES)
        .map_err(|error| Bc2Error::Prerequisite(error.to_string()))?;
    let act_local = std::str::from_utf8(ACT_LOCAL_SOURCE_BYTES)
        .map_err(|error| Bc2Error::Prerequisite(error.to_string()))?;
    let key_start = ip1.find("struct ExprJoinKey {").ok_or_else(|| {
        Bc2Error::Prerequisite("IP-1 ExprJoinKey declaration is absent".to_owned())
    })?;
    let key_end = ip1[key_start..]
        .find("fn classify_position_exprs")
        .map(|offset| key_start + offset)
        .ok_or_else(|| Bc2Error::Prerequisite("IP-1 ExprJoinKey boundary is absent".to_owned()))?;
    let dp_key = &ip1[key_start..key_end];
    let ip1_source_bound = !IP1_SOURCE_BYTES.is_empty();
    let act_local_source_bound = !ACT_LOCAL_SOURCE_BYTES.is_empty();
    let ip1_dynamic_program_reconstructs_aggregate_rows = ip1.contains("fn exhaust_stratum(")
        && ip1.contains("BTreeMap<ExprJoinKey, u128>")
        && ip1.contains("fn row_key(");
    let aggregate_rows_require_candidate_refinement = schema3
        .strata
        .iter()
        .flat_map(|stratum| &stratum.rows)
        .all(|row| parse_u128(&row.count, "Schema-3 parent multiplicity").is_ok_and(|n| n > 1));
    let ip1_dp_key_carries_candidate_telescope =
        dp_key.contains("Telescope") || dp_key.contains("candidate:");
    let ip1_dp_key_carries_normalized_family_identity = dp_key.contains("normal_form")
        || dp_key.contains("normalized_family")
        || dp_key.contains("family_id");
    let ip1_dp_key_carries_role_declaration_identity =
        dp_key.contains("declaration_id") || dp_key.contains("role_declaration");
    let ip1_dp_key_carries_unified_equivalence_closure =
        dp_key.contains("unified_equivalence") || dp_key.contains("equivalence_closure");
    let ip1_dp_key_carries_anchor_relation =
        dp_key.contains("anchor_relation") || dp_key.contains("anchor_disposition");
    let ip1_dp_key_carries_a3_orbit_membership =
        dp_key.contains("a3_orbit") || dp_key.contains("quotient_orbit_id");
    let ip1_dp_key_carries_predecessor_semantic_surface =
        dp_key.contains("predecessor_member") || dp_key.contains("predecessor_semantic");
    let ip1_dp_key_carries_cubical_decision_surface =
        dp_key.contains("cubical_decision") || dp_key.contains("path_quotient");
    let act_local_token_binds_candidate_hash = act_local.contains("pub candidate_hash: String")
        && act_local.contains("candidate_hash(candidate)");
    let act_local_token_binds_exact_prefix_digest = act_local
        .contains("pub predecessor_signature_digest: String")
        && act_local.contains("prefix.digest()");
    let act_local_issuer_is_cross_clause_and_prefix_sensitive = act_local
        .contains("predecessor_members")
        && act_local.contains("unified_quotient")
        && act_local.contains("cubical_decisions")
        && act_local.contains("anchor_families");
    let registered_factorization_theorem_marker_found = ip1
        .contains("IP1_DP_KEY_PROVENANCE_CONGRUENCE")
        || ip1.contains("provenance_factorization_through_ip1")
        || ip1.contains("act_local_provenance_factors_through");
    let provenance_factorization_through_ip1_dp_key_proved = false;
    let source_first_symbolic_join_proved = false;
    let source_first_route_audited = ip1_source_bound
        && act_local_source_bound
        && ip1_dynamic_program_reconstructs_aggregate_rows
        && act_local_token_binds_candidate_hash
        && act_local_token_binds_exact_prefix_digest
        && act_local_issuer_is_cross_clause_and_prefix_sensitive;
    if !source_first_route_audited
        || !aggregate_rows_require_candidate_refinement
        || ip1_dp_key_carries_candidate_telescope
        || ip1_dp_key_carries_normalized_family_identity
        || ip1_dp_key_carries_role_declaration_identity
        || ip1_dp_key_carries_unified_equivalence_closure
        || ip1_dp_key_carries_anchor_relation
        || ip1_dp_key_carries_a3_orbit_membership
        || ip1_dp_key_carries_predecessor_semantic_surface
        || ip1_dp_key_carries_cubical_decision_surface
        || registered_factorization_theorem_marker_found
    {
        return Err(Bc2Error::Invariant(
            "source-first DP/provenance non-factorization audit changed".to_owned(),
        ));
    }
    let exact_obstruction = "The source-first IP-1 dynamic program reconstructs the frozen aggregate rows, but its ExprJoinKey forgets the candidate telescope and every normalized-family, role-declaration, unified-equivalence, anchor, A3-orbit, predecessor-semantic-surface, and cubical-decision coordinate consumed by the act-local issuer. Every frozen bucket is an aggregate rather than a candidate payload, provenance tokens bind the candidate hash and exact prefix, and BC-2 constructs no proof that provenance is congruent on an IP-1 key. Therefore neither rerunning the current source-first API nor reading the archive yields a lawful row-level provenance token; a candidate-preserving refinement or a proof of this exact factorization is required."
        .to_owned();
    let fields = (
        (
            ip1_source_bound,
            act_local_source_bound,
            source_first_route_audited,
            ip1_dynamic_program_reconstructs_aggregate_rows,
            aggregate_rows_require_candidate_refinement,
            ip1_dp_key_carries_candidate_telescope,
            ip1_dp_key_carries_normalized_family_identity,
            ip1_dp_key_carries_role_declaration_identity,
            ip1_dp_key_carries_unified_equivalence_closure,
            ip1_dp_key_carries_anchor_relation,
            ip1_dp_key_carries_a3_orbit_membership,
        ),
        (
            ip1_dp_key_carries_predecessor_semantic_surface,
            ip1_dp_key_carries_cubical_decision_surface,
            act_local_token_binds_candidate_hash,
            act_local_token_binds_exact_prefix_digest,
            act_local_issuer_is_cross_clause_and_prefix_sensitive,
            registered_factorization_theorem_marker_found,
            provenance_factorization_through_ip1_dp_key_proved,
            source_first_symbolic_join_proved,
            BC2_FACTORIZATION_GAP,
            &exact_obstruction,
        ),
    );
    let derivation_hash = tagged_hash("source-first-factorization-audit", &fields);
    Ok(Bc2SourceFirstFactorizationAudit {
        ip1_source_bound,
        act_local_source_bound,
        source_first_route_audited,
        ip1_dynamic_program_reconstructs_aggregate_rows,
        aggregate_rows_require_candidate_refinement,
        ip1_dp_key_carries_candidate_telescope,
        ip1_dp_key_carries_normalized_family_identity,
        ip1_dp_key_carries_role_declaration_identity,
        ip1_dp_key_carries_unified_equivalence_closure,
        ip1_dp_key_carries_anchor_relation,
        ip1_dp_key_carries_a3_orbit_membership,
        ip1_dp_key_carries_predecessor_semantic_surface,
        ip1_dp_key_carries_cubical_decision_surface,
        act_local_token_binds_candidate_hash,
        act_local_token_binds_exact_prefix_digest,
        act_local_issuer_is_cross_clause_and_prefix_sensitive,
        registered_factorization_theorem_marker_found,
        provenance_factorization_through_ip1_dp_key_proved,
        source_first_symbolic_join_proved,
        exact_missing_theorem_id: BC2_FACTORIZATION_GAP.to_owned(),
        exact_obstruction,
        derivation_hash,
    })
}

fn raw_schema3_rows_have_no_candidate_payload(raw: &serde_json::Value) -> Result<bool, Bc2Error> {
    let strata = raw
        .get("strata")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| Bc2Error::Json("Schema-3 strata are absent".to_owned()))?;
    let forbidden = [
        "candidate",
        "telescope",
        "clauses",
        "natural_families",
        "semantic_demand_orbits",
        "marginality_decisions",
        "provenance_tokens",
        "predecessor_semantic_surface",
        "cubical_decisions",
    ];
    Ok(strata.iter().all(|stratum| {
        stratum
            .get("rows")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|rows| {
                rows.iter().all(|row| {
                    row.as_object().is_some_and(|object| {
                        forbidden.iter().all(|key| !object.contains_key(*key))
                    })
                })
            })
    }))
}

fn row_disposition_hash(row: &Bc2RowDisposition) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("row-disposition", &projection)
}

fn build_row_dispositions(
    schema3: &Schema3Projection,
    raw_schema3: &serde_json::Value,
    schema4: &CandidateJoinV4Certificate,
    schema5: &CandidateJoinV5Certificate,
    m1: &Bc2M1TransportAudit,
) -> Result<Vec<Bc2RowDisposition>, Bc2Error> {
    if schema3.strata.len() != schema4.strata.len()
        || schema3.strata.len() != schema5.strata.len()
        || !raw_schema3_rows_have_no_candidate_payload(raw_schema3)?
    {
        return Err(Bc2Error::Prerequisite(
            "Schema-3/4/5 row surfaces do not align or candidate payload appeared".to_owned(),
        ));
    }
    let mut dispositions = Vec::new();
    for (stratum_index, schema3_stratum) in schema3.strata.iter().enumerate() {
        let schema4_stratum = &schema4.strata[stratum_index];
        let schema5_stratum = &schema5.strata[stratum_index];
        if schema3_stratum.kappa != schema4_stratum.kappa
            || schema3_stratum.kappa != schema5_stratum.kappa
            || schema3_stratum.rows.len() != schema4_stratum.rows.len()
            || schema3_stratum.rows.len() != schema5_stratum.rows.len()
        {
            return Err(Bc2Error::Prerequisite(format!(
                "Schema-3/4/5 stratum mismatch at {stratum_index}"
            )));
        }
        for (row_index, schema3_row) in schema3_stratum.rows.iter().enumerate() {
            let schema4_row = &schema4_stratum.rows[row_index];
            let schema5_row = &schema5_stratum.rows[row_index];
            let schema3_row_digest = candidate_join_v4_hash(
                "schema3-projected-row",
                &(stratum_index, row_index, schema3_row),
            );
            let archive_comparator_matched = usize::from(schema4_row.stratum_index)
                == stratum_index
                && usize::from(schema4_row.row_index) == row_index
                && schema4_row.kappa == schema3_row.kappa
                && schema4_row.schema3_row_digest == schema3_row_digest
                && usize::from(schema5_row.stratum_index) == stratum_index
                && usize::from(schema5_row.row_index) == row_index
                && schema5_row.kappa == schema3_row.kappa
                && schema5_row.schema3_row_digest == schema3_row_digest
                && schema5_row.schema4_row_derivation_hash == schema4_row.derivation_hash;
            if !archive_comparator_matched
                || schema4_row.all_required_evidence_proved
                || schema4_row.full_candidate_extraction_join_proved
                || schema5_row.all_required_evidence_proved
                || schema5_row.full_candidate_extraction_join_proved
            {
                return Err(Bc2Error::Prerequisite(format!(
                    "archive comparator shifted or promoted row {stratum_index}/{row_index}"
                )));
            }
            let parent_multiplicity = parse_u128(
                &schema3_row.count,
                &format!("Schema-3 row {stratum_index}/{row_index} multiplicity"),
            )?;
            if parent_multiplicity <= 1 {
                return Err(Bc2Error::Invariant(format!(
                    "Schema-3 row {stratum_index}/{row_index} is not aggregate"
                )));
            }
            let aggregate_row_id =
                format!("schema3:S{stratum_index}/R{row_index}:{schema3_row_digest}");
            let mut disposition = Bc2RowDisposition {
                aggregate_row_id: aggregate_row_id.clone(),
                stratum_index: syntax(stratum_index, "origin-based Schema-3 stratum coordinate"),
                row_index: syntax(row_index, "origin-based Schema-3 row coordinate"),
                kappa: structural(
                    schema3_row.kappa,
                    "candidate clause-arity structural testimony",
                ),
                class: schema3_row.class.clone(),
                clause_local_extraction_proxy: schema3_row.clause_local_extraction_proxy.clone(),
                direct_support: structural(
                    schema3_row.direct_support,
                    "coarse Schema-3 direct-support testimony",
                ),
                amplification_route: schema3_row.amplification_route.clone(),
                parent_multiplicity: structural(
                    &schema3_row.count,
                    "aggregate parent-candidate multiplicity testimony",
                ),
                schema3_row_digest,
                schema4_row_derivation_hash: schema4_row.derivation_hash.clone(),
                schema5_row_derivation_hash: schema5_row.derivation_hash.clone(),
                archive_comparator_matched,
                archive_comparator_is_authority: false,
                candidate_payload_present: false,
                telescope_payload_present: false,
                exact_prefix_digest_present: !schema3.signature_digest.is_empty(),
                candidate_and_prefix_input_complete: false,
                natural_families_extracted: false,
                semantic_demand_orbits_extracted: false,
                marginality_decided: false,
                candidate_local_provenance_token_issued: false,
                branch_transport_replay_proved: false,
                m1_transport_instantiated_for_candidate_families: false,
                order_obstruction_separation_preserved: m1
                    .registered_order_obstruction_inventory_preserved,
                disposition: Bc2RowDispositionStatus::NamedGap,
                named_gap: BC2_ROW_GAP.to_owned(),
                exact_residue: format!(
                    "{aggregate_row_id} has a sealed prefix digest and exact archive comparators, but the IP-1 aggregate key supplies no candidate telescope or provenance-congruence theorem. Candidate-local family/orbit extraction, marginality, M-1 carrier instantiation, and same-act/same-prefix branch replay therefore cannot be issued."
                ),
                promoted: false,
                derivation_hash: String::new(),
            };
            disposition.derivation_hash = row_disposition_hash(&disposition);
            dispositions.push(disposition);
        }
    }
    Ok(dispositions)
}

fn build_coverage(
    schema3: &Schema3Projection,
    rows: &[Bc2RowDisposition],
) -> Result<Bc2CoverageAudit, Bc2Error> {
    let expected_row_count = schema3
        .strata
        .iter()
        .map(|stratum| stratum.rows.len())
        .sum::<usize>();
    let identities = rows
        .iter()
        .map(|row| row.aggregate_row_id.as_str())
        .collect::<BTreeSet<_>>();
    let exact_row_order_and_identity = rows.iter().enumerate().all(|(flat_index, row)| {
        let mut cursor = 0usize;
        for (stratum_index, stratum) in schema3.strata.iter().enumerate() {
            for row_index in 0..stratum.rows.len() {
                if cursor == flat_index {
                    return row
                        .aggregate_row_id
                        .starts_with(&format!("schema3:S{stratum_index}/R{row_index}:blake3:"));
                }
                cursor += 1;
            }
        }
        false
    });
    let aggregate_row_disposition_is_total_and_functional =
        rows.len() == expected_row_count && identities.len() == expected_row_count;
    let named_gap_count = rows
        .iter()
        .filter(|row| row.disposition == Bc2RowDispositionStatus::NamedGap)
        .count();
    let all_rows_act_locally_resolved = rows.iter().all(|row| {
        row.candidate_and_prefix_input_complete
            && row.natural_families_extracted
            && row.semantic_demand_orbits_extracted
            && row.marginality_decided
            && row.candidate_local_provenance_token_issued
            && row.branch_transport_replay_proved
    });
    let silent_residue_count = rows
        .iter()
        .filter(|row| {
            !row.candidate_local_provenance_token_issued
                && (row.named_gap.is_empty()
                    || row.disposition != Bc2RowDispositionStatus::NamedGap)
        })
        .count();
    let promoted_count = rows.iter().filter(|row| row.promoted).count();
    let every_row_archive_join_is_testimony_only = rows
        .iter()
        .all(|row| row.archive_comparator_matched && !row.archive_comparator_is_authority);
    let every_order_obstruction_kept_separate = rows
        .iter()
        .all(|row| row.order_obstruction_separation_preserved);
    let every_gap_keeps_row_unpromoted = rows
        .iter()
        .all(|row| !row.promoted && !row.candidate_local_provenance_token_issued);
    let c8_closed = false;
    let candidate_level_join_proved = false;
    let candidate_provenance_and_branch_transport_conditions_promotable = false;
    let branch_specific_candidate_prefixes_bound = false;
    let cone_level_candidate_claim_issued = false;
    let same_act_same_prefix_cross_branch_transport_is_open_obligation = true;
    let no_silent_residue = silent_residue_count == 0;
    let no_promotions = promoted_count == 0;
    if !exact_row_order_and_identity
        || !aggregate_row_disposition_is_total_and_functional
        || named_gap_count != expected_row_count
        || !every_row_archive_join_is_testimony_only
        || !every_order_obstruction_kept_separate
        || !every_gap_keeps_row_unpromoted
        || all_rows_act_locally_resolved
        || branch_specific_candidate_prefixes_bound
        || cone_level_candidate_claim_issued
        || !same_act_same_prefix_cross_branch_transport_is_open_obligation
        || !no_silent_residue
        || !no_promotions
    {
        return Err(Bc2Error::Invariant(
            "BC-2 row coverage or fail-closed disposition failed".to_owned(),
        ));
    }
    let fields = (
        (
            Bc2Quantifier::FrozenWrappedSurface,
            schema3.strata.len(),
            expected_row_count,
            rows.len(),
            0usize,
            named_gap_count,
            silent_residue_count,
            promoted_count,
            exact_row_order_and_identity,
        ),
        (
            aggregate_row_disposition_is_total_and_functional,
            every_row_archive_join_is_testimony_only,
            every_order_obstruction_kept_separate,
            every_gap_keeps_row_unpromoted,
            all_rows_act_locally_resolved,
            c8_closed,
            candidate_level_join_proved,
            candidate_provenance_and_branch_transport_conditions_promotable,
            branch_specific_candidate_prefixes_bound,
            cone_level_candidate_claim_issued,
            same_act_same_prefix_cross_branch_transport_is_open_obligation,
            no_silent_residue,
            no_promotions,
        ),
    );
    let derivation_hash = tagged_hash("coverage-audit", &fields);
    Ok(Bc2CoverageAudit {
        quantifier: Bc2Quantifier::FrozenWrappedSurface,
        schema3_stratum_count: metadata(
            schema3.strata.len(),
            "frozen Schema-3 strata dispositioned",
        ),
        schema3_row_count: metadata(expected_row_count, "frozen Schema-3 aggregate rows"),
        explicit_row_disposition_count: metadata(rows.len(), "explicit BC-2 row dispositions"),
        act_local_resolved_row_count: metadata(
            0,
            "rows with candidate-local family/orbit provenance proved",
        ),
        named_gap_row_count: metadata(named_gap_count, "rows stopped with an exact named gap"),
        silent_residue_count: metadata(
            silent_residue_count,
            "rows without either proof or named gap",
        ),
        promoted_row_count: metadata(promoted_count, "rows promoted by BC-2"),
        exact_row_order_and_identity,
        aggregate_row_disposition_is_total_and_functional,
        every_row_archive_join_is_testimony_only,
        every_order_obstruction_kept_separate,
        every_gap_keeps_row_unpromoted,
        all_rows_act_locally_resolved,
        c8_closed,
        candidate_level_join_proved,
        candidate_provenance_and_branch_transport_conditions_promotable,
        branch_specific_candidate_prefixes_bound,
        cone_level_candidate_claim_issued,
        same_act_same_prefix_cross_branch_transport_is_open_obligation,
        no_silent_residue,
        no_promotions,
        derivation_hash,
    })
}

fn build_certificate() -> Result<Bc2BoundaryProvenanceJoinV1Certificate, Bc2Error> {
    let bridge_plan = std::str::from_utf8(BRIDGE_PLAN_BYTES)
        .map_err(|error| Bc2Error::Prerequisite(error.to_string()))?;
    let mainline_plan = std::str::from_utf8(MAINLINE_PLAN_BYTES)
        .map_err(|error| Bc2Error::Prerequisite(error.to_string()))?;
    let register = std::str::from_utf8(NU_REGISTER_BYTES)
        .map_err(|error| Bc2Error::Prerequisite(error.to_string()))?;
    if !bridge_plan.contains("## BC-2")
        || !bridge_plan.contains("candidate boundary provenance join")
        || !bridge_plan.contains("F-AL2 verbatim")
        || !bridge_plan.contains("Every aggregate row's provenance resolves act-locally")
        || !bridge_plan.contains("two certified order-axis obstructions")
        || !mainline_plan.contains("M-3")
        || !mainline_plan.contains("M-4")
        || !register.contains("nu-register-split-v1")
        || !register.contains("F-NR6")
    {
        return Err(Bc2Error::Prerequisite(
            "frozen BC-2/mainline/register plan markers failed replay".to_owned(),
        ));
    }

    let m3: M3E7E8BridgeV1Certificate = serde_json::from_slice(M3_CERTIFICATE_BYTES)
        .map_err(|error| Bc2Error::Json(error.to_string()))?;
    let m1: SemanticNuTransportMapsV2Certificate = serde_json::from_slice(M1_CERTIFICATE_BYTES)
        .map_err(|error| Bc2Error::Json(error.to_string()))?;
    let schema3: Schema3Projection =
        serde_json::from_slice(SCHEMA3_BYTES).map_err(|error| Bc2Error::Json(error.to_string()))?;
    let raw_schema3: serde_json::Value =
        serde_json::from_slice(SCHEMA3_BYTES).map_err(|error| Bc2Error::Json(error.to_string()))?;
    let schema4: CandidateJoinV4Certificate =
        serde_json::from_slice(SCHEMA4_BYTES).map_err(|error| Bc2Error::Json(error.to_string()))?;
    let schema5: CandidateJoinV5Certificate =
        serde_json::from_slice(SCHEMA5_BYTES).map_err(|error| Bc2Error::Json(error.to_string()))?;

    if schema3.schema_version != 3
        || schema3.signature_digest != crate::candidate_join::SCHEMA3_SIGNATURE_DIGEST
        || schema3.closure_digest != crate::candidate_join::SCHEMA3_CLOSURE_DIGEST
        || schema3.orbit_derivation_hash != crate::candidate_join::SCHEMA3_ORBIT_DERIVATION_HASH
        || schema3.digest != crate::candidate_join::SCHEMA3_ARCHIVE_INTERNAL_DIGEST
        || schema3.strata.len() != 3
        || schema3
            .strata
            .iter()
            .map(|stratum| stratum.rows.len())
            .collect::<Vec<_>>()
            != [63, 72, 78]
    {
        return Err(Bc2Error::Prerequisite(
            "frozen Schema-3 projection failed exact replay".to_owned(),
        ));
    }

    let m3_baseline = authenticate_m3(&m3)?;
    let act_local_authority = audit_act_local_authority()?;
    let m1_transport = audit_m1(&m1)?;
    let source_first_factorization_audit = audit_source_first_factorization(&schema3)?;
    let row_dispositions =
        build_row_dispositions(&schema3, &raw_schema3, &schema4, &schema5, &m1_transport)?;
    let coverage = build_coverage(&schema3, &row_dispositions)?;
    let global_named_gaps = vec![BC2_GLOBAL_GAP.to_owned(), BC2_FACTORIZATION_GAP.to_owned()];

    if coverage.c8_closed
        || coverage.candidate_level_join_proved
        || coverage.candidate_provenance_and_branch_transport_conditions_promotable
        || !coverage.no_silent_residue
        || !coverage.no_promotions
        || source_first_factorization_audit.source_first_symbolic_join_proved
    {
        return Err(Bc2Error::Invariant(
            "BC-2 attempted a partial or unsupported promotion".to_owned(),
        ));
    }

    let mut certificate = Bc2BoundaryProvenanceJoinV1Certificate {
        schema: BC2_BOUNDARY_PROVENANCE_JOIN_V1_SCHEMA.to_owned(),
        date: BC2_BOUNDARY_PROVENANCE_JOIN_V1_DATE.to_owned(),
        numeric_register_policy: "Every quantitative value introduced by BC-2 is wrapped in Bc2RegisteredValue. Parent multiplicities, kappa, and coarse support are structural_testimony; theorem/map/row inventories and byte lengths are artifact_metadata; stratum/row coordinates are syntax_identifier. Dates, version suffixes, obligation labels, hashes, and aggregate-row IDs are syntax, not untagged value claims.".to_owned(),
        source_bindings: source_bindings(),
        m3_baseline,
        act_local_authority,
        m1_transport,
        source_first_factorization_audit,
        row_dispositions,
        coverage,
        global_named_gaps,
        global_named_gap_count: metadata(2, "global BC-2 named theorem gaps"),
        archive_comparators_are_testimony_only: true,
        desired_verdict_count_bar_hash_or_enumeration_order_used_as_selector: false,
        run_status: Bc2RunStatus::StoppedNamedGaps,
        c8_closed: false,
        candidate_level_join_proved: false,
        bridge_claim_issued: false,
        m4_authorized: false,
        mutation_falsifiers: vec![
            "change_any_bound_source_or_certificate_byte_or_digest_then_replay_must_fail"
                .to_owned(),
            "shift_any_proved_M3_v1_baseline_condition_then_replay_must_fail"
                .to_owned(),
            "conceal_expand_or_repair_the_registered_M3_E5_drift_then_replay_must_fail"
                .to_owned(),
            "consume_a_Schema4_or_Schema5_archive_join_as_provenance_authority_then_replay_must_fail"
                .to_owned(),
            "remove_or_promote_any_explicit_named_gap_row_then_replay_must_fail"
                .to_owned(),
            "reorder_duplicate_or_mutate_any_aggregate_row_id_or_row_digest_then_replay_must_fail"
                .to_owned(),
            "claim_source_first_factorization_without_the_missing_congruence_theorem_then_replay_must_fail"
                .to_owned(),
            "merge_across_any_registered_M1_order_axis_obstruction_then_replay_must_fail"
                .to_owned(),
            "retag_any_published_quantity_then_replay_must_fail".to_owned(),
            "use_value_bar_hash_or_enumeration_order_as_a_selector_then_replay_must_fail".to_owned(),
            "issue_C8_bridge_or_M4_authorization_while_any_row_is_a_named_gap_then_replay_must_fail"
                .to_owned(),
            "introduce_any_F_BC6_parked_scope_item_then_replay_must_fail".to_owned(),
            "add_any_unknown_JSON_field_then_replay_must_fail".to_owned(),
        ],
        permitted_conclusion: "BC-2 exhaustively audits the source-first and archived routes on the exact frozen wrapped surface. The frozen M-3 baseline, act-local F-AL2 authority, every registered M-1 typed map and order obstruction, and the exact Schema-3/4/5 row surface are authenticated. Each aggregate row has an explicit named-gap disposition, archive joins remain testimony only, registered obstructions remain separated, and there is no silent residue. The current IP-1 DP key is not proved provenance-congruent, so no candidate-local provenance token, cone-wide candidate claim, or bridge promotion is issued.".to_owned(),
        required_successor_action: "Refine the source-first IP-1 state with candidate-preserving typed family/orbit/anchor data, or prove a universal theorem that act-local provenance factors through the existing DP key. A later branch-indexed successor must bind each exact candidate/prefix input before discharging the still-open same-act/same-prefix transport obligation. Each explicit BC-2 row gap must become a proof before C8 or M-4 can open.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_hash(&certificate);
    Ok(certificate)
}

pub fn issue_bc2_boundary_provenance_join_v1()
-> Result<Bc2BoundaryProvenanceJoinV1Certificate, Bc2Error> {
    build_certificate()
}

fn invalid_replay(error: impl Into<String>) -> Bc2Replay {
    Bc2Replay {
        valid: false,
        errors: vec![error.into()],
        run_status: Bc2RunStatus::StoppedNamedGaps,
        row_disposition_count: metadata(0, "unavailable row disposition count"),
        named_gap_row_count: metadata(0, "unavailable named-gap row count"),
        c8_closed: false,
        candidate_level_join_proved: false,
        m4_authorized: false,
    }
}

pub fn replay_bc2_boundary_provenance_join_v1(
    claimed: &Bc2BoundaryProvenanceJoinV1Certificate,
) -> Bc2Replay {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_hash(claimed) {
        errors.push("BC-2 certificate self-digest mismatch".to_owned());
    }
    if claimed
        .row_dispositions
        .iter()
        .any(|row| row.derivation_hash != row_disposition_hash(row))
    {
        errors.push("a BC-2 row disposition digest failed".to_owned());
    }
    let row_ids = claimed
        .row_dispositions
        .iter()
        .map(|row| row.aggregate_row_id.as_str())
        .collect::<BTreeSet<_>>();
    if row_ids.len() != claimed.row_dispositions.len() {
        errors.push("BC-2 aggregate row IDs are not unique".to_owned());
    }
    if claimed.coverage.explicit_row_disposition_count.register != Bc2Register::ArtifactMetadata
        || claimed.coverage.named_gap_row_count.register != Bc2Register::ArtifactMetadata
        || claimed.coverage.silent_residue_count.register != Bc2Register::ArtifactMetadata
        || claimed.coverage.promoted_row_count.register != Bc2Register::ArtifactMetadata
        || claimed.row_dispositions.iter().any(|row| {
            row.parent_multiplicity.register != Bc2Register::StructuralTestimony
                || row.stratum_index.register != Bc2Register::SyntaxIdentifier
                || row.row_index.register != Bc2Register::SyntaxIdentifier
                || row.kappa.register != Bc2Register::StructuralTestimony
        })
    {
        errors.push("BC-2 numeric register audit failed".to_owned());
    }
    match build_certificate() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("BC-2 certificate differs from deterministic reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    Bc2Replay {
        valid: errors.is_empty(),
        errors,
        run_status: claimed.run_status,
        row_disposition_count: claimed.coverage.explicit_row_disposition_count.clone(),
        named_gap_row_count: claimed.coverage.named_gap_row_count.clone(),
        c8_closed: claimed.c8_closed,
        candidate_level_join_proved: claimed.candidate_level_join_proved,
        m4_authorized: claimed.m4_authorized,
    }
}

pub fn replay_bc2_boundary_provenance_join_v1_json(json: &str) -> Bc2Replay {
    match serde_json::from_str::<Bc2BoundaryProvenanceJoinV1Certificate>(json) {
        Ok(certificate) => replay_bc2_boundary_provenance_join_v1(&certificate),
        Err(error) => invalid_replay(format!("invalid BC-2 JSON: {error}")),
    }
}

fn register_label(register: Bc2Register) -> &'static str {
    match register {
        Bc2Register::SemanticRegisterAuthority => "semantic_register_authority",
        Bc2Register::SemanticFamilyAuthority => "semantic_family_authority",
        Bc2Register::StructuralTestimony => "structural_testimony",
        Bc2Register::ProofInventory => "proof_inventory",
        Bc2Register::ArtifactMetadata => "artifact_metadata",
        Bc2Register::SyntaxIdentifier => "syntax_identifier",
    }
}

fn render_value(value: &Bc2RegisteredValue) -> String {
    format!(
        "{} [register: `{}`]",
        value.value,
        register_label(value.register)
    )
}

pub fn render_bc2_boundary_provenance_join_v1(
    certificate: &Bc2BoundaryProvenanceJoinV1Certificate,
) -> String {
    let mut out = String::new();
    out.push_str("# BC-2 candidate-boundary provenance join result\n\n");
    out.push_str(&format!(
        "**Date (syntax identifier):** {}. **Outcome:** `stopped_named_gaps`. **Certificate:** `{}`.\n\n",
        certificate.date, certificate.result_digest
    ));
    out.push_str("BC-2 does not close C8. It authenticates the frozen M-3 baseline, the candidate/prefix-only authority, and M-1, then dispositions the exact frozen wrapped aggregate surface without promotion. It issues no cone-wide candidate claim; branch-specific candidate/prefix binding and same-act/same-prefix transport remain open obligations. Version labels, obligation labels, hashes, dates, and aggregate-row IDs are syntax identifiers; every quantitative value below names its register.\n\n");
    out.push_str("## Frozen baseline and authority\n\n");
    out.push_str(&format!(
        "M-3 v1 digest exact/self-valid: **{}/{}**. Its exact proved-condition inventory remains **{}**, its enacted E-5/BI-2 projections remain equal, and the registered E-5 drift is disclosed without repair or concealment. Archive comparators are authority: **false**.\n\n",
        certificate.m3_baseline.result_digest_exact,
        certificate.m3_baseline.self_digest_valid,
        render_value(&metadata(
            certificate
                .m3_baseline
                .exact_predecessor_condition_ordinals
                .len(),
            "proved M-3 v1 baseline conditions"
        )),
    ));
    out.push_str(&format!(
        "M-1 maps consumed: **{}**; order obstructions preserved: **{}**. Every map retains a typed total inverse and identity/composition coherence; no obstruction pair is merged and no new equivalence semantics is assumed.\n\n",
        render_value(&certificate.m1_transport.exported_map_count),
        render_value(&certificate.m1_transport.order_obstruction_count),
    ));
    out.push_str("## Source-first audit\n\n");
    out.push_str(
        &certificate
            .source_first_factorization_audit
            .exact_obstruction,
    );
    out.push_str("\n\n");
    out.push_str(&format!(
        "Exact missing theorem: `{}`. Source-first symbolic join proved: **{}**.\n\n",
        certificate
            .source_first_factorization_audit
            .exact_missing_theorem_id,
        certificate
            .source_first_factorization_audit
            .source_first_symbolic_join_proved,
    ));
    out.push_str("## Exhaustive row disposition\n\n");
    out.push_str(&format!(
        "Schema-3 strata: **{}**. Aggregate rows: **{}**. Explicit dispositions: **{}**. Act-locally resolved rows: **{}**. Named-gap rows: **{}**. Silent residue: **{}**. Promotions: **{}**.\n\n",
        render_value(&certificate.coverage.schema3_stratum_count),
        render_value(&certificate.coverage.schema3_row_count),
        render_value(&certificate.coverage.explicit_row_disposition_count),
        render_value(&certificate.coverage.act_local_resolved_row_count),
        render_value(&certificate.coverage.named_gap_row_count),
        render_value(&certificate.coverage.silent_residue_count),
        render_value(&certificate.coverage.promoted_row_count),
    ));
    out.push_str("Every certificate row exposes the shared identity `schema3:S{stratum}/R{row}:{schema3_row_digest}` and the exact Schema-4/5 comparator hashes. Each row is named `BC2_SCHEMA3_AGGREGATE_KEY_NOT_PROVED_PROVENANCE_CONGRUENT`; none issues a candidate-local token.\n\n");
    out.push_str("## Permitted conclusion\n\n");
    out.push_str(&certificate.permitted_conclusion);
    out.push_str("\n\n## Required successor\n\n");
    out.push_str(&certificate.required_successor_action);
    out.push('\n');
    out
}

pub fn emit_bc2_boundary_provenance_join_v1_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Bc2BoundaryProvenanceJoinV1Certificate, Bc2Error> {
    if certificate_path.exists() || report_path.exists() {
        return Err(Bc2Error::Io(
            "create-new target already exists; no artifact was overwritten".to_owned(),
        ));
    }
    let certificate = issue_bc2_boundary_provenance_join_v1()?;
    let replay = replay_bc2_boundary_provenance_join_v1(&certificate);
    if !replay.valid {
        return Err(Bc2Error::Invariant(format!(
            "new BC-2 certificate did not replay: {}",
            replay.errors.join("; ")
        )));
    }
    let json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Bc2Error::Json(error.to_string()))?;
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(certificate_path)
        .map_err(|error| Bc2Error::Io(error.to_string()))?;
    let certificate_write = certificate_file
        .write_all(&json)
        .and_then(|_| certificate_file.write_all(b"\n"));
    drop(certificate_file);
    if let Err(error) = certificate_write {
        let _ = remove_file(certificate_path);
        return Err(Bc2Error::Io(error.to_string()));
    }
    let report = render_bc2_boundary_provenance_join_v1(&certificate);
    let report_result = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(report_path)
        .and_then(|mut file| file.write_all(report.as_bytes()));
    if let Err(error) = report_result {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(Bc2Error::Io(error.to_string()));
    }
    let emitted_json = match read_to_string(certificate_path) {
        Ok(json) => json,
        Err(error) => {
            let _ = remove_file(certificate_path);
            let _ = remove_file(report_path);
            return Err(Bc2Error::Io(error.to_string()));
        }
    };
    let emitted_replay = replay_bc2_boundary_provenance_join_v1_json(&emitted_json);
    if !emitted_replay.valid {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(Bc2Error::Invariant(format!(
            "emitted BC-2 JSON did not replay: {}",
            emitted_replay.errors.join("; ")
        )));
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::{
        BC2_FACTORIZATION_GAP, BC2_GLOBAL_GAP, BC2_ROW_GAP, Bc2Quantifier, Bc2Register,
        Bc2RowDispositionStatus, Bc2RunStatus, issue_bc2_boundary_provenance_join_v1,
        render_bc2_boundary_provenance_join_v1, replay_bc2_boundary_provenance_join_v1,
        replay_bc2_boundary_provenance_join_v1_json,
    };
    use std::collections::BTreeSet;

    #[test]
    fn issues_exact_exhaustive_fail_closed_certificate() {
        let certificate = issue_bc2_boundary_provenance_join_v1().expect("BC-2 issues");
        let replay = replay_bc2_boundary_provenance_join_v1(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(certificate.run_status, Bc2RunStatus::StoppedNamedGaps);
        assert_eq!(certificate.row_dispositions.len(), 213);
        assert_eq!(certificate.coverage.named_gap_row_count.value, "213");
        assert_eq!(certificate.coverage.silent_residue_count.value, "0");
        assert_eq!(certificate.coverage.promoted_row_count.value, "0");
        assert!(!certificate.c8_closed);
        assert!(!certificate.candidate_level_join_proved);
        assert!(!certificate.bridge_claim_issued);
        assert!(!certificate.m4_authorized);
        assert_eq!(
            certificate.coverage.quantifier,
            Bc2Quantifier::FrozenWrappedSurface
        );
        assert!(
            !certificate
                .coverage
                .branch_specific_candidate_prefixes_bound
        );
        assert!(!certificate.coverage.cone_level_candidate_claim_issued);
        assert!(
            certificate
                .coverage
                .same_act_same_prefix_cross_branch_transport_is_open_obligation
        );
        assert!(
            certificate
                .row_dispositions
                .iter()
                .all(|row| row.disposition == Bc2RowDispositionStatus::NamedGap
                    && row.named_gap == BC2_ROW_GAP
                    && !row.promoted)
        );
        let identities = certificate
            .row_dispositions
            .iter()
            .map(|row| row.aggregate_row_id.as_str())
            .collect::<BTreeSet<_>>();
        assert_eq!(identities.len(), 213);
        assert!(
            certificate.row_dispositions[0]
                .aggregate_row_id
                .starts_with("schema3:S0/R0:blake3:")
        );
    }

    #[test]
    fn source_first_archive_transport_and_register_boundaries_are_exact() {
        let certificate = issue_bc2_boundary_provenance_join_v1().expect("BC-2 issues");
        assert_eq!(
            certificate.global_named_gaps,
            [BC2_GLOBAL_GAP, BC2_FACTORIZATION_GAP]
        );
        assert!(
            certificate
                .source_first_factorization_audit
                .source_first_route_audited
        );
        assert!(
            !certificate
                .source_first_factorization_audit
                .provenance_factorization_through_ip1_dp_key_proved
        );
        assert!(
            !certificate
                .source_first_factorization_audit
                .source_first_symbolic_join_proved
        );
        assert!(certificate.archive_comparators_are_testimony_only);
        assert!(
            certificate
                .row_dispositions
                .iter()
                .all(|row| row.archive_comparator_matched && !row.archive_comparator_is_authority)
        );
        assert_eq!(certificate.m1_transport.maps.len(), 5);
        assert_eq!(certificate.m1_transport.order_obstructions.len(), 2);
        assert!(
            certificate
                .m1_transport
                .order_obstructions
                .iter()
                .all(|row| row.kept_separate_by_bc2)
        );
        assert!(certificate.row_dispositions.iter().all(|row| {
            row.parent_multiplicity.register == Bc2Register::StructuralTestimony
                && row.stratum_index.register == Bc2Register::SyntaxIdentifier
                && row.row_index.register == Bc2Register::SyntaxIdentifier
                && row.kappa.register == Bc2Register::StructuralTestimony
        }));
        let report = render_bc2_boundary_provenance_join_v1(&certificate);
        assert!(report.contains("213 [register: `artifact_metadata`]"));
        assert!(report.contains("0 [register: `artifact_metadata`]"));
        assert!(report.contains(BC2_FACTORIZATION_GAP));
    }

    #[test]
    fn registered_mutations_fail_replay() {
        let certificate = issue_bc2_boundary_provenance_join_v1().expect("BC-2 issues");
        let mut mutations = Vec::new();

        let mut baseline = certificate.clone();
        baseline.m3_baseline.result_digest_exact = false;
        mutations.push(baseline);

        let mut archive_authority = certificate.clone();
        archive_authority.row_dispositions[0].archive_comparator_is_authority = true;
        mutations.push(archive_authority);

        let mut row_id = certificate.clone();
        row_id.row_dispositions[0]
            .aggregate_row_id
            .push_str(":mutated");
        mutations.push(row_id);

        let mut reorder = certificate.clone();
        reorder.row_dispositions.swap(0, 1);
        mutations.push(reorder);

        let mut gap = certificate.clone();
        gap.row_dispositions[0].named_gap.clear();
        mutations.push(gap);

        let mut transport = certificate.clone();
        transport.m1_transport.order_obstructions[0].kept_separate_by_bc2 = false;
        mutations.push(transport);

        let mut factorization = certificate.clone();
        factorization
            .source_first_factorization_audit
            .source_first_symbolic_join_proved = true;
        mutations.push(factorization);

        let mut register = certificate.clone();
        register.row_dispositions[0].parent_multiplicity.register =
            Bc2Register::SemanticFamilyAuthority;
        mutations.push(register);

        let mut selector = certificate.clone();
        selector.desired_verdict_count_bar_hash_or_enumeration_order_used_as_selector = true;
        mutations.push(selector);

        let mut promotion = certificate.clone();
        promotion.row_dispositions[0].promoted = true;
        mutations.push(promotion);

        let mut authorization = certificate.clone();
        authorization.m4_authorized = true;
        mutations.push(authorization);

        let mut digest = certificate;
        digest.result_digest.push('0');
        mutations.push(digest);

        for mutation in mutations {
            assert!(!replay_bc2_boundary_provenance_join_v1(&mutation).valid);
        }
    }

    #[test]
    fn unknown_json_fields_fail_closed() {
        let certificate = issue_bc2_boundary_provenance_join_v1().expect("BC-2 issues");
        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("object")
            .insert("unknown_field".to_owned(), serde_json::Value::Bool(true));
        let replay = replay_bc2_boundary_provenance_join_v1_json(
            &serde_json::to_string(&value).expect("json"),
        );
        assert!(!replay.valid);
    }
}
