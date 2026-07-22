//! Read-only F-SC1 replay harness.  Artifact emission is intentionally left
//! to the integrated create-new run.

use pen_search::chronological_slot_map_v5::{
    issue_chronological_slot_map_v5_certificate, replay_chronological_slot_map_v5_certificate,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let certificate = issue_chronological_slot_map_v5_certificate()?;
    replay_chronological_slot_map_v5_certificate(&certificate)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "valid": true,
            "result_digest": certificate.result_digest,
            "inherited_row_equality_count": certificate.inherited_row_equality_count,
            "prior_71_rows_byte_identical": certificate.prior_71_rows_byte_identical,
            "prior_71_evidence_hashes_identical": certificate.prior_71_evidence_hashes_identical,
            "prior_71_row_hashes_identical": certificate.prior_71_row_hashes_identical,
            "changed_case_ids": certificate.changed_case_ids,
            "sealed": [certificate.sealed_discharge_derived_count, certificate.sealed_discharge_count],
            "former": [certificate.former_derived_instance_ids.len(), certificate.former_expected_instance_ids.len()],
            "t_sm1b": [certificate.t_sm1b_derived_count, certificate.t_sm1b_surface_count],
            "zero_accounting": certificate.f_sc3_zero_accounting_passed,
            "fixed_positive_gate_passed": certificate.fixed_positive_gate_passed,
        }))?
    );
    Ok(())
}
