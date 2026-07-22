//! Frozen entrypoint for the score-blind EGP-v2 Stage 1--4 burned run.
//!
//! ```text
//! cargo run -p pen-search --release --example genesis_egp_v2_bootstrap -- \
//!     --out <new-egp-v2-artifact.json>
//! ```

use pen_search::egp_v2_bootstrap::{replay_egp_v2_bootstrap, run_egp_v2_bootstrap};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn main() {
    let mut out = None;
    let mut args = env::args().skip(1);
    while let Some(argument) = args.next() {
        if argument == "--out" {
            out = args.next();
        }
    }
    let out = out.expect("usage: --out <new-egp-v2-artifact.json>");
    if Path::new(&out)
        .file_name()
        .is_some_and(|name| name == "semantic_reselection.json")
    {
        panic!("refusing to overwrite the immutable EGP-v1 burned artifact");
    }

    let certificate = run_egp_v2_bootstrap().expect("EGP-v2 bootstrap run must publish");
    replay_egp_v2_bootstrap(&certificate).expect("EGP-v2 bootstrap self-replay must agree");

    for stage in &certificate.raw.stages {
        let winner = stage
            .winner
            .as_ref()
            .map(|winner| {
                format!(
                    "{} nu={} kappa={} rho={}",
                    winner.variant.variant_kind(),
                    winner.revised_nu,
                    winner.kappa,
                    winner.rho
                )
            })
            .unwrap_or_else(|| "NO CLEARING CANDIDATE".to_owned());
        println!(
            "stage {}: bar={} base={}/{} variants={} winner={}",
            stage.stage,
            stage.bar_before,
            stage.cone_admitted,
            stage.cone_deduped,
            stage.semantic_variants,
            winner
        );
    }
    println!("outcome: {:?}", certificate.raw.outcome);
    if let Some(divergence) = &certificate.first_divergence {
        println!(
            "first divergence: stage {} {} legacy={} revised={}",
            divergence.stage, divergence.field, divergence.legacy, divergence.revised
        );
    }
    println!("raw digest: {}", certificate.raw.raw_run_digest);
    println!("run digest: {}", certificate.run_digest);

    let json = serde_json::to_string_pretty(&certificate).expect("serialize EGP-v2 artifact");
    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&out)
        .expect("refusing to overwrite an existing burned-run artifact");
    writeln!(output, "{json}").expect("write EGP-v2 artifact");
    println!("wrote {out}");
}
