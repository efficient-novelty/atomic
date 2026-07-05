//! The Stage-2 blind run (`docs/RUNTIME_CALCULUS.md` §5): a single
//! post-freeze execution computing, for the frozen v2 stratum variants, the
//! ν profiles, the crossing index (first k with e_k < 1), and the exported
//! band composition. Output goes to `docs/runtime_calculus_run.json`.
//! Report whatever comes out.
//!
//! Usage:
//!   cargo run -p pen-eval --example runtime_calculus_run -- \
//!     --freeze-commit <sha> --eval-commit <sha> \
//!     --out docs/runtime_calculus_run.json \
//!     [--steps 24] [--chain-levels 6]

use pen_eval::runtime_report::build_runtime_calculus_run;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut freeze_commit = None;
    let mut eval_commit = None;
    let mut out = None;
    let mut steps: u32 = 24;
    let mut chain_levels: u32 = 6;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--freeze-commit" => freeze_commit = args.next(),
            "--eval-commit" => eval_commit = args.next(),
            "--out" => out = args.next().map(PathBuf::from),
            "--steps" => {
                steps = args
                    .next()
                    .expect("--steps needs a value")
                    .parse()
                    .expect("--steps must be a positive integer");
            }
            "--chain-levels" => {
                chain_levels = args
                    .next()
                    .expect("--chain-levels needs a value")
                    .parse()
                    .expect("--chain-levels must be a positive integer");
            }
            other => panic!("unknown argument: {other}"),
        }
    }

    let freeze_commit = freeze_commit.expect("--freeze-commit is required");
    let eval_commit = eval_commit.expect("--eval-commit is required");
    let out = out.expect("--out is required");

    let run = build_runtime_calculus_run(freeze_commit, eval_commit, steps, chain_levels);
    let json = serde_json::to_string_pretty(&run).expect("serialize stage-2 run");
    fs::write(&out, format!("{json}\n"))
        .unwrap_or_else(|error| panic!("write {}: {error}", out.display()));
    println!("stage-2 run written to {}", out.display());
}
