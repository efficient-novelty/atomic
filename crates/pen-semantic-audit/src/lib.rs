//! Finite, manifest-indexed semantic-audit prototype for the Law V2 GF2 core.
//!
//! This crate is intentionally isolated from the root Cargo workspace so that
//! generic audit development cannot drift the issued H3/H4 workspace-manifest
//! and lockfile bindings. It has no live Profile A adapter.

#![forbid(unsafe_code)]

pub mod agda_gate;
pub mod ambient;
pub mod carrier;
pub mod cost;
pub mod finite_rewrite;
mod fragment;
pub mod inventory;
pub mod manifest;
pub mod model;
pub mod normalizer;
pub mod ordinary_beta;
pub mod provenance;
pub mod quotient;
pub mod rewrite_inventory;
pub mod specialization;
pub mod weakening;

pub use agda_gate::{VerifiedAgdaReferenceV1, verify_pinned_agda_reference_v1};
pub use ambient::{
    AmbientExportDispositionV1, AmbientPrimitiveExportBoundaryV1,
    AmbientPrimitiveExportMemberKindV1, VerifiedAmbientPrimitiveExportCensusV1,
    VerifiedAmbientPrimitiveExportClassV1, VerifiedAmbientPrimitiveExportMemberV1,
    verify_ambient_primitive_export_census_v1,
};
pub use carrier::{
    CarrierCertificateV1, PreQ0RawCarrierCertificateV1, VerifiedPreQ0SemanticSeedV1,
    enumerate_pre_q0_raw_families_v1, enumerate_raw_families_v1, enumerate_raw_families_with_q0_v1,
    verify_pre_q0_semantic_seed_v1, verify_semantic_seed_v1, verify_semantic_seed_with_q0_v1,
};
pub use cost::{
    CostAuditCertificateV1, CostAuditCertificateV2, EquationDemandPortBindingV2,
    KernelCostAuditInputV2, audit_kernel_cost_lambda_unit_v2, audit_kernel_cost_v1,
    audit_kernel_cost_v2,
};
pub use finite_rewrite::{
    FiniteRewriteRuleV1, FiniteSubstitutionKindV1, RewriteDispositionOutcomeV1,
    RewriteDispositionV1, RewriteEdgeIdV1, RewriteInapplicableReasonV1, RewriteNodeIdV1,
    RewriteSubstitutionIdV1, TermPositionV1, VerifiedFiniteSubstitutionV1,
    VerifiedOverlapDiagnosticV1, VerifiedRewriteEdgeV1, VerifiedRewriteNodeV1,
    VerifiedRewriteSystemV1, verify_finite_rewrite_system_lambda_unit_v1,
    verify_finite_rewrite_system_v1,
};
pub use inventory::*;
pub use manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, VerifiedCostManifestV1,
    VerifiedCostManifestV2, VerifiedSemanticAuditManifestV1,
    proposed_kernel_cost_lambda_unit_manifest_v2, proposed_kernel_cost_manifest_v2,
    proposed_semantic_audit_lambda_unit_manifest_v1, verify_core_manifests_v1,
    verify_kernel_cost_lambda_unit_manifest_v2, verify_kernel_cost_manifest_v2,
    verify_semantic_audit_lambda_unit_manifest_v1,
};
pub use model::*;
pub use normalizer::{
    FreshConstructorClauseV1, FreshConstructorComputationRequestV1, RestrictedQ0CertificateScopeV1,
    RestrictedQ0ConfluenceCertificateV1, RestrictedQ0TerminationCertificateV1,
    VerifiedFreshConstructorComputationV1, VerifiedFreshConstructorRuleV1,
    VerifiedSourceNormalizedJudgmentV1, verify_fresh_constructor_computation_v1,
    verify_source_normalized_judgment_v1,
};
pub use ordinary_beta::{
    VerifiedOrdinaryBetaDerivationV1, VerifiedOrdinaryBetaReductionStepV1,
    VerifiedSequentialSubstitutionV1, verify_ordinary_beta_derivation_v1,
};
pub use provenance::{
    FamilyDependencySupportV1, ProvenanceCertificateV1, derive_family_dependency_support_v1,
    verify_sr2_injection_v1, verify_sr2_injection_with_support_v1,
};
pub use quotient::{QuotientCertificateV1, quotient_families_v1, quotient_families_with_q0_v1};
pub use rewrite_inventory::{
    VerifiedTypedRewriteEntryV1, VerifiedTypedRewriteInventoryV1, attempt_rewrite_admissibility_v1,
    compile_typed_rewrite_inventory_lambda_unit_v1, compile_typed_rewrite_inventory_v1,
};
pub use specialization::{VerifiedFamilyInstanceV1, verify_family_instance_v1};
pub use weakening::{WeakeningCertificateV1, verify_weakening_and_marginals_v1};
