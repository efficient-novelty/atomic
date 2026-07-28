//! Deterministic census over an explicitly supplied finite demand domain.
//!
//! Completeness from this crate is always relative to the verified signature,
//! finite registry, opaque width-two window, finite rules, and explicit
//! library seeds supplied to the computation. It confers no law authority and
//! makes no claim about an ambient or independently complete scheme calculus.

#![forbid(unsafe_code)]

mod census;
mod certificate;
mod model;

#[cfg(test)]
mod tests;

pub use census::{compute_relative_census, domain_digest, library_digest, window_digest};
pub use certificate::{
    RELATIVE_CENSUS_CERTIFICATE_VERSION, UncheckedRelativeCensusCertificate,
    VerifiedRelativeCensus, verify_relative_census_certificate,
};
pub use model::{
    FamilyId, FiniteDemandDomain, FiniteRule, InstanceId, LibrarySeeds, OpaqueWindow,
    RegisteredFamily, RegisteredInstance, RelativeCensus, RelativeCensusOutcome, StructuralSupport,
    UnknownReason,
};
