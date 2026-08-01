//! The V3 rewrite authority (Phase J, items 1–4): typed Q0 stability,
//! complete edge-local reduction graphs, the all-pairs overlap census
//! with joins, termination/confluence, and predecessor conservativity —
//! over the exact native carrier.
//!
//! This module discharges the three certificate-boundary obligations the
//! V1 finite-rewrite prototype registered and deliberately failed closed
//! on, using exactly the Phase I capabilities that were built to supply
//! them:
//!
//! - *recursively typed subterms and binder-local contexts*: the node
//!   universe is seeded from the carrier-derived typed-occurrence census,
//!   whose occurrences carry kernel-replayed binder-local contexts for
//!   every subterm position of every carrier root;
//! - *finite substitution closure*: stability is stated against the
//!   carrier's direct construction-substitution census — the complete,
//!   derivation-local witness set — never against an open-ended generator
//!   composition closure; and
//! - *complete overlap census*: every unordered pair of applicable
//!   reductions at a node is classified (same-position, nested,
//!   disjoint) and joined at the node's unique normal form.
//!
//! Predecessor conservativity combines the positive empty-equation
//! historical base (`historical_rewrite`) with an independent
//! reconstruction: the subgraph of predecessor-supported nodes is rebuilt
//! from the predecessor boundary alone and must equal the main graph's
//! restriction exactly.
//!
//! The three semantic Q0 rules (beta, provenance-preserving public
//! delta, fresh non-recursive constructor computation) generate edges.
//! The representation rules of the V3 Q0 inventory (canonical de Bruijn,
//! sequential substitution elimination, telescope flattening) and the
//! canonical-unit rule act identically on the kernel term representation
//! — the kernel syntax cannot express a non-canonical form — and are
//! recorded as a certified identity census, not silently dropped.
//!
//! Fail-closed discipline: the only success is the opaque capability; a
//! broken binding, an unjoinable overlap, a cyclic reduction, an
//! unstable edge, or a non-conservative predecessor restriction each
//! stop at their own reason, and the reserved V3 stage reason
//! `MissingRewriteSystemV3` names every still-unavailable prerequisite.

use crate::historical_rewrite::verify_historical_rewrite_system_v1;
use crate::inventory::VerifiedPublicAuditInventoryV1;
use crate::inventory_compatibility::VerifiedPublicInventoryCompatibilityV2;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, VerifiedSemanticAuditManifestV1,
    VerifiedSemanticAuditManifestV2, VerifiedSemanticAuditManifestV3,
};
use crate::model::EquationIdV1;
use crate::native_carrier_v3::{
    VerifiedCarrierRootInventoryV3, VerifiedCarrierSubjectBundleV3,
    VerifiedNativeRankInductiveCarrierV3,
};
use crate::rewrite_inventory::VerifiedTypedRewriteInventoryV1;
use crate::typed_occurrence::{LocalJudgmentV1, TypedOccurrenceRootRequestV1};
use crate::typed_occurrence_v3::VerifiedSynthesisBackedTypedOccurrenceCensusV3;
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Kernel, KernelError,
    OpenJudgment, ResourceKind, Term, VerifiedSignature,
};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;

pub const REWRITE_AUTHORITY_SCHEMA_VERSION_V3: u16 = 1;

/// The three semantic Q0 reduction rules that generate edges.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RewriteRuleV3 {
    OrdinaryBeta,
    PublicDelta { declaration: GlobalId },
    FreshEquation { equation: EquationIdV1 },
}

impl CanonicalEncode for RewriteRuleV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::OrdinaryBeta => encoder.tag(0),
            Self::PublicDelta { declaration } => {
                encoder.tag(1);
                declaration.encode_canonical(encoder);
            }
            Self::FreshEquation { equation } => {
                encoder.tag(2);
                equation.encode_canonical(encoder);
            }
        }
    }
}

/// Exact subterm position: child ordinals from the root.
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct RewritePositionV3(Vec<u16>);

impl RewritePositionV3 {
    pub fn child_ordinals(&self) -> &[u16] {
        &self.0
    }

    fn child(&self, ordinal: u16) -> Self {
        let mut components = self.0.clone();
        components.push(ordinal);
        Self(components)
    }

    fn is_prefix_of(&self, other: &Self) -> bool {
        other.0.len() >= self.0.len() && other.0[..self.0.len()] == self.0[..]
    }
}

impl CanonicalEncode for RewritePositionV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u64(self.0.len() as u64);
        for ordinal in &self.0 {
            encoder.u16(*ordinal);
        }
    }
}

/// The typed judgment carried by one graph node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RewriteNodeJudgmentV3 {
    HasType { ty: Term },
    TypeFormation,
}

impl CanonicalEncode for RewriteNodeJudgmentV3 {
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

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RewriteNodeIdV3(Digest);

impl RewriteNodeIdV3 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for RewriteNodeIdV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// One kernel-checked typed node of the reduction graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RewriteNodeV3 {
    id: RewriteNodeIdV3,
    context: DependentContext,
    term: Term,
    judgment: RewriteNodeJudgmentV3,
}

impl RewriteNodeV3 {
    pub fn id(&self) -> &RewriteNodeIdV3 {
        &self.id
    }

    pub fn context(&self) -> &DependentContext {
        &self.context
    }

    pub fn term(&self) -> &Term {
        &self.term
    }

    pub fn judgment(&self) -> &RewriteNodeJudgmentV3 {
        &self.judgment
    }
}

impl CanonicalEncode for RewriteNodeV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.context.encode_canonical(encoder);
        self.term.encode_canonical(encoder);
        self.judgment.encode_canonical(encoder);
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RewriteEdgeIdV3(Digest);

impl RewriteEdgeIdV3 {
    pub fn digest(&self) -> &Digest {
        &self.0
    }
}

impl CanonicalEncode for RewriteEdgeIdV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.0.encode_canonical(encoder);
    }
}

/// One applicable reduction: source, target, position, rule.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RewriteEdgeV3 {
    id: RewriteEdgeIdV3,
    source: RewriteNodeIdV3,
    target: RewriteNodeIdV3,
    position: RewritePositionV3,
    rule: RewriteRuleV3,
}

impl RewriteEdgeV3 {
    pub fn id(&self) -> &RewriteEdgeIdV3 {
        &self.id
    }

    pub fn source(&self) -> &RewriteNodeIdV3 {
        &self.source
    }

    pub fn target(&self) -> &RewriteNodeIdV3 {
        &self.target
    }

    pub fn position(&self) -> &RewritePositionV3 {
        &self.position
    }

    pub fn rule(&self) -> &RewriteRuleV3 {
        &self.rule
    }
}

impl CanonicalEncode for RewriteEdgeV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.id.encode_canonical(encoder);
        self.source.encode_canonical(encoder);
        self.target.encode_canonical(encoder);
        self.position.encode_canonical(encoder);
        self.rule.encode_canonical(encoder);
    }
}

/// Relation between the positions of two applicable reductions at one
/// node.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OverlapRelationV3 {
    SamePosition,
    Nested,
    Disjoint,
}

impl CanonicalEncode for OverlapRelationV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.tag(match self {
            Self::SamePosition => 0,
            Self::Nested => 1,
            Self::Disjoint => 2,
        });
    }
}

/// One entry of the complete unordered-pair overlap census: two
/// applicable reductions at the same node, their position relation, and
/// the join witness (the node's unique normal form, reachable from both
/// targets).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlapPairV3 {
    node: RewriteNodeIdV3,
    left_edge: RewriteEdgeIdV3,
    right_edge: RewriteEdgeIdV3,
    relation: OverlapRelationV3,
    join_normal_form: RewriteNodeIdV3,
}

impl OverlapPairV3 {
    pub fn node(&self) -> &RewriteNodeIdV3 {
        &self.node
    }

    pub fn left_edge(&self) -> &RewriteEdgeIdV3 {
        &self.left_edge
    }

    pub fn right_edge(&self) -> &RewriteEdgeIdV3 {
        &self.right_edge
    }

    pub fn relation(&self) -> OverlapRelationV3 {
        self.relation
    }

    pub fn join_normal_form(&self) -> &RewriteNodeIdV3 {
        &self.join_normal_form
    }
}

impl CanonicalEncode for OverlapPairV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.node.encode_canonical(encoder);
        self.left_edge.encode_canonical(encoder);
        self.right_edge.encode_canonical(encoder);
        self.relation.encode_canonical(encoder);
        self.join_normal_form.encode_canonical(encoder);
    }
}

/// One substitution-stability witness: one-step commutation of one
/// rewrite edge with one carrier construction substitution. Applying
/// the witness's complete image vector to the edge's endpoints yields
/// kernel-checked judgments in the witness's target context, and
/// replaying the edge's rule at the edge's exact position on the
/// substituted source yields exactly the substituted target.
///
/// The witnesses are the carrier's derivation-local construction
/// substitutions; they are deliberately not composed or closed into a
/// global substitution carrier (the registered lesson that a finite
/// generator list is not closed under arbitrary composition).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubstitutionStabilityV3 {
    edge: RewriteEdgeIdV3,
    witness_digest: Digest,
    substituted_source_digest: Digest,
    substituted_target_digest: Digest,
}

impl SubstitutionStabilityV3 {
    pub fn edge(&self) -> &RewriteEdgeIdV3 {
        &self.edge
    }

    pub fn witness_digest(&self) -> &Digest {
        &self.witness_digest
    }

    pub fn substituted_source_digest(&self) -> &Digest {
        &self.substituted_source_digest
    }

    pub fn substituted_target_digest(&self) -> &Digest {
        &self.substituted_target_digest
    }
}

impl CanonicalEncode for SubstitutionStabilityV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.edge.encode_canonical(encoder);
        self.witness_digest.encode_canonical(encoder);
        self.substituted_source_digest.encode_canonical(encoder);
        self.substituted_target_digest.encode_canonical(encoder);
    }
}

/// Fail-closed failures of the rewrite-authority constructor.
#[derive(Clone, Debug)]
pub enum RewriteAuthorityFailureV3 {
    ExactManifestIdentityMismatch,
    ChainBindingMismatch,
    CarrierBindingMismatch,
    FreshInventoryBindingMismatch,
    MissingTypedRewriteInventory,
    HistoricalBase(AuditUnknownReason),
    NodeUniverse(AuditUnknownReason),
    OutsideFragment(OutsideFragmentReason),
    KernelReplay(AuditUnknownReason),
    NonTerminating,
    NotConfluent,
    UnjoinableOverlap,
    UnstableEdge,
    PredecessorNotConservative,
    ResourceExhausted { stage: &'static str },
}

impl std::fmt::Display for RewriteAuthorityFailureV3 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExactManifestIdentityMismatch => formatter.write_str(
                "the supplied manifests are not the exact registered lambda/unit proposals",
            ),
            Self::ChainBindingMismatch => formatter.write_str(
                "the inventory, compatibility, and census chain is not one verified chain",
            ),
            Self::CarrierBindingMismatch => formatter.write_str(
                "a Phase I capability is not bound to the presented carrier and manifest",
            ),
            Self::FreshInventoryBindingMismatch => formatter.write_str(
                "the typed rewrite inventory does not match the inventory equations exactly",
            ),
            Self::MissingTypedRewriteInventory => formatter.write_str(
                "the inventory has sealed equations but no typed rewrite inventory was presented",
            ),
            Self::HistoricalBase(reason) => {
                write!(
                    formatter,
                    "the historical rewrite base is unavailable: {reason:?}"
                )
            }
            Self::NodeUniverse(reason) => {
                write!(formatter, "node-universe construction failed: {reason:?}")
            }
            Self::OutsideFragment(reason) => {
                write!(formatter, "outside the lambda/unit fragment: {reason:?}")
            }
            Self::KernelReplay(reason) => {
                write!(formatter, "kernel replay failed: {reason:?}")
            }
            Self::NonTerminating => formatter
                .write_str("the finite reduction graph contains a cycle; termination fails"),
            Self::NotConfluent => {
                formatter.write_str("a node reaches two distinct normal forms; confluence fails")
            }
            Self::UnjoinableOverlap => formatter
                .write_str("two applicable reductions at one node do not join at its normal form"),
            Self::UnstableEdge => formatter
                .write_str("an edge is not stable under a carrier construction substitution"),
            Self::PredecessorNotConservative => formatter.write_str(
                "the predecessor restriction does not equal its independent reconstruction",
            ),
            Self::ResourceExhausted { stage } => {
                write!(
                    formatter,
                    "a declared resource bound was exceeded at {stage}"
                )
            }
        }
    }
}

impl std::error::Error for RewriteAuthorityFailureV3 {}

impl RewriteAuthorityFailureV3 {
    fn into_decision<T>(self) -> AuditDecision<T> {
        match self {
            Self::OutsideFragment(reason) => AuditDecision::OutsideFragment(reason),
            Self::ResourceExhausted { .. } => {
                AuditDecision::Unknown(AuditUnknownReason::ResourceExhausted)
            }
            Self::HistoricalBase(reason)
            | Self::NodeUniverse(reason)
            | Self::KernelReplay(reason) => AuditDecision::Unknown(reason),
            Self::ExactManifestIdentityMismatch
            | Self::ChainBindingMismatch
            | Self::CarrierBindingMismatch
            | Self::FreshInventoryBindingMismatch
            | Self::MissingTypedRewriteInventory => {
                AuditDecision::Unknown(AuditUnknownReason::MissingRewriteSystemV3)
            }
            Self::NonTerminating
            | Self::NotConfluent
            | Self::UnjoinableOverlap
            | Self::UnstableEdge
            | Self::PredecessorNotConservative => {
                AuditDecision::Unknown(AuditUnknownReason::MissingRewriteSystemV3)
            }
        }
    }
}

/// The Phase J rewrite authority: the complete typed rank-bounded
/// reduction system over the native carrier, with termination,
/// confluence, the all-pairs overlap census, substitution stability,
/// and predecessor conservativity. Fields are private; there is no
/// deserialization path; the only constructor is the verifier below.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedRewriteAuthorityV3 {
    schema_version: u16,
    semantic_manifest_digest: Digest,
    v1_manifest_digest: Digest,
    v2_manifest_digest: Digest,
    signature_digest: Digest,
    kernel_protocol_digest: Digest,
    carrier_digest: Digest,
    root_inventory_digest: Digest,
    census_digest: Digest,
    subject_bundle_digest: Digest,
    typed_rewrite_inventory_digest: Option<Digest>,
    historical_base_digest: Digest,
    nodes: Arc<[RewriteNodeV3]>,
    edges: Arc<[RewriteEdgeV3]>,
    applicable_disposition_count: u64,
    inapplicable_disposition_count: u64,
    representation_identity_census_digest: Digest,
    overlap_pairs: Arc<[OverlapPairV3]>,
    substitution_stability: Arc<[SubstitutionStabilityV3]>,
    normal_forms: Arc<[(RewriteNodeIdV3, RewriteNodeIdV3)]>,
    termination_rank_digest: Digest,
    predecessor_node_count: u64,
    predecessor_edge_count: u64,
    predecessor_reconstruction_digest: Digest,
    digest: Digest,
}

impl VerifiedRewriteAuthorityV3 {
    pub fn semantic_manifest_digest(&self) -> &Digest {
        &self.semantic_manifest_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn carrier_digest(&self) -> &Digest {
        &self.carrier_digest
    }

    pub fn kernel_protocol_digest(&self) -> &Digest {
        &self.kernel_protocol_digest
    }

    pub fn typed_rewrite_inventory_digest(&self) -> Option<&Digest> {
        self.typed_rewrite_inventory_digest.as_ref()
    }

    pub fn census_digest(&self) -> &Digest {
        &self.census_digest
    }

    pub fn nodes(&self) -> &[RewriteNodeV3] {
        &self.nodes
    }

    pub fn edges(&self) -> &[RewriteEdgeV3] {
        &self.edges
    }

    pub fn applicable_disposition_count(&self) -> u64 {
        self.applicable_disposition_count
    }

    pub fn inapplicable_disposition_count(&self) -> u64 {
        self.inapplicable_disposition_count
    }

    pub fn overlap_pairs(&self) -> &[OverlapPairV3] {
        &self.overlap_pairs
    }

    pub fn substitution_stability(&self) -> &[SubstitutionStabilityV3] {
        &self.substitution_stability
    }

    /// The unique normal form of every node: the confluence witness.
    pub fn normal_forms(&self) -> &[(RewriteNodeIdV3, RewriteNodeIdV3)] {
        &self.normal_forms
    }

    pub fn normal_form_of(&self, node: &RewriteNodeIdV3) -> Option<&RewriteNodeIdV3> {
        self.normal_forms
            .binary_search_by(|(candidate, _)| candidate.cmp(node))
            .ok()
            .map(|index| &self.normal_forms[index].1)
    }

    pub fn predecessor_node_count(&self) -> u64 {
        self.predecessor_node_count
    }

    pub fn predecessor_edge_count(&self) -> u64 {
        self.predecessor_edge_count
    }

    pub fn predecessor_reconstruction_digest(&self) -> &Digest {
        &self.predecessor_reconstruction_digest
    }

    pub fn historical_base_digest(&self) -> &Digest {
        &self.historical_base_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedRewriteAuthorityV3 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.schema_version);
        self.semantic_manifest_digest.encode_canonical(encoder);
        self.v1_manifest_digest.encode_canonical(encoder);
        self.v2_manifest_digest.encode_canonical(encoder);
        self.signature_digest.encode_canonical(encoder);
        self.kernel_protocol_digest.encode_canonical(encoder);
        self.carrier_digest.encode_canonical(encoder);
        self.root_inventory_digest.encode_canonical(encoder);
        self.census_digest.encode_canonical(encoder);
        self.subject_bundle_digest.encode_canonical(encoder);
        match &self.typed_rewrite_inventory_digest {
            Some(digest) => {
                encoder.tag(1);
                digest.encode_canonical(encoder);
            }
            None => encoder.tag(0),
        }
        self.historical_base_digest.encode_canonical(encoder);
        encoder.sequence(&self.nodes);
        encoder.sequence(&self.edges);
        encoder.u64(self.applicable_disposition_count);
        encoder.u64(self.inapplicable_disposition_count);
        self.representation_identity_census_digest
            .encode_canonical(encoder);
        encoder.sequence(&self.overlap_pairs);
        encoder.sequence(&self.substitution_stability);
        encoder.u64(self.normal_forms.len() as u64);
        for (node, normal_form) in self.normal_forms.iter() {
            node.encode_canonical(encoder);
            normal_form.encode_canonical(encoder);
        }
        self.termination_rank_digest.encode_canonical(encoder);
        encoder.u64(self.predecessor_node_count);
        encoder.u64(self.predecessor_edge_count);
        self.predecessor_reconstruction_digest
            .encode_canonical(encoder);
    }
}

struct FreshRuleV3 {
    equation: EquationIdV1,
    owner: GlobalId,
    constructor: GlobalId,
    arity: usize,
    right: Term,
}

struct BuildContext<'a> {
    kernel: &'a Kernel,
    signature: &'a VerifiedSignature,
    delta_bodies: BTreeMap<GlobalId, Term>,
    fresh_rules: Vec<FreshRuleV3>,
    node_limit: usize,
    disposition_limit: usize,
}

/// Exact independently replayed restriction of the successor graph to the
/// predecessor language.  This is private proof material: callers receive
/// only the enclosing rewrite authority, never a constructor from asserted
/// node or edge sets.
struct PredecessorReconstructionV3 {
    nodes: BTreeSet<RewriteNodeIdV3>,
    edges: BTreeSet<RewriteEdgeIdV3>,
    digest: Digest,
}

/// Construct the Phase J rewrite authority over one native carrier.
#[allow(clippy::too_many_arguments)]
pub fn diagnose_rewrite_authority_v3(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    compatibility: &VerifiedPublicInventoryCompatibilityV2,
    carrier: &VerifiedNativeRankInductiveCarrierV3,
    root_inventory: &VerifiedCarrierRootInventoryV3,
    census: &VerifiedSynthesisBackedTypedOccurrenceCensusV3,
    subject_bundle: &VerifiedCarrierSubjectBundleV3,
    typed_rewrite_inventory: Option<&VerifiedTypedRewriteInventoryV1>,
) -> Result<VerifiedRewriteAuthorityV3, RewriteAuthorityFailureV3> {
    use crate::manifest::{
        proposed_semantic_audit_lambda_unit_manifest_v1,
        proposed_semantic_audit_lambda_unit_manifest_v2,
        proposed_semantic_audit_lambda_unit_manifest_v3,
    };
    if v1_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v1()
        || v2_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v2()
        || v3_manifest.manifest() != &proposed_semantic_audit_lambda_unit_manifest_v3()
    {
        return Err(RewriteAuthorityFailureV3::ExactManifestIdentityMismatch);
    }
    if compatibility.v1_manifest_digest() != inventory.manifest_digest()
        || compatibility.inventory_digest() != inventory.digest()
        || compatibility.v2_manifest_digest() != v2_manifest.candidate_digest()
    {
        return Err(RewriteAuthorityFailureV3::ChainBindingMismatch);
    }
    // One manifest, signature, kernel, and carrier identity across every
    // Phase I capability.
    let signature = inventory.successor_boundary();
    let manifest_digest = v3_manifest.candidate_digest();
    if carrier.semantic_manifest_digest() != manifest_digest
        || carrier.inventory_digest() != inventory.digest()
        || carrier.signature_digest() != signature.digest()
        || carrier.kernel_protocol_digest() != &kernel.kernel_protocol_digest()
        || root_inventory.semantic_manifest_digest() != manifest_digest
        || root_inventory.carrier_digest() != carrier.digest()
        || census.semantic_manifest_digest() != manifest_digest
        || census.carrier_digest() != carrier.digest()
        || census.root_inventory_digest() != root_inventory.digest()
        || subject_bundle.semantic_manifest_digest() != manifest_digest
        || subject_bundle.carrier_digest() != carrier.digest()
        || census.subject_bundle_digest() != subject_bundle.digest()
    {
        return Err(RewriteAuthorityFailureV3::CarrierBindingMismatch);
    }

    // The positive empty-equation historical base for the predecessor.
    let historical =
        match verify_historical_rewrite_system_v1(v2_manifest, compatibility, inventory) {
            AuditDecision::Proven(historical) => historical,
            AuditDecision::Unknown(reason) => {
                return Err(RewriteAuthorityFailureV3::HistoricalBase(reason));
            }
            AuditDecision::OutsideFragment(reason) => {
                return Err(RewriteAuthorityFailureV3::OutsideFragment(reason));
            }
        };

    // Delta rules: all and only the bodyful normalized successor
    // declarations of the verified inventory.
    let mut delta_bodies = BTreeMap::new();
    for declaration in inventory.declarations() {
        let normalized = declaration.normalized();
        if let Some(body) = &normalized.body {
            if delta_bodies
                .insert(normalized.id.clone(), body.clone())
                .is_some()
            {
                return Err(RewriteAuthorityFailureV3::NodeUniverse(
                    AuditUnknownReason::ProvenanceCollision,
                ));
            }
        }
    }

    // Fresh rules: the kernel-typed rewrite inventory, exactly one entry
    // per sealed successor equation, cross-checked against the carrier's
    // subject bundle registry. An inventory with equations requires the
    // typed inventory; one without forbids it.
    let fresh_rules = match (inventory.equations().is_empty(), typed_rewrite_inventory) {
        (true, None) => Vec::new(),
        (true, Some(_)) => {
            return Err(RewriteAuthorityFailureV3::FreshInventoryBindingMismatch);
        }
        (false, None) => {
            return Err(RewriteAuthorityFailureV3::MissingTypedRewriteInventory);
        }
        (false, Some(typed)) => {
            if typed.inventory_digest() != inventory.digest()
                || typed.manifest_digest() != v1_manifest.candidate_digest()
            {
                return Err(RewriteAuthorityFailureV3::FreshInventoryBindingMismatch);
            }
            let mut sealed = inventory
                .equations()
                .iter()
                .map(|equation| equation.equation().clone())
                .collect::<BTreeSet<_>>();
            let mut rules = Vec::new();
            for entry in typed.entries() {
                if !sealed.remove(entry.equation()) {
                    return Err(RewriteAuthorityFailureV3::FreshInventoryBindingMismatch);
                }
                rules.push(FreshRuleV3 {
                    equation: entry.equation().clone(),
                    owner: entry.owner_head().clone(),
                    constructor: entry.constructor().clone(),
                    arity: usize::from(entry.arity()),
                    right: entry.right().clone(),
                });
            }
            if !sealed.is_empty() {
                return Err(RewriteAuthorityFailureV3::FreshInventoryBindingMismatch);
            }
            rules
        }
    };

    let build = BuildContext {
        kernel,
        signature,
        delta_bodies,
        fresh_rules,
        node_limit: v3_manifest.manifest().maximum_raw_derivations as usize,
        disposition_limit: v3_manifest.manifest().maximum_tuple_dispositions as usize,
    };

    // Node universe: every root-inventory judgment and every census
    // occurrence (binder-local typed subterms), closed under reduction
    // and under the carrier's finite construction substitutions.
    let mut nodes: Vec<RewriteNodeV3> = Vec::new();
    let mut node_indices: BTreeMap<RewriteNodeIdV3, usize> = BTreeMap::new();
    let mut queue: VecDeque<usize> = VecDeque::new();

    let push_node = |context: DependentContext,
                     term: Term,
                     judgment: RewriteNodeJudgmentV3,
                     nodes: &mut Vec<RewriteNodeV3>,
                     node_indices: &mut BTreeMap<RewriteNodeIdV3, usize>,
                     queue: &mut VecDeque<usize>|
     -> Result<usize, RewriteAuthorityFailureV3> {
        let node = make_node(context, term, judgment);
        if let Some(index) = node_indices.get(&node.id) {
            if nodes[*index] != node {
                return Err(RewriteAuthorityFailureV3::NodeUniverse(
                    AuditUnknownReason::ProvenanceCollision,
                ));
            }
            return Ok(*index);
        }
        if nodes.len() >= build.node_limit {
            return Err(RewriteAuthorityFailureV3::ResourceExhausted {
                stage: "node universe",
            });
        }
        let index = nodes.len();
        node_indices.insert(node.id.clone(), index);
        nodes.push(node);
        queue.push_back(index);
        Ok(index)
    };

    for root in root_inventory.roots() {
        match root.request() {
            TypedOccurrenceRootRequestV1::HasType { context, term, ty } => {
                push_node(
                    context.clone(),
                    term.clone(),
                    RewriteNodeJudgmentV3::HasType { ty: ty.clone() },
                    &mut nodes,
                    &mut node_indices,
                    &mut queue,
                )?;
            }
            TypedOccurrenceRootRequestV1::Equation {
                context,
                left,
                right,
                ty,
            } => {
                push_node(
                    context.clone(),
                    left.clone(),
                    RewriteNodeJudgmentV3::HasType { ty: ty.clone() },
                    &mut nodes,
                    &mut node_indices,
                    &mut queue,
                )?;
                push_node(
                    context.clone(),
                    right.clone(),
                    RewriteNodeJudgmentV3::HasType { ty: ty.clone() },
                    &mut nodes,
                    &mut node_indices,
                    &mut queue,
                )?;
            }
            TypedOccurrenceRootRequestV1::TypeFormation { context, term } => {
                push_node(
                    context.clone(),
                    term.clone(),
                    RewriteNodeJudgmentV3::TypeFormation,
                    &mut nodes,
                    &mut node_indices,
                    &mut queue,
                )?;
            }
        }
    }
    for occurrence in census.batch().occurrences() {
        let judgment = match occurrence.checked_judgment() {
            LocalJudgmentV1::HasType { ty } => RewriteNodeJudgmentV3::HasType { ty: ty.clone() },
            LocalJudgmentV1::TypeFormation { .. } => RewriteNodeJudgmentV3::TypeFormation,
        };
        push_node(
            occurrence.local_context().clone(),
            occurrence.term().clone(),
            judgment,
            &mut nodes,
            &mut node_indices,
            &mut queue,
        )?;
    }
    if nodes.is_empty() {
        return Err(RewriteAuthorityFailureV3::NodeUniverse(
            AuditUnknownReason::IncompleteEnumeration,
        ));
    }

    // The carrier's finite construction substitutions, deduplicated by
    // (source context, target context, images).
    let mut substitution_witnesses: Vec<(Digest, DependentContext, DependentContext, Vec<Term>)> =
        Vec::new();
    let mut seen_witnesses = BTreeSet::new();
    for witness in carrier.substitution_census().witnesses() {
        let key = Digest::of_canonical(
            "pen-semantic-audit/rewrite-authority-substitution-witness/v3",
            &WitnessKeyMaterial {
                source: witness.source_context(),
                target: witness.target_context(),
                images: witness.images(),
            },
        );
        if seen_witnesses.insert(key.clone()) {
            substitution_witnesses.push((
                key,
                witness.source_context().clone(),
                witness.target_context().clone(),
                witness.images().to_vec(),
            ));
        }
    }

    // Saturate: reductions and substitution images, kernel-checking every
    // new node at its judgment.
    let mut edges: Vec<RewriteEdgeV3> = Vec::new();
    let mut edge_indices: BTreeMap<RewriteEdgeIdV3, usize> = BTreeMap::new();
    let mut applicable_count: u64 = 0;
    let mut inapplicable_count: u64 = 0;

    while let Some(index) = queue.pop_front() {
        let node = nodes[index].clone();
        kernel_check_node(&build, &node)?;

        // The node universe is deliberately NOT closed under the
        // construction substitutions: the witnesses are derivation-local
        // and composing them into a global carrier is the registered
        // non-finite closure. Stability is a per-edge one-step
        // commutation census below.

        // Reductions at every position.
        let positions = enumerate_positions(&node.term, kernel.limits().max_depth as usize).ok_or(
            RewriteAuthorityFailureV3::ResourceExhausted {
                stage: "position enumeration",
            },
        )?;
        for (position, focus) in positions {
            let mut consider = |rule: RewriteRuleV3,
                                replacement: Option<Term>,
                                edges: &mut Vec<RewriteEdgeV3>,
                                edge_indices: &mut BTreeMap<RewriteEdgeIdV3, usize>,
                                nodes: &mut Vec<RewriteNodeV3>,
                                node_indices: &mut BTreeMap<RewriteNodeIdV3, usize>,
                                queue: &mut VecDeque<usize>|
             -> Result<(), RewriteAuthorityFailureV3> {
                if applicable_count
                    .checked_add(inapplicable_count)
                    .is_none_or(|total| total as usize >= build.disposition_limit)
                {
                    return Err(RewriteAuthorityFailureV3::ResourceExhausted {
                        stage: "rule dispositions",
                    });
                }
                match replacement {
                    None => {
                        inapplicable_count += 1;
                        Ok(())
                    }
                    Some(replacement) => {
                        applicable_count += 1;
                        let target_term = replace_at_position(
                            &node.term,
                            position.child_ordinals(),
                            &replacement,
                        )
                        .ok_or(RewriteAuthorityFailureV3::NodeUniverse(
                            AuditUnknownReason::MalformedInput,
                        ))?;
                        let target_index = push_node(
                            node.context.clone(),
                            target_term,
                            node.judgment.clone(),
                            nodes,
                            node_indices,
                            queue,
                        )?;
                        let target_id = nodes[target_index].id.clone();
                        let edge = make_edge(node.id.clone(), target_id, position.clone(), rule);
                        if let Some(previous) = edge_indices.get(&edge.id) {
                            if edges[*previous] != edge {
                                return Err(RewriteAuthorityFailureV3::NodeUniverse(
                                    AuditUnknownReason::ProvenanceCollision,
                                ));
                            }
                        } else {
                            edge_indices.insert(edge.id.clone(), edges.len());
                            edges.push(edge);
                        }
                        Ok(())
                    }
                }
            };

            // Ordinary beta.
            let beta = match &focus {
                Term::Apply { function, argument } => match function.as_ref() {
                    Term::Lambda { body, .. } => subst_top(body, argument),
                    _ => None,
                },
                _ => None,
            };
            consider(
                RewriteRuleV3::OrdinaryBeta,
                beta,
                &mut edges,
                &mut edge_indices,
                &mut nodes,
                &mut node_indices,
                &mut queue,
            )?;

            // Public delta for each bodyful declaration.
            for (declaration, body) in &build.delta_bodies {
                let delta = match &focus {
                    Term::Global { id } if id == declaration => Some(body.clone()),
                    _ => None,
                };
                consider(
                    RewriteRuleV3::PublicDelta {
                        declaration: declaration.clone(),
                    },
                    delta,
                    &mut edges,
                    &mut edge_indices,
                    &mut nodes,
                    &mut node_indices,
                    &mut queue,
                )?;
            }

            // Fresh constructor computation for each typed rule.
            for rule in &build.fresh_rules {
                let fresh = match_fresh_rule(rule, &focus);
                consider(
                    RewriteRuleV3::FreshEquation {
                        equation: rule.equation.clone(),
                    },
                    fresh,
                    &mut edges,
                    &mut edge_indices,
                    &mut nodes,
                    &mut node_indices,
                    &mut queue,
                )?;
            }
        }
    }

    // Every edge preserves its typed judgment: source and target replay
    // at the same context and judgment (typed Q0 stability, kernel leg).
    for edge in &edges {
        let source = &nodes[node_indices[&edge.source]];
        let target = &nodes[node_indices[&edge.target]];
        if source.context != target.context || source.judgment != target.judgment {
            return Err(RewriteAuthorityFailureV3::KernelReplay(
                AuditUnknownReason::KernelCouldNotCertify,
            ));
        }
        kernel_check_node(&build, target)?;
    }

    // Termination: the finite graph is acyclic; ranks by longest path.
    let (ranks, normal_forms) = analyze_termination_and_confluence(&nodes, &edges)?;

    // The all-pairs overlap census: every unordered pair of applicable
    // reductions at one node, classified and joined at the node's unique
    // normal form.
    let mut edges_by_source: BTreeMap<RewriteNodeIdV3, Vec<usize>> = BTreeMap::new();
    for (index, edge) in edges.iter().enumerate() {
        edges_by_source
            .entry(edge.source.clone())
            .or_default()
            .push(index);
    }
    let mut overlap_pairs = Vec::new();
    for (node_id, edge_group) in &edges_by_source {
        let node_normal_form = normal_forms
            .get(node_id)
            .ok_or(RewriteAuthorityFailureV3::NotConfluent)?;
        for (left_ordinal, left_index) in edge_group.iter().enumerate() {
            for right_index in edge_group.iter().skip(left_ordinal + 1) {
                if overlap_pairs.len() >= build.disposition_limit {
                    return Err(RewriteAuthorityFailureV3::ResourceExhausted {
                        stage: "overlap pairs",
                    });
                }
                let left = &edges[*left_index];
                let right = &edges[*right_index];
                let relation = if left.position == right.position {
                    OverlapRelationV3::SamePosition
                } else if left.position.is_prefix_of(&right.position)
                    || right.position.is_prefix_of(&left.position)
                {
                    OverlapRelationV3::Nested
                } else {
                    OverlapRelationV3::Disjoint
                };
                // Join: both targets reduce to the same unique normal
                // form as the source; confluence analysis has already
                // certified uniqueness, restated here per pair.
                let left_normal_form = normal_forms
                    .get(&left.target)
                    .ok_or(RewriteAuthorityFailureV3::NotConfluent)?;
                let right_normal_form = normal_forms
                    .get(&right.target)
                    .ok_or(RewriteAuthorityFailureV3::NotConfluent)?;
                if left_normal_form != node_normal_form || right_normal_form != node_normal_form {
                    return Err(RewriteAuthorityFailureV3::UnjoinableOverlap);
                }
                overlap_pairs.push(OverlapPairV3 {
                    node: node_id.clone(),
                    left_edge: left.id.clone(),
                    right_edge: right.id.clone(),
                    relation,
                    join_normal_form: node_normal_form.clone(),
                });
            }
        }
    }

    // Substitution stability: one-step commutation of every edge with
    // every carrier witness applicable to its context. The substituted
    // endpoints are kernel-checked in the witness's target context, and
    // replaying the edge's rule at the edge's exact position on the
    // substituted source must produce exactly the substituted target.
    let mut substitution_stability = Vec::new();
    for edge in &edges {
        let source = &nodes[node_indices[&edge.source]];
        let target = &nodes[node_indices[&edge.target]];
        for (witness_digest, _source_context, target_context, images) in substitution_witnesses
            .iter()
            .filter(|(_, source_ctx, _, _)| *source_ctx == source.context)
        {
            if substitution_stability.len() >= build.disposition_limit {
                return Err(RewriteAuthorityFailureV3::ResourceExhausted {
                    stage: "substitution stability",
                });
            }
            let substituted_source = substitute_node(source, target_context, images)
                .ok_or(RewriteAuthorityFailureV3::UnstableEdge)?;
            let substituted_target = substitute_node(target, target_context, images)
                .ok_or(RewriteAuthorityFailureV3::UnstableEdge)?;
            kernel_check_node(&build, &substituted_source)?;
            kernel_check_node(&build, &substituted_target)?;
            let focus =
                subterm_at_position(&substituted_source.term, edge.position.child_ordinals())
                    .ok_or(RewriteAuthorityFailureV3::UnstableEdge)?;
            let replacement = match &edge.rule {
                RewriteRuleV3::OrdinaryBeta => match &focus {
                    Term::Apply { function, argument } => match function.as_ref() {
                        Term::Lambda { body, .. } => subst_top(body, argument),
                        _ => None,
                    },
                    _ => None,
                },
                RewriteRuleV3::PublicDelta { declaration } => match &focus {
                    Term::Global { id } if id == declaration => {
                        build.delta_bodies.get(declaration).cloned()
                    }
                    _ => None,
                },
                RewriteRuleV3::FreshEquation { equation } => build
                    .fresh_rules
                    .iter()
                    .find(|rule| &rule.equation == equation)
                    .and_then(|rule| match_fresh_rule(rule, &focus)),
            }
            .ok_or(RewriteAuthorityFailureV3::UnstableEdge)?;
            let replayed = replace_at_position(
                &substituted_source.term,
                edge.position.child_ordinals(),
                &replacement,
            )
            .ok_or(RewriteAuthorityFailureV3::UnstableEdge)?;
            if replayed != substituted_target.term {
                return Err(RewriteAuthorityFailureV3::UnstableEdge);
            }
            substitution_stability.push(SubstitutionStabilityV3 {
                edge: edge.id.clone(),
                witness_digest: witness_digest.clone(),
                substituted_source_digest: substituted_source.id.digest().clone(),
                substituted_target_digest: substituted_target.id.digest().clone(),
            });
        }
    }

    // Predecessor conservativity: independently replay beta and every
    // bodyful predecessor delta at every position, against the predecessor
    // signature itself, and require exact equality with the main graph's
    // predecessor restriction. Merely checking that an existing edge has a
    // predecessor-lawful rule tag would not prove completeness or that its
    // target is the result of that rule.
    if historical.inventory_digest() != inventory.digest()
        || historical.boundary_digest() != inventory.predecessor_boundary().digest()
    {
        return Err(RewriteAuthorityFailureV3::PredecessorNotConservative);
    }
    let predecessor = verify_predecessor_conservativity_v3(
        kernel,
        inventory.predecessor_boundary(),
        inventory.digest(),
        historical.digest(),
        &nodes,
        &edges,
        &normal_forms,
        kernel.limits().max_depth as usize,
        build.disposition_limit,
    )?;

    // Representation-rule identity census: the kernel term syntax is
    // de Bruijn-canonical, has no explicit substitution or telescope
    // syntax, and a canonical unit; the V3 representation rules are
    // therefore identities on every node of this graph. Recorded, not
    // silently assumed.
    let representation_identity_census_digest = Digest::of_canonical(
        "pen-semantic-audit/rewrite-authority-representation-identity/v3",
        &RepresentationMaterial {
            node_count: nodes.len() as u64,
            kernel_protocol: &kernel.kernel_protocol_digest(),
        },
    );

    let mut sorted_normal_forms: Vec<(RewriteNodeIdV3, RewriteNodeIdV3)> =
        normal_forms.into_iter().collect();
    sorted_normal_forms.sort_by(|left, right| left.0.cmp(&right.0));
    let termination_rank_digest = Digest::of_canonical(
        "pen-semantic-audit/rewrite-authority-termination-ranks/v3",
        &RankMaterial { ranks: &ranks },
    );

    let mut authority = VerifiedRewriteAuthorityV3 {
        schema_version: REWRITE_AUTHORITY_SCHEMA_VERSION_V3,
        semantic_manifest_digest: manifest_digest.clone(),
        v1_manifest_digest: v1_manifest.candidate_digest().clone(),
        v2_manifest_digest: v2_manifest.candidate_digest().clone(),
        signature_digest: signature.digest().clone(),
        kernel_protocol_digest: kernel.kernel_protocol_digest(),
        carrier_digest: carrier.digest().clone(),
        root_inventory_digest: root_inventory.digest().clone(),
        census_digest: census.digest().clone(),
        subject_bundle_digest: subject_bundle.digest().clone(),
        typed_rewrite_inventory_digest: typed_rewrite_inventory.map(|typed| typed.digest().clone()),
        historical_base_digest: historical.digest().clone(),
        nodes: Arc::from(nodes.into_boxed_slice()),
        edges: Arc::from(edges.into_boxed_slice()),
        applicable_disposition_count: applicable_count,
        inapplicable_disposition_count: inapplicable_count,
        representation_identity_census_digest,
        overlap_pairs: Arc::from(overlap_pairs.into_boxed_slice()),
        substitution_stability: Arc::from(substitution_stability.into_boxed_slice()),
        normal_forms: Arc::from(sorted_normal_forms.into_boxed_slice()),
        termination_rank_digest,
        predecessor_node_count: predecessor.nodes.len() as u64,
        predecessor_edge_count: predecessor.edges.len() as u64,
        predecessor_reconstruction_digest: predecessor.digest,
        digest: Digest::of_bytes(b"pending rewrite authority v3"),
    };
    authority.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-rewrite-authority/v3",
        &authority,
    );
    Ok(authority)
}

#[allow(clippy::too_many_arguments)]
pub fn verify_rewrite_authority_v3(
    v1_manifest: &VerifiedSemanticAuditManifestV1,
    v2_manifest: &VerifiedSemanticAuditManifestV2,
    v3_manifest: &VerifiedSemanticAuditManifestV3,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    compatibility: &VerifiedPublicInventoryCompatibilityV2,
    carrier: &VerifiedNativeRankInductiveCarrierV3,
    root_inventory: &VerifiedCarrierRootInventoryV3,
    census: &VerifiedSynthesisBackedTypedOccurrenceCensusV3,
    subject_bundle: &VerifiedCarrierSubjectBundleV3,
    typed_rewrite_inventory: Option<&VerifiedTypedRewriteInventoryV1>,
) -> AuditDecision<VerifiedRewriteAuthorityV3> {
    match diagnose_rewrite_authority_v3(
        v1_manifest,
        v2_manifest,
        v3_manifest,
        kernel,
        inventory,
        compatibility,
        carrier,
        root_inventory,
        census,
        subject_bundle,
        typed_rewrite_inventory,
    ) {
        Ok(authority) => AuditDecision::Proven(authority),
        Err(failure) => failure.into_decision(),
    }
}

fn make_node(
    context: DependentContext,
    term: Term,
    judgment: RewriteNodeJudgmentV3,
) -> RewriteNodeV3 {
    struct Material<'a> {
        context: &'a DependentContext,
        term: &'a Term,
        judgment: &'a RewriteNodeJudgmentV3,
    }
    impl CanonicalEncode for Material<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.context.encode_canonical(encoder);
            self.term.encode_canonical(encoder);
            self.judgment.encode_canonical(encoder);
        }
    }
    let id = RewriteNodeIdV3(Digest::of_canonical(
        "pen-semantic-audit/rewrite-authority-node/v3",
        &Material {
            context: &context,
            term: &term,
            judgment: &judgment,
        },
    ));
    RewriteNodeV3 {
        id,
        context,
        term,
        judgment,
    }
}

fn make_edge(
    source: RewriteNodeIdV3,
    target: RewriteNodeIdV3,
    position: RewritePositionV3,
    rule: RewriteRuleV3,
) -> RewriteEdgeV3 {
    struct Material<'a> {
        source: &'a RewriteNodeIdV3,
        target: &'a RewriteNodeIdV3,
        position: &'a RewritePositionV3,
        rule: &'a RewriteRuleV3,
    }
    impl CanonicalEncode for Material<'_> {
        fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
            self.source.encode_canonical(encoder);
            self.target.encode_canonical(encoder);
            self.position.encode_canonical(encoder);
            self.rule.encode_canonical(encoder);
        }
    }
    let id = RewriteEdgeIdV3(Digest::of_canonical(
        "pen-semantic-audit/rewrite-authority-edge/v3",
        &Material {
            source: &source,
            target: &target,
            position: &position,
            rule: &rule,
        },
    ));
    RewriteEdgeV3 {
        id,
        source,
        target,
        position,
        rule,
    }
}

fn kernel_check_node(
    build: &BuildContext<'_>,
    node: &RewriteNodeV3,
) -> Result<(), RewriteAuthorityFailureV3> {
    let input = match &node.judgment {
        RewriteNodeJudgmentV3::HasType { ty } => OpenJudgment::HasType {
            context: node.context.clone(),
            term: node.term.clone(),
            ty: ty.clone(),
        },
        RewriteNodeJudgmentV3::TypeFormation => OpenJudgment::TypeFormation {
            context: node.context.clone(),
            term: node.term.clone(),
        },
    };
    build
        .kernel
        .verify_open_judgment(build.signature, &input)
        .map(|_| ())
        .map_err(|error| RewriteAuthorityFailureV3::KernelReplay(kernel_unknown(error)))
}

fn kernel_unknown(error: KernelError) -> AuditUnknownReason {
    match error {
        KernelError::ResourceExhausted(
            ResourceKind::Operations | ResourceKind::Depth | ResourceKind::Normalization,
        ) => AuditUnknownReason::ResourceExhausted,
        _ => AuditUnknownReason::KernelCouldNotCertify,
    }
}

#[allow(clippy::too_many_arguments)]
fn verify_predecessor_conservativity_v3(
    kernel: &Kernel,
    predecessor_signature: &VerifiedSignature,
    inventory_digest: &Digest,
    historical_digest: &Digest,
    nodes: &[RewriteNodeV3],
    edges: &[RewriteEdgeV3],
    normal_forms: &BTreeMap<RewriteNodeIdV3, RewriteNodeIdV3>,
    depth_limit: usize,
    disposition_limit: usize,
) -> Result<PredecessorReconstructionV3, RewriteAuthorityFailureV3> {
    // The independent rule inventory comes directly from the predecessor
    // signature. It is not obtained by filtering the successor rule map.
    let predecessor_declarations = predecessor_signature
        .declarations()
        .iter()
        .map(|declaration| declaration.id.clone())
        .collect::<BTreeSet<_>>();
    let mut predecessor_delta = BTreeMap::new();
    for declaration in predecessor_signature.declarations() {
        if let Some(body) = &declaration.body
            && predecessor_delta
                .insert(declaration.id.clone(), body.clone())
                .is_some()
        {
            return Err(RewriteAuthorityFailureV3::PredecessorNotConservative);
        }
    }

    // The restriction is language-based: every context entry, subject, and
    // result type must mention only predecessor declarations. Recheck every
    // selected node against the predecessor signature rather than inheriting
    // its successor-signature typing judgment.
    let predecessor_candidates = nodes
        .iter()
        .filter(|node| node_mentions_only(node, &predecessor_declarations))
        .map(|node| (node.id.clone(), node))
        .collect::<BTreeMap<_, _>>();
    let predecessor_nodes = predecessor_candidates
        .keys()
        .cloned()
        .collect::<BTreeSet<_>>();
    for node in predecessor_candidates.values() {
        kernel_check_predecessor_node(kernel, predecessor_signature, node)?;
    }

    // First form the exact restriction of the already verified successor
    // graph. Every outgoing edge from an old-language node must remain in the
    // old language and use only beta or a bodyful predecessor delta rule.
    let mut restricted_edges = BTreeMap::new();
    for edge in edges {
        if !predecessor_nodes.contains(&edge.source) {
            continue;
        }
        if !predecessor_nodes.contains(&edge.target)
            || match &edge.rule {
                RewriteRuleV3::OrdinaryBeta => false,
                RewriteRuleV3::PublicDelta { declaration } => {
                    !predecessor_delta.contains_key(declaration)
                }
                RewriteRuleV3::FreshEquation { .. } => true,
            }
        {
            return Err(RewriteAuthorityFailureV3::PredecessorNotConservative);
        }
        match restricted_edges.insert(edge.id.clone(), edge.clone()) {
            Some(previous) if previous != *edge => {
                return Err(RewriteAuthorityFailureV3::PredecessorNotConservative);
            }
            _ => {}
        }
    }

    // Now rebuild the edge relation from scratch. For every predecessor node,
    // enumerate every term position and dispose beta plus every bodyful
    // predecessor delta. Applicable targets are kernel-checked under the
    // predecessor signature and must already occur in the restricted node
    // universe. Exact edge equality proves both directions of conservativity.
    let mut replayed_edges = BTreeMap::new();
    let mut disposition_count = 0_u64;
    for (source_id, source) in &predecessor_candidates {
        let positions = enumerate_positions(&source.term, depth_limit).ok_or(
            RewriteAuthorityFailureV3::ResourceExhausted {
                stage: "predecessor position enumeration",
            },
        )?;
        for (position, focus) in positions {
            let beta = match &focus {
                Term::Apply { function, argument } => match function.as_ref() {
                    Term::Lambda { body, .. } => subst_top(body, argument),
                    _ => None,
                },
                _ => None,
            };
            let mut replay = |rule: RewriteRuleV3,
                              replacement: Option<Term>|
             -> Result<(), RewriteAuthorityFailureV3> {
                disposition_count = disposition_count.checked_add(1).ok_or(
                    RewriteAuthorityFailureV3::ResourceExhausted {
                        stage: "predecessor rule dispositions",
                    },
                )?;
                if usize::try_from(disposition_count)
                    .ok()
                    .is_none_or(|count| count > disposition_limit)
                {
                    return Err(RewriteAuthorityFailureV3::ResourceExhausted {
                        stage: "predecessor rule dispositions",
                    });
                }
                let Some(replacement) = replacement else {
                    return Ok(());
                };
                let target_term =
                    replace_at_position(&source.term, position.child_ordinals(), &replacement)
                        .ok_or(RewriteAuthorityFailureV3::PredecessorNotConservative)?;
                let target =
                    make_node(source.context.clone(), target_term, source.judgment.clone());
                kernel_check_predecessor_node(kernel, predecessor_signature, &target)?;
                let expected_target = predecessor_candidates
                    .get(&target.id)
                    .ok_or(RewriteAuthorityFailureV3::PredecessorNotConservative)?;
                if **expected_target != target {
                    return Err(RewriteAuthorityFailureV3::PredecessorNotConservative);
                }
                let edge = make_edge(source_id.clone(), target.id.clone(), position.clone(), rule);
                match replayed_edges.insert(edge.id.clone(), edge.clone()) {
                    Some(previous) if previous != edge => {
                        Err(RewriteAuthorityFailureV3::PredecessorNotConservative)
                    }
                    _ => Ok(()),
                }
            };

            replay(RewriteRuleV3::OrdinaryBeta, beta)?;
            for (declaration, body) in &predecessor_delta {
                let replacement = match &focus {
                    Term::Global { id } if id == declaration => Some(body.clone()),
                    _ => None,
                };
                replay(
                    RewriteRuleV3::PublicDelta {
                        declaration: declaration.clone(),
                    },
                    replacement,
                )?;
            }
        }
    }
    if replayed_edges != restricted_edges {
        return Err(RewriteAuthorityFailureV3::PredecessorNotConservative);
    }

    let mut predecessor_normal_forms = BTreeMap::new();
    for node in &predecessor_nodes {
        let normal_form = normal_forms
            .get(node)
            .ok_or(RewriteAuthorityFailureV3::PredecessorNotConservative)?;
        if !predecessor_nodes.contains(normal_form) {
            return Err(RewriteAuthorityFailureV3::PredecessorNotConservative);
        }
        predecessor_normal_forms.insert(node.clone(), normal_form.clone());
    }

    // Re-analyze the independently replayed restriction and require its
    // normal-form map to be exactly the restriction of the main theorem.
    let predecessor_node_values = predecessor_candidates
        .values()
        .map(|node| (*node).clone())
        .collect::<Vec<_>>();
    let predecessor_edge_values = replayed_edges.values().cloned().collect::<Vec<_>>();
    let (_, replayed_normal_forms) =
        analyze_termination_and_confluence(&predecessor_node_values, &predecessor_edge_values)
            .map_err(|_| RewriteAuthorityFailureV3::PredecessorNotConservative)?;
    if replayed_normal_forms != predecessor_normal_forms {
        return Err(RewriteAuthorityFailureV3::PredecessorNotConservative);
    }

    let predecessor_edges = replayed_edges.keys().cloned().collect::<BTreeSet<_>>();
    let digest = Digest::of_canonical(
        "pen-semantic-audit/rewrite-authority-predecessor-reconstruction/v3",
        &PredecessorMaterial {
            inventory: inventory_digest,
            boundary: predecessor_signature.digest(),
            historical: historical_digest,
            nodes: &predecessor_nodes,
            edges: &predecessor_edges,
            normal_forms: &predecessor_normal_forms,
            disposition_count,
        },
    );
    Ok(PredecessorReconstructionV3 {
        nodes: predecessor_nodes,
        edges: predecessor_edges,
        digest,
    })
}

fn kernel_check_predecessor_node(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    node: &RewriteNodeV3,
) -> Result<(), RewriteAuthorityFailureV3> {
    let judgment = match &node.judgment {
        RewriteNodeJudgmentV3::HasType { ty } => OpenJudgment::HasType {
            context: node.context.clone(),
            term: node.term.clone(),
            ty: ty.clone(),
        },
        RewriteNodeJudgmentV3::TypeFormation => OpenJudgment::TypeFormation {
            context: node.context.clone(),
            term: node.term.clone(),
        },
    };
    kernel
        .verify_open_judgment(signature, &judgment)
        .map(|_| ())
        .map_err(|_| RewriteAuthorityFailureV3::PredecessorNotConservative)
}

fn node_mentions_only(node: &RewriteNodeV3, allowed: &BTreeSet<GlobalId>) -> bool {
    let mut mentions_only = |term: &Term| term_mentions_only(term, allowed);
    node.context.0.iter().all(&mut mentions_only)
        && mentions_only(&node.term)
        && match &node.judgment {
            RewriteNodeJudgmentV3::HasType { ty } => mentions_only(ty),
            RewriteNodeJudgmentV3::TypeFormation => true,
        }
}

fn term_mentions_only(term: &Term, allowed: &BTreeSet<GlobalId>) -> bool {
    match term {
        Term::Global { id } => allowed.contains(id),
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            term_mentions_only(parameter, allowed) && term_mentions_only(body, allowed)
        }
        Term::Lambda {
            parameter_type,
            body,
        } => term_mentions_only(parameter_type, allowed) && term_mentions_only(body, allowed),
        Term::Apply { function, argument } => {
            term_mentions_only(function, allowed) && term_mentions_only(argument, allowed)
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            term_mentions_only(sigma_type, allowed)
                && term_mentions_only(first, allowed)
                && term_mentions_only(second, allowed)
        }
        Term::First { pair } | Term::Second { pair } => term_mentions_only(pair, allowed),
        Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => true,
    }
}

/// Enumerate every subterm position of a lambda/unit term, root first.
/// Child ordinals: 0 = first child, 1 = second child.
fn enumerate_positions(term: &Term, depth_limit: usize) -> Option<Vec<(RewritePositionV3, Term)>> {
    let mut positions = Vec::new();
    let mut stack = vec![(RewritePositionV3::default(), term.clone())];
    while let Some((position, focus)) = stack.pop() {
        if position.child_ordinals().len() > depth_limit {
            return None;
        }
        match &focus {
            Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
                stack.push((position.child(0), parameter.as_ref().clone()));
                stack.push((position.child(1), body.as_ref().clone()));
            }
            Term::Lambda {
                parameter_type,
                body,
            } => {
                stack.push((position.child(0), parameter_type.as_ref().clone()));
                stack.push((position.child(1), body.as_ref().clone()));
            }
            Term::Apply { function, argument } => {
                stack.push((position.child(0), function.as_ref().clone()));
                stack.push((position.child(1), argument.as_ref().clone()));
            }
            Term::Pair {
                sigma_type,
                first,
                second,
            } => {
                stack.push((position.child(0), sigma_type.as_ref().clone()));
                stack.push((position.child(1), first.as_ref().clone()));
                stack.push((position.child(2), second.as_ref().clone()));
            }
            Term::First { pair } | Term::Second { pair } => {
                stack.push((position.child(0), pair.as_ref().clone()));
            }
            Term::Sort { .. }
            | Term::Var { .. }
            | Term::Global { .. }
            | Term::UnitType
            | Term::Unit => {}
        }
        positions.push((position, focus));
    }
    positions.sort_by(|left, right| left.0.cmp(&right.0));
    Some(positions)
}

fn subterm_at_position(term: &Term, path: &[u16]) -> Option<Term> {
    let Some((&ordinal, rest)) = path.split_first() else {
        return Some(term.clone());
    };
    match (term, ordinal) {
        (Term::Pi { parameter, .. }, 0) | (Term::Sigma { parameter, .. }, 0) => {
            subterm_at_position(parameter, rest)
        }
        (Term::Pi { body, .. }, 1) | (Term::Sigma { body, .. }, 1) => {
            subterm_at_position(body, rest)
        }
        (Term::Lambda { parameter_type, .. }, 0) => subterm_at_position(parameter_type, rest),
        (Term::Lambda { body, .. }, 1) => subterm_at_position(body, rest),
        (Term::Apply { function, .. }, 0) => subterm_at_position(function, rest),
        (Term::Apply { argument, .. }, 1) => subterm_at_position(argument, rest),
        (Term::Pair { sigma_type, .. }, 0) => subterm_at_position(sigma_type, rest),
        (Term::Pair { first, .. }, 1) => subterm_at_position(first, rest),
        (Term::Pair { second, .. }, 2) => subterm_at_position(second, rest),
        (Term::First { pair }, 0) | (Term::Second { pair }, 0) => subterm_at_position(pair, rest),
        _ => None,
    }
}

fn replace_at_position(term: &Term, path: &[u16], replacement: &Term) -> Option<Term> {
    let Some((&ordinal, rest)) = path.split_first() else {
        return Some(replacement.clone());
    };
    Some(match (term, ordinal) {
        (Term::Pi { parameter, body }, 0) => Term::Pi {
            parameter: Box::new(replace_at_position(parameter, rest, replacement)?),
            body: body.clone(),
        },
        (Term::Pi { parameter, body }, 1) => Term::Pi {
            parameter: parameter.clone(),
            body: Box::new(replace_at_position(body, rest, replacement)?),
        },
        (Term::Sigma { parameter, body }, 0) => Term::Sigma {
            parameter: Box::new(replace_at_position(parameter, rest, replacement)?),
            body: body.clone(),
        },
        (Term::Sigma { parameter, body }, 1) => Term::Sigma {
            parameter: parameter.clone(),
            body: Box::new(replace_at_position(body, rest, replacement)?),
        },
        (
            Term::Lambda {
                parameter_type,
                body,
            },
            0,
        ) => Term::Lambda {
            parameter_type: Box::new(replace_at_position(parameter_type, rest, replacement)?),
            body: body.clone(),
        },
        (
            Term::Lambda {
                parameter_type,
                body,
            },
            1,
        ) => Term::Lambda {
            parameter_type: parameter_type.clone(),
            body: Box::new(replace_at_position(body, rest, replacement)?),
        },
        (Term::Apply { function, argument }, 0) => Term::Apply {
            function: Box::new(replace_at_position(function, rest, replacement)?),
            argument: argument.clone(),
        },
        (Term::Apply { function, argument }, 1) => Term::Apply {
            function: function.clone(),
            argument: Box::new(replace_at_position(argument, rest, replacement)?),
        },
        _ => return None,
    })
}

/// Shift free de Bruijn variables at or above `cutoff` by `amount`.
fn shift_term(term: &Term, amount: i64, cutoff: u32) -> Option<Term> {
    Some(match term {
        Term::Var { index } => {
            if *index >= cutoff {
                let shifted = i64::from(*index).checked_add(amount)?;
                Term::Var {
                    index: u32::try_from(shifted).ok()?,
                }
            } else {
                term.clone()
            }
        }
        Term::Pi { parameter, body } => Term::Pi {
            parameter: Box::new(shift_term(parameter, amount, cutoff)?),
            body: Box::new(shift_term(body, amount, cutoff.checked_add(1)?)?),
        },
        Term::Sigma { parameter, body } => Term::Sigma {
            parameter: Box::new(shift_term(parameter, amount, cutoff)?),
            body: Box::new(shift_term(body, amount, cutoff.checked_add(1)?)?),
        },
        Term::Lambda {
            parameter_type,
            body,
        } => Term::Lambda {
            parameter_type: Box::new(shift_term(parameter_type, amount, cutoff)?),
            body: Box::new(shift_term(body, amount, cutoff.checked_add(1)?)?),
        },
        Term::Apply { function, argument } => Term::Apply {
            function: Box::new(shift_term(function, amount, cutoff)?),
            argument: Box::new(shift_term(argument, amount, cutoff)?),
        },
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Term::Pair {
            sigma_type: Box::new(shift_term(sigma_type, amount, cutoff)?),
            first: Box::new(shift_term(first, amount, cutoff)?),
            second: Box::new(shift_term(second, amount, cutoff)?),
        },
        Term::First { pair } => Term::First {
            pair: Box::new(shift_term(pair, amount, cutoff)?),
        },
        Term::Second { pair } => Term::Second {
            pair: Box::new(shift_term(pair, amount, cutoff)?),
        },
        Term::Sort { .. } | Term::Global { .. } | Term::UnitType | Term::Unit => term.clone(),
    })
}

/// Substitute the newest bound variable (index 0) by `image` and shift
/// the remainder down: ordinary beta contraction.
fn subst_top(body: &Term, image: &Term) -> Option<Term> {
    fn walk(term: &Term, image: &Term, depth: u32) -> Option<Term> {
        Some(match term {
            Term::Var { index } => {
                if *index == depth {
                    shift_term(image, i64::from(depth), 0)?
                } else if *index > depth {
                    Term::Var {
                        index: index.checked_sub(1)?,
                    }
                } else {
                    term.clone()
                }
            }
            Term::Pi { parameter, body } => Term::Pi {
                parameter: Box::new(walk(parameter, image, depth)?),
                body: Box::new(walk(body, image, depth.checked_add(1)?)?),
            },
            Term::Sigma { parameter, body } => Term::Sigma {
                parameter: Box::new(walk(parameter, image, depth)?),
                body: Box::new(walk(body, image, depth.checked_add(1)?)?),
            },
            Term::Lambda {
                parameter_type,
                body,
            } => Term::Lambda {
                parameter_type: Box::new(walk(parameter_type, image, depth)?),
                body: Box::new(walk(body, image, depth.checked_add(1)?)?),
            },
            Term::Apply { function, argument } => Term::Apply {
                function: Box::new(walk(function, image, depth)?),
                argument: Box::new(walk(argument, image, depth)?),
            },
            Term::Pair {
                sigma_type,
                first,
                second,
            } => Term::Pair {
                sigma_type: Box::new(walk(sigma_type, image, depth)?),
                first: Box::new(walk(first, image, depth)?),
                second: Box::new(walk(second, image, depth)?),
            },
            Term::First { pair } => Term::First {
                pair: Box::new(walk(pair, image, depth)?),
            },
            Term::Second { pair } => Term::Second {
                pair: Box::new(walk(pair, image, depth)?),
            },
            Term::Sort { .. } | Term::Global { .. } | Term::UnitType | Term::Unit => term.clone(),
        })
    }
    walk(body, image, 0)
}

/// Apply a complete oldest-first image vector: the free variable with
/// oldest-first ordinal `k` (de Bruijn index `len-1-k` at binder depth
/// zero) is replaced by `images[k]`, weakened under `depth` binders.
fn apply_images(term: &Term, images: &[Term], depth: u32) -> Option<Term> {
    Some(match term {
        Term::Var { index } => {
            if *index >= depth {
                let free_index = index.checked_sub(depth)? as usize;
                if free_index >= images.len() {
                    return None;
                }
                let ordinal = images.len().checked_sub(1)?.checked_sub(free_index)?;
                shift_term(&images[ordinal], i64::from(depth), 0)?
            } else {
                term.clone()
            }
        }
        Term::Pi { parameter, body } => Term::Pi {
            parameter: Box::new(apply_images(parameter, images, depth)?),
            body: Box::new(apply_images(body, images, depth.checked_add(1)?)?),
        },
        Term::Sigma { parameter, body } => Term::Sigma {
            parameter: Box::new(apply_images(parameter, images, depth)?),
            body: Box::new(apply_images(body, images, depth.checked_add(1)?)?),
        },
        Term::Lambda {
            parameter_type,
            body,
        } => Term::Lambda {
            parameter_type: Box::new(apply_images(parameter_type, images, depth)?),
            body: Box::new(apply_images(body, images, depth.checked_add(1)?)?),
        },
        Term::Apply { function, argument } => Term::Apply {
            function: Box::new(apply_images(function, images, depth)?),
            argument: Box::new(apply_images(argument, images, depth)?),
        },
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Term::Pair {
            sigma_type: Box::new(apply_images(sigma_type, images, depth)?),
            first: Box::new(apply_images(first, images, depth)?),
            second: Box::new(apply_images(second, images, depth)?),
        },
        Term::First { pair } => Term::First {
            pair: Box::new(apply_images(pair, images, depth)?),
        },
        Term::Second { pair } => Term::Second {
            pair: Box::new(apply_images(pair, images, depth)?),
        },
        Term::Sort { .. } | Term::Global { .. } | Term::UnitType | Term::Unit => term.clone(),
    })
}

fn substitute_node(
    node: &RewriteNodeV3,
    target_context: &DependentContext,
    images: &[Term],
) -> Option<RewriteNodeV3> {
    let term = apply_images(&node.term, images, 0)?;
    let judgment = match &node.judgment {
        RewriteNodeJudgmentV3::HasType { ty } => RewriteNodeJudgmentV3::HasType {
            ty: apply_images(ty, images, 0)?,
        },
        RewriteNodeJudgmentV3::TypeFormation => RewriteNodeJudgmentV3::TypeFormation,
    };
    Some(make_node(target_context.clone(), term, judgment))
}

/// Match one typed fresh rule at a focus: the exact owner-headed spine
/// with the constructor in scrutinee position. Returns the instantiated
/// right-hand side.
///
/// The rule's right side lives in the restricted normalizer's prefix
/// parameter context (the scrutinee slot is consumed by the constructor
/// in the spine), so the oldest-first images are exactly the matched
/// spine arguments before the constructor.
fn match_fresh_rule(rule: &FreshRuleV3, focus: &Term) -> Option<Term> {
    let mut head = focus;
    let mut spine = Vec::new();
    while let Term::Apply { function, argument } = head {
        spine.push(argument.as_ref());
        head = function.as_ref();
    }
    spine.reverse();
    let Term::Global { id } = head else {
        return None;
    };
    if id != &rule.owner || spine.len() != rule.arity {
        return None;
    }
    let scrutinee = rule.arity.checked_sub(1)?;
    let Term::Global { id: constructor } = spine[scrutinee] else {
        return None;
    };
    if constructor != &rule.constructor {
        return None;
    }
    let images: Vec<Term> = spine[..scrutinee]
        .iter()
        .map(|argument| (*argument).clone())
        .collect();
    apply_images(&rule.right, &images, 0)
}

/// Termination (acyclicity with longest-path ranks) and confluence
/// (unique reachable normal form per node) over the finite graph.
#[allow(clippy::type_complexity)]
fn analyze_termination_and_confluence(
    nodes: &[RewriteNodeV3],
    edges: &[RewriteEdgeV3],
) -> Result<
    (
        BTreeMap<RewriteNodeIdV3, u32>,
        BTreeMap<RewriteNodeIdV3, RewriteNodeIdV3>,
    ),
    RewriteAuthorityFailureV3,
> {
    let mut adjacency: BTreeMap<&RewriteNodeIdV3, BTreeSet<&RewriteNodeIdV3>> = nodes
        .iter()
        .map(|node| (&node.id, BTreeSet::new()))
        .collect();
    let mut indegree: BTreeMap<&RewriteNodeIdV3, usize> =
        nodes.iter().map(|node| (&node.id, 0)).collect();
    for edge in edges {
        let targets = adjacency
            .get_mut(&edge.source)
            .ok_or(RewriteAuthorityFailureV3::NotConfluent)?;
        if targets.insert(&edge.target) {
            *indegree
                .get_mut(&edge.target)
                .ok_or(RewriteAuthorityFailureV3::NotConfluent)? += 1;
        }
    }
    let mut ready: BTreeSet<&RewriteNodeIdV3> = indegree
        .iter()
        .filter(|(_, degree)| **degree == 0)
        .map(|(node, _)| *node)
        .collect();
    let mut topological = Vec::with_capacity(nodes.len());
    while let Some(node) = ready.iter().next().copied() {
        ready.remove(node);
        topological.push(node);
        for target in &adjacency[node] {
            let degree = indegree
                .get_mut(target)
                .ok_or(RewriteAuthorityFailureV3::NotConfluent)?;
            *degree -= 1;
            if *degree == 0 {
                ready.insert(target);
            }
        }
    }
    if topological.len() != nodes.len() {
        return Err(RewriteAuthorityFailureV3::NonTerminating);
    }
    let mut ranks: BTreeMap<RewriteNodeIdV3, u32> = BTreeMap::new();
    let mut normal_forms: BTreeMap<RewriteNodeIdV3, RewriteNodeIdV3> = BTreeMap::new();
    for node in topological.iter().rev() {
        let targets = &adjacency[*node];
        if targets.is_empty() {
            ranks.insert((*node).clone(), 0);
            normal_forms.insert((*node).clone(), (*node).clone());
            continue;
        }
        let mut rank = 0_u32;
        let mut reachable = BTreeSet::new();
        for target in targets {
            let target_rank = *ranks
                .get(*target)
                .ok_or(RewriteAuthorityFailureV3::NotConfluent)?;
            rank = rank.max(target_rank.checked_add(1).ok_or(
                RewriteAuthorityFailureV3::ResourceExhausted {
                    stage: "termination ranks",
                },
            )?);
            reachable.insert(
                normal_forms
                    .get(*target)
                    .cloned()
                    .ok_or(RewriteAuthorityFailureV3::NotConfluent)?,
            );
        }
        let reachable: Vec<_> = reachable.into_iter().collect();
        let [normal_form] = reachable.as_slice() else {
            return Err(RewriteAuthorityFailureV3::NotConfluent);
        };
        ranks.insert((*node).clone(), rank);
        normal_forms.insert((*node).clone(), normal_form.clone());
    }
    Ok((ranks, normal_forms))
}

struct WitnessKeyMaterial<'a> {
    source: &'a DependentContext,
    target: &'a DependentContext,
    images: &'a [Term],
}

impl CanonicalEncode for WitnessKeyMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.source.encode_canonical(encoder);
        self.target.encode_canonical(encoder);
        encoder.u64(self.images.len() as u64);
        for image in self.images {
            image.encode_canonical(encoder);
        }
    }
}

struct PredecessorMaterial<'a> {
    inventory: &'a Digest,
    boundary: &'a Digest,
    historical: &'a Digest,
    nodes: &'a BTreeSet<RewriteNodeIdV3>,
    edges: &'a BTreeSet<RewriteEdgeIdV3>,
    normal_forms: &'a BTreeMap<RewriteNodeIdV3, RewriteNodeIdV3>,
    disposition_count: u64,
}

impl CanonicalEncode for PredecessorMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.boundary.encode_canonical(encoder);
        self.historical.encode_canonical(encoder);
        encoder.u64(self.nodes.len() as u64);
        for node in self.nodes {
            node.encode_canonical(encoder);
        }
        encoder.u64(self.edges.len() as u64);
        for edge in self.edges {
            edge.encode_canonical(encoder);
        }
        encoder.u64(self.normal_forms.len() as u64);
        for (node, normal_form) in self.normal_forms {
            node.encode_canonical(encoder);
            normal_form.encode_canonical(encoder);
        }
        encoder.u64(self.disposition_count);
    }
}

struct RepresentationMaterial<'a> {
    node_count: u64,
    kernel_protocol: &'a Digest,
}

impl CanonicalEncode for RepresentationMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u64(self.node_count);
        self.kernel_protocol.encode_canonical(encoder);
    }
}

struct RankMaterial<'a> {
    ranks: &'a BTreeMap<RewriteNodeIdV3, u32>,
}

impl CanonicalEncode for RankMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u64(self.ranks.len() as u64);
        for (node, rank) in self.ranks {
            node.encode_canonical(encoder);
            encoder.u32(*rank);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pen_kernel::{Declaration, KernelLimits, UncheckedSignature};

    struct PredecessorDeltaFixture {
        kernel: Kernel,
        signature: VerifiedSignature,
        declaration: GlobalId,
        source: RewriteNodeV3,
        target: RewriteNodeV3,
        nodes: Vec<RewriteNodeV3>,
        edge: RewriteEdgeV3,
        normal_forms: BTreeMap<RewriteNodeIdV3, RewriteNodeIdV3>,
    }

    fn predecessor_delta_fixture() -> PredecessorDeltaFixture {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let declaration = GlobalId(Digest::of_bytes(b"rewrite-authority/predecessor-delta"));
        let signature = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: declaration.clone(),
                    ty: Term::UnitType,
                    body: Some(Term::Unit),
                }],
            })
            .expect("predecessor signature");
        let judgment = RewriteNodeJudgmentV3::HasType { ty: Term::UnitType };
        let source = make_node(
            DependentContext::default(),
            Term::Global {
                id: declaration.clone(),
            },
            judgment.clone(),
        );
        let target = make_node(DependentContext::default(), Term::Unit, judgment);
        let edge = make_edge(
            source.id.clone(),
            target.id.clone(),
            RewritePositionV3::default(),
            RewriteRuleV3::PublicDelta {
                declaration: declaration.clone(),
            },
        );
        let normal_forms = BTreeMap::from([
            (source.id.clone(), target.id.clone()),
            (target.id.clone(), target.id.clone()),
        ]);
        PredecessorDeltaFixture {
            kernel,
            signature,
            declaration,
            source: source.clone(),
            target: target.clone(),
            nodes: vec![source, target],
            edge,
            normal_forms,
        }
    }

    fn reconstruct_fixture(
        fixture: &PredecessorDeltaFixture,
        nodes: &[RewriteNodeV3],
        edges: &[RewriteEdgeV3],
        normal_forms: &BTreeMap<RewriteNodeIdV3, RewriteNodeIdV3>,
    ) -> Result<PredecessorReconstructionV3, RewriteAuthorityFailureV3> {
        verify_predecessor_conservativity_v3(
            &fixture.kernel,
            &fixture.signature,
            &Digest::of_bytes(b"rewrite-authority/test-inventory"),
            &Digest::of_bytes(b"rewrite-authority/test-historical"),
            nodes,
            edges,
            normal_forms,
            fixture.kernel.limits().max_depth as usize,
            1_024,
        )
    }

    fn unit_lambda_redex() -> Term {
        Term::Apply {
            function: Box::new(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Var { index: 0 }),
            }),
            argument: Box::new(Term::Unit),
        }
    }

    #[test]
    fn predecessor_reconstruction_replays_the_exact_delta_edge() {
        let fixture = predecessor_delta_fixture();
        let reconstruction = reconstruct_fixture(
            &fixture,
            &fixture.nodes,
            std::slice::from_ref(&fixture.edge),
            &fixture.normal_forms,
        )
        .expect("exact predecessor reconstruction");
        assert_eq!(reconstruction.nodes.len(), 2);
        assert_eq!(reconstruction.edges.len(), 1);
        assert!(reconstruction.edges.contains(&fixture.edge.id));
    }

    #[test]
    fn predecessor_reconstruction_rejects_a_missing_lawful_edge() {
        let fixture = predecessor_delta_fixture();
        assert!(matches!(
            reconstruct_fixture(&fixture, &fixture.nodes, &[], &fixture.normal_forms),
            Err(RewriteAuthorityFailureV3::PredecessorNotConservative)
        ));
    }

    #[test]
    fn predecessor_reconstruction_rejects_a_lawful_tag_with_the_wrong_target() {
        let fixture = predecessor_delta_fixture();
        let wrong_target = make_node(
            DependentContext::default(),
            unit_lambda_redex(),
            RewriteNodeJudgmentV3::HasType { ty: Term::UnitType },
        );
        let wrong_edge = make_edge(
            fixture.source.id.clone(),
            wrong_target.id.clone(),
            RewritePositionV3::default(),
            RewriteRuleV3::PublicDelta {
                declaration: fixture.declaration.clone(),
            },
        );
        let nodes = vec![
            fixture.source.clone(),
            fixture.target.clone(),
            wrong_target.clone(),
        ];
        let normal_forms = BTreeMap::from([
            (fixture.source.id.clone(), fixture.target.id.clone()),
            (fixture.target.id.clone(), fixture.target.id.clone()),
            (wrong_target.id.clone(), fixture.target.id.clone()),
        ]);
        assert!(matches!(
            reconstruct_fixture(&fixture, &nodes, &[wrong_edge], &normal_forms),
            Err(RewriteAuthorityFailureV3::PredecessorNotConservative)
        ));
    }

    #[test]
    fn predecessor_reconstruction_rejects_a_normal_form_outside_the_old_language() {
        let fixture = predecessor_delta_fixture();
        let successor = GlobalId(Digest::of_bytes(b"rewrite-authority/successor"));
        let successor_node = make_node(
            DependentContext::default(),
            Term::Global { id: successor },
            RewriteNodeJudgmentV3::HasType { ty: Term::UnitType },
        );
        let nodes = vec![
            fixture.source.clone(),
            fixture.target.clone(),
            successor_node.clone(),
        ];
        let normal_forms = BTreeMap::from([
            (fixture.source.id.clone(), successor_node.id.clone()),
            (fixture.target.id.clone(), fixture.target.id.clone()),
            (successor_node.id.clone(), successor_node.id.clone()),
        ]);
        assert!(matches!(
            reconstruct_fixture(
                &fixture,
                &nodes,
                std::slice::from_ref(&fixture.edge),
                &normal_forms,
            ),
            Err(RewriteAuthorityFailureV3::PredecessorNotConservative)
        ));
    }

    #[test]
    fn subst_top_contracts_the_identity_redex() {
        let Term::Apply { function, argument } = unit_lambda_redex() else {
            panic!("redex");
        };
        let Term::Lambda { body, .. } = *function else {
            panic!("lambda");
        };
        assert_eq!(subst_top(&body, &argument), Some(Term::Unit));
    }

    #[test]
    fn subst_top_shifts_escaping_variables_down() {
        // (lambda. Var1) applied under one outer binder: body Var1 refers
        // to the context variable and must become Var0 after contraction.
        let body = Term::Var { index: 1 };
        assert_eq!(subst_top(&body, &Term::Unit), Some(Term::Var { index: 0 }));
    }

    #[test]
    fn apply_images_uses_oldest_first_ordinals_and_weakens_under_binders() {
        // Context [a, b] oldest-first; Var0 = b, Var1 = a.
        let images = vec![Term::UnitType, Term::Unit];
        assert_eq!(
            apply_images(&Term::Var { index: 0 }, &images, 0),
            Some(Term::Unit)
        );
        assert_eq!(
            apply_images(&Term::Var { index: 1 }, &images, 0),
            Some(Term::UnitType)
        );
        // Under one binder the bound Var0 is untouched and Var1 = b.
        let under = Term::Lambda {
            parameter_type: Box::new(Term::UnitType),
            body: Box::new(Term::Apply {
                function: Box::new(Term::Var { index: 0 }),
                argument: Box::new(Term::Var { index: 1 }),
            }),
        };
        assert_eq!(
            apply_images(&under, &images, 0),
            Some(Term::Lambda {
                parameter_type: Box::new(Term::UnitType),
                body: Box::new(Term::Apply {
                    function: Box::new(Term::Var { index: 0 }),
                    argument: Box::new(Term::Unit),
                }),
            })
        );
    }

    #[test]
    fn fresh_rule_matching_requires_the_exact_spine() {
        let owner = GlobalId(Digest::of_bytes(b"rewrite-authority/owner"));
        let constructor = GlobalId(Digest::of_bytes(b"rewrite-authority/constructor"));
        let other = GlobalId(Digest::of_bytes(b"rewrite-authority/other"));
        let rule = FreshRuleV3 {
            equation: EquationIdV1(Digest::of_bytes(b"rewrite-authority/equation")),
            owner: owner.clone(),
            constructor: constructor.clone(),
            arity: 2,
            // Right side in the prefix rule context [b]: Var0 = b.
            right: Term::Var { index: 0 },
        };
        let apply = |head: &GlobalId, argument: Term, scrutinee: &GlobalId| Term::Apply {
            function: Box::new(Term::Apply {
                function: Box::new(Term::Global { id: head.clone() }),
                argument: Box::new(argument),
            }),
            argument: Box::new(Term::Global {
                id: scrutinee.clone(),
            }),
        };
        // Exact match rewrites to the bound argument.
        assert_eq!(
            match_fresh_rule(&rule, &apply(&owner, Term::Unit, &constructor)),
            Some(Term::Unit)
        );
        // Wrong owner, wrong constructor, wrong arity: no match.
        assert_eq!(
            match_fresh_rule(&rule, &apply(&other, Term::Unit, &constructor)),
            None
        );
        assert_eq!(
            match_fresh_rule(&rule, &apply(&owner, Term::Unit, &other)),
            None
        );
        assert_eq!(
            match_fresh_rule(
                &rule,
                &Term::Apply {
                    function: Box::new(Term::Global { id: owner.clone() }),
                    argument: Box::new(Term::Global {
                        id: constructor.clone()
                    }),
                }
            ),
            None
        );
    }

    #[test]
    fn positions_enumerate_every_subterm_and_replacement_is_exact() {
        let term = unit_lambda_redex();
        let positions = enumerate_positions(&term, 16).expect("positions");
        // Apply, Lambda, parameter type, body, argument.
        assert_eq!(positions.len(), 5);
        assert_eq!(positions[0].0, RewritePositionV3::default());
        let replaced = replace_at_position(&term, &[1], &Term::UnitType).expect("replace");
        assert_eq!(
            replaced,
            Term::Apply {
                function: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::UnitType),
                    body: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::UnitType),
            }
        );
    }

    #[test]
    fn nested_and_disjoint_position_relations_classify_exactly() {
        let root = RewritePositionV3::default();
        let left = root.child(0);
        let right = root.child(1);
        let deep = left.child(1);
        assert!(root.is_prefix_of(&left));
        assert!(left.is_prefix_of(&deep));
        assert!(!left.is_prefix_of(&right));
        assert!(!right.is_prefix_of(&left));
    }

    #[test]
    fn acyclic_graphs_terminate_and_cycles_fail_closed() {
        let node = |label: &[u8]| {
            make_node(
                DependentContext::default(),
                Term::Global {
                    id: GlobalId(Digest::of_bytes(label)),
                },
                RewriteNodeJudgmentV3::TypeFormation,
            )
        };
        let a = node(b"rewrite-authority/a");
        let b = node(b"rewrite-authority/b");
        let edge = |source: &RewriteNodeV3, target: &RewriteNodeV3| {
            make_edge(
                source.id.clone(),
                target.id.clone(),
                RewritePositionV3::default(),
                RewriteRuleV3::OrdinaryBeta,
            )
        };
        let nodes = vec![a.clone(), b.clone()];
        let (ranks, normal_forms) =
            analyze_termination_and_confluence(&nodes, &[edge(&a, &b)]).expect("acyclic");
        assert_eq!(ranks[&a.id], 1);
        assert_eq!(ranks[&b.id], 0);
        assert_eq!(normal_forms[&a.id], b.id);
        assert!(matches!(
            analyze_termination_and_confluence(&nodes, &[edge(&a, &b), edge(&b, &a)]),
            Err(RewriteAuthorityFailureV3::NonTerminating)
        ));
    }
}
