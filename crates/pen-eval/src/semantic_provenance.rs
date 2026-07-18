//! Operational schema-family and debt-credit provenance audit.
//!
//! This module is the conditional-audit boundary between the legacy closed-form novelty
//! evaluator and the proposed extraction-guarded Selective Law.  It does not
//! reinterpret a focus-family label as semantic evidence.  Positive marginal
//! credit must be attached either to one of four finite roles owned by a
//! charged kernel clause, or to an individually enumerated demand orbit that
//! was live before the candidate was proposed.
//!
//! The current MBTT AST has no typed normalizer, univalent quotient, or
//! semantic demand-orbit extractor.  Consequently the historical replay at
//! the bottom of this module reproduces all fifteen numerical scores but
//! reports the J2/J3 bridge as incomplete.  That distinction is intentional:
//! a preserved score is not yet a proved provenance correspondence.

use crate::debt_guard::{directive_debt_timeline, DirectiveDebtRecord};
use crate::nu::structural_nu;
use pen_core::expr::Expr;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::{Telescope, TelescopeClass};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

/// The explicit Selective-Law strengthening stipulates this closed local-role
/// codomain. Its coefficient is enumerated blindly and is independent of
/// every bar, historical score, support size, path dimension, and survivor
/// list. Semantic exhaustiveness is a separate, currently assumed premise.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalRole {
    /// The irreducible public declaration itself.
    KernelHead,
    /// Its single adjoint/eliminative face.
    AdjointMate,
    /// One natural support action (P5 lift, P6 action, or synthesis action).
    SupportAction,
    /// One independent depth-two coherence/interchange family.
    Coherence,
}

impl LocalRole {
    pub const ALL: [Self; 4] = [
        Self::KernelHead,
        Self::AdjointMate,
        Self::SupportAction,
        Self::Coherence,
    ];
}

pub const fn blind_local_role_coefficient() -> u32 {
    LocalRole::ALL.len() as u32
}

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SchemaFamilyId(pub String);

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SchemaInstanceId(pub String);

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct UnivalentClassId(pub String);

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DemandOrbitId(pub String);

#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DemandOutputPositionId(pub String);

/// One point of the finite dependent sum `sum_o Req_H(o)`.
#[derive(Clone, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct DemandOutputRef {
    pub orbit: DemandOrbitId,
    pub position: DemandOutputPositionId,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticEvidenceStatus {
    /// A semantic theorem is named but has not been checked by this crate.
    Assumed,
    Missing,
    /// The premise is backed by a replayable kernel derivation
    /// (SEMANTIC_NORMALIZATION_PROGRAM Phase 1/2): `derivation_hash` on the
    /// carrying `SemanticAssumptionRef` names the BLAKE3 of the derivation.
    Verified,
}

/// A named semantic assumption.  With status `Assumed` this crate does not
/// verify the named result — the reference is only a traceable premise for
/// a conditional calculation.  With status `Verified` the premise carries
/// the BLAKE3 hash of a kernel derivation that replay can re-derive.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticAssumptionRef {
    pub obligation_id: String,
    pub status: SemanticEvidenceStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub derivation_hash: Option<String>,
}

impl SemanticAssumptionRef {
    pub fn assumed(obligation_id: impl Into<String>) -> Self {
        Self {
            obligation_id: obligation_id.into(),
            status: SemanticEvidenceStatus::Assumed,
            derivation_hash: None,
        }
    }

    pub fn missing(obligation: impl Into<String>) -> Self {
        Self {
            obligation_id: obligation.into(),
            status: SemanticEvidenceStatus::Missing,
            derivation_hash: None,
        }
    }

    /// Kernel-verified evidence: requires a non-empty derivation hash.
    pub fn verified(obligation_id: impl Into<String>, derivation_hash: impl Into<String>) -> Self {
        Self {
            obligation_id: obligation_id.into(),
            status: SemanticEvidenceStatus::Verified,
            derivation_hash: Some(derivation_hash.into()),
        }
    }

    pub fn is_supplied_assumption(&self) -> bool {
        match self.status {
            SemanticEvidenceStatus::Assumed => !self.obligation_id.trim().is_empty(),
            SemanticEvidenceStatus::Verified => self.is_kernel_verified(),
            SemanticEvidenceStatus::Missing => false,
        }
    }

    pub fn is_kernel_verified(&self) -> bool {
        self.status == SemanticEvidenceStatus::Verified
            && !self.obligation_id.trim().is_empty()
            && self
                .derivation_hash
                .as_ref()
                .is_some_and(|hash| hash.starts_with("blake3:"))
    }
}

/// A claimed typed beta/eta-normal judgement.  The strings are intended to be
/// canonical fingerprints from a future typed normalizer, not raw source;
/// today their semantic properties remain named assumptions.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TypedNormalJudgement {
    pub context_normal_form: String,
    pub term_normal_form: String,
    pub type_normal_form: String,
    pub normalization: SemanticAssumptionRef,
    pub type_preservation: SemanticAssumptionRef,
}

impl TypedNormalJudgement {
    pub fn assumptions_are_supplied(&self) -> bool {
        !self.context_normal_form.trim().is_empty()
            && !self.term_normal_form.trim().is_empty()
            && !self.type_normal_form.trim().is_empty()
            && self.normalization.is_supplied_assumption()
            && self.type_preservation.is_supplied_assumption()
    }
}

/// One schema is a natural family over its parameters.  Its specializations
/// are instances and do not become additional counting units by default.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TypedNormalFamily {
    pub id: SchemaFamilyId,
    pub judgement: TypedNormalJudgement,
    pub parameters: Vec<String>,
    pub naturality: SemanticAssumptionRef,
    pub univalent_class: UnivalentClassId,
}

impl TypedNormalFamily {
    pub fn assumptions_are_supplied(&self) -> bool {
        !self.id.0.trim().is_empty()
            && !self.univalent_class.0.trim().is_empty()
            && self.judgement.assumptions_are_supplied()
            && self.naturality.is_supplied_assumption()
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SchemaInstance {
    pub id: SchemaInstanceId,
    pub family: SchemaFamilyId,
    pub substitution_normal_form: String,
    /// Only an independently exported, pre-existing demand orbit can split a
    /// uniform specialization into its own counting unit.
    pub independently_exported_demand: Option<DemandOutputRef>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WeakeningWitness {
    pub source: SchemaFamilyId,
    pub weakened: SchemaFamilyId,
    pub substitution_normal_form: String,
    pub type_preservation: SemanticAssumptionRef,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct UnivalentEqualityWitness {
    pub left: SchemaFamilyId,
    pub right: SchemaFamilyId,
    pub equivalence_normal_form: String,
    pub forward_backward: SemanticAssumptionRef,
    pub computation_coherence: SemanticAssumptionRef,
}

impl UnivalentEqualityWitness {
    pub fn assumptions_are_supplied(&self) -> bool {
        !self.equivalence_normal_form.trim().is_empty()
            && self.forward_backward.is_supplied_assumption()
            && self.computation_coherence.is_supplied_assumption()
    }
}

/// Operational marginality.  A weakening image has zero marginal credit; a
/// positive family requires an explicit no-preimage theorem and a fresh head.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MarginalityWitness {
    WeakeningImage {
        source: SchemaFamilyId,
        weakening: WeakeningWitness,
        equality: UnivalentEqualityWitness,
    },
    NoWeakeningPreimage {
        fresh_kernel_clause: u16,
        no_preimage: SemanticAssumptionRef,
    },
}

impl MarginalityWitness {
    fn is_positive_under_supplied_assumption(&self) -> bool {
        match self {
            Self::WeakeningImage { .. } => false,
            Self::NoWeakeningPreimage { no_preimage, .. } => no_preimage.is_supplied_assumption(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct FamilyMapEdge {
    pub source: SchemaFamilyId,
    pub target: SchemaFamilyId,
    pub action: SemanticAssumptionRef,
}

/// Proof-carrying weakening/erasure data for an internal guarded flow.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GuardedInternalityAssumptions {
    pub flow_name: String,
    pub base_families: BTreeSet<SchemaFamilyId>,
    pub extension_families: BTreeSet<SchemaFamilyId>,
    /// Completeness prevents an empty or partial family list from proving
    /// conservativity vacuously.
    pub base_extraction_completeness: SemanticAssumptionRef,
    pub extension_extraction_completeness: SemanticAssumptionRef,
    pub weakening: Vec<FamilyMapEdge>,
    pub erasure: Vec<FamilyMapEdge>,
    pub erasure_after_weakening: SemanticAssumptionRef,
    pub weakening_after_erasure_up_to_univalence: SemanticAssumptionRef,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ConditionalConservativeInternality {
    pub flow_name: String,
    pub family_count: u32,
    pub marginal_nu: u32,
    pub weakening_erasure_are_inverse: bool,
}

/// Checks the finite maps and derives `nu = 0` conditionally on the named
/// extraction and inverse-law assumptions.  This crate cannot verify those
/// semantic assumptions.
pub fn derive_conditional_conservative_internality(
    assumptions: &GuardedInternalityAssumptions,
) -> Result<ConditionalConservativeInternality, ProvenanceError> {
    if assumptions.flow_name.trim().is_empty()
        || !assumptions
            .base_extraction_completeness
            .is_supplied_assumption()
        || !assumptions
            .extension_extraction_completeness
            .is_supplied_assumption()
        || !assumptions.erasure_after_weakening.is_supplied_assumption()
        || !assumptions
            .weakening_after_erasure_up_to_univalence
            .is_supplied_assumption()
    {
        return Err(ProvenanceError::IncompleteInternalityEvidence);
    }

    let weakening = checked_function(
        &assumptions.weakening,
        &assumptions.base_families,
        &assumptions.extension_families,
    )?;
    let erasure = checked_function(
        &assumptions.erasure,
        &assumptions.extension_families,
        &assumptions.base_families,
    )?;
    for (base, extension) in &weakening {
        if erasure.get(extension) != Some(base) {
            return Err(ProvenanceError::InverseLawMismatch);
        }
    }
    for (extension, base) in &erasure {
        if weakening.get(base) != Some(extension) {
            return Err(ProvenanceError::InverseLawMismatch);
        }
    }

    Ok(ConditionalConservativeInternality {
        flow_name: assumptions.flow_name.clone(),
        family_count: assumptions.extension_families.len() as u32,
        marginal_nu: 0,
        weakening_erasure_are_inverse: true,
    })
}

fn checked_function(
    edges: &[FamilyMapEdge],
    domain: &BTreeSet<SchemaFamilyId>,
    codomain: &BTreeSet<SchemaFamilyId>,
) -> Result<BTreeMap<SchemaFamilyId, SchemaFamilyId>, ProvenanceError> {
    let mut map = BTreeMap::new();
    for edge in edges {
        if !domain.contains(&edge.source)
            || !codomain.contains(&edge.target)
            || !edge.action.is_supplied_assumption()
            || map
                .insert(edge.source.clone(), edge.target.clone())
                .is_some()
        {
            return Err(ProvenanceError::InvalidFamilyMap);
        }
    }
    if map.len() != domain.len() {
        return Err(ProvenanceError::InvalidFamilyMap);
    }
    Ok(map)
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum OrbitResolution {
    /// Outstanding when the candidate at `stage` is selected.
    Live,
    /// Already derivable from the sealed library at the audited stage.
    Derivable {
        discharged_by_step: u32,
        derivability: SemanticAssumptionRef,
    },
}

/// An individual semantic demand orbit modulo substitution and univalent
/// equality.  `package` only records which coarse guard generated it; it is
/// never accepted as a replacement for this record.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticDemandOrbit {
    pub id: DemandOrbitId,
    pub package: String,
    pub generated_by_window: [u32; 2],
    pub normalized_demand_type: String,
    /// The finite independently required output positions `Req_H(o)`.
    pub required_outputs: Vec<DemandOutputPosition>,
    pub resolution: OrbitResolution,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DemandOutputPosition {
    pub id: DemandOutputPositionId,
    pub family_class: UnivalentClassId,
    pub normalized_output_type: String,
}

impl SemanticDemandOrbit {
    fn structurally_valid(&self) -> bool {
        let output_ids = self
            .required_outputs
            .iter()
            .map(|output| output.id.clone())
            .collect::<BTreeSet<_>>();
        !self.id.0.trim().is_empty()
            && !self.package.trim().is_empty()
            && !self.normalized_demand_type.trim().is_empty()
            && !self.required_outputs.is_empty()
            && output_ids.len() == self.required_outputs.len()
            && self.required_outputs.iter().all(|output| {
                !output.id.0.trim().is_empty()
                    && !output.family_class.0.trim().is_empty()
                    && !output.normalized_output_type.trim().is_empty()
            })
            && match &self.resolution {
                OrbitResolution::Live => true,
                OrbitResolution::Derivable { derivability, .. } => {
                    derivability.is_supplied_assumption()
                }
            }
    }
}

/// A submitted extraction of demand orbits generated by a depth-two window.
/// The completeness and derivability fields are assumptions until a typed
/// extractor/verifier replaces them with opaque checked certificates.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StageOrbitInventory {
    pub stage: u32,
    pub window: [u32; 2],
    pub orbits: Vec<SemanticDemandOrbit>,
    /// J2: no demand orbit generated by the window is omitted.
    pub extraction_completeness_assumption: SemanticAssumptionRef,
    /// J3: every extracted orbit's live/derivable disposition is decided.
    pub derivability_completeness_assumption: SemanticAssumptionRef,
    /// Locality/persistence: every older live demand is represented or
    /// transported into the active depth-two window.
    pub window_locality_assumption: SemanticAssumptionRef,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StageOrbitAudit {
    pub stage: u32,
    pub coarse_focus_packages: Vec<String>,
    pub focus_label_debt_free: bool,
    pub extracted_orbit_count: usize,
    pub live_orbit_count: usize,
    pub live_required_output_count: usize,
    pub j2_extraction_assumed: bool,
    pub j3_derivability_assumed: bool,
    pub depth_two_window_locality_assumed: bool,
    pub focus_orbit_correspondence_if_assumptions: bool,
    /// `None` means semantic emptiness is not decided.  In particular, an
    /// empty focus label does not produce `Some(true)` by itself.
    pub semantic_debt_empty_if_assumptions: Option<bool>,
    pub obligations: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct SemanticDebtAudit {
    pub stages: Vec<StageOrbitAudit>,
    pub inventory_input_valid: bool,
    pub rejected_inventory_stages: Vec<u32>,
    pub focus_label_o16_empty: bool,
    pub semantic_o16_empty_if_assumptions: Option<bool>,
    pub j2_assumed_at_16: bool,
    pub j3_assumed_at_16: bool,
    pub window_locality_assumed_at_16: bool,
    pub j2_j3_conditionally_sufficient_at_16: bool,
}

/// Refines the engine's package-label timeline with submitted semantic orbit
/// inventories. With no typed extractor, call this with an empty slice; the
/// result keeps O(16)'s label-level fact while refusing the semantic claim.
/// Nonempty inventories yield only conditional results from named assumptions.
pub fn audit_semantic_debt(inventories: &[StageOrbitInventory]) -> SemanticDebtAudit {
    let mut by_stage = BTreeMap::new();
    let mut rejected = BTreeSet::new();
    for inventory in inventories {
        if !(1..=16).contains(&inventory.stage) || rejected.contains(&inventory.stage) {
            rejected.insert(inventory.stage);
            by_stage.remove(&inventory.stage);
            continue;
        }
        if by_stage.insert(inventory.stage, inventory).is_some() {
            by_stage.remove(&inventory.stage);
            rejected.insert(inventory.stage);
        }
    }
    let stages = directive_debt_timeline()
        .iter()
        .map(|coarse| audit_stage_orbits(coarse, by_stage.get(&coarse.stage).copied()))
        .collect::<Vec<_>>();
    let o16 = stages.iter().find(|stage| stage.stage == 16);
    let focus_label_o16_empty = o16
        .map(|stage| stage.focus_label_debt_free)
        .unwrap_or(false);
    let semantic_o16_empty_if_assumptions =
        o16.and_then(|stage| stage.semantic_debt_empty_if_assumptions);
    let j2_assumed_at_16 = o16
        .map(|stage| stage.j2_extraction_assumed)
        .unwrap_or(false);
    let j3_assumed_at_16 = o16
        .map(|stage| stage.j3_derivability_assumed)
        .unwrap_or(false);
    let window_locality_assumed_at_16 = o16
        .map(|stage| stage.depth_two_window_locality_assumed)
        .unwrap_or(false);
    let j2_j3_conditionally_sufficient_at_16 = j2_assumed_at_16
        && j3_assumed_at_16
        && window_locality_assumed_at_16
        && semantic_o16_empty_if_assumptions == Some(true);
    SemanticDebtAudit {
        stages,
        inventory_input_valid: rejected.is_empty(),
        rejected_inventory_stages: rejected.into_iter().collect(),
        focus_label_o16_empty,
        semantic_o16_empty_if_assumptions,
        j2_assumed_at_16,
        j3_assumed_at_16,
        window_locality_assumed_at_16,
        j2_j3_conditionally_sufficient_at_16,
    }
}

fn audit_stage_orbits(
    coarse: &DirectiveDebtRecord,
    inventory: Option<&StageOrbitInventory>,
) -> StageOrbitAudit {
    let coarse_packages = coarse
        .required_packages
        .iter()
        .map(|package| (*package).to_owned())
        .collect::<Vec<_>>();
    let Some(inventory) = inventory else {
        return StageOrbitAudit {
            stage: coarse.stage,
            coarse_focus_packages: coarse_packages,
            focus_label_debt_free: coarse.debt_free,
            extracted_orbit_count: 0,
            live_orbit_count: 0,
            live_required_output_count: 0,
            j2_extraction_assumed: false,
            j3_derivability_assumed: false,
            depth_two_window_locality_assumed: false,
            focus_orbit_correspondence_if_assumptions: false,
            semantic_debt_empty_if_assumptions: None,
            obligations: vec![
                format!(
                    "J2({}): enumerate normalized demand orbits for the depth-two window",
                    coarse.stage
                ),
                format!(
                    "J3({}): decide derivability of every extracted orbit",
                    coarse.stage
                ),
            ],
        };
    };

    let expected_window = stage_window(coarse.stage);
    let orbit_ids = inventory
        .orbits
        .iter()
        .map(|orbit| orbit.id.clone())
        .collect::<BTreeSet<_>>();
    let structurally_valid = inventory.stage == coarse.stage
        && inventory.window == expected_window
        && orbit_ids.len() == inventory.orbits.len()
        && inventory.orbits.iter().all(|orbit| {
            orbit.generated_by_window == expected_window
                && orbit.structurally_valid()
                && match &orbit.resolution {
                    OrbitResolution::Live => true,
                    OrbitResolution::Derivable {
                        discharged_by_step, ..
                    } => *discharged_by_step < coarse.stage,
                }
        });
    let j2 = structurally_valid
        && inventory
            .extraction_completeness_assumption
            .is_supplied_assumption();
    let j3 = structurally_valid
        && inventory
            .derivability_completeness_assumption
            .is_supplied_assumption();
    let window_locality = inventory
        .window_locality_assumption
        .is_supplied_assumption();
    let live_packages = inventory
        .orbits
        .iter()
        .filter(|orbit| matches!(&orbit.resolution, OrbitResolution::Live))
        .map(|orbit| orbit.package.clone())
        .collect::<BTreeSet<_>>();
    let coarse_set = coarse_packages.iter().cloned().collect::<BTreeSet<_>>();
    let focus_correspondence = j2 && j3 && live_packages == coarse_set;
    let live_count = inventory
        .orbits
        .iter()
        .filter(|orbit| matches!(&orbit.resolution, OrbitResolution::Live))
        .count();
    let live_required_output_count = inventory
        .orbits
        .iter()
        .filter(|orbit| matches!(&orbit.resolution, OrbitResolution::Live))
        .map(|orbit| orbit.required_outputs.len())
        .sum();
    let semantic_debt_empty_if_assumptions =
        (j2 && j3 && window_locality).then_some(live_count == 0);
    let mut obligations = Vec::new();
    if !j2 {
        obligations.push(format!(
            "J2({}): extraction completeness missing or malformed",
            coarse.stage
        ));
    }
    if !j3 {
        obligations.push(format!(
            "J3({}): derivability completeness missing or malformed",
            coarse.stage
        ));
    }
    if !window_locality {
        obligations.push(format!(
            "stage {}: depth-two window locality/persistence assumption missing",
            coarse.stage
        ));
    }
    if j2 && j3 && !focus_correspondence {
        obligations.push(format!(
            "stage {}: live semantic orbit packages {:?} differ from focus packages {:?}",
            coarse.stage, live_packages, coarse_set
        ));
    }
    StageOrbitAudit {
        stage: coarse.stage,
        coarse_focus_packages: coarse_packages,
        focus_label_debt_free: coarse.debt_free,
        extracted_orbit_count: inventory.orbits.len(),
        live_orbit_count: live_count,
        live_required_output_count,
        j2_extraction_assumed: j2,
        j3_derivability_assumed: j3,
        depth_two_window_locality_assumed: window_locality,
        focus_orbit_correspondence_if_assumptions: focus_correspondence,
        semantic_debt_empty_if_assumptions,
        obligations,
    }
}

fn stage_window(stage: u32) -> [u32; 2] {
    [stage.saturating_sub(2), stage.saturating_sub(1)]
}

/// The mechanisms whose historical superlinear formulas must be reconciled
/// with provenance.  The rules below count no formula automatically.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditMechanism {
    IntrinsicKernel,
    AdjointCompletion,
    P5InheritedSurface,
    P5LocalAndBridge,
    P6UniformSpecialization,
    DimensionSquared,
    ReferenceSquared,
    DistributiveInheritance,
    CombinatorialSynthesis,
    ModalPairwiseCoherence,
}

impl CreditMechanism {
    fn required_local_role(self) -> LocalRole {
        match self {
            Self::IntrinsicKernel => LocalRole::KernelHead,
            Self::AdjointCompletion => LocalRole::AdjointMate,
            Self::P5InheritedSurface
            | Self::P5LocalAndBridge
            | Self::P6UniformSpecialization
            | Self::CombinatorialSynthesis => LocalRole::SupportAction,
            Self::DimensionSquared
            | Self::ReferenceSquared
            | Self::DistributiveInheritance
            | Self::ModalPairwiseCoherence => LocalRole::Coherence,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ProvenanceAnchor {
    ChargedKernel { clause: u16, role: LocalRole },
    PreExistingDemandOutput { demand: DemandOutputRef },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreditedSchemaFamily {
    pub family: TypedNormalFamily,
    pub instances: Vec<SchemaInstance>,
    pub marginality: MarginalityWitness,
    pub mechanism: CreditMechanism,
    pub anchor: ProvenanceAnchor,
    /// P0 / `Anchors(f, tag)`: the semantic relation between this exact
    /// family and its selected local role or demand output. This is currently
    /// a named assumption, not something this crate can verify.
    pub anchor_validity_assumption: SemanticAssumptionRef,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct CandidateProvenanceSubmission {
    pub candidate_stage: u32,
    pub kappa: u16,
    pub credited_families: Vec<CreditedSchemaFamily>,
    /// Completeness of the typed normal marginal-family enumeration.  Without
    /// it an omitted family could make any finite bound vacuous.
    pub family_extraction_completeness_assumption: SemanticAssumptionRef,
    /// The complete set of individual orbits available as credit anchors.
    pub demand_orbits: Vec<SemanticDemandOrbit>,
    pub orbit_inventory_completeness_assumption: SemanticAssumptionRef,
    /// Every older live demand is represented/transported into the active
    /// depth-two inventory used by this candidate.
    pub window_locality_assumption: SemanticAssumptionRef,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ConditionalAtMostBound {
    pub candidate_stage: u32,
    pub kappa: u16,
    pub local_role_coefficient: u32,
    pub local_capacity: u32,
    pub natural_family_credits: u32,
    pub exported_instance_credits: u32,
    pub local_credits: u32,
    pub debt_orbit_credits: u32,
    pub marginal_nu: u32,
    pub debt_free_if_inventory_exhaustive: bool,
    pub conditional_debt_free_linear_bound_holds: bool,
}

/// Checks the injective provenance map.  The same kernel/role slot or demand
/// demand-output position cannot fund two independent counting units.  A
/// natural family counts once; an instance counts additionally only when it
/// has its own position in `sum_o Req_H(o)`.  Thus a debt-free submission
/// injects into `Fin(kappa) x LocalRole` and conditionally obtains
/// `nu <= 4*kappa`.
pub fn derive_conditional_at_most(
    submission: &CandidateProvenanceSubmission,
) -> Result<ConditionalAtMostBound, ProvenanceError> {
    if submission.kappa == 0 {
        return Err(ProvenanceError::ZeroKappa);
    }
    if submission.candidate_stage == 0 {
        return Err(ProvenanceError::InvalidCandidateStage);
    }
    if !submission
        .family_extraction_completeness_assumption
        .is_supplied_assumption()
    {
        return Err(ProvenanceError::IncompleteFamilyExtraction);
    }
    let orbit_inventory_complete = submission
        .orbit_inventory_completeness_assumption
        .is_supplied_assumption();
    let window_locality_assumed = submission
        .window_locality_assumption
        .is_supplied_assumption();
    let orbit_map = submission
        .demand_orbits
        .iter()
        .map(|orbit| (orbit.id.clone(), orbit))
        .collect::<BTreeMap<_, _>>();
    if orbit_map.len() != submission.demand_orbits.len() {
        return Err(ProvenanceError::DuplicateDemandOrbit);
    }

    let mut family_ids = BTreeSet::new();
    let mut univalent_classes = BTreeSet::new();
    let mut normal_judgements = BTreeSet::new();
    let mut instance_ids = BTreeSet::new();
    let mut local_slots = BTreeSet::new();
    let mut orbit_slots = BTreeSet::new();
    let mut exported_instance_credits = 0_u32;
    for credit in &submission.credited_families {
        if !credit.family.assumptions_are_supplied()
            || !credit.marginality.is_positive_under_supplied_assumption()
            || !family_ids.insert(credit.family.id.clone())
            || !credit.anchor_validity_assumption.is_supplied_assumption()
        {
            return Err(ProvenanceError::InvalidCreditedFamily);
        }
        let normal_key = (
            credit.family.judgement.context_normal_form.clone(),
            credit.family.judgement.term_normal_form.clone(),
            credit.family.judgement.type_normal_form.clone(),
        );
        if !univalent_classes.insert(credit.family.univalent_class.clone())
            || !normal_judgements.insert(normal_key)
        {
            return Err(ProvenanceError::DuplicateSemanticFamily);
        }
        let MarginalityWitness::NoWeakeningPreimage {
            fresh_kernel_clause,
            ..
        } = &credit.marginality
        else {
            return Err(ProvenanceError::InvalidCreditedFamily);
        };
        if u32::from(*fresh_kernel_clause) >= u32::from(submission.kappa) {
            return Err(ProvenanceError::InvalidCreditedFamily);
        }
        for instance in &credit.instances {
            if instance.family != credit.family.id
                || instance.id.0.trim().is_empty()
                || instance.substitution_normal_form.trim().is_empty()
                || !instance_ids.insert(instance.id.clone())
            {
                return Err(ProvenanceError::InvalidSchemaInstance);
            }
            if let Some(instance_demand) = &instance.independently_exported_demand {
                validate_demand_output_ref(
                    instance_demand,
                    &credit.family,
                    submission.candidate_stage,
                    orbit_inventory_complete,
                    window_locality_assumed,
                    &orbit_map,
                )?;
                if !orbit_slots.insert(instance_demand.clone()) {
                    return Err(ProvenanceError::DuplicateDemandAnchor);
                }
                exported_instance_credits = exported_instance_credits.saturating_add(1);
            }
        }

        match &credit.anchor {
            ProvenanceAnchor::ChargedKernel { clause, role } => {
                if u32::from(*clause) >= u32::from(submission.kappa)
                    || clause != fresh_kernel_clause
                    || *role != credit.mechanism.required_local_role()
                {
                    return Err(ProvenanceError::InvalidLocalAnchor);
                }
                if !local_slots.insert((*clause, *role)) {
                    return Err(ProvenanceError::DuplicateLocalAnchor);
                }
            }
            ProvenanceAnchor::PreExistingDemandOutput { demand } => {
                validate_demand_output_ref(
                    demand,
                    &credit.family,
                    submission.candidate_stage,
                    orbit_inventory_complete,
                    window_locality_assumed,
                    &orbit_map,
                )?;
                if !orbit_slots.insert(demand.clone()) {
                    return Err(ProvenanceError::DuplicateDemandAnchor);
                }
            }
        }
    }

    let coefficient = blind_local_role_coefficient();
    let local_capacity = coefficient.saturating_mul(u32::from(submission.kappa));
    let local_credits = local_slots.len() as u32;
    let debt_orbit_credits = orbit_slots.len() as u32;
    let natural_family_credits = submission.credited_families.len() as u32;
    let marginal_nu = natural_family_credits.saturating_add(exported_instance_credits);
    let debt_free = orbit_inventory_complete
        && window_locality_assumed
        && submission
            .demand_orbits
            .iter()
            .all(|orbit| !matches!(&orbit.resolution, OrbitResolution::Live));
    let debt_free_linear_bound_holds = debt_free && marginal_nu <= local_capacity;
    if debt_free && marginal_nu > local_capacity {
        return Err(ProvenanceError::BlindLocalBoundViolation);
    }
    Ok(ConditionalAtMostBound {
        candidate_stage: submission.candidate_stage,
        kappa: submission.kappa,
        local_role_coefficient: coefficient,
        local_capacity,
        natural_family_credits,
        exported_instance_credits,
        local_credits,
        debt_orbit_credits,
        marginal_nu,
        debt_free_if_inventory_exhaustive: debt_free,
        conditional_debt_free_linear_bound_holds: debt_free_linear_bound_holds,
    })
}

fn validate_demand_output_ref(
    demand: &DemandOutputRef,
    family: &TypedNormalFamily,
    candidate_stage: u32,
    orbit_inventory_assumed_complete: bool,
    window_locality_assumed: bool,
    orbit_map: &BTreeMap<DemandOrbitId, &SemanticDemandOrbit>,
) -> Result<(), ProvenanceError> {
    if !orbit_inventory_assumed_complete {
        return Err(ProvenanceError::IncompleteOrbitInventory);
    }
    if !window_locality_assumed {
        return Err(ProvenanceError::MissingWindowLocalityAssumption);
    }
    let Some(witness) = orbit_map.get(&demand.orbit) else {
        return Err(ProvenanceError::MissingDemandOrbit);
    };
    let Some(output) = witness
        .required_outputs
        .iter()
        .find(|output| output.id == demand.position)
    else {
        return Err(ProvenanceError::MissingDemandOutputPosition);
    };
    if !witness.structurally_valid()
        || !matches!(&witness.resolution, OrbitResolution::Live)
        || witness.generated_by_window != stage_window(candidate_stage)
        || output.family_class != family.univalent_class
    {
        return Err(ProvenanceError::DemandOrbitNotPreExistingAndLive);
    }
    if output.normalized_output_type != family.judgement.type_normal_form {
        return Err(ProvenanceError::DemandOutputTypeMismatch);
    }
    Ok(())
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ProvenanceError {
    #[error("the guarded internality assumption set is incomplete")]
    IncompleteInternalityEvidence,
    #[error("a weakening or erasure edge is missing, duplicated, or lacks its assumption")]
    InvalidFamilyMap,
    #[error("weakening and erasure are not inverse finite maps")]
    InverseLawMismatch,
    #[error("kappa must be positive")]
    ZeroKappa,
    #[error("candidate_stage must be positive")]
    InvalidCandidateStage,
    #[error("typed normal marginal-family extraction completeness was not assumed")]
    IncompleteFamilyExtraction,
    #[error("the same semantic demand orbit was submitted twice")]
    DuplicateDemandOrbit,
    #[error("a credited family is untyped, non-marginal, or duplicated")]
    InvalidCreditedFamily,
    #[error("two credited IDs denote the same univalent class or canonical normal family")]
    DuplicateSemanticFamily,
    #[error("a schema instance is malformed or belongs to another family")]
    InvalidSchemaInstance,
    #[error("the kernel anchor is out of range or uses the wrong local role")]
    InvalidLocalAnchor,
    #[error("two independent families use the same kernel/local-role slot")]
    DuplicateLocalAnchor,
    #[error("demand-orbit credit requires an assumed-exhaustive individual-orbit inventory")]
    IncompleteOrbitInventory,
    #[error("demand credit and debt-free status require the depth-two window-locality assumption")]
    MissingWindowLocalityAssumption,
    #[error("the referenced demand orbit is absent")]
    MissingDemandOrbit,
    #[error("the referenced finite required-output position is absent from the demand orbit")]
    MissingDemandOutputPosition,
    #[error("the demand output's normalized type does not match the credited family type")]
    DemandOutputTypeMismatch,
    #[error("the demand orbit was not live before the candidate was proposed")]
    DemandOrbitNotPreExistingAndLive,
    #[error("two independent families use the same demand-orbit output position")]
    DuplicateDemandAnchor,
    #[error("the debt-free submission exceeds the blind finite local-role bound")]
    BlindLocalBoundViolation,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HistoricalAmplificationClaim {
    pub mechanism: CreditMechanism,
    pub claimed_units: u32,
    pub legacy_formula: String,
    pub coarse_focus_packages: Vec<String>,
    /// Always false in the built-in replay until operational families and
    /// individual orbit IDs are extracted from the typed semantics.
    pub orbit_correspondence_available: bool,
    pub obligation: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct HistoricalStepProvenance {
    pub step: u32,
    pub class: TelescopeClass,
    pub kappa: u32,
    pub direct_support: Vec<u32>,
    pub nu_g: u32,
    pub nu_h: u32,
    pub nu_c: u32,
    pub nu_total: u32,
    pub numerical_score_preserved: bool,
    /// A score under the strengthened Selective Law exists only after every
    /// counted family has passed the injective provenance audit.
    pub revised_audited_nu: Option<u32>,
    pub revised_audit_status: RevisedAuditStatus,
    pub coarse_focus_packages: Vec<String>,
    pub schema_family_extraction_available: bool,
    pub orbit_inventory_available: bool,
    pub amplification_claims: Vec<HistoricalAmplificationClaim>,
    pub all_credited_families_provenanced: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RevisedAuditStatus {
    IncompleteTypedFamilyAndOrbitExtraction,
    ProvenanceVerified,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct GenesisProvenanceReplay {
    pub local_role_coefficient: u32,
    pub coefficient_enumerated_without_bar: bool,
    pub steps: Vec<HistoricalStepProvenance>,
    pub numerical_replay_complete: bool,
    pub expected_totals: Vec<u32>,
    pub recomputed_totals: Vec<u32>,
    pub all_historical_credit_provenanced: bool,
    pub j2_window_orbit_extraction_available: bool,
    pub j3_orbit_derivability_available: bool,
    pub unresolved_obligations: Vec<String>,
}

/// Recompute the canonical fifteen-step scores and expose every historical
/// amplification formula that still needs a family-by-family provenance map.
/// No focus label is converted into an orbit witness.
pub fn replay_genesis_provenance() -> GenesisProvenanceReplay {
    const EXPECTED: [u32; 15] = [1, 1, 2, 5, 7, 8, 10, 18, 17, 19, 26, 34, 46, 62, 103];
    let coarse = directive_debt_timeline()
        .into_iter()
        .map(|record| (record.stage, record))
        .collect::<BTreeMap<_, _>>();
    let mut library: Library = Vec::new();
    let mut history = Vec::new();
    let mut steps = Vec::new();

    for step in 1..=15_u32 {
        let telescope = Telescope::reference(step);
        let class = telescope.classify(&library);
        let result = structural_nu(&telescope, &library, &history);
        let focus_packages = coarse
            .get(&step)
            .map(|record| {
                record
                    .required_packages
                    .iter()
                    .map(|package| (*package).to_owned())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        let amplification_claims = historical_amplifications(
            &telescope,
            class,
            &library,
            &history,
            &result,
            &focus_packages,
        );
        let index = (step - 1) as usize;
        steps.push(HistoricalStepProvenance {
            step,
            class,
            kappa: telescope.kappa() as u32,
            direct_support: telescope.lib_refs().into_iter().collect(),
            nu_g: result.nu_g,
            nu_h: result.nu_h,
            nu_c: result.nu_c,
            nu_total: result.total,
            numerical_score_preserved: result.total == EXPECTED[index],
            revised_audited_nu: None,
            revised_audit_status: RevisedAuditStatus::IncompleteTypedFamilyAndOrbitExtraction,
            coarse_focus_packages: focus_packages,
            schema_family_extraction_available: false,
            orbit_inventory_available: false,
            amplification_claims,
            all_credited_families_provenanced: false,
        });
        library.push(LibraryEntry::from_telescope(&telescope, &library));
        history.push((step, result.total));
    }

    let recomputed_totals = steps.iter().map(|step| step.nu_total).collect::<Vec<_>>();
    let numerical_replay_complete = recomputed_totals == EXPECTED;
    GenesisProvenanceReplay {
        local_role_coefficient: blind_local_role_coefficient(),
        coefficient_enumerated_without_bar: true,
        steps,
        numerical_replay_complete,
        expected_totals: EXPECTED.to_vec(),
        recomputed_totals,
        all_historical_credit_provenanced: false,
        j2_window_orbit_extraction_available: false,
        j3_orbit_derivability_available: false,
        unresolved_obligations: vec![
            "J2: implement typed depth-two demand-orbit extraction for every historical window, especially (S14,S15)".to_owned(),
            "J3: decide each extracted orbit's derivability modulo weakening and univalent equality".to_owned(),
            "enumerate normalized historical schema families and inject every credited unit into a charged local role or pre-existing live orbit".to_owned(),
            "construct the guarded Step-15 weakening/erasure inverse certificate before claiming conservative internality".to_owned(),
        ],
    }
}

fn historical_amplifications(
    telescope: &Telescope,
    class: TelescopeClass,
    library: &Library,
    history: &[(u32, u32)],
    result: &crate::nu::StructuralNuResult,
    focus_packages: &[String],
) -> Vec<HistoricalAmplificationClaim> {
    let mut claims = Vec::new();
    let d_squared = telescope
        .path_dimensions()
        .into_iter()
        .map(|dimension| dimension.saturating_mul(dimension))
        .sum::<u32>();
    if d_squared > 0 {
        claims.push(unproved_claim(
            CreditMechanism::DimensionSquared,
            d_squared,
            "sum over path constructors of d_i^2",
            focus_packages,
        ));
    }

    let r = telescope.lib_refs().len() as u32;
    if class == TelescopeClass::Map && telescope.kappa() > 1 {
        claims.push(unproved_claim(
            CreditMechanism::AdjointCompletion,
            2_u32.saturating_mul(telescope.kappa() as u32),
            "two pre/post composition faces per map clause",
            focus_packages,
        ));
        if r > 0 {
            claims.push(unproved_claim(
                CreditMechanism::ReferenceSquared,
                r.saturating_mul(r),
                "ordered r x r interface transport matrix",
                focus_packages,
            ));
        }
    }
    if class == TelescopeClass::Axiomatic {
        let inherited = telescope
            .lib_refs()
            .into_iter()
            .filter_map(|reference| history.iter().find(|(step, _)| *step == reference))
            .map(|(_, nu)| *nu)
            .max()
            .unwrap_or(0);
        if inherited > 0 {
            claims.push(unproved_claim(
                CreditMechanism::P5InheritedSurface,
                inherited,
                "nu of reachability-dominant historical import",
                focus_packages,
            ));
        }
        claims.push(unproved_claim(
            CreditMechanism::P5LocalAndBridge,
            (telescope.kappa() as u32).saturating_add(r.saturating_sub(1)),
            "one local eliminative face per clause plus a support spanning tree",
            focus_packages,
        ));
    }
    if class == TelescopeClass::Modal && !library.is_empty() {
        claims.push(unproved_claim(
            CreditMechanism::P6UniformSpecialization,
            library.len() as u32,
            "one legacy schema per sealed library entry",
            focus_packages,
        ));
        let kinds = top_level_modal_kind_count(telescope);
        let pairwise = kinds.saturating_mul(kinds.saturating_sub(1)) / 2;
        if pairwise > 0 {
            claims.push(unproved_claim(
                CreditMechanism::ModalPairwiseCoherence,
                pairwise,
                "unordered pairwise coherence among distinct modal kinds",
                focus_packages,
            ));
        }
    }
    if result.universe_polymorphism_bonus > 0 {
        claims.push(unproved_claim(
            CreditMechanism::P6UniformSpecialization,
            result.universe_polymorphism_bonus,
            "polymorphic temporal eliminators times library size",
            focus_packages,
        ));
    }
    if result.distributive_law_bonus > 0 {
        claims.push(unproved_claim(
            CreditMechanism::DistributiveInheritance,
            result.distributive_law_bonus,
            "distributive laws times inherited modal novelty",
            focus_packages,
        ));
    }
    if result.infinitesimal_shift_bonus > 0 {
        claims.push(unproved_claim(
            CreditMechanism::CombinatorialSynthesis,
            result.infinitesimal_shift_bonus,
            "temporal shift inherits the loop-bearing library's d_i^2 payload",
            focus_packages,
        ));
    }
    claims
}

fn top_level_modal_kind_count(telescope: &Telescope) -> u32 {
    let has_flat = telescope
        .clauses
        .iter()
        .any(|clause| matches!(&clause.expr, Expr::Flat(_)));
    let has_sharp = telescope
        .clauses
        .iter()
        .any(|clause| matches!(&clause.expr, Expr::Sharp(_)));
    let has_disc = telescope
        .clauses
        .iter()
        .any(|clause| matches!(&clause.expr, Expr::Disc(_)));
    let has_shape = telescope
        .clauses
        .iter()
        .any(|clause| matches!(&clause.expr, Expr::Shape(_)));
    [has_flat, has_sharp, has_disc, has_shape]
        .into_iter()
        .map(u32::from)
        .sum()
}

fn unproved_claim(
    mechanism: CreditMechanism,
    claimed_units: u32,
    formula: &str,
    focus_packages: &[String],
) -> HistoricalAmplificationClaim {
    HistoricalAmplificationClaim {
        mechanism,
        claimed_units,
        legacy_formula: formula.to_owned(),
        coarse_focus_packages: focus_packages.to_vec(),
        orbit_correspondence_available: false,
        obligation: "enumerate the natural families, quotient uniform instances, and give each independent family a unique charged local role or pre-existing semantic demand orbit; the focus label alone is not a witness".to_owned(),
    }
}

/// Machine-readable statement of the unavailable semantic bridge for the
/// Step-15-generated Step-16 internal flows over sealed B15. It is an
/// obligation, not a certificate.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Step15InternalityObligation {
    pub flow: String,
    pub typed_family_extraction_available: bool,
    pub weakening_map_available: bool,
    pub erasure_map_available: bool,
    pub inverse_laws_available: bool,
    pub conclusion_nu_zero_proved: bool,
    pub required_theorem: String,
}

pub fn guarded_step15_internality_obligation() -> Step15InternalityObligation {
    Step15InternalityObligation {
        flow: "Step-15-generated guarded flow over sealed B15 (the Step-16 internal branch)"
            .to_owned(),
        typed_family_extraction_available: false,
        weakening_map_available: false,
        erasure_map_available: false,
        inverse_laws_available: false,
        conclusion_nu_zero_proved: false,
        required_theorem: "construct typed normal schema families for sealed B15 and every Step-15-generated guarded Step-16 internal candidate, then prove erasure after weakening is identity and weakening after erasure is univalently equal to identity".to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assumption(name: &str) -> SemanticAssumptionRef {
        SemanticAssumptionRef::assumed(name)
    }

    fn family(name: &str) -> TypedNormalFamily {
        TypedNormalFamily {
            id: SchemaFamilyId(name.to_owned()),
            judgement: TypedNormalJudgement {
                context_normal_form: "Gamma".to_owned(),
                term_normal_form: format!("nf({name})"),
                type_normal_form: "Schema".to_owned(),
                normalization: assumption("normalization"),
                type_preservation: assumption("subject-reduction"),
            },
            parameters: vec!["A:U".to_owned()],
            naturality: assumption("naturality"),
            univalent_class: UnivalentClassId(format!("u:{name}")),
        }
    }

    fn marginal(clause: u16) -> MarginalityWitness {
        MarginalityWitness::NoWeakeningPreimage {
            fresh_kernel_clause: clause,
            no_preimage: assumption("no-weakening-preimage"),
        }
    }

    fn mechanism_for(role: LocalRole) -> CreditMechanism {
        match role {
            LocalRole::KernelHead => CreditMechanism::IntrinsicKernel,
            LocalRole::AdjointMate => CreditMechanism::AdjointCompletion,
            LocalRole::SupportAction => CreditMechanism::P6UniformSpecialization,
            LocalRole::Coherence => CreditMechanism::DimensionSquared,
        }
    }

    fn local_credit(name: &str, clause: u16, role: LocalRole) -> CreditedSchemaFamily {
        CreditedSchemaFamily {
            family: family(name),
            instances: Vec::new(),
            marginality: marginal(clause),
            mechanism: mechanism_for(role),
            anchor: ProvenanceAnchor::ChargedKernel { clause, role },
            anchor_validity_assumption: assumption("Anchors(f,tag)"),
        }
    }

    fn assumed_complete_submission(
        kappa: u16,
        credits: Vec<CreditedSchemaFamily>,
    ) -> CandidateProvenanceSubmission {
        CandidateProvenanceSubmission {
            candidate_stage: 16,
            kappa,
            credited_families: credits,
            family_extraction_completeness_assumption: assumption("all-marginal-families"),
            demand_orbits: Vec::new(),
            orbit_inventory_completeness_assumption: assumption("exhaustive-empty-orbit-inventory"),
            window_locality_assumption: assumption("all-live-debt-is-window-local"),
        }
    }

    #[test]
    fn blind_role_coefficient_is_four_and_has_no_bar_input() {
        assert_eq!(blind_local_role_coefficient(), 4);
        assert_eq!(LocalRole::ALL.len(), 4);
    }

    #[test]
    fn all_four_roles_per_clause_derive_the_conditional_debt_free_bound() {
        let mut credits = Vec::new();
        for clause in 0..2_u16 {
            for role in LocalRole::ALL {
                credits.push(local_credit(&format!("f-{clause}-{role:?}"), clause, role));
            }
        }
        let theorem = derive_conditional_at_most(&assumed_complete_submission(2, credits))
            .expect("eight distinct local slots");
        assert!(theorem.debt_free_if_inventory_exhaustive);
        assert_eq!(theorem.local_role_coefficient, 4);
        assert_eq!(theorem.local_capacity, 8);
        assert_eq!(theorem.marginal_nu, 8);
        assert!(theorem.conditional_debt_free_linear_bound_holds);
    }

    #[test]
    fn a_ninth_local_family_collides_instead_of_creating_superlinear_credit() {
        let mut credits = Vec::new();
        for clause in 0..2_u16 {
            for role in LocalRole::ALL {
                credits.push(local_credit(&format!("f-{clause}-{role:?}"), clause, role));
            }
        }
        credits.push(local_credit("ninth", 0, LocalRole::Coherence));
        assert_eq!(
            derive_conditional_at_most(&assumed_complete_submission(2, credits)),
            Err(ProvenanceError::DuplicateLocalAnchor)
        );
    }

    #[test]
    fn p5_p6_d_squared_r_squared_and_synthesis_obey_role_injection() {
        for (mechanism, correct_role) in [
            (
                CreditMechanism::P5InheritedSurface,
                LocalRole::SupportAction,
            ),
            (CreditMechanism::P5LocalAndBridge, LocalRole::SupportAction),
            (
                CreditMechanism::P6UniformSpecialization,
                LocalRole::SupportAction,
            ),
            (CreditMechanism::DimensionSquared, LocalRole::Coherence),
            (CreditMechanism::ReferenceSquared, LocalRole::Coherence),
            (
                CreditMechanism::CombinatorialSynthesis,
                LocalRole::SupportAction,
            ),
            (
                CreditMechanism::ModalPairwiseCoherence,
                LocalRole::Coherence,
            ),
        ] {
            let mut credit = local_credit(&format!("{mechanism:?}"), 0, correct_role);
            credit.mechanism = mechanism;
            derive_conditional_at_most(&assumed_complete_submission(1, vec![credit.clone()]))
                .expect("the mechanism uses its designated paid role");
            credit.anchor = ProvenanceAnchor::ChargedKernel {
                clause: 0,
                role: LocalRole::KernelHead,
            };
            assert_eq!(
                derive_conditional_at_most(&assumed_complete_submission(1, vec![credit])),
                Err(ProvenanceError::InvalidLocalAnchor)
            );
        }
    }

    #[test]
    fn a_local_role_must_be_owned_by_the_familys_actual_fresh_clause() {
        let mut credit = local_credit("misanchored", 1, LocalRole::KernelHead);
        credit.marginality = marginal(0);
        assert_eq!(
            derive_conditional_at_most(&assumed_complete_submission(2, vec![credit])),
            Err(ProvenanceError::InvalidLocalAnchor)
        );
    }

    #[test]
    fn every_family_tag_edge_requires_the_explicit_p0_anchor_assumption() {
        let mut credit = local_credit("p0-missing", 0, LocalRole::KernelHead);
        credit.anchor_validity_assumption = SemanticAssumptionRef::missing("Anchors(f,tag)");
        assert_eq!(
            derive_conditional_at_most(&assumed_complete_submission(1, vec![credit])),
            Err(ProvenanceError::InvalidCreditedFamily)
        );
    }

    #[test]
    fn different_ids_cannot_double_count_one_univalent_family() {
        let first = local_credit("first-id", 0, LocalRole::KernelHead);
        let mut second = local_credit("second-id", 0, LocalRole::AdjointMate);
        second.family.univalent_class = first.family.univalent_class.clone();
        assert_eq!(
            derive_conditional_at_most(&assumed_complete_submission(1, vec![first, second])),
            Err(ProvenanceError::DuplicateSemanticFamily)
        );
    }

    #[test]
    fn uniform_instances_are_one_family_without_exported_demand_orbits() {
        let mut credit = local_credit("uniform-action", 0, LocalRole::SupportAction);
        credit.mechanism = CreditMechanism::P6UniformSpecialization;
        credit.instances = (0..20)
            .map(|index| SchemaInstance {
                id: SchemaInstanceId(format!("instance-{index}")),
                family: credit.family.id.clone(),
                substitution_normal_form: format!("A := B{index}"),
                independently_exported_demand: None,
            })
            .collect();
        let theorem = derive_conditional_at_most(&assumed_complete_submission(1, vec![credit]))
            .expect("uniform instances do not multiply their natural family");
        assert_eq!(theorem.marginal_nu, 1);
        assert_eq!(theorem.exported_instance_credits, 0);
    }

    #[test]
    fn an_instance_cannot_claim_independent_credit_from_an_absent_demand_output() {
        let mut credit = local_credit("uniform-action", 0, LocalRole::SupportAction);
        credit.instances.push(SchemaInstance {
            id: SchemaInstanceId("specialization".to_owned()),
            family: credit.family.id.clone(),
            substitution_normal_form: "A := B15".to_owned(),
            independently_exported_demand: Some(DemandOutputRef {
                orbit: DemandOrbitId("orbit-15".to_owned()),
                position: DemandOutputPositionId("output".to_owned()),
            }),
        });
        assert_eq!(
            derive_conditional_at_most(&assumed_complete_submission(1, vec![credit])),
            Err(ProvenanceError::MissingDemandOrbit)
        );
    }

    #[test]
    fn a_preexisting_live_orbit_can_anchor_one_independent_specialization() {
        let mut credit = local_credit("exported-action", 0, LocalRole::SupportAction);
        credit.mechanism = CreditMechanism::P6UniformSpecialization;
        let orbit_id = DemandOrbitId("temporal-action-at-B14".to_owned());
        let demand = DemandOutputRef {
            orbit: orbit_id.clone(),
            position: DemandOutputPositionId("action-output".to_owned()),
        };
        credit.instances.push(SchemaInstance {
            id: SchemaInstanceId("exported-instance".to_owned()),
            family: credit.family.id.clone(),
            substitution_normal_form: "A := B14".to_owned(),
            independently_exported_demand: Some(demand),
        });
        let orbit = SemanticDemandOrbit {
            id: orbit_id,
            package: "temporal_shell".to_owned(),
            generated_by_window: [13, 14],
            normalized_demand_type: "TemporalAction(B14)".to_owned(),
            required_outputs: vec![DemandOutputPosition {
                id: DemandOutputPositionId("action-output".to_owned()),
                family_class: credit.family.univalent_class.clone(),
                normalized_output_type: "Schema".to_owned(),
            }],
            resolution: OrbitResolution::Live,
        };
        let theorem = derive_conditional_at_most(&CandidateProvenanceSubmission {
            candidate_stage: 15,
            kappa: 1,
            credited_families: vec![credit],
            family_extraction_completeness_assumption: assumption("all-marginal-families"),
            demand_orbits: vec![orbit],
            orbit_inventory_completeness_assumption: assumption("all-window-orbits"),
            window_locality_assumption: assumption("all-live-debt-is-window-local"),
        })
        .expect("individual live orbit is a valid provenance anchor");
        assert_eq!(theorem.debt_orbit_credits, 1);
        assert_eq!(theorem.natural_family_credits, 1);
        assert_eq!(theorem.exported_instance_credits, 1);
        assert_eq!(theorem.marginal_nu, 2);
        assert!(!theorem.debt_free_if_inventory_exhaustive);
        assert!(!theorem.conditional_debt_free_linear_bound_holds);
    }

    #[test]
    fn demand_output_type_must_match_the_family_normal_type() {
        let mut credit = local_credit("typed-output", 0, LocalRole::SupportAction);
        let demand = DemandOutputRef {
            orbit: DemandOrbitId("typed-orbit".to_owned()),
            position: DemandOutputPositionId("position".to_owned()),
        };
        credit.instances.push(SchemaInstance {
            id: SchemaInstanceId("typed-instance".to_owned()),
            family: credit.family.id.clone(),
            substitution_normal_form: "A := B14".to_owned(),
            independently_exported_demand: Some(demand.clone()),
        });
        let orbit = SemanticDemandOrbit {
            id: demand.orbit,
            package: "temporal_shell".to_owned(),
            generated_by_window: [13, 14],
            normalized_demand_type: "TemporalAction".to_owned(),
            required_outputs: vec![DemandOutputPosition {
                id: demand.position,
                family_class: credit.family.univalent_class.clone(),
                normalized_output_type: "WrongType".to_owned(),
            }],
            resolution: OrbitResolution::Live,
        };
        assert_eq!(
            derive_conditional_at_most(&CandidateProvenanceSubmission {
                candidate_stage: 15,
                kappa: 1,
                credited_families: vec![credit],
                family_extraction_completeness_assumption: assumption("all-families"),
                demand_orbits: vec![orbit],
                orbit_inventory_completeness_assumption: assumption("all-orbits"),
                window_locality_assumption: assumption("all-live-debt-is-window-local"),
            }),
            Err(ProvenanceError::DemandOutputTypeMismatch)
        );
    }

    #[test]
    fn demand_orbit_must_be_the_candidates_active_depth_two_window() {
        let mut credit = local_credit("windowed-output", 0, LocalRole::SupportAction);
        let demand = DemandOutputRef {
            orbit: DemandOrbitId("malformed-window".to_owned()),
            position: DemandOutputPositionId("position".to_owned()),
        };
        credit.instances.push(SchemaInstance {
            id: SchemaInstanceId("windowed-instance".to_owned()),
            family: credit.family.id.clone(),
            substitution_normal_form: "A := B14".to_owned(),
            independently_exported_demand: Some(demand.clone()),
        });
        let orbit = SemanticDemandOrbit {
            id: demand.orbit,
            package: "temporal_shell".to_owned(),
            generated_by_window: [999, 0],
            normalized_demand_type: "TemporalAction".to_owned(),
            required_outputs: vec![DemandOutputPosition {
                id: demand.position,
                family_class: credit.family.univalent_class.clone(),
                normalized_output_type: "Schema".to_owned(),
            }],
            resolution: OrbitResolution::Live,
        };
        assert_eq!(
            derive_conditional_at_most(&CandidateProvenanceSubmission {
                candidate_stage: 15,
                kappa: 1,
                credited_families: vec![credit],
                family_extraction_completeness_assumption: assumption("all-families"),
                demand_orbits: vec![orbit],
                orbit_inventory_completeness_assumption: assumption("all-orbits"),
                window_locality_assumption: assumption("all-live-debt-is-window-local"),
            }),
            Err(ProvenanceError::DemandOrbitNotPreExistingAndLive)
        );
    }

    #[test]
    fn demand_anchor_requires_window_locality_in_the_candidate_submission() {
        let mut credit = local_credit("locality-output", 0, LocalRole::SupportAction);
        let demand = DemandOutputRef {
            orbit: DemandOrbitId("locality-orbit".to_owned()),
            position: DemandOutputPositionId("position".to_owned()),
        };
        credit.anchor = ProvenanceAnchor::PreExistingDemandOutput {
            demand: demand.clone(),
        };
        let orbit = SemanticDemandOrbit {
            id: demand.orbit,
            package: "temporal_shell".to_owned(),
            generated_by_window: [13, 14],
            normalized_demand_type: "TemporalAction".to_owned(),
            required_outputs: vec![DemandOutputPosition {
                id: demand.position,
                family_class: credit.family.univalent_class.clone(),
                normalized_output_type: "Schema".to_owned(),
            }],
            resolution: OrbitResolution::Live,
        };
        assert_eq!(
            derive_conditional_at_most(&CandidateProvenanceSubmission {
                candidate_stage: 15,
                kappa: 1,
                credited_families: vec![credit],
                family_extraction_completeness_assumption: assumption("all-families"),
                demand_orbits: vec![orbit],
                orbit_inventory_completeness_assumption: assumption("all-orbits"),
                window_locality_assumption: SemanticAssumptionRef::missing("window locality"),
            }),
            Err(ProvenanceError::MissingWindowLocalityAssumption)
        );
    }

    #[test]
    fn one_orbit_can_fund_distinct_finitely_enumerated_required_outputs() {
        let orbit_id = DemandOrbitId("one-orbit".to_owned());
        let mut first = local_credit("first-output-family", 0, LocalRole::SupportAction);
        let mut second = local_credit("second-output-family", 0, LocalRole::SupportAction);
        first.mechanism = CreditMechanism::P5InheritedSurface;
        second.mechanism = CreditMechanism::P6UniformSpecialization;
        first.anchor = ProvenanceAnchor::PreExistingDemandOutput {
            demand: DemandOutputRef {
                orbit: orbit_id.clone(),
                position: DemandOutputPositionId("first".to_owned()),
            },
        };
        second.anchor = ProvenanceAnchor::PreExistingDemandOutput {
            demand: DemandOutputRef {
                orbit: orbit_id.clone(),
                position: DemandOutputPositionId("second".to_owned()),
            },
        };
        let orbit = SemanticDemandOrbit {
            id: orbit_id,
            package: "axiomatic_bundle".to_owned(),
            generated_by_window: [12, 13],
            normalized_demand_type: "RequiredRecordOutputs".to_owned(),
            required_outputs: vec![
                DemandOutputPosition {
                    id: DemandOutputPositionId("first".to_owned()),
                    family_class: first.family.univalent_class.clone(),
                    normalized_output_type: "Schema".to_owned(),
                },
                DemandOutputPosition {
                    id: DemandOutputPositionId("second".to_owned()),
                    family_class: second.family.univalent_class.clone(),
                    normalized_output_type: "Schema".to_owned(),
                },
            ],
            resolution: OrbitResolution::Live,
        };
        let theorem = derive_conditional_at_most(&CandidateProvenanceSubmission {
            candidate_stage: 14,
            kappa: 1,
            credited_families: vec![first, second],
            family_extraction_completeness_assumption: assumption("all-marginal-families"),
            demand_orbits: vec![orbit],
            orbit_inventory_completeness_assumption: assumption("all-window-orbits-and-outputs"),
            window_locality_assumption: assumption("all-live-debt-is-window-local"),
        })
        .expect("uniqueness is on (orbit, required-output-position)");
        assert_eq!(theorem.debt_orbit_credits, 2);
        assert_eq!(theorem.marginal_nu, 2);
    }

    #[test]
    fn an_empty_orbit_vector_is_not_debt_free_without_exhaustiveness_assumption() {
        let mut submission =
            assumed_complete_submission(1, vec![local_credit("head", 0, LocalRole::KernelHead)]);
        submission.orbit_inventory_completeness_assumption =
            SemanticAssumptionRef::missing("J2/J3");
        let theorem = derive_conditional_at_most(&submission)
            .expect("local injection can be audited while debt status stays unknown");
        assert!(!theorem.debt_free_if_inventory_exhaustive);
        assert!(!theorem.conditional_debt_free_linear_bound_holds);
    }

    #[test]
    fn a_partial_family_list_cannot_derive_a_conditional_at_most_bound() {
        let mut submission =
            assumed_complete_submission(1, vec![local_credit("head", 0, LocalRole::KernelHead)]);
        submission.family_extraction_completeness_assumption =
            SemanticAssumptionRef::missing("typed marginal-family exhaustiveness");
        assert_eq!(
            derive_conditional_at_most(&submission),
            Err(ProvenanceError::IncompleteFamilyExtraction)
        );
    }

    #[test]
    fn focus_label_emptiness_does_not_establish_semantic_o16_emptiness() {
        let report = audit_semantic_debt(&[]);
        assert!(report.focus_label_o16_empty);
        assert_eq!(report.semantic_o16_empty_if_assumptions, None);
        assert!(!report.j2_assumed_at_16);
        assert!(!report.j3_assumed_at_16);
        assert!(!report.window_locality_assumed_at_16);
        assert!(!report.j2_j3_conditionally_sufficient_at_16);
    }

    #[test]
    fn assumed_empty_window_inventory_is_only_conditionally_sufficient_at_sixteen() {
        let report = audit_semantic_debt(&[StageOrbitInventory {
            stage: 16,
            window: [14, 15],
            orbits: Vec::new(),
            extraction_completeness_assumption: assumption("extract-all-depth-two-orbits-14-15"),
            derivability_completeness_assumption: assumption("decide-all-depth-two-orbits-14-15"),
            window_locality_assumption: assumption("all-live-debt-is-window-local"),
        }]);
        assert_eq!(report.semantic_o16_empty_if_assumptions, Some(true));
        assert!(report.j2_j3_conditionally_sufficient_at_16);
    }

    #[test]
    fn empty_active_window_does_not_cover_older_debt_without_locality() {
        let report = audit_semantic_debt(&[StageOrbitInventory {
            stage: 16,
            window: [14, 15],
            orbits: Vec::new(),
            extraction_completeness_assumption: assumption("all-active-window-orbits"),
            derivability_completeness_assumption: assumption("all-active-window-decisions"),
            window_locality_assumption: SemanticAssumptionRef::missing(
                "older demands persist into the active window",
            ),
        }]);
        assert!(report.j2_assumed_at_16);
        assert!(report.j3_assumed_at_16);
        assert!(!report.window_locality_assumed_at_16);
        assert_eq!(report.semantic_o16_empty_if_assumptions, None);
        assert!(!report.j2_j3_conditionally_sufficient_at_16);
    }

    #[test]
    fn duplicate_or_out_of_range_inventories_are_rejected_not_overwritten() {
        let inventory = StageOrbitInventory {
            stage: 16,
            window: [14, 15],
            orbits: Vec::new(),
            extraction_completeness_assumption: assumption("all-active-window-orbits"),
            derivability_completeness_assumption: assumption("all-active-window-decisions"),
            window_locality_assumption: assumption("window locality"),
        };
        let mut out_of_range = inventory.clone();
        out_of_range.stage = 17;
        out_of_range.window = [15, 16];
        let report = audit_semantic_debt(&[inventory.clone(), inventory, out_of_range]);
        assert!(!report.inventory_input_valid);
        assert_eq!(report.rejected_inventory_stages, vec![16, 17]);
        assert_eq!(report.semantic_o16_empty_if_assumptions, None);
        assert!(!report.j2_j3_conditionally_sufficient_at_16);
    }

    #[test]
    fn weakening_erasure_assumptions_conditionally_give_zero_marginal_novelty() {
        let b = SchemaFamilyId("base-family".to_owned());
        let x = SchemaFamilyId("extension-family".to_owned());
        let assumptions = GuardedInternalityAssumptions {
            flow_name: "guarded-test-flow".to_owned(),
            base_families: [b.clone()].into_iter().collect(),
            extension_families: [x.clone()].into_iter().collect(),
            base_extraction_completeness: assumption("all-base-families"),
            extension_extraction_completeness: assumption("all-extension-families"),
            weakening: vec![FamilyMapEdge {
                source: b.clone(),
                target: x.clone(),
                action: assumption("weakening"),
            }],
            erasure: vec![FamilyMapEdge {
                source: x,
                target: b,
                action: assumption("erasure"),
            }],
            erasure_after_weakening: assumption("erase-weaken=id"),
            weakening_after_erasure_up_to_univalence: assumption("weaken-erase=ua-id"),
        };
        let theorem = derive_conditional_conservative_internality(&assumptions)
            .expect("the two finite maps and semantic inverse laws agree");
        assert_eq!(theorem.marginal_nu, 0);
        assert!(theorem.weakening_erasure_are_inverse);
    }

    #[test]
    fn canonical_fifteen_scores_replay_but_revised_audit_stays_unproved() {
        let report = replay_genesis_provenance();
        assert_eq!(
            report.recomputed_totals,
            vec![1, 1, 2, 5, 7, 8, 10, 18, 17, 19, 26, 34, 46, 62, 103]
        );
        assert!(report.numerical_replay_complete);
        assert!(!report.all_historical_credit_provenanced);
        assert!(!report.j2_window_orbit_extraction_available);
        assert!(!report.j3_orbit_derivability_available);
        assert!(report.steps.iter().all(|step| {
            step.numerical_score_preserved
                && step.revised_audited_nu.is_none()
                && step.revised_audit_status
                    == RevisedAuditStatus::IncompleteTypedFamilyAndOrbitExtraction
                && !step.all_credited_families_provenanced
        }));
        assert!(report
            .steps
            .iter()
            .flat_map(|step| &step.amplification_claims)
            .all(|claim| !claim.orbit_correspondence_available));
        let mechanisms = report
            .steps
            .iter()
            .flat_map(|step| &step.amplification_claims)
            .map(|claim| claim.mechanism)
            .collect::<BTreeSet<_>>();
        for required in [
            CreditMechanism::AdjointCompletion,
            CreditMechanism::DimensionSquared,
            CreditMechanism::ReferenceSquared,
            CreditMechanism::P5InheritedSurface,
            CreditMechanism::P5LocalAndBridge,
            CreditMechanism::P6UniformSpecialization,
            CreditMechanism::ModalPairwiseCoherence,
            CreditMechanism::DistributiveInheritance,
            CreditMechanism::CombinatorialSynthesis,
        ] {
            assert!(mechanisms.contains(&required), "missing {required:?}");
        }
    }

    #[test]
    fn replay_json_separates_legacy_number_from_revised_status() {
        let json = serde_json::to_value(replay_genesis_provenance())
            .expect("provenance replay is machine-readable");
        let step15 = &json["steps"][14];
        assert_eq!(step15["nu_total"], 103);
        assert_eq!(step15["numerical_score_preserved"], true);
        assert!(step15["revised_audited_nu"].is_null());
        assert_eq!(
            step15["revised_audit_status"],
            "incomplete_typed_family_and_orbit_extraction"
        );
        assert_eq!(json["j2_window_orbit_extraction_available"], false);
        assert_eq!(json["j3_orbit_derivability_available"], false);
    }

    #[test]
    fn step15_internality_is_an_explicit_obligation_not_a_claim() {
        let obligation = guarded_step15_internality_obligation();
        assert!(!obligation.typed_family_extraction_available);
        assert!(!obligation.inverse_laws_available);
        assert!(!obligation.conclusion_nu_zero_proved);
    }
}
