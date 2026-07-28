//! A small, deterministic verifier for the oracle-free Law V2 lane.
//!
//! This crate implements a deliberately limited dependent core. It is useful
//! for replaying certificate structure, scope, typing, normalization,
//! judgmental equality, and exact signature extension. It is **not** the
//! ambient cubical calculus required by the final theory: path types,
//! univalence, Kan operations, finite sums, and the free-sealing universal
//! property remain unavailable and therefore fail closed.

#![forbid(unsafe_code)]

mod certificate;
mod checker;
mod digest;
mod syntax;

pub use certificate::{
    CertificateBinding, CertificateClaim, CertificateError, ScopeInputs, TrustedScope,
    UncheckedClosedSpecializationCertificate, UncheckedDerivationCertificate,
    UncheckedEquivalenceCertificate, UncheckedFreeSealingCertificate, VerifiedClosedSpecialization,
    VerifiedDefinitionalEquivalence, VerifiedDerivation, VerifiedFreeSealing,
};
pub use checker::{
    Kernel, KernelError, KernelLimits, MAX_SAFE_RECURSION_DEPTH, ResourceKind, VerifiedContext,
    VerifiedSignature,
};
pub use digest::{CanonicalEncode, CanonicalEncoder, Digest, DigestError};
pub use syntax::{Declaration, DependentContext, GlobalId, OpenJudgment, Term, UncheckedSignature};
