//! Proof-strength motive-parametric specialization for exact closure judgments.
//!
//! This successor deliberately has no representative theorem and no generic
//! boolean flags.  Its theorem is the total, exhaustive eliminator
//! [`specialize_verified_closure_derivation_v2`]: a caller must first supply
//! one exact replayable source derivation and one exact assignment whose
//! images carry closed `Internal` derivations.  Each of the six closure cases
//! then reissues a closed rule relation for the specialized expression.

use crate::ambient_former_internality::{
    AmbientFormerClosureProjection, issue_ambient_former_closure_token,
    replay_ambient_former_closure_projection,
};
use crate::certified_field_dereference::{
    CertifiedFieldDereferenceProjection, CertifiedPriorField,
    issue_certified_field_dereference_token, replay_certified_field_dereference_projection,
};
use crate::contextual_internality::{
    AmbientContextDeclarationProjection, AmbientContextDeclarationToken,
    AmbientHypothesisProjection, ContextualMotive, ContextualTypedStructureProjection,
    ExplicitAmbientContextDeclarationProjection, ExplicitAmbientContextDeclarationToken,
    ExplicitContextualTypedStructureProjection, issue_ambient_context_declaration_token,
    issue_contextual_typed_structure_token, issue_explicit_ambient_context_declaration_token,
    issue_explicit_contextual_typed_structure_token, replay_ambient_context_declaration_projection,
    replay_contextual_typed_structure_projection,
    replay_explicit_ambient_context_declaration_projection,
    replay_explicit_contextual_typed_structure_projection,
};
use crate::elaborate::{
    DerivationNode, KernelTy, SealedSignature, candidate_hash,
    elaborate_single_clause_with_derivation, elaborate_telescope,
};
use crate::guarded_internality::{
    GuardedClauseWeakeningErasureProjection, issue_guarded_clause_weakening_erasure_token,
    replay_guarded_clause_weakening_erasure_projection, weaken_by_one_ambient,
};
use crate::motive_parametric_coherence::ClosureRuleKind;
use crate::structural_internality::{
    StructuralLambdaClosureProjection, issue_structural_lambda_closure_token,
    replay_structural_lambda_closure_projection,
};
use crate::substitution::{
    SortedParameterContext, SubstitutionImage, is_well_scoped, issue_structural_substitution,
    replay_structural_substitution,
};
use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use thiserror::Error;

pub const MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION: &str =
    "motive-parametric-instantiation-coherence-v2";

/// The eliminator's compile-time case inventory.  This is an API contract,
/// not evidence inferred from representative witnesses.
pub const CLOSURE_RULE_INVENTORY_V2: [ClosureRuleKind; 6] = [
    ClosureRuleKind::Projection,
    ClosureRuleKind::Guarded,
    ClosureRuleKind::Structural,
    ClosureRuleKind::AmbientFormer,
    ClosureRuleKind::Dereference,
    ClosureRuleKind::Contextual,
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum KernelTyProjectionV2 {
    Type,
    Element(Expr),
    Function {
        domain: Box<KernelTyProjectionV2>,
        codomain: Box<KernelTyProjectionV2>,
    },
    PathDeclaration {
        dimension: u32,
    },
    Neutral,
}

impl From<&KernelTy> for KernelTyProjectionV2 {
    fn from(value: &KernelTy) -> Self {
        match value {
            KernelTy::Type => Self::Type,
            KernelTy::El(expression) => Self::Element(expression.clone()),
            KernelTy::Fun(domain, codomain) => Self::Function {
                domain: Box::new(Self::from(domain.as_ref())),
                codomain: Box::new(Self::from(codomain.as_ref())),
            },
            KernelTy::PathDecl { dimension } => Self::PathDeclaration {
                dimension: *dimension,
            },
            KernelTy::Neutral => Self::Neutral,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionSourceEvidenceV2 {
    AmbientParameter {
        parameter: u32,
        declaration: AmbientContextDeclarationProjection,
    },
    CertifiedPriorField {
        clause_index: u16,
        prior_derivation: Box<VerifiedClosureDerivationV2>,
        ambient_declaration: Option<AmbientContextDeclarationProjection>,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosureRuleEvidenceV2 {
    Projection {
        source: ProjectionSourceEvidenceV2,
        leaf_rule: String,
    },
    Guarded {
        base_derivation: Box<VerifiedClosureDerivationV2>,
        ambient_declaration: AmbientContextDeclarationProjection,
        weakening_erasure: GuardedClauseWeakeningErasureProjection,
    },
    Structural {
        closure: StructuralLambdaClosureProjection,
    },
    AmbientFormer {
        closure: AmbientFormerClosureProjection,
        prior_derivations: BTreeMap<u16, Box<VerifiedClosureDerivationV2>>,
    },
    Dereference {
        closure: CertifiedFieldDereferenceProjection,
        prior_derivations: BTreeMap<u16, Box<VerifiedClosureDerivationV2>>,
    },
    Contextual {
        typed_structure: ContextualTypedStructureProjection,
        prior_derivations: BTreeMap<u16, Box<VerifiedClosureDerivationV2>>,
    },
    ExplicitContextual {
        typed_structure: ExplicitContextualTypedStructureProjection,
    },
}

impl ClosureRuleEvidenceV2 {
    fn kind(&self) -> ClosureRuleKind {
        match self {
            Self::Projection { .. } => ClosureRuleKind::Projection,
            Self::Guarded { .. } => ClosureRuleKind::Guarded,
            Self::Structural { .. } => ClosureRuleKind::Structural,
            Self::AmbientFormer { .. } => ClosureRuleKind::AmbientFormer,
            Self::Dereference { .. } => ClosureRuleKind::Dereference,
            Self::Contextual { .. } => ClosureRuleKind::Contextual,
            Self::ExplicitContextual { .. } => ClosureRuleKind::Contextual,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct VerifiedClosureDerivationV2 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub candidate: Telescope,
    pub candidate_hash: String,
    pub elaboration_hash: String,
    pub ambient_arity: u32,
    pub clause_index: u16,
    pub declared_role: ClauseRole,
    pub expression: Expr,
    pub normal_form: Expr,
    pub kernel_ty: KernelTyProjectionV2,
    pub inferred_motive: ContextualMotive,
    pub rule: ClosureRuleKind,
    pub evidence: ClosureRuleEvidenceV2,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedClosureTokenV2 {
    projection: VerifiedClosureDerivationV2,
}

impl VerifiedClosureTokenV2 {
    pub fn projection(&self) -> &VerifiedClosureDerivationV2 {
        &self.projection
    }

    pub fn derivation_hash(&self) -> &str {
        &self.projection.derivation_hash
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedInternalEvidenceV2 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub expression: Expr,
    pub inferred_motive: ContextualMotive,
    pub relation: Box<VerifiedClosureDerivationV2>,
    pub relation_replayed: bool,
    pub closed_candidate: bool,
    pub evidence_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ClosedInternalEvidenceTokenV2 {
    projection: ClosedInternalEvidenceV2,
}

impl ClosedInternalEvidenceTokenV2 {
    pub fn projection(&self) -> &ClosedInternalEvidenceV2 {
        &self.projection
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotiveTypedClosedImageProjectionV2 {
    pub parameter: u32,
    pub motive: ContextualMotive,
    pub term: Expr,
    pub evidence: ClosedInternalEvidenceV2,
    pub evidence_replayed: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotiveTypedClosedAssignmentProjectionV2 {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub source_arity: u32,
    pub images: Vec<MotiveTypedClosedImageProjectionV2>,
    pub assignment_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MotiveTypedClosedAssignmentTokenV2 {
    projection: MotiveTypedClosedAssignmentProjectionV2,
}

impl MotiveTypedClosedAssignmentTokenV2 {
    pub fn projection(&self) -> &MotiveTypedClosedAssignmentProjectionV2 {
        &self.projection
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SubstitutionImageProjectionV2 {
    pub source_parameter: u32,
    pub term: Expr,
}

impl From<&SubstitutionImage> for SubstitutionImageProjectionV2 {
    fn from(value: &SubstitutionImage) -> Self {
        Self {
            source_parameter: value.source_parameter,
            term: value.term.clone(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralSubstitutionProjectionV2 {
    pub source_arity: u32,
    pub target_arity: u32,
    pub images: Vec<SubstitutionImageProjectionV2>,
    pub body: Expr,
    pub result: Expr,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpecializedClosureDerivationV2 {
    pub version: String,
    pub source_derivation: Box<VerifiedClosureDerivationV2>,
    pub assignment: MotiveTypedClosedAssignmentProjectionV2,
    pub substitution: StructuralSubstitutionProjectionV2,
    pub specialized_candidate: Telescope,
    pub specialized_expression: Expr,
    pub specialized_normal_form: Expr,
    pub specialized_kernel_ty: KernelTyProjectionV2,
    pub specialized_motive: ContextualMotive,
    pub specialized_relation: ClosedInternalEvidenceV2,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SpecializedClosureTokenV2 {
    projection: SpecializedClosureDerivationV2,
}

impl SpecializedClosureTokenV2 {
    pub fn projection(&self) -> &SpecializedClosureDerivationV2 {
        &self.projection
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum MotiveParametricCoherenceV2Error {
    #[error("underlying {component} issuer/replay failed: {reason}")]
    Underlying { component: String, reason: String },
    #[error("clause index {clause_index} is out of range")]
    ClauseOutOfRange { clause_index: u16 },
    #[error("closure rule tag and evidence variant disagree")]
    RuleEvidenceMismatch,
    #[error("source is not an exact ambient or prior-field projection leaf")]
    NotProjectionLeaf,
    #[error("projection leaf rule {found} differs from expected rule {expected}")]
    ProjectionLeafRule { expected: String, found: String },
    #[error("ambient declaration is not attached to the exact source candidate")]
    DeclarationCandidateMismatch,
    #[error("ambient declaration motive disagrees with parameter {parameter}")]
    DeclarationMotiveMismatch { parameter: u32 },
    #[error("prior evidence for clause {clause_index} is not the exact certified prefix judgment")]
    PriorEvidenceMismatch { clause_index: u16 },
    #[error("prior evidence for clause {clause_index} is not strictly prior to {source_clause}")]
    PriorEvidenceNotStrict {
        clause_index: u16,
        source_clause: u16,
    },
    #[error(
        "prior evidence for clause {clause_index} has ambient arity {prior_ambient}, but the source candidate has {source_ambient}; fixed-ambient prefix transport is required"
    )]
    PriorAmbientMismatch {
        clause_index: u16,
        prior_ambient: u32,
        source_ambient: u32,
    },
    #[error(
        "prior prefix through clause {clause_index} does not replay with the same kernel derivations inside the source candidate"
    )]
    PriorEmbeddingMismatch { clause_index: u16 },
    #[error("supplied prior inventory differs from the exact field references used by the clause")]
    PriorInventoryMismatch,
    #[error("prior ambient declaration is not the exact motive-prefix of the source declaration")]
    PriorDeclarationMismatch,
    #[error("guarded base derivation does not match the exact weakening/erasure source")]
    GuardedBaseMismatch,
    #[error("closure derivation replay mismatch")]
    ClosureReplayMismatch,
    #[error("closed Internal evidence was requested for ambient arity {ambient}")]
    EvidenceNotClosed { ambient: u32 },
    #[error("closed Internal evidence replay mismatch")]
    ClosedEvidenceReplayMismatch,
    #[error("assignment arity mismatch: motives={motives}, evidence={evidence}")]
    AssignmentArity { motives: u32, evidence: u32 },
    #[error("assignment image {parameter} has motive inconsistent with its replayed evidence")]
    AssignmentMotiveMismatch { parameter: u32 },
    #[error("assignment replay mismatch")]
    AssignmentReplayMismatch,
    #[error("assignment does not match the exact source ambient declaration")]
    AssignmentDeclarationMismatch,
    #[error("structural substitution failed: {0}")]
    Substitution(String),
    #[error("structural substitution projection replay mismatch")]
    SubstitutionReplayMismatch,
    #[error("F-M1 substitution-stability failure at rule {rule:?}: {reason}")]
    SubstitutionStability {
        rule: ClosureRuleKind,
        reason: String,
    },
    #[error("specialized closure derivation replay mismatch")]
    SpecializedReplayMismatch,
    #[error("actual-body issuance requires an exact ambient declaration")]
    MissingAmbientDeclaration,
    #[error("actual-body expression is outside the six replayable closure issuers: {0}")]
    ActualBodyUnsupported(String),
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION, domain, value))
        .expect("motive-parametric v2 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn underlying(component: &str, error: impl std::fmt::Display) -> MotiveParametricCoherenceV2Error {
    MotiveParametricCoherenceV2Error::Underlying {
        component: component.to_owned(),
        reason: error.to_string(),
    }
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

fn weaken_motive_by_one_ambient(motive: &ContextualMotive) -> ContextualMotive {
    match motive {
        ContextualMotive::Type => ContextualMotive::Type,
        ContextualMotive::Element(expression) => {
            ContextualMotive::Element(weaken_by_one_ambient(expression))
        }
        ContextualMotive::Function { domain, codomain } => ContextualMotive::Function {
            domain: Box::new(weaken_motive_by_one_ambient(domain)),
            codomain: Box::new(weaken_motive_by_one_ambient(codomain)),
        },
        ContextualMotive::Neutral => ContextualMotive::Neutral,
    }
}

fn parse_rule_index(rule: &str, prefix: &str) -> Option<u32> {
    rule.strip_prefix(prefix)?.parse().ok()
}

fn collect_field_dependencies(derivation: &DerivationNode, dependencies: &mut BTreeSet<u16>) {
    if let Some(field) = parse_rule_index(&derivation.rule, "field-ref-") {
        if let Ok(field) = u16::try_from(field) {
            dependencies.insert(field);
        }
    }
    for child in &derivation.children {
        collect_field_dependencies(child, dependencies);
    }
}

fn exact_field_dependencies(derivation: &DerivationNode) -> BTreeSet<u16> {
    let mut dependencies = BTreeSet::new();
    collect_field_dependencies(derivation, &mut dependencies);
    dependencies
}

fn candidate_prefix(candidate: &Telescope, clause_index: u16) -> Option<Telescope> {
    let end = usize::from(clause_index).checked_add(1)?;
    Some(Telescope::new(candidate.clauses.get(..end)?.to_vec()))
}

fn finish_verified(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    inferred_motive: ContextualMotive,
    rule: ClosureRuleKind,
    evidence: ClosureRuleEvidenceV2,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    if evidence.kind() != rule {
        return Err(MotiveParametricCoherenceV2Error::RuleEvidenceMismatch);
    }
    let elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| underlying("elaboration", error))?;
    let clause = candidate
        .clauses
        .get(usize::from(clause_index))
        .ok_or(MotiveParametricCoherenceV2Error::ClauseOutOfRange { clause_index })?;
    let clause_elaboration = elaboration
        .clauses
        .get(usize::from(clause_index))
        .ok_or(MotiveParametricCoherenceV2Error::ClauseOutOfRange { clause_index })?;
    let mut projection = VerifiedClosureDerivationV2 {
        version: MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        candidate: candidate.clone(),
        candidate_hash: candidate_hash(candidate),
        elaboration_hash: elaboration.derivation_hash,
        ambient_arity: elaboration.ambient_parameters,
        clause_index,
        declared_role: clause.role,
        expression: clause.expr.clone(),
        normal_form: clause_elaboration.normal_form.clone(),
        kernel_ty: KernelTyProjectionV2::from(&clause_elaboration.kernel_ty),
        inferred_motive,
        rule,
        evidence,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("verified-closure-derivation", &projection);
    Ok(VerifiedClosureTokenV2 { projection })
}

fn declaration_token_from_projection(
    signature: &SealedSignature,
    candidate: &Telescope,
    projection: &AmbientContextDeclarationProjection,
) -> Result<AmbientContextDeclarationToken, MotiveParametricCoherenceV2Error> {
    let token = issue_ambient_context_declaration_token(
        signature,
        candidate,
        projection.visible_library,
        projection
            .hypotheses
            .iter()
            .map(|hypothesis| hypothesis.motive.clone())
            .collect(),
    )
    .map_err(|error| underlying("ambient-context declaration", error))?;
    if token.projection() != projection {
        return Err(MotiveParametricCoherenceV2Error::DeclarationCandidateMismatch);
    }
    replay_ambient_context_declaration_projection(
        signature,
        candidate,
        projection.visible_library,
        projection,
    )
    .map_err(|error| underlying("ambient-context declaration replay", error))?;
    Ok(token)
}

fn explicit_declaration_token_from_projection(
    signature: &SealedSignature,
    projection: &ExplicitAmbientContextDeclarationProjection,
) -> Result<ExplicitAmbientContextDeclarationToken, MotiveParametricCoherenceV2Error> {
    let token = issue_explicit_ambient_context_declaration_token(
        signature,
        &projection.body_telescope,
        projection.visible_library,
        projection
            .hypotheses
            .iter()
            .map(|hypothesis| hypothesis.motive.clone())
            .collect(),
    )
    .map_err(|error| underlying("explicit ambient-context declaration", error))?;
    if token.projection() != projection {
        return Err(MotiveParametricCoherenceV2Error::DeclarationCandidateMismatch);
    }
    replay_explicit_ambient_context_declaration_projection(signature, projection)
        .map_err(|error| underlying("explicit ambient-context declaration replay", error))?;
    Ok(token)
}

fn validate_declaration_attachment(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    declaration: &AmbientContextDeclarationToken,
) -> Result<(), MotiveParametricCoherenceV2Error> {
    if declaration.projection().signature_digest != signature.digest()
        || declaration.projection().visible_library != visible_library
        || declaration.projection().candidate_hash != candidate_hash(candidate)
    {
        return Err(MotiveParametricCoherenceV2Error::DeclarationCandidateMismatch);
    }
    replay_ambient_context_declaration_projection(
        signature,
        candidate,
        visible_library,
        declaration.projection(),
    )
    .map_err(|error| underlying("ambient-context declaration replay", error))
}

fn validate_prior_derivations(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    source_clause: u16,
    priors: &BTreeMap<u16, Box<VerifiedClosureDerivationV2>>,
) -> Result<BTreeMap<u16, String>, MotiveParametricCoherenceV2Error> {
    let source_elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| underlying("prior source elaboration", error))?;
    let source_ambient = source_elaboration.ambient_parameters;
    let mut hashes = BTreeMap::new();
    for (clause_index, prior) in priors {
        if *clause_index >= source_clause {
            return Err(MotiveParametricCoherenceV2Error::PriorEvidenceNotStrict {
                clause_index: *clause_index,
                source_clause,
            });
        }
        let expected_prefix = candidate_prefix(candidate, *clause_index).ok_or(
            MotiveParametricCoherenceV2Error::PriorEvidenceMismatch {
                clause_index: *clause_index,
            },
        )?;
        let expected_clause = expected_prefix
            .clauses
            .get(usize::from(*clause_index))
            .expect("prefix contains terminal clause");
        if prior.visible_library != visible_library
            || prior.signature_digest != signature.digest()
            || prior.clause_index != *clause_index
            || prior.candidate != expected_prefix
            || prior.expression != expected_clause.expr
        {
            return Err(MotiveParametricCoherenceV2Error::PriorEvidenceMismatch {
                clause_index: *clause_index,
            });
        }
        if prior.ambient_arity > source_ambient {
            return Err(MotiveParametricCoherenceV2Error::PriorAmbientMismatch {
                clause_index: *clause_index,
                prior_ambient: prior.ambient_arity,
                source_ambient,
            });
        }
        let prefix_elaboration = elaborate_telescope(signature, &expected_prefix, visible_library)
            .map_err(|error| underlying("prior prefix elaboration", error))?;
        let prefix_len = usize::from(*clause_index) + 1;
        if source_elaboration.clauses.get(..prefix_len)
            != prefix_elaboration.clauses.get(..prefix_len)
        {
            return Err(MotiveParametricCoherenceV2Error::PriorEmbeddingMismatch {
                clause_index: *clause_index,
            });
        }
        reissue_verified_closure_derivation_v2(signature, prior)?;
        hashes.insert(*clause_index, prior.derivation_hash.clone());
    }
    Ok(hashes)
}

fn boxed_priors(
    priors: &BTreeMap<u16, VerifiedClosureDerivationV2>,
) -> BTreeMap<u16, Box<VerifiedClosureDerivationV2>> {
    priors
        .iter()
        .map(|(clause, evidence)| (*clause, Box::new(evidence.clone())))
        .collect()
}

pub fn issue_ambient_projection_closure_derivation_v2(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    declaration: &AmbientContextDeclarationToken,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    // An assignment image carries a judgment in its own singleton witness.
    // Without an explicit candidate-extension transport theorem it can only
    // discharge the exact singleton body judgment, never a field embedded in
    // a larger telescope.
    if clause_index != 0 || candidate.kappa() != 1 {
        return Err(MotiveParametricCoherenceV2Error::ActualBodyUnsupported(
            "ambient projection specialization is certified only for an exact singleton body telescope"
                .to_owned(),
        ));
    }
    validate_declaration_attachment(signature, candidate, visible_library, declaration)?;
    let elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| underlying("ambient projection elaboration", error))?;
    let clause = candidate
        .clauses
        .get(usize::from(clause_index))
        .ok_or(MotiveParametricCoherenceV2Error::ClauseOutOfRange { clause_index })?;
    if clause.role == ClauseRole::Formation {
        return Err(MotiveParametricCoherenceV2Error::ActualBodyUnsupported(
            "ambient projection Formation has no closed zero-credit target relation".to_owned(),
        ));
    }
    let clause_elaboration = elaboration
        .clauses
        .get(usize::from(clause_index))
        .ok_or(MotiveParametricCoherenceV2Error::ClauseOutOfRange { clause_index })?;
    if !matches!(clause.expr, Expr::Var(_)) || !clause_elaboration.derivation.children.is_empty() {
        return Err(MotiveParametricCoherenceV2Error::NotProjectionLeaf);
    }
    let Some(parameter) = parse_rule_index(&clause_elaboration.derivation.rule, "ambient-param-")
    else {
        return Err(MotiveParametricCoherenceV2Error::NotProjectionLeaf);
    };
    let hypothesis = declaration
        .projection()
        .hypotheses
        .iter()
        .find(|hypothesis| hypothesis.parameter == parameter)
        .ok_or(MotiveParametricCoherenceV2Error::DeclarationMotiveMismatch { parameter })?;
    finish_verified(
        signature,
        candidate,
        visible_library,
        clause_index,
        hypothesis.motive.clone(),
        ClosureRuleKind::Projection,
        ClosureRuleEvidenceV2::Projection {
            source: ProjectionSourceEvidenceV2::AmbientParameter {
                parameter,
                declaration: declaration.projection().clone(),
            },
            leaf_rule: clause_elaboration.derivation.rule.clone(),
        },
    )
}

pub fn issue_prior_field_projection_closure_derivation_v2(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    prior_derivation: &VerifiedClosureDerivationV2,
    ambient_declaration: Option<&AmbientContextDeclarationToken>,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    let elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| underlying("prior-field projection elaboration", error))?;
    if elaboration.ambient_parameters == 0 {
        if ambient_declaration.is_some() {
            return Err(MotiveParametricCoherenceV2Error::DeclarationCandidateMismatch);
        }
    } else {
        let declaration = ambient_declaration
            .ok_or(MotiveParametricCoherenceV2Error::MissingAmbientDeclaration)?;
        validate_declaration_attachment(signature, candidate, visible_library, declaration)?;
    }
    let clause = candidate
        .clauses
        .get(usize::from(clause_index))
        .ok_or(MotiveParametricCoherenceV2Error::ClauseOutOfRange { clause_index })?;
    if clause.role == ClauseRole::Formation {
        return Err(MotiveParametricCoherenceV2Error::ActualBodyUnsupported(
            "prior-field projection Formation has no closed dereference target relation".to_owned(),
        ));
    }
    let clause_elaboration = elaboration
        .clauses
        .get(usize::from(clause_index))
        .ok_or(MotiveParametricCoherenceV2Error::ClauseOutOfRange { clause_index })?;
    if !matches!(clause.expr, Expr::Var(_)) || !clause_elaboration.derivation.children.is_empty() {
        return Err(MotiveParametricCoherenceV2Error::NotProjectionLeaf);
    }
    let Some(field) = parse_rule_index(&clause_elaboration.derivation.rule, "field-ref-") else {
        return Err(MotiveParametricCoherenceV2Error::NotProjectionLeaf);
    };
    let field = u16::try_from(field).map_err(|_| {
        MotiveParametricCoherenceV2Error::PriorEvidenceMismatch {
            clause_index: u16::MAX,
        }
    })?;
    let priors = BTreeMap::from([(field, Box::new(prior_derivation.clone()))]);
    validate_prior_derivations(signature, candidate, visible_library, clause_index, &priors)?;
    if let Some(full) = ambient_declaration {
        check_prior_declaration_compatibility(full.projection(), &priors)?;
    }
    finish_verified(
        signature,
        candidate,
        visible_library,
        clause_index,
        prior_derivation.inferred_motive.clone(),
        ClosureRuleKind::Projection,
        ClosureRuleEvidenceV2::Projection {
            source: ProjectionSourceEvidenceV2::CertifiedPriorField {
                clause_index: field,
                prior_derivation: Box::new(prior_derivation.clone()),
                ambient_declaration: ambient_declaration
                    .map(|declaration| declaration.projection().clone()),
            },
            leaf_rule: clause_elaboration.derivation.rule.clone(),
        },
    )
}

pub fn issue_guarded_closure_derivation_v2(
    signature: &SealedSignature,
    guarded_candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    declaration: &AmbientContextDeclarationToken,
    base_derivation: &VerifiedClosureDerivationV2,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    validate_declaration_attachment(signature, guarded_candidate, visible_library, declaration)?;
    if declaration.projection().ambient_arity != 1
        || base_derivation.ambient_arity != 0
        || base_derivation.visible_library != visible_library
        || base_derivation.signature_digest != signature.digest()
        || base_derivation.clause_index != clause_index
    {
        return Err(MotiveParametricCoherenceV2Error::GuardedBaseMismatch);
    }
    if base_derivation.candidate.kappa() != guarded_candidate.kappa()
        || base_derivation
            .candidate
            .clauses
            .iter()
            .zip(&guarded_candidate.clauses)
            .any(|(base, guarded)| {
                base.role != guarded.role || weaken_by_one_ambient(&base.expr) != guarded.expr
            })
    {
        return Err(MotiveParametricCoherenceV2Error::GuardedBaseMismatch);
    }
    let base_token = reissue_verified_closure_derivation_v2(signature, base_derivation)?;
    let weakening = issue_guarded_clause_weakening_erasure_token(
        signature,
        &base_derivation.candidate,
        guarded_candidate,
        visible_library,
        clause_index,
    )
    .map_err(|error| underlying("guarded weakening/erasure", error))?;
    replay_guarded_clause_weakening_erasure_projection(
        signature,
        &base_derivation.candidate,
        guarded_candidate,
        visible_library,
        weakening.projection(),
    )
    .map_err(|error| underlying("guarded weakening/erasure replay", error))?;
    if base_token.projection().expression != weakening.projection().base_expression {
        return Err(MotiveParametricCoherenceV2Error::GuardedBaseMismatch);
    }
    finish_verified(
        signature,
        guarded_candidate,
        visible_library,
        clause_index,
        weaken_motive_by_one_ambient(&base_derivation.inferred_motive),
        ClosureRuleKind::Guarded,
        ClosureRuleEvidenceV2::Guarded {
            base_derivation: Box::new(base_derivation.clone()),
            ambient_declaration: declaration.projection().clone(),
            weakening_erasure: weakening.projection().clone(),
        },
    )
}

pub fn issue_structural_closure_derivation_v2(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    let token =
        issue_structural_lambda_closure_token(signature, candidate, visible_library, clause_index)
            .map_err(|error| underlying("structural lambda closure", error))?;
    replay_structural_lambda_closure_projection(
        signature,
        candidate,
        visible_library,
        token.projection(),
    )
    .map_err(|error| underlying("structural lambda replay", error))?;
    let elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| underlying("structural elaboration", error))?;
    let motive = kernel_motive(&elaboration.clauses[usize::from(clause_index)].kernel_ty);
    finish_verified(
        signature,
        candidate,
        visible_library,
        clause_index,
        motive,
        ClosureRuleKind::Structural,
        ClosureRuleEvidenceV2::Structural {
            closure: token.projection().clone(),
        },
    )
}

pub fn issue_ambient_former_closure_derivation_v2(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    prior_derivations: &BTreeMap<u16, VerifiedClosureDerivationV2>,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    let prior_derivations = boxed_priors(prior_derivations);
    let prior_hashes = validate_prior_derivations(
        signature,
        candidate,
        visible_library,
        clause_index,
        &prior_derivations,
    )?;
    let token = issue_ambient_former_closure_token(
        signature,
        candidate,
        visible_library,
        clause_index,
        &prior_hashes,
    )
    .map_err(|error| underlying("ambient-former closure", error))?;
    replay_ambient_former_closure_projection(
        signature,
        candidate,
        visible_library,
        token.projection(),
    )
    .map_err(|error| underlying("ambient-former replay", error))?;
    if token.projection().certified_prior_clauses != prior_hashes {
        return Err(MotiveParametricCoherenceV2Error::ClosureReplayMismatch);
    }
    let used = token
        .projection()
        .term_evidence
        .certified_field_dependencies
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if used != prior_derivations.keys().copied().collect() {
        return Err(MotiveParametricCoherenceV2Error::PriorInventoryMismatch);
    }
    let elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| underlying("ambient-former elaboration", error))?;
    let motive = kernel_motive(&elaboration.clauses[usize::from(clause_index)].kernel_ty);
    finish_verified(
        signature,
        candidate,
        visible_library,
        clause_index,
        motive,
        ClosureRuleKind::AmbientFormer,
        ClosureRuleEvidenceV2::AmbientFormer {
            closure: token.projection().clone(),
            prior_derivations,
        },
    )
}

pub fn issue_dereference_closure_derivation_v2(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    prior_derivations: &BTreeMap<u16, VerifiedClosureDerivationV2>,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    let prior_derivations = boxed_priors(prior_derivations);
    let prior_hashes = validate_prior_derivations(
        signature,
        candidate,
        visible_library,
        clause_index,
        &prior_derivations,
    )?;
    let certified = prior_derivations
        .iter()
        .map(|(field, evidence)| {
            (
                *field,
                CertifiedPriorField {
                    clause_index: *field,
                    certificate_hash: evidence.derivation_hash.clone(),
                    referent_expression: evidence.expression.clone(),
                },
            )
        })
        .collect::<BTreeMap<_, _>>();
    if certified
        .iter()
        .any(|(field, record)| prior_hashes.get(field) != Some(&record.certificate_hash))
    {
        return Err(MotiveParametricCoherenceV2Error::ClosureReplayMismatch);
    }
    let token = issue_certified_field_dereference_token(
        signature,
        candidate,
        visible_library,
        clause_index,
        &certified,
    )
    .map_err(|error| underlying("certified-field dereference", error))?;
    replay_certified_field_dereference_projection(
        signature,
        candidate,
        visible_library,
        token.projection(),
    )
    .map_err(|error| underlying("certified-field dereference replay", error))?;
    if token.projection().certified_prior_fields != certified {
        return Err(MotiveParametricCoherenceV2Error::ClosureReplayMismatch);
    }
    let direct_elaboration = elaborate_telescope(
        signature,
        &token.projection().direct_candidate,
        visible_library,
    )
    .map_err(|error| underlying("dereference direct elaboration", error))?;
    let motive = kernel_motive(&direct_elaboration.clauses[usize::from(clause_index)].kernel_ty);
    finish_verified(
        signature,
        candidate,
        visible_library,
        clause_index,
        motive,
        ClosureRuleKind::Dereference,
        ClosureRuleEvidenceV2::Dereference {
            closure: token.projection().clone(),
            prior_derivations,
        },
    )
}

pub fn issue_contextual_closure_derivation_v2(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    declaration: &AmbientContextDeclarationToken,
    prior_derivations: &BTreeMap<u16, VerifiedClosureDerivationV2>,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    let clause = candidate
        .clauses
        .get(usize::from(clause_index))
        .ok_or(MotiveParametricCoherenceV2Error::ClauseOutOfRange { clause_index })?;
    if clause.role == ClauseRole::Formation {
        return Err(MotiveParametricCoherenceV2Error::ActualBodyUnsupported(
            "contextual Formation has no closed zero-credit relation in the six-rule target"
                .to_owned(),
        ));
    }
    if !prior_derivations.is_empty() {
        return Err(MotiveParametricCoherenceV2Error::ActualBodyUnsupported(
            "contextual closure with candidate-field support awaits a motive-refinement transport theorem"
                .to_owned(),
        ));
    }
    validate_declaration_attachment(signature, candidate, visible_library, declaration)?;
    let prior_derivations = boxed_priors(prior_derivations);
    let prior_hashes = validate_prior_derivations(
        signature,
        candidate,
        visible_library,
        clause_index,
        &prior_derivations,
    )?;
    let token = issue_contextual_typed_structure_token(
        signature,
        candidate,
        visible_library,
        clause_index,
        declaration,
        &prior_hashes,
    )
    .map_err(|error| underlying("contextual typed structure", error))?;
    replay_contextual_typed_structure_projection(signature, token.projection())
        .map_err(|error| underlying("contextual typed-structure replay", error))?;
    if token.projection().candidate != *candidate
        || token.projection().ambient_context != *declaration.projection()
        || token.projection().certified_prior_clauses != prior_hashes
    {
        return Err(MotiveParametricCoherenceV2Error::ClosureReplayMismatch);
    }
    let elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| underlying("contextual support elaboration", error))?;
    let used = exact_field_dependencies(&elaboration.clauses[usize::from(clause_index)].derivation);
    if used != prior_derivations.keys().copied().collect() {
        return Err(MotiveParametricCoherenceV2Error::PriorInventoryMismatch);
    }
    finish_verified(
        signature,
        candidate,
        visible_library,
        clause_index,
        token.projection().inferred_motive.clone(),
        ClosureRuleKind::Contextual,
        ClosureRuleEvidenceV2::Contextual {
            typed_structure: token.projection().clone(),
            prior_derivations,
        },
    )
}

pub fn issue_explicit_contextual_closure_derivation_v2(
    signature: &SealedSignature,
    declaration: &ExplicitAmbientContextDeclarationToken,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    let context = declaration.projection();
    if context.body_telescope.kappa() != 1 || context.declared_role == ClauseRole::Formation {
        return Err(MotiveParametricCoherenceV2Error::ActualBodyUnsupported(
            "explicit contextual source requires one non-Formation body clause".to_owned(),
        ));
    }
    let typed = issue_explicit_contextual_typed_structure_token(signature, declaration)
        .map_err(|error| underlying("explicit contextual typed structure", error))?;
    replay_explicit_contextual_typed_structure_projection(signature, typed.projection())
        .map_err(|error| underlying("explicit contextual typed-structure replay", error))?;
    if !typed.projection().typed_structure_issued
        || !typed.projection().live_contextual_dependency
        || typed.projection().ambient_context.ambient_arity == 0
    {
        return Err(MotiveParametricCoherenceV2Error::ActualBodyUnsupported(
            "explicit contextual source requires a replayed live ambient dependency".to_owned(),
        ));
    }
    let (elaboration, _) = elaborate_single_clause_with_derivation(
        &context.expression,
        context.ambient_arity,
        &[],
        context.visible_library,
    )
    .map_err(|error| underlying("explicit contextual elaboration", error))?;
    let evidence = ClosureRuleEvidenceV2::ExplicitContextual {
        typed_structure: typed.projection().clone(),
    };
    let mut projection = VerifiedClosureDerivationV2 {
        version: MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library: context.visible_library,
        candidate: context.body_telescope.clone(),
        candidate_hash: context.candidate_hash.clone(),
        elaboration_hash: context.explicit_elaboration_hash.clone(),
        ambient_arity: context.ambient_arity,
        clause_index: 0,
        declared_role: context.declared_role,
        expression: context.expression.clone(),
        normal_form: elaboration.normal_form,
        kernel_ty: KernelTyProjectionV2::from(&elaboration.kernel_ty),
        inferred_motive: typed.projection().inferred_motive.clone(),
        rule: ClosureRuleKind::Contextual,
        evidence,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("verified-closure-derivation", &projection);
    Ok(VerifiedClosureTokenV2 { projection })
}

pub fn issue_exact_explicit_future_body_closure_derivation_v2(
    signature: &SealedSignature,
    body_telescope: &Telescope,
    visible_library: u32,
    declaration: &ExplicitAmbientContextDeclarationToken,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    if declaration.projection().body_telescope != *body_telescope
        || declaration.projection().visible_library != visible_library
        || declaration.projection().candidate_hash != candidate_hash(body_telescope)
    {
        return Err(MotiveParametricCoherenceV2Error::DeclarationCandidateMismatch);
    }
    issue_explicit_contextual_closure_derivation_v2(signature, declaration)
}

fn unboxed_priors(
    priors: &BTreeMap<u16, Box<VerifiedClosureDerivationV2>>,
) -> BTreeMap<u16, VerifiedClosureDerivationV2> {
    priors
        .iter()
        .map(|(clause, evidence)| (*clause, evidence.as_ref().clone()))
        .collect()
}

fn reissue_verified_closure_derivation_v2(
    signature: &SealedSignature,
    projection: &VerifiedClosureDerivationV2,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    if projection.evidence.kind() != projection.rule {
        return Err(MotiveParametricCoherenceV2Error::RuleEvidenceMismatch);
    }
    let reissued = match &projection.evidence {
        ClosureRuleEvidenceV2::Projection { source, .. } => match source {
            ProjectionSourceEvidenceV2::AmbientParameter { declaration, .. } => {
                let declaration = declaration_token_from_projection(
                    signature,
                    &projection.candidate,
                    declaration,
                )?;
                issue_ambient_projection_closure_derivation_v2(
                    signature,
                    &projection.candidate,
                    projection.visible_library,
                    projection.clause_index,
                    &declaration,
                )?
            }
            ProjectionSourceEvidenceV2::CertifiedPriorField {
                prior_derivation,
                ambient_declaration,
                ..
            } => {
                let declaration = ambient_declaration
                    .as_ref()
                    .map(|stored| {
                        declaration_token_from_projection(signature, &projection.candidate, stored)
                    })
                    .transpose()?;
                issue_prior_field_projection_closure_derivation_v2(
                    signature,
                    &projection.candidate,
                    projection.visible_library,
                    projection.clause_index,
                    prior_derivation,
                    declaration.as_ref(),
                )?
            }
        },
        ClosureRuleEvidenceV2::Guarded {
            base_derivation,
            ambient_declaration,
            weakening_erasure,
        } => {
            replay_guarded_clause_weakening_erasure_projection(
                signature,
                &base_derivation.candidate,
                &projection.candidate,
                projection.visible_library,
                weakening_erasure,
            )
            .map_err(|error| underlying("guarded weakening/erasure replay", error))?;
            let declaration = declaration_token_from_projection(
                signature,
                &projection.candidate,
                ambient_declaration,
            )?;
            issue_guarded_closure_derivation_v2(
                signature,
                &projection.candidate,
                projection.visible_library,
                projection.clause_index,
                &declaration,
                base_derivation,
            )?
        }
        ClosureRuleEvidenceV2::Structural { closure } => {
            replay_structural_lambda_closure_projection(
                signature,
                &projection.candidate,
                projection.visible_library,
                closure,
            )
            .map_err(|error| underlying("structural lambda replay", error))?;
            issue_structural_closure_derivation_v2(
                signature,
                &projection.candidate,
                projection.visible_library,
                projection.clause_index,
            )?
        }
        ClosureRuleEvidenceV2::AmbientFormer {
            closure,
            prior_derivations,
        } => {
            replay_ambient_former_closure_projection(
                signature,
                &projection.candidate,
                projection.visible_library,
                closure,
            )
            .map_err(|error| underlying("ambient-former replay", error))?;
            issue_ambient_former_closure_derivation_v2(
                signature,
                &projection.candidate,
                projection.visible_library,
                projection.clause_index,
                &unboxed_priors(prior_derivations),
            )?
        }
        ClosureRuleEvidenceV2::Dereference {
            closure,
            prior_derivations,
        } => {
            replay_certified_field_dereference_projection(
                signature,
                &projection.candidate,
                projection.visible_library,
                closure,
            )
            .map_err(|error| underlying("certified-field dereference replay", error))?;
            issue_dereference_closure_derivation_v2(
                signature,
                &projection.candidate,
                projection.visible_library,
                projection.clause_index,
                &unboxed_priors(prior_derivations),
            )?
        }
        ClosureRuleEvidenceV2::Contextual {
            typed_structure,
            prior_derivations,
        } => {
            replay_contextual_typed_structure_projection(signature, typed_structure)
                .map_err(|error| underlying("contextual typed-structure replay", error))?;
            let declaration = declaration_token_from_projection(
                signature,
                &projection.candidate,
                &typed_structure.ambient_context,
            )?;
            issue_contextual_closure_derivation_v2(
                signature,
                &projection.candidate,
                projection.visible_library,
                projection.clause_index,
                &declaration,
                &unboxed_priors(prior_derivations),
            )?
        }
        ClosureRuleEvidenceV2::ExplicitContextual { typed_structure } => {
            replay_explicit_contextual_typed_structure_projection(signature, typed_structure)
                .map_err(|error| underlying("explicit contextual replay", error))?;
            let declaration = explicit_declaration_token_from_projection(
                signature,
                &typed_structure.ambient_context,
            )?;
            issue_explicit_contextual_closure_derivation_v2(signature, &declaration)?
        }
    };
    if reissued.projection == *projection {
        Ok(reissued)
    } else {
        Err(MotiveParametricCoherenceV2Error::ClosureReplayMismatch)
    }
}

pub fn replay_verified_closure_derivation_v2(
    signature: &SealedSignature,
    projection: &VerifiedClosureDerivationV2,
) -> Result<(), MotiveParametricCoherenceV2Error> {
    reissue_verified_closure_derivation_v2(signature, projection).map(|_| ())
}

pub fn reissue_verified_closure_token_v2(
    signature: &SealedSignature,
    projection: &VerifiedClosureDerivationV2,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    reissue_verified_closure_derivation_v2(signature, projection)
}

/// Candidate-specific actual-body issuer.  Open bodies require the declaration
/// token to be attached to this exact telescope; no declaration-carrier or
/// source-hash adapter exists in this API.
pub fn issue_actual_body_closure_derivation_v2(
    signature: &SealedSignature,
    body_telescope: &Telescope,
    visible_library: u32,
    clause_index: u16,
    declaration: Option<&AmbientContextDeclarationToken>,
    prior_derivations: &BTreeMap<u16, VerifiedClosureDerivationV2>,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    let elaboration = elaborate_telescope(signature, body_telescope, visible_library)
        .map_err(|error| underlying("actual-body elaboration", error))?;
    let clause = body_telescope
        .clauses
        .get(usize::from(clause_index))
        .ok_or(MotiveParametricCoherenceV2Error::ClauseOutOfRange { clause_index })?;
    let clause_elaboration = elaboration
        .clauses
        .get(usize::from(clause_index))
        .ok_or(MotiveParametricCoherenceV2Error::ClauseOutOfRange { clause_index })?;

    if matches!(clause.expr, Expr::Var(_)) && clause_elaboration.derivation.children.is_empty() {
        if parse_rule_index(&clause_elaboration.derivation.rule, "ambient-param-").is_some() {
            let declaration =
                declaration.ok_or(MotiveParametricCoherenceV2Error::MissingAmbientDeclaration)?;
            return issue_ambient_projection_closure_derivation_v2(
                signature,
                body_telescope,
                visible_library,
                clause_index,
                declaration,
            );
        }
        if let Some(field) = parse_rule_index(&clause_elaboration.derivation.rule, "field-ref-") {
            let field = u16::try_from(field).map_err(|_| {
                MotiveParametricCoherenceV2Error::PriorEvidenceMismatch {
                    clause_index: u16::MAX,
                }
            })?;
            let prior = prior_derivations.get(&field).ok_or(
                MotiveParametricCoherenceV2Error::PriorEvidenceMismatch {
                    clause_index: field,
                },
            )?;
            return issue_prior_field_projection_closure_derivation_v2(
                signature,
                body_telescope,
                visible_library,
                clause_index,
                prior,
                declaration,
            );
        }
    }

    if elaboration.ambient_parameters > 0 {
        let declaration =
            declaration.ok_or(MotiveParametricCoherenceV2Error::MissingAmbientDeclaration)?;
        return issue_contextual_closure_derivation_v2(
            signature,
            body_telescope,
            visible_library,
            clause_index,
            declaration,
            prior_derivations,
        );
    }
    if declaration.is_some() {
        return Err(MotiveParametricCoherenceV2Error::DeclarationCandidateMismatch);
    }
    if !prior_derivations.is_empty() {
        return issue_dereference_closure_derivation_v2(
            signature,
            body_telescope,
            visible_library,
            clause_index,
            prior_derivations,
        );
    }
    if matches!(clause.expr, Expr::Lam(_)) {
        if let Ok(token) = issue_structural_closure_derivation_v2(
            signature,
            body_telescope,
            visible_library,
            clause_index,
        ) {
            return Ok(token);
        }
    }
    issue_ambient_former_closure_derivation_v2(
        signature,
        body_telescope,
        visible_library,
        clause_index,
        prior_derivations,
    )
}

/// The future-hole integration entrypoint: the declaration is reissued on the
/// exact one-body telescope and the prior inventory is definitionally empty.
pub fn issue_exact_future_body_closure_derivation_v2(
    signature: &SealedSignature,
    body_telescope: &Telescope,
    visible_library: u32,
    declaration: Option<&AmbientContextDeclarationToken>,
) -> Result<VerifiedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    if body_telescope.kappa() != 1 {
        return Err(MotiveParametricCoherenceV2Error::ActualBodyUnsupported(
            "future body telescope must contain exactly one clause".to_owned(),
        ));
    }
    issue_actual_body_closure_derivation_v2(
        signature,
        body_telescope,
        visible_library,
        0,
        declaration,
        &BTreeMap::new(),
    )
}

pub fn issue_closed_internal_evidence_v2(
    signature: &SealedSignature,
    relation: &VerifiedClosureTokenV2,
) -> Result<ClosedInternalEvidenceTokenV2, MotiveParametricCoherenceV2Error> {
    replay_verified_closure_derivation_v2(signature, relation.projection())?;
    if relation.projection().ambient_arity != 0 {
        return Err(MotiveParametricCoherenceV2Error::EvidenceNotClosed {
            ambient: relation.projection().ambient_arity,
        });
    }
    let mut projection = ClosedInternalEvidenceV2 {
        version: MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library: relation.projection().visible_library,
        expression: relation.projection().expression.clone(),
        inferred_motive: relation.projection().inferred_motive.clone(),
        relation: Box::new(relation.projection().clone()),
        relation_replayed: true,
        closed_candidate: true,
        evidence_hash: String::new(),
    };
    projection.evidence_hash = tagged_hash("closed-internal-evidence", &projection);
    Ok(ClosedInternalEvidenceTokenV2 { projection })
}

fn reissue_closed_internal_evidence_v2(
    signature: &SealedSignature,
    projection: &ClosedInternalEvidenceV2,
) -> Result<ClosedInternalEvidenceTokenV2, MotiveParametricCoherenceV2Error> {
    let relation = reissue_verified_closure_derivation_v2(signature, &projection.relation)?;
    let reissued = issue_closed_internal_evidence_v2(signature, &relation)?;
    if reissued.projection == *projection {
        Ok(reissued)
    } else {
        Err(MotiveParametricCoherenceV2Error::ClosedEvidenceReplayMismatch)
    }
}

pub fn replay_closed_internal_evidence_v2(
    signature: &SealedSignature,
    projection: &ClosedInternalEvidenceV2,
) -> Result<(), MotiveParametricCoherenceV2Error> {
    reissue_closed_internal_evidence_v2(signature, projection).map(|_| ())
}

pub fn issue_motive_typed_closed_assignment_v2(
    signature: &SealedSignature,
    visible_library: u32,
    motives: Vec<ContextualMotive>,
    evidence: Vec<ClosedInternalEvidenceV2>,
) -> Result<MotiveTypedClosedAssignmentTokenV2, MotiveParametricCoherenceV2Error> {
    if motives.len() != evidence.len() {
        return Err(MotiveParametricCoherenceV2Error::AssignmentArity {
            motives: motives.len() as u32,
            evidence: evidence.len() as u32,
        });
    }
    let mut images = Vec::with_capacity(motives.len());
    for (index, (motive, evidence)) in motives.into_iter().zip(evidence).enumerate() {
        let parameter = index as u32 + 1;
        reissue_closed_internal_evidence_v2(signature, &evidence)?;
        if evidence.visible_library != visible_library
            || evidence.signature_digest != signature.digest()
            || evidence.inferred_motive != motive
            || evidence.expression != evidence.relation.expression
            || evidence.relation.ambient_arity != 0
            || evidence.relation.candidate.kappa() != 1
            || evidence.relation.clause_index != 0
            || !is_well_scoped(&evidence.expression, 0)
        {
            return Err(MotiveParametricCoherenceV2Error::AssignmentMotiveMismatch { parameter });
        }
        images.push(MotiveTypedClosedImageProjectionV2 {
            parameter,
            motive,
            term: evidence.expression.clone(),
            evidence,
            evidence_replayed: true,
        });
    }
    let mut projection = MotiveTypedClosedAssignmentProjectionV2 {
        version: MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        source_arity: images.len() as u32,
        images,
        assignment_hash: String::new(),
    };
    projection.assignment_hash = tagged_hash("motive-typed-closed-assignment", &projection);
    Ok(MotiveTypedClosedAssignmentTokenV2 { projection })
}

fn reissue_motive_typed_closed_assignment_v2(
    signature: &SealedSignature,
    projection: &MotiveTypedClosedAssignmentProjectionV2,
) -> Result<MotiveTypedClosedAssignmentTokenV2, MotiveParametricCoherenceV2Error> {
    let reissued = issue_motive_typed_closed_assignment_v2(
        signature,
        projection.visible_library,
        projection
            .images
            .iter()
            .map(|image| image.motive.clone())
            .collect(),
        projection
            .images
            .iter()
            .map(|image| image.evidence.clone())
            .collect(),
    )?;
    if reissued.projection == *projection {
        Ok(reissued)
    } else {
        Err(MotiveParametricCoherenceV2Error::AssignmentReplayMismatch)
    }
}

pub fn replay_motive_typed_closed_assignment_v2(
    signature: &SealedSignature,
    projection: &MotiveTypedClosedAssignmentProjectionV2,
) -> Result<(), MotiveParametricCoherenceV2Error> {
    reissue_motive_typed_closed_assignment_v2(signature, projection).map(|_| ())
}

fn source_hypotheses(
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

fn check_prior_declaration_compatibility(
    full: &AmbientContextDeclarationProjection,
    priors: &BTreeMap<u16, Box<VerifiedClosureDerivationV2>>,
) -> Result<(), MotiveParametricCoherenceV2Error> {
    for prior in priors.values() {
        if prior.ambient_arity == 0 {
            continue;
        }
        let Some(prior_hypotheses) = source_hypotheses(prior) else {
            return Err(MotiveParametricCoherenceV2Error::PriorDeclarationMismatch);
        };
        let count = prior.ambient_arity as usize;
        if prior_hypotheses.len() != count
            || full.hypotheses.len() < count
            || prior_hypotheses != &full.hypotheses[..count]
        {
            return Err(MotiveParametricCoherenceV2Error::PriorDeclarationMismatch);
        }
    }
    Ok(())
}

fn check_assignment_for_source(
    source: &VerifiedClosureDerivationV2,
    assignment: &MotiveTypedClosedAssignmentProjectionV2,
) -> Result<(), MotiveParametricCoherenceV2Error> {
    if source.signature_digest != assignment.signature_digest
        || source.visible_library != assignment.visible_library
        || source.ambient_arity != assignment.source_arity
    {
        return Err(MotiveParametricCoherenceV2Error::AssignmentDeclarationMismatch);
    }
    if source.ambient_arity == 0 {
        if assignment.images.is_empty() {
            return Ok(());
        }
        return Err(MotiveParametricCoherenceV2Error::AssignmentDeclarationMismatch);
    }
    let hypotheses = source_hypotheses(source)
        .ok_or(MotiveParametricCoherenceV2Error::AssignmentDeclarationMismatch)?;
    if hypotheses.len() != source.ambient_arity as usize
        || !hypotheses
            .iter()
            .zip(&assignment.images)
            .all(|(hypothesis, image)| {
                hypothesis.parameter == image.parameter && hypothesis.motive == image.motive
            })
    {
        return Err(MotiveParametricCoherenceV2Error::AssignmentDeclarationMismatch);
    }
    Ok(())
}

fn structural_images_for_clause(
    source_ambient: u32,
    clause_index: u16,
    assignment: &MotiveTypedClosedAssignmentProjectionV2,
) -> Vec<SubstitutionImage> {
    let mut images = assignment
        .images
        .iter()
        .map(|image| SubstitutionImage {
            source_parameter: image.parameter,
            term: image.term.clone(),
        })
        .collect::<Vec<_>>();
    for field in 1..=u32::from(clause_index) {
        images.push(SubstitutionImage {
            source_parameter: source_ambient + field,
            term: Expr::Var(field),
        });
    }
    images
}

fn issue_clause_substitution_projection(
    source_ambient: u32,
    clause_index: u16,
    assignment: &MotiveTypedClosedAssignmentProjectionV2,
    body: &Expr,
) -> Result<StructuralSubstitutionProjectionV2, MotiveParametricCoherenceV2Error> {
    let source_arity = source_ambient + u32::from(clause_index);
    let target_arity = u32::from(clause_index);
    let images = structural_images_for_clause(source_ambient, clause_index, assignment);
    let token = issue_structural_substitution(
        SortedParameterContext::all_type(source_arity),
        SortedParameterContext::all_type(target_arity),
        images.clone(),
        body.clone(),
    )
    .map_err(|error| MotiveParametricCoherenceV2Error::Substitution(error.to_string()))?;
    replay_structural_substitution(&token)
        .map_err(|error| MotiveParametricCoherenceV2Error::Substitution(error.to_string()))?;
    Ok(StructuralSubstitutionProjectionV2 {
        source_arity,
        target_arity,
        images: images
            .iter()
            .map(SubstitutionImageProjectionV2::from)
            .collect(),
        body: body.clone(),
        result: token.result().clone(),
        derivation_hash: token.derivation_hash().to_owned(),
    })
}

fn replay_clause_substitution_projection(
    projection: &StructuralSubstitutionProjectionV2,
) -> Result<(), MotiveParametricCoherenceV2Error> {
    let images = projection
        .images
        .iter()
        .map(|image| SubstitutionImage {
            source_parameter: image.source_parameter,
            term: image.term.clone(),
        })
        .collect::<Vec<_>>();
    let token = issue_structural_substitution(
        SortedParameterContext::all_type(projection.source_arity),
        SortedParameterContext::all_type(projection.target_arity),
        images,
        projection.body.clone(),
    )
    .map_err(|error| MotiveParametricCoherenceV2Error::Substitution(error.to_string()))?;
    replay_structural_substitution(&token)
        .map_err(|error| MotiveParametricCoherenceV2Error::Substitution(error.to_string()))?;
    if token.result() == &projection.result && token.derivation_hash() == projection.derivation_hash
    {
        Ok(())
    } else {
        Err(MotiveParametricCoherenceV2Error::SubstitutionReplayMismatch)
    }
}

fn specialize_candidate(
    source: &VerifiedClosureDerivationV2,
    assignment: &MotiveTypedClosedAssignmentProjectionV2,
) -> Result<Telescope, MotiveParametricCoherenceV2Error> {
    let clauses = source
        .candidate
        .clauses
        .iter()
        .enumerate()
        .map(|(index, clause)| {
            let clause_index = u16::try_from(index).map_err(|_| {
                MotiveParametricCoherenceV2Error::Substitution(
                    "candidate clause index exceeds u16".to_owned(),
                )
            })?;
            let substitution = issue_clause_substitution_projection(
                source.ambient_arity,
                clause_index,
                assignment,
                &clause.expr,
            )?;
            let mut specialized = clause.clone();
            specialized.expr = substitution.result;
            Ok(specialized)
        })
        .collect::<Result<Vec<_>, MotiveParametricCoherenceV2Error>>()?;
    Ok(Telescope::new(clauses))
}

fn specialize_motive(
    motive: &ContextualMotive,
    source_ambient: u32,
    clause_index: u16,
    assignment: &MotiveTypedClosedAssignmentProjectionV2,
) -> Result<ContextualMotive, MotiveParametricCoherenceV2Error> {
    match motive {
        ContextualMotive::Type => Ok(ContextualMotive::Type),
        ContextualMotive::Element(expression) => Ok(ContextualMotive::Element(
            issue_clause_substitution_projection(
                source_ambient,
                clause_index,
                assignment,
                expression,
            )?
            .result,
        )),
        ContextualMotive::Function { domain, codomain } => Ok(ContextualMotive::Function {
            domain: Box::new(specialize_motive(
                domain,
                source_ambient,
                clause_index,
                assignment,
            )?),
            codomain: Box::new(specialize_motive(
                codomain,
                source_ambient,
                clause_index,
                assignment,
            )?),
        }),
        ContextualMotive::Neutral => Ok(ContextualMotive::Neutral),
    }
}

fn assignment_for_derivation(
    signature: &SealedSignature,
    derivation: &VerifiedClosureDerivationV2,
    full: &MotiveTypedClosedAssignmentProjectionV2,
) -> Result<MotiveTypedClosedAssignmentTokenV2, MotiveParametricCoherenceV2Error> {
    if derivation.ambient_arity > full.source_arity {
        return Err(MotiveParametricCoherenceV2Error::AssignmentDeclarationMismatch);
    }
    let take = derivation.ambient_arity as usize;
    let token = issue_motive_typed_closed_assignment_v2(
        signature,
        full.visible_library,
        full.images
            .iter()
            .take(take)
            .map(|image| image.motive.clone())
            .collect(),
        full.images
            .iter()
            .take(take)
            .map(|image| image.evidence.clone())
            .collect(),
    )?;
    check_assignment_for_source(derivation, token.projection())?;
    Ok(token)
}

fn specialize_prior_map(
    signature: &SealedSignature,
    priors: &BTreeMap<u16, Box<VerifiedClosureDerivationV2>>,
    assignment: &MotiveTypedClosedAssignmentProjectionV2,
) -> Result<BTreeMap<u16, VerifiedClosureDerivationV2>, MotiveParametricCoherenceV2Error> {
    let mut specialized = BTreeMap::new();
    for (field, prior) in priors {
        let prior_token = reissue_verified_closure_derivation_v2(signature, prior)?;
        let prior_assignment = assignment_for_derivation(signature, prior, assignment)?;
        let result =
            specialize_verified_closure_derivation_v2(signature, &prior_token, &prior_assignment)?;
        reissue_closed_internal_evidence_v2(signature, &result.projection().specialized_relation)?;
        specialized.insert(
            *field,
            result
                .projection()
                .specialized_relation
                .relation
                .as_ref()
                .clone(),
        );
    }
    Ok(specialized)
}

fn issue_specialized_closed_relation(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    priors: &BTreeMap<u16, VerifiedClosureDerivationV2>,
) -> Result<ClosedInternalEvidenceTokenV2, MotiveParametricCoherenceV2Error> {
    let relation = if priors.is_empty() {
        issue_ambient_former_closure_derivation_v2(
            signature,
            candidate,
            visible_library,
            clause_index,
            priors,
        )?
    } else {
        issue_dereference_closure_derivation_v2(
            signature,
            candidate,
            visible_library,
            clause_index,
            priors,
        )?
    };
    issue_closed_internal_evidence_v2(signature, &relation)
}

/// Exhaustive six-rule eliminator.  There is no motive enumeration and no
/// registered-probe branch: arbitrary motives enter only through a replayed
/// declaration and arbitrary closed instances enter only through replayed
/// `ClosedInternalEvidenceV2` values.
pub fn specialize_verified_closure_derivation_v2(
    signature: &SealedSignature,
    source: &VerifiedClosureTokenV2,
    assignment: &MotiveTypedClosedAssignmentTokenV2,
) -> Result<SpecializedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    replay_verified_closure_derivation_v2(signature, source.projection())?;
    replay_motive_typed_closed_assignment_v2(signature, assignment.projection())?;
    check_assignment_for_source(source.projection(), assignment.projection())?;

    let substitution = issue_clause_substitution_projection(
        source.projection().ambient_arity,
        source.projection().clause_index,
        assignment.projection(),
        &source.projection().expression,
    )?;
    replay_clause_substitution_projection(&substitution)?;
    let specialized_candidate = specialize_candidate(source.projection(), assignment.projection())?;
    let specialized_motive = specialize_motive(
        &source.projection().inferred_motive,
        source.projection().ambient_arity,
        source.projection().clause_index,
        assignment.projection(),
    )?;

    let specialized_relation = match &source.projection().evidence {
        ClosureRuleEvidenceV2::Projection {
            source: ProjectionSourceEvidenceV2::AmbientParameter { parameter, .. },
            ..
        } => {
            let image = assignment
                .projection()
                .images
                .iter()
                .find(|image| image.parameter == *parameter)
                .ok_or(MotiveParametricCoherenceV2Error::AssignmentDeclarationMismatch)?;
            if substitution.result != image.term {
                return Err(MotiveParametricCoherenceV2Error::SubstitutionStability {
                    rule: ClosureRuleKind::Projection,
                    reason: "ambient projection did not specialize to its exact image".to_owned(),
                });
            }
            reissue_closed_internal_evidence_v2(signature, &image.evidence)?;
            issue_specialized_closed_relation(
                signature,
                &specialized_candidate,
                source.projection().visible_library,
                source.projection().clause_index,
                &BTreeMap::new(),
            )?
        }
        ClosureRuleEvidenceV2::Projection {
            source:
                ProjectionSourceEvidenceV2::CertifiedPriorField {
                    clause_index,
                    prior_derivation,
                    ..
                },
            ..
        } => {
            let priors = BTreeMap::from([(*clause_index, prior_derivation.clone())]);
            let specialized_priors =
                specialize_prior_map(signature, &priors, assignment.projection())?;
            issue_specialized_closed_relation(
                signature,
                &specialized_candidate,
                source.projection().visible_library,
                source.projection().clause_index,
                &specialized_priors,
            )?
        }
        ClosureRuleEvidenceV2::Guarded {
            base_derivation, ..
        } => {
            let base = reissue_verified_closure_derivation_v2(signature, base_derivation)?;
            if substitution.result != base.projection().expression
                || specialized_candidate != base.projection().candidate
            {
                return Err(MotiveParametricCoherenceV2Error::SubstitutionStability {
                    rule: ClosureRuleKind::Guarded,
                    reason:
                        "weakening/erasure specialization did not recover the exact base judgment"
                            .to_owned(),
                });
            }
            issue_closed_internal_evidence_v2(signature, &base)?
        }
        ClosureRuleEvidenceV2::Structural { .. }
        | ClosureRuleEvidenceV2::AmbientFormer { .. }
        | ClosureRuleEvidenceV2::Dereference { .. } => {
            if assignment.projection().source_arity != 0
                || specialized_candidate != source.projection().candidate
                || substitution.result != source.projection().expression
            {
                return Err(MotiveParametricCoherenceV2Error::SubstitutionStability {
                    rule: source.projection().rule,
                    reason: "closed closure rule received a nonempty or nonidentity assignment"
                        .to_owned(),
                });
            }
            issue_closed_internal_evidence_v2(signature, source)?
        }
        ClosureRuleEvidenceV2::Contextual {
            prior_derivations, ..
        } => {
            let specialized_priors =
                specialize_prior_map(signature, prior_derivations, assignment.projection())?;
            issue_specialized_closed_relation(
                signature,
                &specialized_candidate,
                source.projection().visible_library,
                source.projection().clause_index,
                &specialized_priors,
            )?
        }
        ClosureRuleEvidenceV2::ExplicitContextual { .. } => issue_specialized_closed_relation(
            signature,
            &specialized_candidate,
            source.projection().visible_library,
            source.projection().clause_index,
            &BTreeMap::new(),
        )?,
    };

    let specialized_elaboration = elaborate_telescope(
        signature,
        &specialized_candidate,
        source.projection().visible_library,
    )
    .map_err(
        |error| MotiveParametricCoherenceV2Error::SubstitutionStability {
            rule: source.projection().rule,
            reason: format!("specialized candidate failed elaboration: {error}"),
        },
    )?;
    if specialized_elaboration.ambient_parameters != 0 {
        return Err(MotiveParametricCoherenceV2Error::SubstitutionStability {
            rule: source.projection().rule,
            reason: "closed assignment did not close the specialized candidate".to_owned(),
        });
    }
    let specialized_clause =
        &specialized_elaboration.clauses[usize::from(source.projection().clause_index)];
    let specialized_kernel_ty = KernelTyProjectionV2::from(&specialized_clause.kernel_ty);
    let specialized_normal_form = specialized_clause.normal_form.clone();
    let relation = specialized_relation.projection();
    if relation.expression != substitution.result
        || relation.inferred_motive != specialized_motive
        || relation.relation.expression != substitution.result
        || relation.relation.kernel_ty != specialized_kernel_ty
        || relation.relation.normal_form != specialized_normal_form
        || relation.relation.candidate != specialized_candidate
        || relation.relation.clause_index != source.projection().clause_index
        || relation.relation.visible_library != source.projection().visible_library
        || relation.relation.signature_digest != source.projection().signature_digest
        || relation.relation.ambient_arity != 0
    {
        return Err(MotiveParametricCoherenceV2Error::SubstitutionStability {
            rule: source.projection().rule,
            reason: "specialized expression, motive, kernel classifier, or normal form disagrees with the reissued closed relation"
                .to_owned(),
        });
    }

    let mut projection = SpecializedClosureDerivationV2 {
        version: MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION.to_owned(),
        source_derivation: Box::new(source.projection().clone()),
        assignment: assignment.projection().clone(),
        substitution,
        specialized_candidate,
        specialized_expression: relation.expression.clone(),
        specialized_normal_form,
        specialized_kernel_ty,
        specialized_motive,
        specialized_relation: relation.clone(),
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("specialized-closure-derivation", &projection);
    Ok(SpecializedClosureTokenV2 { projection })
}

pub fn replay_specialized_closure_derivation_v2(
    signature: &SealedSignature,
    projection: &SpecializedClosureDerivationV2,
) -> Result<(), MotiveParametricCoherenceV2Error> {
    let source = reissue_verified_closure_derivation_v2(signature, &projection.source_derivation)?;
    let assignment = reissue_motive_typed_closed_assignment_v2(signature, &projection.assignment)?;
    let reissued = specialize_verified_closure_derivation_v2(signature, &source, &assignment)?;
    if reissued.projection == *projection {
        Ok(())
    } else {
        Err(MotiveParametricCoherenceV2Error::SpecializedReplayMismatch)
    }
}

/// Persisted-artifact eliminator.  Both public projections are recursively
/// reissued to opaque tokens before the theorem is applied.
pub fn specialize_verified_closure_projection_v2(
    signature: &SealedSignature,
    source: &VerifiedClosureDerivationV2,
    assignment: &MotiveTypedClosedAssignmentProjectionV2,
) -> Result<SpecializedClosureTokenV2, MotiveParametricCoherenceV2Error> {
    let source = reissue_verified_closure_derivation_v2(signature, source)?;
    let assignment = reissue_motive_typed_closed_assignment_v2(signature, assignment)?;
    specialize_verified_closure_derivation_v2(signature, &source, &assignment)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contextual_internality::issue_ambient_context_declaration_token;
    use pen_core::clause::ClauseRec;

    fn singleton(expression: Expr) -> Telescope {
        Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, expression)])
    }

    fn closed_evidence(signature: &SealedSignature, expression: Expr) -> ClosedInternalEvidenceV2 {
        let candidate = singleton(expression);
        let relation =
            issue_exact_future_body_closure_derivation_v2(signature, &candidate, 15, None)
                .expect("closed exact relation");
        issue_closed_internal_evidence_v2(signature, &relation)
            .expect("closed Internal evidence")
            .projection()
            .clone()
    }

    fn type_assignment(signature: &SealedSignature) -> MotiveTypedClosedAssignmentTokenV2 {
        issue_motive_typed_closed_assignment_v2(
            signature,
            15,
            vec![ContextualMotive::Type],
            vec![closed_evidence(signature, Expr::Lib(14))],
        )
        .expect("Type assignment")
    }

    fn declaration(
        signature: &SealedSignature,
        candidate: &Telescope,
        motives: Vec<ContextualMotive>,
    ) -> AmbientContextDeclarationToken {
        issue_ambient_context_declaration_token(signature, candidate, 15, motives)
            .expect("exact ambient declaration")
    }

    #[test]
    fn app_and_id_multi_premise_contextual_cases_reissue_closed_relations() {
        let signature = SealedSignature::genesis_del_h15();
        let assignment = type_assignment(&signature);
        let cases = [
            (
                Expr::App(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
                2,
            ),
            (
                Expr::Id(
                    Box::new(Expr::Lib(15)),
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Lib(14)),
                ),
                3,
            ),
        ];
        for (expression, premise_count) in cases {
            let candidate = singleton(expression);
            let declaration = declaration(&signature, &candidate, vec![ContextualMotive::Type]);
            let source = issue_exact_future_body_closure_derivation_v2(
                &signature,
                &candidate,
                15,
                Some(&declaration),
            )
            .expect("probe-free contextual source");
            assert_eq!(source.projection().rule, ClosureRuleKind::Contextual);
            let specialized = specialize_verified_closure_projection_v2(
                &signature,
                source.projection(),
                assignment.projection(),
            )
            .expect("exact contextual specialization");
            replay_specialized_closure_derivation_v2(&signature, specialized.projection())
                .expect("specialization replay");
            assert_eq!(
                specialized.projection().specialized_candidate,
                specialized
                    .projection()
                    .specialized_relation
                    .relation
                    .candidate
            );
            let ClosureRuleEvidenceV2::AmbientFormer { closure, .. } = &specialized
                .projection()
                .specialized_relation
                .relation
                .evidence
            else {
                panic!("contextual case must reissue the closed ambient-former relation");
            };
            assert_eq!(closure.term_evidence.premises.len(), premise_count);
        }
    }

    #[test]
    fn exact_eliminator_replays_all_six_rule_variants() {
        let signature = SealedSignature::genesis_del_h15();
        let empty = issue_motive_typed_closed_assignment_v2(&signature, 15, Vec::new(), Vec::new())
            .expect("empty assignment");
        let typed = type_assignment(&signature);
        let mut covered = BTreeSet::new();

        let projection_candidate = singleton(Expr::Var(1));
        let projection_declaration = declaration(
            &signature,
            &projection_candidate,
            vec![ContextualMotive::Type],
        );
        let projection = issue_exact_future_body_closure_derivation_v2(
            &signature,
            &projection_candidate,
            15,
            Some(&projection_declaration),
        )
        .expect("projection");
        specialize_verified_closure_derivation_v2(&signature, &projection, &typed)
            .expect("projection specialization");
        covered.insert(projection.projection().rule);

        let base_candidate = singleton(Expr::Lam(Box::new(Expr::Var(1))));
        let base =
            issue_exact_future_body_closure_derivation_v2(&signature, &base_candidate, 15, None)
                .expect("guarded base");
        let guarded_candidate = singleton(weaken_by_one_ambient(&base_candidate.clauses[0].expr));
        let guarded_declaration =
            declaration(&signature, &guarded_candidate, vec![ContextualMotive::Type]);
        let guarded = issue_guarded_closure_derivation_v2(
            &signature,
            &guarded_candidate,
            15,
            0,
            &guarded_declaration,
            base.projection(),
        )
        .expect("guarded");
        specialize_verified_closure_derivation_v2(&signature, &guarded, &typed)
            .expect("guarded specialization");
        covered.insert(guarded.projection().rule);

        let structural_candidate = singleton(Expr::Lam(Box::new(Expr::Univ)));
        let structural =
            issue_structural_closure_derivation_v2(&signature, &structural_candidate, 15, 0)
                .expect("structural");
        specialize_verified_closure_derivation_v2(&signature, &structural, &empty)
            .expect("structural empty specialization");
        covered.insert(structural.projection().rule);

        let ambient_candidate = singleton(Expr::Flat(Box::new(Expr::Lib(15))));
        let ambient = issue_ambient_former_closure_derivation_v2(
            &signature,
            &ambient_candidate,
            15,
            0,
            &BTreeMap::new(),
        )
        .expect("ambient former");
        specialize_verified_closure_derivation_v2(&signature, &ambient, &empty)
            .expect("ambient empty specialization");
        covered.insert(ambient.projection().rule);

        let prior_candidate = singleton(Expr::Lib(15));
        let prior =
            issue_exact_future_body_closure_derivation_v2(&signature, &prior_candidate, 15, None)
                .expect("prior relation");
        let dereference_candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Introduction, Expr::Lib(15)),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ]);
        let dereference = issue_dereference_closure_derivation_v2(
            &signature,
            &dereference_candidate,
            15,
            1,
            &BTreeMap::from([(0, prior.projection().clone())]),
        )
        .expect("dereference");
        specialize_verified_closure_derivation_v2(&signature, &dereference, &empty)
            .expect("dereference empty specialization");
        covered.insert(dereference.projection().rule);

        let contextual_candidate = singleton(Expr::Id(
            Box::new(Expr::Lib(15)),
            Box::new(Expr::Var(1)),
            Box::new(Expr::Lib(14)),
        ));
        let contextual_declaration = declaration(
            &signature,
            &contextual_candidate,
            vec![ContextualMotive::Type],
        );
        let contextual = issue_contextual_closure_derivation_v2(
            &signature,
            &contextual_candidate,
            15,
            0,
            &contextual_declaration,
            &BTreeMap::new(),
        )
        .expect("contextual");
        specialize_verified_closure_derivation_v2(&signature, &contextual, &typed)
            .expect("contextual specialization");
        covered.insert(contextual.projection().rule);

        assert_eq!(
            covered,
            CLOSURE_RULE_INVENTORY_V2
                .into_iter()
                .collect::<BTreeSet<_>>()
        );
    }

    #[test]
    fn hashes_rule_tags_motives_pathcon_and_specialized_mutations_fail_closed() {
        let signature = SealedSignature::genesis_del_h15();
        let evidence = closed_evidence(&signature, Expr::Lib(14));

        let mut random_hash = evidence.clone();
        random_hash.evidence_hash = "blake3:random".to_owned();
        assert!(
            issue_motive_typed_closed_assignment_v2(
                &signature,
                15,
                vec![ContextualMotive::Type],
                vec![random_hash],
            )
            .is_err()
        );

        assert!(matches!(
            issue_motive_typed_closed_assignment_v2(
                &signature,
                15,
                vec![ContextualMotive::Element(Expr::Univ)],
                vec![evidence.clone()],
            ),
            Err(MotiveParametricCoherenceV2Error::AssignmentMotiveMismatch { parameter: 1 })
        ));

        let projection_candidate = singleton(Expr::Var(1));
        let projection_declaration = declaration(
            &signature,
            &projection_candidate,
            vec![ContextualMotive::Type],
        );
        let projection = issue_exact_future_body_closure_derivation_v2(
            &signature,
            &projection_candidate,
            15,
            Some(&projection_declaration),
        )
        .expect("projection control");
        let mut wrong_rule = projection.projection().clone();
        wrong_rule.rule = ClosureRuleKind::Guarded;
        assert_eq!(
            replay_verified_closure_derivation_v2(&signature, &wrong_rule),
            Err(MotiveParametricCoherenceV2Error::RuleEvidenceMismatch)
        );

        let formation_projection =
            Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Var(1))]);
        let formation_declaration = declaration(
            &signature,
            &formation_projection,
            vec![ContextualMotive::Type],
        );
        assert!(matches!(
            issue_ambient_projection_closure_derivation_v2(
                &signature,
                &formation_projection,
                15,
                0,
                &formation_declaration,
            ),
            Err(MotiveParametricCoherenceV2Error::ActualBodyUnsupported(_))
        ));

        let path_candidate = singleton(Expr::Id(
            Box::new(Expr::Lib(15)),
            Box::new(Expr::Var(1)),
            Box::new(Expr::PathCon(1)),
        ));
        let path_declaration =
            declaration(&signature, &path_candidate, vec![ContextualMotive::Type]);
        assert!(
            issue_exact_future_body_closure_derivation_v2(
                &signature,
                &path_candidate,
                15,
                Some(&path_declaration),
            )
            .is_err()
        );

        let contextual_candidate = singleton(Expr::Id(
            Box::new(Expr::Lib(15)),
            Box::new(Expr::Var(1)),
            Box::new(Expr::Lib(14)),
        ));
        let contextual_declaration = declaration(
            &signature,
            &contextual_candidate,
            vec![ContextualMotive::Type],
        );
        let contextual = issue_exact_future_body_closure_derivation_v2(
            &signature,
            &contextual_candidate,
            15,
            Some(&contextual_declaration),
        )
        .expect("contextual control");
        let assignment = type_assignment(&signature);
        let specialized =
            specialize_verified_closure_derivation_v2(&signature, &contextual, &assignment)
                .expect("specialized control");
        let mut mutated = specialized.projection().clone();
        mutated.specialized_expression = Expr::Lib(1);
        assert!(replay_specialized_closure_derivation_v2(&signature, &mutated).is_err());
        let mut mutated_relation = specialized.projection().clone();
        mutated_relation
            .specialized_relation
            .relation
            .derivation_hash = "blake3:mutated-specialized-relation".to_owned();
        assert!(replay_specialized_closure_derivation_v2(&signature, &mutated_relation).is_err());
    }

    #[test]
    fn multi_clause_field_evidence_cannot_be_used_as_an_assignment_image() {
        let signature = SealedSignature::genesis_del_h15();
        let prior_candidate = singleton(Expr::Lib(15));
        let prior =
            issue_exact_future_body_closure_derivation_v2(&signature, &prior_candidate, 15, None)
                .expect("prior");
        let candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Introduction, Expr::Lib(15)),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
        ]);
        let relation = issue_dereference_closure_derivation_v2(
            &signature,
            &candidate,
            15,
            1,
            &BTreeMap::from([(0, prior.projection().clone())]),
        )
        .expect("field relation");
        let evidence = issue_closed_internal_evidence_v2(&signature, &relation)
            .expect("closed contextual evidence")
            .projection()
            .clone();
        assert!(matches!(
            issue_motive_typed_closed_assignment_v2(
                &signature,
                15,
                vec![evidence.inferred_motive.clone()],
                vec![evidence],
            ),
            Err(MotiveParametricCoherenceV2Error::AssignmentMotiveMismatch { parameter: 1 })
        ));
    }

    #[test]
    fn beta_extra_child_and_synthetic_declaration_carrier_fail_closed() {
        let signature = SealedSignature::genesis_del_h15();
        let beta_candidate = singleton(Expr::App(
            Box::new(Expr::Lam(Box::new(Expr::Var(2)))),
            Box::new(Expr::Var(1)),
        ));
        let beta_declaration =
            declaration(&signature, &beta_candidate, vec![ContextualMotive::Type]);
        let beta_error = issue_exact_future_body_closure_derivation_v2(
            &signature,
            &beta_candidate,
            15,
            Some(&beta_declaration),
        )
        .expect_err("beta child must need exact reduced-expression replay");
        assert!(beta_error.to_string().contains("beta-reduction"));

        let body = singleton(Expr::Id(
            Box::new(Expr::Lib(15)),
            Box::new(Expr::Var(1)),
            Box::new(Expr::Lib(14)),
        ));
        let carrier = singleton(Expr::Var(1));
        let carrier_declaration = declaration(&signature, &carrier, vec![ContextualMotive::Type]);
        assert_eq!(
            issue_exact_future_body_closure_derivation_v2(
                &signature,
                &body,
                15,
                Some(&carrier_declaration),
            ),
            Err(MotiveParametricCoherenceV2Error::DeclarationCandidateMismatch)
        );
    }

    #[test]
    fn explicit_context_resolves_binder_ambiguity_and_specializes_exactly() {
        let signature = SealedSignature::genesis_del_h15();
        let body = singleton(Expr::Lam(Box::new(Expr::Var(1))));
        let legacy = elaborate_telescope(&signature, &body, 15).expect("legacy elaboration");
        assert_eq!(
            legacy.ambient_parameters, 0,
            "legacy minimal context treats Var(1) as the local binder"
        );

        let declaration = issue_explicit_ambient_context_declaration_token(
            &signature,
            &body,
            15,
            vec![ContextualMotive::Type],
        )
        .expect("explicit one-parameter declaration");
        let source = issue_exact_explicit_future_body_closure_derivation_v2(
            &signature,
            &body,
            15,
            &declaration,
        )
        .expect("exact explicit source");
        replay_verified_closure_derivation_v2(&signature, source.projection())
            .expect("exact explicit source replay");
        assert_eq!(source.projection().ambient_arity, 1);
        let ClosureRuleEvidenceV2::ExplicitContextual { typed_structure } =
            &source.projection().evidence
        else {
            panic!("explicit declaration must remain explicit in evidence");
        };
        assert!(typed_structure.typed_structure_issued);
        assert_eq!(
            typed_structure
                .ambient_uses
                .iter()
                .map(|usage| usage.parameter)
                .collect::<Vec<_>>(),
            vec![1]
        );

        let assignment = type_assignment(&signature);
        let specialized =
            specialize_verified_closure_derivation_v2(&signature, &source, &assignment)
                .expect("explicit contextual specialization");
        replay_specialized_closure_derivation_v2(&signature, specialized.projection())
            .expect("explicit specialization replay");
        assert_eq!(
            specialized.projection().specialized_expression,
            Expr::Lam(Box::new(Expr::Lib(14)))
        );
        assert_eq!(
            specialized.projection().specialized_relation.expression,
            specialized.projection().specialized_expression
        );
    }

    #[test]
    fn indirect_temporal_head_is_not_total_over_type_assignments() {
        let signature = SealedSignature::genesis_del_h15();
        let body = singleton(Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Eventually(Box::new(Expr::Var(1)))),
            Box::new(Expr::Var(2)),
        ))));
        let declaration = issue_explicit_ambient_context_declaration_token(
            &signature,
            &body,
            15,
            vec![ContextualMotive::Type, ContextualMotive::Type],
        )
        .expect("explicit temporal declaration");
        let source_error = issue_exact_explicit_future_body_closure_derivation_v2(
            &signature,
            &body,
            15,
            &declaration,
        )
        .expect_err("a Type-motive temporal head is not a total function source");
        assert!(source_error.to_string().contains("incompatibly"));
        let safe_image = closed_evidence(&signature, Expr::Lib(14));
        let bare_universe_image = closed_evidence(&signature, Expr::Univ);
        let assignment = issue_motive_typed_closed_assignment_v2(
            &signature,
            15,
            vec![ContextualMotive::Type, ContextualMotive::Type],
            vec![safe_image, bare_universe_image],
        )
        .expect("two-parameter Type assignment");
        let substitution = issue_clause_substitution_projection(
            2,
            0,
            assignment.projection(),
            &body.clauses[0].expr,
        )
        .expect("the certified structural substitution itself is defined");
        let specialized = singleton(substitution.result);
        let error = elaborate_telescope(&signature, &specialized, 15)
            .expect_err("a bare-Univ argument refutes total Type-motive specialization");
        assert!(error.to_string().contains("bare Univ"));
    }

    #[test]
    fn explicit_context_rejects_empty_or_inexact_support_and_projection_mutations() {
        let signature = SealedSignature::genesis_del_h15();
        let body = singleton(Expr::Lam(Box::new(Expr::Var(1))));
        assert!(matches!(
            issue_explicit_ambient_context_declaration_token(
                &signature,
                &body,
                15,
                Vec::new(),
            ),
            Err(crate::contextual_internality::ContextualInternalityError::ExplicitAmbientContextEmpty)
        ));

        let support_gap = singleton(Expr::Lam(Box::new(Expr::Var(2))));
        let support_gap_declaration = issue_explicit_ambient_context_declaration_token(
            &signature,
            &support_gap,
            15,
            vec![ContextualMotive::Type],
        )
        .expect("explicit support-gap declaration itself is well typed");
        assert!(matches!(
            issue_exact_explicit_future_body_closure_derivation_v2(
                &signature,
                &support_gap,
                15,
                &support_gap_declaration,
            ),
            Err(MotiveParametricCoherenceV2Error::Underlying { .. })
        ));

        let declaration = issue_explicit_ambient_context_declaration_token(
            &signature,
            &body,
            15,
            vec![ContextualMotive::Type],
        )
        .expect("explicit declaration");
        let source = issue_exact_explicit_future_body_closure_derivation_v2(
            &signature,
            &body,
            15,
            &declaration,
        )
        .expect("explicit source");

        let mut mutations = Vec::new();
        let mut changed_arity = source.projection().clone();
        if let ClosureRuleEvidenceV2::ExplicitContextual { typed_structure } =
            &mut changed_arity.evidence
        {
            typed_structure.ambient_context.ambient_arity = 2;
        }
        mutations.push(changed_arity);

        let mut changed_body = source.projection().clone();
        if let ClosureRuleEvidenceV2::ExplicitContextual { typed_structure } =
            &mut changed_body.evidence
        {
            typed_structure.ambient_context.body_telescope = singleton(Expr::Var(1));
        }
        mutations.push(changed_body);

        let mut changed_role = source.projection().clone();
        if let ClosureRuleEvidenceV2::ExplicitContextual { typed_structure } =
            &mut changed_role.evidence
        {
            typed_structure.ambient_context.declared_role = ClauseRole::Elimination;
        }
        mutations.push(changed_role);

        let mut changed_motive = source.projection().clone();
        if let ClosureRuleEvidenceV2::ExplicitContextual { typed_structure } =
            &mut changed_motive.evidence
        {
            typed_structure.ambient_context.hypotheses[0].motive =
                ContextualMotive::Element(Expr::Lib(15));
        }
        mutations.push(changed_motive);

        let mut changed_support = source.projection().clone();
        if let ClosureRuleEvidenceV2::ExplicitContextual { typed_structure } =
            &mut changed_support.evidence
        {
            typed_structure.ambient_uses.clear();
        }
        mutations.push(changed_support);

        let mut changed_elaboration = source.projection().clone();
        if let ClosureRuleEvidenceV2::ExplicitContextual { typed_structure } =
            &mut changed_elaboration.evidence
        {
            typed_structure.ambient_context.explicit_elaboration_hash =
                "blake3:mutated-explicit-elaboration".to_owned();
        }
        mutations.push(changed_elaboration);

        for mutation in mutations {
            assert!(
                replay_verified_closure_derivation_v2(&signature, &mutation).is_err(),
                "every explicit projection mutation must fail replay"
            );
        }
    }

    #[test]
    fn guarded_motive_weakening_is_recursive() {
        let motive = ContextualMotive::Function {
            domain: Box::new(ContextualMotive::Element(Expr::Var(1))),
            codomain: Box::new(ContextualMotive::Function {
                domain: Box::new(ContextualMotive::Type),
                codomain: Box::new(ContextualMotive::Element(Expr::App(
                    Box::new(Expr::Var(2)),
                    Box::new(Expr::Var(1)),
                ))),
            }),
        };
        assert_eq!(
            weaken_motive_by_one_ambient(&motive),
            ContextualMotive::Function {
                domain: Box::new(ContextualMotive::Element(Expr::Var(2))),
                codomain: Box::new(ContextualMotive::Function {
                    domain: Box::new(ContextualMotive::Type),
                    codomain: Box::new(ContextualMotive::Element(Expr::App(
                        Box::new(Expr::Var(3)),
                        Box::new(Expr::Var(2)),
                    ))),
                }),
            }
        );
    }

    #[test]
    fn unsupported_prior_and_guarded_boundaries_fail_closed() {
        let signature = SealedSignature::genesis_del_h15();

        let contextual_candidate = singleton(Expr::Id(
            Box::new(Expr::Lib(15)),
            Box::new(Expr::Var(1)),
            Box::new(Expr::Lib(14)),
        ));
        let contextual_declaration = declaration(
            &signature,
            &contextual_candidate,
            vec![ContextualMotive::Type],
        );
        let closed_prior = issue_exact_future_body_closure_derivation_v2(
            &signature,
            &singleton(Expr::Lib(15)),
            15,
            None,
        )
        .expect("closed prior control");
        assert!(matches!(
            issue_contextual_closure_derivation_v2(
                &signature,
                &contextual_candidate,
                15,
                0,
                &contextual_declaration,
                &BTreeMap::from([(0, closed_prior.projection().clone())]),
            ),
            Err(MotiveParametricCoherenceV2Error::ActualBodyUnsupported(reason))
                if reason.contains("motive-refinement transport")
        ));

        let prior_formation_candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Introduction, Expr::Lib(15)),
            ClauseRec::new(ClauseRole::Formation, Expr::Var(1)),
        ]);
        assert!(matches!(
            issue_prior_field_projection_closure_derivation_v2(
                &signature,
                &prior_formation_candidate,
                15,
                1,
                closed_prior.projection(),
                None,
            ),
            Err(MotiveParametricCoherenceV2Error::ActualBodyUnsupported(reason))
                if reason.contains("prior-field projection Formation")
        ));

        let prior_open_candidate = singleton(Expr::Var(1));
        let prior_open_declaration = declaration(
            &signature,
            &prior_open_candidate,
            vec![ContextualMotive::Type],
        );
        let prior_open = issue_ambient_projection_closure_derivation_v2(
            &signature,
            &prior_open_candidate,
            15,
            0,
            &prior_open_declaration,
        )
        .expect("open prior projection");
        let shared_candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(1)),
            ClauseRec::new(ClauseRole::Introduction, Expr::Var(3)),
        ]);
        let incompatible_declaration = declaration(
            &signature,
            &shared_candidate,
            vec![
                ContextualMotive::Element(Expr::Lib(15)),
                ContextualMotive::Type,
            ],
        );
        assert_eq!(
            issue_prior_field_projection_closure_derivation_v2(
                &signature,
                &shared_candidate,
                15,
                1,
                prior_open.projection(),
                Some(&incompatible_declaration),
            ),
            Err(MotiveParametricCoherenceV2Error::PriorDeclarationMismatch)
        );

        let guarded_prior_candidate = singleton(Expr::Lam(Box::new(Expr::Var(1))));
        let guarded_prior = issue_exact_future_body_closure_derivation_v2(
            &signature,
            &guarded_prior_candidate,
            15,
            None,
        )
        .expect("guarded prior control");
        let base_candidate = Telescope::new(vec![
            guarded_prior_candidate.clauses[0].clone(),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Id(
                    Box::new(Expr::Lib(15)),
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Lib(14)),
                ),
            ),
        ]);
        let base = issue_ambient_former_closure_derivation_v2(
            &signature,
            &base_candidate,
            15,
            1,
            &BTreeMap::from([(0, guarded_prior.projection().clone())]),
        )
        .expect("multi-clause guarded base");
        let exact_guarded = Telescope::new(
            base_candidate
                .clauses
                .iter()
                .map(|clause| ClauseRec::new(clause.role, weaken_by_one_ambient(&clause.expr)))
                .collect(),
        );
        let exact_guarded_declaration =
            declaration(&signature, &exact_guarded, vec![ContextualMotive::Type]);
        issue_guarded_closure_derivation_v2(
            &signature,
            &exact_guarded,
            15,
            1,
            &exact_guarded_declaration,
            base.projection(),
        )
        .expect("whole-candidate guarded control");

        let mut unrelated_clause_not_weakened = exact_guarded.clone();
        unrelated_clause_not_weakened.clauses[0].expr = base_candidate.clauses[0].expr.clone();
        let mismatched_declaration = declaration(
            &signature,
            &unrelated_clause_not_weakened,
            vec![ContextualMotive::Type],
        );
        assert_eq!(
            issue_guarded_closure_derivation_v2(
                &signature,
                &unrelated_clause_not_weakened,
                15,
                1,
                &mismatched_declaration,
                base.projection(),
            ),
            Err(MotiveParametricCoherenceV2Error::GuardedBaseMismatch)
        );
    }
}
