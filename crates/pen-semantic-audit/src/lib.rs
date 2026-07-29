//! Finite, manifest-indexed semantic-audit prototype for the Law V2 GF2 core.
//!
//! This crate is intentionally isolated from the root Cargo workspace so that
//! generic audit development cannot drift the issued H3/H4 workspace-manifest
//! and lockfile bindings. It has no live Profile A adapter.

#![forbid(unsafe_code)]

pub mod agda_gate;
pub mod carrier;
pub mod cost;
pub mod manifest;
pub mod model;
pub mod normalizer;
pub mod provenance;
pub mod quotient;
pub mod weakening;

pub use agda_gate::{VerifiedAgdaReferenceV1, verify_pinned_agda_reference_v1};
pub use carrier::{
    CarrierCertificateV1, enumerate_raw_families_v1, enumerate_raw_families_with_q0_v1,
    verify_semantic_seed_v1, verify_semantic_seed_with_q0_v1,
};
pub use cost::{CostAuditCertificateV1, audit_kernel_cost_v1};
pub use manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, VerifiedCostManifestV1,
    VerifiedSemanticAuditManifestV1, verify_core_manifests_v1,
};
pub use model::*;
pub use normalizer::{
    FreshConstructorClauseV1, FreshConstructorComputationRequestV1, RestrictedQ0CertificateScopeV1,
    RestrictedQ0ConfluenceCertificateV1, RestrictedQ0TerminationCertificateV1,
    VerifiedFreshConstructorComputationV1, VerifiedFreshConstructorRuleV1,
    VerifiedSourceNormalizedJudgmentV1, verify_fresh_constructor_computation_v1,
    verify_source_normalized_judgment_v1,
};
pub use provenance::{
    FamilyDependencySupportV1, ProvenanceCertificateV1, derive_family_dependency_support_v1,
    verify_sr2_injection_v1, verify_sr2_injection_with_support_v1,
};
pub use quotient::{QuotientCertificateV1, quotient_families_v1};
pub use weakening::{WeakeningCertificateV1, verify_weakening_and_marginals_v1};
