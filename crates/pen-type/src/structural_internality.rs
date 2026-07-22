//! Opaque, replayable constructor evidence for the closed structural
//! `Internal` fragment.
//!
//! This module deliberately exports one closure constructor: lambda
//! introduction.  A lambda earns `Internal` only when its body already has a
//! replayable structural derivation.  The frozen base derivations are the
//! ambient universe and sealed predecessor constants; nested lambdas recurse
//! through the same constructor.  Applications, variables, candidate fields,
//! ambient parameters, and every other schema constructor fail closed.

use crate::elaborate::{DerivationNode, SealedSignature, candidate_hash, elaborate_telescope};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const STRUCTURAL_LAMBDA_INTERNALITY_VERSION: &str = "kernel-structural-lambda-internality-v1";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuralInternalRule {
    AmbientUniverse,
    SealedPredecessorConstant { step: u32 },
    LambdaIntroduction,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralInternalNodeProjection {
    pub expression: Expr,
    pub scope_len: u32,
    pub derivation_rule: String,
    pub rule: StructuralInternalRule,
    pub child: Option<Box<StructuralInternalNodeProjection>>,
    pub child_already_internal: bool,
    pub constructor_replayed: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralLambdaClosureProjection {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub candidate_hash: String,
    pub elaboration_hash: String,
    pub clause_index: u16,
    pub outer_scope: u32,
    pub body_scope: u32,
    pub expression: Expr,
    pub normal_form: Expr,
    pub body_expression: Expr,
    pub root_rule: String,
    pub body_internal_evidence: StructuralInternalNodeProjection,
    pub body_already_internal: bool,
    pub lambda_constructor_replayed: bool,
    pub closed_candidate: bool,
    pub internal_closure_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

/// Opaque constructor proof.  The serializable projection can be persisted,
/// but only reissuance from the signature, candidate, and clause can create a
/// token.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StructuralLambdaClosureToken {
    projection: StructuralLambdaClosureProjection,
}

impl StructuralLambdaClosureToken {
    pub fn projection(&self) -> &StructuralLambdaClosureProjection {
        &self.projection
    }

    pub fn derivation_hash(&self) -> &str {
        &self.projection.derivation_hash
    }

    pub fn marginal_nu(&self) -> u32 {
        self.projection.marginal_nu
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum StructuralInternalityError {
    #[error("candidate failed elaboration: {0}")]
    Elaboration(String),
    #[error("clause index {clause_index} is out of range")]
    ClauseOutOfRange { clause_index: u16 },
    #[error("structural lambda closure is closed-only; candidate ambient arity is {ambient}")]
    GuardedCandidate { ambient: u32 },
    #[error("clause {clause_index} is not a lambda introduction")]
    NotLambdaIntroduction { clause_index: u16 },
    #[error("lambda elaboration did not replay the one-child lam-intro constructor")]
    LambdaDerivationShape,
    #[error("unsupported structural body {expression:?} under derivation rule {rule}")]
    UnsupportedStructuralBody { expression: Expr, rule: String },
    #[error("structural body derivation shape does not match its expression")]
    BodyDerivationShape,
    #[error("structural lambda closure projection replay mismatch")]
    ReplayMismatch,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(STRUCTURAL_LAMBDA_INTERNALITY_VERSION, domain, value))
        .expect("structural Internal evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn issue_structural_node(
    expression: &Expr,
    derivation: &DerivationNode,
    scope_len: u32,
    visible_library: u32,
) -> Result<StructuralInternalNodeProjection, StructuralInternalityError> {
    let (rule, child, child_already_internal, constructor_replayed) = match expression {
        Expr::Univ if derivation.rule == "univ-form" && derivation.children.is_empty() => {
            (StructuralInternalRule::AmbientUniverse, None, true, true)
        }
        Expr::Lib(step)
            if (1..=visible_library).contains(step)
                && derivation.rule == "library-constant"
                && derivation.children.is_empty() =>
        {
            (
                StructuralInternalRule::SealedPredecessorConstant { step: *step },
                None,
                true,
                true,
            )
        }
        Expr::Lam(body) if derivation.rule == "lam-intro" && derivation.children.len() == 1 => {
            let child = issue_structural_node(
                body,
                &derivation.children[0],
                scope_len + 1,
                visible_library,
            )?;
            let child_already_internal = child.constructor_replayed && child.marginal_nu == 0;
            (
                StructuralInternalRule::LambdaIntroduction,
                Some(Box::new(child)),
                child_already_internal,
                child_already_internal,
            )
        }
        Expr::Lam(_) => return Err(StructuralInternalityError::BodyDerivationShape),
        _ => {
            return Err(StructuralInternalityError::UnsupportedStructuralBody {
                expression: expression.clone(),
                rule: derivation.rule.clone(),
            });
        }
    };
    let marginal_nu = 0;
    let mut projection = StructuralInternalNodeProjection {
        expression: expression.clone(),
        scope_len,
        derivation_rule: derivation.rule.clone(),
        rule,
        child,
        child_already_internal,
        constructor_replayed,
        marginal_nu,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("structural-internal-node", &projection);
    Ok(projection)
}

pub fn issue_structural_lambda_closure_token(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
) -> Result<StructuralLambdaClosureToken, StructuralInternalityError> {
    let elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| StructuralInternalityError::Elaboration(error.to_string()))?;
    if elaboration.ambient_parameters != 0 {
        return Err(StructuralInternalityError::GuardedCandidate {
            ambient: elaboration.ambient_parameters,
        });
    }
    let index = usize::from(clause_index);
    let clause = candidate
        .clauses
        .get(index)
        .ok_or(StructuralInternalityError::ClauseOutOfRange { clause_index })?;
    let clause_elaboration = elaboration
        .clauses
        .get(index)
        .ok_or(StructuralInternalityError::ClauseOutOfRange { clause_index })?;
    let Expr::Lam(body) = &clause.expr else {
        return Err(StructuralInternalityError::NotLambdaIntroduction { clause_index });
    };
    if clause_elaboration.derivation.rule != "lam-intro"
        || clause_elaboration.derivation.children.len() != 1
    {
        return Err(StructuralInternalityError::LambdaDerivationShape);
    }
    let outer_scope = u32::from(clause_index);
    let body_scope = outer_scope + 1;
    let body_internal_evidence = issue_structural_node(
        body,
        &clause_elaboration.derivation.children[0],
        body_scope,
        visible_library,
    )?;
    let body_already_internal =
        body_internal_evidence.constructor_replayed && body_internal_evidence.marginal_nu == 0;
    let lambda_constructor_replayed = body_already_internal;
    let closed_candidate = true;
    let internal_closure_issued = body_already_internal && lambda_constructor_replayed;
    let marginal_nu = 0;
    let mut projection = StructuralLambdaClosureProjection {
        version: STRUCTURAL_LAMBDA_INTERNALITY_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        candidate_hash: candidate_hash(candidate),
        elaboration_hash: elaboration.derivation_hash,
        clause_index,
        outer_scope,
        body_scope,
        expression: clause.expr.clone(),
        normal_form: clause_elaboration.normal_form.clone(),
        body_expression: body.as_ref().clone(),
        root_rule: clause_elaboration.derivation.rule.clone(),
        body_internal_evidence,
        body_already_internal,
        lambda_constructor_replayed,
        closed_candidate,
        internal_closure_issued,
        marginal_nu,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("structural-lambda-closure-token", &projection);
    Ok(StructuralLambdaClosureToken { projection })
}

pub fn replay_structural_lambda_closure_token(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    token: &StructuralLambdaClosureToken,
) -> Result<(), StructuralInternalityError> {
    let reissued = issue_structural_lambda_closure_token(
        signature,
        candidate,
        visible_library,
        token.projection.clause_index,
    )?;
    if reissued == *token {
        Ok(())
    } else {
        Err(StructuralInternalityError::ReplayMismatch)
    }
}

pub fn replay_structural_lambda_closure_projection(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    projection: &StructuralLambdaClosureProjection,
) -> Result<(), StructuralInternalityError> {
    let token = issue_structural_lambda_closure_token(
        signature,
        candidate,
        visible_library,
        projection.clause_index,
    )?;
    if token.projection() == projection {
        Ok(())
    } else {
        Err(StructuralInternalityError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::{ClauseRec, ClauseRole};

    fn candidate(body: Expr) -> Telescope {
        Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(ClauseRole::Introduction, Expr::Lam(Box::new(body))),
        ])
    }

    #[test]
    fn constant_universe_lambda_replays_at_zero_credit() {
        let signature = SealedSignature::genesis_del_h15();
        let candidate = candidate(Expr::Univ);
        let token = issue_structural_lambda_closure_token(&signature, &candidate, 15, 1)
            .expect("structural lambda token");
        assert_eq!(token.marginal_nu(), 0);
        assert!(token.projection().body_already_internal);
        assert!(token.projection().lambda_constructor_replayed);
        assert!(matches!(
            token.projection().body_internal_evidence.rule,
            StructuralInternalRule::AmbientUniverse
        ));
        replay_structural_lambda_closure_token(&signature, &candidate, 15, &token)
            .expect("token replay");
        replay_structural_lambda_closure_projection(&signature, &candidate, 15, token.projection())
            .expect("projection replay");
    }

    #[test]
    fn nested_lambda_and_predecessor_constant_recurse() {
        let signature = SealedSignature::genesis_del_h15();
        for body in [Expr::Lib(1), Expr::Lam(Box::new(Expr::Univ))] {
            let candidate = candidate(body);
            let token = issue_structural_lambda_closure_token(&signature, &candidate, 15, 1)
                .expect("recursive structural token");
            assert!(token.projection().internal_closure_issued);
            assert_eq!(token.projection().marginal_nu, 0);
        }
    }

    #[test]
    fn applications_and_guarded_candidates_fail_closed() {
        let signature = SealedSignature::genesis_del_h15();
        let application = candidate(Expr::App(Box::new(Expr::Lib(1)), Box::new(Expr::Lib(2))));
        assert!(matches!(
            issue_structural_lambda_closure_token(&signature, &application, 15, 1),
            Err(StructuralInternalityError::UnsupportedStructuralBody { .. })
        ));
        let guarded = candidate(Expr::Var(3));
        assert!(matches!(
            issue_structural_lambda_closure_token(&signature, &guarded, 15, 1),
            Err(StructuralInternalityError::GuardedCandidate { ambient: 1 })
        ));
    }

    #[test]
    fn persisted_projection_mutation_fails_replay() {
        let signature = SealedSignature::genesis_del_h15();
        let candidate = candidate(Expr::Univ);
        let token = issue_structural_lambda_closure_token(&signature, &candidate, 15, 1)
            .expect("structural lambda token");
        let mut projection = token.projection().clone();
        projection.marginal_nu = 1;
        assert_eq!(
            replay_structural_lambda_closure_projection(&signature, &candidate, 15, &projection,),
            Err(StructuralInternalityError::ReplayMismatch)
        );
    }
}
