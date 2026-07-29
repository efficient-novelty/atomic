use pen_kernel::{CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Term};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

macro_rules! digest_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
        #[serde(transparent)]
        pub struct $name(pub Digest);

        impl CanonicalEncode for $name {
            fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
                self.0.encode_canonical(encoder);
            }
        }
    };
}

digest_id!(EventIdV1);
digest_id!(SeedIdV1);
digest_id!(RawFamilyIdV1);
digest_id!(FamilyClassIdV1);
digest_id!(ClauseIdV1);
digest_id!(EquationIdV1);
digest_id!(DemandOrbitIdV1);
digest_id!(DemandOutputIdV1);
digest_id!(ContextWitnessIdV1);

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalRoleV1 {
    KernelHead,
    AdjointMate,
    SupportAction,
    Coherence,
}

impl CanonicalEncode for LocalRoleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::KernelHead => 0,
            Self::AdjointMate => 1,
            Self::SupportAction => 2,
            Self::Coherence => 3,
        });
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum GenericJudgmentV1 {
    Term {
        context: DependentContext,
        term: Term,
        ty: Term,
    },
    Equation {
        context: DependentContext,
        left: Term,
        right: Term,
        ty: Term,
    },
}

impl GenericJudgmentV1 {
    pub fn context(&self) -> &DependentContext {
        match self {
            Self::Term { context, .. } | Self::Equation { context, .. } => context,
        }
    }
}

impl CanonicalEncode for GenericJudgmentV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Term { context, term, ty } => {
                encoder.tag(0);
                context.encode_canonical(encoder);
                term.encode_canonical(encoder);
                ty.encode_canonical(encoder);
            }
            Self::Equation {
                context,
                left,
                right,
                ty,
            } => {
                encoder.tag(1);
                context.encode_canonical(encoder);
                left.encode_canonical(encoder);
                right.encode_canonical(encoder);
                ty.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceNormalizedJudgmentV1 {
    pub source_identity: Digest,
    pub source: GenericJudgmentV1,
    pub claimed_normalized: GenericJudgmentV1,
}

impl CanonicalEncode for SourceNormalizedJudgmentV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source_identity.encode_canonical(encoder);
        self.source.encode_canonical(encoder);
        self.claimed_normalized.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicSupportV1 {
    pub events: BTreeSet<EventIdV1>,
    pub declarations: BTreeSet<GlobalId>,
    pub demand_outputs: BTreeSet<DemandOutputIdV1>,
}

impl PublicSupportV1 {
    pub fn union(&self, other: &Self) -> Self {
        Self {
            events: self.events.union(&other.events).cloned().collect(),
            declarations: self
                .declarations
                .union(&other.declarations)
                .cloned()
                .collect(),
            demand_outputs: self
                .demand_outputs
                .union(&other.demand_outputs)
                .cloned()
                .collect(),
        }
    }

    pub fn is_old_support(&self, old_events: &BTreeSet<EventIdV1>) -> bool {
        self.events.is_subset(old_events)
    }
}

impl CanonicalEncode for PublicSupportV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encode_set(encoder, &self.events);
        encode_set(encoder, &self.declarations);
        encode_set(encoder, &self.demand_outputs);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum PublicAvailabilityV1 {
    PredecessorPublicExport {
        target: GlobalId,
    },
    DerivedFromPublicInterface {
        derivation_digest: Digest,
        public_support: BTreeSet<GlobalId>,
    },
    DependencyPriorExport {
        target: GlobalId,
    },
    AmbientOnly,
    OutsideFragment,
    Unknown,
}

impl PublicAvailabilityV1 {
    pub fn is_publicly_supported(&self) -> bool {
        matches!(
            self,
            Self::PredecessorPublicExport { .. }
                | Self::DerivedFromPublicInterface { .. }
                | Self::DependencyPriorExport { .. }
        )
    }
}

impl CanonicalEncode for PublicAvailabilityV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::PredecessorPublicExport { target } => {
                encoder.tag(0);
                target.encode_canonical(encoder);
            }
            Self::DerivedFromPublicInterface {
                derivation_digest,
                public_support,
            } => {
                encoder.tag(1);
                derivation_digest.encode_canonical(encoder);
                encode_set(encoder, public_support);
            }
            Self::DependencyPriorExport { target } => {
                encoder.tag(2);
                target.encode_canonical(encoder);
            }
            Self::AmbientOnly => encoder.tag(3),
            Self::OutsideFragment => encoder.tag(4),
            Self::Unknown => encoder.tag(5),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum AmbientPrimitiveV1 {
    Sort { level: u16 },
    UnitType,
    Unit,
}

impl CanonicalEncode for AmbientPrimitiveV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Sort { level } => {
                encoder.tag(0);
                encoder.u16(*level);
            }
            Self::UnitType => encoder.tag(1),
            Self::Unit => encoder.tag(2),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum HeadPresentationV1 {
    Opaque,
    TransparentDefinition,
    TransparentAlias {
        target: GlobalId,
        availability: PublicAvailabilityV1,
    },
    AmbientPrimitiveFirstExport {
        primitive: AmbientPrimitiveV1,
    },
}

impl CanonicalEncode for HeadPresentationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Opaque => encoder.tag(0),
            Self::TransparentDefinition => encoder.tag(1),
            Self::TransparentAlias {
                target,
                availability,
            } => {
                encoder.tag(2);
                target.encode_canonical(encoder);
                availability.encode_canonical(encoder);
            }
            Self::AmbientPrimitiveFirstExport { primitive } => {
                encoder.tag(3);
                primitive.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicHeadSeedV1 {
    pub declaration: GlobalId,
    pub origin_event: EventIdV1,
    pub judgment: SourceNormalizedJudgmentV1,
    pub presentation: HeadPresentationV1,
    pub claimed_role: LocalRoleV1,
    pub public_support: PublicSupportV1,
    pub source_clause: Option<ClauseIdV1>,
}

impl CanonicalEncode for PublicHeadSeedV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.declaration.encode_canonical(encoder);
        self.origin_event.encode_canonical(encoder);
        self.judgment.encode_canonical(encoder);
        self.presentation.encode_canonical(encoder);
        self.claimed_role.encode_canonical(encoder);
        self.public_support.encode_canonical(encoder);
        encoder.option(&self.source_clause);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicEquationSeedV1 {
    pub equation: EquationIdV1,
    pub owner_head: GlobalId,
    pub origin_event: EventIdV1,
    pub judgment: SourceNormalizedJudgmentV1,
    pub claimed_role: LocalRoleV1,
    pub public_support: PublicSupportV1,
    pub source_clause: Option<ClauseIdV1>,
    pub demand_anchor: Option<(DemandOrbitIdV1, DemandOutputIdV1)>,
}

impl CanonicalEncode for PublicEquationSeedV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.equation.encode_canonical(encoder);
        self.owner_head.encode_canonical(encoder);
        self.origin_event.encode_canonical(encoder);
        self.judgment.encode_canonical(encoder);
        self.claimed_role.encode_canonical(encoder);
        self.public_support.encode_canonical(encoder);
        encoder.option(&self.source_clause);
        match &self.demand_anchor {
            Some((orbit, output)) => {
                encoder.tag(1);
                orbit.encode_canonical(encoder);
                output.encode_canonical(encoder);
            }
            None => encoder.tag(0),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SemanticSchemaSeedV1 {
    PublicHead(PublicHeadSeedV1),
    PublicEquation(PublicEquationSeedV1),
    PublicUniversalInterface {
        interface_digest: Digest,
        origin_event: EventIdV1,
        judgment: SourceNormalizedJudgmentV1,
    },
}

impl CanonicalEncode for SemanticSchemaSeedV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::PublicHead(seed) => {
                encoder.tag(0);
                seed.encode_canonical(encoder);
            }
            Self::PublicEquation(seed) => {
                encoder.tag(1);
                seed.encode_canonical(encoder);
            }
            Self::PublicUniversalInterface {
                interface_digest,
                origin_event,
                judgment,
            } => {
                encoder.tag(2);
                interface_digest.encode_canonical(encoder);
                origin_event.encode_canonical(encoder);
                judgment.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FamilyConstructorV1 {
    PublicHeadSeed {
        seed: SeedIdV1,
    },
    PublicEquationSeed {
        seed: SeedIdV1,
    },
    GenericPublicApplication {
        function: RawFamilyIdV1,
        argument: RawFamilyIdV1,
        context_witness: ContextWitnessIdV1,
    },
    GenericEquationAction {
        equation: RawFamilyIdV1,
        context: RawFamilyIdV1,
        hole_ordinal: u32,
        context_witness: ContextWitnessIdV1,
    },
}

impl CanonicalEncode for FamilyConstructorV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::PublicHeadSeed { seed } => {
                encoder.tag(0);
                seed.encode_canonical(encoder);
            }
            Self::PublicEquationSeed { seed } => {
                encoder.tag(1);
                seed.encode_canonical(encoder);
            }
            Self::GenericPublicApplication {
                function,
                argument,
                context_witness,
            } => {
                encoder.tag(2);
                function.encode_canonical(encoder);
                argument.encode_canonical(encoder);
                context_witness.encode_canonical(encoder);
            }
            Self::GenericEquationAction {
                equation,
                context,
                hole_ordinal,
                context_witness,
            } => {
                encoder.tag(3);
                equation.encode_canonical(encoder);
                context.encode_canonical(encoder);
                encoder.u32(*hole_ordinal);
                context_witness.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawFamilyV1 {
    pub id: RawFamilyIdV1,
    pub rank: u16,
    pub constructor: FamilyConstructorV1,
    pub generic_judgment: GenericJudgmentV1,
    pub role: LocalRoleV1,
    pub public_support: PublicSupportV1,
    pub source_clause: Option<ClauseIdV1>,
    pub demand_anchor: Option<(DemandOrbitIdV1, DemandOutputIdV1)>,
    pub substitution_action_digest: Digest,
}

impl CanonicalEncode for RawFamilyV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        encoder.u16(self.rank);
        self.constructor.encode_canonical(encoder);
        self.generic_judgment.encode_canonical(encoder);
        self.role.encode_canonical(encoder);
        self.public_support.encode_canonical(encoder);
        encoder.option(&self.source_clause);
        match &self.demand_anchor {
            Some((orbit, output)) => {
                encoder.tag(1);
                orbit.encode_canonical(encoder);
                output.encode_canonical(encoder);
            }
            None => encoder.tag(0),
        }
        self.substitution_action_digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyClassV1 {
    pub id: FamilyClassIdV1,
    pub representative: RawFamilyIdV1,
    pub members: Vec<RawFamilyIdV1>,
    pub generic_judgment: GenericJudgmentV1,
    pub role: LocalRoleV1,
    pub canonical_support: PublicSupportV1,
    pub source_clause: Option<ClauseIdV1>,
    pub demand_anchor: Option<(DemandOrbitIdV1, DemandOutputIdV1)>,
    pub substitution_action_digest: Digest,
}

impl CanonicalEncode for FamilyClassV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.representative.encode_canonical(encoder);
        encoder.sequence(&self.members);
        self.generic_judgment.encode_canonical(encoder);
        self.role.encode_canonical(encoder);
        self.canonical_support.encode_canonical(encoder);
        encoder.option(&self.source_clause);
        match &self.demand_anchor {
            Some((orbit, output)) => {
                encoder.tag(1);
                orbit.encode_canonical(encoder);
                output.encode_canonical(encoder);
            }
            None => encoder.tag(0),
        }
        self.substitution_action_digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyInstanceV1 {
    pub originating_class: FamilyClassIdV1,
    pub substitution: Vec<Term>,
    pub specialized_judgment: GenericJudgmentV1,
}

impl CanonicalEncode for FamilyInstanceV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.originating_class.encode_canonical(encoder);
        encoder.sequence(&self.substitution);
        self.specialized_judgment.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ConservativeExtensionV1 {
    pub predecessor_boundary_digest: Digest,
    pub successor_boundary_digest: Digest,
    pub new_event: EventIdV1,
    pub predecessor_events: BTreeSet<EventIdV1>,
    pub discharged_outputs: BTreeSet<DemandOutputIdV1>,
    pub exact_and_conservative: bool,
}

impl CanonicalEncode for ConservativeExtensionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.predecessor_boundary_digest.encode_canonical(encoder);
        self.successor_boundary_digest.encode_canonical(encoder);
        self.new_event.encode_canonical(encoder);
        encode_set(encoder, &self.predecessor_events);
        encode_set(encoder, &self.discharged_outputs);
        encoder.tag(u8::from(self.exact_and_conservative));
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ProvenanceTagV1 {
    ClauseRole {
        clause: ClauseIdV1,
        role: LocalRoleV1,
    },
    DemandOutput {
        orbit: DemandOrbitIdV1,
        output: DemandOutputIdV1,
    },
}

impl CanonicalEncode for ProvenanceTagV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::ClauseRole { clause, role } => {
                encoder.tag(0);
                clause.encode_canonical(encoder);
                role.encode_canonical(encoder);
            }
            Self::DemandOutput { orbit, output } => {
                encoder.tag(1);
                orbit.encode_canonical(encoder);
                output.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ClauseCostDispositionV1 {
    FirstIrreducible,
    TransparentAlias { target: GlobalId },
    ForcedDefinitionalCompletion,
    ForcedProjection,
    DuplicatePresentation { original: ClauseIdV1 },
}

impl CanonicalEncode for ClauseCostDispositionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::FirstIrreducible => encoder.tag(0),
            Self::TransparentAlias { target } => {
                encoder.tag(1);
                target.encode_canonical(encoder);
            }
            Self::ForcedDefinitionalCompletion => encoder.tag(2),
            Self::ForcedProjection => encoder.tag(3),
            Self::DuplicatePresentation { original } => {
                encoder.tag(4);
                original.encode_canonical(encoder);
            }
        }
    }
}

fn encode_set<T: CanonicalEncode>(encoder: &mut CanonicalEncoder, values: &BTreeSet<T>) {
    encoder.u64(values.len() as u64);
    for value in values {
        value.encode_canonical(encoder);
    }
}

pub(crate) fn semantic_seed_id(seed: &SemanticSchemaSeedV1) -> SeedIdV1 {
    SeedIdV1(Digest::of_canonical(
        "pen-semantic-audit/semantic-seed/v1",
        seed,
    ))
}

pub(crate) fn raw_family_id(
    manifest_digest: &Digest,
    rank: u16,
    constructor: &FamilyConstructorV1,
    judgment: &GenericJudgmentV1,
    role: LocalRoleV1,
    support: &PublicSupportV1,
    substitution_action_digest: &Digest,
) -> RawFamilyIdV1 {
    struct Subject<'a> {
        manifest_digest: &'a Digest,
        rank: u16,
        constructor: &'a FamilyConstructorV1,
        judgment: &'a GenericJudgmentV1,
        role: LocalRoleV1,
        support: &'a PublicSupportV1,
        substitution_action_digest: &'a Digest,
    }
    impl CanonicalEncode for Subject<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.manifest_digest.encode_canonical(encoder);
            encoder.u16(self.rank);
            self.constructor.encode_canonical(encoder);
            self.judgment.encode_canonical(encoder);
            self.role.encode_canonical(encoder);
            self.support.encode_canonical(encoder);
            self.substitution_action_digest.encode_canonical(encoder);
        }
    }
    RawFamilyIdV1(Digest::of_canonical(
        "pen-semantic-audit/raw-family/v1",
        &Subject {
            manifest_digest,
            rank,
            constructor,
            judgment,
            role,
            support,
            substitution_action_digest,
        },
    ))
}
