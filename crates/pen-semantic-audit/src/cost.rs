//! Finite, certificate-carrying kernel-cost audit.
//!
//! This module deliberately contains no live Profile-A adapter.  It checks a
//! small generic grammar whose only free reconstruction rules are those named
//! by [`KernelCostManifestV1`].  The checker is exhaustive under the manifest
//! bounds: it never turns failed proof search into irreducibility.

use crate::manifest::{
    AuditDecision, AuditUnknownReason, FreeCompletionRuleV1, OutsideFragmentReason,
    VerifiedCostManifestV1,
};
use crate::model::{
    ClauseCostDispositionV1, ClauseIdV1, DemandOutputIdV1, HeadPresentationV1, LocalRoleV1,
    PublicAvailabilityV1,
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
}

impl CanonicalEncode for SourceToNormalDerivationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Reflexivity => encoder.tag(0),
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ClauseDispositionCertificateV1 {
    pub clause: ClauseIdV1,
    pub disposition: ClauseCostDispositionV1,
    pub closure_round: u32,
    pub reconstructions: Vec<ReconstructionDerivationV1>,
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

#[derive(Clone)]
struct CertifiedBasis {
    seeds: BTreeSet<ClauseIdV1>,
    signature: BTreeSet<ClauseIdV1>,
    independence: Vec<BasisIndependenceCertificateV1>,
}

struct PresentationClasses {
    roots: BTreeMap<ClauseIdV1, ClauseIdV1>,
    members: BTreeMap<ClauseIdV1, BTreeSet<ClauseIdV1>>,
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

fn saturate(
    seeds: &BTreeSet<ClauseIdV1>,
    rules: &[ReconstructionDerivationV1],
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
            .filter(|rule| !members.contains(&rule.output) && rule.premises.is_subset(&members))
            .map(|rule| rule.output.clone())
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
    let mut dispositions = Vec::with_capacity(prepared.clause_ids.len());
    for clause in &prepared.clause_ids {
        let root = prepared
            .class_root
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
            let reconstructions = grounded_reconstructions(clause, closure, &prepared.rules);
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

        let reconstructions = grounded_reconstructions(clause, closure, &prepared.rules);
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

fn dependency_components(
    clauses: &BTreeMap<ClauseIdV1, RawPublicClauseV1>,
    rules: &[ReconstructionDerivationV1],
) -> Vec<DependencyComponentV1> {
    let mut adjacency = clauses
        .iter()
        .map(|(id, clause)| (id.clone(), clause.semantic_dependencies.clone()))
        .collect::<BTreeMap<_, _>>();
    for rule in rules {
        adjacency
            .entry(rule.output.clone())
            .or_default()
            .extend(rule.premises.iter().cloned());
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
    use crate::manifest::{proposed_kernel_cost_manifest_v1, verify_kernel_cost_manifest_v1};
    use crate::model::{AmbientPrimitiveV1, EquationIdV1};

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

    fn verified_manifest() -> VerifiedCostManifestV1 {
        let AuditDecision::Proven(manifest) =
            verify_kernel_cost_manifest_v1(&proposed_kernel_cost_manifest_v1())
        else {
            panic!("proposed generic manifest must verify");
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

    fn audit(input: &KernelCostAuditInputV1) -> AuditDecision<CostAuditCertificateV1> {
        audit_kernel_cost_v1(&verified_manifest(), input)
    }

    fn proven(input: &KernelCostAuditInputV1) -> CostAuditCertificateV1 {
        let AuditDecision::Proven(certificate) = audit(input) else {
            panic!("vector should be proven");
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
}
