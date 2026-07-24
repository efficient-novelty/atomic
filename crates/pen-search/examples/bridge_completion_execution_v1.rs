use pen_search::bridge_completion_execution_v1::{
    emit_bridge_completion_execution_v1_create_new, issue_bridge_completion_execution_v1,
    render_bridge_completion_execution_v1, replay_bridge_completion_execution_v1,
    replay_bridge_completion_execution_v1_json,
};
use std::fs;
use std::path::Path;

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => {
            let certificate =
                issue_bridge_completion_execution_v1().map_err(|error| error.to_string())?;
            let replay = replay_bridge_completion_execution_v1(&certificate);
            if !replay.valid {
                return Err(format!(
                    "new bridge-completion execution manifest did not replay: {}",
                    replay.errors.join("; ")
                ));
            }
            print!("{}", render_bridge_completion_execution_v1(&certificate));
        }
        [command, certificate_path, report_path] if command == "create-new" => {
            let certificate = emit_bridge_completion_execution_v1_create_new(
                Path::new(certificate_path),
                Path::new(report_path),
            )
            .map_err(|error| error.to_string())?;
            print!("{}", render_bridge_completion_execution_v1(&certificate));
        }
        [command, certificate_path] if command == "replay" => {
            let json = fs::read_to_string(certificate_path)
                .map_err(|error| format!("could not read certificate: {error}"))?;
            let replay = replay_bridge_completion_execution_v1_json(&json);
            if !replay.valid {
                return Err(format!(
                    "bridge-completion execution replay failed: {}",
                    replay.errors.join("; ")
                ));
            }
            println!(
                "bridge-completion execution replay valid; status={:?}; m3_v2_issued={}; m3_v1_remains_authoritative={}; m4_authorized={}",
                replay.status,
                replay.m3_v2_issued,
                replay.m3_v1_remains_authoritative,
                replay.m4_authorized
            );
        }
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example bridge_completion_execution_v1 [create-new <certificate.json> <report.md> | replay <certificate.json>]"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("bridge-completion-execution-v1".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("bridge-completion execution worker panicked".into()),
    }
}
