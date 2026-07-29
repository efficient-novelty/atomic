//! Finite rewrite admissibility for the projection-free lambda/unit profile.
//!
//! The typed rewrite inventory deliberately stops before admissibility.  This
//! module closes the finite set of whole typed terms named by the public
//! inventory and the pre-Q0 carrier under the only reductions authorized by
//! the lambda/unit profile:
//!
//! * ordinary one-step beta;
//! * delta for bodyful public declarations; and
//! * the exactly covered typed fresh equations.
//!
//! The current graph routines are non-authorizing diagnostics. The public
//! verifier validates bindings, seed subjects, and the fragment boundary,
//! then stops before capability minting because substitution closure is not
//! finite as currently defined and three completeness obligations remain
//! open. No diagnostic graph can authorize Q0.

use crate::carrier::{
    ContextAmalgamationWitnessV1, PreQ0RawCarrierCertificateV1, rename_judgment_into,
};
use crate::fragment::lambda_unit_term_syntax_violation;
use crate::inventory::VerifiedPublicAuditInventoryV1;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, Q0RuleV1,
    SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1, SubstitutionCarrierRuleV1,
    VerifiedSemanticAuditManifestV1,
};
use crate::model::{EquationIdV1, GenericJudgmentV1, SemanticSchemaSeedV1};
use crate::rewrite_inventory::{VerifiedTypedRewriteEntryV1, VerifiedTypedRewriteInventoryV1};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Kernel, KernelError,
    OpenJudgment, Term, VerifiedSignature,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct RewriteNodeIdV1(Digest);

impl RewriteNodeIdV1 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for RewriteNodeIdV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct RewriteEdgeIdV1(Digest);

impl RewriteEdgeIdV1 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for RewriteEdgeIdV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct RewriteSubstitutionIdV1(Digest);

impl RewriteSubstitutionIdV1 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for RewriteSubstitutionIdV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(transparent)]
pub struct TermPositionV1(Vec<u16>);

impl TermPositionV1 {
    pub fn root() -> Self {
        Self(Vec::new())
    }

    pub fn child_ordinals(&self) -> &[u16] {
        &self.0
    }
}

impl CanonicalEncode for TermPositionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u64(self.0.len() as u64);
        for ordinal in &self.0 {
            encoder.u16(*ordinal);
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FiniteSubstitutionKindV1 {
    Identity,
    ContextEmbeddingLift,
    ForcedNewestArgument,
    Composite,
}

impl CanonicalEncode for FiniteSubstitutionKindV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::Identity => 0,
            Self::ContextEmbeddingLift => 1,
            Self::ForcedNewestArgument => 2,
            Self::Composite => 3,
        });
    }
}

/// A structurally checked renaming from one finite context to another.
///
/// The embedding maps source declaration ordinals (oldest first) to target
/// declaration ordinals.  Fields are private and the type is not
/// deserializable.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedFiniteSubstitutionV1 {
    id: RewriteSubstitutionIdV1,
    kind: FiniteSubstitutionKindV1,
    source_context: DependentContext,
    target_context: DependentContext,
    embedding: Option<Vec<u32>>,
    images: Vec<Term>,
}

impl VerifiedFiniteSubstitutionV1 {
    pub fn id(&self) -> &RewriteSubstitutionIdV1 {
        &self.id
    }

    pub fn kind(&self) -> FiniteSubstitutionKindV1 {
        self.kind
    }

    pub fn source_context(&self) -> &DependentContext {
        &self.source_context
    }

    pub fn target_context(&self) -> &DependentContext {
        &self.target_context
    }

    pub fn embedding(&self) -> Option<&[u32]> {
        self.embedding.as_deref()
    }

    /// Complete source-variable images in oldest-first declaration order.
    pub fn images(&self) -> &[Term] {
        &self.images
    }
}

impl CanonicalEncode for VerifiedFiniteSubstitutionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.kind.encode_canonical(encoder);
        self.source_context.encode_canonical(encoder);
        self.target_context.encode_canonical(encoder);
        match &self.embedding {
            Some(embedding) => {
                encoder.tag(1);
                encode_u32_slice(encoder, embedding);
            }
            None => encoder.tag(0),
        }
        encoder.sequence(&self.images);
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "rule", rename_all = "snake_case")]
pub enum FiniteRewriteRuleV1 {
    CanonicalDeBruijn,
    SequentialSubstitution,
    OrdinaryBeta,
    PublicDelta {
        declaration: GlobalId,
    },
    NoPublicDeltaDeclarations,
    CanonicalUnit,
    FlattenedTelescope,
    TypedFreshEquation {
        equation: EquationIdV1,
        substitution: RewriteSubstitutionIdV1,
    },
}

impl CanonicalEncode for FiniteRewriteRuleV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::CanonicalDeBruijn => encoder.tag(0),
            Self::SequentialSubstitution => encoder.tag(1),
            Self::OrdinaryBeta => encoder.tag(2),
            Self::PublicDelta { declaration } => {
                encoder.tag(3);
                declaration.encode_canonical(encoder);
            }
            Self::NoPublicDeltaDeclarations => encoder.tag(4),
            Self::CanonicalUnit => encoder.tag(5),
            Self::FlattenedTelescope => encoder.tag(6),
            Self::TypedFreshEquation {
                equation,
                substitution,
            } => {
                encoder.tag(7);
                equation.encode_canonical(encoder);
                substitution.encode_canonical(encoder);
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RewriteInapplicableReasonV1 {
    DeBruijnAlreadyCanonical,
    NoExplicitSubstitutionSyntax,
    FocusShapeMismatch,
    GlobalHeadMismatch,
    NoPublicDeltaDeclaration,
    UnitAlreadyCanonical,
    TelescopeAlreadyFlattened,
    ContextMismatch,
    FreshPatternMismatch,
}

impl CanonicalEncode for RewriteInapplicableReasonV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::DeBruijnAlreadyCanonical => 0,
            Self::NoExplicitSubstitutionSyntax => 1,
            Self::FocusShapeMismatch => 2,
            Self::GlobalHeadMismatch => 3,
            Self::NoPublicDeltaDeclaration => 4,
            Self::UnitAlreadyCanonical => 5,
            Self::TelescopeAlreadyFlattened => 6,
            Self::ContextMismatch => 7,
            Self::FreshPatternMismatch => 8,
        });
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum RewriteDispositionOutcomeV1 {
    Applicable { edge: RewriteEdgeIdV1 },
    CertifiedInapplicable { reason: RewriteInapplicableReasonV1 },
}

impl CanonicalEncode for RewriteDispositionOutcomeV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::Applicable { edge } => {
                encoder.tag(0);
                edge.encode_canonical(encoder);
            }
            Self::CertifiedInapplicable { reason } => {
                encoder.tag(1);
                reason.encode_canonical(encoder);
            }
        }
    }
}

/// Exhaustive disposition for one node/rule/syntactic-position triple.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RewriteDispositionV1 {
    node: RewriteNodeIdV1,
    position: TermPositionV1,
    rule: FiniteRewriteRuleV1,
    outcome: RewriteDispositionOutcomeV1,
}

impl RewriteDispositionV1 {
    pub fn node(&self) -> &RewriteNodeIdV1 {
        &self.node
    }

    pub fn position(&self) -> &TermPositionV1 {
        &self.position
    }

    pub fn rule(&self) -> &FiniteRewriteRuleV1 {
        &self.rule
    }

    pub fn outcome(&self) -> &RewriteDispositionOutcomeV1 {
        &self.outcome
    }
}

impl CanonicalEncode for RewriteDispositionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.node.encode_canonical(encoder);
        self.position.encode_canonical(encoder);
        self.rule.encode_canonical(encoder);
        self.outcome.encode_canonical(encoder);
    }
}

/// One kernel-typed one-step edge in the finite whole-term graph.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedRewriteEdgeV1 {
    id: RewriteEdgeIdV1,
    source: RewriteNodeIdV1,
    target: RewriteNodeIdV1,
    position: TermPositionV1,
    rule: FiniteRewriteRuleV1,
}

impl VerifiedRewriteEdgeV1 {
    pub fn id(&self) -> &RewriteEdgeIdV1 {
        &self.id
    }

    pub fn source(&self) -> &RewriteNodeIdV1 {
        &self.source
    }

    pub fn target(&self) -> &RewriteNodeIdV1 {
        &self.target
    }

    pub fn position(&self) -> &TermPositionV1 {
        &self.position
    }

    pub fn rule(&self) -> &FiniteRewriteRuleV1 {
        &self.rule
    }
}

impl CanonicalEncode for VerifiedRewriteEdgeV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.source.encode_canonical(encoder);
        self.target.encode_canonical(encoder);
        self.position.encode_canonical(encoder);
        self.rule.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "judgment", rename_all = "snake_case")]
pub enum RewriteNodeJudgmentV1 {
    HasType { ty: Term },
    TypeFormation,
}

impl CanonicalEncode for RewriteNodeJudgmentV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::HasType { ty } => {
                encoder.tag(0);
                ty.encode_canonical(encoder);
            }
            Self::TypeFormation => encoder.tag(1),
        }
    }
}

/// One member of the finite typed whole-term universe.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedRewriteNodeV1 {
    id: RewriteNodeIdV1,
    context: DependentContext,
    term: Term,
    judgment: RewriteNodeJudgmentV1,
    predecessor_fragment: bool,
    termination_rank: u32,
    normal_form: RewriteNodeIdV1,
}

impl VerifiedRewriteNodeV1 {
    pub fn id(&self) -> &RewriteNodeIdV1 {
        &self.id
    }

    pub fn context(&self) -> &DependentContext {
        &self.context
    }

    pub fn term(&self) -> &Term {
        &self.term
    }

    pub fn judgment(&self) -> &RewriteNodeJudgmentV1 {
        &self.judgment
    }

    pub fn ty(&self) -> Option<&Term> {
        match &self.judgment {
            RewriteNodeJudgmentV1::HasType { ty } => Some(ty),
            RewriteNodeJudgmentV1::TypeFormation => None,
        }
    }

    pub fn is_type_formation(&self) -> bool {
        matches!(self.judgment, RewriteNodeJudgmentV1::TypeFormation)
    }

    pub fn is_predecessor_fragment(&self) -> bool {
        self.predecessor_fragment
    }

    pub fn termination_rank(&self) -> u32 {
        self.termination_rank
    }

    pub fn normal_form(&self) -> &RewriteNodeIdV1 {
        &self.normal_form
    }
}

impl CanonicalEncode for VerifiedRewriteNodeV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.context.encode_canonical(encoder);
        self.term.encode_canonical(encoder);
        self.judgment.encode_canonical(encoder);
        encoder.tag(u8::from(self.predecessor_fragment));
        encoder.u32(self.termination_rank);
        self.normal_form.encode_canonical(encoder);
    }
}

/// Diagnostic for two or more rules applicable at the same node and position.
///
/// A verified system contains only joinable overlaps; the common normal form
/// records the finite join witness.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedOverlapDiagnosticV1 {
    node: RewriteNodeIdV1,
    position: TermPositionV1,
    rules: Vec<FiniteRewriteRuleV1>,
    targets: Vec<RewriteNodeIdV1>,
    common_normal_form: RewriteNodeIdV1,
}

impl VerifiedOverlapDiagnosticV1 {
    pub fn node(&self) -> &RewriteNodeIdV1 {
        &self.node
    }

    pub fn position(&self) -> &TermPositionV1 {
        &self.position
    }

    pub fn rules(&self) -> &[FiniteRewriteRuleV1] {
        &self.rules
    }

    pub fn targets(&self) -> &[RewriteNodeIdV1] {
        &self.targets
    }

    pub fn common_normal_form(&self) -> &RewriteNodeIdV1 {
        &self.common_normal_form
    }
}

impl CanonicalEncode for VerifiedOverlapDiagnosticV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.node.encode_canonical(encoder);
        self.position.encode_canonical(encoder);
        encoder.sequence(&self.rules);
        encoder.sequence(&self.targets);
        self.common_normal_form.encode_canonical(encoder);
    }
}

/// A concrete graph path witnessing stability of one rewrite edge under one
/// verified finite substitution.
///
/// The path includes both endpoints.  Fields are private and the type is not
/// deserializable, so callers cannot manufacture reachability evidence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedSubstitutionReachabilityV1 {
    edge: RewriteEdgeIdV1,
    substitution: RewriteSubstitutionIdV1,
    source: RewriteNodeIdV1,
    target: RewriteNodeIdV1,
    path: Vec<RewriteNodeIdV1>,
}

impl VerifiedSubstitutionReachabilityV1 {
    pub fn edge(&self) -> &RewriteEdgeIdV1 {
        &self.edge
    }

    pub fn substitution(&self) -> &RewriteSubstitutionIdV1 {
        &self.substitution
    }

    pub fn source(&self) -> &RewriteNodeIdV1 {
        &self.source
    }

    pub fn target(&self) -> &RewriteNodeIdV1 {
        &self.target
    }

    /// Canonical shortest path, including the substituted source and target.
    pub fn path(&self) -> &[RewriteNodeIdV1] {
        &self.path
    }
}

impl CanonicalEncode for VerifiedSubstitutionReachabilityV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.edge.encode_canonical(encoder);
        self.substitution.encode_canonical(encoder);
        self.source.encode_canonical(encoder);
        self.target.encode_canonical(encoder);
        encoder.sequence(&self.path);
    }
}

/// Opaque finite rewrite-admissibility capability.
///
/// All fields are private and there is deliberately no `Deserialize`
/// implementation.  The capability is bound to the exact manifest, carrier,
/// public inventory, typed inventory, kernel protocol, and normalizer
/// protocol used by the verifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedRewriteSystemV1 {
    manifest_digest: Digest,
    carrier_digest: Digest,
    inventory_digest: Digest,
    typed_inventory_digest: Digest,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    normalizer_protocol_digest: Digest,
    seed_subject_digest: Digest,
    substitutions: Vec<VerifiedFiniteSubstitutionV1>,
    nodes: Vec<VerifiedRewriteNodeV1>,
    edges: Vec<VerifiedRewriteEdgeV1>,
    dispositions: Vec<RewriteDispositionV1>,
    overlaps: Vec<VerifiedOverlapDiagnosticV1>,
    substitution_reachability: Vec<VerifiedSubstitutionReachabilityV1>,
    substitution_reachability_digest: Digest,
    predecessor_graph_digest: Digest,
    digest: Digest,
}

impl VerifiedRewriteSystemV1 {
    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    pub fn carrier_digest(&self) -> &Digest {
        &self.carrier_digest
    }

    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn typed_inventory_digest(&self) -> &Digest {
        &self.typed_inventory_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.normalizer_protocol_digest
    }

    /// Digest of the exact declaration/equation term-subject census used by
    /// this rewrite theorem.  It is not a full semantic support/anchor census.
    pub fn seed_subject_digest(&self) -> &Digest {
        &self.seed_subject_digest
    }

    pub fn substitutions(&self) -> &[VerifiedFiniteSubstitutionV1] {
        &self.substitutions
    }

    pub fn nodes(&self) -> &[VerifiedRewriteNodeV1] {
        &self.nodes
    }

    pub fn edges(&self) -> &[VerifiedRewriteEdgeV1] {
        &self.edges
    }

    pub fn dispositions(&self) -> &[RewriteDispositionV1] {
        &self.dispositions
    }

    pub fn overlaps(&self) -> &[VerifiedOverlapDiagnosticV1] {
        &self.overlaps
    }

    pub fn substitution_reachability(&self) -> &[VerifiedSubstitutionReachabilityV1] {
        &self.substitution_reachability
    }

    pub fn substitution_reachability_digest(&self) -> &Digest {
        &self.substitution_reachability_digest
    }

    pub fn predecessor_graph_digest(&self) -> &Digest {
        &self.predecessor_graph_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedRewriteSystemV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        self.carrier_digest.encode_canonical(encoder);
        self.inventory_digest.encode_canonical(encoder);
        self.typed_inventory_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.seed_subject_digest.encode_canonical(encoder);
        encoder.sequence(&self.substitutions);
        encoder.sequence(&self.nodes);
        encoder.sequence(&self.edges);
        encoder.sequence(&self.dispositions);
        encoder.sequence(&self.overlaps);
        encoder.sequence(&self.substitution_reachability);
        self.substitution_reachability_digest
            .encode_canonical(encoder);
        self.predecessor_graph_digest.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NodeCandidate {
    id: RewriteNodeIdV1,
    context: DependentContext,
    term: Term,
    judgment: RewriteNodeJudgmentV1,
}

#[derive(Clone, Debug)]
struct RuleInstance {
    rule: FiniteRewriteRuleV1,
    target_context: Option<DependentContext>,
    left: Option<Term>,
    right: Term,
}

#[derive(Clone, Debug)]
struct PositionedTerm {
    position: TermPositionV1,
    binder_depth: u32,
    focus: Term,
}

#[derive(Clone, Debug)]
struct GraphAnalysis {
    ranks: BTreeMap<RewriteNodeIdV1, u32>,
    normal_forms: BTreeMap<RewriteNodeIdV1, RewriteNodeIdV1>,
    adjacency: BTreeMap<RewriteNodeIdV1, BTreeSet<RewriteNodeIdV1>>,
}

#[derive(Clone, Debug)]
enum VerifyFailure {
    Outside(OutsideFragmentReason),
    Unknown(AuditUnknownReason),
}

type VerifyResult<T> = Result<T, VerifyFailure>;

impl From<AuditUnknownReason> for VerifyFailure {
    fn from(reason: AuditUnknownReason) -> Self {
        Self::Unknown(reason)
    }
}

/// Verify the complete finite rewrite graph for the lambda/unit profile.
pub fn verify_finite_rewrite_system_lambda_unit_v1(
    kernel: &Kernel,
    manifest: &VerifiedSemanticAuditManifestV1,
    carrier: &PreQ0RawCarrierCertificateV1,
    inventory: &VerifiedPublicAuditInventoryV1,
    typed_inventory: &VerifiedTypedRewriteInventoryV1,
) -> AuditDecision<VerifiedRewriteSystemV1> {
    match verify_finite_rewrite_system_inner(kernel, manifest, carrier, inventory, typed_inventory)
    {
        Ok(system) => AuditDecision::Proven(system),
        Err(VerifyFailure::Outside(reason)) => AuditDecision::OutsideFragment(reason),
        Err(VerifyFailure::Unknown(reason)) => AuditDecision::Unknown(reason),
    }
}

/// Alias retained for callers whose profile is already fixed by the verified
/// manifest.  This entry point still enforces the lambda/unit profile exactly.
pub fn verify_finite_rewrite_system_v1(
    kernel: &Kernel,
    manifest: &VerifiedSemanticAuditManifestV1,
    carrier: &PreQ0RawCarrierCertificateV1,
    inventory: &VerifiedPublicAuditInventoryV1,
    typed_inventory: &VerifiedTypedRewriteInventoryV1,
) -> AuditDecision<VerifiedRewriteSystemV1> {
    verify_finite_rewrite_system_lambda_unit_v1(
        kernel,
        manifest,
        carrier,
        inventory,
        typed_inventory,
    )
}

fn verify_finite_rewrite_system_inner(
    kernel: &Kernel,
    manifest: &VerifiedSemanticAuditManifestV1,
    carrier: &PreQ0RawCarrierCertificateV1,
    inventory: &VerifiedPublicAuditInventoryV1,
    typed_inventory: &VerifiedTypedRewriteInventoryV1,
) -> VerifyResult<VerifiedRewriteSystemV1> {
    verify_bindings(kernel, manifest, carrier, inventory, typed_inventory)?;
    let seed_subject_digest = verify_carrier_seed_subject_coverage(carrier, inventory)?;
    verify_projection_free(inventory)?;
    validate_input_fragment(manifest, carrier, inventory, typed_inventory)?;
    verify_complete_rewrite_certificate_boundary_v1()?;

    let mut candidates = collect_initial_nodes(carrier, inventory, typed_inventory)?;
    let contexts = collect_contexts(&candidates, carrier.context_witnesses());
    let mut substitutions = derive_substitutions(
        kernel,
        inventory,
        carrier.context_witnesses(),
        &contexts,
        &candidates,
        manifest.manifest().maximum_tuple_dispositions as usize,
    )?;

    let node_limit = manifest.manifest().maximum_raw_derivations as usize;
    let disposition_limit = manifest.manifest().maximum_tuple_dispositions as usize;
    if candidates.len() > node_limit {
        return Err(AuditUnknownReason::ResourceExhausted.into());
    }

    let mut candidate_indices = BTreeMap::new();
    for (index, candidate) in candidates.iter().enumerate() {
        if let Some(previous) = candidate_indices.insert(candidate.id.clone(), index) {
            if candidates[previous] != *candidate {
                return Err(AuditUnknownReason::ProvenanceCollision.into());
            }
        }
    }

    let mut saturation_rounds = 0_usize;
    let (rules, mut edges, mut dispositions) = loop {
        saturation_rounds = saturation_rounds
            .checked_add(1)
            .ok_or(AuditUnknownReason::ResourceExhausted)?;
        if saturation_rounds > disposition_limit {
            return Err(AuditUnknownReason::ResourceExhausted.into());
        }
        let rules =
            build_rule_instances(kernel, manifest, inventory, typed_inventory, &substitutions)?;
        let mut edges = Vec::new();
        let mut edge_indices = BTreeMap::<RewriteEdgeIdV1, usize>::new();
        let mut dispositions = Vec::new();
        let mut cursor = 0_usize;
        while cursor < candidates.len() {
            let node = candidates[cursor].clone();
            kernel_check_node(kernel, inventory, &node)?;

            for substitution in substitutions
                .iter()
                .filter(|substitution| substitution.source_context == node.context)
            {
                let embedded = rename_candidate(&node, substitution)?;
                kernel_check_node(kernel, inventory, &embedded)?;
                insert_candidate(
                    embedded,
                    &mut candidates,
                    &mut candidate_indices,
                    node_limit,
                )?;
            }

            let positions = enumerate_positions(&node.term, kernel.limits().max_depth)?;
            for positioned in positions {
                for rule in &rules {
                    if dispositions.len() >= disposition_limit {
                        return Err(AuditUnknownReason::ResourceExhausted.into());
                    }
                    let reduction = apply_rule_at_focus(kernel, rule, &node.context, &positioned)?;
                    let outcome = match reduction {
                        RuleApplication::Inapplicable(reason) => {
                            RewriteDispositionOutcomeV1::CertifiedInapplicable { reason }
                        }
                        RuleApplication::Applicable(replacement) => {
                            let target_term = replace_at_position(
                                &node.term,
                                positioned.position.child_ordinals(),
                                &replacement,
                            )
                            .ok_or(AuditUnknownReason::MalformedInput)?;
                            let target = make_node_candidate_with_judgment(
                                node.context.clone(),
                                target_term,
                                node.judgment.clone(),
                            );
                            kernel_check_edge(kernel, inventory, &node, &target)?;
                            insert_candidate(
                                target.clone(),
                                &mut candidates,
                                &mut candidate_indices,
                                node_limit,
                            )?;
                            let edge = make_edge(
                                node.id.clone(),
                                target.id.clone(),
                                positioned.position.clone(),
                                rule.rule.clone(),
                            );
                            if let Some(previous) = edge_indices.get(&edge.id).copied() {
                                if edges[previous] != edge {
                                    return Err(AuditUnknownReason::ProvenanceCollision.into());
                                }
                            } else {
                                edge_indices.insert(edge.id.clone(), edges.len());
                                edges.push(edge.clone());
                            }
                            RewriteDispositionOutcomeV1::Applicable {
                                edge: edge.id.clone(),
                            }
                        }
                    };
                    dispositions.push(RewriteDispositionV1 {
                        node: node.id.clone(),
                        position: positioned.position.clone(),
                        rule: rule.rule.clone(),
                        outcome,
                    });
                }
            }
            cursor += 1;
        }
        let next_substitutions = derive_substitutions(
            kernel,
            inventory,
            carrier.context_witnesses(),
            &contexts,
            &candidates,
            disposition_limit,
        )?;
        if next_substitutions == substitutions {
            break (rules, edges, dispositions);
        }
        substitutions = next_substitutions;
    };

    verify_disposition_exhaustiveness(&candidates, &rules, &edges, &dispositions, kernel)?;
    let node_ids = candidates
        .iter()
        .map(|candidate| candidate.id.clone())
        .collect::<BTreeSet<_>>();
    let analysis = analyze_graph(&node_ids, &edges)?;
    let overlaps = verify_overlaps(&edges, &analysis.normal_forms)?;
    let (substitution_reachability, substitution_reachability_digest) =
        verify_substitution_reachability(
            &candidates,
            &candidate_indices,
            &edges,
            &analysis.adjacency,
            &substitutions,
            disposition_limit,
        )?;

    let predecessor_globals = inventory
        .predecessor_boundary()
        .declarations()
        .iter()
        .map(|declaration| declaration.id.clone())
        .collect::<BTreeSet<_>>();
    let predecessor_graph_digest = verify_predecessor_conservativity(
        kernel,
        inventory,
        &candidates,
        &edges,
        &dispositions,
        &analysis.normal_forms,
        &predecessor_globals,
    )?;

    edges.sort_by(|left, right| left.id.cmp(&right.id));
    dispositions.sort_by(|left, right| {
        (
            &left.node,
            &left.position,
            Digest::of_canonical("pen-semantic-audit/finite-rewrite-rule/v1", &left.rule),
        )
            .cmp(&(
                &right.node,
                &right.position,
                Digest::of_canonical("pen-semantic-audit/finite-rewrite-rule/v1", &right.rule),
            ))
    });
    let mut nodes = candidates
        .into_iter()
        .map(|candidate| {
            let termination_rank = analysis
                .ranks
                .get(&candidate.id)
                .copied()
                .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
            let normal_form = analysis
                .normal_forms
                .get(&candidate.id)
                .cloned()
                .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
            Ok(VerifiedRewriteNodeV1 {
                predecessor_fragment: node_is_predecessor(&candidate, &predecessor_globals),
                id: candidate.id,
                context: candidate.context,
                term: candidate.term,
                judgment: candidate.judgment,
                termination_rank,
                normal_form,
            })
        })
        .collect::<Result<Vec<_>, AuditUnknownReason>>()?;
    nodes.sort_by(|left, right| left.id.cmp(&right.id));

    let mut system = VerifiedRewriteSystemV1 {
        manifest_digest: manifest.candidate_digest().clone(),
        carrier_digest: carrier.digest().clone(),
        inventory_digest: inventory.digest().clone(),
        typed_inventory_digest: typed_inventory.digest().clone(),
        signature_digest: inventory.successor_boundary().digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        seed_subject_digest,
        substitutions,
        nodes,
        edges,
        dispositions,
        overlaps,
        substitution_reachability,
        substitution_reachability_digest,
        predecessor_graph_digest,
        digest: Digest::of_bytes(b"pending-finite-rewrite-system"),
    };
    system.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-finite-rewrite-system/v1",
        &system,
    );
    Ok(system)
}

/// Deliberate fail-closed boundary for the current finite prototype.
///
/// Minting remains disabled until one verifier replay closes all three
/// certificate-boundary obligations:
///
/// * recursively typed subterms and binder-local contexts/rule instances;
/// * independent predecessor-universe reconstruction and exact node equality;
/// * every unordered immediate-edge pair, including nested and disjoint
///   positions, in the explicit overlap census.
///
/// The current substitution generators also have an unbounded syntactic
/// composition closure in otherwise tiny examples.  Selecting a prefix at the
/// manifest limit would be truncation, not a finite completeness theorem.
fn verify_complete_rewrite_certificate_boundary_v1() -> VerifyResult<()> {
    Err(AuditUnknownReason::MissingRewriteAdmissibilityTheorem.into())
}

fn verify_bindings(
    kernel: &Kernel,
    manifest: &VerifiedSemanticAuditManifestV1,
    carrier: &PreQ0RawCarrierCertificateV1,
    inventory: &VerifiedPublicAuditInventoryV1,
    typed_inventory: &VerifiedTypedRewriteInventoryV1,
) -> VerifyResult<()> {
    let required_rules = [
        Q0RuleV1::Beta,
        Q0RuleV1::ProvenancePreservingDelta,
        Q0RuleV1::FreshNonrecursiveConstructorComputation,
    ];
    if manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1
        || required_rules
            .iter()
            .any(|rule| !manifest.manifest().q0_rules.contains(rule))
        || manifest
            .manifest()
            .q0_rules
            .contains(&Q0RuleV1::DescriptorForcedProjection)
        || manifest.manifest().substitution_carrier_rules
            != [
                SubstitutionCarrierRuleV1::EmbeddingLift,
                SubstitutionCarrierRuleV1::ForcedNewestArgument,
            ]
        || carrier.manifest_digest() != manifest.candidate_digest()
        || inventory.manifest_digest() != manifest.candidate_digest()
        || typed_inventory.manifest_digest() != manifest.candidate_digest()
    {
        return Err(AuditUnknownReason::ManifestMismatch.into());
    }
    if carrier.signature_digest() != inventory.successor_boundary().digest()
        || typed_inventory.inventory_digest() != inventory.digest()
        || typed_inventory.inventory_coverage_digest() != inventory.coverage().digest()
        || typed_inventory.predecessor_history_digest() != inventory.predecessor_history_digest()
        || typed_inventory.predecessor_boundary_digest()
            != inventory.predecessor_boundary().digest()
        || typed_inventory.successor_boundary_digest() != inventory.successor_boundary().digest()
        || typed_inventory.exact_extension_digest() != inventory.exact_extension().digest()
    {
        return Err(AuditUnknownReason::MalformedInput.into());
    }
    if carrier.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
        || inventory.normalizer_protocol_digest() != &kernel.normalizer_protocol_digest()
        || typed_inventory.normalizer_protocol_digest() != &kernel.normalizer_protocol_digest()
    {
        return Err(AuditUnknownReason::ManifestMismatch.into());
    }
    if !carrier.q3_registry_verified_empty() || !inventory.q3_registry().is_empty() {
        return Err(VerifyFailure::Outside(
            OutsideFragmentReason::NonEmptyQ3Registry,
        ));
    }
    if inventory.successor_boundary().declarations().len()
        != inventory.predecessor_boundary().declarations().len() + 1
        || !inventory.contains_successor_declaration(typed_inventory.fresh_head())
        || inventory.contains_predecessor_declaration(typed_inventory.fresh_head())
    {
        return Err(AuditUnknownReason::MalformedInput.into());
    }
    Ok(())
}

fn verify_carrier_seed_subject_coverage(
    carrier: &PreQ0RawCarrierCertificateV1,
    inventory: &VerifiedPublicAuditInventoryV1,
) -> VerifyResult<Digest> {
    if carrier.verified_seeds().len()
        != inventory.declarations().len() + inventory.equations().len()
    {
        return Err(AuditUnknownReason::IncompleteEnumeration.into());
    }
    let mut matched_declarations = BTreeSet::new();
    let mut matched_equations = BTreeSet::new();
    for verified in carrier.verified_seeds() {
        match verified.seed() {
            SemanticSchemaSeedV1::PublicHead(seed) => {
                let matches = inventory
                    .declarations()
                    .iter()
                    .filter(|declaration| {
                        declaration.declaration() == &seed.declaration
                            && declaration.origin() == &seed.origin_event
                            && declaration.source_identity() == &seed.judgment.source_identity
                            && seed.judgment.source
                                == (GenericJudgmentV1::Term {
                                    context: DependentContext::default(),
                                    term: Term::Global {
                                        id: declaration.declaration().clone(),
                                    },
                                    ty: declaration.source().ty.clone(),
                                })
                            && verified.source_judgment() == &seed.judgment.source
                    })
                    .collect::<Vec<_>>();
                let [declaration] = matches.as_slice() else {
                    return Err(AuditUnknownReason::IncompleteEnumeration.into());
                };
                if !matched_declarations.insert(declaration.declaration().clone()) {
                    return Err(AuditUnknownReason::ProvenanceCollision.into());
                }
            }
            SemanticSchemaSeedV1::PublicEquation(seed) => {
                let matches = inventory
                    .equations()
                    .iter()
                    .filter(|equation| {
                        equation.equation() == &seed.equation
                            && equation.owner_head() == &seed.owner_head
                            && equation.origin() == &seed.origin_event
                            && equation.source_identity() == &seed.judgment.source_identity
                            && equation.source() == &seed.judgment.source
                            && verified.source_judgment() == &seed.judgment.source
                    })
                    .collect::<Vec<_>>();
                let [equation] = matches.as_slice() else {
                    return Err(AuditUnknownReason::IncompleteEnumeration.into());
                };
                if !matched_equations.insert(equation.equation().clone()) {
                    return Err(AuditUnknownReason::ProvenanceCollision.into());
                }
            }
            SemanticSchemaSeedV1::PublicUniversalInterface { .. } => {
                return Err(AuditUnknownReason::IncompleteEnumeration.into());
            }
        }
    }
    if matched_declarations.len() != inventory.declarations().len()
        || matched_equations.len() != inventory.equations().len()
    {
        return Err(AuditUnknownReason::IncompleteEnumeration.into());
    }
    struct Material<'a> {
        carrier: &'a Digest,
        inventory: &'a Digest,
        declaration_count: u64,
        equation_count: u64,
    }
    impl CanonicalEncode for Material<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.carrier.encode_canonical(encoder);
            self.inventory.encode_canonical(encoder);
            encoder.u64(self.declaration_count);
            encoder.u64(self.equation_count);
        }
    }
    Ok(Digest::of_canonical(
        "pen-semantic-audit/verified-seed-term-subject-census/v1",
        &Material {
            carrier: carrier.digest(),
            inventory: inventory.digest(),
            declaration_count: matched_declarations.len() as u64,
            equation_count: matched_equations.len() as u64,
        },
    ))
}

fn verify_projection_free(inventory: &VerifiedPublicAuditInventoryV1) -> VerifyResult<()> {
    if !inventory.forced_projections().is_empty()
        || inventory.coverage().forced_projection_count() != 0
        || !inventory.forced_projection_origins().is_empty()
    {
        return Err(VerifyFailure::Outside(
            OutsideFragmentReason::DescriptorProjection,
        ));
    }
    Ok(())
}

fn validate_input_fragment(
    manifest: &VerifiedSemanticAuditManifestV1,
    carrier: &PreQ0RawCarrierCertificateV1,
    inventory: &VerifiedPublicAuditInventoryV1,
    typed_inventory: &VerifiedTypedRewriteInventoryV1,
) -> VerifyResult<()> {
    let levels = &manifest.manifest().universe_levels;
    for declaration in inventory.successor_boundary().declarations() {
        validate_lambda_unit_term(&declaration.ty, levels)?;
        if let Some(body) = &declaration.body {
            validate_lambda_unit_term(body, levels)?;
        }
    }
    for declaration in inventory.declarations() {
        validate_lambda_unit_term(&declaration.source().ty, levels)?;
        validate_lambda_unit_term(&declaration.normalized().ty, levels)?;
        if let Some(body) = &declaration.source().body {
            validate_lambda_unit_term(body, levels)?;
        }
        if let Some(body) = &declaration.normalized().body {
            validate_lambda_unit_term(body, levels)?;
        }
    }
    for equation in inventory.equations() {
        validate_lambda_unit_judgment(equation.source(), levels)?;
        validate_lambda_unit_judgment(equation.normalized(), levels)?;
    }
    for demand in inventory.predecessor_demand_contracts() {
        validate_lambda_unit_judgment(demand.source_requirement(), levels)?;
        validate_lambda_unit_judgment(demand.normalized_requirement(), levels)?;
    }
    for seed in carrier.verified_seeds() {
        validate_lambda_unit_judgment(seed.source_judgment(), levels)?;
    }
    for raw in carrier.raw_families() {
        validate_lambda_unit_judgment(&raw.generic_judgment, levels)?;
    }
    for witness in carrier.context_witnesses() {
        for term in witness
            .left_context
            .0
            .iter()
            .chain(&witness.right_context.0)
            .chain(&witness.target_context.0)
        {
            validate_lambda_unit_term(term, levels)?;
        }
    }
    for entry in typed_inventory.entries() {
        for term in entry
            .context()
            .0
            .iter()
            .chain([entry.left(), entry.right(), entry.ty()])
        {
            validate_lambda_unit_term(term, levels)?;
        }
    }
    Ok(())
}

fn validate_lambda_unit_judgment(judgment: &GenericJudgmentV1, levels: &[u16]) -> VerifyResult<()> {
    match judgment {
        GenericJudgmentV1::Term { context, term, ty } => {
            for entry in &context.0 {
                validate_lambda_unit_term(entry, levels)?;
            }
            validate_lambda_unit_term(term, levels)?;
            validate_lambda_unit_term(ty, levels)
        }
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } => {
            for entry in &context.0 {
                validate_lambda_unit_term(entry, levels)?;
            }
            validate_lambda_unit_term(left, levels)?;
            validate_lambda_unit_term(right, levels)?;
            validate_lambda_unit_term(ty, levels)
        }
    }
}

fn validate_lambda_unit_term(term: &Term, levels: &[u16]) -> VerifyResult<()> {
    match lambda_unit_term_syntax_violation(term, levels) {
        Some(violation) => Err(VerifyFailure::Outside(violation.outside_reason())),
        None => Ok(()),
    }
}

fn collect_initial_nodes(
    carrier: &PreQ0RawCarrierCertificateV1,
    inventory: &VerifiedPublicAuditInventoryV1,
    typed_inventory: &VerifiedTypedRewriteInventoryV1,
) -> VerifyResult<Vec<NodeCandidate>> {
    let mut nodes = Vec::new();
    let mut indices = BTreeMap::new();
    let unlimited = usize::MAX;

    for declaration in inventory.successor_boundary().declarations() {
        push_initial_formation_node(
            &mut nodes,
            &mut indices,
            DependentContext::default(),
            declaration.ty.clone(),
            unlimited,
        )?;
        push_initial_node(
            &mut nodes,
            &mut indices,
            DependentContext::default(),
            Term::Global {
                id: declaration.id.clone(),
            },
            declaration.ty.clone(),
            unlimited,
        )?;
        if let Some(body) = &declaration.body {
            push_initial_node(
                &mut nodes,
                &mut indices,
                DependentContext::default(),
                body.clone(),
                declaration.ty.clone(),
                unlimited,
            )?;
        }
    }

    for declaration in inventory.declarations() {
        for wire in [declaration.source(), declaration.normalized()] {
            push_initial_formation_node(
                &mut nodes,
                &mut indices,
                DependentContext::default(),
                wire.ty.clone(),
                unlimited,
            )?;
            push_initial_node(
                &mut nodes,
                &mut indices,
                DependentContext::default(),
                Term::Global {
                    id: wire.id.clone(),
                },
                wire.ty.clone(),
                unlimited,
            )?;
            if let Some(body) = &wire.body {
                push_initial_node(
                    &mut nodes,
                    &mut indices,
                    DependentContext::default(),
                    body.clone(),
                    wire.ty.clone(),
                    unlimited,
                )?;
            }
        }
    }

    for equation in inventory.equations() {
        add_judgment_nodes(equation.source(), &mut nodes, &mut indices, unlimited)?;
        add_judgment_nodes(equation.normalized(), &mut nodes, &mut indices, unlimited)?;
    }
    for demand in inventory.predecessor_demand_contracts() {
        add_judgment_nodes(
            demand.source_requirement(),
            &mut nodes,
            &mut indices,
            unlimited,
        )?;
        add_judgment_nodes(
            demand.normalized_requirement(),
            &mut nodes,
            &mut indices,
            unlimited,
        )?;
    }
    for seed in carrier.verified_seeds() {
        add_judgment_nodes(seed.source_judgment(), &mut nodes, &mut indices, unlimited)?;
    }
    for raw in carrier.raw_families() {
        add_judgment_nodes(&raw.generic_judgment, &mut nodes, &mut indices, unlimited)?;
    }
    for entry in typed_inventory.entries() {
        add_context_formation_nodes(entry.context(), &mut nodes, &mut indices, unlimited)?;
        push_initial_formation_node(
            &mut nodes,
            &mut indices,
            entry.context().clone(),
            entry.ty().clone(),
            unlimited,
        )?;
        push_initial_node(
            &mut nodes,
            &mut indices,
            entry.context().clone(),
            entry.left().clone(),
            entry.ty().clone(),
            unlimited,
        )?;
        push_initial_node(
            &mut nodes,
            &mut indices,
            entry.context().clone(),
            entry.right().clone(),
            entry.ty().clone(),
            unlimited,
        )?;
    }
    for witness in carrier.context_witnesses() {
        for context in [
            &witness.left_context,
            &witness.right_context,
            &witness.target_context,
        ] {
            add_context_formation_nodes(context, &mut nodes, &mut indices, unlimited)?;
        }
    }
    Ok(nodes)
}

fn add_judgment_nodes(
    judgment: &GenericJudgmentV1,
    nodes: &mut Vec<NodeCandidate>,
    indices: &mut BTreeMap<RewriteNodeIdV1, usize>,
    limit: usize,
) -> VerifyResult<()> {
    add_context_formation_nodes(judgment.context(), nodes, indices, limit)?;
    match judgment {
        GenericJudgmentV1::Term { context, term, ty } => {
            push_initial_formation_node(nodes, indices, context.clone(), ty.clone(), limit)?;
            push_initial_node(
                nodes,
                indices,
                context.clone(),
                term.clone(),
                ty.clone(),
                limit,
            )
        }
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } => {
            push_initial_formation_node(nodes, indices, context.clone(), ty.clone(), limit)?;
            push_initial_node(
                nodes,
                indices,
                context.clone(),
                left.clone(),
                ty.clone(),
                limit,
            )?;
            push_initial_node(
                nodes,
                indices,
                context.clone(),
                right.clone(),
                ty.clone(),
                limit,
            )
        }
    }
}

fn add_context_formation_nodes(
    context: &DependentContext,
    nodes: &mut Vec<NodeCandidate>,
    indices: &mut BTreeMap<RewriteNodeIdV1, usize>,
    limit: usize,
) -> VerifyResult<()> {
    for (ordinal, entry) in context.0.iter().enumerate() {
        push_initial_formation_node(
            nodes,
            indices,
            DependentContext(context.0[..ordinal].to_vec()),
            entry.clone(),
            limit,
        )?;
    }
    Ok(())
}

fn push_initial_node(
    nodes: &mut Vec<NodeCandidate>,
    indices: &mut BTreeMap<RewriteNodeIdV1, usize>,
    context: DependentContext,
    term: Term,
    ty: Term,
    limit: usize,
) -> VerifyResult<()> {
    insert_candidate(
        make_node_candidate(context, term, ty),
        nodes,
        indices,
        limit,
    )
    .map(|_| ())
}

fn push_initial_formation_node(
    nodes: &mut Vec<NodeCandidate>,
    indices: &mut BTreeMap<RewriteNodeIdV1, usize>,
    context: DependentContext,
    term: Term,
    limit: usize,
) -> VerifyResult<()> {
    insert_candidate(
        make_formation_candidate(context, term),
        nodes,
        indices,
        limit,
    )
    .map(|_| ())
}

fn make_node_candidate(context: DependentContext, term: Term, ty: Term) -> NodeCandidate {
    make_node_candidate_with_judgment(context, term, RewriteNodeJudgmentV1::HasType { ty })
}

fn make_formation_candidate(context: DependentContext, term: Term) -> NodeCandidate {
    make_node_candidate_with_judgment(context, term, RewriteNodeJudgmentV1::TypeFormation)
}

fn make_node_candidate_with_judgment(
    context: DependentContext,
    term: Term,
    judgment: RewriteNodeJudgmentV1,
) -> NodeCandidate {
    struct Material<'a> {
        context: &'a DependentContext,
        term: &'a Term,
        judgment: &'a RewriteNodeJudgmentV1,
    }
    impl CanonicalEncode for Material<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.context.encode_canonical(encoder);
            self.term.encode_canonical(encoder);
            self.judgment.encode_canonical(encoder);
        }
    }
    let id = RewriteNodeIdV1(Digest::of_canonical(
        "pen-semantic-audit/finite-rewrite-node/v1",
        &Material {
            context: &context,
            term: &term,
            judgment: &judgment,
        },
    ));
    NodeCandidate {
        id,
        context,
        term,
        judgment,
    }
}

fn insert_candidate(
    candidate: NodeCandidate,
    nodes: &mut Vec<NodeCandidate>,
    indices: &mut BTreeMap<RewriteNodeIdV1, usize>,
    limit: usize,
) -> VerifyResult<usize> {
    if let Some(index) = indices.get(&candidate.id).copied() {
        if nodes[index] != candidate {
            return Err(AuditUnknownReason::ProvenanceCollision.into());
        }
        return Ok(index);
    }
    if nodes.len() >= limit {
        return Err(AuditUnknownReason::ResourceExhausted.into());
    }
    let index = nodes.len();
    indices.insert(candidate.id.clone(), index);
    nodes.push(candidate);
    Ok(index)
}

fn collect_contexts(
    candidates: &[NodeCandidate],
    witnesses: &[ContextAmalgamationWitnessV1],
) -> Vec<DependentContext> {
    let mut contexts = Vec::new();
    for context in std::iter::once(DependentContext::default())
        .chain(candidates.iter().map(|candidate| candidate.context.clone()))
        .chain(witnesses.iter().flat_map(|witness| {
            [
                witness.left_context.clone(),
                witness.right_context.clone(),
                witness.target_context.clone(),
            ]
        }))
    {
        if !contexts.contains(&context) {
            contexts.push(context);
        }
    }
    let complete_contexts = contexts.clone();
    for context in complete_contexts {
        for prefix_len in 0..context.0.len() {
            let prefix = DependentContext(context.0[..prefix_len].to_vec());
            if !contexts.contains(&prefix) {
                contexts.push(prefix);
            }
        }
    }
    contexts
}

fn derive_substitutions(
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    witnesses: &[ContextAmalgamationWitnessV1],
    contexts: &[DependentContext],
    candidates: &[NodeCandidate],
    limit: usize,
) -> VerifyResult<Vec<VerifiedFiniteSubstitutionV1>> {
    let mut substitutions = Vec::new();
    let mut indices = BTreeMap::new();

    for context in contexts.iter() {
        kernel
            .verify_context(inventory.successor_boundary(), context)
            .map_err(kernel_failure)?;
        let embedding = (0..context.0.len())
            .map(|ordinal| {
                u32::try_from(ordinal)
                    .map_err(|_| VerifyFailure::Unknown(AuditUnknownReason::ResourceExhausted))
            })
            .collect::<VerifyResult<Vec<_>>>()?;
        insert_substitution(
            make_embedding_substitution(
                FiniteSubstitutionKindV1::Identity,
                context.clone(),
                context.clone(),
                embedding,
            ),
            &mut substitutions,
            &mut indices,
            limit,
        )?;
    }

    for witness in witnesses {
        for (source, embedding) in [
            (&witness.left_context, &witness.left_embedding),
            (&witness.right_context, &witness.right_embedding),
        ] {
            let substitution = make_embedding_substitution(
                if source == &witness.target_context
                    && embedding.iter().copied().eq((0_u32..).take(source.0.len()))
                {
                    FiniteSubstitutionKindV1::Identity
                } else {
                    FiniteSubstitutionKindV1::ContextEmbeddingLift
                },
                source.clone(),
                witness.target_context.clone(),
                embedding.clone(),
            );
            validate_substitution(kernel, inventory, &substitution)?;
            insert_substitution(substitution, &mut substitutions, &mut indices, limit)?;
        }
    }

    // Carrier witnesses are positive anchors, not negative evidence. Exhaust
    // every order-preserving embedding between contexts in the finite
    // universe so an omitted witness cannot be mistaken for inapplicability.
    let mut embedding_attempts = 0_usize;
    for source in contexts.iter() {
        for target in contexts.iter() {
            if source.0.len() > target.0.len() {
                continue;
            }
            let embeddings = enumerate_monotone_embeddings(
                source.0.len(),
                target.0.len(),
                limit.saturating_sub(embedding_attempts),
            )?;
            embedding_attempts = embedding_attempts
                .checked_add(embeddings.len())
                .ok_or(AuditUnknownReason::ResourceExhausted)?;
            if embedding_attempts > limit {
                return Err(AuditUnknownReason::ResourceExhausted.into());
            }
            for embedding in embeddings {
                let kind = if source == target
                    && embedding.iter().copied().eq((0_u32..).take(source.0.len()))
                {
                    FiniteSubstitutionKindV1::Identity
                } else {
                    FiniteSubstitutionKindV1::ContextEmbeddingLift
                };
                let candidate =
                    make_embedding_substitution(kind, source.clone(), target.clone(), embedding);
                if embedding_respects_context(&candidate) {
                    insert_substitution(candidate, &mut substitutions, &mut indices, limit)?;
                }
            }
        }
    }

    // Forced-newest generators have the exact form Γ,A -> Γ.  The old
    // variables map identically and the newest image is every enumerated
    // kernel-typed a:A in Γ.
    for source in contexts.iter().filter(|context| !context.0.is_empty()) {
        let prefix_len = source.0.len() - 1;
        let prefix = DependentContext(source.0[..prefix_len].to_vec());
        let Some(_) = contexts.iter().find(|context| **context == prefix) else {
            return Err(AuditUnknownReason::IncompleteEnumeration.into());
        };
        for argument in candidates.iter().filter(|node| node.context == prefix) {
            if kernel_check_term_against_signature(
                kernel,
                inventory.successor_boundary(),
                &prefix,
                &argument.term,
                &source.0[prefix_len],
            )
            .is_err()
            {
                continue;
            }
            let mut images = identity_images(&prefix)?;
            images.push(argument.term.clone());
            let forced = make_general_substitution(
                FiniteSubstitutionKindV1::ForcedNewestArgument,
                source.clone(),
                prefix.clone(),
                images,
            );
            validate_substitution(kernel, inventory, &forced)?;
            insert_substitution(forced, &mut substitutions, &mut indices, limit)?;
        }
    }

    // Close the complete image vectors under composition.  If σ: Γ→Δ and
    // τ: Δ→Θ, the oldest-first images of τ∘σ are σ's images instantiated by
    // τ.
    let mut changed = true;
    while changed {
        changed = false;
        let snapshot = substitutions.clone();
        for first in &snapshot {
            for second in &snapshot {
                if first.target_context != second.source_context {
                    continue;
                }
                let composed = compose_substitutions(first, second)?;
                if !indices.contains_key(&composed.id) {
                    validate_substitution(kernel, inventory, &composed)?;
                    insert_substitution(composed, &mut substitutions, &mut indices, limit)?;
                    changed = true;
                }
            }
        }
    }
    substitutions.sort_by(|left, right| left.id.cmp(&right.id));
    Ok(substitutions)
}

fn compose_substitutions(
    first: &VerifiedFiniteSubstitutionV1,
    second: &VerifiedFiniteSubstitutionV1,
) -> VerifyResult<VerifiedFiniteSubstitutionV1> {
    if first.target_context != second.source_context {
        return Err(AuditUnknownReason::MalformedInput.into());
    }
    let images = first
        .images
        .iter()
        .map(|image| {
            instantiate_term_images(image, first.target_context.0.len(), &second.images, 0)
                .ok_or(AuditUnknownReason::MalformedInput)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(make_general_substitution(
        FiniteSubstitutionKindV1::Composite,
        first.source_context.clone(),
        second.target_context.clone(),
        images,
    ))
}

fn enumerate_monotone_embeddings(
    source_len: usize,
    target_len: usize,
    limit: usize,
) -> VerifyResult<Vec<Vec<u32>>> {
    fn visit(
        remaining: usize,
        next_target: usize,
        target_len: usize,
        limit: usize,
        prefix: &mut Vec<u32>,
        output: &mut Vec<Vec<u32>>,
    ) -> VerifyResult<()> {
        if output.len() >= limit {
            return Err(AuditUnknownReason::ResourceExhausted.into());
        }
        if remaining == 0 {
            output.push(prefix.clone());
            return Ok(());
        }
        let last_start = target_len
            .checked_sub(remaining)
            .ok_or(AuditUnknownReason::MalformedInput)?;
        for target in next_target..=last_start {
            prefix.push(u32::try_from(target).map_err(|_| AuditUnknownReason::ResourceExhausted)?);
            visit(remaining - 1, target + 1, target_len, limit, prefix, output)?;
            prefix.pop();
        }
        Ok(())
    }

    if source_len > target_len {
        return Ok(Vec::new());
    }
    if limit == 0 {
        return Err(AuditUnknownReason::ResourceExhausted.into());
    }
    let mut output = Vec::new();
    visit(
        source_len,
        0,
        target_len,
        limit,
        &mut Vec::new(),
        &mut output,
    )?;
    Ok(output)
}

fn make_embedding_substitution(
    kind: FiniteSubstitutionKindV1,
    source_context: DependentContext,
    target_context: DependentContext,
    embedding: Vec<u32>,
) -> VerifiedFiniteSubstitutionV1 {
    let images = embedding
        .iter()
        .map(|target_ordinal| {
            let target_ordinal = usize::try_from(*target_ordinal).ok()?;
            let free_index = target_context
                .0
                .len()
                .checked_sub(target_ordinal.checked_add(1)?)?;
            Some(Term::Var {
                index: u32::try_from(free_index).ok()?,
            })
        })
        .collect::<Option<Vec<_>>>()
        .unwrap_or_default();
    make_substitution_with_parts(
        kind,
        source_context,
        target_context,
        Some(embedding),
        images,
    )
}

fn make_general_substitution(
    kind: FiniteSubstitutionKindV1,
    source_context: DependentContext,
    target_context: DependentContext,
    images: Vec<Term>,
) -> VerifiedFiniteSubstitutionV1 {
    make_substitution_with_parts(kind, source_context, target_context, None, images)
}

fn make_substitution_with_parts(
    kind: FiniteSubstitutionKindV1,
    source_context: DependentContext,
    target_context: DependentContext,
    embedding: Option<Vec<u32>>,
    images: Vec<Term>,
) -> VerifiedFiniteSubstitutionV1 {
    struct Material<'a> {
        source_context: &'a DependentContext,
        target_context: &'a DependentContext,
        images: &'a [Term],
    }
    impl CanonicalEncode for Material<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.source_context.encode_canonical(encoder);
            self.target_context.encode_canonical(encoder);
            encoder.sequence(self.images);
        }
    }
    let id = RewriteSubstitutionIdV1(Digest::of_canonical(
        "pen-semantic-audit/finite-context-substitution/v1",
        &Material {
            source_context: &source_context,
            target_context: &target_context,
            images: &images,
        },
    ));
    VerifiedFiniteSubstitutionV1 {
        id,
        kind,
        source_context,
        target_context,
        embedding,
        images,
    }
}

fn identity_images(context: &DependentContext) -> VerifyResult<Vec<Term>> {
    (0..context.0.len())
        .map(|ordinal| {
            let free_index = context
                .0
                .len()
                .checked_sub(ordinal + 1)
                .ok_or(AuditUnknownReason::MalformedInput)?;
            Ok(Term::Var {
                index: u32::try_from(free_index)
                    .map_err(|_| AuditUnknownReason::ResourceExhausted)?,
            })
        })
        .collect::<Result<Vec<_>, AuditUnknownReason>>()
        .map_err(VerifyFailure::Unknown)
}

fn insert_substitution(
    substitution: VerifiedFiniteSubstitutionV1,
    substitutions: &mut Vec<VerifiedFiniteSubstitutionV1>,
    indices: &mut BTreeMap<RewriteSubstitutionIdV1, usize>,
    limit: usize,
) -> VerifyResult<usize> {
    if let Some(index) = indices.get(&substitution.id).copied() {
        let existing = &substitutions[index];
        if existing.source_context != substitution.source_context
            || existing.target_context != substitution.target_context
            || existing.images != substitution.images
        {
            return Err(AuditUnknownReason::ProvenanceCollision.into());
        }
        return Ok(index);
    }
    if substitutions.len() >= limit {
        return Err(AuditUnknownReason::ResourceExhausted.into());
    }
    let index = substitutions.len();
    indices.insert(substitution.id.clone(), index);
    substitutions.push(substitution);
    Ok(index)
}

fn validate_substitution(
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    substitution: &VerifiedFiniteSubstitutionV1,
) -> VerifyResult<()> {
    kernel
        .verify_context(inventory.successor_boundary(), &substitution.source_context)
        .map_err(kernel_failure)?;
    kernel
        .verify_context(inventory.successor_boundary(), &substitution.target_context)
        .map_err(kernel_failure)?;
    if substitution.images.len() != substitution.source_context.0.len() {
        return Err(AuditUnknownReason::MalformedInput.into());
    }
    match substitution.kind {
        FiniteSubstitutionKindV1::Identity | FiniteSubstitutionKindV1::ContextEmbeddingLift => {
            if !embedding_respects_context(substitution) {
                return Err(AuditUnknownReason::MalformedInput.into());
            }
        }
        FiniteSubstitutionKindV1::ForcedNewestArgument => {
            let source_len = substitution.source_context.0.len();
            let prefix_len = source_len
                .checked_sub(1)
                .ok_or(AuditUnknownReason::MalformedInput)?;
            let prefix = DependentContext(substitution.source_context.0[..prefix_len].to_vec());
            if substitution.embedding.is_some()
                || substitution.target_context != prefix
                || substitution.images[..prefix_len] != identity_images(&prefix)?
            {
                return Err(AuditUnknownReason::MalformedInput.into());
            }
        }
        FiniteSubstitutionKindV1::Composite => {
            if substitution.embedding.is_some() {
                return Err(AuditUnknownReason::MalformedInput.into());
            }
        }
    }
    for (ordinal, (source_entry, image)) in substitution
        .source_context
        .0
        .iter()
        .zip(&substitution.images)
        .enumerate()
    {
        let expected_type =
            instantiate_term_images(source_entry, ordinal, &substitution.images[..ordinal], 0)
                .ok_or(AuditUnknownReason::MalformedInput)?;
        kernel_check_term(
            kernel,
            inventory,
            &substitution.target_context,
            image,
            &expected_type,
        )?;
    }
    Ok(())
}

fn embedding_respects_context(substitution: &VerifiedFiniteSubstitutionV1) -> bool {
    let Some(embedding) = &substitution.embedding else {
        return false;
    };
    if embedding.len() != substitution.source_context.0.len()
        || embedding.windows(2).any(|window| window[0] >= window[1])
        || embedding.last().is_some_and(|last| {
            usize::try_from(*last).map_or(true, |last| last >= substitution.target_context.0.len())
        })
    {
        return false;
    }

    for (source_ordinal, source_entry) in substitution.source_context.0.iter().enumerate() {
        let Ok(target_ordinal) = usize::try_from(embedding[source_ordinal]) else {
            return false;
        };
        let source_prefix =
            DependentContext(substitution.source_context.0[..source_ordinal].to_vec());
        let target_prefix =
            DependentContext(substitution.target_context.0[..target_ordinal].to_vec());
        let Some(renamed) = rename_term_between_contexts(
            &source_prefix,
            &target_prefix,
            &embedding[..source_ordinal],
            source_entry,
        ) else {
            return false;
        };
        if substitution.target_context.0.get(target_ordinal) != Some(&renamed) {
            return false;
        }
    }
    let expected_images = embedding
        .iter()
        .map(|target_ordinal| {
            let target_ordinal = usize::try_from(*target_ordinal).ok()?;
            let free_index = substitution
                .target_context
                .0
                .len()
                .checked_sub(target_ordinal.checked_add(1)?)?;
            Some(Term::Var {
                index: u32::try_from(free_index).ok()?,
            })
        })
        .collect::<Option<Vec<_>>>();
    expected_images.as_ref() == Some(&substitution.images)
}

fn build_rule_instances(
    kernel: &Kernel,
    manifest: &VerifiedSemanticAuditManifestV1,
    inventory: &VerifiedPublicAuditInventoryV1,
    typed_inventory: &VerifiedTypedRewriteInventoryV1,
    substitutions: &[VerifiedFiniteSubstitutionV1],
) -> VerifyResult<Vec<RuleInstance>> {
    let structural = [
        FiniteRewriteRuleV1::CanonicalDeBruijn,
        FiniteRewriteRuleV1::SequentialSubstitution,
        FiniteRewriteRuleV1::OrdinaryBeta,
        FiniteRewriteRuleV1::CanonicalUnit,
        FiniteRewriteRuleV1::FlattenedTelescope,
    ];
    let mut rules = structural
        .into_iter()
        .map(|rule| RuleInstance {
            rule,
            target_context: None,
            left: None,
            right: Term::Unit,
        })
        .collect::<Vec<_>>();
    let mut public_delta = BTreeSet::new();
    for declaration in inventory.declarations() {
        let normalized = declaration.normalized();
        let Some(body) = &normalized.body else {
            continue;
        };
        if normalized.id != *declaration.declaration()
            || !public_delta.insert(normalized.id.clone())
        {
            return Err(AuditUnknownReason::ProvenanceCollision.into());
        }
        rules.push(RuleInstance {
            rule: FiniteRewriteRuleV1::PublicDelta {
                declaration: normalized.id.clone(),
            },
            target_context: None,
            left: None,
            right: body.clone(),
        });
    }
    if public_delta.is_empty() {
        rules.push(RuleInstance {
            rule: FiniteRewriteRuleV1::NoPublicDeltaDeclarations,
            target_context: None,
            left: None,
            right: Term::Unit,
        });
    }

    let mut fresh_rule_keys = BTreeSet::new();
    for entry in typed_inventory.entries() {
        verify_typed_entry_binding(inventory, typed_inventory, entry)?;
        let mut instance_count = 0_usize;
        for substitution in substitutions
            .iter()
            .filter(|substitution| substitution.source_context == *entry.context())
        {
            let source_len = entry.context().0.len();
            let left = instantiate_term_images(entry.left(), source_len, substitution.images(), 0)
                .ok_or(AuditUnknownReason::MalformedInput)?;
            let right =
                instantiate_term_images(entry.right(), source_len, substitution.images(), 0)
                    .ok_or(AuditUnknownReason::MalformedInput)?;
            let ty = instantiate_term_images(entry.ty(), source_len, substitution.images(), 0)
                .ok_or(AuditUnknownReason::MalformedInput)?;
            kernel_check_term(kernel, inventory, substitution.target_context(), &left, &ty)?;
            kernel_check_term(
                kernel,
                inventory,
                substitution.target_context(),
                &right,
                &ty,
            )?;
            let rule = FiniteRewriteRuleV1::TypedFreshEquation {
                equation: entry.equation().clone(),
                substitution: substitution.id().clone(),
            };
            if !fresh_rule_keys.insert(rule.clone()) {
                return Err(AuditUnknownReason::ProvenanceCollision.into());
            }
            rules.push(RuleInstance {
                rule,
                target_context: Some(substitution.target_context().clone()),
                left: Some(left),
                right,
            });
            instance_count += 1;
        }
        if instance_count == 0 {
            return Err(AuditUnknownReason::IncompleteEnumeration.into());
        }
    }
    rules.sort_by_cached_key(|instance| {
        Digest::of_canonical("pen-semantic-audit/finite-rewrite-rule/v1", &instance.rule)
    });
    verify_manifest_rule_coverage(&manifest.manifest().q0_rules, &rules)?;
    Ok(rules)
}

fn verify_manifest_rule_coverage(
    manifest_rules: &[Q0RuleV1],
    rules: &[RuleInstance],
) -> VerifyResult<()> {
    for manifest_rule in manifest_rules {
        let covered = match manifest_rule {
            Q0RuleV1::DeBruijn => rules
                .iter()
                .any(|rule| rule.rule == FiniteRewriteRuleV1::CanonicalDeBruijn),
            Q0RuleV1::SequentialSubstitution => rules
                .iter()
                .any(|rule| rule.rule == FiniteRewriteRuleV1::SequentialSubstitution),
            Q0RuleV1::Beta => rules
                .iter()
                .any(|rule| rule.rule == FiniteRewriteRuleV1::OrdinaryBeta),
            Q0RuleV1::ProvenancePreservingDelta => rules.iter().any(|rule| {
                matches!(
                    rule.rule,
                    FiniteRewriteRuleV1::PublicDelta { .. }
                        | FiniteRewriteRuleV1::NoPublicDeltaDeclarations
                )
            }),
            Q0RuleV1::Unit => rules
                .iter()
                .any(|rule| rule.rule == FiniteRewriteRuleV1::CanonicalUnit),
            Q0RuleV1::TelescopeFlattening => rules
                .iter()
                .any(|rule| rule.rule == FiniteRewriteRuleV1::FlattenedTelescope),
            Q0RuleV1::FreshNonrecursiveConstructorComputation => rules
                .iter()
                .any(|rule| matches!(rule.rule, FiniteRewriteRuleV1::TypedFreshEquation { .. })),
            Q0RuleV1::DescriptorForcedProjection => false,
        };
        if !covered {
            return Err(AuditUnknownReason::IncompleteEnumeration.into());
        }
    }
    Ok(())
}

fn verify_typed_entry_binding(
    inventory: &VerifiedPublicAuditInventoryV1,
    typed_inventory: &VerifiedTypedRewriteInventoryV1,
    entry: &VerifiedTypedRewriteEntryV1,
) -> VerifyResult<()> {
    if entry.owner_head() != typed_inventory.fresh_head() {
        return Err(AuditUnknownReason::MalformedInput.into());
    }
    let matches = inventory
        .equations()
        .iter()
        .filter(|equation| equation.equation() == entry.equation())
        .collect::<Vec<_>>();
    let [equation] = matches.as_slice() else {
        return Err(AuditUnknownReason::IncompleteEnumeration.into());
    };
    if equation.is_predecessor_public()
        || equation.owner_head() != entry.owner_head()
        || equation.source_identity() != entry.source_identity()
        || equation.demand_port() != entry.demand_port()
    {
        return Err(AuditUnknownReason::MalformedInput.into());
    }
    Ok(())
}

fn rename_term_between_contexts(
    source_context: &DependentContext,
    target_context: &DependentContext,
    embedding: &[u32],
    term: &Term,
) -> Option<Term> {
    let judgment = GenericJudgmentV1::Term {
        context: source_context.clone(),
        term: term.clone(),
        ty: Term::UnitType,
    };
    match rename_judgment_into(&judgment, target_context, embedding)? {
        GenericJudgmentV1::Term { term, .. } => Some(term),
        GenericJudgmentV1::Equation { .. } => None,
    }
}

fn instantiate_term_images(
    term: &Term,
    source_context_len: usize,
    images: &[Term],
    binder_depth: u32,
) -> Option<Term> {
    if images.len() != source_context_len {
        return None;
    }
    match term {
        Term::Sort { .. } | Term::Global { .. } | Term::UnitType | Term::Unit => Some(term.clone()),
        Term::Var { index } if *index < binder_depth => Some(term.clone()),
        Term::Var { index } => {
            let free_index = usize::try_from(index.checked_sub(binder_depth)?).ok()?;
            let source_ordinal = source_context_len.checked_sub(free_index.checked_add(1)?)?;
            shift_term_structural(images.get(source_ordinal)?, i64::from(binder_depth), 0)
        }
        Term::Pi { parameter, body } => Some(Term::Pi {
            parameter: Box::new(instantiate_term_images(
                parameter,
                source_context_len,
                images,
                binder_depth,
            )?),
            body: Box::new(instantiate_term_images(
                body,
                source_context_len,
                images,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Sigma { parameter, body } => Some(Term::Sigma {
            parameter: Box::new(instantiate_term_images(
                parameter,
                source_context_len,
                images,
                binder_depth,
            )?),
            body: Box::new(instantiate_term_images(
                body,
                source_context_len,
                images,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Some(Term::Lambda {
            parameter_type: Box::new(instantiate_term_images(
                parameter_type,
                source_context_len,
                images,
                binder_depth,
            )?),
            body: Box::new(instantiate_term_images(
                body,
                source_context_len,
                images,
                binder_depth.checked_add(1)?,
            )?),
        }),
        Term::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(instantiate_term_images(
                function,
                source_context_len,
                images,
                binder_depth,
            )?),
            argument: Box::new(instantiate_term_images(
                argument,
                source_context_len,
                images,
                binder_depth,
            )?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Some(Term::Pair {
            sigma_type: Box::new(instantiate_term_images(
                sigma_type,
                source_context_len,
                images,
                binder_depth,
            )?),
            first: Box::new(instantiate_term_images(
                first,
                source_context_len,
                images,
                binder_depth,
            )?),
            second: Box::new(instantiate_term_images(
                second,
                source_context_len,
                images,
                binder_depth,
            )?),
        }),
        Term::First { pair } => Some(Term::First {
            pair: Box::new(instantiate_term_images(
                pair,
                source_context_len,
                images,
                binder_depth,
            )?),
        }),
        Term::Second { pair } => Some(Term::Second {
            pair: Box::new(instantiate_term_images(
                pair,
                source_context_len,
                images,
                binder_depth,
            )?),
        }),
    }
}

fn shift_term_structural(term: &Term, amount: i64, cutoff: u32) -> Option<Term> {
    match term {
        Term::Var { index } if *index >= cutoff => {
            let index = i64::from(*index)
                .checked_add(amount)
                .and_then(|index| u32::try_from(index).ok())?;
            Some(Term::Var { index })
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Some(term.clone()),
        Term::Pi { parameter, body } => Some(Term::Pi {
            parameter: Box::new(shift_term_structural(parameter, amount, cutoff)?),
            body: Box::new(shift_term_structural(body, amount, cutoff.checked_add(1)?)?),
        }),
        Term::Sigma { parameter, body } => Some(Term::Sigma {
            parameter: Box::new(shift_term_structural(parameter, amount, cutoff)?),
            body: Box::new(shift_term_structural(body, amount, cutoff.checked_add(1)?)?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Some(Term::Lambda {
            parameter_type: Box::new(shift_term_structural(parameter_type, amount, cutoff)?),
            body: Box::new(shift_term_structural(body, amount, cutoff.checked_add(1)?)?),
        }),
        Term::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(shift_term_structural(function, amount, cutoff)?),
            argument: Box::new(shift_term_structural(argument, amount, cutoff)?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Some(Term::Pair {
            sigma_type: Box::new(shift_term_structural(sigma_type, amount, cutoff)?),
            first: Box::new(shift_term_structural(first, amount, cutoff)?),
            second: Box::new(shift_term_structural(second, amount, cutoff)?),
        }),
        Term::First { pair } => Some(Term::First {
            pair: Box::new(shift_term_structural(pair, amount, cutoff)?),
        }),
        Term::Second { pair } => Some(Term::Second {
            pair: Box::new(shift_term_structural(pair, amount, cutoff)?),
        }),
    }
}

fn rename_candidate(
    node: &NodeCandidate,
    substitution: &VerifiedFiniteSubstitutionV1,
) -> VerifyResult<NodeCandidate> {
    if substitution.source_context != node.context {
        return Err(AuditUnknownReason::MalformedInput.into());
    }
    match &node.judgment {
        RewriteNodeJudgmentV1::HasType { ty } => {
            let term =
                instantiate_term_images(&node.term, node.context.0.len(), &substitution.images, 0)
                    .ok_or(AuditUnknownReason::MalformedInput)?;
            let ty = instantiate_term_images(ty, node.context.0.len(), &substitution.images, 0)
                .ok_or(AuditUnknownReason::MalformedInput)?;
            Ok(make_node_candidate(
                substitution.target_context.clone(),
                term,
                ty,
            ))
        }
        RewriteNodeJudgmentV1::TypeFormation => {
            let term =
                instantiate_term_images(&node.term, node.context.0.len(), &substitution.images, 0)
                    .ok_or(AuditUnknownReason::MalformedInput)?;
            Ok(make_formation_candidate(
                substitution.target_context.clone(),
                term,
            ))
        }
    }
}

fn kernel_check_node(
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    node: &NodeCandidate,
) -> VerifyResult<()> {
    match &node.judgment {
        RewriteNodeJudgmentV1::HasType { ty } => {
            kernel_check_term(kernel, inventory, &node.context, &node.term, ty)
        }
        RewriteNodeJudgmentV1::TypeFormation => kernel
            .verify_open_judgment(
                inventory.successor_boundary(),
                &OpenJudgment::TypeFormation {
                    context: node.context.clone(),
                    term: node.term.clone(),
                },
            )
            .map(|_| ())
            .map_err(kernel_failure),
    }
}

fn kernel_check_term(
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    context: &DependentContext,
    term: &Term,
    ty: &Term,
) -> VerifyResult<()> {
    kernel_check_term_against_signature(kernel, inventory.successor_boundary(), context, term, ty)
}

fn kernel_check_term_against_signature(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    context: &DependentContext,
    term: &Term,
    ty: &Term,
) -> VerifyResult<()> {
    kernel
        .verify_open_judgment(
            signature,
            &OpenJudgment::HasType {
                context: context.clone(),
                term: term.clone(),
                ty: ty.clone(),
            },
        )
        .map(|_| ())
        .map_err(kernel_failure)
}

fn kernel_check_edge(
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    source: &NodeCandidate,
    target: &NodeCandidate,
) -> VerifyResult<()> {
    kernel_check_edge_against_signature(kernel, inventory.successor_boundary(), source, target)
}

fn kernel_check_edge_against_signature(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    source: &NodeCandidate,
    target: &NodeCandidate,
) -> VerifyResult<()> {
    if source.context != target.context || source.judgment != target.judgment {
        return Err(AuditUnknownReason::MalformedInput.into());
    }
    let (source_judgment, target_judgment) = match &source.judgment {
        RewriteNodeJudgmentV1::HasType { ty } => (
            OpenJudgment::HasType {
                context: source.context.clone(),
                term: source.term.clone(),
                ty: ty.clone(),
            },
            OpenJudgment::HasType {
                context: source.context.clone(),
                term: target.term.clone(),
                ty: ty.clone(),
            },
        ),
        RewriteNodeJudgmentV1::TypeFormation => (
            OpenJudgment::TypeFormation {
                context: source.context.clone(),
                term: source.term.clone(),
            },
            OpenJudgment::TypeFormation {
                context: source.context.clone(),
                term: target.term.clone(),
            },
        ),
    };
    kernel
        .verify_open_judgments(signature, &[&source_judgment, &target_judgment])
        .map(|_| ())
        .map_err(kernel_failure)
}

fn kernel_failure(error: KernelError) -> VerifyFailure {
    match error {
        KernelError::ResourceExhausted(_) => {
            VerifyFailure::Unknown(AuditUnknownReason::ResourceExhausted)
        }
        _ => VerifyFailure::Unknown(AuditUnknownReason::KernelCouldNotCertify),
    }
}

fn enumerate_positions(term: &Term, depth_limit: u16) -> VerifyResult<Vec<PositionedTerm>> {
    let mut positions = Vec::new();
    let mut path = Vec::new();
    let mut operations_left = u32::MAX;
    enumerate_positions_inner(
        term,
        0,
        0,
        depth_limit,
        &mut operations_left,
        &mut path,
        &mut positions,
    )?;
    Ok(positions)
}

fn enumerate_positions_inner(
    term: &Term,
    binder_depth: u32,
    depth: u16,
    depth_limit: u16,
    operations_left: &mut u32,
    path: &mut Vec<u16>,
    positions: &mut Vec<PositionedTerm>,
) -> VerifyResult<()> {
    if depth > depth_limit || *operations_left == 0 {
        return Err(AuditUnknownReason::ResourceExhausted.into());
    }
    *operations_left -= 1;
    positions.push(PositionedTerm {
        position: TermPositionV1(path.clone()),
        binder_depth,
        focus: term.clone(),
    });
    let child_depth = depth
        .checked_add(1)
        .ok_or(AuditUnknownReason::ResourceExhausted)?;
    match term {
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            path.push(0);
            enumerate_positions_inner(
                parameter,
                binder_depth,
                child_depth,
                depth_limit,
                operations_left,
                path,
                positions,
            )?;
            path.pop();
            path.push(1);
            enumerate_positions_inner(
                body,
                binder_depth
                    .checked_add(1)
                    .ok_or(AuditUnknownReason::ResourceExhausted)?,
                child_depth,
                depth_limit,
                operations_left,
                path,
                positions,
            )?;
            path.pop();
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            path.push(0);
            enumerate_positions_inner(
                parameter_type,
                binder_depth,
                child_depth,
                depth_limit,
                operations_left,
                path,
                positions,
            )?;
            path.pop();
            path.push(1);
            enumerate_positions_inner(
                body,
                binder_depth
                    .checked_add(1)
                    .ok_or(AuditUnknownReason::ResourceExhausted)?,
                child_depth,
                depth_limit,
                operations_left,
                path,
                positions,
            )?;
            path.pop();
        }
        Term::Apply { function, argument } => {
            path.push(0);
            enumerate_positions_inner(
                function,
                binder_depth,
                child_depth,
                depth_limit,
                operations_left,
                path,
                positions,
            )?;
            path.pop();
            path.push(1);
            enumerate_positions_inner(
                argument,
                binder_depth,
                child_depth,
                depth_limit,
                operations_left,
                path,
                positions,
            )?;
            path.pop();
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            for (ordinal, child) in [(0, sigma_type), (1, first), (2, second)] {
                path.push(ordinal);
                enumerate_positions_inner(
                    child,
                    binder_depth,
                    child_depth,
                    depth_limit,
                    operations_left,
                    path,
                    positions,
                )?;
                path.pop();
            }
        }
        Term::First { pair } | Term::Second { pair } => {
            path.push(0);
            enumerate_positions_inner(
                pair,
                binder_depth,
                child_depth,
                depth_limit,
                operations_left,
                path,
                positions,
            )?;
            path.pop();
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => {}
    }
    Ok(())
}

enum RuleApplication {
    Applicable(Term),
    Inapplicable(RewriteInapplicableReasonV1),
}

fn apply_rule_at_focus(
    kernel: &Kernel,
    rule: &RuleInstance,
    node_context: &DependentContext,
    positioned: &PositionedTerm,
) -> VerifyResult<RuleApplication> {
    match &rule.rule {
        FiniteRewriteRuleV1::CanonicalDeBruijn => Ok(RuleApplication::Inapplicable(
            RewriteInapplicableReasonV1::DeBruijnAlreadyCanonical,
        )),
        FiniteRewriteRuleV1::SequentialSubstitution => Ok(RuleApplication::Inapplicable(
            RewriteInapplicableReasonV1::NoExplicitSubstitutionSyntax,
        )),
        FiniteRewriteRuleV1::OrdinaryBeta => match &positioned.focus {
            Term::Apply { function, argument } => match function.as_ref() {
                Term::Lambda { body, .. } => {
                    let mut budget = TermReplayBudget::new(kernel);
                    Ok(RuleApplication::Applicable(subst_top(
                        body,
                        argument,
                        &mut budget,
                        0,
                    )?))
                }
                _ => Ok(RuleApplication::Inapplicable(
                    RewriteInapplicableReasonV1::FocusShapeMismatch,
                )),
            },
            _ => Ok(RuleApplication::Inapplicable(
                RewriteInapplicableReasonV1::FocusShapeMismatch,
            )),
        },
        FiniteRewriteRuleV1::PublicDelta { declaration } => match &positioned.focus {
            Term::Global { id } if id == declaration => {
                Ok(RuleApplication::Applicable(rule.right.clone()))
            }
            _ => Ok(RuleApplication::Inapplicable(
                RewriteInapplicableReasonV1::GlobalHeadMismatch,
            )),
        },
        FiniteRewriteRuleV1::NoPublicDeltaDeclarations => Ok(RuleApplication::Inapplicable(
            RewriteInapplicableReasonV1::NoPublicDeltaDeclaration,
        )),
        FiniteRewriteRuleV1::CanonicalUnit => Ok(RuleApplication::Inapplicable(
            RewriteInapplicableReasonV1::UnitAlreadyCanonical,
        )),
        FiniteRewriteRuleV1::FlattenedTelescope => Ok(RuleApplication::Inapplicable(
            RewriteInapplicableReasonV1::TelescopeAlreadyFlattened,
        )),
        FiniteRewriteRuleV1::TypedFreshEquation { .. } => {
            if rule.target_context.as_ref() != Some(node_context) {
                return Ok(RuleApplication::Inapplicable(
                    RewriteInapplicableReasonV1::ContextMismatch,
                ));
            }
            let mut budget = TermReplayBudget::new(kernel);
            let lifted_left = shift_term(
                rule.left
                    .as_ref()
                    .ok_or(AuditUnknownReason::MalformedInput)?,
                i64::from(positioned.binder_depth),
                0,
                &mut budget,
                0,
            )?;
            if positioned.focus != lifted_left {
                return Ok(RuleApplication::Inapplicable(
                    RewriteInapplicableReasonV1::FreshPatternMismatch,
                ));
            }
            let mut budget = TermReplayBudget::new(kernel);
            let lifted_right = shift_term(
                &rule.right,
                i64::from(positioned.binder_depth),
                0,
                &mut budget,
                0,
            )?;
            Ok(RuleApplication::Applicable(lifted_right))
        }
    }
}

fn replace_at_position(term: &Term, path: &[u16], replacement: &Term) -> Option<Term> {
    let Some((ordinal, rest)) = path.split_first() else {
        return Some(replacement.clone());
    };
    match (term, *ordinal) {
        (Term::Pi { parameter, body }, 0) => Some(Term::Pi {
            parameter: Box::new(replace_at_position(parameter, rest, replacement)?),
            body: body.clone(),
        }),
        (Term::Pi { parameter, body }, 1) => Some(Term::Pi {
            parameter: parameter.clone(),
            body: Box::new(replace_at_position(body, rest, replacement)?),
        }),
        (Term::Sigma { parameter, body }, 0) => Some(Term::Sigma {
            parameter: Box::new(replace_at_position(parameter, rest, replacement)?),
            body: body.clone(),
        }),
        (Term::Sigma { parameter, body }, 1) => Some(Term::Sigma {
            parameter: parameter.clone(),
            body: Box::new(replace_at_position(body, rest, replacement)?),
        }),
        (
            Term::Lambda {
                parameter_type,
                body,
            },
            0,
        ) => Some(Term::Lambda {
            parameter_type: Box::new(replace_at_position(parameter_type, rest, replacement)?),
            body: body.clone(),
        }),
        (
            Term::Lambda {
                parameter_type,
                body,
            },
            1,
        ) => Some(Term::Lambda {
            parameter_type: parameter_type.clone(),
            body: Box::new(replace_at_position(body, rest, replacement)?),
        }),
        (Term::Apply { function, argument }, 0) => Some(Term::Apply {
            function: Box::new(replace_at_position(function, rest, replacement)?),
            argument: argument.clone(),
        }),
        (Term::Apply { function, argument }, 1) => Some(Term::Apply {
            function: function.clone(),
            argument: Box::new(replace_at_position(argument, rest, replacement)?),
        }),
        (
            Term::Pair {
                sigma_type,
                first,
                second,
            },
            0,
        ) => Some(Term::Pair {
            sigma_type: Box::new(replace_at_position(sigma_type, rest, replacement)?),
            first: first.clone(),
            second: second.clone(),
        }),
        (
            Term::Pair {
                sigma_type,
                first,
                second,
            },
            1,
        ) => Some(Term::Pair {
            sigma_type: sigma_type.clone(),
            first: Box::new(replace_at_position(first, rest, replacement)?),
            second: second.clone(),
        }),
        (
            Term::Pair {
                sigma_type,
                first,
                second,
            },
            2,
        ) => Some(Term::Pair {
            sigma_type: sigma_type.clone(),
            first: first.clone(),
            second: Box::new(replace_at_position(second, rest, replacement)?),
        }),
        (Term::First { pair }, 0) => Some(Term::First {
            pair: Box::new(replace_at_position(pair, rest, replacement)?),
        }),
        (Term::Second { pair }, 0) => Some(Term::Second {
            pair: Box::new(replace_at_position(pair, rest, replacement)?),
        }),
        _ => None,
    }
}

struct TermReplayBudget {
    operations_left: u32,
    depth_limit: u16,
}

impl TermReplayBudget {
    fn new(kernel: &Kernel) -> Self {
        let limits = kernel.limits();
        Self {
            operations_left: limits.max_operations,
            depth_limit: limits.max_depth,
        }
    }

    fn enter(&mut self, depth: u16) -> VerifyResult<()> {
        if depth > self.depth_limit || self.operations_left == 0 {
            return Err(AuditUnknownReason::ResourceExhausted.into());
        }
        self.operations_left -= 1;
        Ok(())
    }
}

fn subst_top(
    body: &Term,
    replacement: &Term,
    budget: &mut TermReplayBudget,
    depth: u16,
) -> VerifyResult<Term> {
    let lifted = shift_term(replacement, 1, 0, budget, depth)?;
    let replaced = substitute_term(body, 0, &lifted, 0, budget, depth)?;
    shift_term(&replaced, -1, 0, budget, depth)
}

fn substitute_term(
    term: &Term,
    target: u32,
    replacement: &Term,
    binder_depth: u32,
    budget: &mut TermReplayBudget,
    depth: u16,
) -> VerifyResult<Term> {
    budget.enter(depth)?;
    let child = depth
        .checked_add(1)
        .ok_or(AuditUnknownReason::ResourceExhausted)?;
    let sought = target
        .checked_add(binder_depth)
        .ok_or(AuditUnknownReason::MalformedInput)?;
    match term {
        Term::Var { index } if *index == sought => {
            shift_term(replacement, i64::from(binder_depth), 0, budget, child)
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(term.clone()),
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(substitute_term(
                parameter,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            body: Box::new(substitute_term(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or(AuditUnknownReason::MalformedInput)?,
                budget,
                child,
            )?),
        }),
        Term::Sigma { parameter, body } => Ok(Term::Sigma {
            parameter: Box::new(substitute_term(
                parameter,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            body: Box::new(substitute_term(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or(AuditUnknownReason::MalformedInput)?,
                budget,
                child,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(substitute_term(
                parameter_type,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            body: Box::new(substitute_term(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or(AuditUnknownReason::MalformedInput)?,
                budget,
                child,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(substitute_term(
                function,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            argument: Box::new(substitute_term(
                argument,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Ok(Term::Pair {
            sigma_type: Box::new(substitute_term(
                sigma_type,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            first: Box::new(substitute_term(
                first,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            second: Box::new(substitute_term(
                second,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
        }),
        Term::First { pair } => Ok(Term::First {
            pair: Box::new(substitute_term(
                pair,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
        }),
        Term::Second { pair } => Ok(Term::Second {
            pair: Box::new(substitute_term(
                pair,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
        }),
    }
}

fn shift_term(
    term: &Term,
    amount: i64,
    cutoff: u32,
    budget: &mut TermReplayBudget,
    depth: u16,
) -> VerifyResult<Term> {
    budget.enter(depth)?;
    let child = depth
        .checked_add(1)
        .ok_or(AuditUnknownReason::ResourceExhausted)?;
    match term {
        Term::Var { index } if *index >= cutoff => {
            let shifted = i64::from(*index)
                .checked_add(amount)
                .and_then(|value| u32::try_from(value).ok())
                .ok_or(AuditUnknownReason::MalformedInput)?;
            Ok(Term::Var { index: shifted })
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(term.clone()),
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(shift_term(parameter, amount, cutoff, budget, child)?),
            body: Box::new(shift_term(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(AuditUnknownReason::MalformedInput)?,
                budget,
                child,
            )?),
        }),
        Term::Sigma { parameter, body } => Ok(Term::Sigma {
            parameter: Box::new(shift_term(parameter, amount, cutoff, budget, child)?),
            body: Box::new(shift_term(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(AuditUnknownReason::MalformedInput)?,
                budget,
                child,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(shift_term(parameter_type, amount, cutoff, budget, child)?),
            body: Box::new(shift_term(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(AuditUnknownReason::MalformedInput)?,
                budget,
                child,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(shift_term(function, amount, cutoff, budget, child)?),
            argument: Box::new(shift_term(argument, amount, cutoff, budget, child)?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Ok(Term::Pair {
            sigma_type: Box::new(shift_term(sigma_type, amount, cutoff, budget, child)?),
            first: Box::new(shift_term(first, amount, cutoff, budget, child)?),
            second: Box::new(shift_term(second, amount, cutoff, budget, child)?),
        }),
        Term::First { pair } => Ok(Term::First {
            pair: Box::new(shift_term(pair, amount, cutoff, budget, child)?),
        }),
        Term::Second { pair } => Ok(Term::Second {
            pair: Box::new(shift_term(pair, amount, cutoff, budget, child)?),
        }),
    }
}

fn make_edge(
    source: RewriteNodeIdV1,
    target: RewriteNodeIdV1,
    position: TermPositionV1,
    rule: FiniteRewriteRuleV1,
) -> VerifiedRewriteEdgeV1 {
    struct Material<'a> {
        source: &'a RewriteNodeIdV1,
        target: &'a RewriteNodeIdV1,
        position: &'a TermPositionV1,
        rule: &'a FiniteRewriteRuleV1,
    }
    impl CanonicalEncode for Material<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.source.encode_canonical(encoder);
            self.target.encode_canonical(encoder);
            self.position.encode_canonical(encoder);
            self.rule.encode_canonical(encoder);
        }
    }
    let id = RewriteEdgeIdV1(Digest::of_canonical(
        "pen-semantic-audit/finite-rewrite-edge/v1",
        &Material {
            source: &source,
            target: &target,
            position: &position,
            rule: &rule,
        },
    ));
    VerifiedRewriteEdgeV1 {
        id,
        source,
        target,
        position,
        rule,
    }
}

fn verify_disposition_exhaustiveness(
    candidates: &[NodeCandidate],
    rules: &[RuleInstance],
    edges: &[VerifiedRewriteEdgeV1],
    dispositions: &[RewriteDispositionV1],
    kernel: &Kernel,
) -> VerifyResult<()> {
    let edge_ids = edges
        .iter()
        .map(|edge| edge.id.clone())
        .collect::<BTreeSet<_>>();
    let mut actual = BTreeSet::new();
    for disposition in dispositions {
        let key = (
            disposition.node.clone(),
            disposition.position.clone(),
            disposition.rule.clone(),
        );
        if !actual.insert(key) {
            return Err(AuditUnknownReason::IncompleteEnumeration.into());
        }
        if let RewriteDispositionOutcomeV1::Applicable { edge } = &disposition.outcome
            && !edge_ids.contains(edge)
        {
            return Err(AuditUnknownReason::IncompleteEnumeration.into());
        }
    }

    let expected_count = candidates.iter().try_fold(0_usize, |total, candidate| {
        let position_count = enumerate_positions(&candidate.term, kernel.limits().max_depth)?.len();
        total
            .checked_add(
                position_count
                    .checked_mul(rules.len())
                    .ok_or(AuditUnknownReason::ResourceExhausted)?,
            )
            .ok_or(VerifyFailure::Unknown(
                AuditUnknownReason::ResourceExhausted,
            ))
    })?;
    if actual.len() != expected_count {
        return Err(AuditUnknownReason::IncompleteEnumeration.into());
    }
    Ok(())
}

fn analyze_graph(
    node_ids: &BTreeSet<RewriteNodeIdV1>,
    edges: &[VerifiedRewriteEdgeV1],
) -> VerifyResult<GraphAnalysis> {
    let mut adjacency = node_ids
        .iter()
        .cloned()
        .map(|node| (node, BTreeSet::new()))
        .collect::<BTreeMap<_, _>>();
    for edge in edges {
        if !node_ids.contains(&edge.source) || !node_ids.contains(&edge.target) {
            return Err(AuditUnknownReason::IncompleteEnumeration.into());
        }
        adjacency
            .get_mut(&edge.source)
            .ok_or(AuditUnknownReason::IncompleteEnumeration)?
            .insert(edge.target.clone());
    }

    let mut indegree = node_ids
        .iter()
        .cloned()
        .map(|node| (node, 0_usize))
        .collect::<BTreeMap<_, _>>();
    for targets in adjacency.values() {
        for target in targets {
            let degree = indegree
                .get_mut(target)
                .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
            *degree = degree
                .checked_add(1)
                .ok_or(AuditUnknownReason::ResourceExhausted)?;
        }
    }
    let mut ready = indegree
        .iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(node, _)| node.clone())
        .collect::<BTreeSet<_>>();
    let mut topological = Vec::with_capacity(node_ids.len());
    while let Some(node) = ready.pop_first() {
        topological.push(node.clone());
        for target in adjacency
            .get(&node)
            .ok_or(AuditUnknownReason::IncompleteEnumeration)?
        {
            let degree = indegree
                .get_mut(target)
                .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
            *degree = degree
                .checked_sub(1)
                .ok_or(AuditUnknownReason::MalformedInput)?;
            if *degree == 0 {
                ready.insert(target.clone());
            }
        }
    }
    if topological.len() != node_ids.len() {
        return Err(AuditUnknownReason::MissingRewriteAdmissibilityTheorem.into());
    }

    let mut ranks = BTreeMap::<RewriteNodeIdV1, u32>::new();
    let mut normal_forms = BTreeMap::<RewriteNodeIdV1, RewriteNodeIdV1>::new();
    for node in topological.iter().rev() {
        let targets = adjacency
            .get(node)
            .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
        if targets.is_empty() {
            ranks.insert(node.clone(), 0);
            normal_forms.insert(node.clone(), node.clone());
            continue;
        }
        let mut rank = 0_u32;
        let mut reachable_normal_forms = BTreeSet::new();
        for target in targets {
            let target_rank: u32 = ranks
                .get(target)
                .copied()
                .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
            rank = rank.max(
                target_rank
                    .checked_add(1)
                    .ok_or(AuditUnknownReason::ResourceExhausted)?,
            );
            reachable_normal_forms.insert(
                normal_forms
                    .get(target)
                    .cloned()
                    .ok_or(AuditUnknownReason::IncompleteEnumeration)?,
            );
        }
        let reachable_normal_forms = reachable_normal_forms.into_iter().collect::<Vec<_>>();
        let [normal_form] = reachable_normal_forms.as_slice() else {
            return Err(AuditUnknownReason::UnknownQuotient.into());
        };
        ranks.insert(node.clone(), rank);
        normal_forms.insert(node.clone(), normal_form.clone());
    }
    Ok(GraphAnalysis {
        ranks,
        normal_forms,
        adjacency,
    })
}

fn verify_overlaps(
    edges: &[VerifiedRewriteEdgeV1],
    normal_forms: &BTreeMap<RewriteNodeIdV1, RewriteNodeIdV1>,
) -> VerifyResult<Vec<VerifiedOverlapDiagnosticV1>> {
    let mut groups =
        BTreeMap::<(RewriteNodeIdV1, TermPositionV1), Vec<&VerifiedRewriteEdgeV1>>::new();
    for edge in edges {
        groups
            .entry((edge.source.clone(), edge.position.clone()))
            .or_default()
            .push(edge);
    }
    let mut diagnostics = Vec::new();
    for ((node, position), group) in groups {
        let distinct_rules = group
            .iter()
            .map(|edge| edge.rule.clone())
            .collect::<BTreeSet<_>>();
        if distinct_rules.len() < 2 {
            continue;
        }
        let targets = group
            .iter()
            .map(|edge| edge.target.clone())
            .collect::<BTreeSet<_>>();
        let reachable = targets
            .iter()
            .map(|target| {
                normal_forms
                    .get(target)
                    .cloned()
                    .ok_or(AuditUnknownReason::IncompleteEnumeration)
            })
            .collect::<Result<BTreeSet<_>, _>>()?;
        let reachable = reachable.into_iter().collect::<Vec<_>>();
        let [common_normal_form] = reachable.as_slice() else {
            return Err(AuditUnknownReason::UnknownQuotient.into());
        };
        diagnostics.push(VerifiedOverlapDiagnosticV1 {
            node,
            position,
            rules: distinct_rules.into_iter().collect(),
            targets: targets.into_iter().collect(),
            common_normal_form: common_normal_form.clone(),
        });
    }
    diagnostics
        .sort_by(|left, right| (&left.node, &left.position).cmp(&(&right.node, &right.position)));
    Ok(diagnostics)
}

fn verify_substitution_reachability(
    candidates: &[NodeCandidate],
    candidate_indices: &BTreeMap<RewriteNodeIdV1, usize>,
    edges: &[VerifiedRewriteEdgeV1],
    adjacency: &BTreeMap<RewriteNodeIdV1, BTreeSet<RewriteNodeIdV1>>,
    substitutions: &[VerifiedFiniteSubstitutionV1],
    limit: usize,
) -> VerifyResult<(Vec<VerifiedSubstitutionReachabilityV1>, Digest)> {
    let mut obligations = Vec::new();
    for edge in edges {
        let source = candidates
            .get(
                *candidate_indices
                    .get(&edge.source)
                    .ok_or(AuditUnknownReason::IncompleteEnumeration)?,
            )
            .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
        let target = candidates
            .get(
                *candidate_indices
                    .get(&edge.target)
                    .ok_or(AuditUnknownReason::IncompleteEnumeration)?,
            )
            .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
        for substitution in substitutions
            .iter()
            .filter(|substitution| substitution.source_context == source.context)
        {
            if obligations.len() >= limit {
                return Err(AuditUnknownReason::ResourceExhausted.into());
            }
            let embedded_source = rename_candidate(source, substitution)?;
            let embedded_target = rename_candidate(target, substitution)?;
            if !candidate_indices.contains_key(&embedded_source.id)
                || !candidate_indices.contains_key(&embedded_target.id)
            {
                return Err(AuditUnknownReason::IncompleteSupport.into());
            }
            let path = find_reachability_path(&embedded_source.id, &embedded_target.id, adjacency)
                .ok_or(AuditUnknownReason::IncompleteSupport)?;
            obligations.push(VerifiedSubstitutionReachabilityV1 {
                edge: edge.id.clone(),
                substitution: substitution.id.clone(),
                source: embedded_source.id,
                target: embedded_target.id,
                path,
            });
        }
    }
    obligations.sort_by(|left, right| {
        (&left.edge, &left.substitution).cmp(&(&right.edge, &right.substitution))
    });
    struct Material<'a>(&'a [VerifiedSubstitutionReachabilityV1]);
    impl CanonicalEncode for Material<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            encoder.sequence(self.0);
        }
    }
    let digest = Digest::of_canonical(
        "pen-semantic-audit/finite-substitution-reachability/v1",
        &Material(&obligations),
    );
    Ok((obligations, digest))
}

fn find_reachability_path(
    source: &RewriteNodeIdV1,
    target: &RewriteNodeIdV1,
    adjacency: &BTreeMap<RewriteNodeIdV1, BTreeSet<RewriteNodeIdV1>>,
) -> Option<Vec<RewriteNodeIdV1>> {
    if source == target {
        return Some(vec![source.clone()]);
    }
    let mut seen = BTreeSet::from([source.clone()]);
    let mut predecessor = BTreeMap::<RewriteNodeIdV1, RewriteNodeIdV1>::new();
    let mut queue = VecDeque::from([source.clone()]);
    while let Some(node) = queue.pop_front() {
        let Some(targets) = adjacency.get(&node) else {
            continue;
        };
        for next in targets {
            if seen.insert(next.clone()) {
                predecessor.insert(next.clone(), node.clone());
                if next == target {
                    let mut path = vec![target.clone()];
                    let mut cursor = target;
                    while cursor != source {
                        cursor = predecessor.get(cursor)?;
                        path.push(cursor.clone());
                    }
                    path.reverse();
                    return Some(path);
                }
                queue.push_back(next.clone());
            }
        }
    }
    None
}

fn verify_predecessor_conservativity(
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    candidates: &[NodeCandidate],
    edges: &[VerifiedRewriteEdgeV1],
    dispositions: &[RewriteDispositionV1],
    normal_forms: &BTreeMap<RewriteNodeIdV1, RewriteNodeIdV1>,
    predecessor_globals: &BTreeSet<GlobalId>,
) -> VerifyResult<Digest> {
    let predecessor_candidates = candidates
        .iter()
        .filter(|node| node_is_predecessor(node, predecessor_globals))
        .map(|node| (node.id.clone(), node))
        .collect::<BTreeMap<_, _>>();
    let predecessor_nodes = predecessor_candidates
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut restricted_edges = BTreeSet::new();
    for edge in edges {
        let source_is_predecessor = predecessor_nodes.contains(&edge.source);
        if !source_is_predecessor {
            continue;
        }
        let target_is_predecessor = predecessor_nodes.contains(&edge.target);
        let rule_is_predecessor = match &edge.rule {
            FiniteRewriteRuleV1::OrdinaryBeta => true,
            FiniteRewriteRuleV1::PublicDelta { declaration } => {
                predecessor_globals.contains(declaration)
            }
            FiniteRewriteRuleV1::CanonicalDeBruijn
            | FiniteRewriteRuleV1::SequentialSubstitution
            | FiniteRewriteRuleV1::NoPublicDeltaDeclarations
            | FiniteRewriteRuleV1::CanonicalUnit
            | FiniteRewriteRuleV1::FlattenedTelescope
            | FiniteRewriteRuleV1::TypedFreshEquation { .. } => false,
        };
        verify_predecessor_edge_flags(
            source_is_predecessor,
            target_is_predecessor,
            rule_is_predecessor,
        )?;
        restricted_edges.insert(edge.id.clone());
    }

    let mut predecessor_rules = vec![RuleInstance {
        rule: FiniteRewriteRuleV1::OrdinaryBeta,
        target_context: None,
        left: None,
        right: Term::Unit,
    }];
    for declaration in inventory.predecessor_boundary().declarations() {
        if let Some(body) = &declaration.body {
            predecessor_rules.push(RuleInstance {
                rule: FiniteRewriteRuleV1::PublicDelta {
                    declaration: declaration.id.clone(),
                },
                target_context: None,
                left: None,
                right: body.clone(),
            });
        }
    }
    let mut independently_replayed_edges = BTreeSet::new();
    let mut unit_disposition_digests = Vec::new();
    for (node_id, node) in &predecessor_candidates {
        for positioned in enumerate_positions(&node.term, kernel.limits().max_depth)? {
            for rule in &predecessor_rules {
                match apply_rule_at_focus(kernel, rule, &node.context, &positioned)? {
                    RuleApplication::Inapplicable(_) => {}
                    RuleApplication::Applicable(replacement) => {
                        let target_term = replace_at_position(
                            &node.term,
                            positioned.position.child_ordinals(),
                            &replacement,
                        )
                        .ok_or(AuditUnknownReason::MalformedInput)?;
                        let target = make_node_candidate_with_judgment(
                            node.context.clone(),
                            target_term,
                            node.judgment.clone(),
                        );
                        kernel_check_edge_against_signature(
                            kernel,
                            inventory.predecessor_boundary(),
                            node,
                            &target,
                        )?;
                        if !predecessor_nodes.contains(&target.id) {
                            return Err(AuditUnknownReason::IncompleteEnumeration.into());
                        }
                        independently_replayed_edges.insert(
                            make_edge(
                                node_id.clone(),
                                target.id,
                                positioned.position.clone(),
                                rule.rule.clone(),
                            )
                            .id,
                        );
                    }
                }
            }
            let matching_unit = dispositions
                .iter()
                .filter(|disposition| {
                    disposition.node == *node_id
                        && disposition.position == positioned.position
                        && disposition.rule == FiniteRewriteRuleV1::CanonicalUnit
                })
                .collect::<Vec<_>>();
            let [unit] = matching_unit.as_slice() else {
                return Err(AuditUnknownReason::IncompleteEnumeration.into());
            };
            if unit.outcome
                != (RewriteDispositionOutcomeV1::CertifiedInapplicable {
                    reason: RewriteInapplicableReasonV1::UnitAlreadyCanonical,
                })
            {
                return Err(AuditUnknownReason::IncompleteEnumeration.into());
            }
            unit_disposition_digests.push(Digest::of_canonical(
                "pen-semantic-audit/predecessor-unit-disposition/v1",
                *unit,
            ));
        }
    }
    if independently_replayed_edges != restricted_edges {
        return Err(AuditUnknownReason::IncompleteSupport.into());
    }

    for node in &predecessor_nodes {
        let normal_form = normal_forms
            .get(node)
            .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
        if !predecessor_nodes.contains(normal_form) {
            return Err(AuditUnknownReason::IncompleteSupport.into());
        }
    }
    unit_disposition_digests.sort();
    struct Material<'a> {
        nodes: &'a BTreeSet<RewriteNodeIdV1>,
        edges: &'a BTreeSet<RewriteEdgeIdV1>,
        unit_dispositions: &'a [Digest],
    }
    impl CanonicalEncode for Material<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            encoder.u64(self.nodes.len() as u64);
            for node in self.nodes {
                node.encode_canonical(encoder);
            }
            encoder.u64(self.edges.len() as u64);
            for edge in self.edges {
                edge.encode_canonical(encoder);
            }
            encoder.sequence(self.unit_dispositions);
        }
    }
    Ok(Digest::of_canonical(
        "pen-semantic-audit/finite-predecessor-conservative-graph/v1",
        &Material {
            nodes: &predecessor_nodes,
            edges: &independently_replayed_edges,
            unit_dispositions: &unit_disposition_digests,
        },
    ))
}

fn verify_predecessor_edge_flags(
    source_is_predecessor: bool,
    target_is_predecessor: bool,
    rule_is_predecessor: bool,
) -> VerifyResult<()> {
    if source_is_predecessor && (!target_is_predecessor || !rule_is_predecessor) {
        return Err(AuditUnknownReason::IncompleteSupport.into());
    }
    Ok(())
}

fn node_is_predecessor(node: &NodeCandidate, predecessor_globals: &BTreeSet<GlobalId>) -> bool {
    node.context
        .0
        .iter()
        .all(|term| term_globals_are_subset(term, predecessor_globals))
        && term_globals_are_subset(&node.term, predecessor_globals)
        && match &node.judgment {
            RewriteNodeJudgmentV1::HasType { ty } => {
                term_globals_are_subset(ty, predecessor_globals)
            }
            RewriteNodeJudgmentV1::TypeFormation => true,
        }
}

fn term_globals_are_subset(term: &Term, allowed: &BTreeSet<GlobalId>) -> bool {
    match term {
        Term::Global { id } => allowed.contains(id),
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            term_globals_are_subset(parameter, allowed) && term_globals_are_subset(body, allowed)
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            term_globals_are_subset(parameter_type, allowed)
                && term_globals_are_subset(body, allowed)
        }
        Term::Apply { function, argument } => {
            term_globals_are_subset(function, allowed) && term_globals_are_subset(argument, allowed)
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            term_globals_are_subset(sigma_type, allowed)
                && term_globals_are_subset(first, allowed)
                && term_globals_are_subset(second, allowed)
        }
        Term::First { pair } | Term::Second { pair } => term_globals_are_subset(pair, allowed),
        Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => true,
    }
}

fn encode_u32_slice(encoder: &mut CanonicalEncoder, values: &[u32]) {
    encoder.u64(values.len() as u64);
    for value in values {
        encoder.u32(*value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::carrier::enumerate_pre_q0_raw_families_v1;
    use crate::inventory::{
        DemandContractIdV1, DemandFamilyIdV1, DemandPortKeyV1, ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
        PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION, PublicDependencyUseV1, PublicSubjectV1,
        UncheckedOriginCutoffQ3RegistryV1, UncheckedPredecessorDemandContractV1,
        UncheckedPublicAuditInventoryV1, UncheckedPublicAvailabilityClaimV1,
        UncheckedPublicDeclarationV1, UncheckedPublicDependencyDagV1, UncheckedPublicEquationV1,
        UncheckedPublicEventCensusV1, UncheckedPublicGroupV1, UncheckedPublicHistoryStepV1,
        UncheckedSourceNormalizedDeclarationV1, verify_public_audit_inventory_v1,
    };
    use crate::manifest::{
        proposed_semantic_audit_lambda_unit_manifest_v1,
        verify_semantic_audit_lambda_unit_manifest_v1,
    };
    use crate::model::{
        DemandOutputIdV1, EventIdV1, HeadPresentationV1, LocalRoleV1, PublicAvailabilityV1,
        PublicEquationSeedV1, PublicHeadSeedV1, PublicSupportV1, SemanticSchemaSeedV1,
        SourceNormalizedJudgmentV1,
    };
    use crate::normalizer::{
        FreshConstructorClauseV1, FreshConstructorComputationRequestV1,
        verify_fresh_constructor_computation_v1,
    };
    use crate::rewrite_inventory::compile_typed_rewrite_inventory_lambda_unit_v1;
    use pen_kernel::{Declaration, KernelLimits, UncheckedSignature};

    fn node(label: &str) -> RewriteNodeIdV1 {
        RewriteNodeIdV1(Digest::of_domain_bytes(
            "pen-semantic-audit/finite-rewrite-test-node/v1",
            label.as_bytes(),
        ))
    }

    fn edge(
        label: &str,
        source: RewriteNodeIdV1,
        target: RewriteNodeIdV1,
    ) -> VerifiedRewriteEdgeV1 {
        VerifiedRewriteEdgeV1 {
            id: RewriteEdgeIdV1(Digest::of_domain_bytes(
                "pen-semantic-audit/finite-rewrite-test-edge/v1",
                label.as_bytes(),
            )),
            source,
            target,
            position: TermPositionV1::root(),
            rule: FiniteRewriteRuleV1::OrdinaryBeta,
        }
    }

    fn fixture_source_declaration(
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

    fn fixture_source_judgment(judgment: GenericJudgmentV1) -> SourceNormalizedJudgmentV1 {
        SourceNormalizedJudgmentV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-judgment/v1",
                &judgment,
            ),
            source: judgment.clone(),
            claimed_normalized: judgment,
        }
    }

    #[test]
    fn end_to_end_lambda_unit_fixture_reaches_the_deliberate_rewrite_theorem_stop() {
        let AuditDecision::Proven(manifest) = verify_semantic_audit_lambda_unit_manifest_v1(
            &proposed_semantic_audit_lambda_unit_manifest_v1(),
        ) else {
            panic!("lambda/unit manifest");
        };
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let constructor = GlobalId(Digest::of_bytes(b"finite-rewrite-constructor"));
        let fresh = GlobalId(Digest::of_bytes(b"finite-rewrite-fresh"));
        let predecessor_event = EventIdV1(Digest::of_bytes(b"finite-rewrite-predecessor"));
        let successor_event = EventIdV1(Digest::of_bytes(b"finite-rewrite-successor"));
        let predecessor_group = GlobalId(Digest::of_bytes(b"finite-rewrite-pred-group"));
        let successor_group = GlobalId(Digest::of_bytes(b"finite-rewrite-succ-group"));
        let equation = EquationIdV1(Digest::of_bytes(b"finite-rewrite-equation"));
        let contract = DemandContractIdV1(Digest::of_bytes(b"finite-rewrite-contract"));
        let port = DemandPortKeyV1 {
            family: DemandFamilyIdV1(Digest::of_bytes(b"finite-rewrite-demand-family")),
            output: DemandOutputIdV1(Digest::of_bytes(b"finite-rewrite-demand-output")),
        };
        let constructor_declaration = Declaration {
            id: constructor.clone(),
            ty: Term::UnitType,
            body: None,
        };
        let fresh_declaration = Declaration {
            id: fresh.clone(),
            ty: Term::Pi {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::Pi {
                    parameter: Box::new(Term::UnitType),
                    body: Box::new(Term::UnitType),
                }),
            },
            body: None,
        };
        let predecessor_wire = UncheckedSignature {
            declarations: vec![constructor_declaration.clone()],
        };
        let predecessor = kernel
            .verify_signature(&predecessor_wire)
            .expect("predecessor signature");
        let AuditDecision::Proven(program) = verify_fresh_constructor_computation_v1(
            &manifest,
            &kernel,
            &predecessor,
            &FreshConstructorComputationRequestV1 {
                fresh_declaration: fresh_declaration.clone(),
                clauses: vec![FreshConstructorClauseV1 {
                    constructor: constructor.clone(),
                    scrutinee_parameter_ordinal: 1,
                }],
            },
        ) else {
            panic!("fresh program");
        };
        let successor_wire = UncheckedSignature {
            declarations: vec![constructor_declaration.clone(), fresh_declaration.clone()],
        };
        let fresh_equation = GenericJudgmentV1::Equation {
            context: DependentContext(vec![Term::UnitType]),
            left: Term::Apply {
                function: Box::new(Term::Apply {
                    function: Box::new(Term::Global { id: fresh.clone() }),
                    argument: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::Global {
                    id: constructor.clone(),
                }),
            },
            right: Term::Var { index: 0 },
            ty: Term::UnitType,
        };
        let dependency_constructor = PublicDependencyUseV1 {
            dependent: PublicSubjectV1::Equation {
                equation: equation.clone(),
            },
            prerequisite: constructor.clone(),
        };
        let dependency_fresh = PublicDependencyUseV1 {
            dependent: PublicSubjectV1::Equation {
                equation: equation.clone(),
            },
            prerequisite: fresh.clone(),
        };
        let equation_source_to_normal = fixture_source_judgment(fresh_equation.clone());
        let constructor_source_to_normal = fixture_source_declaration(&constructor_declaration);
        let fresh_source_to_normal = fixture_source_declaration(&fresh_declaration);
        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history: vec![UncheckedPublicHistoryStepV1 {
                census: UncheckedPublicEventCensusV1 {
                    event: predecessor_event.clone(),
                    added_groups: vec![predecessor_group.clone()],
                    added_declarations: vec![constructor.clone()],
                    added_equations: Vec::new(),
                    added_demand_contracts: vec![contract.clone()],
                    added_forced_projections: Vec::new(),
                },
                successor_boundary: predecessor_wire.clone(),
            }],
            predecessor_boundary: predecessor_wire,
            successor_event: UncheckedPublicEventCensusV1 {
                event: successor_event.clone(),
                added_groups: vec![successor_group.clone()],
                added_declarations: vec![fresh.clone()],
                added_equations: vec![equation.clone()],
                added_demand_contracts: Vec::new(),
                added_forced_projections: Vec::new(),
            },
            successor_boundary: successor_wire,
            declaration_groups: vec![
                UncheckedPublicGroupV1 {
                    group: predecessor_group.clone(),
                    origin: predecessor_event.clone(),
                    declarations: vec![constructor.clone()],
                },
                UncheckedPublicGroupV1 {
                    group: successor_group.clone(),
                    origin: successor_event.clone(),
                    declarations: vec![fresh.clone()],
                },
            ],
            declarations: vec![
                UncheckedPublicDeclarationV1 {
                    declaration: constructor.clone(),
                    origin: predecessor_event.clone(),
                    group: predecessor_group,
                    source_to_normal: constructor_source_to_normal.clone(),
                },
                UncheckedPublicDeclarationV1 {
                    declaration: fresh.clone(),
                    origin: successor_event.clone(),
                    group: successor_group,
                    source_to_normal: fresh_source_to_normal.clone(),
                },
            ],
            equations: vec![UncheckedPublicEquationV1 {
                equation,
                owner_head: fresh.clone(),
                origin: successor_event.clone(),
                source_to_normal: equation_source_to_normal.clone(),
                demand_port: Some(port.clone()),
            }],
            forced_projections: Vec::new(),
            predecessor_demand_contracts: vec![UncheckedPredecessorDemandContractV1 {
                contract,
                origin: predecessor_event.clone(),
                port,
                required_judgment: fixture_source_judgment(GenericJudgmentV1::Term {
                    context: DependentContext::default(),
                    term: Term::Unit,
                    ty: Term::UnitType,
                }),
            }],
            public_availability: vec![
                UncheckedPublicAvailabilityClaimV1 {
                    dependency: dependency_constructor.clone(),
                    claimed: PublicAvailabilityV1::PredecessorPublicExport {
                        target: constructor,
                    },
                },
                UncheckedPublicAvailabilityClaimV1 {
                    dependency: dependency_fresh.clone(),
                    claimed: PublicAvailabilityV1::DependencyPriorExport { target: fresh },
                },
            ],
            dependency_dag: UncheckedPublicDependencyDagV1 {
                edges: vec![dependency_constructor, dependency_fresh],
            },
            q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
                schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
                origin_cutoff: Some(predecessor_event.clone()),
                entries: Vec::new(),
            },
        };
        let AuditDecision::Proven(inventory) =
            verify_public_audit_inventory_v1(&manifest, &kernel, &wire)
        else {
            panic!("public inventory");
        };
        let AuditDecision::Proven(typed) =
            compile_typed_rewrite_inventory_lambda_unit_v1(&manifest, &inventory, &program)
        else {
            panic!("typed inventory");
        };
        let head_judgment =
            |declaration: &Declaration, source_identity: Digest| -> SourceNormalizedJudgmentV1 {
                let source = GenericJudgmentV1::Term {
                    context: DependentContext::default(),
                    term: Term::Global {
                        id: declaration.id.clone(),
                    },
                    ty: declaration.ty.clone(),
                };
                SourceNormalizedJudgmentV1 {
                    source_identity,
                    source: source.clone(),
                    claimed_normalized: source,
                }
            };
        let seeds = vec![
            SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
                declaration: constructor_declaration.id.clone(),
                origin_event: predecessor_event.clone(),
                judgment: head_judgment(
                    &constructor_declaration,
                    constructor_source_to_normal.source_identity,
                ),
                presentation: HeadPresentationV1::Opaque,
                claimed_role: LocalRoleV1::KernelHead,
                public_support: PublicSupportV1::default(),
                source_clause: None,
            }),
            SemanticSchemaSeedV1::PublicHead(PublicHeadSeedV1 {
                declaration: fresh_declaration.id.clone(),
                origin_event: successor_event.clone(),
                judgment: head_judgment(&fresh_declaration, fresh_source_to_normal.source_identity),
                presentation: HeadPresentationV1::Opaque,
                claimed_role: LocalRoleV1::KernelHead,
                public_support: PublicSupportV1::default(),
                source_clause: None,
            }),
            SemanticSchemaSeedV1::PublicEquation(PublicEquationSeedV1 {
                equation: EquationIdV1(Digest::of_bytes(b"finite-rewrite-equation")),
                owner_head: fresh_declaration.id.clone(),
                origin_event: successor_event,
                judgment: equation_source_to_normal,
                claimed_role: LocalRoleV1::Coherence,
                public_support: PublicSupportV1::default(),
                source_clause: None,
                demand_anchor: None,
            }),
        ];
        let carrier_decision = enumerate_pre_q0_raw_families_v1(
            &kernel,
            inventory.successor_boundary(),
            &manifest,
            &seeds,
            &[],
        );
        let AuditDecision::Proven(carrier) = carrier_decision else {
            panic!("pre-Q0 carrier: {carrier_decision:?}");
        };
        let rewrite_decision = verify_finite_rewrite_system_lambda_unit_v1(
            &kernel, &manifest, &carrier, &inventory, &typed,
        );
        assert!(matches!(
            rewrite_decision,
            AuditDecision::Unknown(AuditUnknownReason::MissingRewriteAdmissibilityTheorem)
        ));
    }

    #[test]
    fn adversarial_cycle_cannot_mint_termination_ranks() {
        let a = node("a");
        let b = node("b");
        let nodes = BTreeSet::from([a.clone(), b.clone()]);
        let edges = vec![edge("a-b", a.clone(), b.clone()), edge("b-a", b, a)];
        assert!(matches!(
            analyze_graph(&nodes, &edges),
            Err(VerifyFailure::Unknown(
                AuditUnknownReason::MissingRewriteAdmissibilityTheorem
            ))
        ));
    }

    #[test]
    fn adversarial_nonconfluence_cannot_choose_a_normal_form() {
        let source = node("source");
        let left = node("left-normal");
        let right = node("right-normal");
        let nodes = BTreeSet::from([source.clone(), left.clone(), right.clone()]);
        let edges = vec![
            edge("source-left", source.clone(), left),
            edge("source-right", source, right),
        ];
        assert!(matches!(
            analyze_graph(&nodes, &edges),
            Err(VerifyFailure::Unknown(AuditUnknownReason::UnknownQuotient))
        ));
    }

    #[test]
    fn adversarial_substitution_obligation_requires_graph_reachability() {
        let source = node("source");
        let middle = node("middle");
        let target = node("target");
        let disconnected = node("disconnected");
        let adjacency = BTreeMap::from([
            (source.clone(), BTreeSet::from([middle.clone()])),
            (middle, BTreeSet::from([target.clone()])),
            (target.clone(), BTreeSet::new()),
            (disconnected.clone(), BTreeSet::new()),
        ]);
        assert_eq!(
            find_reachability_path(&source, &target, &adjacency),
            Some(vec![source.clone(), node("middle"), target])
        );
        assert!(find_reachability_path(&source, &disconnected, &adjacency).is_none());
    }

    #[test]
    fn unrestricted_generator_composition_exhausts_a_finite_bound() {
        let one = DependentContext(vec![Term::UnitType]);
        let two = DependentContext(vec![Term::UnitType, Term::UnitType]);
        let embedding = make_embedding_substitution(
            FiniteSubstitutionKindV1::ContextEmbeddingLift,
            one.clone(),
            two.clone(),
            vec![1],
        );
        let function = GlobalId(Digest::of_bytes(b"unbounded-substitution-function"));
        let forced = make_general_substitution(
            FiniteSubstitutionKindV1::ForcedNewestArgument,
            two,
            one.clone(),
            vec![
                Term::Var { index: 0 },
                Term::Apply {
                    function: Box::new(Term::Global { id: function }),
                    argument: Box::new(Term::Var { index: 0 }),
                },
            ],
        );
        let generator = compose_substitutions(&embedding, &forced).expect("endomorphism");
        let mut substitutions = Vec::new();
        let mut indices = BTreeMap::new();
        insert_substitution(generator.clone(), &mut substitutions, &mut indices, 4)
            .expect("first power");

        let mut current = generator.clone();
        let exhausted = loop {
            let next = compose_substitutions(&current, &generator).expect("next power");
            match insert_substitution(next.clone(), &mut substitutions, &mut indices, 4) {
                Ok(_) => current = next,
                Err(failure) => break failure,
            }
        };
        assert!(matches!(
            exhausted,
            VerifyFailure::Unknown(AuditUnknownReason::ResourceExhausted)
        ));
    }

    #[test]
    fn certificate_minting_stays_disabled_at_the_known_theorem_boundary() {
        assert!(matches!(
            verify_complete_rewrite_certificate_boundary_v1(),
            Err(VerifyFailure::Unknown(
                AuditUnknownReason::MissingRewriteAdmissibilityTheorem
            ))
        ));
    }

    #[test]
    fn projections_use_the_projection_specific_outside_disposition() {
        let projection = Term::First {
            pair: Box::new(Term::Unit),
        };
        let sigma = Term::Sigma {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::UnitType),
        };
        assert!(matches!(
            validate_lambda_unit_term(&projection, &[0, 1]),
            Err(VerifyFailure::Outside(
                OutsideFragmentReason::DescriptorProjection
            ))
        ));
        assert!(matches!(
            validate_lambda_unit_term(&sigma, &[0, 1]),
            Err(VerifyFailure::Outside(
                OutsideFragmentReason::UnsupportedTerm
            ))
        ));
    }

    #[test]
    fn adversarial_successor_edge_breaks_predecessor_conservativity() {
        assert!(matches!(
            verify_predecessor_edge_flags(true, true, false),
            Err(VerifyFailure::Unknown(
                AuditUnknownReason::IncompleteSupport
            ))
        ));
        assert!(matches!(
            verify_predecessor_edge_flags(true, false, true),
            Err(VerifyFailure::Unknown(
                AuditUnknownReason::IncompleteSupport
            ))
        ));
        assert!(verify_predecessor_edge_flags(true, true, true).is_ok());
    }

    #[test]
    fn adversarial_embedding_enumeration_respects_the_manifest_bound() {
        // There are six monotone embeddings of a two-entry context into a
        // four-entry context, so a bound of five must fail closed.
        assert!(matches!(
            enumerate_monotone_embeddings(2, 4, 5),
            Err(VerifyFailure::Unknown(
                AuditUnknownReason::ResourceExhausted
            ))
        ));
    }

    #[test]
    fn beta_replay_is_capture_avoiding_under_a_nested_binder() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        // (lambda x. lambda y. x) Var(0)  ==>  lambda y. Var(1).
        let body = Term::Lambda {
            parameter_type: Box::new(Term::UnitType),
            body: Box::new(Term::Var { index: 1 }),
        };
        let mut budget = TermReplayBudget::new(&kernel);
        assert_eq!(
            subst_top(&body, &Term::Var { index: 0 }, &mut budget, 0).expect("substitution"),
            Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Var { index: 1 }),
            }
        );
    }
}
