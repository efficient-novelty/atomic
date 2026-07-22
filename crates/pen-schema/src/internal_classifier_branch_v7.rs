//! Versioned contextual-Internal successor.
//!
//! The successor keeps the explicit ambient declaration outside the frozen
//! raw [`Telescope`] payload.  It also corrects the v7 witness disposition:
//! under absolute levels `Var(3)` is `local-var-1`, while the inferred
//! `ambient-param-1` is unused.  That witness therefore earns Internal by the
//! pre-existing weakening/erasure theorem, not by contextual hypothesis use.

use crate::internal_classifier_branch_v2::{InductiveClauseRecord, InductiveInternalityAttempt};
use crate::internal_classifier_branch_v3::RawCandidateDecisionV4;
use crate::internal_classifier_branch_v4::RawCandidateDecisionV5;
use crate::internal_classifier_branch_v5::RawCandidateDecisionV6;
use crate::internal_classifier_branch_v6::{RawCandidateDecisionV7, classify_raw_candidate_v7};
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::contextual_internality::{
    AmbientContextDeclarationProjection, AmbientContextDeclarationToken,
    ContextualInternalityError, ContextualInternalityProjection, ContextualMotive,
    issue_ambient_context_declaration_token, issue_contextual_internality_token,
    replay_contextual_internality_projection,
};
use pen_type::elaborate::{SealedSignature, elaborate_telescope};
use pen_type::guarded_internality::{
    GuardedClauseWeakeningErasureProjection, erase_one_unused_ambient,
    issue_guarded_clause_weakening_erasure_token,
    replay_guarded_clause_weakening_erasure_projection,
};
use pen_type::normalize::substitute_level;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const CONTEXTUAL_INTERNALITY_SCHEMA: &str = "schema2-contextual-internality-v1";
pub const CONTEXTUAL_INTERNALITY_DATE: &str = "2026-07-21";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/contextual_internality_adjudication.md");
const V7_RESULT_BYTES: &[u8] = include_bytes!("../../../docs/SCHEMA2_GLOBAL_E4_V7_RESULT.md");
const V7_ASSEMBLY_BYTES: &[u8] = include_bytes!("../../../docs/schema2_global_e4_assembly_v7.json");
const TOKEN_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/contextual_internality.rs");
const PREDECESSOR_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v6.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("internal_classifier_branch_v7.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextualClauseEvidence {
    Inherited(InductiveClauseRecord),
    ContextWeakening(GuardedClauseWeakeningErasureProjection),
    Contextual(ContextualInternalityProjection),
}

impl ContextualClauseEvidence {
    fn clause_index(&self) -> u16 {
        match self {
            Self::Inherited(record) => record.clause_index,
            Self::ContextWeakening(projection) => projection.clause_index,
            Self::Contextual(projection) => projection.clause_index,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GuardedClosureCertificate {
    pub candidate: Telescope,
    pub erased_base_candidate: Telescope,
    pub predecessor_decision: RawCandidateDecisionV7,
    pub erased_base_decision: RawCandidateDecisionV7,
    pub clause_evidence: Vec<GuardedClauseWeakeningErasureProjection>,
    pub ambient_parameter_unused_in_every_clause: bool,
    pub both_inverse_laws_replayed_per_clause: bool,
    pub base_candidate_internal: bool,
    pub internal_certificate_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualCandidateCertificate {
    pub candidate: Telescope,
    pub predecessor_decision: RawCandidateDecisionV7,
    pub ambient_context: AmbientContextDeclarationProjection,
    pub clause_evidence: Vec<ContextualClauseEvidence>,
    pub final_internal_clause_hashes: BTreeMap<u16, String>,
    pub every_clause_covered_exactly_once: bool,
    pub live_contextual_clause_count: usize,
    pub context_weakening_clause_count: usize,
    pub every_projection_replayed: bool,
    pub instantiation_coherence_replayed: bool,
    pub no_credit_anchor_or_orbit_minted: bool,
    pub internal_certificate_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RawCandidateDecisionV8 {
    NamedExclusion {
        predecessor_decision: RawCandidateDecisionV7,
    },
    InternalPredecessor {
        predecessor_decision: RawCandidateDecisionV7,
    },
    InternalGuardedClosure {
        certificate: GuardedClosureCertificate,
        marginal_nu: u32,
        derivation_hash: String,
    },
    InternalContextual {
        certificate: ContextualCandidateCertificate,
        marginal_nu: u32,
        derivation_hash: String,
    },
    Classified {
        predecessor_decision: RawCandidateDecisionV7,
    },
    NamedTypedObstruction {
        predecessor_decision: RawCandidateDecisionV7,
        contextual_failure: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualFalsifierRecord {
    pub v7_var3_replayed_as_local_not_ambient: bool,
    pub v7_contextual_misclassification_rejected: bool,
    pub wrong_motive_rejected: bool,
    pub nonformable_motive_rejected: bool,
    pub pathcon_rejected_as_charged: bool,
    pub raw_candidate_without_declaration_remains_unclassified: bool,
    pub projection_mutation_rejected: bool,
    pub zero_credit_preserved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualForbiddenOutputs {
    pub global_e4_v8_executed: bool,
    pub pending_membership_verdicts_issued: bool,
    pub e2b_executed: bool,
    pub historical_totals_issued: bool,
    pub fq2_evaluated: bool,
    pub e5_f1_executed: bool,
    pub bridge_or_fork_executed: bool,
}

impl ContextualForbiddenOutputs {
    fn all_withheld(&self) -> bool {
        !self.global_e4_v8_executed
            && !self.pending_membership_verdicts_issued
            && !self.e2b_executed
            && !self.historical_totals_issued
            && !self.fq2_evaluated
            && !self.e5_f1_executed
            && !self.bridge_or_fork_executed
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualInternalityCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<ContextualSourceBinding>,
    pub predecessor_global_e4_v7_digest: String,
    pub predecessor_global_e4_v7_source_bound: bool,
    pub adopted_contextual_rule_v1: bool,
    pub frozen_raw_telescope_carries_ambient_motives: bool,
    pub v7_witness_predecessor_unknown: bool,
    pub v7_witness_successor_decision: RawCandidateDecisionV8,
    pub v7_witness_now_internal_by_guarded_inverse: bool,
    pub v7_witness_contextual_use_count: usize,
    pub declared_context_control: Telescope,
    pub declared_context: AmbientContextDeclarationProjection,
    pub declared_context_successor_decision: RawCandidateDecisionV8,
    pub declared_context_control_internal: bool,
    pub declared_context_default_decision: RawCandidateDecisionV8,
    pub raw_surface_declaration_disconnect_exposed: bool,
    pub falsifiers: ContextualFalsifierRecord,
    pub marginal_nu: u32,
    pub global_e4_v8_authorized: bool,
    pub forbidden_outputs: ContextualForbiddenOutputs,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ContextualInternalityReplay {
    pub valid: bool,
    pub v7_witness_resolved: bool,
    pub declared_context_control_internal: bool,
    pub motive_typing_replayed: bool,
    pub instantiation_coherence_replayed: bool,
    pub raw_surface_declaration_disconnect_exposed: bool,
    pub falsifiers_replayed: bool,
    pub marginal_nu: u32,
    pub global_e4_v8_authorized: bool,
    pub forbidden_outputs_withheld: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ContextualClassifierError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("constructor evidence failed: {0}")]
    Constructor(String),
    #[error("invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CONTEXTUAL_INTERNALITY_SCHEMA, domain, value))
        .expect("contextual classifier evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<ContextualSourceBinding> {
    [
        (
            "docs/contextual_internality_adjudication.md",
            "adopted_rule_and_falsifiers",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/SCHEMA2_GLOBAL_E4_V7_RESULT.md",
            "claimed_live_ambient_predecessor",
            V7_RESULT_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v7.json",
            "sealed_v7_derivation_source",
            V7_ASSEMBLY_BYTES,
        ),
        (
            "crates/pen-type/src/contextual_internality.rs",
            "explicit_motive_contextual_token",
            TOKEN_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v6.rs",
            "certified_field_predecessor_classifier",
            PREDECESSOR_SOURCE_BYTES,
        ),
        (
            "crates/pen-schema/src/internal_classifier_branch_v7.rs",
            "contextual_classifier_successor",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| ContextualSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn sealed_v7_binding() -> Result<String, ContextualClassifierError> {
    let value: serde_json::Value = serde_json::from_slice(V7_ASSEMBLY_BYTES)
        .map_err(|error| ContextualClassifierError::Json(error.to_string()))?;
    let valid = value.get("schema").and_then(serde_json::Value::as_str)
        == Some("schema2-global-e4-assembly-v7")
        && value
            .pointer("/next_guarded_application_unknown/exact_raw_catalog_member")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        && value
            .get("class_exhaustion_proved")
            .and_then(serde_json::Value::as_bool)
            == Some(false);
    if !valid {
        return Err(ContextualClassifierError::Prerequisite(
            "sealed v7 gap binding failed".to_owned(),
        ));
    }
    value
        .get("result_digest")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| ContextualClassifierError::Prerequisite("v7 digest missing".to_owned()))
}

pub fn v7_guarded_application_candidate() -> Telescope {
    Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(
            ClauseRole::Introduction,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(3)),
                Box::new(Expr::Lib(15)),
            ))),
        ),
    ])
}

/// A raw telescope whose inferred context really is used: `Var(3)` forces one
/// ambient slot while resolving to the lambda binder, and the nested
/// `Var(1)` is consequently the ambient function.
pub fn declared_context_control_candidate() -> Telescope {
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

pub fn declared_context_control_motives() -> Vec<ContextualMotive> {
    vec![ContextualMotive::Function {
        domain: Box::new(ContextualMotive::Type),
        codomain: Box::new(ContextualMotive::Type),
    }]
}

fn predecessor_internal_attempt(
    decision: &RawCandidateDecisionV7,
) -> Option<&InductiveInternalityAttempt> {
    let RawCandidateDecisionV7::NamedTypedObstruction {
        predecessor_decision:
            RawCandidateDecisionV6::NamedTypedObstruction {
                predecessor_decision:
                    RawCandidateDecisionV5::NamedTypedObstruction {
                        predecessor_decision:
                            RawCandidateDecisionV4::NamedTypedObstruction {
                                predecessor_decision:
                                    crate::internal_classifier_branch_v2::RawCandidateDecisionV3::NamedTypedObstruction {
                                        internal_attempt, ..
                                    },
                            },
                    },
            },
    } = decision
    else {
        return None;
    };
    Some(internal_attempt)
}

fn v7_is_internal(decision: &RawCandidateDecisionV7) -> bool {
    matches!(
        decision,
        RawCandidateDecisionV7::InternalPredecessor { .. }
            | RawCandidateDecisionV7::InternalCertifiedField { .. }
    )
}

fn erase_candidate(candidate: &Telescope) -> Result<Telescope, ContextualClassifierError> {
    candidate
        .clauses
        .iter()
        .map(|clause| {
            erase_one_unused_ambient(&clause.expr)
                .map(|expr| ClauseRec::new(clause.role, expr))
                .map_err(|error| ContextualClassifierError::Constructor(error.to_string()))
        })
        .collect::<Result<Vec<_>, _>>()
        .map(Telescope::new)
}

fn issue_guarded_closure(
    candidate: &Telescope,
    predecessor_decision: RawCandidateDecisionV7,
) -> Result<GuardedClosureCertificate, ContextualClassifierError> {
    let signature = SealedSignature::genesis_del_h15();
    let guarded_elaboration = elaborate_telescope(&signature, candidate, 15)
        .map_err(|error| ContextualClassifierError::Constructor(error.to_string()))?;
    if guarded_elaboration.ambient_parameters != 1 {
        return Err(ContextualClassifierError::Constructor(
            "guarded closure currently proves exactly one inferred ambient parameter".to_owned(),
        ));
    }
    let erased_base_candidate = erase_candidate(candidate)?;
    let base_elaboration = elaborate_telescope(&signature, &erased_base_candidate, 15)
        .map_err(|error| ContextualClassifierError::Constructor(error.to_string()))?;
    if base_elaboration.ambient_parameters != 0 {
        return Err(ContextualClassifierError::Constructor(
            "erasure did not produce a closed base candidate".to_owned(),
        ));
    }
    let erased_base_decision = classify_raw_candidate_v7(&erased_base_candidate);
    let base_candidate_internal = v7_is_internal(&erased_base_decision);
    if !base_candidate_internal {
        return Err(ContextualClassifierError::Constructor(
            "erased base candidate has not earned Internal".to_owned(),
        ));
    }
    let mut clause_evidence = Vec::with_capacity(candidate.kappa());
    for clause_index in 0..u16::try_from(candidate.kappa()).expect("kappa fits u16") {
        let token = issue_guarded_clause_weakening_erasure_token(
            &signature,
            &erased_base_candidate,
            candidate,
            15,
            clause_index,
        )
        .map_err(|error| ContextualClassifierError::Constructor(error.to_string()))?;
        let projection = token.projection().clone();
        replay_guarded_clause_weakening_erasure_projection(
            &signature,
            &erased_base_candidate,
            candidate,
            15,
            &projection,
        )
        .map_err(|error| ContextualClassifierError::Constructor(error.to_string()))?;
        clause_evidence.push(projection);
    }
    let ambient_parameter_unused_in_every_clause = clause_evidence
        .iter()
        .all(|projection| projection.inserted_ambient_is_unused);
    let both_inverse_laws_replayed_per_clause = clause_evidence
        .iter()
        .all(|projection| projection.both_inverse_laws_checked);
    let marginal_nu = 0;
    let internal_certificate_issued = base_candidate_internal
        && ambient_parameter_unused_in_every_clause
        && both_inverse_laws_replayed_per_clause;
    let derivation_hash = tagged_hash(
        "guarded-whole-candidate-closure",
        &(
            candidate,
            &erased_base_candidate,
            &predecessor_decision,
            &erased_base_decision,
            &clause_evidence,
            ambient_parameter_unused_in_every_clause,
            both_inverse_laws_replayed_per_clause,
            base_candidate_internal,
            internal_certificate_issued,
            marginal_nu,
        ),
    );
    Ok(GuardedClosureCertificate {
        candidate: candidate.clone(),
        erased_base_candidate,
        predecessor_decision,
        erased_base_decision,
        clause_evidence,
        ambient_parameter_unused_in_every_clause,
        both_inverse_laws_replayed_per_clause,
        base_candidate_internal,
        internal_certificate_issued,
        marginal_nu,
        derivation_hash,
    })
}

fn context_probe(declaration: &AmbientContextDeclarationProjection) -> Option<Expr> {
    let motive = &declaration.hypotheses.first()?.motive;
    match motive {
        ContextualMotive::Type => Some(Expr::Univ),
        ContextualMotive::Function { domain, codomain }
            if domain.as_ref() == &ContextualMotive::Type
                && codomain.as_ref() == &ContextualMotive::Type =>
        {
            Some(Expr::Lam(Box::new(Expr::Univ)))
        }
        _ => None,
    }
}

fn instantiate_one_context(candidate: &Telescope, probe: &Expr) -> Telescope {
    Telescope::new(
        candidate
            .clauses
            .iter()
            .map(|clause| ClauseRec::new(clause.role, substitute_level(&clause.expr, 1, probe)))
            .collect(),
    )
}

fn issue_contextual_candidate(
    candidate: &Telescope,
    predecessor_decision: RawCandidateDecisionV7,
    declaration: &AmbientContextDeclarationToken,
) -> Result<ContextualCandidateCertificate, ContextualClassifierError> {
    let internal_attempt =
        predecessor_internal_attempt(&predecessor_decision).ok_or_else(|| {
            ContextualClassifierError::Constructor(
                "predecessor does not expose an inductive Internal attempt".to_owned(),
            )
        })?;
    let signature = SealedSignature::genesis_del_h15();
    let probe = context_probe(declaration.projection()).ok_or_else(|| {
        ContextualClassifierError::Constructor(
            "no context-weakening probe is registered for this motive".to_owned(),
        )
    })?;
    let closed_probe_candidate = instantiate_one_context(candidate, &probe);
    if elaborate_telescope(&signature, &closed_probe_candidate, 15)
        .map_err(|error| ContextualClassifierError::Constructor(error.to_string()))?
        .ambient_parameters
        != 0
    {
        return Err(ContextualClassifierError::Constructor(
            "context probe did not close candidate".to_owned(),
        ));
    }
    let mut clause_evidence = Vec::new();
    let mut certified = BTreeMap::<u16, String>::new();
    for clause in &internal_attempt.clauses {
        if clause.earned_internal {
            certified.insert(clause.clause_index, clause.derivation_hash.clone());
            clause_evidence.push(ContextualClauseEvidence::Inherited(clause.clone()));
            continue;
        }
        match issue_contextual_internality_token(
            &signature,
            candidate,
            15,
            clause.clause_index,
            declaration,
            &certified,
        ) {
            Ok(token) => {
                let projection = token.projection().clone();
                replay_contextual_internality_projection(&signature, candidate, 15, &projection)
                    .map_err(|error| ContextualClassifierError::Constructor(error.to_string()))?;
                certified.insert(clause.clause_index, projection.derivation_hash.clone());
                clause_evidence.push(ContextualClauseEvidence::Contextual(projection));
            }
            Err(ContextualInternalityError::NoLiveAmbientUse) => {
                let token = issue_guarded_clause_weakening_erasure_token(
                    &signature,
                    &closed_probe_candidate,
                    candidate,
                    15,
                    clause.clause_index,
                )
                .map_err(|error| ContextualClassifierError::Constructor(error.to_string()))?;
                let projection = token.projection().clone();
                replay_guarded_clause_weakening_erasure_projection(
                    &signature,
                    &closed_probe_candidate,
                    candidate,
                    15,
                    &projection,
                )
                .map_err(|error| ContextualClassifierError::Constructor(error.to_string()))?;
                certified.insert(clause.clause_index, projection.derivation_hash.clone());
                clause_evidence.push(ContextualClauseEvidence::ContextWeakening(projection));
            }
            Err(error) => return Err(ContextualClassifierError::Constructor(error.to_string())),
        }
    }
    let every_clause_covered_exactly_once = certified.len() == candidate.kappa()
        && clause_evidence
            .iter()
            .map(ContextualClauseEvidence::clause_index)
            .eq(0..u16::try_from(candidate.kappa()).expect("kappa fits u16"));
    let live_contextual_clause_count = clause_evidence
        .iter()
        .filter(|evidence| matches!(evidence, ContextualClauseEvidence::Contextual(_)))
        .count();
    let context_weakening_clause_count = clause_evidence
        .iter()
        .filter(|evidence| matches!(evidence, ContextualClauseEvidence::ContextWeakening(_)))
        .count();
    let every_projection_replayed = !clause_evidence.is_empty();
    let instantiation_coherence_replayed = clause_evidence.iter().all(|evidence| {
        !matches!(evidence, ContextualClauseEvidence::Contextual(projection) if !projection.instantiation_coherence_replayed)
    });
    let no_credit_anchor_or_orbit_minted = clause_evidence.iter().all(|evidence| match evidence {
        ContextualClauseEvidence::Inherited(_) => true,
        ContextualClauseEvidence::ContextWeakening(_) => true,
        ContextualClauseEvidence::Contextual(projection) => {
            projection.no_credit_anchor_or_orbit_minted && projection.marginal_nu == 0
        }
    });
    let marginal_nu = 0;
    let internal_certificate_issued = every_clause_covered_exactly_once
        && live_contextual_clause_count > 0
        && every_projection_replayed
        && instantiation_coherence_replayed
        && no_credit_anchor_or_orbit_minted;
    let final_internal_clause_hashes = certified;
    let derivation_hash = tagged_hash(
        "contextual-candidate-certificate",
        &(
            candidate,
            &predecessor_decision,
            declaration.projection(),
            &clause_evidence,
            &final_internal_clause_hashes,
            every_clause_covered_exactly_once,
            live_contextual_clause_count,
            context_weakening_clause_count,
            every_projection_replayed,
            instantiation_coherence_replayed,
            no_credit_anchor_or_orbit_minted,
            internal_certificate_issued,
            marginal_nu,
        ),
    );
    Ok(ContextualCandidateCertificate {
        candidate: candidate.clone(),
        predecessor_decision,
        ambient_context: declaration.projection().clone(),
        clause_evidence,
        final_internal_clause_hashes,
        every_clause_covered_exactly_once,
        live_contextual_clause_count,
        context_weakening_clause_count,
        every_projection_replayed,
        instantiation_coherence_replayed,
        no_credit_anchor_or_orbit_minted,
        internal_certificate_issued,
        marginal_nu,
        derivation_hash,
    })
}

fn classify_with_optional_declaration(
    candidate: &Telescope,
    declaration: Option<&AmbientContextDeclarationToken>,
) -> RawCandidateDecisionV8 {
    let predecessor_decision = classify_raw_candidate_v7(candidate);
    if matches!(
        predecessor_decision,
        RawCandidateDecisionV7::NamedTypedObstruction { .. }
    ) {
        if let Ok(certificate) = issue_guarded_closure(candidate, predecessor_decision.clone()) {
            let marginal_nu = certificate.marginal_nu;
            let derivation_hash = tagged_hash(
                "guarded-successor-classification",
                &(&certificate.derivation_hash, marginal_nu),
            );
            return RawCandidateDecisionV8::InternalGuardedClosure {
                certificate,
                marginal_nu,
                derivation_hash,
            };
        }
        if let Some(declaration) = declaration {
            if let Ok(certificate) =
                issue_contextual_candidate(candidate, predecessor_decision.clone(), declaration)
            {
                let marginal_nu = certificate.marginal_nu;
                let derivation_hash = tagged_hash(
                    "contextual-successor-classification",
                    &(&certificate.derivation_hash, marginal_nu),
                );
                return RawCandidateDecisionV8::InternalContextual {
                    certificate,
                    marginal_nu,
                    derivation_hash,
                };
            }
        }
    }
    match predecessor_decision {
        decision @ RawCandidateDecisionV7::NamedExclusion { .. } => {
            RawCandidateDecisionV8::NamedExclusion {
                predecessor_decision: decision,
            }
        }
        decision @ (RawCandidateDecisionV7::InternalPredecessor { .. }
        | RawCandidateDecisionV7::InternalCertifiedField { .. }) => {
            RawCandidateDecisionV8::InternalPredecessor {
                predecessor_decision: decision,
            }
        }
        decision @ RawCandidateDecisionV7::Classified { .. } => {
            RawCandidateDecisionV8::Classified {
                predecessor_decision: decision,
            }
        }
        decision @ RawCandidateDecisionV7::NamedTypedObstruction { .. } => {
            RawCandidateDecisionV8::NamedTypedObstruction {
                predecessor_decision: decision,
                contextual_failure: if declaration.is_some() {
                    "explicit contextual derivation failed its motive or specialization obligations"
                        .to_owned()
                } else {
                    "frozen raw Telescope carries no ambient-motive declaration".to_owned()
                },
            }
        }
    }
}

pub fn classify_raw_candidate_v8(candidate: &Telescope) -> RawCandidateDecisionV8 {
    classify_with_optional_declaration(candidate, None)
}

pub fn classify_raw_candidate_v8_with_declaration(
    candidate: &Telescope,
    declaration: &AmbientContextDeclarationToken,
) -> RawCandidateDecisionV8 {
    classify_with_optional_declaration(candidate, Some(declaration))
}

fn issue_falsifiers(
    control: &Telescope,
    declaration: &AmbientContextDeclarationToken,
) -> ContextualFalsifierRecord {
    let signature = SealedSignature::genesis_del_h15();
    let v7 = v7_guarded_application_candidate();
    let elaboration = elaborate_telescope(&signature, &v7, 15).expect("v7 elaborates");
    let v7_var3_replayed_as_local_not_ambient = elaboration.clauses[1]
        .derivation
        .children
        .first()
        .and_then(|lambda_body| lambda_body.children.first())
        .is_some_and(|head| head.rule == "local-var-1");
    let v7_declaration = issue_ambient_context_declaration_token(
        &signature,
        &v7,
        15,
        declared_context_control_motives(),
    )
    .expect("v7 declaration audit");
    let v7_contextual_misclassification_rejected = matches!(
        issue_contextual_internality_token(
            &signature,
            &v7,
            15,
            1,
            &v7_declaration,
            &BTreeMap::from([(0, "earned".to_owned())]),
        ),
        Err(ContextualInternalityError::NoLiveAmbientUse)
    );
    let wrong = issue_ambient_context_declaration_token(
        &signature,
        control,
        15,
        vec![ContextualMotive::Type],
    )
    .expect("wrong but formable declaration");
    let wrong_motive_rejected = matches!(
        issue_contextual_internality_token(
            &signature,
            control,
            15,
            1,
            &wrong,
            &BTreeMap::from([(0, "earned".to_owned())]),
        ),
        Err(ContextualInternalityError::IllTypedAmbientUse { .. })
    );
    let nonformable_motive_rejected = matches!(
        issue_ambient_context_declaration_token(
            &signature,
            control,
            15,
            vec![ContextualMotive::Neutral],
        ),
        Err(ContextualInternalityError::MotiveNotFormable { .. })
    );
    let path = Telescope::new(vec![
        ClauseRec::new(ClauseRole::Formation, Expr::Univ),
        ClauseRec::new(
            ClauseRole::PathAttach,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Var(3)),
                Box::new(Expr::App(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::PathCon(1)),
                )),
            ))),
        ),
    ]);
    let path_declaration = issue_ambient_context_declaration_token(
        &signature,
        &path,
        15,
        declared_context_control_motives(),
    )
    .expect("path declaration");
    let pathcon_rejected_as_charged = matches!(
        issue_contextual_internality_token(
            &signature,
            &path,
            15,
            1,
            &path_declaration,
            &BTreeMap::from([(0, "earned".to_owned())]),
        ),
        Err(ContextualInternalityError::ChargedPathConstructor)
            | Err(ContextualInternalityError::IllTypedAmbientUse { .. })
    );
    let raw_candidate_without_declaration_remains_unclassified = matches!(
        classify_raw_candidate_v8(control),
        RawCandidateDecisionV8::NamedTypedObstruction { .. }
    );
    let token = issue_contextual_internality_token(
        &signature,
        control,
        15,
        1,
        declaration,
        &BTreeMap::from([(0, "earned".to_owned())]),
    )
    .expect("contextual control token");
    let mut mutation = token.projection().clone();
    mutation.marginal_nu = 1;
    let projection_mutation_rejected = matches!(
        replay_contextual_internality_projection(&signature, control, 15, &mutation),
        Err(ContextualInternalityError::ReplayMismatch)
    );
    let zero_credit_preserved =
        token.marginal_nu() == 0 && token.projection().no_credit_anchor_or_orbit_minted;
    let derivation_hash = tagged_hash(
        "contextual-falsifiers",
        &(
            v7_var3_replayed_as_local_not_ambient,
            v7_contextual_misclassification_rejected,
            wrong_motive_rejected,
            nonformable_motive_rejected,
            pathcon_rejected_as_charged,
            raw_candidate_without_declaration_remains_unclassified,
            projection_mutation_rejected,
            zero_credit_preserved,
        ),
    );
    ContextualFalsifierRecord {
        v7_var3_replayed_as_local_not_ambient,
        v7_contextual_misclassification_rejected,
        wrong_motive_rejected,
        nonformable_motive_rejected,
        pathcon_rejected_as_charged,
        raw_candidate_without_declaration_remains_unclassified,
        projection_mutation_rejected,
        zero_credit_preserved,
        derivation_hash,
    }
}

fn certificate_digest(certificate: &ContextualInternalityCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("contextual-internality-certificate", &projection)
}

pub fn issue_contextual_internality_certificate()
-> Result<ContextualInternalityCertificate, ContextualClassifierError> {
    let predecessor_global_e4_v7_digest = sealed_v7_binding()?;
    let signature = SealedSignature::genesis_del_h15();
    let v7_witness = v7_guarded_application_candidate();
    let v7_witness_predecessor_unknown = matches!(
        classify_raw_candidate_v7(&v7_witness),
        RawCandidateDecisionV7::NamedTypedObstruction { .. }
    );
    let v7_witness_successor_decision = classify_raw_candidate_v8(&v7_witness);
    let (v7_witness_now_internal_by_guarded_inverse, v7_witness_contextual_use_count) =
        match &v7_witness_successor_decision {
            RawCandidateDecisionV8::InternalGuardedClosure { certificate, .. } => {
                (certificate.internal_certificate_issued, 0)
            }
            _ => (false, usize::MAX),
        };

    let declared_context_control = declared_context_control_candidate();
    let declaration = issue_ambient_context_declaration_token(
        &signature,
        &declared_context_control,
        15,
        declared_context_control_motives(),
    )
    .map_err(|error| ContextualClassifierError::Constructor(error.to_string()))?;
    let declared_context = declaration.projection().clone();
    let declared_context_successor_decision =
        classify_raw_candidate_v8_with_declaration(&declared_context_control, &declaration);
    let (
        declared_context_control_internal,
        motive_typing_replayed,
        instantiation_coherence_replayed,
        marginal_nu,
    ) = match &declared_context_successor_decision {
        RawCandidateDecisionV8::InternalContextual {
            certificate,
            marginal_nu,
            ..
        } => (
            certificate.internal_certificate_issued,
            certificate.clause_evidence.iter().all(|evidence| {
                !matches!(evidence, ContextualClauseEvidence::Contextual(projection) if !projection.every_ambient_use_motive_typed)
            }),
            certificate.instantiation_coherence_replayed,
            *marginal_nu,
        ),
        _ => (false, false, false, u32::MAX),
    };
    let declared_context_default_decision = classify_raw_candidate_v8(&declared_context_control);
    let raw_surface_declaration_disconnect_exposed = matches!(
        declared_context_default_decision,
        RawCandidateDecisionV8::NamedTypedObstruction { .. }
    );
    let falsifiers = issue_falsifiers(&declared_context_control, &declaration);
    let falsifiers_replayed = falsifiers.v7_var3_replayed_as_local_not_ambient
        && falsifiers.v7_contextual_misclassification_rejected
        && falsifiers.wrong_motive_rejected
        && falsifiers.nonformable_motive_rejected
        && falsifiers.pathcon_rejected_as_charged
        && falsifiers.raw_candidate_without_declaration_remains_unclassified
        && falsifiers.projection_mutation_rejected
        && falsifiers.zero_credit_preserved;
    let forbidden_outputs = ContextualForbiddenOutputs {
        global_e4_v8_executed: false,
        pending_membership_verdicts_issued: false,
        e2b_executed: false,
        historical_totals_issued: false,
        fq2_evaluated: false,
        e5_f1_executed: false,
        bridge_or_fork_executed: false,
    };
    if !v7_witness_predecessor_unknown
        || !v7_witness_now_internal_by_guarded_inverse
        || v7_witness_contextual_use_count != 0
        || !declared_context_control_internal
        || !motive_typing_replayed
        || !instantiation_coherence_replayed
        || !raw_surface_declaration_disconnect_exposed
        || marginal_nu != 0
        || !falsifiers_replayed
        || !forbidden_outputs.all_withheld()
    {
        return Err(ContextualClassifierError::Invariant(
            "contextual successor disposition drifted".to_owned(),
        ));
    }
    let mut certificate = ContextualInternalityCertificate {
        schema: CONTEXTUAL_INTERNALITY_SCHEMA.to_owned(),
        date: CONTEXTUAL_INTERNALITY_DATE.to_owned(),
        source_bindings: source_bindings(),
        predecessor_global_e4_v7_digest,
        predecessor_global_e4_v7_source_bound: true,
        adopted_contextual_rule_v1: true,
        frozen_raw_telescope_carries_ambient_motives: false,
        v7_witness_predecessor_unknown,
        v7_witness_successor_decision,
        v7_witness_now_internal_by_guarded_inverse,
        v7_witness_contextual_use_count,
        declared_context_control,
        declared_context,
        declared_context_successor_decision,
        declared_context_control_internal,
        declared_context_default_decision,
        raw_surface_declaration_disconnect_exposed,
        falsifiers,
        marginal_nu,
        global_e4_v8_authorized: true,
        forbidden_outputs,
        outcome: "contextual_rule_replays_but_raw_surface_lacks_motive_declarations".to_owned(),
        permitted_conclusion: "The adopted contextual rule is replayable for an explicitly motive-declared open judgment at nu=0. The sealed v7 Var(3) is local-var-1 and instead closes by the existing weakening/erasure inverse theorem. A raw Telescope alone cannot receive contextual credit because it carries no Gamma motive declaration."
            .to_owned(),
        required_successor_action: "Run global E-4 v8 create-new. If an exact live-ambient raw candidate survives solely because its motive declaration is absent, retain F-A5 and version the raw candidate surface to carry an explicit, B15-formable ambient telescope before claiming exhaustion."
            .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> ContextualInternalityReplay {
    ContextualInternalityReplay {
        valid: false,
        v7_witness_resolved: false,
        declared_context_control_internal: false,
        motive_typing_replayed: false,
        instantiation_coherence_replayed: false,
        raw_surface_declaration_disconnect_exposed: false,
        falsifiers_replayed: false,
        marginal_nu: u32::MAX,
        global_e4_v8_authorized: false,
        forbidden_outputs_withheld: false,
        outcome: "contextual_internality_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &ContextualInternalityCertificate,
    expected: &ContextualInternalityCertificate,
) -> ContextualInternalityReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    ContextualInternalityReplay {
        valid: errors.is_empty(),
        v7_witness_resolved: certificate.v7_witness_now_internal_by_guarded_inverse,
        declared_context_control_internal: certificate.declared_context_control_internal,
        motive_typing_replayed: matches!(
            certificate.declared_context_successor_decision,
            RawCandidateDecisionV8::InternalContextual { .. }
        ),
        instantiation_coherence_replayed: matches!(
            &certificate.declared_context_successor_decision,
            RawCandidateDecisionV8::InternalContextual { certificate, .. }
                if certificate.instantiation_coherence_replayed
        ),
        raw_surface_declaration_disconnect_exposed: certificate
            .raw_surface_declaration_disconnect_exposed,
        falsifiers_replayed: certificate.falsifiers.v7_var3_replayed_as_local_not_ambient
            && certificate
                .falsifiers
                .v7_contextual_misclassification_rejected
            && certificate.falsifiers.wrong_motive_rejected
            && certificate.falsifiers.nonformable_motive_rejected
            && certificate.falsifiers.pathcon_rejected_as_charged
            && certificate
                .falsifiers
                .raw_candidate_without_declaration_remains_unclassified
            && certificate.falsifiers.projection_mutation_rejected
            && certificate.falsifiers.zero_credit_preserved,
        marginal_nu: certificate.marginal_nu,
        global_e4_v8_authorized: certificate.global_e4_v8_authorized,
        forbidden_outputs_withheld: certificate.forbidden_outputs.all_withheld(),
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_contextual_internality_certificate(
    certificate: &ContextualInternalityCertificate,
) -> ContextualInternalityReplay {
    let expected = match issue_contextual_internality_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    replay_against_expected(certificate, &expected)
}

pub fn replay_contextual_internality_json(json: &str) -> ContextualInternalityReplay {
    match serde_json::from_str::<ContextualInternalityCertificate>(json) {
        Ok(certificate) => replay_contextual_internality_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_contextual_internality_create_new(
    path: &Path,
) -> Result<ContextualInternalityReplay, ContextualClassifierError> {
    let certificate = issue_contextual_internality_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| ContextualClassifierError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| ContextualClassifierError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| ContextualClassifierError::Io(error.to_string()))?;
    let replay = replay_contextual_internality_certificate(&certificate);
    if !replay.valid {
        return Err(ContextualClassifierError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v7_witness_is_an_unused_context_weakening_image() {
        let decision = classify_raw_candidate_v8(&v7_guarded_application_candidate());
        let RawCandidateDecisionV8::InternalGuardedClosure { certificate, .. } = decision else {
            panic!("v7 witness must close by guarded inverse evidence")
        };
        assert!(certificate.base_candidate_internal);
        assert!(certificate.ambient_parameter_unused_in_every_clause);
        assert!(certificate.both_inverse_laws_replayed_per_clause);
        assert_eq!(certificate.marginal_nu, 0);
    }

    #[test]
    fn explicit_declaration_is_required_for_live_contextual_credit() {
        let signature = SealedSignature::genesis_del_h15();
        let candidate = declared_context_control_candidate();
        assert!(matches!(
            classify_raw_candidate_v8(&candidate),
            RawCandidateDecisionV8::NamedTypedObstruction { .. }
        ));
        let declaration = issue_ambient_context_declaration_token(
            &signature,
            &candidate,
            15,
            declared_context_control_motives(),
        )
        .expect("declaration");
        let decision = classify_raw_candidate_v8_with_declaration(&candidate, &declaration);
        let RawCandidateDecisionV8::InternalContextual { certificate, .. } = decision else {
            panic!("declared candidate must enter contextual Internal")
        };
        assert_eq!(certificate.live_contextual_clause_count, 1);
        assert_eq!(certificate.context_weakening_clause_count, 1);
        assert!(certificate.instantiation_coherence_replayed);
        assert_eq!(certificate.marginal_nu, 0);
    }

    #[test]
    fn certificate_replays_and_mutations_fail_closed() {
        let certificate = issue_contextual_internality_certificate().expect("certificate");
        assert!(replay_against_expected(&certificate, &certificate).valid);
        let mut mutations = Vec::new();
        let mut motive = certificate.clone();
        motive.declared_context_control_internal = false;
        mutations.push(motive);
        let mut raw = certificate.clone();
        raw.frozen_raw_telescope_carries_ambient_motives = true;
        mutations.push(raw);
        let mut credit = certificate.clone();
        credit.marginal_nu = 1;
        mutations.push(credit);
        let mut forbidden = certificate.clone();
        forbidden.forbidden_outputs.e2b_executed = true;
        mutations.push(forbidden);
        for mutation in mutations {
            assert!(!replay_against_expected(&mutation, &certificate).valid);
        }
    }
}
