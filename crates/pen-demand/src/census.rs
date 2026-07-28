use crate::{
    FamilyId, FiniteDemandDomain, InstanceId, LibrarySeeds, OpaqueWindow, RegisteredFamily,
    RelativeCensus, RelativeCensusOutcome, StructuralSupport, UnknownReason,
};
use pen_kernel::{Digest, GlobalId, Kernel, KernelError, OpenJudgment, Term, VerifiedSignature};
use std::collections::{BTreeMap, BTreeSet};

pub fn domain_digest(domain: &FiniteDemandDomain) -> Digest {
    Digest::of_canonical("pen-demand/finite-domain/v1", domain)
}

pub fn window_digest(window: &OpaqueWindow) -> Digest {
    Digest::of_canonical("pen-demand/opaque-window/v1", window)
}

pub fn library_digest(library: &LibrarySeeds) -> Digest {
    Digest::of_canonical("pen-demand/library-seeds/v1", library)
}

/// Compute the least fixed point inside the exact supplied finite domain.
///
/// The operation limit counts deterministic rule inspections. The complete
/// input is structurally preflighted under one aggregate node/depth budget
/// derived from `kernel`, all motives share one kernel verification budget,
/// and support extraction has one aggregate traversal budget.
pub fn compute_relative_census(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    domain: &FiniteDemandDomain,
    window: &OpaqueWindow,
    library: &LibrarySeeds,
    operation_limit: u64,
) -> RelativeCensusOutcome<RelativeCensus> {
    let mut preflight = RelativeInputBudget::for_kernel(kernel);
    if let Err(reason) = preflight_relative_inputs(&mut preflight, domain, window, library) {
        return RelativeCensusOutcome::Unknown(reason);
    }
    compute_relative_census_preflighted(kernel, signature, domain, window, library, operation_limit)
}

pub(crate) fn compute_relative_census_preflighted(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    domain: &FiniteDemandDomain,
    window: &OpaqueWindow,
    library: &LibrarySeeds,
    operation_limit: u64,
) -> RelativeCensusOutcome<RelativeCensus> {
    let validated = match validate_inputs(kernel, signature, domain, window, library) {
        Ok(validated) => validated,
        Err(reason) => return RelativeCensusOutcome::Unknown(reason),
    };

    // Extraction is the structurally supported subset of the pre-registered
    // instance carrier. It does not manufacture or specialize instances.
    let mut support_budget = RelativeInputBudget::for_kernel(kernel);
    let support = match derive_support(
        signature,
        domain,
        window,
        &validated.families,
        &mut support_budget,
    ) {
        Ok(support) => support,
        Err(reason) => return RelativeCensusOutcome::Unknown(reason),
    };
    let active = support
        .iter()
        .filter(|support| support.window_hits.into_iter().any(|hit| hit))
        .map(|support| support.instance_id.clone())
        .collect::<Vec<_>>();

    // Derivability is a separate monotone closure from the explicit library
    // seeds under the finite registered rules.
    let mut budget = OperationBudget::new(operation_limit);
    let mut reached = library
        .instance_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut layers = vec![reached.iter().cloned().collect::<Vec<_>>()];

    if reached.len() != validated.instances.len() {
        loop {
            let mut next = BTreeSet::new();
            for rule in &domain.rules {
                if budget.inspect().is_err() {
                    return RelativeCensusOutcome::Unknown(UnknownReason::ResourceExhausted);
                }
                if !reached.contains(&rule.conclusion)
                    && rule
                        .premises
                        .iter()
                        .all(|premise| reached.contains(premise))
                {
                    next.insert(rule.conclusion.clone());
                }
            }
            if next.is_empty() {
                break;
            }
            reached.extend(next.iter().cloned());
            layers.push(next.into_iter().collect());
            if reached.len() == validated.instances.len() {
                break;
            }
        }
    }

    let unreached = active
        .iter()
        .filter(|instance| !reached.contains(*instance))
        .cloned()
        .collect();

    RelativeCensusOutcome::CompleteRelative(RelativeCensus {
        support,
        active,
        reached: reached.into_iter().collect(),
        unreached,
        layers,
    })
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RelativeInputBudget {
    remaining_nodes: u64,
    max_depth: u16,
}

impl RelativeInputBudget {
    pub(crate) fn for_kernel(kernel: &Kernel) -> Self {
        Self {
            remaining_nodes: u64::from(kernel.limits().max_operations),
            max_depth: kernel.limits().max_depth,
        }
    }

    pub(crate) fn charge(&mut self) -> Result<(), UnknownReason> {
        self.remaining_nodes = self
            .remaining_nodes
            .checked_sub(1)
            .ok_or(UnknownReason::ResourceExhausted)?;
        Ok(())
    }

    pub(crate) fn charge_len(&mut self, len: usize) -> Result<(), UnknownReason> {
        let count = u64::try_from(len).map_err(|_| UnknownReason::ResourceExhausted)?;
        self.remaining_nodes = self
            .remaining_nodes
            .checked_sub(count)
            .ok_or(UnknownReason::ResourceExhausted)?;
        Ok(())
    }

    fn charge_judgment(
        &mut self,
        judgment: &OpenJudgment,
        depth: u16,
    ) -> Result<(), UnknownReason> {
        self.enter(depth)?;
        let child_depth = self.child_depth(depth)?;
        self.charge_len(judgment.context().0.len())?;
        for ty in &judgment.context().0 {
            self.charge_term(ty, child_depth)?;
        }
        match judgment {
            OpenJudgment::TypeFormation { term, .. } => self.charge_term(term, child_depth),
            OpenJudgment::HasType { term, ty, .. } => {
                self.charge_term(term, child_depth)?;
                self.charge_term(ty, child_depth)
            }
            OpenJudgment::DefinitionallyEqual {
                left, right, ty, ..
            } => {
                self.charge_term(left, child_depth)?;
                self.charge_term(right, child_depth)?;
                self.charge_term(ty, child_depth)
            }
        }
    }

    fn charge_term(&mut self, term: &Term, depth: u16) -> Result<(), UnknownReason> {
        self.enter(depth)?;
        match term {
            Term::Sort { .. }
            | Term::Var { .. }
            | Term::Global { .. }
            | Term::UnitType
            | Term::Unit => Ok(()),
            Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
                let child_depth = self.child_depth(depth)?;
                self.charge_term(parameter, child_depth)?;
                self.charge_term(body, child_depth)
            }
            Term::Lambda {
                parameter_type,
                body,
            } => {
                let child_depth = self.child_depth(depth)?;
                self.charge_term(parameter_type, child_depth)?;
                self.charge_term(body, child_depth)
            }
            Term::Apply { function, argument } => {
                let child_depth = self.child_depth(depth)?;
                self.charge_term(function, child_depth)?;
                self.charge_term(argument, child_depth)
            }
            Term::Pair {
                sigma_type,
                first,
                second,
            } => {
                let child_depth = self.child_depth(depth)?;
                self.charge_term(sigma_type, child_depth)?;
                self.charge_term(first, child_depth)?;
                self.charge_term(second, child_depth)
            }
            Term::First { pair } | Term::Second { pair } => {
                self.charge_term(pair, self.child_depth(depth)?)
            }
        }
    }

    fn enter(&mut self, depth: u16) -> Result<(), UnknownReason> {
        if depth > self.max_depth {
            return Err(UnknownReason::ResourceExhausted);
        }
        self.charge()
    }

    fn child_depth(&self, depth: u16) -> Result<u16, UnknownReason> {
        depth
            .checked_add(1)
            .filter(|child| *child <= self.max_depth)
            .ok_or(UnknownReason::ResourceExhausted)
    }
}

pub(crate) fn preflight_relative_inputs(
    budget: &mut RelativeInputBudget,
    domain: &FiniteDemandDomain,
    window: &OpaqueWindow,
    library: &LibrarySeeds,
) -> Result<(), UnknownReason> {
    budget.charge()?;
    budget.charge_len(window.entries.len())?;

    budget.charge()?;
    budget.charge_len(domain.families.len())?;
    for family in &domain.families {
        budget.charge_judgment(&family.motive, 0)?;
    }

    budget.charge_len(domain.instances.len())?;
    for instance in &domain.instances {
        budget.charge_judgment(&instance.motive, 0)?;
    }

    budget.charge_len(domain.rules.len())?;
    for rule in &domain.rules {
        budget.charge()?;
        budget.charge_len(rule.premises.len())?;
    }

    budget.charge()?;
    budget.charge_len(library.instance_ids.len())
}

struct ValidatedInputs<'a> {
    families: BTreeMap<FamilyId, &'a RegisteredFamily>,
    instances: BTreeSet<InstanceId>,
}

fn validate_inputs<'a>(
    kernel: &Kernel,
    signature: &VerifiedSignature,
    domain: &'a FiniteDemandDomain,
    window: &OpaqueWindow,
    library: &LibrarySeeds,
) -> Result<ValidatedInputs<'a>, UnknownReason> {
    if !strictly_sorted_by(&domain.families, |family| &family.id) {
        return Err(UnknownReason::Unsupported);
    }
    if domain
        .families
        .iter()
        .any(|family| !matches!(family.motive, OpenJudgment::TypeFormation { .. }))
        || domain
            .instances
            .iter()
            .any(|instance| !matches!(instance.motive, OpenJudgment::TypeFormation { .. }))
    {
        return Err(UnknownReason::Unsupported);
    }
    let mut motives = Vec::with_capacity(
        domain
            .families
            .len()
            .checked_add(domain.instances.len())
            .ok_or(UnknownReason::ResourceExhausted)?,
    );
    motives.extend(domain.families.iter().map(|family| &family.motive));
    motives.extend(domain.instances.iter().map(|instance| &instance.motive));
    let normalized_motives = kernel
        .verify_open_judgments(signature, &motives)
        .map_err(kernel_reason)?;
    let (normalized_families, normalized_instances) =
        normalized_motives.split_at(domain.families.len());

    if window.entries[0] == window.entries[1]
        || window.entries.iter().any(|entry| {
            !signature
                .declarations()
                .iter()
                .any(|declaration| declaration.id == *entry)
        })
    {
        return Err(UnknownReason::Unsupported);
    }

    let mut families = BTreeMap::new();
    for (family, normalized) in domain.families.iter().zip(normalized_families) {
        if normalized != &family.motive
            || family.id != RegisteredFamily::canonical_id(&family.motive)
        {
            return Err(UnknownReason::Unsupported);
        }
        if families.insert(family.id.clone(), family).is_some() {
            return Err(UnknownReason::Unsupported);
        }
    }

    if !strictly_sorted_by(&domain.instances, |instance| &instance.id) {
        return Err(UnknownReason::Unsupported);
    }
    let mut instances = BTreeSet::new();
    for (instance, normalized) in domain.instances.iter().zip(normalized_instances) {
        if !families.contains_key(&instance.family_id)
            || normalized != &instance.motive
            || instance.id
                != crate::RegisteredInstance::canonical_id(&instance.family_id, &instance.motive)
        {
            return Err(UnknownReason::Unsupported);
        }
        if !instances.insert(instance.id.clone()) {
            return Err(UnknownReason::Unsupported);
        }
    }

    if !strictly_sorted(&domain.rules) {
        return Err(UnknownReason::Unsupported);
    }
    for rule in &domain.rules {
        if !strictly_sorted(&rule.premises)
            || !instances.contains(&rule.conclusion)
            || rule
                .premises
                .iter()
                .any(|premise| !instances.contains(premise))
        {
            return Err(UnknownReason::Unsupported);
        }
    }

    if !strictly_sorted(&library.instance_ids)
        || library
            .instance_ids
            .iter()
            .any(|seed| !instances.contains(seed))
    {
        return Err(UnknownReason::Unsupported);
    }

    Ok(ValidatedInputs {
        families,
        instances,
    })
}

fn kernel_reason(error: KernelError) -> UnknownReason {
    match error {
        KernelError::ResourceExhausted(_) => UnknownReason::ResourceExhausted,
        _ => UnknownReason::Unsupported,
    }
}

fn strictly_sorted<T: Ord>(values: &[T]) -> bool {
    values.windows(2).all(|pair| pair[0] < pair[1])
}

fn strictly_sorted_by<T, K: Ord>(values: &[T], key: impl Fn(&T) -> &K) -> bool {
    values.windows(2).all(|pair| key(&pair[0]) < key(&pair[1]))
}

fn derive_support(
    signature: &VerifiedSignature,
    domain: &FiniteDemandDomain,
    window: &OpaqueWindow,
    families: &BTreeMap<FamilyId, &RegisteredFamily>,
    budget: &mut RelativeInputBudget,
) -> Result<Vec<StructuralSupport>, UnknownReason> {
    budget.charge_len(domain.instances.len())?;
    let mut support = Vec::with_capacity(domain.instances.len());
    for instance in &domain.instances {
        budget.charge()?;
        let Some(family) = families.get(&instance.family_id) else {
            return Err(UnknownReason::Unsupported);
        };
        let mut direct = BTreeSet::new();
        collect_judgment_globals(&family.motive, &mut direct, budget)?;
        collect_judgment_globals(&instance.motive, &mut direct, budget)?;
        let globals = transitive_global_support(signature, direct, budget)?;
        let window_hits = [
            globals.contains(&window.entries[0]),
            globals.contains(&window.entries[1]),
        ];
        support.push(StructuralSupport {
            instance_id: instance.id.clone(),
            globals: globals.into_iter().collect(),
            window_hits,
        });
    }
    Ok(support)
}

fn collect_judgment_globals(
    judgment: &OpenJudgment,
    output: &mut BTreeSet<GlobalId>,
    budget: &mut RelativeInputBudget,
) -> Result<(), UnknownReason> {
    budget.charge()?;
    match judgment {
        OpenJudgment::TypeFormation { context, term } => {
            budget.charge_len(context.0.len())?;
            for ty in &context.0 {
                collect_term_globals(ty, output, budget)?;
            }
            collect_term_globals(term, output, budget)
        }
        OpenJudgment::HasType {
            context, term, ty, ..
        } => {
            budget.charge_len(context.0.len())?;
            for context_ty in &context.0 {
                collect_term_globals(context_ty, output, budget)?;
            }
            collect_term_globals(term, output, budget)?;
            collect_term_globals(ty, output, budget)
        }
        OpenJudgment::DefinitionallyEqual {
            context,
            left,
            right,
            ty,
        } => {
            budget.charge_len(context.0.len())?;
            for context_ty in &context.0 {
                collect_term_globals(context_ty, output, budget)?;
            }
            collect_term_globals(left, output, budget)?;
            collect_term_globals(right, output, budget)?;
            collect_term_globals(ty, output, budget)
        }
    }
}

fn collect_term_globals(
    term: &Term,
    output: &mut BTreeSet<GlobalId>,
    budget: &mut RelativeInputBudget,
) -> Result<(), UnknownReason> {
    budget.charge()?;
    match term {
        Term::Global { id } => {
            output.insert(id.clone());
            Ok(())
        }
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            collect_term_globals(parameter, output, budget)?;
            collect_term_globals(body, output, budget)
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            collect_term_globals(parameter_type, output, budget)?;
            collect_term_globals(body, output, budget)
        }
        Term::Apply { function, argument } => {
            collect_term_globals(function, output, budget)?;
            collect_term_globals(argument, output, budget)
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            collect_term_globals(sigma_type, output, budget)?;
            collect_term_globals(first, output, budget)?;
            collect_term_globals(second, output, budget)
        }
        Term::First { pair } | Term::Second { pair } => collect_term_globals(pair, output, budget),
        Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => Ok(()),
    }
}

fn transitive_global_support(
    signature: &VerifiedSignature,
    direct: BTreeSet<GlobalId>,
    budget: &mut RelativeInputBudget,
) -> Result<BTreeSet<GlobalId>, UnknownReason> {
    budget.charge_len(signature.declarations().len())?;
    let declarations = signature
        .declarations()
        .iter()
        .map(|declaration| (&declaration.id, declaration))
        .collect::<BTreeMap<_, _>>();
    let mut support = BTreeSet::new();
    let mut pending = direct.into_iter().collect::<Vec<_>>();
    while let Some(id) = pending.pop() {
        budget.charge()?;
        if !support.insert(id.clone()) {
            continue;
        }
        if let Some(declaration) = declarations.get(&id) {
            let mut referenced = BTreeSet::new();
            collect_term_globals(&declaration.ty, &mut referenced, budget)?;
            if let Some(body) = &declaration.body {
                collect_term_globals(body, &mut referenced, budget)?;
            }
            pending.extend(referenced);
        }
    }
    Ok(support)
}

#[derive(Clone, Copy, Debug)]
struct OperationBudget {
    remaining: u64,
}

impl OperationBudget {
    fn new(limit: u64) -> Self {
        Self { remaining: limit }
    }

    fn inspect(&mut self) -> Result<(), ()> {
        self.remaining = self.remaining.checked_sub(1).ok_or(())?;
        Ok(())
    }
}
