use anyhow::{Context, Result, bail};
use pen_search::ip1_certification_boundary::{A5Adjudication, ip1_json_pretty, replay_ip1_json};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn main() -> Result<()> {
    let mut output = PathBuf::from("docs/ip1_certification_boundary.json");
    // A5 was explicitly adopted by the law-maker after preregistration.
    let mut adjudication = A5Adjudication::Adopted;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => {
                output = PathBuf::from(args.next().context("--out requires a path")?);
            }
            "--a5" => {
                let value = args.next().context("--a5 requires a value")?;
                adjudication = A5Adjudication::parse(&value)
                    .with_context(|| format!("invalid --a5 value {value:?}"))?;
            }
            "--help" | "-h" => {
                println!(
                    "usage: ip1_certification_boundary [--out PATH] [--a5 undecided|adopted|rejected]"
                );
                return Ok(());
            }
            other => bail!("unknown argument {other:?}"),
        }
    }

    if let Some(parent) = output.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("creating {}", parent.display()))?;
    }
    let mut sink = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&output)
        .with_context(|| {
            format!(
                "create new {} (refusing to overwrite an existing burned artifact)",
                output.display()
            )
        })?;
    let json = ip1_json_pretty(adjudication);
    let replay = replay_ip1_json(&json);
    if !replay.valid {
        bail!("fresh IP-1 certificate failed replay: {:?}", replay.errors);
    }
    sink.write_all(format!("{json}\n").as_bytes())
        .with_context(|| format!("writing new {}", output.display()))?;
    println!(
        "wrote {}: status={}, mechanical_complete={}, theorem={}",
        output.display(),
        replay.theorem_status,
        replay.mechanical_boundary_complete,
        replay.no_certifiably_clearing_step16
    );
    Ok(())
}
