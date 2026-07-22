use pen_search::r_t2_future_hole_confluence_v2::{
    Rt2FutureHoleConfluenceV2Certificate, emit_r_t2_future_hole_confluence_v2_create_new,
    issue_r_t2_future_hole_confluence_v2_certificate,
    replay_r_t2_future_hole_confluence_v2_certificate,
};
use std::path::Path;

fn print_summary(certificate: &Rt2FutureHoleConfluenceV2Certificate) {
    let summary = serde_json::json!({
        "schema": certificate.schema,
        "result_digest": certificate.result_digest,
        "outcome": certificate.outcome,
        "branch_count": certificate.branch_count,
        "complete_scheme_counts": certificate
            .branches
            .iter()
            .map(|branch| branch.complete_certified_scheme_count)
            .collect::<Vec<_>>(),
        "future_semantics_per_branch": certificate
            .branches
            .iter()
            .map(|branch| serde_json::json!({
                "stage4_candidate_hash": branch.stage4_candidate_hash,
                "attempted": branch.future_hole_aggregate.registration_count,
                "unary_registered": branch.future_hole_aggregate.unary_registration_count,
                "structural_registered": branch.future_hole_aggregate.structural_registration_count,
                "gap_scheme_ids": branch.future_hole_aggregate.gap_scheme_ids,
                "exact_registered_coverage": branch
                    .future_hole_aggregate
                    .exact_scheme_and_instance_coverage,
                "successful_rows_provisional_due_motive_parametric_v1": branch
                    .future_hole_aggregate
                    .successful_rows_provisional_due_motive_parametric_v1,
                "successful_rows_law_level_authoritative": branch
                    .future_hole_aggregate
                    .successful_rows_law_level_authoritative,
                "upstream_soundness_blockers": branch
                    .future_hole_aggregate
                    .upstream_soundness_blockers,
                "opaque_motive_gap_replays": branch
                    .future_hole_aggregate
                    .registrations
                    .iter()
                    .flat_map(|row| &row.opaque_motive_reconstructions)
                    .map(|evidence| serde_json::json!({
                        "source_step": evidence.source_step,
                        "source_clause_index": evidence.source_clause_index,
                        "canonical_parameter_index": evidence.canonical_parameter_index,
                        "old_level": evidence.old_level,
                        "ambient": evidence.source_ambient_parameters,
                        "prior_clause_index": evidence.prior_clause_index,
                        "prior_clause_kernel_type": evidence.prior_clause_kernel_type_json,
                        "attempted_contextual_motive": evidence.attempted_contextual_motive,
                        "full_declaration_error": evidence.motive_declaration_error_json,
                        "full_declaration_ambient_limit_replayed": evidence
                            .full_declaration_ambient_limit_replayed,
                        "isolated_motive_error": evidence
                            .isolated_motive_declaration_error_json,
                        "isolated_motive_not_formable_parameter": evidence
                            .isolated_motive_not_formable_parameter,
                        "motive_not_formable_canonical_parameter_index": evidence
                            .motive_not_formable_canonical_parameter_index,
                        "isolated_projection_is_diagnostic_not_original_full_context_judgment": evidence
                            .isolated_projection_is_diagnostic_not_original_full_context_judgment,
                        "exact_replay": evidence.exact_motive_not_formable_replayed,
                        "f_fh4_api_expressivity_gap": evidence
                            .current_contextual_api_expressivity_gap_f_fh4,
                        "semantic_nonexistence_claimed": evidence
                            .future_scheme_nonexistence_claimed,
                    }))
                    .collect::<Vec<_>>(),
            }))
            .collect::<Vec<_>>(),
        "full_relative_a3_coverage": certificate
            .full_relative_a3_seed_constructor_coverage_every_branch,
        "zero_named_gaps": certificate.zero_named_gaps_every_branch,
        "full_adopted_frozen_scope": certificate
            .full_adopted_frozen_semantic_scope_exhaustive_every_branch,
        "pairwise_comparison_count": certificate.pairwise_scheme_set_comparisons.len(),
        "all_stage5_scheme_sets_equivalent": certificate.all_stage5_scheme_sets_equivalent,
        "order_reversal_invariant": certificate.order_reversal_invariant,
        "r_t2_confluence_proved": certificate.r_t2_confluence_proved,
        "r_t2_confluence_refuted": certificate.r_t2_confluence_refuted,
        "r_t3_user_adjudication_opened": certificate.r_t3_user_adjudication_opened,
        "selected_candidate_hash": certificate.selected_candidate_hash,
        "serialized_blockers": certificate.serialized_blockers,
        "disclosed_nonblocking_scope_and_archive_limits": certificate
            .disclosed_nonblocking_scope_and_archive_limits,
    });
    println!("{}", serde_json::to_string_pretty(&summary).unwrap());
}

fn run() -> Result<(), String> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command] if command == "inspect" => {
            let certificate = issue_r_t2_future_hole_confluence_v2_certificate()
                .map_err(|error| error.to_string())?;
            print_summary(&certificate);
        }
        [command, path] if command == "create-new" => {
            let replay = emit_r_t2_future_hole_confluence_v2_create_new(Path::new(path))
                .map_err(|error| error.to_string())?;
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
        }
        [command, path] if command == "replay" => {
            let certificate: Rt2FutureHoleConfluenceV2Certificate = serde_json::from_str(
                &std::fs::read_to_string(path).map_err(|error| error.to_string())?,
            )
            .map_err(|error| error.to_string())?;
            let replay = replay_r_t2_future_hole_confluence_v2_certificate(&certificate);
            println!("{}", serde_json::to_string_pretty(&replay).unwrap());
            if !replay.valid {
                return Err("R-T2 future-hole confluence v2 replay failed".to_owned());
            }
        }
        _ => {
            return Err(
                "usage: r_t2_future_hole_confluence_v2 <inspect|create-new|replay> [artifact.json]"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("r-t2-future-hole-confluence-v2-driver".to_owned())
        .stack_size(256 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("R-T2 future-hole confluence v2 driver panicked".into()),
    }
}
