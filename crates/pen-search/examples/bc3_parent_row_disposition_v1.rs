use pen_search::bc3_parent_row_disposition_v1::{
    emit_bc3_parent_row_disposition_v1_create_new, issue_bc3_parent_row_disposition_v1,
    render_bc3_parent_row_disposition_v1, replay_bc3_parent_row_disposition_v1,
    replay_bc3_parent_row_disposition_v1_json,
};
use std::fs;
use std::path::Path;

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => {
            let certificate =
                issue_bc3_parent_row_disposition_v1().map_err(|error| error.to_string())?;
            let replay = replay_bc3_parent_row_disposition_v1(&certificate);
            if !replay.valid {
                return Err(format!(
                    "new BC-3 certificate did not replay: {}",
                    replay.errors.join("; ")
                ));
            }
            print!("{}", render_bc3_parent_row_disposition_v1(&certificate));
        }
        [command, certificate_path, report_path] if command == "create-new" => {
            let certificate = emit_bc3_parent_row_disposition_v1_create_new(
                Path::new(certificate_path),
                Path::new(report_path),
            )
            .map_err(|error| error.to_string())?;
            print!("{}", render_bc3_parent_row_disposition_v1(&certificate));
        }
        [command, certificate_path] if command == "replay" => {
            let json = fs::read_to_string(certificate_path)
                .map_err(|error| format!("could not read certificate: {error}"))?;
            let replay = replay_bc3_parent_row_disposition_v1_json(&json);
            if !replay.valid {
                return Err(format!("BC-3 replay failed: {}", replay.errors.join("; ")));
            }
            println!(
                "BC-3 replay valid; status={:?}; coverage_complete={}; m3_v2_authorized={}; m4_authorized={}",
                replay.status,
                replay.coverage_complete,
                replay.m3_v2_authorized_by_bc3,
                replay.m4_authorized_by_bc3,
            );
        }
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example bc3_parent_row_disposition_v1 [create-new <certificate.json> <report.md> | replay <certificate.json>]"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("bc3-parent-row-disposition-v1".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("BC-3 worker panicked".into()),
    }
}
