mod support;

use std::fs;

use support::{
    assert_success, fixtures_root, normalize_export_manifest, read_json, read_text, run_pen_cli,
    temp_dir,
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

    fs::remove_dir_all(output_dir).ok();
}
