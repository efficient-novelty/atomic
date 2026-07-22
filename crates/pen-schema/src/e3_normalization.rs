//! Frozen, count-blind E-3 normalization and equality for the typed E-2
//! schema fragment.
//!
//! The only computation rule is schema-level beta.  Ambient E-1 terms and
//! sealed library references are opaque.  Normalization uses a deterministic
//! input-derived budget: every attempt terminates, while exhaustion remains a
//! fail-closed result rather than a claim of total normalization.  Issued
//! tokens re-check typing before and after reduction and retain both source
//! and surviving provenance.
//!
//! This module deliberately issues no generator-membership verdict, family
//! token, count, F-Q2 result, or handoff.  In particular, general C6 and
//! transport of registered boundary-bundle provenance through arbitrary
//! typed substitutions remain explicit successor obligations.

use crate::context::{
    BinderId, FormedSchemaContext, LibraryKey, SchemaContextError, SchemaSupport,
    SubstitutionImage, TermExpr, TypeExpr, TypedExpression, TypedSubstitutionToken,
    expression_support, replay_formed_context, replay_typed_substitution, weakening_substitution,
};
use crate::grammar::{
    ClauseAnchor, OrdinaryGrammarError, OrdinaryInterpretation, OrdinarySchema, OrdinarySchemaKind,
    RegisteredBoundaryBundleRef, SchemaBinderId, SchemaDimBinderId, SchemaDimExpr,
    SchemaLocalDeclaration, SchemaTermExpr, SchemaTypeExpr, SupportSourceTelescope, SupportWindow,
    check_schema_type, infer_schema_term, replay_ordinary_schema,
};
use pen_type::equality::KERNEL_EQUALITY_PROCEDURE;
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const SCHEMA2_E3_FROZEN_FRAGMENT_VERSION: &str = "schema2-e3-frozen-fragment-v1";
pub const SCHEMA2_FROZEN_EQUALITY_PROCEDURE: &str = "schema2-e3-v1-typed-beta-normal-equality";
pub const E3_SUCCESSFUL_NORMALIZATION_SCOPE: &str =
    "successfully normalized well-typed terms in the finite E1/E2 SchemaTermExpr inventory";
pub const E3_TOTAL_NORMALIZATION_GAP: &str =
    "E3_TOTAL_NORMALIZATION_FOR_ALL_WELL_TYPED_SCHEMA2_TERMS_NOT_PROVED";
pub const E3_GENERAL_C6_GAP: &str =
    "C6_V3_GENERAL_HISTORICAL_TERM_LEVEL_COMPLETION_NOT_PROVED_BY_E3_FRAGMENT";
pub const E3_CUBICAL_SUBSTITUTION_PROVENANCE_GAP: &str =
    "E3_REGISTERED_BOUNDARY_BUNDLE_PROVENANCE_TRANSPORT_UNDER_SUBSTITUTION_NOT_PROVED";
pub const E3_CONTEXT_PRESENTATION_GAP: &str =
    "E3_ARBITRARY_ALPHA_EQUIVALENT_CONTEXT_PRESENTATION_QUOTIENT_NOT_PROVED";
pub const E3_GENERAL_UNIVALENT_EQUALITY_GAP: &str =
    "E3_UNIVALENT_EQUALITY_BEYOND_SCOPED_BETA_NORMAL_EQUALITY_NOT_PROVED";
pub const E3_NO_E4_VERDICT: &str =
    "E4_GENERATOR_MEMBERSHIP_AND_INDEPENDENCE_ARE_OUTSIDE_THIS_MODULE";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct BoundaryProvenanceRef {
    pub derivation: String,
    pub source_step: u32,
    pub carrier: TypeExpr,
    pub dimension: u32,
}

impl From<&RegisteredBoundaryBundleRef> for BoundaryProvenanceRef {
    fn from(bundle: &RegisteredBoundaryBundleRef) -> Self {
        Self {
            derivation: bundle.derivation.as_str().to_owned(),
            source_step: bundle.source_step,
            carrier: bundle.carrier.clone(),
            dimension: bundle.dimension,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize)]
pub struct SchemaExpressionProvenance {
    pub ambient_support: SchemaSupport,
    pub local_binders: Vec<SchemaBinderId>,
    pub schema_dimensions: Vec<SchemaDimBinderId>,
    pub registered_boundary_bundles: Vec<BoundaryProvenanceRef>,
}

impl SchemaExpressionProvenance {
    fn is_subset_of(&self, other: &Self) -> bool {
        self.ambient_support.is_subset_of(&other.ambient_support)
            && self
                .local_binders
                .iter()
                .all(|binder| other.local_binders.contains(binder))
            && self
                .schema_dimensions
                .iter()
                .all(|dimension| other.schema_dimensions.contains(dimension))
            && self
                .registered_boundary_bundles
                .iter()
                .all(|bundle| other.registered_boundary_bundles.contains(bundle))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NormalizationProvenance {
    pub source: SchemaExpressionProvenance,
    pub normal_form: SchemaExpressionProvenance,
    pub normal_form_provenance_is_source_derived: bool,
    pub opaque_library_delta_unfolding_used: bool,
    pub boundary_bundle_rewrite_used: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SchemaTermNormalizationToken {
    context: FormedSchemaContext,
    window: SupportWindow,
    locals: Vec<SchemaLocalDeclaration>,
    dimensions: Vec<SchemaDimBinderId>,
    source_term: SchemaTermExpr,
    source_type: SchemaTypeExpr,
    source_type_normal_form: SchemaTypeExpr,
    normal_form: SchemaTermExpr,
    normal_form_type: SchemaTypeExpr,
    normal_form_type_normal_form: SchemaTypeExpr,
    beta_budget: u32,
    beta_steps: u32,
    normalization_is_stable: bool,
    type_preserved: bool,
    provenance: NormalizationProvenance,
    termination_by_construction: bool,
    total_over_all_well_typed_schema2_terms: bool,
    totality_gap: String,
    general_c6_complete: bool,
    general_c6_gap: String,
    e4_verdict_issued: bool,
    e4_boundary: String,
    derivation_hash: String,
}

impl SchemaTermNormalizationToken {
    pub fn context(&self) -> &FormedSchemaContext {
        &self.context
    }

    pub const fn window(&self) -> SupportWindow {
        self.window
    }

    pub fn locals(&self) -> &[SchemaLocalDeclaration] {
        &self.locals
    }

    pub fn dimensions(&self) -> &[SchemaDimBinderId] {
        &self.dimensions
    }

    pub fn source_term(&self) -> &SchemaTermExpr {
        &self.source_term
    }

    pub fn source_type(&self) -> &SchemaTypeExpr {
        &self.source_type
    }

    pub fn normal_form(&self) -> &SchemaTermExpr {
        &self.normal_form
    }

    pub fn normal_form_type(&self) -> &SchemaTypeExpr {
        &self.normal_form_type
    }

    pub fn normal_form_type_normal_form(&self) -> &SchemaTypeExpr {
        &self.normal_form_type_normal_form
    }

    pub const fn beta_budget(&self) -> u32 {
        self.beta_budget
    }

    pub const fn beta_steps(&self) -> u32 {
        self.beta_steps
    }

    pub const fn normalization_is_stable(&self) -> bool {
        self.normalization_is_stable
    }

    pub const fn type_preserved(&self) -> bool {
        self.type_preserved
    }

    pub fn provenance(&self) -> &NormalizationProvenance {
        &self.provenance
    }

    pub const fn termination_by_construction(&self) -> bool {
        self.termination_by_construction
    }

    pub const fn total_over_all_well_typed_schema2_terms(&self) -> bool {
        self.total_over_all_well_typed_schema2_terms
    }

    pub const fn general_c6_complete(&self) -> bool {
        self.general_c6_complete
    }

    pub const fn e4_verdict_issued(&self) -> bool {
        self.e4_verdict_issued
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FrozenSchemaEqualityToken {
    procedure: String,
    kernel_procedure: String,
    left: SchemaTermNormalizationToken,
    right: SchemaTermNormalizationToken,
    common_type_normal_form: SchemaTypeExpr,
    equal: bool,
    source_provenance_retained: bool,
    surviving_provenance_equal: bool,
    family_reclassification_decided: bool,
    relation_scope: String,
    arbitrary_context_presentation_quotient_proved: bool,
    e4_verdict_issued: bool,
    derivation_hash: String,
}

impl FrozenSchemaEqualityToken {
    pub fn procedure(&self) -> &str {
        &self.procedure
    }

    pub fn kernel_procedure(&self) -> &str {
        &self.kernel_procedure
    }

    pub fn left(&self) -> &SchemaTermNormalizationToken {
        &self.left
    }

    pub fn right(&self) -> &SchemaTermNormalizationToken {
        &self.right
    }

    pub fn common_type_normal_form(&self) -> &SchemaTypeExpr {
        &self.common_type_normal_form
    }

    pub const fn equal(&self) -> bool {
        self.equal
    }

    pub const fn source_provenance_retained(&self) -> bool {
        self.source_provenance_retained
    }

    pub const fn surviving_provenance_equal(&self) -> bool {
        self.surviving_provenance_equal
    }

    pub const fn family_reclassification_decided(&self) -> bool {
        self.family_reclassification_decided
    }

    pub fn relation_scope(&self) -> &str {
        &self.relation_scope
    }

    pub const fn arbitrary_context_presentation_quotient_proved(&self) -> bool {
        self.arbitrary_context_presentation_quotient_proved
    }

    pub const fn e4_verdict_issued(&self) -> bool {
        self.e4_verdict_issued
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FrozenEqualityLaw {
    Reflexivity,
    Symmetry,
    Transitivity,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FrozenEqualityLawToken {
    law: FrozenEqualityLaw,
    premises: Vec<FrozenSchemaEqualityToken>,
    conclusion: FrozenSchemaEqualityToken,
    holds: bool,
    derivation_hash: String,
}

impl FrozenEqualityLawToken {
    pub const fn law(&self) -> FrozenEqualityLaw {
        self.law
    }

    pub fn premises(&self) -> &[FrozenSchemaEqualityToken] {
        &self.premises
    }

    pub fn conclusion(&self) -> &FrozenSchemaEqualityToken {
        &self.conclusion
    }

    pub const fn holds(&self) -> bool {
        self.holds
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct NormalizationCongruenceToken {
    normalization_derivation_hash: String,
    source_equals_normal_form: FrozenSchemaEqualityToken,
    holds: bool,
    derivation_hash: String,
}

impl NormalizationCongruenceToken {
    pub fn source_equals_normal_form(&self) -> &FrozenSchemaEqualityToken {
        &self.source_equals_normal_form
    }

    pub const fn holds(&self) -> bool {
        self.holds
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WeakeningNormalizationCongruenceToken {
    source: SchemaTermNormalizationToken,
    weakening: TypedSubstitutionToken,
    target: SchemaTermNormalizationToken,
    same_normal_form: bool,
    same_normal_type: bool,
    provenance_not_reclassified: bool,
    holds: bool,
    derivation_hash: String,
}

impl WeakeningNormalizationCongruenceToken {
    pub fn source(&self) -> &SchemaTermNormalizationToken {
        &self.source
    }

    pub fn weakening(&self) -> &TypedSubstitutionToken {
        &self.weakening
    }

    pub fn target(&self) -> &SchemaTermNormalizationToken {
        &self.target
    }

    pub const fn same_normal_form(&self) -> bool {
        self.same_normal_form
    }

    pub const fn same_normal_type(&self) -> bool {
        self.same_normal_type
    }

    pub const fn provenance_not_reclassified(&self) -> bool {
        self.provenance_not_reclassified
    }

    pub const fn holds(&self) -> bool {
        self.holds
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SubstitutionNormalizationCongruenceToken {
    source: SchemaTermNormalizationToken,
    substitution: TypedSubstitutionToken,
    normalize_after_substitution: SchemaTermNormalizationToken,
    normalize_substituted_normal_form: SchemaTermNormalizationToken,
    same_normal_form: bool,
    same_normal_type: bool,
    cubical_bundle_transport_used: bool,
    holds: bool,
    derivation_hash: String,
}

impl SubstitutionNormalizationCongruenceToken {
    pub fn source(&self) -> &SchemaTermNormalizationToken {
        &self.source
    }

    pub fn substitution(&self) -> &TypedSubstitutionToken {
        &self.substitution
    }

    pub fn normalize_after_substitution(&self) -> &SchemaTermNormalizationToken {
        &self.normalize_after_substitution
    }

    pub fn normalize_substituted_normal_form(&self) -> &SchemaTermNormalizationToken {
        &self.normalize_substituted_normal_form
    }

    pub const fn same_normal_form(&self) -> bool {
        self.same_normal_form
    }

    pub const fn same_normal_type(&self) -> bool {
        self.same_normal_type
    }

    pub const fn cubical_bundle_transport_used(&self) -> bool {
        self.cubical_bundle_transport_used
    }

    pub const fn holds(&self) -> bool {
        self.holds
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrdinaryOriginProvenance {
    pub formation_derivation_hash: String,
    pub anchors: Vec<ClauseAnchor>,
    pub source_telescopes: Vec<SupportSourceTelescope>,
    pub semantic_support: SchemaSupport,
    pub derivation_references: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FrozenOrdinaryNormalForm {
    pub context_derivation_hash: String,
    pub support_window: SupportWindow,
    pub constructor: OrdinarySchemaKind,
    pub interpretation: OrdinaryInterpretation,
    /// A semantic equality key, deliberately not a family/count token.
    pub semantic_equality_key: String,
    pub origin: OrdinaryOriginProvenance,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrdinarySchemaNormalizationToken {
    source: OrdinarySchema,
    normal_form: FrozenOrdinaryNormalForm,
    typed_before_and_after: bool,
    replay_stable: bool,
    provenance_retained: bool,
    reference_or_new_family_decided: bool,
    e4_verdict_issued: bool,
    derivation_hash: String,
}

impl OrdinarySchemaNormalizationToken {
    pub fn source(&self) -> &OrdinarySchema {
        &self.source
    }

    pub fn normal_form(&self) -> &FrozenOrdinaryNormalForm {
        &self.normal_form
    }

    pub const fn typed_before_and_after(&self) -> bool {
        self.typed_before_and_after
    }

    pub const fn replay_stable(&self) -> bool {
        self.replay_stable
    }

    pub const fn provenance_retained(&self) -> bool {
        self.provenance_retained
    }

    pub const fn reference_or_new_family_decided(&self) -> bool {
        self.reference_or_new_family_decided
    }

    pub const fn e4_verdict_issued(&self) -> bool {
        self.e4_verdict_issued
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct FrozenOrdinaryEqualityToken {
    left: OrdinarySchemaNormalizationToken,
    right: OrdinarySchemaNormalizationToken,
    equal: bool,
    origin_provenance_was_not_quotiented: bool,
    family_membership_decided: bool,
    derivation_hash: String,
}

/// Deterministic aggregate replay surface for the honest E-3 fragment.
///
/// This token binds representative positive and negative equality decisions,
/// beta normalization, weakening, a genuine expression-image substitution,
/// and provenance-separated ordinary equality.  The false fields are part of
/// the hash and prevent this scoped audit from being replayed as E-3/E-4
/// completeness.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct E3FragmentAuditToken {
    version: String,
    equality_procedure: String,
    kernel_equality_procedure: String,
    schema_term_constructor_coverage: Vec<String>,
    schema_type_constructor_coverage: Vec<String>,
    ordinary_constructor_coverage: Vec<OrdinarySchemaKind>,
    beta_normalization: SchemaTermNormalizationToken,
    positive_equality: FrozenSchemaEqualityToken,
    negative_equality: FrozenSchemaEqualityToken,
    normalization_congruence: NormalizationCongruenceToken,
    weakening_congruence: WeakeningNormalizationCongruenceToken,
    genuine_substitution_congruence: SubstitutionNormalizationCongruenceToken,
    ordinary_anchor_equality: FrozenOrdinaryEqualityToken,
    termination_by_construction: bool,
    output_type_rechecked_and_preserved: bool,
    replay_stable: bool,
    provenance_source_and_normal_form_retained: bool,
    total_normalization_proved: bool,
    general_univalent_equality_proved: bool,
    arbitrary_context_presentation_quotient_proved: bool,
    cubical_bundle_substitution_transport_proved: bool,
    general_c6_complete: bool,
    e4_generator_completeness_proved: bool,
    independent_verdict_issued: bool,
    ordinary_family_token_issued: bool,
    stage_count_computed: bool,
    fq2_evaluated: bool,
    handoff_issued: bool,
    gaps: Vec<String>,
    derivation_hash: String,
}

impl E3FragmentAuditToken {
    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn equality_procedure(&self) -> &str {
        &self.equality_procedure
    }

    pub fn schema_term_constructor_coverage(&self) -> &[String] {
        &self.schema_term_constructor_coverage
    }

    pub fn schema_type_constructor_coverage(&self) -> &[String] {
        &self.schema_type_constructor_coverage
    }

    pub fn ordinary_constructor_coverage(&self) -> &[OrdinarySchemaKind] {
        &self.ordinary_constructor_coverage
    }

    pub fn beta_normalization(&self) -> &SchemaTermNormalizationToken {
        &self.beta_normalization
    }

    pub fn positive_equality(&self) -> &FrozenSchemaEqualityToken {
        &self.positive_equality
    }

    pub fn negative_equality(&self) -> &FrozenSchemaEqualityToken {
        &self.negative_equality
    }

    pub fn weakening_congruence(&self) -> &WeakeningNormalizationCongruenceToken {
        &self.weakening_congruence
    }

    pub fn genuine_substitution_congruence(&self) -> &SubstitutionNormalizationCongruenceToken {
        &self.genuine_substitution_congruence
    }

    pub fn ordinary_anchor_equality(&self) -> &FrozenOrdinaryEqualityToken {
        &self.ordinary_anchor_equality
    }

    pub const fn termination_by_construction(&self) -> bool {
        self.termination_by_construction
    }

    pub const fn output_type_rechecked_and_preserved(&self) -> bool {
        self.output_type_rechecked_and_preserved
    }

    pub const fn replay_stable(&self) -> bool {
        self.replay_stable
    }

    pub const fn provenance_source_and_normal_form_retained(&self) -> bool {
        self.provenance_source_and_normal_form_retained
    }

    pub const fn total_normalization_proved(&self) -> bool {
        self.total_normalization_proved
    }

    pub const fn general_univalent_equality_proved(&self) -> bool {
        self.general_univalent_equality_proved
    }

    pub const fn arbitrary_context_presentation_quotient_proved(&self) -> bool {
        self.arbitrary_context_presentation_quotient_proved
    }

    pub const fn cubical_bundle_substitution_transport_proved(&self) -> bool {
        self.cubical_bundle_substitution_transport_proved
    }

    pub const fn general_c6_complete(&self) -> bool {
        self.general_c6_complete
    }

    pub const fn e4_generator_completeness_proved(&self) -> bool {
        self.e4_generator_completeness_proved
    }

    pub const fn independent_verdict_issued(&self) -> bool {
        self.independent_verdict_issued
    }

    pub const fn ordinary_family_token_issued(&self) -> bool {
        self.ordinary_family_token_issued
    }

    pub const fn stage_count_computed(&self) -> bool {
        self.stage_count_computed
    }

    pub const fn fq2_evaluated(&self) -> bool {
        self.fq2_evaluated
    }

    pub const fn handoff_issued(&self) -> bool {
        self.handoff_issued
    }

    pub fn gaps(&self) -> &[String] {
        &self.gaps
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

impl FrozenOrdinaryEqualityToken {
    pub fn left(&self) -> &OrdinarySchemaNormalizationToken {
        &self.left
    }

    pub fn right(&self) -> &OrdinarySchemaNormalizationToken {
        &self.right
    }

    pub const fn equal(&self) -> bool {
        self.equal
    }

    pub const fn origin_provenance_was_not_quotiented(&self) -> bool {
        self.origin_provenance_was_not_quotiented
    }

    pub const fn family_membership_decided(&self) -> bool {
        self.family_membership_decided
    }

    pub fn derivation_hash(&self) -> &str {
        &self.derivation_hash
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum E3NormalizationError {
    #[error(transparent)]
    Grammar(#[from] OrdinaryGrammarError),
    #[error(transparent)]
    Context(#[from] SchemaContextError),
    #[error("deterministic beta budget {budget} exhausted after {steps} reductions")]
    DeterministicBudgetExhausted { budget: u32, steps: u32 },
    #[error("normalization changed the inferred type")]
    TypePreservationFailed,
    #[error("normalizing a normal form was not stable")]
    ReplayStabilityFailed,
    #[error("the two equality operands are not in the same typed schema context")]
    EqualityContextMismatch,
    #[error("the two equality operands have distinct frozen normal types")]
    EqualityTypeMismatch,
    #[error("the requested equality law requires positive equality premises")]
    EqualityLawPremiseNotEqual,
    #[error("the transitivity premises do not share an equal middle term")]
    EqualityTransitivityMiddleMismatch,
    #[error("normalization or equality token replay mismatch")]
    ReplayMismatch,
    #[error("normalization congruence did not hold")]
    CongruenceFailed,
    #[error("substitution source does not match the normalization context")]
    SubstitutionSourceMismatch,
    #[error("{gap}")]
    CubicalSubstitutionProvenanceGap { gap: String },
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(
        SCHEMA2_E3_FROZEN_FRAGMENT_VERSION,
        SCHEMA2_FROZEN_EQUALITY_PROCEDURE,
        KERNEL_EQUALITY_PROCEDURE,
        domain,
        payload,
    ))
    .expect("E3 proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn term_nodes(term: &SchemaTermExpr) -> u32 {
    1u32.saturating_add(match term {
        SchemaTermExpr::Ambient { .. } | SchemaTermExpr::Variable { .. } => 0,
        SchemaTermExpr::Pair { left, right }
        | SchemaTermExpr::App {
            function: left,
            argument: right,
        } => term_nodes(left).saturating_add(term_nodes(right)),
        SchemaTermExpr::Lam { domain, body, .. } => {
            type_nodes(domain).saturating_add(term_nodes(body))
        }
        SchemaTermExpr::CubeAt { cube, coordinates } => {
            term_nodes(cube).saturating_add(coordinates.len() as u32)
        }
        SchemaTermExpr::MapCube { function, cube } => {
            term_nodes(function).saturating_add(term_nodes(cube))
        }
    })
}

fn type_nodes(ty: &SchemaTypeExpr) -> u32 {
    1u32.saturating_add(match ty {
        SchemaTypeExpr::Element { .. } => 0,
        SchemaTypeExpr::Product { left, right } => {
            type_nodes(left).saturating_add(type_nodes(right))
        }
        SchemaTypeExpr::Pi {
            domain, codomain, ..
        } => type_nodes(domain).saturating_add(type_nodes(codomain)),
        SchemaTypeExpr::Path { left, right, .. } => {
            term_nodes(left).saturating_add(term_nodes(right))
        }
        SchemaTypeExpr::Cube { boundary, .. } => term_nodes(boundary),
    })
}

fn deterministic_budget(term: &SchemaTermExpr, ty: &SchemaTypeExpr) -> u32 {
    let size = term_nodes(term).saturating_add(type_nodes(ty)).max(1);
    size.saturating_mul(size)
        .saturating_mul(4)
        .clamp(64, 1_000_000)
}

#[derive(Clone, Copy)]
struct BetaState {
    budget: u32,
    steps: u32,
}

impl BetaState {
    fn spend(&mut self) -> Result<(), E3NormalizationError> {
        if self.steps >= self.budget {
            return Err(E3NormalizationError::DeterministicBudgetExhausted {
                budget: self.budget,
                steps: self.steps,
            });
        }
        self.steps = self.steps.saturating_add(1);
        Ok(())
    }
}

fn substitute_term_local(
    term: &SchemaTermExpr,
    binder: SchemaBinderId,
    image: &SchemaTermExpr,
) -> SchemaTermExpr {
    match term {
        SchemaTermExpr::Ambient { .. } => term.clone(),
        SchemaTermExpr::Variable { binder: found } if *found == binder => image.clone(),
        SchemaTermExpr::Variable { .. } => term.clone(),
        SchemaTermExpr::Pair { left, right } => SchemaTermExpr::Pair {
            left: Box::new(substitute_term_local(left, binder, image)),
            right: Box::new(substitute_term_local(right, binder, image)),
        },
        SchemaTermExpr::Lam {
            binder: bound,
            domain,
            body,
        } => SchemaTermExpr::Lam {
            binder: *bound,
            domain: Box::new(substitute_type_local(domain, binder, image)),
            body: if *bound == binder {
                body.clone()
            } else {
                Box::new(substitute_term_local(body, binder, image))
            },
        },
        SchemaTermExpr::App { function, argument } => SchemaTermExpr::App {
            function: Box::new(substitute_term_local(function, binder, image)),
            argument: Box::new(substitute_term_local(argument, binder, image)),
        },
        SchemaTermExpr::CubeAt { cube, coordinates } => SchemaTermExpr::CubeAt {
            cube: Box::new(substitute_term_local(cube, binder, image)),
            coordinates: coordinates.clone(),
        },
        SchemaTermExpr::MapCube { function, cube } => SchemaTermExpr::MapCube {
            function: Box::new(substitute_term_local(function, binder, image)),
            cube: Box::new(substitute_term_local(cube, binder, image)),
        },
    }
}

fn substitute_type_local(
    ty: &SchemaTypeExpr,
    binder: SchemaBinderId,
    image: &SchemaTermExpr,
) -> SchemaTypeExpr {
    match ty {
        SchemaTypeExpr::Element { .. } => ty.clone(),
        SchemaTypeExpr::Product { left, right } => SchemaTypeExpr::Product {
            left: Box::new(substitute_type_local(left, binder, image)),
            right: Box::new(substitute_type_local(right, binder, image)),
        },
        SchemaTypeExpr::Pi {
            binder: bound,
            domain,
            codomain,
        } => SchemaTypeExpr::Pi {
            binder: *bound,
            domain: Box::new(substitute_type_local(domain, binder, image)),
            codomain: if *bound == binder {
                codomain.clone()
            } else {
                Box::new(substitute_type_local(codomain, binder, image))
            },
        },
        SchemaTypeExpr::Path {
            carrier,
            left,
            right,
        } => SchemaTypeExpr::Path {
            carrier: carrier.clone(),
            left: Box::new(substitute_term_local(left, binder, image)),
            right: Box::new(substitute_term_local(right, binder, image)),
        },
        SchemaTypeExpr::Cube {
            carrier,
            boundary,
            dimension,
            source_bundle,
        } => SchemaTypeExpr::Cube {
            carrier: carrier.clone(),
            boundary: Box::new(substitute_term_local(boundary, binder, image)),
            dimension: *dimension,
            source_bundle: source_bundle.clone(),
        },
    }
}

fn normalize_type(
    ty: &SchemaTypeExpr,
    state: &mut BetaState,
) -> Result<SchemaTypeExpr, E3NormalizationError> {
    Ok(match ty {
        SchemaTypeExpr::Element { .. } => ty.clone(),
        SchemaTypeExpr::Product { left, right } => SchemaTypeExpr::Product {
            left: Box::new(normalize_type(left, state)?),
            right: Box::new(normalize_type(right, state)?),
        },
        SchemaTypeExpr::Pi {
            binder,
            domain,
            codomain,
        } => SchemaTypeExpr::Pi {
            binder: *binder,
            domain: Box::new(normalize_type(domain, state)?),
            codomain: Box::new(normalize_type(codomain, state)?),
        },
        SchemaTypeExpr::Path {
            carrier,
            left,
            right,
        } => SchemaTypeExpr::Path {
            carrier: carrier.clone(),
            left: Box::new(normalize_term(left, state)?),
            right: Box::new(normalize_term(right, state)?),
        },
        SchemaTypeExpr::Cube {
            carrier,
            boundary,
            dimension,
            source_bundle,
        } => SchemaTypeExpr::Cube {
            carrier: carrier.clone(),
            boundary: Box::new(normalize_term(boundary, state)?),
            dimension: *dimension,
            source_bundle: source_bundle.clone(),
        },
    })
}

fn normalize_term(
    term: &SchemaTermExpr,
    state: &mut BetaState,
) -> Result<SchemaTermExpr, E3NormalizationError> {
    Ok(match term {
        SchemaTermExpr::Ambient { .. } | SchemaTermExpr::Variable { .. } => term.clone(),
        SchemaTermExpr::Pair { left, right } => SchemaTermExpr::Pair {
            left: Box::new(normalize_term(left, state)?),
            right: Box::new(normalize_term(right, state)?),
        },
        SchemaTermExpr::Lam {
            binder,
            domain,
            body,
        } => SchemaTermExpr::Lam {
            binder: *binder,
            domain: Box::new(normalize_type(domain, state)?),
            body: Box::new(normalize_term(body, state)?),
        },
        SchemaTermExpr::App { function, argument } => {
            let function = normalize_term(function, state)?;
            let argument = normalize_term(argument, state)?;
            if let SchemaTermExpr::Lam { binder, body, .. } = function {
                state.spend()?;
                normalize_term(&substitute_term_local(&body, binder, &argument), state)?
            } else {
                SchemaTermExpr::App {
                    function: Box::new(function),
                    argument: Box::new(argument),
                }
            }
        }
        SchemaTermExpr::CubeAt { cube, coordinates } => SchemaTermExpr::CubeAt {
            cube: Box::new(normalize_term(cube, state)?),
            coordinates: coordinates.clone(),
        },
        SchemaTermExpr::MapCube { function, cube } => SchemaTermExpr::MapCube {
            function: Box::new(normalize_term(function, state)?),
            cube: Box::new(normalize_term(cube, state)?),
        },
    })
}

fn normalize_type_with_budget(
    ty: &SchemaTypeExpr,
    budget: u32,
) -> Result<(SchemaTypeExpr, u32), E3NormalizationError> {
    let mut state = BetaState { budget, steps: 0 };
    let normal = normalize_type(ty, &mut state)?;
    Ok((normal, state.steps))
}

fn insert_ambient_support(
    expression: TypedExpression,
    binders: &mut BTreeSet<BinderId>,
    libraries: &mut BTreeSet<LibraryKey>,
) {
    let support = expression_support(&expression);
    binders.extend(support.binders);
    libraries.extend(support.libraries);
}

fn collect_type_provenance(
    ty: &SchemaTypeExpr,
    ambient_binders: &mut BTreeSet<BinderId>,
    libraries: &mut BTreeSet<LibraryKey>,
    locals: &mut BTreeSet<SchemaBinderId>,
    dimensions: &mut BTreeSet<SchemaDimBinderId>,
    bundles: &mut BTreeSet<BoundaryProvenanceRef>,
) {
    match ty {
        SchemaTypeExpr::Element { carrier } => insert_ambient_support(
            TypedExpression::Type {
                expression: carrier.clone(),
            },
            ambient_binders,
            libraries,
        ),
        SchemaTypeExpr::Product { left, right } => {
            collect_type_provenance(
                left,
                ambient_binders,
                libraries,
                locals,
                dimensions,
                bundles,
            );
            collect_type_provenance(
                right,
                ambient_binders,
                libraries,
                locals,
                dimensions,
                bundles,
            );
        }
        SchemaTypeExpr::Pi {
            binder,
            domain,
            codomain,
        } => {
            collect_type_provenance(
                domain,
                ambient_binders,
                libraries,
                locals,
                dimensions,
                bundles,
            );
            let mut codomain_locals = BTreeSet::new();
            collect_type_provenance(
                codomain,
                ambient_binders,
                libraries,
                &mut codomain_locals,
                dimensions,
                bundles,
            );
            codomain_locals.remove(binder);
            locals.extend(codomain_locals);
        }
        SchemaTypeExpr::Path {
            carrier,
            left,
            right,
        } => {
            insert_ambient_support(
                TypedExpression::Type {
                    expression: carrier.clone(),
                },
                ambient_binders,
                libraries,
            );
            collect_term_provenance(
                left,
                ambient_binders,
                libraries,
                locals,
                dimensions,
                bundles,
            );
            collect_term_provenance(
                right,
                ambient_binders,
                libraries,
                locals,
                dimensions,
                bundles,
            );
        }
        SchemaTypeExpr::Cube {
            carrier,
            boundary,
            source_bundle,
            ..
        } => {
            insert_ambient_support(
                TypedExpression::Type {
                    expression: carrier.clone(),
                },
                ambient_binders,
                libraries,
            );
            collect_term_provenance(
                boundary,
                ambient_binders,
                libraries,
                locals,
                dimensions,
                bundles,
            );
            bundles.insert(BoundaryProvenanceRef::from(source_bundle));
        }
    }
}

fn collect_term_provenance(
    term: &SchemaTermExpr,
    ambient_binders: &mut BTreeSet<BinderId>,
    libraries: &mut BTreeSet<LibraryKey>,
    locals: &mut BTreeSet<SchemaBinderId>,
    dimensions: &mut BTreeSet<SchemaDimBinderId>,
    bundles: &mut BTreeSet<BoundaryProvenanceRef>,
) {
    match term {
        SchemaTermExpr::Ambient { term } => {
            // The type is immaterial for support collection; inferencing has
            // already checked it, while this traversal records the term's
            // actual binders and rigid libraries.
            collect_ambient_term_support(term, ambient_binders, libraries);
        }
        SchemaTermExpr::Variable { binder } => {
            locals.insert(*binder);
        }
        SchemaTermExpr::Pair { left, right }
        | SchemaTermExpr::App {
            function: left,
            argument: right,
        }
        | SchemaTermExpr::MapCube {
            function: left,
            cube: right,
        } => {
            collect_term_provenance(
                left,
                ambient_binders,
                libraries,
                locals,
                dimensions,
                bundles,
            );
            collect_term_provenance(
                right,
                ambient_binders,
                libraries,
                locals,
                dimensions,
                bundles,
            );
        }
        SchemaTermExpr::Lam {
            binder,
            domain,
            body,
        } => {
            collect_type_provenance(
                domain,
                ambient_binders,
                libraries,
                locals,
                dimensions,
                bundles,
            );
            let mut body_locals = BTreeSet::new();
            collect_term_provenance(
                body,
                ambient_binders,
                libraries,
                &mut body_locals,
                dimensions,
                bundles,
            );
            body_locals.remove(binder);
            locals.extend(body_locals);
        }
        SchemaTermExpr::CubeAt { cube, coordinates } => {
            collect_term_provenance(
                cube,
                ambient_binders,
                libraries,
                locals,
                dimensions,
                bundles,
            );
            for coordinate in coordinates {
                if let SchemaDimExpr::Variable { binder } = coordinate {
                    dimensions.insert(*binder);
                }
            }
        }
    }
}

fn collect_ambient_type_support(
    ty: &TypeExpr,
    binders: &mut BTreeSet<BinderId>,
    libraries: &mut BTreeSet<LibraryKey>,
) {
    insert_ambient_support(
        TypedExpression::Type {
            expression: ty.clone(),
        },
        binders,
        libraries,
    );
}

fn collect_ambient_term_support(
    term: &TermExpr,
    binders: &mut BTreeSet<BinderId>,
    libraries: &mut BTreeSet<LibraryKey>,
) {
    match term {
        TermExpr::Variable { binder } => {
            binders.insert(*binder);
        }
        TermExpr::Library { step, symbol } => {
            libraries.insert(LibraryKey {
                step: *step,
                symbol: symbol.clone(),
            });
        }
        TermExpr::TruncPoint { carrier, point } | TermExpr::Reflexivity { carrier, point } => {
            collect_ambient_type_support(carrier, binders, libraries);
            collect_ambient_term_support(point, binders, libraries);
        }
    }
}

fn expression_provenance(term: &SchemaTermExpr, ty: &SchemaTypeExpr) -> SchemaExpressionProvenance {
    let mut ambient_binders = BTreeSet::new();
    let mut libraries = BTreeSet::new();
    let mut locals = BTreeSet::new();
    let mut dimensions = BTreeSet::new();
    let mut bundles = BTreeSet::new();
    collect_term_provenance(
        term,
        &mut ambient_binders,
        &mut libraries,
        &mut locals,
        &mut dimensions,
        &mut bundles,
    );
    collect_type_provenance(
        ty,
        &mut ambient_binders,
        &mut libraries,
        &mut locals,
        &mut dimensions,
        &mut bundles,
    );
    SchemaExpressionProvenance {
        ambient_support: SchemaSupport {
            binders: ambient_binders.into_iter().collect(),
            libraries: libraries.into_iter().collect(),
        },
        local_binders: locals.into_iter().collect(),
        schema_dimensions: dimensions.into_iter().collect(),
        registered_boundary_bundles: bundles.into_iter().collect(),
    }
}

fn check_schema_environment(
    context: &FormedSchemaContext,
    window: SupportWindow,
    locals: &[SchemaLocalDeclaration],
    dimensions: &[SchemaDimBinderId],
) -> Result<(), E3NormalizationError> {
    let mut prefix = Vec::with_capacity(locals.len());
    for local in locals {
        check_schema_type(context, window, &prefix, dimensions, &local.ty)?;
        if prefix
            .iter()
            .any(|prior: &SchemaLocalDeclaration| prior.binder == local.binder)
        {
            return Err(OrdinaryGrammarError::DuplicateSchemaBinder {
                binder: local.binder,
            }
            .into());
        }
        prefix.push(local.clone());
    }
    if dimensions.iter().copied().collect::<BTreeSet<_>>().len() != dimensions.len() {
        return Err(OrdinaryGrammarError::DuplicateSchemaDimension.into());
    }
    Ok(())
}

pub fn issue_schema_term_normalization(
    context: FormedSchemaContext,
    window: SupportWindow,
    locals: Vec<SchemaLocalDeclaration>,
    dimensions: Vec<SchemaDimBinderId>,
    source_term: SchemaTermExpr,
) -> Result<SchemaTermNormalizationToken, E3NormalizationError> {
    replay_formed_context(&context)?;
    check_schema_environment(&context, window, &locals, &dimensions)?;
    let source_type = infer_schema_term(&context, window, &locals, &dimensions, &source_term)?;
    check_schema_type(&context, window, &locals, &dimensions, &source_type)?;
    let beta_budget = deterministic_budget(&source_term, &source_type);
    let mut state = BetaState {
        budget: beta_budget,
        steps: 0,
    };
    let normal_form = normalize_term(&source_term, &mut state)?;
    let beta_steps = state.steps;
    let normal_form_type = infer_schema_term(&context, window, &locals, &dimensions, &normal_form)?;
    check_schema_type(&context, window, &locals, &dimensions, &normal_form_type)?;
    let (source_type_normal_form, _) = normalize_type_with_budget(&source_type, beta_budget)?;
    let (normal_form_type_normal_form, _) =
        normalize_type_with_budget(&normal_form_type, beta_budget)?;
    let type_preserved = source_type_normal_form == normal_form_type_normal_form;
    if !type_preserved {
        return Err(E3NormalizationError::TypePreservationFailed);
    }
    let mut stability_state = BetaState {
        budget: beta_budget,
        steps: 0,
    };
    let replayed_normal = normalize_term(&normal_form, &mut stability_state)?;
    let normalization_is_stable = replayed_normal == normal_form && stability_state.steps == 0;
    if !normalization_is_stable {
        return Err(E3NormalizationError::ReplayStabilityFailed);
    }
    let source_provenance = expression_provenance(&source_term, &source_type);
    let normal_provenance = expression_provenance(&normal_form, &normal_form_type_normal_form);
    let normal_form_provenance_is_source_derived =
        normal_provenance.is_subset_of(&source_provenance);
    if !normal_form_provenance_is_source_derived {
        return Err(E3NormalizationError::ReplayStabilityFailed);
    }
    let provenance = NormalizationProvenance {
        source: source_provenance,
        normal_form: normal_provenance,
        normal_form_provenance_is_source_derived,
        opaque_library_delta_unfolding_used: false,
        boundary_bundle_rewrite_used: false,
    };
    let termination_by_construction = true;
    let total_over_all_well_typed_schema2_terms = false;
    let totality_gap = E3_TOTAL_NORMALIZATION_GAP.to_owned();
    let general_c6_complete = false;
    let general_c6_gap = E3_GENERAL_C6_GAP.to_owned();
    let e4_verdict_issued = false;
    let e4_boundary = E3_NO_E4_VERDICT.to_owned();
    let derivation_hash = tagged_hash(
        "schema-term-normalization",
        &(
            (
                context.derivation_hash(),
                window,
                &locals,
                &dimensions,
                &source_term,
                &source_type,
                &source_type_normal_form,
                &normal_form,
                &normal_form_type,
                &normal_form_type_normal_form,
                beta_budget,
            ),
            (
                beta_steps,
                normalization_is_stable,
                type_preserved,
                &provenance,
                termination_by_construction,
                total_over_all_well_typed_schema2_terms,
                &totality_gap,
                general_c6_complete,
                &general_c6_gap,
                e4_verdict_issued,
                &e4_boundary,
            ),
        ),
    );
    Ok(SchemaTermNormalizationToken {
        context,
        window,
        locals,
        dimensions,
        source_term,
        source_type,
        source_type_normal_form,
        normal_form,
        normal_form_type,
        normal_form_type_normal_form,
        beta_budget,
        beta_steps,
        normalization_is_stable,
        type_preserved,
        provenance,
        termination_by_construction,
        total_over_all_well_typed_schema2_terms,
        totality_gap,
        general_c6_complete,
        general_c6_gap,
        e4_verdict_issued,
        e4_boundary,
        derivation_hash,
    })
}

pub fn replay_schema_term_normalization(
    token: &SchemaTermNormalizationToken,
) -> Result<(), E3NormalizationError> {
    let replay = issue_schema_term_normalization(
        token.context.clone(),
        token.window,
        token.locals.clone(),
        token.dimensions.clone(),
        token.source_term.clone(),
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(E3NormalizationError::ReplayMismatch)
    }
}

fn same_equality_context(
    left: &SchemaTermNormalizationToken,
    right: &SchemaTermNormalizationToken,
) -> bool {
    left.context == right.context
        && left.window == right.window
        && left.locals == right.locals
        && left.dimensions == right.dimensions
}

pub fn issue_frozen_schema_equality(
    left: SchemaTermNormalizationToken,
    right: SchemaTermNormalizationToken,
) -> Result<FrozenSchemaEqualityToken, E3NormalizationError> {
    replay_schema_term_normalization(&left)?;
    replay_schema_term_normalization(&right)?;
    if !same_equality_context(&left, &right) {
        return Err(E3NormalizationError::EqualityContextMismatch);
    }
    if left.normal_form_type_normal_form != right.normal_form_type_normal_form {
        return Err(E3NormalizationError::EqualityTypeMismatch);
    }
    let surviving_provenance_equal = left.provenance.normal_form == right.provenance.normal_form;
    let equal = left.normal_form == right.normal_form && surviving_provenance_equal;
    let common_type_normal_form = left.normal_form_type_normal_form.clone();
    let procedure = SCHEMA2_FROZEN_EQUALITY_PROCEDURE.to_owned();
    let kernel_procedure = KERNEL_EQUALITY_PROCEDURE.to_owned();
    let source_provenance_retained = true;
    let family_reclassification_decided = false;
    let relation_scope = E3_SUCCESSFUL_NORMALIZATION_SCOPE.to_owned();
    let arbitrary_context_presentation_quotient_proved = false;
    let e4_verdict_issued = false;
    let derivation_hash = tagged_hash(
        "frozen-schema-equality",
        &(
            &procedure,
            &kernel_procedure,
            left.derivation_hash(),
            right.derivation_hash(),
            &common_type_normal_form,
            equal,
            &left.provenance.source,
            &right.provenance.source,
            &left.provenance.normal_form,
            &right.provenance.normal_form,
            source_provenance_retained,
            surviving_provenance_equal,
            family_reclassification_decided,
            &relation_scope,
            arbitrary_context_presentation_quotient_proved,
            e4_verdict_issued,
        ),
    );
    Ok(FrozenSchemaEqualityToken {
        procedure,
        kernel_procedure,
        left,
        right,
        common_type_normal_form,
        equal,
        source_provenance_retained,
        surviving_provenance_equal,
        family_reclassification_decided,
        relation_scope,
        arbitrary_context_presentation_quotient_proved,
        e4_verdict_issued,
        derivation_hash,
    })
}

pub fn replay_frozen_schema_equality(
    token: &FrozenSchemaEqualityToken,
) -> Result<(), E3NormalizationError> {
    let replay = issue_frozen_schema_equality(token.left.clone(), token.right.clone())?;
    if replay == *token {
        Ok(())
    } else {
        Err(E3NormalizationError::ReplayMismatch)
    }
}

pub fn issue_equality_reflexivity(
    term: SchemaTermNormalizationToken,
) -> Result<FrozenEqualityLawToken, E3NormalizationError> {
    let conclusion = issue_frozen_schema_equality(term.clone(), term)?;
    let holds = conclusion.equal;
    let premises = Vec::new();
    let derivation_hash = tagged_hash(
        "frozen-equality-law",
        &(
            FrozenEqualityLaw::Reflexivity,
            &premises,
            &conclusion,
            holds,
        ),
    );
    Ok(FrozenEqualityLawToken {
        law: FrozenEqualityLaw::Reflexivity,
        premises,
        conclusion,
        holds,
        derivation_hash,
    })
}

pub fn issue_equality_symmetry(
    premise: FrozenSchemaEqualityToken,
) -> Result<FrozenEqualityLawToken, E3NormalizationError> {
    replay_frozen_schema_equality(&premise)?;
    if !premise.equal {
        return Err(E3NormalizationError::EqualityLawPremiseNotEqual);
    }
    let conclusion = issue_frozen_schema_equality(premise.right.clone(), premise.left.clone())?;
    let holds = conclusion.equal;
    let premises = vec![premise];
    let derivation_hash = tagged_hash(
        "frozen-equality-law",
        &(FrozenEqualityLaw::Symmetry, &premises, &conclusion, holds),
    );
    Ok(FrozenEqualityLawToken {
        law: FrozenEqualityLaw::Symmetry,
        premises,
        conclusion,
        holds,
        derivation_hash,
    })
}

pub fn issue_equality_transitivity(
    first: FrozenSchemaEqualityToken,
    second: FrozenSchemaEqualityToken,
) -> Result<FrozenEqualityLawToken, E3NormalizationError> {
    replay_frozen_schema_equality(&first)?;
    replay_frozen_schema_equality(&second)?;
    if !first.equal || !second.equal {
        return Err(E3NormalizationError::EqualityLawPremiseNotEqual);
    }
    if !same_equality_context(&first.right, &second.left)
        || first.right.normal_form != second.left.normal_form
        || first.right.normal_form_type_normal_form != second.left.normal_form_type_normal_form
        || first.right.provenance.normal_form != second.left.provenance.normal_form
    {
        return Err(E3NormalizationError::EqualityTransitivityMiddleMismatch);
    }
    let conclusion = issue_frozen_schema_equality(first.left.clone(), second.right.clone())?;
    let holds = conclusion.equal;
    let premises = vec![first, second];
    let derivation_hash = tagged_hash(
        "frozen-equality-law",
        &(
            FrozenEqualityLaw::Transitivity,
            &premises,
            &conclusion,
            holds,
        ),
    );
    Ok(FrozenEqualityLawToken {
        law: FrozenEqualityLaw::Transitivity,
        premises,
        conclusion,
        holds,
        derivation_hash,
    })
}

pub fn replay_frozen_equality_law(
    token: &FrozenEqualityLawToken,
) -> Result<(), E3NormalizationError> {
    let replay = match token.law {
        FrozenEqualityLaw::Reflexivity => {
            if !token.premises.is_empty() {
                return Err(E3NormalizationError::ReplayMismatch);
            }
            issue_equality_reflexivity(token.conclusion.left.clone())?
        }
        FrozenEqualityLaw::Symmetry => {
            if token.premises.len() != 1 {
                return Err(E3NormalizationError::ReplayMismatch);
            }
            issue_equality_symmetry(token.premises[0].clone())?
        }
        FrozenEqualityLaw::Transitivity => {
            if token.premises.len() != 2 {
                return Err(E3NormalizationError::ReplayMismatch);
            }
            issue_equality_transitivity(token.premises[0].clone(), token.premises[1].clone())?
        }
    };
    if replay == *token {
        Ok(())
    } else {
        Err(E3NormalizationError::ReplayMismatch)
    }
}

pub fn issue_normalization_congruence(
    normalization: SchemaTermNormalizationToken,
) -> Result<NormalizationCongruenceToken, E3NormalizationError> {
    replay_schema_term_normalization(&normalization)?;
    let normal_normalization = issue_schema_term_normalization(
        normalization.context.clone(),
        normalization.window,
        normalization.locals.clone(),
        normalization.dimensions.clone(),
        normalization.normal_form.clone(),
    )?;
    let source_equals_normal_form =
        issue_frozen_schema_equality(normalization.clone(), normal_normalization)?;
    let holds = source_equals_normal_form.equal;
    if !holds {
        return Err(E3NormalizationError::CongruenceFailed);
    }
    let derivation_hash = tagged_hash(
        "normalization-congruence",
        &(
            normalization.derivation_hash(),
            &source_equals_normal_form,
            holds,
        ),
    );
    Ok(NormalizationCongruenceToken {
        normalization_derivation_hash: normalization.derivation_hash,
        source_equals_normal_form,
        holds,
        derivation_hash,
    })
}

pub fn replay_normalization_congruence(
    token: &NormalizationCongruenceToken,
) -> Result<(), E3NormalizationError> {
    let replay = issue_normalization_congruence(token.source_equals_normal_form.left.clone())?;
    if replay == *token {
        Ok(())
    } else {
        Err(E3NormalizationError::ReplayMismatch)
    }
}

pub fn issue_weakening_normalization_congruence(
    source: SchemaTermNormalizationToken,
    target_context: FormedSchemaContext,
) -> Result<WeakeningNormalizationCongruenceToken, E3NormalizationError> {
    replay_schema_term_normalization(&source)?;
    let weakening = weakening_substitution(&source.context, &target_context)?;
    let target = issue_schema_term_normalization(
        target_context,
        source.window,
        source.locals.clone(),
        source.dimensions.clone(),
        source.source_term.clone(),
    )?;
    let same_normal_form = source.normal_form == target.normal_form;
    let same_normal_type =
        source.normal_form_type_normal_form == target.normal_form_type_normal_form;
    let provenance_not_reclassified = source.provenance == target.provenance;
    let holds = same_normal_form && same_normal_type && provenance_not_reclassified;
    if !holds {
        return Err(E3NormalizationError::CongruenceFailed);
    }
    let derivation_hash = tagged_hash(
        "weakening-normalization-congruence",
        &(
            source.derivation_hash(),
            weakening.derivation_hash(),
            target.derivation_hash(),
            same_normal_form,
            same_normal_type,
            provenance_not_reclassified,
            holds,
        ),
    );
    Ok(WeakeningNormalizationCongruenceToken {
        source,
        weakening,
        target,
        same_normal_form,
        same_normal_type,
        provenance_not_reclassified,
        holds,
        derivation_hash,
    })
}

pub fn replay_weakening_normalization_congruence(
    token: &WeakeningNormalizationCongruenceToken,
) -> Result<(), E3NormalizationError> {
    let replay = issue_weakening_normalization_congruence(
        token.source.clone(),
        token.weakening.target().clone(),
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(E3NormalizationError::ReplayMismatch)
    }
}

#[derive(Default)]
struct AmbientImageMap {
    types: BTreeMap<BinderId, TypeExpr>,
    terms: BTreeMap<BinderId, TermExpr>,
}

fn ambient_image_map(substitution: &TypedSubstitutionToken) -> AmbientImageMap {
    let mut map = AmbientImageMap::default();
    for image in substitution.images() {
        match image {
            SubstitutionImage::Type { source, image } => {
                map.types.insert(*source, image.clone());
            }
            SubstitutionImage::Term { source, image } => {
                map.terms.insert(*source, image.clone());
            }
            SubstitutionImage::RigidLibrary { .. }
            | SubstitutionImage::Dimension { .. }
            | SubstitutionImage::Cofibration { .. } => {}
        }
    }
    map
}

fn substitute_ambient_type(ty: &TypeExpr, map: &AmbientImageMap) -> TypeExpr {
    match ty {
        TypeExpr::Parameter { binder } => {
            map.types.get(binder).cloned().unwrap_or_else(|| ty.clone())
        }
        TypeExpr::Trunc { carrier } => TypeExpr::trunc(substitute_ambient_type(carrier, map)),
        TypeExpr::Path {
            carrier,
            left,
            right,
        } => TypeExpr::Path {
            carrier: Box::new(substitute_ambient_type(carrier, map)),
            left: Box::new(substitute_ambient_term(left, map)),
            right: Box::new(substitute_ambient_term(right, map)),
        },
    }
}

fn substitute_ambient_term(term: &TermExpr, map: &AmbientImageMap) -> TermExpr {
    match term {
        TermExpr::Variable { binder } => map
            .terms
            .get(binder)
            .cloned()
            .unwrap_or_else(|| term.clone()),
        TermExpr::Library { .. } => term.clone(),
        TermExpr::TruncPoint { carrier, point } => TermExpr::TruncPoint {
            carrier: Box::new(substitute_ambient_type(carrier, map)),
            point: Box::new(substitute_ambient_term(point, map)),
        },
        TermExpr::Reflexivity { carrier, point } => TermExpr::Reflexivity {
            carrier: Box::new(substitute_ambient_type(carrier, map)),
            point: Box::new(substitute_ambient_term(point, map)),
        },
    }
}

fn substitute_ambient_schema_type(ty: &SchemaTypeExpr, map: &AmbientImageMap) -> SchemaTypeExpr {
    match ty {
        SchemaTypeExpr::Element { carrier } => SchemaTypeExpr::Element {
            carrier: substitute_ambient_type(carrier, map),
        },
        SchemaTypeExpr::Product { left, right } => SchemaTypeExpr::Product {
            left: Box::new(substitute_ambient_schema_type(left, map)),
            right: Box::new(substitute_ambient_schema_type(right, map)),
        },
        SchemaTypeExpr::Pi {
            binder,
            domain,
            codomain,
        } => SchemaTypeExpr::Pi {
            binder: *binder,
            domain: Box::new(substitute_ambient_schema_type(domain, map)),
            codomain: Box::new(substitute_ambient_schema_type(codomain, map)),
        },
        SchemaTypeExpr::Path {
            carrier,
            left,
            right,
        } => SchemaTypeExpr::Path {
            carrier: substitute_ambient_type(carrier, map),
            left: Box::new(substitute_ambient_schema_term(left, map)),
            right: Box::new(substitute_ambient_schema_term(right, map)),
        },
        SchemaTypeExpr::Cube {
            carrier,
            boundary,
            dimension,
            source_bundle,
        } => SchemaTypeExpr::Cube {
            carrier: substitute_ambient_type(carrier, map),
            boundary: Box::new(substitute_ambient_schema_term(boundary, map)),
            dimension: *dimension,
            source_bundle: RegisteredBoundaryBundleRef {
                derivation: source_bundle.derivation.clone(),
                source_step: source_bundle.source_step,
                carrier: substitute_ambient_type(&source_bundle.carrier, map),
                dimension: source_bundle.dimension,
            },
        },
    }
}

fn substitute_ambient_schema_term(term: &SchemaTermExpr, map: &AmbientImageMap) -> SchemaTermExpr {
    match term {
        SchemaTermExpr::Ambient { term } => SchemaTermExpr::Ambient {
            term: substitute_ambient_term(term, map),
        },
        SchemaTermExpr::Variable { .. } => term.clone(),
        SchemaTermExpr::Pair { left, right } => SchemaTermExpr::Pair {
            left: Box::new(substitute_ambient_schema_term(left, map)),
            right: Box::new(substitute_ambient_schema_term(right, map)),
        },
        SchemaTermExpr::Lam {
            binder,
            domain,
            body,
        } => SchemaTermExpr::Lam {
            binder: *binder,
            domain: Box::new(substitute_ambient_schema_type(domain, map)),
            body: Box::new(substitute_ambient_schema_term(body, map)),
        },
        SchemaTermExpr::App { function, argument } => SchemaTermExpr::App {
            function: Box::new(substitute_ambient_schema_term(function, map)),
            argument: Box::new(substitute_ambient_schema_term(argument, map)),
        },
        SchemaTermExpr::CubeAt { cube, coordinates } => SchemaTermExpr::CubeAt {
            cube: Box::new(substitute_ambient_schema_term(cube, map)),
            coordinates: coordinates.clone(),
        },
        SchemaTermExpr::MapCube { function, cube } => SchemaTermExpr::MapCube {
            function: Box::new(substitute_ambient_schema_term(function, map)),
            cube: Box::new(substitute_ambient_schema_term(cube, map)),
        },
    }
}

fn type_contains_bundle(ty: &SchemaTypeExpr) -> bool {
    match ty {
        SchemaTypeExpr::Element { .. } => false,
        SchemaTypeExpr::Product { left, right } => {
            type_contains_bundle(left) || type_contains_bundle(right)
        }
        SchemaTypeExpr::Pi {
            domain, codomain, ..
        } => type_contains_bundle(domain) || type_contains_bundle(codomain),
        SchemaTypeExpr::Path { left, right, .. } => {
            term_contains_bundle(left) || term_contains_bundle(right)
        }
        SchemaTypeExpr::Cube { .. } => true,
    }
}

fn term_contains_bundle(term: &SchemaTermExpr) -> bool {
    match term {
        SchemaTermExpr::Ambient { .. } | SchemaTermExpr::Variable { .. } => false,
        SchemaTermExpr::Pair { left, right }
        | SchemaTermExpr::App {
            function: left,
            argument: right,
        } => term_contains_bundle(left) || term_contains_bundle(right),
        SchemaTermExpr::Lam { domain, body, .. } => {
            type_contains_bundle(domain) || term_contains_bundle(body)
        }
        SchemaTermExpr::CubeAt { .. } | SchemaTermExpr::MapCube { .. } => true,
    }
}

pub fn issue_substitution_normalization_congruence(
    source: SchemaTermNormalizationToken,
    substitution: TypedSubstitutionToken,
) -> Result<SubstitutionNormalizationCongruenceToken, E3NormalizationError> {
    replay_schema_term_normalization(&source)?;
    replay_typed_substitution(&substitution)?;
    if substitution.source() != &source.context {
        return Err(E3NormalizationError::SubstitutionSourceMismatch);
    }
    if term_contains_bundle(&source.source_term)
        || type_contains_bundle(&source.source_type)
        || source
            .locals
            .iter()
            .any(|local| type_contains_bundle(&local.ty))
    {
        return Err(E3NormalizationError::CubicalSubstitutionProvenanceGap {
            gap: E3_CUBICAL_SUBSTITUTION_PROVENANCE_GAP.to_owned(),
        });
    }
    let map = ambient_image_map(&substitution);
    let target_locals = source
        .locals
        .iter()
        .map(|local| SchemaLocalDeclaration {
            binder: local.binder,
            ty: substitute_ambient_schema_type(&local.ty, &map),
        })
        .collect::<Vec<_>>();
    let substituted_source = substitute_ambient_schema_term(&source.source_term, &map);
    let substituted_normal = substitute_ambient_schema_term(&source.normal_form, &map);
    let normalize_after_substitution = issue_schema_term_normalization(
        substitution.target().clone(),
        source.window,
        target_locals.clone(),
        source.dimensions.clone(),
        substituted_source,
    )?;
    let normalize_substituted_normal_form = issue_schema_term_normalization(
        substitution.target().clone(),
        source.window,
        target_locals,
        source.dimensions.clone(),
        substituted_normal,
    )?;
    let same_normal_form =
        normalize_after_substitution.normal_form == normalize_substituted_normal_form.normal_form;
    let same_normal_type = normalize_after_substitution.normal_form_type_normal_form
        == normalize_substituted_normal_form.normal_form_type_normal_form;
    let cubical_bundle_transport_used = false;
    let holds = same_normal_form && same_normal_type;
    if !holds {
        return Err(E3NormalizationError::CongruenceFailed);
    }
    let derivation_hash = tagged_hash(
        "substitution-normalization-congruence",
        &(
            source.derivation_hash(),
            substitution.derivation_hash(),
            normalize_after_substitution.derivation_hash(),
            normalize_substituted_normal_form.derivation_hash(),
            same_normal_form,
            same_normal_type,
            cubical_bundle_transport_used,
            holds,
        ),
    );
    Ok(SubstitutionNormalizationCongruenceToken {
        source,
        substitution,
        normalize_after_substitution,
        normalize_substituted_normal_form,
        same_normal_form,
        same_normal_type,
        cubical_bundle_transport_used,
        holds,
        derivation_hash,
    })
}

pub fn replay_substitution_normalization_congruence(
    token: &SubstitutionNormalizationCongruenceToken,
) -> Result<(), E3NormalizationError> {
    let replay = issue_substitution_normalization_congruence(
        token.source.clone(),
        token.substitution.clone(),
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(E3NormalizationError::ReplayMismatch)
    }
}

fn interpretation_derivation_references(interpretation: &OrdinaryInterpretation) -> Vec<String> {
    let mut refs = match interpretation {
        OrdinaryInterpretation::PostPathCoherence { operation, .. } => {
            vec![operation.as_str().to_owned()]
        }
        OrdinaryInterpretation::CellAction {
            operation,
            registered_boundary_bundle,
            ..
        } => vec![
            operation.as_str().to_owned(),
            registered_boundary_bundle.as_str().to_owned(),
        ],
        _ => Vec::new(),
    };
    refs.sort();
    refs.dedup();
    refs
}

pub fn issue_ordinary_schema_normalization(
    source: OrdinarySchema,
) -> Result<OrdinarySchemaNormalizationToken, E3NormalizationError> {
    replay_ordinary_schema(&source)?;
    let origin = OrdinaryOriginProvenance {
        formation_derivation_hash: source.formation_derivation_hash().to_owned(),
        anchors: source.support_proof().anchors().to_vec(),
        source_telescopes: source.support_proof().source_telescopes().to_vec(),
        semantic_support: source.semantic_support().clone(),
        derivation_references: interpretation_derivation_references(source.interpretation()),
    };
    let context_derivation_hash = source.context().derivation_hash().to_owned();
    let support_window = source.support_proof().window();
    let semantic_equality_key = tagged_hash(
        "ordinary-semantic-equality-key-not-family-id",
        &(
            &context_derivation_hash,
            support_window,
            source.constructor(),
            source.interpretation(),
        ),
    );
    let normal_form = FrozenOrdinaryNormalForm {
        context_derivation_hash,
        support_window,
        constructor: source.constructor(),
        interpretation: source.interpretation().clone(),
        semantic_equality_key,
        origin,
    };
    let typed_before_and_after = true;
    let replay_stable = true;
    let provenance_retained = true;
    let reference_or_new_family_decided = false;
    let e4_verdict_issued = false;
    let derivation_hash = tagged_hash(
        "ordinary-schema-normalization",
        &(
            source.formation_derivation_hash(),
            &normal_form,
            typed_before_and_after,
            replay_stable,
            provenance_retained,
            reference_or_new_family_decided,
            e4_verdict_issued,
            E3_NO_E4_VERDICT,
        ),
    );
    Ok(OrdinarySchemaNormalizationToken {
        source,
        normal_form,
        typed_before_and_after,
        replay_stable,
        provenance_retained,
        reference_or_new_family_decided,
        e4_verdict_issued,
        derivation_hash,
    })
}

pub fn replay_ordinary_schema_normalization(
    token: &OrdinarySchemaNormalizationToken,
) -> Result<(), E3NormalizationError> {
    let replay = issue_ordinary_schema_normalization(token.source.clone())?;
    if replay == *token {
        Ok(())
    } else {
        Err(E3NormalizationError::ReplayMismatch)
    }
}

pub fn issue_frozen_ordinary_equality(
    left: OrdinarySchemaNormalizationToken,
    right: OrdinarySchemaNormalizationToken,
) -> Result<FrozenOrdinaryEqualityToken, E3NormalizationError> {
    replay_ordinary_schema_normalization(&left)?;
    replay_ordinary_schema_normalization(&right)?;
    if left.source.context() != right.source.context()
        || left.source.support_proof().window() != right.source.support_proof().window()
    {
        return Err(E3NormalizationError::EqualityContextMismatch);
    }
    let equal = left.normal_form.constructor == right.normal_form.constructor
        && left.normal_form.interpretation == right.normal_form.interpretation
        && left.normal_form.semantic_equality_key == right.normal_form.semantic_equality_key;
    let origin_provenance_was_not_quotiented = true;
    let family_membership_decided = false;
    let derivation_hash = tagged_hash(
        "frozen-ordinary-equality",
        &(
            left.derivation_hash(),
            right.derivation_hash(),
            equal,
            origin_provenance_was_not_quotiented,
            family_membership_decided,
        ),
    );
    Ok(FrozenOrdinaryEqualityToken {
        left,
        right,
        equal,
        origin_provenance_was_not_quotiented,
        family_membership_decided,
        derivation_hash,
    })
}

pub fn replay_frozen_ordinary_equality(
    token: &FrozenOrdinaryEqualityToken,
) -> Result<(), E3NormalizationError> {
    let replay = issue_frozen_ordinary_equality(token.left.clone(), token.right.clone())?;
    if replay == *token {
        Ok(())
    } else {
        Err(E3NormalizationError::ReplayMismatch)
    }
}

fn audit_base_context() -> Result<FormedSchemaContext, E3NormalizationError> {
    Ok(crate::context::form_schema_context(vec![
        crate::context::Declaration::TypeParameter {
            binder: BinderId(0),
            name: "A".to_owned(),
            universe: 0,
        },
        crate::context::Declaration::OpaqueElement {
            binder: BinderId(1),
            name: "a".to_owned(),
            ty: TypeExpr::parameter(0),
        },
    ])?)
}

fn audit_element() -> SchemaTypeExpr {
    SchemaTypeExpr::Element {
        carrier: TypeExpr::parameter(0),
    }
}

fn audit_beta_redex() -> SchemaTermExpr {
    SchemaTermExpr::App {
        function: Box::new(SchemaTermExpr::Lam {
            binder: SchemaBinderId(10),
            domain: Box::new(audit_element()),
            body: Box::new(SchemaTermExpr::Variable {
                binder: SchemaBinderId(10),
            }),
        }),
        argument: Box::new(SchemaTermExpr::Ambient {
            term: TermExpr::variable(1),
        }),
    }
}

pub fn issue_e3_fragment_audit() -> Result<E3FragmentAuditToken, E3NormalizationError> {
    let context = audit_base_context()?;
    let window = SupportWindow::new(7, 8)?;
    let beta_normalization = issue_schema_term_normalization(
        context.clone(),
        window,
        Vec::new(),
        Vec::new(),
        audit_beta_redex(),
    )?;
    let normal = issue_schema_term_normalization(
        context.clone(),
        window,
        Vec::new(),
        Vec::new(),
        SchemaTermExpr::Ambient {
            term: TermExpr::variable(1),
        },
    )?;
    let positive_equality =
        issue_frozen_schema_equality(beta_normalization.clone(), normal.clone())?;
    if !positive_equality.equal {
        return Err(E3NormalizationError::CongruenceFailed);
    }
    let negative_local = SchemaLocalDeclaration {
        binder: SchemaBinderId(4),
        ty: audit_element(),
    };
    let negative_left = issue_schema_term_normalization(
        context.clone(),
        window,
        vec![negative_local.clone()],
        Vec::new(),
        SchemaTermExpr::Ambient {
            term: TermExpr::variable(1),
        },
    )?;
    let negative_right = issue_schema_term_normalization(
        context.clone(),
        window,
        vec![negative_local],
        Vec::new(),
        SchemaTermExpr::Variable {
            binder: SchemaBinderId(4),
        },
    )?;
    let negative_equality = issue_frozen_schema_equality(negative_left, negative_right)?;
    if negative_equality.equal {
        return Err(E3NormalizationError::CongruenceFailed);
    }
    let normalization_congruence = issue_normalization_congruence(beta_normalization.clone())?;

    let weakening_target = crate::context::form_schema_context(vec![
        crate::context::Declaration::TypeParameter {
            binder: BinderId(0),
            name: "A".to_owned(),
            universe: 0,
        },
        crate::context::Declaration::OpaqueElement {
            binder: BinderId(1),
            name: "a".to_owned(),
            ty: TypeExpr::parameter(0),
        },
        crate::context::Declaration::IntervalVariable {
            binder: BinderId(2),
            name: "i".to_owned(),
        },
    ])?;
    let weakening_congruence =
        issue_weakening_normalization_congruence(beta_normalization.clone(), weakening_target)?;

    let substitution_target = crate::context::form_schema_context(vec![
        crate::context::Declaration::TypeParameter {
            binder: BinderId(10),
            name: "B".to_owned(),
            universe: 0,
        },
        crate::context::Declaration::OpaqueElement {
            binder: BinderId(11),
            name: "b".to_owned(),
            ty: TypeExpr::parameter(10),
        },
    ])?;
    let genuine_substitution = crate::context::issue_typed_substitution(
        &context,
        &substitution_target,
        vec![
            SubstitutionImage::Type {
                source: BinderId(0),
                image: TypeExpr::trunc(TypeExpr::parameter(10)),
            },
            SubstitutionImage::Term {
                source: BinderId(1),
                image: TermExpr::TruncPoint {
                    carrier: Box::new(TypeExpr::parameter(10)),
                    point: Box::new(TermExpr::variable(11)),
                },
            },
        ],
    )?;
    let genuine_substitution_congruence = issue_substitution_normalization_congruence(
        beta_normalization.clone(),
        genuine_substitution,
    )?;
    if genuine_substitution_congruence
        .substitution
        .genuine_expression_images()
        != 2
    {
        return Err(E3NormalizationError::CongruenceFailed);
    }

    let ordinary_left = crate::grammar::form_ordinary_schema(
        context.clone(),
        OrdinarySchemaKind::PointOrUnitIntro,
        OrdinaryInterpretation::PointOrUnitIntro {
            carrier: TypeExpr::parameter(0),
            point: TermExpr::variable(1),
        },
        crate::grammar::FamilyPresentation::CanonicalFamily,
        window,
        vec![ClauseAnchor { step: 8, clause: 0 }],
    )?;
    let ordinary_right = crate::grammar::form_ordinary_schema(
        context,
        OrdinarySchemaKind::PointOrUnitIntro,
        OrdinaryInterpretation::PointOrUnitIntro {
            carrier: TypeExpr::parameter(0),
            point: TermExpr::variable(1),
        },
        crate::grammar::FamilyPresentation::CanonicalFamily,
        window,
        vec![ClauseAnchor { step: 8, clause: 1 }],
    )?;
    let ordinary_anchor_equality = issue_frozen_ordinary_equality(
        issue_ordinary_schema_normalization(ordinary_left)?,
        issue_ordinary_schema_normalization(ordinary_right)?,
    )?;
    if !ordinary_anchor_equality.equal
        || ordinary_anchor_equality.left.normal_form.origin.anchors
            == ordinary_anchor_equality.right.normal_form.origin.anchors
    {
        return Err(E3NormalizationError::CongruenceFailed);
    }

    let version = SCHEMA2_E3_FROZEN_FRAGMENT_VERSION.to_owned();
    let equality_procedure = SCHEMA2_FROZEN_EQUALITY_PROCEDURE.to_owned();
    let kernel_equality_procedure = KERNEL_EQUALITY_PROCEDURE.to_owned();
    let schema_term_constructor_coverage = [
        "Ambient", "Variable", "Pair", "Lam", "App", "CubeAt", "MapCube",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    let schema_type_constructor_coverage = ["Element", "Product", "Pi", "Path", "Cube"]
        .into_iter()
        .map(str::to_owned)
        .collect::<Vec<_>>();
    let ordinary_constructor_coverage = OrdinarySchemaKind::ALL.to_vec();
    let termination_by_construction = true;
    let output_type_rechecked_and_preserved = beta_normalization.type_preserved;
    let replay_stable = beta_normalization.normalization_is_stable;
    let provenance_source_and_normal_form_retained = beta_normalization
        .provenance
        .normal_form_provenance_is_source_derived;
    let total_normalization_proved = false;
    let general_univalent_equality_proved = false;
    let arbitrary_context_presentation_quotient_proved = false;
    let cubical_bundle_substitution_transport_proved = false;
    let general_c6_complete = false;
    let e4_generator_completeness_proved = false;
    let independent_verdict_issued = false;
    let ordinary_family_token_issued = false;
    let stage_count_computed = false;
    let fq2_evaluated = false;
    let handoff_issued = false;
    let gaps = vec![
        E3_TOTAL_NORMALIZATION_GAP.to_owned(),
        E3_GENERAL_UNIVALENT_EQUALITY_GAP.to_owned(),
        E3_GENERAL_C6_GAP.to_owned(),
        E3_CUBICAL_SUBSTITUTION_PROVENANCE_GAP.to_owned(),
        E3_CONTEXT_PRESENTATION_GAP.to_owned(),
        E3_NO_E4_VERDICT.to_owned(),
    ];
    let derivation_hash = tagged_hash(
        "e3-fragment-audit",
        &(
            (
                &version,
                &equality_procedure,
                &kernel_equality_procedure,
                &schema_term_constructor_coverage,
                &schema_type_constructor_coverage,
                &ordinary_constructor_coverage,
            ),
            (
                beta_normalization.derivation_hash(),
                positive_equality.derivation_hash(),
                negative_equality.derivation_hash(),
                normalization_congruence.derivation_hash(),
                weakening_congruence.derivation_hash(),
                genuine_substitution_congruence.derivation_hash(),
                ordinary_anchor_equality.derivation_hash(),
            ),
            (
                termination_by_construction,
                output_type_rechecked_and_preserved,
                replay_stable,
                provenance_source_and_normal_form_retained,
                total_normalization_proved,
                general_univalent_equality_proved,
                arbitrary_context_presentation_quotient_proved,
                cubical_bundle_substitution_transport_proved,
                general_c6_complete,
                e4_generator_completeness_proved,
                independent_verdict_issued,
                ordinary_family_token_issued,
                stage_count_computed,
                fq2_evaluated,
                handoff_issued,
                &gaps,
            ),
        ),
    );
    Ok(E3FragmentAuditToken {
        version,
        equality_procedure,
        kernel_equality_procedure,
        schema_term_constructor_coverage,
        schema_type_constructor_coverage,
        ordinary_constructor_coverage,
        beta_normalization,
        positive_equality,
        negative_equality,
        normalization_congruence,
        weakening_congruence,
        genuine_substitution_congruence,
        ordinary_anchor_equality,
        termination_by_construction,
        output_type_rechecked_and_preserved,
        replay_stable,
        provenance_source_and_normal_form_retained,
        total_normalization_proved,
        general_univalent_equality_proved,
        arbitrary_context_presentation_quotient_proved,
        cubical_bundle_substitution_transport_proved,
        general_c6_complete,
        e4_generator_completeness_proved,
        independent_verdict_issued,
        ordinary_family_token_issued,
        stage_count_computed,
        fq2_evaluated,
        handoff_issued,
        gaps,
        derivation_hash,
    })
}

pub fn replay_e3_fragment_audit(token: &E3FragmentAuditToken) -> Result<(), E3NormalizationError> {
    let replay = issue_e3_fragment_audit()?;
    if replay == *token {
        Ok(())
    } else {
        Err(E3NormalizationError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::{Declaration, form_schema_context, identity_substitution};
    use crate::grammar::{
        DerivationRef, FamilyPresentation, OrdinaryInterpretation, OrdinarySchemaKind,
        form_ordinary_schema,
    };

    fn base_context() -> FormedSchemaContext {
        crate::context::form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(0),
                name: "A".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(1),
                name: "a".to_owned(),
                ty: TypeExpr::parameter(0),
            },
        ])
        .expect("formed")
    }

    fn element() -> SchemaTypeExpr {
        SchemaTypeExpr::Element {
            carrier: TypeExpr::parameter(0),
        }
    }

    fn beta_redex() -> SchemaTermExpr {
        SchemaTermExpr::App {
            function: Box::new(SchemaTermExpr::Lam {
                binder: SchemaBinderId(10),
                domain: Box::new(element()),
                body: Box::new(SchemaTermExpr::Variable {
                    binder: SchemaBinderId(10),
                }),
            }),
            argument: Box::new(SchemaTermExpr::Ambient {
                term: TermExpr::variable(1),
            }),
        }
    }

    fn normalize(term: SchemaTermExpr) -> SchemaTermNormalizationToken {
        issue_schema_term_normalization(
            base_context(),
            SupportWindow::new(7, 8).unwrap(),
            Vec::new(),
            Vec::new(),
            term,
        )
        .expect("typed normalization")
    }

    #[test]
    fn beta_normalization_is_typed_stable_replayable_and_provenance_preserving() {
        let token = normalize(beta_redex());
        assert_eq!(
            token.normal_form(),
            &SchemaTermExpr::Ambient {
                term: TermExpr::variable(1)
            }
        );
        assert_eq!(token.beta_steps(), 1);
        assert!(token.type_preserved());
        assert!(token.normalization_is_stable());
        assert!(token.termination_by_construction());
        assert!(!token.total_over_all_well_typed_schema2_terms());
        assert!(!token.general_c6_complete());
        assert!(!token.e4_verdict_issued());
        assert!(token.provenance().normal_form_provenance_is_source_derived);
        replay_schema_term_normalization(&token).expect("replay");
    }

    #[test]
    fn frozen_equality_is_beta_equality_with_surviving_provenance_equality() {
        let redex = normalize(beta_redex());
        let normal = normalize(SchemaTermExpr::Ambient {
            term: TermExpr::variable(1),
        });
        let equality = issue_frozen_schema_equality(redex, normal).expect("same typed context");
        assert!(equality.equal);
        assert!(equality.source_provenance_retained());
        assert!(equality.surviving_provenance_equal());
        assert!(!equality.family_reclassification_decided());
        assert!(!equality.e4_verdict_issued);
        replay_frozen_schema_equality(&equality).expect("replay");
    }

    #[test]
    fn unused_predecessor_library_argument_is_erased_without_family_reclassification() {
        let context = form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(0),
                name: "A".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(1),
                name: "a".to_owned(),
                ty: TypeExpr::parameter(0),
            },
            Declaration::LibraryReference {
                binder: BinderId(2),
                name: "predecessor".to_owned(),
                step: 7,
                symbol: "unused-predecessor-point".to_owned(),
                ty: TypeExpr::parameter(0),
            },
        ])
        .unwrap();
        let window = SupportWindow::new(7, 8).unwrap();
        let redex = issue_schema_term_normalization(
            context.clone(),
            window,
            Vec::new(),
            Vec::new(),
            SchemaTermExpr::App {
                function: Box::new(SchemaTermExpr::Lam {
                    binder: SchemaBinderId(10),
                    domain: Box::new(element()),
                    body: Box::new(SchemaTermExpr::Ambient {
                        term: TermExpr::variable(1),
                    }),
                }),
                argument: Box::new(SchemaTermExpr::Ambient {
                    term: TermExpr::Library {
                        step: 7,
                        symbol: "unused-predecessor-point".to_owned(),
                    },
                }),
            },
        )
        .unwrap();
        let direct = issue_schema_term_normalization(
            context,
            window,
            Vec::new(),
            Vec::new(),
            SchemaTermExpr::Ambient {
                term: TermExpr::variable(1),
            },
        )
        .unwrap();
        assert_ne!(redex.provenance().source, direct.provenance().source);
        assert_eq!(
            redex.provenance().normal_form,
            direct.provenance().normal_form
        );
        assert_eq!(redex.provenance().source.ambient_support.libraries.len(), 1);
        assert!(
            direct
                .provenance()
                .source
                .ambient_support
                .libraries
                .is_empty()
        );

        let equality = issue_frozen_schema_equality(redex, direct).unwrap();
        assert!(equality.equal());
        assert!(equality.source_provenance_retained());
        assert!(equality.surviving_provenance_equal());
        assert!(!equality.family_reclassification_decided());
        replay_frozen_schema_equality(&equality).unwrap();
    }

    #[test]
    fn lam_and_pi_bound_binders_are_removed_from_free_provenance() {
        let binder = SchemaBinderId(10);
        let token = normalize(SchemaTermExpr::Lam {
            binder,
            domain: Box::new(element()),
            body: Box::new(SchemaTermExpr::Variable { binder }),
        });
        assert!(token.provenance().source.local_binders.is_empty());
        assert!(token.provenance().normal_form.local_binders.is_empty());
        let mut forged_free_binder = token.clone();
        forged_free_binder
            .provenance
            .normal_form
            .local_binders
            .push(binder);
        assert_eq!(
            replay_schema_term_normalization(&forged_free_binder),
            Err(E3NormalizationError::ReplayMismatch)
        );

        let pi_binder = SchemaBinderId(42);
        let pi = SchemaTypeExpr::Pi {
            binder: pi_binder,
            domain: Box::new(element()),
            codomain: Box::new(SchemaTypeExpr::Path {
                carrier: TypeExpr::parameter(0),
                left: Box::new(SchemaTermExpr::Variable { binder: pi_binder }),
                right: Box::new(SchemaTermExpr::Variable { binder: pi_binder }),
            }),
        };
        let provenance = expression_provenance(
            &SchemaTermExpr::Ambient {
                term: TermExpr::variable(1),
            },
            &pi,
        );
        assert!(!provenance.local_binders.contains(&pi_binder));

        let free_function = expression_provenance(
            &SchemaTermExpr::Variable { binder: pi_binder },
            &SchemaTypeExpr::Pi {
                binder: pi_binder,
                domain: Box::new(element()),
                codomain: Box::new(element()),
            },
        );
        assert_eq!(free_function.local_binders, vec![pi_binder]);
    }

    #[test]
    fn cube_at_variable_coordinate_is_recorded_as_dimension_provenance() {
        let context = base_context();
        let dimension = SchemaDimBinderId(9);
        let cube = SchemaBinderId(8);
        let local = SchemaLocalDeclaration {
            binder: cube,
            ty: SchemaTypeExpr::Cube {
                carrier: TypeExpr::parameter(0),
                boundary: Box::new(SchemaTermExpr::Ambient {
                    term: TermExpr::variable(1),
                }),
                dimension: 1,
                source_bundle: RegisteredBoundaryBundleRef {
                    derivation: DerivationRef::parse(format!("blake3:{}", "d".repeat(64))).unwrap(),
                    source_step: 8,
                    carrier: TypeExpr::parameter(0),
                    dimension: 1,
                },
            },
        };
        let token = issue_schema_term_normalization(
            context,
            SupportWindow::new(7, 8).unwrap(),
            vec![local],
            vec![dimension],
            SchemaTermExpr::CubeAt {
                cube: Box::new(SchemaTermExpr::Variable { binder: cube }),
                coordinates: vec![SchemaDimExpr::Variable { binder: dimension }],
            },
        )
        .unwrap();
        assert_eq!(token.provenance().source.schema_dimensions, vec![dimension]);
        assert_eq!(
            token.provenance().normal_form.schema_dimensions,
            vec![dimension]
        );
        replay_schema_term_normalization(&token).unwrap();
        let mut erased_dimension = token;
        erased_dimension.provenance.source.schema_dimensions.clear();
        assert_eq!(
            replay_schema_term_normalization(&erased_dimension),
            Err(E3NormalizationError::ReplayMismatch)
        );
    }

    #[test]
    fn equivalence_laws_and_normalization_congruence_replay() {
        let redex = normalize(beta_redex());
        let normal = normalize(SchemaTermExpr::Ambient {
            term: TermExpr::variable(1),
        });
        let refl = issue_equality_reflexivity(redex.clone()).unwrap();
        replay_frozen_equality_law(&refl).unwrap();
        let premise = issue_frozen_schema_equality(redex.clone(), normal.clone()).unwrap();
        let symmetric = issue_equality_symmetry(premise.clone()).unwrap();
        replay_frozen_equality_law(&symmetric).unwrap();
        let second = issue_frozen_schema_equality(normal.clone(), redex.clone()).unwrap();
        let transitive = issue_equality_transitivity(premise, second).unwrap();
        replay_frozen_equality_law(&transitive).unwrap();
        let congruence = issue_normalization_congruence(redex).unwrap();
        assert!(congruence.holds);
        replay_normalization_congruence(&congruence).unwrap();
    }

    #[test]
    fn distinct_stuck_terms_remain_distinct() {
        let local = SchemaLocalDeclaration {
            binder: SchemaBinderId(4),
            ty: element(),
        };
        let left = issue_schema_term_normalization(
            base_context(),
            SupportWindow::new(7, 8).unwrap(),
            vec![local.clone()],
            Vec::new(),
            SchemaTermExpr::Ambient {
                term: TermExpr::variable(1),
            },
        )
        .unwrap();
        let right = issue_schema_term_normalization(
            base_context(),
            SupportWindow::new(7, 8).unwrap(),
            vec![local],
            Vec::new(),
            SchemaTermExpr::Variable {
                binder: SchemaBinderId(4),
            },
        )
        .unwrap();
        let decision = issue_frozen_schema_equality(left, right).unwrap();
        assert!(!decision.equal());
        replay_frozen_schema_equality(&decision).unwrap();
    }

    #[test]
    fn weakening_preserves_normalization_and_provenance() {
        let source = normalize(beta_redex());
        let target = form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(0),
                name: "A".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(1),
                name: "a".to_owned(),
                ty: TypeExpr::parameter(0),
            },
            Declaration::IntervalVariable {
                binder: BinderId(2),
                name: "i".to_owned(),
            },
        ])
        .unwrap();
        let token = issue_weakening_normalization_congruence(source, target).unwrap();
        assert!(token.holds);
        assert!(token.provenance_not_reclassified);
        replay_weakening_normalization_congruence(&token).unwrap();
    }

    #[test]
    fn typed_substitution_commutes_with_beta_normalization_in_non_cubical_fragment() {
        let source = normalize(beta_redex());
        let target = form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(10),
                name: "B".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(11),
                name: "b".to_owned(),
                ty: TypeExpr::parameter(10),
            },
        ])
        .unwrap();
        let substitution = crate::context::issue_typed_substitution(
            source.context(),
            &target,
            vec![
                SubstitutionImage::Type {
                    source: BinderId(0),
                    image: TypeExpr::trunc(TypeExpr::parameter(10)),
                },
                SubstitutionImage::Term {
                    source: BinderId(1),
                    image: TermExpr::TruncPoint {
                        carrier: Box::new(TypeExpr::parameter(10)),
                        point: Box::new(TermExpr::variable(11)),
                    },
                },
            ],
        )
        .unwrap();
        assert_eq!(substitution.genuine_expression_images(), 2);
        let token = issue_substitution_normalization_congruence(source, substitution).unwrap();
        assert!(token.holds());
        assert!(!token.cubical_bundle_transport_used());
        assert_eq!(
            token.normalize_after_substitution().normal_form(),
            &SchemaTermExpr::Ambient {
                term: TermExpr::TruncPoint {
                    carrier: Box::new(TypeExpr::parameter(10)),
                    point: Box::new(TermExpr::variable(11)),
                }
            }
        );
        replay_substitution_normalization_congruence(&token).unwrap();
    }

    #[test]
    fn cubical_substitution_fails_closed_without_bundle_transport() {
        let context = base_context();
        let bundle = RegisteredBoundaryBundleRef {
            derivation: DerivationRef::parse(format!("blake3:{}", "a".repeat(64))).unwrap(),
            source_step: 8,
            carrier: TypeExpr::parameter(0),
            dimension: 1,
        };
        let local = SchemaLocalDeclaration {
            binder: SchemaBinderId(8),
            ty: SchemaTypeExpr::Cube {
                carrier: TypeExpr::parameter(0),
                boundary: Box::new(SchemaTermExpr::Ambient {
                    term: TermExpr::variable(1),
                }),
                dimension: 1,
                source_bundle: bundle,
            },
        };
        let source = issue_schema_term_normalization(
            context.clone(),
            SupportWindow::new(7, 8).unwrap(),
            vec![local],
            Vec::new(),
            SchemaTermExpr::Variable {
                binder: SchemaBinderId(8),
            },
        )
        .unwrap();
        let substitution = identity_substitution(&context).unwrap();
        assert!(matches!(
            issue_substitution_normalization_congruence(source, substitution),
            Err(E3NormalizationError::CubicalSubstitutionProvenanceGap { .. })
        ));
    }

    #[test]
    fn ordinary_normal_form_keeps_anchors_outside_semantic_equality_key() {
        let context = base_context();
        let left = form_ordinary_schema(
            context.clone(),
            OrdinarySchemaKind::PointOrUnitIntro,
            OrdinaryInterpretation::PointOrUnitIntro {
                carrier: TypeExpr::parameter(0),
                point: TermExpr::variable(1),
            },
            FamilyPresentation::CanonicalFamily,
            SupportWindow::new(7, 8).unwrap(),
            vec![ClauseAnchor { step: 8, clause: 0 }],
        )
        .unwrap();
        let right = form_ordinary_schema(
            context,
            OrdinarySchemaKind::PointOrUnitIntro,
            OrdinaryInterpretation::PointOrUnitIntro {
                carrier: TypeExpr::parameter(0),
                point: TermExpr::variable(1),
            },
            FamilyPresentation::CanonicalFamily,
            SupportWindow::new(7, 8).unwrap(),
            vec![ClauseAnchor { step: 8, clause: 1 }],
        )
        .unwrap();
        let left = issue_ordinary_schema_normalization(left).unwrap();
        let right = issue_ordinary_schema_normalization(right).unwrap();
        assert_ne!(
            left.normal_form().origin.anchors,
            right.normal_form().origin.anchors
        );
        assert_eq!(
            left.normal_form().semantic_equality_key,
            right.normal_form().semantic_equality_key
        );
        assert!(!left.reference_or_new_family_decided());
        let equality = issue_frozen_ordinary_equality(left, right).unwrap();
        assert!(equality.equal());
        assert!(equality.origin_provenance_was_not_quotiented());
        assert!(!equality.family_membership_decided());
        replay_frozen_ordinary_equality(&equality).unwrap();
    }

    #[test]
    fn ordinary_equality_rejects_unrelated_contexts_and_distinct_windows() {
        let make = |context: FormedSchemaContext, window: SupportWindow, step: u32| {
            form_ordinary_schema(
                context,
                OrdinarySchemaKind::PointOrUnitIntro,
                OrdinaryInterpretation::PointOrUnitIntro {
                    carrier: TypeExpr::parameter(0),
                    point: TermExpr::variable(1),
                },
                FamilyPresentation::CanonicalFamily,
                window,
                vec![ClauseAnchor { step, clause: 0 }],
            )
            .unwrap()
        };
        let left = issue_ordinary_schema_normalization(make(
            base_context(),
            SupportWindow::new(7, 8).unwrap(),
            8,
        ))
        .unwrap();
        let unrelated = form_schema_context(vec![
            Declaration::TypeParameter {
                binder: BinderId(0),
                name: "B".to_owned(),
                universe: 0,
            },
            Declaration::OpaqueElement {
                binder: BinderId(1),
                name: "b".to_owned(),
                ty: TypeExpr::parameter(0),
            },
        ])
        .unwrap();
        let unrelated = issue_ordinary_schema_normalization(make(
            unrelated,
            SupportWindow::new(7, 8).unwrap(),
            8,
        ))
        .unwrap();
        assert_eq!(
            issue_frozen_ordinary_equality(left.clone(), unrelated),
            Err(E3NormalizationError::EqualityContextMismatch)
        );
        let other_window = issue_ordinary_schema_normalization(make(
            base_context(),
            SupportWindow::new(8, 9).unwrap(),
            9,
        ))
        .unwrap();
        assert_eq!(
            issue_frozen_ordinary_equality(left, other_window),
            Err(E3NormalizationError::EqualityContextMismatch)
        );
    }

    #[test]
    fn binder_collision_after_beta_fails_closed_at_output_retyping() {
        let function_type = SchemaTypeExpr::Pi {
            binder: SchemaBinderId(30),
            domain: Box::new(element()),
            codomain: Box::new(element()),
        };
        let colliding = SchemaTermExpr::App {
            function: Box::new(SchemaTermExpr::Lam {
                binder: SchemaBinderId(10),
                domain: Box::new(function_type),
                body: Box::new(SchemaTermExpr::Lam {
                    binder: SchemaBinderId(30),
                    domain: Box::new(element()),
                    body: Box::new(SchemaTermExpr::Variable {
                        binder: SchemaBinderId(10),
                    }),
                }),
            }),
            argument: Box::new(SchemaTermExpr::Lam {
                binder: SchemaBinderId(30),
                domain: Box::new(element()),
                body: Box::new(SchemaTermExpr::Variable {
                    binder: SchemaBinderId(30),
                }),
            }),
        };
        assert!(matches!(
            issue_schema_term_normalization(
                base_context(),
                SupportWindow::new(7, 8).unwrap(),
                Vec::new(),
                Vec::new(),
                colliding,
            ),
            Err(E3NormalizationError::Grammar(
                OrdinaryGrammarError::DuplicateSchemaBinder { .. }
            ))
        ));
    }

    #[test]
    fn malformed_or_differently_typed_equality_is_rejected() {
        let redex = normalize(beta_redex());
        let pair = normalize(SchemaTermExpr::Pair {
            left: Box::new(SchemaTermExpr::Ambient {
                term: TermExpr::variable(1),
            }),
            right: Box::new(SchemaTermExpr::Ambient {
                term: TermExpr::variable(1),
            }),
        });
        assert_eq!(
            issue_frozen_schema_equality(redex, pair),
            Err(E3NormalizationError::EqualityTypeMismatch)
        );
    }

    #[test]
    fn token_mutation_fails_replay() {
        let mut token = normalize(beta_redex());
        token.beta_steps = 0;
        assert_eq!(
            replay_schema_term_normalization(&token),
            Err(E3NormalizationError::ReplayMismatch)
        );

        let redex = normalize(beta_redex());
        let normal = normalize(SchemaTermExpr::Ambient {
            term: TermExpr::variable(1),
        });
        let equality = issue_frozen_schema_equality(redex.clone(), normal).unwrap();
        let mut changed_verdict = equality.clone();
        changed_verdict.equal = false;
        assert_eq!(
            replay_frozen_schema_equality(&changed_verdict),
            Err(E3NormalizationError::ReplayMismatch)
        );
        let mut dropped_source_lineage = equality.clone();
        dropped_source_lineage.source_provenance_retained = false;
        assert_eq!(
            replay_frozen_schema_equality(&dropped_source_lineage),
            Err(E3NormalizationError::ReplayMismatch)
        );
        let mut changed_surviving_provenance = equality.clone();
        changed_surviving_provenance.surviving_provenance_equal = false;
        assert_eq!(
            replay_frozen_schema_equality(&changed_surviving_provenance),
            Err(E3NormalizationError::ReplayMismatch)
        );
        let mut forged_reclassification = equality;
        forged_reclassification.family_reclassification_decided = true;
        assert_eq!(
            replay_frozen_schema_equality(&forged_reclassification),
            Err(E3NormalizationError::ReplayMismatch)
        );
        let substitution = identity_substitution(redex.context()).unwrap();
        let mut congruence =
            issue_substitution_normalization_congruence(redex, substitution).unwrap();
        congruence.same_normal_form = false;
        assert_eq!(
            replay_substitution_normalization_congruence(&congruence),
            Err(E3NormalizationError::ReplayMismatch)
        );
    }

    #[test]
    fn aggregate_fragment_audit_replays_and_cannot_be_promoted() {
        let token = issue_e3_fragment_audit().unwrap();
        assert_eq!(token.schema_term_constructor_coverage().len(), 7);
        assert_eq!(token.schema_type_constructor_coverage().len(), 5);
        assert_eq!(token.ordinary_constructor_coverage().len(), 9);
        assert!(token.positive_equality().equal());
        assert!(token.positive_equality().source_provenance_retained());
        assert!(token.positive_equality().surviving_provenance_equal());
        assert!(!token.positive_equality().family_reclassification_decided());
        assert!(!token.negative_equality().equal());
        assert!(!token.negative_equality().surviving_provenance_equal());
        assert!(!token.negative_equality().family_reclassification_decided());
        assert_eq!(
            token
                .genuine_substitution_congruence()
                .substitution()
                .genuine_expression_images(),
            2
        );
        assert!(token.termination_by_construction());
        assert!(token.output_type_rechecked_and_preserved());
        assert!(token.replay_stable());
        assert!(token.provenance_source_and_normal_form_retained());
        assert!(!token.total_normalization_proved());
        assert!(!token.general_univalent_equality_proved());
        assert!(!token.arbitrary_context_presentation_quotient_proved());
        assert!(!token.cubical_bundle_substitution_transport_proved());
        assert!(!token.general_c6_complete());
        assert!(!token.e4_generator_completeness_proved());
        assert!(!token.independent_verdict_issued());
        assert!(!token.ordinary_family_token_issued());
        assert!(!token.stage_count_computed());
        assert!(!token.fq2_evaluated());
        assert!(!token.handoff_issued());
        replay_e3_fragment_audit(&token).unwrap();

        let mut mutated = token;
        mutated.total_normalization_proved = true;
        assert_eq!(
            replay_e3_fragment_audit(&mutated),
            Err(E3NormalizationError::ReplayMismatch)
        );
    }

    #[test]
    fn procedure_and_scope_are_frozen_and_count_blind() {
        assert_eq!(
            SCHEMA2_FROZEN_EQUALITY_PROCEDURE,
            "schema2-e3-v1-typed-beta-normal-equality"
        );
        assert_eq!(KERNEL_EQUALITY_PROCEDURE, "kernel-v1-beta-normal-equality");
        assert!(E3_SUCCESSFUL_NORMALIZATION_SCOPE.contains("successfully normalized"));
        assert!(E3_NO_E4_VERDICT.contains("E4_GENERATOR_MEMBERSHIP"));
    }
}
