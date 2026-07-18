//! Emit the honest, machine-readable provenance status of the canonical
//! Genesis replay.
//!
//! Since the semantic normalization program (Phases 1-5b), the semantic
//! debt audit runs over the KERNEL-EXTRACTED orbit inventories (nothing
//! is fabricated: the inventories are the image of the total
//! deterministic window extraction with kernel-verified J2/J3/locality),
//! the guarded Step-15 internality carries its kernel certificate, and
//! the burned reselection outcome is referenced as data. The legacy
//! conditional views (empty-inventory audit, structural replay with null
//! revised scores) are retained side by side: the sealed structural
//! trace is regression input, and the program's outcome (B) means there
//! is no completed revised history to overwrite it with.

use pen_eval::demand_orbits::kernel_stage_inventories;
use pen_eval::internality::certify_guarded_step15_internality;
use pen_eval::semantic_provenance::{
    audit_semantic_debt, guarded_step15_internality_obligation, replay_genesis_provenance,
};
use pen_eval::typed_families::predecessor_closure;
use pen_type::elaborate::SealedSignature;
use serde_json::json;
use std::{env, fs, path::PathBuf};

fn main() {
    let output_path = parse_output_path();

    let signature = SealedSignature::genesis_del_h15();
    let closure = predecessor_closure(&signature).expect("predecessor closure");
    let orbits =
        kernel_stage_inventories(&signature, &closure).expect("kernel orbit inventories");
    let kernel_internality =
        certify_guarded_step15_internality(&signature, &closure).expect("internality");

    let report = json!({
        // The kernel-backed audit: J2/J3/locality Verified, semantic
        // O(16) decided.
        "semantic_debt": audit_semantic_debt(&orbits.inventories),
        // Retained conditional view (empty inventories): shows exactly
        // what is refused without the kernel evidence.
        "semantic_debt_without_kernel_inventories": audit_semantic_debt(&[]),
        "kernel_orbit_extraction": {
            "signature_digest": orbits.signature_digest,
            "closure_digest": orbits.closure_digest,
            "derivation_hash": orbits.derivation_hash,
            "locality_ledger_digest": orbits.locality_ledger.digest,
            "locality_holds": orbits.locality_ledger.holds,
        },
        // The legacy obligation record (what was demanded)...
        "guarded_step15_internality": guarded_step15_internality_obligation(),
        // ...and the kernel certificate that discharges it for the
        // strict guarded subspace.
        "guarded_step15_internality_kernel": kernel_internality,
        // The sealed structural replay: regression input, revised
        // columns null BY THE CERTIFIED OUTCOME (B) — the burned
        // reselection halts at stage 2, so no completed revised history
        // exists to fill them (docs/semantic_reselection.json).
        "historical_provenance_replay": replay_genesis_provenance(),
        "semantic_reselection_reference": {
            "artifact": "docs/semantic_reselection.json",
            "outcome": "halted_no_clearing_candidate at stage 2",
            "first_divergence": "stage 1, field score (legacy 1, revised 2)",
        },
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
