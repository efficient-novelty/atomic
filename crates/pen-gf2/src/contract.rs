use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Digest, KernelLimits, OpenJudgment, UncheckedSignature,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const FINITE_FRAGMENT_SCHEMA_VERSION: u16 = 1;
pub const CANONICAL_ORDER_VERSION: u16 = 1;
pub const LAW_REQUEST_SCHEMA_VERSION: u16 = 1;
pub const DECISION_CLAIM_SCHEMA_VERSION: u16 = 1;

pub(crate) const MAX_OPERATIONS: u32 = 1_000_000;
pub(crate) const MAX_NORMALIZATION_FUEL: u32 = 1_000_000;
pub(crate) const MAX_SIGNATURE_DECLARATIONS: u32 = 65_536;
pub(crate) const MAX_CONTEXT_ENTRIES: u32 = 65_536;
pub(crate) const MAX_INVENTORY_ENTRIES: u32 = 262_144;
pub(crate) const MAX_RULE_ARITY: u16 = 1_024;
pub(crate) const MAX_UNIVERSE_LEVEL: u16 = 1_024;
pub(crate) const MAX_SUM_ARITY: u16 = 1_024;
pub(crate) const MAX_CUBE_DIMENSION: u8 = 16;
pub(crate) const MAX_EQUIVALENCE_CHECKS: u32 = 1_000_000;
pub(crate) const REQUIRED_DEMAND_SCHEME_DEPTH: u8 = 2;

/// The canonical ordering rule is part of the fragment identity.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalOrder {
    FeatureThenDigestV1,
}

impl CanonicalEncode for CanonicalOrder {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::FeatureThenDigestV1 => encoder.tag(0),
        }
    }
}

/// Exact feature inventory required by the repaired finite-fragment contract.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum FiniteFeature {
    DependentContextsAndTelescopes,
    FiniteUniverseLevels,
    DependentProducts,
    DependentSums,
    FiniteSumsWithComputation,
    FinitePathOperations,
    FiniteCubicalOperations,
    OpaqueConstants,
    CandidateEquationSets,
    DepthTwoDemandSchemes,
    EquivalenceWitnessGrammar,
}

impl FiniteFeature {
    pub const ALL: [Self; 11] = [
        Self::DependentContextsAndTelescopes,
        Self::FiniteUniverseLevels,
        Self::DependentProducts,
        Self::DependentSums,
        Self::FiniteSumsWithComputation,
        Self::FinitePathOperations,
        Self::FiniteCubicalOperations,
        Self::OpaqueConstants,
        Self::CandidateEquationSets,
        Self::DepthTwoDemandSchemes,
        Self::EquivalenceWitnessGrammar,
    ];
}

impl CanonicalEncode for FiniteFeature {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::DependentContextsAndTelescopes => 0,
            Self::FiniteUniverseLevels => 1,
            Self::DependentProducts => 2,
            Self::DependentSums => 3,
            Self::FiniteSumsWithComputation => 4,
            Self::FinitePathOperations => 5,
            Self::FiniteCubicalOperations => 6,
            Self::OpaqueConstants => 7,
            Self::CandidateEquationSets => 8,
            Self::DepthTwoDemandSchemes => 9,
            Self::EquivalenceWitnessGrammar => 10,
        });
    }
}

/// Canonically ordered identifiers for one finite syntax family.
///
/// The identifiers name carrier entries or rules. They identify material but
/// do not prove that the material is sound or complete.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FiniteFeatureDeclaration {
    pub feature: FiniteFeature,
    pub entries: Vec<Digest>,
}

impl CanonicalEncode for FiniteFeatureDeclaration {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.feature.encode_canonical(encoder);
        encoder.sequence(&self.entries);
    }
}

/// Explicit semantic and verifier bounds for one finite fragment.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct FiniteFragmentLimits {
    pub max_operations: u32,
    pub max_term_depth: u16,
    pub normalization_fuel: u32,
    pub max_signature_declarations: u32,
    pub max_context_entries: u32,
    pub max_inventory_entries: u32,
    pub max_rule_arity: u16,
    pub max_universe_level: u16,
    pub max_sum_arity: u16,
    pub max_cube_dimension: u8,
    pub max_equivalence_checks: u32,
    pub demand_scheme_depth: u8,
}

impl Default for FiniteFragmentLimits {
    fn default() -> Self {
        let kernel = KernelLimits::default();
        Self {
            max_operations: kernel.max_operations,
            max_term_depth: kernel.max_depth,
            normalization_fuel: kernel.normalization_fuel,
            max_signature_declarations: 4_096,
            max_context_entries: 4_096,
            max_inventory_entries: 65_536,
            max_rule_arity: 256,
            max_universe_level: 256,
            max_sum_arity: 256,
            max_cube_dimension: 8,
            max_equivalence_checks: 100_000,
            demand_scheme_depth: REQUIRED_DEMAND_SCHEME_DEPTH,
        }
    }
}

impl FiniteFragmentLimits {
    pub fn kernel_limits(self) -> KernelLimits {
        KernelLimits {
            max_operations: self.max_operations,
            max_depth: self.max_term_depth,
            normalization_fuel: self.normalization_fuel,
        }
    }

    pub(crate) fn invalid_field(self) -> Option<&'static str> {
        if self.max_operations == 0 || self.max_operations > MAX_OPERATIONS {
            return Some("max_operations");
        }
        if self.max_term_depth == 0 || self.max_term_depth > pen_kernel::MAX_SAFE_RECURSION_DEPTH {
            return Some("max_term_depth");
        }
        if self.normalization_fuel == 0 || self.normalization_fuel > MAX_NORMALIZATION_FUEL {
            return Some("normalization_fuel");
        }
        if self.max_signature_declarations == 0
            || self.max_signature_declarations > MAX_SIGNATURE_DECLARATIONS
        {
            return Some("max_signature_declarations");
        }
        if self.max_context_entries == 0 || self.max_context_entries > MAX_CONTEXT_ENTRIES {
            return Some("max_context_entries");
        }
        if self.max_inventory_entries == 0 || self.max_inventory_entries > MAX_INVENTORY_ENTRIES {
            return Some("max_inventory_entries");
        }
        if self.max_rule_arity == 0 || self.max_rule_arity > MAX_RULE_ARITY {
            return Some("max_rule_arity");
        }
        if self.max_universe_level > MAX_UNIVERSE_LEVEL {
            return Some("max_universe_level");
        }
        if self.max_sum_arity < 2 || self.max_sum_arity > MAX_SUM_ARITY {
            return Some("max_sum_arity");
        }
        if self.max_cube_dimension == 0 || self.max_cube_dimension > MAX_CUBE_DIMENSION {
            return Some("max_cube_dimension");
        }
        if self.max_equivalence_checks == 0 || self.max_equivalence_checks > MAX_EQUIVALENCE_CHECKS
        {
            return Some("max_equivalence_checks");
        }
        if self.demand_scheme_depth != REQUIRED_DEMAND_SCHEME_DEPTH {
            return Some("demand_scheme_depth");
        }
        None
    }
}

impl CanonicalEncode for FiniteFragmentLimits {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u32(self.max_operations);
        encoder.u16(self.max_term_depth);
        encoder.u32(self.normalization_fuel);
        encoder.u32(self.max_signature_declarations);
        encoder.u32(self.max_context_entries);
        encoder.u32(self.max_inventory_entries);
        encoder.u16(self.max_rule_arity);
        encoder.u16(self.max_universe_level);
        encoder.u16(self.max_sum_arity);
        encoder.tag(self.max_cube_dimension);
        encoder.u32(self.max_equivalence_checks);
        encoder.tag(self.demand_scheme_depth);
    }
}

/// Deserializable fragment input. It must be structurally validated before use.
///
/// Structural validation establishes versions, bounds, canonical ordering,
/// inventory identity, and native-backend binding. It does not establish a
/// closure theorem, computation rule, or equivalence theorem for the listed
/// entries; those require separate replayable evidence.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedFiniteFragmentManifest {
    pub schema_version: u16,
    pub canonical_order_version: u16,
    pub canonical_order: CanonicalOrder,
    pub limits: FiniteFragmentLimits,
    pub kernel_protocol_digest: Digest,
    pub normalizer_protocol_digest: Digest,
    pub syntax: Vec<FiniteFeatureDeclaration>,
}

impl UncheckedFiniteFragmentManifest {
    /// Compute an identity only. Validation is required before the digest may
    /// be bound into a verified handle.
    pub fn canonical_digest(&self) -> Digest {
        Digest::of_canonical("pen-gf2/finite-fragment-manifest/v1", self)
    }
}

impl CanonicalEncode for UncheckedFiniteFragmentManifest {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        encoder.u16(self.canonical_order_version);
        self.canonical_order.encode_canonical(encoder);
        self.limits.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        encoder.sequence(&self.syntax);
    }
}

/// A request received from a wire or artifact boundary.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UncheckedLawRequest {
    pub schema_version: u16,
    pub subject: UncheckedLawSubject,
}

/// Native kernel judgments and opaque subjects from other fragment features.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum UncheckedLawSubject {
    KernelJudgment {
        signature: UncheckedSignature,
        judgment: OpenJudgment,
    },
    FragmentArtifact {
        feature: FiniteFeature,
        subject_digest: Digest,
    },
}

/// Resource categories that are stable at the finite-fragment boundary.
#[derive(
    Clone, Copy, Debug, Deserialize, Eq, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum FiniteResource {
    Operations,
    TermDepth,
    NormalizationFuel,
}

/// Deserializable decision testimony. This type is intentionally unchecked.
///
/// A digest identifies a claimed subject or evidence object; it does not turn
/// the claim into a proof. Verified handles are produced only by `LawKernel`.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(tag = "decision", rename_all = "snake_case", deny_unknown_fields)]
pub enum UncheckedDecisionClaim {
    Proven {
        schema_version: u16,
        fragment_digest: Digest,
        subject_digest: Digest,
        evidence_digest: Digest,
    },
    Refuted {
        schema_version: u16,
        fragment_digest: Digest,
        subject_digest: Digest,
        evidence_digest: Digest,
    },
    OutsideFragment {
        schema_version: u16,
        fragment_digest: Digest,
        subject_digest: Digest,
        feature: Option<FiniteFeature>,
    },
    ResourceExhausted {
        schema_version: u16,
        fragment_digest: Digest,
        subject_digest: Digest,
        resource: FiniteResource,
    },
}
