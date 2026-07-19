use anyhow::{Context, Result, bail};
use pen_search::ip1_certification_boundary::candidate_join::{
    ip1_candidate_join_json_pretty, replay_ip1_candidate_join_json,
};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "generate".to_owned());
    let path = args.next().map(PathBuf::from);
    match command.as_str() {
        "generate" => {
            if let Some(path) = path {
                let mut output = OpenOptions::new()
                    .write(true)
                    .create_new(true)
                    .open(&path)
                    .with_context(|| {
                        format!(
                            "create new {} (refusing to overwrite an existing certificate)",
                            path.display()
                        )
                    })?;
                let json = ip1_candidate_join_json_pretty();
                output
                    .write_all(json.as_bytes())
                    .with_context(|| format!("write new {}", path.display()))?;
            } else {
                let json = ip1_candidate_join_json_pretty();
                print!("{json}");
            }
        }
        "replay" => {
            let path = path.context("replay requires a certificate path")?;
            let json = std::fs::read_to_string(&path)
                .with_context(|| format!("read {}", path.display()))?;
            let replay = replay_ip1_candidate_join_json(&json);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                bail!("candidate-join replay failed");
            }
        }
        other => bail!("unknown command {other:?}; expected generate or replay"),
    }
    Ok(())
}
