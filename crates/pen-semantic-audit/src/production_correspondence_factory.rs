//! The single private correspondence factory (Phase H).
//!
//! This module is deliberately mounted as a **child module** of
//! [`crate::production_refinement_theorem`], so it is the only code in the
//! crate — besides that module itself, which constructs none of them — that
//! can touch the private fields of the four correspondence capabilities and
//! the combined production refinement. The Rust module system therefore
//! enforces the plan's single-factory rule: no other construction site can
//! exist without editing this file or its parent.
//!
//! The factory consumes the four bridge capabilities:
//!
//! - the verified canonical bundle,
//! - verified safe-Agda acceptance,
//! - the verified independent Rust replay, and
//! - the verified exact transcript agreement,
//!
//! plus the verified inputs they must all be bound to (exact V3 manifest,
//! kernel, verified signature, predecessor-public delta-policy binding, the
//! pinned abstract typing foundation, and the pinned production Agda
//! foundation). Before minting anything it re-checks, in order: exact V3
//! manifest identity; one semantic-manifest and one bundle identity across
//! all four capabilities; that the transcript agreement binds exactly the
//! presented capability instances; **actual byte equality** of the four
//! retained bundle byte arrays and the three retained transcript byte
//! arrays; recomputation of the bundle and transcript digests from those
//! actual bytes; signature, slot-table, kernel-protocol, and
//! synthesis-protocol identity; the delta-policy binding; the public-sort
//! successor prerequisite; the production inventory bridge; a fresh
//! unchanged-kernel replay of the canonical bytes with the canonical
//! transcript re-rendered from the fresh artifacts and byte-compared
//! against every retained transcript; a full independent decode of the
//! canonical bytes (which structurally re-validates every section, the
//! exact Q0/family inventories, and the canonical re-encoding);
//! re-derivation of the wire manifest, signature
//! (including the exact delta-policy entries), and global-slot sections
//! from the verified capability chain, compared against the decoded bytes;
//! the exact accepted-section mask; re-derivation of the generated
//! safe-Agda input package with its source-tree digest and expected
//! checker transcript; and the input-artifact round trip.
//!
//! Only after every check passes does it mint, in dependency order, the
//! four correspondence capabilities, the combined
//! `VerifiedLambdaUnitProductionRefinementV1`, and — through the
//! crate-private continuation constructor in [`crate::typing_metatheory`],
//! which structurally requires the four correspondences — the combined
//! `VerifiedLambdaUnitTypingMetatheoryV1`. Every theorem-family digest is
//! a domain-separated digest over the factory's evidence core, computed
//! only after the actual byte comparisons succeeded; no digest, Boolean,
//! tag, or caller assertion is accepted as evidence anywhere.

use super::{
    VerifiedFiniteContextCorrespondenceV1, VerifiedKernelBaseConversionCorrespondenceV1,
    VerifiedLambdaUnitProductionRefinementV1, VerifiedProductionRefinementAgdaFoundationV1,
    VerifiedSynthesisCodeCorrespondenceV1, VerifiedV3InventoryCorrespondenceV1,
    VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
    diagnose_production_synthesis_protocol_identity_v2,
    ProductionSynthesisProtocolIdentityFailureV1,
};
use crate::agda_gate::{package_expected_checker_transcript_v1, package_source_tree_digest_v1};
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, VerifiedSemanticAuditManifestV3,
    proposed_semantic_audit_lambda_unit_manifest_v3, verify_semantic_audit_lambda_unit_manifest_v3,
};
use crate::production_inventory_bridge::{
    ProductionInventoryBridgeFailureV1, diagnose_production_inventory_bridge_v1,
};
use crate::production_refinement::{
    ProductionRefinementFailureV1, diagnose_global_slot_table_v1,
    verify_lambda_unit_public_sort_successors_v1,
};
use crate::production_refinement_wire_authority::{
    ProductionBridgeFailureV1, VerifiedAgdaProductionAcceptanceV1,
    VerifiedCanonicalProductionBundleV1, VerifiedProductionTranscriptAgreementV1,
    VerifiedRustProductionReplayV1, assemble_wire_acceptance_package_v1,
    expected_accepted_production_section_mask_v1,
};
use crate::production_wire_builder::{
    derive_manifest_surface_wire_v1, derive_signature_surface_wire_v1,
};
use crate::production_transcript::render_production_transcript_v1;
use crate::production_wire_replay::{ProductionReplayFailureV1, replay_production_bundle_v1};
use crate::production_wire_slots::{ProductionWireSlotFailureV1, derive_global_slot_table_wire_v1};
use crate::typing_metatheory::{
    VerifiedLambdaUnitTypingFoundationV1, VerifiedLambdaUnitTypingMetatheoryV1,
    mint_lambda_unit_typing_metatheory_from_production_correspondences_v1,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, Kernel, VerifiedSignature};
use pen_production_wire::{
    EXACT_FAMILY_INVENTORY_V1, EXACT_Q0_INVENTORY_V1, WireErrorV1, decode_bundle_v1,
};

/// Every verified input the factory consumes. All capability fields are
/// opaque, non-deserializable types with deterministic constructors; the
/// factory re-checks their mutual bindings and the actual retained bytes
/// rather than trusting any of them individually.
#[derive(Clone, Copy)]
pub struct ProductionCorrespondenceFactoryInputV1<'input> {
    pub manifest: &'input VerifiedSemanticAuditManifestV3,
    pub kernel: &'input Kernel,
    pub signature: &'input VerifiedSignature,
    pub abstract_foundation: &'input VerifiedLambdaUnitTypingFoundationV1,
    pub production_agda_foundation: &'input VerifiedProductionRefinementAgdaFoundationV1,
    pub delta_policy_binding: &'input VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
    pub bundle: &'input VerifiedCanonicalProductionBundleV1,
    pub acceptance: &'input VerifiedAgdaProductionAcceptanceV1,
    pub replay: &'input VerifiedRustProductionReplayV1,
    pub agreement: &'input VerifiedProductionTranscriptAgreementV1,
}

/// Fail-closed factory failures. No variant can be bypassed by a
/// caller-supplied digest, Boolean, tag, or section mask, and no variant
/// carries partial authority.
#[derive(Clone, Debug)]
pub enum ProductionCorrespondenceFactoryFailureV1 {
    ExactV3ManifestIdentityMismatch,
    SemanticManifestBindingMismatch,
    BundleIdentityBindingMismatch,
    AgreementCapabilityBindingMismatch,
    BundleBytesMismatch,
    TranscriptBytesMismatch,
    BundleDigestRecomputationMismatch,
    TranscriptDigestRecomputationMismatch,
    SignatureBindingMismatch,
    GlobalSlotTable(ProductionRefinementFailureV1),
    SlotTableBindingMismatch,
    KernelProtocolMismatch,
    SynthesisProtocolIdentity(ProductionSynthesisProtocolIdentityFailureV1),
    SynthesisProtocolMismatch,
    DeltaPolicyBindingMismatch,
    PublicSortSuccessorsOutsideFragment(OutsideFragmentReason),
    PublicSortSuccessorsUnavailable(AuditUnknownReason),
    ProductionInventoryBridge(ProductionInventoryBridgeFailureV1),
    Replay(ProductionReplayFailureV1),
    RecomputedTranscriptMismatch,
    WireDecode(WireErrorV1),
    SurfaceDerivation(ProductionWireSlotFailureV1),
    ManifestSurfaceMismatch,
    SignatureSurfaceMismatch,
    GlobalSlotTableSurfaceMismatch,
    InventorySurfaceMismatch,
    AcceptedSectionsMismatch,
    PackageAssembly(ProductionBridgeFailureV1),
    InputArtifactBindingMismatch,
    SourceTreeMismatch,
    CheckerTranscriptMismatch,
}

impl std::fmt::Display for ProductionCorrespondenceFactoryFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactV3ManifestIdentityMismatch => formatter.write_str(
                "semantic manifest is not the exact generic unfrozen lambda/unit V3 candidate",
            ),
            Self::SemanticManifestBindingMismatch => formatter.write_str(
                "the four bridge capabilities are not all bound to the presented V3 manifest",
            ),
            Self::BundleIdentityBindingMismatch => formatter
                .write_str("the four bridge capabilities do not name one canonical bundle digest"),
            Self::AgreementCapabilityBindingMismatch => formatter.write_str(
                "the transcript agreement does not bind the presented bundle, acceptance, and replay capability instances",
            ),
            Self::BundleBytesMismatch => formatter.write_str(
                "the retained canonical bundle byte arrays are not all byte-identical",
            ),
            Self::TranscriptBytesMismatch => formatter.write_str(
                "the retained canonical transcript byte arrays are not all byte-identical",
            ),
            Self::BundleDigestRecomputationMismatch => formatter.write_str(
                "recomputing the bundle digest from the actual bytes contradicts the recorded digest",
            ),
            Self::TranscriptDigestRecomputationMismatch => formatter.write_str(
                "recomputing the transcript digest from the actual bytes contradicts the recorded digest",
            ),
            Self::SignatureBindingMismatch => formatter.write_str(
                "the canonical bundle is not bound to the presented verified signature",
            ),
            Self::GlobalSlotTable(failure) => {
                write!(formatter, "global slot table derivation failed: {failure}")
            }
            Self::SlotTableBindingMismatch => formatter.write_str(
                "the re-derived global slot table digest contradicts the bundle or delta-policy binding",
            ),
            Self::KernelProtocolMismatch => formatter.write_str(
                "the replay kernel protocol is not the presented kernel's protocol",
            ),
            Self::SynthesisProtocolIdentity(failure) => write!(
                formatter,
                "synthesis protocol V2 identity failed: {failure:?}"
            ),
            Self::SynthesisProtocolMismatch => formatter.write_str(
                "the replay synthesis protocol is not the verified V2 implementation protocol",
            ),
            Self::DeltaPolicyBindingMismatch => formatter.write_str(
                "the predecessor-public delta-policy binding does not match the manifest, signature, and slot table",
            ),
            Self::PublicSortSuccessorsOutsideFragment(reason) => write!(
                formatter,
                "public-sort successor prerequisite left the fragment: {reason:?}"
            ),
            Self::PublicSortSuccessorsUnavailable(reason) => write!(
                formatter,
                "public-sort successor prerequisite is unavailable: {reason:?}"
            ),
            Self::ProductionInventoryBridge(failure) => {
                write!(formatter, "production inventory bridge failed: {failure}")
            }
            Self::Replay(failure) => {
                write!(formatter, "fresh unchanged-kernel replay failed: {failure}")
            }
            Self::RecomputedTranscriptMismatch => formatter.write_str(
                "the freshly replayed and rendered transcript contradicts the retained transcripts",
            ),
            Self::WireDecode(error) => {
                write!(formatter, "independent canonical decode failed: {error}")
            }
            Self::SurfaceDerivation(error) => {
                write!(formatter, "wire surface derivation failed: {error}")
            }
            Self::ManifestSurfaceMismatch => formatter.write_str(
                "the decoded manifest section is not the capability-derived manifest surface",
            ),
            Self::SignatureSurfaceMismatch => formatter.write_str(
                "the decoded signature section is not the capability-derived signature surface",
            ),
            Self::GlobalSlotTableSurfaceMismatch => formatter.write_str(
                "the decoded global-slot section is not the verified-signature-derived slot table",
            ),
            Self::InventorySurfaceMismatch => formatter.write_str(
                "the decoded Q0 or family inventory is not the exact required inventory",
            ),
            Self::AcceptedSectionsMismatch => formatter.write_str(
                "the acceptance capability does not carry the exact required section mask",
            ),
            Self::PackageAssembly(failure) => {
                write!(formatter, "acceptance package re-derivation failed: {failure}")
            }
            Self::InputArtifactBindingMismatch => formatter.write_str(
                "re-deriving the generated Agda input artifact contradicts the acceptance capability",
            ),
            Self::SourceTreeMismatch => formatter.write_str(
                "the re-derived acceptance package source tree contradicts the acceptance capability",
            ),
            Self::CheckerTranscriptMismatch => formatter.write_str(
                "the expected pinned-checker transcript contradicts the acceptance capability",
            ),
        }
    }
}

impl std::error::Error for ProductionCorrespondenceFactoryFailureV1 {}

impl From<ProductionWireSlotFailureV1> for ProductionCorrespondenceFactoryFailureV1 {
    fn from(error: ProductionWireSlotFailureV1) -> Self {
        Self::SurfaceDerivation(error)
    }
}

/// The factory's complete output: the four correspondence capabilities,
/// the combined production refinement, and the combined typing
/// metatheory, all minted over one canonical bundle in one call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintedProductionCorrespondencesV1 {
    context_correspondence: VerifiedFiniteContextCorrespondenceV1,
    conversion_correspondence: VerifiedKernelBaseConversionCorrespondenceV1,
    synthesis_correspondence: VerifiedSynthesisCodeCorrespondenceV1,
    inventory_correspondence: VerifiedV3InventoryCorrespondenceV1,
    production_refinement: VerifiedLambdaUnitProductionRefinementV1,
    typing_metatheory: VerifiedLambdaUnitTypingMetatheoryV1,
    evidence_digest: Digest,
}

impl MintedProductionCorrespondencesV1 {
    pub fn context_correspondence(&self) -> &VerifiedFiniteContextCorrespondenceV1 {
        &self.context_correspondence
    }

    pub fn conversion_correspondence(&self) -> &VerifiedKernelBaseConversionCorrespondenceV1 {
        &self.conversion_correspondence
    }

    pub fn synthesis_correspondence(&self) -> &VerifiedSynthesisCodeCorrespondenceV1 {
        &self.synthesis_correspondence
    }

    pub fn inventory_correspondence(&self) -> &VerifiedV3InventoryCorrespondenceV1 {
        &self.inventory_correspondence
    }

    pub fn production_refinement(&self) -> &VerifiedLambdaUnitProductionRefinementV1 {
        &self.production_refinement
    }

    pub fn typing_metatheory(&self) -> &VerifiedLambdaUnitTypingMetatheoryV1 {
        &self.typing_metatheory
    }

    /// The evidence-core digest every theorem-family digest is bound to.
    /// Recorded after the actual byte comparisons; not accepted as input
    /// anywhere.
    pub fn evidence_digest(&self) -> &Digest {
        &self.evidence_digest
    }
}

/// Everything the theorem-family digests bind: the complete identity
/// surface the factory actually checked, recorded only after the actual
/// byte comparisons succeeded.
struct CorrespondenceEvidenceCoreV1<'a> {
    semantic_manifest_digest: &'a Digest,
    signature_digest: &'a Digest,
    kernel_protocol_digest: &'a Digest,
    global_slot_table_digest: &'a Digest,
    public_sort_successor_digest: &'a Digest,
    synthesis_protocol_identity_digest: &'a Digest,
    delta_policy_binding_digest: &'a Digest,
    inventory_bridge_digest: &'a Digest,
    abstract_foundation_digest: &'a Digest,
    production_agda_foundation_digest: &'a Digest,
    bundle_capability_digest: &'a Digest,
    bundle_digest: &'a Digest,
    acceptance_digest: &'a Digest,
    safe_source_tree_digest: &'a Digest,
    checker_transcript_digest: &'a Digest,
    input_artifact_digest: &'a Digest,
    replay_digest: &'a Digest,
    agreement_digest: &'a Digest,
    transcript_digest: &'a Digest,
}

impl CanonicalEncode for CorrespondenceEvidenceCoreV1<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(1);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.global_slot_table_digest.encode_canonical(encoder);
        self.public_sort_successor_digest.encode_canonical(encoder);
        self.synthesis_protocol_identity_digest
            .encode_canonical(encoder);
        self.delta_policy_binding_digest.encode_canonical(encoder);
        self.inventory_bridge_digest.encode_canonical(encoder);
        self.abstract_foundation_digest.encode_canonical(encoder);
        self.production_agda_foundation_digest
            .encode_canonical(encoder);
        self.bundle_capability_digest.encode_canonical(encoder);
        self.bundle_digest.encode_canonical(encoder);
        self.acceptance_digest.encode_canonical(encoder);
        self.safe_source_tree_digest.encode_canonical(encoder);
        self.checker_transcript_digest.encode_canonical(encoder);
        self.input_artifact_digest.encode_canonical(encoder);
        self.replay_digest.encode_canonical(encoder);
        self.agreement_digest.encode_canonical(encoder);
        self.transcript_digest.encode_canonical(encoder);
    }
}

fn theorem_family_digest(domain: &str, evidence: &Digest) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u16(1);
    evidence.encode_canonical(&mut encoder);
    Digest::of_domain_bytes(domain, encoder.as_bytes())
}

/// The single private correspondence factory.
///
/// This is the only function in the crate that can return the four
/// correspondence capabilities, the combined production refinement, or
/// the combined typing metatheory.
///
/// The capability fields stay private outside this module tree, so a
/// caller cannot forge a correspondence by struct literal or reach into
/// one it holds:
///
/// ```compile_fail
/// use pen_semantic_audit::VerifiedFiniteContextCorrespondenceV1;
/// fn forge(capability: &VerifiedFiniteContextCorrespondenceV1) {
///     let _ = &capability.semantic_manifest_digest; // private field
/// }
/// ```
///
/// ```compile_fail
/// use pen_semantic_audit::VerifiedLambdaUnitTypingMetatheoryV1;
/// fn forge() -> VerifiedLambdaUnitTypingMetatheoryV1 {
///     VerifiedLambdaUnitTypingMetatheoryV1 {} // no public constructor
/// }
/// ```
pub fn mint_production_correspondences_v1(
    input: &ProductionCorrespondenceFactoryInputV1<'_>,
) -> Result<MintedProductionCorrespondencesV1, ProductionCorrespondenceFactoryFailureV1> {
    let ProductionCorrespondenceFactoryInputV1 {
        manifest,
        kernel,
        signature,
        abstract_foundation,
        production_agda_foundation,
        delta_policy_binding,
        bundle,
        acceptance,
        replay,
        agreement,
    } = input;

    // 1. Exact V3 manifest identity.
    let exact_wire = proposed_semantic_audit_lambda_unit_manifest_v3();
    let AuditDecision::Proven(exact) = verify_semantic_audit_lambda_unit_manifest_v3(&exact_wire)
    else {
        return Err(ProductionCorrespondenceFactoryFailureV1::ExactV3ManifestIdentityMismatch);
    };
    if manifest.manifest() != &exact_wire || manifest.candidate_digest() != exact.candidate_digest()
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::ExactV3ManifestIdentityMismatch);
    }

    // 2. One semantic-manifest identity across all four capabilities.
    let manifest_digest = manifest.candidate_digest();
    if bundle.semantic_manifest_digest() != manifest_digest
        || acceptance.semantic_manifest_digest() != manifest_digest
        || replay.semantic_manifest_digest() != manifest_digest
        || agreement.semantic_manifest_digest() != manifest_digest
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::SemanticManifestBindingMismatch);
    }

    // 3. One canonical-bundle identity across all four capabilities.
    if acceptance.bundle_digest() != bundle.bundle_digest()
        || replay.bundle_digest() != bundle.bundle_digest()
        || agreement.bundle_digest() != bundle.bundle_digest()
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::BundleIdentityBindingMismatch);
    }

    // 4. The agreement must bind exactly the presented capability
    //    instances, not merely same-named ones.
    if agreement.canonical_bundle_capability_digest() != bundle.digest()
        || agreement.agda_acceptance_digest() != acceptance.digest()
        || agreement.rust_replay_digest() != replay.digest()
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::AgreementCapabilityBindingMismatch);
    }

    // 5. Actual bundle byte equality across all four retained arrays.
    let canonical_bytes = bundle.canonical_bytes();
    if acceptance.accepted_bundle_bytes() != canonical_bytes
        || replay.replayed_bundle_bytes() != canonical_bytes
        || agreement.canonical_bundle_bytes() != canonical_bytes
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::BundleBytesMismatch);
    }

    // 6. Actual transcript byte equality across all three retained arrays.
    let transcript_bytes = replay.rust_transcript();
    if acceptance.agda_transcript() != transcript_bytes
        || agreement.transcript_bytes() != transcript_bytes
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::TranscriptBytesMismatch);
    }

    // 7. Digest recomputation from the actual bytes. Digests record
    //    successful comparisons; recomputing them here keeps a recorded
    //    digest from ever substituting for the bytes.
    if &Digest::of_domain_bytes(
        "law-v2-production-refinement-canonical-bundle/v1",
        canonical_bytes,
    ) != bundle.bundle_digest()
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::BundleDigestRecomputationMismatch);
    }
    if &Digest::of_domain_bytes(
        "law-v2-production-refinement-canonical-transcript/v1",
        transcript_bytes,
    ) != agreement.transcript_digest()
    {
        return Err(
            ProductionCorrespondenceFactoryFailureV1::TranscriptDigestRecomputationMismatch,
        );
    }

    // 8. Signature binding.
    if bundle.verified_signature_digest() != signature.digest() {
        return Err(ProductionCorrespondenceFactoryFailureV1::SignatureBindingMismatch);
    }

    // 9. Slot-table re-derivation and binding.
    let slots = diagnose_global_slot_table_v1(kernel, signature)
        .map_err(ProductionCorrespondenceFactoryFailureV1::GlobalSlotTable)?;
    if bundle.global_slot_table_digest() != slots.digest()
        || slots.signature_digest() != signature.digest()
        || slots.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::SlotTableBindingMismatch);
    }

    // 10. Kernel protocol identity.
    if replay.kernel_protocol_digest() != &kernel.kernel_protocol_digest() {
        return Err(ProductionCorrespondenceFactoryFailureV1::KernelProtocolMismatch);
    }

    // 11. Synthesis protocol identity.
    let synthesis_protocol = diagnose_production_synthesis_protocol_identity_v2()
        .map_err(ProductionCorrespondenceFactoryFailureV1::SynthesisProtocolIdentity)?;
    if replay.synthesis_protocol_digest() != synthesis_protocol.implementation_protocol_digest() {
        return Err(ProductionCorrespondenceFactoryFailureV1::SynthesisProtocolMismatch);
    }

    // 12. Delta-policy binding to the exact manifest, signature, and
    //     re-derived slot table.
    if delta_policy_binding.lambda_unit_v3_manifest_digest() != manifest_digest
        || delta_policy_binding.successor_signature_digest() != signature.digest()
        || delta_policy_binding.global_slot_table_digest() != slots.digest()
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::DeltaPolicyBindingMismatch);
    }

    // 13. Public-sort successor prerequisite (aggregate-diagnostic parity).
    let public_sort_successors =
        match verify_lambda_unit_public_sort_successors_v1(manifest, kernel, signature, &slots) {
            AuditDecision::Proven(verified) => verified,
            AuditDecision::OutsideFragment(reason) => {
                return Err(
                    ProductionCorrespondenceFactoryFailureV1::PublicSortSuccessorsOutsideFragment(
                        reason,
                    ),
                );
            }
            AuditDecision::Unknown(reason) => {
                return Err(
                    ProductionCorrespondenceFactoryFailureV1::PublicSortSuccessorsUnavailable(
                        reason,
                    ),
                );
            }
        };

    // 14. Production inventory bridge.
    let inventory_bridge = diagnose_production_inventory_bridge_v1(manifest)
        .map_err(ProductionCorrespondenceFactoryFailureV1::ProductionInventoryBridge)?;
    if inventory_bridge.semantic_manifest_digest() != manifest_digest {
        return Err(ProductionCorrespondenceFactoryFailureV1::ProductionInventoryBridge(
            ProductionInventoryBridgeFailureV1::ManifestMismatch,
        ));
    }

    // 15. Fresh independent replay: the factory does not merely transfer
    //     the replay capability's evidence — it re-runs the complete
    //     unchanged-kernel replay over the byte-compared bytes, re-renders
    //     the canonical transcript from the fresh computed artifacts, and
    //     requires it byte-identical to every retained transcript. A
    //     decode success below is likewise a fresh structural verdict:
    //     `decode_bundle_v1` re-validates every structural invariant
    //     (including the exact Q0/family inventories) and requires
    //     byte-identical canonical re-encoding.
    let fresh_replay = replay_production_bundle_v1(canonical_bytes)
        .map_err(ProductionCorrespondenceFactoryFailureV1::Replay)?;
    let decoded = decode_bundle_v1(canonical_bytes)
        .map_err(ProductionCorrespondenceFactoryFailureV1::WireDecode)?;
    if fresh_replay.replayed_bytes != canonical_bytes
        || fresh_replay.kernel_protocol_digest != kernel.kernel_protocol_digest()
        || &fresh_replay.synthesis_protocol_digest != replay.synthesis_protocol_digest()
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::RecomputedTranscriptMismatch);
    }
    let fresh_transcript = render_production_transcript_v1(&decoded, &fresh_replay.computed);
    if fresh_transcript != transcript_bytes {
        return Err(ProductionCorrespondenceFactoryFailureV1::RecomputedTranscriptMismatch);
    }

    // 16. The decoded manifest, signature, and slot-table sections must
    //     be exactly what the verified capability chain derives; the
    //     inventories must be exactly the required lists.
    let expected_manifest_surface = derive_manifest_surface_wire_v1(
        manifest,
        &inventory_bridge,
        delta_policy_binding,
        &synthesis_protocol,
    )?;
    if decoded.manifest_surface != expected_manifest_surface {
        return Err(ProductionCorrespondenceFactoryFailureV1::ManifestSurfaceMismatch);
    }
    let expected_signature_surface =
        derive_signature_surface_wire_v1(signature, &slots, delta_policy_binding)?;
    if decoded.signature != expected_signature_surface {
        return Err(ProductionCorrespondenceFactoryFailureV1::SignatureSurfaceMismatch);
    }
    let expected_slot_table = derive_global_slot_table_wire_v1(signature, &slots)?;
    if decoded.global_slot_table != expected_slot_table {
        return Err(ProductionCorrespondenceFactoryFailureV1::GlobalSlotTableSurfaceMismatch);
    }
    if decoded.q0_inventory.ordered_rules != EXACT_Q0_INVENTORY_V1
        || decoded.family_inventory.ordered_codes != EXACT_FAMILY_INVENTORY_V1
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::InventorySurfaceMismatch);
    }

    // 17. Exact accepted-section mask.
    if acceptance.accepted_sections() != &expected_accepted_production_section_mask_v1() {
        return Err(ProductionCorrespondenceFactoryFailureV1::AcceptedSectionsMismatch);
    }

    // 18. Re-derive the generated acceptance package from the actual
    //     bytes: the input artifact must round-trip to exactly the
    //     artifact the acceptance capability carries, and the package
    //     source-tree digest and expected pinned-checker transcript must
    //     match what the acceptance run recorded.
    let (package_sources, input_artifact) =
        assemble_wire_acceptance_package_v1(canonical_bytes, transcript_bytes)
            .map_err(ProductionCorrespondenceFactoryFailureV1::PackageAssembly)?;
    if &input_artifact != acceptance.input_artifact() {
        return Err(ProductionCorrespondenceFactoryFailureV1::InputArtifactBindingMismatch);
    }
    if &package_source_tree_digest_v1(&package_sources) != acceptance.safe_source_tree_digest() {
        return Err(ProductionCorrespondenceFactoryFailureV1::SourceTreeMismatch);
    }
    let expected_checker_transcript = package_expected_checker_transcript_v1(&package_sources);
    if &Digest::of_bytes(expected_checker_transcript.as_bytes())
        != acceptance.checker_transcript_digest()
    {
        return Err(ProductionCorrespondenceFactoryFailureV1::CheckerTranscriptMismatch);
    }

    // Every check has passed. Record the evidence core, then mint.
    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    let evidence_digest = Digest::of_canonical(
        "pen-semantic-audit/production-correspondence-evidence-core/v1",
        &CorrespondenceEvidenceCoreV1 {
            semantic_manifest_digest: manifest_digest,
            signature_digest: signature.digest(),
            kernel_protocol_digest: &kernel_protocol_digest,
            global_slot_table_digest: slots.digest(),
            public_sort_successor_digest: public_sort_successors.digest(),
            synthesis_protocol_identity_digest: synthesis_protocol.digest(),
            delta_policy_binding_digest: delta_policy_binding.digest(),
            inventory_bridge_digest: inventory_bridge.digest(),
            abstract_foundation_digest: abstract_foundation.digest(),
            production_agda_foundation_digest: production_agda_foundation.digest(),
            bundle_capability_digest: bundle.digest(),
            bundle_digest: bundle.bundle_digest(),
            acceptance_digest: acceptance.digest(),
            safe_source_tree_digest: acceptance.safe_source_tree_digest(),
            checker_transcript_digest: acceptance.checker_transcript_digest(),
            input_artifact_digest: input_artifact.digest(),
            replay_digest: replay.digest(),
            agreement_digest: agreement.digest(),
            transcript_digest: agreement.transcript_digest(),
        },
    );
    let family = |domain: &str| theorem_family_digest(domain, &evidence_digest);

    let mut context_correspondence = VerifiedFiniteContextCorrespondenceV1 {
        semantic_manifest_digest: manifest_digest.clone(),
        global_slot_table: slots.clone(),
        agda_foundation: (*production_agda_foundation).clone(),
        variable_lookup_correspondence_digest: family(
            "pen-semantic-audit/production-correspondence/variable-lookup/v1",
        ),
        extension_correspondence_digest: family(
            "pen-semantic-audit/production-correspondence/context-extension/v1",
        ),
        shift_substitution_correspondence_digest: family(
            "pen-semantic-audit/production-correspondence/shift-substitution/v1",
        ),
        structural_round_trip_digest: family(
            "pen-semantic-audit/production-correspondence/context-structural-round-trip/v1",
        ),
        digest: Digest::of_bytes(b"pending finite context correspondence"),
    };
    context_correspondence.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-finite-context-correspondence/v1",
        &context_correspondence,
    );

    let mut conversion_correspondence = VerifiedKernelBaseConversionCorrespondenceV1 {
        semantic_manifest_digest: manifest_digest.clone(),
        kernel_protocol_digest: kernel_protocol_digest.clone(),
        synthesis_protocol_identity: synthesis_protocol.clone(),
        predecessor_public_delta_policy: (*delta_policy_binding).clone(),
        typed_step_preservation_digest: family(
            "pen-semantic-audit/production-correspondence/typed-step-preservation/v1",
        ),
        typed_trace_preservation_digest: family(
            "pen-semantic-audit/production-correspondence/typed-trace-preservation/v1",
        ),
        common_normal_form_conversion_digest: family(
            "pen-semantic-audit/production-correspondence/common-normal-form-conversion/v1",
        ),
        substitution_stability_digest: family(
            "pen-semantic-audit/production-correspondence/conversion-substitution-stability/v1",
        ),
        agda_foundation: (*production_agda_foundation).clone(),
        digest: Digest::of_bytes(b"pending kernel base conversion correspondence"),
    };
    conversion_correspondence.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-kernel-base-conversion-correspondence/v1",
        &conversion_correspondence,
    );

    let mut synthesis_correspondence = VerifiedSynthesisCodeCorrespondenceV1 {
        semantic_manifest_digest: manifest_digest.clone(),
        context_correspondence: context_correspondence.clone(),
        conversion_correspondence: conversion_correspondence.clone(),
        synthesis_protocol_identity: synthesis_protocol.clone(),
        checker_soundness_digest: family(
            "pen-semantic-audit/production-correspondence/synthesis-checker-soundness/v1",
        ),
        production_image_reification_digest: family(
            "pen-semantic-audit/production-correspondence/production-image-reification/v1",
        ),
        structural_round_trip_digest: family(
            "pen-semantic-audit/production-correspondence/synthesis-structural-round-trip/v1",
        ),
        exact_eight_rule_coverage_digest: family(
            "pen-semantic-audit/production-correspondence/exact-eight-rule-coverage/v1",
        ),
        agda_foundation: (*production_agda_foundation).clone(),
        digest: Digest::of_bytes(b"pending synthesis code correspondence"),
    };
    synthesis_correspondence.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-synthesis-code-correspondence/v1",
        &synthesis_correspondence,
    );

    let mut inventory_correspondence = VerifiedV3InventoryCorrespondenceV1 {
        semantic_manifest_digest: manifest_digest.clone(),
        rust_inventory_bridge: inventory_bridge,
        representation_rule_bridge_digest: family(
            "pen-semantic-audit/production-correspondence/representation-rule-bridge/v1",
        ),
        base_q0_bridge_digest: family(
            "pen-semantic-audit/production-correspondence/base-q0-bridge/v1",
        ),
        fresh_rule_schema_bridge_digest: family(
            "pen-semantic-audit/production-correspondence/fresh-rule-schema-bridge/v1",
        ),
        family_constructor_bridge_digest: family(
            "pen-semantic-audit/production-correspondence/family-constructor-bridge/v1",
        ),
        no_extra_no_missing_digest: family(
            "pen-semantic-audit/production-correspondence/inventory-no-extra-no-missing/v1",
        ),
        agda_foundation: (*production_agda_foundation).clone(),
        digest: Digest::of_bytes(b"pending V3 inventory correspondence"),
    };
    inventory_correspondence.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-v3-inventory-correspondence/v1",
        &inventory_correspondence,
    );

    let mut production_refinement = VerifiedLambdaUnitProductionRefinementV1 {
        semantic_manifest_digest: manifest_digest.clone(),
        abstract_foundation: (*abstract_foundation).clone(),
        context_correspondence: context_correspondence.clone(),
        conversion_correspondence: conversion_correspondence.clone(),
        synthesis_correspondence: synthesis_correspondence.clone(),
        inventory_correspondence: inventory_correspondence.clone(),
        agda_foundation: (*production_agda_foundation).clone(),
        digest: Digest::of_bytes(b"pending lambda/unit production refinement"),
    };
    production_refinement.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-lambda-unit-production-refinement/v1",
        &production_refinement,
    );

    let typing_metatheory = mint_lambda_unit_typing_metatheory_from_production_correspondences_v1(
        manifest,
        kernel,
        abstract_foundation,
        &context_correspondence,
        &conversion_correspondence,
        &synthesis_correspondence,
        &inventory_correspondence,
        synthesis_protocol.implementation_protocol_digest(),
        &evidence_digest,
    );

    Ok(MintedProductionCorrespondencesV1 {
        context_correspondence,
        conversion_correspondence,
        synthesis_correspondence,
        inventory_correspondence,
        production_refinement,
        typing_metatheory,
        evidence_digest,
    })
}
