//! PA-1b create-new and replay driver.
//!
//! Usage:
//!   cargo run -p pen-search --example pa1_open_competition_v1 -- create-new <certificate.json> <report.md>
//!   cargo run -p pen-search --example pa1_open_competition_v1 -- replay <certificate.json> <report.md>

use pen_search::pa1_open_competition_v1::{
    Pa1OpenCompetitionV1Certificate, emit_pa1_open_competition_v1_create_new, pa1b_bytes_blake3,
    replay_pa1_open_competition_v1_json, verify_pa1_open_competition_report,
};
use std::fs;
use std::path::Path;

type Fallible<T> = Result<T, Box<dyn std::error::Error>>;

fn main() -> Fallible<()> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [command, certificate, report] if command == "create-new" => {
            let issued =
                emit_pa1_open_competition_v1_create_new(Path::new(certificate), Path::new(report))?;
            let report_bytes = fs::read(report)?;
            println!(
                "PA-1b created: certificate={} report_blake3={} finding={}",
                issued.result_digest,
                pa1b_bytes_blake3(&report_bytes),
                issued.global_finding,
            );
            Ok(())
        }
        [command, certificate, report] if command == "replay" => {
            let json = fs::read_to_string(certificate)?;
            let replay = replay_pa1_open_competition_v1_json(&json);
            if !replay.valid {
                return Err(format!("PA-1b replay failed: {}", replay.errors.join("; ")).into());
            }
            let certificate: Pa1OpenCompetitionV1Certificate = serde_json::from_str(&json)?;
            let report_text = fs::read_to_string(report)?;
            verify_pa1_open_competition_report(&certificate, &report_text)?;
            println!(
                "PA-1b replay valid: certificate={} report_blake3={} qualifying_surfaces={} finding={}",
                certificate.result_digest,
                pa1b_bytes_blake3(report_text.as_bytes()),
                replay.qualifying_surface_count,
                replay.global_finding,
            );
            Ok(())
        }
        _ => Err(
            "usage: pa1_open_competition_v1 [create-new|replay] <certificate.json> <report.md>"
                .into(),
        ),
    }
}
