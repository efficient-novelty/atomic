//! Finite generic prototype for proposed contextual internalization.
//!
//! Nothing in this crate is adopted Law V2 authority. Verified values certify
//! only the bounded generic fragment documented by their types.

#![forbid(unsafe_code)]

pub mod certificate;
pub mod context;
pub mod finite;
pub mod manifest;
pub mod model;

pub use certificate::{
    AnonymousUniversalCertificateV1, HomEquivalenceCellV1, InterfaceExposureV1, TriangleIdentityV1,
    TwoSidedCertificateInputV1, VerifiedTwoSidedCompletionV1, verify_two_sided_completion,
};
pub use context::{
    VerifiedContextProjectionV1, VerifiedProjectionReindexingV1, verify_projection_reindexing,
    verify_single_extension_projection,
};
pub use finite::{VerifiedFiniteProjectionModelV1, verify_finite_projection_model};
pub use manifest::{ProposedContextualManifestV1, proposed_contextual_manifest_v1};
pub use model::{
    AnonymousOperationIdV1, AuditDecisionV1, AuditFailureV1, ContextSideV1,
    FiniteProjectionInputV1, HomMorphismV1, PolarityV1, ProjectionReindexingInputV1,
    SealedPublicJudgmentV1, SubsetObjectV1,
};
