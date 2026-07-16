use pen_eval::epsilon_closure::build_epsilon_closure_run;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let mut freeze_commit = None;
    let mut specification_sha256 = None;
    let mut out = None;

    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--freeze-commit" => freeze_commit = args.next(),
            "--spec-sha256" => specification_sha256 = args.next(),
            "--out" => out = args.next().map(PathBuf::from),
            other => panic!("unknown argument: {other}"),
        }
    }

    let freeze_commit = freeze_commit.expect("--freeze-commit is required");
    let specification_sha256 = specification_sha256.expect("--spec-sha256 is required");
    let out = out.expect("--out is required");
    let report = build_epsilon_closure_run(freeze_commit, specification_sha256);
    let json = serde_json::to_string_pretty(&report).expect("serialize epsilon-closure run");
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&out)
        .unwrap_or_else(|error| panic!("refuse to overwrite {}: {error}", out.display()));
    writeln!(file, "{json}").unwrap_or_else(|error| panic!("write {}: {error}", out.display()));
}
