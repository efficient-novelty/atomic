use pen_search::milestone_certificate_v1::{
    emit_milestone_certificate_v1_create_new, issue_milestone_certificate_v1,
    render_milestone_certificate_v1, replay_milestone_certificate_v1_json,
};
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [] => {
            let certificate = issue_milestone_certificate_v1()?;
            print!("{}", render_milestone_certificate_v1(&certificate));
        }
        [command, docs] if command == "create-new" => {
            let docs = Path::new(docs);
            let certificate = emit_milestone_certificate_v1_create_new(
                &docs.join("milestone_certificate_v1.json"),
                &docs.join("MILESTONE_CERTIFICATE_V1.md"),
                &docs.join("MS1_INDEX.md"),
            )?;
            println!(
                "MS-1 create-new issuance complete: digest={}, laws={}, frontier={}, drift={}, M4={}",
                certificate.result_digest,
                certificate.legislative_record.len(),
                certificate.dnf4_frontier_verbatim.len(),
                certificate.drift_manifest.len(),
                certificate.m4_authorized
            );
        }
        [command, certificate] if command == "replay" => {
            let json = fs::read_to_string(certificate)?;
            let replay = replay_milestone_certificate_v1_json(&json);
            if !replay.valid {
                return Err(format!("MS-1 replay failed: {}", replay.errors.join("; ")).into());
            }
            println!(
                "MS-1 replay valid: laws={}, frontier={}, drift={}, M3v1={}, M4={}",
                replay.law_count,
                replay.frontier_row_count,
                replay.drift_occurrence_count,
                replay.m3_v1_remains_authoritative,
                replay.m4_authorized
            );
        }
        _ => {
            return Err(
                "usage: cargo run -p pen-search --example milestone_certificate_v1 [create-new <docs-dir> | replay <certificate.json>]"
                    .into(),
            );
        }
    }
    Ok(())
}
