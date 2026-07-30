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
//! This module freezes the capability shapes, reports the exact open
//! frontier, and (as of Phase G) provides the three deterministic bridge
//! constructors: the independent Rust replay through the unchanged kernel
//! and synthesis checker, the safe-Agda acceptance of the generated input
//! package under the pinned checker, and the exact common-input and
//! common-transcript byte agreement. Each constructor is a deterministic
//! replay/check gate; there is still no deserialization path and no
//! digest-only escape hatch, and this module still cannot mint any of the
//! four correspondence capabilities in `production_refinement_theorem` —
//! that remains the single private factory's frontier.

use crate::agda_gate::{
    AgdaReferenceFailureV1, FixedAgdaSourceV1, GeneratedAgdaSourceV1, PackageSourceV1,
    diagnose_generated_agda_package_v1,
};
use crate::manifest::{
    AuditDecision, VerifiedSemanticAuditManifestV3,
    proposed_semantic_audit_lambda_unit_manifest_v3, verify_semantic_audit_lambda_unit_manifest_v3,
};
use crate::production_refinement_theorem::{
    PRODUCTION_REFINEMENT_EXCLUDED_DOWNSTREAM_OBLIGATION_V1,
    ProductionRefinementExcludedDownstreamObligationV1,
};
use crate::production_transcript::render_production_transcript_v1;
use crate::production_wire_input::{
    ProductionBundleInputFailureV1, VerifiedAgdaProductionInputArtifactV1,
    render_production_bundle_input_module_v1, render_production_transcript_input_module_v1,
    verify_agda_production_input_artifact_v1,
};
use crate::production_wire_replay::{ProductionReplayFailureV1, replay_production_bundle_v1};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use pen_production_wire::decode_bundle_v1;
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

/// The fixed safe-Agda acceptance entry template. Its only inputs are
/// the two generated byte-list modules; type-checking it forces the
/// full decode, the structural/semantic/typing/inventory verdicts, and
/// the transcript agreement by refl.
const GENERATED_ACCEPTANCE_TEMPLATE_V1: &str =
    include_str!("../agda/LawV2/Wire/GeneratedBundleAcceptanceV1.agda.template");
const GENERATED_ACCEPTANCE_ENTRY_RELATIVE_PATH_V1: &str =
    "LawV2/Wire/GeneratedBundleAcceptanceV1.agda";

macro_rules! wire_source {
    ($path:literal, $module:literal) => {
        FixedAgdaSourceV1 {
            relative_path: concat!("LawV2/Wire/", $path),
            module_name: concat!("LawV2.Wire.", $module),
            bytes: include_bytes!(concat!("../agda/LawV2/Wire/", $path)),
        }
    };
}

macro_rules! lambda_unit_source {
    ($path:literal, $module:literal) => {
        FixedAgdaSourceV1 {
            relative_path: concat!("LawV2/LambdaUnit/", $path),
            module_name: concat!("LawV2.LambdaUnit.", $module),
            bytes: include_bytes!(concat!("../agda/LawV2/LambdaUnit/", $path)),
        }
    };
}

/// The compile-time-fixed safe-Agda sources of the acceptance package,
/// in the exact positions the pinned checker reports them (the entry
/// and the two generated input modules are interleaved by the package
/// assembler). Interfaces are disabled, so every transitive module of
/// the acceptance entry appears exactly once.
const WIRE_ACCEPTANCE_STATIC_SOURCES_V1: &[FixedAgdaSourceV1] = &[
    wire_source!("Bytes.agda", "Bytes"),
    wire_source!("ProductionBundleV1.agda", "ProductionBundleV1"),
    wire_source!("Decoder.agda", "Decoder"),
    wire_source!("Envelope.agda", "Envelope"),
    wire_source!("BundleChecker.agda", "BundleChecker"),
    wire_source!("ContextChecker.agda", "ContextChecker"),
    wire_source!("SynthesisReplayV1.agda", "SynthesisReplayV1"),
    lambda_unit_source!("ProductionSyntaxV1.agda", "ProductionSyntaxV1"),
    lambda_unit_source!("TypingSyntax.agda", "TypingSyntax"),
    lambda_unit_source!("Substitution.agda", "Substitution"),
    lambda_unit_source!("SubstitutionReduction.agda", "SubstitutionReduction"),
    lambda_unit_source!("ProductionDecodingV1.agda", "ProductionDecodingV1"),
    wire_source!("ContextCorrespondenceV1.agda", "ContextCorrespondenceV1"),
    wire_source!("SemanticReplayV1.agda", "SemanticReplayV1"),
    wire_source!("NormalizationV1.agda", "NormalizationV1"),
    wire_source!("SupplementReplayV1.agda", "SupplementReplayV1"),
    wire_source!("TypingReplayV1.agda", "TypingReplayV1"),
    wire_source!("InventoryReplayV1.agda", "InventoryReplayV1"),
    lambda_unit_source!("FamilyNaturality.agda", "FamilyNaturality"),
    lambda_unit_source!(
        "ProductionInventoryBridgeV1.agda",
        "ProductionInventoryBridgeV1"
    ),
    wire_source!("TranscriptRenderV1.agda", "TranscriptRenderV1"),
];

/// Failures of the Phase G capability constructors. Every variant is
/// fail-closed; none can be bypassed by a caller-supplied digest,
/// Boolean, or section mask.
#[derive(Clone, Debug)]
pub enum ProductionBridgeFailureV1 {
    Replay(ProductionReplayFailureV1),
    InputArtifact(ProductionBundleInputFailureV1),
    AgdaGate(AgdaReferenceFailureV1),
    BundleBytesMismatch,
    TranscriptBytesMismatch,
    ManifestBindingMismatch,
}

impl std::fmt::Display for ProductionBridgeFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Replay(error) => write!(formatter, "independent Rust replay failed: {error}"),
            Self::InputArtifact(error) => {
                write!(formatter, "generated input artifact failed: {error}")
            }
            Self::AgdaGate(error) => write!(formatter, "safe-Agda acceptance gate failed: {error}"),
            Self::BundleBytesMismatch => formatter.write_str(
                "the capabilities do not carry the exact same canonical bundle bytes",
            ),
            Self::TranscriptBytesMismatch => formatter.write_str(
                "the Agda and Rust canonical transcripts are not byte-identical",
            ),
            Self::ManifestBindingMismatch => {
                formatter.write_str("the capabilities are not bound to one semantic manifest")
            }
        }
    }
}

impl std::error::Error for ProductionBridgeFailureV1 {}

impl From<ProductionReplayFailureV1> for ProductionBridgeFailureV1 {
    fn from(error: ProductionReplayFailureV1) -> Self {
        Self::Replay(error)
    }
}

impl From<ProductionBundleInputFailureV1> for ProductionBridgeFailureV1 {
    fn from(error: ProductionBundleInputFailureV1) -> Self {
        Self::InputArtifact(error)
    }
}

impl From<AgdaReferenceFailureV1> for ProductionBridgeFailureV1 {
    fn from(error: AgdaReferenceFailureV1) -> Self {
        Self::AgdaGate(error)
    }
}

/// Independent Rust replay of one verified canonical bundle through the
/// unchanged kernel and synthesis checker, minting the replay
/// capability with the rendered canonical transcript.
pub fn verify_rust_production_replay_v1(
    bundle: &VerifiedCanonicalProductionBundleV1,
) -> Result<VerifiedRustProductionReplayV1, ProductionBridgeFailureV1> {
    let evidence = replay_production_bundle_v1(bundle.canonical_bytes())?;
    let decoded = decode_bundle_v1(bundle.canonical_bytes())
        .map_err(|_| ProductionBridgeFailureV1::Replay(ProductionReplayFailureV1::Decode))?;
    let transcript = render_production_transcript_v1(&decoded, &evidence.computed);
    let mut verified = VerifiedRustProductionReplayV1 {
        semantic_manifest_digest: bundle.semantic_manifest_digest().clone(),
        bundle_digest: bundle.bundle_digest().clone(),
        replayed_bundle_bytes: Arc::<[u8]>::from(evidence.replayed_bytes.into_boxed_slice()),
        kernel_protocol_digest: evidence.kernel_protocol_digest,
        synthesis_protocol_digest: evidence.synthesis_protocol_digest,
        rust_transcript: Arc::<[u8]>::from(transcript.into_boxed_slice()),
        digest: Digest::of_bytes(b"pending verified rust production replay"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-rust-production-replay/v1",
        &verified,
    );
    Ok(verified)
}

/// Safe-Agda acceptance of one verified canonical bundle: the exact
/// bundle bytes and the Rust-rendered transcript are embedded in the
/// two generated input modules, the fixed acceptance entry is checked
/// by the pinned Agda over the exact wire source tree, and the
/// resulting run evidence (source-tree and checker-transcript digests)
/// is bound into the capability. The accepted-section mask is emitted
/// by this bridge, never caller-supplied: the acceptance entry forces
/// all four semantic sections, so a successful run accepts all four.
pub fn verify_agda_production_acceptance_v1(
    bundle: &VerifiedCanonicalProductionBundleV1,
    replay: &VerifiedRustProductionReplayV1,
) -> Result<VerifiedAgdaProductionAcceptanceV1, ProductionBridgeFailureV1> {
    if bundle.semantic_manifest_digest() != replay.semantic_manifest_digest()
        || bundle.bundle_digest() != replay.bundle_digest()
    {
        return Err(ProductionBridgeFailureV1::ManifestBindingMismatch);
    }
    if replay.replayed_bundle_bytes() != bundle.canonical_bytes() {
        return Err(ProductionBridgeFailureV1::BundleBytesMismatch);
    }
    let input_source = render_production_bundle_input_module_v1(bundle.canonical_bytes())?;
    let input_artifact =
        verify_agda_production_input_artifact_v1(bundle.canonical_bytes(), input_source.as_bytes())?;
    let transcript_source =
        render_production_transcript_input_module_v1(replay.rust_transcript())?;

    let mut sources = Vec::with_capacity(WIRE_ACCEPTANCE_STATIC_SOURCES_V1.len() + 3);
    sources.push(PackageSourceV1::Generated(GeneratedAgdaSourceV1 {
        relative_path: GENERATED_ACCEPTANCE_ENTRY_RELATIVE_PATH_V1.to_owned(),
        module_name: "LawV2.Wire.GeneratedBundleAcceptanceV1".to_owned(),
        bytes: GENERATED_ACCEPTANCE_TEMPLATE_V1.as_bytes().to_vec(),
    }));
    for source in WIRE_ACCEPTANCE_STATIC_SOURCES_V1 {
        sources.push(PackageSourceV1::Fixed(source.clone()));
    }
    sources.push(PackageSourceV1::Generated(GeneratedAgdaSourceV1 {
        relative_path: "LawV2/Wire/GeneratedProductionBundleInputV1.agda".to_owned(),
        module_name: crate::production_wire_input::PRODUCTION_BUNDLE_INPUT_MODULE_NAME_V1
            .to_owned(),
        bytes: input_source.into_bytes(),
    }));
    sources.push(PackageSourceV1::Generated(GeneratedAgdaSourceV1 {
        relative_path: "LawV2/Wire/GeneratedProductionTranscriptInputV1.agda".to_owned(),
        module_name: crate::production_wire_input::PRODUCTION_TRANSCRIPT_INPUT_MODULE_NAME_V1
            .to_owned(),
        bytes: transcript_source.into_bytes(),
    }));
    let package = diagnose_generated_agda_package_v1(
        &sources,
        GENERATED_ACCEPTANCE_ENTRY_RELATIVE_PATH_V1,
    )?;

    let mask_sections =
        Arc::<[AcceptedProductionSectionV1]>::from(REQUIRED_ACCEPTED_PRODUCTION_SECTIONS_V1);
    let mut accepted_sections = AcceptedProductionSectionMaskV1 {
        sections: mask_sections,
        digest: Digest::of_bytes(b"pending accepted production section mask"),
    };
    accepted_sections.digest = Digest::of_canonical(
        "pen-semantic-audit/accepted-production-section-mask/v1",
        &accepted_sections,
    );
    let mut verified = VerifiedAgdaProductionAcceptanceV1 {
        semantic_manifest_digest: bundle.semantic_manifest_digest().clone(),
        bundle_digest: bundle.bundle_digest().clone(),
        input_artifact,
        safe_source_tree_digest: package.source_tree_digest().clone(),
        checker_transcript_digest: package.checker_stdout_digest().clone(),
        accepted_sections,
        agda_transcript: Arc::<[u8]>::from(replay.rust_transcript().to_vec().into_boxed_slice()),
        digest: Digest::of_bytes(b"pending verified agda production acceptance"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-agda-production-acceptance/v1",
        &verified,
    );
    Ok(verified)
}

/// Exact common-input and common-transcript agreement: all three
/// capabilities must carry the same canonical bundle bytes, and the
/// Agda-verified transcript must equal the Rust-rendered transcript
/// byte-for-byte. Digests are recorded only after those comparisons.
pub fn verify_production_transcript_agreement_v1(
    bundle: &VerifiedCanonicalProductionBundleV1,
    acceptance: &VerifiedAgdaProductionAcceptanceV1,
    replay: &VerifiedRustProductionReplayV1,
) -> Result<VerifiedProductionTranscriptAgreementV1, ProductionBridgeFailureV1> {
    if bundle.semantic_manifest_digest() != acceptance.semantic_manifest_digest()
        || bundle.semantic_manifest_digest() != replay.semantic_manifest_digest()
        || bundle.bundle_digest() != acceptance.bundle_digest()
        || bundle.bundle_digest() != replay.bundle_digest()
    {
        return Err(ProductionBridgeFailureV1::ManifestBindingMismatch);
    }
    if acceptance.accepted_bundle_bytes() != bundle.canonical_bytes()
        || replay.replayed_bundle_bytes() != bundle.canonical_bytes()
    {
        return Err(ProductionBridgeFailureV1::BundleBytesMismatch);
    }
    if acceptance.agda_transcript() != replay.rust_transcript() {
        return Err(ProductionBridgeFailureV1::TranscriptBytesMismatch);
    }
    let transcript_bytes =
        Arc::<[u8]>::from(replay.rust_transcript().to_vec().into_boxed_slice());
    let transcript_digest = Digest::of_domain_bytes(
        "law-v2-production-refinement-canonical-transcript/v1",
        &transcript_bytes,
    );
    let mut verified = VerifiedProductionTranscriptAgreementV1 {
        semantic_manifest_digest: bundle.semantic_manifest_digest().clone(),
        bundle_digest: bundle.bundle_digest().clone(),
        canonical_bundle_bytes: Arc::<[u8]>::from(
            bundle.canonical_bytes().to_vec().into_boxed_slice(),
        ),
        canonical_bundle_capability_digest: bundle.digest().clone(),
        agda_acceptance_digest: acceptance.digest().clone(),
        rust_replay_digest: replay.digest().clone(),
        transcript_bytes,
        transcript_digest,
        digest: Digest::of_bytes(b"pending verified production transcript agreement"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-production-transcript-agreement/v1",
        &verified,
    );
    Ok(verified)
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
    CanonicalProductionAuthorityObligationV1::SafeAgdaGenericBundleAcceptance,
    CanonicalProductionAuthorityObligationV1::IndependentRustKernelReplay,
    CanonicalProductionAuthorityObligationV1::ExactNormalizedTranscriptByteAgreement,
];

pub const CANONICAL_PRODUCTION_AUTHORITY_FRONTIER_V1:
    &[CanonicalProductionAuthorityObligationV1] = &[
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
