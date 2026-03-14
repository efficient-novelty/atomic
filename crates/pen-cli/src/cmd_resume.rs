use crate::cli::ResumeArgs;
use crate::cmd_run::{build_run_manifest, now_utc, write_run_artifacts};
use crate::output::{OutputStyle, render_run_output};
use crate::report::{generate_steps, load_step_reports};
use anyhow::{Context, Result};
use pen_search::config::RuntimeConfig;
use pen_store::manifest::RunManifestV1;
use std::fs;
use std::path::PathBuf;

pub fn resume(args: ResumeArgs) -> Result<String> {
    let run_dir = absolute(args.run_dir)?;
    let manifest_path = run_dir.join("run.json");
    let run_text = fs::read_to_string(&manifest_path)
        .with_context(|| format!("read {}", manifest_path.display()))?;
    let manifest: RunManifestV1 = serde_json::from_str(&run_text).context("parse run manifest")?;

    let config_path = run_dir.join("config.toml");
    let config_text = fs::read_to_string(&config_path)
        .with_context(|| format!("read {}", config_path.display()))?;
    let config = RuntimeConfig::from_toml_str(&config_text).context("parse runtime config")?;
    let target = args.until_step.unwrap_or(config.search.until_step);

    if manifest.position.completed_step >= target {
        let steps = load_step_reports(&run_dir)?;
        return Ok(render_run_output(
            OutputStyle::from_debug(args.debug),
            &manifest.run_id,
            &steps,
        ));
    }

    let generated = generate_steps(target, config.objective.window_depth)?;
    let steps = generated.steps;
    let updated = now_utc()?;
    let new_manifest = build_run_manifest(
        &manifest.run_id,
        &config_text,
        &steps,
        manifest.created_utc,
        updated,
    );

    write_run_artifacts(
        &run_dir,
        &config_text,
        &new_manifest,
        &steps,
        config.objective.window_depth,
        generated.mode.as_str(),
    )?;
    Ok(render_run_output(
        OutputStyle::from_debug(args.debug),
        &manifest.run_id,
        &steps,
    ))
}

fn absolute(path: PathBuf) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

#[cfg(test)]
mod tests {
    use super::resume;
    use crate::cli::{ResumeArgs, RunArgs};
    use crate::cmd_run::run;
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
    fn resume_command_extends_reference_replay() {
        let root = temp_dir("resume");
        run(RunArgs {
            config: "configs/debug.toml".into(),
            root: root.clone(),
            run_id: Some("resume-run".to_owned()),
            until_step: Some(2),
            debug: false,
        })
        .expect("initial run should succeed");

        let output = resume(ResumeArgs {
            run_dir: root.join("resume-run"),
            until_step: Some(4),
            debug: false,
        })
        .expect("resume should succeed");

        assert!(output.contains("completed_step: 4"));
        fs::remove_dir_all(root).ok();
    }
}
