use pen_search::chronological_slot_map_v4::{
    CHRONOLOGICAL_SLOT_MAP_V4_CERTIFICATE_NAME, emit_chronological_slot_map_v4_create_new,
    issue_chronological_slot_map_v4_certificate, render_chronological_slot_map_v4_report,
    replay_chronological_slot_map_v4_json,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        let certificate = issue_chronological_slot_map_v4_certificate()?;
        let json = serde_json::to_string(&certificate)?;
        let replay = replay_chronological_slot_map_v4_json(&json);
        if !replay.valid {
            return Err(format!("in-memory replay failed: {}", replay.errors.join("; ")).into());
        }
        print!("{}", render_chronological_slot_map_v4_report(&certificate));
        return Ok(());
    }
    if let [command, directory] = args.as_slice() {
        let directory = Path::new(directory);
        let replay = match command.as_str() {
            "create-new" => {
                let certificate = emit_chronological_slot_map_v4_create_new(directory)?;
                replay_chronological_slot_map_v4_json(&serde_json::to_string(&certificate)?)
            }
            "replay" => replay_chronological_slot_map_v4_json(&std::fs::read_to_string(
                directory.join(CHRONOLOGICAL_SLOT_MAP_V4_CERTIFICATE_NAME),
            )?),
            _ => {
                return Err(
                    "usage: chronological_slot_map_v4 [create-new|replay DIRECTORY]".into(),
                );
            }
        };
        println!("{}", serde_json::to_string_pretty(&replay)?);
        return if replay.valid {
            Ok(())
        } else {
            Err(format!("artifact replay failed: {}", replay.errors.join("; ")).into())
        };
    }
    Err("usage: chronological_slot_map_v4 [create-new|replay DIRECTORY]".into())
}
