use super::reference::VerifiedGscReferenceAgreement;
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Digest, Kernel};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub const GSC_SEMANTIC_MANIFEST_SCHEMA_VERSION: u16 = 1;
pub const GSC_VERIFIER_MANIFEST_SCHEMA_VERSION: u16 = 3;
pub const GSC_REFERENCE_VECTOR_SPEC_TOKEN: &str = concat!(
    "gsc-inductive-core-generic-four-v1/",
    "two-nullary=proven-supported;",
    "nonrecursive=proven-supported;",
    "recursive=proven-generated-recursive-call-beta;",
    "path=unknown-unsupported-computation-mode"
);

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GscOutcome<T> {
    Proven(T),
    Unknown(GscUnknownReason),
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GscUnknownReason {
    ResourceExhausted,
    UnsupportedManifest,
    UnsupportedVerifier,
    UnsupportedCode,
    UnsupportedComputationMode,
    UnsupportedBoundary,
    MalformedCode,
    MalformedFrame,
    KernelCouldNotCertify,
    FamilyMismatch,
    PortMismatch,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GscProfile {
    ClosedInductiveCompletionCoreV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClosedInductiveGrammar {
    FiniteStrictlyPositiveV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortGrammar {
    TypedTermAndGeneratedEquationV2,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryRuleGrammar {
    UseThenComputeV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CompilerEquationManifest {
    StructuralEliminatorAndConstructorComputationV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportSemantics {
    PrincipalBirthAndNormalizedTypeSupportV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalOrderRule {
    RuleSourcePortDigestV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Q0RewriteGrammar {
    DependentCoreAndAdmittedFreshComputationV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Q2PresentationGrammar {
    BinderTelescopeAndTransparentAliasV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Q3EdgeGrammar {
    EmptyOriginCutoffV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EquationExtensionGrammar {
    FreshNonrecursiveConstructorComputationV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FamilySchema {
    MultiOutputExactPremiseV2,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextMapGrammar {
    IdentityOnlyV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveUseFamilyShape {
    EveryLiveSingleTermOutputUseFamilyV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshEliminatorHeadGrammar {
    OneBodylessHeadClosedOverCanonicalUseContextV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshEliminatorFillerGrammar {
    HeadAppliedToAllContextVariablesInBinderOrderV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedEquationCoverage {
    EveryComputeEquationReferencingExactUsePortV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshHeadIdentityGrammar {
    SemanticBoundaryFrameUsePortOrderedComputePortsAndClosedTypeV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseCandidateCanonicalOrder {
    UseFamilyUsePortThenComputeFamilyOutputPortDigestV1,
}

/// Complete finite response grammar frozen before any live profile is run.
///
/// A live `G-Use` family has exactly one canonical candidate: one bodyless
/// fresh eliminator closed over its canonical context, plus every generated
/// `G-Compute` equation that refers to that exact use port. Subsets of the
/// generated computation equations are not candidates.
#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseCandidateGrammarV1 {
    pub schema_version: u16,
    pub live_use_family_shape: LiveUseFamilyShape,
    pub fresh_head_grammar: FreshEliminatorHeadGrammar,
    pub filler_grammar: FreshEliminatorFillerGrammar,
    pub generated_equation_coverage: GeneratedEquationCoverage,
    pub fresh_head_identity: FreshHeadIdentityGrammar,
    pub canonical_order: ResponseCandidateCanonicalOrder,
    pub max_live_use_families: u16,
    pub max_compute_families_per_use: u16,
    pub max_equations_per_candidate: u16,
    pub max_response_candidates: u16,
}

impl CanonicalEncode for ResponseCandidateGrammarV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.live_use_family_shape.encode_canonical(encoder);
        self.fresh_head_grammar.encode_canonical(encoder);
        self.filler_grammar.encode_canonical(encoder);
        self.generated_equation_coverage.encode_canonical(encoder);
        self.fresh_head_identity.encode_canonical(encoder);
        self.canonical_order.encode_canonical(encoder);
        encoder.u16(self.max_live_use_families);
        encoder.u16(self.max_compute_families_per_use);
        encoder.u16(self.max_equations_per_candidate);
        encoder.u16(self.max_response_candidates);
    }
}

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum OperationalDerivationRule {
    PublicExact,
    Q0Conversion,
    ContextWeakening,
    CheckedSubstitution,
    LambdaIntroduction,
    Application,
    PairIntroduction,
    FirstProjection,
    SecondProjection,
    EquationReplay,
    QuotientTransport,
}

impl CanonicalEncode for OperationalDerivationRule {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::PublicExact => 0,
            Self::Q0Conversion => 1,
            Self::ContextWeakening => 2,
            Self::CheckedSubstitution => 3,
            Self::LambdaIntroduction => 4,
            Self::Application => 5,
            Self::PairIntroduction => 6,
            Self::FirstProjection => 7,
            Self::SecondProjection => 8,
            Self::EquationReplay => 9,
            Self::QuotientTransport => 10,
        });
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalGoalUniverse {
    PortsVerificationPublicCompilerRegistryAndStructuralSubgoalsV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GroundRuleDisposition {
    ApplicableCertifiedInapplicableOrUnknownV1,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OperationalSaturationOrder {
    GoalDigestThenRuleInventoryThenPremiseDigestV1,
}

/// Exact ordered rule inventory and finite execution bounds for derivability.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct OperationalDerivabilityGrammarV1 {
    pub schema_version: u16,
    pub goal_universe: OperationalGoalUniverse,
    pub ordered_rules: Vec<OperationalDerivationRule>,
    pub ground_rule_disposition: GroundRuleDisposition,
    pub saturation_order: OperationalSaturationOrder,
    pub max_goals: u32,
    pub max_grounded_rule_instances: u32,
    pub max_rule_premises: u16,
    pub max_fixed_point_rounds: u32,
}

impl CanonicalEncode for OperationalDerivabilityGrammarV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.goal_universe.encode_canonical(encoder);
        encoder.sequence(&self.ordered_rules);
        self.ground_rule_disposition.encode_canonical(encoder);
        self.saturation_order.encode_canonical(encoder);
        encoder.u32(self.max_goals);
        encoder.u32(self.max_grounded_rule_instances);
        encoder.u16(self.max_rule_premises);
        encoder.u32(self.max_fixed_point_rounds);
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GscReferenceVectorAgreement {
    UnknownNoCrossImplementationReplayV1,
    ProvenRustReplayAndSafeAgdaProofV1,
    ProvenRustReplayAndPinnedPrimitiveSafeAgdaProofV2,
}

impl CanonicalEncode for GscReferenceVectorAgreement {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::UnknownNoCrossImplementationReplayV1 => 0,
            Self::ProvenRustReplayAndSafeAgdaProofV1 => 1,
            Self::ProvenRustReplayAndPinnedPrimitiveSafeAgdaProofV2 => 2,
        });
    }
}

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum GscReferenceVectorId {
    TwoNullaryConstructors,
    OneNonrecursiveArgument,
    OneRecursiveArgumentGeneratedCallBeta,
    PathComputationMode,
}

impl CanonicalEncode for GscReferenceVectorId {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::TwoNullaryConstructors => 0,
            Self::OneNonrecursiveArgument => 1,
            Self::OneRecursiveArgumentGeneratedCallBeta => 2,
            Self::PathComputationMode => 3,
        });
    }
}

#[derive(
    Clone, Copy, Debug, Deserialize, Eq, Hash, JsonSchema, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(rename_all = "snake_case")]
pub enum GscReferenceVectorDisposition {
    ProvenSupported,
    ProvenGeneratedRecursiveCallBeta,
    UnknownUnsupportedComputationMode,
}

impl CanonicalEncode for GscReferenceVectorDisposition {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::ProvenSupported => 0,
            Self::ProvenGeneratedRecursiveCallBeta => 1,
            Self::UnknownUnsupportedComputationMode => 2,
        });
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GscReferenceVectorCaseV1 {
    pub vector: GscReferenceVectorId,
    pub expected_disposition: GscReferenceVectorDisposition,
}

impl CanonicalEncode for GscReferenceVectorCaseV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.vector.encode_canonical(encoder);
        self.expected_disposition.encode_canonical(encoder);
    }
}

/// Ordered generic vectors used to cross-check the Rust and safe Agda
/// interpreters before a live history profile is evaluated.
#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GscReferenceVectorSuiteV1 {
    pub schema_version: u16,
    pub spec_token: String,
    pub ordered_cases: Vec<GscReferenceVectorCaseV1>,
}

impl GscReferenceVectorSuiteV1 {
    pub fn canonical_digest(&self) -> Digest {
        Digest::of_canonical("pen-demand/gsc-reference-vector-suite/v1", self)
    }
}

impl CanonicalEncode for GscReferenceVectorSuiteV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        encoder.text(&self.spec_token);
        encoder.sequence(&self.ordered_cases);
    }
}

pub fn frozen_gsc_reference_vector_suite_v1() -> GscReferenceVectorSuiteV1 {
    GscReferenceVectorSuiteV1 {
        schema_version: 1,
        spec_token: GSC_REFERENCE_VECTOR_SPEC_TOKEN.to_owned(),
        ordered_cases: vec![
            GscReferenceVectorCaseV1 {
                vector: GscReferenceVectorId::TwoNullaryConstructors,
                expected_disposition: GscReferenceVectorDisposition::ProvenSupported,
            },
            GscReferenceVectorCaseV1 {
                vector: GscReferenceVectorId::OneNonrecursiveArgument,
                expected_disposition: GscReferenceVectorDisposition::ProvenSupported,
            },
            GscReferenceVectorCaseV1 {
                vector: GscReferenceVectorId::OneRecursiveArgumentGeneratedCallBeta,
                expected_disposition:
                    GscReferenceVectorDisposition::ProvenGeneratedRecursiveCallBeta,
            },
            GscReferenceVectorCaseV1 {
                vector: GscReferenceVectorId::PathComputationMode,
                expected_disposition:
                    GscReferenceVectorDisposition::UnknownUnsupportedComputationMode,
            },
        ],
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GscReferenceVectorAgreementArtifactV1 {
    pub schema_version: u16,
    pub semantic_manifest_digest: Digest,
    pub vector_suite_digest: Digest,
    pub rust_replay_source_digest: Digest,
    pub rust_runtime_replay_digest: Digest,
    pub agda_proof_source_digest: Digest,
    pub agda_executable_digest: Digest,
    pub agda_version: String,
    pub agda_primitive_tree_digest: Digest,
    pub agda_primitive_source_file_count: u64,
    pub agda_primitive_source_byte_length: u64,
    pub agda_data_dir_probe_argument_protocol_digest: Digest,
    pub agda_data_dir_probe_stdout_digest: Digest,
    pub agda_data_dir_probe_stderr_digest: Digest,
    pub checker_argument_protocol_digest: Digest,
    pub checker_stdout_digest: Digest,
    pub checker_stderr_digest: Digest,
    pub gate_protocol_digest: Digest,
    pub verified_capability_digest: Digest,
    pub status: GscReferenceVectorAgreement,
}

impl GscReferenceVectorAgreementArtifactV1 {
    pub fn canonical_digest(&self) -> Digest {
        Digest::of_canonical("pen-demand/gsc-reference-vector-agreement/v1", self)
    }
}

impl CanonicalEncode for GscReferenceVectorAgreementArtifactV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.vector_suite_digest.encode_canonical(encoder);
        self.rust_replay_source_digest.encode_canonical(encoder);
        self.rust_runtime_replay_digest.encode_canonical(encoder);
        self.agda_proof_source_digest.encode_canonical(encoder);
        self.agda_executable_digest.encode_canonical(encoder);
        encoder.text(&self.agda_version);
        self.agda_primitive_tree_digest.encode_canonical(encoder);
        encoder.u64(self.agda_primitive_source_file_count);
        encoder.u64(self.agda_primitive_source_byte_length);
        self.agda_data_dir_probe_argument_protocol_digest
            .encode_canonical(encoder);
        self.agda_data_dir_probe_stdout_digest
            .encode_canonical(encoder);
        self.agda_data_dir_probe_stderr_digest
            .encode_canonical(encoder);
        self.checker_argument_protocol_digest
            .encode_canonical(encoder);
        self.checker_stdout_digest.encode_canonical(encoder);
        self.checker_stderr_digest.encode_canonical(encoder);
        self.gate_protocol_digest.encode_canonical(encoder);
        self.verified_capability_digest.encode_canonical(encoder);
        self.status.encode_canonical(encoder);
    }
}

macro_rules! encode_single_variant {
    ($($ty:ty),+ $(,)?) => {
        $(
            impl CanonicalEncode for $ty {
                fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
                    encoder.tag(0);
                }
            }
        )+
    };
}

encode_single_variant!(
    GscProfile,
    ClosedInductiveGrammar,
    PortGrammar,
    HistoryRuleGrammar,
    CompilerEquationManifest,
    SupportSemantics,
    CanonicalOrderRule,
    Q0RewriteGrammar,
    Q2PresentationGrammar,
    Q3EdgeGrammar,
    EquationExtensionGrammar,
    FamilySchema,
    ContextMapGrammar,
    LiveUseFamilyShape,
    FreshEliminatorHeadGrammar,
    FreshEliminatorFillerGrammar,
    GeneratedEquationCoverage,
    FreshHeadIdentityGrammar,
    ResponseCandidateCanonicalOrder,
    OperationalGoalUniverse,
    GroundRuleDisposition,
    OperationalSaturationOrder,
);

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GscSemanticLimits {
    pub max_code_nodes: u32,
    pub max_code_term_depth: u16,
    pub max_universe_level: u16,
    pub max_parameters: u16,
    pub max_indices: u16,
    pub max_constructors: u16,
    pub max_constructor_arguments: u16,
    pub max_boundary_ports: u16,
    pub max_family_premises: u16,
    pub max_family_outputs: u16,
    pub max_verification_judgments: u16,
}

impl CanonicalEncode for GscSemanticLimits {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u32(self.max_code_nodes);
        encoder.u16(self.max_code_term_depth);
        encoder.u16(self.max_universe_level);
        encoder.u16(self.max_parameters);
        encoder.u16(self.max_indices);
        encoder.u16(self.max_constructors);
        encoder.u16(self.max_constructor_arguments);
        encoder.u16(self.max_boundary_ports);
        encoder.u16(self.max_family_premises);
        encoder.u16(self.max_family_outputs);
        encoder.u16(self.max_verification_judgments);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GscSemanticManifestV1 {
    pub schema_version: u16,
    pub profile: GscProfile,
    pub semantic_limits: GscSemanticLimits,
    pub declaration_group_grammar: ClosedInductiveGrammar,
    pub port_grammar: PortGrammar,
    pub history_rule_grammar: HistoryRuleGrammar,
    pub compiler_equations: CompilerEquationManifest,
    pub support_semantics: SupportSemantics,
    pub canonical_order: CanonicalOrderRule,
    pub q0_rewrites: Q0RewriteGrammar,
    pub q2_presentations: Q2PresentationGrammar,
    pub q3_edges: Q3EdgeGrammar,
    pub equation_extension_grammar: EquationExtensionGrammar,
    pub response_candidate_grammar: ResponseCandidateGrammarV1,
    pub derivability_grammar: OperationalDerivabilityGrammarV1,
    pub reference_vector_suite: GscReferenceVectorSuiteV1,
    pub family_schema: FamilySchema,
    pub context_map_grammar: ContextMapGrammar,
}

impl GscSemanticManifestV1 {
    pub fn canonical_digest(&self) -> Digest {
        Digest::of_canonical("pen-demand/gsc-semantic-manifest/v1", self)
    }
}

impl CanonicalEncode for GscSemanticManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.profile.encode_canonical(encoder);
        self.semantic_limits.encode_canonical(encoder);
        self.declaration_group_grammar.encode_canonical(encoder);
        self.port_grammar.encode_canonical(encoder);
        self.history_rule_grammar.encode_canonical(encoder);
        self.compiler_equations.encode_canonical(encoder);
        self.support_semantics.encode_canonical(encoder);
        self.canonical_order.encode_canonical(encoder);
        self.q0_rewrites.encode_canonical(encoder);
        self.q2_presentations.encode_canonical(encoder);
        self.q3_edges.encode_canonical(encoder);
        self.equation_extension_grammar.encode_canonical(encoder);
        self.response_candidate_grammar.encode_canonical(encoder);
        self.derivability_grammar.encode_canonical(encoder);
        self.reference_vector_suite.encode_canonical(encoder);
        self.family_schema.encode_canonical(encoder);
        self.context_map_grammar.encode_canonical(encoder);
    }
}

pub fn frozen_gsc_semantic_manifest_v1() -> GscSemanticManifestV1 {
    GscSemanticManifestV1 {
        schema_version: GSC_SEMANTIC_MANIFEST_SCHEMA_VERSION,
        profile: GscProfile::ClosedInductiveCompletionCoreV1,
        semantic_limits: GscSemanticLimits {
            max_code_nodes: 65_536,
            max_code_term_depth: 64,
            max_universe_level: 256,
            max_parameters: 64,
            max_indices: 64,
            max_constructors: 256,
            max_constructor_arguments: 256,
            max_boundary_ports: 256,
            max_family_premises: 256,
            max_family_outputs: 256,
            max_verification_judgments: 512,
        },
        declaration_group_grammar: ClosedInductiveGrammar::FiniteStrictlyPositiveV1,
        port_grammar: PortGrammar::TypedTermAndGeneratedEquationV2,
        history_rule_grammar: HistoryRuleGrammar::UseThenComputeV1,
        compiler_equations:
            CompilerEquationManifest::StructuralEliminatorAndConstructorComputationV1,
        support_semantics: SupportSemantics::PrincipalBirthAndNormalizedTypeSupportV1,
        canonical_order: CanonicalOrderRule::RuleSourcePortDigestV1,
        q0_rewrites: Q0RewriteGrammar::DependentCoreAndAdmittedFreshComputationV1,
        q2_presentations: Q2PresentationGrammar::BinderTelescopeAndTransparentAliasV1,
        q3_edges: Q3EdgeGrammar::EmptyOriginCutoffV1,
        equation_extension_grammar:
            EquationExtensionGrammar::FreshNonrecursiveConstructorComputationV1,
        response_candidate_grammar: ResponseCandidateGrammarV1 {
            schema_version: 1,
            live_use_family_shape: LiveUseFamilyShape::EveryLiveSingleTermOutputUseFamilyV1,
            fresh_head_grammar:
                FreshEliminatorHeadGrammar::OneBodylessHeadClosedOverCanonicalUseContextV1,
            filler_grammar:
                FreshEliminatorFillerGrammar::HeadAppliedToAllContextVariablesInBinderOrderV1,
            generated_equation_coverage:
                GeneratedEquationCoverage::EveryComputeEquationReferencingExactUsePortV1,
            fresh_head_identity:
                FreshHeadIdentityGrammar::SemanticBoundaryFrameUsePortOrderedComputePortsAndClosedTypeV1,
            canonical_order:
                ResponseCandidateCanonicalOrder::UseFamilyUsePortThenComputeFamilyOutputPortDigestV1,
            max_live_use_families: 256,
            max_compute_families_per_use: 256,
            max_equations_per_candidate: 256,
            max_response_candidates: 256,
        },
        derivability_grammar: OperationalDerivabilityGrammarV1 {
            schema_version: 1,
            goal_universe:
                OperationalGoalUniverse::PortsVerificationPublicCompilerRegistryAndStructuralSubgoalsV1,
            ordered_rules: vec![
                OperationalDerivationRule::PublicExact,
                OperationalDerivationRule::Q0Conversion,
                OperationalDerivationRule::ContextWeakening,
                OperationalDerivationRule::CheckedSubstitution,
                OperationalDerivationRule::LambdaIntroduction,
                OperationalDerivationRule::Application,
                OperationalDerivationRule::PairIntroduction,
                OperationalDerivationRule::FirstProjection,
                OperationalDerivationRule::SecondProjection,
                OperationalDerivationRule::EquationReplay,
                OperationalDerivationRule::QuotientTransport,
            ],
            ground_rule_disposition:
                GroundRuleDisposition::ApplicableCertifiedInapplicableOrUnknownV1,
            saturation_order:
                OperationalSaturationOrder::GoalDigestThenRuleInventoryThenPremiseDigestV1,
            max_goals: 65_536,
            max_grounded_rule_instances: 262_144,
            max_rule_premises: 256,
            max_fixed_point_rounds: 65_536,
        },
        reference_vector_suite: frozen_gsc_reference_vector_suite_v1(),
        family_schema: FamilySchema::MultiOutputExactPremiseV2,
        context_map_grammar: ContextMapGrammar::IdentityOnlyV1,
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedGscSemanticManifest {
    manifest: GscSemanticManifestV1,
    digest: Digest,
}

impl VerifiedGscSemanticManifest {
    pub fn manifest(&self) -> &GscSemanticManifestV1 {
        &self.manifest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub fn limits(&self) -> GscSemanticLimits {
        self.manifest.semantic_limits
    }
}

pub fn verify_gsc_semantic_manifest_v1(
    candidate: &GscSemanticManifestV1,
) -> GscOutcome<VerifiedGscSemanticManifest> {
    let frozen = frozen_gsc_semantic_manifest_v1();
    if candidate != &frozen {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest);
    }
    GscOutcome::Proven(VerifiedGscSemanticManifest {
        manifest: frozen,
        digest: candidate.canonical_digest(),
    })
}

#[derive(Clone, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GscVerifierManifestV1 {
    pub schema_version: u16,
    pub semantic_manifest_digest: Digest,
    pub rust_verifier_digest: Digest,
    pub agda_reference_digest: Digest,
    pub toolchain_digest: Digest,
    pub kernel_protocol_digest: Digest,
    pub normalizer_protocol_digest: Digest,
    pub reference_vector_agreement: GscReferenceVectorAgreementArtifactV1,
}

#[derive(Clone, Debug)]
pub struct VerifiedGscVerifierManifest {
    manifest: GscVerifierManifestV1,
    digest: Digest,
}

impl VerifiedGscVerifierManifest {
    pub fn manifest(&self) -> &GscVerifierManifestV1 {
        &self.manifest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.manifest.semantic_manifest_digest
    }
}

impl GscVerifierManifestV1 {
    pub fn canonical_digest(&self) -> Digest {
        Digest::of_canonical("pen-demand/gsc-verifier-manifest/v1", self)
    }
}

impl CanonicalEncode for GscVerifierManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.rust_verifier_digest.encode_canonical(encoder);
        self.agda_reference_digest.encode_canonical(encoder);
        self.toolchain_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.reference_vector_agreement.encode_canonical(encoder);
    }
}

pub fn current_gsc_verifier_manifest_v1(
    semantic: &VerifiedGscSemanticManifest,
    kernel: &Kernel,
    agreement: &VerifiedGscReferenceAgreement,
) -> GscOutcome<GscVerifierManifestV1> {
    if !agreement.binding_is_valid(semantic) {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier);
    }
    let rust_verifier_digest = gsc_verifier_source_digest();
    let agda_reference_digest = gsc_agda_reference_digest();
    let reference_vector_agreement = GscReferenceVectorAgreementArtifactV1 {
        schema_version: 3,
        semantic_manifest_digest: semantic.digest().clone(),
        vector_suite_digest: agreement.vector_suite_digest().clone(),
        rust_replay_source_digest: rust_verifier_digest.clone(),
        rust_runtime_replay_digest: agreement.rust_replay_digest().clone(),
        agda_proof_source_digest: agda_reference_digest.clone(),
        agda_executable_digest: agreement.agda_executable_digest().clone(),
        agda_version: agreement.agda_version().to_owned(),
        agda_primitive_tree_digest: agreement.agda_primitive_tree_digest().clone(),
        agda_primitive_source_file_count: agreement.agda_primitive_source_file_count(),
        agda_primitive_source_byte_length: agreement.agda_primitive_source_byte_length(),
        agda_data_dir_probe_argument_protocol_digest: agreement
            .agda_data_dir_probe_argument_protocol_digest()
            .clone(),
        agda_data_dir_probe_stdout_digest: agreement.agda_data_dir_probe_stdout_digest().clone(),
        agda_data_dir_probe_stderr_digest: agreement.agda_data_dir_probe_stderr_digest().clone(),
        checker_argument_protocol_digest: agreement.checker_argument_protocol_digest().clone(),
        checker_stdout_digest: agreement.checker_stdout_digest().clone(),
        checker_stderr_digest: agreement.checker_stderr_digest().clone(),
        gate_protocol_digest: agreement.gate_protocol_digest().clone(),
        verified_capability_digest: agreement.digest().clone(),
        status: GscReferenceVectorAgreement::ProvenRustReplayAndPinnedPrimitiveSafeAgdaProofV2,
    };
    GscOutcome::Proven(GscVerifierManifestV1 {
        schema_version: GSC_VERIFIER_MANIFEST_SCHEMA_VERSION,
        semantic_manifest_digest: semantic.digest().clone(),
        rust_verifier_digest,
        agda_reference_digest,
        toolchain_digest: gsc_toolchain_digest(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        reference_vector_agreement,
    })
}

pub fn verify_gsc_verifier_manifest_v1(
    semantic: &VerifiedGscSemanticManifest,
    kernel: &Kernel,
    agreement: &VerifiedGscReferenceAgreement,
    candidate: &GscVerifierManifestV1,
) -> GscOutcome<VerifiedGscVerifierManifest> {
    let current = match current_gsc_verifier_manifest_v1(semantic, kernel, agreement) {
        GscOutcome::Proven(current) => current,
        GscOutcome::Unknown(reason) => return GscOutcome::Unknown(reason),
    };
    if candidate != &current {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedVerifier);
    }
    GscOutcome::Proven(VerifiedGscVerifierManifest {
        digest: current.canonical_digest(),
        manifest: current,
    })
}

pub fn gsc_verifier_source_digest() -> Digest {
    canonical_source_digest(
        "pen-demand/gsc-rust-verifier-source/v1",
        &[
            include_bytes!("mod.rs"),
            include_bytes!("manifest.rs"),
            include_bytes!("inductive.rs"),
            include_bytes!("family.rs"),
            include_bytes!("compiler.rs"),
            include_bytes!("reference.rs"),
            include_bytes!("tests.rs"),
            include_bytes!("../lib.rs"),
            include_bytes!("../../Cargo.toml"),
            include_bytes!("../../../pen-gf2-agda/src/lib.rs"),
            include_bytes!("../../../pen-gf2-agda/src/evidence.rs"),
            include_bytes!("../../../pen-gf2-agda/Cargo.toml"),
        ],
    )
}

pub fn gsc_agda_reference_digest() -> Digest {
    canonical_source_digest(
        "pen-demand/gsc-agda-reference-source/v1",
        &[include_bytes!("../../agda/GscInductiveCoreV1.agda")],
    )
}

pub fn gsc_toolchain_digest() -> Digest {
    canonical_source_digest(
        "pen-demand/gsc-toolchain/v1",
        &[
            include_bytes!("../../../../rust-toolchain.toml"),
            include_bytes!("../../../../.cargo/config.toml"),
        ],
    )
}

pub fn gsc_reference_agreement_gate_digest() -> Digest {
    canonical_source_digest(
        "pen-demand/gsc-reference-agreement-gate/v1",
        &[include_bytes!(
            "../../scripts/check_gsc_reference_agreement.ps1"
        )],
    )
}

fn canonical_source_digest(domain: &str, chunks: &[&[u8]]) -> Digest {
    let canonical = chunks
        .iter()
        .map(|chunk| {
            let text = std::str::from_utf8(chunk).expect("embedded verifier source is UTF-8");
            let normalized = text.replace("\r\n", "\n");
            assert!(
                !normalized.contains('\r'),
                "embedded verifier source contains a bare carriage return"
            );
            normalized.into_bytes()
        })
        .collect::<Vec<_>>();
    let slices = canonical.iter().map(Vec::as_slice).collect::<Vec<_>>();
    Digest::of_domain_chunks(domain, &slices)
}
