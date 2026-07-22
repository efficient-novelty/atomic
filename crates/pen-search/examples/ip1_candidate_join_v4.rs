use anyhow::{Context, Result, bail};
use pen_search::candidate_join::{candidate_join_v4_json_pretty, replay_candidate_join_v4_json};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let command = args.next().unwrap_or_else(|| "generate".to_owned());
    match command.as_str() {
        "generate" => {
            let schema3_path = args
                .next()
                .map(PathBuf::from)
                .context("generate requires the archived schema-3 JSON path")?;
            let schema4_path = args
                .next()
                .map(PathBuf::from)
                .context("generate requires a new schema-4 JSON path")?;
            if args.next().is_some() {
                bail!("generate requires exactly a schema-3 input and schema-4 output path");
            }
            let schema3 = std::fs::read(&schema3_path)
                .with_context(|| format!("read {}", schema3_path.display()))?;
            let json = candidate_join_v4_json_pretty(&schema3)?;
            let mut output = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&schema4_path)
                .with_context(|| {
                    format!(
                        "create new {} (refusing to overwrite an existing certificate)",
                        schema4_path.display()
                    )
                })?;
            output
                .write_all(json.as_bytes())
                .with_context(|| format!("write new {}", schema4_path.display()))?;
        }
        "replay" => {
            let schema3_path = args
                .next()
                .map(PathBuf::from)
                .context("replay requires the archived schema-3 JSON path")?;
            let schema4_path = args
                .next()
                .map(PathBuf::from)
                .context("replay requires the schema-4 JSON path")?;
            if args.next().is_some() {
                bail!("replay requires exactly a schema-3 and schema-4 JSON path");
            }
            let schema3 = std::fs::read(&schema3_path)
                .with_context(|| format!("read {}", schema3_path.display()))?;
            let schema4 = std::fs::read_to_string(&schema4_path)
                .with_context(|| format!("read {}", schema4_path.display()))?;
            let replay = replay_candidate_join_v4_json(&schema3, &schema4);
            println!("{}", serde_json::to_string_pretty(&replay)?);
            if !replay.valid {
                bail!("schema-4 candidate-join replay failed");
            }
        }
        other => bail!("unknown command {other:?}; expected generate or replay"),
    }
    Ok(())
}
