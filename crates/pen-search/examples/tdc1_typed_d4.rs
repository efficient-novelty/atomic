//! Two-phase TDC-1 burn runner.
//!
//! Phase 1 emits a bar-independent typed certificate with create-new
//! semantics.  Freeze that artifact before invoking phase 2.  Phase 2 reads
//! and replay-checks the frozen certificate, then performs the registered
//! Z1--Z4 comparison into a separate create-new artifact.

use anyhow::{Context, Result, bail};
use pen_eval::tdc1::{
    Tdc1Certificate, Tdc1Comparison, build_tdc1_certificate, compare_frozen_certificate,
    replay_tdc1_certificate, replay_tdc1_comparison,
};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

fn write_json_new(path: &Path, value: &impl serde::Serialize) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value).context("serialize TDC-1 artifact")?;
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .with_context(|| format!("refusing to overwrite {}", path.display()))?;
    file.write_all(&bytes)
        .with_context(|| format!("write {}", path.display()))?;
    file.write_all(b"\n")
        .with_context(|| format!("finish {}", path.display()))?;
    file.sync_all()
        .with_context(|| format!("sync {}", path.display()))?;
    Ok(())
}

fn usage() -> ! {
    eprintln!(
        "usage:\n  tdc1_typed_d4 certificate <new-certificate.json>\n  \
         tdc1_typed_d4 compare <frozen-certificate.json> <new-comparison.json>\n  \
         tdc1_typed_d4 replay <certificate.json>\n  \
         tdc1_typed_d4 replay-comparison <certificate.json> <comparison.json>"
    );
    std::process::exit(2)
}

fn read_certificate(path: &Path) -> Result<Tdc1Certificate> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parse {}", path.display()))
}

fn read_comparison(path: &Path) -> Result<Tdc1Comparison> {
    let bytes = std::fs::read(path).with_context(|| format!("read {}", path.display()))?;
    serde_json::from_slice(&bytes).with_context(|| format!("parse {}", path.display()))
}

fn main() -> Result<()> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let Some(mode) = args.first().map(String::as_str) else {
        usage();
    };
    match mode {
        "certificate" => {
            if args.len() != 2 {
                usage();
            }
            let output = PathBuf::from(&args[1]);
            let certificate = build_tdc1_certificate().context("build TDC-1 certificate")?;
            write_json_new(&output, &certificate)?;
            println!(
                "wrote certificate {} ({})",
                output.display(),
                certificate.certificate_digest
            );
        }
        "compare" => {
            if args.len() != 3 {
                usage();
            }
            let certificate_path = PathBuf::from(&args[1]);
            let output = PathBuf::from(&args[2]);
            if certificate_path == output {
                bail!("comparison output must be separate from the frozen certificate");
            }
            let certificate = read_certificate(&certificate_path)?;
            let comparison = compare_frozen_certificate(&certificate)
                .context("replay and compare frozen TDC-1 certificate")?;
            write_json_new(&output, &comparison)?;
            println!(
                "wrote comparison {} ({})",
                output.display(),
                comparison.comparison_digest
            );
        }
        "replay" => {
            if args.len() != 2 {
                usage();
            }
            let certificate_path = PathBuf::from(&args[1]);
            let certificate = read_certificate(&certificate_path)?;
            replay_tdc1_certificate(&certificate).context("replay TDC-1 certificate")?;
            println!("replay passed: {}", certificate.certificate_digest);
        }
        "replay-comparison" => {
            if args.len() != 3 {
                usage();
            }
            let certificate_path = PathBuf::from(&args[1]);
            let comparison_path = PathBuf::from(&args[2]);
            let certificate = read_certificate(&certificate_path)?;
            let comparison = read_comparison(&comparison_path)?;
            replay_tdc1_comparison(&certificate, &comparison).context("replay TDC-1 comparison")?;
            println!("comparison replay passed: {}", comparison.comparison_digest);
        }
        _ => usage(),
    }
    Ok(())
}
