//! Replayable candidate-clause weakening/erasure evidence for the frozen
//! absolute-level kernel.
//!
//! Weakening inserts one unused ambient parameter at level 1 and shifts every
//! existing level by one.  Erasure is defined only when level 1 is absent and
//! removes that ambient parameter.  The token binds both candidates to B15,
//! replays their elaborations, and proves both composites by the frozen
//! univalent equality procedure.

use crate::elaborate::{SealedSignature, candidate_hash, elaborate_telescope};
use crate::equality::{EqualityWitness, univalent_equality};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const GUARDED_CLAUSE_INTERNALITY_VERSION: &str = "kernel-guarded-clause-weakening-erasure-v1";
pub const GUARDED_INSERTED_AMBIENT_LEVEL: u32 = 1;
const EQUALITY_FUEL: u32 = 64;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EqualityProjection {
    pub scope_len: u32,
    pub left_normal_form: Expr,
    pub right_normal_form: Expr,
    pub left_steps: u32,
    pub right_steps: u32,
    pub equal: bool,
}

impl From<&EqualityWitness> for EqualityProjection {
    fn from(witness: &EqualityWitness) -> Self {
        Self {
            scope_len: witness.scope_len,
            left_normal_form: witness.left_normal_form.clone(),
            right_normal_form: witness.right_normal_form.clone(),
            left_steps: witness.left_steps,
            right_steps: witness.right_steps,
            equal: witness.equal,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardedClauseWeakeningErasureProjection {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub base_candidate_hash: String,
    pub guarded_candidate_hash: String,
    pub base_elaboration_hash: String,
    pub guarded_elaboration_hash: String,
    pub clause_index: u16,
    pub base_outer_scope: u32,
    pub guarded_outer_scope: u32,
    pub base_expression: Expr,
    pub guarded_expression: Expr,
    pub weakened_expression: Expr,
    pub erased_expression: Expr,
    pub erase_after_weaken_expression: Expr,
    pub weaken_after_erase_expression: Expr,
    pub weakening_matches_guarded: EqualityProjection,
    pub erasure_matches_base: EqualityProjection,
    pub erasure_after_weakening: EqualityProjection,
    pub weakening_after_erasure: EqualityProjection,
    pub inserted_ambient_is_unused: bool,
    pub both_inverse_laws_checked: bool,
    pub derivation_hash: String,
}

/// Opaque proof object.  Callers may inspect its replay projection but cannot
/// construct a token or mutate its proof fields.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GuardedClauseWeakeningErasureToken {
    projection: GuardedClauseWeakeningErasureProjection,
}

impl GuardedClauseWeakeningErasureToken {
    pub fn projection(&self) -> &GuardedClauseWeakeningErasureProjection {
        &self.projection
    }

    pub fn derivation_hash(&self) -> &str {
        &self.projection.derivation_hash
    }

    pub fn both_inverse_laws_checked(&self) -> bool {
        self.projection.both_inverse_laws_checked
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq, Serialize)]
pub enum GuardedInternalityError {
    #[error("variable level 0 is invalid in the frozen absolute-level kernel")]
    InvalidVariableLevelZero,
    #[error("base candidate failed elaboration: {0}")]
    BaseElaboration(String),
    #[error("guarded candidate failed elaboration: {0}")]
    GuardedElaboration(String),
    #[error(
        "expected base ambient arity 0 and guarded ambient arity 1, found {base} and {guarded}"
    )]
    AmbientArity { base: u32, guarded: u32 },
    #[error("base and guarded candidates have different telescope lengths")]
    TelescopeLength,
    #[error("clause index {clause_index} is out of range")]
    ClauseOutOfRange { clause_index: u16 },
    #[error("clause roles differ across weakening")]
    ClauseRoleMismatch,
    #[error("guarded expression uses inserted ambient level 1 and cannot be erased")]
    InsertedAmbientUsed,
    #[error("weakening did not produce the guarded expression")]
    WeakeningMismatch,
    #[error("erasure did not produce the base expression")]
    ErasureMismatch,
    #[error("weakening/erasure inverse law failed")]
    InverseLawMismatch,
    #[error("normalization/equality failed: {0}")]
    Equality(String),
    #[error("guarded clause token replay mismatch")]
    ReplayMismatch,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(GUARDED_CLAUSE_INTERNALITY_VERSION, domain, value))
        .expect("guarded clause evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

/// Insert a new unused ambient parameter at absolute level 1.
pub fn weaken_by_one_ambient(expr: &Expr) -> Expr {
    match expr {
        Expr::Var(level) => Expr::Var(level + 1),
        Expr::Univ => Expr::Univ,
        Expr::Lib(step) => Expr::Lib(*step),
        Expr::PathCon(dimension) => Expr::PathCon(*dimension),
        Expr::App(left, right) => Expr::App(
            Box::new(weaken_by_one_ambient(left)),
            Box::new(weaken_by_one_ambient(right)),
        ),
        Expr::Pi(domain, codomain) => Expr::Pi(
            Box::new(weaken_by_one_ambient(domain)),
            Box::new(weaken_by_one_ambient(codomain)),
        ),
        Expr::Sigma(domain, codomain) => Expr::Sigma(
            Box::new(weaken_by_one_ambient(domain)),
            Box::new(weaken_by_one_ambient(codomain)),
        ),
        Expr::Lam(body) => Expr::Lam(Box::new(weaken_by_one_ambient(body))),
        Expr::Id(ty, left, right) => Expr::Id(
            Box::new(weaken_by_one_ambient(ty)),
            Box::new(weaken_by_one_ambient(left)),
            Box::new(weaken_by_one_ambient(right)),
        ),
        Expr::Refl(body) => Expr::Refl(Box::new(weaken_by_one_ambient(body))),
        Expr::Susp(body) => Expr::Susp(Box::new(weaken_by_one_ambient(body))),
        Expr::Trunc(body) => Expr::Trunc(Box::new(weaken_by_one_ambient(body))),
        Expr::Flat(body) => Expr::Flat(Box::new(weaken_by_one_ambient(body))),
        Expr::Sharp(body) => Expr::Sharp(Box::new(weaken_by_one_ambient(body))),
        Expr::Disc(body) => Expr::Disc(Box::new(weaken_by_one_ambient(body))),
        Expr::Shape(body) => Expr::Shape(Box::new(weaken_by_one_ambient(body))),
        Expr::Next(body) => Expr::Next(Box::new(weaken_by_one_ambient(body))),
        Expr::Eventually(body) => Expr::Eventually(Box::new(weaken_by_one_ambient(body))),
        Expr::Bang(body) => Expr::Bang(Box::new(weaken_by_one_ambient(body))),
        Expr::WhyNot(body) => Expr::WhyNot(Box::new(weaken_by_one_ambient(body))),
    }
}

/// Remove the ambient parameter at absolute level 1.  This is partial: a
/// term that actually mentions the parameter is not an erasure image.
pub fn erase_one_unused_ambient(expr: &Expr) -> Result<Expr, GuardedInternalityError> {
    match expr {
        Expr::Var(0) => Err(GuardedInternalityError::InvalidVariableLevelZero),
        Expr::Var(1) => Err(GuardedInternalityError::InsertedAmbientUsed),
        Expr::Var(level) => Ok(Expr::Var(level - 1)),
        Expr::Univ => Ok(Expr::Univ),
        Expr::Lib(step) => Ok(Expr::Lib(*step)),
        Expr::PathCon(dimension) => Ok(Expr::PathCon(*dimension)),
        Expr::App(left, right) => Ok(Expr::App(
            Box::new(erase_one_unused_ambient(left)?),
            Box::new(erase_one_unused_ambient(right)?),
        )),
        Expr::Pi(domain, codomain) => Ok(Expr::Pi(
            Box::new(erase_one_unused_ambient(domain)?),
            Box::new(erase_one_unused_ambient(codomain)?),
        )),
        Expr::Sigma(domain, codomain) => Ok(Expr::Sigma(
            Box::new(erase_one_unused_ambient(domain)?),
            Box::new(erase_one_unused_ambient(codomain)?),
        )),
        Expr::Lam(body) => Ok(Expr::Lam(Box::new(erase_one_unused_ambient(body)?))),
        Expr::Id(ty, left, right) => Ok(Expr::Id(
            Box::new(erase_one_unused_ambient(ty)?),
            Box::new(erase_one_unused_ambient(left)?),
            Box::new(erase_one_unused_ambient(right)?),
        )),
        Expr::Refl(body) => Ok(Expr::Refl(Box::new(erase_one_unused_ambient(body)?))),
        Expr::Susp(body) => Ok(Expr::Susp(Box::new(erase_one_unused_ambient(body)?))),
        Expr::Trunc(body) => Ok(Expr::Trunc(Box::new(erase_one_unused_ambient(body)?))),
        Expr::Flat(body) => Ok(Expr::Flat(Box::new(erase_one_unused_ambient(body)?))),
        Expr::Sharp(body) => Ok(Expr::Sharp(Box::new(erase_one_unused_ambient(body)?))),
        Expr::Disc(body) => Ok(Expr::Disc(Box::new(erase_one_unused_ambient(body)?))),
        Expr::Shape(body) => Ok(Expr::Shape(Box::new(erase_one_unused_ambient(body)?))),
        Expr::Next(body) => Ok(Expr::Next(Box::new(erase_one_unused_ambient(body)?))),
        Expr::Eventually(body) => Ok(Expr::Eventually(Box::new(erase_one_unused_ambient(body)?))),
        Expr::Bang(body) => Ok(Expr::Bang(Box::new(erase_one_unused_ambient(body)?))),
        Expr::WhyNot(body) => Ok(Expr::WhyNot(Box::new(erase_one_unused_ambient(body)?))),
    }
}

pub fn issue_guarded_clause_weakening_erasure_token(
    signature: &SealedSignature,
    base_candidate: &Telescope,
    guarded_candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
) -> Result<GuardedClauseWeakeningErasureToken, GuardedInternalityError> {
    if base_candidate.kappa() != guarded_candidate.kappa() {
        return Err(GuardedInternalityError::TelescopeLength);
    }
    let base_elaboration = elaborate_telescope(signature, base_candidate, visible_library)
        .map_err(|error| GuardedInternalityError::BaseElaboration(error.to_string()))?;
    let guarded_elaboration = elaborate_telescope(signature, guarded_candidate, visible_library)
        .map_err(|error| GuardedInternalityError::GuardedElaboration(error.to_string()))?;
    if base_elaboration.ambient_parameters != 0 || guarded_elaboration.ambient_parameters != 1 {
        return Err(GuardedInternalityError::AmbientArity {
            base: base_elaboration.ambient_parameters,
            guarded: guarded_elaboration.ambient_parameters,
        });
    }
    let index = usize::from(clause_index);
    let base_clause = base_candidate
        .clauses
        .get(index)
        .ok_or(GuardedInternalityError::ClauseOutOfRange { clause_index })?;
    let guarded_clause = guarded_candidate
        .clauses
        .get(index)
        .ok_or(GuardedInternalityError::ClauseOutOfRange { clause_index })?;
    if base_clause.role != guarded_clause.role {
        return Err(GuardedInternalityError::ClauseRoleMismatch);
    }

    let base_expression = base_clause.expr.clone();
    let guarded_expression = guarded_clause.expr.clone();
    let weakened_expression = weaken_by_one_ambient(&base_expression);
    let erased_expression = erase_one_unused_ambient(&guarded_expression)?;
    let inserted_ambient_is_unused = true;
    let base_outer_scope = u32::from(clause_index);
    let guarded_outer_scope = base_outer_scope + 1;
    let weakening_matches_guarded = univalent_equality(
        &weakened_expression,
        &guarded_expression,
        guarded_outer_scope,
        EQUALITY_FUEL,
    )
    .map_err(|error| GuardedInternalityError::Equality(error.to_string()))?;
    if !weakening_matches_guarded.equal {
        return Err(GuardedInternalityError::WeakeningMismatch);
    }
    let erasure_matches_base = univalent_equality(
        &erased_expression,
        &base_expression,
        base_outer_scope,
        EQUALITY_FUEL,
    )
    .map_err(|error| GuardedInternalityError::Equality(error.to_string()))?;
    if !erasure_matches_base.equal {
        return Err(GuardedInternalityError::ErasureMismatch);
    }

    let erase_after_weaken_expression = erase_one_unused_ambient(&weakened_expression)?;
    let weaken_after_erase_expression = weaken_by_one_ambient(&erased_expression);
    let erasure_after_weakening = univalent_equality(
        &erase_after_weaken_expression,
        &base_expression,
        base_outer_scope,
        EQUALITY_FUEL,
    )
    .map_err(|error| GuardedInternalityError::Equality(error.to_string()))?;
    let weakening_after_erasure = univalent_equality(
        &weaken_after_erase_expression,
        &guarded_expression,
        guarded_outer_scope,
        EQUALITY_FUEL,
    )
    .map_err(|error| GuardedInternalityError::Equality(error.to_string()))?;
    let both_inverse_laws_checked = erasure_after_weakening.equal && weakening_after_erasure.equal;
    if !both_inverse_laws_checked {
        return Err(GuardedInternalityError::InverseLawMismatch);
    }

    let mut projection = GuardedClauseWeakeningErasureProjection {
        version: GUARDED_CLAUSE_INTERNALITY_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        base_candidate_hash: candidate_hash(base_candidate),
        guarded_candidate_hash: candidate_hash(guarded_candidate),
        base_elaboration_hash: base_elaboration.derivation_hash,
        guarded_elaboration_hash: guarded_elaboration.derivation_hash,
        clause_index,
        base_outer_scope,
        guarded_outer_scope,
        base_expression,
        guarded_expression,
        weakened_expression,
        erased_expression,
        erase_after_weaken_expression,
        weaken_after_erase_expression,
        weakening_matches_guarded: EqualityProjection::from(&weakening_matches_guarded),
        erasure_matches_base: EqualityProjection::from(&erasure_matches_base),
        erasure_after_weakening: EqualityProjection::from(&erasure_after_weakening),
        weakening_after_erasure: EqualityProjection::from(&weakening_after_erasure),
        inserted_ambient_is_unused,
        both_inverse_laws_checked,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("guarded-clause-token", &projection);
    Ok(GuardedClauseWeakeningErasureToken { projection })
}

pub fn replay_guarded_clause_weakening_erasure_token(
    signature: &SealedSignature,
    base_candidate: &Telescope,
    guarded_candidate: &Telescope,
    visible_library: u32,
    token: &GuardedClauseWeakeningErasureToken,
) -> Result<(), GuardedInternalityError> {
    let replay = issue_guarded_clause_weakening_erasure_token(
        signature,
        base_candidate,
        guarded_candidate,
        visible_library,
        token.projection.clause_index,
    )?;
    if replay == *token {
        Ok(())
    } else {
        Err(GuardedInternalityError::ReplayMismatch)
    }
}

pub fn replay_guarded_clause_weakening_erasure_projection(
    signature: &SealedSignature,
    base_candidate: &Telescope,
    guarded_candidate: &Telescope,
    visible_library: u32,
    projection: &GuardedClauseWeakeningErasureProjection,
) -> Result<(), GuardedInternalityError> {
    let token = issue_guarded_clause_weakening_erasure_token(
        signature,
        base_candidate,
        guarded_candidate,
        visible_library,
        projection.clause_index,
    )?;
    if token.projection() == projection {
        Ok(())
    } else {
        Err(GuardedInternalityError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::{ClauseRec, ClauseRole};

    fn candidate(identity_level: u32) -> Telescope {
        Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::Var(identity_level))),
            ),
        ])
    }

    #[test]
    fn one_ambient_identity_has_per_clause_inverse_tokens() {
        let signature = SealedSignature::genesis_del_h15();
        let base = candidate(2);
        let guarded = candidate(3);
        for clause_index in [0, 1] {
            let token = issue_guarded_clause_weakening_erasure_token(
                &signature,
                &base,
                &guarded,
                15,
                clause_index,
            )
            .expect("guarded clause token");
            assert!(token.both_inverse_laws_checked());
            replay_guarded_clause_weakening_erasure_token(&signature, &base, &guarded, 15, &token)
                .expect("opaque token replays");
            replay_guarded_clause_weakening_erasure_projection(
                &signature,
                &base,
                &guarded,
                15,
                token.projection(),
            )
            .expect("projection replays");
        }
    }

    #[test]
    fn inserted_ambient_use_cannot_be_erased() {
        assert_eq!(
            erase_one_unused_ambient(&Expr::Var(1)),
            Err(GuardedInternalityError::InsertedAmbientUsed)
        );
    }

    #[test]
    fn projection_mutation_fails_replay() {
        let signature = SealedSignature::genesis_del_h15();
        let base = candidate(2);
        let guarded = candidate(3);
        let token =
            issue_guarded_clause_weakening_erasure_token(&signature, &base, &guarded, 15, 1)
                .expect("token");
        let mut projection = token.projection().clone();
        projection.both_inverse_laws_checked = false;
        assert_eq!(
            replay_guarded_clause_weakening_erasure_projection(
                &signature,
                &base,
                &guarded,
                15,
                &projection,
            ),
            Err(GuardedInternalityError::ReplayMismatch)
        );
    }
}
