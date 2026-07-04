use pen_eval::lambda_trigger::build_lambda_trigger_computation;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let mut freeze_commit = None;
    let mut eval_commit = None;
    let mut out = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--freeze-commit" => freeze_commit = args.next(),
            "--eval-commit" => eval_commit = args.next(),
            "--out" => out = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }

    let freeze_commit = freeze_commit.expect("--freeze-commit is required");
    let eval_commit = eval_commit.expect("--eval-commit is required");
    let out = out.expect("--out is required");
    let computation = build_lambda_trigger_computation(freeze_commit, eval_commit);
    let json = serde_json::to_string_pretty(&computation).expect("serialize computation");
    fs::write(&out, format!("{json}\n"))
        .unwrap_or_else(|error| panic!("write {}: {error}", out.display()));
}
