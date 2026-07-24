//! T-SM1b: motive-typed specialization into an open dependent context.
//!
//! This is an additive successor.  The frozen v2 closed theorem and the v3
//! chronological sweep remain byte-for-byte inputs.  In particular this
//! module never treats a typing derivation, scope check, or digest as an
//! `Internal` proof.  A source and every substitution image must replay
//! either through the frozen six-rule closure calculus or through the
//! adopted dependent-context hypothetical-judgment rule below.

use crate::t_sm1a_contextual_formation_v4::{
    ChronologicalImageInternalProjectionV4, issue_chronological_image_internal_v4,
    issue_exact_public_family_v4, replay_chronological_image_internal_v4,
    replay_exact_public_family_v4,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3ChronologicalInterfaceMode, A3DemandOutputType, A3HistoricalWindow, A3RuleConstructor,
    A3TypedClauseSource, A3TypedDemandInstance, A3TypedDemandScheme,
    generate_a3_window_for_exact_prefix_unbounded, replay_chronological_interface_slot_map,
};
use pen_type::contextual_internality::{AmbientHypothesisProjection, ContextualMotive};
use pen_type::dependent_context::{
    DependentAmbientContextDeclarationProjection, DependentContextMotive,
    DependentTotalSpecializationProjection, issue_dependent_ambient_context_declaration,
    issue_dependent_total_specialization_theorem, replay_dependent_ambient_context_declaration,
    replay_dependent_total_specialization_theorem, replay_sealed_prior_clause_type_reference,
};
use pen_type::elaborate::{
    DerivationNode, KernelTy, SealedSignature, elaborate_single_clause_with_typed_ambient,
};
use pen_type::equality::{EqualityWitness, univalent_equality};
use pen_type::motive_parametric_coherence::ClosureRuleKind;
use pen_type::motive_parametric_coherence_v2::{
    ClosureRuleEvidenceV2, KernelTyProjectionV2, ProjectionSourceEvidenceV2,
    VerifiedClosureDerivationV2, issue_explicit_contextual_closure_derivation_v2,
    replay_verified_closure_derivation_v2,
};
use pen_type::normalize::{normalize, substitute_level, whnf};
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

fn kernel_ty_from_projection(projection: &KernelTyProjectionV2) -> KernelTy {
    match projection {
        KernelTyProjectionV2::Type => KernelTy::Type,
        KernelTyProjectionV2::Element(expression) => KernelTy::El(expression.clone()),
        KernelTyProjectionV2::Function { domain, codomain } => KernelTy::Fun(
            Box::new(kernel_ty_from_projection(domain)),
            Box::new(kernel_ty_from_projection(codomain)),
        ),
        KernelTyProjectionV2::PathDeclaration { dimension } => KernelTy::PathDecl {
            dimension: *dimension,
        },
        KernelTyProjectionV2::Neutral => KernelTy::Neutral,
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExpressionGrammarInductionNodeV4 {
    pub constructor: String,
    pub expression: Expr,
    pub children: Vec<ExpressionGrammarInductionNodeV4>,
    pub induction_hash: String,
}

fn expression_grammar_induction(expression: &Expr) -> ExpressionGrammarInductionNodeV4 {
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
    let children = children
        .into_iter()
        .map(expression_grammar_induction)
        .collect::<Vec<_>>();
    let mut node = ExpressionGrammarInductionNodeV4 {
        constructor: name.to_owned(),
        expression: expression.clone(),
        children,
        induction_hash: String::new(),
    };
    node.induction_hash = tagged_hash("expression-grammar-induction", &node);
    node
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "motive_constructor")]
pub enum MotiveGrammarInductionNodeV4 {
    Type,
    Element {
        expression: ExpressionGrammarInductionNodeV4,
    },
    Function {
        domain: Box<MotiveGrammarInductionNodeV4>,
        codomain: Box<MotiveGrammarInductionNodeV4>,
    },
    Neutral,
    DependentIndependent {
        motive: Box<MotiveGrammarInductionNodeV4>,
    },
    ElementOfApplicationHead {
        head: ExpressionGrammarInductionNodeV4,
    },
    OpaquePriorClause {
        reference_hash: String,
        reference_replayed: bool,
    },
}

fn contextual_motive_induction(motive: &ContextualMotive) -> MotiveGrammarInductionNodeV4 {
    match motive {
        ContextualMotive::Type => MotiveGrammarInductionNodeV4::Type,
        ContextualMotive::Element(expression) => MotiveGrammarInductionNodeV4::Element {
            expression: expression_grammar_induction(expression),
        },
        ContextualMotive::Function { domain, codomain } => MotiveGrammarInductionNodeV4::Function {
            domain: Box::new(contextual_motive_induction(domain)),
            codomain: Box::new(contextual_motive_induction(codomain)),
        },
        ContextualMotive::Neutral => MotiveGrammarInductionNodeV4::Neutral,
    }
}

fn dependent_motive_induction(
    signature: &SealedSignature,
    motive: &DependentContextMotive,
) -> Result<MotiveGrammarInductionNodeV4, OpenSpecializationV4Error> {
    match motive {
        DependentContextMotive::Independent { motive } => {
            Ok(MotiveGrammarInductionNodeV4::DependentIndependent {
                motive: Box::new(contextual_motive_induction(motive)),
            })
        }
        DependentContextMotive::ElementOfApplicationHead { head } => {
            Ok(MotiveGrammarInductionNodeV4::ElementOfApplicationHead {
                head: expression_grammar_induction(head),
            })
        }
        DependentContextMotive::OpaquePriorClause { reference } => {
            replay_sealed_prior_clause_type_reference(signature, reference)
                .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
            Ok(MotiveGrammarInductionNodeV4::OpaquePriorClause {
                reference_hash: reference.reference_hash.clone(),
                reference_replayed: true,
            })
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct KernelRuleInductionNodeV4 {
    pub expression: Expr,
    pub derivation: DerivationNode,
    pub constructor: String,
    pub premises: Vec<KernelRuleInductionNodeV4>,
    pub beta_reduct: Option<Expr>,
    pub induction_hash: String,
}

fn exact_child_count(
    derivation: &DerivationNode,
    expected: usize,
) -> Result<(), OpenSpecializationV4Error> {
    if derivation.children.len() == expected {
        Ok(())
    } else {
        Err(OpenSpecializationV4Error::KernelRuleShape {
            rule: derivation.rule.clone(),
            expected,
            actual: derivation.children.len(),
        })
    }
}

fn kernel_rule_induction(
    expression: &Expr,
    derivation: &DerivationNode,
    ambient_arity: u32,
    local_depth: u32,
) -> Result<KernelRuleInductionNodeV4, OpenSpecializationV4Error> {
    let mut beta_reduct = None;
    let (constructor, child_specs): (&str, Vec<(&Expr, &DerivationNode, u32)>) = match expression {
        Expr::Univ if derivation.rule == "univ-form" => {
            exact_child_count(derivation, 0)?;
            ("Univ", Vec::new())
        }
        Expr::Var(_)
            if derivation.rule.starts_with("ambient-param-")
                || derivation.rule.starts_with("local-var-") =>
        {
            exact_child_count(derivation, 0)?;
            ("Var", Vec::new())
        }
        Expr::Lib(_) if derivation.rule == "library-constant" => {
            exact_child_count(derivation, 0)?;
            ("Lib", Vec::new())
        }
        Expr::Lam(body) if derivation.rule == "lam-intro" => {
            exact_child_count(derivation, 1)?;
            (
                "Lam",
                vec![(body, &derivation.children[0], local_depth + 1)],
            )
        }
        Expr::Pi(domain, codomain) if derivation.rule == "pi-form" => {
            exact_child_count(derivation, 2)?;
            (
                "Pi",
                vec![
                    (domain, &derivation.children[0], local_depth),
                    (codomain, &derivation.children[1], local_depth + 1),
                ],
            )
        }
        Expr::Sigma(domain, codomain) if derivation.rule == "sigma-form" => {
            exact_child_count(derivation, 2)?;
            (
                "Sigma",
                vec![
                    (domain, &derivation.children[0], local_depth),
                    (codomain, &derivation.children[1], local_depth + 1),
                ],
            )
        }
        Expr::App(function, argument)
            if matches!(
                derivation.rule.as_str(),
                "app-beta" | "univ-app-form" | "app-fun" | "app-el-pi" | "app-stuck"
            ) =>
        {
            let expected = if derivation.rule == "app-beta" { 3 } else { 2 };
            exact_child_count(derivation, expected)?;
            let mut specs = vec![
                (function.as_ref(), &derivation.children[0], local_depth),
                (argument.as_ref(), &derivation.children[1], local_depth),
            ];
            if derivation.rule == "app-beta" {
                let scope_len = ambient_arity + local_depth;
                let normalized = whnf(function, scope_len, 512)
                    .map_err(|error| OpenSpecializationV4Error::KernelRule(error.to_string()))?;
                let Expr::Lam(body) = normalized.expr else {
                    return Err(OpenSpecializationV4Error::KernelRule(
                        "app-beta head does not replay to lambda".to_owned(),
                    ));
                };
                let reduced = substitute_level(&body, scope_len + 1, argument);
                beta_reduct = Some(reduced);
                specs.push((
                    beta_reduct.as_ref().expect("just set"),
                    &derivation.children[2],
                    local_depth,
                ));
            }
            ("App", specs)
        }
        Expr::Id(ty, left, right) if derivation.rule == "id-form" => {
            exact_child_count(derivation, 3)?;
            (
                "Id",
                vec![
                    (ty, &derivation.children[0], local_depth),
                    (left, &derivation.children[1], local_depth),
                    (right, &derivation.children[2], local_depth),
                ],
            )
        }
        Expr::Refl(inner) if derivation.rule == "refl-intro" => {
            exact_child_count(derivation, 1)?;
            ("Refl", vec![(inner, &derivation.children[0], local_depth)])
        }
        Expr::Susp(inner) if derivation.rule == "susp-form" => {
            exact_child_count(derivation, 1)?;
            ("Susp", vec![(inner, &derivation.children[0], local_depth)])
        }
        Expr::Trunc(inner) if derivation.rule == "trunc-form" => {
            exact_child_count(derivation, 1)?;
            ("Trunc", vec![(inner, &derivation.children[0], local_depth)])
        }
        Expr::Flat(inner) if derivation.rule == "flat-form" => {
            exact_child_count(derivation, 1)?;
            ("Flat", vec![(inner, &derivation.children[0], local_depth)])
        }
        Expr::Sharp(inner) if derivation.rule == "sharp-form" => {
            exact_child_count(derivation, 1)?;
            ("Sharp", vec![(inner, &derivation.children[0], local_depth)])
        }
        Expr::Disc(inner) if derivation.rule == "disc-form" => {
            exact_child_count(derivation, 1)?;
            ("Disc", vec![(inner, &derivation.children[0], local_depth)])
        }
        Expr::Shape(inner) if derivation.rule == "shape-form" => {
            exact_child_count(derivation, 1)?;
            ("Shape", vec![(inner, &derivation.children[0], local_depth)])
        }
        Expr::Next(inner) if derivation.rule == "next-form" => {
            exact_child_count(derivation, 1)?;
            ("Next", vec![(inner, &derivation.children[0], local_depth)])
        }
        Expr::Eventually(inner) if derivation.rule == "eventually-form" => {
            exact_child_count(derivation, 1)?;
            (
                "Eventually",
                vec![(inner, &derivation.children[0], local_depth)],
            )
        }
        Expr::PathCon(_) | Expr::Bang(_) | Expr::WhyNot(_) => {
            return Err(OpenSpecializationV4Error::ChargedOrOutsideConstructor);
        }
        _ => {
            return Err(OpenSpecializationV4Error::KernelRuleShape {
                rule: derivation.rule.clone(),
                expected: usize::MAX,
                actual: derivation.children.len(),
            });
        }
    };
    let premises = child_specs
        .into_iter()
        .map(|(child, proof, depth)| kernel_rule_induction(child, proof, ambient_arity, depth))
        .collect::<Result<Vec<_>, _>>()?;
    let mut node = KernelRuleInductionNodeV4 {
        expression: expression.clone(),
        derivation: derivation.clone(),
        constructor: constructor.to_owned(),
        premises,
        beta_reduct,
        induction_hash: String::new(),
    };
    node.induction_hash = tagged_hash("kernel-rule-induction", &node);
    Ok(node)
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
    pub exact_kernel_rule_induction: KernelRuleInductionNodeV4,
    pub declared_motive_induction: Vec<MotiveGrammarInductionNodeV4>,
    pub outcome_filtering_audit: (bool, bool),
    pub zero_charge_audit: (u32, u32, u32),
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
    replay_dependent_total_specialization_theorem(signature, declaration, totality.projection())
        .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    if !totality.projection().total_specialization_theorem_issued {
        return Err(OpenSpecializationV4Error::DependentTotalityMissing);
    }
    let exact_kernel_rule_induction = kernel_rule_induction(
        &declaration.expression,
        &declaration.typed_body_derivation,
        declaration.declared_arity,
        0,
    )?;
    let declared_motive_induction = declaration
        .hypotheses
        .iter()
        .map(|hypothesis| dependent_motive_induction(signature, &hypothesis.motive))
        .collect::<Result<Vec<_>, _>>()?;
    let outcome_filtering_audit = (
        declaration.no_outcome_filtering_used,
        totality.projection().no_assignment_outcome_filtering,
    );
    if outcome_filtering_audit != (true, true) {
        return Err(OpenSpecializationV4Error::OutcomeFilteringDetected);
    }
    let zero_charge_audit = (
        declaration.marginal_kappa,
        declaration.marginal_nu,
        declaration.anchors_minted,
    );
    if zero_charge_audit != (0, 0, 0) {
        return Err(OpenSpecializationV4Error::HypothesisChargeDetected);
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
        exact_kernel_rule_induction,
        declared_motive_induction,
        outcome_filtering_audit,
        zero_charge_audit,
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
pub struct EqualityTransportedInternalProjectionV4 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub target_motives: Vec<DependentContextMotive>,
    pub raw_expression: Expr,
    pub raw_declared_role: ClauseRole,
    pub raw_kernel_type: KernelTy,
    pub raw_normal_form: Expr,
    pub raw_typed_derivation: DerivationNode,
    pub normalization_steps: u32,
    pub raw_to_normal_equality: EqualityWitness,
    pub normalized_internal: DependentContextualInternalProjectionV4,
    pub equality_transport_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EqualityTransportedInternalTokenV4 {
    projection: EqualityTransportedInternalProjectionV4,
}

impl EqualityTransportedInternalTokenV4 {
    pub fn projection(&self) -> &EqualityTransportedInternalProjectionV4 {
        &self.projection
    }
}

/// Deterministic univalent transport: every specialized result is normalized
/// first, its normalized dependent `Internal` derivation is replayed, and the
/// raw registered substitution term is retained through exact equality.  This
/// is not an outcome-selected fallback; it is the single target rule for all
/// open specializations.
pub fn issue_equality_transported_internal_v4(
    signature: &SealedSignature,
    visible_library: u32,
    target_motives: Vec<DependentContextMotive>,
    raw_expression: Expr,
    raw_declared_role: ClauseRole,
) -> Result<EqualityTransportedInternalTokenV4, OpenSpecializationV4Error> {
    let scope = target_motives.len() as u32;
    let normalized = normalize(&raw_expression, scope, 2048)
        .map_err(|error| OpenSpecializationV4Error::KernelRule(error.to_string()))?;
    let normalized_body = Telescope::new(vec![ClauseRec::new(
        raw_declared_role,
        normalized.expr.clone(),
    )]);
    let declaration = issue_dependent_ambient_context_declaration(
        signature,
        &normalized_body,
        visible_library,
        target_motives.clone(),
    )
    .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    let normalized_internal =
        issue_dependent_contextual_internal_v4(signature, declaration.projection())?;
    let target_types = &declaration.projection().exact_ambient_kernel_types;
    let (raw_typed, raw_typed_derivation) = elaborate_single_clause_with_typed_ambient(
        &raw_expression,
        target_types,
        &[],
        visible_library,
    )
    .map_err(|error| OpenSpecializationV4Error::KernelRule(error.to_string()))?;
    if raw_typed.kernel_role != raw_declared_role
        || raw_typed.kernel_ty != normalized_internal.projection().inferred_kernel_type
        || raw_typed.normal_form != normalized.expr
    {
        return Err(OpenSpecializationV4Error::EqualityTransportTypingMismatch);
    }
    let raw_to_normal_equality = univalent_equality(&raw_expression, &normalized.expr, scope, 2048)
        .map_err(|error| OpenSpecializationV4Error::KernelRule(error.to_string()))?;
    if !raw_to_normal_equality.equal {
        return Err(OpenSpecializationV4Error::EqualityTransportFailed);
    }
    let mut projection = EqualityTransportedInternalProjectionV4 {
        version: MOTIVE_TYPED_OPEN_SPECIALIZATION_V4_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        target_motives,
        raw_expression,
        raw_declared_role,
        raw_kernel_type: raw_typed.kernel_ty,
        raw_normal_form: normalized.expr,
        raw_typed_derivation,
        normalization_steps: normalized.steps,
        raw_to_normal_equality,
        normalized_internal: normalized_internal.projection().clone(),
        equality_transport_hash: String::new(),
    };
    projection.equality_transport_hash = tagged_hash("equality-transported-internal", &projection);
    Ok(EqualityTransportedInternalTokenV4 { projection })
}

pub fn replay_equality_transported_internal_v4(
    signature: &SealedSignature,
    projection: &EqualityTransportedInternalProjectionV4,
) -> Result<(), OpenSpecializationV4Error> {
    replay_dependent_contextual_internal_v4(signature, &projection.normalized_internal)?;
    let reissued = issue_equality_transported_internal_v4(
        signature,
        projection.visible_library,
        projection.target_motives.clone(),
        projection.raw_expression.clone(),
        projection.raw_declared_role,
    )?;
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
    ChronologicalImage {
        derivation: ChronologicalImageInternalProjectionV4,
    },
}

impl OpenInternalDerivationV4 {
    fn expression(&self) -> &Expr {
        match self {
            Self::FrozenSixRule { derivation } => &derivation.expression,
            Self::DependentContextual { derivation } => &derivation.expression,
            // T-SM1a's body derivation, rather than the wrapper fields, is
            // the actual Internal premise consumed by this theorem.
            Self::ChronologicalImage { derivation } => &derivation.body_derivation.expression,
        }
    }

    fn signature_digest(&self) -> &str {
        match self {
            Self::FrozenSixRule { derivation } => &derivation.signature_digest,
            Self::DependentContextual { derivation } => &derivation.signature_digest,
            Self::ChronologicalImage { derivation } => &derivation.body_derivation.signature_digest,
        }
    }

    fn visible_library(&self) -> u32 {
        match self {
            Self::FrozenSixRule { derivation } => derivation.visible_library,
            Self::DependentContextual { derivation } => derivation.visible_library,
            Self::ChronologicalImage { derivation } => derivation.body_derivation.visible_library,
        }
    }

    fn exact_context(&self) -> Result<Vec<KernelTy>, OpenSpecializationV4Error> {
        match self {
            Self::FrozenSixRule { derivation } => v2_exact_context(derivation),
            Self::DependentContextual { derivation } => {
                Ok(derivation.declaration.exact_ambient_kernel_types.clone())
            }
            Self::ChronologicalImage { derivation } => Ok(derivation
                .body_derivation
                .declared_context
                .iter()
                .map(kernel_ty_from_projection)
                .collect()),
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
        OpenInternalDerivationV4::ChronologicalImage { derivation } => {
            replay_chronological_image_internal_v4(signature, derivation)
                .map_err(|error| OpenSpecializationV4Error::FormationInternal(error.to_string()))?;
            let body = &derivation.body_derivation;
            if body.expression != derivation.raw_expression
                || body.normal_form != derivation.normal_form
                || body.declared_context != derivation.target_context
                || !body.internal_closure_issued
                || body.marginal_kappa != 0
                || body.marginal_nu != 0
                || body.anchors_minted != 0
            {
                return Err(OpenSpecializationV4Error::FormationInternal(
                    "T-SM1a wrapper does not expose its exact zero-credit body derivation"
                        .to_owned(),
                ));
            }
            Ok(())
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
            source:
                ProjectionSourceEvidenceV2::CertifiedPriorField {
                    prior_derivation, ..
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
        OpenInternalDerivationV4::ChronologicalImage { derivation } => {
            replay_chronological_image_internal_v4(signature, derivation)
                .map_err(|error| OpenSpecializationV4Error::FormationInternal(error.to_string()))?;
            let mut node = ClosureRuleInductionNodeV4 {
                rule: ClosureRuleKind::Contextual,
                source_derivation_hash: derivation.body_derivation.derivation_hash.clone(),
                premise_nodes: Vec::new(),
                exact_source_replayed: true,
                no_rule_kind_omitted: true,
                induction_hash: String::new(),
            };
            node.induction_hash = tagged_hash("chronological-image-induction-node", &node);
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
    /// Totality for the deterministic normal form.  The raw registered
    /// substitution is related to this term by the equality transport below;
    /// demanding raw totality here would reject harmless beta-redexes.
    pub normalized_target_totality: DependentTotalSpecializationProjection,
    pub target_motives: Vec<DependentContextMotive>,
    pub images: Vec<MotiveTypedOpenImageProjectionV4>,
    pub substitution_body: Expr,
    pub substitution_result: Expr,
    pub substitution_derivation_hash: String,
    pub source_rule_induction: ClosureRuleInductionNodeV4,
    pub target_motive_induction: Vec<MotiveGrammarInductionNodeV4>,
    pub specialized_internal: EqualityTransportedInternalProjectionV4,
    pub target_may_remain_open: bool,
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
    let source_context =
        SortedParameterContext::new(source_exact_context.iter().map(context_sort).collect());
    let structural_images = terms
        .iter()
        .enumerate()
        .map(|(index, (term, _))| SubstitutionImage {
            source_parameter: index as u32 + 1,
            term: term.clone(),
        })
        .collect::<Vec<_>>();
    let target_sort_context =
        SortedParameterContext::new(vec![ParameterSort::Opaque; target_arity as usize]);
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
            OpenInternalDerivationV4::ChronologicalImage { derivation } => {
                derivation.raw_expression_role
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
    let target_motive_induction = target_motives
        .iter()
        .map(|motive| dependent_motive_induction(signature, motive))
        .collect::<Result<Vec<_>, _>>()?;
    let raw_declared_role = target_declaration.declared_role;
    let specialized = issue_equality_transported_internal_v4(
        signature,
        source.visible_library(),
        target_motives.clone(),
        substitution.result().clone(),
        raw_declared_role,
    )?;
    let specialized_internal = specialized.projection().clone();
    if images
        .iter()
        .any(|image| image.source_instantiated_type != image.target_kernel_type)
    {
        return Err(OpenSpecializationV4Error::InternalPreservationFailed);
    }
    if images.iter().any(|image| !image.internal_evidence_replayed) {
        return Err(OpenSpecializationV4Error::InternalPreservationFailed);
    }
    if specialized_internal.raw_expression != *substitution.result()
        || specialized_internal.target_motives != target_motives
        || specialized_internal
            .normalized_internal
            .declaration
            .exact_ambient_kernel_types
            != target_declaration.exact_ambient_kernel_types
    {
        return Err(OpenSpecializationV4Error::InternalPreservationFailed);
    }
    let target_may_remain_open = target_arity > 0;
    let substitution_body = source.expression().clone();
    let mut projection = MotiveTypedOpenSpecializationProjectionV4 {
        version: MOTIVE_TYPED_OPEN_SPECIALIZATION_V4_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library: source.visible_library(),
        source,
        source_exact_context,
        target_declaration,
        normalized_target_totality: specialized_internal.normalized_internal.totality.clone(),
        target_motives,
        images,
        substitution_body,
        substitution_result: substitution.result().clone(),
        substitution_derivation_hash: substitution.derivation_hash().to_owned(),
        source_rule_induction,
        target_motive_induction,
        specialized_internal,
        target_may_remain_open,
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
    Derived { specialization_hash: String },
    NamedBlocker { blocker: String, reason: String },
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
pub struct DependencyGraphEdgeV4 {
    pub classifier_parameter: u32,
    pub referenced_parameter: u32,
    pub reference_is_strict_predecessor: bool,
    pub reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct LawPreservingAlternativeTrialV4 {
    pub name: String,
    pub preserves_exact_instance_id: bool,
    pub preserves_exact_slot_map: bool,
    pub accepted: bool,
    pub replayed_observation: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExactOpenDependencyCycleBlockerV4 {
    pub version: String,
    pub instance_id: String,
    pub older_source: A3TypedClauseSource,
    pub newest_source: A3TypedClauseSource,
    pub interface_mode: A3ChronologicalInterfaceMode,
    pub interface_assignments: Vec<(u32, u32)>,
    pub first_image: Expr,
    pub registered_renaming_forward: Vec<(u32, u32)>,
    pub first_image_target_parameters: Vec<u32>,
    pub source_second_motive: DependentContextMotive,
    pub source_second_motive_declaration_replayed: bool,
    pub source_internal_derivation_hash: String,
    pub exact_second_image: Expr,
    pub exact_identity_second_image: bool,
    pub substituted_second_classifier: KernelTy,
    pub first_image_contains_second_target_parameter: bool,
    pub substituted_classifier_contains_second_target_parameter: bool,
    pub dependency_graph: Vec<DependencyGraphEdgeV4>,
    pub predecessor_only_condition_violated: bool,
    pub dependency_inversion_proved: bool,
    pub exact_dependent_declaration_error: String,
    pub alternative_trials: Vec<LawPreservingAlternativeTrialV4>,
    pub no_law_preserving_alternative_succeeded: bool,
    pub fixed_f_sm1_gate_must_remain_false: bool,
    pub blocker_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TSm1bCorpusAuditV4 {
    pub version: String,
    pub structural_case_count: usize,
    pub structural_case_ids: Vec<String>,
    pub live_structural_surface_seal: String,
    pub archived_e5_chronological_count: usize,
    pub archived_e5_chronological_ids: Vec<String>,
    pub live_surface_ids_missing_from_archived_e5: Vec<String>,
    pub archived_e5_ids_outside_live_surface: Vec<String>,
    pub archived_e5_covers_live_surface: bool,
    pub registered_v3_gap_count: usize,
    pub registered_v3_gap_ids: Vec<String>,
    pub structural_surface_equals_registered_18: bool,
    pub cases: Vec<TSm1bCorpusCaseV4>,
    pub derived_count: usize,
    pub blocker_counts: BTreeMap<String, usize>,
    pub exact_dependency_cycle_blocker: Option<ExactOpenDependencyCycleBlockerV4>,
    pub theorem_locally_replayable: bool,
    pub t_sm1b_component_complete: bool,
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
        .ok_or_else(|| {
            OpenSpecializationV4Error::Archive("E-5 membership rows absent".to_owned())
        })?;
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
            row.pointer("/membership/gap_id").and_then(Value::as_str)
                == Some("BI_CHRONOLOGICAL_OPEN_SPECIALIZATION_GAP_V3")
        })
        .filter_map(|row| row.get("instance_id").and_then(Value::as_str))
        .map(str::to_owned)
        .collect())
}

type TSm1bLiveSurfaceRow<'a> = (
    &'a A3TypedDemandInstance,
    &'a A3TypedDemandScheme,
    &'a A3TypedClauseSource,
    &'a A3TypedClauseSource,
);

/// Construct the T-SM1b theorem domain from the live A3 window alone.
///
/// In particular, this function has no archive parameter and performs no
/// archive read.  Membership is determined by the operational rule kind and
/// the type of the newest source: T-SM1b owns chronological comparisons whose
/// newest source has a non-`Type` motive.  Frozen E-5 and v3 IDs are only
/// comparators for the already sealed result of this function.
fn live_t_sm1b_surface<'a>(
    window: &'a A3HistoricalWindow,
) -> Result<Vec<TSm1bLiveSurfaceRow<'a>>, OpenSpecializationV4Error> {
    let mut surface = Vec::new();
    for instance in &window.instances {
        let scheme = window
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == instance.scheme_id)
            .ok_or(OpenSpecializationV4Error::ChronologicalShape)?;
        if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison {
            continue;
        }
        let (older, newest) = chronological_sources(window, instance)
            .ok_or(OpenSpecializationV4Error::ChronologicalShape)?;
        if newest.kernel_type == KernelTy::Type {
            continue;
        }
        surface.push((instance, scheme, older, newest));
    }
    surface.sort_by(|left, right| left.0.instance_id.cmp(&right.0.instance_id));
    Ok(surface)
}

fn live_t_sm1b_surface_ids(surface: &[TSm1bLiveSurfaceRow<'_>]) -> Vec<String> {
    surface
        .iter()
        .map(|(instance, _, _, _)| instance.instance_id.clone())
        .collect()
}

fn seal_live_t_sm1b_surface(ids: &[String]) -> String {
    tagged_hash(
        "live-t-sm1b-structural-surface",
        &(
            "chronological_comparison",
            "newest_kernel_type_is_not_type",
            ids,
        ),
    )
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
    replay_chronological_interface_slot_map(interface_slot_map, interface_slot_map.declared_arity)
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
    visible_library: u32,
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
                visible_library,
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
        visible_library,
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
    let role = probe_declaration
        .projection()
        .typed_body_elaboration
        .kernel_role;
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
    visible_library: u32,
    scheme: &A3TypedDemandScheme,
    older: &A3TypedClauseSource,
    newest: &A3TypedClauseSource,
) -> Result<MotiveTypedOpenSpecializationTokenV4, OpenSpecializationV4Error> {
    let source = source_internal_for(signature, visible_library, newest)?;
    let first = first_image(scheme, older)?;
    let target_motives = target_motives_for(newest, older, &first);
    let context_probe = Telescope::new(vec![ClauseRec::new(
        ClauseRole::Introduction,
        first.clone(),
    )]);
    let target_context = issue_dependent_ambient_context_declaration(
        signature,
        &context_probe,
        visible_library,
        target_motives.clone(),
    )
    .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?
    .projection()
    .exact_ambient_kernel_types
    .clone();
    let A3DemandOutputType::ChronologicalInteraction { interface_mode, .. } =
        &scheme.required_output
    else {
        return Err(OpenSpecializationV4Error::ChronologicalShape);
    };
    let first_internal = issue_chronological_image_internal_v4(
        signature,
        visible_library,
        older,
        interface_mode.clone(),
        target_context,
    )
    .map_err(|error| OpenSpecializationV4Error::FormationInternal(error.to_string()))?;
    if first_internal.projection().raw_expression != first {
        return Err(OpenSpecializationV4Error::ImageInternalMismatch { parameter: 1 });
    }
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
        .enumerate()
        .map(|(index, term)| {
            let internal = if index == 0 {
                OpenInternalDerivationV4::ChronologicalImage {
                    derivation: first_internal.projection().clone(),
                }
            } else {
                internal_for_image(signature, visible_library, &target_motives, term.clone())?
            };
            Ok((term, internal))
        })
        .collect::<Result<Vec<_>, OpenSpecializationV4Error>>()?;
    issue_motive_typed_open_specialization_v4(signature, source, target_motives, terms)
}

/// Issue the adopted open/contextual chronological specialization for one
/// exact live A3 row.
///
/// This entry point is signature-parametric and reads no archived corpus or
/// enacted outcome. The instance/scheme/source join is checked before the
/// theorem is instantiated, so callers cannot transplant a positive
/// specialization between rows.
pub fn issue_chronological_open_specialization_for_instance_v4(
    signature: &SealedSignature,
    visible_library: u32,
    instance: &A3TypedDemandInstance,
    scheme: &A3TypedDemandScheme,
    older: &A3TypedClauseSource,
    newest: &A3TypedClauseSource,
) -> Result<MotiveTypedOpenSpecializationTokenV4, OpenSpecializationV4Error> {
    if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison
        || instance.scheme_id != scheme.scheme_id
        || instance.source_anchor_ids != vec![older.anchor_id.clone(), newest.anchor_id.clone()]
        || instance.source_family_keys
            != vec![
                older.canonical_family_key.clone(),
                newest.canonical_family_key.clone(),
            ]
        || older.step >= newest.step
        || !older.exported_public_clause
        || !newest.exported_public_clause
        || newest.kernel_type == KernelTy::Type
    {
        return Err(OpenSpecializationV4Error::ChronologicalShape);
    }
    let A3DemandOutputType::ChronologicalInteraction {
        older_family,
        older_type,
        newest_family,
        newest_type,
        ..
    } = &scheme.required_output
    else {
        return Err(OpenSpecializationV4Error::ChronologicalShape);
    };
    if older_family != &older.canonical_family_key
        || older_type != &older.kernel_type
        || newest_family != &newest.canonical_family_key
        || newest_type != &newest.kernel_type
    {
        return Err(OpenSpecializationV4Error::ChronologicalShape);
    }
    let older_public = issue_exact_public_family_v4(signature, visible_library, older)
        .map_err(|error| OpenSpecializationV4Error::FormationInternal(error.to_string()))?;
    replay_exact_public_family_v4(signature, &older_public)
        .map_err(|error| OpenSpecializationV4Error::FormationInternal(error.to_string()))?;
    let newest_public = issue_exact_public_family_v4(signature, visible_library, newest)
        .map_err(|error| OpenSpecializationV4Error::FormationInternal(error.to_string()))?;
    replay_exact_public_family_v4(signature, &newest_public)
        .map_err(|error| OpenSpecializationV4Error::FormationInternal(error.to_string()))?;
    prove_corpus_case(signature, visible_library, scheme, older, newest)
}

/// Reissue the exact live-row specialization and compare its complete
/// projection. No digest-only shortcut is accepted.
pub fn replay_chronological_open_specialization_for_instance_v4(
    signature: &SealedSignature,
    visible_library: u32,
    instance: &A3TypedDemandInstance,
    scheme: &A3TypedDemandScheme,
    older: &A3TypedClauseSource,
    newest: &A3TypedClauseSource,
    claimed: &MotiveTypedOpenSpecializationProjectionV4,
) -> Result<(), OpenSpecializationV4Error> {
    replay_motive_typed_open_specialization_v4(signature, claimed)?;
    let reissued = issue_chronological_open_specialization_for_instance_v4(
        signature,
        visible_library,
        instance,
        scheme,
        older,
        newest,
    )?;
    if reissued.projection() == claimed {
        Ok(())
    } else {
        Err(OpenSpecializationV4Error::ReplayMismatch)
    }
}

fn blocker_name(error: &OpenSpecializationV4Error) -> &'static str {
    match error {
        OpenSpecializationV4Error::FormationNeedsTSm1a => "T_SM1A_IMAGE_INTERNAL_REQUIRED",
        OpenSpecializationV4Error::Dependent(reason) if reason.contains("non-predecessors") => {
            "T_SM1B_DEPENDENT_TARGET_CYCLE"
        }
        OpenSpecializationV4Error::ImageMotiveMismatch { .. } => {
            "T_SM1B_OPEN_IMAGE_MOTIVE_MISMATCH"
        }
        _ => "T_SM1B_OPEN_SPECIALIZATION_BLOCKER",
    }
}

fn issue_exact_open_dependency_cycle_blocker_for_row_inner_v4(
    signature: &SealedSignature,
    visible_library: u32,
    instance: &A3TypedDemandInstance,
    scheme: &A3TypedDemandScheme,
    older: &A3TypedClauseSource,
    newest: &A3TypedClauseSource,
) -> Result<ExactOpenDependencyCycleBlockerV4, OpenSpecializationV4Error> {
    let live_window = generate_a3_window_for_exact_prefix_unbounded(signature, visible_library + 1)
        .map_err(|error| OpenSpecializationV4Error::Chronological(error.to_string()))?;
    let live_instance = live_window
        .instances
        .iter()
        .find(|live| live.instance_id == instance.instance_id)
        .ok_or(OpenSpecializationV4Error::ChronologicalShape)?;
    let live_scheme = live_window
        .schemes
        .iter()
        .find(|live| live.scheme_id == scheme.scheme_id)
        .ok_or(OpenSpecializationV4Error::ChronologicalShape)?;
    let live_older = live_window
        .typed_sources
        .iter()
        .find(|live| live.anchor_id == older.anchor_id)
        .ok_or(OpenSpecializationV4Error::ChronologicalShape)?;
    let live_newest = live_window
        .typed_sources
        .iter()
        .find(|live| live.anchor_id == newest.anchor_id)
        .ok_or(OpenSpecializationV4Error::ChronologicalShape)?;
    if live_instance != instance
        || live_scheme != scheme
        || live_older != older
        || live_newest != newest
    {
        return Err(OpenSpecializationV4Error::ChronologicalShape);
    }
    if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison
        || instance.scheme_id != scheme.scheme_id
        || instance.source_anchor_ids != vec![older.anchor_id.clone(), newest.anchor_id.clone()]
        || instance.source_family_keys
            != vec![
                older.canonical_family_key.clone(),
                newest.canonical_family_key.clone(),
            ]
        || older.step >= newest.step
        || !older.exported_public_clause
        || !newest.exported_public_clause
        || newest.kernel_type == KernelTy::Type
        || newest.canonical_presentation.parameters.len() != 2
    {
        return Err(OpenSpecializationV4Error::ChronologicalShape);
    }
    let first = first_image(scheme, older)?;
    let target_arity = older
        .canonical_presentation
        .parameters
        .len()
        .max(newest.canonical_presentation.parameters.len()) as u32;
    let dependencies = first
        .var_refs()
        .into_iter()
        .filter(|parameter| *parameter <= target_arity)
        .collect::<Vec<_>>();
    // This is the live canonical graph precondition, evaluated before any
    // attempted specialization: the classifier of target p2 contains target
    // p2 through the exact first realizer.
    if !dependencies.contains(&2) {
        return Err(OpenSpecializationV4Error::CycleSurfaceCardinality { found: 0 });
    }
    let A3DemandOutputType::ChronologicalInteraction {
        older_family,
        older_type,
        newest_family,
        newest_type,
        interface_mode,
        interface_slot_map,
        ..
    } = &scheme.required_output
    else {
        return Err(OpenSpecializationV4Error::ChronologicalShape);
    };
    if older_family != &older.canonical_family_key
        || older_type != &older.kernel_type
        || newest_family != &newest.canonical_family_key
        || newest_type != &newest.kernel_type
    {
        return Err(OpenSpecializationV4Error::ChronologicalShape);
    }
    let older_public = issue_exact_public_family_v4(signature, visible_library, older)
        .map_err(|error| OpenSpecializationV4Error::FormationInternal(error.to_string()))?;
    replay_exact_public_family_v4(signature, &older_public)
        .map_err(|error| OpenSpecializationV4Error::FormationInternal(error.to_string()))?;
    let newest_public = issue_exact_public_family_v4(signature, visible_library, newest)
        .map_err(|error| OpenSpecializationV4Error::FormationInternal(error.to_string()))?;
    replay_exact_public_family_v4(signature, &newest_public)
        .map_err(|error| OpenSpecializationV4Error::FormationInternal(error.to_string()))?;
    replay_chronological_interface_slot_map(interface_slot_map, interface_slot_map.declared_arity)
        .map_err(|error| OpenSpecializationV4Error::Chronological(error.to_string()))?;
    let interface_assignments = interface_slot_map
        .assignments
        .iter()
        .map(|assignment| (assignment.interface_slot, assignment.parameter))
        .collect::<Vec<_>>();
    let registered_renaming_forward = older.canonical_presentation.renaming.forward.clone();
    if !dependencies.iter().all(|parameter| {
        registered_renaming_forward
            .iter()
            .any(|(_, canonical)| canonical == parameter)
    }) {
        return Err(OpenSpecializationV4Error::RegisteredRenamingMismatch);
    }
    let dependency_graph = dependencies
        .iter()
        .map(|parameter| DependencyGraphEdgeV4 {
            classifier_parameter: 2,
            referenced_parameter: *parameter,
            reference_is_strict_predecessor: *parameter < 2,
            reason: format!(
                "target p2 classifier is El(Eventually(first_image)); first_image uses canonical target p{parameter}"
            ),
        })
        .collect::<Vec<_>>();

    let source_body = newest.canonical_presentation.canonical_normal_form.clone();
    let substitution = issue_structural_substitution(
        SortedParameterContext::all_type(2),
        SortedParameterContext::all_type(2),
        vec![
            SubstitutionImage {
                source_parameter: 1,
                term: first.clone(),
            },
            SubstitutionImage {
                source_parameter: 2,
                term: Expr::Var(2),
            },
        ],
        source_body,
    )
    .map_err(|error| OpenSpecializationV4Error::Substitution(error.to_string()))?;
    replay_structural_substitution(&substitution)
        .map_err(|error| OpenSpecializationV4Error::Substitution(error.to_string()))?;
    let specialized_body = Telescope::new(vec![ClauseRec::new(
        newest.kernel_role,
        substitution.result().clone(),
    )]);
    let exact_error = issue_dependent_ambient_context_declaration(
        signature,
        &specialized_body,
        visible_library,
        vec![
            DependentContextMotive::Independent {
                motive: ContextualMotive::Type,
            },
            DependentContextMotive::ElementOfApplicationHead {
                head: Expr::Eventually(Box::new(first.clone())),
            },
        ],
    )
    .expect_err("cycle criterion must be rejected by exact dependent declaration")
    .to_string();

    // Alternative 1: treating canonical Var(2) as a binder contradicts the
    // exact registered renaming (old free level 2 -> canonical parameter 2).
    let renaming_preserves_instance = true;
    let renaming_preserves_map = true;
    let renaming_semantically_succeeds = !registered_renaming_forward
        .iter()
        .any(|(_, canonical)| *canonical == 2);
    let renaming_trial = LawPreservingAlternativeTrialV4 {
        name: "reinterpret_after_exact_registered_renaming".to_owned(),
        preserves_exact_instance_id: renaming_preserves_instance,
        preserves_exact_slot_map: renaming_preserves_map,
        accepted: renaming_preserves_instance
            && renaming_preserves_map
            && renaming_semantically_succeeds,
        replayed_observation: format!(
            "renaming {:?} certifies canonical Var(2) as the second family parameter, not a binder",
            registered_renaming_forward
        ),
    };

    // Alternative 2: DirectType is fixed by the older classifier.  Forcing
    // a pointwise application does not yield a Type image and changes the
    // registered interface interpretation.
    let pointwise_term = Expr::App(Box::new(first.clone()), Box::new(Expr::Var(1)));
    let (pointwise_typed, _) = elaborate_single_clause_with_typed_ambient(
        &pointwise_term,
        &[KernelTy::Type, KernelTy::Type],
        &[],
        visible_library,
    )
    .map_err(|error| OpenSpecializationV4Error::KernelRule(error.to_string()))?;
    let pointwise_preserves_instance =
        !matches!(interface_mode, A3ChronologicalInterfaceMode::DirectType);
    let pointwise_preserves_map = true;
    let pointwise_semantically_succeeds = pointwise_typed.kernel_ty == KernelTy::Type;
    let pointwise_trial = LawPreservingAlternativeTrialV4 {
        name: "force_pointwise_on_direct_type_source".to_owned(),
        preserves_exact_instance_id: pointwise_preserves_instance,
        preserves_exact_slot_map: pointwise_preserves_map,
        accepted: pointwise_preserves_instance
            && pointwise_preserves_map
            && pointwise_semantically_succeeds,
        replayed_observation: format!(
            "registered mode is {:?}; forced pointwise term synthesizes {:?}, not Type",
            interface_mode, pointwise_typed.kernel_ty
        ),
    };

    // Alternative 3a: append a prerequisite element hypothesis.  The exact
    // p2 -> Var(2) map remains, so p2 is still Type and fails the specialized
    // Element(Eventually(first_image)) motive; the extra p3 is irrelevant.
    let appended_motives = vec![
        DependentContextMotive::Independent {
            motive: ContextualMotive::Type,
        },
        DependentContextMotive::Independent {
            motive: ContextualMotive::Type,
        },
        DependentContextMotive::ElementOfApplicationHead {
            head: Expr::Eventually(Box::new(first.clone())),
        },
    ];
    let appended_probe =
        Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, Expr::Var(2))]);
    let appended = issue_dependent_ambient_context_declaration(
        signature,
        &appended_probe,
        visible_library,
        appended_motives,
    )
    .map_err(|error| OpenSpecializationV4Error::Dependent(error.to_string()))?;
    let appended_var2_type = appended
        .projection()
        .typed_body_elaboration
        .kernel_ty
        .clone();
    let expected_var2_type = KernelTy::El(Expr::Eventually(Box::new(first.clone())));
    let appended_preserves_instance = true;
    let appended_preserves_map = interface_assignments == vec![(1, 1), (2, 2)];
    let appended_semantically_succeeds = appended_var2_type == expected_var2_type;
    let appended_trial = LawPreservingAlternativeTrialV4 {
        name: "append_prerequisite_without_changing_slot_map".to_owned(),
        preserves_exact_instance_id: appended_preserves_instance,
        preserves_exact_slot_map: appended_preserves_map,
        accepted: appended_preserves_instance
            && appended_preserves_map
            && appended_semantically_succeeds,
        replayed_observation: format!(
            "exact p2 image remains Var(2) at {:?}; required source motive is {:?}; appended p3 cannot answer p2",
            appended_var2_type, expected_var2_type
        ),
    };

    // Alternative 3b: insert the dependent hypothesis at p2.  This preserves
    // p2 -> Var(2), but its classifier still mentions p2 through first_image
    // and is rejected before any later prerequisite can be declared.
    let inserted_result = issue_dependent_ambient_context_declaration(
        signature,
        &specialized_body,
        visible_library,
        vec![
            DependentContextMotive::Independent {
                motive: ContextualMotive::Type,
            },
            DependentContextMotive::ElementOfApplicationHead {
                head: Expr::Eventually(Box::new(first.clone())),
            },
            DependentContextMotive::Independent {
                motive: ContextualMotive::Type,
            },
        ],
    );
    let inserted_semantically_succeeds = inserted_result.is_ok();
    let inserted_observation = inserted_result
        .map(|token| {
            format!(
                "unexpected declaration {}",
                token.projection().declaration_hash
            )
        })
        .unwrap_or_else(|error| error.to_string());
    let inserted_preserves_instance = true;
    let inserted_preserves_map = interface_assignments == vec![(1, 1), (2, 2)];
    let inserted_trial = LawPreservingAlternativeTrialV4 {
        name: "insert_prerequisite_at_exact_second_slot".to_owned(),
        preserves_exact_instance_id: inserted_preserves_instance,
        preserves_exact_slot_map: inserted_preserves_map,
        accepted: inserted_preserves_instance
            && inserted_preserves_map
            && inserted_semantically_succeeds,
        replayed_observation: inserted_observation,
    };
    let alternative_trials = vec![
        renaming_trial,
        pointwise_trial,
        appended_trial,
        inserted_trial,
    ];
    let no_law_preserving_alternative_succeeded =
        alternative_trials.iter().all(|trial| !trial.accepted);
    if !no_law_preserving_alternative_succeeded {
        return Err(OpenSpecializationV4Error::CycleAlternativeSucceeded);
    }
    let has_forbidden_self_edge = dependency_graph.iter().any(|edge| {
        edge.classifier_parameter == edge.referenced_parameter
            && !edge.reference_is_strict_predecessor
    });
    let source_second_motive = DependentContextMotive::ElementOfApplicationHead {
        head: Expr::Eventually(Box::new(Expr::Var(1))),
    };
    let source_internal = source_internal_for(signature, visible_library, newest)?;
    replay_open_internal(signature, &source_internal)?;
    let (source_second_motive_declaration_replayed, source_internal_derivation_hash) =
        match &source_internal {
            OpenInternalDerivationV4::DependentContextual { derivation } => (
                derivation
                    .declaration
                    .hypotheses
                    .get(1)
                    .is_some_and(|hypothesis| hypothesis.motive == source_second_motive),
                derivation.derivation_hash.clone(),
            ),
            _ => (false, String::new()),
        };
    let exact_second_image = Expr::Var(2);
    let exact_identity_second_image = interface_assignments.contains(&(2, 2));
    let substituted_classifier_body = Expr::Eventually(Box::new(first.clone()));
    let substituted_second_classifier = KernelTy::El(substituted_classifier_body.clone());
    let first_image_contains_second_target_parameter = dependencies.contains(&2);
    let substituted_classifier_contains_second_target_parameter =
        substituted_classifier_body.var_refs().contains(&2);
    let predecessor_only_condition_violated = dependency_graph.iter().any(|edge| {
        edge.classifier_parameter == 2
            && edge.referenced_parameter >= edge.classifier_parameter
            && !edge.reference_is_strict_predecessor
    });
    // Dependency inversion, not the finite alternative list, is the generic
    // obstruction: the exact identity tail maps source p2 to target p2 while
    // the substituted source motive for p2 contains target p2 itself.
    let dependency_inversion_proved = source_second_motive_declaration_replayed
        && exact_identity_second_image
        && exact_second_image == Expr::Var(2)
        && first_image_contains_second_target_parameter
        && substituted_classifier_contains_second_target_parameter
        && predecessor_only_condition_violated
        && has_forbidden_self_edge;
    let fixed_f_sm1_gate_must_remain_false = dependency_inversion_proved && !exact_error.is_empty();
    if !fixed_f_sm1_gate_must_remain_false {
        return Err(OpenSpecializationV4Error::CycleProofIncomplete);
    }
    let mut projection = ExactOpenDependencyCycleBlockerV4 {
        version: MOTIVE_TYPED_OPEN_SPECIALIZATION_V4_VERSION.to_owned(),
        instance_id: instance.instance_id.clone(),
        older_source: older.clone(),
        newest_source: newest.clone(),
        interface_mode: interface_mode.clone(),
        interface_assignments,
        first_image: first,
        registered_renaming_forward,
        first_image_target_parameters: dependencies,
        source_second_motive,
        source_second_motive_declaration_replayed,
        source_internal_derivation_hash,
        exact_second_image,
        exact_identity_second_image,
        substituted_second_classifier,
        first_image_contains_second_target_parameter,
        substituted_classifier_contains_second_target_parameter,
        dependency_graph,
        predecessor_only_condition_violated,
        dependency_inversion_proved,
        exact_dependent_declaration_error: exact_error,
        alternative_trials,
        no_law_preserving_alternative_succeeded,
        fixed_f_sm1_gate_must_remain_false,
        blocker_hash: String::new(),
    };
    projection.blocker_hash = tagged_hash("exact-open-dependency-cycle-blocker", &projection);
    Ok(projection)
}

/// Prove the exact dependency inversion for one supplied live chronological
/// row.
///
/// The row is admitted only when its own canonical dependency graph contains
/// the forbidden `p2 -> p2` edge (equivalently, the exact first realizer
/// contains target `Var(2)`).  This issuer does not enumerate a corpus, read an
/// archive, inspect a historical verdict, or select a row by trying a
/// specialization.
pub fn issue_exact_open_dependency_cycle_blocker_for_row_v4(
    signature: &SealedSignature,
    visible_library: u32,
    instance: &A3TypedDemandInstance,
    scheme: &A3TypedDemandScheme,
    older: &A3TypedClauseSource,
    newest: &A3TypedClauseSource,
) -> Result<ExactOpenDependencyCycleBlockerV4, OpenSpecializationV4Error> {
    issue_exact_open_dependency_cycle_blocker_for_row_inner_v4(
        signature,
        visible_library,
        instance,
        scheme,
        older,
        newest,
    )
}

/// Reissue an exact-row dependency-cycle proof under the supplied signature
/// and compare the complete projection.
pub fn replay_exact_open_dependency_cycle_blocker_for_row_v4(
    signature: &SealedSignature,
    visible_library: u32,
    instance: &A3TypedDemandInstance,
    scheme: &A3TypedDemandScheme,
    older: &A3TypedClauseSource,
    newest: &A3TypedClauseSource,
    projection: &ExactOpenDependencyCycleBlockerV4,
) -> Result<(), OpenSpecializationV4Error> {
    let reissued = issue_exact_open_dependency_cycle_blocker_for_row_v4(
        signature,
        visible_library,
        instance,
        scheme,
        older,
        newest,
    )?;
    if reissued == *projection {
        Ok(())
    } else {
        Err(OpenSpecializationV4Error::ReplayMismatch)
    }
}

/// Preserve the historical no-argument Genesis issuer by first locating the
/// unique row from the live A3 surface, then invoking the exact-row theorem.
pub fn issue_exact_open_dependency_cycle_blocker_v4()
-> Result<ExactOpenDependencyCycleBlockerV4, OpenSpecializationV4Error> {
    let signature = SealedSignature::genesis_del_h15();
    let window = generate_a3_window_for_exact_prefix_unbounded(&signature, 16)
        .map_err(|error| OpenSpecializationV4Error::Chronological(error.to_string()))?;
    let mut matches = Vec::new();
    for (instance, scheme, older, newest) in live_t_sm1b_surface(&window)? {
        if newest.canonical_presentation.parameters.len() != 2 {
            continue;
        }
        let first = first_image(scheme, older)?;
        if first.var_refs().contains(&2) {
            matches.push((
                instance.clone(),
                scheme.clone(),
                older.clone(),
                newest.clone(),
            ));
        }
    }
    if matches.len() != 1 {
        return Err(OpenSpecializationV4Error::CycleSurfaceCardinality {
            found: matches.len(),
        });
    }
    let (instance, scheme, older, newest) = matches.remove(0);
    issue_exact_open_dependency_cycle_blocker_for_row_v4(
        &signature, 15, &instance, &scheme, &older, &newest,
    )
}

pub fn replay_exact_open_dependency_cycle_blocker_v4(
    projection: &ExactOpenDependencyCycleBlockerV4,
) -> Result<(), OpenSpecializationV4Error> {
    let reissued = issue_exact_open_dependency_cycle_blocker_v4()?;
    if reissued == *projection {
        Ok(())
    } else {
        Err(OpenSpecializationV4Error::ReplayMismatch)
    }
}

/// Pure in-memory sweep.  It never writes the positive artifact.  The live A3
/// window and the operational T-SM1b predicate construct and seal the theorem
/// domain before either frozen comparator is read.
pub fn issue_t_sm1b_corpus_audit_v4() -> Result<TSm1bCorpusAuditV4, OpenSpecializationV4Error> {
    let signature = SealedSignature::genesis_del_h15();
    let window = generate_a3_window_for_exact_prefix_unbounded(&signature, 16)
        .map_err(|error| OpenSpecializationV4Error::Chronological(error.to_string()))?;
    let structural = live_t_sm1b_surface(&window)?;
    let structural_case_ids = live_t_sm1b_surface_ids(&structural);
    let live_structural_surface_seal = seal_live_t_sm1b_surface(&structural_case_ids);

    // Comparator reads start only after the live surface has a content seal.
    let archived_e5 = archived_sealed_chronological_ids()?;
    let archived_e5_chronological_ids = archived_e5.iter().cloned().collect::<Vec<_>>();
    let live_surface_id_set = structural_case_ids.iter().cloned().collect::<BTreeSet<_>>();
    let live_surface_ids_missing_from_archived_e5 = live_surface_id_set
        .difference(&archived_e5)
        .cloned()
        .collect::<Vec<_>>();
    let archived_e5_ids_outside_live_surface = archived_e5
        .difference(&live_surface_id_set)
        .cloned()
        .collect::<Vec<_>>();
    let archived_e5_covers_live_surface = live_surface_ids_missing_from_archived_e5.is_empty();
    let registered = archived_v3_specialization_gap_ids()?;
    let registered_v3_gap_ids = registered.iter().cloned().collect::<Vec<_>>();
    let structural_surface_equals_registered_18 = structural_case_ids.len()
        == T_SM1B_REGISTERED_GAP_COUNT
        && registered_v3_gap_ids.len() == T_SM1B_REGISTERED_GAP_COUNT
        && structural_case_ids == registered_v3_gap_ids;
    let mut cases = Vec::new();
    let mut blocker_counts = BTreeMap::new();
    for (instance, scheme, older, newest) in structural {
        let disposition = match prove_corpus_case(&signature, 15, scheme, older, newest) {
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
    let dependency_cycle_count = blocker_counts
        .get("T_SM1B_DEPENDENT_TARGET_CYCLE")
        .copied()
        .unwrap_or(0);
    let exact_dependency_cycle_blocker = if dependency_cycle_count == 1 {
        Some(issue_exact_open_dependency_cycle_blocker_v4()?)
    } else {
        None
    };
    if let Some(blocker) = &exact_dependency_cycle_blocker {
        replay_exact_open_dependency_cycle_blocker_v4(blocker)?;
    }
    let theorem_locally_replayable = cases.iter().all(|case| match &case.disposition {
        TSm1bCaseDispositionV4::Derived {
            specialization_hash,
        } => !specialization_hash.is_empty(),
        TSm1bCaseDispositionV4::NamedBlocker { blocker, reason } => {
            !blocker.is_empty() && !reason.is_empty()
        }
    });
    // This issuer owns only the 18-row T-SM1b component.  It never names or
    // authorizes the full 72/72 + 9/9 F-SM1 gate; that join belongs solely to
    // `chronological_slot_map_v4`.
    let t_sm1b_component_complete = archived_e5_covers_live_surface
        && structural_surface_equals_registered_18
        && derived_count == T_SM1B_REGISTERED_GAP_COUNT
        && blocker_counts.is_empty();
    let mut audit = TSm1bCorpusAuditV4 {
        version: MOTIVE_TYPED_OPEN_SPECIALIZATION_V4_VERSION.to_owned(),
        structural_case_count: structural_case_ids.len(),
        structural_case_ids,
        live_structural_surface_seal,
        archived_e5_chronological_count: archived_e5_chronological_ids.len(),
        archived_e5_chronological_ids,
        live_surface_ids_missing_from_archived_e5,
        archived_e5_ids_outside_live_surface,
        archived_e5_covers_live_surface,
        registered_v3_gap_count: registered_v3_gap_ids.len(),
        registered_v3_gap_ids,
        structural_surface_equals_registered_18,
        cases,
        derived_count,
        blocker_counts,
        exact_dependency_cycle_blocker,
        theorem_locally_replayable,
        t_sm1b_component_complete,
        audit_hash: String::new(),
    };
    audit.audit_hash = tagged_hash("t-sm1b-corpus-audit", &audit);
    Ok(audit)
}

/// Reconstruct the structural surface, every positive derivation, and the
/// exact negative witness.  Equality with a supplied projection is the replay
/// criterion; no archived verdict is accepted as proof.
pub fn replay_t_sm1b_corpus_audit_v4(
    projection: &TSm1bCorpusAuditV4,
) -> Result<(), OpenSpecializationV4Error> {
    let reissued = issue_t_sm1b_corpus_audit_v4()?;
    if reissued == *projection {
        Ok(())
    } else {
        Err(OpenSpecializationV4Error::ReplayMismatch)
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum OpenSpecializationV4Error {
    #[error("frozen Internal replay failed: {0}")]
    FrozenInternal(String),
    #[error("T-SM1a chronological-image Internal replay failed: {0}")]
    FormationInternal(String),
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
    #[error("exact dependency-cycle surface has {found} cases; expected one")]
    CycleSurfaceCardinality { found: usize },
    #[error("registered canonical renaming does not account for every target dependency")]
    RegisteredRenamingMismatch,
    #[error("a law-preserving dependency-cycle alternative unexpectedly succeeded")]
    CycleAlternativeSucceeded,
    #[error("dependency-cycle proof did not derive the fixed-gate consequence")]
    CycleProofIncomplete,
    #[error("source context is absent from the replayed closure evidence")]
    SourceContextUnavailable,
    #[error("Neutral is not a declarable ambient motive")]
    NeutralDeclaredMotive,
    #[error("dependent total-specialization evidence is absent")]
    DependentTotalityMissing,
    #[error("dependent contextual rule detected assignment-outcome filtering")]
    OutcomeFilteringDetected,
    #[error("dependent hypotheses or endpoint premises carried nonzero charge")]
    HypothesisChargeDetected,
    #[error("Formation Internal is the separate T-SM1a obligation")]
    FormationNeedsTSm1a,
    #[error("declared and kernel-derived roles differ")]
    KernelRoleMismatch,
    #[error("charged or outside constructor cannot receive zero-credit Internal closure")]
    ChargedOrOutsideConstructor,
    #[error("kernel rule replay failed: {0}")]
    KernelRule(String),
    #[error(
        "kernel rule {rule} has {actual} premises; exact constructor replay expected {expected}"
    )]
    KernelRuleShape {
        rule: String,
        expected: usize,
        actual: usize,
    },
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
    #[error("equality transport changed the specialized term's kernel type, role, or normal form")]
    EqualityTransportTypingMismatch,
    #[error("univalent equality failed between the raw specialization and its normal form")]
    EqualityTransportFailed,
    #[error("reissuance mismatch")]
    ReplayMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_type::dependent_context::issue_sealed_prior_clause_type_reference;

    fn dependent_internal(
        signature: &SealedSignature,
        motives: &[DependentContextMotive],
        expression: Expr,
    ) -> OpenInternalDerivationV4 {
        internal_for_image(signature, 15, motives, expression)
            .expect("synthetic dependent Internal must issue")
    }

    fn synthetic_specialization() -> MotiveTypedOpenSpecializationTokenV4 {
        let signature = SealedSignature::genesis_del_h15();
        let motives = vec![DependentContextMotive::Independent {
            motive: ContextualMotive::Type,
        }];
        let source = dependent_internal(&signature, &motives, Expr::Lam(Box::new(Expr::Var(1))));
        let image_term = Expr::Var(1);
        let image = dependent_internal(&signature, &motives, image_term.clone());
        issue_motive_typed_open_specialization_v4(
            &signature,
            source,
            motives,
            vec![(image_term, image)],
        )
        .expect("generic open specialization must issue")
    }

    #[test]
    fn structural_sweep_derives_seventeen_and_names_only_the_exact_cycle() {
        let audit = issue_t_sm1b_corpus_audit_v4().expect("corpus audit");
        assert_eq!(audit.structural_case_count, T_SM1B_REGISTERED_GAP_COUNT);
        assert!(!audit.live_structural_surface_seal.is_empty());
        assert_eq!(audit.archived_e5_chronological_count, 72);
        assert!(audit.live_surface_ids_missing_from_archived_e5.is_empty());
        assert_eq!(audit.archived_e5_ids_outside_live_surface.len(), 54);
        assert!(audit.archived_e5_covers_live_surface);
        assert_eq!(audit.registered_v3_gap_count, T_SM1B_REGISTERED_GAP_COUNT);
        assert!(audit.structural_surface_equals_registered_18);
        assert_eq!(audit.derived_count, 17);
        assert_eq!(
            audit.blocker_counts,
            BTreeMap::from([("T_SM1B_DEPENDENT_TARGET_CYCLE".to_owned(), 1)])
        );
        assert!(audit.theorem_locally_replayable);
        assert!(!audit.t_sm1b_component_complete);
        replay_t_sm1b_corpus_audit_v4(&audit).expect("whole audit replay");

        let blocker = audit
            .exact_dependency_cycle_blocker
            .expect("exact blocker evidence");
        assert_eq!(
            blocker.instance_id,
            "blake3:4c9c4272b11cb6aec0c332cd6697401ccd3f95b8bfa2520b6b6dceb6986d3571"
        );
        assert!(blocker.fixed_f_sm1_gate_must_remain_false);
        assert!(blocker.source_second_motive_declaration_replayed);
        assert!(!blocker.source_internal_derivation_hash.is_empty());
        assert!(blocker.exact_identity_second_image);
        assert!(blocker.first_image_contains_second_target_parameter);
        assert!(blocker.substituted_classifier_contains_second_target_parameter);
        assert!(blocker.predecessor_only_condition_violated);
        assert!(blocker.dependency_inversion_proved);
        assert!(blocker.no_law_preserving_alternative_succeeded);
        assert!(
            blocker
                .alternative_trials
                .iter()
                .all(|trial| !trial.accepted)
        );
        replay_exact_open_dependency_cycle_blocker_v4(&blocker).expect("blocker replay");
    }

    #[test]
    fn cycle_blocker_rejects_dependency_edge_and_slot_map_mutations() {
        let blocker = issue_exact_open_dependency_cycle_blocker_v4().expect("exact blocker");

        let mut edge_mutation = blocker.clone();
        let self_edge = edge_mutation
            .dependency_graph
            .iter_mut()
            .find(|edge| edge.classifier_parameter == edge.referenced_parameter)
            .expect("self edge");
        self_edge.reference_is_strict_predecessor = true;
        assert_eq!(
            replay_exact_open_dependency_cycle_blocker_v4(&edge_mutation),
            Err(OpenSpecializationV4Error::ReplayMismatch)
        );

        let mut slot_mutation = blocker.clone();
        slot_mutation.interface_assignments.swap(0, 1);
        assert_eq!(
            replay_exact_open_dependency_cycle_blocker_v4(&slot_mutation),
            Err(OpenSpecializationV4Error::ReplayMismatch)
        );

        let mut acceptance_mutation = blocker;
        acceptance_mutation.alternative_trials[0].accepted = true;
        assert_eq!(
            replay_exact_open_dependency_cycle_blocker_v4(&acceptance_mutation),
            Err(OpenSpecializationV4Error::ReplayMismatch)
        );
    }

    #[test]
    fn exact_row_cycle_blocker_matches_genesis_and_rejects_forged_row() {
        let signature = SealedSignature::genesis_del_h15();
        let legacy = issue_exact_open_dependency_cycle_blocker_v4().expect("legacy blocker");
        let window =
            generate_a3_window_for_exact_prefix_unbounded(&signature, 16).expect("live A3 window");
        let instance = window
            .instances
            .iter()
            .find(|instance| instance.instance_id == legacy.instance_id)
            .expect("exact instance");
        let scheme = window
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == instance.scheme_id)
            .expect("exact scheme");
        let older = window
            .typed_sources
            .iter()
            .find(|source| source.anchor_id == instance.source_anchor_ids[0])
            .expect("older source");
        let newest = window
            .typed_sources
            .iter()
            .find(|source| source.anchor_id == instance.source_anchor_ids[1])
            .expect("newest source");
        let generic = issue_exact_open_dependency_cycle_blocker_for_row_v4(
            &signature, 15, instance, scheme, older, newest,
        )
        .expect("generic blocker");
        assert_eq!(generic, legacy);
        replay_exact_open_dependency_cycle_blocker_for_row_v4(
            &signature, 15, instance, scheme, older, newest, &generic,
        )
        .expect("generic blocker replay");

        let mut forged_instance = instance.clone();
        forged_instance.typing_derivation_hash.push_str("-forged");
        assert!(
            issue_exact_open_dependency_cycle_blocker_for_row_v4(
                &signature,
                15,
                &forged_instance,
                scheme,
                older,
                newest,
            )
            .is_err()
        );
    }

    #[test]
    fn generic_theorem_replays_and_rejects_term_and_proof_mutations() {
        let signature = SealedSignature::genesis_del_h15();
        let token = synthetic_specialization();
        replay_motive_typed_open_specialization_v4(&signature, token.projection())
            .expect("specialization replay");
        replay_equality_transported_internal_v4(
            &signature,
            &token.projection().specialized_internal,
        )
        .expect("equality transport replay");

        let mut term_mutation = token.projection().clone();
        term_mutation.images[0].term = Expr::Univ;
        assert!(replay_motive_typed_open_specialization_v4(&signature, &term_mutation).is_err());

        let mut equality_mutation = token.projection().specialized_internal.clone();
        equality_mutation.raw_to_normal_equality.equal = false;
        assert_eq!(
            replay_equality_transported_internal_v4(&signature, &equality_mutation),
            Err(OpenSpecializationV4Error::ReplayMismatch)
        );

        let mut rule_mutation = token
            .projection()
            .specialized_internal
            .normalized_internal
            .clone();
        rule_mutation.exact_kernel_rule_induction.constructor = "forged".to_owned();
        assert_eq!(
            replay_dependent_contextual_internal_v4(&signature, &rule_mutation),
            Err(OpenSpecializationV4Error::ReplayMismatch)
        );
    }

    #[test]
    fn full_motive_and_expression_grammar_are_inducted_without_shrinking_f_m2() {
        let signature = SealedSignature::genesis_del_h15();
        let contextual = vec![
            ContextualMotive::Type,
            ContextualMotive::Element(Expr::Var(1)),
            ContextualMotive::Function {
                domain: Box::new(ContextualMotive::Type),
                codomain: Box::new(ContextualMotive::Element(Expr::Var(1))),
            },
            ContextualMotive::Neutral,
        ];
        let inducted = contextual
            .iter()
            .map(contextual_motive_induction)
            .collect::<Vec<_>>();
        assert!(matches!(inducted[0], MotiveGrammarInductionNodeV4::Type));
        assert!(matches!(
            inducted[1],
            MotiveGrammarInductionNodeV4::Element { .. }
        ));
        assert!(matches!(
            inducted[2],
            MotiveGrammarInductionNodeV4::Function { .. }
        ));
        assert!(matches!(inducted[3], MotiveGrammarInductionNodeV4::Neutral));

        let source = signature
            .entries()
            .iter()
            .find(|entry| entry.telescope.clauses.len() >= 2)
            .expect("sealed multi-clause source");
        let opaque = issue_sealed_prior_clause_type_reference(
            &signature,
            15,
            source.step,
            &source.candidate_hash,
            1,
            0,
        )
        .expect("opaque prior-clause reference");
        let dependent = vec![
            DependentContextMotive::Independent {
                motive: ContextualMotive::Neutral,
            },
            DependentContextMotive::ElementOfApplicationHead {
                head: Expr::Eventually(Box::new(Expr::Var(1))),
            },
            DependentContextMotive::OpaquePriorClause { reference: opaque },
        ];
        let dependent_nodes = dependent
            .iter()
            .map(|motive| dependent_motive_induction(&signature, motive).expect("motive induction"))
            .collect::<Vec<_>>();
        assert!(matches!(
            dependent_nodes[0],
            MotiveGrammarInductionNodeV4::DependentIndependent { .. }
        ));
        assert!(matches!(
            dependent_nodes[1],
            MotiveGrammarInductionNodeV4::ElementOfApplicationHead { .. }
        ));
        assert!(matches!(
            dependent_nodes[2],
            MotiveGrammarInductionNodeV4::OpaquePriorClause {
                reference_replayed: true,
                ..
            }
        ));

        let unary = |constructor: fn(Box<Expr>) -> Expr| constructor(Box::new(Expr::Univ));
        let expressions = vec![
            Expr::App(Box::new(Expr::Univ), Box::new(Expr::Univ)),
            Expr::Lam(Box::new(Expr::Var(1))),
            Expr::Pi(Box::new(Expr::Univ), Box::new(Expr::Univ)),
            Expr::Sigma(Box::new(Expr::Univ), Box::new(Expr::Univ)),
            Expr::Univ,
            Expr::Var(1),
            Expr::Lib(1),
            Expr::Id(
                Box::new(Expr::Univ),
                Box::new(Expr::Univ),
                Box::new(Expr::Univ),
            ),
            Expr::Refl(Box::new(Expr::Univ)),
            unary(Expr::Susp),
            unary(Expr::Trunc),
            Expr::PathCon(1),
            unary(Expr::Flat),
            unary(Expr::Sharp),
            unary(Expr::Disc),
            unary(Expr::Shape),
            unary(Expr::Next),
            unary(Expr::Eventually),
            unary(Expr::Bang),
            unary(Expr::WhyNot),
        ];
        let constructors = expressions
            .iter()
            .map(|expression| expression_grammar_induction(expression).constructor)
            .collect::<BTreeSet<_>>();
        assert_eq!(constructors.len(), 20);
    }
}
