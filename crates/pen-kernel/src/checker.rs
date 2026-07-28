use crate::dependency_graph::reviewed_production_dependency_graph;
use crate::{Declaration, DependentContext, Digest, OpenJudgment, Term, UncheckedSignature};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use thiserror::Error;

/// Hard ceiling for all recursive verifier passes.
pub const MAX_SAFE_RECURSION_DEPTH: u16 = 256;

#[derive(Clone, Copy, Debug, Deserialize, Eq, JsonSchema, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct KernelLimits {
    pub max_operations: u32,
    pub max_depth: u16,
    pub normalization_fuel: u32,
}

impl Default for KernelLimits {
    fn default() -> Self {
        Self {
            max_operations: 100_000,
            max_depth: 256,
            normalization_fuel: 50_000,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ResourceKind {
    #[error("operation budget")]
    Operations,
    #[error("term depth")]
    Depth,
    #[error("normalization fuel")]
    Normalization,
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum KernelError {
    #[error("resource exhausted: {0}")]
    ResourceExhausted(ResourceKind),
    #[error("kernel limits are zero or exceed the safe recursion ceiling")]
    InvalidLimits,
    #[error("universe level overflow")]
    UniverseOverflow,
    #[error("variable index is out of scope")]
    UnboundVariable,
    #[error("substitution would create an invalid variable")]
    InvalidSubstitution,
    #[error("substitution arity does not match the dependent context")]
    SubstitutionArity,
    #[error("unknown global identifier")]
    UnknownGlobal,
    #[error("duplicate global identifier")]
    DuplicateGlobal,
    #[error("a type was expected")]
    ExpectedType,
    #[error("a dependent function type was expected")]
    ExpectedFunction,
    #[error("a dependent pair type was expected")]
    ExpectedPair,
    #[error("terms have different types or normal forms")]
    TypeMismatch,
    #[error("certificate claim has the wrong judgment form")]
    WrongJudgmentForm,
}

#[derive(Clone, Debug)]
pub struct VerifiedSignature {
    declarations: Vec<Declaration>,
    digest: Digest,
}

impl VerifiedSignature {
    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub fn declarations(&self) -> &[Declaration] {
        &self.declarations
    }

    pub fn normalized_wire(&self) -> UncheckedSignature {
        UncheckedSignature {
            declarations: self.declarations.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VerifiedContext {
    entries: Vec<Term>,
    digest: Digest,
}

impl VerifiedContext {
    pub fn entries(&self) -> &[Term] {
        &self.entries
    }

    pub fn digest(&self) -> &Digest {
        &self.digest
    }

    pub fn normalized_wire(&self) -> DependentContext {
        DependentContext(self.entries.clone())
    }
}

#[derive(Clone, Debug)]
pub struct Kernel {
    limits: KernelLimits,
}

impl Kernel {
    pub fn new(limits: KernelLimits) -> Result<Self, KernelError> {
        if limits.max_operations == 0
            || limits.max_depth == 0
            || limits.max_depth > MAX_SAFE_RECURSION_DEPTH
            || limits.normalization_fuel == 0
        {
            return Err(KernelError::InvalidLimits);
        }
        Ok(Self { limits })
    }

    pub fn limits(&self) -> KernelLimits {
        self.limits
    }

    /// Reproducible digest of the trusted kernel crate sources, its
    /// self-contained Cargo/toolchain inputs, and the canonical resolved
    /// production dependency graph available at build time.
    ///
    /// This is source provenance, not a binary or build-environment
    /// attestation. The workspace manifest is deliberately excluded: its
    /// unrelated membership list and aggregate lockfile change in the
    /// oracle-free isolation build. A canonical filter over the active lock
    /// must exactly match the reviewed kernel-only dependency snapshot.
    pub fn kernel_protocol_digest(&self) -> Digest {
        let dependency_graph = reviewed_production_dependency_graph();
        Digest::of_domain_chunks(
            "kernel-trusted-source-v2",
            &[
                include_bytes!("lib.rs"),
                include_bytes!("syntax.rs"),
                include_bytes!("checker.rs"),
                include_bytes!("certificate.rs"),
                include_bytes!("dependency_graph.rs"),
                include_bytes!("digest.rs"),
                include_bytes!("../Cargo.toml"),
                dependency_graph.as_slice(),
                include_bytes!("../../../rust-toolchain.toml"),
                include_bytes!("../../../.cargo/config.toml"),
            ],
        )
    }

    /// Reproducible source digest for the syntax/checker normalization slice,
    /// the crate-resolved Cargo/toolchain inputs, and the canonical resolved
    /// production dependency graph. This is not a binary or
    /// build-environment attestation.
    pub fn normalizer_protocol_digest(&self) -> Digest {
        let dependency_graph = reviewed_production_dependency_graph();
        Digest::of_domain_chunks(
            "normalizer-trusted-source-v2",
            &[
                include_bytes!("lib.rs"),
                include_bytes!("syntax.rs"),
                include_bytes!("checker.rs"),
                include_bytes!("dependency_graph.rs"),
                include_bytes!("../Cargo.toml"),
                dependency_graph.as_slice(),
                include_bytes!("../../../rust-toolchain.toml"),
                include_bytes!("../../../.cargo/config.toml"),
            ],
        )
    }

    pub fn verify_signature(
        &self,
        input: &UncheckedSignature,
    ) -> Result<VerifiedSignature, KernelError> {
        if input.declarations.len() > self.limits.max_operations as usize {
            return Err(KernelError::ResourceExhausted(ResourceKind::Operations));
        }
        let mut budget = Budget::new(self.limits);
        self.verify_signature_with_budget(input, &mut budget)
    }

    fn verify_signature_with_budget(
        &self,
        input: &UncheckedSignature,
        budget: &mut Budget,
    ) -> Result<VerifiedSignature, KernelError> {
        let mut declarations = Vec::with_capacity(input.declarations.len());
        let mut identifiers = BTreeSet::new();
        for declaration in &input.declarations {
            budget.enter(0)?;
            if !identifiers.insert(declaration.id.clone()) {
                return Err(KernelError::DuplicateGlobal);
            }
            let inferred = infer(&declarations, &[], &declaration.ty, budget, 0)?;
            expect_universe(&declarations, &[], &inferred, budget, 0)?;
            let ty = normalize(&declarations, &declaration.ty, budget, 0)?;
            let body = declaration
                .body
                .as_ref()
                .map(|body| {
                    check(&declarations, &[], body, &ty, budget, 0)?;
                    normalize(&declarations, body, budget, 0)
                })
                .transpose()?;
            declarations.push(Declaration {
                id: declaration.id.clone(),
                ty,
                body,
            });
        }
        let wire = UncheckedSignature {
            declarations: declarations.clone(),
        };
        Ok(VerifiedSignature {
            digest: Digest::of_canonical("normalized-signature-v1", &wire),
            declarations,
        })
    }

    pub fn verify_context(
        &self,
        signature: &VerifiedSignature,
        input: &DependentContext,
    ) -> Result<VerifiedContext, KernelError> {
        self.validate_verified_signature(signature)?;
        let mut budget = Budget::new(self.limits);
        verify_context_with_budget(signature, input, &mut budget)
    }

    pub fn verify_open_judgment(
        &self,
        signature: &VerifiedSignature,
        judgment: &OpenJudgment,
    ) -> Result<OpenJudgment, KernelError> {
        self.validate_verified_signature(signature)?;
        let mut budget = Budget::new(self.limits);
        verify_open_judgment_with_budget(signature, judgment, &mut budget)
    }

    /// Verify several judgments under one aggregate kernel budget.
    ///
    /// This is intended for callers that must not accidentally grant every
    /// item in an untrusted collection a fresh full operation allowance.
    pub fn verify_open_judgments(
        &self,
        signature: &VerifiedSignature,
        judgments: &[&OpenJudgment],
    ) -> Result<Vec<OpenJudgment>, KernelError> {
        if judgments.len() > self.limits.max_operations as usize {
            return Err(KernelError::ResourceExhausted(ResourceKind::Operations));
        }
        let mut budget = Budget::new(self.limits);
        charge_declarations(signature.declarations(), &mut budget, 0)?;
        let mut normalized = Vec::with_capacity(judgments.len());
        for judgment in judgments {
            normalized.push(verify_open_judgment_with_budget(
                signature,
                judgment,
                &mut budget,
            )?);
        }
        Ok(normalized)
    }

    /// Verify the exact syntactic extension represented by `extension`.
    ///
    /// This does not authorize integration into a history and does not claim
    /// the categorical universal property required by the full calculus.
    pub fn verify_extension(
        &self,
        base: &VerifiedSignature,
        extension: &UncheckedSignature,
    ) -> Result<VerifiedSignature, KernelError> {
        self.validate_verified_signature(base)?;
        let combined_len = base
            .declarations()
            .len()
            .checked_add(extension.declarations.len())
            .ok_or(KernelError::ResourceExhausted(ResourceKind::Operations))?;
        if combined_len > self.limits.max_operations as usize {
            return Err(KernelError::ResourceExhausted(ResourceKind::Operations));
        }
        self.validate_certificate_signatures(&[extension])?;
        let mut declarations = base.declarations().to_vec();
        declarations.extend(extension.declarations.iter().cloned());
        self.verify_signature(&UncheckedSignature { declarations })
    }

    fn validate_verified_signature(
        &self,
        signature: &VerifiedSignature,
    ) -> Result<(), KernelError> {
        let mut budget = Budget::new(self.limits);
        charge_declarations(signature.declarations(), &mut budget, 0)
    }

    pub(crate) fn validate_certificate_judgments(
        &self,
        judgments: &[&OpenJudgment],
    ) -> Result<(), KernelError> {
        let mut budget = Budget::new(self.limits);
        for judgment in judgments {
            charge_open_judgment(judgment, &mut budget, 0)?;
        }
        Ok(())
    }

    pub(crate) fn validate_certificate_signatures(
        &self,
        signatures: &[&UncheckedSignature],
    ) -> Result<(), KernelError> {
        let mut budget = Budget::new(self.limits);
        for signature in signatures {
            charge_signature(signature, &mut budget, 0)?;
        }
        Ok(())
    }

    pub(crate) fn validate_certificate_terms(&self, terms: &[Term]) -> Result<(), KernelError> {
        let mut budget = Budget::new(self.limits);
        charge_terms(terms, &mut budget, 0)
    }

    /// Compute and recheck one complete closed specialization.
    ///
    /// The returned wire judgment is a normalized replay result, not a proof
    /// capability and not a theorem that every assignment specializes. A
    /// caller that needs proof authority must still submit the result through
    /// `verify_closed_specialization_certificate`.
    pub fn normalize_closed_specialization(
        &self,
        signature: &VerifiedSignature,
        open: &OpenJudgment,
        assignments: &[Term],
    ) -> Result<OpenJudgment, KernelError> {
        self.replay_closed_specialization(signature, open, assignments)
    }

    pub(crate) fn replay_closed_specialization(
        &self,
        signature: &VerifiedSignature,
        open: &OpenJudgment,
        assignments: &[Term],
    ) -> Result<OpenJudgment, KernelError> {
        self.validate_verified_signature(signature)?;
        self.validate_certificate_judgments(&[open])?;
        self.validate_certificate_terms(assignments)?;

        let normalized_open = self.verify_open_judgment(signature, open)?;
        let context = &normalized_open.context().0;
        if context.len() != assignments.len() {
            return Err(KernelError::SubstitutionArity);
        }

        let mut budget = Budget::new(self.limits);
        let mut normalized_assignments = Vec::with_capacity(assignments.len());
        for (declared_type, assignment) in context.iter().zip(assignments) {
            let specialized_type =
                instantiate_closed(declared_type, &normalized_assignments, &mut budget, 0)?;
            check(
                signature.declarations(),
                &[],
                assignment,
                &specialized_type,
                &mut budget,
                0,
            )?;
            normalized_assignments.push(normalize(
                signature.declarations(),
                assignment,
                &mut budget,
                0,
            )?);
        }

        let specialized =
            instantiate_open_judgment(&normalized_open, &normalized_assignments, &mut budget)?;
        self.verify_open_judgment(signature, &specialized)
    }
}

fn verify_open_judgment_with_budget(
    signature: &VerifiedSignature,
    judgment: &OpenJudgment,
    budget: &mut Budget,
) -> Result<OpenJudgment, KernelError> {
    let context = verify_context_with_budget(signature, judgment.context(), budget)?;
    let entries = context.entries();
    match judgment {
        OpenJudgment::TypeFormation { term, .. } => {
            let inferred = infer(signature.declarations(), entries, term, budget, 0)?;
            expect_universe(signature.declarations(), entries, &inferred, budget, 0)?;
            Ok(OpenJudgment::TypeFormation {
                context: context.normalized_wire(),
                term: normalize(signature.declarations(), term, budget, 0)?,
            })
        }
        OpenJudgment::HasType { term, ty, .. } => {
            let ty_sort = infer(signature.declarations(), entries, ty, budget, 0)?;
            expect_universe(signature.declarations(), entries, &ty_sort, budget, 0)?;
            check(signature.declarations(), entries, term, ty, budget, 0)?;
            Ok(OpenJudgment::HasType {
                context: context.normalized_wire(),
                term: normalize(signature.declarations(), term, budget, 0)?,
                ty: normalize(signature.declarations(), ty, budget, 0)?,
            })
        }
        OpenJudgment::DefinitionallyEqual {
            left, right, ty, ..
        } => {
            let ty_sort = infer(signature.declarations(), entries, ty, budget, 0)?;
            expect_universe(signature.declarations(), entries, &ty_sort, budget, 0)?;
            check(signature.declarations(), entries, left, ty, budget, 0)?;
            check(signature.declarations(), entries, right, ty, budget, 0)?;
            let normal_left = normalize(signature.declarations(), left, budget, 0)?;
            let normal_right = normalize(signature.declarations(), right, budget, 0)?;
            if normal_left != normal_right {
                return Err(KernelError::TypeMismatch);
            }
            Ok(OpenJudgment::DefinitionallyEqual {
                context: context.normalized_wire(),
                left: normal_left.clone(),
                right: normal_left,
                ty: normalize(signature.declarations(), ty, budget, 0)?,
            })
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Budget {
    operations_left: u32,
    depth_limit: u16,
    rewrites_left: u32,
}

impl Budget {
    fn new(limits: KernelLimits) -> Self {
        Self {
            operations_left: limits.max_operations,
            depth_limit: limits.max_depth,
            rewrites_left: limits.normalization_fuel,
        }
    }

    fn enter(&mut self, depth: u16) -> Result<(), KernelError> {
        if depth > self.depth_limit {
            return Err(KernelError::ResourceExhausted(ResourceKind::Depth));
        }
        self.operations_left = self
            .operations_left
            .checked_sub(1)
            .ok_or(KernelError::ResourceExhausted(ResourceKind::Operations))?;
        Ok(())
    }

    fn rewrite(&mut self) -> Result<(), KernelError> {
        self.rewrites_left = self
            .rewrites_left
            .checked_sub(1)
            .ok_or(KernelError::ResourceExhausted(ResourceKind::Normalization))?;
        Ok(())
    }
}

fn next_depth(depth: u16) -> Result<u16, KernelError> {
    depth
        .checked_add(1)
        .ok_or(KernelError::ResourceExhausted(ResourceKind::Depth))
}

fn verify_context_with_budget(
    signature: &VerifiedSignature,
    input: &DependentContext,
    budget: &mut Budget,
) -> Result<VerifiedContext, KernelError> {
    if input.0.len() > budget.operations_left as usize {
        return Err(KernelError::ResourceExhausted(ResourceKind::Operations));
    }
    let mut entries = Vec::with_capacity(input.0.len());
    for ty in &input.0 {
        let inferred = infer(signature.declarations(), &entries, ty, budget, 0)?;
        expect_universe(signature.declarations(), &entries, &inferred, budget, 0)?;
        entries.push(normalize(signature.declarations(), ty, budget, 0)?);
    }
    let normalized = DependentContext(entries.clone());
    Ok(VerifiedContext {
        digest: Digest::of_canonical("dependent-context-v1", &normalized),
        entries,
    })
}

fn infer(
    signature: &[Declaration],
    context: &[Term],
    term: &Term,
    budget: &mut Budget,
    depth: u16,
) -> Result<Term, KernelError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    match term {
        Term::Sort { level } => Ok(Term::Sort {
            level: level.checked_add(1).ok_or(KernelError::UniverseOverflow)?,
        }),
        Term::Var { index } => {
            let distance = index.checked_add(1).ok_or(KernelError::UnboundVariable)?;
            let distance_usize =
                usize::try_from(distance).map_err(|_| KernelError::UnboundVariable)?;
            let position = context
                .len()
                .checked_sub(distance_usize)
                .ok_or(KernelError::UnboundVariable)?;
            shift(
                &context[position],
                i64::from(distance),
                0,
                budget,
                child_depth,
            )
        }
        Term::Global { id } => {
            let declaration = lookup_global(signature, id, budget, child_depth)?;
            charge_term(&declaration.ty, budget, child_depth)?;
            Ok(declaration.ty.clone())
        }
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            let parameter_sort = infer(signature, context, parameter, budget, child_depth)?;
            let parameter_level =
                expect_universe(signature, context, &parameter_sort, budget, child_depth)?;
            let normal_parameter = normalize(signature, parameter, budget, child_depth)?;
            charge_terms(context, budget, child_depth)?;
            let mut extended = context.to_vec();
            extended.push(normal_parameter);
            let body_sort = infer(signature, &extended, body, budget, child_depth)?;
            let body_level =
                expect_universe(signature, &extended, &body_sort, budget, child_depth)?;
            Ok(Term::Sort {
                level: parameter_level.max(body_level),
            })
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            let parameter_sort = infer(signature, context, parameter_type, budget, child_depth)?;
            expect_universe(signature, context, &parameter_sort, budget, child_depth)?;
            let normal_parameter = normalize(signature, parameter_type, budget, child_depth)?;
            charge_terms(context, budget, child_depth)?;
            let mut extended = context.to_vec();
            extended.push(normal_parameter.clone());
            let body_type = infer(signature, &extended, body, budget, child_depth)?;
            Ok(Term::Pi {
                parameter: Box::new(normal_parameter),
                body: Box::new(body_type),
            })
        }
        Term::Apply { function, argument } => {
            let function_type = infer(signature, context, function, budget, child_depth)?;
            let normal_function_type = normalize(signature, &function_type, budget, child_depth)?;
            let Term::Pi { parameter, body } = normal_function_type else {
                return Err(KernelError::ExpectedFunction);
            };
            check(
                signature,
                context,
                argument,
                &parameter,
                budget,
                child_depth,
            )?;
            subst_top(&body, argument, budget, child_depth)
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            let sigma_sort = infer(signature, context, sigma_type, budget, child_depth)?;
            expect_universe(signature, context, &sigma_sort, budget, child_depth)?;
            let normal_sigma = normalize(signature, sigma_type, budget, child_depth)?;
            let Term::Sigma { parameter, body } = &normal_sigma else {
                return Err(KernelError::ExpectedPair);
            };
            check(signature, context, first, parameter, budget, child_depth)?;
            let second_type = subst_top(body, first, budget, child_depth)?;
            check(
                signature,
                context,
                second,
                &second_type,
                budget,
                child_depth,
            )?;
            Ok(normal_sigma)
        }
        Term::First { pair } => {
            let pair_type = infer(signature, context, pair, budget, child_depth)?;
            let normal_pair_type = normalize(signature, &pair_type, budget, child_depth)?;
            let Term::Sigma { parameter, .. } = normal_pair_type else {
                return Err(KernelError::ExpectedPair);
            };
            Ok(*parameter)
        }
        Term::Second { pair } => {
            let pair_type = infer(signature, context, pair, budget, child_depth)?;
            let normal_pair_type = normalize(signature, &pair_type, budget, child_depth)?;
            let Term::Sigma { body, .. } = normal_pair_type else {
                return Err(KernelError::ExpectedPair);
            };
            subst_top(
                &body,
                &Term::First { pair: pair.clone() },
                budget,
                child_depth,
            )
        }
        Term::UnitType => Ok(Term::Sort { level: 0 }),
        Term::Unit => Ok(Term::UnitType),
    }
}

fn check(
    signature: &[Declaration],
    context: &[Term],
    term: &Term,
    expected: &Term,
    budget: &mut Budget,
    depth: u16,
) -> Result<(), KernelError> {
    let inferred = infer(signature, context, term, budget, depth)?;
    let normal_inferred = normalize(signature, &inferred, budget, depth)?;
    let normal_expected = normalize(signature, expected, budget, depth)?;
    if normal_inferred == normal_expected {
        Ok(())
    } else {
        Err(KernelError::TypeMismatch)
    }
}

fn expect_universe(
    signature: &[Declaration],
    context: &[Term],
    inferred: &Term,
    budget: &mut Budget,
    depth: u16,
) -> Result<u16, KernelError> {
    let _ = context;
    match normalize(signature, inferred, budget, depth)? {
        Term::Sort { level } => Ok(level),
        _ => Err(KernelError::ExpectedType),
    }
}

fn normalize(
    signature: &[Declaration],
    term: &Term,
    budget: &mut Budget,
    depth: u16,
) -> Result<Term, KernelError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    match term {
        Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => Ok(term.clone()),
        Term::Global { id } => {
            let declaration = lookup_global(signature, id, budget, child_depth)?;
            if let Some(body) = &declaration.body {
                budget.rewrite()?;
                normalize(signature, body, budget, child_depth)
            } else {
                Ok(term.clone())
            }
        }
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(normalize(signature, parameter, budget, child_depth)?),
            body: Box::new(normalize(signature, body, budget, child_depth)?),
        }),
        Term::Sigma { parameter, body } => Ok(Term::Sigma {
            parameter: Box::new(normalize(signature, parameter, budget, child_depth)?),
            body: Box::new(normalize(signature, body, budget, child_depth)?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(normalize(signature, parameter_type, budget, child_depth)?),
            body: Box::new(normalize(signature, body, budget, child_depth)?),
        }),
        Term::Apply { function, argument } => {
            let normal_function = normalize(signature, function, budget, child_depth)?;
            let normal_argument = normalize(signature, argument, budget, child_depth)?;
            if let Term::Lambda { body, .. } = normal_function {
                budget.rewrite()?;
                let reduced = subst_top(&body, &normal_argument, budget, child_depth)?;
                normalize(signature, &reduced, budget, child_depth)
            } else {
                Ok(Term::Apply {
                    function: Box::new(normal_function),
                    argument: Box::new(normal_argument),
                })
            }
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Ok(Term::Pair {
            sigma_type: Box::new(normalize(signature, sigma_type, budget, child_depth)?),
            first: Box::new(normalize(signature, first, budget, child_depth)?),
            second: Box::new(normalize(signature, second, budget, child_depth)?),
        }),
        Term::First { pair } => {
            let normal_pair = normalize(signature, pair, budget, child_depth)?;
            if let Term::Pair { first, .. } = normal_pair {
                budget.rewrite()?;
                Ok(*first)
            } else {
                Ok(Term::First {
                    pair: Box::new(normal_pair),
                })
            }
        }
        Term::Second { pair } => {
            let normal_pair = normalize(signature, pair, budget, child_depth)?;
            if let Term::Pair { second, .. } = normal_pair {
                budget.rewrite()?;
                Ok(*second)
            } else {
                Ok(Term::Second {
                    pair: Box::new(normal_pair),
                })
            }
        }
    }
}

fn lookup_global<'a>(
    signature: &'a [Declaration],
    id: &crate::GlobalId,
    budget: &mut Budget,
    depth: u16,
) -> Result<&'a Declaration, KernelError> {
    for declaration in signature {
        budget.enter(depth)?;
        if declaration.id == *id {
            return Ok(declaration);
        }
    }
    Err(KernelError::UnknownGlobal)
}

fn charge_signature(
    signature: &UncheckedSignature,
    budget: &mut Budget,
    depth: u16,
) -> Result<(), KernelError> {
    charge_declarations(&signature.declarations, budget, depth)
}

fn charge_declarations(
    declarations: &[Declaration],
    budget: &mut Budget,
    depth: u16,
) -> Result<(), KernelError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    for declaration in declarations {
        budget.enter(child_depth)?;
        charge_term(&declaration.ty, budget, child_depth)?;
        if let Some(body) = &declaration.body {
            charge_term(body, budget, child_depth)?;
        }
    }
    Ok(())
}

fn charge_open_judgment(
    judgment: &OpenJudgment,
    budget: &mut Budget,
    depth: u16,
) -> Result<(), KernelError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    match judgment {
        OpenJudgment::TypeFormation { context, term } => {
            charge_context(context, budget, child_depth)?;
            charge_term(term, budget, child_depth)
        }
        OpenJudgment::HasType { context, term, ty } => {
            charge_context(context, budget, child_depth)?;
            charge_term(term, budget, child_depth)?;
            charge_term(ty, budget, child_depth)
        }
        OpenJudgment::DefinitionallyEqual {
            context,
            left,
            right,
            ty,
        } => {
            charge_context(context, budget, child_depth)?;
            charge_term(left, budget, child_depth)?;
            charge_term(right, budget, child_depth)?;
            charge_term(ty, budget, child_depth)
        }
    }
}

fn charge_context(
    context: &DependentContext,
    budget: &mut Budget,
    depth: u16,
) -> Result<(), KernelError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    charge_terms(&context.0, budget, child_depth)
}

fn charge_terms(terms: &[Term], budget: &mut Budget, depth: u16) -> Result<(), KernelError> {
    for ty in terms {
        charge_term(ty, budget, depth)?;
    }
    Ok(())
}

fn charge_term(term: &Term, budget: &mut Budget, depth: u16) -> Result<(), KernelError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    match term {
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(()),
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            charge_term(parameter, budget, child_depth)?;
            charge_term(body, budget, child_depth)
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            charge_term(parameter_type, budget, child_depth)?;
            charge_term(body, budget, child_depth)
        }
        Term::Apply { function, argument } => {
            charge_term(function, budget, child_depth)?;
            charge_term(argument, budget, child_depth)
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            charge_term(sigma_type, budget, child_depth)?;
            charge_term(first, budget, child_depth)?;
            charge_term(second, budget, child_depth)
        }
        Term::First { pair } | Term::Second { pair } => charge_term(pair, budget, child_depth),
    }
}

fn instantiate_open_judgment(
    judgment: &OpenJudgment,
    assignments: &[Term],
    budget: &mut Budget,
) -> Result<OpenJudgment, KernelError> {
    let context = DependentContext::default();
    match judgment {
        OpenJudgment::TypeFormation { term, .. } => Ok(OpenJudgment::TypeFormation {
            context,
            term: instantiate_closed(term, assignments, budget, 0)?,
        }),
        OpenJudgment::HasType { term, ty, .. } => Ok(OpenJudgment::HasType {
            context,
            term: instantiate_closed(term, assignments, budget, 0)?,
            ty: instantiate_closed(ty, assignments, budget, 0)?,
        }),
        OpenJudgment::DefinitionallyEqual {
            left, right, ty, ..
        } => Ok(OpenJudgment::DefinitionallyEqual {
            context,
            left: instantiate_closed(left, assignments, budget, 0)?,
            right: instantiate_closed(right, assignments, budget, 0)?,
            ty: instantiate_closed(ty, assignments, budget, 0)?,
        }),
    }
}

fn instantiate_closed(
    term: &Term,
    assignments: &[Term],
    budget: &mut Budget,
    depth: u16,
) -> Result<Term, KernelError> {
    charge_term(term, budget, depth)?;
    let mut specialized = term.clone();
    for assignment in assignments.iter().rev() {
        specialized = subst_top(&specialized, assignment, budget, depth)?;
    }
    Ok(specialized)
}

fn subst_top(
    body: &Term,
    replacement: &Term,
    budget: &mut Budget,
    depth: u16,
) -> Result<Term, KernelError> {
    let lifted = shift(replacement, 1, 0, budget, depth)?;
    let replaced = substitute(body, 0, &lifted, 0, budget, depth)?;
    shift(&replaced, -1, 0, budget, depth)
}

fn substitute(
    term: &Term,
    target: u32,
    replacement: &Term,
    binder_depth: u32,
    budget: &mut Budget,
    depth: u16,
) -> Result<Term, KernelError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    let sought = target
        .checked_add(binder_depth)
        .ok_or(KernelError::InvalidSubstitution)?;
    match term {
        Term::Var { index } if *index == sought => {
            shift(replacement, i64::from(binder_depth), 0, budget, child_depth)
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
                child_depth,
            )?),
            body: Box::new(substitute(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or(KernelError::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Sigma { parameter, body } => Ok(Term::Sigma {
            parameter: Box::new(substitute(
                parameter,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
            body: Box::new(substitute(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or(KernelError::InvalidSubstitution)?,
                budget,
                child_depth,
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
                child_depth,
            )?),
            body: Box::new(substitute(
                body,
                target,
                replacement,
                binder_depth
                    .checked_add(1)
                    .ok_or(KernelError::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(substitute(
                function,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
            argument: Box::new(substitute(
                argument,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
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
                child_depth,
            )?),
            first: Box::new(substitute(
                first,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
            second: Box::new(substitute(
                second,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
        }),
        Term::First { pair } => Ok(Term::First {
            pair: Box::new(substitute(
                pair,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
        }),
        Term::Second { pair } => Ok(Term::Second {
            pair: Box::new(substitute(
                pair,
                target,
                replacement,
                binder_depth,
                budget,
                child_depth,
            )?),
        }),
    }
}

fn shift(
    term: &Term,
    amount: i64,
    cutoff: u32,
    budget: &mut Budget,
    depth: u16,
) -> Result<Term, KernelError> {
    budget.enter(depth)?;
    let child_depth = next_depth(depth)?;
    match term {
        Term::Var { index } if *index >= cutoff => {
            let shifted = i64::from(*index)
                .checked_add(amount)
                .ok_or(KernelError::InvalidSubstitution)?;
            let index = u32::try_from(shifted).map_err(|_| KernelError::InvalidSubstitution)?;
            Ok(Term::Var { index })
        }
        Term::Sort { .. }
        | Term::Var { .. }
        | Term::Global { .. }
        | Term::UnitType
        | Term::Unit => Ok(term.clone()),
        Term::Pi { parameter, body } => Ok(Term::Pi {
            parameter: Box::new(shift(parameter, amount, cutoff, budget, child_depth)?),
            body: Box::new(shift(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(KernelError::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Sigma { parameter, body } => Ok(Term::Sigma {
            parameter: Box::new(shift(parameter, amount, cutoff, budget, child_depth)?),
            body: Box::new(shift(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(KernelError::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Lambda {
            parameter_type,
            body,
        } => Ok(Term::Lambda {
            parameter_type: Box::new(shift(parameter_type, amount, cutoff, budget, child_depth)?),
            body: Box::new(shift(
                body,
                amount,
                cutoff
                    .checked_add(1)
                    .ok_or(KernelError::InvalidSubstitution)?,
                budget,
                child_depth,
            )?),
        }),
        Term::Apply { function, argument } => Ok(Term::Apply {
            function: Box::new(shift(function, amount, cutoff, budget, child_depth)?),
            argument: Box::new(shift(argument, amount, cutoff, budget, child_depth)?),
        }),
        Term::Pair {
            sigma_type,
            first,
            second,
        } => Ok(Term::Pair {
            sigma_type: Box::new(shift(sigma_type, amount, cutoff, budget, child_depth)?),
            first: Box::new(shift(first, amount, cutoff, budget, child_depth)?),
            second: Box::new(shift(second, amount, cutoff, budget, child_depth)?),
        }),
        Term::First { pair } => Ok(Term::First {
            pair: Box::new(shift(pair, amount, cutoff, budget, child_depth)?),
        }),
        Term::Second { pair } => Ok(Term::Second {
            pair: Box::new(shift(pair, amount, cutoff, budget, child_depth)?),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::{Kernel, KernelError, KernelLimits, MAX_SAFE_RECURSION_DEPTH, ResourceKind};
    use crate::{
        Declaration, DependentContext, Digest, GlobalId, OpenJudgment, Term, UncheckedSignature,
    };

    fn id(seed: &[u8]) -> GlobalId {
        GlobalId(Digest::of_bytes(seed))
    }

    fn kernel() -> Kernel {
        Kernel::new(KernelLimits::default()).expect("valid limits")
    }

    #[test]
    fn recursion_limit_cannot_exceed_the_hard_ceiling() {
        assert_eq!(
            Kernel::new(KernelLimits {
                max_operations: 1,
                max_depth: MAX_SAFE_RECURSION_DEPTH + 1,
                normalization_fuel: 1,
            })
            .expect_err("unsafe recursion limit"),
            KernelError::InvalidLimits
        );
    }

    #[test]
    fn dependent_context_is_checked_left_to_right() {
        let signature = kernel()
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let valid = DependentContext(vec![Term::Sort { level: 0 }, Term::Var { index: 0 }]);
        kernel()
            .verify_context(&signature, &valid)
            .expect("second type depends on its predecessor");

        let forward = DependentContext(vec![Term::Var { index: 0 }]);
        assert_eq!(
            kernel()
                .verify_context(&signature, &forward)
                .expect_err("forward dependency"),
            KernelError::UnboundVariable
        );
    }

    #[test]
    fn batch_judgments_share_one_operation_budget() {
        let kernel = Kernel::new(KernelLimits {
            max_operations: 4,
            max_depth: 16,
            normalization_fuel: 16,
        })
        .expect("bounded kernel");
        let signature = kernel
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let judgment = OpenJudgment::TypeFormation {
            context: DependentContext::default(),
            term: Term::UnitType,
        };
        kernel
            .verify_open_judgment(&signature, &judgment)
            .expect("one judgment fits");
        assert_eq!(
            kernel
                .verify_open_judgments(&signature, &[&judgment, &judgment])
                .expect_err("the batch must share its budget"),
            KernelError::ResourceExhausted(ResourceKind::Operations)
        );
    }

    #[test]
    fn beta_reduction_is_capture_safe() {
        let signature = kernel()
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let judgment = OpenJudgment::DefinitionallyEqual {
            context: DependentContext::default(),
            left: Term::Apply {
                function: Box::new(Term::Lambda {
                    parameter_type: Box::new(Term::UnitType),
                    body: Box::new(Term::Var { index: 0 }),
                }),
                argument: Box::new(Term::Unit),
            },
            right: Term::Unit,
            ty: Term::UnitType,
        };
        kernel()
            .verify_open_judgment(&signature, &judgment)
            .expect("beta equality");
    }

    #[test]
    fn nested_dependent_beta_reduction_preserves_binders() {
        let signature = kernel()
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let polymorphic_identity = Term::Lambda {
            parameter_type: Box::new(Term::Sort { level: 0 }),
            body: Box::new(Term::Lambda {
                parameter_type: Box::new(Term::Var { index: 0 }),
                body: Box::new(Term::Var { index: 0 }),
            }),
        };
        let application = Term::Apply {
            function: Box::new(Term::Apply {
                function: Box::new(polymorphic_identity),
                argument: Box::new(Term::UnitType),
            }),
            argument: Box::new(Term::Unit),
        };
        let judgment = OpenJudgment::DefinitionallyEqual {
            context: DependentContext::default(),
            left: application,
            right: Term::Unit,
            ty: Term::UnitType,
        };
        kernel()
            .verify_open_judgment(&signature, &judgment)
            .expect("dependent beta equality");
    }

    #[test]
    fn second_projection_has_the_substituted_dependent_type() {
        let signature = kernel()
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let sigma = Term::Sigma {
            parameter: Box::new(Term::Sort { level: 0 }),
            body: Box::new(Term::Var { index: 0 }),
        };
        let pair = Term::Pair {
            sigma_type: Box::new(sigma),
            first: Box::new(Term::UnitType),
            second: Box::new(Term::Unit),
        };
        let judgment = OpenJudgment::HasType {
            context: DependentContext::default(),
            term: Term::Second {
                pair: Box::new(pair),
            },
            ty: Term::UnitType,
        };
        kernel()
            .verify_open_judgment(&signature, &judgment)
            .expect("dependent projection");
    }

    #[test]
    fn application_of_non_function_is_rejected() {
        let signature = kernel()
            .verify_signature(&UncheckedSignature::default())
            .expect("empty signature");
        let judgment = OpenJudgment::HasType {
            context: DependentContext::default(),
            term: Term::Apply {
                function: Box::new(Term::Unit),
                argument: Box::new(Term::Unit),
            },
            ty: Term::UnitType,
        };
        assert_eq!(
            kernel()
                .verify_open_judgment(&signature, &judgment)
                .expect_err("not a function"),
            KernelError::ExpectedFunction
        );
    }

    #[test]
    fn declarations_are_nonrecursive_and_exactly_typed() {
        let first = id(b"first");
        let good = UncheckedSignature {
            declarations: vec![
                Declaration {
                    id: first.clone(),
                    ty: Term::UnitType,
                    body: Some(Term::Unit),
                },
                Declaration {
                    id: id(b"second"),
                    ty: Term::UnitType,
                    body: Some(Term::Global { id: first }),
                },
            ],
        };
        kernel().verify_signature(&good).expect("ordered signature");

        let duplicate = UncheckedSignature {
            declarations: vec![
                Declaration {
                    id: id(b"same"),
                    ty: Term::UnitType,
                    body: None,
                },
                Declaration {
                    id: id(b"same"),
                    ty: Term::UnitType,
                    body: None,
                },
            ],
        };
        assert_eq!(
            kernel()
                .verify_signature(&duplicate)
                .expect_err("duplicate"),
            KernelError::DuplicateGlobal
        );
    }

    #[test]
    fn exhaustion_is_unknown_to_callers_not_a_negative_theorem() {
        let tiny = Kernel::new(KernelLimits {
            max_operations: 1,
            max_depth: 1,
            normalization_fuel: 1,
        })
        .expect("positive");
        let error = tiny
            .verify_signature(&UncheckedSignature {
                declarations: vec![Declaration {
                    id: id(b"limited"),
                    ty: Term::UnitType,
                    body: None,
                }],
            })
            .expect_err("must exhaust");
        assert!(matches!(
            error,
            KernelError::ResourceExhausted(ResourceKind::Operations)
        ));
    }
}
