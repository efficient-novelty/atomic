//! Inventory-bound typing and coverage for the restricted Q0 rewrite program.
//!
//! This module intentionally stops before rewrite admissibility.  It checks
//! that the fresh rules reconstructed by the restricted normalizer correspond
//! one-to-one with the successor-new sealed equations in a replay-minted public
//! inventory.  The inventory now exposes a verified projection census but no
//! projection reduction theorem, so only its verified-empty case is accepted.

use crate::fragment::{
    LambdaUnitSyntaxViolation, lambda_unit_context_syntax_violation,
    lambda_unit_judgment_syntax_violation, lambda_unit_term_syntax_violation,
};
use crate::inventory::{DemandPortKeyV1, VerifiedPublicAuditInventoryV1, VerifiedPublicEquationV1};
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason,
    SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1, VerifiedSemanticAuditManifestV1,
};
use crate::model::{EquationIdV1, GenericJudgmentV1};
use crate::normalizer::{
    RestrictedQ0CertificateScopeV1, VerifiedFreshConstructorComputationV1,
    VerifiedFreshConstructorRuleV1,
};
use pen_kernel::{CanonicalEncode, CanonicalEncoder, DependentContext, Digest, GlobalId, Term};
use std::collections::BTreeSet;

/// One exact correspondence between a kernel-typed fresh rule and a sealed
/// public equation.
///
/// Fields are private and this type deliberately has no `Deserialize`
/// implementation.  Instances can occur only inside a verifier-minted typed
/// rewrite inventory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedTypedRewriteEntryV1 {
    equation: EquationIdV1,
    owner_head: GlobalId,
    constructor: GlobalId,
    arity: u16,
    scrutinee_parameter_ordinal: u16,
    context: DependentContext,
    left: Term,
    right: Term,
    ty: Term,
    source_identity: Digest,
    demand_port: Option<DemandPortKeyV1>,
}

impl VerifiedTypedRewriteEntryV1 {
    pub fn equation(&self) -> &EquationIdV1 {
        &self.equation
    }

    pub fn owner_head(&self) -> &GlobalId {
        &self.owner_head
    }

    pub fn constructor(&self) -> &GlobalId {
        &self.constructor
    }

    pub fn arity(&self) -> u16 {
        self.arity
    }

    pub fn scrutinee_parameter_ordinal(&self) -> u16 {
        self.scrutinee_parameter_ordinal
    }

    pub fn context(&self) -> &DependentContext {
        &self.context
    }

    pub fn left(&self) -> &Term {
        &self.left
    }

    pub fn right(&self) -> &Term {
        &self.right
    }

    pub fn ty(&self) -> &Term {
        &self.ty
    }

    pub fn source_identity(&self) -> &Digest {
        &self.source_identity
    }

    /// Exact strict-prior inventory association, when one was present.
    ///
    /// Carrying this key proves neither demand realization nor discharge and
    /// is never used as rewrite authority.
    pub fn demand_port(&self) -> Option<&DemandPortKeyV1> {
        self.demand_port.as_ref()
    }
}

impl CanonicalEncode for VerifiedTypedRewriteEntryV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.equation.encode_canonical(encoder);
        self.owner_head.encode_canonical(encoder);
        self.constructor.encode_canonical(encoder);
        encoder.u16(self.arity);
        encoder.u16(self.scrutinee_parameter_ordinal);
        self.context.encode_canonical(encoder);
        self.left.encode_canonical(encoder);
        self.right.encode_canonical(encoder);
        self.ty.encode_canonical(encoder);
        self.source_identity.encode_canonical(encoder);
        encoder.option(&self.demand_port);
    }
}

/// Opaque inventory of typed Q0 rewrite rules.
///
/// This is an intermediate capability, not a termination, confluence,
/// substitution, conservativity, or final rewrite-admissibility theorem.
/// Every field is private and the type deliberately has no `Deserialize`
/// implementation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedTypedRewriteInventoryV1 {
    inventory_digest: Digest,
    inventory_coverage_digest: Digest,
    manifest_digest: Digest,
    predecessor_history_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    exact_extension_digest: Digest,
    fresh_program_digest: Digest,
    normalizer_protocol_digest: Digest,
    descriptor_projection_inventory_digest: Digest,
    fresh_head: GlobalId,
    entries: Vec<VerifiedTypedRewriteEntryV1>,
    digest: Digest,
}

impl VerifiedTypedRewriteInventoryV1 {
    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn inventory_coverage_digest(&self) -> &Digest {
        &self.inventory_coverage_digest
    }

    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    pub fn predecessor_history_digest(&self) -> &Digest {
        &self.predecessor_history_digest
    }

    pub fn predecessor_boundary_digest(&self) -> &Digest {
        &self.predecessor_boundary_digest
    }

    pub fn successor_boundary_digest(&self) -> &Digest {
        &self.successor_boundary_digest
    }

    pub fn exact_extension_digest(&self) -> &Digest {
        &self.exact_extension_digest
    }

    pub fn fresh_program_digest(&self) -> &Digest {
        &self.fresh_program_digest
    }

    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.normalizer_protocol_digest
    }

    pub fn descriptor_projection_inventory_digest(&self) -> &Digest {
        &self.descriptor_projection_inventory_digest
    }

    pub fn fresh_head(&self) -> &GlobalId {
        &self.fresh_head
    }

    pub fn entries(&self) -> &[VerifiedTypedRewriteEntryV1] {
        &self.entries
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

impl CanonicalEncode for VerifiedTypedRewriteInventoryV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory_digest.encode_canonical(encoder);
        self.inventory_coverage_digest.encode_canonical(encoder);
        self.manifest_digest.encode_canonical(encoder);
        self.predecessor_history_digest.encode_canonical(encoder);
        self.predecessor_boundary_digest.encode_canonical(encoder);
        self.successor_boundary_digest.encode_canonical(encoder);
        self.exact_extension_digest.encode_canonical(encoder);
        self.fresh_program_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.descriptor_projection_inventory_digest
            .encode_canonical(encoder);
        self.fresh_head.encode_canonical(encoder);
        encoder.sequence(&self.entries);
    }
}

#[derive(Clone, Debug)]
struct VerifiedFreshRewriteCoverageV1 {
    inventory_digest: Digest,
    inventory_coverage_digest: Digest,
    manifest_digest: Digest,
    predecessor_history_digest: Digest,
    predecessor_boundary_digest: Digest,
    successor_boundary_digest: Digest,
    exact_extension_digest: Digest,
    fresh_program_digest: Digest,
    normalizer_protocol_digest: Digest,
    fresh_head: GlobalId,
    entries: Vec<VerifiedTypedRewriteEntryV1>,
}

/// Binding for an exhaustively verified empty projection census.
///
/// A nonempty census cannot mint this type until the descriptor projection
/// verifier reconstructs and checks exact reduction rules.
#[derive(Clone, Debug)]
struct VerifiedDescriptorProjectionInventoryV1 {
    digest: Digest,
}

type RewriteInventoryResult<T> = Result<T, AuditUnknownReason>;

/// Attempt to compile the complete typed rewrite inventory.
///
/// Fresh-rule coverage is checked first so omissions, extras, duplicates, and
/// mismatches are not hidden by the known projection blocker.  The current
/// public inventory has no descriptor reduction theorem, so a nonempty
/// verified projection census returns
/// `Unknown(MissingDescriptorProjectionInventory)`.
pub fn compile_typed_rewrite_inventory_v1(
    inventory: &VerifiedPublicAuditInventoryV1,
    fresh_program: &VerifiedFreshConstructorComputationV1,
) -> AuditDecision<VerifiedTypedRewriteInventoryV1> {
    let fresh = match verify_fresh_rewrite_coverage(inventory, fresh_program) {
        Ok(fresh) => fresh,
        Err(reason) => return AuditDecision::Unknown(reason),
    };
    let projections = match verify_descriptor_projection_inventory(inventory) {
        Ok(projections) => projections,
        Err(reason) => return AuditDecision::Unknown(reason),
    };
    AuditDecision::Proven(finish_typed_rewrite_inventory(fresh, projections))
}

/// Compile the exact typed rewrite inventory for the projection-free
/// lambda/unit successor profile.
///
/// The broader prototype keeps a missing-projection-theorem blocker.  In this
/// independently versioned fragment a nonempty projection census is instead a
/// positive outside-fragment disposition.
pub fn compile_typed_rewrite_inventory_lambda_unit_v1(
    manifest: &VerifiedSemanticAuditManifestV1,
    inventory: &VerifiedPublicAuditInventoryV1,
    fresh_program: &VerifiedFreshConstructorComputationV1,
) -> AuditDecision<VerifiedTypedRewriteInventoryV1> {
    if manifest.manifest().profile_id != SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1
        || inventory.manifest_digest() != manifest.candidate_digest()
        || fresh_program.manifest_digest() != manifest.candidate_digest()
    {
        return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
    }
    if !inventory.forced_projections().is_empty()
        || inventory.coverage().forced_projection_count() != 0
        || !inventory.forced_projection_origins().is_empty()
    {
        return AuditDecision::OutsideFragment(OutsideFragmentReason::DescriptorProjection);
    }
    if let Some(violation) = lambda_unit_rewrite_inventory_syntax_violation(
        inventory,
        fresh_program,
        &manifest.manifest().universe_levels,
    ) {
        return AuditDecision::OutsideFragment(violation.outside_reason());
    }
    compile_typed_rewrite_inventory_v1(inventory, fresh_program)
}

fn lambda_unit_rewrite_inventory_syntax_violation(
    inventory: &VerifiedPublicAuditInventoryV1,
    fresh_program: &VerifiedFreshConstructorComputationV1,
    universe_levels: &[u16],
) -> Option<LambdaUnitSyntaxViolation> {
    inventory
        .declarations()
        .iter()
        .filter_map(|declaration| {
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
        })
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
                .filter_map(|demand| {
                    [
                        lambda_unit_judgment_syntax_violation(
                            demand.source_requirement(),
                            universe_levels,
                        ),
                        lambda_unit_judgment_syntax_violation(
                            demand.normalized_requirement(),
                            universe_levels,
                        ),
                    ]
                    .into_iter()
                    .flatten()
                    .max()
                }),
        )
        .chain(fresh_program.rules().iter().filter_map(|rule| {
            [
                lambda_unit_context_syntax_violation(rule.context(), universe_levels),
                lambda_unit_term_syntax_violation(rule.left(), universe_levels),
                lambda_unit_term_syntax_violation(rule.right(), universe_levels),
                lambda_unit_term_syntax_violation(rule.ty(), universe_levels),
            ]
            .into_iter()
            .flatten()
            .max()
        }))
        .max()
}

/// Deliberately incomplete final theorem gate.
///
/// A typed, exactly covered rule inventory is necessary but does not by itself
/// prove substitution stability, generic termination/confluence,
/// conservativity, or transcript agreement.
pub fn attempt_rewrite_admissibility_v1(
    _inventory: &VerifiedTypedRewriteInventoryV1,
) -> AuditDecision<()> {
    AuditDecision::Unknown(AuditUnknownReason::MissingRewriteAdmissibilityTheorem)
}

fn verify_descriptor_projection_inventory(
    inventory: &VerifiedPublicAuditInventoryV1,
) -> RewriteInventoryResult<VerifiedDescriptorProjectionInventoryV1> {
    if !inventory.forced_projections().is_empty()
        || inventory.coverage().forced_projection_count() != 0
        || !inventory.forced_projection_origins().is_empty()
    {
        return Err(AuditUnknownReason::MissingDescriptorProjectionInventory);
    }
    Ok(VerifiedDescriptorProjectionInventoryV1 {
        digest: Digest::of_canonical(
            "pen-semantic-audit/verified-empty-descriptor-projection-inventory/v1",
            &EmptyProjectionInventoryDigestMaterial {
                inventory: inventory.digest(),
                coverage: inventory.coverage().digest(),
            },
        ),
    })
}

struct EmptyProjectionInventoryDigestMaterial<'a> {
    inventory: &'a Digest,
    coverage: &'a Digest,
}

impl CanonicalEncode for EmptyProjectionInventoryDigestMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory.encode_canonical(encoder);
        self.coverage.encode_canonical(encoder);
        encoder.u64(0);
    }
}

fn verify_fresh_rewrite_coverage(
    inventory: &VerifiedPublicAuditInventoryV1,
    program: &VerifiedFreshConstructorComputationV1,
) -> RewriteInventoryResult<VerifiedFreshRewriteCoverageV1> {
    verify_digest_bindings(inventory, program)?;
    verify_exact_fresh_declaration(inventory, program)?;
    verify_program_certificates(program)?;

    let successor_equations = inventory
        .equations()
        .iter()
        .filter(|equation| !equation.is_predecessor_public())
        .collect::<Vec<_>>();
    if successor_equations.len() != program.rules().len() {
        return Err(AuditUnknownReason::IncompleteEnumeration);
    }
    if successor_equations.iter().any(|equation| {
        equation.owner_head() != program.fresh_head()
            || equation.origin() != inventory.exact_extension().event()
    }) {
        return Err(AuditUnknownReason::IncompleteEnumeration);
    }

    reject_duplicate_program_rules(program.rules())?;

    let mut matched_equations = BTreeSet::new();
    let mut source_identities = BTreeSet::new();
    let mut entries = Vec::with_capacity(program.rules().len());
    for rule in program.rules() {
        let matches = successor_equations
            .iter()
            .filter(|equation| fresh_rule_matches_equation(program.fresh_head(), rule, equation))
            .copied()
            .collect::<Vec<_>>();
        let [equation] = matches.as_slice() else {
            return Err(if matches.is_empty() {
                AuditUnknownReason::IncompleteEnumeration
            } else {
                AuditUnknownReason::ProvenanceCollision
            });
        };
        if !matched_equations.insert(equation.equation().clone())
            || !source_identities.insert(equation.source_identity().clone())
        {
            return Err(AuditUnknownReason::ProvenanceCollision);
        }
        if let Some(port) = equation.demand_port() {
            let matching_contracts = inventory
                .predecessor_demand_contracts()
                .iter()
                .filter(|contract| contract.port() == port)
                .count();
            if matching_contracts != 1 {
                return Err(AuditUnknownReason::IncompleteSupport);
            }
        }
        entries.push(VerifiedTypedRewriteEntryV1 {
            equation: equation.equation().clone(),
            owner_head: equation.owner_head().clone(),
            constructor: rule.constructor().clone(),
            arity: rule.arity(),
            scrutinee_parameter_ordinal: rule.scrutinee_parameter_ordinal(),
            context: rule.context().clone(),
            left: rule.left().clone(),
            right: rule.right().clone(),
            ty: rule.ty().clone(),
            source_identity: equation.source_identity().clone(),
            demand_port: equation.demand_port().cloned(),
        });
    }
    if matched_equations.len() != successor_equations.len() {
        return Err(AuditUnknownReason::IncompleteEnumeration);
    }
    entries.sort_by(|left, right| left.equation.cmp(&right.equation));

    Ok(VerifiedFreshRewriteCoverageV1 {
        inventory_digest: inventory.digest().clone(),
        inventory_coverage_digest: inventory.coverage().digest().clone(),
        manifest_digest: inventory.manifest_digest().clone(),
        predecessor_history_digest: inventory.predecessor_history_digest().clone(),
        predecessor_boundary_digest: inventory.predecessor_boundary().digest().clone(),
        successor_boundary_digest: inventory.successor_boundary().digest().clone(),
        exact_extension_digest: inventory.exact_extension().digest().clone(),
        fresh_program_digest: program.program_digest().clone(),
        normalizer_protocol_digest: program
            .termination_certificate()
            .kernel_normalizer_protocol_digest()
            .clone(),
        fresh_head: program.fresh_head().clone(),
        entries,
    })
}

fn verify_digest_bindings(
    inventory: &VerifiedPublicAuditInventoryV1,
    program: &VerifiedFreshConstructorComputationV1,
) -> RewriteInventoryResult<()> {
    if inventory.manifest_digest() != program.manifest_digest()
        || inventory.normalizer_protocol_digest()
            != program
                .termination_certificate()
                .kernel_normalizer_protocol_digest()
        || inventory.predecessor_boundary().digest() != program.boundary_signature_digest()
        || inventory.successor_boundary().digest() != program.extended_signature().digest()
        || inventory.successor_boundary().declarations()
            != program.extended_signature().declarations()
        || inventory.exact_extension().predecessor_boundary_digest()
            != inventory.predecessor_boundary().digest()
        || inventory.exact_extension().successor_boundary_digest()
            != inventory.successor_boundary().digest()
    {
        return Err(AuditUnknownReason::ManifestMismatch);
    }
    Ok(())
}

fn verify_exact_fresh_declaration(
    inventory: &VerifiedPublicAuditInventoryV1,
    program: &VerifiedFreshConstructorComputationV1,
) -> RewriteInventoryResult<()> {
    let new_declarations = inventory.exact_extension().new_declarations();
    if new_declarations.len() != 1 || new_declarations.first() != Some(program.fresh_head()) {
        return Err(AuditUnknownReason::IncompleteEnumeration);
    }
    let inventory_declaration = inventory
        .public_declaration(program.fresh_head())
        .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
    let program_declaration = program
        .extended_signature()
        .declarations()
        .iter()
        .find(|declaration| &declaration.id == program.fresh_head())
        .ok_or(AuditUnknownReason::IncompleteEnumeration)?;
    if !inventory.is_successor_new_declaration(program.fresh_head())
        || inventory_declaration.normalized() != program_declaration
        || inventory_declaration.normalized().body.is_some()
    {
        return Err(AuditUnknownReason::ManifestMismatch);
    }
    Ok(())
}

fn verify_program_certificates(
    program: &VerifiedFreshConstructorComputationV1,
) -> RewriteInventoryResult<()> {
    let termination = program.termination_certificate();
    let confluence = program.confluence_certificate();
    let scope = RestrictedQ0CertificateScopeV1::KernelNormalFormsAndFreshNullaryConstructorSpines;
    if termination.scope() != scope
        || confluence.scope() != scope
        || termination.program_digest() != program.program_digest()
        || confluence.program_digest() != program.program_digest()
        || termination.kernel_normalizer_protocol_digest()
            != confluence.kernel_normalizer_protocol_digest()
        || !termination.fresh_rhs_contains_no_fresh_head()
        || !termination.each_fresh_step_removes_its_root_head()
        || !termination.dependency_graph_is_acyclic()
        || !confluence.patterns_are_left_linear()
        || !confluence.constructor_patterns_are_pairwise_disjoint()
        || !confluence.fresh_head_has_no_delta_body()
    {
        return Err(AuditUnknownReason::ManifestMismatch);
    }
    Ok(())
}

fn reject_duplicate_program_rules(
    rules: &[VerifiedFreshConstructorRuleV1],
) -> RewriteInventoryResult<()> {
    for (left_index, left) in rules.iter().enumerate() {
        if rules
            .iter()
            .skip(left_index + 1)
            .any(|right| rules_have_same_normalized_equation(left, right))
        {
            return Err(AuditUnknownReason::ProvenanceCollision);
        }
    }
    Ok(())
}

fn rules_have_same_normalized_equation(
    left: &VerifiedFreshConstructorRuleV1,
    right: &VerifiedFreshConstructorRuleV1,
) -> bool {
    left.fresh_head() == right.fresh_head()
        && left.context() == right.context()
        && left.left() == right.left()
        && left.right() == right.right()
        && left.ty() == right.ty()
}

fn fresh_rule_matches_equation(
    fresh_head: &GlobalId,
    rule: &VerifiedFreshConstructorRuleV1,
    equation: &VerifiedPublicEquationV1,
) -> bool {
    if rule.fresh_head() != fresh_head || equation.owner_head() != fresh_head {
        return false;
    }
    matches!(
        equation.normalized(),
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } if context == rule.context()
            && left == rule.left()
            && right == rule.right()
            && ty == rule.ty()
    )
}

fn finish_typed_rewrite_inventory(
    fresh: VerifiedFreshRewriteCoverageV1,
    projections: VerifiedDescriptorProjectionInventoryV1,
) -> VerifiedTypedRewriteInventoryV1 {
    let mut inventory = VerifiedTypedRewriteInventoryV1 {
        inventory_digest: fresh.inventory_digest,
        inventory_coverage_digest: fresh.inventory_coverage_digest,
        manifest_digest: fresh.manifest_digest,
        predecessor_history_digest: fresh.predecessor_history_digest,
        predecessor_boundary_digest: fresh.predecessor_boundary_digest,
        successor_boundary_digest: fresh.successor_boundary_digest,
        exact_extension_digest: fresh.exact_extension_digest,
        fresh_program_digest: fresh.fresh_program_digest,
        normalizer_protocol_digest: fresh.normalizer_protocol_digest,
        descriptor_projection_inventory_digest: projections.digest,
        fresh_head: fresh.fresh_head,
        entries: fresh.entries,
        digest: Digest::of_bytes(b"pending-typed-rewrite-inventory"),
    };
    inventory.digest = Digest::of_canonical(
        "pen-semantic-audit/verified-typed-rewrite-inventory/v1",
        &inventory,
    );
    inventory
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::{
        DemandContractIdV1, DemandFamilyIdV1, ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
        PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION, PublicDependencyUseV1, PublicSubjectV1,
        UncheckedForcedProjectionV1, UncheckedOriginCutoffQ3RegistryV1,
        UncheckedPredecessorDemandContractV1, UncheckedPublicAuditInventoryV1,
        UncheckedPublicAvailabilityClaimV1, UncheckedPublicDeclarationV1,
        UncheckedPublicDependencyDagV1, UncheckedPublicEquationV1, UncheckedPublicEventCensusV1,
        UncheckedPublicGroupV1, UncheckedPublicHistoryStepV1,
        UncheckedSourceNormalizedDeclarationV1, verify_public_audit_inventory_v1,
    };
    use crate::manifest::{
        VerifiedSemanticAuditManifestV1, proposed_semantic_audit_manifest_v1,
        verify_semantic_audit_manifest_v1,
    };
    use crate::model::{
        DemandOutputIdV1, EventIdV1, PublicAvailabilityV1, SourceNormalizedJudgmentV1,
    };
    use crate::normalizer::{
        FreshConstructorClauseV1, FreshConstructorComputationRequestV1,
        verify_fresh_constructor_computation_v1,
    };
    use pen_kernel::{Declaration, Kernel, KernelLimits, UncheckedSignature};
    use std::collections::BTreeSet;

    struct Fixture {
        inventory: VerifiedPublicAuditInventoryV1,
        program: VerifiedFreshConstructorComputationV1,
        expected_source_identities: BTreeSet<Digest>,
        expected_ports: BTreeSet<DemandPortKeyV1>,
    }

    fn digest(label: &str) -> Digest {
        Digest::of_bytes(label.as_bytes())
    }

    fn global(label: &str) -> GlobalId {
        GlobalId(digest(&format!("global/{label}")))
    }

    fn equation_id(label: &str) -> EquationIdV1 {
        EquationIdV1(digest(&format!("equation/{label}")))
    }

    fn event(label: &str) -> EventIdV1 {
        EventIdV1(digest(&format!("event/{label}")))
    }

    fn contract(label: &str) -> DemandContractIdV1 {
        DemandContractIdV1(digest(&format!("contract/{label}")))
    }

    fn port(label: &str) -> DemandPortKeyV1 {
        DemandPortKeyV1 {
            family: DemandFamilyIdV1(digest(&format!("family/{label}"))),
            output: DemandOutputIdV1(digest(&format!("output/{label}"))),
        }
    }

    fn manifest() -> VerifiedSemanticAuditManifestV1 {
        let AuditDecision::Proven(manifest) =
            verify_semantic_audit_manifest_v1(&proposed_semantic_audit_manifest_v1())
        else {
            panic!("proposed semantic manifest verifies");
        };
        manifest
    }

    fn source_declaration(declaration: &Declaration) -> UncheckedSourceNormalizedDeclarationV1 {
        UncheckedSourceNormalizedDeclarationV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-declaration/v1",
                declaration,
            ),
            source: declaration.clone(),
            claimed_normalized: declaration.clone(),
        }
    }

    fn source_judgment(judgment: GenericJudgmentV1) -> SourceNormalizedJudgmentV1 {
        SourceNormalizedJudgmentV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-judgment/v1",
                &judgment,
            ),
            source: judgment.clone(),
            claimed_normalized: judgment,
        }
    }

    fn fresh_judgment(head: &GlobalId, constructor: &GlobalId) -> GenericJudgmentV1 {
        GenericJudgmentV1::Equation {
            context: DependentContext(vec![Term::UnitType]),
            left: Term::Apply {
                function: Box::new(Term::Apply {
                    function: Box::new(Term::Global { id: head.clone() }),
                    argument: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::Global {
                    id: constructor.clone(),
                }),
            },
            right: Term::Var { index: 0 },
            ty: Term::UnitType,
        }
    }

    fn fixture(program_labels: &[&str], inventory_labels: &[&str]) -> Fixture {
        fixture_with_projection(program_labels, inventory_labels, false)
    }

    fn fixture_with_projection(
        program_labels: &[&str],
        inventory_labels: &[&str],
        include_forced_projection: bool,
    ) -> Fixture {
        let manifest = manifest();
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let fresh_head = global("fresh-head");
        let predecessor_event = event("predecessor");
        let successor_event = event("successor");
        let predecessor_group = global("group/predecessor");
        let successor_group = global("group/successor");

        let all_labels = program_labels
            .iter()
            .chain(inventory_labels)
            .copied()
            .collect::<BTreeSet<_>>();
        let constructors = all_labels
            .iter()
            .map(|label| Declaration {
                id: global(&format!("constructor/{label}")),
                ty: Term::UnitType,
                body: None,
            })
            .collect::<Vec<_>>();
        let record_owner = global("forced-projection/record-owner");
        let projection = global("forced-projection/projection");
        let mut predecessor_declarations = constructors.clone();
        if include_forced_projection {
            predecessor_declarations.extend([
                Declaration {
                    id: record_owner.clone(),
                    ty: Term::Sort { level: 0 },
                    body: None,
                },
                Declaration {
                    id: projection.clone(),
                    ty: Term::Global {
                        id: record_owner.clone(),
                    },
                    body: None,
                },
            ]);
        }
        let predecessor_source = UncheckedSignature {
            declarations: predecessor_declarations.clone(),
        };
        let predecessor_boundary = kernel
            .verify_signature(&predecessor_source)
            .expect("predecessor signature");
        let fresh_declaration = Declaration {
            id: fresh_head.clone(),
            ty: Term::Pi {
                parameter: Box::new(Term::UnitType),
                body: Box::new(Term::Pi {
                    parameter: Box::new(Term::UnitType),
                    body: Box::new(Term::UnitType),
                }),
            },
            body: None,
        };
        let request = FreshConstructorComputationRequestV1 {
            fresh_declaration: fresh_declaration.clone(),
            clauses: program_labels
                .iter()
                .map(|label| FreshConstructorClauseV1 {
                    constructor: global(&format!("constructor/{label}")),
                    scrutinee_parameter_ordinal: 1,
                })
                .collect(),
        };
        let AuditDecision::Proven(program) = verify_fresh_constructor_computation_v1(
            &manifest,
            &kernel,
            &predecessor_boundary,
            &request,
        ) else {
            panic!("fresh program verifies");
        };

        let successor_source = UncheckedSignature {
            declarations: predecessor_declarations
                .iter()
                .cloned()
                .chain(std::iter::once(fresh_declaration.clone()))
                .collect(),
        };
        let mut equation_wires = Vec::new();
        let mut equation_ids = Vec::new();
        let mut demand_wires = Vec::new();
        let mut demand_ids = Vec::new();
        let mut dependencies = Vec::new();
        let mut availability = Vec::new();
        let mut expected_source_identities = BTreeSet::new();
        let mut expected_ports = BTreeSet::new();
        if include_forced_projection {
            let dependency = PublicDependencyUseV1 {
                dependent: PublicSubjectV1::Declaration {
                    declaration: projection.clone(),
                },
                prerequisite: record_owner.clone(),
            };
            dependencies.push(dependency.clone());
            availability.push(UncheckedPublicAvailabilityClaimV1 {
                dependency,
                claimed: PublicAvailabilityV1::PredecessorPublicExport {
                    target: record_owner.clone(),
                },
            });
        }
        for (index, label) in inventory_labels.iter().enumerate() {
            let constructor = global(&format!("constructor/{label}"));
            let equation = equation_id(&format!("{index}/{label}"));
            let demand = contract(&format!("{index}/{label}"));
            let demand_port = port(&format!("{index}/{label}"));
            let normalized = fresh_judgment(&fresh_head, &constructor);
            let source_to_normal = source_judgment(normalized);
            expected_source_identities.insert(source_to_normal.source_identity.clone());
            expected_ports.insert(demand_port.clone());
            equation_wires.push(UncheckedPublicEquationV1 {
                equation: equation.clone(),
                owner_head: fresh_head.clone(),
                origin: successor_event.clone(),
                source_to_normal,
                demand_port: Some(demand_port.clone()),
            });
            equation_ids.push(equation.clone());
            demand_wires.push(UncheckedPredecessorDemandContractV1 {
                contract: demand.clone(),
                origin: predecessor_event.clone(),
                port: demand_port,
                required_judgment: source_judgment(GenericJudgmentV1::Term {
                    context: DependentContext::default(),
                    term: Term::Unit,
                    ty: Term::UnitType,
                }),
            });
            demand_ids.push(demand);
            for prerequisite in [constructor, fresh_head.clone()] {
                let dependency = PublicDependencyUseV1 {
                    dependent: PublicSubjectV1::Equation {
                        equation: equation.clone(),
                    },
                    prerequisite: prerequisite.clone(),
                };
                let claimed = if prerequisite == fresh_head {
                    PublicAvailabilityV1::DependencyPriorExport {
                        target: prerequisite,
                    }
                } else {
                    PublicAvailabilityV1::PredecessorPublicExport {
                        target: prerequisite,
                    }
                };
                dependencies.push(dependency.clone());
                availability.push(UncheckedPublicAvailabilityClaimV1 {
                    dependency,
                    claimed,
                });
            }
        }

        let mut declaration_wires = predecessor_declarations
            .iter()
            .map(|declaration| UncheckedPublicDeclarationV1 {
                declaration: declaration.id.clone(),
                origin: predecessor_event.clone(),
                group: predecessor_group.clone(),
                source_to_normal: source_declaration(declaration),
            })
            .collect::<Vec<_>>();
        declaration_wires.push(UncheckedPublicDeclarationV1 {
            declaration: fresh_head.clone(),
            origin: successor_event.clone(),
            group: successor_group.clone(),
            source_to_normal: source_declaration(&fresh_declaration),
        });

        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history: vec![UncheckedPublicHistoryStepV1 {
                census: UncheckedPublicEventCensusV1 {
                    event: predecessor_event.clone(),
                    added_groups: vec![predecessor_group.clone()],
                    added_declarations: predecessor_declarations
                        .iter()
                        .map(|declaration| declaration.id.clone())
                        .collect(),
                    added_equations: Vec::new(),
                    added_demand_contracts: demand_ids,
                    added_forced_projections: include_forced_projection
                        .then_some(projection.clone())
                        .into_iter()
                        .collect(),
                },
                successor_boundary: predecessor_source.clone(),
            }],
            predecessor_boundary: predecessor_source,
            successor_event: UncheckedPublicEventCensusV1 {
                event: successor_event.clone(),
                added_groups: vec![successor_group.clone()],
                added_declarations: vec![fresh_head.clone()],
                added_equations: equation_ids,
                added_demand_contracts: Vec::new(),
                added_forced_projections: Vec::new(),
            },
            successor_boundary: successor_source,
            declaration_groups: vec![
                UncheckedPublicGroupV1 {
                    group: predecessor_group,
                    origin: predecessor_event.clone(),
                    declarations: predecessor_declarations
                        .iter()
                        .map(|declaration| declaration.id.clone())
                        .collect(),
                },
                UncheckedPublicGroupV1 {
                    group: successor_group,
                    origin: successor_event,
                    declarations: vec![fresh_head],
                },
            ],
            declarations: declaration_wires,
            equations: equation_wires,
            forced_projections: include_forced_projection
                .then_some(UncheckedForcedProjectionV1 {
                    projection,
                    record_owner,
                    field_ordinal: 0,
                    origin: predecessor_event.clone(),
                })
                .into_iter()
                .collect(),
            predecessor_demand_contracts: demand_wires,
            public_availability: availability,
            dependency_dag: UncheckedPublicDependencyDagV1 {
                edges: dependencies,
            },
            q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
                schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
                origin_cutoff: Some(predecessor_event),
                entries: Vec::new(),
            },
        };
        let AuditDecision::Proven(inventory) =
            verify_public_audit_inventory_v1(&manifest, &kernel, &wire)
        else {
            panic!("public inventory verifies");
        };
        Fixture {
            inventory,
            program,
            expected_source_identities,
            expected_ports,
        }
    }

    #[test]
    fn exact_fresh_rules_and_sealed_equations_match_one_to_one() {
        let fixture = fixture(&["left", "right"], &["right", "left"]);
        let coverage = verify_fresh_rewrite_coverage(&fixture.inventory, &fixture.program)
            .expect("fresh coverage is exact");
        assert_eq!(coverage.entries.len(), 2);
        assert_eq!(
            coverage
                .entries
                .iter()
                .map(|entry| entry.source_identity().clone())
                .collect::<BTreeSet<_>>(),
            fixture.expected_source_identities
        );
        assert_eq!(
            coverage
                .entries
                .iter()
                .filter_map(|entry| entry.demand_port().cloned())
                .collect::<BTreeSet<_>>(),
            fixture.expected_ports
        );
        assert_eq!(coverage.inventory_digest, *fixture.inventory.digest());
        assert_eq!(
            coverage.fresh_program_digest,
            *fixture.program.program_digest()
        );
    }

    #[test]
    fn omitted_and_extra_fresh_rules_fail_closed() {
        let omitted = fixture(&["left"], &["left", "right"]);
        assert_eq!(
            verify_fresh_rewrite_coverage(&omitted.inventory, &omitted.program)
                .expect_err("omitted rule must fail"),
            AuditUnknownReason::IncompleteEnumeration
        );

        let extra = fixture(&["left", "right"], &["left"]);
        assert_eq!(
            verify_fresh_rewrite_coverage(&extra.inventory, &extra.program)
                .expect_err("extra rule must fail"),
            AuditUnknownReason::IncompleteEnumeration
        );
    }

    #[test]
    fn duplicate_or_mismatched_sealed_equations_fail_closed() {
        let duplicate = fixture(&["left", "right"], &["left", "left"]);
        assert_eq!(
            verify_fresh_rewrite_coverage(&duplicate.inventory, &duplicate.program)
                .expect_err("duplicate equation must fail"),
            AuditUnknownReason::ProvenanceCollision
        );

        let mismatch = fixture(&["left"], &["right"]);
        assert_eq!(
            verify_fresh_rewrite_coverage(&mismatch.inventory, &mismatch.program)
                .expect_err("mismatched equation must fail"),
            AuditUnknownReason::IncompleteEnumeration
        );
    }

    #[test]
    fn public_compiler_accepts_only_the_verified_empty_projection_census() {
        let empty = fixture(&["only"], &["only"]);
        let AuditDecision::Proven(typed) =
            compile_typed_rewrite_inventory_v1(&empty.inventory, &empty.program)
        else {
            panic!("verified-empty projection census should compile");
        };
        assert_eq!(typed.entries().len(), 1);

        let nonempty = fixture_with_projection(&["only"], &["only"], true);
        assert!(matches!(
            compile_typed_rewrite_inventory_v1(&nonempty.inventory, &nonempty.program),
            AuditDecision::Unknown(AuditUnknownReason::MissingDescriptorProjectionInventory)
        ));
    }

    #[test]
    fn typed_inventory_does_not_mint_rewrite_admissibility() {
        let fixture = fixture(&["only"], &["only"]);
        let AuditDecision::Proven(typed) =
            compile_typed_rewrite_inventory_v1(&fixture.inventory, &fixture.program)
        else {
            panic!("typed rewrite inventory should compile");
        };
        assert!(matches!(
            attempt_rewrite_admissibility_v1(&typed),
            AuditDecision::Unknown(AuditUnknownReason::MissingRewriteAdmissibilityTheorem)
        ));
    }
}
