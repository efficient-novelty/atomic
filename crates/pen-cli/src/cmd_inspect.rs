use crate::cli::InspectArgs;
use crate::cmd_run::current_search_compat;
use crate::report::{StepReport, load_step_reports, render_debug_report, render_standard_report};
use anyhow::{Context, Result, bail};
use pen_search::resume::decide_resume;
use pen_store::manifest::{FrontierManifestV1, RunManifestV1};
use std::fs;
use std::path::{Path, PathBuf};

pub fn inspect(args: InspectArgs) -> Result<String> {
    let path = absolute(args.path)?;

    if path.is_dir() && path.join("run.json").exists() {
        return inspect_run_dir(&path);
    }
    if path.file_name().and_then(|name| name.to_str()) == Some("run.json") {
        return inspect_run_dir(
            path.parent()
                .context("run.json must have a parent directory")?,
        );
    }
    if path
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with("-summary.json"))
    {
        let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let step: StepReport = serde_json::from_str(&text).context("parse step summary")?;
        return Ok(format!(
            "step {} ({})\nnu: {}\nkappa: {}\nrho: {}\nbar: {}\ncandidate: {}\ncanonical: {}",
            step.step_index,
            step.label,
            step.accepted.nu,
            step.accepted.clause_kappa,
            step.accepted.rho,
            step.objective_bar,
            step.accepted.candidate_hash,
            step.accepted.canonical_hash
        ));
    }
    if path.file_name().and_then(|name| name.to_str()) == Some("frontier.manifest.json") {
        let text = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
        let frontier: FrontierManifestV1 =
            serde_json::from_str(&text).context("parse frontier manifest")?;
        let decision = decide_resume(&current_search_compat(), &frontier);
        return Ok(format!(
            "frontier step {} band {}\nhot_states: {}\ncold_states: {}\ndedupe_keys: {}\nresume_decision: {:?}",
            frontier.step_index,
            frontier.band_index,
            frontier.counts.hot_states,
            frontier.counts.cold_states,
            frontier.counts.dedupe_keys,
            decision
        ));
    }

    bail!("unsupported inspect target: {}", path.display())
}

fn inspect_run_dir(run_dir: &Path) -> Result<String> {
    let manifest_path = run_dir.join("run.json");
    let run_text = fs::read_to_string(&manifest_path)
        .with_context(|| format!("read {}", manifest_path.display()))?;
    let manifest: RunManifestV1 = serde_json::from_str(&run_text).context("parse run manifest")?;
    let steps = load_step_reports(run_dir)?;

    Ok(format!(
        "{}\n\n{}",
        render_standard_report(&manifest.run_id, &steps),
        render_debug_report(&manifest.run_id, &steps)
    ))
}

fn absolute(path: PathBuf) -> Result<PathBuf> {
    if path.is_absolute() {
        Ok(path)
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}
