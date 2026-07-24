//! Proof-carrying E-4 naturality-generator basis.
//!
//! This module separates three statements which are easy to conflate:
//!
//! 1. the four kinds of elementary generator are operational;
//! 2. every morphism accepted by the current E-1 context judgement has a
//!    replayable, non-catch-all telescope-image presentation; and
//! 3. those presentations generate every naturality square for every
//!    intended `Schema2(W)` constructor.
//!
//! The first two statements are proved here. The third is not: the current
//! E-2 grammar is a historical ordinary-constructor fragment, not a total
//! class-indexed grammar. Consequently global C2/E-4 completeness remains
//! false and this module has no API that can issue `Independent`. Under M1
//! it can still issue final `Generated` evidence from an explicit sub-basis.

use crate::context::{
    BinderId, CofibrationExpr, Declaration, DimExpr, ExchangeToken, FormedSchemaContext,
    SchemaContextError, SubstitutionImage, SubstitutionPreservationToken, SubstitutionSupportToken,
    TermExpr, TypeExpr, TypedExpression, TypedSubstitutionToken, exchange_adjacent,
    form_schema_context, identity_substitution, issue_substitution_preservation,
    issue_substitution_support, issue_typed_substitution, replay_adjacent_exchange,
    replay_formed_context, replay_substitution_preservation, replay_substitution_support,
    replay_typed_substitution, weakening_substitution,
};
use crate::grammar::{
    DERIVED_ACTION_MEMBERSHIP_RULE, DerivationRef, RegisteredBoundaryBundleRef, SchemaBinderId,
    SchemaDimBinderId, SchemaDimExpr, SchemaLocalDeclaration, SchemaTermExpr, SchemaTypeExpr,
    Step8RegisteredSignatures, SupportWindow, check_schema_type, infer_schema_term,
    issue_step8_registered_signatures, replay_step8_registered_signatures,
};
use crate::stage1_r1::{
    R1_MEMBERSHIP_GAP, R1LocalRole, issue_stage1_r1_package_token, replay_stage1_r1_package_token,
};
use crate::step8_r2::{
    Step8R2PrefixLocalTypedSignaturesToken, Step8R2TypedSignaturesToken,
    issue_step8_r2_typed_signatures_token,
    replay_step8_r2_prefix_general_typed_signatures_token_v2,
    replay_step8_r2_prefix_local_typed_signatures_token, replay_step8_r2_typed_signatures_token,
};
use pen_core::telescope::Telescope;
use pen_type::elaborate::{SealedSignature, candidate_hash};
use serde::Serialize;
use thiserror::Error;

pub const E4_GENERATOR_BASIS_VERSION: &str = "schema2-e4-generator-basis-v1";
pub const E4_GLOBAL_COMPLETENESS_GAP: &str =
    "C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS";
pub const E4_CLASS_GRAMMAR_GAP: &str =
    "E4_CLASS_INDEXED_SCHEMA2_CONSTRUCTOR_EXHAUSTIVENESS_NOT_PROVED";
pub const E4_NORMALIZATION_NATURALITY_GAP: &str =
    "E4_GENERATORS_NOT_YET_BOUND_TO_E3_NORMALIZATION_NATURALITY_FOR_ALL_SCHEMA_TERMS";
pub const E4_CUBICAL_SCHEMA_ACTION_GAP: &str =
    "E4_GENERAL_DEPENDENT_CUBICAL_SCHEMA_ACTION_INDUCTION_NOT_PROVED";
pub const E4_M1_RULE: &str = "procedural-adjudication-p1-monotone-membership-m1";
pub const E4_PREFIX_LOCAL_M1_VERSION: &str = "schema2-e4-prefix-local-step8-m1-generated-v1";
pub const E4_PREFIX_GENERAL_M1_VERSION: &str = "schema2-e4-prefix-general-step8-m1-generated-v2";
pub const E4_M1_INDEPENDENT_GAP: &str = "M1_INDEPENDENT_VERDICT_REQUIRES_COMPLETE_E4_BASIS";
pub const CUBE_AT_MAP_CUBE_COMPUTATION_RULE: &str =
    "schema2-constructor-computation-cube-at-map-cube-v1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum E4GeneratorKind {
    TypedAdjacentExchange,
    Weakening,
    TypedInstanceImage,
    CubicalFaceMap,
}

impl E4GeneratorKind {
    pub const ALL: [Self; 4] = [
        Self::TypedAdjacentExchange,
        Self::Weakening,
        Self::TypedInstanceImage,
        Self::CubicalFaceMap,
    ];
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CubicalEndpoint {
    Zero,
    One,
}

impl CubicalEndpoint {
    const fn dimension(self) -> DimExpr {
        match self {
            Self::Zero => DimExpr::Zero,
            Self::One => DimExpr::One,
        }
    }

    const fn value(self) -> bool {
        matches!(self, Self::One)
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubstitutionImageKind {
    Type,
    Term,
    RigidLibrary,
    Dimension,
    Cofibration,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AdjacentExchangeGeneratorToken {
    source: FormedSchemaContext,
    left_index: usize,
    exchange: ExchangeToken,
    morphism: TypedSubstitutionToken,
    derivation_hash: String,
}

impl AdjacentExchangeGeneratorToken {
    pub fn morphism(&self) -> &TypedSubstitutionToken {
        &self.morphism
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WeakeningGeneratorToken {
    source: FormedSchemaContext,
    target: FormedSchemaContext,
    morphism: TypedSubstitutionToken,
    derivation_hash: String,
}

impl WeakeningGeneratorToken {
    pub fn morphism(&self) -> &TypedSubstitutionToken {
        &self.morphism
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

/// One checked telescope-image extension, not a wrapper around an arbitrary
/// complete substitution. The source prefix grows by exactly one declaration
/// at each step in a typed-instance presentation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TypedInstanceImageGeneratorToken {
    full_morphism_derivation_hash: String,
    source_prefix: FormedSchemaContext,
    target: FormedSchemaContext,
    prefix_length: usize,
    principal_source: BinderId,
    principal_image: SubstitutionImage,
    principal_image_kind: SubstitutionImageKind,
    syntactically_non_identity: bool,
    prior_prefix_derivation_hash: Option<String>,
    prefix_substitution: TypedSubstitutionToken,
    derivation_hash: String,
}

impl TypedInstanceImageGeneratorToken {
    pub const fn prefix_length(&self) -> usize {
        self.prefix_length
    }

    pub fn principal_image(&self) -> &SubstitutionImage {
        &self.principal_image
    }

    pub fn prefix_substitution(&self) -> &TypedSubstitutionToken {
        &self.prefix_substitution
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CubicalFaceMapGeneratorToken {
    source: FormedSchemaContext,
    target: FormedSchemaContext,
    interval: BinderId,
    endpoint: CubicalEndpoint,
    dependent_cofibrations_restricted: usize,
    morphism: TypedSubstitutionToken,
    derivation_hash: String,
}

impl CubicalFaceMapGeneratorToken {
    pub fn target(&self) -> &FormedSchemaContext {
        &self.target
    }

    pub fn morphism(&self) -> &TypedSubstitutionToken {
        &self.morphism
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LegalMorphismPresentation {
    Identity,
    AdjacentExchange {
        generator: Box<AdjacentExchangeGeneratorToken>,
    },
    Weakening {
        generator: Box<WeakeningGeneratorToken>,
    },
    CubicalFaceMap {
        generator: Box<CubicalFaceMapGeneratorToken>,
    },
    TypedInstanceImages {
        generators: Vec<TypedInstanceImageGeneratorToken>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LegalMorphismDecompositionToken {
    morphism: TypedSubstitutionToken,
    presentation: LegalMorphismPresentation,
    generator_word_length: usize,
    source_prefixes_strictly_increase: bool,
    reconstruction_replays_original: bool,
    no_arbitrary_substitution_catch_all: bool,
    complete_for_current_e1_context_judgement: bool,
    derivation_hash: String,
}

impl LegalMorphismDecompositionToken {
    pub fn morphism(&self) -> &TypedSubstitutionToken {
        &self.morphism
    }

    pub fn presentation(&self) -> &LegalMorphismPresentation {
        &self.presentation
    }

    pub const fn complete_for_current_e1_context_judgement(&self) -> bool {
        self.complete_for_current_e1_context_judgement
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

/// An E-1 square proving typing and support naturality for one expression.
/// It explicitly does not upgrade this to E-3 normalization naturality.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct E1TypingSupportSquareToken {
    decomposition: LegalMorphismDecompositionToken,
    source_expression: TypedExpression,
    typing_preservation: SubstitutionPreservationToken,
    support_preservation: SubstitutionSupportToken,
    typing_naturality_proved: bool,
    support_naturality_proved: bool,
    normalization_naturality_proved: bool,
    normalization_naturality_gap: String,
    derivation_hash: String,
}

impl E1TypingSupportSquareToken {
    pub const fn typing_naturality_proved(&self) -> bool {
        self.typing_naturality_proved
    }

    pub const fn support_naturality_proved(&self) -> bool {
        self.support_naturality_proved
    }

    pub const fn normalization_naturality_proved(&self) -> bool {
        self.normalization_naturality_proved
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Schema2NaturalityClass {
    Foundation,
    Former,
    Map,
    Axiomatic,
    Modal,
    HitV2,
    Synthesis,
    Unknown,
}

impl Schema2NaturalityClass {
    pub const ALL: [Self; 8] = [
        Self::Foundation,
        Self::Former,
        Self::Map,
        Self::Axiomatic,
        Self::Modal,
        Self::HitV2,
        Self::Synthesis,
        Self::Unknown,
    ];

    const fn gap(self) -> &'static str {
        match self {
            Self::Foundation => "E4_FOUNDATION_GENERATOR_EXHAUSTIVENESS_NOT_PROVED",
            Self::Former => "E4_FORMER_GENERATOR_EXHAUSTIVENESS_NOT_PROVED",
            Self::Map => "E4_MAP_GENERATOR_EXHAUSTIVENESS_NOT_PROVED",
            Self::Axiomatic => "E4_AXIOMATIC_GENERATOR_EXHAUSTIVENESS_NOT_PROVED",
            Self::Modal => "E4_MODAL_GENERATOR_EXHAUSTIVENESS_NOT_PROVED",
            Self::HitV2 => "E4_HIT_V2_GENERATOR_EXHAUSTIVENESS_NOT_PROVED",
            Self::Synthesis => "E4_SYNTHESIS_GENERATOR_EXHAUSTIVENESS_NOT_PROVED",
            Self::Unknown => "E4_UNKNOWN_CLASS_NOT_ELIMINATED_BY_TYPED_CLASSIFICATION",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct PerClassGeneratorCompleteness {
    class: Schema2NaturalityClass,
    e1_context_morphism_presentation_available: bool,
    class_constructor_inventory_exhaustive: bool,
    normalization_naturality_for_class_proved: bool,
    cubical_action_induction_for_class_proved: bool,
    per_class_generator_induction_complete: bool,
    named_gaps: Vec<String>,
}

impl PerClassGeneratorCompleteness {
    pub const fn class(&self) -> Schema2NaturalityClass {
        self.class
    }

    pub const fn complete(&self) -> bool {
        self.per_class_generator_induction_complete
    }

    pub fn named_gaps(&self) -> &[String] {
        &self.named_gaps
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct E4GeneratorBasisAuditToken {
    token_version: String,
    generator_kinds: Vec<E4GeneratorKind>,
    exemplar_derivation_hashes: Vec<String>,
    current_e1_context_morphism_decomposition_complete: bool,
    arbitrary_substitution_used_as_single_generator: bool,
    e1_typing_support_naturality_available: bool,
    per_class: Vec<PerClassGeneratorCompleteness>,
    every_class_complete: bool,
    full_schema2_generator_completeness: bool,
    global_gap: String,
    independent_membership_issuance_enabled: bool,
    derivation_hash: String,
}

impl E4GeneratorBasisAuditToken {
    pub const fn current_e1_context_morphism_decomposition_complete(&self) -> bool {
        self.current_e1_context_morphism_decomposition_complete
    }

    pub fn per_class(&self) -> &[PerClassGeneratorCompleteness] {
        &self.per_class
    }

    pub const fn full_schema2_generator_completeness(&self) -> bool {
        self.full_schema2_generator_completeness
    }

    pub const fn independent_membership_issuance_enabled(&self) -> bool {
        self.independent_membership_issuance_enabled
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Step8MapCubeGenerationProof {
    step8: Step8R2TypedSignaturesToken,
    registered_signatures: Box<Step8RegisteredSignatures>,
    parent_cube_action: Box<ParentCubeActionGeneratedToken>,
    sub_basis_inclusion: E4SubBasisInclusionProof,
    derivation_hash: String,
}

/// Source-first MapCube proof.  Its boundary dependency is the typed
/// boundary derivation sealed by the supplied-prefix R2 token, rather than
/// the archival C6 bundle used by the historical compatibility path.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Step8PrefixLocalMapCubeGenerationProof {
    step8: Step8R2PrefixLocalTypedSignaturesToken,
    registered_signatures: Box<Step8RegisteredSignatures>,
    parent_cube_action: Box<ParentCubeActionGeneratedToken>,
    sub_basis_inclusion: Step8PrefixLocalSubBasisInclusionProof,
    derivation_hash: String,
}

impl Step8PrefixLocalMapCubeGenerationProof {
    pub fn step8(&self) -> &Step8R2PrefixLocalTypedSignaturesToken {
        &self.step8
    }

    pub fn parent_cube_action(&self) -> &ParentCubeActionGeneratedToken {
        &self.parent_cube_action
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

impl Step8MapCubeGenerationProof {
    pub fn parent_cube_action(&self) -> &ParentCubeActionGeneratedToken {
        &self.parent_cube_action
    }
}

/// The sole registered constructor computation used by E-4 in this batch.
/// Its contractum is computed from the redex; callers never supply it.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParentCubeComputationRule {
    CubeAtMapCube,
}

impl ParentCubeComputationRule {
    pub const ALL: [Self; 1] = [Self::CubeAtMapCube];

    pub const fn id(self) -> &'static str {
        match self {
            Self::CubeAtMapCube => CUBE_AT_MAP_CUBE_COMPUTATION_RULE,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ParentCubeFaceEquationToken {
    axis: u32,
    endpoint: CubicalEndpoint,
    coordinates: Vec<SchemaDimExpr>,
    left: SchemaTermExpr,
    right: SchemaTermExpr,
    common_type: SchemaTypeExpr,
    computation_rule: ParentCubeComputationRule,
    computation_rule_id: String,
    face_map: CubicalFaceMapGeneratorToken,
    dimension_interval_correspondence: Vec<(SchemaDimBinderId, BinderId)>,
    derivation_hash: String,
}

impl ParentCubeFaceEquationToken {
    pub const fn axis(&self) -> u32 {
        self.axis
    }

    pub const fn endpoint(&self) -> CubicalEndpoint {
        self.endpoint
    }

    pub fn left(&self) -> &SchemaTermExpr {
        &self.left
    }

    pub fn right(&self) -> &SchemaTermExpr {
        &self.right
    }
}

/// Generic operational judgment `ParentCubeActionGenerated(f,c,row)`.
///
/// Issuance first constructs `MapCube(f,c)`, then requires the proposed row
/// to be that exact AST and to infer the same type. Every cubical face carries
/// a typed `cube_at_map_cube` computation derivation and the corresponding
/// E-1 face-map generator. Frozen equality is neither invoked nor needed.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ParentCubeActionGeneratedToken {
    context: FormedSchemaContext,
    window: SupportWindow,
    locals: Vec<SchemaLocalDeclaration>,
    coordinate_dimensions: Vec<SchemaDimBinderId>,
    parent_function: SchemaTermExpr,
    registered_cube: SchemaTermExpr,
    candidate_row: SchemaTermExpr,
    canonical_generated_row: SchemaTermExpr,
    parent_function_type: SchemaTypeExpr,
    registered_cube_type: SchemaTypeExpr,
    generated_row_type: SchemaTypeExpr,
    face_equations: Vec<ParentCubeFaceEquationToken>,
    computation_rule_id: String,
    frozen_equality_used: bool,
    derivation_hash: String,
}

impl ParentCubeActionGeneratedToken {
    pub fn canonical_generated_row(&self) -> &SchemaTermExpr {
        &self.canonical_generated_row
    }

    pub fn generated_row_type(&self) -> &SchemaTypeExpr {
        &self.generated_row_type
    }

    pub fn face_equations(&self) -> &[ParentCubeFaceEquationToken] {
        &self.face_equations
    }

    pub const fn frozen_equality_used(&self) -> bool {
        self.frozen_equality_used
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct E4SubBasisInclusionProof {
    sub_basis_kinds: Vec<E4GeneratorKind>,
    full_basis_kinds: Vec<E4GeneratorKind>,
    full_basis_audit_derivation_hash: String,
    every_sub_basis_kind_registered: bool,
    derivation_hash: String,
}

/// Definition-level inclusion of the one MapCube generator kind into E-4.
/// This source-first proof does not execute the unrelated weakening exemplar;
/// the full basis inventory is read directly from the closed enum.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Step8PrefixLocalSubBasisInclusionProof {
    sub_basis_kinds: Vec<E4GeneratorKind>,
    full_basis_kinds: Vec<E4GeneratorKind>,
    full_basis_inventory_hash: String,
    every_sub_basis_kind_registered: bool,
    weakening_exemplar_replayed: bool,
    derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum M1GeneratedSubject {
    Step8CellAction {
        step8_derivation_hash: String,
        schema_signature_derivation_hash: String,
    },
}

/// The only membership verdict E-4 can currently issue. There is
/// intentionally no `Independent` sibling type or public constructor.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct M1GeneratedMembershipToken {
    rule: String,
    membership_rule: String,
    subject: M1GeneratedSubject,
    sub_basis_digest: String,
    generating_derivation: Step8MapCubeGenerationProof,
    final_by_basis_monotonicity: bool,
    full_e4_completeness_used: bool,
    derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Step8PrefixLocalM1TypedDependencies {
    pub predecessor_signature_digest: String,
    pub source_telescope_hash: String,
    pub typed_boundary_dependency_hash: String,
    pub typed_boundary_derivation_hash: String,
    pub reference_only_derivation_hash: String,
    pub base_binding_derivation_hash: String,
    pub element_overlay_derivation_hash: String,
    pub schema_signature_derivation_hash: String,
    pub parent_cube_action_derivation_hash: String,
    pub sub_basis_inclusion_derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Step8PrefixLocalM1Capabilities {
    pub supplied_exact_prefix_read: bool,
    pub supplied_exact_candidate_read: bool,
    pub source_bound_typed_boundary_read: bool,
    pub term_level_map_cube_computation_read: bool,
    pub cubical_face_map_rule_read: bool,
    pub definition_level_sub_basis_inclusion_read: bool,
    pub weakening_exemplar_read: bool,
    pub archival_constant_bridge_read: bool,
    pub archive_input_read: bool,
    pub historical_count_input_read: bool,
    pub acceptance_bar_input_read: bool,
    pub membership_verdict_input_read: bool,
    pub enacted_future_input_read: bool,
}

impl Step8PrefixLocalM1Capabilities {
    pub const fn forbidden_inputs_withheld(&self) -> bool {
        !self.weakening_exemplar_read
            && !self.archival_constant_bridge_read
            && !self.archive_input_read
            && !self.historical_count_input_read
            && !self.acceptance_bar_input_read
            && !self.membership_verdict_input_read
            && !self.enacted_future_input_read
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct Step8PrefixLocalM1GeneratedMembershipToken {
    pub token_version: String,
    pub rule: String,
    pub membership_rule: String,
    pub subject: M1GeneratedSubject,
    pub typed_dependencies: Step8PrefixLocalM1TypedDependencies,
    pub sub_basis_digest: String,
    pub generating_derivation: Step8PrefixLocalMapCubeGenerationProof,
    pub final_by_basis_monotonicity: bool,
    pub full_e4_completeness_used: bool,
    pub capabilities: Step8PrefixLocalM1Capabilities,
    pub derivation_hash: String,
}

impl Step8PrefixLocalM1GeneratedMembershipToken {
    pub fn subject(&self) -> &M1GeneratedSubject {
        &self.subject
    }

    pub fn sub_basis_digest(&self) -> &str {
        &self.sub_basis_digest
    }

    pub fn generating_derivation(&self) -> &Step8PrefixLocalMapCubeGenerationProof {
        &self.generating_derivation
    }

    pub const fn final_by_basis_monotonicity(&self) -> bool {
        self.final_by_basis_monotonicity
    }

    pub const fn full_e4_completeness_used(&self) -> bool {
        self.full_e4_completeness_used
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum M1PendingSubject {
    Stage1CarrierException { role: R1LocalRole },
    Step8LeftUnitCoherence,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct M1PendingMembershipRecord {
    subject: M1PendingSubject,
    source_derivation_hash: String,
    obstruction: String,
    independent_verdict_withheld: bool,
}

impl M1PendingMembershipRecord {
    pub const fn subject(&self) -> M1PendingSubject {
        self.subject
    }

    pub fn obstruction(&self) -> &str {
        &self.obstruction
    }
}

/// Explicit F-P1 guard. These are output *flags*, not output values: every
/// field must remain false until a future complete E-4 theorem enables E-2b.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct E4ForbiddenOutputAudit {
    independent_verdict_issued: bool,
    ordinary_family_token_issued: bool,
    historical_stage_score_issued: bool,
    fq2_evaluated: bool,
    agent_a_handoff_issued: bool,
    e2b_quotient_closure_executed: bool,
}

impl E4ForbiddenOutputAudit {
    pub const fn all_withheld(&self) -> bool {
        !self.independent_verdict_issued
            && !self.ordinary_family_token_issued
            && !self.historical_stage_score_issued
            && !self.fq2_evaluated
            && !self.agent_a_handoff_issued
            && !self.e2b_quotient_closure_executed
    }
}

/// Stable top-level replay surface for the partial E-4 development state.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct E4DevelopmentAuditToken {
    token_version: String,
    basis: E4GeneratorBasisAuditToken,
    m1_generated: Vec<M1GeneratedMembershipToken>,
    m1_pending: Vec<M1PendingMembershipRecord>,
    full_e4_completeness: bool,
    global_gap: String,
    forbidden_outputs: E4ForbiddenOutputAudit,
    derivation_hash: String,
}

impl E4DevelopmentAuditToken {
    pub fn basis(&self) -> &E4GeneratorBasisAuditToken {
        &self.basis
    }

    pub fn m1_generated(&self) -> &[M1GeneratedMembershipToken] {
        &self.m1_generated
    }

    pub fn m1_pending(&self) -> &[M1PendingMembershipRecord] {
        &self.m1_pending
    }

    pub const fn full_e4_completeness(&self) -> bool {
        self.full_e4_completeness
    }

    pub fn global_gap(&self) -> &str {
        &self.global_gap
    }

    pub fn forbidden_outputs(&self) -> &E4ForbiddenOutputAudit {
        &self.forbidden_outputs
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

impl M1GeneratedMembershipToken {
    pub fn subject(&self) -> &M1GeneratedSubject {
        &self.subject
    }

    pub fn sub_basis_digest(&self) -> &str {
        &self.sub_basis_digest
    }

    pub fn generating_derivation(&self) -> &Step8MapCubeGenerationProof {
        &self.generating_derivation
    }

    pub const fn final_by_basis_monotonicity(&self) -> bool {
        self.final_by_basis_monotonicity
    }

    pub const fn full_e4_completeness_used(&self) -> bool {
        self.full_e4_completeness_used
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum E4GeneratorError {
    #[error("E-1 context judgement failed: {0}")]
    Context(String),
    #[error("adjacent exchange generator did not replay")]
    AdjacentExchangeReplay,
    #[error("weakening generator requires a strict prefix extension")]
    NotWeakening,
    #[error("binder {binder:?} is not an interval variable")]
    NotInterval { binder: BinderId },
    #[error("typed-instance image index {index} is outside the source telescope")]
    ImageIndexOutOfRange { index: usize },
    #[error("typed-instance prefix chain failed to reconstruct its source morphism")]
    PrefixReconstructionFailed,
    #[error("schema term judgment failed: {0}")]
    Schema(String),
    #[error("schema local or coordinate inventory is malformed")]
    MalformedSchemaInventory,
    #[error("parent cube action requires a typed registered cube")]
    ExpectedRegisteredCube,
    #[error("candidate row is not the canonical MapCube of the supplied parent and cube")]
    CandidateNotCanonicalMapCube,
    #[error("candidate and generated MapCube types differ")]
    ParentCubeTypeMismatch,
    #[error("registered cube_at_map_cube computation did not produce the required right side")]
    CubeComputationMismatch,
    #[error("the current parent-cube rule requires dimension-parametric parent and cube terms")]
    DimensionDependentParentActionUnsupported,
    #[error("no fresh binder remains for the face-map context")]
    FreshBinderExhausted,
    #[error("generator token replay mismatch")]
    ReplayMismatch,
    #[error("Step-8 MapCube evidence failed: {0}")]
    Step8(String),
    #[error("upstream replay dependency failed: {0}")]
    Upstream(String),
    #[error("Step-8 token is not the exact pending R2 cell-action case")]
    Step8InvariantDrift,
}

impl From<SchemaContextError> for E4GeneratorError {
    fn from(error: SchemaContextError) -> Self {
        Self::Context(error.to_string())
    }
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(E4_GENERATOR_BASIS_VERSION, domain, payload))
        .expect("E-4 proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn prefix_local_tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(E4_PREFIX_LOCAL_M1_VERSION, domain, payload))
        .expect("prefix-local E-4 proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn prefix_general_tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(E4_PREFIX_GENERAL_M1_VERSION, domain, payload))
        .expect("prefix-general E-4 proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn prefix_m1_tagged_hash(prefix_general: bool, domain: &str, payload: &impl Serialize) -> String {
    if prefix_general {
        prefix_general_tagged_hash(domain, payload)
    } else {
        prefix_local_tagged_hash(domain, payload)
    }
}

fn identity_image(declaration: &Declaration) -> SubstitutionImage {
    match declaration {
        Declaration::TypeParameter { binder, .. } => SubstitutionImage::Type {
            source: *binder,
            image: TypeExpr::Parameter { binder: *binder },
        },
        Declaration::OpaqueElement { binder, .. } => SubstitutionImage::Term {
            source: *binder,
            image: TermExpr::Variable { binder: *binder },
        },
        Declaration::LibraryReference {
            binder,
            step,
            symbol,
            ..
        } => SubstitutionImage::RigidLibrary {
            source: *binder,
            step: *step,
            symbol: symbol.clone(),
        },
        Declaration::IntervalVariable { binder, .. } => SubstitutionImage::Dimension {
            source: *binder,
            image: DimExpr::Variable(*binder),
        },
        Declaration::CofibrationAssumption {
            binder, formula, ..
        } => SubstitutionImage::Cofibration {
            source: *binder,
            image: formula.clone(),
        },
    }
}

fn image_kind(image: &SubstitutionImage) -> SubstitutionImageKind {
    match image {
        SubstitutionImage::Type { .. } => SubstitutionImageKind::Type,
        SubstitutionImage::Term { .. } => SubstitutionImageKind::Term,
        SubstitutionImage::RigidLibrary { .. } => SubstitutionImageKind::RigidLibrary,
        SubstitutionImage::Dimension { .. } => SubstitutionImageKind::Dimension,
        SubstitutionImage::Cofibration { .. } => SubstitutionImageKind::Cofibration,
    }
}

fn substitute_face_formula(
    formula: &CofibrationExpr,
    interval: BinderId,
    endpoint: CubicalEndpoint,
) -> CofibrationExpr {
    match formula {
        CofibrationExpr::False | CofibrationExpr::True => formula.clone(),
        CofibrationExpr::Endpoint {
            interval: found,
            value,
        } if *found == interval => {
            if *value == endpoint.value() {
                CofibrationExpr::True
            } else {
                CofibrationExpr::False
            }
        }
        CofibrationExpr::Endpoint { .. } => formula.clone(),
        CofibrationExpr::And { terms } => CofibrationExpr::And {
            terms: terms
                .iter()
                .map(|term| substitute_face_formula(term, interval, endpoint))
                .collect(),
        },
        CofibrationExpr::Or { terms } => CofibrationExpr::Or {
            terms: terms
                .iter()
                .map(|term| substitute_face_formula(term, interval, endpoint))
                .collect(),
        },
    }
}

pub fn issue_adjacent_exchange_generator(
    source: &FormedSchemaContext,
    left_index: usize,
) -> Result<AdjacentExchangeGeneratorToken, E4GeneratorError> {
    let exchange = exchange_adjacent(source, left_index)?;
    replay_adjacent_exchange(source, &exchange)
        .map_err(|_| E4GeneratorError::AdjacentExchangeReplay)?;
    let images = source.declarations().iter().map(identity_image).collect();
    let morphism = issue_typed_substitution(source, exchange.after(), images)?;
    let derivation_hash = tagged_hash(
        "typed-adjacent-exchange-generator",
        &(source, left_index, &exchange, &morphism),
    );
    Ok(AdjacentExchangeGeneratorToken {
        source: source.clone(),
        left_index,
        exchange,
        morphism,
        derivation_hash,
    })
}

pub fn replay_adjacent_exchange_generator(
    token: &AdjacentExchangeGeneratorToken,
) -> Result<(), E4GeneratorError> {
    let replay = issue_adjacent_exchange_generator(&token.source, token.left_index)?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

pub fn issue_weakening_generator(
    source: &FormedSchemaContext,
    target: &FormedSchemaContext,
) -> Result<WeakeningGeneratorToken, E4GeneratorError> {
    let morphism = weakening_substitution(source, target).map_err(|error| match error {
        SchemaContextError::InvalidWeakening => E4GeneratorError::NotWeakening,
        other => E4GeneratorError::Context(other.to_string()),
    })?;
    replay_typed_substitution(&morphism)?;
    let derivation_hash = tagged_hash("weakening-generator", &(source, target, &morphism));
    Ok(WeakeningGeneratorToken {
        source: source.clone(),
        target: target.clone(),
        morphism,
        derivation_hash,
    })
}

pub fn replay_weakening_generator(token: &WeakeningGeneratorToken) -> Result<(), E4GeneratorError> {
    let replay = issue_weakening_generator(&token.source, &token.target)?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

pub fn issue_cubical_face_map_generator(
    source: &FormedSchemaContext,
    interval: BinderId,
    endpoint: CubicalEndpoint,
) -> Result<CubicalFaceMapGeneratorToken, E4GeneratorError> {
    if !matches!(
        source.declaration(interval),
        Some(Declaration::IntervalVariable { .. })
    ) {
        return Err(E4GeneratorError::NotInterval { binder: interval });
    }
    let mut dependent_cofibrations_restricted = 0usize;
    let target_declarations = source
        .declarations()
        .iter()
        .filter_map(|declaration| match declaration {
            Declaration::IntervalVariable { binder, .. } if *binder == interval => None,
            Declaration::CofibrationAssumption {
                binder,
                name,
                formula,
            } => {
                let restricted = substitute_face_formula(formula, interval, endpoint);
                if restricted != *formula {
                    dependent_cofibrations_restricted += 1;
                }
                Some(Declaration::CofibrationAssumption {
                    binder: *binder,
                    name: name.clone(),
                    formula: restricted,
                })
            }
            other => Some(other.clone()),
        })
        .collect();
    let target = form_schema_context(target_declarations)?;
    let images = source
        .declarations()
        .iter()
        .map(|declaration| match declaration {
            Declaration::IntervalVariable { binder, .. } if *binder == interval => {
                SubstitutionImage::Dimension {
                    source: *binder,
                    image: endpoint.dimension(),
                }
            }
            Declaration::CofibrationAssumption {
                binder, formula, ..
            } => SubstitutionImage::Cofibration {
                source: *binder,
                image: substitute_face_formula(formula, interval, endpoint),
            },
            other => identity_image(other),
        })
        .collect();
    let morphism = issue_typed_substitution(source, &target, images)?;
    replay_typed_substitution(&morphism)?;
    let derivation_hash = tagged_hash(
        "cubical-face-map-generator",
        &(
            source,
            &target,
            interval,
            endpoint,
            dependent_cofibrations_restricted,
            &morphism,
        ),
    );
    Ok(CubicalFaceMapGeneratorToken {
        source: source.clone(),
        target,
        interval,
        endpoint,
        dependent_cofibrations_restricted,
        morphism,
        derivation_hash,
    })
}

pub fn replay_cubical_face_map_generator(
    token: &CubicalFaceMapGeneratorToken,
) -> Result<(), E4GeneratorError> {
    let replay = issue_cubical_face_map_generator(&token.source, token.interval, token.endpoint)?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

pub fn issue_typed_instance_image_generator(
    morphism: &TypedSubstitutionToken,
    index: usize,
) -> Result<TypedInstanceImageGeneratorToken, E4GeneratorError> {
    replay_typed_substitution(morphism)?;
    let Some(declaration) = morphism.source().declarations().get(index) else {
        return Err(E4GeneratorError::ImageIndexOutOfRange { index });
    };
    let principal_image = morphism.images()[index].clone();
    if principal_image.source() != declaration.binder() {
        return Err(E4GeneratorError::PrefixReconstructionFailed);
    }
    let source_prefix = form_schema_context(morphism.source().declarations()[..=index].to_vec())?;
    let prefix_substitution = issue_typed_substitution(
        &source_prefix,
        morphism.target(),
        morphism.images()[..=index].to_vec(),
    )?;
    let prior_prefix_derivation_hash = if index == 0 {
        None
    } else {
        let prior_source = form_schema_context(morphism.source().declarations()[..index].to_vec())?;
        let prior = issue_typed_substitution(
            &prior_source,
            morphism.target(),
            morphism.images()[..index].to_vec(),
        )?;
        Some(prior.derivation_hash().to_owned())
    };
    let prefix_length = index + 1;
    let principal_source = declaration.binder();
    let principal_image_kind = image_kind(&principal_image);
    let syntactically_non_identity = principal_image != identity_image(declaration);
    let derivation_hash = tagged_hash(
        "typed-instance-image-generator",
        &(
            morphism.derivation_hash(),
            &source_prefix,
            morphism.target(),
            prefix_length,
            principal_source,
            &principal_image,
            principal_image_kind,
            syntactically_non_identity,
            &prior_prefix_derivation_hash,
            &prefix_substitution,
        ),
    );
    Ok(TypedInstanceImageGeneratorToken {
        full_morphism_derivation_hash: morphism.derivation_hash().to_owned(),
        source_prefix,
        target: morphism.target().clone(),
        prefix_length,
        principal_source,
        principal_image,
        principal_image_kind,
        syntactically_non_identity,
        prior_prefix_derivation_hash,
        prefix_substitution,
        derivation_hash,
    })
}

pub fn replay_typed_instance_image_generator(
    morphism: &TypedSubstitutionToken,
    token: &TypedInstanceImageGeneratorToken,
) -> Result<(), E4GeneratorError> {
    let Some(index) = token.prefix_length.checked_sub(1) else {
        return Err(E4GeneratorError::ReplayMismatch);
    };
    let replay = issue_typed_instance_image_generator(morphism, index)?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

fn matches_identity(morphism: &TypedSubstitutionToken) -> Result<bool, E4GeneratorError> {
    if morphism.source() != morphism.target() {
        return Ok(false);
    }
    Ok(identity_substitution(morphism.source())? == *morphism)
}

fn recognize_weakening(
    morphism: &TypedSubstitutionToken,
) -> Result<Option<WeakeningGeneratorToken>, E4GeneratorError> {
    let source = morphism.source();
    let target = morphism.target();
    if target.declarations().len() <= source.declarations().len()
        || target.declarations()[..source.declarations().len()] != *source.declarations()
    {
        return Ok(None);
    }
    let generator = issue_weakening_generator(source, target)?;
    Ok((generator.morphism == *morphism).then_some(generator))
}

fn recognize_face_map(
    morphism: &TypedSubstitutionToken,
) -> Result<Option<CubicalFaceMapGeneratorToken>, E4GeneratorError> {
    for interval in morphism.source().interval_binders() {
        for endpoint in [CubicalEndpoint::Zero, CubicalEndpoint::One] {
            let generator =
                issue_cubical_face_map_generator(morphism.source(), interval, endpoint)?;
            if generator.target == *morphism.target() && generator.morphism == *morphism {
                return Ok(Some(generator));
            }
        }
    }
    Ok(None)
}

fn recognize_adjacent_exchange(
    morphism: &TypedSubstitutionToken,
) -> Result<Option<AdjacentExchangeGeneratorToken>, E4GeneratorError> {
    for left_index in 0..morphism.source().declarations().len().saturating_sub(1) {
        let Ok(generator) = issue_adjacent_exchange_generator(morphism.source(), left_index) else {
            continue;
        };
        if generator.morphism == *morphism {
            return Ok(Some(generator));
        }
    }
    Ok(None)
}

pub fn decompose_legal_morphism(
    morphism: &TypedSubstitutionToken,
) -> Result<LegalMorphismDecompositionToken, E4GeneratorError> {
    replay_typed_substitution(morphism)?;
    let presentation = if matches_identity(morphism)? {
        LegalMorphismPresentation::Identity
    } else if let Some(generator) = recognize_adjacent_exchange(morphism)? {
        LegalMorphismPresentation::AdjacentExchange {
            generator: Box::new(generator),
        }
    } else if let Some(generator) = recognize_weakening(morphism)? {
        LegalMorphismPresentation::Weakening {
            generator: Box::new(generator),
        }
    } else if let Some(generator) = recognize_face_map(morphism)? {
        LegalMorphismPresentation::CubicalFaceMap {
            generator: Box::new(generator),
        }
    } else {
        let generators = (0..morphism.source().declarations().len())
            .map(|index| issue_typed_instance_image_generator(morphism, index))
            .collect::<Result<Vec<_>, _>>()?;
        let Some(last) = generators.last() else {
            return Err(E4GeneratorError::PrefixReconstructionFailed);
        };
        if last.prefix_substitution != *morphism {
            return Err(E4GeneratorError::PrefixReconstructionFailed);
        }
        LegalMorphismPresentation::TypedInstanceImages { generators }
    };
    let (generator_word_length, source_prefixes_strictly_increase) = match &presentation {
        LegalMorphismPresentation::Identity => (0, true),
        LegalMorphismPresentation::AdjacentExchange { .. }
        | LegalMorphismPresentation::Weakening { .. }
        | LegalMorphismPresentation::CubicalFaceMap { .. } => (1, true),
        LegalMorphismPresentation::TypedInstanceImages { generators } => (
            generators.len(),
            generators
                .windows(2)
                .all(|pair| pair[0].prefix_length + 1 == pair[1].prefix_length),
        ),
    };
    let reconstruction_replays_original = true;
    let no_arbitrary_substitution_catch_all = !matches!(
        &presentation,
        LegalMorphismPresentation::TypedInstanceImages { generators }
            if generators.len() == 1 && morphism.source().declarations().len() > 1
    );
    let complete_for_current_e1_context_judgement = reconstruction_replays_original
        && source_prefixes_strictly_increase
        && no_arbitrary_substitution_catch_all;
    let derivation_hash = tagged_hash(
        "legal-context-morphism-decomposition",
        &(
            morphism,
            &presentation,
            generator_word_length,
            source_prefixes_strictly_increase,
            reconstruction_replays_original,
            no_arbitrary_substitution_catch_all,
            complete_for_current_e1_context_judgement,
        ),
    );
    Ok(LegalMorphismDecompositionToken {
        morphism: morphism.clone(),
        presentation,
        generator_word_length,
        source_prefixes_strictly_increase,
        reconstruction_replays_original,
        no_arbitrary_substitution_catch_all,
        complete_for_current_e1_context_judgement,
        derivation_hash,
    })
}

pub fn replay_legal_morphism_decomposition(
    token: &LegalMorphismDecompositionToken,
) -> Result<(), E4GeneratorError> {
    let replay = decompose_legal_morphism(&token.morphism)?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

pub fn issue_e1_typing_support_square(
    decomposition: LegalMorphismDecompositionToken,
    source_expression: TypedExpression,
) -> Result<E1TypingSupportSquareToken, E4GeneratorError> {
    replay_legal_morphism_decomposition(&decomposition)?;
    let typing_preservation =
        issue_substitution_preservation(&decomposition.morphism, source_expression.clone())?;
    let support_preservation =
        issue_substitution_support(&decomposition.morphism, source_expression.clone())?;
    replay_substitution_preservation(&decomposition.morphism, &typing_preservation)?;
    replay_substitution_support(&decomposition.morphism, &support_preservation)?;
    let typing_naturality_proved = typing_preservation.preserved();
    let support_naturality_proved = support_preservation.target_support_within_image_bound();
    let normalization_naturality_proved = false;
    let normalization_naturality_gap = E4_NORMALIZATION_NATURALITY_GAP.to_owned();
    let derivation_hash = tagged_hash(
        "e1-generator-typing-support-square",
        &(
            &decomposition,
            &source_expression,
            &typing_preservation,
            &support_preservation,
            typing_naturality_proved,
            support_naturality_proved,
            normalization_naturality_proved,
            &normalization_naturality_gap,
        ),
    );
    Ok(E1TypingSupportSquareToken {
        decomposition,
        source_expression,
        typing_preservation,
        support_preservation,
        typing_naturality_proved,
        support_naturality_proved,
        normalization_naturality_proved,
        normalization_naturality_gap,
        derivation_hash,
    })
}

pub fn replay_e1_typing_support_square(
    token: &E1TypingSupportSquareToken,
) -> Result<(), E4GeneratorError> {
    let replay = issue_e1_typing_support_square(
        token.decomposition.clone(),
        token.source_expression.clone(),
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

fn exemplar_contexts() -> Result<
    (
        AdjacentExchangeGeneratorToken,
        WeakeningGeneratorToken,
        Vec<TypedInstanceImageGeneratorToken>,
        CubicalFaceMapGeneratorToken,
    ),
    E4GeneratorError,
> {
    let exchange_context = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(1),
            name: "A".to_owned(),
            universe: 0,
        },
        Declaration::TypeParameter {
            binder: BinderId(2),
            name: "B".to_owned(),
            universe: 0,
        },
    ])?;
    let exchange = issue_adjacent_exchange_generator(&exchange_context, 0)?;

    let weak_source = form_schema_context(vec![Declaration::TypeParameter {
        binder: BinderId(10),
        name: "A".to_owned(),
        universe: 0,
    }])?;
    let weak_target = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(10),
            name: "A".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(11),
            name: "x".to_owned(),
            ty: TypeExpr::parameter(10),
        },
    ])?;
    let weakening = issue_weakening_generator(&weak_source, &weak_target)?;

    let instance_source = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(20),
            name: "A".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(21),
            name: "x".to_owned(),
            ty: TypeExpr::parameter(20),
        },
    ])?;
    let instance_target = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(30),
            name: "B".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(31),
            name: "y".to_owned(),
            ty: TypeExpr::parameter(30),
        },
    ])?;
    let instance_morphism = issue_typed_substitution(
        &instance_source,
        &instance_target,
        vec![
            SubstitutionImage::Type {
                source: BinderId(20),
                image: TypeExpr::trunc(TypeExpr::parameter(30)),
            },
            SubstitutionImage::Term {
                source: BinderId(21),
                image: TermExpr::TruncPoint {
                    carrier: Box::new(TypeExpr::parameter(30)),
                    point: Box::new(TermExpr::variable(31)),
                },
            },
        ],
    )?;
    let instance_images = vec![
        issue_typed_instance_image_generator(&instance_morphism, 0)?,
        issue_typed_instance_image_generator(&instance_morphism, 1)?,
    ];

    let face_context = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(40),
            name: "A".to_owned(),
            universe: 0,
        },
        Declaration::IntervalVariable {
            binder: BinderId(41),
            name: "i".to_owned(),
        },
        Declaration::CofibrationAssumption {
            binder: BinderId(42),
            name: "phi".to_owned(),
            formula: CofibrationExpr::endpoint(41, false),
        },
    ])?;
    let face =
        issue_cubical_face_map_generator(&face_context, BinderId(41), CubicalEndpoint::Zero)?;
    Ok((exchange, weakening, instance_images, face))
}

pub fn issue_e4_generator_basis_audit() -> Result<E4GeneratorBasisAuditToken, E4GeneratorError> {
    let (exchange, weakening, instance_images, face) = exemplar_contexts()?;
    let mut exemplar_derivation_hashes = vec![
        exchange.derivation_hash.clone(),
        weakening.derivation_hash.clone(),
        face.derivation_hash.clone(),
    ];
    exemplar_derivation_hashes.extend(
        instance_images
            .iter()
            .map(|token| token.derivation_hash.clone()),
    );
    let per_class = Schema2NaturalityClass::ALL
        .into_iter()
        .map(|class| PerClassGeneratorCompleteness {
            class,
            e1_context_morphism_presentation_available: true,
            class_constructor_inventory_exhaustive: false,
            normalization_naturality_for_class_proved: false,
            cubical_action_induction_for_class_proved: false,
            per_class_generator_induction_complete: false,
            named_gaps: vec![
                class.gap().to_owned(),
                E4_CLASS_GRAMMAR_GAP.to_owned(),
                E4_NORMALIZATION_NATURALITY_GAP.to_owned(),
                E4_CUBICAL_SCHEMA_ACTION_GAP.to_owned(),
            ],
        })
        .collect::<Vec<_>>();
    let every_class_complete = per_class
        .iter()
        .all(PerClassGeneratorCompleteness::complete);
    let mut token = E4GeneratorBasisAuditToken {
        token_version: E4_GENERATOR_BASIS_VERSION.to_owned(),
        generator_kinds: E4GeneratorKind::ALL.to_vec(),
        exemplar_derivation_hashes,
        current_e1_context_morphism_decomposition_complete: true,
        arbitrary_substitution_used_as_single_generator: false,
        e1_typing_support_naturality_available: true,
        per_class,
        every_class_complete,
        full_schema2_generator_completeness: false,
        global_gap: E4_GLOBAL_COMPLETENESS_GAP.to_owned(),
        independent_membership_issuance_enabled: false,
        derivation_hash: String::new(),
    };
    token.derivation_hash = tagged_hash("e4-generator-basis-audit", &token);
    Ok(token)
}

pub fn replay_e4_generator_basis_audit(
    token: &E4GeneratorBasisAuditToken,
) -> Result<(), E4GeneratorError> {
    let replay = issue_e4_generator_basis_audit()?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

fn coordinate_face_context(
    context: &FormedSchemaContext,
    dimensions: &[SchemaDimBinderId],
) -> Result<(FormedSchemaContext, Vec<BinderId>), E4GeneratorError> {
    replay_formed_context(context)?;
    let max_binder = context
        .declarations()
        .iter()
        .map(|declaration| declaration.binder().0)
        .max()
        .unwrap_or(0);
    let mut declarations = context.declarations().to_vec();
    let mut intervals = Vec::with_capacity(dimensions.len());
    for (axis, dimension) in dimensions.iter().enumerate() {
        let offset = u32::try_from(axis)
            .map_err(|_| E4GeneratorError::FreshBinderExhausted)?
            .checked_add(1)
            .ok_or(E4GeneratorError::FreshBinderExhausted)?;
        let binder = BinderId(
            max_binder
                .checked_add(offset)
                .ok_or(E4GeneratorError::FreshBinderExhausted)?,
        );
        declarations.push(Declaration::IntervalVariable {
            binder,
            name: format!("e4_i_{}", dimension.0),
        });
        intervals.push(binder);
    }
    Ok((form_schema_context(declarations)?, intervals))
}

fn reduce_registered_cube_computation(
    redex: &SchemaTermExpr,
) -> Option<(ParentCubeComputationRule, SchemaTermExpr)> {
    let SchemaTermExpr::CubeAt { cube, coordinates } = redex else {
        return None;
    };
    let SchemaTermExpr::MapCube { function, cube } = cube.as_ref() else {
        return None;
    };
    Some((
        ParentCubeComputationRule::CubeAtMapCube,
        SchemaTermExpr::App {
            function: function.clone(),
            argument: Box::new(SchemaTermExpr::CubeAt {
                cube: cube.clone(),
                coordinates: coordinates.clone(),
            }),
        },
    ))
}

fn schema_type_uses_dimension_variable(ty: &SchemaTypeExpr) -> bool {
    match ty {
        SchemaTypeExpr::Element { .. } => false,
        SchemaTypeExpr::Product { left, right } => {
            schema_type_uses_dimension_variable(left) || schema_type_uses_dimension_variable(right)
        }
        SchemaTypeExpr::Pi {
            domain, codomain, ..
        } => {
            schema_type_uses_dimension_variable(domain)
                || schema_type_uses_dimension_variable(codomain)
        }
        SchemaTypeExpr::Path { left, right, .. } => {
            schema_term_uses_dimension_variable(left) || schema_term_uses_dimension_variable(right)
        }
        SchemaTypeExpr::Cube { boundary, .. } => schema_term_uses_dimension_variable(boundary),
    }
}

fn schema_term_uses_dimension_variable(term: &SchemaTermExpr) -> bool {
    match term {
        SchemaTermExpr::Ambient { .. } | SchemaTermExpr::Variable { .. } => false,
        SchemaTermExpr::Pair { left, right } => {
            schema_term_uses_dimension_variable(left) || schema_term_uses_dimension_variable(right)
        }
        SchemaTermExpr::Lam { domain, body, .. } => {
            schema_type_uses_dimension_variable(domain) || schema_term_uses_dimension_variable(body)
        }
        SchemaTermExpr::App { function, argument } => {
            schema_term_uses_dimension_variable(function)
                || schema_term_uses_dimension_variable(argument)
        }
        SchemaTermExpr::CubeAt { cube, coordinates } => {
            schema_term_uses_dimension_variable(cube)
                || coordinates
                    .iter()
                    .any(|coordinate| matches!(coordinate, SchemaDimExpr::Variable { .. }))
        }
        SchemaTermExpr::MapCube { function, cube } => {
            schema_term_uses_dimension_variable(function)
                || schema_term_uses_dimension_variable(cube)
        }
    }
}

pub fn issue_parent_cube_action_generated(
    context: FormedSchemaContext,
    window: SupportWindow,
    locals: Vec<SchemaLocalDeclaration>,
    coordinate_dimensions: Vec<SchemaDimBinderId>,
    parent_function: SchemaTermExpr,
    registered_cube: SchemaTermExpr,
    candidate_row: SchemaTermExpr,
) -> Result<ParentCubeActionGeneratedToken, E4GeneratorError> {
    replay_formed_context(&context)?;
    if coordinate_dimensions
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>()
        .len()
        != coordinate_dimensions.len()
    {
        return Err(E4GeneratorError::MalformedSchemaInventory);
    }
    if schema_term_uses_dimension_variable(&parent_function)
        || schema_term_uses_dimension_variable(&registered_cube)
    {
        return Err(E4GeneratorError::DimensionDependentParentActionUnsupported);
    }
    let mut local_prefix = Vec::with_capacity(locals.len());
    let mut local_binders = std::collections::BTreeSet::new();
    for local in &locals {
        if !local_binders.insert(local.binder) {
            return Err(E4GeneratorError::MalformedSchemaInventory);
        }
        check_schema_type(
            &context,
            window,
            &local_prefix,
            &coordinate_dimensions,
            &local.ty,
        )
        .map_err(|error| E4GeneratorError::Schema(error.to_string()))?;
        local_prefix.push(local.clone());
    }

    let parent_function_type = infer_schema_term(
        &context,
        window,
        &locals,
        &coordinate_dimensions,
        &parent_function,
    )
    .map_err(|error| E4GeneratorError::Schema(error.to_string()))?;
    let registered_cube_type = infer_schema_term(
        &context,
        window,
        &locals,
        &coordinate_dimensions,
        &registered_cube,
    )
    .map_err(|error| E4GeneratorError::Schema(error.to_string()))?;
    let SchemaTypeExpr::Cube { dimension, .. } = &registered_cube_type else {
        return Err(E4GeneratorError::ExpectedRegisteredCube);
    };
    if usize::try_from(*dimension).ok() != Some(coordinate_dimensions.len()) {
        return Err(E4GeneratorError::MalformedSchemaInventory);
    }

    let canonical_generated_row = SchemaTermExpr::MapCube {
        function: Box::new(parent_function.clone()),
        cube: Box::new(registered_cube.clone()),
    };
    let generated_row_type = infer_schema_term(
        &context,
        window,
        &locals,
        &coordinate_dimensions,
        &canonical_generated_row,
    )
    .map_err(|error| E4GeneratorError::Schema(error.to_string()))?;
    let candidate_type = infer_schema_term(
        &context,
        window,
        &locals,
        &coordinate_dimensions,
        &candidate_row,
    )
    .map_err(|error| E4GeneratorError::Schema(error.to_string()))?;
    if candidate_row != canonical_generated_row {
        return Err(E4GeneratorError::CandidateNotCanonicalMapCube);
    }
    if candidate_type != generated_row_type {
        return Err(E4GeneratorError::ParentCubeTypeMismatch);
    }

    let (face_context, coordinate_intervals) =
        coordinate_face_context(&context, &coordinate_dimensions)?;
    let correspondence = coordinate_dimensions
        .iter()
        .copied()
        .zip(coordinate_intervals.iter().copied())
        .collect::<Vec<_>>();
    let mut face_equations = Vec::new();
    for (axis, coordinate_interval) in coordinate_intervals.iter().copied().enumerate() {
        for endpoint in [CubicalEndpoint::Zero, CubicalEndpoint::One] {
            let coordinates = coordinate_dimensions
                .iter()
                .enumerate()
                .map(|(coordinate_axis, binder)| {
                    if coordinate_axis == axis {
                        match endpoint {
                            CubicalEndpoint::Zero => SchemaDimExpr::Zero,
                            CubicalEndpoint::One => SchemaDimExpr::One,
                        }
                    } else {
                        SchemaDimExpr::Variable { binder: *binder }
                    }
                })
                .collect::<Vec<_>>();
            let left = SchemaTermExpr::CubeAt {
                cube: Box::new(canonical_generated_row.clone()),
                coordinates: coordinates.clone(),
            };
            let right = SchemaTermExpr::App {
                function: Box::new(parent_function.clone()),
                argument: Box::new(SchemaTermExpr::CubeAt {
                    cube: Box::new(registered_cube.clone()),
                    coordinates: coordinates.clone(),
                }),
            };
            let left_type =
                infer_schema_term(&context, window, &locals, &coordinate_dimensions, &left)
                    .map_err(|error| E4GeneratorError::Schema(error.to_string()))?;
            let right_type =
                infer_schema_term(&context, window, &locals, &coordinate_dimensions, &right)
                    .map_err(|error| E4GeneratorError::Schema(error.to_string()))?;
            if left_type != right_type {
                return Err(E4GeneratorError::ParentCubeTypeMismatch);
            }
            let Some((computation_rule, contractum)) = reduce_registered_cube_computation(&left)
            else {
                return Err(E4GeneratorError::CubeComputationMismatch);
            };
            if contractum != right {
                return Err(E4GeneratorError::CubeComputationMismatch);
            }
            let face_map =
                issue_cubical_face_map_generator(&face_context, coordinate_interval, endpoint)?;
            replay_cubical_face_map_generator(&face_map)?;
            let mut equation = ParentCubeFaceEquationToken {
                axis: u32::try_from(axis)
                    .map_err(|_| E4GeneratorError::MalformedSchemaInventory)?,
                endpoint,
                coordinates,
                left,
                right,
                common_type: left_type,
                computation_rule,
                computation_rule_id: computation_rule.id().to_owned(),
                face_map,
                dimension_interval_correspondence: correspondence.clone(),
                derivation_hash: String::new(),
            };
            equation.derivation_hash = tagged_hash("parent-cube-face-equation", &equation);
            face_equations.push(equation);
        }
    }
    let frozen_equality_used = false;
    let mut token = ParentCubeActionGeneratedToken {
        context,
        window,
        locals,
        coordinate_dimensions,
        parent_function,
        registered_cube,
        candidate_row,
        canonical_generated_row,
        parent_function_type,
        registered_cube_type,
        generated_row_type,
        face_equations,
        computation_rule_id: CUBE_AT_MAP_CUBE_COMPUTATION_RULE.to_owned(),
        frozen_equality_used,
        derivation_hash: String::new(),
    };
    token.derivation_hash = tagged_hash("parent-cube-action-generated", &token);
    Ok(token)
}

pub fn replay_parent_cube_action_generated(
    token: &ParentCubeActionGeneratedToken,
) -> Result<(), E4GeneratorError> {
    let replay = issue_parent_cube_action_generated(
        token.context.clone(),
        token.window,
        token.locals.clone(),
        token.coordinate_dimensions.clone(),
        token.parent_function.clone(),
        token.registered_cube.clone(),
        token.candidate_row.clone(),
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

fn reconstruct_step8_registered_signatures(
    boundary_derivation_hash: &str,
    expected_schema_signature_derivation_hash: &str,
) -> Result<Step8RegisteredSignatures, E4GeneratorError> {
    let carrier = TypeExpr::parameter(0);
    let base = TermExpr::variable(1);
    let context = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(0),
            name: "S3".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(1),
            name: "base".to_owned(),
            ty: carrier.clone(),
        },
    ])?;
    let bundle = DerivationRef::parse(boundary_derivation_hash.to_owned())
        .map_err(|error| E4GeneratorError::Step8(error.to_string()))?;
    let signatures = issue_step8_registered_signatures(context, carrier.clone(), base, bundle)
        .map_err(|error| E4GeneratorError::Step8(error.to_string()))?;
    replay_step8_registered_signatures(&signatures)
        .map_err(|error| E4GeneratorError::Step8(error.to_string()))?;
    if signatures.derivation_hash() != expected_schema_signature_derivation_hash {
        return Err(E4GeneratorError::Step8InvariantDrift);
    }
    Ok(signatures)
}

fn issue_sub_basis_inclusion(
    sub_basis_kinds: Vec<E4GeneratorKind>,
) -> Result<E4SubBasisInclusionProof, E4GeneratorError> {
    let full_basis = issue_e4_generator_basis_audit()?;
    let sub_basis_set = sub_basis_kinds
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if sub_basis_set.len() != sub_basis_kinds.len() {
        return Err(E4GeneratorError::Step8InvariantDrift);
    }
    let full_basis_kinds = E4GeneratorKind::ALL.to_vec();
    let every_sub_basis_kind_registered = sub_basis_kinds
        .iter()
        .all(|kind| full_basis_kinds.contains(kind));
    if !every_sub_basis_kind_registered {
        return Err(E4GeneratorError::Step8InvariantDrift);
    }
    let mut proof = E4SubBasisInclusionProof {
        sub_basis_kinds,
        full_basis_kinds,
        full_basis_audit_derivation_hash: full_basis.derivation_hash().to_owned(),
        every_sub_basis_kind_registered,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = tagged_hash("e4-sub-basis-inclusion", &proof);
    Ok(proof)
}

struct Step8MapCubeTermProofParts {
    registered_signatures: Step8RegisteredSignatures,
    parent_cube_action: ParentCubeActionGeneratedToken,
}

/// Shared term-level proof for the historical and source-first M1 issuers.
/// It consumes only a typed boundary derivation and the expected Schema2
/// signature hash.  Prefix validation, archival bridges, weakening
/// exemplars, counts, bars and verdicts are intentionally outside this
/// judgment.
fn issue_step8_map_cube_term_proof(
    boundary_derivation_hash: &str,
    expected_schema_signature_derivation_hash: &str,
) -> Result<Step8MapCubeTermProofParts, E4GeneratorError> {
    let registered_signatures = reconstruct_step8_registered_signatures(
        boundary_derivation_hash,
        expected_schema_signature_derivation_hash,
    )?;
    let carrier = TypeExpr::parameter(0);
    let base = TermExpr::variable(1);
    let context = form_schema_context(vec![
        Declaration::TypeParameter {
            binder: BinderId(0),
            name: "S3".to_owned(),
            universe: 0,
        },
        Declaration::OpaqueElement {
            binder: BinderId(1),
            name: "base".to_owned(),
            ty: carrier.clone(),
        },
    ])?;
    let window =
        SupportWindow::new(7, 8).map_err(|error| E4GeneratorError::Step8(error.to_string()))?;
    let bundle_derivation = DerivationRef::parse(boundary_derivation_hash.to_owned())
        .map_err(|error| E4GeneratorError::Step8(error.to_string()))?;
    let bundle = RegisteredBoundaryBundleRef {
        derivation: bundle_derivation,
        source_step: 8,
        carrier: carrier.clone(),
        dimension: 3,
    };
    let element = SchemaTypeExpr::Element {
        carrier: carrier.clone(),
    };
    let path_cell = SchemaBinderId(0);
    let operation = SchemaBinderId(1);
    let pair_argument = SchemaBinderId(2);
    let family_argument = SchemaBinderId(3);
    let translated_point = SchemaBinderId(4);
    let path_cell_declaration = SchemaLocalDeclaration {
        binder: path_cell,
        ty: SchemaTypeExpr::Cube {
            carrier: carrier.clone(),
            boundary: Box::new(SchemaTermExpr::Ambient { term: base }),
            dimension: 3,
            source_bundle: bundle,
        },
    };
    let operation_declaration = SchemaLocalDeclaration {
        binder: operation,
        ty: SchemaTypeExpr::Pi {
            binder: pair_argument,
            domain: Box::new(SchemaTypeExpr::Product {
                left: Box::new(element.clone()),
                right: Box::new(element.clone()),
            }),
            codomain: Box::new(element.clone()),
        },
    };
    let family_declaration = SchemaLocalDeclaration {
        binder: family_argument,
        ty: element.clone(),
    };
    let parent_function = SchemaTermExpr::Lam {
        binder: translated_point,
        domain: Box::new(element.clone()),
        body: Box::new(SchemaTermExpr::App {
            function: Box::new(SchemaTermExpr::Variable { binder: operation }),
            argument: Box::new(SchemaTermExpr::Pair {
                left: Box::new(SchemaTermExpr::Variable {
                    binder: translated_point,
                }),
                right: Box::new(SchemaTermExpr::Variable {
                    binder: family_argument,
                }),
            }),
        }),
    };
    let registered_cube = SchemaTermExpr::Variable { binder: path_cell };
    let SchemaTermExpr::Lam {
        binder: row_binder,
        domain: row_domain,
        body: candidate_body,
    } = registered_signatures.cell_action_term()
    else {
        return Err(E4GeneratorError::Step8InvariantDrift);
    };
    if *row_binder != family_argument || row_domain.as_ref() != &element {
        return Err(E4GeneratorError::Step8InvariantDrift);
    }
    let parent_cube_action = issue_parent_cube_action_generated(
        context.clone(),
        window,
        vec![
            path_cell_declaration.clone(),
            operation_declaration.clone(),
            family_declaration,
        ],
        vec![
            SchemaDimBinderId(10),
            SchemaDimBinderId(11),
            SchemaDimBinderId(12),
        ],
        parent_function,
        registered_cube,
        candidate_body.as_ref().clone(),
    )?;
    replay_parent_cube_action_generated(&parent_cube_action)?;
    let canonical_full_row = SchemaTermExpr::Lam {
        binder: family_argument,
        domain: Box::new(element),
        body: Box::new(parent_cube_action.canonical_generated_row.clone()),
    };
    if &canonical_full_row != registered_signatures.cell_action_term() {
        return Err(E4GeneratorError::Step8InvariantDrift);
    }
    let full_row_type = infer_schema_term(
        &context,
        window,
        &[path_cell_declaration, operation_declaration],
        &[],
        &canonical_full_row,
    )
    .map_err(|error| E4GeneratorError::Schema(error.to_string()))?;
    if &full_row_type != registered_signatures.cell_action_type() {
        return Err(E4GeneratorError::Step8InvariantDrift);
    }
    Ok(Step8MapCubeTermProofParts {
        registered_signatures,
        parent_cube_action,
    })
}

fn issue_prefix_local_sub_basis_inclusion()
-> Result<Step8PrefixLocalSubBasisInclusionProof, E4GeneratorError> {
    let sub_basis_kinds = vec![E4GeneratorKind::CubicalFaceMap];
    let full_basis_kinds = E4GeneratorKind::ALL.to_vec();
    let every_sub_basis_kind_registered = sub_basis_kinds
        .iter()
        .all(|kind| full_basis_kinds.contains(kind));
    if !every_sub_basis_kind_registered {
        return Err(E4GeneratorError::Step8InvariantDrift);
    }
    let full_basis_inventory_hash =
        prefix_local_tagged_hash("closed-e4-generator-kind-inventory", &full_basis_kinds);
    let mut proof = Step8PrefixLocalSubBasisInclusionProof {
        sub_basis_kinds,
        full_basis_kinds,
        full_basis_inventory_hash,
        every_sub_basis_kind_registered,
        weakening_exemplar_replayed: false,
        derivation_hash: String::new(),
    };
    proof.derivation_hash = prefix_local_tagged_hash("sub-basis-inclusion", &proof);
    Ok(proof)
}

pub fn issue_step8_cell_action_m1_generated(
    step8: &Step8R2TypedSignaturesToken,
) -> Result<M1GeneratedMembershipToken, E4GeneratorError> {
    replay_step8_r2_typed_signatures_token(step8)
        .map_err(|error| E4GeneratorError::Step8(error.to_string()))?;
    if step8.rule != DERIVED_ACTION_MEMBERSHIP_RULE
        || step8.registered_boundary_dimension != 3
        || !step8.operation_signature_typed
        || !step8.cell_action_term_typed
        || !step8.cell_action_boundary_derived_by_map_cube
        || step8.cell_action_generator_membership_decided
        || step8.independent_family_tokens_issued != 0
    {
        return Err(E4GeneratorError::Step8InvariantDrift);
    }
    let term_proof = issue_step8_map_cube_term_proof(
        &step8.registered_boundary_bundle_derivation_hash,
        &step8.schema_signature_derivation_hash,
    )?;

    let sub_basis_inclusion = issue_sub_basis_inclusion(vec![E4GeneratorKind::CubicalFaceMap])?;
    let mut generating_derivation = Step8MapCubeGenerationProof {
        step8: step8.clone(),
        registered_signatures: Box::new(term_proof.registered_signatures),
        parent_cube_action: Box::new(term_proof.parent_cube_action),
        sub_basis_inclusion,
        derivation_hash: String::new(),
    };
    generating_derivation.derivation_hash = tagged_hash(
        "step8-map-cube-generating-derivation",
        &generating_derivation,
    );
    let sub_basis_digest = tagged_hash(
        "step8-map-cube-sub-basis",
        &(
            E4GeneratorKind::CubicalFaceMap,
            &generating_derivation.parent_cube_action,
            &generating_derivation.sub_basis_inclusion,
            &step8.schema_signature_derivation_hash,
            &step8.registered_boundary_bundle_derivation_hash,
        ),
    );
    let subject = M1GeneratedSubject::Step8CellAction {
        step8_derivation_hash: step8.derivation_hash.clone(),
        schema_signature_derivation_hash: step8.schema_signature_derivation_hash.clone(),
    };
    let mut token = M1GeneratedMembershipToken {
        rule: E4_M1_RULE.to_owned(),
        membership_rule: DERIVED_ACTION_MEMBERSHIP_RULE.to_owned(),
        subject,
        sub_basis_digest,
        generating_derivation,
        final_by_basis_monotonicity: true,
        full_e4_completeness_used: false,
        derivation_hash: String::new(),
    };
    token.derivation_hash = tagged_hash("m1-generated-membership", &token);
    Ok(token)
}

/// Issue the monotone M1 verdict against a supplied exact Step-8 prefix and
/// candidate.  Both the R2 replay and the MapCube proof remain bound to those
/// inputs; no historical compatibility token is constructed on this path.
fn issue_step8_cell_action_prefix_m1_generated_under(
    predecessor: &SealedSignature,
    current: &Telescope,
    step8: &Step8R2PrefixLocalTypedSignaturesToken,
    prefix_general: bool,
) -> Result<Step8PrefixLocalM1GeneratedMembershipToken, E4GeneratorError> {
    if prefix_general {
        replay_step8_r2_prefix_general_typed_signatures_token_v2(predecessor, current, step8)
    } else {
        replay_step8_r2_prefix_local_typed_signatures_token(predecessor, current, step8)
    }
    .map_err(|error| E4GeneratorError::Step8(error.to_string()))?;
    if step8.rule != DERIVED_ACTION_MEMBERSHIP_RULE
        || step8.source_step != 8
        || step8.predecessor_signature_digest != predecessor.digest()
        || step8.source_telescope_hash != candidate_hash(current)
        || step8.registered_boundary_dimension != 3
        || !step8.operation_signature_typed
        || !step8.cell_action_term_typed
        || !step8.cell_action_boundary_derived_by_map_cube
        || step8.cell_action_generator_membership_decided
        || step8.independent_family_tokens_issued != 0
        || !step8.capabilities.forbidden_inputs_withheld()
    {
        return Err(E4GeneratorError::Step8InvariantDrift);
    }
    let term_proof = issue_step8_map_cube_term_proof(
        &step8.typed_boundary_derivation_hash,
        &step8.schema_signature_derivation_hash,
    )?;
    let sub_basis_inclusion = issue_prefix_local_sub_basis_inclusion()?;
    if sub_basis_inclusion.weakening_exemplar_replayed {
        return Err(E4GeneratorError::Step8InvariantDrift);
    }
    let mut generating_derivation = Step8PrefixLocalMapCubeGenerationProof {
        step8: step8.clone(),
        registered_signatures: Box::new(term_proof.registered_signatures),
        parent_cube_action: Box::new(term_proof.parent_cube_action),
        sub_basis_inclusion,
        derivation_hash: String::new(),
    };
    generating_derivation.derivation_hash = prefix_m1_tagged_hash(
        prefix_general,
        "step8-map-cube-generating-derivation",
        &generating_derivation,
    );
    let sub_basis_digest = prefix_m1_tagged_hash(
        prefix_general,
        "step8-map-cube-sub-basis",
        &(
            E4GeneratorKind::CubicalFaceMap,
            &generating_derivation.parent_cube_action,
            &generating_derivation.sub_basis_inclusion,
            &step8.schema_signature_derivation_hash,
            &step8.typed_boundary_derivation_hash,
        ),
    );
    let subject = M1GeneratedSubject::Step8CellAction {
        step8_derivation_hash: step8.derivation_hash.clone(),
        schema_signature_derivation_hash: step8.schema_signature_derivation_hash.clone(),
    };
    let typed_dependencies = Step8PrefixLocalM1TypedDependencies {
        predecessor_signature_digest: predecessor.digest().to_owned(),
        source_telescope_hash: candidate_hash(current),
        typed_boundary_dependency_hash: step8.typed_boundary_dependency_hash.clone(),
        typed_boundary_derivation_hash: step8.typed_boundary_derivation_hash.clone(),
        reference_only_derivation_hash: step8.reference_only_derivation_hash.clone(),
        base_binding_derivation_hash: step8
            .base_binding_derivation_hash
            .clone()
            .ok_or(E4GeneratorError::Step8InvariantDrift)?,
        element_overlay_derivation_hash: step8
            .element_overlay_derivation_hash
            .clone()
            .ok_or(E4GeneratorError::Step8InvariantDrift)?,
        schema_signature_derivation_hash: step8.schema_signature_derivation_hash.clone(),
        parent_cube_action_derivation_hash: generating_derivation
            .parent_cube_action
            .derivation_hash()
            .to_owned(),
        sub_basis_inclusion_derivation_hash: generating_derivation
            .sub_basis_inclusion
            .derivation_hash
            .clone(),
    };
    let capabilities = Step8PrefixLocalM1Capabilities {
        supplied_exact_prefix_read: true,
        supplied_exact_candidate_read: true,
        source_bound_typed_boundary_read: true,
        term_level_map_cube_computation_read: true,
        cubical_face_map_rule_read: true,
        definition_level_sub_basis_inclusion_read: true,
        weakening_exemplar_read: false,
        archival_constant_bridge_read: false,
        archive_input_read: false,
        historical_count_input_read: false,
        acceptance_bar_input_read: false,
        membership_verdict_input_read: false,
        enacted_future_input_read: false,
    };
    if !capabilities.forbidden_inputs_withheld() {
        return Err(E4GeneratorError::Step8InvariantDrift);
    }
    let mut token = Step8PrefixLocalM1GeneratedMembershipToken {
        token_version: if prefix_general {
            E4_PREFIX_GENERAL_M1_VERSION
        } else {
            E4_PREFIX_LOCAL_M1_VERSION
        }
        .to_owned(),
        rule: E4_M1_RULE.to_owned(),
        membership_rule: DERIVED_ACTION_MEMBERSHIP_RULE.to_owned(),
        subject,
        typed_dependencies,
        sub_basis_digest,
        generating_derivation,
        final_by_basis_monotonicity: true,
        full_e4_completeness_used: false,
        capabilities,
        derivation_hash: String::new(),
    };
    token.derivation_hash =
        prefix_m1_tagged_hash(prefix_general, "m1-generated-membership", &token);
    Ok(token)
}

pub fn issue_step8_cell_action_prefix_local_m1_generated(
    predecessor: &SealedSignature,
    current: &Telescope,
    step8: &Step8R2PrefixLocalTypedSignaturesToken,
) -> Result<Step8PrefixLocalM1GeneratedMembershipToken, E4GeneratorError> {
    issue_step8_cell_action_prefix_m1_generated_under(predecessor, current, step8, false)
}

/// Prefix-general BI-1b successor.  It differs from the v1 API only in the
/// source replay theorem used for the Step-8 typed-boundary dependency.
pub fn issue_step8_cell_action_prefix_general_m1_generated_v2(
    predecessor: &SealedSignature,
    current: &Telescope,
    step8: &Step8R2PrefixLocalTypedSignaturesToken,
) -> Result<Step8PrefixLocalM1GeneratedMembershipToken, E4GeneratorError> {
    issue_step8_cell_action_prefix_m1_generated_under(predecessor, current, step8, true)
}

pub fn replay_step8_cell_action_prefix_local_m1_generated(
    predecessor: &SealedSignature,
    current: &Telescope,
    token: &Step8PrefixLocalM1GeneratedMembershipToken,
) -> Result<(), E4GeneratorError> {
    let replay = issue_step8_cell_action_prefix_local_m1_generated(
        predecessor,
        current,
        token.generating_derivation.step8(),
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

pub fn replay_step8_cell_action_prefix_general_m1_generated_v2(
    predecessor: &SealedSignature,
    current: &Telescope,
    token: &Step8PrefixLocalM1GeneratedMembershipToken,
) -> Result<(), E4GeneratorError> {
    let replay = issue_step8_cell_action_prefix_general_m1_generated_v2(
        predecessor,
        current,
        token.generating_derivation.step8(),
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

pub fn replay_m1_generated_membership(
    token: &M1GeneratedMembershipToken,
) -> Result<(), E4GeneratorError> {
    let replay = issue_step8_cell_action_m1_generated(&token.generating_derivation.step8)?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

pub fn issue_e4_development_audit() -> Result<E4DevelopmentAuditToken, E4GeneratorError> {
    let basis = issue_e4_generator_basis_audit()?;
    let stage1 = issue_stage1_r1_package_token()
        .map_err(|error| E4GeneratorError::Upstream(error.to_string()))?;
    replay_stage1_r1_package_token(&stage1)
        .map_err(|error| E4GeneratorError::Upstream(error.to_string()))?;
    let step8 = issue_step8_r2_typed_signatures_token()
        .map_err(|error| E4GeneratorError::Step8(error.to_string()))?;
    replay_step8_r2_typed_signatures_token(&step8)
        .map_err(|error| E4GeneratorError::Step8(error.to_string()))?;

    let generated = issue_step8_cell_action_m1_generated(&step8)?;
    replay_m1_generated_membership(&generated)?;
    let mut pending = stage1
        .local_role_inventory
        .iter()
        .map(|entry| M1PendingMembershipRecord {
            subject: M1PendingSubject::Stage1CarrierException { role: entry.role },
            source_derivation_hash: stage1.derivation_hash.clone(),
            obstruction: R1_MEMBERSHIP_GAP.to_owned(),
            independent_verdict_withheld: true,
        })
        .collect::<Vec<_>>();
    pending.push(M1PendingMembershipRecord {
        subject: M1PendingSubject::Step8LeftUnitCoherence,
        source_derivation_hash: step8.derivation_hash.clone(),
        obstruction: E4_GLOBAL_COMPLETENESS_GAP.to_owned(),
        independent_verdict_withheld: true,
    });
    let forbidden_outputs = E4ForbiddenOutputAudit {
        independent_verdict_issued: false,
        ordinary_family_token_issued: false,
        historical_stage_score_issued: false,
        fq2_evaluated: false,
        agent_a_handoff_issued: false,
        e2b_quotient_closure_executed: false,
    };
    let mut token = E4DevelopmentAuditToken {
        token_version: E4_GENERATOR_BASIS_VERSION.to_owned(),
        basis,
        m1_generated: vec![generated],
        m1_pending: pending,
        full_e4_completeness: false,
        global_gap: E4_GLOBAL_COMPLETENESS_GAP.to_owned(),
        forbidden_outputs,
        derivation_hash: String::new(),
    };
    token.derivation_hash = tagged_hash("e4-development-audit", &token);
    Ok(token)
}

pub fn replay_e4_development_audit(
    token: &E4DevelopmentAuditToken,
) -> Result<(), E4GeneratorError> {
    let replay = issue_e4_development_audit()?;
    if replay == *token {
        Ok(())
    } else {
        Err(E4GeneratorError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::step8_r2::{
        issue_step8_r2_prefix_local_typed_signatures_token, issue_step8_r2_typed_signatures_token,
    };
    use std::collections::BTreeSet;

    fn dependent_context() -> FormedSchemaContext {
        form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(1),
                name: "A".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(2),
                name: "x".to_owned(),
                ty: TypeExpr::parameter(1),
            },
            Declaration::IntervalVariable {
                binder: BinderId(3),
                name: "i".to_owned(),
            },
            Declaration::CofibrationAssumption {
                binder: BinderId(4),
                name: "phi".to_owned(),
                formula: CofibrationExpr::And {
                    terms: vec![CofibrationExpr::endpoint(3, false), CofibrationExpr::True],
                },
            },
        ])
        .unwrap()
    }

    #[test]
    fn all_four_kinds_operate_but_global_completeness_is_false() {
        let token = issue_e4_generator_basis_audit().unwrap();
        replay_e4_generator_basis_audit(&token).unwrap();
        assert_eq!(token.generator_kinds, E4GeneratorKind::ALL);
        assert!(token.current_e1_context_morphism_decomposition_complete());
        assert!(!token.arbitrary_substitution_used_as_single_generator);
        assert!(!token.full_schema2_generator_completeness());
        assert!(!token.independent_membership_issuance_enabled());
        assert_eq!(token.per_class().len(), 8);
        assert!(token.per_class().iter().all(|class| !class.complete()));
    }

    #[test]
    fn dependent_cubical_face_restricts_cofibration_and_replays() {
        let source = dependent_context();
        let face =
            issue_cubical_face_map_generator(&source, BinderId(3), CubicalEndpoint::Zero).unwrap();
        replay_cubical_face_map_generator(&face).unwrap();
        assert_eq!(face.dependent_cofibrations_restricted, 1);
        assert_eq!(face.target().interval_binders(), Vec::<BinderId>::new());
        assert!(matches!(
            &face.target().declarations()[2],
            Declaration::CofibrationAssumption {
                formula: CofibrationExpr::And { terms }, ..
            } if terms[0] == CofibrationExpr::True
        ));
    }

    #[test]
    fn genuine_instance_is_a_strict_prefix_chain_not_a_catch_all() {
        let source = form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(1),
                name: "A".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(2),
                name: "x".to_owned(),
                ty: TypeExpr::parameter(1),
            },
        ])
        .unwrap();
        let target = form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(10),
                name: "B".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(11),
                name: "y".to_owned(),
                ty: TypeExpr::parameter(10),
            },
        ])
        .unwrap();
        let morphism = issue_typed_substitution(
            &source,
            &target,
            vec![
                SubstitutionImage::Type {
                    source: BinderId(1),
                    image: TypeExpr::trunc(TypeExpr::parameter(10)),
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
        .unwrap();
        let decomposition = decompose_legal_morphism(&morphism).unwrap();
        replay_legal_morphism_decomposition(&decomposition).unwrap();
        let LegalMorphismPresentation::TypedInstanceImages { generators } =
            decomposition.presentation()
        else {
            panic!("genuine instance must use image generators")
        };
        assert_eq!(generators.len(), 2);
        assert_eq!(generators[0].prefix_length(), 1);
        assert_eq!(generators[1].prefix_length(), 2);
        assert_eq!(generators[1].prefix_substitution(), &morphism);
        assert!(decomposition.no_arbitrary_substitution_catch_all);
    }

    #[test]
    fn identity_exchange_weakening_and_face_presentations_are_distinct() {
        let (exchange, weakening, _, face) = exemplar_contexts().unwrap();
        let identity = identity_substitution(exchange.morphism().source()).unwrap();
        assert!(matches!(
            decompose_legal_morphism(&identity).unwrap().presentation(),
            LegalMorphismPresentation::Identity
        ));
        assert!(matches!(
            decompose_legal_morphism(exchange.morphism())
                .unwrap()
                .presentation(),
            LegalMorphismPresentation::AdjacentExchange { .. }
        ));
        assert!(matches!(
            decompose_legal_morphism(weakening.morphism())
                .unwrap()
                .presentation(),
            LegalMorphismPresentation::Weakening { .. }
        ));
        assert!(matches!(
            decompose_legal_morphism(face.morphism())
                .unwrap()
                .presentation(),
            LegalMorphismPresentation::CubicalFaceMap { .. }
        ));
    }

    #[test]
    fn typing_support_square_does_not_claim_normalization_naturality() {
        let source = form_schema_context(vec![Declaration::TypeParameter {
            binder: BinderId(1),
            name: "A".to_owned(),
            universe: 0,
        }])
        .unwrap();
        let target = form_schema_context(vec![Declaration::TypeParameter {
            binder: BinderId(2),
            name: "B".to_owned(),
            universe: 0,
        }])
        .unwrap();
        let morphism = issue_typed_substitution(
            &source,
            &target,
            vec![SubstitutionImage::Type {
                source: BinderId(1),
                image: TypeExpr::parameter(2),
            }],
        )
        .unwrap();
        let square = issue_e1_typing_support_square(
            decompose_legal_morphism(&morphism).unwrap(),
            TypedExpression::Type {
                expression: TypeExpr::parameter(1),
            },
        )
        .unwrap();
        replay_e1_typing_support_square(&square).unwrap();
        assert!(square.typing_naturality_proved());
        assert!(square.support_naturality_proved());
        assert!(!square.normalization_naturality_proved());
    }

    #[test]
    fn exact_step8_map_cube_issues_only_m1_generated() {
        let step8 = issue_step8_r2_typed_signatures_token().unwrap();
        let generated = issue_step8_cell_action_m1_generated(&step8).unwrap();
        replay_m1_generated_membership(&generated).unwrap();
        assert_eq!(
            generated.derivation_hash(),
            "blake3:563760a3d4326aca4fbb08130df2d13fe2c1cedddd1650fb9874ecbe8d5bbe50"
        );
        assert!(generated.final_by_basis_monotonicity());
        assert!(!generated.full_e4_completeness_used());
        assert_eq!(
            generated
                .generating_derivation
                .parent_cube_action
                .face_equations()
                .len(),
            6
        );
        assert!(
            generated
                .generating_derivation
                .parent_cube_action
                .face_equations()
                .iter()
                .all(|equation| {
                    let Some((ParentCubeComputationRule::CubeAtMapCube, contractum)) =
                        reduce_registered_cube_computation(equation.left())
                    else {
                        return false;
                    };
                    contractum == *equation.right()
                        && replay_cubical_face_map_generator(&equation.face_map).is_ok()
                })
        );
        assert_eq!(
            generated
                .generating_derivation
                .parent_cube_action
                .face_equations()
                .iter()
                .map(|equation| (equation.axis(), equation.endpoint()))
                .collect::<BTreeSet<_>>()
                .len(),
            6
        );
        assert!(
            !generated
                .generating_derivation
                .parent_cube_action
                .frozen_equality_used()
        );
        assert!(!generated.sub_basis_digest().is_empty());
    }

    #[test]
    fn prefix_local_step8_map_cube_is_source_bound_and_weakening_free() {
        let prefix = SealedSignature::from_telescopes(
            (1_u32..=7)
                .map(|step| (step, Telescope::reference(step)))
                .collect(),
        );
        let candidate = Telescope::reference(8);
        let step8 =
            issue_step8_r2_prefix_local_typed_signatures_token(&prefix, &candidate).unwrap();
        let generated =
            issue_step8_cell_action_prefix_local_m1_generated(&prefix, &candidate, &step8).unwrap();
        replay_step8_cell_action_prefix_local_m1_generated(&prefix, &candidate, &generated)
            .unwrap();
        assert!(generated.final_by_basis_monotonicity);
        assert!(!generated.full_e4_completeness_used);
        assert_eq!(
            generated.typed_dependencies.predecessor_signature_digest,
            prefix.digest()
        );
        assert_eq!(
            generated.typed_dependencies.source_telescope_hash,
            candidate_hash(&candidate)
        );
        assert_eq!(
            generated
                .generating_derivation
                .parent_cube_action()
                .face_equations()
                .len(),
            6
        );
        assert!(generated.capabilities.source_bound_typed_boundary_read);
        assert!(generated.capabilities.term_level_map_cube_computation_read);
        assert!(
            generated
                .capabilities
                .definition_level_sub_basis_inclusion_read
        );
        assert!(generated.capabilities.forbidden_inputs_withheld());
        assert!(!generated.capabilities.weakening_exemplar_read);
        assert!(
            !generated
                .generating_derivation
                .sub_basis_inclusion
                .weakening_exemplar_replayed
        );
    }

    #[test]
    fn prefix_local_m1_mutation_fails_after_rehash() {
        let prefix = SealedSignature::from_telescopes(
            (1_u32..=7)
                .map(|step| (step, Telescope::reference(step)))
                .collect(),
        );
        let candidate = Telescope::reference(8);
        let step8 =
            issue_step8_r2_prefix_local_typed_signatures_token(&prefix, &candidate).unwrap();
        let mut generated =
            issue_step8_cell_action_prefix_local_m1_generated(&prefix, &candidate, &step8).unwrap();
        generated.capabilities.archive_input_read = true;
        generated.derivation_hash.clear();
        generated.derivation_hash = prefix_local_tagged_hash("m1-generated-membership", &generated);
        assert_eq!(
            replay_step8_cell_action_prefix_local_m1_generated(&prefix, &candidate, &generated,),
            Err(E4GeneratorError::ReplayMismatch)
        );
    }

    #[test]
    fn illegal_exchange_face_and_mutated_m1_proof_fail_closed() {
        let source = dependent_context();
        assert!(matches!(
            issue_adjacent_exchange_generator(&source, 0),
            Err(E4GeneratorError::Context(message)) if message.contains("depends")
        ));
        assert_eq!(
            issue_cubical_face_map_generator(&source, BinderId(2), CubicalEndpoint::Zero),
            Err(E4GeneratorError::NotInterval {
                binder: BinderId(2)
            })
        );
        let step8 = issue_step8_r2_typed_signatures_token().unwrap();
        let mut generated = issue_step8_cell_action_m1_generated(&step8).unwrap();
        generated.sub_basis_digest.push('0');
        assert_eq!(
            replay_m1_generated_membership(&generated),
            Err(E4GeneratorError::ReplayMismatch)
        );

        let mut invented_candidate = issue_step8_cell_action_m1_generated(&step8).unwrap();
        invented_candidate
            .generating_derivation
            .parent_cube_action
            .candidate_row = SchemaTermExpr::Variable {
            binder: SchemaBinderId(0),
        };
        assert_eq!(
            replay_m1_generated_membership(&invented_candidate),
            Err(E4GeneratorError::ReplayMismatch)
        );

        let relation = &issue_step8_cell_action_m1_generated(&step8)
            .unwrap()
            .generating_derivation
            .parent_cube_action;
        assert_eq!(
            issue_parent_cube_action_generated(
                relation.context.clone(),
                relation.window,
                relation.locals.clone(),
                relation.coordinate_dimensions.clone(),
                relation.parent_function.clone(),
                relation.registered_cube.clone(),
                relation.registered_cube.clone(),
            ),
            Err(E4GeneratorError::CandidateNotCanonicalMapCube)
        );

        let mut mutated_equation = relation.as_ref().clone();
        mutated_equation.face_equations[0].right = mutated_equation.face_equations[0].left.clone();
        assert_eq!(
            replay_parent_cube_action_generated(&mutated_equation),
            Err(E4GeneratorError::ReplayMismatch)
        );

        let dimension_dependent_parent = SchemaTermExpr::CubeAt {
            cube: Box::new(relation.registered_cube.clone()),
            coordinates: relation
                .coordinate_dimensions
                .iter()
                .copied()
                .map(|binder| SchemaDimExpr::Variable { binder })
                .collect(),
        };
        assert_eq!(
            issue_parent_cube_action_generated(
                relation.context.clone(),
                relation.window,
                relation.locals.clone(),
                relation.coordinate_dimensions.clone(),
                dimension_dependent_parent,
                relation.registered_cube.clone(),
                relation.candidate_row.clone(),
            ),
            Err(E4GeneratorError::DimensionDependentParentActionUnsupported)
        );
    }

    #[test]
    fn prefix_and_audit_mutations_fail_replay() {
        let (_, _, instance_images, _) = exemplar_contexts().unwrap();
        let full = &instance_images[1].prefix_substitution;
        let mut image = instance_images[0].clone();
        image.derivation_hash.push('0');
        assert_eq!(
            replay_typed_instance_image_generator(full, &image),
            Err(E4GeneratorError::ReplayMismatch)
        );
        let mut audit = issue_e4_generator_basis_audit().unwrap();
        audit.full_schema2_generator_completeness = true;
        audit.independent_membership_issuance_enabled = true;
        assert_eq!(
            replay_e4_generator_basis_audit(&audit),
            Err(E4GeneratorError::ReplayMismatch)
        );
    }

    #[test]
    fn generator_kind_inventory_is_finite_and_duplicate_free() {
        let kinds = E4GeneratorKind::ALL.into_iter().collect::<BTreeSet<_>>();
        assert_eq!(kinds.len(), E4GeneratorKind::ALL.len());
    }

    #[test]
    fn development_audit_is_stable_partial_and_f_p1_clean() {
        let token = issue_e4_development_audit().unwrap();
        replay_e4_development_audit(&token).unwrap();
        assert!(!token.full_e4_completeness());
        assert_eq!(token.m1_generated().len(), 1);
        assert_eq!(token.m1_pending().len(), 5);
        assert!(token.forbidden_outputs().all_withheld());
        assert!(
            token
                .m1_pending()
                .iter()
                .all(|record| !record.obstruction().is_empty())
        );

        let mut invented_output = token;
        invented_output.forbidden_outputs.fq2_evaluated = true;
        assert_eq!(
            replay_e4_development_audit(&invented_output),
            Err(E4GeneratorError::ReplayMismatch)
        );
    }
}
