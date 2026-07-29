//! A deliberately tiny equation-extension verifier.
//!
//! The only admitted rule is reconstructed from a verified one-constructor
//! family pair.  No unchecked equation terms cross this module's public
//! boundary.

use pen_demand::gsc::{
    ComputationMode, ConstructorPortId, ContextMapCode, EquationClauseCode, GscOutcome, GscRule,
    GscUnknownReason, OutputClause, OutputRole, PortKey, PortRef, VerifiedCanonicalDemandFamilyV2,
    VerifiedClosedFormerFrame, VerifiedGscSemanticManifest,
};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Declaration, DependentContext, Digest, GlobalId, Kernel,
    KernelError, MAX_SAFE_RECURSION_DEPTH, OpenJudgment, Term, UncheckedSignature,
    VerifiedSignature,
};
use std::collections::BTreeSet;

/// Opaque evidence that the generated port was instantiated by the exact
/// candidate term, with the newest equation-context variable removed.
#[derive(Clone, Debug)]
pub struct VerifiedGeneratedPortSubstitution {
    source_port: PortKey,
    source_context: DependentContext,
    target_context: DependentContext,
    filler: Term,
    digest: Digest,
}

impl VerifiedGeneratedPortSubstitution {
    pub fn source_port(&self) -> &PortKey {
        &self.source_port
    }

    pub fn source_context(&self) -> &DependentContext {
        &self.source_context
    }

    pub fn target_context(&self) -> &DependentContext {
        &self.target_context
    }

    pub fn filler(&self) -> &Term {
        &self.filler
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Structural facts established for the sole admitted rewrite rule.
#[derive(Clone, Debug)]
pub struct VerifiedRewriteInvariants {
    left_linear: bool,
    nonrecursive: bool,
    no_critical_overlaps: bool,
    redex_measure_decreases: bool,
    old_terms_conservative_by_freshness: bool,
    exact_generated_substitution_preserves_equation: bool,
    digest: Digest,
}

impl VerifiedRewriteInvariants {
    pub fn is_left_linear(&self) -> bool {
        self.left_linear
    }

    pub fn is_nonrecursive(&self) -> bool {
        self.nonrecursive
    }

    pub fn has_no_critical_overlaps(&self) -> bool {
        self.no_critical_overlaps
    }

    pub fn is_terminating_in_admitted_fragment(&self) -> bool {
        self.redex_measure_decreases
    }

    pub fn is_confluent_in_admitted_fragment(&self) -> bool {
        self.left_linear && self.no_critical_overlaps
    }

    pub fn is_conservative_on_old_terms(&self) -> bool {
        self.old_terms_conservative_by_freshness
    }

    /// Whether replaying the compiler's sole generated-port substitution
    /// produced the checked target-context equation represented by this
    /// clause. This is deliberately narrower than a theorem about arbitrary
    /// user-supplied substitutions.
    pub fn exact_generated_substitution_preserves_equation(&self) -> bool {
        self.exact_generated_substitution_preserves_equation
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Opaque evidence for one reconstructed fresh-head computation rule.
#[derive(Clone, Debug)]
pub struct VerifiedFreshEliminatorBeta {
    fresh_head: GlobalId,
    head_type: Term,
    context: DependentContext,
    left: Term,
    right: Term,
    ty: Term,
    extended_signature: VerifiedSignature,
    substitution: VerifiedGeneratedPortSubstitution,
    invariants: VerifiedRewriteInvariants,
    digest: Digest,
}

impl VerifiedFreshEliminatorBeta {
    pub fn fresh_head(&self) -> &GlobalId {
        &self.fresh_head
    }

    pub fn head_type(&self) -> &Term {
        &self.head_type
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

    pub fn extended_signature(&self) -> &VerifiedSignature {
        &self.extended_signature
    }

    pub fn substitution(&self) -> &VerifiedGeneratedPortSubstitution {
        &self.substitution
    }

    pub fn invariants(&self) -> &VerifiedRewriteInvariants {
        &self.invariants
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

/// Joint capability for the complete admitted equation set.
#[derive(Clone, Debug)]
pub struct VerifiedEquationExtensionSet {
    clause: VerifiedFreshEliminatorBeta,
    digest: Digest,
}

impl VerifiedEquationExtensionSet {
    pub fn clause(&self) -> &VerifiedFreshEliminatorBeta {
        &self.clause
    }

    pub fn clauses_len(&self) -> usize {
        1
    }

    pub fn extended_signature(&self) -> &VerifiedSignature {
        self.clause.extended_signature()
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReconstructedCandidate {
    declaration: Declaration,
    constructor: ConstructorPortId,
    source_port: PortKey,
    source_context: DependentContext,
    target_context: DependentContext,
    filler: Term,
    context: DependentContext,
    left: Term,
    right: Term,
    ty: Term,
}

struct EquationVerificationInputs<'a> {
    kernel: &'a Kernel,
    boundary: &'a VerifiedSignature,
    semantic: &'a VerifiedGscSemanticManifest,
    frame: &'a VerifiedClosedFormerFrame,
    use_family: &'a VerifiedCanonicalDemandFamilyV2,
    compute_family: &'a VerifiedCanonicalDemandFamilyV2,
}

pub(crate) fn verify_one_nullary_equation_extension(
    kernel: &Kernel,
    boundary: &VerifiedSignature,
    semantic: &VerifiedGscSemanticManifest,
    frame: &VerifiedClosedFormerFrame,
    use_family: &VerifiedCanonicalDemandFamilyV2,
    compute_family: &VerifiedCanonicalDemandFamilyV2,
) -> GscOutcome<VerifiedEquationExtensionSet> {
    let candidate = match reconstruct_candidate(
        kernel,
        boundary,
        semantic,
        frame,
        use_family,
        compute_family,
    ) {
        Ok(candidate) => candidate,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    verify_reconstructed_candidate(
        EquationVerificationInputs {
            kernel,
            boundary,
            semantic,
            frame,
            use_family,
            compute_family,
        },
        candidate,
        1,
    )
}

fn reconstruct_candidate(
    kernel: &Kernel,
    boundary: &VerifiedSignature,
    semantic: &VerifiedGscSemanticManifest,
    frame: &VerifiedClosedFormerFrame,
    use_family: &VerifiedCanonicalDemandFamilyV2,
    compute_family: &VerifiedCanonicalDemandFamilyV2,
) -> Result<ReconstructedCandidate, GscUnknownReason> {
    if !frame.code().is_one_nullary()
        || frame.code().manifest_digest() != semantic.digest()
        || use_family.semantic_manifest_digest() != semantic.digest()
        || compute_family.semantic_manifest_digest() != semantic.digest()
        || use_family.rule() != GscRule::Use
        || use_family.rank() != 1
        || compute_family.rule() != GscRule::Compute
        || compute_family.rank() != 2
        || !use_family.premise_refs().is_empty()
        || use_family.ports().len() != 1
        || compute_family.ports().len() != 1
        || compute_family.premise_refs().len() != 1
    {
        return Err(GscUnknownReason::FamilyMismatch);
    }

    let constructor = frame
        .code()
        .constructor_id(0)
        .ok_or(GscUnknownReason::FamilyMismatch)?;
    let introduction = frame
        .introduction(constructor)
        .ok_or(GscUnknownReason::FamilyMismatch)?;
    let use_port = &use_family.ports()[0];
    let expected_use_role = OutputRole::UsePort {
        former: frame.id().clone(),
    };
    let OutputClause::TermPort {
        context: use_context,
        motive: use_motive,
        role: use_role,
    } = use_port.clause()
    else {
        return Err(GscUnknownReason::PortMismatch);
    };
    if use_role != &expected_use_role
        || use_context != use_family.parameter_context()
        || use_context.0.len() != 2
    {
        return Err(GscUnknownReason::FamilyMismatch);
    }

    let owner = Term::Global {
        id: frame.owner().clone(),
    };
    let raw_expected_context = DependentContext(vec![
        Term::Pi {
            parameter: Box::new(owner.clone()),
            body: Box::new(Term::Sort {
                level: frame.code().code().universe_level,
            }),
        },
        Term::Apply {
            function: Box::new(Term::Var { index: 0 }),
            argument: Box::new(Term::Global {
                id: introduction.clone(),
            }),
        },
    ]);
    let raw_expected_motive = Term::Pi {
        parameter: Box::new(owner),
        body: Box::new(Term::Apply {
            function: Box::new(Term::Var { index: 2 }),
            argument: Box::new(Term::Var { index: 0 }),
        }),
    };
    let expected_context = match kernel.verify_context(boundary, &raw_expected_context) {
        Ok(context) => context.normalized_wire(),
        Err(error) => return Err(map_kernel_error(error)),
    };
    let expected_use_formation = OpenJudgment::TypeFormation {
        context: expected_context,
        term: raw_expected_motive,
    };
    let (expected_context, expected_motive) =
        match kernel.verify_open_judgment(boundary, &expected_use_formation) {
            Ok(OpenJudgment::TypeFormation { context, term }) => (context, term),
            Err(error) => return Err(map_kernel_error(error)),
            _ => return Err(GscUnknownReason::KernelCouldNotCertify),
        };
    if use_context != &expected_context || use_motive != &expected_motive {
        return Err(GscUnknownReason::FamilyMismatch);
    }

    let premise = &compute_family.premise_refs()[0];
    if premise.context_map != ContextMapCode::IdentityV1
        || premise.expected_role != expected_use_role
        || premise.source
            != (PortRef::Generated {
                port: use_port.key().clone(),
            })
        || compute_family.parameter_context() != use_context
    {
        return Err(GscUnknownReason::PortMismatch);
    }

    let compute_port = &compute_family.ports()[0];
    let OutputClause::EquationPort {
        context: equation_context,
        equation_code,
        role,
    } = compute_port.clause()
    else {
        return Err(GscUnknownReason::PortMismatch);
    };
    let expected_compute_role = OutputRole::ComputationPort {
        former: frame.id().clone(),
        constructor: constructor.clone(),
        computation_mode: ComputationMode::JudgmentalFreshHead,
    };
    if role != &expected_compute_role
        || equation_code
            != &(EquationClauseCode::GeneratedUseBeta {
                former: frame.id().clone(),
                constructor: constructor.clone(),
                use_port: use_port.key().clone(),
            })
        || equation_context.0.len() != use_context.0.len() + 1
        || equation_context.0[..use_context.0.len()] != use_context.0
        || equation_context.0.last() != Some(use_motive)
        || compute_family.verification_judgments().len() != 2
    {
        return Err(GscUnknownReason::PortMismatch);
    }

    let expected_source_left = Term::Apply {
        function: Box::new(Term::Var { index: 0 }),
        argument: Box::new(Term::Global {
            id: introduction.clone(),
        }),
    };
    let expected_source_right = Term::Var { index: 1 };
    let expected_source_ty = Term::Apply {
        function: Box::new(Term::Var { index: 2 }),
        argument: Box::new(Term::Global {
            id: introduction.clone(),
        }),
    };
    let raw_expected_judgments = [
        OpenJudgment::HasType {
            context: equation_context.clone(),
            term: expected_source_left,
            ty: expected_source_ty.clone(),
        },
        OpenJudgment::HasType {
            context: equation_context.clone(),
            term: expected_source_right,
            ty: expected_source_ty,
        },
    ];
    let expected_judgments = match kernel.verify_open_judgments(
        boundary,
        &[&raw_expected_judgments[0], &raw_expected_judgments[1]],
    ) {
        Ok(judgments) => judgments,
        Err(error) => return Err(map_kernel_error(error)),
    };
    if compute_family.verification_judgments() != expected_judgments {
        return Err(GscUnknownReason::FamilyMismatch);
    }

    let head_type = close_over_context(use_context, use_motive.clone());
    let fresh_head = deterministic_fresh_head(
        semantic,
        boundary,
        frame,
        use_port.key(),
        compute_port.key(),
        &head_type,
    );
    if boundary
        .declarations()
        .iter()
        .any(|declaration| declaration.id == fresh_head)
    {
        return Err(GscUnknownReason::FamilyMismatch);
    }
    let filler = apply_context_variables(
        Term::Global {
            id: fresh_head.clone(),
        },
        use_context.0.len(),
    )
    .ok_or(GscUnknownReason::ResourceExhausted)?;

    let source_left = judgment_term(&compute_family.verification_judgments()[0])
        .ok_or(GscUnknownReason::FamilyMismatch)?;
    let source_right = judgment_term(&compute_family.verification_judgments()[1])
        .ok_or(GscUnknownReason::FamilyMismatch)?;
    let source_ty = judgment_type(&compute_family.verification_judgments()[0])
        .ok_or(GscUnknownReason::FamilyMismatch)?;
    let left =
        substitute_newest(source_left, &filler, 0).ok_or(GscUnknownReason::ResourceExhausted)?;
    let right =
        substitute_newest(source_right, &filler, 0).ok_or(GscUnknownReason::ResourceExhausted)?;
    let ty = substitute_newest(source_ty, &filler, 0).ok_or(GscUnknownReason::ResourceExhausted)?;

    let expected_argument = match source_left {
        Term::Apply { argument, .. } => argument.as_ref(),
        _ => return Err(GscUnknownReason::FamilyMismatch),
    };
    let expected_left = Term::Apply {
        function: Box::new(filler.clone()),
        argument: Box::new(expected_argument.clone()),
    };
    let expected_right = Term::Var { index: 0 };
    let expected_ty = Term::Apply {
        function: Box::new(Term::Var { index: 1 }),
        argument: Box::new(expected_argument.clone()),
    };
    if left != expected_left || right != expected_right || ty != expected_ty {
        return Err(GscUnknownReason::FamilyMismatch);
    }

    Ok(ReconstructedCandidate {
        declaration: Declaration {
            id: fresh_head,
            ty: head_type,
            body: None,
        },
        constructor: constructor.clone(),
        source_port: use_port.key().clone(),
        source_context: equation_context.clone(),
        target_context: use_context.clone(),
        filler,
        context: use_context.clone(),
        left,
        right,
        ty,
    })
}

fn verify_reconstructed_candidate(
    inputs: EquationVerificationInputs<'_>,
    candidate: ReconstructedCandidate,
    set_size: usize,
) -> GscOutcome<VerifiedEquationExtensionSet> {
    if set_size != 1 {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }
    let expected = match reconstruct_candidate(
        inputs.kernel,
        inputs.boundary,
        inputs.semantic,
        inputs.frame,
        inputs.use_family,
        inputs.compute_family,
    ) {
        Ok(expected) => expected,
        Err(reason) => return GscOutcome::Unknown(reason),
    };
    if candidate != expected
        || candidate.declaration.body.is_some()
        || contains_global(&candidate.declaration.ty, &candidate.declaration.id)
        || inputs.boundary.declarations().iter().any(|declaration| {
            declaration.id == candidate.declaration.id
                || contains_global(&declaration.ty, &candidate.declaration.id)
                || declaration
                    .body
                    .as_ref()
                    .is_some_and(|body| contains_global(body, &candidate.declaration.id))
        })
        || candidate.context != candidate.target_context
    {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }

    let extension = UncheckedSignature {
        declarations: vec![candidate.declaration.clone()],
    };
    let extended_signature = match inputs.kernel.verify_extension(inputs.boundary, &extension) {
        Ok(signature) => signature,
        Err(error) => return GscOutcome::Unknown(map_kernel_error(error)),
    };
    let left_judgment = OpenJudgment::HasType {
        context: candidate.context.clone(),
        term: candidate.left.clone(),
        ty: candidate.ty.clone(),
    };
    let right_judgment = OpenJudgment::HasType {
        context: candidate.context.clone(),
        term: candidate.right.clone(),
        ty: candidate.ty.clone(),
    };
    if let Err(error) = inputs
        .kernel
        .verify_open_judgments(&extended_signature, &[&left_judgment, &right_judgment])
    {
        return GscOutcome::Unknown(map_kernel_error(error));
    }

    let head = &candidate.declaration.id;
    let left_vars = variables(&candidate.left);
    let right_vars = variables(&candidate.right);
    let left_linear = left_vars.iter().all(|variable| {
        variable_occurrences(&candidate.left, *variable)
            == if *variable == 0 || *variable == 1 {
                1
            } else {
                0
            }
    }) && right_vars.is_subset(&left_vars);
    let nonrecursive = !contains_global(&candidate.right, head)
        && !contains_global(&candidate.declaration.ty, head);
    let fresh_count = global_occurrences(&candidate.left, head);
    let no_nested_fresh_head = fresh_count == 1
        && root_global(&candidate.left) == Some(head)
        && immediate_arguments(&candidate.left)
            .iter()
            .all(|argument| !contains_global(argument, head));
    let old_terms_conservative = inputs.boundary.declarations().iter().all(|declaration| {
        !contains_global(&declaration.ty, head)
            && declaration
                .body
                .as_ref()
                .is_none_or(|body| !contains_global(body, head))
    });
    if !left_linear
        || !nonrecursive
        || !no_nested_fresh_head
        || !old_terms_conservative
        || contains_global(&candidate.right, head)
    {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }

    let substitution_digest = {
        let mut encoder = CanonicalEncoder::new();
        candidate.source_port.encode_canonical(&mut encoder);
        candidate.source_context.encode_canonical(&mut encoder);
        candidate.target_context.encode_canonical(&mut encoder);
        candidate.filler.encode_canonical(&mut encoder);
        Digest::of_domain_bytes("pen-gf2/generated-port-substitution/v1", encoder.as_bytes())
    };
    let substitution = VerifiedGeneratedPortSubstitution {
        source_port: candidate.source_port,
        source_context: candidate.source_context,
        target_context: candidate.target_context,
        filler: candidate.filler,
        digest: substitution_digest,
    };
    let invariants_digest = {
        let mut encoder = CanonicalEncoder::new();
        candidate.declaration.id.encode_canonical(&mut encoder);
        candidate.left.encode_canonical(&mut encoder);
        candidate.right.encode_canonical(&mut encoder);
        encoder.tag(1);
        encoder.tag(1);
        encoder.tag(1);
        encoder.tag(1);
        encoder.tag(1);
        encoder.tag(1);
        Digest::of_domain_bytes("pen-gf2/rewrite-invariants/v1", encoder.as_bytes())
    };
    let invariants = VerifiedRewriteInvariants {
        left_linear,
        nonrecursive,
        no_critical_overlaps: true,
        redex_measure_decreases: true,
        old_terms_conservative_by_freshness: old_terms_conservative,
        exact_generated_substitution_preserves_equation: true,
        digest: invariants_digest,
    };
    let clause_digest = {
        let mut encoder = CanonicalEncoder::new();
        inputs.semantic.digest().encode_canonical(&mut encoder);
        inputs.boundary.digest().encode_canonical(&mut encoder);
        inputs.frame.id().encode_canonical(&mut encoder);
        candidate.constructor.encode_canonical(&mut encoder);
        inputs.use_family.id().encode_canonical(&mut encoder);
        inputs.compute_family.id().encode_canonical(&mut encoder);
        candidate.declaration.encode_canonical(&mut encoder);
        candidate.context.encode_canonical(&mut encoder);
        candidate.left.encode_canonical(&mut encoder);
        candidate.right.encode_canonical(&mut encoder);
        candidate.ty.encode_canonical(&mut encoder);
        substitution.digest().encode_canonical(&mut encoder);
        invariants.digest().encode_canonical(&mut encoder);
        Digest::of_domain_bytes("pen-gf2/fresh-eliminator-beta/v1", encoder.as_bytes())
    };
    let clause = VerifiedFreshEliminatorBeta {
        fresh_head: candidate.declaration.id,
        head_type: candidate.declaration.ty,
        context: candidate.context,
        left: candidate.left,
        right: candidate.right,
        ty: candidate.ty,
        extended_signature,
        substitution,
        invariants,
        digest: clause_digest,
    };
    let set_digest = {
        let mut encoder = CanonicalEncoder::new();
        inputs.semantic.digest().encode_canonical(&mut encoder);
        clause.digest().encode_canonical(&mut encoder);
        encoder.u64(1);
        Digest::of_domain_bytes("pen-gf2/equation-extension-set/v1", encoder.as_bytes())
    };
    GscOutcome::Proven(VerifiedEquationExtensionSet {
        clause,
        digest: set_digest,
    })
}

fn deterministic_fresh_head(
    semantic: &VerifiedGscSemanticManifest,
    boundary: &VerifiedSignature,
    frame: &VerifiedClosedFormerFrame,
    use_port: &PortKey,
    compute_port: &PortKey,
    head_type: &Term,
) -> GlobalId {
    let mut encoder = CanonicalEncoder::new();
    semantic.digest().encode_canonical(&mut encoder);
    boundary.digest().encode_canonical(&mut encoder);
    frame.id().encode_canonical(&mut encoder);
    use_port.encode_canonical(&mut encoder);
    compute_port.encode_canonical(&mut encoder);
    head_type.encode_canonical(&mut encoder);
    GlobalId(Digest::of_domain_bytes(
        "pen-gf2/generated-eliminator-head/v1",
        encoder.as_bytes(),
    ))
}

fn close_over_context(context: &DependentContext, mut body: Term) -> Term {
    for parameter in context.0.iter().rev() {
        body = Term::Pi {
            parameter: Box::new(parameter.clone()),
            body: Box::new(body),
        };
    }
    body
}

fn apply_context_variables(mut function: Term, length: usize) -> Option<Term> {
    for position in 0..length {
        let index = u32::try_from(length.checked_sub(position + 1)?).ok()?;
        function = Term::Apply {
            function: Box::new(function),
            argument: Box::new(Term::Var { index }),
        };
    }
    Some(function)
}

fn judgment_term(judgment: &OpenJudgment) -> Option<&Term> {
    match judgment {
        OpenJudgment::HasType { term, .. } => Some(term),
        _ => None,
    }
}

fn judgment_type(judgment: &OpenJudgment) -> Option<&Term> {
    match judgment {
        OpenJudgment::HasType { ty, .. } => Some(ty),
        _ => None,
    }
}

pub(crate) fn map_kernel_error(error: KernelError) -> GscUnknownReason {
    match error {
        KernelError::ResourceExhausted(_) => GscUnknownReason::ResourceExhausted,
        _ => GscUnknownReason::KernelCouldNotCertify,
    }
}

fn substitute_newest(term: &Term, replacement: &Term, depth: u16) -> Option<Term> {
    if depth > MAX_SAFE_RECURSION_DEPTH {
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
            parameter: Box::new(substitute_newest(parameter, replacement, depth)?),
            body: Box::new(substitute_newest(body, replacement, child)?),
        }),
        Term::Sigma { parameter, body } => Some(Term::Sigma {
            parameter: Box::new(substitute_newest(parameter, replacement, depth)?),
            body: Box::new(substitute_newest(body, replacement, child)?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Some(Term::Lambda {
            parameter_type: Box::new(substitute_newest(parameter_type, replacement, depth)?),
            body: Box::new(substitute_newest(body, replacement, child)?),
        }),
        Term::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(substitute_newest(function, replacement, depth)?),
            argument: Box::new(substitute_newest(argument, replacement, depth)?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Some(Term::Pair {
            sigma_type: Box::new(substitute_newest(sigma_type, replacement, depth)?),
            first: Box::new(substitute_newest(first, replacement, depth)?),
            second: Box::new(substitute_newest(second, replacement, depth)?),
        }),
        Term::First { pair } => Some(Term::First {
            pair: Box::new(substitute_newest(pair, replacement, depth)?),
        }),
        Term::Second { pair } => Some(Term::Second {
            pair: Box::new(substitute_newest(pair, replacement, depth)?),
        }),
    }
}

pub(crate) fn shift(term: &Term, amount: i64, cutoff: u32, depth: u16) -> Option<Term> {
    if depth > MAX_SAFE_RECURSION_DEPTH {
        return None;
    }
    let child = depth.checked_add(1)?;
    match term {
        Term::Var { index } if *index >= cutoff => {
            let shifted = i64::from(*index).checked_add(amount)?;
            Some(Term::Var {
                index: u32::try_from(shifted).ok()?,
            })
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Some(term.clone()),
        Term::Pi { parameter, body } => Some(Term::Pi {
            parameter: Box::new(shift(parameter, amount, cutoff, child)?),
            body: Box::new(shift(body, amount, cutoff.checked_add(1)?, child)?),
        }),
        Term::Sigma { parameter, body } => Some(Term::Sigma {
            parameter: Box::new(shift(parameter, amount, cutoff, child)?),
            body: Box::new(shift(body, amount, cutoff.checked_add(1)?, child)?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Some(Term::Lambda {
            parameter_type: Box::new(shift(parameter_type, amount, cutoff, child)?),
            body: Box::new(shift(body, amount, cutoff.checked_add(1)?, child)?),
        }),
        Term::Apply { function, argument } => Some(Term::Apply {
            function: Box::new(shift(function, amount, cutoff, child)?),
            argument: Box::new(shift(argument, amount, cutoff, child)?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Some(Term::Pair {
            sigma_type: Box::new(shift(sigma_type, amount, cutoff, child)?),
            first: Box::new(shift(first, amount, cutoff, child)?),
            second: Box::new(shift(second, amount, cutoff, child)?),
        }),
        Term::First { pair } => Some(Term::First {
            pair: Box::new(shift(pair, amount, cutoff, child)?),
        }),
        Term::Second { pair } => Some(Term::Second {
            pair: Box::new(shift(pair, amount, cutoff, child)?),
        }),
    }
}

fn root_global(mut term: &Term) -> Option<&GlobalId> {
    while let Term::Apply { function, .. } = term {
        term = function;
    }
    match term {
        Term::Global { id } => Some(id),
        _ => None,
    }
}

fn immediate_arguments(term: &Term) -> Vec<&Term> {
    let mut arguments = Vec::new();
    let mut cursor = term;
    while let Term::Apply { function, argument } = cursor {
        arguments.push(argument.as_ref());
        cursor = function;
    }
    arguments.reverse();
    arguments
}

fn contains_global(term: &Term, target: &GlobalId) -> bool {
    global_occurrences(term, target) != 0
}

fn global_occurrences(term: &Term, target: &GlobalId) -> usize {
    match term {
        Term::Global { id } => usize::from(id == target),
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            global_occurrences(parameter, target) + global_occurrences(body, target)
        }
        Term::Lambda {
            parameter_type,
            body,
        } => global_occurrences(parameter_type, target) + global_occurrences(body, target),
        Term::Apply { function, argument } => {
            global_occurrences(function, target) + global_occurrences(argument, target)
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            global_occurrences(sigma_type, target)
                + global_occurrences(first, target)
                + global_occurrences(second, target)
        }
        Term::First { pair } | Term::Second { pair } => global_occurrences(pair, target),
        Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => 0,
    }
}

fn variables(term: &Term) -> BTreeSet<u32> {
    let mut result = BTreeSet::new();
    collect_variables(term, 0, &mut result);
    result
}

fn collect_variables(term: &Term, binder_depth: u32, output: &mut BTreeSet<u32>) {
    match term {
        Term::Var { index } if *index >= binder_depth => {
            output.insert(index - binder_depth);
        }
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            collect_variables(parameter, binder_depth, output);
            collect_variables(body, binder_depth.saturating_add(1), output);
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            collect_variables(parameter_type, binder_depth, output);
            collect_variables(body, binder_depth.saturating_add(1), output);
        }
        Term::Apply { function, argument } => {
            collect_variables(function, binder_depth, output);
            collect_variables(argument, binder_depth, output);
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            collect_variables(sigma_type, binder_depth, output);
            collect_variables(first, binder_depth, output);
            collect_variables(second, binder_depth, output);
        }
        Term::First { pair } | Term::Second { pair } => {
            collect_variables(pair, binder_depth, output);
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => {}
    }
}

fn variable_occurrences(term: &Term, target: u32) -> usize {
    fn visit(term: &Term, target: u32, depth: u32) -> usize {
        match term {
            Term::Var { index } => usize::from(*index == target.saturating_add(depth)),
            Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
                visit(parameter, target, depth) + visit(body, target, depth.saturating_add(1))
            }
            Term::Lambda {
                parameter_type,
                body,
            } => {
                visit(parameter_type, target, depth) + visit(body, target, depth.saturating_add(1))
            }
            Term::Apply { function, argument } => {
                visit(function, target, depth) + visit(argument, target, depth)
            }
            Term::Pair {
                sigma_type,
                first,
                second,
            } => {
                visit(sigma_type, target, depth)
                    + visit(first, target, depth)
                    + visit(second, target, depth)
            }
            Term::First { pair } | Term::Second { pair } => visit(pair, target, depth),
            Term::Sort { .. } | Term::Global { .. } | Term::UnitType | Term::Unit => 0,
        }
    }
    visit(term, target, 0)
}

#[cfg(test)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum RewriteTamper {
    AlteredHead,
    AlteredConstructor,
    ReversedOrientation,
    AlteredRightSide,
    AlteredType,
    BodyfulHead,
    DuplicateRule,
    MissingRule,
    OldHeadRewrite,
}

#[cfg(test)]
pub(crate) fn rejects_rewrite_tamper_for_test(
    kernel: &Kernel,
    boundary: &VerifiedSignature,
    semantic: &VerifiedGscSemanticManifest,
    frame: &VerifiedClosedFormerFrame,
    use_family: &VerifiedCanonicalDemandFamilyV2,
    compute_family: &VerifiedCanonicalDemandFamilyV2,
    tamper: RewriteTamper,
) -> bool {
    let Ok(mut candidate) = reconstruct_candidate(
        kernel,
        boundary,
        semantic,
        frame,
        use_family,
        compute_family,
    ) else {
        return false;
    };
    let mut set_size = 1;
    match tamper {
        RewriteTamper::AlteredHead => {
            candidate.declaration.id = GlobalId(Digest::of_bytes(b"altered fresh head"));
        }
        RewriteTamper::AlteredConstructor => {
            candidate.constructor = ConstructorPortId(Digest::of_bytes(b"altered constructor"));
        }
        RewriteTamper::ReversedOrientation => {
            std::mem::swap(&mut candidate.left, &mut candidate.right);
        }
        RewriteTamper::AlteredRightSide => {
            candidate.right = Term::Var { index: 1 };
        }
        RewriteTamper::AlteredType => {
            candidate.ty = Term::UnitType;
        }
        RewriteTamper::BodyfulHead => {
            candidate.declaration.body = Some(Term::Unit);
        }
        RewriteTamper::DuplicateRule => set_size = 2,
        RewriteTamper::MissingRule => set_size = 0,
        RewriteTamper::OldHeadRewrite => {
            candidate.declaration.id = frame.owner().clone();
            candidate.left = Term::Global {
                id: frame.owner().clone(),
            };
        }
    }
    matches!(
        verify_reconstructed_candidate(
            EquationVerificationInputs {
                kernel,
                boundary,
                semantic,
                frame,
                use_family,
                compute_family,
            },
            candidate,
            set_size,
        ),
        GscOutcome::Unknown(_)
    )
}
