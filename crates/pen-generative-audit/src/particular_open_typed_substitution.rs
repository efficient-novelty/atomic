//! Particular, replayed open typed substitutions and context morphisms.
//!
//! This module deliberately stops below generic substitution metatheory.  A
//! successful base value records one exact instance checked by two aggregate
//! replays. Identity, composition, binder lift, and judgment reindexing
//! constructors derive and independently recheck one output each; no finite
//! family of those values is universal theorem authority.

use crate::{
    VerifiedGenerativeStructuralOccurrenceGrammarV1,
    generative_capability_kernel_configuration_digest_v1,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, Kernel, KernelError, OpenJudgment,
    ResourceKind, Term, UncheckedSignature, VerifiedSignature,
};

pub const PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_SCHEMA_VERSION_V1: u16 = 1;
pub const PARTICULAR_OPEN_TYPED_SUBSTITUTION_RESOURCE_POLICY_VERSION_V1: u16 = 1;
pub const PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_RULE_COUNT_V1: usize = 16;
/// Reviewed allocation/work ceiling; a caller cannot enlarge it by supplying
/// a more permissive kernel configuration.
pub const PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_OPERATIONS_V1: u32 = 100_000;
pub const PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_DEPTH_V1: u16 = 256;
pub const PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_NORMALIZATION_FUEL_V1: u32 = 50_000;

const PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b1/particular-open-typed-substitution-definition/v1";
const PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_MANIFEST_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b1/particular-open-typed-substitution-protocol/v1";
const PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROPOSAL_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b1/particular-open-typed-substitution-proposal/v1";
const PARTICULAR_OPEN_TYPED_SUBSTITUTION_CANONICAL_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b1/particular-open-typed-substitution-canonical/v1";
const PARTICULAR_OPEN_TYPED_SUBSTITUTION_EVIDENCE_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b1/particular-open-typed-substitution-evidence/v1";
const PARTICULAR_OPEN_TYPED_SUBSTITUTION_IMAGE_EVIDENCE_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b1/particular-open-typed-substitution-image-evidence/v1";
const PARTICULAR_IDENTITY_SUBSTITUTION_EVIDENCE_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b1/particular-identity-substitution-evidence/v1";
const PARTICULAR_COMPOSITE_SUBSTITUTION_EVIDENCE_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b1/particular-composite-substitution-evidence/v1";
const PARTICULAR_BINDER_LIFT_SUBSTITUTION_EVIDENCE_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b1/particular-binder-lift-substitution-evidence/v1";
const PARTICULAR_REINDEXED_OPEN_JUDGMENT_EVIDENCE_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b1/particular-reindexed-open-judgment-evidence/v1";

const PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_ROOT_TAG_V1: u8 = 0xf5;
const PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_MANIFEST_ROOT_TAG_V1: u8 = 0xf6;
const PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROPOSAL_ROOT_TAG_V1: u8 = 0xf7;

/// Closed JG2b2b1 protocol vocabulary. There is no caller extension point.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ParticularOpenTypedSubstitutionProtocolRuleV1 {
    EvidenceIsParticularNotUniversal,
    MorphismOrientationIsDomainDeltaToCodomainGamma,
    ImagesAreCompleteAndOldestCodomainEntryFirst,
    ExpectedImageTypesDeriveFromPriorImages,
    RawAndNormalizedProposalsReceiveSeparateAggregateKernelReplays,
    AllContextsImagesAndJudgmentsAreKernelChecked,
    ExactInputsAndNormalizedReplayEvidenceRemainSeparate,
    IdentityCompositeLiftAndReindexOutputsAreInternallyDerived,
    EveryDerivedOutputIsIndependentlyRechecked,
    CallerExpectedTypesOutputsDigestsAndBooleansAreForbidden,
    AggregateResourcesFailClosedWithoutPartialEvidence,
    KernelLimitsDoNotExceedReviewedHardCeilings,
    ExhaustionMintsNoPositiveOrNegativeFact,
    ParticularEvidenceNeverEstablishesGenericLaws,
    CallerMintedParticularEvidenceIsNeverCensusInput,
    ProtocolMintsNoFactualCensusInterfaceFunctorNaturalityOrDownstreamAuthority,
}

pub const PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_RULES_V1:
    [ParticularOpenTypedSubstitutionProtocolRuleV1;
        PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_RULE_COUNT_V1] = [
    ParticularOpenTypedSubstitutionProtocolRuleV1::EvidenceIsParticularNotUniversal,
    ParticularOpenTypedSubstitutionProtocolRuleV1::MorphismOrientationIsDomainDeltaToCodomainGamma,
    ParticularOpenTypedSubstitutionProtocolRuleV1::ImagesAreCompleteAndOldestCodomainEntryFirst,
    ParticularOpenTypedSubstitutionProtocolRuleV1::ExpectedImageTypesDeriveFromPriorImages,
    ParticularOpenTypedSubstitutionProtocolRuleV1::RawAndNormalizedProposalsReceiveSeparateAggregateKernelReplays,
    ParticularOpenTypedSubstitutionProtocolRuleV1::AllContextsImagesAndJudgmentsAreKernelChecked,
    ParticularOpenTypedSubstitutionProtocolRuleV1::ExactInputsAndNormalizedReplayEvidenceRemainSeparate,
    ParticularOpenTypedSubstitutionProtocolRuleV1::IdentityCompositeLiftAndReindexOutputsAreInternallyDerived,
    ParticularOpenTypedSubstitutionProtocolRuleV1::EveryDerivedOutputIsIndependentlyRechecked,
    ParticularOpenTypedSubstitutionProtocolRuleV1::CallerExpectedTypesOutputsDigestsAndBooleansAreForbidden,
    ParticularOpenTypedSubstitutionProtocolRuleV1::AggregateResourcesFailClosedWithoutPartialEvidence,
    ParticularOpenTypedSubstitutionProtocolRuleV1::KernelLimitsDoNotExceedReviewedHardCeilings,
    ParticularOpenTypedSubstitutionProtocolRuleV1::ExhaustionMintsNoPositiveOrNegativeFact,
    ParticularOpenTypedSubstitutionProtocolRuleV1::ParticularEvidenceNeverEstablishesGenericLaws,
    ParticularOpenTypedSubstitutionProtocolRuleV1::CallerMintedParticularEvidenceIsNeverCensusInput,
    ParticularOpenTypedSubstitutionProtocolRuleV1::ProtocolMintsNoFactualCensusInterfaceFunctorNaturalityOrDownstreamAuthority,
];

impl CanonicalEncode for ParticularOpenTypedSubstitutionProtocolRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::EvidenceIsParticularNotUniversal => 0x60,
            Self::MorphismOrientationIsDomainDeltaToCodomainGamma => 0x61,
            Self::ImagesAreCompleteAndOldestCodomainEntryFirst => 0x62,
            Self::ExpectedImageTypesDeriveFromPriorImages => 0x63,
            Self::RawAndNormalizedProposalsReceiveSeparateAggregateKernelReplays => 0x64,
            Self::AllContextsImagesAndJudgmentsAreKernelChecked => 0x65,
            Self::ExactInputsAndNormalizedReplayEvidenceRemainSeparate => 0x66,
            Self::IdentityCompositeLiftAndReindexOutputsAreInternallyDerived => 0x67,
            Self::EveryDerivedOutputIsIndependentlyRechecked => 0x68,
            Self::CallerExpectedTypesOutputsDigestsAndBooleansAreForbidden => 0x69,
            Self::AggregateResourcesFailClosedWithoutPartialEvidence => 0x6a,
            Self::KernelLimitsDoNotExceedReviewedHardCeilings => 0x6b,
            Self::ExhaustionMintsNoPositiveOrNegativeFact => 0x6c,
            Self::ParticularEvidenceNeverEstablishesGenericLaws => 0x6d,
            Self::CallerMintedParticularEvidenceIsNeverCensusInput => 0x6e,
            Self::ProtocolMintsNoFactualCensusInterfaceFunctorNaturalityOrDownstreamAuthority => {
                0x6f
            }
        });
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticularOpenTypedSubstitutionProtocolDefinitionV1 {
    pub schema_version: u16,
    pub resource_policy_version: u16,
    pub max_operations: u32,
    pub max_depth: u16,
    pub max_normalization_fuel: u32,
    pub rules: Vec<ParticularOpenTypedSubstitutionProtocolRuleV1>,
}

impl CanonicalEncode for ParticularOpenTypedSubstitutionProtocolDefinitionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_ROOT_TAG_V1);
        encoder.u16(self.schema_version);
        encoder.u16(self.resource_policy_version);
        encoder.u32(self.max_operations);
        encoder.u16(self.max_depth);
        encoder.u32(self.max_normalization_fuel);
        encoder.sequence(&self.rules);
    }
}

pub fn proposed_particular_open_typed_substitution_protocol_definition_v1()
-> ParticularOpenTypedSubstitutionProtocolDefinitionV1 {
    ParticularOpenTypedSubstitutionProtocolDefinitionV1 {
        schema_version: PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_SCHEMA_VERSION_V1,
        resource_policy_version: PARTICULAR_OPEN_TYPED_SUBSTITUTION_RESOURCE_POLICY_VERSION_V1,
        max_operations: PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_OPERATIONS_V1,
        max_depth: PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_DEPTH_V1,
        max_normalization_fuel: PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_NORMALIZATION_FUEL_V1,
        rules: PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_RULES_V1.to_vec(),
    }
}

// Frozen canonical transcript; a regression test derives these bytes from the definition.
pub const CANONICAL_PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_BYTES_V1: &[u8] = &[
    0xf5, 0x01, 0x00, 0x01, 0x00, 0xa0, 0x86, 0x01, 0x00, 0x00, 0x01, 0x50, 0xc3, 0x00, 0x00, 0x10,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68,
    0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f,
];
pub const CANONICAL_PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_DIGEST_V1: &str =
    "blake3:460193d192ebab1a0c1d30227026c2c6e1cf23c069449939a6f4e3b3645518cb";
pub const SYNTHETIC_PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_MANIFEST_LENGTH_V1: usize = 672;
pub const SYNTHETIC_PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_MANIFEST_DIGEST_V1: &str =
    "blake3:3a0c35e6c930450326cd4129d61db8cb60601a6cbab723ff0cd93cddd624d436";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParticularOpenTypedSubstitutionProtocolManifestV1 {
    pub definition: ParticularOpenTypedSubstitutionProtocolDefinitionV1,
    pub structural_grammar_manifest_digest: Digest,
    pub substitution_naturality_protocol_manifest_digest: Digest,
    pub jg1_manifest_digest: Digest,
    pub constructor_grammar_manifest_digest: Digest,
    pub scope_grammar_digest: Digest,
    pub kernel_protocol_digest: Digest,
    pub normalizer_protocol_digest: Digest,
    pub kernel_configuration_digest: Digest,
}

impl CanonicalEncode for ParticularOpenTypedSubstitutionProtocolManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_MANIFEST_ROOT_TAG_V1);
        self.definition.encode_canonical(encoder);
        self.structural_grammar_manifest_digest
            .encode_canonical(encoder);
        self.substitution_naturality_protocol_manifest_digest
            .encode_canonical(encoder);
        self.jg1_manifest_digest.encode_canonical(encoder);
        self.constructor_grammar_manifest_digest
            .encode_canonical(encoder);
        self.scope_grammar_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.kernel_configuration_digest.encode_canonical(encoder);
    }
}

pub fn proposed_particular_open_typed_substitution_protocol_v1(
    grammar: &VerifiedGenerativeStructuralOccurrenceGrammarV1,
    kernel: &Kernel,
) -> ParticularOpenTypedSubstitutionProtocolManifestV1 {
    ParticularOpenTypedSubstitutionProtocolManifestV1 {
        definition: proposed_particular_open_typed_substitution_protocol_definition_v1(),
        structural_grammar_manifest_digest: grammar.manifest_digest().clone(),
        substitution_naturality_protocol_manifest_digest: grammar
            .substitution_naturality_protocol_manifest_digest()
            .clone(),
        jg1_manifest_digest: grammar.jg1_manifest_digest().clone(),
        constructor_grammar_manifest_digest: grammar.constructor_grammar_manifest_digest().clone(),
        scope_grammar_digest: grammar.scope_grammar_digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        kernel_configuration_digest: generative_capability_kernel_configuration_digest_v1(kernel),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParticularOpenTypedSubstitutionProtocolFailureV1 {
    SchemaVersionMismatch,
    ResourcePolicyVersionMismatch,
    ResourceCeilingMismatch,
    RuleVocabularyMismatch,
    CanonicalDefinitionTranscriptMismatch,
    CanonicalDefinitionDigestMismatch,
    GrammarKernelProtocolMismatch,
    GrammarNormalizerProtocolMismatch,
    GrammarKernelConfigurationMismatch,
    StructuralGrammarManifestDigestMismatch,
    SubstitutionNaturalityProtocolManifestDigestMismatch,
    Jg1ManifestDigestMismatch,
    ConstructorGrammarManifestDigestMismatch,
    ScopeGrammarDigestMismatch,
    KernelProtocolMismatch,
    NormalizerProtocolMismatch,
    KernelConfigurationMismatch,
}

impl std::fmt::Display for ParticularOpenTypedSubstitutionProtocolFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::SchemaVersionMismatch => "the particular substitution schema is not frozen V1",
            Self::ResourcePolicyVersionMismatch => {
                "the particular substitution resource policy is not frozen V1"
            }
            Self::ResourceCeilingMismatch => {
                "the particular substitution resource ceilings differ from reviewed V1"
            }
            Self::RuleVocabularyMismatch => {
                "the particular substitution rule vocabulary or order differs"
            }
            Self::CanonicalDefinitionTranscriptMismatch => {
                "the particular substitution definition transcript differs"
            }
            Self::CanonicalDefinitionDigestMismatch => {
                "the particular substitution definition digest differs"
            }
            Self::GrammarKernelProtocolMismatch => {
                "the structural grammar is bound to another kernel protocol"
            }
            Self::GrammarNormalizerProtocolMismatch => {
                "the structural grammar is bound to another normalizer protocol"
            }
            Self::GrammarKernelConfigurationMismatch => {
                "the structural grammar is bound to another kernel configuration"
            }
            Self::StructuralGrammarManifestDigestMismatch => {
                "the protocol proposal is bound to another structural grammar"
            }
            Self::SubstitutionNaturalityProtocolManifestDigestMismatch => {
                "the protocol proposal is bound to another JG2b2a protocol"
            }
            Self::Jg1ManifestDigestMismatch => {
                "the protocol proposal is bound to another JG1 grammar"
            }
            Self::ConstructorGrammarManifestDigestMismatch => {
                "the protocol proposal is bound to another constructor grammar"
            }
            Self::ScopeGrammarDigestMismatch => {
                "the protocol proposal is bound to another scope grammar"
            }
            Self::KernelProtocolMismatch => {
                "the protocol proposal is bound to another kernel protocol"
            }
            Self::NormalizerProtocolMismatch => {
                "the protocol proposal is bound to another normalizer protocol"
            }
            Self::KernelConfigurationMismatch => {
                "the protocol proposal is bound to another kernel configuration"
            }
        })
    }
}

impl std::error::Error for ParticularOpenTypedSubstitutionProtocolFailureV1 {}

/// Opaque remintable authority for the exact JG2b2b1 protocol only.
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularOpenTypedSubstitutionProtocolV1;
/// let _forged = VerifiedParticularOpenTypedSubstitutionProtocolV1 {};
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularOpenTypedSubstitutionProtocolV1;
/// let _forged: VerifiedParticularOpenTypedSubstitutionProtocolV1 = Default::default();
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularOpenTypedSubstitutionProtocolV1;
/// use pen_kernel::Digest;
/// let _forged = VerifiedParticularOpenTypedSubstitutionProtocolV1::from_digest(
///     Digest::of_bytes(b"identifier-not-evidence")
/// );
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularOpenTypedSubstitutionProtocolV1;
/// let _forged: VerifiedParticularOpenTypedSubstitutionProtocolV1 =
///     serde_json::from_str("{}").unwrap();
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedParticularOpenTypedSubstitutionProtocolV1 {
    schema_version: u16,
    resource_policy_version: u16,
    definition_digest: Digest,
    structural_grammar_manifest_digest: Digest,
    substitution_naturality_protocol_manifest_digest: Digest,
    jg1_manifest_digest: Digest,
    constructor_grammar_manifest_digest: Digest,
    scope_grammar_digest: Digest,
    kernel_protocol_digest: Digest,
    normalizer_protocol_digest: Digest,
    kernel_configuration_digest: Digest,
    manifest_digest: Digest,
}

impl VerifiedParticularOpenTypedSubstitutionProtocolV1 {
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }
    pub const fn resource_policy_version(&self) -> u16 {
        self.resource_policy_version
    }
    pub fn rules(&self) -> &'static [ParticularOpenTypedSubstitutionProtocolRuleV1] {
        &PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_RULES_V1
    }
    pub fn definition_digest(&self) -> &Digest {
        &self.definition_digest
    }
    pub fn structural_grammar_manifest_digest(&self) -> &Digest {
        &self.structural_grammar_manifest_digest
    }
    pub fn substitution_naturality_protocol_manifest_digest(&self) -> &Digest {
        &self.substitution_naturality_protocol_manifest_digest
    }
    pub fn jg1_manifest_digest(&self) -> &Digest {
        &self.jg1_manifest_digest
    }
    pub fn constructor_grammar_manifest_digest(&self) -> &Digest {
        &self.constructor_grammar_manifest_digest
    }
    pub fn scope_grammar_digest(&self) -> &Digest {
        &self.scope_grammar_digest
    }
    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }
    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.normalizer_protocol_digest
    }
    pub fn kernel_configuration_digest(&self) -> &Digest {
        &self.kernel_configuration_digest
    }
    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }
}

pub fn verify_particular_open_typed_substitution_protocol_v1(
    grammar: &VerifiedGenerativeStructuralOccurrenceGrammarV1,
    kernel: &Kernel,
    manifest: &ParticularOpenTypedSubstitutionProtocolManifestV1,
) -> Result<
    VerifiedParticularOpenTypedSubstitutionProtocolV1,
    ParticularOpenTypedSubstitutionProtocolFailureV1,
> {
    let definition = &manifest.definition;
    if definition.schema_version != PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_SCHEMA_VERSION_V1 {
        return Err(ParticularOpenTypedSubstitutionProtocolFailureV1::SchemaVersionMismatch);
    }
    if definition.resource_policy_version
        != PARTICULAR_OPEN_TYPED_SUBSTITUTION_RESOURCE_POLICY_VERSION_V1
    {
        return Err(
            ParticularOpenTypedSubstitutionProtocolFailureV1::ResourcePolicyVersionMismatch,
        );
    }
    if definition.max_operations != PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_OPERATIONS_V1
        || definition.max_depth != PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_DEPTH_V1
        || definition.max_normalization_fuel
            != PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_NORMALIZATION_FUEL_V1
    {
        return Err(ParticularOpenTypedSubstitutionProtocolFailureV1::ResourceCeilingMismatch);
    }
    if definition.rules.as_slice() != PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_RULES_V1 {
        return Err(ParticularOpenTypedSubstitutionProtocolFailureV1::RuleVocabularyMismatch);
    }
    let mut definition_encoder = CanonicalEncoder::new();
    definition.encode_canonical(&mut definition_encoder);
    if definition_encoder.as_bytes()
        != CANONICAL_PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_BYTES_V1
    {
        return Err(
            ParticularOpenTypedSubstitutionProtocolFailureV1::CanonicalDefinitionTranscriptMismatch,
        );
    }
    let definition_digest = Digest::of_domain_bytes(
        PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_DIGEST_DOMAIN_V1,
        definition_encoder.as_bytes(),
    );
    if definition_digest.as_str()
        != CANONICAL_PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_DIGEST_V1
    {
        return Err(
            ParticularOpenTypedSubstitutionProtocolFailureV1::CanonicalDefinitionDigestMismatch,
        );
    }

    let limits = kernel.limits();
    if limits.max_operations > PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_OPERATIONS_V1
        || limits.max_depth > PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_DEPTH_V1
        || limits.normalization_fuel > PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_NORMALIZATION_FUEL_V1
    {
        return Err(ParticularOpenTypedSubstitutionProtocolFailureV1::ResourceCeilingMismatch);
    }
    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    let normalizer_protocol_digest = kernel.normalizer_protocol_digest();
    let kernel_configuration_digest = generative_capability_kernel_configuration_digest_v1(kernel);
    if grammar.kernel_protocol_digest() != &kernel_protocol_digest {
        return Err(
            ParticularOpenTypedSubstitutionProtocolFailureV1::GrammarKernelProtocolMismatch,
        );
    }
    if grammar.normalizer_protocol_digest() != &normalizer_protocol_digest {
        return Err(
            ParticularOpenTypedSubstitutionProtocolFailureV1::GrammarNormalizerProtocolMismatch,
        );
    }
    if grammar.kernel_configuration_digest() != &kernel_configuration_digest {
        return Err(
            ParticularOpenTypedSubstitutionProtocolFailureV1::GrammarKernelConfigurationMismatch,
        );
    }
    if manifest.structural_grammar_manifest_digest != *grammar.manifest_digest() {
        return Err(
            ParticularOpenTypedSubstitutionProtocolFailureV1::StructuralGrammarManifestDigestMismatch,
        );
    }
    if manifest.substitution_naturality_protocol_manifest_digest
        != *grammar.substitution_naturality_protocol_manifest_digest()
    {
        return Err(
            ParticularOpenTypedSubstitutionProtocolFailureV1::SubstitutionNaturalityProtocolManifestDigestMismatch,
        );
    }
    if manifest.jg1_manifest_digest != *grammar.jg1_manifest_digest() {
        return Err(ParticularOpenTypedSubstitutionProtocolFailureV1::Jg1ManifestDigestMismatch);
    }
    if manifest.constructor_grammar_manifest_digest
        != *grammar.constructor_grammar_manifest_digest()
    {
        return Err(
            ParticularOpenTypedSubstitutionProtocolFailureV1::ConstructorGrammarManifestDigestMismatch,
        );
    }
    if manifest.scope_grammar_digest != *grammar.scope_grammar_digest() {
        return Err(ParticularOpenTypedSubstitutionProtocolFailureV1::ScopeGrammarDigestMismatch);
    }
    if manifest.kernel_protocol_digest != kernel_protocol_digest {
        return Err(ParticularOpenTypedSubstitutionProtocolFailureV1::KernelProtocolMismatch);
    }
    if manifest.normalizer_protocol_digest != normalizer_protocol_digest {
        return Err(ParticularOpenTypedSubstitutionProtocolFailureV1::NormalizerProtocolMismatch);
    }
    if manifest.kernel_configuration_digest != kernel_configuration_digest {
        return Err(ParticularOpenTypedSubstitutionProtocolFailureV1::KernelConfigurationMismatch);
    }
    let manifest_digest = Digest::of_canonical(
        PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_MANIFEST_DIGEST_DOMAIN_V1,
        manifest,
    );
    Ok(VerifiedParticularOpenTypedSubstitutionProtocolV1 {
        schema_version: definition.schema_version,
        resource_policy_version: definition.resource_policy_version,
        definition_digest,
        structural_grammar_manifest_digest: manifest.structural_grammar_manifest_digest.clone(),
        substitution_naturality_protocol_manifest_digest: manifest
            .substitution_naturality_protocol_manifest_digest
            .clone(),
        jg1_manifest_digest: manifest.jg1_manifest_digest.clone(),
        constructor_grammar_manifest_digest: manifest.constructor_grammar_manifest_digest.clone(),
        scope_grammar_digest: manifest.scope_grammar_digest.clone(),
        kernel_protocol_digest,
        normalizer_protocol_digest,
        kernel_configuration_digest,
        manifest_digest,
    })
}

/// Unverified input for one morphism `theta : Delta => Gamma`.
///
/// `domain_context` is `Delta`, where every image lives. `codomain_context`
/// is `Gamma`, whose declarations are replaced. Images are complete and in
/// oldest-first `Gamma` declaration order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedParticularOpenTypedSubstitutionV1 {
    pub domain_context: DependentContext,
    pub codomain_context: DependentContext,
    pub images: Vec<Term>,
}

impl CanonicalEncode for ProposedParticularOpenTypedSubstitutionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROPOSAL_ROOT_TAG_V1);
        self.domain_context.encode_canonical(encoder);
        self.codomain_context.encode_canonical(encoder);
        encoder.sequence(&self.images);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ParticularOpenTypedSubstitutionAuthorityBindingV1 {
    schema_version: u16,
    resource_policy_version: u16,
    protocol_manifest_digest: Digest,
    structural_grammar_manifest_digest: Digest,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    normalizer_protocol_digest: Digest,
    kernel_configuration_digest: Digest,
}

impl CanonicalEncode for ParticularOpenTypedSubstitutionAuthorityBindingV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        encoder.u16(self.resource_policy_version);
        self.protocol_manifest_digest.encode_canonical(encoder);
        self.structural_grammar_manifest_digest
            .encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.kernel_configuration_digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedParticularOpenTypedSubstitutionImageV1 {
    ordinal: u32,
    raw_judgment: OpenJudgment,
    raw_normalized_judgment: OpenJudgment,
    canonical_input_judgment: OpenJudgment,
    canonical_normalized_judgment: OpenJudgment,
    evidence_digest: Digest,
}

impl VerifiedParticularOpenTypedSubstitutionImageV1 {
    pub const fn ordinal(&self) -> u32 {
        self.ordinal
    }
    pub fn raw_judgment(&self) -> &OpenJudgment {
        &self.raw_judgment
    }
    pub fn raw_normalized_judgment(&self) -> &OpenJudgment {
        &self.raw_normalized_judgment
    }
    pub fn canonical_input_judgment(&self) -> &OpenJudgment {
        &self.canonical_input_judgment
    }
    pub fn canonical_normalized_judgment(&self) -> &OpenJudgment {
        &self.canonical_normalized_judgment
    }
    pub fn evidence_digest(&self) -> &Digest {
        &self.evidence_digest
    }
}

impl CanonicalEncode for VerifiedParticularOpenTypedSubstitutionImageV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u32(self.ordinal);
        self.raw_judgment.encode_canonical(encoder);
        self.raw_normalized_judgment.encode_canonical(encoder);
        self.canonical_input_judgment.encode_canonical(encoder);
        self.canonical_normalized_judgment.encode_canonical(encoder);
    }
}

/// Opaque evidence for one exact two-pass checked context morphism.
///
/// The canonical boundary consists of replay-stable normalized contexts and
/// image terms. Substitution can create beta redexes in an internally derived
/// expected type, so both the exact pass-two input judgment and its separately
/// normalized kernel output are retained instead of being conflated.
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularOpenTypedSubstitutionV1;
/// let _forged = VerifiedParticularOpenTypedSubstitutionV1 {};
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularOpenTypedSubstitutionV1;
/// let _forged: VerifiedParticularOpenTypedSubstitutionV1 = Default::default();
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularOpenTypedSubstitutionV1;
/// use pen_kernel::Digest;
/// let _forged = VerifiedParticularOpenTypedSubstitutionV1::from_digest(
///     Digest::of_bytes(b"identifier-not-evidence")
/// );
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularOpenTypedSubstitutionV1;
/// let _forged: VerifiedParticularOpenTypedSubstitutionV1 =
///     serde_json::from_str("{}").unwrap();
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedParticularOpenTypedSubstitutionV1 {
    binding: ParticularOpenTypedSubstitutionAuthorityBindingV1,
    normalized_signature_wire: UncheckedSignature,
    raw_proposal: ProposedParticularOpenTypedSubstitutionV1,
    raw_proposal_digest: Digest,
    canonical_proposal: ProposedParticularOpenTypedSubstitutionV1,
    raw_codomain_context_judgment: OpenJudgment,
    raw_domain_context_judgment: OpenJudgment,
    normalized_codomain_context_judgment: OpenJudgment,
    normalized_domain_context_judgment: OpenJudgment,
    canonical_codomain_context_judgment: OpenJudgment,
    canonical_domain_context_judgment: OpenJudgment,
    images: Vec<VerifiedParticularOpenTypedSubstitutionImageV1>,
    canonical_digest: Digest,
    evidence_digest: Digest,
}

impl VerifiedParticularOpenTypedSubstitutionV1 {
    pub const fn schema_version(&self) -> u16 {
        self.binding.schema_version
    }
    pub const fn resource_policy_version(&self) -> u16 {
        self.binding.resource_policy_version
    }
    pub fn domain_context(&self) -> &DependentContext {
        &self.canonical_proposal.domain_context
    }
    pub fn codomain_context(&self) -> &DependentContext {
        &self.canonical_proposal.codomain_context
    }
    pub fn images(&self) -> &[Term] {
        &self.canonical_proposal.images
    }
    pub fn canonical_images(&self) -> &[Term] {
        &self.canonical_proposal.images
    }
    pub fn raw_proposal(&self) -> &ProposedParticularOpenTypedSubstitutionV1 {
        &self.raw_proposal
    }
    pub fn normalized_signature_wire(&self) -> &UncheckedSignature {
        &self.normalized_signature_wire
    }
    pub fn canonical_proposal(&self) -> &ProposedParticularOpenTypedSubstitutionV1 {
        &self.canonical_proposal
    }
    pub fn image_evidence(&self) -> &[VerifiedParticularOpenTypedSubstitutionImageV1] {
        &self.images
    }
    pub fn raw_codomain_context_judgment(&self) -> &OpenJudgment {
        &self.raw_codomain_context_judgment
    }
    pub fn raw_domain_context_judgment(&self) -> &OpenJudgment {
        &self.raw_domain_context_judgment
    }
    pub fn normalized_codomain_context_judgment(&self) -> &OpenJudgment {
        &self.normalized_codomain_context_judgment
    }
    pub fn normalized_domain_context_judgment(&self) -> &OpenJudgment {
        &self.normalized_domain_context_judgment
    }
    pub fn canonical_codomain_context_judgment(&self) -> &OpenJudgment {
        &self.canonical_codomain_context_judgment
    }
    pub fn canonical_domain_context_judgment(&self) -> &OpenJudgment {
        &self.canonical_domain_context_judgment
    }
    pub fn signature_digest(&self) -> &Digest {
        &self.binding.signature_digest
    }
    pub fn protocol_manifest_digest(&self) -> &Digest {
        &self.binding.protocol_manifest_digest
    }
    pub fn structural_grammar_manifest_digest(&self) -> &Digest {
        &self.binding.structural_grammar_manifest_digest
    }
    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.binding.kernel_protocol_digest
    }
    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.binding.normalizer_protocol_digest
    }
    pub fn kernel_configuration_digest(&self) -> &Digest {
        &self.binding.kernel_configuration_digest
    }
    pub fn raw_proposal_digest(&self) -> &Digest {
        &self.raw_proposal_digest
    }
    pub fn canonical_digest(&self) -> &Digest {
        &self.canonical_digest
    }
    pub fn evidence_digest(&self) -> &Digest {
        &self.evidence_digest
    }
}

impl CanonicalEncode for VerifiedParticularOpenTypedSubstitutionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.binding.encode_canonical(encoder);
        self.normalized_signature_wire.encode_canonical(encoder);
        self.raw_proposal.encode_canonical(encoder);
        self.raw_proposal_digest.encode_canonical(encoder);
        self.canonical_proposal.encode_canonical(encoder);
        self.raw_codomain_context_judgment.encode_canonical(encoder);
        self.raw_domain_context_judgment.encode_canonical(encoder);
        self.normalized_codomain_context_judgment
            .encode_canonical(encoder);
        self.normalized_domain_context_judgment
            .encode_canonical(encoder);
        self.canonical_codomain_context_judgment
            .encode_canonical(encoder);
        self.canonical_domain_context_judgment
            .encode_canonical(encoder);
        encoder.sequence(&self.images);
        self.canonical_digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParticularOpenTypedSubstitutionFailureV1 {
    ProtocolKernelMismatch,
    ProtocolNormalizerMismatch,
    ProtocolKernelConfigurationMismatch,
    KernelLimitsExceedReviewedCeilings,
    SignatureMismatch,
    ImageArityMismatch,
    ContextLengthOverflow,
    VariableIndexOverflow,
    VariableOutOfScope,
    OperationBudgetExhausted,
    DepthBudgetExhausted,
    AllocationFailure,
    AggregateMaterialLimitExceeded,
    KernelResourceExhausted(ResourceKind),
    KernelRejected(KernelError),
    UnexpectedKernelJudgment,
    CanonicalReplayMismatch,
    IntermediateContextMismatch,
    JudgmentContextMismatch,
}

impl std::fmt::Display for ParticularOpenTypedSubstitutionFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProtocolKernelMismatch => {
                formatter.write_str("the protocol is bound to another kernel")
            }
            Self::ProtocolNormalizerMismatch => {
                formatter.write_str("the protocol is bound to another normalizer")
            }
            Self::ProtocolKernelConfigurationMismatch => {
                formatter.write_str("the protocol is bound to another kernel configuration")
            }
            Self::KernelLimitsExceedReviewedCeilings => formatter
                .write_str("kernel limits exceed the reviewed particular-substitution ceilings"),
            Self::SignatureMismatch => formatter
                .write_str("the particular evidence is bound to another verified signature"),
            Self::ImageArityMismatch => {
                formatter.write_str("the image count does not equal the codomain context length")
            }
            Self::ContextLengthOverflow => {
                formatter.write_str("a context length cannot be represented safely")
            }
            Self::VariableIndexOverflow => formatter.write_str("a de Bruijn index overflowed"),
            Self::VariableOutOfScope => formatter
                .write_str("a substituted variable is outside the declared codomain context"),
            Self::OperationBudgetExhausted => {
                formatter.write_str("local structural work exhausted its operation budget")
            }
            Self::DepthBudgetExhausted => {
                formatter.write_str("local structural work exceeded its depth budget")
            }
            Self::AllocationFailure => formatter.write_str("bounded scratch allocation failed"),
            Self::AggregateMaterialLimitExceeded => formatter
                .write_str("the aggregate replay material exceeds the reviewed operation ceiling"),
            Self::KernelResourceExhausted(kind) => {
                write!(formatter, "aggregate kernel replay exhausted {kind}")
            }
            Self::KernelRejected(error) => write!(
                formatter,
                "aggregate kernel replay rejected the proposal: {error}"
            ),
            Self::UnexpectedKernelJudgment => {
                formatter.write_str("the kernel returned a different judgment shape")
            }
            Self::CanonicalReplayMismatch => formatter
                .write_str("the separately rebuilt normalized proposal was not replay-stable"),
            Self::IntermediateContextMismatch => {
                formatter.write_str("the normalized composition boundary contexts differ")
            }
            Self::JudgmentContextMismatch => {
                formatter.write_str("the judgment context is not the substitution codomain")
            }
        }
    }
}

impl std::error::Error for ParticularOpenTypedSubstitutionFailureV1 {}

/// Full evidence for one internally generated and rechecked identity instance.
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularIdentityOpenTypedSubstitutionV1;
/// let _forged = VerifiedParticularIdentityOpenTypedSubstitutionV1 {};
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularIdentityOpenTypedSubstitutionV1;
/// let _forged: VerifiedParticularIdentityOpenTypedSubstitutionV1 = Default::default();
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularIdentityOpenTypedSubstitutionV1;
/// use pen_kernel::Digest;
/// let _forged = VerifiedParticularIdentityOpenTypedSubstitutionV1::from_digest(
///     Digest::of_bytes(b"identifier-not-evidence")
/// );
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularIdentityOpenTypedSubstitutionV1;
/// let _forged: VerifiedParticularIdentityOpenTypedSubstitutionV1 =
///     serde_json::from_str("{}").unwrap();
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedParticularIdentityOpenTypedSubstitutionV1 {
    context_input: DependentContext,
    result: VerifiedParticularOpenTypedSubstitutionV1,
    evidence_digest: Digest,
}

impl VerifiedParticularIdentityOpenTypedSubstitutionV1 {
    pub fn context_input(&self) -> &DependentContext {
        &self.context_input
    }
    pub fn result(&self) -> &VerifiedParticularOpenTypedSubstitutionV1 {
        &self.result
    }
    pub fn evidence_digest(&self) -> &Digest {
        &self.evidence_digest
    }
}

impl CanonicalEncode for VerifiedParticularIdentityOpenTypedSubstitutionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.context_input.encode_canonical(encoder);
        self.result.encode_canonical(encoder);
    }
}

/// Full evidence for one internally generated and rechecked composite.
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularCompositeOpenTypedSubstitutionV1;
/// let _forged = VerifiedParticularCompositeOpenTypedSubstitutionV1 {};
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularCompositeOpenTypedSubstitutionV1;
/// let _forged: VerifiedParticularCompositeOpenTypedSubstitutionV1 = Default::default();
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularCompositeOpenTypedSubstitutionV1;
/// use pen_kernel::Digest;
/// let _forged = VerifiedParticularCompositeOpenTypedSubstitutionV1::from_digest(
///     Digest::of_bytes(b"identifier-not-evidence")
/// );
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularCompositeOpenTypedSubstitutionV1;
/// let _forged: VerifiedParticularCompositeOpenTypedSubstitutionV1 =
///     serde_json::from_str("{}").unwrap();
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedParticularCompositeOpenTypedSubstitutionV1 {
    outer: VerifiedParticularOpenTypedSubstitutionV1,
    inner: VerifiedParticularOpenTypedSubstitutionV1,
    constructed_proposal: ProposedParticularOpenTypedSubstitutionV1,
    result: VerifiedParticularOpenTypedSubstitutionV1,
    evidence_digest: Digest,
}

impl VerifiedParticularCompositeOpenTypedSubstitutionV1 {
    pub fn outer(&self) -> &VerifiedParticularOpenTypedSubstitutionV1 {
        &self.outer
    }
    pub fn inner(&self) -> &VerifiedParticularOpenTypedSubstitutionV1 {
        &self.inner
    }
    pub fn result(&self) -> &VerifiedParticularOpenTypedSubstitutionV1 {
        &self.result
    }
    pub fn constructed_proposal(&self) -> &ProposedParticularOpenTypedSubstitutionV1 {
        &self.constructed_proposal
    }
    pub fn evidence_digest(&self) -> &Digest {
        &self.evidence_digest
    }
}

impl CanonicalEncode for VerifiedParticularCompositeOpenTypedSubstitutionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.outer.encode_canonical(encoder);
        self.inner.encode_canonical(encoder);
        self.constructed_proposal.encode_canonical(encoder);
        self.result.encode_canonical(encoder);
    }
}

/// Full evidence for one internally generated and rechecked dependent lift.
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularBinderLiftOpenTypedSubstitutionV1;
/// let _forged = VerifiedParticularBinderLiftOpenTypedSubstitutionV1 {};
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularBinderLiftOpenTypedSubstitutionV1;
/// let _forged: VerifiedParticularBinderLiftOpenTypedSubstitutionV1 = Default::default();
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularBinderLiftOpenTypedSubstitutionV1;
/// use pen_kernel::Digest;
/// let _forged = VerifiedParticularBinderLiftOpenTypedSubstitutionV1::from_digest(
///     Digest::of_bytes(b"identifier-not-evidence")
/// );
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularBinderLiftOpenTypedSubstitutionV1;
/// let _forged: VerifiedParticularBinderLiftOpenTypedSubstitutionV1 =
///     serde_json::from_str("{}").unwrap();
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedParticularBinderLiftOpenTypedSubstitutionV1 {
    substitution: VerifiedParticularOpenTypedSubstitutionV1,
    parameter_reindexing: VerifiedParticularReindexedOpenJudgmentV1,
    constructed_proposal: ProposedParticularOpenTypedSubstitutionV1,
    result: VerifiedParticularOpenTypedSubstitutionV1,
    evidence_digest: Digest,
}

impl VerifiedParticularBinderLiftOpenTypedSubstitutionV1 {
    pub fn substitution(&self) -> &VerifiedParticularOpenTypedSubstitutionV1 {
        &self.substitution
    }
    pub fn result(&self) -> &VerifiedParticularOpenTypedSubstitutionV1 {
        &self.result
    }
    pub fn parameter_reindexing(&self) -> &VerifiedParticularReindexedOpenJudgmentV1 {
        &self.parameter_reindexing
    }
    pub fn constructed_proposal(&self) -> &ProposedParticularOpenTypedSubstitutionV1 {
        &self.constructed_proposal
    }
    pub fn evidence_digest(&self) -> &Digest {
        &self.evidence_digest
    }
}

impl CanonicalEncode for VerifiedParticularBinderLiftOpenTypedSubstitutionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.substitution.encode_canonical(encoder);
        self.parameter_reindexing.encode_canonical(encoder);
        self.constructed_proposal.encode_canonical(encoder);
        self.result.encode_canonical(encoder);
    }
}

/// Full evidence for one internally derived and replayed judgment endpoint.
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularReindexedOpenJudgmentV1;
/// let _forged = VerifiedParticularReindexedOpenJudgmentV1 {};
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularReindexedOpenJudgmentV1;
/// let _forged: VerifiedParticularReindexedOpenJudgmentV1 = Default::default();
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularReindexedOpenJudgmentV1;
/// use pen_kernel::Digest;
/// let _forged = VerifiedParticularReindexedOpenJudgmentV1::from_digest(
///     Digest::of_bytes(b"identifier-not-evidence")
/// );
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedParticularReindexedOpenJudgmentV1;
/// let _forged: VerifiedParticularReindexedOpenJudgmentV1 =
///     serde_json::from_str("{}").unwrap();
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedParticularReindexedOpenJudgmentV1 {
    substitution: VerifiedParticularOpenTypedSubstitutionV1,
    original_judgment: OpenJudgment,
    derived_reindexed_judgment: OpenJudgment,
    normalized_original_judgment: OpenJudgment,
    normalized_reindexed_judgment: OpenJudgment,
    evidence_digest: Digest,
}

impl VerifiedParticularReindexedOpenJudgmentV1 {
    pub fn substitution(&self) -> &VerifiedParticularOpenTypedSubstitutionV1 {
        &self.substitution
    }
    pub fn original_judgment(&self) -> &OpenJudgment {
        &self.original_judgment
    }
    pub fn derived_reindexed_judgment(&self) -> &OpenJudgment {
        &self.derived_reindexed_judgment
    }
    pub fn normalized_original_judgment(&self) -> &OpenJudgment {
        &self.normalized_original_judgment
    }
    pub fn normalized_reindexed_judgment(&self) -> &OpenJudgment {
        &self.normalized_reindexed_judgment
    }
    pub fn evidence_digest(&self) -> &Digest {
        &self.evidence_digest
    }
}

impl CanonicalEncode for VerifiedParticularReindexedOpenJudgmentV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.substitution.encode_canonical(encoder);
        self.original_judgment.encode_canonical(encoder);
        self.derived_reindexed_judgment.encode_canonical(encoder);
        self.normalized_original_judgment.encode_canonical(encoder);
        self.normalized_reindexed_judgment.encode_canonical(encoder);
    }
}

#[derive(Clone, Copy, Debug)]
struct LocalStructuralBudgetV1 {
    operations_left: u32,
    max_depth: u16,
}

impl LocalStructuralBudgetV1 {
    fn new(kernel: &Kernel) -> Self {
        let limits = kernel.limits();
        Self {
            operations_left: limits
                .max_operations
                .min(PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_OPERATIONS_V1),
            max_depth: limits
                .max_depth
                .min(PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_DEPTH_V1),
        }
    }

    fn charge(&mut self, depth: u16) -> Result<(), ParticularOpenTypedSubstitutionFailureV1> {
        if depth > self.max_depth {
            return Err(ParticularOpenTypedSubstitutionFailureV1::DepthBudgetExhausted);
        }
        self.operations_left = self
            .operations_left
            .checked_sub(1)
            .ok_or(ParticularOpenTypedSubstitutionFailureV1::OperationBudgetExhausted)?;
        Ok(())
    }
}

fn next_structural_depth_v1(depth: u16) -> Result<u16, ParticularOpenTypedSubstitutionFailureV1> {
    depth
        .checked_add(1)
        .ok_or(ParticularOpenTypedSubstitutionFailureV1::DepthBudgetExhausted)
}

/// Iterative preflight used before cloning any caller-controlled syntax.
fn preflight_term_v1(
    term: &Term,
    budget: &mut LocalStructuralBudgetV1,
) -> Result<u32, ParticularOpenTypedSubstitutionFailureV1> {
    let mut stack = Vec::new();
    stack
        .try_reserve_exact(1)
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    stack.push((term, 0_u16));
    let mut count = 0_u32;
    while let Some((term, depth)) = stack.pop() {
        budget.charge(depth)?;
        count = count
            .checked_add(1)
            .ok_or(ParticularOpenTypedSubstitutionFailureV1::OperationBudgetExhausted)?;
        let child_depth = next_structural_depth_v1(depth)?;
        match term {
            Term::Pi { parameter, body }
            | Term::Sigma { parameter, body }
            | Term::Apply {
                function: parameter,
                argument: body,
            } => {
                stack
                    .try_reserve(2)
                    .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
                stack.push((body, child_depth));
                stack.push((parameter, child_depth));
            }
            Term::Lambda {
                parameter_type,
                body,
            } => {
                stack
                    .try_reserve(2)
                    .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
                stack.push((body, child_depth));
                stack.push((parameter_type, child_depth));
            }
            Term::Pair {
                sigma_type,
                first,
                second,
            } => {
                stack
                    .try_reserve(3)
                    .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
                stack.push((second, child_depth));
                stack.push((first, child_depth));
                stack.push((sigma_type, child_depth));
            }
            Term::First { pair } | Term::Second { pair } => {
                stack
                    .try_reserve(1)
                    .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
                stack.push((pair, child_depth));
            }
            Term::Sort { .. }
            | Term::Var { .. }
            | Term::Global { .. }
            | Term::UnitType
            | Term::Unit => {}
        }
    }
    Ok(count)
}

fn preflight_context_v1(
    context: &DependentContext,
    budget: &mut LocalStructuralBudgetV1,
) -> Result<u32, ParticularOpenTypedSubstitutionFailureV1> {
    let mut count = 0_u32;
    for entry in &context.0 {
        count = count
            .checked_add(preflight_term_v1(entry, budget)?)
            .ok_or(ParticularOpenTypedSubstitutionFailureV1::OperationBudgetExhausted)?;
    }
    Ok(count)
}

fn preflight_signature_v1(
    signature: &VerifiedSignature,
    kernel: &Kernel,
) -> Result<(), ParticularOpenTypedSubstitutionFailureV1> {
    let mut budget = LocalStructuralBudgetV1::new(kernel);
    if signature.declarations().len() > budget.operations_left as usize {
        return Err(ParticularOpenTypedSubstitutionFailureV1::OperationBudgetExhausted);
    }
    for declaration in signature.declarations() {
        budget.charge(0)?;
        let _ = preflight_term_v1(&declaration.ty, &mut budget)?;
        if let Some(body) = declaration.body.as_ref() {
            let _ = preflight_term_v1(body, &mut budget)?;
        }
    }
    Ok(())
}

#[derive(Debug)]
struct ProposalMaterialV1 {
    domain_nodes: u32,
    codomain_nodes: u32,
    image_nodes: Vec<u32>,
}

fn preflight_proposal_v1(
    proposal: &ProposedParticularOpenTypedSubstitutionV1,
    kernel: &Kernel,
) -> Result<ProposalMaterialV1, ParticularOpenTypedSubstitutionFailureV1> {
    if proposal.images.len() != proposal.codomain_context.0.len() {
        return Err(ParticularOpenTypedSubstitutionFailureV1::ImageArityMismatch);
    }
    let _ = u32::try_from(proposal.domain_context.0.len())
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::ContextLengthOverflow)?;
    let _ = u32::try_from(proposal.codomain_context.0.len())
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::ContextLengthOverflow)?;
    let mut budget = LocalStructuralBudgetV1::new(kernel);
    let domain_nodes = preflight_context_v1(&proposal.domain_context, &mut budget)?;
    let codomain_nodes = preflight_context_v1(&proposal.codomain_context, &mut budget)?;
    let mut image_nodes = Vec::new();
    image_nodes
        .try_reserve_exact(proposal.images.len())
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    for image in &proposal.images {
        image_nodes.push(preflight_term_v1(image, &mut budget)?);
    }
    Ok(ProposalMaterialV1 {
        domain_nodes,
        codomain_nodes,
        image_nodes,
    })
}

fn shift_term_up_bounded_v1(
    term: &Term,
    amount: u32,
    cutoff: u32,
    structural_depth: u16,
    budget: &mut LocalStructuralBudgetV1,
) -> Result<Term, ParticularOpenTypedSubstitutionFailureV1> {
    budget.charge(structural_depth)?;
    let child_depth = next_structural_depth_v1(structural_depth)?;
    match term {
        Term::Var { index } if *index >= cutoff => Ok(Term::Var {
            index: index
                .checked_add(amount)
                .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?,
        }),
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(term.clone()),
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(shift_term_up_bounded_v1(
                parameter,
                amount,
                cutoff,
                child_depth,
                budget,
            )?),
            body: Box::new(shift_term_up_bounded_v1(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?,
                child_depth,
                budget,
            )?),
        }),
        Term::Sigma { parameter, body } => Ok(Term::Sigma {
            parameter: Box::new(shift_term_up_bounded_v1(
                parameter,
                amount,
                cutoff,
                child_depth,
                budget,
            )?),
            body: Box::new(shift_term_up_bounded_v1(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?,
                child_depth,
                budget,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(shift_term_up_bounded_v1(
                parameter_type,
                amount,
                cutoff,
                child_depth,
                budget,
            )?),
            body: Box::new(shift_term_up_bounded_v1(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?,
                child_depth,
                budget,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(shift_term_up_bounded_v1(
                function,
                amount,
                cutoff,
                child_depth,
                budget,
            )?),
            argument: Box::new(shift_term_up_bounded_v1(
                argument,
                amount,
                cutoff,
                child_depth,
                budget,
            )?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Ok(Term::Pair {
            sigma_type: Box::new(shift_term_up_bounded_v1(
                sigma_type,
                amount,
                cutoff,
                child_depth,
                budget,
            )?),
            first: Box::new(shift_term_up_bounded_v1(
                first,
                amount,
                cutoff,
                child_depth,
                budget,
            )?),
            second: Box::new(shift_term_up_bounded_v1(
                second,
                amount,
                cutoff,
                child_depth,
                budget,
            )?),
        }),
        Term::First { pair } => Ok(Term::First {
            pair: Box::new(shift_term_up_bounded_v1(
                pair,
                amount,
                cutoff,
                child_depth,
                budget,
            )?),
        }),
        Term::Second { pair } => Ok(Term::Second {
            pair: Box::new(shift_term_up_bounded_v1(
                pair,
                amount,
                cutoff,
                child_depth,
                budget,
            )?),
        }),
    }
}

fn reindex_term_bounded_v1(
    term: &Term,
    codomain_context_len: usize,
    images: &[Term],
    binder_depth: u32,
    structural_depth: u16,
    budget: &mut LocalStructuralBudgetV1,
) -> Result<Term, ParticularOpenTypedSubstitutionFailureV1> {
    if images.len() != codomain_context_len {
        return Err(ParticularOpenTypedSubstitutionFailureV1::ImageArityMismatch);
    }
    budget.charge(structural_depth)?;
    let child_depth = next_structural_depth_v1(structural_depth)?;
    match term {
        Term::Sort { .. } | Term::Global { .. } | Term::UnitType | Term::Unit => Ok(term.clone()),
        Term::Var { index } if *index < binder_depth => Ok(term.clone()),
        Term::Var { index } => {
            let free_index = usize::try_from(
                index
                    .checked_sub(binder_depth)
                    .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableOutOfScope)?,
            )
            .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?;
            let ordinal = codomain_context_len
                .checked_sub(
                    free_index
                        .checked_add(1)
                        .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?,
                )
                .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableOutOfScope)?;
            shift_term_up_bounded_v1(
                images
                    .get(ordinal)
                    .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableOutOfScope)?,
                binder_depth,
                0,
                structural_depth,
                budget,
            )
        }
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(reindex_term_bounded_v1(
                parameter,
                codomain_context_len,
                images,
                binder_depth,
                child_depth,
                budget,
            )?),
            body: Box::new(reindex_term_bounded_v1(
                body,
                codomain_context_len,
                images,
                binder_depth
                    .checked_add(1)
                    .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?,
                child_depth,
                budget,
            )?),
        }),
        Term::Sigma { parameter, body } => Ok(Term::Sigma {
            parameter: Box::new(reindex_term_bounded_v1(
                parameter,
                codomain_context_len,
                images,
                binder_depth,
                child_depth,
                budget,
            )?),
            body: Box::new(reindex_term_bounded_v1(
                body,
                codomain_context_len,
                images,
                binder_depth
                    .checked_add(1)
                    .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?,
                child_depth,
                budget,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(reindex_term_bounded_v1(
                parameter_type,
                codomain_context_len,
                images,
                binder_depth,
                child_depth,
                budget,
            )?),
            body: Box::new(reindex_term_bounded_v1(
                body,
                codomain_context_len,
                images,
                binder_depth
                    .checked_add(1)
                    .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?,
                child_depth,
                budget,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(reindex_term_bounded_v1(
                function,
                codomain_context_len,
                images,
                binder_depth,
                child_depth,
                budget,
            )?),
            argument: Box::new(reindex_term_bounded_v1(
                argument,
                codomain_context_len,
                images,
                binder_depth,
                child_depth,
                budget,
            )?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Ok(Term::Pair {
            sigma_type: Box::new(reindex_term_bounded_v1(
                sigma_type,
                codomain_context_len,
                images,
                binder_depth,
                child_depth,
                budget,
            )?),
            first: Box::new(reindex_term_bounded_v1(
                first,
                codomain_context_len,
                images,
                binder_depth,
                child_depth,
                budget,
            )?),
            second: Box::new(reindex_term_bounded_v1(
                second,
                codomain_context_len,
                images,
                binder_depth,
                child_depth,
                budget,
            )?),
        }),
        Term::First { pair } => Ok(Term::First {
            pair: Box::new(reindex_term_bounded_v1(
                pair,
                codomain_context_len,
                images,
                binder_depth,
                child_depth,
                budget,
            )?),
        }),
        Term::Second { pair } => Ok(Term::Second {
            pair: Box::new(reindex_term_bounded_v1(
                pair,
                codomain_context_len,
                images,
                binder_depth,
                child_depth,
                budget,
            )?),
        }),
    }
}

fn derive_expected_image_types_v1(
    proposal: &ProposedParticularOpenTypedSubstitutionV1,
    kernel: &Kernel,
) -> Result<Vec<Term>, ParticularOpenTypedSubstitutionFailureV1> {
    if proposal.images.len() != proposal.codomain_context.0.len() {
        return Err(ParticularOpenTypedSubstitutionFailureV1::ImageArityMismatch);
    }
    let mut budget = LocalStructuralBudgetV1::new(kernel);
    let mut expected_types = Vec::new();
    expected_types
        .try_reserve_exact(proposal.images.len())
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    for (ordinal, codomain_entry) in proposal.codomain_context.0.iter().enumerate() {
        expected_types.push(reindex_term_bounded_v1(
            codomain_entry,
            ordinal,
            &proposal.images[..ordinal],
            0,
            0,
            &mut budget,
        )?);
    }
    Ok(expected_types)
}

fn checked_material_add_v1(
    total: &mut u64,
    value: u64,
    limit: u64,
) -> Result<(), ParticularOpenTypedSubstitutionFailureV1> {
    *total = total
        .checked_add(value)
        .ok_or(ParticularOpenTypedSubstitutionFailureV1::AggregateMaterialLimitExceeded)?;
    if *total > limit {
        return Err(ParticularOpenTypedSubstitutionFailureV1::AggregateMaterialLimitExceeded);
    }
    Ok(())
}

fn preflight_replay_material_v1(
    proposal: &ProposedParticularOpenTypedSubstitutionV1,
    proposal_material: &ProposalMaterialV1,
    expected_types: &[Term],
    kernel: &Kernel,
) -> Result<(), ParticularOpenTypedSubstitutionFailureV1> {
    if expected_types.len() != proposal.images.len()
        || proposal_material.image_nodes.len() != proposal.images.len()
    {
        return Err(ParticularOpenTypedSubstitutionFailureV1::ImageArityMismatch);
    }
    let limit = u64::from(
        kernel
            .limits()
            .max_operations
            .min(PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_OPERATIONS_V1),
    );
    let judgment_count = proposal
        .images
        .len()
        .checked_add(2)
        .ok_or(ParticularOpenTypedSubstitutionFailureV1::AggregateMaterialLimitExceeded)?;
    if judgment_count > limit as usize {
        return Err(ParticularOpenTypedSubstitutionFailureV1::AggregateMaterialLimitExceeded);
    }
    let mut expected_budget = LocalStructuralBudgetV1::new(kernel);
    let mut expected_nodes = Vec::new();
    expected_nodes
        .try_reserve_exact(expected_types.len())
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    for expected in expected_types {
        expected_nodes.push(preflight_term_v1(expected, &mut expected_budget)?);
    }
    let mut aggregate = 0_u64;
    checked_material_add_v1(&mut aggregate, judgment_count as u64, limit)?;
    checked_material_add_v1(
        &mut aggregate,
        u64::from(proposal_material.codomain_nodes) + 1,
        limit,
    )?;
    checked_material_add_v1(
        &mut aggregate,
        u64::from(proposal_material.domain_nodes) + 1,
        limit,
    )?;
    for ((image_nodes, expected_nodes), _) in proposal_material
        .image_nodes
        .iter()
        .zip(&expected_nodes)
        .zip(&proposal.images)
    {
        checked_material_add_v1(
            &mut aggregate,
            u64::from(proposal_material.domain_nodes)
                + u64::from(*image_nodes)
                + u64::from(*expected_nodes),
            limit,
        )?;
    }
    Ok(())
}

fn build_replay_judgments_v1(
    proposal: &ProposedParticularOpenTypedSubstitutionV1,
    expected_types: &[Term],
) -> Result<Vec<OpenJudgment>, ParticularOpenTypedSubstitutionFailureV1> {
    if expected_types.len() != proposal.images.len() {
        return Err(ParticularOpenTypedSubstitutionFailureV1::ImageArityMismatch);
    }
    let capacity = proposal
        .images
        .len()
        .checked_add(2)
        .ok_or(ParticularOpenTypedSubstitutionFailureV1::AggregateMaterialLimitExceeded)?;
    let mut judgments = Vec::new();
    judgments
        .try_reserve_exact(capacity)
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    judgments.push(OpenJudgment::TypeFormation {
        context: proposal.codomain_context.clone(),
        term: Term::UnitType,
    });
    judgments.push(OpenJudgment::TypeFormation {
        context: proposal.domain_context.clone(),
        term: Term::UnitType,
    });
    for (image, expected_type) in proposal.images.iter().zip(expected_types) {
        judgments.push(OpenJudgment::HasType {
            context: proposal.domain_context.clone(),
            term: image.clone(),
            ty: expected_type.clone(),
        });
    }
    Ok(judgments)
}

fn replay_judgments_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    judgments: &[OpenJudgment],
) -> Result<Vec<OpenJudgment>, ParticularOpenTypedSubstitutionFailureV1> {
    let mut references = Vec::new();
    references
        .try_reserve_exact(judgments.len())
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    references.extend(judgments.iter());
    kernel
        .verify_open_judgments(signature, &references)
        .map_err(map_kernel_failure_v1)
}

fn map_kernel_failure_v1(error: KernelError) -> ParticularOpenTypedSubstitutionFailureV1 {
    match error {
        KernelError::ResourceExhausted(kind) => {
            ParticularOpenTypedSubstitutionFailureV1::KernelResourceExhausted(kind)
        }
        other => ParticularOpenTypedSubstitutionFailureV1::KernelRejected(other),
    }
}

fn ensure_protocol_runtime_v1(
    protocol: &VerifiedParticularOpenTypedSubstitutionProtocolV1,
    kernel: &Kernel,
) -> Result<(), ParticularOpenTypedSubstitutionFailureV1> {
    let limits = kernel.limits();
    if limits.max_operations > PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_OPERATIONS_V1
        || limits.max_depth > PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_DEPTH_V1
        || limits.normalization_fuel > PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_NORMALIZATION_FUEL_V1
    {
        return Err(ParticularOpenTypedSubstitutionFailureV1::KernelLimitsExceedReviewedCeilings);
    }
    if protocol.kernel_protocol_digest() != &kernel.kernel_protocol_digest() {
        return Err(ParticularOpenTypedSubstitutionFailureV1::ProtocolKernelMismatch);
    }
    if protocol.normalizer_protocol_digest() != &kernel.normalizer_protocol_digest() {
        return Err(ParticularOpenTypedSubstitutionFailureV1::ProtocolNormalizerMismatch);
    }
    if protocol.kernel_configuration_digest()
        != &generative_capability_kernel_configuration_digest_v1(kernel)
    {
        return Err(ParticularOpenTypedSubstitutionFailureV1::ProtocolKernelConfigurationMismatch);
    }
    Ok(())
}

fn ensure_substitution_binding_v1(
    protocol: &VerifiedParticularOpenTypedSubstitutionProtocolV1,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    substitution: &VerifiedParticularOpenTypedSubstitutionV1,
) -> Result<(), ParticularOpenTypedSubstitutionFailureV1> {
    ensure_protocol_runtime_v1(protocol, kernel)?;
    preflight_signature_v1(signature, kernel)?;
    if substitution.binding.protocol_manifest_digest != *protocol.manifest_digest()
        || substitution.binding.structural_grammar_manifest_digest
            != *protocol.structural_grammar_manifest_digest()
        || substitution.binding.kernel_protocol_digest != kernel.kernel_protocol_digest()
        || substitution.binding.normalizer_protocol_digest != kernel.normalizer_protocol_digest()
        || substitution.binding.kernel_configuration_digest
            != generative_capability_kernel_configuration_digest_v1(kernel)
    {
        return Err(ParticularOpenTypedSubstitutionFailureV1::ProtocolKernelConfigurationMismatch);
    }
    if substitution.binding.signature_digest != *signature.digest()
        || substitution.normalized_signature_wire != signature.normalized_wire()
    {
        return Err(ParticularOpenTypedSubstitutionFailureV1::SignatureMismatch);
    }
    Ok(())
}

fn type_formation_parts_v1(
    judgment: &OpenJudgment,
) -> Result<(&DependentContext, &Term), ParticularOpenTypedSubstitutionFailureV1> {
    match judgment {
        OpenJudgment::TypeFormation { context, term } => Ok((context, term)),
        _ => Err(ParticularOpenTypedSubstitutionFailureV1::UnexpectedKernelJudgment),
    }
}

fn has_type_parts_v1(
    judgment: &OpenJudgment,
) -> Result<(&DependentContext, &Term, &Term), ParticularOpenTypedSubstitutionFailureV1> {
    match judgment {
        OpenJudgment::HasType { context, term, ty } => Ok((context, term, ty)),
        _ => Err(ParticularOpenTypedSubstitutionFailureV1::UnexpectedKernelJudgment),
    }
}

/// Verify one exact `theta : Delta => Gamma` with two separate aggregate
/// public-kernel replays.
///
/// Pass one checks the caller proposal and internally derived dependent image
/// types. Pass two rebuilds the proposal from pass-one normalized contexts and
/// images, derives its dependent image types again, and independently replays
/// that normalized proposal. Failure returns no partial evidence.
pub fn verify_particular_open_typed_substitution_v1(
    protocol: &VerifiedParticularOpenTypedSubstitutionProtocolV1,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    proposal: &ProposedParticularOpenTypedSubstitutionV1,
) -> Result<VerifiedParticularOpenTypedSubstitutionV1, ParticularOpenTypedSubstitutionFailureV1> {
    ensure_protocol_runtime_v1(protocol, kernel)?;
    preflight_signature_v1(signature, kernel)?;
    let normalized_signature_wire = signature.normalized_wire();

    let raw_material = preflight_proposal_v1(proposal, kernel)?;
    let raw_expected_types = derive_expected_image_types_v1(proposal, kernel)?;
    preflight_replay_material_v1(proposal, &raw_material, &raw_expected_types, kernel)?;
    let raw_judgments = build_replay_judgments_v1(proposal, &raw_expected_types)?;
    let raw_normalized = replay_judgments_v1(kernel, signature, &raw_judgments)?;
    if raw_normalized.len() != raw_judgments.len() {
        return Err(ParticularOpenTypedSubstitutionFailureV1::UnexpectedKernelJudgment);
    }

    let (normalized_codomain_context, normalized_codomain_sentinel) =
        type_formation_parts_v1(&raw_normalized[0])?;
    let (normalized_domain_context, normalized_domain_sentinel) =
        type_formation_parts_v1(&raw_normalized[1])?;
    if normalized_codomain_sentinel != &Term::UnitType
        || normalized_domain_sentinel != &Term::UnitType
    {
        return Err(ParticularOpenTypedSubstitutionFailureV1::UnexpectedKernelJudgment);
    }
    let mut normalized_images = Vec::new();
    normalized_images
        .try_reserve_exact(proposal.images.len())
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    for normalized in &raw_normalized[2..] {
        let (context, term, _) = has_type_parts_v1(normalized)?;
        if context != normalized_domain_context {
            return Err(ParticularOpenTypedSubstitutionFailureV1::CanonicalReplayMismatch);
        }
        normalized_images.push(term.clone());
    }
    let canonical_proposal = ProposedParticularOpenTypedSubstitutionV1 {
        domain_context: normalized_domain_context.clone(),
        codomain_context: normalized_codomain_context.clone(),
        images: normalized_images,
    };

    let canonical_material = preflight_proposal_v1(&canonical_proposal, kernel)?;
    let canonical_expected_types = derive_expected_image_types_v1(&canonical_proposal, kernel)?;
    preflight_replay_material_v1(
        &canonical_proposal,
        &canonical_material,
        &canonical_expected_types,
        kernel,
    )?;
    let canonical_judgments =
        build_replay_judgments_v1(&canonical_proposal, &canonical_expected_types)?;
    let canonical_normalized = replay_judgments_v1(kernel, signature, &canonical_judgments)?;
    if canonical_normalized.len() != canonical_judgments.len() {
        return Err(ParticularOpenTypedSubstitutionFailureV1::UnexpectedKernelJudgment);
    }
    if canonical_normalized != raw_normalized {
        return Err(ParticularOpenTypedSubstitutionFailureV1::CanonicalReplayMismatch);
    }
    let (replayed_codomain_context, replayed_codomain_sentinel) =
        type_formation_parts_v1(&canonical_normalized[0])?;
    let (replayed_domain_context, replayed_domain_sentinel) =
        type_formation_parts_v1(&canonical_normalized[1])?;
    if replayed_codomain_context != &canonical_proposal.codomain_context
        || replayed_domain_context != &canonical_proposal.domain_context
        || replayed_codomain_sentinel != &Term::UnitType
        || replayed_domain_sentinel != &Term::UnitType
    {
        return Err(ParticularOpenTypedSubstitutionFailureV1::CanonicalReplayMismatch);
    }
    for (ordinal, normalized) in canonical_normalized[2..].iter().enumerate() {
        let (context, term, _) = has_type_parts_v1(normalized)?;
        if context != &canonical_proposal.domain_context
            || term
                != canonical_proposal
                    .images
                    .get(ordinal)
                    .ok_or(ParticularOpenTypedSubstitutionFailureV1::ImageArityMismatch)?
        {
            return Err(ParticularOpenTypedSubstitutionFailureV1::CanonicalReplayMismatch);
        }
    }

    let binding = ParticularOpenTypedSubstitutionAuthorityBindingV1 {
        schema_version: protocol.schema_version(),
        resource_policy_version: protocol.resource_policy_version(),
        protocol_manifest_digest: protocol.manifest_digest().clone(),
        structural_grammar_manifest_digest: protocol.structural_grammar_manifest_digest().clone(),
        signature_digest: signature.digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        kernel_configuration_digest: generative_capability_kernel_configuration_digest_v1(kernel),
    };
    let raw_proposal_digest = Digest::of_canonical(
        PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROPOSAL_DIGEST_DOMAIN_V1,
        proposal,
    );
    let mut images = Vec::new();
    images
        .try_reserve_exact(proposal.images.len())
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    for ordinal in 0..proposal.images.len() {
        let mut image = VerifiedParticularOpenTypedSubstitutionImageV1 {
            ordinal: u32::try_from(ordinal)
                .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::ContextLengthOverflow)?,
            raw_judgment: raw_judgments[ordinal + 2].clone(),
            raw_normalized_judgment: raw_normalized[ordinal + 2].clone(),
            canonical_input_judgment: canonical_judgments[ordinal + 2].clone(),
            canonical_normalized_judgment: canonical_normalized[ordinal + 2].clone(),
            evidence_digest: Digest::of_bytes(b"pending particular image evidence"),
        };
        image.evidence_digest = Digest::of_canonical(
            PARTICULAR_OPEN_TYPED_SUBSTITUTION_IMAGE_EVIDENCE_DIGEST_DOMAIN_V1,
            &image,
        );
        images.push(image);
    }

    let mut canonical_encoder = CanonicalEncoder::new();
    binding.encode_canonical(&mut canonical_encoder);
    normalized_signature_wire.encode_canonical(&mut canonical_encoder);
    canonical_proposal.encode_canonical(&mut canonical_encoder);
    for normalized in &canonical_normalized {
        normalized.encode_canonical(&mut canonical_encoder);
    }
    let canonical_digest = Digest::of_domain_bytes(
        PARTICULAR_OPEN_TYPED_SUBSTITUTION_CANONICAL_DIGEST_DOMAIN_V1,
        canonical_encoder.as_bytes(),
    );
    let mut verified = VerifiedParticularOpenTypedSubstitutionV1 {
        binding,
        normalized_signature_wire,
        raw_proposal: proposal.clone(),
        raw_proposal_digest,
        canonical_proposal,
        raw_codomain_context_judgment: raw_judgments[0].clone(),
        raw_domain_context_judgment: raw_judgments[1].clone(),
        normalized_codomain_context_judgment: raw_normalized[0].clone(),
        normalized_domain_context_judgment: raw_normalized[1].clone(),
        canonical_codomain_context_judgment: canonical_normalized[0].clone(),
        canonical_domain_context_judgment: canonical_normalized[1].clone(),
        images,
        canonical_digest,
        evidence_digest: Digest::of_bytes(b"pending particular substitution evidence"),
    };
    verified.evidence_digest = Digest::of_canonical(
        PARTICULAR_OPEN_TYPED_SUBSTITUTION_EVIDENCE_DIGEST_DOMAIN_V1,
        &verified,
    );
    Ok(verified)
}

fn reindex_open_judgment_bounded_v1(
    judgment: &OpenJudgment,
    substitution: &VerifiedParticularOpenTypedSubstitutionV1,
    kernel: &Kernel,
) -> Result<OpenJudgment, ParticularOpenTypedSubstitutionFailureV1> {
    let codomain_len = substitution.codomain_context().0.len();
    let images = substitution.canonical_images();
    let mut budget = LocalStructuralBudgetV1::new(kernel);
    match judgment {
        OpenJudgment::TypeFormation { term, .. } => Ok(OpenJudgment::TypeFormation {
            context: substitution.domain_context().clone(),
            term: reindex_term_bounded_v1(term, codomain_len, images, 0, 0, &mut budget)?,
        }),
        OpenJudgment::HasType { term, ty, .. } => Ok(OpenJudgment::HasType {
            context: substitution.domain_context().clone(),
            term: reindex_term_bounded_v1(term, codomain_len, images, 0, 0, &mut budget)?,
            ty: reindex_term_bounded_v1(ty, codomain_len, images, 0, 0, &mut budget)?,
        }),
        OpenJudgment::DefinitionallyEqual {
            left, right, ty, ..
        } => Ok(OpenJudgment::DefinitionallyEqual {
            context: substitution.domain_context().clone(),
            left: reindex_term_bounded_v1(left, codomain_len, images, 0, 0, &mut budget)?,
            right: reindex_term_bounded_v1(right, codomain_len, images, 0, 0, &mut budget)?,
            ty: reindex_term_bounded_v1(ty, codomain_len, images, 0, 0, &mut budget)?,
        }),
    }
}

fn preflight_open_judgments_v1(
    judgments: &[&OpenJudgment],
    kernel: &Kernel,
) -> Result<(), ParticularOpenTypedSubstitutionFailureV1> {
    if judgments.len()
        > kernel
            .limits()
            .max_operations
            .min(PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_OPERATIONS_V1) as usize
    {
        return Err(ParticularOpenTypedSubstitutionFailureV1::AggregateMaterialLimitExceeded);
    }
    let mut budget = LocalStructuralBudgetV1::new(kernel);
    for judgment in judgments {
        let _ = preflight_context_v1(judgment.context(), &mut budget)?;
        match judgment {
            OpenJudgment::TypeFormation { term, .. } => {
                let _ = preflight_term_v1(term, &mut budget)?;
            }
            OpenJudgment::HasType { term, ty, .. } => {
                let _ = preflight_term_v1(term, &mut budget)?;
                let _ = preflight_term_v1(ty, &mut budget)?;
            }
            OpenJudgment::DefinitionallyEqual {
                left, right, ty, ..
            } => {
                let _ = preflight_term_v1(left, &mut budget)?;
                let _ = preflight_term_v1(right, &mut budget)?;
                let _ = preflight_term_v1(ty, &mut budget)?;
            }
        }
    }
    Ok(())
}

/// Internally derive and recheck one identity substitution. This is not an
/// identity law.
pub fn verify_particular_identity_open_typed_substitution_v1(
    protocol: &VerifiedParticularOpenTypedSubstitutionProtocolV1,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &DependentContext,
) -> Result<
    VerifiedParticularIdentityOpenTypedSubstitutionV1,
    ParticularOpenTypedSubstitutionFailureV1,
> {
    ensure_protocol_runtime_v1(protocol, kernel)?;
    let mut context_budget = LocalStructuralBudgetV1::new(kernel);
    let _ = preflight_context_v1(context, &mut context_budget)?;
    let mut images = Vec::new();
    images
        .try_reserve_exact(context.0.len())
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    for ordinal in 0..context.0.len() {
        let index = context
            .0
            .len()
            .checked_sub(
                ordinal
                    .checked_add(1)
                    .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?,
            )
            .ok_or(ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?;
        images.push(Term::Var {
            index: u32::try_from(index)
                .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)?,
        });
    }
    let proposal = ProposedParticularOpenTypedSubstitutionV1 {
        domain_context: context.clone(),
        codomain_context: context.clone(),
        images,
    };
    let result =
        verify_particular_open_typed_substitution_v1(protocol, kernel, signature, &proposal)?;
    let mut verified = VerifiedParticularIdentityOpenTypedSubstitutionV1 {
        context_input: context.clone(),
        result,
        evidence_digest: Digest::of_bytes(b"pending particular identity evidence"),
    };
    verified.evidence_digest = Digest::of_canonical(
        PARTICULAR_IDENTITY_SUBSTITUTION_EVIDENCE_DIGEST_DOMAIN_V1,
        &verified,
    );
    Ok(verified)
}

/// Internally derive and recheck the one composite
/// `theta o rho : Xi => Gamma` from `theta : Delta => Gamma` and
/// `rho : Xi => Delta`. This is not a composition or associativity law.
pub fn verify_particular_composite_open_typed_substitution_v1(
    protocol: &VerifiedParticularOpenTypedSubstitutionProtocolV1,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    outer_theta_delta_to_gamma: &VerifiedParticularOpenTypedSubstitutionV1,
    inner_rho_xi_to_delta: &VerifiedParticularOpenTypedSubstitutionV1,
) -> Result<
    VerifiedParticularCompositeOpenTypedSubstitutionV1,
    ParticularOpenTypedSubstitutionFailureV1,
> {
    ensure_substitution_binding_v1(protocol, kernel, signature, outer_theta_delta_to_gamma)?;
    ensure_substitution_binding_v1(protocol, kernel, signature, inner_rho_xi_to_delta)?;
    if outer_theta_delta_to_gamma.domain_context() != inner_rho_xi_to_delta.codomain_context() {
        return Err(ParticularOpenTypedSubstitutionFailureV1::IntermediateContextMismatch);
    }
    let mut budget = LocalStructuralBudgetV1::new(kernel);
    let mut images = Vec::new();
    images
        .try_reserve_exact(outer_theta_delta_to_gamma.canonical_images().len())
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    for image in outer_theta_delta_to_gamma.canonical_images() {
        images.push(reindex_term_bounded_v1(
            image,
            inner_rho_xi_to_delta.codomain_context().0.len(),
            inner_rho_xi_to_delta.canonical_images(),
            0,
            0,
            &mut budget,
        )?);
    }
    let constructed_proposal = ProposedParticularOpenTypedSubstitutionV1 {
        domain_context: inner_rho_xi_to_delta.domain_context().clone(),
        codomain_context: outer_theta_delta_to_gamma.codomain_context().clone(),
        images,
    };
    let result = verify_particular_open_typed_substitution_v1(
        protocol,
        kernel,
        signature,
        &constructed_proposal,
    )?;
    let mut verified = VerifiedParticularCompositeOpenTypedSubstitutionV1 {
        outer: outer_theta_delta_to_gamma.clone(),
        inner: inner_rho_xi_to_delta.clone(),
        constructed_proposal,
        result,
        evidence_digest: Digest::of_bytes(b"pending particular composite evidence"),
    };
    verified.evidence_digest = Digest::of_canonical(
        PARTICULAR_COMPOSITE_SUBSTITUTION_EVIDENCE_DIGEST_DOMAIN_V1,
        &verified,
    );
    Ok(verified)
}

/// Internally derive and replay one reindexed open judgment. Both the exact
/// original and the exact syntactically derived endpoint are kernel checked in
/// one aggregate call; this is not generic typing/equality preservation.
pub fn verify_particular_reindexed_open_judgment_v1(
    protocol: &VerifiedParticularOpenTypedSubstitutionProtocolV1,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    substitution: &VerifiedParticularOpenTypedSubstitutionV1,
    original_judgment: &OpenJudgment,
) -> Result<VerifiedParticularReindexedOpenJudgmentV1, ParticularOpenTypedSubstitutionFailureV1> {
    ensure_substitution_binding_v1(protocol, kernel, signature, substitution)?;
    preflight_open_judgments_v1(&[original_judgment], kernel)?;
    if original_judgment.context() != substitution.codomain_context() {
        return Err(ParticularOpenTypedSubstitutionFailureV1::JudgmentContextMismatch);
    }
    let derived_reindexed_judgment =
        reindex_open_judgment_bounded_v1(original_judgment, substitution, kernel)?;
    let references = [original_judgment, &derived_reindexed_judgment];
    preflight_open_judgments_v1(&references, kernel)?;
    let normalized = kernel
        .verify_open_judgments(signature, &references)
        .map_err(map_kernel_failure_v1)?;
    if normalized.len() != 2
        || normalized[0].context() != substitution.codomain_context()
        || normalized[1].context() != substitution.domain_context()
    {
        return Err(ParticularOpenTypedSubstitutionFailureV1::CanonicalReplayMismatch);
    }
    let mut verified = VerifiedParticularReindexedOpenJudgmentV1 {
        substitution: substitution.clone(),
        original_judgment: original_judgment.clone(),
        derived_reindexed_judgment,
        normalized_original_judgment: normalized[0].clone(),
        normalized_reindexed_judgment: normalized[1].clone(),
        evidence_digest: Digest::of_bytes(b"pending particular reindexed judgment evidence"),
    };
    verified.evidence_digest = Digest::of_canonical(
        PARTICULAR_REINDEXED_OPEN_JUDGMENT_EVIDENCE_DIGEST_DOMAIN_V1,
        &verified,
    );
    Ok(verified)
}

/// Internally derive and recheck one dependent binder lift
/// `Delta.A[theta] => Gamma.A`. This is not a generic lift theorem.
pub fn verify_particular_binder_lift_open_typed_substitution_v1(
    protocol: &VerifiedParticularOpenTypedSubstitutionProtocolV1,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    substitution: &VerifiedParticularOpenTypedSubstitutionV1,
    codomain_parameter_type: &Term,
) -> Result<
    VerifiedParticularBinderLiftOpenTypedSubstitutionV1,
    ParticularOpenTypedSubstitutionFailureV1,
> {
    ensure_substitution_binding_v1(protocol, kernel, signature, substitution)?;
    let mut parameter_budget = LocalStructuralBudgetV1::new(kernel);
    let _ = preflight_term_v1(codomain_parameter_type, &mut parameter_budget)?;
    let parameter_judgment = OpenJudgment::TypeFormation {
        context: substitution.codomain_context().clone(),
        term: codomain_parameter_type.clone(),
    };
    let parameter_reindexing = verify_particular_reindexed_open_judgment_v1(
        protocol,
        kernel,
        signature,
        substitution,
        &parameter_judgment,
    )?;
    let OpenJudgment::TypeFormation {
        term: reindexed_domain_parameter_type,
        ..
    } = parameter_reindexing.derived_reindexed_judgment()
    else {
        return Err(ParticularOpenTypedSubstitutionFailureV1::UnexpectedKernelJudgment);
    };

    let mut codomain_entries = substitution.codomain_context().0.clone();
    codomain_entries
        .try_reserve(1)
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    codomain_entries.push(codomain_parameter_type.clone());
    let mut domain_entries = substitution.domain_context().0.clone();
    domain_entries
        .try_reserve(1)
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    domain_entries.push(reindexed_domain_parameter_type.clone());

    let mut budget = LocalStructuralBudgetV1::new(kernel);
    let mut images = Vec::new();
    images
        .try_reserve_exact(
            substitution
                .canonical_images()
                .len()
                .checked_add(1)
                .ok_or(ParticularOpenTypedSubstitutionFailureV1::ContextLengthOverflow)?,
        )
        .map_err(|_| ParticularOpenTypedSubstitutionFailureV1::AllocationFailure)?;
    for image in substitution.canonical_images() {
        images.push(shift_term_up_bounded_v1(image, 1, 0, 0, &mut budget)?);
    }
    images.push(Term::Var { index: 0 });
    let constructed_proposal = ProposedParticularOpenTypedSubstitutionV1 {
        domain_context: DependentContext(domain_entries),
        codomain_context: DependentContext(codomain_entries),
        images,
    };
    let result = verify_particular_open_typed_substitution_v1(
        protocol,
        kernel,
        signature,
        &constructed_proposal,
    )?;
    let mut verified = VerifiedParticularBinderLiftOpenTypedSubstitutionV1 {
        substitution: substitution.clone(),
        parameter_reindexing,
        constructed_proposal,
        result,
        evidence_digest: Digest::of_bytes(b"pending particular binder lift evidence"),
    };
    verified.evidence_digest = Digest::of_canonical(
        PARTICULAR_BINDER_LIFT_SUBSTITUTION_EVIDENCE_DIGEST_DOMAIN_V1,
        &verified,
    );
    Ok(verified)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        proposed_generative_capability_constructor_grammar_v1,
        proposed_generative_structural_occurrence_grammar_v1,
        proposed_generative_substitution_naturality_protocol_v1,
        proposed_pre_exposure_generative_capability_grammar_v1,
        verify_generative_capability_constructor_grammar_v1,
        verify_generative_structural_occurrence_grammar_v1,
        verify_generative_substitution_naturality_protocol_v1,
        verify_pre_exposure_generative_capability_grammar_v1,
    };
    use pen_kernel::{GlobalId, KernelLimits, UncheckedSignature};

    fn verified_inputs_with_limits(
        limits: KernelLimits,
    ) -> (
        Kernel,
        VerifiedSignature,
        VerifiedParticularOpenTypedSubstitutionProtocolV1,
    ) {
        let kernel = Kernel::new(limits).expect("kernel");
        let jg1 = verify_pre_exposure_generative_capability_grammar_v1(
            &proposed_pre_exposure_generative_capability_grammar_v1(),
        )
        .expect("JG1");
        let constructors = verify_generative_capability_constructor_grammar_v1(
            &jg1,
            &proposed_generative_capability_constructor_grammar_v1(&jg1),
        )
        .expect("JG2a");
        let entry = verify_generative_substitution_naturality_protocol_v1(
            &jg1,
            &constructors,
            &kernel,
            &proposed_generative_substitution_naturality_protocol_v1(&jg1, &constructors, &kernel),
        )
        .expect("JG2b2a");
        let grammar = verify_generative_structural_occurrence_grammar_v1(
            &entry,
            &jg1,
            &constructors,
            &kernel,
            &proposed_generative_structural_occurrence_grammar_v1(
                &entry,
                &jg1,
                &constructors,
                &kernel,
            ),
        )
        .expect("JG2b2b0");
        let protocol = verify_particular_open_typed_substitution_protocol_v1(
            &grammar,
            &kernel,
            &proposed_particular_open_typed_substitution_protocol_v1(&grammar, &kernel),
        )
        .expect("JG2b2b1 protocol");
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        (kernel, signature, protocol)
    }

    fn verified_inputs() -> (
        Kernel,
        VerifiedSignature,
        VerifiedParticularOpenTypedSubstitutionProtocolV1,
    ) {
        verified_inputs_with_limits(KernelLimits::default())
    }

    fn dependent_unit_substitution(
        protocol: &VerifiedParticularOpenTypedSubstitutionProtocolV1,
        kernel: &Kernel,
        signature: &VerifiedSignature,
    ) -> VerifiedParticularOpenTypedSubstitutionV1 {
        verify_particular_open_typed_substitution_v1(
            protocol,
            kernel,
            signature,
            &ProposedParticularOpenTypedSubstitutionV1 {
                domain_context: DependentContext::default(),
                codomain_context: DependentContext(vec![
                    Term::Sort { level: 0 },
                    Term::Var { index: 0 },
                ]),
                images: vec![Term::UnitType, Term::Unit],
            },
        )
        .expect("dependent unit substitution")
    }

    #[test]
    fn protocol_definition_transcript_and_digest_are_frozen() {
        let definition = proposed_particular_open_typed_substitution_protocol_definition_v1();
        let mut encoder = CanonicalEncoder::new();
        definition.encode_canonical(&mut encoder);
        assert_eq!(
            encoder.as_bytes(),
            CANONICAL_PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_BYTES_V1
        );
        let digest = Digest::of_domain_bytes(
            PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_DIGEST_DOMAIN_V1,
            encoder.as_bytes(),
        );
        assert_eq!(
            digest.as_str(),
            CANONICAL_PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_DIGEST_V1
        );
    }

    #[test]
    fn synthetic_full_protocol_manifest_codec_is_frozen() {
        let synthetic = |label: &[u8]| {
            Digest::of_domain_bytes("law-v2/jg2b2b1/synthetic-protocol-manifest-field/v1", label)
        };
        let manifest = ParticularOpenTypedSubstitutionProtocolManifestV1 {
            definition: proposed_particular_open_typed_substitution_protocol_definition_v1(),
            structural_grammar_manifest_digest: synthetic(b"structural-grammar"),
            substitution_naturality_protocol_manifest_digest: synthetic(b"entry-protocol"),
            jg1_manifest_digest: synthetic(b"jg1"),
            constructor_grammar_manifest_digest: synthetic(b"constructors"),
            scope_grammar_digest: synthetic(b"scope"),
            kernel_protocol_digest: synthetic(b"kernel"),
            normalizer_protocol_digest: synthetic(b"normalizer"),
            kernel_configuration_digest: synthetic(b"configuration"),
        };
        let mut encoder = CanonicalEncoder::new();
        manifest.encode_canonical(&mut encoder);
        assert_eq!(
            encoder.as_bytes().len(),
            SYNTHETIC_PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_MANIFEST_LENGTH_V1
        );
        let digest = Digest::of_domain_bytes(
            PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_MANIFEST_DIGEST_DOMAIN_V1,
            encoder.as_bytes(),
        );
        assert_eq!(
            digest.as_str(),
            SYNTHETIC_PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_MANIFEST_DIGEST_V1
        );
    }

    #[test]
    fn protocol_remints_deterministically_and_rejects_definition_mutations() {
        let (kernel, _, protocol) = verified_inputs();
        assert_eq!(
            protocol.rules(),
            PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_RULES_V1
        );
        assert_eq!(
            protocol.definition_digest().as_str(),
            CANONICAL_PARTICULAR_OPEN_TYPED_SUBSTITUTION_PROTOCOL_DEFINITION_DIGEST_V1
        );

        let jg1 = verify_pre_exposure_generative_capability_grammar_v1(
            &proposed_pre_exposure_generative_capability_grammar_v1(),
        )
        .expect("JG1");
        let constructors = verify_generative_capability_constructor_grammar_v1(
            &jg1,
            &proposed_generative_capability_constructor_grammar_v1(&jg1),
        )
        .expect("JG2a");
        let entry = verify_generative_substitution_naturality_protocol_v1(
            &jg1,
            &constructors,
            &kernel,
            &proposed_generative_substitution_naturality_protocol_v1(&jg1, &constructors, &kernel),
        )
        .expect("entry");
        let grammar = verify_generative_structural_occurrence_grammar_v1(
            &entry,
            &jg1,
            &constructors,
            &kernel,
            &proposed_generative_structural_occurrence_grammar_v1(
                &entry,
                &jg1,
                &constructors,
                &kernel,
            ),
        )
        .expect("grammar");
        let mut manifest =
            proposed_particular_open_typed_substitution_protocol_v1(&grammar, &kernel);
        manifest.definition.rules.swap(0, 1);
        assert_eq!(
            verify_particular_open_typed_substitution_protocol_v1(&grammar, &kernel, &manifest,),
            Err(ParticularOpenTypedSubstitutionProtocolFailureV1::RuleVocabularyMismatch)
        );
        let mut manifest =
            proposed_particular_open_typed_substitution_protocol_v1(&grammar, &kernel);
        manifest.definition.max_operations -= 1;
        assert_eq!(
            verify_particular_open_typed_substitution_protocol_v1(&grammar, &kernel, &manifest,),
            Err(ParticularOpenTypedSubstitutionProtocolFailureV1::ResourceCeilingMismatch)
        );
        let mut manifest =
            proposed_particular_open_typed_substitution_protocol_v1(&grammar, &kernel);
        manifest.definition.schema_version += 1;
        assert_eq!(
            verify_particular_open_typed_substitution_protocol_v1(&grammar, &kernel, &manifest),
            Err(ParticularOpenTypedSubstitutionProtocolFailureV1::SchemaVersionMismatch)
        );
        let mut manifest =
            proposed_particular_open_typed_substitution_protocol_v1(&grammar, &kernel);
        manifest.definition.resource_policy_version += 1;
        assert_eq!(
            verify_particular_open_typed_substitution_protocol_v1(&grammar, &kernel, &manifest),
            Err(ParticularOpenTypedSubstitutionProtocolFailureV1::ResourcePolicyVersionMismatch)
        );
        let mut manifest =
            proposed_particular_open_typed_substitution_protocol_v1(&grammar, &kernel);
        manifest.definition.max_depth -= 1;
        assert_eq!(
            verify_particular_open_typed_substitution_protocol_v1(&grammar, &kernel, &manifest),
            Err(ParticularOpenTypedSubstitutionProtocolFailureV1::ResourceCeilingMismatch)
        );
        let mut manifest =
            proposed_particular_open_typed_substitution_protocol_v1(&grammar, &kernel);
        manifest.definition.max_normalization_fuel -= 1;
        assert_eq!(
            verify_particular_open_typed_substitution_protocol_v1(&grammar, &kernel, &manifest),
            Err(ParticularOpenTypedSubstitutionProtocolFailureV1::ResourceCeilingMismatch)
        );

        let original = proposed_particular_open_typed_substitution_protocol_v1(&grammar, &kernel);
        let second =
            verify_particular_open_typed_substitution_protocol_v1(&grammar, &kernel, &original)
                .expect("deterministic remint");
        assert_eq!(protocol.manifest_digest(), second.manifest_digest());

        macro_rules! digest_mutation {
            ($field:ident, $label:literal, $failure:expr) => {{
                let mut mutation = original.clone();
                mutation.$field =
                    Digest::of_domain_bytes("law-v2/jg2b2b1/protocol-manifest-mutation/v1", $label);
                assert_eq!(
                    verify_particular_open_typed_substitution_protocol_v1(
                        &grammar, &kernel, &mutation,
                    ),
                    Err($failure)
                );
            }};
        }
        digest_mutation!(
            structural_grammar_manifest_digest,
            b"structural",
            ParticularOpenTypedSubstitutionProtocolFailureV1::StructuralGrammarManifestDigestMismatch
        );
        digest_mutation!(
            substitution_naturality_protocol_manifest_digest,
            b"entry",
            ParticularOpenTypedSubstitutionProtocolFailureV1::SubstitutionNaturalityProtocolManifestDigestMismatch
        );
        digest_mutation!(
            jg1_manifest_digest,
            b"jg1",
            ParticularOpenTypedSubstitutionProtocolFailureV1::Jg1ManifestDigestMismatch
        );
        digest_mutation!(
            constructor_grammar_manifest_digest,
            b"constructors",
            ParticularOpenTypedSubstitutionProtocolFailureV1::ConstructorGrammarManifestDigestMismatch
        );
        digest_mutation!(
            scope_grammar_digest,
            b"scope",
            ParticularOpenTypedSubstitutionProtocolFailureV1::ScopeGrammarDigestMismatch
        );
        digest_mutation!(
            kernel_protocol_digest,
            b"kernel",
            ParticularOpenTypedSubstitutionProtocolFailureV1::KernelProtocolMismatch
        );
        digest_mutation!(
            normalizer_protocol_digest,
            b"normalizer",
            ParticularOpenTypedSubstitutionProtocolFailureV1::NormalizerProtocolMismatch
        );
        digest_mutation!(
            kernel_configuration_digest,
            b"configuration",
            ParticularOpenTypedSubstitutionProtocolFailureV1::KernelConfigurationMismatch
        );
    }

    #[test]
    fn protocol_rejects_kernel_limits_above_reviewed_ceiling() {
        let kernel = Kernel::new(KernelLimits {
            max_operations: PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_OPERATIONS_V1 + 1,
            max_depth: PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_DEPTH_V1,
            normalization_fuel: PARTICULAR_OPEN_TYPED_SUBSTITUTION_MAX_NORMALIZATION_FUEL_V1,
        })
        .expect("kernel permits the deliberately overlarge operation profile");
        let jg1 = verify_pre_exposure_generative_capability_grammar_v1(
            &proposed_pre_exposure_generative_capability_grammar_v1(),
        )
        .expect("JG1");
        let constructors = verify_generative_capability_constructor_grammar_v1(
            &jg1,
            &proposed_generative_capability_constructor_grammar_v1(&jg1),
        )
        .expect("JG2a");
        let entry = verify_generative_substitution_naturality_protocol_v1(
            &jg1,
            &constructors,
            &kernel,
            &proposed_generative_substitution_naturality_protocol_v1(&jg1, &constructors, &kernel),
        )
        .expect("entry");
        let grammar = verify_generative_structural_occurrence_grammar_v1(
            &entry,
            &jg1,
            &constructors,
            &kernel,
            &proposed_generative_structural_occurrence_grammar_v1(
                &entry,
                &jg1,
                &constructors,
                &kernel,
            ),
        )
        .expect("grammar");
        assert_eq!(
            verify_particular_open_typed_substitution_protocol_v1(
                &grammar,
                &kernel,
                &proposed_particular_open_typed_substitution_protocol_v1(&grammar, &kernel),
            ),
            Err(ParticularOpenTypedSubstitutionProtocolFailureV1::ResourceCeilingMismatch)
        );
    }

    #[test]
    fn dependent_images_receive_two_complete_replays_and_full_evidence() {
        let (kernel, signature, protocol) = verified_inputs();
        let substitution = dependent_unit_substitution(&protocol, &kernel, &signature);
        assert_eq!(substitution.domain_context(), &DependentContext::default());
        assert_eq!(
            substitution.codomain_context(),
            &DependentContext(vec![Term::Sort { level: 0 }, Term::Var { index: 0 }])
        );
        assert_eq!(
            substitution.canonical_images(),
            &[Term::UnitType, Term::Unit]
        );
        assert_eq!(substitution.image_evidence().len(), 2);
        assert_eq!(
            substitution.normalized_signature_wire(),
            &signature.normalized_wire()
        );
        for image in substitution.image_evidence() {
            assert!(matches!(image.raw_judgment(), OpenJudgment::HasType { .. }));
            assert!(matches!(
                image.raw_normalized_judgment(),
                OpenJudgment::HasType { .. }
            ));
            assert!(matches!(
                image.canonical_input_judgment(),
                OpenJudgment::HasType { .. }
            ));
            assert!(matches!(
                image.canonical_normalized_judgment(),
                OpenJudgment::HasType { .. }
            ));
        }
        assert_ne!(
            substitution.raw_proposal_digest(),
            substitution.evidence_digest()
        );
    }

    #[test]
    fn raw_beta_proposal_and_normalized_replay_remain_separate() {
        let (kernel, signature, protocol) = verified_inputs();
        let beta_unit_type = Term::Apply {
            function: Box::new(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            }),
            argument: Box::new(Term::Unit),
        };
        let substitution = verify_particular_open_typed_substitution_v1(
            &protocol,
            &kernel,
            &signature,
            &ProposedParticularOpenTypedSubstitutionV1 {
                domain_context: DependentContext::default(),
                codomain_context: DependentContext(vec![
                    Term::Sort { level: 0 },
                    Term::Var { index: 0 },
                ]),
                images: vec![beta_unit_type.clone(), Term::Unit],
            },
        )
        .expect("beta-normalizing particular substitution");
        let normal = dependent_unit_substitution(&protocol, &kernel, &signature);
        assert_eq!(substitution.raw_proposal().images[0], beta_unit_type);
        assert_eq!(substitution.canonical_images()[0], Term::UnitType);
        assert_ne!(
            substitution.raw_proposal(),
            substitution.canonical_proposal()
        );
        assert_eq!(substitution.canonical_digest(), normal.canonical_digest());
        assert_ne!(substitution.evidence_digest(), normal.evidence_digest());
    }

    #[test]
    fn image_arity_and_ill_typed_images_fail_without_evidence() {
        let (kernel, signature, protocol) = verified_inputs();
        let codomain = DependentContext(vec![Term::UnitType]);
        assert_eq!(
            verify_particular_open_typed_substitution_v1(
                &protocol,
                &kernel,
                &signature,
                &ProposedParticularOpenTypedSubstitutionV1 {
                    domain_context: DependentContext::default(),
                    codomain_context: codomain.clone(),
                    images: vec![],
                },
            ),
            Err(ParticularOpenTypedSubstitutionFailureV1::ImageArityMismatch)
        );
        assert!(matches!(
            verify_particular_open_typed_substitution_v1(
                &protocol,
                &kernel,
                &signature,
                &ProposedParticularOpenTypedSubstitutionV1 {
                    domain_context: DependentContext::default(),
                    codomain_context: codomain,
                    images: vec![Term::UnitType],
                },
            ),
            Err(ParticularOpenTypedSubstitutionFailureV1::KernelRejected(_))
        ));
    }

    #[test]
    fn empty_codomain_still_replays_both_context_sentinels() {
        let (kernel, signature, protocol) = verified_inputs();
        let domain = DependentContext(vec![Term::UnitType]);
        let substitution = verify_particular_open_typed_substitution_v1(
            &protocol,
            &kernel,
            &signature,
            &ProposedParticularOpenTypedSubstitutionV1 {
                domain_context: domain.clone(),
                codomain_context: DependentContext::default(),
                images: vec![],
            },
        )
        .expect("unique morphism into the empty codomain");
        assert_eq!(
            substitution.raw_codomain_context_judgment().context(),
            &DependentContext::default()
        );
        assert_eq!(
            substitution.raw_domain_context_judgment().context(),
            &domain
        );
        assert_eq!(
            substitution.canonical_codomain_context_judgment().context(),
            &DependentContext::default()
        );
        assert_eq!(
            substitution.canonical_domain_context_judgment().context(),
            &domain
        );
    }

    #[test]
    fn dependent_image_order_is_authority_bearing() {
        let (kernel, signature, protocol) = verified_inputs();
        assert!(matches!(
            verify_particular_open_typed_substitution_v1(
                &protocol,
                &kernel,
                &signature,
                &ProposedParticularOpenTypedSubstitutionV1 {
                    domain_context: DependentContext::default(),
                    codomain_context: DependentContext(vec![
                        Term::Sort { level: 0 },
                        Term::Var { index: 0 },
                    ]),
                    images: vec![Term::Unit, Term::UnitType],
                },
            ),
            Err(ParticularOpenTypedSubstitutionFailureV1::KernelRejected(_))
        ));
    }

    #[test]
    fn aggregate_material_budget_rejects_many_individually_small_images() {
        let (kernel, signature, protocol) = verified_inputs_with_limits(KernelLimits {
            max_operations: 500,
            max_depth: 64,
            normalization_fuel: 500,
        });
        let count = 150;
        let proposal = ProposedParticularOpenTypedSubstitutionV1 {
            domain_context: DependentContext::default(),
            codomain_context: DependentContext(vec![Term::UnitType; count]),
            images: vec![Term::Unit; count],
        };
        assert_eq!(
            verify_particular_open_typed_substitution_v1(&protocol, &kernel, &signature, &proposal,),
            Err(ParticularOpenTypedSubstitutionFailureV1::AggregateMaterialLimitExceeded)
        );
    }

    #[test]
    fn bounded_reindex_transform_is_exhaustive_over_all_twelve_term_forms() {
        let (kernel, _, _) = verified_inputs();
        let global = GlobalId(Digest::of_domain_bytes(
            "law-v2/jg2b2b1/exhaustive-transform-global/v1",
            b"g",
        ));
        let cases = [
            (Term::Sort { level: 0 }, Term::Sort { level: 0 }),
            (Term::Var { index: 0 }, Term::Unit),
            (
                Term::Global { id: global.clone() },
                Term::Global { id: global },
            ),
            (
                Term::Pi {
                    parameter: Box::new(Term::Var { index: 0 }),
                    body: Box::new(Term::Apply {
                        function: Box::new(Term::Var { index: 0 }),
                        argument: Box::new(Term::Var { index: 1 }),
                    }),
                },
                Term::Pi {
                    parameter: Box::new(Term::Unit),
                    body: Box::new(Term::Apply {
                        function: Box::new(Term::Var { index: 0 }),
                        argument: Box::new(Term::Unit),
                    }),
                },
            ),
            (
                Term::Sigma {
                    parameter: Box::new(Term::Var { index: 0 }),
                    body: Box::new(Term::Var { index: 1 }),
                },
                Term::Sigma {
                    parameter: Box::new(Term::Unit),
                    body: Box::new(Term::Unit),
                },
            ),
            (
                Term::Lambda {
                    parameter_type: Box::new(Term::Var { index: 0 }),
                    body: Box::new(Term::Var { index: 1 }),
                },
                Term::Lambda {
                    parameter_type: Box::new(Term::Unit),
                    body: Box::new(Term::Unit),
                },
            ),
            (
                Term::Apply {
                    function: Box::new(Term::Var { index: 0 }),
                    argument: Box::new(Term::Var { index: 0 }),
                },
                Term::Apply {
                    function: Box::new(Term::Unit),
                    argument: Box::new(Term::Unit),
                },
            ),
            (
                Term::Pair {
                    sigma_type: Box::new(Term::Var { index: 0 }),
                    first: Box::new(Term::Var { index: 0 }),
                    second: Box::new(Term::Var { index: 0 }),
                },
                Term::Pair {
                    sigma_type: Box::new(Term::Unit),
                    first: Box::new(Term::Unit),
                    second: Box::new(Term::Unit),
                },
            ),
            (
                Term::First {
                    pair: Box::new(Term::Var { index: 0 }),
                },
                Term::First {
                    pair: Box::new(Term::Unit),
                },
            ),
            (
                Term::Second {
                    pair: Box::new(Term::Var { index: 0 }),
                },
                Term::Second {
                    pair: Box::new(Term::Unit),
                },
            ),
            (Term::UnitType, Term::UnitType),
            (Term::Unit, Term::Unit),
        ];
        let mut budget = LocalStructuralBudgetV1::new(&kernel);
        for (input, expected) in cases {
            assert_eq!(
                reindex_term_bounded_v1(&input, 1, &[Term::Unit], 0, 0, &mut budget)
                    .expect("bounded exhaustive transform"),
                expected
            );
        }
        assert_eq!(
            reindex_term_bounded_v1(&Term::Var { index: 1 }, 1, &[Term::Unit], 0, 0, &mut budget,),
            Err(ParticularOpenTypedSubstitutionFailureV1::VariableOutOfScope)
        );
        assert_eq!(
            shift_term_up_bounded_v1(&Term::Var { index: u32::MAX }, 1, 0, 0, &mut budget,),
            Err(ParticularOpenTypedSubstitutionFailureV1::VariableIndexOverflow)
        );
    }

    #[test]
    fn identity_is_one_rechecked_dependent_instance() {
        let (kernel, signature, protocol) = verified_inputs();
        let context = DependentContext(vec![Term::Sort { level: 0 }, Term::Var { index: 0 }]);
        let identity = verify_particular_identity_open_typed_substitution_v1(
            &protocol, &kernel, &signature, &context,
        )
        .expect("identity instance");
        assert_eq!(
            identity.result().canonical_images(),
            &[Term::Var { index: 1 }, Term::Var { index: 0 }]
        );
        assert_eq!(identity.context_input(), &context);
    }

    #[test]
    fn composition_is_capture_avoiding_and_retains_both_parents() {
        let (kernel, signature, protocol) = verified_inputs();
        let delta = DependentContext(vec![Term::UnitType]);
        let gamma = DependentContext(vec![Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::UnitType),
        }]);
        let outer = verify_particular_open_typed_substitution_v1(
            &protocol,
            &kernel,
            &signature,
            &ProposedParticularOpenTypedSubstitutionV1 {
                domain_context: delta.clone(),
                codomain_context: gamma.clone(),
                images: vec![Term::Lambda {
                    parameter_type: Box::new(Term::UnitType),
                    body: Box::new(Term::Var { index: 1 }),
                }],
            },
        )
        .expect("outer");
        let inner = verify_particular_open_typed_substitution_v1(
            &protocol,
            &kernel,
            &signature,
            &ProposedParticularOpenTypedSubstitutionV1 {
                domain_context: DependentContext::default(),
                codomain_context: delta,
                images: vec![Term::Unit],
            },
        )
        .expect("inner");
        let composite = verify_particular_composite_open_typed_substitution_v1(
            &protocol, &kernel, &signature, &outer, &inner,
        )
        .expect("particular composite");
        assert_eq!(composite.outer(), &outer);
        assert_eq!(composite.inner(), &inner);
        assert_eq!(composite.result().codomain_context(), &gamma);
        assert_eq!(
            composite.result().canonical_images(),
            &[Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Unit),
            }]
        );
    }

    #[test]
    fn composition_rejects_intermediate_signature_and_profile_mismatches() {
        let (kernel, signature, protocol) = verified_inputs();
        let empty_identity = verify_particular_identity_open_typed_substitution_v1(
            &protocol,
            &kernel,
            &signature,
            &DependentContext::default(),
        )
        .expect("empty identity");
        let unit_identity = verify_particular_identity_open_typed_substitution_v1(
            &protocol,
            &kernel,
            &signature,
            &DependentContext(vec![Term::UnitType]),
        )
        .expect("unit identity");
        assert_eq!(
            verify_particular_composite_open_typed_substitution_v1(
                &protocol,
                &kernel,
                &signature,
                unit_identity.result(),
                empty_identity.result(),
            ),
            Err(ParticularOpenTypedSubstitutionFailureV1::IntermediateContextMismatch)
        );

        let other_signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![pen_kernel::Declaration {
                    id: GlobalId(Digest::of_domain_bytes(
                        "law-v2/jg2b2b1/signature-mismatch/v1",
                        b"u",
                    )),
                    ty: Term::UnitType,
                    body: None,
                }],
            })
            .expect("other verified signature");
        assert_eq!(
            verify_particular_composite_open_typed_substitution_v1(
                &protocol,
                &kernel,
                &other_signature,
                empty_identity.result(),
                empty_identity.result(),
            ),
            Err(ParticularOpenTypedSubstitutionFailureV1::SignatureMismatch)
        );

        let (other_kernel, other_signature, other_protocol) =
            verified_inputs_with_limits(KernelLimits {
                max_operations: 90_000,
                max_depth: 200,
                normalization_fuel: 40_000,
            });
        assert_eq!(
            verify_particular_composite_open_typed_substitution_v1(
                &other_protocol,
                &other_kernel,
                &other_signature,
                empty_identity.result(),
                empty_identity.result(),
            ),
            Err(ParticularOpenTypedSubstitutionFailureV1::ProtocolKernelConfigurationMismatch)
        );
    }

    #[test]
    fn binder_lift_retains_parameter_reindexing_and_rechecks_result() {
        let (kernel, signature, protocol) = verified_inputs();
        let context = DependentContext(vec![Term::Sort { level: 0 }]);
        let identity = verify_particular_identity_open_typed_substitution_v1(
            &protocol, &kernel, &signature, &context,
        )
        .expect("identity");
        let lifted = verify_particular_binder_lift_open_typed_substitution_v1(
            &protocol,
            &kernel,
            &signature,
            identity.result(),
            &Term::Var { index: 0 },
        )
        .expect("dependent lift");
        assert!(matches!(
            lifted.parameter_reindexing().normalized_original_judgment(),
            OpenJudgment::TypeFormation { .. }
        ));
        assert!(matches!(
            lifted
                .parameter_reindexing()
                .normalized_reindexed_judgment(),
            OpenJudgment::TypeFormation { .. }
        ));
        assert_eq!(
            lifted.result().canonical_images(),
            &[Term::Var { index: 1 }, Term::Var { index: 0 }]
        );
    }

    #[test]
    fn all_three_open_judgment_forms_are_derived_and_replayed() {
        let (kernel, signature, protocol) = verified_inputs();
        let substitution = dependent_unit_substitution(&protocol, &kernel, &signature);
        let context = substitution.codomain_context().clone();
        let judgments = [
            OpenJudgment::TypeFormation {
                context: context.clone(),
                term: Term::Var { index: 1 },
            },
            OpenJudgment::HasType {
                context: context.clone(),
                term: Term::Var { index: 0 },
                ty: Term::Var { index: 1 },
            },
            OpenJudgment::DefinitionallyEqual {
                context,
                left: Term::Var { index: 0 },
                right: Term::Var { index: 0 },
                ty: Term::Var { index: 1 },
            },
        ];
        for judgment in judgments {
            let replay = verify_particular_reindexed_open_judgment_v1(
                &protocol,
                &kernel,
                &signature,
                &substitution,
                &judgment,
            )
            .expect("particular judgment replay");
            assert_eq!(replay.original_judgment(), &judgment);
            assert_eq!(
                replay.normalized_reindexed_judgment().context(),
                substitution.domain_context()
            );
        }
    }

    fn nested_first(depth: usize) -> Term {
        let mut term = Term::Unit;
        for _ in 0..depth {
            term = Term::First {
                pair: Box::new(term),
            };
        }
        term
    }

    #[test]
    fn reindex_preflights_deep_original_before_context_equality() {
        let (kernel, signature, protocol) = verified_inputs();
        let identity = verify_particular_identity_open_typed_substitution_v1(
            &protocol,
            &kernel,
            &signature,
            &DependentContext::default(),
        )
        .expect("empty identity");
        let deep = nested_first(10_000);
        let judgment = OpenJudgment::TypeFormation {
            context: DependentContext(vec![deep]),
            term: Term::UnitType,
        };
        let result = verify_particular_reindexed_open_judgment_v1(
            &protocol,
            &kernel,
            &signature,
            identity.result(),
            &judgment,
        );
        std::mem::forget(judgment);
        assert_eq!(
            result,
            Err(ParticularOpenTypedSubstitutionFailureV1::DepthBudgetExhausted)
        );
    }

    #[test]
    fn lift_preflights_deep_parameter_before_clone_or_replay() {
        let (kernel, signature, protocol) = verified_inputs();
        let identity = verify_particular_identity_open_typed_substitution_v1(
            &protocol,
            &kernel,
            &signature,
            &DependentContext::default(),
        )
        .expect("empty identity");
        let deep = nested_first(10_000);
        let result = verify_particular_binder_lift_open_typed_substitution_v1(
            &protocol,
            &kernel,
            &signature,
            identity.result(),
            &deep,
        );
        std::mem::forget(deep);
        assert_eq!(
            result,
            Err(ParticularOpenTypedSubstitutionFailureV1::DepthBudgetExhausted)
        );
    }

    #[test]
    fn direct_verifier_preflights_ten_thousand_depth_before_clone_or_hash() {
        let (kernel, signature, protocol) = verified_inputs();
        let deep = nested_first(10_000);
        let proposal = ProposedParticularOpenTypedSubstitutionV1 {
            domain_context: DependentContext(vec![deep]),
            codomain_context: DependentContext::default(),
            images: vec![],
        };
        let result =
            verify_particular_open_typed_substitution_v1(&protocol, &kernel, &signature, &proposal);
        std::mem::forget(proposal);
        assert_eq!(
            result,
            Err(ParticularOpenTypedSubstitutionFailureV1::DepthBudgetExhausted)
        );
    }
}
