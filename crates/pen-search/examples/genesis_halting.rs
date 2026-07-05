//! T1 artifact — the Genesis squeeze verification
//! (`note_canonical_course.md` §7, T1; ch_genesis_mathematics "Why the
//! Sequence Halts"). Drives the strict guarded lane one step past Genesis
//! (reference prefix replayed, engine step machinery unaltered) and records
//! the lane's own step-16 verdict, plus the internal-class ν = 0 check.
//!
//! Usage:
//!   cargo run -p pen-search --release --example genesis_halting -- \
//!     --out docs/halting_verification.json
//!
//! Report whatever comes out: an accepted sixteenth extension is falsifier
//! (a) of the canonical-course note and contradicts the book's halt section.

use pen_search::halting_probe::{
    run_adversarial_probe, step16_surface_diagnostics, verify_genesis_squeeze,
    LaneStep16Outcome,
};
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut out = None;
    let mut search = false;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => out = args.next().map(PathBuf::from),
            // The full lane search is opt-in: both materializing paths have
            // exhausted desktop memory on the step-16 surface. Measure
            // first (default), search once a fitting strategy is chosen.
            "--search" => search = true,
            other => panic!("unknown argument: {other}"),
        }
    }
    let out = out.expect("--out is required");

    if !search {
        let diagnostics = step16_surface_diagnostics();
        let adversarial = run_adversarial_probe();
        let payload = serde_json::json!({
            "artifact": "genesis_halting_adversarial_probe",
            "date": pen_search::halting_probe::T1_DATE,
            "phase": "surface diagnostics + adversarial gate probe \
                      (exhaustive search infeasible at the measured surface; \
                      pass --search only with a strategy fitting it)",
            "surface_diagnostics": diagnostics,
            "adversarial_probe": adversarial,
        });
        let json = serde_json::to_string_pretty(&payload).expect("serialize probe report");
        fs::write(&out, format!("{json}\n"))
            .unwrap_or_else(|error| panic!("write {}: {error}", out.display()));
        println!(
            "T1 probe written to {} — kappa band {}..={}, bar {}, adversarial \
             candidates {}, clearing survivors {}, squeeze_supported: {}",
            out.display(),
            diagnostics.admissibility.min_clause_kappa,
            diagnostics.admissibility.max_clause_kappa,
            adversarial.bar,
            adversarial.candidates.len(),
            adversarial.clearing_survivors,
            adversarial.squeeze_supported,
        );
        for trace in &adversarial.candidates {
            println!(
                "  {}: identified {}, admissibility {} ({}), type_ok {}, connected {}, \
                 nu {}, rho {:?}, clears {}, SURVIVES {}",
                trace.name,
                trace.identified_with_sealed_structure,
                trace.admissibility_class,
                trace.admissibility_reason,
                trace.type_checks,
                trace.connectivity_passes,
                trace.nu_total,
                trace.rho,
                trace.clears_bar,
                trace.survives_all_gates_and_clears,
            );
        }
        return;
    }

    let report = verify_genesis_squeeze();
    let json = serde_json::to_string_pretty(&report).expect("serialize T1 report");
    fs::write(&out, format!("{json}\n"))
        .unwrap_or_else(|error| panic!("write {}: {error}", out.display()));

    let outcome = match &report.outcome {
        LaneStep16Outcome::SqueezeHeld { engine_report } => {
            format!("squeeze held ({engine_report})")
        }
        LaneStep16Outcome::Accepted { rho, objective_bar, .. } => format!(
            "FALSIFIER (a): sixteenth extension sealed, rho {rho} vs bar {objective_bar}"
        ),
        LaneStep16Outcome::ProbeFailure { error } => format!("PROBE FAILURE: {error}"),
    };
    println!(
        "T1 written to {} — bar {}, {}, internal_all_zero: {}, squeeze_verified: {}",
        out.display(),
        report.bar_frozen_constant,
        outcome,
        report.internal_all_zero,
        report.squeeze_verified
    );
}
