//! Proof-carrying synthesis for the exact `pen-kernel` lambda/unit fragment.
//!
//! The mirror algorithm is not trusted. Every candidate type is normalized as
//! a kernel `TypeFormation` judgment, every term is replayed as `HasType`, and
//! the complete derivation is replayed once more under one aggregate kernel
//! budget before an opaque capability is returned.

#![forbid(unsafe_code)]

use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, Kernel, KernelError, OpenJudgment,
    Term, VerifiedContext, VerifiedSignature,
};
use thiserror::Error;

pub const SYNTHESIS_SCHEMA_VERSION_V1: u16 = 1;
pub const SYNTHESIS_PROTOCOL_ID_V1: &str = "pen-kernel-synthesis/lambda-unit/v1";

pub const SYNTHESIS_RULE_INVENTORY_V1: [SynthesisRuleV1; 8] = [
    SynthesisRuleV1::Sort,
    SynthesisRuleV1::UnitType,
    SynthesisRuleV1::Unit,
    SynthesisRuleV1::VariableLookup,
    SynthesisRuleV1::GlobalLookup,
    SynthesisRuleV1::PiFormation,
    SynthesisRuleV1::LambdaIntroduction,
    SynthesisRuleV1::ApplicationElimination,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SynthesisRuleV1 {
    Sort,
    UnitType,
    Unit,
    VariableLookup,
    GlobalLookup,
    PiFormation,
    LambdaIntroduction,
    ApplicationElimination,
}

impl CanonicalEncode for SynthesisRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Sort => 0,
            Self::UnitType => 1,
            Self::Unit => 2,
            Self::VariableLookup => 3,
            Self::GlobalLookup => 4,
            Self::PiFormation => 5,
            Self::LambdaIntroduction => 6,
            Self::ApplicationElimination => 7,
        });
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum SynthesisResourceKind {
    #[error("mirror operation budget")]
    Operations,
    #[error("mirror term depth")]
    Depth,
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum SynthesisError {
    #[error("unchanged kernel rejected a replay: {0}")]
    Kernel(#[from] KernelError),
    #[error("aggregate unchanged-kernel replay rejected the original inputs: {0}")]
    AggregateKernelReplay(KernelError),
    #[error("term or referenced type is outside the exact lambda/unit fragment")]
    OutsideLambdaUnitFragment,
    #[error("variable index is out of scope")]
    UnboundVariable,
    #[error("global identifier is absent from the verified signature")]
    UnknownGlobal,
    #[error("a universe was required by Pi or lambda formation")]
    ExpectedType,
    #[error("application function did not synthesize to a dependent product")]
    ExpectedFunction,
    #[error("capture-safe shift or substitution failed")]
    InvalidSubstitution,
    #[error("synthesis resource exhausted: {0}")]
    ResourceExhausted(SynthesisResourceKind),
    #[error("the aggregate kernel replay differed from an earlier normalized replay")]
    KernelReplayMismatch,
}

/// One original kernel request and its normalized checked output.
#[derive(Clone, Debug)]
pub struct VerifiedKernelReplayV1 {
    input: OpenJudgment,
    output: OpenJudgment,
}

impl VerifiedKernelReplayV1 {
    pub fn input(&self) -> &OpenJudgment {
        &self.input
    }

    pub fn output(&self) -> &OpenJudgment {
        &self.output
    }
}

impl CanonicalEncode for VerifiedKernelReplayV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.input.encode_canonical(encoder);
        self.output.encode_canonical(encoder);
    }
}

/// Opaque proof-producing derivation node. Private fields and the absence of
/// `Deserialize` prevent unverified wire data from becoming a capability.
#[derive(Clone, Debug)]
pub struct VerifiedSynthesisNodeV1 {
    signature_digest: Digest,
    term: Term,
    context: DependentContext,
    inferred_type: Term,
    rule: SynthesisRuleV1,
    premises: Vec<VerifiedSynthesisNodeV1>,
    kernel_replays: Vec<VerifiedKernelReplayV1>,
    kernel_replay_digest: Digest,
    digest: Digest,
}

impl VerifiedSynthesisNodeV1 {
    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn term(&self) -> &Term {
        &self.term
    }

    pub fn context(&self) -> &DependentContext {
        &self.context
    }

    pub fn inferred_type(&self) -> &Term {
        &self.inferred_type
    }

    pub fn rule(&self) -> SynthesisRuleV1 {
        self.rule
    }

    pub fn premises(&self) -> &[VerifiedSynthesisNodeV1] {
        &self.premises
    }

    pub fn kernel_replays(&self) -> &[VerifiedKernelReplayV1] {
        &self.kernel_replays
    }

    pub fn kernel_replay_digest(&self) -> &Digest {
        &self.kernel_replay_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Opaque synthesis capability, bound to both input and normalized replay
/// outputs as well as the synthesis and unchanged-kernel protocols.
#[derive(Clone, Debug)]
pub struct VerifiedSynthesizedJudgmentV1 {
    schema_version: u16,
    signature_digest: Digest,
    input_context: DependentContext,
    context_digest: Digest,
    term: Term,
    normalized_type: Term,
    derivation: VerifiedSynthesisNodeV1,
    aggregate_kernel_replay_digest: Digest,
    synthesis_protocol_digest: Digest,
    kernel_protocol_digest: Digest,
    digest: Digest,
}

impl VerifiedSynthesizedJudgmentV1 {
    pub fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn input_context(&self) -> &DependentContext {
        &self.input_context
    }

    pub fn context_digest(&self) -> &Digest {
        &self.context_digest
    }

    pub fn context(&self) -> &DependentContext {
        self.derivation.context()
    }

    pub fn term(&self) -> &Term {
        &self.term
    }

    pub fn normalized_type(&self) -> &Term {
        &self.normalized_type
    }

    pub fn derivation(&self) -> &VerifiedSynthesisNodeV1 {
        &self.derivation
    }

    pub fn aggregate_kernel_replay_digest(&self) -> &Digest {
        &self.aggregate_kernel_replay_digest
    }

    pub fn synthesis_protocol_digest(&self) -> &Digest {
        &self.synthesis_protocol_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

pub fn synthesis_rule_inventory_digest_v1() -> Digest {
    Digest::of_canonical(
        "lambda-unit-synthesis-rule-inventory-v1",
        &CanonicalRules(&SYNTHESIS_RULE_INVENTORY_V1),
    )
}

/// Source-tree identity for this proposed synthesis protocol. This is source
/// provenance, not a binary or build-environment attestation.
pub fn synthesis_protocol_digest_v1() -> Digest {
    let inventory = synthesis_rule_inventory_digest_v1();
    let canonical = [
        canonical_trusted_text(include_bytes!("lib.rs")),
        canonical_trusted_text(include_bytes!("../Cargo.toml")),
        canonical_trusted_text(include_bytes!("../Cargo.lock")),
        canonical_trusted_text(include_bytes!("../../../rust-toolchain.toml")),
        canonical_trusted_text(include_bytes!("../../../.cargo/config.toml")),
    ];
    let mut chunks = vec![
        SYNTHESIS_PROTOCOL_ID_V1.as_bytes(),
        inventory.as_str().as_bytes(),
    ];
    chunks.extend(canonical.iter().map(Vec::as_slice));
    Digest::of_domain_chunks("pen-kernel-synthesis-protocol-v1", &chunks)
}

pub fn synthesize_lambda_unit_v1(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &DependentContext,
    term: &Term,
) -> Result<VerifiedSynthesizedJudgmentV1, SynthesisError> {
    let mut budget = MirrorBudget::new(kernel);
    for entry in &context.0 {
        ensure_fragment(entry, &mut budget, 0)?;
    }
    let verified_context = kernel.verify_context(signature, context)?;
    for entry in verified_context.entries() {
        ensure_fragment(entry, &mut budget, 0)?;
    }
    let derivation = synthesize_node(kernel, signature, &verified_context, term, &mut budget, 0)?;

    let mut replays = Vec::new();
    collect_replays(&derivation, &mut replays);
    let replay_inputs = replays
        .iter()
        .map(VerifiedKernelReplayV1::input)
        .collect::<Vec<_>>();
    let aggregate = kernel
        .verify_open_judgments(signature, &replay_inputs)
        .map_err(SynthesisError::AggregateKernelReplay)?;
    if aggregate
        .iter()
        .zip(&replays)
        .any(|(actual, recorded)| actual != recorded.output())
    {
        return Err(SynthesisError::KernelReplayMismatch);
    }
    let aggregate_kernel_replay_digest = Digest::of_canonical(
        "lambda-unit-synthesis-aggregate-kernel-replay-v1",
        &CanonicalReplays(&replays),
    );

    let synthesis_protocol_digest = synthesis_protocol_digest_v1();
    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    let normalized_type = derivation.inferred_type.clone();
    let context_digest = verified_context.digest().clone();
    let signature_digest = signature.digest().clone();
    let digest = Digest::of_canonical(
        "verified-synthesized-judgment-v1",
        &CapabilityDigestInput {
            schema_version: SYNTHESIS_SCHEMA_VERSION_V1,
            signature_digest: &signature_digest,
            input_context: context,
            context_digest: &context_digest,
            term,
            normalized_type: &normalized_type,
            derivation_digest: derivation.digest(),
            aggregate_kernel_replay_digest: &aggregate_kernel_replay_digest,
            synthesis_protocol_digest: &synthesis_protocol_digest,
            kernel_protocol_digest: &kernel_protocol_digest,
        },
    );

    Ok(VerifiedSynthesizedJudgmentV1 {
        schema_version: SYNTHESIS_SCHEMA_VERSION_V1,
        signature_digest,
        input_context: context.clone(),
        context_digest,
        term: term.clone(),
        normalized_type,
        derivation,
        aggregate_kernel_replay_digest,
        synthesis_protocol_digest,
        kernel_protocol_digest,
        digest,
    })
}

fn synthesize_node(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &VerifiedContext,
    term: &Term,
    budget: &mut MirrorBudget,
    depth: u16,
) -> Result<VerifiedSynthesisNodeV1, SynthesisError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    match term {
        Term::Sort { level } => {
            let inferred = Term::Sort {
                level: level
                    .checked_add(1)
                    .ok_or(SynthesisError::Kernel(KernelError::UniverseOverflow))?,
            };
            finalize_node(
                kernel,
                signature,
                context,
                term,
                inferred,
                SynthesisRuleV1::Sort,
                Vec::new(),
                Vec::new(),
                budget,
                child_depth,
            )
        }
        Term::UnitType => finalize_node(
            kernel,
            signature,
            context,
            term,
            Term::Sort { level: 0 },
            SynthesisRuleV1::UnitType,
            Vec::new(),
            Vec::new(),
            budget,
            child_depth,
        ),
        Term::Unit => finalize_node(
            kernel,
            signature,
            context,
            term,
            Term::UnitType,
            SynthesisRuleV1::Unit,
            Vec::new(),
            Vec::new(),
            budget,
            child_depth,
        ),
        Term::Var { index } => {
            let distance = index
                .checked_add(1)
                .ok_or(SynthesisError::UnboundVariable)?;
            let distance_usize =
                usize::try_from(distance).map_err(|_| SynthesisError::UnboundVariable)?;
            let position = context
                .entries()
                .len()
                .checked_sub(distance_usize)
                .ok_or(SynthesisError::UnboundVariable)?;
            let inferred = shift_term(
                &context.entries()[position],
                i64::from(distance),
                0,
                budget,
                child_depth,
            )?;
            finalize_node(
                kernel,
                signature,
                context,
                term,
                inferred,
                SynthesisRuleV1::VariableLookup,
                Vec::new(),
                Vec::new(),
                budget,
                child_depth,
            )
        }
        Term::Global { id } => {
            let mut found = None;
            for declaration in signature.declarations() {
                budget.enter(child_depth)?;
                if declaration.id == *id {
                    found = Some(declaration);
                    break;
                }
            }
            let declaration = found.ok_or(SynthesisError::UnknownGlobal)?;
            ensure_fragment(&declaration.ty, budget, child_depth)?;
            finalize_node(
                kernel,
                signature,
                context,
                term,
                declaration.ty.clone(),
                SynthesisRuleV1::GlobalLookup,
                Vec::new(),
                Vec::new(),
                budget,
                child_depth,
            )
        }
        Term::Pi { parameter, body } => {
            let parameter_node =
                synthesize_node(kernel, signature, context, parameter, budget, child_depth)?;
            let parameter_level = expect_sort(parameter_node.inferred_type())?;
            let (normal_parameter, parameter_replay) =
                replay_type_formation(kernel, signature, context, parameter)?;
            let extended = extend_context(kernel, signature, context, normal_parameter)?;
            let body_node =
                synthesize_node(kernel, signature, &extended, body, budget, child_depth)?;
            let body_level = expect_sort(body_node.inferred_type())?;
            finalize_node(
                kernel,
                signature,
                context,
                term,
                Term::Sort {
                    level: parameter_level.max(body_level),
                },
                SynthesisRuleV1::PiFormation,
                vec![parameter_node, body_node],
                vec![parameter_replay],
                budget,
                child_depth,
            )
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            let parameter_node = synthesize_node(
                kernel,
                signature,
                context,
                parameter_type,
                budget,
                child_depth,
            )?;
            expect_sort(parameter_node.inferred_type())?;
            let (normal_parameter, parameter_replay) =
                replay_type_formation(kernel, signature, context, parameter_type)?;
            let extended = extend_context(kernel, signature, context, normal_parameter.clone())?;
            let body_node =
                synthesize_node(kernel, signature, &extended, body, budget, child_depth)?;
            let inferred = Term::Pi {
                parameter: Box::new(normal_parameter),
                body: Box::new(body_node.inferred_type().clone()),
            };
            finalize_node(
                kernel,
                signature,
                context,
                term,
                inferred,
                SynthesisRuleV1::LambdaIntroduction,
                vec![parameter_node, body_node],
                vec![parameter_replay],
                budget,
                child_depth,
            )
        }
        Term::Apply { function, argument } => {
            let function_node =
                synthesize_node(kernel, signature, context, function, budget, child_depth)?;
            let (normal_function_type, function_type_replay) =
                replay_type_formation(kernel, signature, context, function_node.inferred_type())?;
            let Term::Pi { parameter, body } = normal_function_type else {
                return Err(SynthesisError::ExpectedFunction);
            };
            let argument_check = replay_has_type(kernel, signature, context, argument, &parameter)?;
            let argument_node =
                synthesize_node(kernel, signature, context, argument, budget, child_depth)?;
            if normalized_judgment_type(&argument_check) != Some(argument_node.inferred_type()) {
                return Err(SynthesisError::KernelReplayMismatch);
            }
            let inferred = substitute_top(&body, argument, budget, child_depth)?;
            finalize_node(
                kernel,
                signature,
                context,
                term,
                inferred,
                SynthesisRuleV1::ApplicationElimination,
                vec![function_node, argument_node],
                vec![function_type_replay, argument_check],
                budget,
                child_depth,
            )
        }
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            Err(SynthesisError::OutsideLambdaUnitFragment)
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn finalize_node(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &VerifiedContext,
    term: &Term,
    inferred_type: Term,
    rule: SynthesisRuleV1,
    premises: Vec<VerifiedSynthesisNodeV1>,
    mut kernel_replays: Vec<VerifiedKernelReplayV1>,
    budget: &mut MirrorBudget,
    depth: u16,
) -> Result<VerifiedSynthesisNodeV1, SynthesisError> {
    ensure_fragment(&inferred_type, budget, depth)?;
    let (inferred_type, formation) =
        replay_type_formation(kernel, signature, context, &inferred_type)?;
    ensure_fragment(&inferred_type, budget, depth)?;
    let has_type = replay_has_type(kernel, signature, context, term, &inferred_type)?;
    ensure_replay_fragment(formation.output(), budget, depth)?;
    ensure_replay_fragment(has_type.output(), budget, depth)?;
    kernel_replays.push(formation);
    kernel_replays.push(has_type);
    let kernel_replay_digest = Digest::of_canonical(
        "lambda-unit-synthesis-node-kernel-replay-v1",
        &CanonicalReplays(&kernel_replays),
    );
    let context_wire = context.normalized_wire();
    let signature_digest = signature.digest().clone();
    let premise_digests = premises
        .iter()
        .map(|premise| premise.digest().clone())
        .collect::<Vec<_>>();
    let digest = Digest::of_canonical(
        "verified-synthesis-node-v1",
        &NodeDigestInput {
            signature_digest: &signature_digest,
            term,
            context: &context_wire,
            inferred_type: &inferred_type,
            rule,
            premise_digests: &premise_digests,
            kernel_replays: &kernel_replays,
            kernel_replay_digest: &kernel_replay_digest,
        },
    );
    Ok(VerifiedSynthesisNodeV1 {
        signature_digest,
        term: term.clone(),
        context: context_wire,
        inferred_type,
        rule,
        premises,
        kernel_replays,
        kernel_replay_digest,
        digest,
    })
}

fn replay_type_formation(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &VerifiedContext,
    term: &Term,
) -> Result<(Term, VerifiedKernelReplayV1), SynthesisError> {
    let input = OpenJudgment::TypeFormation {
        context: context.normalized_wire(),
        term: term.clone(),
    };
    let output = kernel.verify_open_judgment(signature, &input)?;
    let OpenJudgment::TypeFormation { term, .. } = &output else {
        return Err(SynthesisError::KernelReplayMismatch);
    };
    Ok((term.clone(), VerifiedKernelReplayV1 { input, output }))
}

fn replay_has_type(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &VerifiedContext,
    term: &Term,
    ty: &Term,
) -> Result<VerifiedKernelReplayV1, SynthesisError> {
    let input = OpenJudgment::HasType {
        context: context.normalized_wire(),
        term: term.clone(),
        ty: ty.clone(),
    };
    let output = kernel.verify_open_judgment(signature, &input)?;
    Ok(VerifiedKernelReplayV1 { input, output })
}

fn normalized_judgment_type(replay: &VerifiedKernelReplayV1) -> Option<&Term> {
    match replay.output() {
        OpenJudgment::HasType { ty, .. } => Some(ty),
        _ => None,
    }
}

fn extend_context(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &VerifiedContext,
    entry: Term,
) -> Result<VerifiedContext, SynthesisError> {
    let mut entries = context.entries().to_vec();
    entries.push(entry);
    Ok(kernel.verify_context(signature, &DependentContext(entries))?)
}

fn expect_sort(term: &Term) -> Result<u16, SynthesisError> {
    match term {
        Term::Sort { level } => Ok(*level),
        _ => Err(SynthesisError::ExpectedType),
    }
}

fn ensure_fragment(
    term: &Term,
    budget: &mut MirrorBudget,
    depth: u16,
) -> Result<(), SynthesisError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    match term {
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(()),
        Term::Pi { parameter, body } => {
            ensure_fragment(parameter, budget, child_depth)?;
            ensure_fragment(body, budget, child_depth)
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            ensure_fragment(parameter_type, budget, child_depth)?;
            ensure_fragment(body, budget, child_depth)
        }
        Term::Apply { function, argument } => {
            ensure_fragment(function, budget, child_depth)?;
            ensure_fragment(argument, budget, child_depth)
        }
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            Err(SynthesisError::OutsideLambdaUnitFragment)
        }
    }
}

fn ensure_replay_fragment(
    judgment: &OpenJudgment,
    budget: &mut MirrorBudget,
    depth: u16,
) -> Result<(), SynthesisError> {
    for entry in &judgment.context().0 {
        ensure_fragment(entry, budget, depth)?;
    }
    match judgment {
        OpenJudgment::TypeFormation { term, .. } => ensure_fragment(term, budget, depth),
        OpenJudgment::HasType { term, ty, .. } => {
            ensure_fragment(term, budget, depth)?;
            ensure_fragment(ty, budget, depth)
        }
        OpenJudgment::DefinitionallyEqual {
            left, right, ty, ..
        } => {
            ensure_fragment(left, budget, depth)?;
            ensure_fragment(right, budget, depth)?;
            ensure_fragment(ty, budget, depth)
        }
    }
}

fn substitute_top(
    body: &Term,
    replacement: &Term,
    budget: &mut MirrorBudget,
    depth: u16,
) -> Result<Term, SynthesisError> {
    let lifted = shift_term(replacement, 1, 0, budget, depth)?;
    let replaced = substitute_term(body, 0, &lifted, 0, budget, depth)?;
    shift_term(&replaced, -1, 0, budget, depth)
}

fn substitute_term(
    term: &Term,
    target: u32,
    replacement: &Term,
    binder_depth: u32,
    budget: &mut MirrorBudget,
    depth: u16,
) -> Result<Term, SynthesisError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    let sought = target
        .checked_add(binder_depth)
        .ok_or(SynthesisError::InvalidSubstitution)?;
    match term {
        Term::Var { index } if *index == sought => {
            shift_term(replacement, i64::from(binder_depth), 0, budget, child_depth)
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(term.clone()),
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(substitute_term(
                parameter,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
            body: Box::new(substitute_term(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or(SynthesisError::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(substitute_term(
                parameter_type,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
            body: Box::new(substitute_term(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or(SynthesisError::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(substitute_term(
                function,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
            argument: Box::new(substitute_term(
                argument,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
        }),
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            Err(SynthesisError::OutsideLambdaUnitFragment)
        }
    }
}

fn shift_term(
    term: &Term,
    amount: i64,
    cutoff: u32,
    budget: &mut MirrorBudget,
    depth: u16,
) -> Result<Term, SynthesisError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    match term {
        Term::Var { index } if *index >= cutoff => {
            let shifted = i64::from(*index)
                .checked_add(amount)
                .ok_or(SynthesisError::InvalidSubstitution)?;
            Ok(Term::Var {
                index: u32::try_from(shifted).map_err(|_| SynthesisError::InvalidSubstitution)?,
            })
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(term.clone()),
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(shift_term(parameter, amount, cutoff, budget, child_depth)?),
            body: Box::new(shift_term(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(SynthesisError::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(shift_term(
                parameter_type,
                amount,
                cutoff,
                budget,
                child_depth,
            )?),
            body: Box::new(shift_term(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(SynthesisError::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(shift_term(function, amount, cutoff, budget, child_depth)?),
            argument: Box::new(shift_term(argument, amount, cutoff, budget, child_depth)?),
        }),
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            Err(SynthesisError::OutsideLambdaUnitFragment)
        }
    }
}

fn collect_replays(node: &VerifiedSynthesisNodeV1, output: &mut Vec<VerifiedKernelReplayV1>) {
    output.extend(node.kernel_replays.iter().cloned());
    for premise in &node.premises {
        collect_replays(premise, output);
    }
}

fn next_depth(depth: u16) -> Result<u16, SynthesisError> {
    depth
        .checked_add(1)
        .ok_or(SynthesisError::ResourceExhausted(
            SynthesisResourceKind::Depth,
        ))
}

#[derive(Clone, Copy, Debug)]
struct MirrorBudget {
    operations_left: u32,
    depth_limit: u16,
}

impl MirrorBudget {
    fn new(kernel: &Kernel) -> Self {
        let limits = kernel.limits();
        Self {
            operations_left: limits.max_operations,
            depth_limit: limits.max_depth,
        }
    }

    fn enter(&mut self, depth: u16) -> Result<(), SynthesisError> {
        if depth > self.depth_limit {
            return Err(SynthesisError::ResourceExhausted(
                SynthesisResourceKind::Depth,
            ));
        }
        self.operations_left =
            self.operations_left
                .checked_sub(1)
                .ok_or(SynthesisError::ResourceExhausted(
                    SynthesisResourceKind::Operations,
                ))?;
        Ok(())
    }
}

struct CanonicalRules<'a>(&'a [SynthesisRuleV1]);

impl CanonicalEncode for CanonicalRules<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

struct CanonicalReplays<'a>(&'a [VerifiedKernelReplayV1]);

impl CanonicalEncode for CanonicalReplays<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(self.0);
    }
}

struct NodeDigestInput<'a> {
    signature_digest: &'a Digest,
    term: &'a Term,
    context: &'a DependentContext,
    inferred_type: &'a Term,
    rule: SynthesisRuleV1,
    premise_digests: &'a [Digest],
    kernel_replays: &'a [VerifiedKernelReplayV1],
    kernel_replay_digest: &'a Digest,
}

impl CanonicalEncode for NodeDigestInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.signature_digest.encode_canonical(encoder);
        self.term.encode_canonical(encoder);
        self.context.encode_canonical(encoder);
        self.inferred_type.encode_canonical(encoder);
        self.rule.encode_canonical(encoder);
        encoder.sequence(self.premise_digests);
        encoder.sequence(self.kernel_replays);
        self.kernel_replay_digest.encode_canonical(encoder);
    }
}

struct CapabilityDigestInput<'a> {
    schema_version: u16,
    signature_digest: &'a Digest,
    input_context: &'a DependentContext,
    context_digest: &'a Digest,
    term: &'a Term,
    normalized_type: &'a Term,
    derivation_digest: &'a Digest,
    aggregate_kernel_replay_digest: &'a Digest,
    synthesis_protocol_digest: &'a Digest,
    kernel_protocol_digest: &'a Digest,
}

impl CanonicalEncode for CapabilityDigestInput<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.signature_digest.encode_canonical(encoder);
        self.input_context.encode_canonical(encoder);
        self.context_digest.encode_canonical(encoder);
        self.term.encode_canonical(encoder);
        self.normalized_type.encode_canonical(encoder);
        self.derivation_digest.encode_canonical(encoder);
        self.aggregate_kernel_replay_digest
            .encode_canonical(encoder);
        self.synthesis_protocol_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
    }
}

fn canonical_trusted_text(bytes: &[u8]) -> Vec<u8> {
    let text = std::str::from_utf8(bytes).expect("trusted synthesis source must be UTF-8");
    let canonical = text.replace("\r\n", "\n");
    assert!(
        !canonical.contains('\r'),
        "trusted synthesis source contains a bare carriage return"
    );
    canonical.into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_kernel::{Declaration, GlobalId, KernelLimits, UncheckedSignature};

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("safe kernel")
    }

    fn empty_signature(kernel: &Kernel) -> VerifiedSignature {
        kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature")
    }

    #[test]
    fn primitives_and_rules_replay() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let cases = [
            (
                Term::Sort { level: 0 },
                Term::Sort { level: 1 },
                SynthesisRuleV1::Sort,
            ),
            (
                Term::UnitType,
                Term::Sort { level: 0 },
                SynthesisRuleV1::UnitType,
            ),
            (Term::Unit, Term::UnitType, SynthesisRuleV1::Unit),
        ];
        for (term, expected, rule) in cases {
            let proof =
                synthesize_lambda_unit_v1(&kernel, &signature, &DependentContext::default(), &term)
                    .expect("primitive synthesis");
            assert_eq!(proof.normalized_type(), &expected);
            assert_eq!(proof.derivation().rule(), rule);
            assert_eq!(
                proof.kernel_protocol_digest(),
                &kernel.kernel_protocol_digest()
            );
            assert_eq!(
                proof.synthesis_protocol_digest(),
                &synthesis_protocol_digest_v1()
            );
        }
    }

    #[test]
    fn variable_lookup_uses_exact_de_bruijn_distance_shift() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let context = DependentContext(vec![Term::Sort { level: 0 }, Term::Var { index: 0 }]);
        let newest =
            synthesize_lambda_unit_v1(&kernel, &signature, &context, &Term::Var { index: 0 })
                .expect("dependent newest variable");
        assert_eq!(newest.normalized_type(), &Term::Var { index: 1 });
        let oldest =
            synthesize_lambda_unit_v1(&kernel, &signature, &context, &Term::Var { index: 1 })
                .expect("oldest variable");
        assert_eq!(oldest.normalized_type(), &Term::Sort { level: 0 });
    }

    #[test]
    fn pi_lambda_global_and_dependent_application_replay() {
        let kernel = kernel();
        let function = GlobalId(Digest::of_bytes(b"dependent-global"));
        let function_type = Term::Pi {
            parameter: Box::new(Term::Sort { level: 0 }),
            body: Box::new(Term::Var { index: 0 }),
        };
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: function.clone(),
                    ty: function_type.clone(),
                    body: None,
                }],
            })
            .expect("dependent global");

        let global = synthesize_lambda_unit_v1(
            &kernel,
            &signature,
            &DependentContext::default(),
            &Term::Global {
                id: function.clone(),
            },
        )
        .expect("global lookup");
        assert_eq!(global.normalized_type(), &function_type);
        assert_eq!(global.derivation().rule(), SynthesisRuleV1::GlobalLookup);

        let application = synthesize_lambda_unit_v1(
            &kernel,
            &signature,
            &DependentContext::default(),
            &Term::Apply {
                function: Box::new(Term::Global { id: function }),
                argument: Box::new(Term::UnitType),
            },
        )
        .expect("dependent application");
        assert_eq!(application.normalized_type(), &Term::UnitType);
        assert_eq!(
            application.derivation().rule(),
            SynthesisRuleV1::ApplicationElimination
        );

        let lambda = synthesize_lambda_unit_v1(
            &kernel,
            &signature,
            &DependentContext::default(),
            &Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Var { index: 0 }),
            },
        )
        .expect("identity lambda");
        assert_eq!(
            lambda.normalized_type(),
            &Term::Pi {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            }
        );
    }

    #[test]
    fn variable_in_function_position_is_synthesized() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let context = DependentContext(vec![Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::UnitType),
        }]);
        let proof = synthesize_lambda_unit_v1(
            &kernel,
            &signature,
            &context,
            &Term::Apply {
                function: Box::new(Term::Var { index: 0 }),
                argument: Box::new(Term::Unit),
            },
        )
        .expect("variable-headed application");
        assert_eq!(proof.normalized_type(), &Term::UnitType);
        assert_eq!(
            proof.derivation().premises()[0].rule(),
            SynthesisRuleV1::VariableLookup
        );
    }

    #[test]
    fn rejects_unsupported_unbound_unknown_and_bad_application() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let unsupported = Term::Sigma {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::UnitType),
        };
        assert_eq!(
            synthesize_lambda_unit_v1(
                &kernel,
                &signature,
                &DependentContext::default(),
                &unsupported,
            )
            .expect_err("unsupported syntax"),
            SynthesisError::OutsideLambdaUnitFragment
        );
        assert_eq!(
            synthesize_lambda_unit_v1(
                &kernel,
                &signature,
                &DependentContext::default(),
                &Term::Var { index: 0 },
            )
            .expect_err("unbound variable"),
            SynthesisError::UnboundVariable
        );
        assert_eq!(
            synthesize_lambda_unit_v1(
                &kernel,
                &signature,
                &DependentContext::default(),
                &Term::Global {
                    id: GlobalId(Digest::of_bytes(b"missing")),
                },
            )
            .expect_err("unknown global"),
            SynthesisError::UnknownGlobal
        );
        assert_eq!(
            synthesize_lambda_unit_v1(
                &kernel,
                &signature,
                &DependentContext::default(),
                &Term::Apply {
                    function: Box::new(Term::Unit),
                    argument: Box::new(Term::Unit),
                },
            )
            .expect_err("non-function application"),
            SynthesisError::ExpectedFunction
        );
    }

    #[test]
    fn transparent_global_cannot_normalize_outside_the_fragment() {
        let kernel = kernel();
        let alias = GlobalId(Digest::of_bytes(b"transparent-sigma-alias"));
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: alias.clone(),
                    ty: Term::Sort { level: 0 },
                    body: Some(Term::Sigma {
                        parameter: Box::new(Term::UnitType),
                        body: Box::new(Term::UnitType),
                    }),
                }],
            })
            .expect("kernel supports the broader Sigma fragment");
        assert_eq!(
            synthesize_lambda_unit_v1(
                &kernel,
                &signature,
                &DependentContext::default(),
                &Term::Global { id: alias },
            )
            .expect_err("normalized replay must remain in the exact fragment"),
            SynthesisError::OutsideLambdaUnitFragment
        );
    }

    #[test]
    fn raw_context_cannot_normalize_projection_syntax_into_the_fragment() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        let sigma = Term::Sigma {
            parameter: Box::new(Term::Sort { level: 0 }),
            body: Box::new(Term::Sort { level: 0 }),
        };
        let context = DependentContext(vec![Term::First {
            pair: Box::new(Term::Pair {
                sigma_type: Box::new(sigma),
                first: Box::new(Term::UnitType),
                second: Box::new(Term::UnitType),
            }),
        }]);
        assert_eq!(
            synthesize_lambda_unit_v1(&kernel, &signature, &context, &Term::Unit)
                .expect_err("raw context syntax is part of the fragment boundary"),
            SynthesisError::OutsideLambdaUnitFragment
        );
    }

    #[test]
    fn aggregate_replays_original_inputs_under_one_budget() {
        let kernel = Kernel::new(KernelLimits {
            max_operations: 8,
            max_depth: 16,
            normalization_fuel: 16,
        })
        .expect("bounded kernel");
        let signature = empty_signature(&kernel);
        assert!(matches!(
            synthesize_lambda_unit_v1(
                &kernel,
                &signature,
                &DependentContext::default(),
                &Term::Unit,
            ),
            Err(SynthesisError::AggregateKernelReplay(
                KernelError::ResourceExhausted(_)
            ))
        ));
    }

    #[test]
    fn malformed_context_and_wrong_argument_are_kernel_rejections() {
        let kernel = kernel();
        let signature = empty_signature(&kernel);
        assert_eq!(
            synthesize_lambda_unit_v1(
                &kernel,
                &signature,
                &DependentContext(vec![Term::Var { index: 0 }]),
                &Term::Unit,
            )
            .expect_err("malformed context"),
            SynthesisError::Kernel(KernelError::UnboundVariable)
        );
        let bad = synthesize_lambda_unit_v1(
            &kernel,
            &signature,
            &DependentContext::default(),
            &Term::Apply {
                function: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::UnitType),
                    body: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::UnitType),
            },
        );
        assert_eq!(
            bad.expect_err("wrong argument type"),
            SynthesisError::Kernel(KernelError::TypeMismatch)
        );
    }

    #[test]
    fn mirror_budget_fails_closed_before_unbounded_premise_growth() {
        let kernel = Kernel::new(KernelLimits {
            max_operations: 5,
            max_depth: 4,
            normalization_fuel: 32,
        })
        .expect("small kernel");
        let signature = empty_signature(&kernel);
        let deep = Term::Lambda {
            parameter_type: Box::new(Term::UnitType),
            body: Box::new(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::UnitType),
                    body: Box::new(Term::Unit),
                }),
            }),
        };
        assert!(
            synthesize_lambda_unit_v1(&kernel, &signature, &DependentContext::default(), &deep)
                .is_err()
        );
    }
}
