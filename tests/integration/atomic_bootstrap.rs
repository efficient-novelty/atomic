mod support;

use std::fs;

use support::{
    assert_success, compact_step_summaries, fixtures_root, load_trajectory_fixture,
    normalize_checkpoint, read_json, run_pen_cli, temp_dir, workspace_root,
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
