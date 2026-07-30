use pen_engine::{
    LawV2H4ContinuationOutcomeV1, law_v2_h4_continuation_json_pretty_v1,
    law_v2_h4_continuation_manifest_json_pretty_v1, replay_law_v2_h4_continuation_report_v1,
};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args_os().skip(1);
    match arguments.next() {
        None => println!("{}", law_v2_h4_continuation_json_pretty_v1()),
        Some(flag) if flag == "--manifest" => {
            if arguments.next().is_some() {
                return Err("--manifest accepts no additional arguments".into());
            }
            println!("{}", law_v2_h4_continuation_manifest_json_pretty_v1());
        }
        Some(flag) if flag == "--replay" => {
            let path = PathBuf::from(arguments.next().ok_or("--replay requires a report path")?);
            if arguments.next().is_some() {
                return Err("--replay accepts exactly one report path".into());
            }
            let bytes = std::fs::read(path)?;
            let outcome: LawV2H4ContinuationOutcomeV1 = serde_json::from_slice(&bytes)?;
            let valid = match outcome {
                LawV2H4ContinuationOutcomeV1::Halted { report } => {
                    replay_law_v2_h4_continuation_report_v1(&report)
                }
                LawV2H4ContinuationOutcomeV1::Unknown { .. } => false,
            };
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "status": if valid { "valid" } else { "invalid" },
                    "valid": valid,
                }))?
            );
            if !valid {
                std::process::exit(1);
            }
        }
        Some(flag) => {
            return Err(format!("unsupported argument: {}", flag.to_string_lossy()).into());
        }
    }
    Ok(())
}
