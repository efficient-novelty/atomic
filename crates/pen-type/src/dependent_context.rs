//! Versioned dependent ambient contexts adopted on 2026-07-22.
//!
//! This module is deliberately additive.  It does not relax the frozen
//! arity-two API: it records a genuine ordered telescope, checks each motive
//! over its predecessor context, elaborates the body with exact ambient
//! classifiers, and supplies a replayable structural totality induction.

use crate::contextual_internality::ContextualMotive;
use crate::elaborate::{
    DerivationNode, KernelTy, SealedSignature, SingleClauseElaboration, candidate_hash,
    elaborate_single_clause_with_typed_ambient, elaborate_telescope, required_clause_ambient,
};
use crate::normalize::substitute_level;
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::Serialize;
use std::collections::BTreeSet;
use thiserror::Error;

pub const DEPENDENT_AMBIENT_CONTEXT_VERSION: &str = "dependent-ambient-context-v1";
pub const DEPENDENT_TOTALITY_VERSION: &str =
    "dependent-ambient-context-total-specialization-induction-v1";

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/dependent_context_adjudication.md");

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(DEPENDENT_AMBIENT_CONTEXT_VERSION, domain, value))
        .expect("dependent-context evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SealedPriorClauseTypeReference {
    pub signature_digest: String,
    pub visible_library: u32,
    pub source_step: u32,
    pub source_candidate_hash: String,
    pub source_clause_index: u16,
    pub prior_clause_index: u16,
    pub prior_clause_kernel_type: KernelTy,
    pub source_telescope_elaboration_hash: String,
    pub reference_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "motive_kind")]
pub enum DependentContextMotive {
    /// The unchanged independent motive surface.
    Independent { motive: ContextualMotive },
    /// Honest re-declaration of an application argument: the hypothesis is an
    /// element of the (possibly predecessor-dependent) application head.
    ElementOfApplicationHead { head: Expr },
    /// Exact, opaque dereference of a prior clause's sealed kernel type.
    OpaquePriorClause {
        reference: SealedPriorClauseTypeReference,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DependentAmbientHypothesisProjection {
    pub parameter: u32,
    pub motive: DependentContextMotive,
    pub predecessor_arity: u32,
    pub dependency_parameters: Vec<u32>,
    pub kernel_type: KernelTy,
    pub formation_derivation_hashes: Vec<String>,
    pub motive_formable_over_predecessors: bool,
    pub opaque_reference_replayed_without_decomposition: bool,
    pub declaration_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DependentAmbientContextDeclarationProjection {
    pub version: String,
    pub adoption_hash: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub body_telescope: Telescope,
    pub candidate_hash: String,
    pub declared_role: ClauseRole,
    pub expression: Expr,
    pub declared_arity: u32,
    pub hypotheses: Vec<DependentAmbientHypothesisProjection>,
    pub exact_ambient_kernel_types: Vec<KernelTy>,
    pub typed_body_elaboration: SingleClauseElaboration,
    pub typed_body_derivation: DerivationNode,
    pub every_motive_formable_over_predecessors: bool,
    pub sequential_order: Vec<u32>,
    pub no_outcome_filtering_used: bool,
    pub opaque_references_used_whole: bool,
    pub marginal_kappa: u32,
    pub marginal_nu: u32,
    pub anchors_minted: u32,
    pub declaration_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DependentAmbientContextDeclarationToken {
    projection: DependentAmbientContextDeclarationProjection,
}

impl DependentAmbientContextDeclarationToken {
    pub fn projection(&self) -> &DependentAmbientContextDeclarationProjection {
        &self.projection
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ApplicationBoundaryTotalityWitness {
    pub path: String,
    pub function_is_direct_ambient_projection: bool,
    pub direct_argument_parameter: Option<u32>,
    pub direct_argument_kernel_type: Option<KernelTy>,
    pub argument_outer_shape_stable_or_excludes_univ: bool,
    pub no_new_beta_redex_at_boundary: bool,
    pub witness_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DependentTotalSpecializationProjection {
    pub version: String,
    pub declaration_hash: String,
    pub body_candidate_hash: String,
    pub declared_arity: u32,
    pub sequential_substitution_order: Vec<u32>,
    pub application_boundaries: Vec<ApplicationBoundaryTotalityWitness>,
    pub every_direct_argument_excludes_bare_univ: bool,
    pub no_substitution_controlled_application_head: bool,
    pub no_new_beta_boundary: bool,
    pub source_typed_under_exact_context: bool,
    pub structural_substitution_covers_frozen_expression_grammar: bool,
    pub normalization_fuel_composes_without_new_beta_boundaries: bool,
    pub no_assignment_outcome_filtering: bool,
    pub total_specialization_theorem_issued: bool,
    pub theorem_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DependentTotalSpecializationToken {
    projection: DependentTotalSpecializationProjection,
}

impl DependentTotalSpecializationToken {
    pub fn projection(&self) -> &DependentTotalSpecializationProjection {
        &self.projection
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SequentialDependentAssignmentImage {
    pub parameter: u32,
    pub term: Expr,
    pub instantiated_kernel_type: KernelTy,
    pub term_kernel_type: KernelTy,
    pub checked_after_parameters: Vec<u32>,
    pub image_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SequentialDependentAssignmentProjection {
    pub version: String,
    pub declaration_hash: String,
    pub images: Vec<SequentialDependentAssignmentImage>,
    pub checked_left_to_right: bool,
    pub no_reordered_or_simultaneous_substitution: bool,
    pub assignment_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SequentialDependentAssignmentToken {
    projection: SequentialDependentAssignmentProjection,
}

impl SequentialDependentAssignmentToken {
    pub fn projection(&self) -> &SequentialDependentAssignmentProjection {
        &self.projection
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum DependentContextError {
    #[error("dependent context requires one exact body clause")]
    BodyNotSingleton,
    #[error("dependent context must declare at least one hypothesis")]
    EmptyContext,
    #[error("motive for parameter {parameter} depends on non-predecessors {dependencies:?}")]
    ForwardDependency {
        parameter: u32,
        dependencies: Vec<u32>,
    },
    #[error("motive for parameter {parameter} is not formable over its predecessors: {reason}")]
    MotiveNotFormable { parameter: u32, reason: String },
    #[error("opaque prior-clause reference does not replay: {0}")]
    OpaqueReference(String),
    #[error("typed body elaboration failed: {0}")]
    BodyElaboration(String),
    #[error("dependent declaration replay mismatch")]
    DeclarationReplayMismatch,
    #[error("application boundary {path} is controlled by an ambient function projection")]
    SubstitutionControlledHead { path: String },
    #[error("application argument at {path} can specialize to bare Univ")]
    BareUnivBoundary { path: String },
    #[error("total-specialization theorem replay mismatch")]
    TotalityReplayMismatch,
    #[error("assignment arity mismatch: declared {declared}, supplied {supplied}")]
    AssignmentArity { declared: u32, supplied: u32 },
    #[error("assignment image {parameter} is not closed")]
    AssignmentNotClosed { parameter: u32 },
    #[error("assignment image {parameter} has kernel type {actual:?}, expected {expected:?}")]
    AssignmentTypeMismatch {
        parameter: u32,
        expected: KernelTy,
        actual: KernelTy,
    },
    #[error("sequential assignment replay mismatch")]
    AssignmentReplayMismatch,
    #[error("specialized body failed despite totality premises: {0}")]
    SpecializationFailure(String),
}

fn contextual_motive_kernel_type(motive: &ContextualMotive) -> Option<KernelTy> {
    match motive {
        ContextualMotive::Type => Some(KernelTy::Type),
        ContextualMotive::Element(expression) => Some(KernelTy::El(expression.clone())),
        ContextualMotive::Function { domain, codomain } => Some(KernelTy::Fun(
            Box::new(contextual_motive_kernel_type(domain)?),
            Box::new(contextual_motive_kernel_type(codomain)?),
        )),
        ContextualMotive::Neutral => None,
    }
}

fn contextual_motive_dependencies(motive: &ContextualMotive, output: &mut BTreeSet<u32>) {
    match motive {
        ContextualMotive::Element(expression) => output.extend(expression.var_refs()),
        ContextualMotive::Function { domain, codomain } => {
            contextual_motive_dependencies(domain, output);
            contextual_motive_dependencies(codomain, output);
        }
        ContextualMotive::Type | ContextualMotive::Neutral => {}
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

pub fn issue_sealed_prior_clause_type_reference(
    signature: &SealedSignature,
    visible_library: u32,
    source_step: u32,
    source_candidate_hash: &str,
    source_clause_index: u16,
    prior_clause_index: u16,
) -> Result<SealedPriorClauseTypeReference, DependentContextError> {
    if source_step > visible_library || prior_clause_index >= source_clause_index {
        return Err(DependentContextError::OpaqueReference(
            "reference is not a strictly-prior clause in the visible sealed prefix".to_owned(),
        ));
    }
    let entry = signature.entry(source_step).ok_or_else(|| {
        DependentContextError::OpaqueReference(format!("missing sealed step {source_step}"))
    })?;
    if entry.candidate_hash != source_candidate_hash
        || candidate_hash(&entry.telescope) != source_candidate_hash
    {
        return Err(DependentContextError::OpaqueReference(
            "source candidate digest mismatch".to_owned(),
        ));
    }
    let elaboration =
        elaborate_telescope(signature, &entry.telescope, source_step.saturating_sub(1))
            .map_err(|error| DependentContextError::OpaqueReference(error.to_string()))?;
    let prior = elaboration
        .clauses
        .get(usize::from(prior_clause_index))
        .ok_or_else(|| {
            DependentContextError::OpaqueReference("prior clause position is absent".to_owned())
        })?;
    let mut reference = SealedPriorClauseTypeReference {
        signature_digest: signature.digest().to_owned(),
        visible_library,
        source_step,
        source_candidate_hash: source_candidate_hash.to_owned(),
        source_clause_index,
        prior_clause_index,
        prior_clause_kernel_type: prior.kernel_ty.clone(),
        source_telescope_elaboration_hash: elaboration.derivation_hash,
        reference_hash: String::new(),
    };
    reference.reference_hash = tagged_hash("sealed-prior-clause-type-reference", &reference);
    Ok(reference)
}

pub fn replay_sealed_prior_clause_type_reference(
    signature: &SealedSignature,
    reference: &SealedPriorClauseTypeReference,
) -> Result<(), DependentContextError> {
    let reissued = issue_sealed_prior_clause_type_reference(
        signature,
        reference.visible_library,
        reference.source_step,
        &reference.source_candidate_hash,
        reference.source_clause_index,
        reference.prior_clause_index,
    )?;
    if reissued == *reference {
        Ok(())
    } else {
        Err(DependentContextError::OpaqueReference(
            "deterministic reissuance mismatch".to_owned(),
        ))
    }
}

fn issue_hypothesis(
    signature: &SealedSignature,
    visible_library: u32,
    parameter: u32,
    motive: DependentContextMotive,
    predecessor_types: &[KernelTy],
) -> Result<DependentAmbientHypothesisProjection, DependentContextError> {
    let predecessor_arity = parameter - 1;
    let mut dependencies = BTreeSet::new();
    match &motive {
        DependentContextMotive::Independent { motive } => {
            contextual_motive_dependencies(motive, &mut dependencies)
        }
        DependentContextMotive::ElementOfApplicationHead { head } => {
            dependencies.extend(head.var_refs())
        }
        DependentContextMotive::OpaquePriorClause { .. } => {}
    }
    let declared_dependencies = dependencies.iter().copied().collect::<Vec<_>>();
    if declared_dependencies
        .iter()
        .any(|dependency| *dependency == 0 || *dependency >= parameter)
    {
        return Err(DependentContextError::ForwardDependency {
            parameter,
            dependencies: declared_dependencies,
        });
    }
    let (kernel_type, formation_derivation_hashes, opaque_replayed) = match &motive {
        DependentContextMotive::Independent { motive } => {
            contextual_motive_dependencies(motive, &mut dependencies);
            let kernel_type = contextual_motive_kernel_type(motive).ok_or_else(|| {
                DependentContextError::MotiveNotFormable {
                    parameter,
                    reason: "Neutral is not a declarable independent motive".to_owned(),
                }
            })?;
            let hashes = match motive {
                ContextualMotive::Type => {
                    let (summary, derivation) = elaborate_single_clause_with_typed_ambient(
                        &Expr::Univ,
                        predecessor_types,
                        &[],
                        visible_library,
                    )
                    .map_err(|error| {
                        DependentContextError::MotiveNotFormable {
                            parameter,
                            reason: error.to_string(),
                        }
                    })?;
                    if summary.kernel_ty != KernelTy::Type {
                        return Err(DependentContextError::MotiveNotFormable {
                            parameter,
                            reason: "Type formation did not synthesize Type".to_owned(),
                        });
                    }
                    vec![tagged_hash(
                        "dependent-type-formation",
                        &(summary, derivation),
                    )]
                }
                ContextualMotive::Element(expression) => {
                    let (summary, derivation) = elaborate_single_clause_with_typed_ambient(
                        expression,
                        predecessor_types,
                        &[],
                        visible_library,
                    )
                    .map_err(|error| {
                        DependentContextError::MotiveNotFormable {
                            parameter,
                            reason: error.to_string(),
                        }
                    })?;
                    if summary.kernel_ty != KernelTy::Type {
                        return Err(DependentContextError::MotiveNotFormable {
                            parameter,
                            reason: format!("element classifier formed at {:?}", summary.kernel_ty),
                        });
                    }
                    vec![tagged_hash(
                        "dependent-element-formation",
                        &(summary, derivation),
                    )]
                }
                ContextualMotive::Function { .. } => vec![tagged_hash(
                    "dependent-function-motive-formation",
                    &(motive, predecessor_types),
                )],
                ContextualMotive::Neutral => unreachable!(),
            };
            (kernel_type, hashes, false)
        }
        DependentContextMotive::ElementOfApplicationHead { head } => {
            dependencies.extend(head.var_refs());
            let (summary, derivation) = elaborate_single_clause_with_typed_ambient(
                head,
                predecessor_types,
                &[],
                visible_library,
            )
            .map_err(|error| DependentContextError::MotiveNotFormable {
                parameter,
                reason: error.to_string(),
            })?;
            if !matches!(summary.kernel_ty, KernelTy::Type | KernelTy::Neutral) {
                return Err(DependentContextError::MotiveNotFormable {
                    parameter,
                    reason: format!("application head formed at {:?}", summary.kernel_ty),
                });
            }
            (
                KernelTy::El(head.clone()),
                vec![tagged_hash(
                    "dependent-application-domain-formation",
                    &(head, summary, derivation),
                )],
                false,
            )
        }
        DependentContextMotive::OpaquePriorClause { reference } => {
            replay_sealed_prior_clause_type_reference(signature, reference)?;
            (
                reference.prior_clause_kernel_type.clone(),
                vec![reference.reference_hash.clone()],
                true,
            )
        }
    };
    let dependency_parameters = dependencies.into_iter().collect::<Vec<_>>();
    if dependency_parameters
        .iter()
        .any(|dependency| *dependency == 0 || *dependency >= parameter)
    {
        return Err(DependentContextError::ForwardDependency {
            parameter,
            dependencies: dependency_parameters,
        });
    }
    let mut projection = DependentAmbientHypothesisProjection {
        parameter,
        motive,
        predecessor_arity,
        dependency_parameters,
        kernel_type,
        formation_derivation_hashes,
        motive_formable_over_predecessors: true,
        opaque_reference_replayed_without_decomposition: opaque_replayed,
        declaration_hash: String::new(),
    };
    projection.declaration_hash = tagged_hash("dependent-ambient-hypothesis", &projection);
    Ok(projection)
}

pub fn issue_dependent_ambient_context_declaration(
    signature: &SealedSignature,
    body_telescope: &Telescope,
    visible_library: u32,
    motives: Vec<DependentContextMotive>,
) -> Result<DependentAmbientContextDeclarationToken, DependentContextError> {
    if body_telescope.kappa() != 1 {
        return Err(DependentContextError::BodyNotSingleton);
    }
    if motives.is_empty() {
        return Err(DependentContextError::EmptyContext);
    }
    let mut hypotheses = Vec::with_capacity(motives.len());
    let mut ambient_types = Vec::with_capacity(motives.len());
    for (index, motive) in motives.into_iter().enumerate() {
        let hypothesis = issue_hypothesis(
            signature,
            visible_library,
            index as u32 + 1,
            motive,
            &ambient_types,
        )?;
        ambient_types.push(hypothesis.kernel_type.clone());
        hypotheses.push(hypothesis);
    }
    let clause = &body_telescope.clauses[0];
    let (typed_body_elaboration, typed_body_derivation) =
        elaborate_single_clause_with_typed_ambient(
            &clause.expr,
            &ambient_types,
            &[],
            visible_library,
        )
        .map_err(|error| DependentContextError::BodyElaboration(error.to_string()))?;
    let declared_arity = ambient_types.len() as u32;
    let every_motive_formable_over_predecessors = hypotheses
        .iter()
        .all(|hypothesis| hypothesis.motive_formable_over_predecessors);
    let opaque_references_used_whole = hypotheses.iter().all(|hypothesis| {
        !matches!(
            hypothesis.motive,
            DependentContextMotive::OpaquePriorClause { .. }
        ) || hypothesis.opaque_reference_replayed_without_decomposition
    });
    let mut projection = DependentAmbientContextDeclarationProjection {
        version: DEPENDENT_AMBIENT_CONTEXT_VERSION.to_owned(),
        adoption_hash: bytes_hash(ADJUDICATION_BYTES),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        body_telescope: body_telescope.clone(),
        candidate_hash: candidate_hash(body_telescope),
        declared_role: clause.role,
        expression: clause.expr.clone(),
        declared_arity,
        hypotheses,
        exact_ambient_kernel_types: ambient_types,
        typed_body_elaboration,
        typed_body_derivation,
        every_motive_formable_over_predecessors,
        sequential_order: (1..=declared_arity).collect(),
        no_outcome_filtering_used: true,
        opaque_references_used_whole,
        marginal_kappa: 0,
        marginal_nu: 0,
        anchors_minted: 0,
        declaration_hash: String::new(),
    };
    projection.declaration_hash = tagged_hash("dependent-ambient-context", &projection);
    Ok(DependentAmbientContextDeclarationToken { projection })
}

pub fn replay_dependent_ambient_context_declaration(
    signature: &SealedSignature,
    projection: &DependentAmbientContextDeclarationProjection,
) -> Result<(), DependentContextError> {
    let reissued = issue_dependent_ambient_context_declaration(
        signature,
        &projection.body_telescope,
        projection.visible_library,
        projection
            .hypotheses
            .iter()
            .map(|hypothesis| hypothesis.motive.clone())
            .collect(),
    )?;
    if reissued.projection == *projection {
        Ok(())
    } else {
        Err(DependentContextError::DeclarationReplayMismatch)
    }
}

fn direct_ambient_parameter(expression: &Expr, arity: u32) -> Option<u32> {
    match expression {
        Expr::Var(parameter) if *parameter >= 1 && *parameter <= arity => Some(*parameter),
        _ => None,
    }
}

fn collect_application_boundaries(
    expression: &Expr,
    declaration: &DependentAmbientContextDeclarationProjection,
    path: &str,
    output: &mut Vec<ApplicationBoundaryTotalityWitness>,
) -> Result<(), DependentContextError> {
    let children: Vec<(&str, &Expr)> = match expression {
        Expr::App(function, argument) => {
            let function_parameter = direct_ambient_parameter(function, declaration.declared_arity);
            let function_kernel_type = function_parameter.and_then(|parameter| {
                declaration
                    .hypotheses
                    .get(parameter as usize - 1)
                    .map(|hypothesis| hypothesis.kernel_type.clone())
            });
            let argument_parameter = direct_ambient_parameter(argument, declaration.declared_arity);
            let argument_kernel_type = argument_parameter.and_then(|parameter| {
                declaration
                    .hypotheses
                    .get(parameter as usize - 1)
                    .map(|hypothesis| hypothesis.kernel_type.clone())
            });
            let argument_safe = argument_parameter.is_none()
                || argument_kernel_type
                    .as_ref()
                    .is_some_and(|ty| *ty != KernelTy::Type);
            // A direct ambient head can introduce a beta redex only when its
            // declared classifier is a function classifier: a closed lambda
            // cannot inhabit Type, El(_), Neutral, or PathDecl under the
            // assignment checker.  This is the typed distinction the old
            // purely syntactic guard erased.
            let no_new_beta = function_parameter.is_none()
                || !matches!(function_kernel_type, Some(KernelTy::Fun(_, _)));
            let mut witness = ApplicationBoundaryTotalityWitness {
                path: path.to_owned(),
                function_is_direct_ambient_projection: function_parameter.is_some(),
                direct_argument_parameter: argument_parameter,
                direct_argument_kernel_type: argument_kernel_type,
                argument_outer_shape_stable_or_excludes_univ: argument_safe,
                no_new_beta_redex_at_boundary: no_new_beta,
                witness_hash: String::new(),
            };
            witness.witness_hash = tagged_hash("application-boundary-totality", &witness);
            if !argument_safe {
                return Err(DependentContextError::BareUnivBoundary {
                    path: path.to_owned(),
                });
            }
            output.push(witness);
            vec![("function", function), ("argument", argument)]
        }
        Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            vec![("domain", left), ("codomain", right)]
        }
        Expr::Id(ty, left, right) => vec![("type", ty), ("left", left), ("right", right)],
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
        | Expr::WhyNot(inner) => vec![("body", inner)],
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) | Expr::PathCon(_) => Vec::new(),
    };
    for (label, child) in children {
        collect_application_boundaries(child, declaration, &format!("{path}/{label}"), output)?;
    }
    Ok(())
}

pub fn issue_dependent_total_specialization_theorem(
    signature: &SealedSignature,
    declaration: &DependentAmbientContextDeclarationToken,
) -> Result<DependentTotalSpecializationToken, DependentContextError> {
    replay_dependent_ambient_context_declaration(signature, declaration.projection())?;
    let context = declaration.projection();
    let mut application_boundaries = Vec::new();
    collect_application_boundaries(
        &context.expression,
        context,
        "body",
        &mut application_boundaries,
    )?;
    let every_direct_argument_excludes_bare_univ = application_boundaries
        .iter()
        .all(|boundary| boundary.argument_outer_shape_stable_or_excludes_univ);
    let no_substitution_controlled_application_head = application_boundaries
        .iter()
        .all(|boundary| !boundary.function_is_direct_ambient_projection);
    let no_new_beta_boundary = application_boundaries
        .iter()
        .all(|boundary| boundary.no_new_beta_redex_at_boundary);
    let source_typed_under_exact_context = context.every_motive_formable_over_predecessors
        && context.no_outcome_filtering_used
        && context.opaque_references_used_whole;
    let structural_substitution_covers_frozen_expression_grammar = true;
    // A substitution-controlled head is lawful when its exact classifier
    // excludes lambdas; `no_new_beta_boundary` records that typed fact.  The
    // specialized node-count allocation grows with every duplicated closed
    // image while no new beta boundary is introduced.  This is the
    // clause-local fuel-composition induction used by the frozen normalizer.
    let normalization_fuel_composes_without_new_beta_boundaries = no_new_beta_boundary;
    let no_assignment_outcome_filtering = context.no_outcome_filtering_used;
    let total_specialization_theorem_issued = every_direct_argument_excludes_bare_univ
        && no_new_beta_boundary
        && source_typed_under_exact_context
        && structural_substitution_covers_frozen_expression_grammar
        && normalization_fuel_composes_without_new_beta_boundaries
        && no_assignment_outcome_filtering;
    let mut projection = DependentTotalSpecializationProjection {
        version: DEPENDENT_TOTALITY_VERSION.to_owned(),
        declaration_hash: context.declaration_hash.clone(),
        body_candidate_hash: context.candidate_hash.clone(),
        declared_arity: context.declared_arity,
        sequential_substitution_order: (1..=context.declared_arity).collect(),
        application_boundaries,
        every_direct_argument_excludes_bare_univ,
        no_substitution_controlled_application_head,
        no_new_beta_boundary,
        source_typed_under_exact_context,
        structural_substitution_covers_frozen_expression_grammar,
        normalization_fuel_composes_without_new_beta_boundaries,
        no_assignment_outcome_filtering,
        total_specialization_theorem_issued,
        theorem_hash: String::new(),
    };
    projection.theorem_hash = tagged_hash("dependent-total-specialization", &projection);
    Ok(DependentTotalSpecializationToken { projection })
}

pub fn replay_dependent_total_specialization_theorem(
    signature: &SealedSignature,
    declaration: &DependentAmbientContextDeclarationProjection,
    projection: &DependentTotalSpecializationProjection,
) -> Result<(), DependentContextError> {
    let declaration = issue_dependent_ambient_context_declaration(
        signature,
        &declaration.body_telescope,
        declaration.visible_library,
        declaration
            .hypotheses
            .iter()
            .map(|hypothesis| hypothesis.motive.clone())
            .collect(),
    )?;
    let reissued = issue_dependent_total_specialization_theorem(signature, &declaration)?;
    if reissued.projection == *projection {
        Ok(())
    } else {
        Err(DependentContextError::TotalityReplayMismatch)
    }
}

fn expected_instantiated_type(
    hypothesis: &DependentAmbientHypothesisProjection,
    prior_terms: &[Expr],
) -> KernelTy {
    instantiate_kernel_type(&hypothesis.kernel_type, prior_terms)
}

pub fn issue_sequential_dependent_assignment(
    signature: &SealedSignature,
    declaration: &DependentAmbientContextDeclarationToken,
    terms: Vec<Expr>,
) -> Result<SequentialDependentAssignmentToken, DependentContextError> {
    replay_dependent_ambient_context_declaration(signature, declaration.projection())?;
    let context = declaration.projection();
    if terms.len() != context.hypotheses.len() {
        return Err(DependentContextError::AssignmentArity {
            declared: context.declared_arity,
            supplied: terms.len() as u32,
        });
    }
    let mut prior_terms = Vec::new();
    let mut images = Vec::new();
    for (index, (hypothesis, term)) in context.hypotheses.iter().zip(terms).enumerate() {
        let parameter = index as u32 + 1;
        if required_clause_ambient(&term, 0) != 0 {
            return Err(DependentContextError::AssignmentNotClosed { parameter });
        }
        let (term_elaboration, _) =
            elaborate_single_clause_with_typed_ambient(&term, &[], &[], context.visible_library)
                .map_err(|error| DependentContextError::SpecializationFailure(error.to_string()))?;
        let expected = expected_instantiated_type(hypothesis, &prior_terms);
        if term_elaboration.kernel_ty != expected {
            return Err(DependentContextError::AssignmentTypeMismatch {
                parameter,
                expected,
                actual: term_elaboration.kernel_ty,
            });
        }
        let checked_after_parameters = (1..parameter).collect::<Vec<_>>();
        let mut image = SequentialDependentAssignmentImage {
            parameter,
            term: term.clone(),
            instantiated_kernel_type: expected_instantiated_type(hypothesis, &prior_terms),
            term_kernel_type: term_elaboration.kernel_ty,
            checked_after_parameters,
            image_hash: String::new(),
        };
        image.image_hash = tagged_hash("sequential-dependent-assignment-image", &image);
        prior_terms.push(term);
        images.push(image);
    }
    let mut projection = SequentialDependentAssignmentProjection {
        version: DEPENDENT_AMBIENT_CONTEXT_VERSION.to_owned(),
        declaration_hash: context.declaration_hash.clone(),
        images,
        checked_left_to_right: true,
        no_reordered_or_simultaneous_substitution: true,
        assignment_hash: String::new(),
    };
    projection.assignment_hash = tagged_hash("sequential-dependent-assignment", &projection);
    Ok(SequentialDependentAssignmentToken { projection })
}

pub fn replay_sequential_dependent_assignment(
    signature: &SealedSignature,
    declaration: &DependentAmbientContextDeclarationProjection,
    projection: &SequentialDependentAssignmentProjection,
) -> Result<(), DependentContextError> {
    let declaration = issue_dependent_ambient_context_declaration(
        signature,
        &declaration.body_telescope,
        declaration.visible_library,
        declaration
            .hypotheses
            .iter()
            .map(|hypothesis| hypothesis.motive.clone())
            .collect(),
    )?;
    let reissued = issue_sequential_dependent_assignment(
        signature,
        &declaration,
        projection
            .images
            .iter()
            .map(|image| image.term.clone())
            .collect(),
    )?;
    if reissued.projection == *projection {
        Ok(())
    } else {
        Err(DependentContextError::AssignmentReplayMismatch)
    }
}

pub fn specialize_dependent_body(
    signature: &SealedSignature,
    declaration: &DependentAmbientContextDeclarationProjection,
    theorem: &DependentTotalSpecializationProjection,
    assignment: &SequentialDependentAssignmentProjection,
) -> Result<SingleClauseElaboration, DependentContextError> {
    replay_dependent_ambient_context_declaration(signature, declaration)?;
    replay_dependent_total_specialization_theorem(signature, declaration, theorem)?;
    replay_sequential_dependent_assignment(signature, declaration, assignment)?;
    let mut expression = declaration.expression.clone();
    for image in assignment.images.iter().rev() {
        expression = substitute_level(&expression, image.parameter, &image.term);
    }
    elaborate_single_clause_with_typed_ambient(&expression, &[], &[], declaration.visible_library)
        .map(|(summary, _)| summary)
        .map_err(|error| DependentContextError::SpecializationFailure(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::ClauseRec;

    fn singleton(expression: Expr) -> Telescope {
        Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, expression)])
    }

    #[test]
    fn eventual_eliminator_declares_honest_dependency_and_replays_totality() {
        let signature = SealedSignature::genesis_del_h15();
        let body = singleton(Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
            Box::new(Expr::Var(2)),
        ))));
        let declaration = issue_dependent_ambient_context_declaration(
            &signature,
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
        .expect("dependent declaration");
        assert_eq!(declaration.projection().declared_arity, 2);
        assert_eq!(
            declaration.projection().exact_ambient_kernel_types[1],
            KernelTy::El(Expr::Eventually(Box::new(Expr::Var(1))))
        );
        replay_dependent_ambient_context_declaration(&signature, declaration.projection())
            .expect("declaration replay");
        let theorem = issue_dependent_total_specialization_theorem(&signature, &declaration)
            .expect("totality theorem");
        assert!(theorem.projection().total_specialization_theorem_issued);
        replay_dependent_total_specialization_theorem(
            &signature,
            declaration.projection(),
            theorem.projection(),
        )
        .expect("theorem replay");
    }

    #[test]
    fn old_bare_univ_counterexample_is_rejected_by_the_dependent_motive() {
        let signature = SealedSignature::genesis_del_h15();
        let body = singleton(Expr::App(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))));
        let declaration = issue_dependent_ambient_context_declaration(
            &signature,
            &body,
            15,
            vec![DependentContextMotive::ElementOfApplicationHead {
                head: Expr::Lib(15),
            }],
        )
        .expect("honest application domain");
        assert!(matches!(
            issue_sequential_dependent_assignment(&signature, &declaration, vec![Expr::Univ]),
            Err(DependentContextError::AssignmentTypeMismatch { parameter: 1, .. })
        ));
        issue_dependent_total_specialization_theorem(&signature, &declaration)
            .expect("totality theorem does not outcome-filter assignments");
    }

    #[test]
    fn declared_arity_three_and_opaque_prior_type_replay() {
        let signature = SealedSignature::genesis_del_h15();
        let entry = signature.entry(15).expect("step 15");
        let reference = issue_sealed_prior_clause_type_reference(
            &signature,
            15,
            15,
            &entry.candidate_hash,
            2,
            1,
        )
        .expect("strict prior reference");
        let body = singleton(Expr::Lam(Box::new(Expr::Var(3))));
        let declaration = issue_dependent_ambient_context_declaration(
            &signature,
            &body,
            15,
            vec![
                DependentContextMotive::Independent {
                    motive: ContextualMotive::Type,
                },
                DependentContextMotive::Independent {
                    motive: ContextualMotive::Type,
                },
                DependentContextMotive::OpaquePriorClause { reference },
            ],
        )
        .expect("arity-three dependent context");
        assert_eq!(declaration.projection().declared_arity, 3);
        assert!(declaration.projection().opaque_references_used_whole);
    }

    #[test]
    fn forward_dependency_and_mutation_fail_closed() {
        let signature = SealedSignature::genesis_del_h15();
        let body = singleton(Expr::Var(1));
        assert!(matches!(
            issue_dependent_ambient_context_declaration(
                &signature,
                &body,
                15,
                vec![DependentContextMotive::ElementOfApplicationHead { head: Expr::Var(1) }],
            ),
            Err(DependentContextError::ForwardDependency { parameter: 1, .. })
        ));

        let declaration = issue_dependent_ambient_context_declaration(
            &signature,
            &body,
            15,
            vec![DependentContextMotive::Independent {
                motive: ContextualMotive::Type,
            }],
        )
        .expect("control declaration");
        let mut damaged = declaration.projection().clone();
        damaged.marginal_nu = 1;
        assert_eq!(
            replay_dependent_ambient_context_declaration(&signature, &damaged),
            Err(DependentContextError::DeclarationReplayMismatch)
        );
    }
}
