//! BC-1: depth-two grammar/generator-completeness theorem attempt.
//!
//! The issuer first replays the sealed M-3 v1 baseline, then reconstructs the
//! complete frozen component stock without treating that stock as an
//! independently defined semantic domain.  The existing ledger proves the
//! depth-one trace campaign, the operational ordinary registry, the public
//! cubical inventory, the transparent-former obligations, and dependent
//! context formation at their registered scopes.  It does not independently
//! define the intended depth-two semantic signature surface.  Consequently a
//! generator-completeness theorem over that surface would be circular.  BC-1
//! records that exact obstruction and stops without promotion or
//! semantic-family credit.

use crate::m3_e7_e8_bridge_v1::{
    M3_E7_GAP, M3E7E8BridgeV1Certificate, M3RunStatus, replay_m3_e7_e8_bridge_v1_json,
};
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_schema::e4_generator_basis::{
    E4GeneratorKind, issue_e4_generator_basis_audit, replay_e4_generator_basis_audit,
};
use pen_schema::e34_class_certificate::Schema2E34ClassCertificate;
use pen_schema::e34_class_induction::CubicalConstructorKind;
use pen_schema::grammar::OrdinarySchemaKind;
use pen_schema::grammar_completion::{
    GRAMMAR_COMPLETION_SCHEMA, GrammarCompletionCertificate, RegisteredTraceSignature,
};
use pen_schema::internal_classifier_branch_v5::AmbientFormerInternalityCertificate;
use pen_type::ambient_former_internality::registered_transparent_formers;
use pen_type::elaborate::{SealedSignature, elaborate_telescope};
use pen_type::normalize::normalize;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest as _, Sha256};
use std::collections::BTreeSet;
use std::fs::{OpenOptions, read_to_string, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const BC1_DEPTH_TWO_COMPLETENESS_V1_SCHEMA: &str =
    "bridge-completion-bc1-depth-two-completeness-v1";
pub const BC1_DEPTH_TWO_COMPLETENESS_V1_DATE: &str = "2026-07-23";
pub const BC1_CERTIFICATE_NAME: &str = "bc1_depth_two_completeness_v1.json";
pub const BC1_REPORT_NAME: &str = "BC1_DEPTH_TWO_COMPLETENESS_RESULT.md";

pub const BC1_INDEPENDENT_SURFACE_GAP: &str =
    "BC1_INDEPENDENT_DEPTH_TWO_SEMANTIC_SIGNATURE_SURFACE_NOT_DEFINED";
pub const BC1_GENERATOR_INDUCTION_GAP: &str =
    "BC1_DEPTH_TWO_GENERATOR_COVERAGE_INDUCTION_NOT_PROVED";
pub const BC1_DEPTH_ONE_RESTRICTION_GAP: &str =
    "BC1_DEPTH_ONE_RESTRICTION_OF_DEPTH_TWO_THEOREM_UNFORMABLE";

const EXPECTED_M3_RESULT_DIGEST: &str =
    "blake3:a61fe8093ef9ce61a05aebfe859b0723a88a766a712ea6a2312e0f22ca386ba8";
const EXPECTED_GRAMMAR_RESULT_DIGEST: &str =
    "blake3:26ccc358c25ace6501e81125363064730c1642cfe775b0a931e8298bc5d60215";
const EXPECTED_V4_RESULT_DIGEST: &str =
    "blake3:c5b598eba49f66cb7f845fdab43ab217a40b7d9ce2ec971a91989712711fcc4a";
const EXPECTED_AMBIENT_RESULT_DIGEST: &str =
    "blake3:41b5451dc892b7d131230524df3ca8a5df1efe9fee49ea812c9c1de7944756c4";
const EXPECTED_GLOBAL_E4_RESULT_DIGEST: &str =
    "blake3:118d015f6cabe967052891d4d4dab9660b9837a1f4e3a588fe7f2108bb4d5d2b";
const EXPECTED_A3_RESULT_DIGEST: &str =
    "blake3:bd3f33ed7a15b6f50c421d2e42ed7aaea710fbcab750d0a98a990a1dad3b30db";

const EXPECTED_M3_SHA256: &str = "22cc8450829a403081c1d4db7125523c198c7e902f9f730f415be2fc0f305aa0";
const EXPECTED_GRAMMAR_SHA256: &str =
    "c61e58dca309d80221e9336806436754ef5b81c380f452b338474acade5e9f10";
const EXPECTED_V4_SHA256: &str = "ffe5e195e136aeb3f142ed4b30b6cabcbb36988347b71f9c1deae841d675e56a";
const EXPECTED_AMBIENT_SHA256: &str =
    "5ab7fcb5e324d22114c4639d5f2c9353ef6ceb3bb4975e8b07bc5994b3e40390";
const EXPECTED_GLOBAL_E4_SHA256: &str =
    "66ff643f1ec901be273469e6a5afc61b31d7677aae11f80723b3bcad8fd90c04";
const EXPECTED_A3_SHA256: &str = "73d5d25cf9a91ab82100f14bb1e1cfe424c97ab0ac2503664278df4268ac5337";

const BRIDGE_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/bridge_completion_plan.md");
const MAINLINE_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/mainline_completion_plan.md");
const STEP15_SPEC_BYTES: &[u8] = include_bytes!("../../../docs/step_15_completion_open_problem.md");
const AGENT_E_PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_e_schema2_plan.md");
const GRAMMAR_PLAN_BYTES: &[u8] =
    include_bytes!("../../../docs/agent_e_grammar_completion_plan.md");
const SIGNATURE_ADOPTION_BYTES: &[u8] =
    include_bytes!("../../../docs/steps_9_15_signature_adoption.md");
const SIGNATURE_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/steps_9_15_signature_adjudication.md");
const ENDPOINT_API_BYTES: &[u8] =
    include_bytes!("../../../docs/endpoint_premise_api_adjudication.md");
const AMBIENT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/ambient_former_closure_adjudication.md");
const DEPENDENT_CONTEXT_BYTES: &[u8] =
    include_bytes!("../../../docs/dependent_context_adjudication.md");
const MOTIVE_COHERENCE_BYTES: &[u8] =
    include_bytes!("../../../docs/motive_parametric_coherence_adjudication.md");
const NU_REGISTER_BYTES: &[u8] = include_bytes!("../../../docs/nu_register_adjudication.md");
const M3_BYTES: &[u8] = include_bytes!("../../../docs/schema2_e7_e8_bridge_v1.json");
const GRAMMAR_BYTES: &[u8] = include_bytes!("../../../docs/schema2_grammar_completion_v1.json");
const SCHEMA2_V4_BYTES: &[u8] = include_bytes!("../../../docs/schema2_v4.json");
const AMBIENT_CERTIFICATE_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_ambient_former_internality_v1.json");
const GLOBAL_E4_BYTES: &[u8] = include_bytes!("../../../docs/schema2_global_e4_assembly_v10.json");
const A3_EXHAUSTIVENESS_BYTES: &[u8] =
    include_bytes!("../../../docs/a3_rule_inventory_exhaustiveness_v2.json");
const TRACE_BYTES: &[u8] = include_bytes!("../../pen-core/src/telescope.rs");
const GENERATOR_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/e4_generator_basis.rs");
const DEPENDENT_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/dependent_context.rs");
const AMBIENT_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/ambient_former_internality.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("bc1_depth_two_completeness_v1.rs");

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc1Register {
    SemanticRegisterAuthority,
    SemanticFamilyAuthority,
    StructuralTestimony,
    ArtifactMetadata,
    SyntaxIdentifier,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc1Quantifier {
    FrozenWrappedSurface,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc1RunStatus {
    StoppedNamedGaps,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc1Disposition {
    ReplayedTraceSeed,
    ReplayedOperationalConstructor,
    ReplayedCubicalConstructor,
    ReplayedTransparentFormerRule,
    AdoptedDependentContextRule,
    NamedImpossible,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Bc1ObligationStatus {
    ProvedScoped,
    OpenNamedGap,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1TaggedNumber {
    pub value: usize,
    pub register: Bc1Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: Bc1TaggedNumber,
    pub blake3: String,
    pub sha256: String,
    pub register: Bc1Register,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1M3Baseline {
    pub result_digest: String,
    pub public_replay_valid: bool,
    pub public_replay_errors: Vec<String>,
    pub exact_enacted_regression_reproduced: bool,
    pub proved_bridge_condition_count: Bc1TaggedNumber,
    pub bridge_condition_count: Bc1TaggedNumber,
    pub c2_open_at_baseline: bool,
    pub no_promotions_at_baseline: bool,
    pub baseline_shift_detected: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1TraceRestrictionRow {
    pub component_id: String,
    pub stage_identifier: String,
    pub clause_identifier: String,
    pub registered_name: String,
    pub exact_trace_expression: bool,
    pub exact_trace_role: bool,
    pub exact_predecessor_signature: bool,
    pub term_level_elaboration_rederived: bool,
    pub exact_kernel_type_rederived: bool,
    pub exact_normal_form_rederived: bool,
    pub normalization_stable: bool,
    pub support_local: bool,
    pub naturality_token_retained: bool,
    pub raw_expression_hash: String,
    pub typed_interpretation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1DepthOneRegression {
    pub grammar_result_digest: String,
    pub archived_grammar_bytes_authenticated: bool,
    pub archived_live_definition_replay_valid: bool,
    pub archived_live_replay_errors: Vec<String>,
    pub trace_signature_count: Bc1TaggedNumber,
    pub ordinary_constructor_count: Bc1TaggedNumber,
    pub standalone_depth_one_component_count: Bc1TaggedNumber,
    pub trace_rows: Vec<Bc1TraceRestrictionRow>,
    pub every_trace_row_rederived_from_sealed_trace: bool,
    pub every_ordinary_constructor_normalized_and_natural: bool,
    pub endpoint_publication_complete: bool,
    pub p6_is_derived_closure_with_zero_extra_exports: bool,
    pub no_unit_coherence_generator_issued: bool,
    pub standalone_depth_one_campaign_rederived: bool,
    pub depth_two_restriction_map_defined: bool,
    pub depth_one_rederived_as_depth_two_restriction: bool,
    pub restriction_named_gap: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1SurfaceComponent {
    pub component_id: String,
    pub component_class: String,
    pub disposition: Bc1Disposition,
    pub exact_scope: String,
    pub typed_or_logically_disposed: bool,
    pub normalization_naturality_or_closure_replayed: bool,
    pub zero_charge: bool,
    pub named_impossibility: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1FrozenComponentSurface {
    pub ordinary_component_count: Bc1TaggedNumber,
    pub trace_component_count: Bc1TaggedNumber,
    pub cubical_component_count: Bc1TaggedNumber,
    pub transparent_former_component_count: Bc1TaggedNumber,
    pub dependent_context_rule_count: Bc1TaggedNumber,
    pub frozen_component_stock_count: Bc1TaggedNumber,
    pub explicit_impossibility_count: Bc1TaggedNumber,
    pub components: Vec<Bc1SurfaceComponent>,
    pub explicit_impossibilities: Vec<Bc1SurfaceComponent>,
    pub every_frozen_component_has_explicit_disposition: bool,
    pub every_explicit_impossibility_replayed: bool,
    pub component_stock_is_independent_depth_two_semantic_domain: bool,
    pub component_stock_used_as_completeness_definition: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1Obligation {
    pub obligation: String,
    pub status: Bc1ObligationStatus,
    pub exact_scope: String,
    pub named_gap: Option<String>,
    pub register: Bc1Register,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1DepthTwoAttempt {
    pub quantifier: Bc1Quantifier,
    pub register: Bc1Register,
    pub independently_defined_semantic_domain: bool,
    pub total_typed_interpretation_on_independent_domain: bool,
    pub trace_and_adopted_former_coverage_proved: bool,
    pub typed_context_morphism_decomposition_replayed: bool,
    pub generator_kind_count: Bc1TaggedNumber,
    pub generator_kinds: Vec<String>,
    pub transparent_former_matrix_replayed_at_registered_scope: bool,
    pub dependent_context_rule_replayed_at_registered_scope: bool,
    pub transparent_and_dependent_rules_bound_to_independent_domain: bool,
    pub class_indexed_depth_two_generator_induction_proved: bool,
    pub depth_one_restriction_theorem_proved: bool,
    pub zero_charge_law_preserved: bool,
    pub a3_relative_exhaustiveness_replayed: bool,
    pub a3_absolute_semantic_exhaustiveness_claimed: bool,
    pub global_e4_wrapped_candidate_exhaustion_replayed: bool,
    pub global_e4_used_as_semantic_domain_exhaustiveness: bool,
    pub obligation_count: Bc1TaggedNumber,
    pub obligations: Vec<Bc1Obligation>,
    pub every_obligation_has_explicit_disposition: bool,
    pub silent_residue_claimed_absent_without_domain: bool,
    pub f_bc5_expressivity_gap_fired: bool,
    pub c2_closed: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1NamedGap {
    pub id: String,
    pub phase: String,
    pub quantifier: Bc1Quantifier,
    pub register: Bc1Register,
    pub exact_obstruction: String,
    pub keeps_c2_open: bool,
    pub keeps_rows_unpromoted: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1DepthTwoCompletenessV1Certificate {
    pub schema: String,
    pub date: String,
    pub syntax_identifier_policy: String,
    pub syntax_identifier_register: Bc1Register,
    pub source_bindings: Vec<Bc1SourceBinding>,
    pub m3_v1_baseline: Bc1M3Baseline,
    pub depth_one_regression: Bc1DepthOneRegression,
    pub frozen_component_surface: Bc1FrozenComponentSurface,
    pub depth_two_attempt: Bc1DepthTwoAttempt,
    pub open_gaps: Vec<Bc1NamedGap>,
    pub open_gap_count: Bc1TaggedNumber,
    pub c2_closed: bool,
    pub no_promotions_made: bool,
    pub promoted_row_count: Bc1TaggedNumber,
    pub new_semantic_family_credits: Bc1TaggedNumber,
    pub no_value_bar_hash_or_enumeration_order_selector: bool,
    pub m3_v2_prerequisite_satisfied: bool,
    pub bc1_status: Bc1RunStatus,
    pub mutation_falsifiers: Vec<String>,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bc1Replay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub bc1_status: Bc1RunStatus,
    pub c2_closed: bool,
    pub no_promotions_made: bool,
    pub m3_v2_prerequisite_satisfied: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Bc1Error {
    #[error("BC-1 baseline failure: {0}")]
    Baseline(String),
    #[error("BC-1 input failure: {0}")]
    Input(String),
    #[error("BC-1 theorem invariant failure: {0}")]
    Invariant(String),
    #[error("BC-1 JSON failure: {0}")]
    Json(String),
    #[error("BC-1 create-new I/O failure: {0}")]
    Io(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BC1_DEPTH_TWO_COMPLETENESS_V1_SCHEMA, domain, value))
        .expect("BC-1 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn structural_number(value: usize) -> Bc1TaggedNumber {
    Bc1TaggedNumber {
        value,
        register: Bc1Register::StructuralTestimony,
    }
}

fn metadata_number(value: usize) -> Bc1TaggedNumber {
    Bc1TaggedNumber {
        value,
        register: Bc1Register::ArtifactMetadata,
    }
}

fn semantic_family_number(value: usize) -> Bc1TaggedNumber {
    Bc1TaggedNumber {
        value,
        register: Bc1Register::SemanticFamilyAuthority,
    }
}

fn utf8<'a>(path: &str, bytes: &'a [u8]) -> Result<&'a str, Bc1Error> {
    std::str::from_utf8(bytes).map_err(|_| Bc1Error::Input(format!("{path} is not valid UTF-8")))
}

fn assert_sha(path: &str, bytes: &[u8], expected: &str) -> Result<(), Bc1Error> {
    let found = sha256(bytes);
    if found != expected {
        return Err(Bc1Error::Input(format!(
            "sealed bytes drifted for {path}: expected {expected}, found {found}"
        )));
    }
    Ok(())
}

fn source_bindings() -> Vec<Bc1SourceBinding> {
    [
        (
            "docs/bridge_completion_plan.md",
            "frozen BC-1 mission and fail-closed exit",
            BRIDGE_PLAN_BYTES,
        ),
        (
            "docs/mainline_completion_plan.md",
            "frozen M-3 baseline and M-4 gate",
            MAINLINE_PLAN_BYTES,
        ),
        (
            "docs/step_15_completion_open_problem.md",
            "independent semantic-domain and classifier specification",
            STEP15_SPEC_BYTES,
        ),
        (
            "docs/agent_e_schema2_plan.md",
            "class-indexed E-7 and one-way fallback contract",
            AGENT_E_PLAN_BYTES,
        ),
        (
            "docs/agent_e_grammar_completion_plan.md",
            "depth-one grammar-completion firewall",
            GRAMMAR_PLAN_BYTES,
        ),
        (
            "docs/steps_9_15_signature_adoption.md",
            "adopted trace-faithful signature authority",
            SIGNATURE_ADOPTION_BYTES,
        ),
        (
            "docs/steps_9_15_signature_adjudication.md",
            "independent sealed trace signature proposal",
            SIGNATURE_ADJUDICATION_BYTES,
        ),
        (
            "docs/endpoint_premise_api_adjudication.md",
            "adopted public cubical endpoint rule",
            ENDPOINT_API_BYTES,
        ),
        (
            "docs/ambient_former_closure_adjudication.md",
            "adopted transparent former partition",
            AMBIENT_ADJUDICATION_BYTES,
        ),
        (
            "docs/dependent_context_adjudication.md",
            "adopted dependent context rule",
            DEPENDENT_CONTEXT_BYTES,
        ),
        (
            "docs/motive_parametric_coherence_adjudication.md",
            "generic substitution induction authority",
            MOTIVE_COHERENCE_BYTES,
        ),
        (
            "docs/nu_register_adjudication.md",
            "semantic authority and structural testimony split",
            NU_REGISTER_BYTES,
        ),
        (
            "docs/schema2_e7_e8_bridge_v1.json",
            "sealed M-3 v1 hard baseline",
            M3_BYTES,
        ),
        (
            "docs/schema2_grammar_completion_v1.json",
            "sealed depth-one trace campaign",
            GRAMMAR_BYTES,
        ),
        (
            "docs/schema2_v4.json",
            "sealed ordinary and cubical class induction",
            SCHEMA2_V4_BYTES,
        ),
        (
            "docs/schema2_ambient_former_internality_v1.json",
            "sealed term-level transparent former matrix",
            AMBIENT_CERTIFICATE_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v10.json",
            "sealed wrapped-candidate exhaustion with narrow scope",
            GLOBAL_E4_BYTES,
        ),
        (
            "docs/a3_rule_inventory_exhaustiveness_v2.json",
            "sealed relative A3 exhaustiveness and absolute-scope disclaimer",
            A3_EXHAUSTIVENESS_BYTES,
        ),
        (
            "crates/pen-core/src/telescope.rs",
            "sealed reference trace constructor",
            TRACE_BYTES,
        ),
        (
            "crates/pen-schema/src/e4_generator_basis.rs",
            "typed context-morphism decomposition",
            GENERATOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/dependent_context.rs",
            "dependent-context operational judgments",
            DEPENDENT_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/ambient_former_internality.rs",
            "transparent-former operational closure",
            AMBIENT_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/bc1_depth_two_completeness_v1.rs",
            "BC-1 issuer replay and mutation logic",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| Bc1SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: metadata_number(bytes.len()),
        blake3: bytes_hash(bytes),
        sha256: sha256(bytes),
        register: Bc1Register::ArtifactMetadata,
    })
    .collect()
}

fn json_string(value: &Value, key: &str) -> Result<String, Bc1Error> {
    value
        .get(key)
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .ok_or_else(|| Bc1Error::Json(format!("missing string field {key}")))
}

fn json_bool(value: &Value, key: &str) -> Result<bool, Bc1Error> {
    value
        .get(key)
        .and_then(Value::as_bool)
        .ok_or_else(|| Bc1Error::Json(format!("missing Boolean field {key}")))
}

fn serialized_identifier<T: Serialize>(value: &T) -> Result<String, Bc1Error> {
    serde_json::to_value(value)
        .map_err(|error| Bc1Error::Json(error.to_string()))?
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| Bc1Error::Json("enum did not serialize as an identifier".to_owned()))
}

fn authenticate_sealed_inputs() -> Result<(), Bc1Error> {
    assert_sha(
        "docs/schema2_e7_e8_bridge_v1.json",
        M3_BYTES,
        EXPECTED_M3_SHA256,
    )?;
    assert_sha(
        "docs/schema2_grammar_completion_v1.json",
        GRAMMAR_BYTES,
        EXPECTED_GRAMMAR_SHA256,
    )?;
    assert_sha("docs/schema2_v4.json", SCHEMA2_V4_BYTES, EXPECTED_V4_SHA256)?;
    assert_sha(
        "docs/schema2_ambient_former_internality_v1.json",
        AMBIENT_CERTIFICATE_BYTES,
        EXPECTED_AMBIENT_SHA256,
    )?;
    assert_sha(
        "docs/schema2_global_e4_assembly_v10.json",
        GLOBAL_E4_BYTES,
        EXPECTED_GLOBAL_E4_SHA256,
    )?;
    assert_sha(
        "docs/a3_rule_inventory_exhaustiveness_v2.json",
        A3_EXHAUSTIVENESS_BYTES,
        EXPECTED_A3_SHA256,
    )
}

fn build_m3_baseline() -> Result<(Bc1M3Baseline, M3E7E8BridgeV1Certificate), Bc1Error> {
    let m3_json = utf8("docs/schema2_e7_e8_bridge_v1.json", M3_BYTES)?;
    let replay = replay_m3_e7_e8_bridge_v1_json(m3_json);
    if !replay.valid {
        return Err(Bc1Error::Baseline(format!(
            "M-3 v1 public replay failed: {}",
            replay.errors.join("; ")
        )));
    }
    let m3: M3E7E8BridgeV1Certificate =
        serde_json::from_str(m3_json).map_err(|error| Bc1Error::Json(error.to_string()))?;
    let proved = m3
        .e8
        .bridge_conditions
        .iter()
        .filter(|condition| condition.proved)
        .count();
    let total = m3.e8.bridge_conditions.len();
    let exact_enacted_regression_reproduced = m3.enacted_regression.projections_exactly_equal
        && m3.enacted_regression.enacted_bi2_root_exactly_joins_bi4
        && m3
            .enacted_regression
            .e5_projection
            .theorem12_full_instance_granularity
        && m3.enacted_regression.e5_projection.semantic_o16_empty
        && m3.enacted_regression.e5_projection.f1_excluded;
    if m3.result_digest != EXPECTED_M3_RESULT_DIGEST
        || replay.m3_status != M3RunStatus::StoppedNamedGaps
        || proved != 4
        || total != 9
        || !exact_enacted_regression_reproduced
        || m3.e7.c2_closed
        || m3.e7.named_gap != M3_E7_GAP
        || !m3.zero_promotions_made
        || m3.promoted_row_count.value != 0
        || m3.bridge_claim_issued
        || m3.m4_authorized
    {
        return Err(Bc1Error::Baseline(
            "M-3 v1 baseline shifted from the frozen fail-closed result".to_owned(),
        ));
    }
    let derivation_hash = tagged_hash(
        "m3-v1-baseline",
        &(
            m3.result_digest.as_str(),
            proved,
            total,
            exact_enacted_regression_reproduced,
            m3.e7.named_gap.as_str(),
        ),
    );
    Ok((
        Bc1M3Baseline {
            result_digest: m3.result_digest.clone(),
            public_replay_valid: true,
            public_replay_errors: replay.errors,
            exact_enacted_regression_reproduced,
            proved_bridge_condition_count: metadata_number(proved),
            bridge_condition_count: metadata_number(total),
            c2_open_at_baseline: true,
            no_promotions_at_baseline: true,
            baseline_shift_detected: false,
            derivation_hash,
        },
        m3,
    ))
}

fn trace_row(registration: &RegisteredTraceSignature) -> Result<Bc1TraceRestrictionRow, Bc1Error> {
    let telescope = Telescope::reference(registration.step);
    let index = usize::from(registration.clause_index);
    let clause = telescope.clauses.get(index).ok_or_else(|| {
        Bc1Error::Invariant(format!(
            "registered trace row {}:{} is outside its telescope",
            registration.step, registration.clause_index
        ))
    })?;
    let prefix = SealedSignature::from_telescopes(
        (1..registration.step)
            .map(|step| (step, Telescope::reference(step)))
            .collect(),
    );
    let elaboration = elaborate_telescope(&prefix, &telescope, registration.step - 1)
        .map_err(|error| Bc1Error::Invariant(error.to_string()))?;
    let typed = elaboration.clauses.get(index).ok_or_else(|| {
        Bc1Error::Invariant("elaboration omitted a registered trace row".to_owned())
    })?;
    let scope = elaboration.ambient_parameters + u32::from(registration.clause_index);
    let renormalized = normalize(&typed.normal_form, scope, 256)
        .map_err(|error| Bc1Error::Invariant(error.to_string()))?;
    let exact_trace_expression = clause.expr == registration.raw_expr;
    let exact_trace_role =
        clause.role == registration.declared_role && typed.kernel_role == registration.kernel_role;
    let exact_predecessor_signature = prefix.digest() == registration.predecessor_signature_digest;
    let exact_kernel_type_rederived = serde_json::to_string(&typed.kernel_ty)
        .map_err(|error| Bc1Error::Json(error.to_string()))?
        == registration.kernel_type_json;
    let exact_normal_form_rederived = typed.normal_form == registration.normal_form;
    let normalization_stable = renormalized.expr == typed.normal_form;
    let support_local = clause
        .expr
        .lib_refs()
        .iter()
        .all(|reference| *reference < registration.step)
        && registration.support_local;
    let naturality_token_retained =
        registration.normalization_natural && registration.normalization_stable;
    if !exact_trace_expression
        || !exact_trace_role
        || !exact_predecessor_signature
        || !exact_kernel_type_rederived
        || !exact_normal_form_rederived
        || !normalization_stable
        || !support_local
        || !naturality_token_retained
    {
        return Err(Bc1Error::Invariant(format!(
            "depth-one trace reconstruction failed for {}:{}",
            registration.step, registration.clause_index
        )));
    }
    let raw_expression_hash = tagged_hash("depth-one-raw-expression", &registration.raw_expr);
    let typed_interpretation_hash = tagged_hash(
        "depth-one-typed-interpretation",
        &(
            &typed.kernel_role,
            &registration.kernel_type_json,
            &typed.normal_form,
        ),
    );
    let component_id = format!(
        "trace_stage_{}_clause_{}_{}",
        registration.step, registration.clause_index, registration.registered_name
    );
    let derivation_hash = tagged_hash(
        "depth-one-restriction-row",
        &(
            component_id.as_str(),
            &raw_expression_hash,
            &typed_interpretation_hash,
            exact_predecessor_signature,
            normalization_stable,
            support_local,
        ),
    );
    Ok(Bc1TraceRestrictionRow {
        component_id,
        stage_identifier: format!("stage_{}", registration.step),
        clause_identifier: format!("clause_{}", registration.clause_index),
        registered_name: registration.registered_name.clone(),
        exact_trace_expression,
        exact_trace_role,
        exact_predecessor_signature,
        term_level_elaboration_rederived: true,
        exact_kernel_type_rederived,
        exact_normal_form_rederived,
        normalization_stable,
        support_local,
        naturality_token_retained,
        raw_expression_hash,
        typed_interpretation_hash,
        derivation_hash,
    })
}

fn surface_component(
    component_id: String,
    component_class: &str,
    disposition: Bc1Disposition,
    exact_scope: &str,
    normalized_or_closed: bool,
    zero_charge: bool,
    named_impossibility: Option<&str>,
) -> Bc1SurfaceComponent {
    let derivation_hash = tagged_hash(
        "frozen-surface-component",
        &(
            component_id.as_str(),
            component_class,
            disposition,
            exact_scope,
            normalized_or_closed,
            zero_charge,
            named_impossibility,
        ),
    );
    Bc1SurfaceComponent {
        component_id,
        component_class: component_class.to_owned(),
        disposition,
        exact_scope: exact_scope.to_owned(),
        typed_or_logically_disposed: true,
        normalization_naturality_or_closure_replayed: normalized_or_closed,
        zero_charge,
        named_impossibility: named_impossibility.map(ToOwned::to_owned),
        derivation_hash,
    }
}

fn build_depth_one_and_surface()
-> Result<(Bc1DepthOneRegression, Bc1FrozenComponentSurface), Bc1Error> {
    let grammar: GrammarCompletionCertificate =
        serde_json::from_slice(GRAMMAR_BYTES).map_err(|error| Bc1Error::Json(error.to_string()))?;
    let class_v4: Schema2E34ClassCertificate = serde_json::from_slice(SCHEMA2_V4_BYTES)
        .map_err(|error| Bc1Error::Json(error.to_string()))?;
    let ambient: AmbientFormerInternalityCertificate =
        serde_json::from_slice(AMBIENT_CERTIFICATE_BYTES)
            .map_err(|error| Bc1Error::Json(error.to_string()))?;
    let a3: Value = serde_json::from_slice(A3_EXHAUSTIVENESS_BYTES)
        .map_err(|error| Bc1Error::Json(error.to_string()))?;

    if grammar.schema != GRAMMAR_COMPLETION_SCHEMA
        || grammar.result_digest != EXPECTED_GRAMMAR_RESULT_DIGEST
        || !grammar.all_43_trace_signatures_registered
        || !grammar.full_adopted_grammar_normalization_naturality_complete
        || !grammar.no_extra_primitive_export_rule_holds
        || !grammar.step15_j3_exact
        || grammar.g6_unit_coherence.generator_issued
        || grammar.g7_induction.ordinary_constructor_count != OrdinarySchemaKind::ALL.len()
        || grammar.g7_induction.total_public_cubical_constructor_count
            != CubicalConstructorKind::ALL.len()
        || !grammar.g7_induction.every_endpoint_term_typed
        || !grammar.g7_induction.every_endpoint_substitution_natural
        || !grammar.g7_induction.endpoint_premise_charges_zero
    {
        return Err(Bc1Error::Input(
            "sealed grammar-completion projection failed its exact scope gates".to_owned(),
        ));
    }
    if class_v4.result_digest != EXPECTED_V4_RESULT_DIGEST
        || class_v4.ordinary.registered_constructor_count != OrdinarySchemaKind::ALL.len()
        || !class_v4
            .ordinary
            .inventory_exhaustive_for_operational_registry
        || !class_v4.ordinary.every_constructor_normalized_and_natural
        || class_v4.ordinary.intended_schema2_inventory_exhaustive
    {
        return Err(Bc1Error::Input(
            "sealed v4 ordinary-registry projection failed".to_owned(),
        ));
    }
    let expected_formers = registered_transparent_formers();
    if ambient.result_digest != EXPECTED_AMBIENT_RESULT_DIGEST
        || !ambient.former_inventory_complete
        || ambient.registered_transparent_formers != expected_formers
        || ambient.former_obligations.len() != expected_formers.len()
        || ambient.former_obligations.iter().any(|obligation| {
            !obligation.target_occurs_in_replayed_tree
                || !obligation.typed_result_preserved
                || !obligation.full_provenance_retained
                || obligation.marginal_nu != 0
        })
        || !ambient.falsifiers.pathcon_rejected_as_charged
        || !ambient.falsifiers.candidate_declared_formation_rejected
        || !ambient.falsifiers.uncertified_premise_rejected
        || !ambient.falsifiers.candidate_fresh_head_rejected
        || !ambient
            .falsifiers
            .guarded_candidate_rejected_without_inverse_evidence
        || ambient.marginal_nu != 0
    {
        return Err(Bc1Error::Input(
            "sealed transparent-former matrix failed".to_owned(),
        ));
    }

    let a3_live_replay_errors = a3
        .get("completed_schema2_live_replay_errors")
        .and_then(Value::as_array)
        .ok_or_else(|| Bc1Error::Json("A3 archive lacks grammar drift testimony".to_owned()))?
        .iter()
        .map(|entry| {
            entry
                .as_str()
                .map(ToOwned::to_owned)
                .ok_or_else(|| Bc1Error::Json("A3 drift testimony is not text".to_owned()))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if json_string(&a3, "result_digest")? != EXPECTED_A3_RESULT_DIGEST
        || !json_bool(
            &a3,
            "relative_rule_constructor_inventory_exhaustiveness_proved",
        )?
        || json_bool(&a3, "broader_absolute_semantic_exhaustiveness_claimed")?
        || json_bool(&a3, "completed_schema2_live_definition_replay_succeeded")?
        || a3_live_replay_errors.is_empty()
    {
        return Err(Bc1Error::Input(
            "A3 relative-scope and drift testimony changed".to_owned(),
        ));
    }

    let trace_rows = grammar
        .registrations
        .iter()
        .map(trace_row)
        .collect::<Result<Vec<_>, _>>()?;
    if trace_rows.len() != grammar.registrations.len()
        || trace_rows
            .iter()
            .map(|row| row.component_id.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            != trace_rows.len()
    {
        return Err(Bc1Error::Invariant(
            "depth-one trace restriction has duplicate or missing rows".to_owned(),
        ));
    }

    let ordinary_count = OrdinarySchemaKind::ALL.len();
    let trace_count = trace_rows.len();
    let cubical_count = CubicalConstructorKind::ALL.len();
    let former_count = expected_formers.len();
    let dependent_count = 1usize;

    let mut components = Vec::new();
    for kind in OrdinarySchemaKind::ALL {
        let id = serialized_identifier(&kind)?;
        if !class_v4.ordinary.registered_constructors.contains(&id) {
            return Err(Bc1Error::Invariant(format!(
                "ordinary constructor {id} is absent from v4"
            )));
        }
        components.push(surface_component(
            format!("ordinary_{id}"),
            "operational_ordinary_constructor",
            Bc1Disposition::ReplayedOperationalConstructor,
            "sealed_operational_registry_only",
            true,
            true,
            None,
        ));
    }
    for row in &trace_rows {
        components.push(surface_component(
            row.component_id.clone(),
            "trace_bound_signature",
            Bc1Disposition::ReplayedTraceSeed,
            "exact_sealed_stage_and_predecessor_signature",
            true,
            true,
            None,
        ));
    }
    let endpoint_set = grammar
        .g7_induction
        .public_endpoint_constructor_inventory
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let prior_public_set = class_v4
        .cubical
        .observed_public_constructors
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    for kind in CubicalConstructorKind::ALL {
        let id = serialized_identifier(&kind)?;
        if !endpoint_set.contains(&id) && !prior_public_set.contains(&id) {
            return Err(Bc1Error::Invariant(format!(
                "cubical constructor {id} has no public induction source"
            )));
        }
        components.push(surface_component(
            format!("cubical_{id}"),
            "public_cubical_constructor",
            Bc1Disposition::ReplayedCubicalConstructor,
            "public_twelve_form_endpoint_aware_induction",
            true,
            true,
            None,
        ));
    }
    for former in &expected_formers {
        let id = serialized_identifier(former)?;
        components.push(surface_component(
            format!("transparent_former_{id}"),
            "transparent_former_rule",
            Bc1Disposition::ReplayedTransparentFormerRule,
            "term_level_internal_closure_over_replayed_internal_premises",
            true,
            true,
            None,
        ));
    }
    components.push(surface_component(
        "dependent_ambient_context_rule".to_owned(),
        "dependent_context_rule",
        Bc1Disposition::AdoptedDependentContextRule,
        "sequential_typed_substitution_and_total_specialization",
        true,
        true,
        None,
    ));

    let impossibility_specs = [
        (
            "pathcon_as_transparent_former",
            "charged_PathCon_is_not_zero_credit_transparent_closure",
        ),
        (
            "candidate_declared_formation_as_transparent_former",
            "candidate_declared_formation_is_charged",
        ),
        (
            "uncertified_premise_transparent_closure",
            "transparent_closure_requires_replayed_Internal_premises",
        ),
        (
            "candidate_fresh_application_head_transparent_closure",
            "candidate_fresh_application_head_is_excluded",
        ),
        (
            "unguarded_ambient_parameter_transparent_closure",
            "guarded_inverse_evidence_is_required",
        ),
    ];
    let explicit_impossibilities = impossibility_specs
        .into_iter()
        .map(|(id, gap)| {
            surface_component(
                id.to_owned(),
                "transparent_former_exclusion",
                Bc1Disposition::NamedImpossible,
                "adopted_transparent_charged_partition",
                true,
                true,
                Some(gap),
            )
        })
        .collect::<Vec<_>>();

    let stock_count = ordinary_count + trace_count + cubical_count + former_count + dependent_count;
    if components.len() != stock_count {
        return Err(Bc1Error::Invariant(
            "frozen component stock lost or duplicated a row".to_owned(),
        ));
    }
    let depth_one_count = ordinary_count + trace_count;
    let depth_one_derivation = tagged_hash(
        "standalone-depth-one-reconstruction",
        &(
            grammar.result_digest.as_str(),
            &trace_rows,
            &class_v4.ordinary.aggregate_derivation_hash,
            &a3_live_replay_errors,
        ),
    );
    let depth_one_regression = Bc1DepthOneRegression {
        grammar_result_digest: grammar.result_digest.clone(),
        archived_grammar_bytes_authenticated: true,
        archived_live_definition_replay_valid: false,
        archived_live_replay_errors: a3_live_replay_errors,
        trace_signature_count: structural_number(trace_count),
        ordinary_constructor_count: structural_number(ordinary_count),
        standalone_depth_one_component_count: structural_number(depth_one_count),
        trace_rows,
        every_trace_row_rederived_from_sealed_trace: true,
        every_ordinary_constructor_normalized_and_natural: true,
        endpoint_publication_complete: true,
        p6_is_derived_closure_with_zero_extra_exports: grammar
            .p6_synthesis
            .p6_used_as_derived_closure_only
            && grammar.p6_synthesis.extra_p6_primitive_exports == 0,
        no_unit_coherence_generator_issued: !grammar.g6_unit_coherence.generator_issued,
        standalone_depth_one_campaign_rederived: true,
        depth_two_restriction_map_defined: false,
        depth_one_rederived_as_depth_two_restriction: false,
        restriction_named_gap: BC1_DEPTH_ONE_RESTRICTION_GAP.to_owned(),
        derivation_hash: depth_one_derivation,
    };
    let surface_derivation = tagged_hash(
        "frozen-component-stock",
        &(
            &components,
            &explicit_impossibilities,
            stock_count,
            "not_an_independent_depth_two_semantic_domain",
        ),
    );
    let surface = Bc1FrozenComponentSurface {
        ordinary_component_count: structural_number(ordinary_count),
        trace_component_count: structural_number(trace_count),
        cubical_component_count: structural_number(cubical_count),
        transparent_former_component_count: structural_number(former_count),
        dependent_context_rule_count: structural_number(dependent_count),
        frozen_component_stock_count: structural_number(stock_count),
        explicit_impossibility_count: structural_number(explicit_impossibilities.len()),
        components,
        explicit_impossibilities,
        every_frozen_component_has_explicit_disposition: true,
        every_explicit_impossibility_replayed: true,
        component_stock_is_independent_depth_two_semantic_domain: false,
        component_stock_used_as_completeness_definition: false,
        derivation_hash: surface_derivation,
    };
    Ok((depth_one_regression, surface))
}

fn obligation(
    name: &str,
    status: Bc1ObligationStatus,
    exact_scope: &str,
    named_gap: Option<&str>,
) -> Bc1Obligation {
    let derivation_hash = tagged_hash("bc1-obligation", &(name, status, exact_scope, named_gap));
    Bc1Obligation {
        obligation: name.to_owned(),
        status,
        exact_scope: exact_scope.to_owned(),
        named_gap: named_gap.map(ToOwned::to_owned),
        register: Bc1Register::SemanticRegisterAuthority,
        derivation_hash,
    }
}

fn build_depth_two_attempt() -> Result<Bc1DepthTwoAttempt, Bc1Error> {
    let basis =
        issue_e4_generator_basis_audit().map_err(|error| Bc1Error::Invariant(error.to_string()))?;
    replay_e4_generator_basis_audit(&basis)
        .map_err(|error| Bc1Error::Invariant(error.to_string()))?;
    if !basis.current_e1_context_morphism_decomposition_complete()
        || basis.full_schema2_generator_completeness()
        || basis.independent_membership_issuance_enabled()
    {
        return Err(Bc1Error::Invariant(
            "typed context-morphism decomposition scope changed".to_owned(),
        ));
    }
    let generator_kinds = E4GeneratorKind::ALL
        .iter()
        .map(serialized_identifier)
        .collect::<Result<Vec<_>, _>>()?;
    let dependent_adjudication = utf8(
        "docs/dependent_context_adjudication.md",
        DEPENDENT_CONTEXT_BYTES,
    )?;
    if !dependent_adjudication.contains("Status:** **ADOPTED**")
        || !dependent_adjudication.contains("dependent-ambient-context-v1")
        || !dependent_adjudication.contains("sequential typed substitution")
        || !dependent_adjudication.contains("used whole, minting")
        || !dependent_adjudication.contains("> nothing.")
    {
        return Err(Bc1Error::Input(
            "dependent-context adoption markers are absent".to_owned(),
        ));
    }
    let global: Value = serde_json::from_slice(GLOBAL_E4_BYTES)
        .map_err(|error| Bc1Error::Json(error.to_string()))?;
    let a3: Value = serde_json::from_slice(A3_EXHAUSTIVENESS_BYTES)
        .map_err(|error| Bc1Error::Json(error.to_string()))?;
    let wrapped_exhaustion = global
        .get("wrapped_domain_exhaustion")
        .ok_or_else(|| Bc1Error::Json("global E-4 lacks wrapped-domain evidence".to_owned()))?;
    let global_replayed = json_string(&global, "result_digest")?
        == EXPECTED_GLOBAL_E4_RESULT_DIGEST
        && json_bool(wrapped_exhaustion, "domain_finite")?
        && json_bool(wrapped_exhaustion, "wrapped_classifier_partition_total")?
        && json_bool(
            wrapped_exhaustion,
            "every_contextual_obstruction_has_non_unknown_disposition",
        )?
        && json_bool(&global, "no_unknown_survives")?
        && json_bool(&global, "class_exhaustion_proved")?
        && json_bool(&global, "global_e4_complete")?;
    let a3_relative = json_string(&a3, "result_digest")? == EXPECTED_A3_RESULT_DIGEST
        && json_bool(
            &a3,
            "relative_rule_constructor_inventory_exhaustiveness_proved",
        )?;
    let a3_absolute = json_bool(&a3, "broader_absolute_semantic_exhaustiveness_claimed")?;
    if !global_replayed || !a3_relative || a3_absolute {
        return Err(Bc1Error::Input(
            "scoped exhaustion predecessors changed".to_owned(),
        ));
    }
    let obligations = vec![
        obligation(
            "independently_define_the_intended_depth_two_semantic_signature_surface",
            Bc1ObligationStatus::OpenNamedGap,
            "Sem2_Marg2_and_SLNF_must_be_independent_of_the_generator_closure",
            Some(BC1_INDEPENDENT_SURFACE_GAP),
        ),
        obligation(
            "give_every_depth_two_constructor_a_total_typed_semantic_interpretation",
            Bc1ObligationStatus::OpenNamedGap,
            "the_frozen_stock_has_typed_components_but_no_total_composite_domain",
            Some(BC1_INDEPENDENT_SURFACE_GAP),
        ),
        obligation(
            "prove_trace_and_adopted_former_coverage_of_the_independent_surface",
            Bc1ObligationStatus::OpenNamedGap,
            "coverage_cannot_quantify_before_the_independent_surface_exists",
            Some(BC1_INDEPENDENT_SURFACE_GAP),
        ),
        obligation(
            "decompose_already_typed_context_morphisms_into_the_registered_E4_generator_basis",
            Bc1ObligationStatus::ProvedScoped,
            "current_E1_typed_context_morphism_language",
            None,
        ),
        obligation(
            "replay_the_transparent_former_matrix",
            Bc1ObligationStatus::ProvedScoped,
            "registered_term_level_Internal_premises_only",
            None,
        ),
        obligation(
            "replay_dependent_context_sequential_substitution_and_totality",
            Bc1ObligationStatus::ProvedScoped,
            "registered_dependent_context_declarations",
            None,
        ),
        obligation(
            "prove_class_indexed_generator_induction_over_every_depth_two_schema",
            Bc1ObligationStatus::OpenNamedGap,
            "no_independent_depth_two_domain_exists_for_the_induction_motive",
            Some(BC1_GENERATOR_INDUCTION_GAP),
        ),
        obligation(
            "derive_the_depth_one_campaign_as_the_depth_one_restriction",
            Bc1ObligationStatus::OpenNamedGap,
            "the_standalone_campaign_replays_but_the_restriction_map_is_unformable",
            Some(BC1_DEPTH_ONE_RESTRICTION_GAP),
        ),
        obligation(
            "preserve_zero_charge_for_grammar_closure_and_hypotheses",
            Bc1ObligationStatus::ProvedScoped,
            "endpoint_premises_transparent_closure_and_dependent_hypotheses",
            None,
        ),
    ];
    let derivation_hash = tagged_hash(
        "depth-two-attempt",
        &(
            &generator_kinds,
            &obligations,
            basis.derivation_hash(),
            global_replayed,
            a3_relative,
            a3_absolute,
        ),
    );
    Ok(Bc1DepthTwoAttempt {
        quantifier: Bc1Quantifier::FrozenWrappedSurface,
        register: Bc1Register::SemanticRegisterAuthority,
        independently_defined_semantic_domain: false,
        total_typed_interpretation_on_independent_domain: false,
        trace_and_adopted_former_coverage_proved: false,
        typed_context_morphism_decomposition_replayed: true,
        generator_kind_count: structural_number(generator_kinds.len()),
        generator_kinds,
        transparent_former_matrix_replayed_at_registered_scope: true,
        dependent_context_rule_replayed_at_registered_scope: true,
        transparent_and_dependent_rules_bound_to_independent_domain: false,
        class_indexed_depth_two_generator_induction_proved: false,
        depth_one_restriction_theorem_proved: false,
        zero_charge_law_preserved: true,
        a3_relative_exhaustiveness_replayed: true,
        a3_absolute_semantic_exhaustiveness_claimed: false,
        global_e4_wrapped_candidate_exhaustion_replayed: true,
        global_e4_used_as_semantic_domain_exhaustiveness: false,
        obligation_count: metadata_number(obligations.len()),
        obligations,
        every_obligation_has_explicit_disposition: true,
        silent_residue_claimed_absent_without_domain: false,
        f_bc5_expressivity_gap_fired: true,
        c2_closed: false,
        derivation_hash,
    })
}

fn named_gap(id: &str, phase: &str, exact_obstruction: &str) -> Bc1NamedGap {
    let derivation_hash = tagged_hash(
        "bc1-named-gap",
        &(id, phase, exact_obstruction, "zero_promotion"),
    );
    Bc1NamedGap {
        id: id.to_owned(),
        phase: phase.to_owned(),
        quantifier: Bc1Quantifier::FrozenWrappedSurface,
        register: Bc1Register::SemanticRegisterAuthority,
        exact_obstruction: exact_obstruction.to_owned(),
        keeps_c2_open: true,
        keeps_rows_unpromoted: true,
        derivation_hash,
    }
}

fn certificate_digest(certificate: &Bc1DepthTwoCompletenessV1Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("bc1-depth-two-completeness-certificate", &projection)
}

fn build_certificate() -> Result<Bc1DepthTwoCompletenessV1Certificate, Bc1Error> {
    authenticate_sealed_inputs()?;
    let (m3_v1_baseline, _) = build_m3_baseline()?;
    let (depth_one_regression, frozen_component_surface) = build_depth_one_and_surface()?;
    let depth_two_attempt = build_depth_two_attempt()?;
    let open_gaps = vec![
        named_gap(
            BC1_INDEPENDENT_SURFACE_GAP,
            "BC-1 semantic-domain gate",
            "The adopted ledger independently enumerates trace-bound seeds and proves scoped constructor rules, but it does not define an inductive Sem2/Marg2/SLNF depth-two signature surface or a total typed interpretation of every composite. Defining the intended domain to equal generator closure would make completeness circular and is forbidden.",
        ),
        named_gap(
            BC1_GENERATOR_INDUCTION_GAP,
            "BC-1 generator completeness",
            "The registered typed context-morphism generators decompose morphisms already admitted by the current E1 language, but there is no class-indexed structural induction proving coverage for every member of an independently defined depth-two semantic domain.",
        ),
        named_gap(
            BC1_DEPTH_ONE_RESTRICTION_GAP,
            "BC-1 regression",
            "Every sealed depth-one trace row re-elaborates exactly and the ordinary registry replays, but without a depth-two semantic datatype there is no restriction function whose image can be proved equal to that campaign.",
        ),
    ];
    let mut certificate = Bc1DepthTwoCompletenessV1Certificate {
        schema: BC1_DEPTH_TWO_COMPLETENESS_V1_SCHEMA.to_owned(),
        date: BC1_DEPTH_TWO_COMPLETENESS_V1_DATE.to_owned(),
        syntax_identifier_policy: "Numeric substrings in dates, artifact versions, stages, clauses, theorem names, and gap IDs are syntax identifiers rather than quantitative evidence.".to_owned(),
        syntax_identifier_register: Bc1Register::SyntaxIdentifier,
        source_bindings: source_bindings(),
        m3_v1_baseline,
        depth_one_regression,
        frozen_component_surface,
        depth_two_attempt,
        open_gap_count: metadata_number(open_gaps.len()),
        open_gaps,
        c2_closed: false,
        no_promotions_made: true,
        promoted_row_count: metadata_number(0),
        new_semantic_family_credits: semantic_family_number(0),
        no_value_bar_hash_or_enumeration_order_selector: true,
        m3_v2_prerequisite_satisfied: false,
        bc1_status: Bc1RunStatus::StoppedNamedGaps,
        mutation_falsifiers: vec![
            "change_any_bound_input_byte_or_digest_then_replay_must_fail".to_owned(),
            "shift_any_M3_v1_baseline_condition_or_enacted_regression_then_replay_must_fail"
                .to_owned(),
            "change_or_remove_any_depth_one_trace_row_then_replay_must_fail".to_owned(),
            "treat_the_frozen_component_stock_as_the_independent_semantic_domain_then_replay_must_fail"
                .to_owned(),
            "remove_any_component_or_impossibility_disposition_then_replay_must_fail".to_owned(),
            "promote_context_morphism_decomposition_to_depth_two_generator_completeness_then_replay_must_fail"
                .to_owned(),
            "claim_the_depth_one_restriction_without_a_depth_two_domain_then_replay_must_fail"
                .to_owned(),
            "remove_or_rename_any_BC1_gap_then_replay_must_fail".to_owned(),
            "mint_any_family_credit_or_promote_any_row_then_replay_must_fail".to_owned(),
            "retag_any_published_quantity_or_introduce_a_selector_then_replay_must_fail"
                .to_owned(),
            "add_any_unknown_JSON_field_then_replay_must_fail".to_owned(),
        ],
        permitted_conclusion: "The M-3 v1 baseline is unchanged. The sealed depth-one trace campaign re-derives exactly, every frozen ordinary/cubical/transparent/dependent component has an explicit scoped disposition, and grammar closure mints no semantic-family credit. These scoped results do not constitute an independently defined depth-two semantic domain or a noncircular generator-completeness theorem. C2 remains open and no row is promoted.".to_owned(),
        required_successor_action: "Adjudicate or construct an independent inductive depth-two semantic signature domain with total typed interpretation, without defining it by the generator closure. Then prove trace/adopted-former coverage, the class-indexed generator induction, and the depth-one restriction theorem in a versioned successor before rerunning M-3.".to_owned(),
        result_digest: String::new(),
    };
    if certificate.open_gaps.len() != 3
        || certificate.c2_closed
        || !certificate.no_promotions_made
        || certificate.promoted_row_count.value != 0
        || certificate.new_semantic_family_credits.value != 0
        || certificate.m3_v2_prerequisite_satisfied
        || certificate
            .frozen_component_surface
            .component_stock_used_as_completeness_definition
        || certificate
            .depth_two_attempt
            .class_indexed_depth_two_generator_induction_proved
        || certificate
            .depth_one_regression
            .depth_one_rederived_as_depth_two_restriction
    {
        return Err(Bc1Error::Invariant(
            "BC-1 fail-closed exit invariant failed".to_owned(),
        ));
    }
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

static EXPECTED_CERTIFICATE: OnceLock<Result<Bc1DepthTwoCompletenessV1Certificate, String>> =
    OnceLock::new();

fn expected_certificate() -> Result<&'static Bc1DepthTwoCompletenessV1Certificate, Bc1Error> {
    match EXPECTED_CERTIFICATE
        .get_or_init(|| build_certificate().map_err(|error| error.to_string()))
    {
        Ok(certificate) => Ok(certificate),
        Err(error) => Err(Bc1Error::Input(error.clone())),
    }
}

pub fn issue_bc1_depth_two_completeness_v1()
-> Result<Bc1DepthTwoCompletenessV1Certificate, Bc1Error> {
    expected_certificate().cloned()
}

fn invalid_replay(error: impl Into<String>) -> Bc1Replay {
    Bc1Replay {
        valid: false,
        errors: vec![error.into()],
        bc1_status: Bc1RunStatus::StoppedNamedGaps,
        c2_closed: false,
        no_promotions_made: true,
        m3_v2_prerequisite_satisfied: false,
    }
}

pub fn replay_bc1_depth_two_completeness_v1(
    claimed: &Bc1DepthTwoCompletenessV1Certificate,
) -> Bc1Replay {
    let expected = match expected_certificate() {
        Ok(certificate) => certificate,
        Err(error) => return invalid_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("BC-1 result digest mismatch".to_owned());
    }
    if claimed.source_bindings != source_bindings() {
        errors.push("BC-1 source bindings drifted".to_owned());
    }
    if claimed != expected {
        errors.push("BC-1 certificate differs from deterministic reissuance".to_owned());
    }
    Bc1Replay {
        valid: errors.is_empty(),
        errors,
        bc1_status: claimed.bc1_status,
        c2_closed: claimed.c2_closed,
        no_promotions_made: claimed.no_promotions_made,
        m3_v2_prerequisite_satisfied: claimed.m3_v2_prerequisite_satisfied,
    }
}

pub fn replay_bc1_depth_two_completeness_v1_json(json: &str) -> Bc1Replay {
    match serde_json::from_str::<Bc1DepthTwoCompletenessV1Certificate>(json) {
        Ok(certificate) => replay_bc1_depth_two_completeness_v1(&certificate),
        Err(error) => invalid_replay(format!("invalid BC-1 JSON: {error}")),
    }
}

fn register_label(register: Bc1Register) -> &'static str {
    match register {
        Bc1Register::SemanticRegisterAuthority => "semantic_register_authority",
        Bc1Register::SemanticFamilyAuthority => "semantic_family_authority",
        Bc1Register::StructuralTestimony => "structural_testimony",
        Bc1Register::ArtifactMetadata => "artifact_metadata",
        Bc1Register::SyntaxIdentifier => "syntax_identifier",
    }
}

fn render_number(number: &Bc1TaggedNumber) -> String {
    format!(
        "{} [register: `{}`]",
        number.value,
        register_label(number.register)
    )
}

pub fn render_bc1_depth_two_completeness_v1(
    certificate: &Bc1DepthTwoCompletenessV1Certificate,
) -> String {
    let mut out = String::new();
    out.push_str("# BC-1 depth-two completeness result\n\n");
    out.push_str(&format!(
        "**Date (syntax identifier):** {}. **Status:** `stopped_named_gaps`. **Certificate:** `{}`.\n\n",
        certificate.date, certificate.result_digest
    ));
    out.push_str("Labels in BC-1, M-3, E-1/E-4/E-7, Schema2, stage/clause identifiers, and artifact versions are syntax identifiers. Every quantitative statement below carries its register explicitly.\n\n");
    out.push_str("## Frozen baseline\n\n");
    out.push_str(&format!(
        "M-3 v1 replays without a baseline shift. Its proved bridge conditions remain **{}** of **{}**, the enacted E-5/BI-2 regression reproduces exactly, and the baseline makes no promotion.\n\n",
        render_number(&certificate.m3_v1_baseline.proved_bridge_condition_count),
        render_number(&certificate.m3_v1_baseline.bridge_condition_count),
    ));
    out.push_str("## What re-derived\n\n");
    out.push_str(&format!(
        "The standalone depth-one campaign re-derives **{}** trace signatures and **{}** ordinary constructors, for **{}** scoped depth-one components. The frozen component stock additionally contains **{}** public cubical constructors, **{}** transparent-former rules, and **{}** dependent-context rule. Every component and each of the **{}** registered impossibilities has an explicit disposition.\n\n",
        render_number(&certificate.depth_one_regression.trace_signature_count),
        render_number(&certificate.depth_one_regression.ordinary_constructor_count),
        render_number(&certificate.depth_one_regression.standalone_depth_one_component_count),
        render_number(&certificate.frozen_component_surface.cubical_component_count),
        render_number(&certificate.frozen_component_surface.transparent_former_component_count),
        render_number(&certificate.frozen_component_surface.dependent_context_rule_count),
        render_number(&certificate.frozen_component_surface.explicit_impossibility_count),
    ));
    out.push_str(&format!(
        "The grammar-completion artifact's live source drift remains disclosed as sealed testimony: `{}`. BC-1 reconstructs the exact trace rows directly and does not repair or conceal that drift.\n\n",
        certificate
            .depth_one_regression
            .archived_live_replay_errors
            .join("; ")
    ));
    out.push_str("## Why C2 does not close\n\n");
    out.push_str("The component stock is not promoted into an independently defined semantic domain. Doing so by definition would identify the intended domain with generator closure and make generator completeness circular. The typed context-morphism decomposer, transparent-former matrix, dependent-context rule, A3 relative theorem, and wrapped-candidate exhaustion all retain their narrower scopes.\n\n");
    out.push_str("| Named gap | Exact obstruction |\n|---|---|\n");
    for gap in &certificate.open_gaps {
        out.push_str(&format!("| `{}` | {} |\n", gap.id, gap.exact_obstruction));
    }
    out.push_str(&format!(
        "\nPromoted rows: **{}**. New semantic-family credits: **{}**. C2 closed: **{}**. M-3 v2 prerequisite satisfied: **{}**.\n\n",
        render_number(&certificate.promoted_row_count),
        render_number(&certificate.new_semantic_family_credits),
        certificate.c2_closed,
        certificate.m3_v2_prerequisite_satisfied,
    ));
    out.push_str("## Permitted conclusion\n\n");
    out.push_str(&certificate.permitted_conclusion);
    out.push_str("\n\n## Required successor\n\n");
    out.push_str(&certificate.required_successor_action);
    out.push('\n');
    out
}

pub fn emit_bc1_depth_two_completeness_v1_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Bc1DepthTwoCompletenessV1Certificate, Bc1Error> {
    if certificate_path.exists() || report_path.exists() {
        return Err(Bc1Error::Io(
            "create-new target already exists; no artifact was overwritten".to_owned(),
        ));
    }
    let certificate = issue_bc1_depth_two_completeness_v1()?;
    let replay = replay_bc1_depth_two_completeness_v1(&certificate);
    if !replay.valid {
        return Err(Bc1Error::Invariant(format!(
            "new BC-1 certificate did not replay: {}",
            replay.errors.join("; ")
        )));
    }
    let json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Bc1Error::Json(error.to_string()))?;
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(certificate_path)
        .map_err(|error| Bc1Error::Io(error.to_string()))?;
    let certificate_write = certificate_file
        .write_all(&json)
        .and_then(|_| certificate_file.write_all(b"\n"));
    drop(certificate_file);
    if let Err(error) = certificate_write {
        let _ = remove_file(certificate_path);
        return Err(Bc1Error::Io(error.to_string()));
    }
    let report = render_bc1_depth_two_completeness_v1(&certificate);
    let report_write = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(report_path)
        .and_then(|mut file| file.write_all(report.as_bytes()));
    if let Err(error) = report_write {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(Bc1Error::Io(error.to_string()));
    }
    let emitted = read_to_string(certificate_path).map_err(|error| {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        Bc1Error::Io(error.to_string())
    })?;
    let emitted_replay = replay_bc1_depth_two_completeness_v1_json(&emitted);
    if !emitted_replay.valid {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(Bc1Error::Invariant(format!(
            "emitted BC-1 JSON did not replay: {}",
            emitted_replay.errors.join("; ")
        )));
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bc1_replays_the_baseline_and_stops_with_exact_gaps() {
        let certificate = issue_bc1_depth_two_completeness_v1().expect("BC-1 issues");
        let replay = replay_bc1_depth_two_completeness_v1(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(certificate.bc1_status, Bc1RunStatus::StoppedNamedGaps);
        assert!(!certificate.c2_closed);
        assert!(certificate.no_promotions_made);
        assert_eq!(certificate.promoted_row_count.value, 0);
        assert_eq!(certificate.new_semantic_family_credits.value, 0);
        assert!(!certificate.m3_v2_prerequisite_satisfied);
        assert_eq!(
            certificate
                .open_gaps
                .iter()
                .map(|gap| gap.id.as_str())
                .collect::<Vec<_>>(),
            vec![
                BC1_INDEPENDENT_SURFACE_GAP,
                BC1_GENERATOR_INDUCTION_GAP,
                BC1_DEPTH_ONE_RESTRICTION_GAP,
            ]
        );
    }

    #[test]
    fn depth_one_regression_and_component_dispositions_are_complete() {
        let certificate = issue_bc1_depth_two_completeness_v1().expect("BC-1 issues");
        assert!(
            certificate
                .depth_one_regression
                .every_trace_row_rederived_from_sealed_trace
        );
        assert!(
            certificate
                .depth_one_regression
                .standalone_depth_one_campaign_rederived
        );
        assert!(
            !certificate
                .depth_one_regression
                .depth_one_rederived_as_depth_two_restriction
        );
        assert!(
            certificate
                .frozen_component_surface
                .every_frozen_component_has_explicit_disposition
        );
        assert!(
            !certificate
                .frozen_component_surface
                .component_stock_used_as_completeness_definition
        );
        assert!(
            certificate
                .depth_two_attempt
                .typed_context_morphism_decomposition_replayed
        );
        assert!(
            !certificate
                .depth_two_attempt
                .class_indexed_depth_two_generator_induction_proved
        );
    }

    #[test]
    fn verdict_gap_restriction_register_and_digest_mutations_fail() {
        let certificate = issue_bc1_depth_two_completeness_v1().expect("BC-1 issues");

        let mut verdict = certificate.clone();
        verdict.c2_closed = true;
        assert!(!replay_bc1_depth_two_completeness_v1(&verdict).valid);

        let mut promotion = certificate.clone();
        promotion.promoted_row_count.value = 1;
        assert!(!replay_bc1_depth_two_completeness_v1(&promotion).valid);

        let mut credit = certificate.clone();
        credit.new_semantic_family_credits.value = 1;
        assert!(!replay_bc1_depth_two_completeness_v1(&credit).valid);

        let mut circular = certificate.clone();
        circular
            .frozen_component_surface
            .component_stock_used_as_completeness_definition = true;
        assert!(!replay_bc1_depth_two_completeness_v1(&circular).valid);

        let mut restriction = certificate.clone();
        restriction
            .depth_one_regression
            .depth_one_rederived_as_depth_two_restriction = true;
        assert!(!replay_bc1_depth_two_completeness_v1(&restriction).valid);

        let mut gap = certificate.clone();
        gap.open_gaps[0].id.push_str("_MUTATED");
        assert!(!replay_bc1_depth_two_completeness_v1(&gap).valid);

        let mut register = certificate.clone();
        register
            .frozen_component_surface
            .trace_component_count
            .register = Bc1Register::SemanticFamilyAuthority;
        assert!(!replay_bc1_depth_two_completeness_v1(&register).valid);

        let mut source = certificate.clone();
        source.source_bindings[0].sha256.push('0');
        assert!(!replay_bc1_depth_two_completeness_v1(&source).valid);

        let mut digest = certificate;
        digest.result_digest.push('0');
        assert!(!replay_bc1_depth_two_completeness_v1(&digest).valid);
    }

    #[test]
    fn unknown_json_fields_fail_closed() {
        let certificate = issue_bc1_depth_two_completeness_v1().expect("BC-1 issues");
        let mut value = serde_json::to_value(certificate).expect("serialize");
        value
            .as_object_mut()
            .expect("certificate object")
            .insert("unknown_field".to_owned(), Value::Bool(true));
        let replay = replay_bc1_depth_two_completeness_v1_json(
            &serde_json::to_string(&value).expect("JSON"),
        );
        assert!(!replay.valid);
    }
}
