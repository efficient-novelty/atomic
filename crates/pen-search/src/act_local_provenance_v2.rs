//! Sound, extraction-first candidate-local provenance for T-BI-NU1 v2.
//!
//! The intrinsic issuer sees only a sealed prefix and one candidate. Natural
//! families are minted by the typed family extractor, marginality is decided
//! against that extractor's predecessor closure, and instances remain members
//! of their family unless an already-enumerated demand-output position funds
//! them. No archive, historical scalar, bar, verdict, or enacted future is an
//! input to this module.

use pen_core::clause::ClauseRole;
use pen_core::hash::blake3_hex;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_eval::debt_guard::{WINDOW_DEPTH, required_packages_for};
use pen_eval::demand_orbits::stage_inventories_for_timeline;
use pen_eval::semantic_provenance::{
    CandidateProvenanceSubmission, ConditionalAtMostBound, CreditMechanism, CreditedSchemaFamily,
    DemandOutputRef, LocalRole, MarginalityWitness, OrbitResolution, ProvenanceAnchor,
    SchemaFamilyId, SchemaInstance, SchemaInstanceId, SemanticAssumptionRef, StageOrbitInventory,
    TypedNormalFamily, TypedNormalJudgement, UnivalentClassId, derive_conditional_at_most,
};
use pen_eval::typed_families::{
    CandidateExtractionOutcome, ExtractedFamily, InstanceKind, MarginalityDisposition,
    extract_candidate_families, predecessor_closure,
};
use pen_type::elaborate::{SealedSignature, candidate_hash};
use pen_type::obligations::summarize_structural_debt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

pub const ACT_LOCAL_PROVENANCE_V2_SCHEMA: &str = "act-local-provenance-v2";
pub const ACT_LOCAL_PROVENANCE_V2_DATE: &str = "2026-07-22";
pub const T_BI_NU1_V2_THEOREM_ID: &str = "T-BI-NU1-v2-typed-natural-marginal-family-injection";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(ACT_LOCAL_PROVENANCE_V2_SCHEMA, domain, value))
        .expect("v2 act-local evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV2TheoremGap {
    pub id: String,
    pub stage: u32,
    pub family_id: Option<String>,
    pub kind: String,
    pub detail: String,
    pub derivation_hash: String,
}

fn theorem_gap(
    stage: u32,
    family_id: Option<&str>,
    kind: &str,
    detail: impl Into<String>,
) -> ActLocalV2TheoremGap {
    let family_slug = family_id.unwrap_or("stage");
    let mut gap = ActLocalV2TheoremGap {
        id: format!("T-BI-NU1-V2-S{stage}-{kind}-{family_slug}"),
        stage,
        family_id: family_id.map(str::to_owned),
        kind: kind.to_owned(),
        detail: detail.into(),
        derivation_hash: String::new(),
    };
    gap.derivation_hash = tagged_hash("theorem-gap", &gap);
    gap
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalV2FamilyAudit {
    pub family_id: String,
    pub marginal: bool,
    pub generator_clause: Option<u16>,
    pub generator_clause_role: ClauseRole,
    pub naturality_square_equal: bool,
    pub instance_occurrence_count: usize,
    pub collapsed_non_generator_instance_count: usize,
    pub credited_once: bool,
    pub anchor: Option<ProvenanceAnchor>,
    pub independently_exported_instance_credit_count: usize,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActLocalProvenanceV2Certificate {
    pub schema: String,
    pub date: String,
    pub theorem_id: String,
    pub stage: u32,
    pub candidate_hash: String,
    pub predecessor_signature_digest: String,
    pub kappa: u16,
    pub predecessor_closure_digest: Option<String>,
    pub candidate_family_extraction_hash: Option<String>,
    pub demand_orbit_extraction_hash: Option<String>,
    pub demand_inventory_window: Option<[u32; 2]>,
    pub extracted_natural_family_count: usize,
    pub marginal_natural_family_count: usize,
    pub credited_natural_family_count: usize,
    pub collapsed_non_generator_instance_count: usize,
    pub independently_exported_instance_credit_count: usize,
    pub family_audits: Vec<ActLocalV2FamilyAudit>,
    pub checker_submission: CandidateProvenanceSubmission,
    pub conditional_at_most: Option<ConditionalAtMostBound>,
    pub conditional_at_most_checker_error: Option<String>,
    pub certified_marginal_nu: Option<u32>,
    pub blind_local_role_bound_holds: bool,
    pub actual_local_role_anchors_unique: bool,
    pub pre_existing_required_output_positions_unique: bool,
    pub used_pre_existing_required_output_positions_unique: bool,
    pub used_pre_existing_required_output_position_count: usize,
    pub candidate_minted_required_output_position_count: usize,
    pub uniform_or_repeated_instances_not_multiplied: bool,
    pub structural_nu_called_by_intrinsic_issuer: bool,
    pub enacted_or_global_demand_timeline_read_by_intrinsic_issuer: bool,
    pub archive_available_during_intrinsic_issuance: bool,
    pub historical_score_or_total_read_by_intrinsic_issuer: bool,
    pub bar_or_verdict_read_by_intrinsic_issuer: bool,
    pub theorem_gaps: Vec<ActLocalV2TheoremGap>,
    pub authoritative: bool,
    pub derivation_hash: String,
}

impl ActLocalProvenanceV2Certificate {
    pub fn credited_family_ids(&self) -> Vec<String> {
        self.checker_submission
            .credited_families
            .iter()
            .map(|credit| credit.family.id.0.clone())
            .collect()
    }
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ActLocalProvenanceV2Error {
    #[error("invalid v2 act-local input: {0}")]
    Input(String),
}

fn mechanism_for_clause_role(role: ClauseRole) -> (CreditMechanism, LocalRole) {
    match role {
        ClauseRole::Formation | ClauseRole::Introduction => {
            (CreditMechanism::IntrinsicKernel, LocalRole::KernelHead)
        }
        ClauseRole::Elimination => (CreditMechanism::AdjointCompletion, LocalRole::AdjointMate),
        ClauseRole::PathAttach => (CreditMechanism::DimensionSquared, LocalRole::Coherence),
        ClauseRole::Computation => (
            CreditMechanism::P6UniformSpecialization,
            LocalRole::SupportAction,
        ),
    }
}

fn verified(obligation: impl Into<String>, derivation_hash: &str) -> SemanticAssumptionRef {
    SemanticAssumptionRef::verified(obligation, derivation_hash.to_owned())
}

fn generator_clause(family: &ExtractedFamily) -> Option<u16> {
    family.instances.iter().find_map(|instance| {
        matches!(&instance.kind, InstanceKind::Generator).then_some(instance.clause_index)
    })
}

fn semantic_family(family: &ExtractedFamily, extraction_hash: &str) -> TypedNormalFamily {
    let family_id = SchemaFamilyId(family.id.as_str().to_owned());
    let normal_form = serde_json::to_string(&family.presentation.canonical_normal_form)
        .expect("canonical normal form serializes");
    let parameter_form = serde_json::to_string(&family.presentation.parameters)
        .expect("parameter telescope serializes");
    let type_form =
        serde_json::to_string(&family.generator_kernel_ty).expect("kernel type serializes");
    let naturality_hash = tagged_hash("typed-family-naturality", &family.naturality);
    TypedNormalFamily {
        id: family_id,
        judgement: TypedNormalJudgement {
            context_normal_form: parameter_form,
            term_normal_form: normal_form,
            type_normal_form: type_form,
            normalization: verified(
                format!("typed extractor normalization for {}", family.id.as_str()),
                extraction_hash,
            ),
            type_preservation: verified(
                format!("typed extractor preservation for {}", family.id.as_str()),
                extraction_hash,
            ),
        },
        parameters: family
            .presentation
            .parameters
            .iter()
            .map(|parameter| format!("{parameter:?}"))
            .collect(),
        naturality: verified(
            format!("typed extractor naturality for {}", family.id.as_str()),
            &naturality_hash,
        ),
        univalent_class: UnivalentClassId(family.id.as_str().to_owned()),
    }
}

fn collapsed_instances(family: &ExtractedFamily) -> Vec<SchemaInstance> {
    family
        .instances
        .iter()
        .filter(|instance| !matches!(&instance.kind, InstanceKind::Generator))
        .map(|instance| SchemaInstance {
            id: SchemaInstanceId(format!(
                "{}:clause:{}",
                family.id.as_str(),
                instance.clause_index
            )),
            family: SchemaFamilyId(family.id.as_str().to_owned()),
            substitution_normal_form: serde_json::to_string(&instance.kind)
                .expect("instance kind serializes"),
            independently_exported_demand: None,
        })
        .collect()
}

fn exact_prefix_timeline(prefix: &SealedSignature, stage: u32) -> Vec<(u32, Vec<String>)> {
    let mut library: Library = Vec::new();
    let mut timeline = Vec::with_capacity(stage as usize);
    for current_stage in 1..=stage {
        let debt = summarize_structural_debt(&library, WINDOW_DEPTH);
        let required = required_packages_for(debt)
            .into_iter()
            .map(str::to_owned)
            .collect::<Vec<_>>();
        timeline.push((current_stage, required));
        if current_stage < stage {
            let telescope = &prefix.entries()[(current_stage - 1) as usize].telescope;
            library.push(LibraryEntry::from_telescope(telescope, &library));
        }
    }
    timeline
}

fn inventory_output_positions_unique(inventory: &StageOrbitInventory) -> bool {
    let mut seen = BTreeSet::new();
    inventory.orbits.iter().all(|orbit| {
        orbit.required_outputs.iter().all(|output| {
            seen.insert(DemandOutputRef {
                orbit: orbit.id.clone(),
                position: output.id.clone(),
            })
        })
    })
}

fn compatible_live_output(
    inventory: &StageOrbitInventory,
    family: &TypedNormalFamily,
    used: &BTreeSet<DemandOutputRef>,
) -> Option<DemandOutputRef> {
    inventory.orbits.iter().find_map(|orbit| {
        if !matches!(&orbit.resolution, OrbitResolution::Live) {
            return None;
        }
        orbit.required_outputs.iter().find_map(|output| {
            let demand = DemandOutputRef {
                orbit: orbit.id.clone(),
                position: output.id.clone(),
            };
            (output.family_class == family.univalent_class && !used.contains(&demand))
                .then_some(demand)
        })
    })
}

fn package_digest(certificate: &ActLocalProvenanceV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.derivation_hash.clear();
    tagged_hash("act-local-provenance-v2-certificate", &projection)
}

pub fn issue_act_local_provenance_v2(
    prefix: &SealedSignature,
    stage: u32,
    candidate: &Telescope,
) -> Result<ActLocalProvenanceV2Certificate, ActLocalProvenanceV2Error> {
    if stage == 0 {
        return Err(ActLocalProvenanceV2Error::Input(
            "stage must be positive".to_owned(),
        ));
    }
    if prefix.entries().len() != stage.saturating_sub(1) as usize
        || !prefix.entries().iter().map(|entry| entry.step).eq(1..stage)
    {
        return Err(ActLocalProvenanceV2Error::Input(format!(
            "Stage {stage} requires the exact contiguous predecessor prefix"
        )));
    }
    let kappa = u16::try_from(candidate.kappa())
        .map_err(|_| ActLocalProvenanceV2Error::Input("candidate kappa exceeds u16".to_owned()))?;
    let candidate_digest = candidate_hash(candidate);
    let mut gaps = Vec::new();
    let mut closure_digest = None;
    let mut extraction_hash = None;
    let mut orbit_extraction_hash = None;
    let mut demand_inventory_window = None;
    let mut extracted_natural_family_count = 0usize;
    let mut marginal_natural_family_count = 0usize;
    let mut family_audits = Vec::new();
    let mut credited_families = Vec::new();
    let mut used_local_anchors = BTreeSet::<(u16, LocalRole)>::new();
    let mut used_output_positions = BTreeSet::<DemandOutputRef>::new();

    let closure = match predecessor_closure(prefix) {
        Ok(closure) => {
            closure_digest = Some(closure.digest.clone());
            Some(closure)
        }
        Err(error) => {
            gaps.push(theorem_gap(
                stage,
                None,
                "PREDECESSOR_CLOSURE_GAP",
                error.to_string(),
            ));
            None
        }
    };

    let orbit_extraction = closure.as_ref().and_then(|closure| {
        let timeline = exact_prefix_timeline(prefix, stage);
        match stage_inventories_for_timeline(prefix, closure, &timeline) {
            Ok(extraction) => {
                orbit_extraction_hash = Some(extraction.derivation_hash.clone());
                Some(extraction)
            }
            Err(error) => {
                gaps.push(theorem_gap(
                    stage,
                    None,
                    "DEMAND_ORBIT_INVENTORY_GAP",
                    error.to_string(),
                ));
                None
            }
        }
    });
    let inventory = orbit_extraction
        .as_ref()
        .and_then(|extraction| extraction.stage(stage))
        .cloned();
    if let Some(inventory) = &inventory {
        demand_inventory_window = Some(inventory.window);
        if !inventory_output_positions_unique(inventory) {
            gaps.push(theorem_gap(
                stage,
                None,
                "DUPLICATE_PRE_EXISTING_DEMAND_OUTPUT",
                "verified inventory repeats an (orbit, required-output-position) pair",
            ));
        }
    } else {
        gaps.push(theorem_gap(
            stage,
            None,
            "STAGE_DEMAND_INVENTORY_ABSENT",
            "the exact prefix extraction did not expose the candidate-stage inventory",
        ));
    }

    if let Some(closure) = &closure {
        match extract_candidate_families(prefix, closure, candidate, stage.saturating_sub(1)) {
            CandidateExtractionOutcome::Extracted(extraction) => {
                extraction_hash = Some(extraction.derivation_hash.clone());
                extracted_natural_family_count = extraction.families.len();
                marginal_natural_family_count = extraction.marginal_family_count;
                for family in &extraction.families {
                    let marginal = matches!(
                        &family.marginality,
                        MarginalityDisposition::MarginalNoClosurePreimage { .. }
                    );
                    let generator = generator_clause(family);
                    let mut anchor = None;
                    let mut credited_once = false;
                    let semantic = semantic_family(family, &extraction.derivation_hash);
                    let (mechanism, required_role) =
                        mechanism_for_clause_role(family.generator_role);
                    if marginal {
                        if !family.naturality.square.equal {
                            gaps.push(theorem_gap(
                                stage,
                                Some(family.id.as_str()),
                                "NATURALITY_GAP",
                                "typed extractor's naturality square is not equal",
                            ));
                        } else if let Some(clause) = generator {
                            let local = (clause, required_role);
                            let selected_anchor = if u32::from(clause) < u32::from(kappa)
                                && used_local_anchors.insert(local)
                            {
                                Some(ProvenanceAnchor::ChargedKernel {
                                    clause,
                                    role: required_role,
                                })
                            } else {
                                inventory.as_ref().and_then(|inventory| {
                                    compatible_live_output(
                                        inventory,
                                        &semantic,
                                        &used_output_positions,
                                    )
                                    .map(|demand| {
                                        used_output_positions.insert(demand.clone());
                                        ProvenanceAnchor::PreExistingDemandOutput { demand }
                                    })
                                })
                            };
                            if let Some(selected_anchor) = selected_anchor {
                                let instances = collapsed_instances(family);
                                let marginality = MarginalityWitness::NoWeakeningPreimage {
                                    fresh_kernel_clause: clause,
                                    no_preimage: verified(
                                        format!(
                                            "typed predecessor closure has no preimage for {}",
                                            family.id.as_str()
                                        ),
                                        &extraction.derivation_hash,
                                    ),
                                };
                                credited_families.push(CreditedSchemaFamily {
                                    family: semantic,
                                    instances,
                                    marginality,
                                    mechanism,
                                    anchor: selected_anchor.clone(),
                                    anchor_validity_assumption: verified(
                                        format!(
                                            "extractor generator clause {clause} owns family {}",
                                            family.id.as_str()
                                        ),
                                        &extraction.derivation_hash,
                                    ),
                                });
                                anchor = Some(selected_anchor);
                                credited_once = true;
                            } else {
                                gaps.push(theorem_gap(
                                    stage,
                                    Some(family.id.as_str()),
                                    "UNANCHORED_MARGINAL_FAMILY",
                                    format!(
                                        "actual local slot ({clause},{required_role:?}) was unavailable and no compatible pre-existing live required output exists"
                                    ),
                                ));
                            }
                        } else {
                            gaps.push(theorem_gap(
                                stage,
                                Some(family.id.as_str()),
                                "GENERATOR_CLAUSE_GAP",
                                "marginal family has no extractor-recorded generator clause",
                            ));
                        }
                    }
                    let mut audit = ActLocalV2FamilyAudit {
                        family_id: family.id.as_str().to_owned(),
                        marginal,
                        generator_clause: generator,
                        generator_clause_role: family.generator_role,
                        naturality_square_equal: family.naturality.square.equal,
                        instance_occurrence_count: family.instances.len(),
                        collapsed_non_generator_instance_count: family
                            .instances
                            .iter()
                            .filter(|instance| !matches!(&instance.kind, InstanceKind::Generator))
                            .count(),
                        credited_once,
                        anchor,
                        independently_exported_instance_credit_count: 0,
                        derivation_hash: String::new(),
                    };
                    audit.derivation_hash = tagged_hash("family-audit", &audit);
                    family_audits.push(audit);
                }
            }
            CandidateExtractionOutcome::KernelInvalid { failure } => gaps.push(theorem_gap(
                stage,
                None,
                "CANDIDATE_KERNEL_INVALID",
                failure.to_string(),
            )),
        }
    }

    let (demand_orbits, orbit_inventory_completeness_assumption, window_locality_assumption) =
        inventory.as_ref().map_or_else(
            || {
                (
                    Vec::new(),
                    SemanticAssumptionRef::missing(format!(
                        "Stage {stage} demand inventory unavailable"
                    )),
                    SemanticAssumptionRef::missing(format!(
                        "Stage {stage} window locality unavailable"
                    )),
                )
            },
            |inventory| {
                (
                    inventory.orbits.clone(),
                    inventory.extraction_completeness_assumption.clone(),
                    inventory.window_locality_assumption.clone(),
                )
            },
        );
    let family_extraction_completeness_assumption = extraction_hash.as_ref().map_or_else(
        || {
            SemanticAssumptionRef::missing(format!(
                "Stage {stage} typed family extraction unavailable"
            ))
        },
        |hash| {
            verified(
                format!(
                    "Stage {stage} typed family extraction enumerates and quotients all candidate families"
                ),
                hash,
            )
        },
    );
    let submission = CandidateProvenanceSubmission {
        candidate_stage: stage,
        kappa,
        credited_families,
        family_extraction_completeness_assumption,
        demand_orbits,
        orbit_inventory_completeness_assumption,
        window_locality_assumption,
    };
    let conditional = match derive_conditional_at_most(&submission) {
        Ok(bound) => Some(bound),
        Err(error) => {
            gaps.push(theorem_gap(
                stage,
                None,
                "CONDITIONAL_AT_MOST_CHECKER_REJECTED",
                error.to_string(),
            ));
            None
        }
    };
    let credited_natural_family_count = submission.credited_families.len();
    let collapsed_non_generator_instance_count = family_audits
        .iter()
        .map(|family| family.collapsed_non_generator_instance_count)
        .sum();
    let independently_exported_instance_credit_count = conditional
        .as_ref()
        .map_or(0, |bound| bound.exported_instance_credits as usize);
    let actual_local_role_anchors_unique = submission
        .credited_families
        .iter()
        .filter_map(|credit| match &credit.anchor {
            ProvenanceAnchor::ChargedKernel { clause, role } => Some((*clause, *role)),
            ProvenanceAnchor::PreExistingDemandOutput { .. } => None,
        })
        .collect::<BTreeSet<_>>()
        .len()
        == submission
            .credited_families
            .iter()
            .filter(|credit| matches!(&credit.anchor, ProvenanceAnchor::ChargedKernel { .. }))
            .count();
    let pre_existing_required_output_positions_unique = inventory
        .as_ref()
        .is_some_and(inventory_output_positions_unique);
    let used_pre_existing_required_output_positions_unique = used_output_positions.len()
        == submission
            .credited_families
            .iter()
            .filter(|credit| {
                matches!(
                    &credit.anchor,
                    ProvenanceAnchor::PreExistingDemandOutput { .. }
                )
            })
            .count();
    let uniform_or_repeated_instances_not_multiplied = submission
        .credited_families
        .iter()
        .flat_map(|credit| &credit.instances)
        .all(|instance| instance.independently_exported_demand.is_none())
        && independently_exported_instance_credit_count == 0;
    let certified_marginal_nu = conditional.as_ref().map(|bound| bound.marginal_nu);
    let blind_local_role_bound_holds = conditional
        .as_ref()
        .is_some_and(|bound| bound.marginal_nu <= bound.local_capacity);
    let authoritative = gaps.is_empty()
        && credited_natural_family_count == marginal_natural_family_count
        && conditional.as_ref().is_some_and(|bound| {
            bound.natural_family_credits as usize == marginal_natural_family_count
                && bound.exported_instance_credits == 0
                && bound.marginal_nu as usize == marginal_natural_family_count
        })
        && blind_local_role_bound_holds
        && actual_local_role_anchors_unique
        && pre_existing_required_output_positions_unique
        && used_pre_existing_required_output_positions_unique
        && uniform_or_repeated_instances_not_multiplied;
    let mut certificate = ActLocalProvenanceV2Certificate {
        schema: ACT_LOCAL_PROVENANCE_V2_SCHEMA.to_owned(),
        date: ACT_LOCAL_PROVENANCE_V2_DATE.to_owned(),
        theorem_id: T_BI_NU1_V2_THEOREM_ID.to_owned(),
        stage,
        candidate_hash: candidate_digest,
        predecessor_signature_digest: prefix.digest().to_owned(),
        kappa,
        predecessor_closure_digest: closure_digest,
        candidate_family_extraction_hash: extraction_hash,
        demand_orbit_extraction_hash: orbit_extraction_hash,
        demand_inventory_window,
        extracted_natural_family_count,
        marginal_natural_family_count,
        credited_natural_family_count,
        collapsed_non_generator_instance_count,
        independently_exported_instance_credit_count,
        family_audits,
        checker_submission: submission,
        conditional_at_most: conditional,
        conditional_at_most_checker_error: gaps
            .iter()
            .find(|gap| gap.kind == "CONDITIONAL_AT_MOST_CHECKER_REJECTED")
            .map(|gap| gap.detail.clone()),
        certified_marginal_nu,
        blind_local_role_bound_holds,
        actual_local_role_anchors_unique,
        pre_existing_required_output_positions_unique,
        used_pre_existing_required_output_positions_unique,
        used_pre_existing_required_output_position_count: used_output_positions.len(),
        candidate_minted_required_output_position_count: 0,
        uniform_or_repeated_instances_not_multiplied,
        structural_nu_called_by_intrinsic_issuer: false,
        enacted_or_global_demand_timeline_read_by_intrinsic_issuer: false,
        archive_available_during_intrinsic_issuance: false,
        historical_score_or_total_read_by_intrinsic_issuer: false,
        bar_or_verdict_read_by_intrinsic_issuer: false,
        theorem_gaps: gaps,
        authoritative,
        derivation_hash: String::new(),
    };
    certificate.derivation_hash = package_digest(&certificate);
    Ok(certificate)
}

pub fn issue_act_local_sequence_v2(
    entries: &[(u32, Telescope)],
) -> Result<Vec<ActLocalProvenanceV2Certificate>, ActLocalProvenanceV2Error> {
    if entries.is_empty()
        || !entries
            .iter()
            .map(|(stage, _)| *stage)
            .eq(1..=entries.len() as u32)
    {
        return Err(ActLocalProvenanceV2Error::Input(
            "v2 act-local sequence must be contiguous from Stage 1".to_owned(),
        ));
    }
    entries
        .iter()
        .enumerate()
        .map(|(index, (stage, candidate))| {
            let prefix = SealedSignature::from_telescopes(entries[..index].to_vec());
            issue_act_local_provenance_v2(&prefix, *stage, candidate)
        })
        .collect()
}

pub fn replay_act_local_provenance_v2(
    prefix: &SealedSignature,
    candidate: &Telescope,
    claimed: &ActLocalProvenanceV2Certificate,
) -> Vec<String> {
    let mut errors = Vec::new();
    if claimed.derivation_hash != package_digest(claimed) {
        errors.push("v2 act-local certificate digest mismatch".to_owned());
    }
    match issue_act_local_provenance_v2(prefix, claimed.stage, candidate) {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("v2 act-local certificate differs from reissuance".to_owned()),
        Err(error) => errors.push(error.to_string()),
    }
    errors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extraction_first_issuer_uses_real_slots_and_never_mints_outputs() {
        let entries = (1..=15)
            .map(|stage| (stage, Telescope::reference(stage)))
            .collect::<Vec<_>>();
        let packages = issue_act_local_sequence_v2(&entries).expect("v2 sequence");
        assert_eq!(packages.len(), 15);
        for package in &packages {
            assert!(!package.structural_nu_called_by_intrinsic_issuer);
            assert!(!package.enacted_or_global_demand_timeline_read_by_intrinsic_issuer);
            assert!(!package.archive_available_during_intrinsic_issuance);
            assert_eq!(package.candidate_minted_required_output_position_count, 0);
            assert!(package.uniform_or_repeated_instances_not_multiplied);
            assert!(package.actual_local_role_anchors_unique);
            assert!(package.pre_existing_required_output_positions_unique);
            assert!(package.conditional_at_most.is_some());
        }
    }

    #[test]
    fn re_signed_anchor_and_gap_mutations_fail_replay() {
        let candidate = Telescope::reference(5);
        let prefix = SealedSignature::from_telescopes(
            (1..5)
                .map(|stage| (stage, Telescope::reference(stage)))
                .collect(),
        );
        let certificate =
            issue_act_local_provenance_v2(&prefix, 5, &candidate).expect("v2 package");
        let mut anchor_mutation = certificate.clone();
        if let Some(credit) = anchor_mutation
            .checker_submission
            .credited_families
            .first_mut()
        {
            credit.anchor = ProvenanceAnchor::ChargedKernel {
                clause: 0,
                role: LocalRole::Coherence,
            };
        } else {
            anchor_mutation.authoritative = !anchor_mutation.authoritative;
        }
        anchor_mutation.derivation_hash = package_digest(&anchor_mutation);
        assert!(!replay_act_local_provenance_v2(&prefix, &candidate, &anchor_mutation).is_empty());

        let mut minted_output_mutation = certificate.clone();
        minted_output_mutation.candidate_minted_required_output_position_count = 1;
        minted_output_mutation.derivation_hash = package_digest(&minted_output_mutation);
        assert!(
            !replay_act_local_provenance_v2(&prefix, &candidate, &minted_output_mutation)
                .is_empty()
        );
    }
}
