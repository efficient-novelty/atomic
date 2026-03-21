mod support;

use std::fs;

use support::{
    assert_success, compact_search_space_stats, compact_step_summaries, fixtures_root,
    load_search_space_fixture, load_trajectory_fixture, mutate_latest_frontier_manifest,
    normalize_checkpoint, read_json, read_text, run_claim_certify, run_compare_runs, run_pen_cli,
    temp_dir, workspace_root, write_pressure_config,
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
    assert!(stdout.contains("search_profile: strict_canon_guarded"));
    assert!(stdout.contains("replay_ablation: matches_reference_replay x15"));
    assert!(stdout.contains("late_step_claim: executable_canon step 15 DCT nu=103"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_15.json");
    assert_eq!(actual_steps, expected_steps);
    let actual_search_space = compact_search_space_stats(&run_dir);
    let expected_search_space =
        load_search_space_fixture("reference_search_space_strict_canon_guarded_until_15.json");
    assert_eq!(actual_search_space, expected_search_space);
    let latest = read_text(&run_dir.join("reports").join("latest.txt"));
    let debug = read_text(&run_dir.join("reports").join("latest.debug.txt"));
    assert!(latest.contains("search_profile: strict_canon_guarded"));
    assert!(debug.contains("search_profile: strict_canon_guarded"));
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
    assert_eq!(
        step15_summary["search_profile"].as_str(),
        Some("strict_canon_guarded")
    );

    fs::remove_dir_all(root).ok();
}

#[test]
fn relaxed_shadow_run_preserves_reference_sequence_and_exposes_late_competition() {
    let root = temp_dir("relaxed-shadow");
    let run_dir = root.join("relaxed-shadow");
    let output = run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("relaxed_shadow.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "relaxed-shadow",
        "--until-step",
        "12",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 12"));
    assert!(stdout.contains("search_profile: relaxed_shadow"));
    assert!(stdout.contains("replay_ablation: matches_reference_replay x12"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_12.json");
    assert_eq!(actual_steps, expected_steps);

    let latest = read_text(&run_dir.join("reports").join("latest.txt"));
    let debug = read_text(&run_dir.join("reports").join("latest.debug.txt"));
    assert!(latest.contains("search_profile: relaxed_shadow"));
    assert!(debug.contains("search_profile: relaxed_shadow"));

    let step10 = read_json(
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-10-summary.json"),
    );
    let step11 = read_json(
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-11-summary.json"),
    );
    let step12 = read_json(
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-12-summary.json"),
    );

    assert_eq!(
        step10["search_stats"]["enumerated_candidates"].as_u64(),
        Some(7)
    );
    assert_eq!(
        step10["search_stats"]["admissibility_rejections"].as_u64(),
        Some(5)
    );
    assert_eq!(
        step10["search_stats"]["evaluated_candidates"].as_u64(),
        Some(2)
    );
    assert_eq!(
        step10["search_stats"]["admissibility_diagnostics"]["structural_debt_cap_rejections"]
            .as_u64(),
        Some(5)
    );
    assert_eq!(
        step10["search_stats"]["admissibility_diagnostics"]["admitted_deprioritized"].as_u64(),
        Some(1)
    );

    assert_eq!(
        step11["search_stats"]["enumerated_candidates"].as_u64(),
        Some(24)
    );
    assert_eq!(
        step11["search_stats"]["admissibility_rejections"].as_u64(),
        Some(22)
    );
    assert_eq!(
        step11["search_stats"]["evaluated_candidates"].as_u64(),
        Some(2)
    );
    assert_eq!(
        step11["search_stats"]["scored_candidate_distribution"]["clause_kappa_counts"]["5"]
            .as_u64(),
        Some(1)
    );
    assert_eq!(
        step11["search_stats"]["scored_candidate_distribution"]["clause_kappa_counts"]["6"]
            .as_u64(),
        Some(1)
    );

    assert_eq!(
        step12["search_stats"]["enumerated_candidates"].as_u64(),
        Some(24)
    );
    assert_eq!(
        step12["search_stats"]["admissibility_rejections"].as_u64(),
        Some(22)
    );
    assert_eq!(
        step12["search_stats"]["evaluated_candidates"].as_u64(),
        Some(2)
    );
    assert_eq!(
        step12["search_stats"]["scored_candidate_distribution"]["clears_bar"].as_u64(),
        Some(2)
    );

    fs::remove_dir_all(root).ok();
}

#[test]
fn realistic_shadow_run_preserves_reference_sequence_and_exposes_full_late_competition() {
    let root = temp_dir("realistic-shadow");
    let run_dir = root.join("realistic-shadow");
    let output = run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("realistic_frontier_shadow.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "realistic-shadow",
        "--until-step",
        "15",
    ]);
    let stdout = assert_success(output);

    assert!(stdout.contains("completed_step: 15"));
    assert!(stdout.contains("search_profile: realistic_frontier_shadow"));
    assert!(stdout.contains("replay_ablation: matches_reference_replay x15"));

    let actual_steps = compact_step_summaries(&run_dir);
    let expected_steps = load_trajectory_fixture("reference_steps_until_15.json");
    assert_eq!(actual_steps, expected_steps);

    let latest = read_text(&run_dir.join("reports").join("latest.txt"));
    let debug = read_text(&run_dir.join("reports").join("latest.debug.txt"));
    assert!(latest.contains("search_profile: realistic_frontier_shadow"));
    assert!(debug.contains("search_profile: realistic_frontier_shadow"));
    assert!(debug.contains("prefix frontier: created="));

    for step in 10..=15 {
        let summary = read_json(
            &run_dir
                .join("reports")
                .join("steps")
                .join(format!("step-{step:02}-summary.json")),
        );
        let evaluated = summary["search_stats"]["evaluated_candidates"]
            .as_u64()
            .expect("evaluated_candidates");
        let enumerated = summary["search_stats"]["enumerated_candidates"]
            .as_u64()
            .expect("enumerated_candidates");
        let prefix_explored = summary["search_stats"]["prefix_states_explored"]
            .as_u64()
            .expect("prefix_states_explored");
        let prefixes_created = summary["search_stats"]["prefixes_created"]
            .as_u64()
            .expect("prefixes_created");
        let prefix_merged = summary["search_stats"]["prefix_states_merged_by_signature"]
            .as_u64()
            .expect("prefix_states_merged_by_signature");
        let prefix_exact_pruned = summary["search_stats"]["prefix_states_exact_pruned"]
            .as_u64()
            .expect("prefix_states_exact_pruned");
        let terminal_rank_prunes = summary["search_stats"]["incremental_terminal_rank_prunes"]
            .as_u64()
            .expect("incremental_terminal_rank_prunes");
        let prefix_heuristic_dropped = summary["search_stats"]["prefix_states_heuristic_dropped"]
            .as_u64()
            .expect("prefix_states_heuristic_dropped");
        let prefix_hot = summary["search_stats"]["prefix_frontier_hot_states"]
            .as_u64()
            .expect("prefix_frontier_hot_states");
        let prefix_cold = summary["search_stats"]["prefix_frontier_cold_states"]
            .as_u64()
            .expect("prefix_frontier_cold_states");
        assert!(
            evaluated > 1 || prefix_exact_pruned > 0 || terminal_rank_prunes > 0,
            "expected realistic shadow step {step} to keep competition or exact-prune a competing late branch, got evaluated={evaluated} exact_pruned={prefix_exact_pruned} terminal_rank_prunes={terminal_rank_prunes}",
        );
        assert!(
            prefix_explored > 0,
            "expected realistic shadow step {step} to explore at least one prefix state",
        );
        assert!(
            prefix_merged <= enumerated,
            "expected merged-prefix accounting to stay bounded by the enumerated pool at step {step}, got merged={prefix_merged} enumerated={enumerated}",
        );
        assert!(
            prefix_exact_pruned + prefix_heuristic_dropped + prefix_hot + prefix_cold
                <= prefixes_created,
            "expected realistic shadow step {step} retained/pruned prefix accounting to stay within created prefixes",
        );
    }

    let step13 = read_json(
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-13-summary.json"),
    );
    let step14 = read_json(
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-14-summary.json"),
    );
    let step15 = read_json(
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-15-summary.json"),
    );

    assert_eq!(
        step13["search_stats"]["enumerated_candidates"].as_u64(),
        Some(2)
    );
    assert_eq!(
        step14["search_stats"]["enumerated_candidates"].as_u64(),
        Some(2)
    );
    assert_eq!(
        step15["search_stats"]["enumerated_candidates"].as_u64(),
        Some(2)
    );
    let frontier = read_json(
        &run_dir
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("frontier.manifest.json"),
    );
    assert_eq!(
        frontier["counts"]["prefix_states_explored"].as_u64(),
        step15["search_stats"]["prefix_states_explored"].as_u64()
    );
    assert_eq!(
        frontier["counts"]["prefix_states_merged_by_signature"].as_u64(),
        step15["search_stats"]["prefix_states_merged_by_signature"].as_u64()
    );
    assert_eq!(
        frontier["counts"]["prefix_states_exact_pruned"].as_u64(),
        step15["search_stats"]["prefix_states_exact_pruned"].as_u64()
    );
    assert_eq!(
        frontier["counts"]["prefix_states_heuristic_dropped"].as_u64(),
        step15["search_stats"]["prefix_states_heuristic_dropped"].as_u64()
    );
    let frontier_inspect = assert_success(run_pen_cli([
        "inspect",
        &run_dir
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("frontier.manifest.json")
            .to_string_lossy(),
    ]));
    assert!(frontier_inspect.contains("prefix_explored:"));
    assert!(frontier_inspect.contains("prefix_merged:"));
    assert!(frontier_inspect.contains("prefix_exact_pruned:"));
    assert!(frontier_inspect.contains("prefix_heuristic_dropped:"));
    let step_inspect = assert_success(run_pen_cli([
        "inspect",
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-15-summary.json")
            .to_string_lossy(),
    ]));
    assert!(step_inspect.contains("prefix_frontier: created="));
    assert!(step_inspect.contains(" merged="));

    fs::remove_dir_all(root).ok();
}

#[test]
fn compare_runs_reports_relaxed_shadow_admissibility_and_competition_deltas() {
    let root = temp_dir("relaxed-shadow-compare");
    let guarded_dir = root.join("guarded");
    let relaxed_dir = root.join("relaxed");

    assert_success(run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("strict_canon_guarded.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "guarded",
        "--until-step",
        "12",
    ]));
    assert_success(run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("relaxed_shadow.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "relaxed",
        "--until-step",
        "12",
    ]));

    let text_out = root.join("compare.txt");
    let json_out = root.join("compare.json");
    let stdout = assert_success(run_compare_runs([
        "--baseline",
        "guarded",
        "--lane",
        &format!("guarded={}", guarded_dir.to_string_lossy()),
        "--lane",
        &format!("relaxed={}", relaxed_dir.to_string_lossy()),
        "--text-out",
        &text_out.to_string_lossy(),
        "--json-out",
        &json_out.to_string_lossy(),
    ]));

    assert!(stdout.contains("accepted hashes: all 2 lanes match baseline guarded"));
    assert!(stdout.contains("search-space counts: mismatches detected in relaxed"));
    assert!(stdout.contains("admissibility diagnostics: mismatches detected in relaxed"));
    assert!(stdout.contains("late-step competition: mismatches detected in relaxed"));

    let summary = read_json(&json_out);
    assert_eq!(
        summary["trajectory_consistency"]["status"].as_str(),
        Some("all_match_baseline")
    );
    assert_eq!(
        summary["accepted_hash_consistency"]["status"].as_str(),
        Some("all_match_baseline")
    );
    assert_eq!(
        summary["search_space_count_consistency"]["status"].as_str(),
        Some("deltas_present")
    );
    assert_eq!(
        summary["admissibility_diagnostics_consistency"]["status"].as_str(),
        Some("deltas_present")
    );
    assert_eq!(
        summary["late_step_competition_consistency"]["status"].as_str(),
        Some("deltas_present")
    );

    let relaxed_lane = summary["lanes"]
        .as_array()
        .expect("lanes")
        .iter()
        .find(|lane| lane["label"].as_str() == Some("relaxed"))
        .expect("relaxed lane");
    assert_eq!(
        relaxed_lane["search_profile"].as_str(),
        Some("relaxed_shadow")
    );
    assert_eq!(
        relaxed_lane["accepted_hashes_vs_baseline"]["matches"].as_bool(),
        Some(true)
    );
    assert_eq!(
        delta_step_indexes(&relaxed_lane["search_space_counts_vs_baseline"]["deltas"]),
        vec![10, 11, 12]
    );
    assert_eq!(
        delta_step_indexes(&relaxed_lane["admissibility_diagnostics_vs_baseline"]["deltas"]),
        vec![10, 11, 12]
    );
    assert_eq!(
        delta_step_indexes(&relaxed_lane["late_step_competition_vs_baseline"]["deltas"]),
        vec![10, 11, 12]
    );

    fs::remove_dir_all(root).ok();
}

#[test]
fn compare_runs_reports_demo_phase_floor_and_closure_evidence() {
    let root = temp_dir("demo-compare");
    let guarded_dir = root.join("guarded");
    let demo_dir = root.join("demo");

    assert_success(run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("strict_canon_guarded.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "guarded",
        "--until-step",
        "2",
    ]));
    assert_success(run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("demo_breadth_shadow_10m.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "demo",
        "--until-step",
        "2",
    ]));

    let text_out = root.join("compare.txt");
    let json_out = root.join("compare.json");
    let stdout = assert_success(run_compare_runs([
        "--baseline",
        "guarded",
        "--lane",
        &format!("guarded={}", guarded_dir.to_string_lossy()),
        "--lane",
        &format!("demo={}", demo_dir.to_string_lossy()),
        "--text-out",
        &text_out.to_string_lossy(),
        "--json-out",
        &json_out.to_string_lossy(),
    ]));

    assert!(stdout.contains("Lane demo"));
    assert!(stdout.contains("demo phase latest:"));
    assert!(stdout.contains("demo funnel latest:"));
    assert!(stdout.contains("proof_close_closure="));

    let summary = read_json(&json_out);
    let demo_lane = summary["lanes"]
        .as_array()
        .expect("lanes")
        .iter()
        .find(|lane| lane["label"].as_str() == Some("demo"))
        .expect("demo lane");
    let demo_evidence = demo_lane["demo_phase_evidence"]
        .as_array()
        .expect("demo phase evidence");
    assert_eq!(
        demo_lane["search_profile"].as_str(),
        Some("demo_breadth_shadow")
    );
    assert_eq!(
        demo_evidence
            .first()
            .and_then(|step| step["generated_floor"].as_u64()),
        Some(2144)
    );
    assert_eq!(
        demo_evidence
            .first()
            .and_then(|step| step["generated_floor_status"].as_str()),
        Some("hit")
    );
    assert!(
        demo_evidence
            .last()
            .and_then(|step| step["closure_percent"].as_u64())
            .is_some()
    );
    assert!(
        demo_evidence
            .last()
            .and_then(|step| step["proof_close_reserved_millis"].as_u64())
            .is_some()
    );
    assert!(
        demo_evidence
            .last()
            .and_then(|step| step["proof_close_closure_percent"].as_u64())
            .is_some()
    );

    fs::remove_dir_all(root).ok();
}

#[test]
fn demo_run_and_resume_append_narrative_output_when_requested() {
    let root = temp_dir("demo-narrative-output");
    let run_dir = root.join("demo-narrative");

    let run_stdout = assert_success(run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("demo_breadth_shadow_10m.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "demo-narrative",
        "--until-step",
        "2",
        "--narrative",
    ]));

    assert!(run_stdout.contains("completed_step: 2"));
    assert!(run_stdout.contains("demo narrative"));
    assert!(run_stdout.contains("step 02 demo narrative"));
    assert!(run_stdout.contains("time         ["));
    assert!(run_stdout.contains("closure      ["));

    let resume_stdout = assert_success(run_pen_cli([
        "resume",
        &run_dir.to_string_lossy(),
        "--until-step",
        "3",
        "--narrative",
    ]));

    assert!(resume_stdout.contains("completed_step: 3"));
    assert!(resume_stdout.contains("step 03 demo narrative"));

    fs::remove_dir_all(root).ok();
}

#[test]
fn compare_runs_reports_missing_demo_narrative_artifacts_explicitly() {
    let root = temp_dir("demo-missing-narrative");
    let guarded_dir = root.join("guarded");
    let demo_dir = root.join("demo");

    assert_success(run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("strict_canon_guarded.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "guarded",
        "--until-step",
        "2",
    ]));
    assert_success(run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("demo_breadth_shadow_10m.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "demo",
        "--until-step",
        "2",
    ]));

    fs::remove_file(
        demo_dir
            .join("reports")
            .join("steps")
            .join("step-02-narrative.txt"),
    )
    .expect("remove narrative artifact");
    fs::remove_file(
        demo_dir
            .join("reports")
            .join("steps")
            .join("step-01-events.ndjson"),
    )
    .expect("remove events artifact");

    let json_out = root.join("compare-missing.json");
    let stdout = assert_success(run_compare_runs([
        "--baseline",
        "guarded",
        "--lane",
        &format!("guarded={}", guarded_dir.to_string_lossy()),
        "--lane",
        &format!("demo={}", demo_dir.to_string_lossy()),
        "--json-out",
        &json_out.to_string_lossy(),
    ]));

    assert!(stdout.contains("narrative artifacts: missing"));
    assert!(stdout.contains("text=step 2"));
    assert!(stdout.contains("events=step 1"));

    let summary = read_json(&json_out);
    let demo_lane = summary["lanes"]
        .as_array()
        .expect("lanes")
        .iter()
        .find(|lane| lane["label"].as_str() == Some("demo"))
        .expect("demo lane");
    assert_eq!(
        demo_lane["narrative_artifacts"]["status"].as_str(),
        Some("missing")
    );
    assert_eq!(
        demo_lane["narrative_artifacts"]["missing_narrative_steps"].as_array(),
        Some(&vec![serde_json::Value::from(2)])
    );
    assert_eq!(
        demo_lane["narrative_artifacts"]["missing_event_steps"].as_array(),
        Some(&vec![serde_json::Value::from(1)])
    );

    fs::remove_dir_all(root).ok();
}

#[test]
fn compare_runs_reports_claim_lane_policy_and_reason_audit() {
    let root = temp_dir("claim-compare");
    let guarded_dir = root.join("guarded");
    let claim_dir = root.join("claim");

    assert_success(run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("strict_canon_guarded.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "guarded",
        "--until-step",
        "2",
    ]));
    assert_success(run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("desktop_claim_shadow_smoke.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "claim",
        "--until-step",
        "2",
    ]));

    let json_out = root.join("claim-compare.json");
    let stdout = assert_success(run_compare_runs([
        "--baseline",
        "guarded",
        "--lane",
        &format!("guarded={}", guarded_dir.to_string_lossy()),
        "--lane",
        &format!("claim={}", claim_dir.to_string_lossy()),
        "--json-out",
        &json_out.to_string_lossy(),
    ]));

    assert!(stdout.contains("claim lane audit: attention (claim)"));
    assert!(stdout.contains(
        "search policy: honest guidance_style=claim_debt_guided late_expansion_policy=claim_generic bucket_policy=structural_generic"
    ));
    assert!(stdout.contains("exact-screen reasons: complete"));
    assert!(stdout.contains("prune class coverage: complete"));

    let summary = read_json(&json_out);
    assert_eq!(summary["signoff"]["status"].as_str(), Some("attention"));
    assert_eq!(
        summary["claim_lane_audit"]["status"].as_str(),
        Some("attention")
    );

    let claim_lane = summary["lanes"]
        .as_array()
        .expect("lanes")
        .iter()
        .find(|lane| lane["label"].as_str() == Some("claim"))
        .expect("claim lane");
    assert_eq!(
        claim_lane["claim_lane_audit"]["search_policy"]["status"].as_str(),
        Some("honest")
    );
    assert_eq!(
        claim_lane["claim_lane_audit"]["exact_screen_reasons"]["status"].as_str(),
        Some("complete")
    );
    assert_eq!(
        claim_lane["claim_lane_audit"]["prune_classes"]["status"].as_str(),
        Some("complete")
    );
    assert_eq!(
        claim_lane["claim_lane_audit"]["fallback_honesty"]["status"].as_str(),
        Some("clear")
    );
    assert!(
        claim_lane["claim_lane_audit"]["reasons"]
            .as_array()
            .expect("claim reasons")
            .iter()
            .any(|reason| reason.as_str() == Some("incomplete_step_coverage"))
    );

    fs::remove_dir_all(root).ok();
}

#[test]
fn claim_certification_script_emits_failing_certificate_for_incomplete_smoke_run() {
    let root = temp_dir("claim-certification");
    let guarded_dir = root.join("guarded");
    let claim_dir = root.join("claim");

    assert_success(run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("strict_canon_guarded.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "guarded",
        "--until-step",
        "2",
    ]));
    assert_success(run_pen_cli([
        "run",
        "--config",
        &workspace_root()
            .join("configs")
            .join("desktop_claim_shadow_smoke.toml")
            .to_string_lossy(),
        "--root",
        &root.to_string_lossy(),
        "--run-id",
        "claim",
        "--until-step",
        "2",
    ]));

    let json_out = root.join("claim-certificate.json");
    let text_out = root.join("claim-certificate.txt");
    let output = run_claim_certify([
        "--guarded-run",
        &guarded_dir.to_string_lossy(),
        "--claim-run",
        &claim_dir.to_string_lossy(),
        "--runtime-threshold-ms",
        "1000",
        "--json-out",
        &json_out.to_string_lossy(),
        "--text-out",
        &text_out.to_string_lossy(),
    ]);
    let stdout = String::from_utf8_lossy(&output.stdout).to_string()
        + &String::from_utf8_lossy(&output.stderr);

    assert!(
        !output.status.success(),
        "expected certification to fail for the incomplete smoke run\nstdout:\n{}",
        stdout
    );
    assert!(stdout.contains("Claim Certification: attention"));
    assert!(stdout.contains("accepted_hash_parity: fail"));
    assert!(stdout.contains("search_policy: pass"));
    assert!(stdout.contains("manifest_completeness: pass"));

    let certificate = read_json(&json_out);
    assert_eq!(certificate["status"].as_str(), Some("attention"));
    assert_eq!(
        certificate["checks"]["search_policy"]["status"].as_str(),
        Some("pass")
    );
    assert_eq!(
        certificate["checks"]["early_breadth"]["status"].as_str(),
        Some("fail")
    );
    assert_eq!(
        certificate["checks"]["late_generated_floors"]["status"].as_str(),
        Some("fail")
    );
    assert_eq!(
        certificate["checks"]["manifest_completeness"]["status"].as_str(),
        Some("pass")
    );
    assert_eq!(
        certificate["checks"]["runtime_threshold"]["status"].as_str(),
        Some("pass")
    );
    assert!(read_text(&text_out).contains("Claim Certification: attention"));

    fs::remove_dir_all(root).ok();
}

#[test]
fn compare_runs_reports_workstream4_rollout_parity_and_pressure_sets() {
    let root = temp_dir("workstream4-rollout");
    let root_arg = root.to_string_lossy().to_string();
    let guarded_config = workspace_root()
        .join("configs")
        .join("strict_canon_guarded.toml")
        .to_string_lossy()
        .to_string();
    let realistic_config = workspace_root()
        .join("configs")
        .join("realistic_frontier_shadow.toml")
        .to_string_lossy()
        .to_string();
    let guarded_dir = root.join("guarded");
    let realistic_dir = root.join("realistic");
    let realistic_frontier_resume_dir = root.join("realistic-frontier-resume");
    let realistic_step_resume_dir = root.join("realistic-step-resume");
    let realistic_reevaluate_dir = root.join("realistic-reevaluate");
    let realistic_pressure_dir = root.join("realistic-pressure");
    let realistic_pressure_config = root.join("realistic-pressure.toml");

    assert_success(run_pen_cli([
        "run",
        "--config",
        &guarded_config,
        "--root",
        &root_arg,
        "--run-id",
        "guarded",
        "--until-step",
        "15",
    ]));
    assert_success(run_pen_cli([
        "run",
        "--config",
        &realistic_config,
        "--root",
        &root_arg,
        "--run-id",
        "realistic",
        "--until-step",
        "15",
    ]));
    assert_success(run_pen_cli([
        "run",
        "--config",
        &realistic_config,
        "--root",
        &root_arg,
        "--run-id",
        "realistic-frontier-resume",
        "--until-step",
        "12",
    ]));
    assert_success(run_pen_cli([
        "resume",
        &realistic_frontier_resume_dir.to_string_lossy(),
        "--until-step",
        "15",
    ]));
    assert_success(run_pen_cli([
        "run",
        "--config",
        &realistic_config,
        "--root",
        &root_arg,
        "--run-id",
        "realistic-step-resume",
        "--until-step",
        "12",
    ]));
    mutate_latest_frontier_manifest(&realistic_step_resume_dir, |manifest| {
        manifest["resume_compatible"]["search_semantics_hash"] =
            serde_json::Value::String("blake3:realistic-search-change".to_owned());
    });
    assert_success(run_pen_cli([
        "resume",
        &realistic_step_resume_dir.to_string_lossy(),
        "--until-step",
        "15",
    ]));
    assert_success(run_pen_cli([
        "run",
        "--config",
        &realistic_config,
        "--root",
        &root_arg,
        "--run-id",
        "realistic-reevaluate",
        "--until-step",
        "12",
    ]));
    mutate_latest_frontier_manifest(&realistic_reevaluate_dir, |manifest| {
        manifest["resume_compatible"]["evaluator_hash"] =
            serde_json::Value::String("blake3:realistic-eval-change".to_owned());
    });
    assert_success(run_pen_cli([
        "resume",
        &realistic_reevaluate_dir.to_string_lossy(),
        "--until-step",
        "15",
    ]));

    write_realistic_pressure_config(&realistic_pressure_config);
    assert_success(run_pen_cli([
        "run",
        "--config",
        &realistic_pressure_config.to_string_lossy(),
        "--root",
        &root_arg,
        "--run-id",
        "realistic-pressure",
        "--until-step",
        "15",
    ]));

    let text_out = root.join("compare.txt");
    let json_out = root.join("compare.json");
    let stdout = assert_success(run_compare_runs([
        "--baseline",
        "guarded",
        "--lane",
        &format!("guarded={}", guarded_dir.to_string_lossy()),
        "--lane",
        &format!("realistic={}", realistic_dir.to_string_lossy()),
        "--lane",
        &format!(
            "realistic-frontier-resume={}",
            realistic_frontier_resume_dir.to_string_lossy()
        ),
        "--lane",
        &format!(
            "realistic-step-resume={}",
            realistic_step_resume_dir.to_string_lossy()
        ),
        "--lane",
        &format!(
            "realistic-reevaluate={}",
            realistic_reevaluate_dir.to_string_lossy()
        ),
        "--lane",
        &format!(
            "realistic-pressure={}",
            realistic_pressure_dir.to_string_lossy()
        ),
        "--text-out",
        &text_out.to_string_lossy(),
        "--json-out",
        &json_out.to_string_lossy(),
    ]));

    assert!(stdout.contains("workstream4 rollout: ready"));
    assert!(stdout.contains("workstream4 parity set: ready (realistic, realistic-pressure)"));
    assert!(
        stdout.contains(
            "workstream4 resume set: ready (realistic-frontier-resume, realistic-step-resume, realistic-reevaluate)"
        )
    );
    assert!(stdout.contains("workstream4 pressure set: ready (realistic-pressure)"));

    let summary = read_json(&json_out);
    let rollout = &summary["workstream4_rollout"];
    assert_eq!(rollout["status"].as_str(), Some("ready"));
    assert_eq!(rollout["parity_set"]["status"].as_str(), Some("ready"));
    assert_eq!(rollout["resume_set"]["status"].as_str(), Some("ready"));
    assert_eq!(rollout["pressure_set"]["status"].as_str(), Some("ready"));
    assert_eq!(
        summary["accepted_hash_consistency"]["status"].as_str(),
        Some("all_match_baseline")
    );

    let parity_ready = rollout["parity_set"]["ready_lanes"]
        .as_array()
        .expect("parity ready lanes")
        .iter()
        .map(|value| value.as_str().expect("lane label"))
        .collect::<Vec<_>>();
    assert_eq!(parity_ready, vec!["realistic", "realistic-pressure"]);

    let realistic_row = rollout["parity_set"]["lanes"]
        .as_array()
        .expect("parity rows")
        .iter()
        .find(|row| row["label"].as_str() == Some("realistic"))
        .expect("realistic parity row");
    assert_eq!(
        realistic_row["trajectory_matches_baseline"].as_bool(),
        Some(true)
    );
    assert_eq!(
        realistic_row["accepted_hashes_match_baseline"].as_bool(),
        Some(true)
    );
    assert_eq!(
        realistic_row["step15_claim_matches_baseline"].as_bool(),
        Some(true)
    );
    assert_eq!(
        realistic_row["late_step_competition_differs_from_baseline"].as_bool(),
        Some(true)
    );
    assert_eq!(
        realistic_row["prefix_frontier_present"].as_bool(),
        Some(true)
    );
    assert_eq!(realistic_row["pressure_exercised"].as_bool(), Some(false));
    assert_eq!(
        realistic_row["run_mode"].as_str(),
        Some("atomic_search_bootstrap")
    );

    let resume_modes = rollout["resume_set"]["present_modes"]
        .as_array()
        .expect("resume present modes")
        .iter()
        .map(|value| value.as_str().expect("resume mode"))
        .collect::<Vec<_>>();
    assert_eq!(
        resume_modes,
        vec![
            "frontier_checkpoint_resume",
            "step_checkpoint_reevaluate",
            "step_checkpoint_resume",
        ]
    );
    assert!(
        rollout["resume_set"]["missing_modes"]
            .as_array()
            .expect("resume missing modes")
            .is_empty()
    );
    let realistic_resume_row = rollout["resume_set"]["lanes"]
        .as_array()
        .expect("resume rows")
        .iter()
        .find(|row| row["label"].as_str() == Some("realistic-frontier-resume"))
        .expect("realistic frontier resume row");
    assert_eq!(realistic_resume_row["status"].as_str(), Some("ready"));
    assert_eq!(
        realistic_resume_row["run_mode"].as_str(),
        Some("frontier_checkpoint_resume")
    );
    let realistic_step_resume_row = rollout["resume_set"]["lanes"]
        .as_array()
        .expect("resume rows")
        .iter()
        .find(|row| row["label"].as_str() == Some("realistic-step-resume"))
        .expect("realistic step resume row");
    assert_eq!(
        realistic_step_resume_row["run_mode"].as_str(),
        Some("step_checkpoint_resume")
    );
    let realistic_reevaluate_row = rollout["resume_set"]["lanes"]
        .as_array()
        .expect("resume rows")
        .iter()
        .find(|row| row["label"].as_str() == Some("realistic-reevaluate"))
        .expect("realistic reevaluate row");
    assert_eq!(
        realistic_reevaluate_row["run_mode"].as_str(),
        Some("step_checkpoint_reevaluate")
    );

    let realistic_pressure_row = rollout["pressure_set"]["lanes"]
        .as_array()
        .expect("pressure rows")
        .iter()
        .find(|row| row["label"].as_str() == Some("realistic-pressure"))
        .expect("realistic pressure row");
    assert_eq!(realistic_pressure_row["status"].as_str(), Some("ready"));
    assert_eq!(
        realistic_pressure_row["pressure_exercised"].as_bool(),
        Some(true)
    );
    assert_ne!(
        realistic_pressure_row["latest_frontier_pressure_action"].as_str(),
        Some("none")
    );
    assert_ne!(
        realistic_pressure_row["latest_frontier_governor_state"].as_str(),
        Some("green")
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
    assert!(step_stdout.contains("search_profile: strict_canon_guarded"));
    assert!(step_stdout.contains("replay_ablation: matches_reference_replay"));

    fs::remove_dir_all(root).ok();
}

#[test]
fn realistic_shadow_pressure_run_surfaces_prefix_frontier_pressure_and_drop() {
    let root = temp_dir("realistic-pressure");
    let run_dir = root.join("realistic-pressure-run");
    let config_path = root.join("realistic-pressure.toml");
    let root_arg = root.to_string_lossy().to_string();
    let config_arg = config_path.to_string_lossy().to_string();

    write_realistic_pressure_config(&config_path);

    let stdout = assert_success(run_pen_cli([
        "run",
        "--config",
        &config_arg,
        "--root",
        &root_arg,
        "--run-id",
        "realistic-pressure-run",
        "--until-step",
        "15",
    ]));
    assert!(stdout.contains("search_profile: realistic_frontier_shadow"));

    let step4_summary = read_json(
        &run_dir
            .join("reports")
            .join("steps")
            .join("step-04-summary.json"),
    );
    assert_eq!(
        step4_summary["frontier_pressure"]["pressure_action"].as_str(),
        Some("spill_cold")
    );
    assert!(
        step4_summary["frontier_pressure"]["rss_bytes"]
            .as_u64()
            .expect("frontier pressure rss bytes")
            > 0
    );
    assert!(
        step4_summary["frontier_pressure"]["hot_frontier_bytes"]
            .as_u64()
            .expect("frontier pressure hot bytes")
            > 0
    );
    assert_eq!(
        step4_summary["search_stats"]["incremental_terminal_rank_prunes"].as_u64(),
        Some(3)
    );
    assert_eq!(
        step4_summary["search_stats"]["prefix_frontier_hot_states"].as_u64(),
        Some(1)
    );
    assert_eq!(
        step4_summary["search_stats"]["prefix_frontier_cold_states"].as_u64(),
        Some(0)
    );

    let frontier = read_json(
        &run_dir
            .join("checkpoints")
            .join("frontier")
            .join("step-04")
            .join("band-03")
            .join("frontier.manifest.json"),
    );
    assert_eq!(
        frontier["counts"]["incremental_terminal_rank_prunes"].as_u64(),
        step4_summary["search_stats"]["incremental_terminal_rank_prunes"].as_u64()
    );
    assert!(frontier["memory_snapshot"]["rss_bytes"].is_u64());
    let frontier_inspect = assert_success(run_pen_cli([
        "inspect",
        &run_dir
            .join("checkpoints")
            .join("frontier")
            .join("step-04")
            .join("band-03")
            .join("frontier.manifest.json")
            .to_string_lossy(),
    ]));
    assert!(frontier_inspect.contains("prefix_heuristic_dropped:"));
    assert!(frontier_inspect.contains("memory_snapshot:"));

    let telemetry = read_text(&run_dir.join("telemetry.ndjson"));
    assert!(telemetry.contains("\"search_timing\""));

    fs::remove_dir_all(root).ok();
}

fn delta_step_indexes(value: &serde_json::Value) -> Vec<u32> {
    value
        .as_array()
        .expect("delta array")
        .iter()
        .map(|delta| delta["step_index"].as_u64().expect("step_index") as u32)
        .collect()
}

fn write_realistic_pressure_config(path: &std::path::Path) {
    let config = read_text(
        &workspace_root()
            .join("configs")
            .join("realistic_frontier_shadow.toml"),
    )
    .replace("workers = \"auto\"", "workers = 1")
    .replace("target_rss_gib = 8.5", "target_rss_gib = 0.0000001")
    .replace("soft_rss_gib = 9.0", "soft_rss_gib = 0.0000002")
    .replace("pressure_rss_gib = 10.5", "pressure_rss_gib = 0.0000010")
    .replace("emergency_rss_gib = 11.5", "emergency_rss_gib = 0.0000020")
    .replace("hard_rss_gib = 12.0", "hard_rss_gib = 0.0000040")
    .replace("spill_buffers_gib = 0.50", "spill_buffers_gib = 0.0000001")
    .replace(
        "checkpoint_buffers_gib = 0.25",
        "checkpoint_buffers_gib = 0.0000001",
    )
    .replace("worker_arena_mib = 64", "worker_arena_mib = 0");
    fs::write(path, config).expect("write realistic pressure config");
}
