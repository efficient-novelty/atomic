mod support;

use std::fs;

use support::{
    assert_success, fixtures_root, normalize_export_manifest, read_json, read_text, run_pen_cli,
    temp_dir, workspace_root,
};

#[test]
fn export_agda_emits_the_frozen_manifest_and_stub_source() {
    let output_dir = temp_dir("agda-export");
    let output_dir_arg = output_dir.to_string_lossy().to_string();
    let output = run_pen_cli([
        "export-agda",
        "--until-step",
        "2",
        "--output-dir",
        &output_dir_arg,
    ]);
    let stdout = assert_success(output);
    assert!(stdout.contains("steps: 2"));
    assert!(stdout.contains("source: ReferenceReplayFallback"));

    let actual_manifest = normalize_export_manifest(read_json(&output_dir.join("manifest.json")));
    let expected_manifest = read_json(
        &fixtures_root()
            .join("trajectory")
            .join("reference_export_manifest.json"),
    );
    assert_eq!(actual_manifest, expected_manifest);

    let actual_step = read_text(&output_dir.join("Step02.agda"));
    let expected_step = read_text(
        &fixtures_root()
            .join("trajectory")
            .join("reference_step_02.agda"),
    );
    assert_eq!(actual_step, expected_step);

    let actual_payload = read_text(&output_dir.join("Payload02.agda"));
    let expected_payload = read_text(
        &fixtures_root()
            .join("trajectory")
            .join("reference_payload_02.agda"),
    );
    assert_eq!(actual_payload, expected_payload);

    fs::remove_dir_all(output_dir).ok();
}

#[test]
fn export_agda_prefers_run_artifacts_and_carries_step_fifteen_claims() {
    let root = temp_dir("agda-export-run");
    let run_dir = root.join("agda-run");
    let export_dir = root.join("agda-out");
    let config = workspace_root()
        .join("configs")
        .join("debug.toml")
        .to_string_lossy()
        .to_string();
    let root_arg = root.to_string_lossy().to_string();
    let run_dir_arg = run_dir.to_string_lossy().to_string();
    let export_dir_arg = export_dir.to_string_lossy().to_string();

    assert_success(run_pen_cli([
        "run",
        "--config",
        &config,
        "--root",
        &root_arg,
        "--run-id",
        "agda-run",
        "--until-step",
        "15",
    ]));

    let stdout = assert_success(run_pen_cli([
        "export-agda",
        "--run-dir",
        &run_dir_arg,
        "--output-dir",
        &export_dir_arg,
    ]));
    assert!(stdout.contains("source: AcceptedRunArtifacts"));

    let manifest = read_json(&export_dir.join("manifest.json"));
    assert_eq!(manifest["source"].as_str(), Some("accepted_run_artifacts"));
    assert_eq!(
        manifest["steps"][14]["claims"]["nu_claim"]["nu_total"].as_u64(),
        Some(103)
    );
    assert_eq!(
        manifest["steps"][14]["claims"]["clause_kappa"].as_u64(),
        Some(8)
    );
    assert_eq!(
        manifest["steps"][14]["claims"]["import_steps"]
            .as_array()
            .map(|steps| steps.len()),
        Some(1)
    );
    assert_eq!(
        manifest["steps"][14]["claims"]["import_steps"][0].as_u64(),
        Some(10)
    );

    let step15 = read_text(&export_dir.join("Step15.agda"));
    assert!(step15.contains("open import Payload15 as Payload15"));
    let payload15 = read_text(&export_dir.join("Payload15.agda"));
    assert!(payload15.contains("-- canonical_key:"));
    assert!(payload15.contains("-- nu_total: 103"));

    fs::remove_dir_all(root).ok();
}
