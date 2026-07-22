//! Count-blind execution of adopted R1 on the sealed Stage-1 source shape.
//!
//! This witness deliberately separates the part R1 decides immediately
//! (typed package shape, canonical completion, carrier provenance, and
//! coverage direction) from its generator-membership exception.  The latter
//! stays pending until E-4 supplies a complete basis; absence of another
//! clause is never used as a non-membership proof.

use pen_core::clause::ClauseRole;
use pen_core::expr::Expr;
use pen_core::telescope::Telescope;
use pen_type::elaborate::{KernelTy, SealedSignature, elaborate_telescope};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const STAGE1_R1_TOKEN_VERSION: &str = "schema2-stage1-r1-package-v1";
pub const STAGE1_R1_FORMATION_COMPLETION_PACKAGE_RULE: &str =
    "formation-completion-package-family-rule-v1";
pub const R1_MEMBERSHIP_GAP: &str = "E4_NATURALITY_GENERATOR_MEMBERSHIP_NOT_YET_DECIDABLE";
pub const R1_FALSIFIER: &str = "F-Q4";

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum R1LocalRole {
    KernelHead,
    AdjointMate,
    SupportAction,
    Coherence,
}

impl R1LocalRole {
    pub const ALL: [Self; 4] = [
        Self::KernelHead,
        Self::AdjointMate,
        Self::SupportAction,
        Self::Coherence,
    ];
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum R1CoverageDirection {
    CompletedActionCoversCarrier,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct R1RoleMembershipAudit {
    pub role: R1LocalRole,
    pub decision: String,
    pub generator_membership_token: Option<String>,
    pub obstruction: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Stage1R1PackageToken {
    pub token_version: String,
    pub rule: String,
    pub source_candidate_hash: String,
    pub source_elaboration_hash: String,
    pub source_signature_digest: String,
    pub carrier_clause: u16,
    pub carrier_expression: String,
    pub carrier_kernel_type: String,
    pub completion_clause: u16,
    pub completed_action_expression: String,
    pub completed_action_kernel_type: String,
    pub same_package_dependency_level: u32,
    pub same_package_dependency_resolves_to_clause: u16,
    pub typed_package_shape_replayed: bool,
    pub same_package_dependency_replayed: bool,
    pub canonical_family_expression: String,
    pub carrier_provenance_expression: String,
    pub coverage_direction: R1CoverageDirection,
    pub completed_action_covers_carrier_by_adopted_rule: bool,
    pub local_role_inventory: Vec<R1RoleMembershipAudit>,
    pub completed_package_family_issued: bool,
    pub exception_generator_membership_decided: bool,
    pub total_stage1_family_count: Option<u32>,
    pub falsifier: String,
    pub archived_count_used_as_input: bool,
    pub acceptance_bar_used_as_input: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Stage1R1Error {
    #[error("Stage-1 telescope no longer has the adopted Univ/App(Univ,Var(1)) package shape")]
    SourceShapeDrift,
    #[error("Stage-1 package failed kernel elaboration: {0}")]
    Elaboration(String),
    #[error("Stage-1 R1 token replay mismatch")]
    ReplayMismatch,
}

fn tagged_hash(domain: &str, payload: &impl Serialize) -> String {
    let bytes = serde_json::to_vec(&(STAGE1_R1_TOKEN_VERSION, domain, payload))
        .expect("Stage-1 R1 proof data serializes");
    format!("blake3:{}", blake3::hash(&bytes).to_hex())
}

fn expression_name(expression: &Expr) -> String {
    match expression {
        Expr::Univ => "Univ".to_owned(),
        Expr::App(function, argument)
            if function.as_ref() == &Expr::Univ && argument.as_ref() == &Expr::Var(1) =>
        {
            "App(Univ,Var(1))".to_owned()
        }
        other => format!("{other:?}"),
    }
}

fn kernel_type_name(ty: &KernelTy) -> String {
    match ty {
        KernelTy::Type => "Type".to_owned(),
        other => format!("{other:?}"),
    }
}

pub fn issue_stage1_r1_package_token() -> Result<Stage1R1PackageToken, Stage1R1Error> {
    let telescope = Telescope::reference(1);
    let expected_carrier = Expr::Univ;
    let expected_completion = Expr::App(Box::new(Expr::Univ), Box::new(Expr::Var(1)));
    if telescope.clauses.len() != 2
        || telescope.clauses[0].role != ClauseRole::Formation
        || telescope.clauses[0].expr != expected_carrier
        || telescope.clauses[1].role != ClauseRole::Formation
        || telescope.clauses[1].expr != expected_completion
    {
        return Err(Stage1R1Error::SourceShapeDrift);
    }

    // Stage 1 is checked over the exact empty predecessor signature.  The
    // current telescope is the subject, never an already-visible library
    // entry in its own typing context.
    let signature = SealedSignature::from_telescopes(Vec::new());
    let elaboration = elaborate_telescope(&signature, &telescope, 0)
        .map_err(|error| Stage1R1Error::Elaboration(error.to_string()))?;
    if elaboration.clauses.len() != 2
        || elaboration.clauses[0].kernel_ty != KernelTy::Type
        || elaboration.clauses[1].kernel_ty != KernelTy::Type
        || elaboration.clauses[0].kernel_role != ClauseRole::Formation
        || elaboration.clauses[1].kernel_role != ClauseRole::Formation
    {
        return Err(Stage1R1Error::SourceShapeDrift);
    }

    // Under the frozen level convention, with zero ambient parameters and
    // one prior field, Var(1) in clause 1 resolves to prior clause 0.
    let same_package_dependency_level = 1;
    let same_package_dependency_resolves_to_clause = 0;
    let typed_package_shape_replayed = true;
    let same_package_dependency_replayed = elaboration.ambient_parameters == 0;
    let local_role_inventory = R1LocalRole::ALL
        .into_iter()
        .map(|role| R1RoleMembershipAudit {
            role,
            decision: "pending_generator_membership".to_owned(),
            generator_membership_token: None,
            obstruction: R1_MEMBERSHIP_GAP.to_owned(),
        })
        .collect::<Vec<_>>();
    let mut token = Stage1R1PackageToken {
        token_version: STAGE1_R1_TOKEN_VERSION.to_owned(),
        rule: STAGE1_R1_FORMATION_COMPLETION_PACKAGE_RULE.to_owned(),
        source_candidate_hash: elaboration.subject_hash,
        source_elaboration_hash: elaboration.derivation_hash,
        source_signature_digest: elaboration.signature_digest,
        carrier_clause: 0,
        carrier_expression: expression_name(&telescope.clauses[0].expr),
        carrier_kernel_type: kernel_type_name(&elaboration.clauses[0].kernel_ty),
        completion_clause: 1,
        completed_action_expression: expression_name(&telescope.clauses[1].expr),
        completed_action_kernel_type: kernel_type_name(&elaboration.clauses[1].kernel_ty),
        same_package_dependency_level,
        same_package_dependency_resolves_to_clause,
        typed_package_shape_replayed,
        same_package_dependency_replayed,
        canonical_family_expression: "App(Univ,Var(1))".to_owned(),
        carrier_provenance_expression: "Univ".to_owned(),
        coverage_direction: R1CoverageDirection::CompletedActionCoversCarrier,
        completed_action_covers_carrier_by_adopted_rule: true,
        local_role_inventory,
        completed_package_family_issued: true,
        exception_generator_membership_decided: false,
        total_stage1_family_count: None,
        falsifier: R1_FALSIFIER.to_owned(),
        archived_count_used_as_input: false,
        acceptance_bar_used_as_input: false,
        derivation_hash: String::new(),
    };
    token.derivation_hash = tagged_hash("stage1-package", &token);
    Ok(token)
}

pub fn replay_stage1_r1_package_token(token: &Stage1R1PackageToken) -> Result<(), Stage1R1Error> {
    let replay = issue_stage1_r1_package_token()?;
    if replay == *token {
        Ok(())
    } else {
        Err(Stage1R1Error::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adopted_direction_replays_but_exception_stays_pending() {
        let token = issue_stage1_r1_package_token().expect("Stage-1 package types");
        replay_stage1_r1_package_token(&token).expect("token replays");
        assert_eq!(token.canonical_family_expression, "App(Univ,Var(1))");
        assert_eq!(token.carrier_provenance_expression, "Univ");
        assert!(token.completed_action_covers_carrier_by_adopted_rule);
        assert_eq!(token.local_role_inventory.len(), 4);
        assert!(token.completed_package_family_issued);
        assert!(!token.exception_generator_membership_decided);
        assert_eq!(token.total_stage1_family_count, None);
        assert!(!token.archived_count_used_as_input);
        assert!(!token.acceptance_bar_used_as_input);
    }

    #[test]
    fn canonical_direction_and_pending_decision_are_replay_protected() {
        let token = issue_stage1_r1_package_token().expect("Stage-1 package types");
        let mut reversed = token.clone();
        reversed.canonical_family_expression = "Univ".to_owned();
        assert_eq!(
            replay_stage1_r1_package_token(&reversed),
            Err(Stage1R1Error::ReplayMismatch)
        );
        let mut invented_count = token;
        invented_count.total_stage1_family_count = Some(1);
        assert_eq!(
            replay_stage1_r1_package_token(&invented_count),
            Err(Stage1R1Error::ReplayMismatch)
        );
    }
}
