use pen_search::bi0_semantic_register_v4::{
    BI0_SEMANTIC_REGISTER_V4_CERTIFICATE_NAME, emit_bi0_semantic_register_v4_create_new,
    issue_bi0_semantic_register_v4_certificate, render_bi0_semantic_register_v4_report,
    replay_bi0_semantic_register_v4_json,
};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    if args.is_empty() {
        let certificate = issue_bi0_semantic_register_v4_certificate()?;
        let replay = replay_bi0_semantic_register_v4_json(&serde_json::to_string(&certificate)?);
        if !replay.valid {
            return Err(format!("in-memory replay failed: {}", replay.errors.join("; ")).into());
        }
        print!("{}", render_bi0_semantic_register_v4_report(&certificate));
        return Ok(());
    }
    if let [command, directory] = args.as_slice() {
        let directory = Path::new(directory);
        let replay = match command.as_str() {
            "create-new" => {
                let certificate = emit_bi0_semantic_register_v4_create_new(directory)?;
                replay_bi0_semantic_register_v4_json(&serde_json::to_string(&certificate)?)
            }
            "replay" => replay_bi0_semantic_register_v4_json(&std::fs::read_to_string(
                directory.join(BI0_SEMANTIC_REGISTER_V4_CERTIFICATE_NAME),
            )?),
            _ => {
                return Err("usage: bi0_semantic_register_v4 [create-new|replay DIRECTORY]".into());
            }
        };
        println!("{}", serde_json::to_string_pretty(&replay)?);
        return if replay.valid {
            Ok(())
        } else {
            Err(format!("artifact replay failed: {}", replay.errors.join("; ")).into())
        };
    }
    Err("usage: bi0_semantic_register_v4 [create-new|replay DIRECTORY]".into())
}
