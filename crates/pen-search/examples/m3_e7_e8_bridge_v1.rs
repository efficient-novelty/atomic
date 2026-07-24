use pen_search::m3_e7_e8_bridge_v1::{
    emit_m3_e7_e8_bridge_v1_create_new, issue_m3_e7_e8_bridge_v1, render_m3_e7_e8_bridge_v1,
    replay_m3_e7_e8_bridge_v1, replay_m3_e7_e8_bridge_v1_json,
};
use std::fs;
use std::path::Path;

fn run() -> Result<(), String> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => {
            let certificate = issue_m3_e7_e8_bridge_v1().map_err(|error| error.to_string())?;
            let replay = replay_m3_e7_e8_bridge_v1(&certificate);
            if !replay.valid {
                return Err(format!(
                    "new M-3 certificate did not replay: {}",
                    replay.errors.join("; ")
                ));
            }
            print!("{}", render_m3_e7_e8_bridge_v1(&certificate));
        }
        [command, certificate_path, report_path] if command == "create-new" => {
            let certificate = emit_m3_e7_e8_bridge_v1_create_new(
                Path::new(certificate_path),
                Path::new(report_path),
            )
            .map_err(|error| error.to_string())?;
            print!("{}", render_m3_e7_e8_bridge_v1(&certificate));
        }
        [command, certificate_path] if command == "replay" => {
            let json = fs::read_to_string(certificate_path)
                .map_err(|error| format!("could not read certificate: {error}"))?;
            let replay = replay_m3_e7_e8_bridge_v1_json(&json);
            if !replay.valid {
                return Err(format!("M-3 replay failed: {}", replay.errors.join("; ")));
            }
            println!(
                "M-3 replay valid; status={:?}; bridge_claim_issued={}; m4_authorized={}",
                replay.m3_status, replay.bridge_claim_issued, replay.m4_authorized
            );
        }
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example m3_e7_e8_bridge_v1 [create-new <certificate.json> <report.md> | replay <certificate.json>]"
                    .to_owned(),
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker = std::thread::Builder::new()
        .name("m3-e7-e8-bridge-v1".to_owned())
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)?;
    match worker.join() {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error.into()),
        Err(_) => Err("M-3 worker panicked".into()),
    }
}
