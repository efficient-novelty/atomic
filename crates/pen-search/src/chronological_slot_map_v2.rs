//! Versioned F-SM1 successor with proof-strength chronological Internality.
//!
//! The slot-map declaration remains the adopted order-preserving identity.
//! What changes from v1 is the membership premise: a chronological discharge
//! is `Derived` only after the exact newest-family closure derivation, every
//! exact closed assignment image, and the specialized closure result replay.
//! Unsupported open images are retained as named gaps; they are never promoted
//! from typing to `Internal`.

use crate::branch_invariance_finale::{
    BranchDMembershipDisposition, issue_branch_chronological_membership_audit,
};
use crate::e5_future_hole_finale_v2::{
    E5_FUTURE_HOLE_FINALE_V2_SCHEMA, E5FutureHoleFinaleV2Certificate,
};
use pen_core::hash::blake3_hex;
use pen_eval::a3_demand_grammar::{
    A3DemandOutputType, A3HistoricalWindow, A3RuleConstructor, CHRONOLOGICAL_INTERFACE_SLOT_MAP_V1,
    generate_a3_window_for_exact_prefix_unbounded, replay_chronological_interface_slot_map,
};
use pen_type::elaborate::SealedSignature;
use pen_type::motive_parametric_coherence_v2::MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const CHRONOLOGICAL_SLOT_MAP_V2_SCHEMA: &str = "chronological-interface-slot-map-f-sm1-v2";
pub const CHRONOLOGICAL_SLOT_MAP_V2_DATE: &str = "2026-07-22";
pub const CHRONOLOGICAL_SLOT_MAP_V2_CERTIFICATE_NAME: &str =
    "chronological_interface_slot_map_v2.json";
pub const CHRONOLOGICAL_SLOT_MAP_V2_REPORT_NAME: &str =
    "CHRONOLOGICAL_INTERFACE_SLOT_MAP_V2_RESULT.md";
pub const BI_CHRONOLOGICAL_PREMISE_REPLAY_GAP_V2: &str = "BI_CHRONOLOGICAL_PREMISE_REPLAY_GAP_V2";

const ADJUDICATION_BYTES: &[u8] = include_bytes!("../../../docs/bi0_prerequisites_adjudication.md");
const AUDIT_BURN_BYTES: &[u8] = include_bytes!("../../../docs/bi0_prerequisites_v1_audit_burn.md");
const V1_CERTIFICATE_BYTES: &[u8] =
    include_bytes!("../../../docs/chronological_interface_slot_map_v1.json");
const V1_REPORT_BYTES: &[u8] =
    include_bytes!("../../../docs/CHRONOLOGICAL_INTERFACE_SLOT_MAP_V1_RESULT.md");
const BI0_ARCHIVE_BYTES: &[u8] = include_bytes!("../../../docs/BI_REGRESSION_CERTIFICATE.json");
const E5_ARCHIVE_BYTES: &[u8] =
    include_bytes!("../../../docs/schema2_e5_future_hole_finale_v2_dependent_context.json");
const A3_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-eval/src/a3_demand_grammar.rs");
const SUBSTITUTION_SOURCE_BYTES: &[u8] = include_bytes!("../../pen-type/src/substitution.rs");
const MOTIVE_SOURCE_BYTES: &[u8] =
    include_bytes!("../../pen-type/src/motive_parametric_coherence_v2.rs");
const FINALE_SOURCE_BYTES: &[u8] = include_bytes!("branch_invariance_finale.rs");
const THIS_SOURCE_BYTES: &[u8] = include_bytes!("chronological_slot_map_v2.rs");
const FROZEN_BI0_ARCHIVE_SHA256: &str =
    "5601c530fe74eae2d5e1fecb49ec06c44d84d8f44b94e7f87384c4b973cf0fda";
const FROZEN_E5_ARCHIVE_SHA256: &str =
    "c60a28c82370e88829dfd6457ae52814d4523fcf2b2012378814a4cf0dd32be5";

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapV2SourceBinding {
    pub path: String,
    pub role: String,
    pub byte_length: u64,
    pub blake3: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "membership_status")]
pub enum ChronologicalMembershipV2 {
    DeclarationOnly,
    Derived {
        membership_derivation_hash: String,
        internality_rule_id: String,
        exact_substitution_replayed: bool,
    },
    NamedGap {
        gap_id: String,
        reason: String,
    },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapV2InstanceAudit {
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
    pub zero_charge: bool,
    pub inferred_from_derivation_success: bool,
    pub instance_override_permitted: bool,
    pub independently_sealed_discharge: bool,
    pub membership: ChronologicalMembershipV2,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapV2WindowAudit {
    pub stage: u32,
    pub prefix_signature_digest: String,
    pub instances: Vec<ChronologicalSlotMapV2InstanceAudit>,
    pub derivation_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapV2Certificate {
    pub schema: String,
    pub date: String,
    pub adopted_slot_map_rule: String,
    pub closure_theorem: String,
    pub source_bindings: Vec<ChronologicalSlotMapV2SourceBinding>,
    pub v1_preserved_as_burned_regression_evidence: bool,
    pub audit_burn_is_occasion_not_semantic_definition: bool,
    pub windows: Vec<ChronologicalSlotMapV2WindowAudit>,
    pub live_sweep_digest_before_archive_read: String,
    pub historical_chronological_instance_count: usize,
    pub historical_declaration_replay_count: usize,
    pub historical_instance_ids_unique: bool,
    pub sealed_discharge_count: usize,
    pub sealed_discharge_derived_count: usize,
    pub sealed_discharge_named_gap_count: usize,
    pub named_gap_counts: BTreeMap<String, usize>,
    pub frozen_stage16_89_id_surface_exact: bool,
    pub formerly_gapped_expected_instance_ids: Vec<String>,
    pub formerly_gapped_derived_instance_ids: Vec<String>,
    pub formerly_gapped_named_gap_instance_ids: Vec<String>,
    pub every_declaration_order_preserving_identity: bool,
    pub every_declaration_zero_charge: bool,
    pub f_sm2_no_inference_or_override: bool,
    pub alternative_slot_map_permutation_trial_count: usize,
    pub non_enacted_branch_continuation_count: usize,
    pub f_sm1_passed: bool,
    pub bi0_rerun_authorized: bool,
    pub conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapV2Replay {
    pub valid: bool,
    pub f_sm1_passed: bool,
    pub sealed_discharge_derived_count: usize,
    pub sealed_discharge_named_gap_count: usize,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ChronologicalSlotMapV2Error {
    #[error("v2 slot-map invariant failed: {0}")]
    Invariant(String),
    #[error("JSON error: {0}")]
    Json(String),
    #[error("I/O error: {0}")]
    Io(String),
    #[error("emitted v2 certificate failed replay: {0}")]
    EmittedReplay(String),
}

fn bytes_hash(bytes: &[u8]) -> String {
    format!("blake3:{}", blake3_hex(bytes))
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CHRONOLOGICAL_SLOT_MAP_V2_SCHEMA, domain, value))
        .expect("v2 slot-map evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn external_tagged_hash<T: Serialize + ?Sized>(schema: &str, domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(schema, domain, value)).expect("archive evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

fn source_bindings() -> Vec<ChronologicalSlotMapV2SourceBinding> {
    [
        (
            "docs/bi0_prerequisites_adjudication.md",
            "adopted_rule",
            ADJUDICATION_BYTES,
        ),
        (
            "docs/bi0_prerequisites_v1_audit_burn.md",
            "audit_occasion_only",
            AUDIT_BURN_BYTES,
        ),
        (
            "docs/chronological_interface_slot_map_v1.json",
            "burned_v1_regression_evidence",
            V1_CERTIFICATE_BYTES,
        ),
        (
            "docs/CHRONOLOGICAL_INTERFACE_SLOT_MAP_V1_RESULT.md",
            "burned_v1_report_evidence",
            V1_REPORT_BYTES,
        ),
        (
            "docs/BI_REGRESSION_CERTIFICATE.json",
            "former_gap_ID_comparator_only",
            BI0_ARCHIVE_BYTES,
        ),
        (
            "docs/schema2_e5_future_hole_finale_v2_dependent_context.json",
            "sealed_discharge_comparator_only",
            E5_ARCHIVE_BYTES,
        ),
        (
            "crates/pen-eval/src/a3_demand_grammar.rs",
            "declared_identity_map",
            A3_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/substitution.rs",
            "exact_structural_substitution",
            SUBSTITUTION_SOURCE_BYTES,
        ),
        (
            "crates/pen-type/src/motive_parametric_coherence_v2.rs",
            "proof_strength_closure_specialization",
            MOTIVE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/branch_invariance_finale.rs",
            "strict_chronological_membership_issuer",
            FINALE_SOURCE_BYTES,
        ),
        (
            "crates/pen-search/src/chronological_slot_map_v2.rs",
            "versioned_F_SM1_wrapper",
            THIS_SOURCE_BYTES,
        ),
    ]
    .into_iter()
    .map(|(path, role, bytes)| ChronologicalSlotMapV2SourceBinding {
        path: path.to_owned(),
        role: role.to_owned(),
        byte_length: bytes.len() as u64,
        blake3: bytes_hash(bytes),
    })
    .collect()
}

fn certificate_digest(certificate: &ChronologicalSlotMapV2Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
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

fn source_coordinate(window: &A3HistoricalWindow, anchor: &str) -> Result<(u32, u16), String> {
    window
        .typed_sources
        .iter()
        .find(|source| source.anchor_id == anchor)
        .map(|source| (source.step, source.clause_index))
        .ok_or_else(|| format!("source anchor {anchor} is absent"))
}

fn named_gap(reason: String) -> ChronologicalMembershipV2 {
    let first = reason.split("::").next().unwrap_or_default();
    let gap_id = if first.starts_with("BI_CHRONOLOGICAL_") {
        first.to_owned()
    } else {
        BI_CHRONOLOGICAL_PREMISE_REPLAY_GAP_V2.to_owned()
    };
    ChronologicalMembershipV2::NamedGap { gap_id, reason }
}

fn archive_ids(value: &Value, field: &str) -> Result<Vec<String>, ChronologicalSlotMapV2Error> {
    value
        .get(field)
        .and_then(Value::as_array)
        .ok_or_else(|| ChronologicalSlotMapV2Error::Json(format!("archive field {field} absent")))?
        .iter()
        .map(|row| {
            row.as_str().map(str::to_owned).ok_or_else(|| {
                ChronologicalSlotMapV2Error::Json(format!("archive field {field} is non-string"))
            })
        })
        .collect()
}

pub fn issue_chronological_slot_map_v2_certificate()
-> Result<ChronologicalSlotMapV2Certificate, ChronologicalSlotMapV2Error> {
    let burn = std::str::from_utf8(AUDIT_BURN_BYTES)
        .map_err(|error| ChronologicalSlotMapV2Error::Invariant(error.to_string()))?;
    let audit_burn_is_occasion_not_semantic_definition = burn.contains("implementation audit")
        && burn.contains("BURNED AS BI-0 AUTHORITY")
        && burn.contains("open target variables");
    let full = SealedSignature::genesis_del_h15();
    let mut generated = Vec::new();
    let mut windows = Vec::new();
    let mut all_ids = BTreeSet::new();
    for stage in 1..=16 {
        let prefix = exact_prefix(&full, stage);
        let window = generate_a3_window_for_exact_prefix_unbounded(&prefix, stage)
            .map_err(|error| ChronologicalSlotMapV2Error::Invariant(error.to_string()))?;
        let mut rows = Vec::new();
        for instance in &window.instances {
            let scheme = window
                .schemes
                .iter()
                .find(|scheme| scheme.scheme_id == instance.scheme_id)
                .ok_or_else(|| {
                    ChronologicalSlotMapV2Error::Invariant("orphan A3 instance".to_owned())
                })?;
            if scheme.rule_constructor != A3RuleConstructor::ChronologicalComparison {
                continue;
            }
            let A3DemandOutputType::ChronologicalInteraction {
                interface_slot_map, ..
            } = &scheme.required_output
            else {
                return Err(ChronologicalSlotMapV2Error::Invariant(
                    "chronological scheme has non-chronological output".to_owned(),
                ));
            };
            let newest_anchor = instance.source_anchor_ids.get(1).ok_or_else(|| {
                ChronologicalSlotMapV2Error::Invariant(
                    "chronological newest source absent".to_owned(),
                )
            })?;
            let newest = window
                .typed_sources
                .iter()
                .find(|source| &source.anchor_id == newest_anchor)
                .ok_or_else(|| {
                    ChronologicalSlotMapV2Error::Invariant("newest source unresolved".to_owned())
                })?;
            let arity = newest.canonical_presentation.parameters.len() as u32;
            replay_chronological_interface_slot_map(interface_slot_map, arity)
                .map_err(|error| ChronologicalSlotMapV2Error::Invariant(error.to_string()))?;
            let (older_step, older_clause) = source_coordinate(
                &window,
                instance.source_anchor_ids.first().ok_or_else(|| {
                    ChronologicalSlotMapV2Error::Invariant("older source absent".to_owned())
                })?,
            )
            .map_err(ChronologicalSlotMapV2Error::Invariant)?;
            let (newest_step, newest_clause) = source_coordinate(&window, newest_anchor)
                .map_err(ChronologicalSlotMapV2Error::Invariant)?;
            all_ids.insert(instance.instance_id.clone());
            let mut row = ChronologicalSlotMapV2InstanceAudit {
                stage,
                instance_id: instance.instance_id.clone(),
                scheme_id: scheme.scheme_id.clone(),
                older_step,
                older_clause,
                newest_step,
                newest_clause,
                declared_arity: arity,
                declared_assignments: interface_slot_map
                    .assignments
                    .iter()
                    .map(|row| (row.interface_slot, row.parameter))
                    .collect(),
                declaration_hash: interface_slot_map.declaration_hash.clone(),
                declaration_replayed: true,
                zero_charge: interface_slot_map.kappa_charge == 0
                    && interface_slot_map.nu_charge == 0,
                inferred_from_derivation_success: interface_slot_map
                    .inferred_from_derivation_success,
                instance_override_permitted: interface_slot_map.instance_override_permitted,
                independently_sealed_discharge: false,
                membership: ChronologicalMembershipV2::DeclarationOnly,
                derivation_hash: String::new(),
            };
            row.derivation_hash = tagged_hash("instance", &row);
            rows.push(row);
        }
        let mut audit = ChronologicalSlotMapV2WindowAudit {
            stage,
            prefix_signature_digest: prefix.digest().to_owned(),
            instances: rows,
            derivation_hash: String::new(),
        };
        audit.derivation_hash = tagged_hash("window", &audit);
        windows.push(audit);
        generated.push((prefix, window));
    }
    let live_sweep_digest_before_archive_read = tagged_hash("live-declaration-sweep", &windows);

    // Archives are opened only after the complete declaration sweep is sealed.
    let e5: E5FutureHoleFinaleV2Certificate = serde_json::from_slice(E5_ARCHIVE_BYTES)
        .map_err(|error| ChronologicalSlotMapV2Error::Json(error.to_string()))?;
    let mut e5_projection = e5.clone();
    let e5_observed = e5_projection.result_digest.clone();
    e5_projection.result_digest.clear();
    let e5_valid = sha256_hex(E5_ARCHIVE_BYTES) == FROZEN_E5_ARCHIVE_SHA256
        && e5_observed
            == external_tagged_hash(
                E5_FUTURE_HOLE_FINALE_V2_SCHEMA,
                "certificate",
                &e5_projection,
            );
    let sealed_ids = e5
        .stage16
        .membership_rows
        .iter()
        .filter(|row| row.rule.starts_with("chronological_comparison::"))
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    if !e5_valid || sealed_ids.len() != 72 {
        return Err(ChronologicalSlotMapV2Error::Invariant(
            "frozen sealed-discharge corpus failed validation".to_owned(),
        ));
    }

    for (prefix, window) in &generated {
        for instance in &window.instances {
            if !sealed_ids.contains(&instance.instance_id) {
                continue;
            }
            let scheme = window
                .schemes
                .iter()
                .find(|scheme| scheme.scheme_id == instance.scheme_id)
                .ok_or_else(|| {
                    ChronologicalSlotMapV2Error::Invariant("sealed instance orphaned".to_owned())
                })?;
            let membership = match issue_branch_chronological_membership_audit(
                prefix,
                window.stage.saturating_sub(1),
                window,
                scheme,
                instance,
            ) {
                Ok((row, _))
                    if matches!(row.disposition, BranchDMembershipDisposition::Derivable)
                        && row.internality_rule_id == MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION =>
                {
                    ChronologicalMembershipV2::Derived {
                        membership_derivation_hash: row.derivation_hash,
                        internality_rule_id: row.internality_rule_id,
                        exact_substitution_replayed: row
                            .exact_substitution_from_sealed_preimage_replayed,
                    }
                }
                Ok((row, _)) => named_gap(format!(
                    "{BI_CHRONOLOGICAL_PREMISE_REPLAY_GAP_V2}::issuer returned {:?}",
                    row.disposition
                )),
                Err(reason) => named_gap(reason),
            };
            let row = windows
                .iter_mut()
                .find(|audit| audit.stage == window.stage)
                .and_then(|audit| {
                    audit
                        .instances
                        .iter_mut()
                        .find(|row| row.instance_id == instance.instance_id)
                })
                .ok_or_else(|| {
                    ChronologicalSlotMapV2Error::Invariant(
                        "sealed row absent from live sweep".to_owned(),
                    )
                })?;
            row.independently_sealed_discharge = true;
            row.membership = membership;
            row.derivation_hash.clear();
            row.derivation_hash = tagged_hash("instance", row);
        }
    }
    for window in &mut windows {
        window.derivation_hash.clear();
        window.derivation_hash = tagged_hash("window", window);
    }

    let sealed_rows = windows
        .iter()
        .flat_map(|window| &window.instances)
        .filter(|row| row.independently_sealed_discharge)
        .collect::<Vec<_>>();
    let sealed_discharge_derived_count = sealed_rows
        .iter()
        .filter(|row| matches!(row.membership, ChronologicalMembershipV2::Derived { .. }))
        .count();
    let sealed_discharge_named_gap_count = sealed_rows
        .iter()
        .filter(|row| matches!(row.membership, ChronologicalMembershipV2::NamedGap { .. }))
        .count();
    let sealed_discharge_count = sealed_rows.len();
    let mut named_gap_counts = BTreeMap::new();
    for row in &sealed_rows {
        if let ChronologicalMembershipV2::NamedGap { gap_id, .. } = &row.membership {
            *named_gap_counts.entry(gap_id.clone()).or_insert(0) += 1;
        }
    }
    drop(sealed_rows);

    let frozen_stage16_ids = e5
        .stage16
        .membership_rows
        .iter()
        .map(|row| row.a3_instance_id.clone())
        .collect::<BTreeSet<_>>();
    let live_stage16_ids = generated[15]
        .1
        .instances
        .iter()
        .map(|row| row.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let frozen_stage16_89_id_surface_exact =
        frozen_stage16_ids.len() == 89 && frozen_stage16_ids == live_stage16_ids;

    let bi0: Value = serde_json::from_slice(BI0_ARCHIVE_BYTES)
        .map_err(|error| ChronologicalSlotMapV2Error::Json(error.to_string()))?;
    let mut formerly_gapped_expected_instance_ids =
        archive_ids(&bi0, "enacted_finale_issuer_gap_instance_ids")?;
    formerly_gapped_expected_instance_ids.sort();
    let former_set = formerly_gapped_expected_instance_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let stage16_rows = &windows[15].instances;
    let mut formerly_gapped_derived_instance_ids = stage16_rows
        .iter()
        .filter(|row| former_set.contains(&row.instance_id))
        .filter(|row| matches!(row.membership, ChronologicalMembershipV2::Derived { .. }))
        .map(|row| row.instance_id.clone())
        .collect::<Vec<_>>();
    let mut formerly_gapped_named_gap_instance_ids = stage16_rows
        .iter()
        .filter(|row| former_set.contains(&row.instance_id))
        .filter(|row| matches!(row.membership, ChronologicalMembershipV2::NamedGap { .. }))
        .map(|row| row.instance_id.clone())
        .collect::<Vec<_>>();
    formerly_gapped_derived_instance_ids.sort();
    formerly_gapped_named_gap_instance_ids.sort();

    let historical_chronological_instance_count =
        windows.iter().map(|row| row.instances.len()).sum();
    let historical_declaration_replay_count = windows
        .iter()
        .flat_map(|row| &row.instances)
        .filter(|row| row.declaration_replayed)
        .count();
    let every_declaration_order_preserving_identity = windows.iter().all(|window| {
        window.instances.iter().all(|row| {
            row.declared_assignments
                .iter()
                .enumerate()
                .all(|(index, &(slot, parameter))| slot == index as u32 + 1 && parameter == slot)
        })
    });
    let every_declaration_zero_charge = windows
        .iter()
        .flat_map(|row| &row.instances)
        .all(|row| row.zero_charge);
    let f_sm2_no_inference_or_override = windows
        .iter()
        .flat_map(|row| &row.instances)
        .all(|row| !row.inferred_from_derivation_success && !row.instance_override_permitted);
    let frozen_bi0_valid = sha256_hex(BI0_ARCHIVE_BYTES) == FROZEN_BI0_ARCHIVE_SHA256;
    let f_sm1_passed = audit_burn_is_occasion_not_semantic_definition
        && frozen_bi0_valid
        && every_declaration_order_preserving_identity
        && every_declaration_zero_charge
        && f_sm2_no_inference_or_override
        && frozen_stage16_89_id_surface_exact
        && sealed_discharge_count == 72
        && sealed_discharge_derived_count == 72
        && sealed_discharge_named_gap_count == 0
        && formerly_gapped_expected_instance_ids.len() == 9
        && formerly_gapped_derived_instance_ids == formerly_gapped_expected_instance_ids;
    let mut certificate = ChronologicalSlotMapV2Certificate {
        schema: CHRONOLOGICAL_SLOT_MAP_V2_SCHEMA.to_owned(),
        date: CHRONOLOGICAL_SLOT_MAP_V2_DATE.to_owned(),
        adopted_slot_map_rule: CHRONOLOGICAL_INTERFACE_SLOT_MAP_V1.to_owned(),
        closure_theorem: MOTIVE_PARAMETRIC_COHERENCE_V2_VERSION.to_owned(),
        source_bindings: source_bindings(),
        v1_preserved_as_burned_regression_evidence: !V1_CERTIFICATE_BYTES.is_empty()
            && !V1_REPORT_BYTES.is_empty(),
        audit_burn_is_occasion_not_semantic_definition,
        windows,
        live_sweep_digest_before_archive_read,
        historical_chronological_instance_count,
        historical_declaration_replay_count,
        historical_instance_ids_unique: all_ids.len() == historical_chronological_instance_count,
        sealed_discharge_count,
        sealed_discharge_derived_count,
        sealed_discharge_named_gap_count,
        named_gap_counts,
        frozen_stage16_89_id_surface_exact,
        formerly_gapped_expected_instance_ids,
        formerly_gapped_derived_instance_ids,
        formerly_gapped_named_gap_instance_ids,
        every_declaration_order_preserving_identity,
        every_declaration_zero_charge,
        f_sm2_no_inference_or_override,
        alternative_slot_map_permutation_trial_count: 0,
        non_enacted_branch_continuation_count: 0,
        f_sm1_passed,
        bi0_rerun_authorized: f_sm1_passed,
        conclusion: if f_sm1_passed {
            "F-SM1 v2 passes with proof-strength closure specialization on every sealed discharge; BI-0 rerun is authorized on this prerequisite side."
        } else {
            "F-SM1 v2 fails honestly: one or more exact chronological substitutions cannot instantiate the closed motive-parametric theorem. Named gaps remain; BI-0, BI-1, and the cone stay closed."
        }
        .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

pub fn replay_chronological_slot_map_v2_certificate(
    certificate: &ChronologicalSlotMapV2Certificate,
) -> ChronologicalSlotMapV2Replay {
    let mut errors = Vec::new();
    if certificate.result_digest != certificate_digest(certificate) {
        errors.push("certificate digest mismatch".to_owned());
    }
    match issue_chronological_slot_map_v2_certificate() {
        Ok(expected) if expected == *certificate => {}
        Ok(_) => errors.push("certificate differs from independent v2 reissuance".to_owned()),
        Err(error) => errors.push(format!("independent v2 reissuance failed: {error}")),
    }
    ChronologicalSlotMapV2Replay {
        valid: errors.is_empty(),
        f_sm1_passed: certificate.f_sm1_passed,
        sealed_discharge_derived_count: certificate.sealed_discharge_derived_count,
        sealed_discharge_named_gap_count: certificate.sealed_discharge_named_gap_count,
        errors,
    }
}

pub fn replay_chronological_slot_map_v2_json(text: &str) -> ChronologicalSlotMapV2Replay {
    match serde_json::from_str::<ChronologicalSlotMapV2Certificate>(text) {
        Ok(certificate) => replay_chronological_slot_map_v2_certificate(&certificate),
        Err(error) => ChronologicalSlotMapV2Replay {
            valid: false,
            f_sm1_passed: false,
            sealed_discharge_derived_count: 0,
            sealed_discharge_named_gap_count: 0,
            errors: vec![format!("JSON parse failed: {error}")],
        },
    }
}

pub fn render_chronological_slot_map_v2_report(
    certificate: &ChronologicalSlotMapV2Certificate,
) -> String {
    format!(
        "# Chronological interface slot-map F-SM1 v2 result\n\n- Schema: `{}`\n- Digest: `{}`\n- Slot-map declaration replay: {}/{}\n- Sealed discharges derived / named gap: {} / {}\n- Named gap classes: `{:?}`\n- Former nine derived / named gap: {} / {}\n- Alternative permutations trialed: 0\n- F-SM1: `{}`\n- BI-0 authorized: `{}`\n\n{}\n",
        certificate.schema,
        certificate.result_digest,
        certificate.historical_declaration_replay_count,
        certificate.historical_chronological_instance_count,
        certificate.sealed_discharge_derived_count,
        certificate.sealed_discharge_named_gap_count,
        certificate.named_gap_counts,
        certificate.formerly_gapped_derived_instance_ids.len(),
        certificate.formerly_gapped_named_gap_instance_ids.len(),
        certificate.f_sm1_passed,
        certificate.bi0_rerun_authorized,
        certificate.conclusion,
    )
}

pub fn emit_chronological_slot_map_v2_create_new(
    directory: &Path,
) -> Result<ChronologicalSlotMapV2Replay, ChronologicalSlotMapV2Error> {
    let json_path = directory.join(CHRONOLOGICAL_SLOT_MAP_V2_CERTIFICATE_NAME);
    let report_path = directory.join(CHRONOLOGICAL_SLOT_MAP_V2_REPORT_NAME);
    if json_path.exists() || report_path.exists() {
        return Err(ChronologicalSlotMapV2Error::Io(
            "create-new target already exists".to_owned(),
        ));
    }
    let certificate = issue_chronological_slot_map_v2_certificate()?;
    let replay = replay_chronological_slot_map_v2_certificate(&certificate);
    if !replay.valid {
        return Err(ChronologicalSlotMapV2Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let json = serde_json::to_vec_pretty(&certificate)
        .map_err(|error| ChronologicalSlotMapV2Error::Json(error.to_string()))?;
    let report = render_chronological_slot_map_v2_report(&certificate);
    let mut json_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&json_path)
        .map_err(|error| ChronologicalSlotMapV2Error::Io(error.to_string()))?;
    json_file
        .write_all(&json)
        .map_err(|error| ChronologicalSlotMapV2Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| ChronologicalSlotMapV2Error::Io(error.to_string()))?;
    report_file
        .write_all(report.as_bytes())
        .map_err(|error| ChronologicalSlotMapV2Error::Io(error.to_string()))?;
    Ok(replay)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn v2_replays_named_gaps_without_forging_internal() {
        let certificate = issue_chronological_slot_map_v2_certificate().expect("v2 issues");
        assert!(replay_chronological_slot_map_v2_certificate(&certificate).valid);
        assert_eq!(certificate.sealed_discharge_count, 72);
        assert_eq!(
            certificate.sealed_discharge_derived_count
                + certificate.sealed_discharge_named_gap_count,
            72
        );
        assert!(certificate.sealed_discharge_named_gap_count > 0);
        assert!(!certificate.f_sm1_passed);
        assert!(!certificate.bi0_rerun_authorized);
        assert_eq!(certificate.alternative_slot_map_permutation_trial_count, 0);
    }

    #[test]
    fn v2_replay_rejects_rehashed_gap_mutation() {
        let certificate = issue_chronological_slot_map_v2_certificate().expect("v2 issues");
        let mut mutated = certificate.clone();
        mutated.conclusion.push_str(" forged");
        mutated.result_digest = certificate_digest(&mutated);
        assert!(!replay_chronological_slot_map_v2_certificate(&mutated).valid);
    }
}
