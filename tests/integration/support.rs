#![allow(dead_code)]

use serde::Deserialize;
use serde_json::Value;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct TrajectoryStepFixture {
    pub step_index: u32,
    pub label: String,
    pub objective_bar: String,
    pub candidate_hash: String,
    pub canonical_hash: String,
    pub bit_kappa: u16,
    pub clause_kappa: u16,
    pub nu: u16,
    pub rho: String,
}

pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
}

pub fn fixtures_root() -> PathBuf {
    workspace_root().join("tests").join("fixtures")
}

pub fn temp_dir(name: &str) -> PathBuf {
    let id = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("time should move forward")
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("pen-cli-integration-{name}-{id}"));
    fs::create_dir_all(&dir).expect("temp dir should exist");
    dir
}

pub fn run_pen_cli<I, S>(args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    Command::new(env!("CARGO_BIN_EXE_pen-cli"))
        .args(args)
        .output()
        .expect("pen-cli should run")
}

pub fn assert_success(output: Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    assert!(
        output.status.success(),
        "command failed\nstatus: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        stdout,
        stderr
    );
    format!("{stdout}{stderr}")
}

pub fn read_json(path: &Path) -> Value {
    let text = fs::read_to_string(path).unwrap_or_else(|error| {
        panic!("failed to read {}: {error}", path.display());
    });
    serde_json::from_str(&text).unwrap_or_else(|error| {
        panic!("failed to parse {}: {error}", path.display());
    })
}

pub fn read_text(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| {
        panic!("failed to read {}: {error}", path.display());
    })
}

pub fn load_trajectory_fixture(name: &str) -> Vec<TrajectoryStepFixture> {
    let path = fixtures_root().join("trajectory").join(name);
    let text = read_text(&path);
    serde_json::from_str(&text).unwrap_or_else(|error| {
        panic!("failed to parse {}: {error}", path.display());
    })
}

pub fn compact_step_summaries(run_dir: &Path) -> Vec<TrajectoryStepFixture> {
    let steps_dir = run_dir.join("reports").join("steps");
    let mut files = fs::read_dir(&steps_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", steps_dir.display()))
        .map(|entry| entry.expect("step entry should exist").path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    files.sort();
    files
        .iter()
        .map(|path| compact_step_summary(path))
        .collect()
}

pub fn normalize_checkpoint(mut value: Value) -> Value {
    value["accepted_utc"] = Value::String("<accepted_utc>".to_owned());
    value
}

pub fn normalize_export_manifest(mut value: Value) -> Value {
    value["generated_utc"] = Value::String("<generated_utc>".to_owned());
    value["output_dir"] = Value::String("<output_dir>".to_owned());
    value
}

fn compact_step_summary(path: &Path) -> TrajectoryStepFixture {
    let value = read_json(path);
    TrajectoryStepFixture {
        step_index: value["step_index"].as_u64().expect("step_index") as u32,
        label: value["label"].as_str().expect("label").to_owned(),
        objective_bar: rational_to_string(&value["objective_bar"]),
        candidate_hash: value["accepted"]["candidate_hash"]
            .as_str()
            .expect("candidate_hash")
            .to_owned(),
        canonical_hash: value["accepted"]["canonical_hash"]
            .as_str()
            .expect("canonical_hash")
            .to_owned(),
        bit_kappa: value["accepted"]["bit_kappa"].as_u64().expect("bit_kappa") as u16,
        clause_kappa: value["accepted"]["clause_kappa"]
            .as_u64()
            .expect("clause_kappa") as u16,
        nu: value["accepted"]["nu"].as_u64().expect("nu") as u16,
        rho: rational_to_string(&value["accepted"]["rho"]),
    }
}

fn rational_to_string(value: &Value) -> String {
    let num = value["num"].as_i64().expect("rational numerator");
    let den = value["den"].as_i64().expect("rational denominator");
    format!("{num}/{den}")
}
