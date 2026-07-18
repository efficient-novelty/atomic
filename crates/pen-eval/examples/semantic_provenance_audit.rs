//! Emit the honest, machine-readable provenance status of the canonical
//! Genesis replay.  No orbit inventories are fabricated by this example.

use pen_eval::semantic_provenance::{
    audit_semantic_debt, guarded_step15_internality_obligation, replay_genesis_provenance,
};
use serde_json::json;
use std::{env, fs, path::PathBuf};

fn main() {
    let output_path = parse_output_path();
    let report = json!({
        "semantic_debt": audit_semantic_debt(&[]),
        "guarded_step15_internality": guarded_step15_internality_obligation(),
        "historical_provenance_replay": replay_genesis_provenance(),
    });
    let rendered = serde_json::to_string_pretty(&report).expect("provenance report serializes");
    if let Some(path) = output_path {
        fs::write(&path, format!("{rendered}\n"))
            .unwrap_or_else(|error| panic!("failed to write {}: {error}", path.display()));
    } else {
        println!("{rendered}");
    }
}

fn parse_output_path() -> Option<PathBuf> {
    let mut args = env::args().skip(1);
    match (args.next().as_deref(), args.next(), args.next()) {
        (None, None, None) => None,
        (Some("--out"), Some(path), None) => Some(path.into()),
        _ => panic!("usage: semantic_provenance_audit [--out <path>]"),
    }
}
