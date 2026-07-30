//! Additive proof-carrying synthesis/code protocol V2.
//!
//! Serializable values in this module are authority-free proposals. Opaque
//! `Verified*` values are constructed only after independent reconstruction
//! and unchanged-kernel replay.

use super::{SynthesisError, synthesize_lambda_unit_v1};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Kernel, KernelError,
    OpenJudgment, Term, VerifiedContext, VerifiedSignature,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const SYNTHESIS_PROTOCOL_ID_V2: &str = "pen-kernel-synthesis/lambda-unit/v2";
pub const SYNTHESIS_SCHEMA_VERSION_V2: u16 = 2;
pub const PUBLIC_UNIVERSE_LEVELS_V2: [u16; 2] = [0, 1];
pub const CHECKER_OUTPUT_UNIVERSE_LEVELS_V2: [u16; 3] = [0, 1, 2];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransparentDeltaAuthorityFrontierV2 {
    MissingExactPredecessorPublicPolicyBinding,
}

pub const TRANSPARENT_DELTA_AUTHORITY_FRONTIER_V2: TransparentDeltaAuthorityFrontierV2 =
    TransparentDeltaAuthorityFrontierV2::MissingExactPredecessorPublicPolicyBinding;

/// Exact binder-local kernel replay for congruence premises remains outside
/// this V2 capability. V2 replays every outer trace intermediate, and it
/// reconstructs nested premises syntactically, but it does not claim the
/// stronger binder-local conversion-typing correspondence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NestedCongruenceReplayFrontierV2 {
    MissingBinderLocalKernelReplays,
}

pub const NESTED_CONGRUENCE_REPLAY_FRONTIER_V2: NestedCongruenceReplayFrontierV2 =
    NestedCongruenceReplayFrontierV2::MissingBinderLocalKernelReplays;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DeltaPolicyEntryV2 {
    pub global_slot: u32,
    pub id: GlobalId,
}

impl CanonicalEncode for DeltaPolicyEntryV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u32(self.global_slot);
        self.id.encode_canonical(encoder);
    }
}

/// Authority-free wire. A later semantic layer must prove that this exact
/// ordered table is the predecessor-public delta inventory.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BaseQ0ConversionPolicyV2 {
    pub allowed_transparent_deltas: Vec<DeltaPolicyEntryV2>,
}

impl CanonicalEncode for BaseQ0ConversionPolicyV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.allowed_transparent_deltas);
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedBaseQ0ConversionPolicyV2 {
    wire: BaseQ0ConversionPolicyV2,
    signature_digest: Digest,
    digest: Digest,
}

impl VerifiedBaseQ0ConversionPolicyV2 {
    pub fn wire(&self) -> &BaseQ0ConversionPolicyV2 {
        &self.wire
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    fn permits(&self, slot: u32, id: &GlobalId) -> bool {
        self.wire
            .allowed_transparent_deltas
            .binary_search_by_key(&slot, |entry| entry.global_slot)
            .ok()
            .is_some_and(|index| self.wire.allowed_transparent_deltas[index].id == *id)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NoRedexDispositionV2 {
    Sort,
    Variable,
    GlobalNotEnabledByPolicy,
    Pi,
    Lambda,
    NeutralApplication,
    UnitType,
    Unit,
}

impl CanonicalEncode for NoRedexDispositionV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Sort => 0,
            Self::Variable => 1,
            Self::GlobalNotEnabledByPolicy => 2,
            Self::Pi => 3,
            Self::Lambda => 4,
            Self::NeutralApplication => 5,
            Self::UnitType => 6,
            Self::Unit => 7,
        });
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConversionPathComponentV2 {
    PiParameter,
    PiBody,
    LambdaParameter,
    LambdaBody,
    ApplyFunction,
    ApplyArgument,
}

impl CanonicalEncode for ConversionPathComponentV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::PiParameter => 0,
            Self::PiBody => 1,
            Self::LambdaParameter => 2,
            Self::LambdaBody => 3,
            Self::ApplyFunction => 4,
            Self::ApplyArgument => 5,
        });
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NoRedexEntryV2 {
    pub path: Vec<ConversionPathComponentV2>,
    pub term: Term,
    pub disposition: NoRedexDispositionV2,
}

impl CanonicalEncode for NoRedexEntryV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.path);
        self.term.encode_canonical(encoder);
        self.disposition.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct NoRedexCensusCodeV2 {
    pub entries: Vec<NoRedexEntryV2>,
}

impl CanonicalEncode for NoRedexCensusCodeV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.entries);
    }
}

/// One directed base-Q0 step. Congruence constructors carry the exact child
/// proof and reconstruct both outer endpoints independently.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "rule", rename_all = "snake_case", deny_unknown_fields)]
pub enum BaseQ0ReductionStepCodeV2 {
    Beta {
        source: Term,
        target: Term,
    },
    TransparentDelta {
        source: Term,
        target: Term,
        global_slot: u32,
    },
    PiParameterCongruence {
        source: Term,
        target: Term,
        premise: Box<BaseQ0ReductionStepCodeV2>,
    },
    PiBodyCongruence {
        source: Term,
        target: Term,
        premise: Box<BaseQ0ReductionStepCodeV2>,
    },
    LambdaParameterCongruence {
        source: Term,
        target: Term,
        premise: Box<BaseQ0ReductionStepCodeV2>,
    },
    LambdaBodyCongruence {
        source: Term,
        target: Term,
        premise: Box<BaseQ0ReductionStepCodeV2>,
    },
    ApplyFunctionCongruence {
        source: Term,
        target: Term,
        premise: Box<BaseQ0ReductionStepCodeV2>,
    },
    ApplyArgumentCongruence {
        source: Term,
        target: Term,
        premise: Box<BaseQ0ReductionStepCodeV2>,
    },
}

impl BaseQ0ReductionStepCodeV2 {
    pub fn source(&self) -> &Term {
        match self {
            Self::Beta { source, .. }
            | Self::TransparentDelta { source, .. }
            | Self::PiParameterCongruence { source, .. }
            | Self::PiBodyCongruence { source, .. }
            | Self::LambdaParameterCongruence { source, .. }
            | Self::LambdaBodyCongruence { source, .. }
            | Self::ApplyFunctionCongruence { source, .. }
            | Self::ApplyArgumentCongruence { source, .. } => source,
        }
    }

    pub fn target(&self) -> &Term {
        match self {
            Self::Beta { target, .. }
            | Self::TransparentDelta { target, .. }
            | Self::PiParameterCongruence { target, .. }
            | Self::PiBodyCongruence { target, .. }
            | Self::LambdaParameterCongruence { target, .. }
            | Self::LambdaBodyCongruence { target, .. }
            | Self::ApplyFunctionCongruence { target, .. }
            | Self::ApplyArgumentCongruence { target, .. } => target,
        }
    }
}

impl CanonicalEncode for BaseQ0ReductionStepCodeV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Beta { source, target } => {
                encoder.tag(0);
                source.encode_canonical(encoder);
                target.encode_canonical(encoder);
            }
            Self::TransparentDelta {
                source,
                target,
                global_slot,
            } => {
                encoder.tag(1);
                source.encode_canonical(encoder);
                target.encode_canonical(encoder);
                encoder.u32(*global_slot);
            }
            Self::PiParameterCongruence {
                source,
                target,
                premise,
            } => encode_congruence(2, source, target, premise, encoder),
            Self::PiBodyCongruence {
                source,
                target,
                premise,
            } => encode_congruence(3, source, target, premise, encoder),
            Self::LambdaParameterCongruence {
                source,
                target,
                premise,
            } => encode_congruence(4, source, target, premise, encoder),
            Self::LambdaBodyCongruence {
                source,
                target,
                premise,
            } => encode_congruence(5, source, target, premise, encoder),
            Self::ApplyFunctionCongruence {
                source,
                target,
                premise,
            } => encode_congruence(6, source, target, premise, encoder),
            Self::ApplyArgumentCongruence {
                source,
                target,
                premise,
            } => encode_congruence(7, source, target, premise, encoder),
        }
    }
}

fn encode_congruence(
    tag: u8,
    source: &Term,
    target: &Term,
    premise: &BaseQ0ReductionStepCodeV2,
    encoder: &mut CanonicalEncoder,
) {
    encoder.tag(tag);
    source.encode_canonical(encoder);
    target.encode_canonical(encoder);
    premise.encode_canonical(encoder);
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BaseQ0ReductionTraceCodeV2 {
    pub start: Term,
    pub steps: Vec<BaseQ0ReductionStepCodeV2>,
    pub end: Term,
}

impl CanonicalEncode for BaseQ0ReductionTraceCodeV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.start.encode_canonical(encoder);
        encoder.sequence(&self.steps);
        self.end.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum BaseQ0EndpointJudgmentV2 {
    HasType { expected_type: Term },
    TypeFormation,
}

impl CanonicalEncode for BaseQ0EndpointJudgmentV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::HasType { expected_type } => {
                encoder.tag(0);
                expected_type.encode_canonical(encoder);
            }
            Self::TypeFormation => encoder.tag(1),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BaseQ0ConversionCodeV2 {
    pub context: DependentContext,
    pub left: Term,
    pub right: Term,
    pub endpoint_judgment: BaseQ0EndpointJudgmentV2,
    pub common_normal_form: Term,
    pub left_trace: BaseQ0ReductionTraceCodeV2,
    pub right_trace: BaseQ0ReductionTraceCodeV2,
    pub no_redex_census: NoRedexCensusCodeV2,
}

impl CanonicalEncode for BaseQ0ConversionCodeV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.context.encode_canonical(encoder);
        self.left.encode_canonical(encoder);
        self.right.encode_canonical(encoder);
        self.endpoint_judgment.encode_canonical(encoder);
        self.common_normal_form.encode_canonical(encoder);
        self.left_trace.encode_canonical(encoder);
        self.right_trace.encode_canonical(encoder);
        self.no_redex_census.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedKernelReplayPairV2 {
    input: OpenJudgment,
    output: OpenJudgment,
}

impl VerifiedKernelReplayPairV2 {
    pub fn input(&self) -> &OpenJudgment {
        &self.input
    }

    pub fn output(&self) -> &OpenJudgment {
        &self.output
    }
}

impl CanonicalEncode for VerifiedKernelReplayPairV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.input.encode_canonical(encoder);
        self.output.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedBaseQ0ConversionV2 {
    code: BaseQ0ConversionCodeV2,
    policy_digest: Digest,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    endpoint_replays: Vec<VerifiedKernelReplayPairV2>,
    no_redex_census_digest: Digest,
    digest: Digest,
}

impl VerifiedBaseQ0ConversionV2 {
    pub fn code(&self) -> &BaseQ0ConversionCodeV2 {
        &self.code
    }

    pub fn policy_digest(&self) -> &Digest {
        &self.policy_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn endpoint_replays(&self) -> &[VerifiedKernelReplayPairV2] {
        &self.endpoint_replays
    }

    pub fn no_redex_census_digest(&self) -> &Digest {
        &self.no_redex_census_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum SynthesisV2Error {
    #[error("unchanged kernel rejected replay: {0}")]
    Kernel(#[from] KernelError),
    #[error("V1 untrusted producer could not synthesize: {0}")]
    Producer(String),
    #[error("syntax is outside the lambda/unit fragment")]
    OutsideLambdaUnitFragment,
    #[error("public certificate contains forbidden Sort level {0}")]
    PublicUniverseLevel(u16),
    #[error("checker-produced output contains forbidden Sort level {0}")]
    CheckerOutputUniverseLevel(u16),
    #[error("operation or depth budget exhausted")]
    ResourceExhausted,
    #[error("delta policy slots must be strictly increasing and exact")]
    InvalidDeltaPolicyOrder,
    #[error("delta policy entry does not identify a bodyful verified declaration")]
    InvalidDeltaPolicyEntry,
    #[error("transparent delta is not enabled by the verified policy")]
    DeltaNotEnabled,
    #[error("reduction step endpoints or congruence premise are invalid")]
    InvalidReductionStep,
    #[error("reduction trace endpoints do not chain exactly")]
    InvalidReductionTrace,
    #[error("two terms do not reduce to one common base-Q0 normal form")]
    NotConvertible,
    #[error("claimed no-redex census is incomplete or incorrect")]
    InvalidNoRedexCensus,
    #[error("claimed normal form still has a base-Q0 redex")]
    NormalFormHasRedex,
    #[error("synthesis code does not reconstruct the required rule data")]
    InvalidSynthesisCode,
    #[error("a dependent product type was required")]
    ExpectedFunction,
    #[error("a universe was required")]
    ExpectedType,
    #[error("capture-safe shifting or substitution failed")]
    InvalidSubstitution,
    #[error("aggregate kernel replay disagreed with recorded outputs")]
    KernelReplayMismatch,
    #[error("complete-derivation aggregate replay rejected original inputs: {0}")]
    AggregateKernelReplay(KernelError),
}

impl From<SynthesisError> for SynthesisV2Error {
    fn from(error: SynthesisError) -> Self {
        Self::Producer(error.to_string())
    }
}

pub fn verify_base_q0_conversion_policy_v2(
    signature: &VerifiedSignature,
    wire: &BaseQ0ConversionPolicyV2,
) -> Result<VerifiedBaseQ0ConversionPolicyV2, SynthesisV2Error> {
    if wire.allowed_transparent_deltas.len() > signature.declarations().len() {
        return Err(SynthesisV2Error::InvalidDeltaPolicyEntry);
    }
    let mut previous = None;
    for entry in &wire.allowed_transparent_deltas {
        if previous.is_some_and(|slot| slot >= entry.global_slot) {
            return Err(SynthesisV2Error::InvalidDeltaPolicyOrder);
        }
        let slot = usize::try_from(entry.global_slot)
            .map_err(|_| SynthesisV2Error::InvalidDeltaPolicyEntry)?;
        let Some(declaration) = signature.declarations().get(slot) else {
            return Err(SynthesisV2Error::InvalidDeltaPolicyEntry);
        };
        if declaration.id != entry.id || declaration.body.is_none() {
            return Err(SynthesisV2Error::InvalidDeltaPolicyEntry);
        }
        ensure_checker_output_term(&declaration.ty)?;
        ensure_public_term(
            declaration
                .body
                .as_ref()
                .ok_or(SynthesisV2Error::InvalidDeltaPolicyEntry)?,
        )?;
        previous = Some(entry.global_slot);
    }
    let signature_digest = signature.digest().clone();
    let digest = Digest::of_canonical(
        "verified-base-q0-conversion-policy-v2",
        &PolicyDigestInput {
            signature_digest: &signature_digest,
            wire,
        },
    );
    Ok(VerifiedBaseQ0ConversionPolicyV2 {
        wire: wire.clone(),
        signature_digest,
        digest,
    })
}

pub fn propose_base_q0_conversion_code_v2(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    context: &DependentContext,
    left: &Term,
    right: &Term,
    expected_type: &Term,
) -> Result<BaseQ0ConversionCodeV2, SynthesisV2Error> {
    ensure_checker_output_term(expected_type)?;
    propose_base_q0_conversion_for_judgment_v2(
        kernel,
        signature,
        policy,
        context,
        left,
        right,
        BaseQ0EndpointJudgmentV2::HasType {
            expected_type: expected_type.clone(),
        },
    )
}

/// Propose conversion between types without serializing the kernel-internal
/// universe in which those types are formed. This keeps certificate syntax in
/// the checker-output closure `{0,1,2}` even when the unchanged kernel
/// internally forms a type in a higher universe.
pub fn propose_base_q0_type_conversion_code_v2(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    context: &DependentContext,
    left: &Term,
    right: &Term,
) -> Result<BaseQ0ConversionCodeV2, SynthesisV2Error> {
    propose_base_q0_conversion_for_judgment_v2(
        kernel,
        signature,
        policy,
        context,
        left,
        right,
        BaseQ0EndpointJudgmentV2::TypeFormation,
    )
}

fn propose_base_q0_conversion_for_judgment_v2(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    context: &DependentContext,
    left: &Term,
    right: &Term,
    endpoint_judgment: BaseQ0EndpointJudgmentV2,
) -> Result<BaseQ0ConversionCodeV2, SynthesisV2Error> {
    bind_policy_signature(signature, policy)?;
    ensure_public_context(context)?;
    ensure_checker_output_term(left)?;
    ensure_checker_output_term(right)?;
    let mut budget = V2Budget::new(kernel);
    let left_trace = generate_normalizing_trace(signature, policy, left, &mut budget, 0)?;
    let right_trace = generate_normalizing_trace(signature, policy, right, &mut budget, 0)?;
    if left_trace.end != right_trace.end {
        return Err(SynthesisV2Error::NotConvertible);
    }
    let common_normal_form = left_trace.end.clone();
    let entries =
        reconstruct_no_redex_census(signature, policy, &common_normal_form, &mut budget, 0)?;
    Ok(BaseQ0ConversionCodeV2 {
        context: context.clone(),
        left: left.clone(),
        right: right.clone(),
        endpoint_judgment,
        common_normal_form,
        left_trace,
        right_trace,
        no_redex_census: NoRedexCensusCodeV2 { entries },
    })
}

pub fn verify_base_q0_conversion_code_v2(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    code: &BaseQ0ConversionCodeV2,
) -> Result<VerifiedBaseQ0ConversionV2, SynthesisV2Error> {
    let mut budget = V2Budget::new(kernel);
    verify_conversion_with_budget(kernel, signature, policy, code, &mut budget, 0)
}

fn verify_conversion_with_budget(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    code: &BaseQ0ConversionCodeV2,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<VerifiedBaseQ0ConversionV2, SynthesisV2Error> {
    budget.enter(depth)?;
    bind_policy_signature(signature, policy)?;
    ensure_public_context(&code.context)?;
    ensure_checker_output_term(&code.left)?;
    ensure_checker_output_term(&code.right)?;
    ensure_checker_output_term(&code.common_normal_form)?;
    if let BaseQ0EndpointJudgmentV2::HasType { expected_type } = &code.endpoint_judgment {
        ensure_checker_output_term(expected_type)?;
    }
    if code.left_trace.start != code.left
        || code.left_trace.end != code.common_normal_form
        || code.right_trace.start != code.right
        || code.right_trace.end != code.common_normal_form
    {
        return Err(SynthesisV2Error::InvalidReductionTrace);
    }
    verify_trace(signature, policy, &code.left_trace, budget, depth)?;
    verify_trace(signature, policy, &code.right_trace, budget, depth)?;

    if code.no_redex_census.entries.len() > budget.initial_operations as usize {
        return Err(SynthesisV2Error::ResourceExhausted);
    }
    let expected_census =
        reconstruct_no_redex_census(signature, policy, &code.common_normal_form, budget, depth)?;
    if code.no_redex_census.entries != expected_census {
        return Err(SynthesisV2Error::InvalidNoRedexCensus);
    }

    let mut intermediates = vec![code.left_trace.start.clone()];
    intermediates.extend(
        code.left_trace
            .steps
            .iter()
            .map(|step| step.target().clone()),
    );
    intermediates.push(code.right_trace.start.clone());
    intermediates.extend(
        code.right_trace
            .steps
            .iter()
            .map(|step| step.target().clone()),
    );
    let inputs = intermediates
        .into_iter()
        .map(|term| match &code.endpoint_judgment {
            BaseQ0EndpointJudgmentV2::HasType { expected_type } => OpenJudgment::HasType {
                context: code.context.clone(),
                term,
                ty: expected_type.clone(),
            },
            BaseQ0EndpointJudgmentV2::TypeFormation => OpenJudgment::TypeFormation {
                context: code.context.clone(),
                term,
            },
        })
        .collect::<Vec<_>>();
    if inputs.len() > budget.operations_left as usize {
        return Err(SynthesisV2Error::ResourceExhausted);
    }
    for _ in &inputs {
        budget.enter(depth)?;
    }
    let input_refs = inputs.iter().collect::<Vec<_>>();
    let outputs = kernel.verify_open_judgments(signature, &input_refs)?;
    for output in &outputs {
        ensure_replay_output_closure(output)?;
    }
    if outputs.iter().any(|output| match output {
        OpenJudgment::HasType { term, .. } | OpenJudgment::TypeFormation { term, .. } => {
            term != &code.common_normal_form
        }
        OpenJudgment::DefinitionallyEqual { .. } => true,
    }) {
        return Err(SynthesisV2Error::KernelReplayMismatch);
    }
    let endpoint_replays = inputs
        .into_iter()
        .zip(outputs)
        .map(|(input, output)| VerifiedKernelReplayPairV2 { input, output })
        .collect::<Vec<_>>();
    let no_redex_census_digest =
        Digest::of_canonical("base-q0-no-redex-census-v2", &code.no_redex_census);
    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    let signature_digest = signature.digest().clone();
    let digest = Digest::of_canonical(
        "verified-base-q0-conversion-v2",
        &ConversionDigestInput {
            code,
            policy_digest: policy.digest(),
            signature_digest: &signature_digest,
            kernel_protocol_digest: &kernel_protocol_digest,
            endpoint_replays: &endpoint_replays,
            no_redex_census_digest: &no_redex_census_digest,
        },
    );
    Ok(VerifiedBaseQ0ConversionV2 {
        code: code.clone(),
        policy_digest: policy.digest().clone(),
        signature_digest,
        kernel_protocol_digest,
        endpoint_replays,
        no_redex_census_digest,
        digest,
    })
}

fn verify_trace(
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    trace: &BaseQ0ReductionTraceCodeV2,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<(), SynthesisV2Error> {
    ensure_checker_output_term(&trace.start)?;
    ensure_checker_output_term(&trace.end)?;
    if trace.steps.len() > budget.operations_left as usize {
        return Err(SynthesisV2Error::ResourceExhausted);
    }
    let mut cursor = &trace.start;
    for step in &trace.steps {
        budget.enter(depth)?;
        if step.source() != cursor {
            return Err(SynthesisV2Error::InvalidReductionTrace);
        }
        verify_step(signature, policy, step, budget, depth)?;
        cursor = step.target();
    }
    if cursor != &trace.end {
        return Err(SynthesisV2Error::InvalidReductionTrace);
    }
    Ok(())
}

fn verify_step(
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    step: &BaseQ0ReductionStepCodeV2,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<(), SynthesisV2Error> {
    budget.enter(depth)?;
    ensure_checker_output_term(step.source())?;
    ensure_checker_output_term(step.target())?;
    let child_depth = budget.child_depth(depth)?;
    match step {
        BaseQ0ReductionStepCodeV2::Beta { source, target } => {
            let Term::Apply { function, argument } = source else {
                return Err(SynthesisV2Error::InvalidReductionStep);
            };
            let Term::Lambda { body, .. } = function.as_ref() else {
                return Err(SynthesisV2Error::InvalidReductionStep);
            };
            if substitute_top_v2(body, argument, budget, child_depth)? != *target {
                return Err(SynthesisV2Error::InvalidReductionStep);
            }
        }
        BaseQ0ReductionStepCodeV2::TransparentDelta {
            source,
            target,
            global_slot,
        } => {
            let Term::Global { id } = source else {
                return Err(SynthesisV2Error::InvalidReductionStep);
            };
            if !policy.permits(*global_slot, id) {
                return Err(SynthesisV2Error::DeltaNotEnabled);
            }
            let slot = usize::try_from(*global_slot)
                .map_err(|_| SynthesisV2Error::InvalidReductionStep)?;
            let declaration = signature
                .declarations()
                .get(slot)
                .ok_or(SynthesisV2Error::InvalidReductionStep)?;
            if declaration.id != *id || declaration.body.as_ref() != Some(target) {
                return Err(SynthesisV2Error::InvalidReductionStep);
            }
        }
        BaseQ0ReductionStepCodeV2::PiParameterCongruence {
            source,
            target,
            premise,
        } => verify_binary_congruence(
            source,
            target,
            premise,
            BinaryConstructorV2::Pi,
            true,
            signature,
            policy,
            budget,
            child_depth,
        )?,
        BaseQ0ReductionStepCodeV2::PiBodyCongruence {
            source,
            target,
            premise,
        } => verify_binary_congruence(
            source,
            target,
            premise,
            BinaryConstructorV2::Pi,
            false,
            signature,
            policy,
            budget,
            child_depth,
        )?,
        BaseQ0ReductionStepCodeV2::LambdaParameterCongruence {
            source,
            target,
            premise,
        } => verify_binary_congruence(
            source,
            target,
            premise,
            BinaryConstructorV2::Lambda,
            true,
            signature,
            policy,
            budget,
            child_depth,
        )?,
        BaseQ0ReductionStepCodeV2::LambdaBodyCongruence {
            source,
            target,
            premise,
        } => verify_binary_congruence(
            source,
            target,
            premise,
            BinaryConstructorV2::Lambda,
            false,
            signature,
            policy,
            budget,
            child_depth,
        )?,
        BaseQ0ReductionStepCodeV2::ApplyFunctionCongruence {
            source,
            target,
            premise,
        } => verify_binary_congruence(
            source,
            target,
            premise,
            BinaryConstructorV2::Apply,
            true,
            signature,
            policy,
            budget,
            child_depth,
        )?,
        BaseQ0ReductionStepCodeV2::ApplyArgumentCongruence {
            source,
            target,
            premise,
        } => verify_binary_congruence(
            source,
            target,
            premise,
            BinaryConstructorV2::Apply,
            false,
            signature,
            policy,
            budget,
            child_depth,
        )?,
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum BinaryConstructorV2 {
    Pi,
    Lambda,
    Apply,
}

#[allow(clippy::too_many_arguments)]
fn verify_binary_congruence(
    source: &Term,
    target: &Term,
    premise: &BaseQ0ReductionStepCodeV2,
    constructor: BinaryConstructorV2,
    first_child: bool,
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<(), SynthesisV2Error> {
    let (source_first, source_second) = binary_children(source, constructor)?;
    let (target_first, target_second) = binary_children(target, constructor)?;
    let (changed_source, changed_target, fixed_source, fixed_target) = if first_child {
        (source_first, target_first, source_second, target_second)
    } else {
        (source_second, target_second, source_first, target_first)
    };
    if fixed_source != fixed_target
        || premise.source() != changed_source
        || premise.target() != changed_target
    {
        return Err(SynthesisV2Error::InvalidReductionStep);
    }
    verify_step(signature, policy, premise, budget, depth)
}

fn binary_children(
    term: &Term,
    constructor: BinaryConstructorV2,
) -> Result<(&Term, &Term), SynthesisV2Error> {
    match (constructor, term) {
        (BinaryConstructorV2::Pi, Term::Pi { parameter, body }) => Ok((parameter, body)),
        (
            BinaryConstructorV2::Lambda,
            Term::Lambda {
                parameter_type,
                body,
            },
        ) => Ok((parameter_type, body)),
        (BinaryConstructorV2::Apply, Term::Apply { function, argument }) => {
            Ok((function, argument))
        }
        _ => Err(SynthesisV2Error::InvalidReductionStep),
    }
}

fn generate_normalizing_trace(
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    start: &Term,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<BaseQ0ReductionTraceCodeV2, SynthesisV2Error> {
    ensure_checker_output_term(start)?;
    let mut cursor = start.clone();
    let mut steps = Vec::new();
    loop {
        budget.enter(depth)?;
        let Some(step) = first_reduction_step(signature, policy, &cursor, budget, depth)? else {
            break;
        };
        cursor = step.target().clone();
        steps.push(step);
        if steps.len() > budget.initial_operations as usize {
            return Err(SynthesisV2Error::ResourceExhausted);
        }
    }
    Ok(BaseQ0ReductionTraceCodeV2 {
        start: start.clone(),
        steps,
        end: cursor,
    })
}

fn first_reduction_step(
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    term: &Term,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<Option<BaseQ0ReductionStepCodeV2>, SynthesisV2Error> {
    budget.enter(depth)?;
    let child_depth = budget.child_depth(depth)?;
    if let Term::Apply { function, argument } = term
        && let Term::Lambda { body, .. } = function.as_ref()
    {
        return Ok(Some(BaseQ0ReductionStepCodeV2::Beta {
            source: term.clone(),
            target: substitute_top_v2(body, argument, budget, child_depth)?,
        }));
    }
    if let Term::Global { id } = term
        && let Some((slot, declaration)) = signature
            .declarations()
            .iter()
            .enumerate()
            .find(|(_, declaration)| declaration.id == *id)
        && let Some(body) = &declaration.body
    {
        let slot = u32::try_from(slot).map_err(|_| SynthesisV2Error::ResourceExhausted)?;
        if policy.permits(slot, id) {
            return Ok(Some(BaseQ0ReductionStepCodeV2::TransparentDelta {
                source: term.clone(),
                target: body.clone(),
                global_slot: slot,
            }));
        }
    }
    match term {
        Term::Pi { parameter, body } => {
            if let Some(premise) =
                first_reduction_step(signature, policy, parameter, budget, child_depth)?
            {
                return Ok(Some(BaseQ0ReductionStepCodeV2::PiParameterCongruence {
                    source: term.clone(),
                    target: Term::Pi {
                        parameter: Box::new(premise.target().clone()),
                        body: body.clone(),
                    },
                    premise: Box::new(premise),
                }));
            }
            if let Some(premise) =
                first_reduction_step(signature, policy, body, budget, child_depth)?
            {
                return Ok(Some(BaseQ0ReductionStepCodeV2::PiBodyCongruence {
                    source: term.clone(),
                    target: Term::Pi {
                        parameter: parameter.clone(),
                        body: Box::new(premise.target().clone()),
                    },
                    premise: Box::new(premise),
                }));
            }
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            if let Some(premise) =
                first_reduction_step(signature, policy, parameter_type, budget, child_depth)?
            {
                return Ok(Some(BaseQ0ReductionStepCodeV2::LambdaParameterCongruence {
                    source: term.clone(),
                    target: Term::Lambda {
                        parameter_type: Box::new(premise.target().clone()),
                        body: body.clone(),
                    },
                    premise: Box::new(premise),
                }));
            }
            if let Some(premise) =
                first_reduction_step(signature, policy, body, budget, child_depth)?
            {
                return Ok(Some(BaseQ0ReductionStepCodeV2::LambdaBodyCongruence {
                    source: term.clone(),
                    target: Term::Lambda {
                        parameter_type: parameter_type.clone(),
                        body: Box::new(premise.target().clone()),
                    },
                    premise: Box::new(premise),
                }));
            }
        }
        Term::Apply { function, argument } => {
            if let Some(premise) =
                first_reduction_step(signature, policy, function, budget, child_depth)?
            {
                return Ok(Some(BaseQ0ReductionStepCodeV2::ApplyFunctionCongruence {
                    source: term.clone(),
                    target: Term::Apply {
                        function: Box::new(premise.target().clone()),
                        argument: argument.clone(),
                    },
                    premise: Box::new(premise),
                }));
            }
            if let Some(premise) =
                first_reduction_step(signature, policy, argument, budget, child_depth)?
            {
                return Ok(Some(BaseQ0ReductionStepCodeV2::ApplyArgumentCongruence {
                    source: term.clone(),
                    target: Term::Apply {
                        function: function.clone(),
                        argument: Box::new(premise.target().clone()),
                    },
                    premise: Box::new(premise),
                }));
            }
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => {}
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            return Err(SynthesisV2Error::OutsideLambdaUnitFragment);
        }
    }
    Ok(None)
}

fn reconstruct_no_redex_census(
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    term: &Term,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<Vec<NoRedexEntryV2>, SynthesisV2Error> {
    let mut entries = Vec::new();
    census_term(
        signature,
        policy,
        term,
        &mut Vec::new(),
        &mut entries,
        budget,
        depth,
    )?;
    Ok(entries)
}

#[allow(clippy::too_many_arguments)]
fn census_term(
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    term: &Term,
    path: &mut Vec<ConversionPathComponentV2>,
    entries: &mut Vec<NoRedexEntryV2>,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<(), SynthesisV2Error> {
    budget.enter(depth)?;
    ensure_checker_output_term(term)?;
    let child_depth = budget.child_depth(depth)?;
    let disposition = match term {
        Term::Sort { .. } => NoRedexDispositionV2::Sort,
        Term::Var { .. } => NoRedexDispositionV2::Variable,
        Term::Global { id } => {
            if let Some((slot, declaration)) = signature
                .declarations()
                .iter()
                .enumerate()
                .find(|(_, declaration)| declaration.id == *id)
                && declaration.body.is_some()
                && policy.permits(
                    u32::try_from(slot).map_err(|_| SynthesisV2Error::ResourceExhausted)?,
                    id,
                )
            {
                return Err(SynthesisV2Error::NormalFormHasRedex);
            }
            NoRedexDispositionV2::GlobalNotEnabledByPolicy
        }
        Term::Pi { .. } => NoRedexDispositionV2::Pi,
        Term::Lambda { .. } => NoRedexDispositionV2::Lambda,
        Term::Apply { function, .. } => {
            if matches!(function.as_ref(), Term::Lambda { .. }) {
                return Err(SynthesisV2Error::NormalFormHasRedex);
            }
            NoRedexDispositionV2::NeutralApplication
        }
        Term::UnitType => NoRedexDispositionV2::UnitType,
        Term::Unit => NoRedexDispositionV2::Unit,
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            return Err(SynthesisV2Error::OutsideLambdaUnitFragment);
        }
    };
    entries.push(NoRedexEntryV2 {
        path: path.clone(),
        term: term.clone(),
        disposition,
    });
    match term {
        Term::Pi { parameter, body } => {
            path.push(ConversionPathComponentV2::PiParameter);
            census_term(
                signature,
                policy,
                parameter,
                path,
                entries,
                budget,
                child_depth,
            )?;
            path.pop();
            path.push(ConversionPathComponentV2::PiBody);
            census_term(signature, policy, body, path, entries, budget, child_depth)?;
            path.pop();
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            path.push(ConversionPathComponentV2::LambdaParameter);
            census_term(
                signature,
                policy,
                parameter_type,
                path,
                entries,
                budget,
                child_depth,
            )?;
            path.pop();
            path.push(ConversionPathComponentV2::LambdaBody);
            census_term(signature, policy, body, path, entries, budget, child_depth)?;
            path.pop();
        }
        Term::Apply { function, argument } => {
            path.push(ConversionPathComponentV2::ApplyFunction);
            census_term(
                signature,
                policy,
                function,
                path,
                entries,
                budget,
                child_depth,
            )?;
            path.pop();
            path.push(ConversionPathComponentV2::ApplyArgument);
            census_term(
                signature,
                policy,
                argument,
                path,
                entries,
                budget,
                child_depth,
            )?;
            path.pop();
        }
        _ => {}
    }
    Ok(())
}

/// Exact ordered inventory of the eight authority-free synthesis-code tags.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SynthesisCodeTagV2 {
    Sort,
    UnitType,
    Unit,
    VariableLookup,
    GlobalLookup,
    PiFormation,
    LambdaIntroduction,
    ApplicationElimination,
}

pub const SYNTHESIS_CODE_INVENTORY_V2: [SynthesisCodeTagV2; 8] = [
    SynthesisCodeTagV2::Sort,
    SynthesisCodeTagV2::UnitType,
    SynthesisCodeTagV2::Unit,
    SynthesisCodeTagV2::VariableLookup,
    SynthesisCodeTagV2::GlobalLookup,
    SynthesisCodeTagV2::PiFormation,
    SynthesisCodeTagV2::LambdaIntroduction,
    SynthesisCodeTagV2::ApplicationElimination,
];

impl SynthesisCodeTagV2 {
    pub fn canonical_name(self) -> &'static str {
        match self {
            Self::Sort => "sort",
            Self::UnitType => "unit_type",
            Self::Unit => "unit",
            Self::VariableLookup => "variable_lookup",
            Self::GlobalLookup => "global_lookup",
            Self::PiFormation => "pi_formation",
            Self::LambdaIntroduction => "lambda_introduction",
            Self::ApplicationElimination => "application_elimination",
        }
    }

    fn canonical_tag(self) -> u8 {
        match self {
            Self::Sort => 0,
            Self::UnitType => 1,
            Self::Unit => 2,
            Self::VariableLookup => 3,
            Self::GlobalLookup => 4,
            Self::PiFormation => 5,
            Self::LambdaIntroduction => 6,
            Self::ApplicationElimination => 7,
        }
    }
}

impl CanonicalEncode for SynthesisCodeTagV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(self.canonical_tag());
        encoder.text(self.canonical_name());
    }
}

/// Authority-free exact eight-constructor synthesis program.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "rule", rename_all = "snake_case", deny_unknown_fields)]
pub enum SynthesisCodeV2 {
    Sort {
        level: u16,
    },
    UnitType,
    Unit,
    VariableLookup {
        index: u32,
        context_ordinal: u32,
        shift_distance: u32,
    },
    GlobalLookup {
        id: GlobalId,
        global_slot: u32,
    },
    PiFormation {
        parameter: Box<SynthesisCodeV2>,
        body: Box<SynthesisCodeV2>,
    },
    LambdaIntroduction {
        parameter_type: Box<SynthesisCodeV2>,
        body: Box<SynthesisCodeV2>,
    },
    ApplicationElimination {
        function: Box<SynthesisCodeV2>,
        argument: Box<SynthesisCodeV2>,
        function_conversion: Box<BaseQ0ConversionCodeV2>,
        argument_conversion: Box<BaseQ0ConversionCodeV2>,
    },
}

impl SynthesisCodeV2 {
    pub fn inventory_tag(&self) -> SynthesisCodeTagV2 {
        match self {
            Self::Sort { .. } => SynthesisCodeTagV2::Sort,
            Self::UnitType => SynthesisCodeTagV2::UnitType,
            Self::Unit => SynthesisCodeTagV2::Unit,
            Self::VariableLookup { .. } => SynthesisCodeTagV2::VariableLookup,
            Self::GlobalLookup { .. } => SynthesisCodeTagV2::GlobalLookup,
            Self::PiFormation { .. } => SynthesisCodeTagV2::PiFormation,
            Self::LambdaIntroduction { .. } => SynthesisCodeTagV2::LambdaIntroduction,
            Self::ApplicationElimination { .. } => SynthesisCodeTagV2::ApplicationElimination,
        }
    }
}

impl CanonicalEncode for SynthesisCodeV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(self.inventory_tag().canonical_tag());
        match self {
            Self::Sort { level } => encoder.u16(*level),
            Self::UnitType | Self::Unit => {}
            Self::VariableLookup {
                index,
                context_ordinal,
                shift_distance,
            } => {
                encoder.u32(*index);
                encoder.u32(*context_ordinal);
                encoder.u32(*shift_distance);
            }
            Self::GlobalLookup { id, global_slot } => {
                id.encode_canonical(encoder);
                encoder.u32(*global_slot);
            }
            Self::PiFormation { parameter, body } => {
                parameter.encode_canonical(encoder);
                body.encode_canonical(encoder);
            }
            Self::LambdaIntroduction {
                parameter_type,
                body,
            } => {
                parameter_type.encode_canonical(encoder);
                body.encode_canonical(encoder);
            }
            Self::ApplicationElimination {
                function,
                argument,
                function_conversion,
                argument_conversion,
            } => {
                function.encode_canonical(encoder);
                argument.encode_canonical(encoder);
                function_conversion.encode_canonical(encoder);
                argument_conversion.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedSynthesisNodeV2 {
    code: SynthesisCodeV2,
    signature_digest: Digest,
    policy_digest: Digest,
    synthesis_protocol_digest: Digest,
    kernel_protocol_digest: Digest,
    context: DependentContext,
    term: Term,
    inferred_type: Term,
    premises: Vec<VerifiedSynthesisNodeV2>,
    conversions: Vec<VerifiedBaseQ0ConversionV2>,
    kernel_replays: Vec<VerifiedKernelReplayPairV2>,
    digest: Digest,
}

impl VerifiedSynthesisNodeV2 {
    pub fn code(&self) -> &SynthesisCodeV2 {
        &self.code
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn policy_digest(&self) -> &Digest {
        &self.policy_digest
    }

    pub fn synthesis_protocol_digest(&self) -> &Digest {
        &self.synthesis_protocol_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn context(&self) -> &DependentContext {
        &self.context
    }

    pub fn term(&self) -> &Term {
        &self.term
    }

    pub fn inferred_type(&self) -> &Term {
        &self.inferred_type
    }

    pub fn premises(&self) -> &[VerifiedSynthesisNodeV2] {
        &self.premises
    }

    pub fn conversions(&self) -> &[VerifiedBaseQ0ConversionV2] {
        &self.conversions
    }

    pub fn kernel_replays(&self) -> &[VerifiedKernelReplayPairV2] {
        &self.kernel_replays
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedSynthesisCodeV2 {
    schema_version: u16,
    code: SynthesisCodeV2,
    signature_digest: Digest,
    policy_digest: Digest,
    input_context: DependentContext,
    context_digest: Digest,
    term: Term,
    normalized_type: Term,
    derivation: VerifiedSynthesisNodeV2,
    synthesis_protocol_digest: Digest,
    kernel_protocol_digest: Digest,
    aggregate_kernel_replay_digest: Digest,
    digest: Digest,
}

impl VerifiedSynthesisCodeV2 {
    pub fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn code(&self) -> &SynthesisCodeV2 {
        &self.code
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn policy_digest(&self) -> &Digest {
        &self.policy_digest
    }

    pub fn input_context(&self) -> &DependentContext {
        &self.input_context
    }

    pub fn context_digest(&self) -> &Digest {
        &self.context_digest
    }

    pub fn term(&self) -> &Term {
        &self.term
    }

    pub fn normalized_type(&self) -> &Term {
        &self.normalized_type
    }

    pub fn derivation(&self) -> &VerifiedSynthesisNodeV2 {
        &self.derivation
    }

    pub fn synthesis_protocol_digest(&self) -> &Digest {
        &self.synthesis_protocol_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn aggregate_kernel_replay_digest(&self) -> &Digest {
        &self.aggregate_kernel_replay_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

pub fn propose_synthesis_code_v2(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    context: &DependentContext,
    term: &Term,
) -> Result<SynthesisCodeV2, SynthesisV2Error> {
    bind_policy_signature(signature, policy)?;
    ensure_public_context(context)?;
    ensure_public_term(term)?;
    propose_code_node(kernel, signature, policy, context, term)
}

pub fn verify_synthesis_code_v2(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    context: &DependentContext,
    code: &SynthesisCodeV2,
) -> Result<VerifiedSynthesisCodeV2, SynthesisV2Error> {
    bind_policy_signature(signature, policy)?;
    ensure_public_context(context)?;
    let verified_context = kernel.verify_context(signature, context)?;
    ensure_public_context(&verified_context.normalized_wire())?;
    if verified_context.normalized_wire() != *context {
        return Err(SynthesisV2Error::InvalidSynthesisCode);
    }
    let mut budget = V2Budget::new(kernel);
    let derivation = verify_code_node(
        kernel,
        signature,
        policy,
        &verified_context,
        code,
        &mut budget,
        0,
    )?;
    let mut replay_pairs = Vec::new();
    collect_node_replays(&derivation, &mut replay_pairs);
    let inputs = replay_pairs
        .iter()
        .map(VerifiedKernelReplayPairV2::input)
        .collect::<Vec<_>>();
    let outputs = kernel
        .verify_open_judgments(signature, &inputs)
        .map_err(SynthesisV2Error::AggregateKernelReplay)?;
    for output in &outputs {
        ensure_replay_output_closure(output)?;
    }
    if outputs
        .iter()
        .zip(&replay_pairs)
        .any(|(output, recorded)| output != recorded.output())
    {
        return Err(SynthesisV2Error::KernelReplayMismatch);
    }
    let aggregate_kernel_replay_digest = Digest::of_canonical(
        "synthesis-code-v2-aggregate-kernel-replay",
        &CanonicalReplayPairs(&replay_pairs),
    );
    let synthesis_protocol_digest = synthesis_protocol_digest_v2();
    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    let context_digest = verified_context.digest().clone();
    let signature_digest = signature.digest().clone();
    let normalized_type = derivation.inferred_type.clone();
    let term = derivation.term.clone();
    let digest = Digest::of_canonical(
        "verified-synthesis-code-v2",
        &SynthesisCertificateDigestInput {
            schema_version: SYNTHESIS_SCHEMA_VERSION_V2,
            code,
            signature_digest: &signature_digest,
            policy_digest: policy.digest(),
            input_context: context,
            context_digest: &context_digest,
            term: &term,
            normalized_type: &normalized_type,
            derivation_digest: derivation.digest(),
            synthesis_protocol_digest: &synthesis_protocol_digest,
            kernel_protocol_digest: &kernel_protocol_digest,
            aggregate_kernel_replay_digest: &aggregate_kernel_replay_digest,
        },
    );
    Ok(VerifiedSynthesisCodeV2 {
        schema_version: SYNTHESIS_SCHEMA_VERSION_V2,
        code: code.clone(),
        signature_digest,
        policy_digest: policy.digest().clone(),
        input_context: context.clone(),
        context_digest,
        term,
        normalized_type,
        derivation,
        synthesis_protocol_digest,
        kernel_protocol_digest,
        aggregate_kernel_replay_digest,
        digest,
    })
}

fn propose_code_node(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    context: &DependentContext,
    term: &Term,
) -> Result<SynthesisCodeV2, SynthesisV2Error> {
    match term {
        Term::Sort { level } => {
            if !PUBLIC_UNIVERSE_LEVELS_V2.contains(level) {
                return Err(SynthesisV2Error::PublicUniverseLevel(*level));
            }
            Ok(SynthesisCodeV2::Sort { level: *level })
        }
        Term::UnitType => Ok(SynthesisCodeV2::UnitType),
        Term::Unit => Ok(SynthesisCodeV2::Unit),
        Term::Var { index } => {
            let distance = index
                .checked_add(1)
                .ok_or(SynthesisV2Error::InvalidSynthesisCode)?;
            let ordinal = context
                .0
                .len()
                .checked_sub(
                    usize::try_from(distance)
                        .map_err(|_| SynthesisV2Error::InvalidSynthesisCode)?,
                )
                .ok_or(SynthesisV2Error::InvalidSynthesisCode)?;
            Ok(SynthesisCodeV2::VariableLookup {
                index: *index,
                context_ordinal: u32::try_from(ordinal)
                    .map_err(|_| SynthesisV2Error::ResourceExhausted)?,
                shift_distance: distance,
            })
        }
        Term::Global { id } => {
            let slot = signature
                .declarations()
                .iter()
                .position(|declaration| declaration.id == *id)
                .ok_or(SynthesisV2Error::InvalidSynthesisCode)?;
            Ok(SynthesisCodeV2::GlobalLookup {
                id: id.clone(),
                global_slot: u32::try_from(slot)
                    .map_err(|_| SynthesisV2Error::ResourceExhausted)?,
            })
        }
        Term::Pi { parameter, body } => {
            let parameter_code = propose_code_node(kernel, signature, policy, context, parameter)?;
            let normal_parameter = normalize_type_term(kernel, signature, context, parameter)?;
            let mut extended = context.0.clone();
            extended.push(normal_parameter);
            let extended = kernel
                .verify_context(signature, &DependentContext(extended))?
                .normalized_wire();
            let body_code = propose_code_node(kernel, signature, policy, &extended, body)?;
            Ok(SynthesisCodeV2::PiFormation {
                parameter: Box::new(parameter_code),
                body: Box::new(body_code),
            })
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            let parameter_code =
                propose_code_node(kernel, signature, policy, context, parameter_type)?;
            let normal_parameter = normalize_type_term(kernel, signature, context, parameter_type)?;
            let mut extended = context.0.clone();
            extended.push(normal_parameter);
            let extended = kernel
                .verify_context(signature, &DependentContext(extended))?
                .normalized_wire();
            let body_code = propose_code_node(kernel, signature, policy, &extended, body)?;
            Ok(SynthesisCodeV2::LambdaIntroduction {
                parameter_type: Box::new(parameter_code),
                body: Box::new(body_code),
            })
        }
        Term::Apply { function, argument } => {
            let function_code = propose_code_node(kernel, signature, policy, context, function)?;
            let argument_code = propose_code_node(kernel, signature, policy, context, argument)?;
            let function_proof = synthesize_lambda_unit_v1(kernel, signature, context, function)?;
            let function_type = function_proof.normalized_type().clone();
            let preliminary = propose_base_q0_type_conversion_code_v2(
                kernel,
                signature,
                policy,
                context,
                &function_type,
                &function_type,
            )?;
            let normal_function_type = preliminary.common_normal_form;
            let Term::Pi { parameter, .. } = &normal_function_type else {
                return Err(SynthesisV2Error::ExpectedFunction);
            };
            let function_conversion = propose_base_q0_type_conversion_code_v2(
                kernel,
                signature,
                policy,
                context,
                &function_type,
                &normal_function_type,
            )?;
            let argument_type = synthesize_lambda_unit_v1(kernel, signature, context, argument)?
                .normalized_type()
                .clone();
            let argument_conversion = propose_base_q0_type_conversion_code_v2(
                kernel,
                signature,
                policy,
                context,
                &argument_type,
                parameter,
            )?;
            Ok(SynthesisCodeV2::ApplicationElimination {
                function: Box::new(function_code),
                argument: Box::new(argument_code),
                function_conversion: Box::new(function_conversion),
                argument_conversion: Box::new(argument_conversion),
            })
        }
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            Err(SynthesisV2Error::OutsideLambdaUnitFragment)
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn verify_code_node(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    context: &VerifiedContext,
    code: &SynthesisCodeV2,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<VerifiedSynthesisNodeV2, SynthesisV2Error> {
    budget.enter(depth)?;
    let child_depth = budget.child_depth(depth)?;
    match code {
        SynthesisCodeV2::Sort { level } => {
            if !PUBLIC_UNIVERSE_LEVELS_V2.contains(level) {
                return Err(SynthesisV2Error::PublicUniverseLevel(*level));
            }
            finalize_synthesis_node(
                kernel,
                signature,
                policy,
                context,
                code,
                Term::Sort { level: *level },
                Term::Sort {
                    level: level
                        .checked_add(1)
                        .ok_or(SynthesisV2Error::CheckerOutputUniverseLevel(u16::MAX))?,
                },
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )
        }
        SynthesisCodeV2::UnitType => finalize_synthesis_node(
            kernel,
            signature,
            policy,
            context,
            code,
            Term::UnitType,
            Term::Sort { level: 0 },
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        SynthesisCodeV2::Unit => finalize_synthesis_node(
            kernel,
            signature,
            policy,
            context,
            code,
            Term::Unit,
            Term::UnitType,
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ),
        SynthesisCodeV2::VariableLookup {
            index,
            context_ordinal,
            shift_distance,
        } => {
            let expected_distance = index
                .checked_add(1)
                .ok_or(SynthesisV2Error::InvalidSynthesisCode)?;
            let ordinal = context
                .entries()
                .len()
                .checked_sub(
                    usize::try_from(expected_distance)
                        .map_err(|_| SynthesisV2Error::InvalidSynthesisCode)?,
                )
                .ok_or(SynthesisV2Error::InvalidSynthesisCode)?;
            if *shift_distance != expected_distance
                || usize::try_from(*context_ordinal).ok() != Some(ordinal)
            {
                return Err(SynthesisV2Error::InvalidSynthesisCode);
            }
            let inferred = shift_v2(
                &context.entries()[ordinal],
                i64::from(expected_distance),
                0,
                budget,
                child_depth,
            )?;
            finalize_synthesis_node(
                kernel,
                signature,
                policy,
                context,
                code,
                Term::Var { index: *index },
                inferred,
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )
        }
        SynthesisCodeV2::GlobalLookup { id, global_slot } => {
            let slot = usize::try_from(*global_slot)
                .map_err(|_| SynthesisV2Error::InvalidSynthesisCode)?;
            let declaration = signature
                .declarations()
                .get(slot)
                .ok_or(SynthesisV2Error::InvalidSynthesisCode)?;
            if declaration.id != *id {
                return Err(SynthesisV2Error::InvalidSynthesisCode);
            }
            ensure_checker_output_term(&declaration.ty)?;
            finalize_synthesis_node(
                kernel,
                signature,
                policy,
                context,
                code,
                Term::Global { id: id.clone() },
                declaration.ty.clone(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )
        }
        SynthesisCodeV2::PiFormation { parameter, body } => {
            let parameter_node = verify_code_node(
                kernel,
                signature,
                policy,
                context,
                parameter,
                budget,
                child_depth,
            )?;
            let parameter_level = expect_sort_v2(parameter_node.inferred_type())?;
            let (normal_parameter, parameter_formation_replay) = replay_parameter_type_formation(
                kernel,
                signature,
                &context.normalized_wire(),
                parameter_node.term(),
            )?;
            if normal_parameter != *parameter_node.term() {
                return Err(SynthesisV2Error::InvalidSynthesisCode);
            }
            let extended = extend_verified_context(kernel, signature, context, normal_parameter)?;
            let body_node = verify_code_node(
                kernel,
                signature,
                policy,
                &extended,
                body,
                budget,
                child_depth,
            )?;
            let body_level = expect_sort_v2(body_node.inferred_type())?;
            let term = Term::Pi {
                parameter: Box::new(parameter_node.term().clone()),
                body: Box::new(body_node.term().clone()),
            };
            finalize_synthesis_node(
                kernel,
                signature,
                policy,
                context,
                code,
                term,
                Term::Sort {
                    level: parameter_level.max(body_level),
                },
                vec![parameter_node, body_node],
                Vec::new(),
                vec![parameter_formation_replay],
            )
        }
        SynthesisCodeV2::LambdaIntroduction {
            parameter_type,
            body,
        } => {
            let parameter_node = verify_code_node(
                kernel,
                signature,
                policy,
                context,
                parameter_type,
                budget,
                child_depth,
            )?;
            expect_sort_v2(parameter_node.inferred_type())?;
            let (normal_parameter, parameter_formation_replay) = replay_parameter_type_formation(
                kernel,
                signature,
                &context.normalized_wire(),
                parameter_node.term(),
            )?;
            if normal_parameter != *parameter_node.term() {
                return Err(SynthesisV2Error::InvalidSynthesisCode);
            }
            let extended =
                extend_verified_context(kernel, signature, context, normal_parameter.clone())?;
            let body_node = verify_code_node(
                kernel,
                signature,
                policy,
                &extended,
                body,
                budget,
                child_depth,
            )?;
            let term = Term::Lambda {
                parameter_type: Box::new(parameter_node.term().clone()),
                body: Box::new(body_node.term().clone()),
            };
            let inferred = Term::Pi {
                parameter: Box::new(normal_parameter),
                body: Box::new(body_node.inferred_type().clone()),
            };
            finalize_synthesis_node(
                kernel,
                signature,
                policy,
                context,
                code,
                term,
                inferred,
                vec![parameter_node, body_node],
                Vec::new(),
                vec![parameter_formation_replay],
            )
        }
        SynthesisCodeV2::ApplicationElimination {
            function,
            argument,
            function_conversion,
            argument_conversion,
        } => {
            let function_node = verify_code_node(
                kernel,
                signature,
                policy,
                context,
                function,
                budget,
                child_depth,
            )?;
            let argument_node = verify_code_node(
                kernel,
                signature,
                policy,
                context,
                argument,
                budget,
                child_depth,
            )?;
            let verified_function_conversion = verify_conversion_with_budget(
                kernel,
                signature,
                policy,
                function_conversion,
                budget,
                child_depth,
            )?;
            if function_conversion.context != context.normalized_wire()
                || function_conversion.left != *function_node.inferred_type()
                || function_conversion.right != function_conversion.common_normal_form
                || function_conversion.endpoint_judgment != BaseQ0EndpointJudgmentV2::TypeFormation
            {
                return Err(SynthesisV2Error::InvalidSynthesisCode);
            }
            let Term::Pi { parameter, body } = &function_conversion.common_normal_form else {
                return Err(SynthesisV2Error::ExpectedFunction);
            };
            let verified_argument_conversion = verify_conversion_with_budget(
                kernel,
                signature,
                policy,
                argument_conversion,
                budget,
                child_depth,
            )?;
            if argument_conversion.context != context.normalized_wire()
                || argument_conversion.left != *argument_node.inferred_type()
                || argument_conversion.right != **parameter
                || argument_conversion.endpoint_judgment != BaseQ0EndpointJudgmentV2::TypeFormation
            {
                return Err(SynthesisV2Error::InvalidSynthesisCode);
            }
            let argument_input = OpenJudgment::HasType {
                context: context.normalized_wire(),
                term: argument_node.term().clone(),
                ty: (**parameter).clone(),
            };
            let argument_output = kernel.verify_open_judgment(signature, &argument_input)?;
            ensure_replay_output_closure(&argument_output)?;
            let inferred = substitute_top_v2(body, argument_node.term(), budget, child_depth)?;
            let term = Term::Apply {
                function: Box::new(function_node.term().clone()),
                argument: Box::new(argument_node.term().clone()),
            };
            finalize_synthesis_node(
                kernel,
                signature,
                policy,
                context,
                code,
                term,
                inferred,
                vec![function_node, argument_node],
                vec![verified_function_conversion, verified_argument_conversion],
                vec![VerifiedKernelReplayPairV2 {
                    input: argument_input,
                    output: argument_output,
                }],
            )
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn finalize_synthesis_node(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
    context: &VerifiedContext,
    code: &SynthesisCodeV2,
    term: Term,
    inferred_type: Term,
    premises: Vec<VerifiedSynthesisNodeV2>,
    conversions: Vec<VerifiedBaseQ0ConversionV2>,
    mut kernel_replays: Vec<VerifiedKernelReplayPairV2>,
) -> Result<VerifiedSynthesisNodeV2, SynthesisV2Error> {
    ensure_public_term(&term)?;
    ensure_checker_output_term(&inferred_type)?;
    let formation_input = OpenJudgment::TypeFormation {
        context: context.normalized_wire(),
        term: inferred_type,
    };
    let formation_output = kernel.verify_open_judgment(signature, &formation_input)?;
    ensure_replay_output_closure(&formation_output)?;
    let OpenJudgment::TypeFormation {
        term: normalized_type,
        ..
    } = &formation_output
    else {
        return Err(SynthesisV2Error::KernelReplayMismatch);
    };
    ensure_checker_output_term(normalized_type)?;
    let normalized_type = normalized_type.clone();
    let typing_input = OpenJudgment::HasType {
        context: context.normalized_wire(),
        term: term.clone(),
        ty: normalized_type.clone(),
    };
    let typing_output = kernel.verify_open_judgment(signature, &typing_input)?;
    ensure_replay_output_closure(&typing_output)?;
    kernel_replays.push(VerifiedKernelReplayPairV2 {
        input: formation_input,
        output: formation_output,
    });
    kernel_replays.push(VerifiedKernelReplayPairV2 {
        input: typing_input,
        output: typing_output,
    });
    let premise_digests = premises
        .iter()
        .map(|premise| premise.digest().clone())
        .collect::<Vec<_>>();
    let conversion_digests = conversions
        .iter()
        .map(|conversion| conversion.digest().clone())
        .collect::<Vec<_>>();
    let context_wire = context.normalized_wire();
    let signature_digest = signature.digest().clone();
    let policy_digest = policy.digest().clone();
    let synthesis_protocol_digest = synthesis_protocol_digest_v2();
    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    let digest = Digest::of_canonical(
        "verified-synthesis-node-v2",
        &SynthesisNodeDigestInput {
            code,
            signature_digest: &signature_digest,
            policy_digest: &policy_digest,
            synthesis_protocol_digest: &synthesis_protocol_digest,
            kernel_protocol_digest: &kernel_protocol_digest,
            context: &context_wire,
            term: &term,
            inferred_type: &normalized_type,
            premise_digests: &premise_digests,
            conversion_digests: &conversion_digests,
            kernel_replays: &kernel_replays,
        },
    );
    Ok(VerifiedSynthesisNodeV2 {
        code: code.clone(),
        signature_digest,
        policy_digest,
        synthesis_protocol_digest,
        kernel_protocol_digest,
        context: context_wire,
        term,
        inferred_type: normalized_type,
        premises,
        conversions,
        kernel_replays,
        digest,
    })
}

fn collect_node_replays(
    node: &VerifiedSynthesisNodeV2,
    output: &mut Vec<VerifiedKernelReplayPairV2>,
) {
    output.extend(node.kernel_replays.iter().cloned());
    for conversion in &node.conversions {
        output.extend(conversion.endpoint_replays.iter().cloned());
    }
    for premise in &node.premises {
        collect_node_replays(premise, output);
    }
}

fn normalize_type_term(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &DependentContext,
    term: &Term,
) -> Result<Term, SynthesisV2Error> {
    let output = kernel.verify_open_judgment(
        signature,
        &OpenJudgment::TypeFormation {
            context: context.clone(),
            term: term.clone(),
        },
    )?;
    let OpenJudgment::TypeFormation { term, .. } = output else {
        return Err(SynthesisV2Error::KernelReplayMismatch);
    };
    ensure_public_term(&term)?;
    Ok(term)
}

fn replay_parameter_type_formation(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &DependentContext,
    term: &Term,
) -> Result<(Term, VerifiedKernelReplayPairV2), SynthesisV2Error> {
    let input = OpenJudgment::TypeFormation {
        context: context.clone(),
        term: term.clone(),
    };
    let output = kernel.verify_open_judgment(signature, &input)?;
    ensure_replay_output_closure(&output)?;
    let OpenJudgment::TypeFormation {
        term: normalized_term,
        ..
    } = &output
    else {
        return Err(SynthesisV2Error::KernelReplayMismatch);
    };
    ensure_public_term(normalized_term)?;
    Ok((
        normalized_term.clone(),
        VerifiedKernelReplayPairV2 { input, output },
    ))
}

fn extend_verified_context(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &VerifiedContext,
    entry: Term,
) -> Result<VerifiedContext, SynthesisV2Error> {
    ensure_public_term(&entry)?;
    let mut entries = context.entries().to_vec();
    entries.push(entry);
    let verified = kernel.verify_context(signature, &DependentContext(entries))?;
    ensure_public_context(&verified.normalized_wire())?;
    Ok(verified)
}

fn expect_sort_v2(term: &Term) -> Result<u16, SynthesisV2Error> {
    match term {
        Term::Sort { level } if CHECKER_OUTPUT_UNIVERSE_LEVELS_V2.contains(level) => Ok(*level),
        Term::Sort { level } => Err(SynthesisV2Error::CheckerOutputUniverseLevel(*level)),
        _ => Err(SynthesisV2Error::ExpectedType),
    }
}

fn bind_policy_signature(
    signature: &VerifiedSignature,
    policy: &VerifiedBaseQ0ConversionPolicyV2,
) -> Result<(), SynthesisV2Error> {
    if policy.signature_digest() != signature.digest() {
        return Err(SynthesisV2Error::InvalidDeltaPolicyEntry);
    }
    Ok(())
}

fn ensure_public_context(context: &DependentContext) -> Result<(), SynthesisV2Error> {
    for entry in &context.0 {
        ensure_public_term(entry)?;
    }
    Ok(())
}

fn ensure_public_term(term: &Term) -> Result<(), SynthesisV2Error> {
    ensure_term_levels(term, &PUBLIC_UNIVERSE_LEVELS_V2, true)
}

fn ensure_checker_output_term(term: &Term) -> Result<(), SynthesisV2Error> {
    ensure_term_levels(term, &CHECKER_OUTPUT_UNIVERSE_LEVELS_V2, false)
}

fn ensure_replay_output_closure(output: &OpenJudgment) -> Result<(), SynthesisV2Error> {
    ensure_public_context(output.context())?;
    match output {
        OpenJudgment::TypeFormation { term, .. } => ensure_checker_output_term(term),
        OpenJudgment::HasType { term, ty, .. } => {
            ensure_checker_output_term(term)?;
            ensure_checker_output_term(ty)
        }
        OpenJudgment::DefinitionallyEqual {
            left, right, ty, ..
        } => {
            ensure_checker_output_term(left)?;
            ensure_checker_output_term(right)?;
            ensure_checker_output_term(ty)
        }
    }
}

fn ensure_term_levels(term: &Term, levels: &[u16], public: bool) -> Result<(), SynthesisV2Error> {
    let mut pending = vec![term];
    while let Some(term) = pending.pop() {
        match term {
            Term::Sort { level } if !levels.contains(level) => {
                return Err(if public {
                    SynthesisV2Error::PublicUniverseLevel(*level)
                } else {
                    SynthesisV2Error::CheckerOutputUniverseLevel(*level)
                });
            }
            Term::Sort { .. }
            | Term::Var { .. }
            | Term::Global { .. }
            | Term::UnitType
            | Term::Unit => {}
            Term::Pi { parameter, body }
            | Term::Apply {
                function: parameter,
                argument: body,
            } => {
                pending.push(body);
                pending.push(parameter);
            }
            Term::Lambda {
                parameter_type,
                body,
            } => {
                pending.push(body);
                pending.push(parameter_type);
            }
            Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
                return Err(SynthesisV2Error::OutsideLambdaUnitFragment);
            }
        }
    }
    Ok(())
}

fn substitute_top_v2(
    body: &Term,
    replacement: &Term,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<Term, SynthesisV2Error> {
    let lifted = shift_v2(replacement, 1, 0, budget, depth)?;
    let replaced = substitute_v2(body, 0, &lifted, 0, budget, depth)?;
    shift_v2(&replaced, -1, 0, budget, depth)
}

fn substitute_v2(
    term: &Term,
    target: u32,
    replacement: &Term,
    binder_depth: u32,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<Term, SynthesisV2Error> {
    budget.enter(depth)?;
    let child_depth = budget.child_depth(depth)?;
    let sought = target
        .checked_add(binder_depth)
        .ok_or(SynthesisV2Error::InvalidSubstitution)?;
    match term {
        Term::Var { index } if *index == sought => {
            shift_v2(replacement, i64::from(binder_depth), 0, budget, child_depth)
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(term.clone()),
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(substitute_v2(
                parameter,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
            body: Box::new(substitute_v2(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or(SynthesisV2Error::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(substitute_v2(
                parameter_type,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
            body: Box::new(substitute_v2(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or(SynthesisV2Error::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(substitute_v2(
                function,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
            argument: Box::new(substitute_v2(
                argument,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
        }),
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            Err(SynthesisV2Error::OutsideLambdaUnitFragment)
        }
    }
}

fn shift_v2(
    term: &Term,
    amount: i64,
    cutoff: u32,
    budget: &mut V2Budget,
    depth: u16,
) -> Result<Term, SynthesisV2Error> {
    budget.enter(depth)?;
    let child_depth = budget.child_depth(depth)?;
    match term {
        Term::Var { index } if *index >= cutoff => {
            let shifted = i64::from(*index)
                .checked_add(amount)
                .ok_or(SynthesisV2Error::InvalidSubstitution)?;
            Ok(Term::Var {
                index: u32::try_from(shifted).map_err(|_| SynthesisV2Error::InvalidSubstitution)?,
            })
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(term.clone()),
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(shift_v2(parameter, amount, cutoff, budget, child_depth)?),
            body: Box::new(shift_v2(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(SynthesisV2Error::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(shift_v2(
                parameter_type,
                amount,
                cutoff,
                budget,
                child_depth,
            )?),
            body: Box::new(shift_v2(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(SynthesisV2Error::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(shift_v2(function, amount, cutoff, budget, child_depth)?),
            argument: Box::new(shift_v2(argument, amount, cutoff, budget, child_depth)?),
        }),
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            Err(SynthesisV2Error::OutsideLambdaUnitFragment)
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct V2Budget {
    initial_operations: u32,
    operations_left: u32,
    depth_limit: u16,
}

impl V2Budget {
    fn new(kernel: &Kernel) -> Self {
        let limits = kernel.limits();
        Self {
            initial_operations: limits.max_operations,
            operations_left: limits.max_operations,
            depth_limit: limits.max_depth,
        }
    }

    fn enter(&mut self, depth: u16) -> Result<(), SynthesisV2Error> {
        if depth > self.depth_limit {
            return Err(SynthesisV2Error::ResourceExhausted);
        }
        self.operations_left = self
            .operations_left
            .checked_sub(1)
            .ok_or(SynthesisV2Error::ResourceExhausted)?;
        Ok(())
    }

    fn child_depth(&self, depth: u16) -> Result<u16, SynthesisV2Error> {
        depth
            .checked_add(1)
            .filter(|child| *child <= self.depth_limit)
            .ok_or(SynthesisV2Error::ResourceExhausted)
    }
}

pub fn synthesis_code_inventory_digest_v2() -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.sequence(&SYNTHESIS_CODE_INVENTORY_V2);
    Digest::of_domain_bytes(
        "lambda-unit-synthesis-code-inventory-v2",
        encoder.as_bytes(),
    )
}

pub fn synthesis_protocol_digest_v2() -> Digest {
    let inventory = synthesis_code_inventory_digest_v2();
    let canonical = [
        canonical_source(include_bytes!("lib.rs")),
        canonical_source(include_bytes!("v2.rs")),
        canonical_source(include_bytes!("../Cargo.toml")),
        canonical_source(include_bytes!("../Cargo.lock")),
        canonical_source(include_bytes!("../../../rust-toolchain.toml")),
        canonical_source(include_bytes!("../../../.cargo/config.toml")),
    ];
    let mut chunks = vec![
        SYNTHESIS_PROTOCOL_ID_V2.as_bytes(),
        inventory.as_str().as_bytes(),
    ];
    chunks.extend(canonical.iter().map(Vec::as_slice));
    Digest::of_domain_chunks("pen-kernel-synthesis-protocol-v2", &chunks)
}

fn canonical_source(bytes: &[u8]) -> Vec<u8> {
    let text = std::str::from_utf8(bytes).expect("trusted V2 source must be UTF-8");
    let canonical = text.replace("\r\n", "\n");
    assert!(
        !canonical.contains('\r'),
        "bare carriage return in V2 source"
    );
    canonical.into_bytes()
}

struct PolicyDigestInput<'a> {
    signature_digest: &'a Digest,
    wire: &'a BaseQ0ConversionPolicyV2,
}

impl CanonicalEncode for PolicyDigestInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.signature_digest.encode_canonical(encoder);
        self.wire.encode_canonical(encoder);
    }
}

struct ConversionDigestInput<'a> {
    code: &'a BaseQ0ConversionCodeV2,
    policy_digest: &'a Digest,
    signature_digest: &'a Digest,
    kernel_protocol_digest: &'a Digest,
    endpoint_replays: &'a [VerifiedKernelReplayPairV2],
    no_redex_census_digest: &'a Digest,
}

impl CanonicalEncode for ConversionDigestInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.code.encode_canonical(encoder);
        self.policy_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        encoder.sequence(self.endpoint_replays);
        self.no_redex_census_digest.encode_canonical(encoder);
    }
}

struct SynthesisNodeDigestInput<'a> {
    code: &'a SynthesisCodeV2,
    signature_digest: &'a Digest,
    policy_digest: &'a Digest,
    synthesis_protocol_digest: &'a Digest,
    kernel_protocol_digest: &'a Digest,
    context: &'a DependentContext,
    term: &'a Term,
    inferred_type: &'a Term,
    premise_digests: &'a [Digest],
    conversion_digests: &'a [Digest],
    kernel_replays: &'a [VerifiedKernelReplayPairV2],
}

impl CanonicalEncode for SynthesisNodeDigestInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.code.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.policy_digest.encode_canonical(encoder);
        self.synthesis_protocol_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.context.encode_canonical(encoder);
        self.term.encode_canonical(encoder);
        self.inferred_type.encode_canonical(encoder);
        encoder.sequence(self.premise_digests);
        encoder.sequence(self.conversion_digests);
        encoder.sequence(self.kernel_replays);
    }
}

struct CanonicalReplayPairs<'a>(&'a [VerifiedKernelReplayPairV2]);

impl CanonicalEncode for CanonicalReplayPairs<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

struct SynthesisCertificateDigestInput<'a> {
    schema_version: u16,
    code: &'a SynthesisCodeV2,
    signature_digest: &'a Digest,
    policy_digest: &'a Digest,
    input_context: &'a DependentContext,
    context_digest: &'a Digest,
    term: &'a Term,
    normalized_type: &'a Term,
    derivation_digest: &'a Digest,
    synthesis_protocol_digest: &'a Digest,
    kernel_protocol_digest: &'a Digest,
    aggregate_kernel_replay_digest: &'a Digest,
}

impl CanonicalEncode for SynthesisCertificateDigestInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.code.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.policy_digest.encode_canonical(encoder);
        self.input_context.encode_canonical(encoder);
        self.context_digest.encode_canonical(encoder);
        self.term.encode_canonical(encoder);
        self.normalized_type.encode_canonical(encoder);
        self.derivation_digest.encode_canonical(encoder);
        self.synthesis_protocol_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.aggregate_kernel_replay_digest
            .encode_canonical(encoder);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_kernel::{Declaration, KernelLimits, UncheckedSignature};

    struct Fixture {
        kernel: Kernel,
        signature: VerifiedSignature,
        delta: GlobalId,
        function: GlobalId,
        policy: VerifiedBaseQ0ConversionPolicyV2,
    }

    fn fixture() -> Fixture {
        let kernel = Kernel::new(KernelLimits::default()).expect("safe kernel");
        let delta = GlobalId(Digest::of_bytes(b"v2-delta"));
        let function = GlobalId(Digest::of_bytes(b"v2-function"));
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![
                    Declaration {
                        id: delta.clone(),
                        ty: Term::UnitType,
                        body: Some(Term::Unit),
                    },
                    Declaration {
                        id: function.clone(),
                        ty: unit_endomorphism_type(),
                        body: None,
                    },
                ],
            })
            .expect("verified fixture");
        let policy = verify_base_q0_conversion_policy_v2(
            &signature,
            &BaseQ0ConversionPolicyV2 {
                allowed_transparent_deltas: vec![DeltaPolicyEntryV2 {
                    global_slot: 0,
                    id: delta.clone(),
                }],
            },
        )
        .expect("exact authority-free policy");
        Fixture {
            kernel,
            signature,
            delta,
            function,
            policy,
        }
    }

    fn unit_endomorphism_type() -> Term {
        Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::UnitType),
        }
    }

    fn beta_unit() -> Term {
        Term::Apply {
            function: Box::new(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Var { index: 0 }),
            }),
            argument: Box::new(Term::Unit),
        }
    }

    fn beta_unit_type() -> Term {
        Term::Apply {
            function: Box::new(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            }),
            argument: Box::new(Term::Unit),
        }
    }

    #[test]
    fn public_sort_one_produces_sort_two_but_public_sort_two_is_rejected() {
        let fixture = fixture();
        let context = DependentContext::default();
        let code = propose_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &context,
            &Term::Sort { level: 1 },
        )
        .expect("public Sort 1 code");
        let wire = serde_json::to_string(&code).expect("serializable authority-free code");
        let decoded: SynthesisCodeV2 = serde_json::from_str(&wire).expect("exact code wire");
        let verified = verify_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &context,
            &decoded,
        )
        .expect("checker output Sort 2 is admitted");
        assert_eq!(verified.term(), &Term::Sort { level: 1 });
        assert_eq!(verified.normalized_type(), &Term::Sort { level: 2 });

        assert_eq!(
            verify_synthesis_code_v2(
                &fixture.kernel,
                &fixture.signature,
                &fixture.policy,
                &context,
                &SynthesisCodeV2::Sort { level: 2 },
            )
            .expect_err("public Sort 2 must fail"),
            SynthesisV2Error::PublicUniverseLevel(2)
        );
    }

    #[test]
    fn public_application_can_infer_sort_two_without_serializing_sort_three() {
        let fixture = fixture();
        let context = DependentContext::default();
        let term = Term::Apply {
            function: Box::new(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Sort { level: 1 }),
            }),
            argument: Box::new(Term::Unit),
        };
        let code = propose_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &context,
            &term,
        )
        .expect("public application proposal");
        let SynthesisCodeV2::ApplicationElimination {
            function_conversion,
            argument_conversion,
            ..
        } = &code
        else {
            panic!("application code");
        };
        assert_eq!(
            function_conversion.endpoint_judgment,
            BaseQ0EndpointJudgmentV2::TypeFormation
        );
        assert_eq!(
            argument_conversion.endpoint_judgment,
            BaseQ0EndpointJudgmentV2::TypeFormation
        );
        let wire = serde_json::to_string(&code).expect("authority-free code wire");
        assert!(
            !wire.contains("\"level\":3"),
            "kernel-internal formation universe must not enter certificate syntax"
        );

        let verified = verify_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &context,
            &code,
        )
        .expect("TypeFormation endpoint replays admit inferred Sort 2");
        assert_eq!(verified.term(), &term);
        assert_eq!(verified.normalized_type(), &Term::Sort { level: 2 });
        assert_eq!(
            propose_synthesis_code_v2(
                &fixture.kernel,
                &fixture.signature,
                &fixture.policy,
                &DependentContext(vec![Term::Sort { level: 2 }]),
                &Term::Unit,
            )
            .expect_err("public contexts remain capped at Sort 1"),
            SynthesisV2Error::PublicUniverseLevel(2)
        );

        let lambda_node = &verified.derivation().premises()[0];
        assert!(lambda_node.kernel_replays().iter().any(|replay| {
            matches!(
                replay.input(),
                OpenJudgment::TypeFormation {
                    term: Term::UnitType,
                    ..
                }
            )
        }));
        let mut aggregate_inputs = Vec::new();
        collect_node_replays(verified.derivation(), &mut aggregate_inputs);
        assert!(aggregate_inputs.iter().any(|replay| {
            matches!(
                replay.input(),
                OpenJudgment::TypeFormation {
                    term: Term::UnitType,
                    ..
                }
            )
        }));

        fn assert_node_bindings(
            node: &VerifiedSynthesisNodeV2,
            signature_digest: &Digest,
            policy_digest: &Digest,
            synthesis_protocol_digest: &Digest,
            kernel_protocol_digest: &Digest,
        ) {
            assert_eq!(node.signature_digest(), signature_digest);
            assert_eq!(node.policy_digest(), policy_digest);
            assert_eq!(node.synthesis_protocol_digest(), synthesis_protocol_digest);
            assert_eq!(node.kernel_protocol_digest(), kernel_protocol_digest);
            for premise in node.premises() {
                assert_node_bindings(
                    premise,
                    signature_digest,
                    policy_digest,
                    synthesis_protocol_digest,
                    kernel_protocol_digest,
                );
            }
        }
        assert_node_bindings(
            verified.derivation(),
            fixture.signature.digest(),
            fixture.policy.digest(),
            verified.synthesis_protocol_digest(),
            verified.kernel_protocol_digest(),
        );
    }

    #[test]
    fn pi_parameter_formation_replay_is_recorded() {
        let fixture = fixture();
        let term = Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::UnitType),
        };
        let code = propose_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &DependentContext::default(),
            &term,
        )
        .expect("Pi code");
        let verified = verify_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &DependentContext::default(),
            &code,
        )
        .expect("Pi verification");
        assert!(verified.derivation().kernel_replays().iter().any(|replay| {
            matches!(
                replay.input(),
                OpenJudgment::TypeFormation {
                    term: Term::UnitType,
                    ..
                }
            )
        }));
    }

    #[test]
    fn policy_types_use_checker_output_closure_while_bodies_remain_public() {
        let kernel = Kernel::new(KernelLimits::default()).expect("safe kernel");
        let id = GlobalId(Digest::of_bytes(b"v2-sort-two-declaration"));
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: id.clone(),
                    ty: Term::Pi {
                        parameter: Box::new(Term::UnitType),
                        body: Box::new(Term::Sort { level: 2 }),
                    },
                    body: Some(Term::Lambda {
                        parameter_type: Box::new(Term::UnitType),
                        body: Box::new(Term::Sort { level: 1 }),
                    }),
                }],
            })
            .expect("bodyful declaration with checker-produced Sort 2 type");
        verify_base_q0_conversion_policy_v2(
            &signature,
            &BaseQ0ConversionPolicyV2 {
                allowed_transparent_deltas: vec![DeltaPolicyEntryV2 { global_slot: 0, id }],
            },
        )
        .expect("declaration type admits Sort 2 while its body remains public");
    }

    #[test]
    fn hidden_global_bodies_cannot_escape_replay_output_closure() {
        let kernel = Kernel::new(KernelLimits::default()).expect("safe kernel");
        let high_sink = GlobalId(Digest::of_bytes(b"v2-hidden-sort-three-sink"));
        let hidden_sort = GlobalId(Digest::of_bytes(b"v2-hidden-sort-three-output"));
        let hidden_sigma = GlobalId(Digest::of_bytes(b"v2-hidden-sigma-output"));
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![
                    Declaration {
                        id: high_sink.clone(),
                        ty: Term::Pi {
                            parameter: Box::new(Term::Sort { level: 4 }),
                            body: Box::new(Term::UnitType),
                        },
                        body: None,
                    },
                    Declaration {
                        id: hidden_sort.clone(),
                        ty: Term::UnitType,
                        body: Some(Term::Apply {
                            function: Box::new(Term::Global { id: high_sink }),
                            argument: Box::new(Term::Sort { level: 3 }),
                        }),
                    },
                    Declaration {
                        id: hidden_sigma.clone(),
                        ty: Term::Sort { level: 0 },
                        body: Some(Term::Sigma {
                            parameter: Box::new(Term::UnitType),
                            body: Box::new(Term::UnitType),
                        }),
                    },
                ],
            })
            .expect("kernel accepts unrestricted signature bodies");
        let policy =
            verify_base_q0_conversion_policy_v2(&signature, &BaseQ0ConversionPolicyV2::default())
                .expect("empty conversion policy");
        let context = DependentContext::default();

        assert_eq!(
            verify_synthesis_code_v2(
                &kernel,
                &signature,
                &policy,
                &context,
                &SynthesisCodeV2::GlobalLookup {
                    id: hidden_sort,
                    global_slot: 1,
                },
            )
            .expect_err("normalized Sort 3 must not enter a replay capability"),
            SynthesisV2Error::CheckerOutputUniverseLevel(3)
        );
        assert_eq!(
            verify_synthesis_code_v2(
                &kernel,
                &signature,
                &policy,
                &context,
                &SynthesisCodeV2::GlobalLookup {
                    id: hidden_sigma,
                    global_slot: 2,
                },
            )
            .expect_err("normalized Sigma must not enter a replay capability"),
            SynthesisV2Error::OutsideLambdaUnitFragment
        );
    }

    #[test]
    fn variable_ordinal_shift_and_global_slot_are_reconstructed() {
        let fixture = fixture();
        let context = DependentContext(vec![Term::Sort { level: 0 }, Term::Var { index: 0 }]);
        let code = propose_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &context,
            &Term::Var { index: 0 },
        )
        .expect("variable code");
        let verified = verify_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &context,
            &code,
        )
        .expect("exact ordinal and shift");
        assert_eq!(verified.normalized_type(), &Term::Var { index: 1 });
        let mut tampered = code;
        let SynthesisCodeV2::VariableLookup {
            context_ordinal, ..
        } = &mut tampered
        else {
            panic!("variable code");
        };
        *context_ordinal = 0;
        assert_eq!(
            verify_synthesis_code_v2(
                &fixture.kernel,
                &fixture.signature,
                &fixture.policy,
                &context,
                &tampered,
            )
            .expect_err("ordinal is reconstructed"),
            SynthesisV2Error::InvalidSynthesisCode
        );

        let global_code = propose_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &DependentContext::default(),
            &Term::Global {
                id: fixture.function.clone(),
            },
        )
        .expect("global code");
        let mut bad_slot = global_code;
        let SynthesisCodeV2::GlobalLookup { global_slot, .. } = &mut bad_slot else {
            panic!("global lookup code");
        };
        *global_slot = 0;
        assert_eq!(
            verify_synthesis_code_v2(
                &fixture.kernel,
                &fixture.signature,
                &fixture.policy,
                &DependentContext::default(),
                &bad_slot,
            )
            .expect_err("slot must identify the exact id"),
            SynthesisV2Error::InvalidSynthesisCode
        );
    }

    #[test]
    fn beta_delta_and_all_congruence_forms_verify_exactly() {
        let fixture = fixture();
        let empty = DependentContext::default();
        let beta = propose_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &empty,
            &beta_unit(),
            &Term::Unit,
            &Term::UnitType,
        )
        .expect("beta proposal");
        assert!(matches!(
            beta.left_trace.steps.first(),
            Some(BaseQ0ReductionStepCodeV2::Beta { .. })
        ));
        verify_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &beta,
        )
        .expect("beta certificate");

        let delta_left = Term::Global {
            id: fixture.delta.clone(),
        };
        let delta = propose_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &empty,
            &delta_left,
            &Term::Unit,
            &Term::UnitType,
        )
        .expect("delta proposal");
        assert!(matches!(
            delta.left_trace.steps.first(),
            Some(BaseQ0ReductionStepCodeV2::TransparentDelta { global_slot: 0, .. })
        ));
        verify_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &delta,
        )
        .expect("policy-relative delta");

        let pi_left = Term::Pi {
            parameter: Box::new(beta_unit_type()),
            body: Box::new(beta_unit_type()),
        };
        let pi_right = Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::UnitType),
        };
        let pi = propose_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &empty,
            &pi_left,
            &pi_right,
            &Term::Sort { level: 0 },
        )
        .expect("Pi congruence");
        assert!(matches!(
            pi.left_trace.steps.first(),
            Some(BaseQ0ReductionStepCodeV2::PiParameterCongruence { .. })
        ));
        assert!(
            pi.left_trace
                .steps
                .iter()
                .any(|step| matches!(step, BaseQ0ReductionStepCodeV2::PiBodyCongruence { .. }))
        );
        verify_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &pi,
        )
        .expect("Pi congruence certificate");

        let lambda_left = Term::Lambda {
            parameter_type: Box::new(beta_unit_type()),
            body: Box::new(beta_unit()),
        };
        let lambda_right = Term::Lambda {
            parameter_type: Box::new(Term::UnitType),
            body: Box::new(Term::Unit),
        };
        let lambda = propose_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &empty,
            &lambda_left,
            &lambda_right,
            &unit_endomorphism_type(),
        )
        .expect("Lambda congruence");
        assert!(matches!(
            lambda.left_trace.steps.first(),
            Some(BaseQ0ReductionStepCodeV2::LambdaParameterCongruence { .. })
        ));
        assert!(
            lambda
                .left_trace
                .steps
                .iter()
                .any(|step| matches!(step, BaseQ0ReductionStepCodeV2::LambdaBodyCongruence { .. }))
        );
        verify_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &lambda,
        )
        .expect("Lambda congruence certificate");

        let function_redex = Term::Apply {
            function: Box::new(Term::Lambda {
                parameter_type: Box::new(unit_endomorphism_type()),
                body: Box::new(Term::Var { index: 0 }),
            }),
            argument: Box::new(Term::Global {
                id: fixture.function.clone(),
            }),
        };
        let apply_left = Term::Apply {
            function: Box::new(function_redex),
            argument: Box::new(beta_unit()),
        };
        let apply_right = Term::Apply {
            function: Box::new(Term::Global {
                id: fixture.function.clone(),
            }),
            argument: Box::new(Term::Unit),
        };
        let apply = propose_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &empty,
            &apply_left,
            &apply_right,
            &Term::UnitType,
        )
        .expect("Apply congruence");
        assert!(matches!(
            apply.left_trace.steps.first(),
            Some(BaseQ0ReductionStepCodeV2::ApplyFunctionCongruence { .. })
        ));
        assert!(apply.left_trace.steps.iter().any(|step| matches!(
            step,
            BaseQ0ReductionStepCodeV2::ApplyArgumentCongruence { .. }
        )));
        verify_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &apply,
        )
        .expect("Apply congruence certificate");
    }

    #[test]
    fn no_redex_census_and_policy_kernel_agreement_are_mandatory() {
        let fixture = fixture();
        let empty = DependentContext::default();
        let mut beta = propose_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &empty,
            &beta_unit(),
            &Term::Unit,
            &Term::UnitType,
        )
        .expect("beta proposal");
        beta.no_redex_census.entries.clear();
        assert_eq!(
            verify_base_q0_conversion_code_v2(
                &fixture.kernel,
                &fixture.signature,
                &fixture.policy,
                &beta,
            )
            .expect_err("census omission"),
            SynthesisV2Error::InvalidNoRedexCensus
        );

        let empty_policy = verify_base_q0_conversion_policy_v2(
            &fixture.signature,
            &BaseQ0ConversionPolicyV2::default(),
        )
        .expect("empty policy");
        let global = Term::Global {
            id: fixture.delta.clone(),
        };
        let code = propose_base_q0_conversion_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &empty_policy,
            &empty,
            &global,
            &global,
            &Term::UnitType,
        )
        .expect("policy-local producer treats the global as stuck");
        assert_eq!(
            verify_base_q0_conversion_code_v2(
                &fixture.kernel,
                &fixture.signature,
                &empty_policy,
                &code,
            )
            .expect_err("kernel unfolds the disabled bodyful global"),
            SynthesisV2Error::KernelReplayMismatch
        );
    }

    #[test]
    fn application_code_reconstructs_conversions_and_dependent_result() {
        let fixture = fixture();
        let context = DependentContext(vec![Term::Pi {
            parameter: Box::new(Term::Sort { level: 0 }),
            body: Box::new(Term::Var { index: 0 }),
        }]);
        let term = Term::Apply {
            function: Box::new(Term::Var { index: 0 }),
            argument: Box::new(Term::UnitType),
        };
        let code = propose_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &context,
            &term,
        )
        .expect("untrusted application producer");
        let verified = verify_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &context,
            &code,
        )
        .expect("independent application verifier");
        assert_eq!(verified.term(), &term);
        assert_eq!(verified.normalized_type(), &Term::UnitType);
        assert_eq!(verified.derivation().conversions().len(), 2);
    }

    #[test]
    fn authority_free_wires_reject_claim_fields_and_bad_policy_slots() {
        assert!(
            serde_json::from_str::<BaseQ0ConversionPolicyV2>(
                r#"{"allowed_transparent_deltas":[],"is_public":true}"#
            )
            .is_err()
        );
        let fixture = fixture();
        assert_eq!(
            verify_base_q0_conversion_policy_v2(
                &fixture.signature,
                &BaseQ0ConversionPolicyV2 {
                    allowed_transparent_deltas: vec![DeltaPolicyEntryV2 {
                        global_slot: 1,
                        id: fixture.delta,
                    }],
                },
            )
            .expect_err("slot and id must agree"),
            SynthesisV2Error::InvalidDeltaPolicyEntry
        );
    }

    #[test]
    fn synthesis_inventory_is_exactly_tied_to_all_wire_tags_and_names() {
        let fixture = fixture();
        let application_term = Term::Apply {
            function: Box::new(Term::Global {
                id: fixture.function.clone(),
            }),
            argument: Box::new(Term::Unit),
        };
        let application = propose_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &DependentContext::default(),
            &application_term,
        )
        .expect("application inventory sample");
        let samples = vec![
            SynthesisCodeV2::Sort { level: 0 },
            SynthesisCodeV2::UnitType,
            SynthesisCodeV2::Unit,
            SynthesisCodeV2::VariableLookup {
                index: 0,
                context_ordinal: 0,
                shift_distance: 1,
            },
            SynthesisCodeV2::GlobalLookup {
                id: fixture.function,
                global_slot: 1,
            },
            SynthesisCodeV2::PiFormation {
                parameter: Box::new(SynthesisCodeV2::UnitType),
                body: Box::new(SynthesisCodeV2::UnitType),
            },
            SynthesisCodeV2::LambdaIntroduction {
                parameter_type: Box::new(SynthesisCodeV2::UnitType),
                body: Box::new(SynthesisCodeV2::Unit),
            },
            application,
        ];
        let observed_tags = samples
            .iter()
            .map(SynthesisCodeV2::inventory_tag)
            .collect::<Vec<_>>();
        assert_eq!(observed_tags.as_slice(), &SYNTHESIS_CODE_INVENTORY_V2);
        let observed_wire_names = samples
            .iter()
            .map(|sample| {
                serde_json::to_value(sample)
                    .expect("wire value")
                    .get("rule")
                    .and_then(serde_json::Value::as_str)
                    .expect("internally tagged rule")
                    .to_owned()
            })
            .collect::<Vec<_>>();
        let inventory_names = SYNTHESIS_CODE_INVENTORY_V2
            .iter()
            .map(|tag| tag.canonical_name().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(observed_wire_names, inventory_names);

        let mut fabricated_numeric = CanonicalEncoder::new();
        fabricated_numeric.u64(8);
        for tag in 0_u8..8 {
            fabricated_numeric.tag(tag);
        }
        assert_ne!(
            synthesis_code_inventory_digest_v2(),
            Digest::of_domain_bytes(
                "lambda-unit-synthesis-code-inventory-v2",
                fabricated_numeric.as_bytes(),
            ),
            "inventory digest must bind canonical constructor names, not only a numeric range"
        );
    }

    #[test]
    fn multiple_application_conversions_remain_under_one_aggregate_replay() {
        let fixture = fixture();
        let inner = Term::Apply {
            function: Box::new(Term::Global {
                id: fixture.function.clone(),
            }),
            argument: Box::new(Term::Unit),
        };
        let outer = Term::Apply {
            function: Box::new(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Var { index: 0 }),
            }),
            argument: Box::new(inner),
        };
        let code = propose_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &DependentContext::default(),
            &outer,
        )
        .expect("nested application code");
        let verified = verify_synthesis_code_v2(
            &fixture.kernel,
            &fixture.signature,
            &fixture.policy,
            &DependentContext::default(),
            &code,
        )
        .expect("complete aggregate replay includes nested conversions");
        assert_eq!(verified.normalized_type(), &Term::UnitType);
        assert!(
            !verified
                .aggregate_kernel_replay_digest()
                .as_str()
                .is_empty()
        );

        let mut observed_aggregate_boundary = false;
        for max_operations in 16..512 {
            let bounded = Kernel::new(KernelLimits {
                max_operations,
                max_depth: 64,
                normalization_fuel: 512,
            })
            .expect("bounded kernel");
            if matches!(
                verify_synthesis_code_v2(
                    &bounded,
                    &fixture.signature,
                    &fixture.policy,
                    &DependentContext::default(),
                    &code,
                ),
                Err(SynthesisV2Error::AggregateKernelReplay(
                    KernelError::ResourceExhausted(_)
                ))
            ) {
                observed_aggregate_boundary = true;
                break;
            }
        }
        assert!(
            observed_aggregate_boundary,
            "nested conversion inputs must participate in one final aggregate budget"
        );
    }
}
