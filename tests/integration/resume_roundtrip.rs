mod support;

use std::fs;
use std::path::Path;

use serde_json::Value;
use support::{
    assert_success, compact_step_summaries, fixtures_root, latest_frontier_manifest_path,
    load_trajectory_fixture, mutate_latest_frontier_manifest, read_json, read_text,
    rewrite_config_workers, run_pen_cli, temp_dir, workspace_root,
};

#[test]
fn resume_roundtrip_keeps_the_reference_sequence_and_inspect_output() {
    let root = temp_dir("resume");
    let run_dir = root.join("resume-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let run_dir_arg = run_dir.to_string_lossy().to_string();

    let initial = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "resume-run",
        "--until-step",
        "12",
    ]);
    assert_success(initial);

    rewrite_config_workers(&run_dir.join("config.toml"), 5);
    let stored_frontier_worker_count =
        latest_frontier_manifest(&run_dir)["scheduler"]["worker_count"]
            .as_u64()
            .expect("frontier worker_count");

    let resumed = run_pen_cli(["resume", &run_dir_arg, "--until-step", "15"]);
    let resumed_stdout = assert_success(resumed);
    assert!(resumed_stdout.contains("completed_step: 15"));

    let inspect = run_pen_cli(["inspect", &run_dir_arg]);
    let inspect_stdout = assert_success(inspect);
    assert!(inspect_stdout.contains("search_profile: strict_canon_guarded"));
    assert!(inspect_stdout.contains("latest: step 15 (DCT)"));
    assert!(inspect_stdout.contains("provenance: frontier_checkpoint_resume"));
    assert!(inspect_stdout.contains("step 15 DCT"));
    assert!(inspect_stdout.contains("late_step_claim: executable_canon step 15 DCT nu=103"));
    assert!(inspect_stdout.contains("replay ablation: matches_reference_replay x15"));
    assert!(inspect_stdout.contains("charged_kappa: 8"));
    assert!(inspect_stdout.contains("frontier pressure: state=green action=none"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_15.json");
    assert_eq!(actual_steps, expected_steps);

    let step15_summary = read_json(
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-15-summary.json"),
    );
    assert_eq!(
        step15_summary["canon_evidence"]["late_step_claim"]["matches_accepted"].as_bool(),
        Some(true)
    );
    assert_eq!(
        step15_summary["replay_ablation"]["status"].as_str(),
        Some("matches_reference_replay")
    );

    let actual_library = read_json(
        &run_dir
            .join("checkpoints")
            .join("steps")
            .join("step-04.json"),
    )
    .get("library_snapshot")
    .cloned()
    .expect("library snapshot");
    let expected_library = read_json(
        &fixtures_root()
            .join("libraries")
            .join("reference_window_step_04.json"),
    );
    assert_eq!(actual_library, expected_library);

    let telemetry = read_text(&run_dir.join("telemetry.ndjson"));
    assert!(telemetry.contains("\"mode\":\"frontier_checkpoint_resume\""));
    assert!(telemetry.contains("\"search_profile\":\"strict_canon_guarded\""));
    assert!(telemetry.contains("\"replay_ablation\""));

    let frontier_inspect = run_pen_cli([
        "inspect",
        &run_dir
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("frontier.manifest.json")
            .to_string_lossy(),
    ]);
    let frontier_stdout = assert_success(frontier_inspect);
    assert!(frontier_stdout.contains("frontier step 15 band 8"));
    assert!(frontier_stdout.contains("resume_decision: FrontierCheckpoint"));
    assert!(frontier_stdout.contains("governor_state:"));
    assert!(frontier_stdout.contains("resident_cold_records:"));
    assert!(frontier_stdout.contains("cache_blob: frontier-runtime.json"));
    assert_eq!(
        latest_frontier_manifest(&run_dir)["scheduler"]["worker_count"].as_u64(),
        Some(stored_frontier_worker_count)
    );

    fs::remove_dir_all(root).ok();
}

#[test]
fn resume_roundtrip_falls_back_to_step_checkpoint_when_frontier_search_changes() {
    let root = temp_dir("resume-search-change");
    let run_dir = root.join("resume-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let run_dir_arg = run_dir.to_string_lossy().to_string();

    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "resume-run",
        "--until-step",
        "12",
    ]));

    mutate_latest_frontier_manifest(&run_dir, |manifest| {
        manifest["resume_compatible"]["search_semantics_hash"] =
            Value::String("blake3:other-search".to_owned());
    });

    assert_success(run_pen_cli(["resume", &run_dir_arg, "--until-step", "15"]));
    let telemetry = read_text(&run_dir.join("telemetry.ndjson"));
    assert!(telemetry.contains("\"mode\":\"step_checkpoint_resume\""));

    fs::remove_dir_all(root).ok();
}

#[test]
fn resume_roundtrip_reevaluates_when_frontier_evaluator_changes() {
    let root = temp_dir("resume-eval-change");
    let run_dir = root.join("resume-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let run_dir_arg = run_dir.to_string_lossy().to_string();

    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "resume-run",
        "--until-step",
        "12",
    ]));

    mutate_latest_frontier_manifest(&run_dir, |manifest| {
        manifest["resume_compatible"]["evaluator_hash"] =
            Value::String("blake3:other-eval".to_owned());
    });

    assert_success(run_pen_cli(["resume", &run_dir_arg, "--until-step", "15"]));
    let telemetry = read_text(&run_dir.join("telemetry.ndjson"));
    assert!(telemetry.contains("\"mode\":\"step_checkpoint_reevaluate\""));

    fs::remove_dir_all(root).ok();
}

#[test]
fn realistic_resume_roundtrip_keeps_the_reference_sequence_and_prefix_frontier_evidence() {
    let root = temp_dir("realistic-resume");
    let run_dir = root.join("resume-run");
    let config = workspace_root()
        .join("configs")
        .join("realistic_frontier_shadow.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let run_dir_arg = run_dir.to_string_lossy().to_string();

    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "resume-run",
        "--until-step",
        "12",
    ]));

    let resumed = run_pen_cli(["resume", &run_dir_arg, "--until-step", "15"]);
    let resumed_stdout = assert_success(resumed);
    assert!(resumed_stdout.contains("completed_step: 15"));
    assert!(resumed_stdout.contains("search_profile: realistic_frontier_shadow"));

    let inspect_stdout = assert_success(run_pen_cli(["inspect", &run_dir_arg]));
    assert!(inspect_stdout.contains("search_profile: realistic_frontier_shadow"));
    assert!(inspect_stdout.contains("provenance: frontier_checkpoint_resume"));
    assert!(inspect_stdout.contains("late_step_claim: executable_canon step 15 DCT nu=103"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_15.json");
    assert_eq!(actual_steps, expected_steps);

    let step15_summary = read_json(
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-15-summary.json"),
    );
    assert_eq!(
        step15_summary["search_profile"].as_str(),
        Some("realistic_frontier_shadow")
    );
    assert_eq!(
        step15_summary["provenance"].as_str(),
        Some("frontier_checkpoint_resume")
    );
    assert!(
        step15_summary["search_stats"]["prefix_states_explored"]
            .as_u64()
            .expect("prefix_states_explored")
            > 0
    );
    let step_inspect_stdout = assert_success(run_pen_cli([
        "inspect",
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-15-summary.json")
            .to_string_lossy(),
    ]));
    assert!(step_inspect_stdout.contains("prefix_frontier: created="));

    let telemetry = read_text(&run_dir.join("telemetry.ndjson"));
    assert!(telemetry.contains("\"mode\":\"frontier_checkpoint_resume\""));
    assert!(telemetry.contains("\"search_profile\":\"realistic_frontier_shadow\""));
    assert!(telemetry.contains("\"search_timing\""));

    fs::remove_dir_all(root).ok();
}

#[test]
fn realistic_resume_roundtrip_falls_back_to_step_checkpoint_when_frontier_search_changes() {
    let root = temp_dir("realistic-resume-search-change");
    let run_dir = root.join("resume-run");
    let config = workspace_root()
        .join("configs")
        .join("realistic_frontier_shadow.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let run_dir_arg = run_dir.to_string_lossy().to_string();

    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "resume-run",
        "--until-step",
        "12",
    ]));

    mutate_latest_frontier_manifest(&run_dir, |manifest| {
        manifest["resume_compatible"]["search_semantics_hash"] =
            Value::String("blake3:other-realistic-search".to_owned());
    });

    assert_success(run_pen_cli(["resume", &run_dir_arg, "--until-step", "15"]));
    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_15.json");
    assert_eq!(actual_steps, expected_steps);

    let telemetry = read_text(&run_dir.join("telemetry.ndjson"));
    assert!(telemetry.contains("\"mode\":\"step_checkpoint_resume\""));
    assert!(telemetry.contains("\"search_profile\":\"realistic_frontier_shadow\""));

    fs::remove_dir_all(root).ok();
}

#[test]
fn realistic_resume_roundtrip_reevaluates_when_frontier_evaluator_changes() {
    let root = temp_dir("realistic-resume-eval-change");
    let run_dir = root.join("resume-run");
    let config = workspace_root()
        .join("configs")
        .join("realistic_frontier_shadow.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let run_dir_arg = run_dir.to_string_lossy().to_string();

    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "resume-run",
        "--until-step",
        "12",
    ]));

    mutate_latest_frontier_manifest(&run_dir, |manifest| {
        manifest["resume_compatible"]["evaluator_hash"] =
            Value::String("blake3:other-realistic-eval".to_owned());
    });

    assert_success(run_pen_cli(["resume", &run_dir_arg, "--until-step", "15"]));
    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_15.json");
    assert_eq!(actual_steps, expected_steps);

    let telemetry = read_text(&run_dir.join("telemetry.ndjson"));
    assert!(telemetry.contains("\"mode\":\"step_checkpoint_reevaluate\""));
    assert!(telemetry.contains("\"search_profile\":\"realistic_frontier_shadow\""));

    fs::remove_dir_all(root).ok();
}

#[test]
fn resume_roundtrip_requires_migration_when_frontier_ast_changes() {
    let root = temp_dir("resume-ast-change");
    let run_dir = root.join("resume-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let run_dir_arg = run_dir.to_string_lossy().to_string();

    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "resume-run",
        "--until-step",
        "12",
    ]));

    mutate_latest_frontier_manifest(&run_dir, |manifest| {
        manifest["resume_compatible"]["ast_schema_hash"] =
            Value::String("blake3:other-ast".to_owned());
    });

    let failed = run_pen_cli(["resume", &run_dir_arg, "--until-step", "15"]);
    let stdout = String::from_utf8_lossy(&failed.stdout);
    let stderr = String::from_utf8_lossy(&failed.stderr);
    assert!(!failed.status.success());
    assert!(
        format!("{stdout}{stderr}").contains("migration required"),
        "expected migration failure\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );

    fs::remove_dir_all(root).ok();
}
fn latest_frontier_manifest(run_dir: &Path) -> Value {
    read_json(&latest_frontier_manifest_path(run_dir))
}
