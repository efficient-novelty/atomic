mod support;

use std::fs;

use support::{
    assert_success, compact_step_summaries, fixtures_root, load_trajectory_fixture,
    normalize_checkpoint, read_json, read_text, run_pen_cli, temp_dir, workspace_root,
    write_pressure_config,
};

#[test]
fn bootstrap_run_writes_frozen_reference_artifacts() {
    let root = temp_dir("bootstrap");
    let run_dir = root.join("fixture-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let output = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "fixture-run",
        "--until-step",
        "3",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 3"));
    assert!(run_dir.join("telemetry.ndjson").exists());

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_4.json");
    assert_eq!(actual_steps, expected_steps[..3].to_vec());

    let actual_checkpoint = normalize_checkpoint(read_json(
        &run_dir
            .join("checkpoints")
            .join("steps")
            .join("step-03.json"),
    ));
    let expected_checkpoint = read_json(
        &fixtures_root()
            .join("checkpoints")
            .join("reference_step_03.json"),
    );
    assert_eq!(actual_checkpoint, expected_checkpoint);

    fs::remove_dir_all(root).ok();
}

#[test]
fn bootstrap_run_uses_live_search_through_step_six() {
    let root = temp_dir("bootstrap-step6");
    let run_dir = root.join("fixture-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let output = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "fixture-run",
        "--until-step",
        "6",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 6"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_6.json");
    assert_eq!(actual_steps, expected_steps);

    fs::remove_dir_all(root).ok();
}

#[test]
fn bootstrap_run_uses_live_search_through_step_seven() {
    let root = temp_dir("bootstrap-step7");
    let run_dir = root.join("fixture-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let output = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "fixture-run",
        "--until-step",
        "7",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 7"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_7.json");
    assert_eq!(actual_steps, expected_steps);

    fs::remove_dir_all(root).ok();
}

#[test]
fn bootstrap_run_uses_live_search_through_step_eight() {
    let root = temp_dir("bootstrap-step8");
    let run_dir = root.join("fixture-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let output = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "fixture-run",
        "--until-step",
        "8",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 8"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_8.json");
    assert_eq!(actual_steps, expected_steps);

    fs::remove_dir_all(root).ok();
}

#[test]
fn bootstrap_run_uses_live_search_through_step_nine() {
    let root = temp_dir("bootstrap-step9");
    let run_dir = root.join("fixture-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let output = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "fixture-run",
        "--until-step",
        "9",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 9"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_9.json");
    assert_eq!(actual_steps, expected_steps);

    fs::remove_dir_all(root).ok();
}

#[test]
fn bootstrap_run_uses_live_search_through_step_ten() {
    let root = temp_dir("bootstrap-step10");
    let run_dir = root.join("fixture-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let output = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "fixture-run",
        "--until-step",
        "10",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 10"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_10.json");
    assert_eq!(actual_steps, expected_steps);

    fs::remove_dir_all(root).ok();
}

#[test]
fn bootstrap_run_uses_live_search_through_step_eleven() {
    let root = temp_dir("bootstrap-step11");
    let run_dir = root.join("fixture-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let output = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "fixture-run",
        "--until-step",
        "11",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 11"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_11.json");
    assert_eq!(actual_steps, expected_steps);

    fs::remove_dir_all(root).ok();
}

#[test]
fn bootstrap_run_uses_live_search_through_step_twelve() {
    let root = temp_dir("bootstrap-step12");
    let run_dir = root.join("fixture-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let output = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "fixture-run",
        "--until-step",
        "12",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 12"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_12.json");
    assert_eq!(actual_steps, expected_steps);

    fs::remove_dir_all(root).ok();
}

#[test]
fn bootstrap_run_uses_live_search_through_step_thirteen() {
    let root = temp_dir("bootstrap-step13");
    let run_dir = root.join("fixture-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let output = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "fixture-run",
        "--until-step",
        "13",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 13"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_13.json");
    assert_eq!(actual_steps, expected_steps);

    fs::remove_dir_all(root).ok();
}

#[test]
fn bootstrap_run_uses_live_search_through_step_fourteen() {
    let root = temp_dir("bootstrap-step14");
    let run_dir = root.join("fixture-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let output = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "fixture-run",
        "--until-step",
        "14",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 14"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_14.json");
    assert_eq!(actual_steps, expected_steps);

    fs::remove_dir_all(root).ok();
}

#[test]
fn bootstrap_run_uses_live_search_through_step_fifteen() {
    let root = temp_dir("bootstrap-step15");
    let run_dir = root.join("fixture-run");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let output = run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "fixture-run",
        "--until-step",
        "15",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 15"));
    assert!(stdout.contains("replay_ablation: matches_reference_replay x15"));
    assert!(stdout.contains("late_step_claim: executable_canon step 15 DCT nu=103"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_15.json");
    assert_eq!(actual_steps, expected_steps);
    assert!(
        run_dir
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("frontier.manifest.json")
            .exists()
    );
    assert!(
        run_dir
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("hot-000.bin")
            .exists()
    );
    assert!(
        run_dir
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("frontier-runtime.json")
            .exists()
    );
    assert!(
        run_dir
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("dedupe-000.txt")
            .exists()
    );
    assert!(
        run_dir
            .join("meta.sqlite3")
            .metadata()
            .expect("metadata db")
            .len()
            > 0
    );

    let step15_summary = read_json(
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-15-summary.json"),
    );
    assert_eq!(
        step15_summary["replay_ablation"]["status"].as_str(),
        Some("matches_reference_replay")
    );

    fs::remove_dir_all(root).ok();
}

#[test]
fn bootstrap_run_surfaces_pressure_hardened_spill_backed_retention() {
    let root = temp_dir("bootstrap-pressure");
    let run_dir = root.join("pressure-run");
    let config_path = root.join("pressure.toml");
    let root_arg = root.to_string_lossy().to_string();
    let config_arg = config_path.to_string_lossy().to_string();

    write_pressure_config(&config_path);

    let stdout = assert_success(run_pen_cli([
        "run",
        "--config",
        &config_arg,
        "--root",
        &root_arg,
        "--run-id",
        "pressure-run",
        "--until-step",
        "15",
    ]));

    assert!(stdout.contains("completed_step: 15"));
    let debug = read_text(&run_dir.join("reports").join("latest.debug.txt"));
    assert!(debug.contains("frontier pressure: state=orange action=spill_cold"));
    assert!(debug.contains("retention policy: focus=temporal"));
    assert!(debug.contains("replay ablation: matches_reference_replay x15"));

    let step15_summary = read_json(
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-15-summary.json"),
    );
    assert_eq!(
        step15_summary["frontier_pressure"]["pressure_action"].as_str(),
        Some("spill_cold")
    );
    assert_eq!(
        step15_summary["frontier_pressure"]["governor_state"].as_str(),
        Some("orange")
    );
    assert_eq!(
        step15_summary["replay_ablation"]["status"].as_str(),
        Some("matches_reference_replay")
    );

    let frontier_stdout = assert_success(run_pen_cli([
        "inspect",
        &run_dir
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("frontier.manifest.json")
            .to_string_lossy(),
    ]));
    assert!(frontier_stdout.contains("resident_cold_records:"));
    assert!(frontier_stdout.contains("spilled_cold_records:"));

    let step_stdout = assert_success(run_pen_cli([
        "inspect",
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-15-summary.json")
            .to_string_lossy(),
    ]));
    assert!(step_stdout.contains("replay_ablation: matches_reference_replay"));

    fs::remove_dir_all(root).ok();
}
