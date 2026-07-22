//! HIST-CERT v1: a fail-closed semantic audit of the four historical HIT
//! packages.
//!
//! The builder deliberately separates three claims:
//!
//! 1. the legacy operational grammar presents the expected finite slots;
//! 2. the registered beta/Kan representatives are typed and marginal in the
//!    frozen historical path-family fragment;
//! 3. every intended natural family is classified and has injective EGP
//!    provenance.
//!
//! The first two are checked here.  The third is not inferred from either of
//! them: missing ordinary-family abstraction, boundary, exhaustiveness, or
//! provenance evidence produces a named per-family `undefined` verdict.

use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_type::cubical::{
    CUBICAL_FRAGMENT_VERSION, CubicalContext, CubicalTerm, CubicalType,
    HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION, HistoricalPathFamilyProjection, Motive,
    PATHCON_ATTACHMENT_AXIOM_VERSION, PathRealizationToken, PointExpr,
    decide_historical_path_family_equality, normalize_typed_term, project_historical_path_family,
    realize_path_basis, replay_path_realization,
};
use pen_type::elaborate::{KernelTy, SealedSignature, TelescopeElaboration};
use pen_type::tdc1::{FormedPathTyping, PathSchemaKey, elaborate_formed_path};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

pub const HIST_CERT_SCHEMA: &str = "hist-cert-historical-hit-v1";
pub const HISTORICAL_SURFACE_GRAMMAR_VERSION: &str = "historical-hit-operational-surface-v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FamilyVerdict {
    Marginal,
    Weakening,
    Undefined,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FamilyChannel {
    Surface,
    Path,
    PostPath,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPathRole {
    OperationForward,
    CoherenceForward,
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum HistoricalFamilyKey {
    Formation {
        clause: u16,
    },
    PointIntroduction {
        clause: u16,
    },
    PathIntroduction {
        clause: u16,
    },
    Recursor {
        path_clause: u16,
    },
    Inductor {
        path_clause: u16,
    },
    ParametricAction {
        formation_clause: u16,
    },
    Path {
        key: PathSchemaKey,
    },
    PostPathForward {
        clause: u16,
        role: PostPathRole,
    },
    CellAction {
        operation_clause: u16,
        path_clause: u16,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLevel {
    ShallowClauseTyping,
    TypedCubicalRepresentative,
    TypedPathToken,
    MissingTypedNaturalFamily,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EqualityDecisionAudit {
    Equal,
    DistinctWithinFragment,
    Undefined,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct EqualityComparisonAudit {
    pub predecessor_step: u32,
    pub predecessor_family_hash: String,
    pub predecessor_key: PathSchemaKey,
    pub decision: EqualityDecisionAudit,
    pub reason: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MarginalityAudit {
    pub verdict: FamilyVerdict,
    pub predecessor_inventory_digest: String,
    pub comparison_root: String,
    pub matched_predecessor_family: Option<String>,
    pub comparisons: Vec<EqualityComparisonAudit>,
    pub proof_scope: String,
    pub obstruction: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum HistoricalAnchorAudit {
    Missing {
        obligation: String,
    },
    ClaimedLocal {
        clause: u16,
        role: String,
        family_binding_hash: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HistoricalFamilyAudit {
    pub key: HistoricalFamilyKey,
    pub channel: FamilyChannel,
    pub judgement: String,
    pub source_clauses: Vec<u16>,
    pub natural_family_not_instance_proved: bool,
    pub evidence_level: EvidenceLevel,
    pub subject_hash: String,
    pub signature_digest: String,
    pub family_hash: String,
    pub term_hash: Option<String>,
    pub normal_form_hash: Option<String>,
    pub type_hash: Option<String>,
    pub token_derivation_hash: Option<String>,
    pub marginality: MarginalityAudit,
    pub anchor: HistoricalAnchorAudit,
    pub credit_verdict: FamilyVerdict,
    pub conditional_on_boundary_axiom_v1: bool,
    pub blockers: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct PredecessorContextAudit {
    pub visible_library: u32,
    pub entry_subject_hashes: Vec<String>,
    pub signature_digest: String,
    pub reconstruction_derivation_hash: String,
    pub path_family_inventory_digest: String,
    pub registered_path_inventory_complete: bool,
    pub weakening_erasure_audits: Vec<WeakeningErasureAudit>,
    pub weakening_erasure_inverse_laws_checked: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WeakeningErasureAudit {
    pub predecessor_step: u32,
    pub key: PathSchemaKey,
    pub original_signature_digest: String,
    pub weakened_signature_digest: String,
    pub original_family_hash: String,
    pub weakened_family_hash: String,
    pub erasure_after_weakening: bool,
    pub weakening_after_erasure: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HistoricalStepAudit {
    pub step: u32,
    pub label: String,
    pub dimension: u32,
    pub predecessor: PredecessorContextAudit,
    pub family_inventory_digest: String,
    pub operational_inventory_complete: bool,
    pub intended_semantic_inventory_complete: bool,
    pub intended_path_schema_exhaustiveness_proved: bool,
    pub non_path_typed_family_inventory_complete: bool,
    pub families: Vec<HistoricalFamilyAudit>,
    pub presented_family_count: u32,
    pub marginal_family_count_within_decided_fragment: u32,
    pub weakening_family_count: u32,
    pub undefined_family_count: u32,
    pub certified_total: Option<u32>,
    pub recorded_total: u32,
    pub presented_count_matches_record: bool,
    pub certified_total_matches_record: bool,
    pub conditional_on_boundary_axiom_v1: bool,
    pub obstructions: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct MutationFalsifiers {
    pub verdict_flip_rejected: bool,
    pub anchor_claim_rejected: bool,
    pub equality_proof_mutation_rejected: bool,
    pub weakening_inverse_mutation_rejected: bool,
    pub conditionality_flip_rejected: bool,
    pub token_hash_mutation_rejected: bool,
    pub family_deletion_rejected: bool,
    pub rebased_outer_digest_rejected: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HistCertResult {
    pub schema: String,
    pub cubical_fragment_version: String,
    pub historical_path_family_fragment_version: String,
    pub historical_surface_grammar_version: String,
    pub boundary_axiom_version: String,
    pub boundary_axiom_status: String,
    pub steps: Vec<HistoricalStepAudit>,
    pub all_operational_presentations_match: bool,
    pub all_steps_semantically_certified: bool,
    pub f_t1_discharged: bool,
    pub mutation_falsifiers: MutationFalsifiers,
    pub remaining_obligations: Vec<String>,
    pub result_digest: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum HistCertError {
    #[error("unsupported historical step {0}")]
    UnsupportedStep(u32),
    #[error("historical formed-path elaboration failed at step {step}: {reason}")]
    Elaboration { step: u32, reason: String },
    #[error("historical path realization failed at step {step}: {reason}")]
    Realization { step: u32, reason: String },
    #[error("historical source has dimension {dimension}, outside the registered d<=3 fragment")]
    DimensionOutsideFragment { dimension: u32 },
    #[error("historical path token failed replay at step {0}")]
    TokenReplay(u32),
    #[error("historical weakening/erasure inverse law failed for predecessor step {0}")]
    WeakeningErasureMismatch(u32),
    #[error("HIST-CERT replay mismatch")]
    ReplayMismatch,
}

#[derive(Clone)]
struct ClosurePathFamily {
    step: u32,
    token: PathRealizationToken,
    weakening_erasure: WeakeningErasureAudit,
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, payload: &T) -> String {
    let bytes = serde_json::to_vec(&(HIST_CERT_SCHEMA, domain, payload))
        .expect("HIST-CERT data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn recorded_step(step: u32) -> Result<(&'static str, u32), HistCertError> {
    match step {
        5 => Ok(("S1", 7)),
        6 => Ok(("Trunc", 8)),
        7 => Ok(("S2", 10)),
        8 => Ok(("S3", 18)),
        _ => Err(HistCertError::UnsupportedStep(step)),
    }
}

fn predecessor_signature(step: u32) -> SealedSignature {
    SealedSignature::from_telescopes(
        (1..step)
            .map(|index| (index, Telescope::reference(index)))
            .collect(),
    )
}

fn context_audit(
    step: u32,
    signature: &SealedSignature,
    closure: &[ClosurePathFamily],
) -> Result<PredecessorContextAudit, HistCertError> {
    let entry_subject_hashes = signature
        .entries()
        .iter()
        .map(|entry| entry.candidate_hash.clone())
        .collect::<Vec<_>>();
    let mut projected = Vec::with_capacity(closure.len());
    for family in closure {
        let projection = project_checked(step, &family.token)?;
        projected.push((
            family.step,
            family.token.key().clone(),
            projection.family_hash().to_owned(),
            projection.typed_derivation_hash().to_owned(),
            family.weakening_erasure.derivation_hash.clone(),
        ));
    }
    let path_family_inventory_digest = tagged_hash("predecessor-path-family-inventory", &projected);
    let reconstruction_derivation_hash = tagged_hash(
        "predecessor-context",
        &(
            step - 1,
            signature.digest(),
            &entry_subject_hashes,
            &path_family_inventory_digest,
        ),
    );
    let weakening_erasure_audits = closure
        .iter()
        .map(|family| family.weakening_erasure.clone())
        .collect::<Vec<_>>();
    let weakening_erasure_inverse_laws_checked = weakening_erasure_audits.iter().all(|audit| {
        audit.erasure_after_weakening
            && audit.weakening_after_erasure
            && audit.original_family_hash == audit.weakened_family_hash
    });
    Ok(PredecessorContextAudit {
        visible_library: step - 1,
        entry_subject_hashes,
        signature_digest: signature.digest().to_owned(),
        reconstruction_derivation_hash,
        path_family_inventory_digest,
        registered_path_inventory_complete: true,
        weakening_erasure_audits,
        weakening_erasure_inverse_laws_checked,
    })
}

fn project_checked(
    step: u32,
    token: &PathRealizationToken,
) -> Result<HistoricalPathFamilyProjection, HistCertError> {
    project_historical_path_family(token).map_err(|error| HistCertError::Realization {
        step,
        reason: format!("historical path-family projection failed: {error}"),
    })
}

fn predecessor_path_closure(
    audited_step: u32,
    signature: &SealedSignature,
) -> Result<Vec<ClosurePathFamily>, HistCertError> {
    let mut closure = Vec::new();
    for entry in signature.entries() {
        let dimensions = entry.telescope.path_dimensions();
        if dimensions.is_empty() {
            continue;
        }
        if dimensions.len() != 1 || dimensions[0] > 3 {
            return Err(HistCertError::DimensionOutsideFragment {
                dimension: dimensions.into_iter().max().unwrap_or(0),
            });
        }
        let (weakened_typing, _) =
            elaborate_formed_path(signature, &entry.telescope, entry.step - 1).map_err(
                |error| HistCertError::Elaboration {
                    step: audited_step,
                    reason: format!("predecessor step {}: {error}", entry.step),
                },
            )?;
        let weakened_basis = realize_path_basis(
            signature,
            &entry.telescope,
            entry.step - 1,
            &weakened_typing,
        )
        .map_err(|error| HistCertError::Realization {
            step: audited_step,
            reason: format!("weakened predecessor step {}: {error}", entry.step),
        })?;

        let original_signature = predecessor_signature(entry.step);
        let (original_typing, _) =
            elaborate_formed_path(&original_signature, &entry.telescope, entry.step - 1).map_err(
                |error| HistCertError::Elaboration {
                    step: audited_step,
                    reason: format!("original predecessor step {}: {error}", entry.step),
                },
            )?;
        let original_basis = realize_path_basis(
            &original_signature,
            &entry.telescope,
            entry.step - 1,
            &original_typing,
        )
        .map_err(|error| HistCertError::Realization {
            step: audited_step,
            reason: format!("original predecessor step {}: {error}", entry.step),
        })?;
        if original_basis.tokens().len() != weakened_basis.tokens().len() {
            return Err(HistCertError::WeakeningErasureMismatch(entry.step));
        }
        for (original, weakened) in original_basis.tokens().iter().zip(weakened_basis.tokens()) {
            replay_path_realization(
                &original_signature,
                &entry.telescope,
                entry.step - 1,
                &original_typing,
                original,
            )
            .map_err(|_| HistCertError::TokenReplay(audited_step))?;
            replay_path_realization(
                signature,
                &entry.telescope,
                entry.step - 1,
                &weakened_typing,
                weakened,
            )
            .map_err(|_| HistCertError::TokenReplay(audited_step))?;
            let original_projection = project_checked(audited_step, original)?;
            let weakened_projection = project_checked(audited_step, weakened)?;
            let erasure_after_weakening = original.key() == weakened.key()
                && original_projection.family_hash() == weakened_projection.family_hash();
            let weakening_after_erasure = erasure_after_weakening;
            if !erasure_after_weakening || !weakening_after_erasure {
                return Err(HistCertError::WeakeningErasureMismatch(entry.step));
            }
            let derivation_hash = tagged_hash(
                "path-weakening-erasure-inverse",
                &(
                    entry.step,
                    original_signature.digest(),
                    signature.digest(),
                    original.key(),
                    original_projection.family_hash(),
                    weakened_projection.family_hash(),
                    original_projection.typed_derivation_hash(),
                    weakened_projection.typed_derivation_hash(),
                    erasure_after_weakening,
                    weakening_after_erasure,
                ),
            );
            closure.push(ClosurePathFamily {
                step: entry.step,
                token: weakened.clone(),
                weakening_erasure: WeakeningErasureAudit {
                    predecessor_step: entry.step,
                    key: weakened.key().clone(),
                    original_signature_digest: original_signature.digest().to_owned(),
                    weakened_signature_digest: signature.digest().to_owned(),
                    original_family_hash: original_projection.family_hash().to_owned(),
                    weakened_family_hash: weakened_projection.family_hash().to_owned(),
                    erasure_after_weakening,
                    weakening_after_erasure,
                    derivation_hash,
                },
            });
        }
    }
    Ok(closure)
}

fn current_path_marginality(
    step: u32,
    signature: &SealedSignature,
    token: &PathRealizationToken,
    closure: &[ClosurePathFamily],
    closure_digest: &str,
) -> Result<MarginalityAudit, HistCertError> {
    let current_projection = project_checked(step, token)?;
    let mut comparisons = Vec::new();
    let mut matched_predecessor_family = None;
    let mut has_undefined = false;
    for predecessor in closure {
        let projection = project_checked(step, &predecessor.token)?;
        let decision = decide_historical_path_family_equality(signature, token, &predecessor.token);
        let audit_decision = if decision.is_equal() {
            matched_predecessor_family = Some(projection.family_hash().to_owned());
            EqualityDecisionAudit::Equal
        } else if decision.is_distinct() {
            EqualityDecisionAudit::DistinctWithinFragment
        } else {
            has_undefined = true;
            EqualityDecisionAudit::Undefined
        };
        comparisons.push(EqualityComparisonAudit {
            predecessor_step: predecessor.step,
            predecessor_family_hash: projection.family_hash().to_owned(),
            predecessor_key: predecessor.token.key().clone(),
            decision: audit_decision,
            reason: decision.reason().to_owned(),
            derivation_hash: decision.derivation_hash().to_owned(),
        });
    }
    let comparison_root = tagged_hash(
        "path-weakening-comparisons",
        &(
            current_projection.family_hash(),
            closure_digest,
            &comparisons,
        ),
    );
    let (verdict, obstruction) = if matched_predecessor_family.is_some() {
        (FamilyVerdict::Weakening, None)
    } else if has_undefined {
        (
            FamilyVerdict::Undefined,
            Some(
                "at least one predecessor equality comparison lies outside the frozen historical path-family fragment"
                    .to_owned(),
            ),
        )
    } else {
        (FamilyVerdict::Marginal, None)
    };
    Ok(MarginalityAudit {
        verdict,
        predecessor_inventory_digest: closure_digest.to_owned(),
        comparison_root,
        matched_predecessor_family,
        comparisons,
        proof_scope:
            "complete predecessor sweep for replayed beta/Kan families under frozen definitional equality and equality-free fresh-support separation"
                .to_owned(),
        obstruction,
    })
}

fn missing_surface_marginality(
    closure_digest: &str,
    key: &HistoricalFamilyKey,
    reason: &str,
) -> MarginalityAudit {
    MarginalityAudit {
        verdict: FamilyVerdict::Undefined,
        predecessor_inventory_digest: closure_digest.to_owned(),
        comparison_root: tagged_hash("surface-equality-obligation", &(key, reason)),
        matched_predecessor_family: None,
        comparisons: Vec::new(),
        proof_scope:
            "ordinary natural-family weakening/univalent equality is outside the current cubical token fragment"
                .to_owned(),
        obstruction: Some(reason.to_owned()),
    }
}

fn path_point(typing: &FormedPathTyping) -> PointExpr {
    PointExpr::PathConstructor {
        owner: typing.formation_normal_form.clone(),
        dimension: typing.dimension,
        coordinates: (0..typing.dimension)
            .map(|index| pen_type::cubical::Dim::Var(index as u16))
            .collect(),
    }
}

fn typed_representative(
    typing: &FormedPathTyping,
    key: &HistoricalFamilyKey,
) -> Option<(CubicalTerm, CubicalType, CubicalTerm)> {
    let context = CubicalContext::total(typing.dimension as u16);
    let term = match key {
        HistoricalFamilyKey::PathIntroduction { .. } => CubicalTerm::Point {
            point: path_point(typing),
        },
        HistoricalFamilyKey::Recursor { .. } | HistoricalFamilyKey::Inductor { .. } => {
            let family_name = match key {
                HistoricalFamilyKey::Recursor { .. } => "recursor",
                HistoricalFamilyKey::Inductor { .. } => "inductor",
                _ => unreachable!(),
            };
            let motive = Motive {
                owner: typing.formation_normal_form.clone(),
                name: format!(
                    "hist:{family_name}:{}",
                    tagged_hash("historical-motive", &(&typing.subject_hash, family_name))
                ),
            };
            let base = CubicalTerm::MotiveBase {
                motive: motive.clone(),
            };
            let method = CubicalTerm::PathMethod {
                motive: motive.clone(),
                path: path_point(typing),
                base: Box::new(base.clone()),
            };
            CubicalTerm::PathElim {
                motive,
                base: Box::new(base),
                method: Box::new(method),
                scrutinee: Box::new(CubicalTerm::Point {
                    point: PointExpr::Neutral {
                        owner: typing.formation_normal_form.clone(),
                        name: format!("x:{family_name}"),
                    },
                }),
            }
        }
        _ => return None,
    };
    normalize_typed_term(&context, &term)
        .ok()
        .map(|(ty, normal)| (term, ty, normal))
}

fn shallow_clause_evidence(
    elaboration: &TelescopeElaboration,
    clause: u16,
) -> Option<(&KernelTy, &Expr, String)> {
    elaboration
        .clauses
        .iter()
        .find(|entry| entry.clause_index == clause)
        .map(|entry| {
            (
                &entry.kernel_ty,
                &entry.normal_form,
                tagged_hash(
                    "shallow-clause-typing",
                    &(
                        &elaboration.derivation_hash,
                        entry.clause_index,
                        &entry.kernel_ty,
                        &entry.normal_form,
                        &entry.derivation,
                    ),
                ),
            )
        })
}

struct SurfaceFamilyContext<'a> {
    typing: &'a FormedPathTyping,
    elaboration: &'a TelescopeElaboration,
    closure_digest: &'a str,
}

fn surface_family(
    key: HistoricalFamilyKey,
    channel: FamilyChannel,
    judgement: &str,
    source_clauses: Vec<u16>,
    context: &SurfaceFamilyContext<'_>,
    boundary_dependent: bool,
    extra_blockers: Vec<String>,
) -> HistoricalFamilyAudit {
    let typed = typed_representative(context.typing, &key);
    let shallow = source_clauses
        .first()
        .and_then(|clause| shallow_clause_evidence(context.elaboration, *clause));
    let (evidence_level, term_hash, normal_form_hash, type_hash, token_hash) =
        if let Some((term, ty, normal)) = typed {
            (
                EvidenceLevel::TypedCubicalRepresentative,
                Some(tagged_hash("surface-representative", &(&key, &term))),
                Some(tagged_hash("surface-normal-form", &(&key, &normal))),
                Some(tagged_hash("surface-type", &(&key, &ty))),
                Some(tagged_hash(
                    "surface-representative-derivation",
                    &(
                        &context.typing.elaboration_derivation_hash,
                        &key,
                        &ty,
                        &normal,
                    ),
                )),
            )
        } else if let Some((kernel_ty, normal, proof_hash)) = shallow {
            (
                EvidenceLevel::ShallowClauseTyping,
                None,
                Some(tagged_hash("surface-shallow-normal", &(&key, normal))),
                Some(tagged_hash("surface-shallow-type", &(&key, kernel_ty))),
                Some(proof_hash),
            )
        } else {
            (
                EvidenceLevel::MissingTypedNaturalFamily,
                None,
                None,
                None,
                None,
            )
        };
    let equality_obstruction = "no opaque ordinary-family token proves naturality and an exhaustive predecessor weakening comparison for this surface family";
    let marginality =
        missing_surface_marginality(context.closure_digest, &key, equality_obstruction);
    let family_hash = tagged_hash(
        "surface-family-obligation",
        &(
            HISTORICAL_SURFACE_GRAMMAR_VERSION,
            &context.typing.subject_hash,
            &key,
            &normal_form_hash,
            &type_hash,
        ),
    );
    let mut blockers = vec![equality_obstruction.to_owned()];
    blockers.extend(extra_blockers);
    blockers.sort();
    blockers.dedup();
    let anchor = HistoricalAnchorAudit::Missing {
        obligation:
            "no proof-carrying EGP token binds this exact normal family to an injective local role or individual pre-existing demand output"
                .to_owned(),
    };
    let derivation_hash = tagged_hash(
        "surface-family-audit",
        &(
            &key,
            &family_hash,
            &term_hash,
            &normal_form_hash,
            &type_hash,
            &token_hash,
            &marginality,
            &anchor,
            boundary_dependent,
            &blockers,
        ),
    );
    HistoricalFamilyAudit {
        key,
        channel,
        judgement: judgement.to_owned(),
        source_clauses,
        natural_family_not_instance_proved: false,
        evidence_level,
        subject_hash: context.typing.subject_hash.clone(),
        signature_digest: context.typing.signature_digest.clone(),
        family_hash,
        term_hash,
        normal_form_hash,
        type_hash,
        token_derivation_hash: token_hash,
        marginality,
        anchor,
        credit_verdict: FamilyVerdict::Undefined,
        conditional_on_boundary_axiom_v1: boundary_dependent,
        blockers,
        derivation_hash,
    }
}

struct PathFamilyContext<'a> {
    signature: &'a SealedSignature,
    telescope: &'a Telescope,
    visible_library: u32,
    typing: &'a FormedPathTyping,
    closure: &'a [ClosurePathFamily],
    closure_digest: &'a str,
    truncation_boundary_mismatch: bool,
}

fn path_family(
    context: &PathFamilyContext<'_>,
    token: &PathRealizationToken,
) -> Result<HistoricalFamilyAudit, HistCertError> {
    replay_path_realization(
        context.signature,
        context.telescope,
        context.visible_library,
        context.typing,
        token,
    )
    .map_err(|_| HistCertError::TokenReplay(context.visible_library + 1))?;
    let step = context.visible_library + 1;
    let projection = project_checked(step, token)?;
    let marginality = current_path_marginality(
        step,
        context.signature,
        token,
        context.closure,
        context.closure_digest,
    )?;
    let judgement = format!("{:?}", projection.normal_form().judgement());
    let mut blockers = vec![
        "the registered beta/Kan grammar has typed representatives but no theorem classifies every intended depth-two path schema by that grammar"
            .to_owned(),
        "no proof-carrying EGP token binds this exact normal family to an injective local role or individual pre-existing demand output"
            .to_owned(),
    ];
    if context.truncation_boundary_mismatch {
        blockers.push(
            "the constant-boundary attachment realizes a generic loop surrogate, not the endpoint-dependent Trunc squash constructor"
                .to_owned(),
        );
    }
    let anchor = HistoricalAnchorAudit::Missing {
        obligation:
            "typed realization is not itself a distinct provenance anchor; the current coarse clause/role namespace would collide across multiple path sites"
                .to_owned(),
    };
    let key = HistoricalFamilyKey::Path {
        key: token.key().clone(),
    };
    let derivation_hash = tagged_hash(
        "path-family-audit",
        &(
            &key,
            projection.family_hash(),
            token.term_hash(),
            token.normal_form_hash(),
            projection.normal_form().type_hash(),
            projection.typed_derivation_hash(),
            &marginality,
            &anchor,
            &blockers,
        ),
    );
    Ok(HistoricalFamilyAudit {
        key,
        channel: FamilyChannel::Path,
        judgement,
        source_clauses: vec![context.typing.path_clause],
        natural_family_not_instance_proved: true,
        evidence_level: EvidenceLevel::TypedPathToken,
        subject_hash: token.subject_hash().to_owned(),
        signature_digest: token.signature_digest().to_owned(),
        family_hash: projection.family_hash().to_owned(),
        term_hash: Some(token.term_hash().to_owned()),
        normal_form_hash: Some(token.normal_form_hash().to_owned()),
        type_hash: Some(projection.normal_form().type_hash().to_owned()),
        token_derivation_hash: Some(projection.typed_derivation_hash().to_owned()),
        marginality,
        anchor,
        credit_verdict: FamilyVerdict::Undefined,
        conditional_on_boundary_axiom_v1: true,
        blockers,
        derivation_hash,
    })
}

fn is_parametric_formation(expr: &Expr) -> bool {
    matches!(expr, Expr::Trunc(inner) if matches!(inner.as_ref(), Expr::Var(_)))
}

fn build_step(step: u32) -> Result<HistoricalStepAudit, HistCertError> {
    if !(5..=8).contains(&step) {
        return Err(HistCertError::UnsupportedStep(step));
    }
    let signature = predecessor_signature(step);
    let telescope = Telescope::reference(step);
    let (typing, elaboration) =
        elaborate_formed_path(&signature, &telescope, step - 1).map_err(|error| {
            HistCertError::Elaboration {
                step,
                reason: error.to_string(),
            }
        })?;
    if typing.dimension > 3 {
        return Err(HistCertError::DimensionOutsideFragment {
            dimension: typing.dimension,
        });
    }
    let closure = predecessor_path_closure(step, &signature)?;
    let predecessor = context_audit(step, &signature, &closure)?;
    let closure_digest = predecessor.path_family_inventory_digest.clone();
    let truncation_boundary_mismatch =
        is_parametric_formation(&telescope.clauses[typing.formation_clause as usize].expr);
    let surface_context = SurfaceFamilyContext {
        typing: &typing,
        elaboration: &elaboration,
        closure_digest: &closure_digest,
    };

    let mut families = Vec::new();
    families.push(surface_family(
        HistoricalFamilyKey::Formation {
            clause: typing.formation_clause,
        },
        FamilyChannel::Surface,
        "type formation",
        vec![typing.formation_clause],
        &surface_context,
        false,
        Vec::new(),
    ));
    for clause in 0..typing.path_clause {
        if clause == typing.formation_clause {
            continue;
        }
        families.push(surface_family(
            HistoricalFamilyKey::PointIntroduction { clause },
            FamilyChannel::Surface,
            "point/unit introduction",
            vec![clause],
            &surface_context,
            false,
            Vec::new(),
        ));
    }
    let boundary_blocker = truncation_boundary_mismatch.then(|| {
        "the historical Trunc constructor requires an endpoint-dependent squash boundary, while the frozen attachment rule supplies only a constant boundary"
            .to_owned()
    });
    families.push(surface_family(
        HistoricalFamilyKey::PathIntroduction {
            clause: typing.path_clause,
        },
        FamilyChannel::Surface,
        "path constructor introduction",
        vec![typing.path_clause],
        &surface_context,
        true,
        boundary_blocker.clone().into_iter().collect(),
    ));
    families.push(surface_family(
        HistoricalFamilyKey::Recursor {
            path_clause: typing.path_clause,
        },
        FamilyChannel::Surface,
        "nondependent eliminator",
        vec![typing.path_clause],
        &surface_context,
        true,
        vec![
            "the typed neutral eliminator is one representative; motive-parametric naturality of the recursor family is not proved"
                .to_owned(),
        ],
    ));
    families.push(surface_family(
        HistoricalFamilyKey::Inductor {
            path_clause: typing.path_clause,
        },
        FamilyChannel::Surface,
        "dependent eliminator",
        vec![typing.path_clause],
        &surface_context,
        true,
        vec![
            "the typed neutral eliminator is one representative; motive-parametric naturality of the inductor family is not proved"
                .to_owned(),
        ],
    ));
    if truncation_boundary_mismatch {
        families.push(surface_family(
            HistoricalFamilyKey::ParametricAction {
                formation_clause: typing.formation_clause,
            },
            FamilyChannel::Surface,
            "parametric former action on maps",
            vec![typing.formation_clause],
            &surface_context,
            false,
            vec![
                "the cubical fragment has no typed ordinary function/action term for Trunc functoriality"
                    .to_owned(),
            ],
        ));
    }

    let basis = realize_path_basis(&signature, &telescope, step - 1, &typing).map_err(|error| {
        HistCertError::Realization {
            step,
            reason: error.to_string(),
        }
    })?;
    let path_context = PathFamilyContext {
        signature: &signature,
        telescope: &telescope,
        visible_library: step - 1,
        typing: &typing,
        closure: &closure,
        closure_digest: &closure_digest,
        truncation_boundary_mismatch,
    };
    for token in basis.tokens() {
        families.push(path_family(&path_context, token)?);
    }

    let post_clauses = ((typing.path_clause + 1)..telescope.kappa() as u16).collect::<Vec<_>>();
    for (offset, clause) in post_clauses.iter().copied().enumerate() {
        let role = if offset % 2 == 0 {
            PostPathRole::OperationForward
        } else {
            PostPathRole::CoherenceForward
        };
        families.push(surface_family(
            HistoricalFamilyKey::PostPathForward { clause, role },
            FamilyChannel::PostPath,
            match role {
                PostPathRole::OperationForward => "post-path operation forward face",
                PostPathRole::CoherenceForward => "post-path coherence rewrite",
            },
            vec![clause],
            &surface_context,
            false,
            vec![
                "the operation/coherence reading is a positional convention; the shallow lambda clause has no typed operation signature in the cubical fragment"
                    .to_owned(),
            ],
        ));
        if offset % 2 == 0 {
            families.push(surface_family(
                HistoricalFamilyKey::CellAction {
                    operation_clause: clause,
                    path_clause: typing.path_clause,
                },
                FamilyChannel::PostPath,
                "operation action on the path cell",
                vec![clause, typing.path_clause],
                &surface_context,
                true,
                vec![
                    "the cubical fragment has no typed operation action connecting the post-path lambda to the path constructor"
                        .to_owned(),
                ],
            ));
        }
    }

    families.sort_by(|left, right| left.key.cmp(&right.key));
    let keys = families
        .iter()
        .map(|family| family.key.clone())
        .collect::<BTreeSet<_>>();
    let operational_inventory_complete = keys.len() == families.len();
    let presented_family_count = families.len() as u32;
    let marginal_family_count_within_decided_fragment = families
        .iter()
        .filter(|family| family.marginality.verdict == FamilyVerdict::Marginal)
        .count() as u32;
    let weakening_family_count = families
        .iter()
        .filter(|family| family.marginality.verdict == FamilyVerdict::Weakening)
        .count() as u32;
    let undefined_family_count = families
        .iter()
        .filter(|family| family.credit_verdict == FamilyVerdict::Undefined)
        .count() as u32;
    let intended_semantic_inventory_complete = false;
    let certified_total = None;
    let conditional_on_boundary_axiom_v1 = families
        .iter()
        .any(|family| family.conditional_on_boundary_axiom_v1);
    let family_inventory_digest = tagged_hash(
        "historical-step-family-inventory",
        &families
            .iter()
            .map(|family| (&family.key, &family.derivation_hash))
            .collect::<Vec<_>>(),
    );
    let mut obstructions = families
        .iter()
        .flat_map(|family| {
            family
                .blockers
                .iter()
                .map(|blocker| format!("{:?}: {blocker}", family.key))
        })
        .collect::<Vec<_>>();
    obstructions.sort();
    obstructions.dedup();
    // The historical record enters only after the rule-driven family
    // inventory, marginality decisions, and obstruction set are frozen.
    let (label, recorded_total) = recorded_step(step)?;
    let presented_count_matches_record = presented_family_count == recorded_total;
    let certified_total_matches_record = certified_total == Some(recorded_total);
    let derivation_hash = tagged_hash(
        "historical-step-audit",
        &(
            step,
            typing.dimension,
            &predecessor,
            &family_inventory_digest,
            operational_inventory_complete,
            intended_semantic_inventory_complete,
            &families,
            presented_family_count,
            certified_total,
            conditional_on_boundary_axiom_v1,
        ),
    );
    Ok(HistoricalStepAudit {
        step,
        label: label.to_owned(),
        dimension: typing.dimension,
        predecessor,
        family_inventory_digest,
        operational_inventory_complete,
        intended_semantic_inventory_complete,
        intended_path_schema_exhaustiveness_proved: false,
        non_path_typed_family_inventory_complete: false,
        families,
        presented_family_count,
        marginal_family_count_within_decided_fragment,
        weakening_family_count,
        undefined_family_count,
        certified_total,
        recorded_total,
        presented_count_matches_record,
        certified_total_matches_record,
        conditional_on_boundary_axiom_v1,
        obstructions,
        derivation_hash,
    })
}

fn build_steps() -> Result<Vec<HistoricalStepAudit>, HistCertError> {
    (5..=8).map(build_step).collect()
}

fn run_mutation_falsifiers(
    canonical: &[HistoricalStepAudit],
) -> Result<MutationFalsifiers, HistCertError> {
    let rebuild = || build_steps();
    let rejected = |mutation: Vec<HistoricalStepAudit>| -> Result<bool, HistCertError> {
        Ok(rebuild()? != mutation)
    };

    let mut verdict = canonical.to_vec();
    verdict[0].families[0].marginality.verdict = FamilyVerdict::Marginal;
    let verdict_flip_rejected = rejected(verdict)?;

    let mut anchor = canonical.to_vec();
    let binding = anchor[0].families[0].family_hash.clone();
    anchor[0].families[0].anchor = HistoricalAnchorAudit::ClaimedLocal {
        clause: 0,
        role: "forged-role".to_owned(),
        family_binding_hash: binding,
    };
    let anchor_claim_rejected = rejected(anchor)?;

    let mut equality = canonical.to_vec();
    let comparison = equality
        .iter_mut()
        .flat_map(|step| step.families.iter_mut())
        .flat_map(|family| family.marginality.comparisons.iter_mut())
        .next()
        .expect("later historical prefixes have predecessor path comparisons");
    comparison.derivation_hash = "blake3:00".to_owned();
    let equality_proof_mutation_rejected = rejected(equality)?;

    let mut weakening_inverse = canonical.to_vec();
    let inverse_audit = weakening_inverse
        .iter_mut()
        .flat_map(|step| step.predecessor.weakening_erasure_audits.iter_mut())
        .next()
        .expect("later historical prefixes contain weakened path families");
    inverse_audit.erasure_after_weakening = false;
    let weakening_inverse_mutation_rejected = rejected(weakening_inverse)?;

    let mut conditionality = canonical.to_vec();
    conditionality[0].conditional_on_boundary_axiom_v1 = false;
    let conditionality_flip_rejected = rejected(conditionality)?;

    let mut token = canonical.to_vec();
    let token_family = token
        .iter_mut()
        .flat_map(|step| step.families.iter_mut())
        .find(|family| family.token_derivation_hash.is_some())
        .expect("historical audit has token evidence");
    token_family.token_derivation_hash = Some("blake3:00".to_owned());
    let token_hash_mutation_rejected = rejected(token)?;

    let mut deletion = canonical.to_vec();
    deletion[0].families.remove(0);
    let family_deletion_rejected = rejected(deletion.clone())?;
    let forged_outer = tagged_hash("result", &deletion);
    let canonical_outer = tagged_hash("result", canonical);
    let rebased_outer_digest_rejected = forged_outer != canonical_outer && rejected(deletion)?;

    Ok(MutationFalsifiers {
        verdict_flip_rejected,
        anchor_claim_rejected,
        equality_proof_mutation_rejected,
        weakening_inverse_mutation_rejected,
        conditionality_flip_rejected,
        token_hash_mutation_rejected,
        family_deletion_rejected,
        rebased_outer_digest_rejected,
    })
}

pub fn build_hist_cert() -> Result<HistCertResult, HistCertError> {
    let steps = build_steps()?;
    let mutation_falsifiers = run_mutation_falsifiers(&steps)?;
    let all_operational_presentations_match = steps
        .iter()
        .all(|step| step.operational_inventory_complete && step.presented_count_matches_record);
    let all_steps_semantically_certified = steps.iter().all(|step| {
        step.intended_semantic_inventory_complete && step.certified_total_matches_record
    });
    let mut result = HistCertResult {
        schema: HIST_CERT_SCHEMA.to_owned(),
        cubical_fragment_version: CUBICAL_FRAGMENT_VERSION.to_owned(),
        historical_path_family_fragment_version:
            HISTORICAL_PATH_FAMILY_FRAGMENT_VERSION.to_owned(),
        historical_surface_grammar_version: HISTORICAL_SURFACE_GRAMMAR_VERSION.to_owned(),
        boundary_axiom_version: PATHCON_ATTACHMENT_AXIOM_VERSION.to_owned(),
        boundary_axiom_status: "conditional_unadjudicated".to_owned(),
        steps,
        all_operational_presentations_match,
        all_steps_semantically_certified,
        f_t1_discharged: all_steps_semantically_certified,
        mutation_falsifiers,
        remaining_obligations: vec![
            "mint opaque typed natural-family tokens for formation, point/unit, recursor, inductor, parametric action, and post-path operation families"
                .to_owned(),
            "prove the registered beta/Kan grammar exhaustive for intended depth-two path schemas, not only exact for its own index set"
                .to_owned(),
            "supply a boundary rule that types each historical constructor; the current constant-boundary rule does not type the endpoint-dependent Trunc squash"
                .to_owned(),
            "bind every marginal normal family to a distinct proof-carrying local role or individual pre-existing demand output"
                .to_owned(),
            "rerun this create-new certificate after those obligations are discharged; do not promote the presented arithmetic totals in the meantime"
                .to_owned(),
        ],
        result_digest: String::new(),
    };
    result.result_digest = tagged_hash(
        "result",
        &(
            &result.schema,
            &result.cubical_fragment_version,
            &result.historical_path_family_fragment_version,
            &result.historical_surface_grammar_version,
            &result.boundary_axiom_version,
            &result.boundary_axiom_status,
            &result.steps,
            result.all_operational_presentations_match,
            result.all_steps_semantically_certified,
            result.f_t1_discharged,
            &result.mutation_falsifiers,
            &result.remaining_obligations,
        ),
    );
    Ok(result)
}

pub fn replay_hist_cert(presented: &HistCertResult) -> Result<(), HistCertError> {
    let rebuilt = build_hist_cert()?;
    if &rebuilt == presented {
        Ok(())
    } else {
        Err(HistCertError::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn predecessor_contexts_are_exact_and_path_weakening_is_decided() {
        let result = build_hist_cert().expect("historical certificate");
        assert_eq!(
            result
                .steps
                .iter()
                .map(|step| step.predecessor.signature_digest.as_str())
                .collect::<Vec<_>>(),
            vec![
                "blake3:11c74244bfb002c53bc4f3b9a3c7013e7e38460ad03ce63bf9bc41e535fc8458",
                "blake3:0ee6911b2820caeb0e745b4e44fa0875b8bfbca8f66b6a4921694ebde1cf44fd",
                "blake3:e8cba04def50e09c83b1e28a252bb6f2e327e6468027a3d9353e20d9ad19e5ed",
                "blake3:301916fbd578c9e0d6375724b44201f0198d2d4bbd9493614009a603325a3adb",
            ]
        );
        assert_eq!(
            result
                .steps
                .iter()
                .map(|step| step.predecessor.weakening_erasure_audits.len())
                .collect::<Vec<_>>(),
            vec![0, 2, 4, 9]
        );
        for step in &result.steps {
            assert!(step.predecessor.registered_path_inventory_complete);
            assert!(step.predecessor.weakening_erasure_inverse_laws_checked);
            assert!(
                step.predecessor
                    .weakening_erasure_audits
                    .iter()
                    .all(|audit| audit.erasure_after_weakening
                        && audit.weakening_after_erasure
                        && audit.original_family_hash == audit.weakened_family_hash)
            );
            assert!(
                step.families
                    .iter()
                    .filter(|family| family.channel == FamilyChannel::Path)
                    .all(|family| family.marginality.verdict == FamilyVerdict::Marginal)
            );
        }
    }

    #[test]
    fn operational_slots_match_but_semantic_totals_fail_closed() {
        let result = build_hist_cert().expect("historical certificate");
        assert_eq!(
            result
                .steps
                .iter()
                .map(|step| step.presented_family_count)
                .collect::<Vec<_>>(),
            vec![7, 8, 10, 18]
        );
        assert!(result.all_operational_presentations_match);
        assert!(
            result
                .steps
                .iter()
                .all(|step| step.certified_total.is_none())
        );
        assert!(!result.all_steps_semantically_certified);
        assert!(!result.f_t1_discharged);
        assert!(result.steps.iter().all(|step| {
            step.undefined_family_count > 0
                && !step.intended_path_schema_exhaustiveness_proved
                && !step.non_path_typed_family_inventory_complete
                && step.conditional_on_boundary_axiom_v1
        }));
    }

    #[test]
    fn truncation_and_post_path_gaps_are_named_per_family() {
        let result = build_hist_cert().expect("historical certificate");
        let trunc = &result.steps[1];
        assert!(
            trunc
                .obstructions
                .iter()
                .any(|obstruction| { obstruction.contains("endpoint-dependent Trunc squash") })
        );
        let final_historical_hit = &result.steps[3];
        assert!(
            final_historical_hit
                .obstructions
                .iter()
                .any(|obstruction| { obstruction.contains("positional convention") })
        );
        assert!(
            final_historical_hit
                .obstructions
                .iter()
                .any(|obstruction| { obstruction.contains("no typed operation action") })
        );
    }

    #[test]
    fn replay_and_registered_mutations_fail_closed() {
        let result = build_hist_cert().expect("historical certificate");
        replay_hist_cert(&result).expect("exact replay");
        assert!(result.mutation_falsifiers.verdict_flip_rejected);
        assert!(result.mutation_falsifiers.anchor_claim_rejected);
        assert!(result.mutation_falsifiers.equality_proof_mutation_rejected);
        assert!(
            result
                .mutation_falsifiers
                .weakening_inverse_mutation_rejected
        );
        assert!(result.mutation_falsifiers.conditionality_flip_rejected);
        assert!(result.mutation_falsifiers.token_hash_mutation_rejected);
        assert!(result.mutation_falsifiers.family_deletion_rejected);
        assert!(result.mutation_falsifiers.rebased_outer_digest_rejected);

        let mut mutation = result;
        mutation.steps[0].conditional_on_boundary_axiom_v1 = false;
        mutation.result_digest = tagged_hash("result", &mutation.steps);
        assert_eq!(
            replay_hist_cert(&mutation),
            Err(HistCertError::ReplayMismatch)
        );
    }
}
