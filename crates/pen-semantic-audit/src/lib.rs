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
pub mod demand_orbit_census_v3;
pub mod demand_realization_census_v3;
pub mod family_quotient_v3;
pub mod family_weakening_v3;
pub mod finite_rewrite;
mod fragment;
pub mod historical_rewrite;
pub mod inventory;
pub mod inventory_compatibility;
pub mod manifest;
pub mod marginal_family_set_v3;
pub mod model;
pub mod native_carrier_v3;
pub mod normalizer;
pub mod ordinary_beta;
pub mod production_inventory_bridge;
pub mod production_refinement;
pub mod production_refinement_theorem;
pub mod production_refinement_wire_authority;
pub mod production_transcript;
pub mod production_wire_builder;
pub mod production_wire_input;
pub mod production_wire_replay;
pub mod production_wire_slots;
pub mod provenance;
pub mod quotient;
pub mod restricted_kernel_cost_basis_v3;
pub mod rewrite_authority_v3;
pub mod rewrite_inventory;
pub mod semantic_authority;
pub mod semantic_authority_v3;
pub mod specialization;
pub mod sr2_dependency_support_v3;
pub mod sr2_noninjectivity_v3;
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
pub use demand_orbit_census_v3::{
    DEMAND_ORBIT_CENSUS_SCHEMA_VERSION_V3, DemandOrbitCensusFailureV3, VerifiedDemandOrbitCensusV3,
    diagnose_demand_orbit_census_v3, verify_demand_orbit_census_v3,
};
pub use demand_realization_census_v3::{
    DEMAND_REALIZATION_CENSUS_SCHEMA_VERSION_V3, DemandRealizationCensusFailureV3,
    VerifiedDemandRealizationCensusV3, diagnose_demand_realization_census_v3,
    verify_demand_realization_census_v3,
};
pub use family_quotient_v3::{
    FAMILY_QUOTIENT_SCHEMA_VERSION_V3, FamilyClassIdV3, FamilyQuotientFailureV3,
    NormalizedFamilyIdV3, Q0FamilyImageV3, Q0SeedImageV3, VerifiedFamilyClassV3,
    VerifiedFamilyQuotientV3, diagnose_family_quotient_v3, verify_family_quotient_v3,
};
pub use family_weakening_v3::{
    FAMILY_WEAKENING_SCHEMA_VERSION_V3, FamilyRestrictionMapV3, FamilyWeakeningFailureV3,
    FamilyWeakeningMapV3, RawFamilyWeakeningV3, VerifiedFamilyWeakeningV3,
    diagnose_family_weakening_v3, verify_family_weakening_v3,
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
pub use marginal_family_set_v3::{
    MARGINAL_FAMILY_SET_SCHEMA_VERSION_V3, MarginalFamilySetFailureV3, VerifiedMarginalFamilySetV3,
    diagnose_marginal_family_set_v3, verify_marginal_family_set_v3,
};
pub use model::*;
pub use native_carrier_v3::{
    CarrierRootKindV3, CarrierRootV3, NATIVE_CARRIER_SCHEMA_VERSION_V3, NativeCarrierFailureV3,
    VerifiedCarrierRootInventoryV3, VerifiedCarrierSubjectBundleV3,
    VerifiedNativeRankInductiveCarrierV3, WireInexpressibleReasonV3,
    diagnose_carrier_root_inventory_v3, diagnose_carrier_subject_bundle_v3,
    diagnose_native_rank_inductive_carrier_v3, verify_carrier_root_inventory_v3,
    verify_carrier_subject_bundle_v3, verify_native_rank_inductive_carrier_v3,
};
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
pub use production_inventory_bridge::{
    EXACT_FAMILY_CONSTRUCTOR_SHAPES_V1, EXACT_PRODUCTION_FAMILY_RULE_INVENTORY_V1,
    EXACT_V3_Q0_RULE_INVENTORY_V1, FamilyConstructorShapeV1, ProductionFamilyCodeV1,
    ProductionInventoryBridgeFailureV1, ProductionInventoryKindV1, ProductionQ0CategoryV1,
    VerifiedFamilyProductionMappingV1, VerifiedNoExtraNoMissingInventoryV1,
    VerifiedProductionInventoryBridgeV1, VerifiedQ0ProductionClassificationV1,
    diagnose_production_inventory_bridge_v1, family_constructor_shape_v1,
    production_family_code_for_derivation_rule_v1, production_family_code_for_shape_v1,
    production_family_code_v1, production_q0_category_v1, verify_production_inventory_bridge_v1,
};
pub use production_refinement::{
    GlobalSlotV1, LAMBDA_UNIT_INFERRED_PUBLIC_SORT_SUCCESSOR_LEVELS_V1,
    LAMBDA_UNIT_PUBLIC_AND_INFERRED_SORT_LEVELS_V1, LAMBDA_UNIT_PUBLIC_UNIVERSE_LEVELS_V1,
    PRODUCTION_REFINEMENT_SCHEMA_VERSION_V1, ProductionRefinementFailureV1,
    VerifiedGlobalDependencySlotV1, VerifiedGlobalSlotEntryV1, VerifiedGlobalSlotTableV1,
    VerifiedLambdaUnitPublicSortSuccessorsV1, VerifiedUniverseSuccessorV1,
    diagnose_global_slot_table_v1, verify_global_slot_table_v1,
    verify_lambda_unit_public_sort_successors_v1,
};
pub use production_refinement_theorem::correspondence_factory::{
    MintedProductionCorrespondencesV1, ProductionCorrespondenceFactoryFailureV1,
    ProductionCorrespondenceFactoryInputV1, mint_production_correspondences_v1,
};
pub use production_refinement_theorem::{
    LAMBDA_UNIT_PRODUCTION_REFINEMENT_SCHEMA_VERSION_V1, LambdaUnitProductionRefinementFailureV1,
    PRODUCTION_REFINEMENT_CORRESPONDENCE_FRONTIER_V1,
    PRODUCTION_REFINEMENT_EXCLUDED_DOWNSTREAM_OBLIGATION_V1,
    PredecessorPublicDeltaPolicyBindingFailureV1,
    ProductionRefinementExcludedDownstreamObligationV1, ProductionRefinementFrontierV1,
    ProductionRefinementObligationV1, ProductionSynthesisProtocolIdentityFailureV1,
    V3PredecessorPublicDeltaPolicyBindingFailureV1, VerifiedFiniteContextCorrespondenceV1,
    VerifiedKernelBaseConversionCorrespondenceV1, VerifiedLambdaUnitProductionRefinementV1,
    VerifiedPredecessorPublicDeltaPolicyBindingV1, VerifiedProductionRefinementAgdaFoundationV1,
    VerifiedProductionSynthesisProtocolIdentityV2, VerifiedSynthesisCodeCorrespondenceV1,
    VerifiedV3InventoryCorrespondenceV1, VerifiedV3PredecessorPublicDeltaPolicyBindingV1,
    diagnose_lambda_unit_production_refinement_v1,
    diagnose_lambda_unit_production_refinement_with_delta_policy_v1,
    diagnose_pinned_production_refinement_agda_foundation_v1,
    diagnose_predecessor_public_delta_policy_binding_v1,
    diagnose_production_synthesis_protocol_identity_v2,
    diagnose_v3_predecessor_public_delta_policy_binding_v1,
    verify_lambda_unit_production_refinement_v1,
    verify_pinned_production_refinement_agda_foundation_v1,
    verify_predecessor_public_delta_policy_binding_v1,
    verify_v3_predecessor_public_delta_policy_binding_v1,
};
pub use production_refinement_wire_authority::{
    AcceptedProductionSectionMaskV1, AcceptedProductionSectionV1,
    CANONICAL_PRODUCTION_AUTHORITY_FRONTIER_V1, CANONICAL_PRODUCTION_AUTHORITY_SCHEMA_VERSION_V1,
    COMPLETED_CANONICAL_PRODUCTION_PREREQUISITES_V1, CanonicalProductionAuthorityFailureV1,
    CanonicalProductionAuthorityFrontierV1, CanonicalProductionAuthorityObligationV1,
    ProductionBridgeFailureV1, REQUIRED_ACCEPTED_PRODUCTION_SECTIONS_V1,
    VerifiedAgdaProductionAcceptanceV1, VerifiedCanonicalProductionBundleV1,
    VerifiedProductionTranscriptAgreementV1, VerifiedRustProductionReplayV1,
    diagnose_canonical_production_authority_v1, verify_agda_production_acceptance_v1,
    verify_production_transcript_agreement_v1, verify_rust_production_replay_v1,
};
pub use production_transcript::{
    PRODUCTION_TRANSCRIPT_MAGIC_V1, PRODUCTION_TRANSCRIPT_SCHEMA_VERSION_V1,
    render_production_transcript_v1,
};
pub use production_wire_builder::{
    ProductionBundlePayloadV1, ProductionWireBuilderFailureV1, build_canonical_production_bundle_v1,
};
pub use production_wire_input::{
    MAX_PRODUCTION_BUNDLE_INPUT_BYTES_V1, PRODUCTION_BUNDLE_INPUT_MODULE_NAME_V1,
    ProductionBundleInputFailureV1, VerifiedAgdaProductionInputArtifactV1,
    extract_production_bundle_input_bytes_v1, render_production_bundle_input_module_v1,
    verify_agda_production_input_artifact_v1,
};
pub use production_wire_replay::{
    ProductionReplayComputedV1, ProductionReplayEvidenceV1, ProductionReplayFailureV1,
    replay_production_bundle_v1,
};
pub use production_wire_slots::{
    ProductionWireSlotFailureV1, derive_global_slot_table_wire_v1, term_to_wire_v1,
};
pub use provenance::{
    FamilyDependencySupportV1, ProvenanceCertificateV1, derive_family_dependency_support_v1,
    verify_sr2_injection_v1, verify_sr2_injection_with_support_v1,
};
pub use quotient::{QuotientCertificateV1, quotient_families_v1, quotient_families_with_q0_v1};
pub use restricted_kernel_cost_basis_v3::{
    RESTRICTED_KERNEL_COST_BASIS_SCHEMA_VERSION_V3, RestrictedKernelCostBasisFailureV3,
    RestrictedKernelCostViolationV3, VerifiedRestrictedFirstIrreducibleBasisClassV3,
    VerifiedRestrictedKernelCostBasisV3, diagnose_restricted_kernel_cost_basis_v3,
    verify_restricted_kernel_cost_basis_v3,
};
pub use rewrite_authority_v3::{
    OverlapPairV3, OverlapRelationV3, REWRITE_AUTHORITY_SCHEMA_VERSION_V3,
    RewriteAuthorityFailureV3, RewriteEdgeIdV3, RewriteEdgeV3, RewriteNodeIdV3,
    RewriteNodeJudgmentV3, RewriteNodeV3, RewritePositionV3, RewriteRuleV3,
    SubstitutionStabilityV3, VerifiedRewriteAuthorityV3, diagnose_rewrite_authority_v3,
    verify_rewrite_authority_v3,
};
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
pub use sr2_dependency_support_v3::{
    SR2_DEPENDENCY_SUPPORT_SCHEMA_VERSION_V3, Sr2DependencySupportFailureV3,
    VerifiedSr2DependencySupportCensusV3, VerifiedSr2FamilyDependencySupportV3,
    diagnose_sr2_dependency_support_v3, verify_sr2_dependency_support_v3,
};
pub use sr2_noninjectivity_v3::{
    SR2_LOCAL_ROLE_UNIVERSE_V3, SR2_NONINJECTIVITY_SCHEMA_VERSION_V3, Sr2NonInjectivityFailureV3,
    Sr2PigeonholeWitnessV3, VerifiedSr2NonInjectivityV3, VerifiedSr2RoleFiberV3,
    diagnose_sr2_noninjectivity_v3, verify_sr2_noninjectivity_v3,
};
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
    VerifiedSynthesisBackedTypedOccurrenceCensusV3, VerifiedTypedOccurrenceRootV2,
    VerifiedTypedOccurrenceV2, diagnose_synthesis_backed_typed_occurrence_census_v3,
    verify_carrier_derived_typed_occurrence_census_v3, verify_synthesis_backed_occurrence_batch_v1,
};
pub use typing_metatheory::{
    LambdaUnitTypingMetatheoryFailureV1, TYPING_PRODUCTION_CORRESPONDENCE_FRONTIER_V1,
    TypingProductionCorrespondenceObligationV1, VerifiedLambdaUnitTypingFoundationV1,
    VerifiedLambdaUnitTypingMetatheoryV1, diagnose_lambda_unit_typing_metatheory_v1,
    verify_lambda_unit_typing_metatheory_v1, verify_pinned_lambda_unit_typing_foundation_v1,
};
pub use weakening::{WeakeningCertificateV1, verify_weakening_and_marginals_v1};
