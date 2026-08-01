//! Derived family weakening and restriction for the V3 semantic-family
//! quotient (Phase J, `WeakeningAndRestriction`).
//!
//! This stage accepts no caller family sets, maps, support claims, or
//! conservativity flag. It reconstructs the predecessor Authorized-Q0
//! carrier and quotient from the exact verified inventory, compares that
//! reconstruction with the recursively old-leaf restriction of the
//! successor proof, and mints an opaque class injection with its inverse on
//! the image. The marginal-family complement is deliberately a later stage.

use crate::carrier::{
    CarrierCertificateV1, VerifiedSemanticSeedV1, enumerate_raw_families_with_q0_v1,
    substitution_action_digest,
};
use crate::family_quotient_v3::{
    FamilyClassIdV3, NormalizedFamilyIdV3, Q0SeedImageV3, VerifiedFamilyClassV3,
    VerifiedFamilyQuotientV3, derive_v3_classes, derive_v3_family_ids,
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
    EventIdV1, FamilyClassIdV1, FamilyClassV1, FamilyConstructorV1, PublicSupportV1, RawFamilyIdV1,
    RawFamilyV1, SeedIdV1, SemanticSchemaSeedV1,
};
use crate::quotient::{Q2PairDispositionV1, QuotientCertificateV1, quotient_families_with_q0_v1};
use crate::rewrite_authority_v3::VerifiedRewriteAuthorityV3;
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, GlobalId, Kernel};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub const FAMILY_WEAKENING_SCHEMA_VERSION_V3: u16 = 1;

#[derive(Clone, Debug)]
pub enum FamilyWeakeningFailureV3 {
    ExactManifestIdentityMismatch,
    ChainBindingMismatch,
    NonEmptyPredecessorRewriteUnsupported,
    PredecessorSeedCensusMismatch,
    PredecessorCarrier(AuditUnknownReason),
    OutsideFragment(OutsideFragmentReason),
    RecursiveSupportMismatch,
    RawRestrictionMismatch,
    V3IdentityMismatch,
    PredecessorQuotient(AuditUnknownReason),
    Q1TransportMismatch,
    Q2TransportMismatch,
    Q3TransportMismatch,
    ClassMapNotWellDefined,
    FailedRetraction,
}

impl std::fmt::Display for FamilyWeakeningFailureV3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactManifestIdentityMismatch => formatter.write_str(
                "the supplied manifests are not the exact registered lambda/unit proposals",
            ),
            Self::ChainBindingMismatch => formatter.write_str(
                "the inventory, rewrite authority, and successor quotient are not one verified chain",
            ),
            Self::NonEmptyPredecessorRewriteUnsupported => formatter.write_str(
                "the current historical authority does not issue a nonempty predecessor equation system",
            ),
            Self::PredecessorSeedCensusMismatch => formatter.write_str(
                "the verifier-derived predecessor seeds do not exhaust the predecessor public inventory",
            ),
            Self::PredecessorCarrier(reason) => {
                write!(formatter, "predecessor Authorized-Q0 carrier reconstruction failed: {reason:?}")
            }
            Self::OutsideFragment(reason) => {
                write!(formatter, "outside the registered lambda/unit fragment: {reason:?}")
            }
            Self::RecursiveSupportMismatch => formatter.write_str(
                "recursive constructor support does not induce the exact old/new seed partition",
            ),
            Self::RawRestrictionMismatch => formatter.write_str(
                "the predecessor carrier is not the exact recursively old-leaf successor restriction",
            ),
            Self::V3IdentityMismatch => formatter.write_str(
                "the predecessor and successor raw-family proofs do not preserve V3 identity",
            ),
            Self::PredecessorQuotient(reason) => {
                write!(formatter, "predecessor family quotient reconstruction failed: {reason:?}")
            }
            Self::Q1TransportMismatch => formatter.write_str(
                "the predecessor Q1 census is not the exact successor old-old restriction",
            ),
            Self::Q2TransportMismatch => formatter.write_str(
                "the predecessor Q2 census is not the exact successor old-old restriction",
            ),
            Self::Q3TransportMismatch => formatter.write_str(
                "the predecessor and successor Q3 empty registries do not transport exactly",
            ),
            Self::ClassMapNotWellDefined => formatter.write_str(
                "a predecessor quotient class does not map to exactly one successor class",
            ),
            Self::FailedRetraction => formatter.write_str(
                "the derived class map is not injective and cannot admit the required retraction",
            ),
        }
    }
}

impl std::error::Error for FamilyWeakeningFailureV3 {}

impl FamilyWeakeningFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::OutsideFragment(reason) => AuditDecision::OutsideFragment(reason),
            Self::PredecessorCarrier(AuditUnknownReason::ResourceExhausted)
            | Self::PredecessorQuotient(AuditUnknownReason::ResourceExhausted) => {
                AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted)
            }
            _ => AuditDecision::Unknown(AuditUnknownReason::MissingWeakeningImageConservativity),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FamilyWeakeningMapV3 {
    source: FamilyClassIdV3,
    target: FamilyClassIdV3,
}

impl FamilyWeakeningMapV3 {
    pub fn source(&self) -> &FamilyClassIdV3 {
        &self.source
    }

    pub fn target(&self) -> &FamilyClassIdV3 {
        &self.target
    }
}

impl CanonicalEncode for FamilyWeakeningMapV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source.encode_canonical(encoder);
        self.target.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FamilyRestrictionMapV3 {
    source: FamilyClassIdV3,
    target: FamilyClassIdV3,
}

impl FamilyRestrictionMapV3 {
    pub fn source(&self) -> &FamilyClassIdV3 {
        &self.source
    }

    pub fn target(&self) -> &FamilyClassIdV3 {
        &self.target
    }
}

impl CanonicalEncode for FamilyRestrictionMapV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source.encode_canonical(encoder);
        self.target.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RawFamilyWeakeningV3 {
    source: NormalizedFamilyIdV3,
    target: NormalizedFamilyIdV3,
}

impl RawFamilyWeakeningV3 {
    pub fn source(&self) -> &NormalizedFamilyIdV3 {
        &self.source
    }

    pub fn target(&self) -> &NormalizedFamilyIdV3 {
        &self.target
    }
}

impl CanonicalEncode for RawFamilyWeakeningV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source.encode_canonical(encoder);
        self.target.encode_canonical(encoder);
    }
}

/// Opaque V3 weakening/restriction authority. There is no deserialization
/// path and no constructor from asserted family sets or maps.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedFamilyWeakeningV3 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    v1_manifest_digest: Digest,
    v2_manifest_digest: Digest,
    kernel_protocol_digest: Digest,
    normalizer_protocol_digest: Digest,
    inventory_digest: Digest,
    exact_extension_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    new_event: EventIdV1,
    rewrite_authority_digest: Digest,
    predecessor_reconstruction_digest: Digest,
    successor_quotient_digest: Digest,
    predecessor_seed_census_digest: Digest,
    predecessor_carrier_digest: Digest,
    predecessor_carrier: CarrierCertificateV1,
    recursive_support_digest: Digest,
    raw_weakening_digest: Digest,
    raw_weakening: Arc<[RawFamilyWeakeningV3]>,
    q1_transport_digest: Digest,
    q2_transport_digest: Digest,
    predecessor_quotient_digest: Digest,
    predecessor_quotient: QuotientCertificateV1,
    predecessor_classes: Arc<[VerifiedFamilyClassV3]>,
    class_map_digest: Digest,
    weakening: Arc<[FamilyWeakeningMapV3]>,
    restriction_on_image: Arc<[FamilyRestrictionMapV3]>,
    image: Arc<[FamilyClassIdV3]>,
    digest: Digest,
}

impl VerifiedFamilyWeakeningV3 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn exact_extension_digest(&self) -> &Digest {
        &self.exact_extension_digest
    }

    pub fn predecessor_boundary_digest(&self) -> &Digest {
        &self.predecessor_boundary_digest
    }

    pub fn successor_boundary_digest(&self) -> &Digest {
        &self.successor_boundary_digest
    }

    pub fn new_event(&self) -> &EventIdV1 {
        &self.new_event
    }

    pub fn rewrite_authority_digest(&self) -> &Digest {
        &self.rewrite_authority_digest
    }

    pub fn predecessor_reconstruction_digest(&self) -> &Digest {
        &self.predecessor_reconstruction_digest
    }

    pub fn successor_quotient_digest(&self) -> &Digest {
        &self.successor_quotient_digest
    }

    pub fn predecessor_quotient_digest(&self) -> &Digest {
        &self.predecessor_quotient_digest
    }

    pub fn predecessor_raw_family_count(&self) -> usize {
        self.predecessor_carrier.raw_families().len()
    }

    pub fn predecessor_seed_count(&self) -> usize {
        self.predecessor_carrier.verified_seeds().len()
    }

    pub fn predecessor_q2_disposition_count(&self) -> usize {
        self.predecessor_quotient.q2_pair_dispositions().len()
    }

    pub fn raw_weakening(&self) -> &[RawFamilyWeakeningV3] {
        &self.raw_weakening
    }

    pub fn predecessor_classes(&self) -> &[VerifiedFamilyClassV3] {
        &self.predecessor_classes
    }

    pub fn weakening(&self) -> &[FamilyWeakeningMapV3] {
        &self.weakening
    }

    pub fn restriction_on_image(&self) -> &[FamilyRestrictionMapV3] {
        &self.restriction_on_image
    }

    /// Exact successor-class image of weakening. This is not the marginal
    /// complement and confers no marginal-family authority.
    pub fn image(&self) -> &[FamilyClassIdV3] {
        &self.image
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedFamilyWeakeningV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.v1_manifest_digest.encode_canonical(encoder);
        self.v2_manifest_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.inventory_digest.encode_canonical(encoder);
        self.exact_extension_digest.encode_canonical(encoder);
        self.predecessor_boundary_digest.encode_canonical(encoder);
        self.successor_boundary_digest.encode_canonical(encoder);
        self.new_event.encode_canonical(encoder);
        self.rewrite_authority_digest.encode_canonical(encoder);
        self.predecessor_reconstruction_digest
            .encode_canonical(encoder);
        self.successor_quotient_digest.encode_canonical(encoder);
        self.predecessor_seed_census_digest
            .encode_canonical(encoder);
        self.predecessor_carrier_digest.encode_canonical(encoder);
        self.predecessor_carrier.encode_canonical(encoder);
        self.recursive_support_digest.encode_canonical(encoder);
        self.raw_weakening_digest.encode_canonical(encoder);
        encoder.sequence(&self.raw_weakening);
        self.q1_transport_digest.encode_canonical(encoder);
        self.q2_transport_digest.encode_canonical(encoder);
        self.predecessor_quotient_digest.encode_canonical(encoder);
        self.predecessor_quotient.encode_canonical(encoder);
        encoder.sequence(&self.predecessor_classes);
        self.class_map_digest.encode_canonical(encoder);
        encoder.sequence(&self.weakening);
        encoder.sequence(&self.restriction_on_image);
        encoder.sequence(&self.image);
    }
}

#[allow(clippy::too_many_arguments)]
pub fn diagnose_family_weakening_v3(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    rewrite_authority: &VerifiedRewriteAuthorityV3,
    successor_quotient: &VerifiedFamilyQuotientV3,
) -> Result<VerifiedFamilyWeakeningV3, FamilyWeakeningFailureV3> {
    verify_exact_manifests(v1_manifest, v2_manifest, v3_manifest)?;

    let predecessor_signature = inventory.predecessor_boundary();
    let successor_signature = inventory.successor_boundary();
    let exact_extension = inventory.exact_extension();
    if inventory.manifest_digest() != v1_manifest.candidate_digest()
        || inventory.normalizer_protocol_digest() != &kernel.normalizer_protocol_digest()
        || exact_extension.predecessor_boundary_digest() != predecessor_signature.digest()
        || exact_extension.successor_boundary_digest() != successor_signature.digest()
        || rewrite_authority.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || rewrite_authority.signature_digest() != successor_signature.digest()
        || rewrite_authority.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
        || successor_quotient.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || successor_quotient.v1_manifest_digest() != v1_manifest.candidate_digest()
        || successor_quotient.v2_manifest_digest() != v2_manifest.candidate_digest()
        || successor_quotient.signature_digest() != successor_signature.digest()
        || successor_quotient.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
        || successor_quotient.normalizer_protocol_digest() != &kernel.normalizer_protocol_digest()
        || successor_quotient.inventory_digest() != inventory.digest()
        || successor_quotient.native_carrier_digest() != rewrite_authority.carrier_digest()
        || successor_quotient.rewrite_authority_digest() != rewrite_authority.digest()
    {
        return Err(FamilyWeakeningFailureV3::ChainBindingMismatch);
    }

    let predecessor_declarations = predecessor_signature
        .declarations()
        .iter()
        .map(|declaration| declaration.id.clone())
        .collect::<BTreeSet<_>>();
    let inventoried_predecessor_declarations = inventory
        .declarations()
        .iter()
        .filter(|declaration| predecessor_declarations.contains(declaration.declaration()))
        .map(|declaration| declaration.declaration().clone())
        .collect::<BTreeSet<_>>();
    if inventoried_predecessor_declarations != predecessor_declarations {
        return Err(FamilyWeakeningFailureV3::PredecessorSeedCensusMismatch);
    }
    let predecessor_equations = inventory
        .equations()
        .iter()
        .filter(|equation| equation.is_predecessor_public())
        .map(|equation| equation.equation().clone())
        .collect::<BTreeSet<_>>();
    if !predecessor_equations.is_empty() {
        return Err(FamilyWeakeningFailureV3::NonEmptyPredecessorRewriteUnsupported);
    }

    let successor_carrier = successor_quotient.authorized_carrier_proof();
    let successor_legacy_quotient = successor_quotient.legacy_quotient_proof();
    if successor_carrier.signature_digest() != successor_signature.digest()
        || successor_carrier.manifest_digest() != v1_manifest.candidate_digest()
        || successor_carrier.normalizer_protocol_digest() != &kernel.normalizer_protocol_digest()
        || !successor_carrier.q3_registry_verified_empty()
        || successor_legacy_quotient.signature_digest() != successor_signature.digest()
        || successor_legacy_quotient.manifest_digest() != v1_manifest.candidate_digest()
    {
        return Err(FamilyWeakeningFailureV3::ChainBindingMismatch);
    }

    let mut predecessor_seed_wires = Vec::new();
    let mut predecessor_seed_ids = BTreeSet::new();
    for seed in successor_carrier.verified_seeds() {
        let is_predecessor = match &seed.seed {
            SemanticSchemaSeedV1::PublicHead(head) => {
                predecessor_declarations.contains(&head.declaration)
            }
            SemanticSchemaSeedV1::PublicEquation(equation) => {
                predecessor_equations.contains(&equation.equation)
            }
            SemanticSchemaSeedV1::PublicUniversalInterface { .. } => {
                return Err(FamilyWeakeningFailureV3::PredecessorSeedCensusMismatch);
            }
        };
        verify_seed_partition(
            seed,
            is_predecessor,
            exact_extension.event(),
            &predecessor_declarations,
        )?;
        if is_predecessor {
            if !predecessor_seed_ids.insert(seed.id.clone()) {
                return Err(FamilyWeakeningFailureV3::PredecessorSeedCensusMismatch);
            }
            predecessor_seed_wires.push(seed.seed.clone());
        }
    }
    let expected_predecessor_seed_count = predecessor_declarations
        .len()
        .checked_add(predecessor_equations.len())
        .ok_or(FamilyWeakeningFailureV3::PredecessorSeedCensusMismatch)?;
    if predecessor_seed_ids.len() != expected_predecessor_seed_count
        || successor_carrier.verified_seeds().len()
            != inventory
                .declarations()
                .len()
                .saturating_add(inventory.equations().len())
    {
        return Err(FamilyWeakeningFailureV3::PredecessorSeedCensusMismatch);
    }

    let predecessor_seed_images = successor_quotient
        .seed_images()
        .iter()
        .filter(|image| predecessor_seed_ids.contains(image.normalized()))
        .cloned()
        .collect::<Vec<_>>();
    if predecessor_seed_images.len() != predecessor_seed_ids.len()
        || predecessor_seed_images
            .iter()
            .map(Q0SeedImageV3::normalized)
            .collect::<BTreeSet<_>>()
            != predecessor_seed_ids.iter().collect::<BTreeSet<_>>()
    {
        return Err(FamilyWeakeningFailureV3::PredecessorSeedCensusMismatch);
    }
    let predecessor_seed_census_digest = Digest::of_canonical(
        "pen-semantic-audit/predecessor-authorized-q0-seed-census/v3",
        &PredecessorSeedCensusMaterial {
            inventory: inventory.digest(),
            predecessor_boundary: predecessor_signature.digest(),
            new_event: exact_extension.event(),
            seed_images: &predecessor_seed_images,
        },
    );

    let predecessor_carrier = match enumerate_raw_families_with_q0_v1(
        kernel,
        predecessor_signature,
        v1_manifest,
        &predecessor_seed_wires,
        &[],
        None,
    ) {
        AuditDecision::Proven(carrier) => carrier,
        AuditDecision::Unknown(reason) => {
            return Err(FamilyWeakeningFailureV3::PredecessorCarrier(reason));
        }
        AuditDecision::OutsideFragment(reason) => {
            return Err(FamilyWeakeningFailureV3::OutsideFragment(reason));
        }
    };
    if predecessor_carrier.verified_seeds().len() != predecessor_seed_ids.len()
        || predecessor_carrier
            .verified_seeds()
            .iter()
            .map(|seed| seed.id.clone())
            .collect::<BTreeSet<_>>()
            != predecessor_seed_ids
        || predecessor_carrier.fresh_program_digest().is_some()
        || !predecessor_carrier.q3_registry_verified_empty()
    {
        return Err(FamilyWeakeningFailureV3::PredecessorSeedCensusMismatch);
    }

    let successor_partition = derive_recursive_partition(
        successor_carrier,
        &predecessor_seed_ids,
        exact_extension.event(),
        &predecessor_declarations,
    )?;
    let predecessor_partition = derive_recursive_partition(
        &predecessor_carrier,
        &predecessor_seed_ids,
        exact_extension.event(),
        &predecessor_declarations,
    )?;
    if predecessor_partition.old_families.len() != predecessor_carrier.raw_families().len() {
        return Err(FamilyWeakeningFailureV3::RawRestrictionMismatch);
    }
    verify_exact_raw_restriction(
        &predecessor_carrier,
        successor_carrier,
        &successor_partition.old_families,
    )?;

    let predecessor_carrier_digest = Digest::of_canonical(
        "pen-semantic-audit/predecessor-authorized-q0-carrier/v3",
        &predecessor_carrier,
    );
    let recursive_support_digest = Digest::of_canonical(
        "pen-semantic-audit/family-weakening-recursive-support/v3",
        &RecursiveSupportMaterial {
            predecessor: &predecessor_partition.entries,
            successor: &successor_partition.entries,
        },
    );

    let predecessor_v3_ids =
        derive_v3_family_ids(v3_manifest, &predecessor_carrier, &predecessor_seed_images)
            .map_err(|_| FamilyWeakeningFailureV3::V3IdentityMismatch)?;
    let successor_v3_ids = derive_v3_family_ids(
        v3_manifest,
        successor_carrier,
        successor_quotient.seed_images(),
    )
    .map_err(|_| FamilyWeakeningFailureV3::V3IdentityMismatch)?;
    let mut raw_weakening = Vec::with_capacity(predecessor_v3_ids.len());
    for family in predecessor_carrier.raw_families() {
        let source = predecessor_v3_ids
            .get(&family.id)
            .cloned()
            .ok_or(FamilyWeakeningFailureV3::V3IdentityMismatch)?;
        let target = successor_v3_ids
            .get(&family.id)
            .cloned()
            .ok_or(FamilyWeakeningFailureV3::V3IdentityMismatch)?;
        if source != target {
            return Err(FamilyWeakeningFailureV3::V3IdentityMismatch);
        }
        raw_weakening.push(RawFamilyWeakeningV3 { source, target });
    }
    raw_weakening.sort_by(|left, right| left.source.cmp(&right.source));
    let raw_weakening_digest = Digest::of_canonical(
        "pen-semantic-audit/raw-family-weakening/v3",
        &CanonicalSequence(&raw_weakening),
    );

    let predecessor_quotient = match quotient_families_with_q0_v1(
        kernel,
        predecessor_signature,
        v1_manifest,
        &predecessor_carrier,
        None,
    ) {
        AuditDecision::Proven(quotient) => quotient,
        AuditDecision::Unknown(reason) => {
            return Err(FamilyWeakeningFailureV3::PredecessorQuotient(reason));
        }
        AuditDecision::OutsideFragment(reason) => {
            return Err(FamilyWeakeningFailureV3::OutsideFragment(reason));
        }
    };
    let (q1_transport_digest, q2_transport_digest) = verify_quotient_transport(
        &predecessor_quotient,
        successor_legacy_quotient,
        &successor_partition.old_families,
    )?;

    let predecessor_classes =
        derive_v3_classes(v3_manifest, &predecessor_quotient, &predecessor_v3_ids)
            .map_err(|_| FamilyWeakeningFailureV3::V3IdentityMismatch)?;
    let legacy_class_map = derive_legacy_class_map(
        &predecessor_quotient,
        successor_legacy_quotient,
        &successor_partition.old_families,
    )?;
    let predecessor_class_ids = match_legacy_to_v3_classes(
        predecessor_quotient.classes(),
        &predecessor_classes,
        &predecessor_v3_ids,
    )?;
    let successor_class_ids = match_legacy_to_v3_classes(
        successor_legacy_quotient.classes(),
        successor_quotient.classes(),
        &successor_v3_ids,
    )?;

    let mut weakening = Vec::with_capacity(legacy_class_map.len());
    let mut restriction_on_image = Vec::with_capacity(legacy_class_map.len());
    let mut image = BTreeSet::new();
    for (source, target) in legacy_class_map {
        let source = predecessor_class_ids
            .get(&source)
            .cloned()
            .ok_or(FamilyWeakeningFailureV3::V3IdentityMismatch)?;
        let target = successor_class_ids
            .get(&target)
            .cloned()
            .ok_or(FamilyWeakeningFailureV3::V3IdentityMismatch)?;
        if !image.insert(target.clone()) {
            return Err(FamilyWeakeningFailureV3::FailedRetraction);
        }
        weakening.push(FamilyWeakeningMapV3 {
            source: source.clone(),
            target: target.clone(),
        });
        restriction_on_image.push(FamilyRestrictionMapV3 {
            source: target,
            target: source,
        });
    }
    weakening.sort_by(|left, right| left.source.cmp(&right.source));
    restriction_on_image.sort_by(|left, right| left.source.cmp(&right.source));
    verify_retraction(&weakening, &restriction_on_image)?;
    let image = image.into_iter().collect::<Vec<_>>();
    let class_map_digest = Digest::of_canonical(
        "pen-semantic-audit/family-class-weakening-and-restriction/v3",
        &ClassMapMaterial {
            weakening: &weakening,
            restriction: &restriction_on_image,
            image: &image,
        },
    );
    let predecessor_quotient_digest = Digest::of_canonical(
        "pen-semantic-audit/predecessor-family-quotient/v3",
        &PredecessorQuotientMaterial {
            carrier: &predecessor_carrier_digest,
            quotient: &predecessor_quotient,
            classes: &predecessor_classes,
        },
    );

    let mut verified = VerifiedFamilyWeakeningV3 {
        schema_version: FAMILY_WEAKENING_SCHEMA_VERSION_V3,
        semantic_manifest_digest: v3_manifest.candidate_digest().clone(),
        v1_manifest_digest: v1_manifest.candidate_digest().clone(),
        v2_manifest_digest: v2_manifest.candidate_digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        inventory_digest: inventory.digest().clone(),
        exact_extension_digest: exact_extension.digest().clone(),
        predecessor_boundary_digest: predecessor_signature.digest().clone(),
        successor_boundary_digest: successor_signature.digest().clone(),
        new_event: exact_extension.event().clone(),
        rewrite_authority_digest: rewrite_authority.digest().clone(),
        predecessor_reconstruction_digest: rewrite_authority
            .predecessor_reconstruction_digest()
            .clone(),
        successor_quotient_digest: successor_quotient.digest().clone(),
        predecessor_seed_census_digest,
        predecessor_carrier_digest,
        predecessor_carrier,
        recursive_support_digest,
        raw_weakening_digest,
        raw_weakening: Arc::from(raw_weakening.into_boxed_slice()),
        q1_transport_digest,
        q2_transport_digest,
        predecessor_quotient_digest,
        predecessor_quotient,
        predecessor_classes: Arc::from(predecessor_classes.into_boxed_slice()),
        class_map_digest,
        weakening: Arc::from(weakening.into_boxed_slice()),
        restriction_on_image: Arc::from(restriction_on_image.into_boxed_slice()),
        image: Arc::from(image.into_boxed_slice()),
        digest: Digest::of_bytes(b"pending verified family weakening v3"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-family-weakening-and-restriction/v3",
        &verified,
    );
    Ok(verified)
}

#[allow(clippy::too_many_arguments)]
pub fn verify_family_weakening_v3(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    rewrite_authority: &VerifiedRewriteAuthorityV3,
    successor_quotient: &VerifiedFamilyQuotientV3,
) -> AuditDecision<VerifiedFamilyWeakeningV3> {
    match diagnose_family_weakening_v3(
        v1_manifest,
        v2_manifest,
        v3_manifest,
        kernel,
        inventory,
        rewrite_authority,
        successor_quotient,
    ) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(failure) => failure.into_decision(),
    }
}

fn verify_exact_manifests(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
) -> Result<(), FamilyWeakeningFailureV3> {
    if v1_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v1()
        || v2_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v2()
        || v3_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v3()
    {
        return Err(FamilyWeakeningFailureV3::ExactManifestIdentityMismatch);
    }
    Ok(())
}

fn seed_support(
    seed: &VerifiedSemanticSeedV1,
) -> Result<&PublicSupportV1, FamilyWeakeningFailureV3> {
    match &seed.seed {
        SemanticSchemaSeedV1::PublicHead(head) => Ok(&head.public_support),
        SemanticSchemaSeedV1::PublicEquation(equation) => Ok(&equation.public_support),
        SemanticSchemaSeedV1::PublicUniversalInterface { .. } => {
            Err(FamilyWeakeningFailureV3::PredecessorSeedCensusMismatch)
        }
    }
}

fn seed_origin(seed: &VerifiedSemanticSeedV1) -> Result<&EventIdV1, FamilyWeakeningFailureV3> {
    match &seed.seed {
        SemanticSchemaSeedV1::PublicHead(head) => Ok(&head.origin_event),
        SemanticSchemaSeedV1::PublicEquation(equation) => Ok(&equation.origin_event),
        SemanticSchemaSeedV1::PublicUniversalInterface { .. } => {
            Err(FamilyWeakeningFailureV3::PredecessorSeedCensusMismatch)
        }
    }
}

fn verify_seed_partition(
    seed: &VerifiedSemanticSeedV1,
    is_predecessor: bool,
    new_event: &EventIdV1,
    predecessor_declarations: &BTreeSet<GlobalId>,
) -> Result<(), FamilyWeakeningFailureV3> {
    let support = seed_support(seed)?;
    if !support.demand_outputs.is_empty()
        || is_predecessor != (seed_origin(seed)? != new_event)
        || is_predecessor
            && (support.events.contains(new_event)
                || !support.declarations.is_subset(predecessor_declarations))
        || !is_predecessor && !support.events.contains(new_event)
    {
        return Err(FamilyWeakeningFailureV3::PredecessorSeedCensusMismatch);
    }
    Ok(())
}

#[derive(Clone)]
struct DerivedRecursiveSupport {
    old: bool,
    support: PublicSupportV1,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RecursiveSupportEntryV3 {
    family: RawFamilyIdV1,
    old: bool,
    support: PublicSupportV1,
}

impl CanonicalEncode for RecursiveSupportEntryV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.family.encode_canonical(encoder);
        encoder.tag(u8::from(self.old));
        self.support.encode_canonical(encoder);
    }
}

struct RecursivePartitionV3 {
    old_families: BTreeSet<RawFamilyIdV1>,
    entries: Vec<RecursiveSupportEntryV3>,
}

fn derive_recursive_partition(
    carrier: &CarrierCertificateV1,
    predecessor_seeds: &BTreeSet<SeedIdV1>,
    new_event: &EventIdV1,
    predecessor_declarations: &BTreeSet<GlobalId>,
) -> Result<RecursivePartitionV3, FamilyWeakeningFailureV3> {
    let mut raw_index = BTreeMap::new();
    for family in carrier.raw_families() {
        if raw_index.insert(family.id.clone(), family).is_some() {
            return Err(FamilyWeakeningFailureV3::RecursiveSupportMismatch);
        }
    }
    let mut seed_index = BTreeMap::new();
    for seed in carrier.verified_seeds() {
        if seed_index.insert(seed.id.clone(), seed).is_some() {
            return Err(FamilyWeakeningFailureV3::RecursiveSupportMismatch);
        }
    }

    let mut memo = BTreeMap::new();
    let mut entries = Vec::with_capacity(raw_index.len());
    let mut old_families = BTreeSet::new();
    for family in carrier.raw_families() {
        let derived = derive_recursive_support(
            &family.id,
            &raw_index,
            &seed_index,
            predecessor_seeds,
            &mut memo,
            &mut BTreeSet::new(),
        )?;
        if derived.support != family.public_support
            || !family.public_support.demand_outputs.is_empty()
            || family.substitution_action_digest
                != substitution_action_digest(&family.generic_judgment, family.role)
            || derived.old
                && (derived.support.events.contains(new_event)
                    || !derived
                        .support
                        .declarations
                        .is_subset(predecessor_declarations))
            || !derived.old && !derived.support.events.contains(new_event)
        {
            return Err(FamilyWeakeningFailureV3::RecursiveSupportMismatch);
        }
        if derived.old {
            old_families.insert(family.id.clone());
        }
        entries.push(RecursiveSupportEntryV3 {
            family: family.id.clone(),
            old: derived.old,
            support: derived.support,
        });
    }
    entries.sort_by(|left, right| left.family.cmp(&right.family));
    Ok(RecursivePartitionV3 {
        old_families,
        entries,
    })
}

fn derive_recursive_support(
    id: &RawFamilyIdV1,
    raw_index: &BTreeMap<RawFamilyIdV1, &RawFamilyV1>,
    seed_index: &BTreeMap<SeedIdV1, &VerifiedSemanticSeedV1>,
    predecessor_seeds: &BTreeSet<SeedIdV1>,
    memo: &mut BTreeMap<RawFamilyIdV1, DerivedRecursiveSupport>,
    visiting: &mut BTreeSet<RawFamilyIdV1>,
) -> Result<DerivedRecursiveSupport, FamilyWeakeningFailureV3> {
    if let Some(derived) = memo.get(id) {
        return Ok(derived.clone());
    }
    let family = raw_index
        .get(id)
        .ok_or(FamilyWeakeningFailureV3::RecursiveSupportMismatch)?;
    if !visiting.insert(id.clone()) {
        return Err(FamilyWeakeningFailureV3::RecursiveSupportMismatch);
    }
    let derived = match &family.constructor {
        FamilyConstructorV1::PublicHeadSeed { seed }
        | FamilyConstructorV1::PublicEquationSeed { seed } => {
            let verified = seed_index
                .get(seed)
                .ok_or(FamilyWeakeningFailureV3::RecursiveSupportMismatch)?;
            let constructor_matches = matches!(
                (&family.constructor, &verified.seed),
                (
                    FamilyConstructorV1::PublicHeadSeed { .. },
                    SemanticSchemaSeedV1::PublicHead(_)
                ) | (
                    FamilyConstructorV1::PublicEquationSeed { .. },
                    SemanticSchemaSeedV1::PublicEquation(_)
                )
            );
            if family.rank != 0
                || !constructor_matches
                || family.generic_judgment != verified.normalized_judgment
                || family.role != verified.derived_role
            {
                return Err(FamilyWeakeningFailureV3::RecursiveSupportMismatch);
            }
            DerivedRecursiveSupport {
                old: predecessor_seeds.contains(seed),
                support: seed_support(verified)?.clone(),
            }
        }
        FamilyConstructorV1::GenericPublicApplication {
            function, argument, ..
        } => derive_composite_support(
            family,
            function,
            argument,
            raw_index,
            seed_index,
            predecessor_seeds,
            memo,
            visiting,
        )?,
        FamilyConstructorV1::GenericEquationAction {
            equation, context, ..
        } => derive_composite_support(
            family,
            equation,
            context,
            raw_index,
            seed_index,
            predecessor_seeds,
            memo,
            visiting,
        )?,
    };
    visiting.remove(id);
    memo.insert(id.clone(), derived.clone());
    Ok(derived)
}

#[allow(clippy::too_many_arguments)]
fn derive_composite_support(
    family: &RawFamilyV1,
    left: &RawFamilyIdV1,
    right: &RawFamilyIdV1,
    raw_index: &BTreeMap<RawFamilyIdV1, &RawFamilyV1>,
    seed_index: &BTreeMap<SeedIdV1, &VerifiedSemanticSeedV1>,
    predecessor_seeds: &BTreeSet<SeedIdV1>,
    memo: &mut BTreeMap<RawFamilyIdV1, DerivedRecursiveSupport>,
    visiting: &mut BTreeSet<RawFamilyIdV1>,
) -> Result<DerivedRecursiveSupport, FamilyWeakeningFailureV3> {
    let left_family = raw_index
        .get(left)
        .ok_or(FamilyWeakeningFailureV3::RecursiveSupportMismatch)?;
    let right_family = raw_index
        .get(right)
        .ok_or(FamilyWeakeningFailureV3::RecursiveSupportMismatch)?;
    let expected_rank = left_family
        .rank
        .max(right_family.rank)
        .checked_add(1)
        .ok_or(FamilyWeakeningFailureV3::RecursiveSupportMismatch)?;
    if left_family.rank >= family.rank
        || right_family.rank >= family.rank
        || family.rank != expected_rank
    {
        return Err(FamilyWeakeningFailureV3::RecursiveSupportMismatch);
    }
    let left = derive_recursive_support(
        left,
        raw_index,
        seed_index,
        predecessor_seeds,
        memo,
        visiting,
    )?;
    let right = derive_recursive_support(
        right,
        raw_index,
        seed_index,
        predecessor_seeds,
        memo,
        visiting,
    )?;
    Ok(DerivedRecursiveSupport {
        old: left.old && right.old,
        support: left.support.union(&right.support),
    })
}

fn verify_exact_raw_restriction(
    predecessor: &CarrierCertificateV1,
    successor: &CarrierCertificateV1,
    successor_old: &BTreeSet<RawFamilyIdV1>,
) -> Result<(), FamilyWeakeningFailureV3> {
    let predecessor_ids = predecessor
        .raw_families()
        .iter()
        .map(|family| family.id.clone())
        .collect::<BTreeSet<_>>();
    if predecessor_ids != *successor_old {
        return Err(FamilyWeakeningFailureV3::RawRestrictionMismatch);
    }
    let successor_index = successor
        .raw_families()
        .iter()
        .map(|family| (family.id.clone(), family))
        .collect::<BTreeMap<_, _>>();
    if successor_index.len() != successor.raw_families().len() {
        return Err(FamilyWeakeningFailureV3::RawRestrictionMismatch);
    }
    for family in predecessor.raw_families() {
        if successor_index.get(&family.id).copied() != Some(family) {
            return Err(FamilyWeakeningFailureV3::RawRestrictionMismatch);
        }
    }
    Ok(())
}

fn verify_quotient_transport(
    predecessor: &QuotientCertificateV1,
    successor: &QuotientCertificateV1,
    old_raw: &BTreeSet<RawFamilyIdV1>,
) -> Result<(Digest, Digest), FamilyWeakeningFailureV3> {
    if !predecessor.q3_registry_verified_empty()
        || !successor.q3_registry_verified_empty()
        || !predecessor.q3_edges().is_empty()
        || !successor.q3_edges().is_empty()
    {
        return Err(FamilyWeakeningFailureV3::Q3TransportMismatch);
    }

    let predecessor_q1 = indexed_q1(predecessor)?;
    let successor_q1 = indexed_q1(successor)?;
    let successor_old_q1 = successor_q1
        .into_iter()
        .filter(|(raw, _)| old_raw.contains(raw))
        .collect::<BTreeMap<_, _>>();
    if predecessor_q1.keys().cloned().collect::<BTreeSet<_>>() != *old_raw
        || predecessor_q1 != successor_old_q1
    {
        return Err(FamilyWeakeningFailureV3::Q1TransportMismatch);
    }
    let q1_transport_digest = Digest::of_canonical(
        "pen-semantic-audit/family-weakening-q1-transport/v3",
        &Q1TransportMaterial {
            predecessor: &predecessor_q1,
            successor_old: &successor_old_q1,
        },
    );

    let predecessor_q2 = canonical_disposition_multiset(predecessor.q2_pair_dispositions());
    let successor_old_dispositions = successor
        .q2_pair_dispositions()
        .iter()
        .filter(|disposition| {
            old_raw.contains(&disposition.left) && old_raw.contains(&disposition.right)
        })
        .cloned()
        .collect::<Vec<_>>();
    let successor_q2 = canonical_disposition_multiset(&successor_old_dispositions);
    if predecessor_q2 != successor_q2 {
        return Err(FamilyWeakeningFailureV3::Q2TransportMismatch);
    }
    let q2_transport_digest = Digest::of_canonical(
        "pen-semantic-audit/family-weakening-q2-transport/v3",
        &Q2TransportMaterial {
            predecessor: predecessor.q2_pair_dispositions(),
            successor_old: &successor_old_dispositions,
        },
    );
    Ok((q1_transport_digest, q2_transport_digest))
}

fn indexed_q1(
    quotient: &QuotientCertificateV1,
) -> Result<BTreeMap<RawFamilyIdV1, Digest>, FamilyWeakeningFailureV3> {
    if quotient.fixed_vertices().len() != quotient.q1_identity_edges().len() {
        return Err(FamilyWeakeningFailureV3::Q1TransportMismatch);
    }
    let mut indexed = BTreeMap::new();
    for (raw, edge) in quotient
        .fixed_vertices()
        .iter()
        .zip(quotient.q1_identity_edges())
    {
        let expected = Digest::of_canonical("pen-semantic-audit/q1-identity-edge/v1", raw);
        if edge != &expected || indexed.insert(raw.clone(), edge.clone()).is_some() {
            return Err(FamilyWeakeningFailureV3::Q1TransportMismatch);
        }
    }
    Ok(indexed)
}

fn canonical_disposition_multiset(dispositions: &[Q2PairDispositionV1]) -> BTreeMap<Vec<u8>, u64> {
    let mut multiset = BTreeMap::new();
    for disposition in dispositions {
        let mut encoder = CanonicalEncoder::new();
        disposition.encode_canonical(&mut encoder);
        *multiset.entry(encoder.as_bytes().to_vec()).or_default() += 1;
    }
    multiset
}

fn derive_legacy_class_map(
    predecessor: &QuotientCertificateV1,
    successor: &QuotientCertificateV1,
    old_raw: &BTreeSet<RawFamilyIdV1>,
) -> Result<Vec<(FamilyClassIdV1, FamilyClassIdV1)>, FamilyWeakeningFailureV3> {
    let predecessor_members = class_member_sets(predecessor.classes())?;
    let successor_members = class_member_sets(successor.classes())?;
    let predecessor_union = predecessor_members
        .values()
        .flat_map(|members| members.iter().cloned())
        .collect::<BTreeSet<_>>();
    if predecessor_union != *old_raw {
        return Err(FamilyWeakeningFailureV3::ClassMapNotWellDefined);
    }

    let predecessor_assignment = raw_class_assignment(predecessor)?;
    let successor_assignment = raw_class_assignment(successor)?;
    if predecessor_assignment
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        != *old_raw
        || !old_raw
            .iter()
            .all(|raw| successor_assignment.contains_key(raw))
    {
        return Err(FamilyWeakeningFailureV3::ClassMapNotWellDefined);
    }

    derive_injective_class_map(
        &predecessor_members,
        &successor_members,
        &predecessor_assignment,
        &successor_assignment,
        old_raw,
    )
}

fn derive_injective_class_map(
    predecessor_members: &BTreeMap<FamilyClassIdV1, BTreeSet<RawFamilyIdV1>>,
    successor_members: &BTreeMap<FamilyClassIdV1, BTreeSet<RawFamilyIdV1>>,
    predecessor_assignment: &BTreeMap<RawFamilyIdV1, FamilyClassIdV1>,
    successor_assignment: &BTreeMap<RawFamilyIdV1, FamilyClassIdV1>,
    old_raw: &BTreeSet<RawFamilyIdV1>,
) -> Result<Vec<(FamilyClassIdV1, FamilyClassIdV1)>, FamilyWeakeningFailureV3> {
    let mut result = Vec::with_capacity(predecessor_members.len());
    let mut image = BTreeMap::new();
    for (predecessor_class, members) in predecessor_members {
        if members
            .iter()
            .any(|raw| predecessor_assignment.get(raw) != Some(predecessor_class))
        {
            return Err(FamilyWeakeningFailureV3::ClassMapNotWellDefined);
        }
        let targets = members
            .iter()
            .filter_map(|raw| successor_assignment.get(raw).cloned())
            .collect::<BTreeSet<_>>();
        if targets.len() != 1 {
            return Err(FamilyWeakeningFailureV3::ClassMapNotWellDefined);
        }
        let target = targets
            .into_iter()
            .next()
            .ok_or(FamilyWeakeningFailureV3::ClassMapNotWellDefined)?;
        if image
            .insert(target.clone(), predecessor_class.clone())
            .is_some()
        {
            return Err(FamilyWeakeningFailureV3::FailedRetraction);
        }
        result.push((predecessor_class.clone(), target));
    }

    // Exact old-member intersections are the extensional restriction proof.
    // A nonempty intersection must equal one complete predecessor class; this
    // catches both an uncovered old vertex and a new-family bridge that
    // collapses two old components.
    for (successor_class, members) in successor_members {
        let old_intersection = members
            .intersection(old_raw)
            .cloned()
            .collect::<BTreeSet<_>>();
        match image.get(successor_class) {
            Some(predecessor_class) => {
                if predecessor_members.get(predecessor_class) != Some(&old_intersection) {
                    return Err(FamilyWeakeningFailureV3::FailedRetraction);
                }
            }
            None if !old_intersection.is_empty() => {
                return Err(FamilyWeakeningFailureV3::FailedRetraction);
            }
            None => {}
        }
    }
    result.sort_by(|left, right| left.0.cmp(&right.0));
    Ok(result)
}

fn class_member_sets(
    classes: &[FamilyClassV1],
) -> Result<BTreeMap<FamilyClassIdV1, BTreeSet<RawFamilyIdV1>>, FamilyWeakeningFailureV3> {
    let mut result = BTreeMap::new();
    let mut covered = BTreeSet::new();
    for class in classes {
        let members = class.members.iter().cloned().collect::<BTreeSet<_>>();
        if members.len() != class.members.len()
            || !members.contains(&class.representative)
            || members.iter().any(|member| !covered.insert(member.clone()))
            || result.insert(class.id.clone(), members).is_some()
        {
            return Err(FamilyWeakeningFailureV3::ClassMapNotWellDefined);
        }
    }
    Ok(result)
}

fn raw_class_assignment(
    quotient: &QuotientCertificateV1,
) -> Result<BTreeMap<RawFamilyIdV1, FamilyClassIdV1>, FamilyWeakeningFailureV3> {
    let classes = class_member_sets(quotient.classes())?;
    let mut result = BTreeMap::new();
    for assignment in quotient.raw_to_class() {
        if classes
            .get(&assignment.class)
            .is_none_or(|members| !members.contains(&assignment.raw))
            || result
                .insert(assignment.raw.clone(), assignment.class.clone())
                .is_some()
        {
            return Err(FamilyWeakeningFailureV3::ClassMapNotWellDefined);
        }
    }
    let members = classes
        .values()
        .flat_map(|members| members.iter().cloned())
        .collect::<BTreeSet<_>>();
    if result.keys().cloned().collect::<BTreeSet<_>>() != members {
        return Err(FamilyWeakeningFailureV3::ClassMapNotWellDefined);
    }
    Ok(result)
}

fn match_legacy_to_v3_classes(
    legacy_classes: &[FamilyClassV1],
    v3_classes: &[VerifiedFamilyClassV3],
    raw_ids: &BTreeMap<RawFamilyIdV1, NormalizedFamilyIdV3>,
) -> Result<BTreeMap<FamilyClassIdV1, FamilyClassIdV3>, FamilyWeakeningFailureV3> {
    let mut result = BTreeMap::new();
    let mut used = BTreeSet::new();
    for legacy in legacy_classes {
        let mut members = legacy
            .members
            .iter()
            .map(|member| {
                raw_ids
                    .get(member)
                    .cloned()
                    .ok_or(FamilyWeakeningFailureV3::V3IdentityMismatch)
            })
            .collect::<Result<Vec<_>, _>>()?;
        members.sort();
        let mut matches = v3_classes
            .iter()
            .filter(|class| class.members() == members.as_slice());
        let matched = matches
            .next()
            .ok_or(FamilyWeakeningFailureV3::V3IdentityMismatch)?;
        if matches.next().is_some()
            || !used.insert(matched.id().clone())
            || result
                .insert(legacy.id.clone(), matched.id().clone())
                .is_some()
        {
            return Err(FamilyWeakeningFailureV3::V3IdentityMismatch);
        }
    }
    if used.len() != v3_classes.len() {
        return Err(FamilyWeakeningFailureV3::V3IdentityMismatch);
    }
    Ok(result)
}

fn verify_retraction(
    weakening: &[FamilyWeakeningMapV3],
    restriction: &[FamilyRestrictionMapV3],
) -> Result<(), FamilyWeakeningFailureV3> {
    let restriction = restriction
        .iter()
        .map(|entry| (entry.source.clone(), entry.target.clone()))
        .collect::<BTreeMap<_, _>>();
    if restriction.len() != weakening.len()
        || weakening
            .iter()
            .any(|entry| restriction.get(&entry.target) != Some(&entry.source))
    {
        return Err(FamilyWeakeningFailureV3::FailedRetraction);
    }
    Ok(())
}

struct CanonicalSequence<'a, T>(&'a [T]);

impl<T: CanonicalEncode> CanonicalEncode for CanonicalSequence<'_, T> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

struct PredecessorSeedCensusMaterial<'a> {
    inventory: &'a Digest,
    predecessor_boundary: &'a Digest,
    new_event: &'a EventIdV1,
    seed_images: &'a [Q0SeedImageV3],
}

impl CanonicalEncode for PredecessorSeedCensusMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.predecessor_boundary.encode_canonical(encoder);
        self.new_event.encode_canonical(encoder);
        encoder.sequence(self.seed_images);
    }
}

struct RecursiveSupportMaterial<'a> {
    predecessor: &'a [RecursiveSupportEntryV3],
    successor: &'a [RecursiveSupportEntryV3],
}

impl CanonicalEncode for RecursiveSupportMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.predecessor);
        encoder.sequence(self.successor);
    }
}

struct Q1TransportMaterial<'a> {
    predecessor: &'a BTreeMap<RawFamilyIdV1, Digest>,
    successor_old: &'a BTreeMap<RawFamilyIdV1, Digest>,
}

impl CanonicalEncode for Q1TransportMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encode_raw_digest_map(encoder, self.predecessor);
        encode_raw_digest_map(encoder, self.successor_old);
    }
}

fn encode_raw_digest_map(encoder: &mut CanonicalEncoder, map: &BTreeMap<RawFamilyIdV1, Digest>) {
    encoder.u64(map.len() as u64);
    for (raw, digest) in map {
        raw.encode_canonical(encoder);
        digest.encode_canonical(encoder);
    }
}

struct Q2TransportMaterial<'a> {
    predecessor: &'a [Q2PairDispositionV1],
    successor_old: &'a [Q2PairDispositionV1],
}

impl CanonicalEncode for Q2TransportMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.predecessor);
        encoder.sequence(self.successor_old);
    }
}

struct ClassMapMaterial<'a> {
    weakening: &'a [FamilyWeakeningMapV3],
    restriction: &'a [FamilyRestrictionMapV3],
    image: &'a [FamilyClassIdV3],
}

impl CanonicalEncode for ClassMapMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.weakening);
        encoder.sequence(self.restriction);
        encoder.sequence(self.image);
    }
}

struct PredecessorQuotientMaterial<'a> {
    carrier: &'a Digest,
    quotient: &'a QuotientCertificateV1,
    classes: &'a [VerifiedFamilyClassV3],
}

impl CanonicalEncode for PredecessorQuotientMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.carrier.encode_canonical(encoder);
        self.quotient.encode_canonical(encoder);
        encoder.sequence(self.classes);
    }
}

#[cfg(test)]
mod tests {
    use super::{FamilyWeakeningFailureV3, derive_injective_class_map};
    use crate::model::{FamilyClassIdV1, RawFamilyIdV1};
    use pen_kernel::Digest;
    use std::collections::{BTreeMap, BTreeSet};

    fn raw(label: &[u8]) -> RawFamilyIdV1 {
        RawFamilyIdV1(Digest::of_domain_bytes("family-weakening-test/raw", label))
    }

    fn class(label: &[u8]) -> FamilyClassIdV1 {
        FamilyClassIdV1(Digest::of_domain_bytes(
            "family-weakening-test/class",
            label,
        ))
    }

    #[test]
    fn exact_old_intersections_induce_an_injective_retraction() {
        let (r1, r2, r3, n1, n2) = (raw(b"r1"), raw(b"r2"), raw(b"r3"), raw(b"n1"), raw(b"n2"));
        let (pre_a, pre_b, post_x, post_y, post_z) = (
            class(b"pre-a"),
            class(b"pre-b"),
            class(b"post-x"),
            class(b"post-y"),
            class(b"post-z"),
        );
        let predecessor_members = BTreeMap::from([
            (pre_a.clone(), BTreeSet::from([r1.clone(), r2.clone()])),
            (pre_b.clone(), BTreeSet::from([r3.clone()])),
        ]);
        let successor_members = BTreeMap::from([
            (post_x.clone(), BTreeSet::from([r1.clone(), r2.clone(), n1])),
            (post_y.clone(), BTreeSet::from([r3.clone()])),
            (post_z, BTreeSet::from([n2])),
        ]);
        let predecessor_assignment = BTreeMap::from([
            (r1.clone(), pre_a.clone()),
            (r2.clone(), pre_a.clone()),
            (r3.clone(), pre_b.clone()),
        ]);
        let successor_assignment = BTreeMap::from([
            (r1.clone(), post_x.clone()),
            (r2.clone(), post_x.clone()),
            (r3.clone(), post_y.clone()),
        ]);
        let old = BTreeSet::from([r1, r2, r3]);

        let mapping = derive_injective_class_map(
            &predecessor_members,
            &successor_members,
            &predecessor_assignment,
            &successor_assignment,
            &old,
        )
        .expect("exact intersections should mint the class injection");
        assert_eq!(
            mapping.into_iter().collect::<BTreeMap<_, _>>(),
            BTreeMap::from([(pre_a, post_x), (pre_b, post_y)])
        );
    }

    #[test]
    fn split_predecessor_class_fails_before_minting() {
        let (r1, r2) = (raw(b"split-r1"), raw(b"split-r2"));
        let (pre, post_x, post_y) = (
            class(b"split-pre"),
            class(b"split-post-x"),
            class(b"split-post-y"),
        );
        let predecessor_members =
            BTreeMap::from([(pre.clone(), BTreeSet::from([r1.clone(), r2.clone()]))]);
        let successor_members = BTreeMap::from([
            (post_x.clone(), BTreeSet::from([r1.clone()])),
            (post_y.clone(), BTreeSet::from([r2.clone()])),
        ]);
        let predecessor_assignment = BTreeMap::from([(r1.clone(), pre.clone()), (r2.clone(), pre)]);
        let successor_assignment = BTreeMap::from([(r1.clone(), post_x), (r2.clone(), post_y)]);

        assert!(matches!(
            derive_injective_class_map(
                &predecessor_members,
                &successor_members,
                &predecessor_assignment,
                &successor_assignment,
                &BTreeSet::from([r1, r2]),
            ),
            Err(FamilyWeakeningFailureV3::ClassMapNotWellDefined)
        ));
    }

    #[test]
    fn successor_bridge_between_old_classes_fails_retraction() {
        let (r1, r2, new) = (raw(b"bridge-r1"), raw(b"bridge-r2"), raw(b"bridge-new"));
        let (pre_a, pre_b, post) = (
            class(b"bridge-pre-a"),
            class(b"bridge-pre-b"),
            class(b"bridge-post"),
        );
        let predecessor_members = BTreeMap::from([
            (pre_a.clone(), BTreeSet::from([r1.clone()])),
            (pre_b.clone(), BTreeSet::from([r2.clone()])),
        ]);
        let successor_members =
            BTreeMap::from([(post.clone(), BTreeSet::from([r1.clone(), r2.clone(), new]))]);
        let predecessor_assignment = BTreeMap::from([(r1.clone(), pre_a), (r2.clone(), pre_b)]);
        let successor_assignment = BTreeMap::from([(r1.clone(), post.clone()), (r2.clone(), post)]);

        assert!(matches!(
            derive_injective_class_map(
                &predecessor_members,
                &successor_members,
                &predecessor_assignment,
                &successor_assignment,
                &BTreeSet::from([r1, r2]),
            ),
            Err(FamilyWeakeningFailureV3::FailedRetraction)
        ));
    }
}
