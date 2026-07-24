//! DNF-Q v1: dependent-natural-family-quotient theorem-layer attempt.
//!
//! This module is deliberately fail-closed.  It preserves every piece of
//! reconstructible content from the sealed artifacts, but it never promotes a
//! digest, count, label, or archived verdict into a missing context,
//! substitution, or naturality theorem.

use crate::e5_future_hole_finale_v2::{
    replay_e5_future_hole_finale_v2_certificate, E5FutureHoleFinaleV2Certificate,
};
use crate::naturality_orbit_transport::{
    replay_naturality_orbit_transport_certificate, A3TransportedInstance,
    NaturalityOrbitTransportCertificate,
};
use crate::t_bi_nu1_regression_v5::replay_t_bi_nu1_regression_v5_json;
use crate::t_bi_nu1_regression_v6::replay_t_bi_nu1_regression_v6_json;
use crate::t_d2_1_operational_domain_v2::{
    derive_t_d2_1_contextual_membership_v2, replay_t_d2_1_operational_domain_v2,
    Td21OperationalDomainV2Certificate, Td21V2KernelClassifier, Td21V2OperationalNode,
    Td21V2ParameterTelescope, Td21V2SyntaxNumber,
};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_eval::a3_rule_inventory_exhaustiveness::replay_historical_a3_rule_inventory_exhaustiveness_json;
use pen_type::equality::univalent_equality;
use pen_type::normalize::normalize;
use pen_type::substitution::{
    issue_structural_substitution, replay_structural_substitution, SortedParameterContext,
    SubstitutionImage,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs::{read, read_to_string, remove_file, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use thiserror::Error;

pub const DNFQ_DATE: &str = "2026-07-24";
pub const DNF1_SCHEMA: &str = "dnf1-canonical-dependent-contexts-v1";
pub const DNF2_SCHEMA: &str = "dnf2-unified-typed-judgments-v1";
pub const DNF3_SCHEMA: &str = "dnf3-all-legal-substitution-naturality-v1";
pub const DNF4_SCHEMA: &str = "dnf4-full-content-corpus-projection-v1";

pub const DNF1_CONTEXT_ENUMERATOR_GAP: &str = "DNF1_FULL_CONTEXT_GRAMMAR_ENUMERATOR_GAP";
pub const DNF1_CALCULUS_BRIDGE_GAP: &str = "DNF1_CONTEXT_CALCULUS_IDENTIFICATION_GAP";
pub const DNF1_CANONICITY_INDUCTION_GAP: &str = "DNF1_UNIVERSAL_CANONICITY_INDUCTION_GAP";
pub const DNF2_CONTEXT_ATTACHMENT_GAP: &str = "DNF2_EXACT_DEPENDENT_CONTEXT_ATTACHMENT_GAP";
pub const DNF2_EQUALITY_COMPOSITION_GAP: &str = "DNF2_ALL_SURFACE_FROZEN_EQUALITY_COMPOSITION_GAP";
pub const DNF3_SUBSTITUTION_ENUMERATOR_GAP: &str = "DNF3_ALL_LEGAL_SUBSTITUTION_ENUMERATOR_GAP";
pub const DNF3_NORMALIZATION_INDUCTION_GAP: &str = "DNF3_SUBSTITUTION_NORMALIZATION_INDUCTION_GAP";
pub const DNF3_QUOTIENT_CONGRUENCE_GAP: &str = "DNF3_DEPENDENT_FAMILY_QUOTIENT_CONGRUENCE_GAP";
pub const DNF4_FAMILY_CONTEXT_NATURALITY_GAP: &str =
    "DNF4_FAMILY_CONTEXT_OR_UNIVERSAL_NATURALITY_UNEXPORTED";
pub const DNF4_A3_INSTANCE_FAMILY_CLOSURE_GAP: &str =
    "DNF4_A3_INSTANCE_HAS_LOCAL_SQUARE_BUT_NO_FAMILY_LEVEL_CLOSURE";
pub const DNF4_UNARY_FULL_CONTENT_GAP: &str = "DNF4_UNARY_DEPENDENT_DECLARATION_BODY_UNEXPORTED";

const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/dnfq_theorem_layer_plan.md");
const OPERATIONAL_DOMAIN_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_operational_domain_adjudication.md");
const DEPENDENT_CONTEXT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/dependent_context_adjudication.md");
const SUPPORT_COMPREHENSION_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/support_comprehension_adjudication.md");
const DEPENDENT_CONTEXT_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/dependent_context.rs");
const SCHEMA_CONTEXT_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/context.rs");
const SUPPORT_COMPREHENSION_SOURCE_BYTES: &[u8] =
    include_bytes!("support_comprehension_hardening_v6.rs");
const TD21_V2_BYTES: &[u8] = include_bytes!("../../../docs/t_d2_1_operational_domain_v2.json");
const V5_BYTES: &[u8] = include_bytes!("../../../docs/t_bi_nu1_semantic_provenance_v5.json");
const V6_BYTES: &[u8] = include_bytes!("../../../docs/t_bi_nu1_semantic_provenance_v6.json");
const NATURALITY_BYTES: &[u8] = include_bytes!("../../../docs/naturality_orbit_transport_v1.json");
const TRANSPORT_MAPS_BYTES: &[u8] =
    include_bytes!("../../../docs/semantic_nu_transport_maps_v2.json");
const A3_BYTES: &[u8] = include_bytes!("../../../docs/a3_rule_inventory_exhaustiveness_v2.json");
const E5_MEMBERSHIP_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_e5_future_hole_finale_v2_dependent_context.json");

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DnfRunStatus {
    Passed,
    StoppedNamedGaps,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DnfRegister {
    Law,
    FrozenBound,
    ConstructedContent,
    SealedTestimony,
    Gate,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DnfSourceBinding {
    pub path: String,
    pub role: String,
    pub register: DnfRegister,
    pub byte_length: usize,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DnfNamedGap {
    pub gap_id: String,
    pub exact_obstruction: String,
    pub no_proxy_accepted: bool,
    pub zero_charge: bool,
    pub keeps_t_d2_2_closed: bool,
    pub keeps_m4_unauthorized: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DnfDisplayContextEntry {
    pub display_name: String,
    pub classifier: Td21V2KernelClassifier,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DnfDisplayContext {
    pub entries: Vec<DnfDisplayContextEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dnf1CanonicalizationProbe {
    pub first_presentation: DnfDisplayContext,
    pub second_presentation: DnfDisplayContext,
    pub first_canonical: Td21V2ParameterTelescope,
    pub second_canonical: Td21V2ParameterTelescope,
    pub display_names_erased: bool,
    pub canonical_forms_equal: bool,
    pub dependent_prefix_typed: bool,
    pub classifier_normalization_idempotent: bool,
    pub replayed_without_domain_membership_proxy: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dnf1CanonicalContextsCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<DnfSourceBinding>,
    pub frozen_bound_snapshot_digest: String,
    pub frozen_public_entry_count: u32,
    pub frozen_public_clause_count: u32,
    pub frozen_context_arity_bound: u32,
    pub frozen_binder_bound: u32,
    pub frozen_dimension_bound: u32,
    pub frozen_semantic_depth_bound: u32,
    pub bounds_replayed_unchanged: bool,
    pub canonicalization_probe: Dnf1CanonicalizationProbe,
    pub type_and_element_fragment_canonicalized: bool,
    pub function_interval_cofibration_and_opaque_resident_fragment_canonicalized: bool,
    pub full_context_grammar_finitely_enumerated: bool,
    pub schema_and_kernel_context_calculi_identified: bool,
    pub universal_canonicity_proved: bool,
    pub hash_count_label_or_verdict_used_as_context_content: bool,
    pub bounds_modified_or_parameterized: bool,
    pub named_gaps: Vec<DnfNamedGap>,
    pub status: DnfRunStatus,
    pub t_d2_2_reopened: bool,
    pub m4_authorized: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dnf2TranslationRow {
    pub row_id: String,
    pub stage: u32,
    pub family_id: String,
    pub source_surface: String,
    pub source_presentation: Value,
    pub unified_presentation: Value,
    pub exact_payload_preserved: bool,
    pub source_typed_normalized_natural: bool,
    pub canonical_context: Option<Td21V2ParameterTelescope>,
    pub context_reconstructed_from_content: bool,
    pub source_equality_delegate_replayed: bool,
    pub structure_preserving_translation: bool,
    pub full_typed_judgment: bool,
    pub exact_obstruction: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dnf2UnifiedJudgmentsCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<DnfSourceBinding>,
    pub dnf1_result_digest: String,
    pub sealed_v5_replay_valid: bool,
    pub sealed_v5_replay_errors: Vec<String>,
    pub sealed_v5_logical_projection_accepted: bool,
    pub translations: Vec<Dnf2TranslationRow>,
    pub core_translation_count: usize,
    pub ordinary_translation_count: usize,
    pub cubical_translation_count: usize,
    pub r1_translation_count: usize,
    pub exact_payload_translation_count: usize,
    pub locally_context_complete_count: usize,
    pub all_four_surfaces_represented: bool,
    pub every_translation_drops_no_content: bool,
    pub every_surface_has_full_typed_judgment: bool,
    pub translations_compose_with_frozen_equality: bool,
    pub archive_digest_or_claim_used_as_missing_presentation: bool,
    pub named_gaps: Vec<DnfNamedGap>,
    pub status: DnfRunStatus,
    pub t_d2_2_reopened: bool,
    pub m4_authorized: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dnf3ConcreteSquare {
    pub a3_instance_id: String,
    pub mode: String,
    pub canonical_context: Td21V2ParameterTelescope,
    pub generic_family: Expr,
    pub typed_image: Expr,
    pub required_output: Expr,
    pub required_output_normal_form: Expr,
    pub required_output_kernel_type_json: String,
    pub substitution_token: Value,
    pub substituted_output: Expr,
    pub normalized_generic_family: Expr,
    pub normalized_typed_image: Expr,
    pub normalized_substitution_token: Value,
    pub normalized_substituted_output: Expr,
    pub normalization_equality_witness: Value,
    pub exact_substitution_replayed: bool,
    pub normalization_square_replayed: bool,
    pub family_level_all_substitution_closure_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dnf3NaturalityClosureCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<DnfSourceBinding>,
    pub dnf1_result_digest: String,
    pub dnf2_result_digest: String,
    pub predecessor_naturality_replay_valid: bool,
    pub predecessor_naturality_replay_errors: Vec<String>,
    pub predecessor_naturality_logical_projection_accepted: bool,
    pub concrete_squares: Vec<Dnf3ConcreteSquare>,
    pub direct_square_count: usize,
    pub pointwise_square_count: usize,
    pub concrete_square_count: usize,
    pub every_concrete_square_replayed: bool,
    pub exported_transport_map_artifact_bound: bool,
    pub every_legal_substitution_enumerated: bool,
    pub normalization_commutes_by_constructor_induction: bool,
    pub quotient_congruence_under_every_legal_substitution: bool,
    pub universal_naturality_closure_proved: bool,
    pub sampled_square_or_hash_promoted_to_universal_theorem: bool,
    pub named_gaps: Vec<DnfNamedGap>,
    pub status: DnfRunStatus,
    pub t_d2_2_reopened: bool,
    pub m4_authorized: bool,
    pub result_digest: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Dnf4RowKind {
    ProvedFamily,
    A3Membership,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "disposition")]
pub enum Dnf4ProjectionDisposition {
    Projected {
        projection_derivation_hash: String,
    },
    NamedGap {
        gap_id: String,
        exact_reason: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dnf4ProjectionRow {
    pub row_id: String,
    pub row_kind: Dnf4RowKind,
    pub stage: u32,
    pub source_identifier: String,
    pub source_surface: String,
    pub source_content_snapshot: Option<Value>,
    pub canonical_telescope: Option<Td21V2ParameterTelescope>,
    pub unified_presentation: Option<Value>,
    pub substitution_set: Option<Vec<Value>>,
    pub naturality_witnesses: Option<Vec<Value>>,
    pub source_content_reconstructed: bool,
    pub full_telescope_constructed: bool,
    pub full_presentation_constructed: bool,
    pub full_substitution_set_constructed: bool,
    pub full_naturality_witnesses_constructed: bool,
    pub archive_claim_hash_or_count_used_as_missing_content: bool,
    pub disposition: Dnf4ProjectionDisposition,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Dnf4CorpusProjectionCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<DnfSourceBinding>,
    pub dnf1_result_digest: String,
    pub dnf2_result_digest: String,
    pub dnf3_result_digest: String,
    pub sealed_v6_replay_valid: bool,
    pub sealed_v6_replay_errors: Vec<String>,
    pub sealed_v6_logical_projection_accepted: bool,
    pub sealed_a3_replay_valid: bool,
    pub sealed_a3_replay_errors: Vec<String>,
    pub sealed_a3_logical_projection_accepted: bool,
    pub sealed_e5_membership_replay_valid: bool,
    pub sealed_e5_membership_replay_errors: Vec<String>,
    pub sealed_e5_membership_archive_digest_valid: bool,
    pub sealed_e5_membership_logical_projection_accepted: bool,
    pub rows: Vec<Dnf4ProjectionRow>,
    pub expected_family_row_count: usize,
    pub observed_family_row_count: usize,
    pub expected_a3_row_count: usize,
    pub observed_a3_row_count: usize,
    pub expected_total_row_count: usize,
    pub observed_total_row_count: usize,
    pub projected_family_count: usize,
    pub projected_a3_count: usize,
    pub named_gap_family_count: usize,
    pub named_gap_a3_count: usize,
    pub every_row_has_exactly_one_disposition: bool,
    pub no_representative_sampling: bool,
    pub no_backfill: bool,
    pub all_150_rows_explicitly_disposed: bool,
    pub full_content_projection_complete: bool,
    pub status: DnfRunStatus,
    pub t_d2_1_v3_may_recognize_all_rows: bool,
    pub t_d2_2_reopened: bool,
    pub m4_authorized: bool,
    pub result_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DnfReplay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub status: Option<DnfRunStatus>,
    pub t_d2_2_reopened: bool,
    pub m4_authorized: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum DnfqError {
    #[error("DNF-Q input failure: {0}")]
    Input(String),
    #[error("DNF-Q invariant failure: {0}")]
    Invariant(String),
    #[error("DNF-Q JSON failure: {0}")]
    Json(String),
    #[error("DNF-Q I/O failure: {0}")]
    Io(String),
}

#[derive(Clone)]
struct DnfSuite {
    dnf1: Dnf1CanonicalContextsCertificate,
    dnf2: Dnf2UnifiedJudgmentsCertificate,
    dnf3: Dnf3NaturalityClosureCertificate,
    dnf4: Dnf4CorpusProjectionCertificate,
}

fn tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("DNF-Q evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn external_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("external evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn e5_archive_digest_valid(certificate: &E5FutureHoleFinaleV2Certificate) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed
        == external_tagged_hash(
            "schema2-e5-future-hole-finale-v2",
            "certificate",
            &projection,
        )
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn source_binding(path: &str, role: &str, register: DnfRegister, bytes: &[u8]) -> DnfSourceBinding {
    DnfSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        register,
        byte_length: bytes.len(),
        blake3: bytes_hash(bytes),
    }
}

fn law_bindings() -> Vec<DnfSourceBinding> {
    vec![
        source_binding(
            "docs/dnfq_theorem_layer_plan.md",
            "frozen DNF-Q construction brief and falsifiers",
            DnfRegister::Law,
            PLAN_BYTES,
        ),
        source_binding(
            "docs/schema2_operational_domain_adjudication.md",
            "frozen operational bounds; read-only in DNF-Q",
            DnfRegister::FrozenBound,
            OPERATIONAL_DOMAIN_ADJUDICATION_BYTES,
        ),
        source_binding(
            "docs/dependent_context_adjudication.md",
            "adopted dependent ambient context rules",
            DnfRegister::Law,
            DEPENDENT_CONTEXT_ADJUDICATION_BYTES,
        ),
        source_binding(
            "docs/support_comprehension_adjudication.md",
            "adopted canonical dependency analysis and acyclicity",
            DnfRegister::Law,
            SUPPORT_COMPREHENSION_ADJUDICATION_BYTES,
        ),
    ]
}

fn dnf1_bindings() -> Vec<DnfSourceBinding> {
    let mut bindings = law_bindings();
    bindings.extend([
        source_binding(
            "crates/pen-type/src/dependent_context.rs",
            "sequential dependent context implementation",
            DnfRegister::ConstructedContent,
            DEPENDENT_CONTEXT_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-schema/src/context.rs",
            "typed schema and cubical context/substitution calculus",
            DnfRegister::ConstructedContent,
            SCHEMA_CONTEXT_SOURCE_BYTES,
        ),
        source_binding(
            "crates/pen-search/src/support_comprehension_hardening_v6.rs",
            "closed-instance support-comprehension canonicity theorem",
            DnfRegister::ConstructedContent,
            SUPPORT_COMPREHENSION_SOURCE_BYTES,
        ),
        source_binding(
            "docs/t_d2_1_operational_domain_v2.json",
            "frozen operational bounds and current context gap",
            DnfRegister::FrozenBound,
            TD21_V2_BYTES,
        ),
    ]);
    bindings
}

fn dnf2_bindings() -> Vec<DnfSourceBinding> {
    let mut bindings = law_bindings();
    bindings.push(source_binding(
        "docs/t_bi_nu1_semantic_provenance_v5.json",
        "full v5 core/ordinary/cubical and R1 source presentations",
        DnfRegister::ConstructedContent,
        V5_BYTES,
    ));
    bindings
}

fn dnf3_bindings() -> Vec<DnfSourceBinding> {
    let mut bindings = law_bindings();
    bindings.extend([
        source_binding(
            "docs/naturality_orbit_transport_v1.json",
            "expression-bearing A3 transport rows",
            DnfRegister::ConstructedContent,
            NATURALITY_BYTES,
        ),
        source_binding(
            "docs/semantic_nu_transport_maps_v2.json",
            "M-1 exported map inventory; testimony only until objects replay",
            DnfRegister::SealedTestimony,
            TRANSPORT_MAPS_BYTES,
        ),
    ]);
    bindings
}

fn dnf4_bindings() -> Vec<DnfSourceBinding> {
    let mut bindings = law_bindings();
    bindings.extend([
        source_binding(
            "docs/t_bi_nu1_semantic_provenance_v6.json",
            "61 proved-family row inventory",
            DnfRegister::SealedTestimony,
            V6_BYTES,
        ),
        source_binding(
            "docs/a3_rule_inventory_exhaustiveness_v2.json",
            "89 eligible A3 source inventory",
            DnfRegister::SealedTestimony,
            A3_BYTES,
        ),
        source_binding(
            "docs/schema2_e5_future_hole_finale_v2_dependent_context.json",
            "89 membership rows at their sealed evidence granularity",
            DnfRegister::SealedTestimony,
            E5_MEMBERSHIP_BYTES,
        ),
        source_binding(
            "docs/naturality_orbit_transport_v1.json",
            "72 expression-bearing chronological transports",
            DnfRegister::ConstructedContent,
            NATURALITY_BYTES,
        ),
    ]);
    bindings
}

fn named_gap(schema: &str, gap_id: &str, exact_obstruction: &str) -> DnfNamedGap {
    let derivation_hash = tagged_hash(schema, "named-gap", &(gap_id, exact_obstruction));
    DnfNamedGap {
        gap_id: gap_id.to_owned(),
        exact_obstruction: exact_obstruction.to_owned(),
        no_proxy_accepted: true,
        zero_charge: true,
        keeps_t_d2_2_closed: true,
        keeps_m4_unauthorized: true,
        derivation_hash,
    }
}

fn parse_value(bytes: &[u8], label: &str) -> Result<Value, DnfqError> {
    serde_json::from_slice(bytes).map_err(|error| DnfqError::Json(format!("{label}: {error}")))
}

fn value_array<'a>(value: &'a Value, pointer: &str) -> Result<&'a Vec<Value>, DnfqError> {
    value
        .pointer(pointer)
        .and_then(Value::as_array)
        .ok_or_else(|| DnfqError::Input(format!("missing array at {pointer}")))
}

fn value_u32(value: &Value, field: &str) -> Result<u32, DnfqError> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|number| u32::try_from(number).ok())
        .ok_or_else(|| DnfqError::Input(format!("missing u32 field {field}")))
}

fn value_str<'a>(value: &'a Value, field: &str) -> Result<&'a str, DnfqError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| DnfqError::Input(format!("missing string field {field}")))
}

fn context_variable(parameter: u32) -> Td21V2OperationalNode {
    Td21V2OperationalNode::ContextVariable {
        parameter: Td21V2SyntaxNumber {
            value: parameter,
            bound_name: "sealed_maximum_free_scope_length".to_owned(),
        },
    }
}

fn canonicalize_display_context(
    presentation: &DnfDisplayContext,
) -> Result<Td21V2ParameterTelescope, DnfqError> {
    let mut hypotheses = Vec::new();
    for entry in &presentation.entries {
        let classifier = match &entry.classifier {
            Td21V2KernelClassifier::Type => Td21V2KernelClassifier::Type,
            Td21V2KernelClassifier::Element { type_expression } => {
                let prefix = u32::try_from(hypotheses.len())
                    .map_err(|_| DnfqError::Invariant("context prefix exceeds u32".to_owned()))?;
                let normalized = normalize(type_expression, prefix, 1024)
                    .map_err(|error| DnfqError::Input(error.to_string()))?;
                Td21V2KernelClassifier::Element {
                    type_expression: normalized.expr,
                }
            }
            other => {
                return Err(DnfqError::Input(format!(
                    "the public common context API cannot canonicalize classifier {other:?}"
                )));
            }
        };
        hypotheses.push(classifier);
    }
    Ok(Td21V2ParameterTelescope { hypotheses })
}

fn dnf1_digest(certificate: &Dnf1CanonicalContextsCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash(DNF1_SCHEMA, "certificate", &projection)
}

fn build_dnf1() -> Result<Dnf1CanonicalContextsCertificate, DnfqError> {
    let td21: Td21OperationalDomainV2Certificate = serde_json::from_slice(TD21_V2_BYTES)
        .map_err(|error| DnfqError::Json(error.to_string()))?;
    let td21_replay = replay_t_d2_1_operational_domain_v2(&td21);
    if !td21_replay.valid {
        return Err(DnfqError::Input(format!(
            "frozen T-D2-1 v2 did not replay: {}",
            td21_replay.errors.join("; ")
        )));
    }
    let first_presentation = DnfDisplayContext {
        entries: vec![
            DnfDisplayContextEntry {
                display_name: "A".to_owned(),
                classifier: Td21V2KernelClassifier::Type,
            },
            DnfDisplayContextEntry {
                display_name: "x".to_owned(),
                classifier: Td21V2KernelClassifier::Element {
                    type_expression: Expr::Var(1),
                },
            },
        ],
    };
    let second_presentation = DnfDisplayContext {
        entries: vec![
            DnfDisplayContextEntry {
                display_name: "RenamedCarrier".to_owned(),
                classifier: Td21V2KernelClassifier::Type,
            },
            DnfDisplayContextEntry {
                display_name: "renamedPoint".to_owned(),
                classifier: Td21V2KernelClassifier::Element {
                    type_expression: Expr::Var(1),
                },
            },
        ],
    };
    let first_canonical = canonicalize_display_context(&first_presentation)?;
    let second_canonical = canonicalize_display_context(&second_presentation)?;
    let decision = derive_t_d2_1_contextual_membership_v2(&first_canonical, &context_variable(2))
        .map_err(|error| DnfqError::Input(error.to_string()))?;
    let classifier_normalization_idempotent = match &first_canonical.hypotheses[1] {
        Td21V2KernelClassifier::Element { type_expression } => {
            normalize(type_expression, 1, 1024)
                .map_err(|error| DnfqError::Input(error.to_string()))?
                .expr
                == *type_expression
        }
        _ => false,
    };
    let display_names_erased = first_presentation.entries[0].display_name
        != second_presentation.entries[0].display_name
        && first_presentation.entries[1].display_name
            != second_presentation.entries[1].display_name;
    let canonical_forms_equal = first_canonical == second_canonical;
    let dependent_prefix_typed = decision.typed_judgment_accepted
        && !decision.domain_member
        && decision
            .derivation
            .as_ref()
            .is_some_and(|derivation| derivation.parameter_coordinates_used == vec![2]);
    let replayed_without_domain_membership_proxy =
        dependent_prefix_typed && !decision.adopted_quotient_established;
    let mut probe = Dnf1CanonicalizationProbe {
        first_presentation,
        second_presentation,
        first_canonical,
        second_canonical,
        display_names_erased,
        canonical_forms_equal,
        dependent_prefix_typed,
        classifier_normalization_idempotent,
        replayed_without_domain_membership_proxy,
        derivation_hash: String::new(),
    };
    probe.derivation_hash = tagged_hash(DNF1_SCHEMA, "canonicalization-probe", &probe);
    let type_and_element_fragment_canonicalized = probe.display_names_erased
        && probe.canonical_forms_equal
        && probe.dependent_prefix_typed
        && probe.classifier_normalization_idempotent
        && probe.replayed_without_domain_membership_proxy;
    let named_gaps = vec![
        named_gap(
            DNF1_SCHEMA,
            DNF1_CONTEXT_ENUMERATOR_GAP,
            "The common context API canonicalizes Type and earlier-prefix Element declarations, but it has no single finite rank/unrank grammar covering function declarations, opaque residents, intervals, cofibrations, and their recursively bounded motives.",
        ),
        named_gap(
            DNF1_SCHEMA,
            DNF1_CALCULUS_BRIDGE_GAP,
            "The kernel dependent-context calculus and the Schema2/cubical context calculus each replay their own judgments, but no theorem identifies their declarations, substitutions, and equality on one common context object.",
        ),
        named_gap(
            DNF1_SCHEMA,
            DNF1_CANONICITY_INDUCTION_GAP,
            "Two nontrivial dependent presentations normalize to one context in the implemented fragment, but no constructor induction proves that result for every context admitted by the full adopted grammar.",
        ),
    ];
    let bounds_replayed_unchanged = td21.bound_snapshot_digest
        == td21.bound_snapshot.derivation_hash
        && td21.bound_snapshot.public_entry_count == 15
        && td21.bound_snapshot.public_clause_count == 64
        && td21.bound_snapshot.maximum_free_scope_length == 9
        && td21.bound_snapshot.maximum_binder_nesting == 2
        && td21.bound_snapshot.maximum_path_dimension == 3
        && td21.bound_snapshot.semantic_former_depth == 2;
    let mut certificate = Dnf1CanonicalContextsCertificate {
        schema: DNF1_SCHEMA.to_owned(),
        date: DNFQ_DATE.to_owned(),
        source_bindings: dnf1_bindings(),
        frozen_bound_snapshot_digest: td21.bound_snapshot_digest,
        frozen_public_entry_count: td21.bound_snapshot.public_entry_count,
        frozen_public_clause_count: td21.bound_snapshot.public_clause_count,
        frozen_context_arity_bound: td21.bound_snapshot.maximum_free_scope_length,
        frozen_binder_bound: td21.bound_snapshot.maximum_binder_nesting,
        frozen_dimension_bound: td21.bound_snapshot.maximum_path_dimension,
        frozen_semantic_depth_bound: td21.bound_snapshot.semantic_former_depth,
        bounds_replayed_unchanged,
        canonicalization_probe: probe,
        type_and_element_fragment_canonicalized,
        function_interval_cofibration_and_opaque_resident_fragment_canonicalized: false,
        full_context_grammar_finitely_enumerated: false,
        schema_and_kernel_context_calculi_identified: false,
        universal_canonicity_proved: false,
        hash_count_label_or_verdict_used_as_context_content: false,
        bounds_modified_or_parameterized: false,
        named_gaps,
        status: DnfRunStatus::StoppedNamedGaps,
        t_d2_2_reopened: false,
        m4_authorized: false,
        result_digest: String::new(),
    };
    certificate.result_digest = dnf1_digest(&certificate);
    Ok(certificate)
}

fn parameter_context_from_core_presentation(
    presentation: &Value,
) -> Option<Td21V2ParameterTelescope> {
    let sorts = presentation.get("parameter_sorts")?.as_array()?;
    let mut hypotheses = Vec::new();
    for sort in sorts {
        if sort.as_str()? != "Type" {
            return None;
        }
        hypotheses.push(Td21V2KernelClassifier::Type);
    }
    Some(Td21V2ParameterTelescope { hypotheses })
}

fn translation_row(
    stage: u32,
    family_id: &str,
    source_surface: &str,
    source_presentation: Value,
    source_typed_normalized_natural: bool,
    canonical_context: Option<Td21V2ParameterTelescope>,
    source_equality_delegate_replayed: bool,
) -> Dnf2TranslationRow {
    let unified_presentation = json!({
        "surface": source_surface,
        "exact_source_payload": source_presentation.clone(),
    });
    let exact_payload_preserved = unified_presentation
        .get("exact_source_payload")
        .is_some_and(|payload| payload == &source_presentation);
    let context_reconstructed_from_content = canonical_context.is_some();
    let structure_preserving_translation =
        exact_payload_preserved && source_typed_normalized_natural;
    let full_typed_judgment = structure_preserving_translation
        && context_reconstructed_from_content
        && source_equality_delegate_replayed;
    let exact_obstruction = (!full_typed_judgment).then(|| {
        format!(
            "The exact {source_surface} payload is retained, but its canonical dependent context or equality-composition witness is not reconstructible from the public source presentation."
        )
    });
    let row_id = tagged_hash(
        DNF2_SCHEMA,
        "translation-row-id",
        &(stage, family_id, source_surface, &source_presentation),
    );
    let mut row = Dnf2TranslationRow {
        row_id,
        stage,
        family_id: family_id.to_owned(),
        source_surface: source_surface.to_owned(),
        source_presentation,
        unified_presentation,
        exact_payload_preserved,
        source_typed_normalized_natural,
        canonical_context,
        context_reconstructed_from_content,
        source_equality_delegate_replayed,
        structure_preserving_translation,
        full_typed_judgment,
        exact_obstruction,
        derivation_hash: String::new(),
    };
    row.derivation_hash = tagged_hash(DNF2_SCHEMA, "translation-row", &row);
    row
}

fn dnf2_digest(certificate: &Dnf2UnifiedJudgmentsCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash(DNF2_SCHEMA, "certificate", &projection)
}

fn build_dnf2(
    dnf1: &Dnf1CanonicalContextsCertificate,
) -> Result<Dnf2UnifiedJudgmentsCertificate, DnfqError> {
    let v5_text =
        std::str::from_utf8(V5_BYTES).map_err(|error| DnfqError::Input(error.to_string()))?;
    let v5_replay = replay_t_bi_nu1_regression_v5_json(v5_text);
    let sealed_v5_replay_valid = v5_replay.valid;
    let sealed_v5_replay_errors = v5_replay.errors.clone();
    let sealed_v5_logical_projection_accepted = sealed_v5_replay_valid
        || (sealed_v5_replay_errors
            == ["T-BI v5 certificate differs from create-new reissuance".to_owned()]
            && v5_replay.theorem_proved
            && v5_replay.extraction_complete
            && v5_replay.operational_regression_exact
            && v5_replay.f_al1_prime_passed);
    if !sealed_v5_logical_projection_accepted {
        return Err(DnfqError::Input(format!(
            "sealed v5 artifact did not replay: {}",
            v5_replay.errors.join("; ")
        )));
    }
    let v5 = parse_value(V5_BYTES, "v5 semantic provenance")?;
    let packages = value_array(&v5, "/intrinsic_sequence/packages")?;
    let mut translations = Vec::new();
    for package in packages {
        let stage = value_u32(package, "stage")?;
        let equality_delegate = package
            .pointer("/unified_quotient/equivalence_closure/proved")
            .and_then(Value::as_bool)
            == Some(true);
        for member in value_array(package, "/unified_quotient/current_members")? {
            let family_id = value_str(member, "family_id")?;
            let presentation = member
                .get("presentation")
                .cloned()
                .ok_or_else(|| DnfqError::Input("v5 member lacks presentation".to_owned()))?;
            let surface = value_str(&presentation, "presentation")?.to_owned();
            let context = if surface == "core_expr" {
                parameter_context_from_core_presentation(&presentation)
            } else {
                None
            };
            let typed = member
                .get("typed_normalized_natural")
                .and_then(Value::as_bool)
                == Some(true);
            translations.push(translation_row(
                stage,
                family_id,
                &surface,
                presentation,
                typed,
                context,
                equality_delegate,
            ));
        }
    }
    let stage1 = packages
        .iter()
        .find(|package| package.get("stage").and_then(Value::as_u64) == Some(1))
        .ok_or_else(|| DnfqError::Input("v5 lacks Stage-1 package".to_owned()))?;
    let r1_cases = value_array(stage1, "/stage1_carrier_role_case_proofs")?;
    let kernel_head = r1_cases
        .iter()
        .find(|case| case.get("role").and_then(Value::as_str) == Some("kernel_head"))
        .ok_or_else(|| DnfqError::Input("Stage-1 R1 kernel-head case is absent".to_owned()))?;
    let r1_source = json!({
        "presentation": "r1_package",
        "carrier_clause": kernel_head.get("carrier_clause"),
        "completion_clause": kernel_head.get("completion_clause"),
        "carrier_expr": kernel_head.get("carrier_expr"),
        "completion_expr": kernel_head.get("completion_expr"),
        "carrier_normal_form": kernel_head.get("carrier_normal_form"),
        "completion_normal_form": kernel_head.get("completion_normal_form"),
        "candidate_clause_roles": kernel_head.get("candidate_clause_roles"),
        "exact_formation_completion_package": kernel_head.get("exact_formation_completion_package"),
    });
    let r1_typed = kernel_head.get("proved").and_then(Value::as_bool) == Some(true)
        && kernel_head
            .get("exact_formation_completion_package")
            .and_then(Value::as_bool)
            == Some(true);
    translations.push(translation_row(
        1,
        "stage1-r1-formation-completion-package",
        "r1_package",
        r1_source,
        r1_typed,
        Some(Td21V2ParameterTelescope {
            hypotheses: vec![Td21V2KernelClassifier::Type],
        }),
        stage1
            .pointer("/unified_quotient/equivalence_closure/proved")
            .and_then(Value::as_bool)
            == Some(true),
    ));
    translations.sort_by(|left, right| {
        (left.stage, &left.source_surface, &left.family_id).cmp(&(
            right.stage,
            &right.source_surface,
            &right.family_id,
        ))
    });
    let core_translation_count = translations
        .iter()
        .filter(|row| row.source_surface == "core_expr")
        .count();
    let ordinary_translation_count = translations
        .iter()
        .filter(|row| row.source_surface == "ordinary_schema2")
        .count();
    let cubical_translation_count = translations
        .iter()
        .filter(|row| row.source_surface == "cubical_path")
        .count();
    let r1_translation_count = translations
        .iter()
        .filter(|row| row.source_surface == "r1_package")
        .count();
    let exact_payload_translation_count = translations
        .iter()
        .filter(|row| row.exact_payload_preserved && row.structure_preserving_translation)
        .count();
    let locally_context_complete_count = translations
        .iter()
        .filter(|row| row.full_typed_judgment)
        .count();
    let all_four_surfaces_represented = core_translation_count > 0
        && ordinary_translation_count > 0
        && cubical_translation_count > 0
        && r1_translation_count == 1;
    let every_translation_drops_no_content =
        translations.iter().all(|row| row.exact_payload_preserved);
    let every_surface_has_full_typed_judgment =
        translations.iter().all(|row| row.full_typed_judgment);
    let translations_compose_with_frozen_equality = every_surface_has_full_typed_judgment
        && translations
            .iter()
            .all(|row| row.source_equality_delegate_replayed);
    let named_gaps = vec![
        named_gap(
            DNF2_SCHEMA,
            DNF2_CONTEXT_ATTACHMENT_GAP,
            "Exact source payloads are retained for all four surfaces, but ordinary, cubical, and opaque-parameter core members do not export an exact canonical dependent telescope. Wrapping their payload is not a typed-judgment translation.",
        ),
        named_gap(
            DNF2_SCHEMA,
            DNF2_EQUALITY_COMPOSITION_GAP,
            "The v5 finite equality closure replays on its historical members, but without exact target contexts there is no theorem that every surface translation composes with frozen equality as a typed judgment.",
        ),
    ];
    let mut certificate = Dnf2UnifiedJudgmentsCertificate {
        schema: DNF2_SCHEMA.to_owned(),
        date: DNFQ_DATE.to_owned(),
        source_bindings: dnf2_bindings(),
        dnf1_result_digest: dnf1.result_digest.clone(),
        sealed_v5_replay_valid,
        sealed_v5_replay_errors,
        sealed_v5_logical_projection_accepted,
        translations,
        core_translation_count,
        ordinary_translation_count,
        cubical_translation_count,
        r1_translation_count,
        exact_payload_translation_count,
        locally_context_complete_count,
        all_four_surfaces_represented,
        every_translation_drops_no_content,
        every_surface_has_full_typed_judgment,
        translations_compose_with_frozen_equality,
        archive_digest_or_claim_used_as_missing_presentation: false,
        named_gaps,
        status: DnfRunStatus::StoppedNamedGaps,
        t_d2_2_reopened: false,
        m4_authorized: false,
        result_digest: String::new(),
    };
    certificate.result_digest = dnf2_digest(&certificate);
    Ok(certificate)
}

fn reconstruct_square(row: &A3TransportedInstance) -> Result<Dnf3ConcreteSquare, DnfqError> {
    let context = SortedParameterContext::all_type(1);
    let substitution = issue_structural_substitution(
        context.clone(),
        context.clone(),
        vec![SubstitutionImage {
            source_parameter: 1,
            term: row.typed_image.clone(),
        }],
        row.generic_family.clone(),
    )
    .map_err(|error| DnfqError::Input(error.to_string()))?;
    replay_structural_substitution(&substitution)
        .map_err(|error| DnfqError::Input(error.to_string()))?;
    let exact_substitution_replayed = substitution.result() == &row.required_output;
    let normalized_generic = normalize(&row.generic_family, 1, 4096)
        .map_err(|error| DnfqError::Input(error.to_string()))?
        .expr;
    let normalized_image = normalize(&row.typed_image, 1, 4096)
        .map_err(|error| DnfqError::Input(error.to_string()))?
        .expr;
    let normalized_substitution = issue_structural_substitution(
        context.clone(),
        context,
        vec![SubstitutionImage {
            source_parameter: 1,
            term: normalized_image.clone(),
        }],
        normalized_generic.clone(),
    )
    .map_err(|error| DnfqError::Input(error.to_string()))?;
    replay_structural_substitution(&normalized_substitution)
        .map_err(|error| DnfqError::Input(error.to_string()))?;
    let equality = univalent_equality(
        &row.required_output_normal_form,
        normalized_substitution.result(),
        1,
        4096,
    )
    .map_err(|error| DnfqError::Input(error.to_string()))?;
    let normalization_square_replayed = equality.equal && row.normalization_equality.equal;
    let mut square = Dnf3ConcreteSquare {
        a3_instance_id: row.a3_instance_id.clone(),
        mode: row.mode.clone(),
        canonical_context: Td21V2ParameterTelescope {
            hypotheses: vec![Td21V2KernelClassifier::Type],
        },
        generic_family: row.generic_family.clone(),
        typed_image: row.typed_image.clone(),
        required_output: row.required_output.clone(),
        required_output_normal_form: row.required_output_normal_form.clone(),
        required_output_kernel_type_json: row.required_output_kernel_type_json.clone(),
        substitution_token: serde_json::to_value(&substitution)
            .map_err(|error| DnfqError::Json(error.to_string()))?,
        substituted_output: substitution.result().clone(),
        normalized_generic_family: normalized_generic,
        normalized_typed_image: normalized_image,
        normalized_substitution_token: serde_json::to_value(&normalized_substitution)
            .map_err(|error| DnfqError::Json(error.to_string()))?,
        normalized_substituted_output: normalized_substitution.result().clone(),
        normalization_equality_witness: serde_json::to_value(&equality)
            .map_err(|error| DnfqError::Json(error.to_string()))?,
        exact_substitution_replayed,
        normalization_square_replayed,
        family_level_all_substitution_closure_proved: false,
        derivation_hash: String::new(),
    };
    square.derivation_hash = tagged_hash(DNF3_SCHEMA, "concrete-square", &square);
    Ok(square)
}

fn dnf3_digest(certificate: &Dnf3NaturalityClosureCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash(DNF3_SCHEMA, "certificate", &projection)
}

fn build_dnf3(
    dnf1: &Dnf1CanonicalContextsCertificate,
    dnf2: &Dnf2UnifiedJudgmentsCertificate,
) -> Result<Dnf3NaturalityClosureCertificate, DnfqError> {
    let predecessor: NaturalityOrbitTransportCertificate = serde_json::from_slice(NATURALITY_BYTES)
        .map_err(|error| DnfqError::Json(error.to_string()))?;
    let predecessor_replay = replay_naturality_orbit_transport_certificate(&predecessor);
    let predecessor_naturality_replay_errors = predecessor_replay.errors.clone();
    let predecessor_naturality_logical_projection_accepted = predecessor_replay.valid
        || (predecessor_naturality_replay_errors
            == ["certificate differs from independent definition replay".to_owned()]
            && predecessor_replay.direct_regression_64_to_8
            && predecessor_replay.pointwise_joined_to_eight
            && predecessor_replay.unary_pending_count == 17);
    if !predecessor_naturality_logical_projection_accepted {
        return Err(DnfqError::Input(format!(
            "naturality predecessor failed replay: {}",
            predecessor_replay.errors.join("; ")
        )));
    }
    let mut concrete_squares = predecessor
        .a3
        .direct_instances
        .iter()
        .chain(&predecessor.a3.pointwise_instances)
        .map(reconstruct_square)
        .collect::<Result<Vec<_>, _>>()?;
    concrete_squares.sort_by(|left, right| left.a3_instance_id.cmp(&right.a3_instance_id));
    let direct_square_count = concrete_squares
        .iter()
        .filter(|square| square.mode == "direct_type")
        .count();
    let pointwise_square_count = concrete_squares
        .iter()
        .filter(|square| square.mode == "pointwise_type")
        .count();
    let concrete_square_count = concrete_squares.len();
    let every_concrete_square_replayed = concrete_squares
        .iter()
        .all(|square| square.exact_substitution_replayed && square.normalization_square_replayed);
    let transport_maps = parse_value(TRANSPORT_MAPS_BYTES, "transport maps")?;
    let exported_transport_map_artifact_bound = transport_maps
        .get("typed_transport_maps")
        .and_then(Value::as_array)
        .is_some_and(|maps| !maps.is_empty())
        && transport_maps
            .get("no_new_equivalence_semantics_assumed")
            .and_then(Value::as_bool)
            == Some(true);
    let named_gaps = vec![
        named_gap(
            DNF3_SCHEMA,
            DNF3_SUBSTITUTION_ENUMERATOR_GAP,
            "Seventy-two exact historical substitutions are reconstructed as objects, but the full space of legal dependent, expression-image, and cubical face substitutions has no finite constructor eliminator shared by all four surfaces.",
        ),
        named_gap(
            DNF3_SCHEMA,
            DNF3_NORMALIZATION_INDUCTION_GAP,
            "Every historical chronological square commutes, but there is no generic constructor induction proving normalization commutation for an arbitrary legal substitution and arbitrary unified judgment.",
        ),
        named_gap(
            DNF3_SCHEMA,
            DNF3_QUOTIENT_CONGRUENCE_GAP,
            "The reconstructed squares prove their own instances. They do not establish that the dependent natural-family quotient is a congruence under every legal substitution, especially across ordinary and cubical presentations.",
        ),
    ];
    let mut certificate = Dnf3NaturalityClosureCertificate {
        schema: DNF3_SCHEMA.to_owned(),
        date: DNFQ_DATE.to_owned(),
        source_bindings: dnf3_bindings(),
        dnf1_result_digest: dnf1.result_digest.clone(),
        dnf2_result_digest: dnf2.result_digest.clone(),
        predecessor_naturality_replay_valid: predecessor_replay.valid,
        predecessor_naturality_replay_errors,
        predecessor_naturality_logical_projection_accepted,
        concrete_squares,
        direct_square_count,
        pointwise_square_count,
        concrete_square_count,
        every_concrete_square_replayed,
        exported_transport_map_artifact_bound,
        every_legal_substitution_enumerated: false,
        normalization_commutes_by_constructor_induction: false,
        quotient_congruence_under_every_legal_substitution: false,
        universal_naturality_closure_proved: false,
        sampled_square_or_hash_promoted_to_universal_theorem: false,
        named_gaps,
        status: DnfRunStatus::StoppedNamedGaps,
        t_d2_2_reopened: false,
        m4_authorized: false,
        result_digest: String::new(),
    };
    certificate.result_digest = dnf3_digest(&certificate);
    Ok(certificate)
}

fn projected(disposition: &Dnf4ProjectionDisposition) -> bool {
    matches!(disposition, Dnf4ProjectionDisposition::Projected { .. })
}

fn projection_row_hash(row: &mut Dnf4ProjectionRow) {
    row.derivation_hash = tagged_hash(DNF4_SCHEMA, "projection-row", row);
}

fn dnf4_digest(certificate: &Dnf4CorpusProjectionCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash(DNF4_SCHEMA, "certificate", &projection)
}

fn build_dnf4(
    dnf1: &Dnf1CanonicalContextsCertificate,
    dnf2: &Dnf2UnifiedJudgmentsCertificate,
    dnf3: &Dnf3NaturalityClosureCertificate,
) -> Result<Dnf4CorpusProjectionCertificate, DnfqError> {
    let v6_text =
        std::str::from_utf8(V6_BYTES).map_err(|error| DnfqError::Input(error.to_string()))?;
    let v6_replay = replay_t_bi_nu1_regression_v6_json(v6_text);
    let sealed_v6_replay_errors = v6_replay.errors.clone();
    let sealed_v6_logical_projection_accepted = v6_replay.valid
        || (sealed_v6_replay_errors
            == ["T-BI v6 certificate differs from deterministic create-new reissuance".to_owned()]
            && v6_replay.theorem_proved
            && v6_replay.extraction_complete
            && v6_replay.operational_regression_exact
            && v6_replay.f_al1_prime_passed);
    if !sealed_v6_logical_projection_accepted {
        return Err(DnfqError::Input(format!(
            "sealed v6 artifact failed replay: {}",
            v6_replay.errors.join("; ")
        )));
    }
    let a3_text =
        std::str::from_utf8(A3_BYTES).map_err(|error| DnfqError::Input(error.to_string()))?;
    let a3_replay = replay_historical_a3_rule_inventory_exhaustiveness_json(a3_text);
    let sealed_a3_replay_errors = a3_replay.errors.clone();
    let sealed_a3_logical_projection_accepted = a3_replay.valid
        || (sealed_a3_replay_errors == ["certificate differs from definition replay".to_owned()]
            && a3_replay.historical_window_count == 16
            && a3_replay.relative_exhaustiveness_proved
            && !a3_replay.absolute_exhaustiveness_claimed);
    if !sealed_a3_logical_projection_accepted {
        return Err(DnfqError::Input(format!(
            "sealed A3 artifact failed replay: {}",
            a3_replay.errors.join("; ")
        )));
    }
    let e5: E5FutureHoleFinaleV2Certificate = serde_json::from_slice(E5_MEMBERSHIP_BYTES)
        .map_err(|error| DnfqError::Json(error.to_string()))?;
    let e5_replay = replay_e5_future_hole_finale_v2_certificate(&e5);
    let sealed_e5_membership_replay_errors = e5_replay.errors.clone();
    let sealed_e5_membership_archive_digest_valid = e5_archive_digest_valid(&e5);
    let sealed_e5_membership_logical_projection_accepted = e5_replay.valid
        || (sealed_e5_membership_replay_errors
            == ["prerequisite failed: stable A3 exhaustiveness certificate did not replay: certificate differs from definition replay".to_owned()]
            && sealed_e5_membership_archive_digest_valid
            && e5.stage16.membership_rows.len() == 89
            && e5.stage16.underdetermined_instance_ids.is_empty()
            && e5.e5_complete);
    if !sealed_e5_membership_logical_projection_accepted {
        return Err(DnfqError::Input(format!(
            "sealed E5 artifact failed replay: {}",
            e5_replay.errors.join("; ")
        )));
    }
    let v6 = parse_value(V6_BYTES, "v6 semantic provenance")?;
    let mut translation_by_stage_family = BTreeMap::new();
    for row in &dnf2.translations {
        translation_by_stage_family.insert((row.stage, row.family_id.clone()), row);
    }
    let r1_translation = dnf2
        .translations
        .iter()
        .find(|row| row.source_surface == "r1_package")
        .ok_or_else(|| DnfqError::Invariant("DNF-2 lacks its R1 row".to_owned()))?;
    let mut rows = Vec::new();
    for package in value_array(&v6, "/intrinsic_sequence/packages")? {
        let stage = value_u32(package, "stage")?;
        for role in value_array(package, "/role_rows")? {
            if role.get("resolution_class").and_then(Value::as_str) != Some("proved_family") {
                continue;
            }
            let declaration_id = value_str(role, "declaration_id")?;
            let family_id = value_str(role, "resolved_family_id")?;
            let translation = translation_by_stage_family
                .get(&(stage, family_id.to_owned()))
                .copied()
                .or_else(|| (stage == 1).then_some(r1_translation));
            let source_snapshot = translation.map(|row| row.source_presentation.clone());
            let canonical_telescope = translation.and_then(|row| row.canonical_context.clone());
            let unified_presentation = translation.map(|row| row.unified_presentation.clone());
            let source_surface = translation
                .map(|row| row.source_surface.clone())
                .unwrap_or_else(|| "unjoined_family_surface".to_owned());
            let source_content_reconstructed = source_snapshot.is_some();
            let full_telescope_constructed = canonical_telescope.is_some();
            let full_presentation_constructed = unified_presentation.is_some();
            let exact_reason = if source_content_reconstructed {
                "The exact historical family presentation is reconstructed, but the public artifacts do not construct its complete dependent telescope together with an all-legal-substitution naturality and univalent-congruence family. The family remains certified at its original claim granularity."
            } else {
                "The proved-family declaration does not join any full source presentation. Its family identifier and derivation hashes are testimony, not content, and are not backfilled."
            };
            let mut row = Dnf4ProjectionRow {
                row_id: format!("family:{stage}:{declaration_id}"),
                row_kind: Dnf4RowKind::ProvedFamily,
                stage,
                source_identifier: family_id.to_owned(),
                source_surface,
                source_content_snapshot: source_snapshot,
                canonical_telescope,
                unified_presentation,
                substitution_set: None,
                naturality_witnesses: None,
                source_content_reconstructed,
                full_telescope_constructed,
                full_presentation_constructed,
                full_substitution_set_constructed: false,
                full_naturality_witnesses_constructed: false,
                archive_claim_hash_or_count_used_as_missing_content: false,
                disposition: Dnf4ProjectionDisposition::NamedGap {
                    gap_id: DNF4_FAMILY_CONTEXT_NATURALITY_GAP.to_owned(),
                    exact_reason: exact_reason.to_owned(),
                },
                derivation_hash: String::new(),
            };
            projection_row_hash(&mut row);
            rows.push(row);
        }
    }
    let e5_value = parse_value(E5_MEMBERSHIP_BYTES, "E5 membership")?;
    let membership_rows = value_array(&e5_value, "/stage16/membership_rows")?;
    let squares = dnf3
        .concrete_squares
        .iter()
        .map(|square| (square.a3_instance_id.as_str(), square))
        .collect::<BTreeMap<_, _>>();
    for membership in membership_rows {
        let instance_id = value_str(membership, "a3_instance_id")?;
        let rule = value_str(membership, "rule")?;
        let stage = value_u32(membership, "source_step")?;
        let square = squares.get(instance_id).copied();
        let source_snapshot = square.map(|square| {
            json!({
                "generic_family": square.generic_family,
                "typed_image": square.typed_image,
                "required_output": square.required_output,
                "required_output_normal_form": square.required_output_normal_form,
                "required_output_kernel_type_json": square.required_output_kernel_type_json,
            })
        });
        let canonical_telescope = square.map(|square| square.canonical_context.clone());
        let unified_presentation = source_snapshot.as_ref().map(|snapshot| {
            json!({
                "surface": "a3_instance",
                "exact_source_payload": snapshot,
            })
        });
        let substitution_set = square.map(|square| vec![square.substitution_token.clone()]);
        let naturality_witnesses = square.map(|square| {
            vec![
                square.normalized_substitution_token.clone(),
                square.normalization_equality_witness.clone(),
            ]
        });
        let local_content_complete = square.is_some_and(|square| {
            square.exact_substitution_replayed && square.normalization_square_replayed
        });
        let (gap_id, exact_reason) = if local_content_complete {
            (
                DNF4_A3_INSTANCE_FAMILY_CLOSURE_GAP,
                "The concrete A3 substitution object and normalization square replay exactly, but DNF-3 does not prove naturality of the containing family under every legal substitution. One commuting instance cannot establish membership in the adopted natural-family quotient.",
            )
        } else {
            (
                DNF4_UNARY_FULL_CONTENT_GAP,
                "The membership certificate exports claim-level source/body/totality hashes but not the unary action's actual dependent declaration, body, substitution object, and naturality square. F-DNF2 forbids reconstructing what the archive must have meant.",
            )
        };
        let mut row = Dnf4ProjectionRow {
            row_id: format!("a3:{instance_id}"),
            row_kind: Dnf4RowKind::A3Membership,
            stage,
            source_identifier: instance_id.to_owned(),
            source_surface: rule.to_owned(),
            source_content_snapshot: source_snapshot,
            canonical_telescope,
            unified_presentation,
            substitution_set,
            naturality_witnesses,
            source_content_reconstructed: local_content_complete,
            full_telescope_constructed: local_content_complete,
            full_presentation_constructed: local_content_complete,
            full_substitution_set_constructed: local_content_complete,
            full_naturality_witnesses_constructed: local_content_complete,
            archive_claim_hash_or_count_used_as_missing_content: false,
            disposition: Dnf4ProjectionDisposition::NamedGap {
                gap_id: gap_id.to_owned(),
                exact_reason: exact_reason.to_owned(),
            },
            derivation_hash: String::new(),
        };
        projection_row_hash(&mut row);
        rows.push(row);
    }
    rows.sort_by(|left, right| {
        (left.row_kind, left.stage, &left.row_id).cmp(&(right.row_kind, right.stage, &right.row_id))
    });
    let observed_family_row_count = rows
        .iter()
        .filter(|row| row.row_kind == Dnf4RowKind::ProvedFamily)
        .count();
    let observed_a3_row_count = rows
        .iter()
        .filter(|row| row.row_kind == Dnf4RowKind::A3Membership)
        .count();
    let projected_family_count = rows
        .iter()
        .filter(|row| row.row_kind == Dnf4RowKind::ProvedFamily && projected(&row.disposition))
        .count();
    let projected_a3_count = rows
        .iter()
        .filter(|row| row.row_kind == Dnf4RowKind::A3Membership && projected(&row.disposition))
        .count();
    let named_gap_family_count = observed_family_row_count - projected_family_count;
    let named_gap_a3_count = observed_a3_row_count - projected_a3_count;
    let every_row_has_exactly_one_disposition = rows.iter().all(|row| {
        matches!(
            row.disposition,
            Dnf4ProjectionDisposition::Projected { .. }
                | Dnf4ProjectionDisposition::NamedGap { .. }
        )
    });
    let all_150_rows_explicitly_disposed = observed_family_row_count == 61
        && observed_a3_row_count == 89
        && rows.len() == 150
        && every_row_has_exactly_one_disposition;
    let full_content_projection_complete = all_150_rows_explicitly_disposed
        && projected_family_count == 61
        && projected_a3_count == 89;
    let mut certificate = Dnf4CorpusProjectionCertificate {
        schema: DNF4_SCHEMA.to_owned(),
        date: DNFQ_DATE.to_owned(),
        source_bindings: dnf4_bindings(),
        dnf1_result_digest: dnf1.result_digest.clone(),
        dnf2_result_digest: dnf2.result_digest.clone(),
        dnf3_result_digest: dnf3.result_digest.clone(),
        sealed_v6_replay_valid: v6_replay.valid,
        sealed_v6_replay_errors,
        sealed_v6_logical_projection_accepted,
        sealed_a3_replay_valid: a3_replay.valid,
        sealed_a3_replay_errors,
        sealed_a3_logical_projection_accepted,
        sealed_e5_membership_replay_valid: e5_replay.valid,
        sealed_e5_membership_replay_errors,
        sealed_e5_membership_archive_digest_valid,
        sealed_e5_membership_logical_projection_accepted,
        rows,
        expected_family_row_count: 61,
        observed_family_row_count,
        expected_a3_row_count: 89,
        observed_a3_row_count,
        expected_total_row_count: 150,
        observed_total_row_count: observed_family_row_count + observed_a3_row_count,
        projected_family_count,
        projected_a3_count,
        named_gap_family_count,
        named_gap_a3_count,
        every_row_has_exactly_one_disposition,
        no_representative_sampling: true,
        no_backfill: true,
        all_150_rows_explicitly_disposed,
        full_content_projection_complete,
        status: if full_content_projection_complete {
            DnfRunStatus::Passed
        } else {
            DnfRunStatus::StoppedNamedGaps
        },
        t_d2_1_v3_may_recognize_all_rows: full_content_projection_complete,
        t_d2_2_reopened: false,
        m4_authorized: false,
        result_digest: String::new(),
    };
    certificate.result_digest = dnf4_digest(&certificate);
    Ok(certificate)
}

fn build_suite() -> Result<DnfSuite, DnfqError> {
    let dnf1 = build_dnf1()?;
    let dnf2 = build_dnf2(&dnf1)?;
    let dnf3 = build_dnf3(&dnf1, &dnf2)?;
    let dnf4 = build_dnf4(&dnf1, &dnf2, &dnf3)?;
    Ok(DnfSuite {
        dnf1,
        dnf2,
        dnf3,
        dnf4,
    })
}

static SUITE: OnceLock<Result<DnfSuite, String>> = OnceLock::new();

fn expected_suite() -> Result<&'static DnfSuite, DnfqError> {
    match SUITE.get_or_init(|| build_suite().map_err(|error| error.to_string())) {
        Ok(suite) => Ok(suite),
        Err(error) => Err(DnfqError::Input(error.clone())),
    }
}

pub fn issue_dnf1_canonical_contexts_v1() -> Result<Dnf1CanonicalContextsCertificate, DnfqError> {
    Ok(expected_suite()?.dnf1.clone())
}

pub fn issue_dnf2_unified_judgments_v1() -> Result<Dnf2UnifiedJudgmentsCertificate, DnfqError> {
    Ok(expected_suite()?.dnf2.clone())
}

pub fn issue_dnf3_naturality_closure_v1() -> Result<Dnf3NaturalityClosureCertificate, DnfqError> {
    Ok(expected_suite()?.dnf3.clone())
}

pub fn issue_dnf4_corpus_projection_v1() -> Result<Dnf4CorpusProjectionCertificate, DnfqError> {
    Ok(expected_suite()?.dnf4.clone())
}

fn failed_replay(error: impl Into<String>) -> DnfReplay {
    DnfReplay {
        valid: false,
        errors: vec![error.into()],
        status: None,
        t_d2_2_reopened: false,
        m4_authorized: false,
    }
}

fn replay_common<T: Eq>(
    claimed: &T,
    expected: &T,
    digest_matches: bool,
    status: DnfRunStatus,
    t_d2_2_reopened: bool,
    m4_authorized: bool,
) -> DnfReplay {
    let mut errors = Vec::new();
    if !digest_matches {
        errors.push("certificate digest mismatch".to_owned());
    }
    if claimed != expected {
        errors.push("certificate differs from deterministic reconstruction".to_owned());
    }
    if !errors.is_empty() {
        return failed_replay(errors.join("; "));
    }
    DnfReplay {
        valid: true,
        errors,
        status: Some(status),
        t_d2_2_reopened,
        m4_authorized,
    }
}

pub fn replay_dnf1_canonical_contexts_v1(claimed: &Dnf1CanonicalContextsCertificate) -> DnfReplay {
    let expected = match expected_suite() {
        Ok(suite) => &suite.dnf1,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_common(
        claimed,
        expected,
        claimed.result_digest == dnf1_digest(claimed),
        expected.status,
        expected.t_d2_2_reopened,
        expected.m4_authorized,
    )
}

pub fn replay_dnf2_unified_judgments_v1(claimed: &Dnf2UnifiedJudgmentsCertificate) -> DnfReplay {
    let expected = match expected_suite() {
        Ok(suite) => &suite.dnf2,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_common(
        claimed,
        expected,
        claimed.result_digest == dnf2_digest(claimed),
        expected.status,
        expected.t_d2_2_reopened,
        expected.m4_authorized,
    )
}

pub fn replay_dnf3_naturality_closure_v1(claimed: &Dnf3NaturalityClosureCertificate) -> DnfReplay {
    let expected = match expected_suite() {
        Ok(suite) => &suite.dnf3,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_common(
        claimed,
        expected,
        claimed.result_digest == dnf3_digest(claimed),
        expected.status,
        expected.t_d2_2_reopened,
        expected.m4_authorized,
    )
}

pub fn replay_dnf4_corpus_projection_v1(claimed: &Dnf4CorpusProjectionCertificate) -> DnfReplay {
    let expected = match expected_suite() {
        Ok(suite) => &suite.dnf4,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_common(
        claimed,
        expected,
        claimed.result_digest == dnf4_digest(claimed),
        expected.status,
        expected.t_d2_2_reopened,
        expected.m4_authorized,
    )
}

pub fn replay_dnf1_canonical_contexts_v1_json(json: &str) -> DnfReplay {
    match serde_json::from_str::<Dnf1CanonicalContextsCertificate>(json) {
        Ok(certificate) => replay_dnf1_canonical_contexts_v1(&certificate),
        Err(error) => failed_replay(format!("invalid DNF-1 JSON: {error}")),
    }
}

pub fn replay_dnf2_unified_judgments_v1_json(json: &str) -> DnfReplay {
    match serde_json::from_str::<Dnf2UnifiedJudgmentsCertificate>(json) {
        Ok(certificate) => replay_dnf2_unified_judgments_v1(&certificate),
        Err(error) => failed_replay(format!("invalid DNF-2 JSON: {error}")),
    }
}

pub fn replay_dnf3_naturality_closure_v1_json(json: &str) -> DnfReplay {
    match serde_json::from_str::<Dnf3NaturalityClosureCertificate>(json) {
        Ok(certificate) => replay_dnf3_naturality_closure_v1(&certificate),
        Err(error) => failed_replay(format!("invalid DNF-3 JSON: {error}")),
    }
}

pub fn replay_dnf4_corpus_projection_v1_json(json: &str) -> DnfReplay {
    match serde_json::from_str::<Dnf4CorpusProjectionCertificate>(json) {
        Ok(certificate) => replay_dnf4_corpus_projection_v1(&certificate),
        Err(error) => failed_replay(format!("invalid DNF-4 JSON: {error}")),
    }
}

fn gap_markdown(gaps: &[DnfNamedGap]) -> String {
    gaps.iter()
        .map(|gap| format!("- `{}`: {}", gap.gap_id, gap.exact_obstruction))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn render_dnf1(certificate: &Dnf1CanonicalContextsCertificate) -> String {
    format!(
        "# DNF-1 canonical dependent contexts result\n\n\
**Status:** `{:?}`. **Certificate:** `{}`.\n\n\
The frozen domain bounds replay unchanged: entries/clauses {}/{}, context {}, binder {}, dimension {}, semantic depth {}. The implemented Type/Element fragment canonicalizes two renamed presentations to the same dependent context, and the dependent prefix is kernel-typed. This local theorem is `{}`.\n\n\
The full context grammar is finitely enumerated: `{}`. Universal canonicity: `{}`.\n\n\
## Named gaps\n\n{}\n\n\
T-D2-2 reopened: `false`. M-4 authorized: `false`.\n",
        certificate.status,
        certificate.result_digest,
        certificate.frozen_public_entry_count,
        certificate.frozen_public_clause_count,
        certificate.frozen_context_arity_bound,
        certificate.frozen_binder_bound,
        certificate.frozen_dimension_bound,
        certificate.frozen_semantic_depth_bound,
        certificate.type_and_element_fragment_canonicalized,
        certificate.full_context_grammar_finitely_enumerated,
        certificate.universal_canonicity_proved,
        gap_markdown(&certificate.named_gaps),
    )
}

pub fn render_dnf2(certificate: &Dnf2UnifiedJudgmentsCertificate) -> String {
    format!(
        "# DNF-2 unified typed judgments result\n\n\
**Status:** `{:?}`. **Certificate:** `{}`.\n\n\
Exact payload-preserving translations: {}/{} (core {}, ordinary {}, cubical {}, R1 {}). Locally context-complete typed judgments: {}. All four surfaces are represented: `{}`; every source payload is retained byte-structurally: `{}`.\n\n\
Every surface has a full typed judgment: `{}`. Equality composition on the full translated domain: `{}`.\n\n\
## Named gaps\n\n{}\n\n\
No archive identifier or digest was used as missing presentation content. T-D2-2 reopened: `false`. M-4 authorized: `false`.\n",
        certificate.status,
        certificate.result_digest,
        certificate.exact_payload_translation_count,
        certificate.translations.len(),
        certificate.core_translation_count,
        certificate.ordinary_translation_count,
        certificate.cubical_translation_count,
        certificate.r1_translation_count,
        certificate.locally_context_complete_count,
        certificate.all_four_surfaces_represented,
        certificate.every_translation_drops_no_content,
        certificate.every_surface_has_full_typed_judgment,
        certificate.translations_compose_with_frozen_equality,
        gap_markdown(&certificate.named_gaps),
    )
}

pub fn render_dnf3(certificate: &Dnf3NaturalityClosureCertificate) -> String {
    format!(
        "# DNF-3 naturality closure result\n\n\
**Status:** `{:?}`. **Certificate:** `{}`.\n\n\
Concrete substitution objects and normalization squares replay: {}/{} (direct {}, pointwise {}). These rows carry expressions, substitutions, and equality witnesses; they are not count proxies.\n\n\
Every legal substitution enumerated: `{}`. Generic normalization induction: `{}`. Quotient congruence: `{}`. Universal naturality closure: `{}`.\n\n\
## Named gaps\n\n{}\n\n\
No concrete row or proof hash was promoted to a universal theorem. T-D2-2 reopened: `false`. M-4 authorized: `false`.\n",
        certificate.status,
        certificate.result_digest,
        certificate
            .concrete_squares
            .iter()
            .filter(|square| square.exact_substitution_replayed && square.normalization_square_replayed)
            .count(),
        certificate.concrete_square_count,
        certificate.direct_square_count,
        certificate.pointwise_square_count,
        certificate.every_legal_substitution_enumerated,
        certificate.normalization_commutes_by_constructor_induction,
        certificate.quotient_congruence_under_every_legal_substitution,
        certificate.universal_naturality_closure_proved,
        gap_markdown(&certificate.named_gaps),
    )
}

pub fn render_dnf4(certificate: &Dnf4CorpusProjectionCertificate) -> String {
    let mut out = format!(
        "# DNF-4 full-content corpus projection result\n\n\
**Status:** `{:?}`. **Certificate:** `{}`.\n\n\
All {}/{} rows are explicitly disposed: families {}/61, A3 {}/89. Projected: {} families and {} A3 rows. Named gaps: {} families and {} A3 rows. No sampling: `{}`. No backfill: `{}`.\n\n\
The 72 chronological A3 rows retain their reconstructed local substitution objects and commuting squares, but remain named gaps because family-level all-substitution naturality is unproved. The 17 unary rows expose only claim-granularity evidence. The 61 family rows retain every available source presentation but lack the complete context/naturality package required by the quotient.\n\n\
## Per-row dispositions\n\n\
| Kind | Stage | Source | Surface | Local content | Disposition | Gap |\n\
|---|---:|---|---|---:|---|---|\n",
        certificate.status,
        certificate.result_digest,
        certificate.observed_total_row_count,
        certificate.expected_total_row_count,
        certificate.observed_family_row_count,
        certificate.observed_a3_row_count,
        certificate.projected_family_count,
        certificate.projected_a3_count,
        certificate.named_gap_family_count,
        certificate.named_gap_a3_count,
        certificate.no_representative_sampling,
        certificate.no_backfill,
    );
    for row in &certificate.rows {
        let (disposition, gap) = match &row.disposition {
            Dnf4ProjectionDisposition::Projected { .. } => ("projected", ""),
            Dnf4ProjectionDisposition::NamedGap { gap_id, .. } => ("named-gap", gap_id.as_str()),
        };
        out.push_str(&format!(
            "| `{:?}` | {} | `{}` | `{}` | {} | `{}` | `{}` |\n",
            row.row_kind,
            row.stage,
            row.source_identifier,
            row.source_surface,
            row.source_content_reconstructed,
            disposition,
            gap,
        ));
    }
    out.push_str(
        "\nFull-content projection complete: `false`. T-D2-1 v3 may recognize all rows: `false`. T-D2-2 reopened: `false`. M-4 authorized: `false`.\n",
    );
    out
}

#[derive(Clone)]
pub struct DnfArtifactPayload {
    pub path: PathBuf,
    pub bytes: Vec<u8>,
}

fn json_payload<T: Serialize>(
    path: &Path,
    certificate: &T,
) -> Result<DnfArtifactPayload, DnfqError> {
    let mut bytes = serde_json::to_vec_pretty(certificate)
        .map_err(|error| DnfqError::Json(error.to_string()))?;
    bytes.push(b'\n');
    Ok(DnfArtifactPayload {
        path: path.to_path_buf(),
        bytes,
    })
}

fn text_payload(path: &Path, report: String) -> DnfArtifactPayload {
    DnfArtifactPayload {
        path: path.to_path_buf(),
        bytes: report.into_bytes(),
    }
}

pub fn dnf_artifact_payloads(docs: &Path) -> Result<Vec<DnfArtifactPayload>, DnfqError> {
    let suite = expected_suite()?;
    Ok(vec![
        json_payload(&docs.join("dnf1_canonical_contexts_v1.json"), &suite.dnf1)?,
        text_payload(
            &docs.join("DNF1_CANONICAL_CONTEXTS_RESULT.md"),
            render_dnf1(&suite.dnf1),
        ),
        json_payload(&docs.join("dnf2_unified_judgments_v1.json"), &suite.dnf2)?,
        text_payload(
            &docs.join("DNF2_UNIFIED_JUDGMENTS_RESULT.md"),
            render_dnf2(&suite.dnf2),
        ),
        json_payload(&docs.join("dnf3_naturality_closure_v1.json"), &suite.dnf3)?,
        text_payload(
            &docs.join("DNF3_NATURALITY_CLOSURE_RESULT.md"),
            render_dnf3(&suite.dnf3),
        ),
        json_payload(&docs.join("dnf4_corpus_projection_v1.json"), &suite.dnf4)?,
        text_payload(
            &docs.join("DNF4_CORPUS_PROJECTION_RESULT.md"),
            render_dnf4(&suite.dnf4),
        ),
    ])
}

pub fn emit_dnf_artifacts_create_new(docs: &Path) -> Result<(), DnfqError> {
    let payloads = dnf_artifact_payloads(docs)?;
    if let Some(existing) = payloads.iter().find(|payload| payload.path.exists()) {
        return Err(DnfqError::Io(format!(
            "create-new target already exists: {}",
            existing.path.display()
        )));
    }
    let mut created = Vec::<PathBuf>::new();
    let result = (|| -> Result<(), DnfqError> {
        for payload in &payloads {
            let mut file = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&payload.path)
                .map_err(|error| DnfqError::Io(error.to_string()))?;
            created.push(payload.path.clone());
            file.write_all(&payload.bytes)
                .and_then(|_| file.sync_all())
                .map_err(|error| DnfqError::Io(error.to_string()))?;
        }
        for payload in &payloads {
            let emitted = read(&payload.path).map_err(|error| DnfqError::Io(error.to_string()))?;
            if emitted != payload.bytes {
                return Err(DnfqError::Invariant(format!(
                    "emitted bytes drifted for {}",
                    payload.path.display()
                )));
            }
        }
        let replays = [
            replay_dnf1_canonical_contexts_v1_json(
                &read_to_string(docs.join("dnf1_canonical_contexts_v1.json"))
                    .map_err(|error| DnfqError::Io(error.to_string()))?,
            ),
            replay_dnf2_unified_judgments_v1_json(
                &read_to_string(docs.join("dnf2_unified_judgments_v1.json"))
                    .map_err(|error| DnfqError::Io(error.to_string()))?,
            ),
            replay_dnf3_naturality_closure_v1_json(
                &read_to_string(docs.join("dnf3_naturality_closure_v1.json"))
                    .map_err(|error| DnfqError::Io(error.to_string()))?,
            ),
            replay_dnf4_corpus_projection_v1_json(
                &read_to_string(docs.join("dnf4_corpus_projection_v1.json"))
                    .map_err(|error| DnfqError::Io(error.to_string()))?,
            ),
        ];
        if replays.iter().any(|replay| !replay.valid) {
            return Err(DnfqError::Invariant(
                "at least one emitted DNF certificate failed replay".to_owned(),
            ));
        }
        Ok(())
    })();
    if let Err(error) = result {
        for path in created {
            let _ = remove_file(path);
        }
        return Err(error);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reseal_dnf1(certificate: &mut Dnf1CanonicalContextsCertificate) {
        certificate.result_digest = dnf1_digest(certificate);
    }

    fn reseal_dnf2(certificate: &mut Dnf2UnifiedJudgmentsCertificate) {
        certificate.result_digest = dnf2_digest(certificate);
    }

    fn reseal_dnf3(certificate: &mut Dnf3NaturalityClosureCertificate) {
        certificate.result_digest = dnf3_digest(certificate);
    }

    fn reseal_dnf4(certificate: &mut Dnf4CorpusProjectionCertificate) {
        certificate.result_digest = dnf4_digest(certificate);
    }

    #[test]
    fn dnf1_preserves_bounds_and_stops_at_the_full_context_theorem() {
        let certificate = issue_dnf1_canonical_contexts_v1().expect("DNF-1");
        assert!(replay_dnf1_canonical_contexts_v1(&certificate).valid);
        assert!(certificate.bounds_replayed_unchanged);
        assert!(
            certificate.type_and_element_fragment_canonicalized,
            "{:#?}",
            certificate.canonicalization_probe
        );
        assert!(!certificate.full_context_grammar_finitely_enumerated);
        assert!(!certificate.universal_canonicity_proved);
        assert_eq!(certificate.named_gaps.len(), 3);
    }

    #[test]
    fn dnf2_retains_all_source_payloads_without_claiming_missing_contexts() {
        let certificate = issue_dnf2_unified_judgments_v1().expect("DNF-2");
        assert!(replay_dnf2_unified_judgments_v1(&certificate).valid);
        assert_eq!(certificate.core_translation_count, 62);
        assert_eq!(certificate.ordinary_translation_count, 24);
        assert_eq!(certificate.cubical_translation_count, 19);
        assert_eq!(certificate.r1_translation_count, 1);
        assert!(certificate.every_translation_drops_no_content);
        assert!(!certificate.every_surface_has_full_typed_judgment);
    }

    #[test]
    fn dnf3_reconstructs_all_72_squares_but_never_generalizes_from_them() {
        let certificate = issue_dnf3_naturality_closure_v1().expect("DNF-3");
        assert!(replay_dnf3_naturality_closure_v1(&certificate).valid);
        assert_eq!(certificate.direct_square_count, 64);
        assert_eq!(certificate.pointwise_square_count, 8);
        assert!(certificate.every_concrete_square_replayed);
        assert!(!certificate.universal_naturality_closure_proved);
        assert!(!certificate.sampled_square_or_hash_promoted_to_universal_theorem);
    }

    #[test]
    fn dnf4_explicitly_disposes_all_150_rows_without_backfill() {
        let certificate = issue_dnf4_corpus_projection_v1().expect("DNF-4");
        assert!(replay_dnf4_corpus_projection_v1(&certificate).valid);
        assert_eq!(certificate.observed_family_row_count, 61);
        assert_eq!(certificate.observed_a3_row_count, 89);
        assert_eq!(certificate.rows.len(), 150);
        assert!(certificate.all_150_rows_explicitly_disposed);
        assert_eq!(certificate.projected_family_count, 0);
        assert_eq!(certificate.projected_a3_count, 0);
        assert_eq!(certificate.named_gap_family_count, 61);
        assert_eq!(certificate.named_gap_a3_count, 89);
        assert!(!certificate.full_content_projection_complete);
    }

    #[test]
    fn resealed_nested_forgery_is_rejected_at_every_layer() {
        let mut dnf1 = issue_dnf1_canonical_contexts_v1().expect("DNF-1");
        dnf1.universal_canonicity_proved = true;
        reseal_dnf1(&mut dnf1);
        assert!(!replay_dnf1_canonical_contexts_v1(&dnf1).valid);

        let mut dnf2 = issue_dnf2_unified_judgments_v1().expect("DNF-2");
        dnf2.translations[0].unified_presentation = Value::Null;
        reseal_dnf2(&mut dnf2);
        assert!(!replay_dnf2_unified_judgments_v1(&dnf2).valid);

        let mut dnf3 = issue_dnf3_naturality_closure_v1().expect("DNF-3");
        dnf3.universal_naturality_closure_proved = true;
        reseal_dnf3(&mut dnf3);
        assert!(!replay_dnf3_naturality_closure_v1(&dnf3).valid);

        let mut dnf4 = issue_dnf4_corpus_projection_v1().expect("DNF-4");
        dnf4.rows[0].disposition = Dnf4ProjectionDisposition::Projected {
            projection_derivation_hash: "forged".to_owned(),
        };
        dnf4.projected_family_count = 1;
        reseal_dnf4(&mut dnf4);
        let replay = replay_dnf4_corpus_projection_v1(&dnf4);
        assert!(!replay.valid);
        assert!(!replay.t_d2_2_reopened);
        assert!(!replay.m4_authorized);
    }

    #[test]
    fn unknown_json_fields_fail_closed() {
        let certificate = issue_dnf4_corpus_projection_v1().expect("DNF-4");
        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("object")
            .insert("unknown_field".to_owned(), Value::Bool(true));
        let replay =
            replay_dnf4_corpus_projection_v1_json(&serde_json::to_string(&value).expect("json"));
        assert!(!replay.valid);
        assert_eq!(replay.status, None);
        assert!(!replay.t_d2_2_reopened);
        assert!(!replay.m4_authorized);
    }
}
