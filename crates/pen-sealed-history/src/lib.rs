//! JG2b1b0: target-neutral sealed-log protocol grammar.
//!
//! This crate freezes lifecycle rules only. It intentionally has no log
//! producer and mints no frame, branch-head, EOF, finalization, or history
//! authority. The terminal rule is completeness through one authoritative
//! finalized head, not a claim about a globally latest history; later or
//! forked branches have distinct heads. Resource exhaustion is always failure,
//! never EOF.

#![forbid(unsafe_code)]

use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, Kernel, KernelLimits};

pub const TARGET_NEUTRAL_SEALED_LOG_PROTOCOL_SCHEMA_VERSION_V1: u16 = 1;
/// V1 binds conforming-producer limits to the verified kernel configuration.
/// Exceeding any limit is failure and never establishes a terminal event.
pub const TARGET_NEUTRAL_SEALED_LOG_RESOURCE_POLICY_VERSION_V1: u16 = 1;
pub const TARGET_NEUTRAL_SEALED_LOG_LIFECYCLE_RULE_COUNT_V1: usize = 10;

const TARGET_NEUTRAL_SEALED_LOG_RULES_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b1b0/target-neutral-sealed-log-rules/v1";
const TARGET_NEUTRAL_SEALED_LOG_PROTOCOL_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b1b0/target-neutral-sealed-log-protocol/v1";
const TARGET_NEUTRAL_SEALED_LOG_KERNEL_CONFIGURATION_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b1b0/kernel-configuration/v1";

const TARGET_NEUTRAL_SEALED_LOG_RULES_ROOT_TAG_V1: u8 = 0xc1;
const TARGET_NEUTRAL_SEALED_LOG_PROTOCOL_ROOT_TAG_V1: u8 = 0xc2;
const TARGET_NEUTRAL_SEALED_LOG_KERNEL_CONFIGURATION_ROOT_TAG_V1: u8 = 0xc3;

/// Canonical genesis is the empty signature independently accepted by the
/// bound kernel. There is no caller-supplied initial boundary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TargetNeutralSealedLogGenesisRuleV1 {
    RequireKernelVerifiedEmptyBoundary,
}

impl CanonicalEncode for TargetNeutralSealedLogGenesisRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::RequireKernelVerifiedEmptyBoundary => encoder.tag(0x10),
        }
    }
}

/// A conforming producer must derive exactly one event delimiter from each
/// successfully committed append operation.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TargetNeutralSealedLogDelimiterRuleV1 {
    OneSuccessfulAppendCommitPerEvent,
}

impl CanonicalEncode for TargetNeutralSealedLogDelimiterRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::OneSuccessfulAppendCommitPerEvent => encoder.tag(0x11),
        }
    }
}

/// A conforming producer must commit events in one strict single-parent chain.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TargetNeutralSealedLogOrderingRuleV1 {
    StrictSingleParentHashChain,
}

impl CanonicalEncode for TargetNeutralSealedLogOrderingRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::StrictSingleParentHashChain => encoder.tag(0x12),
        }
    }
}

/// A conforming append authority is owned by one private, non-cloneable writer.
/// Supplying a transcript to a verifier is not an append operation and cannot
/// create producer authority.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TargetNeutralSealedLogIngressRuleV1 {
    PrivateLinearWriterIsSoleIngress,
}

impl CanonicalEncode for TargetNeutralSealedLogIngressRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::PrivateLinearWriterIsSoleIngress => encoder.tag(0x13),
        }
    }
}

/// Every independently opened, continued, or forked writer epoch must receive
/// a producer-minted identity distinct from every other live writer epoch.
/// This makes parallel children distinct histories rather than equivocations
/// under one branch/epoch identity.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TargetNeutralSealedLogBranchIdentityRuleV1 {
    DistinctProducerMintedWriterEpochIdentity,
}

impl CanonicalEncode for TargetNeutralSealedLogBranchIdentityRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::DistinctProducerMintedWriterEpochIdentity => encoder.tag(0x14),
        }
    }
}

/// No retrospective verifier may promote caller-supplied events, metadata, or
/// a recomputed root into producer-owned log authority.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TargetNeutralSealedLogPromotionRuleV1 {
    NoRetrospectiveTranscriptPromotion,
}

impl CanonicalEncode for TargetNeutralSealedLogPromotionRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::NoRetrospectiveTranscriptPromotion => encoder.tag(0x15),
        }
    }
}

/// Event count is bounded by `max_depth`; retained extension and terminal
/// declarations are bounded by `max_operations`; aggregate replay material is
/// bounded by `max_operations * (2 * max_depth + 1)` with checked arithmetic
/// and fallible retained-output reservation. Any reported resource failure
/// leaves the writer unclosed and can never yield a prefix, head, finalization,
/// or EOF token. An unrecoverable allocator abort likewise yields no token.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TargetNeutralSealedLogResourceRuleV1 {
    KernelProfileBoundsWithFallibleOutputReservationAndNoFailureFinalization,
}

impl CanonicalEncode for TargetNeutralSealedLogResourceRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::KernelProfileBoundsWithFallibleOutputReservationAndNoFailureFinalization => {
                encoder.tag(0x16);
            }
        }
    }
}

/// A conforming producer may issue a finalization capability only by consuming
/// the corresponding open branch state.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TargetNeutralSealedLogFinalizationRuleV1 {
    ConsumingBranchFinalization,
}

impl CanonicalEncode for TargetNeutralSealedLogFinalizationRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::ConsumingBranchFinalization => encoder.tag(0x17),
        }
    }
}

/// A finalized head is designated by the private producer while consuming its
/// owned writer state. It may never be minted by recomputing a digest, count,
/// or terminal boundary from a caller-supplied transcript.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TargetNeutralSealedLogHeadAuthorityRuleV1 {
    ProducerMintedOnlyFromConsumedOwnedWriter,
}

impl CanonicalEncode for TargetNeutralSealedLogHeadAuthorityRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::ProducerMintedOnlyFromConsumedOwnedWriter => encoder.tag(0x18),
        }
    }
}

/// The consuming JG2b1b2 verifier reconstructs the complete committed branch
/// through its authoritative finalized head and terminal public boundary
/// rather than trusting a terminal digest or count. This is not global
/// latest-history authority.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TargetNeutralSealedLogTerminalRuleV1 {
    RequireCompleteThroughFinalizedHeadByIndependentReplay,
}

impl CanonicalEncode for TargetNeutralSealedLogTerminalRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::RequireCompleteThroughFinalizedHeadByIndependentReplay => encoder.tag(0x19),
        }
    }
}

/// Exact closed JG2b1b0 lifecycle-rule vocabulary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetNeutralSealedLogRulesManifestV1 {
    pub schema_version: u16,
    pub resource_policy_version: u16,
    pub genesis_rule: TargetNeutralSealedLogGenesisRuleV1,
    pub delimiter_rule: TargetNeutralSealedLogDelimiterRuleV1,
    pub ordering_rule: TargetNeutralSealedLogOrderingRuleV1,
    pub ingress_rule: TargetNeutralSealedLogIngressRuleV1,
    pub branch_identity_rule: TargetNeutralSealedLogBranchIdentityRuleV1,
    pub promotion_rule: TargetNeutralSealedLogPromotionRuleV1,
    pub resource_rule: TargetNeutralSealedLogResourceRuleV1,
    pub finalization_rule: TargetNeutralSealedLogFinalizationRuleV1,
    pub head_authority_rule: TargetNeutralSealedLogHeadAuthorityRuleV1,
    pub terminal_rule: TargetNeutralSealedLogTerminalRuleV1,
}

impl CanonicalEncode for TargetNeutralSealedLogRulesManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(TARGET_NEUTRAL_SEALED_LOG_RULES_ROOT_TAG_V1);
        encoder.u16(self.schema_version);
        encoder.u16(self.resource_policy_version);
        self.genesis_rule.encode_canonical(encoder);
        self.delimiter_rule.encode_canonical(encoder);
        self.ordering_rule.encode_canonical(encoder);
        self.ingress_rule.encode_canonical(encoder);
        self.branch_identity_rule.encode_canonical(encoder);
        self.promotion_rule.encode_canonical(encoder);
        self.resource_rule.encode_canonical(encoder);
        self.finalization_rule.encode_canonical(encoder);
        self.head_authority_rule.encode_canonical(encoder);
        self.terminal_rule.encode_canonical(encoder);
    }
}

pub fn proposed_target_neutral_sealed_log_rules_v1() -> TargetNeutralSealedLogRulesManifestV1 {
    TargetNeutralSealedLogRulesManifestV1 {
        schema_version: TARGET_NEUTRAL_SEALED_LOG_PROTOCOL_SCHEMA_VERSION_V1,
        resource_policy_version: TARGET_NEUTRAL_SEALED_LOG_RESOURCE_POLICY_VERSION_V1,
        genesis_rule: TargetNeutralSealedLogGenesisRuleV1::RequireKernelVerifiedEmptyBoundary,
        delimiter_rule: TargetNeutralSealedLogDelimiterRuleV1::OneSuccessfulAppendCommitPerEvent,
        ordering_rule: TargetNeutralSealedLogOrderingRuleV1::StrictSingleParentHashChain,
        ingress_rule: TargetNeutralSealedLogIngressRuleV1::PrivateLinearWriterIsSoleIngress,
        branch_identity_rule:
            TargetNeutralSealedLogBranchIdentityRuleV1::DistinctProducerMintedWriterEpochIdentity,
        promotion_rule: TargetNeutralSealedLogPromotionRuleV1::NoRetrospectiveTranscriptPromotion,
        resource_rule:
            TargetNeutralSealedLogResourceRuleV1::KernelProfileBoundsWithFallibleOutputReservationAndNoFailureFinalization,
        finalization_rule: TargetNeutralSealedLogFinalizationRuleV1::ConsumingBranchFinalization,
        head_authority_rule:
            TargetNeutralSealedLogHeadAuthorityRuleV1::ProducerMintedOnlyFromConsumedOwnedWriter,
        terminal_rule:
            TargetNeutralSealedLogTerminalRuleV1::RequireCompleteThroughFinalizedHeadByIndependentReplay,
    }
}

/// Frozen 15-byte canonical JG2b1b0 lifecycle-rule transcript.
pub const CANONICAL_TARGET_NEUTRAL_SEALED_LOG_RULES_BYTES_V1: &[u8] = &[
    0xc1, 0x01, 0x00, 0x01, 0x00, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19,
];

/// Frozen domain-separated identity of the canonical lifecycle-rule
/// transcript.
pub const CANONICAL_TARGET_NEUTRAL_SEALED_LOG_RULES_DIGEST_V1: &str =
    "blake3:b1ab522d4c388abb26b8ff0384344bfd350904d32a78da44d2b410a3360f44bf";

/// Complete unverified protocol proposal. Kernel and resource configuration
/// identities are explicit so a token cannot be detached and replayed under
/// a different verifier profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TargetNeutralSealedLogProtocolManifestV1 {
    pub rules: TargetNeutralSealedLogRulesManifestV1,
    pub kernel_protocol_digest: Digest,
    pub normalizer_protocol_digest: Digest,
    pub kernel_configuration_digest: Digest,
}

impl CanonicalEncode for TargetNeutralSealedLogProtocolManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(TARGET_NEUTRAL_SEALED_LOG_PROTOCOL_ROOT_TAG_V1);
        self.rules.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.kernel_configuration_digest.encode_canonical(encoder);
    }
}

pub fn target_neutral_sealed_log_kernel_configuration_digest_v1(kernel: &Kernel) -> Digest {
    let KernelLimits {
        max_operations,
        max_depth,
        normalization_fuel,
    } = kernel.limits();
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(TARGET_NEUTRAL_SEALED_LOG_KERNEL_CONFIGURATION_ROOT_TAG_V1);
    encoder.u16(TARGET_NEUTRAL_SEALED_LOG_RESOURCE_POLICY_VERSION_V1);
    encoder.u32(max_operations);
    encoder.u16(max_depth);
    encoder.u32(normalization_fuel);
    Digest::of_domain_bytes(
        TARGET_NEUTRAL_SEALED_LOG_KERNEL_CONFIGURATION_DIGEST_DOMAIN_V1,
        encoder.as_bytes(),
    )
}

pub fn proposed_target_neutral_sealed_log_protocol_v1(
    kernel: &Kernel,
) -> TargetNeutralSealedLogProtocolManifestV1 {
    TargetNeutralSealedLogProtocolManifestV1 {
        rules: proposed_target_neutral_sealed_log_rules_v1(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        kernel_configuration_digest: target_neutral_sealed_log_kernel_configuration_digest_v1(
            kernel,
        ),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TargetNeutralSealedLogProtocolFailureV1 {
    SchemaVersionMismatch,
    ResourcePolicyVersionMismatch,
    RuleVocabularyMismatch,
    CanonicalRulesTranscriptMismatch,
    CanonicalRulesDigestMismatch,
    KernelProtocolMismatch,
    NormalizerProtocolMismatch,
    KernelConfigurationMismatch,
}

impl std::fmt::Display for TargetNeutralSealedLogProtocolFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::SchemaVersionMismatch => {
                "the sealed-log protocol schema version is not the frozen version"
            }
            Self::ResourcePolicyVersionMismatch => {
                "the sealed-log resource-policy version is not the frozen version"
            }
            Self::RuleVocabularyMismatch => {
                "the sealed-log lifecycle rules are not the exact closed vocabulary"
            }
            Self::CanonicalRulesTranscriptMismatch => {
                "the typed lifecycle rules do not encode to the frozen canonical transcript"
            }
            Self::CanonicalRulesDigestMismatch => {
                "the canonical lifecycle-rule transcript has the wrong domain-separated digest"
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

impl std::error::Error for TargetNeutralSealedLogProtocolFailureV1 {}

/// Opaque proof of the exact JG2b1b0 protocol grammar and verifier profile.
///
/// This capability describes a producer contract only. It contains no
/// events and grants no log, EOF, terminality, finalization, or history fact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedTargetNeutralSealedLogProtocolV1 {
    schema_version: u16,
    resource_policy_version: u16,
    rules_digest: Digest,
    kernel_protocol_digest: Digest,
    normalizer_protocol_digest: Digest,
    kernel_configuration_digest: Digest,
    manifest_digest: Digest,
}

impl VerifiedTargetNeutralSealedLogProtocolV1 {
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub const fn resource_policy_version(&self) -> u16 {
        self.resource_policy_version
    }

    pub fn rules_digest(&self) -> &Digest {
        &self.rules_digest
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

pub fn verify_target_neutral_sealed_log_protocol_v1(
    kernel: &Kernel,
    manifest: &TargetNeutralSealedLogProtocolManifestV1,
) -> Result<VerifiedTargetNeutralSealedLogProtocolV1, TargetNeutralSealedLogProtocolFailureV1> {
    if manifest.rules.schema_version != TARGET_NEUTRAL_SEALED_LOG_PROTOCOL_SCHEMA_VERSION_V1 {
        return Err(TargetNeutralSealedLogProtocolFailureV1::SchemaVersionMismatch);
    }
    if manifest.rules.resource_policy_version
        != TARGET_NEUTRAL_SEALED_LOG_RESOURCE_POLICY_VERSION_V1
    {
        return Err(TargetNeutralSealedLogProtocolFailureV1::ResourcePolicyVersionMismatch);
    }
    if manifest.rules != proposed_target_neutral_sealed_log_rules_v1() {
        return Err(TargetNeutralSealedLogProtocolFailureV1::RuleVocabularyMismatch);
    }

    let mut rules_encoder = CanonicalEncoder::new();
    manifest.rules.encode_canonical(&mut rules_encoder);
    if rules_encoder.as_bytes() != CANONICAL_TARGET_NEUTRAL_SEALED_LOG_RULES_BYTES_V1 {
        return Err(TargetNeutralSealedLogProtocolFailureV1::CanonicalRulesTranscriptMismatch);
    }
    let rules_digest = Digest::of_domain_bytes(
        TARGET_NEUTRAL_SEALED_LOG_RULES_DIGEST_DOMAIN_V1,
        rules_encoder.as_bytes(),
    );
    if rules_digest.as_str() != CANONICAL_TARGET_NEUTRAL_SEALED_LOG_RULES_DIGEST_V1 {
        return Err(TargetNeutralSealedLogProtocolFailureV1::CanonicalRulesDigestMismatch);
    }

    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    if manifest.kernel_protocol_digest != kernel_protocol_digest {
        return Err(TargetNeutralSealedLogProtocolFailureV1::KernelProtocolMismatch);
    }
    let normalizer_protocol_digest = kernel.normalizer_protocol_digest();
    if manifest.normalizer_protocol_digest != normalizer_protocol_digest {
        return Err(TargetNeutralSealedLogProtocolFailureV1::NormalizerProtocolMismatch);
    }
    let kernel_configuration_digest =
        target_neutral_sealed_log_kernel_configuration_digest_v1(kernel);
    if manifest.kernel_configuration_digest != kernel_configuration_digest {
        return Err(TargetNeutralSealedLogProtocolFailureV1::KernelConfigurationMismatch);
    }

    let manifest_digest = Digest::of_canonical(
        TARGET_NEUTRAL_SEALED_LOG_PROTOCOL_DIGEST_DOMAIN_V1,
        manifest,
    );
    Ok(VerifiedTargetNeutralSealedLogProtocolV1 {
        schema_version: manifest.rules.schema_version,
        resource_policy_version: manifest.rules.resource_policy_version,
        rules_digest,
        kernel_protocol_digest,
        normalizer_protocol_digest,
        kernel_configuration_digest,
        manifest_digest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_kernel::KernelLimits;
    use std::collections::BTreeSet;

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("valid kernel")
    }

    #[test]
    fn canonical_rules_transcript_and_digest_are_frozen() {
        let rules = proposed_target_neutral_sealed_log_rules_v1();
        let mut encoder = CanonicalEncoder::new();
        rules.encode_canonical(&mut encoder);
        assert_eq!(
            encoder.as_bytes(),
            CANONICAL_TARGET_NEUTRAL_SEALED_LOG_RULES_BYTES_V1
        );
        assert_eq!(
            Digest::of_domain_bytes(
                TARGET_NEUTRAL_SEALED_LOG_RULES_DIGEST_DOMAIN_V1,
                encoder.as_bytes(),
            )
            .as_str(),
            CANONICAL_TARGET_NEUTRAL_SEALED_LOG_RULES_DIGEST_V1
        );
    }

    #[test]
    fn lifecycle_rule_tags_are_pairwise_distinct() {
        fn bytes(rule: &impl CanonicalEncode) -> Vec<u8> {
            let mut encoder = CanonicalEncoder::new();
            rule.encode_canonical(&mut encoder);
            encoder.as_bytes().to_vec()
        }

        let tags = [
            bytes(
                &TargetNeutralSealedLogGenesisRuleV1::RequireKernelVerifiedEmptyBoundary,
            ),
            bytes(
                &TargetNeutralSealedLogDelimiterRuleV1::OneSuccessfulAppendCommitPerEvent,
            ),
            bytes(&TargetNeutralSealedLogOrderingRuleV1::StrictSingleParentHashChain),
            bytes(&TargetNeutralSealedLogIngressRuleV1::PrivateLinearWriterIsSoleIngress),
            bytes(
                &TargetNeutralSealedLogBranchIdentityRuleV1::DistinctProducerMintedWriterEpochIdentity,
            ),
            bytes(
                &TargetNeutralSealedLogPromotionRuleV1::NoRetrospectiveTranscriptPromotion,
            ),
            bytes(
                &TargetNeutralSealedLogResourceRuleV1::KernelProfileBoundsWithFallibleOutputReservationAndNoFailureFinalization,
            ),
            bytes(
                &TargetNeutralSealedLogFinalizationRuleV1::ConsumingBranchFinalization,
            ),
            bytes(
                &TargetNeutralSealedLogHeadAuthorityRuleV1::ProducerMintedOnlyFromConsumedOwnedWriter,
            ),
            bytes(
                &TargetNeutralSealedLogTerminalRuleV1::RequireCompleteThroughFinalizedHeadByIndependentReplay,
            ),
        ];

        assert!(tags.iter().all(|tag| tag.len() == 1));
        assert_eq!(
            tags.into_iter().collect::<BTreeSet<_>>().len(),
            TARGET_NEUTRAL_SEALED_LOG_LIFECYCLE_RULE_COUNT_V1
        );
    }

    #[test]
    fn exact_protocol_verifies_and_remints_deterministically() {
        let kernel = kernel();
        let proposal = proposed_target_neutral_sealed_log_protocol_v1(&kernel);
        let first = verify_target_neutral_sealed_log_protocol_v1(&kernel, &proposal)
            .expect("exact protocol");
        let second = verify_target_neutral_sealed_log_protocol_v1(&kernel, &proposal)
            .expect("deterministic remint");

        assert_eq!(first, second);
        assert_eq!(first.schema_version(), 1);
        assert_eq!(first.resource_policy_version(), 1);
        assert_eq!(
            first.rules_digest().as_str(),
            CANONICAL_TARGET_NEUTRAL_SEALED_LOG_RULES_DIGEST_V1
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
            &target_neutral_sealed_log_kernel_configuration_digest_v1(&kernel)
        );
        assert_eq!(first.manifest_digest(), second.manifest_digest());
    }

    #[test]
    fn schema_and_resource_policy_mutations_fail_closed() {
        let kernel = kernel();
        let proposal = proposed_target_neutral_sealed_log_protocol_v1(&kernel);

        let mut mutation = proposal.clone();
        mutation.rules.schema_version += 1;
        assert_eq!(
            verify_target_neutral_sealed_log_protocol_v1(&kernel, &mutation),
            Err(TargetNeutralSealedLogProtocolFailureV1::SchemaVersionMismatch)
        );

        let mut mutation = proposal;
        mutation.rules.resource_policy_version += 1;
        assert_eq!(
            verify_target_neutral_sealed_log_protocol_v1(&kernel, &mutation),
            Err(TargetNeutralSealedLogProtocolFailureV1::ResourcePolicyVersionMismatch)
        );
    }

    #[test]
    fn kernel_normalizer_and_configuration_mutations_fail_closed() {
        let kernel = kernel();
        let proposal = proposed_target_neutral_sealed_log_protocol_v1(&kernel);

        let mut mutation = proposal.clone();
        mutation.kernel_protocol_digest = Digest::of_bytes(b"wrong-kernel");
        assert_eq!(
            verify_target_neutral_sealed_log_protocol_v1(&kernel, &mutation),
            Err(TargetNeutralSealedLogProtocolFailureV1::KernelProtocolMismatch)
        );

        let mut mutation = proposal.clone();
        mutation.normalizer_protocol_digest = Digest::of_bytes(b"wrong-normalizer");
        assert_eq!(
            verify_target_neutral_sealed_log_protocol_v1(&kernel, &mutation),
            Err(TargetNeutralSealedLogProtocolFailureV1::NormalizerProtocolMismatch)
        );

        let mut mutation = proposal;
        mutation.kernel_configuration_digest = Digest::of_bytes(b"wrong-configuration");
        assert_eq!(
            verify_target_neutral_sealed_log_protocol_v1(&kernel, &mutation),
            Err(TargetNeutralSealedLogProtocolFailureV1::KernelConfigurationMismatch)
        );
    }

    #[test]
    fn protocol_is_bound_to_the_exact_kernel_resource_profile() {
        let source = kernel();
        let proposal = proposed_target_neutral_sealed_log_protocol_v1(&source);
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
            assert_ne!(
                target_neutral_sealed_log_kernel_configuration_digest_v1(&source),
                target_neutral_sealed_log_kernel_configuration_digest_v1(&drifted)
            );
            assert_eq!(
                verify_target_neutral_sealed_log_protocol_v1(&drifted, &proposal),
                Err(TargetNeutralSealedLogProtocolFailureV1::KernelConfigurationMismatch)
            );

            let drifted_proposal = proposed_target_neutral_sealed_log_protocol_v1(&drifted);
            verify_target_neutral_sealed_log_protocol_v1(&drifted, &drifted_proposal)
                .expect("same rules under an explicitly distinct resource profile");
        }
    }
}
