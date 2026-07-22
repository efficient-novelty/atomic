//! Fail-closed E-5 successor audit for the proposed full historical A3 join.
//!
//! The audit deliberately separates three things that earlier attempts
//! conflated: a finite candidate generator, typed semantic demand outputs,
//! and membership of those outputs in `D(B15)`.  Only the first exists for
//! the newly generated unary/chronological/higher inventory.  The exact J3
//! predecessor remains a positive 64-instance/8-family theorem; the new
//! generator's 64 singleton direct orbits contradict that quotient, so this
//! successor cannot issue F1, semantic O(16), Theorem 12, or the bridge.

use crate::e5_demand_projection::{E5DemandProjectionCertificate, E5MembershipDisposition};
use pen_core::hash::blake3_hex;
use pen_eval::a3_demand_grammar::{
    A3ChronologicalInterfaceMode, A3DemandOutputType, A3HistoricalDemandGrammar,
    A3HistoricalWindow, A3RuleConstructor, UntrustedCallerDemandInventory,
    audit_a3_caller_inventory_noninterference, issue_historical_a3_demand_grammar,
    replay_historical_a3_demand_grammar,
};
use pen_eval::debt_guard::directive_debt_timeline;
use pen_type::elaborate::SealedSignature;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const E5_A3_SUCCESSOR_SCHEMA: &str = "schema2-e5-a3-successor-gap-v1";
pub const E5_A3_SUCCESSOR_DATE: &str = "2026-07-21";
pub const A3_OUTPUT_TERM_GAP: &str = "A3_DEMAND_OUTPUT_TERM_AND_TYPING_JUDGMENT_UNDEFINED";
pub const A3_EXHAUSTIVENESS_GAP: &str = "A3_RULE_CONSTRUCTOR_INVENTORY_EXHAUSTIVENESS_UNPROVED";
pub const A3_QUOTIENT_GAP: &str =
    "A3_DIRECT_J3_NATURAL_FAMILY_QUOTIENT_REGRESSION_64_SINGLETONS_VS_8_ORBITS";
pub const A3_D_MEMBERSHIP_GAP: &str = "A3_OUTPUT_LEVEL_D_B15_MEMBERSHIP_NOT_EXECUTABLE";
pub const A3_STAGE3_TRANSPORT_GAP: &str = "A3_STAGE3_TO_STAGE4_INSTANCE_ORBIT_TRANSPORT_UNPROVED";

const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const J3_PREDECESSOR_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_e5_demand_projection_v1.json");
const PLAN_BYTES: &[u8] = include_bytes!("../../../docs/agent_e_schema2_plan.md");
const SPEC_BYTES: &[u8] = include_bytes!("../../../docs/step_15_completion_open_problem.md");
const THEOREM_BYTES: &[u8] = include_bytes!("../../../docs/t1_result.md");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("e5_a3_successor.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3RestrictedJ3Audit {
    pub bound_archive_loaded: bool,
    pub archive_result_digest: String,
    pub direct_instance_count: usize,
    pub natural_family_orbit_count: usize,
    pub every_instance_typed: bool,
    pub every_instance_derivability_decided: bool,
    pub every_orbit_derivable: bool,
    pub restricted_f1_excluded: bool,
    pub exact_positive_conclusion: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3Stage16GeneratorAudit {
    pub orientation: String,
    pub typed_source_count: usize,
    pub unary_raw_seed_count: usize,
    pub unary_promoted_metadata_instance_count: usize,
    pub chronological_raw_seed_count: usize,
    pub direct_chronological_metadata_instance_count: usize,
    pub pointwise_chronological_metadata_instance_count: usize,
    pub chronological_promoted_metadata_instance_count: usize,
    pub higher_raw_seed_count: usize,
    pub higher_promoted_metadata_instance_count: usize,
    pub higher_rejection_reason: Option<String>,
    pub completion_metadata_instance_count: usize,
    pub total_metadata_instance_count: usize,
    pub total_candidate_orbit_count: usize,
    pub singleton_candidate_orbit_count: usize,
    pub unary_candidate_orbit_count: usize,
    pub direct_candidate_orbit_count: usize,
    pub pointwise_candidate_orbit_count: usize,
    pub distinct_newest_j3_source_count_in_direct_instances: usize,
    pub direct_predecessor_orbit_count: usize,
    pub direct_quotient_agrees_with_predecessor: bool,
    pub uniform_specializations_not_multiplied: bool,
    pub source_occurrence_formation_replayed: bool,
    pub demand_output_terms_constructed: bool,
    pub demand_output_terms_kernel_typed: bool,
    pub d_membership_decided: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3ProjectionRow {
    pub stage: u32,
    pub independently_replayed_completion_constructors: Vec<String>,
    pub coarse_regression_packages: Vec<String>,
    pub projected_focus: Option<String>,
    pub pre_structural_band: bool,
    pub demand_precedes_jurisdiction: bool,
    pub labels_match_as_regression: bool,
    pub semantic_output_orbit_joined: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3HistoricalProjectionAudit {
    pub window_count: usize,
    pub all_windows_finite_by_enumeration: bool,
    pub raw_snapshot_completion_predicates_count_blind: bool,
    pub empty_and_forged_caller_inventories_byte_identical: bool,
    pub rows: Vec<E5A3ProjectionRow>,
    pub coarse_ladder_labels_reproduced: bool,
    pub stage3_wrinkle_reproduced_as_regression: bool,
    pub stage3_instance_orbit_transport_proved: bool,
    pub output_level_focus_projection_proved: bool,
    pub weakening_stable_instance_discharge_proved: bool,
    pub locality_and_expiration_complete: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3DefinitionBoundaryAudit {
    pub generator_candidate_replays: bool,
    pub rule_constructor_inventory_closed_in_source: bool,
    pub rule_constructor_inventory_exhaustiveness_proved: bool,
    pub each_scheme_has_a_metadata_output_description: bool,
    pub each_scheme_has_a_constructed_semantic_output_term: bool,
    pub each_semantic_output_term_has_a_kernel_typing_derivation: bool,
    pub source_reflexivity_accepted_as_unary_action_proof: bool,
    pub motive_parametric_theorem_used_without_a_source_closure_derivation: bool,
    pub full_a3_identification_proved: bool,
    pub named_gaps: Vec<String>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3F1Audit {
    pub full_a3_f1_executable: bool,
    pub demanded_but_underdetermined_instance: Option<String>,
    pub f1_triggered: bool,
    pub f1_excluded: bool,
    pub semantic_o16_empty: Option<bool>,
    pub theorem12_instance_granularity_proved: bool,
    pub theorem12_refuted: bool,
    pub t_bf2_authorized: bool,
    pub bridge_authorized: bool,
    pub status: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3SuccessorCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<E5A3SourceBinding>,
    pub restricted_j3: E5A3RestrictedJ3Audit,
    pub stage16: E5A3Stage16GeneratorAudit,
    pub historical_projection: E5A3HistoricalProjectionAudit,
    pub definition_boundary: E5A3DefinitionBoundaryAudit,
    pub f1: E5A3F1Audit,
    pub e5_successor_attempted: bool,
    pub e5_complete: bool,
    pub semantic_o16_certificate_issued: bool,
    pub t_bf2_executed: bool,
    pub bridge_executed: bool,
    pub bar_free_law_adopted: bool,
    pub halt_claim_issued: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5A3SuccessorReplay {
    pub valid: bool,
    pub metadata_instance_count: usize,
    pub candidate_orbit_count: usize,
    pub direct_quotient_regression_found: bool,
    pub full_a3_identification_proved: bool,
    pub f1_executable: bool,
    pub semantic_o16_empty: Option<bool>,
    pub e5_complete: bool,
    pub bridge_authorized: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum E5A3SuccessorError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("A3 generator failed: {0}")]
    Generator(String),
    #[error("A3 audit invariant failed: {0}")]
    Invariant(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(E5_A3_SUCCESSOR_SCHEMA, domain, value))
        .expect("E-5 A3 audit data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<E5A3SourceBinding> {
    [
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "candidate_generator_and_explicit_fail_closed_boundary",
            A3_SOURCE_BYTES,
        ),
        (
            "docs/schema2_e5_demand_projection_v1.json",
            "exact_restricted_j3_predecessor",
            J3_PREDECESSOR_BYTES,
        ),
        (
            "docs/agent_e_schema2_plan.md",
            "e5_acceptance_and_falsifiers",
            PLAN_BYTES,
        ),
        (
            "docs/step_15_completion_open_problem.md",
            "operational_schema_and_output_typing_requirements",
            SPEC_BYTES,
        ),
        (
            "docs/t1_result.md",
            "theorem12_and_f1_granularity_boundary",
            THEOREM_BYTES,
        ),
        (
            "crates/pen-search/src/e5_a3_successor.rs",
            "this_fail_closed_successor_audit",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| E5A3SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn certificate_digest(certificate: &E5A3SuccessorCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("e5-a3-successor-gap-certificate", &projection)
}

fn parse_predecessor() -> Result<E5DemandProjectionCertificate, E5A3SuccessorError> {
    serde_json::from_slice(J3_PREDECESSOR_BYTES)
        .map_err(|error| E5A3SuccessorError::Json(error.to_string()))
}

fn restricted_j3_audit(
    predecessor: &E5DemandProjectionCertificate,
) -> Result<E5A3RestrictedJ3Audit, E5A3SuccessorError> {
    let every_instance_typed = predecessor.restricted_window.every_instance_typed;
    let every_instance_derivability_decided =
        predecessor
            .restricted_window
            .instances
            .iter()
            .all(|instance| {
                matches!(
                    instance.disposition,
                    E5MembershipDisposition::Derivable { .. }
                )
            });
    let every_orbit_derivable = predecessor
        .restricted_window
        .orbits
        .iter()
        .all(|orbit| orbit.every_instance_derivable);
    if predecessor.restricted_window.raw_instance_count != 64
        || predecessor.restricted_window.orbit_count != 8
        || !every_instance_typed
        || !every_instance_derivability_decided
        || !every_orbit_derivable
        || !predecessor.f1.restricted_j3_f1_excluded
    {
        return Err(E5A3SuccessorError::Prerequisite(
            "bound restricted J3 archive no longer carries the exact positive theorem".to_owned(),
        ));
    }
    Ok(E5A3RestrictedJ3Audit {
        bound_archive_loaded: true,
        archive_result_digest: predecessor.result_digest.clone(),
        direct_instance_count: predecessor.restricted_window.raw_instance_count,
        natural_family_orbit_count: predecessor.restricted_window.orbit_count,
        every_instance_typed,
        every_instance_derivability_decided,
        every_orbit_derivable,
        restricted_f1_excluded: predecessor.f1.restricted_j3_f1_excluded,
        exact_positive_conclusion: "The 64 direct Type specializations are typed and D(B15)-derivable, and quotient to eight J3 natural-family orbits; no stronger A3 claim is inherited.".to_owned(),
    })
}

fn seed_counts(
    window: &A3HistoricalWindow,
    rule: A3RuleConstructor,
) -> (usize, usize, Option<String>) {
    window
        .seed_dispositions
        .iter()
        .find(|row| row.rule_constructor == rule)
        .map(|row| {
            (
                row.raw_seed_count,
                row.generated_typed_instance_count,
                row.rejection_reason.map(|reason| format!("{reason:?}")),
            )
        })
        .unwrap_or((0, 0, Some("missing_seed_disposition".to_owned())))
}

fn stage16_generator_audit(
    grammar: &A3HistoricalDemandGrammar,
    predecessor: &E5A3RestrictedJ3Audit,
) -> Result<E5A3Stage16GeneratorAudit, E5A3SuccessorError> {
    let window = grammar
        .windows
        .iter()
        .find(|window| window.stage == 16)
        .ok_or_else(|| E5A3SuccessorError::Invariant("missing Stage-16 window".to_owned()))?;
    let schemes = window
        .schemes
        .iter()
        .map(|scheme| (scheme.scheme_id.as_str(), scheme))
        .collect::<BTreeMap<_, _>>();
    let sources = window
        .typed_sources
        .iter()
        .map(|source| (source.anchor_id.as_str(), source))
        .collect::<BTreeMap<_, _>>();
    let mut unary = 0;
    let mut direct = 0;
    let mut pointwise = 0;
    let mut higher = 0;
    let mut completion = 0;
    let mut direct_newest_sources = BTreeSet::new();
    for instance in &window.instances {
        let scheme = schemes.get(instance.scheme_id.as_str()).ok_or_else(|| {
            E5A3SuccessorError::Invariant(format!(
                "instance {} has no scheme",
                instance.instance_id
            ))
        })?;
        match (&scheme.rule_constructor, &scheme.required_output) {
            (A3RuleConstructor::UnaryAction, _) => unary += 1,
            (
                A3RuleConstructor::ChronologicalComparison,
                A3DemandOutputType::ChronologicalInteraction {
                    interface_mode: A3ChronologicalInterfaceMode::DirectType,
                    ..
                },
            ) => {
                direct += 1;
                if let Some(newest_anchor) = instance.source_anchor_ids.get(1) {
                    let newest = sources.get(newest_anchor.as_str()).ok_or_else(|| {
                        E5A3SuccessorError::Invariant(format!(
                            "direct instance has absent newest source {newest_anchor}"
                        ))
                    })?;
                    direct_newest_sources.insert((newest.step, newest.clause_index));
                }
            }
            (
                A3RuleConstructor::ChronologicalComparison,
                A3DemandOutputType::ChronologicalInteraction {
                    interface_mode: A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { .. },
                    ..
                },
            ) => pointwise += 1,
            (A3RuleConstructor::HigherOpenBoxReduction, _) => higher += 1,
            (A3RuleConstructor::StructuralCompletionHole, _) => completion += 1,
            _ => {
                return Err(E5A3SuccessorError::Invariant(format!(
                    "rule/output mismatch in scheme {}",
                    scheme.scheme_id
                )));
            }
        }
    }
    let orbit_kind = |orbit: &pen_eval::a3_demand_grammar::A3DemandOrbit| {
        schemes
            .get(orbit.scheme_id.as_str())
            .map(|scheme| (&scheme.rule_constructor, &scheme.required_output))
    };
    let unary_candidate_orbit_count = window
        .orbits
        .iter()
        .filter(|orbit| {
            orbit_kind(orbit).is_some_and(|(rule, _)| *rule == A3RuleConstructor::UnaryAction)
        })
        .count();
    let direct_candidate_orbit_count = window
        .orbits
        .iter()
        .filter(|orbit| {
            orbit_kind(orbit).is_some_and(|(rule, output)| {
                *rule == A3RuleConstructor::ChronologicalComparison
                    && matches!(
                        output,
                        A3DemandOutputType::ChronologicalInteraction {
                            interface_mode: A3ChronologicalInterfaceMode::DirectType,
                            ..
                        }
                    )
            })
        })
        .count();
    let pointwise_candidate_orbit_count = window
        .orbits
        .iter()
        .filter(|orbit| {
            orbit_kind(orbit).is_some_and(|(rule, output)| {
                *rule == A3RuleConstructor::ChronologicalComparison
                    && matches!(
                        output,
                        A3DemandOutputType::ChronologicalInteraction {
                            interface_mode:
                                A3ChronologicalInterfaceMode::PointwiseTypeValuedFunction { .. },
                            ..
                        }
                    )
            })
        })
        .count();
    let singleton_candidate_orbit_count = window
        .orbits
        .iter()
        .filter(|orbit| orbit.member_instance_ids.len() == 1)
        .count();
    let (unary_raw_seed_count, unary_generated, _) =
        seed_counts(window, A3RuleConstructor::UnaryAction);
    let (chronological_raw_seed_count, chronological_generated, _) =
        seed_counts(window, A3RuleConstructor::ChronologicalComparison);
    let (higher_raw_seed_count, higher_generated, higher_rejection_reason) =
        seed_counts(window, A3RuleConstructor::HigherOpenBoxReduction);
    let (_, completion_generated, _) =
        seed_counts(window, A3RuleConstructor::StructuralCompletionHole);
    let direct_quotient_agrees_with_predecessor =
        direct_candidate_orbit_count == predecessor.natural_family_orbit_count;
    let uniform_specializations_not_multiplied = direct_quotient_agrees_with_predecessor
        && direct_newest_sources.len() == predecessor.natural_family_orbit_count;
    let derivation_hash = tagged_hash(
        "stage16-candidate-generator-audit",
        &serde_json::json!({
            "stage": window.stage,
            "window_derivation": window.window_derivation_hash,
            "unary": unary,
            "direct": direct,
            "pointwise": pointwise,
            "higher": higher,
            "completion": completion,
            "instances": window.instances.len(),
            "orbits": window.orbits.len(),
            "singleton_orbits": singleton_candidate_orbit_count,
            "unary_orbits": unary_candidate_orbit_count,
            "direct_orbits": direct_candidate_orbit_count,
            "pointwise_orbits": pointwise_candidate_orbit_count,
            "direct_newest_sources": direct_newest_sources,
            "predecessor_orbits": predecessor.natural_family_orbit_count,
            "quotient_agrees": direct_quotient_agrees_with_predecessor,
            "uniform_not_multiplied": uniform_specializations_not_multiplied,
            "output_terms_constructed": grammar.boundary.demand_output_terms_constructed,
            "output_terms_typed": grammar.boundary.demand_output_terms_kernel_typed,
            "d_membership": grammar.boundary.d_membership_decided,
        }),
    );
    Ok(E5A3Stage16GeneratorAudit {
        orientation: "W=(S15,S14)".to_owned(),
        typed_source_count: window.typed_sources.len(),
        unary_raw_seed_count,
        unary_promoted_metadata_instance_count: unary_generated,
        chronological_raw_seed_count,
        direct_chronological_metadata_instance_count: direct,
        pointwise_chronological_metadata_instance_count: pointwise,
        chronological_promoted_metadata_instance_count: chronological_generated,
        higher_raw_seed_count,
        higher_promoted_metadata_instance_count: higher_generated,
        higher_rejection_reason,
        completion_metadata_instance_count: completion_generated,
        total_metadata_instance_count: window.instances.len(),
        total_candidate_orbit_count: window.orbits.len(),
        singleton_candidate_orbit_count,
        unary_candidate_orbit_count,
        direct_candidate_orbit_count,
        pointwise_candidate_orbit_count,
        distinct_newest_j3_source_count_in_direct_instances: direct_newest_sources.len(),
        direct_predecessor_orbit_count: predecessor.natural_family_orbit_count,
        direct_quotient_agrees_with_predecessor,
        uniform_specializations_not_multiplied,
        source_occurrence_formation_replayed: window.every_instance_typed,
        demand_output_terms_constructed: grammar.boundary.demand_output_terms_constructed,
        demand_output_terms_kernel_typed: grammar.boundary.demand_output_terms_kernel_typed,
        d_membership_decided: grammar.boundary.d_membership_decided,
        derivation_hash,
    })
}

fn historical_projection_audit(
    grammar: &A3HistoricalDemandGrammar,
    caller_noninterference: &pen_eval::a3_demand_grammar::A3CallerInventoryNoninterference,
) -> Result<E5A3HistoricalProjectionAudit, E5A3SuccessorError> {
    let windows = grammar
        .windows
        .iter()
        .map(|window| (window.stage, window))
        .collect::<BTreeMap<_, _>>();
    let mut rows = Vec::new();
    for coarse in directive_debt_timeline() {
        let window = windows.get(&coarse.stage).ok_or_else(|| {
            E5A3SuccessorError::Invariant(format!(
                "candidate generator lacks stage {}",
                coarse.stage
            ))
        })?;
        let independently_replayed_completion_constructors = window
            .constructor_evidence
            .iter()
            .map(|evidence| evidence.constructor.slug().to_owned())
            .collect::<Vec<_>>();
        let coarse_regression_packages = coarse
            .required_packages
            .iter()
            .map(|package| (*package).to_owned())
            .collect::<Vec<_>>();
        let projected_focus = window
            .focus_projection
            .projected_focus
            .map(|constructor| constructor.slug().to_owned());
        let labels_match_as_regression =
            independently_replayed_completion_constructors == coarse_regression_packages;
        let semantic_output_orbit_joined = false;
        let derivation_hash = tagged_hash(
            "historical-focus-regression-row",
            &(
                coarse.stage,
                &window.focus_projection.derivation_hash,
                &independently_replayed_completion_constructors,
                &coarse_regression_packages,
                &projected_focus,
                coarse.pre_structural_band,
                window.focus_projection.demand_precedes_jurisdiction,
                labels_match_as_regression,
                semantic_output_orbit_joined,
            ),
        );
        rows.push(E5A3ProjectionRow {
            stage: coarse.stage,
            independently_replayed_completion_constructors,
            coarse_regression_packages,
            projected_focus,
            pre_structural_band: coarse.pre_structural_band,
            demand_precedes_jurisdiction: window.focus_projection.demand_precedes_jurisdiction,
            labels_match_as_regression,
            semantic_output_orbit_joined,
            derivation_hash,
        });
    }
    let coarse_ladder_labels_reproduced = rows.iter().all(|row| row.labels_match_as_regression);
    let stage3_wrinkle_reproduced_as_regression =
        rows.iter().find(|row| row.stage == 3).is_some_and(|row| {
            row.pre_structural_band
                && row.demand_precedes_jurisdiction
                && row.projected_focus.is_none()
                && row.independently_replayed_completion_constructors
                    == vec!["former_eliminator".to_owned()]
        });
    let stage3_instance_orbit_transport_proved = false;
    let output_level_focus_projection_proved = false;
    let weakening_stable_instance_discharge_proved = false;
    let locality_and_expiration_complete = false;
    let derivation_hash = tagged_hash(
        "historical-focus-regression-boundary",
        &(
            &rows,
            grammar
                .boundary
                .completion_predicates_replayed_from_raw_snapshot_without_coarse_labels,
            caller_noninterference.empty_inventory_equal,
            caller_noninterference.forged_inventory_equal,
            coarse_ladder_labels_reproduced,
            stage3_wrinkle_reproduced_as_regression,
            stage3_instance_orbit_transport_proved,
            output_level_focus_projection_proved,
            weakening_stable_instance_discharge_proved,
            locality_and_expiration_complete,
        ),
    );
    Ok(E5A3HistoricalProjectionAudit {
        window_count: grammar.windows.len(),
        all_windows_finite_by_enumeration: grammar
            .windows
            .iter()
            .all(|window| window.finite_by_construction),
        raw_snapshot_completion_predicates_count_blind: grammar
            .boundary
            .completion_predicates_replayed_from_raw_snapshot_without_coarse_labels,
        empty_and_forged_caller_inventories_byte_identical: caller_noninterference
            .empty_inventory_equal
            && caller_noninterference.forged_inventory_equal,
        rows,
        coarse_ladder_labels_reproduced,
        stage3_wrinkle_reproduced_as_regression,
        stage3_instance_orbit_transport_proved,
        output_level_focus_projection_proved,
        weakening_stable_instance_discharge_proved,
        locality_and_expiration_complete,
        derivation_hash,
    })
}

fn definition_boundary_audit(
    grammar: &A3HistoricalDemandGrammar,
    stage16: &E5A3Stage16GeneratorAudit,
) -> E5A3DefinitionBoundaryAudit {
    let generator_candidate_replays = true;
    let rule_constructor_inventory_closed_in_source = grammar.boundary.rule_constructors.len()
        == A3RuleConstructor::ALL.len()
        && grammar
            .boundary
            .rule_constructors
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
            == A3RuleConstructor::ALL.into_iter().collect::<BTreeSet<_>>();
    let rule_constructor_inventory_exhaustiveness_proved = grammar
        .boundary
        .rule_constructor_inventory_exhaustiveness_proved;
    let each_scheme_has_a_metadata_output_description = grammar.windows.iter().all(|window| {
        window.instances.iter().all(|instance| {
            window
                .schemes
                .iter()
                .any(|scheme| scheme.scheme_id == instance.scheme_id)
        })
    });
    let each_scheme_has_a_constructed_semantic_output_term =
        grammar.boundary.demand_output_terms_constructed;
    let each_semantic_output_term_has_a_kernel_typing_derivation =
        grammar.boundary.demand_output_terms_kernel_typed;
    let source_reflexivity_accepted_as_unary_action_proof = false;
    let motive_parametric_theorem_used_without_a_source_closure_derivation = false;
    let full_a3_identification_proved = rule_constructor_inventory_exhaustiveness_proved
        && each_scheme_has_a_constructed_semantic_output_term
        && each_semantic_output_term_has_a_kernel_typing_derivation
        && stage16.direct_quotient_agrees_with_predecessor
        && stage16.d_membership_decided;
    let named_gaps = vec![
        A3_OUTPUT_TERM_GAP.to_owned(),
        A3_EXHAUSTIVENESS_GAP.to_owned(),
        A3_QUOTIENT_GAP.to_owned(),
        A3_D_MEMBERSHIP_GAP.to_owned(),
        A3_STAGE3_TRANSPORT_GAP.to_owned(),
    ];
    let derivation_hash = tagged_hash(
        "definition-boundary",
        &(
            generator_candidate_replays,
            rule_constructor_inventory_closed_in_source,
            rule_constructor_inventory_exhaustiveness_proved,
            each_scheme_has_a_metadata_output_description,
            each_scheme_has_a_constructed_semantic_output_term,
            each_semantic_output_term_has_a_kernel_typing_derivation,
            source_reflexivity_accepted_as_unary_action_proof,
            motive_parametric_theorem_used_without_a_source_closure_derivation,
            full_a3_identification_proved,
            &named_gaps,
        ),
    );
    E5A3DefinitionBoundaryAudit {
        generator_candidate_replays,
        rule_constructor_inventory_closed_in_source,
        rule_constructor_inventory_exhaustiveness_proved,
        each_scheme_has_a_metadata_output_description,
        each_scheme_has_a_constructed_semantic_output_term,
        each_semantic_output_term_has_a_kernel_typing_derivation,
        source_reflexivity_accepted_as_unary_action_proof,
        motive_parametric_theorem_used_without_a_source_closure_derivation,
        full_a3_identification_proved,
        named_gaps,
        derivation_hash,
    }
}

pub fn issue_e5_a3_successor_certificate() -> Result<E5A3SuccessorCertificate, E5A3SuccessorError> {
    let predecessor = parse_predecessor()?;
    let restricted_j3 = restricted_j3_audit(&predecessor)?;
    let signature = SealedSignature::genesis_del_h15();
    let grammar = issue_historical_a3_demand_grammar(&signature)
        .map_err(|error| E5A3SuccessorError::Generator(error.to_string()))?;
    replay_historical_a3_demand_grammar(&signature, &grammar)
        .map_err(|error| E5A3SuccessorError::Generator(error.to_string()))?;
    let forged_inventory = (1..=16)
        .map(|stage| UntrustedCallerDemandInventory {
            stage,
            claimed_focus_labels: vec!["forged_focus".to_owned()],
            claimed_demand_ids: vec![format!("forged-demand-{stage}")],
        })
        .collect::<Vec<_>>();
    let caller_noninterference =
        audit_a3_caller_inventory_noninterference(&signature, &forged_inventory)
            .map_err(|error| E5A3SuccessorError::Generator(error.to_string()))?;
    let stage16 = stage16_generator_audit(&grammar, &restricted_j3)?;
    let historical_projection = historical_projection_audit(&grammar, &caller_noninterference)?;
    let definition_boundary = definition_boundary_audit(&grammar, &stage16);

    let full_a3_f1_executable = definition_boundary.full_a3_identification_proved
        && historical_projection.output_level_focus_projection_proved
        && historical_projection.locality_and_expiration_complete;
    let demanded_but_underdetermined_instance = None;
    let f1_triggered = false;
    let f1_excluded = false;
    let semantic_o16_empty = None;
    let theorem12_instance_granularity_proved = false;
    let theorem12_refuted = false;
    let t_bf2_authorized = false;
    let bridge_authorized = false;
    let status = "not_triggered_and_not_excluded: the candidate generator fails the direct J3 quotient regression and has no typed semantic output judgment, so full-A3 F1 is not executable".to_owned();
    let f1_derivation_hash = tagged_hash(
        "f1-fail-closed-disposition",
        &(
            full_a3_f1_executable,
            &demanded_but_underdetermined_instance,
            f1_triggered,
            f1_excluded,
            semantic_o16_empty,
            theorem12_instance_granularity_proved,
            theorem12_refuted,
            t_bf2_authorized,
            bridge_authorized,
            &status,
            &definition_boundary.derivation_hash,
        ),
    );
    let f1 = E5A3F1Audit {
        full_a3_f1_executable,
        demanded_but_underdetermined_instance,
        f1_triggered,
        f1_excluded,
        semantic_o16_empty,
        theorem12_instance_granularity_proved,
        theorem12_refuted,
        t_bf2_authorized,
        bridge_authorized,
        status,
        derivation_hash: f1_derivation_hash,
    };
    let mut certificate = E5A3SuccessorCertificate {
        schema: E5_A3_SUCCESSOR_SCHEMA.to_owned(),
        date: E5_A3_SUCCESSOR_DATE.to_owned(),
        source_bindings: source_bindings(),
        restricted_j3,
        stage16,
        historical_projection,
        definition_boundary,
        f1,
        e5_successor_attempted: true,
        e5_complete: false,
        semantic_o16_certificate_issued: false,
        t_bf2_executed: false,
        bridge_executed: false,
        bar_free_law_adopted: false,
        halt_claim_issued: false,
        outcome: "e5_a3_successor_stopped_fail_closed_on_output_typing_and_orbit_quotient_gaps"
            .to_owned(),
        permitted_conclusion: "The count-blind candidate generator is finite and nonvacuous, and independently reproduces the coarse completion labels and Stage-3 wrinkle from raw structural premises. At Stage 16 it emits 89 metadata occurrences. The exact predecessor theorem remains 64 typed direct instances quotienting to eight D(B15)-derivable J3 families. The new generator instead makes those 64 direct cases singleton orbits and constructs no semantic demand output term for unary/higher/completion rules. Therefore it is not full A3 C(W).".to_owned(),
        required_successor_action: "Define each A3 rule as an intrinsically typed demand expression/hole with semantic interpretation; prove the rule inventory exhaustive; quotient the 64 direct specializations back to the eight established J3 natural-family orbits without multiplying uniform instances; construct a genuine Stage-3-to-4 demand-orbit transport; then rerun this successor. T-BF2 and the bridge remain queued. Separately, T-BF1/T-BF3 have already failed on the Stage-4 parsimony tie, so no bar-free adoption follows even after E-5 is repaired.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> E5A3SuccessorReplay {
    E5A3SuccessorReplay {
        valid: false,
        metadata_instance_count: 0,
        candidate_orbit_count: 0,
        direct_quotient_regression_found: false,
        full_a3_identification_proved: false,
        f1_executable: false,
        semantic_o16_empty: None,
        e5_complete: false,
        bridge_authorized: false,
        outcome: "e5_a3_successor_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &E5A3SuccessorCertificate,
    expected: &E5A3SuccessorCertificate,
) -> E5A3SuccessorReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    E5A3SuccessorReplay {
        valid: errors.is_empty(),
        metadata_instance_count: certificate.stage16.total_metadata_instance_count,
        candidate_orbit_count: certificate.stage16.total_candidate_orbit_count,
        direct_quotient_regression_found: !certificate
            .stage16
            .direct_quotient_agrees_with_predecessor,
        full_a3_identification_proved: certificate
            .definition_boundary
            .full_a3_identification_proved,
        f1_executable: certificate.f1.full_a3_f1_executable,
        semantic_o16_empty: certificate.f1.semantic_o16_empty,
        e5_complete: certificate.e5_complete,
        bridge_authorized: certificate.f1.bridge_authorized,
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_e5_a3_successor_certificate(
    certificate: &E5A3SuccessorCertificate,
) -> E5A3SuccessorReplay {
    match issue_e5_a3_successor_certificate() {
        Ok(expected) => replay_against_expected(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn replay_e5_a3_successor_json(json: &str) -> E5A3SuccessorReplay {
    match serde_json::from_str::<E5A3SuccessorCertificate>(json) {
        Ok(certificate) => replay_e5_a3_successor_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_e5_a3_successor_create_new(
    path: &Path,
) -> Result<E5A3SuccessorReplay, E5A3SuccessorError> {
    let certificate = issue_e5_a3_successor_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| E5A3SuccessorError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| E5A3SuccessorError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| E5A3SuccessorError::Io(error.to_string()))?;
    let replay = replay_e5_a3_successor_certificate(&certificate);
    if !replay.valid {
        return Err(E5A3SuccessorError::EmittedReplay(replay.errors.join("; ")));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn expected() -> E5A3SuccessorCertificate {
        static CERTIFICATE: OnceLock<E5A3SuccessorCertificate> = OnceLock::new();
        CERTIFICATE
            .get_or_init(|| issue_e5_a3_successor_certificate().expect("E-5 A3 gap audit"))
            .clone()
    }

    #[test]
    fn candidate_generator_is_nonvacuous_but_fails_the_j3_quotient_regression() {
        let certificate = expected();
        assert_eq!(certificate.stage16.total_metadata_instance_count, 89);
        assert_eq!(certificate.stage16.total_candidate_orbit_count, 89);
        assert_eq!(
            certificate
                .stage16
                .direct_chronological_metadata_instance_count,
            64
        );
        assert_eq!(certificate.stage16.direct_candidate_orbit_count, 64);
        assert_eq!(certificate.restricted_j3.natural_family_orbit_count, 8);
        assert_eq!(
            certificate
                .stage16
                .distinct_newest_j3_source_count_in_direct_instances,
            8
        );
        assert!(!certificate.stage16.direct_quotient_agrees_with_predecessor);
        assert!(!certificate.stage16.uniform_specializations_not_multiplied);
    }

    #[test]
    fn no_missing_term_judgment_is_promoted_to_f1_or_semantic_o16() {
        let certificate = expected();
        assert!(
            certificate
                .historical_projection
                .coarse_ladder_labels_reproduced
        );
        assert!(
            certificate
                .historical_projection
                .stage3_wrinkle_reproduced_as_regression
        );
        assert!(
            !certificate
                .definition_boundary
                .full_a3_identification_proved
        );
        assert!(!certificate.f1.full_a3_f1_executable);
        assert!(!certificate.f1.f1_triggered);
        assert!(!certificate.f1.f1_excluded);
        assert_eq!(certificate.f1.semantic_o16_empty, None);
        assert!(!certificate.e5_complete);
        assert!(!certificate.f1.bridge_authorized);
        assert!(!certificate.t_bf2_executed);
        assert!(!certificate.halt_claim_issued);
    }

    #[test]
    fn replay_rejects_a_redigested_false_promotion() {
        let expected = expected();
        let mut promoted = expected.clone();
        promoted.definition_boundary.full_a3_identification_proved = true;
        promoted.f1.full_a3_f1_executable = true;
        promoted.f1.f1_excluded = true;
        promoted.f1.semantic_o16_empty = Some(true);
        promoted.e5_complete = true;
        promoted.f1.bridge_authorized = true;
        promoted.result_digest = certificate_digest(&promoted);
        let replay = replay_e5_a3_successor_certificate(&promoted);
        assert!(!replay.valid);
    }
}
