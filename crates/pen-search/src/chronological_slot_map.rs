//! F-SM1: replay the adopted chronological interface slot map.
//!
//! The live sweep is completed and sealed before either historical archive
//! is parsed. Archives therefore witness ID/count stability only; they cannot
//! select a map or repair a failed derivation.

use crate::branch_invariance_finale::{
    BranchDMembershipDisposition, issue_branch_chronological_membership_audit,
};
use crate::e5_future_hole_finale_v2::{
    E5_FUTURE_HOLE_FINALE_V2_SCHEMA, E5FutureHoleFinaleV2Certificate,
};
use pen_core::hash::blake3_hex;
use pen_eval::a3_demand_grammar::{
    A3DemandOutputType, A3RuleConstructor, CHRONOLOGICAL_INTERFACE_SLOT_MAP_V1,
    generate_a3_window_for_exact_prefix_unbounded, replay_chronological_interface_slot_map,
};
use pen_type::elaborate::SealedSignature;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const CHRONOLOGICAL_SLOT_MAP_CERTIFICATE_SCHEMA: &str =
    "chronological-interface-slot-map-f-sm1-v1";
pub const CHRONOLOGICAL_SLOT_MAP_CERTIFICATE_DATE: &str = "2026-07-22";

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/bi0_prerequisites_adjudication.md");
const BI0_ARCHIVE_BYTES: &[u8] = include_bytes!("../../../docs/BI_REGRESSION_CERTIFICATE.json");
const E5_ARCHIVE_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_e5_future_hole_finale_v2_dependent_context.json");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const SUBSTITUTION_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/substitution.rs");
const FINALE_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_finale.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("chronological_slot_map.rs");
const FROZEN_BI0_ARCHIVE_SHA256: &str =
    "5601c530fe74eae2d5e1fecb49ec06c44d84d8f44b94e7f87384c4b973cf0fda";
const FROZEN_E5_ARCHIVE_SHA256: &str =
    "c60a28c82370e88829dfd6457ae52814d4523fcf2b2012378814a4cf0dd32be5";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SlotMapSourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapInstanceAudit {
    pub stage: u32,
    pub instance_id: String,
    pub scheme_id: String,
    pub older_step: u32,
    pub older_clause: u16,
    pub newest_step: u32,
    pub newest_clause: u16,
    pub declared_arity: u32,
    pub declared_assignments: Vec<(u32, u32)>,
    pub declaration_hash: String,
    pub declaration_replayed: bool,
    pub inferred_from_derivation_success: bool,
    pub instance_override_permitted: bool,
    pub zero_charge: bool,
    pub independently_sealed_discharge: bool,
    pub membership_derivation_hash: Option<String>,
    pub membership_derived: bool,
    pub exact_substitution_replayed: bool,
    pub source_order_negative_control_rejected: bool,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapWindowAudit {
    pub stage: u32,
    pub exact_prefix_signature_digest: String,
    pub chronological_instance_count: usize,
    pub replayed_instance_count: usize,
    pub instance_ids: Vec<String>,
    pub instances: Vec<ChronologicalSlotMapInstanceAudit>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapCertificate {
    pub schema: String,
    pub date: String,
    pub adopted_rule: String,
    pub source_bindings: Vec<SlotMapSourceBinding>,
    pub sealed_historical_window_count: usize,
    pub windows: Vec<ChronologicalSlotMapWindowAudit>,
    pub historical_chronological_instance_count: usize,
    pub historical_chronological_replayed_count: usize,
    pub historical_chronological_instance_ids_unique: bool,
    pub sealed_chronological_discharge_scope: String,
    pub sealed_chronological_discharge_count: usize,
    pub sealed_chronological_discharge_replayed_count: usize,
    pub sealed_chronological_discharge_instance_ids: Vec<String>,
    pub raw_a3_instances_not_reclassified_as_discharges: bool,
    pub every_declaration_order_preserving_identity: bool,
    pub every_declaration_zero_charge: bool,
    pub every_membership_derived: bool,
    pub alternative_slot_map_permutation_trial_count: usize,
    pub instance_override_count: usize,
    pub outcome_or_verdict_used_as_definition_input: bool,
    pub archives_parsed_only_after_live_sweep_sealed: bool,
    pub archives_are_regression_witnesses_not_authority: bool,
    pub frozen_bi0_nine_gap_archive_byte_pin_valid: bool,
    pub frozen_e5_archive_byte_pin_valid: bool,
    pub frozen_e5_archive_digest_valid: bool,
    pub frozen_stage16_inventory_count: usize,
    pub live_stage16_inventory_count: usize,
    pub frozen_stage16_instance_ids: Vec<String>,
    pub live_stage16_instance_ids: Vec<String>,
    pub frozen_stage16_89_id_surface_exact: bool,
    pub formerly_gapped_witness: String,
    pub formerly_gapped_expected_instance_ids: Vec<String>,
    pub formerly_gapped_observed_instance_ids: Vec<String>,
    pub formerly_gapped_nine_ids_exact: bool,
    pub formerly_gapped_nine_all_derived: bool,
    pub f_sm1_passed: bool,
    pub f_sm2_no_inference_or_override: bool,
    pub non_enacted_branch_continuation_count: usize,
    pub bi0_rerun_required: bool,
    pub bi1_or_cone_authorized: bool,
    pub conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapReplay {
    pub valid: bool,
    pub historical_instance_count: usize,
    pub formerly_gapped_derived_count: usize,
    pub frozen_stage16_ids_exact: bool,
    pub f_sm1_passed: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ChronologicalSlotMapError {
    #[error("historical slot-map invariant failed: {0}")]
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

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CHRONOLOGICAL_SLOT_MAP_CERTIFICATE_SCHEMA, domain, value))
        .expect("slot-map evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn external_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("archive evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<SlotMapSourceBinding> {
    [
        (
            "docs/bi0_prerequisites_adjudication.md",
            "adopted_identity_map_and_F_SM1_F_SM2",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/BI_REGRESSION_CERTIFICATE.json",
            "frozen_nine_gap_ID_regression_witness_only",
            BI0_ARCHIVE_BYTES,
        ),
        (
            "docs/schema2_e5_future_hole_finale_v2_dependent_context.json",
            "frozen_stage16_89_ID_regression_witness_only",
            E5_ARCHIVE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "rule_level_enumerated_declaration",
            A3_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/substitution.rs",
            "capture_safe_structural_substitution_replay",
            SUBSTITUTION_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/branch_invariance_finale.rs",
            "intrinsic_chronological_membership_issuer",
            FINALE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/chronological_slot_map.rs",
            "F_SM1_historical_sweep_and_replay",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| SlotMapSourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn certificate_digest(certificate: &ChronologicalSlotMapCertificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

fn frozen_e5_digest_valid(certificate: &E5FutureHoleFinaleV2Certificate) -> bool {
    let mut projection = certificate.clone();
    let observed = projection.result_digest.clone();
    projection.result_digest.clear();
    observed == external_tagged_hash(E5_FUTURE_HOLE_FINALE_V2_SCHEMA, "certificate", &projection)
}

fn exact_prefix(full: &SealedSignature, stage: u32) -> SealedSignature {
    SealedSignature::from_telescopes(
        full.entries()
            .iter()
            .filter(|entry| entry.step < stage)
            .map(|entry| (entry.step, entry.telescope.clone()))
            .collect(),
    )
}

fn source_coordinate(
    window: &pen_eval::a3_demand_grammar::A3HistoricalWindow,
    anchor: &str,
) -> Result<(u32, u16), ChronologicalSlotMapError> {
    window
        .typed_sources
        .iter()
        .find(|source| source.anchor_id == anchor)
        .map(|source| (source.step, source.clause_index))
        .ok_or_else(|| {
            ChronologicalSlotMapError::Invariant(format!(
                "source anchor {anchor} is absent from Stage {}",
                window.stage
            ))
        })
}

fn archive_ids(value: &Value, field: &str) -> Result<Vec<String>, ChronologicalSlotMapError> {
    let rows = value.get(field).and_then(Value::as_array).ok_or_else(|| {
        ChronologicalSlotMapError::Json(format!("archive field {field} is absent"))
    })?;
    rows.iter()
        .map(|row| {
            row.as_str().map(str::to_owned).ok_or_else(|| {
                ChronologicalSlotMapError::Json(format!(
                    "archive field {field} contains a non-string"
                ))
            })
        })
        .collect()
}

/// Issue F-SM1. The function returns on the first live replay error and never
/// tries another permutation.
pub fn issue_chronological_slot_map_certificate()
-> Result<ChronologicalSlotMapCertificate, ChronologicalSlotMapError> {
    let full = SealedSignature::genesis_del_h15();
    let mut windows = Vec::new();
    let mut all_live_ids = BTreeSet::new();
    let mut generated_windows = Vec::new();

    // Seal the complete live sweep before reading either archive below.
    for stage in 1..=16 {
        let prefix = exact_prefix(&full, stage);
        let window = generate_a3_window_for_exact_prefix_unbounded(&prefix, stage)
            .map_err(|error| ChronologicalSlotMapError::Invariant(error.to_string()))?;
        let mut instances = Vec::new();
        for instance in &window.instances {
            let scheme = window
                .schemes
                .iter()
                .find(|scheme| scheme.scheme_id == instance.scheme_id)
                .ok_or_else(|| {
                    ChronologicalSlotMapError::Invariant(format!(
                        "orphan instance {} at Stage {stage}",
                        instance.instance_id
                    ))
                })?;
            if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison {
                continue;
            }
            let A3DemandOutputType::ChronologicalInteraction {
                interface_slot_map, ..
            } = &scheme.required_output
            else {
                return Err(ChronologicalSlotMapError::Invariant(format!(
                    "chronological scheme {} has the wrong output",
                    scheme.scheme_id
                )));
            };
            let newest_anchor = instance.source_anchor_ids.get(1).ok_or_else(|| {
                ChronologicalSlotMapError::Invariant(format!(
                    "chronological instance {} has no newest source",
                    instance.instance_id
                ))
            })?;
            let newest_source = window
                .typed_sources
                .iter()
                .find(|source| &source.anchor_id == newest_anchor)
                .ok_or_else(|| {
                    ChronologicalSlotMapError::Invariant(format!(
                        "newest source {newest_anchor} does not resolve"
                    ))
                })?;
            let declared_arity = u32::try_from(
                newest_source.canonical_presentation.parameters.len(),
            )
            .map_err(|_| {
                ChronologicalSlotMapError::Invariant(
                    "chronological arity does not fit u32".to_owned(),
                )
            })?;
            replay_chronological_interface_slot_map(interface_slot_map, declared_arity).map_err(
                |error| {
                    ChronologicalSlotMapError::Invariant(format!(
                        "Stage {stage} instance {} slot-map replay failed: {error}",
                        instance.instance_id
                    ))
                },
            )?;
            let (older_step, older_clause) = source_coordinate(
                &window,
                instance.source_anchor_ids.first().ok_or_else(|| {
                    ChronologicalSlotMapError::Invariant(format!(
                        "chronological instance {} has no older source",
                        instance.instance_id
                    ))
                })?,
            )?;
            let (newest_step, newest_clause) = source_coordinate(&window, newest_anchor)?;
            all_live_ids.insert(instance.instance_id.clone());
            let declared_assignments = interface_slot_map
                .assignments
                .iter()
                .map(|assignment| (assignment.interface_slot, assignment.parameter))
                .collect::<Vec<_>>();
            let zero_charge =
                interface_slot_map.kappa_charge == 0 && interface_slot_map.nu_charge == 0;
            let mut audit = ChronologicalSlotMapInstanceAudit {
                stage,
                instance_id: instance.instance_id.clone(),
                scheme_id: scheme.scheme_id.clone(),
                older_step,
                older_clause,
                newest_step,
                newest_clause,
                declared_arity,
                declared_assignments,
                declaration_hash: interface_slot_map.declaration_hash.clone(),
                declaration_replayed: true,
                inferred_from_derivation_success: interface_slot_map
                    .inferred_from_derivation_success,
                instance_override_permitted: interface_slot_map.instance_override_permitted,
                zero_charge,
                independently_sealed_discharge: false,
                membership_derivation_hash: None,
                membership_derived: false,
                exact_substitution_replayed: false,
                source_order_negative_control_rejected: false,
                derivation_hash: String::new(),
            };
            audit.derivation_hash = tagged_hash("instance-audit", &audit);
            instances.push(audit);
        }
        let instance_ids = instances
            .iter()
            .map(|instance| instance.instance_id.clone())
            .collect::<Vec<_>>();
        let replayed_instance_count = instances
            .iter()
            .filter(|instance| instance.declaration_replayed && instance.zero_charge)
            .count();
        let mut audit = ChronologicalSlotMapWindowAudit {
            stage,
            exact_prefix_signature_digest: prefix.digest().to_owned(),
            chronological_instance_count: instances.len(),
            replayed_instance_count,
            instance_ids,
            instances,
            derivation_hash: String::new(),
        };
        audit.derivation_hash = tagged_hash("window-audit", &audit);
        windows.push(audit);
        generated_windows.push((prefix, window));
    }

    let historical_chronological_instance_count = windows
        .iter()
        .map(|window| window.chronological_instance_count)
        .sum::<usize>();
    let historical_chronological_replayed_count = windows
        .iter()
        .map(|window| window.replayed_instance_count)
        .sum::<usize>();

    // Archive parsing begins only after the complete raw-instance declaration
    // sweep above is fixed. The archived corpus, not current proof success,
    // selects which instances are independently sealed discharges.
    let e5_archive: E5FutureHoleFinaleV2Certificate = serde_json::from_slice(E5_ARCHIVE_BYTES)
        .map_err(|error| ChronologicalSlotMapError::Json(error.to_string()))?;
    let frozen_e5_archive_byte_pin_valid = sha256_hex(E5_ARCHIVE_BYTES) == FROZEN_E5_ARCHIVE_SHA256;
    let frozen_e5_archive_digest_valid = frozen_e5_digest_valid(&e5_archive);
    if !frozen_e5_archive_byte_pin_valid || !frozen_e5_archive_digest_valid {
        return Err(ChronologicalSlotMapError::Invariant(
            "the independently frozen E-5 discharge corpus failed its self-digest".to_owned(),
        ));
    }
    let sealed_chronological_discharge_instance_ids = e5_archive
        .stage16
        .membership_rows
        .iter()
        .filter(|row| row.rule.starts_with("chronological_comparison::"))
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    if sealed_chronological_discharge_instance_ids.len() != 72 {
        return Err(ChronologicalSlotMapError::Invariant(format!(
            "frozen chronological discharge corpus has {} IDs, expected 72",
            sealed_chronological_discharge_instance_ids.len()
        )));
    }

    let mut sealed_chronological_discharge_replayed_count = 0_usize;
    for (prefix, window) in &generated_windows {
        for instance in &window.instances {
            if !sealed_chronological_discharge_instance_ids.contains(&instance.instance_id) {
                continue;
            }
            let scheme = window
                .schemes
                .iter()
                .find(|scheme| scheme.scheme_id == instance.scheme_id)
                .ok_or_else(|| {
                    ChronologicalSlotMapError::Invariant(format!(
                        "sealed discharge {} is orphaned",
                        instance.instance_id
                    ))
                })?;
            if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison {
                return Err(ChronologicalSlotMapError::Invariant(format!(
                    "sealed chronological discharge {} joined a non-chronological scheme",
                    instance.instance_id
                )));
            }
            let (membership, _) = issue_branch_chronological_membership_audit(
                prefix,
                window.stage.saturating_sub(1),
                window,
                scheme,
                instance,
            )
            .map_err(|error| {
                ChronologicalSlotMapError::Invariant(format!(
                    "F-SM1 stopped at sealed Stage {} discharge {}: {error}",
                    window.stage, instance.instance_id
                ))
            })?;
            let membership_derived = matches!(
                membership.disposition,
                BranchDMembershipDisposition::Derivable
            );
            if !membership_derived
                || !membership.exact_substitution_from_sealed_preimage_replayed
                || !membership.source_or_evidence_swap_rejected
            {
                return Err(ChronologicalSlotMapError::Invariant(format!(
                    "sealed chronological discharge {} did not replay every proof premise",
                    instance.instance_id
                )));
            }
            let audit = windows
                .iter_mut()
                .find(|audit| audit.stage == window.stage)
                .and_then(|audit| {
                    audit
                        .instances
                        .iter_mut()
                        .find(|audit| audit.instance_id == instance.instance_id)
                })
                .ok_or_else(|| {
                    ChronologicalSlotMapError::Invariant(format!(
                        "sealed discharge {} is absent from the declaration sweep",
                        instance.instance_id
                    ))
                })?;
            audit.independently_sealed_discharge = true;
            audit.membership_derivation_hash = Some(membership.derivation_hash);
            audit.membership_derived = true;
            audit.exact_substitution_replayed = true;
            audit.source_order_negative_control_rejected = true;
            audit.derivation_hash.clear();
            audit.derivation_hash = tagged_hash("instance-audit", audit);
            sealed_chronological_discharge_replayed_count += 1;
        }
    }
    for window in &mut windows {
        window.derivation_hash.clear();
        window.derivation_hash = tagged_hash("window-audit", window);
    }
    if sealed_chronological_discharge_replayed_count
        != sealed_chronological_discharge_instance_ids.len()
    {
        return Err(ChronologicalSlotMapError::Invariant(format!(
            "replayed {sealed_chronological_discharge_replayed_count}/{} sealed chronological discharges",
            sealed_chronological_discharge_instance_ids.len()
        )));
    }
    let mut frozen_stage16_instance_ids = e5_archive
        .stage16
        .membership_rows
        .iter()
        .map(|row| row.a3_instance_id.clone())
        .collect::<Vec<_>>();
    frozen_stage16_instance_ids.sort();
    if !windows.iter().any(|window| window.stage == 16) {
        return Err(ChronologicalSlotMapError::Invariant(
            "Stage 16 is absent".to_owned(),
        ));
    }
    // Add the non-chronological IDs to compare the complete frozen 89 surface.
    let stage16_prefix = exact_prefix(&full, 16);
    let stage16_window = generate_a3_window_for_exact_prefix_unbounded(&stage16_prefix, 16)
        .map_err(|error| ChronologicalSlotMapError::Invariant(error.to_string()))?;
    let mut live_stage16_instance_ids = stage16_window
        .instances
        .iter()
        .map(|row| row.instance_id.clone())
        .collect::<Vec<_>>();
    live_stage16_instance_ids.sort();
    let frozen_stage16_89_id_surface_exact = frozen_e5_archive_digest_valid
        && frozen_stage16_instance_ids.len() == 89
        && live_stage16_instance_ids.len() == 89
        && frozen_stage16_instance_ids == live_stage16_instance_ids;

    let bi0_value: Value = serde_json::from_slice(BI0_ARCHIVE_BYTES)
        .map_err(|error| ChronologicalSlotMapError::Json(error.to_string()))?;
    let frozen_bi0_nine_gap_archive_byte_pin_valid =
        sha256_hex(BI0_ARCHIVE_BYTES) == FROZEN_BI0_ARCHIVE_SHA256;
    let mut formerly_gapped_expected_instance_ids =
        archive_ids(&bi0_value, "enacted_finale_issuer_gap_instance_ids")?;
    formerly_gapped_expected_instance_ids.sort();
    let expected_gap_set = formerly_gapped_expected_instance_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut formerly_gapped_observed_instance_ids = windows
        .iter()
        .find(|window| window.stage == 16)
        .expect("Stage 16 was checked above")
        .instances
        .iter()
        .filter(|row| expected_gap_set.contains(&row.instance_id))
        .filter(|row| {
            row.older_step == 14
                && row.older_clause <= 8
                && row.newest_step == 15
                && row.newest_clause == 6
                && row.declared_arity == 2
                && row.membership_derived
        })
        .map(|row| row.instance_id.clone())
        .collect::<Vec<_>>();
    formerly_gapped_observed_instance_ids.sort();
    let formerly_gapped_nine_ids_exact = formerly_gapped_expected_instance_ids.len() == 9
        && formerly_gapped_observed_instance_ids == formerly_gapped_expected_instance_ids;
    let formerly_gapped_nine_all_derived = formerly_gapped_nine_ids_exact;

    let every_declaration_order_preserving_identity = windows.iter().all(|window| {
        window.instances.iter().all(|instance| {
            instance.declaration_replayed
                && instance.declared_assignments.iter().enumerate().all(
                    |(index, &(slot, parameter))| slot == index as u32 + 1 && parameter == slot,
                )
        })
    });
    let every_declaration_zero_charge = windows
        .iter()
        .flat_map(|window| &window.instances)
        .all(|instance| instance.zero_charge);
    let every_membership_derived = sealed_chronological_discharge_replayed_count > 0
        && sealed_chronological_discharge_replayed_count
            == sealed_chronological_discharge_instance_ids.len();
    let f_sm2_no_inference_or_override =
        windows
            .iter()
            .flat_map(|window| &window.instances)
            .all(|instance| {
                !instance.inferred_from_derivation_success && !instance.instance_override_permitted
            });
    let f_sm1_passed = every_declaration_order_preserving_identity
        && every_declaration_zero_charge
        && every_membership_derived
        && f_sm2_no_inference_or_override
        && all_live_ids.len() == historical_chronological_instance_count
        && frozen_bi0_nine_gap_archive_byte_pin_valid
        && frozen_e5_archive_byte_pin_valid
        && frozen_stage16_89_id_surface_exact
        && formerly_gapped_nine_all_derived;
    if !f_sm1_passed {
        return Err(ChronologicalSlotMapError::Invariant(
            "F-SM1 aggregate regression failed".to_owned(),
        ));
    }

    let mut certificate = ChronologicalSlotMapCertificate {
        schema: CHRONOLOGICAL_SLOT_MAP_CERTIFICATE_SCHEMA.to_owned(),
        date: CHRONOLOGICAL_SLOT_MAP_CERTIFICATE_DATE.to_owned(),
        adopted_rule: CHRONOLOGICAL_INTERFACE_SLOT_MAP_V1.to_owned(),
        source_bindings: source_bindings(),
        sealed_historical_window_count: windows.len(),
        windows,
        historical_chronological_instance_count,
        historical_chronological_replayed_count,
        historical_chronological_instance_ids_unique: all_live_ids.len()
            == historical_chronological_instance_count,
        sealed_chronological_discharge_scope: "the 72 chronological D-membership rows of the independently frozen Stage-16 E-5 surface; no independently sealed cross-window membership corpus is asserted".to_owned(),
        sealed_chronological_discharge_count: sealed_chronological_discharge_instance_ids.len(),
        sealed_chronological_discharge_replayed_count,
        sealed_chronological_discharge_instance_ids: sealed_chronological_discharge_instance_ids
            .into_iter()
            .collect(),
        raw_a3_instances_not_reclassified_as_discharges: true,
        every_declaration_order_preserving_identity,
        every_declaration_zero_charge,
        every_membership_derived,
        alternative_slot_map_permutation_trial_count: 0,
        instance_override_count: 0,
        outcome_or_verdict_used_as_definition_input: false,
        archives_parsed_only_after_live_sweep_sealed: true,
        archives_are_regression_witnesses_not_authority: true,
        frozen_bi0_nine_gap_archive_byte_pin_valid,
        frozen_e5_archive_byte_pin_valid,
        frozen_e5_archive_digest_valid,
        frozen_stage16_inventory_count: frozen_stage16_instance_ids.len(),
        live_stage16_inventory_count: live_stage16_instance_ids.len(),
        frozen_stage16_instance_ids,
        live_stage16_instance_ids,
        frozen_stage16_89_id_surface_exact,
        formerly_gapped_witness: "preserved failed BI-0 v1 artifact docs/BI_REGRESSION_CERTIFICATE.json, byte-pinned before the versioned BI-0 v2 rerun".to_owned(),
        formerly_gapped_expected_instance_ids,
        formerly_gapped_observed_instance_ids,
        formerly_gapped_nine_ids_exact,
        formerly_gapped_nine_all_derived,
        f_sm1_passed,
        f_sm2_no_inference_or_override,
        non_enacted_branch_continuation_count: 0,
        bi0_rerun_required: true,
        bi1_or_cone_authorized: false,
        conclusion: "F-SM1 passes: every raw historical chronological instance carries the declared identity map, every independently sealed chronological discharge replays typed membership, the nine BI-0 issuer gaps derive, and the frozen Stage-16 89-ID surface is unchanged. This certificate opens only the BI-0 rerun.".to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

pub fn replay_chronological_slot_map_certificate(
    certificate: &ChronologicalSlotMapCertificate,
) -> ChronologicalSlotMapReplay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("certificate digest mismatch".to_owned());
    }
    match issue_chronological_slot_map_certificate() {
        Ok(expected) if expected == *certificate => {}
        Ok(_) => errors.push("certificate differs from independent reissuance".to_owned()),
        Err(error) => errors.push(format!("independent reissuance failed: {error}")),
    }
    ChronologicalSlotMapReplay {
        valid: errors.is_empty(),
        historical_instance_count: certificate.historical_chronological_instance_count,
        formerly_gapped_derived_count: certificate.formerly_gapped_observed_instance_ids.len(),
        frozen_stage16_ids_exact: certificate.frozen_stage16_89_id_surface_exact,
        f_sm1_passed: certificate.f_sm1_passed,
        errors,
    }
}

pub fn replay_chronological_slot_map_json(text: &str) -> ChronologicalSlotMapReplay {
    match serde_json::from_str::<ChronologicalSlotMapCertificate>(text) {
        Ok(certificate) => replay_chronological_slot_map_certificate(&certificate),
        Err(error) => ChronologicalSlotMapReplay {
            valid: false,
            historical_instance_count: 0,
            formerly_gapped_derived_count: 0,
            frozen_stage16_ids_exact: false,
            f_sm1_passed: false,
            errors: vec![format!("JSON parse failed: {error}")],
        },
    }
}

pub fn render_chronological_slot_map_report(
    certificate: &ChronologicalSlotMapCertificate,
) -> String {
    format!(
        "# Chronological interface slot-map F-SM1 result\n\n- Schema: `{}`\n- Date: {}\n- Digest: `{}`\n- Historical windows swept: {}\n- Raw chronological instances with declaration replay: {}/{}\n- Independently sealed chronological discharges with typed membership replay: {}/{}\n- Sealed discharge scope: {}\n- Frozen Stage-16 instance IDs unchanged: `{}` ({}/89)\n- Former BI-0 gaps derived: {}/9\n- Former-gap witness: {}\n- Order-preserving declaration / zero charge: `{}` / `{}`\n- Alternative slot-map permutations trialed: {}\n- Non-enacted continuations run: {}\n- F-SM1: `{}`\n\nThe raw A3 instance universe and the independently archived discharge corpus are distinct; no raw seed was promoted by current proof success. No independently sealed cross-window membership corpus is claimed.\n\n{}\n",
        certificate.schema,
        certificate.date,
        certificate.result_digest,
        certificate.sealed_historical_window_count,
        certificate.historical_chronological_replayed_count,
        certificate.historical_chronological_instance_count,
        certificate.sealed_chronological_discharge_replayed_count,
        certificate.sealed_chronological_discharge_count,
        certificate.sealed_chronological_discharge_scope,
        certificate.frozen_stage16_89_id_surface_exact,
        certificate.live_stage16_inventory_count,
        certificate.formerly_gapped_observed_instance_ids.len(),
        certificate.formerly_gapped_witness,
        certificate.every_declaration_order_preserving_identity,
        certificate.every_declaration_zero_charge,
        certificate.alternative_slot_map_permutation_trial_count,
        certificate.non_enacted_branch_continuation_count,
        certificate.f_sm1_passed,
        certificate.conclusion,
    )
}

pub fn emit_chronological_slot_map_create_new(
    json_path: &Path,
    report_path: &Path,
) -> Result<ChronologicalSlotMapReplay, ChronologicalSlotMapError> {
    if json_path.exists() || report_path.exists() {
        return Err(ChronologicalSlotMapError::Io(
            "create-new target already exists".to_owned(),
        ));
    }
    let certificate = issue_chronological_slot_map_certificate()?;
    let replay = replay_chronological_slot_map_certificate(&certificate);
    if !replay.valid {
        return Err(ChronologicalSlotMapError::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| ChronologicalSlotMapError::Json(error.to_string()))?;
    let report = render_chronological_slot_map_report(&certificate);
    let mut json_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(json_path)
        .map_err(|error| ChronologicalSlotMapError::Io(error.to_string()))?;
    json_file
        .write_all(&json)
        .map_err(|error| ChronologicalSlotMapError::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(report_path)
        .map_err(|error| ChronologicalSlotMapError::Io(error.to_string()))?;
    report_file
        .write_all(report.as_bytes())
        .map_err(|error| ChronologicalSlotMapError::Io(error.to_string()))?;
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn v1_is_burned_when_strict_closure_specialization_finds_a_named_gap() {
        let error = issue_chronological_slot_map_certificate()
            .expect_err("v1 must not survive the proof-strength successor");
        assert!(error.to_string().contains("BI_CHRONOLOGICAL_"));
    }

    #[test]
    #[ignore = "v1 artifact is preserved as burned evidence; mutation replay moved to v2"]
    fn certificate_mutations_fail_definition_replay() {
        let certificate = issue_chronological_slot_map_certificate().expect("F-SM1 issues");

        let mut id_mutation = certificate.clone();
        id_mutation.windows[15].instance_ids[0].push_str("::forged");
        id_mutation.result_digest = certificate_digest(&id_mutation);
        assert!(!replay_chronological_slot_map_certificate(&id_mutation).valid);

        let mut map_mutation = certificate.clone();
        map_mutation.windows[15].instances[0].declared_assignments[0] = (1, 2);
        map_mutation.windows[15].instances[0].derivation_hash =
            tagged_hash("instance-audit", &map_mutation.windows[15].instances[0]);
        map_mutation.windows[15].derivation_hash =
            tagged_hash("window-audit", &map_mutation.windows[15]);
        map_mutation.result_digest = certificate_digest(&map_mutation);
        assert!(!replay_chronological_slot_map_certificate(&map_mutation).valid);

        let mut inference_mutation = certificate.clone();
        inference_mutation.windows[15].instances[0].inferred_from_derivation_success = true;
        inference_mutation.windows[15].instances[0].derivation_hash = tagged_hash(
            "instance-audit",
            &inference_mutation.windows[15].instances[0],
        );
        inference_mutation.windows[15].derivation_hash =
            tagged_hash("window-audit", &inference_mutation.windows[15]);
        inference_mutation.result_digest = certificate_digest(&inference_mutation);
        assert!(!replay_chronological_slot_map_certificate(&inference_mutation).valid);

        let mut verdict_mutation = certificate;
        verdict_mutation.f_sm1_passed = false;
        verdict_mutation.result_digest = certificate_digest(&verdict_mutation);
        assert!(!replay_chronological_slot_map_certificate(&verdict_mutation).valid);
    }

    #[test]
    #[ignore = "v1 create-new is forbidden after the proof-strength audit burn"]
    fn create_new_emits_once_and_the_json_replays() {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        let base = std::env::temp_dir();
        let json_path = base.join(format!(
            "atomic-chronological-slot-map-{nonce}-{}.json",
            std::process::id()
        ));
        let report_path = base.join(format!(
            "atomic-chronological-slot-map-{nonce}-{}.md",
            std::process::id()
        ));
        let replay = emit_chronological_slot_map_create_new(&json_path, &report_path)
            .expect("first create-new succeeds");
        assert!(replay.valid);
        let text = std::fs::read_to_string(&json_path).expect("JSON artifact exists");
        assert!(replay_chronological_slot_map_json(&text).valid);
        assert!(report_path.is_file());
        assert!(matches!(
            emit_chronological_slot_map_create_new(&json_path, &report_path),
            Err(ChronologicalSlotMapError::Io(_))
        ));
        std::fs::remove_file(&json_path).expect("remove temporary JSON artifact");
        std::fs::remove_file(&report_path).expect("remove temporary report artifact");
    }
}
