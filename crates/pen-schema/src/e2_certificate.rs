//! Create-new, fail-closed successor certificate for the resumed E-2 phase.
//!
//! The user's R1/R2 adjudications are authoritative policy inputs.  They do
//! not, by themselves, decide the generator-membership questions that those
//! rules delegate to E-4.  This certificate therefore distinguishes
//! "adopted" from "operationally decided", replays the registered Step-8
//! signatures, and stops at F-Q4 without issuing historical handoff credit.

use crate::certificate::replay_schema2_json;
use crate::context::{BinderId, Declaration, TermExpr, TypeExpr, form_schema_context};
use crate::grammar::{
    ARBITRARY_SEALED_SUPPORT_WINDOW_GAP, ClauseAnchor, DerivationRef, E4_GENERATOR_MEMBERSHIP_GAP,
    FamilyPresentation, ORDINARY_CONSTRUCTOR_REGISTRY, ORDINARY_SCHEMA2_GRAMMAR_VERSION,
    OrdinaryInterpretation, OrdinarySchemaKind, SupportWindow, UnitOrientation,
    form_ordinary_schema, ordinary_constructor_registry_digest,
    replay_ordinary_constructor_registry,
};
use crate::ordinary::{
    ORDINARY_REALIZER_TOKEN_VERSION, issue_ordinary_typed_realizer, replay_ordinary_typed_realizer,
    requires_e4_generator_membership,
};
use crate::stage1_r1::{issue_stage1_r1_package_token, replay_stage1_r1_package_token};
use crate::step8_r2::{
    issue_step8_r2_typed_signatures_token, replay_step8_r2_typed_signatures_token,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const SCHEMA2_E2_CERTIFICATE_SCHEMA: &str = "schema2-e2-successor-v2";
pub const SCHEMA2_E2_CERTIFICATE_DATE: &str = "2026-07-19";
pub const R1_RULE_ID: &str = "formation-completion-package-family-rule-v1";
pub const R2_RULE_ID: &str = "derived-action-generator-membership-rule-v1";
pub const FQ4_OBSTRUCTION_ID: &str = "F-Q4_E4_GENERATOR_MEMBERSHIP_UNDECIDABLE_IN_CURRENT_FRAGMENT";

const PREDECESSOR_BYTES: &[u8] = include_bytes!("../../../docs/schema2_v1.json");
const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/e2_quotient_adjudications.md");
const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_e_schema2_plan.md");
const HIST_CERT_BYTES: &[u8] = include_bytes!("../../../docs/hist_cert_v3.json");
const STAGE1_DIAGNOSTIC_BYTES: &[u8] = include_bytes!("../../../docs/semantic_reselection.json");
const HSPACE_BYTES: &[u8] = include_bytes!("../../../docs/HSPACE_ENUMERATION.md");
const NATURALITY_BASIS_BYTES: &[u8] = include_bytes!("../../pen-eval/src/naturality_basis.rs");
const SEMANTIC_PROVENANCE_BYTES: &[u8] =
    include_bytes!("../../pen-eval/src/semantic_provenance.rs");
const TYPED_BOUNDARY_BYTES: &[u8] = include_bytes!("../../pen-type/src/cubical/typed_boundary.rs");
const PREDECESSOR_RESULT_BYTES: &[u8] = include_bytes!("../../../docs/SCHEMA2_RESULT.md");
const GRAMMAR_SOURCE_BYTES: &[u8] = include_bytes!("grammar.rs");
const ORDINARY_SOURCE_BYTES: &[u8] = include_bytes!("ordinary.rs");
const STAGE1_R1_SOURCE_BYTES: &[u8] = include_bytes!("stage1_r1.rs");
const STEP8_R2_SOURCE_BYTES: &[u8] = include_bytes!("step8_r2.rs");
const SCHEMA2_E2_AGDA_BYTES: &[u8] = include_bytes!("../../../agda/Schema2E2.agda");
const CONTEXT_SOURCE_BYTES: &[u8] = include_bytes!("context.rs");
const TELESCOPE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/telescope.rs");
const ELABORATE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Schema2E2CertificateError {
    #[error("source artifact {name} failed exact binding: {reason}")]
    SourceBinding { name: String, reason: String },
    #[error("source artifact {name} has an unexpected shape: {reason}")]
    SourceShape { name: String, reason: String },
    #[error("ordinary E-2 replay failed: {0}")]
    Ordinary(String),
    #[error("certificate JSON failed: {0}")]
    Json(String),
    #[error("create-new artifact I/O failed: {0}")]
    Io(String),
    #[error("new E-2 certificate did not replay: {0}")]
    EmittedReplay(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E2SourceBinding {
    pub name: String,
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
    pub exact_bytes_pinned: bool,
    pub semantic_projection_replayed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E2PhaseStatus {
    pub phase: String,
    pub status: String,
    pub theorem_token_issued: bool,
    pub blocked_by: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PredecessorEvidence {
    pub schema: String,
    pub result_digest: String,
    pub exact_certificate_replay_valid: bool,
    pub scoped_e1_fragment_complete: bool,
    pub global_c1_retired: bool,
    pub predecessor_e2_status: String,
    pub successor_does_not_overwrite_predecessor: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AdjudicationState {
    AdoptedPolicy,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalDecisionState {
    PendingGeneratorMembership,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RuleAdoptionEvidence {
    pub rule_id: String,
    pub adjudication_state: AdjudicationState,
    pub adoption_text_exactly_pinned: bool,
    pub count_blind_decision_procedure_required: bool,
    pub operational_decision_state: OperationalDecisionState,
    pub operational_membership_verdict_issued: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum E2R1CoverageDirection {
    CompletionCoversCarrier,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct R1CarrierShape {
    pub source_clause: u32,
    pub clause_role: String,
    pub constructor: String,
    pub semantic_role: String,
    pub independently_credited: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct R1CompletionShape {
    pub source_clause: u32,
    pub clause_role: String,
    pub constructor: String,
    pub head: String,
    pub level_variable: u32,
    pub semantic_judgement: String,
    pub canonical_package_representative: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct R1Stage1Evidence {
    pub rule_id: String,
    pub source_stage: u32,
    pub source_candidate_hash: String,
    pub carrier: R1CarrierShape,
    pub completion: R1CompletionShape,
    pub exact_syntactic_package_shape_replayed: bool,
    pub coverage_direction: E2R1CoverageDirection,
    pub actual_source_elaboration_hash: String,
    pub actual_source_signature_digest: String,
    pub actual_package_derivation_hash: String,
    pub actual_stage1_typed_source_witness_issued: bool,
    pub actual_stage1_typed_source_gap: Option<String>,
    pub pending_local_role_exception_cases: Vec<String>,
    pub every_local_role_exception_case_pending: bool,
    pub completed_package_family_issued: bool,
    pub carrier_exception_membership_decided: bool,
    pub carrier_exception_gap: String,
    pub separate_carrier_family_issued: bool,
    pub historical_family_handoff_issued: bool,
    pub total_stage1_family_cardinality_omitted_pending_membership: bool,
    pub archived_count_or_bar_used_as_rule_input: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CarrierReference {
    pub code: String,
    pub element_type: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MuSignature {
    pub source_clause: u32,
    pub name: String,
    pub carrier: CarrierReference,
    pub domain: Vec<String>,
    pub codomain: String,
    pub registered_naturality_arguments: Vec<u32>,
    pub naturality_generator_proof_issued: bool,
    pub grammar_realizer_derivation_hash: String,
    pub typed_schema_replayed: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitOrientationEvidence {
    Left,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LeftUnitSignature {
    pub source_clause: u32,
    pub operation: String,
    pub left_term: String,
    pub right_term: String,
    pub equality: String,
    pub orientation: UnitOrientationEvidence,
    pub right_unit_disposition: String,
    pub grammar_realizer_derivation_hash: String,
    pub typed_schema_replayed: bool,
    pub generator_membership_decided: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CellActionSignature {
    pub operation_clause: u32,
    pub path_clause: u32,
    pub operation: String,
    pub cell_constructor: String,
    pub cell_dimension: u32,
    pub registered_bundle_kind: String,
    pub registered_typed_boundary_derivation_hash: String,
    pub registered_boundary_bundle_derivation_hash: String,
    pub historical_package_child_derivation_hash: String,
    pub grammar_realizer_derivation_hash: String,
    pub typed_schema_replayed: bool,
    pub boundary_derived_by_map_cube: bool,
    pub generator_membership_decided: bool,
    pub independent_support_action_issued: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct R2Step8Evidence {
    pub rule_id: String,
    pub source_clause_shapes: Vec<String>,
    pub registered_interpretation_overlay_replayed: bool,
    pub raw_pen_core_to_schema2_term_elaboration_proved: bool,
    pub raw_elaboration_bridge_gap: String,
    pub exact_registered_signature_derivation_hash: String,
    pub mu: MuSignature,
    pub left_unit: LeftUnitSignature,
    pub cell_action: CellActionSignature,
    pub all_registered_signatures_replayed: bool,
    pub fq1_typed_signature_failure_triggered: bool,
    pub grammar_external_bundle_archive_join_proved: bool,
    pub certificate_exact_bundle_digest_join_replayed: bool,
    pub r2_membership_basis_available: bool,
    pub ordinary_handoff_token_issued: bool,
    pub archived_count_or_bar_used_as_rule_input: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OrdinaryGrammarEvidence {
    pub grammar_version: String,
    pub realizer_version: String,
    pub constructor_registry_digest: String,
    pub finite_registry_replayed: bool,
    pub every_registered_constructor_depth_at_most_two: bool,
    pub count_inputs_absent: bool,
    pub agda_safe_without_k_source_bound: bool,
    pub agda_generator_independence_theorem_declared: bool,
    pub agda_rust_operational_correspondence_proved: bool,
    pub global_c1_bridge_retired: bool,
    pub frozen_normal_forms_available: bool,
    pub generator_completeness_proved: bool,
    pub generic_quotient_judgements_complete: bool,
    pub generic_uniform_instance_membership_pending: bool,
    pub generic_r1_same_package_dependency_proved: bool,
    pub historical_reference_support_fragment_only: bool,
    pub arbitrary_sealed_support_window_grammar_proved: bool,
    pub arbitrary_support_window_gap: String,
    pub historical_ordinary_realizers_complete: bool,
    pub agent_a_handoff_token_issued: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E2HistoricalRegressionProjection {
    pub source_projection_replayed: bool,
    pub archived_raw_operational_totals: Vec<u32>,
    pub archived_registered_path_typed_and_marginal_subtotals: Vec<u32>,
    pub archived_ordinary_unresolved_totals: Vec<u32>,
    pub count_used_as_constructor_or_membership_input: bool,
    pub revised_e2_output_regression_run: bool,
    pub fq2_evaluated: bool,
    pub ordinary_historical_tokens_complete: bool,
    pub agent_a_handoff_token_issued: bool,
    pub status: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Fq4Evidence {
    pub falsifier_id: String,
    pub triggered: bool,
    pub required_decider: String,
    pub available_basis_version: String,
    pub available_basis_scope: Vec<String>,
    pub missing_basis_capabilities: Vec<String>,
    pub affected_rule_applications: Vec<String>,
    pub affected_rows_remain_pending: bool,
    pub no_independence_verdict_inferred_from_labels_or_positions: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FalsifierStatus {
    pub fq1_triggered: bool,
    pub fq2_evaluated: bool,
    pub fq2_divergence_recorded: bool,
    pub fq3_triggered: bool,
    pub fq4: Fq4Evidence,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E2ObstructionStatus {
    pub global_c1_arbitrary_typed_instance_sort_preservation_retired: bool,
    pub c2_depth_two_schema_grammar_and_generator_completeness_retired: bool,
    pub e3_frozen_normalization_and_equality_complete: bool,
    pub e4_generator_basis_complete: bool,
    pub e5_demand_projection_complete: bool,
    pub c6_general_historical_term_level_completion_retired: bool,
    pub c8_candidate_boundary_provenance_join_retired: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E2ConclusionBoundary {
    pub historical_scores_recomputed: bool,
    pub step16_acceptance_computation_run: bool,
    pub closing_inequality_computed: bool,
    pub semantic_o16_decided: bool,
    pub global_halt_proved: bool,
    pub global_continuation_proved: bool,
    pub permitted_conclusion: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schema2E2Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<E2SourceBinding>,
    pub predecessor: PredecessorEvidence,
    pub adjudications: Vec<RuleAdoptionEvidence>,
    pub r1_stage1: R1Stage1Evidence,
    pub r2_step8: R2Step8Evidence,
    pub ordinary_grammar: OrdinaryGrammarEvidence,
    pub historical_regression: E2HistoricalRegressionProjection,
    pub falsifiers: FalsifierStatus,
    pub phases: Vec<E2PhaseStatus>,
    pub obstructions_retired: E2ObstructionStatus,
    pub conclusion_boundary: E2ConclusionBoundary,
    pub e2_complete: bool,
    pub outcome: String,
    pub remaining_obligations: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Schema2E2Replay {
    pub valid: bool,
    pub adjudications_adopted: bool,
    pub step8_signatures_replayed: bool,
    pub fq4_triggered: bool,
    pub e2_complete: bool,
    pub later_phases_unexecuted: bool,
    pub global_c1_retired: bool,
    pub c2_retired: bool,
    pub handoff_token_issued: bool,
    pub global_halt_proved: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Copy)]
struct SourceSpec {
    name: &'static str,
    path: &'static str,
    role: &'static str,
    bytes: &'static [u8],
    expected_length: u64,
    expected_blake3: &'static str,
    semantic_projection_replayed: bool,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(SCHEMA2_E2_CERTIFICATE_SCHEMA, domain, value))
        .expect("E-2 certificate proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3::hash(bytes).to_hex())
}

fn source_manifest() -> Vec<SourceSpec> {
    vec![
        SourceSpec {
            name: "schema2_v1",
            path: "docs/schema2_v1.json",
            role: "exact_predecessor_certificate",
            bytes: PREDECESSOR_BYTES,
            expected_length: 27_265,
            expected_blake3: "blake3:7ef97df03d67c0f10b79cbd920aa9d0ab01642a9271eb826ee20123328cd4e23",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "e2_quotient_adjudications",
            path: "docs/e2_quotient_adjudications.md",
            role: "adopted_r1_r2_authority",
            bytes: ADJUDICATION_BYTES,
            expected_length: 6_355,
            expected_blake3: "blake3:6078f8e1f7209a7ba1fe082e44ae4dc491ea5e142e806f1c1b518c221d96a45d",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "agent_e_schema2_plan",
            path: "docs/agent_e_schema2_plan.md",
            role: "governing_strict_order_plan",
            bytes: PLAN_BYTES,
            expected_length: 9_562,
            expected_blake3: "blake3:d06d2039f1cd83e2d5509478aede30d2e09e2548f262cea9e0f88abbfe2767db",
            semantic_projection_replayed: false,
        },
        SourceSpec {
            name: "hist_cert_v3",
            path: "docs/hist_cert_v3.json",
            role: "registered_step8_constant_boundary_bundle",
            bytes: HIST_CERT_BYTES,
            expected_length: 178_331,
            expected_blake3: "blake3:dee9ceed570935917a7e40130caffdb014af412080b7dd9592c2fe8a59c9b05d",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "semantic_reselection_v1",
            path: "docs/semantic_reselection.json",
            role: "burned_stage1_source_shape_diagnostic_only",
            bytes: STAGE1_DIAGNOSTIC_BYTES,
            expected_length: 4_107,
            expected_blake3: "blake3:c3dc9a3060abb68985ace5ed1cf1b465f074f86c74a41506ab991f08460ba6e7",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "hspace_enumeration",
            path: "docs/HSPACE_ENUMERATION.md",
            role: "registered_step8_interpretation_input_not_membership_proof",
            bytes: HSPACE_BYTES,
            expected_length: 9_699,
            expected_blake3: "blake3:97954d23e8e8d754bf6d9cea87034ea4017bd9dcae88d61728be91e2e82c4308",
            semantic_projection_replayed: false,
        },
        SourceSpec {
            name: "raw_naturality_basis",
            path: "crates/pen-eval/src/naturality_basis.rs",
            role: "available_basis_scope_and_explicit_c2_gap",
            bytes: NATURALITY_BASIS_BYTES,
            expected_length: 21_554,
            expected_blake3: "blake3:6eaf0afb85bcae4be28f6b023b6912354748abd24760203d9b4881b3f7c16ea5",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "semantic_provenance_local_roles",
            path: "crates/pen-eval/src/semantic_provenance.rs",
            role: "frozen_four_case_local_role_inventory",
            bytes: SEMANTIC_PROVENANCE_BYTES,
            expected_length: 78_867,
            expected_blake3: "blake3:57cba7d721f42d3dfe1f8bacec805e9125db8204c0f88e975da14555a050b479",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "typed_boundary_source",
            path: "crates/pen-type/src/cubical/typed_boundary.rs",
            role: "typed_step8_constant_boundary_replay_source",
            bytes: TYPED_BOUNDARY_BYTES,
            expected_length: 199_287,
            expected_blake3: "blake3:0d3f746ddb3a3c28441980f3ab9733762a94fe46f380be5d5a174659c07e8274",
            semantic_projection_replayed: false,
        },
        SourceSpec {
            name: "schema2_v1_result",
            path: "docs/SCHEMA2_RESULT.md",
            role: "predecessor_scope_and_conclusion_boundary",
            bytes: PREDECESSOR_RESULT_BYTES,
            expected_length: 7_648,
            expected_blake3: "blake3:a5b9b2140f691756362cfbfdbb20a2285e9828912e92644544e46d16830293e2",
            semantic_projection_replayed: false,
        },
        SourceSpec {
            name: "schema2_context_source",
            path: "crates/pen-schema/src/context.rs",
            role: "e1_formation_and_expression_typing_rules_used_by_e2",
            bytes: CONTEXT_SOURCE_BYTES,
            expected_length: 71_070,
            expected_blake3: "blake3:d0de2a49de234034ec60e62e7735658055bc4e9e3abedd5790c7d471de5f1904",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "reference_telescope_source",
            path: "crates/pen-core/src/telescope.rs",
            role: "actual_stage1_and_step8_reference_telescopes",
            bytes: TELESCOPE_SOURCE_BYTES,
            expected_length: 19_443,
            expected_blake3: "blake3:d9831c28c380d1fdc313f757b393e26e81df1505d87ae35291d3eb8f214cf545",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "kernel_elaborator_source",
            path: "crates/pen-type/src/elaborate.rs",
            role: "stage1_kernel_elaboration_and_signature_digest",
            bytes: ELABORATE_SOURCE_BYTES,
            expected_length: 100_581,
            expected_blake3: "blake3:cc1027fa5e53ac1417b2bf01685bbd463cf832fe36c1b37339c360b7fc75c648",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "schema2_ordinary_grammar_source",
            path: "crates/pen-schema/src/grammar.rs",
            role: "operational_e2_grammar",
            bytes: GRAMMAR_SOURCE_BYTES,
            expected_length: 69_162,
            expected_blake3: "blake3:c86b9620c3d07cdf1512d33db1114eb108d68969c4868acf40cdbc5d67d04c8c",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "schema2_ordinary_realizer_source",
            path: "crates/pen-schema/src/ordinary.rs",
            role: "operational_e2_realizer_issuer",
            bytes: ORDINARY_SOURCE_BYTES,
            expected_length: 26_148,
            expected_blake3: "blake3:54bf0b295a86e349f0bdaf8cd0a27aaa7a5d094e0f6fcbcab7b4aedcd6f8f154",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "stage1_r1_source",
            path: "crates/pen-schema/src/stage1_r1.rs",
            role: "actual_stage1_univ_app_kernel_replay",
            bytes: STAGE1_R1_SOURCE_BYTES,
            expected_length: 9_627,
            expected_blake3: "blake3:677875dd0ecf02a075639f51fabbbac0367cd05d8deeb464601ee2ba97893f86",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "step8_r2_source",
            path: "crates/pen-schema/src/step8_r2.rs",
            role: "exact_step8_s3_bundle_and_signature_replay",
            bytes: STEP8_R2_SOURCE_BYTES,
            expected_length: 11_143,
            expected_blake3: "blake3:834be06764a5259269391454feed6ed486685f1c15cb2e9c874866407f277fbe",
            semantic_projection_replayed: true,
        },
        SourceSpec {
            name: "schema2_e2_agda",
            path: "agda/Schema2E2.agda",
            role: "safe_without_k_e2_boundary_model",
            bytes: SCHEMA2_E2_AGDA_BYTES,
            expected_length: 4_824,
            expected_blake3: "blake3:125232bed90c447feab3fcb15a83772d567e3b28855c4771c3a017e85d99ba59",
            semantic_projection_replayed: true,
        },
    ]
}

fn bind_sources() -> Result<Vec<E2SourceBinding>, Schema2E2CertificateError> {
    source_manifest()
        .into_iter()
        .map(|spec| {
            let byte_length = spec.bytes.len() as u64;
            let blake3 = bytes_hash(spec.bytes);
            let expected_blake3 = spec.expected_blake3;
            if byte_length != spec.expected_length || blake3 != expected_blake3 {
                return Err(Schema2E2CertificateError::SourceBinding {
                    name: spec.name.to_owned(),
                    reason: format!(
                        "expected {}/{expected_blake3}, found {byte_length}/{blake3}",
                        spec.expected_length
                    ),
                });
            }
            Ok(E2SourceBinding {
                name: spec.name.to_owned(),
                path: spec.path.to_owned(),
                role: spec.role.to_owned(),
                byte_length,
                blake3,
                exact_bytes_pinned: true,
                semantic_projection_replayed: spec.semantic_projection_replayed,
            })
        })
        .collect()
}

fn parse_source_json(name: &str, bytes: &[u8]) -> Result<Value, Schema2E2CertificateError> {
    serde_json::from_slice(bytes).map_err(|error| Schema2E2CertificateError::SourceShape {
        name: name.to_owned(),
        reason: format!("invalid JSON: {error}"),
    })
}

fn shape_error(name: &str, reason: impl Into<String>) -> Schema2E2CertificateError {
    Schema2E2CertificateError::SourceShape {
        name: name.to_owned(),
        reason: reason.into(),
    }
}

fn expect_str<'a>(
    name: &str,
    value: &'a Value,
    field: &str,
) -> Result<&'a str, Schema2E2CertificateError> {
    value
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| shape_error(name, format!("missing string field {field}")))
}

fn validate_adoptions() -> Result<Vec<RuleAdoptionEvidence>, Schema2E2CertificateError> {
    let source = std::str::from_utf8(ADJUDICATION_BYTES)
        .map_err(|error| shape_error("e2_quotient_adjudications", error.to_string()))?;
    let required_fragments = [
        "Status:** **ADOPTED",
        "formation-completion-package-family-rule-v1",
        "derived-action-generator-membership-rule-v1",
        "R1 adopted",
        "R2 adopted",
        "Perfect. I hereby adopt R1 *and* R2.",
        "F-Q4",
    ];
    for fragment in required_fragments {
        if !source.contains(fragment) {
            return Err(shape_error(
                "e2_quotient_adjudications",
                format!("missing adopted-authority fragment {fragment:?}"),
            ));
        }
    }
    Ok([R1_RULE_ID, R2_RULE_ID]
        .into_iter()
        .map(|rule_id| RuleAdoptionEvidence {
            rule_id: rule_id.to_owned(),
            adjudication_state: AdjudicationState::AdoptedPolicy,
            adoption_text_exactly_pinned: true,
            count_blind_decision_procedure_required: true,
            operational_decision_state: OperationalDecisionState::PendingGeneratorMembership,
            operational_membership_verdict_issued: false,
        })
        .collect())
}

fn validate_predecessor() -> Result<PredecessorEvidence, Schema2E2CertificateError> {
    let json = std::str::from_utf8(PREDECESSOR_BYTES)
        .map_err(|error| shape_error("schema2_v1", error.to_string()))?;
    let replay = replay_schema2_json(json);
    if !replay.valid {
        return Err(shape_error(
            "schema2_v1",
            format!("predecessor replay failed: {:?}", replay.errors),
        ));
    }
    let value = parse_source_json("schema2_v1", PREDECESSOR_BYTES)?;
    let schema = expect_str("schema2_v1", &value, "schema")?.to_owned();
    let result_digest = expect_str("schema2_v1", &value, "result_digest")?.to_owned();
    let e1 = value
        .get("e1")
        .ok_or_else(|| shape_error("schema2_v1", "missing e1 evidence"))?;
    let e2 = value
        .get("e2")
        .ok_or_else(|| shape_error("schema2_v1", "missing e2 evidence"))?;
    let scoped_e1_fragment_complete = e1
        .get("scoped_e1_fragment_proved")
        .and_then(Value::as_bool)
        .ok_or_else(|| shape_error("schema2_v1", "missing scoped E1 status"))?;
    let global_c1_retired = e1
        .get("global_c1_retired")
        .and_then(Value::as_bool)
        .ok_or_else(|| shape_error("schema2_v1", "missing global C1 status"))?;
    let predecessor_e2_status = expect_str("schema2_v1.e2", e2, "status")?.to_owned();
    if schema != "schema2-phase-batch-v1"
        || !scoped_e1_fragment_complete
        || global_c1_retired
        || predecessor_e2_status != "partial_adjudication_required"
    {
        return Err(shape_error(
            "schema2_v1",
            "predecessor scope/status no longer matches the accepted v1 boundary",
        ));
    }
    Ok(PredecessorEvidence {
        schema,
        result_digest,
        exact_certificate_replay_valid: true,
        scoped_e1_fragment_complete,
        global_c1_retired,
        predecessor_e2_status,
        successor_does_not_overwrite_predecessor: true,
    })
}

#[derive(Clone, Debug)]
struct Stage1SourceProjection {
    candidate_hash: String,
}

fn validate_stage1_source_shape() -> Result<Stage1SourceProjection, Schema2E2CertificateError> {
    let value = parse_source_json("semantic_reselection_v1", STAGE1_DIAGNOSTIC_BYTES)?;
    if value.get("burned_run").and_then(Value::as_bool) != Some(true) {
        return Err(shape_error(
            "semantic_reselection_v1",
            "diagnostic is not marked burned",
        ));
    }
    let stage = value
        .get("stages")
        .and_then(Value::as_array)
        .and_then(|stages| {
            stages
                .iter()
                .find(|stage| stage.get("stage").and_then(Value::as_u64) == Some(1))
        })
        .ok_or_else(|| shape_error("semantic_reselection_v1", "missing Stage-1 record"))?;
    let winner = stage
        .get("winner")
        .ok_or_else(|| shape_error("semantic_reselection_v1", "missing Stage-1 winner"))?;
    let candidate_hash = expect_str("semantic_reselection_v1.stage1", winner, "candidate_hash")?;
    let clauses = winner
        .get("telescope")
        .and_then(|telescope| telescope.get("clauses"))
        .and_then(Value::as_array)
        .ok_or_else(|| shape_error("semantic_reselection_v1", "missing Stage-1 clauses"))?;
    let expected_app = serde_json::json!({"App": ["Univ", {"Var": 1}]});
    let exact_shape = clauses.len() == 2
        && clauses[0].get("role").and_then(Value::as_str) == Some("formation")
        && clauses[0].get("expr") == Some(&Value::String("Univ".to_owned()))
        && clauses[1].get("role").and_then(Value::as_str) == Some("formation")
        && clauses[1].get("expr") == Some(&expected_app);
    if !exact_shape {
        return Err(shape_error(
            "semantic_reselection_v1",
            "Stage-1 telescope is not [Univ, App(Univ, Var(1))]",
        ));
    }
    Ok(Stage1SourceProjection {
        candidate_hash: candidate_hash.to_owned(),
    })
}

#[derive(Clone, Debug)]
struct Step8BundleProjection {
    kind: String,
    typed_boundary_derivation_hash: String,
    bundle_derivation_hash: String,
    child_derivation_hash: String,
}

fn validate_step8_bundle() -> Result<Step8BundleProjection, Schema2E2CertificateError> {
    let value = parse_source_json("hist_cert_v3", HIST_CERT_BYTES)?;
    if expect_str("hist_cert_v3", &value, "schema")? != "hist-cert-historical-hit-v3" {
        return Err(shape_error("hist_cert_v3", "unexpected schema"));
    }
    let package = value
        .get("packages")
        .and_then(Value::as_array)
        .and_then(|packages| {
            packages
                .iter()
                .find(|package| package.get("step").and_then(Value::as_u64) == Some(8))
        })
        .ok_or_else(|| shape_error("hist_cert_v3", "missing Step-8 package"))?;
    let kind = expect_str("hist_cert_v3.step8", package, "bundle_kind")?.to_owned();
    let evidence_kind = expect_str("hist_cert_v3.step8", package, "bundle_evidence_kind")?;
    let typed_boundary_derivation_hash = expect_str(
        "hist_cert_v3.step8",
        package,
        "typed_boundary_derivation_hash",
    )?
    .to_owned();
    let bundle_derivation_hash =
        expect_str("hist_cert_v3.step8", package, "bundle_derivation_hash")?.to_owned();
    let child_derivation_hash =
        expect_str("hist_cert_v3.step8", package, "child_derivation_hash")?.to_owned();
    if kind != "s3"
        || evidence_kind != "archival_constant_bridge"
        || !typed_boundary_derivation_hash.starts_with("blake3:")
        || !bundle_derivation_hash.starts_with("blake3:")
        || !child_derivation_hash.starts_with("blake3:")
    {
        return Err(shape_error(
            "hist_cert_v3",
            "Step-8 package is not the registered typed S3 constant-boundary bundle",
        ));
    }
    Ok(Step8BundleProjection {
        kind,
        typed_boundary_derivation_hash,
        bundle_derivation_hash,
        child_derivation_hash,
    })
}

fn validate_available_basis_scope() -> Result<(), Schema2E2CertificateError> {
    let source = std::str::from_utf8(NATURALITY_BASIS_BYTES)
        .map_err(|error| shape_error("raw_naturality_basis", error.to_string()))?;
    for fragment in [
        "typed-family-sort-aware-renaming-basis-v1",
        "C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS",
        "sort-preserving",
        "parameter renamings",
        "weakening by one unused parameter",
        "identity_checked",
        "composition_closed",
    ] {
        if !source.contains(fragment) {
            return Err(shape_error(
                "raw_naturality_basis",
                format!("missing basis-scope fragment {fragment:?}"),
            ));
        }
    }
    let roles = std::str::from_utf8(SEMANTIC_PROVENANCE_BYTES)
        .map_err(|error| shape_error("semantic_provenance_local_roles", error.to_string()))?;
    for fragment in [
        "pub enum LocalRole",
        "KernelHead",
        "AdjointMate",
        "SupportAction",
        "Coherence",
    ] {
        if !roles.contains(fragment) {
            return Err(shape_error(
                "semantic_provenance_local_roles",
                format!("missing LocalRole fragment {fragment:?}"),
            ));
        }
    }
    Ok(())
}

fn u32_array_field(
    name: &str,
    value: &Value,
    field: &str,
) -> Result<Vec<u32>, Schema2E2CertificateError> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| shape_error(name, format!("missing array field {field}")))?
        .iter()
        .map(|entry| {
            entry
                .as_u64()
                .and_then(|number| u32::try_from(number).ok())
                .ok_or_else(|| shape_error(name, format!("non-u32 entry in {field}")))
        })
        .collect()
}

fn derive_historical_regression_projection()
-> Result<E2HistoricalRegressionProjection, Schema2E2CertificateError> {
    let value = parse_source_json("hist_cert_v3", HIST_CERT_BYTES)?;
    let regression = value
        .get("f_b2")
        .ok_or_else(|| shape_error("hist_cert_v3", "missing f_b2 projection"))?;
    let raw = u32_array_field("hist_cert_v3.f_b2", regression, "observed_raw_totals")?;
    let typed = u32_array_field(
        "hist_cert_v3.f_b2",
        regression,
        "observed_typed_key_subtotals",
    )?;
    let marginal = u32_array_field(
        "hist_cert_v3.f_b2",
        regression,
        "observed_marginal_subtotals",
    )?;
    let ordinary = u32_array_field(
        "hist_cert_v3.f_b2",
        regression,
        "observed_ordinary_unresolved",
    )?;
    let source_projection_replayed = regression
        .get("f_b2_regression_signature_replayed")
        .and_then(Value::as_bool)
        == Some(true)
        && regression
            .get("partial_but_regression_clean")
            .and_then(Value::as_bool)
            == Some(true)
        && typed == marginal
        && raw.len() == 4
        && typed.len() == 4
        && ordinary.len() == 4;
    if !source_projection_replayed {
        return Err(shape_error(
            "hist_cert_v3",
            "archived F-B2 projection no longer replays as four partial records",
        ));
    }
    Ok(E2HistoricalRegressionProjection {
        source_projection_replayed,
        archived_raw_operational_totals: raw,
        archived_registered_path_typed_and_marginal_subtotals: marginal,
        archived_ordinary_unresolved_totals: ordinary,
        count_used_as_constructor_or_membership_input: false,
        revised_e2_output_regression_run: false,
        fq2_evaluated: false,
        ordinary_historical_tokens_complete: false,
        agent_a_handoff_token_issued: false,
        status: "archived_projection_replayed_revised_e2_output_pending_fq4_membership".to_owned(),
    })
}

fn derive_r1_stage1(
    diagnostic: &Stage1SourceProjection,
) -> Result<R1Stage1Evidence, Schema2E2CertificateError> {
    let token = issue_stage1_r1_package_token()
        .map_err(|error| Schema2E2CertificateError::Ordinary(error.to_string()))?;
    replay_stage1_r1_package_token(&token)
        .map_err(|error| Schema2E2CertificateError::Ordinary(error.to_string()))?;
    let every_local_role_exception_case_pending = token.local_role_inventory.len() == 4
        && token.local_role_inventory.iter().all(|entry| {
            entry.decision == "pending_generator_membership"
                && entry.generator_membership_token.is_none()
                && entry.obstruction == E4_GENERATOR_MEMBERSHIP_GAP
        });
    let pending_local_role_exception_cases = token
        .local_role_inventory
        .iter()
        .map(|entry| {
            serde_json::to_value(entry.role)
                .expect("R1 local role serializes")
                .as_str()
                .expect("R1 local role serializes as a string")
                .to_owned()
        })
        .collect::<Vec<_>>();
    if token.rule != R1_RULE_ID
        || token.source_candidate_hash != diagnostic.candidate_hash
        || token.carrier_expression != "Univ"
        || token.completed_action_expression != "App(Univ,Var(1))"
        || token.canonical_family_expression != "App(Univ,Var(1))"
        || token.carrier_provenance_expression != "Univ"
        || !token.typed_package_shape_replayed
        || !token.same_package_dependency_replayed
        || !token.completed_action_covers_carrier_by_adopted_rule
        || !every_local_role_exception_case_pending
        || !token.completed_package_family_issued
        || token.exception_generator_membership_decided
        || token.total_stage1_family_count.is_some()
        || token.archived_count_used_as_input
        || token.acceptance_bar_used_as_input
    {
        return Err(Schema2E2CertificateError::Ordinary(
            "actual Stage-1 R1 token crossed its adopted fail-closed boundary".to_owned(),
        ));
    }
    Ok(R1Stage1Evidence {
        rule_id: R1_RULE_ID.to_owned(),
        source_stage: 1,
        source_candidate_hash: token.source_candidate_hash,
        carrier: R1CarrierShape {
            source_clause: u32::from(token.carrier_clause),
            clause_role: "formation".to_owned(),
            constructor: token.carrier_expression,
            semantic_role: "package_carrier_provenance".to_owned(),
            independently_credited: false,
        },
        completion: R1CompletionShape {
            source_clause: u32::from(token.completion_clause),
            clause_role: "formation_completion".to_owned(),
            constructor: "App".to_owned(),
            head: "Univ".to_owned(),
            level_variable: token.same_package_dependency_level,
            semantic_judgement: "App(Univ,Var(1)) : Type".to_owned(),
            canonical_package_representative: true,
        },
        exact_syntactic_package_shape_replayed: true,
        coverage_direction: E2R1CoverageDirection::CompletionCoversCarrier,
        actual_source_elaboration_hash: token.source_elaboration_hash,
        actual_source_signature_digest: token.source_signature_digest,
        actual_package_derivation_hash: token.derivation_hash,
        actual_stage1_typed_source_witness_issued: true,
        actual_stage1_typed_source_gap: None,
        pending_local_role_exception_cases,
        every_local_role_exception_case_pending,
        completed_package_family_issued: token.completed_package_family_issued,
        carrier_exception_membership_decided: false,
        carrier_exception_gap: E4_GENERATOR_MEMBERSHIP_GAP.to_owned(),
        separate_carrier_family_issued: false,
        historical_family_handoff_issued: false,
        total_stage1_family_cardinality_omitted_pending_membership: true,
        archived_count_or_bar_used_as_rule_input: false,
    })
}

fn step8_context() -> Result<crate::context::FormedSchemaContext, Schema2E2CertificateError> {
    form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(0),
            name: "S3".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(1),
            name: "base".to_owned(),
            ty: TypeExpr::parameter(0),
        },
        Declaration::OpaqueElement {
            binder: BinderId(2),
            name: "x".to_owned(),
            ty: TypeExpr::parameter(0),
        },
    ])
    .map_err(|error| Schema2E2CertificateError::Ordinary(error.to_string()))
}

fn ordinary_step8_realizer(
    context: crate::context::FormedSchemaContext,
    kind: OrdinarySchemaKind,
    interpretation: OrdinaryInterpretation,
    anchors: Vec<ClauseAnchor>,
) -> Result<crate::ordinary::OrdinaryTypedRealizerToken, Schema2E2CertificateError> {
    let schema = form_ordinary_schema(
        context,
        kind,
        interpretation,
        FamilyPresentation::CanonicalFamily,
        SupportWindow::new(7, 8)
            .map_err(|error| Schema2E2CertificateError::Ordinary(error.to_string()))?,
        anchors,
    )
    .map_err(|error| Schema2E2CertificateError::Ordinary(error.to_string()))?;
    let token = issue_ordinary_typed_realizer(schema)
        .map_err(|error| Schema2E2CertificateError::Ordinary(error.to_string()))?;
    replay_ordinary_typed_realizer(&token)
        .map_err(|error| Schema2E2CertificateError::Ordinary(error.to_string()))?;
    Ok(token)
}

fn derive_r2_step8(
    bundle: &Step8BundleProjection,
) -> Result<R2Step8Evidence, Schema2E2CertificateError> {
    let source = issue_step8_r2_typed_signatures_token()
        .map_err(|error| Schema2E2CertificateError::Ordinary(error.to_string()))?;
    replay_step8_r2_typed_signatures_token(&source)
        .map_err(|error| Schema2E2CertificateError::Ordinary(error.to_string()))?;
    let certificate_exact_bundle_digest_join_replayed = source
        .registered_boundary_archive_join_replayed
        && source.registered_boundary_kind == bundle.kind
        && source.registered_boundary_typing_derivation_hash
            == bundle.typed_boundary_derivation_hash
        && source.registered_boundary_bundle_derivation_hash == bundle.bundle_derivation_hash
        && source.registered_boundary_child_derivation_hash == bundle.child_derivation_hash;
    if source.rule != R2_RULE_ID
        || source.source_step != 8
        || !source.registered_interpretation_overlay_replayed
        || source.raw_pen_core_to_schema2_term_elaboration_proved
        || !certificate_exact_bundle_digest_join_replayed
        || !source.operation_signature_typed
        || !source.left_unit_signature_typed
        || !source.cell_action_term_typed
        || !source.cell_action_boundary_derived_by_map_cube
        || !source.first_slot_orientation
        || source.fq1_triggered
        || source.coherence_generator_membership_decided
        || source.cell_action_generator_membership_decided
        || source.independent_family_tokens_issued != 0
        || source.archived_count_used_as_input
        || source.acceptance_bar_used_as_input
    {
        return Err(Schema2E2CertificateError::Ordinary(
            "source-bound Step-8 R2 token crossed its adopted fail-closed boundary".to_owned(),
        ));
    }
    let context = step8_context()?;
    let carrier = TypeExpr::parameter(0);
    let operation = ordinary_step8_realizer(
        context.clone(),
        OrdinarySchemaKind::PostPathOperation,
        OrdinaryInterpretation::PostPathOperation {
            carrier: carrier.clone(),
            arity: 2,
        },
        vec![ClauseAnchor { step: 8, clause: 3 }],
    )?;
    let operation_ref = DerivationRef::parse(operation.derivation_hash().to_owned())
        .map_err(|error| Schema2E2CertificateError::Ordinary(error.to_string()))?;
    let coherence = ordinary_step8_realizer(
        context.clone(),
        OrdinarySchemaKind::PostPathCoherence,
        OrdinaryInterpretation::PostPathCoherence {
            carrier: carrier.clone(),
            operation: operation_ref.clone(),
            unit: TermExpr::variable(1),
            variable: TermExpr::variable(2),
            orientation: UnitOrientation::Left,
        },
        vec![ClauseAnchor { step: 8, clause: 4 }],
    )?;
    let cell = ordinary_step8_realizer(
        context,
        OrdinarySchemaKind::CellAction,
        OrdinaryInterpretation::CellAction {
            carrier,
            operation: operation_ref,
            cell_dimension: 3,
            registered_boundary_bundle: DerivationRef::parse(
                source.registered_boundary_bundle_derivation_hash.clone(),
            )
            .map_err(|error| Schema2E2CertificateError::Ordinary(error.to_string()))?,
        },
        vec![
            ClauseAnchor { step: 8, clause: 2 },
            ClauseAnchor { step: 8, clause: 3 },
        ],
    )?;
    if requires_e4_generator_membership(&operation)
        || !requires_e4_generator_membership(&coherence)
        || !requires_e4_generator_membership(&cell)
    {
        return Err(Schema2E2CertificateError::Ordinary(
            "R2 ordinary dispositions do not match the adopted membership boundary".to_owned(),
        ));
    }

    Ok(R2Step8Evidence {
        rule_id: R2_RULE_ID.to_owned(),
        source_clause_shapes: source.source_clause_shapes,
        registered_interpretation_overlay_replayed: true,
        raw_pen_core_to_schema2_term_elaboration_proved: false,
        raw_elaboration_bridge_gap:
            "C1_STEP8_POSITIONAL_PEN_CORE_TO_REGISTERED_SCHEMA2_TERM_ELABORATION".to_owned(),
        exact_registered_signature_derivation_hash: source.schema_signature_derivation_hash,
        mu: MuSignature {
            source_clause: 3,
            name: "mu".to_owned(),
            carrier: CarrierReference {
                code: "S3".to_owned(),
                element_type: "El(S3)".to_owned(),
            },
            domain: vec!["El(S3)".to_owned(), "El(S3)".to_owned()],
            codomain: "El(S3)".to_owned(),
            registered_naturality_arguments: vec![0, 1],
            naturality_generator_proof_issued: false,
            grammar_realizer_derivation_hash: operation.derivation_hash().to_owned(),
            typed_schema_replayed: true,
        },
        left_unit: LeftUnitSignature {
            source_clause: 4,
            operation: "mu".to_owned(),
            left_term: "mu(base,x)".to_owned(),
            right_term: "x".to_owned(),
            equality: "Path(El(S3),mu(base,x),x)".to_owned(),
            orientation: UnitOrientationEvidence::Left,
            right_unit_disposition: "d1_mate_not_second_family".to_owned(),
            grammar_realizer_derivation_hash: coherence.derivation_hash().to_owned(),
            typed_schema_replayed: true,
            generator_membership_decided: false,
        },
        cell_action: CellActionSignature {
            operation_clause: 3,
            path_clause: 2,
            operation: "mu".to_owned(),
            cell_constructor: "PathCon(3)".to_owned(),
            cell_dimension: 3,
            registered_bundle_kind: bundle.kind.clone(),
            registered_typed_boundary_derivation_hash: bundle
                .typed_boundary_derivation_hash
                .clone(),
            registered_boundary_bundle_derivation_hash: bundle.bundle_derivation_hash.clone(),
            historical_package_child_derivation_hash: bundle.child_derivation_hash.clone(),
            grammar_realizer_derivation_hash: cell.derivation_hash().to_owned(),
            typed_schema_replayed: true,
            boundary_derived_by_map_cube: true,
            generator_membership_decided: false,
            independent_support_action_issued: false,
        },
        all_registered_signatures_replayed: true,
        fq1_typed_signature_failure_triggered: false,
        grammar_external_bundle_archive_join_proved: false,
        certificate_exact_bundle_digest_join_replayed,
        r2_membership_basis_available: false,
        ordinary_handoff_token_issued: false,
        archived_count_or_bar_used_as_rule_input: false,
    })
}

fn validate_e2_agda_boundary() -> Result<(), Schema2E2CertificateError> {
    let source = std::str::from_utf8(SCHEMA2_E2_AGDA_BYTES)
        .map_err(|error| shape_error("schema2_e2_agda", error.to_string()))?;
    let safe = source.contains("{-# OPTIONS --safe --without-K #-}")
        && source.contains("step8-cell-action")
        && source.contains("Scope boundary")
        && !source.lines().any(|line| {
            let line = line.trim_start();
            line == "postulate" || line.starts_with("postulate ")
        });
    if !safe {
        return Err(shape_error(
            "schema2_e2_agda",
            "safe/without-K Step-8 boundary discipline failed",
        ));
    }
    Ok(())
}

fn certificate_digest(certificate: &Schema2E2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("successor-certificate", &projection)
}

pub fn build_schema2_e2_certificate() -> Result<Schema2E2Certificate, Schema2E2CertificateError> {
    let source_bindings = bind_sources()?;
    let predecessor = validate_predecessor()?;
    let adjudications = validate_adoptions()?;
    let stage1_source = validate_stage1_source_shape()?;
    let step8_bundle = validate_step8_bundle()?;
    validate_available_basis_scope()?;
    validate_e2_agda_boundary()?;

    let r1_stage1 = derive_r1_stage1(&stage1_source)?;
    let r2_step8 = derive_r2_step8(&step8_bundle)?;
    let historical_regression = derive_historical_regression_projection()?;
    let constructor_registry_digest = ordinary_constructor_registry_digest();
    let finite_registry_replayed =
        replay_ordinary_constructor_registry(&constructor_registry_digest);
    let every_registered_constructor_depth_at_most_two = ORDINARY_CONSTRUCTOR_REGISTRY
        .iter()
        .all(|descriptor| descriptor.depth.value() <= 2);
    if !finite_registry_replayed || !every_registered_constructor_depth_at_most_two {
        return Err(Schema2E2CertificateError::Ordinary(
            "ordinary constructor registry failed its finite depth-two replay".to_owned(),
        ));
    }

    let ordinary_grammar = OrdinaryGrammarEvidence {
        grammar_version: ORDINARY_SCHEMA2_GRAMMAR_VERSION.to_owned(),
        realizer_version: ORDINARY_REALIZER_TOKEN_VERSION.to_owned(),
        constructor_registry_digest,
        finite_registry_replayed,
        every_registered_constructor_depth_at_most_two,
        count_inputs_absent: true,
        agda_safe_without_k_source_bound: true,
        agda_generator_independence_theorem_declared: false,
        agda_rust_operational_correspondence_proved: false,
        global_c1_bridge_retired: false,
        frozen_normal_forms_available: false,
        generator_completeness_proved: false,
        generic_quotient_judgements_complete: false,
        generic_uniform_instance_membership_pending: true,
        generic_r1_same_package_dependency_proved: false,
        historical_reference_support_fragment_only: true,
        arbitrary_sealed_support_window_grammar_proved: false,
        arbitrary_support_window_gap: ARBITRARY_SEALED_SUPPORT_WINDOW_GAP.to_owned(),
        historical_ordinary_realizers_complete: false,
        agent_a_handoff_token_issued: false,
    };
    let fq4 = Fq4Evidence {
        falsifier_id: FQ4_OBSTRUCTION_ID.to_owned(),
        triggered: true,
        required_decider: "complete E-4 generator-membership procedure over frozen typed equality"
            .to_owned(),
        available_basis_version: "typed-family-sort-aware-renaming-basis-v1".to_owned(),
        available_basis_scope: vec![
            "sort_preserving_parameter_renamings".to_owned(),
            "identity".to_owned(),
            "composition".to_owned(),
            "weakening_by_one_unused_coarse_parameter".to_owned(),
        ],
        missing_basis_capabilities: vec![
            "E3_frozen_typed_normalization_and_univalent_equality".to_owned(),
            "typed_instance_substitution_generator_decomposition".to_owned(),
            "cubical_face_map_generator_decomposition".to_owned(),
            "parent_operation_naturality_closure_membership".to_owned(),
        ],
        affected_rule_applications: vec![
            "R1_clause_2_stage1_carrier_independent_local_role_exception".to_owned(),
            "R2_clause_1_step8_left_unit_coherence".to_owned(),
            "R2_clause_1_step8_mu_action_on_PathCon_3".to_owned(),
        ],
        affected_rows_remain_pending: true,
        no_independence_verdict_inferred_from_labels_or_positions: true,
    };
    let phases = vec![
        E2PhaseStatus {
            phase: "E-1".to_owned(),
            status: "predecessor_scoped_fragment_complete_global_c1_open".to_owned(),
            theorem_token_issued: false,
            blocked_by: Some(
                "C1_PEN_CORE_EXPR_TO_SCHEMA2_TYPED_EXPRESSION_BRIDGE_AND_AGDA_RUST_CORRESPONDENCE"
                    .to_owned(),
            ),
        },
        E2PhaseStatus {
            phase: "E-2".to_owned(),
            status: "partial_registered_signatures_replayed_fq4_membership_pending".to_owned(),
            theorem_token_issued: false,
            blocked_by: Some(FQ4_OBSTRUCTION_ID.to_owned()),
        },
    ];
    let mut phases = phases;
    for phase in 3..=8 {
        phases.push(E2PhaseStatus {
            phase: format!("E-{phase}"),
            status: "not_executed_strict_order".to_owned(),
            theorem_token_issued: false,
            blocked_by: Some(
                "global_C1_open_and_E2_FQ4_generator_membership_dependency".to_owned(),
            ),
        });
    }

    let mut certificate = Schema2E2Certificate {
        schema: SCHEMA2_E2_CERTIFICATE_SCHEMA.to_owned(),
        date: SCHEMA2_E2_CERTIFICATE_DATE.to_owned(),
        source_bindings,
        predecessor,
        adjudications,
        r1_stage1,
        r2_step8,
        ordinary_grammar,
        historical_regression,
        falsifiers: FalsifierStatus {
            fq1_triggered: false,
            fq2_evaluated: false,
            fq2_divergence_recorded: false,
            fq3_triggered: false,
            fq4,
        },
        phases,
        obstructions_retired: E2ObstructionStatus {
            global_c1_arbitrary_typed_instance_sort_preservation_retired: false,
            c2_depth_two_schema_grammar_and_generator_completeness_retired: false,
            e3_frozen_normalization_and_equality_complete: false,
            e4_generator_basis_complete: false,
            e5_demand_projection_complete: false,
            c6_general_historical_term_level_completion_retired: false,
            c8_candidate_boundary_provenance_join_retired: false,
        },
        conclusion_boundary: E2ConclusionBoundary {
            historical_scores_recomputed: false,
            step16_acceptance_computation_run: false,
            closing_inequality_computed: false,
            semantic_o16_decided: false,
            global_halt_proved: false,
            global_continuation_proved: false,
            permitted_conclusion: "R1's actual Stage-1 package and the adopted registered-overlay Step-8 signature checker replay count-blindly; F-Q1 is not triggered in that checker, while the raw pen_core overlay elaboration bridge remains open and F-Q4 leaves R1/R2 generator membership and every affected credit/handoff pending"
                .to_owned(),
        },
        e2_complete: false,
        outcome: "partial_e2_registered_signatures_replay_fq4_generator_membership_pending"
            .to_owned(),
        remaining_obligations: vec![
            "resolve the E-2/E-4 dependency without weakening strict-order or importing historical counts: supply frozen equality and a complete typed generator-membership decider"
                .to_owned(),
            "decide every R1 LocalRole exception case and the R2 coherence/cell-action membership cases with replayable E-4 tokens"
                .to_owned(),
            "only after those verdicts, issue individual historical ordinary-family and Agent A handoff tokens and run the count-as-output regression"
                .to_owned(),
            "retire the global C1 pen_core elaboration and Agda-to-Rust correspondence gaps"
                .to_owned(),
            "generalize the historical-reference anchor replay to arbitrary sealed two-step support windows before any C2 completeness claim"
                .to_owned(),
            "leave E-3 through E-8 unexecuted until their prerequisite boundary is lawfully reopened"
                .to_owned(),
        ],
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> Schema2E2Replay {
    Schema2E2Replay {
        valid: false,
        adjudications_adopted: false,
        step8_signatures_replayed: false,
        fq4_triggered: false,
        e2_complete: false,
        later_phases_unexecuted: false,
        global_c1_retired: false,
        c2_retired: false,
        handoff_token_issued: false,
        global_halt_proved: false,
        outcome: "replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against(
    certificate: &Schema2E2Certificate,
    expected: &Schema2E2Certificate,
) -> Schema2E2Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    Schema2E2Replay {
        valid: errors.is_empty(),
        adjudications_adopted: certificate.adjudications.len() == 2
            && certificate
                .adjudications
                .iter()
                .all(|entry| entry.adjudication_state == AdjudicationState::AdoptedPolicy),
        step8_signatures_replayed: certificate.r2_step8.all_registered_signatures_replayed,
        fq4_triggered: certificate.falsifiers.fq4.triggered,
        e2_complete: certificate.e2_complete,
        later_phases_unexecuted: certificate
            .phases
            .iter()
            .skip(2)
            .all(|phase| phase.status == "not_executed_strict_order"),
        global_c1_retired: certificate
            .obstructions_retired
            .global_c1_arbitrary_typed_instance_sort_preservation_retired,
        c2_retired: certificate
            .obstructions_retired
            .c2_depth_two_schema_grammar_and_generator_completeness_retired,
        handoff_token_issued: certificate.ordinary_grammar.agent_a_handoff_token_issued,
        global_halt_proved: certificate.conclusion_boundary.global_halt_proved,
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_schema2_e2_certificate(certificate: &Schema2E2Certificate) -> Schema2E2Replay {
    match build_schema2_e2_certificate() {
        Ok(expected) => replay_against(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn schema2_e2_json_pretty() -> Result<String, Schema2E2CertificateError> {
    serde_json::to_string_pretty(&build_schema2_e2_certificate()?)
        .map(|json| format!("{json}\n"))
        .map_err(|error| Schema2E2CertificateError::Json(error.to_string()))
}

pub fn replay_schema2_e2_json(json: &str) -> Schema2E2Replay {
    let raw: Value = match serde_json::from_str(json) {
        Ok(value) => value,
        Err(error) => return failed_replay(format!("invalid JSON: {error}")),
    };
    let certificate: Schema2E2Certificate = match serde_json::from_str(json) {
        Ok(certificate) => certificate,
        Err(error) => return failed_replay(format!("certificate shape error: {error}")),
    };
    let typed = serde_json::to_value(&certificate).expect("E-2 certificate serializes");
    if raw != typed {
        return failed_replay("JSON contains unknown, duplicate, or ignored structure");
    }
    replay_schema2_e2_certificate(&certificate)
}

pub fn emit_schema2_e2_create_new(
    path: &Path,
) -> Result<Schema2E2Replay, Schema2E2CertificateError> {
    let json = schema2_e2_json_pretty()?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| Schema2E2CertificateError::Io(error.to_string()))?;
    output
        .write_all(json.as_bytes())
        .and_then(|()| output.flush())
        .map_err(|error| Schema2E2CertificateError::Io(error.to_string()))?;
    let replay = replay_schema2_e2_json(&json);
    if !replay.valid {
        return Err(Schema2E2CertificateError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn redigest(certificate: &mut Schema2E2Certificate) {
        certificate.result_digest = certificate_digest(certificate);
    }

    fn assert_rejected(mut certificate: Schema2E2Certificate) {
        redigest(&mut certificate);
        assert!(!replay_schema2_e2_certificate(&certificate).valid);
    }

    #[test]
    fn exact_source_pin_manifest_matches_embedded_bytes() {
        let mismatches = source_manifest()
            .into_iter()
            .filter_map(|spec| {
                let actual_length = spec.bytes.len() as u64;
                let actual_digest = bytes_hash(spec.bytes);
                (actual_length != spec.expected_length || actual_digest != spec.expected_blake3)
                    .then_some(format!("{}: {actual_length}/{actual_digest}", spec.name))
            })
            .collect::<Vec<_>>();
        assert!(mismatches.is_empty(), "{}", mismatches.join("\n"));
    }

    #[test]
    fn adopted_rules_replay_typed_inputs_and_stop_exactly_at_fq4() {
        let certificate = build_schema2_e2_certificate().expect("E-2 certificate");
        assert_eq!(certificate.adjudications.len(), 2);
        assert!(certificate.adjudications.iter().all(|entry| {
            entry.adjudication_state == AdjudicationState::AdoptedPolicy
                && entry.operational_decision_state
                    == OperationalDecisionState::PendingGeneratorMembership
                && !entry.operational_membership_verdict_issued
        }));
        assert!(certificate.r1_stage1.exact_syntactic_package_shape_replayed);
        assert!(
            certificate
                .r1_stage1
                .actual_stage1_typed_source_witness_issued
        );
        assert_eq!(certificate.r1_stage1.actual_stage1_typed_source_gap, None);
        assert_eq!(certificate.r1_stage1.carrier.constructor, "Univ");
        assert_eq!(certificate.r1_stage1.completion.constructor, "App");
        assert!(
            certificate
                .r1_stage1
                .completion
                .canonical_package_representative
        );
        assert_eq!(
            certificate.r1_stage1.coverage_direction,
            E2R1CoverageDirection::CompletionCoversCarrier
        );
        assert!(
            certificate
                .r1_stage1
                .every_local_role_exception_case_pending
        );
        assert_eq!(
            certificate
                .r1_stage1
                .pending_local_role_exception_cases
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["kernel_head", "adjoint_mate", "support_action", "coherence"]
        );
        assert!(!certificate.r1_stage1.carrier_exception_membership_decided);
        assert!(certificate.r1_stage1.completed_package_family_issued);
        assert!(
            certificate
                .r1_stage1
                .total_stage1_family_cardinality_omitted_pending_membership
        );
        assert!(!certificate.r1_stage1.historical_family_handoff_issued);

        assert!(certificate.r2_step8.all_registered_signatures_replayed);
        assert!(!certificate.r2_step8.fq1_typed_signature_failure_triggered);
        assert!(
            certificate
                .r2_step8
                .registered_interpretation_overlay_replayed
        );
        assert!(
            !certificate
                .r2_step8
                .raw_pen_core_to_schema2_term_elaboration_proved
        );
        assert!(!certificate.r2_step8.mu.naturality_generator_proof_issued);
        assert!(
            certificate
                .r2_step8
                .certificate_exact_bundle_digest_join_replayed
        );
        assert!(!certificate.r2_step8.left_unit.generator_membership_decided);
        assert!(
            !certificate
                .r2_step8
                .cell_action
                .generator_membership_decided
        );
        assert!(
            certificate
                .r2_step8
                .cell_action
                .boundary_derived_by_map_cube
        );
        assert!(certificate.falsifiers.fq4.triggered);
        assert!(!certificate.falsifiers.fq2_evaluated);
        assert!(!certificate.e2_complete);
        assert!(!certificate.ordinary_grammar.agent_a_handoff_token_issued);
        assert!(
            !certificate
                .ordinary_grammar
                .agda_rust_operational_correspondence_proved
        );
        assert!(
            !certificate
                .ordinary_grammar
                .generic_quotient_judgements_complete
        );
        assert!(
            certificate
                .ordinary_grammar
                .generic_uniform_instance_membership_pending
        );
        assert!(
            !certificate
                .ordinary_grammar
                .generic_r1_same_package_dependency_proved
        );
        assert!(
            certificate
                .ordinary_grammar
                .historical_reference_support_fragment_only
        );
        assert!(
            !certificate
                .ordinary_grammar
                .arbitrary_sealed_support_window_grammar_proved
        );
        assert!(!certificate.conclusion_boundary.global_halt_proved);
    }

    #[test]
    fn archived_counts_are_projection_outputs_never_rule_inputs() {
        let certificate = build_schema2_e2_certificate().expect("E-2 certificate");
        assert_eq!(
            certificate
                .historical_regression
                .archived_raw_operational_totals,
            [7, 8, 10, 18]
        );
        assert_eq!(
            certificate
                .historical_regression
                .archived_registered_path_typed_and_marginal_subtotals,
            [2, 2, 5, 10]
        );
        assert_eq!(
            certificate
                .historical_regression
                .archived_ordinary_unresolved_totals,
            [5, 6, 5, 8]
        );
        assert!(certificate.historical_regression.source_projection_replayed);
        assert!(
            !certificate
                .historical_regression
                .count_used_as_constructor_or_membership_input
        );
        assert!(
            !certificate
                .historical_regression
                .revised_e2_output_regression_run
        );
        assert!(!certificate.historical_regression.fq2_evaluated);
        assert!(
            !certificate
                .historical_regression
                .ordinary_historical_tokens_complete
        );
        assert!(
            !certificate
                .historical_regression
                .agent_a_handoff_token_issued
        );
        let value = serde_json::to_value(&certificate).expect("certificate projects");
        assert!(value["r1_stage1"].get("package_family_count").is_none());
    }

    #[test]
    fn strict_json_rejects_unknown_and_duplicate_fields() {
        let json = schema2_e2_json_pretty().expect("JSON");
        assert!(replay_schema2_e2_json(&json).valid);
        let unknown = json.replacen("{\n", "{\n  \"unknown\": true,\n", 1);
        assert!(!replay_schema2_e2_json(&unknown).valid);
        let duplicate = json.replacen(
            "\"schema\": \"schema2-e2-successor-v2\",",
            "\"schema\": \"schema2-e2-successor-v2\",\n  \"schema\": \"schema2-e2-successor-v2\",",
            1,
        );
        assert!(!replay_schema2_e2_json(&duplicate).valid);
        let unknown_deep = json.replacen(
            "\"falsifiers\": {",
            "\"falsifiers\": {\n    \"invented\": true,",
            1,
        );
        assert!(!replay_schema2_e2_json(&unknown_deep).valid);
    }

    #[test]
    fn membership_count_handoff_phase_and_halt_mutations_fail_after_redigest() {
        let expected = build_schema2_e2_certificate().expect("E-2 certificate");

        let mut r1_membership = expected.clone();
        r1_membership.r1_stage1.carrier_exception_membership_decided = true;
        assert_rejected(r1_membership);

        let mut r2_membership = expected.clone();
        r2_membership
            .r2_step8
            .cell_action
            .generator_membership_decided = true;
        r2_membership
            .r2_step8
            .cell_action
            .independent_support_action_issued = true;
        assert_rejected(r2_membership);

        let mut count_input = expected.clone();
        count_input
            .historical_regression
            .count_used_as_constructor_or_membership_input = true;
        assert_rejected(count_input);

        let mut changed_count = expected.clone();
        changed_count
            .historical_regression
            .archived_raw_operational_totals[0] += 1;
        assert_rejected(changed_count);

        let mut handoff = expected.clone();
        handoff.ordinary_grammar.agent_a_handoff_token_issued = true;
        assert_rejected(handoff);

        let mut e2 = expected.clone();
        e2.e2_complete = true;
        assert_rejected(e2);

        let mut later = expected.clone();
        later.phases[2].status = "complete".to_owned();
        later.phases[2].theorem_token_issued = true;
        later.phases[2].blocked_by = None;
        assert_rejected(later);

        let mut halt = expected;
        halt.conclusion_boundary.global_halt_proved = true;
        assert_rejected(halt);
    }

    #[test]
    fn create_new_emission_refuses_overwrite_and_preserves_first_artifact() {
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "pen-schema-e2-create-new-{}-{counter}",
            std::process::id()
        ));
        std::fs::create_dir(&directory).expect("unique temporary directory");
        let path = directory.join("schema2_e2.json");
        let replay = emit_schema2_e2_create_new(&path).expect("first create-new emit");
        assert!(replay.valid);
        let first = std::fs::read(&path).expect("first artifact bytes");
        assert!(matches!(
            emit_schema2_e2_create_new(&path),
            Err(Schema2E2CertificateError::Io(_))
        ));
        assert_eq!(std::fs::read(&path).expect("preserved artifact"), first);
        std::fs::remove_file(&path).expect("remove test artifact");
        std::fs::remove_dir(&directory).expect("remove test directory");
    }
}
