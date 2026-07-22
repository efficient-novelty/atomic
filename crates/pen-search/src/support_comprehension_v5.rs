//! Canonical support-comprehension repair for the unique T-SM1b cycle.
//!
//! This module is an additive successor to the v4 open-specialization
//! theorem.  It does not relax the predecessor condition.  Instead it
//! reconstructs the independent support of the sealed chronological
//! instance, declares the resulting dependent telescope, and realizes the
//! two interface slots over that telescope.

use crate::motive_typed_open_specialization_v4::{
    DependentContextualInternalProjectionV4, ExactOpenDependencyCycleBlockerV4,
    MotiveTypedOpenSpecializationProjectionV4, OpenInternalDerivationV4,
    issue_dependent_contextual_internal_v4, issue_exact_open_dependency_cycle_blocker_v4,
    issue_motive_typed_open_specialization_v4, replay_exact_open_dependency_cycle_blocker_v4,
    replay_motive_typed_open_specialization_v4,
};
use crate::t_sm1a_contextual_formation_v4::issue_chronological_image_internal_v4;
use pen_core::clause::{ClauseRec, ClauseRole};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::a3_demand_grammar::{
    A3ChronologicalInterfaceMode, A3DemandOutputType, A3RuleConstructor, A3TypedClauseSource,
    A3TypedDemandInstance, A3TypedDemandScheme, generate_a3_window_for_exact_prefix_unbounded,
    replay_chronological_interface_slot_map,
};
use pen_eval::typed_families::ParamSort;
use pen_type::contextual_internality::ContextualMotive;
use pen_type::dependent_context::{
    DependentAmbientContextDeclarationProjection, DependentContextMotive,
    issue_dependent_ambient_context_declaration, replay_dependent_ambient_context_declaration,
};
use pen_type::elaborate::{KernelTy, SealedSignature, elaborate_single_clause_with_typed_ambient};
use pen_type::normalize::substitute_level;
use serde::Serialize;
use std::collections::BTreeSet;
use thiserror::Error;

pub const SUPPORT_COMPREHENSION_V5_VERSION: &str =
    "support-comprehension-context-v1-chronological-successor-v5";
pub const SUPPORT_COMPREHENSION_V5_DATE: &str = "2026-07-22";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/support_comprehension_adjudication.md");

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(SUPPORT_COMPREHENSION_V5_VERSION, domain, value))
        .expect("support-comprehension evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn analysis_step(
    step: u32,
    rule: &str,
    input_hashes: Vec<String>,
    output_hash: String,
    replayed_fact: String,
) -> CanonicalSupportAnalysisStepV5 {
    let mut projection = CanonicalSupportAnalysisStepV5 {
        step,
        rule: rule.to_owned(),
        input_hashes,
        output_hash,
        replayed_fact,
        selected_by_derivation_success: false,
        step_hash: String::new(),
    };
    projection.step_hash = tagged_hash("canonical-support-analysis-step", &projection);
    projection
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct IndependentSupportGeneratorV5 {
    pub support_parameter: u32,
    pub sealed_older_parameter: u32,
    pub parameter_sort: ParamSort,
    pub kernel_type: KernelTy,
    pub occurs_in_first_realizer: bool,
    pub has_no_classifier_dependencies: bool,
    pub generator_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalInterfaceRealizerV5 {
    pub interface_slot: u32,
    pub expression: Expr,
    pub required_kernel_type: KernelTy,
    pub realized_by_fresh_support_hypothesis: bool,
    pub consumed_whole: bool,
    pub kappa_charge: u32,
    pub nu_charge: u32,
    pub anchors_minted: u32,
    pub demand_orbits_minted: u32,
    pub realizer_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupportDependencyEdgeV5 {
    pub classifier_parameter: u32,
    pub referenced_parameter: u32,
    pub reference_is_strict_predecessor: bool,
    pub edge_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalSupportAnalysisStepV5 {
    pub step: u32,
    pub rule: String,
    pub input_hashes: Vec<String>,
    pub output_hash: String,
    pub replayed_fact: String,
    pub selected_by_derivation_success: bool,
    pub step_hash: String,
}

/// Output of the deterministic analysis which runs before the repaired
/// specialization is attempted.  Keeping this projection separate makes the
/// canonicity criterion replayable rather than a comment on the successful
/// derivation.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalSupportAnalysisV5 {
    pub version: String,
    pub date: String,
    pub adoption_hash: String,
    pub sealed_instance: A3TypedDemandInstance,
    pub sealed_scheme: A3TypedDemandScheme,
    pub older_source: A3TypedClauseSource,
    pub newest_source: A3TypedClauseSource,
    pub sealed_input_hash: String,
    pub v4_cycle_blocker_regression_hash: String,
    pub v4_blocker_matches_sealed_analysis: bool,
    pub interface_mode: A3ChronologicalInterfaceMode,
    pub flattened_interface_assignments: Vec<(u32, u32)>,
    pub source_interface_motives: Vec<DependentContextMotive>,
    pub first_realizer: Expr,
    pub first_realizer_free_parameters: Vec<u32>,
    pub maximal_independent_generators: Vec<IndependentSupportGeneratorV5>,
    pub every_first_realizer_dependency_is_a_generator: bool,
    pub every_generator_is_used: bool,
    pub flattened_second_realizer: Expr,
    pub flattened_second_actual_type: KernelTy,
    pub required_second_type_after_first_realizer: KernelTy,
    pub flattened_second_realizer_has_classifier_collision: bool,
    pub least_fresh_support_parameter: u32,
    pub support_motives: Vec<DependentContextMotive>,
    pub interface_realizers: Vec<CanonicalInterfaceRealizerV5>,
    pub no_padding_or_unused_generator: bool,
    pub unique_minimal_support_extension_proved: bool,
    pub analysis_trace: Vec<CanonicalSupportAnalysisStepV5>,
    pub analysis_trace_is_total_and_ordered: bool,
    pub support_selected_without_derivation_success: bool,
    pub analysis_precedes_specialization: bool,
    pub analysis_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupportComprehensionDerivationV5 {
    pub version: String,
    pub date: String,
    pub canonical_analysis: CanonicalSupportAnalysisV5,
    pub support_probe_declaration: DependentAmbientContextDeclarationProjection,
    pub specialization: MotiveTypedOpenSpecializationProjectionV4,
    pub specialized_expression: Expr,
    pub support_dependency_graph: Vec<SupportDependencyEdgeV5>,
    pub topological_order: Vec<u32>,
    pub every_dependency_is_a_strict_predecessor: bool,
    pub support_context_acyclic: bool,
    pub support_context_canonical: bool,
    pub support_parameters_kappa_charge: u32,
    pub support_parameters_nu_charge: u32,
    pub support_parameters_anchors_minted: u32,
    pub support_parameters_demand_orbits_minted: u32,
    pub realizers_kappa_charge: u32,
    pub realizers_nu_charge: u32,
    pub realizers_anchors_minted: u32,
    pub realizers_demand_orbits_minted: u32,
    pub zero_accounting_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SupportComprehensionV5Error {
    #[error("v4 cycle witness failed: {0}")]
    V4(String),
    #[error("sealed A3 instance reconstruction failed: {0}")]
    SealedInstance(String),
    #[error("canonical dependency analysis failed: {0}")]
    CanonicalAnalysis(String),
    #[error("dependent support declaration failed: {0}")]
    Dependent(String),
    #[error("chronological image Internal failed: {0}")]
    ImageInternal(String),
    #[error("support-comprehension specialization failed: {0}")]
    Specialization(String),
    #[error("support-comprehension replay mismatch")]
    ReplayMismatch,
}

#[derive(Clone, Debug)]
struct SealedSupportAnalysisInputV5 {
    instance: A3TypedDemandInstance,
    scheme: A3TypedDemandScheme,
    older_source: A3TypedClauseSource,
    newest_source: A3TypedClauseSource,
    interface_mode: A3ChronologicalInterfaceMode,
    flattened_interface_assignments: Vec<(u32, u32)>,
    source_interface_motives: Vec<DependentContextMotive>,
    first_realizer: Expr,
}

fn source_motives_from_sealed_expression(
    newest: &A3TypedClauseSource,
) -> Option<Vec<DependentContextMotive>> {
    if newest.canonical_presentation.parameters.len() != 2 {
        return None;
    }
    let Expr::Lam(body) = &newest.canonical_presentation.canonical_normal_form else {
        return None;
    };
    let Expr::App(function, argument) = body.as_ref() else {
        return None;
    };
    let Expr::Eventually(head) = function.as_ref() else {
        return None;
    };
    if argument.as_ref() != &Expr::Var(2) || head.var_refs().iter().any(|parameter| *parameter >= 2)
    {
        return None;
    }
    Some(vec![
        DependentContextMotive::Independent {
            motive: ContextualMotive::Type,
        },
        DependentContextMotive::ElementOfApplicationHead {
            head: Expr::Eventually(head.clone()),
        },
    ])
}

/// Select the comprehension input solely from the sealed A3 surface.  The
/// selection criterion is the forbidden self-dependency in the flattened
/// slot map; neither the v4 blocker nor a successful repaired derivation is
/// available to this function.
fn exact_sealed_input() -> Result<SealedSupportAnalysisInputV5, SupportComprehensionV5Error> {
    let signature = SealedSignature::genesis_del_h15();
    let window = generate_a3_window_for_exact_prefix_unbounded(&signature, 16)
        .map_err(|error| SupportComprehensionV5Error::SealedInstance(error.to_string()))?;
    let mut candidates = Vec::new();
    for instance in &window.instances {
        if instance.source_anchor_ids.len() != 2 {
            continue;
        }
        let Some(scheme) = window
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == instance.scheme_id)
        else {
            return Err(SupportComprehensionV5Error::SealedInstance(
                "live A3 instance has no scheme".to_owned(),
            ));
        };
        if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison {
            continue;
        }
        let Some(older_source) = window
            .typed_sources
            .iter()
            .find(|source| source.anchor_id == instance.source_anchor_ids[0])
        else {
            return Err(SupportComprehensionV5Error::SealedInstance(
                "chronological older source is absent".to_owned(),
            ));
        };
        let Some(newest_source) = window
            .typed_sources
            .iter()
            .find(|source| source.anchor_id == instance.source_anchor_ids[1])
        else {
            return Err(SupportComprehensionV5Error::SealedInstance(
                "chronological newest source is absent".to_owned(),
            ));
        };
        if newest_source.kernel_type == KernelTy::Type {
            continue;
        }
        let Some(source_interface_motives) = source_motives_from_sealed_expression(newest_source)
        else {
            continue;
        };
        let A3DemandOutputType::ChronologicalInteraction {
            interface_mode,
            interface_slot_map,
            ..
        } = &scheme.required_output
        else {
            continue;
        };
        replay_chronological_interface_slot_map(
            interface_slot_map,
            interface_slot_map.declared_arity,
        )
        .map_err(|error| SupportComprehensionV5Error::SealedInstance(error.to_string()))?;
        let first_realizer = match interface_mode {
            A3ChronologicalInterfaceMode::DirectType => older_source
                .canonical_presentation
                .canonical_normal_form
                .clone(),
            A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { .. } => Expr::App(
                Box::new(
                    older_source
                        .canonical_presentation
                        .canonical_normal_form
                        .clone(),
                ),
                Box::new(Expr::Var(1)),
            ),
        };
        // Parameter 2 occurs in the classifier of flattened p2.  This is
        // the canonical graph criterion; no attempted repair is consulted.
        if !first_realizer.var_refs().contains(&2) {
            continue;
        }
        candidates.push(SealedSupportAnalysisInputV5 {
            instance: instance.clone(),
            scheme: scheme.clone(),
            older_source: older_source.clone(),
            newest_source: newest_source.clone(),
            interface_mode: interface_mode.clone(),
            flattened_interface_assignments: interface_slot_map
                .assignments
                .iter()
                .map(|assignment| (assignment.interface_slot, assignment.parameter))
                .collect(),
            source_interface_motives,
            first_realizer,
        });
    }
    if candidates.len() != 1 {
        return Err(SupportComprehensionV5Error::SealedInstance(format!(
            "canonical sealed dependency analysis found {} comprehension candidates, expected one",
            candidates.len()
        )));
    }
    Ok(candidates.remove(0))
}

fn canonical_support_analysis() -> Result<CanonicalSupportAnalysisV5, SupportComprehensionV5Error> {
    let input = exact_sealed_input()?;
    let instance = input.instance;
    let scheme = input.scheme;
    let older_source = input.older_source;
    let newest_source = input.newest_source;
    let interface_mode = input.interface_mode;
    let flattened_interface_assignments = input.flattened_interface_assignments;
    let source_interface_motives = input.source_interface_motives;
    let first_realizer = input.first_realizer;
    let A3DemandOutputType::ChronologicalInteraction {
        interface_slot_map, ..
    } = &scheme.required_output
    else {
        return Err(SupportComprehensionV5Error::CanonicalAnalysis(
            "sealed scheme is not a chronological interaction".to_owned(),
        ));
    };
    replay_chronological_interface_slot_map(interface_slot_map, interface_slot_map.declared_arity)
        .map_err(|error| SupportComprehensionV5Error::CanonicalAnalysis(error.to_string()))?;
    if flattened_interface_assignments != vec![(1, 1), (2, 2)] {
        return Err(SupportComprehensionV5Error::CanonicalAnalysis(
            "the exact flattened identity interface did not replay".to_owned(),
        ));
    }

    let first_realizer_free_parameters = first_realizer.var_refs().into_iter().collect::<Vec<_>>();
    let generator_parameters = first_realizer_free_parameters
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let older_parameters = &older_source.canonical_presentation.parameters;
    if generator_parameters.is_empty()
        || generator_parameters
            .iter()
            .any(|parameter| *parameter == 0 || *parameter as usize > older_parameters.len())
    {
        return Err(SupportComprehensionV5Error::CanonicalAnalysis(
            "first realizer refers outside the sealed older support".to_owned(),
        ));
    }
    let mut maximal_independent_generators = Vec::new();
    for parameter in generator_parameters.iter().copied() {
        let sort = older_parameters[parameter as usize - 1].clone();
        let kernel_type = match sort {
            ParamSort::Type => KernelTy::Type,
            _ => {
                return Err(SupportComprehensionV5Error::CanonicalAnalysis(format!(
                    "older generator q{parameter} is not an independent Type parameter"
                )));
            }
        };
        let mut generator = IndependentSupportGeneratorV5 {
            support_parameter: parameter,
            sealed_older_parameter: parameter,
            parameter_sort: sort,
            kernel_type,
            occurs_in_first_realizer: first_realizer_free_parameters.contains(&parameter),
            has_no_classifier_dependencies: true,
            generator_hash: String::new(),
        };
        generator.generator_hash = tagged_hash("independent-support-generator", &generator);
        maximal_independent_generators.push(generator);
    }
    let every_first_realizer_dependency_is_a_generator = first_realizer_free_parameters
        .iter()
        .all(|parameter| generator_parameters.contains(parameter));
    let every_generator_is_used = maximal_independent_generators
        .iter()
        .all(|generator| generator.occurs_in_first_realizer);
    if !every_first_realizer_dependency_is_a_generator || !every_generator_is_used {
        return Err(SupportComprehensionV5Error::CanonicalAnalysis(
            "maximal independent support is not exact".to_owned(),
        ));
    }

    let DependentContextMotive::ElementOfApplicationHead { head: source_head } =
        &source_interface_motives[1]
    else {
        return Err(SupportComprehensionV5Error::CanonicalAnalysis(
            "the second source motive is not an element application".to_owned(),
        ));
    };
    let required_head_after_first = substitute_level(source_head, 1, &first_realizer);
    let required_second_type_after_first_realizer = KernelTy::El(required_head_after_first.clone());
    let flattened_second_realizer = Expr::Var(2);
    let independent_types = maximal_independent_generators
        .iter()
        .map(|generator| generator.kernel_type.clone())
        .collect::<Vec<_>>();
    let (flattened_second_typed, _) = elaborate_single_clause_with_typed_ambient(
        &flattened_second_realizer,
        &independent_types,
        &[],
        15,
    )
    .map_err(|error| SupportComprehensionV5Error::CanonicalAnalysis(error.to_string()))?;
    let flattened_second_actual_type = flattened_second_typed.kernel_ty;
    let flattened_second_realizer_has_classifier_collision =
        flattened_second_actual_type != required_second_type_after_first_realizer;
    if !flattened_second_realizer_has_classifier_collision {
        return Err(SupportComprehensionV5Error::CanonicalAnalysis(
            "flattened second slot has no classifier collision to comprehend".to_owned(),
        ));
    }

    let least_fresh_support_parameter = maximal_independent_generators.len() as u32 + 1;
    let mut support_motives = maximal_independent_generators
        .iter()
        .map(|_| DependentContextMotive::Independent {
            motive: ContextualMotive::Type,
        })
        .collect::<Vec<_>>();
    support_motives.push(DependentContextMotive::ElementOfApplicationHead {
        head: required_head_after_first,
    });
    let realizer_terms = [
        first_realizer.clone(),
        Expr::Var(least_fresh_support_parameter),
    ];
    let required_types = [
        KernelTy::Type,
        required_second_type_after_first_realizer.clone(),
    ];
    let mut interface_realizers = Vec::new();
    for (index, (expression, required_kernel_type)) in
        realizer_terms.into_iter().zip(required_types).enumerate()
    {
        let mut realizer = CanonicalInterfaceRealizerV5 {
            interface_slot: index as u32 + 1,
            expression,
            required_kernel_type,
            realized_by_fresh_support_hypothesis: index == 1,
            consumed_whole: true,
            kappa_charge: 0,
            nu_charge: 0,
            anchors_minted: 0,
            demand_orbits_minted: 0,
            realizer_hash: String::new(),
        };
        realizer.realizer_hash = tagged_hash("canonical-interface-realizer", &realizer);
        interface_realizers.push(realizer);
    }
    let no_padding_or_unused_generator = support_motives.len()
        == maximal_independent_generators.len() + 1
        && every_generator_is_used
        && interface_realizers[1].expression == Expr::Var(least_fresh_support_parameter);
    let unique_minimal_support_extension_proved = flattened_second_realizer_has_classifier_collision
        && least_fresh_support_parameter == generator_parameters.len() as u32 + 1
        && no_padding_or_unused_generator;
    if !unique_minimal_support_extension_proved {
        return Err(SupportComprehensionV5Error::CanonicalAnalysis(
            "the least fresh dependent witness was not uniquely determined".to_owned(),
        ));
    }
    let sealed_input_hash = tagged_hash(
        "sealed-support-analysis-input",
        &(&instance, &scheme, &older_source, &newest_source),
    );
    let first_realizer_hash = tagged_hash("analysis-first-realizer", &first_realizer);
    let generator_hash = tagged_hash(
        "analysis-maximal-independent-generators",
        &maximal_independent_generators,
    );
    let instantiated_classifier_hash = tagged_hash(
        "analysis-instantiated-second-classifier",
        &required_second_type_after_first_realizer,
    );
    let collision_hash = tagged_hash(
        "analysis-flattened-collision",
        &(
            &flattened_second_realizer,
            &flattened_second_actual_type,
            &required_second_type_after_first_realizer,
        ),
    );
    let support_hash = tagged_hash(
        "analysis-canonical-support-output",
        &(
            least_fresh_support_parameter,
            &support_motives,
            &interface_realizers,
        ),
    );
    let analysis_trace = vec![
        analysis_step(
            1,
            "seal-live-a3-instance-and-source-pair",
            vec![],
            sealed_input_hash.clone(),
            "The unique forbidden self-dependency is selected from the live sealed A3 surface without consulting v4 or a repaired derivation.".to_owned(),
        ),
        analysis_step(
            2,
            "extract-first-interface-realizer",
            vec![sealed_input_hash.clone()],
            first_realizer_hash.clone(),
            format!(
                "The declared {:?} interface computes p1 from the sealed older family.",
                interface_mode
            ),
        ),
        analysis_step(
            3,
            "maximal-independent-generator-closure",
            vec![first_realizer_hash],
            generator_hash.clone(),
            format!(
                "Free parameters {:?} are exactly the used independent generators.",
                first_realizer_free_parameters
            ),
        ),
        analysis_step(
            4,
            "instantiate-source-second-classifier",
            vec![generator_hash],
            instantiated_classifier_hash.clone(),
            format!(
                "Substituting p1 yields the exact required classifier {:?}.",
                required_second_type_after_first_realizer
            ),
        ),
        analysis_step(
            5,
            "detect-flattened-classifier-collision",
            vec![instantiated_classifier_hash],
            collision_hash.clone(),
            format!(
                "Flattened p2 has {:?}, not {:?}; the least fresh support index is {}.",
                flattened_second_actual_type,
                required_second_type_after_first_realizer,
                least_fresh_support_parameter
            ),
        ),
        analysis_step(
            6,
            "emit-minimal-support-context-and-realizers",
            vec![collision_hash],
            support_hash,
            "The generators are followed by exactly one dependent witness; p1 is the composite and p2 is that witness, with no padding.".to_owned(),
        ),
    ];
    let analysis_trace_is_total_and_ordered = analysis_trace
        .iter()
        .enumerate()
        .all(|(index, step)| step.step == index as u32 + 1);
    let support_selected_without_derivation_success = analysis_trace
        .iter()
        .all(|step| !step.selected_by_derivation_success);
    let mut analysis = CanonicalSupportAnalysisV5 {
        version: SUPPORT_COMPREHENSION_V5_VERSION.to_owned(),
        date: SUPPORT_COMPREHENSION_V5_DATE.to_owned(),
        adoption_hash: bytes_hash(ADJUDICATION_BYTES),
        sealed_instance: instance,
        sealed_scheme: scheme,
        older_source,
        newest_source,
        sealed_input_hash,
        v4_cycle_blocker_regression_hash: String::new(),
        v4_blocker_matches_sealed_analysis: false,
        interface_mode,
        flattened_interface_assignments,
        source_interface_motives,
        first_realizer,
        first_realizer_free_parameters,
        maximal_independent_generators,
        every_first_realizer_dependency_is_a_generator,
        every_generator_is_used,
        flattened_second_realizer,
        flattened_second_actual_type,
        required_second_type_after_first_realizer,
        flattened_second_realizer_has_classifier_collision,
        least_fresh_support_parameter,
        support_motives,
        interface_realizers,
        no_padding_or_unused_generator,
        unique_minimal_support_extension_proved,
        analysis_trace,
        analysis_trace_is_total_and_ordered,
        support_selected_without_derivation_success,
        analysis_precedes_specialization: true,
        analysis_hash: String::new(),
    };
    analysis.analysis_hash = tagged_hash("canonical-support-analysis", &analysis);
    Ok(analysis)
}

fn attach_v4_cycle_regression(
    mut analysis: CanonicalSupportAnalysisV5,
    blocker: &ExactOpenDependencyCycleBlockerV4,
) -> Result<CanonicalSupportAnalysisV5, SupportComprehensionV5Error> {
    let blocker_matches = blocker.instance_id == analysis.sealed_instance.instance_id
        && blocker.older_source == analysis.older_source
        && blocker.newest_source == analysis.newest_source
        && blocker.interface_mode == analysis.interface_mode
        && blocker.interface_assignments == analysis.flattened_interface_assignments
        && blocker.first_image == analysis.first_realizer
        && analysis.source_interface_motives.get(1) == Some(&blocker.source_second_motive);
    if !blocker_matches {
        return Err(SupportComprehensionV5Error::CanonicalAnalysis(
            "non-selecting v4 cycle regression does not match the sealed analysis".to_owned(),
        ));
    }
    let pre_regression_analysis_hash = analysis.analysis_hash.clone();
    analysis.v4_cycle_blocker_regression_hash = blocker.blocker_hash.clone();
    analysis.v4_blocker_matches_sealed_analysis = true;
    analysis.analysis_trace.push(analysis_step(
        7,
        "compare-v4-cycle-witness-after-support-selection",
        vec![pre_regression_analysis_hash],
        blocker.blocker_hash.clone(),
        "The v4 negative theorem agrees with the already-selected sealed dependency surface; it did not select the support context.".to_owned(),
    ));
    analysis.analysis_trace_is_total_and_ordered = analysis
        .analysis_trace
        .iter()
        .enumerate()
        .all(|(index, step)| step.step == index as u32 + 1);
    analysis.support_selected_without_derivation_success = analysis
        .analysis_trace
        .iter()
        .all(|step| !step.selected_by_derivation_success);
    analysis.analysis_hash.clear();
    analysis.analysis_hash = tagged_hash("canonical-support-analysis", &analysis);
    Ok(analysis)
}

fn source_internal(
    signature: &SealedSignature,
    analysis: &CanonicalSupportAnalysisV5,
) -> Result<OpenInternalDerivationV4, SupportComprehensionV5Error> {
    let body = Telescope::new(vec![ClauseRec::new(
        analysis.newest_source.kernel_role,
        analysis
            .newest_source
            .canonical_presentation
            .canonical_normal_form
            .clone(),
    )]);
    let declaration = issue_dependent_ambient_context_declaration(
        signature,
        &body,
        15,
        analysis.source_interface_motives.clone(),
    )
    .map_err(|error| SupportComprehensionV5Error::Dependent(error.to_string()))?;
    let internal = issue_dependent_contextual_internal_v4(signature, declaration.projection())
        .map_err(|error| SupportComprehensionV5Error::V4(error.to_string()))?;
    Ok(OpenInternalDerivationV4::DependentContextual {
        derivation: internal.projection().clone(),
    })
}

fn projection_internal(
    signature: &SealedSignature,
    motives: &[DependentContextMotive],
    term: Expr,
) -> Result<DependentContextualInternalProjectionV4, SupportComprehensionV5Error> {
    let probe = Telescope::new(vec![ClauseRec::new(ClauseRole::Introduction, term.clone())]);
    let probe_declaration =
        issue_dependent_ambient_context_declaration(signature, &probe, 15, motives.to_vec())
            .map_err(|error| SupportComprehensionV5Error::Dependent(error.to_string()))?;
    let role = probe_declaration
        .projection()
        .typed_body_elaboration
        .kernel_role;
    let body = Telescope::new(vec![ClauseRec::new(role, term)]);
    let declaration =
        issue_dependent_ambient_context_declaration(signature, &body, 15, motives.to_vec())
            .map_err(|error| SupportComprehensionV5Error::Dependent(error.to_string()))?;
    let internal = issue_dependent_contextual_internal_v4(signature, declaration.projection())
        .map_err(|error| SupportComprehensionV5Error::V4(error.to_string()))?;
    Ok(internal.projection().clone())
}

pub fn issue_support_comprehension_derivation_v5()
-> Result<SupportComprehensionDerivationV5, SupportComprehensionV5Error> {
    let signature = SealedSignature::genesis_del_h15();

    // Canonical support selection consumes only the live sealed instance.
    // The v4 blocker is replayed afterward as a non-selecting regression.
    let analysis = canonical_support_analysis()?;
    let blocker = issue_exact_open_dependency_cycle_blocker_v4()
        .map_err(|error| SupportComprehensionV5Error::V4(error.to_string()))?;
    replay_exact_open_dependency_cycle_blocker_v4(&blocker)
        .map_err(|error| SupportComprehensionV5Error::V4(error.to_string()))?;
    let analysis = attach_v4_cycle_regression(analysis, &blocker)?;

    // The analysis is completed and sealed before any repaired
    // specialization is attempted.
    let witness_term = Expr::Var(analysis.least_fresh_support_parameter);
    let probe = Telescope::new(vec![ClauseRec::new(
        ClauseRole::Introduction,
        witness_term.clone(),
    )]);
    let support_probe_token = issue_dependent_ambient_context_declaration(
        &signature,
        &probe,
        15,
        analysis.support_motives.clone(),
    )
    .map_err(|error| SupportComprehensionV5Error::Dependent(error.to_string()))?;
    replay_dependent_ambient_context_declaration(&signature, support_probe_token.projection())
        .map_err(|error| SupportComprehensionV5Error::Dependent(error.to_string()))?;
    let support_probe_declaration = support_probe_token.projection().clone();

    let source = source_internal(&signature, &analysis)?;
    let first_image = issue_chronological_image_internal_v4(
        &signature,
        15,
        &analysis.older_source,
        analysis.interface_mode.clone(),
        support_probe_declaration.exact_ambient_kernel_types.clone(),
    )
    .map_err(|error| SupportComprehensionV5Error::ImageInternal(error.to_string()))?;
    let second_image =
        projection_internal(&signature, &analysis.support_motives, witness_term.clone())?;
    let specialization_token = issue_motive_typed_open_specialization_v4(
        &signature,
        source,
        analysis.support_motives.clone(),
        vec![
            (
                analysis.first_realizer.clone(),
                OpenInternalDerivationV4::ChronologicalImage {
                    derivation: first_image.projection().clone(),
                },
            ),
            (
                witness_term,
                OpenInternalDerivationV4::DependentContextual {
                    derivation: second_image,
                },
            ),
        ],
    )
    .map_err(|error| SupportComprehensionV5Error::Specialization(error.to_string()))?;
    replay_motive_typed_open_specialization_v4(&signature, specialization_token.projection())
        .map_err(|error| SupportComprehensionV5Error::Specialization(error.to_string()))?;
    let specialization = specialization_token.projection().clone();

    let mut support_dependency_graph = Vec::new();
    for hypothesis in &support_probe_declaration.hypotheses {
        for referenced_parameter in &hypothesis.dependency_parameters {
            let mut edge = SupportDependencyEdgeV5 {
                classifier_parameter: hypothesis.parameter,
                referenced_parameter: *referenced_parameter,
                reference_is_strict_predecessor: *referenced_parameter < hypothesis.parameter,
                edge_hash: String::new(),
            };
            edge.edge_hash = tagged_hash("support-dependency-edge", &edge);
            support_dependency_graph.push(edge);
        }
    }
    support_dependency_graph
        .sort_by_key(|edge| (edge.classifier_parameter, edge.referenced_parameter));
    let topological_order = support_probe_declaration.sequential_order.clone();
    let every_dependency_is_a_strict_predecessor = support_dependency_graph
        .iter()
        .all(|edge| edge.reference_is_strict_predecessor);
    let support_context_acyclic = every_dependency_is_a_strict_predecessor
        && support_probe_declaration.every_motive_formable_over_predecessors
        && topological_order == (1..=support_probe_declaration.declared_arity).collect::<Vec<_>>();
    let support_context_canonical = analysis.unique_minimal_support_extension_proved
        && analysis.analysis_trace_is_total_and_ordered
        && analysis.support_selected_without_derivation_success
        && analysis.v4_blocker_matches_sealed_analysis
        && analysis.analysis_precedes_specialization
        && support_probe_declaration.declared_arity == analysis.support_motives.len() as u32
        && support_probe_declaration
            .hypotheses
            .iter()
            .map(|hypothesis| &hypothesis.motive)
            .eq(analysis.support_motives.iter());
    let support_parameters_kappa_charge = support_probe_declaration.marginal_kappa;
    let support_parameters_nu_charge = support_probe_declaration.marginal_nu;
    let support_parameters_anchors_minted = support_probe_declaration.anchors_minted;
    let support_parameters_demand_orbits_minted = 0;
    let realizers_kappa_charge = analysis
        .interface_realizers
        .iter()
        .map(|realizer| realizer.kappa_charge)
        .sum();
    let realizers_nu_charge = analysis
        .interface_realizers
        .iter()
        .map(|realizer| realizer.nu_charge)
        .sum();
    let realizers_anchors_minted = analysis
        .interface_realizers
        .iter()
        .map(|realizer| realizer.anchors_minted)
        .sum();
    let realizers_demand_orbits_minted = analysis
        .interface_realizers
        .iter()
        .map(|realizer| realizer.demand_orbits_minted)
        .sum();
    let zero_accounting_proved = support_parameters_kappa_charge == 0
        && support_parameters_nu_charge == 0
        && support_parameters_anchors_minted == 0
        && support_parameters_demand_orbits_minted == 0
        && realizers_kappa_charge == 0
        && realizers_nu_charge == 0
        && realizers_anchors_minted == 0
        && realizers_demand_orbits_minted == 0
        && specialization
            .images
            .iter()
            .all(|image| image.internal_evidence_replayed);
    if !support_context_acyclic || !support_context_canonical || !zero_accounting_proved {
        return Err(SupportComprehensionV5Error::CanonicalAnalysis(
            "support context failed acyclicity, canonicity, or zero-accounting replay".to_owned(),
        ));
    }
    let specialized_expression = specialization.substitution_result.clone();
    let mut derivation = SupportComprehensionDerivationV5 {
        version: SUPPORT_COMPREHENSION_V5_VERSION.to_owned(),
        date: SUPPORT_COMPREHENSION_V5_DATE.to_owned(),
        canonical_analysis: analysis,
        support_probe_declaration,
        specialization,
        specialized_expression,
        support_dependency_graph,
        topological_order,
        every_dependency_is_a_strict_predecessor,
        support_context_acyclic,
        support_context_canonical,
        support_parameters_kappa_charge,
        support_parameters_nu_charge,
        support_parameters_anchors_minted,
        support_parameters_demand_orbits_minted,
        realizers_kappa_charge,
        realizers_nu_charge,
        realizers_anchors_minted,
        realizers_demand_orbits_minted,
        zero_accounting_proved,
        derivation_hash: String::new(),
    };
    derivation.derivation_hash = tagged_hash("support-comprehension-derivation", &derivation);
    Ok(derivation)
}

pub fn replay_support_comprehension_derivation_v5(
    claimed: &SupportComprehensionDerivationV5,
) -> Result<(), SupportComprehensionV5Error> {
    let reissued = issue_support_comprehension_derivation_v5()?;
    if reissued == *claimed {
        Ok(())
    } else {
        Err(SupportComprehensionV5Error::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exact_cycle_reconstructs_as_the_canonical_three_parameter_comprehension() {
        let derivation = issue_support_comprehension_derivation_v5().expect("support derivation");
        let analysis = &derivation.canonical_analysis;
        assert_eq!(analysis.maximal_independent_generators.len(), 2);
        assert_eq!(analysis.least_fresh_support_parameter, 3);
        assert_eq!(
            analysis.interface_realizers[0].expression,
            analysis.first_realizer
        );
        assert_eq!(analysis.interface_realizers[1].expression, Expr::Var(3));
        assert_eq!(analysis.support_motives.len(), 3);
        assert!(analysis.unique_minimal_support_extension_proved);
        assert_eq!(analysis.analysis_trace.len(), 7);
        assert!(analysis.analysis_trace_is_total_and_ordered);
        assert!(analysis.support_selected_without_derivation_success);
        assert!(analysis.v4_blocker_matches_sealed_analysis);
        assert_eq!(
            analysis.support_motives[2],
            DependentContextMotive::ElementOfApplicationHead {
                head: Expr::Eventually(Box::new(analysis.first_realizer.clone())),
            }
        );
        assert!(analysis.interface_realizers.iter().all(|realizer| {
            realizer.kappa_charge == 0
                && realizer.nu_charge == 0
                && realizer.anchors_minted == 0
                && realizer.demand_orbits_minted == 0
        }));
        assert!(derivation.support_context_acyclic);
        assert!(derivation.support_context_canonical);
        assert!(derivation.zero_accounting_proved);
        replay_support_comprehension_derivation_v5(&derivation).expect("replay");
    }

    #[test]
    fn dependency_and_accounting_mutations_fail_replay() {
        let derivation = issue_support_comprehension_derivation_v5().expect("support derivation");
        let mut dependency_mutation = derivation.clone();
        dependency_mutation.support_dependency_graph[0].referenced_parameter = 3;
        assert!(replay_support_comprehension_derivation_v5(&dependency_mutation).is_err());

        let mut charge_mutation = derivation;
        charge_mutation.realizers_nu_charge = 1;
        assert!(replay_support_comprehension_derivation_v5(&charge_mutation).is_err());
    }
}
