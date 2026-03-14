mod support;

use std::fs;

use support::{
    assert_success, compact_step_summaries, fixtures_root, load_trajectory_fixture, read_json,
    run_pen_cli, temp_dir, workspace_root,
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
        "2",
    ]);
    assert_success(initial);

    let resumed = run_pen_cli(["resume", &run_dir_arg, "--until-step", "4"]);
    let resumed_stdout = assert_success(resumed);
    assert!(resumed_stdout.contains("completed_step: 4"));

    let inspect = run_pen_cli(["inspect", &run_dir_arg]);
    let inspect_stdout = assert_success(inspect);
    assert!(inspect_stdout.contains("latest: step 4 (Pi)"));
    assert!(inspect_stdout.contains("step 04 Pi"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_4.json");
    assert_eq!(actual_steps, expected_steps);

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

    fs::remove_dir_all(root).ok();
}
