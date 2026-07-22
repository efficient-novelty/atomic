//! Create-new artifact wrapper for the adopted support-comprehension theorem
//! and its full F-SC1 chronological regression.

use crate::chronological_slot_map_v5::{
    ChronologicalSlotMapV5Certificate, issue_chronological_slot_map_v5_certificate,
    replay_chronological_slot_map_v5_certificate,
};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use thiserror::Error;

pub const SUPPORT_COMPREHENSION_V5_ARTIFACT_NAME: &str =
    "support_comprehension_chronological_v5.json";
pub const SUPPORT_COMPREHENSION_V5_REPORT_NAME: &str =
    "SUPPORT_COMPREHENSION_CHRONOLOGICAL_V5_RESULT.md";

#[derive(Clone, Debug, Error, Eq, PartialEq)]
pub enum SupportComprehensionArtifactV5Error {
    #[error("support-comprehension issuance failed: {0}")]
    Issue(String),
    #[error("support-comprehension JSON failed: {0}")]
    Json(String),
    #[error("support-comprehension artifact I/O failed: {0}")]
    Io(String),
    #[error("support-comprehension artifact replay failed: {0}")]
    Replay(String),
}

fn render_report(certificate: &ChronologicalSlotMapV5Certificate) -> String {
    format!(
        "# Support-comprehension chronological v5 result\n\n\
         **Date:** {}. **Outcome:** `{}`.\n\n\
         The canonical support analysis reconstructs the exact cyclic presentation at its comprehension type. F-SC1 replays **{}/71** inherited rows byte-for-byte, derives **{}/{}** sealed chronological discharges and **{}/{}** former-gap members, and closes T-SM1b at **{}/{}**.\n\n\
         Exact 54+18 baseline partition: **{}**. Exact sealed 72-ID join: **{}**. Exactly one changed case: **{}**. Unique normal form within the closed canonical dependency-analysis grammar: **{}**. Typed zero-mint capability on the exact replayed registration surface: **{}**. Composite replacement evidence bound: **{}**. Zero support/realizer accounting: **{}**. BI-0 chronological prerequisite reopened: **{}**.\n\n\
         This artifact issues no ledger, divergence verdict, or branch.\n\n\
         Certificate digest: `{}`.\n",
        certificate.date,
        certificate.outcome,
        certificate.inherited_row_equality_count,
        certificate.sealed_discharge_derived_count,
        certificate.sealed_discharge_count,
        certificate.former_derived_instance_ids.len(),
        certificate.former_expected_instance_ids.len(),
        certificate.t_sm1b_derived_count,
        certificate.t_sm1b_surface_count,
        certificate.baseline_exact_54_plus_18_partition,
        certificate.baseline_exact_sealed_72_id_join,
        certificate.exactly_one_case_changed,
        certificate.support_graph_normal_form_proved,
        certificate.support_zero_mint_capability_proved,
        certificate.support_replacement_evidence_bound,
        certificate.f_sc3_zero_accounting_passed,
        certificate.bi0_chronological_prerequisite_reopened,
        certificate.result_digest,
    )
}

pub fn emit_support_comprehension_v5_create_new(
    directory: &Path,
) -> Result<ChronologicalSlotMapV5Certificate, SupportComprehensionArtifactV5Error> {
    let certificate = issue_chronological_slot_map_v5_certificate()
        .map_err(|error| SupportComprehensionArtifactV5Error::Issue(error.to_string()))?;
    replay_chronological_slot_map_v5_certificate(&certificate)
        .map_err(|error| SupportComprehensionArtifactV5Error::Replay(error.to_string()))?;
    let json = serde_json::to_string_pretty(&certificate)
        .map_err(|error| SupportComprehensionArtifactV5Error::Json(error.to_string()))?;
    let certificate_path = directory.join(SUPPORT_COMPREHENSION_V5_ARTIFACT_NAME);
    let report_path = directory.join(SUPPORT_COMPREHENSION_V5_REPORT_NAME);
    if certificate_path.exists() || report_path.exists() {
        return Err(SupportComprehensionArtifactV5Error::Io(format!(
            "create-new target already exists: {} or {}",
            certificate_path.display(),
            report_path.display()
        )));
    }
    let mut certificate_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&certificate_path)
        .map_err(|error| SupportComprehensionArtifactV5Error::Io(error.to_string()))?;
    certificate_file
        .write_all(json.as_bytes())
        .map_err(|error| SupportComprehensionArtifactV5Error::Io(error.to_string()))?;
    let mut report_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&report_path)
        .map_err(|error| SupportComprehensionArtifactV5Error::Io(error.to_string()))?;
    report_file
        .write_all(render_report(&certificate).as_bytes())
        .map_err(|error| SupportComprehensionArtifactV5Error::Io(error.to_string()))?;
    Ok(certificate)
}

pub fn replay_support_comprehension_v5_directory(
    directory: &Path,
) -> Result<(), SupportComprehensionArtifactV5Error> {
    let claimed_json =
        std::fs::read_to_string(directory.join(SUPPORT_COMPREHENSION_V5_ARTIFACT_NAME))
            .map_err(|error| SupportComprehensionArtifactV5Error::Io(error.to_string()))?;
    let claimed: serde_json::Value = serde_json::from_str(&claimed_json)
        .map_err(|error| SupportComprehensionArtifactV5Error::Json(error.to_string()))?;
    let expected = issue_chronological_slot_map_v5_certificate()
        .map_err(|error| SupportComprehensionArtifactV5Error::Issue(error.to_string()))?;
    replay_chronological_slot_map_v5_certificate(&expected)
        .map_err(|error| SupportComprehensionArtifactV5Error::Replay(error.to_string()))?;
    let expected = serde_json::to_value(expected)
        .map_err(|error| SupportComprehensionArtifactV5Error::Json(error.to_string()))?;
    if claimed == expected {
        Ok(())
    } else {
        Err(SupportComprehensionArtifactV5Error::Replay(
            "stored JSON differs from a create-new reissuance".to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_is_explicitly_gate_only() {
        let certificate = issue_chronological_slot_map_v5_certificate().expect("v5 certificate");
        let report = render_report(&certificate);
        assert!(report.contains("72/72"));
        assert!(report.contains("18/18"));
        assert!(report.contains("issues no ledger"));
    }
}
