//! Proof hardening for the exact support-comprehension successor.
//!
//! The v5 derivation reconstructs the right typed context. This module adds
//! two proof objects that are intentionally independent of successful
//! specialization: a finite graph-normal-form/uniqueness theorem and a typed
//! no-mint capability theorem for hypotheses and references.

use crate::support_comprehension_v5::{
    CanonicalSupportAnalysisV5, SupportComprehensionDerivationV5,
    replay_support_comprehension_derivation_for_row_v5, replay_support_comprehension_derivation_v5,
};
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_eval::a3_demand_grammar::{
    A3TypedClauseSource, A3TypedDemandInstance, A3TypedDemandScheme,
};
use pen_type::dependent_context::DependentContextMotive;
use pen_type::elaborate::SealedSignature;
use serde::Serialize;
use std::collections::BTreeSet;
use thiserror::Error;

pub const SUPPORT_COMPREHENSION_HARDENING_V6_SCHEMA: &str =
    "support-comprehension-graph-normal-form-zero-mint-v6";
pub const SUPPORT_COMPREHENSION_HARDENING_V6_DATE: &str = "2026-07-22";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(SUPPORT_COMPREHENSION_HARDENING_V6_SCHEMA, domain, value))
        .expect("support hardening evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupportGeneratorSubsetDecisionV6 {
    pub generator_parameters: Vec<u32>,
    pub covers_every_first_realizer_dependency: bool,
    pub contains_no_unused_generator: bool,
    pub consists_only_of_independent_type_parameters: bool,
    pub admissible_support: bool,
    pub decision_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupportOrderFactorizationDecisionV6 {
    pub generator_renaming: Vec<u32>,
    pub is_permutation_of_exact_generators: bool,
    pub preserves_declaration_order: bool,
    pub preserves_first_realizer_dependency_graph: bool,
    pub canonical_factorization: bool,
    pub decision_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupportGraphNormalFormV6 {
    pub theorem_id: String,
    pub bound_analysis_hash: String,
    pub sealed_input_hash: String,
    pub older_parameter_universe: Vec<u32>,
    pub first_realizer_free_parameters: Vec<u32>,
    pub enumerated_generator_subsets: Vec<SupportGeneratorSubsetDecisionV6>,
    pub exact_one_admissible_generator_subset: bool,
    pub admissible_generator_subset: Vec<u32>,
    pub every_generator_classifier_dependency_recomputed_empty: bool,
    pub dependent_witness_is_unique_least_fresh_node: bool,
    pub dependent_motive_is_exact_instantiated_classifier: bool,
    pub interface_slot_dependencies: Vec<(u32, Vec<u32>)>,
    pub factorization_decisions: Vec<SupportOrderFactorizationDecisionV6>,
    pub exact_one_order_preserving_factorization: bool,
    pub canonical_dependency_analysis_candidate_grammar_exhaustive: bool,
    pub every_candidate_in_closed_canonical_dependency_analysis_factors_through_comprehension: bool,
    pub selected_by_specialization_success: bool,
    pub proof_scope: String,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportInterfaceOperationV6 {
    IndependentTypeHypothesis,
    DependentElementHypothesis,
    OpaquePriorClauseHypothesis,
    CompositeInterfaceReference,
    FreshHypothesisReference,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportInterfaceCapabilityV6 {
    DeclareTypedHypothesis,
    ReadTypedHypothesis,
    ComposeTypedExpression,
    KappaCredit,
    NuCredit,
    AnchorMint,
    DemandOrbitMint,
}

impl SupportInterfaceCapabilityV6 {
    fn forbidden() -> BTreeSet<Self> {
        [
            Self::KappaCredit,
            Self::NuCredit,
            Self::AnchorMint,
            Self::DemandOrbitMint,
        ]
        .into_iter()
        .collect()
    }
}

fn operation_capabilities(
    operation: SupportInterfaceOperationV6,
) -> Vec<SupportInterfaceCapabilityV6> {
    use SupportInterfaceCapabilityV6 as C;
    match operation {
        SupportInterfaceOperationV6::IndependentTypeHypothesis
        | SupportInterfaceOperationV6::DependentElementHypothesis
        | SupportInterfaceOperationV6::OpaquePriorClauseHypothesis => {
            vec![C::DeclareTypedHypothesis]
        }
        SupportInterfaceOperationV6::CompositeInterfaceReference => {
            vec![C::ReadTypedHypothesis, C::ComposeTypedExpression]
        }
        SupportInterfaceOperationV6::FreshHypothesisReference => {
            vec![C::ReadTypedHypothesis]
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupportNoMintCapabilityRowV6 {
    pub row_id: String,
    pub operation: SupportInterfaceOperationV6,
    pub bound_evidence_hash: String,
    pub capabilities: Vec<SupportInterfaceCapabilityV6>,
    pub forbidden_capabilities: Vec<SupportInterfaceCapabilityV6>,
    pub no_mint: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupportZeroMintCapabilityV6 {
    pub theorem_id: String,
    pub bound_support_derivation_hash: String,
    pub rows: Vec<SupportNoMintCapabilityRowV6>,
    pub expected_row_bindings: Vec<(String, SupportInterfaceOperationV6, String)>,
    pub exact_hypothesis_and_realizer_surface: bool,
    pub exact_typed_operation_bindings: bool,
    pub every_capability_derived_from_closed_operation_enum: bool,
    pub no_forbidden_capability: bool,
    pub derived_kappa_charge: u32,
    pub derived_nu_charge: u32,
    pub derived_anchor_mints: u32,
    pub derived_demand_orbit_mints: u32,
    pub projection_agrees_with_derived_zero: bool,
    pub proof_scope: String,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SupportComprehensionHardeningV6Token {
    pub schema: String,
    pub date: String,
    pub bound_support_derivation_hash: String,
    pub base_derivation_replayed: bool,
    pub graph_normal_form: SupportGraphNormalFormV6,
    pub zero_mint_capability: SupportZeroMintCapabilityV6,
    pub proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SupportComprehensionHardeningV6Error {
    #[error("support-comprehension base derivation failed replay: {0}")]
    Base(String),
    #[error("support graph-normal-form proof failed: {0}")]
    Graph(String),
    #[error("support zero-mint proof failed: {0}")]
    ZeroMint(String),
    #[error("support hardening replay mismatch")]
    ReplayMismatch,
}

fn subsets(universe: &[u32]) -> Vec<Vec<u32>> {
    (0..(1_usize << universe.len()))
        .map(|mask| {
            universe
                .iter()
                .enumerate()
                .filter_map(|(index, parameter)| ((mask & (1 << index)) != 0).then_some(*parameter))
                .collect::<Vec<_>>()
        })
        .collect()
}

fn permutations(values: &[u32]) -> Vec<Vec<u32>> {
    fn visit(prefix: &mut Vec<u32>, remaining: &mut Vec<u32>, output: &mut Vec<Vec<u32>>) {
        if remaining.is_empty() {
            output.push(prefix.clone());
            return;
        }
        for index in 0..remaining.len() {
            let value = remaining.remove(index);
            prefix.push(value);
            visit(prefix, remaining, output);
            prefix.pop();
            remaining.insert(index, value);
        }
    }
    let mut output = Vec::new();
    visit(&mut Vec::new(), &mut values.to_vec(), &mut output);
    output
}

fn motive_dependencies(motive: &DependentContextMotive) -> Vec<u32> {
    match motive {
        DependentContextMotive::Independent { .. } => Vec::new(),
        DependentContextMotive::ElementOfApplicationHead { head } => {
            head.var_refs().into_iter().collect()
        }
        DependentContextMotive::OpaquePriorClause { .. } => Vec::new(),
    }
}

fn graph_normal_form(
    analysis: &CanonicalSupportAnalysisV5,
) -> Result<SupportGraphNormalFormV6, SupportComprehensionHardeningV6Error> {
    let older_parameter_universe = (1..=analysis
        .older_source
        .canonical_presentation
        .parameters
        .len())
        .map(|value| u32::try_from(value).expect("small sealed parameter surface"))
        .collect::<Vec<_>>();
    if older_parameter_universe.len() > 16 {
        return Err(SupportComprehensionHardeningV6Error::Graph(
            "sealed support universe exceeds finite hardening bound".to_owned(),
        ));
    }
    let free_set = analysis
        .first_realizer_free_parameters
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let independent_parameters = analysis
        .maximal_independent_generators
        .iter()
        .filter(|generator| {
            generator.parameter_sort == pen_eval::typed_families::ParamSort::Type
                && generator.kernel_type == pen_type::elaborate::KernelTy::Type
        })
        .map(|generator| generator.support_parameter)
        .collect::<BTreeSet<_>>();
    let enumerated_generator_subsets = subsets(&older_parameter_universe)
        .into_iter()
        .map(|generator_parameters| {
            let set = generator_parameters
                .iter()
                .copied()
                .collect::<BTreeSet<_>>();
            let covers_every_first_realizer_dependency = free_set.is_subset(&set);
            let contains_no_unused_generator = set.is_subset(&free_set);
            let consists_only_of_independent_type_parameters =
                set.is_subset(&independent_parameters);
            let admissible_support = covers_every_first_realizer_dependency
                && contains_no_unused_generator
                && consists_only_of_independent_type_parameters;
            let mut decision = SupportGeneratorSubsetDecisionV6 {
                generator_parameters,
                covers_every_first_realizer_dependency,
                contains_no_unused_generator,
                consists_only_of_independent_type_parameters,
                admissible_support,
                decision_hash: String::new(),
            };
            decision.decision_hash = tagged_hash("generator-subset-decision", &decision);
            decision
        })
        .collect::<Vec<_>>();
    let admissible = enumerated_generator_subsets
        .iter()
        .filter(|decision| decision.admissible_support)
        .map(|decision| decision.generator_parameters.clone())
        .collect::<Vec<_>>();
    let exact_one_admissible_generator_subset = admissible.len() == 1;
    let admissible_generator_subset = admissible.first().cloned().unwrap_or_default();
    let every_generator_classifier_dependency_recomputed_empty = analysis
        .maximal_independent_generators
        .iter()
        .all(|generator| {
            analysis
                .support_motives
                .get((generator.support_parameter - 1) as usize)
                .is_some_and(|motive| motive_dependencies(motive).is_empty())
        });
    let dependent_witness_is_unique_least_fresh_node = analysis.least_fresh_support_parameter
        == admissible_generator_subset.len() as u32 + 1
        && analysis.support_motives.len() == admissible_generator_subset.len() + 1
        && analysis.interface_realizers.get(1).is_some_and(|realizer| {
            realizer.expression == Expr::Var(analysis.least_fresh_support_parameter)
                && realizer.realized_by_fresh_support_hypothesis
        });
    let dependent_motive_is_exact_instantiated_classifier =
        analysis.support_motives.last().is_some_and(|motive| {
            matches!(
                motive,
                DependentContextMotive::ElementOfApplicationHead { head }
                    if pen_type::elaborate::KernelTy::El(head.clone())
                        == analysis.required_second_type_after_first_realizer
            )
        });
    let interface_slot_dependencies = analysis
        .interface_realizers
        .iter()
        .map(|realizer| {
            (
                realizer.interface_slot,
                realizer
                    .expression
                    .var_refs()
                    .into_iter()
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    let factorization_decisions = permutations(&admissible_generator_subset)
        .into_iter()
        .map(|generator_renaming| {
            let is_permutation_of_exact_generators =
                generator_renaming.iter().copied().collect::<BTreeSet<_>>()
                    == admissible_generator_subset.iter().copied().collect();
            let preserves_declaration_order = generator_renaming == admissible_generator_subset;
            let preserves_first_realizer_dependency_graph = is_permutation_of_exact_generators
                && generator_renaming.iter().copied().collect::<BTreeSet<_>>() == free_set;
            let canonical_factorization = is_permutation_of_exact_generators
                && preserves_declaration_order
                && preserves_first_realizer_dependency_graph;
            let mut decision = SupportOrderFactorizationDecisionV6 {
                generator_renaming,
                is_permutation_of_exact_generators,
                preserves_declaration_order,
                preserves_first_realizer_dependency_graph,
                canonical_factorization,
                decision_hash: String::new(),
            };
            decision.decision_hash = tagged_hash("order-factorization-decision", &decision);
            decision
        })
        .collect::<Vec<_>>();
    let exact_one_order_preserving_factorization = factorization_decisions
        .iter()
        .filter(|decision| decision.canonical_factorization)
        .count()
        == 1;
    let expected_subset_count = 1_usize << older_parameter_universe.len();
    let expected_factorization_count = (1..=admissible_generator_subset.len()).product::<usize>();
    let canonical_dependency_analysis_candidate_grammar_exhaustive =
        enumerated_generator_subsets.len() == expected_subset_count
            && enumerated_generator_subsets
                .iter()
                .map(|decision| decision.generator_parameters.clone())
                .collect::<BTreeSet<_>>()
                .len()
                == expected_subset_count
            && factorization_decisions.len() == expected_factorization_count
            && factorization_decisions
                .iter()
                .map(|decision| decision.generator_renaming.clone())
                .collect::<BTreeSet<_>>()
                .len()
                == expected_factorization_count;
    let every_candidate_in_closed_canonical_dependency_analysis_factors_through_comprehension =
        canonical_dependency_analysis_candidate_grammar_exhaustive
            && analysis.no_padding_or_unused_generator
            && exact_one_admissible_generator_subset
            && admissible_generator_subset == analysis.first_realizer_free_parameters
            && every_generator_classifier_dependency_recomputed_empty
            && dependent_witness_is_unique_least_fresh_node
            && dependent_motive_is_exact_instantiated_classifier
            && interface_slot_dependencies
                == vec![
                    (1, analysis.first_realizer_free_parameters.clone()),
                    (2, vec![analysis.least_fresh_support_parameter]),
                ]
            && exact_one_order_preserving_factorization;
    let selected_by_specialization_success = !analysis.analysis_precedes_specialization
        || !analysis.support_selected_without_derivation_success
        || analysis
            .analysis_trace
            .iter()
            .any(|step| step.selected_by_derivation_success);
    let proof_scope = "Finite graph-normal-form theorem over the closed canonical dependency-analysis grammar for the exact sealed instance: all subsets of the sealed older-parameter universe, all declaration-order permutations of the unique minimal generator set, and the uniquely typed least-fresh witness. It does not quantify arbitrary hand-shaped ambient contexts outside that grammar.".to_owned();
    let proved =
        every_candidate_in_closed_canonical_dependency_analysis_factors_through_comprehension
            && !selected_by_specialization_success;
    let mut theorem = SupportGraphNormalFormV6 {
        theorem_id: "T-SC-GNF1-exact-support-graph-normal-form".to_owned(),
        bound_analysis_hash: analysis.analysis_hash.clone(),
        sealed_input_hash: analysis.sealed_input_hash.clone(),
        older_parameter_universe,
        first_realizer_free_parameters: analysis.first_realizer_free_parameters.clone(),
        enumerated_generator_subsets,
        exact_one_admissible_generator_subset,
        admissible_generator_subset,
        every_generator_classifier_dependency_recomputed_empty,
        dependent_witness_is_unique_least_fresh_node,
        dependent_motive_is_exact_instantiated_classifier,
        interface_slot_dependencies,
        factorization_decisions,
        exact_one_order_preserving_factorization,
        canonical_dependency_analysis_candidate_grammar_exhaustive,
        every_candidate_in_closed_canonical_dependency_analysis_factors_through_comprehension,
        selected_by_specialization_success,
        proof_scope,
        proved,
        derivation_hash: String::new(),
    };
    theorem.derivation_hash = tagged_hash("support-graph-normal-form", &theorem);
    if theorem.proved {
        Ok(theorem)
    } else {
        Err(SupportComprehensionHardeningV6Error::Graph(
            "exact live graph did not have one canonical comprehension factorization".to_owned(),
        ))
    }
}

fn zero_mint_capability(
    derivation: &SupportComprehensionDerivationV5,
) -> Result<SupportZeroMintCapabilityV6, SupportComprehensionHardeningV6Error> {
    let forbidden = SupportInterfaceCapabilityV6::forbidden();
    let mut inputs = derivation
        .support_probe_declaration
        .hypotheses
        .iter()
        .map(|hypothesis| {
            let operation = match &hypothesis.motive {
                DependentContextMotive::Independent { .. } => {
                    SupportInterfaceOperationV6::IndependentTypeHypothesis
                }
                DependentContextMotive::ElementOfApplicationHead { .. } => {
                    SupportInterfaceOperationV6::DependentElementHypothesis
                }
                DependentContextMotive::OpaquePriorClause { .. } => {
                    SupportInterfaceOperationV6::OpaquePriorClauseHypothesis
                }
            };
            (
                format!("hypothesis-{}", hypothesis.parameter),
                operation,
                hypothesis.declaration_hash.clone(),
            )
        })
        .collect::<Vec<_>>();
    inputs.extend(
        derivation
            .canonical_analysis
            .interface_realizers
            .iter()
            .map(|realizer| {
                (
                    format!("realizer-{}", realizer.interface_slot),
                    if realizer.realized_by_fresh_support_hypothesis {
                        SupportInterfaceOperationV6::FreshHypothesisReference
                    } else {
                        SupportInterfaceOperationV6::CompositeInterfaceReference
                    },
                    realizer.realizer_hash.clone(),
                )
            }),
    );
    let expected_row_bindings = inputs.clone();
    let rows = inputs
        .into_iter()
        .map(|(row_id, operation, bound_evidence_hash)| {
            let capabilities = operation_capabilities(operation);
            let forbidden_capabilities = capabilities
                .iter()
                .filter(|capability| forbidden.contains(capability))
                .copied()
                .collect::<Vec<_>>();
            let no_mint = forbidden_capabilities.is_empty();
            let mut row = SupportNoMintCapabilityRowV6 {
                row_id,
                operation,
                bound_evidence_hash,
                capabilities,
                forbidden_capabilities,
                no_mint,
                derivation_hash: String::new(),
            };
            row.derivation_hash = tagged_hash("support-no-mint-row", &row);
            row
        })
        .collect::<Vec<_>>();
    let exact_hypothesis_and_realizer_surface = rows.len()
        == derivation.support_probe_declaration.hypotheses.len()
            + derivation.canonical_analysis.interface_realizers.len()
        && rows
            .iter()
            .map(|row| row.row_id.as_str())
            .collect::<BTreeSet<_>>()
            .len()
            == rows.len();
    let exact_typed_operation_bindings = rows
        .iter()
        .map(|row| {
            (
                row.row_id.clone(),
                row.operation,
                row.bound_evidence_hash.clone(),
            )
        })
        .eq(expected_row_bindings.iter().cloned())
        && rows.iter().all(|row| !row.bound_evidence_hash.is_empty())
        && derivation.support_probe_declaration.hypotheses.len()
            == derivation.support_probe_declaration.declared_arity as usize
        && derivation
            .support_probe_declaration
            .hypotheses
            .iter()
            .enumerate()
            .all(|(index, hypothesis)| hypothesis.parameter == index as u32 + 1)
        && derivation
            .canonical_analysis
            .interface_realizers
            .iter()
            .enumerate()
            .all(|(index, realizer)| realizer.interface_slot == index as u32 + 1);
    let every_capability_derived_from_closed_operation_enum = rows
        .iter()
        .all(|row| row.capabilities == operation_capabilities(row.operation));
    let no_forbidden_capability = rows
        .iter()
        .all(|row| row.no_mint && row.forbidden_capabilities.is_empty());
    let derived_kappa_charge = rows
        .iter()
        .filter(|row| {
            row.capabilities
                .contains(&SupportInterfaceCapabilityV6::KappaCredit)
        })
        .count() as u32;
    let derived_nu_charge = rows
        .iter()
        .filter(|row| {
            row.capabilities
                .contains(&SupportInterfaceCapabilityV6::NuCredit)
        })
        .count() as u32;
    let derived_anchor_mints = rows
        .iter()
        .filter(|row| {
            row.capabilities
                .contains(&SupportInterfaceCapabilityV6::AnchorMint)
        })
        .count() as u32;
    let derived_demand_orbit_mints = rows
        .iter()
        .filter(|row| {
            row.capabilities
                .contains(&SupportInterfaceCapabilityV6::DemandOrbitMint)
        })
        .count() as u32;
    let projection_agrees_with_derived_zero = derived_kappa_charge == 0
        && derived_nu_charge == 0
        && derived_anchor_mints == 0
        && derived_demand_orbit_mints == 0
        && derivation.support_parameters_kappa_charge == derived_kappa_charge
        && derivation.support_parameters_nu_charge == derived_nu_charge
        && derivation.support_parameters_anchors_minted == derived_anchor_mints
        && derivation.support_parameters_demand_orbits_minted == derived_demand_orbit_mints
        && derivation.realizers_kappa_charge == derived_kappa_charge
        && derivation.realizers_nu_charge == derived_nu_charge
        && derivation.realizers_anchors_minted == derived_anchor_mints
        && derivation.realizers_demand_orbits_minted == derived_demand_orbit_mints;
    let proof_scope = "Typed capability theorem for the exact support registration surface recorded by the replayed base derivation: every declared hypothesis and every interface realizer is bound to its declaration/realizer hash and classified by the closed operation enum. Together with the base zero-accounting projection, this excludes minting on this registration path; it is not a whole-program effect theorem for unrelated call paths.".to_owned();
    let proved = exact_hypothesis_and_realizer_surface
        && exact_typed_operation_bindings
        && every_capability_derived_from_closed_operation_enum
        && no_forbidden_capability
        && projection_agrees_with_derived_zero;
    let mut theorem = SupportZeroMintCapabilityV6 {
        theorem_id: "T-SC-ZM1-support-reference-zero-mint-capability".to_owned(),
        bound_support_derivation_hash: derivation.derivation_hash.clone(),
        rows,
        expected_row_bindings,
        exact_hypothesis_and_realizer_surface,
        exact_typed_operation_bindings,
        every_capability_derived_from_closed_operation_enum,
        no_forbidden_capability,
        derived_kappa_charge,
        derived_nu_charge,
        derived_anchor_mints,
        derived_demand_orbit_mints,
        projection_agrees_with_derived_zero,
        proof_scope,
        proved,
        derivation_hash: String::new(),
    };
    theorem.derivation_hash = tagged_hash("support-zero-mint-capability", &theorem);
    if theorem.proved {
        Ok(theorem)
    } else {
        Err(SupportComprehensionHardeningV6Error::ZeroMint(
            "hypothesis/reference capability algebra did not derive exact zero accounting"
                .to_owned(),
        ))
    }
}

fn token_digest(token: &SupportComprehensionHardeningV6Token) -> String {
    let mut projection = token.clone();
    projection.derivation_hash.clear();
    tagged_hash("support-hardening-token", &projection)
}

fn issue_support_comprehension_hardening_core_v6(
    derivation: &SupportComprehensionDerivationV5,
) -> Result<SupportComprehensionHardeningV6Token, SupportComprehensionHardeningV6Error> {
    let base_derivation_replayed = true;
    let graph_normal_form = graph_normal_form(&derivation.canonical_analysis)?;
    let zero_mint_capability = zero_mint_capability(derivation)?;
    let proved =
        base_derivation_replayed && graph_normal_form.proved && zero_mint_capability.proved;
    let mut token = SupportComprehensionHardeningV6Token {
        schema: SUPPORT_COMPREHENSION_HARDENING_V6_SCHEMA.to_owned(),
        date: SUPPORT_COMPREHENSION_HARDENING_V6_DATE.to_owned(),
        bound_support_derivation_hash: derivation.derivation_hash.clone(),
        base_derivation_replayed,
        graph_normal_form,
        zero_mint_capability,
        proved,
        derivation_hash: String::new(),
    };
    token.derivation_hash = token_digest(&token);
    Ok(token)
}

/// Harden one exact live support-comprehension row.  The base derivation is
/// first reissued from the supplied signature and complete A3 row; the graph
/// normal form and no-mint capability theorems are then bound to that exact
/// base hash.
pub fn issue_support_comprehension_hardening_for_row_v6(
    signature: &SealedSignature,
    visible_library: u32,
    instance: &A3TypedDemandInstance,
    scheme: &A3TypedDemandScheme,
    older: &A3TypedClauseSource,
    newest: &A3TypedClauseSource,
    derivation: &SupportComprehensionDerivationV5,
) -> Result<SupportComprehensionHardeningV6Token, SupportComprehensionHardeningV6Error> {
    replay_support_comprehension_derivation_for_row_v5(
        signature,
        visible_library,
        instance,
        scheme,
        older,
        newest,
        derivation,
    )
    .map_err(|error| SupportComprehensionHardeningV6Error::Base(error.to_string()))?;
    issue_support_comprehension_hardening_core_v6(derivation)
}

/// Preserve the Genesis v6 API.
pub fn issue_support_comprehension_hardening_v6(
    derivation: &SupportComprehensionDerivationV5,
) -> Result<SupportComprehensionHardeningV6Token, SupportComprehensionHardeningV6Error> {
    replay_support_comprehension_derivation_v5(derivation)
        .map_err(|error| SupportComprehensionHardeningV6Error::Base(error.to_string()))?;
    issue_support_comprehension_hardening_core_v6(derivation)
}

/// Replay exact-row hardening by reissuing both the base v5 theorem and the
/// complete v6 token under the supplied signature.
pub fn replay_support_comprehension_hardening_for_row_v6(
    signature: &SealedSignature,
    visible_library: u32,
    instance: &A3TypedDemandInstance,
    scheme: &A3TypedDemandScheme,
    older: &A3TypedClauseSource,
    newest: &A3TypedClauseSource,
    derivation: &SupportComprehensionDerivationV5,
    claimed: &SupportComprehensionHardeningV6Token,
) -> Result<(), SupportComprehensionHardeningV6Error> {
    replay_support_comprehension_derivation_for_row_v5(
        signature,
        visible_library,
        instance,
        scheme,
        older,
        newest,
        derivation,
    )
    .map_err(|error| SupportComprehensionHardeningV6Error::Base(error.to_string()))?;
    if claimed.derivation_hash != token_digest(claimed) {
        return Err(SupportComprehensionHardeningV6Error::ReplayMismatch);
    }
    let expected = issue_support_comprehension_hardening_for_row_v6(
        signature,
        visible_library,
        instance,
        scheme,
        older,
        newest,
        derivation,
    )?;
    if expected == *claimed {
        Ok(())
    } else {
        Err(SupportComprehensionHardeningV6Error::ReplayMismatch)
    }
}

pub fn replay_support_comprehension_hardening_v6(
    derivation: &SupportComprehensionDerivationV5,
    claimed: &SupportComprehensionHardeningV6Token,
) -> Result<(), SupportComprehensionHardeningV6Error> {
    replay_support_comprehension_derivation_v5(derivation)
        .map_err(|error| SupportComprehensionHardeningV6Error::Base(error.to_string()))?;
    if claimed.derivation_hash != token_digest(claimed) {
        return Err(SupportComprehensionHardeningV6Error::ReplayMismatch);
    }
    let expected = issue_support_comprehension_hardening_v6(derivation)?;
    if expected == *claimed {
        Ok(())
    } else {
        Err(SupportComprehensionHardeningV6Error::ReplayMismatch)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support_comprehension_v5::issue_support_comprehension_derivation_v5;

    #[test]
    fn exact_graph_has_one_ordered_comprehension_and_zero_mint_capability() {
        let derivation = issue_support_comprehension_derivation_v5().expect("support derivation");
        let token = issue_support_comprehension_hardening_v6(&derivation).expect("hardening");
        assert!(token.proved);
        assert_eq!(
            token
                .graph_normal_form
                .enumerated_generator_subsets
                .iter()
                .filter(|decision| decision.admissible_support)
                .count(),
            1
        );
        assert!(token.zero_mint_capability.no_forbidden_capability);
        replay_support_comprehension_hardening_v6(&derivation, &token).expect("replay");
    }

    #[test]
    fn forbidden_mint_capability_and_fake_factorization_fail_replay() {
        let derivation = issue_support_comprehension_derivation_v5().expect("support derivation");
        let token = issue_support_comprehension_hardening_v6(&derivation).expect("hardening");
        let mut mint = token.clone();
        mint.zero_mint_capability.rows[0]
            .capabilities
            .push(SupportInterfaceCapabilityV6::NuCredit);
        mint.derivation_hash = token_digest(&mint);
        assert!(replay_support_comprehension_hardening_v6(&derivation, &mint).is_err());

        let mut factorization = token;
        factorization.graph_normal_form.factorization_decisions[0].canonical_factorization = false;
        factorization.derivation_hash = token_digest(&factorization);
        assert!(replay_support_comprehension_hardening_v6(&derivation, &factorization).is_err());
    }

    #[test]
    fn stale_or_mutated_base_derivation_cannot_issue_or_replay_hardening() {
        let derivation = issue_support_comprehension_derivation_v5().expect("support derivation");
        let token = issue_support_comprehension_hardening_v6(&derivation).expect("hardening");
        let mut stale = derivation;
        stale.specialized_expression = Expr::Var(999);
        assert!(issue_support_comprehension_hardening_v6(&stale).is_err());
        assert!(replay_support_comprehension_hardening_v6(&stale, &token).is_err());
    }

    #[test]
    fn exact_row_hardening_matches_genesis_and_rejects_row_mutation() {
        let signature = SealedSignature::genesis_del_h15();
        let derivation = issue_support_comprehension_derivation_v5().expect("support derivation");
        let analysis = &derivation.canonical_analysis;
        let generic = issue_support_comprehension_hardening_for_row_v6(
            &signature,
            15,
            &analysis.sealed_instance,
            &analysis.sealed_scheme,
            &analysis.older_source,
            &analysis.newest_source,
            &derivation,
        )
        .expect("generic hardening");
        let legacy =
            issue_support_comprehension_hardening_v6(&derivation).expect("legacy hardening");
        assert_eq!(generic, legacy);
        assert!(generic.proved);
        assert_eq!(
            derivation.specialization.signature_digest,
            signature.digest()
        );
        replay_support_comprehension_hardening_for_row_v6(
            &signature,
            15,
            &analysis.sealed_instance,
            &analysis.sealed_scheme,
            &analysis.older_source,
            &analysis.newest_source,
            &derivation,
            &generic,
        )
        .expect("generic hardening replay");

        let mut forged_scheme = analysis.sealed_scheme.clone();
        forged_scheme.formation_derivation_hash.push_str("-forged");
        assert!(
            issue_support_comprehension_hardening_for_row_v6(
                &signature,
                15,
                &analysis.sealed_instance,
                &forged_scheme,
                &analysis.older_source,
                &analysis.newest_source,
                &derivation,
            )
            .is_err()
        );
    }

    #[test]
    fn legacy_v6_projection_remains_byte_lineage_compatible() {
        let derivation = issue_support_comprehension_derivation_v5().expect("support derivation");
        let hardening =
            issue_support_comprehension_hardening_v6(&derivation).expect("support hardening");
        let archived: serde_json::Value = serde_json::from_slice(include_bytes!(
            "../../../docs/support_comprehension_chronological_v5.json"
        ))
        .expect("archived v5 JSON");
        assert_eq!(
            serde_json::to_value(&hardening).expect("current v6 projection"),
            archived["support_comprehension_hardening"],
            "generic exact-row API must not alter the immutable v6 projection"
        );
    }
}
