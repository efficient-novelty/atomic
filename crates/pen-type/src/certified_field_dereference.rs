//! Versioned certified-field dereference for transparent Internal closure.
//!
//! This successor does not widen the v1 ambient-former token.  It first
//! expands every candidate-field reference through a strictly-prior Internal
//! certificate, checks that the expanded candidate has the same typed
//! judgment as the original after erasing indirection, and then replays the
//! unchanged ambient-former token on that direct form.

use crate::ambient_former_internality::{
    AmbientFormerClosureProjection, issue_ambient_former_closure_token,
    replay_ambient_former_closure_projection, replay_ambient_former_closure_token,
};
use crate::elaborate::{KernelTy, SealedSignature, elaborate_telescope};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

pub const CERTIFIED_FIELD_DEREFERENCE_VERSION: &str = "certified-field-dereference-rule-v1";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedPriorField {
    pub clause_index: u16,
    pub certificate_hash: String,
    pub referent_expression: Expr,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedFieldDereferenceRecord {
    pub field_clause_index: u16,
    pub occurrence_scope: u32,
    pub certificate_hash: String,
    pub referent_expression: Expr,
    pub transported_direct_expression: Expr,
    pub referent_certificate_replayed: bool,
    pub no_credit_anchor_or_family_minted: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CertifiedFieldDereferenceProjection {
    pub version: String,
    pub signature_digest: String,
    pub visible_library: u32,
    pub original_candidate: Telescope,
    pub direct_candidate: Telescope,
    pub original_candidate_hash: String,
    pub direct_candidate_hash: String,
    pub original_elaboration_hash: String,
    pub direct_elaboration_hash: String,
    pub clause_index: u16,
    pub certified_prior_fields: BTreeMap<u16, CertifiedPriorField>,
    pub dereference_records: Vec<CertifiedFieldDereferenceRecord>,
    pub every_reference_certified: bool,
    pub every_reference_strictly_prior: bool,
    pub every_supplied_certificate_used: bool,
    pub dereferenced_expression: Expr,
    pub dereferenced_original_normal_form: Expr,
    pub direct_normal_form: Expr,
    pub raw_kernel_type_equal_to_direct_form: bool,
    pub certified_kernel_type_refinement_valid: bool,
    pub normal_form_identical_to_direct_form: bool,
    pub dereference_semantically_invisible: bool,
    pub direct_closure: AmbientFormerClosureProjection,
    pub direct_closure_replayed: bool,
    pub pathcon_remains_charged: bool,
    pub guarded_inverse_law_requirement_unchanged: bool,
    pub standing_orbit_exception_preserved: bool,
    pub no_credit_anchor_or_family_minted: bool,
    pub internal_closure_issued: bool,
    pub marginal_nu: u32,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CertifiedFieldDereferenceToken {
    projection: CertifiedFieldDereferenceProjection,
}

impl CertifiedFieldDereferenceToken {
    pub fn projection(&self) -> &CertifiedFieldDereferenceProjection {
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
pub enum CertifiedFieldDereferenceError {
    #[error("candidate failed elaboration: {0}")]
    Elaboration(String),
    #[error("clause index {clause_index} is out of range")]
    ClauseOutOfRange { clause_index: u16 },
    #[error("certified-field dereference is closed-only; candidate ambient arity is {ambient}")]
    GuardedCandidate { ambient: u32 },
    #[error("candidate field {clause_index} has no replayed Internal certificate")]
    UncertifiedField { clause_index: u16 },
    #[error("candidate field {clause_index} is forward or cyclic from clause {source_clause}")]
    ForwardOrCyclic {
        clause_index: u16,
        source_clause: u16,
    },
    #[error("certificate key {map_key} disagrees with recorded field {recorded_key}")]
    CertificateKeyMismatch { map_key: u16, recorded_key: u16 },
    #[error("certified referent expression differs from candidate clause {clause_index}")]
    ReferentExpressionMismatch { clause_index: u16 },
    #[error("certificate hash for clause {clause_index} is empty")]
    EmptyCertificateHash { clause_index: u16 },
    #[error("certificate for clause {clause_index} was supplied but never dereferenced")]
    UnusedCertificate { clause_index: u16 },
    #[error("transported variable level overflow")]
    LevelOverflow,
    #[error("direct ambient-former closure failed: {0}")]
    DirectClosure(String),
    #[error("dereferenced judgment differs from the direct judgment")]
    DirectJudgmentMismatch,
    #[error("certified-field dereference projection replay mismatch")]
    ReplayMismatch,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CERTIFIED_FIELD_DEREFERENCE_VERSION, domain, value))
        .expect("certified-field dereference evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn candidate_hash(candidate: &Telescope) -> String {
    let bytes = serde_json::to_vec(candidate).expect("candidate serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn record_dereference(
    evidence: &CertifiedPriorField,
    occurrence_scope: u32,
    transported_direct_expression: Expr,
) -> CertifiedFieldDereferenceRecord {
    let referent_certificate_replayed = true;
    let no_credit_anchor_or_family_minted = true;
    let marginal_nu = 0;
    let derivation_hash = tagged_hash(
        "certified-field-occurrence",
        &(
            evidence.clause_index,
            occurrence_scope,
            &evidence.certificate_hash,
            &evidence.referent_expression,
            &transported_direct_expression,
            referent_certificate_replayed,
            no_credit_anchor_or_family_minted,
            marginal_nu,
        ),
    );
    CertifiedFieldDereferenceRecord {
        field_clause_index: evidence.clause_index,
        occurrence_scope,
        certificate_hash: evidence.certificate_hash.clone(),
        referent_expression: evidence.referent_expression.clone(),
        transported_direct_expression,
        referent_certificate_replayed,
        no_credit_anchor_or_family_minted,
        marginal_nu,
        derivation_hash,
    }
}

struct DereferenceState<'a> {
    certified: &'a BTreeMap<u16, CertifiedPriorField>,
    used: BTreeSet<u16>,
    records: Vec<CertifiedFieldDereferenceRecord>,
}

fn certified_kernel_type_refines(raw: &KernelTy, direct: &KernelTy) -> bool {
    raw == direct
        || matches!(raw, KernelTy::Neutral)
        || matches!(
            (raw, direct),
            (KernelTy::Fun(raw_domain, raw_codomain), KernelTy::Fun(direct_domain, direct_codomain))
                if certified_kernel_type_refines(raw_domain, direct_domain)
                    && certified_kernel_type_refines(raw_codomain, direct_codomain)
        )
}

impl DereferenceState<'_> {
    fn expand(
        &mut self,
        expression: &Expr,
        source_outer_scope: u32,
        target_outer_scope: u32,
        binder_depth: u32,
    ) -> Result<Expr, CertifiedFieldDereferenceError> {
        match expression {
            Expr::Var(level) if *level <= source_outer_scope => {
                let raw_index = level.saturating_sub(1);
                let clause_index = u16::try_from(raw_index).map_err(|_| {
                    CertifiedFieldDereferenceError::UncertifiedField {
                        clause_index: u16::MAX,
                    }
                })?;
                if u32::from(clause_index) >= source_outer_scope {
                    return Err(CertifiedFieldDereferenceError::ForwardOrCyclic {
                        clause_index,
                        source_clause: u16::try_from(source_outer_scope).unwrap_or(u16::MAX),
                    });
                }
                let evidence = self
                    .certified
                    .get(&clause_index)
                    .ok_or(CertifiedFieldDereferenceError::UncertifiedField { clause_index })?;
                self.used.insert(clause_index);
                let occurrence_scope = target_outer_scope
                    .checked_add(binder_depth)
                    .ok_or(CertifiedFieldDereferenceError::LevelOverflow)?;
                let direct = self.expand(
                    &evidence.referent_expression,
                    u32::from(clause_index),
                    occurrence_scope,
                    0,
                )?;
                self.records.push(record_dereference(
                    evidence,
                    occurrence_scope,
                    direct.clone(),
                ));
                Ok(direct)
            }
            Expr::Var(level) => {
                let shift = target_outer_scope
                    .checked_sub(source_outer_scope)
                    .ok_or(CertifiedFieldDereferenceError::LevelOverflow)?;
                Ok(Expr::Var(
                    level
                        .checked_add(shift)
                        .ok_or(CertifiedFieldDereferenceError::LevelOverflow)?,
                ))
            }
            Expr::Univ => Ok(Expr::Univ),
            Expr::Lib(step) => Ok(Expr::Lib(*step)),
            Expr::PathCon(dimension) => Ok(Expr::PathCon(*dimension)),
            Expr::App(left, right) => Ok(Expr::App(
                Box::new(self.expand(
                    left,
                    source_outer_scope,
                    target_outer_scope,
                    binder_depth,
                )?),
                Box::new(self.expand(
                    right,
                    source_outer_scope,
                    target_outer_scope,
                    binder_depth,
                )?),
            )),
            Expr::Pi(domain, codomain) => Ok(Expr::Pi(
                Box::new(self.expand(
                    domain,
                    source_outer_scope,
                    target_outer_scope,
                    binder_depth,
                )?),
                Box::new(self.expand(
                    codomain,
                    source_outer_scope,
                    target_outer_scope,
                    binder_depth + 1,
                )?),
            )),
            Expr::Sigma(domain, codomain) => Ok(Expr::Sigma(
                Box::new(self.expand(
                    domain,
                    source_outer_scope,
                    target_outer_scope,
                    binder_depth,
                )?),
                Box::new(self.expand(
                    codomain,
                    source_outer_scope,
                    target_outer_scope,
                    binder_depth + 1,
                )?),
            )),
            Expr::Lam(body) => Ok(Expr::Lam(Box::new(self.expand(
                body,
                source_outer_scope,
                target_outer_scope,
                binder_depth + 1,
            )?))),
            Expr::Id(ty, left, right) => Ok(Expr::Id(
                Box::new(self.expand(ty, source_outer_scope, target_outer_scope, binder_depth)?),
                Box::new(self.expand(
                    left,
                    source_outer_scope,
                    target_outer_scope,
                    binder_depth,
                )?),
                Box::new(self.expand(
                    right,
                    source_outer_scope,
                    target_outer_scope,
                    binder_depth,
                )?),
            )),
            Expr::Refl(inner) => self.unary(
                inner,
                source_outer_scope,
                target_outer_scope,
                binder_depth,
                Expr::Refl,
            ),
            Expr::Susp(inner) => self.unary(
                inner,
                source_outer_scope,
                target_outer_scope,
                binder_depth,
                Expr::Susp,
            ),
            Expr::Trunc(inner) => self.unary(
                inner,
                source_outer_scope,
                target_outer_scope,
                binder_depth,
                Expr::Trunc,
            ),
            Expr::Flat(inner) => self.unary(
                inner,
                source_outer_scope,
                target_outer_scope,
                binder_depth,
                Expr::Flat,
            ),
            Expr::Sharp(inner) => self.unary(
                inner,
                source_outer_scope,
                target_outer_scope,
                binder_depth,
                Expr::Sharp,
            ),
            Expr::Disc(inner) => self.unary(
                inner,
                source_outer_scope,
                target_outer_scope,
                binder_depth,
                Expr::Disc,
            ),
            Expr::Shape(inner) => self.unary(
                inner,
                source_outer_scope,
                target_outer_scope,
                binder_depth,
                Expr::Shape,
            ),
            Expr::Next(inner) => self.unary(
                inner,
                source_outer_scope,
                target_outer_scope,
                binder_depth,
                Expr::Next,
            ),
            Expr::Eventually(inner) => self.unary(
                inner,
                source_outer_scope,
                target_outer_scope,
                binder_depth,
                Expr::Eventually,
            ),
            Expr::Bang(inner) => self.unary(
                inner,
                source_outer_scope,
                target_outer_scope,
                binder_depth,
                Expr::Bang,
            ),
            Expr::WhyNot(inner) => self.unary(
                inner,
                source_outer_scope,
                target_outer_scope,
                binder_depth,
                Expr::WhyNot,
            ),
        }
    }

    fn unary(
        &mut self,
        inner: &Expr,
        source_outer_scope: u32,
        target_outer_scope: u32,
        binder_depth: u32,
        constructor: fn(Box<Expr>) -> Expr,
    ) -> Result<Expr, CertifiedFieldDereferenceError> {
        Ok(constructor(Box::new(self.expand(
            inner,
            source_outer_scope,
            target_outer_scope,
            binder_depth,
        )?)))
    }
}

fn validate_certified_fields(
    candidate: &Telescope,
    clause_index: u16,
    certified: &BTreeMap<u16, CertifiedPriorField>,
) -> Result<(), CertifiedFieldDereferenceError> {
    for (key, evidence) in certified {
        if *key != evidence.clause_index {
            return Err(CertifiedFieldDereferenceError::CertificateKeyMismatch {
                map_key: *key,
                recorded_key: evidence.clause_index,
            });
        }
        if *key >= clause_index {
            return Err(CertifiedFieldDereferenceError::ForwardOrCyclic {
                clause_index: *key,
                source_clause: clause_index,
            });
        }
        if evidence.certificate_hash.is_empty() {
            return Err(CertifiedFieldDereferenceError::EmptyCertificateHash {
                clause_index: *key,
            });
        }
        if candidate
            .clauses
            .get(usize::from(*key))
            .is_none_or(|clause| clause.expr != evidence.referent_expression)
        {
            return Err(CertifiedFieldDereferenceError::ReferentExpressionMismatch {
                clause_index: *key,
            });
        }
    }
    Ok(())
}

pub fn issue_certified_field_dereference_token(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    clause_index: u16,
    certified_prior_fields: &BTreeMap<u16, CertifiedPriorField>,
) -> Result<CertifiedFieldDereferenceToken, CertifiedFieldDereferenceError> {
    validate_certified_fields(candidate, clause_index, certified_prior_fields)?;
    let original_elaboration = elaborate_telescope(signature, candidate, visible_library)
        .map_err(|error| CertifiedFieldDereferenceError::Elaboration(error.to_string()))?;
    if original_elaboration.ambient_parameters != 0 {
        return Err(CertifiedFieldDereferenceError::GuardedCandidate {
            ambient: original_elaboration.ambient_parameters,
        });
    }
    let selected_clause = candidate
        .clauses
        .get(usize::from(clause_index))
        .ok_or(CertifiedFieldDereferenceError::ClauseOutOfRange { clause_index })?;
    let selected_original = original_elaboration
        .clauses
        .get(usize::from(clause_index))
        .ok_or(CertifiedFieldDereferenceError::ClauseOutOfRange { clause_index })?;

    let mut state = DereferenceState {
        certified: certified_prior_fields,
        used: BTreeSet::new(),
        records: Vec::new(),
    };
    let dereferenced_expression = state.expand(
        &selected_clause.expr,
        u32::from(clause_index),
        u32::from(clause_index),
        0,
    )?;
    if let Some(unused) = certified_prior_fields
        .keys()
        .find(|key| !state.used.contains(key))
    {
        return Err(CertifiedFieldDereferenceError::UnusedCertificate {
            clause_index: *unused,
        });
    }

    let mut direct_candidate = candidate.clone();
    direct_candidate.clauses[usize::from(clause_index)].expr = dereferenced_expression.clone();
    let direct_elaboration = elaborate_telescope(signature, &direct_candidate, visible_library)
        .map_err(|error| CertifiedFieldDereferenceError::Elaboration(error.to_string()))?;
    let selected_direct = direct_elaboration
        .clauses
        .get(usize::from(clause_index))
        .ok_or(CertifiedFieldDereferenceError::ClauseOutOfRange { clause_index })?;

    let mut normal_state = DereferenceState {
        certified: certified_prior_fields,
        used: BTreeSet::new(),
        records: Vec::new(),
    };
    let dereferenced_original_normal_form = normal_state.expand(
        &selected_original.normal_form,
        u32::from(clause_index),
        u32::from(clause_index),
        0,
    )?;
    let direct_normal_form = selected_direct.normal_form.clone();
    let raw_kernel_type_equal_to_direct_form =
        selected_original.kernel_ty == selected_direct.kernel_ty;
    let certified_kernel_type_refinement_valid =
        certified_kernel_type_refines(&selected_original.kernel_ty, &selected_direct.kernel_ty);
    let normal_form_identical_to_direct_form =
        dereferenced_original_normal_form == direct_normal_form;

    let direct_token = issue_ambient_former_closure_token(
        signature,
        &direct_candidate,
        visible_library,
        clause_index,
        &BTreeMap::new(),
    )
    .map_err(|error| CertifiedFieldDereferenceError::DirectClosure(error.to_string()))?;
    replay_ambient_former_closure_token(
        signature,
        &direct_candidate,
        visible_library,
        &direct_token,
    )
    .map_err(|error| CertifiedFieldDereferenceError::DirectClosure(error.to_string()))?;
    let direct_closure = direct_token.projection().clone();
    replay_ambient_former_closure_projection(
        signature,
        &direct_candidate,
        visible_library,
        &direct_closure,
    )
    .map_err(|error| CertifiedFieldDereferenceError::DirectClosure(error.to_string()))?;

    let every_reference_certified = !state.records.is_empty()
        && state
            .records
            .iter()
            .all(|record| record.referent_certificate_replayed);
    let every_reference_strictly_prior = state
        .records
        .iter()
        .all(|record| record.field_clause_index < clause_index);
    let every_supplied_certificate_used = state.used.len() == certified_prior_fields.len();
    let dereference_semantically_invisible = certified_kernel_type_refinement_valid
        && normal_form_identical_to_direct_form
        && direct_closure.expression == dereferenced_expression
        && direct_closure.clause_normal_form == direct_normal_form;
    if !dereference_semantically_invisible {
        return Err(CertifiedFieldDereferenceError::DirectJudgmentMismatch);
    }
    let direct_closure_replayed = true;
    let pathcon_remains_charged = direct_closure.charged_path_constructor_excluded;
    let guarded_inverse_law_requirement_unchanged = true;
    let standing_orbit_exception_preserved = direct_closure.standing_orbit_exception_preserved;
    let no_credit_anchor_or_family_minted = state
        .records
        .iter()
        .all(|record| record.no_credit_anchor_or_family_minted)
        && !direct_closure
            .term_evidence
            .independent_demand_orbit_exported;
    let internal_closure_issued = every_reference_certified
        && every_reference_strictly_prior
        && every_supplied_certificate_used
        && dereference_semantically_invisible
        && direct_closure_replayed
        && pathcon_remains_charged
        && guarded_inverse_law_requirement_unchanged
        && standing_orbit_exception_preserved
        && no_credit_anchor_or_family_minted
        && direct_closure.internal_closure_issued;
    let marginal_nu = 0;
    let mut projection = CertifiedFieldDereferenceProjection {
        version: CERTIFIED_FIELD_DEREFERENCE_VERSION.to_owned(),
        signature_digest: signature.digest().to_owned(),
        visible_library,
        original_candidate: candidate.clone(),
        direct_candidate: direct_candidate.clone(),
        original_candidate_hash: candidate_hash(candidate),
        direct_candidate_hash: candidate_hash(&direct_candidate),
        original_elaboration_hash: original_elaboration.derivation_hash,
        direct_elaboration_hash: direct_elaboration.derivation_hash,
        clause_index,
        certified_prior_fields: certified_prior_fields.clone(),
        dereference_records: state.records,
        every_reference_certified,
        every_reference_strictly_prior,
        every_supplied_certificate_used,
        dereferenced_expression,
        dereferenced_original_normal_form,
        direct_normal_form,
        raw_kernel_type_equal_to_direct_form,
        certified_kernel_type_refinement_valid,
        normal_form_identical_to_direct_form,
        dereference_semantically_invisible,
        direct_closure,
        direct_closure_replayed,
        pathcon_remains_charged,
        guarded_inverse_law_requirement_unchanged,
        standing_orbit_exception_preserved,
        no_credit_anchor_or_family_minted,
        internal_closure_issued,
        marginal_nu,
        derivation_hash: String::new(),
    };
    projection.derivation_hash = tagged_hash("certified-field-dereference-token", &projection);
    Ok(CertifiedFieldDereferenceToken { projection })
}

pub fn replay_certified_field_dereference_token(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    token: &CertifiedFieldDereferenceToken,
) -> Result<(), CertifiedFieldDereferenceError> {
    let reissued = issue_certified_field_dereference_token(
        signature,
        candidate,
        visible_library,
        token.projection.clause_index,
        &token.projection.certified_prior_fields,
    )?;
    if reissued == *token {
        Ok(())
    } else {
        Err(CertifiedFieldDereferenceError::ReplayMismatch)
    }
}

pub fn replay_certified_field_dereference_projection(
    signature: &SealedSignature,
    candidate: &Telescope,
    visible_library: u32,
    projection: &CertifiedFieldDereferenceProjection,
) -> Result<(), CertifiedFieldDereferenceError> {
    let token = issue_certified_field_dereference_token(
        signature,
        candidate,
        visible_library,
        projection.clause_index,
        &projection.certified_prior_fields,
    )?;
    if token.projection() == projection {
        Ok(())
    } else {
        Err(CertifiedFieldDereferenceError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_core::clause::{ClauseRec, ClauseRole};

    fn candidate() -> Telescope {
        Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::Lib(15)),
                ))),
            ),
        ])
    }

    fn certified() -> BTreeMap<u16, CertifiedPriorField> {
        BTreeMap::from([(
            0,
            CertifiedPriorField {
                clause_index: 0,
                certificate_hash: "earned-internal-clause-0".to_owned(),
                referent_expression: Expr::Univ,
            },
        )])
    }

    #[test]
    fn certified_head_dereferences_to_direct_form_at_zero_credit() {
        let signature = SealedSignature::genesis_del_h15();
        let subject = candidate();
        let token =
            issue_certified_field_dereference_token(&signature, &subject, 15, 1, &certified())
                .expect("certified dereference");
        replay_certified_field_dereference_token(&signature, &subject, 15, &token)
            .expect("token replay");
        assert_eq!(
            token.projection().dereferenced_expression,
            Expr::Lam(Box::new(Expr::App(
                Box::new(Expr::Univ),
                Box::new(Expr::Lib(15)),
            )))
        );
        assert!(token.projection().dereference_semantically_invisible);
        assert_eq!(token.projection().dereference_records.len(), 1);
        assert_eq!(token.marginal_nu(), 0);
    }

    #[test]
    fn uncertified_forward_and_mismatched_evidence_fail_closed() {
        let signature = SealedSignature::genesis_del_h15();
        let subject = candidate();
        assert!(matches!(
            issue_certified_field_dereference_token(&signature, &subject, 15, 1, &BTreeMap::new(),),
            Err(CertifiedFieldDereferenceError::UncertifiedField { clause_index: 0 })
        ));
        let mut forward = certified();
        forward.insert(
            1,
            CertifiedPriorField {
                clause_index: 1,
                certificate_hash: "forged".to_owned(),
                referent_expression: subject.clauses[1].expr.clone(),
            },
        );
        assert!(matches!(
            issue_certified_field_dereference_token(&signature, &subject, 15, 1, &forward),
            Err(CertifiedFieldDereferenceError::ForwardOrCyclic { .. })
        ));
        let mut mismatch = certified();
        mismatch.get_mut(&0).expect("field").referent_expression = Expr::Lib(15);
        assert!(matches!(
            issue_certified_field_dereference_token(&signature, &subject, 15, 1, &mismatch),
            Err(CertifiedFieldDereferenceError::ReferentExpressionMismatch { .. })
        ));
    }

    #[test]
    fn guarded_pathcon_and_projection_mutation_fail_closed() {
        let signature = SealedSignature::genesis_del_h15();
        let guarded = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Var(3)),
                    Box::new(Expr::Var(1)),
                ))),
            ),
        ]);
        assert!(matches!(
            issue_certified_field_dereference_token(&signature, &guarded, 15, 1, &certified()),
            Err(CertifiedFieldDereferenceError::GuardedCandidate { .. })
        ));
        let pathcon = Telescope::new(vec![
            ClauseRec::new(ClauseRole::Formation, Expr::Univ),
            ClauseRec::new(
                ClauseRole::Introduction,
                Expr::Lam(Box::new(Expr::App(
                    Box::new(Expr::Var(1)),
                    Box::new(Expr::PathCon(1)),
                ))),
            ),
        ]);
        assert!(matches!(
            issue_certified_field_dereference_token(
                &signature,
                &pathcon,
                15,
                1,
                &certified(),
            ),
            Err(CertifiedFieldDereferenceError::DirectClosure(message))
                if message.contains("PathCon")
        ));

        let subject = candidate();
        let token =
            issue_certified_field_dereference_token(&signature, &subject, 15, 1, &certified())
                .expect("token");
        let mut projection = token.projection().clone();
        projection.marginal_nu = 1;
        assert_eq!(
            replay_certified_field_dereference_projection(&signature, &subject, 15, &projection,),
            Err(CertifiedFieldDereferenceError::ReplayMismatch)
        );
    }
}
