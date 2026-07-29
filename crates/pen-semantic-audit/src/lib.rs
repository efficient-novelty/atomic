//! Finite, manifest-indexed semantic-audit prototype for the Law V2 GF2 core.
//!
//! This crate is intentionally isolated from the root Cargo workspace so that
//! generic audit development cannot drift the issued H3/H4 workspace-manifest
//! and lockfile bindings. It has no live Profile A adapter.

#![forbid(unsafe_code)]

pub mod agda_gate;
pub mod ambient;
pub mod carrier;
pub mod construction_substitution;
pub mod cost;
pub mod finite_rewrite;
mod fragment;
pub mod historical_rewrite;
pub mod inventory;
pub mod inventory_compatibility;
pub mod manifest;
pub mod model;
pub mod normalizer;
pub mod ordinary_beta;
pub mod provenance;
pub mod quotient;
pub mod rewrite_inventory;
pub mod semantic_authority;
pub mod semantic_authority_v3;
pub mod specialization;
pub mod substitution_metatheory;
pub mod typed_occurrence;
pub mod typed_occurrence_v3;
pub mod typing_metatheory;
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
pub use construction_substitution::{
    CarrierTupleIdV2, ConstructionContextIdV2, ConstructionSubstitutionIdV2,
    ConstructionSubstitutionKindV2, EmbeddingSideV2, EquationFillerSideV2, OneHoleContextIdV2,
    VerifiedConstructionSubstitutionCensusV2, VerifiedDirectConstructionSubstitutionV2,
    VerifiedRankInductiveCarrierV2, attempt_rank_inductive_carrier_v2,
    verify_construction_substitution_census_v2,
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
pub use historical_rewrite::{
    HistoricalBaseQ0RuleV1, VerifiedEmptyHistoricalRewriteSystemV1,
    VerifiedHistoricalRewriteSystemV1, verify_historical_rewrite_system_v1,
};
pub use inventory::*;
pub use inventory_compatibility::{
    PreservedPublicInventoryInvariantV2, VerifiedPublicInventoryCompatibilityV2,
    verify_public_inventory_compatibility_v2,
};
pub use manifest::{
    AuditDecision, AuditUnknownReason, ConservativityProtocolV2, ConstructionSubstitutionRuleV2,
    DemandProvenanceProtocolV3, EquationPortMetadataProtocolV3, FreshRuleMatchingProtocolV2,
    GenericSubstitutionTheoremV2, HistoricalAuthorityProtocolV3, HistoricalRewriteProtocolV2,
    OutsideFragmentReason, OverlapCensusProtocolV2, SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V3,
    SEMANTIC_AUDIT_SCHEMA_VERSION_V3, SemanticAuditManifestV2, SemanticAuditManifestV3,
    SemanticAuthorityStageV3, SemanticIdentityProtocolV3, SubstitutionCensusScopeV2,
    TypedOccurrenceCensusProtocolV2, VerifiedCostManifestV1, VerifiedCostManifestV2,
    VerifiedSemanticAuditManifestV1, VerifiedSemanticAuditManifestV2,
    VerifiedSemanticAuditManifestV3, proposed_kernel_cost_lambda_unit_manifest_v2,
    proposed_kernel_cost_manifest_v2, proposed_semantic_audit_lambda_unit_manifest_v1,
    proposed_semantic_audit_lambda_unit_manifest_v2,
    proposed_semantic_audit_lambda_unit_manifest_v3, verify_core_manifests_v1,
    verify_kernel_cost_lambda_unit_manifest_v2, verify_kernel_cost_manifest_v2,
    verify_semantic_audit_lambda_unit_manifest_v1, verify_semantic_audit_lambda_unit_manifest_v2,
    verify_semantic_audit_lambda_unit_manifest_v3,
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
pub use semantic_authority::{
    DemandAnchorBlockerV1, PublicClauseSubjectV1, PublicSemanticSeedSubjectV2,
    VerifiedDemandAnchorCensusV1, VerifiedDemandOrbitV1, VerifiedDemandPortV1,
    VerifiedDemandRealizationV1, VerifiedPreQ0SemanticSeedV2, VerifiedPublicClauseCensusV1,
    VerifiedPublicClauseIdentityV1, VerifiedSemanticSeedCensusV2, demand_anchor_blockers_v1,
    verify_demand_anchor_census_v1, verify_public_clause_census_v1, verify_semantic_seed_census_v2,
};
pub use semantic_authority_v3::{
    PublicSemanticSeedSubjectV3, SeedIdV3, VerifiedBoundEquationPortMetadataV3,
    VerifiedPreQ0SemanticSeedV3, VerifiedSemanticSeedBaseCensusV3, VerifiedStructuralSupportV1,
    verify_semantic_seed_base_census_v3,
};
pub use specialization::{VerifiedFamilyInstanceV1, verify_family_instance_v1};
pub use substitution_metatheory::{
    SubstitutionMetatheoryFailureV1, TypedSimultaneousSubstitutionV1,
    VerifiedRawSubstitutionAlgebraV1, VerifiedSubstitutionMetatheoryV1,
    compose_typed_substitutions_v1, diagnose_substitution_metatheory_v1,
    identity_typed_substitution_v1, lift_typed_substitution_v1,
    verify_pinned_raw_substitution_algebra_v1, verify_substitution_metatheory_v1,
    verify_typed_simultaneous_substitution_v1, weakening_typed_substitution_v1,
};
pub use typed_occurrence::{
    LocalJudgmentV1, TYPED_OCCURRENCE_CENSUS_SCHEMA_VERSION_V1, TypedBinderKindV1,
    TypedOccurrenceIdV1, TypedOccurrencePathComponentV1, TypedOccurrencePathV1,
    TypedOccurrenceRootIdV1, TypedOccurrenceRootRequestV1, VerifiedTypedOccurrenceCensusV1,
    VerifiedTypedOccurrenceRootV1, VerifiedTypedOccurrenceV1, verify_typed_occurrence_census_v2,
};
pub use typed_occurrence_v3::{
    SYNTHESIS_BACKED_OCCURRENCE_BATCH_SCHEMA_VERSION_V1,
    SYNTHESIS_BACKED_OCCURRENCE_CENSUS_PREREQUISITES_V3,
    SynthesisBackedOccurrenceCensusPrerequisiteV3, TypedOccurrenceIdV2, TypedOccurrencePathV2,
    TypedOccurrenceRootIdV2, VerifiedSynthesisBackedOccurrenceBatchV1,
    VerifiedTypedOccurrenceRootV2, VerifiedTypedOccurrenceV2,
    diagnose_synthesis_backed_typed_occurrence_census_v3,
    verify_synthesis_backed_occurrence_batch_v1,
};
pub use typing_metatheory::{
    LambdaUnitTypingMetatheoryFailureV1, TYPING_PRODUCTION_CORRESPONDENCE_FRONTIER_V1,
    TypingProductionCorrespondenceObligationV1, VerifiedLambdaUnitTypingFoundationV1,
    VerifiedLambdaUnitTypingMetatheoryV1, diagnose_lambda_unit_typing_metatheory_v1,
    verify_lambda_unit_typing_metatheory_v1, verify_pinned_lambda_unit_typing_foundation_v1,
};
pub use weakening::{WeakeningCertificateV1, verify_weakening_and_marginals_v1};
