//! JG2b2a entry protocol for substitution and naturality.
//!
//! This module freezes the premises that the later JG2b2b generic theorem and
//! JG2b2c executable occurrence census must satisfy. It deliberately does not
//! derive a public occurrence, an admissible substitution, a naturality
//! square, or any downstream factual authority. In particular, finite direct
//! witnesses and replayed particular closed specializations are not promoted
//! to a generic universal theorem.

use crate::{
    COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_RESOURCE_POLICY_VERSION_V1,
    COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_SCHEMA_VERSION_V1,
    VerifiedGenerativeCapabilityConstructorGrammarV1,
    VerifiedPreExposureGenerativeCapabilityGrammarV1,
    generative_capability_kernel_configuration_digest_v1,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, Kernel};

pub const GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_SCHEMA_VERSION_V1: u16 = 1;
pub const GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_RULE_COUNT_V1: usize = 19;

const GENERATIVE_SUBSTITUTION_NATURALITY_RULES_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2a/substitution-naturality-entry-rules/v1";
const GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2a/substitution-naturality-entry-protocol/v1";
const GENERATIVE_SUBSTITUTION_NATURALITY_RULES_ROOT_TAG_V1: u8 = 0xf1;
const GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_ROOT_TAG_V1: u8 = 0xf2;

/// Closed, ordered JG2b2a contract.
///
/// These rules distinguish the finite census of declaration births and
/// direct witnesses from the generic substitution/naturality theorem that a
/// positive census row must cite. The vocabulary has no caller extension
/// point and no catch-all variant.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeSubstitutionNaturalityProtocolRuleV1 {
    OpaqueCompleteThroughHeadIsSoleHistoryInput,
    InterfaceRealizationsAndEqualitiesAreExecutableAndKernelChecked,
    /// JG2b2b must freeze the structural occurrence grammar before JG2b2c
    /// enumerates it. Identity retains history, birth, declaration, root
    /// field, binder, and structural path, with normalized subject plus
    /// separate exact evidence; equal terms at distinct paths stay distinct.
    OccurrenceGrammarAndStructuralPathIdentityAreExecutableAndFrozen,
    BirthOccurrencesDeriveExactEventGlobalAndPathIdentity,
    ConstructorAndRoleTagsAreNeverCallerInputs,
    /// There is one verifier-derived typed classifier disposition for every
    /// occurrence/constructor pair, rather than one aggregate caller label.
    EveryOccurrenceConstructorPairHasExactlyOneDerivedTypedDisposition,
    /// The matrix proves exact `occurrence_count * constructor_count`
    /// cardinality, with neither omitted nor duplicated pairs.
    PairDispositionMatrixHasExactCardinalityAndCoverage,
    PairResultsAggregateToUniqueZeroOrTypedAmbiguity,
    PublicContextsAndBinderOrientationAreDerived,
    SourceTargetContextsAndDependentImagesAreKernelTyped,
    /// The generic package proves typed substitution, identity, composition,
    /// binder lifting, typing preservation, typed-equality preservation, and
    /// normalization/reindexing compatibility as one explicit obligation.
    IdentityCompositionLiftingTypingEqualityAndNormalizationRequireGenericTheorems,
    FiniteDirectWitnessesAndParticularClosedSpecializationsNeverEstablishUniversality,
    EveryFrozenConstructorHasDefinedNaturalitySquareAndReindexingAction,
    /// A positive census row retains verifier-minted theorem-application
    /// evidence binding the full normalized implementation, exact local
    /// context, typed inputs and outputs, reindexing actions, both square
    /// endpoints, and kernel-checked equality to the generic theorem subject.
    /// A constructor tag, interface shape, or theorem digest is insufficient.
    PositiveRowsBindFullConcreteTheoremApplicationEvidence,
    FiniteBirthOccurrenceCensusLinksEveryOccurrenceToGenericTheoremOrTypedNegative,
    ResourceExhaustionAbortsWithoutPositiveOrNegativeFact,
    CallerLaterHeldOutOracleAndProfileEvidenceAreForbidden,
    ProtocolMintsNoFactualHistoryOccurrenceSubstitutionOrNaturalityAuthority,
    ProtocolMintsNoCarrierQuotientGainBootstrapOrSelectionAuthority,
}

pub const GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_RULES_V1:
    [GenerativeSubstitutionNaturalityProtocolRuleV1;
        GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_RULE_COUNT_V1] = [
    GenerativeSubstitutionNaturalityProtocolRuleV1::OpaqueCompleteThroughHeadIsSoleHistoryInput,
    GenerativeSubstitutionNaturalityProtocolRuleV1::InterfaceRealizationsAndEqualitiesAreExecutableAndKernelChecked,
    GenerativeSubstitutionNaturalityProtocolRuleV1::OccurrenceGrammarAndStructuralPathIdentityAreExecutableAndFrozen,
    GenerativeSubstitutionNaturalityProtocolRuleV1::BirthOccurrencesDeriveExactEventGlobalAndPathIdentity,
    GenerativeSubstitutionNaturalityProtocolRuleV1::ConstructorAndRoleTagsAreNeverCallerInputs,
    GenerativeSubstitutionNaturalityProtocolRuleV1::EveryOccurrenceConstructorPairHasExactlyOneDerivedTypedDisposition,
    GenerativeSubstitutionNaturalityProtocolRuleV1::PairDispositionMatrixHasExactCardinalityAndCoverage,
    GenerativeSubstitutionNaturalityProtocolRuleV1::PairResultsAggregateToUniqueZeroOrTypedAmbiguity,
    GenerativeSubstitutionNaturalityProtocolRuleV1::PublicContextsAndBinderOrientationAreDerived,
    GenerativeSubstitutionNaturalityProtocolRuleV1::SourceTargetContextsAndDependentImagesAreKernelTyped,
    GenerativeSubstitutionNaturalityProtocolRuleV1::IdentityCompositionLiftingTypingEqualityAndNormalizationRequireGenericTheorems,
    GenerativeSubstitutionNaturalityProtocolRuleV1::FiniteDirectWitnessesAndParticularClosedSpecializationsNeverEstablishUniversality,
    GenerativeSubstitutionNaturalityProtocolRuleV1::EveryFrozenConstructorHasDefinedNaturalitySquareAndReindexingAction,
    GenerativeSubstitutionNaturalityProtocolRuleV1::PositiveRowsBindFullConcreteTheoremApplicationEvidence,
    GenerativeSubstitutionNaturalityProtocolRuleV1::FiniteBirthOccurrenceCensusLinksEveryOccurrenceToGenericTheoremOrTypedNegative,
    GenerativeSubstitutionNaturalityProtocolRuleV1::ResourceExhaustionAbortsWithoutPositiveOrNegativeFact,
    GenerativeSubstitutionNaturalityProtocolRuleV1::CallerLaterHeldOutOracleAndProfileEvidenceAreForbidden,
    GenerativeSubstitutionNaturalityProtocolRuleV1::ProtocolMintsNoFactualHistoryOccurrenceSubstitutionOrNaturalityAuthority,
    GenerativeSubstitutionNaturalityProtocolRuleV1::ProtocolMintsNoCarrierQuotientGainBootstrapOrSelectionAuthority,
];

impl CanonicalEncode for GenerativeSubstitutionNaturalityProtocolRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::OpaqueCompleteThroughHeadIsSoleHistoryInput => 0x40,
            Self::InterfaceRealizationsAndEqualitiesAreExecutableAndKernelChecked => 0x41,
            Self::OccurrenceGrammarAndStructuralPathIdentityAreExecutableAndFrozen => 0x42,
            Self::BirthOccurrencesDeriveExactEventGlobalAndPathIdentity => 0x43,
            Self::ConstructorAndRoleTagsAreNeverCallerInputs => 0x44,
            Self::EveryOccurrenceConstructorPairHasExactlyOneDerivedTypedDisposition => 0x45,
            Self::PairDispositionMatrixHasExactCardinalityAndCoverage => 0x46,
            Self::PairResultsAggregateToUniqueZeroOrTypedAmbiguity => 0x47,
            Self::PublicContextsAndBinderOrientationAreDerived => 0x48,
            Self::SourceTargetContextsAndDependentImagesAreKernelTyped => 0x49,
            Self::IdentityCompositionLiftingTypingEqualityAndNormalizationRequireGenericTheorems => 0x4a,
            Self::FiniteDirectWitnessesAndParticularClosedSpecializationsNeverEstablishUniversality => 0x4b,
            Self::EveryFrozenConstructorHasDefinedNaturalitySquareAndReindexingAction => 0x4c,
            Self::PositiveRowsBindFullConcreteTheoremApplicationEvidence => 0x4d,
            Self::FiniteBirthOccurrenceCensusLinksEveryOccurrenceToGenericTheoremOrTypedNegative => 0x4e,
            Self::ResourceExhaustionAbortsWithoutPositiveOrNegativeFact => 0x4f,
            Self::CallerLaterHeldOutOracleAndProfileEvidenceAreForbidden => 0x50,
            Self::ProtocolMintsNoFactualHistoryOccurrenceSubstitutionOrNaturalityAuthority => 0x51,
            Self::ProtocolMintsNoCarrierQuotientGainBootstrapOrSelectionAuthority => 0x52,
        });
    }
}

/// Exact unverified JG2b2a rule proposal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerativeSubstitutionNaturalityRulesManifestV1 {
    pub schema_version: u16,
    pub rules: Vec<GenerativeSubstitutionNaturalityProtocolRuleV1>,
}

impl CanonicalEncode for GenerativeSubstitutionNaturalityRulesManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(GENERATIVE_SUBSTITUTION_NATURALITY_RULES_ROOT_TAG_V1);
        encoder.u16(self.schema_version);
        encoder.sequence(&self.rules);
    }
}

pub fn proposed_generative_substitution_naturality_rules_v1()
-> GenerativeSubstitutionNaturalityRulesManifestV1 {
    GenerativeSubstitutionNaturalityRulesManifestV1 {
        schema_version: GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_SCHEMA_VERSION_V1,
        rules: GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_RULES_V1.to_vec(),
    }
}

/// Frozen 30-byte canonical JG2b2a rule transcript.
pub const CANONICAL_GENERATIVE_SUBSTITUTION_NATURALITY_RULES_BYTES_V1: &[u8] = &[
    0xf1, 0x01, 0x00, 0x13, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x41, 0x42, 0x43, 0x44,
    0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50, 0x51, 0x52,
];

/// Frozen domain-separated identity of the JG2b2a rule transcript.
pub const CANONICAL_GENERATIVE_SUBSTITUTION_NATURALITY_RULES_DIGEST_V1: &str =
    "blake3:880e153e750f17d93723e16d5febcd39655cae9a35fe6b44a3c9b86c32a6f0c9";

/// Complete unverified JG2b2a protocol proposal.
///
/// A protocol is reusable grammar authority. It binds the prerequisite
/// complete-history schema/resource versions and the supplied grammar/kernel
/// identities, but contains no particular complete history or factual census.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerativeSubstitutionNaturalityProtocolManifestV1 {
    pub rules: GenerativeSubstitutionNaturalityRulesManifestV1,
    pub complete_history_schema_version: u16,
    pub complete_history_resource_policy_version: u16,
    pub jg1_manifest_digest: Digest,
    pub constructor_grammar_manifest_digest: Digest,
    pub scope_grammar_digest: Digest,
    pub kernel_protocol_digest: Digest,
    pub normalizer_protocol_digest: Digest,
    pub kernel_configuration_digest: Digest,
}

impl CanonicalEncode for GenerativeSubstitutionNaturalityProtocolManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_ROOT_TAG_V1);
        self.rules.encode_canonical(encoder);
        encoder.u16(self.complete_history_schema_version);
        encoder.u16(self.complete_history_resource_policy_version);
        self.jg1_manifest_digest.encode_canonical(encoder);
        self.constructor_grammar_manifest_digest
            .encode_canonical(encoder);
        self.scope_grammar_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.kernel_configuration_digest.encode_canonical(encoder);
    }
}

pub fn proposed_generative_substitution_naturality_protocol_v1(
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
) -> GenerativeSubstitutionNaturalityProtocolManifestV1 {
    GenerativeSubstitutionNaturalityProtocolManifestV1 {
        rules: proposed_generative_substitution_naturality_rules_v1(),
        complete_history_schema_version:
            COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_SCHEMA_VERSION_V1,
        complete_history_resource_policy_version:
            COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_RESOURCE_POLICY_VERSION_V1,
        jg1_manifest_digest: jg1.manifest_digest().clone(),
        constructor_grammar_manifest_digest: constructors.manifest_digest().clone(),
        scope_grammar_digest: constructors.scope_grammar_digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        kernel_configuration_digest: generative_capability_kernel_configuration_digest_v1(kernel),
    }
}

/// Fail-closed reasons why the exact JG2b2a entry protocol was not minted.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerativeSubstitutionNaturalityProtocolFailureV1 {
    SchemaVersionMismatch,
    CompleteHistorySchemaVersionMismatch,
    CompleteHistoryResourcePolicyVersionMismatch,
    ConstructorGrammarJg1Mismatch,
    Jg1ManifestDigestMismatch,
    ConstructorGrammarManifestDigestMismatch,
    ScopeGrammarDigestMismatch,
    RuleVocabularyMismatch,
    CanonicalRulesTranscriptMismatch,
    CanonicalRulesDigestMismatch,
    KernelProtocolMismatch,
    NormalizerProtocolMismatch,
    KernelConfigurationMismatch,
}

impl std::fmt::Display for GenerativeSubstitutionNaturalityProtocolFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::SchemaVersionMismatch => {
                "the substitution/naturality protocol schema version is not frozen V1"
            }
            Self::CompleteHistorySchemaVersionMismatch => {
                "the protocol targets a different complete-history schema"
            }
            Self::CompleteHistoryResourcePolicyVersionMismatch => {
                "the protocol targets a different complete-history resource policy"
            }
            Self::ConstructorGrammarJg1Mismatch => {
                "the verified constructor grammar is not bound to the supplied JG1 grammar"
            }
            Self::Jg1ManifestDigestMismatch => {
                "the protocol manifest is bound to a different JG1 grammar"
            }
            Self::ConstructorGrammarManifestDigestMismatch => {
                "the protocol manifest is bound to a different constructor grammar"
            }
            Self::ScopeGrammarDigestMismatch => {
                "the protocol manifest is bound to a different combined scope grammar"
            }
            Self::RuleVocabularyMismatch => {
                "the substitution/naturality rules are not the exact closed vocabulary"
            }
            Self::CanonicalRulesTranscriptMismatch => {
                "the typed substitution/naturality rules do not encode to the frozen transcript"
            }
            Self::CanonicalRulesDigestMismatch => {
                "the canonical rule transcript has the wrong domain-separated digest"
            }
            Self::KernelProtocolMismatch => {
                "the protocol manifest is bound to a different kernel protocol"
            }
            Self::NormalizerProtocolMismatch => {
                "the protocol manifest is bound to a different normalizer protocol"
            }
            Self::KernelConfigurationMismatch => {
                "the protocol manifest is bound to a different kernel resource configuration"
            }
        })
    }
}

impl std::error::Error for GenerativeSubstitutionNaturalityProtocolFailureV1 {}

/// Opaque proof of the exact JG2b2a entry grammar.
///
/// This cloneable token is protocol authority only. It contains no complete
/// history, birth occurrence, interface realization, public context,
/// substitution, direct witness, naturality result, carrier member, quotient,
/// gain, bootstrap value, or selection result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedGenerativeSubstitutionNaturalityProtocolV1 {
    schema_version: u16,
    complete_history_schema_version: u16,
    complete_history_resource_policy_version: u16,
    rules_digest: Digest,
    jg1_manifest_digest: Digest,
    constructor_grammar_manifest_digest: Digest,
    scope_grammar_digest: Digest,
    kernel_protocol_digest: Digest,
    normalizer_protocol_digest: Digest,
    kernel_configuration_digest: Digest,
    manifest_digest: Digest,
}

impl VerifiedGenerativeSubstitutionNaturalityProtocolV1 {
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub const fn complete_history_schema_version(&self) -> u16 {
        self.complete_history_schema_version
    }

    pub const fn complete_history_resource_policy_version(&self) -> u16 {
        self.complete_history_resource_policy_version
    }

    pub fn rules_digest(&self) -> &Digest {
        &self.rules_digest
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

pub fn verify_generative_substitution_naturality_protocol_v1(
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
    manifest: &GenerativeSubstitutionNaturalityProtocolManifestV1,
) -> Result<
    VerifiedGenerativeSubstitutionNaturalityProtocolV1,
    GenerativeSubstitutionNaturalityProtocolFailureV1,
> {
    if manifest.rules.schema_version
        != GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_SCHEMA_VERSION_V1
    {
        return Err(GenerativeSubstitutionNaturalityProtocolFailureV1::SchemaVersionMismatch);
    }
    if manifest.complete_history_schema_version
        != COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_SCHEMA_VERSION_V1
    {
        return Err(
            GenerativeSubstitutionNaturalityProtocolFailureV1::CompleteHistorySchemaVersionMismatch,
        );
    }
    if manifest.complete_history_resource_policy_version
        != COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_RESOURCE_POLICY_VERSION_V1
    {
        return Err(
            GenerativeSubstitutionNaturalityProtocolFailureV1::CompleteHistoryResourcePolicyVersionMismatch,
        );
    }
    if constructors.jg1_manifest_digest() != jg1.manifest_digest() {
        return Err(
            GenerativeSubstitutionNaturalityProtocolFailureV1::ConstructorGrammarJg1Mismatch,
        );
    }
    if manifest.jg1_manifest_digest != *jg1.manifest_digest() {
        return Err(GenerativeSubstitutionNaturalityProtocolFailureV1::Jg1ManifestDigestMismatch);
    }
    if manifest.constructor_grammar_manifest_digest != *constructors.manifest_digest() {
        return Err(
            GenerativeSubstitutionNaturalityProtocolFailureV1::ConstructorGrammarManifestDigestMismatch,
        );
    }
    if manifest.scope_grammar_digest != *constructors.scope_grammar_digest() {
        return Err(GenerativeSubstitutionNaturalityProtocolFailureV1::ScopeGrammarDigestMismatch);
    }
    if manifest.rules != proposed_generative_substitution_naturality_rules_v1() {
        return Err(GenerativeSubstitutionNaturalityProtocolFailureV1::RuleVocabularyMismatch);
    }

    let mut rules_encoder = CanonicalEncoder::new();
    manifest.rules.encode_canonical(&mut rules_encoder);
    if rules_encoder.as_bytes() != CANONICAL_GENERATIVE_SUBSTITUTION_NATURALITY_RULES_BYTES_V1 {
        return Err(
            GenerativeSubstitutionNaturalityProtocolFailureV1::CanonicalRulesTranscriptMismatch,
        );
    }
    let rules_digest = Digest::of_domain_bytes(
        GENERATIVE_SUBSTITUTION_NATURALITY_RULES_DIGEST_DOMAIN_V1,
        rules_encoder.as_bytes(),
    );
    if rules_digest.as_str() != CANONICAL_GENERATIVE_SUBSTITUTION_NATURALITY_RULES_DIGEST_V1 {
        return Err(
            GenerativeSubstitutionNaturalityProtocolFailureV1::CanonicalRulesDigestMismatch,
        );
    }

    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    if manifest.kernel_protocol_digest != kernel_protocol_digest {
        return Err(GenerativeSubstitutionNaturalityProtocolFailureV1::KernelProtocolMismatch);
    }
    let normalizer_protocol_digest = kernel.normalizer_protocol_digest();
    if manifest.normalizer_protocol_digest != normalizer_protocol_digest {
        return Err(GenerativeSubstitutionNaturalityProtocolFailureV1::NormalizerProtocolMismatch);
    }
    let kernel_configuration_digest = generative_capability_kernel_configuration_digest_v1(kernel);
    if manifest.kernel_configuration_digest != kernel_configuration_digest {
        return Err(GenerativeSubstitutionNaturalityProtocolFailureV1::KernelConfigurationMismatch);
    }

    let manifest_digest = Digest::of_canonical(
        GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_DIGEST_DOMAIN_V1,
        manifest,
    );
    Ok(VerifiedGenerativeSubstitutionNaturalityProtocolV1 {
        schema_version: manifest.rules.schema_version,
        complete_history_schema_version: manifest.complete_history_schema_version,
        complete_history_resource_policy_version: manifest.complete_history_resource_policy_version,
        rules_digest,
        jg1_manifest_digest: manifest.jg1_manifest_digest.clone(),
        constructor_grammar_manifest_digest: manifest.constructor_grammar_manifest_digest.clone(),
        scope_grammar_digest: manifest.scope_grammar_digest.clone(),
        kernel_protocol_digest,
        normalizer_protocol_digest,
        kernel_configuration_digest,
        manifest_digest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        proposed_generative_capability_constructor_grammar_v1,
        proposed_pre_exposure_generative_capability_grammar_v1,
        verify_generative_capability_constructor_grammar_v1,
        verify_pre_exposure_generative_capability_grammar_v1,
    };
    use pen_kernel::KernelLimits;
    use std::collections::BTreeSet;

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("valid kernel")
    }

    fn authorities() -> (
        VerifiedPreExposureGenerativeCapabilityGrammarV1,
        VerifiedGenerativeCapabilityConstructorGrammarV1,
    ) {
        let jg1 = verify_pre_exposure_generative_capability_grammar_v1(
            &proposed_pre_exposure_generative_capability_grammar_v1(),
        )
        .expect("exact JG1 grammar");
        let constructors = verify_generative_capability_constructor_grammar_v1(
            &jg1,
            &proposed_generative_capability_constructor_grammar_v1(&jg1),
        )
        .expect("exact constructor grammar");
        (jg1, constructors)
    }

    #[test]
    fn exact_rules_are_closed_ordered_and_pairwise_distinct() {
        let proposal = proposed_generative_substitution_naturality_rules_v1();

        assert_eq!(proposal.schema_version, 1);
        assert_eq!(
            proposal.rules.as_slice(),
            GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_RULES_V1
        );
        assert_eq!(proposal.rules.len(), 19);

        let encodings = proposal
            .rules
            .iter()
            .map(|rule| {
                let mut encoder = CanonicalEncoder::new();
                rule.encode_canonical(&mut encoder);
                encoder.as_bytes().to_vec()
            })
            .collect::<Vec<_>>();
        assert!(encodings.iter().all(|encoding| encoding.len() == 1));
        assert_eq!(
            encodings.into_iter().collect::<BTreeSet<_>>().len(),
            GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_RULE_COUNT_V1
        );
    }

    #[test]
    fn canonical_rules_transcript_and_digest_are_frozen() {
        let proposal = proposed_generative_substitution_naturality_rules_v1();
        let mut encoder = CanonicalEncoder::new();
        proposal.encode_canonical(&mut encoder);

        assert_eq!(encoder.as_bytes().len(), 30);
        assert_eq!(
            encoder.as_bytes(),
            CANONICAL_GENERATIVE_SUBSTITUTION_NATURALITY_RULES_BYTES_V1
        );
        assert_eq!(
            Digest::of_domain_bytes(
                GENERATIVE_SUBSTITUTION_NATURALITY_RULES_DIGEST_DOMAIN_V1,
                encoder.as_bytes(),
            )
            .as_str(),
            CANONICAL_GENERATIVE_SUBSTITUTION_NATURALITY_RULES_DIGEST_V1
        );
    }

    #[test]
    fn synthetic_full_protocol_manifest_codec_fixture_is_frozen() {
        let fixture = GenerativeSubstitutionNaturalityProtocolManifestV1 {
            rules: proposed_generative_substitution_naturality_rules_v1(),
            complete_history_schema_version: 0x0102,
            complete_history_resource_policy_version: 0x0304,
            jg1_manifest_digest: Digest::of_bytes(b"jg1-codec-fixture"),
            constructor_grammar_manifest_digest: Digest::of_bytes(b"constructor-codec-fixture"),
            scope_grammar_digest: Digest::of_bytes(b"scope-codec-fixture"),
            kernel_protocol_digest: Digest::of_bytes(b"kernel-codec-fixture"),
            normalizer_protocol_digest: Digest::of_bytes(b"normalizer-codec-fixture"),
            kernel_configuration_digest: Digest::of_bytes(b"configuration-codec-fixture"),
        };

        let mut encoder = CanonicalEncoder::new();
        fixture.encode_canonical(&mut encoder);
        assert_eq!(encoder.as_bytes().len(), 509);
        assert_eq!(
            Digest::of_domain_bytes(
                GENERATIVE_SUBSTITUTION_NATURALITY_PROTOCOL_DIGEST_DOMAIN_V1,
                encoder.as_bytes(),
            )
            .as_str(),
            "blake3:769c28c5d47b2231218f48f31d2e6995aa4a75d842b088d232919efc334a7812"
        );
    }

    #[test]
    fn exact_protocol_verifies_and_remints_deterministically() {
        let kernel = kernel();
        let (jg1, constructors) = authorities();
        let proposal =
            proposed_generative_substitution_naturality_protocol_v1(&jg1, &constructors, &kernel);
        let first = verify_generative_substitution_naturality_protocol_v1(
            &jg1,
            &constructors,
            &kernel,
            &proposal,
        )
        .expect("exact protocol");
        let second = verify_generative_substitution_naturality_protocol_v1(
            &jg1,
            &constructors,
            &kernel,
            &proposal,
        )
        .expect("deterministic remint");

        assert_eq!(first, second);
        assert_eq!(first.schema_version(), 1);
        assert_eq!(
            first.complete_history_schema_version(),
            COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_SCHEMA_VERSION_V1
        );
        assert_eq!(
            first.complete_history_resource_policy_version(),
            COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_RESOURCE_POLICY_VERSION_V1
        );
        assert_eq!(
            first.rules_digest().as_str(),
            CANONICAL_GENERATIVE_SUBSTITUTION_NATURALITY_RULES_DIGEST_V1
        );
        assert_eq!(first.jg1_manifest_digest(), jg1.manifest_digest());
        assert_eq!(
            first.constructor_grammar_manifest_digest(),
            constructors.manifest_digest()
        );
        assert_eq!(
            first.scope_grammar_digest(),
            constructors.scope_grammar_digest()
        );
        assert_eq!(
            first.kernel_protocol_digest(),
            &kernel.kernel_protocol_digest()
        );
        assert_eq!(
            first.normalizer_protocol_digest(),
            &kernel.normalizer_protocol_digest()
        );
        assert_eq!(
            first.kernel_configuration_digest(),
            &generative_capability_kernel_configuration_digest_v1(&kernel)
        );
        assert_eq!(first.manifest_digest(), second.manifest_digest());
    }

    #[test]
    fn schema_and_complete_history_version_mutations_fail_closed() {
        let kernel = kernel();
        let (jg1, constructors) = authorities();
        let proposal =
            proposed_generative_substitution_naturality_protocol_v1(&jg1, &constructors, &kernel);

        let mut mutation = proposal.clone();
        mutation.rules.schema_version += 1;
        assert_eq!(
            verify_generative_substitution_naturality_protocol_v1(
                &jg1,
                &constructors,
                &kernel,
                &mutation,
            ),
            Err(GenerativeSubstitutionNaturalityProtocolFailureV1::SchemaVersionMismatch)
        );

        let mut mutation = proposal.clone();
        mutation.complete_history_schema_version += 1;
        assert_eq!(
            verify_generative_substitution_naturality_protocol_v1(
                &jg1,
                &constructors,
                &kernel,
                &mutation,
            ),
            Err(
                GenerativeSubstitutionNaturalityProtocolFailureV1::CompleteHistorySchemaVersionMismatch
            )
        );

        let mut mutation = proposal;
        mutation.complete_history_resource_policy_version += 1;
        assert_eq!(
            verify_generative_substitution_naturality_protocol_v1(
                &jg1,
                &constructors,
                &kernel,
                &mutation,
            ),
            Err(
                GenerativeSubstitutionNaturalityProtocolFailureV1::CompleteHistoryResourcePolicyVersionMismatch
            )
        );
    }

    #[test]
    fn every_upstream_authority_binding_mutation_fails_closed() {
        let kernel = kernel();
        let (jg1, constructors) = authorities();
        let proposal =
            proposed_generative_substitution_naturality_protocol_v1(&jg1, &constructors, &kernel);

        let mutations = [
            (
                0_u8,
                GenerativeSubstitutionNaturalityProtocolFailureV1::Jg1ManifestDigestMismatch,
            ),
            (
                1_u8,
                GenerativeSubstitutionNaturalityProtocolFailureV1::ConstructorGrammarManifestDigestMismatch,
            ),
            (
                2_u8,
                GenerativeSubstitutionNaturalityProtocolFailureV1::ScopeGrammarDigestMismatch,
            ),
        ];
        for (field, expected) in mutations {
            let mut mutation = proposal.clone();
            match field {
                0 => mutation.jg1_manifest_digest = Digest::of_bytes(b"wrong-jg1"),
                1 => {
                    mutation.constructor_grammar_manifest_digest =
                        Digest::of_bytes(b"wrong-constructors")
                }
                2 => mutation.scope_grammar_digest = Digest::of_bytes(b"wrong-scope"),
                _ => unreachable!(),
            }
            assert_eq!(
                verify_generative_substitution_naturality_protocol_v1(
                    &jg1,
                    &constructors,
                    &kernel,
                    &mutation,
                ),
                Err(expected)
            );
        }
    }

    #[test]
    fn rule_omission_duplication_reorder_and_replacement_fail_closed() {
        let kernel = kernel();
        let (jg1, constructors) = authorities();
        let proposal =
            proposed_generative_substitution_naturality_protocol_v1(&jg1, &constructors, &kernel);

        let mut mutations = Vec::new();
        let mut omitted = proposal.clone();
        omitted.rules.rules.pop();
        mutations.push(omitted);

        let mut duplicated = proposal.clone();
        duplicated.rules.rules[1] = duplicated.rules.rules[0];
        mutations.push(duplicated);

        let mut reordered = proposal.clone();
        reordered.rules.rules.swap(0, 1);
        mutations.push(reordered);

        let mut replaced = proposal;
        replaced.rules.rules[8] =
            GenerativeSubstitutionNaturalityProtocolRuleV1::ResourceExhaustionAbortsWithoutPositiveOrNegativeFact;
        mutations.push(replaced);

        for mutation in mutations {
            assert_eq!(
                verify_generative_substitution_naturality_protocol_v1(
                    &jg1,
                    &constructors,
                    &kernel,
                    &mutation,
                ),
                Err(GenerativeSubstitutionNaturalityProtocolFailureV1::RuleVocabularyMismatch)
            );
        }
    }

    #[test]
    fn kernel_normalizer_and_all_configuration_mutations_fail_closed() {
        let kernel = kernel();
        let (jg1, constructors) = authorities();
        let proposal =
            proposed_generative_substitution_naturality_protocol_v1(&jg1, &constructors, &kernel);

        let mut mutation = proposal.clone();
        mutation.kernel_protocol_digest = Digest::of_bytes(b"wrong-kernel");
        assert_eq!(
            verify_generative_substitution_naturality_protocol_v1(
                &jg1,
                &constructors,
                &kernel,
                &mutation,
            ),
            Err(GenerativeSubstitutionNaturalityProtocolFailureV1::KernelProtocolMismatch)
        );

        let mut mutation = proposal.clone();
        mutation.normalizer_protocol_digest = Digest::of_bytes(b"wrong-normalizer");
        assert_eq!(
            verify_generative_substitution_naturality_protocol_v1(
                &jg1,
                &constructors,
                &kernel,
                &mutation,
            ),
            Err(GenerativeSubstitutionNaturalityProtocolFailureV1::NormalizerProtocolMismatch)
        );

        let defaults = KernelLimits::default();
        let drifted_profiles = [
            KernelLimits {
                max_operations: defaults.max_operations - 1,
                ..defaults
            },
            KernelLimits {
                max_depth: defaults.max_depth - 1,
                ..defaults
            },
            KernelLimits {
                normalization_fuel: defaults.normalization_fuel - 1,
                ..defaults
            },
        ];
        for limits in drifted_profiles {
            let drifted = Kernel::new(limits).expect("valid drifted kernel");
            assert_eq!(
                verify_generative_substitution_naturality_protocol_v1(
                    &jg1,
                    &constructors,
                    &drifted,
                    &proposal,
                ),
                Err(GenerativeSubstitutionNaturalityProtocolFailureV1::KernelConfigurationMismatch)
            );

            let drifted_proposal = proposed_generative_substitution_naturality_protocol_v1(
                &jg1,
                &constructors,
                &drifted,
            );
            verify_generative_substitution_naturality_protocol_v1(
                &jg1,
                &constructors,
                &drifted,
                &drifted_proposal,
            )
            .expect("same protocol under a distinct explicitly bound resource profile");
        }
    }
}
