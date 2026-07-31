//! Native rank-0/1/2 V3 carrier, carrier-derived root inventory, and the
//! carrier-projected canonical subject bundle (Phase I).
//!
//! The stage after the Phase H typing metatheory in the registered V3
//! authority order is `NativeRankInductiveCarrier`. This module discharges
//! it by the exact route the V2 gate documented: the registered rank-0/1/2
//! carrier enumeration is run over seed wires derived **natively** from the
//! verified public inventory — never from a caller-supplied seed list — and
//! an exact one-to-one correspondence is proven between the enumeration's
//! verified seeds and the demand-neutral V3 seed census. The carrier
//! capability additionally requires the direct construction-substitution
//! census for every rank-positive family and the two Phase H capabilities
//! (production refinement and typing metatheory), honoring the authority
//! ordering `Refinement -> Carrier` and `Metatheory -> Carrier`.
//!
//! From the carrier this module derives:
//!
//! - the complete root inventory: every family judgment, kernel-replayed
//!   and deduplicated, as typed-occurrence root requests — equation
//!   judgments (equation seeds and generic equation actions) contribute
//!   equation roots whose right sides are the reducts; and
//! - one canonical subject bundle: the carrier's exact wire projection
//!   (family payloads in enumeration order, fresh-rule schemas derived
//!   from fresh-shaped inventory equations), built through the Phase G
//!   canonical builder, decoded back, compared section-by-section against
//!   the derived projection, and replayed through the generic
//!   accepted-bundle checker (the unchanged-kernel replay).
//!
//! Everything here is fail-closed: a caller cannot supply seeds, roots,
//! payloads, or digests, and no capability in this module can be minted
//! from an empty carrier.

use crate::carrier::{PreQ0RawCarrierCertificateV1, enumerate_pre_q0_raw_families_v1};
use crate::construction_substitution::{
    VerifiedConstructionSubstitutionCensusV2, verify_construction_substitution_census_v2,
};
use crate::inventory::VerifiedPublicAuditInventoryV1;
use crate::inventory_compatibility::VerifiedPublicInventoryCompatibilityV2;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, VerifiedSemanticAuditManifestV1,
    VerifiedSemanticAuditManifestV2, VerifiedSemanticAuditManifestV3,
    proposed_semantic_audit_lambda_unit_manifest_v1,
    proposed_semantic_audit_lambda_unit_manifest_v2,
    proposed_semantic_audit_lambda_unit_manifest_v3,
};
use crate::model::{
    EquationIdV1, FamilyConstructorV1, GenericJudgmentV1, LocalRoleV1, RawFamilyIdV1, RawFamilyV1,
    SeedIdV1, SemanticSchemaSeedV1, HeadPresentationV1, PublicHeadSeedV1, PublicEquationSeedV1,
    SourceNormalizedJudgmentV1, semantic_seed_id,
};
use crate::production_inventory_bridge::VerifiedProductionInventoryBridgeV1;
use crate::production_refinement::VerifiedGlobalSlotTableV1;
use crate::production_refinement_theorem::{
    VerifiedLambdaUnitProductionRefinementV1, VerifiedProductionSynthesisProtocolIdentityV2,
    VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
};
use crate::production_refinement_wire_authority::VerifiedCanonicalProductionBundleV1;
use crate::production_wire_builder::{ProductionBundlePayloadV1, build_canonical_production_bundle_v1};
use crate::production_wire_replay::replay_production_bundle_v1;
use crate::production_wire_slots::{digest_wire_id, term_to_wire_v1};
use crate::semantic_authority::VerifiedPublicClauseCensusV1;
use crate::semantic_authority_v3::VerifiedSemanticSeedBaseCensusV3;
use crate::typed_occurrence::TypedOccurrenceRootRequestV1;
use crate::typing_metatheory::VerifiedLambdaUnitTypingMetatheoryV1;
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, Kernel, KernelError,
    OpenJudgment, ResourceKind, Term, VerifiedSignature,
};
use pen_production_wire::{
    FamilyJudgmentWireV1, FamilyPayloadWireV1, FreshRuleSchemaWireV1, ProductionContextWireV1,
    SeedSourceWireV1, decode_bundle_v1,
};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

pub const NATIVE_CARRIER_SCHEMA_VERSION_V3: u16 = 1;

/// Fail-closed failures of the Phase I constructors. No variant carries
/// partial authority and none can be bypassed by a caller-supplied seed
/// list, root list, payload, digest, Boolean, or tag.
#[derive(Clone, Debug)]
pub enum NativeCarrierFailureV3 {
    ExactManifestIdentityMismatch,
    CarrierManifestSurfaceMismatch,
    ChainBindingMismatch,
    SeedCensusBindingMismatch,
    ProductionAuthorityBindingMismatch,
    NonEmptyQ3Registry,
    EmptyCarrier,
    Enumeration(AuditUnknownReason),
    EnumerationOutsideFragment(OutsideFragmentReason),
    SubstitutionCensus(AuditUnknownReason),
    SeedCorrespondenceMismatch,
    CarrierBindingMismatch,
    RootReplay(AuditUnknownReason),
    RootInventoryMismatch,
    EmptyRootInventory,
    UnsupportedCarrierProjection,
    EquationNotFreshShaped,
    SubjectBundleBuild(String),
    SubjectBundleDecode,
    SubjectBundleProjectionMismatch,
    SubjectBundleReplay(String),
}

impl std::fmt::Display for NativeCarrierFailureV3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactManifestIdentityMismatch => formatter.write_str(
                "the supplied manifests are not the exact registered lambda/unit V1/V2/V3 proposals",
            ),
            Self::CarrierManifestSurfaceMismatch => formatter.write_str(
                "the V1 and V3 manifests disagree on the carrier-relevant surface",
            ),
            Self::ChainBindingMismatch => formatter.write_str(
                "the inventory, compatibility, and clause census are not one verified chain",
            ),
            Self::SeedCensusBindingMismatch => formatter.write_str(
                "the V3 seed census is not bound to the presented manifest and inventory chain",
            ),
            Self::ProductionAuthorityBindingMismatch => formatter.write_str(
                "the production refinement or typing metatheory is not bound to the exact V3 manifest and kernel",
            ),
            Self::NonEmptyQ3Registry => {
                formatter.write_str("the inventory Q3 registry is not verified empty")
            }
            Self::EmptyCarrier => formatter.write_str(
                "the inventory yields no seeds; an empty carrier cannot carry completeness authority",
            ),
            Self::Enumeration(reason) => {
                write!(formatter, "carrier enumeration failed: {reason:?}")
            }
            Self::EnumerationOutsideFragment(reason) => {
                write!(formatter, "carrier enumeration left the fragment: {reason:?}")
            }
            Self::SubstitutionCensus(reason) => {
                write!(formatter, "construction-substitution census failed: {reason:?}")
            }
            Self::SeedCorrespondenceMismatch => formatter.write_str(
                "the enumeration seeds and the V3 seed census are not in exact one-to-one correspondence",
            ),
            Self::CarrierBindingMismatch => formatter.write_str(
                "the supplied capability is not bound to the presented carrier",
            ),
            Self::RootReplay(reason) => {
                write!(formatter, "root kernel replay failed: {reason:?}")
            }
            Self::RootInventoryMismatch => formatter.write_str(
                "the root inventory does not match the carrier it claims to cover",
            ),
            Self::EmptyRootInventory => formatter.write_str(
                "the carrier yields no roots; an empty root inventory cannot carry completeness authority",
            ),
            Self::UnsupportedCarrierProjection => formatter.write_str(
                "a carrier family cannot be projected onto the canonical wire",
            ),
            Self::EquationNotFreshShaped => formatter.write_str(
                "an inventory equation is not an exact fresh computation rule and cannot enter the wire equation registry",
            ),
            Self::SubjectBundleBuild(error) => {
                write!(formatter, "subject bundle construction failed: {error}")
            }
            Self::SubjectBundleDecode => {
                formatter.write_str("the subject bundle bytes did not decode")
            }
            Self::SubjectBundleProjectionMismatch => formatter.write_str(
                "the decoded subject bundle does not equal the carrier's exact wire projection",
            ),
            Self::SubjectBundleReplay(error) => {
                write!(formatter, "generic accepted-bundle checker rejected the subject bundle: {error}")
            }
        }
    }
}

impl std::error::Error for NativeCarrierFailureV3 {}

impl NativeCarrierFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::EnumerationOutsideFragment(reason) => AuditDecision::OutsideFragment(reason),
            Self::Enumeration(reason)
            | Self::SubstitutionCensus(reason)
            | Self::RootReplay(reason) => AuditDecision::Unknown(reason),
            Self::ExactManifestIdentityMismatch
            | Self::CarrierManifestSurfaceMismatch
            | Self::ChainBindingMismatch
            | Self::SeedCensusBindingMismatch
            | Self::ProductionAuthorityBindingMismatch => {
                AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
            }
            Self::NonEmptyQ3Registry => {
                AuditDecision::OutsideFragment(OutsideFragmentReason::NonEmptyQ3Registry)
            }
            Self::EmptyCarrier | Self::EmptyRootInventory => {
                AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
            }
            Self::SeedCorrespondenceMismatch
            | Self::CarrierBindingMismatch
            | Self::RootInventoryMismatch
            | Self::SubjectBundleProjectionMismatch => {
                AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
            }
            Self::UnsupportedCarrierProjection
            | Self::EquationNotFreshShaped
            | Self::SubjectBundleBuild(_)
            | Self::SubjectBundleDecode
            | Self::SubjectBundleReplay(_) => {
                AuditDecision::Unknown(AuditUnknownReason::MalformedInput)
            }
        }
    }
}

/// The native rank-0/1/2 V3 carrier capability.
///
/// Fields are private, there is no `Deserialize` implementation, and the
/// only constructor derives every seed from the verified public inventory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedNativeRankInductiveCarrierV3 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    v1_manifest_digest: Digest,
    v2_manifest_digest: Digest,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    inventory_digest: Digest,
    inventory_compatibility_digest: Digest,
    public_clause_census_digest: Digest,
    seed_census_digest: Digest,
    production_refinement_digest: Digest,
    typing_metatheory_digest: Digest,
    carrier: PreQ0RawCarrierCertificateV1,
    substitution_census: VerifiedConstructionSubstitutionCensusV2,
    seed_correspondence_digest: Digest,
    equation_registry: Arc<[(SeedIdV1, EquationIdV1, pen_kernel::GlobalId)]>,
    head_registry: Arc<[(SeedIdV1, pen_kernel::GlobalId)]>,
    digest: Digest,
}

impl VerifiedNativeRankInductiveCarrierV3 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn seed_census_digest(&self) -> &Digest {
        &self.seed_census_digest
    }

    pub fn production_refinement_digest(&self) -> &Digest {
        &self.production_refinement_digest
    }

    pub fn typing_metatheory_digest(&self) -> &Digest {
        &self.typing_metatheory_digest
    }

    pub fn families(&self) -> &[RawFamilyV1] {
        self.carrier.raw_families()
    }

    pub fn carrier_certificate_digest(&self) -> &Digest {
        self.carrier.digest()
    }

    pub fn substitution_census_digest(&self) -> &Digest {
        self.substitution_census.digest()
    }

    pub fn seed_correspondence_digest(&self) -> &Digest {
        &self.seed_correspondence_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    fn equation_for_seed(&self, seed: &SeedIdV1) -> Option<(&EquationIdV1, &pen_kernel::GlobalId)> {
        self.equation_registry
            .iter()
            .find(|(id, _, _)| id == seed)
            .map(|(_, equation, owner)| (equation, owner))
    }

    fn head_for_seed(&self, seed: &SeedIdV1) -> Option<&pen_kernel::GlobalId> {
        self.head_registry
            .iter()
            .find(|(id, _)| id == seed)
            .map(|(_, head)| head)
    }
}

impl CanonicalEncode for VerifiedNativeRankInductiveCarrierV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.v1_manifest_digest.encode_canonical(encoder);
        self.v2_manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.inventory_digest.encode_canonical(encoder);
        self.inventory_compatibility_digest.encode_canonical(encoder);
        self.public_clause_census_digest.encode_canonical(encoder);
        self.seed_census_digest.encode_canonical(encoder);
        self.production_refinement_digest.encode_canonical(encoder);
        self.typing_metatheory_digest.encode_canonical(encoder);
        self.carrier.encode_canonical(encoder);
        self.carrier.digest().encode_canonical(encoder);
        self.substitution_census.encode_canonical(encoder);
        self.seed_correspondence_digest.encode_canonical(encoder);
        encoder.u64(self.equation_registry.len() as u64);
        for (seed, equation, owner) in self.equation_registry.iter() {
            seed.encode_canonical(encoder);
            equation.encode_canonical(encoder);
            owner.encode_canonical(encoder);
        }
        encoder.u64(self.head_registry.len() as u64);
        for (seed, head) in self.head_registry.iter() {
            seed.encode_canonical(encoder);
            head.encode_canonical(encoder);
        }
    }
}

/// Derive the exact seed wires from the verified inventory: one public-head
/// seed per inventory declaration and one public-equation seed per inventory
/// equation, with judgments, supports, roles, and lineage taken from the
/// verified inventory and clause census. Callers cannot influence this list.
#[allow(clippy::type_complexity)]
fn derive_carrier_seed_wires_v3(
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
) -> Result<
    (
        Vec<SemanticSchemaSeedV1>,
        Vec<(SeedIdV1, pen_kernel::GlobalId)>,
        Vec<(SeedIdV1, EquationIdV1, pen_kernel::GlobalId)>,
    ),
    NativeCarrierFailureV3,
> {
    let mut seeds = Vec::new();
    let mut head_registry = Vec::new();
    let mut equation_registry = Vec::new();

    for declaration in inventory.declarations() {
        let clause_id = public_clauses
            .clause_for_declaration(declaration.declaration())
            .ok_or(NativeCarrierFailureV3::ChainBindingMismatch)?;
        let clause = public_clauses
            .clauses()
            .iter()
            .find(|candidate| candidate.id() == clause_id)
            .ok_or(NativeCarrierFailureV3::ChainBindingMismatch)?;
        let source_judgment = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Global {
                id: declaration.declaration().clone(),
            },
            ty: declaration.source().ty.clone(),
        };
        // Presentation is derived from the verified declaration body; it is
        // not part of kernel truth, only of the seed wire, and both values
        // are accepted by the seed verifier without an alias claim.
        let presentation = if declaration.source().body.is_some() {
            HeadPresentationV1::TransparentDefinition
        } else {
            HeadPresentationV1::Opaque
        };
        let wire = SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
            declaration: declaration.declaration().clone(),
            origin_event: declaration.origin().clone(),
            judgment: SourceNormalizedJudgmentV1 {
                source_identity: declaration.source_identity().clone(),
                source: source_judgment.clone(),
                claimed_normalized: source_judgment,
            },
            presentation,
            claimed_role: LocalRoleV1::KernelHead,
            public_support: clause.public_support().clone(),
            source_clause: Some(clause.id().clone()),
        });
        head_registry.push((semantic_seed_id(&wire), declaration.declaration().clone()));
        seeds.push(wire);
    }

    for equation in inventory.equations() {
        let clause_id = public_clauses
            .clause_for_equation(equation.equation())
            .ok_or(NativeCarrierFailureV3::ChainBindingMismatch)?;
        let clause = public_clauses
            .clauses()
            .iter()
            .find(|candidate| candidate.id() == clause_id)
            .ok_or(NativeCarrierFailureV3::ChainBindingMismatch)?;
        let wire = SemanticSchemaSeedV1::PublicEquation(PublicEquationSeedV1 {
            equation: equation.equation().clone(),
            owner_head: equation.owner_head().clone(),
            origin_event: equation.origin().clone(),
            judgment: SourceNormalizedJudgmentV1 {
                source_identity: equation.source_identity().clone(),
                source: equation.source().clone(),
                claimed_normalized: equation.source().clone(),
            },
            claimed_role: LocalRoleV1::Coherence,
            public_support: clause.public_support().clone(),
            source_clause: Some(clause.id().clone()),
            demand_anchor: None,
        });
        equation_registry.push((
            semantic_seed_id(&wire),
            equation.equation().clone(),
            equation.owner_head().clone(),
        ));
        seeds.push(wire);
    }

    Ok((seeds, head_registry, equation_registry))
}

fn verify_exact_registered_manifests(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
) -> Result<(), NativeCarrierFailureV3> {
    if v1_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v1()
        || v2_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v2()
        || v3_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v3()
    {
        return Err(NativeCarrierFailureV3::ExactManifestIdentityMismatch);
    }
    let v1 = v1_manifest.manifest();
    let v3 = v3_manifest.manifest();
    if v1.universe_levels != v3.universe_levels
        || v1.maximum_rank != v3.maximum_rank
        || v1.supported_seed_kinds != v3.supported_seed_kinds
        || v1.ordered_derivation_rules != v3.ordered_derivation_rules
        || v1.context_carrier_rules != v3.context_carrier_rules
        || v1.maximum_seeds != v3.maximum_seeds
        || v1.maximum_context_entries != v3.maximum_context_entries
        || v1.maximum_raw_derivations != v3.maximum_raw_derivations
        || v1.maximum_tuple_dispositions != v3.maximum_tuple_dispositions
    {
        return Err(NativeCarrierFailureV3::CarrierManifestSurfaceMismatch);
    }
    Ok(())
}

/// Construct the native rank-0/1/2 V3 carrier.
///
/// Every seed comes from the verified inventory; the registered rank-2
/// closure is exhausted by the preserved enumeration engine; the direct
/// construction-substitution census is reconstructed for every
/// rank-positive family; the enumeration seeds are matched one-to-one
/// against the demand-neutral V3 seed census; and the Phase H production
/// refinement and typing metatheory capabilities are required and bound.
#[allow(clippy::too_many_arguments)]
pub fn diagnose_native_rank_inductive_carrier_v3(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory_compatibility: &VerifiedPublicInventoryCompatibilityV2,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    seed_census: &VerifiedSemanticSeedBaseCensusV3,
    production_refinement: &VerifiedLambdaUnitProductionRefinementV1,
    typing_metatheory: &VerifiedLambdaUnitTypingMetatheoryV1,
) -> Result<VerifiedNativeRankInductiveCarrierV3, NativeCarrierFailureV3> {
    verify_exact_registered_manifests(v1_manifest, v2_manifest, v3_manifest)?;

    // One verified inventory chain (the same bindings the seed census
    // requires, restated here so this capability cannot outlive them).
    if inventory_compatibility.v1_manifest_digest() != inventory.manifest_digest()
        || inventory_compatibility.inventory_digest() != inventory.digest()
        || inventory_compatibility.v2_manifest_digest() != v2_manifest.candidate_digest()
        || public_clauses.inventory_digest() != inventory.digest()
    {
        return Err(NativeCarrierFailureV3::ChainBindingMismatch);
    }
    if seed_census.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || seed_census.inventory_compatibility_digest() != inventory_compatibility.digest()
        || seed_census.inventory_digest() != inventory.digest()
        || seed_census.public_clause_census_digest() != public_clauses.digest()
    {
        return Err(NativeCarrierFailureV3::SeedCensusBindingMismatch);
    }
    if production_refinement.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || typing_metatheory.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || typing_metatheory.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
    {
        return Err(NativeCarrierFailureV3::ProductionAuthorityBindingMismatch);
    }
    if !inventory.q3_registry().is_empty() {
        return Err(NativeCarrierFailureV3::NonEmptyQ3Registry);
    }

    let signature = inventory.successor_boundary();
    let (seed_wires, head_registry, equation_registry) =
        derive_carrier_seed_wires_v3(inventory, public_clauses)?;
    if seed_wires.is_empty() {
        return Err(NativeCarrierFailureV3::EmptyCarrier);
    }

    let carrier =
        match enumerate_pre_q0_raw_families_v1(kernel, signature, v1_manifest, &seed_wires, &[]) {
            AuditDecision::Proven(carrier) => carrier,
            AuditDecision::Unknown(reason) => {
                return Err(NativeCarrierFailureV3::Enumeration(reason));
            }
            AuditDecision::OutsideFragment(reason) => {
                return Err(NativeCarrierFailureV3::EnumerationOutsideFragment(reason));
            }
        };
    let substitution_census = match verify_construction_substitution_census_v2(
        kernel,
        signature,
        v1_manifest,
        v2_manifest,
        &carrier,
    ) {
        AuditDecision::Proven(census) => census,
        AuditDecision::Unknown(reason) => {
            return Err(NativeCarrierFailureV3::SubstitutionCensus(reason));
        }
        AuditDecision::OutsideFragment(_) => {
            return Err(NativeCarrierFailureV3::SubstitutionCensus(
                AuditUnknownReason::MalformedInput,
            ));
        }
    };

    // Exact one-to-one seed correspondence: every enumeration seed matches
    // exactly one demand-neutral V3 census seed on subject, source
    // judgment, origin, structural support, and role — and vice versa.
    let carrier_seeds = carrier.verified_seeds();
    if carrier_seeds.len() != seed_census.seeds().len() {
        return Err(NativeCarrierFailureV3::SeedCorrespondenceMismatch);
    }
    let mut matched_census_seeds = BTreeSet::new();
    let mut correspondence = Vec::with_capacity(carrier_seeds.len());
    for carrier_seed in carrier_seeds {
        let census_seed = seed_census
            .seeds()
            .iter()
            .find(|candidate| seeds_correspond(carrier_seed.seed(), candidate))
            .ok_or(NativeCarrierFailureV3::SeedCorrespondenceMismatch)?;
        if carrier_seed.source_judgment() != census_seed.source_judgment()
            || carrier_seed.derived_role() != census_seed.local_role()
        {
            return Err(NativeCarrierFailureV3::SeedCorrespondenceMismatch);
        }
        if !matched_census_seeds.insert(census_seed.id().clone()) {
            return Err(NativeCarrierFailureV3::SeedCorrespondenceMismatch);
        }
        correspondence.push((carrier_seed.id().clone(), census_seed.id().clone()));
    }
    if matched_census_seeds.len() != seed_census.seeds().len() {
        return Err(NativeCarrierFailureV3::SeedCorrespondenceMismatch);
    }
    let seed_correspondence_digest = Digest::of_canonical(
        "pen-semantic-audit/native-carrier-seed-correspondence/v3",
        &SeedCorrespondenceMaterialV3 {
            carrier_digest: carrier.digest(),
            seed_census_digest: seed_census.digest(),
            pairs: &correspondence,
        },
    );

    let mut verified = VerifiedNativeRankInductiveCarrierV3 {
        schema_version: NATIVE_CARRIER_SCHEMA_VERSION_V3,
        semantic_manifest_digest: v3_manifest.candidate_digest().clone(),
        v1_manifest_digest: v1_manifest.candidate_digest().clone(),
        v2_manifest_digest: v2_manifest.candidate_digest().clone(),
        signature_digest: signature.digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        inventory_digest: inventory.digest().clone(),
        inventory_compatibility_digest: inventory_compatibility.digest().clone(),
        public_clause_census_digest: public_clauses.digest().clone(),
        seed_census_digest: seed_census.digest().clone(),
        production_refinement_digest: production_refinement.digest().clone(),
        typing_metatheory_digest: typing_metatheory.digest().clone(),
        carrier,
        substitution_census,
        seed_correspondence_digest,
        equation_registry: Arc::from(equation_registry.into_boxed_slice()),
        head_registry: Arc::from(head_registry.into_boxed_slice()),
        digest: Digest::of_bytes(b"pending native rank-inductive carrier v3"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-native-rank-inductive-carrier/v3",
        &verified,
    );
    Ok(verified)
}

#[allow(clippy::too_many_arguments)]
pub fn verify_native_rank_inductive_carrier_v3(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory_compatibility: &VerifiedPublicInventoryCompatibilityV2,
    inventory: &VerifiedPublicAuditInventoryV1,
    public_clauses: &VerifiedPublicClauseCensusV1,
    seed_census: &VerifiedSemanticSeedBaseCensusV3,
    production_refinement: &VerifiedLambdaUnitProductionRefinementV1,
    typing_metatheory: &VerifiedLambdaUnitTypingMetatheoryV1,
) -> AuditDecision<VerifiedNativeRankInductiveCarrierV3> {
    match diagnose_native_rank_inductive_carrier_v3(
        v1_manifest,
        v2_manifest,
        v3_manifest,
        kernel,
        inventory_compatibility,
        inventory,
        public_clauses,
        seed_census,
        production_refinement,
        typing_metatheory,
    ) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(failure) => failure.into_decision(),
    }
}

fn seeds_correspond(
    carrier_seed: &SemanticSchemaSeedV1,
    census_seed: &crate::semantic_authority_v3::VerifiedPreQ0SemanticSeedV3,
) -> bool {
    use crate::semantic_authority_v3::PublicSemanticSeedSubjectV3;
    match (carrier_seed, census_seed.subject()) {
        (
            SemanticSchemaSeedV1::PublicHead(head),
            PublicSemanticSeedSubjectV3::Declaration { declaration, .. },
        ) => {
            &head.declaration == declaration
                && &head.origin_event == census_seed.origin_event()
                && head.public_support.events == *census_seed.structural_support().events()
                && head.public_support.declarations
                    == *census_seed.structural_support().declarations()
                && head.public_support.demand_outputs.is_empty()
        }
        (
            SemanticSchemaSeedV1::PublicEquation(equation),
            PublicSemanticSeedSubjectV3::Equation {
                equation: census_equation,
                owner_head,
                ..
            },
        ) => {
            &equation.equation == census_equation
                && &equation.owner_head == owner_head
                && &equation.origin_event == census_seed.origin_event()
                && equation.public_support.events == *census_seed.structural_support().events()
                && equation.public_support.declarations
                    == *census_seed.structural_support().declarations()
                && equation.public_support.demand_outputs.is_empty()
        }
        _ => false,
    }
}

struct SeedCorrespondenceMaterialV3<'a> {
    carrier_digest: &'a Digest,
    seed_census_digest: &'a Digest,
    pairs: &'a [(SeedIdV1, crate::semantic_authority_v3::SeedIdV3)],
}

impl CanonicalEncode for SeedCorrespondenceMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.carrier_digest.encode_canonical(encoder);
        self.seed_census_digest.encode_canonical(encoder);
        encoder.u64(self.pairs.len() as u64);
        for (carrier_seed, census_seed) in self.pairs {
            carrier_seed.encode_canonical(encoder);
            census_seed.encode_canonical(encoder);
        }
    }
}

/// Provenance of one root inside the carrier-derived root inventory.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CarrierRootKindV3 {
    /// The family judgment is a term judgment (seed head or application).
    Subject,
    /// The family judgment is an equation judgment; its left and right
    /// sides — for generic equation actions, the plugged source and
    /// reduct — are both covered by the equation root.
    EquationWithReducts,
}

impl CanonicalEncode for CarrierRootKindV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Subject => 0,
            Self::EquationWithReducts => 1,
        });
    }
}

/// One deduplicated, kernel-replayed root with its carrier provenance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CarrierRootV3 {
    request: TypedOccurrenceRootRequestV1,
    kind: CarrierRootKindV3,
    source_families: Vec<RawFamilyIdV1>,
}

impl CarrierRootV3 {
    pub fn request(&self) -> &TypedOccurrenceRootRequestV1 {
        &self.request
    }

    pub fn kind(&self) -> CarrierRootKindV3 {
        self.kind
    }

    pub fn source_families(&self) -> &[RawFamilyIdV1] {
        &self.source_families
    }
}

impl CanonicalEncode for CarrierRootV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.request.encode_canonical(encoder);
        self.kind.encode_canonical(encoder);
        encoder.u64(self.source_families.len() as u64);
        for family in &self.source_families {
            family.encode_canonical(encoder);
        }
    }
}

/// The complete carrier-derived root inventory: every family judgment of
/// the native carrier, kernel-replayed, deduplicated, and ordered. This is
/// the only lawful root source for the V3 typed-occurrence census; there
/// is no constructor from a caller root list.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCarrierRootInventoryV3 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    carrier_digest: Digest,
    roots: Arc<[CarrierRootV3]>,
    roots_digest: Digest,
    digest: Digest,
}

impl VerifiedCarrierRootInventoryV3 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn carrier_digest(&self) -> &Digest {
        &self.carrier_digest
    }

    pub fn roots(&self) -> &[CarrierRootV3] {
        &self.roots
    }

    pub fn requests(&self) -> Vec<TypedOccurrenceRootRequestV1> {
        self.roots.iter().map(|root| root.request.clone()).collect()
    }

    pub fn roots_digest(&self) -> &Digest {
        &self.roots_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedCarrierRootInventoryV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.carrier_digest.encode_canonical(encoder);
        encoder.sequence(&self.roots);
        self.roots_digest.encode_canonical(encoder);
    }
}

/// Derive the complete root inventory from the native carrier: one root
/// request per distinct kernel-replayed family judgment. Term judgments
/// become `HasType` roots; equation judgments (equation seeds and generic
/// equation actions) become `Equation` roots, so both equation sides —
/// including every action reduct — are inside the inventory.
pub fn diagnose_carrier_root_inventory_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    carrier: &VerifiedNativeRankInductiveCarrierV3,
) -> Result<VerifiedCarrierRootInventoryV3, NativeCarrierFailureV3> {
    if carrier.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || carrier.inventory_digest() != inventory.digest()
        || carrier.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
    {
        return Err(NativeCarrierFailureV3::CarrierBindingMismatch);
    }
    let signature = inventory.successor_boundary();
    if carrier.signature_digest() != signature.digest() {
        return Err(NativeCarrierFailureV3::CarrierBindingMismatch);
    }

    let mut deduped: BTreeMap<Vec<u8>, CarrierRootV3> = BTreeMap::new();
    for family in carrier.families() {
        let (request, kind) = match &family.generic_judgment {
            GenericJudgmentV1::Term { context, term, ty } => {
                let replay = replay_has_type(kernel, signature, context, term, ty)?;
                (
                    TypedOccurrenceRootRequestV1::HasType {
                        context: replay.0,
                        term: term.clone(),
                        ty: replay.1,
                    },
                    CarrierRootKindV3::Subject,
                )
            }
            GenericJudgmentV1::Equation {
                context,
                left,
                right,
                ty,
            } => {
                let left_replay = replay_has_type(kernel, signature, context, left, ty)?;
                let right_replay = replay_has_type(kernel, signature, context, right, ty)?;
                if left_replay != right_replay {
                    return Err(NativeCarrierFailureV3::RootReplay(
                        AuditUnknownReason::KernelCouldNotCertify,
                    ));
                }
                (
                    TypedOccurrenceRootRequestV1::Equation {
                        context: left_replay.0,
                        left: left.clone(),
                        right: right.clone(),
                        ty: left_replay.1,
                    },
                    CarrierRootKindV3::EquationWithReducts,
                )
            }
        };
        let mut key_encoder = CanonicalEncoder::new();
        request.encode_canonical(&mut key_encoder);
        let key = key_encoder.as_bytes().to_vec();
        match deduped.get_mut(&key) {
            Some(root) => {
                if root.kind != kind {
                    return Err(NativeCarrierFailureV3::RootInventoryMismatch);
                }
                root.source_families.push(family.id.clone());
            }
            None => {
                deduped.insert(
                    key,
                    CarrierRootV3 {
                        request,
                        kind,
                        source_families: vec![family.id.clone()],
                    },
                );
            }
        }
    }
    if deduped.is_empty() {
        return Err(NativeCarrierFailureV3::EmptyRootInventory);
    }
    let roots: Vec<CarrierRootV3> = deduped.into_values().collect();
    let roots_digest = Digest::of_canonical(
        "pen-semantic-audit/carrier-root-inventory-roots/v3",
        &CanonicalSlice(&roots),
    );
    let mut verified = VerifiedCarrierRootInventoryV3 {
        schema_version: NATIVE_CARRIER_SCHEMA_VERSION_V3,
        semantic_manifest_digest: v3_manifest.candidate_digest().clone(),
        signature_digest: signature.digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        carrier_digest: carrier.digest().clone(),
        roots: Arc::from(roots.into_boxed_slice()),
        roots_digest,
        digest: Digest::of_bytes(b"pending carrier root inventory v3"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-carrier-root-inventory/v3",
        &verified,
    );
    Ok(verified)
}

pub fn verify_carrier_root_inventory_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    carrier: &VerifiedNativeRankInductiveCarrierV3,
) -> AuditDecision<VerifiedCarrierRootInventoryV3> {
    match diagnose_carrier_root_inventory_v3(v3_manifest, kernel, inventory, carrier) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(failure) => failure.into_decision(),
    }
}

fn replay_has_type(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &DependentContext,
    term: &Term,
    ty: &Term,
) -> Result<(DependentContext, Term), NativeCarrierFailureV3> {
    let input = OpenJudgment::HasType {
        context: context.clone(),
        term: term.clone(),
        ty: ty.clone(),
    };
    let output = kernel
        .verify_open_judgment(signature, &input)
        .map_err(|error| NativeCarrierFailureV3::RootReplay(kernel_unknown(error)))?;
    let OpenJudgment::HasType { context, ty, .. } = output else {
        return Err(NativeCarrierFailureV3::RootReplay(
            AuditUnknownReason::KernelCouldNotCertify,
        ));
    };
    Ok((context, ty))
}

fn kernel_unknown(error: KernelError) -> AuditUnknownReason {
    match error {
        KernelError::ResourceExhausted(
            ResourceKind::Operations | ResourceKind::Depth | ResourceKind::Normalization,
        ) => AuditUnknownReason::ResourceExhausted,
        _ => AuditUnknownReason::KernelCouldNotCertify,
    }
}

struct CanonicalSlice<'a, T>(&'a [T]);

impl<T: CanonicalEncode> CanonicalEncode for CanonicalSlice<'_, T> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

/// Why one carrier family cannot enter the canonical wire. These are
/// registered wire asymmetries, not silent omissions: the family's roots
/// remain covered by the carrier-derived root inventory and the
/// typed-occurrence census, and the missing wire carrier is exactly the
/// Phase J rewrite authority the wire deferred by dropping
/// `hole_ordinal`/`context_witness`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WireInexpressibleReasonV3 {
    /// A generic equation action whose amalgamated judgment context
    /// differs from its source family's context; the wire's action
    /// payload has no context-witness slot to relate them.
    ContextGrowingEquationAction,
    /// An equation seed whose inventory equation is not an exact fresh
    /// computation rule; the wire equation registry holds only
    /// fresh-rule schemas.
    NonFreshEquation,
    /// A family referencing a wire-inexpressible family.
    ReferencesInexpressibleFamily,
}

impl CanonicalEncode for WireInexpressibleReasonV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::ContextGrowingEquationAction => 0,
            Self::NonFreshEquation => 1,
            Self::ReferencesInexpressibleFamily => 2,
        });
    }
}

/// The canonical subject bundle: the carrier's exact wire projection,
/// built through the capability-bound canonical builder, decoded back and
/// compared section-by-section, and accepted by the generic
/// accepted-bundle checker (the independent unchanged-kernel replay).
/// Wire-inexpressible families are recorded, never silently dropped.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCarrierSubjectBundleV3 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    signature_digest: Digest,
    carrier_digest: Digest,
    bundle: VerifiedCanonicalProductionBundleV1,
    family_projection_digest: Digest,
    fresh_projection_digest: Digest,
    wire_inexpressible: Arc<[(RawFamilyIdV1, WireInexpressibleReasonV3)]>,
    wire_inexpressible_digest: Digest,
    replay_kernel_protocol_digest: Digest,
    replay_synthesis_protocol_digest: Digest,
    digest: Digest,
}

impl VerifiedCarrierSubjectBundleV3 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn carrier_digest(&self) -> &Digest {
        &self.carrier_digest
    }

    pub fn bundle(&self) -> &VerifiedCanonicalProductionBundleV1 {
        &self.bundle
    }

    pub fn family_projection_digest(&self) -> &Digest {
        &self.family_projection_digest
    }

    pub fn fresh_projection_digest(&self) -> &Digest {
        &self.fresh_projection_digest
    }

    /// The registered wire-inexpressible families (context-growing
    /// generic equation actions and non-fresh equations, plus anything
    /// referencing them). Their roots stay covered by the root inventory
    /// and the typed-occurrence census.
    pub fn wire_inexpressible(&self) -> &[(RawFamilyIdV1, WireInexpressibleReasonV3)] {
        &self.wire_inexpressible
    }

    pub fn wire_inexpressible_digest(&self) -> &Digest {
        &self.wire_inexpressible_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedCarrierSubjectBundleV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.carrier_digest.encode_canonical(encoder);
        self.bundle.encode_canonical(encoder);
        self.family_projection_digest.encode_canonical(encoder);
        self.fresh_projection_digest.encode_canonical(encoder);
        encoder.u64(self.wire_inexpressible.len() as u64);
        for (family, reason) in self.wire_inexpressible.iter() {
            family.encode_canonical(encoder);
            reason.encode_canonical(encoder);
        }
        self.wire_inexpressible_digest.encode_canonical(encoder);
        self.replay_kernel_protocol_digest.encode_canonical(encoder);
        self.replay_synthesis_protocol_digest
            .encode_canonical(encoder);
    }
}

/// Build and check the canonical subject bundle for one native carrier.
///
/// The registered wire projection is:
///
/// - head seeds project to `Seed`/`PublicHead` payloads carrying their
///   exact judgment;
/// - equation seeds project to `Seed`/`PublicEquation` payloads judging
///   the equation's owner head at its exact declared type, and their
///   equations enter the wire equation registry as fresh-rule schemas —
///   which requires the inventory equation to be an exact fresh
///   computation rule (owner applied to pattern variables with the
///   constructor in scrutinee position, owner-free right side, bodyless
///   distinct owner and constructor); anything else fails closed;
/// - applications project to `GenericPublicApplication` payloads carrying
///   their exact judgment and component references; and
/// - generic equation actions project to `GenericEquationAction` payloads
///   whose subject is the right-side plug — the reduct — with the source
///   type preserved; the hole ordinal and context witness stay in the
///   carrier (the wire drops them; their rewrite-step relation is Phase J
///   authority).
#[allow(clippy::too_many_arguments)]
pub fn diagnose_carrier_subject_bundle_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    slots: &VerifiedGlobalSlotTableV1,
    inventory_bridge: &VerifiedProductionInventoryBridgeV1,
    delta_policy: &VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
    synthesis: &VerifiedProductionSynthesisProtocolIdentityV2,
    carrier: &VerifiedNativeRankInductiveCarrierV3,
) -> Result<VerifiedCarrierSubjectBundleV3, NativeCarrierFailureV3> {
    if carrier.semantic_manifest_digest() != v3_manifest.candidate_digest()
        || carrier.inventory_digest() != inventory.digest()
        || carrier.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
    {
        return Err(NativeCarrierFailureV3::CarrierBindingMismatch);
    }
    let signature = inventory.successor_boundary();
    if carrier.signature_digest() != signature.digest()
        || slots.signature_digest() != signature.digest()
    {
        return Err(NativeCarrierFailureV3::CarrierBindingMismatch);
    }

    let (fresh_rule_schemas, family_payloads, wire_inexpressible) =
        derive_wire_projection(signature, slots, carrier)?;
    let payload = ProductionBundlePayloadV1 {
        contexts: Vec::new(),
        conversions: Vec::new(),
        conversion_typing_supplements: Vec::new(),
        synthesis_codes: Vec::new(),
        fresh_rule_schemas: fresh_rule_schemas.clone(),
        family_payloads: family_payloads.clone(),
    };
    let bundle = build_canonical_production_bundle_v1(
        v3_manifest,
        signature,
        slots,
        inventory_bridge,
        delta_policy,
        synthesis,
        payload,
    )
    .map_err(|error| NativeCarrierFailureV3::SubjectBundleBuild(error.to_string()))?;

    // Independent decode; the decoded sections must equal the derived
    // projection exactly. This is the byte-visible statement that the
    // canonical subject bundle contains exactly the carrier's roots.
    let decoded = decode_bundle_v1(bundle.canonical_bytes())
        .map_err(|_| NativeCarrierFailureV3::SubjectBundleDecode)?;
    if decoded.family_payloads != family_payloads
        || decoded.fresh_rule_schemas != fresh_rule_schemas
        || !decoded.contexts.is_empty()
        || !decoded.conversions.is_empty()
        || !decoded.conversion_typing_supplements.is_empty()
        || !decoded.synthesis_codes.is_empty()
    {
        return Err(NativeCarrierFailureV3::SubjectBundleProjectionMismatch);
    }

    // The generic accepted-bundle checker: the independent replay through
    // the unchanged kernel (structural validation, canonical re-encoding,
    // and every family judgment through `Kernel::verify_open_judgment`).
    let replay_evidence = replay_production_bundle_v1(bundle.canonical_bytes())
        .map_err(|error| NativeCarrierFailureV3::SubjectBundleReplay(error.to_string()))?;
    if replay_evidence.replayed_bytes != bundle.canonical_bytes()
        || replay_evidence.kernel_protocol_digest != kernel.kernel_protocol_digest()
    {
        return Err(NativeCarrierFailureV3::SubjectBundleReplay(
            "replay identity mismatch".to_owned(),
        ));
    }

    let family_projection_digest = Digest::of_canonical(
        "pen-semantic-audit/carrier-subject-family-projection/v3",
        &WirePayloadProjectionMaterialV3 {
            carrier_digest: carrier.digest(),
            bundle_digest: bundle.bundle_digest(),
            payload_count: family_payloads.len() as u64,
        },
    );
    let fresh_projection_digest = Digest::of_canonical(
        "pen-semantic-audit/carrier-subject-fresh-projection/v3",
        &WirePayloadProjectionMaterialV3 {
            carrier_digest: carrier.digest(),
            bundle_digest: bundle.bundle_digest(),
            payload_count: fresh_rule_schemas.len() as u64,
        },
    );
    // Every carrier family is either a wire payload or a registered
    // inexpressible entry; the two partitions must cover the carrier
    // exactly.
    if family_payloads.len() + wire_inexpressible.len() != carrier.families().len() {
        return Err(NativeCarrierFailureV3::SubjectBundleProjectionMismatch);
    }
    let wire_inexpressible_digest = Digest::of_canonical(
        "pen-semantic-audit/carrier-subject-wire-inexpressible/v3",
        &WireInexpressibleMaterialV3 {
            carrier_digest: carrier.digest(),
            entries: &wire_inexpressible,
        },
    );
    let mut verified = VerifiedCarrierSubjectBundleV3 {
        schema_version: NATIVE_CARRIER_SCHEMA_VERSION_V3,
        semantic_manifest_digest: v3_manifest.candidate_digest().clone(),
        signature_digest: signature.digest().clone(),
        carrier_digest: carrier.digest().clone(),
        bundle,
        family_projection_digest,
        fresh_projection_digest,
        wire_inexpressible: Arc::from(wire_inexpressible.into_boxed_slice()),
        wire_inexpressible_digest,
        replay_kernel_protocol_digest: replay_evidence.kernel_protocol_digest,
        replay_synthesis_protocol_digest: replay_evidence.synthesis_protocol_digest,
        digest: Digest::of_bytes(b"pending carrier subject bundle v3"),
    };
    verified.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-carrier-subject-bundle/v3",
        &verified,
    );
    Ok(verified)
}

#[allow(clippy::too_many_arguments)]
pub fn verify_carrier_subject_bundle_v3(
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    slots: &VerifiedGlobalSlotTableV1,
    inventory_bridge: &VerifiedProductionInventoryBridgeV1,
    delta_policy: &VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
    synthesis: &VerifiedProductionSynthesisProtocolIdentityV2,
    carrier: &VerifiedNativeRankInductiveCarrierV3,
) -> AuditDecision<VerifiedCarrierSubjectBundleV3> {
    match diagnose_carrier_subject_bundle_v3(
        v3_manifest,
        kernel,
        inventory,
        slots,
        inventory_bridge,
        delta_policy,
        synthesis,
        carrier,
    ) {
        Ok(verified) => AuditDecision::Proven(verified),
        Err(failure) => failure.into_decision(),
    }
}

struct WirePayloadProjectionMaterialV3<'a> {
    carrier_digest: &'a Digest,
    bundle_digest: &'a Digest,
    payload_count: u64,
}

impl CanonicalEncode for WirePayloadProjectionMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.carrier_digest.encode_canonical(encoder);
        self.bundle_digest.encode_canonical(encoder);
        encoder.u64(self.payload_count);
    }
}

struct WireInexpressibleMaterialV3<'a> {
    carrier_digest: &'a Digest,
    entries: &'a [(RawFamilyIdV1, WireInexpressibleReasonV3)],
}

impl CanonicalEncode for WireInexpressibleMaterialV3<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.carrier_digest.encode_canonical(encoder);
        encoder.u64(self.entries.len() as u64);
        for (family, reason) in self.entries {
            family.encode_canonical(encoder);
            reason.encode_canonical(encoder);
        }
    }
}

fn wire_context(
    context: &DependentContext,
    slots: &VerifiedGlobalSlotTableV1,
) -> Result<ProductionContextWireV1, NativeCarrierFailureV3> {
    Ok(ProductionContextWireV1 {
        entries_oldest_first: context
            .0
            .iter()
            .map(|entry| term_to_wire_v1(entry, slots))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| NativeCarrierFailureV3::UnsupportedCarrierProjection)?,
    })
}

fn wire_term(
    term: &Term,
    slots: &VerifiedGlobalSlotTableV1,
) -> Result<pen_production_wire::WireTermV1, NativeCarrierFailureV3> {
    term_to_wire_v1(term, slots).map_err(|_| NativeCarrierFailureV3::UnsupportedCarrierProjection)
}

fn owner_slot_u32(
    id: &pen_kernel::GlobalId,
    slots: &VerifiedGlobalSlotTableV1,
) -> Result<u32, NativeCarrierFailureV3> {
    let slot = slots
        .slot_for_global(id)
        .ok_or(NativeCarrierFailureV3::UnsupportedCarrierProjection)?;
    u32::try_from(slot.ordinal()).map_err(|_| NativeCarrierFailureV3::UnsupportedCarrierProjection)
}

/// Split an application spine: `f a1 .. an` yields `(f, [a1, .., an])`.
fn split_spine(term: &Term) -> (&Term, Vec<&Term>) {
    let mut head = term;
    let mut arguments = Vec::new();
    while let Term::Apply { function, argument } = head {
        arguments.push(argument.as_ref());
        head = function.as_ref();
    }
    arguments.reverse();
    (head, arguments)
}

fn term_mentions_global(term: &Term, id: &pen_kernel::GlobalId) -> bool {
    match term {
        Term::Global { id: candidate } => candidate == id,
        Term::Pi { parameter, body } => {
            term_mentions_global(parameter, id) || term_mentions_global(body, id)
        }
        Term::Lambda {
            parameter_type,
            body,
        } => term_mentions_global(parameter_type, id) || term_mentions_global(body, id),
        Term::Apply { function, argument } => {
            term_mentions_global(function, id) || term_mentions_global(argument, id)
        }
        Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => false,
        Term::Sigma { parameter, body } => {
            term_mentions_global(parameter, id) || term_mentions_global(body, id)
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            term_mentions_global(sigma_type, id)
                || term_mentions_global(first, id)
                || term_mentions_global(second, id)
        }
        Term::First { pair } | Term::Second { pair } => term_mentions_global(pair, id),
    }
}

/// Derive the wire fresh-rule schema from one inventory equation judgment.
///
/// The judgment must be the exact fresh computation shape mirrored by the
/// wire validator: context of length `arity >= 2`, left side an
/// application spine `owner v_{s-1} .. v_0-pattern .. constructor` whose
/// head is the bodyless owner, whose first `scrutinee` arguments are the
/// pattern variables in the exact wire order, and whose scrutinee-position
/// argument is a distinct bodyless constructor global; the right side must
/// not mention the owner.
fn derive_fresh_schema(
    equation: &EquationIdV1,
    owner: &pen_kernel::GlobalId,
    judgment: &GenericJudgmentV1,
    signature: &VerifiedSignature,
    slots: &VerifiedGlobalSlotTableV1,
) -> Result<FreshRuleSchemaWireV1, NativeCarrierFailureV3> {
    let GenericJudgmentV1::Equation {
        context,
        left,
        right,
        ty,
    } = judgment
    else {
        return Err(NativeCarrierFailureV3::EquationNotFreshShaped);
    };
    let arity = context.0.len();
    if arity < 2 {
        return Err(NativeCarrierFailureV3::EquationNotFreshShaped);
    }
    let scrutinee = arity - 1;
    let (head, arguments) = split_spine(left);
    let Term::Global { id: head_id } = head else {
        return Err(NativeCarrierFailureV3::EquationNotFreshShaped);
    };
    if head_id != owner || arguments.len() != scrutinee + 1 {
        return Err(NativeCarrierFailureV3::EquationNotFreshShaped);
    }
    let Term::Global { id: constructor_id } = arguments[scrutinee] else {
        return Err(NativeCarrierFailureV3::EquationNotFreshShaped);
    };
    if constructor_id == owner {
        return Err(NativeCarrierFailureV3::EquationNotFreshShaped);
    }
    for (ordinal, argument) in arguments[..scrutinee].iter().enumerate() {
        if **argument
            != (Term::Var {
                index: (scrutinee - ordinal - 1) as u32,
            })
        {
            return Err(NativeCarrierFailureV3::EquationNotFreshShaped);
        }
    }
    if term_mentions_global(right, owner) {
        return Err(NativeCarrierFailureV3::EquationNotFreshShaped);
    }
    let bodyless = |id: &pen_kernel::GlobalId| {
        signature
            .declarations()
            .iter()
            .any(|declaration| &declaration.id == id && declaration.body.is_none())
    };
    if !bodyless(owner) || !bodyless(constructor_id) {
        return Err(NativeCarrierFailureV3::EquationNotFreshShaped);
    }
    Ok(FreshRuleSchemaWireV1 {
        equation_id: digest_wire_id(&equation.0)
            .map_err(|_| NativeCarrierFailureV3::UnsupportedCarrierProjection)?,
        owner_slot: owner_slot_u32(owner, slots)?,
        constructor_slot: owner_slot_u32(constructor_id, slots)?,
        parameter_context: wire_context(context, slots)?,
        left: wire_term(left, slots)?,
        right: wire_term(right, slots)?,
        ty: wire_term(ty, slots)?,
        scrutinee_ordinal: scrutinee as u32,
        arity: u16::try_from(arity)
            .map_err(|_| NativeCarrierFailureV3::UnsupportedCarrierProjection)?,
    })
}

/// The registered carrier-to-wire projection: fresh-rule schemas from the
/// carrier's fresh-shaped equation seeds and one family payload per
/// wire-expressible carrier family, in enumeration order. Families the
/// wire cannot carry are returned in the inexpressible census with their
/// exact reasons; nothing is silently dropped.
#[allow(clippy::type_complexity)]
fn derive_wire_projection(
    signature: &VerifiedSignature,
    slots: &VerifiedGlobalSlotTableV1,
    carrier: &VerifiedNativeRankInductiveCarrierV3,
) -> Result<
    (
        Vec<FreshRuleSchemaWireV1>,
        Vec<FamilyPayloadWireV1>,
        Vec<(RawFamilyIdV1, WireInexpressibleReasonV3)>,
    ),
    NativeCarrierFailureV3,
> {
    let families_by_id: BTreeMap<&RawFamilyIdV1, &RawFamilyV1> = carrier
        .families()
        .iter()
        .map(|family| (&family.id, family))
        .collect();
    let mut fresh_rule_schemas = Vec::new();
    let mut seen_equations = BTreeSet::new();
    let mut equation_seed_families: BTreeMap<RawFamilyIdV1, EquationIdV1> = BTreeMap::new();
    let mut family_payloads = Vec::with_capacity(carrier.families().len());
    let mut inexpressible: Vec<(RawFamilyIdV1, WireInexpressibleReasonV3)> = Vec::new();
    let mut inexpressible_ids: BTreeSet<RawFamilyIdV1> = BTreeSet::new();
    let skip =
        |family: &RawFamilyV1,
         reason: WireInexpressibleReasonV3,
         inexpressible: &mut Vec<(RawFamilyIdV1, WireInexpressibleReasonV3)>,
         inexpressible_ids: &mut BTreeSet<RawFamilyIdV1>| {
            inexpressible.push((family.id.clone(), reason));
            inexpressible_ids.insert(family.id.clone());
        };

    for family in carrier.families() {
        let family_id = digest_wire_id(&family.id.0)
            .map_err(|_| NativeCarrierFailureV3::UnsupportedCarrierProjection)?;
        let payload = match &family.constructor {
            FamilyConstructorV1::PublicHeadSeed { seed } => {
                let head = carrier
                    .head_for_seed(seed)
                    .ok_or(NativeCarrierFailureV3::UnsupportedCarrierProjection)?;
                let GenericJudgmentV1::Term { context, term, ty } = &family.generic_judgment
                else {
                    return Err(NativeCarrierFailureV3::UnsupportedCarrierProjection);
                };
                FamilyPayloadWireV1::Seed {
                    family_id,
                    source: SeedSourceWireV1::PublicHead {
                        owner_slot: owner_slot_u32(head, slots)?,
                    },
                    judgment: FamilyJudgmentWireV1 {
                        context: wire_context(context, slots)?,
                        subject: wire_term(term, slots)?,
                        ty: wire_term(ty, slots)?,
                    },
                }
            }
            FamilyConstructorV1::PublicEquationSeed { seed } => {
                let (equation, owner) = carrier
                    .equation_for_seed(seed)
                    .ok_or(NativeCarrierFailureV3::UnsupportedCarrierProjection)?;
                let schema = derive_fresh_schema(
                    equation,
                    owner,
                    &family.generic_judgment,
                    signature,
                    slots,
                );
                let schema = match schema {
                    Ok(schema) => schema,
                    Err(NativeCarrierFailureV3::EquationNotFreshShaped) => {
                        skip(
                            family,
                            WireInexpressibleReasonV3::NonFreshEquation,
                            &mut inexpressible,
                            &mut inexpressible_ids,
                        );
                        continue;
                    }
                    Err(other) => return Err(other),
                };
                if seen_equations.insert(equation.clone()) {
                    fresh_rule_schemas.push(schema);
                }
                equation_seed_families.insert(family.id.clone(), equation.clone());
                // The registered projection: the equation seed's payload
                // judges the equation's owner head at its exact declared
                // type in the empty context.
                let declared = signature
                    .declarations()
                    .iter()
                    .find(|declaration| &declaration.id == owner)
                    .ok_or(NativeCarrierFailureV3::UnsupportedCarrierProjection)?;
                FamilyPayloadWireV1::Seed {
                    family_id,
                    source: SeedSourceWireV1::PublicEquation {
                        equation_id: digest_wire_id(&equation.0)
                            .map_err(|_| NativeCarrierFailureV3::UnsupportedCarrierProjection)?,
                    },
                    judgment: FamilyJudgmentWireV1 {
                        context: ProductionContextWireV1::default(),
                        subject: wire_term(&Term::Global { id: owner.clone() }, slots)?,
                        ty: wire_term(&declared.ty, slots)?,
                    },
                }
            }
            FamilyConstructorV1::GenericPublicApplication {
                function, argument, ..
            } => {
                if inexpressible_ids.contains(function) || inexpressible_ids.contains(argument) {
                    skip(
                        family,
                        WireInexpressibleReasonV3::ReferencesInexpressibleFamily,
                        &mut inexpressible,
                        &mut inexpressible_ids,
                    );
                    continue;
                }
                let GenericJudgmentV1::Term { context, term, ty } = &family.generic_judgment
                else {
                    return Err(NativeCarrierFailureV3::UnsupportedCarrierProjection);
                };
                FamilyPayloadWireV1::GenericPublicApplication {
                    family_id,
                    function_family_id: digest_wire_id(&function.0)
                        .map_err(|_| NativeCarrierFailureV3::UnsupportedCarrierProjection)?,
                    argument_family_id: digest_wire_id(&argument.0)
                        .map_err(|_| NativeCarrierFailureV3::UnsupportedCarrierProjection)?,
                    judgment: FamilyJudgmentWireV1 {
                        context: wire_context(context, slots)?,
                        subject: wire_term(term, slots)?,
                        ty: wire_term(ty, slots)?,
                    },
                }
            }
            FamilyConstructorV1::GenericEquationAction {
                equation, context, ..
            } => {
                if inexpressible_ids.contains(equation) || inexpressible_ids.contains(context) {
                    skip(
                        family,
                        WireInexpressibleReasonV3::ReferencesInexpressibleFamily,
                        &mut inexpressible,
                        &mut inexpressible_ids,
                    );
                    continue;
                }
                let equation_id = equation_seed_families
                    .get(equation)
                    .ok_or(NativeCarrierFailureV3::UnsupportedCarrierProjection)?;
                let source_family = families_by_id
                    .get(context)
                    .ok_or(NativeCarrierFailureV3::UnsupportedCarrierProjection)?;
                let GenericJudgmentV1::Equation {
                    context: judgment_context,
                    right,
                    ty,
                    ..
                } = &family.generic_judgment
                else {
                    return Err(NativeCarrierFailureV3::UnsupportedCarrierProjection);
                };
                // The wire's action payload has no context-witness slot:
                // it can only carry an action judged in exactly its
                // source family's context. Context-growing amalgamations
                // are registered as wire-inexpressible; their equation
                // roots (and reducts) remain in the root inventory.
                if judgment_context != source_family.generic_judgment.context() {
                    skip(
                        family,
                        WireInexpressibleReasonV3::ContextGrowingEquationAction,
                        &mut inexpressible,
                        &mut inexpressible_ids,
                    );
                    continue;
                }
                FamilyPayloadWireV1::GenericEquationAction {
                    family_id,
                    equation_id: digest_wire_id(&equation_id.0)
                        .map_err(|_| NativeCarrierFailureV3::UnsupportedCarrierProjection)?,
                    source_family_id: digest_wire_id(&context.0)
                        .map_err(|_| NativeCarrierFailureV3::UnsupportedCarrierProjection)?,
                    judgment: FamilyJudgmentWireV1 {
                        context: wire_context(judgment_context, slots)?,
                        subject: wire_term(right, slots)?,
                        ty: wire_term(ty, slots)?,
                    },
                }
            }
        };
        family_payloads.push(payload);
    }
    Ok((fresh_rule_schemas, family_payloads, inexpressible))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::verify_semantic_audit_lambda_unit_manifest_v1;
    use crate::production_refinement::diagnose_global_slot_table_v1;
    use pen_kernel::{Declaration, GlobalId, KernelLimits, UncheckedSignature};

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("kernel")
    }

    fn global(label: &[u8]) -> GlobalId {
        GlobalId(Digest::of_bytes(label))
    }

    fn uelim_pi() -> Term {
        Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::Pi {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            }),
        }
    }

    fn fresh_signature(kernel: &Kernel) -> pen_kernel::VerifiedSignature {
        kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![
                    Declaration {
                        id: global(b"native-carrier/constructor"),
                        ty: Term::UnitType,
                        body: None,
                    },
                    Declaration {
                        id: global(b"native-carrier/owner"),
                        ty: uelim_pi(),
                        body: None,
                    },
                    Declaration {
                        id: global(b"native-carrier/bodyful"),
                        ty: Term::UnitType,
                        body: Some(Term::Unit),
                    },
                ],
            })
            .expect("signature")
    }

    fn fresh_equation_judgment(owner: &GlobalId, constructor: &GlobalId) -> GenericJudgmentV1 {
        GenericJudgmentV1::Equation {
            context: DependentContext(vec![Term::UnitType, Term::UnitType]),
            left: Term::Apply {
                function: Box::new(Term::Apply {
                    function: Box::new(Term::Global { id: owner.clone() }),
                    argument: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::Global {
                    id: constructor.clone(),
                }),
            },
            right: Term::Var { index: 0 },
            ty: Term::UnitType,
        }
    }

    #[test]
    fn split_spine_returns_head_and_ordered_arguments() {
        let term = Term::Apply {
            function: Box::new(Term::Apply {
                function: Box::new(Term::Global {
                    id: global(b"native-carrier/spine"),
                }),
                argument: Box::new(Term::Var { index: 1 }),
            }),
            argument: Box::new(Term::Unit),
        };
        let (head, arguments) = split_spine(&term);
        assert_eq!(
            head,
            &Term::Global {
                id: global(b"native-carrier/spine")
            }
        );
        assert_eq!(arguments, vec![&Term::Var { index: 1 }, &Term::Unit]);
    }

    #[test]
    fn fresh_schema_derivation_accepts_the_exact_pattern_and_rejects_deviations() {
        let kernel = kernel();
        let signature = fresh_signature(&kernel);
        let AuditDecision::Proven(v1_manifest) = verify_semantic_audit_lambda_unit_manifest_v1(
            &proposed_semantic_audit_lambda_unit_manifest_v1(),
        ) else {
            panic!("V1 manifest");
        };
        let _ = v1_manifest;
        let slots = diagnose_global_slot_table_v1(&kernel, &signature).expect("slots");
        let owner = global(b"native-carrier/owner");
        let constructor = global(b"native-carrier/constructor");
        let bodyful = global(b"native-carrier/bodyful");
        let equation = EquationIdV1(Digest::of_bytes(b"native-carrier/equation"));

        let schema = derive_fresh_schema(
            &equation,
            &owner,
            &fresh_equation_judgment(&owner, &constructor),
            &signature,
            &slots,
        )
        .expect("exact fresh pattern derives");
        assert_eq!(schema.arity, 2);
        assert_eq!(schema.scrutinee_ordinal, 1);
        assert_eq!(schema.owner_slot, 1);
        assert_eq!(schema.constructor_slot, 0);

        // A bodyful constructor is rejected.
        assert!(matches!(
            derive_fresh_schema(
                &equation,
                &owner,
                &fresh_equation_judgment(&owner, &bodyful),
                &signature,
                &slots,
            ),
            Err(NativeCarrierFailureV3::EquationNotFreshShaped)
        ));

        // An owner-mentioning right side is rejected.
        let GenericJudgmentV1::Equation {
            context, left, ty, ..
        } = fresh_equation_judgment(&owner, &constructor)
        else {
            panic!("equation judgment");
        };
        assert!(matches!(
            derive_fresh_schema(
                &equation,
                &owner,
                &GenericJudgmentV1::Equation {
                    context,
                    left: left.clone(),
                    right: left,
                    ty,
                },
                &signature,
                &slots,
            ),
            Err(NativeCarrierFailureV3::EquationNotFreshShaped)
        ));

        // A non-pattern left side (constant argument in a variable
        // position) is rejected.
        assert!(matches!(
            derive_fresh_schema(
                &equation,
                &owner,
                &GenericJudgmentV1::Equation {
                    context: DependentContext(vec![Term::UnitType, Term::UnitType]),
                    left: Term::Apply {
                        function: Box::new(Term::Apply {
                            function: Box::new(Term::Global { id: owner.clone() }),
                            argument: Box::new(Term::Unit),
                        }),
                        argument: Box::new(Term::Global {
                            id: constructor.clone(),
                        }),
                    },
                    right: Term::Var { index: 0 },
                    ty: Term::UnitType,
                },
                &signature,
                &slots,
            ),
            Err(NativeCarrierFailureV3::EquationNotFreshShaped)
        ));
    }

    #[test]
    fn term_mentions_global_walks_every_lambda_unit_position() {
        let owner = global(b"native-carrier/owner");
        let other = global(b"native-carrier/constructor");
        let term = Term::Lambda {
            parameter_type: Box::new(Term::UnitType),
            body: Box::new(Term::Apply {
                function: Box::new(Term::Global { id: owner.clone() }),
                argument: Box::new(Term::Var { index: 0 }),
            }),
        };
        assert!(term_mentions_global(&term, &owner));
        assert!(!term_mentions_global(&term, &other));
    }
}
