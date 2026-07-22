//! E-5 demand projection: a fail-closed instance audit of the exact J3 surface.
//!
//! This module deliberately separates two claims.  The exact eight Step-15
//! J3 schemes can be specialized against every typed Step-14 formation
//! family and checked in `D(B15)`.  That finite check is not, by itself, an
//! extraction-completeness theorem for the abstract A3 `C(W)` assignment.

use crate::global_e4_assembly_v10::{GlobalE4V10Certificate, replay_global_e4_v10_json};
use crate::phase5b_history_certification::Phase5bHistoryCertificate;
use crate::phase5b_reselection_v3::{
    Phase5bReselectionV3Burn, Phase5bReselectionV3Program, replay_phase5b_reselection_v3_burn,
    replay_phase5b_reselection_v3_program,
};
use pen_core::clause::ClauseRec;
use pen_core::expr::Expr;
use pen_core::hash::blake3_hex;
use pen_core::telescope::Telescope;
use pen_eval::demand_completeness::{
    DemandCompletenessCertificate, IntendedRuleSeedKind, replay_demand_completeness_json,
};
use pen_schema::grammar_completion::{
    GrammarCompletionCertificate, RegisteredTraceSignature, replay_grammar_completion_json,
};
use pen_type::elaborate::{SealedSignature, elaborate_telescope};
use pen_type::equality::{KERNEL_EQUALITY_PROCEDURE, univalent_equality};
use pen_type::substitution::{
    SortedParameterContext, SubstitutionImage, issue_structural_substitution,
    replay_structural_substitution,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const E5_DEMAND_PROJECTION_SCHEMA: &str = "schema2-e5-demand-projection-v1";
pub const E5_DEMAND_PROJECTION_DATE: &str = "2026-07-21";
pub const E5_RESTRICTED_GRAMMAR: &str = "exact-j3-temporal-schemes-over-typed-step14-formations-v1";
pub const E5_COMPLETENESS_GAP: &str =
    "E5_A3_CW_EXTRACTION_AND_HISTORICAL_FOCUS_PROJECTION_NOT_PROVED";

const BAR_FREE_PROPOSAL_BYTES: &[u8] = include_bytes!("../../../docs/bar_free_law_proposal.md");
const V3_PROGRAM_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_program_v3.json");
const V3_BURN_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_reselection_burn_v3.json");
const GRAMMAR_BYTES: &[u8] = include_bytes!("../../../docs/schema2_grammar_completion_v1.json");
const GLOBAL_E4_BYTES: &[u8] = include_bytes!("../../../docs/schema2_global_e4_assembly_v10.json");
const DEMAND_COMPLETE_BYTES: &[u8] = include_bytes!("../../../docs/demand_completeness_v1.json");
const HISTORY_BYTES: &[u8] = include_bytes!("../../../docs/phase5b_full_history_v1.json");
const THEOREM_BYTES: &[u8] = include_bytes!("../../../docs/t1_result.md");
const SUBSTITUTION_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/substitution.rs");
const EQUALITY_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/equality.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("e5_demand_projection.rs");

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5PrerequisiteAudit {
    pub reselection_v3_replayed: bool,
    pub completed_through_stage15: bool,
    pub e5_authorized_by_reselection: bool,
    pub global_e4_archive_declares_complete: bool,
    pub global_e4_live_replay_valid: bool,
    pub global_e4_live_replay_errors: Vec<String>,
    pub grammar_archive_declares_j3_exact: bool,
    pub grammar_archive_declares_normalization_naturality: bool,
    pub grammar_live_replay_valid: bool,
    pub grammar_live_replay_errors: Vec<String>,
    pub grammar_drift_bound_explicitly: bool,
    pub demand_complete_v1_live_replay_valid: bool,
    pub demand_complete_v1_named_d4_gap: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum E5J3SchemeKind {
    OperatorFormer,
    Comparison,
    Interaction,
    Exchange,
    Eliminator,
    Composition,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "status")]
pub enum E5MembershipDisposition {
    Derivable {
        structural_substitution_hash: String,
        result_elaboration_hash: String,
        normalization_equality_hash: String,
        d_membership_derivation_hash: String,
    },
    Underdetermined {
        exact_reason: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5DemandInstance {
    pub instance_id: String,
    pub family_id: String,
    pub orbit_id: String,
    pub scheme_kind: E5J3SchemeKind,
    pub scheme_clause: u16,
    pub scheme_name: String,
    pub interface_step: u32,
    pub interface_clause: u16,
    pub interface_name: String,
    pub interface_kernel_type_json: String,
    pub generic_scheme: Expr,
    pub typed_image: Expr,
    pub required_output: Expr,
    pub required_output_normal_form: Option<Expr>,
    pub required_output_kernel_type_json: Option<String>,
    pub depth: u8,
    pub support_steps: Vec<u32>,
    pub natural_family_not_new_uniform_family: bool,
    pub independently_exported_output_orbit: bool,
    pub disposition: E5MembershipDisposition,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5OrbitRecord {
    pub orbit_id: String,
    pub family_id: String,
    pub scheme_clause: u16,
    pub scheme_name: String,
    pub instance_ids: Vec<String>,
    pub uniform_instances_collapsed: bool,
    pub independently_exported_output_positions: u32,
    pub every_instance_derivable: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5RestrictedWindowAudit {
    pub grammar: String,
    pub window_newest_step: u32,
    pub window_older_step: u32,
    pub orientation: String,
    pub typed_interface_formation_count: usize,
    pub exact_j3_scheme_count: usize,
    pub raw_instance_count: usize,
    pub checked_instance_count: usize,
    pub orbit_count: usize,
    pub instances: Vec<E5DemandInstance>,
    pub orbits: Vec<E5OrbitRecord>,
    pub finite: bool,
    pub generated_without_caller_focus_labels: bool,
    pub all_empty_caller_timeline_cannot_suppress_generation: bool,
    pub equality_procedure: String,
    pub every_instance_typed: bool,
    pub every_instance_derivability_decided: bool,
    pub every_orbit_derivable: bool,
    pub first_underdetermined_instance: Option<String>,
    pub exhaustive_unless_f1_stopped_early: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5ExtractionCompletenessAudit {
    pub agent_d_stage16_clause_seed_count: usize,
    pub agent_d_stage16_unary_seed_count: usize,
    pub agent_d_stage16_binary_seed_count: usize,
    pub agent_d_stage16_higher_seed_count: usize,
    pub agent_d_stage16_total_preinstance_seed_count: usize,
    pub preinstance_seeds_promoted_wholesale_to_cw: bool,
    pub historical_focus_ladder_rederived_from_independent_cw: bool,
    pub stage3_wrinkle_rederived_from_independent_cw: bool,
    pub restricted_j3_inventory_proved_equal_to_all_a3_cw: bool,
    pub window_locality_and_expiration_proved_for_all_orbits: bool,
    pub inherited_history_capacity_has_serialized_instances: bool,
    pub inherited_history_capacity_has_per_instance_d_proofs: bool,
    pub gap_id: String,
    pub exact_gap: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5F1Audit {
    pub restricted_j3_check_executable: bool,
    pub restricted_j3_demanded_but_underdetermined_instance: Option<String>,
    pub restricted_j3_f1_triggered: bool,
    pub restricted_j3_f1_excluded: bool,
    pub full_a3_f1_executable: bool,
    pub full_a3_f1_triggered: bool,
    pub full_a3_f1_excluded: bool,
    pub theorem12_instance_granularity_proved: bool,
    pub theorem12_refuted: bool,
    pub semantic_o16_empty: Option<bool>,
    pub status: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5DemandProjectionCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<E5SourceBinding>,
    pub prerequisite: E5PrerequisiteAudit,
    pub restricted_window: E5RestrictedWindowAudit,
    pub extraction_completeness: E5ExtractionCompletenessAudit,
    pub f1: E5F1Audit,
    pub e5_attempt_executed: bool,
    pub e5_complete: bool,
    pub bar_free_proposal_adopted: bool,
    pub t_bf1_replayed: bool,
    pub t_bf2_replayed: bool,
    pub t_bf3_replayed: bool,
    pub bridge_authorized: bool,
    pub halt_claim_issued: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5DemandProjectionReplay {
    pub valid: bool,
    pub restricted_instance_count: usize,
    pub restricted_orbit_count: usize,
    pub restricted_f1_triggered: bool,
    pub restricted_f1_excluded: bool,
    pub full_f1_executable: bool,
    pub e5_complete: bool,
    pub theorem12_proved: bool,
    pub bridge_authorized: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum E5DemandProjectionError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(E5_DEMAND_PROJECTION_SCHEMA, domain, value))
        .expect("E-5 proof data serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<E5SourceBinding> {
    [
        (
            "docs/bar_free_law_proposal.md",
            "proposal_not_adoption_authority",
            BAR_FREE_PROPOSAL_BYTES,
        ),
        (
            "docs/phase5b_reselection_program_v3.json",
            "preregistered_reselection_program",
            V3_PROGRAM_BYTES,
        ),
        (
            "docs/phase5b_reselection_burn_v3.json",
            "completed_revised_prefix",
            V3_BURN_BYTES,
        ),
        (
            "docs/schema2_grammar_completion_v1.json",
            "frozen_exact_j3_signature_surface",
            GRAMMAR_BYTES,
        ),
        (
            "docs/schema2_global_e4_assembly_v10.json",
            "completed_candidate_classifier_surface",
            GLOBAL_E4_BYTES,
        ),
        (
            "docs/demand_completeness_v1.json",
            "independent_preinstance_seed_stock",
            DEMAND_COMPLETE_BYTES,
        ),
        (
            "docs/phase5b_full_history_v1.json",
            "inherited_capacity_claim_audit_only",
            HISTORY_BYTES,
        ),
        (
            "docs/t1_result.md",
            "guard_rail_a3_theorem12_and_f1_statement",
            THEOREM_BYTES,
        ),
        (
            "crates/pen-type/src/substitution.rs",
            "structural_substitution_replay",
            SUBSTITUTION_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/equality.rs",
            "frozen_univalent_equality",
            EQUALITY_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/e5_demand_projection.rs",
            "this_fail_closed_issuer",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| E5SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn scheme_kind(clause: u16) -> E5J3SchemeKind {
    match clause {
        0 | 1 => E5J3SchemeKind::OperatorFormer,
        2 => E5J3SchemeKind::Comparison,
        3 => E5J3SchemeKind::Interaction,
        4 | 5 => E5J3SchemeKind::Exchange,
        6 => E5J3SchemeKind::Eliminator,
        7 => E5J3SchemeKind::Composition,
        _ => unreachable!("exact J3 clause inventory is 0..8"),
    }
}

fn certificate_digest(certificate: &E5DemandProjectionCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("e5-demand-projection-certificate", &projection)
}

fn parse_json<T: for<'de> Deserialize<'de>>(
    bytes: &[u8],
    name: &str,
) -> Result<T, E5DemandProjectionError> {
    serde_json::from_slice(bytes)
        .map_err(|error| E5DemandProjectionError::Json(format!("{name}: {error}")))
}

fn issue_instance(
    signature: &SealedSignature,
    interface: &RegisteredTraceSignature,
    scheme: &RegisteredTraceSignature,
) -> E5DemandInstance {
    let family_id = tagged_hash(
        "j3-natural-family",
        &(
            scheme.step,
            scheme.clause_index,
            &scheme.registered_name,
            &scheme.normal_form,
            &scheme.normalization_naturality_hash,
        ),
    );
    let instance_id = tagged_hash(
        "j3-typed-instance",
        &(
            &family_id,
            interface.step,
            interface.clause_index,
            &interface.registered_name,
            &interface.normal_form,
        ),
    );
    let orbit_id = tagged_hash("j3-natural-family-orbit", &family_id);
    let mut record = E5DemandInstance {
        instance_id: instance_id.clone(),
        family_id: family_id.clone(),
        orbit_id,
        scheme_kind: scheme_kind(scheme.clause_index),
        scheme_clause: scheme.clause_index,
        scheme_name: scheme.registered_name.clone(),
        interface_step: interface.step,
        interface_clause: interface.clause_index,
        interface_name: interface.registered_name.clone(),
        interface_kernel_type_json: interface.kernel_type_json.clone(),
        generic_scheme: scheme.raw_expr.clone(),
        typed_image: interface.raw_expr.clone(),
        required_output: scheme.raw_expr.clone(),
        required_output_normal_form: None,
        required_output_kernel_type_json: None,
        depth: 2,
        support_steps: vec![14, 15],
        natural_family_not_new_uniform_family: true,
        independently_exported_output_orbit: false,
        disposition: E5MembershipDisposition::Underdetermined {
            exact_reason: "instance construction did not run".to_owned(),
        },
    };

    let result = (|| -> Result<E5MembershipDisposition, String> {
        if interface.kernel_type_json != "\"Type\"" {
            return Err(format!(
                "typed hole requires Type, found {}",
                interface.kernel_type_json
            ));
        }
        if !interface.term_level_elaboration_succeeded
            || !interface.normalization_natural
            || !scheme.term_level_elaboration_succeeded
            || !scheme.normalization_natural
        {
            return Err("source registration is not typed and normalization-natural".to_owned());
        }
        let substitution = issue_structural_substitution(
            SortedParameterContext::all_type(1),
            SortedParameterContext::all_type(1),
            vec![SubstitutionImage {
                source_parameter: 1,
                term: interface.raw_expr.clone(),
            }],
            scheme.raw_expr.clone(),
        )
        .map_err(|error| format!("structural substitution failed: {error}"))?;
        replay_structural_substitution(&substitution)
            .map_err(|error| format!("structural substitution replay failed: {error}"))?;
        record.required_output = substitution.result().clone();
        let telescope = Telescope::new(vec![ClauseRec::new(
            scheme.declared_role,
            record.required_output.clone(),
        )]);
        let elaboration = elaborate_telescope(signature, &telescope, 15)
            .map_err(|error| format!("required output failed typing: {error}"))?;
        let clause = &elaboration.clauses[0];
        record.required_output_normal_form = Some(clause.normal_form.clone());
        record.required_output_kernel_type_json = Some(
            serde_json::to_string(&clause.kernel_ty)
                .map_err(|error| format!("kernel type serialization failed: {error}"))?,
        );
        let normalized_substitution = issue_structural_substitution(
            SortedParameterContext::all_type(1),
            SortedParameterContext::all_type(1),
            vec![SubstitutionImage {
                source_parameter: 1,
                term: interface.normal_form.clone(),
            }],
            scheme.normal_form.clone(),
        )
        .map_err(|error| format!("normal-form substitution failed: {error}"))?;
        replay_structural_substitution(&normalized_substitution)
            .map_err(|error| format!("normal-form substitution replay failed: {error}"))?;
        let equality = univalent_equality(
            &clause.normal_form,
            normalized_substitution.result(),
            elaboration.ambient_parameters,
            4096,
        )
        .map_err(|error| format!("normalization equality failed: {error}"))?;
        if !equality.equal {
            return Err("substitution did not commute with frozen normalization".to_owned());
        }
        let result_elaboration_hash = tagged_hash(
            "typed-required-output",
            &(
                signature.digest(),
                elaboration.ambient_parameters,
                &record.required_output,
                &clause.kernel_ty,
                &clause.normal_form,
                &elaboration.derivation_hash,
            ),
        );
        let normalization_equality_hash = tagged_hash(
            "normalization-substitution-equality",
            &(
                &equality,
                normalized_substitution.derivation_hash(),
                &scheme.normalization_naturality_hash,
                &interface.normalization_naturality_hash,
            ),
        );
        let d_membership_derivation_hash = tagged_hash(
            "typed-kernel-scheme-specialization-in-d-b15",
            &(
                &family_id,
                &instance_id,
                substitution.derivation_hash(),
                &result_elaboration_hash,
                &normalization_equality_hash,
                "kernel schemes are closed under replayed typed substitution",
            ),
        );
        Ok(E5MembershipDisposition::Derivable {
            structural_substitution_hash: substitution.derivation_hash().to_owned(),
            result_elaboration_hash,
            normalization_equality_hash,
            d_membership_derivation_hash,
        })
    })();
    record.disposition = result
        .unwrap_or_else(|exact_reason| E5MembershipDisposition::Underdetermined { exact_reason });
    record
}

pub fn issue_e5_demand_projection_certificate()
-> Result<E5DemandProjectionCertificate, E5DemandProjectionError> {
    let program: Phase5bReselectionV3Program = parse_json(V3_PROGRAM_BYTES, "v3 program")?;
    let program_errors = replay_phase5b_reselection_v3_program(&program);
    if !program_errors.is_empty() {
        return Err(E5DemandProjectionError::Prerequisite(format!(
            "v3 program replay failed: {}",
            program_errors.join("; ")
        )));
    }
    let burn: Phase5bReselectionV3Burn = parse_json(V3_BURN_BYTES, "v3 burn")?;
    let burn_replay = replay_phase5b_reselection_v3_burn(&program, &burn);
    if !burn_replay.valid || !burn_replay.completed_through_stage15 || !burn_replay.e5_f1_authorized
    {
        return Err(E5DemandProjectionError::Prerequisite(format!(
            "v3 did not authorize E-5: {:?}",
            burn_replay.errors
        )));
    }

    let grammar: GrammarCompletionCertificate = parse_json(GRAMMAR_BYTES, "grammar")?;
    let grammar_replay = replay_grammar_completion_json(
        std::str::from_utf8(GRAMMAR_BYTES)
            .map_err(|error| E5DemandProjectionError::Json(error.to_string()))?,
    );
    if !grammar.step15_j3_exact
        || !grammar.full_adopted_grammar_normalization_naturality_complete
        || grammar
            .registrations
            .iter()
            .filter(|row| row.step == 15)
            .count()
            != 8
    {
        return Err(E5DemandProjectionError::Prerequisite(
            "frozen grammar archive does not carry exact typed J3".to_owned(),
        ));
    }

    let global_e4: GlobalE4V10Certificate = parse_json(GLOBAL_E4_BYTES, "global E4")?;
    let global_e4_replay = replay_global_e4_v10_json(
        std::str::from_utf8(GLOBAL_E4_BYTES)
            .map_err(|error| E5DemandProjectionError::Json(error.to_string()))?,
    );
    if !global_e4.global_e4_complete || !global_e4.class_exhaustion_proved {
        return Err(E5DemandProjectionError::Prerequisite(
            "global E4 archive is incomplete".to_owned(),
        ));
    }

    let demand_complete: DemandCompletenessCertificate =
        parse_json(DEMAND_COMPLETE_BYTES, "demand complete")?;
    let demand_replay = replay_demand_completeness_json(
        std::str::from_utf8(DEMAND_COMPLETE_BYTES)
            .map_err(|error| E5DemandProjectionError::Json(error.to_string()))?,
    );
    let stage16 = demand_complete
        .intended_window_specifications
        .iter()
        .find(|window| window.stage == 16)
        .ok_or_else(|| {
            E5DemandProjectionError::Prerequisite("missing stage-16 seed stock".to_owned())
        })?;
    let unary_seed_count = stage16
        .rule_seeds
        .iter()
        .filter(|seed| seed.kind == IntendedRuleSeedKind::UnaryAction)
        .count();
    let binary_seed_count = stage16
        .rule_seeds
        .iter()
        .filter(|seed| seed.kind == IntendedRuleSeedKind::BinaryComparison)
        .count();
    let higher_seed_count = stage16
        .rule_seeds
        .iter()
        .filter(|seed| seed.kind == IntendedRuleSeedKind::HigherOpenBoxReduction)
        .count();

    let history: Phase5bHistoryCertificate = parse_json(HISTORY_BYTES, "history")?;
    let inherited_history_capacity_has_serialized_instances = false;
    let inherited_history_capacity_has_per_instance_d_proofs = history.steps.iter().any(|step| {
        step.demand_grammar.every_generated_orbit_derivable
            && step.demand_grammar.total_orbit_capacity == 0
    });

    let interface = grammar
        .registrations
        .iter()
        .filter(|row| row.step == 14 && row.kernel_type_json == "\"Type\"")
        .collect::<Vec<_>>();
    let schemes = grammar
        .registrations
        .iter()
        .filter(|row| row.step == 15)
        .collect::<Vec<_>>();
    if interface.len() != 8 || schemes.len() != 8 {
        return Err(E5DemandProjectionError::Invariant(format!(
            "expected 8 typed interface formations and 8 exact J3 schemes, found {}/{}",
            interface.len(),
            schemes.len()
        )));
    }
    let signature = SealedSignature::genesis_del_h15();
    let raw_instance_count = interface.len() * schemes.len();
    let mut instances = Vec::with_capacity(raw_instance_count);
    let mut first_underdetermined_instance = None;
    'outer: for scheme in &schemes {
        for source in &interface {
            let instance = issue_instance(&signature, source, scheme);
            if matches!(
                instance.disposition,
                E5MembershipDisposition::Underdetermined { .. }
            ) {
                first_underdetermined_instance = Some(instance.instance_id.clone());
                instances.push(instance);
                break 'outer;
            }
            instances.push(instance);
        }
    }
    let checked_instance_count = instances.len();
    let every_instance_derivability_decided = instances.iter().all(|instance| {
        matches!(
            instance.disposition,
            E5MembershipDisposition::Derivable { .. }
        )
    });
    let exhaustive_unless_f1_stopped_early =
        first_underdetermined_instance.is_some() || checked_instance_count == raw_instance_count;

    let mut orbit_instances = BTreeMap::<String, Vec<&E5DemandInstance>>::new();
    for instance in &instances {
        orbit_instances
            .entry(instance.orbit_id.clone())
            .or_default()
            .push(instance);
    }
    let mut orbits = Vec::new();
    for (orbit_id, members) in orbit_instances {
        let first = members[0];
        let instance_ids = members
            .iter()
            .map(|instance| instance.instance_id.clone())
            .collect::<Vec<_>>();
        let every_instance_derivable = members.iter().all(|instance| {
            matches!(
                instance.disposition,
                E5MembershipDisposition::Derivable { .. }
            )
        });
        let derivation_hash = tagged_hash(
            "natural-family-instance-orbit",
            &(
                &orbit_id,
                &first.family_id,
                first.scheme_clause,
                &instance_ids,
                every_instance_derivable,
                "uniform_specializations_not_multiplied_without_independent_export",
            ),
        );
        orbits.push(E5OrbitRecord {
            orbit_id,
            family_id: first.family_id.clone(),
            scheme_clause: first.scheme_clause,
            scheme_name: first.scheme_name.clone(),
            uniform_instances_collapsed: members.len() > 1,
            independently_exported_output_positions: 1,
            instance_ids,
            every_instance_derivable,
            derivation_hash,
        });
    }
    orbits.sort_by_key(|orbit| orbit.scheme_clause);
    let every_orbit_derivable = orbits.iter().all(|orbit| orbit.every_instance_derivable);
    let every_instance_typed = instances.iter().all(|instance| {
        instance.required_output_normal_form.is_some()
            && instance.required_output_kernel_type_json.is_some()
    });
    let restricted_derivation_hash = tagged_hash(
        "restricted-stage16-window-audit",
        &(
            E5_RESTRICTED_GRAMMAR,
            &instances,
            &orbits,
            every_instance_typed,
            every_instance_derivability_decided,
            every_orbit_derivable,
            &first_underdetermined_instance,
        ),
    );
    let restricted_window = E5RestrictedWindowAudit {
        grammar: E5_RESTRICTED_GRAMMAR.to_owned(),
        window_newest_step: 15,
        window_older_step: 14,
        orientation: "W=(S15,S14)".to_owned(),
        typed_interface_formation_count: interface.len(),
        exact_j3_scheme_count: schemes.len(),
        raw_instance_count,
        checked_instance_count,
        orbit_count: orbits.len(),
        instances,
        orbits,
        finite: true,
        generated_without_caller_focus_labels: true,
        all_empty_caller_timeline_cannot_suppress_generation: raw_instance_count > 0,
        equality_procedure: KERNEL_EQUALITY_PROCEDURE.to_owned(),
        every_instance_typed,
        every_instance_derivability_decided,
        every_orbit_derivable,
        first_underdetermined_instance: first_underdetermined_instance.clone(),
        exhaustive_unless_f1_stopped_early,
        derivation_hash: restricted_derivation_hash,
    };

    let extraction_derivation_hash = tagged_hash(
        "extraction-completeness-boundary",
        &(
            stage16.clause_seeds.len(),
            unary_seed_count,
            binary_seed_count,
            higher_seed_count,
            stage16.rule_seeds.len(),
            &demand_complete.primary_gap_id,
            E5_COMPLETENESS_GAP,
            inherited_history_capacity_has_serialized_instances,
            inherited_history_capacity_has_per_instance_d_proofs,
        ),
    );
    let extraction_completeness = E5ExtractionCompletenessAudit {
        agent_d_stage16_clause_seed_count: stage16.clause_seeds.len(),
        agent_d_stage16_unary_seed_count: unary_seed_count,
        agent_d_stage16_binary_seed_count: binary_seed_count,
        agent_d_stage16_higher_seed_count: higher_seed_count,
        agent_d_stage16_total_preinstance_seed_count: stage16.rule_seeds.len(),
        preinstance_seeds_promoted_wholesale_to_cw: false,
        historical_focus_ladder_rederived_from_independent_cw: false,
        stage3_wrinkle_rederived_from_independent_cw: false,
        restricted_j3_inventory_proved_equal_to_all_a3_cw: false,
        window_locality_and_expiration_proved_for_all_orbits: false,
        inherited_history_capacity_has_serialized_instances,
        inherited_history_capacity_has_per_instance_d_proofs,
        gap_id: E5_COMPLETENESS_GAP.to_owned(),
        exact_gap: "The exact J3 specialization subgrammar is operational, but neither A3 nor the completed candidate classifier proves that its eight natural-family orbits exhaust every ordinary, binary, higher-open-box, and future-hole demand generated by (S15,S14). The historical focus ladder therefore cannot be re-derived from this restricted inventory without importing its labels.".to_owned(),
        derivation_hash: extraction_derivation_hash,
    };

    let restricted_f1_triggered = first_underdetermined_instance.is_some();
    let restricted_f1_excluded = !restricted_f1_triggered
        && restricted_window.checked_instance_count == restricted_window.raw_instance_count
        && restricted_window.every_orbit_derivable;
    let full_a3_f1_executable = false;
    let f1_derivation_hash = tagged_hash(
        "f1-disposition",
        &(
            restricted_f1_triggered,
            restricted_f1_excluded,
            full_a3_f1_executable,
            &first_underdetermined_instance,
            E5_COMPLETENESS_GAP,
        ),
    );
    let f1 = E5F1Audit {
        restricted_j3_check_executable: true,
        restricted_j3_demanded_but_underdetermined_instance: first_underdetermined_instance,
        restricted_j3_f1_triggered: restricted_f1_triggered,
        restricted_j3_f1_excluded: restricted_f1_excluded,
        full_a3_f1_executable,
        full_a3_f1_triggered: false,
        full_a3_f1_excluded: false,
        theorem12_instance_granularity_proved: false,
        theorem12_refuted: false,
        semantic_o16_empty: None,
        status: if restricted_f1_triggered {
            "restricted_j3_demanded_but_underdetermined_instance_found; stop before any promotion to full A3 F1"
        } else {
            "exact_j3_subgrammar_clean_but_full_a3_f1_not_executable_without_extraction_completeness"
        }
        .to_owned(),
        derivation_hash: f1_derivation_hash,
    };

    let prerequisite = E5PrerequisiteAudit {
        reselection_v3_replayed: burn_replay.valid,
        completed_through_stage15: burn_replay.completed_through_stage15,
        e5_authorized_by_reselection: burn_replay.e5_f1_authorized,
        global_e4_archive_declares_complete: global_e4.global_e4_complete,
        global_e4_live_replay_valid: global_e4_replay.valid,
        global_e4_live_replay_errors: global_e4_replay.errors,
        grammar_archive_declares_j3_exact: grammar.step15_j3_exact,
        grammar_archive_declares_normalization_naturality: grammar
            .full_adopted_grammar_normalization_naturality_complete,
        grammar_live_replay_valid: grammar_replay.valid,
        grammar_live_replay_errors: grammar_replay.errors,
        grammar_drift_bound_explicitly: !grammar_replay.valid,
        demand_complete_v1_live_replay_valid: demand_replay.valid,
        demand_complete_v1_named_d4_gap: !demand_complete.extractor_identified_with_intended_cw,
    };

    let e5_complete = false;
    let mut certificate = E5DemandProjectionCertificate {
        schema: E5_DEMAND_PROJECTION_SCHEMA.to_owned(),
        date: E5_DEMAND_PROJECTION_DATE.to_owned(),
        source_bindings: source_bindings(),
        prerequisite,
        restricted_window,
        extraction_completeness,
        f1,
        e5_attempt_executed: true,
        e5_complete,
        bar_free_proposal_adopted: false,
        t_bf1_replayed: false,
        t_bf2_replayed: false,
        t_bf3_replayed: false,
        bridge_authorized: false,
        halt_claim_issued: false,
        outcome: if restricted_f1_triggered {
            "e5_stopped_on_restricted_j3_underdetermined_instance"
        } else {
            "e5_partial_exact_j3_clean_a3_extraction_completeness_open"
        }
        .to_owned(),
        permitted_conclusion: "The exact eight J3 temporal natural families have been tested at each of the eight typed Step-14 formation instances. This is a restricted D(B15) result only; it is not equality with the abstract A3 C(S15,S14), not semantic O(16) emptiness, and not Theorem 12.".to_owned(),
        required_successor_action: "Define the missing count-blind demand constructors and prove that their generated historical C(W) focus projection reproduces the ladder and stage-3 wrinkle; then prove the restricted J3 inventory plus those constructors is extraction-complete and rerun E-5 create-new. Do not adopt the bar-free proposal or run the bridge from this partial result.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> E5DemandProjectionReplay {
    E5DemandProjectionReplay {
        valid: false,
        restricted_instance_count: 0,
        restricted_orbit_count: 0,
        restricted_f1_triggered: false,
        restricted_f1_excluded: false,
        full_f1_executable: false,
        e5_complete: false,
        theorem12_proved: false,
        bridge_authorized: false,
        outcome: "e5_replay_failed".to_owned(),
        errors: vec![error.into()],
    }
}

fn replay_against_expected(
    certificate: &E5DemandProjectionCertificate,
    expected: &E5DemandProjectionCertificate,
) -> E5DemandProjectionReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != expected {
        errors.push("certificate differs from definition replay".to_owned());
    }
    E5DemandProjectionReplay {
        valid: errors.is_empty(),
        restricted_instance_count: certificate.restricted_window.checked_instance_count,
        restricted_orbit_count: certificate.restricted_window.orbit_count,
        restricted_f1_triggered: certificate.f1.restricted_j3_f1_triggered,
        restricted_f1_excluded: certificate.f1.restricted_j3_f1_excluded,
        full_f1_executable: certificate.f1.full_a3_f1_executable,
        e5_complete: certificate.e5_complete,
        theorem12_proved: certificate.f1.theorem12_instance_granularity_proved,
        bridge_authorized: certificate.bridge_authorized,
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn replay_e5_demand_projection_certificate(
    certificate: &E5DemandProjectionCertificate,
) -> E5DemandProjectionReplay {
    match issue_e5_demand_projection_certificate() {
        Ok(expected) => replay_against_expected(certificate, &expected),
        Err(error) => failed_replay(error.to_string()),
    }
}

pub fn replay_e5_demand_projection_json(json: &str) -> E5DemandProjectionReplay {
    match serde_json::from_str::<E5DemandProjectionCertificate>(json) {
        Ok(certificate) => replay_e5_demand_projection_certificate(&certificate),
        Err(error) => failed_replay(format!("JSON parse failed: {error}")),
    }
}

pub fn emit_e5_demand_projection_create_new(
    path: &Path,
) -> Result<E5DemandProjectionReplay, E5DemandProjectionError> {
    let certificate = issue_e5_demand_projection_certificate()?;
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| E5DemandProjectionError::Json(error.to_string()))?;
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| E5DemandProjectionError::Io(error.to_string()))?;
    output
        .write_all(&bytes)
        .and_then(|_| output.write_all(b"\n"))
        .map_err(|error| E5DemandProjectionError::Io(error.to_string()))?;
    let replay = replay_e5_demand_projection_certificate(&certificate);
    if !replay.valid {
        return Err(E5DemandProjectionError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn expected() -> E5DemandProjectionCertificate {
        static CERTIFICATE: OnceLock<E5DemandProjectionCertificate> = OnceLock::new();
        CERTIFICATE
            .get_or_init(|| issue_e5_demand_projection_certificate().expect("E-5 attempt"))
            .clone()
    }

    #[test]
    fn exact_j3_check_never_promotes_itself_to_a3_completeness() {
        let certificate = expected();
        assert_eq!(certificate.restricted_window.raw_instance_count, 64);
        assert!(
            certificate
                .restricted_window
                .generated_without_caller_focus_labels
        );
        assert!(
            certificate
                .restricted_window
                .all_empty_caller_timeline_cannot_suppress_generation
        );
        assert!(
            !certificate
                .extraction_completeness
                .restricted_j3_inventory_proved_equal_to_all_a3_cw
        );
        assert!(!certificate.f1.full_a3_f1_executable);
        assert!(!certificate.e5_complete);
        assert!(!certificate.bridge_authorized);
        assert!(!certificate.halt_claim_issued);
    }

    #[test]
    fn replay_rejects_redigested_semantic_promotions() {
        let expected = expected();
        let mut promoted = expected.clone();
        promoted.e5_complete = true;
        promoted.f1.full_a3_f1_executable = true;
        promoted.f1.full_a3_f1_excluded = true;
        promoted.f1.theorem12_instance_granularity_proved = true;
        promoted.f1.semantic_o16_empty = Some(true);
        promoted.bridge_authorized = true;
        promoted.halt_claim_issued = true;
        promoted.result_digest = certificate_digest(&promoted);
        assert!(!replay_against_expected(&promoted, &expected).valid);
    }

    #[test]
    fn strict_json_replay_rejects_unknown_fields() {
        let certificate = expected();
        let json = serde_json::to_string_pretty(&certificate).expect("JSON");
        let unknown = json.replacen("{\n", "{\n  \"unknown\": true,\n", 1);
        assert!(!replay_e5_demand_projection_json(&unknown).valid);
    }
}
