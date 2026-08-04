//! JG2b1b2 consuming completeness verification through one producer-finalized
//! branch head.
//!
//! The only public verifier consumes the opaque JG2b1b1 closed artifact and
//! accepts an independently supplied kernel. It does not accept caller event
//! slices, counts, boundaries, digests, EOF flags, or branch selectors.

use crate::linear_append_log::{
    ClosedGenerativeSealedLogReplayPartsV1, EXACT_HEAD_STEP_DOMAIN_V1, EXACT_HEAD_STEP_ROOT_TAG_V1,
    NORMALIZED_HEAD_STEP_DOMAIN_V1, NORMALIZED_HEAD_STEP_ROOT_TAG_V1,
    NORMALIZED_RETAINED_EXTENSION_DOMAIN_V1, ProcessLocalGenerativeBranchIdV1,
    encode_process_local_branch_id_v1, exact_append_frame_digest_v1, exact_log_genesis_digest_v1,
    head_step_digest_v1, normalized_append_frame_digest_v1, normalized_log_genesis_digest_v1,
    producer_finalized_head_commitment_from_parts_v1,
};
use crate::{
    ClosedGenerativeSealedLogV1, ContiguousGenerativeStageChainFailureV1,
    GenerativeCapabilityStageSurfaceFailureV1, VerifiedContiguousGenerativeStageChainV1,
    VerifiedGenerativeCapabilityConstructorGrammarV1, VerifiedGenerativeCapabilityStageSurfaceV1,
    VerifiedPreExposureGenerativeCapabilityGrammarV1, derive_contiguous_generative_stage_chain_v1,
    generative_capability_candidate_digest_v1,
    generative_capability_kernel_configuration_digest_v1,
    verify_generative_capability_stage_surface_v1,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, CertificateError, Digest, Kernel, KernelError,
    UncheckedFreeSealingCertificate, UncheckedSignature, VerifiedSignature,
};
use pen_sealed_history::{
    VerifiedTargetNeutralSealedLogProtocolV1,
    target_neutral_sealed_log_kernel_configuration_digest_v1,
};

pub const COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_SCHEMA_VERSION_V1: u16 = 1;
pub const COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_RESOURCE_POLICY_VERSION_V1: u16 =
    1;

const COMPLETE_THROUGH_HEAD_COMMITMENT_DOMAIN_V1: &str =
    "law-v2/jg2b1b2/complete-target-neutral-history-through-producer-finalized-head/v1";
const COMPLETE_THROUGH_HEAD_COMMITMENT_ROOT_TAG_V1: u8 = 0xe1;

/// Exact retained field that failed fresh replay comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CompleteThroughHeadRetainedFieldV1 {
    EventOrdinal,
    PredecessorDigest,
    SuccessorDigest,
    NormalizedExtensionDigest,
    CandidateDigest,
    SealingSubjectDigest,
    ExactStageManifestDigest,
    NormalizedFrameDigest,
    PreNormalizedHeadDigest,
    PostNormalizedHeadDigest,
    ExactFrameDigest,
    PreExactHeadDigest,
    PostExactHeadDigest,
}

/// Fail-closed reasons why JG2b1b2 complete-through-head authority was not
/// minted. The consumed closed artifact is never reconstructed from caller
/// data on failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1 {
    ProtocolKernelMismatch,
    ProtocolNormalizerMismatch,
    ProtocolKernelConfigurationMismatch,
    ConstructorGrammarJg1Mismatch,
    StageCountLimitExceeded,
    RetainedExtensionLimitExceeded,
    TerminalDeclarationLimitExceeded,
    AggregateDeclarationMaterialOverflow,
    AggregateDeclarationMaterialLimitExceeded,
    OutputAllocationFailure,
    WireCountOverflow,
    InitialBoundaryKernelFailure(KernelError),
    EventReplayKernelFailure {
        event_index: usize,
        error: KernelError,
    },
    SealingSubjectKernelFailure {
        event_index: usize,
        error: KernelError,
    },
    SealingVerificationFailure {
        event_index: usize,
        error: CertificateError,
    },
    StageVerificationFailure {
        event_index: usize,
        error: GenerativeCapabilityStageSurfaceFailureV1,
    },
    RetainedFrameMismatch {
        event_index: usize,
        field: CompleteThroughHeadRetainedFieldV1,
    },
    RetainedExtensionCountMismatch,
    AggregateDeclarationMaterialMismatch,
    TerminalBoundaryMismatch,
    TerminalRetainedCountMismatch,
    NormalizedLogHeadMismatch,
    ExactBranchHeadMismatch,
    ProducerFinalizedHeadMismatch,
    ContiguousChainFailure(ContiguousGenerativeStageChainFailureV1),
    ContiguousChainEventCountMismatch,
    ContiguousChainEventMismatch {
        event_index: usize,
    },
    ContiguousChainTerminalMismatch,
    ContiguousChainBirthCoverageMismatch,
}

impl std::fmt::Display for CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProtocolKernelMismatch => {
                formatter.write_str("the replay kernel has a different kernel protocol")
            }
            Self::ProtocolNormalizerMismatch => {
                formatter.write_str("the replay kernel has a different normalizer protocol")
            }
            Self::ProtocolKernelConfigurationMismatch => formatter
                .write_str("the replay kernel has a different sealed-log resource configuration"),
            Self::ConstructorGrammarJg1Mismatch => {
                formatter.write_str("the owned constructor grammar is not bound to owned JG1")
            }
            Self::StageCountLimitExceeded => {
                formatter.write_str("the owned frame count exceeds the kernel depth bound")
            }
            Self::RetainedExtensionLimitExceeded => formatter
                .write_str("the retained extension count exceeds the kernel operation bound"),
            Self::TerminalDeclarationLimitExceeded => formatter
                .write_str("the terminal declaration count exceeds the kernel operation bound"),
            Self::AggregateDeclarationMaterialOverflow => {
                formatter.write_str("checked aggregate declaration-material arithmetic overflowed")
            }
            Self::AggregateDeclarationMaterialLimitExceeded => {
                formatter.write_str("aggregate declaration material exceeds the frozen V1 bound")
            }
            Self::OutputAllocationFailure => {
                formatter.write_str("bounded fresh-stage allocation failed")
            }
            Self::WireCountOverflow => {
                formatter.write_str("a derived complete-history count does not fit V1")
            }
            Self::InitialBoundaryKernelFailure(error) => {
                write!(
                    formatter,
                    "fresh empty-boundary verification failed: {error}"
                )
            }
            Self::EventReplayKernelFailure { event_index, error } => {
                write!(
                    formatter,
                    "event {event_index} extension replay failed: {error}"
                )
            }
            Self::SealingSubjectKernelFailure { event_index, error } => {
                write!(
                    formatter,
                    "event {event_index} sealing subject failed: {error}"
                )
            }
            Self::SealingVerificationFailure { event_index, error } => {
                write!(
                    formatter,
                    "event {event_index} sealing replay failed: {error}"
                )
            }
            Self::StageVerificationFailure { event_index, error } => {
                write!(formatter, "event {event_index} JG2a replay failed: {error}")
            }
            Self::RetainedFrameMismatch { event_index, field } => {
                write!(
                    formatter,
                    "event {event_index} retained {field:?} mismatches replay"
                )
            }
            Self::RetainedExtensionCountMismatch => {
                formatter.write_str("the replayed retained-extension total mismatches the producer")
            }
            Self::AggregateDeclarationMaterialMismatch => formatter
                .write_str("the replayed aggregate declaration material mismatches the producer"),
            Self::TerminalBoundaryMismatch => {
                formatter.write_str("the replayed terminal boundary mismatches the producer")
            }
            Self::TerminalRetainedCountMismatch => formatter.write_str(
                "terminal declarations do not exactly equal retained extension declarations",
            ),
            Self::NormalizedLogHeadMismatch => {
                formatter.write_str("the replayed normalized producer head mismatches")
            }
            Self::ExactBranchHeadMismatch => {
                formatter.write_str("the replayed exact append-state head mismatches")
            }
            Self::ProducerFinalizedHeadMismatch => {
                formatter.write_str("the replayed producer-finalized head mismatches")
            }
            Self::ContiguousChainFailure(error) => {
                write!(formatter, "fresh JG2b1a derivation failed: {error}")
            }
            Self::ContiguousChainEventCountMismatch => {
                formatter.write_str("fresh JG2b1a event count mismatches the complete replay")
            }
            Self::ContiguousChainEventMismatch { event_index } => {
                write!(
                    formatter,
                    "fresh JG2b1a event {event_index} mismatches its stage"
                )
            }
            Self::ContiguousChainTerminalMismatch => {
                formatter.write_str("fresh JG2b1a terminal boundary mismatches complete replay")
            }
            Self::ContiguousChainBirthCoverageMismatch => formatter.write_str(
                "fresh JG2b1a birth census does not cover the complete terminal boundary",
            ),
        }
    }
}

impl std::error::Error for CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1 {}

/// Opaque, linear proof that the complete owned JG2b1b1 frame vector was
/// freshly replayed through its producer-finalized branch head and that exact
/// JG2b1a adjacency and declaration-birth coverage were freshly derived.
///
/// This is branch-relative completeness. It carries no globally-latest, EOF,
/// persistence, substitution, naturality, support, carrier, gain, or selection
/// authority. It is intentionally not `Clone`, `Debug`, serializable,
/// default-constructible, or canonically encodable by callers.
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1;
///
/// fn require_clone<T: Clone>() {}
///
/// fn duplicate() {
///     require_clone::<VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1>();
/// }
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1;
///
/// fn require_debug<T: std::fmt::Debug>() {}
///
/// fn disclose() {
///     require_debug::<VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1>();
/// }
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1;
///
/// fn serialize(value: &VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1) {
///     let _ = serde_json::to_string(value);
/// }
///
/// fn deserialize(bytes: &str) {
///     let _: VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1 =
///         serde_json::from_str(bytes).unwrap();
/// }
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1;
///
/// fn default_mint() {
///     let _ = VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1::default();
/// }
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1;
/// use pen_kernel::CanonicalEncode;
///
/// fn require_canonical_encode<T: CanonicalEncode>() {}
///
/// fn public_codec() {
///     require_canonical_encode::<VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1>();
/// }
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::{
///     ClosedGenerativeSealedLogV1,
///     VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1,
/// };
///
/// fn promote_without_replay(closed: ClosedGenerativeSealedLogV1) {
///     let _ = VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1::from(closed);
/// }
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1;
///
/// fn promote_transcript(bytes: &[u8]) {
///     let _ = VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1::from_transcript(bytes);
/// }
/// ```
pub struct VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1 {
    schema_version: u16,
    resource_policy_version: u16,
    protocol: VerifiedTargetNeutralSealedLogProtocolV1,
    jg1: VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: VerifiedGenerativeCapabilityConstructorGrammarV1,
    branch_id: ProcessLocalGenerativeBranchIdV1,
    producer_finalized_head_commitment_digest: Digest,
    normalized_log_head_digest: Digest,
    exact_branch_head_digest: Digest,
    event_count: u64,
    retained_extension_count: u64,
    aggregate_declaration_material: u64,
    complete_through_head_commitment_digest: Digest,
    reconstructed_stages: Vec<VerifiedGenerativeCapabilityStageSurfaceV1>,
    contiguous_chain: VerifiedContiguousGenerativeStageChainV1,
}

impl VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1 {
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub const fn resource_policy_version(&self) -> u16 {
        self.resource_policy_version
    }

    pub fn protocol_manifest_digest(&self) -> &Digest {
        self.protocol.manifest_digest()
    }

    pub fn jg1_manifest_digest(&self) -> &Digest {
        self.jg1.manifest_digest()
    }

    pub fn constructor_grammar_manifest_digest(&self) -> &Digest {
        self.constructors.manifest_digest()
    }

    pub fn producer_finalized_head_commitment_digest(&self) -> &Digest {
        &self.producer_finalized_head_commitment_digest
    }

    /// Producer's normalized rolling log head, distinct from JG2b1a history.
    pub fn normalized_log_head_digest(&self) -> &Digest {
        &self.normalized_log_head_digest
    }

    /// Producer's terminal rolling exact append-state head, not finalization.
    pub fn exact_branch_head_digest(&self) -> &Digest {
        &self.exact_branch_head_digest
    }

    pub const fn event_count(&self) -> u64 {
        self.event_count
    }

    pub const fn retained_extension_count(&self) -> u64 {
        self.retained_extension_count
    }

    pub const fn aggregate_declaration_material(&self) -> u64 {
        self.aggregate_declaration_material
    }

    pub fn terminal_boundary(&self) -> &VerifiedSignature {
        self.contiguous_chain.terminal_boundary()
    }

    /// JG2b1a's independently derived normalized-history state.
    pub fn normalized_history_digest(&self) -> &Digest {
        self.contiguous_chain.normalized_history_digest()
    }

    pub fn normalized_chain_digest(&self) -> &Digest {
        self.contiguous_chain.normalized_chain_digest()
    }

    pub fn exact_chain_evidence_binding_digest(&self) -> &Digest {
        self.contiguous_chain.exact_evidence_binding_digest()
    }

    pub fn declaration_birth_count(&self) -> usize {
        self.contiguous_chain.declaration_births().len()
    }

    pub fn complete_through_head_commitment_digest(&self) -> &Digest {
        &self.complete_through_head_commitment_digest
    }

    #[allow(dead_code)]
    pub(crate) fn branch_id_for_next_gate(&self) -> &ProcessLocalGenerativeBranchIdV1 {
        &self.branch_id
    }

    #[allow(dead_code)]
    pub(crate) fn protocol_for_next_gate(&self) -> &VerifiedTargetNeutralSealedLogProtocolV1 {
        &self.protocol
    }

    #[allow(dead_code)]
    pub(crate) fn jg1_for_next_gate(&self) -> &VerifiedPreExposureGenerativeCapabilityGrammarV1 {
        &self.jg1
    }

    #[allow(dead_code)]
    pub(crate) fn constructors_for_next_gate(
        &self,
    ) -> &VerifiedGenerativeCapabilityConstructorGrammarV1 {
        &self.constructors
    }

    #[allow(dead_code)]
    pub(crate) fn reconstructed_stages_for_next_gate(
        &self,
    ) -> &[VerifiedGenerativeCapabilityStageSurfaceV1] {
        &self.reconstructed_stages
    }

    #[allow(dead_code)]
    pub(crate) fn contiguous_chain_for_next_gate(
        &self,
    ) -> &VerifiedContiguousGenerativeStageChainV1 {
        &self.contiguous_chain
    }
}

/// Consume one producer-finalized JG2b1b1 artifact and independently verify
/// the complete owned frame vector through that exact head.
///
/// There is deliberately no transcript/count/digest argument:
///
/// ```compile_fail
/// use pen_generative_audit::{
///     ClosedGenerativeSealedLogV1,
///     verify_complete_target_neutral_generative_history_through_head_v1,
/// };
/// use pen_kernel::Kernel;
///
/// fn caller_supplied_transcript(
///     closed: ClosedGenerativeSealedLogV1,
///     kernel: &Kernel,
///     transcript: &[u8],
/// ) {
///     let _ = verify_complete_target_neutral_generative_history_through_head_v1(
///         closed,
///         kernel,
///         transcript,
///     );
/// }
/// ```
///
/// The closed artifact must be passed by value:
///
/// ```compile_fail
/// use pen_generative_audit::{
///     ClosedGenerativeSealedLogV1,
///     verify_complete_target_neutral_generative_history_through_head_v1,
/// };
/// use pen_kernel::Kernel;
///
/// fn borrowed(closed: &ClosedGenerativeSealedLogV1, kernel: &Kernel) {
///     let _ = verify_complete_target_neutral_generative_history_through_head_v1(closed, kernel);
/// }
/// ```
///
/// One owned closed artifact cannot be replayed twice:
///
/// ```compile_fail
/// use pen_generative_audit::{
///     ClosedGenerativeSealedLogV1,
///     verify_complete_target_neutral_generative_history_through_head_v1,
/// };
/// use pen_kernel::Kernel;
///
/// fn twice(closed: ClosedGenerativeSealedLogV1, kernel: &Kernel) {
///     let _ = verify_complete_target_neutral_generative_history_through_head_v1(closed, kernel);
///     let _ = verify_complete_target_neutral_generative_history_through_head_v1(closed, kernel);
/// }
/// ```
pub fn verify_complete_target_neutral_generative_history_through_head_v1(
    closed: ClosedGenerativeSealedLogV1,
    kernel: &Kernel,
) -> Result<
    VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1,
    CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1,
> {
    let parts = closed.into_replay_parts();
    verify_owned_replay_parts_v1(parts, kernel)
}

fn verify_owned_replay_parts_v1(
    parts: ClosedGenerativeSealedLogReplayPartsV1,
    kernel: &Kernel,
) -> Result<
    VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1,
    CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1,
> {
    if parts.protocol.kernel_protocol_digest() != &kernel.kernel_protocol_digest() {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ProtocolKernelMismatch,
        );
    }
    if parts.protocol.normalizer_protocol_digest() != &kernel.normalizer_protocol_digest() {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ProtocolNormalizerMismatch,
        );
    }
    if parts.protocol.kernel_configuration_digest()
        != &target_neutral_sealed_log_kernel_configuration_digest_v1(kernel)
    {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ProtocolKernelConfigurationMismatch,
        );
    }
    if parts.constructors.jg1_manifest_digest() != parts.jg1.manifest_digest() {
        return Err(CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ConstructorGrammarJg1Mismatch);
    }

    let limits = kernel.limits();
    let max_stage_count = usize::from(limits.max_depth);
    let max_declarations = u64::from(limits.max_operations);
    let aggregate_factor = u64::from(limits.max_depth)
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::AggregateDeclarationMaterialOverflow,
        )?;
    let aggregate_limit = max_declarations.checked_mul(aggregate_factor).ok_or(
        CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::AggregateDeclarationMaterialOverflow,
    )?;
    if parts.frames.len() > max_stage_count {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::StageCountLimitExceeded,
        );
    }
    if parts.retained_extension_count > max_declarations {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionLimitExceeded,
        );
    }
    let stored_terminal_count = u64::try_from(parts.terminal_boundary.declarations().len())
        .map_err(|_| {
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
        })?;
    if stored_terminal_count > max_declarations {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::TerminalDeclarationLimitExceeded,
        );
    }
    if parts.aggregate_declaration_material > aggregate_limit {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::AggregateDeclarationMaterialLimitExceeded,
        );
    }

    // Audit the complete owned vector before any replay allocation. This scan
    // derives its own retained count from every raw candidate; no stored count
    // or caller prefix controls enumeration.
    let mut prescanned_retained_extension_count = 0_u64;
    for retained in &parts.frames {
        let candidate_count =
            u64::try_from(retained.exact_candidate.declarations.len()).map_err(|_| {
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
            })?;
        if candidate_count > max_declarations {
            return Err(
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionLimitExceeded,
            );
        }
        prescanned_retained_extension_count = prescanned_retained_extension_count
            .checked_add(candidate_count)
            .ok_or(
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::AggregateDeclarationMaterialOverflow,
            )?;
        if prescanned_retained_extension_count > max_declarations {
            return Err(
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionLimitExceeded,
            );
        }
    }
    if prescanned_retained_extension_count != parts.retained_extension_count {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionCountMismatch,
        );
    }
    if prescanned_retained_extension_count != stored_terminal_count {
        return Err(CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::TerminalRetainedCountMismatch);
    }

    let event_count = u64::try_from(parts.frames.len()).map_err(|_| {
        CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
    })?;
    let mut reconstructed_stages = Vec::new();
    reconstructed_stages
        .try_reserve_exact(parts.frames.len())
        .map_err(|_| {
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::OutputAllocationFailure
        })?;
    let mut current = kernel
        .verify_signature(&UncheckedSignature::default())
        .map_err(
        CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::InitialBoundaryKernelFailure,
    )?;
    let mut normalized_head = normalized_log_genesis_digest_v1(
        &parts.protocol,
        &parts.jg1,
        &parts.constructors,
        kernel,
        &current,
    );
    let mut exact_head = exact_log_genesis_digest_v1(
        &parts.branch_id,
        &parts.protocol,
        &parts.jg1,
        &parts.constructors,
        kernel,
        &normalized_head,
    );
    let mut retained_extension_count = 0_u64;
    let mut aggregate_declaration_material = 0_u64;

    for (event_index, retained) in parts.frames.iter().enumerate() {
        let ordinal = u64::try_from(event_index).map_err(|_| {
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
        })?;
        require_retained_field_v1(
            retained.event_ordinal == ordinal,
            event_index,
            CompleteThroughHeadRetainedFieldV1::EventOrdinal,
        )?;
        require_retained_field_v1(
            retained.predecessor_digest == *current.digest(),
            event_index,
            CompleteThroughHeadRetainedFieldV1::PredecessorDigest,
        )?;
        require_retained_field_v1(
            retained.pre_normalized_head_digest == normalized_head,
            event_index,
            CompleteThroughHeadRetainedFieldV1::PreNormalizedHeadDigest,
        )?;
        require_retained_field_v1(
            retained.pre_exact_head_digest == exact_head,
            event_index,
            CompleteThroughHeadRetainedFieldV1::PreExactHeadDigest,
        )?;

        let successor = kernel
            .verify_extension(&current, &retained.exact_candidate)
            .map_err(|error| {
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::EventReplayKernelFailure {
                    event_index,
                    error,
                }
            })?;
        require_retained_field_v1(
            retained.successor_digest == *successor.digest(),
            event_index,
            CompleteThroughHeadRetainedFieldV1::SuccessorDigest,
        )?;
        let sealing_subject = kernel
            .sealing_subject_digest(&current, &retained.exact_candidate)
            .map_err(|error| {
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::SealingSubjectKernelFailure {
                    event_index,
                    error,
                }
            })?;
        require_retained_field_v1(
            retained.sealing_subject_digest == sealing_subject,
            event_index,
            CompleteThroughHeadRetainedFieldV1::SealingSubjectDigest,
        )?;
        let certificate = UncheckedFreeSealingCertificate {
            binding: retained.scope.binding().clone(),
            subject_digest: sealing_subject,
            extension: retained.exact_candidate.clone(),
            normalized_sealed_signature: successor.normalized_wire(),
        };
        let sealing = kernel
            .verify_free_sealing_certificate(
                &retained.scope,
                &current,
                &retained.exact_candidate,
                &certificate,
            )
            .map_err(|error| {
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::SealingVerificationFailure {
                    event_index,
                    error,
                }
            })?;
        let stage = verify_generative_capability_stage_surface_v1(
            &parts.jg1,
            &parts.constructors,
            kernel,
            &retained.scope,
            &current,
            &retained.exact_candidate,
            &sealing,
        )
        .map_err(|error| {
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::StageVerificationFailure {
                event_index,
                error,
            }
        })?;

        require_retained_field_v1(
            retained.candidate_digest
                == generative_capability_candidate_digest_v1(&retained.exact_candidate),
            event_index,
            CompleteThroughHeadRetainedFieldV1::CandidateDigest,
        )?;
        require_retained_field_v1(
            retained.candidate_digest == *stage.candidate_digest(),
            event_index,
            CompleteThroughHeadRetainedFieldV1::CandidateDigest,
        )?;
        let normalized_extension_digest = Digest::of_canonical(
            NORMALIZED_RETAINED_EXTENSION_DOMAIN_V1,
            stage.normalized_candidate(),
        );
        require_retained_field_v1(
            retained.normalized_extension_digest == normalized_extension_digest,
            event_index,
            CompleteThroughHeadRetainedFieldV1::NormalizedExtensionDigest,
        )?;
        require_retained_field_v1(
            retained.exact_stage_manifest_digest == *stage.manifest_digest(),
            event_index,
            CompleteThroughHeadRetainedFieldV1::ExactStageManifestDigest,
        )?;

        let normalized_frame = normalized_append_frame_digest_v1(
            &parts.protocol,
            &parts.jg1,
            &parts.constructors,
            kernel,
            ordinal,
            &normalized_head,
            stage.predecessor_boundary(),
            stage.normalized_candidate(),
            stage.successor_boundary(),
        )
        .map_err(|_| {
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
        })?;
        require_retained_field_v1(
            retained.normalized_frame_digest == normalized_frame,
            event_index,
            CompleteThroughHeadRetainedFieldV1::NormalizedFrameDigest,
        )?;
        let post_normalized_head = head_step_digest_v1(
            NORMALIZED_HEAD_STEP_DOMAIN_V1,
            NORMALIZED_HEAD_STEP_ROOT_TAG_V1,
            &normalized_head,
            &normalized_frame,
        );
        require_retained_field_v1(
            retained.post_normalized_head_digest == post_normalized_head,
            event_index,
            CompleteThroughHeadRetainedFieldV1::PostNormalizedHeadDigest,
        )?;

        let exact_frame = exact_append_frame_digest_v1(
            &parts.branch_id,
            &parts.protocol,
            &parts.jg1,
            &parts.constructors,
            kernel,
            ordinal,
            &exact_head,
            &normalized_frame,
            &retained.scope,
            &retained.exact_candidate,
            &sealing,
            &stage,
        );
        require_retained_field_v1(
            retained.exact_frame_digest == exact_frame,
            event_index,
            CompleteThroughHeadRetainedFieldV1::ExactFrameDigest,
        )?;
        let post_exact_head = head_step_digest_v1(
            EXACT_HEAD_STEP_DOMAIN_V1,
            EXACT_HEAD_STEP_ROOT_TAG_V1,
            &exact_head,
            &exact_frame,
        );
        require_retained_field_v1(
            retained.post_exact_head_digest == post_exact_head,
            event_index,
            CompleteThroughHeadRetainedFieldV1::PostExactHeadDigest,
        )?;

        let predecessor_count = u64::try_from(current.declarations().len()).map_err(|_| {
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
        })?;
        let extension_count = u64::try_from(stage.normalized_candidate().declarations.len())
            .map_err(|_| {
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
            })?;
        let successor_count = u64::try_from(stage.successor_boundary().declarations().len())
            .map_err(|_| {
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
            })?;
        retained_extension_count = retained_extension_count
            .checked_add(extension_count)
            .ok_or(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::AggregateDeclarationMaterialOverflow,
        )?;
        aggregate_declaration_material = aggregate_declaration_material
            .checked_add(predecessor_count)
            .and_then(|value| value.checked_add(extension_count))
            .and_then(|value| value.checked_add(successor_count))
            .ok_or(
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::AggregateDeclarationMaterialOverflow,
            )?;
        if retained_extension_count > max_declarations {
            return Err(
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionLimitExceeded,
            );
        }
        if aggregate_declaration_material > aggregate_limit {
            return Err(
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::AggregateDeclarationMaterialLimitExceeded,
            );
        }

        current = stage.successor_boundary().clone();
        normalized_head = post_normalized_head;
        exact_head = post_exact_head;
        reconstructed_stages.push(stage);
    }

    if !same_boundary_v1(&current, &parts.terminal_boundary) {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::TerminalBoundaryMismatch,
        );
    }
    if normalized_head != parts.normalized_log_head_digest {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::NormalizedLogHeadMismatch,
        );
    }
    if exact_head != parts.exact_branch_head_digest {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ExactBranchHeadMismatch,
        );
    }
    if retained_extension_count != parts.retained_extension_count {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionCountMismatch,
        );
    }
    if aggregate_declaration_material != parts.aggregate_declaration_material {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::AggregateDeclarationMaterialMismatch,
        );
    }
    let terminal_count = u64::try_from(current.declarations().len()).map_err(|_| {
        CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
    })?;
    if terminal_count != retained_extension_count {
        return Err(CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::TerminalRetainedCountMismatch);
    }

    let finalized_head = producer_finalized_head_commitment_from_parts_v1(
        &parts.protocol,
        &parts.jg1,
        &parts.constructors,
        kernel,
        &parts.branch_id,
        event_count,
        &normalized_head,
        &exact_head,
        &current,
        retained_extension_count,
        aggregate_declaration_material,
    );
    if finalized_head != parts.producer_finalized_head_commitment_digest {
        return Err(CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ProducerFinalizedHeadMismatch);
    }

    let contiguous_chain = derive_contiguous_generative_stage_chain_v1(
        &parts.jg1,
        &parts.constructors,
        kernel,
        &reconstructed_stages,
    )
    .map_err(CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ContiguousChainFailure)?;
    if contiguous_chain.events().len() != reconstructed_stages.len() {
        return Err(CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ContiguousChainEventCountMismatch);
    }
    for (event_index, ((event, stage), retained)) in contiguous_chain
        .events()
        .iter()
        .zip(&reconstructed_stages)
        .zip(&parts.frames)
        .enumerate()
    {
        if event.event_ordinal()
            != u64::try_from(event_index).map_err(|_| {
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
            })?
            || event.predecessor_digest() != stage.predecessor_digest()
            || event.successor_digest() != stage.successor_digest()
            || event.normalized_extension() != stage.normalized_candidate()
            || event.exact_stage_manifest_digest() != &retained.exact_stage_manifest_digest
        {
            return Err(
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ContiguousChainEventMismatch {
                    event_index,
                },
            );
        }
    }
    if !same_boundary_v1(contiguous_chain.terminal_boundary(), &current) {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ContiguousChainTerminalMismatch,
        );
    }
    if contiguous_chain.declaration_births().len() != current.declarations().len() {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ContiguousChainBirthCoverageMismatch,
        );
    }
    let chain_replayed_terminal = contiguous_chain
        .replay_boundary_after(kernel, parts.frames.len())
        .map_err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ContiguousChainFailure,
        )?;
    if !same_boundary_v1(&chain_replayed_terminal, &current) {
        return Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ContiguousChainTerminalMismatch,
        );
    }

    let complete_through_head_commitment_digest = complete_through_head_commitment_digest_v1(
        &parts.protocol,
        &parts.jg1,
        &parts.constructors,
        kernel,
        &parts.branch_id,
        &finalized_head,
        &normalized_head,
        &exact_head,
        event_count,
        retained_extension_count,
        aggregate_declaration_material,
        &current,
        &contiguous_chain,
    )?;

    Ok(
        VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1 {
            schema_version:
                COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_SCHEMA_VERSION_V1,
            resource_policy_version:
                COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_RESOURCE_POLICY_VERSION_V1,
            protocol: parts.protocol,
            jg1: parts.jg1,
            constructors: parts.constructors,
            branch_id: parts.branch_id,
            producer_finalized_head_commitment_digest: finalized_head,
            normalized_log_head_digest: normalized_head,
            exact_branch_head_digest: exact_head,
            event_count,
            retained_extension_count,
            aggregate_declaration_material,
            complete_through_head_commitment_digest,
            reconstructed_stages,
            contiguous_chain,
        },
    )
}

fn require_retained_field_v1(
    condition: bool,
    event_index: usize,
    field: CompleteThroughHeadRetainedFieldV1,
) -> Result<(), CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1> {
    if condition {
        Ok(())
    } else {
        Err(
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedFrameMismatch {
                event_index,
                field,
            },
        )
    }
}

fn same_boundary_v1(left: &VerifiedSignature, right: &VerifiedSignature) -> bool {
    left.digest() == right.digest() && left.declarations() == right.declarations()
}

#[allow(clippy::too_many_arguments)]
fn complete_through_head_commitment_digest_v1(
    protocol: &VerifiedTargetNeutralSealedLogProtocolV1,
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
    branch_id: &ProcessLocalGenerativeBranchIdV1,
    producer_finalized_head_digest: &Digest,
    normalized_log_head_digest: &Digest,
    exact_branch_head_digest: &Digest,
    event_count: u64,
    retained_extension_count: u64,
    aggregate_declaration_material: u64,
    terminal_boundary: &VerifiedSignature,
    chain: &VerifiedContiguousGenerativeStageChainV1,
) -> Result<Digest, CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1> {
    let terminal_count = u64::try_from(terminal_boundary.declarations().len()).map_err(|_| {
        CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
    })?;
    let birth_count = u64::try_from(chain.declaration_births().len()).map_err(|_| {
        CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::WireCountOverflow
    })?;
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(COMPLETE_THROUGH_HEAD_COMMITMENT_ROOT_TAG_V1);
    encoder.u16(COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_SCHEMA_VERSION_V1);
    encoder.u16(COMPLETE_TARGET_NEUTRAL_GENERATIVE_HISTORY_THROUGH_HEAD_RESOURCE_POLICY_VERSION_V1);
    protocol.manifest_digest().encode_canonical(&mut encoder);
    jg1.manifest_digest().encode_canonical(&mut encoder);
    constructors
        .manifest_digest()
        .encode_canonical(&mut encoder);
    constructors
        .scope_grammar_digest()
        .encode_canonical(&mut encoder);
    kernel
        .kernel_protocol_digest()
        .encode_canonical(&mut encoder);
    kernel
        .normalizer_protocol_digest()
        .encode_canonical(&mut encoder);
    generative_capability_kernel_configuration_digest_v1(kernel).encode_canonical(&mut encoder);
    protocol
        .kernel_configuration_digest()
        .encode_canonical(&mut encoder);
    encode_process_local_branch_id_v1(&mut encoder, branch_id);
    producer_finalized_head_digest.encode_canonical(&mut encoder);
    normalized_log_head_digest.encode_canonical(&mut encoder);
    exact_branch_head_digest.encode_canonical(&mut encoder);
    encoder.u64(event_count);
    encoder.u64(retained_extension_count);
    encoder.u64(aggregate_declaration_material);
    terminal_boundary.digest().encode_canonical(&mut encoder);
    encoder.u64(terminal_count);
    encoder.sequence(terminal_boundary.declarations());
    chain
        .normalized_history_digest()
        .encode_canonical(&mut encoder);
    chain
        .normalized_chain_digest()
        .encode_canonical(&mut encoder);
    chain
        .exact_evidence_binding_digest()
        .encode_canonical(&mut encoder);
    encoder.u64(birth_count);
    encoder.sequence(chain.declaration_births());
    Ok(Digest::of_domain_bytes(
        COMPLETE_THROUGH_HEAD_COMMITMENT_DOMAIN_V1,
        encoder.as_bytes(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_append_log::RetainedGenerativeSealedFrameV1;
    use crate::{
        GenerativeSealedLogProducerFailureV1, OpenGenerativeSealedLogV1,
        append_verified_generative_stage_v1, begin_generative_sealed_log_v1,
        close_generative_sealed_log_v1, proposed_generative_capability_constructor_grammar_v1,
        proposed_pre_exposure_generative_capability_grammar_v1,
        verify_generative_capability_constructor_grammar_v1,
        verify_pre_exposure_generative_capability_grammar_v1,
    };
    use pen_kernel::{
        CertificateClaim, Declaration, GlobalId, KernelLimits, ScopeInputs, Term, TrustedScope,
        VerifiedFreeSealing,
    };
    use pen_sealed_history::{
        proposed_target_neutral_sealed_log_protocol_v1,
        verify_target_neutral_sealed_log_protocol_v1,
    };

    type Authorities = (
        VerifiedTargetNeutralSealedLogProtocolV1,
        VerifiedPreExposureGenerativeCapabilityGrammarV1,
        VerifiedGenerativeCapabilityConstructorGrammarV1,
    );

    fn digest(label: &[u8]) -> Digest {
        Digest::of_bytes(label)
    }

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("valid kernel")
    }

    fn authorities(kernel: &Kernel) -> Authorities {
        let protocol = verify_target_neutral_sealed_log_protocol_v1(
            kernel,
            &proposed_target_neutral_sealed_log_protocol_v1(kernel),
        )
        .expect("exact sealed-log protocol");
        let jg1 = verify_pre_exposure_generative_capability_grammar_v1(
            &proposed_pre_exposure_generative_capability_grammar_v1(),
        )
        .expect("exact JG1");
        let constructors = verify_generative_capability_constructor_grammar_v1(
            &jg1,
            &proposed_generative_capability_constructor_grammar_v1(&jg1),
        )
        .expect("exact constructors");
        (protocol, jg1, constructors)
    }

    fn candidate(labels: &[&[u8]]) -> UncheckedSignature {
        UncheckedSignature {
            declarations: labels
                .iter()
                .map(|label| Declaration {
                    id: GlobalId(digest(label)),
                    ty: Term::UnitType,
                    body: None,
                })
                .collect(),
        }
    }

    fn redex_candidate(label: &[u8]) -> UncheckedSignature {
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

    fn scope_and_sealing(
        kernel: &Kernel,
        constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
        predecessor: &VerifiedSignature,
        exact_candidate: &UncheckedSignature,
        scope_label: &[u8],
    ) -> (TrustedScope, VerifiedFreeSealing) {
        let scope = TrustedScope::new(
            kernel,
            CertificateClaim::FreeSealingFragment,
            ScopeInputs {
                law_digest: digest(b"complete-law"),
                grammar_digest: constructors.scope_grammar_digest().clone(),
                scheme_calculus_digest: digest(b"complete-scheme"),
                blindness_contract_digest: digest(b"complete-blindness"),
                bootstrap_contract_digest: digest(b"complete-bootstrap"),
                history_digest: digest(scope_label),
                public_boundary_digest: predecessor.digest().clone(),
                derivation_basis_digest: digest(b"complete-basis"),
                active_window_digest: digest(b"complete-window"),
                candidate_digest: generative_capability_candidate_digest_v1(exact_candidate),
            },
        );
        let successor = kernel
            .verify_extension(predecessor, exact_candidate)
            .expect("valid extension");
        let certificate = UncheckedFreeSealingCertificate {
            binding: scope.binding().clone(),
            subject_digest: kernel
                .sealing_subject_digest(predecessor, exact_candidate)
                .expect("sealing subject"),
            extension: exact_candidate.clone(),
            normalized_sealed_signature: successor.normalized_wire(),
        };
        let sealing = kernel
            .verify_free_sealing_certificate(&scope, predecessor, exact_candidate, &certificate)
            .expect("verified sealing");
        (scope, sealing)
    }

    fn append(
        open: &mut OpenGenerativeSealedLogV1,
        kernel: &Kernel,
        constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
        exact_candidate: &UncheckedSignature,
        scope_label: &[u8],
    ) -> Result<(), GenerativeSealedLogProducerFailureV1> {
        let (scope, sealing) = scope_and_sealing(
            kernel,
            constructors,
            open.current_boundary(),
            exact_candidate,
            scope_label,
        );
        append_verified_generative_stage_v1(open, &scope, exact_candidate, &sealing).map(|_| ())
    }

    fn two_event_closed(kernel: &Kernel) -> ClosedGenerativeSealedLogV1 {
        let (protocol, jg1, constructors) = authorities(kernel);
        let mut open = begin_generative_sealed_log_v1(&protocol, &jg1, &constructors, kernel)
            .expect("open producer");
        append(
            &mut open,
            kernel,
            &constructors,
            &redex_candidate(b"complete-a"),
            b"complete-scope-a",
        )
        .expect("first append");
        append(
            &mut open,
            kernel,
            &constructors,
            &candidate(&[b"complete-b", b"complete-c"]),
            b"complete-scope-b",
        )
        .expect("second append");
        close_generative_sealed_log_v1(open)
    }

    fn singleton_event_closed(kernel: &Kernel, labels: &[&[u8]]) -> ClosedGenerativeSealedLogV1 {
        let (protocol, jg1, constructors) = authorities(kernel);
        let mut open = begin_generative_sealed_log_v1(&protocol, &jg1, &constructors, kernel)
            .expect("open singleton-event producer");
        for label in labels {
            append(
                &mut open,
                kernel,
                &constructors,
                &candidate(&[*label]),
                label,
            )
            .expect("singleton append");
        }
        close_generative_sealed_log_v1(open)
    }

    fn failure(
        result: Result<
            VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1,
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1,
        >,
    ) -> CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1 {
        match result {
            Ok(_) => panic!("mutation must not mint complete-through-head authority"),
            Err(error) => error,
        }
    }

    #[test]
    fn empty_and_two_event_owned_histories_verify_through_their_finalized_heads() {
        let kernel = kernel();
        let (protocol, jg1, constructors) = authorities(&kernel);
        let empty_open = begin_generative_sealed_log_v1(&protocol, &jg1, &constructors, &kernel)
            .expect("empty open producer");
        let empty = verify_complete_target_neutral_generative_history_through_head_v1(
            close_generative_sealed_log_v1(empty_open),
            &kernel,
        )
        .expect("complete empty owned history");
        assert_eq!(empty.event_count(), 0);
        assert_eq!(empty.retained_extension_count(), 0);
        assert_eq!(empty.aggregate_declaration_material(), 0);
        assert!(empty.terminal_boundary().declarations().is_empty());
        assert_eq!(empty.declaration_birth_count(), 0);
        assert!(empty.reconstructed_stages_for_next_gate().is_empty());

        let closed = two_event_closed(&kernel);
        let expected_finalized = closed.producer_finalized_head_commitment_digest().clone();
        let complete =
            verify_complete_target_neutral_generative_history_through_head_v1(closed, &kernel)
                .expect("complete two-event owned history");
        assert_eq!(complete.schema_version(), 1);
        assert_eq!(complete.resource_policy_version(), 1);
        assert_eq!(complete.event_count(), 2);
        assert_eq!(complete.retained_extension_count(), 3);
        assert_eq!(complete.aggregate_declaration_material(), 8);
        assert_eq!(complete.terminal_boundary().declarations().len(), 3);
        assert_eq!(complete.declaration_birth_count(), 3);
        assert_eq!(
            complete.producer_finalized_head_commitment_digest(),
            &expected_finalized
        );
        assert_ne!(
            complete.normalized_log_head_digest(),
            complete.normalized_history_digest(),
            "producer and JG2b1a normalized domains remain distinct"
        );
        assert_eq!(complete.reconstructed_stages_for_next_gate().len(), 2);
        assert_eq!(complete.contiguous_chain_for_next_gate().events().len(), 2);
    }

    #[test]
    fn retained_field_candidate_order_prefix_and_branch_substitutions_fail_closed() {
        let kernel = kernel();

        let mut ordinal = two_event_closed(&kernel);
        ordinal.frames_mut_for_test()[1].event_ordinal = 9;
        assert_eq!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(ordinal, &kernel,)
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedFrameMismatch {
                event_index: 1,
                field: CompleteThroughHeadRetainedFieldV1::EventOrdinal,
            }
        );

        let mut digest_mutation = two_event_closed(&kernel);
        digest_mutation.frames_mut_for_test()[0].normalized_frame_digest = digest(b"mutation");
        assert_eq!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(
                    digest_mutation,
                    &kernel,
                )
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedFrameMismatch {
                event_index: 0,
                field: CompleteThroughHeadRetainedFieldV1::NormalizedFrameDigest,
            }
        );

        let mut candidate_mutation = two_event_closed(&kernel);
        candidate_mutation.frames_mut_for_test()[0]
            .exact_candidate
            .declarations[0]
            .id = GlobalId(digest(b"mutated-candidate"));
        assert_eq!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(
                    candidate_mutation,
                    &kernel,
                )
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedFrameMismatch {
                event_index: 0,
                field: CompleteThroughHeadRetainedFieldV1::SuccessorDigest,
            }
        );

        let mut reorder = two_event_closed(&kernel);
        reorder.frames_mut_for_test().swap(0, 1);
        assert!(matches!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(reorder, &kernel,)
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedFrameMismatch {
                event_index: 0,
                ..
            }
        ));

        let mut prefix = two_event_closed(&kernel);
        prefix.frames_mut_for_test().pop();
        assert_eq!(
            failure(verify_complete_target_neutral_generative_history_through_head_v1(
                prefix, &kernel,
            )),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionCountMismatch
        );

        let mut left = two_event_closed(&kernel);
        let mut right = two_event_closed(&kernel);
        std::mem::swap(
            left.branch_id_mut_for_test(),
            right.branch_id_mut_for_test(),
        );
        assert_eq!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(left, &kernel,)
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedFrameMismatch {
                event_index: 0,
                field: CompleteThroughHeadRetainedFieldV1::PreExactHeadDigest,
            }
        );
    }

    #[test]
    fn every_retained_digest_and_pre_post_head_field_is_rederived() {
        type Mutation = fn(&mut RetainedGenerativeSealedFrameV1);
        let mutations: [(CompleteThroughHeadRetainedFieldV1, Mutation); 12] = [
            (
                CompleteThroughHeadRetainedFieldV1::PredecessorDigest,
                |frame| frame.predecessor_digest = digest(b"wrong-predecessor"),
            ),
            (
                CompleteThroughHeadRetainedFieldV1::SuccessorDigest,
                |frame| frame.successor_digest = digest(b"wrong-successor"),
            ),
            (
                CompleteThroughHeadRetainedFieldV1::NormalizedExtensionDigest,
                |frame| frame.normalized_extension_digest = digest(b"wrong-extension"),
            ),
            (
                CompleteThroughHeadRetainedFieldV1::CandidateDigest,
                |frame| frame.candidate_digest = digest(b"wrong-candidate"),
            ),
            (
                CompleteThroughHeadRetainedFieldV1::SealingSubjectDigest,
                |frame| frame.sealing_subject_digest = digest(b"wrong-sealing"),
            ),
            (
                CompleteThroughHeadRetainedFieldV1::ExactStageManifestDigest,
                |frame| frame.exact_stage_manifest_digest = digest(b"wrong-stage"),
            ),
            (
                CompleteThroughHeadRetainedFieldV1::NormalizedFrameDigest,
                |frame| frame.normalized_frame_digest = digest(b"wrong-normalized-frame"),
            ),
            (
                CompleteThroughHeadRetainedFieldV1::PreNormalizedHeadDigest,
                |frame| frame.pre_normalized_head_digest = digest(b"wrong-pre-normalized"),
            ),
            (
                CompleteThroughHeadRetainedFieldV1::PostNormalizedHeadDigest,
                |frame| frame.post_normalized_head_digest = digest(b"wrong-post-normalized"),
            ),
            (
                CompleteThroughHeadRetainedFieldV1::ExactFrameDigest,
                |frame| frame.exact_frame_digest = digest(b"wrong-exact-frame"),
            ),
            (
                CompleteThroughHeadRetainedFieldV1::PreExactHeadDigest,
                |frame| frame.pre_exact_head_digest = digest(b"wrong-pre-exact"),
            ),
            (
                CompleteThroughHeadRetainedFieldV1::PostExactHeadDigest,
                |frame| frame.post_exact_head_digest = digest(b"wrong-post-exact"),
            ),
        ];

        let kernel = kernel();
        for (expected_field, mutate) in mutations {
            let mut closed = two_event_closed(&kernel);
            mutate(&mut closed.frames_mut_for_test()[0]);
            assert_eq!(
                failure(
                    verify_complete_target_neutral_generative_history_through_head_v1(
                        closed, &kernel,
                    )
                ),
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedFrameMismatch {
                    event_index: 0,
                    field: expected_field,
                }
            );
        }
    }

    #[test]
    fn duplicate_trailing_and_interior_omission_cannot_be_promoted() {
        let kernel = kernel();

        let mut duplicate = singleton_event_closed(&kernel, &[b"dup-a", b"dup-b"]);
        let mut duplicate_donor = singleton_event_closed(&kernel, &[b"dup-a"]);
        let duplicate_frame = duplicate_donor
            .frames_mut_for_test()
            .pop()
            .expect("donor frame");
        duplicate.frames_mut_for_test().insert(1, duplicate_frame);
        assert_eq!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(
                    duplicate, &kernel,
                )
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionCountMismatch
        );

        let mut trailing = singleton_event_closed(&kernel, &[b"trail-a", b"trail-b"]);
        let mut trailing_donor = singleton_event_closed(&kernel, &[b"trail-c"]);
        let trailing_frame = trailing_donor
            .frames_mut_for_test()
            .pop()
            .expect("donor frame");
        trailing.frames_mut_for_test().push(trailing_frame);
        assert_eq!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(
                    trailing, &kernel,
                )
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionCountMismatch
        );

        let mut omitted = singleton_event_closed(&kernel, &[b"omit-a", b"omit-b", b"omit-c"]);
        omitted.frames_mut_for_test().remove(1);
        assert_eq!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(
                    omitted, &kernel,
                )
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionCountMismatch
        );
    }

    #[test]
    fn resource_totals_and_finalized_head_mutations_fail_closed() {
        let kernel = kernel();
        let limits = kernel.limits();

        let mut retained_overflow = two_event_closed(&kernel);
        *retained_overflow.retained_extension_count_mut_for_test() =
            u64::from(limits.max_operations) + 1;
        assert_eq!(
            failure(verify_complete_target_neutral_generative_history_through_head_v1(
                retained_overflow,
                &kernel,
            )),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionLimitExceeded
        );

        let aggregate_limit =
            u64::from(limits.max_operations) * (u64::from(limits.max_depth) * 2 + 1);
        let mut aggregate_overflow = two_event_closed(&kernel);
        *aggregate_overflow.aggregate_declaration_material_mut_for_test() = aggregate_limit + 1;
        assert_eq!(
            failure(verify_complete_target_neutral_generative_history_through_head_v1(
                aggregate_overflow,
                &kernel,
            )),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::AggregateDeclarationMaterialLimitExceeded
        );

        let mut wrong_total = two_event_closed(&kernel);
        *wrong_total.retained_extension_count_mut_for_test() = 2;
        assert_eq!(
            failure(verify_complete_target_neutral_generative_history_through_head_v1(
                wrong_total,
                &kernel,
            )),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::RetainedExtensionCountMismatch
        );

        let mut finalized = two_event_closed(&kernel);
        *finalized.producer_finalized_head_mut_for_test() = digest(b"wrong-finalized-head");
        assert_eq!(
            failure(verify_complete_target_neutral_generative_history_through_head_v1(
                finalized, &kernel,
            )),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ProducerFinalizedHeadMismatch
        );
    }

    #[test]
    fn terminal_producer_heads_and_within_limit_aggregate_are_recomputed() {
        let kernel = kernel();

        let mut terminal = two_event_closed(&kernel);
        *terminal.terminal_boundary_mut_for_test() = kernel
            .verify_signature(&candidate(&[
                b"alternate-terminal-a",
                b"alternate-terminal-b",
                b"alternate-terminal-c",
            ]))
            .expect("alternate same-sized terminal");
        assert_eq!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(
                    terminal, &kernel,
                )
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::TerminalBoundaryMismatch
        );

        let mut normalized = two_event_closed(&kernel);
        *normalized.normalized_log_head_mut_for_test() = digest(b"wrong-normalized-log-head");
        assert_eq!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(
                    normalized, &kernel,
                )
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::NormalizedLogHeadMismatch
        );

        let mut exact = two_event_closed(&kernel);
        *exact.exact_branch_head_mut_for_test() = digest(b"wrong-exact-append-head");
        assert_eq!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(exact, &kernel,)
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ExactBranchHeadMismatch
        );

        let mut aggregate = two_event_closed(&kernel);
        *aggregate.aggregate_declaration_material_mut_for_test() = 7;
        assert_eq!(
            failure(
                verify_complete_target_neutral_generative_history_through_head_v1(
                    aggregate, &kernel,
                )
            ),
            CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::AggregateDeclarationMaterialMismatch
        );
    }

    #[test]
    fn independently_supplied_kernel_configuration_must_match_owned_protocol() {
        let source = kernel();
        let defaults = KernelLimits::default();
        let drifts = [
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
        for drift in drifts {
            let closed = two_event_closed(&source);
            let drifted = Kernel::new(drift).expect("valid drifted kernel");
            assert_eq!(
                failure(
                    verify_complete_target_neutral_generative_history_through_head_v1(
                        closed, &drifted,
                    )
                ),
                CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1::ProtocolKernelConfigurationMismatch
            );
        }
    }

    #[test]
    fn compatible_parallel_branches_both_verify_with_distinct_exact_final_and_complete_bindings() {
        let kernel = kernel();
        let left = verify_complete_target_neutral_generative_history_through_head_v1(
            two_event_closed(&kernel),
            &kernel,
        )
        .expect("complete left branch");
        let right = verify_complete_target_neutral_generative_history_through_head_v1(
            two_event_closed(&kernel),
            &kernel,
        )
        .expect("complete right branch");

        assert_eq!(
            left.normalized_log_head_digest(),
            right.normalized_log_head_digest()
        );
        assert_eq!(
            left.normalized_chain_digest(),
            right.normalized_chain_digest()
        );
        assert_ne!(
            left.exact_branch_head_digest(),
            right.exact_branch_head_digest()
        );
        assert_ne!(
            left.producer_finalized_head_commitment_digest(),
            right.producer_finalized_head_commitment_digest()
        );
        assert_ne!(
            left.complete_through_head_commitment_digest(),
            right.complete_through_head_commitment_digest()
        );
    }

    #[test]
    fn split_and_coalesced_owned_histories_both_verify_but_remain_distinct() {
        let kernel = kernel();
        let split_closed = singleton_event_closed(&kernel, &[b"segment-a", b"segment-b"]);

        let (protocol, jg1, constructors) = authorities(&kernel);
        let mut coalesced_open =
            begin_generative_sealed_log_v1(&protocol, &jg1, &constructors, &kernel)
                .expect("open coalesced producer");
        append(
            &mut coalesced_open,
            &kernel,
            &constructors,
            &candidate(&[b"segment-a", b"segment-b"]),
            b"coalesced-scope",
        )
        .expect("coalesced append");
        let coalesced_closed = close_generative_sealed_log_v1(coalesced_open);

        let split = verify_complete_target_neutral_generative_history_through_head_v1(
            split_closed,
            &kernel,
        )
        .expect("complete split history");
        let coalesced = verify_complete_target_neutral_generative_history_through_head_v1(
            coalesced_closed,
            &kernel,
        )
        .expect("complete coalesced history");

        assert_eq!(
            split.terminal_boundary().declarations(),
            coalesced.terminal_boundary().declarations()
        );
        assert_ne!(split.event_count(), coalesced.event_count());
        assert_ne!(
            split.normalized_log_head_digest(),
            coalesced.normalized_log_head_digest()
        );
        assert_ne!(
            split.normalized_chain_digest(),
            coalesced.normalized_chain_digest()
        );
        assert_ne!(
            split.complete_through_head_commitment_digest(),
            coalesced.complete_through_head_commitment_digest()
        );
    }

    #[test]
    fn complete_through_head_codec_fixture_is_deterministic() {
        let kernel = kernel();
        let (protocol, jg1, constructors) = authorities(&kernel);
        let mut open = begin_generative_sealed_log_v1(&protocol, &jg1, &constructors, &kernel)
            .expect("open fixed branch");
        open.set_branch_ordinal_for_test(11);
        append(
            &mut open,
            &kernel,
            &constructors,
            &candidate(&[b"complete-codec-a", b"complete-codec-b"]),
            b"complete-codec-scope",
        )
        .expect("fixed append");
        let complete = verify_complete_target_neutral_generative_history_through_head_v1(
            close_generative_sealed_log_v1(open),
            &kernel,
        )
        .expect("fixed complete-through-head replay");
        assert_eq!(
            complete.complete_through_head_commitment_digest().as_str(),
            "blake3:9d50883ed51e02d9deb86b4aaf97432f8165f35474e9a06c55370456ba74d4e6"
        );
    }
}
