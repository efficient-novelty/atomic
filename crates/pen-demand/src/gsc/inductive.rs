use super::family::{GscOriginEventId, PublicSourceId};
use super::manifest::{GscOutcome, GscUnknownReason, VerifiedGscSemanticManifest};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Kernel, OpenJudgment,
    Term, VerifiedSignature,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION: u16 = 1;
pub const CLOSED_FORMER_FRAME_SCHEMA_VERSION: u16 = 1;

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct FormerCodeId(pub Digest);

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct ConstructorPortId(pub Digest);

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct ClosedFormerFrameId(pub Digest);

impl CanonicalEncode for FormerCodeId {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

impl CanonicalEncode for ConstructorPortId {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

impl CanonicalEncode for ClosedFormerFrameId {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "form", rename_all = "snake_case", deny_unknown_fields)]
pub enum CodeTerm {
    Sort {
        level: u16,
    },
    Bound {
        index: u32,
    },
    Public {
        id: GlobalId,
    },
    DependentFunction {
        parameter: Box<CodeTerm>,
        body: Box<CodeTerm>,
    },
    Apply {
        function: Box<CodeTerm>,
        argument: Box<CodeTerm>,
    },
    OwnerApp {
        parameters: Vec<CodeTerm>,
        indices: Vec<CodeTerm>,
    },
}

impl CanonicalEncode for CodeTerm {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Sort { level } => {
                encoder.tag(0);
                encoder.u16(*level);
            }
            Self::Bound { index } => {
                encoder.tag(1);
                encoder.u32(*index);
            }
            Self::Public { id } => {
                encoder.tag(2);
                id.encode_canonical(encoder);
            }
            Self::DependentFunction { parameter, body } => {
                encoder.tag(3);
                parameter.encode_canonical(encoder);
                body.encode_canonical(encoder);
            }
            Self::Apply { function, argument } => {
                encoder.tag(4);
                function.encode_canonical(encoder);
                argument.encode_canonical(encoder);
            }
            Self::OwnerApp {
                parameters,
                indices,
            } => {
                encoder.tag(5);
                encoder.sequence(parameters);
                encoder.sequence(indices);
            }
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(transparent)]
pub struct TelescopeCode(pub Vec<CodeTerm>);

impl CanonicalEncode for TelescopeCode {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.0);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "boundary", rename_all = "snake_case", deny_unknown_fields)]
pub enum BoundaryPortCode {
    Path { left: CodeTerm, right: CodeTerm },
}

impl CanonicalEncode for BoundaryPortCode {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Path { left, right } => {
                encoder.tag(0);
                left.encode_canonical(encoder);
                right.encode_canonical(encoder);
            }
        }
    }
}

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ComputationMode {
    JudgmentalFreshHead,
    PathTerm,
}

impl CanonicalEncode for ComputationMode {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::JudgmentalFreshHead => 0,
            Self::PathTerm => 1,
        });
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConstructorCode {
    pub arguments: TelescopeCode,
    pub result_indices: Vec<CodeTerm>,
    pub recursive_positions: Vec<u16>,
    pub boundary_ports: Vec<BoundaryPortCode>,
}

impl CanonicalEncode for ConstructorCode {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.arguments.encode_canonical(encoder);
        encoder.sequence(&self.result_indices);
        encoder.u64(self.recursive_positions.len() as u64);
        for position in &self.recursive_positions {
            encoder.u16(*position);
        }
        encoder.sequence(&self.boundary_ports);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedInductiveCode {
    pub schema_version: u16,
    pub universe_level: u16,
    pub parameters: TelescopeCode,
    pub indices: TelescopeCode,
    pub constructors: Vec<ConstructorCode>,
    pub generated_computation_mode: ComputationMode,
}

impl CanonicalEncode for ClosedInductiveCode {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        encoder.u16(self.universe_level);
        self.parameters.encode_canonical(encoder);
        self.indices.encode_canonical(encoder);
        encoder.sequence(&self.constructors);
        self.generated_computation_mode.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedClosedInductiveCode {
    code: ClosedInductiveCode,
    id: FormerCodeId,
    constructor_ids: Vec<ConstructorPortId>,
    semantic_manifest_digest: Digest,
}

impl VerifiedClosedInductiveCode {
    pub fn code(&self) -> &ClosedInductiveCode {
        &self.code
    }

    pub fn id(&self) -> &FormerCodeId {
        &self.id
    }

    pub fn constructor_ids(&self) -> &[ConstructorPortId] {
        &self.constructor_ids
    }

    pub fn constructor_id(&self, index: usize) -> Option<&ConstructorPortId> {
        self.constructor_ids.get(index)
    }

    pub fn manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn is_one_nullary(&self) -> bool {
        self.code.parameters.0.is_empty()
            && self.code.indices.0.is_empty()
            && self.code.constructors.len() == 1
            && self.code.constructors[0].arguments.0.is_empty()
            && self.code.constructors[0].result_indices.is_empty()
            && self.code.constructors[0].recursive_positions.is_empty()
            && self.code.constructors[0].boundary_ports.is_empty()
            && self.code.generated_computation_mode == ComputationMode::JudgmentalFreshHead
    }

    pub fn supports_core_compiler(&self) -> bool {
        self.code.parameters.0.is_empty()
            && self.code.indices.0.is_empty()
            && self
                .code
                .constructors
                .iter()
                .all(|constructor| constructor.boundary_ports.is_empty())
            && self.code.generated_computation_mode == ComputationMode::JudgmentalFreshHead
    }
}

pub fn verify_closed_inductive_code(
    manifest: &VerifiedGscSemanticManifest,
    code: &ClosedInductiveCode,
) -> GscOutcome<VerifiedClosedInductiveCode> {
    if code.schema_version != CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
    }
    let limits = manifest.limits();
    if code.universe_level > limits.max_universe_level
        || code.parameters.0.len() > usize::from(limits.max_parameters)
        || code.indices.0.len() > usize::from(limits.max_indices)
        || code.constructors.is_empty()
        || code.constructors.len() > usize::from(limits.max_constructors)
    {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
    }

    let mut budget = CodeBudget::new(limits.max_code_nodes);
    let parameter_count = code.parameters.0.len();
    let index_count = code.indices.0.len();
    for (position, ty) in code.parameters.0.iter().enumerate() {
        let owner_shape = match validate_code_term(
            ty,
            position,
            parameter_count,
            index_count,
            limits.max_code_term_depth,
            limits.max_universe_level,
            0,
            &mut budget,
        ) {
            Ok(owner_shape) => owner_shape,
            Err(failure) => return GscOutcome::Unknown(failure.unknown_reason()),
        };
        if owner_shape != OwnerShape::Absent {
            return GscOutcome::Unknown(GscUnknownReason::MalformedCode);
        }
    }
    for (position, ty) in code.indices.0.iter().enumerate() {
        let owner_shape = match validate_code_term(
            ty,
            parameter_count + position,
            parameter_count,
            index_count,
            limits.max_code_term_depth,
            limits.max_universe_level,
            0,
            &mut budget,
        ) {
            Ok(owner_shape) => owner_shape,
            Err(failure) => return GscOutcome::Unknown(failure.unknown_reason()),
        };
        if owner_shape != OwnerShape::Absent {
            return GscOutcome::Unknown(GscUnknownReason::MalformedCode);
        }
    }

    for constructor in &code.constructors {
        if constructor.arguments.0.len() > usize::from(limits.max_constructor_arguments)
            || constructor.boundary_ports.len() > usize::from(limits.max_boundary_ports)
            || constructor.result_indices.len() != index_count
            || !strictly_sorted(&constructor.recursive_positions)
        {
            return GscOutcome::Unknown(GscUnknownReason::MalformedCode);
        }
        let base_scope = parameter_count + index_count;
        let mut derived_recursive = Vec::new();
        for (position, argument) in constructor.arguments.0.iter().enumerate() {
            let owner_shape = match validate_code_term(
                argument,
                base_scope + position,
                parameter_count,
                index_count,
                limits.max_code_term_depth,
                limits.max_universe_level,
                0,
                &mut budget,
            ) {
                Ok(owner_shape) => owner_shape,
                Err(failure) => return GscOutcome::Unknown(failure.unknown_reason()),
            };
            match owner_shape {
                OwnerShape::Absent => {}
                OwnerShape::Whole => derived_recursive.push(position as u16),
                OwnerShape::Nested => {
                    return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
                }
            }
        }
        if derived_recursive != constructor.recursive_positions {
            return GscOutcome::Unknown(GscUnknownReason::MalformedCode);
        }
        let result_scope = base_scope + constructor.arguments.0.len();
        for index in &constructor.result_indices {
            let owner_shape = match validate_code_term(
                index,
                result_scope,
                parameter_count,
                index_count,
                limits.max_code_term_depth,
                limits.max_universe_level,
                0,
                &mut budget,
            ) {
                Ok(owner_shape) => owner_shape,
                Err(failure) => return GscOutcome::Unknown(failure.unknown_reason()),
            };
            if owner_shape != OwnerShape::Absent {
                return GscOutcome::Unknown(GscUnknownReason::MalformedCode);
            }
        }
        for boundary in &constructor.boundary_ports {
            match boundary {
                BoundaryPortCode::Path { left, right } => {
                    if let Err(failure) = validate_code_term(
                        left,
                        result_scope,
                        parameter_count,
                        index_count,
                        limits.max_code_term_depth,
                        limits.max_universe_level,
                        0,
                        &mut budget,
                    ) {
                        return GscOutcome::Unknown(failure.unknown_reason());
                    }
                    if let Err(failure) = validate_code_term(
                        right,
                        result_scope,
                        parameter_count,
                        index_count,
                        limits.max_code_term_depth,
                        limits.max_universe_level,
                        0,
                        &mut budget,
                    ) {
                        return GscOutcome::Unknown(failure.unknown_reason());
                    }
                }
            }
        }
    }

    let id = former_code_id(manifest.digest(), code);
    let constructor_ids = (0..code.constructors.len())
        .map(|index| constructor_port_id(&id, index as u32))
        .collect();
    GscOutcome::Proven(VerifiedClosedInductiveCode {
        code: code.clone(),
        id,
        constructor_ids,
        semantic_manifest_digest: manifest.digest().clone(),
    })
}

fn former_code_id(manifest: &Digest, code: &ClosedInductiveCode) -> FormerCodeId {
    let mut encoder = CanonicalEncoder::new();
    manifest.encode_canonical(&mut encoder);
    code.encode_canonical(&mut encoder);
    FormerCodeId(Digest::of_domain_bytes(
        "pen-demand/gsc-former-code-id/v1",
        encoder.as_bytes(),
    ))
}

fn constructor_port_id(former: &FormerCodeId, index: u32) -> ConstructorPortId {
    let mut encoder = CanonicalEncoder::new();
    former.encode_canonical(&mut encoder);
    encoder.u32(index);
    ConstructorPortId(Digest::of_domain_bytes(
        "pen-demand/gsc-constructor-port-id/v1",
        encoder.as_bytes(),
    ))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OwnerShape {
    Absent,
    Whole,
    Nested,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CodeValidationFailure {
    ResourceExhausted,
    Malformed,
}

impl CodeValidationFailure {
    fn unknown_reason(self) -> GscUnknownReason {
        match self {
            Self::ResourceExhausted => GscUnknownReason::ResourceExhausted,
            Self::Malformed => GscUnknownReason::MalformedCode,
        }
    }
}

/// Performs scope validation and owner detection in the same bounded walk.
///
/// Keeping these checks together is important: a separate recursive owner
/// scan could overflow the stack before the manifest depth/node bounds were
/// applied.
#[allow(clippy::too_many_arguments)]
fn validate_code_term(
    term: &CodeTerm,
    scope: usize,
    parameter_count: usize,
    index_count: usize,
    max_depth: u16,
    max_universe_level: u16,
    depth: u16,
    budget: &mut CodeBudget,
) -> Result<OwnerShape, CodeValidationFailure> {
    if depth > max_depth || !budget.charge() {
        return Err(CodeValidationFailure::ResourceExhausted);
    }
    let child_depth = depth
        .checked_add(1)
        .ok_or(CodeValidationFailure::ResourceExhausted)?;
    match term {
        CodeTerm::Sort { level } => {
            if *level <= max_universe_level {
                Ok(OwnerShape::Absent)
            } else {
                Err(CodeValidationFailure::Malformed)
            }
        }
        CodeTerm::Public { .. } => Ok(OwnerShape::Absent),
        CodeTerm::Bound { index } => {
            if (*index as usize) < scope {
                Ok(OwnerShape::Absent)
            } else {
                Err(CodeValidationFailure::Malformed)
            }
        }
        CodeTerm::DependentFunction { parameter, body } => {
            let parameter_shape = validate_code_term(
                parameter,
                scope,
                parameter_count,
                index_count,
                max_depth,
                max_universe_level,
                child_depth,
                budget,
            )?;
            let body_shape = validate_code_term(
                body,
                scope
                    .checked_add(1)
                    .ok_or(CodeValidationFailure::ResourceExhausted)?,
                parameter_count,
                index_count,
                max_depth,
                max_universe_level,
                child_depth,
                budget,
            )?;
            Ok(nested_owner_shape(parameter_shape, body_shape))
        }
        CodeTerm::Apply { function, argument } => {
            let function_shape = validate_code_term(
                function,
                scope,
                parameter_count,
                index_count,
                max_depth,
                max_universe_level,
                child_depth,
                budget,
            )?;
            let argument_shape = validate_code_term(
                argument,
                scope,
                parameter_count,
                index_count,
                max_depth,
                max_universe_level,
                child_depth,
                budget,
            )?;
            Ok(nested_owner_shape(function_shape, argument_shape))
        }
        CodeTerm::OwnerApp {
            parameters,
            indices,
        } => {
            if parameters.len() != parameter_count || indices.len() != index_count {
                return Err(CodeValidationFailure::Malformed);
            }
            for argument in parameters.iter().chain(indices) {
                if validate_code_term(
                    argument,
                    scope,
                    parameter_count,
                    index_count,
                    max_depth,
                    max_universe_level,
                    child_depth,
                    budget,
                )? != OwnerShape::Absent
                {
                    return Err(CodeValidationFailure::Malformed);
                }
            }
            Ok(OwnerShape::Whole)
        }
    }
}

fn nested_owner_shape(left: OwnerShape, right: OwnerShape) -> OwnerShape {
    if left == OwnerShape::Absent && right == OwnerShape::Absent {
        OwnerShape::Absent
    } else {
        OwnerShape::Nested
    }
}

struct CodeBudget {
    remaining: u32,
}

impl CodeBudget {
    fn new(limit: u32) -> Self {
        Self { remaining: limit }
    }

    fn charge(&mut self) -> bool {
        let Some(remaining) = self.remaining.checked_sub(1) else {
            return false;
        };
        self.remaining = remaining;
        true
    }
}

fn strictly_sorted(values: &[u16]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IntroductionAlias {
    pub constructor: ConstructorPortId,
    pub introduction: GlobalId,
}

impl CanonicalEncode for IntroductionAlias {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.constructor.encode_canonical(encoder);
        self.introduction.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClosedFormerFrame {
    pub schema_version: u16,
    pub code_id: FormerCodeId,
    pub owner: GlobalId,
    pub introductions: Vec<IntroductionAlias>,
    pub source_declarations: Vec<GlobalId>,
    pub principal_sources: Vec<PublicSourceId>,
    pub birth_support: Vec<GscOriginEventId>,
}

impl CanonicalEncode for ClosedFormerFrame {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.code_id.encode_canonical(encoder);
        self.owner.encode_canonical(encoder);
        encoder.sequence(&self.introductions);
        encoder.sequence(&self.source_declarations);
        encoder.sequence(&self.principal_sources);
        encoder.sequence(&self.birth_support);
    }
}

/// Structural evidence for a code/declaration shape only.
///
/// This capability proves that the owner and introductions have the types
/// reconstructed from the verified code. It does **not** prove that the group
/// is a complete registered history export, or that `principal_sources` and
/// `birth_support` are authentic declaration origins. Law-facing callers must
/// consume it only through a history-bound authoritative wrapper.
#[derive(Clone, Debug)]
pub struct VerifiedClosedFormerFrame {
    frame: ClosedFormerFrame,
    id: ClosedFormerFrameId,
    code: VerifiedClosedInductiveCode,
}

impl VerifiedClosedFormerFrame {
    pub fn frame(&self) -> &ClosedFormerFrame {
        &self.frame
    }

    pub fn id(&self) -> &ClosedFormerFrameId {
        &self.id
    }

    pub fn code(&self) -> &VerifiedClosedInductiveCode {
        &self.code
    }

    pub fn owner(&self) -> &GlobalId {
        &self.frame.owner
    }

    pub fn introductions(&self) -> &[IntroductionAlias] {
        &self.frame.introductions
    }

    pub fn introduction(&self, constructor: &ConstructorPortId) -> Option<&GlobalId> {
        self.frame
            .introductions
            .iter()
            .find(|alias| &alias.constructor == constructor)
            .map(|alias| &alias.introduction)
    }

    pub fn principal_sources(&self) -> &[PublicSourceId] {
        &self.frame.principal_sources
    }

    pub fn birth_support(&self) -> &[GscOriginEventId] {
        &self.frame.birth_support
    }
}

/// Checks only the structural kernel-facing portion of a former frame.
///
/// The provenance fields remain canonical caller-supplied metadata here. A
/// verified history/export-index layer must bind them to declaration origins
/// and group completeness before this frame has Law authority.
pub fn verify_closed_former_frame(
    manifest: &VerifiedGscSemanticManifest,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    code: &VerifiedClosedInductiveCode,
    frame: &ClosedFormerFrame,
) -> GscOutcome<VerifiedClosedFormerFrame> {
    if code.manifest_digest() != manifest.digest()
        || frame.schema_version != CLOSED_FORMER_FRAME_SCHEMA_VERSION
        || &frame.code_id != code.id()
        || frame.introductions.len() != code.constructor_ids().len()
        || frame.principal_sources.len() != frame.source_declarations.len()
        || !strictly_sorted_ids(&frame.birth_support)
        || !all_unique(&frame.principal_sources)
    {
        return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
    }

    for (alias, constructor) in frame.introductions.iter().zip(code.constructor_ids()) {
        if &alias.constructor != constructor {
            return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
        }
    }
    let expected_sources = std::iter::once(frame.owner.clone())
        .chain(
            frame
                .introductions
                .iter()
                .map(|alias| alias.introduction.clone()),
        )
        .collect::<Vec<_>>();
    if frame.source_declarations != expected_sources {
        return GscOutcome::Unknown(GscUnknownReason::MalformedFrame);
    }

    if !code.code().parameters.0.is_empty() || !code.code().indices.0.is_empty() {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
    }

    let owner_formation = OpenJudgment::TypeFormation {
        context: DependentContext::default(),
        term: Term::Global {
            id: frame.owner.clone(),
        },
    };
    let mut judgments = vec![owner_formation];
    for (alias, constructor) in frame.introductions.iter().zip(&code.code().constructors) {
        let mut result = Term::Global {
            id: frame.owner.clone(),
        };
        for argument in constructor.arguments.0.iter().rev() {
            let Some(parameter) = code_term_to_kernel(argument, &frame.owner) else {
                return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
            };
            result = Term::Pi {
                parameter: Box::new(parameter),
                body: Box::new(result),
            };
        }
        judgments.push(OpenJudgment::HasType {
            context: DependentContext::default(),
            term: Term::Global {
                id: alias.introduction.clone(),
            },
            ty: result,
        });
    }
    let references = judgments.iter().collect::<Vec<_>>();
    if kernel
        .verify_open_judgments(signature, &references)
        .is_err()
    {
        return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
    }

    let id = ClosedFormerFrameId(Digest::of_canonical(
        "pen-demand/gsc-closed-former-frame-id/v1",
        frame,
    ));
    GscOutcome::Proven(VerifiedClosedFormerFrame {
        frame: frame.clone(),
        id,
        code: code.clone(),
    })
}

fn strictly_sorted_ids<T: Ord>(values: &[T]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn all_unique<T: Ord + Clone>(values: &[T]) -> bool {
    values.iter().cloned().collect::<BTreeSet<_>>().len() == values.len()
}

pub(crate) fn code_term_to_kernel(term: &CodeTerm, owner: &GlobalId) -> Option<Term> {
    match term {
        CodeTerm::Sort { level } => Some(Term::Sort { level: *level }),
        CodeTerm::Bound { index } => Some(Term::Var { index: *index }),
        CodeTerm::Public { id } => Some(Term::Global { id: id.clone() }),
        CodeTerm::DependentFunction { parameter, body } => Some(Term::Pi {
            parameter: Box::new(code_term_to_kernel(parameter, owner)?),
            body: Box::new(code_term_to_kernel(body, owner)?),
        }),
        CodeTerm::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(code_term_to_kernel(function, owner)?),
            argument: Box::new(code_term_to_kernel(argument, owner)?),
        }),
        CodeTerm::OwnerApp {
            parameters,
            indices,
        } => {
            let mut result = Term::Global { id: owner.clone() };
            for argument in parameters.iter().chain(indices) {
                result = Term::Apply {
                    function: Box::new(result),
                    argument: Box::new(code_term_to_kernel(argument, owner)?),
                };
            }
            Some(result)
        }
    }
}
