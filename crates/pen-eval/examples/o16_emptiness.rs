//! The O(16)-emptiness artifact (Guard-Rail Theorem, engine face;
//! ch_open_problems Problem Two, first step).
//!
//! Usage:
//!   cargo run -p pen-eval --example o16_emptiness -- \
//!     --out docs/o16_emptiness.json

use pen_eval::debt_guard::o16_emptiness_report;
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

    let report = o16_emptiness_report();
    let json = serde_json::to_string_pretty(&report).expect("serialize O(16) report");
    fs::write(&out, format!("{json}\n"))
        .unwrap_or_else(|error| panic!("write {}: {error}", out.display()));

    for record in &report.timeline {
        println!(
            "stage {:>2}{}: O = {:?}, focus = {}",
            record.stage,
            if record.pre_structural_band {
                " (pre-structural band)"
            } else {
                ""
            },
            record.required_packages,
            record.focus_family,
        );
    }
    println!(
        "O(16) written to {} — o16_empty: {}, guarded stages: {:?}, first debt-free: {:?}, \
         persistence: {}, guard_rail_verified: {}",
        out.display(),
        report.o16_empty,
        report.guarded_stages,
        report.first_debt_free_stage,
        report.discharge_persistence_holds,
        report.guard_rail_verified,
    );
}
