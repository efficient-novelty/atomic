//! Verifier-minted ordinary delta/beta reconstruction evidence.
//!
//! This module derives its entire input from an opaque verified public
//! inventory. Callers select an equation identifier but cannot serialize a
//! reduction trace or a substitution claim into a verified capability.

use crate::fragment::{
    LambdaUnitSyntaxViolation, lambda_unit_judgment_syntax_violation,
    lambda_unit_term_syntax_violation,
};
use crate::inventory::VerifiedPublicAuditInventoryV1;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, Q0RuleV1,
    SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1, VerifiedSemanticAuditManifestV1,
};
use crate::model::{EquationIdV1, GenericJudgmentV1};
use crate::normalizer::normalize_generated_judgment_v1;
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Digest, GlobalId, Kernel, KernelError, OpenJudgment, Term,
};
use serde::Serialize;

/// One checked capture-avoiding substitution in a left-to-right beta spine.
///
/// Values of this type are minted only as fields of
/// [`VerifiedOrdinaryBetaDerivationV1`]. They intentionally have no
/// `Deserialize` implementation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedSequentialSubstitutionV1 {
    argument_ordinal: u16,
    argument: Term,
    parameter_type: Term,
    body_before: Term,
    body_after: Term,
    result_type_before: Term,
    result_type_after: Term,
}

impl VerifiedSequentialSubstitutionV1 {
    pub fn argument_ordinal(&self) -> u16 {
        self.argument_ordinal
    }

    pub fn argument(&self) -> &Term {
        &self.argument
    }

    pub fn parameter_type(&self) -> &Term {
        &self.parameter_type
    }

    pub fn body_before(&self) -> &Term {
        &self.body_before
    }

    pub fn body_after(&self) -> &Term {
        &self.body_after
    }

    pub fn result_type_before(&self) -> &Term {
        &self.result_type_before
    }

    pub fn result_type_after(&self) -> &Term {
        &self.result_type_after
    }
}

impl CanonicalEncode for VerifiedSequentialSubstitutionV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        encoder.u16(self.argument_ordinal);
        self.argument.encode_canonical(encoder);
        self.parameter_type.encode_canonical(encoder);
        self.body_before.encode_canonical(encoder);
        self.body_after.encode_canonical(encoder);
        self.result_type_before.encode_canonical(encoder);
        self.result_type_after.encode_canonical(encoder);
    }
}

/// A replayed ordinary-reduction step.
///
/// `BetaContract` records the focused redex. When that redex occurs beneath
/// pending application nodes, the following `ApplicationCongruence` step
/// records the exact whole-term lift. `KernelCongruenceNormalization` binds
/// any remaining ordinary kernel normalization of the reconstructed
/// contractum.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(tag = "step", rename_all = "snake_case")]
pub enum VerifiedOrdinaryBetaReductionStepV1 {
    DeltaUnfoldHead {
        owner_head: GlobalId,
        before: Term,
        after: Term,
    },
    BetaContract {
        argument_ordinal: u16,
        redex: Term,
        contractum: Term,
    },
    ApplicationCongruence {
        argument_ordinal: u16,
        pending_argument_count: u16,
        before: Term,
        after: Term,
    },
    KernelCongruenceNormalization {
        before: Term,
        after: Term,
        normalizer_protocol_digest: Digest,
    },
}

impl CanonicalEncode for VerifiedOrdinaryBetaReductionStepV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        match self {
            Self::DeltaUnfoldHead {
                owner_head,
                before,
                after,
            } => {
                encoder.tag(0);
                owner_head.encode_canonical(encoder);
                before.encode_canonical(encoder);
                after.encode_canonical(encoder);
            }
            Self::BetaContract {
                argument_ordinal,
                redex,
                contractum,
            } => {
                encoder.tag(1);
                encoder.u16(*argument_ordinal);
                redex.encode_canonical(encoder);
                contractum.encode_canonical(encoder);
            }
            Self::ApplicationCongruence {
                argument_ordinal,
                pending_argument_count,
                before,
                after,
            } => {
                encoder.tag(2);
                encoder.u16(*argument_ordinal);
                encoder.u16(*pending_argument_count);
                before.encode_canonical(encoder);
                after.encode_canonical(encoder);
            }
            Self::KernelCongruenceNormalization {
                before,
                after,
                normalizer_protocol_digest,
            } => {
                encoder.tag(3);
                before.encode_canonical(encoder);
                after.encode_canonical(encoder);
                normalizer_protocol_digest.encode_canonical(encoder);
            }
        }
    }
}

/// Opaque proof capability for one exact successor-public ordinary
/// delta/beta equation.
///
/// All fields are private and the type intentionally has no `Deserialize`
/// implementation. The only constructor is
/// [`verify_ordinary_beta_derivation_v1`].
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedOrdinaryBetaDerivationV1 {
    inventory_digest: Digest,
    manifest_digest: Digest,
    normalizer_protocol_digest: Digest,
    successor_boundary_digest: Digest,
    equation: EquationIdV1,
    equation_source_identity: Digest,
    owner_head: GlobalId,
    owner_source_identity: Digest,
    owner_was_predecessor_public: bool,
    arguments: Vec<Term>,
    reconstructed_source: GenericJudgmentV1,
    replayed_normalized: GenericJudgmentV1,
    substitutions: Vec<VerifiedSequentialSubstitutionV1>,
    reduction_steps: Vec<VerifiedOrdinaryBetaReductionStepV1>,
    derivation_digest: Digest,
}

impl VerifiedOrdinaryBetaDerivationV1 {
    pub fn inventory_digest(&self) -> &Digest {
        &self.inventory_digest
    }

    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.normalizer_protocol_digest
    }

    pub fn successor_boundary_digest(&self) -> &Digest {
        &self.successor_boundary_digest
    }

    pub fn equation(&self) -> &EquationIdV1 {
        &self.equation
    }

    pub fn equation_source_identity(&self) -> &Digest {
        &self.equation_source_identity
    }

    pub fn owner_head(&self) -> &GlobalId {
        &self.owner_head
    }

    pub fn owner_source_identity(&self) -> &Digest {
        &self.owner_source_identity
    }

    pub fn owner_was_predecessor_public(&self) -> bool {
        self.owner_was_predecessor_public
    }

    pub fn arguments(&self) -> &[Term] {
        &self.arguments
    }

    pub fn reconstructed_source(&self) -> &GenericJudgmentV1 {
        &self.reconstructed_source
    }

    pub fn replayed_normalized(&self) -> &GenericJudgmentV1 {
        &self.replayed_normalized
    }

    pub fn substitutions(&self) -> &[VerifiedSequentialSubstitutionV1] {
        &self.substitutions
    }

    pub fn reduction_steps(&self) -> &[VerifiedOrdinaryBetaReductionStepV1] {
        &self.reduction_steps
    }

    pub fn derivation_digest(&self) -> &Digest {
        &self.derivation_digest
    }
}

impl CanonicalEncode for VerifiedOrdinaryBetaDerivationV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory_digest.encode_canonical(encoder);
        self.manifest_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.successor_boundary_digest.encode_canonical(encoder);
        self.equation.encode_canonical(encoder);
        self.equation_source_identity.encode_canonical(encoder);
        self.owner_head.encode_canonical(encoder);
        self.owner_source_identity.encode_canonical(encoder);
        encoder.tag(u8::from(self.owner_was_predecessor_public));
        encoder.sequence(&self.arguments);
        self.reconstructed_source.encode_canonical(encoder);
        self.replayed_normalized.encode_canonical(encoder);
        encoder.sequence(&self.substitutions);
        encoder.sequence(&self.reduction_steps);
        self.derivation_digest.encode_canonical(encoder);
    }
}

struct OrdinaryBetaDigestMaterial<'a> {
    inventory_digest: &'a Digest,
    manifest_digest: &'a Digest,
    normalizer_protocol_digest: &'a Digest,
    successor_boundary_digest: &'a Digest,
    equation: &'a EquationIdV1,
    equation_source_identity: &'a Digest,
    owner_head: &'a GlobalId,
    owner_source_identity: &'a Digest,
    owner_was_predecessor_public: bool,
    arguments: &'a [Term],
    reconstructed_source: &'a GenericJudgmentV1,
    replayed_normalized: &'a GenericJudgmentV1,
    substitutions: &'a [VerifiedSequentialSubstitutionV1],
    reduction_steps: &'a [VerifiedOrdinaryBetaReductionStepV1],
}

impl CanonicalEncode for OrdinaryBetaDigestMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.inventory_digest.encode_canonical(encoder);
        self.manifest_digest.encode_canonical(encoder);
        self.normalizer_protocol_digest.encode_canonical(encoder);
        self.successor_boundary_digest.encode_canonical(encoder);
        self.equation.encode_canonical(encoder);
        self.equation_source_identity.encode_canonical(encoder);
        self.owner_head.encode_canonical(encoder);
        self.owner_source_identity.encode_canonical(encoder);
        encoder.tag(u8::from(self.owner_was_predecessor_public));
        encoder.sequence(self.arguments);
        self.reconstructed_source.encode_canonical(encoder);
        self.replayed_normalized.encode_canonical(encoder);
        encoder.sequence(self.substitutions);
        encoder.sequence(self.reduction_steps);
    }
}

#[derive(Clone, Debug)]
enum OrdinaryBetaFailure {
    Outside(OutsideFragmentReason),
    Unknown(AuditUnknownReason),
}

type OrdinaryBetaResult<T> = Result<T, OrdinaryBetaFailure>;

fn decision<T>(result: OrdinaryBetaResult<T>) -> AuditDecision<T> {
    match result {
        Ok(value) => AuditDecision::Proven(value),
        Err(OrdinaryBetaFailure::Outside(reason)) => AuditDecision::OutsideFragment(reason),
        Err(OrdinaryBetaFailure::Unknown(reason)) => AuditDecision::Unknown(reason),
    }
}

fn unknown(reason: AuditUnknownReason) -> OrdinaryBetaFailure {
    OrdinaryBetaFailure::Unknown(reason)
}

fn missing_derivation() -> OrdinaryBetaFailure {
    unknown(AuditUnknownReason::MissingOrdinaryBetaDerivation)
}

fn kernel_failure(error: KernelError) -> OrdinaryBetaFailure {
    match error {
        KernelError::ResourceExhausted(_) => unknown(AuditUnknownReason::ResourceExhausted),
        _ => unknown(AuditUnknownReason::KernelCouldNotCertify),
    }
}

/// Reconstruct an exact ordinary delta/beta equation from verified inventory
/// declarations and equations.
///
/// The selected equation must be successor-public, have no demand-port
/// association, be oriented with an application spine headed by its exact
/// owner, and have the exact capture-avoiding sequential contractum and
/// dependent result type on its source side. The kernel then independently
/// replays definitional equality and the inventory-bound normal form.
pub fn verify_ordinary_beta_derivation_v1(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    equation_id: &EquationIdV1,
) -> AuditDecision<VerifiedOrdinaryBetaDerivationV1> {
    decision(verify_ordinary_beta_derivation_inner(
        manifest,
        kernel,
        inventory,
        equation_id,
    ))
}

fn verify_ordinary_beta_derivation_inner(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    inventory: &VerifiedPublicAuditInventoryV1,
    equation_id: &EquationIdV1,
) -> OrdinaryBetaResult<VerifiedOrdinaryBetaDerivationV1> {
    let required = [
        Q0RuleV1::DeBruijn,
        Q0RuleV1::SequentialSubstitution,
        Q0RuleV1::Beta,
        Q0RuleV1::ProvenancePreservingDelta,
    ];
    if inventory.manifest_digest() != manifest.candidate_digest()
        || inventory.normalizer_protocol_digest() != &kernel.normalizer_protocol_digest()
        || !required
            .iter()
            .all(|rule| manifest.manifest().q0_rules.contains(rule))
        || !manifest.manifest().q0_eta_registry_empty
    {
        return Err(unknown(AuditUnknownReason::ManifestMismatch));
    }
    if manifest.manifest().profile_id == SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1
        && let Some(violation) = lambda_unit_ordinary_beta_syntax_violation(
            inventory,
            &manifest.manifest().universe_levels,
        )
    {
        return Err(OrdinaryBetaFailure::Outside(violation.outside_reason()));
    }

    let equation = inventory
        .equations()
        .iter()
        .find(|candidate| candidate.equation() == equation_id)
        .ok_or_else(|| unknown(AuditUnknownReason::IncompleteEnumeration))?;
    if equation.is_predecessor_public() || equation.demand_port().is_some() {
        return Err(missing_derivation());
    }
    let owner = inventory
        .public_declaration(equation.owner_head())
        .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?;
    let body = owner
        .normalized()
        .body
        .as_ref()
        .ok_or_else(missing_derivation)?;
    if owner.source().body.is_none() {
        return Err(missing_derivation());
    }
    let boundary_owner = inventory
        .successor_boundary()
        .declarations()
        .iter()
        .find(|declaration| declaration.id == *equation.owner_head())
        .ok_or_else(|| unknown(AuditUnknownReason::IncompleteSupport))?;
    if boundary_owner != owner.normalized() {
        return Err(unknown(AuditUnknownReason::NormalizationFailure));
    }

    let GenericJudgmentV1::Equation {
        context,
        left,
        right,
        ty,
    } = equation.source()
    else {
        return Err(missing_derivation());
    };
    let (head, arguments) = application_spine(left);
    if head
        != (Term::Global {
            id: equation.owner_head().clone(),
        })
        || arguments.len() > usize::from(manifest.manifest().maximum_context_entries)
    {
        return Err(missing_derivation());
    }
    if arguments.is_empty()
        && matches!(
            body,
            Term::Global { .. } | Term::Sort { .. } | Term::UnitType | Term::Unit
        )
    {
        // Alias and ambient-first-export classification requires different
        // inventory evidence; body shape alone cannot mint ordinary beta.
        return Err(missing_derivation());
    }

    let mut budget = ReplayBudget::new(kernel);
    let delta_after = apply_spine(body.clone(), arguments.iter().cloned());
    let mut reduction_steps = vec![VerifiedOrdinaryBetaReductionStepV1::DeltaUnfoldHead {
        owner_head: equation.owner_head().clone(),
        before: left.clone(),
        after: delta_after,
    }];
    let mut substitutions = Vec::with_capacity(arguments.len());
    let mut residual_body = body.clone();
    let mut residual_type = owner.normalized().ty.clone();

    for (index, argument) in arguments.iter().enumerate() {
        let ordinal =
            u16::try_from(index).map_err(|_| unknown(AuditUnknownReason::ResourceExhausted))?;
        let Term::Lambda {
            parameter_type,
            body: lambda_body,
        } = &residual_body
        else {
            return Err(missing_derivation());
        };
        let Term::Pi {
            parameter,
            body: result_body,
        } = &residual_type
        else {
            return Err(missing_derivation());
        };
        if parameter_type.as_ref() != parameter.as_ref() {
            return Err(missing_derivation());
        }

        let checked_argument = kernel
            .verify_open_judgment(
                inventory.successor_boundary(),
                &OpenJudgment::HasType {
                    context: context.clone(),
                    term: argument.clone(),
                    ty: parameter.as_ref().clone(),
                },
            )
            .map_err(kernel_failure)?;
        let OpenJudgment::HasType {
            context: checked_context,
            term: checked_term,
            ty: checked_type,
        } = checked_argument
        else {
            return Err(unknown(AuditUnknownReason::KernelCouldNotCertify));
        };
        if checked_context != *context || checked_term != *argument || checked_type != **parameter {
            // The explicit small-step trace is over normal arguments and
            // parameter types. A future congruence proof can safely widen it.
            return Err(missing_derivation());
        }

        let body_before = residual_body.clone();
        let type_before = residual_type.clone();
        let body_after = subst_top(lambda_body, argument, &mut budget, 0)?;
        let type_after = subst_top(result_body, argument, &mut budget, 0)?;
        let redex = Term::Apply {
            function: Box::new(body_before.clone()),
            argument: Box::new(argument.clone()),
        };
        reduction_steps.push(VerifiedOrdinaryBetaReductionStepV1::BetaContract {
            argument_ordinal: ordinal,
            redex: redex.clone(),
            contractum: body_after.clone(),
        });

        let pending = &arguments[index + 1..];
        if !pending.is_empty() {
            let pending_argument_count = u16::try_from(pending.len())
                .map_err(|_| unknown(AuditUnknownReason::ResourceExhausted))?;
            reduction_steps.push(VerifiedOrdinaryBetaReductionStepV1::ApplicationCongruence {
                argument_ordinal: ordinal,
                pending_argument_count,
                before: apply_spine(redex, pending.iter().cloned()),
                after: apply_spine(body_after.clone(), pending.iter().cloned()),
            });
        }
        substitutions.push(VerifiedSequentialSubstitutionV1 {
            argument_ordinal: ordinal,
            argument: argument.clone(),
            parameter_type: parameter.as_ref().clone(),
            body_before,
            body_after: body_after.clone(),
            result_type_before: type_before,
            result_type_after: type_after.clone(),
        });
        residual_body = body_after;
        residual_type = type_after;
    }

    if right != &residual_body || ty != &residual_type {
        return Err(missing_derivation());
    }
    let reconstructed_source = GenericJudgmentV1::Equation {
        context: context.clone(),
        left: left.clone(),
        right: residual_body.clone(),
        ty: residual_type.clone(),
    };
    if &reconstructed_source != equation.source() {
        return Err(missing_derivation());
    }

    let checked_equality = kernel
        .verify_open_judgment(
            inventory.successor_boundary(),
            &OpenJudgment::DefinitionallyEqual {
                context: context.clone(),
                left: left.clone(),
                right: right.clone(),
                ty: ty.clone(),
            },
        )
        .map_err(kernel_failure)?;
    let OpenJudgment::DefinitionallyEqual {
        context: normalized_context,
        left: normalized_left,
        right: normalized_right,
        ty: normalized_type,
    } = checked_equality
    else {
        return Err(unknown(AuditUnknownReason::KernelCouldNotCertify));
    };
    let kernel_normalized = GenericJudgmentV1::Equation {
        context: normalized_context,
        left: normalized_left.clone(),
        right: normalized_right.clone(),
        ty: normalized_type,
    };
    if &kernel_normalized != equation.normalized() {
        return Err(unknown(AuditUnknownReason::NormalizationFailure));
    }

    let replayed_normalized = match normalize_generated_judgment_v1(
        manifest,
        kernel,
        inventory.successor_boundary(),
        &reconstructed_source,
        None,
    ) {
        AuditDecision::Proven(value) => value,
        AuditDecision::OutsideFragment(reason) => {
            return Err(OrdinaryBetaFailure::Outside(reason));
        }
        AuditDecision::Unknown(reason) => return Err(unknown(reason)),
    };
    if replayed_normalized != *equation.normalized() || normalized_left != normalized_right {
        return Err(unknown(AuditUnknownReason::NormalizationFailure));
    }
    if residual_body != normalized_right {
        reduction_steps.push(
            VerifiedOrdinaryBetaReductionStepV1::KernelCongruenceNormalization {
                before: residual_body,
                after: normalized_right,
                normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
            },
        );
    }

    let owner_was_predecessor_public =
        inventory.contains_predecessor_declaration(equation.owner_head());
    if !owner_was_predecessor_public
        && !inventory.is_successor_new_declaration(equation.owner_head())
    {
        return Err(unknown(AuditUnknownReason::IncompleteSupport));
    }
    let material = OrdinaryBetaDigestMaterial {
        inventory_digest: inventory.digest(),
        manifest_digest: manifest.candidate_digest(),
        normalizer_protocol_digest: inventory.normalizer_protocol_digest(),
        successor_boundary_digest: inventory.successor_boundary().digest(),
        equation: equation.equation(),
        equation_source_identity: equation.source_identity(),
        owner_head: equation.owner_head(),
        owner_source_identity: owner.source_identity(),
        owner_was_predecessor_public,
        arguments: &arguments,
        reconstructed_source: &reconstructed_source,
        replayed_normalized: &replayed_normalized,
        substitutions: &substitutions,
        reduction_steps: &reduction_steps,
    };
    let derivation_digest = Digest::of_canonical(
        "pen-semantic-audit/verified-ordinary-beta-derivation/v1",
        &material,
    );
    Ok(VerifiedOrdinaryBetaDerivationV1 {
        inventory_digest: inventory.digest().clone(),
        manifest_digest: manifest.candidate_digest().clone(),
        normalizer_protocol_digest: inventory.normalizer_protocol_digest().clone(),
        successor_boundary_digest: inventory.successor_boundary().digest().clone(),
        equation: equation.equation().clone(),
        equation_source_identity: equation.source_identity().clone(),
        owner_head: equation.owner_head().clone(),
        owner_source_identity: owner.source_identity().clone(),
        owner_was_predecessor_public,
        arguments,
        reconstructed_source,
        replayed_normalized,
        substitutions,
        reduction_steps,
        derivation_digest,
    })
}

fn lambda_unit_ordinary_beta_syntax_violation(
    inventory: &VerifiedPublicAuditInventoryV1,
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
        .max()
}

fn application_spine(term: &Term) -> (Term, Vec<Term>) {
    let mut head = term;
    let mut reversed = Vec::new();
    while let Term::Apply { function, argument } = head {
        reversed.push(argument.as_ref().clone());
        head = function;
    }
    reversed.reverse();
    (head.clone(), reversed)
}

fn apply_spine<I>(head: Term, arguments: I) -> Term
where
    I: IntoIterator<Item = Term>,
{
    arguments
        .into_iter()
        .fold(head, |function, argument| Term::Apply {
            function: Box::new(function),
            argument: Box::new(argument),
        })
}

struct ReplayBudget {
    operations_left: u32,
    depth_limit: u16,
}

impl ReplayBudget {
    fn new(kernel: &Kernel) -> Self {
        let limits = kernel.limits();
        Self {
            operations_left: limits.max_operations,
            depth_limit: limits.max_depth,
        }
    }

    fn enter(&mut self, depth: u16) -> OrdinaryBetaResult<()> {
        if depth > self.depth_limit || self.operations_left == 0 {
            return Err(unknown(AuditUnknownReason::ResourceExhausted));
        }
        self.operations_left -= 1;
        Ok(())
    }
}

fn subst_top(
    body: &Term,
    replacement: &Term,
    budget: &mut ReplayBudget,
    depth: u16,
) -> OrdinaryBetaResult<Term> {
    let lifted = shift(replacement, 1, 0, budget, depth)?;
    let replaced = substitute(body, 0, &lifted, 0, budget, depth)?;
    shift(&replaced, -1, 0, budget, depth)
}

fn substitute(
    term: &Term,
    target: u32,
    replacement: &Term,
    binder_depth: u32,
    budget: &mut ReplayBudget,
    depth: u16,
) -> OrdinaryBetaResult<Term> {
    budget.enter(depth)?;
    let child = depth
        .checked_add(1)
        .ok_or_else(|| unknown(AuditUnknownReason::ResourceExhausted))?;
    let sought = target
        .checked_add(binder_depth)
        .ok_or_else(|| unknown(AuditUnknownReason::MalformedInput))?;
    match term {
        Term::Var { index } if *index == sought => {
            shift(replacement, i64::from(binder_depth), 0, budget, child)
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(term.clone()),
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(substitute(
                parameter,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            body: Box::new(substitute(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or_else(|| unknown(AuditUnknownReason::MalformedInput))?,
                budget,
                child,
            )?),
        }),
        Term::Sigma { parameter, body } => Ok(Term::Sigma {
            parameter: Box::new(substitute(
                parameter,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            body: Box::new(substitute(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or_else(|| unknown(AuditUnknownReason::MalformedInput))?,
                budget,
                child,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(substitute(
                parameter_type,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            body: Box::new(substitute(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or_else(|| unknown(AuditUnknownReason::MalformedInput))?,
                budget,
                child,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(substitute(
                function,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            argument: Box::new(substitute(
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
            sigma_type: Box::new(substitute(
                sigma_type,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            first: Box::new(substitute(
                first,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
            second: Box::new(substitute(
                second,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
        }),
        Term::First { pair } => Ok(Term::First {
            pair: Box::new(substitute(
                pair,
                target,
                replacement,
                binder_depth,
                budget,
                child,
            )?),
        }),
        Term::Second { pair } => Ok(Term::Second {
            pair: Box::new(substitute(
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

fn shift(
    term: &Term,
    amount: i64,
    cutoff: u32,
    budget: &mut ReplayBudget,
    depth: u16,
) -> OrdinaryBetaResult<Term> {
    budget.enter(depth)?;
    let child = depth
        .checked_add(1)
        .ok_or_else(|| unknown(AuditUnknownReason::ResourceExhausted))?;
    match term {
        Term::Var { index } if *index >= cutoff => {
            let shifted = i64::from(*index)
                .checked_add(amount)
                .and_then(|value| u32::try_from(value).ok())
                .ok_or_else(|| unknown(AuditUnknownReason::MalformedInput))?;
            Ok(Term::Var { index: shifted })
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(term.clone()),
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(shift(parameter, amount, cutoff, budget, child)?),
            body: Box::new(shift(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or_else(|| unknown(AuditUnknownReason::MalformedInput))?,
                budget,
                child,
            )?),
        }),
        Term::Sigma { parameter, body } => Ok(Term::Sigma {
            parameter: Box::new(shift(parameter, amount, cutoff, budget, child)?),
            body: Box::new(shift(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or_else(|| unknown(AuditUnknownReason::MalformedInput))?,
                budget,
                child,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(shift(parameter_type, amount, cutoff, budget, child)?),
            body: Box::new(shift(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or_else(|| unknown(AuditUnknownReason::MalformedInput))?,
                budget,
                child,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(shift(function, amount, cutoff, budget, child)?),
            argument: Box::new(shift(argument, amount, cutoff, budget, child)?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Ok(Term::Pair {
            sigma_type: Box::new(shift(sigma_type, amount, cutoff, budget, child)?),
            first: Box::new(shift(first, amount, cutoff, budget, child)?),
            second: Box::new(shift(second, amount, cutoff, budget, child)?),
        }),
        Term::First { pair } => Ok(Term::First {
            pair: Box::new(shift(pair, amount, cutoff, budget, child)?),
        }),
        Term::Second { pair } => Ok(Term::Second {
            pair: Box::new(shift(pair, amount, cutoff, budget, child)?),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inventory::{
        ORIGIN_CUTOFF_Q3_SCHEMA_VERSION, PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
        PublicDependencyUseV1, PublicSubjectV1, UncheckedOriginCutoffQ3RegistryV1,
        UncheckedPublicAuditInventoryV1, UncheckedPublicAvailabilityClaimV1,
        UncheckedPublicDeclarationV1, UncheckedPublicDependencyDagV1, UncheckedPublicEquationV1,
        UncheckedPublicEventCensusV1, UncheckedPublicGroupV1, UncheckedPublicHistoryStepV1,
        UncheckedSourceNormalizedDeclarationV1, verify_public_audit_inventory_v1,
    };
    use crate::manifest::{proposed_semantic_audit_manifest_v1, verify_semantic_audit_manifest_v1};
    use crate::model::{EventIdV1, PublicAvailabilityV1, SourceNormalizedJudgmentV1};
    use pen_kernel::{Declaration, KernelLimits, UncheckedSignature};

    #[derive(Clone, Copy)]
    enum FixtureShape {
        OneArgument,
        MultiArgument,
        ForgedConvertibleRight,
        SealedNonBeta,
    }

    struct Fixture {
        manifest: VerifiedSemanticAuditManifestV1,
        kernel: Kernel,
        inventory: VerifiedPublicAuditInventoryV1,
        equation: EquationIdV1,
    }

    fn digest(label: &str) -> Digest {
        Digest::of_domain_bytes("pen-semantic-audit/ordinary-beta-test/v1", label.as_bytes())
    }

    fn global(label: &str) -> GlobalId {
        GlobalId(digest(&format!("global/{label}")))
    }

    fn event(label: &str) -> EventIdV1 {
        EventIdV1(digest(&format!("event/{label}")))
    }

    fn equation_id(label: &str) -> EquationIdV1 {
        EquationIdV1(digest(&format!("equation/{label}")))
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

    fn source_equation(
        source: GenericJudgmentV1,
        normalized: GenericJudgmentV1,
    ) -> SourceNormalizedJudgmentV1 {
        SourceNormalizedJudgmentV1 {
            source_identity: Digest::of_canonical(
                "pen-semantic-audit/inventory-source-judgment/v1",
                &source,
            ),
            source,
            claimed_normalized: normalized,
        }
    }

    fn dependency(dependent: PublicSubjectV1, prerequisite: &GlobalId) -> PublicDependencyUseV1 {
        PublicDependencyUseV1 {
            dependent,
            prerequisite: prerequisite.clone(),
        }
    }

    fn apply(head: Term, arguments: impl IntoIterator<Item = Term>) -> Term {
        arguments
            .into_iter()
            .fold(head, |function, argument| Term::Apply {
                function: Box::new(function),
                argument: Box::new(argument),
            })
    }

    fn fixture(shape: FixtureShape) -> Fixture {
        let AuditDecision::Proven(manifest) =
            verify_semantic_audit_manifest_v1(&proposed_semantic_audit_manifest_v1())
        else {
            panic!("semantic manifest verifies");
        };
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");

        let a_type = global("A");
        let a = global("a");
        let b = global("b");
        let f = global("f");
        let group_type = global("group/A");
        let group_terms = global("group/terms");
        let group_function = global("group/function");
        let event_type = event("type");
        let event_terms = event("terms");
        let event_function = event("function");
        let equation = equation_id("successor-beta");
        let multi = matches!(
            shape,
            FixtureShape::MultiArgument | FixtureShape::ForgedConvertibleRight
        );

        let declaration_type = Declaration {
            id: a_type.clone(),
            ty: Term::Sort { level: 0 },
            body: None,
        };
        let declaration_a = Declaration {
            id: a.clone(),
            ty: Term::Global { id: a_type.clone() },
            body: None,
        };
        let declaration_b = Declaration {
            id: b.clone(),
            ty: Term::Global { id: a_type.clone() },
            body: None,
        };
        let declaration_f = if multi {
            Declaration {
                id: f.clone(),
                ty: Term::Pi {
                    parameter: Box::new(Term::Global { id: a_type.clone() }),
                    body: Box::new(Term::Pi {
                        parameter: Box::new(Term::Global { id: a_type.clone() }),
                        body: Box::new(Term::Global { id: a_type.clone() }),
                    }),
                },
                body: Some(Term::Lambda {
                    parameter_type: Box::new(Term::Global { id: a_type.clone() }),
                    body: Box::new(Term::Lambda {
                        parameter_type: Box::new(Term::Global { id: a_type.clone() }),
                        body: Box::new(Term::Var { index: 1 }),
                    }),
                }),
            }
        } else {
            Declaration {
                id: f.clone(),
                ty: Term::Pi {
                    parameter: Box::new(Term::Global { id: a_type.clone() }),
                    body: Box::new(Term::Global { id: a_type.clone() }),
                },
                body: Some(Term::Lambda {
                    parameter_type: Box::new(Term::Global { id: a_type.clone() }),
                    body: Box::new(Term::Var { index: 0 }),
                }),
            }
        };

        let arguments = if multi {
            vec![
                Term::Global { id: a.clone() },
                Term::Global { id: b.clone() },
            ]
        } else {
            vec![Term::Global { id: a.clone() }]
        };
        let beta_left = apply(Term::Global { id: f.clone() }, arguments);
        let exact_right = Term::Global { id: a.clone() };
        let source_right = match shape {
            FixtureShape::ForgedConvertibleRight => Term::Apply {
                function: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::Global { id: a_type.clone() }),
                    body: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::Global { id: a.clone() }),
            },
            FixtureShape::OneArgument | FixtureShape::MultiArgument => exact_right.clone(),
            FixtureShape::SealedNonBeta => exact_right.clone(),
        };
        let source_left = if matches!(shape, FixtureShape::SealedNonBeta) {
            exact_right.clone()
        } else {
            beta_left
        };
        let source_judgment = GenericJudgmentV1::Equation {
            context: Default::default(),
            left: source_left,
            right: source_right,
            ty: Term::Global { id: a_type.clone() },
        };
        let normalized_judgment = GenericJudgmentV1::Equation {
            context: Default::default(),
            left: exact_right.clone(),
            right: exact_right,
            ty: Term::Global { id: a_type.clone() },
        };

        let boundary_type = UncheckedSignature {
            declarations: vec![declaration_type.clone()],
        };
        let mut predecessor_declarations = vec![declaration_type.clone(), declaration_a.clone()];
        if multi {
            predecessor_declarations.push(declaration_b.clone());
        }
        let predecessor_boundary = UncheckedSignature {
            declarations: predecessor_declarations.clone(),
        };
        let mut successor_declarations = predecessor_declarations;
        successor_declarations.push(declaration_f.clone());
        let successor_boundary = UncheckedSignature {
            declarations: successor_declarations,
        };

        let mut dependencies = vec![
            dependency(
                PublicSubjectV1::Declaration {
                    declaration: a.clone(),
                },
                &a_type,
            ),
            dependency(
                PublicSubjectV1::Declaration {
                    declaration: f.clone(),
                },
                &a_type,
            ),
            dependency(
                PublicSubjectV1::Equation {
                    equation: equation.clone(),
                },
                &a_type,
            ),
            dependency(
                PublicSubjectV1::Equation {
                    equation: equation.clone(),
                },
                &a,
            ),
            dependency(
                PublicSubjectV1::Equation {
                    equation: equation.clone(),
                },
                &f,
            ),
        ];
        if multi {
            dependencies.push(dependency(
                PublicSubjectV1::Declaration {
                    declaration: b.clone(),
                },
                &a_type,
            ));
            dependencies.push(dependency(
                PublicSubjectV1::Equation {
                    equation: equation.clone(),
                },
                &b,
            ));
        }
        let public_availability = dependencies
            .iter()
            .map(|dependency| UncheckedPublicAvailabilityClaimV1 {
                dependency: dependency.clone(),
                claimed: if dependency.prerequisite == f {
                    PublicAvailabilityV1::DependencyPriorExport { target: f.clone() }
                } else {
                    PublicAvailabilityV1::PredecessorPublicExport {
                        target: dependency.prerequisite.clone(),
                    }
                },
            })
            .collect();

        let mut term_declarations = vec![a.clone()];
        if multi {
            term_declarations.push(b.clone());
        }
        let mut declaration_groups = vec![
            UncheckedPublicGroupV1 {
                group: group_type.clone(),
                origin: event_type.clone(),
                declarations: vec![a_type.clone()],
            },
            UncheckedPublicGroupV1 {
                group: group_terms.clone(),
                origin: event_terms.clone(),
                declarations: term_declarations,
            },
            UncheckedPublicGroupV1 {
                group: group_function.clone(),
                origin: event_function.clone(),
                declarations: vec![f.clone()],
            },
        ];
        declaration_groups.sort_by(|left, right| left.group.cmp(&right.group));

        let mut declarations = vec![
            UncheckedPublicDeclarationV1 {
                declaration: a_type.clone(),
                origin: event_type.clone(),
                group: group_type.clone(),
                source_to_normal: source_declaration(&declaration_type),
            },
            UncheckedPublicDeclarationV1 {
                declaration: a.clone(),
                origin: event_terms.clone(),
                group: group_terms.clone(),
                source_to_normal: source_declaration(&declaration_a),
            },
            UncheckedPublicDeclarationV1 {
                declaration: f.clone(),
                origin: event_function.clone(),
                group: group_function.clone(),
                source_to_normal: source_declaration(&declaration_f),
            },
        ];
        if multi {
            declarations.push(UncheckedPublicDeclarationV1 {
                declaration: b.clone(),
                origin: event_terms.clone(),
                group: group_terms.clone(),
                source_to_normal: source_declaration(&declaration_b),
            });
        }

        let wire = UncheckedPublicAuditInventoryV1 {
            schema_version: PUBLIC_AUDIT_INVENTORY_SCHEMA_VERSION,
            predecessor_history: vec![
                UncheckedPublicHistoryStepV1 {
                    census: UncheckedPublicEventCensusV1 {
                        event: event_type.clone(),
                        added_groups: vec![group_type],
                        added_declarations: vec![a_type.clone()],
                        added_equations: Vec::new(),
                        added_forced_projections: Vec::new(),
                        added_demand_contracts: Vec::new(),
                    },
                    successor_boundary: boundary_type,
                },
                UncheckedPublicHistoryStepV1 {
                    census: UncheckedPublicEventCensusV1 {
                        event: event_terms.clone(),
                        added_groups: vec![group_terms],
                        added_declarations: if multi {
                            vec![a.clone(), b.clone()]
                        } else {
                            vec![a.clone()]
                        },
                        added_equations: Vec::new(),
                        added_forced_projections: Vec::new(),
                        added_demand_contracts: Vec::new(),
                    },
                    successor_boundary: predecessor_boundary.clone(),
                },
            ],
            predecessor_boundary,
            successor_event: UncheckedPublicEventCensusV1 {
                event: event_function.clone(),
                added_groups: vec![group_function],
                added_declarations: vec![f.clone()],
                added_equations: vec![equation.clone()],
                added_forced_projections: Vec::new(),
                added_demand_contracts: Vec::new(),
            },
            successor_boundary,
            declaration_groups,
            declarations,
            equations: vec![UncheckedPublicEquationV1 {
                equation: equation.clone(),
                owner_head: f,
                origin: event_function,
                source_to_normal: source_equation(source_judgment, normalized_judgment),
                demand_port: None,
            }],
            forced_projections: Vec::new(),
            predecessor_demand_contracts: Vec::new(),
            public_availability,
            dependency_dag: UncheckedPublicDependencyDagV1 {
                edges: dependencies,
            },
            q3_registry: UncheckedOriginCutoffQ3RegistryV1 {
                schema_version: ORIGIN_CUTOFF_Q3_SCHEMA_VERSION,
                origin_cutoff: Some(event_terms),
                entries: Vec::new(),
            },
        };
        let AuditDecision::Proven(inventory) =
            verify_public_audit_inventory_v1(&manifest, &kernel, &wire)
        else {
            panic!("ordinary-beta fixture inventory verifies");
        };
        Fixture {
            manifest,
            kernel,
            inventory,
            equation,
        }
    }

    #[test]
    fn one_argument_beta_replays_delta_and_checked_substitution() {
        let fixture = fixture(FixtureShape::OneArgument);
        let AuditDecision::Proven(verified) = verify_ordinary_beta_derivation_v1(
            &fixture.manifest,
            &fixture.kernel,
            &fixture.inventory,
            &fixture.equation,
        ) else {
            panic!("one-argument beta verifies");
        };
        assert_eq!(verified.arguments().len(), 1);
        assert_eq!(verified.substitutions().len(), 1);
        assert!(!verified.owner_was_predecessor_public());
        assert!(matches!(
            verified.reduction_steps(),
            [
                VerifiedOrdinaryBetaReductionStepV1::DeltaUnfoldHead { .. },
                VerifiedOrdinaryBetaReductionStepV1::BetaContract { .. }
            ]
        ));

        let AuditDecision::Proven(replayed) = verify_ordinary_beta_derivation_v1(
            &fixture.manifest,
            &fixture.kernel,
            &fixture.inventory,
            &fixture.equation,
        ) else {
            panic!("deterministic replay verifies");
        };
        assert_eq!(verified, replayed);
        assert_eq!(verified.derivation_digest(), replayed.derivation_digest());
    }

    #[test]
    fn multi_argument_beta_is_sequential_and_congruence_lifted() {
        let fixture = fixture(FixtureShape::MultiArgument);
        let AuditDecision::Proven(verified) = verify_ordinary_beta_derivation_v1(
            &fixture.manifest,
            &fixture.kernel,
            &fixture.inventory,
            &fixture.equation,
        ) else {
            panic!("multi-argument beta verifies");
        };
        assert_eq!(verified.arguments().len(), 2);
        assert_eq!(
            verified
                .substitutions()
                .iter()
                .map(VerifiedSequentialSubstitutionV1::argument_ordinal)
                .collect::<Vec<_>>(),
            vec![0, 1]
        );
        assert_eq!(
            verified
                .reduction_steps()
                .iter()
                .filter(|step| matches!(
                    step,
                    VerifiedOrdinaryBetaReductionStepV1::BetaContract { .. }
                ))
                .count(),
            2
        );
        assert!(verified.reduction_steps().iter().any(|step| matches!(
            step,
            VerifiedOrdinaryBetaReductionStepV1::ApplicationCongruence {
                argument_ordinal: 0,
                pending_argument_count: 1,
                ..
            }
        )));
    }

    #[test]
    fn convertible_but_nonexact_contractum_and_nonbeta_equation_fail_closed() {
        for shape in [
            FixtureShape::ForgedConvertibleRight,
            FixtureShape::SealedNonBeta,
        ] {
            let fixture = fixture(shape);
            assert!(matches!(
                verify_ordinary_beta_derivation_v1(
                    &fixture.manifest,
                    &fixture.kernel,
                    &fixture.inventory,
                    &fixture.equation,
                ),
                AuditDecision::Unknown(AuditUnknownReason::MissingOrdinaryBetaDerivation)
            ));
        }
    }

    #[test]
    fn unlisted_equation_cannot_select_a_verified_derivation() {
        let fixture = fixture(FixtureShape::OneArgument);
        assert!(matches!(
            verify_ordinary_beta_derivation_v1(
                &fixture.manifest,
                &fixture.kernel,
                &fixture.inventory,
                &equation_id("not-in-inventory"),
            ),
            AuditDecision::Unknown(AuditUnknownReason::IncompleteEnumeration)
        ));
    }
}
