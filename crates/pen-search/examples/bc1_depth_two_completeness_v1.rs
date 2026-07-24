use pen_search::bc1_depth_two_completeness_v1::{
    emit_bc1_depth_two_completeness_v1_create_new, issue_bc1_depth_two_completeness_v1,
    render_bc1_depth_two_completeness_v1, replay_bc1_depth_two_completeness_v1,
    replay_bc1_depth_two_completeness_v1_json,
};
use std::fs;
use std::path::Path;

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => {
            let certificate =
                issue_bc1_depth_two_completeness_v1().map_err(|error| error.to_string())?;
            let replay = replay_bc1_depth_two_completeness_v1(&certificate);
            if !replay.valid {
                return Err(format!(
                    "new BC-1 certificate did not replay: {}",
                    replay.errors.join("; ")
                ));
            }
            print!("{}", render_bc1_depth_two_completeness_v1(&certificate));
        }
        [command, certificate_path, report_path] if command == "create-new" => {
            let certificate = emit_bc1_depth_two_completeness_v1_create_new(
                Path::new(certificate_path),
                Path::new(report_path),
            )
            .map_err(|error| error.to_string())?;
            print!("{}", render_bc1_depth_two_completeness_v1(&certificate));
        }
        [command, certificate_path] if command == "replay" => {
            let json = fs::read_to_string(certificate_path)
                .map_err(|error| format!("could not read certificate: {error}"))?;
            let replay = replay_bc1_depth_two_completeness_v1_json(&json);
            if !replay.valid {
                return Err(format!("BC-1 replay failed: {}", replay.errors.join("; ")));
            }
            println!(
                "BC-1 replay valid; status={:?}; c2_closed={}; no_promotions={}; m3_v2_prerequisite_satisfied={}",
                replay.bc1_status,
                replay.c2_closed,
                replay.no_promotions_made,
                replay.m3_v2_prerequisite_satisfied,
            );
        }
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example bc1_depth_two_completeness_v1 [create-new <certificate.json> <report.md> | replay <certificate.json>]"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("bc1-depth-two-completeness-v1".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("BC-1 worker panicked".into()),
    }
}
