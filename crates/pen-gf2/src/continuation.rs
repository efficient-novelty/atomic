//! Exact free sealing and prospective-census continuation for the
//! one-nullary inductive-completion slice.
//!
//! This module is a versioned successor to the frozen H3 experiment.  It does
//! not add a demand generator.  It only specifies how the unique, already
//! certified response is integrated and how the same finite `G-Use` /
//! `G-Compute` inventory is re-decided against the cumulative library.

use crate::VerifiedOneNullaryInductiveSlice;
use crate::operational::{
    OneNullaryUseDecision, decide_one_nullary_use, verify_demand_connected_response,
};
use pen_demand::gsc::{
    GscFamilyId, GscOriginEventId, GscOutcome, GscRule, GscUnknownReason, PortKey,
    VerifiedGscSemanticManifest,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Declaration, Digest, Kernel, UncheckedSignature,
    VerifiedSignature,
};
use pen_law::{
    DEMAND_WINDOW_WIDTH, EventId, VerifiedHistory, VerifiedRegisteredActiveDemandInventory,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const LAW_V2_CONTINUATION_SEMANTIC_MANIFEST_SCHEMA_VERSION: u16 = 1;

const CONTINUATION_MANIFEST_DOMAIN: &str = "pen-gf2/law-v2-continuation-semantic-manifest/v1";
const SELECTED_SOURCE_DOMAIN: &str = "pen-gf2/law-v2-continuation-selected-source/v1";
const SELECTED_BINDING_DOMAIN: &str = "pen-gf2/law-v2-continuation-selected-binding/v1";
const EXTENSION_DOMAIN: &str = "pen-gf2/law-v2-continuation-extension/v1";
const EVENT_EXPORT_DOMAIN: &str = "pen-gf2/law-v2-continuation-event-export/v1";
const EVENT_ID_DOMAIN: &str = "pen-gf2/law-v2-continuation-event-id/v1";
const HISTORY_STEP_DOMAIN: &str = "pen-gf2/law-v2-continuation-history-step/v1";
const ANCHOR_DOMAIN: &str = "pen-gf2/law-v2-continuation-anchor/v1";
const Q3_DOMAIN: &str = "pen-gf2/law-v2-continuation-empty-q3/v1";
const INITIALITY_DOMAIN: &str = "pen-gf2/law-v2-continuation-free-seal-initiality/v1";
const FREE_SEAL_DOMAIN: &str = "pen-gf2/law-v2-continuation-free-seal/v1";
const CONTINUATION_INVENTORY_DOMAIN: &str = "pen-gf2/law-v2-continuation-active-inventory/v1";
const EXTRACTION_DOMAIN: &str = "pen-gf2/law-v2-continuation-extraction/v1";
const DERIVABILITY_DOMAIN: &str = "pen-gf2/law-v2-continuation-derivability/v1";
const EXPIRATION_DOMAIN: &str = "pen-gf2/law-v2-continuation-expiration/v1";
const CENSUS_DOMAIN: &str = "pen-gf2/law-v2-continuation-census/v1";
const EXCLUSION_DOMAIN: &str = "pen-gf2/law-v2-continuation-positive-cost-exclusion/v1";
const HALT_DOMAIN: &str = "pen-gf2/law-v2-continuation-halt/v1";

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationIntegrationRule {
    ExactSingletonFreshHeadAndGeneratedEquationsV1,
}

impl CanonicalEncode for ContinuationIntegrationRule {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(0);
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationExtractionRule {
    RegisteredFamiliesWhoseBirthSupportTouchesActiveWindowV1,
}

impl CanonicalEncode for ContinuationExtractionRule {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(0);
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationLibraryRule {
    CumulativeBoundaryAndSealedGeneratedEquationsV1,
}

impl CanonicalEncode for ContinuationLibraryRule {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(0);
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationExpirationRule {
    PriorLivePortsRemainDerivedUnderExactCumulativeWeakeningV1,
}

impl CanonicalEncode for ContinuationExpirationRule {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(0);
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuationHaltRule {
    CompleteExtractedAllDerivedAndZeroLiveForbidsPositiveCostV1,
}

impl CanonicalEncode for ContinuationHaltRule {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(0);
    }
}

/// The additive semantic decision needed to integrate a selected H3 response
/// and inspect its prospective successor.  It binds, but does not modify, the
/// frozen GSC semantic manifest.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LawV2ContinuationSemanticManifestV1 {
    pub schema_version: u16,
    pub base_gsc_semantic_manifest_digest: Digest,
    pub demand_window_width: u8,
    pub integration_rule: ContinuationIntegrationRule,
    pub extraction_rule: ContinuationExtractionRule,
    pub library_rule: ContinuationLibraryRule,
    pub expiration_rule: ContinuationExpirationRule,
    pub halt_rule: ContinuationHaltRule,
}

impl CanonicalEncode for LawV2ContinuationSemanticManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.base_gsc_semantic_manifest_digest
            .encode_canonical(encoder);
        encoder.u16(u16::from(self.demand_window_width));
        self.integration_rule.encode_canonical(encoder);
        self.extraction_rule.encode_canonical(encoder);
        self.library_rule.encode_canonical(encoder);
        self.expiration_rule.encode_canonical(encoder);
        self.halt_rule.encode_canonical(encoder);
    }
}

impl LawV2ContinuationSemanticManifestV1 {
    pub fn canonical_digest(&self) -> Digest {
        Digest::of_domain_bytes(
            CONTINUATION_MANIFEST_DOMAIN,
            canonical_bytes(self).as_slice(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedLawV2ContinuationSemanticManifestV1 {
    manifest: LawV2ContinuationSemanticManifestV1,
    digest: Digest,
}

impl VerifiedLawV2ContinuationSemanticManifestV1 {
    pub fn manifest(&self) -> &LawV2ContinuationSemanticManifestV1 {
        &self.manifest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

pub fn frozen_law_v2_continuation_semantic_manifest_v1(
    base: &VerifiedGscSemanticManifest,
) -> LawV2ContinuationSemanticManifestV1 {
    LawV2ContinuationSemanticManifestV1 {
        schema_version: LAW_V2_CONTINUATION_SEMANTIC_MANIFEST_SCHEMA_VERSION,
        base_gsc_semantic_manifest_digest: base.digest().clone(),
        demand_window_width: DEMAND_WINDOW_WIDTH,
        integration_rule:
            ContinuationIntegrationRule::ExactSingletonFreshHeadAndGeneratedEquationsV1,
        extraction_rule:
            ContinuationExtractionRule::RegisteredFamiliesWhoseBirthSupportTouchesActiveWindowV1,
        library_rule: ContinuationLibraryRule::CumulativeBoundaryAndSealedGeneratedEquationsV1,
        expiration_rule:
            ContinuationExpirationRule::PriorLivePortsRemainDerivedUnderExactCumulativeWeakeningV1,
        halt_rule:
            ContinuationHaltRule::CompleteExtractedAllDerivedAndZeroLiveForbidsPositiveCostV1,
    }
}

pub fn verify_law_v2_continuation_semantic_manifest_v1(
    base: &VerifiedGscSemanticManifest,
    candidate: &LawV2ContinuationSemanticManifestV1,
) -> GscOutcome<VerifiedLawV2ContinuationSemanticManifestV1> {
    let frozen = frozen_law_v2_continuation_semantic_manifest_v1(base);
    if candidate != &frozen {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest);
    }
    GscOutcome::Proven(VerifiedLawV2ContinuationSemanticManifestV1 {
        digest: frozen.canonical_digest(),
        manifest: frozen,
    })
}

#[derive(Clone, Debug)]
pub struct VerifiedOneNullaryFreeSealV1 {
    ordinal: u16,
    predecessor: EventId,
    event_id: EventId,
    pre_history_digest: Digest,
    post_history_digest: Digest,
    pre_boundary_digest: Digest,
    post_boundary: VerifiedSignature,
    extension: Declaration,
    extension_digest: Digest,
    source_identity: Digest,
    binding_identity: Digest,
    event_export_digest: Digest,
    activated_closed_former_group_count: u32,
    equation_set_digest: Digest,
    response_digest: Digest,
    quotient_digest: Digest,
    active_anchor_digest: Digest,
    empty_q3_registry_digest: Digest,
    initiality_certificate_digest: Digest,
    digest: Digest,
}

impl VerifiedOneNullaryFreeSealV1 {
    pub fn ordinal(&self) -> u16 {
        self.ordinal
    }

    pub fn predecessor(&self) -> &EventId {
        &self.predecessor
    }

    pub fn event_id(&self) -> &EventId {
        &self.event_id
    }

    pub fn pre_history_digest(&self) -> &Digest {
        &self.pre_history_digest
    }

    pub fn post_history_digest(&self) -> &Digest {
        &self.post_history_digest
    }

    pub fn pre_boundary_digest(&self) -> &Digest {
        &self.pre_boundary_digest
    }

    pub fn post_boundary(&self) -> &VerifiedSignature {
        &self.post_boundary
    }

    pub fn extension(&self) -> &Declaration {
        &self.extension
    }

    pub fn extension_digest(&self) -> &Digest {
        &self.extension_digest
    }

    pub fn source_identity(&self) -> &Digest {
        &self.source_identity
    }

    pub fn binding_identity(&self) -> &Digest {
        &self.binding_identity
    }

    pub fn event_export_digest(&self) -> &Digest {
        &self.event_export_digest
    }

    pub fn activated_closed_former_group_count(&self) -> u32 {
        self.activated_closed_former_group_count
    }

    pub fn equation_set_digest(&self) -> &Digest {
        &self.equation_set_digest
    }

    pub fn response_digest(&self) -> &Digest {
        &self.response_digest
    }

    pub fn quotient_digest(&self) -> &Digest {
        &self.quotient_digest
    }

    pub fn active_anchor_digest(&self) -> &Digest {
        &self.active_anchor_digest
    }

    pub fn empty_q3_registry_digest(&self) -> &Digest {
        &self.empty_q3_registry_digest
    }

    pub fn initiality_certificate_digest(&self) -> &Digest {
        &self.initiality_certificate_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedH4DemandDecisionV1 {
    family_id: GscFamilyId,
    rule: GscRule,
    port: PortKey,
    evidence_digest: Digest,
}

impl VerifiedH4DemandDecisionV1 {
    pub fn family_id(&self) -> &GscFamilyId {
        &self.family_id
    }

    pub fn rule(&self) -> GscRule {
        self.rule
    }

    pub fn port(&self) -> &PortKey {
        &self.port
    }

    pub fn evidence_digest(&self) -> &Digest {
        &self.evidence_digest
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedH4CensusV1 {
    active_older: EventId,
    active_newer: EventId,
    active_anchor_digest: Digest,
    registered_inventory_binding_digest: Digest,
    continuation_inventory_binding_digest: Digest,
    active_frame_binding_digests: Vec<Digest>,
    decisions: Vec<VerifiedH4DemandDecisionV1>,
    extracted_family_count: u32,
    extracted_port_count: u32,
    derived_port_count: u32,
    live_orbit_count: u32,
    extraction_complete: bool,
    derivability_complete: bool,
    expiration_complete: bool,
    cumulative_weakening_replay_digest: Digest,
    extraction_digest: Digest,
    derivability_digest: Digest,
    expiration_digest: Digest,
    digest: Digest,
}

impl VerifiedH4CensusV1 {
    pub fn active_older(&self) -> &EventId {
        &self.active_older
    }

    pub fn active_newer(&self) -> &EventId {
        &self.active_newer
    }

    pub fn active_anchor_digest(&self) -> &Digest {
        &self.active_anchor_digest
    }

    pub fn registered_inventory_binding_digest(&self) -> &Digest {
        &self.registered_inventory_binding_digest
    }

    pub fn continuation_inventory_binding_digest(&self) -> &Digest {
        &self.continuation_inventory_binding_digest
    }

    pub fn active_frame_binding_digests(&self) -> &[Digest] {
        &self.active_frame_binding_digests
    }

    pub fn decisions(&self) -> &[VerifiedH4DemandDecisionV1] {
        &self.decisions
    }

    pub fn extracted_family_count(&self) -> u32 {
        self.extracted_family_count
    }

    pub fn extracted_port_count(&self) -> u32 {
        self.extracted_port_count
    }

    pub fn derived_port_count(&self) -> u32 {
        self.derived_port_count
    }

    pub fn live_orbit_count(&self) -> u32 {
        self.live_orbit_count
    }

    pub fn extraction_is_complete(&self) -> bool {
        self.extraction_complete
    }

    pub fn derivability_is_complete(&self) -> bool {
        self.derivability_complete
    }

    pub fn expiration_is_complete(&self) -> bool {
        self.expiration_complete
    }

    pub fn cumulative_weakening_replay_digest(&self) -> &Digest {
        &self.cumulative_weakening_replay_digest
    }

    pub fn extraction_digest(&self) -> &Digest {
        &self.extraction_digest
    }

    pub fn derivability_digest(&self) -> &Digest {
        &self.derivability_digest
    }

    pub fn expiration_digest(&self) -> &Digest {
        &self.expiration_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedH4HaltCertificateV1 {
    continuation_semantic_manifest_digest: Digest,
    base_gsc_semantic_manifest_digest: Digest,
    free_seal: VerifiedOneNullaryFreeSealV1,
    census: VerifiedH4CensusV1,
    positive_cost_exclusion_digest: Digest,
    halted_after_sealed_act_count: u16,
    prospective_stage_ordinal: u16,
    digest: Digest,
}

impl VerifiedH4HaltCertificateV1 {
    pub fn continuation_semantic_manifest_digest(&self) -> &Digest {
        &self.continuation_semantic_manifest_digest
    }

    pub fn base_gsc_semantic_manifest_digest(&self) -> &Digest {
        &self.base_gsc_semantic_manifest_digest
    }

    pub fn free_seal(&self) -> &VerifiedOneNullaryFreeSealV1 {
        &self.free_seal
    }

    pub fn census(&self) -> &VerifiedH4CensusV1 {
        &self.census
    }

    pub fn positive_cost_exclusion_digest(&self) -> &Digest {
        &self.positive_cost_exclusion_digest
    }

    pub fn halted_after_sealed_act_count(&self) -> u16 {
        self.halted_after_sealed_act_count
    }

    pub fn prospective_stage_ordinal(&self) -> u16 {
        self.prospective_stage_ordinal
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

pub fn continue_one_nullary_inductive_completion_v1(
    kernel: &Kernel,
    base_semantic: &VerifiedGscSemanticManifest,
    continuation_semantic: &VerifiedLawV2ContinuationSemanticManifestV1,
    history: &VerifiedHistory,
    inventory: &VerifiedRegisteredActiveDemandInventory,
    slice: &VerifiedOneNullaryInductiveSlice,
) -> GscOutcome<VerifiedH4HaltCertificateV1> {
    if continuation_semantic
        .manifest()
        .base_gsc_semantic_manifest_digest
        != *base_semantic.digest()
        || continuation_semantic.manifest().demand_window_width != DEMAND_WINDOW_WIDTH
        || DEMAND_WINDOW_WIDTH != 2
        || !inventory.binding_is_valid()
        || inventory.semantic_manifest_digest() != base_semantic.digest()
        || inventory.binding_digest() != slice.registered_active_demand_inventory_binding_digest()
        || inventory.tuples().len() != 1
        || inventory.family_count() != 2
        || inventory.port_count() != 2
        || !slice.candidate_carrier().is_exhaustive()
        || slice.candidate_carrier().candidate_count() != 1
        || !slice.quotient().is_complete()
        || slice.quotient().classes_len() != 1
    {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest);
    }

    let free_seal = match verify_free_seal(
        base_semantic,
        continuation_semantic,
        history,
        inventory,
        slice,
    ) {
        Ok(seal) => seal,
        Err(reason) => return GscOutcome::Unknown(reason),
    };

    let tuple = &inventory.tuples()[0];
    if !tuple.binding_is_valid() {
        return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
    }
    let active_older = free_seal.predecessor().clone();
    let active_newer = free_seal.event_id().clone();
    let active_origins = [
        GscOriginEventId(active_older.0.clone()),
        GscOriginEventId(active_newer.0.clone()),
    ];
    let support_touches_active_window = tuple
        .frame()
        .principal_source_bindings()
        .iter()
        .any(|source| active_origins.contains(source.origin()));
    if !support_touches_active_window {
        return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
    }

    let use_family = tuple.use_family();
    let [compute_tuple] = tuple.compute_families() else {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    };
    let compute_family = compute_tuple.family();
    let derived = match decide_one_nullary_use(
        kernel,
        free_seal.post_boundary(),
        base_semantic,
        use_family.inner(),
    ) {
        GscOutcome::Proven(OneNullaryUseDecision::Derived(derived)) => derived,
        GscOutcome::Proven(OneNullaryUseDecision::Underived(_)) => {
            return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
        }
        GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
    };
    let cumulative_response = match verify_demand_connected_response(
        use_family.inner(),
        compute_family.inner(),
        slice.equations(),
        derived,
    ) {
        GscOutcome::Proven(response) => response,
        GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
    };
    if cumulative_response.digest() != slice.response().digest()
        || !cumulative_response.all_outputs_filled()
        || !cumulative_response.all_positive_clauses_connected()
    {
        return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
    }

    let decisions = vec![
        VerifiedH4DemandDecisionV1 {
            family_id: use_family.inner().id().clone(),
            rule: GscRule::Use,
            port: use_family.inner().ports()[0].key().clone(),
            evidence_digest: cumulative_response.use_derivation().digest().clone(),
        },
        VerifiedH4DemandDecisionV1 {
            family_id: compute_family.inner().id().clone(),
            rule: GscRule::Compute,
            port: compute_family.inner().ports()[0].key().clone(),
            evidence_digest: slice.equations().digest().clone(),
        },
    ];
    let active_frame_binding_digests = vec![tuple.frame().binding_digest().clone()];
    let continuation_inventory_binding_digest = {
        let mut encoder = CanonicalEncoder::new();
        base_semantic.digest().encode_canonical(&mut encoder);
        continuation_semantic
            .digest()
            .encode_canonical(&mut encoder);
        inventory.binding_digest().encode_canonical(&mut encoder);
        free_seal
            .active_anchor_digest()
            .encode_canonical(&mut encoder);
        encoder.sequence(&active_frame_binding_digests);
        for decision in &decisions {
            decision.family_id.encode_canonical(&mut encoder);
            decision.rule.encode_canonical(&mut encoder);
            decision.port.encode_canonical(&mut encoder);
        }
        Digest::of_domain_bytes(CONTINUATION_INVENTORY_DOMAIN, encoder.as_bytes())
    };
    let extraction_digest = {
        let mut encoder = CanonicalEncoder::new();
        continuation_semantic
            .digest()
            .encode_canonical(&mut encoder);
        inventory.binding_digest().encode_canonical(&mut encoder);
        free_seal
            .active_anchor_digest()
            .encode_canonical(&mut encoder);
        encoder.sequence(&active_origins);
        encoder.sequence(&active_frame_binding_digests);
        continuation_inventory_binding_digest.encode_canonical(&mut encoder);
        encoder.u32(2);
        encoder.u32(2);
        Digest::of_domain_bytes(EXTRACTION_DOMAIN, encoder.as_bytes())
    };
    let derivability_digest = {
        let mut encoder = CanonicalEncoder::new();
        extraction_digest.encode_canonical(&mut encoder);
        cumulative_response.digest().encode_canonical(&mut encoder);
        for decision in &decisions {
            decision.family_id.encode_canonical(&mut encoder);
            decision.rule.encode_canonical(&mut encoder);
            decision.port.encode_canonical(&mut encoder);
            decision.evidence_digest.encode_canonical(&mut encoder);
        }
        encoder.u32(2);
        encoder.u32(0);
        Digest::of_domain_bytes(DERIVABILITY_DOMAIN, encoder.as_bytes())
    };
    let cumulative_weakening_replay_digest = {
        let mut encoder = CanonicalEncoder::new();
        slice.pre_response().digest().encode_canonical(&mut encoder);
        slice.response().digest().encode_canonical(&mut encoder);
        cumulative_response.digest().encode_canonical(&mut encoder);
        history
            .active_anchor()
            .digest()
            .encode_canonical(&mut encoder);
        free_seal
            .active_anchor_digest()
            .encode_canonical(&mut encoder);
        Digest::of_domain_bytes(
            "pen-gf2/law-v2-continuation-cumulative-weakening/v1",
            encoder.as_bytes(),
        )
    };
    let expiration_digest = {
        let mut encoder = CanonicalEncoder::new();
        continuation_semantic
            .manifest()
            .expiration_rule
            .encode_canonical(&mut encoder);
        cumulative_weakening_replay_digest.encode_canonical(&mut encoder);
        encoder.u32(2);
        encoder.u32(2);
        encoder.u32(0);
        Digest::of_domain_bytes(EXPIRATION_DOMAIN, encoder.as_bytes())
    };
    let census_digest = {
        let mut encoder = CanonicalEncoder::new();
        free_seal
            .post_history_digest()
            .encode_canonical(&mut encoder);
        free_seal
            .active_anchor_digest()
            .encode_canonical(&mut encoder);
        extraction_digest.encode_canonical(&mut encoder);
        derivability_digest.encode_canonical(&mut encoder);
        expiration_digest.encode_canonical(&mut encoder);
        encoder.tag(1);
        encoder.tag(1);
        encoder.tag(1);
        encoder.u32(2);
        encoder.u32(2);
        encoder.u32(2);
        encoder.u32(0);
        Digest::of_domain_bytes(CENSUS_DOMAIN, encoder.as_bytes())
    };
    let census = VerifiedH4CensusV1 {
        active_older,
        active_newer,
        active_anchor_digest: free_seal.active_anchor_digest().clone(),
        registered_inventory_binding_digest: inventory.binding_digest().clone(),
        continuation_inventory_binding_digest,
        active_frame_binding_digests,
        decisions,
        extracted_family_count: 2,
        extracted_port_count: 2,
        derived_port_count: 2,
        live_orbit_count: 0,
        extraction_complete: true,
        derivability_complete: true,
        expiration_complete: true,
        cumulative_weakening_replay_digest,
        extraction_digest,
        derivability_digest,
        expiration_digest,
        digest: census_digest,
    };

    let positive_cost_exclusion_digest = {
        let mut encoder = CanonicalEncoder::new();
        continuation_semantic
            .manifest()
            .halt_rule
            .encode_canonical(&mut encoder);
        base_semantic
            .manifest()
            .response_candidate_grammar
            .encode_canonical(&mut encoder);
        census.digest().encode_canonical(&mut encoder);
        encoder.u32(census.live_orbit_count());
        encoder.tag(1);
        Digest::of_domain_bytes(EXCLUSION_DOMAIN, encoder.as_bytes())
    };
    let halted_after_sealed_act_count = free_seal.ordinal();
    let prospective_stage_ordinal = halted_after_sealed_act_count.saturating_add(1);
    let digest = {
        let mut encoder = CanonicalEncoder::new();
        continuation_semantic
            .digest()
            .encode_canonical(&mut encoder);
        base_semantic.digest().encode_canonical(&mut encoder);
        free_seal.digest().encode_canonical(&mut encoder);
        census.digest().encode_canonical(&mut encoder);
        positive_cost_exclusion_digest.encode_canonical(&mut encoder);
        encoder.u16(halted_after_sealed_act_count);
        encoder.u16(prospective_stage_ordinal);
        Digest::of_domain_bytes(HALT_DOMAIN, encoder.as_bytes())
    };
    GscOutcome::Proven(VerifiedH4HaltCertificateV1 {
        continuation_semantic_manifest_digest: continuation_semantic.digest().clone(),
        base_gsc_semantic_manifest_digest: base_semantic.digest().clone(),
        free_seal,
        census,
        positive_cost_exclusion_digest,
        halted_after_sealed_act_count,
        prospective_stage_ordinal,
        digest,
    })
}

fn verify_free_seal(
    base_semantic: &VerifiedGscSemanticManifest,
    continuation_semantic: &VerifiedLawV2ContinuationSemanticManifestV1,
    history: &VerifiedHistory,
    inventory: &VerifiedRegisteredActiveDemandInventory,
    slice: &VerifiedOneNullaryInductiveSlice,
) -> Result<VerifiedOneNullaryFreeSealV1, GscUnknownReason> {
    let Some(pre_boundary) = history.boundary_snapshots().last() else {
        return Err(GscUnknownReason::UnsupportedBoundary);
    };
    let Some(last_event) = history.events().last() else {
        return Err(GscUnknownReason::UnsupportedBoundary);
    };
    if history.events().len() != 3
        || last_event.ordinal() != 3
        || history.active_anchor().newer() != last_event.event_id()
        || pre_boundary.digest() != inventory.cutoff_boundary_digest()
    {
        return Err(GscUnknownReason::UnsupportedBoundary);
    }

    let equations = slice.equations();
    let response = slice.response();
    let carrier = slice.candidate_carrier();
    let quotient = slice.quotient();
    let post_boundary = equations.extended_signature();
    let pre_declarations = pre_boundary.declarations();
    let post_declarations = post_boundary.declarations();
    if post_declarations.len() != pre_declarations.len().saturating_add(1)
        || post_declarations[..pre_declarations.len()] != *pre_declarations
    {
        return Err(GscUnknownReason::UnsupportedBoundary);
    }
    let extension = post_declarations
        .last()
        .cloned()
        .ok_or(GscUnknownReason::UnsupportedBoundary)?;
    let clause = equations.clause();
    let invariants = clause.invariants();
    if extension.id != *clause.fresh_head()
        || extension.ty != *clause.head_type()
        || extension.body.is_some()
        || carrier.canonical_fresh_head() != clause.fresh_head()
        || carrier.equation_set_digest() != equations.digest()
        || carrier.response_digest() != response.digest()
        || quotient.response_count() != 1
        || quotient.classes_len() != 1
        || quotient.class().equation_digest() != equations.digest()
        || !invariants.is_left_linear()
        || !invariants.is_nonrecursive()
        || !invariants.has_no_critical_overlaps()
        || !invariants.is_terminating_in_admitted_fragment()
        || !invariants.is_confluent_in_admitted_fragment()
        || !invariants.is_conservative_on_old_terms()
        || !invariants.exact_generated_substitution_preserves_equation()
    {
        return Err(GscUnknownReason::KernelCouldNotCertify);
    }

    let extension_digest = Digest::of_domain_bytes(
        EXTENSION_DOMAIN,
        canonical_bytes(&UncheckedSignature {
            declarations: vec![extension.clone()],
        })
        .as_slice(),
    );
    let source_identity = {
        let mut encoder = CanonicalEncoder::new();
        quotient.class().digest().encode_canonical(&mut encoder);
        response.digest().encode_canonical(&mut encoder);
        carrier.digest().encode_canonical(&mut encoder);
        Digest::of_domain_bytes(SELECTED_SOURCE_DOMAIN, encoder.as_bytes())
    };
    let binding_identity = {
        let mut encoder = CanonicalEncoder::new();
        base_semantic.digest().encode_canonical(&mut encoder);
        continuation_semantic
            .digest()
            .encode_canonical(&mut encoder);
        pre_boundary.digest().encode_canonical(&mut encoder);
        post_boundary.digest().encode_canonical(&mut encoder);
        extension_digest.encode_canonical(&mut encoder);
        equations.digest().encode_canonical(&mut encoder);
        Digest::of_domain_bytes(SELECTED_BINDING_DOMAIN, encoder.as_bytes())
    };
    let event_export_digest = {
        let mut encoder = CanonicalEncoder::new();
        extension.id.encode_canonical(&mut encoder);
        equations.digest().encode_canonical(&mut encoder);
        response.digest().encode_canonical(&mut encoder);
        encoder.u32(0);
        Digest::of_domain_bytes(EVENT_EXPORT_DOMAIN, encoder.as_bytes())
    };
    let ordinal = last_event.ordinal().saturating_add(1);
    let predecessor = last_event.event_id().clone();
    let event_id = {
        let mut encoder = CanonicalEncoder::new();
        history.digest().encode_canonical(&mut encoder);
        encoder.u16(ordinal);
        predecessor.encode_canonical(&mut encoder);
        pre_boundary.digest().encode_canonical(&mut encoder);
        post_boundary.digest().encode_canonical(&mut encoder);
        extension_digest.encode_canonical(&mut encoder);
        source_identity.encode_canonical(&mut encoder);
        binding_identity.encode_canonical(&mut encoder);
        event_export_digest.encode_canonical(&mut encoder);
        EventId(Digest::of_domain_bytes(EVENT_ID_DOMAIN, encoder.as_bytes()))
    };
    let post_history_digest = {
        let mut encoder = CanonicalEncoder::new();
        history.digest().encode_canonical(&mut encoder);
        event_id.encode_canonical(&mut encoder);
        post_boundary.digest().encode_canonical(&mut encoder);
        extension_digest.encode_canonical(&mut encoder);
        event_export_digest.encode_canonical(&mut encoder);
        Digest::of_domain_bytes(HISTORY_STEP_DOMAIN, encoder.as_bytes())
    };
    let active_anchor_digest = {
        let mut encoder = CanonicalEncoder::new();
        event_id.encode_canonical(&mut encoder);
        predecessor.encode_canonical(&mut encoder);
        post_boundary.digest().encode_canonical(&mut encoder);
        Digest::of_domain_bytes(ANCHOR_DOMAIN, encoder.as_bytes())
    };
    let empty_q3_registry_digest = {
        let mut encoder = CanonicalEncoder::new();
        active_anchor_digest.encode_canonical(&mut encoder);
        history
            .origin_cutoff_q3_registry()
            .digest()
            .encode_canonical(&mut encoder);
        encoder.u32(0);
        Digest::of_domain_bytes(Q3_DOMAIN, encoder.as_bytes())
    };
    let initiality_certificate_digest = {
        let mut encoder = CanonicalEncoder::new();
        continuation_semantic
            .manifest()
            .integration_rule
            .encode_canonical(&mut encoder);
        pre_boundary.digest().encode_canonical(&mut encoder);
        post_boundary.digest().encode_canonical(&mut encoder);
        extension.encode_canonical(&mut encoder);
        equations.digest().encode_canonical(&mut encoder);
        invariants.digest().encode_canonical(&mut encoder);
        quotient.digest().encode_canonical(&mut encoder);
        encoder.u32(1);
        encoder.u32(1);
        encoder.tag(1);
        Digest::of_domain_bytes(INITIALITY_DOMAIN, encoder.as_bytes())
    };
    let digest = {
        let mut encoder = CanonicalEncoder::new();
        continuation_semantic
            .digest()
            .encode_canonical(&mut encoder);
        history.digest().encode_canonical(&mut encoder);
        slice.digest().encode_canonical(&mut encoder);
        encoder.u16(ordinal);
        event_id.encode_canonical(&mut encoder);
        post_history_digest.encode_canonical(&mut encoder);
        post_boundary.digest().encode_canonical(&mut encoder);
        extension_digest.encode_canonical(&mut encoder);
        source_identity.encode_canonical(&mut encoder);
        binding_identity.encode_canonical(&mut encoder);
        event_export_digest.encode_canonical(&mut encoder);
        encoder.u32(0);
        equations.digest().encode_canonical(&mut encoder);
        response.digest().encode_canonical(&mut encoder);
        quotient.digest().encode_canonical(&mut encoder);
        active_anchor_digest.encode_canonical(&mut encoder);
        empty_q3_registry_digest.encode_canonical(&mut encoder);
        initiality_certificate_digest.encode_canonical(&mut encoder);
        Digest::of_domain_bytes(FREE_SEAL_DOMAIN, encoder.as_bytes())
    };
    Ok(VerifiedOneNullaryFreeSealV1 {
        ordinal,
        predecessor,
        event_id,
        pre_history_digest: history.digest().clone(),
        post_history_digest,
        pre_boundary_digest: pre_boundary.digest().clone(),
        post_boundary: post_boundary.clone(),
        extension,
        extension_digest,
        source_identity,
        binding_identity,
        event_export_digest,
        activated_closed_former_group_count: 0,
        equation_set_digest: equations.digest().clone(),
        response_digest: response.digest().clone(),
        quotient_digest: quotient.digest().clone(),
        active_anchor_digest,
        empty_q3_registry_digest,
        initiality_certificate_digest,
        digest,
    })
}

fn canonical_bytes<T: CanonicalEncode>(value: &T) -> Vec<u8> {
    let mut encoder = CanonicalEncoder::new();
    value.encode_canonical(&mut encoder);
    encoder.as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::{
        frozen_law_v2_continuation_semantic_manifest_v1,
        verify_law_v2_continuation_semantic_manifest_v1,
    };
    use pen_demand::gsc::{
        GscOutcome, frozen_gsc_semantic_manifest_v1, verify_gsc_semantic_manifest_v1,
    };

    #[test]
    fn continuation_manifest_is_additive_and_mutation_fails_closed() {
        let base = match verify_gsc_semantic_manifest_v1(&frozen_gsc_semantic_manifest_v1()) {
            GscOutcome::Proven(base) => base,
            GscOutcome::Unknown(reason) => panic!("frozen base semantic failed: {reason:?}"),
        };
        let manifest = frozen_law_v2_continuation_semantic_manifest_v1(&base);
        assert_eq!(manifest.base_gsc_semantic_manifest_digest, *base.digest());
        assert!(matches!(
            verify_law_v2_continuation_semantic_manifest_v1(&base, &manifest),
            GscOutcome::Proven(_)
        ));

        let mut changed = manifest;
        changed.demand_window_width = changed.demand_window_width.saturating_add(1);
        assert!(matches!(
            verify_law_v2_continuation_semantic_manifest_v1(&base, &changed),
            GscOutcome::Unknown(_)
        ));
    }
}
