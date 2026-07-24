//! T-D2-3 successor: lossless candidate-content stream for the frozen IP-1
//! aggregate surface.
//!
//! This module is deliberately additive.  It does not change the IP-1
//! aggregate program and it does not revise the frozen T-D2-3 v1
//! candidate/clause key schema.  Instead it:
//!
//! 1. regenerates the raw per-position expression catalogs from the frozen
//!    enumerator contract;
//! 2. gives their Cartesian products a checked mixed-radix rank/unrank map;
//! 3. independently replays the IP-1 expression-class fold and requires its
//!    complete symbolic row histogram to equal the public aggregate result;
//! 4. materializes any requested candidate by ordinal; and
//! 5. feeds that exact telescope to the public v1 key issuer.
//!
//! The full surface is never collected as telescopes.  Consumers request
//! bounded chunks, so even a very large finite product remains a genuine
//! content stream rather than an aggregate-count proxy.

use crate::enumerate::{
    EnumerationContext, LateFamilySurface, assess_raw_surface_membership, enumerate_exprs,
};
use crate::ip1_certification_boundary::candidate_join::{
    AmplificationRoute, CandidateJoinRow, ClauseLocalExtractionProxy, Ip1CandidateJoinCertificate,
    OperationalA5Verdict, run_ip1_candidate_join_with_caps,
};
use crate::t_d2_3_candidate_provenance_key_schema_v1::{
    Td23CandidateKeySet, issue_t_d2_3_candidate_keys,
    issue_t_d2_3_candidate_provenance_key_schema_v1, replay_t_d2_3_candidate_keys,
    replay_t_d2_3_candidate_provenance_key_schema_v1,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::{Telescope, TelescopeClass};
use pen_eval::typed_families::{
    ClauseClosureDisposition, PredecessorClosure, clause_presentation, closure_clause_disposition,
    predecessor_closure,
};
use pen_type::elaborate::{
    ElabError, SealedSignature, candidate_hash, elaborate_single_clause,
    minimal_ambient_parameters, required_clause_ambient,
};
use pen_type::normalize::normalize;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, HashMap};
use std::ops::Range;
use thiserror::Error;

pub const T_D2_3_CANDIDATE_CONTENT_STREAM_V2: &str = "t-d2-3-candidate-content-stream-v2";
pub const T_D2_3_CANDIDATE_CONTENT_STREAM_V2_DATE: &str = "2026-07-24";
pub const T_D2_3_STREAM_MIN_KAPPA: u16 = 2;
pub const T_D2_3_STREAM_MAX_KAPPA: u16 = 4;
pub const T_D2_3_STREAM_MAX_EXPR_NODES: u8 = 6;
pub const T_D2_3_STREAM_VISIBLE_LIBRARY: u32 = 15;

pub const T_D2_3_V2_CATALOG_DUPLICATE_GAP: &str = "TD23V2_POSITION_CATALOG_IS_NOT_CONTENT_UNIQUE";
pub const T_D2_3_V2_CATALOG_DIGEST_COLLISION_GAP: &str =
    "TD23V2_POSITION_CATALOG_CONTENT_DIGEST_COLLISION";
pub const T_D2_3_V2_PRODUCT_OVERFLOW_GAP: &str = "TD23V2_MIXED_RADIX_PRODUCT_EXCEEDS_U128";
pub const T_D2_3_V2_AGGREGATE_WIDTH_GAP: &str =
    "TD23V2_STREAM_WIDTHS_DO_NOT_FACTOR_AGGREGATE_RAW_PRODUCT";
pub const T_D2_3_V2_AGGREGATE_CONTEXT_GAP: &str =
    "TD23V2_LOCAL_STREAM_CONTEXT_DOES_NOT_EQUAL_FROZEN_IP1_CONTEXT";
pub const T_D2_3_V2_ENUMERATOR_ORDER_GAP: &str = "TD23V2_ENUMERATOR_CANONICAL_ORDER_CONTRACT_DRIFT";
pub const T_D2_3_V2_AGGREGATE_TOTAL_GAP: &str =
    "TD23V2_STREAM_TOTAL_DOES_NOT_EQUAL_AGGREGATE_RAW_TOTAL";
pub const T_D2_3_V2_ROW_FACTORIZATION_GAP: &str =
    "TD23V2_SYMBOLIC_ROW_HISTOGRAM_DOES_NOT_EQUAL_AGGREGATE_QUOTIENT";
pub const T_D2_3_V2_V1_SCHEMA_GAP: &str = "TD23V2_FROZEN_V1_KEY_SCHEMA_DID_NOT_REPLAY";
pub const T_D2_3_V2_V1_ADAPTER_GAP: &str = "TD23V2_STREAM_TO_V1_KEY_ADAPTER_DID_NOT_REPLAY";
pub const T_D2_3_V2_PROVENANCE_FACTORIZATION_GAP: &str =
    "TD23V2_CANDIDATE_BOUND_V1_KEYS_NOT_PROVED_PROVENANCE_CONGRUENT_ON_SYMBOLIC_ROWS";

const STEP16_BASE_AMBIENT: u32 = 2;
const MAX_AMBIENT: u32 = 2;

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/depth_two_domain_adjudication.md");
const ENUMERATOR_SOURCE_BYTES: &[u8] = include_bytes!("enumerate.rs");
const AGGREGATE_SOURCE_BYTES: &[u8] = include_bytes!("ip1_candidate_join.rs");
const V1_KEY_SOURCE_BYTES: &[u8] = include_bytes!("t_d2_3_candidate_provenance_key_schema_v1.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_d2_3_candidate_content_stream_v2.rs");

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td23V2Register {
    KernelTypingAuthority,
    StructuralProof,
    AggregateRegression,
    ArtifactMetadata,
    SyntaxIdentifier,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td23V2RunStatus {
    CompleteGateUnchanged,
    BoundedRegressionOnlyGateUnchanged,
    StoppedNamedGaps,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23V2RegisteredNumber {
    pub decimal: String,
    pub register: Td23V2Register,
    pub meaning: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23V2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: Td23V2RegisteredNumber,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23V2CatalogProof {
    pub position: Td23V2RegisteredNumber,
    pub scope_size: Td23V2RegisteredNumber,
    pub width: Td23V2RegisteredNumber,
    pub content_unique: bool,
    pub content_digest_collision_absent: bool,
    pub enumerator_order_is_canonical: bool,
    pub catalog_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23V2AggregateContextAudit {
    pub aggregate_position_context_section_digest: String,
    pub local_context_projection_digest: String,
    pub enumerator_canonical_order_contract_exact: bool,
    pub aggregate_uses_same_enumerate_exprs_api: bool,
    pub aggregate_context_field_contract_exact: bool,
    pub local_context_values_exact: bool,
    pub same_generator_same_arguments_proved: bool,
    pub raw_catalog_content_identity_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23V2DownstreamProvenanceBoundary {
    pub raw_candidate_totals: Vec<Td23V2RegisteredNumber>,
    pub exhaustive_candidate_key_issuance_performed: bool,
    pub frozen_v1_candidate_keys_are_content_specific: bool,
    pub aggregate_rows_proved_sufficient_for_candidate_provenance: bool,
    pub finite_provenance_congruence_theorem_proved: bool,
    pub bc2_candidate_level_join_prerequisite_satisfied: bool,
    pub named_boundary: String,
    pub exact_obstruction: String,
    pub required_successor_theorem: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23V2StratumFactorization {
    pub kappa: Td23V2RegisteredNumber,
    pub position_widths: Vec<Td23V2RegisteredNumber>,
    pub stream_raw_total: Td23V2RegisteredNumber,
    pub aggregate_raw_total: Td23V2RegisteredNumber,
    pub aggregate_joined_total: Td23V2RegisteredNumber,
    pub aggregate_row_count: Td23V2RegisteredNumber,
    pub mixed_radix_product_exact: bool,
    pub rank_unrank_bijection_proved: bool,
    pub aggregate_position_widths_exact: bool,
    pub aggregate_raw_total_exact: bool,
    pub aggregate_joined_total_exact: bool,
    pub symbolic_row_histogram_exact: bool,
    pub boundary_key_adapter_replay_exact: bool,
    pub boundary_candidate_digests: Vec<String>,
    pub factorization_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23V2KeyAdapterProof {
    pub frozen_v1_schema: String,
    pub frozen_v1_schema_digest: String,
    pub v1_schema_certificate_replayed: bool,
    pub v1_schema_fixed_before_join: bool,
    pub v1_schema_replayed_before_aggregate_outcome: bool,
    pub v1_schema_fields_redeclared_by_v2: bool,
    pub v1_issuer_called_on_materialized_telescope: bool,
    pub typed_and_kernel_failure_results_both_preserved: bool,
    pub adapter_is_total_at_rust_type: bool,
    pub join_outcome_read: bool,
    pub selector_value_or_bar_read: bool,
    pub proof_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23V2GateRecord {
    pub exact_scope: String,
    pub candidate_level_join_executed: bool,
    pub bridge_outcome_read: bool,
    pub t_d2_1_or_t_d2_2_outcome_read: bool,
    pub m3_v1_remains_authoritative: bool,
    pub m3_successor_issued: bool,
    pub m4_authorized: bool,
    pub gate_evaluated: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23CandidateContentStreamV2Certificate {
    pub schema: String,
    pub date: String,
    pub exact_scope: String,
    pub min_kappa: Td23V2RegisteredNumber,
    pub max_kappa: Td23V2RegisteredNumber,
    pub max_expr_nodes: Td23V2RegisteredNumber,
    pub visible_library: Td23V2RegisteredNumber,
    pub signature_digest: String,
    pub aggregate_certificate_digest: String,
    pub source_bindings: Vec<Td23V2SourceBinding>,
    pub aggregate_context_audit: Td23V2AggregateContextAudit,
    pub catalog_proofs: Vec<Td23V2CatalogProof>,
    pub strata: Vec<Td23V2StratumFactorization>,
    pub key_adapter: Td23V2KeyAdapterProof,
    pub downstream_provenance_boundary: Td23V2DownstreamProvenanceBoundary,
    pub stream_is_lazy: bool,
    pub complete_candidate_telescopes_materialized_at_once: bool,
    pub mixed_radix_rank_and_unrank_available: bool,
    pub bounded_chunk_iterator_available: bool,
    pub every_materialized_item_carries_candidate_content: bool,
    pub every_materialized_item_carries_aggregate_row_locator: bool,
    pub every_materialized_item_is_fed_to_frozen_v1_key_issuer: bool,
    pub aggregate_rows_used_to_define_catalogs_or_ordinals: bool,
    pub aggregate_rows_read_only_as_factorization_regression: bool,
    pub candidate_or_key_content_filtered_by_aggregate_row: bool,
    pub full_aggregate_candidate_content_stream_exported: bool,
    pub aggregate_surface_factorization_proved: bool,
    pub canonical_full_surface_caps: bool,
    pub bounded_surface_regression_complete: bool,
    pub open_gaps: Vec<String>,
    pub status: Td23V2RunStatus,
    pub gate: Td23V2GateRecord,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23CandidateContentItemV2 {
    pub schema: String,
    pub kappa: Td23V2RegisteredNumber,
    pub ordinal: Td23V2RegisteredNumber,
    pub catalog_indices: Vec<Td23V2RegisteredNumber>,
    pub candidate: Telescope,
    pub candidate_digest: String,
    pub raw_surface_member: bool,
    pub raw_surface_rejections: Vec<String>,
    pub aggregate_row_index: Td23V2RegisteredNumber,
    pub aggregate_row: CandidateJoinRow,
    pub aggregate_row_digest: String,
    pub frozen_v1_key_set: Td23CandidateKeySet,
    pub frozen_v1_key_replay_errors: Vec<String>,
    pub content_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23CandidateContentStreamV2Replay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub status: Td23V2RunStatus,
    pub full_stream_exported: bool,
    pub aggregate_surface_factorization_proved: bool,
    pub m3_v1_remains_authoritative: bool,
    pub m4_authorized: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Td23CandidateContentStreamV2Error {
    #[error("T-D2-3 v2 input failure: {0}")]
    Input(String),
    #[error("T-D2-3 v2 arithmetic failure: {0}")]
    Arithmetic(String),
    #[error("T-D2-3 v2 invariant failure: {0}")]
    Invariant(String),
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct AmbientFactsV2 {
    ambient: u32,
    used_fields: Vec<u16>,
    marginal_by_assignment: Vec<bool>,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ExprFactorV2 {
    formation: bool,
    shortfall: u32,
    per_ambient: Vec<AmbientFactsV2>,
    invalid_named: bool,
    unclassified: bool,
    basic_formation: bool,
    top_path: bool,
    top_modal: bool,
    temporal_like: bool,
    top_suspension: bool,
    has_lib: bool,
    former_root: bool,
    support_mask: u8,
    modal_kind: u8,
    path_basis: u8,
    synthesis_sites: u8,
    app14: bool,
    app15: bool,
    potential_typed_app14: bool,
    potential_typed_app15: bool,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DpStateV2 {
    formation_bits: u8,
    max_shortfall: u32,
    any_marginal: bool,
    any_invalid: bool,
    any_unclassified: bool,
    all_basic: bool,
    any_path: bool,
    any_modal: bool,
    any_temporal_like: bool,
    any_suspension: bool,
    first_two_have_lib: bool,
    any_lib: bool,
    all_former_root: bool,
    support_mask: u8,
    modal_kind_mask: u8,
    path_basis: u8,
    synthesis_sites: u8,
    app14: bool,
    app15: bool,
    potential_typed_app14: bool,
    potential_typed_app15: bool,
}

impl DpStateV2 {
    fn initial() -> Self {
        Self {
            formation_bits: 0,
            max_shortfall: 0,
            any_marginal: false,
            any_invalid: false,
            any_unclassified: false,
            all_basic: true,
            any_path: false,
            any_modal: false,
            any_temporal_like: false,
            any_suspension: false,
            first_two_have_lib: true,
            any_lib: false,
            all_former_root: true,
            support_mask: 0,
            modal_kind_mask: 0,
            path_basis: 0,
            synthesis_sites: 0,
            app14: false,
            app15: false,
            potential_typed_app14: false,
            potential_typed_app15: false,
        }
    }
}

struct PositionCatalogV2 {
    content: SerializedExpressionCatalogV2,
    factor_histogram: BTreeMap<ExprFactorV2, u128>,
}

/// Compact, lossless storage for one expression catalog.  Each record is
/// encoded as an eight-byte little-endian length followed by the exact
/// canonical serde JSON payload.  The length prefixes make the concatenation
/// unambiguous; offsets permit O(1) ordinal lookup without retaining millions
/// of recursively boxed `Expr` values.
struct SerializedExpressionCatalogV2 {
    records: Vec<u8>,
    offsets: Vec<u64>,
}

impl SerializedExpressionCatalogV2 {
    fn from_expressions(expressions: &[Expr]) -> (Self, bool, bool) {
        let mut records = Vec::new();
        let mut offsets = Vec::with_capacity(expressions.len());
        let mut first_index_by_digest = HashMap::<[u8; 32], usize>::new();
        let mut content_unique = true;
        let mut content_digest_collision_absent = true;
        for expression in expressions {
            let payload =
                serde_json::to_vec(expression).expect("raw expression serializes canonically");
            let digest: [u8; 32] = Sha256::digest(&payload).into();
            if let Some(prior_index) = first_index_by_digest.get(&digest).copied() {
                match serialized_payload_at(&records, &offsets, prior_index) {
                    Some(prior_payload) if prior_payload == payload.as_slice() => {
                        content_unique = false;
                    }
                    Some(_) => {
                        // Digest equality is never treated as content
                        // identity. A collision fails the proof closed.
                        content_digest_collision_absent = false;
                    }
                    None => {
                        content_unique = false;
                        content_digest_collision_absent = false;
                    }
                }
            } else {
                first_index_by_digest.insert(digest, offsets.len());
            }
            offsets.push(records.len() as u64);
            records.extend_from_slice(&(payload.len() as u64).to_le_bytes());
            records.extend_from_slice(&payload);
        }
        (
            Self { records, offsets },
            content_unique,
            content_digest_collision_absent,
        )
    }

    fn len(&self) -> usize {
        self.offsets.len()
    }

    fn payload_at(&self, index: usize) -> Result<&[u8], Td23CandidateContentStreamV2Error> {
        self.offsets.get(index).ok_or_else(|| {
            Td23CandidateContentStreamV2Error::Input(format!(
                "expression-catalog index {index} is out of range"
            ))
        })?;
        serialized_payload_at(&self.records, &self.offsets, index).ok_or_else(|| {
            Td23CandidateContentStreamV2Error::Invariant(
                "expression-record length prefix or payload is invalid".to_owned(),
            )
        })
    }

    fn expression_at(&self, index: usize) -> Result<Expr, Td23CandidateContentStreamV2Error> {
        serde_json::from_slice(self.payload_at(index)?).map_err(|error| {
            Td23CandidateContentStreamV2Error::Invariant(format!(
                "stored expression payload does not deserialize: {error}"
            ))
        })
    }

    fn index_of(&self, expression: &Expr) -> Option<usize> {
        let needle =
            serde_json::to_vec(expression).expect("candidate expression serializes canonically");
        (0..self.len()).find(|index| {
            self.payload_at(*index)
                .is_ok_and(|payload| payload == needle.as_slice())
        })
    }

    fn digest(&self) -> String {
        bytes_hash(&self.records)
    }
}

fn serialized_payload_at<'a>(records: &'a [u8], offsets: &[u64], index: usize) -> Option<&'a [u8]> {
    let start = usize::try_from(*offsets.get(index)?).ok()?;
    let length_end = start.checked_add(8)?;
    let prefix: [u8; 8] = records.get(start..length_end)?.try_into().ok()?;
    let length = usize::try_from(u64::from_le_bytes(prefix)).ok()?;
    let end = length_end.checked_add(length)?;
    records.get(length_end..end)
}

/// Runtime content-stream handle.  Catalogs are materialized once; telescope
/// products are not.  The handle intentionally is not serializable.
pub struct Td23CandidateContentStreamV2 {
    signature: SealedSignature,
    closure: PredecessorClosure,
    domains14: Vec<Expr>,
    domains15: Vec<Expr>,
    min_kappa: u16,
    max_kappa: u16,
    max_expr_nodes: u8,
    catalogs: Vec<PositionCatalogV2>,
    aggregate: Ip1CandidateJoinCertificate,
    certificate: Td23CandidateContentStreamV2Certificate,
}

pub struct Td23CandidateContentIterV2<'a> {
    stream: &'a Td23CandidateContentStreamV2,
    kappa: u16,
    next: u128,
    end: u128,
}

impl Iterator for Td23CandidateContentIterV2<'_> {
    type Item = Result<Td23CandidateContentItemV2, Td23CandidateContentStreamV2Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next >= self.end {
            return None;
        }
        let ordinal = self.next;
        self.next += 1;
        Some(self.stream.item_at(self.kappa, ordinal))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.next);
        match usize::try_from(remaining) {
            Ok(exact) => (exact, Some(exact)),
            Err(_) => (usize::MAX, None),
        }
    }
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_D2_3_CANDIDATE_CONTENT_STREAM_V2, domain, value))
        .expect("T-D2-3 v2 evidence serializes");
    bytes_hash(&bytes)
}

fn registered_number(
    value: impl ToString,
    register: Td23V2Register,
    meaning: &str,
) -> Td23V2RegisteredNumber {
    Td23V2RegisteredNumber {
        decimal: value.to_string(),
        register,
        meaning: meaning.to_owned(),
    }
}

fn metadata_number(value: impl ToString, meaning: &str) -> Td23V2RegisteredNumber {
    registered_number(value, Td23V2Register::ArtifactMetadata, meaning)
}

fn syntax_number(value: impl ToString, meaning: &str) -> Td23V2RegisteredNumber {
    registered_number(value, Td23V2Register::SyntaxIdentifier, meaning)
}

fn structural_number(value: impl ToString, meaning: &str) -> Td23V2RegisteredNumber {
    registered_number(value, Td23V2Register::StructuralProof, meaning)
}

fn aggregate_number(value: impl ToString, meaning: &str) -> Td23V2RegisteredNumber {
    registered_number(value, Td23V2Register::AggregateRegression, meaning)
}

fn kernel_number(value: impl ToString, meaning: &str) -> Td23V2RegisteredNumber {
    registered_number(value, Td23V2Register::KernelTypingAuthority, meaning)
}

fn source_bindings() -> Vec<Td23V2SourceBinding> {
    [
        (
            "docs/depth_two_domain_adjudication.md",
            "adopted outcome-blind T-D2-3 construction obligation",
            ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-search/src/enumerate.rs",
            "independent raw expression catalog authority",
            ENUMERATOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/ip1_candidate_join.rs",
            "frozen aggregate quotient target; never a candidate-content source",
            AGGREGATE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_d2_3_candidate_provenance_key_schema_v1.rs",
            "already-frozen candidate/clause key issuer",
            V1_KEY_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_d2_3_candidate_content_stream_v2.rs",
            "lossless rank/unrank, factorization replay, and v1 adapter",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Td23V2SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: metadata_number(bytes.len(), "exact bound source byte length"),
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn position_context(position: u32, max_expr_nodes: u8) -> EnumerationContext {
    EnumerationContext {
        library_size: T_D2_3_STREAM_VISIBLE_LIBRARY,
        scope_size: STEP16_BASE_AMBIENT + position,
        max_path_dimension: 1,
        include_trunc: false,
        include_modal: true,
        include_temporal: true,
        include_linear_exponential: false,
        max_expr_nodes,
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct EnumerationContextProjectionV2 {
    library_size: u32,
    scope_size: u32,
    max_path_dimension: u32,
    include_trunc: bool,
    include_modal: bool,
    include_temporal: bool,
    include_linear_exponential: bool,
    max_expr_nodes: u8,
    require_former_eliminator_clauses: bool,
    require_initial_hit_clauses: bool,
    require_truncation_hit_clauses: bool,
    require_higher_hit_clauses: bool,
    require_sphere_lift_clauses: bool,
    require_axiomatic_bundle_clauses: bool,
    require_modal_shell_clauses: bool,
    require_connection_shell_clauses: bool,
    require_curvature_shell_clauses: bool,
    require_operator_bundle_clauses: bool,
    require_hilbert_functional_clauses: bool,
    require_temporal_shell_clauses: bool,
    historical_anchor_ref: Option<u32>,
    late_family_surface: LateFamilySurface,
}

fn project_context(context: EnumerationContext) -> EnumerationContextProjectionV2 {
    EnumerationContextProjectionV2 {
        library_size: context.library_size,
        scope_size: context.scope_size,
        max_path_dimension: context.max_path_dimension,
        include_trunc: context.include_trunc,
        include_modal: context.include_modal,
        include_temporal: context.include_temporal,
        include_linear_exponential: context.include_linear_exponential,
        max_expr_nodes: context.max_expr_nodes,
        require_former_eliminator_clauses: context.require_former_eliminator_clauses,
        require_initial_hit_clauses: context.require_initial_hit_clauses,
        require_truncation_hit_clauses: context.require_truncation_hit_clauses,
        require_higher_hit_clauses: context.require_higher_hit_clauses,
        require_sphere_lift_clauses: context.require_sphere_lift_clauses,
        require_axiomatic_bundle_clauses: context.require_axiomatic_bundle_clauses,
        require_modal_shell_clauses: context.require_modal_shell_clauses,
        require_connection_shell_clauses: context.require_connection_shell_clauses,
        require_curvature_shell_clauses: context.require_curvature_shell_clauses,
        require_operator_bundle_clauses: context.require_operator_bundle_clauses,
        require_hilbert_functional_clauses: context.require_hilbert_functional_clauses,
        require_temporal_shell_clauses: context.require_temporal_shell_clauses,
        historical_anchor_ref: context.historical_anchor_ref,
        late_family_surface: context.late_family_surface,
    }
}

fn literal_expected_context(position: u32, max_expr_nodes: u8) -> EnumerationContextProjectionV2 {
    project_context(EnumerationContext {
        library_size: 15,
        scope_size: 2 + position,
        max_path_dimension: 1,
        include_trunc: false,
        include_modal: true,
        include_temporal: true,
        include_linear_exponential: false,
        max_expr_nodes,
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
    })
}

fn audit_aggregate_context_contract(
    max_kappa: u16,
    max_expr_nodes: u8,
) -> Td23V2AggregateContextAudit {
    let aggregate_source = std::str::from_utf8(AGGREGATE_SOURCE_BYTES).unwrap_or_default();
    let section_start = aggregate_source
        .find("fn position_context(position: u32, max_expr_nodes: u8) -> EnumerationContext {");
    let section = section_start
        .and_then(|start| {
            aggregate_source[start..]
                .find("\n#[cfg(test)]\nfn primary_role")
                .map(|length| &aggregate_source[start..start + length])
        })
        .unwrap_or_default();
    let required_field_contract = [
        "library_size: STEP16_VISIBLE_LIBRARY",
        "scope_size: STEP16_BASE_AMBIENT + position",
        "max_path_dimension: 1",
        "include_trunc: false",
        "include_modal: true",
        "include_temporal: true",
        "include_linear_exponential: false",
        "max_expr_nodes,",
        "require_former_eliminator_clauses: false",
        "require_initial_hit_clauses: false",
        "require_truncation_hit_clauses: false",
        "require_higher_hit_clauses: false",
        "require_sphere_lift_clauses: false",
        "require_axiomatic_bundle_clauses: false",
        "require_modal_shell_clauses: false",
        "require_connection_shell_clauses: false",
        "require_curvature_shell_clauses: false",
        "require_operator_bundle_clauses: false",
        "require_hilbert_functional_clauses: false",
        "require_temporal_shell_clauses: false",
        "historical_anchor_ref: None",
        "late_family_surface: LateFamilySurface::None",
    ];
    let aggregate_context_field_contract_exact = !section.is_empty()
        && aggregate_source.contains("const STEP16_VISIBLE_LIBRARY: u32 = 15;")
        && aggregate_source.contains("const STEP16_BASE_AMBIENT: u32 = 2;")
        && required_field_contract
            .iter()
            .all(|required| section.contains(required));
    let aggregate_uses_same_enumerate_exprs_api = aggregate_source
        .contains("let exprs = enumerate_exprs(position_context(position, max_expr_nodes));");

    let enumerator_source = std::str::from_utf8(ENUMERATOR_SOURCE_BYTES).unwrap_or_default();
    let enumerator_canonical_order_contract_exact = enumerator_source
        .contains("pub fn enumerate_exprs(context: EnumerationContext) -> Vec<Expr>")
        && enumerator_source.contains("unique_sorted_exprs(all)")
        && enumerator_source.contains("fn unique_sorted_exprs(exprs: Vec<Expr>) -> Vec<Expr>")
        && enumerator_source.contains("keyed.entry(expr_sort_key(&expr)).or_insert(expr);")
        && enumerator_source.contains("keyed.into_values().collect()");

    let local_contexts = (0..u32::from(max_kappa))
        .map(|position| project_context(position_context(position, max_expr_nodes)))
        .collect::<Vec<_>>();
    let expected_contexts = (0..u32::from(max_kappa))
        .map(|position| literal_expected_context(position, max_expr_nodes))
        .collect::<Vec<_>>();
    let local_context_values_exact = local_contexts == expected_contexts;
    let same_generator_same_arguments_proved = enumerator_canonical_order_contract_exact
        && aggregate_uses_same_enumerate_exprs_api
        && aggregate_context_field_contract_exact
        && local_context_values_exact;
    let raw_catalog_content_identity_proved = same_generator_same_arguments_proved;
    let mut audit = Td23V2AggregateContextAudit {
        aggregate_position_context_section_digest: bytes_hash(section.as_bytes()),
        local_context_projection_digest: tagged_hash(
            "local-enumeration-context-projections",
            &local_contexts,
        ),
        enumerator_canonical_order_contract_exact,
        aggregate_uses_same_enumerate_exprs_api,
        aggregate_context_field_contract_exact,
        local_context_values_exact,
        same_generator_same_arguments_proved,
        raw_catalog_content_identity_proved,
        derivation_hash: String::new(),
    };
    audit.derivation_hash = tagged_hash("aggregate-context-audit", &audit);
    audit
}

fn primary_role(expr: &Expr) -> ClauseRole {
    match expr {
        Expr::Univ | Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Id(_, _, _) => {
            ClauseRole::Formation
        }
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Univ) => ClauseRole::Formation,
        Expr::App(function, _) if matches!(function.as_ref(), Expr::Lam(_)) => {
            ClauseRole::Elimination
        }
        Expr::PathCon(_) => ClauseRole::PathAttach,
        Expr::Var(_) | Expr::Lam(_) | Expr::Refl(_) => ClauseRole::Introduction,
        Expr::App(_, _) => ClauseRole::Introduction,
        Expr::Susp(_)
        | Expr::Trunc(_)
        | Expr::Flat(_)
        | Expr::Sharp(_)
        | Expr::Disc(_)
        | Expr::Shape(_)
        | Expr::Next(_)
        | Expr::Eventually(_)
        | Expr::Bang(_)
        | Expr::WhyNot(_)
        | Expr::Lib(_) => ClauseRole::Formation,
    }
}

fn is_basic_formation_entry(expr: &Expr) -> bool {
    matches!(expr, Expr::Univ | Expr::Var(_))
        || matches!(expr, Expr::App(left, _) if matches!(left.as_ref(), Expr::Univ))
}

fn is_former_root(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Pi(_, _) | Expr::Sigma(_, _) | Expr::Lam(_) | Expr::App(_, _)
    )
}

fn top_modal_kind(expr: &Expr) -> u8 {
    match expr {
        Expr::Flat(_) => 1,
        Expr::Sharp(_) => 2,
        Expr::Disc(_) => 4,
        Expr::Shape(_) => 8,
        _ => 0,
    }
}

fn is_polymorphic_temporal_site(expr: &Expr) -> bool {
    matches!(expr, Expr::Lam(body) if matches!(body.as_ref(), Expr::App(function, _) if matches!(function.as_ref(), Expr::Eventually(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_)))))
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Next(inner2) | Expr::Bang(inner2) if matches!(inner2.as_ref(), Expr::Var(_))))
                    && matches!(codomain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        )
        || matches!(
            expr,
            Expr::Pi(domain, codomain)
                if matches!(domain.as_ref(), Expr::Next(inner) | Expr::Bang(inner) if matches!(inner.as_ref(), Expr::Var(_)))
                    && matches!(codomain.as_ref(), Expr::Eventually(inner) | Expr::WhyNot(inner) if matches!(inner.as_ref(), Expr::Var(_)))
        )
}

fn is_spatial_temporal_site(expr: &Expr) -> bool {
    matches!(
        expr,
        Expr::Lam(body)
            if matches!(
                body.as_ref(),
                Expr::App(function, argument)
                    if matches!(function.as_ref(), Expr::Lib(_))
                        && matches!(
                            argument.as_ref(),
                            Expr::Next(inner) | Expr::Eventually(inner)
                                | Expr::Bang(inner) | Expr::WhyNot(inner)
                                if matches!(inner.as_ref(), Expr::Var(_))
                        )
            )
    )
}

fn synthesis_site_count(expr: &Expr) -> u8 {
    u8::from(is_polymorphic_temporal_site(expr)) + u8::from(is_spatial_temporal_site(expr))
}

fn lib_mask(expr: &Expr) -> u8 {
    let refs = expr.lib_refs();
    u8::from(refs.contains(&14)) | (u8::from(refs.contains(&15)) << 1)
}

fn contains_direct_application(expr: &Expr, dominant: u32) -> bool {
    match expr {
        Expr::App(function, argument) => {
            matches!(function.as_ref(), Expr::Lib(step) if *step == dominant)
                || contains_direct_application(function, dominant)
                || contains_direct_application(argument, dominant)
        }
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            contains_direct_application(domain, dominant)
                || contains_direct_application(codomain, dominant)
        }
        Expr::Id(ty, left, right) => {
            contains_direct_application(ty, dominant)
                || contains_direct_application(left, dominant)
                || contains_direct_application(right, dominant)
        }
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => contains_direct_application(inner, dominant),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
    }
}

fn vars_are_binder_internal(expr: &Expr, source_scope: u32) -> bool {
    match expr {
        Expr::Var(level) => *level > source_scope,
        Expr::App(a, b) | Expr::Pi(a, b) | Expr::Sigma(a, b) => {
            vars_are_binder_internal(a, source_scope) && vars_are_binder_internal(b, source_scope)
        }
        Expr::Id(a, b, c) => {
            vars_are_binder_internal(a, source_scope)
                && vars_are_binder_internal(b, source_scope)
                && vars_are_binder_internal(c, source_scope)
        }
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
        | Expr::Bang(inner)
        | Expr::WhyNot(inner) => vars_are_binder_internal(inner, source_scope),
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => true,
    }
}

fn transportable_exported_domains(signature: &SealedSignature, import: u32) -> Vec<Expr> {
    let Some(entry) = signature.entry(import) else {
        return Vec::new();
    };
    let ambient = minimal_ambient_parameters(&entry.telescope);
    entry
        .formation_clauses
        .iter()
        .filter_map(|index| {
            let clause = &entry.telescope.clauses[usize::from(*index)];
            let Expr::Pi(domain, _) = &clause.expr else {
                return None;
            };
            let source_scope = ambient + u32::from(*index);
            vars_are_binder_internal(domain, source_scope).then(|| (**domain).clone())
        })
        .collect()
}

fn normalized_argument_matches(argument: &Expr, scope: u32, exported_domains: &[Expr]) -> bool {
    let Ok(argument_nf) = normalize(argument, scope, 256) else {
        return false;
    };
    exported_domains.iter().any(|domain| {
        normalize(domain, 0, 256).is_ok_and(|domain_nf| argument_nf.expr == domain_nf.expr)
    })
}

fn has_potential_typed_application(
    expr: &Expr,
    dominant: u32,
    clause_scope: u32,
    exported_domains: &[Expr],
) -> bool {
    fn walk(
        expr: &Expr,
        dominant: u32,
        scope: u32,
        binders: u32,
        exported_domains: &[Expr],
    ) -> bool {
        match expr {
            Expr::App(function, argument) => {
                (matches!(function.as_ref(), Expr::Lib(step) if *step == dominant)
                    && normalized_argument_matches(argument, scope + binders, exported_domains))
                    || walk(function, dominant, scope, binders, exported_domains)
                    || walk(argument, dominant, scope, binders, exported_domains)
            }
            Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
                walk(domain, dominant, scope, binders, exported_domains)
                    || walk(codomain, dominant, scope, binders + 1, exported_domains)
            }
            Expr::Lam(body) => walk(body, dominant, scope, binders + 1, exported_domains),
            Expr::Id(ty, left, right) => {
                walk(ty, dominant, scope, binders, exported_domains)
                    || walk(left, dominant, scope, binders, exported_domains)
                    || walk(right, dominant, scope, binders, exported_domains)
            }
            Expr::Refl(inner)
            | Expr::Susp(inner)
            | Expr::Trunc(inner)
            | Expr::Flat(inner)
            | Expr::Sharp(inner)
            | Expr::Disc(inner)
            | Expr::Shape(inner)
            | Expr::Next(inner)
            | Expr::Eventually(inner)
            | Expr::Bang(inner)
            | Expr::WhyNot(inner) => walk(inner, dominant, scope, binders, exported_domains),
            Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => false,
        }
    }
    walk(expr, dominant, clause_scope, 0, exported_domains)
}

fn classify_expr(
    signature: &SealedSignature,
    closure: &PredecessorClosure,
    position: u32,
    expr: &Expr,
    domains14: &[Expr],
    domains15: &[Expr],
) -> ExprFactorV2 {
    let shortfall = required_clause_ambient(expr, position);
    let formation_probe = elaborate_single_clause(
        expr,
        MAX_AMBIENT,
        &vec![ClauseRole::Formation; position as usize],
        T_D2_3_STREAM_VISIBLE_LIBRARY,
    );
    let (formation, invalid_named, unclassified) = match formation_probe {
        Ok(elaborated) => (
            elaborated.kernel_role == ClauseRole::Formation,
            false,
            false,
        ),
        Err(ElabError::BareUnivArgument) => (false, true, false),
        Err(_) => (false, false, true),
    };
    let mut per_ambient = Vec::new();
    if !invalid_named && !unclassified {
        for ambient in shortfall..=MAX_AMBIENT {
            let Ok(elaborated) = elaborate_single_clause(
                expr,
                ambient,
                &vec![ClauseRole::Formation; position as usize],
                T_D2_3_STREAM_VISIBLE_LIBRARY,
            ) else {
                continue;
            };
            let free_scope = ambient + position;
            let probe = clause_presentation(
                &elaborated.normal_form,
                free_scope,
                &vec![ClauseRole::Formation; position as usize],
                ambient,
            );
            let used_fields = probe
                .renaming
                .forward
                .iter()
                .filter(|(level, _)| *level > ambient)
                .map(|(level, _)| (*level - ambient - 1) as u16)
                .collect::<Vec<_>>();
            let mut marginal_by_assignment = Vec::new();
            for assignment in 0..(1usize << used_fields.len()) {
                let mut roles = vec![ClauseRole::Introduction; position as usize];
                for (bit, field) in used_fields.iter().enumerate() {
                    if assignment & (1 << bit) != 0 {
                        roles[usize::from(*field)] = ClauseRole::Formation;
                    }
                }
                let presentation =
                    clause_presentation(&elaborated.normal_form, free_scope, &roles, ambient);
                marginal_by_assignment.push(
                    closure_clause_disposition(signature, closure, &presentation)
                        == ClauseClosureDisposition::Marginal,
                );
            }
            per_ambient.push(AmbientFactsV2 {
                ambient,
                used_fields,
                marginal_by_assignment,
            });
        }
    }
    let support_mask = lib_mask(expr);
    let app14 = contains_direct_application(expr, 14);
    let app15 = contains_direct_application(expr, 15);
    let clause_scope = MAX_AMBIENT + position;
    ExprFactorV2 {
        formation,
        shortfall,
        per_ambient,
        invalid_named,
        unclassified,
        basic_formation: is_basic_formation_entry(expr),
        top_path: matches!(expr, Expr::PathCon(_)),
        top_modal: expr.is_modal(),
        temporal_like: expr.is_temporal_like(),
        top_suspension: matches!(expr, Expr::Susp(_)),
        has_lib: support_mask != 0,
        former_root: is_former_root(expr),
        support_mask,
        modal_kind: top_modal_kind(expr),
        path_basis: match expr {
            Expr::PathCon(dimension) => {
                u8::try_from(1 + dimension.saturating_mul(*dimension)).unwrap_or(u8::MAX)
            }
            _ => 0,
        },
        synthesis_sites: synthesis_site_count(expr),
        app14,
        app15,
        potential_typed_app14: app14
            && has_potential_typed_application(expr, 14, clause_scope, domains14),
        potential_typed_app15: app15
            && has_potential_typed_application(expr, 15, clause_scope, domains15),
    }
}

fn apply_factor(
    mut state: DpStateV2,
    factor: &ExprFactorV2,
    position: usize,
    final_ambient: u32,
) -> Option<DpStateV2> {
    if factor.shortfall > final_ambient {
        return None;
    }
    if factor.formation {
        state.formation_bits |= 1u8 << position;
    }
    state.max_shortfall = state.max_shortfall.max(factor.shortfall);
    state.any_invalid |= factor.invalid_named;
    state.any_unclassified |= factor.unclassified;
    if !factor.invalid_named && !factor.unclassified {
        let facts = factor
            .per_ambient
            .iter()
            .find(|facts| facts.ambient == final_ambient)?;
        let mut assignment = 0usize;
        for (bit, field) in facts.used_fields.iter().enumerate() {
            if state.formation_bits & (1u8 << *field as u8) != 0 {
                assignment |= 1 << bit;
            }
        }
        state.any_marginal |= facts.marginal_by_assignment[assignment];
    }
    state.all_basic &= factor.basic_formation;
    state.any_path |= factor.top_path;
    state.any_modal |= factor.top_modal;
    state.any_temporal_like |= factor.temporal_like;
    state.any_suspension |= factor.top_suspension;
    if position < 2 {
        state.first_two_have_lib &= factor.has_lib;
    }
    state.any_lib |= factor.has_lib;
    state.all_former_root &= factor.former_root;
    state.support_mask |= factor.support_mask;
    state.modal_kind_mask |= factor.modal_kind;
    state.path_basis = state.path_basis.saturating_add(factor.path_basis);
    state.synthesis_sites = state.synthesis_sites.saturating_add(factor.synthesis_sites);
    state.app14 |= factor.app14;
    state.app15 |= factor.app15;
    state.potential_typed_app14 |= factor.potential_typed_app14;
    state.potential_typed_app15 |= factor.potential_typed_app15;
    Some(state)
}

fn classify_state(kappa: u16, state: &DpStateV2) -> TelescopeClass {
    if state.all_basic && !state.any_lib {
        TelescopeClass::Foundation
    } else if state.any_path {
        TelescopeClass::Hit
    } else if state.any_modal && !state.any_temporal_like {
        TelescopeClass::Modal
    } else if state.any_temporal_like {
        TelescopeClass::Synthesis
    } else if state.any_suspension {
        TelescopeClass::Suspension
    } else if (2..=4).contains(&usize::from(kappa)) && state.first_two_have_lib {
        TelescopeClass::Map
    } else if kappa >= 3 && state.any_lib && !state.any_modal {
        TelescopeClass::Axiomatic
    } else if state.all_former_root && !state.any_lib {
        TelescopeClass::Former
    } else {
        TelescopeClass::Unknown
    }
}

fn extraction_disposition(state: &DpStateV2) -> ClauseLocalExtractionProxy {
    if state.any_unclassified {
        ClauseLocalExtractionProxy::KernelUnclassified
    } else if state.any_invalid {
        ClauseLocalExtractionProxy::KernelInvalidBareUniv
    } else if state.any_marginal {
        ClauseLocalExtractionProxy::EgpMarginal
    } else {
        ClauseLocalExtractionProxy::Internal
    }
}

fn support_count(mask: u8) -> u8 {
    mask.count_ones() as u8
}

fn route_for(class: TelescopeClass, state: &DpStateV2) -> AmplificationRoute {
    match class {
        TelescopeClass::Hit => AmplificationRoute::HCapabilityUnavailable,
        TelescopeClass::Synthesis if state.synthesis_sites == 0 => {
            AmplificationRoute::SynthesisNoSites
        }
        TelescopeClass::Synthesis => AmplificationRoute::SynthesisCapabilityUnavailable,
        TelescopeClass::Axiomatic => match state.support_mask {
            3 => AmplificationRoute::P5NoUniqueDominantImport,
            1 if !state.app14 => AmplificationRoute::P5NoDominantApplications,
            2 if !state.app15 => AmplificationRoute::P5NoDominantApplications,
            1 if state.potential_typed_app14 => AmplificationRoute::P5IssuerAuditResidual,
            2 if state.potential_typed_app15 => AmplificationRoute::P5IssuerAuditResidual,
            1 | 2 => AmplificationRoute::P5LiftNotTypedAgainstExportedFormation,
            _ => AmplificationRoute::P5NoUniqueDominantImport,
        },
        _ => AmplificationRoute::NoAmplificationRequired,
    }
}

fn base_local_bound(class: TelescopeClass, state: &DpStateV2, kappa: u16) -> Option<u32> {
    let k = u32::from(kappa);
    let r = u32::from(support_count(state.support_mask));
    Some(match class {
        TelescopeClass::Foundation => k,
        TelescopeClass::Former | TelescopeClass::Unknown => 2 * k,
        TelescopeClass::Suspension => 5,
        TelescopeClass::Map => 2 * k + r * r,
        TelescopeClass::Modal => {
            let kinds = state.modal_kind_mask.count_ones();
            k + r + kinds.saturating_mul(kinds.saturating_sub(1)) / 2
        }
        TelescopeClass::Synthesis if state.synthesis_sites == 0 => k,
        TelescopeClass::Hit | TelescopeClass::Axiomatic | TelescopeClass::Synthesis => {
            return None;
        }
    })
}

fn row_key(kappa: u16, state: &DpStateV2) -> CandidateJoinRow {
    let clause_local_extraction_proxy = extraction_disposition(state);
    let clause_probe_succeeded = matches!(
        clause_local_extraction_proxy,
        ClauseLocalExtractionProxy::Internal | ClauseLocalExtractionProxy::EgpMarginal
    );
    let class = classify_state(kappa, state);
    let conditional_egp_nu_upper_bound = clause_probe_succeeded.then_some(u32::from(kappa));
    let conditional_route_nu_upper_bound = if clause_probe_succeeded {
        let egp = u32::from(kappa);
        Some(
            base_local_bound(class, state, kappa)
                .unwrap_or(egp)
                .max(egp),
        )
    } else {
        None
    };
    CandidateJoinRow {
        kappa,
        class,
        clause_local_extraction_proxy,
        direct_support: support_count(state.support_mask),
        amplification_route: route_for(class, state),
        implemented_egp_bridge_returns_bound: false,
        intended_typed_egp_proved: false,
        implemented_exact_egp_nu: None,
        implemented_egp_nu_upper_bound: None,
        conditional_on_successful_full_egp_bridge_nu_upper_bound: conditional_egp_nu_upper_bound,
        intended_semantic_nu_upper_bound: None,
        implemented_route_nu_upper_bound: None,
        conditional_clause_local_route_nu_upper_bound: conditional_route_nu_upper_bound,
        operational_a5_verdict: if clause_probe_succeeded {
            OperationalA5Verdict::UnrankableTypedFamilyResidual
        } else {
            OperationalA5Verdict::KernelInvalid
        },
        count: 0,
    }
}

fn product(widths: &[usize]) -> Option<u128> {
    widths.iter().try_fold(1u128, |accumulator, width| {
        accumulator.checked_mul(*width as u128)
    })
}

fn symbolic_rows(kappa: u16, catalogs: &[PositionCatalogV2]) -> Vec<CandidateJoinRow> {
    let mut joined: BTreeMap<CandidateJoinRow, u128> = BTreeMap::new();
    for final_ambient in 0..=MAX_AMBIENT {
        let mut states = BTreeMap::from([(DpStateV2::initial(), 1u128)]);
        for (position, catalog) in catalogs.iter().take(usize::from(kappa)).enumerate() {
            let mut next = BTreeMap::new();
            for (state, count) in &states {
                for (factor, factor_count) in &catalog.factor_histogram {
                    if let Some(state) =
                        apply_factor(state.clone(), factor, position, final_ambient)
                    {
                        *next.entry(state).or_insert(0u128) += count * factor_count;
                    }
                }
            }
            states = next;
        }
        for (state, count) in states {
            if state.max_shortfall != final_ambient {
                continue;
            }
            *joined.entry(row_key(kappa, &state)).or_insert(0) += count;
        }
    }
    let mut rows = joined
        .into_iter()
        .map(|(mut row, count)| {
            row.count = count;
            row
        })
        .collect::<Vec<_>>();
    rows.sort();
    rows
}

fn fold_concrete_factors(
    kappa: u16,
    factors: &[ExprFactorV2],
) -> Result<CandidateJoinRow, Td23CandidateContentStreamV2Error> {
    let final_ambient = factors
        .iter()
        .map(|factor| factor.shortfall)
        .max()
        .unwrap_or(0);
    if final_ambient > MAX_AMBIENT {
        return Err(Td23CandidateContentStreamV2Error::Invariant(format!(
            "candidate requires ambient {final_ambient}, above frozen maximum {MAX_AMBIENT}"
        )));
    }
    let mut state = DpStateV2::initial();
    for (position, factor) in factors.iter().enumerate() {
        state = apply_factor(state, factor, position, final_ambient).ok_or_else(|| {
            Td23CandidateContentStreamV2Error::Invariant(format!(
                "candidate factor at position {position} has no frozen ambient-{final_ambient} fold"
            ))
        })?;
    }
    if state.max_shortfall != final_ambient {
        return Err(Td23CandidateContentStreamV2Error::Invariant(
            "concrete factor fold did not preserve maximum ambient shortfall".to_owned(),
        ));
    }
    Ok(row_key(kappa, &state))
}

fn candidate_from_indices(
    catalogs: &[PositionCatalogV2],
    indices: &[usize],
) -> Result<Telescope, Td23CandidateContentStreamV2Error> {
    let clauses = indices
        .iter()
        .enumerate()
        .map(|(position, index)| {
            let expr = catalogs[position].content.expression_at(*index)?;
            Ok(ClauseRec::new(primary_role(&expr), expr))
        })
        .collect::<Result<Vec<_>, Td23CandidateContentStreamV2Error>>()?;
    Ok(Telescope::new(clauses))
}

fn unrank_indices(
    catalogs: &[PositionCatalogV2],
    kappa: u16,
    ordinal: u128,
) -> Result<Vec<usize>, Td23CandidateContentStreamV2Error> {
    let widths = catalogs
        .iter()
        .take(usize::from(kappa))
        .map(|catalog| catalog.content.len())
        .collect::<Vec<_>>();
    let total = product(&widths).ok_or_else(|| {
        Td23CandidateContentStreamV2Error::Arithmetic(
            "mixed-radix candidate product exceeds u128".to_owned(),
        )
    })?;
    if ordinal >= total {
        return Err(Td23CandidateContentStreamV2Error::Input(format!(
            "ordinal {ordinal} is outside kappa-{kappa} stream of length {total}"
        )));
    }
    let mut residue = ordinal;
    let mut indices = vec![0usize; usize::from(kappa)];
    for position in (0..usize::from(kappa)).rev() {
        let width = widths[position] as u128;
        indices[position] =
            usize::try_from(residue % width).expect("catalog index is bounded by a usize width");
        residue /= width;
    }
    if residue != 0 {
        return Err(Td23CandidateContentStreamV2Error::Invariant(
            "mixed-radix unranking left a nonzero residue".to_owned(),
        ));
    }
    Ok(indices)
}

#[cfg(test)]
fn candidate_digest_at(
    catalogs: &[PositionCatalogV2],
    kappa: u16,
    ordinal: u128,
) -> Result<String, Td23CandidateContentStreamV2Error> {
    let indices = unrank_indices(catalogs, kappa, ordinal)?;
    Ok(candidate_hash(&candidate_from_indices(catalogs, &indices)?))
}

fn certificate_digest(certificate: &Td23CandidateContentStreamV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn item_digest(item: &Td23CandidateContentItemV2) -> String {
    let mut projection = item.clone();
    projection.content_digest.clear();
    tagged_hash("candidate-content-item", &projection)
}

fn validate_caps(
    min_kappa: u16,
    max_kappa: u16,
    max_expr_nodes: u8,
) -> Result<(), Td23CandidateContentStreamV2Error> {
    if min_kappa == 0 || min_kappa > max_kappa {
        return Err(Td23CandidateContentStreamV2Error::Input(format!(
            "invalid kappa interval {min_kappa}..={max_kappa}"
        )));
    }
    if max_kappa > 8 {
        return Err(Td23CandidateContentStreamV2Error::Input(
            "the frozen aggregate formation-bit encoding supports at most eight clauses".to_owned(),
        ));
    }
    if max_expr_nodes == 0 {
        return Err(Td23CandidateContentStreamV2Error::Input(
            "max_expr_nodes must be positive".to_owned(),
        ));
    }
    Ok(())
}

fn add_gap(open_gaps: &mut Vec<String>, gap: &str, detail: impl AsRef<str>) {
    open_gaps.push(format!("{gap}: {}", detail.as_ref()));
}

/// Opens a runtime stream over an arbitrary frozen-IP1 cap.  This is public
/// for bounded replay and tests; the canonical issuer below fixes 2..=4 and
/// the six-node catalog.
pub fn open_t_d2_3_candidate_content_stream_v2_with_caps(
    min_kappa: u16,
    max_kappa: u16,
    max_expr_nodes: u8,
) -> Result<Td23CandidateContentStreamV2, Td23CandidateContentStreamV2Error> {
    validate_caps(min_kappa, max_kappa, max_expr_nodes)?;
    let signature = SealedSignature::genesis_del_h15();
    let closure = predecessor_closure(&signature).map_err(|error| {
        Td23CandidateContentStreamV2Error::Input(format!(
            "typed predecessor closure failed: {error}"
        ))
    })?;
    let domains14 = transportable_exported_domains(&signature, 14);
    let domains15 = transportable_exported_domains(&signature, 15);
    let mut open_gaps = Vec::new();
    // Literal F-D2-4 sequencing: authenticate the already-frozen schema
    // before computing even the aggregate regression outcome.
    let v1 = issue_t_d2_3_candidate_provenance_key_schema_v1().map_err(|error| {
        Td23CandidateContentStreamV2Error::Input(format!(
            "frozen v1 key-schema issuance failed: {error}"
        ))
    })?;
    let v1_replay = replay_t_d2_3_candidate_provenance_key_schema_v1(&v1);
    if !v1_replay.valid {
        add_gap(
            &mut open_gaps,
            T_D2_3_V2_V1_SCHEMA_GAP,
            v1_replay.errors.join("; "),
        );
    }
    let aggregate_context_audit = audit_aggregate_context_contract(max_kappa, max_expr_nodes);
    if !aggregate_context_audit.raw_catalog_content_identity_proved {
        add_gap(
            &mut open_gaps,
            T_D2_3_V2_AGGREGATE_CONTEXT_GAP,
            "the frozen aggregate and local stream are not source-certified calls to the same enumerator with the same complete context",
        );
    }
    if !aggregate_context_audit.enumerator_canonical_order_contract_exact {
        add_gap(
            &mut open_gaps,
            T_D2_3_V2_ENUMERATOR_ORDER_GAP,
            "enumerate_exprs no longer ends in the frozen content-keyed canonical-order projection",
        );
    }
    // Run the frozen aggregate before retaining the compact content catalogs.
    // This sequencing keeps the full six-node replay's peak memory near the
    // larger of the two passes rather than their sum.
    let aggregate = run_ip1_candidate_join_with_caps(min_kappa, max_kappa, max_expr_nodes);

    let mut catalogs = Vec::new();
    let mut catalog_proofs = Vec::new();
    for position in 0..u32::from(max_kappa) {
        let expressions = enumerate_exprs(position_context(position, max_expr_nodes));
        let (content, content_unique, content_digest_collision_absent) =
            SerializedExpressionCatalogV2::from_expressions(&expressions);
        if !content_unique {
            add_gap(
                &mut open_gaps,
                T_D2_3_V2_CATALOG_DUPLICATE_GAP,
                format!("position {position}"),
            );
        }
        if !content_digest_collision_absent {
            add_gap(
                &mut open_gaps,
                T_D2_3_V2_CATALOG_DIGEST_COLLISION_GAP,
                format!("position {position}"),
            );
        }
        let mut factor_histogram = BTreeMap::new();
        for expression in &expressions {
            let factor = classify_expr(
                &signature, &closure, position, expression, &domains14, &domains15,
            );
            *factor_histogram.entry(factor).or_insert(0u128) += 1;
        }
        catalog_proofs.push(Td23V2CatalogProof {
            position: syntax_number(position, "zero-based raw-product position"),
            scope_size: structural_number(
                STEP16_BASE_AMBIENT + position,
                "frozen enumerator scope at this position",
            ),
            width: structural_number(
                expressions.len(),
                "content-unique expression options at this position",
            ),
            content_unique,
            content_digest_collision_absent,
            enumerator_order_is_canonical: aggregate_context_audit
                .enumerator_canonical_order_contract_exact,
            catalog_digest: tagged_hash(
                "position-catalog",
                &(position, expressions.len(), content.digest()),
            ),
        });
        catalogs.push(PositionCatalogV2 {
            content,
            factor_histogram,
        });
    }

    let mut strata = Vec::new();
    for kappa in min_kappa..=max_kappa {
        let widths = catalogs
            .iter()
            .take(usize::from(kappa))
            .map(|catalog| catalog.content.len())
            .collect::<Vec<_>>();
        let stream_total = product(&widths);
        if stream_total.is_none() {
            add_gap(
                &mut open_gaps,
                T_D2_3_V2_PRODUCT_OVERFLOW_GAP,
                format!("kappa {kappa}"),
            );
        }
        let stream_total_value = stream_total.unwrap_or(0);
        let aggregate_stratum = aggregate
            .strata
            .iter()
            .find(|stratum| stratum.kappa == kappa)
            .ok_or_else(|| {
                Td23CandidateContentStreamV2Error::Invariant(format!(
                    "public aggregate certificate lacks kappa-{kappa} stratum"
                ))
            })?;
        let aggregate_position_widths_exact = aggregate_stratum.per_position_widths
            == widths.iter().map(|width| *width as u64).collect::<Vec<_>>();
        if !aggregate_position_widths_exact {
            add_gap(
                &mut open_gaps,
                T_D2_3_V2_AGGREGATE_WIDTH_GAP,
                format!(
                    "kappa {kappa}: stream widths {:?}, aggregate widths {:?}",
                    widths, aggregate_stratum.per_position_widths
                ),
            );
        }
        let aggregate_raw_total_exact =
            stream_total.is_some_and(|total| total == aggregate_stratum.raw_total);
        if !aggregate_raw_total_exact {
            add_gap(
                &mut open_gaps,
                T_D2_3_V2_AGGREGATE_TOTAL_GAP,
                format!(
                    "kappa {kappa}: stream {stream_total_value}, aggregate raw {}",
                    aggregate_stratum.raw_total
                ),
            );
        }
        let aggregate_joined_total_exact =
            stream_total.is_some_and(|total| total == aggregate_stratum.joined_total);
        if !aggregate_joined_total_exact {
            add_gap(
                &mut open_gaps,
                T_D2_3_V2_AGGREGATE_TOTAL_GAP,
                format!(
                    "kappa {kappa}: stream {stream_total_value}, aggregate joined {}",
                    aggregate_stratum.joined_total
                ),
            );
        }
        let replayed_rows = symbolic_rows(kappa, &catalogs);
        let symbolic_row_histogram_exact = replayed_rows == aggregate_stratum.rows;
        if !symbolic_row_histogram_exact {
            let first_mismatch = replayed_rows
                .iter()
                .zip(&aggregate_stratum.rows)
                .position(|(replayed, aggregate)| replayed != aggregate)
                .unwrap_or_else(|| replayed_rows.len().min(aggregate_stratum.rows.len()));
            let replayed_digest = replayed_rows
                .get(first_mismatch)
                .map(|row| tagged_hash("mismatched-replayed-row", row))
                .unwrap_or_else(|| "absent".to_owned());
            let aggregate_digest = aggregate_stratum
                .rows
                .get(first_mismatch)
                .map(|row| tagged_hash("mismatched-aggregate-row", row))
                .unwrap_or_else(|| "absent".to_owned());
            add_gap(
                &mut open_gaps,
                T_D2_3_V2_ROW_FACTORIZATION_GAP,
                format!(
                    "kappa {kappa}: replayed rows {}, aggregate rows {}, first mismatch {first_mismatch}, replayed {replayed_digest}, aggregate {aggregate_digest}",
                    replayed_rows.len(),
                    aggregate_stratum.rows.len()
                ),
            );
        }

        let mut boundary_candidate_digests = Vec::new();
        let mut boundary_key_adapter_replay_exact = stream_total.is_some();
        if let Some(total) = stream_total {
            let boundary_ordinals = if total <= 1 {
                vec![0]
            } else {
                vec![0, total - 1]
            };
            for ordinal in boundary_ordinals {
                let indices = unrank_indices(&catalogs, kappa, ordinal)?;
                let candidate = candidate_from_indices(&catalogs, &indices)?;
                let key_set = issue_t_d2_3_candidate_keys(
                    &signature,
                    &candidate,
                    T_D2_3_STREAM_VISIBLE_LIBRARY,
                );
                let replay_errors = replay_t_d2_3_candidate_keys(
                    &signature,
                    &candidate,
                    T_D2_3_STREAM_VISIBLE_LIBRARY,
                    &key_set,
                );
                boundary_key_adapter_replay_exact &= replay_errors.is_empty()
                    && key_set.candidate_digest == candidate_hash(&candidate)
                    && key_set.schema_digest == v1.key_schema.schema_digest;
                boundary_candidate_digests.push(candidate_hash(&candidate));
            }
        }
        if !boundary_key_adapter_replay_exact {
            add_gap(
                &mut open_gaps,
                T_D2_3_V2_V1_ADAPTER_GAP,
                format!("kappa {kappa} boundary replay"),
            );
        }
        let rank_unrank_bijection_proved = stream_total.is_some()
            && catalog_proofs
                .iter()
                .take(usize::from(kappa))
                .all(|catalog| catalog.content_unique && catalog.content_digest_collision_absent);
        let mut factorization = Td23V2StratumFactorization {
            kappa: syntax_number(kappa, "clause-count stratum"),
            position_widths: widths
                .iter()
                .map(|width| structural_number(*width, "mixed-radix width"))
                .collect(),
            stream_raw_total: structural_number(
                stream_total_value,
                "Cartesian-product stream cardinality",
            ),
            aggregate_raw_total: aggregate_number(
                aggregate_stratum.raw_total,
                "public IP-1 raw-product cardinality",
            ),
            aggregate_joined_total: aggregate_number(
                aggregate_stratum.joined_total,
                "public IP-1 joined-row cardinality",
            ),
            aggregate_row_count: aggregate_number(
                aggregate_stratum.rows.len(),
                "public IP-1 quotient row count",
            ),
            mixed_radix_product_exact: stream_total.is_some(),
            rank_unrank_bijection_proved,
            aggregate_position_widths_exact,
            aggregate_raw_total_exact,
            aggregate_joined_total_exact,
            symbolic_row_histogram_exact,
            boundary_key_adapter_replay_exact,
            boundary_candidate_digests,
            factorization_digest: String::new(),
        };
        factorization.factorization_digest = tagged_hash("stratum-factorization", &factorization);
        strata.push(factorization);
    }

    let mut key_adapter = Td23V2KeyAdapterProof {
        frozen_v1_schema: v1.schema.clone(),
        frozen_v1_schema_digest: v1.key_schema.schema_digest.clone(),
        v1_schema_certificate_replayed: v1_replay.valid,
        v1_schema_fixed_before_join: v1.key_schema.schema_fixed_before_any_join_outcome,
        v1_schema_replayed_before_aggregate_outcome: true,
        v1_schema_fields_redeclared_by_v2: false,
        v1_issuer_called_on_materialized_telescope: true,
        typed_and_kernel_failure_results_both_preserved: true,
        adapter_is_total_at_rust_type: true,
        join_outcome_read: false,
        selector_value_or_bar_read: false,
        proof_digest: String::new(),
    };
    key_adapter.proof_digest = tagged_hash("v1-key-adapter", &key_adapter);

    let all_catalogs_unique = catalog_proofs.iter().all(|proof| {
        proof.content_unique
            && proof.content_digest_collision_absent
            && proof.enumerator_order_is_canonical
    });
    let aggregate_surface_factorization_proved = aggregate_context_audit
        .raw_catalog_content_identity_proved
        && all_catalogs_unique
        && strata.iter().all(|stratum| {
            stratum.mixed_radix_product_exact
                && stratum.rank_unrank_bijection_proved
                && stratum.aggregate_position_widths_exact
                && stratum.aggregate_raw_total_exact
                && stratum.aggregate_joined_total_exact
                && stratum.symbolic_row_histogram_exact
        });
    let key_adapter_proved = key_adapter.v1_schema_certificate_replayed
        && key_adapter.v1_schema_fixed_before_join
        && key_adapter.v1_schema_replayed_before_aggregate_outcome
        && !key_adapter.v1_schema_fields_redeclared_by_v2
        && strata
            .iter()
            .all(|stratum| stratum.boundary_key_adapter_replay_exact);
    let canonical_full_surface_caps = min_kappa == T_D2_3_STREAM_MIN_KAPPA
        && max_kappa == T_D2_3_STREAM_MAX_KAPPA
        && max_expr_nodes == T_D2_3_STREAM_MAX_EXPR_NODES;
    let bounded_surface_regression_complete =
        aggregate_surface_factorization_proved && key_adapter_proved && open_gaps.is_empty();
    let full_aggregate_candidate_content_stream_exported =
        canonical_full_surface_caps && bounded_surface_regression_complete;
    let status = if full_aggregate_candidate_content_stream_exported {
        Td23V2RunStatus::CompleteGateUnchanged
    } else if bounded_surface_regression_complete {
        Td23V2RunStatus::BoundedRegressionOnlyGateUnchanged
    } else {
        Td23V2RunStatus::StoppedNamedGaps
    };
    let gate = Td23V2GateRecord {
        exact_scope: "This construction exports candidate content and frozen v1 keys only. It does not execute BC2, inspect a bridge verdict, or move M3/M4.".to_owned(),
        candidate_level_join_executed: false,
        bridge_outcome_read: false,
        t_d2_1_or_t_d2_2_outcome_read: false,
        m3_v1_remains_authoritative: true,
        m3_successor_issued: false,
        m4_authorized: false,
        gate_evaluated: false,
        derivation_hash: tagged_hash(
            "gate-non-action",
            &(false, false, false, true, false, false, false),
        ),
    };
    let mut downstream_provenance_boundary = Td23V2DownstreamProvenanceBoundary {
        raw_candidate_totals: strata
            .iter()
            .map(|stratum| stratum.stream_raw_total.clone())
            .collect(),
        exhaustive_candidate_key_issuance_performed: false,
        frozen_v1_candidate_keys_are_content_specific: true,
        aggregate_rows_proved_sufficient_for_candidate_provenance: false,
        finite_provenance_congruence_theorem_proved: false,
        bc2_candidate_level_join_prerequisite_satisfied: false,
        named_boundary: T_D2_3_V2_PROVENANCE_FACTORIZATION_GAP.to_owned(),
        exact_obstruction: "The lossless stream can issue the frozen v1 keys for every ordinal, but the canonical Cartesian strata are too large to treat exhaustive candidate-by-candidate execution as completion evidence. The frozen candidate digest is content-specific, while no theorem yet proves that every provenance-bearing BC2 observation factors through the finite symbolic row basis.".to_owned(),
        required_successor_theorem: "Prove, without changing the v1 key schema, that the candidate-level provenance join is congruent on an independently specified finite refinement of the symbolic expression/row classes; replay boundary representatives and aggregate the refinement back to every frozen IP-1 parent row.".to_owned(),
        derivation_hash: String::new(),
    };
    downstream_provenance_boundary.derivation_hash = tagged_hash(
        "downstream-provenance-boundary",
        &downstream_provenance_boundary,
    );
    let mut certificate = Td23CandidateContentStreamV2Certificate {
        schema: T_D2_3_CANDIDATE_CONTENT_STREAM_V2.to_owned(),
        date: T_D2_3_CANDIDATE_CONTENT_STREAM_V2_DATE.to_owned(),
        exact_scope: "Lossless, lazy candidate Telescope stream over the frozen IP-1 raw Cartesian product, with an independent symbolic factorization into the public quotient and an adapter to the already-frozen T-D2-3 v1 key issuer.".to_owned(),
        min_kappa: syntax_number(min_kappa, "inclusive stream kappa lower bound"),
        max_kappa: syntax_number(max_kappa, "inclusive stream kappa upper bound"),
        max_expr_nodes: structural_number(
            max_expr_nodes,
            "frozen raw expression node cap",
        ),
        visible_library: kernel_number(
            T_D2_3_STREAM_VISIBLE_LIBRARY,
            "frozen signature prefix visible to the v1 key issuer",
        ),
        signature_digest: signature.digest().to_owned(),
        aggregate_certificate_digest: aggregate.digest.clone(),
        source_bindings: source_bindings(),
        aggregate_context_audit,
        catalog_proofs,
        strata,
        key_adapter,
        downstream_provenance_boundary,
        stream_is_lazy: true,
        complete_candidate_telescopes_materialized_at_once: false,
        mixed_radix_rank_and_unrank_available: true,
        bounded_chunk_iterator_available: true,
        every_materialized_item_carries_candidate_content: true,
        every_materialized_item_carries_aggregate_row_locator: true,
        every_materialized_item_is_fed_to_frozen_v1_key_issuer: true,
        aggregate_rows_used_to_define_catalogs_or_ordinals: false,
        aggregate_rows_read_only_as_factorization_regression: true,
        candidate_or_key_content_filtered_by_aggregate_row: false,
        full_aggregate_candidate_content_stream_exported,
        aggregate_surface_factorization_proved,
        canonical_full_surface_caps,
        bounded_surface_regression_complete,
        open_gaps,
        status,
        gate,
        permitted_conclusion: if full_aggregate_candidate_content_stream_exported {
            "The canonical frozen IP-1 aggregate surface now has a lossless candidate-content stream. Every ordinal determines one exact Telescope, one exact aggregate-row locator, and one deterministic result from the unchanged T-D2-3 v1 key issuer. This closes only the aggregate content-stream integration gap; it does not execute the candidate-level provenance join or move M3/M4.".to_owned()
        } else if bounded_surface_regression_complete {
            "The requested bounded cap is a complete regression of the stream and factorization algorithms. It is not the canonical 2..=4, six-node surface and therefore does not issue the full-surface theorem or change any downstream prerequisite.".to_owned()
        } else {
            "The attempted stream is fail-closed. Candidate-content coverage or factorization has an exact named gap; no BC2 join or successor gate is authorized.".to_owned()
        },
        required_successor_action: if full_aggregate_candidate_content_stream_exported {
            "Prove the named finite provenance-congruence successor theorem before BC2: candidate-bound v1 keys must factor through an independently specified finite refinement, whose representatives replay and aggregate back to every frozen IP-1 parent row. Brute-force traversal of the canonical raw products is not completion evidence.".to_owned()
        } else if bounded_surface_regression_complete {
            "Run the canonical full-surface issuer; bounded caps are regression witnesses only.".to_owned()
        } else {
            "Resolve the published stream/factorization gaps without redefining the raw catalogs, aggregate quotient, or frozen v1 key schema, then replay this version.".to_owned()
        },
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);

    Ok(Td23CandidateContentStreamV2 {
        signature,
        closure,
        domains14,
        domains15,
        min_kappa,
        max_kappa,
        max_expr_nodes,
        catalogs,
        aggregate,
        certificate,
    })
}

pub fn open_t_d2_3_candidate_content_stream_v2()
-> Result<Td23CandidateContentStreamV2, Td23CandidateContentStreamV2Error> {
    open_t_d2_3_candidate_content_stream_v2_with_caps(
        T_D2_3_STREAM_MIN_KAPPA,
        T_D2_3_STREAM_MAX_KAPPA,
        T_D2_3_STREAM_MAX_EXPR_NODES,
    )
}

pub fn issue_t_d2_3_candidate_content_stream_v2_with_caps(
    min_kappa: u16,
    max_kappa: u16,
    max_expr_nodes: u8,
) -> Result<Td23CandidateContentStreamV2Certificate, Td23CandidateContentStreamV2Error> {
    Ok(
        open_t_d2_3_candidate_content_stream_v2_with_caps(min_kappa, max_kappa, max_expr_nodes)?
            .certificate,
    )
}

pub fn issue_t_d2_3_candidate_content_stream_v2()
-> Result<Td23CandidateContentStreamV2Certificate, Td23CandidateContentStreamV2Error> {
    issue_t_d2_3_candidate_content_stream_v2_with_caps(
        T_D2_3_STREAM_MIN_KAPPA,
        T_D2_3_STREAM_MAX_KAPPA,
        T_D2_3_STREAM_MAX_EXPR_NODES,
    )
}

impl Td23CandidateContentStreamV2 {
    pub fn certificate(&self) -> &Td23CandidateContentStreamV2Certificate {
        &self.certificate
    }

    pub fn min_kappa(&self) -> u16 {
        self.min_kappa
    }

    pub fn max_kappa(&self) -> u16 {
        self.max_kappa
    }

    pub fn max_expr_nodes(&self) -> u8 {
        self.max_expr_nodes
    }

    pub fn len(&self, kappa: u16) -> Result<u128, Td23CandidateContentStreamV2Error> {
        self.validate_kappa(kappa)?;
        let widths = self
            .catalogs
            .iter()
            .take(usize::from(kappa))
            .map(|catalog| catalog.content.len())
            .collect::<Vec<_>>();
        product(&widths).ok_or_else(|| {
            Td23CandidateContentStreamV2Error::Arithmetic(format!(
                "kappa-{kappa} stream cardinality exceeds u128"
            ))
        })
    }

    fn validate_kappa(&self, kappa: u16) -> Result<(), Td23CandidateContentStreamV2Error> {
        if !(self.min_kappa..=self.max_kappa).contains(&kappa) {
            return Err(Td23CandidateContentStreamV2Error::Input(format!(
                "kappa {kappa} is outside stream interval {}..={}",
                self.min_kappa, self.max_kappa
            )));
        }
        Ok(())
    }

    pub fn candidate_at(
        &self,
        kappa: u16,
        ordinal: u128,
    ) -> Result<Telescope, Td23CandidateContentStreamV2Error> {
        self.validate_kappa(kappa)?;
        let indices = unrank_indices(&self.catalogs, kappa, ordinal)?;
        candidate_from_indices(&self.catalogs, &indices)
    }

    pub fn rank_candidate(
        &self,
        candidate: &Telescope,
    ) -> Result<u128, Td23CandidateContentStreamV2Error> {
        let kappa = u16::try_from(candidate.clauses.len()).map_err(|_| {
            Td23CandidateContentStreamV2Error::Input(
                "candidate clause count exceeds u16".to_owned(),
            )
        })?;
        self.validate_kappa(kappa)?;
        let mut ordinal = 0u128;
        for (position, clause) in candidate.clauses.iter().enumerate() {
            if clause.role != primary_role(&clause.expr) {
                return Err(Td23CandidateContentStreamV2Error::Input(format!(
                    "candidate clause {position} has role {:?}, expected {:?}",
                    clause.role,
                    primary_role(&clause.expr)
                )));
            }
            let catalog = &self.catalogs[position];
            let index = catalog.content.index_of(&clause.expr).ok_or_else(|| {
                Td23CandidateContentStreamV2Error::Input(format!(
                    "candidate clause {position} is absent from the frozen raw catalog"
                ))
            })?;
            ordinal = ordinal
                .checked_mul(catalog.content.len() as u128)
                .and_then(|prefix| prefix.checked_add(index as u128))
                .ok_or_else(|| {
                    Td23CandidateContentStreamV2Error::Arithmetic(
                        "mixed-radix candidate rank exceeds u128".to_owned(),
                    )
                })?;
        }
        Ok(ordinal)
    }

    pub fn item_at(
        &self,
        kappa: u16,
        ordinal: u128,
    ) -> Result<Td23CandidateContentItemV2, Td23CandidateContentStreamV2Error> {
        self.validate_kappa(kappa)?;
        let indices = unrank_indices(&self.catalogs, kappa, ordinal)?;
        let candidate = candidate_from_indices(&self.catalogs, &indices)?;
        let factors = candidate
            .clauses
            .iter()
            .enumerate()
            .map(|(position, clause)| {
                classify_expr(
                    &self.signature,
                    &self.closure,
                    position as u32,
                    &clause.expr,
                    &self.domains14,
                    &self.domains15,
                )
            })
            .collect::<Vec<_>>();
        let aggregate_row = fold_concrete_factors(kappa, &factors)?;
        let aggregate_stratum = self
            .aggregate
            .strata
            .iter()
            .find(|stratum| stratum.kappa == kappa)
            .ok_or_else(|| {
                Td23CandidateContentStreamV2Error::Invariant(format!(
                    "aggregate certificate lacks kappa-{kappa} stratum"
                ))
            })?;
        let aggregate_row_index = aggregate_stratum
            .rows
            .iter()
            .position(|row| {
                let mut key = row.clone();
                key.count = 0;
                key == aggregate_row
            })
            .ok_or_else(|| {
                Td23CandidateContentStreamV2Error::Invariant(format!(
                    "candidate ordinal {ordinal} factors to no aggregate row"
                ))
            })?;
        let raw_membership =
            assess_raw_surface_membership(position_context(0, self.max_expr_nodes), &candidate);
        if !raw_membership.is_member {
            return Err(Td23CandidateContentStreamV2Error::Invariant(format!(
                "materialized candidate ordinal {ordinal} failed raw-surface replay: {}",
                raw_membership.rejection_reasons.join("; ")
            )));
        }
        let frozen_v1_key_set =
            issue_t_d2_3_candidate_keys(&self.signature, &candidate, T_D2_3_STREAM_VISIBLE_LIBRARY);
        let frozen_v1_key_replay_errors = replay_t_d2_3_candidate_keys(
            &self.signature,
            &candidate,
            T_D2_3_STREAM_VISIBLE_LIBRARY,
            &frozen_v1_key_set,
        );
        if !frozen_v1_key_replay_errors.is_empty() {
            return Err(Td23CandidateContentStreamV2Error::Invariant(format!(
                "frozen v1 key replay failed for candidate ordinal {ordinal}: {}",
                frozen_v1_key_replay_errors.join("; ")
            )));
        }
        let candidate_digest = candidate_hash(&candidate);
        if frozen_v1_key_set.candidate_digest != candidate_digest {
            return Err(Td23CandidateContentStreamV2Error::Invariant(
                "frozen v1 key issuer changed the candidate digest".to_owned(),
            ));
        }
        let mut item = Td23CandidateContentItemV2 {
            schema: T_D2_3_CANDIDATE_CONTENT_STREAM_V2.to_owned(),
            kappa: syntax_number(kappa, "clause-count stratum"),
            ordinal: syntax_number(ordinal, "zero-based mixed-radix stream ordinal"),
            catalog_indices: indices
                .iter()
                .enumerate()
                .map(|(position, index)| {
                    syntax_number(
                        *index,
                        &format!("position-{position} expression-catalog index"),
                    )
                })
                .collect(),
            candidate,
            candidate_digest,
            raw_surface_member: raw_membership.is_member,
            raw_surface_rejections: raw_membership.rejection_reasons,
            aggregate_row_index: syntax_number(
                aggregate_row_index,
                "zero-based row index in the public IP-1 stratum",
            ),
            aggregate_row_digest: tagged_hash("aggregate-row-key", &aggregate_row),
            aggregate_row,
            frozen_v1_key_set,
            frozen_v1_key_replay_errors,
            content_digest: String::new(),
        };
        item.content_digest = item_digest(&item);
        Ok(item)
    }

    pub fn chunk(
        &self,
        kappa: u16,
        range: Range<u128>,
    ) -> Result<Td23CandidateContentIterV2<'_>, Td23CandidateContentStreamV2Error> {
        let total = self.len(kappa)?;
        if range.start > range.end || range.end > total {
            return Err(Td23CandidateContentStreamV2Error::Input(format!(
                "invalid kappa-{kappa} chunk {}..{} for stream length {total}",
                range.start, range.end
            )));
        }
        Ok(Td23CandidateContentIterV2 {
            stream: self,
            kappa,
            next: range.start,
            end: range.end,
        })
    }
}

/// Replays one serialized stream item against a live stream handle.  The
/// replay re-ranks the candidate, re-factors its aggregate row, and reissues
/// the frozen v1 key set through [`Td23CandidateContentStreamV2::item_at`].
pub fn replay_t_d2_3_candidate_content_item_v2(
    stream: &Td23CandidateContentStreamV2,
    claimed: &Td23CandidateContentItemV2,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.content_digest != item_digest(claimed) {
        errors.push("T-D2-3 v2 candidate-content item digest mismatch".to_owned());
    }
    let kappa = match claimed.kappa.decimal.parse::<u16>() {
        Ok(value) => value,
        Err(error) => {
            errors.push(format!("invalid item kappa: {error}"));
            return errors;
        }
    };
    let ordinal = match claimed.ordinal.decimal.parse::<u128>() {
        Ok(value) => value,
        Err(error) => {
            errors.push(format!("invalid item ordinal: {error}"));
            return errors;
        }
    };
    match stream.item_at(kappa, ordinal) {
        Ok(expected) if expected != *claimed => {
            errors.push(
                "T-D2-3 v2 candidate-content item differs from deterministic reissuance".to_owned(),
            );
        }
        Ok(_) => {}
        Err(error) => errors.push(format!(
            "T-D2-3 v2 candidate-content item could not be reissued: {error}"
        )),
    }
    errors
}

fn invalid_replay(error: impl Into<String>) -> Td23CandidateContentStreamV2Replay {
    Td23CandidateContentStreamV2Replay {
        valid: false,
        errors: vec![error.into()],
        status: Td23V2RunStatus::StoppedNamedGaps,
        full_stream_exported: false,
        aggregate_surface_factorization_proved: false,
        m3_v1_remains_authoritative: true,
        m4_authorized: false,
    }
}

pub fn replay_t_d2_3_candidate_content_stream_v2(
    claimed: &Td23CandidateContentStreamV2Certificate,
) -> Td23CandidateContentStreamV2Replay {
    let min_kappa = match claimed.min_kappa.decimal.parse::<u16>() {
        Ok(value) => value,
        Err(error) => return invalid_replay(format!("invalid min_kappa: {error}")),
    };
    let max_kappa = match claimed.max_kappa.decimal.parse::<u16>() {
        Ok(value) => value,
        Err(error) => return invalid_replay(format!("invalid max_kappa: {error}")),
    };
    let max_expr_nodes = match claimed.max_expr_nodes.decimal.parse::<u8>() {
        Ok(value) => value,
        Err(error) => return invalid_replay(format!("invalid max_expr_nodes: {error}")),
    };
    let expected = match issue_t_d2_3_candidate_content_stream_v2_with_caps(
        min_kappa,
        max_kappa,
        max_expr_nodes,
    ) {
        Ok(certificate) => certificate,
        Err(error) => return invalid_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("T-D2-3 v2 certificate digest mismatch".to_owned());
    }
    if claimed.source_bindings != source_bindings() {
        errors.push("T-D2-3 v2 source bindings drifted".to_owned());
    }
    if claimed != &expected {
        errors.push("T-D2-3 v2 certificate differs from deterministic reissuance".to_owned());
    }
    let valid = errors.is_empty();
    Td23CandidateContentStreamV2Replay {
        valid,
        errors,
        status: if valid {
            claimed.status
        } else {
            Td23V2RunStatus::StoppedNamedGaps
        },
        full_stream_exported: valid && claimed.full_aggregate_candidate_content_stream_exported,
        aggregate_surface_factorization_proved: valid
            && claimed.aggregate_surface_factorization_proved,
        // Invalid successor evidence never dislodges the frozen baseline.
        m3_v1_remains_authoritative: true,
        m4_authorized: valid && claimed.gate.m4_authorized,
    }
}

pub fn replay_t_d2_3_candidate_content_stream_v2_json(
    json: &str,
) -> Td23CandidateContentStreamV2Replay {
    match serde_json::from_str::<Td23CandidateContentStreamV2Certificate>(json) {
        Ok(certificate) => replay_t_d2_3_candidate_content_stream_v2(&certificate),
        Err(error) => invalid_replay(format!("invalid T-D2-3 v2 JSON: {error}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reseal(certificate: &mut Td23CandidateContentStreamV2Certificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    #[test]
    fn one_node_surface_streams_losslessly_and_factors_exactly() {
        let stream = open_t_d2_3_candidate_content_stream_v2_with_caps(2, 4, 1)
            .expect("bounded stream opens");
        let certificate = stream.certificate();
        assert_eq!(
            certificate.status,
            Td23V2RunStatus::BoundedRegressionOnlyGateUnchanged
        );
        assert!(certificate.open_gaps.is_empty());
        assert!(!certificate.full_aggregate_candidate_content_stream_exported);
        assert!(!certificate.canonical_full_surface_caps);
        assert!(certificate.bounded_surface_regression_complete);
        assert!(certificate.aggregate_surface_factorization_proved);
        assert!(certificate.key_adapter.v1_schema_certificate_replayed);
        assert!(!certificate.aggregate_rows_used_to_define_catalogs_or_ordinals);
        assert!(certificate.aggregate_rows_read_only_as_factorization_regression);
        assert!(!certificate.candidate_or_key_content_filtered_by_aggregate_row);
        assert!(!certificate.gate.candidate_level_join_executed);
        assert!(certificate.gate.m3_v1_remains_authoritative);
        assert!(!certificate.gate.m4_authorized);

        for kappa in 2..=4 {
            let total = stream.len(kappa).expect("finite stratum");
            let mut histogram = BTreeMap::<CandidateJoinRow, u128>::new();
            for item in stream.chunk(kappa, 0..total).expect("complete chunk") {
                let item = item.expect("stream item");
                assert!(item.raw_surface_member, "{:?}", item.raw_surface_rejections);
                assert!(item.frozen_v1_key_replay_errors.is_empty());
                assert_eq!(
                    stream
                        .rank_candidate(&item.candidate)
                        .expect("candidate ranks"),
                    item.ordinal.decimal.parse::<u128>().expect("ordinal")
                );
                let mut row = item.aggregate_row;
                row.count = 0;
                *histogram.entry(row).or_insert(0) += 1;
            }
            let expected = stream
                .aggregate
                .strata
                .iter()
                .find(|stratum| stratum.kappa == kappa)
                .expect("aggregate stratum")
                .rows
                .iter()
                .map(|row| {
                    let mut key = row.clone();
                    let count = key.count;
                    key.count = 0;
                    (key, count)
                })
                .collect::<BTreeMap<_, _>>();
            assert_eq!(histogram, expected, "kappa {kappa}");
        }
    }

    #[test]
    fn rank_unrank_and_chunks_are_content_sensitive() {
        let stream =
            open_t_d2_3_candidate_content_stream_v2_with_caps(2, 2, 2).expect("stream opens");
        let total = stream.len(2).expect("finite");
        for ordinal in [0, total / 2, total - 1] {
            let candidate = stream.candidate_at(2, ordinal).expect("unrank");
            assert_eq!(stream.rank_candidate(&candidate).expect("rank"), ordinal);
            let item = stream.item_at(2, ordinal).expect("item");
            assert_eq!(item.candidate, candidate);
            assert!(item.frozen_v1_key_replay_errors.is_empty());
            assert!(replay_t_d2_3_candidate_content_item_v2(&stream, &item).is_empty());
        }
        assert!(
            stream
                .chunk(2, total..total)
                .expect("empty tail")
                .next()
                .is_none()
        );
        assert!(stream.chunk(2, total..total + 1).is_err());

        let mut outside = stream.candidate_at(2, 0).expect("candidate");
        outside.clauses[0].role = ClauseRole::Elimination;
        assert!(stream.rank_candidate(&outside).is_err());

        let mut mutated = stream.item_at(2, 0).expect("item");
        mutated.candidate.clauses[0].expr = Expr::Lib(1);
        mutated.content_digest = item_digest(&mutated);
        assert!(!replay_t_d2_3_candidate_content_item_v2(&stream, &mutated).is_empty());
    }

    #[test]
    fn replay_binds_factorization_schema_and_gate_non_action() {
        let certificate = issue_t_d2_3_candidate_content_stream_v2_with_caps(2, 2, 1)
            .expect("certificate issues");
        let replay = replay_t_d2_3_candidate_content_stream_v2(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);

        let mut row = certificate.clone();
        row.strata[0].symbolic_row_histogram_exact = false;
        reseal(&mut row);
        assert!(!replay_t_d2_3_candidate_content_stream_v2(&row).valid);

        let mut schema = certificate.clone();
        schema.key_adapter.frozen_v1_schema_digest.push('0');
        reseal(&mut schema);
        assert!(!replay_t_d2_3_candidate_content_stream_v2(&schema).valid);

        let mut gate = certificate.clone();
        gate.gate.m4_authorized = true;
        reseal(&mut gate);
        let invalid = replay_t_d2_3_candidate_content_stream_v2(&gate);
        assert!(!invalid.valid);
        assert_eq!(invalid.status, Td23V2RunStatus::StoppedNamedGaps);
        assert!(!invalid.full_stream_exported);
        assert!(!invalid.aggregate_surface_factorization_proved);
        assert!(invalid.m3_v1_remains_authoritative);
        assert!(!invalid.m4_authorized);
    }

    #[test]
    fn unknown_json_fields_fail_closed() {
        let certificate = issue_t_d2_3_candidate_content_stream_v2_with_caps(2, 2, 1)
            .expect("certificate issues");
        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("object")
            .insert("unknown_field".to_owned(), serde_json::Value::Bool(true));
        let replay = replay_t_d2_3_candidate_content_stream_v2_json(
            &serde_json::to_string(&value).expect("json"),
        );
        assert!(!replay.valid);
    }

    #[test]
    fn candidate_digest_helper_agrees_with_materialization() {
        let stream =
            open_t_d2_3_candidate_content_stream_v2_with_caps(2, 2, 1).expect("stream opens");
        let total = stream.len(2).expect("finite");
        for ordinal in [0, total - 1] {
            assert_eq!(
                candidate_digest_at(&stream.catalogs, 2, ordinal).expect("helper"),
                candidate_hash(&stream.candidate_at(2, ordinal).expect("candidate"))
            );
        }
    }
}
