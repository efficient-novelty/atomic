//! JG2b2b0 structural occurrence grammar.
//!
//! This module freezes the history-independent syntax needed to identify a
//! declaration subterm structurally: two ordered declaration roots, the exact
//! twelve-form kernel term grammar, thirteen field-specific child steps,
//! three binder-entering steps, deterministic preorder, and the fields that a
//! later history-bound occurrence identity must retain.  It deliberately
//! mints no birth occurrence, typing fact, classifier disposition,
//! substitution, theorem, or naturality authority.

use crate::{
    VerifiedGenerativeCapabilityConstructorGrammarV1,
    VerifiedGenerativeSubstitutionNaturalityProtocolV1,
    VerifiedPreExposureGenerativeCapabilityGrammarV1,
    generative_capability_kernel_configuration_digest_v1,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, Declaration, Digest, Kernel, Term};

pub const GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_SCHEMA_VERSION_V1: u16 = 1;
pub const GENERATIVE_STRUCTURAL_OCCURRENCE_ROOT_COUNT_V1: usize = 2;
pub const GENERATIVE_STRUCTURAL_OCCURRENCE_NODE_KIND_COUNT_V1: usize = 12;
pub const GENERATIVE_STRUCTURAL_OCCURRENCE_PATH_STEP_COUNT_V1: usize = 13;
pub const GENERATIVE_STRUCTURAL_OCCURRENCE_BINDER_STEP_COUNT_V1: usize = 3;
pub const GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_DOMAIN_COUNT_V1: usize = 2;
pub const GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_FIELD_COUNT_V1: usize = 14;
pub const GENERATIVE_STRUCTURAL_OCCURRENCE_RULE_COUNT_V1: usize = 18;

const GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b0/structural-occurrence-grammar-definition/v1";
const GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b0/structural-occurrence-grammar/v1";
const GENERATIVE_STRUCTURAL_DECLARATION_INPUT_DIGEST_DOMAIN_V1: &str =
    "law-v2/jg2b2b0/structural-declaration-input/v1";
const GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1: &str = "law-v2/jg2b2b0/structural-subject/v1";
const GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_ROOT_TAG_V1: u8 = 0xf3;
const GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_ROOT_TAG_V1: u8 = 0xf4;

/// Ordered declaration fields from which structural traversal begins.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeStructuralOccurrenceRootV1 {
    DeclarationType,
    DeclarationBody,
}

pub const GENERATIVE_STRUCTURAL_OCCURRENCE_ROOTS_V1: [GenerativeStructuralOccurrenceRootV1;
    GENERATIVE_STRUCTURAL_OCCURRENCE_ROOT_COUNT_V1] = [
    GenerativeStructuralOccurrenceRootV1::DeclarationType,
    GenerativeStructuralOccurrenceRootV1::DeclarationBody,
];

impl CanonicalEncode for GenerativeStructuralOccurrenceRootV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::DeclarationType => 0x00,
            Self::DeclarationBody => 0x01,
        });
    }
}

/// Exact closed image of the twelve current [`Term`] forms.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeStructuralOccurrenceNodeKindV1 {
    Sort,
    Var,
    Global,
    Pi,
    Sigma,
    Lambda,
    Apply,
    Pair,
    First,
    Second,
    UnitType,
    Unit,
}

pub const GENERATIVE_STRUCTURAL_OCCURRENCE_NODE_KINDS_V1:
    [GenerativeStructuralOccurrenceNodeKindV1; GENERATIVE_STRUCTURAL_OCCURRENCE_NODE_KIND_COUNT_V1] = [
    GenerativeStructuralOccurrenceNodeKindV1::Sort,
    GenerativeStructuralOccurrenceNodeKindV1::Var,
    GenerativeStructuralOccurrenceNodeKindV1::Global,
    GenerativeStructuralOccurrenceNodeKindV1::Pi,
    GenerativeStructuralOccurrenceNodeKindV1::Sigma,
    GenerativeStructuralOccurrenceNodeKindV1::Lambda,
    GenerativeStructuralOccurrenceNodeKindV1::Apply,
    GenerativeStructuralOccurrenceNodeKindV1::Pair,
    GenerativeStructuralOccurrenceNodeKindV1::First,
    GenerativeStructuralOccurrenceNodeKindV1::Second,
    GenerativeStructuralOccurrenceNodeKindV1::UnitType,
    GenerativeStructuralOccurrenceNodeKindV1::Unit,
];

impl CanonicalEncode for GenerativeStructuralOccurrenceNodeKindV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Sort => 0x10,
            Self::Var => 0x11,
            Self::Global => 0x12,
            Self::Pi => 0x13,
            Self::Sigma => 0x14,
            Self::Lambda => 0x15,
            Self::Apply => 0x16,
            Self::Pair => 0x17,
            Self::First => 0x18,
            Self::Second => 0x19,
            Self::UnitType => 0x1a,
            Self::Unit => 0x1b,
        });
    }
}

/// Field-specific structural path steps.  Generic child ordinals are not
/// identities because they would erase the parent-field meaning.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeStructuralOccurrencePathStepV1 {
    PiParameter,
    PiBody,
    SigmaParameter,
    SigmaBody,
    LambdaParameterType,
    LambdaBody,
    ApplyFunction,
    ApplyArgument,
    PairSigmaType,
    PairFirst,
    PairSecond,
    FirstPair,
    SecondPair,
}

pub const GENERATIVE_STRUCTURAL_OCCURRENCE_PATH_STEPS_V1:
    [GenerativeStructuralOccurrencePathStepV1; GENERATIVE_STRUCTURAL_OCCURRENCE_PATH_STEP_COUNT_V1] = [
    GenerativeStructuralOccurrencePathStepV1::PiParameter,
    GenerativeStructuralOccurrencePathStepV1::PiBody,
    GenerativeStructuralOccurrencePathStepV1::SigmaParameter,
    GenerativeStructuralOccurrencePathStepV1::SigmaBody,
    GenerativeStructuralOccurrencePathStepV1::LambdaParameterType,
    GenerativeStructuralOccurrencePathStepV1::LambdaBody,
    GenerativeStructuralOccurrencePathStepV1::ApplyFunction,
    GenerativeStructuralOccurrencePathStepV1::ApplyArgument,
    GenerativeStructuralOccurrencePathStepV1::PairSigmaType,
    GenerativeStructuralOccurrencePathStepV1::PairFirst,
    GenerativeStructuralOccurrencePathStepV1::PairSecond,
    GenerativeStructuralOccurrencePathStepV1::FirstPair,
    GenerativeStructuralOccurrencePathStepV1::SecondPair,
];

pub const GENERATIVE_STRUCTURAL_OCCURRENCE_BINDER_STEPS_V1:
    [GenerativeStructuralOccurrencePathStepV1;
        GENERATIVE_STRUCTURAL_OCCURRENCE_BINDER_STEP_COUNT_V1] = [
    GenerativeStructuralOccurrencePathStepV1::PiBody,
    GenerativeStructuralOccurrencePathStepV1::SigmaBody,
    GenerativeStructuralOccurrencePathStepV1::LambdaBody,
];

impl GenerativeStructuralOccurrencePathStepV1 {
    pub const fn parent_kind(self) -> GenerativeStructuralOccurrenceNodeKindV1 {
        match self {
            Self::PiParameter | Self::PiBody => GenerativeStructuralOccurrenceNodeKindV1::Pi,
            Self::SigmaParameter | Self::SigmaBody => {
                GenerativeStructuralOccurrenceNodeKindV1::Sigma
            }
            Self::LambdaParameterType | Self::LambdaBody => {
                GenerativeStructuralOccurrenceNodeKindV1::Lambda
            }
            Self::ApplyFunction | Self::ApplyArgument => {
                GenerativeStructuralOccurrenceNodeKindV1::Apply
            }
            Self::PairSigmaType | Self::PairFirst | Self::PairSecond => {
                GenerativeStructuralOccurrenceNodeKindV1::Pair
            }
            Self::FirstPair => GenerativeStructuralOccurrenceNodeKindV1::First,
            Self::SecondPair => GenerativeStructuralOccurrenceNodeKindV1::Second,
        }
    }

    pub const fn child_ordinal(self) -> u8 {
        match self {
            Self::PiParameter
            | Self::SigmaParameter
            | Self::LambdaParameterType
            | Self::ApplyFunction
            | Self::PairSigmaType
            | Self::FirstPair
            | Self::SecondPair => 0,
            Self::PiBody
            | Self::SigmaBody
            | Self::LambdaBody
            | Self::ApplyArgument
            | Self::PairFirst => 1,
            Self::PairSecond => 2,
        }
    }

    pub const fn enters_binder(self) -> bool {
        matches!(self, Self::PiBody | Self::SigmaBody | Self::LambdaBody)
    }
}

impl CanonicalEncode for GenerativeStructuralOccurrencePathStepV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::PiParameter => 0x20,
            Self::PiBody => 0x21,
            Self::SigmaParameter => 0x22,
            Self::SigmaBody => 0x23,
            Self::LambdaParameterType => 0x24,
            Self::LambdaBody => 0x25,
            Self::ApplyFunction => 0x26,
            Self::ApplyArgument => 0x27,
            Self::PairSigmaType => 0x28,
            Self::PairFirst => 0x29,
            Self::PairSecond => 0x2a,
            Self::FirstPair => 0x2b,
            Self::SecondPair => 0x2c,
        });
    }
}

/// Separate comparison domains for normalized structure and exact authority.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeStructuralOccurrenceIdentityDomainV1 {
    NormalizedStructuralOccurrence,
    ExactOccurrenceAuthority,
}

pub const GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_DOMAINS_V1:
    [GenerativeStructuralOccurrenceIdentityDomainV1;
        GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_DOMAIN_COUNT_V1] = [
    GenerativeStructuralOccurrenceIdentityDomainV1::NormalizedStructuralOccurrence,
    GenerativeStructuralOccurrenceIdentityDomainV1::ExactOccurrenceAuthority,
];

impl CanonicalEncode for GenerativeStructuralOccurrenceIdentityDomainV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::NormalizedStructuralOccurrence => 0x30,
            Self::ExactOccurrenceAuthority => 0x31,
        });
    }
}

/// Exact fields required of the later factual occurrence identity.
///
/// Binder depth and local context are derived from the path; their inclusion
/// here requires the later verifier to recompute them rather than accept them
/// as independent caller claims.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeStructuralOccurrenceIdentityFieldV1 {
    CompleteThroughHeadCommitment,
    BirthEventId,
    BirthEventOrdinal,
    DeclarationGlobalId,
    BirthExtensionOffset,
    TerminalDeclarationOrdinal,
    BirthPrefixSignatureDigest,
    DeclarationRoot,
    BinderDepth,
    LocalBinderContextDigest,
    StructuralPath,
    NodeKind,
    NormalizedStructuralSubjectDigest,
    ExactStageEvidenceDigest,
}

pub const GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_FIELDS_V1:
    [GenerativeStructuralOccurrenceIdentityFieldV1;
        GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_FIELD_COUNT_V1] = [
    GenerativeStructuralOccurrenceIdentityFieldV1::CompleteThroughHeadCommitment,
    GenerativeStructuralOccurrenceIdentityFieldV1::BirthEventId,
    GenerativeStructuralOccurrenceIdentityFieldV1::BirthEventOrdinal,
    GenerativeStructuralOccurrenceIdentityFieldV1::DeclarationGlobalId,
    GenerativeStructuralOccurrenceIdentityFieldV1::BirthExtensionOffset,
    GenerativeStructuralOccurrenceIdentityFieldV1::TerminalDeclarationOrdinal,
    GenerativeStructuralOccurrenceIdentityFieldV1::BirthPrefixSignatureDigest,
    GenerativeStructuralOccurrenceIdentityFieldV1::DeclarationRoot,
    GenerativeStructuralOccurrenceIdentityFieldV1::BinderDepth,
    GenerativeStructuralOccurrenceIdentityFieldV1::LocalBinderContextDigest,
    GenerativeStructuralOccurrenceIdentityFieldV1::StructuralPath,
    GenerativeStructuralOccurrenceIdentityFieldV1::NodeKind,
    GenerativeStructuralOccurrenceIdentityFieldV1::NormalizedStructuralSubjectDigest,
    GenerativeStructuralOccurrenceIdentityFieldV1::ExactStageEvidenceDigest,
];

impl CanonicalEncode for GenerativeStructuralOccurrenceIdentityFieldV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::CompleteThroughHeadCommitment => 0x40,
            Self::BirthEventId => 0x41,
            Self::BirthEventOrdinal => 0x42,
            Self::DeclarationGlobalId => 0x43,
            Self::BirthExtensionOffset => 0x44,
            Self::TerminalDeclarationOrdinal => 0x45,
            Self::BirthPrefixSignatureDigest => 0x46,
            Self::DeclarationRoot => 0x47,
            Self::BinderDepth => 0x48,
            Self::LocalBinderContextDigest => 0x49,
            Self::StructuralPath => 0x4a,
            Self::NodeKind => 0x4b,
            Self::NormalizedStructuralSubjectDigest => 0x4c,
            Self::ExactStageEvidenceDigest => 0x4d,
        });
    }
}

/// Closed semantic rules governing the grammar and its authority boundary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GenerativeStructuralOccurrenceGrammarRuleV1 {
    GrammarIsHistoryIndependent,
    DeclarationRootsAreTypeThenPresentBody,
    TraversalIsDeterministicPreorder,
    ChildStepsAreClosedFieldSpecificAndParentChecked,
    OnlyDependentBodiesEnterBinders,
    ContextEntriesAreOldestFirst,
    DeBruijnZeroNamesNewestBinder,
    BinderDepthAndContextDeriveFromPath,
    EqualSubjectsAtDistinctPathsRemainDistinct,
    NormalizedSubjectAndExactEvidenceRemainSeparate,
    NormalizedIdentityExcludesCompleteHeadAndExactStageEvidence,
    ExactAuthorityIdentityExtendsNormalizedWithCompleteHeadAndExactStageEvidence,
    StructuralSubjectDigestIsBottomUpFieldOrderedMerkle,
    FutureOccurrenceIdentityRetainsEveryFrozenField,
    CallerSyntaxTraversalMintsNoFactualOccurrence,
    NestedTypingAndClassificationRequireLaterAuthority,
    ResourceExhaustionMintsNoPartialFact,
    GrammarMintsNoSubstitutionNaturalityOrDownstreamAuthority,
}

pub const GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_RULES_V1:
    [GenerativeStructuralOccurrenceGrammarRuleV1;
        GENERATIVE_STRUCTURAL_OCCURRENCE_RULE_COUNT_V1] = [
    GenerativeStructuralOccurrenceGrammarRuleV1::GrammarIsHistoryIndependent,
    GenerativeStructuralOccurrenceGrammarRuleV1::DeclarationRootsAreTypeThenPresentBody,
    GenerativeStructuralOccurrenceGrammarRuleV1::TraversalIsDeterministicPreorder,
    GenerativeStructuralOccurrenceGrammarRuleV1::ChildStepsAreClosedFieldSpecificAndParentChecked,
    GenerativeStructuralOccurrenceGrammarRuleV1::OnlyDependentBodiesEnterBinders,
    GenerativeStructuralOccurrenceGrammarRuleV1::ContextEntriesAreOldestFirst,
    GenerativeStructuralOccurrenceGrammarRuleV1::DeBruijnZeroNamesNewestBinder,
    GenerativeStructuralOccurrenceGrammarRuleV1::BinderDepthAndContextDeriveFromPath,
    GenerativeStructuralOccurrenceGrammarRuleV1::EqualSubjectsAtDistinctPathsRemainDistinct,
    GenerativeStructuralOccurrenceGrammarRuleV1::NormalizedSubjectAndExactEvidenceRemainSeparate,
    GenerativeStructuralOccurrenceGrammarRuleV1::NormalizedIdentityExcludesCompleteHeadAndExactStageEvidence,
    GenerativeStructuralOccurrenceGrammarRuleV1::ExactAuthorityIdentityExtendsNormalizedWithCompleteHeadAndExactStageEvidence,
    GenerativeStructuralOccurrenceGrammarRuleV1::StructuralSubjectDigestIsBottomUpFieldOrderedMerkle,
    GenerativeStructuralOccurrenceGrammarRuleV1::FutureOccurrenceIdentityRetainsEveryFrozenField,
    GenerativeStructuralOccurrenceGrammarRuleV1::CallerSyntaxTraversalMintsNoFactualOccurrence,
    GenerativeStructuralOccurrenceGrammarRuleV1::NestedTypingAndClassificationRequireLaterAuthority,
    GenerativeStructuralOccurrenceGrammarRuleV1::ResourceExhaustionMintsNoPartialFact,
    GenerativeStructuralOccurrenceGrammarRuleV1::GrammarMintsNoSubstitutionNaturalityOrDownstreamAuthority,
];

impl CanonicalEncode for GenerativeStructuralOccurrenceGrammarRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::GrammarIsHistoryIndependent => 0x60,
            Self::DeclarationRootsAreTypeThenPresentBody => 0x61,
            Self::TraversalIsDeterministicPreorder => 0x62,
            Self::ChildStepsAreClosedFieldSpecificAndParentChecked => 0x63,
            Self::OnlyDependentBodiesEnterBinders => 0x64,
            Self::ContextEntriesAreOldestFirst => 0x65,
            Self::DeBruijnZeroNamesNewestBinder => 0x66,
            Self::BinderDepthAndContextDeriveFromPath => 0x67,
            Self::EqualSubjectsAtDistinctPathsRemainDistinct => 0x68,
            Self::NormalizedSubjectAndExactEvidenceRemainSeparate => 0x69,
            Self::NormalizedIdentityExcludesCompleteHeadAndExactStageEvidence => 0x6a,
            Self::ExactAuthorityIdentityExtendsNormalizedWithCompleteHeadAndExactStageEvidence => {
                0x6b
            }
            Self::StructuralSubjectDigestIsBottomUpFieldOrderedMerkle => 0x6c,
            Self::FutureOccurrenceIdentityRetainsEveryFrozenField => 0x6d,
            Self::CallerSyntaxTraversalMintsNoFactualOccurrence => 0x6e,
            Self::NestedTypingAndClassificationRequireLaterAuthority => 0x6f,
            Self::ResourceExhaustionMintsNoPartialFact => 0x70,
            Self::GrammarMintsNoSubstitutionNaturalityOrDownstreamAuthority => 0x71,
        });
    }
}

/// Exact unverified static definition proposal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerativeStructuralOccurrenceDefinitionManifestV1 {
    pub schema_version: u16,
    pub roots: Vec<GenerativeStructuralOccurrenceRootV1>,
    pub node_kinds: Vec<GenerativeStructuralOccurrenceNodeKindV1>,
    pub path_steps: Vec<GenerativeStructuralOccurrencePathStepV1>,
    pub binder_entering_steps: Vec<GenerativeStructuralOccurrencePathStepV1>,
    pub identity_domains: Vec<GenerativeStructuralOccurrenceIdentityDomainV1>,
    pub identity_fields: Vec<GenerativeStructuralOccurrenceIdentityFieldV1>,
    pub rules: Vec<GenerativeStructuralOccurrenceGrammarRuleV1>,
}

impl CanonicalEncode for GenerativeStructuralOccurrenceDefinitionManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_ROOT_TAG_V1);
        encoder.u16(self.schema_version);
        encoder.sequence(&self.roots);
        encoder.sequence(&self.node_kinds);
        encoder.sequence(&self.path_steps);
        encoder.sequence(&self.binder_entering_steps);
        encoder.sequence(&self.identity_domains);
        encoder.sequence(&self.identity_fields);
        encoder.sequence(&self.rules);
    }
}

pub fn proposed_generative_structural_occurrence_definition_v1()
-> GenerativeStructuralOccurrenceDefinitionManifestV1 {
    GenerativeStructuralOccurrenceDefinitionManifestV1 {
        schema_version: GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_SCHEMA_VERSION_V1,
        roots: GENERATIVE_STRUCTURAL_OCCURRENCE_ROOTS_V1.to_vec(),
        node_kinds: GENERATIVE_STRUCTURAL_OCCURRENCE_NODE_KINDS_V1.to_vec(),
        path_steps: GENERATIVE_STRUCTURAL_OCCURRENCE_PATH_STEPS_V1.to_vec(),
        binder_entering_steps: GENERATIVE_STRUCTURAL_OCCURRENCE_BINDER_STEPS_V1.to_vec(),
        identity_domains: GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_DOMAINS_V1.to_vec(),
        identity_fields: GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_FIELDS_V1.to_vec(),
        rules: GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_RULES_V1.to_vec(),
    }
}

/// Frozen 123-byte transcript of the exact JG2b2b0 static definition.
pub const CANONICAL_GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_BYTES_V1: &[u8] = &[
    0xf3, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x0c, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a,
    0x1b, 0x0d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26,
    0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x21, 0x23,
    0x25, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, 0x31, 0x0e, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c,
    0x4d, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71,
];

/// Frozen domain-separated identity of the static definition transcript.
pub const CANONICAL_GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_DIGEST_V1: &str =
    "blake3:bb514d85e64f3f530633de7f0f5d48c17d61d4e6156b364061cd740accd94b80";

/// Full unverified grammar proposal binding JG2b2a and the exact kernel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerativeStructuralOccurrenceGrammarManifestV1 {
    pub definition: GenerativeStructuralOccurrenceDefinitionManifestV1,
    pub substitution_naturality_protocol_manifest_digest: Digest,
    pub jg1_manifest_digest: Digest,
    pub constructor_grammar_manifest_digest: Digest,
    pub scope_grammar_digest: Digest,
    pub kernel_protocol_digest: Digest,
    pub normalizer_protocol_digest: Digest,
    pub kernel_configuration_digest: Digest,
}

impl CanonicalEncode for GenerativeStructuralOccurrenceGrammarManifestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_ROOT_TAG_V1);
        self.definition.encode_canonical(encoder);
        self.substitution_naturality_protocol_manifest_digest
            .encode_canonical(encoder);
        self.jg1_manifest_digest.encode_canonical(encoder);
        self.constructor_grammar_manifest_digest
            .encode_canonical(encoder);
        self.scope_grammar_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.kernel_configuration_digest.encode_canonical(encoder);
    }
}

pub fn proposed_generative_structural_occurrence_grammar_v1(
    protocol: &VerifiedGenerativeSubstitutionNaturalityProtocolV1,
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
) -> GenerativeStructuralOccurrenceGrammarManifestV1 {
    GenerativeStructuralOccurrenceGrammarManifestV1 {
        definition: proposed_generative_structural_occurrence_definition_v1(),
        substitution_naturality_protocol_manifest_digest: protocol.manifest_digest().clone(),
        jg1_manifest_digest: jg1.manifest_digest().clone(),
        constructor_grammar_manifest_digest: constructors.manifest_digest().clone(),
        scope_grammar_digest: constructors.scope_grammar_digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        kernel_configuration_digest: generative_capability_kernel_configuration_digest_v1(kernel),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerativeStructuralOccurrenceGrammarFailureV1 {
    SchemaVersionMismatch,
    RootVocabularyMismatch,
    NodeKindVocabularyMismatch,
    PathStepVocabularyMismatch,
    BinderStepVocabularyMismatch,
    IdentityDomainVocabularyMismatch,
    IdentityFieldVocabularyMismatch,
    RuleVocabularyMismatch,
    CanonicalDefinitionTranscriptMismatch,
    CanonicalDefinitionDigestMismatch,
    ConstructorGrammarJg1Mismatch,
    ProtocolJg1Mismatch,
    ProtocolConstructorGrammarMismatch,
    ProtocolScopeGrammarMismatch,
    ProtocolKernelMismatch,
    ProtocolNormalizerMismatch,
    ProtocolKernelConfigurationMismatch,
    ProtocolManifestDigestMismatch,
    Jg1ManifestDigestMismatch,
    ConstructorGrammarManifestDigestMismatch,
    ScopeGrammarDigestMismatch,
    KernelProtocolMismatch,
    NormalizerProtocolMismatch,
    KernelConfigurationMismatch,
}

impl std::fmt::Display for GenerativeStructuralOccurrenceGrammarFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::SchemaVersionMismatch => "the structural occurrence schema is not frozen V1",
            Self::RootVocabularyMismatch => "the declaration-root vocabulary or order differs",
            Self::NodeKindVocabularyMismatch => "the term-node vocabulary or order differs",
            Self::PathStepVocabularyMismatch => {
                "the structural path-step vocabulary or order differs"
            }
            Self::BinderStepVocabularyMismatch => {
                "the binder-entering path-step vocabulary differs"
            }
            Self::IdentityDomainVocabularyMismatch => {
                "the normalized/exact occurrence identity domains differ"
            }
            Self::IdentityFieldVocabularyMismatch => "the future occurrence identity fields differ",
            Self::RuleVocabularyMismatch => "the structural occurrence rules or order differ",
            Self::CanonicalDefinitionTranscriptMismatch => {
                "the structural definition does not encode to the frozen transcript"
            }
            Self::CanonicalDefinitionDigestMismatch => {
                "the structural definition transcript has the wrong digest"
            }
            Self::ConstructorGrammarJg1Mismatch => {
                "the constructor grammar is not bound to the supplied JG1 grammar"
            }
            Self::ProtocolJg1Mismatch => {
                "the JG2b2a protocol is not bound to the supplied JG1 grammar"
            }
            Self::ProtocolConstructorGrammarMismatch => {
                "the JG2b2a protocol is not bound to the supplied constructor grammar"
            }
            Self::ProtocolScopeGrammarMismatch => {
                "the JG2b2a protocol is not bound to the supplied scope grammar"
            }
            Self::ProtocolKernelMismatch => {
                "the JG2b2a protocol is not bound to this kernel protocol"
            }
            Self::ProtocolNormalizerMismatch => {
                "the JG2b2a protocol is not bound to this normalizer protocol"
            }
            Self::ProtocolKernelConfigurationMismatch => {
                "the JG2b2a protocol is not bound to this kernel configuration"
            }
            Self::ProtocolManifestDigestMismatch => {
                "the grammar proposal is bound to a different JG2b2a protocol"
            }
            Self::Jg1ManifestDigestMismatch => {
                "the grammar proposal is bound to a different JG1 grammar"
            }
            Self::ConstructorGrammarManifestDigestMismatch => {
                "the grammar proposal is bound to a different constructor grammar"
            }
            Self::ScopeGrammarDigestMismatch => {
                "the grammar proposal is bound to a different scope grammar"
            }
            Self::KernelProtocolMismatch => {
                "the grammar proposal is bound to a different kernel protocol"
            }
            Self::NormalizerProtocolMismatch => {
                "the grammar proposal is bound to a different normalizer protocol"
            }
            Self::KernelConfigurationMismatch => {
                "the grammar proposal is bound to a different kernel configuration"
            }
        })
    }
}

impl std::error::Error for GenerativeStructuralOccurrenceGrammarFailureV1 {}

/// Opaque, remintable proof of the exact JG2b2b0 grammar.
///
/// It cannot be manufactured directly:
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedGenerativeStructuralOccurrenceGrammarV1;
/// let _forged = VerifiedGenerativeStructuralOccurrenceGrammarV1 {};
/// ```
///
/// It has no default or digest-promotion constructor:
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedGenerativeStructuralOccurrenceGrammarV1;
/// let _forged: VerifiedGenerativeStructuralOccurrenceGrammarV1 = Default::default();
/// ```
///
/// ```compile_fail
/// use pen_generative_audit::VerifiedGenerativeStructuralOccurrenceGrammarV1;
/// use pen_kernel::Digest;
/// let digest = Digest::of_bytes(b"identifier-not-evidence");
/// let _forged = VerifiedGenerativeStructuralOccurrenceGrammarV1::from_digest(digest);
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedGenerativeStructuralOccurrenceGrammarV1 {
    schema_version: u16,
    definition_digest: Digest,
    substitution_naturality_protocol_manifest_digest: Digest,
    jg1_manifest_digest: Digest,
    constructor_grammar_manifest_digest: Digest,
    scope_grammar_digest: Digest,
    kernel_protocol_digest: Digest,
    normalizer_protocol_digest: Digest,
    kernel_configuration_digest: Digest,
    manifest_digest: Digest,
}

impl VerifiedGenerativeStructuralOccurrenceGrammarV1 {
    pub const fn schema_version(&self) -> u16 {
        self.schema_version
    }

    pub fn roots(&self) -> &'static [GenerativeStructuralOccurrenceRootV1] {
        &GENERATIVE_STRUCTURAL_OCCURRENCE_ROOTS_V1
    }

    pub fn node_kinds(&self) -> &'static [GenerativeStructuralOccurrenceNodeKindV1] {
        &GENERATIVE_STRUCTURAL_OCCURRENCE_NODE_KINDS_V1
    }

    pub fn path_steps(&self) -> &'static [GenerativeStructuralOccurrencePathStepV1] {
        &GENERATIVE_STRUCTURAL_OCCURRENCE_PATH_STEPS_V1
    }

    pub fn binder_entering_steps(&self) -> &'static [GenerativeStructuralOccurrencePathStepV1] {
        &GENERATIVE_STRUCTURAL_OCCURRENCE_BINDER_STEPS_V1
    }

    pub fn identity_domains(&self) -> &'static [GenerativeStructuralOccurrenceIdentityDomainV1] {
        &GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_DOMAINS_V1
    }

    pub fn identity_fields(&self) -> &'static [GenerativeStructuralOccurrenceIdentityFieldV1] {
        &GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_FIELDS_V1
    }

    pub fn rules(&self) -> &'static [GenerativeStructuralOccurrenceGrammarRuleV1] {
        &GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_RULES_V1
    }

    pub fn definition_digest(&self) -> &Digest {
        &self.definition_digest
    }

    pub fn substitution_naturality_protocol_manifest_digest(&self) -> &Digest {
        &self.substitution_naturality_protocol_manifest_digest
    }

    pub fn jg1_manifest_digest(&self) -> &Digest {
        &self.jg1_manifest_digest
    }

    pub fn constructor_grammar_manifest_digest(&self) -> &Digest {
        &self.constructor_grammar_manifest_digest
    }

    pub fn scope_grammar_digest(&self) -> &Digest {
        &self.scope_grammar_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.normalizer_protocol_digest
    }

    pub fn kernel_configuration_digest(&self) -> &Digest {
        &self.kernel_configuration_digest
    }

    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }
}

pub fn verify_generative_structural_occurrence_grammar_v1(
    protocol: &VerifiedGenerativeSubstitutionNaturalityProtocolV1,
    jg1: &VerifiedPreExposureGenerativeCapabilityGrammarV1,
    constructors: &VerifiedGenerativeCapabilityConstructorGrammarV1,
    kernel: &Kernel,
    manifest: &GenerativeStructuralOccurrenceGrammarManifestV1,
) -> Result<
    VerifiedGenerativeStructuralOccurrenceGrammarV1,
    GenerativeStructuralOccurrenceGrammarFailureV1,
> {
    let definition = &manifest.definition;
    if definition.schema_version != GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_SCHEMA_VERSION_V1 {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::SchemaVersionMismatch);
    }
    if definition.roots.as_slice() != GENERATIVE_STRUCTURAL_OCCURRENCE_ROOTS_V1 {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::RootVocabularyMismatch);
    }
    if definition.node_kinds.as_slice() != GENERATIVE_STRUCTURAL_OCCURRENCE_NODE_KINDS_V1 {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::NodeKindVocabularyMismatch);
    }
    if definition.path_steps.as_slice() != GENERATIVE_STRUCTURAL_OCCURRENCE_PATH_STEPS_V1 {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::PathStepVocabularyMismatch);
    }
    if definition.binder_entering_steps.as_slice()
        != GENERATIVE_STRUCTURAL_OCCURRENCE_BINDER_STEPS_V1
    {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::BinderStepVocabularyMismatch);
    }
    if definition.identity_domains.as_slice()
        != GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_DOMAINS_V1
    {
        return Err(
            GenerativeStructuralOccurrenceGrammarFailureV1::IdentityDomainVocabularyMismatch,
        );
    }
    if definition.identity_fields.as_slice() != GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_FIELDS_V1
    {
        return Err(
            GenerativeStructuralOccurrenceGrammarFailureV1::IdentityFieldVocabularyMismatch,
        );
    }
    if definition.rules.as_slice() != GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_RULES_V1 {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::RuleVocabularyMismatch);
    }

    let mut definition_encoder = CanonicalEncoder::new();
    definition.encode_canonical(&mut definition_encoder);
    if definition_encoder.as_bytes()
        != CANONICAL_GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_BYTES_V1
    {
        return Err(
            GenerativeStructuralOccurrenceGrammarFailureV1::CanonicalDefinitionTranscriptMismatch,
        );
    }
    let definition_digest = Digest::of_domain_bytes(
        GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_DIGEST_DOMAIN_V1,
        definition_encoder.as_bytes(),
    );
    if definition_digest.as_str() != CANONICAL_GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_DIGEST_V1
    {
        return Err(
            GenerativeStructuralOccurrenceGrammarFailureV1::CanonicalDefinitionDigestMismatch,
        );
    }

    if constructors.jg1_manifest_digest() != jg1.manifest_digest() {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::ConstructorGrammarJg1Mismatch);
    }
    if protocol.jg1_manifest_digest() != jg1.manifest_digest() {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::ProtocolJg1Mismatch);
    }
    if protocol.constructor_grammar_manifest_digest() != constructors.manifest_digest() {
        return Err(
            GenerativeStructuralOccurrenceGrammarFailureV1::ProtocolConstructorGrammarMismatch,
        );
    }
    if protocol.scope_grammar_digest() != constructors.scope_grammar_digest() {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::ProtocolScopeGrammarMismatch);
    }

    let kernel_protocol_digest = kernel.kernel_protocol_digest();
    let normalizer_protocol_digest = kernel.normalizer_protocol_digest();
    let kernel_configuration_digest = generative_capability_kernel_configuration_digest_v1(kernel);
    if protocol.kernel_protocol_digest() != &kernel_protocol_digest {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::ProtocolKernelMismatch);
    }
    if protocol.normalizer_protocol_digest() != &normalizer_protocol_digest {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::ProtocolNormalizerMismatch);
    }
    if protocol.kernel_configuration_digest() != &kernel_configuration_digest {
        return Err(
            GenerativeStructuralOccurrenceGrammarFailureV1::ProtocolKernelConfigurationMismatch,
        );
    }
    if manifest.substitution_naturality_protocol_manifest_digest != *protocol.manifest_digest() {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::ProtocolManifestDigestMismatch);
    }
    if manifest.jg1_manifest_digest != *jg1.manifest_digest() {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::Jg1ManifestDigestMismatch);
    }
    if manifest.constructor_grammar_manifest_digest != *constructors.manifest_digest() {
        return Err(
            GenerativeStructuralOccurrenceGrammarFailureV1::ConstructorGrammarManifestDigestMismatch,
        );
    }
    if manifest.scope_grammar_digest != *constructors.scope_grammar_digest() {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::ScopeGrammarDigestMismatch);
    }
    if manifest.kernel_protocol_digest != kernel_protocol_digest {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::KernelProtocolMismatch);
    }
    if manifest.normalizer_protocol_digest != normalizer_protocol_digest {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::NormalizerProtocolMismatch);
    }
    if manifest.kernel_configuration_digest != kernel_configuration_digest {
        return Err(GenerativeStructuralOccurrenceGrammarFailureV1::KernelConfigurationMismatch);
    }

    let manifest_digest = Digest::of_canonical(
        GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_DIGEST_DOMAIN_V1,
        manifest,
    );
    Ok(VerifiedGenerativeStructuralOccurrenceGrammarV1 {
        schema_version: definition.schema_version,
        definition_digest,
        substitution_naturality_protocol_manifest_digest: manifest
            .substitution_naturality_protocol_manifest_digest
            .clone(),
        jg1_manifest_digest: manifest.jg1_manifest_digest.clone(),
        constructor_grammar_manifest_digest: manifest.constructor_grammar_manifest_digest.clone(),
        scope_grammar_digest: manifest.scope_grammar_digest.clone(),
        kernel_protocol_digest,
        normalizer_protocol_digest,
        kernel_configuration_digest,
        manifest_digest,
    })
}

/// Exact node kind derived by exhaustive matching over [`Term`].
pub fn generative_structural_occurrence_node_kind_v1(
    term: &Term,
) -> GenerativeStructuralOccurrenceNodeKindV1 {
    match term {
        Term::Sort { .. } => GenerativeStructuralOccurrenceNodeKindV1::Sort,
        Term::Var { .. } => GenerativeStructuralOccurrenceNodeKindV1::Var,
        Term::Global { .. } => GenerativeStructuralOccurrenceNodeKindV1::Global,
        Term::Pi { .. } => GenerativeStructuralOccurrenceNodeKindV1::Pi,
        Term::Sigma { .. } => GenerativeStructuralOccurrenceNodeKindV1::Sigma,
        Term::Lambda { .. } => GenerativeStructuralOccurrenceNodeKindV1::Lambda,
        Term::Apply { .. } => GenerativeStructuralOccurrenceNodeKindV1::Apply,
        Term::Pair { .. } => GenerativeStructuralOccurrenceNodeKindV1::Pair,
        Term::First { .. } => GenerativeStructuralOccurrenceNodeKindV1::First,
        Term::Second { .. } => GenerativeStructuralOccurrenceNodeKindV1::Second,
        Term::UnitType => GenerativeStructuralOccurrenceNodeKindV1::UnitType,
        Term::Unit => GenerativeStructuralOccurrenceNodeKindV1::Unit,
    }
}

fn structural_child_count_v1(term: &Term) -> usize {
    match term {
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => 0,
        Term::Pi { .. } | Term::Sigma { .. } | Term::Lambda { .. } | Term::Apply { .. } => 2,
        Term::Pair { .. } => 3,
        Term::First { .. } | Term::Second { .. } => 1,
    }
}

fn structural_child_v1(
    term: &Term,
    ordinal: usize,
) -> Option<(GenerativeStructuralOccurrencePathStepV1, &Term)> {
    match (term, ordinal) {
        (Term::Pi { parameter, .. }, 0) => Some((
            GenerativeStructuralOccurrencePathStepV1::PiParameter,
            parameter,
        )),
        (Term::Pi { body, .. }, 1) => {
            Some((GenerativeStructuralOccurrencePathStepV1::PiBody, body))
        }
        (Term::Sigma { parameter, .. }, 0) => Some((
            GenerativeStructuralOccurrencePathStepV1::SigmaParameter,
            parameter,
        )),
        (Term::Sigma { body, .. }, 1) => {
            Some((GenerativeStructuralOccurrencePathStepV1::SigmaBody, body))
        }
        (Term::Lambda { parameter_type, .. }, 0) => Some((
            GenerativeStructuralOccurrencePathStepV1::LambdaParameterType,
            parameter_type,
        )),
        (Term::Lambda { body, .. }, 1) => {
            Some((GenerativeStructuralOccurrencePathStepV1::LambdaBody, body))
        }
        (Term::Apply { function, .. }, 0) => Some((
            GenerativeStructuralOccurrencePathStepV1::ApplyFunction,
            function,
        )),
        (Term::Apply { argument, .. }, 1) => Some((
            GenerativeStructuralOccurrencePathStepV1::ApplyArgument,
            argument,
        )),
        (Term::Pair { sigma_type, .. }, 0) => Some((
            GenerativeStructuralOccurrencePathStepV1::PairSigmaType,
            sigma_type,
        )),
        (Term::Pair { first, .. }, 1) => {
            Some((GenerativeStructuralOccurrencePathStepV1::PairFirst, first))
        }
        (Term::Pair { second, .. }, 2) => {
            Some((GenerativeStructuralOccurrencePathStepV1::PairSecond, second))
        }
        (Term::First { pair }, 0) => {
            Some((GenerativeStructuralOccurrencePathStepV1::FirstPair, pair))
        }
        (Term::Second { pair }, 0) => {
            Some((GenerativeStructuralOccurrencePathStepV1::SecondPair, pair))
        }
        (
            Term::Sort { .. }
            | Term::Var { .. }
            | Term::Global { .. }
            | Term::UnitType
            | Term::Unit,
            _,
        )
        | (Term::Pi { .. } | Term::Sigma { .. } | Term::Lambda { .. } | Term::Apply { .. }, 2..)
        | (Term::Pair { .. }, 3..)
        | (Term::First { .. } | Term::Second { .. }, 1..) => None,
    }
}

/// One node in an explicitly unverified traversal proposal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GenerativeStructuralTraversalNodeV1 {
    root: GenerativeStructuralOccurrenceRootV1,
    parent_ordinal: Option<u32>,
    incoming_step: Option<GenerativeStructuralOccurrencePathStepV1>,
    structural_depth: u16,
    binder_depth: u16,
    node_kind: GenerativeStructuralOccurrenceNodeKindV1,
    structural_subject_digest: Digest,
}

impl GenerativeStructuralTraversalNodeV1 {
    pub const fn root(&self) -> GenerativeStructuralOccurrenceRootV1 {
        self.root
    }
    pub const fn parent_ordinal(&self) -> Option<u32> {
        self.parent_ordinal
    }
    pub const fn incoming_step(&self) -> Option<GenerativeStructuralOccurrencePathStepV1> {
        self.incoming_step
    }
    pub const fn structural_depth(&self) -> u16 {
        self.structural_depth
    }
    pub const fn binder_depth(&self) -> u16 {
        self.binder_depth
    }
    pub const fn node_kind(&self) -> GenerativeStructuralOccurrenceNodeKindV1 {
        self.node_kind
    }
    pub fn structural_subject_digest(&self) -> &Digest {
        &self.structural_subject_digest
    }
}

/// A deterministic but non-authoritative traversal over caller-supplied syntax.
///
/// Supplying a kernel-verified normalized declaration gives paths in normalized
/// syntax, but this value itself never proves that provenance.  JG2b2c must
/// derive its own normalized declarations from owned complete-history evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProposedGenerativeStructuralDeclarationTraversalV1 {
    declaration_input_digest: Digest,
    nodes: Vec<GenerativeStructuralTraversalNodeV1>,
}

impl ProposedGenerativeStructuralDeclarationTraversalV1 {
    pub fn declaration_input_digest(&self) -> &Digest {
        &self.declaration_input_digest
    }
    pub fn nodes(&self) -> &[GenerativeStructuralTraversalNodeV1] {
        &self.nodes
    }

    pub fn structural_path(
        &self,
        node_ordinal: usize,
    ) -> Result<Vec<GenerativeStructuralOccurrencePathStepV1>, GenerativeStructuralTraversalFailureV1>
    {
        let Some(node) = self.nodes.get(node_ordinal) else {
            return Err(GenerativeStructuralTraversalFailureV1::NodeOrdinalOutOfRange);
        };
        let mut reversed = Vec::new();
        reversed
            .try_reserve_exact(node.structural_depth as usize)
            .map_err(|_| GenerativeStructuralTraversalFailureV1::AllocationFailure)?;
        let mut cursor = node_ordinal;
        loop {
            let current = &self.nodes[cursor];
            match (current.parent_ordinal, current.incoming_step) {
                (Some(parent), Some(step)) => {
                    reversed.push(step);
                    cursor = parent as usize;
                }
                (None, None) => break,
                (Some(_), None) | (None, Some(_)) => {
                    unreachable!("private traversal arena is coherent")
                }
            }
        }
        reversed.reverse();
        Ok(reversed)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GenerativeStructuralTraversalFailureV1 {
    KernelConfigurationMismatch,
    OperationBudgetExhausted,
    DepthBudgetExhausted,
    BinderDepthOverflow,
    NodeOrdinalOverflow,
    AllocationFailure,
    NodeOrdinalOutOfRange,
}

impl std::fmt::Display for GenerativeStructuralTraversalFailureV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::KernelConfigurationMismatch => {
                "the traversal kernel configuration differs from the verified grammar"
            }
            Self::OperationBudgetExhausted => "structural traversal exhausted the operation budget",
            Self::DepthBudgetExhausted => "structural traversal exceeded the depth budget",
            Self::BinderDepthOverflow => "structural traversal binder depth overflowed",
            Self::NodeOrdinalOverflow => "structural traversal node ordinal overflowed",
            Self::AllocationFailure => "structural traversal allocation failed",
            Self::NodeOrdinalOutOfRange => "structural traversal node ordinal is out of range",
        })
    }
}

impl std::error::Error for GenerativeStructuralTraversalFailureV1 {}

struct TraversalFrame<'a> {
    root: GenerativeStructuralOccurrenceRootV1,
    parent_ordinal: Option<u32>,
    incoming_step: Option<GenerativeStructuralOccurrencePathStepV1>,
    structural_depth: u16,
    binder_depth: u16,
    term: &'a Term,
}

struct PendingTraversalNode<'a> {
    root: GenerativeStructuralOccurrenceRootV1,
    parent_ordinal: Option<u32>,
    incoming_step: Option<GenerativeStructuralOccurrencePathStepV1>,
    structural_depth: u16,
    binder_depth: u16,
    node_kind: GenerativeStructuralOccurrenceNodeKindV1,
    term: &'a Term,
    child_ordinals: [Option<u32>; 3],
}

fn bottom_up_structural_subject_digest_v1(
    term: &Term,
    child_digests: [Option<&Digest>; 3],
) -> Digest {
    let child = |ordinal: usize| {
        child_digests[ordinal]
            .expect("private postorder supplies every structurally required child digest")
            .as_str()
            .as_bytes()
    };
    match term {
        Term::Sort { level } => {
            let tag = [0x00];
            let level = level.to_le_bytes();
            Digest::of_domain_chunks(
                GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1,
                &[&tag, &level],
            )
        }
        Term::Var { index } => {
            let tag = [0x01];
            let index = index.to_le_bytes();
            Digest::of_domain_chunks(
                GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1,
                &[&tag, &index],
            )
        }
        Term::Global { id } => {
            let tag = [0x02];
            Digest::of_domain_chunks(
                GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1,
                &[&tag, id.0.as_str().as_bytes()],
            )
        }
        Term::Pi { .. } => {
            let tag = [0x03];
            Digest::of_domain_chunks(
                GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1,
                &[&tag, child(0), child(1)],
            )
        }
        Term::Sigma { .. } => {
            let tag = [0x04];
            Digest::of_domain_chunks(
                GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1,
                &[&tag, child(0), child(1)],
            )
        }
        Term::Lambda { .. } => {
            let tag = [0x05];
            Digest::of_domain_chunks(
                GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1,
                &[&tag, child(0), child(1)],
            )
        }
        Term::Apply { .. } => {
            let tag = [0x06];
            Digest::of_domain_chunks(
                GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1,
                &[&tag, child(0), child(1)],
            )
        }
        Term::Pair { .. } => {
            let tag = [0x07];
            Digest::of_domain_chunks(
                GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1,
                &[&tag, child(0), child(1), child(2)],
            )
        }
        Term::First { .. } => {
            let tag = [0x08];
            Digest::of_domain_chunks(
                GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1,
                &[&tag, child(0)],
            )
        }
        Term::Second { .. } => {
            let tag = [0x09];
            Digest::of_domain_chunks(
                GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1,
                &[&tag, child(0)],
            )
        }
        Term::UnitType => {
            let tag = [0x0a];
            Digest::of_domain_chunks(GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1, &[&tag])
        }
        Term::Unit => {
            let tag = [0x0b];
            Digest::of_domain_chunks(GENERATIVE_STRUCTURAL_SUBJECT_DIGEST_DOMAIN_V1, &[&tag])
        }
    }
}

fn structural_declaration_input_digest_v1(
    declaration: &Declaration,
    type_subject_digest: &Digest,
    body_subject_digest: Option<&Digest>,
) -> Digest {
    let body_tag = [u8::from(body_subject_digest.is_some())];
    match body_subject_digest {
        Some(body_subject_digest) => Digest::of_domain_chunks(
            GENERATIVE_STRUCTURAL_DECLARATION_INPUT_DIGEST_DOMAIN_V1,
            &[
                declaration.id.0.as_str().as_bytes(),
                type_subject_digest.as_str().as_bytes(),
                &body_tag,
                body_subject_digest.as_str().as_bytes(),
            ],
        ),
        None => Digest::of_domain_chunks(
            GENERATIVE_STRUCTURAL_DECLARATION_INPUT_DIGEST_DOMAIN_V1,
            &[
                declaration.id.0.as_str().as_bytes(),
                type_subject_digest.as_str().as_bytes(),
                &body_tag,
            ],
        ),
    }
}

/// Execute the frozen preorder over one caller-supplied declaration.
///
/// The result is named `Proposed` because neither the declaration nor its
/// provenance is verified here.  Failure returns no partial proposal.
pub fn proposed_generative_structural_declaration_traversal_v1(
    grammar: &VerifiedGenerativeStructuralOccurrenceGrammarV1,
    kernel: &Kernel,
    declaration: &Declaration,
) -> Result<
    ProposedGenerativeStructuralDeclarationTraversalV1,
    GenerativeStructuralTraversalFailureV1,
> {
    if grammar.kernel_configuration_digest()
        != &generative_capability_kernel_configuration_digest_v1(kernel)
    {
        return Err(GenerativeStructuralTraversalFailureV1::KernelConfigurationMismatch);
    }

    let limits = kernel.limits();
    let mut pending_nodes: Vec<PendingTraversalNode<'_>> = Vec::new();
    let mut stack = Vec::new();
    stack
        .try_reserve_exact(usize::from(declaration.body.is_some()) + 1)
        .map_err(|_| GenerativeStructuralTraversalFailureV1::AllocationFailure)?;
    if let Some(body) = declaration.body.as_ref() {
        stack.push(TraversalFrame {
            root: GenerativeStructuralOccurrenceRootV1::DeclarationBody,
            parent_ordinal: None,
            incoming_step: None,
            structural_depth: 0,
            binder_depth: 0,
            term: body,
        });
    }
    stack.push(TraversalFrame {
        root: GenerativeStructuralOccurrenceRootV1::DeclarationType,
        parent_ordinal: None,
        incoming_step: None,
        structural_depth: 0,
        binder_depth: 0,
        term: &declaration.ty,
    });

    while let Some(frame) = stack.pop() {
        if frame.structural_depth > limits.max_depth {
            return Err(GenerativeStructuralTraversalFailureV1::DepthBudgetExhausted);
        }
        if pending_nodes.len() >= limits.max_operations as usize {
            return Err(GenerativeStructuralTraversalFailureV1::OperationBudgetExhausted);
        }
        let node_ordinal = u32::try_from(pending_nodes.len())
            .map_err(|_| GenerativeStructuralTraversalFailureV1::NodeOrdinalOverflow)?;
        if let (Some(parent_ordinal), Some(incoming_step)) =
            (frame.parent_ordinal, frame.incoming_step)
        {
            pending_nodes[parent_ordinal as usize].child_ordinals
                [incoming_step.child_ordinal() as usize] = Some(node_ordinal);
        }
        pending_nodes
            .try_reserve(1)
            .map_err(|_| GenerativeStructuralTraversalFailureV1::AllocationFailure)?;
        pending_nodes.push(PendingTraversalNode {
            root: frame.root,
            parent_ordinal: frame.parent_ordinal,
            incoming_step: frame.incoming_step,
            structural_depth: frame.structural_depth,
            binder_depth: frame.binder_depth,
            node_kind: generative_structural_occurrence_node_kind_v1(frame.term),
            term: frame.term,
            child_ordinals: [None; 3],
        });

        let child_count = structural_child_count_v1(frame.term);
        stack
            .try_reserve(child_count)
            .map_err(|_| GenerativeStructuralTraversalFailureV1::AllocationFailure)?;
        for child_ordinal in (0..child_count).rev() {
            let (step, child) = structural_child_v1(frame.term, child_ordinal)
                .expect("closed child count agrees with exhaustive child projection");
            let structural_depth = frame
                .structural_depth
                .checked_add(1)
                .ok_or(GenerativeStructuralTraversalFailureV1::DepthBudgetExhausted)?;
            let binder_depth = if step.enters_binder() {
                frame
                    .binder_depth
                    .checked_add(1)
                    .ok_or(GenerativeStructuralTraversalFailureV1::BinderDepthOverflow)?
            } else {
                frame.binder_depth
            };
            stack.push(TraversalFrame {
                root: frame.root,
                parent_ordinal: Some(node_ordinal),
                incoming_step: Some(step),
                structural_depth,
                binder_depth,
                term: child,
            });
        }
    }

    let mut subject_digests = Vec::new();
    subject_digests
        .try_reserve_exact(pending_nodes.len())
        .map_err(|_| GenerativeStructuralTraversalFailureV1::AllocationFailure)?;
    subject_digests.resize_with(pending_nodes.len(), || None);
    for node_ordinal in (0..pending_nodes.len()).rev() {
        let digest = {
            let pending = &pending_nodes[node_ordinal];
            let child_digests = pending.child_ordinals.map(|child_ordinal| {
                child_ordinal.map(|child_ordinal| {
                    subject_digests[child_ordinal as usize]
                        .as_ref()
                        .expect("reverse preorder computes children before their parent")
                })
            });
            bottom_up_structural_subject_digest_v1(pending.term, child_digests)
        };
        subject_digests[node_ordinal] = Some(digest);
    }

    let mut nodes = Vec::new();
    nodes
        .try_reserve_exact(pending_nodes.len())
        .map_err(|_| GenerativeStructuralTraversalFailureV1::AllocationFailure)?;
    for (pending, structural_subject_digest) in pending_nodes.into_iter().zip(subject_digests) {
        nodes.push(GenerativeStructuralTraversalNodeV1 {
            root: pending.root,
            parent_ordinal: pending.parent_ordinal,
            incoming_step: pending.incoming_step,
            structural_depth: pending.structural_depth,
            binder_depth: pending.binder_depth,
            node_kind: pending.node_kind,
            structural_subject_digest: structural_subject_digest
                .expect("every pending traversal node receives one bottom-up digest"),
        });
    }

    let type_subject_digest = nodes[0].structural_subject_digest();
    let body_subject_digest = nodes
        .iter()
        .find(|node| {
            node.parent_ordinal().is_none()
                && node.root() == GenerativeStructuralOccurrenceRootV1::DeclarationBody
        })
        .map(GenerativeStructuralTraversalNodeV1::structural_subject_digest);
    let declaration_input_digest = structural_declaration_input_digest_v1(
        declaration,
        type_subject_digest,
        body_subject_digest,
    );

    Ok(ProposedGenerativeStructuralDeclarationTraversalV1 {
        declaration_input_digest,
        nodes,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        proposed_generative_capability_constructor_grammar_v1,
        proposed_generative_substitution_naturality_protocol_v1,
        proposed_pre_exposure_generative_capability_grammar_v1,
        verify_generative_capability_constructor_grammar_v1,
        verify_generative_substitution_naturality_protocol_v1,
        verify_pre_exposure_generative_capability_grammar_v1,
    };
    use pen_kernel::{GlobalId, KernelLimits, UncheckedSignature};
    use std::collections::BTreeSet;

    fn verified_inputs() -> (
        VerifiedPreExposureGenerativeCapabilityGrammarV1,
        VerifiedGenerativeCapabilityConstructorGrammarV1,
        VerifiedGenerativeSubstitutionNaturalityProtocolV1,
        Kernel,
    ) {
        let jg1 = verify_pre_exposure_generative_capability_grammar_v1(
            &proposed_pre_exposure_generative_capability_grammar_v1(),
        )
        .expect("JG1");
        let constructors = verify_generative_capability_constructor_grammar_v1(
            &jg1,
            &proposed_generative_capability_constructor_grammar_v1(&jg1),
        )
        .expect("JG2a");
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let protocol = verify_generative_substitution_naturality_protocol_v1(
            &jg1,
            &constructors,
            &kernel,
            &proposed_generative_substitution_naturality_protocol_v1(&jg1, &constructors, &kernel),
        )
        .expect("JG2b2a");
        (jg1, constructors, protocol, kernel)
    }

    fn verified_grammar() -> (VerifiedGenerativeStructuralOccurrenceGrammarV1, Kernel) {
        let (jg1, constructors, protocol, kernel) = verified_inputs();
        let grammar = verify_generative_structural_occurrence_grammar_v1(
            &protocol,
            &jg1,
            &constructors,
            &kernel,
            &proposed_generative_structural_occurrence_grammar_v1(
                &protocol,
                &jg1,
                &constructors,
                &kernel,
            ),
        )
        .expect("JG2b2b0");
        (grammar, kernel)
    }

    fn declaration(ty: Term, body: Option<Term>) -> Declaration {
        Declaration {
            id: GlobalId(Digest::of_bytes(b"jg2b2b0-declaration")),
            ty,
            body,
        }
    }

    #[test]
    fn exact_definition_transcript_and_digest_are_pinned() {
        let definition = proposed_generative_structural_occurrence_definition_v1();
        let mut encoder = CanonicalEncoder::new();
        definition.encode_canonical(&mut encoder);
        assert_eq!(encoder.as_bytes().len(), 123);
        assert_eq!(
            encoder.as_bytes(),
            CANONICAL_GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_BYTES_V1
        );
        assert_eq!(
            Digest::of_domain_bytes(
                GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_DIGEST_DOMAIN_V1,
                encoder.as_bytes(),
            )
            .as_str(),
            CANONICAL_GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_DIGEST_V1
        );
    }

    #[test]
    fn synthetic_full_grammar_manifest_codec_fixture_is_frozen() {
        let fixture = GenerativeStructuralOccurrenceGrammarManifestV1 {
            definition: proposed_generative_structural_occurrence_definition_v1(),
            substitution_naturality_protocol_manifest_digest: Digest::of_bytes(
                b"jg2b2a-codec-fixture",
            ),
            jg1_manifest_digest: Digest::of_bytes(b"jg1-codec-fixture"),
            constructor_grammar_manifest_digest: Digest::of_bytes(b"constructor-codec-fixture"),
            scope_grammar_digest: Digest::of_bytes(b"scope-codec-fixture"),
            kernel_protocol_digest: Digest::of_bytes(b"kernel-codec-fixture"),
            normalizer_protocol_digest: Digest::of_bytes(b"normalizer-codec-fixture"),
            kernel_configuration_digest: Digest::of_bytes(b"configuration-codec-fixture"),
        };

        let mut encoder = CanonicalEncoder::new();
        fixture.encode_canonical(&mut encoder);
        assert_eq!(encoder.as_bytes().len(), 677);
        assert_eq!(
            Digest::of_domain_bytes(
                GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_DIGEST_DOMAIN_V1,
                encoder.as_bytes(),
            )
            .as_str(),
            "blake3:5e25ebf83319b1d687a95e3a8720892f22ec669230ea1dd414eae891c3f5c798"
        );
    }

    #[test]
    fn exact_closed_vocabularies_and_parent_fields_are_frozen() {
        assert_eq!(GENERATIVE_STRUCTURAL_OCCURRENCE_ROOTS_V1.len(), 2);
        assert_eq!(GENERATIVE_STRUCTURAL_OCCURRENCE_NODE_KINDS_V1.len(), 12);
        assert_eq!(GENERATIVE_STRUCTURAL_OCCURRENCE_PATH_STEPS_V1.len(), 13);
        assert_eq!(GENERATIVE_STRUCTURAL_OCCURRENCE_BINDER_STEPS_V1.len(), 3);
        assert_eq!(
            GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_DOMAINS_V1.len(),
            2
        );
        assert_eq!(
            GENERATIVE_STRUCTURAL_OCCURRENCE_IDENTITY_FIELDS_V1.len(),
            14
        );
        assert_eq!(GENERATIVE_STRUCTURAL_OCCURRENCE_GRAMMAR_RULES_V1.len(), 18);
        assert_eq!(
            GENERATIVE_STRUCTURAL_OCCURRENCE_PATH_STEPS_V1
                .iter()
                .copied()
                .filter(|step| step.enters_binder())
                .collect::<Vec<_>>(),
            GENERATIVE_STRUCTURAL_OCCURRENCE_BINDER_STEPS_V1
        );
        assert!(
            GENERATIVE_STRUCTURAL_OCCURRENCE_PATH_STEPS_V1
                .iter()
                .all(|step| {
                    step.child_ordinal()
                        < match step.parent_kind() {
                            GenerativeStructuralOccurrenceNodeKindV1::Pair => 3,
                            GenerativeStructuralOccurrenceNodeKindV1::Pi
                            | GenerativeStructuralOccurrenceNodeKindV1::Sigma
                            | GenerativeStructuralOccurrenceNodeKindV1::Lambda
                            | GenerativeStructuralOccurrenceNodeKindV1::Apply => 2,
                            GenerativeStructuralOccurrenceNodeKindV1::First
                            | GenerativeStructuralOccurrenceNodeKindV1::Second => 1,
                            GenerativeStructuralOccurrenceNodeKindV1::Sort
                            | GenerativeStructuralOccurrenceNodeKindV1::Var
                            | GenerativeStructuralOccurrenceNodeKindV1::Global
                            | GenerativeStructuralOccurrenceNodeKindV1::UnitType
                            | GenerativeStructuralOccurrenceNodeKindV1::Unit => 0,
                        }
                })
        );
    }

    #[test]
    fn grammar_is_remintable_and_binds_every_parent_authority() {
        let (jg1, constructors, protocol, kernel) = verified_inputs();
        let proposal = proposed_generative_structural_occurrence_grammar_v1(
            &protocol,
            &jg1,
            &constructors,
            &kernel,
        );
        let first = verify_generative_structural_occurrence_grammar_v1(
            &protocol,
            &jg1,
            &constructors,
            &kernel,
            &proposal,
        )
        .expect("first");
        let second = verify_generative_structural_occurrence_grammar_v1(
            &protocol,
            &jg1,
            &constructors,
            &kernel,
            &proposal,
        )
        .expect("second");
        assert_eq!(first, second);
        assert_eq!(
            first.definition_digest().as_str(),
            CANONICAL_GENERATIVE_STRUCTURAL_OCCURRENCE_DEFINITION_DIGEST_V1
        );
        assert_eq!(
            first.substitution_naturality_protocol_manifest_digest(),
            protocol.manifest_digest()
        );
        assert_eq!(first.jg1_manifest_digest(), jg1.manifest_digest());
        assert_eq!(
            first.constructor_grammar_manifest_digest(),
            constructors.manifest_digest()
        );
        assert_eq!(
            first.scope_grammar_digest(),
            constructors.scope_grammar_digest()
        );
        assert_eq!(
            first.kernel_protocol_digest(),
            &kernel.kernel_protocol_digest()
        );
        assert_eq!(
            first.normalizer_protocol_digest(),
            &kernel.normalizer_protocol_digest()
        );
    }

    #[test]
    fn every_static_vocabulary_mutation_is_rejected() {
        let (jg1, constructors, protocol, kernel) = verified_inputs();
        let proposal = proposed_generative_structural_occurrence_grammar_v1(
            &protocol,
            &jg1,
            &constructors,
            &kernel,
        );
        let mutations: Vec<(
            GenerativeStructuralOccurrenceGrammarManifestV1,
            GenerativeStructuralOccurrenceGrammarFailureV1,
        )> = vec![
            (
                {
                    let mut m = proposal.clone();
                    m.definition.schema_version += 1;
                    m
                },
                GenerativeStructuralOccurrenceGrammarFailureV1::SchemaVersionMismatch,
            ),
            (
                {
                    let mut m = proposal.clone();
                    m.definition.roots.reverse();
                    m
                },
                GenerativeStructuralOccurrenceGrammarFailureV1::RootVocabularyMismatch,
            ),
            (
                {
                    let mut m = proposal.clone();
                    m.definition.node_kinds.pop();
                    m
                },
                GenerativeStructuralOccurrenceGrammarFailureV1::NodeKindVocabularyMismatch,
            ),
            (
                {
                    let mut m = proposal.clone();
                    m.definition.path_steps.rotate_left(1);
                    m
                },
                GenerativeStructuralOccurrenceGrammarFailureV1::PathStepVocabularyMismatch,
            ),
            (
                {
                    let mut m = proposal.clone();
                    m.definition.binder_entering_steps.reverse();
                    m
                },
                GenerativeStructuralOccurrenceGrammarFailureV1::BinderStepVocabularyMismatch,
            ),
            (
                {
                    let mut m = proposal.clone();
                    m.definition.identity_domains.reverse();
                    m
                },
                GenerativeStructuralOccurrenceGrammarFailureV1::IdentityDomainVocabularyMismatch,
            ),
            (
                {
                    let mut m = proposal.clone();
                    m.definition.identity_fields.pop();
                    m
                },
                GenerativeStructuralOccurrenceGrammarFailureV1::IdentityFieldVocabularyMismatch,
            ),
            (
                {
                    let mut m = proposal.clone();
                    m.definition.rules.rotate_right(1);
                    m
                },
                GenerativeStructuralOccurrenceGrammarFailureV1::RuleVocabularyMismatch,
            ),
        ];
        for (mutation, expected) in mutations {
            assert_eq!(
                verify_generative_structural_occurrence_grammar_v1(
                    &protocol,
                    &jg1,
                    &constructors,
                    &kernel,
                    &mutation
                ),
                Err(expected),
            );
        }
    }

    #[test]
    fn every_manifest_binding_mutation_is_rejected() {
        let (jg1, constructors, protocol, kernel) = verified_inputs();
        let proposal = proposed_generative_structural_occurrence_grammar_v1(
            &protocol,
            &jg1,
            &constructors,
            &kernel,
        );
        let changed = Digest::of_bytes(b"changed-binding");
        let mutations: Vec<(GenerativeStructuralOccurrenceGrammarManifestV1, GenerativeStructuralOccurrenceGrammarFailureV1)> = vec![
            ({ let mut m = proposal.clone(); m.substitution_naturality_protocol_manifest_digest = changed.clone(); m }, GenerativeStructuralOccurrenceGrammarFailureV1::ProtocolManifestDigestMismatch),
            ({ let mut m = proposal.clone(); m.jg1_manifest_digest = changed.clone(); m }, GenerativeStructuralOccurrenceGrammarFailureV1::Jg1ManifestDigestMismatch),
            ({ let mut m = proposal.clone(); m.constructor_grammar_manifest_digest = changed.clone(); m }, GenerativeStructuralOccurrenceGrammarFailureV1::ConstructorGrammarManifestDigestMismatch),
            ({ let mut m = proposal.clone(); m.scope_grammar_digest = changed.clone(); m }, GenerativeStructuralOccurrenceGrammarFailureV1::ScopeGrammarDigestMismatch),
            ({ let mut m = proposal.clone(); m.kernel_protocol_digest = changed.clone(); m }, GenerativeStructuralOccurrenceGrammarFailureV1::KernelProtocolMismatch),
            ({ let mut m = proposal.clone(); m.normalizer_protocol_digest = changed.clone(); m }, GenerativeStructuralOccurrenceGrammarFailureV1::NormalizerProtocolMismatch),
            ({ let mut m = proposal.clone(); m.kernel_configuration_digest = changed.clone(); m }, GenerativeStructuralOccurrenceGrammarFailureV1::KernelConfigurationMismatch),
        ];
        for (mutation, expected) in mutations {
            assert_eq!(
                verify_generative_structural_occurrence_grammar_v1(
                    &protocol,
                    &jg1,
                    &constructors,
                    &kernel,
                    &mutation
                ),
                Err(expected),
            );
        }
    }

    fn all_forms_term() -> Term {
        Term::Pair {
            sigma_type: Box::new(Term::Sigma {
                parameter: Box::new(Term::Sort { level: 0 }),
                body: Box::new(Term::Pi {
                    parameter: Box::new(Term::UnitType),
                    body: Box::new(Term::Unit),
                }),
            }),
            first: Box::new(Term::Apply {
                function: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::Global {
                        id: GlobalId(Digest::of_bytes(b"fixture-global")),
                    }),
                    body: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::First {
                    pair: Box::new(Term::Unit),
                }),
            }),
            second: Box::new(Term::Second {
                pair: Box::new(Term::UnitType),
            }),
        }
    }

    #[test]
    fn exhaustive_term_fixture_has_exact_preorder_and_all_thirteen_edges() {
        let (grammar, kernel) = verified_grammar();
        let input = declaration(all_forms_term(), None);
        let traversal =
            proposed_generative_structural_declaration_traversal_v1(&grammar, &kernel, &input)
                .expect("traversal");
        assert_eq!(traversal.nodes().len(), 14);
        assert_eq!(
            traversal.nodes()[0].structural_subject_digest().as_str(),
            "blake3:b7a9005cfebe8f6bc83599b9a0b60ca9c581a7f2b89df21f1168a335561c95b1"
        );
        assert_eq!(
            traversal.declaration_input_digest().as_str(),
            "blake3:822c9464aacd18c2e6e6629a6068f67692707d88921dcedd80cd959fa5823892"
        );
        let kinds = traversal
            .nodes()
            .iter()
            .map(|node| node.node_kind())
            .collect::<BTreeSet<_>>();
        assert_eq!(
            kinds,
            GENERATIVE_STRUCTURAL_OCCURRENCE_NODE_KINDS_V1
                .into_iter()
                .collect()
        );
        let steps = traversal
            .nodes()
            .iter()
            .filter_map(|node| node.incoming_step())
            .collect::<Vec<_>>();
        assert_eq!(steps.len(), 13);
        assert_eq!(
            steps.iter().copied().collect::<BTreeSet<_>>(),
            GENERATIVE_STRUCTURAL_OCCURRENCE_PATH_STEPS_V1
                .into_iter()
                .collect()
        );
        let expected_paths = vec![
            vec![],
            vec![GenerativeStructuralOccurrencePathStepV1::PairSigmaType],
            vec![
                GenerativeStructuralOccurrencePathStepV1::PairSigmaType,
                GenerativeStructuralOccurrencePathStepV1::SigmaParameter,
            ],
            vec![
                GenerativeStructuralOccurrencePathStepV1::PairSigmaType,
                GenerativeStructuralOccurrencePathStepV1::SigmaBody,
            ],
            vec![
                GenerativeStructuralOccurrencePathStepV1::PairSigmaType,
                GenerativeStructuralOccurrencePathStepV1::SigmaBody,
                GenerativeStructuralOccurrencePathStepV1::PiParameter,
            ],
            vec![
                GenerativeStructuralOccurrencePathStepV1::PairSigmaType,
                GenerativeStructuralOccurrencePathStepV1::SigmaBody,
                GenerativeStructuralOccurrencePathStepV1::PiBody,
            ],
            vec![GenerativeStructuralOccurrencePathStepV1::PairFirst],
            vec![
                GenerativeStructuralOccurrencePathStepV1::PairFirst,
                GenerativeStructuralOccurrencePathStepV1::ApplyFunction,
            ],
            vec![
                GenerativeStructuralOccurrencePathStepV1::PairFirst,
                GenerativeStructuralOccurrencePathStepV1::ApplyFunction,
                GenerativeStructuralOccurrencePathStepV1::LambdaParameterType,
            ],
            vec![
                GenerativeStructuralOccurrencePathStepV1::PairFirst,
                GenerativeStructuralOccurrencePathStepV1::ApplyFunction,
                GenerativeStructuralOccurrencePathStepV1::LambdaBody,
            ],
            vec![
                GenerativeStructuralOccurrencePathStepV1::PairFirst,
                GenerativeStructuralOccurrencePathStepV1::ApplyArgument,
            ],
            vec![
                GenerativeStructuralOccurrencePathStepV1::PairFirst,
                GenerativeStructuralOccurrencePathStepV1::ApplyArgument,
                GenerativeStructuralOccurrencePathStepV1::FirstPair,
            ],
            vec![GenerativeStructuralOccurrencePathStepV1::PairSecond],
            vec![
                GenerativeStructuralOccurrencePathStepV1::PairSecond,
                GenerativeStructuralOccurrencePathStepV1::SecondPair,
            ],
        ];
        let actual_paths = (0..traversal.nodes().len())
            .map(|ordinal| traversal.structural_path(ordinal).expect("path"))
            .collect::<Vec<_>>();
        assert_eq!(actual_paths, expected_paths);
    }

    #[test]
    fn binder_depth_is_derived_only_on_dependent_body_edges() {
        let (grammar, kernel) = verified_grammar();
        let term = Term::Pi {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::Sigma {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::UnitType),
                    body: Box::new(Term::Var { index: 0 }),
                }),
            }),
        };
        let traversal = proposed_generative_structural_declaration_traversal_v1(
            &grammar,
            &kernel,
            &declaration(term, None),
        )
        .expect("traversal");
        assert_eq!(
            traversal
                .nodes()
                .iter()
                .map(|node| node.binder_depth())
                .collect::<Vec<_>>(),
            vec![0, 0, 1, 1, 2, 2, 3],
        );
    }

    #[test]
    fn equal_terms_at_distinct_paths_remain_distinct() {
        let (grammar, kernel) = verified_grammar();
        let traversal = proposed_generative_structural_declaration_traversal_v1(
            &grammar,
            &kernel,
            &declaration(
                Term::Apply {
                    function: Box::new(Term::Unit),
                    argument: Box::new(Term::Unit),
                },
                None,
            ),
        )
        .expect("traversal");
        assert_eq!(
            traversal.nodes()[1].structural_subject_digest(),
            traversal.nodes()[2].structural_subject_digest()
        );
        assert_ne!(
            traversal.structural_path(1).expect("left path"),
            traversal.structural_path(2).expect("right path")
        );
    }

    #[test]
    fn body_root_exists_if_and_only_if_body_is_present() {
        let (grammar, kernel) = verified_grammar();
        let bodyless = proposed_generative_structural_declaration_traversal_v1(
            &grammar,
            &kernel,
            &declaration(Term::UnitType, None),
        )
        .expect("bodyless");
        let bodied = proposed_generative_structural_declaration_traversal_v1(
            &grammar,
            &kernel,
            &declaration(Term::UnitType, Some(Term::Unit)),
        )
        .expect("bodied");
        assert_eq!(
            bodyless
                .nodes()
                .iter()
                .filter(|node| node.parent_ordinal().is_none())
                .map(|node| node.root())
                .collect::<Vec<_>>(),
            vec![GenerativeStructuralOccurrenceRootV1::DeclarationType]
        );
        assert_eq!(
            bodied
                .nodes()
                .iter()
                .filter(|node| node.parent_ordinal().is_none())
                .map(|node| node.root())
                .collect::<Vec<_>>(),
            GENERATIVE_STRUCTURAL_OCCURRENCE_ROOTS_V1
        );
    }

    #[test]
    fn normalized_paths_and_exact_input_evidence_are_separate() {
        let (grammar, kernel) = verified_grammar();
        let raw = declaration(
            Term::Apply {
                function: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::Sort { level: 0 }),
                    body: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::UnitType),
            },
            Some(Term::Unit),
        );
        let verified = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![raw.clone()],
            })
            .expect("valid beta-redex declaration");
        let raw_traversal =
            proposed_generative_structural_declaration_traversal_v1(&grammar, &kernel, &raw)
                .expect("raw traversal");
        let normalized_traversal = proposed_generative_structural_declaration_traversal_v1(
            &grammar,
            &kernel,
            &verified.declarations()[0],
        )
        .expect("normalized traversal");
        assert_eq!(
            raw_traversal.nodes()[0].node_kind(),
            GenerativeStructuralOccurrenceNodeKindV1::Apply
        );
        assert_eq!(
            normalized_traversal.nodes()[0].node_kind(),
            GenerativeStructuralOccurrenceNodeKindV1::UnitType
        );
        assert_ne!(
            raw_traversal.declaration_input_digest(),
            normalized_traversal.declaration_input_digest()
        );
    }

    #[test]
    fn resource_failure_returns_no_partial_proposal() {
        let (jg1, constructors, _, _) = verified_inputs();
        let limits = KernelLimits {
            max_operations: 1,
            max_depth: 1,
            normalization_fuel: 1,
        };
        let kernel = Kernel::new(limits).expect("small kernel");
        let protocol = verify_generative_substitution_naturality_protocol_v1(
            &jg1,
            &constructors,
            &kernel,
            &proposed_generative_substitution_naturality_protocol_v1(&jg1, &constructors, &kernel),
        )
        .expect("small protocol");
        let grammar = verify_generative_structural_occurrence_grammar_v1(
            &protocol,
            &jg1,
            &constructors,
            &kernel,
            &proposed_generative_structural_occurrence_grammar_v1(
                &protocol,
                &jg1,
                &constructors,
                &kernel,
            ),
        )
        .expect("small grammar");
        assert_eq!(
            proposed_generative_structural_declaration_traversal_v1(
                &grammar,
                &kernel,
                &declaration(
                    Term::First {
                        pair: Box::new(Term::Unit)
                    },
                    None
                ),
            ),
            Err(GenerativeStructuralTraversalFailureV1::OperationBudgetExhausted),
        );
        assert_eq!(
            ProposedGenerativeStructuralDeclarationTraversalV1 {
                declaration_input_digest: Digest::of_bytes(b"empty"),
                nodes: Vec::new(),
            }
            .structural_path(0),
            Err(GenerativeStructuralTraversalFailureV1::NodeOrdinalOutOfRange),
        );
    }

    #[test]
    fn traversal_rejects_configuration_drift_and_depth_exhaustion() {
        let (default_grammar, _) = verified_grammar();
        let shallow_kernel = Kernel::new(KernelLimits {
            max_operations: 10,
            max_depth: 1,
            normalization_fuel: 10,
        })
        .expect("shallow kernel");
        assert_eq!(
            proposed_generative_structural_declaration_traversal_v1(
                &default_grammar,
                &shallow_kernel,
                &declaration(Term::UnitType, None),
            ),
            Err(GenerativeStructuralTraversalFailureV1::KernelConfigurationMismatch),
        );

        let (jg1, constructors, _, _) = verified_inputs();
        let protocol = verify_generative_substitution_naturality_protocol_v1(
            &jg1,
            &constructors,
            &shallow_kernel,
            &proposed_generative_substitution_naturality_protocol_v1(
                &jg1,
                &constructors,
                &shallow_kernel,
            ),
        )
        .expect("shallow protocol");
        let shallow_grammar = verify_generative_structural_occurrence_grammar_v1(
            &protocol,
            &jg1,
            &constructors,
            &shallow_kernel,
            &proposed_generative_structural_occurrence_grammar_v1(
                &protocol,
                &jg1,
                &constructors,
                &shallow_kernel,
            ),
        )
        .expect("shallow grammar");
        let mut deep_term = Term::Unit;
        for _ in 0..10_000 {
            deep_term = Term::First {
                pair: Box::new(deep_term),
            };
        }
        let deep_declaration = declaration(deep_term, None);
        let deep_result = proposed_generative_structural_declaration_traversal_v1(
            &shallow_grammar,
            &shallow_kernel,
            &deep_declaration,
        );
        // Avoid recursively dropping the intentionally adversarial caller tree
        // after proving that the traversal itself never recursively entered it.
        std::mem::forget(deep_declaration);
        assert_eq!(
            deep_result,
            Err(GenerativeStructuralTraversalFailureV1::DepthBudgetExhausted),
        );
    }
}
