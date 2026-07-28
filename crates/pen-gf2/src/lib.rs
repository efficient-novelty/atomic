//! Versioned, bounded contracts for a finite proof fragment.
//!
//! The manifest describes the complete finite language that a caller intends
//! to use. A backend may implement only a subset of that language. Unsupported
//! syntax and resource exhaustion are explicit decisions; neither is negative
//! evidence. Deserializable decisions are claims only, while successful replay
//! returns opaque, non-deserializable handles.

#![forbid(unsafe_code)]

mod adapter;
mod contract;

pub use adapter::{
    FragmentValidationError, LawDecision, LawKernel, NativeKernelAdapter, NativeVerifiedProof,
    NativeVerifiedRefutation, OutsideFragment, OutsideReason, ResourceExhausted,
    VerifiedFiniteFragment,
};
pub use contract::{
    CANONICAL_ORDER_VERSION, CanonicalOrder, DECISION_CLAIM_SCHEMA_VERSION,
    FINITE_FRAGMENT_SCHEMA_VERSION, FiniteFeature, FiniteFeatureDeclaration, FiniteFragmentLimits,
    FiniteResource, LAW_REQUEST_SCHEMA_VERSION, UncheckedDecisionClaim,
    UncheckedFiniteFragmentManifest, UncheckedLawRequest, UncheckedLawSubject,
};
