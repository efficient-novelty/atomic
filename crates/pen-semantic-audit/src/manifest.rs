use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use serde::{Deserialize, Serialize};

pub const SEMANTIC_AUDIT_SCHEMA_VERSION: u16 = 1;
pub const KERNEL_COST_SCHEMA_VERSION: u16 = 1;
pub const KERNEL_COST_SCHEMA_VERSION_V2: u16 = 2;
pub const SEMANTIC_AUDIT_PROFILE_ID: &str = "gf2-semantic-audit-core-v1";
pub const KERNEL_COST_PROFILE_ID: &str = "gf2-kernel-cost-core-v1";
pub const KERNEL_COST_PROFILE_ID_V2: &str = "gf2-kernel-cost-core-v2";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuditDecision<T> {
    Proven(T),
    OutsideFragment(OutsideFragmentReason),
    Unknown(AuditUnknownReason),
}

impl<T> AuditDecision<T> {
    pub fn map<U>(self, map: impl FnOnce(T) -> U) -> AuditDecision<U> {
        match self {
            Self::Proven(value) => AuditDecision::Proven(map(value)),
            Self::OutsideFragment(reason) => AuditDecision::OutsideFragment(reason),
            Self::Unknown(reason) => AuditDecision::Unknown(reason),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutsideFragmentReason {
    Path,
    Univalence,
    CubicalTransport,
    ContextualAdjoint,
    RecursiveRewrite,
    OverlappingRewrite,
    UnsupportedTerm,
    UnsupportedUniverseLevel,
    UniversalInterface,
    NonEmptyQ3Registry,
    UnregisteredFamilyConstructor,
    UnregisteredProjection,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditUnknownReason {
    ResourceExhausted,
    UnsupportedVerifier,
    MalformedManifest,
    MalformedInput,
    KernelCouldNotCertify,
    MissingTupleDisposition,
    IncompleteEnumeration,
    NormalizationFailure,
    UnknownQuotient,
    MissingNegativeEvidence,
    MissingVerifiedPublicInventory,
    MissingAmbientFirstExportInventory,
    MissingDescriptorProjectionInventory,
    MissingFreeCompletionTheorem,
    MissingOrdinaryBetaDerivation,
    MissingRewriteAdmissibilityTheorem,
    NonUniqueBasis,
    FailedRetraction,
    IncompleteSupport,
    AmbiguousProvenance,
    ProvenanceCollision,
    RoleMismatch,
    ManifestMismatch,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ManifestAuthority {
    GenericPrototypeOnly,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SeedKind {
    PublicHead,
    PublicEquation,
    PublicUniversalInterfaceReserved,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DerivationRuleV1 {
    Seed,
    GenericPublicApplication,
    GenericEquationAction,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextCarrierRuleV1 {
    SeedTelescope,
    IdenticalTelescope,
    DependencyRespectingDisjointInterleaving,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubstitutionCarrierRuleV1 {
    EmbeddingLift,
    ForcedNewestArgument,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Q0RuleV1 {
    DeBruijn,
    SequentialSubstitution,
    Beta,
    ProvenancePreservingDelta,
    Unit,
    TelescopeFlattening,
    DescriptorForcedProjection,
    FreshNonrecursiveConstructorComputation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Q2RuleV1 {
    BinderRenaming,
    IndependentDeclarationExchange,
    ExplicitTelescopeCurryIsomorphism,
    PriorPublicTransparentAlias,
    DuplicateTransparentField,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Q3RuleV1 {
    VerifiedEmptyOriginCutoffRegistry,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClauseDispositionKindV1 {
    FirstIrreducible,
    TransparentAlias,
    ForcedDefinitionalCompletion,
    ForcedProjection,
    DuplicatePresentation,
    OutsideFragment,
    Unknown,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreeCompletionRuleV1 {
    OrdinaryBeta,
    PriorPublicTransparentAlias,
    DescriptorForcedProjection,
    CertifiedFreshConstructorComputation,
    DuplicatePresentationDeletion,
}

/// The V2 cost-free reconstruction inventory.
///
/// In particular, this inventory has no fresh-constructor-computation rule.
/// A separately sealed equation for a bodyless fresh head is a distinct public
/// commitment unless exact predecessor-public replay or Q2 duplication is
/// independently proved. Rewrite admissibility is governed by the semantic
/// normalizer, not by this cost inventory.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreeCompletionRuleV2 {
    OrdinaryBetaOfBodyfulDefinition,
    PriorPublicTransparentAlias,
    DescriptorForcedProjection,
    DuplicatePresentationDeletion,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticAuditManifestV1 {
    pub schema_version: u16,
    pub profile_id: String,
    pub authority: ManifestAuthority,
    pub frozen: bool,
    pub live_profile_a_access: bool,
    pub universe_levels: Vec<u16>,
    pub maximum_rank: u16,
    pub supported_seed_kinds: Vec<SeedKind>,
    pub ordered_derivation_rules: Vec<DerivationRuleV1>,
    pub context_carrier_rules: Vec<ContextCarrierRuleV1>,
    pub substitution_carrier_rules: Vec<SubstitutionCarrierRuleV1>,
    pub q0_rules: Vec<Q0RuleV1>,
    pub q0_eta_registry_empty: bool,
    pub q2_rules: Vec<Q2RuleV1>,
    pub q3_rule: Q3RuleV1,
    pub maximum_seeds: u16,
    pub maximum_context_entries: u16,
    pub maximum_raw_derivations: u32,
    pub maximum_tuple_dispositions: u32,
    pub maximum_q2_pair_witnesses: u32,
}

impl CanonicalEncode for SemanticAuditManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        encoder.text(&self.profile_id);
        self.authority.encode_canonical(encoder);
        encoder.tag(u8::from(self.frozen));
        encoder.tag(u8::from(self.live_profile_a_access));
        encoder.u64(self.universe_levels.len() as u64);
        for level in &self.universe_levels {
            encoder.u16(*level);
        }
        encoder.u16(self.maximum_rank);
        encoder.sequence(&self.supported_seed_kinds);
        encoder.sequence(&self.ordered_derivation_rules);
        encoder.sequence(&self.context_carrier_rules);
        encoder.sequence(&self.substitution_carrier_rules);
        encoder.sequence(&self.q0_rules);
        encoder.tag(u8::from(self.q0_eta_registry_empty));
        encoder.sequence(&self.q2_rules);
        self.q3_rule.encode_canonical(encoder);
        encoder.u16(self.maximum_seeds);
        encoder.u16(self.maximum_context_entries);
        encoder.u32(self.maximum_raw_derivations);
        encoder.u32(self.maximum_tuple_dispositions);
        encoder.u32(self.maximum_q2_pair_witnesses);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KernelCostManifestV1 {
    pub schema_version: u16,
    pub profile_id: String,
    pub authority: ManifestAuthority,
    pub frozen: bool,
    pub live_profile_a_access: bool,
    pub public_ambient_distinction: bool,
    pub dependency_prior_not_serial_order: bool,
    pub equation_free_descriptor_metadata: bool,
    pub exact_source_normal_pairs: bool,
    pub complete_negative_evidence_required: bool,
    pub unique_basis_required: bool,
    pub ordered_dispositions: Vec<ClauseDispositionKindV1>,
    pub free_completion_rules: Vec<FreeCompletionRuleV1>,
    pub maximum_clauses: u16,
    pub maximum_basis_subsets: u32,
    pub maximum_saturation_rounds: u32,
}

/// Proposed generic kernel-cost profile that separates rewrite admissibility
/// from whether a separately sealed equation is cost-free.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KernelCostManifestV2 {
    pub schema_version: u16,
    pub profile_id: String,
    pub authority: ManifestAuthority,
    pub frozen: bool,
    pub live_profile_a_access: bool,
    pub public_ambient_distinction: bool,
    pub dependency_prior_not_serial_order: bool,
    pub equation_free_descriptor_metadata: bool,
    pub exact_source_normal_pairs: bool,
    pub complete_negative_evidence_required: bool,
    pub unique_basis_required: bool,
    pub separately_sealed_bodyless_equations_are_paid: bool,
    pub rewrite_admissibility_separate_from_cost: bool,
    pub ordered_dispositions: Vec<ClauseDispositionKindV1>,
    pub free_completion_rules: Vec<FreeCompletionRuleV2>,
    pub maximum_clauses: u16,
    pub maximum_basis_subsets: u32,
    pub maximum_saturation_rounds: u32,
}

impl CanonicalEncode for KernelCostManifestV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        encoder.text(&self.profile_id);
        self.authority.encode_canonical(encoder);
        encoder.tag(u8::from(self.frozen));
        encoder.tag(u8::from(self.live_profile_a_access));
        encoder.tag(u8::from(self.public_ambient_distinction));
        encoder.tag(u8::from(self.dependency_prior_not_serial_order));
        encoder.tag(u8::from(self.equation_free_descriptor_metadata));
        encoder.tag(u8::from(self.exact_source_normal_pairs));
        encoder.tag(u8::from(self.complete_negative_evidence_required));
        encoder.tag(u8::from(self.unique_basis_required));
        encoder.tag(u8::from(self.separately_sealed_bodyless_equations_are_paid));
        encoder.tag(u8::from(self.rewrite_admissibility_separate_from_cost));
        encoder.sequence(&self.ordered_dispositions);
        encoder.sequence(&self.free_completion_rules);
        encoder.u16(self.maximum_clauses);
        encoder.u32(self.maximum_basis_subsets);
        encoder.u32(self.maximum_saturation_rounds);
    }
}

impl CanonicalEncode for KernelCostManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        encoder.text(&self.profile_id);
        self.authority.encode_canonical(encoder);
        encoder.tag(u8::from(self.frozen));
        encoder.tag(u8::from(self.live_profile_a_access));
        encoder.tag(u8::from(self.public_ambient_distinction));
        encoder.tag(u8::from(self.dependency_prior_not_serial_order));
        encoder.tag(u8::from(self.equation_free_descriptor_metadata));
        encoder.tag(u8::from(self.exact_source_normal_pairs));
        encoder.tag(u8::from(self.complete_negative_evidence_required));
        encoder.tag(u8::from(self.unique_basis_required));
        encoder.sequence(&self.ordered_dispositions);
        encoder.sequence(&self.free_completion_rules);
        encoder.u16(self.maximum_clauses);
        encoder.u32(self.maximum_basis_subsets);
        encoder.u32(self.maximum_saturation_rounds);
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedSemanticAuditManifestV1 {
    manifest: SemanticAuditManifestV1,
    candidate_digest: Digest,
}

impl VerifiedSemanticAuditManifestV1 {
    pub fn manifest(&self) -> &SemanticAuditManifestV1 {
        &self.manifest
    }

    /// Candidate identity for generic development. This is not an adopted or
    /// frozen Law-V2 semantic digest.
    pub fn candidate_digest(&self) -> &Digest {
        &self.candidate_digest
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedCostManifestV1 {
    manifest: KernelCostManifestV1,
    candidate_digest: Digest,
}

#[derive(Clone, Debug)]
pub struct VerifiedCostManifestV2 {
    manifest: KernelCostManifestV2,
    candidate_digest: Digest,
}

impl VerifiedCostManifestV2 {
    pub fn manifest(&self) -> &KernelCostManifestV2 {
        &self.manifest
    }

    /// Candidate identity for generic development. This is not an adopted or
    /// frozen Law-V2 cost digest and has no live Profile-A authority.
    pub fn candidate_digest(&self) -> &Digest {
        &self.candidate_digest
    }
}

impl VerifiedCostManifestV1 {
    pub fn manifest(&self) -> &KernelCostManifestV1 {
        &self.manifest
    }

    /// Candidate identity for generic development. This is not an adopted or
    /// frozen Law-V2 cost digest.
    pub fn candidate_digest(&self) -> &Digest {
        &self.candidate_digest
    }
}

pub fn proposed_semantic_audit_manifest_v1() -> SemanticAuditManifestV1 {
    SemanticAuditManifestV1 {
        schema_version: SEMANTIC_AUDIT_SCHEMA_VERSION,
        profile_id: SEMANTIC_AUDIT_PROFILE_ID.to_owned(),
        authority: ManifestAuthority::GenericPrototypeOnly,
        frozen: false,
        live_profile_a_access: false,
        universe_levels: vec![0, 1],
        maximum_rank: 2,
        supported_seed_kinds: vec![
            SeedKind::PublicHead,
            SeedKind::PublicEquation,
            SeedKind::PublicUniversalInterfaceReserved,
        ],
        ordered_derivation_rules: vec![
            DerivationRuleV1::Seed,
            DerivationRuleV1::GenericPublicApplication,
            DerivationRuleV1::GenericEquationAction,
        ],
        context_carrier_rules: vec![
            ContextCarrierRuleV1::SeedTelescope,
            ContextCarrierRuleV1::IdenticalTelescope,
            ContextCarrierRuleV1::DependencyRespectingDisjointInterleaving,
        ],
        substitution_carrier_rules: vec![
            SubstitutionCarrierRuleV1::EmbeddingLift,
            SubstitutionCarrierRuleV1::ForcedNewestArgument,
        ],
        q0_rules: vec![
            Q0RuleV1::DeBruijn,
            Q0RuleV1::SequentialSubstitution,
            Q0RuleV1::Beta,
            Q0RuleV1::ProvenancePreservingDelta,
            Q0RuleV1::Unit,
            Q0RuleV1::TelescopeFlattening,
            Q0RuleV1::DescriptorForcedProjection,
            Q0RuleV1::FreshNonrecursiveConstructorComputation,
        ],
        q0_eta_registry_empty: true,
        q2_rules: vec![
            Q2RuleV1::BinderRenaming,
            Q2RuleV1::IndependentDeclarationExchange,
            Q2RuleV1::ExplicitTelescopeCurryIsomorphism,
            Q2RuleV1::PriorPublicTransparentAlias,
            Q2RuleV1::DuplicateTransparentField,
        ],
        q3_rule: Q3RuleV1::VerifiedEmptyOriginCutoffRegistry,
        maximum_seeds: 64,
        maximum_context_entries: 32,
        maximum_raw_derivations: 16_384,
        maximum_tuple_dispositions: 262_144,
        maximum_q2_pair_witnesses: 262_144,
    }
}

pub fn proposed_kernel_cost_manifest_v1() -> KernelCostManifestV1 {
    KernelCostManifestV1 {
        schema_version: KERNEL_COST_SCHEMA_VERSION,
        profile_id: KERNEL_COST_PROFILE_ID.to_owned(),
        authority: ManifestAuthority::GenericPrototypeOnly,
        frozen: false,
        live_profile_a_access: false,
        public_ambient_distinction: true,
        dependency_prior_not_serial_order: true,
        equation_free_descriptor_metadata: true,
        exact_source_normal_pairs: true,
        complete_negative_evidence_required: true,
        unique_basis_required: true,
        ordered_dispositions: vec![
            ClauseDispositionKindV1::FirstIrreducible,
            ClauseDispositionKindV1::TransparentAlias,
            ClauseDispositionKindV1::ForcedDefinitionalCompletion,
            ClauseDispositionKindV1::ForcedProjection,
            ClauseDispositionKindV1::DuplicatePresentation,
            ClauseDispositionKindV1::OutsideFragment,
            ClauseDispositionKindV1::Unknown,
        ],
        free_completion_rules: vec![
            FreeCompletionRuleV1::OrdinaryBeta,
            FreeCompletionRuleV1::PriorPublicTransparentAlias,
            FreeCompletionRuleV1::DescriptorForcedProjection,
            FreeCompletionRuleV1::CertifiedFreshConstructorComputation,
            FreeCompletionRuleV1::DuplicatePresentationDeletion,
        ],
        maximum_clauses: 16,
        maximum_basis_subsets: 65_536,
        maximum_saturation_rounds: 64,
    }
}

pub fn proposed_kernel_cost_manifest_v2() -> KernelCostManifestV2 {
    KernelCostManifestV2 {
        schema_version: KERNEL_COST_SCHEMA_VERSION_V2,
        profile_id: KERNEL_COST_PROFILE_ID_V2.to_owned(),
        authority: ManifestAuthority::GenericPrototypeOnly,
        frozen: false,
        live_profile_a_access: false,
        public_ambient_distinction: true,
        dependency_prior_not_serial_order: true,
        equation_free_descriptor_metadata: true,
        exact_source_normal_pairs: true,
        complete_negative_evidence_required: true,
        unique_basis_required: true,
        separately_sealed_bodyless_equations_are_paid: true,
        rewrite_admissibility_separate_from_cost: true,
        ordered_dispositions: vec![
            ClauseDispositionKindV1::FirstIrreducible,
            ClauseDispositionKindV1::TransparentAlias,
            ClauseDispositionKindV1::ForcedDefinitionalCompletion,
            ClauseDispositionKindV1::ForcedProjection,
            ClauseDispositionKindV1::DuplicatePresentation,
            ClauseDispositionKindV1::OutsideFragment,
            ClauseDispositionKindV1::Unknown,
        ],
        free_completion_rules: vec![
            FreeCompletionRuleV2::OrdinaryBetaOfBodyfulDefinition,
            FreeCompletionRuleV2::PriorPublicTransparentAlias,
            FreeCompletionRuleV2::DescriptorForcedProjection,
            FreeCompletionRuleV2::DuplicatePresentationDeletion,
        ],
        maximum_clauses: 16,
        maximum_basis_subsets: 65_536,
        maximum_saturation_rounds: 64,
    }
}

pub fn verify_semantic_audit_manifest_v1(
    manifest: &SemanticAuditManifestV1,
) -> AuditDecision<VerifiedSemanticAuditManifestV1> {
    if manifest != &proposed_semantic_audit_manifest_v1() {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedManifest);
    }
    AuditDecision::Proven(VerifiedSemanticAuditManifestV1 {
        candidate_digest: Digest::of_canonical(
            "pen-semantic-audit/proposed-semantic-manifest/v1",
            manifest,
        ),
        manifest: manifest.clone(),
    })
}

pub fn verify_kernel_cost_manifest_v1(
    manifest: &KernelCostManifestV1,
) -> AuditDecision<VerifiedCostManifestV1> {
    if manifest != &proposed_kernel_cost_manifest_v1() {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedManifest);
    }
    AuditDecision::Proven(VerifiedCostManifestV1 {
        candidate_digest: Digest::of_canonical(
            "pen-semantic-audit/proposed-cost-manifest/v1",
            manifest,
        ),
        manifest: manifest.clone(),
    })
}

pub fn verify_kernel_cost_manifest_v2(
    manifest: &KernelCostManifestV2,
) -> AuditDecision<VerifiedCostManifestV2> {
    if manifest != &proposed_kernel_cost_manifest_v2() {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedManifest);
    }
    AuditDecision::Proven(VerifiedCostManifestV2 {
        candidate_digest: Digest::of_canonical(
            "pen-semantic-audit/proposed-cost-manifest/v2",
            manifest,
        ),
        manifest: manifest.clone(),
    })
}

pub fn verify_core_manifests_v1()
-> AuditDecision<(VerifiedSemanticAuditManifestV1, VerifiedCostManifestV1)> {
    let semantic = match verify_semantic_audit_manifest_v1(&proposed_semantic_audit_manifest_v1()) {
        AuditDecision::Proven(manifest) => manifest,
        AuditDecision::OutsideFragment(reason) => {
            return AuditDecision::OutsideFragment(reason);
        }
        AuditDecision::Unknown(reason) => return AuditDecision::Unknown(reason),
    };
    let cost = match verify_kernel_cost_manifest_v1(&proposed_kernel_cost_manifest_v1()) {
        AuditDecision::Proven(manifest) => manifest,
        AuditDecision::OutsideFragment(reason) => {
            return AuditDecision::OutsideFragment(reason);
        }
        AuditDecision::Unknown(reason) => return AuditDecision::Unknown(reason),
    };
    AuditDecision::Proven((semantic, cost))
}

macro_rules! canonical_tags {
    ($type:ty, {$($variant:path => $tag:expr),+ $(,)?}) => {
        impl CanonicalEncode for $type {
            fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
                encoder.tag(match self {
                    $($variant => $tag,)+
                });
            }
        }
    };
}

canonical_tags!(ManifestAuthority, {
    ManifestAuthority::GenericPrototypeOnly => 0
});
canonical_tags!(SeedKind, {
    SeedKind::PublicHead => 0,
    SeedKind::PublicEquation => 1,
    SeedKind::PublicUniversalInterfaceReserved => 2
});
canonical_tags!(DerivationRuleV1, {
    DerivationRuleV1::Seed => 0,
    DerivationRuleV1::GenericPublicApplication => 1,
    DerivationRuleV1::GenericEquationAction => 2
});
canonical_tags!(ContextCarrierRuleV1, {
    ContextCarrierRuleV1::SeedTelescope => 0,
    ContextCarrierRuleV1::IdenticalTelescope => 1,
    ContextCarrierRuleV1::DependencyRespectingDisjointInterleaving => 2
});
canonical_tags!(SubstitutionCarrierRuleV1, {
    SubstitutionCarrierRuleV1::EmbeddingLift => 0,
    SubstitutionCarrierRuleV1::ForcedNewestArgument => 1
});
canonical_tags!(Q0RuleV1, {
    Q0RuleV1::DeBruijn => 0,
    Q0RuleV1::SequentialSubstitution => 1,
    Q0RuleV1::Beta => 2,
    Q0RuleV1::ProvenancePreservingDelta => 3,
    Q0RuleV1::Unit => 4,
    Q0RuleV1::TelescopeFlattening => 5,
    Q0RuleV1::DescriptorForcedProjection => 6,
    Q0RuleV1::FreshNonrecursiveConstructorComputation => 7
});
canonical_tags!(Q2RuleV1, {
    Q2RuleV1::BinderRenaming => 0,
    Q2RuleV1::IndependentDeclarationExchange => 1,
    Q2RuleV1::ExplicitTelescopeCurryIsomorphism => 2,
    Q2RuleV1::PriorPublicTransparentAlias => 3,
    Q2RuleV1::DuplicateTransparentField => 4
});
canonical_tags!(Q3RuleV1, {
    Q3RuleV1::VerifiedEmptyOriginCutoffRegistry => 0
});
canonical_tags!(ClauseDispositionKindV1, {
    ClauseDispositionKindV1::FirstIrreducible => 0,
    ClauseDispositionKindV1::TransparentAlias => 1,
    ClauseDispositionKindV1::ForcedDefinitionalCompletion => 2,
    ClauseDispositionKindV1::ForcedProjection => 3,
    ClauseDispositionKindV1::DuplicatePresentation => 4,
    ClauseDispositionKindV1::OutsideFragment => 5,
    ClauseDispositionKindV1::Unknown => 6
});
canonical_tags!(FreeCompletionRuleV1, {
    FreeCompletionRuleV1::OrdinaryBeta => 0,
    FreeCompletionRuleV1::PriorPublicTransparentAlias => 1,
    FreeCompletionRuleV1::DescriptorForcedProjection => 2,
    FreeCompletionRuleV1::CertifiedFreshConstructorComputation => 3,
    FreeCompletionRuleV1::DuplicatePresentationDeletion => 4
});
canonical_tags!(FreeCompletionRuleV2, {
    FreeCompletionRuleV2::OrdinaryBetaOfBodyfulDefinition => 0,
    FreeCompletionRuleV2::PriorPublicTransparentAlias => 1,
    FreeCompletionRuleV2::DescriptorForcedProjection => 2,
    FreeCompletionRuleV2::DuplicatePresentationDeletion => 3
});

#[cfg(test)]
mod tests {
    use super::{
        AuditDecision, AuditUnknownReason, FreeCompletionRuleV2, proposed_kernel_cost_manifest_v1,
        proposed_kernel_cost_manifest_v2, proposed_semantic_audit_manifest_v1,
        verify_core_manifests_v1, verify_kernel_cost_manifest_v2,
        verify_semantic_audit_manifest_v1,
    };

    #[test]
    fn proposed_manifests_verify_but_have_no_live_authority() {
        let AuditDecision::Proven((semantic, cost)) = verify_core_manifests_v1() else {
            panic!("exact proposed manifests should verify");
        };
        assert!(!semantic.manifest().frozen);
        assert!(!semantic.manifest().live_profile_a_access);
        assert!(!cost.manifest().frozen);
        assert!(!cost.manifest().live_profile_a_access);
        assert_ne!(semantic.candidate_digest(), cost.candidate_digest());
    }

    #[test]
    fn semantic_manifest_is_closed() {
        let mut changed = proposed_semantic_audit_manifest_v1();
        changed.maximum_rank = 3;
        assert!(matches!(
            verify_semantic_audit_manifest_v1(&changed),
            AuditDecision::Unknown(AuditUnknownReason::MalformedManifest)
        ));
        assert_eq!(
            proposed_kernel_cost_manifest_v1().maximum_basis_subsets,
            65_536
        );
    }

    #[test]
    fn v2_cost_manifest_omits_fresh_completion_and_has_no_live_authority() {
        let proposed = proposed_kernel_cost_manifest_v2();
        assert_eq!(proposed.profile_id, "gf2-kernel-cost-core-v2");
        assert!(!proposed.frozen);
        assert!(!proposed.live_profile_a_access);
        assert!(proposed.separately_sealed_bodyless_equations_are_paid);
        assert!(proposed.rewrite_admissibility_separate_from_cost);
        assert_eq!(
            proposed.free_completion_rules,
            vec![
                FreeCompletionRuleV2::OrdinaryBetaOfBodyfulDefinition,
                FreeCompletionRuleV2::PriorPublicTransparentAlias,
                FreeCompletionRuleV2::DescriptorForcedProjection,
                FreeCompletionRuleV2::DuplicatePresentationDeletion,
            ]
        );

        let AuditDecision::Proven(verified) = verify_kernel_cost_manifest_v2(&proposed) else {
            panic!("exact V2 generic manifest should verify");
        };
        assert!(!verified.manifest().frozen);
        assert!(!verified.manifest().live_profile_a_access);
    }

    #[test]
    fn v2_cost_manifest_is_closed() {
        let mut changed = proposed_kernel_cost_manifest_v2();
        changed.rewrite_admissibility_separate_from_cost = false;
        assert!(matches!(
            verify_kernel_cost_manifest_v2(&changed),
            AuditDecision::Unknown(AuditUnknownReason::MalformedManifest)
        ));
    }
}
