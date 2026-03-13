use anyhow::{Context, Result, bail};
use schemars::schema_for;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() -> Result<()> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("generate-schemas") => generate_schemas(repo_root()?),
        Some(other) => bail!("unknown xtask command: {other}"),
        None => bail!("usage: cargo xtask <generate-schemas>"),
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
