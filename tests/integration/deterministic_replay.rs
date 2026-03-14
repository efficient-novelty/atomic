mod support;

use std::fs;

use support::{
    assert_success, compact_step_summaries, load_trajectory_fixture, read_text, run_pen_cli,
    temp_dir, workspace_root,
};

#[test]
fn repeated_runs_with_the_same_inputs_are_deterministic() {
    let root_a = temp_dir("deterministic-a");
    let root_b = temp_dir("deterministic-b");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();

    for root in [&root_a, &root_b] {
        let root_arg = root.to_string_lossy().to_string();
        let output = run_pen_cli([
            "run",
            "--config",
            &config,
            "--root",
            &root_arg,
            "--run-id",
            "deterministic-run",
            "--until-step",
            "5",
        ]);
        assert_success(output);
    }

    let run_dir_a = root_a.join("deterministic-run");
    let run_dir_b = root_b.join("deterministic-run");
    let steps_a = compact_step_summaries(&run_dir_a);
    let steps_b = compact_step_summaries(&run_dir_b);
    let expected_steps = load_trajectory_fixture("reference_steps_until_5.json");

    assert_eq!(steps_a, steps_b);
    assert_eq!(steps_a, expected_steps);

    let debug_a = read_text(&run_dir_a.join("reports").join("latest.debug.txt"));
    let debug_b = read_text(&run_dir_b.join("reports").join("latest.debug.txt"));
    assert_eq!(debug_a, debug_b);

    fs::remove_dir_all(root_a).ok();
    fs::remove_dir_all(root_b).ok();
}
