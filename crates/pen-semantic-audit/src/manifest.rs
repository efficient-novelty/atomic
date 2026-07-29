use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest};
use serde::{Deserialize, Serialize};

pub const SEMANTIC_AUDIT_SCHEMA_VERSION: u16 = 1;
pub const SEMANTIC_AUDIT_SCHEMA_VERSION_V2: u16 = 2;
pub const SEMANTIC_AUDIT_SCHEMA_VERSION_V3: u16 = 3;
pub const KERNEL_COST_SCHEMA_VERSION: u16 = 1;
pub const KERNEL_COST_SCHEMA_VERSION_V2: u16 = 2;
pub const SEMANTIC_AUDIT_PROFILE_ID: &str = "gf2-semantic-audit-core-v1";
pub const KERNEL_COST_PROFILE_ID: &str = "gf2-kernel-cost-core-v1";
pub const KERNEL_COST_PROFILE_ID_V2: &str = "gf2-kernel-cost-core-v2";
pub const SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1: &str = "gf2-semantic-audit-lambda-unit-v1";
pub const SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2: &str = "gf2-semantic-audit-lambda-unit-v2";
pub const SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3: &str = "gf2-semantic-audit-lambda-unit-v3";
pub const KERNEL_COST_LAMBDA_UNIT_PROFILE_ID_V2: &str = "gf2-kernel-cost-lambda-unit-v2";

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
    DescriptorProjection,
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
    MissingSubstitutionMetatheory,
    MissingDemandOrbitAuthority,
    MissingDemandRealizationAuthority,
    MissingSemanticSeedCensus,
    MissingRankInductiveCarrierTheorem,
    MissingTypedOccurrenceCensus,
    MissingHistoricalRewriteAuthority,
    MissingOverlapCensus,
    MissingWeakeningImageConservativity,
    MissingLambdaUnitTypingMetatheory,
    MissingDemandNeutralSemanticSeedCensus,
    MissingNativeRankInductiveCarrierV3,
    MissingSynthesisBackedTypedOccurrenceCensus,
    MissingRewriteSystemV3,
    MissingFamilyQuotientV3,
    MissingWeakeningMarginalAuthority,
    MissingDemandOrbitCensusV2,
    MissingDemandRealizationCensusV2,
    MissingSr2ProvenanceAssignment,
    MissingIssuedPriorRewriteSystemV3,
    DemandAuthorityEnteredStructuralIdentity,
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

/// Direct substitutions consumed by finite rank-at-most-two constructors.
///
/// This is deliberately not a presentation of every typed substitution and
/// has no global composite constructor.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstructionSubstitutionRuleV2 {
    ContextEmbedding,
    ForcedNewestArgument,
    OneHoleInstantiation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubstitutionCensusScopeV2 {
    DirectConstructionWitnessesOnly,
}

/// Generic laws required of arbitrary well-typed simultaneous
/// substitutions. They are theorem obligations, not members of a finite
/// substitution census.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GenericSubstitutionTheoremV2 {
    Identity,
    Composition,
    Associativity,
    PiBinderLifting,
    LambdaBinderLifting,
    Weakening,
    LiftingCompositionCommutation,
    CaptureAvoidance,
    TypingPreservation,
    OrdinaryBetaStability,
    ProvenancePreservingDeltaStability,
    UnitStability,
    InventoriedFreshEquationStability,
    GenericPublicApplicationNaturality,
    GenericEquationActionNaturality,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshRuleMatchingProtocolV2 {
    EdgeLocalTypedLeftLinear,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TypedOccurrenceCensusProtocolV2 {
    BinderLocalTermsAndJudgmentTypes,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoricalRewriteProtocolV2 {
    IndependentSubjectsWithVerifiedHistory,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConservativityProtocolV2 {
    WeakeningImage,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OverlapCensusProtocolV2 {
    ExhaustiveImmediateEdgePairs,
}

/// Exact V3 source of semantic seed and raw-family support.
///
/// Demand contracts, orbits, outputs, and realizations are intentionally
/// absent: they are provenance evidence assigned only after quotienting and
/// marginal-family identification.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticIdentityProtocolV3 {
    DemandNeutralStructuralSupport,
}

/// Authority carried by an inventoried equation port before SR2.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EquationPortMetadataProtocolV3 {
    BoundNonAuthoritative,
}

/// Stage at which typed demand evidence becomes semantic authority.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DemandProvenanceProtocolV3 {
    PostQuotientWeakeningMarginalSr2Only,
}

/// Historical authority admitted by the V3 rewrite construction.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoricalAuthorityProtocolV3 {
    EmptyBaseOrExactIssuedPriorRewriteSystem,
}

/// Closed V3 authority order. The sequence is part of the candidate digest.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticAuthorityStageV3 {
    PublicClauseCensus,
    DemandNeutralSemanticSeedCensus,
    LambdaUnitTypingAndSynthesisMetatheory,
    NativeRankInductiveCarrier,
    SynthesisBackedTypedOccurrenceCensus,
    RewriteSystem,
    FamilyQuotient,
    WeakeningAndRestriction,
    MarginalFamilySet,
    DemandOrbitCensus,
    DemandRealizationCensus,
    Sr2ProvenanceAssignment,
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

/// Projection-free semantic successor that separates finite construction
/// witnesses from the generic metatheory of arbitrary typed substitutions.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticAuditManifestV2 {
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
    pub substitution_census_scope: SubstitutionCensusScopeV2,
    pub construction_substitution_rules: Vec<ConstructionSubstitutionRuleV2>,
    pub generic_substitution_theorems: Vec<GenericSubstitutionTheoremV2>,
    pub fresh_rule_matching_protocol: FreshRuleMatchingProtocolV2,
    pub typed_occurrence_census_protocol: TypedOccurrenceCensusProtocolV2,
    pub historical_rewrite_protocol: HistoricalRewriteProtocolV2,
    pub conservativity_protocol: ConservativityProtocolV2,
    pub overlap_census_protocol: OverlapCensusProtocolV2,
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

impl CanonicalEncode for SemanticAuditManifestV2 {
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
        self.substitution_census_scope.encode_canonical(encoder);
        encoder.sequence(&self.construction_substitution_rules);
        encoder.sequence(&self.generic_substitution_theorems);
        self.fresh_rule_matching_protocol.encode_canonical(encoder);
        self.typed_occurrence_census_protocol
            .encode_canonical(encoder);
        self.historical_rewrite_protocol.encode_canonical(encoder);
        self.conservativity_protocol.encode_canonical(encoder);
        self.overlap_census_protocol.encode_canonical(encoder);
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

/// Demand-neutral successor of the lambda/unit V2 semantic proposal.
///
/// V3 preserves the projection-free rank-at-most-two construction and rewrite
/// surface, while correcting the authority order: structural support defines
/// seeds and families; demand provenance is attached only downstream at SR2.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticAuditManifestV3 {
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
    pub substitution_census_scope: SubstitutionCensusScopeV2,
    pub construction_substitution_rules: Vec<ConstructionSubstitutionRuleV2>,
    pub generic_substitution_theorems: Vec<GenericSubstitutionTheoremV2>,
    pub fresh_rule_matching_protocol: FreshRuleMatchingProtocolV2,
    pub typed_occurrence_census_protocol: TypedOccurrenceCensusProtocolV2,
    pub historical_rewrite_protocol: HistoricalRewriteProtocolV2,
    pub conservativity_protocol: ConservativityProtocolV2,
    pub overlap_census_protocol: OverlapCensusProtocolV2,
    pub semantic_identity_protocol: SemanticIdentityProtocolV3,
    pub equation_port_metadata_protocol: EquationPortMetadataProtocolV3,
    pub demand_provenance_protocol: DemandProvenanceProtocolV3,
    pub historical_authority_protocol: HistoricalAuthorityProtocolV3,
    pub authority_order: Vec<SemanticAuthorityStageV3>,
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

impl CanonicalEncode for SemanticAuditManifestV3 {
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
        self.substitution_census_scope.encode_canonical(encoder);
        encoder.sequence(&self.construction_substitution_rules);
        encoder.sequence(&self.generic_substitution_theorems);
        self.fresh_rule_matching_protocol.encode_canonical(encoder);
        self.typed_occurrence_census_protocol
            .encode_canonical(encoder);
        self.historical_rewrite_protocol.encode_canonical(encoder);
        self.conservativity_protocol.encode_canonical(encoder);
        self.overlap_census_protocol.encode_canonical(encoder);
        self.semantic_identity_protocol.encode_canonical(encoder);
        self.equation_port_metadata_protocol
            .encode_canonical(encoder);
        self.demand_provenance_protocol.encode_canonical(encoder);
        self.historical_authority_protocol.encode_canonical(encoder);
        encoder.sequence(&self.authority_order);
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
pub struct VerifiedSemanticAuditManifestV2 {
    manifest: SemanticAuditManifestV2,
    candidate_digest: Digest,
}

#[derive(Clone, Debug)]
pub struct VerifiedSemanticAuditManifestV3 {
    manifest: SemanticAuditManifestV3,
    candidate_digest: Digest,
}

impl VerifiedSemanticAuditManifestV3 {
    pub fn manifest(&self) -> &SemanticAuditManifestV3 {
        &self.manifest
    }

    /// Candidate identity for generic development. This is not an adopted or
    /// frozen Law-V2 semantic digest and has no live Profile-A authority.
    pub fn candidate_digest(&self) -> &Digest {
        &self.candidate_digest
    }
}

impl VerifiedSemanticAuditManifestV2 {
    pub fn manifest(&self) -> &SemanticAuditManifestV2 {
        &self.manifest
    }

    /// Candidate identity for generic development. This is not an adopted or
    /// frozen Law-V2 semantic digest and has no live Profile-A authority.
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

/// Projection-free successor of the broader semantic-audit proposal.
///
/// This remains a generic, unfrozen proposal.  The successor changes only the
/// manifest-indexed fragment boundary: records and descriptor-forced
/// projections are outside the lambda/unit fragment rather than pending
/// theorem obligations inside it.
pub fn proposed_semantic_audit_lambda_unit_manifest_v1() -> SemanticAuditManifestV1 {
    let mut manifest = proposed_semantic_audit_manifest_v1();
    manifest.profile_id = SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1.to_owned();
    manifest
        .q0_rules
        .retain(|rule| *rule != Q0RuleV1::DescriptorForcedProjection);
    manifest
}

/// Projection-free semantic successor with finite direct construction
/// witnesses and a separate generic substitution theorem.
///
/// The construction census is not the category of all typed substitutions:
/// arbitrary identity, composition, lifting, typing, reduction stability, and
/// family naturality are governed by the theorem obligations below.
pub fn proposed_semantic_audit_lambda_unit_manifest_v2() -> SemanticAuditManifestV2 {
    SemanticAuditManifestV2 {
        schema_version: SEMANTIC_AUDIT_SCHEMA_VERSION_V2,
        profile_id: SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V2.to_owned(),
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
        substitution_census_scope: SubstitutionCensusScopeV2::DirectConstructionWitnessesOnly,
        construction_substitution_rules: vec![
            ConstructionSubstitutionRuleV2::ContextEmbedding,
            ConstructionSubstitutionRuleV2::ForcedNewestArgument,
            ConstructionSubstitutionRuleV2::OneHoleInstantiation,
        ],
        generic_substitution_theorems: vec![
            GenericSubstitutionTheoremV2::Identity,
            GenericSubstitutionTheoremV2::Composition,
            GenericSubstitutionTheoremV2::Associativity,
            GenericSubstitutionTheoremV2::PiBinderLifting,
            GenericSubstitutionTheoremV2::LambdaBinderLifting,
            GenericSubstitutionTheoremV2::Weakening,
            GenericSubstitutionTheoremV2::LiftingCompositionCommutation,
            GenericSubstitutionTheoremV2::CaptureAvoidance,
            GenericSubstitutionTheoremV2::TypingPreservation,
            GenericSubstitutionTheoremV2::OrdinaryBetaStability,
            GenericSubstitutionTheoremV2::ProvenancePreservingDeltaStability,
            GenericSubstitutionTheoremV2::UnitStability,
            GenericSubstitutionTheoremV2::InventoriedFreshEquationStability,
            GenericSubstitutionTheoremV2::GenericPublicApplicationNaturality,
            GenericSubstitutionTheoremV2::GenericEquationActionNaturality,
        ],
        fresh_rule_matching_protocol: FreshRuleMatchingProtocolV2::EdgeLocalTypedLeftLinear,
        typed_occurrence_census_protocol:
            TypedOccurrenceCensusProtocolV2::BinderLocalTermsAndJudgmentTypes,
        historical_rewrite_protocol:
            HistoricalRewriteProtocolV2::IndependentSubjectsWithVerifiedHistory,
        conservativity_protocol: ConservativityProtocolV2::WeakeningImage,
        overlap_census_protocol: OverlapCensusProtocolV2::ExhaustiveImmediateEdgePairs,
        q0_rules: vec![
            Q0RuleV1::DeBruijn,
            Q0RuleV1::SequentialSubstitution,
            Q0RuleV1::Beta,
            Q0RuleV1::ProvenancePreservingDelta,
            Q0RuleV1::Unit,
            Q0RuleV1::TelescopeFlattening,
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

/// Demand-neutral V3 successor preserving the exact V2 lambda/unit surface.
pub fn proposed_semantic_audit_lambda_unit_manifest_v3() -> SemanticAuditManifestV3 {
    let v2 = proposed_semantic_audit_lambda_unit_manifest_v2();
    SemanticAuditManifestV3 {
        schema_version: SEMANTIC_AUDIT_SCHEMA_VERSION_V3,
        profile_id: SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3.to_owned(),
        authority: v2.authority,
        frozen: false,
        live_profile_a_access: false,
        universe_levels: v2.universe_levels,
        maximum_rank: v2.maximum_rank,
        supported_seed_kinds: v2.supported_seed_kinds,
        ordered_derivation_rules: v2.ordered_derivation_rules,
        context_carrier_rules: v2.context_carrier_rules,
        substitution_census_scope: v2.substitution_census_scope,
        construction_substitution_rules: v2.construction_substitution_rules,
        generic_substitution_theorems: v2.generic_substitution_theorems,
        fresh_rule_matching_protocol: v2.fresh_rule_matching_protocol,
        typed_occurrence_census_protocol: v2.typed_occurrence_census_protocol,
        historical_rewrite_protocol: v2.historical_rewrite_protocol,
        conservativity_protocol: v2.conservativity_protocol,
        overlap_census_protocol: v2.overlap_census_protocol,
        semantic_identity_protocol: SemanticIdentityProtocolV3::DemandNeutralStructuralSupport,
        equation_port_metadata_protocol: EquationPortMetadataProtocolV3::BoundNonAuthoritative,
        demand_provenance_protocol:
            DemandProvenanceProtocolV3::PostQuotientWeakeningMarginalSr2Only,
        historical_authority_protocol:
            HistoricalAuthorityProtocolV3::EmptyBaseOrExactIssuedPriorRewriteSystem,
        authority_order: vec![
            SemanticAuthorityStageV3::PublicClauseCensus,
            SemanticAuthorityStageV3::DemandNeutralSemanticSeedCensus,
            SemanticAuthorityStageV3::LambdaUnitTypingAndSynthesisMetatheory,
            SemanticAuthorityStageV3::NativeRankInductiveCarrier,
            SemanticAuthorityStageV3::SynthesisBackedTypedOccurrenceCensus,
            SemanticAuthorityStageV3::RewriteSystem,
            SemanticAuthorityStageV3::FamilyQuotient,
            SemanticAuthorityStageV3::WeakeningAndRestriction,
            SemanticAuthorityStageV3::MarginalFamilySet,
            SemanticAuthorityStageV3::DemandOrbitCensus,
            SemanticAuthorityStageV3::DemandRealizationCensus,
            SemanticAuthorityStageV3::Sr2ProvenanceAssignment,
        ],
        q0_rules: v2.q0_rules,
        q0_eta_registry_empty: v2.q0_eta_registry_empty,
        q2_rules: v2.q2_rules,
        q3_rule: v2.q3_rule,
        maximum_seeds: v2.maximum_seeds,
        maximum_context_entries: v2.maximum_context_entries,
        maximum_raw_derivations: v2.maximum_raw_derivations,
        maximum_tuple_dispositions: v2.maximum_tuple_dispositions,
        maximum_q2_pair_witnesses: v2.maximum_q2_pair_witnesses,
    }
}

/// Projection-free successor of the broader V2 kernel-cost proposal.
pub fn proposed_kernel_cost_lambda_unit_manifest_v2() -> KernelCostManifestV2 {
    let mut manifest = proposed_kernel_cost_manifest_v2();
    manifest.profile_id = KERNEL_COST_LAMBDA_UNIT_PROFILE_ID_V2.to_owned();
    manifest
        .ordered_dispositions
        .retain(|kind| *kind != ClauseDispositionKindV1::ForcedProjection);
    manifest
        .free_completion_rules
        .retain(|rule| *rule != FreeCompletionRuleV2::DescriptorForcedProjection);
    manifest
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

pub fn verify_semantic_audit_lambda_unit_manifest_v1(
    manifest: &SemanticAuditManifestV1,
) -> AuditDecision<VerifiedSemanticAuditManifestV1> {
    if manifest != &proposed_semantic_audit_lambda_unit_manifest_v1() {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedManifest);
    }
    AuditDecision::Proven(VerifiedSemanticAuditManifestV1 {
        candidate_digest: Digest::of_canonical(
            "pen-semantic-audit/proposed-semantic-lambda-unit-manifest/v1",
            manifest,
        ),
        manifest: manifest.clone(),
    })
}

pub fn verify_semantic_audit_lambda_unit_manifest_v2(
    manifest: &SemanticAuditManifestV2,
) -> AuditDecision<VerifiedSemanticAuditManifestV2> {
    if manifest != &proposed_semantic_audit_lambda_unit_manifest_v2() {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedManifest);
    }
    AuditDecision::Proven(VerifiedSemanticAuditManifestV2 {
        candidate_digest: Digest::of_canonical(
            "pen-semantic-audit/proposed-semantic-lambda-unit-manifest/v2",
            manifest,
        ),
        manifest: manifest.clone(),
    })
}

pub fn verify_semantic_audit_lambda_unit_manifest_v3(
    manifest: &SemanticAuditManifestV3,
) -> AuditDecision<VerifiedSemanticAuditManifestV3> {
    if manifest != &proposed_semantic_audit_lambda_unit_manifest_v3() {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedManifest);
    }
    AuditDecision::Proven(VerifiedSemanticAuditManifestV3 {
        candidate_digest: Digest::of_canonical(
            "pen-semantic-audit/proposed-semantic-lambda-unit-manifest/v3",
            manifest,
        ),
        manifest: manifest.clone(),
    })
}

pub fn verify_kernel_cost_lambda_unit_manifest_v2(
    manifest: &KernelCostManifestV2,
) -> AuditDecision<VerifiedCostManifestV2> {
    if manifest != &proposed_kernel_cost_lambda_unit_manifest_v2() {
        return AuditDecision::Unknown(AuditUnknownReason::MalformedManifest);
    }
    AuditDecision::Proven(VerifiedCostManifestV2 {
        candidate_digest: Digest::of_canonical(
            "pen-semantic-audit/proposed-cost-lambda-unit-manifest/v2",
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
canonical_tags!(ConstructionSubstitutionRuleV2, {
    ConstructionSubstitutionRuleV2::ContextEmbedding => 0,
    ConstructionSubstitutionRuleV2::ForcedNewestArgument => 1,
    ConstructionSubstitutionRuleV2::OneHoleInstantiation => 2
});
canonical_tags!(SubstitutionCensusScopeV2, {
    SubstitutionCensusScopeV2::DirectConstructionWitnessesOnly => 0
});
canonical_tags!(GenericSubstitutionTheoremV2, {
    GenericSubstitutionTheoremV2::Identity => 0,
    GenericSubstitutionTheoremV2::Composition => 1,
    GenericSubstitutionTheoremV2::Associativity => 2,
    GenericSubstitutionTheoremV2::PiBinderLifting => 3,
    GenericSubstitutionTheoremV2::LambdaBinderLifting => 4,
    GenericSubstitutionTheoremV2::Weakening => 5,
    GenericSubstitutionTheoremV2::LiftingCompositionCommutation => 6,
    GenericSubstitutionTheoremV2::CaptureAvoidance => 7,
    GenericSubstitutionTheoremV2::TypingPreservation => 8,
    GenericSubstitutionTheoremV2::OrdinaryBetaStability => 9,
    GenericSubstitutionTheoremV2::ProvenancePreservingDeltaStability => 10,
    GenericSubstitutionTheoremV2::UnitStability => 11,
    GenericSubstitutionTheoremV2::InventoriedFreshEquationStability => 12,
    GenericSubstitutionTheoremV2::GenericPublicApplicationNaturality => 13,
    GenericSubstitutionTheoremV2::GenericEquationActionNaturality => 14
});
canonical_tags!(FreshRuleMatchingProtocolV2, {
    FreshRuleMatchingProtocolV2::EdgeLocalTypedLeftLinear => 0
});
canonical_tags!(TypedOccurrenceCensusProtocolV2, {
    TypedOccurrenceCensusProtocolV2::BinderLocalTermsAndJudgmentTypes => 0
});
canonical_tags!(HistoricalRewriteProtocolV2, {
    HistoricalRewriteProtocolV2::IndependentSubjectsWithVerifiedHistory => 0
});
canonical_tags!(ConservativityProtocolV2, {
    ConservativityProtocolV2::WeakeningImage => 0
});
canonical_tags!(OverlapCensusProtocolV2, {
    OverlapCensusProtocolV2::ExhaustiveImmediateEdgePairs => 0
});
canonical_tags!(SemanticIdentityProtocolV3, {
    SemanticIdentityProtocolV3::DemandNeutralStructuralSupport => 0
});
canonical_tags!(EquationPortMetadataProtocolV3, {
    EquationPortMetadataProtocolV3::BoundNonAuthoritative => 0
});
canonical_tags!(DemandProvenanceProtocolV3, {
    DemandProvenanceProtocolV3::PostQuotientWeakeningMarginalSr2Only => 0
});
canonical_tags!(HistoricalAuthorityProtocolV3, {
    HistoricalAuthorityProtocolV3::EmptyBaseOrExactIssuedPriorRewriteSystem => 0
});
canonical_tags!(SemanticAuthorityStageV3, {
    SemanticAuthorityStageV3::PublicClauseCensus => 0,
    SemanticAuthorityStageV3::DemandNeutralSemanticSeedCensus => 1,
    SemanticAuthorityStageV3::LambdaUnitTypingAndSynthesisMetatheory => 2,
    SemanticAuthorityStageV3::NativeRankInductiveCarrier => 3,
    SemanticAuthorityStageV3::SynthesisBackedTypedOccurrenceCensus => 4,
    SemanticAuthorityStageV3::RewriteSystem => 5,
    SemanticAuthorityStageV3::FamilyQuotient => 6,
    SemanticAuthorityStageV3::WeakeningAndRestriction => 7,
    SemanticAuthorityStageV3::MarginalFamilySet => 8,
    SemanticAuthorityStageV3::DemandOrbitCensus => 9,
    SemanticAuthorityStageV3::DemandRealizationCensus => 10,
    SemanticAuthorityStageV3::Sr2ProvenanceAssignment => 11
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
        AuditDecision, AuditUnknownReason, ClauseDispositionKindV1, ConservativityProtocolV2,
        ConstructionSubstitutionRuleV2, FreeCompletionRuleV2, FreshRuleMatchingProtocolV2,
        GenericSubstitutionTheoremV2, HistoricalAuthorityProtocolV3, HistoricalRewriteProtocolV2,
        OverlapCensusProtocolV2, Q0RuleV1, Q3RuleV1, SemanticAuthorityStageV3,
        SemanticIdentityProtocolV3, SubstitutionCensusScopeV2, TypedOccurrenceCensusProtocolV2,
        proposed_kernel_cost_lambda_unit_manifest_v2, proposed_kernel_cost_manifest_v1,
        proposed_kernel_cost_manifest_v2, proposed_semantic_audit_lambda_unit_manifest_v1,
        proposed_semantic_audit_lambda_unit_manifest_v2,
        proposed_semantic_audit_lambda_unit_manifest_v3, proposed_semantic_audit_manifest_v1,
        verify_core_manifests_v1, verify_kernel_cost_lambda_unit_manifest_v2,
        verify_kernel_cost_manifest_v2, verify_semantic_audit_lambda_unit_manifest_v1,
        verify_semantic_audit_lambda_unit_manifest_v2,
        verify_semantic_audit_lambda_unit_manifest_v3, verify_semantic_audit_manifest_v1,
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

    #[test]
    fn lambda_unit_successors_are_projection_free_and_have_no_live_authority() {
        let semantic = proposed_semantic_audit_lambda_unit_manifest_v1();
        assert_eq!(semantic.profile_id, "gf2-semantic-audit-lambda-unit-v1");
        assert!(!semantic.frozen);
        assert!(!semantic.live_profile_a_access);
        assert!(
            !semantic
                .q0_rules
                .contains(&Q0RuleV1::DescriptorForcedProjection)
        );
        let AuditDecision::Proven(verified_semantic) =
            verify_semantic_audit_lambda_unit_manifest_v1(&semantic)
        else {
            panic!("exact lambda/unit semantic successor should verify");
        };

        let cost = proposed_kernel_cost_lambda_unit_manifest_v2();
        assert_eq!(cost.profile_id, "gf2-kernel-cost-lambda-unit-v2");
        assert!(!cost.frozen);
        assert!(!cost.live_profile_a_access);
        assert!(
            !cost
                .ordered_dispositions
                .contains(&ClauseDispositionKindV1::ForcedProjection)
        );
        assert!(
            !cost
                .free_completion_rules
                .contains(&FreeCompletionRuleV2::DescriptorForcedProjection)
        );
        let AuditDecision::Proven(verified_cost) =
            verify_kernel_cost_lambda_unit_manifest_v2(&cost)
        else {
            panic!("exact lambda/unit cost successor should verify");
        };
        assert_ne!(
            verified_semantic.candidate_digest(),
            verified_cost.candidate_digest()
        );
    }

    #[test]
    fn lambda_unit_successor_manifests_are_closed() {
        let mut semantic = proposed_semantic_audit_lambda_unit_manifest_v1();
        semantic.q0_rules.push(Q0RuleV1::DescriptorForcedProjection);
        assert!(matches!(
            verify_semantic_audit_lambda_unit_manifest_v1(&semantic),
            AuditDecision::Unknown(AuditUnknownReason::MalformedManifest)
        ));

        let mut cost = proposed_kernel_cost_lambda_unit_manifest_v2();
        cost.free_completion_rules
            .push(FreeCompletionRuleV2::DescriptorForcedProjection);
        assert!(matches!(
            verify_kernel_cost_lambda_unit_manifest_v2(&cost),
            AuditDecision::Unknown(AuditUnknownReason::MalformedManifest)
        ));
    }

    #[test]
    fn lambda_unit_v2_separates_direct_witnesses_from_generic_substitution() {
        let proposed = proposed_semantic_audit_lambda_unit_manifest_v2();
        assert_eq!(proposed.profile_id, "gf2-semantic-audit-lambda-unit-v2");
        assert_eq!(proposed.schema_version, 2);
        assert!(!proposed.frozen);
        assert!(!proposed.live_profile_a_access);
        assert_eq!(
            proposed.substitution_census_scope,
            SubstitutionCensusScopeV2::DirectConstructionWitnessesOnly
        );
        assert_eq!(
            proposed.construction_substitution_rules,
            vec![
                ConstructionSubstitutionRuleV2::ContextEmbedding,
                ConstructionSubstitutionRuleV2::ForcedNewestArgument,
                ConstructionSubstitutionRuleV2::OneHoleInstantiation,
            ]
        );
        assert_eq!(
            proposed.generic_substitution_theorems,
            vec![
                GenericSubstitutionTheoremV2::Identity,
                GenericSubstitutionTheoremV2::Composition,
                GenericSubstitutionTheoremV2::Associativity,
                GenericSubstitutionTheoremV2::PiBinderLifting,
                GenericSubstitutionTheoremV2::LambdaBinderLifting,
                GenericSubstitutionTheoremV2::Weakening,
                GenericSubstitutionTheoremV2::LiftingCompositionCommutation,
                GenericSubstitutionTheoremV2::CaptureAvoidance,
                GenericSubstitutionTheoremV2::TypingPreservation,
                GenericSubstitutionTheoremV2::OrdinaryBetaStability,
                GenericSubstitutionTheoremV2::ProvenancePreservingDeltaStability,
                GenericSubstitutionTheoremV2::UnitStability,
                GenericSubstitutionTheoremV2::InventoriedFreshEquationStability,
                GenericSubstitutionTheoremV2::GenericPublicApplicationNaturality,
                GenericSubstitutionTheoremV2::GenericEquationActionNaturality,
            ]
        );
        assert_eq!(
            proposed.fresh_rule_matching_protocol,
            FreshRuleMatchingProtocolV2::EdgeLocalTypedLeftLinear
        );
        assert_eq!(
            proposed.typed_occurrence_census_protocol,
            TypedOccurrenceCensusProtocolV2::BinderLocalTermsAndJudgmentTypes
        );
        assert_eq!(
            proposed.historical_rewrite_protocol,
            HistoricalRewriteProtocolV2::IndependentSubjectsWithVerifiedHistory
        );
        assert_eq!(
            proposed.conservativity_protocol,
            ConservativityProtocolV2::WeakeningImage
        );
        assert_eq!(
            proposed.overlap_census_protocol,
            OverlapCensusProtocolV2::ExhaustiveImmediateEdgePairs
        );
        assert_eq!(
            proposed.q3_rule,
            Q3RuleV1::VerifiedEmptyOriginCutoffRegistry
        );
        assert!(
            !proposed
                .q0_rules
                .contains(&Q0RuleV1::DescriptorForcedProjection)
        );

        let AuditDecision::Proven(verified_v2) =
            verify_semantic_audit_lambda_unit_manifest_v2(&proposed)
        else {
            panic!("exact lambda/unit V2 semantic successor should verify");
        };
        let AuditDecision::Proven(verified_v1) = verify_semantic_audit_lambda_unit_manifest_v1(
            &proposed_semantic_audit_lambda_unit_manifest_v1(),
        ) else {
            panic!("preserved lambda/unit V1 semantic successor should verify");
        };
        assert_ne!(
            verified_v2.candidate_digest(),
            verified_v1.candidate_digest()
        );
    }

    #[test]
    fn lambda_unit_v2_manifest_is_closed() {
        let mut changed = proposed_semantic_audit_lambda_unit_manifest_v2();
        changed.generic_substitution_theorems.pop();
        assert!(matches!(
            verify_semantic_audit_lambda_unit_manifest_v2(&changed),
            AuditDecision::Unknown(AuditUnknownReason::MalformedManifest)
        ));

        let mut changed = proposed_semantic_audit_lambda_unit_manifest_v2();
        changed.q0_rules.push(Q0RuleV1::DescriptorForcedProjection);
        assert!(matches!(
            verify_semantic_audit_lambda_unit_manifest_v2(&changed),
            AuditDecision::Unknown(AuditUnknownReason::MalformedManifest)
        ));
    }

    #[test]
    fn lambda_unit_v3_is_demand_neutral_closed_and_has_no_live_authority() {
        let proposed = proposed_semantic_audit_lambda_unit_manifest_v3();
        assert_eq!(proposed.profile_id, "gf2-semantic-audit-lambda-unit-v3");
        assert_eq!(proposed.schema_version, 3);
        assert!(!proposed.frozen);
        assert!(!proposed.live_profile_a_access);
        assert_eq!(
            proposed.semantic_identity_protocol,
            SemanticIdentityProtocolV3::DemandNeutralStructuralSupport
        );
        assert_eq!(
            proposed.historical_authority_protocol,
            HistoricalAuthorityProtocolV3::EmptyBaseOrExactIssuedPriorRewriteSystem
        );
        assert_eq!(
            proposed.authority_order,
            vec![
                SemanticAuthorityStageV3::PublicClauseCensus,
                SemanticAuthorityStageV3::DemandNeutralSemanticSeedCensus,
                SemanticAuthorityStageV3::LambdaUnitTypingAndSynthesisMetatheory,
                SemanticAuthorityStageV3::NativeRankInductiveCarrier,
                SemanticAuthorityStageV3::SynthesisBackedTypedOccurrenceCensus,
                SemanticAuthorityStageV3::RewriteSystem,
                SemanticAuthorityStageV3::FamilyQuotient,
                SemanticAuthorityStageV3::WeakeningAndRestriction,
                SemanticAuthorityStageV3::MarginalFamilySet,
                SemanticAuthorityStageV3::DemandOrbitCensus,
                SemanticAuthorityStageV3::DemandRealizationCensus,
                SemanticAuthorityStageV3::Sr2ProvenanceAssignment,
            ]
        );

        let AuditDecision::Proven(verified) =
            verify_semantic_audit_lambda_unit_manifest_v3(&proposed)
        else {
            panic!("exact V3 proposal should verify");
        };
        assert!(!verified.manifest().frozen);
        assert!(!verified.manifest().live_profile_a_access);

        let mut changed = proposed;
        changed.authority_order.swap(1, 9);
        assert!(matches!(
            verify_semantic_audit_lambda_unit_manifest_v3(&changed),
            AuditDecision::Unknown(AuditUnknownReason::MalformedManifest)
        ));
    }

    #[test]
    fn lambda_unit_v3_fail_closed_reasons_have_stable_wire_names() {
        let cases = [
            (
                AuditUnknownReason::MissingLambdaUnitTypingMetatheory,
                "\"missing_lambda_unit_typing_metatheory\"",
            ),
            (
                AuditUnknownReason::MissingDemandNeutralSemanticSeedCensus,
                "\"missing_demand_neutral_semantic_seed_census\"",
            ),
            (
                AuditUnknownReason::MissingNativeRankInductiveCarrierV3,
                "\"missing_native_rank_inductive_carrier_v3\"",
            ),
            (
                AuditUnknownReason::MissingSynthesisBackedTypedOccurrenceCensus,
                "\"missing_synthesis_backed_typed_occurrence_census\"",
            ),
            (
                AuditUnknownReason::MissingRewriteSystemV3,
                "\"missing_rewrite_system_v3\"",
            ),
            (
                AuditUnknownReason::MissingFamilyQuotientV3,
                "\"missing_family_quotient_v3\"",
            ),
            (
                AuditUnknownReason::MissingWeakeningMarginalAuthority,
                "\"missing_weakening_marginal_authority\"",
            ),
            (
                AuditUnknownReason::MissingDemandOrbitCensusV2,
                "\"missing_demand_orbit_census_v2\"",
            ),
            (
                AuditUnknownReason::MissingDemandRealizationCensusV2,
                "\"missing_demand_realization_census_v2\"",
            ),
            (
                AuditUnknownReason::MissingSr2ProvenanceAssignment,
                "\"missing_sr2_provenance_assignment\"",
            ),
            (
                AuditUnknownReason::MissingIssuedPriorRewriteSystemV3,
                "\"missing_issued_prior_rewrite_system_v3\"",
            ),
            (
                AuditUnknownReason::DemandAuthorityEnteredStructuralIdentity,
                "\"demand_authority_entered_structural_identity\"",
            ),
        ];

        for (reason, expected) in cases {
            assert_eq!(
                serde_json::to_string(&reason).expect("unknown reason should serialize"),
                expected
            );
        }
    }

    #[test]
    fn lambda_unit_v2_fail_closed_reasons_have_stable_wire_names() {
        let cases = [
            (
                AuditUnknownReason::MissingSubstitutionMetatheory,
                "\"missing_substitution_metatheory\"",
            ),
            (
                AuditUnknownReason::MissingDemandOrbitAuthority,
                "\"missing_demand_orbit_authority\"",
            ),
            (
                AuditUnknownReason::MissingDemandRealizationAuthority,
                "\"missing_demand_realization_authority\"",
            ),
            (
                AuditUnknownReason::MissingSemanticSeedCensus,
                "\"missing_semantic_seed_census\"",
            ),
            (
                AuditUnknownReason::MissingRankInductiveCarrierTheorem,
                "\"missing_rank_inductive_carrier_theorem\"",
            ),
            (
                AuditUnknownReason::MissingTypedOccurrenceCensus,
                "\"missing_typed_occurrence_census\"",
            ),
            (
                AuditUnknownReason::MissingHistoricalRewriteAuthority,
                "\"missing_historical_rewrite_authority\"",
            ),
            (
                AuditUnknownReason::MissingOverlapCensus,
                "\"missing_overlap_census\"",
            ),
            (
                AuditUnknownReason::MissingWeakeningImageConservativity,
                "\"missing_weakening_image_conservativity\"",
            ),
        ];

        for (reason, expected) in cases {
            assert_eq!(
                serde_json::to_string(&reason).expect("unknown reason should serialize"),
                expected
            );
        }
    }
}
