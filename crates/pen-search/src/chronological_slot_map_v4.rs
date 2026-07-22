//! Fixed-gate chronological slot-map join for T-SM1a and T-SM1b.
//!
//! This version does not promote a partial corpus.  It first reissues the
//! two theorem components, then joins their case identities to the 72
//! independently sealed chronological discharges and the frozen nine-row
//! former-gap surface.  A negative theorem result is a valid certificate;
//! only an exact 72/72, 9/9, zero-gap result opens the BI-0 prerequisite.

use crate::chronological_slot_map_v3::{
    ChronologicalMembershipV3, issue_chronological_slot_map_audit_v3,
    replay_chronological_slot_map_audit_v3,
};
use crate::motive_typed_open_specialization_v4::{
    TSm1bCaseDispositionV4, issue_t_sm1b_corpus_audit_v4, replay_t_sm1b_corpus_audit_v4,
};
use crate::t_sm1a_contextual_formation_v4::{issue_t_sm1a_audit_v4, replay_t_sm1a_audit_v4};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const CHRONOLOGICAL_SLOT_MAP_V4_SCHEMA: &str =
    "chronological-interface-slot-map-fixed-positive-gate-v4";
pub const CHRONOLOGICAL_SLOT_MAP_V4_DATE: &str = "2026-07-22";
pub const CHRONOLOGICAL_SLOT_MAP_V4_CERTIFICATE_NAME: &str =
    "chronological_interface_slot_map_v4.json";
pub const CHRONOLOGICAL_SLOT_MAP_V4_REPORT_NAME: &str =
    "CHRONOLOGICAL_INTERFACE_SLOT_MAP_V4_RESULT.md";

const EXPECTED_T_SM1A: usize = 54;
const EXPECTED_T_SM1B: usize = 18;
const EXPECTED_SEALED: usize = 72;
const EXPECTED_FORMER: usize = 9;

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(CHRONOLOGICAL_SLOT_MAP_V4_SCHEMA, domain, value))
        .expect("chronological v4 evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "disposition")]
pub enum ChronologicalCaseDispositionV4 {
    Derived { evidence_hash: String },
    NamedBlocker { blocker: String, reason: String },
}

impl ChronologicalCaseDispositionV4 {
    fn is_derived(&self) -> bool {
        matches!(self, Self::Derived { .. })
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalCaseJoinV4 {
    pub instance_id: String,
    pub theorem_component: String,
    pub older_step: u32,
    pub older_clause: u16,
    pub newest_step: u32,
    pub newest_clause: u16,
    pub independently_sealed_discharge: bool,
    pub former_gap_instance: bool,
    pub disposition: ChronologicalCaseDispositionV4,
    pub row_hash: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapV4Certificate {
    pub schema: String,
    pub date: String,
    pub t_sm1a_audit_hash: String,
    pub t_sm1a_reissued: bool,
    pub t_sm1a_surface_count: usize,
    pub t_sm1a_derived_count: usize,
    pub t_sm1a_named_gap_count: usize,
    pub t_sm1b_audit_hash: String,
    pub t_sm1b_reissued: bool,
    pub t_sm1b_surface_count: usize,
    pub t_sm1b_derived_count: usize,
    pub t_sm1b_named_gap_count: usize,
    pub t_sm1b_blocker_counts: BTreeMap<String, usize>,
    pub frozen_v3_reissuance_hash: String,
    pub frozen_v3_reissued: bool,
    pub cases: Vec<ChronologicalCaseJoinV4>,
    pub joined_case_count: usize,
    pub joined_case_ids_unique: bool,
    pub theorem_surfaces_disjoint: bool,
    pub exact_54_plus_18_partition: bool,
    pub exact_sealed_72_id_join: bool,
    pub sealed_discharge_count: usize,
    pub sealed_discharge_derived_count: usize,
    pub sealed_discharge_named_gap_count: usize,
    pub former_expected_instance_ids: Vec<String>,
    pub former_derived_instance_ids: Vec<String>,
    pub former_named_gap_instance_ids: Vec<String>,
    pub former_nine_exact: bool,
    pub no_partial_promotion: bool,
    pub fixed_positive_gate_passed: bool,
    pub bi0_prerequisite_reopened: bool,
    pub positive_artifact_permitted: bool,
    pub outcome: String,
    pub conclusion: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ChronologicalSlotMapV4Replay {
    pub valid: bool,
    pub fixed_positive_gate_passed: bool,
    pub sealed_discharge_derived_count: usize,
    pub sealed_discharge_named_gap_count: usize,
    pub former_derived_count: usize,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum ChronologicalSlotMapV4Error {
    #[error("T-SM1a issuance failed: {0}")]
    TSm1a(String),
    #[error("T-SM1b issuance failed: {0}")]
    TSm1b(String),
    #[error("frozen v3 reissuance failed: {0}")]
    FrozenV3(String),
    #[error("chronological v4 join failed: {0}")]
    Join(String),
    #[error("chronological v4 JSON failed: {0}")]
    Json(String),
    #[error("chronological v4 I/O failed: {0}")]
    Io(String),
    #[error("emitted chronological v4 artifact failed replay: {0}")]
    EmittedReplay(String),
}

fn certificate_digest(certificate: &ChronologicalSlotMapV4Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

/// Reissue both theorem components and join their exact case identities to
/// the fixed 72/72 + 9/9 positive gate.  Named blockers are evidence, not an
/// error path: they produce an honest negative certificate.
pub fn issue_chronological_slot_map_v4_certificate()
-> Result<ChronologicalSlotMapV4Certificate, ChronologicalSlotMapV4Error> {
    let t_sm1a = issue_t_sm1a_audit_v4()
        .map_err(|error| ChronologicalSlotMapV4Error::TSm1a(error.to_string()))?;
    let t_sm1a_replay = replay_t_sm1a_audit_v4(&t_sm1a);
    let t_sm1a_reissued = t_sm1a_replay.valid;

    let t_sm1b = issue_t_sm1b_corpus_audit_v4()
        .map_err(|error| ChronologicalSlotMapV4Error::TSm1b(error.to_string()))?;
    replay_t_sm1b_corpus_audit_v4(&t_sm1b)
        .map_err(|error| ChronologicalSlotMapV4Error::TSm1b(error.to_string()))?;
    let t_sm1b_reissued = true;

    // The v3 theorem issuer validates its fixed E-5 and BI-0 comparator
    // inputs.  Its negative membership verdicts have no authority here; only
    // the independently sealed 72 IDs and former-nine IDs are consumed.
    let v3 = issue_chronological_slot_map_audit_v3()
        .map_err(|error| ChronologicalSlotMapV4Error::FrozenV3(error.to_string()))?;
    let v3_replay = replay_chronological_slot_map_audit_v3(&v3);
    if !v3_replay.valid {
        return Err(ChronologicalSlotMapV4Error::FrozenV3(
            v3_replay.errors.join("; "),
        ));
    }
    let frozen_v3_reissued = true;
    let sealed_ids = v3
        .windows
        .iter()
        .flat_map(|window| &window.instances)
        .filter(|row| row.independently_sealed_discharge)
        .map(|row| row.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let v3_named_ids = v3
        .windows
        .iter()
        .flat_map(|window| &window.instances)
        .filter(|row| {
            row.independently_sealed_discharge
                && matches!(row.membership, ChronologicalMembershipV3::NamedGap { .. })
        })
        .map(|row| row.instance_id.clone())
        .collect::<BTreeSet<_>>();
    if sealed_ids.len() != EXPECTED_SEALED || v3_named_ids != sealed_ids {
        return Err(ChronologicalSlotMapV4Error::FrozenV3(format!(
            "fixed v3 surface is not the expected 72 named theorem gaps: sealed={}, named={}",
            sealed_ids.len(),
            v3_named_ids.len(),
        )));
    }
    let former_set = v3
        .formerly_gapped_expected_instance_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if former_set.len() != EXPECTED_FORMER {
        return Err(ChronologicalSlotMapV4Error::FrozenV3(format!(
            "fixed former-gap surface has {} IDs, expected {EXPECTED_FORMER}",
            former_set.len(),
        )));
    }

    let mut cases = Vec::with_capacity(EXPECTED_SEALED);
    for case in &t_sm1a.cases {
        let disposition = match (&case.gap, case.replayed) {
            (None, true) => ChronologicalCaseDispositionV4::Derived {
                evidence_hash: case
                    .specialization_derivation_hash
                    .clone()
                    .unwrap_or_else(|| case.row_hash.clone()),
            },
            (Some(gap), _) => ChronologicalCaseDispositionV4::NamedBlocker {
                blocker: gap.gap_id.clone(),
                reason: gap.reason.clone(),
            },
            (None, false) => ChronologicalCaseDispositionV4::NamedBlocker {
                blocker: "T_SM1A_REPLAY_FAILED".to_owned(),
                reason: "case did not replay".to_owned(),
            },
        };
        let mut row = ChronologicalCaseJoinV4 {
            instance_id: case.instance_id.clone(),
            theorem_component: "T-SM1a".to_owned(),
            older_step: case.older_step,
            older_clause: case.older_clause,
            newest_step: case.newest_step,
            newest_clause: case.newest_clause,
            independently_sealed_discharge: sealed_ids.contains(&case.instance_id),
            former_gap_instance: former_set.contains(&case.instance_id),
            disposition,
            row_hash: String::new(),
        };
        row.row_hash = tagged_hash("case", &row);
        cases.push(row);
    }
    for case in &t_sm1b.cases {
        let disposition = match &case.disposition {
            TSm1bCaseDispositionV4::Derived {
                specialization_hash,
            } => ChronologicalCaseDispositionV4::Derived {
                evidence_hash: specialization_hash.clone(),
            },
            TSm1bCaseDispositionV4::NamedBlocker { blocker, reason } => {
                ChronologicalCaseDispositionV4::NamedBlocker {
                    blocker: blocker.clone(),
                    reason: reason.clone(),
                }
            }
        };
        let mut row = ChronologicalCaseJoinV4 {
            instance_id: case.instance_id.clone(),
            theorem_component: "T-SM1b".to_owned(),
            older_step: case.older_step,
            older_clause: case.older_clause,
            newest_step: case.newest_step,
            newest_clause: case.newest_clause,
            independently_sealed_discharge: sealed_ids.contains(&case.instance_id),
            former_gap_instance: former_set.contains(&case.instance_id),
            disposition,
            row_hash: String::new(),
        };
        row.row_hash = tagged_hash("case", &row);
        cases.push(row);
    }
    cases.sort_by(|left, right| left.instance_id.cmp(&right.instance_id));

    let joined_ids = cases
        .iter()
        .map(|case| case.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let joined_case_count = cases.len();
    let joined_case_ids_unique = joined_ids.len() == joined_case_count;
    let t_sm1a_ids = cases
        .iter()
        .filter(|case| case.theorem_component == "T-SM1a")
        .map(|case| case.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let t_sm1b_ids = cases
        .iter()
        .filter(|case| case.theorem_component == "T-SM1b")
        .map(|case| case.instance_id.clone())
        .collect::<BTreeSet<_>>();
    let theorem_surfaces_disjoint = t_sm1a_ids.is_disjoint(&t_sm1b_ids);
    let exact_54_plus_18_partition = t_sm1a_ids.len() == EXPECTED_T_SM1A
        && t_sm1b_ids.len() == EXPECTED_T_SM1B
        && joined_case_count == EXPECTED_SEALED
        && theorem_surfaces_disjoint;
    let exact_sealed_72_id_join =
        joined_ids == sealed_ids && cases.iter().all(|case| case.independently_sealed_discharge);
    let sealed_discharge_count = cases
        .iter()
        .filter(|case| case.independently_sealed_discharge)
        .count();
    let sealed_discharge_derived_count = cases
        .iter()
        .filter(|case| case.independently_sealed_discharge && case.disposition.is_derived())
        .count();
    let sealed_discharge_named_gap_count = sealed_discharge_count - sealed_discharge_derived_count;

    let mut former_expected_instance_ids = former_set.iter().cloned().collect::<Vec<_>>();
    former_expected_instance_ids.sort();
    let mut former_derived_instance_ids = cases
        .iter()
        .filter(|case| case.former_gap_instance && case.disposition.is_derived())
        .map(|case| case.instance_id.clone())
        .collect::<Vec<_>>();
    former_derived_instance_ids.sort();
    let mut former_named_gap_instance_ids = cases
        .iter()
        .filter(|case| case.former_gap_instance && !case.disposition.is_derived())
        .map(|case| case.instance_id.clone())
        .collect::<Vec<_>>();
    former_named_gap_instance_ids.sort();
    let former_nine_exact = former_expected_instance_ids.len() == EXPECTED_FORMER
        && former_derived_instance_ids == former_expected_instance_ids
        && former_named_gap_instance_ids.is_empty();

    let t_sm1a_surface_count = t_sm1a.cases.len();
    let t_sm1a_derived_count = t_sm1a.derived_count;
    let t_sm1a_named_gap_count = t_sm1a.named_gap_count;
    let t_sm1b_surface_count = t_sm1b.cases.len();
    let t_sm1b_derived_count = t_sm1b.derived_count;
    let t_sm1b_named_gap_count = t_sm1b_surface_count - t_sm1b_derived_count;
    let fixed_positive_gate_passed = t_sm1a_reissued
        && t_sm1b_reissued
        && frozen_v3_reissued
        && t_sm1a.t_sm1a_passed
        && t_sm1a_surface_count == EXPECTED_T_SM1A
        && t_sm1a_derived_count == EXPECTED_T_SM1A
        && t_sm1a_named_gap_count == 0
        && t_sm1b.structural_surface_equals_registered_18
        && t_sm1b_surface_count == EXPECTED_T_SM1B
        && t_sm1b_derived_count == EXPECTED_T_SM1B
        && t_sm1b_named_gap_count == 0
        && t_sm1b.t_sm1b_component_complete
        && exact_54_plus_18_partition
        && exact_sealed_72_id_join
        && sealed_discharge_count == EXPECTED_SEALED
        && sealed_discharge_derived_count == EXPECTED_SEALED
        && sealed_discharge_named_gap_count == 0
        && former_nine_exact;
    let no_partial_promotion = !fixed_positive_gate_passed
        || (sealed_discharge_derived_count == EXPECTED_SEALED && former_nine_exact);
    let mut certificate = ChronologicalSlotMapV4Certificate {
        schema: CHRONOLOGICAL_SLOT_MAP_V4_SCHEMA.to_owned(),
        date: CHRONOLOGICAL_SLOT_MAP_V4_DATE.to_owned(),
        t_sm1a_audit_hash: t_sm1a.audit_hash,
        t_sm1a_reissued,
        t_sm1a_surface_count,
        t_sm1a_derived_count,
        t_sm1a_named_gap_count,
        t_sm1b_audit_hash: t_sm1b.audit_hash,
        t_sm1b_reissued,
        t_sm1b_surface_count,
        t_sm1b_derived_count,
        t_sm1b_named_gap_count,
        t_sm1b_blocker_counts: t_sm1b.blocker_counts,
        frozen_v3_reissuance_hash: v3.audit_hash,
        frozen_v3_reissued,
        cases,
        joined_case_count,
        joined_case_ids_unique,
        theorem_surfaces_disjoint,
        exact_54_plus_18_partition,
        exact_sealed_72_id_join,
        sealed_discharge_count,
        sealed_discharge_derived_count,
        sealed_discharge_named_gap_count,
        former_expected_instance_ids,
        former_derived_instance_ids,
        former_named_gap_instance_ids,
        former_nine_exact,
        no_partial_promotion,
        fixed_positive_gate_passed,
        bi0_prerequisite_reopened: fixed_positive_gate_passed,
        positive_artifact_permitted: fixed_positive_gate_passed,
        outcome: if fixed_positive_gate_passed {
            "F_SM1_V4_FIXED_POSITIVE_GATE_PASSED"
        } else {
            "F_SM1_V4_HONEST_NEGATIVE"
        }
        .to_owned(),
        conclusion: if fixed_positive_gate_passed {
            "All 72 sealed chronological discharges and all nine former-gap instances have exact replayable theorem evidence. The chronological side of BI-0 is reopened; no BI-1 or cone work is executed here."
        } else {
            "At least one of the fixed 72 chronological discharges remains a named theorem blocker. Partial promotion is forbidden, so the chronological side of BI-0 remains closed and no BI-1 or cone work is authorized."
        }
        .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

pub fn replay_chronological_slot_map_v4_certificate(
    claimed: &ChronologicalSlotMapV4Certificate,
) -> ChronologicalSlotMapV4Replay {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("chronological v4 certificate digest mismatch".to_owned());
        return ChronologicalSlotMapV4Replay {
            valid: false,
            fixed_positive_gate_passed: false,
            sealed_discharge_derived_count: claimed.sealed_discharge_derived_count,
            sealed_discharge_named_gap_count: claimed.sealed_discharge_named_gap_count,
            former_derived_count: claimed.former_derived_instance_ids.len(),
            errors,
        };
    }
    match issue_chronological_slot_map_v4_certificate() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("certificate differs from create-new reissuance".to_owned()),
        Err(error) => errors.push(format!("create-new reissuance failed: {error}")),
    }
    let logical_gate = claimed.t_sm1a_reissued
        && claimed.t_sm1b_reissued
        && claimed.exact_54_plus_18_partition
        && claimed.exact_sealed_72_id_join
        && claimed.sealed_discharge_derived_count == EXPECTED_SEALED
        && claimed.sealed_discharge_named_gap_count == 0
        && claimed.former_nine_exact;
    if claimed.fixed_positive_gate_passed != logical_gate
        || claimed.bi0_prerequisite_reopened != logical_gate
        || claimed.positive_artifact_permitted != logical_gate
        || !claimed.no_partial_promotion
    {
        errors.push("fixed positive gate or no-partial-promotion surface is invalid".to_owned());
    }
    ChronologicalSlotMapV4Replay {
        valid: errors.is_empty(),
        fixed_positive_gate_passed: claimed.fixed_positive_gate_passed,
        sealed_discharge_derived_count: claimed.sealed_discharge_derived_count,
        sealed_discharge_named_gap_count: claimed.sealed_discharge_named_gap_count,
        former_derived_count: claimed.former_derived_instance_ids.len(),
        errors,
    }
}

pub fn replay_chronological_slot_map_v4_json(json: &str) -> ChronologicalSlotMapV4Replay {
    match serde_json::from_str::<ChronologicalSlotMapV4Certificate>(json) {
        Ok(certificate) => replay_chronological_slot_map_v4_certificate(&certificate),
        Err(error) => ChronologicalSlotMapV4Replay {
            valid: false,
            fixed_positive_gate_passed: false,
            sealed_discharge_derived_count: 0,
            sealed_discharge_named_gap_count: 0,
            former_derived_count: 0,
            errors: vec![error.to_string()],
        },
    }
}

pub fn render_chronological_slot_map_v4_report(
    certificate: &ChronologicalSlotMapV4Certificate,
) -> String {
    let blockers = if certificate.t_sm1b_blocker_counts.is_empty() {
        "none".to_owned()
    } else {
        certificate
            .t_sm1b_blocker_counts
            .iter()
            .map(|(name, count)| format!("{name}={count}"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!(
        "# Chronological slot-map v4 result\n\n**Date:** {}. **Outcome:** `{}`.\n\nT-SM1a: {}/{} derived. T-SM1b: {}/{} derived (blockers: {}). Exact sealed join: **{}**; sealed derivations: **{}/{}**; former-gap derivations: **{}/{}**. The fixed positive gate passed: **{}**.\n\n{}\n\nCertificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.t_sm1a_derived_count,
        certificate.t_sm1a_surface_count,
        certificate.t_sm1b_derived_count,
        certificate.t_sm1b_surface_count,
        blockers,
        certificate.exact_sealed_72_id_join,
        certificate.sealed_discharge_derived_count,
        certificate.sealed_discharge_count,
        certificate.former_derived_instance_ids.len(),
        certificate.former_expected_instance_ids.len(),
        certificate.fixed_positive_gate_passed,
        certificate.conclusion,
        certificate.result_digest,
    )
}

pub fn emit_chronological_slot_map_v4_create_new(
    directory: &Path,
) -> Result<ChronologicalSlotMapV4Certificate, ChronologicalSlotMapV4Error> {
    let certificate = issue_chronological_slot_map_v4_certificate()?;
    let json_path = directory.join(CHRONOLOGICAL_SLOT_MAP_V4_CERTIFICATE_NAME);
    let report_path = directory.join(CHRONOLOGICAL_SLOT_MAP_V4_REPORT_NAME);
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| ChronologicalSlotMapV4Error::Json(error.to_string()))?;
    let mut json_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&json_path)
        .map_err(|error| ChronologicalSlotMapV4Error::Io(error.to_string()))?;
    json_file
        .write_all(json.as_bytes())
        .map_err(|error| ChronologicalSlotMapV4Error::Io(error.to_string()))?;
    let replay = replay_chronological_slot_map_v4_json(&json);
    if !replay.valid {
        return Err(ChronologicalSlotMapV4Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| ChronologicalSlotMapV4Error::Io(error.to_string()))?;
    report_file
        .write_all(render_chronological_slot_map_v4_report(&certificate).as_bytes())
        .map_err(|error| ChronologicalSlotMapV4Error::Io(error.to_string()))?;
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_gate_never_promotes_a_partial_corpus() {
        let certificate =
            issue_chronological_slot_map_v4_certificate().expect("v4 chronological certificate");
        assert_eq!(certificate.t_sm1a_surface_count, EXPECTED_T_SM1A);
        assert_eq!(certificate.t_sm1b_surface_count, EXPECTED_T_SM1B);
        assert_eq!(certificate.joined_case_count, EXPECTED_SEALED);
        assert!(certificate.exact_54_plus_18_partition);
        assert!(certificate.exact_sealed_72_id_join);
        assert!(certificate.no_partial_promotion);
        if certificate.sealed_discharge_derived_count != EXPECTED_SEALED {
            assert!(!certificate.fixed_positive_gate_passed);
            assert!(!certificate.bi0_prerequisite_reopened);
            assert!(!certificate.positive_artifact_permitted);
        }
        assert!(replay_chronological_slot_map_v4_certificate(&certificate).valid);
    }

    #[test]
    fn case_and_gate_mutations_fail_replay() {
        let certificate =
            issue_chronological_slot_map_v4_certificate().expect("v4 chronological certificate");
        let mut case_mutation = certificate.clone();
        case_mutation.cases[0].instance_id.push_str("-forged");
        assert!(!replay_chronological_slot_map_v4_certificate(&case_mutation).valid);

        let mut gate_mutation = certificate.clone();
        gate_mutation.fixed_positive_gate_passed = !gate_mutation.fixed_positive_gate_passed;
        assert!(!replay_chronological_slot_map_v4_certificate(&gate_mutation).valid);

        let mut unknown = serde_json::to_value(&certificate).expect("serialize certificate");
        unknown
            .as_object_mut()
            .expect("certificate object")
            .insert("unknown_field".to_owned(), serde_json::Value::Bool(true));
        assert!(!replay_chronological_slot_map_v4_json(&unknown.to_string()).valid);
    }
}
