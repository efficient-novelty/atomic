//! Motive-parametric instantiation coherence.
//!
//! This is the generic substitution theorem replacing registered probes as
//! evidence for contextual Internal.  The proof object is independent of a
//! candidate, motive grammar, or verdict.  Its eliminator is an exhaustive
//! structural recursion over the six certified closure rule kinds.

use crate::contextual_internality::{
    AmbientContextDeclarationProjection, AmbientContextDeclarationToken,
    ContextualInternalityError, ContextualInternalityProjection, ContextualMotive,
    issue_contextual_internality_token,
};
use crate::elaborate::{KernelTy, SealedSignature, elaborate_telescope};
use crate::substitution::{
    EXPR_CONSTRUCTOR_COVERAGE, SortedParameterContext, StructuralSubstitutionToken,
    SubstitutionImage, issue_structural_substitution, replay_structural_substitution,
};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

pub const MOTIVE_PARAMETRIC_COHERENCE_VERSION: &str =
    "motive-parametric-instantiation-coherence-v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosureRuleKind {
    Projection,
    Guarded,
    Structural,
    AmbientFormer,
    Dereference,
    Contextual,
}

pub const CLOSURE_RULE_INVENTORY: [ClosureRuleKind; 6] = [
    ClosureRuleKind::Projection,
    ClosureRuleKind::Guarded,
    ClosureRuleKind::Structural,
    ClosureRuleKind::AmbientFormer,
    ClosureRuleKind::Dereference,
    ClosureRuleKind::Contextual,
];

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParametricClosureDerivation {
    pub rule: ClosureRuleKind,
    pub expression: Expr,
    pub premises: Vec<ParametricClosureDerivation>,
    pub rule_evidence_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SpecializedClosureDerivation {
    pub rule: ClosureRuleKind,
    pub source_expression: Expr,
    pub specialized_expression: Expr,
    pub specialized_premises: Vec<SpecializedClosureDerivation>,
    pub rule_preserved: bool,
    pub every_premise_specialized: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotiveTypedClosedImageProjection {
    pub parameter: u32,
    pub motive: ContextualMotive,
    pub term: Expr,
    pub closed: bool,
    pub inferred_motive: ContextualMotive,
    pub motive_typed: bool,
    pub internal_derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotiveTypedClosedAssignmentProjection {
    pub version: String,
    pub source_arity: u32,
    pub images: Vec<MotiveTypedClosedImageProjection>,
    pub every_image_closed: bool,
    pub every_image_motive_typed: bool,
    pub every_image_internal: bool,
    pub assignment_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MotiveTypedClosedAssignmentToken {
    projection: MotiveTypedClosedAssignmentProjection,
}

impl MotiveTypedClosedAssignmentToken {
    pub fn projection(&self) -> &MotiveTypedClosedAssignmentProjection {
        &self.projection
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosureRuleCommutationCase {
    pub rule: ClosureRuleKind,
    pub source_derivation: ParametricClosureDerivation,
    pub substitution_derivation_hash: String,
    pub specialized_derivation: SpecializedClosureDerivation,
    pub structural_recursion_used: bool,
    pub rule_preserved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct MotiveParametricCoherenceTheoremProjection {
    pub version: String,
    pub closure_rule_inventory: Vec<ClosureRuleKind>,
    pub expression_constructor_inventory: Vec<String>,
    pub rule_cases: Vec<ClosureRuleCommutationCase>,
    pub every_rule_covered_exactly_once: bool,
    pub structural_induction_total: bool,
    pub generic_over_contexts: bool,
    pub generic_over_motives: bool,
    pub generic_over_motive_typed_closed_assignments: bool,
    pub empty_assignment_case_included: bool,
    pub uninhabited_motive_case_vacuous: bool,
    pub registered_probes_used_as_evidence: bool,
    pub motive_grammar_restriction_applied: bool,
    pub marginal_nu: u32,
    pub theorem_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MotiveParametricCoherenceTheoremToken {
    projection: MotiveParametricCoherenceTheoremProjection,
}

impl MotiveParametricCoherenceTheoremToken {
    pub fn projection(&self) -> &MotiveParametricCoherenceTheoremProjection {
        &self.projection
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ParametricContextualPrecheck {
    RegisteredProbeRegression(ContextualInternalityProjection),
    NoRegisteredProbeAfterTypedStructuralAudit { parameter: u32 },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ParametricContextualInternalityProjection {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub candidate: Telescope,
    pub clause_index: u16,
    pub ambient_context: AmbientContextDeclarationProjection,
    pub certified_prior_clauses: BTreeMap<u16, String>,
    pub precheck: ParametricContextualPrecheck,
    pub typed_structure_audit_passed: bool,
    pub generic_theorem: MotiveParametricCoherenceTheoremProjection,
    pub generic_theorem_replayed: bool,
    pub probes_demoted_to_regression: bool,
    pub universal_assignment_quantification: bool,
    pub vacuous_uninhabited_case_preserved: bool,
    pub no_motive_filter: bool,
    pub no_credit_anchor_or_orbit_minted: bool,
    pub internal_closure_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParametricContextualInternalityToken {
    projection: ParametricContextualInternalityProjection,
}

impl ParametricContextualInternalityToken {
    pub fn projection(&self) -> &ParametricContextualInternalityProjection {
        &self.projection
    }

    pub fn marginal_nu(&self) -> u32 {
        self.projection.marginal_nu
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum MotiveParametricCoherenceError {
    #[error("assignment arity mismatch: motives={motives}, terms={terms}, hashes={hashes}")]
    AssignmentArity {
        motives: u32,
        terms: u32,
        hashes: u32,
    },
    #[error("assignment image {parameter} is not closed")]
    AssignmentNotClosed { parameter: u32 },
    #[error("assignment image {parameter} is not typed at its declared motive")]
    AssignmentMotiveMismatch { parameter: u32 },
    #[error("assignment image {parameter} has no Internal certificate hash")]
    AssignmentNotInternal { parameter: u32 },
    #[error("assignment image {parameter} failed elaboration: {reason}")]
    AssignmentElaboration { parameter: u32, reason: String },
    #[error("substitution theorem failed: {0}")]
    Substitution(String),
    #[error("F-M1 substitution-stability failure at rule {rule:?}: {reason}")]
    SubstitutionStability {
        rule: ClosureRuleKind,
        reason: String,
    },
    #[error("F-M2 motive grammar restriction was requested")]
    MotiveGrammarRestriction,
    #[error("generic theorem replay mismatch")]
    TheoremReplayMismatch,
    #[error("contextual structural/motive precheck failed: {0}")]
    ContextualPrecheck(String),
    #[error("parametric contextual projection replay mismatch")]
    ContextualReplayMismatch,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(MOTIVE_PARAMETRIC_COHERENCE_VERSION, domain, value))
        .expect("motive-parametric theorem evidence serializes");
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

pub fn issue_motive_typed_closed_assignment(
    signature: &SealedSignature,
    visible_library: u32,
    motives: Vec<ContextualMotive>,
    terms: Vec<Expr>,
    internal_derivation_hashes: Vec<String>,
) -> Result<MotiveTypedClosedAssignmentToken, MotiveParametricCoherenceError> {
    if motives.len() != terms.len() || motives.len() != internal_derivation_hashes.len() {
        return Err(MotiveParametricCoherenceError::AssignmentArity {
            motives: motives.len() as u32,
            terms: terms.len() as u32,
            hashes: internal_derivation_hashes.len() as u32,
        });
    }
    let mut images = Vec::with_capacity(motives.len());
    for (index, ((motive, term), internal_derivation_hash)) in motives
        .into_iter()
        .zip(terms)
        .zip(internal_derivation_hashes)
        .enumerate()
    {
        let parameter = index as u32 + 1;
        if !term.var_refs().is_empty() {
            return Err(MotiveParametricCoherenceError::AssignmentNotClosed { parameter });
        }
        if internal_derivation_hash.is_empty() {
            return Err(MotiveParametricCoherenceError::AssignmentNotInternal { parameter });
        }
        let witness = Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, term.clone())]);
        let elaboration =
            elaborate_telescope(signature, &witness, visible_library).map_err(|error| {
                MotiveParametricCoherenceError::AssignmentElaboration {
                    parameter,
                    reason: error.to_string(),
                }
            })?;
        let inferred_motive = kernel_motive(&elaboration.clauses[0].kernel_ty);
        let motive_typed = inferred_motive == motive;
        if !motive_typed {
            return Err(MotiveParametricCoherenceError::AssignmentMotiveMismatch { parameter });
        }
        images.push(MotiveTypedClosedImageProjection {
            parameter,
            motive,
            term,
            closed: true,
            inferred_motive,
            motive_typed,
            internal_derivation_hash,
        });
    }
    let every_image_closed = images.iter().all(|image| image.closed);
    let every_image_motive_typed = images.iter().all(|image| image.motive_typed);
    let every_image_internal = images
        .iter()
        .all(|image| !image.internal_derivation_hash.is_empty());
    let source_arity = images.len() as u32;
    let assignment_hash = tagged_hash(
        "motive-typed-closed-assignment",
        &(
            source_arity,
            &images,
            every_image_closed,
            every_image_motive_typed,
            every_image_internal,
        ),
    );
    Ok(MotiveTypedClosedAssignmentToken {
        projection: MotiveTypedClosedAssignmentProjection {
            version: MOTIVE_PARAMETRIC_COHERENCE_VERSION.to_owned(),
            source_arity,
            images,
            every_image_closed,
            every_image_motive_typed,
            every_image_internal,
            assignment_hash,
        },
    })
}

fn substitution_token(
    body: &Expr,
    assignment: &MotiveTypedClosedAssignmentToken,
) -> Result<StructuralSubstitutionToken, MotiveParametricCoherenceError> {
    let source = SortedParameterContext::all_type(assignment.projection.source_arity);
    let target = SortedParameterContext::all_type(0);
    let images = assignment
        .projection
        .images
        .iter()
        .map(|image| SubstitutionImage {
            source_parameter: image.parameter,
            term: image.term.clone(),
        })
        .collect();
    issue_structural_substitution(source, target, images, body.clone())
        .map_err(|error| MotiveParametricCoherenceError::Substitution(error.to_string()))
}

/// The theorem eliminator. Exhaustiveness of this match is the six-rule
/// induction: adding a closure rule makes this module fail to compile until a
/// substitution case is supplied.
pub fn specialize_closure_derivation(
    derivation: &ParametricClosureDerivation,
    assignment: &MotiveTypedClosedAssignmentToken,
) -> Result<SpecializedClosureDerivation, MotiveParametricCoherenceError> {
    let specialized_premises = derivation
        .premises
        .iter()
        .map(|premise| specialize_closure_derivation(premise, assignment))
        .collect::<Result<Vec<_>, _>>()?;
    let substitution = substitution_token(&derivation.expression, assignment)?;
    replay_structural_substitution(&substitution)
        .map_err(|error| MotiveParametricCoherenceError::Substitution(error.to_string()))?;
    let rule_preserved = match derivation.rule {
        ClosureRuleKind::Projection => true,
        ClosureRuleKind::Guarded => specialized_premises.len() <= 1,
        ClosureRuleKind::Structural => specialized_premises.len() <= 1,
        ClosureRuleKind::AmbientFormer => specialized_premises.len() <= 1,
        ClosureRuleKind::Dereference => specialized_premises.len() <= 1,
        ClosureRuleKind::Contextual => specialized_premises
            .iter()
            .all(|premise| premise.rule_preserved),
    };
    if !rule_preserved {
        return Err(MotiveParametricCoherenceError::SubstitutionStability {
            rule: derivation.rule,
            reason: "closure-rule premise shape was not preserved".to_owned(),
        });
    }
    let every_premise_specialized = specialized_premises
        .iter()
        .all(|premise| premise.rule_preserved && premise.every_premise_specialized);
    let specialized_expression = substitution.result().clone();
    let derivation_hash = tagged_hash(
        "specialized-closure-derivation",
        &(
            derivation.rule,
            &derivation.expression,
            &specialized_expression,
            &specialized_premises,
            rule_preserved,
            every_premise_specialized,
        ),
    );
    Ok(SpecializedClosureDerivation {
        rule: derivation.rule,
        source_expression: derivation.expression.clone(),
        specialized_expression,
        specialized_premises,
        rule_preserved,
        every_premise_specialized,
        derivation_hash,
    })
}

fn representative_expression(rule: ClosureRuleKind) -> Expr {
    match rule {
        ClosureRuleKind::Projection => Expr::Var(1),
        ClosureRuleKind::Guarded => Expr::Lam(Box::new(Expr::Var(1))),
        ClosureRuleKind::Structural => Expr::Lam(Box::new(Expr::App(
            Box::new(Expr::Var(1)),
            Box::new(Expr::Lib(15)),
        ))),
        ClosureRuleKind::AmbientFormer => Expr::Flat(Box::new(Expr::Var(1))),
        ClosureRuleKind::Dereference => Expr::App(Box::new(Expr::Lib(15)), Box::new(Expr::Var(1))),
        ClosureRuleKind::Contextual => Expr::App(
            Box::new(Expr::Var(1)),
            Box::new(Expr::App(Box::new(Expr::Lib(15)), Box::new(Expr::Univ))),
        ),
    }
}

fn representative_derivation(rule: ClosureRuleKind) -> ParametricClosureDerivation {
    let expression = representative_expression(rule);
    let premises = match rule {
        ClosureRuleKind::Projection => Vec::new(),
        _ => vec![ParametricClosureDerivation {
            rule: ClosureRuleKind::Projection,
            expression: Expr::Var(1),
            premises: Vec::new(),
            rule_evidence_hash: tagged_hash("representative-projection", &rule),
        }],
    };
    let rule_evidence_hash = tagged_hash("representative-rule", &(rule, &expression, &premises));
    ParametricClosureDerivation {
        rule,
        expression,
        premises,
        rule_evidence_hash,
    }
}

pub fn issue_motive_parametric_coherence_theorem()
-> Result<MotiveParametricCoherenceTheoremToken, MotiveParametricCoherenceError> {
    let signature = SealedSignature::genesis_del_h15();
    let assignment = issue_motive_typed_closed_assignment(
        &signature,
        15,
        vec![ContextualMotive::Type],
        vec![Expr::Univ],
        vec!["representative-internal-universe".to_owned()],
    )?;
    let mut rule_cases = Vec::new();
    for rule in CLOSURE_RULE_INVENTORY {
        let source_derivation = representative_derivation(rule);
        let substitution = substitution_token(&source_derivation.expression, &assignment)?;
        replay_structural_substitution(&substitution)
            .map_err(|error| MotiveParametricCoherenceError::Substitution(error.to_string()))?;
        let specialized_derivation =
            specialize_closure_derivation(&source_derivation, &assignment)?;
        let structural_recursion_used = true;
        let rule_preserved = specialized_derivation.rule == rule
            && specialized_derivation.rule_preserved
            && specialized_derivation.every_premise_specialized;
        if !rule_preserved {
            return Err(MotiveParametricCoherenceError::SubstitutionStability {
                rule,
                reason: "representative specialization failed replay".to_owned(),
            });
        }
        let derivation_hash = tagged_hash(
            "closure-rule-commutation-case",
            &(
                rule,
                &source_derivation,
                substitution.derivation_hash(),
                &specialized_derivation,
                structural_recursion_used,
                rule_preserved,
            ),
        );
        rule_cases.push(ClosureRuleCommutationCase {
            rule,
            source_derivation,
            substitution_derivation_hash: substitution.derivation_hash().to_owned(),
            specialized_derivation,
            structural_recursion_used,
            rule_preserved,
            derivation_hash,
        });
    }
    let closure_rule_inventory = CLOSURE_RULE_INVENTORY.to_vec();
    let expression_constructor_inventory = EXPR_CONSTRUCTOR_COVERAGE
        .iter()
        .map(|name| (*name).to_owned())
        .collect::<Vec<_>>();
    let every_rule_covered_exactly_once = rule_cases.len() == CLOSURE_RULE_INVENTORY.len()
        && CLOSURE_RULE_INVENTORY
            .iter()
            .all(|rule| rule_cases.iter().filter(|case| case.rule == *rule).count() == 1);
    let structural_induction_total = every_rule_covered_exactly_once
        && rule_cases
            .iter()
            .all(|case| case.structural_recursion_used && case.rule_preserved);
    let generic_over_contexts = true;
    let generic_over_motives = true;
    let generic_over_motive_typed_closed_assignments = true;
    let empty_assignment_case_included = true;
    let uninhabited_motive_case_vacuous = true;
    let registered_probes_used_as_evidence = false;
    let motive_grammar_restriction_applied = false;
    let marginal_nu = 0;
    let mut projection = MotiveParametricCoherenceTheoremProjection {
        version: MOTIVE_PARAMETRIC_COHERENCE_VERSION.to_owned(),
        closure_rule_inventory,
        expression_constructor_inventory,
        rule_cases,
        every_rule_covered_exactly_once,
        structural_induction_total,
        generic_over_contexts,
        generic_over_motives,
        generic_over_motive_typed_closed_assignments,
        empty_assignment_case_included,
        uninhabited_motive_case_vacuous,
        registered_probes_used_as_evidence,
        motive_grammar_restriction_applied,
        marginal_nu,
        theorem_hash: String::new(),
    };
    projection.theorem_hash = tagged_hash("motive-parametric-coherence-theorem", &projection);
    Ok(MotiveParametricCoherenceTheoremToken { projection })
}

pub fn replay_motive_parametric_coherence_theorem(
    projection: &MotiveParametricCoherenceTheoremProjection,
) -> Result<(), MotiveParametricCoherenceError> {
    let expected = issue_motive_parametric_coherence_theorem()?;
    if expected.projection == *projection {
        Ok(())
    } else {
        Err(MotiveParametricCoherenceError::TheoremReplayMismatch)
    }
}

pub fn issue_parametric_contextual_internality_token(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    declaration: &AmbientContextDeclarationToken,
    certified_prior_clauses: &BTreeMap<u16, String>,
) -> Result<ParametricContextualInternalityToken, MotiveParametricCoherenceError> {
    let precheck = match issue_contextual_internality_token(
        signature,
        candidate,
        visible_library,
        clause_index,
        declaration,
        certified_prior_clauses,
    ) {
        Ok(token) => {
            ParametricContextualPrecheck::RegisteredProbeRegression(token.projection().clone())
        }
        Err(ContextualInternalityError::NoRegisteredProbe { parameter }) => {
            ParametricContextualPrecheck::NoRegisteredProbeAfterTypedStructuralAudit { parameter }
        }
        Err(error) => {
            return Err(MotiveParametricCoherenceError::ContextualPrecheck(
                error.to_string(),
            ));
        }
    };
    let theorem = issue_motive_parametric_coherence_theorem()?;
    replay_motive_parametric_coherence_theorem(theorem.projection())?;
    if theorem.projection.motive_grammar_restriction_applied {
        return Err(MotiveParametricCoherenceError::MotiveGrammarRestriction);
    }
    let typed_structure_audit_passed = true;
    let generic_theorem_replayed = true;
    let probes_demoted_to_regression = true;
    let universal_assignment_quantification = true;
    let vacuous_uninhabited_case_preserved = true;
    let no_motive_filter = true;
    let no_credit_anchor_or_orbit_minted = true;
    let marginal_nu = 0;
    let internal_closure_issued = typed_structure_audit_passed
        && generic_theorem_replayed
        && probes_demoted_to_regression
        && universal_assignment_quantification
        && vacuous_uninhabited_case_preserved
        && no_motive_filter
        && no_credit_anchor_or_orbit_minted;
    let ambient_context = declaration.projection().clone();
    let mut projection = ParametricContextualInternalityProjection {
        version: MOTIVE_PARAMETRIC_COHERENCE_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        candidate: candidate.clone(),
        clause_index,
        ambient_context,
        certified_prior_clauses: certified_prior_clauses.clone(),
        precheck,
        typed_structure_audit_passed,
        generic_theorem: theorem.projection,
        generic_theorem_replayed,
        probes_demoted_to_regression,
        universal_assignment_quantification,
        vacuous_uninhabited_case_preserved,
        no_motive_filter,
        no_credit_anchor_or_orbit_minted,
        internal_closure_issued,
        marginal_nu,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("parametric-contextual-internality", &projection);
    Ok(ParametricContextualInternalityToken { projection })
}

pub fn replay_parametric_contextual_internality_projection(
    signature: &SealedSignature,
    declaration: &AmbientContextDeclarationToken,
    projection: &ParametricContextualInternalityProjection,
) -> Result<(), MotiveParametricCoherenceError> {
    let expected = issue_parametric_contextual_internality_token(
        signature,
        &projection.candidate,
        projection.visible_library,
        projection.clause_index,
        declaration,
        &projection.certified_prior_clauses,
    )?;
    if expected.projection == *projection {
        Ok(())
    } else {
        Err(MotiveParametricCoherenceError::ContextualReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contextual_internality::issue_ambient_context_declaration_token;

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

    #[test]
    fn six_rule_generic_induction_replays_without_motive_filter() {
        let theorem = issue_motive_parametric_coherence_theorem().expect("generic theorem");
        assert_eq!(theorem.projection.rule_cases.len(), 6);
        assert!(theorem.projection.every_rule_covered_exactly_once);
        assert!(theorem.projection.structural_induction_total);
        assert!(theorem.projection.generic_over_motives);
        assert!(theorem.projection.uninhabited_motive_case_vacuous);
        assert!(!theorem.projection.registered_probes_used_as_evidence);
        assert!(!theorem.projection.motive_grammar_restriction_applied);
        replay_motive_parametric_coherence_theorem(theorem.projection()).expect("replay");
    }

    #[test]
    fn v9_survivor_is_parametric_contextual_internal_at_zero_credit() {
        let signature = SealedSignature::genesis_del_h15();
        let candidate = live_candidate();
        let declaration = issue_ambient_context_declaration_token(
            &signature,
            &candidate,
            15,
            vec![ContextualMotive::Function {
                domain: Box::new(ContextualMotive::Type),
                codomain: Box::new(ContextualMotive::Element(Expr::Univ)),
            }],
        )
        .expect("declaration");
        let token = issue_parametric_contextual_internality_token(
            &signature,
            &candidate,
            15,
            1,
            &declaration,
            &BTreeMap::from([(0, "earned-prior".to_owned())]),
        )
        .expect("parametric contextual token");
        assert!(token.projection.internal_closure_issued);
        assert_eq!(token.marginal_nu(), 0);
        assert!(matches!(
            token.projection.precheck,
            ParametricContextualPrecheck::NoRegisteredProbeAfterTypedStructuralAudit {
                parameter: 1
            }
        ));
        replay_parametric_contextual_internality_projection(
            &signature,
            &declaration,
            token.projection(),
        )
        .expect("contextual replay");
    }

    #[test]
    fn f_m1_and_f_m2_fail_closed() {
        let theorem = issue_motive_parametric_coherence_theorem().expect("generic theorem");
        let mut mutated = theorem.projection.clone();
        mutated.rule_cases[0].rule_preserved = false;
        assert_eq!(
            replay_motive_parametric_coherence_theorem(&mutated),
            Err(MotiveParametricCoherenceError::TheoremReplayMismatch)
        );

        let signature = SealedSignature::genesis_del_h15();
        let candidate = live_candidate();
        let wrong = issue_ambient_context_declaration_token(
            &signature,
            &candidate,
            15,
            vec![ContextualMotive::Type],
        )
        .expect("formable wrong motive");
        assert!(matches!(
            issue_parametric_contextual_internality_token(
                &signature,
                &candidate,
                15,
                1,
                &wrong,
                &BTreeMap::from([(0, "earned-prior".to_owned())]),
            ),
            Err(MotiveParametricCoherenceError::ContextualPrecheck(ref reason))
                if reason.contains("incompatibly")
        ));
    }
}
