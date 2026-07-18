//! Emit the dual Step-16 certificate.
//!
//! Usage:
//!   cargo run -p pen-search --release --example genesis_certified_halt -- \
//!     --out docs/certified_halt_verification.json

use pen_search::certified_halt::{
    certified_halt_json_pretty, replay_certified_halt_certificate_json,
};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut out = None;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => out = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }
    let out = out.expect("--out is required");
    let json = certified_halt_json_pretty();
    let replay = replay_certified_halt_certificate_json(&json);
    assert!(
        replay.valid,
        "fresh certificate must replay: {:?}",
        replay.errors
    );

    fs::write(&out, format!("{json}\n"))
        .unwrap_or_else(|error| panic!("write {}: {error}", out.display()));
    println!(
        "wrote {}: shipped_sat={}, corrected_calculus_unsat={}, original_global_halt_proven={}",
        out.display(),
        replay.shipped_sat,
        replay.corrected_calculus_unsat,
        replay.original_global_halt_proven,
    );
}
