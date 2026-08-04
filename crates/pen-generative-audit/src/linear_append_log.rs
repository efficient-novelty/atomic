//! JG2b1b1 process-local linear append producer.
//!
//! One successful append derives one JG2a stage and one committed frame. A
//! consuming close ends only that writer epoch. The resulting token is not an
//! independently replayed snapshot, EOF, complete-history, or globally latest
//! branch authority; JG2b1b2 separately consumes and replays it.

use crate::{
    GenerativeCapabilityStageSurfaceFailureV1, VerifiedGenerativeCapabilityConstructorGrammarV1,
    VerifiedPreExposureGenerativeCapabilityGrammarV1,
    generative_capability_kernel_configuration_digest_v1,
    verify_generative_capability_stage_surface_v1,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Digest, Kernel, KernelError, TrustedScope,
    UncheckedSignature, VerifiedFreeSealing, VerifiedSignature,
};
use pen_sealed_history::{
    VerifiedTargetNeutralSealedLogProtocolV1,
    target_neutral_sealed_log_kernel_configuration_digest_v1,
};
use std::sync::atomic::{AtomicU64, Ordering};

pub const GENERATIVE_LINEAR_APPEND_LOG_SCHEMA_VERSION_V1: u16 = 1;
pub const GENERATIVE_LINEAR_APPEND_LOG_RESOURCE_POLICY_VERSION_V1: u16 = 1;

const NORMALIZED_LOG_GENESIS_DOMAIN_V1: &str =
    "law-v2/jg2b1b1/normalized-generative-log-genesis/v1";
const EXACT_LOG_GENESIS_DOMAIN_V1: &str = "law-v2/jg2b1b1/exact-generative-log-genesis/v1";
const NORMALIZED_APPEND_FRAME_DOMAIN_V1: &str = "law-v2/jg2b1b1/normalized-append-frame/v1";
const EXACT_APPEND_FRAME_DOMAIN_V1: &str = "law-v2/jg2b1b1/exact-append-frame/v1";
pub(crate) const NORMALIZED_HEAD_STEP_DOMAIN_V1: &str = "law-v2/jg2b1b1/normalized-head-step/v1";
pub(crate) const EXACT_HEAD_STEP_DOMAIN_V1: &str = "law-v2/jg2b1b1/exact-head-step/v1";
pub(crate) const NORMALIZED_RETAINED_EXTENSION_DOMAIN_V1: &str =
    "law-v2/jg2b1b1/normalized-retained-extension/v1";
const PRODUCER_FINALIZED_HEAD_COMMITMENT_DOMAIN_V1: &str =
    "law-v2/jg2b1b1/producer-finalized-writer-head/v1";

const NORMALIZED_LOG_GENESIS_ROOT_TAG_V1: u8 = 0xd1;
const EXACT_LOG_GENESIS_ROOT_TAG_V1: u8 = 0xd2;
const NORMALIZED_APPEND_FRAME_ROOT_TAG_V1: u8 = 0xd3;
const EXACT_APPEND_FRAME_ROOT_TAG_V1: u8 = 0xd4;
pub(crate) const NORMALIZED_HEAD_STEP_ROOT_TAG_V1: u8 = 0xd5;
pub(crate) const EXACT_HEAD_STEP_ROOT_TAG_V1: u8 = 0xd6;
const PRODUCER_FINALIZED_HEAD_COMMITMENT_ROOT_TAG_V1: u8 = 0xd7;

static NEXT_PROCESS_LOCAL_BRANCH_ORDINAL_V1: AtomicU64 = AtomicU64::new(1);

/// Opaque identity unique only among successfully allocated branches in this
/// process. It has no persistence, cross-process, or globally-latest meaning.
#[derive(Clone, Eq, PartialEq)]
pub struct ProcessLocalGenerativeBranchIdV1(u64);

pub(crate) fn encode_process_local_branch_id_v1(
    encoder: &mut CanonicalEncoder,
    branch_id: &ProcessLocalGenerativeBranchIdV1,
) {
    encoder.u64(branch_id.0);
}

/// Small immutable receipt for one successful append. It identifies the
/// committed frame but is not a finalization or history capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerativeSealedAppendCommitmentV1 {
    event_ordinal: u64,
    normalized_frame_digest: Digest,
    normalized_head_digest: Digest,
    exact_frame_digest: Digest,
    exact_branch_head_digest: Digest,
    successor_digest: Digest,
}

impl GenerativeSealedAppendCommitmentV1 {
    pub const fn event_ordinal(&self) -> u64 {
        self.event_ordinal
    }

    pub fn normalized_frame_digest(&self) -> &Digest {
        &self.normalized_frame_digest
    }

    pub fn normalized_head_digest(&self) -> &Digest {
        &self.normalized_head_digest
    }

    pub fn exact_frame_digest(&self) -> &Digest {
        &self.exact_frame_digest
    }

    /// Rolling exact append-state head after this event. It is not the
    /// producer-finalized head minted by consuming close.
    pub fn exact_branch_head_digest(&self) -> &Digest {
        &self.exact_branch_head_digest
    }

    pub fn successor_digest(&self) -> &Digest {
        &self.successor_digest
    }
}

/// Compact exact evidence retained for the consuming JG2b1b2 fresh replay.
/// Cumulative stage and sealing tokens are deliberately not retained.
#[allow(dead_code)]
pub(crate) struct RetainedGenerativeSealedFrameV1 {
    pub(crate) event_ordinal: u64,
    pub(crate) scope: TrustedScope,
    pub(crate) exact_candidate: UncheckedSignature,
    pub(crate) predecessor_digest: Digest,
    pub(crate) successor_digest: Digest,
    pub(crate) normalized_extension_digest: Digest,
    pub(crate) candidate_digest: Digest,
    pub(crate) sealing_subject_digest: Digest,
    pub(crate) exact_stage_manifest_digest: Digest,
    pub(crate) normalized_frame_digest: Digest,
    pub(crate) pre_normalized_head_digest: Digest,
    pub(crate) post_normalized_head_digest: Digest,
    pub(crate) exact_frame_digest: Digest,
    pub(crate) pre_exact_head_digest: Digest,
    pub(crate) post_exact_head_digest: Digest,
}

/// Linear process-local writer. It is intentionally neither `Clone` nor
/// serializable, and its fields are private.
pub struct OpenGenerativeSealedLogV1 {
    protocol: VerifiedTargetNeutralSealedLogProtocolV1,
    jg1: VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: Kernel,
    branch_id: ProcessLocalGenerativeBranchIdV1,
    current_boundary: VerifiedSignature,
    frames: Vec<RetainedGenerativeSealedFrameV1>,
    normalized_head_digest: Digest,
    exact_branch_head_digest: Digest,
    retained_extension_count: usize,
    aggregate_declaration_material: usize,
    max_stage_count: usize,
    max_declarations: usize,
    aggregate_declaration_material_limit: usize,
}

impl OpenGenerativeSealedLogV1 {
    pub fn branch_id(&self) -> &ProcessLocalGenerativeBranchIdV1 {
        &self.branch_id
    }

    pub fn event_count(&self) -> usize {
        self.frames.len()
    }

    pub fn current_boundary(&self) -> &VerifiedSignature {
        &self.current_boundary
    }

    pub fn normalized_head_digest(&self) -> &Digest {
        &self.normalized_head_digest
    }

    /// Current rolling exact append-state head. Open state is not finalized;
    /// only consuming close mints the distinct producer-finalized commitment.
    pub fn exact_branch_head_digest(&self) -> &Digest {
        &self.exact_branch_head_digest
    }

    #[cfg(test)]
    pub(crate) fn set_branch_ordinal_for_test(&mut self, ordinal: u64) {
        assert!(
            self.frames.is_empty(),
            "test branch identity must precede append"
        );
        self.branch_id = ProcessLocalGenerativeBranchIdV1(ordinal);
        self.exact_branch_head_digest = exact_log_genesis_digest_v1(
            &self.branch_id,
            &self.protocol,
            &self.jg1,
            &self.constructors,
            &self.kernel,
            &self.normalized_head_digest,
        );
    }
}

/// Consumed writer epoch and its commitments. This is not independently
/// replayed and therefore grants no EOF, complete-history, terminality, or
/// globally-latest authority. It remains non-cloneable so JG2b1b2 consumes
/// exactly one owned evidence bundle during independent replay.
///
/// ```compile_fail
/// use pen_generative_audit::ClosedGenerativeSealedLogV1;
///
/// fn require_clone<T: Clone>() {}
///
/// fn duplicate() {
///     require_clone::<ClosedGenerativeSealedLogV1>();
/// }
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::ClosedGenerativeSealedLogV1;
///
/// fn require_debug<T: std::fmt::Debug>() {}
///
/// fn disclose() {
///     require_debug::<ClosedGenerativeSealedLogV1>();
/// }
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::ClosedGenerativeSealedLogV1;
///
/// fn inspect_frames(closed: &ClosedGenerativeSealedLogV1) {
///     let _ = closed.frames();
/// }
/// ```
pub struct ClosedGenerativeSealedLogV1 {
    protocol: VerifiedTargetNeutralSealedLogProtocolV1,
    jg1: VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: VerifiedGenerativeCapabilityConstructorGrammarV1,
    branch_id: ProcessLocalGenerativeBranchIdV1,
    frames: Vec<RetainedGenerativeSealedFrameV1>,
    terminal_boundary: VerifiedSignature,
    normalized_log_head_digest: Digest,
    exact_branch_head_digest: Digest,
    producer_finalized_head_commitment_digest: Digest,
    retained_extension_count: u64,
    aggregate_declaration_material: u64,
}

/// Owned, crate-private handoff consumed exactly once by JG2b1b2. This is not
/// cloneable, formattable, serializable, or publicly constructible.
pub(crate) struct ClosedGenerativeSealedLogReplayPartsV1 {
    pub(crate) protocol: VerifiedTargetNeutralSealedLogProtocolV1,
    pub(crate) jg1: VerifiedPreExposureGenerativeCapabilityGrammarV1,
    pub(crate) constructors: VerifiedGenerativeCapabilityConstructorGrammarV1,
    pub(crate) branch_id: ProcessLocalGenerativeBranchIdV1,
    pub(crate) frames: Vec<RetainedGenerativeSealedFrameV1>,
    pub(crate) terminal_boundary: VerifiedSignature,
    pub(crate) normalized_log_head_digest: Digest,
    pub(crate) exact_branch_head_digest: Digest,
    pub(crate) producer_finalized_head_commitment_digest: Digest,
    pub(crate) retained_extension_count: u64,
    pub(crate) aggregate_declaration_material: u64,
}

impl ClosedGenerativeSealedLogV1 {
    pub fn branch_id(&self) -> &ProcessLocalGenerativeBranchIdV1 {
        &self.branch_id
    }

    pub fn event_count(&self) -> usize {
        self.frames.len()
    }

    pub fn terminal_boundary(&self) -> &VerifiedSignature {
        &self.terminal_boundary
    }

    /// Terminal rolling normalized append-state head. This is deliberately
    /// named separately from JG2b1a's reconstructed normalized history digest.
    pub fn normalized_log_head_digest(&self) -> &Digest {
        &self.normalized_log_head_digest
    }

    /// Terminal rolling exact append-state head committed inside the
    /// producer-finalized head. This digest alone is not a finalized head.
    pub fn exact_branch_head_digest(&self) -> &Digest {
        &self.exact_branch_head_digest
    }

    /// Distinct producer-designated finalized head, minted only by consuming
    /// the sole open writer. Only JG2b1b2 independent consuming replay may
    /// promote this commitment to complete-through-head authority.
    pub fn producer_finalized_head_commitment_digest(&self) -> &Digest {
        &self.producer_finalized_head_commitment_digest
    }

    pub const fn retained_extension_count(&self) -> u64 {
        self.retained_extension_count
    }

    pub const fn aggregate_declaration_material(&self) -> u64 {
        self.aggregate_declaration_material
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

    pub(crate) fn into_replay_parts(self) -> ClosedGenerativeSealedLogReplayPartsV1 {
        let ClosedGenerativeSealedLogV1 {
            protocol,
            jg1,
            constructors,
            branch_id,
            frames,
            terminal_boundary,
            normalized_log_head_digest,
            exact_branch_head_digest,
            producer_finalized_head_commitment_digest,
            retained_extension_count,
            aggregate_declaration_material,
        } = self;
        ClosedGenerativeSealedLogReplayPartsV1 {
            protocol,
            jg1,
            constructors,
            branch_id,
            frames,
            terminal_boundary,
            normalized_log_head_digest,
            exact_branch_head_digest,
            producer_finalized_head_commitment_digest,
            retained_extension_count,
            aggregate_declaration_material,
        }
    }

    #[cfg(test)]
    pub(crate) fn frames_mut_for_test(&mut self) -> &mut Vec<RetainedGenerativeSealedFrameV1> {
        &mut self.frames
    }

    #[cfg(test)]
    pub(crate) fn branch_id_mut_for_test(&mut self) -> &mut ProcessLocalGenerativeBranchIdV1 {
        &mut self.branch_id
    }

    #[cfg(test)]
    pub(crate) fn retained_extension_count_mut_for_test(&mut self) -> &mut u64 {
        &mut self.retained_extension_count
    }

    #[cfg(test)]
    pub(crate) fn aggregate_declaration_material_mut_for_test(&mut self) -> &mut u64 {
        &mut self.aggregate_declaration_material
    }

    #[cfg(test)]
    pub(crate) fn producer_finalized_head_mut_for_test(&mut self) -> &mut Digest {
        &mut self.producer_finalized_head_commitment_digest
    }

    #[cfg(test)]
    pub(crate) fn terminal_boundary_mut_for_test(&mut self) -> &mut VerifiedSignature {
        &mut self.terminal_boundary
    }

    #[cfg(test)]
    pub(crate) fn normalized_log_head_mut_for_test(&mut self) -> &mut Digest {
        &mut self.normalized_log_head_digest
    }

    #[cfg(test)]
    pub(crate) fn exact_branch_head_mut_for_test(&mut self) -> &mut Digest {
        &mut self.exact_branch_head_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GenerativeSealedLogProducerFailureV1 {
    ConstructorGrammarJg1Mismatch,
    ProtocolKernelMismatch,
    ProtocolNormalizerMismatch,
    ProtocolKernelConfigurationMismatch,
    BranchIdentityExhausted,
    InitialBoundaryKernelFailure(KernelError),
    StageCountLimitExceeded,
    RetainedExtensionLimitExceeded,
    AggregateDeclarationMaterialOverflow,
    AggregateDeclarationMaterialLimitExceeded,
    OutputAllocationFailure,
    StageVerification(GenerativeCapabilityStageSurfaceFailureV1),
    WireCountOverflow,
}

impl std::fmt::Display for GenerativeSealedLogProducerFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConstructorGrammarJg1Mismatch => formatter
                .write_str("the constructor grammar is not bound to the supplied JG1 grammar"),
            Self::ProtocolKernelMismatch => {
                formatter.write_str("the sealed-log protocol binds a different kernel")
            }
            Self::ProtocolNormalizerMismatch => {
                formatter.write_str("the sealed-log protocol binds a different normalizer")
            }
            Self::ProtocolKernelConfigurationMismatch => formatter.write_str(
                "the sealed-log protocol binds a different kernel resource configuration",
            ),
            Self::BranchIdentityExhausted => {
                formatter.write_str("the process-local branch identity counter is exhausted")
            }
            Self::InitialBoundaryKernelFailure(error) => {
                write!(
                    formatter,
                    "the kernel rejected the empty log boundary: {error}"
                )
            }
            Self::StageCountLimitExceeded => {
                formatter.write_str("the append would exceed the kernel depth event bound")
            }
            Self::RetainedExtensionLimitExceeded => formatter
                .write_str("the append would exceed the retained-extension declaration bound"),
            Self::AggregateDeclarationMaterialOverflow => {
                formatter.write_str("checked aggregate declaration-material arithmetic overflowed")
            }
            Self::AggregateDeclarationMaterialLimitExceeded => formatter.write_str(
                "the append would exceed the aggregate declaration-material resource bound",
            ),
            Self::OutputAllocationFailure => {
                formatter.write_str("bounded retained-frame allocation failed")
            }
            Self::StageVerification(error) => {
                write!(
                    formatter,
                    "the append failed exact JG2a verification: {error}"
                )
            }
            Self::WireCountOverflow => {
                formatter.write_str("a derived producer count does not fit the V1 wire format")
            }
        }
    }
}

impl std::error::Error for GenerativeSealedLogProducerFailureV1 {}

/// Begin a new process-local branch at the independently verified empty
/// boundary. Parallel calls intentionally allocate distinct exact identities.
pub fn begin_generative_sealed_log_v1(
    protocol: &VerifiedTargetNeutralSealedLogProtocolV1,
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
) -> Result<OpenGenerativeSealedLogV1, GenerativeSealedLogProducerFailureV1> {
    if constructors.jg1_manifest_digest() != jg1.manifest_digest() {
        return Err(GenerativeSealedLogProducerFailureV1::ConstructorGrammarJg1Mismatch);
    }
    if protocol.kernel_protocol_digest() != &kernel.kernel_protocol_digest() {
        return Err(GenerativeSealedLogProducerFailureV1::ProtocolKernelMismatch);
    }
    if protocol.normalizer_protocol_digest() != &kernel.normalizer_protocol_digest() {
        return Err(GenerativeSealedLogProducerFailureV1::ProtocolNormalizerMismatch);
    }
    if protocol.kernel_configuration_digest()
        != &target_neutral_sealed_log_kernel_configuration_digest_v1(kernel)
    {
        return Err(GenerativeSealedLogProducerFailureV1::ProtocolKernelConfigurationMismatch);
    }

    let limits = kernel.limits();
    let max_stage_count = usize::from(limits.max_depth);
    let max_declarations = usize::try_from(limits.max_operations)
        .map_err(|_| GenerativeSealedLogProducerFailureV1::AggregateDeclarationMaterialOverflow)?;
    let aggregate_factor = max_stage_count
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .ok_or(GenerativeSealedLogProducerFailureV1::AggregateDeclarationMaterialOverflow)?;
    let aggregate_declaration_material_limit = max_declarations
        .checked_mul(aggregate_factor)
        .ok_or(GenerativeSealedLogProducerFailureV1::AggregateDeclarationMaterialOverflow)?;
    let initial_boundary = kernel
        .verify_signature(&UncheckedSignature::default())
        .map_err(GenerativeSealedLogProducerFailureV1::InitialBoundaryKernelFailure)?;
    let branch_id = next_process_local_branch_id_v1()?;
    let normalized_head_digest =
        normalized_log_genesis_digest_v1(protocol, jg1, constructors, kernel, &initial_boundary);
    let exact_branch_head_digest = exact_log_genesis_digest_v1(
        &branch_id,
        protocol,
        jg1,
        constructors,
        kernel,
        &normalized_head_digest,
    );

    Ok(OpenGenerativeSealedLogV1 {
        protocol: protocol.clone(),
        jg1: jg1.clone(),
        constructors: constructors.clone(),
        kernel: kernel.clone(),
        branch_id,
        current_boundary: initial_boundary,
        frames: Vec::new(),
        normalized_head_digest,
        exact_branch_head_digest,
        retained_extension_count: 0,
        aggregate_declaration_material: 0,
        max_stage_count,
        max_declarations,
        aggregate_declaration_material_limit,
    })
}

/// Verify and append one event. All proof and resource work is completed
/// before authority-bearing writer state changes.
pub fn append_verified_generative_stage_v1(
    open: &mut OpenGenerativeSealedLogV1,
    scope: &TrustedScope,
    exact_candidate: &UncheckedSignature,
    sealing: &VerifiedFreeSealing,
) -> Result<GenerativeSealedAppendCommitmentV1, GenerativeSealedLogProducerFailureV1> {
    if open.frames.len() >= open.max_stage_count {
        return Err(GenerativeSealedLogProducerFailureV1::StageCountLimitExceeded);
    }
    let proposed_retained_count = open
        .retained_extension_count
        .checked_add(exact_candidate.declarations.len())
        .ok_or(GenerativeSealedLogProducerFailureV1::AggregateDeclarationMaterialOverflow)?;
    if proposed_retained_count > open.max_declarations {
        return Err(GenerativeSealedLogProducerFailureV1::RetainedExtensionLimitExceeded);
    }

    let stage = verify_generative_capability_stage_surface_v1(
        &open.jg1,
        &open.constructors,
        &open.kernel,
        scope,
        &open.current_boundary,
        exact_candidate,
        sealing,
    )
    .map_err(GenerativeSealedLogProducerFailureV1::StageVerification)?;

    let predecessor_count = open.current_boundary.declarations().len();
    let extension_count = stage.normalized_candidate().declarations.len();
    let successor_count = stage.successor_boundary().declarations().len();
    let event_material = predecessor_count
        .checked_add(extension_count)
        .and_then(|value| value.checked_add(successor_count))
        .ok_or(GenerativeSealedLogProducerFailureV1::AggregateDeclarationMaterialOverflow)?;
    let aggregate_declaration_material = open
        .aggregate_declaration_material
        .checked_add(event_material)
        .ok_or(GenerativeSealedLogProducerFailureV1::AggregateDeclarationMaterialOverflow)?;
    if aggregate_declaration_material > open.aggregate_declaration_material_limit {
        return Err(
            GenerativeSealedLogProducerFailureV1::AggregateDeclarationMaterialLimitExceeded,
        );
    }
    if extension_count != exact_candidate.declarations.len() {
        return Err(GenerativeSealedLogProducerFailureV1::RetainedExtensionLimitExceeded);
    }

    let event_ordinal = u64::try_from(open.frames.len())
        .map_err(|_| GenerativeSealedLogProducerFailureV1::WireCountOverflow)?;
    let normalized_frame_digest = normalized_append_frame_digest_v1(
        &open.protocol,
        &open.jg1,
        &open.constructors,
        &open.kernel,
        event_ordinal,
        &open.normalized_head_digest,
        stage.predecessor_boundary(),
        stage.normalized_candidate(),
        stage.successor_boundary(),
    )?;
    let normalized_head_digest = head_step_digest_v1(
        NORMALIZED_HEAD_STEP_DOMAIN_V1,
        NORMALIZED_HEAD_STEP_ROOT_TAG_V1,
        &open.normalized_head_digest,
        &normalized_frame_digest,
    );
    let exact_frame_digest = exact_append_frame_digest_v1(
        &open.branch_id,
        &open.protocol,
        &open.jg1,
        &open.constructors,
        &open.kernel,
        event_ordinal,
        &open.exact_branch_head_digest,
        &normalized_frame_digest,
        scope,
        exact_candidate,
        sealing,
        &stage,
    );
    let exact_branch_head_digest = head_step_digest_v1(
        EXACT_HEAD_STEP_DOMAIN_V1,
        EXACT_HEAD_STEP_ROOT_TAG_V1,
        &open.exact_branch_head_digest,
        &exact_frame_digest,
    );
    let normalized_extension_digest = Digest::of_canonical(
        NORMALIZED_RETAINED_EXTENSION_DOMAIN_V1,
        stage.normalized_candidate(),
    );
    let successor = stage.successor_boundary().clone();
    let receipt = GenerativeSealedAppendCommitmentV1 {
        event_ordinal,
        normalized_frame_digest: normalized_frame_digest.clone(),
        normalized_head_digest: normalized_head_digest.clone(),
        exact_frame_digest: exact_frame_digest.clone(),
        exact_branch_head_digest: exact_branch_head_digest.clone(),
        successor_digest: successor.digest().clone(),
    };
    let retained = RetainedGenerativeSealedFrameV1 {
        event_ordinal,
        scope: scope.clone(),
        exact_candidate: exact_candidate.clone(),
        predecessor_digest: stage.predecessor_digest().clone(),
        successor_digest: stage.successor_digest().clone(),
        normalized_extension_digest,
        candidate_digest: stage.candidate_digest().clone(),
        sealing_subject_digest: stage.sealing_subject_digest().clone(),
        exact_stage_manifest_digest: stage.manifest_digest().clone(),
        normalized_frame_digest,
        pre_normalized_head_digest: open.normalized_head_digest.clone(),
        post_normalized_head_digest: normalized_head_digest.clone(),
        exact_frame_digest,
        pre_exact_head_digest: open.exact_branch_head_digest.clone(),
        post_exact_head_digest: exact_branch_head_digest.clone(),
    };

    open.frames
        .try_reserve(1)
        .map_err(|_| GenerativeSealedLogProducerFailureV1::OutputAllocationFailure)?;
    open.frames.push(retained);
    open.current_boundary = successor;
    open.normalized_head_digest = normalized_head_digest;
    open.exact_branch_head_digest = exact_branch_head_digest;
    open.retained_extension_count = proposed_retained_count;
    open.aggregate_declaration_material = aggregate_declaration_material;
    Ok(receipt)
}

/// Consume a writer epoch. This operation performs no independent snapshot
/// replay and therefore does not discharge JG2b1b2.
pub fn close_generative_sealed_log_v1(
    open: OpenGenerativeSealedLogV1,
) -> ClosedGenerativeSealedLogV1 {
    // These casts are lossless by construction: frame count is bounded by a
    // u16 kernel depth, retained declarations by a u32 operation count, and
    // aggregate material by u32::MAX * (2 * u16::MAX + 1), which fits u64.
    // Keeping close infallible avoids consuming and dropping the sole writer
    // on a post-consumption conversion failure.
    let event_count = open.frames.len() as u64;
    let retained_extension_count = open.retained_extension_count as u64;
    let aggregate_declaration_material = open.aggregate_declaration_material as u64;
    let producer_finalized_head_commitment_digest =
        producer_finalized_head_commitment_from_parts_v1(
            &open.protocol,
            &open.jg1,
            &open.constructors,
            &open.kernel,
            &open.branch_id,
            event_count,
            &open.normalized_head_digest,
            &open.exact_branch_head_digest,
            &open.current_boundary,
            retained_extension_count,
            aggregate_declaration_material,
        );

    ClosedGenerativeSealedLogV1 {
        protocol: open.protocol,
        jg1: open.jg1,
        constructors: open.constructors,
        branch_id: open.branch_id,
        frames: open.frames,
        terminal_boundary: open.current_boundary,
        normalized_log_head_digest: open.normalized_head_digest,
        exact_branch_head_digest: open.exact_branch_head_digest,
        producer_finalized_head_commitment_digest,
        retained_extension_count,
        aggregate_declaration_material,
    }
}

fn next_process_local_branch_id_v1()
-> Result<ProcessLocalGenerativeBranchIdV1, GenerativeSealedLogProducerFailureV1> {
    NEXT_PROCESS_LOCAL_BRANCH_ORDINAL_V1
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |value| {
            value.checked_add(1)
        })
        .map(ProcessLocalGenerativeBranchIdV1)
        .map_err(|_| GenerativeSealedLogProducerFailureV1::BranchIdentityExhausted)
}

fn encode_common_protocol_binding_v1(
    encoder: &mut CanonicalEncoder,
    protocol: &VerifiedTargetNeutralSealedLogProtocolV1,
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
) {
    encoder.u16(GENERATIVE_LINEAR_APPEND_LOG_SCHEMA_VERSION_V1);
    encoder.u16(GENERATIVE_LINEAR_APPEND_LOG_RESOURCE_POLICY_VERSION_V1);
    protocol.manifest_digest().encode_canonical(encoder);
    jg1.manifest_digest().encode_canonical(encoder);
    constructors.manifest_digest().encode_canonical(encoder);
    constructors
        .scope_grammar_digest()
        .encode_canonical(encoder);
    kernel.kernel_protocol_digest().encode_canonical(encoder);
    kernel
        .normalizer_protocol_digest()
        .encode_canonical(encoder);
    generative_capability_kernel_configuration_digest_v1(kernel).encode_canonical(encoder);
    protocol
        .kernel_configuration_digest()
        .encode_canonical(encoder);
}

pub(crate) fn normalized_log_genesis_digest_v1(
    protocol: &VerifiedTargetNeutralSealedLogProtocolV1,
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
    empty_boundary: &VerifiedSignature,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(NORMALIZED_LOG_GENESIS_ROOT_TAG_V1);
    encode_common_protocol_binding_v1(&mut encoder, protocol, jg1, constructors, kernel);
    empty_boundary.digest().encode_canonical(&mut encoder);
    encoder.sequence(empty_boundary.declarations());
    Digest::of_domain_bytes(NORMALIZED_LOG_GENESIS_DOMAIN_V1, encoder.as_bytes())
}

pub(crate) fn exact_log_genesis_digest_v1(
    branch_id: &ProcessLocalGenerativeBranchIdV1,
    protocol: &VerifiedTargetNeutralSealedLogProtocolV1,
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
    normalized_genesis_digest: &Digest,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(EXACT_LOG_GENESIS_ROOT_TAG_V1);
    encode_common_protocol_binding_v1(&mut encoder, protocol, jg1, constructors, kernel);
    encode_process_local_branch_id_v1(&mut encoder, branch_id);
    normalized_genesis_digest.encode_canonical(&mut encoder);
    Digest::of_domain_bytes(EXACT_LOG_GENESIS_DOMAIN_V1, encoder.as_bytes())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn normalized_append_frame_digest_v1(
    protocol: &VerifiedTargetNeutralSealedLogProtocolV1,
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
    event_ordinal: u64,
    pre_head_digest: &Digest,
    predecessor: &VerifiedSignature,
    normalized_extension: &UncheckedSignature,
    successor: &VerifiedSignature,
) -> Result<Digest, GenerativeSealedLogProducerFailureV1> {
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(NORMALIZED_APPEND_FRAME_ROOT_TAG_V1);
    encode_common_protocol_binding_v1(&mut encoder, protocol, jg1, constructors, kernel);
    encoder.u64(event_ordinal);
    pre_head_digest.encode_canonical(&mut encoder);
    predecessor.digest().encode_canonical(&mut encoder);
    encoder.u64(
        u64::try_from(predecessor.declarations().len())
            .map_err(|_| GenerativeSealedLogProducerFailureV1::WireCountOverflow)?,
    );
    encoder.sequence(predecessor.declarations());
    encoder.u64(
        u64::try_from(normalized_extension.declarations.len())
            .map_err(|_| GenerativeSealedLogProducerFailureV1::WireCountOverflow)?,
    );
    normalized_extension.encode_canonical(&mut encoder);
    successor.digest().encode_canonical(&mut encoder);
    encoder.u64(
        u64::try_from(successor.declarations().len())
            .map_err(|_| GenerativeSealedLogProducerFailureV1::WireCountOverflow)?,
    );
    encoder.sequence(successor.declarations());
    Ok(Digest::of_domain_bytes(
        NORMALIZED_APPEND_FRAME_DOMAIN_V1,
        encoder.as_bytes(),
    ))
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn exact_append_frame_digest_v1(
    branch_id: &ProcessLocalGenerativeBranchIdV1,
    protocol: &VerifiedTargetNeutralSealedLogProtocolV1,
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
    event_ordinal: u64,
    pre_exact_head_digest: &Digest,
    normalized_frame_digest: &Digest,
    scope: &TrustedScope,
    exact_candidate: &UncheckedSignature,
    sealing: &VerifiedFreeSealing,
    stage: &crate::VerifiedGenerativeCapabilityStageSurfaceV1,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(EXACT_APPEND_FRAME_ROOT_TAG_V1);
    encode_common_protocol_binding_v1(&mut encoder, protocol, jg1, constructors, kernel);
    encode_process_local_branch_id_v1(&mut encoder, branch_id);
    encoder.u64(event_ordinal);
    pre_exact_head_digest.encode_canonical(&mut encoder);
    normalized_frame_digest.encode_canonical(&mut encoder);
    scope.binding().encode_canonical(&mut encoder);
    scope.digest().encode_canonical(&mut encoder);
    exact_candidate.encode_canonical(&mut encoder);
    stage.candidate_digest().encode_canonical(&mut encoder);
    sealing.scope_digest().encode_canonical(&mut encoder);
    sealing.subject_digest().encode_canonical(&mut encoder);
    sealing
        .sealed_signature()
        .digest()
        .encode_canonical(&mut encoder);
    encoder.sequence(sealing.sealed_signature().declarations());
    stage
        .sealing_subject_digest()
        .encode_canonical(&mut encoder);
    stage.manifest_digest().encode_canonical(&mut encoder);
    Digest::of_domain_bytes(EXACT_APPEND_FRAME_DOMAIN_V1, encoder.as_bytes())
}

pub(crate) fn head_step_digest_v1(
    domain: &str,
    root_tag: u8,
    pre_head_digest: &Digest,
    frame_digest: &Digest,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(root_tag);
    pre_head_digest.encode_canonical(&mut encoder);
    frame_digest.encode_canonical(&mut encoder);
    Digest::of_domain_bytes(domain, encoder.as_bytes())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn producer_finalized_head_commitment_from_parts_v1(
    protocol: &VerifiedTargetNeutralSealedLogProtocolV1,
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
    branch_id: &ProcessLocalGenerativeBranchIdV1,
    event_count: u64,
    normalized_head_digest: &Digest,
    exact_branch_head_digest: &Digest,
    terminal_boundary: &VerifiedSignature,
    retained_extension_count: u64,
    aggregate_declaration_material: u64,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.tag(PRODUCER_FINALIZED_HEAD_COMMITMENT_ROOT_TAG_V1);
    encode_common_protocol_binding_v1(&mut encoder, protocol, jg1, constructors, kernel);
    encode_process_local_branch_id_v1(&mut encoder, branch_id);
    encoder.u64(event_count);
    normalized_head_digest.encode_canonical(&mut encoder);
    exact_branch_head_digest.encode_canonical(&mut encoder);
    terminal_boundary.digest().encode_canonical(&mut encoder);
    encoder.sequence(terminal_boundary.declarations());
    encoder.u64(retained_extension_count);
    encoder.u64(aggregate_declaration_material);
    Digest::of_domain_bytes(
        PRODUCER_FINALIZED_HEAD_COMMITMENT_DOMAIN_V1,
        encoder.as_bytes(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        generative_capability_candidate_digest_v1,
        proposed_generative_capability_constructor_grammar_v1,
        proposed_pre_exposure_generative_capability_grammar_v1,
        verify_generative_capability_constructor_grammar_v1,
        verify_pre_exposure_generative_capability_grammar_v1,
    };
    use pen_kernel::{
        CertificateClaim, Declaration, GlobalId, KernelLimits, ScopeInputs, Term,
        UncheckedFreeSealingCertificate,
    };
    use pen_sealed_history::{
        proposed_target_neutral_sealed_log_protocol_v1,
        verify_target_neutral_sealed_log_protocol_v1,
    };

    fn digest(label: &[u8]) -> Digest {
        Digest::of_bytes(label)
    }

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("valid kernel")
    }

    fn authorities(
        kernel: &Kernel,
    ) -> (
        VerifiedTargetNeutralSealedLogProtocolV1,
        VerifiedPreExposureGenerativeCapabilityGrammarV1,
        VerifiedGenerativeCapabilityConstructorGrammarV1,
    ) {
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

    fn normal_candidate(label: &[u8]) -> UncheckedSignature {
        UncheckedSignature {
            declarations: vec![Declaration {
                id: GlobalId(digest(label)),
                ty: Term::UnitType,
                body: Some(Term::Unit),
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
                law_digest: digest(b"law"),
                grammar_digest: constructors.scope_grammar_digest().clone(),
                scheme_calculus_digest: digest(b"scheme"),
                blindness_contract_digest: digest(b"blindness"),
                bootstrap_contract_digest: digest(b"bootstrap"),
                history_digest: digest(scope_label),
                public_boundary_digest: predecessor.digest().clone(),
                derivation_basis_digest: digest(b"basis"),
                active_window_digest: digest(b"window"),
                candidate_digest: generative_capability_candidate_digest_v1(exact_candidate),
            },
        );
        let successor = kernel
            .verify_extension(predecessor, exact_candidate)
            .expect("valid exact candidate");
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

    fn append_candidate(
        open: &mut OpenGenerativeSealedLogV1,
        kernel: &Kernel,
        constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
        exact_candidate: &UncheckedSignature,
        scope_label: &[u8],
    ) -> GenerativeSealedAppendCommitmentV1 {
        let (scope, sealing) = scope_and_sealing(
            kernel,
            constructors,
            open.current_boundary(),
            exact_candidate,
            scope_label,
        );
        append_verified_generative_stage_v1(open, &scope, exact_candidate, &sealing)
            .expect("successful linear append")
    }

    fn begin(
        kernel: &Kernel,
        protocol: &VerifiedTargetNeutralSealedLogProtocolV1,
        jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
        constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    ) -> OpenGenerativeSealedLogV1 {
        begin_generative_sealed_log_v1(protocol, jg1, constructors, kernel)
            .expect("open process-local writer")
    }

    #[test]
    fn parallel_identical_branches_share_normalized_identity_and_resist_exact_branch_substitution()
    {
        let kernel = kernel();
        let (protocol, jg1, constructors) = authorities(&kernel);
        let mut left = begin(&kernel, &protocol, &jg1, &constructors);
        let mut right = begin(&kernel, &protocol, &jg1, &constructors);
        assert!(left.branch_id() != right.branch_id());
        assert_eq!(
            left.normalized_head_digest(),
            right.normalized_head_digest()
        );
        assert_ne!(
            left.exact_branch_head_digest(),
            right.exact_branch_head_digest()
        );

        let candidate = candidate(&[b"a"]);
        let left_receipt =
            append_candidate(&mut left, &kernel, &constructors, &candidate, b"same-scope");
        let right_receipt = append_candidate(
            &mut right,
            &kernel,
            &constructors,
            &candidate,
            b"same-scope",
        );
        assert_eq!(
            left_receipt.normalized_head_digest(),
            right_receipt.normalized_head_digest()
        );
        assert_ne!(
            left_receipt.exact_branch_head_digest(),
            right_receipt.exact_branch_head_digest()
        );

        let left = close_generative_sealed_log_v1(left);
        let right = close_generative_sealed_log_v1(right);
        assert_eq!(
            left.normalized_log_head_digest(),
            right.normalized_log_head_digest()
        );
        assert!(left.branch_id() != right.branch_id());
        assert_ne!(
            left.exact_branch_head_digest(),
            right.exact_branch_head_digest()
        );
        assert_ne!(
            left.producer_finalized_head_commitment_digest(),
            right.producer_finalized_head_commitment_digest()
        );
    }

    #[test]
    fn empty_close_is_an_out_of_band_finalization_not_an_event() {
        let kernel = kernel();
        let (protocol, jg1, constructors) = authorities(&kernel);
        let open = begin(&kernel, &protocol, &jg1, &constructors);
        let normalized_genesis = open.normalized_head_digest().clone();
        let exact_genesis = open.exact_branch_head_digest().clone();
        let empty_boundary = open.current_boundary().digest().clone();

        let closed = close_generative_sealed_log_v1(open);

        assert_eq!(closed.event_count(), 0);
        assert!(closed.terminal_boundary().declarations().is_empty());
        assert_eq!(closed.terminal_boundary().digest(), &empty_boundary);
        assert_eq!(closed.normalized_log_head_digest(), &normalized_genesis);
        assert_eq!(closed.exact_branch_head_digest(), &exact_genesis);
        assert_ne!(
            closed.producer_finalized_head_commitment_digest(),
            &exact_genesis,
            "consuming close is a distinct control commitment, not event zero"
        );
    }

    #[test]
    fn successful_appends_derive_adjacency_ordinals_and_consuming_close_boundary() {
        let kernel = kernel();
        let (protocol, jg1, constructors) = authorities(&kernel);
        let mut open = begin(&kernel, &protocol, &jg1, &constructors);
        let first = append_candidate(
            &mut open,
            &kernel,
            &constructors,
            &candidate(&[b"a"]),
            b"first",
        );
        let second = append_candidate(
            &mut open,
            &kernel,
            &constructors,
            &candidate(&[b"b"]),
            b"second",
        );
        assert_eq!(first.event_ordinal(), 0);
        assert_eq!(second.event_ordinal(), 1);
        assert_eq!(open.event_count(), 2);
        assert_eq!(open.current_boundary().declarations().len(), 2);

        let terminal_digest = open.current_boundary().digest().clone();
        let closed = close_generative_sealed_log_v1(open);
        assert_eq!(closed.event_count(), 2);
        assert_eq!(closed.terminal_boundary().digest(), &terminal_digest);
        assert_eq!(closed.retained_extension_count(), 2);
    }

    #[test]
    fn failed_append_is_atomic_for_empty_candidate_and_stale_branch_evidence() {
        let kernel = kernel();
        let (protocol, jg1, constructors) = authorities(&kernel);
        let mut open = begin(&kernel, &protocol, &jg1, &constructors);

        let empty = UncheckedSignature::default();
        let (empty_scope, empty_sealing) = scope_and_sealing(
            &kernel,
            &constructors,
            open.current_boundary(),
            &empty,
            b"empty",
        );
        let before_normalized = open.normalized_head_digest().clone();
        let before_exact = open.exact_branch_head_digest().clone();
        let before_boundary = open.current_boundary().digest().clone();
        assert!(matches!(
            append_verified_generative_stage_v1(&mut open, &empty_scope, &empty, &empty_sealing),
            Err(GenerativeSealedLogProducerFailureV1::StageVerification(
                GenerativeCapabilityStageSurfaceFailureV1::EmptyCandidate
            ))
        ));
        assert_eq!(open.event_count(), 0);
        assert_eq!(open.normalized_head_digest(), &before_normalized);
        assert_eq!(open.exact_branch_head_digest(), &before_exact);
        assert_eq!(open.current_boundary().digest(), &before_boundary);

        let stale_candidate = candidate(&[b"stale"]);
        let (stale_scope, stale_sealing) = scope_and_sealing(
            &kernel,
            &constructors,
            open.current_boundary(),
            &stale_candidate,
            b"stale",
        );
        append_candidate(
            &mut open,
            &kernel,
            &constructors,
            &candidate(&[b"fresh"]),
            b"fresh",
        );
        let state = (
            open.event_count(),
            open.normalized_head_digest().clone(),
            open.exact_branch_head_digest().clone(),
            open.current_boundary().digest().clone(),
        );
        assert!(
            append_verified_generative_stage_v1(
                &mut open,
                &stale_scope,
                &stale_candidate,
                &stale_sealing
            )
            .is_err()
        );
        assert_eq!(open.event_count(), state.0);
        assert_eq!(open.normalized_head_digest(), &state.1);
        assert_eq!(open.exact_branch_head_digest(), &state.2);
        assert_eq!(open.current_boundary().digest(), &state.3);
    }

    #[test]
    fn normalization_and_scope_are_excluded_only_from_normalized_history() {
        let kernel = kernel();
        let (protocol, jg1, constructors) = authorities(&kernel);
        let open = begin(&kernel, &protocol, &jg1, &constructors);
        let redex = redex_candidate(b"n");
        let normal = normal_candidate(b"n");
        let (redex_scope, redex_sealing) = scope_and_sealing(
            &kernel,
            &constructors,
            open.current_boundary(),
            &redex,
            b"scope-redex",
        );
        let (normal_scope, normal_sealing) = scope_and_sealing(
            &kernel,
            &constructors,
            open.current_boundary(),
            &normal,
            b"scope-normal",
        );
        let redex_stage = verify_generative_capability_stage_surface_v1(
            &jg1,
            &constructors,
            &kernel,
            &redex_scope,
            open.current_boundary(),
            &redex,
            &redex_sealing,
        )
        .expect("redex stage");
        let normal_stage = verify_generative_capability_stage_surface_v1(
            &jg1,
            &constructors,
            &kernel,
            &normal_scope,
            open.current_boundary(),
            &normal,
            &normal_sealing,
        )
        .expect("normal stage");
        let redex_normalized_frame = normalized_append_frame_digest_v1(
            &protocol,
            &jg1,
            &constructors,
            &kernel,
            0,
            open.normalized_head_digest(),
            redex_stage.predecessor_boundary(),
            redex_stage.normalized_candidate(),
            redex_stage.successor_boundary(),
        )
        .expect("normalized redex frame");
        let normal_normalized_frame = normalized_append_frame_digest_v1(
            &protocol,
            &jg1,
            &constructors,
            &kernel,
            0,
            open.normalized_head_digest(),
            normal_stage.predecessor_boundary(),
            normal_stage.normalized_candidate(),
            normal_stage.successor_boundary(),
        )
        .expect("normalized normal frame");
        assert_eq!(redex_normalized_frame, normal_normalized_frame);

        let redex_exact_frame = exact_append_frame_digest_v1(
            open.branch_id(),
            &protocol,
            &jg1,
            &constructors,
            &kernel,
            0,
            open.exact_branch_head_digest(),
            &redex_normalized_frame,
            &redex_scope,
            &redex,
            &redex_sealing,
            &redex_stage,
        );
        let normal_exact_frame = exact_append_frame_digest_v1(
            open.branch_id(),
            &protocol,
            &jg1,
            &constructors,
            &kernel,
            0,
            open.exact_branch_head_digest(),
            &normal_normalized_frame,
            &normal_scope,
            &normal,
            &normal_sealing,
            &normal_stage,
        );
        assert_ne!(redex_exact_frame, normal_exact_frame);

        let (alternate_scope, alternate_sealing) = scope_and_sealing(
            &kernel,
            &constructors,
            open.current_boundary(),
            &normal,
            b"scope-alternate",
        );
        let alternate_stage = verify_generative_capability_stage_surface_v1(
            &jg1,
            &constructors,
            &kernel,
            &alternate_scope,
            open.current_boundary(),
            &normal,
            &alternate_sealing,
        )
        .expect("alternate-scope stage");
        let alternate_exact_frame = exact_append_frame_digest_v1(
            open.branch_id(),
            &protocol,
            &jg1,
            &constructors,
            &kernel,
            0,
            open.exact_branch_head_digest(),
            &normal_normalized_frame,
            &alternate_scope,
            &normal,
            &alternate_sealing,
            &alternate_stage,
        );
        assert_ne!(
            normal_exact_frame, alternate_exact_frame,
            "scope and verified sealing evidence belong only to exact identity"
        );
    }

    #[test]
    fn split_and_coalesced_commits_have_distinct_normalized_histories() {
        let kernel = kernel();
        let (protocol, jg1, constructors) = authorities(&kernel);
        let mut split = begin(&kernel, &protocol, &jg1, &constructors);
        append_candidate(
            &mut split,
            &kernel,
            &constructors,
            &candidate(&[b"a"]),
            b"split-a",
        );
        append_candidate(
            &mut split,
            &kernel,
            &constructors,
            &candidate(&[b"b"]),
            b"split-b",
        );
        let mut coalesced = begin(&kernel, &protocol, &jg1, &constructors);
        append_candidate(
            &mut coalesced,
            &kernel,
            &constructors,
            &candidate(&[b"a", b"b"]),
            b"coalesced",
        );

        assert_eq!(
            split.current_boundary().declarations(),
            coalesced.current_boundary().declarations()
        );
        assert_ne!(
            split.normalized_head_digest(),
            coalesced.normalized_head_digest()
        );
    }

    #[test]
    fn depth_and_retained_extension_limits_fail_without_closing_or_mutating() {
        let depth_kernel = Kernel::new(KernelLimits {
            max_depth: 1,
            ..KernelLimits::default()
        })
        .expect("valid shallow kernel");
        let (protocol, jg1, constructors) = authorities(&depth_kernel);
        let mut open = begin(&depth_kernel, &protocol, &jg1, &constructors);
        append_candidate(
            &mut open,
            &depth_kernel,
            &constructors,
            &candidate(&[b"a"]),
            b"first",
        );
        let state = (
            open.event_count(),
            open.normalized_head_digest().clone(),
            open.current_boundary().digest().clone(),
        );
        let extra = candidate(&[b"b"]);
        let (scope, sealing) = scope_and_sealing(
            &depth_kernel,
            &constructors,
            open.current_boundary(),
            &extra,
            b"extra",
        );
        assert_eq!(
            append_verified_generative_stage_v1(&mut open, &scope, &extra, &sealing),
            Err(GenerativeSealedLogProducerFailureV1::StageCountLimitExceeded)
        );
        assert_eq!(open.event_count(), state.0);
        assert_eq!(open.normalized_head_digest(), &state.1);
        assert_eq!(open.current_boundary().digest(), &state.2);

        let operation_kernel = Kernel::new(KernelLimits {
            max_operations: 1,
            ..KernelLimits::default()
        })
        .expect("valid operation-bounded kernel");
        let (protocol, jg1, constructors) = authorities(&operation_kernel);
        let mut bounded = begin(&operation_kernel, &protocol, &jg1, &constructors);
        let too_many = candidate(&[b"x", b"y"]);
        let dummy_kernel = kernel();
        let (_, dummy_jg1, dummy_constructors) = authorities(&dummy_kernel);
        let dummy_predecessor = dummy_kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty");
        let dummy_candidate = candidate(&[b"dummy"]);
        let (dummy_scope, dummy_sealing) = scope_and_sealing(
            &dummy_kernel,
            &dummy_constructors,
            &dummy_predecessor,
            &dummy_candidate,
            b"dummy",
        );
        let _ = dummy_jg1;
        assert_eq!(
            append_verified_generative_stage_v1(
                &mut bounded,
                &dummy_scope,
                &too_many,
                &dummy_sealing
            ),
            Err(GenerativeSealedLogProducerFailureV1::RetainedExtensionLimitExceeded)
        );
        assert_eq!(bounded.event_count(), 0);
    }

    #[test]
    fn protocol_configuration_drift_is_rejected_at_begin() {
        let source = kernel();
        let (protocol, jg1, constructors) = authorities(&source);
        let drifted = Kernel::new(KernelLimits {
            max_operations: KernelLimits::default().max_operations - 1,
            ..KernelLimits::default()
        })
        .expect("valid drifted kernel");
        assert!(matches!(
            begin_generative_sealed_log_v1(&protocol, &jg1, &constructors, &drifted),
            Err(GenerativeSealedLogProducerFailureV1::ProtocolKernelConfigurationMismatch)
        ));
    }

    #[test]
    fn exact_frame_and_producer_finalized_head_codec_are_pinned() {
        let kernel = kernel();
        let (protocol, jg1, constructors) = authorities(&kernel);
        let mut open = begin(&kernel, &protocol, &jg1, &constructors);
        open.branch_id = ProcessLocalGenerativeBranchIdV1(7);
        open.exact_branch_head_digest = exact_log_genesis_digest_v1(
            &open.branch_id,
            &protocol,
            &jg1,
            &constructors,
            &kernel,
            &open.normalized_head_digest,
        );
        let receipt = append_candidate(
            &mut open,
            &kernel,
            &constructors,
            &candidate(&[b"codec-a", b"codec-b"]),
            b"codec-scope",
        );
        let closed = close_generative_sealed_log_v1(open);
        assert_eq!(
            receipt.exact_frame_digest().as_str(),
            "blake3:07cca3ec7494d48a8266728fd038ad035c355b7d51c465fae559ad36712391b0"
        );
        assert_eq!(
            receipt.exact_branch_head_digest().as_str(),
            "blake3:d0dd23ffae1db8e04b4957b12d6efc44159a109b2a80a1129a7b61fc012c1897"
        );
        assert_eq!(
            closed.producer_finalized_head_commitment_digest().as_str(),
            "blake3:0716930c303c0cd4f6808ebca88081f35688e1b727c2306c9f90a1fa8548b155"
        );
    }

    #[test]
    fn closed_two_event_evidence_supports_non_authoritative_fresh_shadow_replay() {
        let kernel = kernel();
        let (protocol, jg1, constructors) = authorities(&kernel);
        let mut open = begin(&kernel, &protocol, &jg1, &constructors);
        append_candidate(
            &mut open,
            &kernel,
            &constructors,
            &redex_candidate(b"shadow-a"),
            b"shadow-scope-a",
        );
        append_candidate(
            &mut open,
            &kernel,
            &constructors,
            &candidate(&[b"shadow-b", b"shadow-c"]),
            b"shadow-scope-b",
        );
        let closed = close_generative_sealed_log_v1(open);

        // This deliberately remains a test-only comparison fixture. It mints
        // no replay capability and grants no JG2b1b2 completeness authority.
        let parts = closed.into_replay_parts();
        let replay_protocol = &parts.protocol;
        let replay_jg1 = &parts.jg1;
        let replay_constructors = &parts.constructors;
        let mut replayed_predecessor = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("fresh empty replay boundary");
        let mut replayed_normalized_head = normalized_log_genesis_digest_v1(
            replay_protocol,
            replay_jg1,
            replay_constructors,
            &kernel,
            &replayed_predecessor,
        );
        let mut replayed_exact_head = exact_log_genesis_digest_v1(
            &parts.branch_id,
            replay_protocol,
            replay_jg1,
            replay_constructors,
            &kernel,
            &replayed_normalized_head,
        );
        let mut replayed_retained_extension_count = 0_u64;
        let mut replayed_aggregate_declaration_material = 0_u64;

        for (index, retained) in parts.frames.iter().enumerate() {
            let event_ordinal = u64::try_from(index).expect("bounded replay ordinal");
            assert_eq!(retained.event_ordinal, event_ordinal);
            assert_eq!(retained.predecessor_digest, *replayed_predecessor.digest());
            assert_eq!(
                retained.pre_normalized_head_digest,
                replayed_normalized_head
            );
            assert_eq!(retained.pre_exact_head_digest, replayed_exact_head);

            let independently_replayed_successor = kernel
                .verify_extension(&replayed_predecessor, &retained.exact_candidate)
                .expect("fresh successor replay");
            let independently_replayed_subject = kernel
                .sealing_subject_digest(&replayed_predecessor, &retained.exact_candidate)
                .expect("fresh sealing subject");
            let reconstructed_certificate = UncheckedFreeSealingCertificate {
                binding: retained.scope.binding().clone(),
                subject_digest: independently_replayed_subject.clone(),
                extension: retained.exact_candidate.clone(),
                normalized_sealed_signature: independently_replayed_successor.normalized_wire(),
            };
            let replayed_sealing = kernel
                .verify_free_sealing_certificate(
                    &retained.scope,
                    &replayed_predecessor,
                    &retained.exact_candidate,
                    &reconstructed_certificate,
                )
                .expect("fresh sealing replay");
            assert_eq!(replayed_sealing.scope_digest(), retained.scope.digest());
            assert_eq!(
                replayed_sealing.subject_digest(),
                &retained.sealing_subject_digest
            );
            assert_eq!(
                replayed_sealing.sealed_signature().digest(),
                &retained.successor_digest
            );
            assert_eq!(
                replayed_sealing.sealed_signature().declarations(),
                independently_replayed_successor.declarations()
            );

            let replayed_stage = verify_generative_capability_stage_surface_v1(
                replay_jg1,
                replay_constructors,
                &kernel,
                &retained.scope,
                &replayed_predecessor,
                &retained.exact_candidate,
                &replayed_sealing,
            )
            .expect("fresh JG2a stage replay");
            assert_eq!(
                retained.predecessor_digest,
                *replayed_stage.predecessor_digest()
            );
            assert_eq!(
                retained.successor_digest,
                *replayed_stage.successor_digest()
            );
            assert_eq!(
                retained.candidate_digest,
                *replayed_stage.candidate_digest()
            );
            assert_eq!(
                retained.sealing_subject_digest,
                *replayed_stage.sealing_subject_digest()
            );
            assert_eq!(
                retained.exact_stage_manifest_digest,
                *replayed_stage.manifest_digest()
            );
            assert_eq!(
                retained.normalized_extension_digest,
                Digest::of_canonical(
                    "law-v2/jg2b1b1/normalized-retained-extension/v1",
                    replayed_stage.normalized_candidate(),
                )
            );

            let replayed_normalized_frame = normalized_append_frame_digest_v1(
                replay_protocol,
                replay_jg1,
                replay_constructors,
                &kernel,
                event_ordinal,
                &replayed_normalized_head,
                replayed_stage.predecessor_boundary(),
                replayed_stage.normalized_candidate(),
                replayed_stage.successor_boundary(),
            )
            .expect("fresh normalized frame replay");
            assert_eq!(retained.normalized_frame_digest, replayed_normalized_frame);
            let post_normalized_head = head_step_digest_v1(
                NORMALIZED_HEAD_STEP_DOMAIN_V1,
                NORMALIZED_HEAD_STEP_ROOT_TAG_V1,
                &replayed_normalized_head,
                &replayed_normalized_frame,
            );
            assert_eq!(retained.post_normalized_head_digest, post_normalized_head);

            let replayed_exact_frame = exact_append_frame_digest_v1(
                &parts.branch_id,
                replay_protocol,
                replay_jg1,
                replay_constructors,
                &kernel,
                event_ordinal,
                &replayed_exact_head,
                &replayed_normalized_frame,
                &retained.scope,
                &retained.exact_candidate,
                &replayed_sealing,
                &replayed_stage,
            );
            assert_eq!(retained.exact_frame_digest, replayed_exact_frame);
            let post_exact_head = head_step_digest_v1(
                EXACT_HEAD_STEP_DOMAIN_V1,
                EXACT_HEAD_STEP_ROOT_TAG_V1,
                &replayed_exact_head,
                &replayed_exact_frame,
            );
            assert_eq!(retained.post_exact_head_digest, post_exact_head);

            let predecessor_count = u64::try_from(replayed_predecessor.declarations().len())
                .expect("bounded predecessor count");
            let extension_count =
                u64::try_from(replayed_stage.normalized_candidate().declarations.len())
                    .expect("bounded extension count");
            let successor_count =
                u64::try_from(replayed_stage.successor_boundary().declarations().len())
                    .expect("bounded successor count");
            replayed_retained_extension_count = replayed_retained_extension_count
                .checked_add(extension_count)
                .expect("checked retained count");
            replayed_aggregate_declaration_material = replayed_aggregate_declaration_material
                .checked_add(predecessor_count)
                .and_then(|value| value.checked_add(extension_count))
                .and_then(|value| value.checked_add(successor_count))
                .expect("checked aggregate material");

            replayed_predecessor = replayed_stage.successor_boundary().clone();
            replayed_normalized_head = post_normalized_head;
            replayed_exact_head = post_exact_head;
        }

        assert_eq!(parts.frames.len(), 2);
        assert_eq!(
            parts.terminal_boundary.digest(),
            replayed_predecessor.digest()
        );
        assert_eq!(
            parts.terminal_boundary.declarations(),
            replayed_predecessor.declarations()
        );
        assert_eq!(&parts.normalized_log_head_digest, &replayed_normalized_head);
        assert_eq!(&parts.exact_branch_head_digest, &replayed_exact_head);
        assert_eq!(
            parts.retained_extension_count,
            replayed_retained_extension_count
        );
        assert_eq!(
            parts.aggregate_declaration_material,
            replayed_aggregate_declaration_material
        );

        let limits = kernel.limits();
        let aggregate_limit = u64::from(limits.max_operations)
            .checked_mul(
                u64::from(limits.max_depth)
                    .checked_mul(2)
                    .and_then(|value| value.checked_add(1))
                    .expect("checked aggregate factor"),
            )
            .expect("checked aggregate limit");
        assert!(
            u64::try_from(parts.frames.len()).expect("event count") <= u64::from(limits.max_depth)
        );
        assert!(parts.retained_extension_count <= u64::from(limits.max_operations));
        assert!(
            u64::try_from(parts.terminal_boundary.declarations().len()).expect("terminal count")
                <= u64::from(limits.max_operations)
        );
        assert!(parts.aggregate_declaration_material <= aggregate_limit);

        let mut finalized_head_encoder = CanonicalEncoder::new();
        finalized_head_encoder.tag(PRODUCER_FINALIZED_HEAD_COMMITMENT_ROOT_TAG_V1);
        encode_common_protocol_binding_v1(
            &mut finalized_head_encoder,
            replay_protocol,
            replay_jg1,
            replay_constructors,
            &kernel,
        );
        encode_process_local_branch_id_v1(&mut finalized_head_encoder, &parts.branch_id);
        finalized_head_encoder.u64(u64::try_from(parts.frames.len()).expect("event count"));
        replayed_normalized_head.encode_canonical(&mut finalized_head_encoder);
        replayed_exact_head.encode_canonical(&mut finalized_head_encoder);
        replayed_predecessor
            .digest()
            .encode_canonical(&mut finalized_head_encoder);
        finalized_head_encoder.sequence(replayed_predecessor.declarations());
        finalized_head_encoder.u64(replayed_retained_extension_count);
        finalized_head_encoder.u64(replayed_aggregate_declaration_material);
        let replayed_finalized_head = Digest::of_domain_bytes(
            PRODUCER_FINALIZED_HEAD_COMMITMENT_DOMAIN_V1,
            finalized_head_encoder.as_bytes(),
        );
        assert_eq!(
            &parts.producer_finalized_head_commitment_digest,
            &replayed_finalized_head
        );
    }
}
