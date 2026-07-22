use pen_search::branch_invariance_program::{
    emit_bi_program_after_bi0_create_new, emit_bi0_regression_create_new,
    replay_bi_program_directory, replay_bi0_regression_directory,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [command, directory] if command == "create-bi0" => {
            let regression = emit_bi0_regression_create_new(Path::new(directory))?;
            println!(
                "created BI-0 v2 only: passed={} F-BI5={} prerequisites(slot-map/T-BI)={}/{} ledger_provenance={}/{} token_count_exact={} finale_gaps={} issuer_gaps={} realization_gaps={} digest={}",
                regression.bi0_passed,
                regression.f_bi5_triggered,
                regression.chronological_slot_map_replay_valid,
                regression.t_bi_nu1_replay_valid,
                regression.enacted_complete_ledger_exact_provenance_count,
                regression.enacted_complete_ledger_entry_count,
                regression.enacted_complete_ledger_authoritative_token_count_exact,
                regression.enacted_finale_expressivity_gaps.len(),
                regression.enacted_finale_issuer_gap_instance_ids.len(),
                regression.enacted_finale_realization_gap_ids.len(),
                regression.result_digest,
            );
        }
        [command, directory] if command == "replay-bi0" => {
            let replay = replay_bi0_regression_directory(Path::new(directory));
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err(format!("BI-0 v2 replay failed: {:?}", replay.errors).into());
            }
        }
        [command, directory] if command == "create-after-bi0" => {
            let bundle = emit_bi_program_after_bi0_create_new(Path::new(directory))?;
            println!(
                "read passing BI-0 v2; created {} BI-1 branch certificates and BI-4: zone={} cone_digest={}",
                bundle.branches.len(),
                bundle.cone.outcome_zone,
                bundle.cone.result_digest,
            );
        }
        [command, directory] if command == "replay-after-bi0" => {
            let replay = replay_bi_program_directory(Path::new(directory));
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                return Err(format!("post-BI-0 v2 replay failed: {:?}", replay.errors).into());
            }
        }
        _ => {
            return Err(
                "usage: branch_invariance_program <create-bi0|replay-bi0|create-after-bi0|replay-after-bi0> <output-directory>"
                    .into(),
            );
        }
    }
    Ok(())
}
