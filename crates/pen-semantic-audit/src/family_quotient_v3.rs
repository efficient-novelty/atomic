//! Authorized-Q0 native-carrier twin and complete family quotient (Phase J,
//! family-quotient stage).
//!
//! The pre-Q0 native carrier is deliberately minted before rewrite authority.
//! This module is the only downstream path that turns it into the fixed
//! Q0-normalized vertex set used by the semantic-family quotient:
//!
//! 1. recover the exact verifier-minted inventory seeds from the native
//!    carrier (there is no caller seed list);
//! 2. compute each seed's Q0 normal form under the exact fresh program bound
//!    by the typed rewrite inventory;
//! 3. re-run the registered carrier engine in `AuthorizedQ0` mode;
//! 4. compare the normal form of every pre-Q0 family with the independent V3
//!    rewrite graph's unique normal form and prove an exact multiset image;
//! 5. exhaust Q1/Q2 on that fixed vertex set with the verified-empty Q3
//!    registry; and
//! 6. bind the carrier, rewrite theorem, program, and quotient into one opaque
//!    deterministic capability.
//!
//! This is the semantic-family quotient used by the value audit. It is not
//! the later complete quotient of candidate/discharger presentations used to
//! form a branch cone.

use crate::carrier::{
    CarrierCertificateV1, enumerate_raw_families_with_q0_v1, substitution_action_digest,
};
use crate::inventory::VerifiedPublicAuditInventoryV1;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, VerifiedSemanticAuditManifestV1,
    VerifiedSemanticAuditManifestV2, VerifiedSemanticAuditManifestV3,
    proposed_semantic_audit_lambda_unit_manifest_v1,
    proposed_semantic_audit_lambda_unit_manifest_v2,
    proposed_semantic_audit_lambda_unit_manifest_v3,
};
use crate::model::{
    FamilyConstructorV1, GenericJudgmentV1, RawFamilyIdV1, SeedIdV1, SemanticSchemaSeedV1,
    semantic_seed_id,
};
use crate::native_carrier_v3::VerifiedNativeRankInductiveCarrierV3;
use crate::normalizer::{VerifiedFreshConstructorComputationV1, normalize_generated_judgment_v1};
use crate::quotient::{QuotientCertificateV1, quotient_families_with_q0_v1};
use crate::rewrite_authority_v3::{RewriteNodeJudgmentV3, VerifiedRewriteAuthorityV3};
use crate::rewrite_inventory::VerifiedTypedRewriteInventoryV1;
use crate::semantic_authority_v3::SeedIdV3;
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, Kernel, KernelError, OpenJudgment,
    ResourceKind, Term,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub const FAMILY_QUOTIENT_SCHEMA_VERSION_V3: u16 = 1;

/// Detailed fail-closed diagnostics for the V3 family-quotient constructor.
#[derive(Clone, Debug)]
pub enum FamilyQuotientFailureV3 {
    ExactManifestIdentityMismatch,
    ChainBindingMismatch,
    MissingFreshProgram,
    UnexpectedFreshProgram,
    FreshProgramBindingMismatch,
    SeedNormalization(AuditUnknownReason),
    CarrierEnumeration(AuditUnknownReason),
    OutsideFragment(OutsideFragmentReason),
    RewriteNormalFormMismatch,
    CarrierImageMismatch,
    V3IdentityMismatch,
    Quotient(AuditUnknownReason),
    QuotientCoverageMismatch,
    EmptyQuotient,
}

impl std::fmt::Display for FamilyQuotientFailureV3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactManifestIdentityMismatch => formatter.write_str(
                "the supplied manifests are not the exact registered lambda/unit proposals",
            ),
            Self::ChainBindingMismatch => formatter.write_str(
                "the inventory, native carrier, and rewrite theorem are not one verified chain",
            ),
            Self::MissingFreshProgram => formatter.write_str(
                "an equation-bearing inventory requires its exact typed inventory and fresh program",
            ),
            Self::UnexpectedFreshProgram => formatter.write_str(
                "an equationless inventory forbids a typed inventory or fresh program",
            ),
            Self::FreshProgramBindingMismatch => formatter.write_str(
                "the typed rewrite inventory, rewrite theorem, and fresh program do not agree exactly",
            ),
            Self::SeedNormalization(reason) => {
                write!(formatter, "native seed normalization failed: {reason:?}")
            }
            Self::CarrierEnumeration(reason) => {
                write!(formatter, "authorized-Q0 carrier enumeration failed: {reason:?}")
            }
            Self::OutsideFragment(reason) => {
                write!(formatter, "outside the registered lambda/unit fragment: {reason:?}")
            }
            Self::RewriteNormalFormMismatch => formatter.write_str(
                "the restricted Q0 normalizer and the V3 rewrite theorem disagree on a native family",
            ),
            Self::CarrierImageMismatch => formatter.write_str(
                "the authorized-Q0 carrier is not the exact multiset image of the native carrier",
            ),
            Self::V3IdentityMismatch => formatter.write_str(
                "the internal V1 proof carrier could not be re-identified on the demand-neutral V3 family surface",
            ),
            Self::Quotient(reason) => {
                write!(formatter, "the complete family quotient failed: {reason:?}")
            }
            Self::QuotientCoverageMismatch => formatter.write_str(
                "the quotient does not cover every fixed normalized vertex exactly once",
            ),
            Self::EmptyQuotient => {
                formatter.write_str("an empty family quotient cannot carry semantic authority")
            }
        }
    }
}

impl std::error::Error for FamilyQuotientFailureV3 {}

impl FamilyQuotientFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::OutsideFragment(reason) => AuditDecision::OutsideFragment(reason),
            Self::SeedNormalization(AuditUnknownReason::ResourceExhausted)
            | Self::CarrierEnumeration(AuditUnknownReason::ResourceExhausted)
            | Self::Quotient(AuditUnknownReason::ResourceExhausted) => {
                AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted)
            }
            _ => AuditDecision::Unknown(AuditUnknownReason::MissingFamilyQuotientV3),
        }
    }
}

/// One exact correspondence between a pre-Q0 seed and the same internally
/// derived seed carrying its authorized normal-form claim.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Q0SeedImageV3 {
    source: SeedIdV1,
    normalized: SeedIdV1,
    semantic: SeedIdV3,
    normalized_judgment_digest: Digest,
}

impl Q0SeedImageV3 {
    pub fn source(&self) -> &SeedIdV1 {
        &self.source
    }

    pub fn normalized(&self) -> &SeedIdV1 {
        &self.normalized
    }

    /// Demand-neutral semantic identity inherited from the checked native
    /// carrier correspondence. Unlike the legacy proof IDs, this identity
    /// excludes V1 clause and equation-port metadata.
    pub fn semantic(&self) -> &SeedIdV3 {
        &self.semantic
    }
}

impl CanonicalEncode for Q0SeedImageV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source.encode_canonical(encoder);
        self.normalized.encode_canonical(encoder);
        self.semantic.encode_canonical(encoder);
        self.normalized_judgment_digest.encode_canonical(encoder);
    }
}

/// One deterministic pairing in the exact multiset image from native
/// pre-Q0 families to authorized-Q0 families. Constructor references may
/// receive new identifiers after normalization, so the pairing is by the
/// complete semantic-family payload rather than by raw tree identifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Q0FamilyImageV3 {
    source: RawFamilyIdV1,
    legacy_normalized: RawFamilyIdV1,
    normalized: NormalizedFamilyIdV3,
    normalized_judgment_digest: Digest,
}

impl Q0FamilyImageV3 {
    pub fn source(&self) -> &RawFamilyIdV1 {
        &self.source
    }

    pub fn normalized(&self) -> &NormalizedFamilyIdV3 {
        &self.normalized
    }
}

impl CanonicalEncode for Q0FamilyImageV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source.encode_canonical(encoder);
        self.legacy_normalized.encode_canonical(encoder);
        self.normalized.encode_canonical(encoder);
        self.normalized_judgment_digest.encode_canonical(encoder);
    }
}

/// Demand-neutral V3 identity for one authorized-Q0 raw derivation. The
/// legacy V1 raw identifier is retained only in the private proof crosswalk;
/// it is never promoted as the V3 family identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NormalizedFamilyIdV3(Digest);

impl NormalizedFamilyIdV3 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for NormalizedFamilyIdV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// Demand-neutral V3 semantic-family class identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FamilyClassIdV3(Digest);

impl FamilyClassIdV3 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for FamilyClassIdV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// One quotient class on the V3 identity surface. Its representative and
/// members are V3 derivation identifiers; V1 identifiers appear only in the
/// private checked correspondence to the legacy quotient core.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedFamilyClassV3 {
    id: FamilyClassIdV3,
    representative: NormalizedFamilyIdV3,
    members: Arc<[NormalizedFamilyIdV3]>,
    generic_judgment: GenericJudgmentV1,
    role: crate::model::LocalRoleV1,
    canonical_support: crate::model::PublicSupportV1,
    source_clause: Option<crate::model::ClauseIdV1>,
    substitution_action_digest: Digest,
}

impl VerifiedFamilyClassV3 {
    pub fn id(&self) -> &FamilyClassIdV3 {
        &self.id
    }

    pub fn representative(&self) -> &NormalizedFamilyIdV3 {
        &self.representative
    }

    pub fn members(&self) -> &[NormalizedFamilyIdV3] {
        &self.members
    }

    pub fn generic_judgment(&self) -> &GenericJudgmentV1 {
        &self.generic_judgment
    }

    pub fn role(&self) -> crate::model::LocalRoleV1 {
        self.role
    }

    pub fn canonical_support(&self) -> &crate::model::PublicSupportV1 {
        &self.canonical_support
    }

    pub fn source_clause(&self) -> Option<&crate::model::ClauseIdV1> {
        self.source_clause.as_ref()
    }

    pub fn substitution_action_digest(&self) -> &Digest {
        &self.substitution_action_digest
    }
}

impl CanonicalEncode for VerifiedFamilyClassV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.representative.encode_canonical(encoder);
        encoder.sequence(&self.members);
        self.generic_judgment.encode_canonical(encoder);
        self.role.encode_canonical(encoder);
        self.canonical_support.encode_canonical(encoder);
        encoder.option(&self.source_clause);
        self.substitution_action_digest.encode_canonical(encoder);
    }
}

/// Opaque V3 family-quotient authority. There is no deserialization path and
/// no constructor from caller-provided seeds, vertices, classes, or digests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedFamilyQuotientV3 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    v1_manifest_digest: Digest,
    v2_manifest_digest: Digest,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    normalizer_protocol_digest: Digest,
    inventory_digest: Digest,
    native_carrier_digest: Digest,
    rewrite_authority_digest: Digest,
    typed_rewrite_inventory_digest: Option<Digest>,
    fresh_program_digest: Option<Digest>,
    authorized_carrier_digest: Digest,
    seed_images: Arc<[Q0SeedImageV3]>,
    family_images: Arc<[Q0FamilyImageV3]>,
    authorized_carrier: CarrierCertificateV1,
    legacy_quotient_digest: Digest,
    quotient_digest: Digest,
    quotient: QuotientCertificateV1,
    classes: Arc<[VerifiedFamilyClassV3]>,
    digest: Digest,
}

impl VerifiedFamilyQuotientV3 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.normalizer_protocol_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn native_carrier_digest(&self) -> &Digest {
        &self.native_carrier_digest
    }

    pub fn rewrite_authority_digest(&self) -> &Digest {
        &self.rewrite_authority_digest
    }

    pub fn fresh_program_digest(&self) -> Option<&Digest> {
        self.fresh_program_digest.as_ref()
    }

    pub fn typed_rewrite_inventory_digest(&self) -> Option<&Digest> {
        self.typed_rewrite_inventory_digest.as_ref()
    }

    pub fn authorized_carrier_digest(&self) -> &Digest {
        &self.authorized_carrier_digest
    }

    pub fn seed_images(&self) -> &[Q0SeedImageV3] {
        &self.seed_images
    }

    pub fn family_images(&self) -> &[Q0FamilyImageV3] {
        &self.family_images
    }

    pub fn normalized_family_count(&self) -> usize {
        self.authorized_carrier.raw_families().len()
    }

    pub fn classes(&self) -> &[VerifiedFamilyClassV3] {
        &self.classes
    }

    pub fn quotient_digest(&self) -> &Digest {
        &self.quotient_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub(crate) fn v1_manifest_digest(&self) -> &Digest {
        &self.v1_manifest_digest
    }

    pub(crate) fn v2_manifest_digest(&self) -> &Digest {
        &self.v2_manifest_digest
    }

    pub(crate) fn authorized_carrier_proof(&self) -> &CarrierCertificateV1 {
        &self.authorized_carrier
    }

    pub(crate) fn legacy_quotient_proof(&self) -> &QuotientCertificateV1 {
        &self.quotient
    }
}

impl CanonicalEncode for VerifiedFamilyQuotientV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.v1_manifest_digest.encode_canonical(encoder);
        self.v2_manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.inventory_digest.encode_canonical(encoder);
        self.native_carrier_digest.encode_canonical(encoder);
        self.rewrite_authority_digest.encode_canonical(encoder);
        encoder.option(&self.typed_rewrite_inventory_digest);
        encoder.option(&self.fresh_program_digest);
        self.authorized_carrier_digest.encode_canonical(encoder);
        encoder.sequence(&self.seed_images);
        encoder.sequence(&self.family_images);
        self.authorized_carrier.encode_canonical(encoder);
        self.legacy_quotient_digest.encode_canonical(encoder);
        self.quotient_digest.encode_canonical(encoder);
        self.quotient.encode_canonical(encoder);
        encoder.sequence(&self.classes);
    }
}

/// Build the authorized-Q0 twin and complete Q1/Q2/Q3 family quotient over
/// one exact native/rewrite chain.
#[allow(clippy::too_many_arguments)]
pub fn diagnose_family_quotient_v3(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    native_carrier: &VerifiedNativeRankInductiveCarrierV3,
    rewrite_authority: &VerifiedRewriteAuthorityV3,
    typed_rewrite_inventory: Option<&VerifiedTypedRewriteInventoryV1>,
    fresh_program: Option<&VerifiedFreshConstructorComputationV1>,
) -> Result<VerifiedFamilyQuotientV3, FamilyQuotientFailureV3> {
    verify_exact_manifests(v1_manifest, v2_manifest, v3_manifest)?;

    let signature = inventory.successor_boundary();
    if inventory.manifest_digest() != v1_manifest.candidate_digest()
        || native_carrier.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || native_carrier.signature_digest() != signature.digest()
        || native_carrier.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
        || native_carrier.inventory_digest() != inventory.digest()
        || rewrite_authority.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || rewrite_authority.signature_digest() != signature.digest()
        || rewrite_authority.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
        || rewrite_authority.carrier_digest() != native_carrier.digest()
    {
        return Err(FamilyQuotientFailureV3::ChainBindingMismatch);
    }

    verify_fresh_bindings(
        v1_manifest,
        kernel,
        inventory,
        rewrite_authority,
        typed_rewrite_inventory,
        fresh_program,
    )?;

    let pre_q0 = native_carrier.pre_q0_carrier();
    if pre_q0.manifest_digest() != v1_manifest.candidate_digest()
        || pre_q0.signature_digest() != signature.digest()
        || pre_q0.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
        || !pre_q0.q3_registry_verified_empty()
    {
        return Err(FamilyQuotientFailureV3::ChainBindingMismatch);
    }

    let mut normalized_seed_wires = Vec::with_capacity(pre_q0.verified_seeds().len());
    let mut seed_images = Vec::with_capacity(pre_q0.verified_seeds().len());
    for seed in pre_q0.verified_seeds() {
        let normalized = normalize_judgment(
            v1_manifest,
            kernel,
            signature,
            seed.source_judgment(),
            fresh_program,
        )?;
        verify_judgment_normal_form(
            kernel,
            signature,
            rewrite_authority,
            seed.source_judgment(),
            &normalized,
        )?;
        let normalized_wire = with_normalized_claim(seed.seed().clone(), normalized.clone());
        let normalized_id = semantic_seed_id(&normalized_wire);
        let semantic = native_carrier
            .v3_seed_id(seed.id())
            .cloned()
            .ok_or(FamilyQuotientFailureV3::V3IdentityMismatch)?;
        seed_images.push(Q0SeedImageV3 {
            source: seed.id().clone(),
            normalized: normalized_id,
            semantic,
            normalized_judgment_digest: Digest::of_canonical(
                "pen-semantic-audit/q0-seed-normalized-judgment/v3",
                &normalized,
            ),
        });
        normalized_seed_wires.push(normalized_wire);
    }
    if normalized_seed_wires.is_empty() {
        return Err(FamilyQuotientFailureV3::EmptyQuotient);
    }

    let authorized_carrier = match enumerate_raw_families_with_q0_v1(
        kernel,
        signature,
        v1_manifest,
        &normalized_seed_wires,
        &[],
        fresh_program,
    ) {
        AuditDecision::Proven(carrier) => carrier,
        AuditDecision::Unknown(reason) => {
            return Err(FamilyQuotientFailureV3::CarrierEnumeration(reason));
        }
        AuditDecision::OutsideFragment(reason) => {
            return Err(FamilyQuotientFailureV3::OutsideFragment(reason));
        }
    };

    let expected_seed_ids = seed_images
        .iter()
        .map(|image| image.normalized.clone())
        .collect::<BTreeSet<_>>();
    let actual_seed_ids = authorized_carrier
        .verified_seeds()
        .iter()
        .map(|seed| seed.id.clone())
        .collect::<BTreeSet<_>>();
    if expected_seed_ids.len() != seed_images.len()
        || actual_seed_ids != expected_seed_ids
        || authorized_carrier.fresh_program_digest()
            != fresh_program.map(VerifiedFreshConstructorComputationV1::program_digest)
    {
        return Err(FamilyQuotientFailureV3::CarrierImageMismatch);
    }

    let v3_family_ids = derive_v3_family_ids(v3_manifest, &authorized_carrier, &seed_images)?;
    let family_images = verify_exact_family_image(
        v1_manifest,
        kernel,
        signature,
        pre_q0,
        &authorized_carrier,
        rewrite_authority,
        fresh_program,
        &seed_images,
        &v3_family_ids,
    )?;

    let quotient = match quotient_families_with_q0_v1(
        kernel,
        signature,
        v1_manifest,
        &authorized_carrier,
        fresh_program,
    ) {
        AuditDecision::Proven(quotient) => quotient,
        AuditDecision::Unknown(reason) => {
            return Err(FamilyQuotientFailureV3::Quotient(reason));
        }
        AuditDecision::OutsideFragment(reason) => {
            return Err(FamilyQuotientFailureV3::OutsideFragment(reason));
        }
    };
    verify_quotient_coverage(&authorized_carrier, &quotient)?;
    if quotient.classes().is_empty() {
        return Err(FamilyQuotientFailureV3::EmptyQuotient);
    }
    let classes = derive_v3_classes(v3_manifest, &quotient, &v3_family_ids)?;

    let authorized_carrier_digest = Digest::of_canonical(
        "pen-semantic-audit/authorized-q0-native-carrier/v3",
        &authorized_carrier,
    );
    let legacy_quotient_digest = Digest::of_canonical(
        "pen-semantic-audit/legacy-family-quotient-proof-core/v1",
        &quotient,
    );
    let quotient_digest = Digest::of_canonical(
        "pen-semantic-audit/complete-family-quotient/v3",
        &V3QuotientDigestMaterial {
            semantic_manifest_digest: v3_manifest.candidate_digest(),
            authorized_carrier_digest: &authorized_carrier_digest,
            legacy_proof_digest: &legacy_quotient_digest,
            classes: &classes,
        },
    );
    let mut verified = VerifiedFamilyQuotientV3 {
        schema_version: FAMILY_QUOTIENT_SCHEMA_VERSION_V3,
        semantic_manifest_digest: v3_manifest.candidate_digest().clone(),
        v1_manifest_digest: v1_manifest.candidate_digest().clone(),
        v2_manifest_digest: v2_manifest.candidate_digest().clone(),
        signature_digest: signature.digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        inventory_digest: inventory.digest().clone(),
        native_carrier_digest: native_carrier.digest().clone(),
        rewrite_authority_digest: rewrite_authority.digest().clone(),
        typed_rewrite_inventory_digest: typed_rewrite_inventory.map(|typed| typed.digest().clone()),
        fresh_program_digest: fresh_program.map(|fresh| fresh.program_digest().clone()),
        authorized_carrier_digest,
        seed_images: Arc::from(seed_images.into_boxed_slice()),
        family_images: Arc::from(family_images.into_boxed_slice()),
        authorized_carrier,
        legacy_quotient_digest,
        quotient_digest,
        quotient,
        classes: Arc::from(classes.into_boxed_slice()),
        digest: Digest::of_bytes(b"pending verified family quotient v3"),
    };
    verified.digest =
        Digest::of_canonical("pen-semantic-audit/verified-family-quotient/v3", &verified);
    Ok(verified)
}

#[allow(clippy::too_many_arguments)]
pub fn verify_family_quotient_v3(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    native_carrier: &VerifiedNativeRankInductiveCarrierV3,
    rewrite_authority: &VerifiedRewriteAuthorityV3,
    typed_rewrite_inventory: Option<&VerifiedTypedRewriteInventoryV1>,
    fresh_program: Option<&VerifiedFreshConstructorComputationV1>,
) -> AuditDecision<VerifiedFamilyQuotientV3> {
    match diagnose_family_quotient_v3(
        v1_manifest,
        v2_manifest,
        v3_manifest,
        kernel,
        inventory,
        native_carrier,
        rewrite_authority,
        typed_rewrite_inventory,
        fresh_program,
    ) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(failure) => failure.into_decision(),
    }
}

fn verify_exact_manifests(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
) -> Result<(), FamilyQuotientFailureV3> {
    if v1_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v1()
        || v2_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v2()
        || v3_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v3()
    {
        return Err(FamilyQuotientFailureV3::ExactManifestIdentityMismatch);
    }
    Ok(())
}

fn verify_fresh_bindings(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    rewrite_authority: &VerifiedRewriteAuthorityV3,
    typed: Option<&VerifiedTypedRewriteInventoryV1>,
    fresh: Option<&VerifiedFreshConstructorComputationV1>,
) -> Result<(), FamilyQuotientFailureV3> {
    if inventory.equations().is_empty() {
        if typed.is_some()
            || fresh.is_some()
            || rewrite_authority.typed_rewrite_inventory_digest().is_some()
        {
            return Err(FamilyQuotientFailureV3::UnexpectedFreshProgram);
        }
        return Ok(());
    }

    let (Some(typed), Some(fresh)) = (typed, fresh) else {
        return Err(FamilyQuotientFailureV3::MissingFreshProgram);
    };
    if rewrite_authority.typed_rewrite_inventory_digest() != Some(typed.digest())
        || typed.inventory_digest() != inventory.digest()
        || typed.manifest_digest() != v1_manifest.candidate_digest()
        || typed.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
        || typed.successor_boundary_digest() != inventory.successor_boundary().digest()
        || typed.normalizer_protocol_digest() != &kernel.normalizer_protocol_digest()
        || typed.fresh_program_digest() != fresh.program_digest()
        || fresh.manifest_digest() != v1_manifest.candidate_digest()
        || fresh.boundary_signature_digest() != inventory.predecessor_boundary().digest()
        || fresh.extended_signature().digest() != inventory.successor_boundary().digest()
        || fresh
            .termination_certificate()
            .kernel_normalizer_protocol_digest()
            != &kernel.normalizer_protocol_digest()
        || fresh
            .confluence_certificate()
            .kernel_normalizer_protocol_digest()
            != &kernel.normalizer_protocol_digest()
    {
        return Err(FamilyQuotientFailureV3::FreshProgramBindingMismatch);
    }
    Ok(())
}

fn with_normalized_claim(
    mut seed: SemanticSchemaSeedV1,
    normalized: GenericJudgmentV1,
) -> SemanticSchemaSeedV1 {
    match &mut seed {
        SemanticSchemaSeedV1::PublicHead(seed) => seed.judgment.claimed_normalized = normalized,
        SemanticSchemaSeedV1::PublicEquation(seed) => seed.judgment.claimed_normalized = normalized,
        SemanticSchemaSeedV1::PublicUniversalInterface { judgment, .. } => {
            judgment.claimed_normalized = normalized
        }
    }
    seed
}

fn normalize_judgment(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    signature: &pen_kernel::VerifiedSignature,
    judgment: &GenericJudgmentV1,
    fresh: Option<&VerifiedFreshConstructorComputationV1>,
) -> Result<GenericJudgmentV1, FamilyQuotientFailureV3> {
    match normalize_generated_judgment_v1(manifest, kernel, signature, judgment, fresh) {
        AuditDecision::Proven(normalized) => Ok(normalized),
        AuditDecision::Unknown(reason) => Err(FamilyQuotientFailureV3::SeedNormalization(reason)),
        AuditDecision::OutsideFragment(reason) => {
            Err(FamilyQuotientFailureV3::OutsideFragment(reason))
        }
    }
}

#[derive(Clone)]
enum FamilyConstructorIdentityV3 {
    PublicHeadSeed {
        seed: SeedIdV3,
    },
    PublicEquationSeed {
        seed: SeedIdV3,
    },
    GenericPublicApplication {
        function: NormalizedFamilyIdV3,
        argument: NormalizedFamilyIdV3,
        context_witness: Digest,
    },
    GenericEquationAction {
        equation: NormalizedFamilyIdV3,
        context: NormalizedFamilyIdV3,
        hole_ordinal: u32,
        context_witness: Digest,
    },
}

impl CanonicalEncode for FamilyConstructorIdentityV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::PublicHeadSeed { seed } => {
                encoder.tag(0);
                seed.encode_canonical(encoder);
            }
            Self::PublicEquationSeed { seed } => {
                encoder.tag(1);
                seed.encode_canonical(encoder);
            }
            Self::GenericPublicApplication {
                function,
                argument,
                context_witness,
            } => {
                encoder.tag(2);
                function.encode_canonical(encoder);
                argument.encode_canonical(encoder);
                context_witness.encode_canonical(encoder);
            }
            Self::GenericEquationAction {
                equation,
                context,
                hole_ordinal,
                context_witness,
            } => {
                encoder.tag(3);
                equation.encode_canonical(encoder);
                context.encode_canonical(encoder);
                encoder.u32(*hole_ordinal);
                context_witness.encode_canonical(encoder);
            }
        }
    }
}

struct ContextWitnessIdentityMaterialV3<'a> {
    manifest_digest: &'a Digest,
    witness: &'a crate::carrier::ContextAmalgamationWitnessV1,
}

impl CanonicalEncode for ContextWitnessIdentityMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        self.witness.kind.encode_canonical(encoder);
        self.witness.left_context.encode_canonical(encoder);
        self.witness.right_context.encode_canonical(encoder);
        self.witness.target_context.encode_canonical(encoder);
        encoder.u64(self.witness.left_embedding.len() as u64);
        for ordinal in &self.witness.left_embedding {
            encoder.u32(*ordinal);
        }
        encoder.u64(self.witness.right_embedding.len() as u64);
        for ordinal in &self.witness.right_embedding {
            encoder.u32(*ordinal);
        }
    }
}

struct RawFamilyIdentityMaterialV3<'a> {
    manifest_digest: &'a Digest,
    rank: u16,
    constructor: &'a FamilyConstructorIdentityV3,
    judgment: &'a GenericJudgmentV1,
    role: crate::model::LocalRoleV1,
    support_events: &'a BTreeSet<crate::model::EventIdV1>,
    support_declarations: &'a BTreeSet<pen_kernel::GlobalId>,
    substitution_action_digest: &'a Digest,
}

impl CanonicalEncode for RawFamilyIdentityMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        encoder.u16(self.rank);
        self.constructor.encode_canonical(encoder);
        self.judgment.encode_canonical(encoder);
        self.role.encode_canonical(encoder);
        encoder.u64(self.support_events.len() as u64);
        for event in self.support_events {
            event.encode_canonical(encoder);
        }
        encoder.u64(self.support_declarations.len() as u64);
        for declaration in self.support_declarations {
            declaration.encode_canonical(encoder);
        }
        self.substitution_action_digest.encode_canonical(encoder);
    }
}

/// Re-identify the legacy proof carrier on a domain-separated V3 surface.
/// All constructor references are recursively translated to V3 identifiers;
/// the V1 identifiers are used only as lookup keys in this checked map.
pub(crate) fn derive_v3_family_ids(
    manifest: &VerifiedSemanticAuditManifestV3,
    carrier: &CarrierCertificateV1,
    seed_images: &[Q0SeedImageV3],
) -> Result<BTreeMap<RawFamilyIdV1, NormalizedFamilyIdV3>, FamilyQuotientFailureV3> {
    let mut seed_ids = BTreeMap::new();
    for image in seed_images {
        if seed_ids
            .insert(image.normalized.clone(), image.semantic.clone())
            .is_some()
        {
            return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
        }
    }
    if seed_ids.len() != carrier.verified_seeds().len() {
        return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
    }
    for seed in carrier.verified_seeds() {
        let demand_neutral = match &seed.seed {
            SemanticSchemaSeedV1::PublicHead(seed) => seed.public_support.demand_outputs.is_empty(),
            SemanticSchemaSeedV1::PublicEquation(seed) => {
                seed.public_support.demand_outputs.is_empty() && seed.demand_anchor.is_none()
            }
            SemanticSchemaSeedV1::PublicUniversalInterface { .. } => false,
        };
        if !demand_neutral {
            return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
        }
        if !seed_ids.contains_key(&seed.id) {
            return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
        }
    }

    let mut witness_ids = BTreeMap::new();
    for witness in carrier.context_witnesses() {
        let id = Digest::of_canonical(
            "pen-semantic-audit/context-amalgamation-witness/v3",
            &ContextWitnessIdentityMaterialV3 {
                manifest_digest: manifest.candidate_digest(),
                witness,
            },
        );
        if witness_ids.insert(witness.id.clone(), id).is_some() {
            return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
        }
    }

    let mut family_ids = BTreeMap::new();
    let mut unique_v3_ids = BTreeSet::new();
    for family in carrier.raw_families() {
        if !family.public_support.demand_outputs.is_empty() || family.demand_anchor.is_some() {
            return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
        }
        let constructor = match &family.constructor {
            FamilyConstructorV1::PublicHeadSeed { seed } => {
                FamilyConstructorIdentityV3::PublicHeadSeed {
                    seed: seed_ids
                        .get(seed)
                        .cloned()
                        .ok_or(FamilyQuotientFailureV3::V3IdentityMismatch)?,
                }
            }
            FamilyConstructorV1::PublicEquationSeed { seed } => {
                FamilyConstructorIdentityV3::PublicEquationSeed {
                    seed: seed_ids
                        .get(seed)
                        .cloned()
                        .ok_or(FamilyQuotientFailureV3::V3IdentityMismatch)?,
                }
            }
            FamilyConstructorV1::GenericPublicApplication {
                function,
                argument,
                context_witness,
            } => FamilyConstructorIdentityV3::GenericPublicApplication {
                function: family_ids
                    .get(function)
                    .cloned()
                    .ok_or(FamilyQuotientFailureV3::V3IdentityMismatch)?,
                argument: family_ids
                    .get(argument)
                    .cloned()
                    .ok_or(FamilyQuotientFailureV3::V3IdentityMismatch)?,
                context_witness: witness_ids
                    .get(context_witness)
                    .cloned()
                    .ok_or(FamilyQuotientFailureV3::V3IdentityMismatch)?,
            },
            FamilyConstructorV1::GenericEquationAction {
                equation,
                context,
                hole_ordinal,
                context_witness,
            } => FamilyConstructorIdentityV3::GenericEquationAction {
                equation: family_ids
                    .get(equation)
                    .cloned()
                    .ok_or(FamilyQuotientFailureV3::V3IdentityMismatch)?,
                context: family_ids
                    .get(context)
                    .cloned()
                    .ok_or(FamilyQuotientFailureV3::V3IdentityMismatch)?,
                hole_ordinal: *hole_ordinal,
                context_witness: witness_ids
                    .get(context_witness)
                    .cloned()
                    .ok_or(FamilyQuotientFailureV3::V3IdentityMismatch)?,
            },
        };
        let id = normalized_family_identity_v3(manifest.candidate_digest(), family, &constructor);
        if !unique_v3_ids.insert(id.clone()) || family_ids.insert(family.id.clone(), id).is_some() {
            return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
        }
    }
    if family_ids.len() != carrier.raw_families().len() {
        return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
    }
    Ok(family_ids)
}

fn normalized_family_identity_v3(
    manifest_digest: &Digest,
    family: &crate::model::RawFamilyV1,
    constructor: &FamilyConstructorIdentityV3,
) -> NormalizedFamilyIdV3 {
    NormalizedFamilyIdV3(Digest::of_canonical(
        "pen-semantic-audit/authorized-q0-raw-family/v3",
        &RawFamilyIdentityMaterialV3 {
            manifest_digest,
            rank: family.rank,
            constructor,
            judgment: &family.generic_judgment,
            role: family.role,
            support_events: &family.public_support.events,
            support_declarations: &family.public_support.declarations,
            substitution_action_digest: &family.substitution_action_digest,
        },
    ))
}

struct FamilyClassIdentityMaterialV3<'a> {
    manifest_digest: &'a Digest,
    members: &'a [NormalizedFamilyIdV3],
    judgment: &'a GenericJudgmentV1,
    role: crate::model::LocalRoleV1,
    support_events: &'a BTreeSet<crate::model::EventIdV1>,
    support_declarations: &'a BTreeSet<pen_kernel::GlobalId>,
    substitution_action_digest: &'a Digest,
}

impl CanonicalEncode for FamilyClassIdentityMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        encoder.sequence(self.members);
        self.judgment.encode_canonical(encoder);
        self.role.encode_canonical(encoder);
        encoder.u64(self.support_events.len() as u64);
        for event in self.support_events {
            event.encode_canonical(encoder);
        }
        encoder.u64(self.support_declarations.len() as u64);
        for declaration in self.support_declarations {
            declaration.encode_canonical(encoder);
        }
        self.substitution_action_digest.encode_canonical(encoder);
    }
}

pub(crate) fn derive_v3_classes(
    manifest: &VerifiedSemanticAuditManifestV3,
    quotient: &QuotientCertificateV1,
    raw_ids: &BTreeMap<RawFamilyIdV1, NormalizedFamilyIdV3>,
) -> Result<Vec<VerifiedFamilyClassV3>, FamilyQuotientFailureV3> {
    let mut classes = Vec::with_capacity(quotient.classes().len());
    let mut class_ids = BTreeSet::new();
    let mut covered = BTreeSet::new();
    for class in quotient.classes() {
        if !class.canonical_support.demand_outputs.is_empty() || class.demand_anchor.is_some() {
            return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
        }
        let representative = raw_ids
            .get(&class.representative)
            .cloned()
            .ok_or(FamilyQuotientFailureV3::V3IdentityMismatch)?;
        let mut members = Vec::with_capacity(class.members.len());
        for member in &class.members {
            let mapped = raw_ids
                .get(member)
                .cloned()
                .ok_or(FamilyQuotientFailureV3::V3IdentityMismatch)?;
            if !covered.insert(mapped.clone()) {
                return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
            }
            members.push(mapped);
        }
        members.sort();
        if members.binary_search(&representative).is_err() {
            return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
        }
        let id = family_class_identity_v3(manifest.candidate_digest(), class, &members);
        if !class_ids.insert(id.clone()) {
            return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
        }
        classes.push(VerifiedFamilyClassV3 {
            id,
            representative,
            members: Arc::from(members.into_boxed_slice()),
            generic_judgment: class.generic_judgment.clone(),
            role: class.role,
            canonical_support: class.canonical_support.clone(),
            source_clause: class.source_clause.clone(),
            substitution_action_digest: class.substitution_action_digest.clone(),
        });
    }
    if covered.len() != raw_ids.len() {
        return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
    }
    classes.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(classes)
}

fn family_class_identity_v3(
    manifest_digest: &Digest,
    class: &crate::model::FamilyClassV1,
    members: &[NormalizedFamilyIdV3],
) -> FamilyClassIdV3 {
    FamilyClassIdV3(Digest::of_canonical(
        "pen-semantic-audit/family-class/v3",
        &FamilyClassIdentityMaterialV3 {
            manifest_digest,
            members,
            judgment: &class.generic_judgment,
            role: class.role,
            support_events: &class.canonical_support.events,
            support_declarations: &class.canonical_support.declarations,
            substitution_action_digest: &class.substitution_action_digest,
        },
    ))
}

struct V3QuotientDigestMaterial<'a> {
    semantic_manifest_digest: &'a Digest,
    authorized_carrier_digest: &'a Digest,
    legacy_proof_digest: &'a Digest,
    classes: &'a [VerifiedFamilyClassV3],
}

impl CanonicalEncode for V3QuotientDigestMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.authorized_carrier_digest.encode_canonical(encoder);
        self.legacy_proof_digest.encode_canonical(encoder);
        encoder.sequence(self.classes);
    }
}

#[allow(clippy::too_many_arguments)]
fn verify_exact_family_image(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    signature: &pen_kernel::VerifiedSignature,
    pre: &crate::carrier::PreQ0RawCarrierCertificateV1,
    normalized: &CarrierCertificateV1,
    rewrite: &VerifiedRewriteAuthorityV3,
    fresh: Option<&VerifiedFreshConstructorComputationV1>,
    seed_images: &[Q0SeedImageV3],
    v3_family_ids: &BTreeMap<RawFamilyIdV1, NormalizedFamilyIdV3>,
) -> Result<Vec<Q0FamilyImageV3>, FamilyQuotientFailureV3> {
    let seed_targets = seed_images
        .iter()
        .map(|image| (image.source.clone(), image.normalized.clone()))
        .collect::<BTreeMap<_, _>>();
    if seed_targets.len() != seed_images.len() {
        return Err(FamilyQuotientFailureV3::CarrierImageMismatch);
    }

    // Context-witness IDs are legacy proof identifiers. Match their complete
    // structural payloads so constructor translation does not assume those
    // IDs survive re-enumeration.
    let mut witness_targets = BTreeMap::new();
    let mut claimed_witnesses = BTreeSet::new();
    for source in pre.context_witnesses() {
        let mut matches = normalized
            .context_witnesses()
            .iter()
            .filter(|target| context_witness_payload_equal(source, target));
        let Some(target) = matches.next() else {
            return Err(FamilyQuotientFailureV3::CarrierImageMismatch);
        };
        if matches.next().is_some()
            || !claimed_witnesses.insert(target.id.clone())
            || witness_targets
                .insert(source.id.clone(), target.id.clone())
                .is_some()
        {
            return Err(FamilyQuotientFailureV3::CarrierImageMismatch);
        }
    }
    if claimed_witnesses.len() != normalized.context_witnesses().len() {
        return Err(FamilyQuotientFailureV3::CarrierImageMismatch);
    }

    let mut family_targets = BTreeMap::new();
    let mut claimed_targets = BTreeSet::new();
    let mut images = Vec::with_capacity(pre.raw_families().len());
    for family in pre.raw_families() {
        let judgment =
            normalize_judgment(manifest, kernel, signature, &family.generic_judgment, fresh)?;
        verify_judgment_normal_form(
            kernel,
            signature,
            rewrite,
            &family.generic_judgment,
            &judgment,
        )?;
        let action = substitution_action_digest(&judgment, family.role);
        let constructor = translate_family_constructor(
            &family.constructor,
            &seed_targets,
            &family_targets,
            &witness_targets,
        )?;
        let mut matches = normalized.raw_families().iter().filter(|target| {
            !claimed_targets.contains(&target.id)
                && target.rank == family.rank
                && target.constructor == constructor
                && target.generic_judgment == judgment
                && target.role == family.role
                && target.public_support == family.public_support
                && target.source_clause == family.source_clause
                && target.demand_anchor == family.demand_anchor
                && target.substitution_action_digest == action
        });
        let Some(target) = matches.next() else {
            return Err(FamilyQuotientFailureV3::CarrierImageMismatch);
        };
        if matches.next().is_some() || !claimed_targets.insert(target.id.clone()) {
            return Err(FamilyQuotientFailureV3::CarrierImageMismatch);
        }
        if family_targets
            .insert(family.id.clone(), target.id.clone())
            .is_some()
        {
            return Err(FamilyQuotientFailureV3::CarrierImageMismatch);
        }
        let judgment_digest = Digest::of_canonical(
            "pen-semantic-audit/q0-family-normalized-judgment/v3",
            &judgment,
        );
        let Some(v3_id) = v3_family_ids.get(&target.id).cloned() else {
            return Err(FamilyQuotientFailureV3::V3IdentityMismatch);
        };
        images.push(Q0FamilyImageV3 {
            source: family.id.clone(),
            legacy_normalized: target.id.clone(),
            normalized: v3_id,
            normalized_judgment_digest: judgment_digest,
        });
    }
    if images.len() != pre.raw_families().len()
        || claimed_targets.len() != normalized.raw_families().len()
    {
        return Err(FamilyQuotientFailureV3::CarrierImageMismatch);
    }
    images.sort_by(|left, right| left.source.cmp(&right.source));
    Ok(images)
}

fn context_witness_payload_equal(
    left: &crate::carrier::ContextAmalgamationWitnessV1,
    right: &crate::carrier::ContextAmalgamationWitnessV1,
) -> bool {
    left.kind == right.kind
        && left.left_context == right.left_context
        && left.right_context == right.right_context
        && left.target_context == right.target_context
        && left.left_embedding == right.left_embedding
        && left.right_embedding == right.right_embedding
}

fn translate_family_constructor(
    source: &FamilyConstructorV1,
    seed_targets: &BTreeMap<SeedIdV1, SeedIdV1>,
    family_targets: &BTreeMap<RawFamilyIdV1, RawFamilyIdV1>,
    witness_targets: &BTreeMap<crate::model::ContextWitnessIdV1, crate::model::ContextWitnessIdV1>,
) -> Result<FamilyConstructorV1, FamilyQuotientFailureV3> {
    let missing = || FamilyQuotientFailureV3::CarrierImageMismatch;
    Ok(match source {
        FamilyConstructorV1::PublicHeadSeed { seed } => FamilyConstructorV1::PublicHeadSeed {
            seed: seed_targets.get(seed).cloned().ok_or_else(missing)?,
        },
        FamilyConstructorV1::PublicEquationSeed { seed } => {
            FamilyConstructorV1::PublicEquationSeed {
                seed: seed_targets.get(seed).cloned().ok_or_else(missing)?,
            }
        }
        FamilyConstructorV1::GenericPublicApplication {
            function,
            argument,
            context_witness,
        } => FamilyConstructorV1::GenericPublicApplication {
            function: family_targets.get(function).cloned().ok_or_else(missing)?,
            argument: family_targets.get(argument).cloned().ok_or_else(missing)?,
            context_witness: witness_targets
                .get(context_witness)
                .cloned()
                .ok_or_else(missing)?,
        },
        FamilyConstructorV1::GenericEquationAction {
            equation,
            context,
            hole_ordinal,
            context_witness,
        } => FamilyConstructorV1::GenericEquationAction {
            equation: family_targets.get(equation).cloned().ok_or_else(missing)?,
            context: family_targets.get(context).cloned().ok_or_else(missing)?,
            hole_ordinal: *hole_ordinal,
            context_witness: witness_targets
                .get(context_witness)
                .cloned()
                .ok_or_else(missing)?,
        },
    })
}

fn verify_judgment_normal_form(
    kernel: &Kernel,
    signature: &pen_kernel::VerifiedSignature,
    rewrite: &VerifiedRewriteAuthorityV3,
    source: &GenericJudgmentV1,
    normalized: &GenericJudgmentV1,
) -> Result<(), FamilyQuotientFailureV3> {
    match (source, normalized) {
        (
            GenericJudgmentV1::Term { context, term, ty },
            GenericJudgmentV1::Term {
                context: normalized_context,
                term: normalized_term,
                ty: normalized_ty,
            },
        ) => verify_term_normal_form(
            kernel,
            signature,
            rewrite,
            context,
            term,
            ty,
            normalized_context,
            normalized_term,
            normalized_ty,
        ),
        (
            GenericJudgmentV1::Equation {
                context,
                left,
                right,
                ty,
            },
            GenericJudgmentV1::Equation {
                context: normalized_context,
                left: normalized_left,
                right: normalized_right,
                ty: normalized_ty,
            },
        ) => {
            verify_term_normal_form(
                kernel,
                signature,
                rewrite,
                context,
                left,
                ty,
                normalized_context,
                normalized_left,
                normalized_ty,
            )?;
            verify_term_normal_form(
                kernel,
                signature,
                rewrite,
                context,
                right,
                ty,
                normalized_context,
                normalized_right,
                normalized_ty,
            )
        }
        _ => Err(FamilyQuotientFailureV3::RewriteNormalFormMismatch),
    }
}

#[allow(clippy::too_many_arguments)]
fn verify_term_normal_form(
    kernel: &Kernel,
    signature: &pen_kernel::VerifiedSignature,
    rewrite: &VerifiedRewriteAuthorityV3,
    source_context: &DependentContext,
    source_term: &Term,
    source_ty: &Term,
    normalized_context: &DependentContext,
    normalized_term: &Term,
    normalized_ty: &Term,
) -> Result<(), FamilyQuotientFailureV3> {
    let replay = kernel
        .verify_open_judgment(
            signature,
            &OpenJudgment::HasType {
                context: source_context.clone(),
                term: source_term.clone(),
                ty: source_ty.clone(),
            },
        )
        .map_err(|error| FamilyQuotientFailureV3::SeedNormalization(kernel_unknown(error)))?;
    let OpenJudgment::HasType {
        context: replay_context,
        ty: replay_ty,
        ..
    } = replay
    else {
        return Err(FamilyQuotientFailureV3::RewriteNormalFormMismatch);
    };
    if &replay_context != normalized_context || &replay_ty != normalized_ty {
        return Err(FamilyQuotientFailureV3::RewriteNormalFormMismatch);
    }
    let source_node = rewrite.nodes().iter().find(|node| {
        node.context() == &replay_context
            && node.term() == source_term
            && node.judgment()
                == &RewriteNodeJudgmentV3::HasType {
                    ty: replay_ty.clone(),
                }
    });
    let Some(source_node) = source_node else {
        return Err(FamilyQuotientFailureV3::RewriteNormalFormMismatch);
    };
    let Some(normal_form_id) = rewrite.normal_form_of(source_node.id()) else {
        return Err(FamilyQuotientFailureV3::RewriteNormalFormMismatch);
    };
    let Some(normal_form) = rewrite
        .nodes()
        .iter()
        .find(|node| node.id() == normal_form_id)
    else {
        return Err(FamilyQuotientFailureV3::RewriteNormalFormMismatch);
    };
    if normal_form.context() != normalized_context
        || normal_form.term() != normalized_term
        || normal_form.judgment()
            != &(RewriteNodeJudgmentV3::HasType {
                ty: normalized_ty.clone(),
            })
    {
        return Err(FamilyQuotientFailureV3::RewriteNormalFormMismatch);
    }
    Ok(())
}

fn verify_quotient_coverage(
    carrier: &CarrierCertificateV1,
    quotient: &QuotientCertificateV1,
) -> Result<(), FamilyQuotientFailureV3> {
    let fixed = carrier
        .raw_families()
        .iter()
        .map(|family| family.id.clone())
        .collect::<Vec<_>>();
    if quotient.fixed_vertices() != fixed.as_slice()
        || quotient.raw_to_class().len() != fixed.len()
        || !quotient.q3_registry_verified_empty()
    {
        return Err(FamilyQuotientFailureV3::QuotientCoverageMismatch);
    }
    let class_ids = quotient
        .classes()
        .iter()
        .map(|class| class.id.clone())
        .collect::<BTreeSet<_>>();
    let mut covered = BTreeSet::new();
    for mapping in quotient.raw_to_class() {
        if !covered.insert(mapping.raw.clone()) || !class_ids.contains(&mapping.class) {
            return Err(FamilyQuotientFailureV3::QuotientCoverageMismatch);
        }
    }
    if covered != fixed.into_iter().collect() {
        return Err(FamilyQuotientFailureV3::QuotientCoverageMismatch);
    }
    Ok(())
}

fn kernel_unknown(error: KernelError) -> AuditUnknownReason {
    match error {
        KernelError::ResourceExhausted(
            ResourceKind::Operations | ResourceKind::Depth | ResourceKind::Normalization,
        ) => AuditUnknownReason::ResourceExhausted,
        _ => AuditUnknownReason::KernelCouldNotCertify,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{
        ClauseIdV1, ContextWitnessIdV1, FamilyClassIdV1, FamilyClassV1, LocalRoleV1,
        PublicSupportV1, RawFamilyV1,
    };

    fn judgment() -> GenericJudgmentV1 {
        GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Unit,
            ty: Term::UnitType,
        }
    }

    fn family(source_clause: ClauseIdV1) -> RawFamilyV1 {
        RawFamilyV1 {
            id: RawFamilyIdV1(Digest::of_bytes(b"family-quotient-v3/legacy-family")),
            rank: 1,
            constructor: FamilyConstructorV1::GenericPublicApplication {
                function: RawFamilyIdV1(Digest::of_bytes(b"family-quotient-v3/function")),
                argument: RawFamilyIdV1(Digest::of_bytes(b"family-quotient-v3/argument")),
                context_witness: ContextWitnessIdV1(Digest::of_bytes(
                    b"family-quotient-v3/witness",
                )),
            },
            generic_judgment: judgment(),
            role: LocalRoleV1::SupportAction,
            public_support: PublicSupportV1::default(),
            source_clause: Some(source_clause),
            demand_anchor: None,
            substitution_action_digest: Digest::of_bytes(b"family-quotient-v3/action"),
        }
    }

    #[test]
    fn legacy_clause_lineage_cannot_change_v3_family_or_class_identity() {
        let manifest = Digest::of_bytes(b"family-quotient-v3/manifest");
        let first = family(ClauseIdV1(Digest::of_bytes(b"port-sensitive-clause-a")));
        let second = family(ClauseIdV1(Digest::of_bytes(b"port-sensitive-clause-b")));
        let constructor = FamilyConstructorIdentityV3::GenericPublicApplication {
            function: NormalizedFamilyIdV3(Digest::of_bytes(b"v3-function")),
            argument: NormalizedFamilyIdV3(Digest::of_bytes(b"v3-argument")),
            context_witness: Digest::of_bytes(b"v3-context-witness"),
        };
        assert_eq!(
            normalized_family_identity_v3(&manifest, &first, &constructor),
            normalized_family_identity_v3(&manifest, &second, &constructor)
        );

        let members = [NormalizedFamilyIdV3(Digest::of_bytes(b"v3-member"))];
        let first_class = FamilyClassV1 {
            id: FamilyClassIdV1(Digest::of_bytes(b"legacy-class")),
            representative: first.id.clone(),
            members: vec![first.id.clone()],
            generic_judgment: first.generic_judgment.clone(),
            role: first.role,
            canonical_support: first.public_support.clone(),
            source_clause: first.source_clause.clone(),
            demand_anchor: None,
            substitution_action_digest: first.substitution_action_digest.clone(),
        };
        let mut second_class = first_class.clone();
        second_class.source_clause = second.source_clause;
        assert_eq!(
            family_class_identity_v3(&manifest, &first_class, &members),
            family_class_identity_v3(&manifest, &second_class, &members)
        );
    }

    #[test]
    fn constructor_tree_and_class_membership_are_identity_bearing() {
        let manifest = Digest::of_bytes(b"family-quotient-v3/manifest");
        let family = family(ClauseIdV1(Digest::of_bytes(b"legacy-clause")));
        let left = FamilyConstructorIdentityV3::GenericPublicApplication {
            function: NormalizedFamilyIdV3(Digest::of_bytes(b"v3-left")),
            argument: NormalizedFamilyIdV3(Digest::of_bytes(b"v3-right")),
            context_witness: Digest::of_bytes(b"v3-context-witness"),
        };
        let right = FamilyConstructorIdentityV3::GenericPublicApplication {
            function: NormalizedFamilyIdV3(Digest::of_bytes(b"v3-right")),
            argument: NormalizedFamilyIdV3(Digest::of_bytes(b"v3-left")),
            context_witness: Digest::of_bytes(b"v3-context-witness"),
        };
        assert_ne!(
            normalized_family_identity_v3(&manifest, &family, &left),
            normalized_family_identity_v3(&manifest, &family, &right)
        );

        let class = FamilyClassV1 {
            id: FamilyClassIdV1(Digest::of_bytes(b"legacy-class")),
            representative: family.id.clone(),
            members: vec![family.id.clone()],
            generic_judgment: family.generic_judgment.clone(),
            role: family.role,
            canonical_support: family.public_support.clone(),
            source_clause: family.source_clause.clone(),
            demand_anchor: None,
            substitution_action_digest: family.substitution_action_digest.clone(),
        };
        assert_ne!(
            family_class_identity_v3(
                &manifest,
                &class,
                &[NormalizedFamilyIdV3(Digest::of_bytes(b"member-a"))],
            ),
            family_class_identity_v3(
                &manifest,
                &class,
                &[NormalizedFamilyIdV3(Digest::of_bytes(b"member-b"))],
            )
        );
    }
}
