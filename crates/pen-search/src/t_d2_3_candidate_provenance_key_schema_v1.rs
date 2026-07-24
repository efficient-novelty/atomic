//! T-D2-3: outcome-blind candidate/provenance key schema.
//!
//! The schema is defined solely from a candidate telescope, its exact
//! clause occurrence, and coordinates replayed by the frozen kernel and
//! normalizer.  The aggregate candidate join is neither imported nor run.
//! This file deliberately stops at key issuance and source-contract audit.

use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::typed_families::clause_presentation;
use pen_type::elaborate::{
    ClauseFailure, KernelTy, SealedSignature, candidate_hash, elaborate_telescope,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::{OpenOptions, read, read_to_string, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const T_D2_3_CANDIDATE_PROVENANCE_KEY_SCHEMA_V1: &str =
    "t-d2-3-candidate-provenance-key-schema-v1";
pub const T_D2_3_CANDIDATE_PROVENANCE_KEY_SCHEMA_V1_DATE: &str = "2026-07-23";
pub const T_D2_3_CERTIFICATE_NAME: &str = "t_d2_3_candidate_provenance_key_schema_v1.json";
pub const T_D2_3_REPORT_NAME: &str = "T_D2_3_CANDIDATE_PROVENANCE_KEY_SCHEMA_RESULT.md";
pub const T_D2_3_AGGREGATE_STREAM_GAP: &str =
    "TD23_AGGREGATE_DP_DOES_NOT_EXPORT_COMPLETE_CANDIDATE_CONTENT_STREAM";
pub const FROZEN_M3_V1_RESULT_DIGEST: &str =
    "blake3:a61fe8093ef9ce61a05aebfe859b0723a88a766a712ea6a2312e0f22ca386ba8";

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/depth_two_domain_adjudication.md");
const M3_V1_CERTIFICATE_BYTES: &[u8] = include_bytes!("../../../docs/schema2_e7_e8_bridge_v1.json");
const TELESCOPE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/telescope.rs");
const CLAUSE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/clause.rs");
const EXPR_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/expr.rs");
const ELABORATE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");
const NORMALIZE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/normalize.rs");
const EQUALITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/equality.rs");
const TYPED_FAMILIES_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/typed_families.rs");
const AGGREGATE_SOURCE_BYTES: &[u8] = include_bytes!("ip1_candidate_join.rs");
const PROVENANCE_SOURCE_BYTES: &[u8] = include_bytes!("act_local_semantic_provenance_v5.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("t_d2_3_candidate_provenance_key_schema_v1.rs");

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td23Register {
    KernelTypingAuthority,
    StructuralTestimony,
    ProofInventory,
    ArtifactMetadata,
    SyntaxIdentifier,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td23Quantifier {
    FrozenSchemaDefinition,
    GenericExplicitCandidate,
    ExactSealedHistoricalCandidate,
    StaticSourceContract,
    FrozenGateNonAction,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td23RunStatus {
    SchemaFrozenNamedIntegrationGap,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23RegisteredNumber {
    pub decimal: String,
    pub register: Td23Register,
    pub meaning: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: Td23RegisteredNumber,
    pub blake3: String,
    pub register: Td23Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23SchemaFieldRule {
    pub ordinal: Td23RegisteredNumber,
    pub field_id: String,
    pub source_authority: String,
    pub exact_derivation_rule: String,
    pub register: Td23Register,
    pub required: bool,
    pub join_outcome_dependent: bool,
    pub selector_value_or_bar_dependent: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23CandidateProvenanceKeySchema {
    pub schema: String,
    pub quantifier: Td23Quantifier,
    pub exact_scope: String,
    pub field_rules: Vec<Td23SchemaFieldRule>,
    pub field_surface_size: Td23RegisteredNumber,
    pub canonical_serialization_order: Vec<String>,
    pub candidate_digest_required: bool,
    pub telescope_clause_path_required: bool,
    pub typed_coordinate_bundle_required: bool,
    pub schema_fixed_before_candidate_regression: bool,
    pub schema_fixed_before_any_join_outcome: bool,
    pub generator_inventory_used_to_define_schema: bool,
    pub aggregate_row_set_used_to_define_schema: bool,
    pub join_outcome_used_to_define_schema: bool,
    pub schema_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23TelescopeClausePath {
    pub root: String,
    pub clause_index: Td23RegisteredNumber,
    pub clause_content_digest: String,
    pub declared_role: ClauseRole,
    pub raw_expression_digest: String,
    pub raw_expression_constructor: String,
    pub path_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23TypedCoordinateBundle {
    pub signature_digest: String,
    pub visible_library: Td23RegisteredNumber,
    pub ambient_parameters: Td23RegisteredNumber,
    pub free_scope_length: Td23RegisteredNumber,
    pub candidate_elaboration_derivation_hash: String,
    pub clause_derivation_digest: String,
    pub prior_kernel_role_prefix_digest: String,
    pub kernel_role: ClauseRole,
    pub kernel_type_constructor: String,
    pub kernel_type_digest: String,
    pub normal_form_constructor: String,
    pub normal_form_digest: String,
    pub canonical_presentation_digest: String,
    pub parameter_sort_digest: String,
    pub renaming_map_digest: String,
    pub typed_coordinate_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23CandidateProvenanceKey {
    pub schema_digest: String,
    pub candidate_digest: String,
    pub clause_path: Td23TelescopeClausePath,
    pub typed_coordinates: Td23TypedCoordinateBundle,
    pub key_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23CandidateFailure {
    pub gap_id: String,
    pub candidate_digest: String,
    pub failure_clause: Td23RegisteredNumber,
    pub failure_digest: String,
    pub exact_obstruction: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23CandidateKeySet {
    pub schema: String,
    pub quantifier: Td23Quantifier,
    pub exact_scope: String,
    pub schema_digest: String,
    pub signature_digest: String,
    pub visible_library: Td23RegisteredNumber,
    pub candidate_digest: String,
    pub candidate_clause_surface_size: Td23RegisteredNumber,
    pub key_surface_size: Td23RegisteredNumber,
    pub typed: bool,
    pub keys: Vec<Td23CandidateProvenanceKey>,
    pub failure: Option<Td23CandidateFailure>,
    pub every_clause_has_one_key: bool,
    pub duplicate_key_digest_absent: bool,
    pub selector_value_or_bar_input_read: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23HistoricalRegressionRow {
    pub entry_identifier: Td23RegisteredNumber,
    pub quantifier: Td23Quantifier,
    pub exact_scope: String,
    pub sealed_candidate_digest: String,
    pub visible_library: Td23RegisteredNumber,
    pub key_set: Td23CandidateKeySet,
    pub sealed_candidate_digest_exact: bool,
    pub clause_key_bijection: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23HistoricalRegression {
    pub signature_digest: String,
    pub candidate_surface_size: Td23RegisteredNumber,
    pub clause_surface_size: Td23RegisteredNumber,
    pub key_surface_size: Td23RegisteredNumber,
    pub rows: Vec<Td23HistoricalRegressionRow>,
    pub every_sealed_candidate_typed: bool,
    pub every_sealed_clause_keyed: bool,
    pub key_digest_unique_within_candidate: bool,
    pub schema_digest_constant_across_candidates: bool,
    pub regression_complete: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23StaticSourceAudit {
    pub quantifier: Td23Quantifier,
    pub exact_scope: String,
    pub aggregate_key_section_digest: String,
    pub aggregate_fold_section_digest: String,
    pub candidate_hash_contract_found: bool,
    pub typed_clause_occurrence_contract_found: bool,
    pub provenance_candidate_identity_contract_found: bool,
    pub provenance_clause_occurrence_contract_found: bool,
    pub aggregate_key_carries_candidate_digest: bool,
    pub aggregate_key_carries_telescope_clause_path: bool,
    pub aggregate_fold_materializes_candidate_telescopes: bool,
    pub complete_candidate_content_stream_exported: bool,
    pub schema_supplies_candidate_identity_coordinate: bool,
    pub schema_supplies_clause_occurrence_coordinate: bool,
    pub schema_supplies_kernel_typed_coordinates: bool,
    pub candidate_clause_occurrence_join_expressible: bool,
    pub aggregate_surface_materialization_proved: bool,
    pub aggregate_source_executed: bool,
    pub join_artifact_read: bool,
    pub desired_verdict_read: bool,
    pub named_gap: String,
    pub exact_obstruction: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23ZeroChargeAudit {
    pub kappa_minted: Td23RegisteredNumber,
    pub semantic_family_nu_minted: Td23RegisteredNumber,
    pub anchors_minted: Td23RegisteredNumber,
    pub demand_orbits_minted: Td23RegisteredNumber,
    pub selector_input_read: bool,
    pub value_input_read: bool,
    pub bar_input_read: bool,
    pub enumeration_order_used_as_selector: bool,
    pub zero_charge: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23GateRecord {
    pub quantifier: Td23Quantifier,
    pub exact_scope: String,
    pub m3_v1_result_digest: String,
    pub m3_v1_certificate_bytes_blake3: String,
    pub m3_v1_certificate_fields_read: bool,
    pub m3_v1_remains_authoritative: bool,
    pub m3_successor_issued: bool,
    pub m4_authorized: bool,
    pub gate_evaluated: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23CandidateProvenanceKeySchemaV1Certificate {
    pub schema: String,
    pub date: String,
    pub numeric_register_policy: String,
    pub source_bindings: Vec<Td23SourceBinding>,
    pub source_surface_size: Td23RegisteredNumber,
    pub key_schema: Td23CandidateProvenanceKeySchema,
    pub source_audit: Td23StaticSourceAudit,
    pub historical_regression: Td23HistoricalRegression,
    pub zero_charge: Td23ZeroChargeAudit,
    pub open_gaps: Vec<String>,
    pub open_gap_surface_size: Td23RegisteredNumber,
    pub key_schema_frozen: bool,
    pub generic_candidate_key_issuer_available: bool,
    pub full_aggregate_candidate_key_inventory_issued: bool,
    pub join_executed: bool,
    pub desired_verdict_consulted: bool,
    pub gate: Td23GateRecord,
    pub status: Td23RunStatus,
    pub mutation_falsifiers: Vec<String>,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td23Replay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub status: Td23RunStatus,
    pub key_schema_frozen: bool,
    pub join_executed: bool,
    pub m3_v1_remains_authoritative: bool,
    pub m4_authorized: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Td23Error {
    #[error("T-D2-3 input failure: {0}")]
    Input(String),
    #[error("T-D2-3 invariant failure: {0}")]
    Invariant(String),
    #[error("T-D2-3 JSON failure: {0}")]
    Json(String),
    #[error("T-D2-3 create-new I/O failure: {0}")]
    Io(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_D2_3_CANDIDATE_PROVENANCE_KEY_SCHEMA_V1, domain, value))
        .expect("T-D2-3 evidence serializes");
    bytes_hash(&bytes)
}

fn registered_number(
    value: impl ToString,
    register: Td23Register,
    meaning: &str,
) -> Td23RegisteredNumber {
    Td23RegisteredNumber {
        decimal: value.to_string(),
        register,
        meaning: meaning.to_owned(),
    }
}

fn metadata_number(value: impl ToString, meaning: &str) -> Td23RegisteredNumber {
    registered_number(value, Td23Register::ArtifactMetadata, meaning)
}

fn syntax_number(value: impl ToString, meaning: &str) -> Td23RegisteredNumber {
    registered_number(value, Td23Register::SyntaxIdentifier, meaning)
}

fn kernel_number(value: impl ToString, meaning: &str) -> Td23RegisteredNumber {
    registered_number(value, Td23Register::KernelTypingAuthority, meaning)
}

fn source_bindings() -> Vec<Td23SourceBinding> {
    [
        (
            "docs/depth_two_domain_adjudication.md",
            "adopted T-D2-3 construction obligation and key-shopping falsifier",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/schema2_e7_e8_bridge_v1.json",
            "frozen authoritative gate artifact; bytes bound but fields not read",
            M3_V1_CERTIFICATE_BYTES,
        ),
        (
            "crates/pen-core/src/telescope.rs",
            "candidate telescope content model",
            TELESCOPE_SOURCE_BYTES,
        ),
        (
            "crates/pen-core/src/clause.rs",
            "telescope clause role and record model",
            CLAUSE_SOURCE_BYTES,
        ),
        (
            "crates/pen-core/src/expr.rs",
            "sealed expression content model",
            EXPR_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/elaborate.rs",
            "candidate digest and frozen kernel typing coordinates",
            ELABORATE_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/normalize.rs",
            "frozen typed normalizer",
            NORMALIZE_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/equality.rs",
            "frozen equality procedure used by canonical presentation",
            EQUALITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/typed_families.rs",
            "candidate subject and clause-occurrence natural-family contract",
            TYPED_FAMILIES_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/ip1_candidate_join.rs",
            "static aggregate-key and fold contract audit; never executed",
            AGGREGATE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/act_local_semantic_provenance_v5.rs",
            "candidate-identity and source-clause provenance contract",
            PROVENANCE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/t_d2_3_candidate_provenance_key_schema_v1.rs",
            "outcome-blind schema issuer and replay implementation",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Td23SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: metadata_number(bytes.len(), "exact bound byte length"),
        blake3: bytes_hash(bytes),
        register: Td23Register::ArtifactMetadata,
    })
    .collect()
}

fn schema_field(
    ordinal: usize,
    field_id: &str,
    source_authority: &str,
    exact_derivation_rule: &str,
    register: Td23Register,
) -> Td23SchemaFieldRule {
    let ordinal = syntax_number(ordinal, "schema-field ordinal; syntax identifier only");
    let derivation_hash = tagged_hash(
        "schema-field",
        &(
            &ordinal,
            field_id,
            source_authority,
            exact_derivation_rule,
            register,
        ),
    );
    Td23SchemaFieldRule {
        ordinal,
        field_id: field_id.to_owned(),
        source_authority: source_authority.to_owned(),
        exact_derivation_rule: exact_derivation_rule.to_owned(),
        register,
        required: true,
        join_outcome_dependent: false,
        selector_value_or_bar_dependent: false,
        derivation_hash,
    }
}

fn key_schema() -> Td23CandidateProvenanceKeySchema {
    let field_rules = vec![
        schema_field(
            1,
            "schema_digest",
            "versioned T-D2-3 schema definition",
            "digest of this ordered field-rule vector with its digest slot cleared",
            Td23Register::ArtifactMetadata,
        ),
        schema_field(
            2,
            "candidate_digest",
            "frozen kernel candidate_hash",
            "BLAKE3 of the serialized telescope content",
            Td23Register::ArtifactMetadata,
        ),
        schema_field(
            3,
            "telescope_clause_path",
            "sealed Telescope clause order",
            "candidate-root plus exact zero-based clause occurrence",
            Td23Register::SyntaxIdentifier,
        ),
        schema_field(
            4,
            "clause_content_digest",
            "sealed ClauseRec content",
            "BLAKE3 of the exact declared role and raw expression",
            Td23Register::ArtifactMetadata,
        ),
        schema_field(
            5,
            "signature_digest",
            "frozen SealedSignature",
            "exact signature digest used for candidate elaboration",
            Td23Register::KernelTypingAuthority,
        ),
        schema_field(
            6,
            "visible_library",
            "frozen elaborator invocation",
            "exact visible signature prefix bound supplied to elaboration",
            Td23Register::KernelTypingAuthority,
        ),
        schema_field(
            7,
            "ambient_and_free_scope",
            "frozen elaborator",
            "minimal ambient parameters and clause-local free-scope length",
            Td23Register::KernelTypingAuthority,
        ),
        schema_field(
            8,
            "candidate_elaboration_derivation",
            "frozen elaborator",
            "derivation hash returned by successful whole-candidate elaboration",
            Td23Register::KernelTypingAuthority,
        ),
        schema_field(
            9,
            "prior_kernel_role_prefix",
            "frozen elaborator",
            "digest of kernel-derived roles strictly preceding the clause",
            Td23Register::KernelTypingAuthority,
        ),
        schema_field(
            10,
            "kernel_role_and_type",
            "frozen elaborator",
            "kernel-derived role plus digest and constructor of the clause classifier",
            Td23Register::KernelTypingAuthority,
        ),
        schema_field(
            11,
            "typed_normal_form",
            "frozen normalizer through elaboration",
            "digest and constructor of the clause normal form",
            Td23Register::KernelTypingAuthority,
        ),
        schema_field(
            12,
            "canonical_presentation",
            "adopted natural-family quotient",
            "digest of the canonical normal form, parameter sorts, and checked renaming",
            Td23Register::KernelTypingAuthority,
        ),
        schema_field(
            13,
            "typed_coordinate_digest",
            "T-D2-3 typed bundle",
            "digest of every preceding typed coordinate",
            Td23Register::ProofInventory,
        ),
        schema_field(
            14,
            "key_digest",
            "T-D2-3 key issuer",
            "digest of the complete candidate-and-clause key with this slot cleared",
            Td23Register::ArtifactMetadata,
        ),
    ];
    let canonical_serialization_order = field_rules
        .iter()
        .map(|field| field.field_id.clone())
        .collect::<Vec<_>>();
    let mut schema = Td23CandidateProvenanceKeySchema {
        schema: T_D2_3_CANDIDATE_PROVENANCE_KEY_SCHEMA_V1.to_owned(),
        quantifier: Td23Quantifier::FrozenSchemaDefinition,
        exact_scope: "Generic key for one explicit candidate telescope and one exact clause occurrence, using only sealed content plus frozen kernel/normalizer derivations."
            .to_owned(),
        field_surface_size: metadata_number(
            field_rules.len(),
            "ordered schema-field surface size",
        ),
        field_rules,
        canonical_serialization_order,
        candidate_digest_required: true,
        telescope_clause_path_required: true,
        typed_coordinate_bundle_required: true,
        schema_fixed_before_candidate_regression: true,
        schema_fixed_before_any_join_outcome: true,
        generator_inventory_used_to_define_schema: false,
        aggregate_row_set_used_to_define_schema: false,
        join_outcome_used_to_define_schema: false,
        schema_digest: String::new(),
    };
    schema.schema_digest = schema_digest(&schema);
    schema
}

fn schema_digest(schema: &Td23CandidateProvenanceKeySchema) -> String {
    let mut projection = schema.clone();
    projection.schema_digest.clear();
    tagged_hash("key-schema", &projection)
}

fn expr_constructor(expr: &Expr) -> &'static str {
    match expr {
        Expr::App(_, _) => "app",
        Expr::Lam(_) => "lam",
        Expr::Pi(_, _) => "pi",
        Expr::Sigma(_, _) => "sigma",
        Expr::Univ => "univ",
        Expr::Var(_) => "var",
        Expr::Lib(_) => "lib",
        Expr::Id(_, _, _) => "id",
        Expr::Refl(_) => "refl",
        Expr::Susp(_) => "susp",
        Expr::Trunc(_) => "trunc",
        Expr::PathCon(_) => "path_con",
        Expr::Flat(_) => "flat",
        Expr::Sharp(_) => "sharp",
        Expr::Disc(_) => "disc",
        Expr::Shape(_) => "shape",
        Expr::Next(_) => "next",
        Expr::Eventually(_) => "eventually",
        Expr::Bang(_) => "bang",
        Expr::WhyNot(_) => "why_not",
    }
}

fn kernel_type_constructor(kernel_ty: &KernelTy) -> &'static str {
    match kernel_ty {
        KernelTy::Type => "type",
        KernelTy::El(_) => "el",
        KernelTy::Fun(_, _) => "fun",
        KernelTy::PathDecl { .. } => "path_decl",
        KernelTy::Neutral => "neutral",
    }
}

fn clause_path(
    candidate_digest: &str,
    clause_index: usize,
    clause: &pen_core::clause::ClauseRec,
) -> Td23TelescopeClausePath {
    let clause_index = syntax_number(
        clause_index,
        "zero-based telescope clause occurrence; syntax identifier only",
    );
    let clause_content_digest = tagged_hash("clause-content", clause);
    let raw_expression_digest = tagged_hash("raw-expression", &clause.expr);
    let raw_expression_constructor = expr_constructor(&clause.expr).to_owned();
    let path_digest = tagged_hash(
        "telescope-clause-path",
        &(
            candidate_digest,
            &clause_index,
            &clause_content_digest,
            clause.role,
            &raw_expression_digest,
            &raw_expression_constructor,
        ),
    );
    Td23TelescopeClausePath {
        root: "candidate_telescope".to_owned(),
        clause_index,
        clause_content_digest,
        declared_role: clause.role,
        raw_expression_digest,
        raw_expression_constructor,
        path_digest,
    }
}

fn key_digest(key: &Td23CandidateProvenanceKey) -> String {
    let mut projection = key.clone();
    projection.key_digest.clear();
    tagged_hash("candidate-provenance-key", &projection)
}

fn candidate_key_set_digest(key_set: &Td23CandidateKeySet) -> String {
    let mut projection = key_set.clone();
    projection.result_digest.clear();
    tagged_hash("candidate-key-set", &projection)
}

fn failure_from_clause(candidate_digest: &str, failure: &ClauseFailure) -> Td23CandidateFailure {
    Td23CandidateFailure {
        gap_id: "TD23_EXPLICIT_CANDIDATE_KERNEL_INVALID".to_owned(),
        candidate_digest: candidate_digest.to_owned(),
        failure_clause: syntax_number(
            failure.clause_index,
            "kernel failure clause occurrence; syntax identifier only",
        ),
        failure_digest: tagged_hash("candidate-kernel-failure", failure),
        exact_obstruction: failure.to_string(),
    }
}

pub fn issue_t_d2_3_candidate_keys(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
) -> Td23CandidateKeySet {
    let schema = key_schema();
    let subject = candidate_hash(candidate);
    let exact_scope = "One explicit candidate telescope at one explicit frozen signature-prefix bound; no aggregate row, join result, score, or selector is an input."
        .to_owned();
    let mut result = match elaborate_telescope(signature, candidate, visible_library) {
        Err(failure) => Td23CandidateKeySet {
            schema: T_D2_3_CANDIDATE_PROVENANCE_KEY_SCHEMA_V1.to_owned(),
            quantifier: Td23Quantifier::GenericExplicitCandidate,
            exact_scope,
            schema_digest: schema.schema_digest,
            signature_digest: signature.digest().to_owned(),
            visible_library: kernel_number(
                visible_library,
                "visible sealed signature prefix used by the kernel",
            ),
            candidate_digest: subject.clone(),
            candidate_clause_surface_size: metadata_number(
                candidate.clauses.len(),
                "explicit candidate clause surface size",
            ),
            key_surface_size: metadata_number(0, "issued key surface size"),
            typed: false,
            keys: Vec::new(),
            failure: Some(failure_from_clause(&subject, &failure)),
            every_clause_has_one_key: false,
            duplicate_key_digest_absent: true,
            selector_value_or_bar_input_read: false,
            result_digest: String::new(),
        },
        Ok(elaboration) => {
            let prior_roles = elaboration
                .clauses
                .iter()
                .map(|clause| clause.kernel_role)
                .collect::<Vec<_>>();
            let mut keys = Vec::with_capacity(candidate.clauses.len());
            for (raw_clause, typed_clause) in candidate.clauses.iter().zip(&elaboration.clauses) {
                let index = usize::from(typed_clause.clause_index);
                let path = clause_path(&subject, index, raw_clause);
                let free_scope =
                    elaboration.ambient_parameters + u32::from(typed_clause.clause_index);
                let presentation = clause_presentation(
                    &typed_clause.normal_form,
                    free_scope,
                    &prior_roles[..index],
                    elaboration.ambient_parameters,
                );
                let kernel_type_constructor =
                    kernel_type_constructor(&typed_clause.kernel_ty).to_owned();
                let kernel_type_digest = tagged_hash("kernel-type", &typed_clause.kernel_ty);
                let normal_form_constructor =
                    expr_constructor(&typed_clause.normal_form).to_owned();
                let normal_form_digest =
                    tagged_hash("typed-normal-form", &typed_clause.normal_form);
                let canonical_presentation_digest =
                    tagged_hash("canonical-presentation", &presentation);
                let parameter_sort_digest =
                    tagged_hash("canonical-parameter-sorts", &presentation.parameters);
                let renaming_map_digest =
                    tagged_hash("canonical-renaming-map", &presentation.renaming);
                let clause_derivation_digest =
                    tagged_hash("clause-elaboration-derivation", &typed_clause.derivation);
                let prior_kernel_role_prefix_digest =
                    tagged_hash("prior-kernel-role-prefix", &prior_roles[..index]);
                let typed_coordinate_digest = tagged_hash(
                    "typed-coordinate-bundle",
                    &(
                        signature.digest(),
                        visible_library,
                        elaboration.ambient_parameters,
                        free_scope,
                        elaboration.derivation_hash.as_str(),
                        &clause_derivation_digest,
                        &prior_kernel_role_prefix_digest,
                        typed_clause.kernel_role,
                        &kernel_type_constructor,
                        &kernel_type_digest,
                        &normal_form_constructor,
                        &normal_form_digest,
                        &canonical_presentation_digest,
                        &parameter_sort_digest,
                        &renaming_map_digest,
                    ),
                );
                let typed_coordinates = Td23TypedCoordinateBundle {
                    signature_digest: signature.digest().to_owned(),
                    visible_library: kernel_number(
                        visible_library,
                        "visible sealed signature prefix used by the kernel",
                    ),
                    ambient_parameters: kernel_number(
                        elaboration.ambient_parameters,
                        "minimal ambient parameter count derived by the kernel",
                    ),
                    free_scope_length: kernel_number(
                        free_scope,
                        "clause-local free scope derived by the kernel",
                    ),
                    candidate_elaboration_derivation_hash: elaboration.derivation_hash.clone(),
                    clause_derivation_digest,
                    prior_kernel_role_prefix_digest,
                    kernel_role: typed_clause.kernel_role,
                    kernel_type_constructor,
                    kernel_type_digest,
                    normal_form_constructor,
                    normal_form_digest,
                    canonical_presentation_digest,
                    parameter_sort_digest,
                    renaming_map_digest,
                    typed_coordinate_digest,
                };
                let mut key = Td23CandidateProvenanceKey {
                    schema_digest: schema.schema_digest.clone(),
                    candidate_digest: subject.clone(),
                    clause_path: path,
                    typed_coordinates,
                    key_digest: String::new(),
                };
                key.key_digest = key_digest(&key);
                keys.push(key);
            }
            let distinct = keys
                .iter()
                .map(|key| key.key_digest.clone())
                .collect::<BTreeSet<_>>();
            Td23CandidateKeySet {
                schema: T_D2_3_CANDIDATE_PROVENANCE_KEY_SCHEMA_V1.to_owned(),
                quantifier: Td23Quantifier::GenericExplicitCandidate,
                exact_scope,
                schema_digest: schema.schema_digest,
                signature_digest: signature.digest().to_owned(),
                visible_library: kernel_number(
                    visible_library,
                    "visible sealed signature prefix used by the kernel",
                ),
                candidate_digest: subject,
                candidate_clause_surface_size: metadata_number(
                    candidate.clauses.len(),
                    "explicit candidate clause surface size",
                ),
                key_surface_size: metadata_number(keys.len(), "issued key surface size"),
                typed: true,
                every_clause_has_one_key: keys.len() == candidate.clauses.len(),
                duplicate_key_digest_absent: distinct.len() == keys.len(),
                keys,
                failure: None,
                selector_value_or_bar_input_read: false,
                result_digest: String::new(),
            }
        }
    };
    result.result_digest = candidate_key_set_digest(&result);
    result
}

pub fn replay_t_d2_3_candidate_keys(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    claimed: &Td23CandidateKeySet,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.result_digest != candidate_key_set_digest(claimed) {
        errors.push("T-D2-3 candidate key-set digest mismatch".to_owned());
    }
    let expected = issue_t_d2_3_candidate_keys(signature, candidate, visible_library);
    if expected != *claimed {
        errors.push("T-D2-3 candidate key set differs from deterministic reissuance".to_owned());
    }
    errors
}

fn source_text<'a>(label: &str, bytes: &'a [u8]) -> Result<&'a str, Td23Error> {
    std::str::from_utf8(bytes)
        .map_err(|error| Td23Error::Input(format!("{label} is not UTF-8: {error}")))
}

fn source_section<'a>(
    label: &str,
    source: &'a str,
    start: &str,
    end: &str,
) -> Result<&'a str, Td23Error> {
    let start_index = source
        .find(start)
        .ok_or_else(|| Td23Error::Input(format!("{label} lacks `{start}`")))?;
    let tail = &source[start_index..];
    let end_index = tail
        .find(end)
        .ok_or_else(|| Td23Error::Input(format!("{label} lacks `{end}` after `{start}`")))?;
    Ok(&tail[..end_index])
}

fn build_source_audit(
    schema: &Td23CandidateProvenanceKeySchema,
) -> Result<Td23StaticSourceAudit, Td23Error> {
    let aggregate = source_text("aggregate source", AGGREGATE_SOURCE_BYTES)?;
    let elaborate = source_text("candidate kernel source", ELABORATE_SOURCE_BYTES)?;
    let families = source_text("typed-family source", TYPED_FAMILIES_SOURCE_BYTES)?;
    let provenance = source_text("provenance source", PROVENANCE_SOURCE_BYTES)?;
    let aggregate_key = source_section(
        "aggregate source",
        aggregate,
        "struct ExprJoinKey {",
        "fn classify_position_exprs(",
    )?;
    let aggregate_fold = source_section(
        "aggregate source",
        aggregate,
        "fn exhaust_stratum(",
        "fn certificate_digest(",
    )?;
    let candidate_hash_contract_found = elaborate
        .contains("pub fn candidate_hash(telescope: &Telescope)")
        && elaborate.contains("serde_json::to_vec(telescope)");
    let typed_clause_occurrence_contract_found = families.contains("pub struct InstanceRecord")
        && families.contains("pub clause_index: u16")
        && families.contains("pub subject_hash: String");
    let provenance_candidate_identity_contract_found =
        provenance.contains("pub candidate_hash: String");
    let provenance_clause_occurrence_contract_found =
        provenance.contains("pub source_clauses: Vec<u16>");
    let aggregate_key_carries_candidate_digest =
        aggregate_key.contains("candidate_hash") || aggregate_key.contains("candidate_digest");
    let aggregate_key_carries_telescope_clause_path =
        aggregate_key.contains("clause_index") || aggregate_key.contains("clause_path");
    let aggregate_fold_materializes_candidate_telescopes =
        aggregate_fold.contains("Telescope") || aggregate_fold.contains("candidate_hash(");
    let complete_candidate_content_stream_exported = false;
    let schema_supplies_candidate_identity_coordinate = schema.candidate_digest_required;
    let schema_supplies_clause_occurrence_coordinate = schema.telescope_clause_path_required;
    let schema_supplies_kernel_typed_coordinates = schema.typed_coordinate_bundle_required;
    let candidate_clause_occurrence_join_expressible = candidate_hash_contract_found
        && typed_clause_occurrence_contract_found
        && provenance_candidate_identity_contract_found
        && provenance_clause_occurrence_contract_found
        && schema_supplies_candidate_identity_coordinate
        && schema_supplies_clause_occurrence_coordinate
        && schema_supplies_kernel_typed_coordinates;
    let aggregate_surface_materialization_proved = complete_candidate_content_stream_exported
        && aggregate_key_carries_candidate_digest
        && aggregate_key_carries_telescope_clause_path
        && aggregate_fold_materializes_candidate_telescopes;
    if !candidate_clause_occurrence_join_expressible
        || aggregate_key_carries_candidate_digest
        || aggregate_key_carries_telescope_clause_path
        || aggregate_fold_materializes_candidate_telescopes
        || aggregate_surface_materialization_proved
    {
        return Err(Td23Error::Invariant(
            "static candidate/provenance source contracts drifted".to_owned(),
        ));
    }
    let exact_scope = "Static data-contract slices only: the aggregate key declaration and aggregate fold are hashed and inspected for payload coordinates; no aggregate computation or outcome artifact is evaluated."
        .to_owned();
    let exact_obstruction = "The current aggregate dynamic program maps expression classes into aggregate rows and does not export a complete stream of candidate Telescope payloads. The frozen T-D2-3 key schema can issue one replayable key per clause for any explicit typed candidate, but it cannot materialize keys for candidate payloads the aggregate API does not expose."
        .to_owned();
    let derivation_hash = tagged_hash(
        "static-source-audit",
        &(
            &exact_scope,
            bytes_hash(aggregate_key.as_bytes()),
            bytes_hash(aggregate_fold.as_bytes()),
            candidate_hash_contract_found,
            typed_clause_occurrence_contract_found,
            provenance_candidate_identity_contract_found,
            provenance_clause_occurrence_contract_found,
            aggregate_key_carries_candidate_digest,
            aggregate_key_carries_telescope_clause_path,
            aggregate_fold_materializes_candidate_telescopes,
            candidate_clause_occurrence_join_expressible,
            &exact_obstruction,
        ),
    );
    Ok(Td23StaticSourceAudit {
        quantifier: Td23Quantifier::StaticSourceContract,
        exact_scope,
        aggregate_key_section_digest: bytes_hash(aggregate_key.as_bytes()),
        aggregate_fold_section_digest: bytes_hash(aggregate_fold.as_bytes()),
        candidate_hash_contract_found,
        typed_clause_occurrence_contract_found,
        provenance_candidate_identity_contract_found,
        provenance_clause_occurrence_contract_found,
        aggregate_key_carries_candidate_digest,
        aggregate_key_carries_telescope_clause_path,
        aggregate_fold_materializes_candidate_telescopes,
        complete_candidate_content_stream_exported,
        schema_supplies_candidate_identity_coordinate,
        schema_supplies_clause_occurrence_coordinate,
        schema_supplies_kernel_typed_coordinates,
        candidate_clause_occurrence_join_expressible,
        aggregate_surface_materialization_proved,
        aggregate_source_executed: false,
        join_artifact_read: false,
        desired_verdict_read: false,
        named_gap: T_D2_3_AGGREGATE_STREAM_GAP.to_owned(),
        exact_obstruction,
        derivation_hash,
    })
}

fn build_historical_regression(
    schema: &Td23CandidateProvenanceKeySchema,
) -> Result<Td23HistoricalRegression, Td23Error> {
    let signature = SealedSignature::genesis_del_h15();
    let mut rows = Vec::new();
    for entry in signature.entries() {
        let visible_library = entry.step.saturating_sub(1);
        let key_set = issue_t_d2_3_candidate_keys(&signature, &entry.telescope, visible_library);
        let sealed_candidate_digest_exact = key_set.candidate_digest == entry.candidate_hash;
        let clause_key_bijection = key_set.typed
            && key_set.every_clause_has_one_key
            && key_set.keys.len() == entry.telescope.clauses.len();
        if !sealed_candidate_digest_exact
            || !clause_key_bijection
            || key_set.schema_digest != schema.schema_digest
            || !replay_t_d2_3_candidate_keys(
                &signature,
                &entry.telescope,
                visible_library,
                &key_set,
            )
            .is_empty()
        {
            return Err(Td23Error::Invariant(format!(
                "sealed candidate key regression failed at entry {}",
                entry.step
            )));
        }
        let exact_scope = format!(
            "Exact sealed historical candidate entry `{}` at its predecessor signature prefix.",
            entry.step
        );
        let derivation_hash = tagged_hash(
            "historical-regression-row",
            &(
                entry.step,
                &exact_scope,
                &entry.candidate_hash,
                visible_library,
                &key_set,
            ),
        );
        rows.push(Td23HistoricalRegressionRow {
            entry_identifier: syntax_number(
                entry.step,
                "sealed historical entry identifier; syntax identifier only",
            ),
            quantifier: Td23Quantifier::ExactSealedHistoricalCandidate,
            exact_scope,
            sealed_candidate_digest: entry.candidate_hash.clone(),
            visible_library: kernel_number(
                visible_library,
                "visible predecessor signature prefix used by the kernel",
            ),
            key_set,
            sealed_candidate_digest_exact,
            clause_key_bijection,
            derivation_hash,
        });
    }
    let clause_surface = rows
        .iter()
        .map(|row| {
            row.key_set
                .candidate_clause_surface_size
                .decimal
                .parse::<usize>()
        })
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| Td23Error::Invariant(error.to_string()))?
        .into_iter()
        .sum::<usize>();
    let key_surface = rows.iter().map(|row| row.key_set.keys.len()).sum::<usize>();
    let every_sealed_candidate_typed = rows.iter().all(|row| row.key_set.typed);
    let every_sealed_clause_keyed =
        rows.iter().all(|row| row.clause_key_bijection) && clause_surface == key_surface;
    let key_digest_unique_within_candidate = rows.iter().all(|row| {
        row.key_set
            .keys
            .iter()
            .map(|key| key.key_digest.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            == row.key_set.keys.len()
    });
    let schema_digest_constant_across_candidates = rows
        .iter()
        .all(|row| row.key_set.schema_digest == schema.schema_digest);
    let regression_complete = !rows.is_empty()
        && every_sealed_candidate_typed
        && every_sealed_clause_keyed
        && key_digest_unique_within_candidate
        && schema_digest_constant_across_candidates;
    if !regression_complete {
        return Err(Td23Error::Invariant(
            "sealed candidate regression is incomplete".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "historical-regression",
        &(
            signature.digest(),
            &rows,
            clause_surface,
            key_surface,
            regression_complete,
        ),
    );
    Ok(Td23HistoricalRegression {
        signature_digest: signature.digest().to_owned(),
        candidate_surface_size: metadata_number(
            rows.len(),
            "sealed historical candidate regression surface size",
        ),
        clause_surface_size: metadata_number(
            clause_surface,
            "sealed historical clause regression surface size",
        ),
        key_surface_size: metadata_number(
            key_surface,
            "issued historical key regression surface size",
        ),
        rows,
        every_sealed_candidate_typed,
        every_sealed_clause_keyed,
        key_digest_unique_within_candidate,
        schema_digest_constant_across_candidates,
        regression_complete,
        derivation_hash,
    })
}

fn certificate_digest(certificate: &Td23CandidateProvenanceKeySchemaV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn build_certificate() -> Result<Td23CandidateProvenanceKeySchemaV1Certificate, Td23Error> {
    let adjudication = source_text("depth-two adjudication", ADJUDICATION_BYTES)?;
    if !adjudication.contains("depth-two-semantic-domain-v1")
        || !adjudication.contains("T-D2-3 (BC2 provenance-bearing keys)")
        || !adjudication.contains("**Adopted.**")
    {
        return Err(Td23Error::Input(
            "adopted T-D2-3 adjudication text did not authenticate".to_owned(),
        ));
    }

    // F-D2-4 order: the schema is frozen before any candidate regression
    // or source integration audit is constructed.  No join API exists in
    // this module's dependency graph.
    let key_schema = key_schema();
    let source_audit = build_source_audit(&key_schema)?;
    let historical_regression = build_historical_regression(&key_schema)?;
    let zero_fields = (0usize, 0usize, 0usize, 0usize, false, false, false, false);
    let zero_charge = Td23ZeroChargeAudit {
        kappa_minted: metadata_number(zero_fields.0, "kappa minted by key description"),
        semantic_family_nu_minted: metadata_number(
            zero_fields.1,
            "semantic-family nu minted by key description",
        ),
        anchors_minted: metadata_number(zero_fields.2, "anchors minted by key description"),
        demand_orbits_minted: metadata_number(
            zero_fields.3,
            "demand orbits minted by key description",
        ),
        selector_input_read: zero_fields.4,
        value_input_read: zero_fields.5,
        bar_input_read: zero_fields.6,
        enumeration_order_used_as_selector: zero_fields.7,
        zero_charge: true,
        derivation_hash: tagged_hash("zero-charge", &zero_fields),
    };
    let gate_scope = "Non-action record only: this schema issuer neither parses the authoritative bridge certificate nor evaluates or changes its gate."
        .to_owned();
    let gate = Td23GateRecord {
        quantifier: Td23Quantifier::FrozenGateNonAction,
        exact_scope: gate_scope.clone(),
        m3_v1_result_digest: FROZEN_M3_V1_RESULT_DIGEST.to_owned(),
        m3_v1_certificate_bytes_blake3: bytes_hash(M3_V1_CERTIFICATE_BYTES),
        m3_v1_certificate_fields_read: false,
        m3_v1_remains_authoritative: true,
        m3_successor_issued: false,
        m4_authorized: false,
        gate_evaluated: false,
        derivation_hash: tagged_hash(
            "gate-non-action",
            &(
                gate_scope,
                FROZEN_M3_V1_RESULT_DIGEST,
                bytes_hash(M3_V1_CERTIFICATE_BYTES),
                false,
                true,
                false,
                false,
                false,
            ),
        ),
    };
    let bindings = source_bindings();
    let open_gaps = vec![source_audit.named_gap.clone()];
    let mut certificate = Td23CandidateProvenanceKeySchemaV1Certificate {
        schema: T_D2_3_CANDIDATE_PROVENANCE_KEY_SCHEMA_V1.to_owned(),
        date: T_D2_3_CANDIDATE_PROVENANCE_KEY_SCHEMA_V1_DATE.to_owned(),
        numeric_register_policy: "Every public quantitative datum is a tagged decimal value. Decimal substrings in dates, schema versions, theorem labels, candidate digests, and clause paths are syntax identifiers; predicates make no hidden quantitative claim."
            .to_owned(),
        source_surface_size: metadata_number(
            bindings.len(),
            "exact bound source surface size",
        ),
        source_bindings: bindings,
        key_schema,
        source_audit,
        historical_regression,
        zero_charge,
        open_gap_surface_size: metadata_number(
            open_gaps.len(),
            "remaining named integration-gap surface size",
        ),
        open_gaps,
        key_schema_frozen: true,
        generic_candidate_key_issuer_available: true,
        full_aggregate_candidate_key_inventory_issued: false,
        join_executed: false,
        desired_verdict_consulted: false,
        gate,
        status: Td23RunStatus::SchemaFrozenNamedIntegrationGap,
        mutation_falsifiers: vec![
            "change_any_schema_field_order_rule_or_dependency_then_replay_must_fail".to_owned(),
            "change_any_candidate_digest_clause_path_or_typed_coordinate_then_replay_must_fail"
                .to_owned(),
            "make_schema_or_key_depend_on_any_join_outcome_selector_value_or_bar_then_replay_must_fail"
                .to_owned(),
            "claim_aggregate_candidate_coverage_without_a_complete_content_stream_then_replay_must_fail"
                .to_owned(),
            "remove_or_rename_the_integration_gap_then_replay_must_fail".to_owned(),
            "mint_any_credit_anchor_or_demand_orbit_then_replay_must_fail".to_owned(),
            "change_the_gate_non_action_record_then_replay_must_fail".to_owned(),
            "change_any_bound_source_byte_or_digest_then_replay_must_fail".to_owned(),
            "add_any_unknown_JSON_field_then_replay_must_fail".to_owned(),
        ],
        permitted_conclusion: "The versioned candidate/provenance key schema is frozen outcome-blind. For every explicit candidate accepted by the frozen kernel, it deterministically issues one key per telescope clause, carrying candidate digest, exact clause path, and kernel-derived typed coordinates. The sealed historical regression is complete. The current aggregate API does not export the complete candidate-content stream needed to materialize this schema over its aggregate surface, so no candidate-level join or full aggregate inventory is claimed."
            .to_owned(),
        required_successor_action: "Add a versioned candidate-content stream or a lossless candidate-preserving refinement to the aggregate generator, feed each explicit candidate through the already-frozen T-D2-3 key issuer, and only afterward run a separately versioned candidate-level join. The key schema may not be revised in response to that join."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

static EXPECTED_CERTIFICATE: OnceLock<
    Result<Td23CandidateProvenanceKeySchemaV1Certificate, String>,
> = OnceLock::new();

fn expected_certificate()
-> Result<&'static Td23CandidateProvenanceKeySchemaV1Certificate, Td23Error> {
    match EXPECTED_CERTIFICATE
        .get_or_init(|| build_certificate().map_err(|error| error.to_string()))
    {
        Ok(certificate) => Ok(certificate),
        Err(error) => Err(Td23Error::Input(error.clone())),
    }
}

pub fn issue_t_d2_3_candidate_provenance_key_schema_v1()
-> Result<Td23CandidateProvenanceKeySchemaV1Certificate, Td23Error> {
    expected_certificate().cloned()
}

fn invalid_replay(error: impl Into<String>) -> Td23Replay {
    Td23Replay {
        valid: false,
        errors: vec![error.into()],
        status: Td23RunStatus::SchemaFrozenNamedIntegrationGap,
        key_schema_frozen: false,
        join_executed: false,
        m3_v1_remains_authoritative: true,
        m4_authorized: false,
    }
}

pub fn replay_t_d2_3_candidate_provenance_key_schema_v1(
    claimed: &Td23CandidateProvenanceKeySchemaV1Certificate,
) -> Td23Replay {
    let expected = match expected_certificate() {
        Ok(certificate) => certificate,
        Err(error) => return invalid_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("T-D2-3 certificate digest mismatch".to_owned());
    }
    if claimed.source_bindings != source_bindings() {
        errors.push("T-D2-3 source bindings drifted".to_owned());
    }
    if claimed != expected {
        errors.push("T-D2-3 certificate differs from deterministic reissuance".to_owned());
    }
    Td23Replay {
        valid: errors.is_empty(),
        errors,
        status: claimed.status,
        key_schema_frozen: claimed.key_schema_frozen,
        join_executed: claimed.join_executed,
        m3_v1_remains_authoritative: claimed.gate.m3_v1_remains_authoritative,
        m4_authorized: claimed.gate.m4_authorized,
    }
}

pub fn replay_t_d2_3_candidate_provenance_key_schema_v1_json(json: &str) -> Td23Replay {
    match serde_json::from_str::<Td23CandidateProvenanceKeySchemaV1Certificate>(json) {
        Ok(certificate) => replay_t_d2_3_candidate_provenance_key_schema_v1(&certificate),
        Err(error) => invalid_replay(format!("invalid T-D2-3 JSON: {error}")),
    }
}

fn register_label(register: Td23Register) -> &'static str {
    match register {
        Td23Register::KernelTypingAuthority => "kernel_typing_authority",
        Td23Register::StructuralTestimony => "structural_testimony",
        Td23Register::ProofInventory => "proof_inventory",
        Td23Register::ArtifactMetadata => "artifact_metadata",
        Td23Register::SyntaxIdentifier => "syntax_identifier",
    }
}

fn render_number(number: &Td23RegisteredNumber) -> String {
    format!(
        "{} [register: `{}`; meaning: {}]",
        number.decimal,
        register_label(number.register),
        number.meaning
    )
}

pub fn render_t_d2_3_candidate_provenance_key_schema_v1(
    certificate: &Td23CandidateProvenanceKeySchemaV1Certificate,
) -> String {
    let mut out = String::new();
    out.push_str("# T-D2-3 candidate/provenance key-schema result\n\n");
    out.push_str(&format!(
        "**Date (syntax identifier):** {}. **Status:** `schema_frozen_named_integration_gap`. **Certificate:** `{}`.\n\n",
        certificate.date, certificate.result_digest
    ));
    out.push_str("The key schema is frozen before any candidate-level join is run. No join artifact or desired verdict is an input. M-3 v1 remains authoritative and M-4 remains unauthorized.\n\n");
    out.push_str("## Frozen key schema\n\n");
    out.push_str(&format!(
        "Schema digest: `{}`. Ordered field surface: **{}**. Candidate digest, telescope clause path, and the typed coordinate bundle are mandatory. Every field rule is outcome-independent and selector/value/bar-independent.\n\n",
        certificate.key_schema.schema_digest,
        render_number(&certificate.key_schema.field_surface_size),
    ));
    out.push_str("| Field | Source authority | Exact derivation |\n|---|---|---|\n");
    for field in &certificate.key_schema.field_rules {
        out.push_str(&format!(
            "| `{}` | {} | {} |\n",
            field.field_id, field.source_authority, field.exact_derivation_rule
        ));
    }
    out.push_str("\n## Sealed-content regression\n\n");
    out.push_str(&format!(
        "Candidates: **{}**. Clauses: **{}**. Issued keys: **{}**. Every sealed candidate is typed, every clause has one key, key digests are unique within each candidate, and the schema digest is constant across the regression.\n\n",
        render_number(&certificate.historical_regression.candidate_surface_size),
        render_number(&certificate.historical_regression.clause_surface_size),
        render_number(&certificate.historical_regression.key_surface_size),
    ));
    out.push_str("| Entry | Candidate digest | Clause keys | Replay |\n|---|---|---|---|\n");
    for row in &certificate.historical_regression.rows {
        out.push_str(&format!(
            "| {} | `{}` | {} | {} |\n",
            render_number(&row.entry_identifier),
            row.sealed_candidate_digest,
            render_number(&row.key_set.key_surface_size),
            row.clause_key_bijection,
        ));
    }
    out.push_str("\n## Static integration audit\n\n");
    out.push_str("The candidate kernel exposes the telescope digest; typed-family and provenance modules address occurrences by candidate identity plus clause coordinate. The new schema supplies those coordinates and their kernel derivations. The aggregate key and fold were audited as source contracts only and were not executed.\n\n");
    out.push_str(&format!(
        "Remaining gap: `{}` — {}\n\n",
        certificate.source_audit.named_gap, certificate.source_audit.exact_obstruction
    ));
    out.push_str("## Zero charge and gate\n\n");
    out.push_str(&format!(
        "Minted kappa: **{}**; semantic-family nu: **{}**; anchors: **{}**; demand orbits: **{}**. No selector, value, bar, or enumeration-order selector was read. M-3 successor issued: **{}**; M-4 authorized: **{}**.\n\n",
        render_number(&certificate.zero_charge.kappa_minted),
        render_number(&certificate.zero_charge.semantic_family_nu_minted),
        render_number(&certificate.zero_charge.anchors_minted),
        render_number(&certificate.zero_charge.demand_orbits_minted),
        certificate.gate.m3_successor_issued,
        certificate.gate.m4_authorized,
    ));
    out.push_str("## Permitted conclusion\n\n");
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

pub fn emit_t_d2_3_candidate_provenance_key_schema_v1_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Td23CandidateProvenanceKeySchemaV1Certificate, Td23Error> {
    if certificate_path == report_path {
        return Err(Td23Error::Io(
            "certificate and report targets must be distinct".to_owned(),
        ));
    }
    if certificate_path.exists() || report_path.exists() {
        return Err(Td23Error::Io(
            "create-new target already exists; no artifact was overwritten".to_owned(),
        ));
    }
    let certificate = issue_t_d2_3_candidate_provenance_key_schema_v1()?;
    let replay = replay_t_d2_3_candidate_provenance_key_schema_v1(&certificate);
    if !replay.valid {
        return Err(Td23Error::Invariant(format!(
            "new T-D2-3 certificate did not replay: {}",
            replay.errors.join("; ")
        )));
    }
    let mut json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Td23Error::Json(error.to_string()))?;
    json.push(b'\n');
    let report = render_t_d2_3_candidate_provenance_key_schema_v1(&certificate);
    let mut certificate_created = false;
    let mut report_created = false;
    let result = (|| -> Result<(), Td23Error> {
        let mut certificate_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(certificate_path)
            .map_err(|error| Td23Error::Io(error.to_string()))?;
        certificate_created = true;
        certificate_file
            .write_all(&json)
            .and_then(|_| certificate_file.sync_all())
            .map_err(|error| Td23Error::Io(error.to_string()))?;
        drop(certificate_file);

        let mut report_file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(report_path)
            .map_err(|error| Td23Error::Io(error.to_string()))?;
        report_created = true;
        report_file
            .write_all(report.as_bytes())
            .and_then(|_| report_file.sync_all())
            .map_err(|error| Td23Error::Io(error.to_string()))?;
        drop(report_file);

        let emitted_json =
            read_to_string(certificate_path).map_err(|error| Td23Error::Io(error.to_string()))?;
        let emitted_report = read(report_path).map_err(|error| Td23Error::Io(error.to_string()))?;
        if emitted_json.as_bytes() != json.as_slice() {
            return Err(Td23Error::Invariant(
                "emitted T-D2-3 certificate bytes differ from create-new payload".to_owned(),
            ));
        }
        if emitted_report.as_slice() != report.as_bytes() {
            return Err(Td23Error::Invariant(
                "emitted T-D2-3 report differs from deterministic rendering".to_owned(),
            ));
        }
        let emitted_replay = replay_t_d2_3_candidate_provenance_key_schema_v1_json(&emitted_json);
        if !emitted_replay.valid {
            return Err(Td23Error::Invariant(format!(
                "emitted T-D2-3 certificate did not replay: {}",
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
        Td23CandidateProvenanceKeySchemaV1Certificate, Td23Quantifier, Td23RunStatus,
        certificate_digest, issue_t_d2_3_candidate_keys,
        issue_t_d2_3_candidate_provenance_key_schema_v1, replay_t_d2_3_candidate_keys,
        replay_t_d2_3_candidate_provenance_key_schema_v1,
        replay_t_d2_3_candidate_provenance_key_schema_v1_json,
    };
    use pen_core::clause::{ClauseRec, ClauseRole};
    use pen_core::expr::Expr;
    use pen_core::telescope::Telescope;
    use pen_type::elaborate::SealedSignature;

    fn reseal(certificate: &mut Td23CandidateProvenanceKeySchemaV1Certificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    #[test]
    fn freezes_outcome_blind_schema_and_named_integration_gap() {
        let certificate = issue_t_d2_3_candidate_provenance_key_schema_v1().expect("T-D2-3 issues");
        let replay = replay_t_d2_3_candidate_provenance_key_schema_v1(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(
            certificate.status,
            Td23RunStatus::SchemaFrozenNamedIntegrationGap
        );
        assert!(certificate.key_schema_frozen);
        assert!(certificate.generic_candidate_key_issuer_available);
        assert!(!certificate.full_aggregate_candidate_key_inventory_issued);
        assert!(!certificate.join_executed);
        assert!(!certificate.desired_verdict_consulted);
        assert!(certificate.key_schema.schema_fixed_before_any_join_outcome);
        assert!(
            certificate.key_schema.field_rules.iter().all(
                |field| !field.join_outcome_dependent && !field.selector_value_or_bar_dependent
            )
        );
        assert_eq!(
            certificate.open_gaps,
            vec![super::T_D2_3_AGGREGATE_STREAM_GAP]
        );
        assert!(certificate.gate.m3_v1_remains_authoritative);
        assert!(!certificate.gate.m4_authorized);
    }

    #[test]
    fn generic_candidate_key_replay_is_content_and_clause_sensitive() {
        let signature = SealedSignature::genesis_del_h15();
        let candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ]);
        let keys = issue_t_d2_3_candidate_keys(&signature, &candidate, 15);
        assert!(keys.typed);
        assert!(keys.every_clause_has_one_key);
        assert_eq!(keys.keys.len(), candidate.clauses.len());
        assert!(replay_t_d2_3_candidate_keys(&signature, &candidate, 15, &keys).is_empty());

        let mutated = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(ClauseRole::Introduction, Expr::Lib(1)),
        ]);
        assert!(!replay_t_d2_3_candidate_keys(&signature, &mutated, 15, &keys).is_empty());
        let mutated_keys = issue_t_d2_3_candidate_keys(&signature, &mutated, 15);
        assert_eq!(keys.schema_digest, mutated_keys.schema_digest);
        assert_ne!(keys.candidate_digest, mutated_keys.candidate_digest);
        assert_ne!(keys.keys[1].key_digest, mutated_keys.keys[1].key_digest);
    }

    #[test]
    fn schema_key_source_gap_charge_and_gate_mutations_fail_after_reseal() {
        let certificate = issue_t_d2_3_candidate_provenance_key_schema_v1().expect("T-D2-3 issues");

        let mut schema = certificate.clone();
        schema.key_schema.field_rules.swap(0, 1);
        reseal(&mut schema);
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&schema).valid);

        let mut key = certificate.clone();
        key.historical_regression.rows[0].key_set.keys[0]
            .candidate_digest
            .push('0');
        reseal(&mut key);
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&key).valid);

        let mut path = certificate.clone();
        path.historical_regression.rows[0].key_set.keys[0]
            .clause_path
            .path_digest
            .push('0');
        reseal(&mut path);
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&path).valid);

        let mut typed = certificate.clone();
        typed.historical_regression.rows[0].key_set.keys[0]
            .typed_coordinates
            .normal_form_digest
            .push('0');
        reseal(&mut typed);
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&typed).valid);

        let mut source = certificate.clone();
        source.source_bindings[0].blake3.push('0');
        reseal(&mut source);
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&source).valid);

        let mut outcome = certificate.clone();
        outcome.source_audit.desired_verdict_read = true;
        reseal(&mut outcome);
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&outcome).valid);

        let mut coverage = certificate.clone();
        coverage.full_aggregate_candidate_key_inventory_issued = true;
        reseal(&mut coverage);
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&coverage).valid);

        let mut gap = certificate.clone();
        gap.open_gaps.clear();
        reseal(&mut gap);
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&gap).valid);

        let mut charge = certificate.clone();
        charge.zero_charge.anchors_minted.decimal = "1".to_owned();
        reseal(&mut charge);
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&charge).valid);

        let mut gate = certificate.clone();
        gate.gate.m4_authorized = true;
        reseal(&mut gate);
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&gate).valid);

        let mut quantifier = certificate.clone();
        quantifier.historical_regression.rows[0].quantifier = Td23Quantifier::FrozenGateNonAction;
        reseal(&mut quantifier);
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&quantifier).valid);

        let mut digest = certificate;
        digest.result_digest.push('0');
        assert!(!replay_t_d2_3_candidate_provenance_key_schema_v1(&digest).valid);
    }

    #[test]
    fn unknown_json_fields_fail_closed() {
        let certificate = issue_t_d2_3_candidate_provenance_key_schema_v1().expect("T-D2-3 issues");
        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("object")
            .insert("unknown_field".to_owned(), serde_json::Value::Bool(true));
        let replay = replay_t_d2_3_candidate_provenance_key_schema_v1_json(
            &serde_json::to_string(&value).expect("json"),
        );
        assert!(!replay.valid);
    }
}
