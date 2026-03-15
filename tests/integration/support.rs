#![allow(dead_code)]

use fs2::FileExt;
use serde::Deserialize;
use serde_json::Value;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::fs;
use std::io::ErrorKind;
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
pub struct SearchSpaceStepFixture {
    pub step_index: u32,
    pub enumerated: u64,
    pub well_formed: u64,
    pub admissibility_rejected: u64,
    pub evaluated: u64,
    pub canonical: u64,
    pub semantically_minimal: u64,
    pub retained: u64,
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
    let _lock = integration_process_lock();
    Command::new(env!("CARGO_BIN_EXE_pen-cli"))
        .args(args)
        .output()
        .expect("pen-cli should run")
}

pub fn run_compare_runs<I, S>(args: I) -> Output
where
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let script = workspace_root().join("scripts").join("compare_runs.py");
    let args = args
        .into_iter()
        .map(|arg| arg.as_ref().to_os_string())
        .collect::<Vec<OsString>>();

    for (program, prefix) in python_invocations() {
        let mut command = Command::new(program);
        command.args(prefix).arg(&script).args(&args);
        match command.output() {
            Ok(output) => return output,
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => panic!("failed to run {}: {error}", script.display()),
        }
    }

    panic!("no Python interpreter found for {}", script.display());
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

pub fn latest_frontier_manifest_path(run_dir: &Path) -> PathBuf {
    let run_manifest = read_json(&run_dir.join("run.json"));
    let step_index = run_manifest["position"]["completed_step"]
        .as_u64()
        .expect("completed_step");
    let band_index = run_manifest["position"]["active_band"]
        .as_u64()
        .expect("active_band");
    run_dir
        .join("checkpoints")
        .join("frontier")
        .join(format!("step-{step_index:02}"))
        .join(format!("band-{band_index:02}"))
        .join("frontier.manifest.json")
}

pub fn mutate_latest_frontier_manifest(run_dir: &Path, mutate: impl FnOnce(&mut Value)) {
    let path = latest_frontier_manifest_path(run_dir);
    let mut manifest = read_json(&path);
    mutate(&mut manifest);
    fs::write(
        &path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(&manifest).expect("serialize manifest")
        ),
    )
    .expect("rewrite frontier manifest");
}

pub fn rewrite_config_workers(path: &Path, workers: u16) {
    let updated = read_text(path).replace("workers = 1", &format!("workers = {workers}"));
    fs::write(path, updated).expect("rewrite config");
}

pub fn write_pressure_config(path: &Path) {
    fs::write(path, pressure_config()).expect("write pressure config");
}

pub fn load_trajectory_fixture(name: &str) -> Vec<TrajectoryStepFixture> {
    let path = fixtures_root().join("trajectory").join(name);
    let text = read_text(&path);
    serde_json::from_str(&text).unwrap_or_else(|error| {
        panic!("failed to parse {}: {error}", path.display());
    })
}

pub fn load_search_space_fixture(name: &str) -> Vec<SearchSpaceStepFixture> {
    let path = fixtures_root().join("search_space").join(name);
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

pub fn compact_search_space_stats(run_dir: &Path) -> Vec<SearchSpaceStepFixture> {
    let steps_dir = run_dir.join("reports").join("steps");
    let mut files = fs::read_dir(&steps_dir)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", steps_dir.display()))
        .map(|entry| entry.expect("step entry should exist").path())
        .filter(|path| path.extension().and_then(|ext| ext.to_str()) == Some("json"))
        .collect::<Vec<_>>();
    files.sort();
    files
        .iter()
        .map(|path| compact_search_space_step(path))
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

fn compact_search_space_step(path: &Path) -> SearchSpaceStepFixture {
    let value = read_json(path);
    let stats = &value["search_stats"];
    SearchSpaceStepFixture {
        step_index: value["step_index"].as_u64().expect("step_index") as u32,
        enumerated: stats["enumerated_candidates"].as_u64().expect("enumerated"),
        well_formed: stats["well_formed_candidates"]
            .as_u64()
            .expect("well_formed"),
        admissibility_rejected: stats["admissibility_rejections"]
            .as_u64()
            .expect("admissibility_rejected"),
        evaluated: stats["evaluated_candidates"].as_u64().expect("evaluated"),
        canonical: stats["canonical_candidates"].as_u64().expect("canonical"),
        semantically_minimal: stats["semantically_minimal_candidates"]
            .as_u64()
            .expect("semantically_minimal"),
        retained: stats["retained_candidates"].as_u64().expect("retained"),
    }
}

fn rational_to_string(value: &Value) -> String {
    let num = value["num"].as_i64().expect("rational numerator");
    let den = value["den"].as_i64().expect("rational denominator");
    format!("{num}/{den}")
}

fn pressure_config() -> String {
    read_text(&workspace_root().join("configs").join("debug.toml"))
        .replace("target_rss_gib = 4.0", "target_rss_gib = 0.0000001")
        .replace("soft_rss_gib = 4.5", "soft_rss_gib = 0.0000002")
        .replace("pressure_rss_gib = 5.0", "pressure_rss_gib = 0.0000010")
        .replace("emergency_rss_gib = 5.5", "emergency_rss_gib = 0.0000020")
        .replace("hard_rss_gib = 6.0", "hard_rss_gib = 0.0000040")
        .replace("spill_buffers_gib = 0.25", "spill_buffers_gib = 0.0000001")
        .replace(
            "checkpoint_buffers_gib = 0.25",
            "checkpoint_buffers_gib = 0.0000001",
        )
        .replace("worker_arena_mib = 32", "worker_arena_mib = 0")
}

fn python_invocations() -> Vec<(&'static str, Vec<&'static str>)> {
    if cfg!(windows) {
        vec![("python", vec![]), ("py", vec!["-3"])]
    } else {
        vec![("python3", vec![]), ("python", vec![])]
    }
}

fn integration_process_lock() -> fs::File {
    let lock_path = std::env::temp_dir().join("pen-cli-integration.lock");
    let file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(&lock_path)
        .unwrap_or_else(|error| {
            panic!("failed to open {}: {error}", lock_path.display());
        });
    // Serialize full pen-cli subprocesses across integration-test threads and binaries.
    file.lock_exclusive().unwrap_or_else(|error| {
        panic!("failed to lock {}: {error}", lock_path.display());
    });
    file
}
