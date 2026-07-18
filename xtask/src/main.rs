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
        Some("token-seal-check") => token_seal_check(repo_root()?),
        Some(other) => bail!("unknown xtask command: {other}"),
        None => bail!(
            "usage: cargo xtask <generate-schemas|export-reference-agda [until_step]|claim-replay-harness <capture|benchmark> ...|token-seal-check>"
        ),
    }
}

/// SEMANTIC_NORMALIZATION_PROGRAM ground rule 4: no public token
/// constructors. Scans the ENTIRE pen-type source tree (whitespace-
/// normalized, so multi-line signatures and wrapped returns like
/// `Result<Self, E>` are caught) for any `pub fn` returning a token
/// outside the sanctioned issuance allowlist, public token fields,
/// Deserialize derives or manual impls, and Default derives. The same
/// scan runs as a pen-type unit test (the repo has no CI, so `cargo test`
/// is the enforcement surface); this subcommand additionally prints the
/// module boundary hash recorded by downstream certificates.
fn token_seal_check(root: PathBuf) -> Result<()> {
    let src_dir = root.join("crates/pen-type/src");
    let mut sources = Vec::new();
    for entry in fs::read_dir(&src_dir).with_context(|| format!("read {}", src_dir.display()))? {
        let path = entry?.path();
        if path.extension().is_some_and(|ext| ext == "rs") {
            let text =
                fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
            sources.push((path, text));
        }
    }
    sources.sort_by(|a, b| a.0.cmp(&b.0));

    let mut violations = Vec::new();
    let mut boundary = Vec::new();
    for (path, text) in &sources {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("?");
        boundary.extend_from_slice(text.as_bytes());
        violations.extend(
            scan_token_seal(text)
                .into_iter()
                .map(|violation| format!("{name}: {violation}")),
        );
    }

    let boundary_hash = format!("blake3:{}", pen_core::hash::blake3_hex(&boundary));
    if violations.is_empty() {
        println!("token-seal-check: ok");
        println!("token_module_boundary_hash: {boundary_hash}");
        Ok(())
    } else {
        for violation in &violations {
            eprintln!("token-seal-check violation: {violation}");
        }
        bail!("token seal check failed with {} violation(s)", violations.len());
    }
}

const TOKEN_TYPES: [&str; 3] = ["TypedEliminatorToken", "NaturalityToken", "TypedLiftToken"];
const SANCTIONED_TOKEN_FNS: [&str; 3] = [
    "issue_typed_eliminator_token",
    "issue_naturality_token",
    "issue_typed_lift_token",
];

/// Shared scan logic; mirrored in pen-type's `token_seal` test. Any change
/// here must be mirrored there (both fail closed on divergence-prone
/// patterns rather than trying to parse Rust).
fn scan_token_seal(source: &str) -> Vec<String> {
    let mut violations = Vec::new();
    let flat = source.split_whitespace().collect::<Vec<_>>().join(" ");

    // 1. Public functions returning a token anywhere in the crate,
    //    including wrapped returns, minus the sanctioned issuance fns.
    let mut search = 0;
    while let Some(offset) = flat[search..].find("pub fn ") {
        let start = search + offset;
        let rest = &flat[start..];
        let signature_end = rest.find('{').unwrap_or(rest.len());
        let signature = &rest[..signature_end];
        let fn_name = signature
            .trim_start_matches("pub fn ")
            .split(['(', '<', ' '])
            .next()
            .unwrap_or("");
        if let Some(return_position) = signature.rfind("->") {
            let return_type = &signature[return_position..];
            let returns_token = TOKEN_TYPES.iter().any(|token| return_type.contains(token));
            if returns_token && !SANCTIONED_TOKEN_FNS.contains(&fn_name) {
                violations.push(format!(
                    "public fn `{fn_name}` returns a token outside the sanctioned issuance list"
                ));
            }
        }
        search = start + 7;
    }

    // 2. Public fields, `pub fn ... -> Self`, and Default derives inside
    //    token structs/impls; Deserialize anywhere near a token type.
    for token in TOKEN_TYPES {
        if let Some(struct_start) = flat.find(&format!("pub struct {token} {{")) {
            if let Some(body) = brace_block(&flat[struct_start..]) {
                let inner = &body[body.find('{').unwrap_or(0) + 1..];
                if inner.contains("pub ") {
                    violations.push(format!("{token} exposes a public field"));
                }
            }
            let preamble_start = struct_start.saturating_sub(600);
            let preamble = &flat[preamble_start..struct_start];
            if preamble.contains("Deserialize") {
                violations.push(format!("{token} derives Deserialize (a public constructor)"));
            }
            if preamble.contains("Default") {
                violations.push(format!("{token} derives Default (a public constructor)"));
            }
        }
        if flat.contains(&format!("Deserialize for {token}"))
            || flat.contains(&format!("Deserialize<'de> for {token}"))
        {
            violations.push(format!("manual Deserialize impl for {token}"));
        }
        let mut impl_search = 0;
        while let Some(offset) = flat[impl_search..].find(&format!("impl {token} {{")) {
            let impl_start = impl_search + offset;
            if let Some(body) = brace_block(&flat[impl_start..]) {
                let mut fn_search = 0;
                while let Some(fn_offset) = body[fn_search..].find("pub fn ") {
                    let fn_start = fn_search + fn_offset;
                    let signature_end =
                        body[fn_start..].find('{').unwrap_or(body.len() - fn_start);
                    let signature = &body[fn_start..fn_start + signature_end];
                    if let Some(return_position) = signature.rfind("->")
                        && signature[return_position..].contains("Self")
                    {
                        violations.push(format!("{token} exposes a public constructor"));
                    }
                    fn_search = fn_start + 7;
                }
                impl_search = impl_start + body.len();
            } else {
                break;
            }
        }
    }
    violations
}

/// The source slice from the first `{` to its matching close brace.
fn brace_block(source: &str) -> Option<&str> {
    let open = source.find('{')?;
    let mut depth = 0usize;
    for (index, character) in source[open..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[..open + index + 1]);
                }
            }
            _ => {}
        }
    }
    None
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
