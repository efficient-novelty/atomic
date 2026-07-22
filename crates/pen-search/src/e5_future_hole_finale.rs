//! Create-new E-5 finale under `future-hole-hypothesis-definition-v1`.
//!
//! This certificate joins, rather than rewrites, three independently
//! replayable surfaces: the complete count-blind A3 grammar, the certified
//! chronological transports, and the adopted future-hole closure.  It then
//! performs the instance-level D decision and projects the resulting
//! structural scheme ledger back to the historical coarse O-ladder.

use crate::naturality_orbit_transport::{
    NaturalityOrbitTransportCertificate, issue_naturality_orbit_transport_certificate,
    replay_naturality_orbit_transport_certificate,
};
use pen_core::hash::blake3_hex;
use pen_eval::a3_demand_grammar::{
    A3RuleConstructor, A3SeedRejectionReason, UntrustedCallerDemandInventory,
    audit_a3_caller_inventory_noninterference, issue_historical_a3_demand_grammar,
    replay_historical_a3_demand_grammar,
};
use pen_eval::debt_guard::directive_debt_timeline;
use pen_eval::future_hole_hypothesis::{
    FutureHoleHypothesisCertificate, issue_future_hole_hypothesis_certificate,
    replay_future_hole_hypothesis_certificate,
};
use pen_type::elaborate::SealedSignature;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const E5_FUTURE_HOLE_FINALE_SCHEMA: &str = "schema2-e5-future-hole-finale-v1";
pub const E5_FUTURE_HOLE_FINALE_DATE: &str = "2026-07-21";

const ADJUDICATION_BYTES: &[u8] =
    include_bytes!("../../../docs/future_hole_definition_adjudication.md");
const FUTURE_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/future_hole_hypothesis.rs");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const TRANSPORT_SOURCE_BYTES: &[u8] = include_bytes!("naturality_orbit_transport.rs");
const DEBT_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/debt_guard.rs");
const THEOREM_BYTES: &[u8] = include_bytes!("../../../docs/t1_result.md");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("e5_future_hole_finale.rs");

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(E5_FUTURE_HOLE_FINALE_SCHEMA, domain, value))
        .expect("E-5 finale evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5FinaleSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5FinaleFrozenDriftAudit {
    pub transport_archive_replayed: bool,
    pub grammar_archive_digest_valid: bool,
    pub grammar_live_replay_valid: bool,
    pub grammar_live_replay_errors: Vec<String>,
    pub j3_predecessor_archive_digest_valid: bool,
    pub j3_predecessor_live_replay_valid: bool,
    pub j3_predecessor_live_replay_errors: Vec<String>,
    pub frozen_surface_drift_bound_explicitly: bool,
    pub historical_artifact_reinterpreted_after_drift: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5FinaleMembershipRow {
    pub a3_instance_id: String,
    pub a3_scheme_id: String,
    pub rule: String,
    pub source_step: Option<u32>,
    pub source_clause: Option<u16>,
    pub family_id: Option<String>,
    pub orbit_id: Option<String>,
    pub d_membership_derivation_hash: String,
    pub hypothetical_derivation_replayed: bool,
    pub output_kernel_typed: bool,
    pub uniform_specialization_not_new_family: bool,
    pub independently_exported_output_orbit: bool,
    pub hole_charge_zero: Option<bool>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5FinaleStage16Audit {
    pub a3_grammar_live_replay_valid: bool,
    pub caller_inventory_noninterference_replayed: bool,
    pub caller_inventory_noninterference_hash: String,
    pub a3_metadata_instance_count: usize,
    pub unary_instance_count: usize,
    pub direct_chronological_instance_count: usize,
    pub pointwise_chronological_instance_count: usize,
    pub higher_instance_count: usize,
    pub structural_instance_count: usize,
    pub higher_seed_rejected_for_no_typed_path_witness: bool,
    pub direct_regression_64_to_8: bool,
    pub pointwise_joined_to_same_eight_families: bool,
    pub old_unary_gap_ids_joined_exactly: bool,
    pub membership_rows: Vec<E5FinaleMembershipRow>,
    pub derivable_instance_ids: Vec<String>,
    pub underdetermined_instance_ids: Vec<String>,
    pub inventory_partition_exact: bool,
    pub every_instance_d_decided: bool,
    pub every_instance_derivable: bool,
    pub full_a3_output_grammar_complete: bool,
    pub semantic_o16_empty: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5Stage3To4FutureSchemeTransport {
    pub stage3_a3_instance_id: String,
    pub stage4_a3_instance_id: String,
    pub occurrence_ids_intentionally_not_equated: bool,
    pub same_constructor: bool,
    pub same_open_body: bool,
    pub same_declared_motives: bool,
    pub same_source_and_output_kernel_types: bool,
    pub same_transparent_closure_tree: bool,
    pub both_support_local_before_extension: bool,
    pub both_expressions_preserved_by_extension: bool,
    pub both_motives_preserved_by_extension: bool,
    pub same_whole_step4_filler: bool,
    pub same_specialized_expression: bool,
    pub both_clause4_prime_instantiations_replayed: bool,
    pub semantic_scheme_transport_proved: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5HistoricalFocusRow {
    pub stage: u32,
    pub coarse_required_packages: Vec<String>,
    pub a3_constructor_evidence: Vec<String>,
    pub future_hole_constructors: Vec<String>,
    pub discharged_constructors: Vec<String>,
    pub projected_focus: Option<String>,
    pub demand_precedes_jurisdiction: bool,
    pub stage_three_wrinkle_replayed: bool,
    pub focus_projection_reproduces_coarse_record: bool,
    pub every_standing_hole_discharged_by_specialization: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5HistoricalLedgerAudit {
    pub rows: Vec<E5HistoricalFocusRow>,
    pub stage3_to_stage4_future_scheme_transport: E5Stage3To4FutureSchemeTransport,
    pub historical_structural_hole_count: usize,
    pub historical_discharge_count: usize,
    pub old_structural_gap_ids_joined_exactly: bool,
    pub f_fh1_every_historical_discharge_rederived: bool,
    pub focus_projection_reproduces_entire_coarse_ladder: bool,
    pub stage_three_wrinkle_reproduced: bool,
    pub coarse_o16_empty: bool,
    pub semantic_o16_agrees_with_coarse_projection: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5FinaleF1Audit {
    pub full_a3_grammar_available: bool,
    pub f1_executable: bool,
    pub f1_executed: bool,
    pub demanded_instance_count: usize,
    pub derivable_instance_count: usize,
    pub demanded_but_underdetermined_instance: Option<String>,
    pub f1_triggered: bool,
    pub f1_excluded: bool,
    pub theorem12_full_instance_granularity_proved: bool,
    pub theorem12_refuted: bool,
    pub semantic_o16_empty: Option<bool>,
    pub disposition: String,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5FutureHoleFinaleCertificate {
    pub schema: String,
    pub date: String,
    pub source_bindings: Vec<E5FinaleSourceBinding>,
    pub desired_count_bar_winner_or_f1_verdict_used_as_definition_input: bool,
    pub future_hole_certificate: FutureHoleHypothesisCertificate,
    pub frozen_drift: E5FinaleFrozenDriftAudit,
    pub stage16: E5FinaleStage16Audit,
    pub historical: E5HistoricalLedgerAudit,
    pub f_fh2_no_inferred_repaired_or_outcome_selected_motive: bool,
    pub f_fh3_no_credit_anchor_or_orbit_from_holes_or_fillings: bool,
    pub f_fh4_named_gaps: Vec<String>,
    pub f1: E5FinaleF1Audit,
    pub e5_complete: bool,
    pub semantic_o16_certificate_issued: bool,
    pub t_bf2_authorized: bool,
    pub t_bf2_executed: bool,
    pub bridge_prerequisite_satisfied: bool,
    pub bridge_authorized: bool,
    pub bridge_executed: bool,
    pub bar_free_adoption_authorized: bool,
    pub bar_free_law_adopted: bool,
    pub halt_claim_issued: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct E5FutureHoleFinaleReplay {
    pub valid: bool,
    pub membership_count: usize,
    pub underdetermined_count: usize,
    pub historical_discharge_count: usize,
    pub semantic_o16_empty: bool,
    pub f1_executed: bool,
    pub f1_excluded: bool,
    pub e5_complete: bool,
    pub t_bf2_authorized: bool,
    pub bridge_authorized: bool,
    pub outcome: String,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum E5FutureHoleFinaleError {
    #[error("prerequisite failed: {0}")]
    Prerequisite(String),
    #[error("E-5 finale invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted certificate did not replay: {0}")]
    EmittedReplay(String),
}

fn source_bindings() -> Vec<E5FinaleSourceBinding> {
    [
        (
            "docs/future_hole_definition_adjudication.md",
            "adopted_future_hole_definition",
            ADJUDICATION_BYTES,
        ),
        (
            "crates/pen-eval/src/future_hole_hypothesis.rs",
            "typed_open_judgment_closure_and_specialization_replay",
            FUTURE_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "count_blind_full_A3_generator",
            A3_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/naturality_orbit_transport.rs",
            "certified_chronological_D_memberships_and_64_to_8_regression",
            TRANSPORT_SOURCE_BYTES,
        ),
        (
            "crates/pen-eval/src/debt_guard.rs",
            "historical_coarse_O_ladder_regression_corpus",
            DEBT_SOURCE_BYTES,
        ),
        (
            "docs/t1_result.md",
            "Theorem_12_and_F1_statement",
            THEOREM_BYTES,
        ),
        (
            "crates/pen-search/src/e5_future_hole_finale.rs",
            "this_create_new_instance_audit",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| E5FinaleSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn certificate_digest(certificate: &E5FutureHoleFinaleCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn transport_prerequisite() -> Result<NaturalityOrbitTransportCertificate, E5FutureHoleFinaleError>
{
    let transport = issue_naturality_orbit_transport_certificate()
        .map_err(|error| E5FutureHoleFinaleError::Prerequisite(error.to_string()))?;
    let replay = replay_naturality_orbit_transport_certificate(&transport);
    if !replay.valid {
        return Err(E5FutureHoleFinaleError::Prerequisite(
            replay.errors.join("; "),
        ));
    }
    if !transport.a3.stage16_inventory_partition_exact
        || !transport.a3.chronological_instance_transport_bijection
        || !transport.a3.j3_regression_64_to_8
        || !transport.a3.pointwise_instances_join_existing_j3_families
        || !transport
            .a3
            .chronological_outputs_constructed_and_kernel_typed
        || transport.a3.unary_pending_outputs.len() != 17
        || transport.a3.structural_completion_pending_outputs.len() != 13
        || !transport.a3.frozen_surface_drift_bound_explicitly
    {
        return Err(E5FutureHoleFinaleError::Prerequisite(
            "transport does not preserve the exact 17+64+8 partition, historical 13 holes, and explicit frozen-drift boundary"
                .to_owned(),
        ));
    }
    Ok(transport)
}

fn membership_row_hash(row: &E5FinaleMembershipRow) -> String {
    let mut projection = row.clone();
    projection.derivation_hash.clear();
    tagged_hash("D-membership-row", &projection)
}

pub fn issue_e5_future_hole_finale_certificate()
-> Result<E5FutureHoleFinaleCertificate, E5FutureHoleFinaleError> {
    let future = issue_future_hole_hypothesis_certificate()
        .map_err(|error| E5FutureHoleFinaleError::Prerequisite(error.to_string()))?;
    let future_replay = replay_future_hole_hypothesis_certificate(&future);
    if !future_replay.valid {
        return Err(E5FutureHoleFinaleError::Prerequisite(
            future_replay.errors.join("; "),
        ));
    }
    let transport = transport_prerequisite()?;
    let signature = SealedSignature::genesis_del_h15();
    let grammar = issue_historical_a3_demand_grammar(&signature)
        .map_err(|error| E5FutureHoleFinaleError::Prerequisite(error.to_string()))?;
    replay_historical_a3_demand_grammar(&signature, &grammar)
        .map_err(|error| E5FutureHoleFinaleError::Prerequisite(error.to_string()))?;
    let caller_noninterference = audit_a3_caller_inventory_noninterference(
        &signature,
        &[UntrustedCallerDemandInventory {
            stage: 16,
            claimed_focus_labels: vec!["forged_future_focus".to_owned()],
            claimed_demand_ids: vec!["forged_future_demand".to_owned()],
        }],
    )
    .map_err(|error| E5FutureHoleFinaleError::Prerequisite(error.to_string()))?;
    let caller_inventory_noninterference_replayed = caller_noninterference.empty_inventory_equal
        && caller_noninterference.forged_inventory_equal;
    if !caller_inventory_noninterference_replayed {
        return Err(E5FutureHoleFinaleError::Invariant(
            "caller focus labels or demand ids influenced A3 generation".to_owned(),
        ));
    }
    let stage16_window = grammar
        .windows
        .iter()
        .find(|window| window.stage == 16)
        .ok_or_else(|| E5FutureHoleFinaleError::Invariant("A3 omits Stage 16".to_owned()))?;

    let old_unary_ids = transport
        .a3
        .unary_pending_outputs
        .iter()
        .map(|row| {
            Ok((
                row.a3_instance_id.clone().ok_or_else(|| {
                    E5FutureHoleFinaleError::Invariant(
                        "old unary gap omitted instance id".to_owned(),
                    )
                })?,
                row.a3_scheme_id.clone().ok_or_else(|| {
                    E5FutureHoleFinaleError::Invariant("old unary gap omitted scheme id".to_owned())
                })?,
            ))
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    let new_unary_ids = future
        .unary_actions
        .iter()
        .map(|row| (row.a3_instance_id.clone(), row.a3_scheme_id.clone()))
        .collect::<BTreeSet<_>>();
    let old_unary_gap_ids_joined_exactly = old_unary_ids == new_unary_ids;
    if !old_unary_gap_ids_joined_exactly {
        return Err(E5FutureHoleFinaleError::Invariant(
            "future closure does not join exactly the 17 formerly pending unary outputs".to_owned(),
        ));
    }

    let mut membership_rows = Vec::new();
    for open in &future.unary_actions {
        let scheme = stage16_window
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == open.a3_scheme_id)
            .ok_or_else(|| {
                E5FutureHoleFinaleError::Invariant(format!(
                    "unary future row {} has no Stage-16 scheme",
                    open.a3_instance_id
                ))
            })?;
        if scheme.rule_constructor != A3RuleConstructor::UnaryAction {
            return Err(E5FutureHoleFinaleError::Invariant(format!(
                "unary future row {} does not join a unary A3 scheme",
                open.a3_instance_id
            )));
        }
        let mut row = E5FinaleMembershipRow {
            a3_instance_id: open.a3_instance_id.clone(),
            a3_scheme_id: open.a3_scheme_id.clone(),
            rule: "unary_action_by_future_hypothesis_closure".to_owned(),
            source_step: open.source_step,
            source_clause: open.source_clause,
            family_id: None,
            orbit_id: None,
            d_membership_derivation_hash: open.d_membership_derivation_hash.clone(),
            hypothetical_derivation_replayed: open.hypothetical_derivation_replayable
                && open.d_membership_issued
                && open.named_gap.is_none(),
            output_kernel_typed: open.open_judgment_kernel_typed && open.output_type_preserved,
            uniform_specialization_not_new_family: true,
            independently_exported_output_orbit: false,
            hole_charge_zero: Some(
                open.charge.marginal_kappa == 0
                    && open.charge.marginal_nu == 0
                    && open.charge.anchors_minted == 0
                    && open.charge.demand_orbits_minted == 0
                    && !open.charge.credit_minted
                    && !open.charge.filler_minted_credit,
            ),
            derivation_hash: String::new(),
        };
        row.derivation_hash = membership_row_hash(&row);
        membership_rows.push(row);
    }
    for transported in transport
        .a3
        .direct_instances
        .iter()
        .chain(&transport.a3.pointwise_instances)
    {
        let a3_instance = stage16_window
            .instances
            .iter()
            .find(|instance| instance.instance_id == transported.a3_instance_id)
            .ok_or_else(|| {
                E5FutureHoleFinaleError::Invariant(format!(
                    "transported chronological instance {} is absent from Stage 16",
                    transported.a3_instance_id
                ))
            })?;
        let a3_scheme = stage16_window
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == a3_instance.scheme_id)
            .ok_or_else(|| {
                E5FutureHoleFinaleError::Invariant(
                    "transported chronological instance has no A3 scheme".to_owned(),
                )
            })?;
        if a3_instance.scheme_id != transported.a3_scheme_id
            || a3_scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison
        {
            return Err(E5FutureHoleFinaleError::Invariant(format!(
                "transported instance {} does not join its exact chronological scheme",
                transported.a3_instance_id
            )));
        }
        let mut row = E5FinaleMembershipRow {
            a3_instance_id: transported.a3_instance_id.clone(),
            a3_scheme_id: transported.a3_scheme_id.clone(),
            rule: format!("chronological_comparison::{}", transported.mode),
            source_step: Some(transported.newest_step),
            source_clause: Some(transported.newest_clause),
            family_id: Some(transported.family_id.clone()),
            orbit_id: Some(transported.orbit_id.clone()),
            d_membership_derivation_hash: transported.d_membership_derivation_hash.clone(),
            hypothetical_derivation_replayed: true,
            output_kernel_typed: transported.normalization_equality.equal
                && !transported.required_output_kernel_type_json.is_empty(),
            uniform_specialization_not_new_family: transported
                .uniform_specialization_not_new_family,
            independently_exported_output_orbit: transported.independently_exported_output_orbit,
            hole_charge_zero: None,
            derivation_hash: String::new(),
        };
        row.derivation_hash = membership_row_hash(&row);
        membership_rows.push(row);
    }
    membership_rows.sort_by(|left, right| left.a3_instance_id.cmp(&right.a3_instance_id));

    let all_instance_ids = stage16_window
        .instances
        .iter()
        .map(|instance| instance.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let derivable_ids = membership_rows
        .iter()
        .filter(|row| {
            row.hypothetical_derivation_replayed
                && row.output_kernel_typed
                && row.uniform_specialization_not_new_family
                && !row.independently_exported_output_orbit
                && row.hole_charge_zero != Some(false)
        })
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let underdetermined_ids = all_instance_ids
        .difference(&derivable_ids)
        .cloned()
        .collect::<Vec<_>>();

    let mut higher_instance_count = 0;
    let mut structural_instance_count = 0;
    for instance in &stage16_window.instances {
        let scheme = stage16_window
            .schemes
            .iter()
            .find(|scheme| scheme.scheme_id == instance.scheme_id)
            .ok_or_else(|| E5FutureHoleFinaleError::Invariant("orphan A3 instance".to_owned()))?;
        match scheme.rule_constructor {
            A3RuleConstructor::HigherOpenBoxReduction => higher_instance_count += 1,
            A3RuleConstructor::StructuralCompletionHole => structural_instance_count += 1,
            _ => {}
        }
    }
    let higher_seed_rejected_for_no_typed_path_witness = stage16_window
        .seed_dispositions
        .iter()
        .find(|row| row.rule_constructor == A3RuleConstructor::HigherOpenBoxReduction)
        .is_some_and(|row| {
            row.promoted_seed_count == 0
                && row.generated_typed_instance_count == 0
                && row.rejection_reason == Some(A3SeedRejectionReason::NoTypedPathWitness)
        });
    let membership_instance_ids = membership_rows
        .iter()
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let inventory_partition_exact = membership_rows.len() == all_instance_ids.len()
        && future.unary_actions.len() == 17
        && transport.a3.direct_instances.len() == 64
        && transport.a3.pointwise_instances.len() == 8
        && higher_instance_count == 0
        && structural_instance_count == 0
        && higher_seed_rejected_for_no_typed_path_witness
        && membership_instance_ids == all_instance_ids;
    if !inventory_partition_exact {
        return Err(E5FutureHoleFinaleError::Invariant(format!(
            "Stage-16 D partition drifted: total={}, unary={}, direct={}, pointwise={}, higher={}, structural={}, derivable={}",
            all_instance_ids.len(),
            future.unary_actions.len(),
            transport.a3.direct_instances.len(),
            transport.a3.pointwise_instances.len(),
            higher_instance_count,
            structural_instance_count,
            derivable_ids.len()
        )));
    }

    let old_structural_ids = transport
        .a3
        .structural_completion_pending_outputs
        .iter()
        .map(|row| {
            row.a3_instance_id.clone().ok_or_else(|| {
                E5FutureHoleFinaleError::Invariant(
                    "old structural gap omitted instance id".to_owned(),
                )
            })
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    let new_structural_ids = future
        .structural_holes
        .iter()
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let old_structural_gap_ids_joined_exactly = old_structural_ids == new_structural_ids;
    if !old_structural_gap_ids_joined_exactly {
        return Err(E5FutureHoleFinaleError::Invariant(
            "future closure does not join exactly the 13 formerly pending structural outputs"
                .to_owned(),
        ));
    }

    let timeline = directive_debt_timeline();
    let mut historical_rows = Vec::new();
    for coarse in &timeline {
        let window = grammar
            .windows
            .iter()
            .find(|window| window.stage == coarse.stage)
            .ok_or_else(|| {
                E5FutureHoleFinaleError::Invariant(format!(
                    "A3 omits historical Stage {}",
                    coarse.stage
                ))
            })?;
        let coarse_required_packages = coarse
            .required_packages
            .iter()
            .map(|package| (*package).to_owned())
            .collect::<Vec<_>>();
        let raw_constructor_evidence = window
            .constructor_evidence
            .iter()
            .map(|evidence| evidence.constructor.slug().to_owned())
            .collect::<Vec<_>>();
        let mut a3_constructor_evidence = Vec::new();
        for link in &window.focus_projection.completion_orbits {
            let scheme = window
                .schemes
                .iter()
                .find(|scheme| scheme.scheme_id == link.scheme_id)
                .ok_or_else(|| {
                    E5FutureHoleFinaleError::Invariant(format!(
                        "Stage {} completion link has no scheme",
                        coarse.stage
                    ))
                })?;
            let orbit = window
                .orbits
                .iter()
                .find(|orbit| orbit.orbit_id == link.orbit_id)
                .ok_or_else(|| {
                    E5FutureHoleFinaleError::Invariant(format!(
                        "Stage {} completion link has no orbit",
                        coarse.stage
                    ))
                })?;
            if orbit.scheme_id != scheme.scheme_id || orbit.member_instance_ids.len() != 1 {
                return Err(E5FutureHoleFinaleError::Invariant(format!(
                    "Stage {} completion scheme/orbit join is not singleton",
                    coarse.stage
                )));
            }
            let instance_id = &orbit.member_instance_ids[0];
            let open = future
                .structural_holes
                .iter()
                .find(|row| {
                    row.stage == coarse.stage
                        && row.a3_instance_id == *instance_id
                        && row.a3_scheme_id == scheme.scheme_id
                })
                .ok_or_else(|| {
                    E5FutureHoleFinaleError::Invariant(format!(
                        "Stage {} completion instance does not join a future hypothesis",
                        coarse.stage
                    ))
                })?;
            let discharge = future
                .historical_discharges
                .iter()
                .find(|row| {
                    row.registration_stage == coarse.stage
                        && row.a3_instance_id == *instance_id
                        && row.a3_scheme_id == scheme.scheme_id
                })
                .ok_or_else(|| {
                    E5FutureHoleFinaleError::Invariant(format!(
                        "Stage {} future hypothesis has no historical discharge",
                        coarse.stage
                    ))
                })?;
            if open.structural_constructor.as_deref() != Some(link.constructor.slug())
                || !open.d_membership_issued
                || !open.hypothetical_derivation_replayable
                || !discharge.clause4_prime_instantiation_replayed
                || discharge.named_gap.is_some()
            {
                return Err(E5FutureHoleFinaleError::Invariant(format!(
                    "Stage {} semantic completion join failed its typed replay",
                    coarse.stage
                )));
            }
            a3_constructor_evidence.push(link.constructor.slug().to_owned());
        }
        if a3_constructor_evidence != raw_constructor_evidence {
            return Err(E5FutureHoleFinaleError::Invariant(format!(
                "Stage {} focus links do not reproduce constructor evidence",
                coarse.stage
            )));
        }
        let future_hole_constructors = future
            .structural_holes
            .iter()
            .filter(|row| row.stage == coarse.stage)
            .filter_map(|row| row.structural_constructor.clone())
            .collect::<Vec<_>>();
        let discharged_constructors = future
            .historical_discharges
            .iter()
            .filter(|row| row.registration_stage == coarse.stage)
            .map(|row| row.structural_constructor.clone())
            .collect::<Vec<_>>();
        let projected_focus = window
            .focus_projection
            .projected_focus
            .map(|constructor| constructor.slug().to_owned());
        let stage_three_wrinkle_replayed = coarse.stage == 3
            && coarse_required_packages == vec!["former_eliminator".to_owned()]
            && a3_constructor_evidence == coarse_required_packages
            && projected_focus.is_none()
            && window.focus_projection.demand_precedes_jurisdiction;
        let focus_projection_reproduces_coarse_record = a3_constructor_evidence
            == coarse_required_packages
            && future_hole_constructors == coarse_required_packages;
        let every_standing_hole_discharged_by_specialization = discharged_constructors
            == coarse_required_packages
            && future
                .historical_discharges
                .iter()
                .filter(|row| row.registration_stage == coarse.stage)
                .all(|row| row.clause4_prime_instantiation_replayed && row.named_gap.is_none());
        let derivation_hash = tagged_hash(
            "historical-focus-row",
            &(
                coarse.stage,
                &coarse_required_packages,
                &a3_constructor_evidence,
                &future_hole_constructors,
                &discharged_constructors,
                &projected_focus,
                window.focus_projection.demand_precedes_jurisdiction,
                stage_three_wrinkle_replayed,
                focus_projection_reproduces_coarse_record,
                every_standing_hole_discharged_by_specialization,
            ),
        );
        historical_rows.push(E5HistoricalFocusRow {
            stage: coarse.stage,
            coarse_required_packages,
            a3_constructor_evidence,
            future_hole_constructors,
            discharged_constructors,
            projected_focus,
            demand_precedes_jurisdiction: window.focus_projection.demand_precedes_jurisdiction,
            stage_three_wrinkle_replayed,
            focus_projection_reproduces_coarse_record,
            every_standing_hole_discharged_by_specialization,
            derivation_hash,
        });
    }
    let stage3_open = future
        .structural_holes
        .iter()
        .find(|row| row.stage == 3)
        .ok_or_else(|| {
            E5FutureHoleFinaleError::Invariant(
                "Stage 3 former-eliminator future hypothesis is absent".to_owned(),
            )
        })?;
    let stage4_open = future
        .structural_holes
        .iter()
        .find(|row| row.stage == 4)
        .ok_or_else(|| {
            E5FutureHoleFinaleError::Invariant(
                "Stage 4 former-eliminator future hypothesis is absent".to_owned(),
            )
        })?;
    let stage3_discharge = future
        .historical_discharges
        .iter()
        .find(|row| row.registration_stage == 3)
        .ok_or_else(|| {
            E5FutureHoleFinaleError::Invariant("Stage 3 discharge is absent".to_owned())
        })?;
    let stage4_discharge = future
        .historical_discharges
        .iter()
        .find(|row| row.registration_stage == 4)
        .ok_or_else(|| {
            E5FutureHoleFinaleError::Invariant("Stage 4 discharge is absent".to_owned())
        })?;
    let occurrence_ids_intentionally_not_equated =
        stage3_open.a3_instance_id != stage4_open.a3_instance_id;
    let same_constructor = stage3_open.structural_constructor == stage4_open.structural_constructor
        && stage3_open.structural_constructor.as_deref() == Some("former_eliminator");
    let same_open_body = stage3_open.body == stage4_open.body;
    let same_declared_motives = stage3_open.declared_motives == stage4_open.declared_motives;
    let same_source_and_output_kernel_types = stage3_open.source_kernel_type_json
        == stage4_open.source_kernel_type_json
        && stage3_open.output_kernel_type_json == stage4_open.output_kernel_type_json;
    let same_transparent_closure_tree = stage3_open.closure_tree == stage4_open.closure_tree;
    let both_support_local_before_extension = stage3_discharge.support_local_before_extension
        && stage4_discharge.support_local_before_extension;
    let both_expressions_preserved_by_extension = stage3_discharge
        .expression_preserved_across_prefix_extension
        && stage4_discharge.expression_preserved_across_prefix_extension;
    let both_motives_preserved_by_extension = stage3_discharge
        .motives_preserved_across_prefix_extension
        && stage4_discharge.motives_preserved_across_prefix_extension;
    let same_whole_step4_filler = stage3_discharge.filler_step == 4
        && stage4_discharge.filler_step == 4
        && stage3_discharge.package_level_filler == stage4_discharge.package_level_filler
        && stage3_discharge.filler_is_whole_sealed_entry_not_clause_reference
        && stage4_discharge.filler_is_whole_sealed_entry_not_clause_reference;
    let same_specialized_expression = stage3_discharge.specialized_expression
        == stage4_discharge.specialized_expression
        && stage3_discharge.specialized_expression_is_filler
        && stage4_discharge.specialized_expression_is_filler;
    let both_clause4_prime_instantiations_replayed = stage3_discharge
        .clause4_prime_instantiation_replayed
        && stage4_discharge.clause4_prime_instantiation_replayed;
    let semantic_scheme_transport_proved = occurrence_ids_intentionally_not_equated
        && same_constructor
        && same_open_body
        && same_declared_motives
        && same_source_and_output_kernel_types
        && same_transparent_closure_tree
        && both_support_local_before_extension
        && both_expressions_preserved_by_extension
        && both_motives_preserved_by_extension
        && same_whole_step4_filler
        && same_specialized_expression
        && both_clause4_prime_instantiations_replayed;
    if !semantic_scheme_transport_proved {
        return Err(E5FutureHoleFinaleError::Invariant(
            "Stage-3 to Stage-4 former-eliminator future-scheme transport failed".to_owned(),
        ));
    }
    let stage3_to_stage4_hash = tagged_hash(
        "Stage-3-to-Stage-4-future-scheme-transport",
        &(
            &stage3_open.a3_instance_id,
            &stage4_open.a3_instance_id,
            occurrence_ids_intentionally_not_equated,
            same_constructor,
            same_open_body,
            same_declared_motives,
            same_source_and_output_kernel_types,
            same_transparent_closure_tree,
            both_support_local_before_extension,
            both_expressions_preserved_by_extension,
            both_motives_preserved_by_extension,
            same_whole_step4_filler,
            same_specialized_expression,
            both_clause4_prime_instantiations_replayed,
            semantic_scheme_transport_proved,
        ),
    );
    let stage3_to_stage4_future_scheme_transport = E5Stage3To4FutureSchemeTransport {
        stage3_a3_instance_id: stage3_open.a3_instance_id.clone(),
        stage4_a3_instance_id: stage4_open.a3_instance_id.clone(),
        occurrence_ids_intentionally_not_equated,
        same_constructor,
        same_open_body,
        same_declared_motives,
        same_source_and_output_kernel_types,
        same_transparent_closure_tree,
        both_support_local_before_extension,
        both_expressions_preserved_by_extension,
        both_motives_preserved_by_extension,
        same_whole_step4_filler,
        same_specialized_expression,
        both_clause4_prime_instantiations_replayed,
        semantic_scheme_transport_proved,
        derivation_hash: stage3_to_stage4_hash,
    };
    let focus_projection_reproduces_entire_coarse_ladder = historical_rows
        .iter()
        .all(|row| row.focus_projection_reproduces_coarse_record);
    let f_fh1_every_historical_discharge_rederived = historical_rows
        .iter()
        .all(|row| row.every_standing_hole_discharged_by_specialization)
        && future.f_fh1_every_historical_discharge_replayed;
    let stage_three_wrinkle_reproduced = historical_rows
        .iter()
        .find(|row| row.stage == 3)
        .is_some_and(|row| row.stage_three_wrinkle_replayed);
    let coarse_o16_empty = historical_rows
        .iter()
        .find(|row| row.stage == 16)
        .is_some_and(|row| row.coarse_required_packages.is_empty());
    let semantic_o16_empty = inventory_partition_exact
        && underdetermined_ids.is_empty()
        && structural_instance_count == 0
        && stage16_window.constructor_evidence.is_empty();
    let semantic_o16_agrees_with_coarse_projection = semantic_o16_empty && coarse_o16_empty;
    if !focus_projection_reproduces_entire_coarse_ladder
        || !f_fh1_every_historical_discharge_rederived
        || !stage_three_wrinkle_reproduced
    {
        return Err(E5FutureHoleFinaleError::Invariant(
            "historical focus projection, F-FH1, or Stage-3 wrinkle regression failed".to_owned(),
        ));
    }

    let historical_hash = tagged_hash(
        "historical-ledger",
        &(
            &historical_rows,
            &stage3_to_stage4_future_scheme_transport,
            future.structural_hole_count,
            future.historical_discharges.len(),
            old_structural_gap_ids_joined_exactly,
            f_fh1_every_historical_discharge_rederived,
            focus_projection_reproduces_entire_coarse_ladder,
            stage_three_wrinkle_reproduced,
            coarse_o16_empty,
            semantic_o16_agrees_with_coarse_projection,
        ),
    );
    let historical = E5HistoricalLedgerAudit {
        rows: historical_rows,
        stage3_to_stage4_future_scheme_transport,
        historical_structural_hole_count: future.structural_hole_count,
        historical_discharge_count: future.historical_discharges.len(),
        old_structural_gap_ids_joined_exactly,
        f_fh1_every_historical_discharge_rederived,
        focus_projection_reproduces_entire_coarse_ladder,
        stage_three_wrinkle_reproduced,
        coarse_o16_empty,
        semantic_o16_agrees_with_coarse_projection,
        derivation_hash: historical_hash,
    };

    let frozen_hash = tagged_hash(
        "frozen-drift-audit",
        &(
            &transport.result_digest,
            transport.a3.grammar_archive_digest_valid,
            transport.a3.grammar_live_replay_valid,
            &transport.a3.grammar_live_replay_errors,
            transport.a3.j3_predecessor_archive_digest_valid,
            transport.a3.j3_predecessor_live_replay_valid,
            &transport.a3.j3_predecessor_live_replay_errors,
            transport.a3.frozen_surface_drift_bound_explicitly,
            false,
        ),
    );
    let frozen_drift = E5FinaleFrozenDriftAudit {
        transport_archive_replayed: true,
        grammar_archive_digest_valid: transport.a3.grammar_archive_digest_valid,
        grammar_live_replay_valid: transport.a3.grammar_live_replay_valid,
        grammar_live_replay_errors: transport.a3.grammar_live_replay_errors.clone(),
        j3_predecessor_archive_digest_valid: transport.a3.j3_predecessor_archive_digest_valid,
        j3_predecessor_live_replay_valid: transport.a3.j3_predecessor_live_replay_valid,
        j3_predecessor_live_replay_errors: transport.a3.j3_predecessor_live_replay_errors.clone(),
        frozen_surface_drift_bound_explicitly: transport.a3.frozen_surface_drift_bound_explicitly,
        historical_artifact_reinterpreted_after_drift: false,
        derivation_hash: frozen_hash,
    };

    let derivable_instance_ids = derivable_ids.iter().cloned().collect::<Vec<_>>();
    let stage16_hash = tagged_hash(
        "Stage-16-full-A3-D-audit",
        &(
            (
                true,
                caller_inventory_noninterference_replayed,
                &caller_noninterference.derivation_hash,
            ),
            (
                stage16_window.instances.len(),
                future.unary_actions.len(),
                transport.a3.direct_instances.len(),
                transport.a3.pointwise_instances.len(),
                higher_instance_count,
                structural_instance_count,
                higher_seed_rejected_for_no_typed_path_witness,
            ),
            (
                transport.a3.j3_regression_64_to_8,
                transport.a3.pointwise_instances_join_existing_j3_families,
                old_unary_gap_ids_joined_exactly,
            ),
            &membership_rows,
            &derivable_instance_ids,
            &underdetermined_ids,
            inventory_partition_exact,
            semantic_o16_empty,
        ),
    );
    let stage16 = E5FinaleStage16Audit {
        a3_grammar_live_replay_valid: true,
        caller_inventory_noninterference_replayed,
        caller_inventory_noninterference_hash: caller_noninterference.derivation_hash,
        a3_metadata_instance_count: stage16_window.instances.len(),
        unary_instance_count: future.unary_actions.len(),
        direct_chronological_instance_count: transport.a3.direct_instances.len(),
        pointwise_chronological_instance_count: transport.a3.pointwise_instances.len(),
        higher_instance_count,
        structural_instance_count,
        higher_seed_rejected_for_no_typed_path_witness,
        direct_regression_64_to_8: transport.a3.j3_regression_64_to_8,
        pointwise_joined_to_same_eight_families: transport
            .a3
            .pointwise_instances_join_existing_j3_families,
        old_unary_gap_ids_joined_exactly,
        membership_rows,
        derivable_instance_ids,
        underdetermined_instance_ids: underdetermined_ids,
        inventory_partition_exact,
        every_instance_d_decided: true,
        every_instance_derivable: semantic_o16_empty,
        full_a3_output_grammar_complete: inventory_partition_exact
            && future.future_hole_definition_complete,
        semantic_o16_empty,
        derivation_hash: stage16_hash,
    };

    let full_a3_grammar_available = stage16.full_a3_output_grammar_complete
        && historical.f_fh1_every_historical_discharge_rederived;
    let f1_executable = full_a3_grammar_available;
    let f1_executed = f1_executable;
    let demanded_but_underdetermined_instance =
        stage16.underdetermined_instance_ids.first().cloned();
    let f1_triggered = f1_executed && demanded_but_underdetermined_instance.is_some();
    let f1_excluded = f1_executed && demanded_but_underdetermined_instance.is_none();
    let theorem12_full_instance_granularity_proved = f1_excluded
        && stage16.semantic_o16_empty
        && historical.semantic_o16_agrees_with_coarse_projection;
    let f1_hash = tagged_hash(
        "F1-final-disposition",
        &(
            full_a3_grammar_available,
            f1_executable,
            f1_executed,
            stage16.a3_metadata_instance_count,
            stage16.derivable_instance_ids.len(),
            &demanded_but_underdetermined_instance,
            f1_triggered,
            f1_excluded,
            theorem12_full_instance_granularity_proved,
            stage16.semantic_o16_empty,
        ),
    );
    let f1_disposition = if f1_triggered {
        "F1_TRIGGERED_DEMANDED_BUT_UNDERDETERMINED_INSTANCE"
    } else if f1_excluded {
        "F1_EXCLUDED_FULL_A3_SEMANTIC_O16_EMPTY"
    } else {
        "F1_NOT_EXECUTABLE"
    };
    let f1 = E5FinaleF1Audit {
        full_a3_grammar_available,
        f1_executable,
        f1_executed,
        demanded_instance_count: stage16.a3_metadata_instance_count,
        derivable_instance_count: stage16.derivable_instance_ids.len(),
        demanded_but_underdetermined_instance,
        f1_triggered,
        f1_excluded,
        theorem12_full_instance_granularity_proved,
        theorem12_refuted: f1_triggered,
        semantic_o16_empty: Some(stage16.semantic_o16_empty),
        disposition: f1_disposition.to_owned(),
        derivation_hash: f1_hash,
    };

    let e5_complete = future.future_hole_definition_complete
        && stage16.full_a3_output_grammar_complete
        && historical.f_fh1_every_historical_discharge_rederived
        && f1.f1_executed;
    let t_bf2_authorized = e5_complete && f1.theorem12_full_instance_granularity_proved;
    let outcome = if f1.f1_triggered {
        "E5_COMPLETE_F1_TRIGGERED_THEOREM12_REFUTED"
    } else if f1.f1_excluded {
        "E5_COMPLETE_F1_EXCLUDED_SEMANTIC_O16_EMPTY"
    } else {
        "E5_INCOMPLETE_F1_NOT_EXECUTABLE"
    };
    let permitted_conclusion = if f1.f1_triggered {
        "The full A3 audit found a demanded-but-underdetermined Stage-16 instance; F1 fires and Theorem 12 is refuted at instance granularity. T-BF2 and the bridge remain closed."
    } else if f1.f1_excluded {
        "The full A3 Stage-16 demand inventory is instance-certified derivable; semantic O(16)=empty and Theorem 12's F1 refinement passes. This authorizes T-BF2 and satisfies the E-5 side of the bridge gate only."
    } else {
        "The full A3/F1 experiment has not obtained a total executable disposition; no downstream theorem is authorized."
    };
    let required_successor_action = if t_bf2_authorized {
        "Run T-BF2 and complete the independently preregistered R-T2 confluence artifact; authorize the bridge only if every remaining gate replays."
    } else {
        "Publish the F1 or future-hole obstruction and keep T-BF2 and the bridge closed."
    };
    let f_fh2_no_inferred_repaired_or_outcome_selected_motive =
        future.f_fh2_no_motive_inference_repair_or_outcome_selection;
    let f_fh3_no_credit_anchor_or_orbit_from_holes_or_fillings =
        future.f_fh3_zero_charge_and_no_credit_anchor_or_orbit;
    let f_fh4_named_gaps = future.f_fh4_named_gaps.clone();
    let mut certificate = E5FutureHoleFinaleCertificate {
        schema: E5_FUTURE_HOLE_FINALE_SCHEMA.to_owned(),
        date: E5_FUTURE_HOLE_FINALE_DATE.to_owned(),
        source_bindings: source_bindings(),
        desired_count_bar_winner_or_f1_verdict_used_as_definition_input: false,
        future_hole_certificate: future,
        frozen_drift,
        stage16,
        historical,
        f_fh2_no_inferred_repaired_or_outcome_selected_motive,
        f_fh3_no_credit_anchor_or_orbit_from_holes_or_fillings,
        f_fh4_named_gaps,
        f1,
        e5_complete,
        semantic_o16_certificate_issued: e5_complete,
        t_bf2_authorized,
        t_bf2_executed: false,
        bridge_prerequisite_satisfied: t_bf2_authorized,
        bridge_authorized: false,
        bridge_executed: false,
        bar_free_adoption_authorized: false,
        bar_free_law_adopted: false,
        halt_claim_issued: false,
        outcome: outcome.to_owned(),
        permitted_conclusion: permitted_conclusion.to_owned(),
        required_successor_action: required_successor_action.to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

fn failed_replay(error: impl Into<String>) -> E5FutureHoleFinaleReplay {
    E5FutureHoleFinaleReplay {
        valid: false,
        membership_count: 0,
        underdetermined_count: 0,
        historical_discharge_count: 0,
        semantic_o16_empty: false,
        f1_executed: false,
        f1_excluded: false,
        e5_complete: false,
        t_bf2_authorized: false,
        bridge_authorized: false,
        outcome: "REPLAY_FAILED".to_owned(),
        errors: vec![error.into()],
    }
}

pub fn replay_e5_future_hole_finale_certificate(
    certificate: &E5FutureHoleFinaleCertificate,
) -> E5FutureHoleFinaleReplay {
    let expected = match issue_e5_future_hole_finale_certificate() {
        Ok(expected) => expected,
        Err(error) => return failed_replay(error.to_string()),
    };
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("result digest mismatch".to_owned());
    }
    if certificate != &expected {
        errors.push("certificate differs from independent E-5 finale replay".to_owned());
    }
    E5FutureHoleFinaleReplay {
        valid: errors.is_empty(),
        membership_count: certificate.stage16.membership_rows.len(),
        underdetermined_count: certificate.stage16.underdetermined_instance_ids.len(),
        historical_discharge_count: certificate.historical.historical_discharge_count,
        semantic_o16_empty: certificate.stage16.semantic_o16_empty,
        f1_executed: certificate.f1.f1_executed,
        f1_excluded: certificate.f1.f1_excluded,
        e5_complete: certificate.e5_complete,
        t_bf2_authorized: certificate.t_bf2_authorized,
        bridge_authorized: certificate.bridge_authorized,
        outcome: certificate.outcome.clone(),
        errors,
    }
}

pub fn emit_e5_future_hole_finale_create_new(
    path: &Path,
) -> Result<E5FutureHoleFinaleReplay, E5FutureHoleFinaleError> {
    let certificate = issue_e5_future_hole_finale_certificate()?;
    let replay = replay_e5_future_hole_finale_certificate(&certificate);
    if !replay.valid {
        return Err(E5FutureHoleFinaleError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let bytes = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| E5FutureHoleFinaleError::Json(error.to_string()))?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| E5FutureHoleFinaleError::Io(error.to_string()))?;
    file.write_all(&bytes)
        .and_then(|_| file.write_all(b"\n"))
        .map_err(|error| E5FutureHoleFinaleError::Io(error.to_string()))?;
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::OnceLock;

    fn certificate() -> E5FutureHoleFinaleCertificate {
        static CERTIFICATE: OnceLock<E5FutureHoleFinaleCertificate> = OnceLock::new();
        CERTIFICATE
            .get_or_init(|| issue_e5_future_hole_finale_certificate().unwrap())
            .clone()
    }

    #[test]
    fn full_a3_partition_executes_f1_and_issues_semantic_o16() {
        let certificate = certificate();
        assert_eq!(certificate.stage16.a3_metadata_instance_count, 89);
        assert_eq!(certificate.stage16.unary_instance_count, 17);
        assert_eq!(certificate.stage16.direct_chronological_instance_count, 64);
        assert_eq!(
            certificate.stage16.pointwise_chronological_instance_count,
            8
        );
        assert_eq!(certificate.stage16.underdetermined_instance_ids.len(), 0);
        assert!(certificate.stage16.semantic_o16_empty);
        assert!(certificate.f1.f1_executed);
        assert!(certificate.f1.f1_excluded);
        assert!(certificate.f1.theorem12_full_instance_granularity_proved);
        assert!(certificate.e5_complete);
        assert!(certificate.t_bf2_authorized);
        assert!(!certificate.bridge_authorized);
        assert!(!certificate.halt_claim_issued);
        assert!(replay_e5_future_hole_finale_certificate(&certificate).valid);
    }

    #[test]
    fn historical_projection_replays_all_discharges_and_the_stage_three_wrinkle() {
        let certificate = certificate();
        assert_eq!(certificate.historical.rows.len(), 16);
        assert_eq!(certificate.historical.historical_structural_hole_count, 13);
        assert_eq!(certificate.historical.historical_discharge_count, 13);
        assert!(
            certificate
                .historical
                .f_fh1_every_historical_discharge_rederived
        );
        assert!(
            certificate
                .historical
                .focus_projection_reproduces_entire_coarse_ladder
        );
        assert!(certificate.historical.stage_three_wrinkle_reproduced);
        let stage3 = certificate
            .historical
            .rows
            .iter()
            .find(|row| row.stage == 3)
            .unwrap();
        assert_eq!(stage3.coarse_required_packages, ["former_eliminator"]);
        assert!(stage3.projected_focus.is_none());
        assert!(stage3.demand_precedes_jurisdiction);
    }

    #[test]
    fn semantic_o16_and_credit_forgery_is_rejected() {
        let mut forged = certificate();
        forged.stage16.semantic_o16_empty = false;
        forged.future_hole_certificate.unary_actions[0]
            .charge
            .credit_minted = true;
        forged.result_digest = certificate_digest(&forged);
        assert!(!replay_e5_future_hole_finale_certificate(&forged).valid);
    }
}
