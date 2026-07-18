//! Frozen reproduction path for the Phase 5b burned reselection run
//! (SEMANTIC_NORMALIZATION_PROGRAM §5b):
//!
//! ```text
//! cargo run -p pen-search --release --example genesis_semantic_reselection -- \
//!     --out docs/semantic_reselection.json
//! ```

use pen_search::semantic_reselection::{replay_semantic_reselection, run_semantic_reselection};
use std::env;
use std::fs;

fn main() {
    let mut out = None;
    let mut args = env::args().skip(1);
    while let Some(argument) = args.next() {
        if argument == "--out" {
            out = args.next();
        }
    }
    let out = out.expect("usage: --out <path>");

    let certificate = match run_semantic_reselection() {
        Ok(certificate) => certificate,
        Err(error) => {
            // A failed stage is itself a certified result: publish it.
            eprintln!("reselection halted: {error}");
            std::process::exit(2);
        }
    };
    replay_semantic_reselection(&certificate).expect("self-replay must reconstruct the run");

    for stage in &certificate.stages {
        let winner_summary = stage
            .winner
            .as_ref()
            .map(|winner| {
                format!(
                    "winner nu {} kappa {} rho {}",
                    winner.revised_nu, winner.kappa, winner.rho
                )
            })
            .unwrap_or_else(|| "NO CLEARING CANDIDATE".to_string());
        println!(
            "stage {:2}: bar {:>9} cone {:>4} (admitted {:>4}, deduped {:>4}) clearing {:>3} \
             {} | legacy nu {:>3} winner_match {} score_match {}",
            stage.stage,
            stage.bar,
            stage.cone_enumerated,
            stage.cone_admitted,
            stage.cone_deduped,
            stage.clearing,
            winner_summary,
            stage.legacy_nu,
            stage.winner_matches_legacy,
            stage.score_matches_legacy,
        );
    }
    println!("outcome: {:?}", certificate.outcome);
    println!("winners_reenact: {}", certificate.winners_reenact);
    println!("semantic_reenactment: {}", certificate.semantic_reenactment);
    if let Some(divergence) = &certificate.first_divergence {
        println!(
            "first_divergence: stage {} field {} legacy {} revised {}",
            divergence.stage, divergence.field, divergence.legacy, divergence.revised
        );
    }
    println!(
        "revised_bar_16: {} (legacy {}, matches {})",
        certificate
            .revised_bar_16
            .as_deref()
            .unwrap_or("none (run halted before stage 15)"),
        certificate.legacy_bar_16,
        certificate.bar_16_matches
    );
    println!("digest: {}", certificate.digest);

    let json = serde_json::to_string_pretty(&certificate).expect("serialize certificate");
    fs::write(&out, format!("{json}\n")).expect("write artifact");
    println!("wrote {out}");
}
