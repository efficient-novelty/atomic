//! Fail-closed authority boundary for the canonical production-refinement
//! bundle protocol.
//!
//! The production wire, safe-Agda decoder/checker, and independent Rust
//! replay are intentionally separate implementations. Similar data types,
//! successful local checks, or matching digests do not prove that those
//! implementations accepted the same semantic judgments. Authority requires
//! all of the following over the exact V3 candidate:
//!
//! - one canonical bundle whose signature/global-slot section was derived
//!   from a [`pen_kernel::VerifiedSignature`] and
//!   [`crate::production_refinement::VerifiedGlobalSlotTableV1`], never from
//!   caller-supplied ordering;
//! - safe-Agda acceptance of the exact bytes embedded in its input artifact;
//! - independent Rust decoding, checking, and unchanged-kernel replay of the
//!   exact same bytes; and
//! - equality of the actual canonical transcript bytes, not merely their
//!   digests.
//!
//! This module freezes the capability shapes and reports the exact open
//! frontier. It has no capability constructors, deserialization path, or
//! digest-only escape hatch. In particular, it cannot mint any of the four
//! correspondence capabilities in `production_refinement_theorem`.

use crate::manifest::{
    AuditDecision, VerifiedSemanticAuditManifestV3,
    proposed_semantic_audit_lambda_unit_manifest_v3, verify_semantic_audit_lambda_unit_manifest_v3,
};
use crate::production_refinement_theorem::{
    PRODUCTION_REFINEMENT_EXCLUDED_DOWNSTREAM_OBLIGATION_V1,
    ProductionRefinementExcludedDownstreamObligationV1,
};
use crate::production_wire_input::VerifiedAgdaProductionInputArtifactV1;
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use std::sync::Arc;

pub const CANONICAL_PRODUCTION_AUTHORITY_SCHEMA_VERSION_V1: u16 = 1;

/// The four semantic sections that safe Agda must accept for one bundle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcceptedProductionSectionV1 {
    FiniteContextsAndGlobals,
    BaseConversions,
    SynthesisCodes,
    V3InventoryAndFamilies,
}

impl CanonicalEncode for AcceptedProductionSectionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::FiniteContextsAndGlobals => 0,
            Self::BaseConversions => 1,
            Self::SynthesisCodes => 2,
            Self::V3InventoryAndFamilies => 3,
        });
    }
}

pub const REQUIRED_ACCEPTED_PRODUCTION_SECTIONS_V1: &[AcceptedProductionSectionV1] = &[
    AcceptedProductionSectionV1::FiniteContextsAndGlobals,
    AcceptedProductionSectionV1::BaseConversions,
    AcceptedProductionSectionV1::SynthesisCodes,
    AcceptedProductionSectionV1::V3InventoryAndFamilies,
];

/// Opaque exact-section mask emitted by the future safe-Agda acceptance
/// bridge. It is not a caller-provided bit field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcceptedProductionSectionMaskV1 {
    sections: Arc<[AcceptedProductionSectionV1]>,
    digest: Digest,
}

impl AcceptedProductionSectionMaskV1 {
    pub fn sections(&self) -> &[AcceptedProductionSectionV1] {
        &self.sections
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for AcceptedProductionSectionMaskV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.sections);
    }
}

/// Opaque canonical-bundle capability reserved for the semantic-side wire
/// builder.
///
/// A future constructor must consume the exact verified signature and
/// declaration-order global-slot table, derive every wire slot internally,
/// decode the resulting bytes, re-encode them, and compare the actual byte
/// arrays. Supplying an ordered wire signature directly is forbidden.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCanonicalProductionBundleV1 {
    semantic_manifest_digest: Digest,
    verified_signature_digest: Digest,
    global_slot_table_digest: Digest,
    canonical_bytes: Arc<[u8]>,
    rust_decode_digest: Digest,
    rust_reencode_digest: Digest,
    bundle_digest: Digest,
    digest: Digest,
}

impl VerifiedCanonicalProductionBundleV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn verified_signature_digest(&self) -> &Digest {
        &self.verified_signature_digest
    }

    pub fn global_slot_table_digest(&self) -> &Digest {
        &self.global_slot_table_digest
    }

    pub fn canonical_bytes(&self) -> &[u8] {
        &self.canonical_bytes
    }

    pub fn rust_decode_digest(&self) -> &Digest {
        &self.rust_decode_digest
    }

    pub fn rust_reencode_digest(&self) -> &Digest {
        &self.rust_reencode_digest
    }

    pub fn bundle_digest(&self) -> &Digest {
        &self.bundle_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedCanonicalProductionBundleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.verified_signature_digest.encode_canonical(encoder);
        self.global_slot_table_digest.encode_canonical(encoder);
        encoder.bytes(&self.canonical_bytes);
        self.rust_decode_digest.encode_canonical(encoder);
        self.rust_reencode_digest.encode_canonical(encoder);
        self.bundle_digest.encode_canonical(encoder);
    }
}

pub(crate) fn verified_canonical_production_bundle_from_exact_round_trip_v1(
    semantic_manifest_digest: &Digest,
    verified_signature_digest: &Digest,
    global_slot_table_digest: &Digest,
    canonical_bytes: Vec<u8>,
    decoded: &pen_production_wire::ProductionRefinementBundleV1,
) -> Option<VerifiedCanonicalProductionBundleV1> {
    let reencoded = pen_production_wire::canonical_reencode_bundle_v1(decoded).ok()?;
    if reencoded != canonical_bytes {
        return None;
    }
    let canonical_bytes = Arc::<[u8]>::from(canonical_bytes);
    let rust_decode_digest = Digest::of_domain_bytes(
        "law-v2-production-refinement-rust-decoded-bundle/v1",
        &reencoded,
    );
    let rust_reencode_digest = Digest::of_domain_bytes(
        "law-v2-production-refinement-rust-reencoded-bundle/v1",
        &reencoded,
    );
    let bundle_digest = Digest::of_domain_bytes(
        "law-v2-production-refinement-canonical-bundle/v1",
        &canonical_bytes,
    );
    let mut verified = VerifiedCanonicalProductionBundleV1 {
        semantic_manifest_digest: semantic_manifest_digest.clone(),
        verified_signature_digest: verified_signature_digest.clone(),
        global_slot_table_digest: global_slot_table_digest.clone(),
        canonical_bytes,
        rust_decode_digest,
        rust_reencode_digest,
        bundle_digest,
        digest: Digest::of_bytes(b"pending verified canonical production bundle"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-canonical-production-bundle/v1",
        &verified,
    );
    Some(verified)
}
/// Opaque safe-Agda acceptance capability.
///
/// `accepted_bundle_bytes` are the bytes extracted back out of the generated
/// Agda input module. They are retained so that the eventual factory can
/// compare actual bytes with the canonical artifact. Source-tree and
/// transcript digests do not replace this byte comparison.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedAgdaProductionAcceptanceV1 {
    semantic_manifest_digest: Digest,
    bundle_digest: Digest,
    input_artifact: VerifiedAgdaProductionInputArtifactV1,
    safe_source_tree_digest: Digest,
    checker_transcript_digest: Digest,
    accepted_sections: AcceptedProductionSectionMaskV1,
    agda_transcript: Arc<[u8]>,
    digest: Digest,
}

impl VerifiedAgdaProductionAcceptanceV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn bundle_digest(&self) -> &Digest {
        &self.bundle_digest
    }

    pub fn input_artifact(&self) -> &VerifiedAgdaProductionInputArtifactV1 {
        &self.input_artifact
    }

    pub fn accepted_bundle_bytes(&self) -> &[u8] {
        self.input_artifact.canonical_bytes()
    }

    pub fn safe_source_tree_digest(&self) -> &Digest {
        &self.safe_source_tree_digest
    }

    pub fn checker_transcript_digest(&self) -> &Digest {
        &self.checker_transcript_digest
    }

    pub fn accepted_sections(&self) -> &AcceptedProductionSectionMaskV1 {
        &self.accepted_sections
    }

    pub fn agda_transcript(&self) -> &[u8] {
        &self.agda_transcript
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedAgdaProductionAcceptanceV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.bundle_digest.encode_canonical(encoder);
        self.input_artifact.encode_canonical(encoder);
        self.safe_source_tree_digest.encode_canonical(encoder);
        self.checker_transcript_digest.encode_canonical(encoder);
        self.accepted_sections.encode_canonical(encoder);
        encoder.bytes(&self.agda_transcript);
    }
}

/// Opaque independent Rust replay capability.
///
/// The replayed bytes are retained so that the future factory can establish
/// exact common-input identity before it considers any digest or transcript.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedRustProductionReplayV1 {
    semantic_manifest_digest: Digest,
    bundle_digest: Digest,
    replayed_bundle_bytes: Arc<[u8]>,
    kernel_protocol_digest: Digest,
    synthesis_protocol_digest: Digest,
    rust_transcript: Arc<[u8]>,
    digest: Digest,
}

impl VerifiedRustProductionReplayV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn bundle_digest(&self) -> &Digest {
        &self.bundle_digest
    }

    pub fn replayed_bundle_bytes(&self) -> &[u8] {
        &self.replayed_bundle_bytes
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn synthesis_protocol_digest(&self) -> &Digest {
        &self.synthesis_protocol_digest
    }

    pub fn rust_transcript(&self) -> &[u8] {
        &self.rust_transcript
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedRustProductionReplayV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.bundle_digest.encode_canonical(encoder);
        encoder.bytes(&self.replayed_bundle_bytes);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.synthesis_protocol_digest.encode_canonical(encoder);
        encoder.bytes(&self.rust_transcript);
    }
}

/// Opaque proof that both implementations consumed the exact same bundle
/// bytes and emitted the exact same normalized transcript bytes.
///
/// The common bytes are stored directly. A transcript digest is recorded only
/// after byte equality and cannot be substituted for it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedProductionTranscriptAgreementV1 {
    semantic_manifest_digest: Digest,
    bundle_digest: Digest,
    canonical_bundle_bytes: Arc<[u8]>,
    canonical_bundle_capability_digest: Digest,
    agda_acceptance_digest: Digest,
    rust_replay_digest: Digest,
    transcript_bytes: Arc<[u8]>,
    transcript_digest: Digest,
    digest: Digest,
}

impl VerifiedProductionTranscriptAgreementV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn bundle_digest(&self) -> &Digest {
        &self.bundle_digest
    }

    pub fn canonical_bundle_bytes(&self) -> &[u8] {
        &self.canonical_bundle_bytes
    }

    pub fn canonical_bundle_capability_digest(&self) -> &Digest {
        &self.canonical_bundle_capability_digest
    }

    pub fn agda_acceptance_digest(&self) -> &Digest {
        &self.agda_acceptance_digest
    }

    pub fn rust_replay_digest(&self) -> &Digest {
        &self.rust_replay_digest
    }

    pub fn transcript_bytes(&self) -> &[u8] {
        &self.transcript_bytes
    }

    pub fn transcript_digest(&self) -> &Digest {
        &self.transcript_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedProductionTranscriptAgreementV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.bundle_digest.encode_canonical(encoder);
        encoder.bytes(&self.canonical_bundle_bytes);
        self.canonical_bundle_capability_digest
            .encode_canonical(encoder);
        self.agda_acceptance_digest.encode_canonical(encoder);
        self.rust_replay_digest.encode_canonical(encoder);
        encoder.bytes(&self.transcript_bytes);
        self.transcript_digest.encode_canonical(encoder);
    }
}

/// Exact open obligations before the transcript agreement capability can
/// exist.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalProductionAuthorityObligationV1 {
    CanonicalWireCodecAndFullConsumption,
    VerifiedSlotDerivedWireSignature,
    SafeAgdaGenericBundleAcceptance,
    ExactAgdaInputArtifactByteEquality,
    IndependentRustKernelReplay,
    ExactNormalizedTranscriptByteAgreement,
    SinglePrivateCorrespondenceFactory,
}

impl CanonicalEncode for CanonicalProductionAuthorityObligationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::CanonicalWireCodecAndFullConsumption => 0,
            Self::VerifiedSlotDerivedWireSignature => 1,
            Self::SafeAgdaGenericBundleAcceptance => 2,
            Self::ExactAgdaInputArtifactByteEquality => 3,
            Self::IndependentRustKernelReplay => 4,
            Self::ExactNormalizedTranscriptByteAgreement => 5,
            Self::SinglePrivateCorrespondenceFactory => 6,
        });
    }
}

pub const COMPLETED_CANONICAL_PRODUCTION_PREREQUISITES_V1:
    &[CanonicalProductionAuthorityObligationV1] = &[
    CanonicalProductionAuthorityObligationV1::CanonicalWireCodecAndFullConsumption,
    CanonicalProductionAuthorityObligationV1::VerifiedSlotDerivedWireSignature,
    CanonicalProductionAuthorityObligationV1::ExactAgdaInputArtifactByteEquality,
];

pub const CANONICAL_PRODUCTION_AUTHORITY_FRONTIER_V1:
    &[CanonicalProductionAuthorityObligationV1] = &[
    CanonicalProductionAuthorityObligationV1::SafeAgdaGenericBundleAcceptance,
    CanonicalProductionAuthorityObligationV1::IndependentRustKernelReplay,
    CanonicalProductionAuthorityObligationV1::ExactNormalizedTranscriptByteAgreement,
    CanonicalProductionAuthorityObligationV1::SinglePrivateCorrespondenceFactory,
];
/// Exact V3-bound, fail-closed canonical-bridge frontier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalProductionAuthorityFrontierV1 {
    semantic_manifest_digest: Digest,
    unresolved_obligations: &'static [CanonicalProductionAuthorityObligationV1],
    excluded_downstream_obligation: ProductionRefinementExcludedDownstreamObligationV1,
    digest: Digest,
}

impl CanonicalProductionAuthorityFrontierV1 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn unresolved_obligations(&self) -> &[CanonicalProductionAuthorityObligationV1] {
        self.unresolved_obligations
    }

    pub fn excluded_downstream_obligation(
        &self,
    ) -> ProductionRefinementExcludedDownstreamObligationV1 {
        self.excluded_downstream_obligation
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for CanonicalProductionAuthorityFrontierV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(CANONICAL_PRODUCTION_AUTHORITY_SCHEMA_VERSION_V1);
        self.semantic_manifest_digest.encode_canonical(encoder);
        encoder.sequence(self.unresolved_obligations);
        self.excluded_downstream_obligation
            .encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalProductionAuthorityFailureV1 {
    ExactV3ManifestIdentityMismatch,
    MissingCanonicalBridgeAuthorities {
        frontier: Box<CanonicalProductionAuthorityFrontierV1>,
    },
}

impl std::fmt::Display for CanonicalProductionAuthorityFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactV3ManifestIdentityMismatch => formatter.write_str(
                "semantic manifest is not the exact generic unfrozen lambda/unit V3 candidate",
            ),
            Self::MissingCanonicalBridgeAuthorities { .. } => formatter.write_str(
                "canonical bundle construction, safe-Agda acceptance, independent Rust replay, exact common-input bytes, exact transcript bytes, and the private correspondence factory are not all verified",
            ),
        }
    }
}

impl std::error::Error for CanonicalProductionAuthorityFailureV1 {}

/// Diagnose the V2 canonical production bridge.
///
/// This function first binds the exact unchanged V3 candidate and then stops
/// at the explicit bridge frontier. It cannot be made to succeed by supplying
/// bundle or transcript digests: it accepts neither.
pub fn diagnose_canonical_production_authority_v1(
    manifest: &VerifiedSemanticAuditManifestV3,
) -> Result<VerifiedProductionTranscriptAgreementV1, CanonicalProductionAuthorityFailureV1> {
    verify_exact_v3_identity(manifest)?;

    let mut frontier = CanonicalProductionAuthorityFrontierV1 {
        semantic_manifest_digest: manifest.candidate_digest().clone(),
        unresolved_obligations: CANONICAL_PRODUCTION_AUTHORITY_FRONTIER_V1,
        excluded_downstream_obligation: PRODUCTION_REFINEMENT_EXCLUDED_DOWNSTREAM_OBLIGATION_V1,
        digest: Digest::of_bytes(b"pending canonical production authority frontier"),
    };
    frontier.digest = Digest::of_canonical(
        "pen-semantic-audit/canonical-production-authority-frontier/v1",
        &frontier,
    );
    Err(
        CanonicalProductionAuthorityFailureV1::MissingCanonicalBridgeAuthorities {
            frontier: Box::new(frontier),
        },
    )
}

fn verify_exact_v3_identity(
    manifest: &VerifiedSemanticAuditManifestV3,
) -> Result<(), CanonicalProductionAuthorityFailureV1> {
    let exact_wire = proposed_semantic_audit_lambda_unit_manifest_v3();
    let AuditDecision::Proven(exact) = verify_semantic_audit_lambda_unit_manifest_v3(&exact_wire)
    else {
        return Err(CanonicalProductionAuthorityFailureV1::ExactV3ManifestIdentityMismatch);
    };
    if manifest.manifest() != &exact_wire || manifest.candidate_digest() != exact.candidate_digest()
    {
        return Err(CanonicalProductionAuthorityFailureV1::ExactV3ManifestIdentityMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{
        SemanticAuditManifestV3, proposed_semantic_audit_lambda_unit_manifest_v3,
        verify_semantic_audit_lambda_unit_manifest_v3,
    };

    #[test]
    fn exact_v3_stops_at_the_complete_byte_authority_frontier() {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v3(
            &proposed_semantic_audit_lambda_unit_manifest_v3(),
        ) else {
            panic!("exact lambda/unit V3 manifest");
        };
        let CanonicalProductionAuthorityFailureV1::MissingCanonicalBridgeAuthorities { frontier } =
            diagnose_canonical_production_authority_v1(&manifest)
                .expect_err("authority must remain unminted")
        else {
            panic!("exact V3 must stop at the canonical bridge frontier");
        };
        assert_eq!(
            frontier.unresolved_obligations(),
            CANONICAL_PRODUCTION_AUTHORITY_FRONTIER_V1
        );
        assert_eq!(
            frontier.excluded_downstream_obligation(),
            PRODUCTION_REFINEMENT_EXCLUDED_DOWNSTREAM_OBLIGATION_V1
        );
    }

    #[test]
    fn a_changed_v3_candidate_cannot_obtain_the_required_input_capability() {
        let mut wire: SemanticAuditManifestV3 = proposed_semantic_audit_lambda_unit_manifest_v3();
        wire.maximum_context_entries = wire.maximum_context_entries.saturating_sub(1);
        assert!(matches!(
            verify_semantic_audit_lambda_unit_manifest_v3(&wire),
            AuditDecision::Unknown(_)
        ));
    }
    #[test]
    fn accepted_section_order_is_exact_and_complete() {
        assert_eq!(
            REQUIRED_ACCEPTED_PRODUCTION_SECTIONS_V1,
            &[
                AcceptedProductionSectionV1::FiniteContextsAndGlobals,
                AcceptedProductionSectionV1::BaseConversions,
                AcceptedProductionSectionV1::SynthesisCodes,
                AcceptedProductionSectionV1::V3InventoryAndFamilies,
            ]
        );
    }
}
