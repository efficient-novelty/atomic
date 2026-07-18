//! SH-1 T1--T3 measurement artifact.
//!
//! This executable contains no hypothesis adjudication. It emits the factual
//! seal-and-continue trace, all three exact staircase calculations, and the
//! declared capacity-oracle trust boundary. Interpretation is performed only
//! after this JSON has been sealed.

use pen_eval::sh1_staircase::calculate_registered_t2_suite;
use pen_search::halting_probe::{
    DEFAULT_STEP16_CONTINUATION_SURVIVOR, run_seal_and_continue_probe,
};
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

const ARTIFACT_SCHEMA: &str = "sh1.experiment.measurements.v1";
const HYPOTHESIS_FREEZE_COMMIT: &str = "7ab526f";
const T4_FREEZE_COMMIT: &str = "4b408c8";

fn main() {
    let mut out = None;
    let mut steps = 4_usize;
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--out" => out = args.next().map(PathBuf::from),
            "--steps" => {
                steps = args
                    .next()
                    .expect("--steps requires a value")
                    .parse()
                    .expect("--steps must be a positive integer")
            }
            other => panic!("unknown argument: {other}"),
        }
    }
    let out = out.expect("--out is required");

    let t1 = run_seal_and_continue_probe(DEFAULT_STEP16_CONTINUATION_SURVIVOR, steps)
        .expect("registered T1 seal-and-continue probe must execute");
    let t2_t3 = calculate_registered_t2_suite()
        .expect("registered exact staircase and declared capacity suite must execute");

    let payload = serde_json::json!({
        "artifact_schema": ARTIFACT_SCHEMA,
        "date": "2026-07-18",
        "status": "burned_measurements_no_interpretation",
        "hypothesis_freeze": {
            "path": "docs/staircase_hypothesis.md",
            "commit": HYPOTHESIS_FREEZE_COMMIT,
        },
        "blind_t4_freeze": {
            "path": "docs/SH1_T4_BLIND_VALUATION_SPEC.md",
            "commit": T4_FREEZE_COMMIT,
        },
        "t1": t1,
        "t2_t3": t2_t3,
    });
    let json = serde_json::to_string_pretty(&payload).expect("SH-1 payload serializes");

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&out)
        .unwrap_or_else(|error| {
            panic!(
                "refusing to overwrite SH-1 artifact {}: {error}",
                out.display()
            )
        });
    file.write_all(json.as_bytes())
        .and_then(|_| file.write_all(b"\n"))
        .unwrap_or_else(|error| panic!("write {}: {error}", out.display()));

    println!(
        "SH-1 measurements burned to {}: T1 seals {}, quadratic plateaus {}, capacity halt {:?}",
        out.display(),
        payload["t1"]["completed_seals"],
        payload["t2_t3"]["formed_quadratic"]["plateaus"]
            .as_array()
            .map_or(0, Vec::len),
        payload["t2_t3"]["capacity_gated_quadratic"]["halted_at"],
    );
}
