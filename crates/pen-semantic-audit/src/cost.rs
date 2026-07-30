//! Finite, certificate-carrying kernel-cost audit.
//!
//! This module deliberately contains no live Profile-A adapter.  It checks a
//! small generic grammar whose only free reconstruction rules are those named
//! by its verified cost manifest.  The checker is exhaustive under the
//! manifest bounds: it never turns failed proof search into irreducibility.

use crate::ambient::{
    AmbientExportDispositionV1, AmbientPrimitiveExportMemberKindV1,
    VerifiedAmbientPrimitiveExportCensusV1, verify_ambient_primitive_export_census_v1,
};
use crate::finite_rewrite::VerifiedRewriteSystemV1;
use crate::fragment::{
    LambdaUnitSyntaxViolation, lambda_unit_context_syntax_violation,
    lambda_unit_judgment_syntax_violation, lambda_unit_term_syntax_violation,
};
use crate::manifest::{
    AuditDecision, AuditUnknownReason, FreeCompletionRuleV1, FreeCompletionRuleV2,
    KERNEL_COST_LAMBDA_UNIT_PROFILE_ID_V2, KERNEL_COST_PROFILE_ID_V2, OutsideFragmentReason,
    SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1, VerifiedCostManifestV1, VerifiedCostManifestV2,
    VerifiedSemanticAuditManifestV1,
};
use crate::model::{
    ClauseCostDispositionV1, ClauseIdV1, DemandOutputIdV1, EquationIdV1, GenericJudgmentV1,
    HeadPresentationV1, LocalRoleV1, PublicAvailabilityV1,
};
use crate::{
    DemandPortKeyV1, PublicSubjectV1, VerifiedPublicAuditInventoryV1, VerifiedPublicDeclarationV1,
    VerifiedPublicEquationV1, ordinary_beta::VerifiedOrdinaryBetaDerivationV1,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Term};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// The V1 source-to-normal grammar is intentionally small and replayable.
///
/// A future profile may add checked kernel steps.  It must not accept an
/// opaque digest as a substitute for such a derivation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceToNormalDerivationV1 {
    Reflexivity,
    /// Untrusted marker selecting a verifier-minted ordinary-beta path.
    ///
    /// V1 rejects this marker.  Public V2 accepts it only when the source and
    /// normal pair exactly match the verified inventory and an opaque
    /// `VerifiedOrdinaryBetaDerivationV1` is supplied separately.
    VerifierReplayedOrdinaryBeta,
}

impl CanonicalEncode for SourceToNormalDerivationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Reflexivity => encoder.tag(0),
            Self::VerifierReplayedOrdinaryBeta => encoder.tag(1),
        }
    }
}

/// A source declaration and its claimed normal view form one raw clause.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceNormalizedDeclarationV1 {
    pub source_identity: Digest,
    pub source_context: DependentContext,
    pub source_type: Term,
    pub source_body: Option<Term>,
    pub normalized_context: DependentContext,
    pub normalized_type: Term,
    pub normalized_body: Option<Term>,
    pub source_to_normal_derivation: SourceToNormalDerivationV1,
}

impl CanonicalEncode for SourceNormalizedDeclarationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source_identity.encode_canonical(encoder);
        self.source_context.encode_canonical(encoder);
        self.source_type.encode_canonical(encoder);
        encoder.option(&self.source_body);
        self.normalized_context.encode_canonical(encoder);
        self.normalized_type.encode_canonical(encoder);
        encoder.option(&self.normalized_body);
        self.source_to_normal_derivation.encode_canonical(encoder);
    }
}

/// A source equation and its claimed normal view form one raw clause.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SourceNormalizedEquationV1 {
    pub source_identity: Digest,
    pub source_context: DependentContext,
    pub source_left: Term,
    pub source_right: Term,
    pub source_type: Term,
    pub normalized_context: DependentContext,
    pub normalized_left: Term,
    pub normalized_right: Term,
    pub normalized_type: Term,
    pub source_to_normal_derivation: SourceToNormalDerivationV1,
}

impl CanonicalEncode for SourceNormalizedEquationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source_identity.encode_canonical(encoder);
        self.source_context.encode_canonical(encoder);
        self.source_left.encode_canonical(encoder);
        self.source_right.encode_canonical(encoder);
        self.source_type.encode_canonical(encoder);
        self.normalized_context.encode_canonical(encoder);
        self.normalized_left.encode_canonical(encoder);
        self.normalized_right.encode_canonical(encoder);
        self.normalized_type.encode_canonical(encoder);
        self.source_to_normal_derivation.encode_canonical(encoder);
    }
}

/// Closed tags admitted by equation-free metadata.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ComputationModeV1 {
    ClosedNullary,
}

impl CanonicalEncode for ComputationModeV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::ClosedNullary => encoder.tag(0),
        }
    }
}

/// Closed descriptor-field grammar.
///
/// None of these variants can carry a term, equation, equation identifier,
/// digest, byte string, or caller-defined extension payload.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum EquationFreeDescriptorKindV1 {
    OperationRole {
        role: LocalRoleV1,
    },
    Former {
        former: GlobalId,
        constructor: Option<GlobalId>,
        computation_mode: ComputationModeV1,
    },
    RecordField {
        record: GlobalId,
        field_ordinal: u16,
        field_type: GlobalId,
    },
    FreshComputationContract {
        demand_output: DemandOutputIdV1,
        constructor: GlobalId,
        result: GlobalId,
        result_type: GlobalId,
        computation_mode: ComputationModeV1,
    },
}

impl CanonicalEncode for EquationFreeDescriptorKindV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::OperationRole { role } => {
                encoder.tag(0);
                role.encode_canonical(encoder);
            }
            Self::Former {
                former,
                constructor,
                computation_mode,
            } => {
                encoder.tag(1);
                former.encode_canonical(encoder);
                encoder.option(constructor);
                computation_mode.encode_canonical(encoder);
            }
            Self::RecordField {
                record,
                field_ordinal,
                field_type,
            } => {
                encoder.tag(2);
                record.encode_canonical(encoder);
                encoder.u16(*field_ordinal);
                field_type.encode_canonical(encoder);
            }
            Self::FreshComputationContract {
                demand_output,
                constructor,
                result,
                result_type,
                computation_mode,
            } => {
                encoder.tag(3);
                demand_output.encode_canonical(encoder);
                constructor.encode_canonical(encoder);
                result.encode_canonical(encoder);
                result_type.encode_canonical(encoder);
                computation_mode.encode_canonical(encoder);
            }
        }
    }
}

/// Equation-free immutable metadata attached to its owning declaration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EquationFreeDescriptorV1 {
    pub owner: GlobalId,
    pub source_clause: ClauseIdV1,
    pub descriptor: EquationFreeDescriptorKindV1,
    pub public_support: BTreeSet<GlobalId>,
}

impl CanonicalEncode for EquationFreeDescriptorV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.owner.encode_canonical(encoder);
        self.source_clause.encode_canonical(encoder);
        self.descriptor.encode_canonical(encoder);
        encode_set(encoder, &self.public_support);
    }
}

/// The exact raw public-clause grammar accepted by the V1 checker.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RawPublicClauseKindV1 {
    PublicDeclaration {
        head: GlobalId,
        pair: SourceNormalizedDeclarationV1,
        presentation: HeadPresentationV1,
        public_group: GlobalId,
        equation_free_descriptors: Vec<EquationFreeDescriptorV1>,
    },
    PublicEquation {
        equation: crate::model::EquationIdV1,
        owner_head: GlobalId,
        demand_output: Option<DemandOutputIdV1>,
        pair: SourceNormalizedEquationV1,
    },
    ForcedProjectionClause {
        projection: GlobalId,
        record_owner: ClauseIdV1,
        field_ordinal: u16,
        pair: SourceNormalizedDeclarationV1,
    },
}

impl CanonicalEncode for RawPublicClauseKindV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::PublicDeclaration {
                head,
                pair,
                presentation,
                public_group,
                equation_free_descriptors,
            } => {
                encoder.tag(0);
                head.encode_canonical(encoder);
                pair.encode_canonical(encoder);
                presentation.encode_canonical(encoder);
                public_group.encode_canonical(encoder);
                encoder.sequence(equation_free_descriptors);
            }
            Self::PublicEquation {
                equation,
                owner_head,
                demand_output,
                pair,
            } => {
                encoder.tag(1);
                equation.encode_canonical(encoder);
                owner_head.encode_canonical(encoder);
                encoder.option(demand_output);
                pair.encode_canonical(encoder);
            }
            Self::ForcedProjectionClause {
                projection,
                record_owner,
                field_ordinal,
                pair,
            } => {
                encoder.tag(2);
                projection.encode_canonical(encoder);
                record_owner.encode_canonical(encoder);
                encoder.u16(*field_ordinal);
                pair.encode_canonical(encoder);
            }
        }
    }
}

/// One raw clause plus its exact semantic dependency edges.
///
/// These edges may contain SCCs.  They are distinct from the acyclic public
/// precedence relation used by `DependencyPriorExport`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RawPublicClauseV1 {
    pub id: ClauseIdV1,
    pub semantic_dependencies: BTreeSet<ClauseIdV1>,
    pub clause: RawPublicClauseKindV1,
}

impl CanonicalEncode for RawPublicClauseV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        encode_set(encoder, &self.semantic_dependencies);
        self.clause.encode_canonical(encoder);
    }
}

/// An order-independent edge in the canonical public dependency DAG.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicDependencyEdgeV1 {
    pub dependent: ClauseIdV1,
    pub prerequisite: ClauseIdV1,
}

impl CanonicalEncode for PublicDependencyEdgeV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.dependent.encode_canonical(encoder);
        self.prerequisite.encode_canonical(encoder);
    }
}

/// Replayable structural cases establishing the fresh-computation theorem.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshTypingProofV1 {
    ClosedPublicReferences,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshSubstitutionProofV1 {
    ClosedEquation,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshRewriteProofV1 {
    FreshHeadSingleNonrecursiveRule,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshRoundTripProofV1 {
    DeleteThenDeterministicallyRegenerate,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshUniversalPropertyV1 {
    UniqueClosedDemandExtension,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshTransportProofV1 {
    StructuralQ0Q2Q3,
}

macro_rules! canonical_singleton_tag {
    ($type:ty, $variant:path) => {
        impl CanonicalEncode for $type {
            fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
                match self {
                    $variant => encoder.tag(0),
                }
            }
        }
    };
}

canonical_singleton_tag!(
    FreshTypingProofV1,
    FreshTypingProofV1::ClosedPublicReferences
);
canonical_singleton_tag!(
    FreshSubstitutionProofV1,
    FreshSubstitutionProofV1::ClosedEquation
);
canonical_singleton_tag!(
    FreshRewriteProofV1,
    FreshRewriteProofV1::FreshHeadSingleNonrecursiveRule
);
canonical_singleton_tag!(
    FreshRoundTripProofV1,
    FreshRoundTripProofV1::DeleteThenDeterministicallyRegenerate
);
canonical_singleton_tag!(
    FreshUniversalPropertyV1,
    FreshUniversalPropertyV1::UniqueClosedDemandExtension
);
canonical_singleton_tag!(
    FreshTransportProofV1,
    FreshTransportProofV1::StructuralQ0Q2Q3
);

/// A serialized claim for the only fresh-computation schema in this prototype.
///
/// The conclusion is reconstructed from the pre-existing equation-free
/// contract on `head`; this record cannot carry an equation payload.  It is
/// deliberately not an opaque, verifier-minted theorem capability: the
/// prototype therefore validates its shape but returns
/// `MissingFreeCompletionTheorem` before accepting the reconstruction as free.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExactFreshCompletionProofV1 {
    pub head: ClauseIdV1,
    pub demand_output: DemandOutputIdV1,
    pub typing: FreshTypingProofV1,
    pub substitution_stability: FreshSubstitutionProofV1,
    pub rewrite_system: FreshRewriteProofV1,
    pub exact_api_round_trip: FreshRoundTripProofV1,
    pub universal_property: FreshUniversalPropertyV1,
    pub transport_invariance: FreshTransportProofV1,
}

impl CanonicalEncode for ExactFreshCompletionProofV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.head.encode_canonical(encoder);
        self.demand_output.encode_canonical(encoder);
        self.typing.encode_canonical(encoder);
        self.substitution_stability.encode_canonical(encoder);
        self.rewrite_system.encode_canonical(encoder);
        self.exact_api_round_trip.encode_canonical(encoder);
        self.universal_property.encode_canonical(encoder);
        self.transport_invariance.encode_canonical(encoder);
    }
}

/// Exact finite reconstruction grammar.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "rule", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReconstructionProofV1 {
    OrdinaryBeta {
        definition: ClauseIdV1,
    },
    PriorPublicTransparentAlias {
        declaration: ClauseIdV1,
        target: GlobalId,
    },
    DescriptorForcedProjection {
        descriptor_owner: ClauseIdV1,
        field_ordinal: u16,
    },
    CertifiedFreshConstructorComputation {
        theorem: ExactFreshCompletionProofV1,
    },
    DuplicatePresentationDeletion {
        original: ClauseIdV1,
    },
}

impl ReconstructionProofV1 {
    pub fn rule(&self) -> FreeCompletionRuleV1 {
        match self {
            Self::OrdinaryBeta { .. } => FreeCompletionRuleV1::OrdinaryBeta,
            Self::PriorPublicTransparentAlias { .. } => {
                FreeCompletionRuleV1::PriorPublicTransparentAlias
            }
            Self::DescriptorForcedProjection { .. } => {
                FreeCompletionRuleV1::DescriptorForcedProjection
            }
            Self::CertifiedFreshConstructorComputation { .. } => {
                FreeCompletionRuleV1::CertifiedFreshConstructorComputation
            }
            Self::DuplicatePresentationDeletion { .. } => {
                FreeCompletionRuleV1::DuplicatePresentationDeletion
            }
        }
    }
}

impl CanonicalEncode for ReconstructionProofV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::OrdinaryBeta { definition } => {
                encoder.tag(0);
                definition.encode_canonical(encoder);
            }
            Self::PriorPublicTransparentAlias {
                declaration,
                target,
            } => {
                encoder.tag(1);
                declaration.encode_canonical(encoder);
                target.encode_canonical(encoder);
            }
            Self::DescriptorForcedProjection {
                descriptor_owner,
                field_ordinal,
            } => {
                encoder.tag(2);
                descriptor_owner.encode_canonical(encoder);
                encoder.u16(*field_ordinal);
            }
            Self::CertifiedFreshConstructorComputation { theorem } => {
                encoder.tag(3);
                theorem.encode_canonical(encoder);
            }
            Self::DuplicatePresentationDeletion { original } => {
                encoder.tag(4);
                original.encode_canonical(encoder);
            }
        }
    }
}

/// One Horn-style reconstruction rule.  `premises` is checked against the
/// exact premise set determined by `proof`.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReconstructionDerivationV1 {
    pub output: ClauseIdV1,
    pub premises: BTreeSet<ClauseIdV1>,
    pub proof: ReconstructionProofV1,
}

impl CanonicalEncode for ReconstructionDerivationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.output.encode_canonical(encoder);
        encode_set(encoder, &self.premises);
        self.proof.encode_canonical(encoder);
    }
}

/// A checkable Horn-closure model separating `target` from `against`.
///
/// `checked_rules` must equal the complete manifest rule inventory.  The
/// verifier checks that `realized` contains `against`, omits `target`, and is
/// closed under every supplied, structurally verified reconstruction.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompleteNegativeEvidenceV1 {
    pub target: ClauseIdV1,
    pub against: BTreeSet<ClauseIdV1>,
    pub realized: BTreeSet<ClauseIdV1>,
    pub checked_rules: Vec<FreeCompletionRuleV1>,
}

impl CanonicalEncode for CompleteNegativeEvidenceV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.target.encode_canonical(encoder);
        encode_set(encoder, &self.against);
        encode_set(encoder, &self.realized);
        encoder.sequence(&self.checked_rules);
    }
}

/// Complete finite negative evidence under the V2 cost-free rule inventory.
///
/// The verifier checks the inventory exactly; omitting a rule or retaining
/// the superseded fresh-completion rule cannot establish irreducibility.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompleteNegativeEvidenceV2 {
    pub target: ClauseIdV1,
    pub against: BTreeSet<ClauseIdV1>,
    pub realized: BTreeSet<ClauseIdV1>,
    pub checked_rules: Vec<FreeCompletionRuleV2>,
}

impl CanonicalEncode for CompleteNegativeEvidenceV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.target.encode_canonical(encoder);
        encode_set(encoder, &self.against);
        encode_set(encoder, &self.realized);
        encoder.sequence(&self.checked_rules);
    }
}

/// Explicit Q2 evidence used only to compare otherwise distinct bases.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationEquivalenceProofV1 {
    DuplicateTransparentField,
}

impl CanonicalEncode for PresentationEquivalenceProofV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::DuplicateTransparentField => encoder.tag(0),
        }
    }
}

/// A directed, presentation-independent choice of a Q2 class representative.
///
/// The direction is part of the supplied quotient proof, not declaration
/// order and not a hash tie-break.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PresentationEquivalenceV1 {
    pub representative: ClauseIdV1,
    pub equivalent: ClauseIdV1,
    pub proof: PresentationEquivalenceProofV1,
}

/// V2 Q2 presentation witnesses are schema-separated from V1.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PresentationEquivalenceProofV2 {
    DuplicateTransparentField,
    DuplicateNormalizedEquation,
}

impl CanonicalEncode for PresentationEquivalenceProofV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::DuplicateTransparentField => 0,
            Self::DuplicateNormalizedEquation => 1,
        });
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PresentationEquivalenceV2 {
    pub representative: ClauseIdV1,
    pub equivalent: ClauseIdV1,
    pub proof: PresentationEquivalenceProofV2,
}

impl CanonicalEncode for PresentationEquivalenceV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.representative.encode_canonical(encoder);
        self.equivalent.encode_canonical(encoder);
        self.proof.encode_canonical(encoder);
    }
}

impl CanonicalEncode for PresentationEquivalenceV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.representative.encode_canonical(encoder);
        self.equivalent.encode_canonical(encoder);
        self.proof.encode_canonical(encoder);
    }
}

/// Complete input to the isolated generic cost audit.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KernelCostAuditInputV1 {
    pub clauses: Vec<RawPublicClauseV1>,
    pub public_dependency_dag: Vec<PublicDependencyEdgeV1>,
    pub reconstructions: Vec<ReconstructionDerivationV1>,
    pub negative_evidence: Vec<CompleteNegativeEvidenceV1>,
    pub presentation_equivalences: Vec<PresentationEquivalenceV1>,
}

/// Untrusted V2 reconstruction syntax. Availability-based rules are absent:
/// declaration aliases and exact predecessor-equation replays are derived
/// only from the opaque verified inventory.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "rule", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReconstructionProofV2 {
    OrdinaryBetaOfBodyfulDefinition {
        definition: ClauseIdV1,
    },
    DescriptorForcedProjection {
        descriptor_owner: ClauseIdV1,
        field_ordinal: u16,
    },
    DuplicatePresentationDeletion {
        original: ClauseIdV1,
    },
}

impl ReconstructionProofV2 {
    fn rule(&self) -> FreeCompletionRuleV2 {
        match self {
            Self::OrdinaryBetaOfBodyfulDefinition { .. } => {
                FreeCompletionRuleV2::OrdinaryBetaOfBodyfulDefinition
            }
            Self::DescriptorForcedProjection { .. } => {
                FreeCompletionRuleV2::DescriptorForcedProjection
            }
            Self::DuplicatePresentationDeletion { .. } => {
                FreeCompletionRuleV2::DuplicatePresentationDeletion
            }
        }
    }
}

impl CanonicalEncode for ReconstructionProofV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::OrdinaryBetaOfBodyfulDefinition { definition } => {
                encoder.tag(0);
                definition.encode_canonical(encoder);
            }
            Self::DescriptorForcedProjection {
                descriptor_owner,
                field_ordinal,
            } => {
                encoder.tag(1);
                descriptor_owner.encode_canonical(encoder);
                encoder.u16(*field_ordinal);
            }
            Self::DuplicatePresentationDeletion { original } => {
                encoder.tag(2);
                original.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ReconstructionDerivationV2 {
    pub output: ClauseIdV1,
    pub premises: BTreeSet<ClauseIdV1>,
    pub proof: ReconstructionProofV2,
}

impl CanonicalEncode for ReconstructionDerivationV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.output.encode_canonical(encoder);
        encode_set(encoder, &self.premises);
        self.proof.encode_canonical(encoder);
    }
}

/// Exact equation-to-demand-port metadata for the V2 public inventory.
///
/// The full key is carried here because the V1 raw equation wire contains
/// only the output component.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EquationDemandPortBindingV2 {
    pub equation: EquationIdV1,
    pub demand_port: Option<DemandPortKeyV1>,
}

impl CanonicalEncode for EquationDemandPortBindingV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.equation.encode_canonical(encoder);
        encoder.option(&self.demand_port);
    }
}

/// Complete input to the isolated generic V2 cost audit.
///
/// Availability-based reconstructions are intentionally not serialized.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KernelCostAuditInputV2 {
    pub clauses: Vec<RawPublicClauseV1>,
    pub public_dependency_dag: Vec<PublicDependencyEdgeV1>,
    pub equation_demand_ports: Vec<EquationDemandPortBindingV2>,
    pub reconstructions: Vec<ReconstructionDerivationV2>,
    pub negative_evidence: Vec<CompleteNegativeEvidenceV2>,
    pub presentation_equivalences: Vec<PresentationEquivalenceV2>,
}

/// Inventory-replayed reconstruction proof recorded in a V2 certificate.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub enum VerifiedReconstructionProofV2 {
    OrdinaryBetaOfVerifiedDeclaration {
        owner_head: GlobalId,
        derivation_digest: Digest,
    },
    PriorPublicTransparentAlias {
        declaration: ClauseIdV1,
        target: GlobalId,
    },
    PriorPublicEquationReplay {
        predecessor_equation: EquationIdV1,
        owner_head: GlobalId,
    },
    DescriptorForcedProjection {
        descriptor_owner: ClauseIdV1,
        field_ordinal: u16,
        projection_census_digest: Digest,
    },
    DuplicatePresentationDeletion {
        original: ClauseIdV1,
    },
    PriorPublicAmbientReexport {
        declaration: ClauseIdV1,
        primitive_class_digest: Digest,
        predecessor_class_digest: Digest,
    },
    AmbientFirstExportClassDuplicate {
        primitive_class_digest: Digest,
        peer: ClauseIdV1,
    },
}

impl CanonicalEncode for VerifiedReconstructionProofV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::OrdinaryBetaOfVerifiedDeclaration {
                owner_head,
                derivation_digest,
            } => {
                encoder.tag(0);
                owner_head.encode_canonical(encoder);
                derivation_digest.encode_canonical(encoder);
            }
            Self::PriorPublicTransparentAlias {
                declaration,
                target,
            } => {
                encoder.tag(1);
                declaration.encode_canonical(encoder);
                target.encode_canonical(encoder);
            }
            Self::PriorPublicEquationReplay {
                predecessor_equation,
                owner_head,
            } => {
                encoder.tag(2);
                predecessor_equation.encode_canonical(encoder);
                owner_head.encode_canonical(encoder);
            }
            Self::DescriptorForcedProjection {
                descriptor_owner,
                field_ordinal,
                projection_census_digest,
            } => {
                encoder.tag(3);
                descriptor_owner.encode_canonical(encoder);
                encoder.u16(*field_ordinal);
                projection_census_digest.encode_canonical(encoder);
            }
            Self::DuplicatePresentationDeletion { original } => {
                encoder.tag(4);
                original.encode_canonical(encoder);
            }
            Self::PriorPublicAmbientReexport {
                declaration,
                primitive_class_digest,
                predecessor_class_digest,
            } => {
                encoder.tag(5);
                declaration.encode_canonical(encoder);
                primitive_class_digest.encode_canonical(encoder);
                predecessor_class_digest.encode_canonical(encoder);
            }
            Self::AmbientFirstExportClassDuplicate {
                primitive_class_digest,
                peer,
            } => {
                encoder.tag(6);
                primitive_class_digest.encode_canonical(encoder);
                peer.encode_canonical(encoder);
            }
        }
    }
}

/// Opaque reconstruction edge minted by the V2 verifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedReconstructionDerivationV2 {
    output: ClauseIdV1,
    premises: BTreeSet<ClauseIdV1>,
    proof: VerifiedReconstructionProofV2,
}

impl VerifiedReconstructionDerivationV2 {
    pub fn output(&self) -> &ClauseIdV1 {
        &self.output
    }

    pub fn premises(&self) -> &BTreeSet<ClauseIdV1> {
        &self.premises
    }

    pub fn proof(&self) -> &VerifiedReconstructionProofV2 {
        &self.proof
    }
}

impl CanonicalEncode for VerifiedReconstructionDerivationV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.output.encode_canonical(encoder);
        encode_set(encoder, &self.premises);
        self.proof.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClauseDispositionCertificateV1 {
    pub clause: ClauseIdV1,
    pub disposition: ClauseCostDispositionV1,
    pub closure_round: u32,
    pub reconstructions: Vec<ReconstructionDerivationV1>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ClauseDispositionCertificateV2 {
    clause: ClauseIdV1,
    disposition: ClauseCostDispositionV2,
    closure_round: u32,
    reconstructions: Vec<VerifiedReconstructionDerivationV2>,
}

/// A V2 cost disposition.
///
/// The ambient-first-export case is deliberately class-valued.  Simultaneous
/// direct exports are interchangeable representatives of one Q2 class, so a
/// certificate must not turn a class digest into a caller-controlled
/// [`ClauseIdV1`] and thereby select one declaration as the "original".
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ClauseCostDispositionV2 {
    FirstIrreducible,
    TransparentAlias { target: GlobalId },
    ForcedDefinitionalCompletion,
    ForcedProjection,
    DuplicatePresentation { original: ClauseIdV1 },
    AmbientFirstExportClassMember { primitive_class_digest: Digest },
    AmbientPriorPublicReexport { predecessor_class_digest: Digest },
}

impl ClauseCostDispositionV2 {
    /// Convert dispositions shared with V1; class-valued V2 dispositions have
    /// no representative-selecting V1 equivalent.
    pub fn legacy_v1(&self) -> Option<ClauseCostDispositionV1> {
        match self {
            Self::FirstIrreducible => Some(ClauseCostDispositionV1::FirstIrreducible),
            Self::TransparentAlias { target } => Some(ClauseCostDispositionV1::TransparentAlias {
                target: target.clone(),
            }),
            Self::ForcedDefinitionalCompletion => {
                Some(ClauseCostDispositionV1::ForcedDefinitionalCompletion)
            }
            Self::ForcedProjection => Some(ClauseCostDispositionV1::ForcedProjection),
            Self::DuplicatePresentation { original } => {
                Some(ClauseCostDispositionV1::DuplicatePresentation {
                    original: original.clone(),
                })
            }
            Self::AmbientFirstExportClassMember { .. }
            | Self::AmbientPriorPublicReexport { .. } => None,
        }
    }
}

impl From<ClauseCostDispositionV1> for ClauseCostDispositionV2 {
    fn from(disposition: ClauseCostDispositionV1) -> Self {
        match disposition {
            ClauseCostDispositionV1::FirstIrreducible => Self::FirstIrreducible,
            ClauseCostDispositionV1::TransparentAlias { target } => {
                Self::TransparentAlias { target }
            }
            ClauseCostDispositionV1::ForcedDefinitionalCompletion => {
                Self::ForcedDefinitionalCompletion
            }
            ClauseCostDispositionV1::ForcedProjection => Self::ForcedProjection,
            ClauseCostDispositionV1::DuplicatePresentation { original } => {
                Self::DuplicatePresentation { original }
            }
        }
    }
}

impl CanonicalEncode for ClauseCostDispositionV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::FirstIrreducible => encoder.tag(0),
            Self::TransparentAlias { target } => {
                encoder.tag(1);
                target.encode_canonical(encoder);
            }
            Self::ForcedDefinitionalCompletion => encoder.tag(2),
            Self::ForcedProjection => encoder.tag(3),
            Self::DuplicatePresentation { original } => {
                encoder.tag(4);
                original.encode_canonical(encoder);
            }
            Self::AmbientFirstExportClassMember {
                primitive_class_digest,
            } => {
                encoder.tag(5);
                primitive_class_digest.encode_canonical(encoder);
            }
            Self::AmbientPriorPublicReexport {
                predecessor_class_digest,
            } => {
                encoder.tag(6);
                predecessor_class_digest.encode_canonical(encoder);
            }
        }
    }
}

impl ClauseDispositionCertificateV2 {
    pub fn clause(&self) -> &ClauseIdV1 {
        &self.clause
    }

    pub fn disposition(&self) -> &ClauseCostDispositionV2 {
        &self.disposition
    }

    pub fn closure_round(&self) -> u32 {
        self.closure_round
    }

    pub fn reconstructions(&self) -> &[VerifiedReconstructionDerivationV2] {
        &self.reconstructions
    }
}

impl CanonicalEncode for ClauseDispositionCertificateV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.clause.encode_canonical(encoder);
        self.disposition.encode_canonical(encoder);
        encoder.u32(self.closure_round);
        encoder.sequence(&self.reconstructions);
    }
}

impl CanonicalEncode for ClauseDispositionCertificateV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.clause.encode_canonical(encoder);
        self.disposition.encode_canonical(encoder);
        encoder.u32(self.closure_round);
        encoder.sequence(&self.reconstructions);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BasisIndependenceCertificateV1 {
    pub basis_representative: BTreeSet<ClauseIdV1>,
    pub removed: ClauseIdV1,
    pub negative_evidence: CompleteNegativeEvidenceV1,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct BasisIndependenceCertificateV2 {
    basis_representative: BTreeSet<ClauseIdV1>,
    removed: ClauseIdV1,
    negative_evidence: CompleteNegativeEvidenceV2,
}

impl CanonicalEncode for BasisIndependenceCertificateV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encode_set(encoder, &self.basis_representative);
        self.removed.encode_canonical(encoder);
        self.negative_evidence.encode_canonical(encoder);
    }
}

impl BasisIndependenceCertificateV2 {
    pub fn basis_representative(&self) -> &BTreeSet<ClauseIdV1> {
        &self.basis_representative
    }

    pub fn removed(&self) -> &ClauseIdV1 {
        &self.removed
    }

    pub fn negative_evidence(&self) -> &CompleteNegativeEvidenceV2 {
        &self.negative_evidence
    }
}

impl CanonicalEncode for BasisIndependenceCertificateV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encode_set(encoder, &self.basis_representative);
        self.removed.encode_canonical(encoder);
        self.negative_evidence.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DependencyComponentV1 {
    pub members: BTreeSet<ClauseIdV1>,
}

impl CanonicalEncode for DependencyComponentV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encode_set(encoder, &self.members);
    }
}

/// Exact successful output of the finite cost audit.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CostAuditCertificateV1 {
    pub manifest_digest: Digest,
    pub exact_api_digest: Digest,
    pub kernel_cost: u16,
    pub basis_classes: Vec<BTreeSet<ClauseIdV1>>,
    pub basis_representatives: Vec<BTreeSet<ClauseIdV1>>,
    pub dispositions: Vec<ClauseDispositionCertificateV1>,
    pub independence: Vec<BasisIndependenceCertificateV1>,
    pub dependency_components: Vec<DependencyComponentV1>,
    pub tested_basis_subsets: u32,
}

impl CanonicalEncode for CostAuditCertificateV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        self.exact_api_digest.encode_canonical(encoder);
        encoder.u16(self.kernel_cost);
        encode_set_sequence(encoder, &self.basis_classes);
        encode_set_sequence(encoder, &self.basis_representatives);
        encoder.sequence(&self.dispositions);
        encoder.sequence(&self.independence);
        encoder.sequence(&self.dependency_components);
        encoder.u32(self.tested_basis_subsets);
    }
}

impl CostAuditCertificateV1 {
    pub fn certificate_digest(&self) -> Digest {
        Digest::of_canonical("pen-semantic-audit/kernel-cost-certificate/v1", self)
    }
}

/// Exact successful output of the finite V2 cost audit.
///
/// This remains a generic prototype certificate. Its manifest digest has no
/// live Profile-A authority.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CostAuditCertificateV2 {
    manifest_digest: Digest,
    inventory_digest: Digest,
    exact_api_digest: Digest,
    kernel_cost: u16,
    basis_classes: Vec<BTreeSet<ClauseIdV1>>,
    basis_representatives: Vec<BTreeSet<ClauseIdV1>>,
    dispositions: Vec<ClauseDispositionCertificateV2>,
    independence: Vec<BasisIndependenceCertificateV2>,
    dependency_components: Vec<DependencyComponentV1>,
    tested_basis_subsets: u32,
}

impl CanonicalEncode for CostAuditCertificateV2 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        self.inventory_digest.encode_canonical(encoder);
        self.exact_api_digest.encode_canonical(encoder);
        encoder.u16(self.kernel_cost);
        encode_set_sequence(encoder, &self.basis_classes);
        encode_set_sequence(encoder, &self.basis_representatives);
        encoder.sequence(&self.dispositions);
        encoder.sequence(&self.independence);
        encoder.sequence(&self.dependency_components);
        encoder.u32(self.tested_basis_subsets);
    }
}

impl CostAuditCertificateV2 {
    pub fn certificate_digest(&self) -> Digest {
        Digest::of_canonical("pen-semantic-audit/kernel-cost-certificate/v2", self)
    }

    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn exact_api_digest(&self) -> &Digest {
        &self.exact_api_digest
    }

    pub fn kernel_cost(&self) -> u16 {
        self.kernel_cost
    }

    pub fn basis_classes(&self) -> &[BTreeSet<ClauseIdV1>] {
        &self.basis_classes
    }

    pub fn basis_representatives(&self) -> &[BTreeSet<ClauseIdV1>] {
        &self.basis_representatives
    }

    pub fn dispositions(&self) -> &[ClauseDispositionCertificateV2] {
        &self.dispositions
    }

    pub fn independence(&self) -> &[BasisIndependenceCertificateV2] {
        &self.independence
    }

    pub fn dependency_components(&self) -> &[DependencyComponentV1] {
        &self.dependency_components
    }

    pub fn tested_basis_subsets(&self) -> u32 {
        self.tested_basis_subsets
    }
}

fn encode_set<T: CanonicalEncode>(encoder: &mut CanonicalEncoder, values: &BTreeSet<T>) {
    encoder.u64(values.len() as u64);
    for value in values {
        value.encode_canonical(encoder);
    }
}

fn encode_set_sequence<T: CanonicalEncode>(encoder: &mut CanonicalEncoder, values: &[BTreeSet<T>]) {
    encoder.u64(values.len() as u64);
    for value in values {
        encode_set(encoder, value);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CostFailure {
    Outside(OutsideFragmentReason),
    Unknown(AuditUnknownReason),
}

impl CostFailure {
    fn decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::Outside(reason) => AuditDecision::OutsideFragment(reason),
            Self::Unknown(reason) => AuditDecision::Unknown(reason),
        }
    }
}

type CostResult<T> = Result<T, CostFailure>;

struct PreparedAudit {
    clause_ids: Vec<ClauseIdV1>,
    rules: Vec<ReconstructionDerivationV1>,
    negative: BTreeMap<(ClauseIdV1, BTreeSet<ClauseIdV1>), CompleteNegativeEvidenceV1>,
    class_root: BTreeMap<ClauseIdV1, ClauseIdV1>,
    class_members: BTreeMap<ClauseIdV1, BTreeSet<ClauseIdV1>>,
    exact_api_digest: Digest,
    dependency_components: Vec<DependencyComponentV1>,
}

struct PreparedAuditV2 {
    clause_ids: Vec<ClauseIdV1>,
    rules: Vec<VerifiedReconstructionDerivationV2>,
    negative: BTreeMap<(ClauseIdV1, BTreeSet<ClauseIdV1>), CompleteNegativeEvidenceV2>,
    class_root: BTreeMap<ClauseIdV1, PresentationClassRootV2>,
    class_members: BTreeMap<PresentationClassRootV2, BTreeSet<ClauseIdV1>>,
    inventory_digest: Digest,
    exact_api_digest: Digest,
    dependency_components: Vec<DependencyComponentV1>,
}

#[derive(Clone)]
struct CertifiedBasis {
    seeds: BTreeSet<ClauseIdV1>,
    signature: BTreeSet<ClauseIdV1>,
    independence: Vec<BasisIndependenceCertificateV1>,
}

#[derive(Clone)]
struct CertifiedBasisV2 {
    seeds: BTreeSet<ClauseIdV1>,
    signature: BTreeSet<PresentationClassRootV2>,
    independence: Vec<BasisIndependenceCertificateV2>,
}

#[derive(Clone, Copy)]
enum InventoryBindingV2<'a> {
    Verified(&'a VerifiedPublicAuditInventoryV1),
    #[cfg(test)]
    FixtureOnly,
}

impl InventoryBindingV2<'_> {
    fn digest(&self) -> Digest {
        match self {
            Self::Verified(inventory) => inventory.digest().clone(),
            #[cfg(test)]
            Self::FixtureOnly => Digest::of_domain_bytes(
                "pen-semantic-audit/kernel-cost-v2-fixture-only",
                b"no-authority",
            ),
        }
    }

    fn verified(&self) -> Option<&VerifiedPublicAuditInventoryV1> {
        match self {
            Self::Verified(inventory) => Some(*inventory),
            #[cfg(test)]
            Self::FixtureOnly => None,
        }
    }
}

#[derive(Clone, Copy)]
struct CostAuthoritiesV2<'a> {
    semantic_manifest: Option<&'a VerifiedSemanticAuditManifestV1>,
    ambient_export_census: Option<&'a VerifiedAmbientPrimitiveExportCensusV1>,
    ordinary_beta_derivations: &'a [VerifiedOrdinaryBetaDerivationV1],
}

impl CostAuthoritiesV2<'_> {
    fn none() -> Self {
        Self {
            semantic_manifest: None,
            ambient_export_census: None,
            ordinary_beta_derivations: &[],
        }
    }
}

struct PresentationClasses {
    roots: BTreeMap<ClauseIdV1, ClauseIdV1>,
    members: BTreeMap<ClauseIdV1, BTreeSet<ClauseIdV1>>,
}

/// A disjoint V2 quotient identity.
///
/// In particular, `AmbientFirstExport(d)` is never equal to
/// `Clause(ClauseIdV1(d))`, even when a caller deliberately chooses that
/// clause identifier.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PresentationClassRootV2 {
    Clause(ClauseIdV1),
    AmbientFirstExport(Digest),
}

struct PresentationClassesV2 {
    roots: BTreeMap<ClauseIdV1, PresentationClassRootV2>,
    members: BTreeMap<PresentationClassRootV2, BTreeSet<ClauseIdV1>>,
}

#[derive(Clone, Debug)]
struct Closure {
    members: BTreeSet<ClauseIdV1>,
    rounds: BTreeMap<ClauseIdV1, u32>,
}

/// Exhaustively audit the kernel-clause cost under a verified frozen-shape
/// manifest.
///
/// The manifest supplied by [`VerifiedCostManifestV1`] remains a generic
/// candidate manifest: a successful result is a generic certificate and does
/// not acquire live Profile-A authority.
pub fn audit_kernel_cost_v1(
    manifest: &VerifiedCostManifestV1,
    input: &KernelCostAuditInputV1,
) -> AuditDecision<CostAuditCertificateV1> {
    match audit_kernel_cost_inner(manifest, input) {
        Ok(certificate) => AuditDecision::Proven(certificate),
        Err(failure) => failure.decision(),
    }
}

/// Exhaustively audit kernel-clause cost under the proposed V2 profile.
///
/// V2 deliberately does not consult the semantic rewrite-admissibility
/// certificate. A separately sealed equation owned by a bodyless fresh head
/// therefore remains a basis candidate unless an admitted predecessor-public
/// replay or duplicate reconstruction applies. This function has no live
/// Profile-A adapter or authority.
pub fn audit_kernel_cost_v2(
    manifest: &VerifiedCostManifestV2,
    inventory: &VerifiedPublicAuditInventoryV1,
    input: &KernelCostAuditInputV2,
) -> AuditDecision<CostAuditCertificateV2> {
    if manifest.manifest().profile_id != KERNEL_COST_PROFILE_ID_V2 {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }
    match audit_kernel_cost_inner_v2(
        manifest,
        InventoryBindingV2::Verified(inventory),
        CostAuthoritiesV2::none(),
        input,
    ) {
        Ok(certificate) => AuditDecision::Proven(certificate),
        Err(failure) => failure.decision(),
    }
}

/// Audit the projection-free lambda/unit successor with verifier-minted
/// rewrite-system and ordinary-beta authority.
///
/// This successor is generic and unfrozen.  It does not expose Profile A or
/// authorize any live value. Projection-bearing inventories and clauses are
/// positively outside this fragment. Because the rewrite verifier currently
/// fails closed, no caller can yet obtain the capability required here.
pub fn audit_kernel_cost_lambda_unit_v2(
    cost_manifest: &VerifiedCostManifestV2,
    semantic_manifest: &VerifiedSemanticAuditManifestV1,
    inventory: &VerifiedPublicAuditInventoryV1,
    rewrite_system: &VerifiedRewriteSystemV1,
    ordinary_beta_derivations: &[VerifiedOrdinaryBetaDerivationV1],
    input: &KernelCostAuditInputV2,
) -> AuditDecision<CostAuditCertificateV2> {
    if rewrite_system.manifest_digest() != semantic_manifest.candidate_digest()
        || rewrite_system.inventory_digest() != inventory.digest()
        || rewrite_system.signature_digest() != inventory.successor_boundary().digest()
        || rewrite_system.normalizer_protocol_digest() != inventory.normalizer_protocol_digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }
    audit_kernel_cost_lambda_unit_components_v2(
        cost_manifest,
        semantic_manifest,
        inventory,
        ordinary_beta_derivations,
        input,
    )
}

/// Non-authorizing component integration retained for generic unit tests.
///
/// Only the public wrapper above can expose a cost certificate outside this
/// module, and it requires an exact verified rewrite-system capability.
fn audit_kernel_cost_lambda_unit_components_v2(
    cost_manifest: &VerifiedCostManifestV2,
    semantic_manifest: &VerifiedSemanticAuditManifestV1,
    inventory: &VerifiedPublicAuditInventoryV1,
    ordinary_beta_derivations: &[VerifiedOrdinaryBetaDerivationV1],
    input: &KernelCostAuditInputV2,
) -> AuditDecision<CostAuditCertificateV2> {
    if cost_manifest.manifest().profile_id != KERNEL_COST_LAMBDA_UNIT_PROFILE_ID_V2
        || semantic_manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1
        || inventory.manifest_digest() != semantic_manifest.candidate_digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }
    if !inventory.forced_projections().is_empty()
        || inventory.coverage().forced_projection_count() != 0
        || !inventory.forced_projection_origins().is_empty()
        || input.clauses.iter().any(|clause| {
            matches!(
                clause.clause,
                RawPublicClauseKindV1::ForcedProjectionClause { .. }
            )
        })
    {
        return AuditDecision::OutsideFragment(OutsideFragmentReason::DescriptorProjection);
    }
    if let Some(violation) = lambda_unit_cost_syntax_violation(
        inventory,
        input,
        &semantic_manifest.manifest().universe_levels,
    ) {
        return AuditDecision::OutsideFragment(violation.outside_reason());
    }
    let ambient_export_census = match verify_ambient_primitive_export_census_v1(inventory) {
        AuditDecision::Proven(census) => census,
        AuditDecision::OutsideFragment(reason) => {
            return AuditDecision::OutsideFragment(reason);
        }
        AuditDecision::Unknown(reason) => return AuditDecision::Unknown(reason),
    };
    match audit_kernel_cost_inner_v2(
        cost_manifest,
        InventoryBindingV2::Verified(inventory),
        CostAuthoritiesV2 {
            semantic_manifest: Some(semantic_manifest),
            ambient_export_census: Some(&ambient_export_census),
            ordinary_beta_derivations,
        },
        input,
    ) {
        Ok(certificate) => AuditDecision::Proven(certificate),
        Err(failure) => failure.decision(),
    }
}

fn lambda_unit_cost_syntax_violation(
    inventory: &VerifiedPublicAuditInventoryV1,
    input: &KernelCostAuditInputV2,
    universe_levels: &[u16],
) -> Option<LambdaUnitSyntaxViolation> {
    input
        .clauses
        .iter()
        .filter_map(|clause| raw_clause_lambda_unit_syntax_violation(clause, universe_levels))
        .chain(inventory.declarations().iter().filter_map(|declaration| {
            [
                lambda_unit_term_syntax_violation(&declaration.source().ty, universe_levels),
                declaration
                    .source()
                    .body
                    .as_ref()
                    .and_then(|body| lambda_unit_term_syntax_violation(body, universe_levels)),
                lambda_unit_term_syntax_violation(&declaration.normalized().ty, universe_levels),
                declaration
                    .normalized()
                    .body
                    .as_ref()
                    .and_then(|body| lambda_unit_term_syntax_violation(body, universe_levels)),
            ]
            .into_iter()
            .flatten()
            .max()
        }))
        .chain(inventory.equations().iter().filter_map(|equation| {
            [
                lambda_unit_judgment_syntax_violation(equation.source(), universe_levels),
                lambda_unit_judgment_syntax_violation(equation.normalized(), universe_levels),
            ]
            .into_iter()
            .flatten()
            .max()
        }))
        .chain(
            inventory
                .predecessor_demand_contracts()
                .iter()
                .filter_map(|contract| {
                    [
                        lambda_unit_judgment_syntax_violation(
                            contract.source_requirement(),
                            universe_levels,
                        ),
                        lambda_unit_judgment_syntax_violation(
                            contract.normalized_requirement(),
                            universe_levels,
                        ),
                    ]
                    .into_iter()
                    .flatten()
                    .max()
                }),
        )
        .max()
}

fn raw_clause_lambda_unit_syntax_violation(
    clause: &RawPublicClauseV1,
    universe_levels: &[u16],
) -> Option<LambdaUnitSyntaxViolation> {
    match &clause.clause {
        RawPublicClauseKindV1::PublicDeclaration {
            pair,
            equation_free_descriptors,
            ..
        } => declaration_pair_lambda_unit_syntax_violation(pair, universe_levels).max(
            equation_free_descriptors
                .iter()
                .filter_map(lambda_unit_descriptor_syntax_violation)
                .max(),
        ),
        RawPublicClauseKindV1::ForcedProjectionClause { pair, .. } => {
            declaration_pair_lambda_unit_syntax_violation(pair, universe_levels)
        }
        RawPublicClauseKindV1::PublicEquation { pair, .. } => [
            lambda_unit_context_syntax_violation(&pair.source_context, universe_levels),
            lambda_unit_term_syntax_violation(&pair.source_left, universe_levels),
            lambda_unit_term_syntax_violation(&pair.source_right, universe_levels),
            lambda_unit_term_syntax_violation(&pair.source_type, universe_levels),
            lambda_unit_context_syntax_violation(&pair.normalized_context, universe_levels),
            lambda_unit_term_syntax_violation(&pair.normalized_left, universe_levels),
            lambda_unit_term_syntax_violation(&pair.normalized_right, universe_levels),
            lambda_unit_term_syntax_violation(&pair.normalized_type, universe_levels),
        ]
        .into_iter()
        .flatten()
        .max(),
    }
}

fn lambda_unit_descriptor_syntax_violation(
    descriptor: &EquationFreeDescriptorV1,
) -> Option<LambdaUnitSyntaxViolation> {
    match &descriptor.descriptor {
        EquationFreeDescriptorKindV1::OperationRole { .. } => None,
        EquationFreeDescriptorKindV1::RecordField { .. } => {
            Some(LambdaUnitSyntaxViolation::DescriptorProjection)
        }
        EquationFreeDescriptorKindV1::Former { .. }
        | EquationFreeDescriptorKindV1::FreshComputationContract { .. } => {
            Some(LambdaUnitSyntaxViolation::UnsupportedTerm)
        }
    }
}

fn declaration_pair_lambda_unit_syntax_violation(
    pair: &SourceNormalizedDeclarationV1,
    universe_levels: &[u16],
) -> Option<LambdaUnitSyntaxViolation> {
    [
        lambda_unit_context_syntax_violation(&pair.source_context, universe_levels),
        lambda_unit_term_syntax_violation(&pair.source_type, universe_levels),
        pair.source_body
            .as_ref()
            .and_then(|body| lambda_unit_term_syntax_violation(body, universe_levels)),
        lambda_unit_context_syntax_violation(&pair.normalized_context, universe_levels),
        lambda_unit_term_syntax_violation(&pair.normalized_type, universe_levels),
        pair.normalized_body
            .as_ref()
            .and_then(|body| lambda_unit_term_syntax_violation(body, universe_levels)),
    ]
    .into_iter()
    .flatten()
    .max()
}

#[cfg(test)]
fn audit_kernel_cost_v2_fixture(
    manifest: &VerifiedCostManifestV2,
    input: &KernelCostAuditInputV2,
) -> AuditDecision<CostAuditCertificateV2> {
    match audit_kernel_cost_inner_v2(
        manifest,
        InventoryBindingV2::FixtureOnly,
        CostAuthoritiesV2::none(),
        input,
    ) {
        Ok(certificate) => AuditDecision::Proven(certificate),
        Err(failure) => failure.decision(),
    }
}

fn audit_kernel_cost_inner(
    manifest: &VerifiedCostManifestV1,
    input: &KernelCostAuditInputV1,
) -> CostResult<CostAuditCertificateV1> {
    let limits = manifest.manifest();
    if input.clauses.len() > usize::from(limits.maximum_clauses) {
        return Err(CostFailure::Unknown(AuditUnknownReason::ResourceExhausted));
    }

    let subset_count = 1_u32
        .checked_shl(
            u32::try_from(input.clauses.len())
                .map_err(|_| CostFailure::Unknown(AuditUnknownReason::ResourceExhausted))?,
        )
        .ok_or(CostFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    if subset_count > limits.maximum_basis_subsets {
        return Err(CostFailure::Unknown(AuditUnknownReason::ResourceExhausted));
    }

    let prepared = prepare_audit(manifest, input)?;
    let all: BTreeSet<_> = prepared.clause_ids.iter().cloned().collect();
    let mut certified = Vec::new();
    let mut incomplete_candidate = false;
    let mut tested = 0_u32;

    for mask in 0..subset_count {
        tested = tested
            .checked_add(1)
            .ok_or(CostFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
        let seeds = mask_to_set(mask, &prepared.clause_ids);
        let closure = saturate(&seeds, &prepared.rules, limits.maximum_saturation_rounds)?;
        if closure.members != all {
            continue;
        }

        let mut independent = true;
        let mut independence = Vec::new();
        let mut complete_negative = true;
        for removed in &seeds {
            let mut reduced = seeds.clone();
            reduced.remove(removed);
            let reduced_closure =
                saturate(&reduced, &prepared.rules, limits.maximum_saturation_rounds)?;
            if reduced_closure.members == all {
                independent = false;
                break;
            }

            let Some(evidence) = prepared
                .negative
                .get(&(removed.clone(), reduced.clone()))
                .cloned()
            else {
                complete_negative = false;
                continue;
            };
            independence.push(BasisIndependenceCertificateV1 {
                basis_representative: seeds.clone(),
                removed: removed.clone(),
                negative_evidence: evidence,
            });
        }
        if !independent {
            continue;
        }

        let signature = seeds
            .iter()
            .map(|clause| {
                prepared
                    .class_root
                    .get(clause)
                    .cloned()
                    .ok_or(CostFailure::Unknown(AuditUnknownReason::MalformedInput))
            })
            .collect::<CostResult<BTreeSet<_>>>()?;

        if complete_negative {
            certified.push(CertifiedBasis {
                seeds,
                signature,
                independence,
            });
        } else {
            incomplete_candidate = true;
        }
    }

    if incomplete_candidate {
        return Err(CostFailure::Unknown(
            AuditUnknownReason::MissingNegativeEvidence,
        ));
    }
    if certified.is_empty() {
        return Err(CostFailure::Unknown(
            AuditUnknownReason::IncompleteEnumeration,
        ));
    }

    certified.sort_by(|left, right| left.seeds.cmp(&right.seeds));
    let signatures: BTreeSet<_> = certified
        .iter()
        .map(|basis| basis.signature.clone())
        .collect();
    if signatures.len() != 1 {
        return Err(CostFailure::Unknown(AuditUnknownReason::NonUniqueBasis));
    }
    let signature = signatures
        .iter()
        .next()
        .cloned()
        .ok_or(CostFailure::Unknown(
            AuditUnknownReason::IncompleteEnumeration,
        ))?;

    let canonical_seed = signature.clone();
    let closure = saturate(
        &canonical_seed,
        &prepared.rules,
        limits.maximum_saturation_rounds,
    )?;
    if closure.members != all {
        return Err(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient));
    }

    let dispositions = build_dispositions(&prepared, &signature, &closure)?;
    let basis_classes = signature
        .iter()
        .map(|root| {
            prepared
                .class_members
                .get(root)
                .cloned()
                .ok_or(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient))
        })
        .collect::<CostResult<Vec<_>>>()?;
    let basis_representatives = certified
        .iter()
        .map(|basis| basis.seeds.clone())
        .collect::<Vec<_>>();
    let mut independence = certified
        .into_iter()
        .flat_map(|basis| basis.independence)
        .collect::<Vec<_>>();
    independence.sort_by(|left, right| {
        left.basis_representative
            .cmp(&right.basis_representative)
            .then_with(|| left.removed.cmp(&right.removed))
    });

    let kernel_cost = u16::try_from(basis_classes.len())
        .map_err(|_| CostFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    Ok(CostAuditCertificateV1 {
        manifest_digest: manifest.candidate_digest().clone(),
        exact_api_digest: prepared.exact_api_digest,
        kernel_cost,
        basis_classes,
        basis_representatives,
        dispositions,
        independence,
        dependency_components: prepared.dependency_components,
        tested_basis_subsets: tested,
    })
}

fn audit_kernel_cost_inner_v2(
    manifest: &VerifiedCostManifestV2,
    inventory: InventoryBindingV2<'_>,
    authorities: CostAuthoritiesV2<'_>,
    input: &KernelCostAuditInputV2,
) -> CostResult<CostAuditCertificateV2> {
    let limits = manifest.manifest();
    if input.clauses.len() > usize::from(limits.maximum_clauses) {
        return Err(CostFailure::Unknown(AuditUnknownReason::ResourceExhausted));
    }

    let subset_count = 1_u32
        .checked_shl(
            u32::try_from(input.clauses.len())
                .map_err(|_| CostFailure::Unknown(AuditUnknownReason::ResourceExhausted))?,
        )
        .ok_or(CostFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    if subset_count > limits.maximum_basis_subsets {
        return Err(CostFailure::Unknown(AuditUnknownReason::ResourceExhausted));
    }

    let prepared = prepare_audit_v2(manifest, inventory, authorities, input)?;
    let all: BTreeSet<_> = prepared.clause_ids.iter().cloned().collect();
    let mut certified = Vec::new();
    let mut incomplete_candidate = false;
    let mut tested = 0_u32;

    for mask in 0..subset_count {
        tested = tested
            .checked_add(1)
            .ok_or(CostFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
        let seeds = mask_to_set(mask, &prepared.clause_ids);
        let closure = saturate(&seeds, &prepared.rules, limits.maximum_saturation_rounds)?;
        if closure.members != all {
            continue;
        }

        let mut independent = true;
        let mut independence = Vec::new();
        let mut complete_negative = true;
        for removed in &seeds {
            let mut reduced = seeds.clone();
            reduced.remove(removed);
            let reduced_closure =
                saturate(&reduced, &prepared.rules, limits.maximum_saturation_rounds)?;
            if reduced_closure.members == all {
                independent = false;
                break;
            }

            let Some(evidence) = prepared
                .negative
                .get(&(removed.clone(), reduced.clone()))
                .cloned()
            else {
                complete_negative = false;
                continue;
            };
            independence.push(BasisIndependenceCertificateV2 {
                basis_representative: seeds.clone(),
                removed: removed.clone(),
                negative_evidence: evidence,
            });
        }
        if !independent {
            continue;
        }

        let signature = seeds
            .iter()
            .map(|clause| {
                prepared
                    .class_root
                    .get(clause)
                    .cloned()
                    .ok_or(CostFailure::Unknown(AuditUnknownReason::MalformedInput))
            })
            .collect::<CostResult<BTreeSet<_>>>()?;

        if complete_negative {
            certified.push(CertifiedBasisV2 {
                seeds,
                signature,
                independence,
            });
        } else {
            incomplete_candidate = true;
        }
    }

    if incomplete_candidate {
        return Err(CostFailure::Unknown(
            AuditUnknownReason::MissingNegativeEvidence,
        ));
    }
    if certified.is_empty() {
        return Err(CostFailure::Unknown(
            AuditUnknownReason::IncompleteEnumeration,
        ));
    }

    certified.sort_by(|left, right| left.seeds.cmp(&right.seeds));
    let signatures: BTreeSet<_> = certified
        .iter()
        .map(|basis| basis.signature.clone())
        .collect();
    if signatures.len() != 1 {
        return Err(CostFailure::Unknown(AuditUnknownReason::NonUniqueBasis));
    }
    let signature = signatures
        .iter()
        .next()
        .cloned()
        .ok_or(CostFailure::Unknown(
            AuditUnknownReason::IncompleteEnumeration,
        ))?;

    // Replay from actual clause identifiers only. Ordinary quotient roots
    // have their canonical clause as a seed; an ambient first-export class
    // contributes all of its interchangeable direct members. Seeding the
    // whole ambient class is intentionally non-minimal: it is used only to
    // produce a representative-free disposition replay after the minimal
    // bases have already been certified.
    let mut canonical_seed = BTreeSet::new();
    for root in &signature {
        match root {
            PresentationClassRootV2::Clause(clause) => {
                canonical_seed.insert(clause.clone());
            }
            PresentationClassRootV2::AmbientFirstExport(_) => {
                canonical_seed.extend(
                    prepared
                        .class_members
                        .get(root)
                        .ok_or(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient))?
                        .iter()
                        .cloned(),
                );
            }
        }
    }
    let closure = saturate(
        &canonical_seed,
        &prepared.rules,
        limits.maximum_saturation_rounds,
    )?;
    if closure.members != all {
        return Err(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient));
    }

    let dispositions = build_dispositions_v2(&prepared, &signature, &closure)?;
    let basis_classes = signature
        .iter()
        .map(|root| {
            prepared
                .class_members
                .get(root)
                .cloned()
                .ok_or(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient))
        })
        .collect::<CostResult<Vec<_>>>()?;
    let basis_representatives = certified
        .iter()
        .map(|basis| basis.seeds.clone())
        .collect::<Vec<_>>();
    let mut independence = certified
        .into_iter()
        .flat_map(|basis| basis.independence)
        .collect::<Vec<_>>();
    independence.sort_by(|left, right| {
        left.basis_representative
            .cmp(&right.basis_representative)
            .then_with(|| left.removed.cmp(&right.removed))
    });

    let kernel_cost = u16::try_from(basis_classes.len())
        .map_err(|_| CostFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    Ok(CostAuditCertificateV2 {
        manifest_digest: manifest.candidate_digest().clone(),
        inventory_digest: prepared.inventory_digest,
        exact_api_digest: prepared.exact_api_digest,
        kernel_cost,
        basis_classes,
        basis_representatives,
        dispositions,
        independence,
        dependency_components: prepared.dependency_components,
        tested_basis_subsets: tested,
    })
}

fn prepare_audit(
    manifest: &VerifiedCostManifestV1,
    input: &KernelCostAuditInputV1,
) -> CostResult<PreparedAudit> {
    let mut clauses = BTreeMap::new();
    let mut source_identities = BTreeSet::new();
    let mut public_heads = BTreeMap::new();
    for clause in &input.clauses {
        if clauses.insert(clause.id.clone(), clause.clone()).is_some() {
            return malformed();
        }
        let source_identity = validate_raw_clause(clause)?;
        if !source_identities.insert(source_identity) {
            return Err(CostFailure::Unknown(
                AuditUnknownReason::ProvenanceCollision,
            ));
        }
        if let Some(head) = clause_head(clause)
            && public_heads
                .insert(head.clone(), clause.id.clone())
                .is_some()
        {
            return Err(CostFailure::Unknown(
                AuditUnknownReason::ProvenanceCollision,
            ));
        }
    }
    let clause_ids = clauses.keys().cloned().collect::<Vec<_>>();
    for clause in clauses.values() {
        if !clause
            .semantic_dependencies
            .iter()
            .all(|dependency| clauses.contains_key(dependency))
        {
            return malformed();
        }
    }

    let dag = validate_public_dependency_dag(&clauses, &input.public_dependency_dag)?;
    let mut rules = input.reconstructions.clone();
    sort_canonical(&mut rules, "pen-semantic-audit/reconstruction-sort/v1");
    reject_canonical_duplicates(&rules, "pen-semantic-audit/reconstruction-sort/v1")?;
    for rule in &rules {
        validate_reconstruction(rule, &clauses, &public_heads, &dag, &rules, manifest)?;
    }
    require_structural_reconstructions(&clauses, &rules)?;

    let presentation_classes =
        validate_presentation_equivalences(&clauses, &rules, &input.presentation_equivalences)?;

    let mut negative = BTreeMap::new();
    for evidence in &input.negative_evidence {
        validate_negative_evidence(evidence, &clauses, &rules, manifest)?;
        let key = (evidence.target.clone(), evidence.against.clone());
        if negative.insert(key, evidence.clone()).is_some() {
            return malformed();
        }
    }

    let exact_api_digest = exact_api_digest(input);
    let dependency_components = dependency_components(&clauses, &rules);
    Ok(PreparedAudit {
        clause_ids,
        rules,
        negative,
        class_root: presentation_classes.roots,
        class_members: presentation_classes.members,
        exact_api_digest,
        dependency_components,
    })
}

fn prepare_audit_v2(
    manifest: &VerifiedCostManifestV2,
    inventory: InventoryBindingV2<'_>,
    authorities: CostAuthoritiesV2<'_>,
    input: &KernelCostAuditInputV2,
) -> CostResult<PreparedAuditV2> {
    if let Some(verified) = inventory.verified() {
        reject_unrepresented_ordinary_beta_v2(verified, authorities)?;
    }
    let mut clauses = BTreeMap::new();
    let mut public_heads = BTreeMap::new();
    for clause in &input.clauses {
        if clauses.insert(clause.id.clone(), clause.clone()).is_some() {
            return malformed();
        }
        validate_raw_clause_v2(clause)?;
        if let Some(head) = clause_head(clause)
            && public_heads
                .insert(head.clone(), clause.id.clone())
                .is_some()
        {
            return Err(CostFailure::Unknown(
                AuditUnknownReason::ProvenanceCollision,
            ));
        }
    }
    let clause_ids = clauses.keys().cloned().collect::<Vec<_>>();
    for clause in clauses.values() {
        if !clause
            .semantic_dependencies
            .iter()
            .all(|dependency| clauses.contains_key(dependency))
        {
            return malformed();
        }
    }

    let equation_demand_ports = collect_equation_demand_ports_v2(&input.equation_demand_ports)?;
    if let Some(verified) = inventory.verified() {
        verify_cost_inventory_coverage_v2(
            &clauses,
            input,
            &equation_demand_ports,
            verified,
            authorities,
        )?;
    } else {
        validate_public_dependency_dag(&clauses, &input.public_dependency_dag)?;
        validate_fixture_equation_demand_ports_v2(&clauses, &equation_demand_ports)?;
    }

    let mut wire_rules = input.reconstructions.clone();
    sort_canonical(
        &mut wire_rules,
        "pen-semantic-audit/reconstruction-wire-sort/v2",
    );
    reject_canonical_duplicates(
        &wire_rules,
        "pen-semantic-audit/reconstruction-wire-sort/v2",
    )?;
    let mut rules = wire_rules
        .iter()
        .map(|rule| {
            validate_reconstruction_v2(
                rule,
                &clauses,
                &equation_demand_ports,
                inventory.verified(),
                authorities,
                manifest,
            )
        })
        .collect::<CostResult<Vec<_>>>()?;
    if let Some(verified) = inventory.verified() {
        let derived = derive_inventory_reconstructions_v2(
            &clauses,
            &public_heads,
            verified,
            manifest,
            authorities,
        )?;
        rules.extend(derived);
    }
    sort_canonical(
        &mut rules,
        "pen-semantic-audit/verified-reconstruction-sort/v2",
    );
    reject_canonical_duplicates(&rules, "pen-semantic-audit/verified-reconstruction-sort/v2")?;
    require_structural_reconstructions_v2(&clauses, &rules, inventory.verified())?;

    let mut presentation_equivalences = input.presentation_equivalences.clone();
    sort_canonical(
        &mut presentation_equivalences,
        "pen-semantic-audit/presentation-equivalence-sort/v2",
    );
    reject_canonical_duplicates(
        &presentation_equivalences,
        "pen-semantic-audit/presentation-equivalence-sort/v2",
    )?;
    let presentation_classes = validate_presentation_equivalences_v2(
        &clauses,
        &rules,
        &presentation_equivalences,
        &equation_demand_ports,
        inventory.verified(),
        authorities,
    )?;

    let mut negative = BTreeMap::new();
    for evidence in &input.negative_evidence {
        validate_negative_evidence_v2(evidence, &clauses, &rules, manifest)?;
        let key = (evidence.target.clone(), evidence.against.clone());
        if negative.insert(key, evidence.clone()).is_some() {
            return malformed();
        }
    }

    let inventory_digest = inventory.digest();
    let exact_api_digest = exact_api_digest_v2(input, &inventory_digest);
    let dependency_components = dependency_components(&clauses, &rules);
    Ok(PreparedAuditV2 {
        clause_ids,
        rules,
        negative,
        class_root: presentation_classes.roots,
        class_members: presentation_classes.members,
        inventory_digest,
        exact_api_digest,
        dependency_components,
    })
}

fn reject_unrepresented_ordinary_beta_v2(
    inventory: &VerifiedPublicAuditInventoryV1,
    authorities: CostAuthoritiesV2<'_>,
) -> CostResult<()> {
    if !authorities.ordinary_beta_derivations.is_empty() && authorities.semantic_manifest.is_none()
    {
        return Err(CostFailure::Unknown(AuditUnknownReason::ManifestMismatch));
    }
    if let Some(semantic_manifest) = authorities.semantic_manifest
        && (semantic_manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1
            || inventory.manifest_digest() != semantic_manifest.candidate_digest())
    {
        return Err(CostFailure::Unknown(AuditUnknownReason::ManifestMismatch));
    }

    let mut required = BTreeSet::new();
    for equation in inventory
        .equations()
        .iter()
        .filter(|equation| !equation.is_predecessor_public())
    {
        if equation.source() == equation.normalized() {
            continue;
        }
        let Some(owner) = inventory.public_declaration(equation.owner_head()) else {
            return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
        };
        let requires_definition_reduction = owner.normalized().body.as_ref().is_some_and(|body| {
            !matches!(
                body,
                Term::Global { .. } | Term::Sort { .. } | Term::UnitType | Term::Unit
            )
        });
        if requires_definition_reduction {
            required.insert(equation.equation().clone());
            let matches = authorities
                .ordinary_beta_derivations
                .iter()
                .filter(|derivation| {
                    derivation.inventory_digest() == inventory.digest()
                        && derivation.manifest_digest() == inventory.manifest_digest()
                        && derivation.normalizer_protocol_digest()
                            == inventory.normalizer_protocol_digest()
                        && derivation.successor_boundary_digest()
                            == inventory.successor_boundary().digest()
                        && derivation.equation() == equation.equation()
                        && derivation.equation_source_identity() == equation.source_identity()
                        && derivation.owner_head() == equation.owner_head()
                        && derivation.owner_source_identity() == owner.source_identity()
                        && derivation.reconstructed_source() == equation.source()
                        && derivation.replayed_normalized() == equation.normalized()
                })
                .count();
            if matches == 0 {
                return Err(CostFailure::Unknown(
                    AuditUnknownReason::MissingOrdinaryBetaDerivation,
                ));
            }
            if matches != 1 {
                return Err(CostFailure::Unknown(
                    AuditUnknownReason::ProvenanceCollision,
                ));
            }
        }
    }
    let supplied = authorities
        .ordinary_beta_derivations
        .iter()
        .map(|derivation| derivation.equation().clone())
        .collect::<BTreeSet<_>>();
    if supplied.len() != authorities.ordinary_beta_derivations.len() {
        return Err(CostFailure::Unknown(
            AuditUnknownReason::ProvenanceCollision,
        ));
    }
    if supplied != required {
        return Err(CostFailure::Unknown(
            AuditUnknownReason::IncompleteEnumeration,
        ));
    }
    Ok(())
}

fn validate_raw_clause(clause: &RawPublicClauseV1) -> CostResult<Digest> {
    match &clause.clause {
        RawPublicClauseKindV1::PublicDeclaration {
            head,
            pair,
            presentation,
            equation_free_descriptors,
            ..
        } => {
            validate_declaration_pair(pair)?;
            validate_presentation(pair, presentation)?;
            for descriptor in equation_free_descriptors {
                if descriptor.owner != *head || descriptor.source_clause != clause.id {
                    return malformed();
                }
                validate_descriptor(descriptor)?;
            }
            let mut descriptors = equation_free_descriptors.clone();
            sort_canonical(&mut descriptors, "pen-semantic-audit/descriptor-sort/v1");
            reject_canonical_duplicates(&descriptors, "pen-semantic-audit/descriptor-sort/v1")?;
            Ok(pair.source_identity.clone())
        }
        RawPublicClauseKindV1::PublicEquation { pair, .. } => {
            validate_equation_pair(pair)?;
            Ok(pair.source_identity.clone())
        }
        RawPublicClauseKindV1::ForcedProjectionClause { pair, .. } => {
            validate_declaration_pair(pair)?;
            Ok(pair.source_identity.clone())
        }
    }
}

fn validate_raw_clause_v2(clause: &RawPublicClauseV1) -> CostResult<()> {
    match &clause.clause {
        RawPublicClauseKindV1::PublicDeclaration {
            head,
            pair,
            presentation,
            equation_free_descriptors,
            ..
        } => {
            validate_declaration_pair(pair)?;
            match presentation {
                HeadPresentationV1::Opaque if pair.normalized_body.is_none() => {}
                HeadPresentationV1::TransparentDefinition if pair.normalized_body.is_some() => {}
                HeadPresentationV1::TransparentAlias {
                    target,
                    availability,
                } if pair.normalized_body.as_ref()
                    == Some(&Term::Global { id: target.clone() })
                    && !matches!(
                        availability,
                        PublicAvailabilityV1::OutsideFragment | PublicAvailabilityV1::Unknown
                    ) => {}
                HeadPresentationV1::AmbientPrimitiveFirstExport { .. } => {
                    validate_presentation(pair, presentation)?;
                }
                _ => return malformed(),
            }
            for descriptor in equation_free_descriptors {
                if descriptor.owner != *head || descriptor.source_clause != clause.id {
                    return malformed();
                }
                validate_descriptor(descriptor)?;
            }
            let mut descriptors = equation_free_descriptors.clone();
            sort_canonical(&mut descriptors, "pen-semantic-audit/descriptor-sort/v2");
            reject_canonical_duplicates(&descriptors, "pen-semantic-audit/descriptor-sort/v2")?;
        }
        RawPublicClauseKindV1::PublicEquation { pair, .. } => {
            match pair.source_to_normal_derivation {
                SourceToNormalDerivationV1::Reflexivity => validate_equation_pair(pair)?,
                SourceToNormalDerivationV1::VerifierReplayedOrdinaryBeta => {
                    if pair.source_context != pair.normalized_context {
                        return Err(CostFailure::Unknown(
                            AuditUnknownReason::NormalizationFailure,
                        ));
                    }
                }
            }
        }
        RawPublicClauseKindV1::ForcedProjectionClause { pair, .. } => {
            validate_declaration_pair(pair)?;
        }
    }
    Ok(())
}

fn validate_declaration_pair(pair: &SourceNormalizedDeclarationV1) -> CostResult<()> {
    match pair.source_to_normal_derivation {
        SourceToNormalDerivationV1::Reflexivity
            if pair.source_context == pair.normalized_context
                && pair.source_type == pair.normalized_type
                && pair.source_body == pair.normalized_body =>
        {
            Ok(())
        }
        SourceToNormalDerivationV1::Reflexivity => Err(CostFailure::Unknown(
            AuditUnknownReason::NormalizationFailure,
        )),
        SourceToNormalDerivationV1::VerifierReplayedOrdinaryBeta => malformed(),
    }
}

fn validate_equation_pair(pair: &SourceNormalizedEquationV1) -> CostResult<()> {
    match pair.source_to_normal_derivation {
        SourceToNormalDerivationV1::Reflexivity
            if pair.source_context == pair.normalized_context
                && pair.source_left == pair.normalized_left
                && pair.source_right == pair.normalized_right
                && pair.source_type == pair.normalized_type =>
        {
            Ok(())
        }
        SourceToNormalDerivationV1::Reflexivity => Err(CostFailure::Unknown(
            AuditUnknownReason::NormalizationFailure,
        )),
        SourceToNormalDerivationV1::VerifierReplayedOrdinaryBeta => Err(CostFailure::Unknown(
            AuditUnknownReason::MissingOrdinaryBetaDerivation,
        )),
    }
}

fn validate_presentation(
    pair: &SourceNormalizedDeclarationV1,
    presentation: &HeadPresentationV1,
) -> CostResult<()> {
    match presentation {
        HeadPresentationV1::Opaque => {
            if pair.normalized_body.is_some() {
                return malformed();
            }
        }
        HeadPresentationV1::TransparentDefinition => {
            if pair.normalized_body.is_none() {
                return malformed();
            }
        }
        HeadPresentationV1::TransparentAlias {
            target,
            availability,
        } => {
            if pair.normalized_body.as_ref() != Some(&Term::Global { id: target.clone() }) {
                return malformed();
            }
            match availability {
                PublicAvailabilityV1::PredecessorPublicExport {
                    target: public_target,
                }
                | PublicAvailabilityV1::DependencyPriorExport {
                    target: public_target,
                } if public_target != target => return malformed(),
                PublicAvailabilityV1::DerivedFromPublicInterface { public_support, .. }
                    if public_support.is_empty() =>
                {
                    return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
                }
                PublicAvailabilityV1::OutsideFragment => {
                    return Err(CostFailure::Outside(OutsideFragmentReason::UnsupportedTerm));
                }
                PublicAvailabilityV1::Unknown => {
                    return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
                }
                _ => {}
            }
        }
        HeadPresentationV1::AmbientPrimitiveFirstExport { primitive } => {
            let expected = match primitive {
                crate::model::AmbientPrimitiveV1::Sort { level } => Term::Sort { level: *level },
                crate::model::AmbientPrimitiveV1::UnitType => Term::UnitType,
                crate::model::AmbientPrimitiveV1::Unit => Term::Unit,
            };
            if pair.normalized_body.as_ref() != Some(&expected) {
                return malformed();
            }
        }
    }
    Ok(())
}

fn validate_descriptor(descriptor: &EquationFreeDescriptorV1) -> CostResult<()> {
    match &descriptor.descriptor {
        EquationFreeDescriptorKindV1::OperationRole { .. } => {}
        EquationFreeDescriptorKindV1::Former {
            former,
            constructor,
            ..
        } => {
            if !descriptor.public_support.contains(former)
                || constructor
                    .as_ref()
                    .is_some_and(|value| !descriptor.public_support.contains(value))
            {
                return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
            }
        }
        EquationFreeDescriptorKindV1::RecordField {
            record, field_type, ..
        } => {
            if !descriptor.public_support.contains(record)
                || !descriptor.public_support.contains(field_type)
            {
                return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
            }
        }
        EquationFreeDescriptorKindV1::FreshComputationContract {
            constructor,
            result,
            result_type,
            ..
        } => {
            if !descriptor.public_support.contains(constructor)
                || !descriptor.public_support.contains(result)
                || !descriptor.public_support.contains(result_type)
            {
                return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
            }
        }
    }
    Ok(())
}

fn collect_equation_demand_ports_v2(
    bindings: &[EquationDemandPortBindingV2],
) -> CostResult<BTreeMap<EquationIdV1, Option<DemandPortKeyV1>>> {
    let mut ports = BTreeMap::new();
    for binding in bindings {
        if ports
            .insert(binding.equation.clone(), binding.demand_port.clone())
            .is_some()
        {
            return Err(CostFailure::Unknown(
                AuditUnknownReason::ProvenanceCollision,
            ));
        }
    }
    Ok(ports)
}

fn validate_fixture_equation_demand_ports_v2(
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    equation_demand_ports: &BTreeMap<EquationIdV1, Option<DemandPortKeyV1>>,
) -> CostResult<()> {
    let equations = clauses
        .values()
        .filter_map(|clause| match &clause.clause {
            RawPublicClauseKindV1::PublicEquation {
                equation,
                demand_output,
                ..
            } => Some((equation, demand_output)),
            _ => None,
        })
        .collect::<BTreeMap<_, _>>();
    for (equation, demand_port) in equation_demand_ports {
        let demand_output = equations
            .get(equation)
            .ok_or(CostFailure::Unknown(AuditUnknownReason::MalformedInput))?;
        if demand_port.as_ref().map(|port| &port.output) != demand_output.as_ref() {
            return malformed();
        }
    }
    Ok(())
}

fn verify_cost_inventory_coverage_v2(
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    input: &KernelCostAuditInputV2,
    equation_demand_ports: &BTreeMap<EquationIdV1, Option<DemandPortKeyV1>>,
    inventory: &VerifiedPublicAuditInventoryV1,
    authorities: CostAuthoritiesV2<'_>,
) -> CostResult<()> {
    let expected_declarations = inventory
        .exact_extension()
        .new_declarations()
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let expected_equations = inventory
        .equations()
        .iter()
        .filter(|equation| !equation.is_predecessor_public())
        .map(|equation| equation.equation().clone())
        .collect::<BTreeSet<_>>();
    let expected_forced_projections = inventory
        .forced_projections()
        .iter()
        .filter(|projection| expected_declarations.contains(projection.projection()))
        .map(|projection| projection.projection().clone())
        .collect::<BTreeSet<_>>();
    if !expected_forced_projections.is_empty() {
        return Err(CostFailure::Unknown(
            AuditUnknownReason::MissingDescriptorProjectionInventory,
        ));
    }
    if equation_demand_ports
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>()
        != expected_equations
    {
        return Err(CostFailure::Unknown(
            AuditUnknownReason::IncompleteEnumeration,
        ));
    }

    let mut seen_declarations = BTreeSet::new();
    let mut seen_equations = BTreeSet::new();
    let mut declaration_clause = BTreeMap::new();
    let mut equation_clause = BTreeMap::new();

    for clause in clauses.values() {
        match &clause.clause {
            RawPublicClauseKindV1::PublicDeclaration {
                head,
                pair,
                presentation,
                public_group,
                equation_free_descriptors,
            } => {
                if !equation_free_descriptors.is_empty() {
                    return Err(CostFailure::Unknown(
                        AuditUnknownReason::MissingDescriptorProjectionInventory,
                    ));
                }
                if !seen_declarations.insert(head.clone())
                    || declaration_clause
                        .insert(head.clone(), clause.id.clone())
                        .is_some()
                {
                    return Err(CostFailure::Unknown(
                        AuditUnknownReason::ProvenanceCollision,
                    ));
                }
                let verified = inventory
                    .public_declaration(head)
                    .ok_or(CostFailure::Unknown(
                        AuditUnknownReason::IncompleteEnumeration,
                    ))?;
                let expected_presentation =
                    inventory_presentation_v2(head, verified, inventory, authorities)?;
                if verified.group() != public_group
                    || presentation != &expected_presentation
                    || !declaration_pair_matches_inventory(pair, verified)
                {
                    return Err(CostFailure::Unknown(
                        AuditUnknownReason::NormalizationFailure,
                    ));
                }
            }
            RawPublicClauseKindV1::ForcedProjectionClause { .. } => {
                // The inventory currently proves census, type, origin, and
                // owner precedence only. It does not yet contain exhaustive
                // descriptor/reduction authority, so public cost V2 must not
                // mint a free projection from this wire clause.
                return Err(CostFailure::Unknown(
                    AuditUnknownReason::MissingDescriptorProjectionInventory,
                ));
            }
            RawPublicClauseKindV1::PublicEquation {
                equation,
                owner_head,
                demand_output,
                pair,
            } => {
                if !seen_equations.insert(equation.clone())
                    || equation_clause
                        .insert(equation.clone(), clause.id.clone())
                        .is_some()
                {
                    return Err(CostFailure::Unknown(
                        AuditUnknownReason::ProvenanceCollision,
                    ));
                }
                let verified = inventory
                    .equations()
                    .iter()
                    .find(|candidate| candidate.equation() == equation)
                    .ok_or(CostFailure::Unknown(
                        AuditUnknownReason::IncompleteEnumeration,
                    ))?;
                if verified.is_predecessor_public()
                    || verified.owner_head() != owner_head
                    || equation_demand_ports.get(equation) != Some(&verified.demand_port().cloned())
                    || verified.demand_port().map(|port| &port.output) != demand_output.as_ref()
                    || !equation_pair_matches_inventory(pair, verified)
                {
                    return Err(CostFailure::Unknown(
                        AuditUnknownReason::NormalizationFailure,
                    ));
                }
            }
        }
    }

    if seen_declarations != expected_declarations || seen_equations != expected_equations {
        return Err(CostFailure::Unknown(
            AuditUnknownReason::IncompleteEnumeration,
        ));
    }

    let mut expected_edges = BTreeSet::new();
    let mut expected_dependencies = clauses
        .keys()
        .cloned()
        .map(|clause| (clause, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for dependency in inventory.dependency_dag().edges() {
        let dependent = match &dependency.dependent {
            PublicSubjectV1::Declaration { declaration } => declaration_clause.get(declaration),
            PublicSubjectV1::Equation { equation } => equation_clause.get(equation),
            PublicSubjectV1::DemandContract { .. } => None,
        };
        let prerequisite = declaration_clause.get(&dependency.prerequisite);
        if let (Some(dependent), Some(prerequisite)) = (dependent, prerequisite) {
            let edge = PublicDependencyEdgeV1 {
                dependent: dependent.clone(),
                prerequisite: prerequisite.clone(),
            };
            expected_dependencies
                .get_mut(dependent)
                .ok_or(CostFailure::Unknown(
                    AuditUnknownReason::IncompleteEnumeration,
                ))?
                .insert(prerequisite.clone());
            expected_edges.insert(edge);
        }
    }
    let supplied_edges = input
        .public_dependency_dag
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if supplied_edges.len() != input.public_dependency_dag.len() || supplied_edges != expected_edges
    {
        return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
    }
    for clause in clauses.values() {
        if expected_dependencies.get(&clause.id) != Some(&clause.semantic_dependencies) {
            return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
        }
    }
    Ok(())
}

fn inventory_presentation_v2(
    head: &GlobalId,
    declaration: &VerifiedPublicDeclarationV1,
    inventory: &VerifiedPublicAuditInventoryV1,
    authorities: CostAuthoritiesV2<'_>,
) -> CostResult<HeadPresentationV1> {
    let presentation = match declaration.normalized().body.as_ref() {
        None => HeadPresentationV1::Opaque,
        Some(Term::Global { id: target }) => {
            let availability = inventory
                .declaration_availability(head, target)
                .cloned()
                .ok_or(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport))?;
            HeadPresentationV1::TransparentAlias {
                target: target.clone(),
                availability,
            }
        }
        Some(Term::Sort { .. } | Term::UnitType | Term::Unit) => {
            let census = authorities
                .ambient_export_census
                .ok_or(CostFailure::Unknown(
                    AuditUnknownReason::MissingAmbientFirstExportInventory,
                ))?;
            if census.inventory_digest() != inventory.digest()
                || census.predecessor_boundary_digest() != inventory.predecessor_boundary().digest()
                || census.successor_boundary_digest() != inventory.successor_boundary().digest()
            {
                return Err(CostFailure::Unknown(AuditUnknownReason::ManifestMismatch));
            }
            let class =
                census
                    .class_for_successor_declaration(head)
                    .ok_or(CostFailure::Unknown(
                        AuditUnknownReason::MissingAmbientFirstExportInventory,
                    ))?;
            let member = class
                .successor_new_member(head)
                .ok_or(CostFailure::Unknown(
                    AuditUnknownReason::MissingAmbientFirstExportInventory,
                ))?;
            match member.presentation() {
                AmbientPrimitiveExportMemberKindV1::TransparentAlias {
                    immediate_target, ..
                } => {
                    let availability = inventory
                        .declaration_availability(head, immediate_target)
                        .cloned()
                        .ok_or(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport))?;
                    HeadPresentationV1::TransparentAlias {
                        target: immediate_target.clone(),
                        availability,
                    }
                }
                AmbientPrimitiveExportMemberKindV1::Direct => match class.disposition() {
                    AmbientExportDispositionV1::FirstPublicExportClass => {
                        HeadPresentationV1::AmbientPrimitiveFirstExport {
                            primitive: class.primitive().clone(),
                        }
                    }
                    AmbientExportDispositionV1::PriorPublicReexport { .. } => {
                        HeadPresentationV1::TransparentDefinition
                    }
                },
            }
        }
        Some(_) => HeadPresentationV1::TransparentDefinition,
    };
    Ok(presentation)
}

fn declaration_pair_matches_inventory(
    pair: &SourceNormalizedDeclarationV1,
    verified: &VerifiedPublicDeclarationV1,
) -> bool {
    pair.source_identity == *verified.source_identity()
        && pair.source_context.0.is_empty()
        && pair.normalized_context.0.is_empty()
        && pair.source_type == verified.source().ty
        && pair.source_body == verified.source().body
        && pair.normalized_type == verified.normalized().ty
        && pair.normalized_body == verified.normalized().body
}

fn equation_pair_matches_inventory(
    pair: &SourceNormalizedEquationV1,
    verified: &VerifiedPublicEquationV1,
) -> bool {
    pair.source_identity == *verified.source_identity()
        && source_equation(pair) == *verified.source()
        && normalized_equation(pair) == *verified.normalized()
}

fn source_equation(pair: &SourceNormalizedEquationV1) -> GenericJudgmentV1 {
    GenericJudgmentV1::Equation {
        context: pair.source_context.clone(),
        left: pair.source_left.clone(),
        right: pair.source_right.clone(),
        ty: pair.source_type.clone(),
    }
}

fn normalized_equation(pair: &SourceNormalizedEquationV1) -> GenericJudgmentV1 {
    GenericJudgmentV1::Equation {
        context: pair.normalized_context.clone(),
        left: pair.normalized_left.clone(),
        right: pair.normalized_right.clone(),
        ty: pair.normalized_type.clone(),
    }
}

fn validate_public_dependency_dag(
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    edges: &[PublicDependencyEdgeV1],
) -> CostResult<BTreeMap<ClauseIdV1, BTreeSet<ClauseIdV1>>> {
    let mut dag = BTreeMap::<ClauseIdV1, BTreeSet<ClauseIdV1>>::new();
    for clause in clauses.keys() {
        dag.insert(clause.clone(), BTreeSet::new());
    }
    for edge in edges {
        if edge.dependent == edge.prerequisite
            || !clauses.contains_key(&edge.dependent)
            || !clauses.contains_key(&edge.prerequisite)
        {
            return malformed();
        }
        if !dag
            .get_mut(&edge.dependent)
            .ok_or(CostFailure::Unknown(AuditUnknownReason::MalformedInput))?
            .insert(edge.prerequisite.clone())
        {
            return malformed();
        }
    }

    fn visit(
        node: &ClauseIdV1,
        dag: &BTreeMap<ClauseIdV1, BTreeSet<ClauseIdV1>>,
        temporary: &mut BTreeSet<ClauseIdV1>,
        permanent: &mut BTreeSet<ClauseIdV1>,
    ) -> bool {
        if permanent.contains(node) {
            return true;
        }
        if !temporary.insert(node.clone()) {
            return false;
        }
        if dag.get(node).is_some_and(|dependencies| {
            dependencies
                .iter()
                .any(|dependency| !visit(dependency, dag, temporary, permanent))
        }) {
            return false;
        }
        temporary.remove(node);
        permanent.insert(node.clone());
        true
    }

    let mut temporary = BTreeSet::new();
    let mut permanent = BTreeSet::new();
    for clause in clauses.keys() {
        if !visit(clause, &dag, &mut temporary, &mut permanent) {
            return malformed();
        }
    }
    Ok(dag)
}

fn dag_reaches(
    dag: &BTreeMap<ClauseIdV1, BTreeSet<ClauseIdV1>>,
    dependent: &ClauseIdV1,
    prerequisite: &ClauseIdV1,
) -> bool {
    let mut pending = vec![dependent.clone()];
    let mut seen = BTreeSet::new();
    while let Some(current) = pending.pop() {
        if !seen.insert(current.clone()) {
            continue;
        }
        let Some(next) = dag.get(&current) else {
            continue;
        };
        if next.contains(prerequisite) {
            return true;
        }
        pending.extend(next.iter().cloned());
    }
    false
}

fn validate_reconstruction(
    reconstruction: &ReconstructionDerivationV1,
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    public_heads: &BTreeMap<GlobalId, ClauseIdV1>,
    dag: &BTreeMap<ClauseIdV1, BTreeSet<ClauseIdV1>>,
    all_rules: &[ReconstructionDerivationV1],
    manifest: &VerifiedCostManifestV1,
) -> CostResult<()> {
    if !clauses.contains_key(&reconstruction.output)
        || !reconstruction
            .premises
            .iter()
            .all(|premise| clauses.contains_key(premise))
        || !manifest
            .manifest()
            .free_completion_rules
            .contains(&reconstruction.proof.rule())
    {
        return malformed();
    }

    let expected = match &reconstruction.proof {
        ReconstructionProofV1::OrdinaryBeta { definition } => {
            validate_ordinary_beta(&reconstruction.output, definition, clauses)?;
            singleton(definition.clone())
        }
        ReconstructionProofV1::PriorPublicTransparentAlias {
            declaration,
            target,
        } => {
            if declaration != &reconstruction.output {
                return malformed();
            }
            validate_alias(declaration, target, clauses, public_heads, dag)?
        }
        ReconstructionProofV1::DescriptorForcedProjection {
            descriptor_owner,
            field_ordinal,
        } => {
            validate_projection(
                &reconstruction.output,
                descriptor_owner,
                *field_ordinal,
                clauses,
            )?;
            singleton(descriptor_owner.clone())
        }
        ReconstructionProofV1::CertifiedFreshConstructorComputation { theorem } => {
            validate_fresh_completion(&reconstruction.output, theorem, clauses, all_rules)?;
            singleton(theorem.head.clone())
        }
        ReconstructionProofV1::DuplicatePresentationDeletion { original } => {
            let output = clauses
                .get(&reconstruction.output)
                .ok_or(CostFailure::Unknown(AuditUnknownReason::MalformedInput))?;
            let original_clause = clauses
                .get(original)
                .ok_or(CostFailure::Unknown(AuditUnknownReason::MalformedInput))?;
            if reconstruction.output == *original
                || !duplicate_presentation(output, original_clause)
            {
                return malformed();
            }
            singleton(original.clone())
        }
    };
    if reconstruction.premises != expected {
        return malformed();
    }
    Ok(())
}

fn validate_reconstruction_v2(
    reconstruction: &ReconstructionDerivationV2,
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    equation_demand_ports: &BTreeMap<EquationIdV1, Option<DemandPortKeyV1>>,
    inventory: Option<&VerifiedPublicAuditInventoryV1>,
    authorities: CostAuthoritiesV2<'_>,
    manifest: &VerifiedCostManifestV2,
) -> CostResult<VerifiedReconstructionDerivationV2> {
    if !clauses.contains_key(&reconstruction.output)
        || !reconstruction
            .premises
            .iter()
            .all(|premise| clauses.contains_key(premise))
        || !manifest
            .manifest()
            .free_completion_rules
            .contains(&reconstruction.proof.rule())
    {
        return malformed();
    }

    let (expected, proof) = match &reconstruction.proof {
        ReconstructionProofV2::OrdinaryBetaOfBodyfulDefinition { definition } => {
            if inventory.is_none() {
                validate_ordinary_beta(&reconstruction.output, definition, clauses)?;
            }
            let (equation, owner_head) = match &clauses
                .get(&reconstruction.output)
                .ok_or(CostFailure::Unknown(AuditUnknownReason::MalformedInput))?
                .clause
            {
                RawPublicClauseKindV1::PublicEquation {
                    equation,
                    owner_head,
                    ..
                } => (equation.clone(), owner_head.clone()),
                _ => return malformed(),
            };
            let definition_head = match &clauses
                .get(definition)
                .ok_or(CostFailure::Unknown(AuditUnknownReason::MalformedInput))?
                .clause
            {
                RawPublicClauseKindV1::PublicDeclaration { head, .. } => head,
                _ => return malformed(),
            };
            if definition_head != &owner_head {
                return malformed();
            }
            let derivation_digest = if let Some(inventory) = inventory {
                let matches = authorities
                    .ordinary_beta_derivations
                    .iter()
                    .filter(|derivation| {
                        derivation.inventory_digest() == inventory.digest()
                            && derivation.equation() == &equation
                            && derivation.owner_head() == &owner_head
                    })
                    .collect::<Vec<_>>();
                let [derivation] = matches.as_slice() else {
                    return Err(if matches.is_empty() {
                        CostFailure::Unknown(AuditUnknownReason::MissingOrdinaryBetaDerivation)
                    } else {
                        CostFailure::Unknown(AuditUnknownReason::ProvenanceCollision)
                    });
                };
                derivation.derivation_digest().clone()
            } else {
                Digest::of_domain_bytes(
                    "pen-semantic-audit/kernel-cost-v2-fixture-ordinary-beta",
                    b"fixture-only-no-authority",
                )
            };
            (
                singleton(definition.clone()),
                VerifiedReconstructionProofV2::OrdinaryBetaOfVerifiedDeclaration {
                    owner_head,
                    derivation_digest,
                },
            )
        }
        ReconstructionProofV2::DescriptorForcedProjection {
            descriptor_owner,
            field_ordinal,
        } => {
            let inventory = inventory.ok_or(CostFailure::Unknown(
                AuditUnknownReason::MissingVerifiedPublicInventory,
            ))?;
            let projection_census_digest = validate_projection_v2(
                &reconstruction.output,
                descriptor_owner,
                *field_ordinal,
                clauses,
                inventory,
            )?;
            (
                singleton(descriptor_owner.clone()),
                VerifiedReconstructionProofV2::DescriptorForcedProjection {
                    descriptor_owner: descriptor_owner.clone(),
                    field_ordinal: *field_ordinal,
                    projection_census_digest,
                },
            )
        }
        ReconstructionProofV2::DuplicatePresentationDeletion { original } => {
            let output = clauses
                .get(&reconstruction.output)
                .ok_or(CostFailure::Unknown(AuditUnknownReason::MalformedInput))?;
            let original_clause = clauses
                .get(original)
                .ok_or(CostFailure::Unknown(AuditUnknownReason::MalformedInput))?;
            if reconstruction.output == *original
                || !duplicate_presentation_v2(
                    output,
                    original_clause,
                    clauses,
                    equation_demand_ports,
                    inventory,
                )?
            {
                return malformed();
            }
            (
                singleton(original.clone()),
                VerifiedReconstructionProofV2::DuplicatePresentationDeletion {
                    original: original.clone(),
                },
            )
        }
    };
    if reconstruction.premises != expected {
        return malformed();
    }
    Ok(VerifiedReconstructionDerivationV2 {
        output: reconstruction.output.clone(),
        premises: expected,
        proof,
    })
}

fn derive_inventory_reconstructions_v2(
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    public_heads: &BTreeMap<GlobalId, ClauseIdV1>,
    inventory: &VerifiedPublicAuditInventoryV1,
    manifest: &VerifiedCostManifestV2,
    authorities: CostAuthoritiesV2<'_>,
) -> CostResult<Vec<VerifiedReconstructionDerivationV2>> {
    let mut derived = Vec::new();
    for clause in clauses.values() {
        if let (Some(census), RawPublicClauseKindV1::PublicDeclaration { head, .. }) =
            (authorities.ambient_export_census, &clause.clause)
            && let Some(class) = census.class_for_successor_declaration(head)
            && let Some(member) = class.successor_new_member(head)
            && member.presentation().is_direct()
            && let AmbientExportDispositionV1::PriorPublicReexport {
                predecessor_q2_class_digest,
            } = class.disposition()
        {
            if !manifest
                .manifest()
                .free_completion_rules
                .contains(&FreeCompletionRuleV2::DuplicatePresentationDeletion)
            {
                return malformed();
            }
            derived.push(VerifiedReconstructionDerivationV2 {
                output: clause.id.clone(),
                premises: BTreeSet::new(),
                proof: VerifiedReconstructionProofV2::PriorPublicAmbientReexport {
                    declaration: clause.id.clone(),
                    primitive_class_digest: class.digest().clone(),
                    predecessor_class_digest: predecessor_q2_class_digest.clone(),
                },
            });
        }

        match &clause.clause {
            RawPublicClauseKindV1::PublicDeclaration {
                head,
                presentation:
                    HeadPresentationV1::TransparentAlias {
                        target,
                        availability,
                    },
                ..
            } => {
                if !manifest
                    .manifest()
                    .free_completion_rules
                    .contains(&FreeCompletionRuleV2::PriorPublicTransparentAlias)
                {
                    return malformed();
                }
                let replayed = inventory
                    .declaration_availability(head, target)
                    .ok_or(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport))?;
                if replayed != availability {
                    return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
                }
                let premises = match replayed {
                    PublicAvailabilityV1::PredecessorPublicExport {
                        target: replayed_target,
                    } if replayed_target == target
                        && inventory.contains_predecessor_declaration(target) =>
                    {
                        BTreeSet::new()
                    }
                    PublicAvailabilityV1::DependencyPriorExport {
                        target: replayed_target,
                    } if replayed_target == target => {
                        let prerequisite = public_heads
                            .get(target)
                            .ok_or(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport))?;
                        singleton(prerequisite.clone())
                    }
                    _ => {
                        return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
                    }
                };
                derived.push(VerifiedReconstructionDerivationV2 {
                    output: clause.id.clone(),
                    premises,
                    proof: VerifiedReconstructionProofV2::PriorPublicTransparentAlias {
                        declaration: clause.id.clone(),
                        target: target.clone(),
                    },
                });
            }
            RawPublicClauseKindV1::PublicEquation {
                owner_head, pair, ..
            } => {
                let normalized = normalized_equation(pair);
                if let Some(predecessor_equation) =
                    inventory.predecessor_public_equation_id(&normalized)
                {
                    if !manifest
                        .manifest()
                        .free_completion_rules
                        .contains(&FreeCompletionRuleV2::PriorPublicTransparentAlias)
                    {
                        return malformed();
                    }
                    derived.push(VerifiedReconstructionDerivationV2 {
                        output: clause.id.clone(),
                        premises: BTreeSet::new(),
                        proof: VerifiedReconstructionProofV2::PriorPublicEquationReplay {
                            predecessor_equation: predecessor_equation.clone(),
                            owner_head: owner_head.clone(),
                        },
                    });
                }
            }
            _ => {}
        }
    }

    if let Some(census) = authorities.ambient_export_census {
        for class in census.classes() {
            if !matches!(
                class.disposition(),
                AmbientExportDispositionV1::FirstPublicExportClass
            ) {
                continue;
            }
            let members = class
                .successor_new_direct_members()
                .map(|member| {
                    public_heads
                        .get(member.declaration())
                        .cloned()
                        .ok_or(CostFailure::Unknown(
                            AuditUnknownReason::IncompleteEnumeration,
                        ))
                })
                .collect::<CostResult<Vec<_>>>()?;
            if members.len() > 1
                && !manifest
                    .manifest()
                    .free_completion_rules
                    .contains(&FreeCompletionRuleV2::DuplicatePresentationDeletion)
            {
                return malformed();
            }
            for output in &members {
                for peer in &members {
                    if output == peer {
                        continue;
                    }
                    derived.push(VerifiedReconstructionDerivationV2 {
                        output: output.clone(),
                        premises: singleton(peer.clone()),
                        proof: VerifiedReconstructionProofV2::AmbientFirstExportClassDuplicate {
                            primitive_class_digest: class.q2_duplicate_class_digest().clone(),
                            peer: peer.clone(),
                        },
                    });
                }
            }
        }
    }
    Ok(derived)
}

fn validate_ordinary_beta(
    output: &ClauseIdV1,
    definition: &ClauseIdV1,
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
) -> CostResult<()> {
    let Some(RawPublicClauseV1 {
        clause:
            RawPublicClauseKindV1::PublicDeclaration {
                head,
                pair: definition_pair,
                presentation: HeadPresentationV1::TransparentDefinition,
                ..
            },
        ..
    }) = clauses.get(definition)
    else {
        return malformed();
    };
    let Some(body) = &definition_pair.normalized_body else {
        return malformed();
    };
    let Some(RawPublicClauseV1 {
        clause:
            RawPublicClauseKindV1::PublicEquation {
                owner_head,
                demand_output,
                pair,
                ..
            },
        ..
    }) = clauses.get(output)
    else {
        return malformed();
    };
    if owner_head != head
        || demand_output.is_some()
        || pair.normalized_context != definition_pair.normalized_context
        || pair.normalized_left != (Term::Global { id: head.clone() })
        || pair.normalized_right != *body
        || pair.normalized_type != definition_pair.normalized_type
    {
        return malformed();
    }
    Ok(())
}

fn validate_alias(
    declaration: &ClauseIdV1,
    target: &GlobalId,
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    public_heads: &BTreeMap<GlobalId, ClauseIdV1>,
    dag: &BTreeMap<ClauseIdV1, BTreeSet<ClauseIdV1>>,
) -> CostResult<BTreeSet<ClauseIdV1>> {
    let Some(RawPublicClauseV1 {
        clause:
            RawPublicClauseKindV1::PublicDeclaration {
                presentation:
                    HeadPresentationV1::TransparentAlias {
                        target: actual_target,
                        availability,
                    },
                ..
            },
        ..
    }) = clauses.get(declaration)
    else {
        return malformed();
    };
    if actual_target != target {
        return malformed();
    }
    match availability {
        PublicAvailabilityV1::PredecessorPublicExport {
            target: public_target,
        } if public_target == target => Ok(BTreeSet::new()),
        PublicAvailabilityV1::DerivedFromPublicInterface { .. } => Ok(BTreeSet::new()),
        PublicAvailabilityV1::DependencyPriorExport {
            target: public_target,
        } if public_target == target => {
            let prerequisite = public_heads
                .get(target)
                .ok_or(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport))?;
            if !dag_reaches(dag, declaration, prerequisite) {
                return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
            }
            Ok(singleton(prerequisite.clone()))
        }
        PublicAvailabilityV1::AmbientOnly => {
            Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport))
        }
        PublicAvailabilityV1::OutsideFragment => {
            Err(CostFailure::Outside(OutsideFragmentReason::UnsupportedTerm))
        }
        PublicAvailabilityV1::Unknown => {
            Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport))
        }
        _ => malformed(),
    }
}

fn validate_projection(
    output: &ClauseIdV1,
    descriptor_owner: &ClauseIdV1,
    field_ordinal: u16,
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
) -> CostResult<()> {
    let Some(RawPublicClauseV1 {
        clause:
            RawPublicClauseKindV1::ForcedProjectionClause {
                record_owner,
                field_ordinal: output_ordinal,
                ..
            },
        ..
    }) = clauses.get(output)
    else {
        return malformed();
    };
    if record_owner != descriptor_owner || *output_ordinal != field_ordinal {
        return malformed();
    }
    let Some(RawPublicClauseV1 {
        clause:
            RawPublicClauseKindV1::PublicDeclaration {
                head,
                equation_free_descriptors,
                ..
            },
        ..
    }) = clauses.get(descriptor_owner)
    else {
        return malformed();
    };
    let matches = equation_free_descriptors.iter().filter(|descriptor| {
        matches!(
            &descriptor.descriptor,
            EquationFreeDescriptorKindV1::RecordField {
                record,
                field_ordinal: ordinal,
                ..
            } if record == head && *ordinal == field_ordinal
        )
    });
    if matches.count() != 1 {
        return malformed();
    }
    Ok(())
}

fn validate_projection_v2(
    output: &ClauseIdV1,
    descriptor_owner: &ClauseIdV1,
    field_ordinal: u16,
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    inventory: &VerifiedPublicAuditInventoryV1,
) -> CostResult<Digest> {
    let Some(RawPublicClauseV1 {
        clause:
            RawPublicClauseKindV1::ForcedProjectionClause {
                projection,
                record_owner,
                field_ordinal: output_ordinal,
                pair,
            },
        ..
    }) = clauses.get(output)
    else {
        return malformed();
    };
    if record_owner != descriptor_owner || *output_ordinal != field_ordinal {
        return malformed();
    }
    let Some(RawPublicClauseV1 {
        clause:
            RawPublicClauseKindV1::PublicDeclaration {
                head,
                pair: owner_pair,
                equation_free_descriptors,
                ..
            },
        ..
    }) = clauses.get(descriptor_owner)
    else {
        return malformed();
    };
    let matching = equation_free_descriptors
        .iter()
        .filter_map(|descriptor| {
            let EquationFreeDescriptorKindV1::RecordField {
                record,
                field_ordinal: ordinal,
                field_type,
            } = &descriptor.descriptor
            else {
                return None;
            };
            (record == head && *ordinal == field_ordinal).then_some(field_type)
        })
        .collect::<Vec<_>>();
    let [field_type] = matching.as_slice() else {
        return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
    };
    let Some(Term::Sigma {
        parameter,
        body: sigma_body,
    }) = owner_pair.normalized_body.as_ref()
    else {
        return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
    };
    let selector = match field_ordinal {
        0 if **parameter
            == (Term::Global {
                id: (*field_type).clone(),
            }) =>
        {
            Term::First {
                pair: Box::new(Term::Var { index: 0 }),
            }
        }
        1 if **sigma_body
            == (Term::Global {
                id: (*field_type).clone(),
            }) =>
        {
            Term::Second {
                pair: Box::new(Term::Var { index: 0 }),
            }
        }
        _ => {
            return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
        }
    };
    let expected_type = Term::Pi {
        parameter: Box::new(Term::Global { id: head.clone() }),
        body: Box::new(Term::Global {
            id: (*field_type).clone(),
        }),
    };
    let expected_body = Some(Term::Lambda {
        parameter_type: Box::new(Term::Global { id: head.clone() }),
        body: Box::new(selector),
    });
    if !pair.source_context.0.is_empty()
        || !pair.normalized_context.0.is_empty()
        || pair.source_type != expected_type
        || pair.normalized_type != expected_type
        || pair.source_body != expected_body
        || pair.normalized_body != expected_body
    {
        return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
    }
    let verified = inventory
        .public_declaration(projection)
        .ok_or(CostFailure::Unknown(
            AuditUnknownReason::IncompleteEnumeration,
        ))?;
    let projection_census = inventory
        .forced_projection(projection)
        .ok_or(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport))?;
    if !inventory.is_successor_new_declaration(projection)
        || !declaration_pair_matches_inventory(pair, verified)
        || projection_census.record_owner() != head
        || projection_census.field_ordinal() != field_ordinal
        || projection_census.origin() != verified.origin()
        || projection_census.normalized_type() != &pair.normalized_type
    {
        return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
    }
    Ok(projection_census.descriptor_digest().clone())
}

fn validate_fresh_completion(
    output: &ClauseIdV1,
    theorem: &ExactFreshCompletionProofV1,
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    all_rules: &[ReconstructionDerivationV1],
) -> CostResult<()> {
    let Some(RawPublicClauseV1 {
        clause:
            RawPublicClauseKindV1::PublicDeclaration {
                head,
                pair: head_pair,
                presentation: HeadPresentationV1::Opaque,
                equation_free_descriptors,
                ..
            },
        ..
    }) = clauses.get(&theorem.head)
    else {
        return malformed();
    };
    if head_pair.normalized_body.is_some() {
        return malformed();
    }
    let mut contracts = equation_free_descriptors.iter().filter_map(|descriptor| {
        let EquationFreeDescriptorKindV1::FreshComputationContract {
            demand_output,
            constructor,
            result,
            result_type,
            computation_mode: ComputationModeV1::ClosedNullary,
        } = &descriptor.descriptor
        else {
            return None;
        };
        (demand_output == &theorem.demand_output).then_some((
            descriptor,
            constructor,
            result,
            result_type,
        ))
    });
    let Some((descriptor, constructor, result, result_type)) = contracts.next() else {
        return malformed();
    };
    if contracts.next().is_some()
        || !descriptor.public_support.contains(constructor)
        || !descriptor.public_support.contains(result)
        || !descriptor.public_support.contains(result_type)
    {
        return malformed();
    }

    let Some(RawPublicClauseV1 {
        clause:
            RawPublicClauseKindV1::PublicEquation {
                owner_head,
                demand_output: Some(output_demand),
                pair,
                ..
            },
        ..
    }) = clauses.get(output)
    else {
        return malformed();
    };
    let expected_left = Term::Apply {
        function: Box::new(Term::Global { id: head.clone() }),
        argument: Box::new(Term::Global {
            id: constructor.clone(),
        }),
    };
    if owner_head != head
        || output_demand != &theorem.demand_output
        || !pair.normalized_context.0.is_empty()
        || pair.normalized_left != expected_left
        || pair.normalized_right != (Term::Global { id: result.clone() })
        || pair.normalized_type
            != (Term::Global {
                id: result_type.clone(),
            })
    {
        return malformed();
    }

    let matching_equations = clauses
        .values()
        .filter(|clause| {
            matches!(
                &clause.clause,
                RawPublicClauseKindV1::PublicEquation {
                    owner_head,
                    demand_output: Some(demand),
                    ..
                } if owner_head == head && demand == &theorem.demand_output
            )
        })
        .count();
    let matching_theorems = all_rules
        .iter()
        .filter(|rule| {
            matches!(
                &rule.proof,
                ReconstructionProofV1::CertifiedFreshConstructorComputation {
                    theorem: candidate
                } if candidate.head == theorem.head
                    && candidate.demand_output == theorem.demand_output
            )
        })
        .count();
    if matching_equations != 1 || matching_theorems != 1 {
        return malformed();
    }
    Err(CostFailure::Unknown(
        AuditUnknownReason::MissingFreeCompletionTheorem,
    ))
}

fn duplicate_presentation(left: &RawPublicClauseV1, right: &RawPublicClauseV1) -> bool {
    match (&left.clause, &right.clause) {
        (
            RawPublicClauseKindV1::PublicDeclaration {
                pair: left_pair,
                presentation: left_presentation,
                public_group: left_group,
                ..
            },
            RawPublicClauseKindV1::PublicDeclaration {
                pair: right_pair,
                presentation: right_presentation,
                public_group: right_group,
                ..
            },
        ) => {
            is_transparent(left_presentation)
                && is_transparent(right_presentation)
                && left_group == right_group
                && left_pair.normalized_context == right_pair.normalized_context
                && left_pair.normalized_type == right_pair.normalized_type
                && left_pair.normalized_body == right_pair.normalized_body
        }
        _ => false,
    }
}

fn duplicate_presentation_v2(
    left: &RawPublicClauseV1,
    right: &RawPublicClauseV1,
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    equation_demand_ports: &BTreeMap<EquationIdV1, Option<DemandPortKeyV1>>,
    inventory: Option<&VerifiedPublicAuditInventoryV1>,
) -> CostResult<bool> {
    let duplicate = match (&left.clause, &right.clause) {
        (
            RawPublicClauseKindV1::PublicDeclaration {
                head: left_head,
                pair: left_pair,
                presentation: left_presentation,
                public_group: left_group,
                equation_free_descriptors: left_descriptors,
            },
            RawPublicClauseKindV1::PublicDeclaration {
                head: right_head,
                pair: right_pair,
                presentation: right_presentation,
                public_group: right_group,
                equation_free_descriptors: right_descriptors,
            },
        ) => {
            let local = is_transparent(left_presentation)
                && is_transparent(right_presentation)
                && left_presentation == right_presentation
                && left_group == right_group
                && left.semantic_dependencies == right.semantic_dependencies
                && declaration_contents_equal(left_pair, right_pair)
                && descriptor_payloads_equal(left_descriptors, right_descriptors);
            if !local {
                false
            } else if let Some(inventory) = inventory {
                let Some(left_verified) = inventory.public_declaration(left_head) else {
                    return Ok(false);
                };
                let Some(right_verified) = inventory.public_declaration(right_head) else {
                    return Ok(false);
                };
                left_verified.origin() == right_verified.origin()
                    && left_verified.group() == right_verified.group()
                    && declaration_pair_matches_inventory(left_pair, left_verified)
                    && declaration_pair_matches_inventory(right_pair, right_verified)
            } else {
                true
            }
        }
        (
            RawPublicClauseKindV1::PublicEquation {
                equation: left_equation,
                owner_head: left_owner,
                demand_output: left_demand,
                pair: left_pair,
            },
            RawPublicClauseKindV1::PublicEquation {
                equation: right_equation,
                owner_head: right_owner,
                demand_output: right_demand,
                pair: right_pair,
            },
        ) => {
            let owner_is_verified = if let Some(inventory) = inventory {
                inventory.public_declaration(left_owner).is_some()
            } else {
                clauses
                    .values()
                    .filter(|clause| {
                        matches!(
                            &clause.clause,
                            RawPublicClauseKindV1::PublicDeclaration { head, .. }
                                if head == left_owner
                        )
                    })
                    .count()
                    == 1
            };
            let bound_ports_match = match (
                equation_demand_ports.get(left_equation),
                equation_demand_ports.get(right_equation),
            ) {
                (Some(left_port), Some(right_port)) => left_port == right_port,
                (None, None) => inventory.is_none(),
                _ => false,
            };
            let local = left_owner == right_owner
                && left_demand == right_demand
                && bound_ports_match
                && left.semantic_dependencies == right.semantic_dependencies
                && equation_contents_equal(left_pair, right_pair)
                && owner_is_verified;
            if !local {
                false
            } else if let Some(inventory) = inventory {
                let left_verified = inventory
                    .equations()
                    .iter()
                    .find(|candidate| candidate.equation() == left_equation);
                let right_verified = inventory
                    .equations()
                    .iter()
                    .find(|candidate| candidate.equation() == right_equation);
                match (left_verified, right_verified) {
                    (Some(left_verified), Some(right_verified)) => {
                        left_verified.owner_head() == right_verified.owner_head()
                            && left_verified.origin() == right_verified.origin()
                            && left_verified.demand_port() == right_verified.demand_port()
                            && left_verified.normalized() == right_verified.normalized()
                            && equation_pair_matches_inventory(left_pair, left_verified)
                            && equation_pair_matches_inventory(right_pair, right_verified)
                    }
                    _ => false,
                }
            } else {
                true
            }
        }
        _ => false,
    };
    Ok(duplicate)
}

fn declaration_contents_equal(
    left: &SourceNormalizedDeclarationV1,
    right: &SourceNormalizedDeclarationV1,
) -> bool {
    left.source_context == right.source_context
        && left.source_type == right.source_type
        && left.source_body == right.source_body
        && left.normalized_context == right.normalized_context
        && left.normalized_type == right.normalized_type
        && left.normalized_body == right.normalized_body
        && left.source_to_normal_derivation == right.source_to_normal_derivation
}

fn equation_contents_equal(
    left: &SourceNormalizedEquationV1,
    right: &SourceNormalizedEquationV1,
) -> bool {
    left.source_identity == right.source_identity
        && left.source_context == right.source_context
        && left.source_left == right.source_left
        && left.source_right == right.source_right
        && left.source_type == right.source_type
        && left.normalized_context == right.normalized_context
        && left.normalized_left == right.normalized_left
        && left.normalized_right == right.normalized_right
        && left.normalized_type == right.normalized_type
        && left.source_to_normal_derivation == right.source_to_normal_derivation
}

fn descriptor_payloads_equal(
    left: &[EquationFreeDescriptorV1],
    right: &[EquationFreeDescriptorV1],
) -> bool {
    if left.len() != right.len() {
        return false;
    }
    let mut left_payloads = left
        .iter()
        .map(|descriptor| (&descriptor.descriptor, &descriptor.public_support))
        .collect::<Vec<_>>();
    let mut right_payloads = right
        .iter()
        .map(|descriptor| (&descriptor.descriptor, &descriptor.public_support))
        .collect::<Vec<_>>();
    left_payloads.sort_by(|left, right| {
        Digest::of_canonical("pen-semantic-audit/descriptor-kind/v2", left.0).cmp(
            &Digest::of_canonical("pen-semantic-audit/descriptor-kind/v2", right.0),
        )
    });
    right_payloads.sort_by(|left, right| {
        Digest::of_canonical("pen-semantic-audit/descriptor-kind/v2", left.0).cmp(
            &Digest::of_canonical("pen-semantic-audit/descriptor-kind/v2", right.0),
        )
    });
    left_payloads == right_payloads
}

fn is_transparent(presentation: &HeadPresentationV1) -> bool {
    matches!(
        presentation,
        HeadPresentationV1::TransparentDefinition | HeadPresentationV1::TransparentAlias { .. }
    )
}

fn require_structural_reconstructions(
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    rules: &[ReconstructionDerivationV1],
) -> CostResult<()> {
    for clause in clauses.values() {
        let required = match &clause.clause {
            RawPublicClauseKindV1::PublicDeclaration {
                presentation: HeadPresentationV1::TransparentAlias { availability, .. },
                ..
            } if availability.is_publicly_supported() => {
                Some(FreeCompletionRuleV1::PriorPublicTransparentAlias)
            }
            RawPublicClauseKindV1::ForcedProjectionClause { .. } => {
                Some(FreeCompletionRuleV1::DescriptorForcedProjection)
            }
            _ => None,
        };
        if let Some(required) = required
            && !rules
                .iter()
                .any(|rule| rule.output == clause.id && rule.proof.rule() == required)
        {
            return Err(CostFailure::Unknown(
                AuditUnknownReason::MissingTupleDisposition,
            ));
        }
    }

    for clause in clauses.values() {
        let Some(required) = required_equation_completion(clause, clauses)? else {
            continue;
        };
        let present = rules.iter().any(|rule| {
            if rule.output != clause.id {
                return false;
            }
            match (&required, &rule.proof) {
                (
                    RequiredEquationCompletion::OrdinaryBeta { definition },
                    ReconstructionProofV1::OrdinaryBeta { definition: actual },
                ) => definition == actual,
                (
                    RequiredEquationCompletion::Fresh {
                        head,
                        demand_output,
                    },
                    ReconstructionProofV1::CertifiedFreshConstructorComputation { theorem },
                ) => theorem.head == *head && theorem.demand_output == *demand_output,
                _ => false,
            }
        });
        if !present {
            return Err(CostFailure::Unknown(
                AuditUnknownReason::MissingTupleDisposition,
            ));
        }
    }
    Ok(())
}

fn require_structural_reconstructions_v2(
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    rules: &[VerifiedReconstructionDerivationV2],
    inventory: Option<&VerifiedPublicAuditInventoryV1>,
) -> CostResult<()> {
    for clause in clauses.values() {
        let present = match &clause.clause {
            RawPublicClauseKindV1::PublicDeclaration {
                presentation: HeadPresentationV1::TransparentAlias { target, .. },
                ..
            } => rules.iter().any(|rule| {
                rule.output == clause.id
                    && matches!(
                        &rule.proof,
                        VerifiedReconstructionProofV2::PriorPublicTransparentAlias {
                            target: actual,
                            ..
                        } if actual == target
                    )
            }),
            RawPublicClauseKindV1::ForcedProjectionClause {
                record_owner,
                field_ordinal,
                ..
            } => rules.iter().any(|rule| {
                rule.output == clause.id
                    && matches!(
                        &rule.proof,
                        VerifiedReconstructionProofV2::DescriptorForcedProjection {
                            descriptor_owner,
                            field_ordinal: actual,
                            ..
                        } if descriptor_owner == record_owner && actual == field_ordinal
                    )
            }),
            _ => true,
        };
        if !present {
            return Err(CostFailure::Unknown(
                AuditUnknownReason::MissingTupleDisposition,
            ));
        }
    }

    if inventory.is_none() {
        for clause in clauses.values() {
            let Some(RequiredEquationCompletion::OrdinaryBeta { .. }) =
                required_equation_completion(clause, clauses)?
            else {
                continue;
            };
            let RawPublicClauseKindV1::PublicEquation { owner_head, .. } = &clause.clause else {
                unreachable!("ordinary beta completion belongs to an equation");
            };
            if !rules.iter().any(|rule| {
                rule.output == clause.id
                    && matches!(
                        &rule.proof,
                        VerifiedReconstructionProofV2::OrdinaryBetaOfVerifiedDeclaration {
                            owner_head: actual,
                            ..
                        } if actual == owner_head
                    )
            }) {
                return Err(CostFailure::Unknown(
                    AuditUnknownReason::MissingTupleDisposition,
                ));
            }
        }
    }
    Ok(())
}

enum RequiredEquationCompletion {
    OrdinaryBeta {
        definition: ClauseIdV1,
    },
    Fresh {
        head: ClauseIdV1,
        demand_output: DemandOutputIdV1,
    },
}

fn required_equation_completion(
    output: &RawPublicClauseV1,
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
) -> CostResult<Option<RequiredEquationCompletion>> {
    let RawPublicClauseKindV1::PublicEquation {
        owner_head,
        demand_output,
        pair,
        ..
    } = &output.clause
    else {
        return Ok(None);
    };
    let owners = clauses
        .values()
        .filter(|candidate| {
            matches!(
                &candidate.clause,
                RawPublicClauseKindV1::PublicDeclaration { head, .. } if head == owner_head
            )
        })
        .collect::<Vec<_>>();
    let [owner] = owners.as_slice() else {
        return Err(CostFailure::Unknown(AuditUnknownReason::IncompleteSupport));
    };
    let RawPublicClauseKindV1::PublicDeclaration {
        head,
        pair: declaration,
        presentation,
        equation_free_descriptors,
        ..
    } = &owner.clause
    else {
        unreachable!("owner query selected a public declaration");
    };

    if demand_output.is_none()
        && matches!(presentation, HeadPresentationV1::TransparentDefinition)
        && declaration.normalized_body.as_ref().is_some_and(|body| {
            pair.normalized_context == declaration.normalized_context
                && pair.normalized_left == (Term::Global { id: head.clone() })
                && pair.normalized_right == *body
                && pair.normalized_type == declaration.normalized_type
        })
    {
        return Ok(Some(RequiredEquationCompletion::OrdinaryBeta {
            definition: owner.id.clone(),
        }));
    }

    let Some(demand_output) = demand_output else {
        return Ok(None);
    };
    let matching_contracts = equation_free_descriptors
        .iter()
        .filter(|descriptor| {
            let EquationFreeDescriptorKindV1::FreshComputationContract {
                demand_output: contract_output,
                constructor,
                result,
                result_type,
                computation_mode: ComputationModeV1::ClosedNullary,
            } = &descriptor.descriptor
            else {
                return false;
            };
            contract_output == demand_output
                && pair.normalized_context.0.is_empty()
                && pair.normalized_left
                    == (Term::Apply {
                        function: Box::new(Term::Global { id: head.clone() }),
                        argument: Box::new(Term::Global {
                            id: constructor.clone(),
                        }),
                    })
                && pair.normalized_right == (Term::Global { id: result.clone() })
                && pair.normalized_type
                    == (Term::Global {
                        id: result_type.clone(),
                    })
        })
        .count();
    match matching_contracts {
        0 => Ok(None),
        1 => Ok(Some(RequiredEquationCompletion::Fresh {
            head: owner.id.clone(),
            demand_output: demand_output.clone(),
        })),
        _ => malformed(),
    }
}

fn validate_presentation_equivalences(
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    rules: &[ReconstructionDerivationV1],
    equivalences: &[PresentationEquivalenceV1],
) -> CostResult<PresentationClasses> {
    let mut parent = BTreeMap::<ClauseIdV1, ClauseIdV1>::new();
    for rule in rules {
        let ReconstructionProofV1::DuplicatePresentationDeletion { original } = &rule.proof else {
            continue;
        };
        if parent
            .insert(rule.output.clone(), original.clone())
            .is_some()
        {
            return Err(CostFailure::Unknown(AuditUnknownReason::NonUniqueBasis));
        }
    }
    for equivalence in equivalences {
        if equivalence.representative == equivalence.equivalent {
            return malformed();
        }
        let representative = clauses
            .get(&equivalence.representative)
            .ok_or(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient))?;
        let equivalent = clauses
            .get(&equivalence.equivalent)
            .ok_or(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient))?;
        match equivalence.proof {
            PresentationEquivalenceProofV1::DuplicateTransparentField => {
                if !duplicate_presentation(representative, equivalent) {
                    return Err(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient));
                }
            }
        }
        if !rules.iter().any(|rule| {
            rule.output == equivalence.equivalent
                && matches!(
                    &rule.proof,
                    ReconstructionProofV1::DuplicatePresentationDeletion { original }
                        if original == &equivalence.representative
                )
        }) {
            return Err(CostFailure::Unknown(
                AuditUnknownReason::MissingTupleDisposition,
            ));
        }
        if parent.get(&equivalence.equivalent) != Some(&equivalence.representative) {
            return Err(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient));
        }
    }

    let mut roots = BTreeMap::new();
    for clause in clauses.keys() {
        let mut current = clause.clone();
        let mut path = BTreeSet::new();
        while let Some(next) = parent.get(&current) {
            if !path.insert(current.clone()) {
                return Err(CostFailure::Unknown(AuditUnknownReason::NonUniqueBasis));
            }
            current = next.clone();
        }
        roots.insert(clause.clone(), current);
    }

    let ordered = clauses.values().collect::<Vec<_>>();
    for (left_index, left) in ordered.iter().enumerate() {
        for right in ordered.iter().skip(left_index + 1) {
            if duplicate_presentation(left, right) && roots.get(&left.id) != roots.get(&right.id) {
                return Err(CostFailure::Unknown(
                    AuditUnknownReason::MissingTupleDisposition,
                ));
            }
        }
    }
    let mut members = BTreeMap::<ClauseIdV1, BTreeSet<ClauseIdV1>>::new();
    for (clause, root) in &roots {
        members
            .entry(root.clone())
            .or_default()
            .insert(clause.clone());
    }
    Ok(PresentationClasses { roots, members })
}

fn validate_presentation_equivalences_v2(
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    rules: &[VerifiedReconstructionDerivationV2],
    equivalences: &[PresentationEquivalenceV2],
    equation_demand_ports: &BTreeMap<EquationIdV1, Option<DemandPortKeyV1>>,
    inventory: Option<&VerifiedPublicAuditInventoryV1>,
    authorities: CostAuthoritiesV2<'_>,
) -> CostResult<PresentationClassesV2> {
    let expected_equivalences = rules
        .iter()
        .filter_map(|rule| match &rule.proof {
            VerifiedReconstructionProofV2::DuplicatePresentationDeletion { original } => {
                Some((original.clone(), rule.output.clone()))
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let supplied_equivalences = equivalences
        .iter()
        .map(|equivalence| {
            (
                equivalence.representative.clone(),
                equivalence.equivalent.clone(),
            )
        })
        .collect::<BTreeSet<_>>();
    if supplied_equivalences.len() != equivalences.len() {
        return malformed();
    }
    if supplied_equivalences != expected_equivalences {
        return Err(CostFailure::Unknown(
            AuditUnknownReason::MissingTupleDisposition,
        ));
    }

    let mut parent = BTreeMap::<ClauseIdV1, ClauseIdV1>::new();
    for rule in rules {
        let VerifiedReconstructionProofV2::DuplicatePresentationDeletion { original } = &rule.proof
        else {
            continue;
        };
        if parent
            .insert(rule.output.clone(), original.clone())
            .is_some()
        {
            return Err(CostFailure::Unknown(AuditUnknownReason::NonUniqueBasis));
        }
    }
    for equivalence in equivalences {
        if equivalence.representative == equivalence.equivalent {
            return malformed();
        }
        let representative = clauses
            .get(&equivalence.representative)
            .ok_or(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient))?;
        let equivalent = clauses
            .get(&equivalence.equivalent)
            .ok_or(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient))?;
        let kind_matches = match equivalence.proof {
            PresentationEquivalenceProofV2::DuplicateTransparentField => matches!(
                (&representative.clause, &equivalent.clause),
                (
                    RawPublicClauseKindV1::PublicDeclaration { .. },
                    RawPublicClauseKindV1::PublicDeclaration { .. }
                )
            ),
            PresentationEquivalenceProofV2::DuplicateNormalizedEquation => matches!(
                (&representative.clause, &equivalent.clause),
                (
                    RawPublicClauseKindV1::PublicEquation { .. },
                    RawPublicClauseKindV1::PublicEquation { .. }
                )
            ),
        };
        if !kind_matches
            || !duplicate_presentation_v2(
                representative,
                equivalent,
                clauses,
                equation_demand_ports,
                inventory,
            )?
        {
            return Err(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient));
        }
        if !rules.iter().any(|rule| {
            rule.output == equivalence.equivalent
                && matches!(
                    &rule.proof,
                    VerifiedReconstructionProofV2::DuplicatePresentationDeletion { original }
                        if original == &equivalence.representative
                )
        }) {
            return Err(CostFailure::Unknown(
                AuditUnknownReason::MissingTupleDisposition,
            ));
        }
        if parent.get(&equivalence.equivalent) != Some(&equivalence.representative) {
            return Err(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient));
        }
    }

    let mut roots = BTreeMap::new();
    for clause in clauses.keys() {
        let mut current = clause.clone();
        let mut path = BTreeSet::new();
        while let Some(next) = parent.get(&current) {
            if !path.insert(current.clone()) {
                return Err(CostFailure::Unknown(AuditUnknownReason::NonUniqueBasis));
            }
            current = next.clone();
        }
        roots.insert(clause.clone(), PresentationClassRootV2::Clause(current));
    }

    if let Some(census) = authorities.ambient_export_census {
        let head_clauses = clauses
            .values()
            .filter_map(|clause| match &clause.clause {
                RawPublicClauseKindV1::PublicDeclaration { head, .. } => {
                    Some((head.clone(), clause.id.clone()))
                }
                _ => None,
            })
            .collect::<BTreeMap<_, _>>();
        for class in census.classes() {
            if !matches!(
                class.disposition(),
                AmbientExportDispositionV1::FirstPublicExportClass
            ) {
                continue;
            }
            let direct_members = class
                .successor_new_direct_members()
                .map(|member| {
                    head_clauses
                        .get(member.declaration())
                        .cloned()
                        .ok_or(CostFailure::Unknown(
                            AuditUnknownReason::IncompleteEnumeration,
                        ))
                })
                .collect::<CostResult<Vec<_>>>()?;
            if direct_members.len() < 2 {
                continue;
            }
            let class_root = PresentationClassRootV2::AmbientFirstExport(
                class.q2_duplicate_class_digest().clone(),
            );
            for member in direct_members {
                if roots.get(&member) != Some(&PresentationClassRootV2::Clause(member.clone())) {
                    return Err(CostFailure::Unknown(AuditUnknownReason::NonUniqueBasis));
                }
                roots.insert(member, class_root.clone());
            }
        }
    }

    let ordered = clauses.values().collect::<Vec<_>>();
    for (left_index, left) in ordered.iter().enumerate() {
        for right in ordered.iter().skip(left_index + 1) {
            if duplicate_presentation_v2(left, right, clauses, equation_demand_ports, inventory)?
                && roots.get(&left.id) != roots.get(&right.id)
            {
                return Err(CostFailure::Unknown(
                    AuditUnknownReason::MissingTupleDisposition,
                ));
            }
        }
    }
    let mut members = BTreeMap::<PresentationClassRootV2, BTreeSet<ClauseIdV1>>::new();
    for (clause, root) in &roots {
        members
            .entry(root.clone())
            .or_default()
            .insert(clause.clone());
    }
    Ok(PresentationClassesV2 { roots, members })
}

fn validate_negative_evidence(
    evidence: &CompleteNegativeEvidenceV1,
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    rules: &[ReconstructionDerivationV1],
    manifest: &VerifiedCostManifestV1,
) -> CostResult<()> {
    if evidence.checked_rules != manifest.manifest().free_completion_rules
        || !clauses.contains_key(&evidence.target)
        || evidence.against.contains(&evidence.target)
        || evidence.realized.contains(&evidence.target)
        || !evidence.against.is_subset(&evidence.realized)
        || !evidence
            .against
            .iter()
            .chain(evidence.realized.iter())
            .all(|clause| clauses.contains_key(clause))
    {
        return malformed();
    }
    for rule in rules {
        if rule.premises.is_subset(&evidence.realized) && !evidence.realized.contains(&rule.output)
        {
            return malformed();
        }
    }
    Ok(())
}

fn validate_negative_evidence_v2(
    evidence: &CompleteNegativeEvidenceV2,
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    rules: &[VerifiedReconstructionDerivationV2],
    manifest: &VerifiedCostManifestV2,
) -> CostResult<()> {
    if evidence.checked_rules != manifest.manifest().free_completion_rules
        || !clauses.contains_key(&evidence.target)
        || evidence.against.contains(&evidence.target)
        || evidence.realized.contains(&evidence.target)
        || !evidence
            .against
            .iter()
            .chain(evidence.realized.iter())
            .all(|clause| clauses.contains_key(clause))
    {
        return malformed();
    }
    let replayed = saturate(
        &evidence.against,
        rules,
        manifest.manifest().maximum_saturation_rounds,
    )?;
    if replayed.members != evidence.realized {
        return malformed();
    }
    Ok(())
}

trait HornReconstruction {
    fn output(&self) -> &ClauseIdV1;
    fn premises(&self) -> &BTreeSet<ClauseIdV1>;
}

impl HornReconstruction for ReconstructionDerivationV1 {
    fn output(&self) -> &ClauseIdV1 {
        &self.output
    }

    fn premises(&self) -> &BTreeSet<ClauseIdV1> {
        &self.premises
    }
}

impl HornReconstruction for VerifiedReconstructionDerivationV2 {
    fn output(&self) -> &ClauseIdV1 {
        &self.output
    }

    fn premises(&self) -> &BTreeSet<ClauseIdV1> {
        &self.premises
    }
}

fn saturate<R: HornReconstruction>(
    seeds: &BTreeSet<ClauseIdV1>,
    rules: &[R],
    maximum_rounds: u32,
) -> CostResult<Closure> {
    let mut members = seeds.clone();
    let mut rounds = seeds
        .iter()
        .cloned()
        .map(|seed| (seed, 0_u32))
        .collect::<BTreeMap<_, _>>();
    let mut round = 0_u32;
    loop {
        let additions = rules
            .iter()
            .filter(|rule| !members.contains(rule.output()) && rule.premises().is_subset(&members))
            .map(|rule| rule.output().clone())
            .collect::<BTreeSet<_>>();
        if additions.is_empty() {
            return Ok(Closure { members, rounds });
        }
        round = round
            .checked_add(1)
            .ok_or(CostFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
        if round > maximum_rounds {
            return Err(CostFailure::Unknown(AuditUnknownReason::ResourceExhausted));
        }
        for addition in additions {
            rounds.insert(addition.clone(), round);
            members.insert(addition);
        }
    }
}

fn build_dispositions(
    prepared: &PreparedAudit,
    signature: &BTreeSet<ClauseIdV1>,
    closure: &Closure,
) -> CostResult<Vec<ClauseDispositionCertificateV1>> {
    build_dispositions_common(
        &prepared.clause_ids,
        &prepared.class_root,
        &prepared.rules,
        signature,
        closure,
    )
}

fn build_dispositions_v2(
    prepared: &PreparedAuditV2,
    signature: &BTreeSet<PresentationClassRootV2>,
    closure: &Closure,
) -> CostResult<Vec<ClauseDispositionCertificateV2>> {
    let mut dispositions = Vec::with_capacity(prepared.clause_ids.len());
    for clause in &prepared.clause_ids {
        let root = prepared
            .class_root
            .get(clause)
            .ok_or(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient))?;
        let closure_round = *closure.rounds.get(clause).ok_or(CostFailure::Unknown(
            AuditUnknownReason::IncompleteEnumeration,
        ))?;

        if let PresentationClassRootV2::AmbientFirstExport(primitive_class_digest) = root
            && signature.contains(root)
        {
            if closure_round != 0 {
                return Err(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient));
            }
            dispositions.push(ClauseDispositionCertificateV2 {
                clause: clause.clone(),
                disposition: ClauseCostDispositionV2::AmbientFirstExportClassMember {
                    primitive_class_digest: primitive_class_digest.clone(),
                },
                closure_round,
                reconstructions: Vec::new(),
            });
            continue;
        }
        if signature.contains(&PresentationClassRootV2::Clause(clause.clone())) {
            dispositions.push(ClauseDispositionCertificateV2 {
                clause: clause.clone(),
                disposition: ClauseCostDispositionV2::FirstIrreducible,
                closure_round,
                reconstructions: Vec::new(),
            });
            continue;
        }
        if let PresentationClassRootV2::Clause(original) = root
            && original != clause
            && signature.contains(root)
        {
            let reconstructions = grounded_reconstructions_v2(clause, closure, &prepared.rules);
            dispositions.push(ClauseDispositionCertificateV2 {
                clause: clause.clone(),
                disposition: ClauseCostDispositionV2::DuplicatePresentation {
                    original: original.clone(),
                },
                closure_round,
                reconstructions,
            });
            continue;
        }

        let reconstructions = grounded_reconstructions_v2(clause, closure, &prepared.rules);
        if reconstructions.is_empty() {
            return Err(CostFailure::Unknown(
                AuditUnknownReason::MissingTupleDisposition,
            ));
        }
        let disposition = disposition_from_rules_v2(&reconstructions)?;
        dispositions.push(ClauseDispositionCertificateV2 {
            clause: clause.clone(),
            disposition,
            closure_round,
            reconstructions,
        });
    }
    Ok(dispositions)
}

fn build_dispositions_common(
    clause_ids: &[ClauseIdV1],
    class_root: &BTreeMap<ClauseIdV1, ClauseIdV1>,
    rules: &[ReconstructionDerivationV1],
    signature: &BTreeSet<ClauseIdV1>,
    closure: &Closure,
) -> CostResult<Vec<ClauseDispositionCertificateV1>> {
    let mut dispositions = Vec::with_capacity(clause_ids.len());
    for clause in clause_ids {
        let root = class_root
            .get(clause)
            .ok_or(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient))?;
        let closure_round = *closure.rounds.get(clause).ok_or(CostFailure::Unknown(
            AuditUnknownReason::IncompleteEnumeration,
        ))?;

        if signature.contains(clause) {
            dispositions.push(ClauseDispositionCertificateV1 {
                clause: clause.clone(),
                disposition: ClauseCostDispositionV1::FirstIrreducible,
                closure_round,
                reconstructions: Vec::new(),
            });
            continue;
        }
        if root != clause && signature.contains(root) {
            let reconstructions = grounded_reconstructions(clause, closure, rules);
            dispositions.push(ClauseDispositionCertificateV1 {
                clause: clause.clone(),
                disposition: ClauseCostDispositionV1::DuplicatePresentation {
                    original: root.clone(),
                },
                closure_round,
                reconstructions,
            });
            continue;
        }

        let reconstructions = grounded_reconstructions(clause, closure, rules);
        if reconstructions.is_empty() {
            return Err(CostFailure::Unknown(
                AuditUnknownReason::MissingTupleDisposition,
            ));
        }
        let disposition = disposition_from_rules(&reconstructions)?;
        dispositions.push(ClauseDispositionCertificateV1 {
            clause: clause.clone(),
            disposition,
            closure_round,
            reconstructions,
        });
    }
    Ok(dispositions)
}

fn grounded_reconstructions(
    clause: &ClauseIdV1,
    closure: &Closure,
    rules: &[ReconstructionDerivationV1],
) -> Vec<ReconstructionDerivationV1> {
    let Some(output_round) = closure.rounds.get(clause) else {
        return Vec::new();
    };
    rules
        .iter()
        .filter(|rule| {
            rule.output == *clause
                && rule.premises.iter().all(|premise| {
                    closure
                        .rounds
                        .get(premise)
                        .is_some_and(|premise_round| premise_round < output_round)
                })
        })
        .cloned()
        .collect()
}

fn disposition_from_rules(
    rules: &[ReconstructionDerivationV1],
) -> CostResult<ClauseCostDispositionV1> {
    let duplicate_originals = rules
        .iter()
        .filter_map(|rule| match &rule.proof {
            ReconstructionProofV1::DuplicatePresentationDeletion { original } => {
                Some(original.clone())
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    if duplicate_originals.len() > 1 {
        return Err(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient));
    }
    if let Some(original) = duplicate_originals.into_iter().next() {
        return Ok(ClauseCostDispositionV1::DuplicatePresentation { original });
    }
    if rules.iter().any(|rule| {
        matches!(
            &rule.proof,
            ReconstructionProofV1::DescriptorForcedProjection { .. }
        )
    }) {
        return Ok(ClauseCostDispositionV1::ForcedProjection);
    }
    let alias_targets = rules
        .iter()
        .filter_map(|rule| match &rule.proof {
            ReconstructionProofV1::PriorPublicTransparentAlias { target, .. } => {
                Some(target.clone())
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    if alias_targets.len() > 1 {
        return malformed();
    }
    if let Some(target) = alias_targets.into_iter().next() {
        return Ok(ClauseCostDispositionV1::TransparentAlias { target });
    }
    if rules.iter().any(|rule| {
        matches!(
            &rule.proof,
            ReconstructionProofV1::OrdinaryBeta { .. }
                | ReconstructionProofV1::CertifiedFreshConstructorComputation { .. }
        )
    }) {
        return Ok(ClauseCostDispositionV1::ForcedDefinitionalCompletion);
    }
    Err(CostFailure::Unknown(
        AuditUnknownReason::MissingTupleDisposition,
    ))
}

fn grounded_reconstructions_v2(
    clause: &ClauseIdV1,
    closure: &Closure,
    rules: &[VerifiedReconstructionDerivationV2],
) -> Vec<VerifiedReconstructionDerivationV2> {
    let Some(output_round) = closure.rounds.get(clause) else {
        return Vec::new();
    };
    rules
        .iter()
        .filter(|rule| {
            rule.output == *clause
                && rule.premises.iter().all(|premise| {
                    closure
                        .rounds
                        .get(premise)
                        .is_some_and(|premise_round| premise_round < output_round)
                })
        })
        .cloned()
        .collect()
}

fn disposition_from_rules_v2(
    rules: &[VerifiedReconstructionDerivationV2],
) -> CostResult<ClauseCostDispositionV2> {
    let ambient_first_export_classes = rules
        .iter()
        .filter_map(|rule| match &rule.proof {
            VerifiedReconstructionProofV2::AmbientFirstExportClassDuplicate {
                primitive_class_digest,
                ..
            } => Some(primitive_class_digest.clone()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    let ambient_originals = rules
        .iter()
        .filter_map(|rule| match &rule.proof {
            VerifiedReconstructionProofV2::PriorPublicAmbientReexport {
                predecessor_class_digest,
                ..
            } => Some(predecessor_class_digest.clone()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    if ambient_first_export_classes.len() > 1
        || ambient_originals.len() > 1
        || (!ambient_first_export_classes.is_empty() && !ambient_originals.is_empty())
    {
        return Err(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient));
    }
    if let Some(primitive_class_digest) = ambient_first_export_classes.into_iter().next() {
        return Ok(ClauseCostDispositionV2::AmbientFirstExportClassMember {
            primitive_class_digest,
        });
    }
    if let Some(predecessor_class_digest) = ambient_originals.into_iter().next() {
        return Ok(ClauseCostDispositionV2::AmbientPriorPublicReexport {
            predecessor_class_digest,
        });
    }
    let duplicate_originals = rules
        .iter()
        .filter_map(|rule| match &rule.proof {
            VerifiedReconstructionProofV2::DuplicatePresentationDeletion { original } => {
                Some(original.clone())
            }
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    if duplicate_originals.len() > 1 {
        return Err(CostFailure::Unknown(AuditUnknownReason::UnknownQuotient));
    }
    if let Some(original) = duplicate_originals.into_iter().next() {
        return Ok(ClauseCostDispositionV2::DuplicatePresentation { original });
    }
    if rules.iter().any(|rule| {
        matches!(
            rule.proof,
            VerifiedReconstructionProofV2::DescriptorForcedProjection { .. }
        )
    }) {
        return Ok(ClauseCostDispositionV2::ForcedProjection);
    }
    let alias_targets = rules
        .iter()
        .filter_map(|rule| match &rule.proof {
            VerifiedReconstructionProofV2::PriorPublicTransparentAlias { target, .. }
            | VerifiedReconstructionProofV2::PriorPublicEquationReplay {
                owner_head: target, ..
            } => Some(target.clone()),
            _ => None,
        })
        .collect::<BTreeSet<_>>();
    if alias_targets.len() > 1 {
        return malformed();
    }
    if let Some(target) = alias_targets.into_iter().next() {
        return Ok(ClauseCostDispositionV2::TransparentAlias { target });
    }
    if rules.iter().any(|rule| {
        matches!(
            rule.proof,
            VerifiedReconstructionProofV2::OrdinaryBetaOfVerifiedDeclaration { .. }
        )
    }) {
        return Ok(ClauseCostDispositionV2::ForcedDefinitionalCompletion);
    }
    Err(CostFailure::Unknown(
        AuditUnknownReason::MissingTupleDisposition,
    ))
}

fn mask_to_set(mask: u32, ids: &[ClauseIdV1]) -> BTreeSet<ClauseIdV1> {
    ids.iter()
        .enumerate()
        .filter(|(index, _)| mask & (1_u32 << index) != 0)
        .map(|(_, clause)| clause.clone())
        .collect()
}

fn exact_api_digest(input: &KernelCostAuditInputV1) -> Digest {
    let mut clauses = input.clauses.clone();
    clauses.sort_by(|left, right| left.id.cmp(&right.id));
    for clause in &mut clauses {
        if let RawPublicClauseKindV1::PublicDeclaration {
            equation_free_descriptors,
            ..
        } = &mut clause.clause
        {
            sort_canonical(
                equation_free_descriptors,
                "pen-semantic-audit/descriptor-sort/v1",
            );
        }
    }
    let mut public_dependency_dag = input.public_dependency_dag.clone();
    public_dependency_dag.sort();
    let mut reconstructions = input.reconstructions.clone();
    sort_canonical(
        &mut reconstructions,
        "pen-semantic-audit/reconstruction-sort/v1",
    );
    let mut negative_evidence = input.negative_evidence.clone();
    sort_canonical(
        &mut negative_evidence,
        "pen-semantic-audit/negative-evidence-sort/v1",
    );
    let mut presentation_equivalences = input.presentation_equivalences.clone();
    sort_canonical(
        &mut presentation_equivalences,
        "pen-semantic-audit/presentation-equivalence-sort/v1",
    );

    let mut encoder = CanonicalEncoder::new();
    encoder.sequence(&clauses);
    encoder.sequence(&public_dependency_dag);
    encoder.sequence(&reconstructions);
    encoder.sequence(&negative_evidence);
    encoder.sequence(&presentation_equivalences);
    Digest::of_domain_bytes(
        "pen-semantic-audit/exact-kernel-cost-api/v1",
        encoder.as_bytes(),
    )
}

fn exact_api_digest_v2(input: &KernelCostAuditInputV2, inventory_digest: &Digest) -> Digest {
    let mut clauses = input.clauses.clone();
    clauses.sort_by(|left, right| left.id.cmp(&right.id));
    for clause in &mut clauses {
        if let RawPublicClauseKindV1::PublicDeclaration {
            equation_free_descriptors,
            ..
        } = &mut clause.clause
        {
            sort_canonical(
                equation_free_descriptors,
                "pen-semantic-audit/descriptor-sort/v1",
            );
        }
    }
    let mut public_dependency_dag = input.public_dependency_dag.clone();
    public_dependency_dag.sort();
    let mut equation_demand_ports = input.equation_demand_ports.clone();
    sort_canonical(
        &mut equation_demand_ports,
        "pen-semantic-audit/equation-demand-port-sort/v2",
    );
    let mut reconstructions = input.reconstructions.clone();
    sort_canonical(
        &mut reconstructions,
        "pen-semantic-audit/reconstruction-sort/v2",
    );
    let mut negative_evidence = input.negative_evidence.clone();
    sort_canonical(
        &mut negative_evidence,
        "pen-semantic-audit/negative-evidence-sort/v2",
    );
    let mut presentation_equivalences = input.presentation_equivalences.clone();
    sort_canonical(
        &mut presentation_equivalences,
        "pen-semantic-audit/presentation-equivalence-sort/v2",
    );

    let mut encoder = CanonicalEncoder::new();
    inventory_digest.encode_canonical(&mut encoder);
    encoder.sequence(&clauses);
    encoder.sequence(&public_dependency_dag);
    encoder.sequence(&equation_demand_ports);
    encoder.sequence(&reconstructions);
    encoder.sequence(&negative_evidence);
    encoder.sequence(&presentation_equivalences);
    Digest::of_domain_bytes(
        "pen-semantic-audit/exact-kernel-cost-api/v2",
        encoder.as_bytes(),
    )
}

fn dependency_components<R: HornReconstruction>(
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    rules: &[R],
) -> Vec<DependencyComponentV1> {
    let mut adjacency = clauses
        .iter()
        .map(|(id, clause)| (id.clone(), clause.semantic_dependencies.clone()))
        .collect::<BTreeMap<_, _>>();
    for rule in rules {
        adjacency
            .entry(rule.output().clone())
            .or_default()
            .extend(rule.premises().iter().cloned());
    }

    struct Tarjan<'a> {
        adjacency: &'a BTreeMap<ClauseIdV1, BTreeSet<ClauseIdV1>>,
        index: usize,
        indices: BTreeMap<ClauseIdV1, usize>,
        lowlinks: BTreeMap<ClauseIdV1, usize>,
        stack: Vec<ClauseIdV1>,
        on_stack: BTreeSet<ClauseIdV1>,
        components: Vec<BTreeSet<ClauseIdV1>>,
    }
    impl Tarjan<'_> {
        fn connect(&mut self, node: ClauseIdV1) {
            let node_index = self.index;
            self.index += 1;
            self.indices.insert(node.clone(), node_index);
            self.lowlinks.insert(node.clone(), node_index);
            self.stack.push(node.clone());
            self.on_stack.insert(node.clone());

            let neighbors = self.adjacency.get(&node).cloned().unwrap_or_default();
            for neighbor in neighbors {
                if !self.indices.contains_key(&neighbor) {
                    self.connect(neighbor.clone());
                    let neighbor_low = self.lowlinks[&neighbor];
                    let node_low = self.lowlinks[&node];
                    self.lowlinks
                        .insert(node.clone(), node_low.min(neighbor_low));
                } else if self.on_stack.contains(&neighbor) {
                    let neighbor_index = self.indices[&neighbor];
                    let node_low = self.lowlinks[&node];
                    self.lowlinks
                        .insert(node.clone(), node_low.min(neighbor_index));
                }
            }

            if self.lowlinks[&node] == self.indices[&node] {
                let mut component = BTreeSet::new();
                while let Some(member) = self.stack.pop() {
                    self.on_stack.remove(&member);
                    component.insert(member.clone());
                    if member == node {
                        break;
                    }
                }
                self.components.push(component);
            }
        }
    }

    let mut tarjan = Tarjan {
        adjacency: &adjacency,
        index: 0,
        indices: BTreeMap::new(),
        lowlinks: BTreeMap::new(),
        stack: Vec::new(),
        on_stack: BTreeSet::new(),
        components: Vec::new(),
    };
    for node in adjacency.keys() {
        if !tarjan.indices.contains_key(node) {
            tarjan.connect(node.clone());
        }
    }

    let mut remaining = tarjan.components;
    remaining.sort();
    let component_of = remaining
        .iter()
        .enumerate()
        .flat_map(|(index, component)| component.iter().cloned().map(move |member| (member, index)))
        .collect::<BTreeMap<_, _>>();
    let mut ordered = Vec::with_capacity(remaining.len());
    let mut emitted = BTreeSet::new();
    while ordered.len() < remaining.len() {
        let Some(next) = remaining.iter().enumerate().find_map(|(index, component)| {
            if emitted.contains(&index) {
                return None;
            }
            let ready = component.iter().all(|member| {
                adjacency.get(member).is_none_or(|dependencies| {
                    dependencies.iter().all(|dependency| {
                        let dependency_component = component_of[dependency];
                        dependency_component == index || emitted.contains(&dependency_component)
                    })
                })
            });
            ready.then_some(index)
        }) else {
            break;
        };
        emitted.insert(next);
        ordered.push(DependencyComponentV1 {
            members: remaining[next].clone(),
        });
    }
    ordered
}

fn clause_head(clause: &RawPublicClauseV1) -> Option<&GlobalId> {
    match &clause.clause {
        RawPublicClauseKindV1::PublicDeclaration { head, .. } => Some(head),
        RawPublicClauseKindV1::ForcedProjectionClause { projection, .. } => Some(projection),
        RawPublicClauseKindV1::PublicEquation { .. } => None,
    }
}

fn singleton<T: Ord>(value: T) -> BTreeSet<T> {
    std::iter::once(value).collect()
}

fn sort_canonical<T: CanonicalEncode>(values: &mut [T], domain: &str) {
    values.sort_by_cached_key(|value| Digest::of_canonical(domain, value));
}

fn reject_canonical_duplicates<T: CanonicalEncode>(values: &[T], domain: &str) -> CostResult<()> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(Digest::of_canonical(domain, value)) {
            return malformed();
        }
    }
    Ok(())
}

fn malformed<T>() -> CostResult<T> {
    Err(CostFailure::Unknown(AuditUnknownReason::MalformedInput))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn audit_kernel_cost_lambda_unit_v2(
        cost_manifest: &VerifiedCostManifestV2,
        semantic_manifest: &VerifiedSemanticAuditManifestV1,
        inventory: &VerifiedPublicAuditInventoryV1,
        ordinary_beta_derivations: &[VerifiedOrdinaryBetaDerivationV1],
        input: &KernelCostAuditInputV2,
    ) -> AuditDecision<CostAuditCertificateV2> {
        audit_kernel_cost_lambda_unit_components_v2(
            cost_manifest,
            semantic_manifest,
            inventory,
            ordinary_beta_derivations,
            input,
        )
    }

    #[test]
    fn lambda_unit_metadata_allows_operation_roles_but_not_record_fields() {
        let owner = GlobalId(Digest::of_bytes(b"lambda-unit-metadata-owner"));
        let clause = ClauseIdV1(Digest::of_bytes(b"lambda-unit-metadata-clause"));
        let operation_role = EquationFreeDescriptorV1 {
            owner: owner.clone(),
            source_clause: clause.clone(),
            descriptor: EquationFreeDescriptorKindV1::OperationRole {
                role: LocalRoleV1::KernelHead,
            },
            public_support: BTreeSet::new(),
        };
        assert_eq!(
            lambda_unit_descriptor_syntax_violation(&operation_role),
            None
        );

        let record_field = EquationFreeDescriptorV1 {
            owner: owner.clone(),
            source_clause: clause,
            descriptor: EquationFreeDescriptorKindV1::RecordField {
                record: owner.clone(),
                field_ordinal: 0,
                field_type: owner,
            },
            public_support: BTreeSet::new(),
        };
        assert_eq!(
            lambda_unit_descriptor_syntax_violation(&record_field),
            Some(LambdaUnitSyntaxViolation::DescriptorProjection)
        );
    }
    use crate::inventory::{
        DemandContractIdV1, DemandFamilyIdV1, DemandPortKeyV1, ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
        PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION, UncheckedOriginCutoffQ3RegistryV1,
        UncheckedPredecessorDemandContractV1, UncheckedPublicAuditInventoryV1,
        UncheckedPublicAvailabilityClaimV1, UncheckedPublicDeclarationV1,
        UncheckedPublicDependencyDagV1, UncheckedPublicEquationV1, UncheckedPublicEventCensusV1,
        UncheckedPublicGroupV1, UncheckedPublicHistoryStepV1,
        UncheckedSourceNormalizedDeclarationV1, verify_public_audit_inventory_v1,
    };
    use crate::manifest::{
        proposed_kernel_cost_lambda_unit_manifest_v2, proposed_kernel_cost_manifest_v1,
        proposed_kernel_cost_manifest_v2, proposed_semantic_audit_lambda_unit_manifest_v1,
        verify_core_manifests_v1, verify_kernel_cost_lambda_unit_manifest_v2,
        verify_kernel_cost_manifest_v1, verify_kernel_cost_manifest_v2,
        verify_semantic_audit_lambda_unit_manifest_v1,
    };
    use crate::model::{AmbientPrimitiveV1, EquationIdV1, EventIdV1, SourceNormalizedJudgmentV1};
    use crate::ordinary_beta::verify_ordinary_beta_derivation_v1;
    use pen_kernel::{Declaration, Kernel, KernelLimits, UncheckedSignature};

    fn digest(label: &str) -> Digest {
        Digest::of_domain_bytes("pen-semantic-audit/cost-test/v1", label.as_bytes())
    }

    fn clause_id(label: &str) -> ClauseIdV1 {
        ClauseIdV1(digest(&format!("clause/{label}")))
    }

    fn global(label: &str) -> GlobalId {
        GlobalId(digest(&format!("global/{label}")))
    }

    fn equation_id(label: &str) -> EquationIdV1 {
        EquationIdV1(digest(&format!("equation/{label}")))
    }

    fn demand_id(label: &str) -> DemandOutputIdV1 {
        DemandOutputIdV1(digest(&format!("demand/{label}")))
    }

    fn event_id(label: &str) -> EventIdV1 {
        EventIdV1(digest(&format!("event/{label}")))
    }

    fn demand_contract_id(label: &str) -> DemandContractIdV1 {
        DemandContractIdV1(digest(&format!("contract/{label}")))
    }

    fn inventory_source_declaration(
        declaration: &Declaration,
    ) -> UncheckedSourceNormalizedDeclarationV1 {
        UncheckedSourceNormalizedDeclarationV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-declaration/v1",
                declaration,
            ),
            source: declaration.clone(),
            claimed_normalized: declaration.clone(),
        }
    }

    fn inventory_source_equation(judgment: GenericJudgmentV1) -> SourceNormalizedJudgmentV1 {
        SourceNormalizedJudgmentV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-judgment/v1",
                &judgment,
            ),
            source: judgment.clone(),
            claimed_normalized: judgment,
        }
    }

    fn inventory_dependency(
        dependent: PublicSubjectV1,
        prerequisite: &GlobalId,
    ) -> crate::inventory::PublicDependencyUseV1 {
        crate::inventory::PublicDependencyUseV1 {
            dependent,
            prerequisite: prerequisite.clone(),
        }
    }

    #[derive(Clone, Copy)]
    enum PublicEquationFixtureMode {
        Paid,
        PredecessorReplay,
        Duplicate,
        AliasPresentation,
        PredecessorOwnerDuplicate,
        AmbientBody,
        BodyfulBeta,
    }

    struct PublicV2Fixture {
        inventory: VerifiedPublicAuditInventoryV1,
        input: KernelCostAuditInputV2,
        head_clause: ClauseIdV1,
        equation_clause: ClauseIdV1,
        duplicate_clause: Option<ClauseIdV1>,
        predecessor_equation: EquationIdV1,
        successor_equation: EquationIdV1,
    }

    struct AmbientCostFixture {
        inventory: VerifiedPublicAuditInventoryV1,
        input: KernelCostAuditInputV2,
        class_digest: Digest,
        q2_class_digest: Digest,
        predecessor_q2_class_digest: Option<Digest>,
    }

    fn ambient_declaration(label: &str, body: Term) -> Declaration {
        Declaration {
            id: global(label),
            ty: Term::Sort { level: 0 },
            body: Some(body),
        }
    }

    fn ambient_cost_fixture(
        semantic_manifest: &VerifiedSemanticAuditManifestV1,
        cost_manifest: &VerifiedCostManifestV2,
        predecessor: Vec<Declaration>,
        successor_new: Vec<Declaration>,
        clause_ids: Vec<ClauseIdV1>,
    ) -> AmbientCostFixture {
        assert!(!successor_new.is_empty());
        assert_eq!(successor_new.len(), clause_ids.len());

        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let predecessor_event = event_id("ambient-cost/predecessor");
        let successor_event = event_id("ambient-cost/successor");
        let predecessor_group = global("ambient-cost/group/predecessor");
        let successor_group = global("ambient-cost/group/successor");
        let predecessor_boundary = UncheckedSignature {
            declarations: predecessor.clone(),
        };
        let mut all_declarations = predecessor.clone();
        all_declarations.extend(successor_new.clone());
        let successor_boundary = UncheckedSignature {
            declarations: all_declarations.clone(),
        };
        let normalized_successor = kernel
            .verify_signature(&successor_boundary)
            .expect("ambient cost signature verifies")
            .normalized_wire();
        let normalized_by_id = normalized_successor
            .declarations
            .iter()
            .map(|declaration| (declaration.id.clone(), declaration))
            .collect::<BTreeMap<_, _>>();
        let predecessor_ids = predecessor
            .iter()
            .map(|declaration| declaration.id.clone())
            .collect::<BTreeSet<_>>();
        let successor_ids = successor_new
            .iter()
            .map(|declaration| declaration.id.clone())
            .collect::<BTreeSet<_>>();

        let mut declaration_groups = Vec::new();
        if !predecessor_ids.is_empty() {
            declaration_groups.push(UncheckedPublicGroupV1 {
                group: predecessor_group.clone(),
                origin: predecessor_event.clone(),
                declarations: predecessor_ids.iter().cloned().collect(),
            });
        }
        declaration_groups.push(UncheckedPublicGroupV1 {
            group: successor_group.clone(),
            origin: successor_event.clone(),
            declarations: successor_ids.iter().cloned().collect(),
        });

        let declarations = all_declarations
            .iter()
            .map(|declaration| {
                let is_predecessor = predecessor_ids.contains(&declaration.id);
                UncheckedPublicDeclarationV1 {
                    declaration: declaration.id.clone(),
                    origin: if is_predecessor {
                        predecessor_event.clone()
                    } else {
                        successor_event.clone()
                    },
                    group: if is_predecessor {
                        predecessor_group.clone()
                    } else {
                        successor_group.clone()
                    },
                    source_to_normal: UncheckedSourceNormalizedDeclarationV1 {
                        source_identity: Digest::of_canonical(
                            "pen-semantic-audit/inventory-source-declaration/v1",
                            declaration,
                        ),
                        source: declaration.clone(),
                        claimed_normalized: (*normalized_by_id
                            .get(&declaration.id)
                            .expect("normalized ambient declaration"))
                        .clone(),
                    },
                }
            })
            .collect::<Vec<_>>();
        let predecessor_history = if predecessor_ids.is_empty() {
            Vec::new()
        } else {
            vec![UncheckedPublicHistoryStepV1 {
                census: UncheckedPublicEventCensusV1 {
                    event: predecessor_event.clone(),
                    added_groups: vec![predecessor_group],
                    added_declarations: predecessor_ids.iter().cloned().collect(),
                    added_equations: Vec::new(),
                    added_forced_projections: Vec::new(),
                    added_demand_contracts: Vec::new(),
                },
                successor_boundary: predecessor_boundary.clone(),
            }]
        };
        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history,
            predecessor_boundary,
            successor_event: UncheckedPublicEventCensusV1 {
                event: successor_event,
                added_groups: vec![successor_group],
                added_declarations: successor_ids.iter().cloned().collect(),
                added_equations: Vec::new(),
                added_forced_projections: Vec::new(),
                added_demand_contracts: Vec::new(),
            },
            successor_boundary,
            declaration_groups,
            declarations,
            equations: Vec::new(),
            forced_projections: Vec::new(),
            predecessor_demand_contracts: Vec::new(),
            public_availability: Vec::new(),
            dependency_dag: UncheckedPublicDependencyDagV1 { edges: Vec::new() },
            q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
                schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
                origin_cutoff: (!predecessor_ids.is_empty()).then_some(predecessor_event),
                entries: Vec::new(),
            },
        };
        let AuditDecision::Proven(inventory) =
            verify_public_audit_inventory_v1(semantic_manifest, &kernel, &wire)
        else {
            panic!("ambient cost inventory verifies");
        };
        let AuditDecision::Proven(census) = verify_ambient_primitive_export_census_v1(&inventory)
        else {
            panic!("ambient export census verifies");
        };
        let class = census
            .class_for_successor_declaration(&successor_new[0].id)
            .expect("successor declaration has an ambient class");
        let class_digest = class.digest().clone();
        let q2_class_digest = class.q2_duplicate_class_digest().clone();
        let predecessor_q2_class_digest =
            class.disposition().predecessor_q2_class_digest().cloned();

        let clauses = successor_new
            .iter()
            .zip(&clause_ids)
            .map(|(declaration, clause_id)| {
                let verified = inventory
                    .public_declaration(&declaration.id)
                    .expect("successor declaration is public");
                let member = class
                    .successor_new_member(&declaration.id)
                    .expect("successor declaration belongs to the class");
                assert!(member.presentation().is_direct());
                let presentation = match class.disposition() {
                    AmbientExportDispositionV1::FirstPublicExportClass => {
                        HeadPresentationV1::AmbientPrimitiveFirstExport {
                            primitive: class.primitive().clone(),
                        }
                    }
                    AmbientExportDispositionV1::PriorPublicReexport { .. } => {
                        HeadPresentationV1::TransparentDefinition
                    }
                };
                RawPublicClauseV1 {
                    id: clause_id.clone(),
                    semantic_dependencies: BTreeSet::new(),
                    clause: RawPublicClauseKindV1::PublicDeclaration {
                        head: declaration.id.clone(),
                        pair: SourceNormalizedDeclarationV1 {
                            source_identity: verified.source_identity().clone(),
                            source_context: DependentContext::default(),
                            source_type: verified.source().ty.clone(),
                            source_body: verified.source().body.clone(),
                            normalized_context: DependentContext::default(),
                            normalized_type: verified.normalized().ty.clone(),
                            normalized_body: verified.normalized().body.clone(),
                            source_to_normal_derivation: SourceToNormalDerivationV1::Reflexivity,
                        },
                        presentation,
                        public_group: verified.group().clone(),
                        equation_free_descriptors: Vec::new(),
                    },
                }
            })
            .collect::<Vec<_>>();
        let negative_evidence = if matches!(
            class.disposition(),
            AmbientExportDispositionV1::FirstPublicExportClass
        ) {
            clause_ids
                .iter()
                .cloned()
                .map(|target| CompleteNegativeEvidenceV2 {
                    target,
                    against: BTreeSet::new(),
                    realized: BTreeSet::new(),
                    checked_rules: cost_manifest.manifest().free_completion_rules.clone(),
                })
                .collect()
        } else {
            Vec::new()
        };

        AmbientCostFixture {
            inventory,
            input: KernelCostAuditInputV2 {
                clauses,
                negative_evidence,
                ..KernelCostAuditInputV2::default()
            },
            class_digest,
            q2_class_digest,
            predecessor_q2_class_digest,
        }
    }

    fn public_equation_fixture(mode: PublicEquationFixtureMode) -> PublicV2Fixture {
        let AuditDecision::Proven((semantic_manifest, _)) = verify_core_manifests_v1() else {
            panic!("generic semantic manifest verifies");
        };
        public_equation_fixture_for_manifest(mode, &semantic_manifest)
    }

    fn public_equation_fixture_for_manifest(
        mode: PublicEquationFixtureMode,
        semantic_manifest: &VerifiedSemanticAuditManifestV1,
    ) -> PublicV2Fixture {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let equation_only = matches!(mode, PublicEquationFixtureMode::PredecessorOwnerDuplicate);
        let duplicate_mode = matches!(
            mode,
            PublicEquationFixtureMode::Duplicate
                | PublicEquationFixtureMode::PredecessorOwnerDuplicate
        );

        let type_head = global("public/A");
        let predecessor_term = global("public/a");
        let successor_head = global("public/f");
        let group_type = global("public/group/A");
        let group_term = global("public/group/a");
        let group_successor = global("public/group/f");
        let event_type = event_id("public/type");
        let event_term = event_id("public/term");
        let event_successor = event_id("public/successor");
        let predecessor_equation = equation_id("public/predecessor-equation");
        let successor_equation = equation_id("public/successor-equation");
        let duplicate_equation = equation_id("public/successor-equation-duplicate");
        let demand_contract = demand_contract_id("public/compute");
        let demand_output = demand_id("public/compute");
        let demand_port = DemandPortKeyV1 {
            family: DemandFamilyIdV1(digest("public/demand-family")),
            output: demand_output.clone(),
        };

        let declaration_type = Declaration {
            id: type_head.clone(),
            ty: Term::Sort { level: 0 },
            body: None,
        };
        let declaration_term = Declaration {
            id: predecessor_term.clone(),
            ty: Term::Global {
                id: type_head.clone(),
            },
            body: None,
        };
        let declaration_successor = Declaration {
            id: successor_head.clone(),
            ty: if matches!(mode, PublicEquationFixtureMode::AliasPresentation) {
                Term::Global {
                    id: type_head.clone(),
                }
            } else if matches!(mode, PublicEquationFixtureMode::AmbientBody) {
                Term::UnitType
            } else {
                Term::Pi {
                    parameter: Box::new(Term::Global {
                        id: type_head.clone(),
                    }),
                    body: Box::new(Term::Global {
                        id: type_head.clone(),
                    }),
                }
            },
            body: match mode {
                PublicEquationFixtureMode::AliasPresentation => Some(Term::Global {
                    id: predecessor_term.clone(),
                }),
                PublicEquationFixtureMode::AmbientBody => Some(Term::Unit),
                PublicEquationFixtureMode::BodyfulBeta => Some(Term::Lambda {
                    parameter_type: Box::new(Term::Global {
                        id: type_head.clone(),
                    }),
                    body: Box::new(Term::Var { index: 0 }),
                }),
                _ => None,
            },
        };
        let paid_equation = GenericJudgmentV1::Equation {
            context: DependentContext::default(),
            left: Term::Global {
                id: predecessor_term.clone(),
            },
            right: Term::Global {
                id: predecessor_term.clone(),
            },
            ty: Term::Global {
                id: type_head.clone(),
            },
        };
        let bodyful_beta_source = GenericJudgmentV1::Equation {
            context: DependentContext::default(),
            left: Term::Apply {
                function: Box::new(Term::Global {
                    id: successor_head.clone(),
                }),
                argument: Box::new(Term::Global {
                    id: predecessor_term.clone(),
                }),
            },
            right: Term::Global {
                id: predecessor_term.clone(),
            },
            ty: Term::Global {
                id: type_head.clone(),
            },
        };
        let unrelated_predecessor = GenericJudgmentV1::Equation {
            context: DependentContext::default(),
            left: Term::Unit,
            right: Term::Unit,
            ty: Term::UnitType,
        };
        let predecessor_judgment = match mode {
            PublicEquationFixtureMode::PredecessorReplay => paid_equation.clone(),
            PublicEquationFixtureMode::Paid
            | PublicEquationFixtureMode::Duplicate
            | PublicEquationFixtureMode::AliasPresentation
            | PublicEquationFixtureMode::PredecessorOwnerDuplicate
            | PublicEquationFixtureMode::AmbientBody
            | PublicEquationFixtureMode::BodyfulBeta => unrelated_predecessor,
        };

        let boundary_type = UncheckedSignature {
            declarations: vec![declaration_type.clone()],
        };
        let predecessor_boundary = UncheckedSignature {
            declarations: vec![declaration_type.clone(), declaration_term.clone()],
        };
        let successor_boundary = if equation_only {
            predecessor_boundary.clone()
        } else {
            UncheckedSignature {
                declarations: vec![
                    declaration_type.clone(),
                    declaration_term.clone(),
                    declaration_successor.clone(),
                ],
            }
        };

        let mut dependencies = vec![
            inventory_dependency(
                PublicSubjectV1::Declaration {
                    declaration: predecessor_term.clone(),
                },
                &type_head,
            ),
            inventory_dependency(
                PublicSubjectV1::Equation {
                    equation: predecessor_equation.clone(),
                },
                &predecessor_term,
            ),
            inventory_dependency(
                PublicSubjectV1::DemandContract {
                    contract: demand_contract.clone(),
                },
                &type_head,
            ),
            inventory_dependency(
                PublicSubjectV1::DemandContract {
                    contract: demand_contract.clone(),
                },
                &predecessor_term,
            ),
        ];
        if !equation_only && !matches!(mode, PublicEquationFixtureMode::AmbientBody) {
            dependencies.push(inventory_dependency(
                PublicSubjectV1::Declaration {
                    declaration: successor_head.clone(),
                },
                &type_head,
            ));
        }
        if matches!(mode, PublicEquationFixtureMode::PredecessorReplay) {
            dependencies.push(inventory_dependency(
                PublicSubjectV1::Equation {
                    equation: predecessor_equation.clone(),
                },
                &type_head,
            ));
        }
        if matches!(mode, PublicEquationFixtureMode::AliasPresentation) {
            dependencies.push(inventory_dependency(
                PublicSubjectV1::Declaration {
                    declaration: successor_head.clone(),
                },
                &predecessor_term,
            ));
        }
        let successor_equation_ids = match mode {
            PublicEquationFixtureMode::Duplicate
            | PublicEquationFixtureMode::PredecessorOwnerDuplicate => {
                vec![successor_equation.clone(), duplicate_equation.clone()]
            }
            PublicEquationFixtureMode::Paid
            | PublicEquationFixtureMode::PredecessorReplay
            | PublicEquationFixtureMode::AliasPresentation
            | PublicEquationFixtureMode::AmbientBody
            | PublicEquationFixtureMode::BodyfulBeta => vec![successor_equation.clone()],
        };
        for equation in &successor_equation_ids {
            let mut prerequisites = vec![&type_head, &predecessor_term];
            if !equation_only {
                prerequisites.push(&successor_head);
            }
            for prerequisite in prerequisites {
                dependencies.push(inventory_dependency(
                    PublicSubjectV1::Equation {
                        equation: equation.clone(),
                    },
                    prerequisite,
                ));
            }
        }
        let availability = dependencies
            .iter()
            .map(|dependency| {
                let claimed = if dependency.prerequisite == successor_head {
                    PublicAvailabilityV1::DependencyPriorExport {
                        target: dependency.prerequisite.clone(),
                    }
                } else {
                    PublicAvailabilityV1::PredecessorPublicExport {
                        target: dependency.prerequisite.clone(),
                    }
                };
                UncheckedPublicAvailabilityClaimV1 {
                    dependency: dependency.clone(),
                    claimed,
                }
            })
            .collect::<Vec<_>>();

        let successor_equation_owner = if equation_only {
            predecessor_term.clone()
        } else {
            successor_head.clone()
        };
        let successor_source_to_normal = if matches!(mode, PublicEquationFixtureMode::BodyfulBeta) {
            SourceNormalizedJudgmentV1 {
                source_identity: Digest::of_canonical(
                    "pen-semantic-audit/inventory-source-judgment/v1",
                    &bodyful_beta_source,
                ),
                source: bodyful_beta_source,
                claimed_normalized: paid_equation.clone(),
            }
        } else {
            inventory_source_equation(paid_equation.clone())
        };
        let mut successor_equations = vec![UncheckedPublicEquationV1 {
            equation: successor_equation.clone(),
            owner_head: successor_equation_owner.clone(),
            origin: event_successor.clone(),
            source_to_normal: successor_source_to_normal.clone(),
            demand_port: (!matches!(mode, PublicEquationFixtureMode::BodyfulBeta))
                .then_some(demand_port.clone()),
        }];
        if duplicate_mode {
            successor_equations.push(UncheckedPublicEquationV1 {
                equation: duplicate_equation.clone(),
                owner_head: successor_equation_owner.clone(),
                origin: event_successor.clone(),
                source_to_normal: successor_source_to_normal,
                demand_port: Some(demand_port.clone()),
            });
        }
        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history: vec![
                UncheckedPublicHistoryStepV1 {
                    census: UncheckedPublicEventCensusV1 {
                        event: event_type.clone(),
                        added_groups: vec![group_type.clone()],
                        added_declarations: vec![type_head.clone()],
                        added_equations: Vec::new(),
                        added_forced_projections: Vec::new(),
                        added_demand_contracts: Vec::new(),
                    },
                    successor_boundary: boundary_type,
                },
                UncheckedPublicHistoryStepV1 {
                    census: UncheckedPublicEventCensusV1 {
                        event: event_term.clone(),
                        added_groups: vec![group_term.clone()],
                        added_declarations: vec![predecessor_term.clone()],
                        added_equations: vec![predecessor_equation.clone()],
                        added_forced_projections: Vec::new(),
                        added_demand_contracts: vec![demand_contract.clone()],
                    },
                    successor_boundary: predecessor_boundary.clone(),
                },
            ],
            predecessor_boundary,
            successor_event: UncheckedPublicEventCensusV1 {
                event: event_successor.clone(),
                added_groups: if equation_only {
                    Vec::new()
                } else {
                    vec![group_successor.clone()]
                },
                added_declarations: if equation_only {
                    Vec::new()
                } else {
                    vec![successor_head.clone()]
                },
                added_equations: successor_equation_ids.clone(),
                added_forced_projections: Vec::new(),
                added_demand_contracts: Vec::new(),
            },
            successor_boundary,
            declaration_groups: vec![
                UncheckedPublicGroupV1 {
                    group: group_type.clone(),
                    origin: event_type.clone(),
                    declarations: vec![type_head.clone()],
                },
                UncheckedPublicGroupV1 {
                    group: group_term.clone(),
                    origin: event_term.clone(),
                    declarations: vec![predecessor_term.clone()],
                },
            ]
            .into_iter()
            .chain((!equation_only).then(|| UncheckedPublicGroupV1 {
                group: group_successor.clone(),
                origin: event_successor.clone(),
                declarations: vec![successor_head.clone()],
            }))
            .collect(),
            declarations: vec![
                UncheckedPublicDeclarationV1 {
                    declaration: type_head.clone(),
                    origin: event_type.clone(),
                    group: group_type,
                    source_to_normal: inventory_source_declaration(&declaration_type),
                },
                UncheckedPublicDeclarationV1 {
                    declaration: predecessor_term.clone(),
                    origin: event_term.clone(),
                    group: group_term,
                    source_to_normal: inventory_source_declaration(&declaration_term),
                },
            ]
            .into_iter()
            .chain((!equation_only).then(|| UncheckedPublicDeclarationV1 {
                declaration: successor_head.clone(),
                origin: event_successor.clone(),
                group: group_successor.clone(),
                source_to_normal: inventory_source_declaration(&declaration_successor),
            }))
            .collect(),
            equations: std::iter::once(UncheckedPublicEquationV1 {
                equation: predecessor_equation.clone(),
                owner_head: predecessor_term.clone(),
                origin: event_term.clone(),
                source_to_normal: inventory_source_equation(predecessor_judgment),
                demand_port: None,
            })
            .chain(successor_equations)
            .collect(),
            forced_projections: Vec::new(),
            predecessor_demand_contracts: vec![UncheckedPredecessorDemandContractV1 {
                contract: demand_contract,
                origin: event_term.clone(),
                port: demand_port.clone(),
                required_judgment: inventory_source_equation(paid_equation.clone()),
            }],
            public_availability: availability,
            dependency_dag: UncheckedPublicDependencyDagV1 {
                edges: dependencies,
            },
            q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
                schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
                origin_cutoff: Some(event_term),
                entries: Vec::new(),
            },
        };
        let inventory_decision =
            verify_public_audit_inventory_v1(semantic_manifest, &kernel, &wire);
        let AuditDecision::Proven(inventory) = inventory_decision else {
            panic!("public cost inventory fixture verifies: {inventory_decision:?}");
        };

        let head_clause = clause_id("public/head");
        let equation_clause = clause_id("public/equation");
        let duplicate_clause = clause_id("public/equation-duplicate");
        let head_pair = SourceNormalizedDeclarationV1 {
            source_identity: inventory_source_declaration(&declaration_successor).source_identity,
            source_context: DependentContext::default(),
            source_type: declaration_successor.ty.clone(),
            source_body: declaration_successor.body.clone(),
            normalized_context: DependentContext::default(),
            normalized_type: declaration_successor.ty.clone(),
            normalized_body: declaration_successor.body.clone(),
            source_to_normal_derivation: SourceToNormalDerivationV1::Reflexivity,
        };
        let verified_successor_equation = inventory
            .equations()
            .iter()
            .find(|equation| equation.equation() == &successor_equation)
            .expect("successor equation is in the verified inventory");
        let equation_source_identity = verified_successor_equation.source_identity().clone();
        let GenericJudgmentV1::Equation {
            context: source_context,
            left: source_left,
            right: source_right,
            ty: source_type,
        } = verified_successor_equation.source()
        else {
            unreachable!();
        };
        let GenericJudgmentV1::Equation {
            context: normalized_context,
            left: normalized_left,
            right: normalized_right,
            ty: normalized_type,
        } = verified_successor_equation.normalized()
        else {
            unreachable!();
        };
        let equation_pair = SourceNormalizedEquationV1 {
            source_identity: equation_source_identity,
            source_context: source_context.clone(),
            source_left: source_left.clone(),
            source_right: source_right.clone(),
            source_type: source_type.clone(),
            normalized_context: normalized_context.clone(),
            normalized_left: normalized_left.clone(),
            normalized_right: normalized_right.clone(),
            normalized_type: normalized_type.clone(),
            source_to_normal_derivation: if matches!(mode, PublicEquationFixtureMode::BodyfulBeta) {
                SourceToNormalDerivationV1::VerifierReplayedOrdinaryBeta
            } else {
                SourceToNormalDerivationV1::Reflexivity
            },
        };
        let head_raw = RawPublicClauseV1 {
            id: head_clause.clone(),
            semantic_dependencies: BTreeSet::new(),
            clause: RawPublicClauseKindV1::PublicDeclaration {
                head: successor_head.clone(),
                pair: head_pair,
                presentation: match mode {
                    PublicEquationFixtureMode::AliasPresentation => {
                        HeadPresentationV1::TransparentAlias {
                            target: predecessor_term.clone(),
                            availability: PublicAvailabilityV1::PredecessorPublicExport {
                                target: predecessor_term.clone(),
                            },
                        }
                    }
                    PublicEquationFixtureMode::AmbientBody => {
                        HeadPresentationV1::AmbientPrimitiveFirstExport {
                            primitive: crate::model::AmbientPrimitiveV1::Unit,
                        }
                    }
                    PublicEquationFixtureMode::BodyfulBeta => {
                        HeadPresentationV1::TransparentDefinition
                    }
                    _ => HeadPresentationV1::Opaque,
                },
                public_group: group_successor,
                equation_free_descriptors: Vec::new(),
            },
        };
        let equation_raw = RawPublicClauseV1 {
            id: equation_clause.clone(),
            semantic_dependencies: if equation_only {
                BTreeSet::new()
            } else {
                singleton(head_clause.clone())
            },
            clause: RawPublicClauseKindV1::PublicEquation {
                equation: successor_equation.clone(),
                owner_head: successor_equation_owner.clone(),
                demand_output: (!matches!(mode, PublicEquationFixtureMode::BodyfulBeta))
                    .then_some(demand_output.clone()),
                pair: equation_pair.clone(),
            },
        };
        let mut clauses = if equation_only {
            vec![equation_raw]
        } else {
            vec![head_raw, equation_raw]
        };
        let mut public_dependency_dag = if equation_only {
            Vec::new()
        } else {
            vec![PublicDependencyEdgeV1 {
                dependent: equation_clause.clone(),
                prerequisite: head_clause.clone(),
            }]
        };
        let duplicate_clause_option = if duplicate_mode {
            clauses.push(RawPublicClauseV1 {
                id: duplicate_clause.clone(),
                semantic_dependencies: if equation_only {
                    BTreeSet::new()
                } else {
                    singleton(head_clause.clone())
                },
                clause: RawPublicClauseKindV1::PublicEquation {
                    equation: duplicate_equation,
                    owner_head: successor_equation_owner,
                    demand_output: Some(demand_output),
                    pair: equation_pair,
                },
            });
            if !equation_only {
                public_dependency_dag.push(PublicDependencyEdgeV1 {
                    dependent: duplicate_clause.clone(),
                    prerequisite: head_clause.clone(),
                });
            }
            Some(duplicate_clause.clone())
        } else {
            None
        };

        let mut input = KernelCostAuditInputV2 {
            clauses,
            public_dependency_dag,
            equation_demand_ports: successor_equation_ids
                .iter()
                .cloned()
                .map(|equation| EquationDemandPortBindingV2 {
                    equation,
                    demand_port: (!matches!(mode, PublicEquationFixtureMode::BodyfulBeta))
                        .then_some(demand_port.clone()),
                })
                .collect(),
            ..KernelCostAuditInputV2::default()
        };
        match mode {
            PublicEquationFixtureMode::Paid => {
                input.negative_evidence = vec![
                    negative_v2(
                        head_clause.clone(),
                        singleton(equation_clause.clone()),
                        singleton(equation_clause.clone()),
                    ),
                    negative_v2(
                        equation_clause.clone(),
                        singleton(head_clause.clone()),
                        singleton(head_clause.clone()),
                    ),
                ];
            }
            PublicEquationFixtureMode::PredecessorReplay => {
                input.negative_evidence = vec![negative_v2(
                    head_clause.clone(),
                    BTreeSet::new(),
                    singleton(equation_clause.clone()),
                )];
            }
            PublicEquationFixtureMode::Duplicate => {
                let duplicate = duplicate_clause_option
                    .as_ref()
                    .expect("duplicate fixture has a second equation");
                input.reconstructions = vec![duplicate_rule_v2(
                    duplicate.clone(),
                    equation_clause.clone(),
                )];
                input.presentation_equivalences = vec![PresentationEquivalenceV2 {
                    representative: equation_clause.clone(),
                    equivalent: duplicate.clone(),
                    proof: PresentationEquivalenceProofV2::DuplicateNormalizedEquation,
                }];
                input.negative_evidence = vec![
                    negative_v2(
                        head_clause.clone(),
                        singleton(equation_clause.clone()),
                        [equation_clause.clone(), duplicate.clone()]
                            .into_iter()
                            .collect(),
                    ),
                    negative_v2(
                        equation_clause.clone(),
                        singleton(head_clause.clone()),
                        singleton(head_clause.clone()),
                    ),
                ];
            }
            PublicEquationFixtureMode::AliasPresentation => {
                input.negative_evidence = vec![negative_v2(
                    equation_clause.clone(),
                    BTreeSet::new(),
                    singleton(head_clause.clone()),
                )];
            }
            PublicEquationFixtureMode::PredecessorOwnerDuplicate => {
                let duplicate = duplicate_clause_option
                    .as_ref()
                    .expect("equation-only duplicate fixture has a second equation");
                input.reconstructions = vec![duplicate_rule_v2(
                    duplicate.clone(),
                    equation_clause.clone(),
                )];
                input.presentation_equivalences = vec![PresentationEquivalenceV2 {
                    representative: equation_clause.clone(),
                    equivalent: duplicate.clone(),
                    proof: PresentationEquivalenceProofV2::DuplicateNormalizedEquation,
                }];
                input.negative_evidence = vec![negative_v2(
                    equation_clause.clone(),
                    BTreeSet::new(),
                    BTreeSet::new(),
                )];
            }
            PublicEquationFixtureMode::AmbientBody => {
                input.negative_evidence = vec![
                    negative_v2(
                        head_clause.clone(),
                        singleton(equation_clause.clone()),
                        singleton(equation_clause.clone()),
                    ),
                    negative_v2(
                        equation_clause.clone(),
                        singleton(head_clause.clone()),
                        singleton(head_clause.clone()),
                    ),
                ];
            }
            PublicEquationFixtureMode::BodyfulBeta => {
                input.negative_evidence = vec![
                    negative_v2(
                        head_clause.clone(),
                        singleton(equation_clause.clone()),
                        singleton(equation_clause.clone()),
                    ),
                    negative_v2(
                        equation_clause.clone(),
                        singleton(head_clause.clone()),
                        singleton(head_clause.clone()),
                    ),
                ];
            }
        }
        PublicV2Fixture {
            inventory,
            input,
            head_clause,
            equation_clause,
            duplicate_clause: duplicate_clause_option,
            predecessor_equation,
            successor_equation,
        }
    }

    fn verified_manifest() -> VerifiedCostManifestV1 {
        let AuditDecision::Proven(manifest) =
            verify_kernel_cost_manifest_v1(&proposed_kernel_cost_manifest_v1())
        else {
            panic!("proposed generic manifest must verify");
        };
        manifest
    }

    fn verified_manifest_v2() -> VerifiedCostManifestV2 {
        let AuditDecision::Proven(manifest) =
            verify_kernel_cost_manifest_v2(&proposed_kernel_cost_manifest_v2())
        else {
            panic!("proposed generic V2 manifest must verify");
        };
        manifest
    }

    fn verified_lambda_unit_manifest_v2() -> VerifiedCostManifestV2 {
        let AuditDecision::Proven(manifest) = verify_kernel_cost_lambda_unit_manifest_v2(
            &proposed_kernel_cost_lambda_unit_manifest_v2(),
        ) else {
            panic!("proposed lambda/unit V2 manifest must verify");
        };
        manifest
    }

    fn declaration_pair(label: &str, body: Option<Term>) -> SourceNormalizedDeclarationV1 {
        SourceNormalizedDeclarationV1 {
            source_identity: digest(&format!("source/{label}")),
            source_context: DependentContext::default(),
            source_type: Term::Sort { level: 0 },
            source_body: body.clone(),
            normalized_context: DependentContext::default(),
            normalized_type: Term::Sort { level: 0 },
            normalized_body: body,
            source_to_normal_derivation: SourceToNormalDerivationV1::Reflexivity,
        }
    }

    fn declaration(
        label: &str,
        presentation: HeadPresentationV1,
        body: Option<Term>,
    ) -> RawPublicClauseV1 {
        declaration_in_group(label, &format!("group/{label}"), presentation, body)
    }

    fn declaration_in_group(
        label: &str,
        group: &str,
        presentation: HeadPresentationV1,
        body: Option<Term>,
    ) -> RawPublicClauseV1 {
        RawPublicClauseV1 {
            id: clause_id(label),
            semantic_dependencies: BTreeSet::new(),
            clause: RawPublicClauseKindV1::PublicDeclaration {
                head: global(label),
                pair: declaration_pair(label, body),
                presentation,
                public_group: global(group),
                equation_free_descriptors: Vec::new(),
            },
        }
    }

    fn transparent_definition(label: &str) -> RawPublicClauseV1 {
        declaration(
            label,
            HeadPresentationV1::TransparentDefinition,
            Some(Term::Unit),
        )
    }

    fn negative(
        target: ClauseIdV1,
        against: BTreeSet<ClauseIdV1>,
        realized: BTreeSet<ClauseIdV1>,
    ) -> CompleteNegativeEvidenceV1 {
        CompleteNegativeEvidenceV1 {
            target,
            against,
            realized,
            checked_rules: proposed_kernel_cost_manifest_v1().free_completion_rules,
        }
    }

    fn singleton_negative(target: ClauseIdV1) -> CompleteNegativeEvidenceV1 {
        negative(target, BTreeSet::new(), BTreeSet::new())
    }

    fn negative_v2(
        target: ClauseIdV1,
        against: BTreeSet<ClauseIdV1>,
        realized: BTreeSet<ClauseIdV1>,
    ) -> CompleteNegativeEvidenceV2 {
        CompleteNegativeEvidenceV2 {
            target,
            against,
            realized,
            checked_rules: proposed_kernel_cost_manifest_v2().free_completion_rules,
        }
    }

    fn audit(input: &KernelCostAuditInputV1) -> AuditDecision<CostAuditCertificateV1> {
        audit_kernel_cost_v1(&verified_manifest(), input)
    }

    fn proven(input: &KernelCostAuditInputV1) -> CostAuditCertificateV1 {
        let AuditDecision::Proven(certificate) = audit(input) else {
            panic!("vector should be proven");
        };
        certificate
    }

    fn audit_v2(input: &KernelCostAuditInputV2) -> AuditDecision<CostAuditCertificateV2> {
        audit_kernel_cost_v2_fixture(&verified_manifest_v2(), input)
    }

    fn proven_v2(input: &KernelCostAuditInputV2) -> CostAuditCertificateV2 {
        let AuditDecision::Proven(certificate) = audit_v2(input) else {
            panic!("V2 vector should be proven");
        };
        certificate
    }

    fn disposition(
        certificate: &CostAuditCertificateV1,
        clause: &ClauseIdV1,
    ) -> ClauseCostDispositionV1 {
        certificate
            .dispositions
            .iter()
            .find(|entry| &entry.clause == clause)
            .expect("every raw clause must have one disposition")
            .disposition
            .clone()
    }

    fn alias_rule(
        declaration: ClauseIdV1,
        target: GlobalId,
        premises: BTreeSet<ClauseIdV1>,
    ) -> ReconstructionDerivationV1 {
        ReconstructionDerivationV1 {
            output: declaration.clone(),
            premises,
            proof: ReconstructionProofV1::PriorPublicTransparentAlias {
                declaration,
                target,
            },
        }
    }

    fn duplicate_rule(output: ClauseIdV1, original: ClauseIdV1) -> ReconstructionDerivationV1 {
        ReconstructionDerivationV1 {
            output,
            premises: singleton(original.clone()),
            proof: ReconstructionProofV1::DuplicatePresentationDeletion { original },
        }
    }

    fn duplicate_rule_v2(output: ClauseIdV1, original: ClauseIdV1) -> ReconstructionDerivationV2 {
        ReconstructionDerivationV2 {
            output,
            premises: singleton(original.clone()),
            proof: ReconstructionProofV2::DuplicatePresentationDeletion { original },
        }
    }

    fn fresh_vector(with_theorem: bool) -> (KernelCostAuditInputV1, ClauseIdV1, ClauseIdV1) {
        let head_id = clause_id("fresh/head");
        let equation_clause = clause_id("fresh/equation");
        let head = global("fresh/head");
        let constructor = global("fresh/constructor");
        let result = global("fresh/result");
        let result_type = global("fresh/result-type");
        let demand = demand_id("fresh/contract");
        let descriptor = EquationFreeDescriptorV1 {
            owner: head.clone(),
            source_clause: head_id.clone(),
            descriptor: EquationFreeDescriptorKindV1::FreshComputationContract {
                demand_output: demand.clone(),
                constructor: constructor.clone(),
                result: result.clone(),
                result_type: result_type.clone(),
                computation_mode: ComputationModeV1::ClosedNullary,
            },
            public_support: [constructor.clone(), result.clone(), result_type.clone()]
                .into_iter()
                .collect(),
        };
        let head_clause = RawPublicClauseV1 {
            id: head_id.clone(),
            semantic_dependencies: BTreeSet::new(),
            clause: RawPublicClauseKindV1::PublicDeclaration {
                head: head.clone(),
                pair: declaration_pair("fresh/head", None),
                presentation: HeadPresentationV1::Opaque,
                public_group: global("fresh/group"),
                equation_free_descriptors: if with_theorem {
                    vec![descriptor]
                } else {
                    Vec::new()
                },
            },
        };
        let left = Term::Apply {
            function: Box::new(Term::Global { id: head.clone() }),
            argument: Box::new(Term::Global {
                id: constructor.clone(),
            }),
        };
        let equation_pair = SourceNormalizedEquationV1 {
            source_identity: digest("source/fresh/equation"),
            source_context: DependentContext::default(),
            source_left: left.clone(),
            source_right: Term::Global { id: result.clone() },
            source_type: Term::Global {
                id: result_type.clone(),
            },
            normalized_context: DependentContext::default(),
            normalized_left: left,
            normalized_right: Term::Global { id: result },
            normalized_type: Term::Global { id: result_type },
            source_to_normal_derivation: SourceToNormalDerivationV1::Reflexivity,
        };
        let equation = RawPublicClauseV1 {
            id: equation_clause.clone(),
            semantic_dependencies: singleton(head_id.clone()),
            clause: RawPublicClauseKindV1::PublicEquation {
                equation: equation_id("fresh/equation"),
                owner_head: head,
                demand_output: Some(demand.clone()),
                pair: equation_pair,
            },
        };
        let reconstructions = if with_theorem {
            vec![ReconstructionDerivationV1 {
                output: equation_clause.clone(),
                premises: singleton(head_id.clone()),
                proof: ReconstructionProofV1::CertifiedFreshConstructorComputation {
                    theorem: ExactFreshCompletionProofV1 {
                        head: head_id.clone(),
                        demand_output: demand,
                        typing: FreshTypingProofV1::ClosedPublicReferences,
                        substitution_stability: FreshSubstitutionProofV1::ClosedEquation,
                        rewrite_system: FreshRewriteProofV1::FreshHeadSingleNonrecursiveRule,
                        exact_api_round_trip:
                            FreshRoundTripProofV1::DeleteThenDeterministicallyRegenerate,
                        universal_property: FreshUniversalPropertyV1::UniqueClosedDemandExtension,
                        transport_invariance: FreshTransportProofV1::StructuralQ0Q2Q3,
                    },
                },
            }]
        } else {
            Vec::new()
        };
        (
            KernelCostAuditInputV1 {
                clauses: vec![head_clause, equation],
                public_dependency_dag: Vec::new(),
                reconstructions,
                negative_evidence: Vec::new(),
                presentation_equivalences: Vec::new(),
            },
            head_id,
            equation_clause,
        )
    }

    #[test]
    fn vector_01_first_public_export_of_ambient_primitive_costs_one() {
        let id = clause_id("ambient-unit-type");
        let clause = declaration(
            "ambient-unit-type",
            HeadPresentationV1::AmbientPrimitiveFirstExport {
                primitive: AmbientPrimitiveV1::UnitType,
            },
            Some(Term::UnitType),
        );
        let certificate = proven(&KernelCostAuditInputV1 {
            clauses: vec![clause],
            negative_evidence: vec![singleton_negative(id.clone())],
            ..KernelCostAuditInputV1::default()
        });
        assert_eq!(certificate.kernel_cost, 1);
        assert_eq!(
            disposition(&certificate, &id),
            ClauseCostDispositionV1::FirstIrreducible
        );
    }

    #[test]
    fn vector_02_predecessor_public_alias_is_free() {
        let id = clause_id("alias");
        let target = global("predecessor-target");
        let clause = declaration(
            "alias",
            HeadPresentationV1::TransparentAlias {
                target: target.clone(),
                availability: PublicAvailabilityV1::PredecessorPublicExport {
                    target: target.clone(),
                },
            },
            Some(Term::Global { id: target.clone() }),
        );
        let certificate = proven(&KernelCostAuditInputV1 {
            clauses: vec![clause],
            reconstructions: vec![alias_rule(id.clone(), target.clone(), BTreeSet::new())],
            ..KernelCostAuditInputV1::default()
        });
        assert_eq!(certificate.kernel_cost, 0);
        assert_eq!(
            disposition(&certificate, &id),
            ClauseCostDispositionV1::TransparentAlias { target }
        );
    }

    #[test]
    fn vector_03_bodyful_definition_is_one_clause_and_beta_is_implicit() {
        let id = clause_id("definition");
        let certificate = proven(&KernelCostAuditInputV1 {
            clauses: vec![transparent_definition("definition")],
            negative_evidence: vec![singleton_negative(id.clone())],
            ..KernelCostAuditInputV1::default()
        });
        assert_eq!(certificate.kernel_cost, 1);
        assert_eq!(certificate.dispositions.len(), 1);
        assert_eq!(
            disposition(&certificate, &id),
            ClauseCostDispositionV1::FirstIrreducible
        );
    }

    #[test]
    fn explicit_ordinary_beta_clause_cannot_omit_its_free_reconstruction() {
        let definition = clause_id("definition-with-equation");
        let equation = RawPublicClauseV1 {
            id: clause_id("definition-with-equation/beta"),
            semantic_dependencies: singleton(definition.clone()),
            clause: RawPublicClauseKindV1::PublicEquation {
                equation: equation_id("definition-with-equation/beta"),
                owner_head: global("definition-with-equation"),
                demand_output: None,
                pair: SourceNormalizedEquationV1 {
                    source_identity: digest("source/definition-with-equation/beta"),
                    source_context: DependentContext::default(),
                    source_left: Term::Global {
                        id: global("definition-with-equation"),
                    },
                    source_right: Term::Unit,
                    source_type: Term::Sort { level: 0 },
                    normalized_context: DependentContext::default(),
                    normalized_left: Term::Global {
                        id: global("definition-with-equation"),
                    },
                    normalized_right: Term::Unit,
                    normalized_type: Term::Sort { level: 0 },
                    source_to_normal_derivation: SourceToNormalDerivationV1::Reflexivity,
                },
            },
        };
        let input = KernelCostAuditInputV1 {
            clauses: vec![transparent_definition("definition-with-equation"), equation],
            ..KernelCostAuditInputV1::default()
        };
        assert!(matches!(
            audit(&input),
            AuditDecision::Unknown(AuditUnknownReason::MissingTupleDisposition)
        ));
    }

    #[test]
    fn vector_04_fresh_equation_waits_for_free_completion_theorem() {
        let (mut input, head, _equation) = fresh_vector(true);
        input
            .negative_evidence
            .push(singleton_negative(head.clone()));
        assert!(matches!(
            audit(&input),
            AuditDecision::Unknown(AuditUnknownReason::MissingFreeCompletionTheorem)
        ));
    }

    #[test]
    fn applicable_fresh_completion_cannot_be_omitted_and_charged() {
        let (mut input, _, _) = fresh_vector(true);
        input.reconstructions.clear();
        assert!(matches!(
            audit(&input),
            AuditDecision::Unknown(AuditUnknownReason::MissingTupleDisposition)
        ));
    }

    #[test]
    fn vector_05_separated_fresh_equation_costs_and_missing_evidence_is_unknown() {
        let (mut input, head, equation) = fresh_vector(false);
        input.negative_evidence = vec![
            negative(
                head.clone(),
                singleton(equation.clone()),
                singleton(equation.clone()),
            ),
            negative(
                equation.clone(),
                singleton(head.clone()),
                singleton(head.clone()),
            ),
        ];
        let certificate = proven(&input);
        assert_eq!(certificate.kernel_cost, 2);
        assert_eq!(
            disposition(&certificate, &equation),
            ClauseCostDispositionV1::FirstIrreducible
        );

        input.negative_evidence.pop();
        assert!(matches!(
            audit(&input),
            AuditDecision::Unknown(AuditUnknownReason::MissingNegativeEvidence)
        ));
    }

    #[test]
    fn vector_06_descriptor_forces_projection() {
        let owner = clause_id("record");
        let projection = clause_id("record/projection");
        let record_head = global("record");
        let field_type = global("record/field-type");
        let descriptor = EquationFreeDescriptorV1 {
            owner: record_head.clone(),
            source_clause: owner.clone(),
            descriptor: EquationFreeDescriptorKindV1::RecordField {
                record: record_head.clone(),
                field_ordinal: 0,
                field_type: field_type.clone(),
            },
            public_support: [record_head.clone(), field_type].into_iter().collect(),
        };
        let owner_clause = RawPublicClauseV1 {
            id: owner.clone(),
            semantic_dependencies: BTreeSet::new(),
            clause: RawPublicClauseKindV1::PublicDeclaration {
                head: record_head,
                pair: declaration_pair("record", None),
                presentation: HeadPresentationV1::Opaque,
                public_group: global("record/group"),
                equation_free_descriptors: vec![descriptor],
            },
        };
        let projection_clause = RawPublicClauseV1 {
            id: projection.clone(),
            semantic_dependencies: singleton(owner.clone()),
            clause: RawPublicClauseKindV1::ForcedProjectionClause {
                projection: global("record/projection"),
                record_owner: owner.clone(),
                field_ordinal: 0,
                pair: declaration_pair("record/projection", None),
            },
        };
        let certificate = proven(&KernelCostAuditInputV1 {
            clauses: vec![owner_clause, projection_clause],
            reconstructions: vec![ReconstructionDerivationV1 {
                output: projection.clone(),
                premises: singleton(owner.clone()),
                proof: ReconstructionProofV1::DescriptorForcedProjection {
                    descriptor_owner: owner.clone(),
                    field_ordinal: 0,
                },
            }],
            negative_evidence: vec![singleton_negative(owner)],
            ..KernelCostAuditInputV1::default()
        });
        assert_eq!(certificate.kernel_cost, 1);
        assert_eq!(
            disposition(&certificate, &projection),
            ClauseCostDispositionV1::ForcedProjection
        );
    }

    #[test]
    fn vector_07_duplicate_transparent_field_is_free() {
        let original = clause_id("field/original");
        let duplicate = clause_id("field/duplicate");
        let group = "field/group";
        let original_clause = declaration_in_group(
            "field/original",
            group,
            HeadPresentationV1::TransparentDefinition,
            Some(Term::Unit),
        );
        let duplicate_clause = declaration_in_group(
            "field/duplicate",
            group,
            HeadPresentationV1::TransparentDefinition,
            Some(Term::Unit),
        );
        let certificate = proven(&KernelCostAuditInputV1 {
            clauses: vec![duplicate_clause, original_clause],
            reconstructions: vec![duplicate_rule(duplicate.clone(), original.clone())],
            negative_evidence: vec![singleton_negative(original.clone())],
            ..KernelCostAuditInputV1::default()
        });
        assert_eq!(certificate.kernel_cost, 1);
        assert_eq!(
            disposition(&certificate, &duplicate),
            ClauseCostDispositionV1::DuplicatePresentation { original }
        );
    }

    #[test]
    fn vector_08_mutually_dependent_scc_can_have_two_irreducibles() {
        let left = clause_id("mutual/left");
        let right = clause_id("mutual/right");
        let mut left_clause = transparent_definition("mutual/left");
        left_clause.semantic_dependencies.insert(right.clone());
        let mut right_clause = transparent_definition("mutual/right");
        right_clause.semantic_dependencies.insert(left.clone());
        let certificate = proven(&KernelCostAuditInputV1 {
            clauses: vec![left_clause, right_clause],
            negative_evidence: vec![
                negative(
                    left.clone(),
                    singleton(right.clone()),
                    singleton(right.clone()),
                ),
                negative(
                    right.clone(),
                    singleton(left.clone()),
                    singleton(left.clone()),
                ),
            ],
            ..KernelCostAuditInputV1::default()
        });
        assert_eq!(certificate.kernel_cost, 2);
        assert!(certificate.dependency_components.iter().any(|component| {
            component.members == [left.clone(), right.clone()].into_iter().collect()
        }));
    }

    #[test]
    fn vector_09_independent_declaration_order_is_invariant() {
        let left = clause_id("order/left");
        let right = clause_id("order/right");
        let left_clause = transparent_definition("order/left");
        let right_clause = transparent_definition("order/right");
        let evidence = vec![
            negative(
                left.clone(),
                singleton(right.clone()),
                singleton(right.clone()),
            ),
            negative(
                right.clone(),
                singleton(left.clone()),
                singleton(left.clone()),
            ),
        ];
        let forward = proven(&KernelCostAuditInputV1 {
            clauses: vec![left_clause.clone(), right_clause.clone()],
            negative_evidence: evidence.clone(),
            ..KernelCostAuditInputV1::default()
        });
        let reversed = proven(&KernelCostAuditInputV1 {
            clauses: vec![right_clause, left_clause],
            negative_evidence: evidence.into_iter().rev().collect(),
            ..KernelCostAuditInputV1::default()
        });
        assert_eq!(forward, reversed);
        assert_eq!(forward.certificate_digest(), reversed.certificate_digest());
    }

    #[test]
    fn vector_10_inequivalent_minimal_bases_are_unknown() {
        let left = clause_id("nonunique/left");
        let right = clause_id("nonunique/right");
        let group = "nonunique/group";
        let left_clause = declaration_in_group(
            "nonunique/left",
            group,
            HeadPresentationV1::TransparentDefinition,
            Some(Term::Unit),
        );
        let right_clause = declaration_in_group(
            "nonunique/right",
            group,
            HeadPresentationV1::TransparentDefinition,
            Some(Term::Unit),
        );
        let input = KernelCostAuditInputV1 {
            clauses: vec![left_clause, right_clause],
            reconstructions: vec![
                duplicate_rule(left.clone(), right.clone()),
                duplicate_rule(right.clone(), left.clone()),
            ],
            negative_evidence: vec![singleton_negative(left), singleton_negative(right)],
            ..KernelCostAuditInputV1::default()
        };
        assert!(matches!(
            audit(&input),
            AuditDecision::Unknown(AuditUnknownReason::NonUniqueBasis)
        ));
    }

    #[test]
    fn v2_public_paid_equation_binds_exact_inventory() {
        let fixture = public_equation_fixture(PublicEquationFixtureMode::Paid);
        assert!(matches!(
            audit_kernel_cost_v2(
                &verified_lambda_unit_manifest_v2(),
                &fixture.inventory,
                &fixture.input
            ),
            AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch)
        ));
        let AuditDecision::Proven(certificate) =
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &fixture.input)
        else {
            panic!("inventory-bound paid equation vector should verify");
        };
        assert_eq!(certificate.kernel_cost(), 2);
        assert_eq!(certificate.inventory_digest(), fixture.inventory.digest());
        assert_eq!(
            disposition_v2(&certificate, &fixture.head_clause),
            ClauseCostDispositionV1::FirstIrreducible
        );
        assert_eq!(
            disposition_v2(&certificate, &fixture.equation_clause),
            ClauseCostDispositionV1::FirstIrreducible
        );

        let mut forged_source = fixture.input.clone();
        let RawPublicClauseKindV1::PublicEquation { pair, .. } = &mut forged_source
            .clauses
            .iter_mut()
            .find(|clause| clause.id == fixture.equation_clause)
            .expect("equation exists")
            .clause
        else {
            unreachable!();
        };
        pair.source_left = Term::Unit;
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &forged_source),
            AuditDecision::Unknown(AuditUnknownReason::NormalizationFailure)
        ));
    }

    #[test]
    fn v2_public_presentation_is_derived_from_inventory_not_caller_tag() {
        let fixture = public_equation_fixture(PublicEquationFixtureMode::AliasPresentation);
        let AuditDecision::Proven(certificate) =
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &fixture.input)
        else {
            panic!("inventory-derived alias presentation should verify");
        };
        assert_eq!(certificate.kernel_cost(), 1);
        assert_eq!(
            disposition_v2(&certificate, &fixture.head_clause),
            ClauseCostDispositionV1::TransparentAlias {
                target: global("public/a")
            }
        );

        let mut contradiction = fixture.input.clone();
        let RawPublicClauseKindV1::PublicDeclaration { presentation, .. } = &mut contradiction
            .clauses
            .iter_mut()
            .find(|clause| clause.id == fixture.head_clause)
            .expect("head exists")
            .clause
        else {
            unreachable!();
        };
        *presentation = HeadPresentationV1::TransparentDefinition;
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &contradiction),
            AuditDecision::Unknown(AuditUnknownReason::NormalizationFailure)
        ));

        let ambient = public_equation_fixture(PublicEquationFixtureMode::AmbientBody);
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &ambient.inventory, &ambient.input),
            AuditDecision::Unknown(AuditUnknownReason::MissingAmbientFirstExportInventory)
        ));

        let beta = public_equation_fixture(PublicEquationFixtureMode::BodyfulBeta);
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &beta.inventory, &beta.input),
            AuditDecision::Unknown(AuditUnknownReason::MissingOrdinaryBetaDerivation)
        ));
    }

    #[test]
    fn lambda_unit_cost_accepts_only_inventory_minted_ordinary_beta() {
        let AuditDecision::Proven(semantic_manifest) =
            verify_semantic_audit_lambda_unit_manifest_v1(
                &proposed_semantic_audit_lambda_unit_manifest_v1(),
            )
        else {
            panic!("lambda/unit semantic manifest");
        };
        let AuditDecision::Proven(cost_manifest) = verify_kernel_cost_lambda_unit_manifest_v2(
            &proposed_kernel_cost_lambda_unit_manifest_v2(),
        ) else {
            panic!("lambda/unit cost manifest");
        };
        let mut fixture = public_equation_fixture_for_manifest(
            PublicEquationFixtureMode::BodyfulBeta,
            &semantic_manifest,
        );
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let AuditDecision::Proven(beta) = verify_ordinary_beta_derivation_v1(
            &semantic_manifest,
            &kernel,
            &fixture.inventory,
            &fixture.successor_equation,
        ) else {
            panic!("ordinary beta capability");
        };
        fixture.input.reconstructions = vec![ReconstructionDerivationV2 {
            output: fixture.equation_clause.clone(),
            premises: singleton(fixture.head_clause.clone()),
            proof: ReconstructionProofV2::OrdinaryBetaOfBodyfulDefinition {
                definition: fixture.head_clause.clone(),
            },
        }];
        fixture.input.negative_evidence = vec![negative_v2(
            fixture.head_clause.clone(),
            BTreeSet::new(),
            BTreeSet::new(),
        )];
        fixture.input.negative_evidence[0].checked_rules =
            cost_manifest.manifest().free_completion_rules.clone();

        assert!(matches!(
            audit_kernel_cost_lambda_unit_v2(
                &cost_manifest,
                &semantic_manifest,
                &fixture.inventory,
                &[],
                &fixture.input,
            ),
            AuditDecision::Unknown(AuditUnknownReason::MissingOrdinaryBetaDerivation)
        ));

        let decision = audit_kernel_cost_lambda_unit_v2(
            &cost_manifest,
            &semantic_manifest,
            &fixture.inventory,
            std::slice::from_ref(&beta),
            &fixture.input,
        );
        let AuditDecision::Proven(certificate) = decision else {
            panic!("inventory-minted ordinary beta should close the cost rule: {decision:?}");
        };
        assert_eq!(certificate.kernel_cost(), 1);
        assert_eq!(
            disposition_v2(&certificate, &fixture.equation_clause),
            ClauseCostDispositionV1::ForcedDefinitionalCompletion
        );

        assert!(matches!(
            audit_kernel_cost_lambda_unit_v2(
                &cost_manifest,
                &semantic_manifest,
                &fixture.inventory,
                &[beta.clone(), beta],
                &fixture.input,
            ),
            AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision)
        ));
    }

    #[test]
    fn lambda_unit_cost_derives_first_ambient_export_from_complete_census() {
        let AuditDecision::Proven(semantic_manifest) =
            verify_semantic_audit_lambda_unit_manifest_v1(
                &proposed_semantic_audit_lambda_unit_manifest_v1(),
            )
        else {
            panic!("lambda/unit semantic manifest");
        };
        let AuditDecision::Proven(cost_manifest) = verify_kernel_cost_lambda_unit_manifest_v2(
            &proposed_kernel_cost_lambda_unit_manifest_v2(),
        ) else {
            panic!("lambda/unit cost manifest");
        };
        let mut fixture = public_equation_fixture_for_manifest(
            PublicEquationFixtureMode::AmbientBody,
            &semantic_manifest,
        );
        for evidence in &mut fixture.input.negative_evidence {
            evidence.checked_rules = cost_manifest.manifest().free_completion_rules.clone();
        }

        let AuditDecision::Proven(certificate) = audit_kernel_cost_lambda_unit_v2(
            &cost_manifest,
            &semantic_manifest,
            &fixture.inventory,
            &[],
            &fixture.input,
        ) else {
            panic!("complete ambient census should close first-export classification");
        };
        assert_eq!(certificate.kernel_cost(), 2);
        assert_eq!(
            disposition_v2(&certificate, &fixture.head_clause),
            ClauseCostDispositionV1::FirstIrreducible
        );
    }

    #[test]
    fn lambda_unit_cost_quotients_simultaneous_ambient_exports_without_a_representative() {
        let AuditDecision::Proven(semantic_manifest) =
            verify_semantic_audit_lambda_unit_manifest_v1(
                &proposed_semantic_audit_lambda_unit_manifest_v1(),
            )
        else {
            panic!("lambda/unit semantic manifest");
        };
        let AuditDecision::Proven(cost_manifest) = verify_kernel_cost_lambda_unit_manifest_v2(
            &proposed_kernel_cost_lambda_unit_manifest_v2(),
        ) else {
            panic!("lambda/unit cost manifest");
        };
        let first = ambient_declaration("ambient-cost/simultaneous-a", Term::UnitType);
        let second = ambient_declaration("ambient-cost/simultaneous-b", Term::UnitType);
        let first_clause = clause_id("ambient-cost/simultaneous-a");
        let second_clause = clause_id("ambient-cost/simultaneous-b");
        let fixture = ambient_cost_fixture(
            &semantic_manifest,
            &cost_manifest,
            Vec::new(),
            vec![first, second],
            vec![first_clause.clone(), second_clause.clone()],
        );

        let AuditDecision::Proven(forward) = audit_kernel_cost_lambda_unit_v2(
            &cost_manifest,
            &semantic_manifest,
            &fixture.inventory,
            &[],
            &fixture.input,
        ) else {
            panic!("simultaneous first-export class should have a proven quotient");
        };
        assert_eq!(forward.kernel_cost(), 1);
        assert_eq!(
            forward.basis_classes(),
            &[[first_clause.clone(), second_clause.clone()]
                .into_iter()
                .collect()]
        );
        assert_eq!(forward.basis_representatives().len(), 2);
        assert!(
            forward
                .basis_representatives()
                .contains(&singleton(first_clause.clone()))
        );
        assert!(
            forward
                .basis_representatives()
                .contains(&singleton(second_clause.clone()))
        );
        for clause in [&first_clause, &second_clause] {
            let disposition = forward
                .dispositions()
                .iter()
                .find(|entry| entry.clause() == clause)
                .expect("both direct exports have a disposition");
            assert_eq!(
                disposition.disposition(),
                &ClauseCostDispositionV2::AmbientFirstExportClassMember {
                    primitive_class_digest: fixture.q2_class_digest.clone(),
                }
            );
            assert_eq!(disposition.closure_round(), 0);
            assert!(disposition.reconstructions().is_empty());
        }

        let mut reversed_input = fixture.input.clone();
        reversed_input.clauses.reverse();
        reversed_input.negative_evidence.reverse();
        let AuditDecision::Proven(reversed) = audit_kernel_cost_lambda_unit_v2(
            &cost_manifest,
            &semantic_manifest,
            &fixture.inventory,
            &[],
            &reversed_input,
        ) else {
            panic!("caller declaration order must not affect the quotient");
        };
        assert_eq!(forward, reversed);
        assert_eq!(forward.certificate_digest(), reversed.certificate_digest());
    }

    #[test]
    fn ambient_class_digest_clause_id_collisions_do_not_select_a_representative() {
        let AuditDecision::Proven(semantic_manifest) =
            verify_semantic_audit_lambda_unit_manifest_v1(
                &proposed_semantic_audit_lambda_unit_manifest_v1(),
            )
        else {
            panic!("lambda/unit semantic manifest");
        };
        let AuditDecision::Proven(cost_manifest) = verify_kernel_cost_lambda_unit_manifest_v2(
            &proposed_kernel_cost_lambda_unit_manifest_v2(),
        ) else {
            panic!("lambda/unit cost manifest");
        };
        let first = ambient_declaration("ambient-cost/collision-a", Term::UnitType);
        let second = ambient_declaration("ambient-cost/collision-b", Term::UnitType);
        let probe = ambient_cost_fixture(
            &semantic_manifest,
            &cost_manifest,
            Vec::new(),
            vec![first.clone(), second.clone()],
            vec![
                clause_id("ambient-cost/collision-probe-a"),
                clause_id("ambient-cost/collision-probe-b"),
            ],
        );

        for colliding_digest in [probe.q2_class_digest, probe.class_digest] {
            let colliding_clause = ClauseIdV1(colliding_digest);
            let peer_clause = clause_id("ambient-cost/collision-peer");
            assert_ne!(colliding_clause, peer_clause);
            let fixture = ambient_cost_fixture(
                &semantic_manifest,
                &cost_manifest,
                Vec::new(),
                vec![first.clone(), second.clone()],
                vec![colliding_clause.clone(), peer_clause.clone()],
            );
            let AuditDecision::Proven(certificate) = audit_kernel_cost_lambda_unit_v2(
                &cost_manifest,
                &semantic_manifest,
                &fixture.inventory,
                &[],
                &fixture.input,
            ) else {
                panic!("a caller ClauseId/digest collision must not change the quotient");
            };

            assert_eq!(certificate.kernel_cost(), 1);
            assert_eq!(certificate.basis_representatives().len(), 2);
            for clause in [&colliding_clause, &peer_clause] {
                let disposition = certificate
                    .dispositions()
                    .iter()
                    .find(|entry| entry.clause() == clause)
                    .expect("both peers have a disposition");
                assert_eq!(
                    disposition.disposition(),
                    &ClauseCostDispositionV2::AmbientFirstExportClassMember {
                        primitive_class_digest: fixture.q2_class_digest.clone(),
                    }
                );
            }
        }
    }

    #[test]
    fn prior_ambient_reexport_keeps_predecessor_digest_out_of_clause_id_space() {
        let AuditDecision::Proven(semantic_manifest) =
            verify_semantic_audit_lambda_unit_manifest_v1(
                &proposed_semantic_audit_lambda_unit_manifest_v1(),
            )
        else {
            panic!("lambda/unit semantic manifest");
        };
        let AuditDecision::Proven(cost_manifest) = verify_kernel_cost_lambda_unit_manifest_v2(
            &proposed_kernel_cost_lambda_unit_manifest_v2(),
        ) else {
            panic!("lambda/unit cost manifest");
        };
        let predecessor = ambient_declaration("ambient-cost/prior-predecessor", Term::UnitType);
        let successor = ambient_declaration("ambient-cost/prior-successor", Term::UnitType);
        let probe = ambient_cost_fixture(
            &semantic_manifest,
            &cost_manifest,
            vec![predecessor.clone()],
            vec![successor.clone()],
            vec![clause_id("ambient-cost/prior-probe")],
        );
        let predecessor_class_digest = probe
            .predecessor_q2_class_digest
            .expect("prior reexport carries its predecessor class digest");
        let colliding_clause = ClauseIdV1(predecessor_class_digest.clone());
        let fixture = ambient_cost_fixture(
            &semantic_manifest,
            &cost_manifest,
            vec![predecessor],
            vec![successor],
            vec![colliding_clause.clone()],
        );
        let AuditDecision::Proven(certificate) = audit_kernel_cost_lambda_unit_v2(
            &cost_manifest,
            &semantic_manifest,
            &fixture.inventory,
            &[],
            &fixture.input,
        ) else {
            panic!("prior ambient reexport should remain freely replayable");
        };

        assert_eq!(certificate.kernel_cost(), 0);
        let disposition = certificate
            .dispositions()
            .iter()
            .find(|entry| entry.clause() == &colliding_clause)
            .expect("prior reexport has a disposition");
        assert_eq!(
            disposition.disposition(),
            &ClauseCostDispositionV2::AmbientPriorPublicReexport {
                predecessor_class_digest: predecessor_class_digest.clone(),
            }
        );
        assert!(disposition.reconstructions().iter().any(|rule| {
            matches!(
                rule.proof(),
                VerifiedReconstructionProofV2::PriorPublicAmbientReexport {
                    predecessor_class_digest: actual,
                    ..
                } if actual == &predecessor_class_digest
            )
        }));
    }

    #[test]
    fn lambda_unit_cost_rejects_nested_descriptor_projection_before_inner_validation() {
        let AuditDecision::Proven(semantic_manifest) =
            verify_semantic_audit_lambda_unit_manifest_v1(
                &proposed_semantic_audit_lambda_unit_manifest_v1(),
            )
        else {
            panic!("lambda/unit semantic manifest");
        };
        let AuditDecision::Proven(cost_manifest) = verify_kernel_cost_lambda_unit_manifest_v2(
            &proposed_kernel_cost_lambda_unit_manifest_v2(),
        ) else {
            panic!("lambda/unit cost manifest");
        };
        let projected_unit_type = Term::First {
            pair: Box::new(Term::Pair {
                sigma_type: Box::new(Term::Sigma {
                    parameter: Box::new(Term::Sort { level: 0 }),
                    body: Box::new(Term::Sort { level: 0 }),
                }),
                first: Box::new(Term::UnitType),
                second: Box::new(Term::UnitType),
            }),
        };
        let declaration =
            ambient_declaration("ambient-cost/projection-inventory", projected_unit_type);
        let fixture = ambient_cost_fixture(
            &semantic_manifest,
            &cost_manifest,
            Vec::new(),
            vec![declaration],
            vec![clause_id("ambient-cost/projection-inventory")],
        );

        assert!(matches!(
            audit_kernel_cost_lambda_unit_v2(
                &cost_manifest,
                &semantic_manifest,
                &fixture.inventory,
                &[],
                &fixture.input,
            ),
            AuditDecision::OutsideFragment(OutsideFragmentReason::DescriptorProjection)
        ));

        let mut raw_projection = public_equation_fixture_for_manifest(
            PublicEquationFixtureMode::Paid,
            &semantic_manifest,
        );
        for evidence in &mut raw_projection.input.negative_evidence {
            evidence.checked_rules = cost_manifest.manifest().free_completion_rules.clone();
        }
        let RawPublicClauseKindV1::PublicEquation { pair, .. } = &mut raw_projection
            .input
            .clauses
            .iter_mut()
            .find(|clause| clause.id == raw_projection.equation_clause)
            .expect("equation clause")
            .clause
        else {
            unreachable!();
        };
        pair.normalized_right = Term::Apply {
            function: Box::new(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::First {
                    pair: Box::new(Term::Var { index: 0 }),
                }),
            }),
            argument: Box::new(Term::Unit),
        };
        assert!(matches!(
            audit_kernel_cost_lambda_unit_v2(
                &cost_manifest,
                &semantic_manifest,
                &raw_projection.inventory,
                &[],
                &raw_projection.input,
            ),
            AuditDecision::OutsideFragment(OutsideFragmentReason::DescriptorProjection)
        ));
    }

    #[test]
    fn lambda_unit_cost_rejects_plain_sigma_from_verified_inventory() {
        let AuditDecision::Proven(semantic_manifest) =
            verify_semantic_audit_lambda_unit_manifest_v1(
                &proposed_semantic_audit_lambda_unit_manifest_v1(),
            )
        else {
            panic!("lambda/unit semantic manifest");
        };
        let AuditDecision::Proven(cost_manifest) = verify_kernel_cost_lambda_unit_manifest_v2(
            &proposed_kernel_cost_lambda_unit_manifest_v2(),
        ) else {
            panic!("lambda/unit cost manifest");
        };
        let predecessor = Declaration {
            id: global("lambda-unit-outside/plain-sigma"),
            ty: Term::Sigma {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::UnitType),
            },
            body: None,
        };
        let successor = ambient_declaration("lambda-unit-outside/sigma-successor", Term::UnitType);
        let fixture = ambient_cost_fixture(
            &semantic_manifest,
            &cost_manifest,
            vec![predecessor],
            vec![successor],
            vec![clause_id("lambda-unit-outside/sigma-successor")],
        );

        assert!(matches!(
            audit_kernel_cost_lambda_unit_v2(
                &cost_manifest,
                &semantic_manifest,
                &fixture.inventory,
                &[],
                &fixture.input,
            ),
            AuditDecision::OutsideFragment(OutsideFragmentReason::UnsupportedTerm)
        ));
    }

    #[test]
    fn lambda_unit_cost_rejects_plain_pair_from_raw_clause() {
        let AuditDecision::Proven(semantic_manifest) =
            verify_semantic_audit_lambda_unit_manifest_v1(
                &proposed_semantic_audit_lambda_unit_manifest_v1(),
            )
        else {
            panic!("lambda/unit semantic manifest");
        };
        let AuditDecision::Proven(cost_manifest) = verify_kernel_cost_lambda_unit_manifest_v2(
            &proposed_kernel_cost_lambda_unit_manifest_v2(),
        ) else {
            panic!("lambda/unit cost manifest");
        };
        let mut fixture = public_equation_fixture_for_manifest(
            PublicEquationFixtureMode::Paid,
            &semantic_manifest,
        );
        let RawPublicClauseKindV1::PublicEquation { pair, .. } = &mut fixture
            .input
            .clauses
            .iter_mut()
            .find(|clause| clause.id == fixture.equation_clause)
            .expect("equation clause")
            .clause
        else {
            unreachable!();
        };
        pair.normalized_right = Term::Pair {
            sigma_type: Box::new(Term::UnitType),
            first: Box::new(Term::Unit),
            second: Box::new(Term::Unit),
        };

        assert!(matches!(
            audit_kernel_cost_lambda_unit_v2(
                &cost_manifest,
                &semantic_manifest,
                &fixture.inventory,
                &[],
                &fixture.input,
            ),
            AuditDecision::OutsideFragment(OutsideFragmentReason::UnsupportedTerm)
        ));
    }

    #[test]
    fn v2_public_clause_coverage_rejects_empty_omitted_extra_and_mismatch() {
        let fixture = public_equation_fixture(PublicEquationFixtureMode::Paid);
        assert!(matches!(
            audit_kernel_cost_v2(
                &verified_manifest_v2(),
                &fixture.inventory,
                &KernelCostAuditInputV2::default()
            ),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));

        let mut omitted = fixture.input.clone();
        omitted
            .clauses
            .retain(|clause| clause.id != fixture.equation_clause);
        omitted.public_dependency_dag.clear();
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &omitted),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));

        let mut extra = fixture.input.clone();
        let mut extra_clause = extra.clauses[0].clone();
        extra_clause.id = clause_id("public/extra");
        extra.clauses.push(extra_clause);
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &extra),
            AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision)
                | AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));

        let duplicate_inventory =
            public_equation_fixture(PublicEquationFixtureMode::Duplicate).inventory;
        assert!(matches!(
            audit_kernel_cost_v2(
                &verified_manifest_v2(),
                &duplicate_inventory,
                &fixture.input
            ),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));

        let mut missing_port = fixture.input.clone();
        missing_port.equation_demand_ports.clear();
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &missing_port),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));

        let mut duplicate_port = fixture.input.clone();
        duplicate_port
            .equation_demand_ports
            .push(duplicate_port.equation_demand_ports[0].clone());
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &duplicate_port),
            AuditDecision::Unknown(AuditUnknownReason::ProvenanceCollision)
        ));

        let mut extra_port = fixture.input.clone();
        extra_port
            .equation_demand_ports
            .push(EquationDemandPortBindingV2 {
                equation: equation_id("public/extra-port"),
                demand_port: None,
            });
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &extra_port),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));

        let mut forged_family = fixture.input.clone();
        forged_family.equation_demand_ports[0]
            .demand_port
            .as_mut()
            .expect("paid equation has an exact port")
            .family = DemandFamilyIdV1(digest("public/forged-demand-family"));
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &forged_family),
            AuditDecision::Unknown(AuditUnknownReason::NormalizationFailure)
        ));
    }

    #[test]
    fn v2_public_predecessor_equation_replay_is_inventory_derived_and_free() {
        let fixture = public_equation_fixture(PublicEquationFixtureMode::PredecessorReplay);
        let AuditDecision::Proven(certificate) =
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &fixture.input)
        else {
            panic!("exact predecessor equation replay should verify");
        };
        assert_eq!(certificate.kernel_cost(), 1);
        assert_eq!(
            disposition_v2(&certificate, &fixture.equation_clause),
            ClauseCostDispositionV1::TransparentAlias {
                target: global("public/f")
            }
        );
        let equation_disposition = certificate
            .dispositions()
            .iter()
            .find(|entry| entry.clause() == &fixture.equation_clause)
            .expect("equation disposition exists");
        assert!(equation_disposition.reconstructions().iter().any(|rule| {
            matches!(
                rule.proof(),
                VerifiedReconstructionProofV2::PriorPublicEquationReplay {
                    predecessor_equation,
                    ..
                } if predecessor_equation == &fixture.predecessor_equation
            )
        }));
    }

    #[test]
    fn v2_public_duplicate_binds_provenance_dependencies_and_demand_port() {
        let fixture = public_equation_fixture(PublicEquationFixtureMode::Duplicate);
        let duplicate = fixture
            .duplicate_clause
            .clone()
            .expect("duplicate fixture has second equation");
        let AuditDecision::Proven(certificate) =
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &fixture.input)
        else {
            panic!("exact inventory-bound duplicate should verify");
        };
        assert_eq!(certificate.kernel_cost(), 2);
        assert_eq!(
            disposition_v2(&certificate, &duplicate),
            ClauseCostDispositionV1::DuplicatePresentation {
                original: fixture.equation_clause.clone()
            }
        );

        let mut dependency_tamper = fixture.input.clone();
        dependency_tamper
            .clauses
            .iter_mut()
            .find(|clause| clause.id == duplicate)
            .expect("duplicate clause exists")
            .semantic_dependencies
            .clear();
        assert!(matches!(
            audit_kernel_cost_v2(
                &verified_manifest_v2(),
                &fixture.inventory,
                &dependency_tamper
            ),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteSupport)
        ));

        let mut demand_tamper = fixture.input.clone();
        let RawPublicClauseKindV1::PublicEquation { demand_output, .. } = &mut demand_tamper
            .clauses
            .iter_mut()
            .find(|clause| clause.id == duplicate)
            .expect("duplicate clause exists")
            .clause
        else {
            unreachable!();
        };
        *demand_output = None;
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &demand_tamper),
            AuditDecision::Unknown(AuditUnknownReason::NormalizationFailure)
        ));

        let mut omitted_equivalence = fixture.input.clone();
        omitted_equivalence.presentation_equivalences.clear();
        assert!(matches!(
            audit_kernel_cost_v2(
                &verified_manifest_v2(),
                &fixture.inventory,
                &omitted_equivalence
            ),
            AuditDecision::Unknown(AuditUnknownReason::MissingTupleDisposition)
        ));

        let mut duplicate_equivalence = fixture.input.clone();
        duplicate_equivalence
            .presentation_equivalences
            .push(duplicate_equivalence.presentation_equivalences[0].clone());
        assert!(matches!(
            audit_kernel_cost_v2(
                &verified_manifest_v2(),
                &fixture.inventory,
                &duplicate_equivalence
            ),
            AuditDecision::Unknown(AuditUnknownReason::MalformedInput)
        ));
    }

    #[test]
    fn v2_public_duplicate_equations_can_have_predecessor_public_owner() {
        let fixture = public_equation_fixture(PublicEquationFixtureMode::PredecessorOwnerDuplicate);
        let duplicate = fixture
            .duplicate_clause
            .clone()
            .expect("equation-only extension has a duplicate");
        let AuditDecision::Proven(certificate) =
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &fixture.input)
        else {
            panic!("inventory-owned equation-only duplicate should verify");
        };
        assert_eq!(certificate.kernel_cost(), 1);
        assert_eq!(
            disposition_v2(&certificate, &duplicate),
            ClauseCostDispositionV1::DuplicatePresentation {
                original: fixture.equation_clause.clone()
            }
        );
    }

    #[test]
    fn v2_public_forged_alias_descriptor_and_projection_payload_fail_closed() {
        let fixture = public_equation_fixture(PublicEquationFixtureMode::Paid);
        let mut alias = fixture.input.clone();
        let RawPublicClauseKindV1::PublicDeclaration {
            pair, presentation, ..
        } = &mut alias
            .clauses
            .iter_mut()
            .find(|clause| clause.id == fixture.head_clause)
            .expect("head exists")
            .clause
        else {
            unreachable!();
        };
        let target = global("public/a");
        pair.source_body = Some(Term::Global { id: target.clone() });
        pair.normalized_body = pair.source_body.clone();
        *presentation = HeadPresentationV1::TransparentAlias {
            target: target.clone(),
            availability: PublicAvailabilityV1::PredecessorPublicExport { target },
        };
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &alias),
            AuditDecision::Unknown(AuditUnknownReason::NormalizationFailure)
        ));

        let mut descriptor = fixture.input.clone();
        let head_clause = descriptor
            .clauses
            .iter_mut()
            .find(|clause| clause.id == fixture.head_clause)
            .expect("head exists");
        let RawPublicClauseKindV1::PublicDeclaration {
            head,
            equation_free_descriptors,
            ..
        } = &mut head_clause.clause
        else {
            unreachable!();
        };
        equation_free_descriptors.push(EquationFreeDescriptorV1 {
            owner: head.clone(),
            source_clause: fixture.head_clause.clone(),
            descriptor: EquationFreeDescriptorKindV1::OperationRole {
                role: LocalRoleV1::KernelHead,
            },
            public_support: BTreeSet::new(),
        });
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &descriptor),
            AuditDecision::Unknown(AuditUnknownReason::MissingDescriptorProjectionInventory)
        ));

        let mut projection = fixture.input.clone();
        let head = projection
            .clauses
            .iter_mut()
            .find(|clause| clause.id == fixture.head_clause)
            .expect("head exists");
        let projection_id = global("public/f");
        head.clause = RawPublicClauseKindV1::ForcedProjectionClause {
            projection: projection_id,
            record_owner: fixture.head_clause.clone(),
            field_ordinal: 0,
            pair: declaration_pair("forged-projection", Some(Term::Unit)),
        };
        assert!(matches!(
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &projection),
            AuditDecision::Unknown(AuditUnknownReason::MissingDescriptorProjectionInventory)
        ));
    }

    #[test]
    fn v2_public_inventory_and_input_order_are_digest_stable() {
        let fixture = public_equation_fixture(PublicEquationFixtureMode::Paid);
        let AuditDecision::Proven(forward) =
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &fixture.input)
        else {
            panic!("forward vector verifies");
        };
        let mut reordered = fixture.input.clone();
        reordered.clauses.reverse();
        reordered.negative_evidence.reverse();
        reordered.public_dependency_dag.reverse();
        reordered.equation_demand_ports.reverse();
        let AuditDecision::Proven(reverse) =
            audit_kernel_cost_v2(&verified_manifest_v2(), &fixture.inventory, &reordered)
        else {
            panic!("reordered vector verifies");
        };
        assert_eq!(forward, reverse);
        assert_eq!(forward.certificate_digest(), reverse.certificate_digest());
        assert_eq!(forward.inventory_digest(), fixture.inventory.digest());
    }

    #[test]
    fn v2_bodyless_fresh_head_and_sealed_equation_are_separately_paid() {
        let (mut v1, head, equation) = fresh_vector(true);
        v1.reconstructions.clear();
        let input = KernelCostAuditInputV2 {
            clauses: v1.clauses,
            negative_evidence: vec![
                negative_v2(
                    head.clone(),
                    singleton(equation.clone()),
                    singleton(equation.clone()),
                ),
                negative_v2(
                    equation.clone(),
                    singleton(head.clone()),
                    singleton(head.clone()),
                ),
            ],
            ..KernelCostAuditInputV2::default()
        };

        let certificate = proven_v2(&input);
        assert_eq!(certificate.kernel_cost(), 2);
        assert_eq!(
            certificate.manifest_digest(),
            verified_manifest_v2().candidate_digest()
        );
        assert_eq!(
            disposition_v2(&certificate, &head),
            ClauseCostDispositionV1::FirstIrreducible
        );
        assert_eq!(
            disposition_v2(&certificate, &equation),
            ClauseCostDispositionV1::FirstIrreducible
        );
    }

    #[test]
    fn v2_fresh_completion_claim_is_not_cost_evidence() {
        let (v1, _, _) = fresh_vector(true);
        let forged = serde_json::to_value(&v1.reconstructions[0])
            .expect("V1 reconstruction claim serializes");
        assert!(serde_json::from_value::<ReconstructionDerivationV2>(forged).is_err());
    }

    #[test]
    fn v2_duplicate_normalized_equation_is_free() {
        let (mut v1, head, equation) = fresh_vector(true);
        v1.reconstructions.clear();
        let mut duplicate = v1
            .clauses
            .iter()
            .find(|clause| clause.id == equation)
            .cloned()
            .expect("fresh vector has its equation");
        let duplicate_id = clause_id("fresh/equation/duplicate");
        duplicate.id = duplicate_id.clone();
        let RawPublicClauseKindV1::PublicEquation {
            equation: duplicate_equation,
            ..
        } = &mut duplicate.clause
        else {
            panic!("selected clause is an equation");
        };
        *duplicate_equation = equation_id("fresh/equation/duplicate");
        v1.clauses.push(duplicate);

        let input = KernelCostAuditInputV2 {
            clauses: v1.clauses,
            reconstructions: vec![duplicate_rule_v2(duplicate_id.clone(), equation.clone())],
            negative_evidence: vec![
                negative_v2(
                    head.clone(),
                    singleton(equation.clone()),
                    [equation.clone(), duplicate_id.clone()]
                        .into_iter()
                        .collect(),
                ),
                negative_v2(
                    equation.clone(),
                    singleton(head.clone()),
                    singleton(head.clone()),
                ),
            ],
            presentation_equivalences: vec![PresentationEquivalenceV2 {
                representative: equation.clone(),
                equivalent: duplicate_id.clone(),
                proof: PresentationEquivalenceProofV2::DuplicateNormalizedEquation,
            }],
            ..KernelCostAuditInputV2::default()
        };

        let certificate = proven_v2(&input);
        assert_eq!(certificate.kernel_cost(), 2);
        assert_eq!(
            disposition_v2(&certificate, &duplicate_id),
            ClauseCostDispositionV1::DuplicatePresentation { original: equation }
        );
    }

    #[test]
    fn v2_missing_or_tampered_negative_inventory_is_unknown() {
        let (mut v1, head, equation) = fresh_vector(true);
        v1.reconstructions.clear();
        let mut input = KernelCostAuditInputV2 {
            clauses: v1.clauses,
            negative_evidence: vec![
                negative_v2(
                    head.clone(),
                    singleton(equation.clone()),
                    singleton(equation.clone()),
                ),
                negative_v2(
                    equation.clone(),
                    singleton(head.clone()),
                    singleton(head.clone()),
                ),
            ],
            ..KernelCostAuditInputV2::default()
        };
        input.negative_evidence.pop();
        assert!(matches!(
            audit_v2(&input),
            AuditDecision::Unknown(AuditUnknownReason::MissingNegativeEvidence)
        ));

        input.negative_evidence.push(negative_v2(
            equation,
            singleton(head.clone()),
            singleton(head),
        ));
        input.negative_evidence[0].checked_rules.pop();
        assert!(matches!(
            audit_v2(&input),
            AuditDecision::Unknown(AuditUnknownReason::MalformedInput)
        ));

        let (mut v1, head, equation) = fresh_vector(true);
        v1.reconstructions.clear();
        let spurious = clause_id("v2/spurious-realized");
        v1.clauses
            .push(transparent_definition("v2/spurious-realized"));
        let mut forged = negative_v2(head, singleton(equation.clone()), singleton(equation));
        forged.realized.insert(spurious);
        let forged_input = KernelCostAuditInputV2 {
            clauses: v1.clauses,
            negative_evidence: vec![forged],
            ..KernelCostAuditInputV2::default()
        };
        assert!(matches!(
            audit_v2(&forged_input),
            AuditDecision::Unknown(AuditUnknownReason::MalformedInput)
        ));
    }

    fn disposition_v2(
        certificate: &CostAuditCertificateV2,
        clause: &ClauseIdV1,
    ) -> ClauseCostDispositionV1 {
        certificate
            .dispositions()
            .iter()
            .find(|entry| entry.clause() == clause)
            .expect("every raw clause must have one V2 disposition")
            .disposition()
            .legacy_v1()
            .expect("test helper requires a disposition shared with V1")
    }
}
