use super::inductive::{ClosedFormerFrameId, ComputationMode, ConstructorPortId};
use super::manifest::{GscOutcome, GscUnknownReason, VerifiedGscSemanticManifest};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Kernel, OpenJudgment,
    Term, VerifiedSignature,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const CANONICAL_DEMAND_FAMILY_SCHEMA_VERSION: u16 = 2;

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct GscOriginEventId(pub Digest);

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct PublicSourceId(pub Digest);

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct PublicPortId(pub Digest);

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct GscFamilyId(pub Digest);

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct OutputPortId(pub Digest);

macro_rules! encode_digest_id {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl CanonicalEncode for $ty {
                fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
                    self.0.encode_canonical(encoder);
                }
            }
        )+
    };
}

encode_digest_id!(
    GscOriginEventId,
    PublicSourceId,
    PublicPortId,
    GscFamilyId,
    OutputPortId,
);

impl PublicSourceId {
    pub fn for_declaration(origin: &GscOriginEventId, declaration: &GlobalId) -> Self {
        let mut encoder = CanonicalEncoder::new();
        origin.encode_canonical(&mut encoder);
        declaration.encode_canonical(&mut encoder);
        Self(Digest::of_domain_bytes(
            "pen-demand/gsc-public-source-id/v1",
            encoder.as_bytes(),
        ))
    }
}

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum GscRule {
    Use,
    Compute,
}

impl CanonicalEncode for GscRule {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Use => 0,
            Self::Compute => 1,
        });
    }
}

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(deny_unknown_fields)]
pub struct PortKey {
    pub family_id: GscFamilyId,
    pub output_port_id: OutputPortId,
}

impl PortKey {
    pub fn new(family_id: GscFamilyId, output_port_id: OutputPortId) -> Self {
        Self {
            family_id,
            output_port_id,
        }
    }

    pub fn family_id(&self) -> &GscFamilyId {
        &self.family_id
    }

    pub fn output_port_id(&self) -> &OutputPortId {
        &self.output_port_id
    }
}

impl CanonicalEncode for PortKey {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.family_id.encode_canonical(encoder);
        self.output_port_id.encode_canonical(encoder);
    }
}

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(tag = "source", rename_all = "snake_case", deny_unknown_fields)]
pub enum PortRef {
    Public { port: PublicPortId },
    Generated { port: PortKey },
}

impl CanonicalEncode for PortRef {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Public { port } => {
                encoder.tag(0);
                port.encode_canonical(encoder);
            }
            Self::Generated { port } => {
                encoder.tag(1);
                port.encode_canonical(encoder);
            }
        }
    }
}

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum ContextMapCode {
    IdentityV1,
}

impl CanonicalEncode for ContextMapCode {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(0);
    }
}

#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(tag = "role", rename_all = "snake_case", deny_unknown_fields)]
pub enum OutputRole {
    UsePort {
        former: ClosedFormerFrameId,
    },
    ComputationPort {
        former: ClosedFormerFrameId,
        constructor: ConstructorPortId,
        computation_mode: ComputationMode,
    },
}

impl CanonicalEncode for OutputRole {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::UsePort { former } => {
                encoder.tag(0);
                former.encode_canonical(encoder);
            }
            Self::ComputationPort {
                former,
                constructor,
                computation_mode,
            } => {
                encoder.tag(1);
                former.encode_canonical(encoder);
                constructor.encode_canonical(encoder);
                computation_mode.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PremiseRef {
    pub source: PortRef,
    pub expected_role: OutputRole,
    pub context_map: ContextMapCode,
}

impl CanonicalEncode for PremiseRef {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source.encode_canonical(encoder);
        self.expected_role.encode_canonical(encoder);
        self.context_map.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "equation", rename_all = "snake_case", deny_unknown_fields)]
pub enum EquationClauseCode {
    GeneratedUseBeta {
        former: ClosedFormerFrameId,
        constructor: ConstructorPortId,
        use_port: PortKey,
    },
}

impl EquationClauseCode {
    pub fn former(&self) -> &ClosedFormerFrameId {
        match self {
            Self::GeneratedUseBeta { former, .. } => former,
        }
    }

    pub fn constructor(&self) -> &ConstructorPortId {
        match self {
            Self::GeneratedUseBeta { constructor, .. } => constructor,
        }
    }

    pub fn use_port(&self) -> &PortKey {
        match self {
            Self::GeneratedUseBeta { use_port, .. } => use_port,
        }
    }
}

impl CanonicalEncode for EquationClauseCode {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::GeneratedUseBeta {
                former,
                constructor,
                use_port,
            } => {
                encoder.tag(0);
                former.encode_canonical(encoder);
                constructor.encode_canonical(encoder);
                use_port.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "clause", rename_all = "snake_case", deny_unknown_fields)]
pub enum OutputClause {
    TermPort {
        context: DependentContext,
        motive: Term,
        role: OutputRole,
    },
    EquationPort {
        context: DependentContext,
        equation_code: EquationClauseCode,
        role: OutputRole,
    },
}

impl OutputClause {
    pub fn context(&self) -> &DependentContext {
        match self {
            Self::TermPort { context, .. } | Self::EquationPort { context, .. } => context,
        }
    }

    pub fn role(&self) -> &OutputRole {
        match self {
            Self::TermPort { role, .. } | Self::EquationPort { role, .. } => role,
        }
    }

    pub fn motive(&self) -> Option<&Term> {
        match self {
            Self::TermPort { motive, .. } => Some(motive),
            Self::EquationPort { .. } => None,
        }
    }

    pub fn equation_code(&self) -> Option<&EquationClauseCode> {
        match self {
            Self::TermPort { .. } => None,
            Self::EquationPort { equation_code, .. } => Some(equation_code),
        }
    }

    fn kind_tag(&self) -> u8 {
        match self {
            Self::TermPort { .. } => 0,
            Self::EquationPort { .. } => 1,
        }
    }
}

impl CanonicalEncode for OutputClause {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::TermPort {
                context,
                motive,
                role,
            } => {
                encoder.tag(0);
                context.encode_canonical(encoder);
                motive.encode_canonical(encoder);
                role.encode_canonical(encoder);
            }
            Self::EquationPort {
                context,
                equation_code,
                role,
            } => {
                encoder.tag(1);
                context.encode_canonical(encoder);
                equation_code.encode_canonical(encoder);
                role.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalDemandFamilyV2 {
    pub schema_version: u16,
    pub semantic_manifest_digest: Digest,
    pub rule: GscRule,
    pub rank: u8,
    pub parameter_context: DependentContext,
    pub premise_refs: Vec<PremiseRef>,
    pub output_clauses: Vec<OutputClause>,
    pub verification_judgments: Vec<OpenJudgment>,
    pub principal_sources: Vec<PublicSourceId>,
    pub birth_support: Vec<GscOriginEventId>,
    pub fixed_type_support: Vec<GlobalId>,
    pub parameter_support_projections: Vec<u32>,
}

impl CanonicalEncode for CanonicalDemandFamilyV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.rule.encode_canonical(encoder);
        encoder.tag(self.rank);
        self.parameter_context.encode_canonical(encoder);
        encoder.sequence(&self.premise_refs);
        encoder.sequence(&self.output_clauses);
        encoder.sequence(&self.verification_judgments);
        encoder.sequence(&self.principal_sources);
        encoder.sequence(&self.birth_support);
        encoder.sequence(&self.fixed_type_support);
        encoder.u64(self.parameter_support_projections.len() as u64);
        for projection in &self.parameter_support_projections {
            encoder.u32(*projection);
        }
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedOutputPort {
    key: PortKey,
    ordinal: u32,
    clause: OutputClause,
}

impl VerifiedOutputPort {
    pub fn key(&self) -> &PortKey {
        &self.key
    }

    pub fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub fn clause(&self) -> &OutputClause {
        &self.clause
    }

    pub fn role(&self) -> &OutputRole {
        self.clause.role()
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedCanonicalDemandFamilyV2 {
    id: GscFamilyId,
    family: CanonicalDemandFamilyV2,
    ports: Vec<VerifiedOutputPort>,
}

impl VerifiedCanonicalDemandFamilyV2 {
    pub fn id(&self) -> &GscFamilyId {
        &self.id
    }

    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.family.semantic_manifest_digest
    }

    pub fn rule(&self) -> GscRule {
        self.family.rule
    }

    pub fn rank(&self) -> u8 {
        self.family.rank
    }

    pub fn parameter_context(&self) -> &DependentContext {
        &self.family.parameter_context
    }

    pub fn premise_refs(&self) -> &[PremiseRef] {
        &self.family.premise_refs
    }

    pub fn output_clauses(&self) -> &[OutputClause] {
        &self.family.output_clauses
    }

    pub fn verification_judgments(&self) -> &[OpenJudgment] {
        &self.family.verification_judgments
    }

    pub fn principal_sources(&self) -> &[PublicSourceId] {
        &self.family.principal_sources
    }

    pub fn birth_support(&self) -> &[GscOriginEventId] {
        &self.family.birth_support
    }

    pub fn fixed_type_support(&self) -> &[GlobalId] {
        &self.family.fixed_type_support
    }

    pub fn parameter_support_projections(&self) -> &[u32] {
        &self.family.parameter_support_projections
    }

    pub fn ports(&self) -> &[VerifiedOutputPort] {
        &self.ports
    }

    pub fn port(&self, key: &PortKey) -> Option<&VerifiedOutputPort> {
        if &key.family_id != self.id() {
            return None;
        }
        self.ports.iter().find(|port| port.key() == key)
    }

    pub fn canonical_wire(&self) -> &CanonicalDemandFamilyV2 {
        &self.family
    }
}

pub(crate) fn verify_compiled_family(
    manifest: &VerifiedGscSemanticManifest,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    mut family: CanonicalDemandFamilyV2,
) -> GscOutcome<VerifiedCanonicalDemandFamilyV2> {
    let limits = manifest.limits();
    if family.schema_version != CANONICAL_DEMAND_FAMILY_SCHEMA_VERSION
        || &family.semantic_manifest_digest != manifest.digest()
        || family.rank == 0
        || family.rank > 2
        || family.premise_refs.len() > usize::from(limits.max_family_premises)
        || family.output_clauses.is_empty()
        || family.output_clauses.len() > usize::from(limits.max_family_outputs)
        || family.verification_judgments.len() > usize::from(limits.max_verification_judgments)
        || !all_unique(&family.principal_sources)
        || !strictly_sorted(&family.birth_support)
        || !strictly_sorted(&family.fixed_type_support)
        || !strictly_sorted(&family.parameter_support_projections)
        || family
            .parameter_support_projections
            .iter()
            .any(|index| (*index as usize) >= family.parameter_context.0.len())
    {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }

    family.parameter_context = match kernel.verify_context(signature, &family.parameter_context) {
        Ok(context) => context.normalized_wire(),
        Err(_) => return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify),
    };
    for clause in &mut family.output_clauses {
        match clause {
            OutputClause::TermPort {
                context, motive, ..
            } => {
                let judgment = OpenJudgment::TypeFormation {
                    context: context.clone(),
                    term: motive.clone(),
                };
                let normalized = match kernel.verify_open_judgment(signature, &judgment) {
                    Ok(OpenJudgment::TypeFormation { context, term }) => (context, term),
                    _ => {
                        return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
                    }
                };
                *context = normalized.0;
                *motive = normalized.1;
            }
            OutputClause::EquationPort { context, .. } => {
                *context = match kernel.verify_context(signature, context) {
                    Ok(context) => context.normalized_wire(),
                    Err(_) => {
                        return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
                    }
                };
            }
        }
    }
    let references = family.verification_judgments.iter().collect::<Vec<_>>();
    family.verification_judgments = match kernel.verify_open_judgments(signature, &references) {
        Ok(judgments) => judgments,
        Err(_) => return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify),
    };
    if family.output_clauses.iter().any(|clause| {
        clause.context().0.len() < family.parameter_context.0.len()
            || clause.context().0[..family.parameter_context.0.len()] != family.parameter_context.0
    }) {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }

    let id = GscFamilyId(Digest::of_canonical("pen-demand/gsc-family-id/v2", &family));
    let ports = family
        .output_clauses
        .iter()
        .enumerate()
        .map(|(ordinal, clause)| {
            let mut encoder = CanonicalEncoder::new();
            manifest.digest().encode_canonical(&mut encoder);
            id.encode_canonical(&mut encoder);
            encoder.u32(ordinal as u32);
            encoder.tag(clause.kind_tag());
            clause.role().encode_canonical(&mut encoder);
            let output_port_id = OutputPortId(Digest::of_domain_bytes(
                "pen-demand/gsc-output-port-id/v2",
                encoder.as_bytes(),
            ));
            VerifiedOutputPort {
                key: PortKey::new(id.clone(), output_port_id),
                ordinal: ordinal as u32,
                clause: clause.clone(),
            }
        })
        .collect();

    GscOutcome::Proven(VerifiedCanonicalDemandFamilyV2 { id, family, ports })
}

fn strictly_sorted<T: Ord>(values: &[T]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn all_unique<T: Ord + Clone>(values: &[T]) -> bool {
    values.iter().cloned().collect::<BTreeSet<_>>().len() == values.len()
}
