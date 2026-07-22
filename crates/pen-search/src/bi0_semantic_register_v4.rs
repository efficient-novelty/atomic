//! First BI-0 attempt under the semantic-family nu register and F-AL1-prime.
//!
//! The T-BI-NU1 and chronological issuers remain independent.  This module
//! stores only their exact projections and joins their gates.  It never calls
//! BI-1, enumerates a non-enacted cone, or turns a partial F-SM1 corpus into a
//! positive prerequisite.

use crate::chronological_slot_map_v4::{
    ChronologicalCaseDispositionV4, issue_chronological_slot_map_v4_certificate,
};
use crate::t_bi_nu1_regression_v4::{BareRegisterRowV4, issue_t_bi_nu1_regression_v4_certificate};
use pen_core::hash::blake3_hex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const BI0_SEMANTIC_REGISTER_V4_SCHEMA: &str =
    "branch-invariance-bi0-semantic-register-f-al1-prime-v4";
pub const BI0_SEMANTIC_REGISTER_V4_DATE: &str = "2026-07-22";
pub const BI0_SEMANTIC_REGISTER_V4_CERTIFICATE_NAME: &str = "bi0_semantic_register_v4.json";
pub const BI0_SEMANTIC_REGISTER_V4_REPORT_NAME: &str = "BI0_SEMANTIC_REGISTER_V4_RESULT.md";

fn tagged_hash<T: Serialize + ?Sized>(domain: &str, value: &T) -> String {
    let bytes = serde_json::to_vec(&(BI0_SEMANTIC_REGISTER_V4_SCHEMA, domain, value))
        .expect("BI-0 semantic-register evidence serializes");
    format!("blake3:{}", blake3_hex(&bytes))
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0OperationalProjectionV4 {
    pub stage: u32,
    pub winner_candidate_hash: String,
    pub winner_admissible_under_recorded_regime: bool,
    pub operational_row_exact: bool,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0SemanticRegisterV4Certificate {
    pub schema: String,
    pub date: String,
    pub t_bi_nu1_result_digest: String,
    pub t_bi_nu1_intrinsic_seal: String,
    pub t_bi_role_declaration_count: usize,
    pub t_bi_proved_family_count: usize,
    pub t_bi_named_impossibility_count: usize,
    pub t_bi_named_registry_residual_count: usize,
    pub t_bi_resolved_role_declaration_count: usize,
    pub t_bi_silent_residue_count: usize,
    pub t_bi_extraction_complete: bool,
    pub t_bi_operational_rows: Vec<Bi0OperationalProjectionV4>,
    pub t_bi_exact_operational_regression: bool,
    pub structural_register: Vec<u32>,
    pub authoritative_semantic_register: Option<Vec<u32>>,
    pub non_authoritative_extraction_floor: Option<Vec<u32>>,
    pub bare_register_table: Vec<BareRegisterRowV4>,
    pub bare_register_table_exact: bool,
    pub t_bi_f_al1_prime_passed: bool,
    pub t_bi_side_passed: bool,
    pub chronological_result_digest: String,
    pub chronological_exact_54_plus_18_partition: bool,
    pub chronological_exact_sealed_72_id_join: bool,
    pub chronological_derived_count: usize,
    pub chronological_named_gap_count: usize,
    pub chronological_former_derived_count: usize,
    pub chronological_former_expected_count: usize,
    pub chronological_blocker_counts: BTreeMap<String, usize>,
    pub chronological_blocking_case_ids: Vec<String>,
    pub chronological_no_partial_promotion: bool,
    pub f_sm1_fixed_positive_gate_passed: bool,
    pub bi0_attempt_executed: bool,
    pub bi0_passed: bool,
    pub bi1_invoked: bool,
    pub non_enacted_cone_invoked: bool,
    pub non_enacted_branch_work_executed: bool,
    pub outcome: String,
    pub permitted_conclusion: String,
    pub required_successor_action: String,
    pub result_digest: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Bi0SemanticRegisterV4Replay {
    pub valid: bool,
    pub bi0_passed: bool,
    pub t_bi_side_passed: bool,
    pub f_sm1_fixed_positive_gate_passed: bool,
    pub bi1_invoked: bool,
    pub non_enacted_cone_invoked: bool,
    pub errors: Vec<String>,
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum Bi0SemanticRegisterV4Error {
    #[error("T-BI-NU1 v4 issuance failed: {0}")]
    TBi(String),
    #[error("chronological v4 issuance failed: {0}")]
    Chronological(String),
    #[error("BI-0 semantic-register JSON failed: {0}")]
    Json(String),
    #[error("BI-0 semantic-register I/O failed: {0}")]
    Io(String),
    #[error("emitted BI-0 semantic-register artifact failed replay: {0}")]
    EmittedReplay(String),
}

fn certificate_digest(certificate: &Bi0SemanticRegisterV4Certificate) -> String {
    let mut projection = certificate.clone();
    projection.result_digest.clear();
    tagged_hash("certificate", &projection)
}

/// Execute the first BI-0 prerequisite join.  Fresh issuer results are used
/// directly; top-level replay reissues both sides and requires byte-exact
/// equality with this projection.
pub fn issue_bi0_semantic_register_v4_certificate()
-> Result<Bi0SemanticRegisterV4Certificate, Bi0SemanticRegisterV4Error> {
    let t_bi = issue_t_bi_nu1_regression_v4_certificate()
        .map_err(|error| Bi0SemanticRegisterV4Error::TBi(error.to_string()))?;
    let chronological = issue_chronological_slot_map_v4_certificate()
        .map_err(|error| Bi0SemanticRegisterV4Error::Chronological(error.to_string()))?;

    let t_bi_operational_rows = t_bi
        .operational
        .rows
        .iter()
        .map(|row| Bi0OperationalProjectionV4 {
            stage: row.stage,
            winner_candidate_hash: row.winner_candidate_hash.clone(),
            winner_admissible_under_recorded_regime: row.winner_admissible_under_recorded_regime,
            operational_row_exact: row.operational_row_exact,
        })
        .collect::<Vec<_>>();
    let operational_projection_exact = t_bi_operational_rows.len() == 15
        && t_bi_operational_rows.iter().map(|row| row.stage).eq(1..=15)
        && t_bi_operational_rows
            .iter()
            .all(|row| row.winner_admissible_under_recorded_regime && row.operational_row_exact);
    let bare_register_table = t_bi.registers.table.clone();
    let bare_register_table_exact = bare_register_table.len() == 15
        && bare_register_table.iter().map(|row| row.stage).eq(1..=15)
        && t_bi.registers.table_has_exactly_fifteen_rows
        && t_bi.registers.table_rows_have_exactly_three_bare_fields
        && bare_register_table.iter().all(|row| {
            serde_json::to_value(row)
                .ok()
                .and_then(|value| value.as_object().cloned())
                .is_some_and(|object| {
                    object.len() == 3
                        && object.contains_key("stage")
                        && object.contains_key("structural")
                        && object.contains_key("semantic")
                })
        });
    let t_bi_side_passed = t_bi.role_declaration_count == 250
        && t_bi.resolved_role_declaration_count == 250
        && t_bi.proved_family_declaration_count + t_bi.theorem_impossibility_declaration_count
            == 250
        && t_bi.named_registry_residual_count == 0
        && t_bi.silent_residue_count == 0
        && t_bi.extraction_complete
        && t_bi.operational.exact_operational_regression
        && operational_projection_exact
        && bare_register_table_exact
        && t_bi.f_al1_prime_passed
        && t_bi.t_bi_nu1_proved_on_enacted_branch
        && t_bi.bi0_rerun_authorized_by_t_bi_nu1_side
        && !t_bi.non_enacted_branch_work_executed;

    let mut chronological_blocking_case_ids = chronological
        .cases
        .iter()
        .filter(|case| {
            matches!(
                case.disposition,
                ChronologicalCaseDispositionV4::NamedBlocker { .. }
            )
        })
        .map(|case| case.instance_id.clone())
        .collect::<Vec<_>>();
    chronological_blocking_case_ids.sort();
    let f_sm1_fixed_positive_gate_passed = chronological.fixed_positive_gate_passed
        && chronological.bi0_prerequisite_reopened
        && chronological.positive_artifact_permitted
        && chronological.sealed_discharge_derived_count == 72
        && chronological.sealed_discharge_named_gap_count == 0
        && chronological.former_nine_exact
        && chronological_blocking_case_ids.is_empty();
    let bi0_attempt_executed = true;
    let bi0_passed = t_bi_side_passed && f_sm1_fixed_positive_gate_passed;

    // This artifact is BI-0 only.  A pass would reopen BI-1 as a successor;
    // it would still not run it in the same certificate.
    let bi1_invoked = false;
    let non_enacted_cone_invoked = false;
    let non_enacted_branch_work_executed = false;
    let mut certificate = Bi0SemanticRegisterV4Certificate {
        schema: BI0_SEMANTIC_REGISTER_V4_SCHEMA.to_owned(),
        date: BI0_SEMANTIC_REGISTER_V4_DATE.to_owned(),
        t_bi_nu1_result_digest: t_bi.result_digest,
        t_bi_nu1_intrinsic_seal: t_bi
            .intrinsic_issuance_digest_sealed_before_post_seal_inputs,
        t_bi_role_declaration_count: t_bi.role_declaration_count,
        t_bi_proved_family_count: t_bi.proved_family_declaration_count,
        t_bi_named_impossibility_count: t_bi.theorem_impossibility_declaration_count,
        t_bi_named_registry_residual_count: t_bi.named_registry_residual_count,
        t_bi_resolved_role_declaration_count: t_bi.resolved_role_declaration_count,
        t_bi_silent_residue_count: t_bi.silent_residue_count,
        t_bi_extraction_complete: t_bi.extraction_complete,
        t_bi_operational_rows,
        t_bi_exact_operational_regression: t_bi.operational.exact_operational_regression,
        structural_register: t_bi.registers.structural_vector,
        authoritative_semantic_register: t_bi.registers.authoritative_semantic_vector,
        non_authoritative_extraction_floor: t_bi
            .registers
            .non_authoritative_extraction_floor,
        bare_register_table,
        bare_register_table_exact,
        t_bi_f_al1_prime_passed: t_bi.f_al1_prime_passed,
        t_bi_side_passed,
        chronological_result_digest: chronological.result_digest,
        chronological_exact_54_plus_18_partition: chronological
            .exact_54_plus_18_partition,
        chronological_exact_sealed_72_id_join: chronological.exact_sealed_72_id_join,
        chronological_derived_count: chronological.sealed_discharge_derived_count,
        chronological_named_gap_count: chronological.sealed_discharge_named_gap_count,
        chronological_former_derived_count: chronological.former_derived_instance_ids.len(),
        chronological_former_expected_count: chronological
            .former_expected_instance_ids
            .len(),
        chronological_blocker_counts: chronological.t_sm1b_blocker_counts,
        chronological_blocking_case_ids,
        chronological_no_partial_promotion: chronological.no_partial_promotion,
        f_sm1_fixed_positive_gate_passed,
        bi0_attempt_executed,
        bi0_passed,
        bi1_invoked,
        non_enacted_cone_invoked,
        non_enacted_branch_work_executed,
        outcome: if bi0_passed {
            "BI0_SEMANTIC_REGISTER_V4_PASSED"
        } else if !t_bi_side_passed && !f_sm1_fixed_positive_gate_passed {
            "BI0_SEMANTIC_REGISTER_V4_FALSE_AT_BOTH_PREREQUISITES"
        } else if !t_bi_side_passed {
            "BI0_SEMANTIC_REGISTER_V4_FALSE_AT_T_BI_NU1"
        } else if !f_sm1_fixed_positive_gate_passed {
            "BI0_SEMANTIC_REGISTER_V4_FALSE_AT_F_SM1"
        } else {
            unreachable!("BI-0 can be false only when a prerequisite is false")
        }
        .to_owned(),
        permitted_conclusion: if bi0_passed {
            "Both independent BI-0 prerequisites pass. BI-1 is reopened as a separate create-new successor, but no cone or non-enacted branch is evaluated by this artifact."
        } else if !t_bi_side_passed && !f_sm1_fixed_positive_gate_passed {
            "BI-0 is false at both prerequisites. T-BI-NU1 has no complete semantic ledger, and the fixed chronological positive gate forbids partial promotion. BI-1 and the non-enacted cone remain closed."
        } else if !t_bi_side_passed {
            "BI-0 is false at T-BI-NU1. The replay-valid negative certificate carries only a non-authoritative extraction floor; no complete semantic ledger or divergence table exists. BI-1 and the non-enacted cone remain closed."
        } else {
            "BI-0 is false. The complete T-BI semantic ledger remains certified independently, but the fixed chronological positive gate does not admit partial promotion. BI-1 and the non-enacted cone remain closed."
        }
        .to_owned(),
        required_successor_action: if bi0_passed {
            "Run BI-1 create-new under the separately adopted branch-invariance program."
        } else if !t_bi_side_passed && !f_sm1_fixed_positive_gate_passed {
            "Close the named T-BI unified-registry/C1/anchor residuals and replace the cyclic chronological identity slot map with a versioned dependency-respecting successor; then rerun both prerequisites and BI-0 create-new."
        } else if !t_bi_side_passed {
            "Close the named T-BI unified-registry/C1/anchor residuals without consulting the structural register, then rerun this BI-0 artifact create-new."
        } else {
            "Resolve the named chronological theorem blocker without changing the fixed 72/72 and 9/9 gate, then rerun this BI-0 artifact create-new."
        }
        .to_owned(),
        result_digest: String::new(),
    };
    certificate.result_digest = certificate_digest(&certificate);
    Ok(certificate)
}

pub fn replay_bi0_semantic_register_v4_certificate(
    claimed: &Bi0SemanticRegisterV4Certificate,
) -> Bi0SemanticRegisterV4Replay {
    let mut errors = Vec::new();
    if claimed.result_digest != certificate_digest(claimed) {
        errors.push("BI-0 semantic-register certificate digest mismatch".to_owned());
        return Bi0SemanticRegisterV4Replay {
            valid: false,
            bi0_passed: false,
            t_bi_side_passed: claimed.t_bi_side_passed,
            f_sm1_fixed_positive_gate_passed: claimed.f_sm1_fixed_positive_gate_passed,
            bi1_invoked: claimed.bi1_invoked,
            non_enacted_cone_invoked: claimed.non_enacted_cone_invoked,
            errors,
        };
    }
    match issue_bi0_semantic_register_v4_certificate() {
        Ok(expected) if expected == *claimed => {}
        Ok(_) => errors.push("certificate differs from create-new reissuance".to_owned()),
        Err(error) => errors.push(format!("create-new reissuance failed: {error}")),
    }
    let logical_t_bi = claimed.t_bi_role_declaration_count == 250
        && claimed.t_bi_resolved_role_declaration_count == 250
        && claimed.t_bi_proved_family_count + claimed.t_bi_named_impossibility_count == 250
        && claimed.t_bi_named_registry_residual_count == 0
        && claimed.t_bi_silent_residue_count == 0
        && claimed.t_bi_extraction_complete
        && claimed.t_bi_exact_operational_regression
        && claimed.bare_register_table_exact
        && claimed.t_bi_f_al1_prime_passed;
    let logical_f_sm1 = claimed.chronological_exact_54_plus_18_partition
        && claimed.chronological_exact_sealed_72_id_join
        && claimed.chronological_derived_count == 72
        && claimed.chronological_named_gap_count == 0
        && claimed.chronological_former_derived_count
            == claimed.chronological_former_expected_count
        && claimed.chronological_former_expected_count == 9
        && claimed.chronological_blocking_case_ids.is_empty();
    let logical_bi0 = logical_t_bi && logical_f_sm1;
    if claimed.t_bi_side_passed != logical_t_bi
        || claimed.f_sm1_fixed_positive_gate_passed != logical_f_sm1
        || claimed.bi0_passed != logical_bi0
        || !claimed.bi0_attempt_executed
        || claimed.bi1_invoked
        || claimed.non_enacted_cone_invoked
        || claimed.non_enacted_branch_work_executed
        || !claimed.chronological_no_partial_promotion
    {
        errors.push("BI-0 prerequisite join or sequencing surface is invalid".to_owned());
    }
    Bi0SemanticRegisterV4Replay {
        valid: errors.is_empty(),
        bi0_passed: claimed.bi0_passed,
        t_bi_side_passed: claimed.t_bi_side_passed,
        f_sm1_fixed_positive_gate_passed: claimed.f_sm1_fixed_positive_gate_passed,
        bi1_invoked: claimed.bi1_invoked,
        non_enacted_cone_invoked: claimed.non_enacted_cone_invoked,
        errors,
    }
}

pub fn replay_bi0_semantic_register_v4_json(json: &str) -> Bi0SemanticRegisterV4Replay {
    match serde_json::from_str::<Bi0SemanticRegisterV4Certificate>(json) {
        Ok(certificate) => replay_bi0_semantic_register_v4_certificate(&certificate),
        Err(error) => Bi0SemanticRegisterV4Replay {
            valid: false,
            bi0_passed: false,
            t_bi_side_passed: false,
            f_sm1_fixed_positive_gate_passed: false,
            bi1_invoked: false,
            non_enacted_cone_invoked: false,
            errors: vec![error.to_string()],
        },
    }
}

pub fn render_bi0_semantic_register_v4_report(
    certificate: &Bi0SemanticRegisterV4Certificate,
) -> String {
    let register_section = if certificate.authoritative_semantic_register.is_some() {
        let mut table = String::new();
        for row in &certificate.bare_register_table {
            table.push_str(&format!(
                "| {} | {} | {} |\n",
                row.stage, row.structural, row.semantic
            ));
        }
        format!(
            "The semantic register and structural testimony are reproduced below as the required bare, uninterpreted table.\n\n| Stage | Structural | Semantic |\n|---:|---:|---:|\n{table}"
        )
    } else {
        format!(
            "F-NR3 suppresses the register-divergence table. The T-BI side supplied only this non-authoritative extraction floor: `{:?}`; BI-0 does not treat it as a semantic ledger.",
            certificate
                .non_authoritative_extraction_floor
                .as_deref()
                .unwrap_or(&[])
        )
    };
    format!(
        "# BI-0 semantic-register v4 result\n\n**Date:** {}. **Outcome:** `{}`.\n\nT-BI-NU1/F-AL1-prime: **{}** (resolved {}/{} declarations; named registry residuals {}; silent residue {}). F-SM1: **{}** ({}/72 sealed derivations; {}/9 former-gap derivations). BI-0: **{}**. BI-1 invoked: **{}**. Non-enacted cone invoked: **{}**.\n\n{}\n\n{}\n\nNext: {}\n\nCertificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.t_bi_side_passed,
        certificate.t_bi_resolved_role_declaration_count,
        certificate.t_bi_role_declaration_count,
        certificate.t_bi_named_registry_residual_count,
        certificate.t_bi_silent_residue_count,
        certificate.f_sm1_fixed_positive_gate_passed,
        certificate.chronological_derived_count,
        certificate.chronological_former_derived_count,
        certificate.bi0_passed,
        certificate.bi1_invoked,
        certificate.non_enacted_cone_invoked,
        register_section,
        certificate.permitted_conclusion,
        certificate.required_successor_action,
        certificate.result_digest,
    )
}

pub fn emit_bi0_semantic_register_v4_create_new(
    directory: &Path,
) -> Result<Bi0SemanticRegisterV4Certificate, Bi0SemanticRegisterV4Error> {
    let certificate = issue_bi0_semantic_register_v4_certificate()?;
    let json_path = directory.join(BI0_SEMANTIC_REGISTER_V4_CERTIFICATE_NAME);
    let report_path = directory.join(BI0_SEMANTIC_REGISTER_V4_REPORT_NAME);
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| Bi0SemanticRegisterV4Error::Json(error.to_string()))?;
    let mut json_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&json_path)
        .map_err(|error| Bi0SemanticRegisterV4Error::Io(error.to_string()))?;
    json_file
        .write_all(json.as_bytes())
        .map_err(|error| Bi0SemanticRegisterV4Error::Io(error.to_string()))?;
    let replay = replay_bi0_semantic_register_v4_json(&json);
    if !replay.valid {
        return Err(Bi0SemanticRegisterV4Error::EmittedReplay(
            replay.errors.join("; "),
        ));
    }
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| Bi0SemanticRegisterV4Error::Io(error.to_string()))?;
    report_file
        .write_all(render_bi0_semantic_register_v4_report(&certificate).as_bytes())
        .map_err(|error| Bi0SemanticRegisterV4Error::Io(error.to_string()))?;
    Ok(certificate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bi0_short_circuits_before_bi1_and_cone_when_either_gate_is_false() {
        let certificate =
            issue_bi0_semantic_register_v4_certificate().expect("BI-0 v4 certificate");
        assert!(!certificate.t_bi_side_passed);
        assert_eq!(certificate.t_bi_named_registry_residual_count, 250);
        assert_eq!(certificate.t_bi_silent_residue_count, 0);
        assert!(!certificate.bare_register_table_exact);
        assert!(certificate.authoritative_semantic_register.is_none());
        assert!(certificate.non_authoritative_extraction_floor.is_some());
        assert_eq!(
            certificate.outcome,
            "BI0_SEMANTIC_REGISTER_V4_FALSE_AT_BOTH_PREREQUISITES"
        );
        assert!(!certificate.bi0_passed);
        assert!(!certificate.bi1_invoked);
        assert!(!certificate.non_enacted_cone_invoked);
        assert!(!certificate.non_enacted_branch_work_executed);
        assert!(replay_bi0_semantic_register_v4_certificate(&certificate).valid);
    }

    #[test]
    fn digest_gate_and_case_mutations_fail_replay() {
        let certificate =
            issue_bi0_semantic_register_v4_certificate().expect("BI-0 v4 certificate");
        let mut gate = certificate.clone();
        gate.bi0_passed = !gate.bi0_passed;
        assert!(!replay_bi0_semantic_register_v4_certificate(&gate).valid);

        let mut case = certificate.clone();
        case.chronological_blocking_case_ids
            .push("forged".to_owned());
        assert!(!replay_bi0_semantic_register_v4_certificate(&case).valid);

        let mut unknown = serde_json::to_value(&certificate).expect("serialize certificate");
        unknown
            .as_object_mut()
            .expect("certificate object")
            .insert("unknown_field".to_owned(), serde_json::Value::Bool(true));
        assert!(!replay_bi0_semantic_register_v4_json(&unknown.to_string()).valid);
    }
}
