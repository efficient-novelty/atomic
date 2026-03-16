mod support;

use std::fs;

use serde_json::Value;
use support::{
    assert_success, compact_step_summaries, load_trajectory_fixture,
    mutate_latest_frontier_manifest, read_json, read_text, run_compare_runs, run_pen_cli, temp_dir,
    workspace_root, write_pressure_config,
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
            "15",
        ]);
        assert_success(output);
    }

    let run_dir_a = root_a.join("deterministic-run");
    let run_dir_b = root_b.join("deterministic-run");
    let steps_a = compact_step_summaries(&run_dir_a);
    let steps_b = compact_step_summaries(&run_dir_b);
    let expected_steps = load_trajectory_fixture("reference_steps_until_15.json");

    assert_eq!(steps_a, steps_b);
    assert_eq!(steps_a, expected_steps);

    let debug_a = read_text(&run_dir_a.join("reports").join("latest.debug.txt"));
    let debug_b = read_text(&run_dir_b.join("reports").join("latest.debug.txt"));
    assert_eq!(debug_a, debug_b);
    let latest_a = read_text(&run_dir_a.join("reports").join("latest.txt"));
    assert!(latest_a.contains("search_profile: strict_canon_guarded"));
    assert!(latest_a.contains("replay_ablation: matches_reference_replay x15"));
    assert!(debug_a.contains("search_profile: strict_canon_guarded"));
    assert!(debug_a.contains("late_step_claim: executable_canon step 15 DCT nu=103"));
    assert!(debug_a.contains("replay ablation: matches_reference_replay x15"));
    assert!(debug_a.contains("canon evidence: charged_kappa=8"));
    assert!(debug_a.contains("frontier pressure: state=green action=none"));

    let frontier_manifest_a = read_json(
        &run_dir_a
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("frontier.manifest.json"),
    );
    let frontier_manifest_b = read_json(
        &run_dir_b
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("frontier.manifest.json"),
    );
    assert_eq!(frontier_manifest_a, frontier_manifest_b);
    assert_eq!(frontier_manifest_a["step_index"].as_u64(), Some(15));
    assert_eq!(frontier_manifest_a["band_index"].as_u64(), Some(8));
    assert!(
        run_dir_a
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("frontier-runtime.json.sha256")
            .exists()
    );
    assert!(
        run_dir_a
            .join("meta.sqlite3")
            .metadata()
            .expect("metadata db")
            .len()
            > 0
    );

    let cache_blob_a = read_json(
        &run_dir_a
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("frontier-runtime.json"),
    );
    let cache_blob_b = read_json(
        &run_dir_b
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("frontier-runtime.json"),
    );
    assert_eq!(cache_blob_a, cache_blob_b);

    let step15_summary = read_json(
        &run_dir_a
            .join("reports")
            .join("steps")
            .join("step-15-summary.json"),
    );
    assert_eq!(
        step15_summary["canon_evidence"]["charged_clause_kappa"].as_u64(),
        Some(8)
    );
    assert_eq!(
        step15_summary["canon_evidence"]["late_step_claim"]["status"].as_str(),
        Some("executable_canon")
    );
    assert_eq!(
        step15_summary["canon_evidence"]["late_step_claim"]["adopted_label"].as_str(),
        Some("DCT")
    );
    assert_eq!(
        step15_summary["canon_evidence"]["late_step_claim"]["adopted_nu"].as_u64(),
        Some(103)
    );
    assert_eq!(
        step15_summary["replay_ablation"]["status"].as_str(),
        Some("matches_reference_replay")
    );
    assert_eq!(
        step15_summary["replay_ablation"]["nu_delta"].as_i64(),
        Some(0)
    );
    assert_eq!(
        step15_summary["replay_ablation"]["clause_kappa_delta"].as_i64(),
        Some(0)
    );

    let hot_shard_a = fs::read(
        run_dir_a
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("hot-000.bin"),
    )
    .expect("step 15 hot shard should exist");
    let hot_shard_b = fs::read(
        run_dir_b
            .join("checkpoints")
            .join("frontier")
            .join("step-15")
            .join("band-08")
            .join("hot-000.bin"),
    )
    .expect("step 15 hot shard should exist");
    assert_eq!(hot_shard_a, hot_shard_b);

    let telemetry = read_text(&run_dir_a.join("telemetry.ndjson"));
    assert!(telemetry.contains("\"replay_ablation\""));
    assert!(telemetry.contains("\"status\":\"matches_reference_replay\""));
    assert!(telemetry.contains("\"prune_samples\""));

    fs::remove_dir_all(root_a).ok();
    fs::remove_dir_all(root_b).ok();
}

#[test]
fn compare_runs_script_emits_a_deterministic_evidence_signoff() {
    let root = temp_dir("compare-runs");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();

    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "cold-live",
        "--until-step",
        "15",
    ]));
    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "live-rerun",
        "--until-step",
        "15",
    ]));

    let frontier_resume_dir = root.join("frontier-resume");
    let frontier_resume_arg = frontier_resume_dir.to_string_lossy().to_string();
    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "frontier-resume",
        "--until-step",
        "12",
    ]));
    assert_success(run_pen_cli([
        "resume",
        &frontier_resume_arg,
        "--until-step",
        "15",
    ]));

    let step_resume_dir = root.join("compat-step-resume");
    let step_resume_arg = step_resume_dir.to_string_lossy().to_string();
    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "compat-step-resume",
        "--until-step",
        "12",
    ]));
    mutate_latest_frontier_manifest(&step_resume_dir, |manifest| {
        manifest["resume_compatible"]["search_semantics_hash"] =
            Value::String("blake3:other-search".to_owned());
    });
    assert_success(run_pen_cli([
        "resume",
        &step_resume_arg,
        "--until-step",
        "15",
    ]));

    let reevaluate_dir = root.join("compat-reevaluate");
    let reevaluate_arg = reevaluate_dir.to_string_lossy().to_string();
    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "compat-reevaluate",
        "--until-step",
        "12",
    ]));
    mutate_latest_frontier_manifest(&reevaluate_dir, |manifest| {
        manifest["resume_compatible"]["evaluator_hash"] =
            Value::String("blake3:other-eval".to_owned());
    });
    assert_success(run_pen_cli([
        "resume",
        &reevaluate_arg,
        "--until-step",
        "15",
    ]));

    let pressure_config = root.join("pressure.toml");
    write_pressure_config(&pressure_config);
    let pressure_config_arg = pressure_config.to_string_lossy().to_string();
    assert_success(run_pen_cli([
        "run",
        "--config",
        &pressure_config_arg,
        "--root",
        &root_arg,
        "--run-id",
        "spill-pressure",
        "--until-step",
        "15",
    ]));

    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "reference-replay",
        "--until-step",
        "16",
    ]));

    let text_a = root.join("comparison-a.txt");
    let json_a = root.join("comparison-a.json");
    let args_a = compare_args(&root, &text_a, &json_a);
    let stdout_a = assert_success(run_compare_runs(&args_a));

    let text_b = root.join("comparison-b.txt");
    let json_b = root.join("comparison-b.json");
    let args_b = compare_args(&root, &text_b, &json_b);
    let stdout_b = assert_success(run_compare_runs(&args_b));

    assert_eq!(stdout_a, stdout_b);
    assert_eq!(read_text(&text_a), read_text(&text_b));
    assert_eq!(read_text(&json_a), read_text(&json_b));
    assert!(stdout_a.contains("Comparison Signoff: ready"));
    assert!(stdout_a.contains("accepted hashes: all 7 lanes match baseline cold-live"));
    assert!(stdout_a.contains(
        "search-space counts: mismatches detected in compat-reevaluate, reference-replay"
    ));
    assert!(stdout_a.contains("late-step competition: all 7 lanes match baseline cold-live"));
    assert!(stdout_a.contains("Lane spill-pressure"));
    assert!(stdout_a.contains("search profile: strict_canon_guarded"));
    assert!(stdout_a.contains("governor sequence: 01-15 orange/spill_cold"));

    let summary = read_json(&json_a);
    assert_eq!(summary["signoff"]["status"].as_str(), Some("ready"));
    assert_eq!(summary["comparison_version"].as_u64(), Some(4));
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
        summary["late_step_competition_consistency"]["status"].as_str(),
        Some("all_match_baseline")
    );
    assert_eq!(
        summary["step15_claim_boundary"]["status"].as_str(),
        Some("consistent")
    );
    assert_eq!(
        summary["workstream4_rollout"]["status"].as_str(),
        Some("not_present")
    );
    assert_eq!(summary["baseline_lane"].as_str(), Some("cold-live"));
    assert_eq!(summary["lanes"].as_array().map(Vec::len), Some(7));

    let cold_live = lane_summary(&summary, "cold-live");
    assert_eq!(
        cold_live["search_profile"].as_str(),
        Some("strict_canon_guarded")
    );
    assert_eq!(
        cold_live["run_mode"].as_str(),
        Some("atomic_search_bootstrap")
    );
    assert_eq!(
        cold_live["trajectory_vs_baseline"]["matches"].as_bool(),
        Some(true)
    );
    assert_eq!(
        cold_live["accepted_hashes_vs_baseline"]["matches"].as_bool(),
        Some(true)
    );
    assert_eq!(
        cold_live["search_space_counts_vs_baseline"]["matches"].as_bool(),
        Some(true)
    );
    assert_eq!(
        cold_live["late_step_competition_vs_baseline"]["matches"].as_bool(),
        Some(true)
    );

    let frontier_resume = lane_summary(&summary, "frontier-resume");
    assert_eq!(
        frontier_resume["run_mode"].as_str(),
        Some("frontier_checkpoint_resume")
    );
    assert_eq!(
        frontier_resume["provenance_summary"]["frontier_checkpoint_resume"].as_u64(),
        Some(3)
    );

    let step_resume = lane_summary(&summary, "compat-step-resume");
    assert_eq!(
        step_resume["run_mode"].as_str(),
        Some("step_checkpoint_resume")
    );
    assert_eq!(
        step_resume["provenance_summary"]["step_checkpoint_resume"].as_u64(),
        Some(3)
    );

    let reevaluate = lane_summary(&summary, "compat-reevaluate");
    assert_eq!(
        reevaluate["run_mode"].as_str(),
        Some("step_checkpoint_reevaluate")
    );
    assert_eq!(
        reevaluate["provenance_summary"]["step_checkpoint_reevaluate"].as_u64(),
        Some(15)
    );
    assert_eq!(
        reevaluate["search_space_counts_vs_baseline"]["matches"].as_bool(),
        Some(false)
    );

    let pressure = lane_summary(&summary, "spill-pressure");
    assert_eq!(
        pressure["governor_summary"]["actions"]["spill_cold"].as_u64(),
        Some(15)
    );
    assert_eq!(
        pressure["latest_frontier"]["pressure_action"].as_str(),
        Some("spill_cold")
    );

    let reference = lane_summary(&summary, "reference-replay");
    assert_eq!(
        reference["search_profile"].as_str(),
        Some("strict_canon_guarded")
    );
    assert_eq!(reference["run_mode"].as_str(), Some("reference_replay"));
    assert_eq!(
        reference["provenance_summary"]["reference_replay"].as_u64(),
        Some(15)
    );
    assert_eq!(
        reference["latest_frontier"]["present"].as_bool(),
        Some(false)
    );
    assert_eq!(
        reference["accepted_hashes_vs_baseline"]["matches"].as_bool(),
        Some(true)
    );
    assert_eq!(
        reference["search_space_counts_vs_baseline"]["matches"].as_bool(),
        Some(false)
    );
    assert_eq!(
        reference["late_step_competition_vs_baseline"]["matches"].as_bool(),
        Some(true)
    );

    fs::remove_dir_all(root).ok();
}

fn compare_args(
    root: &std::path::Path,
    text_out: &std::path::Path,
    json_out: &std::path::Path,
) -> Vec<String> {
    vec![
        "--baseline".to_owned(),
        "cold-live".to_owned(),
        "--lane".to_owned(),
        lane_arg("cold-live", &root.join("cold-live")),
        "--lane".to_owned(),
        lane_arg("live-rerun", &root.join("live-rerun")),
        "--lane".to_owned(),
        lane_arg("frontier-resume", &root.join("frontier-resume")),
        "--lane".to_owned(),
        lane_arg("compat-step-resume", &root.join("compat-step-resume")),
        "--lane".to_owned(),
        lane_arg("compat-reevaluate", &root.join("compat-reevaluate")),
        "--lane".to_owned(),
        lane_arg("spill-pressure", &root.join("spill-pressure")),
        "--lane".to_owned(),
        lane_arg("reference-replay", &root.join("reference-replay")),
        "--text-out".to_owned(),
        text_out.to_string_lossy().to_string(),
        "--json-out".to_owned(),
        json_out.to_string_lossy().to_string(),
    ]
}

fn lane_arg(label: &str, path: &std::path::Path) -> String {
    format!("{label}={}", path.to_string_lossy())
}

fn lane_summary<'a>(summary: &'a Value, label: &str) -> &'a Value {
    summary["lanes"]
        .as_array()
        .expect("lanes")
        .iter()
        .find(|lane| lane["label"].as_str() == Some(label))
        .unwrap_or_else(|| panic!("missing lane {label}"))
}
