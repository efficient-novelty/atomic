use anyhow::{Context, Result, bail};
use pen_agda::export::{ExportStepInput, export_steps};
use pen_agda::manifest::ExportSource;
use pen_core::library::{Library, LibraryEntry};
use pen_core::telescope::Telescope;
use pen_eval::bar::DiscoveryRecord;
use pen_search::config::RuntimeConfig;
use pen_search::engine::claim_replay::{
    ClaimRemainingOneSurfaceTarget, benchmark_claim_remaining_one_replay_fixtures,
    capture_claim_remaining_one_replay_fixtures_with_seed,
    default_claim_remaining_one_surface_targets, read_claim_remaining_one_replay_fixtures,
    render_claim_remaining_one_replay_benchmark_text, write_claim_remaining_one_replay_benchmark,
    write_claim_remaining_one_replay_fixtures,
};
use pen_search::expand::evaluate_candidate;
use schemars::schema_for;
use std::collections::BTreeSet;
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
        Some("claim-replay-harness") => claim_replay_harness(repo_root()?, args.collect()),
        Some(other) => bail!("unknown xtask command: {other}"),
        None => bail!(
            "usage: cargo xtask <generate-schemas|export-reference-agda [until_step]|claim-replay-harness <capture|benchmark> ...>"
        ),
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
            bit_kappa: evaluated.bit_kappa,
            clause_kappa: evaluated.clause_kappa,
            nu: evaluated.nu,
            rho: evaluated.rho,
            telescope,
        });
    }

    export_steps(
        &root.join("agda").join("Generated"),
        "xtask-reference-export",
        &steps,
        false,
        ExportSource::ReferenceReplayFallback,
    )?;
    Ok(())
}

fn claim_replay_harness(root: PathBuf, args: Vec<String>) -> Result<()> {
    let mut args = args.into_iter();
    match args.next().as_deref() {
        Some("capture") => {
            let output_path = args
                .next()
                .map(PathBuf::from)
                .unwrap_or_else(|| default_claim_replay_fixture_path(&root));
            let surface_targets = args
                .next()
                .map(|value| parse_claim_replay_surface_targets(&value))
                .transpose()?
                .unwrap_or_else(default_claim_remaining_one_surface_targets);
            let requested_targets = surface_targets.iter().copied().collect::<BTreeSet<_>>();
            let mut seed_fixtures = if output_path.exists() {
                read_claim_remaining_one_replay_fixtures(&output_path)?
            } else {
                Vec::new()
            };
            seed_fixtures.retain(|fixture| {
                requested_targets.contains(&ClaimRemainingOneSurfaceTarget {
                    prefix_cache_groups: fixture.surface.prefix_cache_groups,
                    prefix_cache_candidates: fixture.surface.prefix_cache_candidates,
                })
            });
            seed_fixtures.sort_by_key(|fixture| {
                (
                    fixture.surface.prefix_cache_groups,
                    fixture.surface.prefix_cache_candidates,
                    fixture.surface.prefix_states_explored,
                )
            });
            seed_fixtures.dedup_by(|left, right| {
                left.surface.prefix_cache_groups == right.surface.prefix_cache_groups
                    && left.surface.prefix_cache_candidates == right.surface.prefix_cache_candidates
            });
            let reused_count = seed_fixtures.len();
            let fixtures = capture_claim_remaining_one_replay_fixtures_with_seed(
                &surface_targets,
                &seed_fixtures,
                Some(&output_path),
            )?;
            write_claim_remaining_one_replay_fixtures(&output_path, &fixtures)?;
            println!(
                "captured {} claim replay fixtures into {} ({} reused, {} newly captured)",
                fixtures.len(),
                output_path.display(),
                reused_count,
                fixtures.len().saturating_sub(reused_count)
            );
            Ok(())
        }
        Some("benchmark") => {
            let fixture_path = args
                .next()
                .map(PathBuf::from)
                .unwrap_or_else(|| default_claim_replay_fixture_path(&root));
            let iterations = args
                .next()
                .map(|value| value.parse::<usize>().context("parse benchmark iterations"))
                .transpose()?
                .unwrap_or(10);
            let json_output_path = args.next().map(PathBuf::from);
            let fixtures = read_claim_remaining_one_replay_fixtures(&fixture_path)?;
            let benchmark = benchmark_claim_remaining_one_replay_fixtures(&fixtures, iterations)?;
            let rendered = render_claim_remaining_one_replay_benchmark_text(&benchmark);
            println!("{rendered}");
            if let Some(json_output_path) = json_output_path {
                write_claim_remaining_one_replay_benchmark(&json_output_path, &benchmark)?;
            }
            Ok(())
        }
        Some(other) => bail!("unknown claim-replay-harness command: {other}"),
        None => bail!(
            "usage: cargo xtask claim-replay-harness <capture [output_path] [surfaces_csv]|benchmark [fixture_path] [iterations] [json_output_path]>"
        ),
    }
}

fn default_claim_replay_fixture_path(root: &Path) -> PathBuf {
    root.join("tests")
        .join("fixtures")
        .join("claim_runtime")
        .join("remaining_one_plateau_fixtures.json")
}

fn parse_claim_replay_surface_targets(value: &str) -> Result<Vec<ClaimRemainingOneSurfaceTarget>> {
    if value.trim().is_empty() {
        bail!("claim replay surface target list must not be empty");
    }

    value
        .split(',')
        .map(|entry| {
            let entry = entry.trim();
            let (groups, candidates) = entry.split_once('/').ok_or_else(|| {
                anyhow::anyhow!("surface target '{entry}' must use groups/candidates")
            })?;
            Ok(ClaimRemainingOneSurfaceTarget {
                prefix_cache_groups: groups
                    .parse::<usize>()
                    .with_context(|| format!("parse groups in '{entry}'"))?,
                prefix_cache_candidates: candidates
                    .parse::<usize>()
                    .with_context(|| format!("parse candidates in '{entry}'"))?,
            })
        })
        .collect()
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
