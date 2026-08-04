//! JG2b1a authority for a normalized contiguous chain of JG2a stages.
//!
//! The authority in this module is deliberately relative to the complete
//! slice supplied to [`derive_contiguous_generative_stage_chain_v1`]. It does
//! not prove that the slice is the actual complete history, that its final
//! event is an authoritative EOF, or that its segmentation is unique.

use crate::{
    VerifiedGenerativeCapabilityConstructorGrammarV1, VerifiedGenerativeCapabilityStageSurfaceV1,
    VerifiedPreExposureGenerativeCapabilityGrammarV1,
    generative_capability_kernel_configuration_digest_v1,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Digest, GlobalId, Kernel, KernelError, UncheckedSignature,
    VerifiedSignature,
};

pub const CONTIGUOUS_GENERATIVE_STAGE_CHAIN_SCHEMA_VERSION_V1: u16 = 1;
/// V1 deliberately makes chain authority relative to the bound kernel
/// resource profile: stage count is capped by `max_depth`, terminal/retained
/// declarations by `max_operations`, and aggregate inspected material by
/// `max_operations * (2 * max_depth + 1)` using checked arithmetic.
pub const CONTIGUOUS_GENERATIVE_STAGE_CHAIN_RESOURCE_POLICY_VERSION_V1: u16 = 1;

const NORMALIZED_TRANSITION_ID_DOMAIN_V1: &str =
    "law-v2/jg2b1a/normalized-generative-transition-id/v1";
const NORMALIZED_HISTORY_GENESIS_DOMAIN_V1: &str = "law-v2/jg2b1a/normalized-history-genesis/v1";
const GENERATIVE_HISTORY_EVENT_ID_DOMAIN_V1: &str = "law-v2/jg2b1a/generative-history-event-id/v1";
const NORMALIZED_HISTORY_STEP_DOMAIN_V1: &str = "law-v2/jg2b1a/normalized-history-step/v1";
const NORMALIZED_CHAIN_ID_DOMAIN_V1: &str = "law-v2/jg2b1a/normalized-contiguous-stage-chain/v1";
const EXACT_CHAIN_EVIDENCE_DOMAIN_V1: &str = "law-v2/jg2b1a/exact-contiguous-stage-evidence/v1";

const NORMALIZED_TRANSITION_ROOT_TAG_V1: u8 = 0xb1;
const NORMALIZED_HISTORY_GENESIS_ROOT_TAG_V1: u8 = 0xb2;
const GENERATIVE_HISTORY_EVENT_ROOT_TAG_V1: u8 = 0xb3;
const NORMALIZED_HISTORY_STEP_ROOT_TAG_V1: u8 = 0xb4;
const DECLARATION_BIRTH_ROOT_TAG_V1: u8 = 0xb5;
const NORMALIZED_CHAIN_ROOT_TAG_V1: u8 = 0xb6;
const EXACT_CHAIN_EVIDENCE_ROOT_TAG_V1: u8 = 0xb7;

/// Canonical identity of one normalized predecessor/extension/successor
/// transition. Exact candidate syntax and arbitrary trusted-scope fields are
/// intentionally outside this identity.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NormalizedGenerativeTransitionIdV1(Digest);

impl NormalizedGenerativeTransitionIdV1 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for NormalizedGenerativeTransitionIdV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// History-relative identity of one event. This is distinct from a normalized
/// transition identity because it also binds the derived prefix and ordinal.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct GenerativeHistoryEventIdV1(Digest);

impl GenerativeHistoryEventIdV1 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for GenerativeHistoryEventIdV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// One declaration's unique birth within the supplied normalized chain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedGenerativeDeclarationBirthV1 {
    global_id: GlobalId,
    event_id: GenerativeHistoryEventIdV1,
    event_ordinal: u64,
    extension_offset: u64,
    terminal_declaration_ordinal: u64,
}

impl VerifiedGenerativeDeclarationBirthV1 {
    pub fn global_id(&self) -> &GlobalId {
        &self.global_id
    }

    pub fn event_id(&self) -> &GenerativeHistoryEventIdV1 {
        &self.event_id
    }

    pub const fn event_ordinal(&self) -> u64 {
        self.event_ordinal
    }

    pub const fn extension_offset(&self) -> u64 {
        self.extension_offset
    }

    pub const fn terminal_declaration_ordinal(&self) -> u64 {
        self.terminal_declaration_ordinal
    }
}

impl CanonicalEncode for VerifiedGenerativeDeclarationBirthV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(DECLARATION_BIRTH_ROOT_TAG_V1);
        self.global_id.encode_canonical(encoder);
        self.event_id.encode_canonical(encoder);
        encoder.u64(self.event_ordinal);
        encoder.u64(self.extension_offset);
        encoder.u64(self.terminal_declaration_ordinal);
    }
}

/// One replayed event in a verified relative chain.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedContiguousGenerativeStageEventV1 {
    event_id: GenerativeHistoryEventIdV1,
    transition_id: NormalizedGenerativeTransitionIdV1,
    event_ordinal: u64,
    pre_history_digest: Digest,
    post_history_digest: Digest,
    predecessor_digest: Digest,
    successor_digest: Digest,
    predecessor_declaration_count: u64,
    extension_declaration_count: u64,
    successor_declaration_count: u64,
    normalized_extension: UncheckedSignature,
    exact_stage_manifest_digest: Digest,
}

impl VerifiedContiguousGenerativeStageEventV1 {
    pub fn event_id(&self) -> &GenerativeHistoryEventIdV1 {
        &self.event_id
    }

    pub fn transition_id(&self) -> &NormalizedGenerativeTransitionIdV1 {
        &self.transition_id
    }

    pub const fn event_ordinal(&self) -> u64 {
        self.event_ordinal
    }

    pub fn pre_history_digest(&self) -> &Digest {
        &self.pre_history_digest
    }

    pub fn post_history_digest(&self) -> &Digest {
        &self.post_history_digest
    }

    pub fn predecessor_digest(&self) -> &Digest {
        &self.predecessor_digest
    }

    pub fn successor_digest(&self) -> &Digest {
        &self.successor_digest
    }

    pub const fn predecessor_declaration_count(&self) -> u64 {
        self.predecessor_declaration_count
    }

    pub const fn extension_declaration_count(&self) -> u64 {
        self.extension_declaration_count
    }

    pub const fn successor_declaration_count(&self) -> u64 {
        self.successor_declaration_count
    }

    pub fn normalized_extension(&self) -> &UncheckedSignature {
        &self.normalized_extension
    }

    /// Exact JG2a evidence identity, retained separately from the normalized
    /// transition and event identities.
    pub fn exact_stage_manifest_digest(&self) -> &Digest {
        &self.exact_stage_manifest_digest
    }
}

/// Fail-closed reasons why a relative contiguous-chain token was not minted
/// or could not be replayed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContiguousGenerativeStageChainFailureV1 {
    ConstructorGrammarJg1Mismatch,
    StageCountLimitExceeded,
    AggregateDeclarationMaterialOverflow,
    AggregateDeclarationMaterialLimitExceeded,
    OutputAllocationFailure,
    InitialBoundaryKernelFailure(KernelError),
    StageJg1Mismatch {
        stage_index: usize,
    },
    StageConstructorGrammarMismatch {
        stage_index: usize,
    },
    StageScopeGrammarMismatch {
        stage_index: usize,
    },
    StageKernelProtocolMismatch {
        stage_index: usize,
    },
    StageNormalizerProtocolMismatch {
        stage_index: usize,
    },
    StageKernelConfigurationMismatch {
        stage_index: usize,
    },
    NonAdjacentStage {
        stage_index: usize,
    },
    StageReplayKernelFailure {
        stage_index: usize,
        error: KernelError,
    },
    StageReplaySuccessorMismatch {
        stage_index: usize,
    },
    WireCountOverflow,
    DuplicateDeclarationBirth,
    TerminalBirthCoverageMismatch,
    ReplayEventCountOutOfRange,
    ReplayKernelProtocolMismatch,
    ReplayNormalizerProtocolMismatch,
    ReplayKernelConfigurationMismatch,
    ReplayKernelFailure {
        event_index: usize,
        error: KernelError,
    },
    ReplayIdentityMismatch {
        event_index: usize,
    },
    ReplayTerminalBoundaryMismatch,
}

impl std::fmt::Display for ContiguousGenerativeStageChainFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConstructorGrammarJg1Mismatch => formatter.write_str(
                "the verified constructor grammar is not bound to the supplied JG1 grammar",
            ),
            Self::StageCountLimitExceeded => {
                formatter.write_str("the stage count exceeds the kernel depth resource bound")
            }
            Self::AggregateDeclarationMaterialOverflow => {
                formatter.write_str("checked aggregate declaration-material arithmetic overflowed")
            }
            Self::AggregateDeclarationMaterialLimitExceeded => formatter.write_str(
                "the chain exceeds its checked aggregate declaration-material resource bound",
            ),
            Self::OutputAllocationFailure => {
                formatter.write_str("bounded chain-output allocation failed")
            }
            Self::InitialBoundaryKernelFailure(error) => {
                write!(
                    formatter,
                    "the canonical empty boundary failed kernel replay: {error}"
                )
            }
            Self::StageJg1Mismatch { stage_index } => {
                write!(
                    formatter,
                    "stage {stage_index} is bound to a different JG1 grammar"
                )
            }
            Self::StageConstructorGrammarMismatch { stage_index } => write!(
                formatter,
                "stage {stage_index} is bound to a different constructor grammar"
            ),
            Self::StageScopeGrammarMismatch { stage_index } => write!(
                formatter,
                "stage {stage_index} is bound to a different combined scope grammar"
            ),
            Self::StageKernelProtocolMismatch { stage_index } => write!(
                formatter,
                "stage {stage_index} is bound to a different kernel protocol"
            ),
            Self::StageNormalizerProtocolMismatch { stage_index } => write!(
                formatter,
                "stage {stage_index} is bound to a different normalizer protocol"
            ),
            Self::StageKernelConfigurationMismatch { stage_index } => write!(
                formatter,
                "stage {stage_index} is bound to a different kernel configuration"
            ),
            Self::NonAdjacentStage { stage_index } => write!(
                formatter,
                "stage {stage_index} does not begin at the replayed preceding boundary"
            ),
            Self::StageReplayKernelFailure { stage_index, error } => write!(
                formatter,
                "stage {stage_index} failed normalized extension replay: {error}"
            ),
            Self::StageReplaySuccessorMismatch { stage_index } => write!(
                formatter,
                "stage {stage_index} does not end at its independently replayed successor"
            ),
            Self::WireCountOverflow => formatter
                .write_str("a derived chain count cannot be represented by the V1 wire format"),
            Self::DuplicateDeclarationBirth => {
                formatter.write_str("one global identifier has more than one declaration birth")
            }
            Self::TerminalBirthCoverageMismatch => formatter.write_str(
                "declaration-birth keys do not exactly equal terminal-boundary identifiers",
            ),
            Self::ReplayEventCountOutOfRange => {
                formatter.write_str("the requested replay prefix exceeds the relative chain")
            }
            Self::ReplayKernelProtocolMismatch => {
                formatter.write_str("the replay kernel has a different kernel protocol identity")
            }
            Self::ReplayNormalizerProtocolMismatch => formatter
                .write_str("the replay kernel has a different normalizer protocol identity"),
            Self::ReplayKernelConfigurationMismatch => {
                formatter.write_str("the replay kernel has a different resource configuration")
            }
            Self::ReplayKernelFailure { event_index, error } => write!(
                formatter,
                "relative event {event_index} failed independent replay: {error}"
            ),
            Self::ReplayIdentityMismatch { event_index } => write!(
                formatter,
                "relative event {event_index} does not reproduce its derived identities"
            ),
            Self::ReplayTerminalBoundaryMismatch => formatter.write_str(
                "complete relative-chain replay differs from the stored terminal boundary",
            ),
        }
    }
}

impl std::error::Error for ContiguousGenerativeStageChainFailureV1 {}

/// Opaque proof of exact adjacency and replay for every stage in the supplied
/// slice, plus an exhaustive declaration-birth index relative to that slice.
///
/// This type carries no complete-history, EOF, unique-segmentation,
/// substitution, naturality, closure, support, carrier, or gain authority.
#[derive(Clone, Debug)]
pub struct VerifiedContiguousGenerativeStageChainV1 {
    schema_version: u16,
    resource_policy_version: u16,
    jg1_manifest_digest: Digest,
    constructor_grammar_manifest_digest: Digest,
    scope_grammar_digest: Digest,
    kernel_protocol_digest: Digest,
    normalizer_protocol_digest: Digest,
    kernel_configuration_digest: Digest,
    initial_boundary: VerifiedSignature,
    terminal_boundary: VerifiedSignature,
    events: Vec<VerifiedContiguousGenerativeStageEventV1>,
    declaration_births: Vec<VerifiedGenerativeDeclarationBirthV1>,
    normalized_history_digest: Digest,
    normalized_chain_digest: Digest,
    exact_evidence_binding_digest: Digest,
}

impl PartialEq for VerifiedContiguousGenerativeStageChainV1 {
    /// Exact token equality includes the exact-evidence binding. Use
    /// [`Self::same_normalized_chain_as`] for scope/raw-syntax-neutral
    /// normalized identity.
    fn eq(&self, other: &Self) -> bool {
        self.schema_version == other.schema_version
            && self.resource_policy_version == other.resource_policy_version
            && self.jg1_manifest_digest == other.jg1_manifest_digest
            && self.constructor_grammar_manifest_digest == other.constructor_grammar_manifest_digest
            && self.scope_grammar_digest == other.scope_grammar_digest
            && self.kernel_protocol_digest == other.kernel_protocol_digest
            && self.normalizer_protocol_digest == other.normalizer_protocol_digest
            && self.kernel_configuration_digest == other.kernel_configuration_digest
            && same_boundary(&self.initial_boundary, &other.initial_boundary)
            && same_boundary(&self.terminal_boundary, &other.terminal_boundary)
            && self.events == other.events
            && self.declaration_births == other.declaration_births
            && self.normalized_history_digest == other.normalized_history_digest
            && self.normalized_chain_digest == other.normalized_chain_digest
            && self.exact_evidence_binding_digest == other.exact_evidence_binding_digest
    }
}

impl Eq for VerifiedContiguousGenerativeStageChainV1 {}

impl VerifiedContiguousGenerativeStageChainV1 {
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    /// Identifies the frozen, kernel-resource-profile-relative V1 policy.
    pub const fn resource_policy_version(&self) -> u16 {
        self.resource_policy_version
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

    pub fn initial_boundary(&self) -> &VerifiedSignature {
        &self.initial_boundary
    }

    pub fn terminal_boundary(&self) -> &VerifiedSignature {
        &self.terminal_boundary
    }

    pub fn events(&self) -> &[VerifiedContiguousGenerativeStageEventV1] {
        &self.events
    }

    pub fn declaration_births(&self) -> &[VerifiedGenerativeDeclarationBirthV1] {
        &self.declaration_births
    }

    pub fn declaration_birth(
        &self,
        global_id: &GlobalId,
    ) -> Option<&VerifiedGenerativeDeclarationBirthV1> {
        self.declaration_births
            .binary_search_by(|birth| birth.global_id().cmp(global_id))
            .ok()
            .map(|index| &self.declaration_births[index])
    }

    pub fn normalized_history_digest(&self) -> &Digest {
        &self.normalized_history_digest
    }

    pub fn normalized_chain_digest(&self) -> &Digest {
        &self.normalized_chain_digest
    }

    /// Compare only the target-neutral normalized chain identity, excluding
    /// exact stage manifests and their arbitrary trusted-scope/raw syntax.
    pub fn same_normalized_chain_as(&self, other: &Self) -> bool {
        self.normalized_chain_digest == other.normalized_chain_digest
    }

    /// Binds the ordered exact JG2a stage manifests separately from the
    /// target-neutral normalized chain identity.
    pub fn exact_evidence_binding_digest(&self) -> &Digest {
        &self.exact_evidence_binding_digest
    }

    /// Reconstruct a prefix solely from the internally derived empty boundary
    /// and retained normalized extensions.
    pub fn replay_boundary_after(
        &self,
        kernel: &Kernel,
        event_count: usize,
    ) -> Result<VerifiedSignature, ContiguousGenerativeStageChainFailureV1> {
        if event_count > self.events.len() {
            return Err(ContiguousGenerativeStageChainFailureV1::ReplayEventCountOutOfRange);
        }
        if kernel.kernel_protocol_digest() != self.kernel_protocol_digest {
            return Err(ContiguousGenerativeStageChainFailureV1::ReplayKernelProtocolMismatch);
        }
        if kernel.normalizer_protocol_digest() != self.normalizer_protocol_digest {
            return Err(ContiguousGenerativeStageChainFailureV1::ReplayNormalizerProtocolMismatch);
        }
        if generative_capability_kernel_configuration_digest_v1(kernel)
            != self.kernel_configuration_digest
        {
            return Err(ContiguousGenerativeStageChainFailureV1::ReplayKernelConfigurationMismatch);
        }

        let mut current = kernel
            .verify_signature(&UncheckedSignature::default())
            .map_err(ContiguousGenerativeStageChainFailureV1::InitialBoundaryKernelFailure)?;
        let mut history_digest = normalized_history_genesis_digest_v1(
            &self.jg1_manifest_digest,
            &self.constructor_grammar_manifest_digest,
            &self.scope_grammar_digest,
            &self.kernel_protocol_digest,
            &self.normalizer_protocol_digest,
            &self.kernel_configuration_digest,
            &current,
        );

        for (event_index, event) in self.events.iter().take(event_count).enumerate() {
            let replayed = kernel
                .verify_extension(&current, event.normalized_extension())
                .map_err(
                    |error| ContiguousGenerativeStageChainFailureV1::ReplayKernelFailure {
                        event_index,
                        error,
                    },
                )?;
            let transition_id = normalized_transition_id_v1(
                &self.jg1_manifest_digest,
                &self.constructor_grammar_manifest_digest,
                &self.scope_grammar_digest,
                &self.kernel_protocol_digest,
                &self.normalizer_protocol_digest,
                &self.kernel_configuration_digest,
                &current,
                event.normalized_extension(),
                &replayed,
            )?;
            let ordinal = u64::try_from(event_index)
                .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?;
            let event_id = generative_history_event_id_v1(&history_digest, ordinal, &transition_id);
            let post_history_digest = normalized_history_step_digest_v1(&history_digest, &event_id);

            if event.event_ordinal != ordinal
                || event.pre_history_digest != history_digest
                || event.transition_id != transition_id
                || event.event_id != event_id
                || event.post_history_digest != post_history_digest
                || event.predecessor_digest != *current.digest()
                || event.successor_digest != *replayed.digest()
                || event.predecessor_declaration_count
                    != u64::try_from(current.declarations().len())
                        .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?
                || event.extension_declaration_count
                    != u64::try_from(event.normalized_extension.declarations.len())
                        .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?
                || event.successor_declaration_count
                    != u64::try_from(replayed.declarations().len())
                        .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?
            {
                return Err(
                    ContiguousGenerativeStageChainFailureV1::ReplayIdentityMismatch { event_index },
                );
            }

            current = replayed;
            history_digest = post_history_digest;
        }

        if event_count == self.events.len()
            && (!same_boundary(&current, &self.terminal_boundary)
                || history_digest != self.normalized_history_digest)
        {
            return Err(ContiguousGenerativeStageChainFailureV1::ReplayTerminalBoundaryMismatch);
        }

        Ok(current)
    }
}

#[allow(clippy::too_many_arguments)]
fn normalized_transition_id_v1(
    jg1_manifest_digest: &Digest,
    constructor_grammar_manifest_digest: &Digest,
    scope_grammar_digest: &Digest,
    kernel_protocol_digest: &Digest,
    normalizer_protocol_digest: &Digest,
    kernel_configuration_digest: &Digest,
    predecessor: &VerifiedSignature,
    normalized_extension: &UncheckedSignature,
    successor: &VerifiedSignature,
) -> Result<NormalizedGenerativeTransitionIdV1, ContiguousGenerativeStageChainFailureV1> {
    let predecessor_count = u64::try_from(predecessor.declarations().len())
        .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?;
    let extension_count = u64::try_from(normalized_extension.declarations.len())
        .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?;
    let successor_count = u64::try_from(successor.declarations().len())
        .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?;
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(NORMALIZED_TRANSITION_ROOT_TAG_V1);
    encoder.u16(CONTIGUOUS_GENERATIVE_STAGE_CHAIN_SCHEMA_VERSION_V1);
    encoder.u16(CONTIGUOUS_GENERATIVE_STAGE_CHAIN_RESOURCE_POLICY_VERSION_V1);
    jg1_manifest_digest.encode_canonical(&mut encoder);
    constructor_grammar_manifest_digest.encode_canonical(&mut encoder);
    scope_grammar_digest.encode_canonical(&mut encoder);
    kernel_protocol_digest.encode_canonical(&mut encoder);
    normalizer_protocol_digest.encode_canonical(&mut encoder);
    kernel_configuration_digest.encode_canonical(&mut encoder);
    encoder.u64(predecessor_count);
    encoder.sequence(predecessor.declarations());
    encoder.u64(extension_count);
    normalized_extension.encode_canonical(&mut encoder);
    encoder.u64(successor_count);
    encoder.sequence(successor.declarations());
    Ok(NormalizedGenerativeTransitionIdV1(Digest::of_domain_bytes(
        NORMALIZED_TRANSITION_ID_DOMAIN_V1,
        encoder.as_bytes(),
    )))
}

#[allow(clippy::too_many_arguments)]
fn normalized_history_genesis_digest_v1(
    jg1_manifest_digest: &Digest,
    constructor_grammar_manifest_digest: &Digest,
    scope_grammar_digest: &Digest,
    kernel_protocol_digest: &Digest,
    normalizer_protocol_digest: &Digest,
    kernel_configuration_digest: &Digest,
    empty_boundary: &VerifiedSignature,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(NORMALIZED_HISTORY_GENESIS_ROOT_TAG_V1);
    encoder.u16(CONTIGUOUS_GENERATIVE_STAGE_CHAIN_SCHEMA_VERSION_V1);
    encoder.u16(CONTIGUOUS_GENERATIVE_STAGE_CHAIN_RESOURCE_POLICY_VERSION_V1);
    jg1_manifest_digest.encode_canonical(&mut encoder);
    constructor_grammar_manifest_digest.encode_canonical(&mut encoder);
    scope_grammar_digest.encode_canonical(&mut encoder);
    kernel_protocol_digest.encode_canonical(&mut encoder);
    normalizer_protocol_digest.encode_canonical(&mut encoder);
    kernel_configuration_digest.encode_canonical(&mut encoder);
    encoder.sequence(empty_boundary.declarations());
    Digest::of_domain_bytes(NORMALIZED_HISTORY_GENESIS_DOMAIN_V1, encoder.as_bytes())
}

fn generative_history_event_id_v1(
    pre_history_digest: &Digest,
    event_ordinal: u64,
    transition_id: &NormalizedGenerativeTransitionIdV1,
) -> GenerativeHistoryEventIdV1 {
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(GENERATIVE_HISTORY_EVENT_ROOT_TAG_V1);
    pre_history_digest.encode_canonical(&mut encoder);
    encoder.u64(event_ordinal);
    transition_id.encode_canonical(&mut encoder);
    GenerativeHistoryEventIdV1(Digest::of_domain_bytes(
        GENERATIVE_HISTORY_EVENT_ID_DOMAIN_V1,
        encoder.as_bytes(),
    ))
}

fn normalized_history_step_digest_v1(
    pre_history_digest: &Digest,
    event_id: &GenerativeHistoryEventIdV1,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(NORMALIZED_HISTORY_STEP_ROOT_TAG_V1);
    pre_history_digest.encode_canonical(&mut encoder);
    event_id.encode_canonical(&mut encoder);
    Digest::of_domain_bytes(NORMALIZED_HISTORY_STEP_DOMAIN_V1, encoder.as_bytes())
}

fn same_boundary(left: &VerifiedSignature, right: &VerifiedSignature) -> bool {
    left.digest() == right.digest() && left.declarations() == right.declarations()
}

/// Derive a normalized contiguous stage chain from canonical genesis.
///
/// Every input stage is grammar/configuration checked, required to be exactly
/// adjacent to the preceding replay result, and independently replayed. Event
/// ordinals, transition IDs, event IDs, history digests, and declaration
/// births are derived internally. The resulting authority is exhaustive only
/// relative to `stages`.
pub fn derive_contiguous_generative_stage_chain_v1(
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructor_grammar: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
    stages: &[VerifiedGenerativeCapabilityStageSurfaceV1],
) -> Result<VerifiedContiguousGenerativeStageChainV1, ContiguousGenerativeStageChainFailureV1> {
    if constructor_grammar.jg1_manifest_digest() != jg1.manifest_digest() {
        return Err(ContiguousGenerativeStageChainFailureV1::ConstructorGrammarJg1Mismatch);
    }

    let limits = kernel.limits();
    let max_stage_count = usize::from(limits.max_depth);
    if stages.len() > max_stage_count {
        return Err(ContiguousGenerativeStageChainFailureV1::StageCountLimitExceeded);
    }
    let max_declarations = usize::try_from(limits.max_operations).map_err(|_| {
        ContiguousGenerativeStageChainFailureV1::AggregateDeclarationMaterialOverflow
    })?;
    let aggregate_factor = max_stage_count
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or(ContiguousGenerativeStageChainFailureV1::AggregateDeclarationMaterialOverflow)?;
    let aggregate_limit = max_declarations
        .checked_mul(aggregate_factor)
        .ok_or(ContiguousGenerativeStageChainFailureV1::AggregateDeclarationMaterialOverflow)?;
    let mut aggregate_material = 0usize;
    let mut retained_extension_count = 0usize;
    for stage in stages {
        let predecessor_count = stage.predecessor_boundary().declarations().len();
        let extension_count = stage.normalized_candidate().declarations.len();
        let successor_count = stage.successor_boundary().declarations().len();
        if predecessor_count > max_declarations
            || extension_count > max_declarations
            || successor_count > max_declarations
        {
            return Err(
                ContiguousGenerativeStageChainFailureV1::AggregateDeclarationMaterialLimitExceeded,
            );
        }
        aggregate_material = aggregate_material
            .checked_add(predecessor_count)
            .and_then(|value| value.checked_add(extension_count))
            .and_then(|value| value.checked_add(successor_count))
            .ok_or(ContiguousGenerativeStageChainFailureV1::AggregateDeclarationMaterialOverflow)?;
        retained_extension_count = retained_extension_count
            .checked_add(extension_count)
            .ok_or(ContiguousGenerativeStageChainFailureV1::AggregateDeclarationMaterialOverflow)?;
    }
    if aggregate_material > aggregate_limit || retained_extension_count > max_declarations {
        return Err(
            ContiguousGenerativeStageChainFailureV1::AggregateDeclarationMaterialLimitExceeded,
        );
    }

    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    let normalizer_protocol_digest = kernel.normalizer_protocol_digest();
    let kernel_configuration_digest = generative_capability_kernel_configuration_digest_v1(kernel);
    let initial_boundary = kernel
        .verify_signature(&UncheckedSignature::default())
        .map_err(ContiguousGenerativeStageChainFailureV1::InitialBoundaryKernelFailure)?;
    let mut current = initial_boundary.clone();
    let mut history_digest = normalized_history_genesis_digest_v1(
        jg1.manifest_digest(),
        constructor_grammar.manifest_digest(),
        constructor_grammar.scope_grammar_digest(),
        &kernel_protocol_digest,
        &normalizer_protocol_digest,
        &kernel_configuration_digest,
        &initial_boundary,
    );

    let mut events = Vec::new();
    events
        .try_reserve_exact(stages.len())
        .map_err(|_| ContiguousGenerativeStageChainFailureV1::OutputAllocationFailure)?;
    let mut declaration_births = Vec::new();
    declaration_births
        .try_reserve_exact(retained_extension_count)
        .map_err(|_| ContiguousGenerativeStageChainFailureV1::OutputAllocationFailure)?;

    for (stage_index, stage) in stages.iter().enumerate() {
        if stage.jg1_manifest_digest() != jg1.manifest_digest() {
            return Err(ContiguousGenerativeStageChainFailureV1::StageJg1Mismatch { stage_index });
        }
        if stage.constructor_grammar_manifest_digest() != constructor_grammar.manifest_digest() {
            return Err(
                ContiguousGenerativeStageChainFailureV1::StageConstructorGrammarMismatch {
                    stage_index,
                },
            );
        }
        if stage.scope_grammar_digest() != constructor_grammar.scope_grammar_digest() {
            return Err(
                ContiguousGenerativeStageChainFailureV1::StageScopeGrammarMismatch { stage_index },
            );
        }
        if stage.kernel_protocol_digest() != &kernel_protocol_digest {
            return Err(
                ContiguousGenerativeStageChainFailureV1::StageKernelProtocolMismatch {
                    stage_index,
                },
            );
        }
        if stage.normalizer_protocol_digest() != &normalizer_protocol_digest {
            return Err(
                ContiguousGenerativeStageChainFailureV1::StageNormalizerProtocolMismatch {
                    stage_index,
                },
            );
        }
        if stage.kernel_configuration_digest() != &kernel_configuration_digest {
            return Err(
                ContiguousGenerativeStageChainFailureV1::StageKernelConfigurationMismatch {
                    stage_index,
                },
            );
        }
        if !same_boundary(stage.predecessor_boundary(), &current) {
            return Err(ContiguousGenerativeStageChainFailureV1::NonAdjacentStage { stage_index });
        }

        let replayed_successor = kernel
            .verify_extension(&current, stage.normalized_candidate())
            .map_err(
                |error| ContiguousGenerativeStageChainFailureV1::StageReplayKernelFailure {
                    stage_index,
                    error,
                },
            )?;
        if !same_boundary(stage.successor_boundary(), &replayed_successor) {
            return Err(
                ContiguousGenerativeStageChainFailureV1::StageReplaySuccessorMismatch {
                    stage_index,
                },
            );
        }

        let event_ordinal = u64::try_from(stage_index)
            .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?;
        let transition_id = normalized_transition_id_v1(
            jg1.manifest_digest(),
            constructor_grammar.manifest_digest(),
            constructor_grammar.scope_grammar_digest(),
            &kernel_protocol_digest,
            &normalizer_protocol_digest,
            &kernel_configuration_digest,
            &current,
            stage.normalized_candidate(),
            &replayed_successor,
        )?;
        let event_id =
            generative_history_event_id_v1(&history_digest, event_ordinal, &transition_id);
        let post_history_digest = normalized_history_step_digest_v1(&history_digest, &event_id);

        let predecessor_count = current.declarations().len();
        for (extension_offset, declaration) in
            stage.normalized_candidate().declarations.iter().enumerate()
        {
            let terminal_declaration_ordinal = predecessor_count
                .checked_add(extension_offset)
                .ok_or(ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?;
            declaration_births.push(VerifiedGenerativeDeclarationBirthV1 {
                global_id: declaration.id.clone(),
                event_id: event_id.clone(),
                event_ordinal,
                extension_offset: u64::try_from(extension_offset)
                    .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?,
                terminal_declaration_ordinal: u64::try_from(terminal_declaration_ordinal)
                    .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?,
            });
        }

        events.push(VerifiedContiguousGenerativeStageEventV1 {
            event_id,
            transition_id,
            event_ordinal,
            pre_history_digest: history_digest,
            post_history_digest: post_history_digest.clone(),
            predecessor_digest: current.digest().clone(),
            successor_digest: replayed_successor.digest().clone(),
            predecessor_declaration_count: u64::try_from(predecessor_count)
                .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?,
            extension_declaration_count: u64::try_from(
                stage.normalized_candidate().declarations.len(),
            )
            .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?,
            successor_declaration_count: u64::try_from(replayed_successor.declarations().len())
                .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?,
            normalized_extension: stage.normalized_candidate().clone(),
            exact_stage_manifest_digest: stage.manifest_digest().clone(),
        });
        current = replayed_successor;
        history_digest = post_history_digest;
    }

    declaration_births.sort_by(|left, right| left.global_id.cmp(&right.global_id));
    if declaration_births
        .windows(2)
        .any(|window| window[0].global_id == window[1].global_id)
    {
        return Err(ContiguousGenerativeStageChainFailureV1::DuplicateDeclarationBirth);
    }

    let mut terminal_ids = Vec::new();
    terminal_ids
        .try_reserve_exact(current.declarations().len())
        .map_err(|_| ContiguousGenerativeStageChainFailureV1::OutputAllocationFailure)?;
    terminal_ids.extend(
        current
            .declarations()
            .iter()
            .map(|declaration| declaration.id.clone()),
    );
    terminal_ids.sort();
    if terminal_ids.len() != declaration_births.len()
        || terminal_ids
            .iter()
            .zip(&declaration_births)
            .any(|(global_id, birth)| global_id != birth.global_id())
    {
        return Err(ContiguousGenerativeStageChainFailureV1::TerminalBirthCoverageMismatch);
    }

    let event_count = u64::try_from(events.len())
        .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?;
    let birth_count = u64::try_from(declaration_births.len())
        .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?;
    let normalized_chain_digest = normalized_chain_digest_v1(
        jg1.manifest_digest(),
        constructor_grammar.manifest_digest(),
        constructor_grammar.scope_grammar_digest(),
        &kernel_protocol_digest,
        &normalizer_protocol_digest,
        &kernel_configuration_digest,
        event_count,
        &history_digest,
        &current,
        birth_count,
        &declaration_births,
    );
    let exact_evidence_binding_digest =
        exact_chain_evidence_digest_v1(&normalized_chain_digest, stages)?;

    Ok(VerifiedContiguousGenerativeStageChainV1 {
        schema_version: CONTIGUOUS_GENERATIVE_STAGE_CHAIN_SCHEMA_VERSION_V1,
        resource_policy_version: CONTIGUOUS_GENERATIVE_STAGE_CHAIN_RESOURCE_POLICY_VERSION_V1,
        jg1_manifest_digest: jg1.manifest_digest().clone(),
        constructor_grammar_manifest_digest: constructor_grammar.manifest_digest().clone(),
        scope_grammar_digest: constructor_grammar.scope_grammar_digest().clone(),
        kernel_protocol_digest,
        normalizer_protocol_digest,
        kernel_configuration_digest,
        initial_boundary,
        terminal_boundary: current,
        events,
        declaration_births,
        normalized_history_digest: history_digest,
        normalized_chain_digest,
        exact_evidence_binding_digest,
    })
}

#[allow(clippy::too_many_arguments)]
fn normalized_chain_digest_v1(
    jg1_manifest_digest: &Digest,
    constructor_grammar_manifest_digest: &Digest,
    scope_grammar_digest: &Digest,
    kernel_protocol_digest: &Digest,
    normalizer_protocol_digest: &Digest,
    kernel_configuration_digest: &Digest,
    event_count: u64,
    normalized_history_digest: &Digest,
    terminal_boundary: &VerifiedSignature,
    birth_count: u64,
    declaration_births: &[VerifiedGenerativeDeclarationBirthV1],
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(NORMALIZED_CHAIN_ROOT_TAG_V1);
    encoder.u16(CONTIGUOUS_GENERATIVE_STAGE_CHAIN_SCHEMA_VERSION_V1);
    encoder.u16(CONTIGUOUS_GENERATIVE_STAGE_CHAIN_RESOURCE_POLICY_VERSION_V1);
    jg1_manifest_digest.encode_canonical(&mut encoder);
    constructor_grammar_manifest_digest.encode_canonical(&mut encoder);
    scope_grammar_digest.encode_canonical(&mut encoder);
    kernel_protocol_digest.encode_canonical(&mut encoder);
    normalizer_protocol_digest.encode_canonical(&mut encoder);
    kernel_configuration_digest.encode_canonical(&mut encoder);
    encoder.u64(event_count);
    normalized_history_digest.encode_canonical(&mut encoder);
    terminal_boundary.digest().encode_canonical(&mut encoder);
    encoder.sequence(terminal_boundary.declarations());
    encoder.u64(birth_count);
    encoder.sequence(declaration_births);
    Digest::of_domain_bytes(NORMALIZED_CHAIN_ID_DOMAIN_V1, encoder.as_bytes())
}

fn exact_chain_evidence_digest_v1(
    normalized_chain_digest: &Digest,
    stages: &[VerifiedGenerativeCapabilityStageSurfaceV1],
) -> Result<Digest, ContiguousGenerativeStageChainFailureV1> {
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(EXACT_CHAIN_EVIDENCE_ROOT_TAG_V1);
    encoder.u16(CONTIGUOUS_GENERATIVE_STAGE_CHAIN_SCHEMA_VERSION_V1);
    encoder.u16(CONTIGUOUS_GENERATIVE_STAGE_CHAIN_RESOURCE_POLICY_VERSION_V1);
    normalized_chain_digest.encode_canonical(&mut encoder);
    encoder.u64(
        u64::try_from(stages.len())
            .map_err(|_| ContiguousGenerativeStageChainFailureV1::WireCountOverflow)?,
    );
    for stage in stages {
        stage.manifest_digest().encode_canonical(&mut encoder);
    }
    Ok(Digest::of_domain_bytes(
        EXACT_CHAIN_EVIDENCE_DOMAIN_V1,
        encoder.as_bytes(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        generative_capability_candidate_digest_v1,
        proposed_generative_capability_constructor_grammar_v1,
        proposed_pre_exposure_generative_capability_grammar_v1,
        verify_generative_capability_constructor_grammar_v1,
        verify_generative_capability_stage_surface_v1,
        verify_pre_exposure_generative_capability_grammar_v1,
    };
    use pen_kernel::{
        CertificateClaim, Declaration, ScopeInputs, Term, TrustedScope,
        UncheckedFreeSealingCertificate,
    };

    fn digest(label: &[u8]) -> Digest {
        Digest::of_bytes(label)
    }

    fn kernel() -> Kernel {
        Kernel::new(pen_kernel::KernelLimits::default()).expect("valid kernel")
    }

    fn grammars() -> (
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

    fn declaration(label: &[u8]) -> Declaration {
        Declaration {
            id: GlobalId(digest(label)),
            ty: Term::UnitType,
            body: None,
        }
    }

    fn candidate(labels: &[&[u8]]) -> UncheckedSignature {
        UncheckedSignature {
            declarations: labels.iter().map(|label| declaration(label)).collect(),
        }
    }

    fn normalizing_candidate(label: &[u8]) -> UncheckedSignature {
        UncheckedSignature {
            declarations: vec![Declaration {
                id: GlobalId(digest(label)),
                ty: Term::UnitType,
                body: Some(Term::Apply {
                    function: Box::new(Term::Lambda {
                        parameter_type: Box::new(Term::UnitType),
                        body: Box::new(Term::Var { index: 0 }),
                    }),
                    argument: Box::new(Term::Unit),
                }),
            }],
        }
    }

    fn normalized_candidate_with_body(label: &[u8]) -> UncheckedSignature {
        UncheckedSignature {
            declarations: vec![Declaration {
                id: GlobalId(digest(label)),
                ty: Term::UnitType,
                body: Some(Term::Unit),
            }],
        }
    }

    fn exact_scope(
        kernel: &Kernel,
        constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
        predecessor: &VerifiedSignature,
        exact_candidate: &UncheckedSignature,
        scope_label: &[u8],
    ) -> TrustedScope {
        TrustedScope::new(
            kernel,
            CertificateClaim::FreeSealingFragment,
            ScopeInputs {
                law_digest: digest(b"law"),
                grammar_digest: constructors.scope_grammar_digest().clone(),
                scheme_calculus_digest: digest(b"scheme"),
                blindness_contract_digest: digest(b"blindness"),
                bootstrap_contract_digest: digest(b"bootstrap"),
                history_digest: digest(scope_label),
                public_boundary_digest: predecessor.digest().clone(),
                derivation_basis_digest: digest(b"derivation-basis"),
                active_window_digest: digest(b"active-window"),
                candidate_digest: generative_capability_candidate_digest_v1(exact_candidate),
            },
        )
    }

    fn stage(
        jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
        constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
        kernel: &Kernel,
        predecessor: &VerifiedSignature,
        exact_candidate: &UncheckedSignature,
        scope_label: &[u8],
    ) -> VerifiedGenerativeCapabilityStageSurfaceV1 {
        let scope = exact_scope(
            kernel,
            constructors,
            predecessor,
            exact_candidate,
            scope_label,
        );
        let replayed = kernel
            .verify_extension(predecessor, exact_candidate)
            .expect("valid exact extension");
        let certificate = UncheckedFreeSealingCertificate {
            binding: scope.binding().clone(),
            subject_digest: kernel
                .sealing_subject_digest(predecessor, exact_candidate)
                .expect("sealing subject"),
            extension: exact_candidate.clone(),
            normalized_sealed_signature: replayed.normalized_wire(),
        };
        let sealing = kernel
            .verify_free_sealing_certificate(&scope, predecessor, exact_candidate, &certificate)
            .expect("verified free sealing");
        verify_generative_capability_stage_surface_v1(
            jg1,
            constructors,
            kernel,
            &scope,
            predecessor,
            exact_candidate,
            &sealing,
        )
        .expect("verified exact stage")
    }

    fn stages_for(
        jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
        constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
        kernel: &Kernel,
        candidates: &[UncheckedSignature],
        scope_prefix: &[u8],
    ) -> Vec<VerifiedGenerativeCapabilityStageSurfaceV1> {
        let mut predecessor = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty predecessor");
        candidates
            .iter()
            .enumerate()
            .map(|(index, exact_candidate)| {
                let mut scope_label = scope_prefix.to_vec();
                scope_label.extend_from_slice(&index.to_le_bytes());
                let verified = stage(
                    jg1,
                    constructors,
                    kernel,
                    &predecessor,
                    exact_candidate,
                    &scope_label,
                );
                predecessor = verified.successor_boundary().clone();
                verified
            })
            .collect()
    }

    #[test]
    fn empty_chain_is_deterministic_and_relative_only() {
        let kernel = kernel();
        let (jg1, constructors) = grammars();

        let first = derive_contiguous_generative_stage_chain_v1(&jg1, &constructors, &kernel, &[])
            .expect("empty relative chain");
        let second = derive_contiguous_generative_stage_chain_v1(&jg1, &constructors, &kernel, &[])
            .expect("deterministic empty relative chain");

        assert_eq!(first, second);
        assert_eq!(first.schema_version(), 1);
        assert_eq!(first.resource_policy_version(), 1);
        assert!(first.events().is_empty());
        assert!(first.declaration_births().is_empty());
        assert!(first.initial_boundary().declarations().is_empty());
        assert!(same_boundary(
            first.initial_boundary(),
            first.terminal_boundary()
        ));
        assert!(same_boundary(
            &first
                .replay_boundary_after(&kernel, 0)
                .expect("empty replay"),
            first.terminal_boundary()
        ));
        assert_eq!(
            first
                .replay_boundary_after(&kernel, 1)
                .expect_err("out-of-range prefix must fail"),
            ContiguousGenerativeStageChainFailureV1::ReplayEventCountOutOfRange
        );
    }

    #[test]
    fn adjacent_replay_derives_events_and_exact_multi_declaration_birth_coverage() {
        let kernel = kernel();
        let (jg1, constructors) = grammars();
        let stages = stages_for(
            &jg1,
            &constructors,
            &kernel,
            &[candidate(&[b"a", b"b"]), candidate(&[b"c"])],
            b"births",
        );
        let chain =
            derive_contiguous_generative_stage_chain_v1(&jg1, &constructors, &kernel, &stages)
                .expect("adjacent chain");

        assert_eq!(chain.events().len(), 2);
        assert_eq!(chain.declaration_births().len(), 3);
        assert_eq!(chain.terminal_boundary().declarations().len(), 3);
        assert_eq!(chain.events()[0].event_ordinal(), 0);
        assert_eq!(chain.events()[0].extension_declaration_count(), 2);
        assert_eq!(chain.events()[1].event_ordinal(), 1);
        assert_eq!(chain.events()[1].predecessor_declaration_count(), 2);
        assert_eq!(chain.events()[1].successor_declaration_count(), 3);
        assert_ne!(
            chain.events()[0].transition_id().digest(),
            chain.events()[0].event_id().digest(),
            "transition and history-relative event identities use distinct domains"
        );

        let birth_a = chain
            .declaration_birth(&GlobalId(digest(b"a")))
            .expect("birth a");
        assert_eq!(birth_a.event_ordinal(), 0);
        assert_eq!(birth_a.extension_offset(), 0);
        assert_eq!(birth_a.terminal_declaration_ordinal(), 0);
        let birth_b = chain
            .declaration_birth(&GlobalId(digest(b"b")))
            .expect("birth b");
        assert_eq!(birth_b.event_ordinal(), 0);
        assert_eq!(birth_b.extension_offset(), 1);
        assert_eq!(birth_b.terminal_declaration_ordinal(), 1);
        let birth_c = chain
            .declaration_birth(&GlobalId(digest(b"c")))
            .expect("birth c");
        assert_eq!(birth_c.event_ordinal(), 1);
        assert_eq!(birth_c.extension_offset(), 0);
        assert_eq!(birth_c.terminal_declaration_ordinal(), 2);
        assert_eq!(birth_c.event_id(), chain.events()[1].event_id());
        assert!(
            chain
                .declaration_birth(&GlobalId(digest(b"absent")))
                .is_none()
        );

        let after_first = chain
            .replay_boundary_after(&kernel, 1)
            .expect("first prefix replay");
        assert!(same_boundary(&after_first, stages[0].successor_boundary()));
        let after_all = chain
            .replay_boundary_after(&kernel, 2)
            .expect("complete relative replay");
        assert!(same_boundary(&after_all, chain.terminal_boundary()));
    }

    #[test]
    fn scope_and_raw_normalization_evidence_do_not_enter_normalized_chain_identity() {
        let kernel = kernel();
        let (jg1, constructors) = grammars();
        let redex = normalizing_candidate(b"normalizing");
        let normal = normalized_candidate_with_body(b"normalizing");
        let redex_stages = stages_for(
            &jg1,
            &constructors,
            &kernel,
            std::slice::from_ref(&redex),
            b"scope-redex",
        );
        let normal_stages = stages_for(
            &jg1,
            &constructors,
            &kernel,
            std::slice::from_ref(&normal),
            b"scope-normal",
        );
        assert_ne!(
            redex_stages[0].candidate_digest(),
            normal_stages[0].candidate_digest()
        );
        assert_ne!(
            redex_stages[0].scope_digest(),
            normal_stages[0].scope_digest()
        );
        assert_eq!(
            redex_stages[0].normalized_candidate(),
            normal_stages[0].normalized_candidate()
        );

        let redex_chain = derive_contiguous_generative_stage_chain_v1(
            &jg1,
            &constructors,
            &kernel,
            &redex_stages,
        )
        .expect("redex chain");
        let normal_chain = derive_contiguous_generative_stage_chain_v1(
            &jg1,
            &constructors,
            &kernel,
            &normal_stages,
        )
        .expect("normal chain");

        assert_eq!(
            redex_chain.events()[0].transition_id(),
            normal_chain.events()[0].transition_id()
        );
        assert_eq!(
            redex_chain.events()[0].event_id(),
            normal_chain.events()[0].event_id()
        );
        assert_eq!(
            redex_chain.normalized_chain_digest(),
            normal_chain.normalized_chain_digest()
        );
        assert!(redex_chain.same_normalized_chain_as(&normal_chain));
        assert_ne!(
            redex_chain, normal_chain,
            "exact token equality intentionally retains evidence provenance"
        );
        assert_ne!(
            redex_chain.exact_evidence_binding_digest(),
            normal_chain.exact_evidence_binding_digest()
        );
    }

    #[test]
    fn omission_reorder_duplication_and_genuine_branch_splice_fail_adjacency() {
        let kernel = kernel();
        let (jg1, constructors) = grammars();
        let main = stages_for(
            &jg1,
            &constructors,
            &kernel,
            &[candidate(&[b"a"]), candidate(&[b"b"]), candidate(&[b"c"])],
            b"main",
        );
        let branch = stages_for(
            &jg1,
            &constructors,
            &kernel,
            &[candidate(&[b"x"]), candidate(&[b"y"])],
            b"branch",
        );

        let cases: Vec<(Vec<_>, usize)> = vec![
            (vec![main[0].clone(), main[2].clone()], 1),
            (vec![main[1].clone(), main[0].clone()], 0),
            (vec![main[0].clone(), main[0].clone()], 1),
            (vec![main[0].clone(), branch[1].clone()], 1),
        ];
        for (stages, expected_index) in cases {
            assert_eq!(
                derive_contiguous_generative_stage_chain_v1(&jg1, &constructors, &kernel, &stages,)
                    .expect_err("nonadjacent sequence must fail"),
                ContiguousGenerativeStageChainFailureV1::NonAdjacentStage {
                    stage_index: expected_index
                }
            );
        }
    }

    #[test]
    fn valid_truncation_remains_a_distinct_shorter_relative_chain() {
        let kernel = kernel();
        let (jg1, constructors) = grammars();
        let stages = stages_for(
            &jg1,
            &constructors,
            &kernel,
            &[candidate(&[b"a"]), candidate(&[b"b"])],
            b"truncate",
        );
        let prefix =
            derive_contiguous_generative_stage_chain_v1(&jg1, &constructors, &kernel, &stages[..1])
                .expect("valid prefix");
        let full =
            derive_contiguous_generative_stage_chain_v1(&jg1, &constructors, &kernel, &stages)
                .expect("valid full supplied sequence");

        assert_eq!(prefix.events().len(), 1);
        assert_eq!(full.events().len(), 2);
        assert_ne!(
            prefix.normalized_chain_digest(),
            full.normalized_chain_digest()
        );
        assert!(same_boundary(
            prefix.terminal_boundary(),
            stages[0].successor_boundary()
        ));
    }

    #[test]
    fn split_and_coalesced_segmentations_are_valid_distinct_relative_chains() {
        let kernel = kernel();
        let (jg1, constructors) = grammars();
        let split_stages = stages_for(
            &jg1,
            &constructors,
            &kernel,
            &[candidate(&[b"a"]), candidate(&[b"b"])],
            b"split",
        );
        let coalesced_stages = stages_for(
            &jg1,
            &constructors,
            &kernel,
            &[candidate(&[b"a", b"b"])],
            b"coalesced",
        );
        let split = derive_contiguous_generative_stage_chain_v1(
            &jg1,
            &constructors,
            &kernel,
            &split_stages,
        )
        .expect("split relative chain");
        let coalesced = derive_contiguous_generative_stage_chain_v1(
            &jg1,
            &constructors,
            &kernel,
            &coalesced_stages,
        )
        .expect("coalesced relative chain");

        assert_eq!(split.events().len(), 2);
        assert_eq!(coalesced.events().len(), 1);
        assert_eq!(
            split.terminal_boundary().declarations(),
            coalesced.terminal_boundary().declarations()
        );
        assert_ne!(
            split.normalized_history_digest(),
            coalesced.normalized_history_digest()
        );
        assert_ne!(
            split.normalized_chain_digest(),
            coalesced.normalized_chain_digest()
        );
        assert_ne!(
            split
                .declaration_birth(&GlobalId(digest(b"b")))
                .expect("split b birth")
                .event_ordinal(),
            coalesced
                .declaration_birth(&GlobalId(digest(b"b")))
                .expect("coalesced b birth")
                .event_ordinal()
        );
    }

    #[test]
    fn kernel_configuration_drift_fails_before_chain_replay() {
        let source_kernel = kernel();
        let (jg1, constructors) = grammars();
        let stages = stages_for(
            &jg1,
            &constructors,
            &source_kernel,
            &[candidate(&[b"a"])],
            b"configuration",
        );
        let mut limits = pen_kernel::KernelLimits::default();
        limits.max_operations -= 1;
        let drifted_kernel = Kernel::new(limits).expect("valid drifted kernel");

        assert_eq!(
            derive_contiguous_generative_stage_chain_v1(
                &jg1,
                &constructors,
                &drifted_kernel,
                &stages,
            )
            .expect_err("configuration drift must fail"),
            ContiguousGenerativeStageChainFailureV1::StageKernelConfigurationMismatch {
                stage_index: 0
            }
        );

        let chain = derive_contiguous_generative_stage_chain_v1(
            &jg1,
            &constructors,
            &source_kernel,
            &stages,
        )
        .expect("source configuration chain");
        assert_eq!(
            chain
                .replay_boundary_after(&drifted_kernel, 1)
                .expect_err("replay configuration drift must fail"),
            ContiguousGenerativeStageChainFailureV1::ReplayKernelConfigurationMismatch
        );
    }

    #[test]
    fn stage_depth_resource_bound_fails_closed_before_stage_binding_checks() {
        let source_kernel = kernel();
        let (jg1, constructors) = grammars();
        let stages = stages_for(
            &jg1,
            &constructors,
            &source_kernel,
            &[candidate(&[b"a"]), candidate(&[b"b"])],
            b"depth",
        );
        let shallow_kernel = Kernel::new(pen_kernel::KernelLimits {
            max_operations: 100_000,
            max_depth: 1,
            normalization_fuel: 50_000,
        })
        .expect("valid shallow kernel");

        assert_eq!(
            derive_contiguous_generative_stage_chain_v1(
                &jg1,
                &constructors,
                &shallow_kernel,
                &stages,
            )
            .expect_err("stage count over max depth must fail"),
            ContiguousGenerativeStageChainFailureV1::StageCountLimitExceeded
        );
    }
}
