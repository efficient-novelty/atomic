use pen_search::bc2_boundary_provenance_join_v1::{
    emit_bc2_boundary_provenance_join_v1_create_new, issue_bc2_boundary_provenance_join_v1,
    render_bc2_boundary_provenance_join_v1, replay_bc2_boundary_provenance_join_v1,
    replay_bc2_boundary_provenance_join_v1_json,
};
use std::fs;
use std::path::Path;

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => {
            let certificate =
                issue_bc2_boundary_provenance_join_v1().map_err(|error| error.to_string())?;
            let replay = replay_bc2_boundary_provenance_join_v1(&certificate);
            if !replay.valid {
                return Err(format!(
                    "new BC-2 certificate did not replay: {}",
                    replay.errors.join("; ")
                ));
            }
            print!("{}", render_bc2_boundary_provenance_join_v1(&certificate));
        }
        [command, certificate_path, report_path] if command == "create-new" => {
            let certificate = emit_bc2_boundary_provenance_join_v1_create_new(
                Path::new(certificate_path),
                Path::new(report_path),
            )
            .map_err(|error| error.to_string())?;
            print!("{}", render_bc2_boundary_provenance_join_v1(&certificate));
        }
        [command, certificate_path] if command == "replay" => {
            let json = fs::read_to_string(certificate_path)
                .map_err(|error| format!("could not read certificate: {error}"))?;
            let replay = replay_bc2_boundary_provenance_join_v1_json(&json);
            if !replay.valid {
                return Err(format!("BC-2 replay failed: {}", replay.errors.join("; ")));
            }
            println!(
                "BC-2 replay valid; status={:?}; rows={}; named_gaps={}; c8_closed={}; m4_authorized={}",
                replay.run_status,
                replay.row_disposition_count.value,
                replay.named_gap_row_count.value,
                replay.c8_closed,
                replay.m4_authorized
            );
        }
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example bc2_boundary_provenance_join_v1 [create-new <certificate.json> <report.md> | replay <certificate.json>]"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run().map_err(Into::into)
}
