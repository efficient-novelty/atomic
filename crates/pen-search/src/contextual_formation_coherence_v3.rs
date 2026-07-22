//! Contextual Formation closure and open identity-tail specialization.
//!
//! This is a versioned successor to the closed-only assignment boundary in
//! `motive_parametric_coherence_v2`.  It deliberately does not alter that
//! frozen theorem.  The new rule is narrower than arbitrary contextual
//! Formation:
//!
//! * the source must be the canonical presentation of a replayed family in
//!   the typed predecessor closure;
//! * the source judgment must synthesize `Type` under its exact canonical
//!   parameter context **and** carry a replayed `VerifiedClosureDerivationV2`;
//! * the first assignment image is either another sealed Type family or the
//!   pointwise application of a sealed Type-valued family;
//! * every remaining image is the ordered identity `Var(i)`, and every image
//!   carries its own exact contextual `VerifiedClosureDerivationV2`;
//! * structural substitution, target-context typing, normalization, and the
//!   resulting contextual Formation judgment are replayed exactly.
//!
//! Thus neither typing nor predecessor-family membership is promoted to
//! `Internal`, and a candidate-fresh Formation still receives no zero-credit
//! rule.  The present frozen calculus cannot issue the Formation premise (and
//! has no general open-motive eliminator for the remaining rows), so the v3
//! corpus sweep currently reports named gaps rather than a pass.

use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::A3TypedClauseSource;
use pen_eval::typed_families::{ParamSort, clause_presentation, predecessor_closure};
use pen_type::contextual_internality::ContextualMotive;
use pen_type::contextual_internality::{
    issue_ambient_context_declaration_token, issue_explicit_ambient_context_declaration_token,
};
use pen_type::elaborate::{
    KernelTy, SealedSignature, elaborate_single_clause_with_typed_ambient, elaborate_telescope,
};
use pen_type::equality::{EqualityWitness, univalent_equality};
use pen_type::motive_parametric_coherence_v2::{
    VerifiedClosureDerivationV2, issue_actual_body_closure_derivation_v2,
    issue_explicit_contextual_closure_derivation_v2, replay_verified_closure_derivation_v2,
};
use pen_type::substitution::{
    ParameterSort, SortedParameterContext, SubstitutionImage, is_well_scoped,
    issue_structural_substitution, replay_structural_substitution,
};
use serde::Serialize;
use thiserror::Error;

pub const CONTEXTUAL_FORMATION_COHERENCE_V3_VERSION: &str =
    "sealed-contextual-formation-open-identity-tail-specialization-v3";
const NORMALIZATION_FUEL: u32 = 512;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedNaturalFamilyProjectionV3 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub source: A3TypedClauseSource,
    pub predecessor_closure_digest: String,
    pub closure_provider_step: u32,
    pub closure_provider_clause: u16,
    pub closure_family_id: String,
    pub canonical_context: Vec<KernelTy>,
    pub canonical_expression: Expr,
    pub canonical_kernel_role: ClauseRole,
    pub canonical_kernel_type: KernelTy,
    pub source_telescope_replayed: bool,
    pub canonical_presentation_replayed: bool,
    pub canonical_typing_replayed: bool,
    pub no_charged_or_outside_constructor: bool,
    pub past_only_authority: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SealedNaturalFamilyTokenV3 {
    projection: SealedNaturalFamilyProjectionV3,
}

impl SealedNaturalFamilyTokenV3 {
    pub fn projection(&self) -> &SealedNaturalFamilyProjectionV3 {
        &self.projection
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContextualFormationClosureProjectionV3 {
    pub version: String,
    pub sealed_family: SealedNaturalFamilyProjectionV3,
    pub declared_role_is_formation: bool,
    pub inferred_motive: ContextualMotive,
    pub contextual_result_is_type: bool,
    pub internal_premise: VerifiedClosureDerivationV2,
    pub genuine_internal_premise_replayed: bool,
    pub candidate_fresh_formation_admitted: bool,
    pub independent_demand_orbit_exported: bool,
    pub internal_closure_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContextualFormationClosureTokenV3 {
    projection: ContextualFormationClosureProjectionV3,
}

impl ContextualFormationClosureTokenV3 {
    pub fn projection(&self) -> &ContextualFormationClosureProjectionV3 {
        &self.projection
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "origin")]
pub enum FirstImageOriginV3 {
    DirectSealedTypeFamily {
        source: A3TypedClauseSource,
    },
    PointwiseSealedTypeValuedFamily {
        source: A3TypedClauseSource,
        argument_parameter: u32,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "image_kind")]
pub enum ContextualAssignmentImageOriginV3 {
    DirectSealedTypeFamily {
        evidence: SealedNaturalFamilyProjectionV3,
    },
    PointwiseSealedTypeValuedFamily {
        evidence: SealedNaturalFamilyProjectionV3,
        argument_parameter: u32,
    },
    OrderedIdentityTail {
        target_parameter: u32,
    },
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContextualAssignmentImageProjectionV3 {
    pub source_parameter: u32,
    pub source_classifier: KernelTy,
    pub term: Expr,
    pub target_classifier: KernelTy,
    pub target_well_scoped: bool,
    pub exact_classifier_preserved: bool,
    pub internal_evidence: VerifiedClosureDerivationV2,
    pub genuine_internal_evidence_replayed: bool,
    pub origin: ContextualAssignmentImageOriginV3,
    pub typing_derivation_hash: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrderedOpenAssignmentProjectionV3 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub source_context: Vec<KernelTy>,
    pub target_context: Vec<KernelTy>,
    pub images: Vec<ContextualAssignmentImageProjectionV3>,
    pub source_arity: u32,
    pub target_arity: u32,
    pub first_image_genuine_internal_evidence_replayed: bool,
    pub ordered_identity_tail_exact: bool,
    pub no_permutation_or_instance_override: bool,
    pub open_target_context_permitted: bool,
    pub every_image_motive_typed: bool,
    pub every_image_genuine_internal_evidence_replayed: bool,
    pub assignment_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OrderedOpenAssignmentTokenV3 {
    projection: OrderedOpenAssignmentProjectionV3,
    first_origin: FirstImageOriginV3,
}

impl OrderedOpenAssignmentTokenV3 {
    pub fn projection(&self) -> &OrderedOpenAssignmentProjectionV3 {
        &self.projection
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DerivedContextualFormationProjectionV3 {
    pub expression: Expr,
    pub target_context: Vec<KernelTy>,
    pub inferred_motive: ContextualMotive,
    pub kernel_type: KernelTy,
    pub kernel_role: ClauseRole,
    pub normal_form: Expr,
    pub well_scoped_in_target: bool,
    pub contextual_typing_replayed: bool,
    pub source_internal_replayed: bool,
    pub every_assignment_image_internal_replayed: bool,
    pub substitution_stability_replayed: bool,
    pub candidate_fresh_formation_admitted: bool,
    pub internal_closure_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OpenFormationSpecializationProjectionV3 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub source: ContextualFormationClosureProjectionV3,
    pub assignment: OrderedOpenAssignmentProjectionV3,
    pub substitution_body: Expr,
    pub substitution_images: Vec<SubstitutionImage>,
    pub substitution_result: Expr,
    pub substitution_derivation_hash: String,
    pub structural_substitution_replayed: bool,
    pub source_typing_derivation_hash: String,
    pub result_typing_derivation_hash: String,
    pub exact_kernel_type_preserved: bool,
    pub exact_result_replayed: bool,
    pub source_normal_form_substitution_result: Expr,
    pub normalization_commutes: EqualityWitness,
    pub specialized_relation: DerivedContextualFormationProjectionV3,
    pub target_may_remain_open: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct OpenFormationSpecializationTokenV3 {
    projection: OpenFormationSpecializationProjectionV3,
}

impl OpenFormationSpecializationTokenV3 {
    pub fn projection(&self) -> &OpenFormationSpecializationProjectionV3 {
        &self.projection
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ContextualFormationCoherenceV3Error {
    #[error("sealed family premise failed: {0}")]
    SealedFamily(String),
    #[error("contextual Formation premise failed: {0}")]
    Formation(String),
    #[error("ordered open assignment failed: {0}")]
    Assignment(String),
    #[error("open Formation specialization failed: {0}")]
    Specialization(String),
    #[error("v3 replay mismatch")]
    ReplayMismatch,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CONTEXTUAL_FORMATION_COHERENCE_V3_VERSION, domain, value))
        .expect("v3 contextual Formation evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

pub fn kernel_context_from_parameter_sorts(parameters: &[ParamSort]) -> Vec<KernelTy> {
    parameters
        .iter()
        .map(|sort| match sort {
            ParamSort::Type => KernelTy::Type,
            ParamSort::Opaque => KernelTy::Neutral,
        })
        .collect()
}

fn kernel_motive(ty: &KernelTy) -> ContextualMotive {
    match ty {
        KernelTy::Type => ContextualMotive::Type,
        KernelTy::El(expression) => ContextualMotive::Element(expression.clone()),
        KernelTy::Fun(domain, codomain) => ContextualMotive::Function {
            domain: Box::new(kernel_motive(domain)),
            codomain: Box::new(kernel_motive(codomain)),
        },
        KernelTy::PathDecl { .. } | KernelTy::Neutral => ContextualMotive::Neutral,
    }
}

fn structural_context(types: &[KernelTy]) -> SortedParameterContext {
    SortedParameterContext::new(
        types
            .iter()
            .map(|ty| {
                if *ty == KernelTy::Type {
                    ParameterSort::Type
                } else {
                    ParameterSort::Opaque
                }
            })
            .collect(),
    )
}

fn contains_charged_or_outside(expression: &Expr) -> bool {
    let recurse = |inner: &Expr| contains_charged_or_outside(inner);
    match expression {
        Expr::PathCon(_) | Expr::Bang(_) | Expr::WhyNot(_) => true,
        Expr::App(left, right) | Expr::Pi(left, right) | Expr::Sigma(left, right) => {
            recurse(left) || recurse(right)
        }
        Expr::Id(ty, left, right) => recurse(ty) || recurse(left) || recurse(right),
        Expr::Lam(inner)
        | Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner) => recurse(inner),
        Expr::Univ | Expr::Var(_) | Expr::Lib(_) => false,
    }
}

pub fn issue_sealed_natural_family_v3(
    signature: &SealedSignature,
    visible_library: u32,
    source: &A3TypedClauseSource,
) -> Result<SealedNaturalFamilyTokenV3, ContextualFormationCoherenceV3Error> {
    if source.step == 0 || source.step > visible_library || !source.exported_public_clause {
        return Err(ContextualFormationCoherenceV3Error::SealedFamily(
            "source is not a public act in the visible sealed past".to_owned(),
        ));
    }
    let entry = signature.entry(source.step).ok_or_else(|| {
        ContextualFormationCoherenceV3Error::SealedFamily(format!(
            "sealed step {} is absent from the supplied prefix",
            source.step
        ))
    })?;
    let elaboration = elaborate_telescope(signature, &entry.telescope, source.step - 1)
        .map_err(|error| ContextualFormationCoherenceV3Error::SealedFamily(error.to_string()))?;
    let clause = elaboration
        .clauses
        .get(usize::from(source.clause_index))
        .ok_or_else(|| {
            ContextualFormationCoherenceV3Error::SealedFamily(
                "source clause is outside its sealed telescope".to_owned(),
            )
        })?;
    let prior_roles = elaboration
        .clauses
        .iter()
        .map(|clause| clause.kernel_role)
        .collect::<Vec<_>>();
    let presentation = clause_presentation(
        &clause.normal_form,
        elaboration.ambient_parameters + u32::from(source.clause_index),
        &prior_roles[..usize::from(source.clause_index)],
        elaboration.ambient_parameters,
    );
    let source_telescope_replayed = entry.candidate_hash == source.candidate_hash
        && elaboration.derivation_hash == source.telescope_elaboration_hash
        && clause.kernel_role == source.kernel_role
        && clause.kernel_ty == source.kernel_type
        && clause.normal_form == source.normal_form;
    let canonical_presentation_replayed = presentation == source.canonical_presentation;
    if !source_telescope_replayed || !canonical_presentation_replayed {
        return Err(ContextualFormationCoherenceV3Error::SealedFamily(
            "typed source does not replay from its exact sealed telescope".to_owned(),
        ));
    }

    let closure = predecessor_closure(signature)
        .map_err(|error| ContextualFormationCoherenceV3Error::SealedFamily(error.to_string()))?;
    let provider = closure
        .families
        .iter()
        .find(|family| {
            family.step == source.step
                && family.clause_index == source.clause_index
                && family.presentation == source.canonical_presentation
        })
        .or_else(|| {
            closure
                .families
                .iter()
                .find(|family| family.presentation == source.canonical_presentation)
        })
        .ok_or_else(|| {
            ContextualFormationCoherenceV3Error::SealedFamily(
                "canonical source has no typed predecessor-closure family".to_owned(),
            )
        })?;
    let canonical_context =
        kernel_context_from_parameter_sorts(&source.canonical_presentation.parameters);
    let canonical_expression = source.canonical_presentation.canonical_normal_form.clone();
    if !is_well_scoped(&canonical_expression, canonical_context.len() as u32) {
        return Err(ContextualFormationCoherenceV3Error::SealedFamily(
            "canonical expression is not scoped by its declared parameter context".to_owned(),
        ));
    }
    let (canonical, derivation) = elaborate_single_clause_with_typed_ambient(
        &canonical_expression,
        &canonical_context,
        &[],
        visible_library,
    )
    .map_err(|error| ContextualFormationCoherenceV3Error::SealedFamily(error.to_string()))?;
    let canonical_typing_replayed =
        canonical.kernel_role == source.kernel_role && canonical.kernel_ty == source.kernel_type;
    let no_charged_or_outside_constructor = !contains_charged_or_outside(&canonical_expression);
    if !canonical_typing_replayed || !no_charged_or_outside_constructor {
        return Err(ContextualFormationCoherenceV3Error::SealedFamily(
            "canonical source typing drifted or uses a charged/outside constructor".to_owned(),
        ));
    }
    let past_only_authority = source.step <= visible_library;
    let mut projection = SealedNaturalFamilyProjectionV3 {
        version: CONTEXTUAL_FORMATION_COHERENCE_V3_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        source: source.clone(),
        predecessor_closure_digest: closure.digest,
        closure_provider_step: provider.step,
        closure_provider_clause: provider.clause_index,
        closure_family_id: provider.id.as_str().to_owned(),
        canonical_context,
        canonical_expression,
        canonical_kernel_role: canonical.kernel_role,
        canonical_kernel_type: canonical.kernel_ty,
        source_telescope_replayed,
        canonical_presentation_replayed,
        canonical_typing_replayed,
        no_charged_or_outside_constructor,
        past_only_authority,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash(
        "sealed-natural-family",
        &(&projection, &canonical.normal_form, &derivation),
    );
    Ok(SealedNaturalFamilyTokenV3 { projection })
}

pub fn replay_sealed_natural_family_v3(
    signature: &SealedSignature,
    projection: &SealedNaturalFamilyProjectionV3,
) -> Result<(), ContextualFormationCoherenceV3Error> {
    let reissued =
        issue_sealed_natural_family_v3(signature, projection.visible_library, &projection.source)?;
    if reissued.projection == *projection {
        Ok(())
    } else {
        Err(ContextualFormationCoherenceV3Error::ReplayMismatch)
    }
}

fn issue_contextual_type_family_closure_impl_v3(
    signature: &SealedSignature,
    visible_library: u32,
    source: &A3TypedClauseSource,
    require_declared_formation: bool,
) -> Result<ContextualFormationClosureTokenV3, ContextualFormationCoherenceV3Error> {
    let sealed = issue_sealed_natural_family_v3(signature, visible_library, source)?;
    let declared_role_is_formation = sealed.projection.canonical_kernel_role
        == ClauseRole::Formation
        && sealed.projection.source.kernel_role == ClauseRole::Formation;
    let contextual_result_is_type = sealed.projection.canonical_kernel_type == KernelTy::Type;
    if !contextual_result_is_type {
        return Err(ContextualFormationCoherenceV3Error::Specialization(
            "source motive lies outside the Formation/Type specialization theorem; a general replayed contextual-motive eliminator is still required"
                .to_owned(),
        ));
    }
    if require_declared_formation && !declared_role_is_formation {
        return Err(ContextualFormationCoherenceV3Error::Formation(
            "sealed contextual source is not an exact Type-forming family".to_owned(),
        ));
    }
    // A typed predecessor family is not an Internal judgment.  Reuse only a
    // proof object issued by the frozen Internal calculus.  We try both its
    // inferred-context and explicit-context entrypoints so unused canonical
    // parameters are not silently erased.  At present both entrypoints reject
    // Formation; that rejection is the honest v3 theorem gap.
    let body = Telescope::new(vec![ClauseRec::new(
        sealed.projection.canonical_kernel_role,
        sealed.projection.canonical_expression.clone(),
    )]);
    let motives = sealed
        .projection
        .canonical_context
        .iter()
        .map(kernel_motive)
        .collect::<Vec<_>>();
    let mut premise_errors = Vec::new();
    let mut internal_premise = None;
    if motives.is_empty() {
        match issue_actual_body_closure_derivation_v2(
            signature,
            &body,
            visible_library,
            0,
            None,
            &std::collections::BTreeMap::new(),
        ) {
            Ok(token) => internal_premise = Some(token.projection().clone()),
            Err(error) => premise_errors.push(format!("closed issuer: {error}")),
        }
    } else {
        match issue_ambient_context_declaration_token(
            signature,
            &body,
            visible_library,
            motives.clone(),
        ) {
            Ok(declaration) => match issue_actual_body_closure_derivation_v2(
                signature,
                &body,
                visible_library,
                0,
                Some(&declaration),
                &std::collections::BTreeMap::new(),
            ) {
                Ok(token) => internal_premise = Some(token.projection().clone()),
                Err(error) => premise_errors.push(format!("inferred-context issuer: {error}")),
            },
            Err(error) => premise_errors.push(format!("inferred-context declaration: {error}")),
        }
        match issue_explicit_ambient_context_declaration_token(
            signature,
            &body,
            visible_library,
            motives,
        ) {
            Ok(declaration) => {
                match issue_explicit_contextual_closure_derivation_v2(signature, &declaration) {
                    Ok(token) => internal_premise = Some(token.projection().clone()),
                    Err(error) => premise_errors.push(format!("explicit-context issuer: {error}")),
                }
            }
            Err(error) => premise_errors.push(format!("explicit-context declaration: {error}")),
        }
    }
    let internal_premise = internal_premise.ok_or_else(|| {
        ContextualFormationCoherenceV3Error::Formation(format!(
            "no genuine contextual Internal premise exists for the sealed Type-family source ({})",
            premise_errors.join("; ")
        ))
    })?;
    replay_verified_closure_derivation_v2(signature, &internal_premise).map_err(|error| {
        ContextualFormationCoherenceV3Error::Formation(format!(
            "Internal premise failed replay: {error}"
        ))
    })?;
    let genuine_internal_premise_replayed = internal_premise.signature_digest == signature.digest()
        && internal_premise.visible_library == visible_library
        && internal_premise.candidate == body
        && internal_premise.clause_index == 0
        && internal_premise.declared_role == sealed.projection.canonical_kernel_role
        && internal_premise.expression == sealed.projection.canonical_expression
        && internal_premise.ambient_arity == sealed.projection.canonical_context.len() as u32
        && internal_premise.inferred_motive == ContextualMotive::Type;
    if !genuine_internal_premise_replayed {
        return Err(ContextualFormationCoherenceV3Error::Formation(
            "replayed Internal premise does not bind the exact contextual Formation judgment"
                .to_owned(),
        ));
    }
    let candidate_fresh_formation_admitted = false;
    let independent_demand_orbit_exported = false;
    let internal_closure_issued = genuine_internal_premise_replayed
        && !candidate_fresh_formation_admitted
        && !independent_demand_orbit_exported;
    let mut projection = ContextualFormationClosureProjectionV3 {
        version: CONTEXTUAL_FORMATION_COHERENCE_V3_VERSION.to_owned(),
        sealed_family: sealed.projection.clone(),
        declared_role_is_formation,
        inferred_motive: ContextualMotive::Type,
        contextual_result_is_type,
        internal_premise,
        genuine_internal_premise_replayed,
        candidate_fresh_formation_admitted,
        independent_demand_orbit_exported,
        internal_closure_issued,
        marginal_nu: 0,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("contextual-formation-closure", &projection);
    Ok(ContextualFormationClosureTokenV3 { projection })
}

/// The new theorem obligation: a declared Formation family may enter only
/// through genuine replayed Internal evidence.  The present frozen calculus
/// rejects that premise, so callers receive a named theorem gap.
pub fn issue_contextual_formation_closure_v3(
    signature: &SealedSignature,
    visible_library: u32,
    source: &A3TypedClauseSource,
) -> Result<ContextualFormationClosureTokenV3, ContextualFormationCoherenceV3Error> {
    issue_contextual_type_family_closure_impl_v3(signature, visible_library, source, true)
}

/// Existing non-Formation Type-family sources need no new Formation rule,
/// but still must replay a genuine frozen Internal derivation.
pub fn issue_contextual_type_family_closure_v3(
    signature: &SealedSignature,
    visible_library: u32,
    source: &A3TypedClauseSource,
) -> Result<ContextualFormationClosureTokenV3, ContextualFormationCoherenceV3Error> {
    issue_contextual_type_family_closure_impl_v3(signature, visible_library, source, false)
}

fn reissue_contextual_formation_closure_v3(
    signature: &SealedSignature,
    projection: &ContextualFormationClosureProjectionV3,
) -> Result<ContextualFormationClosureTokenV3, ContextualFormationCoherenceV3Error> {
    let reissued = if projection.declared_role_is_formation {
        issue_contextual_formation_closure_v3(
            signature,
            projection.sealed_family.visible_library,
            &projection.sealed_family.source,
        )?
    } else {
        issue_contextual_type_family_closure_v3(
            signature,
            projection.sealed_family.visible_library,
            &projection.sealed_family.source,
        )?
    };
    if reissued.projection == *projection {
        Ok(reissued)
    } else {
        Err(ContextualFormationCoherenceV3Error::ReplayMismatch)
    }
}

pub fn replay_contextual_formation_closure_v3(
    signature: &SealedSignature,
    projection: &ContextualFormationClosureProjectionV3,
) -> Result<(), ContextualFormationCoherenceV3Error> {
    reissue_contextual_formation_closure_v3(signature, projection).map(|_| ())
}

fn issue_first_image(
    signature: &SealedSignature,
    visible_library: u32,
    target_context: &[KernelTy],
    origin: &FirstImageOriginV3,
) -> Result<(Expr, ContextualAssignmentImageOriginV3), ContextualFormationCoherenceV3Error> {
    match origin {
        FirstImageOriginV3::DirectSealedTypeFamily { source } => {
            let evidence = issue_sealed_natural_family_v3(signature, visible_library, source)?;
            if evidence.projection.canonical_kernel_type != KernelTy::Type {
                return Err(ContextualFormationCoherenceV3Error::Assignment(
                    "direct first image is not a sealed Type family".to_owned(),
                ));
            }
            Ok((
                evidence.projection.canonical_expression.clone(),
                ContextualAssignmentImageOriginV3::DirectSealedTypeFamily {
                    evidence: evidence.projection.clone(),
                },
            ))
        }
        FirstImageOriginV3::PointwiseSealedTypeValuedFamily {
            source,
            argument_parameter,
        } => {
            let evidence = issue_sealed_natural_family_v3(signature, visible_library, source)?;
            let KernelTy::Fun(_, codomain) = &evidence.projection.canonical_kernel_type else {
                return Err(ContextualFormationCoherenceV3Error::Assignment(
                    "pointwise first image source is not a sealed function family".to_owned(),
                ));
            };
            if codomain.as_ref() != &KernelTy::Type
                || *argument_parameter == 0
                || *argument_parameter > target_context.len() as u32
            {
                return Err(ContextualFormationCoherenceV3Error::Assignment(
                    "pointwise first image is not Type-valued over a live target parameter"
                        .to_owned(),
                ));
            }
            Ok((
                Expr::App(
                    Box::new(evidence.projection.canonical_expression.clone()),
                    Box::new(Expr::Var(*argument_parameter)),
                ),
                ContextualAssignmentImageOriginV3::PointwiseSealedTypeValuedFamily {
                    evidence: evidence.projection.clone(),
                    argument_parameter: *argument_parameter,
                },
            ))
        }
    }
}

pub fn issue_ordered_open_assignment_v3(
    signature: &SealedSignature,
    visible_library: u32,
    source_context: Vec<KernelTy>,
    target_context: Vec<KernelTy>,
    first_origin: FirstImageOriginV3,
    internal_evidence: Vec<VerifiedClosureDerivationV2>,
) -> Result<OrderedOpenAssignmentTokenV3, ContextualFormationCoherenceV3Error> {
    if source_context.is_empty()
        || target_context.len() < source_context.len()
        || source_context[0] != KernelTy::Type
    {
        return Err(ContextualFormationCoherenceV3Error::Assignment(
            "identity-tail specialization needs a Type first parameter and a target at least as large as the source"
                .to_owned(),
        ));
    }
    if internal_evidence.len() != source_context.len() {
        return Err(ContextualFormationCoherenceV3Error::Assignment(format!(
            "genuine contextual Internal evidence is required for every image: received {}, expected {}",
            internal_evidence.len(),
            source_context.len()
        )));
    }
    let (first_term, first_image_origin) =
        issue_first_image(signature, visible_library, &target_context, &first_origin)?;
    let mut images = Vec::with_capacity(source_context.len());
    for (index, (source_classifier, internal_evidence)) in
        source_context.iter().zip(internal_evidence).enumerate()
    {
        let source_parameter = index as u32 + 1;
        let (term, origin) = if index == 0 {
            (first_term.clone(), first_image_origin.clone())
        } else {
            (
                Expr::Var(source_parameter),
                ContextualAssignmentImageOriginV3::OrderedIdentityTail {
                    target_parameter: source_parameter,
                },
            )
        };
        if !is_well_scoped(&term, target_context.len() as u32) {
            return Err(ContextualFormationCoherenceV3Error::Assignment(format!(
                "image {source_parameter} is outside the target context"
            )));
        }
        let (typed, derivation) = elaborate_single_clause_with_typed_ambient(
            &term,
            &target_context,
            &[],
            visible_library,
        )
        .map_err(|error| ContextualFormationCoherenceV3Error::Assignment(error.to_string()))?;
        let exact_classifier_preserved = typed.kernel_ty == *source_classifier;
        if !exact_classifier_preserved {
            return Err(ContextualFormationCoherenceV3Error::Assignment(format!(
                "image {source_parameter} has classifier {:?}, expected {:?}",
                typed.kernel_ty, source_classifier
            )));
        }
        replay_verified_closure_derivation_v2(signature, &internal_evidence).map_err(|error| {
            ContextualFormationCoherenceV3Error::Assignment(format!(
                "image {source_parameter} Internal evidence failed replay: {error}"
            ))
        })?;
        let genuine_internal_evidence_replayed = internal_evidence.signature_digest
            == signature.digest()
            && internal_evidence.visible_library == visible_library
            && internal_evidence.ambient_arity == target_context.len() as u32
            && internal_evidence.clause_index == 0
            && internal_evidence.expression == term
            && internal_evidence.inferred_motive == kernel_motive(source_classifier)
            && internal_evidence.candidate.kappa() == 1
            && internal_evidence.candidate.clauses[0].expr == term;
        if !genuine_internal_evidence_replayed {
            return Err(ContextualFormationCoherenceV3Error::Assignment(format!(
                "image {source_parameter} Internal evidence does not bind the exact open image judgment"
            )));
        }
        let typing_derivation_hash = tagged_hash(
            "assignment-image-typing",
            &(&term, &target_context, &typed, &derivation),
        );
        let mut image = ContextualAssignmentImageProjectionV3 {
            source_parameter,
            source_classifier: source_classifier.clone(),
            term,
            target_classifier: typed.kernel_ty,
            target_well_scoped: true,
            exact_classifier_preserved,
            internal_evidence,
            genuine_internal_evidence_replayed,
            origin,
            typing_derivation_hash,
            derivation_hash: String::new(),
        };
        image.derivation_hash = tagged_hash("assignment-image", &image);
        images.push(image);
    }
    let ordered_identity_tail_exact = images.iter().skip(1).all(|image| {
        image.term == Expr::Var(image.source_parameter)
            && matches!(
                image.origin,
                ContextualAssignmentImageOriginV3::OrderedIdentityTail { target_parameter }
                    if target_parameter == image.source_parameter
            )
    });
    let every_image_motive_typed = images
        .iter()
        .all(|image| image.target_well_scoped && image.exact_classifier_preserved);
    let every_image_genuine_internal_evidence_replayed = images
        .iter()
        .all(|image| image.genuine_internal_evidence_replayed);
    let first_image_genuine_internal_evidence_replayed =
        images[0].genuine_internal_evidence_replayed;
    let mut projection = OrderedOpenAssignmentProjectionV3 {
        version: CONTEXTUAL_FORMATION_COHERENCE_V3_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        source_arity: source_context.len() as u32,
        target_arity: target_context.len() as u32,
        source_context,
        target_context,
        images,
        first_image_genuine_internal_evidence_replayed,
        ordered_identity_tail_exact,
        no_permutation_or_instance_override: true,
        open_target_context_permitted: true,
        every_image_motive_typed,
        every_image_genuine_internal_evidence_replayed,
        assignment_hash: String::new(),
    };
    projection.assignment_hash = tagged_hash("ordered-open-assignment", &projection);
    Ok(OrderedOpenAssignmentTokenV3 {
        projection,
        first_origin,
    })
}

fn first_origin_from_projection(
    projection: &OrderedOpenAssignmentProjectionV3,
) -> Result<FirstImageOriginV3, ContextualFormationCoherenceV3Error> {
    let first = projection.images.first().ok_or_else(|| {
        ContextualFormationCoherenceV3Error::Assignment("assignment has no first image".to_owned())
    })?;
    match &first.origin {
        ContextualAssignmentImageOriginV3::DirectSealedTypeFamily { evidence } => {
            Ok(FirstImageOriginV3::DirectSealedTypeFamily {
                source: evidence.source.clone(),
            })
        }
        ContextualAssignmentImageOriginV3::PointwiseSealedTypeValuedFamily {
            evidence,
            argument_parameter,
        } => Ok(FirstImageOriginV3::PointwiseSealedTypeValuedFamily {
            source: evidence.source.clone(),
            argument_parameter: *argument_parameter,
        }),
        ContextualAssignmentImageOriginV3::OrderedIdentityTail { .. } => {
            Err(ContextualFormationCoherenceV3Error::Assignment(
                "first image cannot be an identity-tail image".to_owned(),
            ))
        }
    }
}

fn reissue_ordered_open_assignment_v3(
    signature: &SealedSignature,
    projection: &OrderedOpenAssignmentProjectionV3,
) -> Result<OrderedOpenAssignmentTokenV3, ContextualFormationCoherenceV3Error> {
    let first_origin = first_origin_from_projection(projection)?;
    let reissued = issue_ordered_open_assignment_v3(
        signature,
        projection.visible_library,
        projection.source_context.clone(),
        projection.target_context.clone(),
        first_origin,
        projection
            .images
            .iter()
            .map(|image| image.internal_evidence.clone())
            .collect(),
    )?;
    if reissued.projection == *projection {
        Ok(reissued)
    } else {
        Err(ContextualFormationCoherenceV3Error::ReplayMismatch)
    }
}

pub fn replay_ordered_open_assignment_v3(
    signature: &SealedSignature,
    projection: &OrderedOpenAssignmentProjectionV3,
) -> Result<(), ContextualFormationCoherenceV3Error> {
    reissue_ordered_open_assignment_v3(signature, projection).map(|_| ())
}

pub fn specialize_contextual_formation_v3(
    signature: &SealedSignature,
    source: &ContextualFormationClosureTokenV3,
    assignment: &OrderedOpenAssignmentTokenV3,
) -> Result<OpenFormationSpecializationTokenV3, ContextualFormationCoherenceV3Error> {
    replay_contextual_formation_closure_v3(signature, source.projection())?;
    replay_ordered_open_assignment_v3(signature, assignment.projection())?;
    let source_family = &source.projection.sealed_family;
    if source_family.signature_digest != assignment.projection.signature_digest
        || source_family.visible_library != assignment.projection.visible_library
        || source_family.canonical_context != assignment.projection.source_context
    {
        return Err(ContextualFormationCoherenceV3Error::Specialization(
            "assignment is not attached to the exact source Formation context".to_owned(),
        ));
    }
    let substitution_images = assignment
        .projection
        .images
        .iter()
        .map(|image| SubstitutionImage {
            source_parameter: image.source_parameter,
            term: image.term.clone(),
        })
        .collect::<Vec<_>>();
    let substitution = issue_structural_substitution(
        structural_context(&assignment.projection.source_context),
        structural_context(&assignment.projection.target_context),
        substitution_images.clone(),
        source_family.canonical_expression.clone(),
    )
    .map_err(|error| ContextualFormationCoherenceV3Error::Specialization(error.to_string()))?;
    replay_structural_substitution(&substitution)
        .map_err(|error| ContextualFormationCoherenceV3Error::Specialization(error.to_string()))?;
    let substitution_result = substitution.result().clone();
    let (source_typed, source_derivation) = elaborate_single_clause_with_typed_ambient(
        &source_family.canonical_expression,
        &assignment.projection.source_context,
        &[],
        source_family.visible_library,
    )
    .map_err(|error| ContextualFormationCoherenceV3Error::Specialization(error.to_string()))?;
    let (result_typed, result_derivation) = elaborate_single_clause_with_typed_ambient(
        &substitution_result,
        &assignment.projection.target_context,
        &[],
        source_family.visible_library,
    )
    .map_err(|error| ContextualFormationCoherenceV3Error::Specialization(error.to_string()))?;
    let exact_kernel_type_preserved = source_typed.kernel_ty == KernelTy::Type
        && result_typed.kernel_ty == source_typed.kernel_ty
        && result_typed.kernel_role == ClauseRole::Formation;
    if !exact_kernel_type_preserved
        || !is_well_scoped(
            &substitution_result,
            assignment.projection.target_context.len() as u32,
        )
    {
        return Err(ContextualFormationCoherenceV3Error::Specialization(
            "open substitution did not preserve an exact well-scoped Formation judgment".to_owned(),
        ));
    }
    let normal_form_substitution = issue_structural_substitution(
        structural_context(&assignment.projection.source_context),
        structural_context(&assignment.projection.target_context),
        substitution_images.clone(),
        source_typed.normal_form.clone(),
    )
    .map_err(|error| ContextualFormationCoherenceV3Error::Specialization(error.to_string()))?;
    replay_structural_substitution(&normal_form_substitution)
        .map_err(|error| ContextualFormationCoherenceV3Error::Specialization(error.to_string()))?;
    let normalization_commutes = univalent_equality(
        normal_form_substitution.result(),
        &result_typed.normal_form,
        assignment.projection.target_arity,
        NORMALIZATION_FUEL,
    )
    .map_err(|error| ContextualFormationCoherenceV3Error::Specialization(error.to_string()))?;
    if !normalization_commutes.equal {
        return Err(ContextualFormationCoherenceV3Error::Specialization(
            "normalization does not commute with the exact open substitution".to_owned(),
        ));
    }
    let source_typing_derivation_hash = tagged_hash(
        "specialization-source-typing",
        &(&source_typed, &source_derivation),
    );
    let result_typing_derivation_hash = tagged_hash(
        "specialization-result-typing",
        &(&result_typed, &result_derivation),
    );
    let mut specialized_relation = DerivedContextualFormationProjectionV3 {
        expression: substitution_result.clone(),
        target_context: assignment.projection.target_context.clone(),
        inferred_motive: ContextualMotive::Type,
        kernel_type: result_typed.kernel_ty.clone(),
        kernel_role: result_typed.kernel_role,
        normal_form: result_typed.normal_form.clone(),
        well_scoped_in_target: true,
        contextual_typing_replayed: true,
        source_internal_replayed: source.projection.genuine_internal_premise_replayed,
        every_assignment_image_internal_replayed: assignment
            .projection
            .images
            .iter()
            .all(|image| image.genuine_internal_evidence_replayed),
        substitution_stability_replayed: normalization_commutes.equal,
        candidate_fresh_formation_admitted: false,
        internal_closure_issued: source.projection.genuine_internal_premise_replayed
            && assignment
                .projection
                .images
                .iter()
                .all(|image| image.genuine_internal_evidence_replayed)
            && normalization_commutes.equal,
        marginal_nu: 0,
        derivation_hash: String::new(),
    };
    specialized_relation.derivation_hash =
        tagged_hash("derived-contextual-formation", &specialized_relation);
    let exact_result_replayed = specialized_relation.expression == substitution_result;
    let mut projection = OpenFormationSpecializationProjectionV3 {
        version: CONTEXTUAL_FORMATION_COHERENCE_V3_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library: source_family.visible_library,
        source: source.projection.clone(),
        assignment: assignment.projection.clone(),
        substitution_body: source_family.canonical_expression.clone(),
        substitution_images,
        substitution_result,
        substitution_derivation_hash: substitution.derivation_hash().to_owned(),
        structural_substitution_replayed: true,
        source_typing_derivation_hash,
        result_typing_derivation_hash,
        exact_kernel_type_preserved,
        exact_result_replayed,
        source_normal_form_substitution_result: normal_form_substitution.result().clone(),
        normalization_commutes,
        specialized_relation,
        target_may_remain_open: true,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("open-formation-specialization", &projection);
    Ok(OpenFormationSpecializationTokenV3 { projection })
}

pub fn replay_open_formation_specialization_v3(
    signature: &SealedSignature,
    projection: &OpenFormationSpecializationProjectionV3,
) -> Result<(), ContextualFormationCoherenceV3Error> {
    let source = reissue_contextual_formation_closure_v3(signature, &projection.source)?;
    let assignment = reissue_ordered_open_assignment_v3(signature, &projection.assignment)?;
    let reissued = specialize_contextual_formation_v3(signature, &source, &assignment)?;
    if reissued.projection == *projection {
        Ok(())
    } else {
        Err(ContextualFormationCoherenceV3Error::ReplayMismatch)
    }
}
