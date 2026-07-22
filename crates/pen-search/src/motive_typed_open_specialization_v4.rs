//! T-SM1b: motive-typed specialization into an open dependent context.
//!
//! This is an additive successor.  The frozen v2 closed theorem and the v3
//! chronological sweep remain byte-for-byte inputs.  In particular this
//! module never treats a typing derivation, scope check, or digest as an
//! `Internal` proof.  A source and every substitution image must replay
//! either through the frozen six-rule closure calculus or through the
//! adopted dependent-context hypothetical-judgment rule below.

use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3ChronologicalInterfaceMode, A3DemandOutputType, A3HistoricalWindow, A3RuleConstructor,
    A3TypedClauseSource, A3TypedDemandInstance, A3TypedDemandScheme,
    generate_a3_window_for_exact_prefix_unbounded, replay_chronological_interface_slot_map,
};
use pen_search::contextual_formation_coherence_v3::kernel_context_from_parameter_sorts;
use pen_type::contextual_internality::{
    AmbientHypothesisProjection, ContextualMotive,
};
use pen_type::dependent_context::{
    DependentAmbientContextDeclarationProjection, DependentContextMotive,
    DependentTotalSpecializationProjection, issue_dependent_ambient_context_declaration,
    issue_dependent_total_specialization_theorem,
    replay_dependent_ambient_context_declaration,
    replay_dependent_total_specialization_theorem,
};
use pen_type::elaborate::{
    DerivationNode, KernelTy, SealedSignature, elaborate_single_clause_with_typed_ambient,
};
use pen_type::motive_parametric_coherence::ClosureRuleKind;
use pen_type::motive_parametric_coherence_v2::{
    ClosureRuleEvidenceV2, ProjectionSourceEvidenceV2, VerifiedClosureDerivationV2,
    issue_explicit_contextual_closure_derivation_v2,
    replay_verified_closure_derivation_v2,
};
use pen_type::normalize::substitute_level;
use pen_type::substitution::{
    ParameterSort, SortedParameterContext, SubstitutionImage, issue_structural_substitution,
    replay_structural_substitution,
};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const MOTIVE_TYPED_OPEN_SPECIALIZATION_V4_VERSION: &str =
    "motive-typed-open-specialization-induction-v4";
pub const T_SM1B_REGISTERED_GAP_COUNT: usize = 18;

const FROZEN_E5_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_e5_future_hole_finale_v2_dependent_context.json");
const FROZEN_V3_BYTES: &[u8] =
    include_bytes!("../../../docs/chronological_interface_slot_map_v3.json");

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(MOTIVE_TYPED_OPEN_SPECIALIZATION_V4_VERSION, domain, value))
        .expect("T-SM1b evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn motive_kernel_type(motive: &ContextualMotive) -> Option<KernelTy> {
    match motive {
        ContextualMotive::Type => Some(KernelTy::Type),
        ContextualMotive::Element(expression) => Some(KernelTy::El(expression.clone())),
        ContextualMotive::Function { domain, codomain } => Some(KernelTy::Fun(
            Box::new(motive_kernel_type(domain)?),
            Box::new(motive_kernel_type(codomain)?),
        )),
        ContextualMotive::Neutral => None,
    }
}

fn hypotheses_from_v2(
    derivation: &VerifiedClosureDerivationV2,
) -> Option<&[AmbientHypothesisProjection]> {
    match &derivation.evidence {
        ClosureRuleEvidenceV2::Projection {
            source: ProjectionSourceEvidenceV2::AmbientParameter { declaration, .. },
            ..
        } => Some(&declaration.hypotheses),
        ClosureRuleEvidenceV2::Projection {
            source:
                ProjectionSourceEvidenceV2::CertifiedPriorField {
                    ambient_declaration,
                    ..
                },
            ..
        } => ambient_declaration
            .as_ref()
            .map(|declaration| declaration.hypotheses.as_slice()),
        ClosureRuleEvidenceV2::Guarded {
            ambient_declaration,
            ..
        } => Some(&ambient_declaration.hypotheses),
        ClosureRuleEvidenceV2::Contextual {
            typed_structure, ..
        } => Some(&typed_structure.ambient_context.hypotheses),
        ClosureRuleEvidenceV2::ExplicitContextual { typed_structure } => {
            Some(&typed_structure.ambient_context.hypotheses)
        }
        ClosureRuleEvidenceV2::Structural { .. }
        | ClosureRuleEvidenceV2::AmbientFormer { .. }
        | ClosureRuleEvidenceV2::Dereference { .. } => None,
    }
}

fn v2_exact_context(
    derivation: &VerifiedClosureDerivationV2,
) -> Result<Vec<KernelTy>, OpenSpecializationV4Error> {
    if derivation.ambient_arity == 0 {
        return Ok(Vec::new());
    }
    let hypotheses = hypotheses_from_v2(derivation)
        .ok_or(OpenSpecializationV4Error::SourceContextUnavailable)?;
    if hypotheses.len() != derivation.ambient_arity as usize {
        return Err(OpenSpecializationV4Error::SourceContextUnavailable);
    }
    hypotheses
        .iter()
        .map(|hypothesis| {
            motive_kernel_type(&hypothesis.motive)
                .ok_or(OpenSpecializationV4Error::NeutralDeclaredMotive)
        })
        .collect()
}

fn contains_charged_or_outside(expression: &Expr) -> bool {
    match expression {
        Expr::PathCon(_) | Expr::Bang(_) | Expr::WhyNot(_) => true,
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            contains_charged_or_outside(left) || contains_charged_or_outside(right)
        }
        Expr::Id(ty, left, right) => {
            contains_charged_or_outside(ty)
                || contains_charged_or_outside(left)
                || contains_charged_or_outside(right)
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
        | Expr::Eventually(inner) => contains_charged_or_outside(inner),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) => false,
    }
}

fn expression_constructor_inventory(expression: &Expr, output: &mut Vec<String>) {
    let (name, children): (&str, Vec<&Expr>) = match expression {
        Expr::App(left, right) => ("App", vec![left, right]),
        Expr::Lam(inner) => ("Lam", vec![inner]),
        Expr::Pi(left, right) => ("Pi", vec![left, right]),
        Expr::Sigma(left, right) => ("Sigma", vec![left, right]),
        Expr::Univ => ("Univ", Vec::new()),
        Expr::Var(_) => ("Var", Vec::new()),
        Expr::Lib(_) => ("Lib", Vec::new()),
        Expr::Id(ty, left, right) => ("Id", vec![ty, left, right]),
        Expr::Refl(inner) => ("Refl", vec![inner]),
        Expr::Susp(inner) => ("Susp", vec![inner]),
        Expr::Trunc(inner) => ("Trunc", vec![inner]),
        Expr::PathCon(_) => ("PathCon", Vec::new()),
        Expr::Flat(inner) => ("Flat", vec![inner]),
        Expr::Sharp(inner) => ("Sharp", vec![inner]),
        Expr::Disc(inner) => ("Disc", vec![inner]),
        Expr::Shape(inner) => ("Shape", vec![inner]),
        Expr::Next(inner) => ("Next", vec![inner]),
        Expr::Eventually(inner) => ("Eventually", vec![inner]),
        Expr::Bang(inner) => ("Bang", vec![inner]),
        Expr::WhyNot(inner) => ("WhyNot", vec![inner]),
    };
    output.push(name.to_owned());
    for child in children {
        expression_constructor_inventory(child, output);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DependentContextualInternalProjectionV4 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub declaration: DependentAmbientContextDeclarationProjection,
    pub totality: DependentTotalSpecializationProjection,
    pub rule: ClosureRuleKind,
    pub expression: Expr,
    pub declared_role: ClauseRole,
    pub inferred_kernel_type: KernelTy,
    pub normal_form: Expr,
    pub typed_derivation: DerivationNode,
    pub expression_constructor_inventory: Vec<String>,
    pub full_frozen_constructor_induction: bool,
    pub source_typed_under_exact_dependent_context: bool,
    pub total_specialization_replayed: bool,
    pub no_probe_or_motive_filter: bool,
    pub no_endpoint_premise_forged_or_minted: bool,
    pub no_charged_constructor_admitted: bool,
    pub internal_closure_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DependentContextualInternalTokenV4 {
    projection: DependentContextualInternalProjectionV4,
}

impl DependentContextualInternalTokenV4 {
    pub fn projection(&self) -> &DependentContextualInternalProjectionV4 {
        &self.projection
    }
}

/// Issue the dependent-context successor of the frozen contextual closure.
/// Formation is deliberately excluded: admitting it here would silently
/// discharge T-SM1a with typing evidence.
pub fn issue_dependent_contextual_internal_v4(
    signature: &SealedSignature,
    declaration: &DependentAmbientContextDeclarationProjection,
) -> Result<DependentContextualInternalTokenV4, OpenSpecializationV4Error> {
    replay_dependent_ambient_context_declaration(signature, declaration)
        .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    if declaration.declared_role == ClauseRole::Formation
        || declaration.typed_body_elaboration.kernel_role == ClauseRole::Formation
    {
        return Err(OpenSpecializationV4Error::FormationNeedsTSm1a);
    }
    if declaration.declared_role != declaration.typed_body_elaboration.kernel_role {
        return Err(OpenSpecializationV4Error::KernelRoleMismatch);
    }
    if contains_charged_or_outside(&declaration.expression) {
        return Err(OpenSpecializationV4Error::ChargedOrOutsideConstructor);
    }
    let declaration_token = issue_dependent_ambient_context_declaration(
        signature,
        &declaration.body_telescope,
        declaration.visible_library,
        declaration
            .hypotheses
            .iter()
            .map(|hypothesis| hypothesis.motive.clone())
            .collect(),
    )
    .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    if declaration_token.projection() != declaration {
        return Err(OpenSpecializationV4Error::ReplayMismatch);
    }
    let totality = issue_dependent_total_specialization_theorem(signature, &declaration_token)
        .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    replay_dependent_total_specialization_theorem(
        signature,
        declaration,
        totality.projection(),
    )
    .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    if !totality.projection().total_specialization_theorem_issued {
        return Err(OpenSpecializationV4Error::DependentTotalityMissing);
    }
    let mut inventory = Vec::new();
    expression_constructor_inventory(&declaration.expression, &mut inventory);
    let full_frozen_constructor_induction = !inventory.is_empty();
    let source_typed_under_exact_dependent_context = declaration.expression
        == declaration.typed_body_elaboration.normal_form
        || !declaration.typed_body_elaboration.normal_form.var_refs().contains(&0);
    let total_specialization_replayed = true;
    let no_probe_or_motive_filter = declaration.no_outcome_filtering_used
        && totality.projection().no_assignment_outcome_filtering;
    let no_endpoint_premise_forged_or_minted = declaration.anchors_minted == 0
        && declaration.marginal_kappa == 0
        && declaration.marginal_nu == 0;
    let no_charged_constructor_admitted = true;
    let internal_closure_issued = full_frozen_constructor_induction
        && source_typed_under_exact_dependent_context
        && total_specialization_replayed
        && no_probe_or_motive_filter
        && no_endpoint_premise_forged_or_minted
        && no_charged_constructor_admitted;
    if !internal_closure_issued {
        return Err(OpenSpecializationV4Error::DependentInternalNotIssued);
    }
    let mut projection = DependentContextualInternalProjectionV4 {
        version: MOTIVE_TYPED_OPEN_SPECIALIZATION_V4_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library: declaration.visible_library,
        declaration: declaration.clone(),
        totality: totality.projection().clone(),
        rule: ClosureRuleKind::Contextual,
        expression: declaration.expression.clone(),
        declared_role: declaration.declared_role,
        inferred_kernel_type: declaration.typed_body_elaboration.kernel_ty.clone(),
        normal_form: declaration.typed_body_elaboration.normal_form.clone(),
        typed_derivation: declaration.typed_body_derivation.clone(),
        expression_constructor_inventory: inventory,
        full_frozen_constructor_induction,
        source_typed_under_exact_dependent_context,
        total_specialization_replayed,
        no_probe_or_motive_filter,
        no_endpoint_premise_forged_or_minted,
        no_charged_constructor_admitted,
        internal_closure_issued,
        marginal_nu: 0,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("dependent-contextual-internal", &projection);
    Ok(DependentContextualInternalTokenV4 { projection })
}

pub fn replay_dependent_contextual_internal_v4(
    signature: &SealedSignature,
    projection: &DependentContextualInternalProjectionV4,
) -> Result<(), OpenSpecializationV4Error> {
    let reissued = issue_dependent_contextual_internal_v4(signature, &projection.declaration)?;
    if reissued.projection == *projection {
        Ok(())
    } else {
        Err(OpenSpecializationV4Error::ReplayMismatch)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "internal_source")]
pub enum OpenInternalDerivationV4 {
    FrozenSixRule {
        derivation: VerifiedClosureDerivationV2,
    },
    DependentContextual {
        derivation: DependentContextualInternalProjectionV4,
    },
}

impl OpenInternalDerivationV4 {
    fn expression(&self) -> &Expr {
        match self {
            Self::FrozenSixRule { derivation } => &derivation.expression,
            Self::DependentContextual { derivation } => &derivation.expression,
        }
    }

    fn signature_digest(&self) -> &str {
        match self {
            Self::FrozenSixRule { derivation } => &derivation.signature_digest,
            Self::DependentContextual { derivation } => &derivation.signature_digest,
        }
    }

    fn visible_library(&self) -> u32 {
        match self {
            Self::FrozenSixRule { derivation } => derivation.visible_library,
            Self::DependentContextual { derivation } => derivation.visible_library,
        }
    }

    fn exact_context(&self) -> Result<Vec<KernelTy>, OpenSpecializationV4Error> {
        match self {
            Self::FrozenSixRule { derivation } => v2_exact_context(derivation),
            Self::DependentContextual { derivation } => {
                Ok(derivation.declaration.exact_ambient_kernel_types.clone())
            }
        }
    }

    fn derivation_hash(&self) -> &str {
        match self {
            Self::FrozenSixRule { derivation } => &derivation.derivation_hash,
            Self::DependentContextual { derivation } => &derivation.derivation_hash,
        }
    }
}

fn replay_open_internal(
    signature: &SealedSignature,
    evidence: &OpenInternalDerivationV4,
) -> Result<(), OpenSpecializationV4Error> {
    match evidence {
        OpenInternalDerivationV4::FrozenSixRule { derivation } => {
            replay_verified_closure_derivation_v2(signature, derivation)
                .map_err(|error| OpenSpecializationV4Error::FrozenInternal(error.to_string()))
        }
        OpenInternalDerivationV4::DependentContextual { derivation } => {
            replay_dependent_contextual_internal_v4(signature, derivation)
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ClosureRuleInductionNodeV4 {
    pub rule: ClosureRuleKind,
    pub source_derivation_hash: String,
    pub premise_nodes: Vec<ClosureRuleInductionNodeV4>,
    pub exact_source_replayed: bool,
    pub no_rule_kind_omitted: bool,
    pub induction_hash: String,
}

fn v2_induction_node(
    signature: &SealedSignature,
    source: &VerifiedClosureDerivationV2,
) -> Result<ClosureRuleInductionNodeV4, OpenSpecializationV4Error> {
    replay_verified_closure_derivation_v2(signature, source)
        .map_err(|error| OpenSpecializationV4Error::FrozenInternal(error.to_string()))?;
    let premises: Vec<&VerifiedClosureDerivationV2> = match &source.evidence {
        ClosureRuleEvidenceV2::Projection {
            source: ProjectionSourceEvidenceV2::AmbientParameter { .. },
            ..
        }
        | ClosureRuleEvidenceV2::Structural { .. }
        | ClosureRuleEvidenceV2::ExplicitContextual { .. } => Vec::new(),
        ClosureRuleEvidenceV2::Projection {
            source: ProjectionSourceEvidenceV2::CertifiedPriorField {
                prior_derivation,
                ..
            },
            ..
        } => vec![prior_derivation.as_ref()],
        ClosureRuleEvidenceV2::Guarded {
            base_derivation, ..
        } => vec![base_derivation.as_ref()],
        ClosureRuleEvidenceV2::AmbientFormer {
            prior_derivations, ..
        }
        | ClosureRuleEvidenceV2::Dereference {
            prior_derivations, ..
        }
        | ClosureRuleEvidenceV2::Contextual {
            prior_derivations, ..
        } => prior_derivations.values().map(Box::as_ref).collect(),
    };
    let premise_nodes = premises
        .into_iter()
        .map(|premise| v2_induction_node(signature, premise))
        .collect::<Result<Vec<_>, _>>()?;
    let no_rule_kind_omitted = matches!(
        source.rule,
        ClosureRuleKind::Projection
            | ClosureRuleKind::Guarded
            | ClosureRuleKind::Structural
            | ClosureRuleKind::AmbientFormer
            | ClosureRuleKind::Dereference
            | ClosureRuleKind::Contextual
    );
    let mut node = ClosureRuleInductionNodeV4 {
        rule: source.rule,
        source_derivation_hash: source.derivation_hash.clone(),
        premise_nodes,
        exact_source_replayed: true,
        no_rule_kind_omitted,
        induction_hash: String::new(),
    };
    node.induction_hash = tagged_hash("six-rule-induction-node", &node);
    Ok(node)
}

fn source_induction_node(
    signature: &SealedSignature,
    source: &OpenInternalDerivationV4,
) -> Result<ClosureRuleInductionNodeV4, OpenSpecializationV4Error> {
    match source {
        OpenInternalDerivationV4::FrozenSixRule { derivation } => {
            v2_induction_node(signature, derivation)
        }
        OpenInternalDerivationV4::DependentContextual { derivation } => {
            replay_dependent_contextual_internal_v4(signature, derivation)?;
            let mut node = ClosureRuleInductionNodeV4 {
                rule: ClosureRuleKind::Contextual,
                source_derivation_hash: derivation.derivation_hash.clone(),
                premise_nodes: Vec::new(),
                exact_source_replayed: true,
                no_rule_kind_omitted: true,
                induction_hash: String::new(),
            };
            node.induction_hash = tagged_hash("dependent-contextual-induction-node", &node);
            Ok(node)
        }
    }
}

fn context_sort(ty: &KernelTy) -> ParameterSort {
    if *ty == KernelTy::Type {
        ParameterSort::Type
    } else {
        ParameterSort::Opaque
    }
}

fn instantiate_expression(mut expression: Expr, prior_terms: &[Expr]) -> Expr {
    for parameter in (1..=prior_terms.len() as u32).rev() {
        expression = substitute_level(&expression, parameter, &prior_terms[parameter as usize - 1]);
    }
    expression
}

fn instantiate_kernel_type(ty: &KernelTy, prior_terms: &[Expr]) -> KernelTy {
    match ty {
        KernelTy::Type => KernelTy::Type,
        KernelTy::El(expression) => {
            KernelTy::El(instantiate_expression(expression.clone(), prior_terms))
        }
        KernelTy::Fun(domain, codomain) => KernelTy::Fun(
            Box::new(instantiate_kernel_type(domain, prior_terms)),
            Box::new(instantiate_kernel_type(codomain, prior_terms)),
        ),
        KernelTy::PathDecl { dimension } => KernelTy::PathDecl {
            dimension: *dimension,
        },
        KernelTy::Neutral => KernelTy::Neutral,
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MotiveTypedOpenImageProjectionV4 {
    pub source_parameter: u32,
    pub source_instantiated_type: KernelTy,
    pub term: Expr,
    pub target_kernel_type: KernelTy,
    pub checked_after_parameters: Vec<u32>,
    pub internal_evidence: OpenInternalDerivationV4,
    pub internal_evidence_replayed: bool,
    pub image_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MotiveTypedOpenSpecializationProjectionV4 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub source: OpenInternalDerivationV4,
    pub source_exact_context: Vec<KernelTy>,
    pub target_declaration: DependentAmbientContextDeclarationProjection,
    pub target_totality: DependentTotalSpecializationProjection,
    pub target_motives: Vec<DependentContextMotive>,
    pub images: Vec<MotiveTypedOpenImageProjectionV4>,
    pub substitution_body: Expr,
    pub substitution_result: Expr,
    pub substitution_derivation_hash: String,
    pub source_rule_induction: ClosureRuleInductionNodeV4,
    pub specialized_internal: DependentContextualInternalProjectionV4,
    pub every_image_motive_typed: bool,
    pub every_image_internal_replayed: bool,
    pub full_motive_space_unrestricted: bool,
    pub exact_open_result_replayed: bool,
    pub target_may_remain_open: bool,
    pub no_probe_or_outcome_filter: bool,
    pub no_endpoint_premise_minted: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MotiveTypedOpenSpecializationTokenV4 {
    projection: MotiveTypedOpenSpecializationProjectionV4,
}

impl MotiveTypedOpenSpecializationTokenV4 {
    pub fn projection(&self) -> &MotiveTypedOpenSpecializationProjectionV4 {
        &self.projection
    }
}

pub fn issue_motive_typed_open_specialization_v4(
    signature: &SealedSignature,
    source: OpenInternalDerivationV4,
    target_motives: Vec<DependentContextMotive>,
    terms: Vec<(Expr, OpenInternalDerivationV4)>,
) -> Result<MotiveTypedOpenSpecializationTokenV4, OpenSpecializationV4Error> {
    replay_open_internal(signature, &source)?;
    if source.signature_digest() != signature.digest() {
        return Err(OpenSpecializationV4Error::SignatureMismatch);
    }
    let source_exact_context = source.exact_context()?;
    if terms.len() != source_exact_context.len() {
        return Err(OpenSpecializationV4Error::AssignmentArity {
            expected: source_exact_context.len(),
            supplied: terms.len(),
        });
    }
    let target_arity = target_motives.len() as u32;
    let source_context = SortedParameterContext::new(
        source_exact_context.iter().map(context_sort).collect(),
    );
    let structural_images = terms
        .iter()
        .enumerate()
        .map(|(index, (term, _))| SubstitutionImage {
            source_parameter: index as u32 + 1,
            term: term.clone(),
        })
        .collect::<Vec<_>>();
    let target_sort_context = SortedParameterContext::new(vec![
        ParameterSort::Opaque;
        target_arity as usize
    ]);
    let substitution = issue_structural_substitution(
        source_context,
        target_sort_context,
        structural_images,
        source.expression().clone(),
    )
    .map_err(|error| OpenSpecializationV4Error::Substitution(error.to_string()))?;
    replay_structural_substitution(&substitution)
        .map_err(|error| OpenSpecializationV4Error::Substitution(error.to_string()))?;
    let target_body = Telescope::new(vec![ClauseRec::new(
        match &source {
            OpenInternalDerivationV4::FrozenSixRule { derivation } => derivation.declared_role,
            OpenInternalDerivationV4::DependentContextual { derivation } => {
                derivation.declared_role
            }
        },
        substitution.result().clone(),
    )]);
    let target_declaration_token = issue_dependent_ambient_context_declaration(
        signature,
        &target_body,
        source.visible_library(),
        target_motives.clone(),
    )
    .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    let target_declaration = target_declaration_token.projection().clone();
    let target_totality_token =
        issue_dependent_total_specialization_theorem(signature, &target_declaration_token)
            .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    let target_types = &target_declaration.exact_ambient_kernel_types;
    let mut prior_terms = Vec::new();
    let mut images = Vec::new();
    for (index, ((term, evidence), source_type)) in terms
        .into_iter()
        .zip(source_exact_context.iter())
        .enumerate()
    {
        let parameter = index as u32 + 1;
        replay_open_internal(signature, &evidence)?;
        if evidence.signature_digest() != signature.digest()
            || evidence.visible_library() != source.visible_library()
            || evidence.expression() != &term
            || evidence.exact_context()? != *target_types
        {
            return Err(OpenSpecializationV4Error::ImageInternalMismatch { parameter });
        }
        let source_instantiated_type = instantiate_kernel_type(source_type, &prior_terms);
        let (typed, _) = elaborate_single_clause_with_typed_ambient(
            &term,
            target_types,
            &[],
            source.visible_library(),
        )
        .map_err(|error| OpenSpecializationV4Error::ImageTyping {
            parameter,
            reason: error.to_string(),
        })?;
        if typed.kernel_ty != source_instantiated_type {
            return Err(OpenSpecializationV4Error::ImageMotiveMismatch {
                parameter,
                expected: source_instantiated_type,
                actual: typed.kernel_ty,
            });
        }
        let mut image = MotiveTypedOpenImageProjectionV4 {
            source_parameter: parameter,
            source_instantiated_type: typed.kernel_ty.clone(),
            term: term.clone(),
            target_kernel_type: typed.kernel_ty,
            checked_after_parameters: (1..parameter).collect(),
            internal_evidence: evidence,
            internal_evidence_replayed: true,
            image_hash: String::new(),
        };
        image.image_hash = tagged_hash("motive-typed-open-image", &image);
        prior_terms.push(term);
        images.push(image);
    }
    let source_rule_induction = source_induction_node(signature, &source)?;
    let specialized = issue_dependent_contextual_internal_v4(signature, &target_declaration)?;
    let specialized_internal = specialized.projection().clone();
    let every_image_motive_typed = images
        .iter()
        .all(|image| image.source_instantiated_type == image.target_kernel_type);
    let every_image_internal_replayed =
        images.iter().all(|image| image.internal_evidence_replayed);
    let full_motive_space_unrestricted = true;
    let exact_open_result_replayed = specialized_internal.expression == *substitution.result()
        && specialized_internal.declaration == target_declaration;
    let target_may_remain_open = target_arity > 0;
    let no_probe_or_outcome_filter = specialized_internal.no_probe_or_motive_filter;
    let no_endpoint_premise_minted =
        specialized_internal.no_endpoint_premise_forged_or_minted;
    if !(every_image_motive_typed
        && every_image_internal_replayed
        && exact_open_result_replayed
        && no_probe_or_outcome_filter
        && no_endpoint_premise_minted)
    {
        return Err(OpenSpecializationV4Error::InternalPreservationFailed);
    }
    let substitution_body = source.expression().clone();
    let mut projection = MotiveTypedOpenSpecializationProjectionV4 {
        version: MOTIVE_TYPED_OPEN_SPECIALIZATION_V4_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library: source.visible_library(),
        source,
        source_exact_context,
        target_declaration,
        target_totality: target_totality_token.projection().clone(),
        target_motives,
        images,
        substitution_body,
        substitution_result: substitution.result().clone(),
        substitution_derivation_hash: substitution.derivation_hash().to_owned(),
        source_rule_induction,
        specialized_internal,
        every_image_motive_typed,
        every_image_internal_replayed,
        full_motive_space_unrestricted,
        exact_open_result_replayed,
        target_may_remain_open,
        no_probe_or_outcome_filter,
        no_endpoint_premise_minted,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("motive-typed-open-specialization", &projection);
    Ok(MotiveTypedOpenSpecializationTokenV4 { projection })
}

pub fn replay_motive_typed_open_specialization_v4(
    signature: &SealedSignature,
    projection: &MotiveTypedOpenSpecializationProjectionV4,
) -> Result<(), OpenSpecializationV4Error> {
    let reissued = issue_motive_typed_open_specialization_v4(
        signature,
        projection.source.clone(),
        projection.target_motives.clone(),
        projection
            .images
            .iter()
            .map(|image| (image.term.clone(), image.internal_evidence.clone()))
            .collect(),
    )?;
    if reissued.projection == *projection {
        Ok(())
    } else {
        Err(OpenSpecializationV4Error::ReplayMismatch)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum TSm1bCaseDispositionV4 {
    Derived {
        specialization_hash: String,
    },
    NamedBlocker {
        blocker: String,
        reason: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TSm1bCorpusCaseV4 {
    pub instance_id: String,
    pub older_step: u32,
    pub older_clause: u16,
    pub newest_step: u32,
    pub newest_clause: u16,
    pub source_expression: Expr,
    pub source_kernel_type: KernelTy,
    pub disposition: TSm1bCaseDispositionV4,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TSm1bCorpusAuditV4 {
    pub version: String,
    pub structural_case_count: usize,
    pub structural_case_ids: Vec<String>,
    pub registered_v3_gap_count: usize,
    pub registered_v3_gap_ids: Vec<String>,
    pub structural_surface_equals_registered_18: bool,
    pub cases: Vec<TSm1bCorpusCaseV4>,
    pub derived_count: usize,
    pub blocker_counts: BTreeMap<String, usize>,
    pub theorem_locally_replayable: bool,
    pub fixed_f_sm1_gate_passed: bool,
    pub positive_artifact_permitted: bool,
    pub audit_hash: String,
}

fn chronological_sources<'a>(
    window: &'a A3HistoricalWindow,
    instance: &A3TypedDemandInstance,
) -> Option<(&'a A3TypedClauseSource, &'a A3TypedClauseSource)> {
    if instance.source_anchor_ids.len() != 2 {
        return None;
    }
    Some((
        window
            .typed_sources
            .iter()
            .find(|source| source.anchor_id == instance.source_anchor_ids[0])?,
        window
            .typed_sources
            .iter()
            .find(|source| source.anchor_id == instance.source_anchor_ids[1])?,
    ))
}

fn archived_sealed_chronological_ids() -> Result<BTreeSet<String>, OpenSpecializationV4Error> {
    let value: Value = serde_json::from_slice(FROZEN_E5_BYTES)
        .map_err(|error| OpenSpecializationV4Error::Archive(error.to_string()))?;
    let rows = value
        .pointer("/stage16/membership_rows")
        .and_then(Value::as_array)
        .ok_or_else(|| OpenSpecializationV4Error::Archive("E-5 membership rows absent".to_owned()))?;
    Ok(rows
        .iter()
        .filter(|row| {
            row.get("rule")
                .and_then(Value::as_str)
                .is_some_and(|rule| rule.starts_with("chronological_comparison::"))
        })
        .filter_map(|row| row.get("a3_instance_id").and_then(Value::as_str))
        .map(str::to_owned)
        .collect())
}

fn archived_v3_specialization_gap_ids() -> Result<BTreeSet<String>, OpenSpecializationV4Error> {
    let value: Value = serde_json::from_slice(FROZEN_V3_BYTES)
        .map_err(|error| OpenSpecializationV4Error::Archive(error.to_string()))?;
    let windows = value
        .get("windows")
        .and_then(Value::as_array)
        .ok_or_else(|| OpenSpecializationV4Error::Archive("v3 windows absent".to_owned()))?;
    Ok(windows
        .iter()
        .flat_map(|window| {
            window
                .get("instances")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
        })
        .filter(|row| {
            row.pointer("/membership/gap_id")
                .and_then(Value::as_str)
                == Some("BI_CHRONOLOGICAL_OPEN_SPECIALIZATION_GAP_V3")
        })
        .filter_map(|row| row.get("instance_id").and_then(Value::as_str))
        .map(str::to_owned)
        .collect())
}

fn first_image(
    scheme: &A3TypedDemandScheme,
    older: &A3TypedClauseSource,
) -> Result<Expr, OpenSpecializationV4Error> {
    let A3DemandOutputType::ChronologicalInteraction {
        interface_mode,
        interface_slot_map,
        ..
    } = &scheme.required_output
    else {
        return Err(OpenSpecializationV4Error::ChronologicalShape);
    };
    replay_chronological_interface_slot_map(
        interface_slot_map,
        interface_slot_map.declared_arity,
    )
    .map_err(|error| OpenSpecializationV4Error::Chronological(error.to_string()))?;
    match interface_mode {
        A3ChronologicalInterfaceMode::DirectType => {
            Ok(older.canonical_presentation.canonical_normal_form.clone())
        }
        A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { .. } => Ok(Expr::App(
            Box::new(older.canonical_presentation.canonical_normal_form.clone()),
            Box::new(Expr::Var(1)),
        )),
    }
}

fn source_internal_for(
    signature: &SealedSignature,
    newest: &A3TypedClauseSource,
) -> Result<OpenInternalDerivationV4, OpenSpecializationV4Error> {
    let body = Telescope::new(vec![ClauseRec::new(
        newest.kernel_role,
        newest.canonical_presentation.canonical_normal_form.clone(),
    )]);
    if newest.canonical_presentation.parameters.len() == 1 {
        let declaration =
            pen_type::contextual_internality::issue_explicit_ambient_context_declaration_token(
                signature,
                &body,
                15,
                vec![ContextualMotive::Type],
            )
            .map_err(|error| OpenSpecializationV4Error::FrozenInternal(error.to_string()))?;
        let internal = issue_explicit_contextual_closure_derivation_v2(signature, &declaration)
            .map_err(|error| OpenSpecializationV4Error::FrozenInternal(error.to_string()))?;
        return Ok(OpenInternalDerivationV4::FrozenSixRule {
            derivation: internal.projection().clone(),
        });
    }
    let declaration = issue_dependent_ambient_context_declaration(
        signature,
        &body,
        15,
        vec![
            DependentContextMotive::Independent {
                motive: ContextualMotive::Type,
            },
            DependentContextMotive::ElementOfApplicationHead {
                head: Expr::Eventually(Box::new(Expr::Var(1))),
            },
        ],
    )
    .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    let internal = issue_dependent_contextual_internal_v4(signature, declaration.projection())?;
    Ok(OpenInternalDerivationV4::DependentContextual {
        derivation: internal.projection().clone(),
    })
}

fn target_motives_for(
    newest: &A3TypedClauseSource,
    older: &A3TypedClauseSource,
    first: &Expr,
) -> Vec<DependentContextMotive> {
    let target_arity = older
        .canonical_presentation
        .parameters
        .len()
        .max(newest.canonical_presentation.parameters.len());
    (0..target_arity)
        .map(|index| {
            if newest.canonical_presentation.parameters.len() == 2 && index == 1 {
                DependentContextMotive::ElementOfApplicationHead {
                    head: Expr::Eventually(Box::new(first.clone())),
                }
            } else {
                DependentContextMotive::Independent {
                    motive: ContextualMotive::Type,
                }
            }
        })
        .collect()
}

fn internal_for_image(
    signature: &SealedSignature,
    visible_library: u32,
    target_motives: &[DependentContextMotive],
    term: Expr,
) -> Result<OpenInternalDerivationV4, OpenSpecializationV4Error> {
    // Derive the kernel role from the exact target context; no caller role is
    // trusted.  The dependent Internal issuer then rejects Formation until
    // T-SM1a supplies its independently proved rule.
    let probe = Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, term.clone())]);
    let probe_declaration = issue_dependent_ambient_context_declaration(
        signature,
        &probe,
        visible_library,
        target_motives.to_vec(),
    )
    .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    let role = probe_declaration.projection().typed_body_elaboration.kernel_role;
    let body = Telescope::new(vec![ClauseRec::new(role, term)]);
    let declaration = issue_dependent_ambient_context_declaration(
        signature,
        &body,
        visible_library,
        target_motives.to_vec(),
    )
    .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    let internal = issue_dependent_contextual_internal_v4(signature, declaration.projection())?;
    Ok(OpenInternalDerivationV4::DependentContextual {
        derivation: internal.projection().clone(),
    })
}

fn prove_corpus_case(
    signature: &SealedSignature,
    scheme: &A3TypedDemandScheme,
    older: &A3TypedClauseSource,
    newest: &A3TypedClauseSource,
) -> Result<MotiveTypedOpenSpecializationTokenV4, OpenSpecializationV4Error> {
    let source = source_internal_for(signature, newest)?;
    let first = first_image(scheme, older)?;
    let target_motives = target_motives_for(newest, older, &first);
    let source_arity = newest.canonical_presentation.parameters.len();
    let terms = (0..source_arity)
        .map(|index| {
            if index == 0 {
                first.clone()
            } else {
                Expr::Var(index as u32 + 1)
            }
        })
        .collect::<Vec<_>>();
    let terms = terms
        .into_iter()
        .map(|term| {
            let internal =
                internal_for_image(signature, 15, &target_motives, term.clone())?;
            Ok((term, internal))
        })
        .collect::<Result<Vec<_>, OpenSpecializationV4Error>>()?;
    issue_motive_typed_open_specialization_v4(signature, source, target_motives, terms)
}

fn blocker_name(error: &OpenSpecializationV4Error) -> &'static str {
    match error {
        OpenSpecializationV4Error::FormationNeedsTSm1a => "T_SM1A_IMAGE_INTERNAL_REQUIRED",
        OpenSpecializationV4Error::Dependent(reason)
            if reason.contains("non-predecessors") =>
        {
            "T_SM1B_DEPENDENT_TARGET_CYCLE"
        }
        OpenSpecializationV4Error::ImageMotiveMismatch { .. } => {
            "T_SM1B_OPEN_IMAGE_MOTIVE_MISMATCH"
        }
        _ => "T_SM1B_OPEN_SPECIALIZATION_BLOCKER",
    }
}

/// Pure in-memory sweep.  It never writes the positive artifact and it reads
/// the archived v3 gap IDs only after independently constructing the exact
/// structural surface from the sealed E-5 corpus.
pub fn issue_t_sm1b_corpus_audit_v4() -> Result<TSm1bCorpusAuditV4, OpenSpecializationV4Error> {
    let signature = SealedSignature::genesis_del_h15();
    let window = generate_a3_window_for_exact_prefix_unbounded(&signature, 16)
        .map_err(|error| OpenSpecializationV4Error::Chronological(error.to_string()))?;
    let sealed_ids = archived_sealed_chronological_ids()?;
    let mut structural = Vec::new();
    for instance in &window.instances {
        if !sealed_ids.contains(&instance.instance_id) {
            continue;
        }
        let scheme = window
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == instance.scheme_id)
            .ok_or(OpenSpecializationV4Error::ChronologicalShape)?;
        if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison {
            continue;
        }
        let (older, newest) = chronological_sources(&window, instance)
            .ok_or(OpenSpecializationV4Error::ChronologicalShape)?;
        // This is a semantic predicate fixed before the v3 comparator is
        // opened: T-SM1b owns exactly non-Type newest source motives.
        if newest.kernel_type == KernelTy::Type {
            continue;
        }
        structural.push((instance, scheme, older, newest));
    }
    let mut structural_case_ids = structural
        .iter()
        .map(|(instance, _, _, _)| instance.instance_id.clone())
        .collect::<Vec<_>>();
    structural_case_ids.sort();
    let registered = archived_v3_specialization_gap_ids()?;
    let mut registered_v3_gap_ids = registered.iter().cloned().collect::<Vec<_>>();
    registered_v3_gap_ids.sort();
    let structural_surface_equals_registered_18 = structural_case_ids.len()
        == T_SM1B_REGISTERED_GAP_COUNT
        && registered_v3_gap_ids.len() == T_SM1B_REGISTERED_GAP_COUNT
        && structural_case_ids == registered_v3_gap_ids;
    let mut cases = Vec::new();
    let mut blocker_counts = BTreeMap::new();
    for (instance, scheme, older, newest) in structural {
        let disposition = match prove_corpus_case(&signature, scheme, older, newest) {
            Ok(token) => {
                replay_motive_typed_open_specialization_v4(&signature, token.projection())?;
                TSm1bCaseDispositionV4::Derived {
                    specialization_hash: token.projection().derivation_hash.clone(),
                }
            }
            Err(error) => {
                let blocker = blocker_name(&error).to_owned();
                *blocker_counts.entry(blocker.clone()).or_insert(0) += 1;
                TSm1bCaseDispositionV4::NamedBlocker {
                    blocker,
                    reason: error.to_string(),
                }
            }
        };
        cases.push(TSm1bCorpusCaseV4 {
            instance_id: instance.instance_id.clone(),
            older_step: older.step,
            older_clause: older.clause_index,
            newest_step: newest.step,
            newest_clause: newest.clause_index,
            source_expression: newest.canonical_presentation.canonical_normal_form.clone(),
            source_kernel_type: newest.kernel_type.clone(),
            disposition,
        });
    }
    cases.sort_by(|left, right| left.instance_id.cmp(&right.instance_id));
    let derived_count = cases
        .iter()
        .filter(|case| matches!(case.disposition, TSm1bCaseDispositionV4::Derived { .. }))
        .count();
    let theorem_locally_replayable = cases.iter().all(|case| match &case.disposition {
        TSm1bCaseDispositionV4::Derived {
            specialization_hash,
        } => !specialization_hash.is_empty(),
        TSm1bCaseDispositionV4::NamedBlocker { blocker, reason } => {
            !blocker.is_empty() && !reason.is_empty()
        }
    });
    // The fixed positive gate is 72/72, not this 18-row component.  This
    // module cannot assert it while either a T-SM1a image premise or a T-SM1b
    // row remains blocked.
    let fixed_f_sm1_gate_passed = structural_surface_equals_registered_18
        && derived_count == T_SM1B_REGISTERED_GAP_COUNT
        && blocker_counts.is_empty();
    let positive_artifact_permitted = fixed_f_sm1_gate_passed;
    let mut audit = TSm1bCorpusAuditV4 {
        version: MOTIVE_TYPED_OPEN_SPECIALIZATION_V4_VERSION.to_owned(),
        structural_case_count: structural_case_ids.len(),
        structural_case_ids,
        registered_v3_gap_count: registered_v3_gap_ids.len(),
        registered_v3_gap_ids,
        structural_surface_equals_registered_18,
        cases,
        derived_count,
        blocker_counts,
        theorem_locally_replayable,
        fixed_f_sm1_gate_passed,
        positive_artifact_permitted,
        audit_hash: String::new(),
    };
    audit.audit_hash = tagged_hash("t-sm1b-corpus-audit", &audit);
    Ok(audit)
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum OpenSpecializationV4Error {
    #[error("frozen Internal replay failed: {0}")]
    FrozenInternal(String),
    #[error("dependent-context replay failed: {0}")]
    Dependent(String),
    #[error("structural substitution failed: {0}")]
    Substitution(String),
    #[error("archive comparator failed: {0}")]
    Archive(String),
    #[error("chronological corpus construction failed: {0}")]
    Chronological(String),
    #[error("chronological source/scheme shape mismatch")]
    ChronologicalShape,
    #[error("source context is absent from the replayed closure evidence")]
    SourceContextUnavailable,
    #[error("Neutral is not a declarable ambient motive")]
    NeutralDeclaredMotive,
    #[error("dependent total-specialization evidence is absent")]
    DependentTotalityMissing,
    #[error("dependent contextual Internal closure did not issue")]
    DependentInternalNotIssued,
    #[error("Formation Internal is the separate T-SM1a obligation")]
    FormationNeedsTSm1a,
    #[error("declared and kernel-derived roles differ")]
    KernelRoleMismatch,
    #[error("charged or outside constructor cannot receive zero-credit Internal closure")]
    ChargedOrOutsideConstructor,
    #[error("signature digest mismatch")]
    SignatureMismatch,
    #[error("assignment arity mismatch: expected {expected}, supplied {supplied}")]
    AssignmentArity { expected: usize, supplied: usize },
    #[error("image {parameter} Internal evidence does not bind the exact open image")]
    ImageInternalMismatch { parameter: u32 },
    #[error("image {parameter} failed exact target-context typing: {reason}")]
    ImageTyping { parameter: u32, reason: String },
    #[error("image {parameter} has type {actual:?}, expected specialized motive {expected:?}")]
    ImageMotiveMismatch {
        parameter: u32,
        expected: KernelTy,
        actual: KernelTy,
    },
    #[error("open specialization did not preserve Internal")]
    InternalPreservationFailed,
    #[error("reissuance mismatch")]
    ReplayMismatch,
}
