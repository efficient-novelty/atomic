use crate::cli::ExportAgdaArgs;
use crate::report::{StepReport, load_step_reports, replay_reference_steps};
use anyhow::{Context, Result};
use pen_agda::export::{ExportStepInput, export_steps};
use pen_search::config::RuntimeConfig;
use std::fs;
use std::path::{Path, PathBuf};

pub fn export_agda(args: ExportAgdaArgs) -> Result<String> {
    let (run_id, steps, output_dir) = if let Some(run_dir) = args.run_dir.clone() {
        let run_dir = absolute(run_dir)?;
        let reports = load_step_reports(&run_dir)?;
        let run_id = run_dir
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("run")
            .to_owned();
        let output_dir = args.output_dir.unwrap_or_else(|| run_dir.join("agda"));
        (run_id, reports, output_dir)
    } else {
        let workspace = workspace_root();
        let config_text = fs::read_to_string(workspace.join("configs").join("default.toml"))
            .context("read default config")?;
        let config = RuntimeConfig::from_toml_str(&config_text).context("parse default config")?;
        let until_step = args.until_step.unwrap_or(config.search.until_step);
        let reports = replay_reference_steps(until_step, config.objective.window_depth)?;
        let output_dir = args
            .output_dir
            .unwrap_or_else(|| workspace.join("agda").join("Generated"));
        ("reference-export".to_owned(), reports, output_dir)
    };

    let step_inputs = steps.iter().map(step_to_export_input).collect::<Vec<_>>();
    let manifest = export_steps(&output_dir, &run_id, &step_inputs, args.verify)?;

    Ok(format!(
        "agda export written to {}\nsteps: {}\nverify_requested: {}",
        output_dir.display(),
        manifest.steps.len(),
        manifest.verify_requested
    ))
}

fn step_to_export_input(step: &StepReport) -> ExportStepInput {
    ExportStepInput {
        step_index: step.step_index,
        label: step.label.clone(),
        candidate_hash: step.accepted.candidate_hash.clone(),
        canonical_hash: step.accepted.canonical_hash.clone(),
        telescope: step.telescope.clone(),
    }
}

fn absolute(path: PathBuf) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("workspace root")
        .to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::export_agda;
    use crate::cli::ExportAgdaArgs;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temp_dir(name: &str) -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should move forward")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("pen-cli-{name}-{id}"));
        fs::create_dir_all(&dir).expect("temp dir should exist");
        dir
    }

    #[test]
    fn export_command_writes_manifest() {
        let output_dir = temp_dir("agda-export");
        let output = export_agda(ExportAgdaArgs {
            run_dir: None,
            output_dir: Some(output_dir.clone()),
            until_step: Some(2),
            verify: false,
        })
        .expect("export should succeed");

        assert!(output.contains("steps: 2"));
        assert!(output_dir.join("manifest.json").exists());
        fs::remove_dir_all(output_dir).ok();
    }
}
