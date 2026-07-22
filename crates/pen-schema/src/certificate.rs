//! Fail-closed SCHEMA2 v1 phase-batch certificate.
//!
//! This certificate closes only the E-1 context/substitution fragment.  E-2
//! stops at two choices which the frozen record does not decide.  The stop is
//! intentional: no historical cardinality, threshold, or later acceptance
//! result is used to manufacture a grammar constructor.

use crate::context::{
    BinderId, CofibrationExpr, Declaration, DimExpr, FormedSchemaContext, SubstitutionImage,
    TermExpr, TypeExpr, TypedExpression, compose_typed_substitutions, exchange_adjacent,
    form_schema_context, identity_substitution, issue_substitution_preservation,
    issue_substitution_support, issue_typed_substitution, replay_adjacent_exchange,
    replay_formed_context, replay_substitution_preservation, replay_substitution_support,
    replay_typed_substitution, replay_typed_substitution_composition, weakening_substitution,
};
use crate::trunc_regression::{
    TruncEndpointMapKind, issue_trunc_endpoint_map_regression, replay_trunc_endpoint_map_regression,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

pub const SCHEMA2_CERTIFICATE_SCHEMA: &str = "schema2-phase-batch-v1";
pub const SCHEMA2_CERTIFICATE_DATE: &str = "2026-07-19";
pub const STAGE1_UNIVERSE_ADJUDICATION: &str =
    "ADJUDICATION_REQUIRED_STAGE1_UNIVERSE_PACKAGE_BOUNDARY";
pub const STEP8_POST_PATH_ADJUDICATION: &str =
    "ADJUDICATION_REQUIRED_STEP8_POST_PATH_TYPED_SEMANTICS";

const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_e_schema2_plan.md");
const OPEN_PROBLEM_BYTES: &[u8] =
    include_bytes!("../../../docs/step_15_completion_open_problem.md");
const SEMANTIC_PROGRAM_BYTES: &[u8] =
    include_bytes!("../../../docs/SEMANTIC_NORMALIZATION_PROGRAM.md");
const T4_BYTES: &[u8] = include_bytes!("../../../docs/SH1_T4_BLIND_VALUATION_SPEC.md");
const SEMANTIC_RESELECTION_BYTES: &[u8] = include_bytes!("../../../docs/semantic_reselection.json");
const HIST_CERT_V3_BYTES: &[u8] = include_bytes!("../../../docs/hist_cert_v3.json");
const TRUNC_ENDPOINT_BYTES: &[u8] = include_bytes!("../../../docs/trunc_endpoint_realizer_v1.json");
const KERNEL_BRIDGE_AGDA_BYTES: &[u8] = include_bytes!("../../../agda/KernelBridge.agda");
const SCHEMA2_AGDA_BYTES: &[u8] = include_bytes!("../../../agda/Schema2.agda");

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Schema2CertificateError {
    #[error("source artifact {name} failed exact binding: {reason}")]
    SourceBinding { name: String, reason: String },
    #[error("source artifact {name} has an unexpected shape: {reason}")]
    SourceShape { name: String, reason: String },
    #[error("E-1 context/substitution replay failed: {0}")]
    Context(String),
    #[error("E-1 TRUNC endpoint replay failed: {0}")]
    Trunc(String),
    #[error("E-2 ordinary-family projection failed: {0}")]
    OrdinaryProjection(String),
    #[error("certificate JSON failed: {0}")]
    Json(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceBinding {
    pub name: String,
    pub path: String,
    pub byte_length: u64,
    pub blake3: String,
    pub exact_bytes_pinned: bool,
    pub certificate_projection_replayed: bool,
    pub role: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PhaseStatus {
    pub phase: String,
    pub status: String,
    pub theorem_token_issued: bool,
    pub blocked_by: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EndpointMapKind {
    Identity,
    Swap,
    CollapseToX,
    CollapseToY,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EndpointMapEvidence {
    pub kind: EndpointMapKind,
    pub zero_target_binder: u32,
    pub one_target_binder: u32,
    pub dependent_images_checked: u32,
    pub genuine_expression_images: u32,
    pub substitution_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E1Evidence {
    pub context_fragment_version: String,
    pub five_context_entry_kinds_formed: bool,
    pub formed_context_derivation_hash: String,
    pub identity_replayed: bool,
    pub identity_derivation_hash: String,
    pub genuine_dependent_expression_image_replayed: bool,
    pub genuine_expression_image_count: u32,
    pub genuine_instance_derivation_hash: String,
    pub expression_typing_preservation_replayed: bool,
    pub preservation_witness_count: u32,
    pub substitution_support_replayed: bool,
    pub support_witness_count: u32,
    pub identity_support_exact: bool,
    pub genuine_target_support_within_image_bound: bool,
    pub support_token_inventory_digest: String,
    pub composition_replayed: bool,
    pub composition_sequential_images_equal: bool,
    pub composition_token_digest: String,
    pub weakening_replayed: bool,
    pub weakening_derivation_hash: String,
    pub legal_adjacent_exchange_replayed: bool,
    pub exchange_inverse_on_declarations: bool,
    pub exchange_token_digest: String,
    pub trunc_endpoint_maps_rederived: bool,
    pub trunc_endpoint_exact_four_map_join: bool,
    pub trunc_endpoint_maps: Vec<EndpointMapEvidence>,
    pub trunc_regression_token_digest: String,
    pub agda_sources_declare_safe_without_k: bool,
    pub agda_sources_contain_no_postulate_declaration: bool,
    pub agda_compiler_check_is_external_acceptance_grader: bool,
    pub agda_to_rust_issuer_soundness_proved: bool,
    pub all_pen_core_expr_to_typed_e1_elaboration_proved: bool,
    pub rust_graders_replayed: bool,
    pub scoped_e1_fragment_proved: bool,
    pub global_c1_retired: bool,
    pub theorem_token_digest: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdinaryFamilyKind {
    FreshFormation,
    PointOrUnitIntro,
    PathConstructorIntro,
    Recursor,
    Inductor,
    TruncParametricAction,
    PostPathOperation,
    PostPathCoherence,
    PostPathCellAction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrdinaryFamilyProjectionRow {
    pub legacy_kind: String,
    pub legacy_role: Option<String>,
    pub semantic_kind: OrdinaryFamilyKind,
    pub source_clauses: Vec<u32>,
    pub required_opaque_token_kind: String,
    pub archived_natural_family_proved: bool,
    pub archived_full_predecessor_sweep_proved: bool,
    pub typed_realizer_issued: bool,
    pub typed_realizer_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrdinaryStepProjection {
    pub step: u32,
    pub rows: Vec<OrdinaryFamilyProjectionRow>,
    pub derived_row_count: u32,
    pub path_constructor_intro_preserved: bool,
    pub typed_realizer_inventory_complete: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct HistoricalRegressionProjection {
    pub raw_operational_totals: Vec<u32>,
    pub registered_path_typed_and_marginal_subtotals: Vec<u32>,
    pub ordinary_unresolved_counts_derived_from_rows: Vec<u32>,
    pub ordinary_steps: Vec<OrdinaryStepProjection>,
    pub archive_regression_signature_replayed: bool,
    pub ordinary_counts_match_archived_unresolved_projection: bool,
    pub count_used_as_constructor_input: bool,
    pub ordinary_typed_realizers_complete: bool,
    pub full_historical_totals_certified: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage1UniverseDiagnostic {
    pub stage: u32,
    pub candidate_hash: String,
    pub clause_forms: Vec<String>,
    pub burned_extractor_family_units: u32,
    pub sealed_record_family_units: u32,
    pub family_unit_mismatch_reproduced: bool,
    pub diagnosis: String,
    pub frozen_record_decides_package_boundary: bool,
    pub count_used_to_choose_package_boundary: bool,
    pub adjudication_id: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdjudicationOption {
    pub option_id: String,
    pub interpretation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AdjudicationRequired {
    pub id: String,
    pub phase: String,
    pub constructor_scope: String,
    pub issue: String,
    pub frozen_record_decides: bool,
    pub options: Vec<AdjudicationOption>,
    pub unresolved_questions: Vec<String>,
    pub selected_option: Option<String>,
    pub typed_realizer_token: Option<String>,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E2Evidence {
    pub status: String,
    pub preflight_only_while_global_e1_remains_open: bool,
    pub historical_projection: HistoricalRegressionProjection,
    pub stage1_universe_diagnostic: Stage1UniverseDiagnostic,
    pub adjudications: Vec<AdjudicationRequired>,
    pub every_adjudication_unselected: bool,
    pub every_blocked_constructor_has_no_typed_realizer_token: bool,
    pub grammar_complete: bool,
    pub ordinary_family_realizers_complete: bool,
    pub historical_ordinary_remainder_closed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObstructionStatus {
    pub c1_global_arbitrary_typed_instance_sort_preservation: bool,
    pub c2_depth_two_schema_grammar_and_generator_completeness: bool,
    pub c6_general_historical_term_level_completion: bool,
    pub d4_cw_instance_grammar_orbit_quotient_membership: bool,
    pub c8_candidate_boundary_provenance_join: bool,
    pub domain_wide_c3_candidate_extraction_bridge: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardRailStatus {
    pub f1_executable: bool,
    pub f1_triggered: bool,
    pub f1_excluded: bool,
    pub f5_executable: bool,
    pub f5_triggered: bool,
    pub f5_excluded: bool,
    pub semantic_o16_empty: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConclusionBoundary {
    pub later_acceptance_computation_run: bool,
    pub closing_inequality_computed: bool,
    pub global_halt_proved: bool,
    pub global_continuation_proved: bool,
    pub permitted_conclusion: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schema2Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<SourceBinding>,
    pub phases: Vec<PhaseStatus>,
    pub e1: E1Evidence,
    pub e2: E2Evidence,
    pub obstructions_retired: ObstructionStatus,
    pub guard_rails: GuardRailStatus,
    pub conclusion_boundary: ConclusionBoundary,
    pub outcome: String,
    pub remaining_obligations: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schema2Replay {
    pub valid: bool,
    pub e1_complete: bool,
    pub e1_scoped_fragment_complete: bool,
    pub e2_adjudication_required: bool,
    pub unresolved_adjudication_count: u32,
    pub later_phases_blocked_by_strict_order: bool,
    pub global_c1_retired: bool,
    pub c2_retired: bool,
    pub semantic_o16_decided: bool,
    pub global_halt_proved: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(SCHEMA2_CERTIFICATE_SCHEMA, domain, value))
        .expect("SCHEMA2 proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

fn certificate_digest(certificate: &Schema2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("phase-batch-certificate", &projection)
}

fn bind_source(
    name: &str,
    path: &str,
    role: &str,
    bytes: &[u8],
    expected_length: u64,
    expected_blake3: &str,
    certificate_projection_replayed: bool,
) -> Result<SourceBinding, Schema2CertificateError> {
    let byte_length = bytes.len() as u64;
    let blake3 = bytes_hash(bytes);
    if byte_length != expected_length || blake3 != expected_blake3 {
        return Err(Schema2CertificateError::SourceBinding {
            name: name.to_owned(),
            reason: format!(
                "expected {expected_length}/{expected_blake3}, found {byte_length}/{blake3}"
            ),
        });
    }
    Ok(SourceBinding {
        name: name.to_owned(),
        path: path.to_owned(),
        byte_length,
        blake3,
        exact_bytes_pinned: true,
        certificate_projection_replayed,
        role: role.to_owned(),
    })
}

#[allow(clippy::type_complexity)]
fn source_manifest() -> [(
    &'static str,
    &'static str,
    &'static str,
    &'static [u8],
    u64,
    &'static str,
    bool,
); 9] {
    [
        (
            "agent_e_schema2_plan",
            "docs/agent_e_schema2_plan.md",
            "governing_execution_plan",
            PLAN_BYTES,
            9_562,
            "blake3:d06d2039f1cd83e2d5509478aede30d2e09e2548f262cea9e0f88abbfe2767db",
            false,
        ),
        (
            "step_15_completion_open_problem",
            "docs/step_15_completion_open_problem.md",
            "frozen_requirements_document",
            OPEN_PROBLEM_BYTES,
            15_507,
            "blake3:2026c9a017f3723cadcadc70890dcc416ef37ea4f051af06d29ceeffe6c8b544",
            false,
        ),
        (
            "semantic_normalization_program",
            "docs/SEMANTIC_NORMALIZATION_PROGRAM.md",
            "frozen_family_instance_definitions",
            SEMANTIC_PROGRAM_BYTES,
            24_891,
            "blake3:8f54933d8f4d7d5f8a6d1c6dd1a37985aaf4dd3cedee1ab6565514460f622c96",
            false,
        ),
        (
            "sh1_t4_blind_valuation_spec",
            "docs/SH1_T4_BLIND_VALUATION_SPEC.md",
            "frozen_path_family_rule",
            T4_BYTES,
            9_340,
            "blake3:bc10291c85342904bee92c22c2b38d92cea93e5d8bcfe3926ed2d6f21805654b",
            false,
        ),
        (
            "semantic_reselection_v1",
            "docs/semantic_reselection.json",
            "burned_stage1_diagnostic",
            SEMANTIC_RESELECTION_BYTES,
            4_107,
            "blake3:c3dc9a3060abb68985ace5ed1cf1b465f074f86c74a41506ab991f08460ba6e7",
            true,
        ),
        (
            "hist_cert_v3",
            "docs/hist_cert_v3.json",
            "archived_historical_regression",
            HIST_CERT_V3_BYTES,
            178_331,
            "blake3:dee9ceed570935917a7e40130caffdb014af412080b7dd9592c2fe8a59c9b05d",
            true,
        ),
        (
            "trunc_endpoint_realizer_v1",
            "docs/trunc_endpoint_realizer_v1.json",
            "archived_endpoint_realizer",
            TRUNC_ENDPOINT_BYTES,
            21_372,
            "blake3:f878c10711a29d3f34ee279cdee8ada56d7aac32a9317452c6f4d60fef7c01ee",
            true,
        ),
        (
            "kernel_bridge_agda",
            "agda/KernelBridge.agda",
            "read_only_agda_base",
            KERNEL_BRIDGE_AGDA_BYTES,
            11_224,
            "blake3:4c6f0c215837d0b81fbd472d4119357a41c86d20989b5d91575de614c37a2095",
            false,
        ),
        (
            "schema2_agda",
            "agda/Schema2.agda",
            "e1_agda_proof_source",
            SCHEMA2_AGDA_BYTES,
            25_002,
            "blake3:06531792b49eb1bba506247bea96e00c1422ca42dabd8726f206d531833f6a30",
            false,
        ),
    ]
}

fn source_bindings() -> Result<Vec<SourceBinding>, Schema2CertificateError> {
    source_manifest()
        .into_iter()
        .map(|(name, path, role, bytes, length, digest, replayed)| {
            bind_source(name, path, role, bytes, length, digest, replayed)
        })
        .collect()
}

fn proof_source_discipline_holds(bytes: &[u8]) -> bool {
    let source = String::from_utf8_lossy(bytes);
    source.contains("{-# OPTIONS --safe --without-K #-}")
        && !source.lines().any(|line| {
            let trimmed = line.trim_start();
            trimmed == "postulate" || trimmed.starts_with("postulate ")
        })
}

fn five_kind_context() -> Result<FormedSchemaContext, Schema2CertificateError> {
    let a = TypeExpr::parameter(1);
    form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(1),
            name: "A".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(2),
            name: "x".to_owned(),
            ty: a.clone(),
        },
        Declaration::LibraryReference {
            binder: BinderId(3),
            name: "base".to_owned(),
            step: 1,
            symbol: "base".to_owned(),
            ty: a,
        },
        Declaration::IntervalVariable {
            binder: BinderId(4),
            name: "i".to_owned(),
        },
        Declaration::CofibrationAssumption {
            binder: BinderId(5),
            name: "i0".to_owned(),
            formula: CofibrationExpr::endpoint(4, false),
        },
    ])
    .map_err(|error| Schema2CertificateError::Context(error.to_string()))
}

fn point_instance_contexts(
    source_type: u32,
    source_point: u32,
    target_type: u32,
    target_point: u32,
) -> Result<(FormedSchemaContext, FormedSchemaContext), Schema2CertificateError> {
    let source = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(source_type),
            name: format!("A{source_type}"),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(source_point),
            name: format!("x{source_point}"),
            ty: TypeExpr::trunc(TypeExpr::parameter(source_type)),
        },
    ])
    .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    let target = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(target_type),
            name: format!("A{target_type}"),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(target_point),
            name: format!("a{target_point}"),
            ty: TypeExpr::parameter(target_type),
        },
    ])
    .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    Ok((source, target))
}

fn map_kind(kind: TruncEndpointMapKind) -> EndpointMapKind {
    match kind {
        TruncEndpointMapKind::Identity => EndpointMapKind::Identity,
        TruncEndpointMapKind::Swap => EndpointMapKind::Swap,
        TruncEndpointMapKind::CollapseToX => EndpointMapKind::CollapseToX,
        TruncEndpointMapKind::CollapseToY => EndpointMapKind::CollapseToY,
    }
}

fn derive_e1_evidence() -> Result<E1Evidence, Schema2CertificateError> {
    let context = five_kind_context()?;
    replay_formed_context(&context)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;

    let identity = identity_substitution(&context)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    replay_typed_substitution(&identity)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;

    let expressions = [
        TypedExpression::Type {
            expression: TypeExpr::parameter(1),
        },
        TypedExpression::Term {
            expression: TermExpr::variable(2),
            ty: TypeExpr::parameter(1),
        },
        TypedExpression::Dimension {
            expression: DimExpr::Variable(BinderId(4)),
        },
        TypedExpression::Cofibration {
            expression: CofibrationExpr::endpoint(4, false),
        },
    ];
    let mut support_token_digests = Vec::new();
    let mut identity_support_exact = true;
    for expression in expressions {
        let witness = issue_substitution_preservation(&identity, expression.clone())
            .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
        replay_substitution_preservation(&identity, &witness)
            .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
        if !witness.preserved() {
            return Err(Schema2CertificateError::Context(
                "a substitution-preservation witness was not preserved".to_owned(),
            ));
        }
        let support = issue_substitution_support(&identity, expression)
            .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
        replay_substitution_support(&identity, &support)
            .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
        identity_support_exact &= support.target_support_within_image_bound()
            && support.target_support() == support.source_support();
        support_token_digests.push(tagged_hash("e1-support-token", &support));
    }

    let (point_source, point_middle) = point_instance_contexts(1, 2, 10, 11)?;
    let genuine = issue_typed_substitution(
        &point_source,
        &point_middle,
        vec![
            SubstitutionImage::Type {
                source: BinderId(1),
                image: TypeExpr::parameter(10),
            },
            SubstitutionImage::Term {
                source: BinderId(2),
                image: TermExpr::TruncPoint {
                    carrier: Box::new(TypeExpr::parameter(10)),
                    point: Box::new(TermExpr::variable(11)),
                },
            },
        ],
    )
    .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    replay_typed_substitution(&genuine)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    let source_point_expression = TypedExpression::Term {
        expression: TermExpr::variable(2),
        ty: TypeExpr::trunc(TypeExpr::parameter(1)),
    };
    let genuine_preservation =
        issue_substitution_preservation(&genuine, source_point_expression.clone())
            .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    replay_substitution_preservation(&genuine, &genuine_preservation)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    let genuine_support = issue_substitution_support(&genuine, source_point_expression.clone())
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    replay_substitution_support(&genuine, &genuine_support)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    support_token_digests.push(tagged_hash("e1-support-token", &genuine_support));

    let point_target = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(20),
            name: "A20".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(21),
            name: "a21".to_owned(),
            ty: TypeExpr::parameter(20),
        },
    ])
    .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    let second = issue_typed_substitution(
        &point_middle,
        &point_target,
        vec![
            SubstitutionImage::Type {
                source: BinderId(10),
                image: TypeExpr::parameter(20),
            },
            SubstitutionImage::Term {
                source: BinderId(11),
                image: TermExpr::variable(21),
            },
        ],
    )
    .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    let composition = compose_typed_substitutions(&genuine, &second)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    replay_typed_substitution_composition(&genuine, &second, &composition)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    replay_typed_substitution(composition.composed())
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    let composition_support =
        issue_substitution_support(composition.composed(), source_point_expression)
            .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    replay_substitution_support(composition.composed(), &composition_support)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    support_token_digests.push(tagged_hash("e1-support-token", &composition_support));

    let weakening_source = form_schema_context(vec![Declaration::TypeParameter {
        binder: BinderId(30),
        name: "A30".to_owned(),
        universe: 0,
    }])
    .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    let weakening_target = form_schema_context(vec![
        weakening_source.declarations()[0].clone(),
        Declaration::IntervalVariable {
            binder: BinderId(31),
            name: "i31".to_owned(),
        },
    ])
    .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    let weakening = weakening_substitution(&weakening_source, &weakening_target)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    replay_typed_substitution(&weakening)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    let exchange = exchange_adjacent(&weakening_target, 0)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    replay_adjacent_exchange(&weakening_target, &exchange)
        .map_err(|error| Schema2CertificateError::Context(error.to_string()))?;
    if !exchange.inverse_on_declarations() {
        return Err(Schema2CertificateError::Context(
            "legal adjacent exchange failed its inverse check".to_owned(),
        ));
    }

    let trunc = issue_trunc_endpoint_map_regression()
        .map_err(|error| Schema2CertificateError::Trunc(error.to_string()))?;
    replay_trunc_endpoint_map_regression(&trunc)
        .map_err(|error| Schema2CertificateError::Trunc(error.to_string()))?;
    let trunc_endpoint_maps = trunc
        .instances()
        .iter()
        .map(|instance| {
            let (zero, one) = instance.endpoints();
            EndpointMapEvidence {
                kind: map_kind(instance.kind()),
                zero_target_binder: zero,
                one_target_binder: one,
                dependent_images_checked: u32::try_from(
                    instance.substitution().dependent_images_checked(),
                )
                .expect("small E-1 count"),
                genuine_expression_images: u32::try_from(
                    instance.substitution().genuine_expression_images(),
                )
                .expect("small E-1 count"),
                substitution_derivation_hash: instance.substitution().derivation_hash().to_owned(),
            }
        })
        .collect::<Vec<_>>();
    let exact_map_inventory = trunc_endpoint_maps
        == vec![
            EndpointMapEvidence {
                kind: EndpointMapKind::Identity,
                zero_target_binder: 2,
                one_target_binder: 3,
                dependent_images_checked: 2,
                genuine_expression_images: 0,
                substitution_derivation_hash: trunc_endpoint_maps[0]
                    .substitution_derivation_hash
                    .clone(),
            },
            EndpointMapEvidence {
                kind: EndpointMapKind::Swap,
                zero_target_binder: 3,
                one_target_binder: 2,
                dependent_images_checked: 2,
                genuine_expression_images: 0,
                substitution_derivation_hash: trunc_endpoint_maps[1]
                    .substitution_derivation_hash
                    .clone(),
            },
            EndpointMapEvidence {
                kind: EndpointMapKind::CollapseToX,
                zero_target_binder: 2,
                one_target_binder: 2,
                dependent_images_checked: 2,
                genuine_expression_images: 0,
                substitution_derivation_hash: trunc_endpoint_maps[2]
                    .substitution_derivation_hash
                    .clone(),
            },
            EndpointMapEvidence {
                kind: EndpointMapKind::CollapseToY,
                zero_target_binder: 3,
                one_target_binder: 3,
                dependent_images_checked: 2,
                genuine_expression_images: 0,
                substitution_derivation_hash: trunc_endpoint_maps[3]
                    .substitution_derivation_hash
                    .clone(),
            },
        ];
    if !exact_map_inventory {
        return Err(Schema2CertificateError::Trunc(
            "the four endpoint maps changed kind, order, endpoints, or typing evidence".to_owned(),
        ));
    }

    let rust_graders_replayed = genuine.genuine_expression_images() == 1
        && genuine_preservation.preserved()
        && identity_support_exact
        && genuine_support.target_support_within_image_bound()
        && composition_support.target_support_within_image_bound()
        && composition.sequential_images_equal()
        && exchange.inverse_on_declarations()
        && trunc.exact_four_map_join()
        && exact_map_inventory;
    if !rust_graders_replayed {
        return Err(Schema2CertificateError::Context(
            "one or more E-1 graders did not replay".to_owned(),
        ));
    }
    let agda_sources_declare_safe_without_k = proof_source_discipline_holds(SCHEMA2_AGDA_BYTES)
        && proof_source_discipline_holds(KERNEL_BRIDGE_AGDA_BYTES);
    if !agda_sources_declare_safe_without_k {
        return Err(Schema2CertificateError::Context(
            "Agda source discipline check failed".to_owned(),
        ));
    }

    let mut evidence = E1Evidence {
        context_fragment_version: crate::context::SCHEMA2_CONTEXT_FRAGMENT_VERSION.to_owned(),
        five_context_entry_kinds_formed: context.declarations().len() == 5,
        formed_context_derivation_hash: context.derivation_hash().to_owned(),
        identity_replayed: true,
        identity_derivation_hash: identity.derivation_hash().to_owned(),
        genuine_dependent_expression_image_replayed: true,
        genuine_expression_image_count: u32::try_from(genuine.genuine_expression_images())
            .expect("small E-1 count"),
        genuine_instance_derivation_hash: genuine.derivation_hash().to_owned(),
        expression_typing_preservation_replayed: genuine_preservation.preserved(),
        preservation_witness_count: 5,
        substitution_support_replayed: true,
        support_witness_count: u32::try_from(support_token_digests.len())
            .expect("small E-1 support inventory"),
        identity_support_exact,
        genuine_target_support_within_image_bound: genuine_support
            .target_support_within_image_bound(),
        support_token_inventory_digest: tagged_hash(
            "e1-support-token-inventory",
            &support_token_digests,
        ),
        composition_replayed: true,
        composition_sequential_images_equal: composition.sequential_images_equal(),
        composition_token_digest: tagged_hash("e1-composition-token", &composition),
        weakening_replayed: true,
        weakening_derivation_hash: weakening.derivation_hash().to_owned(),
        legal_adjacent_exchange_replayed: true,
        exchange_inverse_on_declarations: exchange.inverse_on_declarations(),
        exchange_token_digest: tagged_hash("e1-exchange-token", &exchange),
        trunc_endpoint_maps_rederived: true,
        trunc_endpoint_exact_four_map_join: trunc.exact_four_map_join(),
        trunc_endpoint_maps,
        trunc_regression_token_digest: tagged_hash("e1-trunc-regression-token", &trunc),
        agda_sources_declare_safe_without_k,
        agda_sources_contain_no_postulate_declaration: true,
        agda_compiler_check_is_external_acceptance_grader: true,
        agda_to_rust_issuer_soundness_proved: false,
        all_pen_core_expr_to_typed_e1_elaboration_proved: false,
        rust_graders_replayed,
        scoped_e1_fragment_proved: true,
        global_c1_retired: false,
        theorem_token_digest: String::new(),
    };
    evidence.theorem_token_digest = tagged_hash("e1-scoped-c1-theorem", &evidence);
    Ok(evidence)
}

fn parse_json_source(name: &str, bytes: &[u8]) -> Result<Value, Schema2CertificateError> {
    serde_json::from_slice(bytes).map_err(|error| Schema2CertificateError::SourceShape {
        name: name.to_owned(),
        reason: error.to_string(),
    })
}

fn field<'a>(
    value: &'a Value,
    name: &str,
    source: &str,
) -> Result<&'a Value, Schema2CertificateError> {
    value
        .get(name)
        .ok_or_else(|| Schema2CertificateError::SourceShape {
            name: source.to_owned(),
            reason: format!("missing field {name}"),
        })
}

fn as_u32(value: &Value, source: &str, field_name: &str) -> Result<u32, Schema2CertificateError> {
    value
        .as_u64()
        .and_then(|number| u32::try_from(number).ok())
        .ok_or_else(|| Schema2CertificateError::SourceShape {
            name: source.to_owned(),
            reason: format!("field {field_name} is not a u32"),
        })
}

fn as_string(
    value: &Value,
    source: &str,
    field_name: &str,
) -> Result<String, Schema2CertificateError> {
    value
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| Schema2CertificateError::SourceShape {
            name: source.to_owned(),
            reason: format!("field {field_name} is not a string"),
        })
}

fn as_bool(value: &Value, source: &str, field_name: &str) -> Result<bool, Schema2CertificateError> {
    value
        .as_bool()
        .ok_or_else(|| Schema2CertificateError::SourceShape {
            name: source.to_owned(),
            reason: format!("field {field_name} is not a boolean"),
        })
}

fn u32_array(
    value: &Value,
    source: &str,
    field_name: &str,
) -> Result<Vec<u32>, Schema2CertificateError> {
    value
        .as_array()
        .ok_or_else(|| Schema2CertificateError::SourceShape {
            name: source.to_owned(),
            reason: format!("field {field_name} is not an array"),
        })?
        .iter()
        .map(|item| as_u32(item, source, field_name))
        .collect()
}

fn map_ordinary_kind(
    legacy_kind: &str,
    legacy_role: Option<&str>,
) -> Result<OrdinaryFamilyKind, Schema2CertificateError> {
    match (legacy_kind, legacy_role) {
        ("formation", _) => Ok(OrdinaryFamilyKind::FreshFormation),
        ("point_introduction", _) => Ok(OrdinaryFamilyKind::PointOrUnitIntro),
        ("path_introduction", _) => Ok(OrdinaryFamilyKind::PathConstructorIntro),
        ("recursor", _) => Ok(OrdinaryFamilyKind::Recursor),
        ("inductor", _) => Ok(OrdinaryFamilyKind::Inductor),
        ("parametric_action", _) => Ok(OrdinaryFamilyKind::TruncParametricAction),
        ("post_path_forward", Some("operation_forward")) => {
            Ok(OrdinaryFamilyKind::PostPathOperation)
        }
        ("post_path_forward", Some("coherence_forward")) => {
            Ok(OrdinaryFamilyKind::PostPathCoherence)
        }
        ("cell_action", _) => Ok(OrdinaryFamilyKind::PostPathCellAction),
        _ => Err(Schema2CertificateError::OrdinaryProjection(format!(
            "unfrozen ordinary-family key {legacy_kind}/{legacy_role:?}"
        ))),
    }
}

fn project_historical_regression() -> Result<HistoricalRegressionProjection, Schema2CertificateError>
{
    const SOURCE: &str = "hist_cert_v3";
    let root = parse_json_source(SOURCE, HIST_CERT_V3_BYTES)?;
    if field(&root, "schema", SOURCE)?.as_str() != Some("hist-cert-historical-hit-v3")
        || field(&root, "outcome", SOURCE)?.as_str() != Some("partial_but_regression_clean")
    {
        return Err(Schema2CertificateError::SourceShape {
            name: SOURCE.to_owned(),
            reason: "unexpected schema or outcome".to_owned(),
        });
    }
    let packages = field(&root, "packages", SOURCE)?
        .as_array()
        .ok_or_else(|| Schema2CertificateError::SourceShape {
            name: SOURCE.to_owned(),
            reason: "packages is not an array".to_owned(),
        })?;
    let mut ordinary_steps = Vec::new();
    for package in packages {
        let step = as_u32(field(package, "step", SOURCE)?, SOURCE, "step")?;
        let gaps = field(package, "ordinary_family_gaps", SOURCE)?
            .as_array()
            .ok_or_else(|| Schema2CertificateError::SourceShape {
                name: SOURCE.to_owned(),
                reason: format!("ordinary_family_gaps at step {step} is not an array"),
            })?;
        let mut rows = Vec::new();
        for gap in gaps {
            let key = field(gap, "key", SOURCE)?;
            let legacy_kind = as_string(field(key, "kind", SOURCE)?, SOURCE, "key.kind")?;
            let legacy_role = key
                .get("role")
                .and_then(Value::as_str)
                .map(ToOwned::to_owned);
            let semantic_kind = map_ordinary_kind(&legacy_kind, legacy_role.as_deref())?;
            let source_clauses = u32_array(
                field(gap, "source_clauses", SOURCE)?,
                SOURCE,
                "source_clauses",
            )?;
            let required_opaque_token_kind = as_string(
                field(gap, "required_opaque_token", SOURCE)?,
                SOURCE,
                "required_opaque_token",
            )?;
            let archived_natural_family_proved = as_bool(
                field(gap, "natural_family_proved", SOURCE)?,
                SOURCE,
                "natural_family_proved",
            )?;
            let archived_full_predecessor_sweep_proved = as_bool(
                field(gap, "full_predecessor_sweep_proved", SOURCE)?,
                SOURCE,
                "full_predecessor_sweep_proved",
            )?;
            if archived_natural_family_proved || archived_full_predecessor_sweep_proved {
                return Err(Schema2CertificateError::OrdinaryProjection(format!(
                    "step {step} unexpectedly contains a proved ordinary-family row"
                )));
            }
            rows.push(OrdinaryFamilyProjectionRow {
                legacy_kind,
                legacy_role,
                semantic_kind,
                source_clauses,
                required_opaque_token_kind,
                archived_natural_family_proved,
                archived_full_predecessor_sweep_proved,
                typed_realizer_issued: false,
                typed_realizer_token: None,
            });
        }
        let path_constructor_intro_preserved = rows
            .iter()
            .any(|row| row.semantic_kind == OrdinaryFamilyKind::PathConstructorIntro);
        if !path_constructor_intro_preserved {
            return Err(Schema2CertificateError::OrdinaryProjection(format!(
                "step {step} lost PathConstructorIntro"
            )));
        }
        ordinary_steps.push(OrdinaryStepProjection {
            step,
            derived_row_count: u32::try_from(rows.len()).expect("small historical inventory"),
            rows,
            path_constructor_intro_preserved,
            typed_realizer_inventory_complete: false,
        });
    }
    if ordinary_steps
        .iter()
        .map(|step| step.step)
        .collect::<Vec<_>>()
        != [5, 6, 7, 8]
    {
        return Err(Schema2CertificateError::OrdinaryProjection(
            "historical package steps are not exactly 5,6,7,8".to_owned(),
        ));
    }
    let ordinary_unresolved_counts_derived_from_rows = ordinary_steps
        .iter()
        .map(|step| step.derived_row_count)
        .collect::<Vec<_>>();
    let regression = field(&root, "f_b2", SOURCE)?;
    let archived_ordinary = u32_array(
        field(regression, "observed_ordinary_unresolved", SOURCE)?,
        SOURCE,
        "observed_ordinary_unresolved",
    )?;
    let ordinary_counts_match_archived_unresolved_projection =
        ordinary_unresolved_counts_derived_from_rows == archived_ordinary;
    if !ordinary_counts_match_archived_unresolved_projection {
        return Err(Schema2CertificateError::OrdinaryProjection(
            "row-derived ordinary inventory differs from the archive projection".to_owned(),
        ));
    }
    let raw_operational_totals = u32_array(
        field(regression, "observed_raw_totals", SOURCE)?,
        SOURCE,
        "observed_raw_totals",
    )?;
    let registered_path_typed_and_marginal_subtotals = u32_array(
        field(regression, "observed_marginal_subtotals", SOURCE)?,
        SOURCE,
        "observed_marginal_subtotals",
    )?;
    let archive_regression_signature_replayed = as_bool(
        field(regression, "f_b2_regression_signature_replayed", SOURCE)?,
        SOURCE,
        "f_b2_regression_signature_replayed",
    )?;
    if !archive_regression_signature_replayed {
        return Err(Schema2CertificateError::OrdinaryProjection(
            "archived F-B2 regression is not replayed".to_owned(),
        ));
    }
    Ok(HistoricalRegressionProjection {
        raw_operational_totals,
        registered_path_typed_and_marginal_subtotals,
        ordinary_unresolved_counts_derived_from_rows,
        ordinary_steps,
        archive_regression_signature_replayed,
        ordinary_counts_match_archived_unresolved_projection,
        count_used_as_constructor_input: false,
        ordinary_typed_realizers_complete: false,
        full_historical_totals_certified: false,
    })
}

fn diagnose_stage1_universe() -> Result<Stage1UniverseDiagnostic, Schema2CertificateError> {
    const SOURCE: &str = "semantic_reselection_v1";
    let root = parse_json_source(SOURCE, SEMANTIC_RESELECTION_BYTES)?;
    if field(&root, "burned_run", SOURCE)?.as_bool() != Some(true) {
        return Err(Schema2CertificateError::SourceShape {
            name: SOURCE.to_owned(),
            reason: "diagnostic run is not marked burned".to_owned(),
        });
    }
    let stages = field(&root, "stages", SOURCE)?.as_array().ok_or_else(|| {
        Schema2CertificateError::SourceShape {
            name: SOURCE.to_owned(),
            reason: "stages is not an array".to_owned(),
        }
    })?;
    let stage1 = stages
        .iter()
        .find(|stage| stage.get("stage").and_then(Value::as_u64) == Some(1))
        .ok_or_else(|| Schema2CertificateError::SourceShape {
            name: SOURCE.to_owned(),
            reason: "stage 1 is absent".to_owned(),
        })?;
    let winner = field(stage1, "winner", SOURCE)?;
    let candidate_hash = as_string(
        field(winner, "candidate_hash", SOURCE)?,
        SOURCE,
        "winner.candidate_hash",
    )?;
    let clauses = field(field(winner, "telescope", SOURCE)?, "clauses", SOURCE)?
        .as_array()
        .ok_or_else(|| Schema2CertificateError::SourceShape {
            name: SOURCE.to_owned(),
            reason: "winner telescope clauses is not an array".to_owned(),
        })?;
    let clause_shapes_match = clauses.len() == 2
        && clauses[0].get("expr").and_then(Value::as_str) == Some("Univ")
        && clauses[1]
            .get("expr")
            .and_then(|expr| expr.get("App"))
            .and_then(Value::as_array)
            .is_some_and(|app| {
                app.len() == 2
                    && app[0].as_str() == Some("Univ")
                    && app[1].get("Var").and_then(Value::as_u64) == Some(1)
            });
    if !clause_shapes_match {
        return Err(Schema2CertificateError::SourceShape {
            name: SOURCE.to_owned(),
            reason: "stage-1 winner is not the frozen Univ/App(Univ,Var(1)) package".to_owned(),
        });
    }
    let scored = field(stage1, "scored", SOURCE)?
        .as_array()
        .and_then(|items| items.first())
        .ok_or_else(|| Schema2CertificateError::SourceShape {
            name: SOURCE.to_owned(),
            reason: "stage-1 scored diagnostic is absent".to_owned(),
        })?;
    let burned_extractor_family_units = as_u32(
        field(scored, "marginal_families", SOURCE)?,
        SOURCE,
        "marginal_families",
    )?;
    let sealed_record_family_units =
        as_u32(field(stage1, "legacy_nu", SOURCE)?, SOURCE, "legacy_nu")?;
    let first_divergence = field(&root, "first_divergence", SOURCE)?;
    let divergence_is_stage1 = first_divergence.get("stage").and_then(Value::as_u64) == Some(1)
        && first_divergence.get("field").and_then(Value::as_str) == Some("score");
    let family_unit_mismatch_reproduced = burned_extractor_family_units == 2
        && sealed_record_family_units == 1
        && divergence_is_stage1;
    if !family_unit_mismatch_reproduced {
        return Err(Schema2CertificateError::SourceShape {
            name: SOURCE.to_owned(),
            reason: "stage-1 family-unit divergence did not replay".to_owned(),
        });
    }
    Ok(Stage1UniverseDiagnostic {
        stage: 1,
        candidate_hash,
        clause_forms: vec!["Univ".to_owned(), "App(Univ,Var(1))".to_owned()],
        burned_extractor_family_units,
        sealed_record_family_units,
        family_unit_mismatch_reproduced,
        diagnosis: "the burned clause-family extractor treated the nullary Univ formation and dependent App(Univ,m) formation as two marginal families; the sealed trace records one family unit, but the frozen record does not say whether the package is an arena-plus-family, a formation/completion pair, or two families".to_owned(),
        frozen_record_decides_package_boundary: false,
        count_used_to_choose_package_boundary: false,
        adjudication_id: STAGE1_UNIVERSE_ADJUDICATION.to_owned(),
    })
}

fn validate_trunc_archive_shape() -> Result<(), Schema2CertificateError> {
    const SOURCE: &str = "trunc_endpoint_realizer_v1";
    let root = parse_json_source(SOURCE, TRUNC_ENDPOINT_BYTES)?;
    let schema = field(&root, "schema", SOURCE)?.as_str().unwrap_or_default();
    if !schema.contains("trunc") {
        return Err(Schema2CertificateError::SourceShape {
            name: SOURCE.to_owned(),
            reason: format!("unexpected schema {schema:?}"),
        });
    }
    Ok(())
}

fn adjudications() -> Vec<AdjudicationRequired> {
    vec![
        AdjudicationRequired {
            id: STAGE1_UNIVERSE_ADJUDICATION.to_owned(),
            phase: "E-2".to_owned(),
            constructor_scope: "ordinary formation at the stage-1 Univ/App(Univ,m) package"
                .to_owned(),
            issue: "the frozen record does not define the natural-family boundary inside the two-clause Universe package".to_owned(),
            frozen_record_decides: false,
            options: vec![
                AdjudicationOption {
                    option_id: "A".to_owned(),
                    interpretation: "only App(Univ,m) is the exported family; Univ is arena/formation infrastructure and is excluded as a separate family".to_owned(),
                },
                AdjudicationOption {
                    option_id: "B".to_owned(),
                    interpretation: "Univ and App(Univ,m) form one formation/completion natural-family package".to_owned(),
                },
                AdjudicationOption {
                    option_id: "C".to_owned(),
                    interpretation: "Univ and App(Univ,m) are two distinct natural families".to_owned(),
                },
            ],
            unresolved_questions: vec![
                "which operational package-boundary rule is adopted independently of the recorded family-unit count?".to_owned(),
            ],
            selected_option: None,
            typed_realizer_token: None,
            status: "adjudication_required".to_owned(),
        },
        AdjudicationRequired {
            id: STEP8_POST_PATH_ADJUDICATION.to_owned(),
            phase: "E-2".to_owned(),
            constructor_scope: "Step-8 post-path operation, coherence, and cell-action rows"
                .to_owned(),
            issue: "the archive names three post-path rows but does not supply their typed natural-family semantics or instance quotient".to_owned(),
            frozen_record_decides: false,
            options: Vec::new(),
            unresolved_questions: vec![
                "what is the typed natural-family signature for the clause-3 mu operation?".to_owned(),
                "what is the orientation and typed equality content of the clause-4 unit-coherence row?".to_owned(),
                "what typed constructor realizes the clause-3 by clause-2 cell action?".to_owned(),
                "which of these exported rows is an independent family and which is an instance of an existing family?".to_owned(),
            ],
            selected_option: None,
            typed_realizer_token: None,
            status: "adjudication_required".to_owned(),
        },
    ]
}

fn phase_statuses() -> Vec<PhaseStatus> {
    let mut phases = vec![
        PhaseStatus {
            phase: "E-1".to_owned(),
            status: "scoped_operational_fragment_complete_global_c1_open".to_owned(),
            theorem_token_issued: true,
            blocked_by: None,
        },
        PhaseStatus {
            phase: "E-2".to_owned(),
            status: "partial_adjudication_required".to_owned(),
            theorem_token_issued: false,
            blocked_by: None,
        },
    ];
    phases.extend((3_u32..=8).map(|number| PhaseStatus {
        phase: format!("E-{number}"),
        status: "not_started_blocked_by_e1_global_gap_and_e2_adjudication_strict_order".to_owned(),
        theorem_token_issued: false,
        blocked_by: Some("E-1,E-2".to_owned()),
    }));
    phases
}

pub fn build_schema2_certificate() -> Result<Schema2Certificate, Schema2CertificateError> {
    let source_bindings = source_bindings()?;
    validate_trunc_archive_shape()?;
    let e1 = derive_e1_evidence()?;
    let historical_projection = project_historical_regression()?;
    let stage1_universe_diagnostic = diagnose_stage1_universe()?;
    let adjudications = adjudications();
    let every_adjudication_unselected = adjudications
        .iter()
        .all(|entry| entry.selected_option.is_none());
    let every_blocked_constructor_has_no_typed_realizer_token = adjudications
        .iter()
        .all(|entry| entry.typed_realizer_token.is_none());
    if !every_adjudication_unselected || !every_blocked_constructor_has_no_typed_realizer_token {
        return Err(Schema2CertificateError::OrdinaryProjection(
            "an undecided constructor was silently selected or tokenized".to_owned(),
        ));
    }
    let mut certificate = Schema2Certificate {
        schema: SCHEMA2_CERTIFICATE_SCHEMA.to_owned(),
        date: SCHEMA2_CERTIFICATE_DATE.to_owned(),
        source_bindings,
        phases: phase_statuses(),
        e1,
        e2: E2Evidence {
            status: "partial_adjudication_required".to_owned(),
            preflight_only_while_global_e1_remains_open: true,
            historical_projection,
            stage1_universe_diagnostic,
            adjudications,
            every_adjudication_unselected,
            every_blocked_constructor_has_no_typed_realizer_token,
            grammar_complete: false,
            ordinary_family_realizers_complete: false,
            historical_ordinary_remainder_closed: false,
        },
        obstructions_retired: ObstructionStatus {
            c1_global_arbitrary_typed_instance_sort_preservation: false,
            c2_depth_two_schema_grammar_and_generator_completeness: false,
            c6_general_historical_term_level_completion: false,
            d4_cw_instance_grammar_orbit_quotient_membership: false,
            c8_candidate_boundary_provenance_join: false,
            domain_wide_c3_candidate_extraction_bridge: false,
        },
        guard_rails: GuardRailStatus {
            f1_executable: false,
            f1_triggered: false,
            f1_excluded: false,
            f5_executable: false,
            f5_triggered: false,
            f5_excluded: false,
            semantic_o16_empty: None,
        },
        conclusion_boundary: ConclusionBoundary {
            later_acceptance_computation_run: false,
            closing_inequality_computed: false,
            global_halt_proved: false,
            global_continuation_proved: false,
            permitted_conclusion: "the E-1 operational fragment replays, but global C1 remains open; the E-2 preflight also requires explicit adjudication before strict-order work may continue".to_owned(),
        },
        outcome: "partial_e1_scoped_fragment_e2_adjudication_required".to_owned(),
        remaining_obligations: vec![
            "prove an Agda-to-Rust issuer soundness bridge for every E-1 token and elaborate every relevant pen_core::Expr into the typed E-1 syntax before retiring global C1".to_owned(),
            STAGE1_UNIVERSE_ADJUDICATION.to_owned(),
            STEP8_POST_PATH_ADJUDICATION.to_owned(),
            "after adjudication, finish E-2 typed ordinary-family realizers and rerun the 5-8 regression as grammar outputs".to_owned(),
            "execute E-3 through E-8 in the prescribed strict order".to_owned(),
        ],
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> Schema2Replay {
    Schema2Replay {
        valid: false,
        e1_complete: false,
        e1_scoped_fragment_complete: false,
        e2_adjudication_required: false,
        unresolved_adjudication_count: 0,
        later_phases_blocked_by_strict_order: false,
        global_c1_retired: false,
        c2_retired: false,
        semantic_o16_decided: false,
        global_halt_proved: false,
        outcome: "replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against(
    certificate: &Schema2Certificate,
    expected: &Schema2Certificate,
) -> Schema2Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    Schema2Replay {
        valid: errors.is_empty(),
        e1_complete: certificate.e1.global_c1_retired,
        e1_scoped_fragment_complete: certificate.e1.scoped_e1_fragment_proved,
        e2_adjudication_required: certificate.e2.status == "partial_adjudication_required",
        unresolved_adjudication_count: u32::try_from(certificate.e2.adjudications.len())
            .expect("small adjudication inventory"),
        later_phases_blocked_by_strict_order: certificate.phases.iter().skip(2).all(|phase| {
            phase.status == "not_started_blocked_by_e1_global_gap_and_e2_adjudication_strict_order"
        }),
        global_c1_retired: certificate
            .obstructions_retired
            .c1_global_arbitrary_typed_instance_sort_preservation,
        c2_retired: certificate
            .obstructions_retired
            .c2_depth_two_schema_grammar_and_generator_completeness,
        semantic_o16_decided: certificate.guard_rails.semantic_o16_empty.is_some(),
        global_halt_proved: certificate.conclusion_boundary.global_halt_proved,
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_schema2_certificate(certificate: &Schema2Certificate) -> Schema2Replay {
    match build_schema2_certificate() {
        Ok(expected) => replay_against(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn schema2_json_pretty() -> Result<String, Schema2CertificateError> {
    serde_json::to_string_pretty(&build_schema2_certificate()?)
        .map(|json| format!("{json}\n"))
        .map_err(|error| Schema2CertificateError::Json(error.to_string()))
}

pub fn replay_schema2_json(json: &str) -> Schema2Replay {
    let raw: Value = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(error) => return failed_replay(format!("invalid JSON: {error}")),
    };
    let certificate: Schema2Certificate = match serde_json::from_str(json) {
        Ok(certificate) => certificate,
        Err(error) => return failed_replay(format!("certificate shape error: {error}")),
    };
    let typed = serde_json::to_value(&certificate).expect("certificate serializes");
    if raw != typed {
        return failed_replay("JSON contains unknown, duplicate, or ignored structure");
    }
    replay_schema2_certificate(&certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug, Eq, PartialEq)]
    enum ValuePathItem {
        Field(String),
        Index(usize),
    }

    fn redigest(certificate: &mut Schema2Certificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    fn assert_rejected(mut mutation: Schema2Certificate, expected: &Schema2Certificate) {
        redigest(&mut mutation);
        assert!(!replay_against(&mutation, expected).valid);
    }

    fn collect_paths(
        value: &Value,
        path: &mut Vec<ValuePathItem>,
        leaves: &mut Vec<Vec<ValuePathItem>>,
        arrays: &mut Vec<Vec<ValuePathItem>>,
    ) {
        match value {
            Value::Object(object) => {
                for (name, child) in object {
                    path.push(ValuePathItem::Field(name.clone()));
                    collect_paths(child, path, leaves, arrays);
                    path.pop();
                }
            }
            Value::Array(array) => {
                arrays.push(path.clone());
                for (index, child) in array.iter().enumerate() {
                    path.push(ValuePathItem::Index(index));
                    collect_paths(child, path, leaves, arrays);
                    path.pop();
                }
            }
            _ => leaves.push(path.clone()),
        }
    }

    fn value_at_path_mut<'a>(mut value: &'a mut Value, path: &[ValuePathItem]) -> &'a mut Value {
        for item in path {
            value = match item {
                ValuePathItem::Field(field) => value
                    .as_object_mut()
                    .and_then(|object| object.get_mut(field))
                    .expect("recorded object path"),
                ValuePathItem::Index(index) => value
                    .as_array_mut()
                    .and_then(|array| array.get_mut(*index))
                    .expect("recorded array path"),
            };
        }
        value
    }

    fn assert_redigested_value_rejected(value: Value, expected: &Schema2Certificate) {
        let Ok(mut mutation): Result<Schema2Certificate, _> = serde_json::from_value(value) else {
            return;
        };
        redigest(&mut mutation);
        assert!(!replay_against(&mutation, expected).valid);
    }

    #[test]
    fn source_pin_manifest_matches_embedded_bytes() {
        let mismatches = source_manifest()
            .into_iter()
            .filter_map(|(name, _, _, bytes, length, digest, _)| {
                let actual_length = bytes.len() as u64;
                let actual_digest = bytes_hash(bytes);
                (actual_length != length || actual_digest != digest)
                    .then_some(format!("{name}: {actual_length}, {actual_digest}"))
            })
            .collect::<Vec<_>>();
        assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
    }

    #[test]
    fn partial_certificate_is_fail_closed_at_exact_e2_adjudications() {
        let certificate = build_schema2_certificate().expect("certificate");
        assert_eq!(certificate.phases.len(), 8);
        assert_eq!(
            certificate.phases[0].status,
            "scoped_operational_fragment_complete_global_c1_open"
        );
        assert_eq!(
            certificate.phases[1].status,
            "partial_adjudication_required"
        );
        assert!(certificate.e1.scoped_e1_fragment_proved);
        assert!(!certificate.e1.global_c1_retired);
        assert!(!certificate.e1.agda_to_rust_issuer_soundness_proved);
        assert!(
            !certificate
                .e1
                .all_pen_core_expr_to_typed_e1_elaboration_proved
        );
        assert!(certificate.e1.rust_graders_replayed);
        assert!(certificate.e1.substitution_support_replayed);
        assert!(certificate.e1.identity_support_exact);
        assert!(certificate.e1.genuine_target_support_within_image_bound);
        assert_eq!(certificate.e1.trunc_endpoint_maps.len(), 4);
        assert_eq!(certificate.e2.adjudications.len(), 2);
        assert!(certificate.e2.preflight_only_while_global_e1_remains_open);
        assert!(certificate.e2.every_adjudication_unselected);
        assert!(certificate.e2.adjudications.iter().all(|entry| {
            entry.selected_option.is_none() && entry.typed_realizer_token.is_none()
        }));
        assert_eq!(
            certificate
                .e2
                .historical_projection
                .ordinary_unresolved_counts_derived_from_rows,
            [5, 6, 5, 8]
        );
        assert!(
            certificate
                .e2
                .historical_projection
                .ordinary_steps
                .iter()
                .all(|step| step.path_constructor_intro_preserved)
        );
        assert!(
            !certificate
                .e2
                .historical_projection
                .count_used_as_constructor_input
        );
        assert!(!certificate.e2.grammar_complete);
        assert!(
            !certificate
                .obstructions_retired
                .c1_global_arbitrary_typed_instance_sort_preservation
        );
        assert!(
            !certificate
                .obstructions_retired
                .c2_depth_two_schema_grammar_and_generator_completeness
        );
        assert_eq!(certificate.guard_rails.semantic_o16_empty, None);
        assert!(!certificate.guard_rails.f1_executable);
        assert!(!certificate.guard_rails.f1_excluded);
        assert!(!certificate.guard_rails.f5_executable);
        assert!(!certificate.guard_rails.f5_excluded);
        assert!(
            !certificate
                .conclusion_boundary
                .later_acceptance_computation_run
        );
        assert!(!certificate.conclusion_boundary.global_halt_proved);
    }

    #[test]
    fn strict_json_replay_rejects_unknown_and_duplicate_fields() {
        let json = schema2_json_pretty().expect("json");
        assert!(replay_schema2_json(&json).valid);
        let unknown = json.replacen("{\n", "{\n  \"unknown\": true,\n", 1);
        assert!(!replay_schema2_json(&unknown).valid);
        let duplicate = json.replacen(
            "\"schema\": \"schema2-phase-batch-v1\",",
            "\"schema\": \"schema2-phase-batch-v1\",\n  \"schema\": \"schema2-phase-batch-v1\",",
            1,
        );
        assert!(!replay_schema2_json(&duplicate).valid);
        let unknown_deep = json.replacen(
            "\"guard_rails\": {",
            "\"guard_rails\": {\n    \"unknown\": true,",
            1,
        );
        assert!(!replay_schema2_json(&unknown_deep).valid);
    }

    #[test]
    fn selections_promotions_tokens_path_deletion_sources_and_maps_fail_replay() {
        let expected = build_schema2_certificate().expect("certificate");

        let mut selection = expected.clone();
        selection.e2.adjudications[0].selected_option = Some("A".to_owned());
        assert_rejected(selection, &expected);

        let mut invented_step8_option = expected.clone();
        invented_step8_option.e2.adjudications[1]
            .options
            .push(AdjudicationOption {
                option_id: "fabricated".to_owned(),
                interpretation: "not present in the frozen record".to_owned(),
            });
        assert_rejected(invented_step8_option, &expected);

        let mut typed_token = expected.clone();
        typed_token.e2.historical_projection.ordinary_steps[0].rows[0].typed_realizer_token =
            Some("fabricated-token".to_owned());
        typed_token.e2.historical_projection.ordinary_steps[0].rows[0].typed_realizer_issued = true;
        assert_rejected(typed_token, &expected);

        let mut path_deletion = expected.clone();
        let rows = &mut path_deletion.e2.historical_projection.ordinary_steps[0].rows;
        let index = rows
            .iter()
            .position(|row| row.semantic_kind == OrdinaryFamilyKind::PathConstructorIntro)
            .expect("path row");
        rows.remove(index);
        path_deletion.e2.historical_projection.ordinary_steps[0].derived_row_count -= 1;
        path_deletion.e2.historical_projection.ordinary_steps[0].path_constructor_intro_preserved =
            false;
        assert_rejected(path_deletion, &expected);

        let mut source_change = expected.clone();
        source_change.source_bindings[0].blake3.push_str("-mutated");
        assert_rejected(source_change, &expected);

        let mut e1_change = expected.clone();
        e1_change.e1.scoped_e1_fragment_proved = false;
        assert_rejected(e1_change, &expected);

        let mut map_change = expected.clone();
        map_change.e1.trunc_endpoint_maps.swap(0, 1);
        assert_rejected(map_change, &expected);

        let mut c1_promotion = expected.clone();
        c1_promotion.e1.global_c1_retired = true;
        c1_promotion
            .obstructions_retired
            .c1_global_arbitrary_typed_instance_sort_preservation = true;
        assert_rejected(c1_promotion, &expected);

        let mut c2_promotion = expected.clone();
        c2_promotion
            .obstructions_retired
            .c2_depth_two_schema_grammar_and_generator_completeness = true;
        assert_rejected(c2_promotion, &expected);

        let mut f1_exclusion = expected.clone();
        f1_exclusion.guard_rails.f1_excluded = true;
        assert_rejected(f1_exclusion, &expected);

        let mut semantic_o16 = expected.clone();
        semantic_o16.guard_rails.semantic_o16_empty = Some(true);
        assert_rejected(semantic_o16, &expected);

        let mut halt = expected.clone();
        halt.conclusion_boundary.global_halt_proved = true;
        assert_rejected(halt, &expected);

        let mut later_phase = expected.clone();
        later_phase.phases[2].status = "complete".to_owned();
        later_phase.phases[2].theorem_token_issued = true;
        later_phase.phases[2].blocked_by = None;
        assert_rejected(later_phase, &expected);
    }

    #[test]
    fn every_scalar_and_nonempty_array_mutation_fails_after_redigest() {
        let expected = build_schema2_certificate().expect("certificate");
        let original = serde_json::to_value(&expected).expect("certificate projects");
        let mut leaves = Vec::new();
        let mut arrays = Vec::new();
        collect_paths(&original, &mut Vec::new(), &mut leaves, &mut arrays);
        let digest_path = [ValuePathItem::Field("result_digest".to_owned())];
        for path in leaves {
            if path == digest_path {
                continue;
            }
            let mut mutation = original.clone();
            match value_at_path_mut(&mut mutation, &path) {
                Value::Bool(value) => *value = !*value,
                Value::Number(value) => {
                    let number = value.as_u64().expect("unsigned certificate number");
                    *value_at_path_mut(&mut mutation, &path) = Value::from(number + 1);
                }
                Value::String(value) => value.push_str(":mutated"),
                value @ Value::Null => *value = Value::Bool(true),
                Value::Array(_) | Value::Object(_) => unreachable!(),
            }
            assert_redigested_value_rejected(mutation, &expected);
        }
        for path in arrays {
            let items = value_at_path_mut(&mut original.clone(), &path)
                .as_array()
                .expect("array")
                .clone();
            if items.is_empty() {
                continue;
            }
            let mut deletion = original.clone();
            value_at_path_mut(&mut deletion, &path)
                .as_array_mut()
                .expect("array")
                .remove(0);
            assert_redigested_value_rejected(deletion, &expected);

            let mut duplication = original.clone();
            value_at_path_mut(&mut duplication, &path)
                .as_array_mut()
                .expect("array")
                .push(items[0].clone());
            assert_redigested_value_rejected(duplication, &expected);

            if let Some(other) = items.iter().position(|item| item != &items[0]) {
                let mut reorder = original.clone();
                value_at_path_mut(&mut reorder, &path)
                    .as_array_mut()
                    .expect("array")
                    .swap(0, other);
                assert_redigested_value_rejected(reorder, &expected);
            }
        }
    }
}
