use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, GlobalId, OpenJudgment};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Canonical identifier for one registered schema family.
#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct FamilyId(pub Digest);

impl CanonicalEncode for FamilyId {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// Canonical identifier for one concrete family instance.
#[derive(
    Clone, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct InstanceId(pub Digest);

impl CanonicalEncode for InstanceId {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// Exactly two opaque public-boundary anchors. Their array representation is
/// the width invariant; there is no runtime width setting.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OpaqueWindow {
    pub entries: [GlobalId; 2],
}

impl CanonicalEncode for OpaqueWindow {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.entries);
    }
}

/// An abstract family and its kernel-checkable type-formation motive.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisteredFamily {
    pub id: FamilyId,
    pub motive: OpenJudgment,
}

impl RegisteredFamily {
    pub fn canonical_id(motive: &OpenJudgment) -> FamilyId {
        FamilyId(Digest::of_canonical("pen-demand/family-id/v1", motive))
    }

    pub fn canonical(motive: OpenJudgment) -> Self {
        Self {
            id: Self::canonical_id(&motive),
            motive,
        }
    }
}

impl CanonicalEncode for RegisteredFamily {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.motive.encode_canonical(encoder);
    }
}

/// A concrete pre-registered instance remains distinct from the family that
/// classifies it. `family_id` is a relative registry assumption checked only
/// for an exact reference; it is not a substitution or naturality theorem.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RegisteredInstance {
    pub id: InstanceId,
    pub family_id: FamilyId,
    pub motive: OpenJudgment,
}

impl RegisteredInstance {
    pub fn canonical_id(family_id: &FamilyId, motive: &OpenJudgment) -> InstanceId {
        let mut encoder = CanonicalEncoder::new();
        family_id.encode_canonical(&mut encoder);
        motive.encode_canonical(&mut encoder);
        InstanceId(Digest::of_domain_bytes(
            "pen-demand/instance-id/v1",
            encoder.as_bytes(),
        ))
    }

    pub fn canonical(family_id: FamilyId, motive: OpenJudgment) -> Self {
        Self {
            id: Self::canonical_id(&family_id, &motive),
            family_id,
            motive,
        }
    }
}

impl CanonicalEncode for RegisteredInstance {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.family_id.encode_canonical(encoder);
        self.motive.encode_canonical(encoder);
    }
}

/// A finite monotone rule over registered instance identifiers.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FiniteRule {
    pub premises: Vec<InstanceId>,
    pub conclusion: InstanceId,
}

impl CanonicalEncode for FiniteRule {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.premises);
        self.conclusion.encode_canonical(encoder);
    }
}

/// The finite domain whose relative closure is computed.
#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FiniteDemandDomain {
    pub families: Vec<RegisteredFamily>,
    pub instances: Vec<RegisteredInstance>,
    pub rules: Vec<FiniteRule>,
}

impl CanonicalEncode for FiniteDemandDomain {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.families);
        encoder.sequence(&self.instances);
        encoder.sequence(&self.rules);
    }
}

/// Explicit initial instance identifiers supplied as relative trusted input
/// by the library. Reference validation does not derive or prove these seeds.
#[derive(Clone, Debug, Default, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LibrarySeeds {
    pub instance_ids: Vec<InstanceId>,
}

impl CanonicalEncode for LibrarySeeds {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.instance_ids);
    }
}

/// Support is computed from global references in the family and instance
/// motives, closed transitively through verified declaration types and bodies.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct StructuralSupport {
    pub instance_id: InstanceId,
    pub globals: Vec<GlobalId>,
    pub window_hits: [bool; 2],
}

impl CanonicalEncode for StructuralSupport {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.instance_id.encode_canonical(encoder);
        encoder.sequence(&self.globals);
        for hit in self.window_hits {
            encoder.tag(u8::from(hit));
        }
    }
}

/// A completed fixed-point calculation relative to the explicit finite input.
///
/// `active` is the structurally extracted subset of the pre-registered
/// carrier. `reached` is the separate library closure inside that domain.
/// `unreached` is `active - reached`, not a claim about demands outside the
/// supplied domain. Layer zero is exactly the explicit seed set.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RelativeCensus {
    pub support: Vec<StructuralSupport>,
    pub active: Vec<InstanceId>,
    pub reached: Vec<InstanceId>,
    pub unreached: Vec<InstanceId>,
    pub layers: Vec<Vec<InstanceId>>,
}

impl CanonicalEncode for RelativeCensus {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.sequence(&self.support);
        encoder.sequence(&self.active);
        encoder.sequence(&self.reached);
        encoder.sequence(&self.unreached);
        encoder.u64(self.layers.len() as u64);
        for layer in &self.layers {
            encoder.sequence(layer);
        }
    }
}

/// The only two dispositions exposed by the relative census boundary.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UnknownReason {
    ResourceExhausted,
    Unsupported,
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "status", content = "result", rename_all = "snake_case")]
pub enum RelativeCensusOutcome<T> {
    CompleteRelative(T),
    Unknown(UnknownReason),
}
