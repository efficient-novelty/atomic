//! Capability-bound construction of the canonical production bundle.
//!
//! Callers supply certificate payloads but cannot supply the V3 manifest
//! surface, signature ordering, global-slot mapping, or predecessor delta
//! policy. Those fields are derived from opaque verified capabilities.

use crate::manifest::{
    VerifiedSemanticAuditManifestV3, proposed_semantic_audit_lambda_unit_manifest_v3,
};
use crate::production_inventory_bridge::VerifiedProductionInventoryBridgeV1;
use crate::production_refinement::VerifiedGlobalSlotTableV1;
use crate::production_refinement_theorem::{
    VerifiedProductionSynthesisProtocolIdentityV2, VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
};
use crate::production_refinement_wire_authority::{
    VerifiedCanonicalProductionBundleV1,
    verified_canonical_production_bundle_from_exact_round_trip_v1,
};
use crate::production_wire_slots::{
    ProductionWireSlotFailureV1, derive_global_slot_table_wire_v1, digest_wire_id,
};
use pen_kernel::VerifiedSignature;
use pen_production_wire::{
    CHECKER_UNIVERSE_LEVELS_V1, ConversionCertificateWireV1, ConversionTypingSupplementWireV1,
    EXACT_FAMILY_INVENTORY_V1, EXACT_Q0_INVENTORY_V1, EXACT_SYNTHESIS_INVENTORY_V1,
    FORMATION_WITNESS_LEVELS_V1, FamilyInventoryWireV1, FamilyPayloadWireV1, FreshRuleSchemaWireV1,
    ManifestAuthorityWireV1, ProductionContextWireV1, ProductionRefinementBundleV1,
    ProductionSignatureWireV1, Q0InventoryWireV1, SYNTHESIS_PROTOCOL_ID_V2,
    SYNTHESIS_SCHEMA_VERSION_V2, SynthesisCertificateWireV1, V3CorrespondenceManifestWireV1,
    WireErrorV1, WireHeaderV1, decode_bundle_v1, encode_bundle_v1,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProductionBundlePayloadV1 {
    pub contexts: Vec<ProductionContextWireV1>,
    pub conversions: Vec<ConversionCertificateWireV1>,
    pub conversion_typing_supplements: Vec<ConversionTypingSupplementWireV1>,
    pub synthesis_codes: Vec<SynthesisCertificateWireV1>,
    pub fresh_rule_schemas: Vec<FreshRuleSchemaWireV1>,
    pub family_payloads: Vec<FamilyPayloadWireV1>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProductionWireBuilderFailureV1 {
    ExactV3ManifestMismatch,
    SignatureSlotBindingMismatch,
    InventoryBridgeBindingMismatch,
    DeltaPolicyBindingMismatch,
    SynthesisProtocolBindingMismatch,
    SlotTranslation(ProductionWireSlotFailureV1),
    Wire(WireErrorV1),
    ExactRoundTripMismatch,
}

impl std::fmt::Display for ProductionWireBuilderFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactV3ManifestMismatch => {
                formatter.write_str("the bundle is not bound to the exact unchanged V3 candidate")
            }
            Self::SignatureSlotBindingMismatch => formatter.write_str(
                "the signature, verified slot table, and delta policy do not share one identity",
            ),
            Self::InventoryBridgeBindingMismatch => {
                formatter.write_str("the inventory bridge belongs to another semantic manifest")
            }
            Self::DeltaPolicyBindingMismatch => formatter.write_str(
                "the predecessor-public delta policy belongs to another V3 manifest or slot table",
            ),
            Self::SynthesisProtocolBindingMismatch => {
                formatter.write_str("the synthesis protocol identity is not the exact V2 surface")
            }
            Self::SlotTranslation(error) => write!(formatter, "slot translation failed: {error}"),
            Self::Wire(error) => write!(formatter, "canonical wire failed: {error}"),
            Self::ExactRoundTripMismatch => {
                formatter.write_str("the decoded bundle did not re-encode to the exact input bytes")
            }
        }
    }
}

impl std::error::Error for ProductionWireBuilderFailureV1 {}

impl From<ProductionWireSlotFailureV1> for ProductionWireBuilderFailureV1 {
    fn from(error: ProductionWireSlotFailureV1) -> Self {
        Self::SlotTranslation(error)
    }
}

impl From<WireErrorV1> for ProductionWireBuilderFailureV1 {
    fn from(error: WireErrorV1) -> Self {
        Self::Wire(error)
    }
}

pub fn build_canonical_production_bundle_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
    signature: &VerifiedSignature,
    slots: &VerifiedGlobalSlotTableV1,
    inventory: &VerifiedProductionInventoryBridgeV1,
    delta_policy: &VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
    synthesis: &VerifiedProductionSynthesisProtocolIdentityV2,
    payload: ProductionBundlePayloadV1,
) -> Result<VerifiedCanonicalProductionBundleV1, ProductionWireBuilderFailureV1> {
    let manifest_wire = manifest.manifest();
    if manifest_wire != &proposed_semantic_audit_lambda_unit_manifest_v3() {
        return Err(ProductionWireBuilderFailureV1::ExactV3ManifestMismatch);
    }
    if slots.signature_digest() != signature.digest()
        || delta_policy.successor_signature_digest() != signature.digest()
        || delta_policy.global_slot_table_digest() != slots.digest()
    {
        return Err(ProductionWireBuilderFailureV1::SignatureSlotBindingMismatch);
    }
    if inventory.semantic_manifest_digest() != manifest.candidate_digest() {
        return Err(ProductionWireBuilderFailureV1::InventoryBridgeBindingMismatch);
    }
    if delta_policy.lambda_unit_v3_manifest_digest() != manifest.candidate_digest() {
        return Err(ProductionWireBuilderFailureV1::DeltaPolicyBindingMismatch);
    }
    if synthesis.protocol_id().as_bytes() != SYNTHESIS_PROTOCOL_ID_V2
        || synthesis.schema_version() != SYNTHESIS_SCHEMA_VERSION_V2
        || synthesis.public_universe_levels() != [0, 1]
        || synthesis.checker_output_universe_levels() != CHECKER_UNIVERSE_LEVELS_V1
        || synthesis.synthesis_code_constructor_count() != EXACT_SYNTHESIS_INVENTORY_V1.len() as u64
    {
        return Err(ProductionWireBuilderFailureV1::SynthesisProtocolBindingMismatch);
    }

    let global_slot_table = derive_global_slot_table_wire_v1(signature, slots)?;
    let allowed_transparent_deltas = delta_policy
        .ledger_binding()
        .exact_ordered_entries()
        .iter()
        .map(|entry| {
            Ok(pen_production_wire::DeltaPolicyEntryWireV1 {
                global_slot: entry.global_slot,
                global_id_bytes: digest_wire_id(&entry.id.0)?,
            })
        })
        .collect::<Result<Vec<_>, ProductionWireSlotFailureV1>>()?;

    let bundle = ProductionRefinementBundleV1 {
        header: WireHeaderV1::canonical(),
        manifest_surface: V3CorrespondenceManifestWireV1 {
            semantic_schema_version: manifest_wire.schema_version,
            profile_id: manifest_wire.profile_id.as_bytes().to_vec(),
            semantic_manifest_digest: digest_wire_id(manifest.candidate_digest())?,
            authority: ManifestAuthorityWireV1::GenericPrototypeOnly,
            frozen: manifest_wire.frozen,
            live_profile_a_access: manifest_wire.live_profile_a_access,
            production_inventory_bridge_digest: digest_wire_id(inventory.digest())?,
            public_universe_levels: manifest_wire.universe_levels.clone(),
            checker_universe_levels: CHECKER_UNIVERSE_LEVELS_V1.to_vec(),
            formation_witness_levels: FORMATION_WITNESS_LEVELS_V1.to_vec(),
            maximum_context_entries: manifest_wire.maximum_context_entries,
            synthesis_rule_inventory: EXACT_SYNTHESIS_INVENTORY_V1.to_vec(),
            predecessor_delta_policy_binding_digest: digest_wire_id(delta_policy.digest())?,
            synthesis_protocol_id: synthesis.protocol_id().as_bytes().to_vec(),
            synthesis_schema_version: synthesis.schema_version(),
        },
        signature: ProductionSignatureWireV1 {
            signature_digest: digest_wire_id(signature.digest())?,
            kernel_protocol_digest: digest_wire_id(slots.kernel_protocol_digest())?,
            global_slot_table_digest: digest_wire_id(slots.digest())?,
            allowed_transparent_deltas,
        },
        global_slot_table,
        contexts: payload.contexts,
        conversions: payload.conversions,
        conversion_typing_supplements: payload.conversion_typing_supplements,
        synthesis_codes: payload.synthesis_codes,
        q0_inventory: Q0InventoryWireV1 {
            ordered_rules: EXACT_Q0_INVENTORY_V1.to_vec(),
        },
        fresh_rule_schemas: payload.fresh_rule_schemas,
        family_inventory: FamilyInventoryWireV1 {
            ordered_codes: EXACT_FAMILY_INVENTORY_V1.to_vec(),
        },
        family_payloads: payload.family_payloads,
    };
    let canonical_bytes = encode_bundle_v1(&bundle)?;
    let decoded = decode_bundle_v1(&canonical_bytes)?;
    verified_canonical_production_bundle_from_exact_round_trip_v1(
        manifest.candidate_digest(),
        signature.digest(),
        slots.digest(),
        canonical_bytes,
        &decoded,
    )
    .ok_or(ProductionWireBuilderFailureV1::ExactRoundTripMismatch)
}
