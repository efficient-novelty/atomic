//! T-D2-1 v2: operational depth-two semantic-domain construction.
//!
//! The public data model is intentionally declared separately from the
//! create-new issuer below.  T-D2-2 consumes this surface without depending
//! on the private enumeration representation.

use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_type::ambient_former_internality::registered_transparent_formers;
use pen_type::elaborate::{
    DerivationNode, KernelTy, SealedSignature, elaborate_single_clause_with_typed_ambient,
    elaborate_telescope,
};
use pen_type::equality::{KERNEL_EQUALITY_PROCEDURE, univalent_equality};
use pen_type::normalize::{normalize, whnf};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{OpenOptions, read_to_string, remove_file};
use std::io::Write;
use std::path::Path;
use std::sync::OnceLock;
use thiserror::Error;

pub const T_D2_1_OPERATIONAL_DOMAIN_V2_SCHEMA: &str = "t-d2-1-operational-domain-build-v2";
pub const T_D2_1_OPERATIONAL_DOMAIN_V2_DATE: &str = "2026-07-24";

const SEMANTIC_FORMER_DEPTH_BOUND: u32 = 2;
const EXPECTED_PUBLIC_ENTRY_COUNT: u32 = 15;
const EXPECTED_PUBLIC_CLAUSE_COUNT: u32 = 64;
const EXPECTED_MAXIMUM_AMBIENT_PARAMETERS: u32 = 2;
const EXPECTED_MAXIMUM_FREE_SCOPE_LENGTH: u32 = 9;
const EXPECTED_MAXIMUM_PATH_DIMENSION: u32 = 3;
const EXPECTED_PROVED_FAMILY_COUNT: u32 = 61;
const EXPECTED_MEMBERSHIP_ROW_COUNT: u32 = 89;

const OPERATIONAL_ADJUDICATION_SHA256: &str =
    "bec3df54b782371f496dc9912f491c8f8ab5cf796ace59a5e1c852aaa998b77d";
const PARENT_ADJUDICATION_SHA256: &str =
    "fffd48c0eb5f89b6728842aa6f9de5aa43013ab867278d6b8ec6b2737ed92151";
const AMBIENT_FORMER_ADJUDICATION_SHA256: &str =
    "ca53cac5330999620b03863abc00378e1a2da556989d57414262278f47473fc2";
const DEPENDENT_CONTEXT_ADJUDICATION_SHA256: &str =
    "62d7f33b2504591b52de6897f06ae4699b237dcf91b02b3aee979ed3d16ab523";
const QUOTIENT_ADJUDICATION_SHA256: &str =
    "ba9b57379e4b53c9f1ef54982e7c4551754ad82262d11805dc6e7c752f66b897";
const OPEN_PROBLEM_SHA256: &str =
    "fe3b2eb8f17f12563596d7a937d39d2618625503cf4ba1af8bba33e224309c82";
const EXPR_SOURCE_SHA256: &str = "f4da000f35a343fe6ad749c36e12d56330c8787efc73bb9c01a0289f65ea2531";
const ELABORATE_SOURCE_SHA256: &str =
    "6b713bb5286648a1c7a22c55a2446af8eca58223343ce2cd716c49748e447719";
const AMBIENT_FORMER_SOURCE_SHA256: &str =
    "ec151b8d46e42fe691acb6db065411fcae344e29283cdd7dd8f72e2e0889ef08";
const CONTEXT_SOURCE_SHA256: &str =
    "a89a1b5007222d046d43f4df97d2a5411facea5f4bc0d709d7dca12dacf9c260";
const NORMALIZE_SOURCE_SHA256: &str =
    "7d3398b60708bf1ac132609e0bf84d6a26383f9dcc90f11f99d5803ed9da73f2";
const EQUALITY_SOURCE_SHA256: &str =
    "3aa5fd8d4e9667c1534f39fc0c76ef956e6372301b767f2442c918c6b85c3401";
const FAMILY_CORPUS_SHA256: &str =
    "51419e53f9dcd0f9c61ddb21c0c116ab92de05d5b5e142792535eeda45136a18";
const A3_CORPUS_SHA256: &str = "73d5d25cf9a91ab82100f14bb1e1cfe424c97ab0ac2503664278df4268ac5337";
const MEMBERSHIP_CORPUS_SHA256: &str =
    "c60a28c82370e88829dfd6457ae52814d4523fcf2b2012378814a4cf0dd32be5";

const OPERATIONAL_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_operational_domain_adjudication.md");
const PARENT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/depth_two_domain_adjudication.md");
const AMBIENT_FORMER_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/ambient_former_closure_adjudication.md");
const DEPENDENT_CONTEXT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/dependent_context_adjudication.md");
const QUOTIENT_ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/e2_quotient_adjudications.md");
const OPEN_PROBLEM_BYTES: &[u8] =
    include_bytes!("../../../docs/step_15_completion_open_problem.md");
const EXPR_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-core/src/expr.rs");
const ELABORATE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/elaborate.rs");
const AMBIENT_FORMER_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/ambient_former_internality.rs");
const CONTEXT_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-schema/src/context.rs");
const NORMALIZE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/normalize.rs");
const EQUALITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/equality.rs");
const FAMILY_CORPUS_BYTES: &[u8] =
    include_bytes!("../../../docs/t_bi_nu1_semantic_provenance_v6.json");
const A3_CORPUS_BYTES: &[u8] =
    include_bytes!("../../../docs/a3_rule_inventory_exhaustiveness_v2.json");
const MEMBERSHIP_CORPUS_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_e5_future_hole_finale_v2_dependent_context.json");

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td21V2EvidenceRegister {
    DefinitionInput,
    BoundCitation,
    CarrierSeal,
    SemanticFamilyAuthority,
    StructuralTestimony,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td21V2RunStatus {
    Passed,
    StoppedNamedGaps,
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td21V2SemanticDepth {
    ResidentZero,
    FormerOne,
    FormerTwo,
}

impl Td21V2SemanticDepth {
    pub const fn value(self) -> u32 {
        match self {
            Self::ResidentZero => 0,
            Self::FormerOne => 1,
            Self::FormerTwo => 2,
        }
    }
}

#[derive(Copy, Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td21V2PublicFormer {
    AmbientUniverse,
    LambdaIntroduction,
    Application,
    PiFormation,
    SigmaFormation,
    IdentityFormation,
    ReflexivityIntroduction,
    SuspensionFormation,
    TruncationFormation,
    FlatFormation,
    SharpFormation,
    DiscreteFormation,
    ShapeFormation,
    NextFormation,
    EventuallyFormation,
    PathConstructor,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2SyntaxNumber {
    pub value: u32,
    pub bound_name: String,
}

/// Serializable mirror of the frozen kernel classifier used by an explicit
/// parameter telescope.  A context is part of a judgment: it is never
/// reconstructed from the number of leaf occurrences in the term.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "classifier")]
pub enum Td21V2KernelClassifier {
    Type,
    Element {
        type_expression: Expr,
    },
    Function {
        domain: Box<Td21V2KernelClassifier>,
        codomain: Box<Td21V2KernelClassifier>,
    },
    PathDeclaration {
        dimension: u32,
    },
    Neutral,
}

impl Td21V2KernelClassifier {
    fn to_kernel_ty(&self) -> KernelTy {
        match self {
            Self::Type => KernelTy::Type,
            Self::Element { type_expression } => KernelTy::El(type_expression.clone()),
            Self::Function { domain, codomain } => KernelTy::Fun(
                Box::new(domain.to_kernel_ty()),
                Box::new(codomain.to_kernel_ty()),
            ),
            Self::PathDeclaration { dimension } => KernelTy::PathDecl {
                dimension: *dimension,
            },
            Self::Neutral => KernelTy::Neutral,
        }
    }
}

/// Exact sequential parameter telescope for a contextual judgment.
///
/// `Element { type_expression }` entries are replay-checked against the
/// preceding prefix.  `Type` entries are the universe-classified parameters
/// used by the historical nondependent surface.  The remaining classifier
/// forms are retained in the data model but currently fail closed because the
/// frozen public kernel API has no declaration checker for them.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2ParameterTelescope {
    pub hypotheses: Vec<Td21V2KernelClassifier>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Td21V2ConstructiveRule {
    ResidentClause,
    ContextProjection,
    BoundProjection,
    FormerApplication,
}

/// Evidence required by the adopted natural-family quotient.
///
/// This is intentionally not reducible to a hash of a normal form and
/// classifier.  Until all four proof components are replayable, no
/// `quotient_key` and no depth-two domain membership verdict may issue.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2NaturalFamilyQuotientEvidence {
    pub parameter_telescope_digest: String,
    pub canonical_weakening_maps_checked: bool,
    pub canonical_renaming_maps_checked: bool,
    pub naturality_derivation_replayed: bool,
    pub univalent_equality_witnesses_replayed: bool,
    pub adopted_quotient_established: bool,
    pub exact_obstruction: Option<String>,
    pub derivation_hash: String,
}

/// One constructor-replayable typing/normalization derivation.  Each child
/// records the binder prefix under which it was checked, so a bound variable
/// can never be accepted by replaying the child as a closed term.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2ConstructiveDerivation {
    pub rule: Td21V2ConstructiveRule,
    pub former: Option<Td21V2PublicFormer>,
    pub exact_node: Td21V2OperationalNode,
    pub parameter_telescope_digest: String,
    pub active_binder_classifiers: Vec<Td21V2KernelClassifier>,
    pub parameter_coordinates_used: Vec<u32>,
    pub exact_canonical_presentation: Expr,
    pub kernel_type_json: String,
    pub normal_form: Expr,
    pub semantic_depth: Td21V2SemanticDepth,
    pub kernel_rule: String,
    pub kernel_derivation_hash: String,
    pub normalization_derivation_hash: String,
    pub children: Vec<Td21V2ConstructiveDerivation>,
    pub every_child_context_replayed: bool,
    pub presentation_preserves_coordinate_sharing: bool,
    pub presentation_preserves_resident_content: bool,
    pub typed_and_normalized: bool,
    pub carrier_key: String,
    pub quotient_evidence: Td21V2NaturalFamilyQuotientEvidence,
    pub quotient_key: Option<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2ContextualMembershipDecision {
    pub typed_judgment_accepted: bool,
    pub adopted_quotient_established: bool,
    pub domain_member: bool,
    pub exact_obstruction: Option<String>,
    pub derivation: Option<Td21V2ConstructiveDerivation>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "node")]
pub enum Td21V2OperationalNode {
    ResidentClause {
        entry_identifier: Td21V2SyntaxNumber,
        candidate_digest: String,
        clause_index: Td21V2SyntaxNumber,
        declared_role: String,
        normal_form: Expr,
    },
    ContextVariable {
        parameter: Td21V2SyntaxNumber,
    },
    BoundVariable {
        binder_level: Td21V2SyntaxNumber,
    },
    FormerApplication {
        former: Td21V2PublicFormer,
        arguments: Vec<Td21V2OperationalNode>,
        dimension: Option<Td21V2SyntaxNumber>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2CarrierRepresentative {
    pub carrier_key: String,
    pub quotient_key: String,
    pub semantic_depth: Td21V2SemanticDepth,
    pub node: Td21V2OperationalNode,
    pub normal_form: Expr,
    pub typed: bool,
    pub kernel_type_json: String,
    pub typing_derivation_hash: String,
    pub normalization_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: usize,
    pub blake3: String,
    pub sha256: String,
    pub register: Td21V2EvidenceRegister,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2NamedGap {
    pub id: String,
    pub phase: String,
    pub exact_obstruction: String,
    pub keeps_t_d2_2_closed: bool,
    pub keeps_m4_unauthorized: bool,
    pub zero_charge: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2BoundCitation {
    pub bound_name: String,
    pub value: u32,
    pub exact_source: String,
    pub replayed_before_corpus_access: bool,
    pub resolved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2BoundSnapshot {
    pub public_entry_count: u32,
    pub public_clause_count: u32,
    pub maximum_ambient_parameter_count: u32,
    pub maximum_free_scope_length: u32,
    pub maximum_binder_nesting: u32,
    pub maximum_path_dimension: u32,
    pub semantic_former_depth: u32,
    pub transparent_former_count: u32,
    pub citations: Vec<Td21V2BoundCitation>,
    pub all_citations_resolved_before_corpus_access: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2ContextFinitenessProof {
    pub display_names_erased: bool,
    pub library_symbols_replaced_by_resident_coordinates: bool,
    pub declaration_width_bounded: bool,
    pub cofibration_conjunction_width_bounded: bool,
    pub cofibration_disjunction_width_bounded: bool,
    pub recursive_context_depth_bounded_by_semantic_depth: bool,
    pub no_raw_ast_or_node_cap_used: bool,
    pub structural_membership_decidable: bool,
    pub finite_inductive_grammar_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2NormalizationProof {
    pub induction_measure: String,
    pub every_recursive_call_strictly_decreases: bool,
    pub resident_normal_forms_replayed: bool,
    pub every_enumerated_representative_typed: bool,
    pub every_enumerated_representative_normalized: bool,
    pub total_on_bounded_carrier: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2QuotientProof {
    pub equality_procedure: String,
    pub canonical_parameter_renaming_total: bool,
    pub uniform_specializations_not_multiplied: bool,
    pub structural_equality_decidable: bool,
    pub every_carrier_row_has_one_quotient_key: bool,
    pub quotient_key_collisions_replayed_by_equality: bool,
    pub quotient_decidable: bool,
    pub quotient_finitely_enumerable: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2FormerRule {
    pub former: Td21V2PublicFormer,
    pub arity: u32,
    pub binder_argument_positions: Vec<u32>,
    pub dimension_required: bool,
    pub zero_arity_former_is_semantic_depth_zero: bool,
    pub derivation_hash: String,
}

/// Legacy node-shape grammar retained as structural diagnostic testimony.
///
/// The explicit representative vector in the certificate is deliberately a
/// regression basis.  This grammar does not enumerate dependent parameter
/// telescopes and therefore is not a finite carrier enumerator or a quotient
/// bound.  The corresponding universal booleans remain false.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2SymbolicCarrierGrammar {
    pub resident_clause_keys: Vec<String>,
    pub context_variable_range_inclusive: (u32, u32),
    pub bound_variable_range_inclusive: (u32, u32),
    pub former_rules: Vec<Td21V2FormerRule>,
    pub maximum_semantic_depth: u32,
    pub maximum_context_arity: u32,
    pub maximum_binder_nesting: u32,
    pub maximum_path_dimension: u32,
    pub legacy_node_shape_recurrence_diagnostic_decimal: String,
    pub every_member_has_exactly_one_root_rule: bool,
    pub child_semantic_depth_strictly_smaller: bool,
    pub membership_elimination_total: bool,
    pub finite_enumerator_constructed: bool,
    pub quotient_key_decision_total_after_typed_normalization: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2SymbolicCarrierInduction {
    pub every_member_has_operational_node: bool,
    pub cases_exactly_resident_context_bound_or_former: bool,
    pub former_arguments_are_members_at_strictly_lower_semantic_depth: bool,
    pub recursion_well_founded: bool,
    pub quotient_respects_constructor_congruence: bool,
    pub induction_total: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2IndependenceFirewall {
    pub definition_inputs: Vec<String>,
    pub observer_inputs: Vec<String>,
    pub bounds_frozen_before_corpus_access: bool,
    pub carrier_sealed_before_corpus_access: bool,
    pub family_corpus_reaches_carrier_definition: bool,
    pub a3_corpus_reaches_carrier_definition: bool,
    pub raw_ast_height_cap_used: bool,
    pub generator_inventory_used_as_carrier: bool,
    pub dataflow_disjoint: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2Regression {
    pub corpus_opened_after_pre_corpus_carrier_seal: bool,
    pub family_corpus_authenticated: bool,
    pub expected_family_count: u32,
    pub observed_family_count: u32,
    pub recognized_family_count: u32,
    pub every_family_recognized: bool,
    pub membership_row_corpus_authenticated: bool,
    pub expected_membership_row_count: u32,
    pub observed_membership_row_count: u32,
    pub recognized_membership_row_count: u32,
    pub every_membership_row_recognized: bool,
    pub corpus_rows_seeded_or_bounded_carrier: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2MembershipDecision {
    pub member: bool,
    pub semantic_depth: Option<Td21V2SemanticDepth>,
    pub carrier_key: Option<String>,
    pub quotient_key: Option<String>,
    pub normal_form: Option<Expr>,
    pub exact_reason: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21OperationalDomainV2Certificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<Td21V2SourceBinding>,
    pub history_signature_digest: String,
    pub bound_snapshot_digest: String,
    pub pre_corpus_carrier_seal: String,
    pub carrier_definition_digest: String,
    pub bound_snapshot: Td21V2BoundSnapshot,
    pub context_finiteness: Td21V2ContextFinitenessProof,
    pub normalization_proof: Td21V2NormalizationProof,
    pub quotient_proof: Td21V2QuotientProof,
    pub symbolic_carrier_grammar: Td21V2SymbolicCarrierGrammar,
    pub symbolic_carrier_induction: Td21V2SymbolicCarrierInduction,
    pub independence_firewall: Td21V2IndependenceFirewall,
    /// Explicit regression/basis representatives.  Consumers must inspect
    /// `quotient_representatives_extensionally_complete`; otherwise they use
    /// `symbolic_carrier_grammar` and the public membership eliminator.
    pub quotient_representatives: Vec<Td21V2CarrierRepresentative>,
    pub quotient_representatives_extensionally_complete: bool,
    pub carrier_representative_count: u32,
    pub quotient_representative_count: u32,
    pub regression: Td21V2Regression,
    pub open_gaps: Vec<Td21V2NamedGap>,
    pub fixed_fragment_operationally_defined: bool,
    pub membership_decidable: bool,
    pub typed_normalization_total: bool,
    pub quotient_decidable: bool,
    pub quotient_finitely_enumerable: bool,
    pub family_regression_passed: bool,
    pub membership_row_regression_passed: bool,
    pub t_d2_2_prerequisite_satisfied: bool,
    pub m4_authorized: bool,
    pub status: Td21V2RunStatus,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Td21V2Replay {
    pub valid: bool,
    pub errors: Vec<String>,
    pub status: Td21V2RunStatus,
    pub fixed_fragment_operationally_defined: bool,
    pub membership_decidable: bool,
    pub quotient_finitely_enumerable: bool,
    pub family_regression_passed: bool,
    pub membership_row_regression_passed: bool,
    pub t_d2_2_prerequisite_satisfied: bool,
    pub m4_authorized: bool,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Td21V2Error {
    #[error("T-D2-1 v2 input failure: {0}")]
    Input(String),
    #[error("T-D2-1 v2 bound extraction failure: {0}")]
    BoundExtraction(String),
    #[error("T-D2-1 v2 enumeration failure: {0}")]
    Enumeration(String),
    #[error("T-D2-1 v2 serialization failure: {0}")]
    Serialization(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(T_D2_1_OPERATIONAL_DOMAIN_V2_SCHEMA, domain, value))
        .expect("T-D2-1 v2 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_binding(
    path: &str,
    role: &str,
    bytes: &[u8],
    register: Td21V2EvidenceRegister,
) -> Td21V2SourceBinding {
    Td21V2SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len(),
        blake3: bytes_hash(bytes),
        sha256: sha256(bytes),
        register,
    }
}

fn definition_source_bindings() -> Vec<Td21V2SourceBinding> {
    [
        (
            "docs/schema2_operational_domain_adjudication.md",
            "adopted operational bound authority",
            OPERATIONAL_ADJUDICATION_BYTES,
            Td21V2EvidenceRegister::BoundCitation,
        ),
        (
            "docs/depth_two_domain_adjudication.md",
            "adopted independent-domain and quotient target",
            PARENT_ADJUDICATION_BYTES,
            Td21V2EvidenceRegister::DefinitionInput,
        ),
        (
            "docs/ambient_former_closure_adjudication.md",
            "adopted closed transparent-former inventory",
            AMBIENT_FORMER_ADJUDICATION_BYTES,
            Td21V2EvidenceRegister::BoundCitation,
        ),
        (
            "docs/dependent_context_adjudication.md",
            "adopted dependent-context declaration discipline",
            DEPENDENT_CONTEXT_ADJUDICATION_BYTES,
            Td21V2EvidenceRegister::BoundCitation,
        ),
        (
            "docs/e2_quotient_adjudications.md",
            "adopted natural-family versus instance quotient",
            QUOTIENT_ADJUDICATION_BYTES,
            Td21V2EvidenceRegister::DefinitionInput,
        ),
        (
            "docs/step_15_completion_open_problem.md",
            "typed Schema2 obligations and proxy prohibition",
            OPEN_PROBLEM_BYTES,
            Td21V2EvidenceRegister::DefinitionInput,
        ),
        (
            "crates/pen-core/src/expr.rs",
            "closed kernel expression alphabet",
            EXPR_SOURCE_BYTES,
            Td21V2EvidenceRegister::DefinitionInput,
        ),
        (
            "crates/pen-type/src/elaborate.rs",
            "sealed signature and frozen typed elaborator",
            ELABORATE_SOURCE_BYTES,
            Td21V2EvidenceRegister::BoundCitation,
        ),
        (
            "crates/pen-type/src/ambient_former_internality.rs",
            "executable transparent-former registry",
            AMBIENT_FORMER_SOURCE_BYTES,
            Td21V2EvidenceRegister::BoundCitation,
        ),
        (
            "crates/pen-schema/src/context.rs",
            "typed dependent context constructor surface",
            CONTEXT_SOURCE_BYTES,
            Td21V2EvidenceRegister::BoundCitation,
        ),
        (
            "crates/pen-type/src/normalize.rs",
            "frozen terminating beta normalizer",
            NORMALIZE_SOURCE_BYTES,
            Td21V2EvidenceRegister::DefinitionInput,
        ),
        (
            "crates/pen-type/src/equality.rs",
            "frozen univalent equality decision procedure",
            EQUALITY_SOURCE_BYTES,
            Td21V2EvidenceRegister::DefinitionInput,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes, register)| source_binding(path, role, bytes, register))
    .collect()
}

/// These bytes are intentionally not hashed or parsed until the independent
/// bound snapshot and pre-corpus carrier seal have been constructed.
fn observer_source_bindings() -> Vec<Td21V2SourceBinding> {
    [
        (
            "docs/t_bi_nu1_semantic_provenance_v6.json",
            "post-seal 61-family regression observer",
            FAMILY_CORPUS_BYTES,
        ),
        (
            "docs/a3_rule_inventory_exhaustiveness_v2.json",
            "post-seal exact A3 occurrence/source regression observer",
            A3_CORPUS_BYTES,
        ),
        (
            "docs/schema2_e5_future_hole_finale_v2_dependent_context.json",
            "post-seal 89-row typed membership regression observer",
            MEMBERSHIP_CORPUS_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| {
        source_binding(
            path,
            role,
            bytes,
            Td21V2EvidenceRegister::StructuralTestimony,
        )
    })
    .collect()
}

fn citation(
    bound_name: &str,
    value: u32,
    exact_source: &str,
    bytes: &[u8],
    expected_sha256: &str,
) -> Td21V2BoundCitation {
    let found = sha256(bytes);
    let resolved = found == expected_sha256;
    let derivation_hash = tagged_hash(
        "bound-citation",
        &(
            bound_name,
            value,
            exact_source,
            &found,
            expected_sha256,
            resolved,
        ),
    );
    Td21V2BoundCitation {
        bound_name: bound_name.to_owned(),
        value,
        exact_source: exact_source.to_owned(),
        replayed_before_corpus_access: true,
        resolved,
        derivation_hash,
    }
}

fn role_label(role: ClauseRole) -> String {
    format!("{role:?}").to_ascii_lowercase()
}

fn syntax_number(value: u32, bound_name: &str) -> Td21V2SyntaxNumber {
    Td21V2SyntaxNumber {
        value,
        bound_name: bound_name.to_owned(),
    }
}

fn expression_binder_nesting(expr: &Expr, active: u32) -> u32 {
    match expr {
        Expr::Lam(body) => expression_binder_nesting(body, active + 1).max(active + 1),
        Expr::Pi(domain, codomain) | Expr::Sigma(domain, codomain) => {
            expression_binder_nesting(domain, active)
                .max(expression_binder_nesting(codomain, active + 1))
                .max(active + 1)
        }
        Expr::App(left, right) => {
            expression_binder_nesting(left, active).max(expression_binder_nesting(right, active))
        }
        Expr::Id(ty, left, right) => expression_binder_nesting(ty, active)
            .max(expression_binder_nesting(left, active))
            .max(expression_binder_nesting(right, active)),
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
        | Expr::WhyNot(inner) => expression_binder_nesting(inner, active),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => active,
    }
}

fn expression_max_path_dimension(expr: &Expr) -> Option<u32> {
    match expr {
        Expr::PathCon(dimension) => Some(*dimension),
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            expression_max_path_dimension(left)
                .into_iter()
                .chain(expression_max_path_dimension(right))
                .max()
        }
        Expr::Id(ty, left, right) => expression_max_path_dimension(ty)
            .into_iter()
            .chain(expression_max_path_dimension(left))
            .chain(expression_max_path_dimension(right))
            .max(),
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
        | Expr::WhyNot(inner) => expression_max_path_dimension(inner),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) => None,
    }
}

fn expression_semantic_former_depth(expr: &Expr) -> u32 {
    match expr {
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => 0,
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => 1_u32
            .saturating_add(
                expression_semantic_former_depth(left).max(expression_semantic_former_depth(right)),
            ),
        Expr::Id(ty, left, right) => 1_u32.saturating_add(
            expression_semantic_former_depth(ty)
                .max(expression_semantic_former_depth(left))
                .max(expression_semantic_former_depth(right)),
        ),
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
        | Expr::WhyNot(inner) => 1_u32.saturating_add(expression_semantic_former_depth(inner)),
    }
}

#[derive(Clone)]
struct ResidentClause {
    key: String,
    entry_identifier: u32,
    candidate_digest: String,
    clause_index: u16,
    declared_role: ClauseRole,
    normal_form: Expr,
    kernel_type_json: String,
    typing_derivation_hash: String,
}

#[derive(Clone)]
struct OperationalDefinition {
    signature_digest: String,
    bound_snapshot: Td21V2BoundSnapshot,
    residents: BTreeMap<(u32, u16), ResidentClause>,
    grammar: Td21V2SymbolicCarrierGrammar,
    induction: Td21V2SymbolicCarrierInduction,
    context_finiteness: Td21V2ContextFinitenessProof,
    carrier_definition_digest: String,
}

fn former_rules() -> Vec<Td21V2FormerRule> {
    use Td21V2PublicFormer as F;
    let specs = [
        (F::AmbientUniverse, 0, Vec::new(), false, true),
        (F::LambdaIntroduction, 1, vec![0], false, false),
        (F::Application, 2, Vec::new(), false, false),
        (F::PiFormation, 2, vec![1], false, false),
        (F::SigmaFormation, 2, vec![1], false, false),
        (F::IdentityFormation, 3, Vec::new(), false, false),
        (F::ReflexivityIntroduction, 1, Vec::new(), false, false),
        (F::SuspensionFormation, 1, Vec::new(), false, false),
        (F::TruncationFormation, 1, Vec::new(), false, false),
        (F::FlatFormation, 1, Vec::new(), false, false),
        (F::SharpFormation, 1, Vec::new(), false, false),
        (F::DiscreteFormation, 1, Vec::new(), false, false),
        (F::ShapeFormation, 1, Vec::new(), false, false),
        (F::NextFormation, 1, Vec::new(), false, false),
        (F::EventuallyFormation, 1, Vec::new(), false, false),
        (F::PathConstructor, 0, Vec::new(), true, true),
    ];
    specs
        .into_iter()
        .map(
            |(
                former,
                arity,
                binder_argument_positions,
                dimension_required,
                zero_arity_former_is_semantic_depth_zero,
            )| {
                let derivation_hash = tagged_hash(
                    "former-rule",
                    &(
                        former,
                        arity,
                        &binder_argument_positions,
                        dimension_required,
                        zero_arity_former_is_semantic_depth_zero,
                    ),
                );
                Td21V2FormerRule {
                    former,
                    arity,
                    binder_argument_positions,
                    dimension_required,
                    zero_arity_former_is_semantic_depth_zero,
                    derivation_hash,
                }
            },
        )
        .collect()
}

fn legacy_node_shape_recurrence(
    depth: u32,
    active_binders: u32,
    resident_count: u32,
    context_arity: u32,
    binder_bound: u32,
    dimension_bound: u32,
) -> u128 {
    let leaves = u128::from(resident_count)
        + u128::from(context_arity)
        + u128::from(active_binders)
        + 1
        + u128::from(dimension_bound + 1);
    if depth == 0 {
        return leaves;
    }
    let same = legacy_node_shape_recurrence(
        depth - 1,
        active_binders,
        resident_count,
        context_arity,
        binder_bound,
        dimension_bound,
    );
    let under_binder = if active_binders < binder_bound {
        legacy_node_shape_recurrence(
            depth - 1,
            active_binders + 1,
            resident_count,
            context_arity,
            binder_bound,
            dimension_bound,
        )
    } else {
        0
    };
    leaves
        .saturating_add(9_u128.saturating_mul(same))
        .saturating_add(same.saturating_mul(same))
        .saturating_add(2_u128.saturating_mul(same).saturating_mul(under_binder))
        .saturating_add(same.saturating_pow(3))
        .saturating_add(under_binder)
}

/// Definition-only executable projection.  It is deliberately a closed list
/// of rule identifiers and never hashes this source file: the latter also
/// contains post-seal observer constants and regression code.
fn definition_implementation_projection() -> [&'static str; 10] {
    [
        "context-indexed-judgment-v1",
        "sequential-dependent-context-prefix-check-v1",
        "resident-root-context-transport-required-v1",
        "nested-resident-requires-clause-embedding-v1",
        "context-coordinate-preserving-expression-v1",
        "binder-child-context-propagation-v1",
        "constructor-recursive-kernel-derivation-v1",
        "typed-normalization-is-concrete-not-universal-v1",
        "natural-family-quotient-four-witness-gate-v1",
        "regression-requires-exact-presentation-link-v1",
    ]
}

fn build_operational_definition() -> Result<OperationalDefinition, Td21V2Error> {
    // Definition phase.  No observer bytes are hashed or parsed in this
    // function; the returned digest is the pre-corpus carrier definition.
    let signature = SealedSignature::genesis_del_h15();
    let mut residents = BTreeMap::new();
    let mut public_clause_count = 0_u32;
    let mut maximum_ambient_parameter_count = 0_u32;
    let mut maximum_free_scope_length = 0_u32;
    let mut maximum_binder_nesting = 0_u32;
    let mut maximum_path_dimension = 0_u32;

    for entry in signature.entries() {
        let elaboration =
            elaborate_telescope(&signature, &entry.telescope, entry.step.saturating_sub(1))
                .map_err(|error| {
                    Td21V2Error::BoundExtraction(format!(
                        "sealed entry {} failed typed replay: {error}",
                        entry.step
                    ))
                })?;
        maximum_ambient_parameter_count =
            maximum_ambient_parameter_count.max(elaboration.ambient_parameters);
        for clause in &elaboration.clauses {
            public_clause_count = public_clause_count.saturating_add(1);
            maximum_free_scope_length = maximum_free_scope_length
                .max(elaboration.ambient_parameters + u32::from(clause.clause_index));
            maximum_binder_nesting =
                maximum_binder_nesting.max(expression_binder_nesting(&clause.normal_form, 0));
            if let Some(dimension) = expression_max_path_dimension(&clause.normal_form) {
                maximum_path_dimension = maximum_path_dimension.max(dimension);
            }
            let key = tagged_hash(
                "resident-clause",
                &(
                    signature.digest(),
                    entry.step,
                    &entry.candidate_hash,
                    clause.clause_index,
                    clause.declared_role,
                    &clause.normal_form,
                    &clause.kernel_ty,
                    &elaboration.derivation_hash,
                ),
            );
            let resident = ResidentClause {
                key,
                entry_identifier: entry.step,
                candidate_digest: entry.candidate_hash.clone(),
                clause_index: clause.clause_index,
                declared_role: clause.declared_role,
                normal_form: clause.normal_form.clone(),
                kernel_type_json: serde_json::to_string(&clause.kernel_ty)
                    .map_err(|error| Td21V2Error::Serialization(error.to_string()))?,
                typing_derivation_hash: elaboration.derivation_hash.clone(),
            };
            residents.insert((entry.step, clause.clause_index), resident);
        }
    }

    let transparent_former_count =
        u32::try_from(registered_transparent_formers().len()).map_err(|_| {
            Td21V2Error::BoundExtraction("transparent former count exceeds u32".to_owned())
        })?;
    let public_entry_count = u32::try_from(signature.len())
        .map_err(|_| Td21V2Error::BoundExtraction("entry count exceeds u32".to_owned()))?;
    let citations = vec![
        citation(
            "public_entry_count",
            public_entry_count,
            "SealedSignature::genesis_del_h15 entry enumeration",
            ELABORATE_SOURCE_BYTES,
            ELABORATE_SOURCE_SHA256,
        ),
        citation(
            "public_clause_count",
            public_clause_count,
            "typed replay of every SealedSignature::genesis_del_h15 telescope",
            ELABORATE_SOURCE_BYTES,
            ELABORATE_SOURCE_SHA256,
        ),
        citation(
            "maximum_ambient_parameter_count",
            maximum_ambient_parameter_count,
            "typed replay of sealed telescope declaration contexts",
            ELABORATE_SOURCE_BYTES,
            ELABORATE_SOURCE_SHA256,
        ),
        citation(
            "closed_public_alphabet",
            transparent_former_count,
            "ambient-former closure adoption plus registered_transparent_formers()",
            AMBIENT_FORMER_ADJUDICATION_BYTES,
            AMBIENT_FORMER_ADJUDICATION_SHA256,
        ),
        citation(
            "semantic_former_depth",
            SEMANTIC_FORMER_DEPTH_BOUND,
            "schema2-operational-domain-v1 clause 2",
            OPERATIONAL_ADJUDICATION_BYTES,
            OPERATIONAL_ADJUDICATION_SHA256,
        ),
        citation(
            "maximum_context_arity",
            maximum_free_scope_length,
            "SealedSignature::genesis_del_h15 plus dependent-context adoption",
            DEPENDENT_CONTEXT_ADJUDICATION_BYTES,
            DEPENDENT_CONTEXT_ADJUDICATION_SHA256,
        ),
        citation(
            "maximum_binder_nesting",
            maximum_binder_nesting,
            "recursive binder dereference over sealed elaborated normal forms",
            ELABORATE_SOURCE_BYTES,
            ELABORATE_SOURCE_SHA256,
        ),
        citation(
            "maximum_path_dimension",
            maximum_path_dimension,
            "recursive PathCon dereference over sealed public normal forms",
            ELABORATE_SOURCE_BYTES,
            ELABORATE_SOURCE_SHA256,
        ),
    ];
    let all_citations_resolved_before_corpus_access =
        citations.iter().all(|citation| citation.resolved);
    let mut bound_snapshot = Td21V2BoundSnapshot {
        public_entry_count,
        public_clause_count,
        maximum_ambient_parameter_count,
        maximum_free_scope_length,
        maximum_binder_nesting,
        maximum_path_dimension,
        semantic_former_depth: SEMANTIC_FORMER_DEPTH_BOUND,
        transparent_former_count,
        citations,
        all_citations_resolved_before_corpus_access,
        derivation_hash: String::new(),
    };
    bound_snapshot.derivation_hash = tagged_hash(
        "bound-snapshot",
        &(
            bound_snapshot.public_entry_count,
            bound_snapshot.public_clause_count,
            bound_snapshot.maximum_ambient_parameter_count,
            bound_snapshot.maximum_free_scope_length,
            bound_snapshot.maximum_binder_nesting,
            bound_snapshot.maximum_path_dimension,
            bound_snapshot.semantic_former_depth,
            bound_snapshot.transparent_former_count,
            &bound_snapshot.citations,
            bound_snapshot.all_citations_resolved_before_corpus_access,
        ),
    );

    let resident_clause_keys = residents
        .values()
        .map(|resident| resident.key.clone())
        .collect::<Vec<_>>();
    let node_shape_diagnostic = legacy_node_shape_recurrence(
        SEMANTIC_FORMER_DEPTH_BOUND,
        0,
        public_clause_count,
        maximum_free_scope_length,
        maximum_binder_nesting,
        maximum_path_dimension,
    );
    let mut grammar = Td21V2SymbolicCarrierGrammar {
        resident_clause_keys,
        context_variable_range_inclusive: (1, maximum_free_scope_length),
        bound_variable_range_inclusive: (1, maximum_binder_nesting),
        former_rules: former_rules(),
        maximum_semantic_depth: SEMANTIC_FORMER_DEPTH_BOUND,
        maximum_context_arity: maximum_free_scope_length,
        maximum_binder_nesting,
        maximum_path_dimension,
        legacy_node_shape_recurrence_diagnostic_decimal: node_shape_diagnostic.to_string(),
        every_member_has_exactly_one_root_rule: true,
        child_semantic_depth_strictly_smaller: true,
        membership_elimination_total: false,
        finite_enumerator_constructed: false,
        quotient_key_decision_total_after_typed_normalization: false,
        derivation_hash: String::new(),
    };
    grammar.derivation_hash = tagged_hash(
        "symbolic-carrier-grammar",
        &(
            &grammar.resident_clause_keys,
            grammar.context_variable_range_inclusive,
            grammar.bound_variable_range_inclusive,
            &grammar.former_rules,
            grammar.maximum_semantic_depth,
            grammar.maximum_context_arity,
            grammar.maximum_binder_nesting,
            grammar.maximum_path_dimension,
            &grammar.legacy_node_shape_recurrence_diagnostic_decimal,
            grammar.every_member_has_exactly_one_root_rule,
            grammar.child_semantic_depth_strictly_smaller,
            grammar.membership_elimination_total,
            grammar.finite_enumerator_constructed,
            grammar.quotient_key_decision_total_after_typed_normalization,
        ),
    );
    let mut induction = Td21V2SymbolicCarrierInduction {
        every_member_has_operational_node: false,
        cases_exactly_resident_context_bound_or_former: true,
        former_arguments_are_members_at_strictly_lower_semantic_depth: true,
        recursion_well_founded: true,
        quotient_respects_constructor_congruence: false,
        induction_total: false,
        derivation_hash: String::new(),
    };
    induction.derivation_hash = tagged_hash(
        "symbolic-carrier-induction",
        &(
            induction.every_member_has_operational_node,
            induction.cases_exactly_resident_context_bound_or_former,
            induction.former_arguments_are_members_at_strictly_lower_semantic_depth,
            induction.recursion_well_founded,
            induction.quotient_respects_constructor_congruence,
            induction.induction_total,
            &grammar.derivation_hash,
        ),
    );
    let mut context_finiteness = Td21V2ContextFinitenessProof {
        display_names_erased: true,
        library_symbols_replaced_by_resident_coordinates: true,
        declaration_width_bounded: true,
        cofibration_conjunction_width_bounded: true,
        cofibration_disjunction_width_bounded: true,
        recursive_context_depth_bounded_by_semantic_depth: true,
        no_raw_ast_or_node_cap_used: true,
        structural_membership_decidable: false,
        finite_inductive_grammar_proved: false,
        derivation_hash: String::new(),
    };
    context_finiteness.derivation_hash = tagged_hash(
        "context-finiteness",
        &(
            &bound_snapshot.derivation_hash,
            sha256(CONTEXT_SOURCE_BYTES),
            context_finiteness.display_names_erased,
            context_finiteness.library_symbols_replaced_by_resident_coordinates,
            context_finiteness.declaration_width_bounded,
            context_finiteness.cofibration_conjunction_width_bounded,
            context_finiteness.cofibration_disjunction_width_bounded,
            context_finiteness.recursive_context_depth_bounded_by_semantic_depth,
            context_finiteness.no_raw_ast_or_node_cap_used,
            context_finiteness.structural_membership_decidable,
            context_finiteness.finite_inductive_grammar_proved,
        ),
    );
    let carrier_definition_digest = tagged_hash(
        "carrier-definition",
        &(
            definition_implementation_projection(),
            signature.digest(),
            &bound_snapshot,
            &grammar,
            &induction,
            &context_finiteness,
            definition_source_bindings(),
        ),
    );
    Ok(OperationalDefinition {
        signature_digest: signature.digest().to_owned(),
        bound_snapshot,
        residents,
        grammar,
        induction,
        context_finiteness,
        carrier_definition_digest,
    })
}

#[derive(Clone)]
struct ValidatedNode {
    semantic_depth: Td21V2SemanticDepth,
    parameter_coordinates: BTreeSet<u32>,
}

#[derive(Clone)]
struct FullMembershipDecision {
    public: Td21V2MembershipDecision,
    kernel_type_json: String,
    typing_derivation_hash: String,
    normalization_derivation_hash: String,
}

fn depth_from_value(value: u32) -> Td21V2SemanticDepth {
    match value {
        0 => Td21V2SemanticDepth::ResidentZero,
        1 => Td21V2SemanticDepth::FormerOne,
        _ => Td21V2SemanticDepth::FormerTwo,
    }
}

fn former_rule<'a>(
    definition: &'a OperationalDefinition,
    former: Td21V2PublicFormer,
) -> Option<&'a Td21V2FormerRule> {
    definition
        .grammar
        .former_rules
        .iter()
        .find(|rule| rule.former == former)
}

fn validate_syntax_number(
    number: &Td21V2SyntaxNumber,
    expected_bound_name: &str,
    minimum: u32,
    maximum: u32,
) -> Result<(), String> {
    if number.bound_name != expected_bound_name {
        return Err(format!(
            "coordinate {} cites {}, expected {}",
            number.value, number.bound_name, expected_bound_name
        ));
    }
    if number.value < minimum || number.value > maximum {
        return Err(format!(
            "coordinate {} lies outside inclusive bound {}..={}",
            number.value, minimum, maximum
        ));
    }
    Ok(())
}

fn validate_node(
    definition: &OperationalDefinition,
    node: &Td21V2OperationalNode,
    context_arity: u32,
    active_binders: u32,
) -> Result<ValidatedNode, String> {
    match node {
        Td21V2OperationalNode::ResidentClause {
            entry_identifier,
            candidate_digest,
            clause_index,
            declared_role,
            normal_form,
        } => {
            validate_syntax_number(
                entry_identifier,
                "sealed_entry_identifier",
                1,
                definition.bound_snapshot.public_entry_count,
            )?;
            validate_syntax_number(
                clause_index,
                "sealed_clause_index",
                0,
                definition
                    .bound_snapshot
                    .public_clause_count
                    .saturating_sub(1),
            )?;
            let clause_index_u16 = u16::try_from(clause_index.value)
                .map_err(|_| "resident clause index exceeds u16".to_owned())?;
            let resident = definition
                .residents
                .get(&(entry_identifier.value, clause_index_u16))
                .ok_or_else(|| {
                    format!(
                        "no sealed resident at entry {}, clause {}",
                        entry_identifier.value, clause_index.value
                    )
                })?;
            if candidate_digest != &resident.candidate_digest
                || declared_role != &role_label(resident.declared_role)
                || normal_form != &resident.normal_form
            {
                return Err("resident coordinate content does not replay exactly".to_owned());
            }
            Ok(ValidatedNode {
                semantic_depth: Td21V2SemanticDepth::ResidentZero,
                parameter_coordinates: BTreeSet::new(),
            })
        }
        Td21V2OperationalNode::ContextVariable { parameter } => {
            validate_syntax_number(
                parameter,
                "sealed_maximum_free_scope_length",
                1,
                context_arity,
            )?;
            let mut parameter_coordinates = BTreeSet::new();
            parameter_coordinates.insert(parameter.value);
            Ok(ValidatedNode {
                semantic_depth: Td21V2SemanticDepth::ResidentZero,
                parameter_coordinates,
            })
        }
        Td21V2OperationalNode::BoundVariable { binder_level } => {
            validate_syntax_number(
                binder_level,
                "sealed_maximum_binder_nesting",
                1,
                definition.bound_snapshot.maximum_binder_nesting,
            )?;
            if binder_level.value > active_binders {
                return Err(format!(
                    "bound variable {} is not in the active binder prefix {}",
                    binder_level.value, active_binders
                ));
            }
            Ok(ValidatedNode {
                semantic_depth: Td21V2SemanticDepth::ResidentZero,
                parameter_coordinates: BTreeSet::new(),
            })
        }
        Td21V2OperationalNode::FormerApplication {
            former,
            arguments,
            dimension,
        } => {
            let rule = former_rule(definition, *former)
                .ok_or_else(|| format!("former {former:?} is outside the public alphabet"))?;
            if arguments.len() != rule.arity as usize {
                return Err(format!(
                    "former {former:?} has arity {}, received {} arguments",
                    rule.arity,
                    arguments.len()
                ));
            }
            if rule.dimension_required {
                let dimension = dimension
                    .as_ref()
                    .ok_or_else(|| format!("former {former:?} requires a dimension"))?;
                validate_syntax_number(
                    dimension,
                    "sealed_maximum_path_dimension",
                    0,
                    definition.bound_snapshot.maximum_path_dimension,
                )?;
            } else if dimension.is_some() {
                return Err(format!("former {former:?} does not accept a dimension"));
            }
            if rule.zero_arity_former_is_semantic_depth_zero {
                return Ok(ValidatedNode {
                    semantic_depth: Td21V2SemanticDepth::ResidentZero,
                    parameter_coordinates: BTreeSet::new(),
                });
            }
            let mut child_max_depth = 0_u32;
            let mut parameter_coordinates = BTreeSet::new();
            for (index, argument) in arguments.iter().enumerate() {
                let binds = rule.binder_argument_positions.contains(&(index as u32));
                let child_binders = active_binders + u32::from(binds);
                if child_binders > definition.bound_snapshot.maximum_binder_nesting {
                    return Err(format!(
                        "former {former:?} would exceed sealed binder nesting {}",
                        definition.bound_snapshot.maximum_binder_nesting
                    ));
                }
                let child = validate_node(definition, argument, context_arity, child_binders)?;
                child_max_depth = child_max_depth.max(child.semantic_depth.value());
                parameter_coordinates.extend(child.parameter_coordinates);
            }
            let depth = child_max_depth.saturating_add(1);
            if depth > definition.bound_snapshot.semantic_former_depth {
                return Err(format!(
                    "semantic former depth {depth} exceeds adopted depth {}",
                    definition.bound_snapshot.semantic_former_depth
                ));
            }
            Ok(ValidatedNode {
                semantic_depth: depth_from_value(depth),
                parameter_coordinates,
            })
        }
    }
}

fn exact_family_expression(
    node: &Td21V2OperationalNode,
    ambient_parameters: u32,
    nested: bool,
) -> Result<Expr, String> {
    match node {
        Td21V2OperationalNode::ResidentClause { .. } => {
            if nested {
                Err(
                    "a resident clause nested below a former has no exact clause-level embedding in the frozen Expr/kernel API"
                        .to_owned(),
                )
            } else {
                Err("resident roots use their sealed normal presentation directly".to_owned())
            }
        }
        Td21V2OperationalNode::ContextVariable { parameter } => Ok(Expr::Var(parameter.value)),
        Td21V2OperationalNode::BoundVariable { binder_level } => {
            Ok(Expr::Var(ambient_parameters + binder_level.value))
        }
        Td21V2OperationalNode::FormerApplication {
            former,
            arguments,
            dimension,
        } => {
            use Td21V2PublicFormer as F;
            let argument =
                |index: usize| exact_family_expression(&arguments[index], ambient_parameters, true);
            match former {
                F::AmbientUniverse => Ok(Expr::Univ),
                F::LambdaIntroduction => Ok(Expr::Lam(Box::new(argument(0)?))),
                F::Application => Ok(Expr::App(Box::new(argument(0)?), Box::new(argument(1)?))),
                F::PiFormation => Ok(Expr::Pi(Box::new(argument(0)?), Box::new(argument(1)?))),
                F::SigmaFormation => {
                    Ok(Expr::Sigma(Box::new(argument(0)?), Box::new(argument(1)?)))
                }
                F::IdentityFormation => Ok(Expr::Id(
                    Box::new(argument(0)?),
                    Box::new(argument(1)?),
                    Box::new(argument(2)?),
                )),
                F::ReflexivityIntroduction => Ok(Expr::Refl(Box::new(argument(0)?))),
                F::SuspensionFormation => Ok(Expr::Susp(Box::new(argument(0)?))),
                F::TruncationFormation => Ok(Expr::Trunc(Box::new(argument(0)?))),
                F::FlatFormation => Ok(Expr::Flat(Box::new(argument(0)?))),
                F::SharpFormation => Ok(Expr::Sharp(Box::new(argument(0)?))),
                F::DiscreteFormation => Ok(Expr::Disc(Box::new(argument(0)?))),
                F::ShapeFormation => Ok(Expr::Shape(Box::new(argument(0)?))),
                F::NextFormation => Ok(Expr::Next(Box::new(argument(0)?))),
                F::EventuallyFormation => Ok(Expr::Eventually(Box::new(argument(0)?))),
                F::PathConstructor => Ok(Expr::PathCon(
                    dimension.as_ref().expect("validated dimension").value,
                )),
            }
        }
    }
}

fn nonmember(reason: String) -> FullMembershipDecision {
    let derivation_hash = tagged_hash("nonmember", &reason);
    FullMembershipDecision {
        public: Td21V2MembershipDecision {
            member: false,
            semantic_depth: None,
            carrier_key: None,
            quotient_key: None,
            normal_form: None,
            exact_reason: reason,
            derivation_hash,
        },
        kernel_type_json: String::new(),
        typing_derivation_hash: String::new(),
        normalization_derivation_hash: String::new(),
    }
}

fn parameter_telescope_digest(context: &Td21V2ParameterTelescope) -> String {
    tagged_hash("parameter-telescope", context)
}

fn validate_parameter_telescope(
    definition: &OperationalDefinition,
    context: &Td21V2ParameterTelescope,
) -> Result<Vec<KernelTy>, String> {
    if context.hypotheses.len() > definition.bound_snapshot.maximum_free_scope_length as usize {
        return Err(format!(
            "parameter telescope arity {} exceeds sealed maximum {}",
            context.hypotheses.len(),
            definition.bound_snapshot.maximum_free_scope_length
        ));
    }
    let mut prefix = Vec::<KernelTy>::new();
    for (index, classifier) in context.hypotheses.iter().enumerate() {
        match classifier {
            Td21V2KernelClassifier::Type => prefix.push(classifier.to_kernel_ty()),
            Td21V2KernelClassifier::Element { type_expression } => {
                let prefix_arity = u32::try_from(index)
                    .map_err(|_| "parameter telescope index exceeds u32".to_owned())?;
                if expression_binder_nesting(type_expression, 0)
                    > definition.bound_snapshot.maximum_binder_nesting
                {
                    return Err(format!(
                        "dependent classifier at parameter {} exceeds sealed binder nesting {}",
                        index + 1,
                        definition.bound_snapshot.maximum_binder_nesting
                    ));
                }
                if expression_semantic_former_depth(type_expression)
                    > definition.bound_snapshot.semantic_former_depth
                {
                    return Err(format!(
                        "dependent classifier at parameter {} exceeds adopted semantic former depth {}",
                        index + 1,
                        definition.bound_snapshot.semantic_former_depth
                    ));
                }
                if expression_max_path_dimension(type_expression).is_some_and(|dimension| {
                    dimension > definition.bound_snapshot.maximum_path_dimension
                }) {
                    return Err(format!(
                        "dependent classifier at parameter {} exceeds sealed cubical dimension {}",
                        index + 1,
                        definition.bound_snapshot.maximum_path_dimension
                    ));
                }
                if type_expression
                    .var_refs()
                    .iter()
                    .any(|coordinate| *coordinate == 0 || *coordinate > prefix_arity)
                {
                    return Err(format!(
                        "dependent classifier at parameter {} refers outside its earlier prefix",
                        index + 1
                    ));
                }
                let (elaboration, derivation) = elaborate_single_clause_with_typed_ambient(
                    type_expression,
                    &prefix,
                    &[],
                    15,
                )
                .map_err(|error| {
                    format!(
                        "dependent classifier at parameter {} failed frozen typing replay: {error}",
                        index + 1
                    )
                })?;
                if elaboration.kernel_ty != KernelTy::Type {
                    return Err(format!(
                        "dependent classifier at parameter {} does not synthesize Type",
                        index + 1
                    ));
                }
                let replay_hash = tagged_hash(
                    "dependent-context-entry",
                    &(index, type_expression, &prefix, &elaboration, &derivation),
                );
                if replay_hash.is_empty() {
                    return Err("dependent context replay did not produce evidence".to_owned());
                }
                prefix.push(classifier.to_kernel_ty());
            }
            Td21V2KernelClassifier::Function { .. }
            | Td21V2KernelClassifier::PathDeclaration { .. }
            | Td21V2KernelClassifier::Neutral => {
                return Err(format!(
                    "parameter {} uses classifier {:?}; the frozen public kernel exposes no sequential declaration checker for this classifier",
                    index + 1,
                    classifier
                ));
            }
        }
    }
    Ok(prefix)
}

fn incomplete_quotient_evidence(
    context: &Td21V2ParameterTelescope,
    exact_presentation: &Expr,
    normal_form: &Expr,
    kernel_type_json: &str,
) -> Td21V2NaturalFamilyQuotientEvidence {
    let parameter_telescope_digest = parameter_telescope_digest(context);
    let exact_obstruction = Some(
        "The frozen APIs provide typing and normalization but no replayable parameter-telescope naturality derivation, checked weakening/renaming family, or univalent-equality witness family. A normal-form/type hash is not the adopted natural-family quotient."
            .to_owned(),
    );
    let mut evidence = Td21V2NaturalFamilyQuotientEvidence {
        parameter_telescope_digest,
        canonical_weakening_maps_checked: false,
        canonical_renaming_maps_checked: false,
        naturality_derivation_replayed: false,
        univalent_equality_witnesses_replayed: false,
        adopted_quotient_established: false,
        exact_obstruction,
        derivation_hash: String::new(),
    };
    evidence.derivation_hash = tagged_hash(
        "natural-family-quotient-evidence",
        &(
            &evidence.parameter_telescope_digest,
            exact_presentation,
            normal_form,
            kernel_type_json,
            evidence.canonical_weakening_maps_checked,
            evidence.canonical_renaming_maps_checked,
            evidence.naturality_derivation_replayed,
            evidence.univalent_equality_witnesses_replayed,
            evidence.adopted_quotient_established,
            &evidence.exact_obstruction,
        ),
    );
    evidence
}

fn expression_node_count(expr: &Expr) -> u32 {
    match expr {
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => 1_u32
            .saturating_add(expression_node_count(left))
            .saturating_add(expression_node_count(right)),
        Expr::Id(ty, left, right) => 1_u32
            .saturating_add(expression_node_count(ty))
            .saturating_add(expression_node_count(left))
            .saturating_add(expression_node_count(right)),
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
        | Expr::WhyNot(inner) => 1_u32.saturating_add(expression_node_count(inner)),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => 1,
    }
}

fn binder_classifier_for_child(
    former: Td21V2PublicFormer,
    arguments: &[Td21V2OperationalNode],
    ambient_parameters: u32,
    active_binders: u32,
) -> Result<Td21V2KernelClassifier, String> {
    match former {
        Td21V2PublicFormer::LambdaIntroduction => Ok(Td21V2KernelClassifier::Neutral),
        Td21V2PublicFormer::PiFormation | Td21V2PublicFormer::SigmaFormation => {
            let domain = arguments
                .first()
                .ok_or_else(|| "dependent former lacks its domain argument".to_owned())?;
            let domain_expression = exact_family_expression(domain, ambient_parameters, true)?;
            let fuel = expression_node_count(&domain_expression).max(16);
            let domain_whnf = whnf(
                &domain_expression,
                ambient_parameters.saturating_add(active_binders),
                fuel,
            )
            .map_err(|error| {
                format!("dependent binder-domain weak-head normalization failed: {error}")
            })?;
            Ok(Td21V2KernelClassifier::Element {
                type_expression: domain_whnf.expr,
            })
        }
        _ => Err(format!(
            "former {former:?} declared an unexpected binder argument"
        )),
    }
}

fn build_constructive_derivation(
    definition: &OperationalDefinition,
    context: &Td21V2ParameterTelescope,
    node: &Td21V2OperationalNode,
    exact_presentation: &Expr,
    kernel_node: &DerivationNode,
    active_binders: &[Td21V2KernelClassifier],
) -> Result<Td21V2ConstructiveDerivation, String> {
    let context_arity = u32::try_from(context.hypotheses.len())
        .map_err(|_| "parameter telescope arity exceeds u32".to_owned())?;
    let active_binder_count = u32::try_from(active_binders.len())
        .map_err(|_| "active binder count exceeds u32".to_owned())?;
    let validated = validate_node(definition, node, context_arity, active_binder_count)?;
    let scope_len = context_arity.saturating_add(active_binder_count);
    let fuel = expression_node_count(exact_presentation).max(16);
    let normalized = normalize(exact_presentation, scope_len, fuel)
        .map_err(|error| format!("subderivation normalization failed: {error}"))?;
    let kernel_type_json = serde_json::to_string(&kernel_node.kernel_ty)
        .map_err(|error| format!("kernel classifier serialization failed: {error}"))?;
    let mut children = Vec::new();
    let (rule, former) = match node {
        Td21V2OperationalNode::ResidentClause { .. } => {
            return Err(
                "resident clauses may only be replayed as opaque roots; a nested resident has no exact kernel presentation"
                    .to_owned(),
            );
        }
        Td21V2OperationalNode::ContextVariable { .. } => {
            (Td21V2ConstructiveRule::ContextProjection, None)
        }
        Td21V2OperationalNode::BoundVariable { .. } => {
            (Td21V2ConstructiveRule::BoundProjection, None)
        }
        Td21V2OperationalNode::FormerApplication {
            former, arguments, ..
        } => {
            if kernel_node.children.len() < arguments.len() {
                return Err(format!(
                    "kernel rule {} exposes {} child derivations for {former:?}, expected at least {}",
                    kernel_node.rule,
                    kernel_node.children.len(),
                    arguments.len()
                ));
            }
            let former_rule = former_rule(definition, *former)
                .ok_or_else(|| format!("former {former:?} has no operational rule"))?;
            for (index, argument) in arguments.iter().enumerate() {
                let child_expression = exact_family_expression(argument, context_arity, true)?;
                let mut child_binders = active_binders.to_vec();
                if former_rule
                    .binder_argument_positions
                    .contains(&(index as u32))
                {
                    child_binders.push(binder_classifier_for_child(
                        *former,
                        arguments,
                        context_arity,
                        active_binder_count,
                    )?);
                }
                children.push(build_constructive_derivation(
                    definition,
                    context,
                    argument,
                    &child_expression,
                    &kernel_node.children[index],
                    &child_binders,
                )?);
            }
            (Td21V2ConstructiveRule::FormerApplication, Some(*former))
        }
    };
    let parameter_coordinates_used = validated
        .parameter_coordinates
        .iter()
        .copied()
        .collect::<Vec<_>>();
    let parameter_telescope_digest = parameter_telescope_digest(context);
    let kernel_derivation_hash = tagged_hash(
        "context-indexed-kernel-node",
        &(
            &parameter_telescope_digest,
            active_binders,
            kernel_node,
            exact_presentation,
        ),
    );
    let normalization_derivation_hash = tagged_hash(
        "context-indexed-normalization",
        &(
            &parameter_telescope_digest,
            active_binders,
            exact_presentation,
            &normalized.expr,
            normalized.steps,
            fuel,
        ),
    );
    let carrier_key = tagged_hash(
        "context-indexed-carrier",
        &(
            &parameter_telescope_digest,
            active_binders,
            node,
            exact_presentation,
            &kernel_type_json,
        ),
    );
    let quotient_evidence = incomplete_quotient_evidence(
        context,
        exact_presentation,
        &normalized.expr,
        &kernel_type_json,
    );
    let every_child_context_replayed = children
        .iter()
        .all(|child| child.typed_and_normalized && child.every_child_context_replayed);
    let mut derivation = Td21V2ConstructiveDerivation {
        rule,
        former,
        exact_node: node.clone(),
        parameter_telescope_digest,
        active_binder_classifiers: active_binders.to_vec(),
        parameter_coordinates_used,
        exact_canonical_presentation: exact_presentation.clone(),
        kernel_type_json,
        normal_form: normalized.expr,
        semantic_depth: validated.semantic_depth,
        kernel_rule: kernel_node.rule.clone(),
        kernel_derivation_hash,
        normalization_derivation_hash,
        children,
        every_child_context_replayed,
        presentation_preserves_coordinate_sharing: true,
        presentation_preserves_resident_content: true,
        typed_and_normalized: true,
        carrier_key,
        quotient_evidence,
        quotient_key: None,
        derivation_hash: String::new(),
    };
    derivation.derivation_hash = tagged_hash("constructive-contextual-derivation", &derivation);
    Ok(derivation)
}

fn contextual_nonmember(reason: String) -> Td21V2ContextualMembershipDecision {
    let derivation_hash = tagged_hash("contextual-nonmember", &reason);
    Td21V2ContextualMembershipDecision {
        typed_judgment_accepted: false,
        adopted_quotient_established: false,
        domain_member: false,
        exact_obstruction: Some(reason),
        derivation: None,
        derivation_hash,
    }
}

/// Replay a judgment at an explicit dependent parameter telescope.
///
/// A successful typing derivation is deliberately distinct from membership in
/// the *quotiented* domain.  The latter remains false until the adopted
/// weakening/renaming/naturality/univalent evidence is supplied.
pub fn derive_t_d2_1_contextual_membership_v2(
    context: &Td21V2ParameterTelescope,
    node: &Td21V2OperationalNode,
) -> Result<Td21V2ContextualMembershipDecision, Td21V2Error> {
    let definition = operational_definition()?;
    let ambient_types = match validate_parameter_telescope(definition, context) {
        Ok(types) => types,
        Err(reason) => return Ok(contextual_nonmember(reason)),
    };
    let context_arity = u32::try_from(context.hypotheses.len())
        .map_err(|_| Td21V2Error::Enumeration("context arity exceeds u32".to_owned()))?;
    if let Err(reason) = validate_node(definition, node, context_arity, 0) {
        return Ok(contextual_nonmember(reason));
    }

    let derivation = if let Td21V2OperationalNode::ResidentClause {
        entry_identifier,
        clause_index,
        ..
    } = node
    {
        let index = u16::try_from(clause_index.value)
            .map_err(|_| Td21V2Error::Enumeration("clause index exceeds u16".to_owned()))?;
        let resident = definition
            .residents
            .get(&(entry_identifier.value, index))
            .ok_or_else(|| Td21V2Error::Enumeration("validated resident disappeared".to_owned()))?;
        let obstruction_evidence = tagged_hash(
            "resident-context-transport-obstruction",
            &(
                &resident.key,
                &resident.normal_form,
                &resident.kernel_type_json,
                &resident.typing_derivation_hash,
                parameter_telescope_digest(context),
            ),
        );
        return Ok(contextual_nonmember(format!(
            "sealed resident {} is authenticated with its exact normal form and kernel classifier, but the frozen resident record does not retain the exact original parameter telescope, prior-field classifiers, or a weakening/erasure transport into the requested context; accepting it here would erase dependent context (obstruction evidence {})",
            resident.key, obstruction_evidence
        )));
    } else {
        let expression = match exact_family_expression(node, context_arity, false) {
            Ok(expression) => expression,
            Err(reason) => return Ok(contextual_nonmember(reason)),
        };
        let (elaboration, kernel_derivation) = match elaborate_single_clause_with_typed_ambient(
            &expression,
            &ambient_types,
            &[],
            15,
        ) {
            Ok(result) => result,
            Err(error) => {
                return Ok(contextual_nonmember(format!(
                    "frozen typed elaborator rejected exact contextual presentation: {error}"
                )));
            }
        };
        let derivation = match build_constructive_derivation(
            definition,
            context,
            node,
            &expression,
            &kernel_derivation,
            &[],
        ) {
            Ok(derivation) => derivation,
            Err(reason) => return Ok(contextual_nonmember(reason)),
        };
        if derivation.normal_form != elaboration.normal_form
            || derivation.kernel_type_json
                != serde_json::to_string(&elaboration.kernel_ty)
                    .map_err(|error| Td21V2Error::Serialization(error.to_string()))?
        {
            return Ok(contextual_nonmember(
                "recursive derivation conclusion disagrees with whole-judgment kernel replay"
                    .to_owned(),
            ));
        }
        derivation
    };
    let adopted_quotient_established = derivation.quotient_evidence.adopted_quotient_established;
    let exact_obstruction = derivation.quotient_evidence.exact_obstruction.clone();
    let domain_member = derivation.typed_and_normalized
        && derivation.every_child_context_replayed
        && adopted_quotient_established;
    let derivation_hash = tagged_hash(
        "contextual-membership-decision",
        &(
            true,
            adopted_quotient_established,
            domain_member,
            &exact_obstruction,
            &derivation,
        ),
    );
    Ok(Td21V2ContextualMembershipDecision {
        typed_judgment_accepted: true,
        adopted_quotient_established,
        domain_member,
        exact_obstruction,
        derivation: Some(derivation),
        derivation_hash,
    })
}

pub fn replay_t_d2_1_contextual_membership_v2(
    context: &Td21V2ParameterTelescope,
    node: &Td21V2OperationalNode,
    claimed: &Td21V2ContextualMembershipDecision,
) -> Result<bool, Td21V2Error> {
    Ok(&derive_t_d2_1_contextual_membership_v2(context, node)? == claimed)
}

fn maximum_context_coordinate(node: &Td21V2OperationalNode) -> u32 {
    match node {
        Td21V2OperationalNode::ContextVariable { parameter } => parameter.value,
        Td21V2OperationalNode::FormerApplication { arguments, .. } => arguments
            .iter()
            .map(maximum_context_coordinate)
            .max()
            .unwrap_or(0),
        Td21V2OperationalNode::ResidentClause { .. }
        | Td21V2OperationalNode::BoundVariable { .. } => 0,
    }
}

fn decide_with_definition(
    definition: &OperationalDefinition,
    node: &Td21V2OperationalNode,
) -> Result<FullMembershipDecision, Td21V2Error> {
    let inferred_arity = maximum_context_coordinate(node);
    if inferred_arity > definition.bound_snapshot.maximum_free_scope_length {
        return Ok(nonmember(format!(
            "exact context coordinate {} exceeds sealed context arity {}",
            inferred_arity, definition.bound_snapshot.maximum_free_scope_length
        )));
    }
    let context = Td21V2ParameterTelescope {
        hypotheses: vec![Td21V2KernelClassifier::Type; inferred_arity as usize],
    };
    let decision = derive_t_d2_1_contextual_membership_v2(&context, node)?;
    let Some(derivation) = decision.derivation else {
        return Ok(nonmember(decision.exact_obstruction.unwrap_or_else(|| {
            "contextual derivation failed closed".to_owned()
        })));
    };
    let exact_reason = if decision.domain_member {
        "exact contextual judgment carries the complete adopted natural-family quotient evidence"
            .to_owned()
    } else {
        decision.exact_obstruction.unwrap_or_else(|| {
            "typed contextual judgment lacks adopted natural-family quotient evidence".to_owned()
        })
    };
    let public_derivation_hash = tagged_hash(
        "membership",
        &(
            decision.domain_member,
            derivation.semantic_depth,
            &derivation.carrier_key,
            &derivation.quotient_key,
            &derivation.normal_form,
            &derivation.kernel_type_json,
            &derivation.kernel_derivation_hash,
            &derivation.normalization_derivation_hash,
            &exact_reason,
        ),
    );
    Ok(FullMembershipDecision {
        public: Td21V2MembershipDecision {
            member: decision.domain_member,
            semantic_depth: Some(derivation.semantic_depth),
            carrier_key: Some(derivation.carrier_key),
            quotient_key: derivation.quotient_key,
            normal_form: Some(derivation.normal_form),
            exact_reason,
            derivation_hash: public_derivation_hash,
        },
        kernel_type_json: derivation.kernel_type_json,
        typing_derivation_hash: derivation.kernel_derivation_hash,
        normalization_derivation_hash: derivation.normalization_derivation_hash,
    })
}

static OPERATIONAL_DEFINITION: OnceLock<Result<OperationalDefinition, String>> = OnceLock::new();

fn operational_definition() -> Result<&'static OperationalDefinition, Td21V2Error> {
    match OPERATIONAL_DEFINITION
        .get_or_init(|| build_operational_definition().map_err(|error| error.to_string()))
    {
        Ok(definition) => Ok(definition),
        Err(error) => Err(Td21V2Error::Input(error.clone())),
    }
}

pub fn decide_t_d2_1_operational_membership_v2(
    node: &Td21V2OperationalNode,
) -> Result<Td21V2MembershipDecision, Td21V2Error> {
    Ok(decide_with_definition(operational_definition()?, node)?.public)
}

fn resident_node(resident: &ResidentClause) -> Td21V2OperationalNode {
    Td21V2OperationalNode::ResidentClause {
        entry_identifier: syntax_number(resident.entry_identifier, "sealed_entry_identifier"),
        candidate_digest: resident.candidate_digest.clone(),
        clause_index: syntax_number(u32::from(resident.clause_index), "sealed_clause_index"),
        declared_role: role_label(resident.declared_role),
        normal_form: resident.normal_form.clone(),
    }
}

fn context_variable(value: u32) -> Td21V2OperationalNode {
    Td21V2OperationalNode::ContextVariable {
        parameter: syntax_number(value, "sealed_maximum_free_scope_length"),
    }
}

fn bound_variable(value: u32) -> Td21V2OperationalNode {
    Td21V2OperationalNode::BoundVariable {
        binder_level: syntax_number(value, "sealed_maximum_binder_nesting"),
    }
}

fn former_node(
    former: Td21V2PublicFormer,
    arguments: Vec<Td21V2OperationalNode>,
    dimension: Option<u32>,
) -> Td21V2OperationalNode {
    Td21V2OperationalNode::FormerApplication {
        former,
        arguments,
        dimension: dimension.map(|value| syntax_number(value, "sealed_maximum_path_dimension")),
    }
}

fn representative_from_node(
    definition: &OperationalDefinition,
    node: Td21V2OperationalNode,
) -> Result<Option<Td21V2CarrierRepresentative>, Td21V2Error> {
    let decision = decide_with_definition(definition, &node)?;
    if !decision.public.member {
        return Ok(None);
    }
    Ok(Some(Td21V2CarrierRepresentative {
        carrier_key: decision
            .public
            .carrier_key
            .clone()
            .expect("member has carrier key"),
        quotient_key: decision
            .public
            .quotient_key
            .clone()
            .expect("member has quotient key"),
        semantic_depth: decision
            .public
            .semantic_depth
            .expect("member has semantic depth"),
        node,
        normal_form: decision
            .public
            .normal_form
            .clone()
            .expect("member has normal form"),
        typed: true,
        kernel_type_json: decision.kernel_type_json,
        typing_derivation_hash: decision.typing_derivation_hash,
        normalization_derivation_hash: decision.normalization_derivation_hash,
    }))
}

fn build_explicit_regression_basis(
    definition: &OperationalDefinition,
) -> Result<Vec<Td21V2CarrierRepresentative>, Td21V2Error> {
    use Td21V2PublicFormer as F;
    let mut nodes = definition
        .residents
        .values()
        .map(resident_node)
        .collect::<Vec<_>>();
    nodes.push(context_variable(1));
    nodes.push(former_node(F::AmbientUniverse, Vec::new(), None));
    for dimension in 0..=definition.bound_snapshot.maximum_path_dimension {
        nodes.push(former_node(F::PathConstructor, Vec::new(), Some(dimension)));
    }
    let binder_body = if definition.bound_snapshot.maximum_binder_nesting > 0 {
        bound_variable(1)
    } else {
        context_variable(1)
    };
    nodes.extend([
        former_node(F::LambdaIntroduction, vec![binder_body.clone()], None),
        former_node(
            F::Application,
            vec![context_variable(1), context_variable(2)],
            None,
        ),
        former_node(
            F::PiFormation,
            vec![context_variable(1), binder_body.clone()],
            None,
        ),
        former_node(
            F::SigmaFormation,
            vec![context_variable(1), binder_body],
            None,
        ),
        former_node(
            F::IdentityFormation,
            vec![
                context_variable(1),
                context_variable(2),
                context_variable(3),
            ],
            None,
        ),
        former_node(F::ReflexivityIntroduction, vec![context_variable(1)], None),
        former_node(F::SuspensionFormation, vec![context_variable(1)], None),
        former_node(F::TruncationFormation, vec![context_variable(1)], None),
        former_node(F::FlatFormation, vec![context_variable(1)], None),
        former_node(F::SharpFormation, vec![context_variable(1)], None),
        former_node(F::DiscreteFormation, vec![context_variable(1)], None),
        former_node(F::ShapeFormation, vec![context_variable(1)], None),
        former_node(F::NextFormation, vec![context_variable(1)], None),
        former_node(F::EventuallyFormation, vec![context_variable(1)], None),
    ]);
    let mut representatives = Vec::new();
    let mut seen_carrier_keys = BTreeSet::new();
    for node in nodes {
        if let Some(representative) = representative_from_node(definition, node)?
            && seen_carrier_keys.insert(representative.carrier_key.clone())
        {
            representatives.push(representative);
        }
    }
    representatives.sort_by(|left, right| left.carrier_key.cmp(&right.carrier_key));
    Ok(representatives)
}

fn quotient_collision_replay(
    representatives: &[Td21V2CarrierRepresentative],
) -> Result<bool, Td21V2Error> {
    let mut by_key = BTreeMap::<&str, Vec<&Td21V2CarrierRepresentative>>::new();
    for representative in representatives {
        by_key
            .entry(&representative.quotient_key)
            .or_default()
            .push(representative);
    }
    for rows in by_key.values() {
        for left in rows {
            for right in rows {
                let equality = univalent_equality(&left.normal_form, &right.normal_form, 9, 64)
                    .map_err(|error| {
                        Td21V2Error::Enumeration(format!(
                            "basis quotient collision replay failed: {error}"
                        ))
                    })?;
                if !equality.equal || left.kernel_type_json != right.kernel_type_json {
                    return Ok(false);
                }
            }
        }
    }
    Ok(true)
}

fn value_array<'a>(value: &'a Value, pointer: &str) -> Result<&'a Vec<Value>, Td21V2Error> {
    value
        .pointer(pointer)
        .and_then(Value::as_array)
        .ok_or_else(|| Td21V2Error::Input(format!("observer JSON is missing array at {pointer}")))
}

fn value_u32(value: &Value, field: &str) -> Option<u32> {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|number| u32::try_from(number).ok())
}

fn exact_observer_judgment(
    value: &Value,
) -> Option<(Td21V2ParameterTelescope, Td21V2OperationalNode)> {
    let context = serde_json::from_value(value.get("t_d2_1_parameter_telescope")?.clone()).ok()?;
    let node = serde_json::from_value(value.get("t_d2_1_operational_node")?.clone()).ok()?;
    Some((context, node))
}

fn family_regression(definition: &OperationalDefinition) -> Result<(u32, u32, bool), Td21V2Error> {
    if sha256(FAMILY_CORPUS_BYTES) != FAMILY_CORPUS_SHA256 {
        return Ok((0, 0, false));
    }
    let corpus: Value = serde_json::from_slice(FAMILY_CORPUS_BYTES)
        .map_err(|error| Td21V2Error::Serialization(error.to_string()))?;
    let packages = value_array(&corpus, "/intrinsic_sequence/packages")?;
    let mut observed = 0_u32;
    let mut recognized = 0_u32;
    for package in packages {
        let stage = value_u32(package, "stage");
        let candidate_hash = package.get("candidate_hash").and_then(Value::as_str);
        let family_rows = package
            .get("family_rows")
            .and_then(Value::as_array)
            .ok_or_else(|| Td21V2Error::Input("family package lacks family_rows".to_owned()))?;
        let role_rows = package
            .get("role_rows")
            .and_then(Value::as_array)
            .ok_or_else(|| Td21V2Error::Input("family package lacks role_rows".to_owned()))?;
        for role in role_rows {
            if role.get("resolution_class").and_then(Value::as_str) != Some("proved_family") {
                continue;
            }
            observed = observed.saturating_add(1);
            let family_id = role.get("resolved_family_id").and_then(Value::as_str);
            let family = family_id.and_then(|family_id| {
                family_rows.iter().find(|family| {
                    family.get("family_id").and_then(Value::as_str) == Some(family_id)
                })
            });
            let exact_judgment = family.and_then(exact_observer_judgment);
            let exact_candidate = stage.zip(candidate_hash).is_some_and(|(stage, hash)| {
                definition.residents.values().any(|resident| {
                    resident.entry_identifier == stage && resident.candidate_digest == hash
                })
            });
            let exact_quotient_inputs = family.is_some_and(|family| {
                family
                    .get("parameter_telescope_naturality_derivation_hash")
                    .and_then(Value::as_str)
                    .is_some_and(|hash| !hash.is_empty())
                    && family
                        .get("canonical_weakening_renaming_derivation_hash")
                        .and_then(Value::as_str)
                        .is_some_and(|hash| !hash.is_empty())
                    && family
                        .get("univalent_equality_witness_family_hash")
                        .and_then(Value::as_str)
                        .is_some_and(|hash| !hash.is_empty())
            });
            let member = if let Some((context, node)) = exact_judgment {
                derive_t_d2_1_contextual_membership_v2(&context, &node)?.domain_member
            } else {
                false
            };
            if role.get("resolved").and_then(Value::as_bool) == Some(true)
                && exact_candidate
                && exact_quotient_inputs
                && member
            {
                recognized = recognized.saturating_add(1);
            }
        }
    }
    Ok((
        observed,
        recognized,
        observed == EXPECTED_PROVED_FAMILY_COUNT && recognized == observed,
    ))
}

fn membership_row_regression(
    definition: &OperationalDefinition,
) -> Result<(u32, u32, bool), Td21V2Error> {
    if sha256(A3_CORPUS_BYTES) != A3_CORPUS_SHA256
        || sha256(MEMBERSHIP_CORPUS_BYTES) != MEMBERSHIP_CORPUS_SHA256
    {
        return Ok((0, 0, false));
    }
    let a3: Value = serde_json::from_slice(A3_CORPUS_BYTES)
        .map_err(|error| Td21V2Error::Serialization(error.to_string()))?;
    let membership: Value = serde_json::from_slice(MEMBERSHIP_CORPUS_BYTES)
        .map_err(|error| Td21V2Error::Serialization(error.to_string()))?;
    let window = value_array(&a3, "/historical_windows")?
        .iter()
        .find(|window| value_u32(window, "stage") == Some(16))
        .ok_or_else(|| Td21V2Error::Input("A3 corpus lacks Stage-16 window".to_owned()))?;
    let seeds = window
        .get("base_seed_dispositions")
        .and_then(Value::as_array)
        .ok_or_else(|| Td21V2Error::Input("Stage-16 A3 window lacks base seeds".to_owned()))?;
    let membership_rows = value_array(&membership, "/stage16/membership_rows")?;
    let membership_by_id = membership_rows
        .iter()
        .filter_map(|row| {
            row.get("a3_instance_id")
                .and_then(Value::as_str)
                .map(|id| (id, row))
        })
        .collect::<BTreeMap<_, _>>();
    let mut observed = 0_u32;
    let mut recognized = 0_u32;
    for seed in seeds {
        if seed.get("eligible").and_then(Value::as_bool) != Some(true) {
            continue;
        }
        observed = observed.saturating_add(1);
        let instance_id = seed.get("promoted_instance_id").and_then(Value::as_str);
        let exact_sources = seed
            .get("raw_sources")
            .and_then(Value::as_array)
            .is_some_and(|sources| {
                !sources.is_empty()
                    && sources.iter().all(|source| {
                        let step = value_u32(source, "step");
                        let clause = value_u32(source, "clause_index")
                            .and_then(|value| u16::try_from(value).ok());
                        let candidate = source.get("candidate_hash").and_then(Value::as_str);
                        step.zip(clause)
                            .and_then(|coordinate| definition.residents.get(&coordinate))
                            .zip(candidate)
                            .is_some_and(|(resident, candidate)| {
                                resident.candidate_digest == candidate
                            })
                    })
            });
        let typed_row = instance_id.and_then(|id| membership_by_id.get(id).copied());
        let typed_row_testimony = typed_row.is_some_and(|row| {
            row.get("hypothetical_derivation_replayed")
                .and_then(Value::as_bool)
                == Some(true)
                && row.get("output_kernel_typed").and_then(Value::as_bool) == Some(true)
                && row
                    .get("uniform_specialization_not_new_family")
                    .and_then(Value::as_bool)
                    == Some(true)
                && row
                    .get("d_membership_derivation_hash")
                    .and_then(Value::as_str)
                    .is_some_and(|hash| !hash.is_empty())
        });
        let exact_judgment = typed_row.and_then(exact_observer_judgment);
        let exact_instance_link = typed_row.is_some_and(|row| {
            row.get("exact_instance_substitution_derivation_hash")
                .and_then(Value::as_str)
                .is_some_and(|hash| !hash.is_empty())
                && row
                    .get("exact_output_presentation_digest")
                    .and_then(Value::as_str)
                    .is_some_and(|hash| !hash.is_empty())
        });
        let domain_member = if let Some((context, node)) = exact_judgment {
            derive_t_d2_1_contextual_membership_v2(&context, &node)?.domain_member
        } else {
            false
        };
        if exact_sources
            && typed_row_testimony
            && exact_instance_link
            && domain_member
            && seed
                .get("every_source_joined_to_exact_typed_public_clause")
                .and_then(Value::as_bool)
                == Some(true)
            && seed
                .get("required_output_shape_replayed")
                .and_then(Value::as_bool)
                == Some(true)
        {
            recognized = recognized.saturating_add(1);
        }
    }
    let exact_membership_surface = membership_rows.len() == EXPECTED_MEMBERSHIP_ROW_COUNT as usize
        && membership
            .pointer("/stage16/every_instance_d_decided")
            .and_then(Value::as_bool)
            == Some(true)
        && membership
            .pointer("/stage16/d_partition_complete")
            .and_then(Value::as_bool)
            == Some(true);
    Ok((
        observed,
        recognized,
        exact_membership_surface
            && observed == EXPECTED_MEMBERSHIP_ROW_COUNT
            && recognized == observed,
    ))
}

fn definition_sources_authenticated() -> bool {
    [
        (
            OPERATIONAL_ADJUDICATION_BYTES,
            OPERATIONAL_ADJUDICATION_SHA256,
        ),
        (PARENT_ADJUDICATION_BYTES, PARENT_ADJUDICATION_SHA256),
        (
            AMBIENT_FORMER_ADJUDICATION_BYTES,
            AMBIENT_FORMER_ADJUDICATION_SHA256,
        ),
        (
            DEPENDENT_CONTEXT_ADJUDICATION_BYTES,
            DEPENDENT_CONTEXT_ADJUDICATION_SHA256,
        ),
        (QUOTIENT_ADJUDICATION_BYTES, QUOTIENT_ADJUDICATION_SHA256),
        (OPEN_PROBLEM_BYTES, OPEN_PROBLEM_SHA256),
        (EXPR_SOURCE_BYTES, EXPR_SOURCE_SHA256),
        (ELABORATE_SOURCE_BYTES, ELABORATE_SOURCE_SHA256),
        (AMBIENT_FORMER_SOURCE_BYTES, AMBIENT_FORMER_SOURCE_SHA256),
        (CONTEXT_SOURCE_BYTES, CONTEXT_SOURCE_SHA256),
        (NORMALIZE_SOURCE_BYTES, NORMALIZE_SOURCE_SHA256),
        (EQUALITY_SOURCE_BYTES, EQUALITY_SOURCE_SHA256),
    ]
    .into_iter()
    .all(|(bytes, expected)| sha256(bytes) == expected)
}

fn named_gap(id: &str, phase: &str, exact_obstruction: String) -> Td21V2NamedGap {
    let derivation_hash = tagged_hash("named-gap", &(id, phase, &exact_obstruction));
    Td21V2NamedGap {
        id: id.to_owned(),
        phase: phase.to_owned(),
        exact_obstruction,
        keeps_t_d2_2_closed: true,
        keeps_m4_unauthorized: true,
        zero_charge: true,
        derivation_hash,
    }
}

fn certificate_digest(certificate: &Td21OperationalDomainV2Certificate) -> String {
    let mut payload = certificate.clone();
    payload.result_digest.clear();
    tagged_hash("certificate", &payload)
}

fn build_certificate() -> Result<Td21OperationalDomainV2Certificate, Td21V2Error> {
    let definition = operational_definition()?;
    let definition_sources_ok = definition_sources_authenticated();
    let bounds_match_sealed_regression = definition.bound_snapshot.public_entry_count
        == EXPECTED_PUBLIC_ENTRY_COUNT
        && definition.bound_snapshot.public_clause_count == EXPECTED_PUBLIC_CLAUSE_COUNT
        && definition.bound_snapshot.maximum_ambient_parameter_count
            == EXPECTED_MAXIMUM_AMBIENT_PARAMETERS
        && definition.bound_snapshot.maximum_free_scope_length
            == EXPECTED_MAXIMUM_FREE_SCOPE_LENGTH
        && definition.bound_snapshot.maximum_path_dimension == EXPECTED_MAXIMUM_PATH_DIMENSION
        && definition.bound_snapshot.semantic_former_depth == SEMANTIC_FORMER_DEPTH_BOUND
        && definition.bound_snapshot.transparent_former_count == 17;
    let quotient_representatives = build_explicit_regression_basis(definition)?;
    let quotient_representatives_extensionally_complete = false;
    let basis_quotient_collision_replay = quotient_collision_replay(&quotient_representatives)?;
    let quotient_representative_count = u32::try_from(
        quotient_representatives
            .iter()
            .map(|row| row.quotient_key.as_str())
            .collect::<BTreeSet<_>>()
            .len(),
    )
    .map_err(|_| Td21V2Error::Enumeration("basis quotient count exceeds u32".to_owned()))?;
    let carrier_representative_count = u32::try_from(quotient_representatives.len())
        .map_err(|_| Td21V2Error::Enumeration("basis carrier count exceeds u32".to_owned()))?;

    // The seal is fixed before either corpus function below is called.
    let pre_corpus_carrier_seal = tagged_hash(
        "pre-corpus-carrier-seal",
        &(
            &definition.signature_digest,
            &definition.bound_snapshot.derivation_hash,
            &definition.carrier_definition_digest,
            &definition.grammar.derivation_hash,
            &definition.induction.derivation_hash,
            quotient_representatives
                .iter()
                .map(|row| (&row.carrier_key, &row.quotient_key))
                .collect::<Vec<_>>(),
        ),
    );

    let (observed_family_count, recognized_family_count, family_regression_passed) =
        family_regression(definition)?;
    let (
        observed_membership_row_count,
        recognized_membership_row_count,
        membership_row_regression_passed,
    ) = membership_row_regression(definition)?;
    let observer_bindings = observer_source_bindings();
    let mut source_bindings = definition_source_bindings();
    source_bindings.extend(observer_bindings);

    let fixed_fragment_operationally_defined = definition_sources_ok
        && bounds_match_sealed_regression
        && definition
            .bound_snapshot
            .all_citations_resolved_before_corpus_access
        && definition
            .context_finiteness
            .finite_inductive_grammar_proved
        && definition.grammar.finite_enumerator_constructed
        && definition.induction.induction_total;
    let membership_decidable = fixed_fragment_operationally_defined
        && definition.grammar.membership_elimination_total
        && definition
            .context_finiteness
            .structural_membership_decidable;
    // Universal conclusions depend only on the constructor theorem witness.
    // The explicit vector below is structural testimony and never participates
    // in these gates, even negatively.
    let typed_normalization_total = membership_decidable && definition.induction.induction_total;
    let quotient_decidable = typed_normalization_total
        && definition
            .grammar
            .quotient_key_decision_total_after_typed_normalization
        && definition
            .induction
            .quotient_respects_constructor_congruence;
    let quotient_finitely_enumerable = quotient_decidable
        && definition.grammar.finite_enumerator_constructed
        && definition.induction.induction_total;

    let mut normalization_proof = Td21V2NormalizationProof {
        induction_measure: "Implemented concrete recursion is lexicographic in the operational constructor subtree with the exact binder prefix carried to each child. The required universal induction over dependent contexts and nested resident content is not yet proved; no regression sample is promoted to that theorem.".to_owned(),
        every_recursive_call_strictly_decreases: true,
        resident_normal_forms_replayed: definition.residents.len()
            == EXPECTED_PUBLIC_CLAUSE_COUNT as usize,
        every_enumerated_representative_typed: !quotient_representatives.is_empty()
            && quotient_representatives.iter().all(|row| row.typed),
        every_enumerated_representative_normalized: !quotient_representatives.is_empty()
            && quotient_representatives
                .iter()
                .all(|row| !row.normalization_derivation_hash.is_empty()),
        total_on_bounded_carrier: typed_normalization_total,
        derivation_hash: String::new(),
    };
    normalization_proof.derivation_hash = tagged_hash(
        "normalization-proof",
        &(
            &normalization_proof.induction_measure,
            normalization_proof.every_recursive_call_strictly_decreases,
            normalization_proof.resident_normal_forms_replayed,
            normalization_proof.every_enumerated_representative_typed,
            normalization_proof.every_enumerated_representative_normalized,
            normalization_proof.total_on_bounded_carrier,
            sha256(NORMALIZE_SOURCE_BYTES),
        ),
    );
    let mut quotient_proof = Td21V2QuotientProof {
        equality_procedure: KERNEL_EQUALITY_PROCEDURE.to_owned(),
        canonical_parameter_renaming_total: false,
        uniform_specializations_not_multiplied: false,
        structural_equality_decidable: false,
        every_carrier_row_has_one_quotient_key: false,
        quotient_key_collisions_replayed_by_equality: false,
        quotient_decidable,
        quotient_finitely_enumerable,
        derivation_hash: String::new(),
    };
    quotient_proof.derivation_hash = tagged_hash(
        "quotient-proof",
        &(
            &quotient_proof.equality_procedure,
            quotient_proof.canonical_parameter_renaming_total,
            quotient_proof.uniform_specializations_not_multiplied,
            quotient_proof.structural_equality_decidable,
            quotient_proof.every_carrier_row_has_one_quotient_key,
            quotient_proof.quotient_key_collisions_replayed_by_equality,
            quotient_proof.quotient_decidable,
            quotient_proof.quotient_finitely_enumerable,
            basis_quotient_collision_replay,
            &definition
                .grammar
                .legacy_node_shape_recurrence_diagnostic_decimal,
        ),
    );
    let definition_inputs = definition_source_bindings()
        .into_iter()
        .map(|binding| binding.path)
        .collect::<Vec<_>>();
    let observer_inputs = observer_source_bindings()
        .into_iter()
        .map(|binding| binding.path)
        .collect::<Vec<_>>();
    let mut independence_firewall = Td21V2IndependenceFirewall {
        definition_inputs,
        observer_inputs,
        bounds_frozen_before_corpus_access: true,
        carrier_sealed_before_corpus_access: true,
        family_corpus_reaches_carrier_definition: false,
        a3_corpus_reaches_carrier_definition: false,
        raw_ast_height_cap_used: false,
        generator_inventory_used_as_carrier: false,
        dataflow_disjoint: true,
        derivation_hash: String::new(),
    };
    independence_firewall.derivation_hash = tagged_hash(
        "independence-firewall",
        &(
            &independence_firewall.definition_inputs,
            &independence_firewall.observer_inputs,
            &pre_corpus_carrier_seal,
            independence_firewall.bounds_frozen_before_corpus_access,
            independence_firewall.carrier_sealed_before_corpus_access,
            independence_firewall.family_corpus_reaches_carrier_definition,
            independence_firewall.a3_corpus_reaches_carrier_definition,
            independence_firewall.raw_ast_height_cap_used,
            independence_firewall.generator_inventory_used_as_carrier,
            independence_firewall.dataflow_disjoint,
        ),
    );
    let mut regression = Td21V2Regression {
        corpus_opened_after_pre_corpus_carrier_seal: true,
        family_corpus_authenticated: sha256(FAMILY_CORPUS_BYTES) == FAMILY_CORPUS_SHA256,
        expected_family_count: EXPECTED_PROVED_FAMILY_COUNT,
        observed_family_count,
        recognized_family_count,
        every_family_recognized: family_regression_passed,
        membership_row_corpus_authenticated: sha256(A3_CORPUS_BYTES) == A3_CORPUS_SHA256
            && sha256(MEMBERSHIP_CORPUS_BYTES) == MEMBERSHIP_CORPUS_SHA256,
        expected_membership_row_count: EXPECTED_MEMBERSHIP_ROW_COUNT,
        observed_membership_row_count,
        recognized_membership_row_count,
        every_membership_row_recognized: membership_row_regression_passed,
        corpus_rows_seeded_or_bounded_carrier: false,
        derivation_hash: String::new(),
    };
    regression.derivation_hash = tagged_hash(
        "post-seal-regression",
        &(
            &pre_corpus_carrier_seal,
            regression.corpus_opened_after_pre_corpus_carrier_seal,
            regression.family_corpus_authenticated,
            regression.expected_family_count,
            regression.observed_family_count,
            regression.recognized_family_count,
            regression.every_family_recognized,
            regression.membership_row_corpus_authenticated,
            regression.expected_membership_row_count,
            regression.observed_membership_row_count,
            regression.recognized_membership_row_count,
            regression.every_membership_row_recognized,
            regression.corpus_rows_seeded_or_bounded_carrier,
        ),
    );

    let mut open_gaps = Vec::new();
    if !definition_sources_ok
        || !definition
            .bound_snapshot
            .all_citations_resolved_before_corpus_access
    {
        open_gaps.push(named_gap(
            "TD21_V2_BOUND_CITATION_UNRESOLVED",
            "bound_extraction",
            "At least one adopted definition or bound citation no longer matches its frozen digest."
                .to_owned(),
        ));
    }
    if !bounds_match_sealed_regression {
        open_gaps.push(named_gap(
            "TD21_V2_SEALED_BOUND_REGRESSION_DIVERGENCE",
            "bound_extraction",
            format!(
                "Recomputed H15 interface is entries/clauses/ambient/scope/dimension = {}/{}/{}/{}/{}, expected 15/64/2/9/3.",
                definition.bound_snapshot.public_entry_count,
                definition.bound_snapshot.public_clause_count,
                definition.bound_snapshot.maximum_ambient_parameter_count,
                definition.bound_snapshot.maximum_free_scope_length,
                definition.bound_snapshot.maximum_path_dimension,
            ),
        ));
    }
    if !family_regression_passed {
        open_gaps.push(named_gap(
            "TD21_V2_61_FAMILY_REGRESSION_DIVERGENCE",
            "post_seal_regression",
            format!(
                "Recognized {recognized_family_count}/{observed_family_count} observed proved families; expected 61/61. Each successor family row must export its exact t_d2_1_parameter_telescope, t_d2_1_operational_node, canonical weakening/renaming derivation, naturality derivation, and univalent-equality witness family; the authenticated archive currently exports only family/shape identifiers and aggregate proof hashes."
            ),
        ));
    }
    if !membership_row_regression_passed {
        open_gaps.push(named_gap(
            "TD21_V2_89_MEMBERSHIP_REGRESSION_DIVERGENCE",
            "post_seal_regression",
            format!(
                "Recognized {recognized_membership_row_count}/{observed_membership_row_count} typed membership rows; expected 89/89. Each successor row must export its exact t_d2_1_parameter_telescope, t_d2_1_operational_node, exact instance-substitution derivation, and output-presentation digest linked to the family quotient; generic unary/chronological shape labels are not presentations."
            ),
        ));
    }
    if !definition
        .context_finiteness
        .finite_inductive_grammar_proved
    {
        open_gaps.push(named_gap(
            "TD21_V2_DEPENDENT_CONTEXT_ENUMERATOR_GAP",
            "context_finiteness",
            "The symbolic node grammar does not enumerate exact sequential dependent parameter telescopes. The contextual API checks Type and earlier-prefix Element classifiers, but the frozen public kernel lacks a declaration checker for Function, PathDeclaration, and Neutral hypotheses; therefore the full adopted context fragment is not finitely enumerated."
                .to_owned(),
        ));
    }
    if !definition.induction.induction_total {
        open_gaps.push(named_gap(
            "TD21_V2_CONSTRUCTOR_NORMALIZATION_INDUCTION_GAP",
            "typed_normalization",
            "Concrete nonresident contextual judgments now carry recursive constructor derivations, and binder children replay under their active binder prefixes. No universal per-constructor theorem covers the entire symbolic carrier: the sealed resident record lacks its exact original dependent telescope and weakening/erasure transport, and a resident nested below a former also has no exact clause-level Expr/kernel embedding. The non-extensional regression basis is not used to infer totality."
                .to_owned(),
        ));
    }
    if !definition
        .induction
        .quotient_respects_constructor_congruence
    {
        open_gaps.push(named_gap(
            "TD21_V2_ADOPTED_NATURAL_FAMILY_QUOTIENT_GAP",
            "quotient",
            "No replayable theorem currently supplies the adopted quotient's parameter telescope, checked weakening and canonical renaming maps, naturality derivation, and frozen univalent-equality witness family for every contextual judgment. Normal-form and kernel-type hashes are explicitly not accepted as quotient evidence."
                .to_owned(),
        ));
    }
    if !quotient_finitely_enumerable {
        open_gaps.push(named_gap(
            "TD21_V2_QUOTIENT_THEOREM_OBLIGATION_FAILED",
            "quotient",
            "Universal typed normalization and the adopted quotient decision are unproved, and no finite enumerator for contextual quotient judgments has been constructed."
                .to_owned(),
        ));
    }
    let t_d2_2_prerequisite_satisfied = fixed_fragment_operationally_defined
        && membership_decidable
        && typed_normalization_total
        && quotient_decidable
        && quotient_finitely_enumerable
        && family_regression_passed
        && membership_row_regression_passed
        && open_gaps.is_empty();
    let status = if t_d2_2_prerequisite_satisfied {
        Td21V2RunStatus::Passed
    } else {
        Td21V2RunStatus::StoppedNamedGaps
    };
    let mut certificate = Td21OperationalDomainV2Certificate {
        schema: T_D2_1_OPERATIONAL_DOMAIN_V2_SCHEMA.to_owned(),
        date: T_D2_1_OPERATIONAL_DOMAIN_V2_DATE.to_owned(),
        source_bindings,
        history_signature_digest: definition.signature_digest.clone(),
        bound_snapshot_digest: definition.bound_snapshot.derivation_hash.clone(),
        pre_corpus_carrier_seal,
        carrier_definition_digest: definition.carrier_definition_digest.clone(),
        bound_snapshot: definition.bound_snapshot.clone(),
        context_finiteness: definition.context_finiteness.clone(),
        normalization_proof,
        quotient_proof,
        symbolic_carrier_grammar: definition.grammar.clone(),
        symbolic_carrier_induction: definition.induction.clone(),
        independence_firewall,
        quotient_representatives,
        quotient_representatives_extensionally_complete,
        carrier_representative_count,
        quotient_representative_count,
        regression,
        open_gaps,
        fixed_fragment_operationally_defined,
        membership_decidable,
        typed_normalization_total,
        quotient_decidable,
        quotient_finitely_enumerable,
        family_regression_passed,
        membership_row_regression_passed,
        t_d2_2_prerequisite_satisfied,
        m4_authorized: false,
        status,
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

static EXPECTED_CERTIFICATE: OnceLock<Result<Td21OperationalDomainV2Certificate, String>> =
    OnceLock::new();

fn expected_certificate() -> Result<&'static Td21OperationalDomainV2Certificate, Td21V2Error> {
    match EXPECTED_CERTIFICATE
        .get_or_init(|| build_certificate().map_err(|error| error.to_string()))
    {
        Ok(certificate) => Ok(certificate),
        Err(error) => Err(Td21V2Error::Input(error.clone())),
    }
}

pub fn issue_t_d2_1_operational_domain_v2()
-> Result<Td21OperationalDomainV2Certificate, Td21V2Error> {
    expected_certificate().cloned()
}

fn invalid_replay(error: impl Into<String>) -> Td21V2Replay {
    Td21V2Replay {
        valid: false,
        errors: vec![error.into()],
        status: Td21V2RunStatus::StoppedNamedGaps,
        fixed_fragment_operationally_defined: false,
        membership_decidable: false,
        quotient_finitely_enumerable: false,
        family_regression_passed: false,
        membership_row_regression_passed: false,
        t_d2_2_prerequisite_satisfied: false,
        m4_authorized: false,
    }
}

pub fn replay_t_d2_1_operational_domain_v2(
    claimed: &Td21OperationalDomainV2Certificate,
) -> Td21V2Replay {
    let expected = match expected_certificate() {
        Ok(certificate) => certificate,
        Err(error) => return invalid_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("T-D2-1 v2 result digest mismatch".to_owned());
    }
    if claimed != expected {
        errors.push("T-D2-1 v2 certificate differs from deterministic reissuance".to_owned());
    }
    if !errors.is_empty() {
        let mut replay = invalid_replay(errors.join("; "));
        replay.errors = errors;
        return replay;
    }
    Td21V2Replay {
        valid: true,
        errors,
        status: expected.status,
        fixed_fragment_operationally_defined: expected.fixed_fragment_operationally_defined,
        membership_decidable: expected.membership_decidable,
        quotient_finitely_enumerable: expected.quotient_finitely_enumerable,
        family_regression_passed: expected.family_regression_passed,
        membership_row_regression_passed: expected.membership_row_regression_passed,
        t_d2_2_prerequisite_satisfied: expected.t_d2_2_prerequisite_satisfied,
        m4_authorized: expected.m4_authorized,
    }
}

pub fn replay_t_d2_1_operational_domain_v2_json(bytes: &[u8]) -> Result<Td21V2Replay, Td21V2Error> {
    let certificate = serde_json::from_slice(bytes)
        .map_err(|error| Td21V2Error::Serialization(error.to_string()))?;
    Ok(replay_t_d2_1_operational_domain_v2(&certificate))
}

pub fn render_t_d2_1_operational_domain_v2(
    certificate: &Td21OperationalDomainV2Certificate,
) -> String {
    let gaps = if certificate.open_gaps.is_empty() {
        "none".to_owned()
    } else {
        certificate
            .open_gaps
            .iter()
            .map(|gap| format!("- `{}`: {}", gap.id, gap.exact_obstruction))
            .collect::<Vec<_>>()
            .join("\n")
    };
    format!(
        "# T-D2-1 operational depth-two domain v2\n\n\
**Date:** {}. **Status:** `{:?}`. **Certificate:** `{}`.\n\n\
The H15 public boundary replays at {}/{} entries/clauses, maximum ambient/free scope {}/{}, binder nesting {}, and cubical dimension {}. Every bound citation resolved before the regression corpora were opened.\n\n\
The context-indexed API preserves repeated coordinates and replays binder children under their actual binder prefixes. Its explicit {}-row vector is non-extensional structural testimony only. The grammar's legacy node-shape recurrence ({}) omits dependent-telescope enumeration and is not a carrier or quotient bound. Universal membership, typed normalization, adopted-quotient decision, and finite enumeration are `{}`, `{}`, `{}`, `{}`; none is inferred from that vector.\n\n\
Post-seal exact-content regressions recognize {}/{} proved families and {}/{} exact typed A3 rows. The archived corpora authenticate the counts but do not carry the parameter telescopes, canonical presentations, instance substitutions, or naturality/univalent witnesses needed to recognize them as domain judgments.\n\n\
## Named gaps and required successor contract\n\n{}\n\n\
T-D2-2 prerequisite: `{}`. M-4 remains unauthorized: `{}`.\n",
        certificate.date,
        certificate.status,
        certificate.result_digest,
        certificate.bound_snapshot.public_entry_count,
        certificate.bound_snapshot.public_clause_count,
        certificate.bound_snapshot.maximum_ambient_parameter_count,
        certificate.bound_snapshot.maximum_free_scope_length,
        certificate.bound_snapshot.maximum_binder_nesting,
        certificate.bound_snapshot.maximum_path_dimension,
        certificate.carrier_representative_count,
        certificate
            .symbolic_carrier_grammar
            .legacy_node_shape_recurrence_diagnostic_decimal,
        certificate.membership_decidable,
        certificate.typed_normalization_total,
        certificate.quotient_decidable,
        certificate.quotient_finitely_enumerable,
        certificate.regression.recognized_family_count,
        certificate.regression.expected_family_count,
        certificate.regression.recognized_membership_row_count,
        certificate.regression.expected_membership_row_count,
        gaps,
        certificate.t_d2_2_prerequisite_satisfied,
        certificate.m4_authorized,
    )
}

pub fn emit_t_d2_1_operational_domain_v2_create_new(
    certificate_path: &Path,
    report_path: &Path,
) -> Result<Td21OperationalDomainV2Certificate, Td21V2Error> {
    if certificate_path.exists() || report_path.exists() {
        return Err(Td21V2Error::Input(
            "create-new target already exists; no artifact was overwritten".to_owned(),
        ));
    }
    let certificate = issue_t_d2_1_operational_domain_v2()?;
    let replay = replay_t_d2_1_operational_domain_v2(&certificate);
    if !replay.valid {
        return Err(Td21V2Error::Input(format!(
            "new certificate failed replay: {}",
            replay.errors.join("; ")
        )));
    }
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| Td21V2Error::Serialization(error.to_string()))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(certificate_path)
        .map_err(|error| Td21V2Error::Input(error.to_string()))?;
    if let Err(error) = file.write_all(&bytes).and_then(|_| file.write_all(b"\n")) {
        drop(file);
        let _ = remove_file(certificate_path);
        return Err(Td21V2Error::Input(error.to_string()));
    }
    drop(file);
    let report = render_t_d2_1_operational_domain_v2(&certificate);
    if let Err(error) = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(report_path)
        .and_then(|mut file| file.write_all(report.as_bytes()))
    {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(Td21V2Error::Input(error.to_string()));
    }
    let emitted =
        read_to_string(certificate_path).map_err(|error| Td21V2Error::Input(error.to_string()))?;
    let emitted_replay = replay_t_d2_1_operational_domain_v2_json(emitted.as_bytes())?;
    if !emitted_replay.valid {
        let _ = remove_file(certificate_path);
        let _ = remove_file(report_path);
        return Err(Td21V2Error::Input(
            "emitted JSON failed deterministic replay".to_owned(),
        ));
    }
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operational_domain_issues_and_replays() {
        let certificate = issue_t_d2_1_operational_domain_v2().expect("issuer");
        let replay = replay_t_d2_1_operational_domain_v2(&certificate);
        assert!(replay.valid, "{:?}", replay.errors);
        assert_eq!(certificate.bound_snapshot.public_entry_count, 15);
        assert_eq!(certificate.bound_snapshot.public_clause_count, 64);
        assert_eq!(
            certificate.bound_snapshot.maximum_ambient_parameter_count,
            2
        );
        assert_eq!(certificate.bound_snapshot.maximum_free_scope_length, 9);
        assert_eq!(certificate.bound_snapshot.maximum_path_dimension, 3);
        assert!(!certificate.quotient_representatives_extensionally_complete);
        assert!(!certificate.symbolic_carrier_induction.induction_total);
        assert_eq!(certificate.regression.observed_family_count, 61);
        assert_eq!(certificate.regression.observed_membership_row_count, 89);
        assert_eq!(certificate.regression.recognized_family_count, 0);
        assert_eq!(certificate.regression.recognized_membership_row_count, 0);
        assert!(!certificate.family_regression_passed);
        assert!(!certificate.membership_row_regression_passed);
        assert!(!certificate.typed_normalization_total);
        assert!(!certificate.quotient_decidable);
        assert!(!certificate.quotient_finitely_enumerable);
        assert!(!certificate.t_d2_2_prerequisite_satisfied);
        assert_eq!(certificate.status, Td21V2RunStatus::StoppedNamedGaps);
        let gap_ids = certificate
            .open_gaps
            .iter()
            .map(|gap| gap.id.as_str())
            .collect::<BTreeSet<_>>();
        assert!(gap_ids.contains("TD21_V2_61_FAMILY_REGRESSION_DIVERGENCE"));
        assert!(gap_ids.contains("TD21_V2_89_MEMBERSHIP_REGRESSION_DIVERGENCE"));
        assert!(gap_ids.contains("TD21_V2_DEPENDENT_CONTEXT_ENUMERATOR_GAP"));
        assert!(gap_ids.contains("TD21_V2_CONSTRUCTOR_NORMALIZATION_INDUCTION_GAP"));
        assert!(gap_ids.contains("TD21_V2_ADOPTED_NATURAL_FAMILY_QUOTIENT_GAP"));
        assert!(
            !certificate
                .independence_firewall
                .definition_inputs
                .iter()
                .any(|path| path.ends_with("t_d2_1_operational_domain_v2.rs"))
        );
        assert!(
            certificate
                .independence_firewall
                .observer_inputs
                .iter()
                .all(|path| !certificate
                    .independence_firewall
                    .definition_inputs
                    .contains(path))
        );
        assert!(!certificate.m4_authorized);
    }

    #[test]
    fn semantic_depth_three_is_rejected_without_an_ast_cap() {
        let variable = context_variable(1);
        let once = former_node(
            Td21V2PublicFormer::SuspensionFormation,
            vec![variable],
            None,
        );
        let twice = former_node(Td21V2PublicFormer::SuspensionFormation, vec![once], None);
        let thrice = former_node(Td21V2PublicFormer::SuspensionFormation, vec![twice], None);
        let decision = derive_t_d2_1_contextual_membership_v2(
            &Td21V2ParameterTelescope {
                hypotheses: vec![Td21V2KernelClassifier::Type],
            },
            &thrice,
        )
        .expect("decision");
        assert!(!decision.typed_judgment_accepted);
        assert!(
            decision
                .exact_obstruction
                .as_deref()
                .is_some_and(|reason| reason.contains("semantic former depth"))
        );
    }

    #[test]
    fn binder_child_replays_under_its_actual_context_but_not_closed() {
        let body = bound_variable(1);
        let lambda = former_node(
            Td21V2PublicFormer::LambdaIntroduction,
            vec![body.clone()],
            None,
        );
        let context = Td21V2ParameterTelescope {
            hypotheses: Vec::new(),
        };
        let lambda_decision =
            derive_t_d2_1_contextual_membership_v2(&context, &lambda).expect("lambda decision");
        assert!(lambda_decision.typed_judgment_accepted);
        assert!(!lambda_decision.domain_member);
        let lambda_derivation = lambda_decision.derivation.expect("lambda derivation");
        assert_eq!(lambda_derivation.children.len(), 1);
        assert_eq!(
            lambda_derivation.children[0].active_binder_classifiers,
            vec![Td21V2KernelClassifier::Neutral]
        );
        assert_eq!(
            lambda_derivation.children[0].exact_canonical_presentation,
            Expr::Var(1)
        );
        assert!(
            replay_t_d2_1_contextual_membership_v2(
                &context,
                &lambda,
                &derive_t_d2_1_contextual_membership_v2(&context, &lambda).expect("second issue")
            )
            .expect("replay")
        );

        let closed =
            derive_t_d2_1_contextual_membership_v2(&context, &body).expect("closed decision");
        assert!(!closed.typed_judgment_accepted);
        assert!(
            closed
                .exact_obstruction
                .as_deref()
                .is_some_and(|reason| reason.contains("active binder prefix 0"))
        );
    }

    #[test]
    fn repeated_context_coordinate_is_not_split_into_fresh_parameters() {
        let node = former_node(
            Td21V2PublicFormer::Application,
            vec![context_variable(1), context_variable(1)],
            None,
        );
        let context = Td21V2ParameterTelescope {
            hypotheses: vec![Td21V2KernelClassifier::Type],
        };
        let decision = derive_t_d2_1_contextual_membership_v2(&context, &node).expect("decision");
        assert!(decision.typed_judgment_accepted);
        let derivation = decision.derivation.expect("derivation");
        assert_eq!(derivation.parameter_coordinates_used, vec![1]);
        assert_eq!(
            derivation.exact_canonical_presentation,
            Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Var(1)))
        );
    }

    #[test]
    fn sequential_dependent_context_prefix_is_replayed_exactly() {
        let context = Td21V2ParameterTelescope {
            hypotheses: vec![
                Td21V2KernelClassifier::Type,
                Td21V2KernelClassifier::Element {
                    type_expression: Expr::Var(1),
                },
            ],
        };
        let decision = derive_t_d2_1_contextual_membership_v2(&context, &context_variable(2))
            .expect("dependent context decision");
        assert!(decision.typed_judgment_accepted);
        assert!(!decision.domain_member);
        let derivation = decision.derivation.expect("dependent derivation");
        assert_eq!(derivation.exact_canonical_presentation, Expr::Var(2));
        assert_eq!(
            derivation.kernel_type_json,
            serde_json::to_string(&KernelTy::El(Expr::Var(1))).expect("kernel type")
        );
        assert_eq!(derivation.parameter_coordinates_used, vec![2]);
    }

    #[test]
    fn nested_resident_and_unbounded_dependent_context_fail_closed() {
        let definition = operational_definition().expect("definition");
        let resident = definition.residents.values().next().expect("resident");
        let nested = former_node(
            Td21V2PublicFormer::SuspensionFormation,
            vec![resident_node(resident)],
            None,
        );
        let decision = derive_t_d2_1_contextual_membership_v2(
            &Td21V2ParameterTelescope {
                hypotheses: Vec::new(),
            },
            &nested,
        )
        .expect("nested decision");
        assert!(!decision.typed_judgment_accepted);
        assert!(
            decision
                .exact_obstruction
                .as_deref()
                .is_some_and(|reason| reason.contains("no exact clause-level embedding"))
        );

        let context = Td21V2ParameterTelescope {
            hypotheses: vec![
                Td21V2KernelClassifier::Type,
                Td21V2KernelClassifier::Element {
                    type_expression: Expr::Susp(Box::new(Expr::Susp(Box::new(Expr::Susp(
                        Box::new(Expr::Var(1)),
                    ))))),
                },
            ],
        };
        let context_decision =
            derive_t_d2_1_contextual_membership_v2(&context, &context_variable(1))
                .expect("context decision");
        assert!(!context_decision.typed_judgment_accepted);
        assert!(
            context_decision
                .exact_obstruction
                .as_deref()
                .is_some_and(|reason| reason.contains("semantic former depth"))
        );
    }

    #[test]
    fn deterministic_and_resealed_nested_mutations_fail_closed() {
        let certificate = issue_t_d2_1_operational_domain_v2().expect("issuer");
        let mut mutation = certificate.clone();
        mutation.membership_decidable = !mutation.membership_decidable;
        let replay = replay_t_d2_1_operational_domain_v2(&mutation);
        assert!(!replay.valid);
        assert!(!replay.membership_decidable);
        assert!(!replay.quotient_finitely_enumerable);
        assert!(!replay.t_d2_2_prerequisite_satisfied);
        let mut mutation = certificate.clone();
        mutation.pre_corpus_carrier_seal.push('0');
        assert!(!replay_t_d2_1_operational_domain_v2(&mutation).valid);
        let mut mutation = certificate.clone();
        mutation.m4_authorized = true;
        mutation.result_digest = certificate_digest(&mutation);
        let replay = replay_t_d2_1_operational_domain_v2(&mutation);
        assert!(!replay.valid);
        assert!(!replay.m4_authorized);

        let mut mutation = certificate.clone();
        mutation.normalization_proof.total_on_bounded_carrier = true;
        mutation.typed_normalization_total = true;
        mutation.result_digest = certificate_digest(&mutation);
        assert!(!replay_t_d2_1_operational_domain_v2(&mutation).valid);

        let mut mutation = certificate.clone();
        mutation.quotient_proof.canonical_parameter_renaming_total = true;
        mutation
            .quotient_proof
            .uniform_specializations_not_multiplied = true;
        mutation.quotient_proof.structural_equality_decidable = true;
        mutation
            .quotient_proof
            .every_carrier_row_has_one_quotient_key = true;
        mutation
            .quotient_proof
            .quotient_key_collisions_replayed_by_equality = true;
        mutation.quotient_proof.quotient_decidable = true;
        mutation.quotient_decidable = true;
        mutation.result_digest = certificate_digest(&mutation);
        assert!(!replay_t_d2_1_operational_domain_v2(&mutation).valid);

        let mut mutation = certificate.clone();
        mutation.context_finiteness.finite_inductive_grammar_proved = true;
        mutation.symbolic_carrier_induction.induction_total = true;
        mutation.result_digest = certificate_digest(&mutation);
        assert!(!replay_t_d2_1_operational_domain_v2(&mutation).valid);

        let mut mutation = certificate;
        mutation.regression.every_family_recognized = true;
        mutation.regression.every_membership_row_recognized = true;
        mutation.family_regression_passed = true;
        mutation.membership_row_regression_passed = true;
        mutation.result_digest = certificate_digest(&mutation);
        let replay = replay_t_d2_1_operational_domain_v2(&mutation);
        assert!(!replay.valid);
        assert!(!replay.family_regression_passed);
        assert!(!replay.membership_row_regression_passed);
    }
}
