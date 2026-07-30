//! Finite, typed operational search for the one-nullary completion profile.

use crate::rewrite::{VerifiedEquationExtensionSet, map_kernel_error, shift};
use pen_demand::gsc::{
    ContextMapCode, GroundRuleDisposition, GscOutcome, GscRule, GscUnknownReason,
    OperationalDerivabilityGrammarV1, OperationalGoalUniverse, OperationalSaturationOrder,
    OutputClause, PortKey, PortRef, VerifiedCanonicalDemandFamilyV2, VerifiedGscSemanticManifest,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Kernel, KernelError,
    OpenJudgment, Term, VerifiedSignature,
};
use std::collections::{BTreeMap, BTreeSet};

/// The complete rule inventory of the frozen finite operational grammar.
pub use pen_demand::gsc::OperationalDerivationRule as OperationalRule;

pub const ORDERED_OPERATIONAL_RULES: [OperationalRule; 11] = [
    OperationalRule::PublicExact,
    OperationalRule::Q0Conversion,
    OperationalRule::ContextWeakening,
    OperationalRule::CheckedSubstitution,
    OperationalRule::LambdaIntroduction,
    OperationalRule::Application,
    OperationalRule::PairIntroduction,
    OperationalRule::FirstProjection,
    OperationalRule::SecondProjection,
    OperationalRule::EquationReplay,
    OperationalRule::QuotientTransport,
];

/// One checked node in a finite derivation DAG.
#[derive(Clone, Debug)]
pub struct VerifiedDerivationNode {
    ordinal: u32,
    context: DependentContext,
    term: Term,
    ty: Term,
    rule: OperationalRule,
    premises: Vec<u32>,
}

impl VerifiedDerivationNode {
    pub fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub fn context(&self) -> &DependentContext {
        &self.context
    }

    pub fn term(&self) -> &Term {
        &self.term
    }

    pub fn ty(&self) -> &Term {
        &self.ty
    }

    pub fn rule(&self) -> OperationalRule {
        self.rule
    }

    pub fn premises(&self) -> &[u32] {
        &self.premises
    }
}

/// Opaque, kernel-replayed derivation evidence.
#[derive(Clone, Debug)]
pub struct VerifiedDerivationDag {
    nodes: Vec<VerifiedDerivationNode>,
    conclusion: u32,
    digest: Digest,
}

impl VerifiedDerivationDag {
    pub fn nodes(&self) -> &[VerifiedDerivationNode] {
        &self.nodes
    }

    pub fn conclusion(&self) -> &VerifiedDerivationNode {
        &self.nodes[self.conclusion as usize]
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub(crate) fn contains_global(&self, target: &GlobalId) -> bool {
        self.nodes
            .iter()
            .any(|node| term_contains_global(&node.term, target))
    }
}

/// Rule-specific reason shared by the certified-inapplicable groundings in
/// one finite saturation run.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CertifiedInapplicabilityReason {
    NormalFormsDiffer,
    GoalIsNotDependentFunction,
    PrincipalIsNotDependentFunction,
    ArgumentTypeMismatch,
    GoalIsNotDependentPair,
    PrincipalIsNotDependentPair,
    NoBodyInFiniteLibrary,
    NoEquationAvailableBeforeResponse,
    NoQuotientClassAvailableBeforeResponse,
}

/// Reason-specific evidence for a disjoint set of grounded rule instances.
#[derive(Clone, Debug)]
pub struct VerifiedInapplicabilityEvidence {
    reason: CertifiedInapplicabilityReason,
    instances: u32,
    grounding_set_digest: Digest,
}

impl VerifiedInapplicabilityEvidence {
    pub fn reason(&self) -> CertifiedInapplicabilityReason {
        self.reason
    }

    pub fn instances(&self) -> u32 {
        self.instances
    }

    pub fn grounding_set_digest(&self) -> &Digest {
        &self.grounding_set_digest
    }
}

/// Exhaustive disposition counts for every grounding of one operational rule.
#[derive(Clone, Debug)]
pub struct VerifiedGroundRuleDisposition {
    rule: OperationalRule,
    grounded_instances: u32,
    applicable_instances: u32,
    certified_inapplicable_instances: u32,
    inapplicability_evidence: Vec<VerifiedInapplicabilityEvidence>,
}

impl VerifiedGroundRuleDisposition {
    pub fn rule(&self) -> OperationalRule {
        self.rule
    }

    pub fn grounded_instances(&self) -> u32 {
        self.grounded_instances
    }

    pub fn applicable_instances(&self) -> u32 {
        self.applicable_instances
    }

    pub fn certified_inapplicable_instances(&self) -> u32 {
        self.certified_inapplicable_instances
    }

    pub fn reason(&self) -> Option<CertifiedInapplicabilityReason> {
        let [evidence] = self.inapplicability_evidence.as_slice() else {
            return None;
        };
        Some(evidence.reason())
    }

    pub fn inapplicability_evidence(&self) -> &[VerifiedInapplicabilityEvidence] {
        &self.inapplicability_evidence
    }

    pub fn is_exhaustively_decided(&self) -> bool {
        self.grounded_instances
            == self
                .applicable_instances
                .saturating_add(self.certified_inapplicable_instances)
    }
}

/// Canonical-form argument that closes the gap between finite neutral
/// saturation and a negative use-port decision.
#[derive(Clone, Debug)]
pub struct VerifiedCanonicalFormAnalysis {
    single_outer_function: bool,
    stuck_atomic_body: bool,
    neutral_heads_exhausted: bool,
    higher_order_argument_gap_absent: bool,
    introduction_rules_separated: bool,
    digest: Digest,
}

impl VerifiedCanonicalFormAnalysis {
    pub fn has_single_outer_function(&self) -> bool {
        self.single_outer_function
    }

    pub fn has_stuck_atomic_body(&self) -> bool {
        self.stuck_atomic_body
    }

    pub fn neutral_heads_are_exhausted(&self) -> bool {
        self.neutral_heads_exhausted
    }

    pub fn has_no_higher_order_argument_gap(&self) -> bool {
        self.higher_order_argument_gap_absent
    }

    pub fn introduction_rules_are_separated(&self) -> bool {
        self.introduction_rules_separated
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Complete negative decision relative to the exact finite normal-form
/// library implemented by this module.
#[derive(Clone, Debug)]
pub struct VerifiedUnderivedUse {
    port: PortKey,
    context: DependentContext,
    target: Term,
    examined_terms: u32,
    dispositions: Vec<VerifiedGroundRuleDisposition>,
    canonical_forms: VerifiedCanonicalFormAnalysis,
    grounded_rule_instances: u32,
    structural_goal_count: u32,
    library_nodes: u32,
    closure_base_digest: Digest,
    closure_frontier_digests: Vec<Digest>,
    closure_fixed_point_digest: Digest,
    goal_rule_coverage_digest: Digest,
    canonical_replay_digest: Digest,
    grounding_inventory_digest: Digest,
    library_inventory_digest: Digest,
    grammar_digest: Digest,
    digest: Digest,
}

impl VerifiedUnderivedUse {
    pub fn port(&self) -> &PortKey {
        &self.port
    }

    pub fn context(&self) -> &DependentContext {
        &self.context
    }

    pub fn target(&self) -> &Term {
        &self.target
    }

    pub fn examined_terms(&self) -> u32 {
        self.examined_terms
    }

    pub fn dispositions(&self) -> &[VerifiedGroundRuleDisposition] {
        &self.dispositions
    }

    pub fn canonical_forms(&self) -> &VerifiedCanonicalFormAnalysis {
        &self.canonical_forms
    }

    pub fn grounded_rule_instances(&self) -> u32 {
        self.grounded_rule_instances
    }

    pub fn structural_goal_count(&self) -> u32 {
        self.structural_goal_count
    }

    pub fn library_nodes(&self) -> u32 {
        self.library_nodes
    }

    pub fn closure_base_digest(&self) -> &Digest {
        &self.closure_base_digest
    }

    pub fn closure_frontier_digests(&self) -> &[Digest] {
        &self.closure_frontier_digests
    }

    pub fn closure_fixed_point_digest(&self) -> &Digest {
        &self.closure_fixed_point_digest
    }

    pub fn goal_rule_coverage_digest(&self) -> &Digest {
        &self.goal_rule_coverage_digest
    }

    pub fn canonical_replay_digest(&self) -> &Digest {
        &self.canonical_replay_digest
    }

    pub fn grounding_inventory_digest(&self) -> &Digest {
        &self.grounding_inventory_digest
    }

    pub fn library_inventory_digest(&self) -> &Digest {
        &self.library_inventory_digest
    }

    pub fn is_complete_relative_to_grammar(&self) -> bool {
        self.dispositions.len() == ORDERED_OPERATIONAL_RULES.len()
            && self
                .dispositions
                .iter()
                .zip(ORDERED_OPERATIONAL_RULES)
                .all(|(disposition, rule)| {
                    disposition.rule() == rule && disposition.is_exhaustively_decided()
                })
            && self.canonical_forms.has_single_outer_function()
            && self.canonical_forms.has_stuck_atomic_body()
            && self.canonical_forms.neutral_heads_are_exhausted()
            && self.canonical_forms.has_no_higher_order_argument_gap()
            && self.canonical_forms.introduction_rules_are_separated()
            && self.structural_goal_count == 2
            && !self.closure_frontier_digests.is_empty()
    }

    pub fn grammar_digest(&self) -> &Digest {
        &self.grammar_digest
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Positive use-port decision.
#[derive(Clone, Debug)]
pub struct VerifiedDerivedUse {
    port: PortKey,
    context: DependentContext,
    target: Term,
    term: Term,
    derivation: VerifiedDerivationDag,
    digest: Digest,
}

impl VerifiedDerivedUse {
    pub fn port(&self) -> &PortKey {
        &self.port
    }

    pub fn context(&self) -> &DependentContext {
        &self.context
    }

    pub fn target(&self) -> &Term {
        &self.target
    }

    pub fn term(&self) -> &Term {
        &self.term
    }

    pub fn derivation(&self) -> &VerifiedDerivationDag {
        &self.derivation
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug)]
pub(crate) enum OneNullaryUseDecision {
    Derived(VerifiedDerivedUse),
    Underived(VerifiedUnderivedUse),
}

/// One output-port discharge in the response DAG.
#[derive(Clone, Debug)]
pub struct VerifiedPortDischarge {
    port: PortKey,
    rule: OperationalRule,
    premises: Vec<PortKey>,
    evidence_digest: Digest,
}

impl VerifiedPortDischarge {
    pub fn port(&self) -> &PortKey {
        &self.port
    }

    pub fn rule(&self) -> OperationalRule {
        self.rule
    }

    pub fn premises(&self) -> &[PortKey] {
        &self.premises
    }

    pub fn evidence_digest(&self) -> &Digest {
        &self.evidence_digest
    }
}

/// Opaque evidence that every output is filled in one coherent environment
/// and every generated positive-cost clause lies on an output path.
#[derive(Clone, Debug)]
pub struct VerifiedDemandConnectedResponse {
    use_derivation: VerifiedDerivedUse,
    equation_set_digest: Digest,
    discharges: Vec<VerifiedPortDischarge>,
    all_outputs_filled: bool,
    all_positive_clauses_connected: bool,
    digest: Digest,
}

impl VerifiedDemandConnectedResponse {
    pub fn use_derivation(&self) -> &VerifiedDerivedUse {
        &self.use_derivation
    }

    pub fn equation_set_digest(&self) -> &Digest {
        &self.equation_set_digest
    }

    pub fn discharges(&self) -> &[VerifiedPortDischarge] {
        &self.discharges
    }

    pub fn all_outputs_filled(&self) -> bool {
        self.all_outputs_filled
    }

    pub fn all_positive_clauses_connected(&self) -> bool {
        self.all_positive_clauses_connected
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug)]
struct SearchNode {
    context: DependentContext,
    term: Term,
    ty: Term,
    rule: OperationalRule,
    premises: Vec<usize>,
}

#[derive(Debug)]
struct SearchState<'a> {
    kernel: &'a Kernel,
    signature: &'a VerifiedSignature,
    grammar: &'a OperationalDerivabilityGrammarV1,
    probe_cache: BTreeMap<Digest, Option<OpenJudgment>>,
    normalization_cache: BTreeMap<Digest, Term>,
}

#[derive(Clone, Debug)]
struct StructuralGoal {
    context: DependentContext,
    target: Term,
    digest: Digest,
}

#[derive(Clone, Debug)]
struct GoalLibrary {
    goal: StructuralGoal,
    nodes: Vec<SearchNode>,
    base_digest: Digest,
    fixed_point_digest: Digest,
}

#[derive(Clone, Debug)]
struct GoalLibraryBuilder {
    goal: StructuralGoal,
    nodes: Vec<SearchNode>,
    base_digest: Digest,
    frontier_digests: Vec<Digest>,
    seen: BTreeSet<Digest>,
    fixed: bool,
}

#[derive(Clone, Debug)]
struct PositiveDerivationFrontier {
    goal: StructuralGoal,
    nodes: Vec<SearchNode>,
    exact_index: usize,
    frontier_digests: Vec<Digest>,
    digest: Digest,
}

enum ClosureSearchOutcome {
    ExactOuter(PositiveDerivationFrontier),
    ExactBody(PositiveDerivationFrontier),
    Complete(ClosureAudit),
}

#[derive(Clone, Debug)]
struct ClosureAudit {
    libraries: Vec<GoalLibrary>,
    structural_goal_count: u32,
    library_nodes: u32,
    base_digest: Digest,
    frontier_digests: Vec<Digest>,
    fixed_point_digest: Digest,
    library_inventory_digest: Digest,
    unsupported_introduction_argument_seen: bool,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct GroundingKey {
    structural_goal_digest: Digest,
    rule: OperationalRule,
    premise_digest: Digest,
    conclusion_digest: Digest,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum GroundDecision {
    Applicable,
    CertifiedInapplicable(CertifiedInapplicabilityReason),
}

#[derive(Clone, Debug)]
enum GroundingWitness {
    TypedNode {
        goal: StructuralGoal,
        node: SearchNode,
    },
    Q0 {
        goal: StructuralGoal,
        node: SearchNode,
    },
    Application {
        goal: StructuralGoal,
        function: SearchNode,
        argument: SearchNode,
    },
    Projection {
        goal: StructuralGoal,
        principal: SearchNode,
        first: bool,
    },
    Lambda {
        goal: StructuralGoal,
        body_goal: Option<StructuralGoal>,
        body_node: Option<SearchNode>,
    },
    GoalShape {
        goal: StructuralGoal,
        reason: CertifiedInapplicabilityReason,
    },
    EmptyPreResponseRegistry {
        goal: StructuralGoal,
        reason: CertifiedInapplicabilityReason,
    },
}

struct FinalizedGroundingAudit {
    dispositions: Vec<VerifiedGroundRuleDisposition>,
    grounded_rule_instances: u32,
    structural_goal_count: u32,
    library_nodes: u32,
    closure_base_digest: Digest,
    closure_frontier_digests: Vec<Digest>,
    closure_fixed_point_digest: Digest,
    goal_rule_coverage_digest: Digest,
    canonical_replay_digest: Digest,
    grounding_inventory_digest: Digest,
    library_inventory_digest: Digest,
}

impl SearchState<'_> {
    fn probe(
        &mut self,
        context: &DependentContext,
        term: &Term,
        ty: &Term,
    ) -> Result<Option<OpenJudgment>, GscUnknownReason> {
        let key = probe_request_digest(self.signature, context, term, ty);
        if let Some(cached) = self.probe_cache.get(&key) {
            return Ok(cached.clone());
        }
        if self.probe_cache.len() >= self.grammar.max_grounded_rule_instances as usize {
            return Err(GscUnknownReason::ResourceExhausted);
        }
        let judgment = OpenJudgment::HasType {
            context: context.clone(),
            term: term.clone(),
            ty: ty.clone(),
        };
        let result = match self.kernel.verify_open_judgment(self.signature, &judgment) {
            Ok(normalized) => Some(normalized),
            Err(KernelError::ResourceExhausted(_)) => {
                return Err(GscUnknownReason::ResourceExhausted);
            }
            Err(KernelError::TypeMismatch) => None,
            Err(_) => return Err(GscUnknownReason::KernelCouldNotCertify),
        };
        self.probe_cache.insert(key, result.clone());
        Ok(result)
    }

    fn normalize_type(
        &mut self,
        context: &DependentContext,
        ty: &Term,
    ) -> Result<Term, GscUnknownReason> {
        let key = normalization_request_digest(self.signature, context, ty);
        if let Some(cached) = self.normalization_cache.get(&key) {
            return Ok(cached.clone());
        }
        if self.normalization_cache.len() >= self.grammar.max_grounded_rule_instances as usize {
            return Err(GscUnknownReason::ResourceExhausted);
        }
        let judgment = OpenJudgment::TypeFormation {
            context: context.clone(),
            term: ty.clone(),
        };
        let normalized = match self.kernel.verify_open_judgment(self.signature, &judgment) {
            Ok(OpenJudgment::TypeFormation { term, .. }) => term,
            Err(error) => return Err(map_kernel_error(error)),
            _ => return Err(GscUnknownReason::KernelCouldNotCertify),
        };
        self.normalization_cache.insert(key, normalized.clone());
        Ok(normalized)
    }

    fn examined_requests(&self) -> Result<u32, GscUnknownReason> {
        u32::try_from(
            self.probe_cache
                .len()
                .saturating_add(self.normalization_cache.len()),
        )
        .map_err(|_| GscUnknownReason::ResourceExhausted)
    }
}

pub(crate) fn decide_one_nullary_use(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    semantic: &VerifiedGscSemanticManifest,
    family: &VerifiedCanonicalDemandFamilyV2,
) -> GscOutcome<OneNullaryUseDecision> {
    if family.rule() != GscRule::Use
        || family.rank() != 1
        || family.ports().len() != 1
        || !family.premise_refs().is_empty()
    {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }
    let grammar = &semantic.manifest().derivability_grammar;
    if grammar.goal_universe
        != OperationalGoalUniverse::PortsVerificationPublicCompilerRegistryAndStructuralSubgoalsV1
        || grammar.ordered_rules != ORDERED_OPERATIONAL_RULES
        || grammar.ground_rule_disposition
            != GroundRuleDisposition::ApplicableCertifiedInapplicableOrUnknownV1
        || grammar.saturation_order
            != OperationalSaturationOrder::GoalDigestThenRuleInventoryThenPremiseDigestV1
        || grammar.max_goals == 0
        || grammar.max_grounded_rule_instances == 0
        || grammar.max_rule_premises < 2
        || grammar.max_fixed_point_rounds == 0
    {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedManifest);
    }
    let port = &family.ports()[0];
    let OutputClause::TermPort {
        context, motive, ..
    } = port.clause()
    else {
        return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
    };
    if context != family.parameter_context() {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }

    let mut state = SearchState {
        kernel,
        signature,
        grammar,
        probe_cache: BTreeMap::new(),
        normalization_cache: BTreeMap::new(),
    };
    let target = match state.normalize_type(context, motive) {
        Ok(target) => target,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    if !is_exact_one_nullary_use_goal(context, &target) {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
    }
    let (outer_goal, structural_goals) = match exact_structural_goals(&mut state, context, &target)
    {
        Ok(goals) => goals,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    let closure = match search_or_complete_closure(&mut state, &outer_goal, structural_goals) {
        Ok(ClosureSearchOutcome::ExactOuter(positive)) => {
            return derived_from_outer_frontier(port.key(), context, target, positive);
        }
        Ok(ClosureSearchOutcome::ExactBody(positive)) => {
            return derived_from_body_frontier(
                &mut state,
                port.key(),
                context,
                target,
                &outer_goal,
                positive,
            );
        }
        Ok(ClosureSearchOutcome::Complete(closure)) => closure,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    let audit = match replay_complete_grounding_inventory(&mut state, &closure) {
        Ok(audit) => audit,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    if closure.unsupported_introduction_argument_seen {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
    }

    let examined_terms = match state.examined_requests() {
        Ok(count) => count,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    let canonical_forms = canonical_form_analysis(
        context,
        &target,
        true,
        !closure.unsupported_introduction_argument_seen,
        audit.structural_goal_count == 2,
    );
    let grammar_digest = operational_grammar_digest(semantic);
    let digest = {
        let mut encoder = CanonicalEncoder::new();
        port.key().encode_canonical(&mut encoder);
        context.encode_canonical(&mut encoder);
        target.encode_canonical(&mut encoder);
        encoder.u32(examined_terms);
        grammar_digest.encode_canonical(&mut encoder);
        encode_dispositions(&audit.dispositions, &mut encoder);
        canonical_forms.digest().encode_canonical(&mut encoder);
        encoder.u32(audit.grounded_rule_instances);
        encoder.u32(audit.structural_goal_count);
        encoder.u32(audit.library_nodes);
        audit.closure_base_digest.encode_canonical(&mut encoder);
        encoder.u64(audit.closure_frontier_digests.len() as u64);
        for frontier in &audit.closure_frontier_digests {
            frontier.encode_canonical(&mut encoder);
        }
        audit
            .closure_fixed_point_digest
            .encode_canonical(&mut encoder);
        audit
            .goal_rule_coverage_digest
            .encode_canonical(&mut encoder);
        audit.canonical_replay_digest.encode_canonical(&mut encoder);
        audit
            .grounding_inventory_digest
            .encode_canonical(&mut encoder);
        audit
            .library_inventory_digest
            .encode_canonical(&mut encoder);
        Digest::of_domain_bytes("pen-gf2/underived-use/v1", encoder.as_bytes())
    };
    GscOutcome::Proven(OneNullaryUseDecision::Underived(VerifiedUnderivedUse {
        port: port.key().clone(),
        context: context.clone(),
        target,
        examined_terms,
        dispositions: audit.dispositions,
        canonical_forms,
        grounded_rule_instances: audit.grounded_rule_instances,
        structural_goal_count: audit.structural_goal_count,
        library_nodes: audit.library_nodes,
        closure_base_digest: audit.closure_base_digest,
        closure_frontier_digests: audit.closure_frontier_digests,
        closure_fixed_point_digest: audit.closure_fixed_point_digest,
        goal_rule_coverage_digest: audit.goal_rule_coverage_digest,
        canonical_replay_digest: audit.canonical_replay_digest,
        grounding_inventory_digest: audit.grounding_inventory_digest,
        library_inventory_digest: audit.library_inventory_digest,
        grammar_digest,
        digest,
    }))
}

fn derived_from_outer_frontier(
    port: &PortKey,
    context: &DependentContext,
    target: Term,
    positive: PositiveDerivationFrontier,
) -> GscOutcome<OneNullaryUseDecision> {
    if positive.goal.context != *context
        || positive.goal.target != target
        || positive.frontier_digests.is_empty()
    {
        return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
    }
    let Some(node) = positive.nodes.get(positive.exact_index) else {
        return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
    };
    let dag = relevant_dag(&positive.nodes, positive.exact_index);
    let digest = {
        let mut encoder = CanonicalEncoder::new();
        port.encode_canonical(&mut encoder);
        context.encode_canonical(&mut encoder);
        target.encode_canonical(&mut encoder);
        node.term.encode_canonical(&mut encoder);
        dag.digest().encode_canonical(&mut encoder);
        positive.digest.encode_canonical(&mut encoder);
        Digest::of_domain_bytes("pen-gf2/derived-use/v1", encoder.as_bytes())
    };
    GscOutcome::Proven(OneNullaryUseDecision::Derived(VerifiedDerivedUse {
        port: port.clone(),
        context: context.clone(),
        target,
        term: node.term.clone(),
        derivation: dag,
        digest,
    }))
}

fn derived_from_body_frontier(
    state: &mut SearchState<'_>,
    port: &PortKey,
    context: &DependentContext,
    target: Term,
    outer_goal: &StructuralGoal,
    positive: PositiveDerivationFrontier,
) -> GscOutcome<OneNullaryUseDecision> {
    if positive.goal.context == *context || positive.frontier_digests.is_empty() {
        return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
    }
    let Term::Pi { parameter, .. } = &outer_goal.target else {
        return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
    };
    let Some(body_node) = positive.nodes.get(positive.exact_index) else {
        return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
    };
    let lambda = Term::Lambda {
        parameter_type: parameter.clone(),
        body: Box::new(body_node.term.clone()),
    };
    match state.probe(context, &lambda, &target) {
        Ok(Some(_)) => {}
        Ok(None) | Err(GscUnknownReason::KernelCouldNotCertify) => {
            return GscOutcome::Unknown(GscUnknownReason::KernelCouldNotCertify);
        }
        Err(reason) => return GscOutcome::Unknown(reason),
    }
    let mut dag = relevant_dag(&positive.nodes, positive.exact_index);
    let premise = dag.conclusion;
    let ordinal = match u32::try_from(dag.nodes.len()) {
        Ok(ordinal) => ordinal,
        Err(_) => return GscOutcome::Unknown(GscUnknownReason::ResourceExhausted),
    };
    dag.nodes.push(VerifiedDerivationNode {
        ordinal,
        context: context.clone(),
        term: lambda.clone(),
        ty: target.clone(),
        rule: OperationalRule::LambdaIntroduction,
        premises: vec![premise],
    });
    dag.conclusion = ordinal;
    dag.digest = derivation_digest(&dag.nodes, ordinal);
    let digest = {
        let mut encoder = CanonicalEncoder::new();
        port.encode_canonical(&mut encoder);
        context.encode_canonical(&mut encoder);
        target.encode_canonical(&mut encoder);
        lambda.encode_canonical(&mut encoder);
        dag.digest().encode_canonical(&mut encoder);
        positive.digest.encode_canonical(&mut encoder);
        Digest::of_domain_bytes("pen-gf2/derived-use/v1", encoder.as_bytes())
    };
    GscOutcome::Proven(OneNullaryUseDecision::Derived(VerifiedDerivedUse {
        port: port.clone(),
        context: context.clone(),
        target,
        term: lambda,
        derivation: dag,
        digest,
    }))
}

fn exact_structural_goals(
    state: &mut SearchState<'_>,
    context: &DependentContext,
    target: &Term,
) -> Result<(StructuralGoal, Vec<StructuralGoal>), GscUnknownReason> {
    let Term::Pi { parameter, body } = target else {
        return Err(GscUnknownReason::UnsupportedCode);
    };
    let outer = StructuralGoal {
        context: context.clone(),
        target: target.clone(),
        digest: operational_goal_digest(context, target),
    };
    let mut body_context = context.clone();
    body_context.0.push((**parameter).clone());
    let body_target = state.normalize_type(&body_context, body)?;
    let body_goal = StructuralGoal {
        digest: operational_goal_digest(&body_context, &body_target),
        context: body_context,
        target: body_target,
    };
    if outer.digest == body_goal.digest {
        return Err(GscUnknownReason::KernelCouldNotCertify);
    }
    let mut goals = vec![outer.clone(), body_goal];
    goals.sort_by(|left, right| left.digest.cmp(&right.digest));
    if goals.len() > state.grammar.max_goals as usize {
        return Err(GscUnknownReason::ResourceExhausted);
    }
    Ok((outer, goals))
}

fn search_or_complete_closure(
    state: &mut SearchState<'_>,
    outer_goal: &StructuralGoal,
    mut goals: Vec<StructuralGoal>,
) -> Result<ClosureSearchOutcome, GscUnknownReason> {
    goals.sort_by(|left, right| left.digest.cmp(&right.digest));
    if goals.len() != 2 || !goals.iter().any(|goal| goal.digest == outer_goal.digest) {
        return Err(GscUnknownReason::KernelCouldNotCertify);
    }
    let mut builders = Vec::with_capacity(goals.len());
    for goal in goals {
        let occupied = builders
            .iter()
            .map(|builder: &GoalLibraryBuilder| builder.nodes.len())
            .sum::<usize>();
        let remaining = (state.grammar.max_goals as usize)
            .checked_sub(occupied)
            .ok_or(GscUnknownReason::ResourceExhausted)?;
        if remaining == 0 {
            return Err(GscUnknownReason::ResourceExhausted);
        }
        builders.push(initialize_goal_library(state, goal, remaining)?);
    }
    let base_frontiers = builders
        .iter()
        .map(|builder| builder.nodes.clone())
        .collect::<Vec<_>>();
    let mut global_frontier_digests = vec![global_frontier_digest(0, &builders, &base_frontiers)];
    if let Some(positive) =
        select_canonical_positive(state, outer_goal, &builders, &global_frontier_digests)?
    {
        return Ok(positive);
    }
    for round in 1..=state.grammar.max_fixed_point_rounds {
        if application_grounding_lower_bound(&builders)?
            > state.grammar.max_grounded_rule_instances as usize
        {
            return Err(GscUnknownReason::ResourceExhausted);
        }
        let round_start_nodes = builders
            .iter()
            .map(|builder| builder.nodes.len())
            .sum::<usize>();
        let mut round_frontiers = vec![Vec::new(); builders.len()];
        let mut reserved_frontier_nodes = 0_usize;
        for index in 0..builders.len() {
            if builders[index].fixed {
                continue;
            }
            let other_nodes = builders
                .iter()
                .enumerate()
                .filter(|(other, _)| *other != index)
                .map(|(_, builder)| builder.nodes.len())
                .sum::<usize>();
            let max_for_goal = (state.grammar.max_goals as usize)
                .checked_sub(other_nodes)
                .and_then(|remaining| remaining.checked_sub(reserved_frontier_nodes))
                .ok_or(GscUnknownReason::ResourceExhausted)?;
            let frontier = advance_goal_library(state, &builders[index], round, max_for_goal)?;
            reserved_frontier_nodes = reserved_frontier_nodes
                .checked_add(frontier.len())
                .ok_or(GscUnknownReason::ResourceExhausted)?;
            round_frontiers[index] = frontier;
        }
        global_frontier_digests.push(global_frontier_digest(round, &builders, &round_frontiers));
        let mut added_any = false;
        for (index, frontier) in round_frontiers.into_iter().enumerate() {
            if builders[index].fixed {
                continue;
            }
            if frontier.is_empty() {
                let empty_digest = frontier_digest(&builders[index].goal, round, &[]);
                builders[index].frontier_digests.push(empty_digest);
                builders[index].fixed = true;
                continue;
            }
            added_any = true;
            let local_digest = frontier_digest(&builders[index].goal, round, &frontier);
            builders[index].frontier_digests.push(local_digest);
            for node in &frontier {
                builders[index].seen.insert(search_node_digest(node));
            }
            builders[index].nodes.extend(frontier);
        }
        let round_end_nodes = builders
            .iter()
            .map(|builder| builder.nodes.len())
            .sum::<usize>();
        if round_end_nodes > state.grammar.max_goals as usize || round_end_nodes < round_start_nodes
        {
            return Err(GscUnknownReason::ResourceExhausted);
        }
        if let Some(positive) =
            select_canonical_positive(state, outer_goal, &builders, &global_frontier_digests)?
        {
            return Ok(positive);
        }
        if builders.iter().all(|builder| builder.fixed) {
            break;
        }
        if !added_any || round == state.grammar.max_fixed_point_rounds {
            return Err(GscUnknownReason::ResourceExhausted);
        }
    }
    if !builders.iter().all(|builder| builder.fixed) {
        return Err(GscUnknownReason::ResourceExhausted);
    }
    let mut libraries = builders
        .into_iter()
        .map(finish_goal_library)
        .collect::<Vec<_>>();
    libraries.sort_by(|left, right| left.goal.digest.cmp(&right.goal.digest));

    let mut all_nodes = BTreeSet::new();
    for library in &libraries {
        for node in &library.nodes {
            all_nodes.insert(search_node_digest(node));
        }
    }
    if all_nodes.len() > state.grammar.max_goals as usize {
        return Err(GscUnknownReason::ResourceExhausted);
    }
    let structural_goal_count =
        u32::try_from(libraries.len()).map_err(|_| GscUnknownReason::ResourceExhausted)?;
    let library_nodes =
        u32::try_from(all_nodes.len()).map_err(|_| GscUnknownReason::ResourceExhausted)?;
    let base_digest = {
        let mut encoder = CanonicalEncoder::new();
        encoder.u64(libraries.len() as u64);
        for library in &libraries {
            library.goal.digest.encode_canonical(&mut encoder);
            library.base_digest.encode_canonical(&mut encoder);
        }
        Digest::of_domain_bytes("pen-gf2/synchronous-closure-base/v1", encoder.as_bytes())
    };
    let frontier_digests = global_frontier_digests;
    let fixed_point_digest = {
        let mut encoder = CanonicalEncoder::new();
        base_digest.encode_canonical(&mut encoder);
        encoder.u64(frontier_digests.len() as u64);
        for frontier in &frontier_digests {
            frontier.encode_canonical(&mut encoder);
        }
        for library in &libraries {
            library.fixed_point_digest.encode_canonical(&mut encoder);
        }
        Digest::of_domain_bytes(
            "pen-gf2/synchronous-closure-fixed-point/v1",
            encoder.as_bytes(),
        )
    };
    let library_inventory_digest = {
        let mut encoder = CanonicalEncoder::new();
        encoder.u64(all_nodes.len() as u64);
        for node in &all_nodes {
            node.encode_canonical(&mut encoder);
        }
        fixed_point_digest.encode_canonical(&mut encoder);
        Digest::of_domain_bytes("pen-gf2/ordered-library-inventory/v1", encoder.as_bytes())
    };
    let unsupported_introduction_argument_seen = libraries
        .iter()
        .flat_map(|library| &library.nodes)
        .any(|node| {
            matches!(
                &node.ty,
                Term::Pi { parameter, .. }
                    if matches!(
                        parameter.as_ref(),
                        Term::Sort { .. } | Term::Pi { .. } | Term::Sigma { .. }
                    )
            )
        });
    Ok(ClosureSearchOutcome::Complete(ClosureAudit {
        libraries,
        structural_goal_count,
        library_nodes,
        base_digest,
        frontier_digests,
        fixed_point_digest,
        library_inventory_digest,
        unsupported_introduction_argument_seen,
    }))
}

fn application_grounding_lower_bound(
    builders: &[GoalLibraryBuilder],
) -> Result<usize, GscUnknownReason> {
    builders.iter().try_fold(0_usize, |total, builder| {
        let pairs = builder
            .nodes
            .len()
            .checked_mul(builder.nodes.len())
            .and_then(|count| count.checked_mul(2))
            .ok_or(GscUnknownReason::ResourceExhausted)?;
        total
            .checked_add(pairs)
            .ok_or(GscUnknownReason::ResourceExhausted)
    })
}

fn initialize_goal_library(
    state: &mut SearchState<'_>,
    goal: StructuralGoal,
    max_nodes: usize,
) -> Result<GoalLibraryBuilder, GscUnknownReason> {
    let mut base = BTreeMap::<Digest, SearchNode>::new();
    for index in 0..goal.context.0.len() {
        let distance = u32::try_from(index + 1).map_err(|_| GscUnknownReason::ResourceExhausted)?;
        let position = goal
            .context
            .0
            .len()
            .checked_sub(index + 1)
            .ok_or(GscUnknownReason::MalformedCode)?;
        let raw_ty = shift(&goal.context.0[position], i64::from(distance), 0, 0)
            .ok_or(GscUnknownReason::ResourceExhausted)?;
        let ty = state.normalize_type(&goal.context, &raw_ty)?;
        insert_library_node(
            &mut base,
            SearchNode {
                context: goal.context.clone(),
                term: Term::Var {
                    index: u32::try_from(index).map_err(|_| GscUnknownReason::ResourceExhausted)?,
                },
                ty,
                rule: OperationalRule::ContextWeakening,
                premises: Vec::new(),
            },
            max_nodes,
            0,
        )?;
    }
    let declarations = state.signature.declarations().to_vec();
    for declaration in declarations {
        let ty = state.normalize_type(&goal.context, &declaration.ty)?;
        insert_library_node(
            &mut base,
            SearchNode {
                context: goal.context.clone(),
                term: Term::Global { id: declaration.id },
                ty,
                rule: OperationalRule::PublicExact,
                premises: Vec::new(),
            },
            max_nodes,
            0,
        )?;
    }
    for (term, raw_ty) in [
        (Term::Unit, Term::UnitType),
        (Term::UnitType, Term::Sort { level: 0 }),
    ] {
        let ty = state.normalize_type(&goal.context, &raw_ty)?;
        insert_library_node(
            &mut base,
            SearchNode {
                context: goal.context.clone(),
                term,
                ty,
                rule: OperationalRule::PublicExact,
                premises: Vec::new(),
            },
            max_nodes,
            0,
        )?;
    }
    let mut nodes = base.into_values().collect::<Vec<_>>();
    nodes.sort_by_key(|node| (node.rule, empty_premise_digest(), search_node_digest(node)));
    let base_digest = frontier_digest(&goal, 0, &nodes);
    let seen = nodes
        .iter()
        .map(search_node_digest)
        .collect::<BTreeSet<_>>();
    Ok(GoalLibraryBuilder {
        goal,
        nodes,
        base_digest: base_digest.clone(),
        frontier_digests: vec![base_digest],
        seen,
        fixed: false,
    })
}

fn advance_goal_library(
    state: &mut SearchState<'_>,
    builder: &GoalLibraryBuilder,
    _round: u32,
    max_nodes: usize,
) -> Result<Vec<SearchNode>, GscUnknownReason> {
    let mut candidates = BTreeMap::<(OperationalRule, Digest, Digest), SearchNode>::new();
    let mut application_pairs = BTreeMap::<Digest, (usize, usize)>::new();
    for (function_index, function) in builder.nodes.iter().enumerate() {
        if !matches!(function.ty, Term::Pi { .. }) {
            continue;
        }
        for (argument_index, argument) in builder.nodes.iter().enumerate() {
            let premise = application_premise_digest(function, argument);
            if application_pairs
                .insert(premise, (function_index, argument_index))
                .is_some()
            {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
        }
    }
    for (_, (function_index, argument_index)) in application_pairs {
        let Term::Pi { parameter, body } = &builder.nodes[function_index].ty else {
            return Err(GscUnknownReason::KernelCouldNotCertify);
        };
        if state
            .probe(
                &builder.goal.context,
                &builder.nodes[argument_index].term,
                parameter,
            )?
            .is_none()
        {
            continue;
        }
        let raw_ty = substitute_newest_local(body, &builder.nodes[argument_index].term, 0)
            .ok_or(GscUnknownReason::ResourceExhausted)?;
        let ty = state.normalize_type(&builder.goal.context, &raw_ty)?;
        let term = Term::Apply {
            function: Box::new(builder.nodes[function_index].term.clone()),
            argument: Box::new(builder.nodes[argument_index].term.clone()),
        };
        if state.probe(&builder.goal.context, &term, &ty)?.is_none() {
            return Err(GscUnknownReason::KernelCouldNotCertify);
        }
        let node = SearchNode {
            context: builder.goal.context.clone(),
            term,
            ty,
            rule: OperationalRule::Application,
            premises: vec![function_index, argument_index],
        };
        let digest = search_node_digest(&node);
        if !builder.seen.contains(&digest) {
            candidates.insert(
                (
                    OperationalRule::Application,
                    application_premise_digest(
                        &builder.nodes[function_index],
                        &builder.nodes[argument_index],
                    ),
                    digest,
                ),
                node,
            );
        }
    }
    let mut projections = BTreeMap::<(OperationalRule, Digest), (usize, bool)>::new();
    for (index, node) in builder.nodes.iter().enumerate() {
        if !matches!(node.ty, Term::Sigma { .. }) {
            continue;
        }
        let premise = search_node_digest(node);
        projections.insert(
            (OperationalRule::FirstProjection, premise.clone()),
            (index, true),
        );
        projections.insert((OperationalRule::SecondProjection, premise), (index, false));
    }
    for ((rule, _), (pair_index, first)) in projections {
        let pair_node = &builder.nodes[pair_index];
        let Term::Sigma { parameter, body } = &pair_node.ty else {
            return Err(GscUnknownReason::KernelCouldNotCertify);
        };
        let (term, raw_ty) = if first {
            (
                Term::First {
                    pair: Box::new(pair_node.term.clone()),
                },
                (**parameter).clone(),
            )
        } else {
            let first_term = Term::First {
                pair: Box::new(pair_node.term.clone()),
            };
            (
                Term::Second {
                    pair: Box::new(pair_node.term.clone()),
                },
                substitute_newest_local(body, &first_term, 0)
                    .ok_or(GscUnknownReason::ResourceExhausted)?,
            )
        };
        let ty = state.normalize_type(&builder.goal.context, &raw_ty)?;
        if state.probe(&builder.goal.context, &term, &ty)?.is_none() {
            return Err(GscUnknownReason::KernelCouldNotCertify);
        }
        let node = SearchNode {
            context: builder.goal.context.clone(),
            term,
            ty,
            rule,
            premises: vec![pair_index],
        };
        let digest = search_node_digest(&node);
        if !builder.seen.contains(&digest) {
            candidates.insert((rule, search_node_digest(pair_node), digest), node);
        }
    }
    let mut frontier = Vec::new();
    let mut frontier_conclusions = BTreeSet::new();
    for (_, node) in candidates {
        let conclusion = search_node_digest(&node);
        if !frontier_conclusions.insert(conclusion) {
            continue;
        }
        if builder.nodes.len().saturating_add(frontier.len()) >= max_nodes {
            return Err(GscUnknownReason::ResourceExhausted);
        }
        frontier.push(node);
    }
    Ok(frontier)
}

fn select_canonical_positive(
    state: &mut SearchState<'_>,
    outer_goal: &StructuralGoal,
    builders: &[GoalLibraryBuilder],
    global_frontier_digests: &[Digest],
) -> Result<Option<ClosureSearchOutcome>, GscUnknownReason> {
    let Term::Pi { parameter, .. } = &outer_goal.target else {
        return Err(GscUnknownReason::KernelCouldNotCertify);
    };
    let mut candidates = BTreeMap::<GroundingKey, (bool, usize, usize)>::new();
    for (builder_index, builder) in builders.iter().enumerate() {
        for (node_index, node) in builder.nodes.iter().enumerate() {
            if state
                .probe(&builder.goal.context, &node.term, &builder.goal.target)?
                .is_none()
            {
                continue;
            }
            let (key, is_outer) = if builder.goal.digest == outer_goal.digest {
                (
                    GroundingKey {
                        structural_goal_digest: outer_goal.digest.clone(),
                        rule: OperationalRule::Q0Conversion,
                        premise_digest: search_node_digest(node),
                        conclusion_digest: typed_judgment_digest(
                            &outer_goal.context,
                            &node.term,
                            &outer_goal.target,
                        ),
                    },
                    true,
                )
            } else {
                let lambda = Term::Lambda {
                    parameter_type: parameter.clone(),
                    body: Box::new(node.term.clone()),
                };
                if state
                    .probe(&outer_goal.context, &lambda, &outer_goal.target)?
                    .is_none()
                {
                    return Err(GscUnknownReason::KernelCouldNotCertify);
                }
                (
                    GroundingKey {
                        structural_goal_digest: outer_goal.digest.clone(),
                        rule: OperationalRule::LambdaIntroduction,
                        premise_digest: search_node_digest(node),
                        conclusion_digest: typed_judgment_digest(
                            &outer_goal.context,
                            &lambda,
                            &outer_goal.target,
                        ),
                    },
                    false,
                )
            };
            if candidates
                .insert(key, (is_outer, builder_index, node_index))
                .is_some()
            {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
        }
    }
    let Some((_, (is_outer, builder_index, node_index))) = candidates.into_iter().next() else {
        return Ok(None);
    };
    let builder = &builders[builder_index];
    let positive = positive_frontier(
        builder.goal.clone(),
        builder.nodes.clone(),
        node_index,
        global_frontier_digests.to_vec(),
    );
    Ok(Some(if is_outer {
        ClosureSearchOutcome::ExactOuter(positive)
    } else {
        ClosureSearchOutcome::ExactBody(positive)
    }))
}

fn finish_goal_library(builder: GoalLibraryBuilder) -> GoalLibrary {
    let fixed_point_digest = {
        let mut encoder = CanonicalEncoder::new();
        builder.goal.digest.encode_canonical(&mut encoder);
        encoder.u64(builder.nodes.len() as u64);
        for node in &builder.nodes {
            search_node_digest(node).encode_canonical(&mut encoder);
        }
        encoder.u64(builder.frontier_digests.len() as u64);
        for frontier in &builder.frontier_digests {
            frontier.encode_canonical(&mut encoder);
        }
        encoder.tag(u8::from(builder.fixed));
        Digest::of_domain_bytes("pen-gf2/goal-library-fixed-point/v1", encoder.as_bytes())
    };
    GoalLibrary {
        goal: builder.goal,
        nodes: builder.nodes,
        base_digest: builder.base_digest,
        fixed_point_digest,
    }
}

fn positive_frontier(
    goal: StructuralGoal,
    nodes: Vec<SearchNode>,
    exact_index: usize,
    frontier_digests: Vec<Digest>,
) -> PositiveDerivationFrontier {
    let digest = {
        let mut encoder = CanonicalEncoder::new();
        goal.digest.encode_canonical(&mut encoder);
        encoder.u64(nodes.len() as u64);
        for node in &nodes {
            search_node_digest(node).encode_canonical(&mut encoder);
        }
        encoder.u64(frontier_digests.len() as u64);
        for frontier in &frontier_digests {
            frontier.encode_canonical(&mut encoder);
        }
        encoder.u64(exact_index as u64);
        Digest::of_domain_bytes("pen-gf2/canonical-positive-frontier/v1", encoder.as_bytes())
    };
    PositiveDerivationFrontier {
        goal,
        nodes,
        exact_index,
        frontier_digests,
        digest,
    }
}

fn insert_library_node(
    nodes: &mut BTreeMap<Digest, SearchNode>,
    node: SearchNode,
    max_nodes: usize,
    occupied_nodes: usize,
) -> Result<(), GscUnknownReason> {
    let key = search_node_digest(&node);
    if let Some(existing) = nodes.get(&key) {
        if existing.context != node.context || existing.term != node.term || existing.ty != node.ty
        {
            return Err(GscUnknownReason::KernelCouldNotCertify);
        }
        return Ok(());
    }
    if occupied_nodes.saturating_add(nodes.len()) >= max_nodes {
        return Err(GscUnknownReason::ResourceExhausted);
    }
    nodes.insert(key, node);
    Ok(())
}

fn frontier_digest(goal: &StructuralGoal, round: u32, nodes: &[SearchNode]) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    goal.digest.encode_canonical(&mut encoder);
    encoder.u32(round);
    encoder.u64(nodes.len() as u64);
    for node in nodes {
        search_node_digest(node).encode_canonical(&mut encoder);
    }
    Digest::of_domain_bytes("pen-gf2/goal-library-frontier/v1", encoder.as_bytes())
}

fn global_frontier_digest(
    round: u32,
    builders: &[GoalLibraryBuilder],
    frontiers: &[Vec<SearchNode>],
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u32(round);
    encoder.u64(builders.len() as u64);
    for (builder, frontier) in builders.iter().zip(frontiers) {
        builder.goal.digest.encode_canonical(&mut encoder);
        encoder.u64(frontier.len() as u64);
        for node in frontier {
            search_node_digest(node).encode_canonical(&mut encoder);
        }
    }
    Digest::of_domain_bytes("pen-gf2/synchronous-global-frontier/v1", encoder.as_bytes())
}

fn replay_complete_grounding_inventory(
    state: &mut SearchState<'_>,
    closure: &ClosureAudit,
) -> Result<FinalizedGroundingAudit, GscUnknownReason> {
    let witnesses =
        build_grounding_witnesses(closure, state.grammar.max_grounded_rule_instances as usize)?;
    let mut decisions = BTreeMap::new();
    let mut coverage = BTreeSet::new();
    let goal_digests = closure
        .libraries
        .iter()
        .map(|library| library.goal.digest.clone())
        .collect::<BTreeSet<_>>();
    for (key, witness) in &witnesses {
        if !goal_digests.contains(&key.structural_goal_digest) {
            return Err(GscUnknownReason::KernelCouldNotCertify);
        }
        let decision = replay_grounding(state, closure, key, witness)?;
        decisions.insert(key.clone(), decision);
        coverage.insert((key.structural_goal_digest.clone(), key.rule));
    }
    for goal in &goal_digests {
        for rule in ORDERED_OPERATIONAL_RULES {
            if !coverage.contains(&(goal.clone(), rule)) {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
        }
    }
    let goal_rule_coverage_digest = {
        let mut encoder = CanonicalEncoder::new();
        encoder.u64(coverage.len() as u64);
        for (goal, rule) in &coverage {
            goal.encode_canonical(&mut encoder);
            rule.encode_canonical(&mut encoder);
        }
        Digest::of_domain_bytes("pen-gf2/goal-rule-coverage/v1", encoder.as_bytes())
    };
    let canonical_replay_digest = {
        let mut encoder = CanonicalEncoder::new();
        encoder.u64(decisions.len() as u64);
        for (key, decision) in &decisions {
            encode_grounding_key(key, &mut encoder);
            encode_ground_decision(*decision, &mut encoder);
        }
        Digest::of_domain_bytes("pen-gf2/canonical-grounding-replay/v1", encoder.as_bytes())
    };
    let grounding_inventory_digest = {
        let mut encoder = CanonicalEncoder::new();
        closure.base_digest.encode_canonical(&mut encoder);
        encoder.u64(closure.frontier_digests.len() as u64);
        for frontier in &closure.frontier_digests {
            frontier.encode_canonical(&mut encoder);
        }
        closure.fixed_point_digest.encode_canonical(&mut encoder);
        closure
            .library_inventory_digest
            .encode_canonical(&mut encoder);
        goal_rule_coverage_digest.encode_canonical(&mut encoder);
        canonical_replay_digest.encode_canonical(&mut encoder);
        Digest::of_domain_bytes("pen-gf2/ordered-grounding-inventory/v1", encoder.as_bytes())
    };
    let dispositions = dispositions_from_decisions(&decisions)?;
    Ok(FinalizedGroundingAudit {
        dispositions,
        grounded_rule_instances: u32::try_from(decisions.len())
            .map_err(|_| GscUnknownReason::ResourceExhausted)?,
        structural_goal_count: closure.structural_goal_count,
        library_nodes: closure.library_nodes,
        closure_base_digest: closure.base_digest.clone(),
        closure_frontier_digests: closure.frontier_digests.clone(),
        closure_fixed_point_digest: closure.fixed_point_digest.clone(),
        goal_rule_coverage_digest,
        canonical_replay_digest,
        grounding_inventory_digest,
        library_inventory_digest: closure.library_inventory_digest.clone(),
    })
}

fn build_grounding_witnesses(
    closure: &ClosureAudit,
    max_groundings: usize,
) -> Result<BTreeMap<GroundingKey, GroundingWitness>, GscUnknownReason> {
    let mut witnesses = BTreeMap::new();
    for library in &closure.libraries {
        let goal = &library.goal;
        for node in &library.nodes {
            if matches!(
                node.rule,
                OperationalRule::PublicExact | OperationalRule::ContextWeakening
            ) {
                insert_grounding_witness(
                    &mut witnesses,
                    GroundingKey {
                        structural_goal_digest: goal.digest.clone(),
                        rule: node.rule,
                        premise_digest: empty_premise_digest(),
                        conclusion_digest: typed_judgment_digest(
                            &node.context,
                            &node.term,
                            &node.ty,
                        ),
                    },
                    GroundingWitness::TypedNode {
                        goal: goal.clone(),
                        node: node.clone(),
                    },
                    max_groundings,
                )?;
            }
            insert_grounding_witness(
                &mut witnesses,
                GroundingKey {
                    structural_goal_digest: goal.digest.clone(),
                    rule: OperationalRule::Q0Conversion,
                    premise_digest: search_node_digest(node),
                    conclusion_digest: typed_judgment_digest(
                        &goal.context,
                        &node.term,
                        &goal.target,
                    ),
                },
                GroundingWitness::Q0 {
                    goal: goal.clone(),
                    node: node.clone(),
                },
                max_groundings,
            )?;
            for first in [true, false] {
                let (rule, conclusion_term, conclusion_ty) = match (&node.ty, first) {
                    (Term::Sigma { parameter, .. }, true) => (
                        OperationalRule::FirstProjection,
                        Term::First {
                            pair: Box::new(node.term.clone()),
                        },
                        (**parameter).clone(),
                    ),
                    (Term::Sigma { body, .. }, false) => {
                        let first_term = Term::First {
                            pair: Box::new(node.term.clone()),
                        };
                        let raw_ty = substitute_newest_local(body, &first_term, 0)
                            .ok_or(GscUnknownReason::ResourceExhausted)?;
                        (
                            OperationalRule::SecondProjection,
                            Term::Second {
                                pair: Box::new(node.term.clone()),
                            },
                            raw_ty,
                        )
                    }
                    (_, true) => (
                        OperationalRule::FirstProjection,
                        Term::First {
                            pair: Box::new(node.term.clone()),
                        },
                        goal.target.clone(),
                    ),
                    (_, false) => (
                        OperationalRule::SecondProjection,
                        Term::Second {
                            pair: Box::new(node.term.clone()),
                        },
                        goal.target.clone(),
                    ),
                };
                insert_grounding_witness(
                    &mut witnesses,
                    GroundingKey {
                        structural_goal_digest: goal.digest.clone(),
                        rule,
                        premise_digest: search_node_digest(node),
                        conclusion_digest: typed_judgment_digest(
                            &goal.context,
                            &conclusion_term,
                            &conclusion_ty,
                        ),
                    },
                    GroundingWitness::Projection {
                        goal: goal.clone(),
                        principal: node.clone(),
                        first,
                    },
                    max_groundings,
                )?;
            }
        }
        for function in &library.nodes {
            for argument in &library.nodes {
                let premise_digest = application_premise_digest(function, argument);
                let conclusion_term = Term::Apply {
                    function: Box::new(function.term.clone()),
                    argument: Box::new(argument.term.clone()),
                };
                let conclusion_ty = match &function.ty {
                    Term::Pi { body, .. } => substitute_newest_local(body, &argument.term, 0)
                        .ok_or(GscUnknownReason::ResourceExhausted)?,
                    _ => goal.target.clone(),
                };
                let conclusion_digest =
                    typed_judgment_digest(&goal.context, &conclusion_term, &conclusion_ty);
                for rule in [
                    OperationalRule::CheckedSubstitution,
                    OperationalRule::Application,
                ] {
                    insert_grounding_witness(
                        &mut witnesses,
                        GroundingKey {
                            structural_goal_digest: goal.digest.clone(),
                            rule,
                            premise_digest: premise_digest.clone(),
                            conclusion_digest: conclusion_digest.clone(),
                        },
                        GroundingWitness::Application {
                            goal: goal.clone(),
                            function: function.clone(),
                            argument: argument.clone(),
                        },
                        max_groundings,
                    )?;
                }
            }
        }
        match &goal.target {
            Term::Pi { parameter, .. } => {
                let mut body_context = goal.context.clone();
                body_context.0.push((**parameter).clone());
                let body_library = closure
                    .libraries
                    .iter()
                    .find(|candidate| candidate.goal.context == body_context)
                    .ok_or(GscUnknownReason::KernelCouldNotCertify)?;
                if body_library.nodes.is_empty() {
                    insert_grounding_witness(
                        &mut witnesses,
                        GroundingKey {
                            structural_goal_digest: goal.digest.clone(),
                            rule: OperationalRule::LambdaIntroduction,
                            premise_digest: empty_premise_digest(),
                            conclusion_digest: grounding_sentinel_conclusion_digest(
                                goal,
                                OperationalRule::LambdaIntroduction,
                            ),
                        },
                        GroundingWitness::Lambda {
                            goal: goal.clone(),
                            body_goal: Some(body_library.goal.clone()),
                            body_node: None,
                        },
                        max_groundings,
                    )?;
                } else {
                    for body_node in &body_library.nodes {
                        let lambda = Term::Lambda {
                            parameter_type: parameter.clone(),
                            body: Box::new(body_node.term.clone()),
                        };
                        insert_grounding_witness(
                            &mut witnesses,
                            GroundingKey {
                                structural_goal_digest: goal.digest.clone(),
                                rule: OperationalRule::LambdaIntroduction,
                                premise_digest: search_node_digest(body_node),
                                conclusion_digest: typed_judgment_digest(
                                    &goal.context,
                                    &lambda,
                                    &goal.target,
                                ),
                            },
                            GroundingWitness::Lambda {
                                goal: goal.clone(),
                                body_goal: Some(body_library.goal.clone()),
                                body_node: Some(body_node.clone()),
                            },
                            max_groundings,
                        )?;
                    }
                }
            }
            _ => {
                insert_grounding_witness(
                    &mut witnesses,
                    GroundingKey {
                        structural_goal_digest: goal.digest.clone(),
                        rule: OperationalRule::LambdaIntroduction,
                        premise_digest: empty_premise_digest(),
                        conclusion_digest: grounding_sentinel_conclusion_digest(
                            goal,
                            OperationalRule::LambdaIntroduction,
                        ),
                    },
                    GroundingWitness::Lambda {
                        goal: goal.clone(),
                        body_goal: None,
                        body_node: None,
                    },
                    max_groundings,
                )?;
            }
        }
        if matches!(goal.target, Term::Sigma { .. }) {
            return Err(GscUnknownReason::UnsupportedCode);
        }
        insert_grounding_witness(
            &mut witnesses,
            GroundingKey {
                structural_goal_digest: goal.digest.clone(),
                rule: OperationalRule::PairIntroduction,
                premise_digest: empty_premise_digest(),
                conclusion_digest: grounding_sentinel_conclusion_digest(
                    goal,
                    OperationalRule::PairIntroduction,
                ),
            },
            GroundingWitness::GoalShape {
                goal: goal.clone(),
                reason: CertifiedInapplicabilityReason::GoalIsNotDependentPair,
            },
            max_groundings,
        )?;
        for (rule, reason) in [
            (
                OperationalRule::EquationReplay,
                CertifiedInapplicabilityReason::NoEquationAvailableBeforeResponse,
            ),
            (
                OperationalRule::QuotientTransport,
                CertifiedInapplicabilityReason::NoQuotientClassAvailableBeforeResponse,
            ),
        ] {
            insert_grounding_witness(
                &mut witnesses,
                GroundingKey {
                    structural_goal_digest: goal.digest.clone(),
                    rule,
                    premise_digest: empty_premise_digest(),
                    conclusion_digest: grounding_sentinel_conclusion_digest(goal, rule),
                },
                GroundingWitness::EmptyPreResponseRegistry {
                    goal: goal.clone(),
                    reason,
                },
                max_groundings,
            )?;
        }
    }
    Ok(witnesses)
}

fn insert_grounding_witness(
    witnesses: &mut BTreeMap<GroundingKey, GroundingWitness>,
    key: GroundingKey,
    witness: GroundingWitness,
    max_groundings: usize,
) -> Result<(), GscUnknownReason> {
    if !witnesses.contains_key(&key) && witnesses.len() >= max_groundings {
        return Err(GscUnknownReason::ResourceExhausted);
    }
    if witnesses.insert(key, witness).is_some() {
        return Err(GscUnknownReason::KernelCouldNotCertify);
    }
    Ok(())
}

fn replay_grounding(
    state: &mut SearchState<'_>,
    closure: &ClosureAudit,
    key: &GroundingKey,
    witness: &GroundingWitness,
) -> Result<GroundDecision, GscUnknownReason> {
    match witness {
        GroundingWitness::TypedNode { goal, node } => {
            verify_scope_and_node_key(key, goal, node, empty_premise_digest())?;
            if node.rule != key.rule || state.probe(&node.context, &node.term, &node.ty)?.is_none()
            {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            ensure_final_library_member(closure, goal, &node.term, &node.ty)?;
            Ok(GroundDecision::Applicable)
        }
        GroundingWitness::Q0 { goal, node } => {
            verify_scope(key, goal, OperationalRule::Q0Conversion)?;
            if key.premise_digest != search_node_digest(node)
                || key.conclusion_digest
                    != typed_judgment_digest(&goal.context, &node.term, &goal.target)
                || state.probe(&node.context, &node.term, &node.ty)?.is_none()
            {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            if state
                .probe(&goal.context, &node.term, &goal.target)?
                .is_some()
            {
                ensure_final_library_member(closure, goal, &node.term, &node.ty)?;
                Ok(GroundDecision::Applicable)
            } else {
                Ok(GroundDecision::CertifiedInapplicable(
                    CertifiedInapplicabilityReason::NormalFormsDiffer,
                ))
            }
        }
        GroundingWitness::Application {
            goal,
            function,
            argument,
        } => {
            if !matches!(
                key.rule,
                OperationalRule::CheckedSubstitution | OperationalRule::Application
            ) {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            verify_scope(key, goal, key.rule)?;
            if key.premise_digest != application_premise_digest(function, argument)
                || state
                    .probe(&function.context, &function.term, &function.ty)?
                    .is_none()
                || state
                    .probe(&argument.context, &argument.term, &argument.ty)?
                    .is_none()
            {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            let Term::Pi { parameter, body } = &function.ty else {
                let attempted = Term::Apply {
                    function: Box::new(function.term.clone()),
                    argument: Box::new(argument.term.clone()),
                };
                if key.conclusion_digest
                    != typed_judgment_digest(&goal.context, &attempted, &goal.target)
                {
                    return Err(GscUnknownReason::KernelCouldNotCertify);
                }
                return Ok(GroundDecision::CertifiedInapplicable(
                    CertifiedInapplicabilityReason::PrincipalIsNotDependentFunction,
                ));
            };
            let raw_ty = substitute_newest_local(body, &argument.term, 0)
                .ok_or(GscUnknownReason::ResourceExhausted)?;
            let term = Term::Apply {
                function: Box::new(function.term.clone()),
                argument: Box::new(argument.term.clone()),
            };
            if key.conclusion_digest != typed_judgment_digest(&goal.context, &term, &raw_ty) {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            if state
                .probe(&goal.context, &argument.term, parameter)?
                .is_none()
            {
                return Ok(GroundDecision::CertifiedInapplicable(
                    CertifiedInapplicabilityReason::ArgumentTypeMismatch,
                ));
            }
            let result_ty = state.normalize_type(&goal.context, &raw_ty)?;
            if key.rule == OperationalRule::Application {
                if state.probe(&goal.context, &term, &result_ty)?.is_none() {
                    return Err(GscUnknownReason::KernelCouldNotCertify);
                }
                ensure_final_library_member(closure, goal, &term, &result_ty)?;
            }
            Ok(GroundDecision::Applicable)
        }
        GroundingWitness::Projection {
            goal,
            principal,
            first,
        } => {
            let rule = if *first {
                OperationalRule::FirstProjection
            } else {
                OperationalRule::SecondProjection
            };
            verify_scope(key, goal, rule)?;
            if key.premise_digest != search_node_digest(principal)
                || state
                    .probe(&principal.context, &principal.term, &principal.ty)?
                    .is_none()
            {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            let Term::Sigma { parameter, body } = &principal.ty else {
                let term = if *first {
                    Term::First {
                        pair: Box::new(principal.term.clone()),
                    }
                } else {
                    Term::Second {
                        pair: Box::new(principal.term.clone()),
                    }
                };
                if key.conclusion_digest
                    != typed_judgment_digest(&goal.context, &term, &goal.target)
                {
                    return Err(GscUnknownReason::KernelCouldNotCertify);
                }
                return Ok(GroundDecision::CertifiedInapplicable(
                    CertifiedInapplicabilityReason::PrincipalIsNotDependentPair,
                ));
            };
            let (term, raw_result_ty) = if *first {
                (
                    Term::First {
                        pair: Box::new(principal.term.clone()),
                    },
                    (**parameter).clone(),
                )
            } else {
                let first_term = Term::First {
                    pair: Box::new(principal.term.clone()),
                };
                let raw_ty = substitute_newest_local(body, &first_term, 0)
                    .ok_or(GscUnknownReason::ResourceExhausted)?;
                (
                    Term::Second {
                        pair: Box::new(principal.term.clone()),
                    },
                    raw_ty,
                )
            };
            if key.conclusion_digest != typed_judgment_digest(&goal.context, &term, &raw_result_ty)
            {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            let result_ty = state.normalize_type(&goal.context, &raw_result_ty)?;
            if state.probe(&goal.context, &term, &result_ty)?.is_none() {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            ensure_final_library_member(closure, goal, &term, &result_ty)?;
            Ok(GroundDecision::Applicable)
        }
        GroundingWitness::Lambda {
            goal,
            body_goal,
            body_node,
        } => {
            verify_scope(key, goal, OperationalRule::LambdaIntroduction)?;
            let Term::Pi { parameter, body } = &goal.target else {
                if body_goal.is_some()
                    || body_node.is_some()
                    || key.premise_digest != empty_premise_digest()
                    || key.conclusion_digest
                        != grounding_sentinel_conclusion_digest(
                            goal,
                            OperationalRule::LambdaIntroduction,
                        )
                {
                    return Err(GscUnknownReason::KernelCouldNotCertify);
                }
                return Ok(GroundDecision::CertifiedInapplicable(
                    CertifiedInapplicabilityReason::GoalIsNotDependentFunction,
                ));
            };
            let Some(body_goal) = body_goal else {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            };
            let mut expected_context = goal.context.clone();
            expected_context.0.push((**parameter).clone());
            let expected_body = state.normalize_type(&expected_context, body)?;
            if body_goal.context != expected_context || body_goal.target != expected_body {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            let Some(body_node) = body_node else {
                if key.premise_digest != empty_premise_digest()
                    || key.conclusion_digest
                        != grounding_sentinel_conclusion_digest(
                            goal,
                            OperationalRule::LambdaIntroduction,
                        )
                {
                    return Err(GscUnknownReason::KernelCouldNotCertify);
                }
                return Ok(GroundDecision::CertifiedInapplicable(
                    CertifiedInapplicabilityReason::NoBodyInFiniteLibrary,
                ));
            };
            if key.premise_digest != search_node_digest(body_node)
                || state
                    .probe(&body_node.context, &body_node.term, &body_node.ty)?
                    .is_none()
            {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            let lambda = Term::Lambda {
                parameter_type: parameter.clone(),
                body: Box::new(body_node.term.clone()),
            };
            if key.conclusion_digest != typed_judgment_digest(&goal.context, &lambda, &goal.target)
            {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            if state
                .probe(&body_goal.context, &body_node.term, &body_goal.target)?
                .is_none()
            {
                return Ok(GroundDecision::CertifiedInapplicable(
                    CertifiedInapplicabilityReason::NoBodyInFiniteLibrary,
                ));
            }
            if state.probe(&goal.context, &lambda, &goal.target)?.is_none() {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            Ok(GroundDecision::Applicable)
        }
        GroundingWitness::GoalShape { goal, reason } => {
            verify_scope(key, goal, OperationalRule::PairIntroduction)?;
            if *reason != CertifiedInapplicabilityReason::GoalIsNotDependentPair
                || matches!(goal.target, Term::Sigma { .. })
                || key.premise_digest != empty_premise_digest()
                || key.conclusion_digest
                    != grounding_sentinel_conclusion_digest(goal, OperationalRule::PairIntroduction)
            {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            Ok(GroundDecision::CertifiedInapplicable(*reason))
        }
        GroundingWitness::EmptyPreResponseRegistry { goal, reason } => {
            let expected_rule = match reason {
                CertifiedInapplicabilityReason::NoEquationAvailableBeforeResponse => {
                    OperationalRule::EquationReplay
                }
                CertifiedInapplicabilityReason::NoQuotientClassAvailableBeforeResponse => {
                    OperationalRule::QuotientTransport
                }
                _ => return Err(GscUnknownReason::KernelCouldNotCertify),
            };
            verify_scope(key, goal, expected_rule)?;
            if key.premise_digest != empty_premise_digest()
                || key.conclusion_digest
                    != grounding_sentinel_conclusion_digest(goal, expected_rule)
            {
                return Err(GscUnknownReason::KernelCouldNotCertify);
            }
            Ok(GroundDecision::CertifiedInapplicable(*reason))
        }
    }
}

fn verify_scope(
    key: &GroundingKey,
    goal: &StructuralGoal,
    rule: OperationalRule,
) -> Result<(), GscUnknownReason> {
    if key.structural_goal_digest != goal.digest
        || key.rule != rule
        || goal.digest != operational_goal_digest(&goal.context, &goal.target)
    {
        return Err(GscUnknownReason::KernelCouldNotCertify);
    }
    Ok(())
}

fn verify_scope_and_node_key(
    key: &GroundingKey,
    goal: &StructuralGoal,
    node: &SearchNode,
    premise_digest: Digest,
) -> Result<(), GscUnknownReason> {
    verify_scope(key, goal, node.rule)?;
    if node.context != goal.context
        || key.premise_digest != premise_digest
        || key.conclusion_digest != typed_judgment_digest(&node.context, &node.term, &node.ty)
    {
        return Err(GscUnknownReason::KernelCouldNotCertify);
    }
    Ok(())
}

fn ensure_final_library_member(
    closure: &ClosureAudit,
    goal: &StructuralGoal,
    term: &Term,
    ty: &Term,
) -> Result<(), GscUnknownReason> {
    let Some(library) = closure
        .libraries
        .iter()
        .find(|library| library.goal.digest == goal.digest)
    else {
        return Err(GscUnknownReason::KernelCouldNotCertify);
    };
    let expected = library_node_digest(&goal.context, term, ty);
    if !library
        .nodes
        .iter()
        .any(|node| search_node_digest(node) == expected)
    {
        return Err(GscUnknownReason::KernelCouldNotCertify);
    }
    Ok(())
}

fn dispositions_from_decisions(
    decisions: &BTreeMap<GroundingKey, GroundDecision>,
) -> Result<Vec<VerifiedGroundRuleDisposition>, GscUnknownReason> {
    let mut dispositions = Vec::with_capacity(ORDERED_OPERATIONAL_RULES.len());
    for rule in ORDERED_OPERATIONAL_RULES {
        let mut applicable = 0_u32;
        let mut by_reason = BTreeMap::<CertifiedInapplicabilityReason, Vec<&GroundingKey>>::new();
        for (key, decision) in decisions {
            if key.rule != rule {
                continue;
            }
            match decision {
                GroundDecision::Applicable => {
                    applicable = applicable
                        .checked_add(1)
                        .ok_or(GscUnknownReason::ResourceExhausted)?;
                }
                GroundDecision::CertifiedInapplicable(reason) => {
                    by_reason.entry(*reason).or_default().push(key);
                }
            }
        }
        let mut inapplicable = 0_u32;
        let mut evidence = Vec::with_capacity(by_reason.len());
        for (reason, keys) in by_reason {
            let instances =
                u32::try_from(keys.len()).map_err(|_| GscUnknownReason::ResourceExhausted)?;
            inapplicable = inapplicable
                .checked_add(instances)
                .ok_or(GscUnknownReason::ResourceExhausted)?;
            let grounding_set_digest = {
                let mut encoder = CanonicalEncoder::new();
                rule.encode_canonical(&mut encoder);
                encoder.tag(inapplicability_reason_tag(reason));
                encoder.u64(keys.len() as u64);
                for key in keys {
                    encode_grounding_key(key, &mut encoder);
                }
                Digest::of_domain_bytes("pen-gf2/inapplicable-grounding-set/v1", encoder.as_bytes())
            };
            evidence.push(VerifiedInapplicabilityEvidence {
                reason,
                instances,
                grounding_set_digest,
            });
        }
        dispositions.push(VerifiedGroundRuleDisposition {
            rule,
            grounded_instances: applicable
                .checked_add(inapplicable)
                .ok_or(GscUnknownReason::ResourceExhausted)?,
            applicable_instances: applicable,
            certified_inapplicable_instances: inapplicable,
            inapplicability_evidence: evidence,
        });
    }
    Ok(dispositions)
}

fn encode_grounding_key(key: &GroundingKey, encoder: &mut CanonicalEncoder) {
    key.structural_goal_digest.encode_canonical(encoder);
    key.rule.encode_canonical(encoder);
    key.premise_digest.encode_canonical(encoder);
    key.conclusion_digest.encode_canonical(encoder);
}

fn encode_ground_decision(decision: GroundDecision, encoder: &mut CanonicalEncoder) {
    match decision {
        GroundDecision::Applicable => encoder.tag(0),
        GroundDecision::CertifiedInapplicable(reason) => {
            encoder.tag(1);
            encoder.tag(inapplicability_reason_tag(reason));
        }
    }
}

fn relevant_dag(nodes: &[SearchNode], conclusion: usize) -> VerifiedDerivationDag {
    fn collect(index: usize, nodes: &[SearchNode], included: &mut BTreeSet<usize>) {
        if !included.insert(index) {
            return;
        }
        for premise in &nodes[index].premises {
            collect(*premise, nodes, included);
        }
    }
    let mut included = BTreeSet::new();
    collect(conclusion, nodes, &mut included);
    let old_indices = included.into_iter().collect::<Vec<_>>();
    let remap = old_indices
        .iter()
        .enumerate()
        .map(|(new, old)| (*old, new as u32))
        .collect::<BTreeMap<_, _>>();
    let verified = old_indices
        .iter()
        .map(|old| {
            let node = &nodes[*old];
            VerifiedDerivationNode {
                ordinal: remap[old],
                context: node.context.clone(),
                term: node.term.clone(),
                ty: node.ty.clone(),
                rule: node.rule,
                premises: node.premises.iter().map(|index| remap[index]).collect(),
            }
        })
        .collect::<Vec<_>>();
    let conclusion = remap[&conclusion];
    let digest = derivation_digest(&verified, conclusion);
    VerifiedDerivationDag {
        nodes: verified,
        conclusion,
        digest,
    }
}

pub(crate) fn verify_demand_connected_response(
    use_family: &VerifiedCanonicalDemandFamilyV2,
    compute_family: &VerifiedCanonicalDemandFamilyV2,
    equation_set: &VerifiedEquationExtensionSet,
    use_derivation: VerifiedDerivedUse,
) -> GscOutcome<VerifiedDemandConnectedResponse> {
    if use_family.ports().len() != 1
        || compute_family.ports().len() != 1
        || compute_family.premise_refs().len() != 1
        || use_derivation.port() != use_family.ports()[0].key()
        || use_derivation.term() != equation_set.clause().substitution().filler()
        || !use_derivation
            .derivation()
            .contains_global(equation_set.clause().fresh_head())
    {
        return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
    }
    let premise = &compute_family.premise_refs()[0];
    if premise.context_map != ContextMapCode::IdentityV1
        || premise.source
            != (PortRef::Generated {
                port: use_family.ports()[0].key().clone(),
            })
        || equation_set.clause().substitution().source_port() != use_family.ports()[0].key()
    {
        return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
    }
    let use_discharge = VerifiedPortDischarge {
        port: use_family.ports()[0].key().clone(),
        rule: OperationalRule::Q0Conversion,
        premises: Vec::new(),
        evidence_digest: use_derivation.digest().clone(),
    };
    let equation_discharge = VerifiedPortDischarge {
        port: compute_family.ports()[0].key().clone(),
        rule: OperationalRule::EquationReplay,
        premises: vec![use_family.ports()[0].key().clone()],
        evidence_digest: equation_set.digest().clone(),
    };
    let discharges = vec![use_discharge, equation_discharge];
    let digest = {
        let mut encoder = CanonicalEncoder::new();
        use_derivation.digest().encode_canonical(&mut encoder);
        equation_set.digest().encode_canonical(&mut encoder);
        for discharge in &discharges {
            discharge.port.encode_canonical(&mut encoder);
            discharge.rule.encode_canonical(&mut encoder);
            encoder.sequence(&discharge.premises);
            discharge.evidence_digest.encode_canonical(&mut encoder);
        }
        encoder.tag(1);
        encoder.tag(1);
        Digest::of_domain_bytes("pen-gf2/demand-connected-response/v1", encoder.as_bytes())
    };
    GscOutcome::Proven(VerifiedDemandConnectedResponse {
        use_derivation,
        equation_set_digest: equation_set.digest().clone(),
        discharges,
        all_outputs_filled: true,
        all_positive_clauses_connected: true,
        digest,
    })
}

pub fn operational_grammar_digest(semantic: &VerifiedGscSemanticManifest) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u16(1);
    semantic
        .manifest()
        .derivability_grammar
        .encode_canonical(&mut encoder);
    Digest::of_domain_bytes("pen-gf2/operational-grammar/v1", encoder.as_bytes())
}

fn is_exact_one_nullary_use_goal(context: &DependentContext, target: &Term) -> bool {
    let [motive_type, method_type] = context.0.as_slice() else {
        return false;
    };
    let Term::Pi {
        parameter: owner,
        body: universe,
    } = motive_type
    else {
        return false;
    };
    if !matches!(universe.as_ref(), Term::Sort { .. })
        || !matches!(
            method_type,
            Term::Apply {
                function,
                ..
            } if function.as_ref() == &Term::Var { index: 0 }
        )
    {
        return false;
    }
    matches!(
        target,
        Term::Pi {
            parameter,
            body,
        } if parameter == owner
            && matches!(
                body.as_ref(),
                Term::Apply {
                    function,
                    argument,
                } if function.as_ref() == &Term::Var { index: 2 }
                    && argument.as_ref() == &Term::Var { index: 0 }
            )
    )
}

fn canonical_form_analysis(
    context: &DependentContext,
    target: &Term,
    closure_reached_fixed_point: bool,
    higher_order_argument_gap_absent: bool,
    all_structural_goals_covered: bool,
) -> VerifiedCanonicalFormAnalysis {
    let exact_shape = is_exact_one_nullary_use_goal(context, target);
    let neutral_heads_exhausted = closure_reached_fixed_point && all_structural_goals_covered;
    let introduction_rules_separated = exact_shape && all_structural_goals_covered;
    let digest = {
        let mut encoder = CanonicalEncoder::new();
        context.encode_canonical(&mut encoder);
        target.encode_canonical(&mut encoder);
        encoder.tag(u8::from(exact_shape));
        encoder.tag(u8::from(exact_shape));
        encoder.tag(u8::from(neutral_heads_exhausted));
        encoder.tag(u8::from(higher_order_argument_gap_absent));
        encoder.tag(u8::from(introduction_rules_separated));
        Digest::of_domain_bytes("pen-gf2/canonical-form-analysis/v1", encoder.as_bytes())
    };
    VerifiedCanonicalFormAnalysis {
        single_outer_function: exact_shape,
        stuck_atomic_body: exact_shape,
        neutral_heads_exhausted,
        higher_order_argument_gap_absent,
        introduction_rules_separated,
        digest,
    }
}

fn encode_dispositions(
    dispositions: &[VerifiedGroundRuleDisposition],
    encoder: &mut CanonicalEncoder,
) {
    encoder.u64(dispositions.len() as u64);
    for disposition in dispositions {
        disposition.rule.encode_canonical(encoder);
        encoder.u32(disposition.grounded_instances);
        encoder.u32(disposition.applicable_instances);
        encoder.u32(disposition.certified_inapplicable_instances);
        encoder.u64(disposition.inapplicability_evidence.len() as u64);
        for evidence in &disposition.inapplicability_evidence {
            encoder.tag(inapplicability_reason_tag(evidence.reason));
            encoder.u32(evidence.instances);
            evidence.grounding_set_digest.encode_canonical(encoder);
        }
    }
}

fn inapplicability_reason_tag(reason: CertifiedInapplicabilityReason) -> u8 {
    match reason {
        CertifiedInapplicabilityReason::NormalFormsDiffer => 0,
        CertifiedInapplicabilityReason::GoalIsNotDependentFunction => 1,
        CertifiedInapplicabilityReason::PrincipalIsNotDependentFunction => 2,
        CertifiedInapplicabilityReason::ArgumentTypeMismatch => 3,
        CertifiedInapplicabilityReason::GoalIsNotDependentPair => 4,
        CertifiedInapplicabilityReason::PrincipalIsNotDependentPair => 5,
        CertifiedInapplicabilityReason::NoBodyInFiniteLibrary => 6,
        CertifiedInapplicabilityReason::NoEquationAvailableBeforeResponse => 7,
        CertifiedInapplicabilityReason::NoQuotientClassAvailableBeforeResponse => 8,
    }
}

fn operational_goal_digest(context: &DependentContext, target: &Term) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    context.encode_canonical(&mut encoder);
    target.encode_canonical(&mut encoder);
    Digest::of_domain_bytes("pen-gf2/operational-goal/v1", encoder.as_bytes())
}

fn typed_judgment_digest(context: &DependentContext, term: &Term, ty: &Term) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    context.encode_canonical(&mut encoder);
    term.encode_canonical(&mut encoder);
    ty.encode_canonical(&mut encoder);
    Digest::of_domain_bytes(
        "pen-gf2/grounding-conclusion-judgment/v1",
        encoder.as_bytes(),
    )
}

fn grounding_sentinel_conclusion_digest(goal: &StructuralGoal, rule: OperationalRule) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    goal.digest.encode_canonical(&mut encoder);
    rule.encode_canonical(&mut encoder);
    Digest::of_domain_bytes(
        "pen-gf2/grounding-sentinel-conclusion/v1",
        encoder.as_bytes(),
    )
}

fn probe_request_digest(
    signature: &VerifiedSignature,
    context: &DependentContext,
    term: &Term,
    ty: &Term,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    signature.digest().encode_canonical(&mut encoder);
    context.encode_canonical(&mut encoder);
    term.encode_canonical(&mut encoder);
    ty.encode_canonical(&mut encoder);
    Digest::of_domain_bytes("pen-gf2/operational-probe-request/v1", encoder.as_bytes())
}

fn normalization_request_digest(
    signature: &VerifiedSignature,
    context: &DependentContext,
    ty: &Term,
) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    signature.digest().encode_canonical(&mut encoder);
    context.encode_canonical(&mut encoder);
    ty.encode_canonical(&mut encoder);
    Digest::of_domain_bytes(
        "pen-gf2/operational-normalization-request/v1",
        encoder.as_bytes(),
    )
}

fn library_node_digest(context: &DependentContext, term: &Term, ty: &Term) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    context.encode_canonical(&mut encoder);
    term.encode_canonical(&mut encoder);
    ty.encode_canonical(&mut encoder);
    Digest::of_domain_bytes("pen-gf2/library-node/v1", encoder.as_bytes())
}

fn search_node_digest(node: &SearchNode) -> Digest {
    library_node_digest(&node.context, &node.term, &node.ty)
}

fn application_premise_digest(function: &SearchNode, argument: &SearchNode) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    search_node_digest(function).encode_canonical(&mut encoder);
    search_node_digest(argument).encode_canonical(&mut encoder);
    Digest::of_domain_bytes("pen-gf2/application-premises/v1", encoder.as_bytes())
}

fn empty_premise_digest() -> Digest {
    Digest::of_domain_bytes("pen-gf2/empty-premises/v1", &[])
}

fn derivation_digest(nodes: &[VerifiedDerivationNode], conclusion: u32) -> Digest {
    let mut encoder = CanonicalEncoder::new();
    encoder.u64(nodes.len() as u64);
    for node in nodes {
        encoder.u32(node.ordinal);
        node.context.encode_canonical(&mut encoder);
        node.term.encode_canonical(&mut encoder);
        node.ty.encode_canonical(&mut encoder);
        node.rule.encode_canonical(&mut encoder);
        encoder.u64(node.premises.len() as u64);
        for premise in &node.premises {
            encoder.u32(*premise);
        }
    }
    encoder.u32(conclusion);
    Digest::of_domain_bytes("pen-gf2/typed-derivation-dag/v1", encoder.as_bytes())
}

fn term_contains_global(term: &Term, target: &GlobalId) -> bool {
    match term {
        Term::Global { id } => id == target,
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            term_contains_global(parameter, target) || term_contains_global(body, target)
        }
        Term::Lambda {
            parameter_type,
            body,
        } => term_contains_global(parameter_type, target) || term_contains_global(body, target),
        Term::Apply { function, argument } => {
            term_contains_global(function, target) || term_contains_global(argument, target)
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            term_contains_global(sigma_type, target)
                || term_contains_global(first, target)
                || term_contains_global(second, target)
        }
        Term::First { pair } | Term::Second { pair } => term_contains_global(pair, target),
        Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => false,
    }
}

fn substitute_newest_local(term: &Term, replacement: &Term, depth: u16) -> Option<Term> {
    if depth > pen_kernel::MAX_SAFE_RECURSION_DEPTH {
        return None;
    }
    let child = depth.checked_add(1)?;
    let binder_depth = u32::from(depth);
    match term {
        Term::Var { index } if *index == binder_depth => {
            shift(replacement, i64::from(binder_depth), 0, child)
        }
        Term::Var { index } if *index > binder_depth => Some(Term::Var {
            index: index.checked_sub(1)?,
        }),
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Some(term.clone()),
        Term::Pi { parameter, body } => Some(Term::Pi {
            parameter: Box::new(substitute_newest_local(parameter, replacement, depth)?),
            body: Box::new(substitute_newest_local(body, replacement, child)?),
        }),
        Term::Sigma { parameter, body } => Some(Term::Sigma {
            parameter: Box::new(substitute_newest_local(parameter, replacement, depth)?),
            body: Box::new(substitute_newest_local(body, replacement, child)?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Some(Term::Lambda {
            parameter_type: Box::new(substitute_newest_local(parameter_type, replacement, depth)?),
            body: Box::new(substitute_newest_local(body, replacement, child)?),
        }),
        Term::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(substitute_newest_local(function, replacement, depth)?),
            argument: Box::new(substitute_newest_local(argument, replacement, depth)?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Some(Term::Pair {
            sigma_type: Box::new(substitute_newest_local(sigma_type, replacement, depth)?),
            first: Box::new(substitute_newest_local(first, replacement, depth)?),
            second: Box::new(substitute_newest_local(second, replacement, depth)?),
        }),
        Term::First { pair } => Some(Term::First {
            pair: Box::new(substitute_newest_local(pair, replacement, depth)?),
        }),
        Term::Second { pair } => Some(Term::Second {
            pair: Box::new(substitute_newest_local(pair, replacement, depth)?),
        }),
    }
}

#[cfg(test)]
pub(crate) fn rejects_incomplete_fixed_point_for_test(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    semantic: &VerifiedGscSemanticManifest,
    family: &VerifiedCanonicalDemandFamilyV2,
) -> bool {
    let Some(port) = family.ports().first() else {
        return false;
    };
    let OutputClause::TermPort {
        context, motive, ..
    } = port.clause()
    else {
        return false;
    };
    let mut grammar = semantic.manifest().derivability_grammar.clone();
    grammar.max_fixed_point_rounds = 1;
    let mut state = SearchState {
        kernel,
        signature,
        grammar: &grammar,
        probe_cache: BTreeMap::new(),
        normalization_cache: BTreeMap::new(),
    };
    let Ok(target) = state.normalize_type(context, motive) else {
        return false;
    };
    let Ok((outer_goal, structural_goals)) = exact_structural_goals(&mut state, context, &target)
    else {
        return false;
    };
    matches!(
        search_or_complete_closure(&mut state, &outer_goal, structural_goals),
        Err(GscUnknownReason::ResourceExhausted)
    )
}

#[cfg(test)]
pub(crate) fn canonical_replay_is_permutation_invariant_for_test(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    semantic: &VerifiedGscSemanticManifest,
    family: &VerifiedCanonicalDemandFamilyV2,
) -> bool {
    let Some(port) = family.ports().first() else {
        return false;
    };
    let OutputClause::TermPort {
        context, motive, ..
    } = port.clause()
    else {
        return false;
    };
    let grammar = &semantic.manifest().derivability_grammar;
    let mut state = SearchState {
        kernel,
        signature,
        grammar,
        probe_cache: BTreeMap::new(),
        normalization_cache: BTreeMap::new(),
    };
    let Ok(target) = state.normalize_type(context, motive) else {
        return false;
    };
    let Ok((outer, goals)) = exact_structural_goals(&mut state, context, &target) else {
        return false;
    };
    let Ok(ClosureSearchOutcome::Complete(closure)) =
        search_or_complete_closure(&mut state, &outer, goals)
    else {
        return false;
    };
    let Ok(original) = replay_complete_grounding_inventory(&mut state, &closure) else {
        return false;
    };

    let mut permuted = closure.clone();
    permuted.libraries.reverse();
    for library in &mut permuted.libraries {
        library.nodes.reverse();
    }
    let mut permuted_state = SearchState {
        kernel,
        signature,
        grammar,
        probe_cache: BTreeMap::new(),
        normalization_cache: BTreeMap::new(),
    };
    let Ok(replayed) = replay_complete_grounding_inventory(&mut permuted_state, &permuted) else {
        return false;
    };
    original.canonical_replay_digest == replayed.canonical_replay_digest
        && original.grounding_inventory_digest == replayed.grounding_inventory_digest
        && original.goal_rule_coverage_digest == replayed.goal_rule_coverage_digest
}

#[cfg(test)]
pub(crate) fn operational_caps_reject_before_insert_for_test(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    semantic: &VerifiedGscSemanticManifest,
    family: &VerifiedCanonicalDemandFamilyV2,
) -> bool {
    let Some(port) = family.ports().first() else {
        return false;
    };
    let OutputClause::TermPort {
        context, motive, ..
    } = port.clause()
    else {
        return false;
    };
    let mut one_goal_grammar = semantic.manifest().derivability_grammar.clone();
    one_goal_grammar.max_goals = 1;
    let mut one_goal_state = SearchState {
        kernel,
        signature,
        grammar: &one_goal_grammar,
        probe_cache: BTreeMap::new(),
        normalization_cache: BTreeMap::new(),
    };
    let Ok(target) = one_goal_state.normalize_type(context, motive) else {
        return false;
    };
    let structural_cap = matches!(
        exact_structural_goals(&mut one_goal_state, context, &target),
        Err(GscUnknownReason::ResourceExhausted)
    );

    let grammar = &semantic.manifest().derivability_grammar;
    let mut state = SearchState {
        kernel,
        signature,
        grammar,
        probe_cache: BTreeMap::new(),
        normalization_cache: BTreeMap::new(),
    };
    let Ok(target) = state.normalize_type(context, motive) else {
        return false;
    };
    let Ok((outer, goals)) = exact_structural_goals(&mut state, context, &target) else {
        return false;
    };
    let Ok(ClosureSearchOutcome::Complete(closure)) =
        search_or_complete_closure(&mut state, &outer, goals)
    else {
        return false;
    };
    let Some(library) = closure.libraries.first() else {
        return false;
    };
    let Some(node) = library.nodes.first() else {
        return false;
    };
    let mut node_map = BTreeMap::new();
    let node_cap = matches!(
        insert_library_node(&mut node_map, node.clone(), 0, 0),
        Err(GscUnknownReason::ResourceExhausted)
    ) && node_map.is_empty();
    let key = GroundingKey {
        structural_goal_digest: library.goal.digest.clone(),
        rule: node.rule,
        premise_digest: empty_premise_digest(),
        conclusion_digest: typed_judgment_digest(&node.context, &node.term, &node.ty),
    };
    let witness = GroundingWitness::TypedNode {
        goal: library.goal.clone(),
        node: node.clone(),
    };
    let mut grounding_map = BTreeMap::new();
    let grounding_cap = matches!(
        insert_grounding_witness(&mut grounding_map, key, witness, 0),
        Err(GscUnknownReason::ResourceExhausted)
    ) && grounding_map.is_empty();
    structural_cap && node_cap && grounding_cap
}
