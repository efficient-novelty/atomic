use pen_kernel::{DependentContext, Digest, OpenJudgment};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Total outcome type. Unsupported and undecidable inputs never become proofs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuditDecisionV1<T> {
    Proven(T),
    Derived(T),
    OutsideFragment(AuditFailureV1),
    ResourceExhausted(AuditFailureV1),
    Unknown(AuditFailureV1),
}

impl<T> AuditDecisionV1<T> {
    pub fn proven(self) -> Result<T, AuditFailureV1> {
        match self {
            Self::Proven(value) | Self::Derived(value) => Ok(value),
            Self::OutsideFragment(error)
            | Self::ResourceExhausted(error)
            | Self::Unknown(error) => Err(error),
        }
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum AuditFailureV1 {
    #[error("kernel rejected input: {0}")]
    KernelRejected(String),
    #[error("kernel resource bound was exhausted: {0}")]
    KernelResourceExhausted(String),
    #[error("the input is not an exact one-entry context extension")]
    NotSingleExtension,
    #[error("the supplied context is not the verified projection source or target")]
    ContextMismatch,
    #[error("the operation is identity or presentation-trivial")]
    TrivialAction,
    #[error("the public judgment is not both sealed and public")]
    NotSealedPublic,
    #[error("term reindexing exceeds the bounded syntax fragment")]
    UnsupportedReindexingSyntax,
    #[error("finite carrier is empty, malformed, or outside its bound")]
    InvalidFiniteCarrier,
    #[error("finite projection references a missing base atom")]
    InvalidFiniteProjection,
    #[error("finite interpretation is not bound to the verified context action")]
    UnboundFiniteInterpretation,
    #[error("certificate has the wrong profile, model, action, or polarity binding")]
    CertificateBindingMismatch,
    #[error("certificate operation table is incomplete or incorrect")]
    OperationTableMismatch,
    #[error("certificate unit is incomplete or invalid")]
    UnitMismatch,
    #[error("certificate counit is incomplete or invalid")]
    CounitMismatch,
    #[error("certificate hom equivalence is incomplete or invalid")]
    HomEquivalenceMismatch,
    #[error("certificate triangle identity is incomplete or invalid")]
    TriangleIdentityMismatch,
    #[error("only one universal polarity was supplied")]
    TwoSidedCompletionRequired,
    #[error("an interface available only to the verifier is not public payment")]
    AmbientOnlyInterface,
    #[error("this prototype cannot authorize production payment")]
    ProductionAuthorityUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SealedPublicJudgmentV1 {
    pub judgment: OpenJudgment,
    pub birth_digest: Digest,
    pub type_support_digest: Digest,
    pub sealed: bool,
    pub public: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProjectionReindexingInputV1 {
    pub base: DependentContext,
    pub extension: DependentContext,
    pub public_judgment: SealedPublicJudgmentV1,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextSideV1 {
    Base,
    Extension,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolarityV1 {
    LeftOfReindexing,
    RightOfReindexing,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AnonymousOperationIdV1(pub Digest);

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubsetObjectV1 {
    pub side: ContextSideV1,
    pub mask: u16,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HomMorphismV1 {
    pub side: ContextSideV1,
    pub source: u16,
    pub target: u16,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FiniteProjectionInputV1 {
    pub context_projection_digest: Digest,
    pub reindexing_action_digest: Digest,
    /// Anonymous atom identifiers; order is presentation data only.
    pub base_atoms: Vec<Digest>,
    /// Anonymous atom identifiers; order is presentation data only.
    pub extension_atoms: Vec<Digest>,
    /// For each extension atom, the index of its base image.
    pub projection: Vec<u8>,
}
