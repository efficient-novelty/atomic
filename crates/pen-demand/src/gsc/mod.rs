//! Frozen, oracle-free grammar for the first closed-inductive demand slice.
//!
//! This module is deliberately separate from the relative census DTOs.  Its
//! semantic manifest identifies a calculus; its verifier manifest identifies
//! one implementation of that calculus.

mod compiler;
mod family;
mod inductive;
mod manifest;
mod reference;

#[cfg(test)]
mod tests;

pub use compiler::{compile_compute_v1, compile_use_v1};
pub use family::{
    CANONICAL_DEMAND_FAMILY_SCHEMA_VERSION, CanonicalDemandFamilyV2, ContextMapCode,
    EquationClauseCode, GscFamilyId, GscOriginEventId, GscRule, OutputClause, OutputPortId,
    OutputRole, PortKey, PortRef, PremiseRef, PublicPortId, PublicSourceId,
    VerifiedCanonicalDemandFamilyV2, VerifiedOutputPort,
};
pub use inductive::{
    BoundaryPortCode, CLOSED_FORMER_FRAME_SCHEMA_VERSION, CLOSED_INDUCTIVE_CODE_SCHEMA_VERSION,
    ClosedFormerFrame, ClosedFormerFrameId, ClosedInductiveCode, CodeTerm, ComputationMode,
    ConstructorCode, ConstructorPortId, FormerCodeId, IntroductionAlias, TelescopeCode,
    VerifiedClosedFormerFrame, VerifiedClosedInductiveCode, verify_closed_former_frame,
    verify_closed_inductive_code,
};
pub use manifest::{
    CanonicalOrderRule, ClosedInductiveGrammar, CompilerEquationManifest, ContextMapGrammar,
    EquationExtensionGrammar, FamilySchema, FreshEliminatorFillerGrammar,
    FreshEliminatorHeadGrammar, FreshHeadIdentityGrammar, GSC_REFERENCE_VECTOR_SPEC_TOKEN,
    GSC_SEMANTIC_MANIFEST_SCHEMA_VERSION, GSC_VERIFIER_MANIFEST_SCHEMA_VERSION,
    GeneratedEquationCoverage, GroundRuleDisposition, GscOutcome, GscProfile,
    GscReferenceVectorAgreement, GscReferenceVectorAgreementArtifactV1, GscReferenceVectorCaseV1,
    GscReferenceVectorDisposition, GscReferenceVectorId, GscReferenceVectorSuiteV1,
    GscSemanticLimits, GscSemanticManifestV1, GscUnknownReason, GscVerifierManifestV1,
    HistoryRuleGrammar, LiveUseFamilyShape, OperationalDerivabilityGrammarV1,
    OperationalDerivationRule, OperationalGoalUniverse, OperationalSaturationOrder, PortGrammar,
    Q0RewriteGrammar, Q2PresentationGrammar, Q3EdgeGrammar, ResponseCandidateCanonicalOrder,
    ResponseCandidateGrammarV1, SupportSemantics, VerifiedGscSemanticManifest,
    VerifiedGscVerifierManifest, current_gsc_verifier_manifest_v1,
    frozen_gsc_reference_vector_suite_v1, frozen_gsc_semantic_manifest_v1,
    gsc_agda_reference_digest, gsc_reference_agreement_gate_digest, gsc_toolchain_digest,
    gsc_verifier_source_digest, verify_gsc_semantic_manifest_v1, verify_gsc_verifier_manifest_v1,
};
pub use reference::{
    VerifiedGscReferenceAgreement, VerifiedGscReferenceVectorReplay,
    replay_gsc_reference_vectors_v1, verify_gsc_reference_agreement_v1,
};
