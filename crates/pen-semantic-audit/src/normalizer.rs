//! Restricted, fail-closed Q0 normalization.
//!
//! The kernel remains the authority for ordinary beta/delta normalization and
//! typing.  This module adds exactly one small grammar of fresh computation
//! rules: an opaque fresh head whose final (designated) argument is an opaque
//! nullary constructor reduces to the argument immediately preceding that
//! scrutinee.  Callers provide no left- or right-hand rewrite terms; both are
//! reconstructed here and checked by `pen-kernel`.

use crate::fragment::lambda_unit_term_syntax_violation;
use crate::manifest::{
    AuditDecision, AuditUnknownReason, OutsideFragmentReason, Q0RuleV1,
    SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1, VerifiedSemanticAuditManifestV1,
};
use crate::model::{GenericJudgmentV1, SourceNormalizedJudgmentV1};
use pen_kernel::{
    CanonicalEncode, CanonicalEncoder, Declaration, DependentContext, Digest, GlobalId, Kernel,
    KernelError, OpenJudgment, Term, UncheckedSignature, VerifiedSignature,
};
use std::collections::BTreeSet;

/// A constructor pattern for the restricted fresh-computation grammar.
///
/// The designated scrutinee must be the final parameter of the fresh head.
/// Its result is not supplied here: it is always reconstructed as the
/// immediately preceding parameter.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreshConstructorClauseV1 {
    pub constructor: GlobalId,
    pub scrutinee_parameter_ordinal: u16,
}

impl CanonicalEncode for FreshConstructorClauseV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.constructor.encode_canonical(encoder);
        encoder.u16(self.scrutinee_parameter_ordinal);
    }
}

/// Untrusted input for one fresh, nonrecursive constructor-computation
/// program.  No arbitrary rewrite payload is accepted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreshConstructorComputationRequestV1 {
    pub fresh_declaration: Declaration,
    pub clauses: Vec<FreshConstructorClauseV1>,
}

impl CanonicalEncode for FreshConstructorComputationRequestV1 {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.fresh_declaration.encode_canonical(encoder);
        encoder.sequence(&self.clauses);
    }
}

/// The exact, narrow scope of the certificates emitted by this module.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RestrictedQ0CertificateScopeV1 {
    /// Kernel beta/delta normal forms extended only by root-spine rules of the
    /// form `fresh(prefix, constructor) -> last(prefix)`.
    KernelNormalFormsAndFreshNullaryConstructorSpines,
}

/// A termination certificate for the restricted grammar, not for arbitrary
/// user-provided rewriting.
#[derive(Clone, Debug)]
pub struct RestrictedQ0TerminationCertificateV1 {
    scope: RestrictedQ0CertificateScopeV1,
    kernel_normalizer_protocol_digest: Digest,
    program_digest: Digest,
    fresh_rhs_contains_no_fresh_head: bool,
    each_fresh_step_removes_its_root_head: bool,
    dependency_graph_is_acyclic: bool,
}

impl RestrictedQ0TerminationCertificateV1 {
    pub fn scope(&self) -> RestrictedQ0CertificateScopeV1 {
        self.scope
    }

    pub fn kernel_normalizer_protocol_digest(&self) -> &Digest {
        &self.kernel_normalizer_protocol_digest
    }

    pub fn program_digest(&self) -> &Digest {
        &self.program_digest
    }

    pub fn fresh_rhs_contains_no_fresh_head(&self) -> bool {
        self.fresh_rhs_contains_no_fresh_head
    }

    pub fn each_fresh_step_removes_its_root_head(&self) -> bool {
        self.each_fresh_step_removes_its_root_head
    }

    pub fn dependency_graph_is_acyclic(&self) -> bool {
        self.dependency_graph_is_acyclic
    }
}

/// A confluence certificate for the same restricted grammar.
///
/// It claims only left-linearity and absence of critical pairs for the
/// reconstructed constructor patterns.  Ordinary conversion is delegated to
/// the bound kernel normalizer.
#[derive(Clone, Debug)]
pub struct RestrictedQ0ConfluenceCertificateV1 {
    scope: RestrictedQ0CertificateScopeV1,
    kernel_normalizer_protocol_digest: Digest,
    program_digest: Digest,
    patterns_are_left_linear: bool,
    constructor_patterns_are_pairwise_disjoint: bool,
    fresh_head_has_no_delta_body: bool,
}

impl RestrictedQ0ConfluenceCertificateV1 {
    pub fn scope(&self) -> RestrictedQ0CertificateScopeV1 {
        self.scope
    }

    pub fn kernel_normalizer_protocol_digest(&self) -> &Digest {
        &self.kernel_normalizer_protocol_digest
    }

    pub fn program_digest(&self) -> &Digest {
        &self.program_digest
    }

    pub fn patterns_are_left_linear(&self) -> bool {
        self.patterns_are_left_linear
    }

    pub fn constructor_patterns_are_pairwise_disjoint(&self) -> bool {
        self.constructor_patterns_are_pairwise_disjoint
    }

    pub fn fresh_head_has_no_delta_body(&self) -> bool {
        self.fresh_head_has_no_delta_body
    }
}

/// A reconstructed and kernel-typed fresh computation rule.
#[derive(Clone, Debug)]
pub struct VerifiedFreshConstructorRuleV1 {
    fresh_head: GlobalId,
    constructor: GlobalId,
    arity: u16,
    scrutinee_parameter_ordinal: u16,
    context: DependentContext,
    left: Term,
    right: Term,
    ty: Term,
}

impl VerifiedFreshConstructorRuleV1 {
    pub fn fresh_head(&self) -> &GlobalId {
        &self.fresh_head
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
}

/// Verified ownership, typing, termination, and nonoverlap for a restricted
/// fresh-computation program.
#[derive(Clone, Debug)]
pub struct VerifiedFreshConstructorComputationV1 {
    manifest_digest: Digest,
    boundary_signature_digest: Digest,
    extended_signature: VerifiedSignature,
    fresh_head: GlobalId,
    rules: Vec<VerifiedFreshConstructorRuleV1>,
    program_digest: Digest,
    termination: RestrictedQ0TerminationCertificateV1,
    confluence: RestrictedQ0ConfluenceCertificateV1,
}

impl VerifiedFreshConstructorComputationV1 {
    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    pub fn boundary_signature_digest(&self) -> &Digest {
        &self.boundary_signature_digest
    }

    pub fn extended_signature(&self) -> &VerifiedSignature {
        &self.extended_signature
    }

    pub fn fresh_head(&self) -> &GlobalId {
        &self.fresh_head
    }

    pub fn rules(&self) -> &[VerifiedFreshConstructorRuleV1] {
        &self.rules
    }

    pub fn program_digest(&self) -> &Digest {
        &self.program_digest
    }

    pub fn termination_certificate(&self) -> &RestrictedQ0TerminationCertificateV1 {
        &self.termination
    }

    pub fn confluence_certificate(&self) -> &RestrictedQ0ConfluenceCertificateV1 {
        &self.confluence
    }
}

/// Evidence that an exact source/claimed-normal pair was replayed.
#[derive(Clone, Debug)]
pub struct VerifiedSourceNormalizedJudgmentV1 {
    source_identity: Digest,
    normalized: GenericJudgmentV1,
    normalized_digest: Digest,
    signature_digest: Digest,
    manifest_digest: Digest,
    normalizer_protocol_digest: Digest,
    fresh_program_digest: Option<Digest>,
}

impl VerifiedSourceNormalizedJudgmentV1 {
    pub fn source_identity(&self) -> &Digest {
        &self.source_identity
    }

    pub fn normalized(&self) -> &GenericJudgmentV1 {
        &self.normalized
    }

    pub fn normalized_digest(&self) -> &Digest {
        &self.normalized_digest
    }

    pub fn signature_digest(&self) -> &Digest {
        &self.signature_digest
    }

    pub fn manifest_digest(&self) -> &Digest {
        &self.manifest_digest
    }

    pub fn normalizer_protocol_digest(&self) -> &Digest {
        &self.normalizer_protocol_digest
    }

    pub fn fresh_program_digest(&self) -> Option<&Digest> {
        self.fresh_program_digest.as_ref()
    }
}

#[derive(Clone, Debug)]
enum AuditFailure {
    Outside(OutsideFragmentReason),
    Unknown(AuditUnknownReason),
}

type AuditResult<T> = Result<T, AuditFailure>;

fn decision<T>(result: AuditResult<T>) -> AuditDecision<T> {
    match result {
        Ok(value) => AuditDecision::Proven(value),
        Err(AuditFailure::Outside(reason)) => AuditDecision::OutsideFragment(reason),
        Err(AuditFailure::Unknown(reason)) => AuditDecision::Unknown(reason),
    }
}

fn kernel_failure(error: KernelError) -> AuditFailure {
    match error {
        KernelError::ResourceExhausted(_) => {
            AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted)
        }
        _ => AuditFailure::Unknown(AuditUnknownReason::KernelCouldNotCertify),
    }
}

fn malformed() -> AuditFailure {
    AuditFailure::Unknown(AuditUnknownReason::MalformedInput)
}

/// Verify and reconstruct a caller-safe fresh constructor-computation
/// program.
pub fn verify_fresh_constructor_computation_v1(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    boundary: &VerifiedSignature,
    request: &FreshConstructorComputationRequestV1,
) -> AuditDecision<VerifiedFreshConstructorComputationV1> {
    decision(verify_fresh_constructor_computation_inner(
        manifest, kernel, boundary, request,
    ))
}

fn verify_fresh_constructor_computation_inner(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    boundary: &VerifiedSignature,
    request: &FreshConstructorComputationRequestV1,
) -> AuditResult<VerifiedFreshConstructorComputationV1> {
    let required = [
        Q0RuleV1::DeBruijn,
        Q0RuleV1::SequentialSubstitution,
        Q0RuleV1::Beta,
        Q0RuleV1::ProvenancePreservingDelta,
        Q0RuleV1::FreshNonrecursiveConstructorComputation,
    ];
    if !required
        .iter()
        .all(|rule| manifest.manifest().q0_rules.contains(rule))
        || !manifest.manifest().q0_eta_registry_empty
    {
        return Err(AuditFailure::Unknown(AuditUnknownReason::ManifestMismatch));
    }
    if request.clauses.is_empty()
        || request.clauses.len() > usize::from(manifest.manifest().maximum_seeds)
        || request.fresh_declaration.body.is_some()
    {
        return Err(malformed());
    }

    let mut work = WorkBudget::new(kernel);
    check_signature_fragment(boundary, manifest, &mut work)?;
    check_term_fragment(&request.fresh_declaration.ty, manifest, &mut work, 0)?;

    let fresh_head = request.fresh_declaration.id.clone();
    if boundary
        .declarations()
        .iter()
        .any(|declaration| declaration.id == fresh_head)
    {
        return Err(malformed());
    }
    if contains_global(&request.fresh_declaration.ty, &fresh_head, &mut work, 0)? {
        return Err(AuditFailure::Outside(
            OutsideFragmentReason::RecursiveRewrite,
        ));
    }

    let extension = UncheckedSignature {
        declarations: vec![request.fresh_declaration.clone()],
    };
    let extended = kernel
        .verify_extension(boundary, &extension)
        .map_err(kernel_failure)?;
    let Some(normal_fresh) = extended.declarations().last() else {
        return Err(malformed());
    };
    if normal_fresh.id != fresh_head || normal_fresh.body.is_some() {
        return Err(malformed());
    }

    let (parameters, result_type) = pi_telescope(
        &normal_fresh.ty,
        &mut work,
        manifest.manifest().maximum_context_entries,
    )?;
    if parameters.len() < 2 {
        return Err(malformed());
    }
    let arity = u16::try_from(parameters.len()).map_err(|_| malformed())?;
    let expected_scrutinee = arity.checked_sub(1).ok_or_else(malformed)?;

    let mut seen_constructors = BTreeSet::new();
    for clause in &request.clauses {
        if clause.scrutinee_parameter_ordinal != expected_scrutinee {
            return Err(malformed());
        }
        if !seen_constructors.insert(clause.constructor.clone()) {
            return Err(AuditFailure::Outside(
                OutsideFragmentReason::OverlappingRewrite,
            ));
        }
    }

    let prefix_len = parameters.len() - 1;
    let prefix_context = DependentContext(parameters[..prefix_len].to_vec());
    let scrutinee_type = parameters[prefix_len].clone();
    let mut rules = Vec::with_capacity(request.clauses.len());

    for clause in &request.clauses {
        if clause.constructor == fresh_head {
            return Err(AuditFailure::Outside(
                OutsideFragmentReason::RecursiveRewrite,
            ));
        }
        let Some(constructor_declaration) = boundary
            .declarations()
            .iter()
            .find(|declaration| declaration.id == clause.constructor)
        else {
            return Err(malformed());
        };
        // A transparent constructor would disappear during the kernel's delta
        // pass and therefore cannot own a stable constructor pattern.
        if constructor_declaration.body.is_some() {
            return Err(malformed());
        }

        let constructor = Term::Global {
            id: clause.constructor.clone(),
        };
        let mut arguments = context_variables(prefix_len)?;
        arguments.push(constructor.clone());
        let left = apply_spine(
            Term::Global {
                id: fresh_head.clone(),
            },
            arguments,
        );
        let right = Term::Var { index: 0 };
        let ty = subst_top(&result_type, &constructor, &mut work, 0)?;

        let constructor_judgment = OpenJudgment::HasType {
            context: prefix_context.clone(),
            term: constructor,
            ty: scrutinee_type.clone(),
        };
        let left_judgment = OpenJudgment::HasType {
            context: prefix_context.clone(),
            term: left,
            ty: ty.clone(),
        };
        let right_judgment = OpenJudgment::HasType {
            context: prefix_context.clone(),
            term: right,
            ty,
        };
        let checked = kernel
            .verify_open_judgments(
                &extended,
                &[&constructor_judgment, &left_judgment, &right_judgment],
            )
            .map_err(kernel_failure)?;
        let [
            OpenJudgment::HasType {
                context: constructor_context,
                ..
            },
            OpenJudgment::HasType {
                context,
                term: left,
                ty,
            },
            OpenJudgment::HasType {
                context: right_context,
                term: right,
                ty: right_ty,
            },
        ] = checked.as_slice()
        else {
            return Err(AuditFailure::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            ));
        };
        if constructor_context != context || right_context != context || right_ty != ty {
            return Err(AuditFailure::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            ));
        }
        if contains_global(right, &fresh_head, &mut work, 0)? {
            return Err(AuditFailure::Outside(
                OutsideFragmentReason::RecursiveRewrite,
            ));
        }
        if !is_reconstructed_pattern(
            left,
            &fresh_head,
            &clause.constructor,
            usize::from(expected_scrutinee),
        ) || right != &(Term::Var { index: 0 })
        {
            return Err(AuditFailure::Unknown(
                AuditUnknownReason::KernelCouldNotCertify,
            ));
        }
        rules.push(VerifiedFreshConstructorRuleV1 {
            fresh_head: fresh_head.clone(),
            constructor: clause.constructor.clone(),
            arity,
            scrutinee_parameter_ordinal: expected_scrutinee,
            context: context.clone(),
            left: left.clone(),
            right: right.clone(),
            ty: ty.clone(),
        });
    }

    let program_digest = Digest::of_canonical(
        "pen-semantic-audit/restricted-fresh-computation-program/v1",
        &ProgramDigestMaterial {
            manifest_digest: manifest.candidate_digest(),
            boundary_signature_digest: boundary.digest(),
            extended_signature_digest: extended.digest(),
            request,
        },
    );
    let normalizer_digest = kernel.normalizer_protocol_digest();
    let scope = RestrictedQ0CertificateScopeV1::KernelNormalFormsAndFreshNullaryConstructorSpines;
    let termination = RestrictedQ0TerminationCertificateV1 {
        scope,
        kernel_normalizer_protocol_digest: normalizer_digest.clone(),
        program_digest: program_digest.clone(),
        fresh_rhs_contains_no_fresh_head: true,
        each_fresh_step_removes_its_root_head: true,
        dependency_graph_is_acyclic: true,
    };
    let confluence = RestrictedQ0ConfluenceCertificateV1 {
        scope,
        kernel_normalizer_protocol_digest: normalizer_digest,
        program_digest: program_digest.clone(),
        patterns_are_left_linear: true,
        constructor_patterns_are_pairwise_disjoint: true,
        fresh_head_has_no_delta_body: true,
    };
    Ok(VerifiedFreshConstructorComputationV1 {
        manifest_digest: manifest.candidate_digest().clone(),
        boundary_signature_digest: boundary.digest().clone(),
        extended_signature: extended,
        fresh_head,
        rules,
        program_digest,
        termination,
        confluence,
    })
}

struct ProgramDigestMaterial<'a> {
    manifest_digest: &'a Digest,
    boundary_signature_digest: &'a Digest,
    extended_signature_digest: &'a Digest,
    request: &'a FreshConstructorComputationRequestV1,
}

impl CanonicalEncode for ProgramDigestMaterial<'_> {
    fn encode_canonical(&self, encoder: &mut CanonicalEncoder) {
        self.manifest_digest.encode_canonical(encoder);
        self.boundary_signature_digest.encode_canonical(encoder);
        self.extended_signature_digest.encode_canonical(encoder);
        self.request.encode_canonical(encoder);
    }
}

/// Check that the claimed judgment is the exact normal form computed from its
/// source.  Both source and claim are independently typed; equations have
/// each side typed separately because a certified fresh equation is not an
/// ordinary kernel definitional equality before this restricted extension is
/// applied.
pub fn verify_source_normalized_judgment_v1(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    pair: &SourceNormalizedJudgmentV1,
    fresh: Option<&VerifiedFreshConstructorComputationV1>,
) -> AuditDecision<VerifiedSourceNormalizedJudgmentV1> {
    decision(verify_source_normalized_judgment_inner(
        manifest, kernel, signature, pair, fresh,
    ))
}

pub(crate) fn normalize_generated_judgment_v1(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    judgment: &GenericJudgmentV1,
    fresh: Option<&VerifiedFreshConstructorComputationV1>,
) -> AuditDecision<GenericJudgmentV1> {
    if let Some(program) = fresh {
        if program.manifest_digest() != manifest.candidate_digest()
            || program.extended_signature().digest() != signature.digest()
        {
            return AuditDecision::Unknown(AuditUnknownReason::ManifestMismatch);
        }
    }
    let mut work = WorkBudget::new(kernel);
    decision(
        check_judgment_fragment(judgment, manifest, &mut work).and_then(|()| {
            normalize_generic_judgment(kernel, signature, judgment, fresh, &mut work)
        }),
    )
}

fn verify_source_normalized_judgment_inner(
    manifest: &VerifiedSemanticAuditManifestV1,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    pair: &SourceNormalizedJudgmentV1,
    fresh: Option<&VerifiedFreshConstructorComputationV1>,
) -> AuditResult<VerifiedSourceNormalizedJudgmentV1> {
    if let Some(program) = fresh {
        if program.manifest_digest() != manifest.candidate_digest()
            || program.extended_signature().digest() != signature.digest()
        {
            return Err(AuditFailure::Unknown(AuditUnknownReason::ManifestMismatch));
        }
    }
    let mut work = WorkBudget::new(kernel);
    check_judgment_fragment(&pair.source, manifest, &mut work)?;
    check_judgment_fragment(&pair.claimed_normalized, manifest, &mut work)?;

    let normalized = normalize_generic_judgment(kernel, signature, &pair.source, fresh, &mut work)?;
    let normalized_claim = normalize_generic_judgment(
        kernel,
        signature,
        &pair.claimed_normalized,
        fresh,
        &mut work,
    )?;
    // Exactness is deliberate: merely being convertible is not an accepted
    // source/normal pair, and a non-normal claim cannot certify itself.
    if normalized != pair.claimed_normalized || normalized_claim != pair.claimed_normalized {
        return Err(AuditFailure::Unknown(
            AuditUnknownReason::NormalizationFailure,
        ));
    }

    Ok(VerifiedSourceNormalizedJudgmentV1 {
        source_identity: pair.source_identity.clone(),
        normalized_digest: Digest::of_canonical(
            "pen-semantic-audit/restricted-q0-normalized-judgment/v1",
            &normalized,
        ),
        normalized,
        signature_digest: signature.digest().clone(),
        manifest_digest: manifest.candidate_digest().clone(),
        normalizer_protocol_digest: kernel.normalizer_protocol_digest(),
        fresh_program_digest: fresh.map(|program| program.program_digest().clone()),
    })
}

fn normalize_generic_judgment(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    judgment: &GenericJudgmentV1,
    fresh: Option<&VerifiedFreshConstructorComputationV1>,
    work: &mut WorkBudget,
) -> AuditResult<GenericJudgmentV1> {
    let mut current = kernel_normalize_generic(kernel, signature, judgment)?;
    let Some(program) = fresh else {
        return Ok(current);
    };

    loop {
        work.rewrite()?;
        let (rewritten, changed) = fresh_normalize_generic(&current, program, work)?;
        if !changed {
            return Ok(current);
        }
        current = kernel_normalize_generic(kernel, signature, &rewritten)?;
    }
}

fn kernel_normalize_generic(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    judgment: &GenericJudgmentV1,
) -> AuditResult<GenericJudgmentV1> {
    match judgment {
        GenericJudgmentV1::Term { context, term, ty } => {
            let checked = kernel
                .verify_open_judgment(
                    signature,
                    &OpenJudgment::HasType {
                        context: context.clone(),
                        term: term.clone(),
                        ty: ty.clone(),
                    },
                )
                .map_err(kernel_failure)?;
            let OpenJudgment::HasType { context, term, ty } = checked else {
                return Err(AuditFailure::Unknown(
                    AuditUnknownReason::KernelCouldNotCertify,
                ));
            };
            Ok(GenericJudgmentV1::Term { context, term, ty })
        }
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } => {
            let left_judgment = OpenJudgment::HasType {
                context: context.clone(),
                term: left.clone(),
                ty: ty.clone(),
            };
            let right_judgment = OpenJudgment::HasType {
                context: context.clone(),
                term: right.clone(),
                ty: ty.clone(),
            };
            let checked = kernel
                .verify_open_judgments(signature, &[&left_judgment, &right_judgment])
                .map_err(kernel_failure)?;
            let [
                OpenJudgment::HasType {
                    context: left_context,
                    term: left,
                    ty: left_ty,
                },
                OpenJudgment::HasType {
                    context: right_context,
                    term: right,
                    ty: right_ty,
                },
            ] = checked.as_slice()
            else {
                return Err(AuditFailure::Unknown(
                    AuditUnknownReason::KernelCouldNotCertify,
                ));
            };
            if left_context != right_context || left_ty != right_ty {
                return Err(AuditFailure::Unknown(
                    AuditUnknownReason::KernelCouldNotCertify,
                ));
            }
            Ok(GenericJudgmentV1::Equation {
                context: left_context.clone(),
                left: left.clone(),
                right: right.clone(),
                ty: left_ty.clone(),
            })
        }
    }
}

fn fresh_normalize_generic(
    judgment: &GenericJudgmentV1,
    program: &VerifiedFreshConstructorComputationV1,
    work: &mut WorkBudget,
) -> AuditResult<(GenericJudgmentV1, bool)> {
    match judgment {
        GenericJudgmentV1::Term { context, term, ty } => {
            let (context, context_changed) = fresh_normalize_context(context, program, work)?;
            let (term, term_changed) = fresh_normalize_term(term, program, work, 0)?;
            let (ty, ty_changed) = fresh_normalize_term(ty, program, work, 0)?;
            Ok((
                GenericJudgmentV1::Term { context, term, ty },
                context_changed || term_changed || ty_changed,
            ))
        }
        GenericJudgmentV1::Equation {
            context,
            left,
            right,
            ty,
        } => {
            let (context, context_changed) = fresh_normalize_context(context, program, work)?;
            let (left, left_changed) = fresh_normalize_term(left, program, work, 0)?;
            let (right, right_changed) = fresh_normalize_term(right, program, work, 0)?;
            let (ty, ty_changed) = fresh_normalize_term(ty, program, work, 0)?;
            Ok((
                GenericJudgmentV1::Equation {
                    context,
                    left,
                    right,
                    ty,
                },
                context_changed || left_changed || right_changed || ty_changed,
            ))
        }
    }
}

fn fresh_normalize_context(
    context: &DependentContext,
    program: &VerifiedFreshConstructorComputationV1,
    work: &mut WorkBudget,
) -> AuditResult<(DependentContext, bool)> {
    let mut changed = false;
    let mut entries = Vec::with_capacity(context.0.len());
    for entry in &context.0 {
        let (entry, entry_changed) = fresh_normalize_term(entry, program, work, 0)?;
        entries.push(entry);
        changed |= entry_changed;
    }
    Ok((DependentContext(entries), changed))
}

fn fresh_normalize_term(
    term: &Term,
    program: &VerifiedFreshConstructorComputationV1,
    work: &mut WorkBudget,
    depth: u16,
) -> AuditResult<(Term, bool)> {
    work.enter(depth)?;
    let child = depth
        .checked_add(1)
        .ok_or(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    let (rebuilt, changed) = match term {
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => (term.clone(), false),
        Term::Pi { parameter, body } => {
            let (parameter, first) = fresh_normalize_term(parameter, program, work, child)?;
            let (body, second) = fresh_normalize_term(body, program, work, child)?;
            (
                Term::Pi {
                    parameter: Box::new(parameter),
                    body: Box::new(body),
                },
                first || second,
            )
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            let (parameter_type, first) =
                fresh_normalize_term(parameter_type, program, work, child)?;
            let (body, second) = fresh_normalize_term(body, program, work, child)?;
            (
                Term::Lambda {
                    parameter_type: Box::new(parameter_type),
                    body: Box::new(body),
                },
                first || second,
            )
        }
        Term::Apply { function, argument } => {
            let (function, first) = fresh_normalize_term(function, program, work, child)?;
            let (argument, second) = fresh_normalize_term(argument, program, work, child)?;
            (
                Term::Apply {
                    function: Box::new(function),
                    argument: Box::new(argument),
                },
                first || second,
            )
        }
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            return Err(AuditFailure::Outside(
                OutsideFragmentReason::UnsupportedTerm,
            ));
        }
    };

    let (head, arguments) = split_spine(rebuilt);
    let Term::Global { id } = &head else {
        return Ok((apply_spine(head, arguments), changed));
    };
    if id != program.fresh_head() {
        return Ok((apply_spine(head, arguments), changed));
    }
    for rule in program.rules() {
        let arity = usize::from(rule.arity());
        let scrutinee = usize::from(rule.scrutinee_parameter_ordinal());
        if arguments.len() < arity
            || arguments.get(scrutinee)
                != Some(&Term::Global {
                    id: rule.constructor().clone(),
                })
        {
            continue;
        }
        // The verifier established `scrutinee > 0` and reconstructed the RHS
        // as the immediately preceding argument.
        let mut result = arguments[scrutinee - 1].clone();
        result = apply_spine(result, arguments[arity..].to_vec());
        let (result, _) = fresh_normalize_term(&result, program, work, child)?;
        return Ok((result, true));
    }
    Ok((apply_spine(head, arguments), changed))
}

fn check_signature_fragment(
    signature: &VerifiedSignature,
    manifest: &VerifiedSemanticAuditManifestV1,
    work: &mut WorkBudget,
) -> AuditResult<()> {
    for declaration in signature.declarations() {
        check_term_fragment(&declaration.ty, manifest, work, 0)?;
        if let Some(body) = &declaration.body {
            check_term_fragment(body, manifest, work, 0)?;
        }
    }
    Ok(())
}

fn check_judgment_fragment(
    judgment: &GenericJudgmentV1,
    manifest: &VerifiedSemanticAuditManifestV1,
    work: &mut WorkBudget,
) -> AuditResult<()> {
    if judgment.context().0.len() > usize::from(manifest.manifest().maximum_context_entries) {
        return Err(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted));
    }
    for entry in &judgment.context().0 {
        check_term_fragment(entry, manifest, work, 0)?;
    }
    match judgment {
        GenericJudgmentV1::Term { term, ty, .. } => {
            check_term_fragment(term, manifest, work, 0)?;
            check_term_fragment(ty, manifest, work, 0)?;
        }
        GenericJudgmentV1::Equation {
            left, right, ty, ..
        } => {
            check_term_fragment(left, manifest, work, 0)?;
            check_term_fragment(right, manifest, work, 0)?;
            check_term_fragment(ty, manifest, work, 0)?;
        }
    }
    Ok(())
}

fn check_term_fragment(
    term: &Term,
    manifest: &VerifiedSemanticAuditManifestV1,
    work: &mut WorkBudget,
    depth: u16,
) -> AuditResult<()> {
    work.enter(depth)?;
    if depth == 0
        && manifest.manifest().profile_id == SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1
        && let Some(violation) =
            lambda_unit_term_syntax_violation(term, &manifest.manifest().universe_levels)
    {
        return Err(AuditFailure::Outside(violation.outside_reason()));
    }
    let child = depth
        .checked_add(1)
        .ok_or(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    match term {
        Term::Sort { level } => {
            if !manifest.manifest().universe_levels.contains(level) {
                return Err(AuditFailure::Outside(
                    OutsideFragmentReason::UnsupportedUniverseLevel,
                ));
            }
        }
        Term::Var { .. } | Term::Global { .. } | Term::UnitType | Term::Unit => {}
        Term::Pi { parameter, body } => {
            check_term_fragment(parameter, manifest, work, child)?;
            check_term_fragment(body, manifest, work, child)?;
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            check_term_fragment(parameter_type, manifest, work, child)?;
            check_term_fragment(body, manifest, work, child)?;
        }
        Term::Apply { function, argument } => {
            check_term_fragment(function, manifest, work, child)?;
            check_term_fragment(argument, manifest, work, child)?;
        }
        Term::First { .. } | Term::Second { .. }
            if manifest.manifest().profile_id == SEMANTIC_AUDIT_LAMBDA_UNIT_PROFILE_ID_V1 =>
        {
            return Err(AuditFailure::Outside(
                OutsideFragmentReason::DescriptorProjection,
            ));
        }
        Term::Sigma { .. } | Term::Pair { .. } | Term::First { .. } | Term::Second { .. } => {
            return Err(AuditFailure::Outside(
                OutsideFragmentReason::UnsupportedTerm,
            ));
        }
    }
    Ok(())
}

fn pi_telescope(
    term: &Term,
    work: &mut WorkBudget,
    maximum_context_entries: u16,
) -> AuditResult<(Vec<Term>, Term)> {
    let mut parameters = Vec::new();
    let mut cursor = term;
    loop {
        work.enter(0)?;
        let Term::Pi { parameter, body } = cursor else {
            break;
        };
        if parameters.len() >= usize::from(maximum_context_entries) {
            return Err(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted));
        }
        parameters.push((**parameter).clone());
        cursor = body;
    }
    Ok((parameters, cursor.clone()))
}

fn context_variables(length: usize) -> AuditResult<Vec<Term>> {
    (0..length)
        .map(|ordinal| {
            let index = length
                .checked_sub(ordinal + 1)
                .and_then(|index| u32::try_from(index).ok())
                .ok_or_else(malformed)?;
            Ok(Term::Var { index })
        })
        .collect()
}

fn apply_spine(mut head: Term, arguments: Vec<Term>) -> Term {
    for argument in arguments {
        head = Term::Apply {
            function: Box::new(head),
            argument: Box::new(argument),
        };
    }
    head
}

fn split_spine(mut term: Term) -> (Term, Vec<Term>) {
    let mut reversed = Vec::new();
    while let Term::Apply { function, argument } = term {
        reversed.push(*argument);
        term = *function;
    }
    reversed.reverse();
    (term, reversed)
}

fn is_reconstructed_pattern(
    term: &Term,
    fresh_head: &GlobalId,
    constructor: &GlobalId,
    scrutinee: usize,
) -> bool {
    let (head, arguments) = split_spine(term.clone());
    head == (Term::Global {
        id: fresh_head.clone(),
    }) && arguments.len() == scrutinee + 1
        && arguments.get(scrutinee)
            == Some(&Term::Global {
                id: constructor.clone(),
            })
        && arguments[..scrutinee]
            .iter()
            .enumerate()
            .all(|(ordinal, argument)| {
                argument
                    == &Term::Var {
                        index: u32::try_from(scrutinee - ordinal - 1).unwrap_or(u32::MAX),
                    }
            })
}

fn contains_global(
    term: &Term,
    target: &GlobalId,
    work: &mut WorkBudget,
    depth: u16,
) -> AuditResult<bool> {
    work.enter(depth)?;
    let child = depth
        .checked_add(1)
        .ok_or(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    Ok(match term {
        Term::Global { id } => id == target,
        Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => false,
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            contains_global(parameter, target, work, child)?
                || contains_global(body, target, work, child)?
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            contains_global(parameter_type, target, work, child)?
                || contains_global(body, target, work, child)?
        }
        Term::Apply { function, argument } => {
            contains_global(function, target, work, child)?
                || contains_global(argument, target, work, child)?
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            contains_global(sigma_type, target, work, child)?
                || contains_global(first, target, work, child)?
                || contains_global(second, target, work, child)?
        }
        Term::First { pair } | Term::Second { pair } => contains_global(pair, target, work, child)?,
    })
}

fn subst_top(
    body: &Term,
    replacement: &Term,
    work: &mut WorkBudget,
    depth: u16,
) -> AuditResult<Term> {
    let lifted = shift(replacement, 1, 0, work, depth)?;
    let replaced = substitute(body, 0, &lifted, 0, work, depth)?;
    shift(&replaced, -1, 0, work, depth)
}

fn substitute(
    term: &Term,
    target: u32,
    replacement: &Term,
    binder_depth: u32,
    work: &mut WorkBudget,
    depth: u16,
) -> AuditResult<Term> {
    work.enter(depth)?;
    let child = depth
        .checked_add(1)
        .ok_or(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    let sought = target.checked_add(binder_depth).ok_or_else(malformed)?;
    match term {
        Term::Var { index } if *index == sought => {
            shift(replacement, i64::from(binder_depth), 0, work, child)
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
                work,
                child,
            )?),
            body: Box::new(substitute(
                body,
                target,
                replacement,
                binder_depth.checked_add(1).ok_or_else(malformed)?,
                work,
                child,
            )?),
        }),
        Term::Sigma { parameter, body } => Ok(Term::Sigma {
            parameter: Box::new(substitute(
                parameter,
                target,
                replacement,
                binder_depth,
                work,
                child,
            )?),
            body: Box::new(substitute(
                body,
                target,
                replacement,
                binder_depth.checked_add(1).ok_or_else(malformed)?,
                work,
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
                work,
                child,
            )?),
            body: Box::new(substitute(
                body,
                target,
                replacement,
                binder_depth.checked_add(1).ok_or_else(malformed)?,
                work,
                child,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(substitute(
                function,
                target,
                replacement,
                binder_depth,
                work,
                child,
            )?),
            argument: Box::new(substitute(
                argument,
                target,
                replacement,
                binder_depth,
                work,
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
                work,
                child,
            )?),
            first: Box::new(substitute(
                first,
                target,
                replacement,
                binder_depth,
                work,
                child,
            )?),
            second: Box::new(substitute(
                second,
                target,
                replacement,
                binder_depth,
                work,
                child,
            )?),
        }),
        Term::First { pair } => Ok(Term::First {
            pair: Box::new(substitute(
                pair,
                target,
                replacement,
                binder_depth,
                work,
                child,
            )?),
        }),
        Term::Second { pair } => Ok(Term::Second {
            pair: Box::new(substitute(
                pair,
                target,
                replacement,
                binder_depth,
                work,
                child,
            )?),
        }),
    }
}

fn shift(
    term: &Term,
    amount: i64,
    cutoff: u32,
    work: &mut WorkBudget,
    depth: u16,
) -> AuditResult<Term> {
    work.enter(depth)?;
    let child = depth
        .checked_add(1)
        .ok_or(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
    match term {
        Term::Var { index } if *index >= cutoff => {
            let shifted = i64::from(*index)
                .checked_add(amount)
                .and_then(|value| u32::try_from(value).ok())
                .ok_or_else(malformed)?;
            Ok(Term::Var { index: shifted })
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(term.clone()),
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(shift(parameter, amount, cutoff, work, child)?),
            body: Box::new(shift(
                body,
                amount,
                cutoff.checked_add(1).ok_or_else(malformed)?,
                work,
                child,
            )?),
        }),
        Term::Sigma { parameter, body } => Ok(Term::Sigma {
            parameter: Box::new(shift(parameter, amount, cutoff, work, child)?),
            body: Box::new(shift(
                body,
                amount,
                cutoff.checked_add(1).ok_or_else(malformed)?,
                work,
                child,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(shift(parameter_type, amount, cutoff, work, child)?),
            body: Box::new(shift(
                body,
                amount,
                cutoff.checked_add(1).ok_or_else(malformed)?,
                work,
                child,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(shift(function, amount, cutoff, work, child)?),
            argument: Box::new(shift(argument, amount, cutoff, work, child)?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Ok(Term::Pair {
            sigma_type: Box::new(shift(sigma_type, amount, cutoff, work, child)?),
            first: Box::new(shift(first, amount, cutoff, work, child)?),
            second: Box::new(shift(second, amount, cutoff, work, child)?),
        }),
        Term::First { pair } => Ok(Term::First {
            pair: Box::new(shift(pair, amount, cutoff, work, child)?),
        }),
        Term::Second { pair } => Ok(Term::Second {
            pair: Box::new(shift(pair, amount, cutoff, work, child)?),
        }),
    }
}

struct WorkBudget {
    operations_left: u32,
    rewrites_left: u32,
    depth_limit: u16,
}

impl WorkBudget {
    fn new(kernel: &Kernel) -> Self {
        let limits = kernel.limits();
        Self {
            operations_left: limits.max_operations,
            rewrites_left: limits.normalization_fuel,
            depth_limit: limits.max_depth,
        }
    }

    fn enter(&mut self, depth: u16) -> AuditResult<()> {
        if depth > self.depth_limit {
            return Err(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted));
        }
        self.operations_left = self
            .operations_left
            .checked_sub(1)
            .ok_or(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
        Ok(())
    }

    fn rewrite(&mut self) -> AuditResult<()> {
        self.rewrites_left = self
            .rewrites_left
            .checked_sub(1)
            .ok_or(AuditFailure::Unknown(AuditUnknownReason::ResourceExhausted))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        AuditFailure, FreshConstructorClauseV1, FreshConstructorComputationRequestV1, WorkBudget,
        check_term_fragment, verify_fresh_constructor_computation_v1,
        verify_source_normalized_judgment_v1,
    };
    use crate::manifest::{
        AuditDecision, AuditUnknownReason, OutsideFragmentReason,
        proposed_semantic_audit_lambda_unit_manifest_v1, proposed_semantic_audit_manifest_v1,
        verify_semantic_audit_lambda_unit_manifest_v1, verify_semantic_audit_manifest_v1,
    };
    use crate::model::{GenericJudgmentV1, SourceNormalizedJudgmentV1};
    use pen_kernel::{
        Declaration, DependentContext, Digest, GlobalId, Kernel, KernelLimits, Term,
        UncheckedSignature,
    };

    fn id(label: &[u8]) -> GlobalId {
        GlobalId(Digest::of_bytes(label))
    }

    fn manifest() -> crate::manifest::VerifiedSemanticAuditManifestV1 {
        let AuditDecision::Proven(manifest) =
            verify_semantic_audit_manifest_v1(&proposed_semantic_audit_manifest_v1())
        else {
            panic!("proposed manifest should verify");
        };
        manifest
    }

    fn pair(
        source: GenericJudgmentV1,
        normalized: GenericJudgmentV1,
    ) -> SourceNormalizedJudgmentV1 {
        SourceNormalizedJudgmentV1 {
            source_identity: Digest::of_bytes(b"normalizer-test-source"),
            source,
            claimed_normalized: normalized,
        }
    }

    #[test]
    fn lambda_unit_nested_projection_has_projection_precedence() {
        let AuditDecision::Proven(lambda_manifest) = verify_semantic_audit_lambda_unit_manifest_v1(
            &proposed_semantic_audit_lambda_unit_manifest_v1(),
        ) else {
            panic!("lambda/unit manifest");
        };
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let term = Term::Sigma {
            parameter: Box::new(Term::UnitType),
            body: Box::new(Term::First {
                pair: Box::new(Term::Var { index: 0 }),
            }),
        };
        assert!(matches!(
            check_term_fragment(&term, &lambda_manifest, &mut WorkBudget::new(&kernel), 0),
            Err(AuditFailure::Outside(
                OutsideFragmentReason::DescriptorProjection
            ))
        ));
    }

    #[test]
    fn ordinary_kernel_beta_is_replayed_exactly() {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let source = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Apply {
                function: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::UnitType),
                    body: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::Unit),
            },
            ty: Term::UnitType,
        };
        let normalized = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Unit,
            ty: Term::UnitType,
        };
        assert!(matches!(
            verify_source_normalized_judgment_v1(
                &manifest(),
                &kernel,
                &signature,
                &pair(source, normalized),
                None,
            ),
            AuditDecision::Proven(_)
        ));
    }

    fn fresh_fixture() -> (
        Kernel,
        crate::manifest::VerifiedSemanticAuditManifestV1,
        pen_kernel::VerifiedSignature,
        GlobalId,
        GlobalId,
    ) {
        let kernel = Kernel::new(KernelLimits::default()).expect("kernel");
        let constructor = id(b"opaque-nullary-constructor");
        let head = id(b"fresh-computation-head");
        let boundary = kernel
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: constructor.clone(),
                    ty: Term::UnitType,
                    body: None,
                }],
            })
            .expect("boundary");
        (kernel, manifest(), boundary, constructor, head)
    }

    fn request(head: GlobalId, constructor: GlobalId) -> FreshConstructorComputationRequestV1 {
        FreshConstructorComputationRequestV1 {
            fresh_declaration: Declaration {
                id: head,
                ty: Term::Pi {
                    parameter: Box::new(Term::UnitType),
                    body: Box::new(Term::Pi {
                        parameter: Box::new(Term::UnitType),
                        body: Box::new(Term::UnitType),
                    }),
                },
                body: None,
            },
            clauses: vec![FreshConstructorClauseV1 {
                constructor,
                scrutinee_parameter_ordinal: 1,
            }],
        }
    }

    #[test]
    fn fresh_constructor_spine_reduces_to_reconstructed_result() {
        let (kernel, manifest, boundary, constructor, head) = fresh_fixture();
        let AuditDecision::Proven(program) = verify_fresh_constructor_computation_v1(
            &manifest,
            &kernel,
            &boundary,
            &request(head.clone(), constructor.clone()),
        ) else {
            panic!("fresh program should verify");
        };
        let source_term = Term::Apply {
            function: Box::new(Term::Apply {
                function: Box::new(Term::Global { id: head }),
                argument: Box::new(Term::Unit),
            }),
            argument: Box::new(Term::Global { id: constructor }),
        };
        let source = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: source_term,
            ty: Term::UnitType,
        };
        let normalized = GenericJudgmentV1::Term {
            context: DependentContext::default(),
            term: Term::Unit,
            ty: Term::UnitType,
        };
        assert!(matches!(
            verify_source_normalized_judgment_v1(
                &manifest,
                &kernel,
                program.extended_signature(),
                &pair(source, normalized),
                Some(&program),
            ),
            AuditDecision::Proven(_)
        ));
        assert!(
            program
                .termination_certificate()
                .dependency_graph_is_acyclic()
        );
        assert!(
            program
                .confluence_certificate()
                .constructor_patterns_are_pairwise_disjoint()
        );
    }

    #[test]
    fn malformed_scrutinee_position_is_rejected() {
        let (kernel, manifest, boundary, constructor, head) = fresh_fixture();
        let mut malformed = request(head, constructor);
        malformed.clauses[0].scrutinee_parameter_ordinal = 0;
        assert!(matches!(
            verify_fresh_constructor_computation_v1(&manifest, &kernel, &boundary, &malformed,),
            AuditDecision::Unknown(AuditUnknownReason::MalformedInput)
        ));
    }

    #[test]
    fn recursive_fresh_type_is_rejected_before_kernel_admission() {
        let (kernel, manifest, boundary, constructor, head) = fresh_fixture();
        let recursive = FreshConstructorComputationRequestV1 {
            fresh_declaration: Declaration {
                id: head.clone(),
                ty: Term::Pi {
                    parameter: Box::new(Term::Global { id: head }),
                    body: Box::new(Term::UnitType),
                },
                body: None,
            },
            clauses: vec![FreshConstructorClauseV1 {
                constructor,
                scrutinee_parameter_ordinal: 0,
            }],
        };
        assert!(matches!(
            verify_fresh_constructor_computation_v1(&manifest, &kernel, &boundary, &recursive,),
            AuditDecision::OutsideFragment(OutsideFragmentReason::RecursiveRewrite)
        ));
    }

    #[test]
    fn duplicate_constructor_patterns_are_rejected_as_overlap() {
        let (kernel, manifest, boundary, constructor, head) = fresh_fixture();
        let mut overlap = request(head, constructor);
        overlap.clauses.push(overlap.clauses[0].clone());
        assert!(matches!(
            verify_fresh_constructor_computation_v1(&manifest, &kernel, &boundary, &overlap,),
            AuditDecision::OutsideFragment(OutsideFragmentReason::OverlappingRewrite)
        ));
    }
}
