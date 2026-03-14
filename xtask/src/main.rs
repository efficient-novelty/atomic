use anyhow::{Context, Result, bail};
use pen_agda::export::{ExportStepInput, export_steps};
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_eval::bar::DiscoveryRecord;
use pen_search::config::RuntimeConfig;
use pen_search::expand::evaluate_candidate;
use schemars::schema_for;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("generate-schemas") => generate_schemas(repo_root()?),
        Some("export-reference-agda") => {
            let until_step = args
                .next()
                .map(|value| value.parse::<u32>().context("parse until_step"))
                .transpose()?
                .unwrap_or(15);
            export_reference_agda(repo_root()?, until_step)
        }
        Some(other) => bail!("unknown xtask command: {other}"),
        None => bail!("usage: cargo xtask <generate-schemas|export-reference-agda [until_step]>"),
    }
}

fn repo_root() -> Result<PathBuf> {
    let cwd = env::current_dir().context("read current working directory")?;
    if cwd.join("Cargo.toml").exists() {
        Ok(cwd)
    } else {
        cwd.parent()
            .map(Path::to_path_buf)
            .context("xtask must run from the repo root or xtask directory")
    }
}

fn generate_schemas(root: PathBuf) -> Result<()> {
    write_schema(
        &root.join("schemas/run_manifest_v1.schema.json"),
        &schema_for!(pen_store::manifest::RunManifestV1),
    )?;
    write_schema(
        &root.join("schemas/step_checkpoint_v1.schema.json"),
        &schema_for!(pen_store::manifest::StepCheckpointV1),
    )?;
    write_schema(
        &root.join("schemas/frontier_manifest_v1.schema.json"),
        &schema_for!(pen_store::manifest::FrontierManifestV1),
    )?;
    write_schema(
        &root.join("schemas/telemetry_event_v1.schema.json"),
        &schema_for!(pen_store::telemetry::TelemetryEventV1),
    )?;

    Ok(())
}

fn write_schema(path: &Path, schema: &schemars::Schema) -> Result<()> {
    let json = serde_json::to_string_pretty(schema).context("serialize schema")?;
    fs::write(path, format!("{json}\n")).with_context(|| format!("write {}", path.display()))
}

fn export_reference_agda(root: PathBuf, until_step: u32) -> Result<()> {
    let config_text = fs::read_to_string(root.join("configs").join("default.toml"))
        .context("read default config")?;
    let config = RuntimeConfig::from_toml_str(&config_text).context("parse default config")?;
    let target = until_step.min(config.search.until_step).min(15);

    let mut library: Library = Vec::new();
    let mut history: Vec<DiscoveryRecord> = Vec::new();
    let mut steps = Vec::new();
    for step_index in 1..=target {
        let telescope = Telescope::reference(step_index);
        let evaluated = evaluate_candidate(&library, &history, telescope.clone())
            .with_context(|| format!("evaluate reference step {step_index}"))?;
        history.push(DiscoveryRecord::new(
            step_index,
            u32::from(evaluated.nu),
            u32::from(evaluated.clause_kappa),
        ));
        library.push(LibraryEntry::from_telescope(&telescope, &library));
        steps.push(ExportStepInput {
            step_index,
            label: step_label(step_index).to_owned(),
            candidate_hash: evaluated.candidate_hash,
            canonical_hash: evaluated.canonical_hash,
            telescope,
        });
    }

    export_steps(
        &root.join("agda").join("Generated"),
        "xtask-reference-export",
        &steps,
        false,
    )?;
    Ok(())
}

fn step_label(step_index: u32) -> &'static str {
    match step_index {
        1 => "Universe",
        2 => "Unit",
        3 => "Witness",
        4 => "Pi",
        5 => "S1",
        6 => "Trunc",
        7 => "S2",
        8 => "S3",
        9 => "Hopf",
        10 => "Cohesion",
        11 => "Connections",
        12 => "Curvature",
        13 => "Metric",
        14 => "Hilbert",
        15 => "DCT",
        _ => "Unknown",
    }
}
