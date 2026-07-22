//! Motive-sensitive `Internal` evidence for judgments under an explicit
//! ambient telescope.
//!
//! The frozen raw [`Telescope`] format stores clauses only; it does not store
//! the motives of its inferred ambient parameters.  This module therefore
//! keeps declaration and use separate.  A declaration token binds an ordered
//! list of B15-formable motives to one exact candidate hash.  A contextual
//! token can then admit those hypotheses, and only those hypotheses, while
//! replaying the frozen derivation and registered closed instantiations.

use crate::ambient_former_internality::{
    AmbientFormerClosureProjection, issue_ambient_former_closure_token,
    replay_ambient_former_closure_projection,
};
use crate::elaborate::{
    DerivationNode, KernelTy, SealedSignature, candidate_hash,
    elaborate_single_clause_with_derivation, elaborate_telescope,
};
use crate::normalize::substitute_level;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const CONTEXTUAL_INTERNALITY_VERSION: &str = "motive-sensitive-contextual-internality-rule-v1";
pub const CONTEXTUAL_TYPED_STRUCTURE_VERSION: &str =
    "motive-sensitive-contextual-typed-structure-v2";
pub const EXPLICIT_CONTEXTUAL_TYPED_STRUCTURE_VERSION: &str =
    "explicit-ambient-contextual-typed-structure-v2";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextualMotive {
    Type,
    Element(Expr),
    Function {
        domain: Box<ContextualMotive>,
        codomain: Box<ContextualMotive>,
    },
    /// May occur as the classifier of a local term, but is never accepted as
    /// the declared motive of an ambient hypothesis.
    Neutral,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientHypothesisProjection {
    pub parameter: u32,
    pub motive: ContextualMotive,
    pub b15_formable: bool,
    pub formation_derivation_hashes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientContextDeclarationProjection {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub candidate_hash: String,
    pub ambient_arity: u32,
    pub hypotheses: Vec<AmbientHypothesisProjection>,
    pub every_motive_b15_formable: bool,
    pub declaration_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AmbientContextDeclarationToken {
    projection: AmbientContextDeclarationProjection,
}

impl AmbientContextDeclarationToken {
    pub fn projection(&self) -> &AmbientContextDeclarationProjection {
        &self.projection
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AmbientUseProjection {
    pub parameter: u32,
    pub expression: Expr,
    pub derivation_rule: String,
    pub declared_motive: ContextualMotive,
    pub used_in_application_head: bool,
    pub motive_checked: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualInstantiationProjection {
    pub probe_arguments: BTreeMap<u32, Expr>,
    pub probe_argument_derivation_hashes: BTreeMap<u32, String>,
    pub instantiated_candidate: Telescope,
    pub instantiated_candidate_hash: String,
    pub closed_after_instantiation: bool,
    pub closed_clause_evidence: AmbientFormerClosureProjection,
    pub replayed_under_closed_rules: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualInternalityProjection {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub candidate_hash: String,
    pub elaboration_hash: String,
    pub clause_index: u16,
    pub expression: Expr,
    pub ambient_context: AmbientContextDeclarationProjection,
    pub ambient_uses: Vec<AmbientUseProjection>,
    pub certified_prior_clauses: BTreeMap<u16, String>,
    pub every_ambient_use_motive_typed: bool,
    pub every_motive_b15_formable: bool,
    pub every_dependency_strictly_prior: bool,
    pub instantiations: Vec<ContextualInstantiationProjection>,
    pub instantiation_coherence_replayed: bool,
    pub pathcon_remains_charged: bool,
    pub standing_orbit_exception_preserved: bool,
    pub no_credit_anchor_or_orbit_minted: bool,
    pub live_contextual_dependency: bool,
    pub internal_closure_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

/// Probe-free evidence for the typed-structure half of contextual
/// internality.  Unlike [`ContextualInternalityProjection`], this projection
/// makes no finite-instantiation claim: it binds the exact declaration,
/// candidate, kernel derivation, motive audit, and prior-field inventory that
/// a generic substitution theorem must consume.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualTypedStructureProjection {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub candidate: Telescope,
    pub candidate_hash: String,
    pub elaboration_hash: String,
    pub clause_index: u16,
    pub declared_role: ClauseRole,
    pub expression: Expr,
    pub inferred_motive: ContextualMotive,
    pub ambient_context: AmbientContextDeclarationProjection,
    pub ambient_uses: Vec<AmbientUseProjection>,
    pub certified_prior_clauses: BTreeMap<u16, String>,
    pub declaration_replayed: bool,
    pub every_ambient_use_motive_typed: bool,
    pub every_motive_b15_formable: bool,
    pub every_dependency_strictly_prior: bool,
    pub live_contextual_dependency: bool,
    pub pathcon_remains_charged: bool,
    pub no_registered_probe_used: bool,
    pub no_motive_filter_applied: bool,
    pub typed_structure_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContextualTypedStructureToken {
    projection: ContextualTypedStructureProjection,
}

/// Declaration of an exact singleton body in an explicitly supplied ambient
/// context.  Unlike the legacy declaration, this does not infer/minimize the
/// wrapper context from raw syntax.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExplicitAmbientContextDeclarationProjection {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub body_telescope: Telescope,
    pub candidate_hash: String,
    pub declared_role: ClauseRole,
    pub expression: Expr,
    pub ambient_arity: u32,
    pub hypotheses: Vec<AmbientHypothesisProjection>,
    pub every_motive_b15_formable: bool,
    pub explicit_elaboration_hash: String,
    pub declaration_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExplicitAmbientContextDeclarationToken {
    projection: ExplicitAmbientContextDeclarationProjection,
}

impl ExplicitAmbientContextDeclarationToken {
    pub fn projection(&self) -> &ExplicitAmbientContextDeclarationProjection {
        &self.projection
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExplicitContextualTypedStructureProjection {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub body_telescope: Telescope,
    pub candidate_hash: String,
    pub declared_role: ClauseRole,
    pub expression: Expr,
    pub normal_form: Expr,
    pub inferred_motive: ContextualMotive,
    pub ambient_context: ExplicitAmbientContextDeclarationProjection,
    pub ambient_uses: Vec<AmbientUseProjection>,
    pub every_ambient_use_motive_typed: bool,
    pub every_motive_b15_formable: bool,
    pub live_contextual_dependency: bool,
    pub pathcon_remains_charged: bool,
    pub explicit_elaboration_replayed: bool,
    pub typed_structure_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExplicitContextualTypedStructureToken {
    projection: ExplicitContextualTypedStructureProjection,
}

impl ExplicitContextualTypedStructureToken {
    pub fn projection(&self) -> &ExplicitContextualTypedStructureProjection {
        &self.projection
    }
}

impl ContextualTypedStructureToken {
    pub fn projection(&self) -> &ContextualTypedStructureProjection {
        &self.projection
    }

    pub fn marginal_nu(&self) -> u32 {
        self.projection.marginal_nu
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ContextualInternalityToken {
    projection: ContextualInternalityProjection,
}

impl ContextualInternalityToken {
    pub fn projection(&self) -> &ContextualInternalityProjection {
        &self.projection
    }

    pub fn marginal_nu(&self) -> u32 {
        self.projection.marginal_nu
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum ContextualInternalityError {
    #[error("candidate failed elaboration: {0}")]
    Elaboration(String),
    #[error("ambient declaration is not bound to this candidate")]
    DeclarationCandidateMismatch,
    #[error("ambient declaration arity {declared} differs from inferred arity {inferred}")]
    DeclarationArity { declared: u32, inferred: u32 },
    #[error("ambient hypothesis declarations must be exactly the ordered parameters 1..={ambient}")]
    DeclarationOrder { ambient: u32 },
    #[error("ambient motive for parameter {parameter} is not B15-formable")]
    MotiveNotFormable { parameter: u32 },
    #[error("clause index {clause_index} is out of range")]
    ClauseOutOfRange { clause_index: u16 },
    #[error("ambient parameter {parameter} has no declaration")]
    UndeclaredAmbientParameter { parameter: u32 },
    #[error("ambient parameter {parameter} is used incompatibly with its declared motive")]
    IllTypedAmbientUse { parameter: u32 },
    #[error("candidate field {clause_index} lacks a strictly-prior Internal certificate")]
    UncertifiedCandidateField { clause_index: u16 },
    #[error("charged PathCon is not contextual Internal content")]
    ChargedPathConstructor,
    #[error("linear-exponential constructor is outside the frozen grammar")]
    OutsideFrozenGrammar,
    #[error("frozen derivation shape does not match expression {expression:?}")]
    DerivationShape { expression: Expr },
    #[error(
        "contextual beta-reduction needs an exact reduced-expression replay; the legacy derivation child is not accepted as generic evidence"
    )]
    BetaReductionNeedsExactReplay,
    #[error(
        "the selected clause has no live ambient-hypothesis use; weakening/erasure evidence is required instead"
    )]
    NoLiveAmbientUse,
    #[error("no registered closed probes inhabit ambient motive for parameter {parameter}")]
    NoRegisteredProbe { parameter: u32 },
    #[error("closed probe for parameter {parameter} failed: {reason}")]
    ProbeFailure { parameter: u32, reason: String },
    #[error("contextual specialization failed closed-rule replay: {0}")]
    InstantiationReplay(String),
    #[error("contextual projection replay mismatch")]
    ReplayMismatch,
    #[error("explicit ambient declarations require one exact body clause")]
    ExplicitBodyNotSingleton,
    #[error("explicit contextual declarations require at least one live ambient parameter")]
    ExplicitAmbientContextEmpty,
    #[error("explicit ambient declaration is not bound to this exact body/context")]
    ExplicitDeclarationMismatch,
    #[error(
        "explicit body uses ambient parameters {used:?}, expected exact support 1..={declared}"
    )]
    ExplicitAmbientSupportMismatch { declared: u32, used: Vec<u32> },
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CONTEXTUAL_INTERNALITY_VERSION, domain, value))
        .expect("contextual Internal evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn typed_structure_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CONTEXTUAL_TYPED_STRUCTURE_VERSION, domain, value))
        .expect("contextual typed-structure evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn explicit_context_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(EXPLICIT_CONTEXTUAL_TYPED_STRUCTURE_VERSION, domain, value))
        .expect("explicit contextual evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn kernel_motive(ty: &KernelTy) -> ContextualMotive {
    match ty {
        KernelTy::Type => ContextualMotive::Type,
        KernelTy::El(expr) => ContextualMotive::Element(expr.clone()),
        KernelTy::Fun(domain, codomain) => ContextualMotive::Function {
            domain: Box::new(kernel_motive(domain)),
            codomain: Box::new(kernel_motive(codomain)),
        },
        KernelTy::PathDecl { .. } | KernelTy::Neutral => ContextualMotive::Neutral,
    }
}

fn parse_rule_index(rule: &str, prefix: &str) -> Option<u32> {
    rule.strip_prefix(prefix)?.parse().ok()
}

fn motive_formation_hashes(
    signature: &SealedSignature,
    motive: &ContextualMotive,
    visible_library: u32,
) -> Result<Vec<String>, ContextualInternalityError> {
    match motive {
        ContextualMotive::Type => {
            let witness = Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, Expr::Univ)]);
            let elaboration = elaborate_telescope(signature, &witness, visible_library)
                .map_err(|error| ContextualInternalityError::Elaboration(error.to_string()))?;
            Ok(vec![elaboration.derivation_hash])
        }
        ContextualMotive::Element(expr) => {
            if expr.var_refs().is_empty() && !contains_charged_or_outside(expr) {
                let witness =
                    Telescope::new(vec![ClauseRec::new(ClauseRole::Formation, expr.clone())]);
                let elaboration = elaborate_telescope(signature, &witness, visible_library)
                    .map_err(|error| ContextualInternalityError::Elaboration(error.to_string()))?;
                if elaboration.clauses[0].kernel_ty == KernelTy::Type {
                    return Ok(vec![elaboration.derivation_hash]);
                }
            }
            Err(ContextualInternalityError::MotiveNotFormable { parameter: 0 })
        }
        ContextualMotive::Function { domain, codomain } => {
            let mut hashes = motive_formation_hashes(signature, domain, visible_library)?;
            hashes.extend(motive_formation_hashes(
                signature,
                codomain,
                visible_library,
            )?);
            Ok(hashes)
        }
        ContextualMotive::Neutral => {
            Err(ContextualInternalityError::MotiveNotFormable { parameter: 0 })
        }
    }
}

fn contains_charged_or_outside(expr: &Expr) -> bool {
    match expr {
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

pub fn issue_ambient_context_declaration_token(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    motives: Vec<ContextualMotive>,
) -> Result<AmbientContextDeclarationToken, ContextualInternalityError> {
    let elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| ContextualInternalityError::Elaboration(error.to_string()))?;
    let declared = motives.len() as u32;
    if declared != elaboration.ambient_parameters {
        return Err(ContextualInternalityError::DeclarationArity {
            declared,
            inferred: elaboration.ambient_parameters,
        });
    }
    let mut hypotheses = Vec::with_capacity(motives.len());
    for (index, motive) in motives.into_iter().enumerate() {
        let parameter = index as u32 + 1;
        let formation_derivation_hashes =
            motive_formation_hashes(signature, &motive, visible_library)
                .map_err(|_| ContextualInternalityError::MotiveNotFormable { parameter })?;
        hypotheses.push(AmbientHypothesisProjection {
            parameter,
            motive,
            b15_formable: true,
            formation_derivation_hashes,
        });
    }
    let every_motive_b15_formable = hypotheses.iter().all(|motive| motive.b15_formable);
    let mut projection = AmbientContextDeclarationProjection {
        version: CONTEXTUAL_INTERNALITY_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        candidate_hash: candidate_hash(candidate),
        ambient_arity: elaboration.ambient_parameters,
        hypotheses,
        every_motive_b15_formable,
        declaration_hash: String::new(),
    };
    projection.declaration_hash = tagged_hash("ambient-context-declaration", &projection);
    Ok(AmbientContextDeclarationToken { projection })
}

pub fn replay_ambient_context_declaration_projection(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    projection: &AmbientContextDeclarationProjection,
) -> Result<(), ContextualInternalityError> {
    let motives = projection
        .hypotheses
        .iter()
        .map(|hypothesis| hypothesis.motive.clone())
        .collect();
    let token =
        issue_ambient_context_declaration_token(signature, candidate, visible_library, motives)?;
    if token.projection() == projection {
        Ok(())
    } else {
        Err(ContextualInternalityError::ReplayMismatch)
    }
}

pub fn issue_explicit_ambient_context_declaration_token(
    signature: &SealedSignature,
    body_telescope: &Telescope,
    visible_library: u32,
    motives: Vec<ContextualMotive>,
) -> Result<ExplicitAmbientContextDeclarationToken, ContextualInternalityError> {
    if body_telescope.kappa() != 1 {
        return Err(ContextualInternalityError::ExplicitBodyNotSingleton);
    }
    if motives.is_empty() {
        return Err(ContextualInternalityError::ExplicitAmbientContextEmpty);
    }
    let clause = &body_telescope.clauses[0];
    let ambient_arity = motives.len() as u32;
    let (elaboration, derivation) =
        elaborate_single_clause_with_derivation(&clause.expr, ambient_arity, &[], visible_library)
            .map_err(|error| ContextualInternalityError::Elaboration(error.to_string()))?;
    let mut hypotheses = Vec::with_capacity(motives.len());
    for (index, motive) in motives.into_iter().enumerate() {
        let parameter = index as u32 + 1;
        let formation_derivation_hashes =
            motive_formation_hashes(signature, &motive, visible_library)
                .map_err(|_| ContextualInternalityError::MotiveNotFormable { parameter })?;
        hypotheses.push(AmbientHypothesisProjection {
            parameter,
            motive,
            b15_formable: true,
            formation_derivation_hashes,
        });
    }
    let every_motive_b15_formable = hypotheses.iter().all(|item| item.b15_formable);
    let explicit_elaboration_hash = explicit_context_hash(
        "explicit-single-body-elaboration",
        &(
            signature.digest(),
            visible_library,
            body_telescope,
            ambient_arity,
            &elaboration,
            &derivation,
        ),
    );
    let mut projection = ExplicitAmbientContextDeclarationProjection {
        version: EXPLICIT_CONTEXTUAL_TYPED_STRUCTURE_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        body_telescope: body_telescope.clone(),
        candidate_hash: candidate_hash(body_telescope),
        declared_role: clause.role,
        expression: clause.expr.clone(),
        ambient_arity,
        hypotheses,
        every_motive_b15_formable,
        explicit_elaboration_hash,
        declaration_hash: String::new(),
    };
    projection.declaration_hash =
        explicit_context_hash("explicit-ambient-context-declaration", &projection);
    Ok(ExplicitAmbientContextDeclarationToken { projection })
}

pub fn replay_explicit_ambient_context_declaration_projection(
    signature: &SealedSignature,
    projection: &ExplicitAmbientContextDeclarationProjection,
) -> Result<(), ContextualInternalityError> {
    let token = issue_explicit_ambient_context_declaration_token(
        signature,
        &projection.body_telescope,
        projection.visible_library,
        projection
            .hypotheses
            .iter()
            .map(|hypothesis| hypothesis.motive.clone())
            .collect(),
    )?;
    if token.projection() == projection {
        Ok(())
    } else {
        Err(ContextualInternalityError::ReplayMismatch)
    }
}

#[derive(Clone)]
struct Audit {
    motive: ContextualMotive,
    uses: Vec<AmbientUseProjection>,
}

fn merge_uses(parts: impl IntoIterator<Item = Audit>) -> Vec<AmbientUseProjection> {
    parts.into_iter().flat_map(|part| part.uses).collect()
}

fn audit_term(
    expression: &Expr,
    derivation: &DerivationNode,
    hypotheses: &[AmbientHypothesisProjection],
    certified_prior: &BTreeMap<u16, String>,
    application_head: bool,
) -> Result<Audit, ContextualInternalityError> {
    match expression {
        Expr::PathCon(_) => return Err(ContextualInternalityError::ChargedPathConstructor),
        Expr::Bang(_) | Expr::WhyNot(_) => {
            return Err(ContextualInternalityError::OutsideFrozenGrammar);
        }
        Expr::Var(_) if derivation.children.is_empty() => {
            if let Some(parameter) = parse_rule_index(&derivation.rule, "ambient-param-") {
                let hypothesis = hypotheses
                    .iter()
                    .find(|hypothesis| hypothesis.parameter == parameter)
                    .ok_or(ContextualInternalityError::UndeclaredAmbientParameter { parameter })?;
                return Ok(Audit {
                    motive: hypothesis.motive.clone(),
                    uses: vec![AmbientUseProjection {
                        parameter,
                        expression: expression.clone(),
                        derivation_rule: derivation.rule.clone(),
                        declared_motive: hypothesis.motive.clone(),
                        used_in_application_head: application_head,
                        motive_checked: true,
                    }],
                });
            }
            if let Some(clause) = parse_rule_index(&derivation.rule, "field-ref-") {
                let clause_index = u16::try_from(clause).map_err(|_| {
                    ContextualInternalityError::UncertifiedCandidateField {
                        clause_index: u16::MAX,
                    }
                })?;
                if !certified_prior.contains_key(&clause_index) {
                    return Err(ContextualInternalityError::UncertifiedCandidateField {
                        clause_index,
                    });
                }
            }
            return Ok(Audit {
                motive: kernel_motive(&derivation.kernel_ty),
                uses: Vec::new(),
            });
        }
        Expr::Univ | Expr::Lib(_) if derivation.children.is_empty() => {
            return Ok(Audit {
                motive: kernel_motive(&derivation.kernel_ty),
                uses: Vec::new(),
            });
        }
        Expr::App(function, argument) if derivation.children.len() >= 2 => {
            let function_audit = audit_term(
                function,
                &derivation.children[0],
                hypotheses,
                certified_prior,
                true,
            )?;
            let argument_audit = audit_term(
                argument,
                &derivation.children[1],
                hypotheses,
                certified_prior,
                false,
            )?;
            let live_parameters = function_audit
                .uses
                .iter()
                .map(|usage| usage.parameter)
                .collect::<BTreeSet<_>>();
            let motive = if live_parameters.is_empty() {
                kernel_motive(&derivation.kernel_ty)
            } else {
                let ContextualMotive::Function { domain, codomain } = &function_audit.motive else {
                    return Err(ContextualInternalityError::IllTypedAmbientUse {
                        parameter: *live_parameters.iter().next().expect("nonempty"),
                    });
                };
                if domain.as_ref() != &argument_audit.motive {
                    return Err(ContextualInternalityError::IllTypedAmbientUse {
                        parameter: *live_parameters.iter().next().expect("nonempty"),
                    });
                }
                codomain.as_ref().clone()
            };
            let mut uses = function_audit.uses;
            uses.extend(argument_audit.uses);
            for reduced in derivation.children.iter().skip(2) {
                uses.extend(
                    audit_term(expression, reduced, hypotheses, certified_prior, false)?.uses,
                );
            }
            return Ok(Audit { motive, uses });
        }
        Expr::Lam(body) if derivation.children.len() == 1 => {
            let body = audit_term(
                body,
                &derivation.children[0],
                hypotheses,
                certified_prior,
                false,
            )?;
            return Ok(Audit {
                motive: ContextualMotive::Function {
                    domain: Box::new(ContextualMotive::Neutral),
                    codomain: Box::new(body.motive),
                },
                uses: body.uses,
            });
        }
        Expr::Pi(left, right) | Expr::Sigma(left, right) if derivation.children.len() == 2 => {
            let left = audit_term(
                left,
                &derivation.children[0],
                hypotheses,
                certified_prior,
                false,
            )?;
            let right = audit_term(
                right,
                &derivation.children[1],
                hypotheses,
                certified_prior,
                false,
            )?;
            return Ok(Audit {
                motive: ContextualMotive::Type,
                uses: merge_uses([left, right]),
            });
        }
        Expr::Id(ty, left, right) if derivation.children.len() == 3 => {
            let parts = [
                audit_term(
                    ty,
                    &derivation.children[0],
                    hypotheses,
                    certified_prior,
                    false,
                )?,
                audit_term(
                    left,
                    &derivation.children[1],
                    hypotheses,
                    certified_prior,
                    false,
                )?,
                audit_term(
                    right,
                    &derivation.children[2],
                    hypotheses,
                    certified_prior,
                    false,
                )?,
            ];
            return Ok(Audit {
                motive: ContextualMotive::Type,
                uses: merge_uses(parts),
            });
        }
        Expr::Refl(inner)
        | Expr::Susp(inner)
        | Expr::Trunc(inner)
        | Expr::Flat(inner)
        | Expr::Sharp(inner)
        | Expr::Disc(inner)
        | Expr::Shape(inner)
        | Expr::Next(inner)
        | Expr::Eventually(inner)
            if derivation.children.len() == 1 =>
        {
            let inner = audit_term(
                inner,
                &derivation.children[0],
                hypotheses,
                certified_prior,
                false,
            )?;
            return Ok(Audit {
                motive: kernel_motive(&derivation.kernel_ty),
                uses: inner.uses,
            });
        }
        _ => {}
    }
    Err(ContextualInternalityError::DerivationShape {
        expression: expression.clone(),
    })
}

fn contains_beta_reduction_child(derivation: &DerivationNode) -> bool {
    (derivation.rule == "app-beta" && derivation.children.len() > 2)
        || derivation
            .children
            .iter()
            .any(contains_beta_reduction_child)
}

pub fn issue_explicit_contextual_typed_structure_token(
    signature: &SealedSignature,
    declaration: &ExplicitAmbientContextDeclarationToken,
) -> Result<ExplicitContextualTypedStructureToken, ContextualInternalityError> {
    let context = declaration.projection();
    if context.body_telescope.kappa() != 1 {
        return Err(ContextualInternalityError::ExplicitBodyNotSingleton);
    }
    replay_explicit_ambient_context_declaration_projection(signature, context)?;
    let clause = &context.body_telescope.clauses[0];
    if context.signature_digest != signature.digest()
        || context.candidate_hash != candidate_hash(&context.body_telescope)
        || context.declared_role != clause.role
        || context.expression != clause.expr
    {
        return Err(ContextualInternalityError::ExplicitDeclarationMismatch);
    }
    let (elaboration, derivation) = elaborate_single_clause_with_derivation(
        &clause.expr,
        context.ambient_arity,
        &[],
        context.visible_library,
    )
    .map_err(|error| ContextualInternalityError::Elaboration(error.to_string()))?;
    if contains_beta_reduction_child(&derivation) {
        return Err(ContextualInternalityError::BetaReductionNeedsExactReplay);
    }
    let audit = audit_term(
        &clause.expr,
        &derivation,
        &context.hypotheses,
        &BTreeMap::new(),
        false,
    )?;
    let mut used = audit
        .uses
        .iter()
        .map(|usage| usage.parameter)
        .collect::<Vec<_>>();
    used.sort_unstable();
    used.dedup();
    let expected = (1..=context.ambient_arity).collect::<Vec<_>>();
    if used != expected {
        return Err(ContextualInternalityError::ExplicitAmbientSupportMismatch {
            declared: context.ambient_arity,
            used,
        });
    }
    let every_ambient_use_motive_typed = audit.uses.iter().all(|usage| usage.motive_checked);
    let every_motive_b15_formable = context.every_motive_b15_formable;
    let live_contextual_dependency = !audit.uses.is_empty();
    let pathcon_remains_charged = !contains_charged_or_outside(&clause.expr);
    let explicit_elaboration_replayed = true;
    let typed_structure_issued = every_ambient_use_motive_typed
        && every_motive_b15_formable
        && live_contextual_dependency
        && pathcon_remains_charged
        && explicit_elaboration_replayed;
    if !typed_structure_issued {
        return Err(ContextualInternalityError::NoLiveAmbientUse);
    }
    let mut projection = ExplicitContextualTypedStructureProjection {
        version: EXPLICIT_CONTEXTUAL_TYPED_STRUCTURE_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library: context.visible_library,
        body_telescope: context.body_telescope.clone(),
        candidate_hash: context.candidate_hash.clone(),
        declared_role: clause.role,
        expression: clause.expr.clone(),
        normal_form: elaboration.normal_form,
        inferred_motive: audit.motive,
        ambient_context: context.clone(),
        ambient_uses: audit.uses,
        every_ambient_use_motive_typed,
        every_motive_b15_formable,
        live_contextual_dependency,
        pathcon_remains_charged,
        explicit_elaboration_replayed,
        typed_structure_issued,
        marginal_nu: 0,
        derivation_hash: String::new(),
    };
    projection.derivation_hash =
        explicit_context_hash("explicit-contextual-typed-structure", &projection);
    Ok(ExplicitContextualTypedStructureToken { projection })
}

pub fn replay_explicit_contextual_typed_structure_projection(
    signature: &SealedSignature,
    projection: &ExplicitContextualTypedStructureProjection,
) -> Result<(), ContextualInternalityError> {
    let declaration = issue_explicit_ambient_context_declaration_token(
        signature,
        &projection.body_telescope,
        projection.visible_library,
        projection
            .ambient_context
            .hypotheses
            .iter()
            .map(|hypothesis| hypothesis.motive.clone())
            .collect(),
    )?;
    let token = issue_explicit_contextual_typed_structure_token(signature, &declaration)?;
    if token.projection() == projection {
        Ok(())
    } else {
        Err(ContextualInternalityError::ReplayMismatch)
    }
}

/// Issue the exact typed-structure premise used by the generic substitution
/// theorem.  This deliberately stops before the legacy registered-probe
/// experiment: the returned token says only that the frozen kernel derivation
/// is well typed under the declared ambient telescope and that every candidate
/// field dependency is backed by a strictly-prior certificate.
pub fn issue_contextual_typed_structure_token(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    declaration: &AmbientContextDeclarationToken,
    certified_prior_clauses: &BTreeMap<u16, String>,
) -> Result<ContextualTypedStructureToken, ContextualInternalityError> {
    let elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| ContextualInternalityError::Elaboration(error.to_string()))?;
    if declaration.projection.candidate_hash != candidate_hash(candidate)
        || declaration.projection.signature_digest != signature.digest()
        || declaration.projection.visible_library != visible_library
    {
        return Err(ContextualInternalityError::DeclarationCandidateMismatch);
    }
    if declaration.projection.ambient_arity != elaboration.ambient_parameters {
        return Err(ContextualInternalityError::DeclarationArity {
            declared: declaration.projection.ambient_arity,
            inferred: elaboration.ambient_parameters,
        });
    }
    if !declaration
        .projection
        .hypotheses
        .iter()
        .map(|hypothesis| hypothesis.parameter)
        .eq(1..=elaboration.ambient_parameters)
    {
        return Err(ContextualInternalityError::DeclarationOrder {
            ambient: elaboration.ambient_parameters,
        });
    }
    replay_ambient_context_declaration_projection(
        signature,
        candidate,
        visible_library,
        &declaration.projection,
    )?;

    let index = usize::from(clause_index);
    let clause = candidate
        .clauses
        .get(index)
        .ok_or(ContextualInternalityError::ClauseOutOfRange { clause_index })?;
    let clause_elaboration = elaboration
        .clauses
        .get(index)
        .ok_or(ContextualInternalityError::ClauseOutOfRange { clause_index })?;
    if contains_beta_reduction_child(&clause_elaboration.derivation) {
        return Err(ContextualInternalityError::BetaReductionNeedsExactReplay);
    }
    if certified_prior_clauses
        .keys()
        .any(|dependency| *dependency >= clause_index)
    {
        return Err(ContextualInternalityError::UncertifiedCandidateField { clause_index });
    }

    let audit = audit_term(
        &clause.expr,
        &clause_elaboration.derivation,
        &declaration.projection.hypotheses,
        certified_prior_clauses,
        false,
    )?;
    if audit.uses.is_empty() {
        return Err(ContextualInternalityError::NoLiveAmbientUse);
    }

    let every_ambient_use_motive_typed = audit.uses.iter().all(|usage| usage.motive_checked);
    let every_motive_b15_formable = declaration.projection.every_motive_b15_formable;
    let every_dependency_strictly_prior = certified_prior_clauses
        .keys()
        .all(|dependency| *dependency < clause_index);
    let declaration_replayed = true;
    let live_contextual_dependency = !audit.uses.is_empty();
    let pathcon_remains_charged = !contains_charged_or_outside(&clause.expr);
    let no_registered_probe_used = true;
    let no_motive_filter_applied = true;
    let marginal_nu = 0;
    let typed_structure_issued = declaration_replayed
        && every_ambient_use_motive_typed
        && every_motive_b15_formable
        && every_dependency_strictly_prior
        && live_contextual_dependency
        && pathcon_remains_charged;

    let mut projection = ContextualTypedStructureProjection {
        version: CONTEXTUAL_TYPED_STRUCTURE_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        candidate: candidate.clone(),
        candidate_hash: candidate_hash(candidate),
        elaboration_hash: elaboration.derivation_hash,
        clause_index,
        declared_role: clause.role,
        expression: clause.expr.clone(),
        inferred_motive: audit.motive,
        ambient_context: declaration.projection.clone(),
        ambient_uses: audit.uses,
        certified_prior_clauses: certified_prior_clauses.clone(),
        declaration_replayed,
        every_ambient_use_motive_typed,
        every_motive_b15_formable,
        every_dependency_strictly_prior,
        live_contextual_dependency,
        pathcon_remains_charged,
        no_registered_probe_used,
        no_motive_filter_applied,
        typed_structure_issued,
        marginal_nu,
        derivation_hash: String::new(),
    };
    projection.derivation_hash =
        typed_structure_hash("contextual-typed-structure-token", &projection);
    Ok(ContextualTypedStructureToken { projection })
}

pub fn replay_contextual_typed_structure_projection(
    signature: &SealedSignature,
    projection: &ContextualTypedStructureProjection,
) -> Result<(), ContextualInternalityError> {
    let declaration = issue_ambient_context_declaration_token(
        signature,
        &projection.candidate,
        projection.visible_library,
        projection
            .ambient_context
            .hypotheses
            .iter()
            .map(|hypothesis| hypothesis.motive.clone())
            .collect(),
    )?;
    let token = issue_contextual_typed_structure_token(
        signature,
        &projection.candidate,
        projection.visible_library,
        projection.clause_index,
        &declaration,
        &projection.certified_prior_clauses,
    )?;
    if token.projection() == projection {
        Ok(())
    } else {
        Err(ContextualInternalityError::ReplayMismatch)
    }
}

fn registered_probes(motive: &ContextualMotive) -> Option<[Expr; 2]> {
    match motive {
        ContextualMotive::Type => Some([Expr::Univ, Expr::Lib(15)]),
        ContextualMotive::Function { domain, codomain }
            if domain.as_ref() == &ContextualMotive::Type
                && codomain.as_ref() == &ContextualMotive::Type =>
        {
            Some([
                Expr::Lam(Box::new(Expr::Univ)),
                Expr::Lam(Box::new(Expr::Lib(15))),
            ])
        }
        _ => None,
    }
}

fn probe_derivation_hash(
    signature: &SealedSignature,
    parameter: u32,
    probe: &Expr,
    visible_library: u32,
) -> Result<String, ContextualInternalityError> {
    if !probe.var_refs().is_empty() || contains_charged_or_outside(probe) {
        return Err(ContextualInternalityError::ProbeFailure {
            parameter,
            reason: "probe is not closed transparent content".to_owned(),
        });
    }
    let candidate = Telescope::new(vec![ClauseRec::new(
        ClauseRole::Introduction,
        probe.clone(),
    )]);
    elaborate_telescope(signature, &candidate, visible_library)
        .map(|elaboration| elaboration.derivation_hash)
        .map_err(|error| ContextualInternalityError::ProbeFailure {
            parameter,
            reason: error.to_string(),
        })
}

fn instantiate_candidate(candidate: &Telescope, arguments: &BTreeMap<u32, Expr>) -> Telescope {
    let mut clauses = candidate.clauses.clone();
    for parameter in arguments.keys().copied().rev() {
        let value = &arguments[&parameter];
        for clause in &mut clauses {
            clause.expr = substitute_level(&clause.expr, parameter, value);
        }
    }
    Telescope::new(clauses)
}

fn issue_instantiations(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    context: &AmbientContextDeclarationProjection,
    certified_prior: &BTreeMap<u16, String>,
) -> Result<Vec<ContextualInstantiationProjection>, ContextualInternalityError> {
    let mut probe_sets = Vec::with_capacity(context.hypotheses.len());
    for hypothesis in &context.hypotheses {
        let probes = registered_probes(&hypothesis.motive).ok_or(
            ContextualInternalityError::NoRegisteredProbe {
                parameter: hypothesis.parameter,
            },
        )?;
        probe_sets.push((hypothesis.parameter, probes));
    }
    let mut projections = Vec::with_capacity(2);
    for probe_index in 0..2 {
        let mut probe_arguments = BTreeMap::new();
        let mut probe_argument_derivation_hashes = BTreeMap::new();
        for (parameter, probes) in &probe_sets {
            let probe = probes[probe_index].clone();
            let hash = probe_derivation_hash(signature, *parameter, &probe, visible_library)?;
            probe_arguments.insert(*parameter, probe);
            probe_argument_derivation_hashes.insert(*parameter, hash);
        }
        let instantiated_candidate = instantiate_candidate(candidate, &probe_arguments);
        let elaboration = elaborate_telescope(signature, &instantiated_candidate, visible_library)
            .map_err(|error| ContextualInternalityError::InstantiationReplay(error.to_string()))?;
        let closed_after_instantiation = elaboration.ambient_parameters == 0;
        if !closed_after_instantiation {
            return Err(ContextualInternalityError::InstantiationReplay(
                "registered probes did not close the ambient telescope".to_owned(),
            ));
        }
        let token = issue_ambient_former_closure_token(
            signature,
            &instantiated_candidate,
            visible_library,
            clause_index,
            certified_prior,
        )
        .map_err(|error| ContextualInternalityError::InstantiationReplay(error.to_string()))?;
        let closed_clause_evidence = token.projection().clone();
        replay_ambient_former_closure_projection(
            signature,
            &instantiated_candidate,
            visible_library,
            &closed_clause_evidence,
        )
        .map_err(|error| ContextualInternalityError::InstantiationReplay(error.to_string()))?;
        let replayed_under_closed_rules = true;
        let instantiated_candidate_hash = candidate_hash(&instantiated_candidate);
        let derivation_hash = tagged_hash(
            "contextual-instantiation",
            &(
                &probe_arguments,
                &probe_argument_derivation_hashes,
                &instantiated_candidate,
                &instantiated_candidate_hash,
                closed_after_instantiation,
                &closed_clause_evidence,
                replayed_under_closed_rules,
            ),
        );
        projections.push(ContextualInstantiationProjection {
            probe_arguments,
            probe_argument_derivation_hashes,
            instantiated_candidate,
            instantiated_candidate_hash,
            closed_after_instantiation,
            closed_clause_evidence,
            replayed_under_closed_rules,
            derivation_hash,
        });
    }
    Ok(projections)
}

pub fn issue_contextual_internality_token(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    declaration: &AmbientContextDeclarationToken,
    certified_prior_clauses: &BTreeMap<u16, String>,
) -> Result<ContextualInternalityToken, ContextualInternalityError> {
    let elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| ContextualInternalityError::Elaboration(error.to_string()))?;
    if declaration.projection.candidate_hash != candidate_hash(candidate)
        || declaration.projection.signature_digest != signature.digest()
        || declaration.projection.visible_library != visible_library
    {
        return Err(ContextualInternalityError::DeclarationCandidateMismatch);
    }
    if declaration.projection.ambient_arity != elaboration.ambient_parameters {
        return Err(ContextualInternalityError::DeclarationArity {
            declared: declaration.projection.ambient_arity,
            inferred: elaboration.ambient_parameters,
        });
    }
    if !declaration
        .projection
        .hypotheses
        .iter()
        .map(|hypothesis| hypothesis.parameter)
        .eq(1..=elaboration.ambient_parameters)
    {
        return Err(ContextualInternalityError::DeclarationOrder {
            ambient: elaboration.ambient_parameters,
        });
    }
    replay_ambient_context_declaration_projection(
        signature,
        candidate,
        visible_library,
        &declaration.projection,
    )?;
    let index = usize::from(clause_index);
    let clause = candidate
        .clauses
        .get(index)
        .ok_or(ContextualInternalityError::ClauseOutOfRange { clause_index })?;
    let clause_elaboration = elaboration
        .clauses
        .get(index)
        .ok_or(ContextualInternalityError::ClauseOutOfRange { clause_index })?;
    if certified_prior_clauses
        .keys()
        .any(|dependency| *dependency >= clause_index)
    {
        return Err(ContextualInternalityError::UncertifiedCandidateField { clause_index });
    }
    let audit = audit_term(
        &clause.expr,
        &clause_elaboration.derivation,
        &declaration.projection.hypotheses,
        certified_prior_clauses,
        false,
    )?;
    if audit.uses.is_empty() {
        return Err(ContextualInternalityError::NoLiveAmbientUse);
    }
    let instantiations = issue_instantiations(
        signature,
        candidate,
        visible_library,
        clause_index,
        &declaration.projection,
        certified_prior_clauses,
    )?;
    let every_ambient_use_motive_typed = audit.uses.iter().all(|usage| usage.motive_checked);
    let every_motive_b15_formable = declaration.projection.every_motive_b15_formable;
    let every_dependency_strictly_prior = certified_prior_clauses
        .keys()
        .all(|dependency| *dependency < clause_index);
    let instantiation_coherence_replayed = !instantiations.is_empty()
        && instantiations
            .iter()
            .all(|probe| probe.closed_after_instantiation && probe.replayed_under_closed_rules);
    let pathcon_remains_charged = true;
    let standing_orbit_exception_preserved = true;
    let no_credit_anchor_or_orbit_minted = true;
    let live_contextual_dependency = true;
    let marginal_nu = 0;
    let internal_closure_issued = every_ambient_use_motive_typed
        && every_motive_b15_formable
        && every_dependency_strictly_prior
        && instantiation_coherence_replayed
        && pathcon_remains_charged
        && standing_orbit_exception_preserved
        && no_credit_anchor_or_orbit_minted;
    let mut projection = ContextualInternalityProjection {
        version: CONTEXTUAL_INTERNALITY_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        candidate_hash: candidate_hash(candidate),
        elaboration_hash: elaboration.derivation_hash,
        clause_index,
        expression: clause.expr.clone(),
        ambient_context: declaration.projection.clone(),
        ambient_uses: audit.uses,
        certified_prior_clauses: certified_prior_clauses.clone(),
        every_ambient_use_motive_typed,
        every_motive_b15_formable,
        every_dependency_strictly_prior,
        instantiations,
        instantiation_coherence_replayed,
        pathcon_remains_charged,
        standing_orbit_exception_preserved,
        no_credit_anchor_or_orbit_minted,
        live_contextual_dependency,
        internal_closure_issued,
        marginal_nu,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("contextual-internality-token", &projection);
    Ok(ContextualInternalityToken { projection })
}

pub fn replay_contextual_internality_projection(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    projection: &ContextualInternalityProjection,
) -> Result<(), ContextualInternalityError> {
    let declaration = issue_ambient_context_declaration_token(
        signature,
        candidate,
        visible_library,
        projection
            .ambient_context
            .hypotheses
            .iter()
            .map(|hypothesis| hypothesis.motive.clone())
            .collect(),
    )?;
    let token = issue_contextual_internality_token(
        signature,
        candidate,
        visible_library,
        projection.clause_index,
        &declaration,
        &projection.certified_prior_clauses,
    )?;
    if token.projection() == projection {
        Ok(())
    } else {
        Err(ContextualInternalityError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn live_candidate() -> Telescope {
        Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Var(3)),
                    Box::new(Expr::App(Box::new(Expr::Var(1)), Box::new(Expr::Lib(15)))),
                ))),
            ),
        ])
    }

    fn function_motive() -> ContextualMotive {
        ContextualMotive::Function {
            domain: Box::new(ContextualMotive::Type),
            codomain: Box::new(ContextualMotive::Type),
        }
    }

    #[test]
    fn explicit_live_context_replays_at_zero_credit() {
        let signature = SealedSignature::genesis_del_h15();
        let candidate = live_candidate();
        let declaration = issue_ambient_context_declaration_token(
            &signature,
            &candidate,
            15,
            vec![function_motive()],
        )
        .expect("declaration");
        let prior = BTreeMap::from([(0, "earned-internal-clause-0".to_owned())]);
        let token =
            issue_contextual_internality_token(&signature, &candidate, 15, 1, &declaration, &prior)
                .expect("contextual token");
        assert!(token.projection().internal_closure_issued);
        assert!(token.projection().ambient_uses[0].used_in_application_head);
        assert!(token.projection().instantiation_coherence_replayed);
        assert_eq!(token.marginal_nu(), 0);
        replay_contextual_internality_projection(&signature, &candidate, 15, token.projection())
            .expect("projection replay");
    }

    #[test]
    fn v7_witness_has_no_live_ambient_use() {
        let signature = SealedSignature::genesis_del_h15();
        let candidate = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Var(3)),
                    Box::new(Expr::Lib(15)),
                ))),
            ),
        ]);
        let declaration = issue_ambient_context_declaration_token(
            &signature,
            &candidate,
            15,
            vec![function_motive()],
        )
        .expect("declaration");
        assert_eq!(
            issue_contextual_internality_token(
                &signature,
                &candidate,
                15,
                1,
                &declaration,
                &BTreeMap::from([(0, "earned".to_owned())]),
            ),
            Err(ContextualInternalityError::NoLiveAmbientUse)
        );
    }

    #[test]
    fn ill_typed_motive_pathcon_and_mutation_fail_closed() {
        let signature = SealedSignature::genesis_del_h15();
        let candidate = live_candidate();
        let wrong = issue_ambient_context_declaration_token(
            &signature,
            &candidate,
            15,
            vec![ContextualMotive::Type],
        )
        .expect("formable wrong declaration");
        assert!(matches!(
            issue_contextual_internality_token(
                &signature,
                &candidate,
                15,
                1,
                &wrong,
                &BTreeMap::from([(0, "earned".to_owned())]),
            ),
            Err(ContextualInternalityError::IllTypedAmbientUse { parameter: 1 })
        ));

        let path = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(
                ClauseRole::PathAttach,
                Expr::Lam(Box::new(Expr::Pi(
                    Box::new(Expr::App(
                        Box::new(Expr::Var(1)),
                        Box::new(Expr::PathCon(1)),
                    )),
                    Box::new(Expr::Var(4)),
                ))),
            ),
        ]);
        let declaration =
            issue_ambient_context_declaration_token(&signature, &path, 15, vec![function_motive()])
                .expect("path declaration");
        assert!(matches!(
            issue_contextual_internality_token(
                &signature,
                &path,
                15,
                1,
                &declaration,
                &BTreeMap::from([(0, "earned".to_owned())]),
            ),
            Err(ContextualInternalityError::ChargedPathConstructor)
                | Err(ContextualInternalityError::IllTypedAmbientUse { .. })
        ));

        let declaration = issue_ambient_context_declaration_token(
            &signature,
            &candidate,
            15,
            vec![function_motive()],
        )
        .expect("declaration");
        let token = issue_contextual_internality_token(
            &signature,
            &candidate,
            15,
            1,
            &declaration,
            &BTreeMap::from([(0, "earned".to_owned())]),
        )
        .expect("control");
        let mut projection = token.projection().clone();
        projection.marginal_nu = 1;
        assert_eq!(
            replay_contextual_internality_projection(&signature, &candidate, 15, &projection),
            Err(ContextualInternalityError::ReplayMismatch)
        );
    }
}
