use pen_search::t_d2_3_candidate_provenance_key_schema_v1::{
    emit_t_d2_3_candidate_provenance_key_schema_v1_create_new,
    issue_t_d2_3_candidate_provenance_key_schema_v1,
    render_t_d2_3_candidate_provenance_key_schema_v1,
    replay_t_d2_3_candidate_provenance_key_schema_v1,
    replay_t_d2_3_candidate_provenance_key_schema_v1_json,
};
use std::fs;
use std::path::Path;

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => {
            let certificate = issue_t_d2_3_candidate_provenance_key_schema_v1()
                .map_err(|error| error.to_string())?;
            let replay = replay_t_d2_3_candidate_provenance_key_schema_v1(&certificate);
            if !replay.valid {
                return Err(format!(
                    "new T-D2-3 certificate did not replay: {}",
                    replay.errors.join("; ")
                ));
            }
            print!(
                "{}",
                render_t_d2_3_candidate_provenance_key_schema_v1(&certificate)
            );
        }
        [command, certificate_path, report_path] if command == "create-new" => {
            let certificate = emit_t_d2_3_candidate_provenance_key_schema_v1_create_new(
                Path::new(certificate_path),
                Path::new(report_path),
            )
            .map_err(|error| error.to_string())?;
            print!(
                "{}",
                render_t_d2_3_candidate_provenance_key_schema_v1(&certificate)
            );
        }
        [command, certificate_path] if command == "replay" => {
            let json = fs::read_to_string(certificate_path)
                .map_err(|error| format!("could not read certificate: {error}"))?;
            let replay = replay_t_d2_3_candidate_provenance_key_schema_v1_json(&json);
            if !replay.valid {
                return Err(format!(
                    "T-D2-3 replay failed: {}",
                    replay.errors.join("; ")
                ));
            }
            println!(
                "T-D2-3 replay valid; status={:?}; key_schema_frozen={}; join_executed={}; m3_v1_remains_authoritative={}; m4_authorized={}",
                replay.status,
                replay.key_schema_frozen,
                replay.join_executed,
                replay.m3_v1_remains_authoritative,
                replay.m4_authorized
            );
        }
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example t_d2_3_candidate_provenance_key_schema_v1 [create-new <certificate.json> <report.md> | replay <certificate.json>]"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("t-d2-3-candidate-provenance-key-schema-v1".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("T-D2-3 worker panicked".into()),
    }
}
