use super::family::{
    CANONICAL_DEMAND_FAMILY_SCHEMA_VERSION, CanonicalDemandFamilyV2, ContextMapCode,
    EquationClauseCode, GscRule, OutputClause, OutputRole, PortKey, PortRef, PremiseRef,
    VerifiedCanonicalDemandFamilyV2, verify_compiled_family,
};
use super::inductive::{
    ConstructorCode, ConstructorPortId, VerifiedClosedFormerFrame, code_term_to_kernel,
};
use super::manifest::{GscOutcome, GscUnknownReason, VerifiedGscSemanticManifest};
use pen_kernel::{DependentContext, GlobalId, Kernel, OpenJudgment, Term, VerifiedSignature};
use std::collections::{BTreeMap, BTreeSet};

pub fn compile_use_v1(
    manifest: &VerifiedGscSemanticManifest,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    frame: &VerifiedClosedFormerFrame,
) -> GscOutcome<VerifiedCanonicalDemandFamilyV2> {
    if frame.code().manifest_digest() != manifest.digest() {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }
    if !frame.code().supports_core_compiler() {
        return unsupported_frame(frame);
    }

    let owner = Term::Global {
        id: frame.owner().clone(),
    };
    let motive_type = Term::Pi {
        parameter: Box::new(owner.clone()),
        body: Box::new(Term::Sort {
            level: frame.code().code().universe_level,
        }),
    };
    let mut parameter_types = vec![motive_type];
    for (constructor_index, constructor) in frame.code().code().constructors.iter().enumerate() {
        let prior_methods = constructor_index;
        let Some(method_type) = method_type(frame, constructor_index, constructor, prior_methods)
        else {
            return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
        };
        parameter_types.push(method_type);
    }
    let parameter_context = DependentContext(parameter_types);
    let method_count = frame.code().code().constructors.len();
    let output_motive = Term::Pi {
        parameter: Box::new(owner),
        body: Box::new(Term::Apply {
            function: Box::new(Term::Var {
                index: (method_count + 1) as u32,
            }),
            argument: Box::new(Term::Var { index: 0 }),
        }),
    };
    let role = OutputRole::UsePort {
        former: frame.id().clone(),
    };
    let output = OutputClause::TermPort {
        context: parameter_context.clone(),
        motive: output_motive.clone(),
        role,
    };
    let verification_judgments = vec![OpenJudgment::TypeFormation {
        context: parameter_context.clone(),
        term: output_motive,
    }];
    let family = CanonicalDemandFamilyV2 {
        schema_version: CANONICAL_DEMAND_FAMILY_SCHEMA_VERSION,
        semantic_manifest_digest: manifest.digest().clone(),
        rule: GscRule::Use,
        rank: 1,
        parameter_context,
        premise_refs: Vec::new(),
        output_clauses: vec![output],
        verification_judgments,
        principal_sources: frame.principal_sources().to_vec(),
        birth_support: frame.birth_support().to_vec(),
        fixed_type_support: fixed_type_support(signature, frame),
        parameter_support_projections: (0..=method_count as u32).collect(),
    };
    verify_compiled_family(manifest, kernel, signature, family)
}

pub fn compile_compute_v1(
    manifest: &VerifiedGscSemanticManifest,
    kernel: &Kernel,
    signature: &VerifiedSignature,
    frame: &VerifiedClosedFormerFrame,
    use_family: &VerifiedCanonicalDemandFamilyV2,
    use_port: &PortKey,
    constructor: &ConstructorPortId,
) -> GscOutcome<VerifiedCanonicalDemandFamilyV2> {
    if frame.code().manifest_digest() != manifest.digest()
        || use_family.semantic_manifest_digest() != manifest.digest()
        || use_family.rule() != GscRule::Use
        || use_family.rank() != 1
    {
        return GscOutcome::Unknown(GscUnknownReason::FamilyMismatch);
    }
    if !frame.code().supports_core_compiler() {
        return unsupported_frame(frame);
    }
    let Some(source_port) = use_family.port(use_port) else {
        return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
    };
    let expected_use_role = OutputRole::UsePort {
        former: frame.id().clone(),
    };
    if source_port.role() != &expected_use_role {
        return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
    }
    let OutputClause::TermPort {
        context: source_context,
        motive: source_motive,
        ..
    } = source_port.clause()
    else {
        return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
    };
    if source_context != use_family.parameter_context() {
        return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
    }

    let Some(constructor_index) = frame
        .code()
        .constructor_ids()
        .iter()
        .position(|candidate| candidate == constructor)
    else {
        return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
    };
    let constructor_code = &frame.code().code().constructors[constructor_index];
    let Some(introduction) = frame.introduction(constructor) else {
        return GscOutcome::Unknown(GscUnknownReason::PortMismatch);
    };

    let method_count = frame.code().code().constructors.len();
    let Some(local) = computation_arguments(frame.owner(), constructor_code) else {
        return GscOutcome::Unknown(GscUnknownReason::UnsupportedCode);
    };
    let local_count = local.binders.len();
    let intro_application = apply_arguments(
        Term::Global {
            id: introduction.clone(),
        },
        &local.argument_indices,
    );
    let motive = Term::Apply {
        function: Box::new(Term::Var {
            index: (method_count + 1 + local_count) as u32,
        }),
        argument: Box::new(intro_application.clone()),
    };
    let left = Term::Apply {
        function: Box::new(Term::Var {
            index: local_count as u32,
        }),
        argument: Box::new(intro_application),
    };
    let method_index = method_count - constructor_index + local_count;
    let mut right = Term::Var {
        index: method_index as u32,
    };
    for index in &local.argument_indices {
        right = Term::Apply {
            function: Box::new(right),
            argument: Box::new(Term::Var { index: *index }),
        };
    }
    for recursive_position in &constructor_code.recursive_positions {
        let Some(argument_index) = local.argument_indices.get(usize::from(*recursive_position))
        else {
            return GscOutcome::Unknown(GscUnknownReason::MalformedCode);
        };
        let recursive_call = Term::Apply {
            function: Box::new(Term::Var {
                index: local_count as u32,
            }),
            argument: Box::new(Term::Var {
                index: *argument_index,
            }),
        };
        right = Term::Apply {
            function: Box::new(right),
            argument: Box::new(recursive_call),
        };
    }

    let mut equation_context = use_family.parameter_context().clone();
    equation_context.0.push(source_motive.clone());
    equation_context.0.extend(local.binders);
    let computation_role = OutputRole::ComputationPort {
        former: frame.id().clone(),
        constructor: constructor.clone(),
        computation_mode: frame.code().code().generated_computation_mode,
    };
    let family = CanonicalDemandFamilyV2 {
        schema_version: CANONICAL_DEMAND_FAMILY_SCHEMA_VERSION,
        semantic_manifest_digest: manifest.digest().clone(),
        rule: GscRule::Compute,
        rank: 2,
        parameter_context: use_family.parameter_context().clone(),
        premise_refs: vec![PremiseRef {
            source: PortRef::Generated {
                port: use_port.clone(),
            },
            expected_role: expected_use_role,
            context_map: ContextMapCode::IdentityV1,
        }],
        output_clauses: vec![OutputClause::EquationPort {
            context: equation_context.clone(),
            equation_code: EquationClauseCode::GeneratedUseBeta {
                former: frame.id().clone(),
                constructor: constructor.clone(),
                use_port: use_port.clone(),
            },
            role: computation_role,
        }],
        verification_judgments: vec![
            OpenJudgment::HasType {
                context: equation_context.clone(),
                term: left,
                ty: motive.clone(),
            },
            OpenJudgment::HasType {
                context: equation_context,
                term: right,
                ty: motive,
            },
        ],
        principal_sources: frame.principal_sources().to_vec(),
        birth_support: frame.birth_support().to_vec(),
        fixed_type_support: fixed_type_support(signature, frame),
        parameter_support_projections: (0..use_family.parameter_context().0.len() as u32).collect(),
    };
    verify_compiled_family(manifest, kernel, signature, family)
}

fn unsupported_frame<T>(frame: &VerifiedClosedFormerFrame) -> GscOutcome<T> {
    if !frame
        .code()
        .code()
        .constructors
        .iter()
        .all(|constructor| constructor.boundary_ports.is_empty())
    {
        GscOutcome::Unknown(GscUnknownReason::UnsupportedBoundary)
    } else if frame.code().code().generated_computation_mode
        != super::inductive::ComputationMode::JudgmentalFreshHead
    {
        GscOutcome::Unknown(GscUnknownReason::UnsupportedComputationMode)
    } else {
        GscOutcome::Unknown(GscUnknownReason::UnsupportedCode)
    }
}

fn method_type(
    frame: &VerifiedClosedFormerFrame,
    constructor_index: usize,
    constructor: &ConstructorCode,
    prior_methods: usize,
) -> Option<Term> {
    let introduction = frame
        .introduction(frame.code().constructor_id(constructor_index)?)?
        .clone();
    let local = computation_locals(frame.owner(), constructor, prior_methods)?;
    let local_count = local.binders.len();
    let intro_application =
        apply_arguments(Term::Global { id: introduction }, &local.argument_indices);
    let mut result = Term::Apply {
        function: Box::new(Term::Var {
            index: (prior_methods + local_count) as u32,
        }),
        argument: Box::new(intro_application),
    };
    for parameter in local.binders.into_iter().rev() {
        result = Term::Pi {
            parameter: Box::new(parameter),
            body: Box::new(result),
        };
    }
    Some(result)
}

struct ComputationLocals {
    binders: Vec<Term>,
    argument_indices: Vec<u32>,
}

struct ComputationArguments {
    binders: Vec<Term>,
    argument_indices: Vec<u32>,
}

fn computation_arguments(
    owner: &GlobalId,
    constructor: &ConstructorCode,
) -> Option<ComputationArguments> {
    let binders = constructor
        .arguments
        .0
        .iter()
        .map(|argument| code_term_to_kernel(argument, owner))
        .collect::<Option<Vec<_>>>()?;
    let total = binders.len();
    let argument_indices = (0..total)
        .map(|position| (total - 1 - position) as u32)
        .collect();
    Some(ComputationArguments {
        binders,
        argument_indices,
    })
}

fn computation_locals(
    owner: &GlobalId,
    constructor: &ConstructorCode,
    p_outer_index: usize,
) -> Option<ComputationLocals> {
    let mut binders = constructor
        .arguments
        .0
        .iter()
        .map(|argument| code_term_to_kernel(argument, owner))
        .collect::<Option<Vec<_>>>()?;
    let argument_positions = (0..binders.len()).collect::<Vec<_>>();
    for recursive_position in &constructor.recursive_positions {
        let argument_position = usize::from(*recursive_position);
        let argument_index = binders.len().checked_sub(1 + argument_position)?;
        let p_index = p_outer_index.checked_add(binders.len())?;
        let hypothesis = Term::Apply {
            function: Box::new(Term::Var {
                index: p_index as u32,
            }),
            argument: Box::new(Term::Var {
                index: argument_index as u32,
            }),
        };
        binders.push(hypothesis);
    }
    let total = binders.len();
    let argument_indices = argument_positions
        .into_iter()
        .map(|position| (total - 1 - position) as u32)
        .collect();
    Some(ComputationLocals {
        binders,
        argument_indices,
    })
}

fn apply_arguments(mut function: Term, arguments: &[u32]) -> Term {
    for index in arguments {
        function = Term::Apply {
            function: Box::new(function),
            argument: Box::new(Term::Var { index: *index }),
        };
    }
    function
}

fn fixed_type_support(
    signature: &VerifiedSignature,
    frame: &VerifiedClosedFormerFrame,
) -> Vec<GlobalId> {
    let declarations = signature
        .declarations()
        .iter()
        .map(|declaration| (&declaration.id, declaration))
        .collect::<BTreeMap<_, _>>();
    let mut support = frame
        .frame()
        .source_declarations
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut pending = support.iter().cloned().collect::<Vec<_>>();
    while let Some(global) = pending.pop() {
        let Some(declaration) = declarations.get(&global) else {
            continue;
        };
        let mut direct = BTreeSet::new();
        collect_globals(&declaration.ty, &mut direct);
        for dependency in direct {
            if support.insert(dependency.clone()) {
                pending.push(dependency);
            }
        }
    }
    support.into_iter().collect()
}

fn collect_globals(term: &Term, output: &mut BTreeSet<GlobalId>) {
    match term {
        Term::Global { id } => {
            output.insert(id.clone());
        }
        Term::Pi { parameter, body } | Term::Sigma { parameter, body } => {
            collect_globals(parameter, output);
            collect_globals(body, output);
        }
        Term::Lambda {
            parameter_type,
            body,
        } => {
            collect_globals(parameter_type, output);
            collect_globals(body, output);
        }
        Term::Apply { function, argument } => {
            collect_globals(function, output);
            collect_globals(argument, output);
        }
        Term::Pair {
            sigma_type,
            first,
            second,
        } => {
            collect_globals(sigma_type, output);
            collect_globals(first, output);
            collect_globals(second, output);
        }
        Term::First { pair } | Term::Second { pair } => collect_globals(pair, output),
        Term::Sort { .. } | Term::Var { .. } | Term::UnitType | Term::Unit => {}
    }
}
