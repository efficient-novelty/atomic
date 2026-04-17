use crate::claim_evidence::{
    EARLY_GENERATED_TARGETS, LATE_GENERATED_FLOORS, accepted_hash_parity_check, breadth_check,
    current_utc_timestamp, load_run, manifest_completeness_check, runtime_threshold_check,
    write_text,
};
use crate::cli::BenchmarkClaimLaneArgs;
use anyhow::{Result, bail};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
struct BenchmarkEntry {
    claim_run: String,
    completed_step: u32,
    runtime_ms: u64,
    parity_status: String,
    early_breadth_status: String,
    late_generated_floors_status: String,
    runtime_threshold_status: String,
    manifest_status: String,
}

#[derive(Clone, Debug, Serialize)]
struct BenchmarkAggregate {
    claim_run_count: usize,
    completed_step_15_count: usize,
    parity_success_count: usize,
    full_early_breadth_hit_count: usize,
    full_late_floor_hit_count: usize,
    runtime_threshold_pass_count: usize,
    min_ms: u64,
    median_ms: u64,
    p90_ms: u64,
    max_ms: u64,
}

#[derive(Clone, Debug, Serialize)]
struct ClaimBenchmark {
    benchmark_version: u32,
    generated_utc: String,
    guarded_run: String,
    runtime_threshold_ms: Option<u64>,
    aggregate: BenchmarkAggregate,
    runs: Vec<BenchmarkEntry>,
}

pub fn benchmark_claim_lane(args: BenchmarkClaimLaneArgs) -> Result<String> {
    if args.claim_run.is_empty() {
        bail!("provide at least one --claim-run directory");
    }

    let guarded = load_run(&args.guarded_run)?;
    let mut entries = Vec::new();
    for claim_path in &args.claim_run {
        let claim = load_run(claim_path)?;
        let parity = accepted_hash_parity_check(&guarded, &claim);
        let early = breadth_check(&claim, EARLY_GENERATED_TARGETS, true);
        let late = breadth_check(&claim, LATE_GENERATED_FLOORS, false);
        let runtime = runtime_threshold_check(&claim, args.runtime_threshold_ms);
        let manifest = manifest_completeness_check(&claim.manifest);

        entries.push(BenchmarkEntry {
            claim_run: claim_path.display().to_string(),
            completed_step: claim.manifest.position.completed_step,
            runtime_ms: runtime.total_runtime_ms,
            parity_status: if parity.status == "ready" {
                "pass".to_owned()
            } else {
                "fail".to_owned()
            },
            early_breadth_status: early.status,
            late_generated_floors_status: late.status,
            runtime_threshold_status: runtime.status,
            manifest_status: manifest.status,
        });
    }

    let runtimes = entries
        .iter()
        .map(|entry| entry.runtime_ms)
        .collect::<Vec<_>>();
    let aggregate = BenchmarkAggregate {
        claim_run_count: entries.len(),
        completed_step_15_count: entries
            .iter()
            .filter(|entry| entry.completed_step >= 15)
            .count(),
        parity_success_count: entries
            .iter()
            .filter(|entry| entry.parity_status == "pass")
            .count(),
        full_early_breadth_hit_count: entries
            .iter()
            .filter(|entry| entry.early_breadth_status == "pass")
            .count(),
        full_late_floor_hit_count: entries
            .iter()
            .filter(|entry| entry.late_generated_floors_status == "pass")
            .count(),
        runtime_threshold_pass_count: entries
            .iter()
            .filter(|entry| entry.runtime_threshold_status == "pass")
            .count(),
        min_ms: *runtimes.iter().min().unwrap_or(&0),
        median_ms: percentile(&runtimes, 0.5),
        p90_ms: percentile(&runtimes, 0.9),
        max_ms: *runtimes.iter().max().unwrap_or(&0),
    };
    let benchmark = ClaimBenchmark {
        benchmark_version: 1,
        generated_utc: current_utc_timestamp(),
        guarded_run: args.guarded_run.display().to_string(),
        runtime_threshold_ms: args.runtime_threshold_ms,
        aggregate,
        runs: entries,
    };
    let text = render_benchmark_text(&benchmark);

    if let Some(path) = args.json_out.as_deref() {
        write_text(path, &(serde_json::to_string_pretty(&benchmark)? + "\n"))?;
    }
    if let Some(path) = args.text_out.as_deref() {
        write_text(path, &text)?;
    }

    Ok(text)
}

fn percentile(values: &[u64], fraction: f64) -> u64 {
    if values.is_empty() {
        return 0;
    }
    let mut ordered = values.to_vec();
    ordered.sort_unstable();
    let rank = ((ordered.len() as f64 * fraction).ceil() as usize).saturating_sub(1);
    ordered[rank]
}

fn render_benchmark_text(benchmark: &ClaimBenchmark) -> String {
    let aggregate = &benchmark.aggregate;
    let mut lines = vec![
        "Claim Benchmark Summary".to_owned(),
        format!("guarded run: {}", benchmark.guarded_run),
        format!("claim run count: {}", aggregate.claim_run_count),
        format!(
            "runtime ms: min={} median={} p90={} max={}",
            aggregate.min_ms, aggregate.median_ms, aggregate.p90_ms, aggregate.max_ms
        ),
        format!(
            "completed step-15 count: {}",
            aggregate.completed_step_15_count
        ),
        format!("parity success count: {}", aggregate.parity_success_count),
        format!(
            "full early breadth hit count: {}",
            aggregate.full_early_breadth_hit_count
        ),
        format!(
            "full late floor hit count: {}",
            aggregate.full_late_floor_hit_count
        ),
    ];
    if benchmark.runtime_threshold_ms.is_some() {
        lines.push(format!(
            "runtime threshold pass count: {}",
            aggregate.runtime_threshold_pass_count
        ));
    }
    lines.push("per-run details:".to_owned());
    lines.extend(benchmark.runs.iter().map(|entry| {
        format!(
            "  - {}: completed_step={} runtime_ms={} parity={} early={} late={} manifest={}",
            entry.claim_run,
            entry.completed_step,
            entry.runtime_ms,
            entry.parity_status,
            entry.early_breadth_status,
            entry.late_generated_floors_status,
            entry.manifest_status
        )
    }));
    format!("{}\n", lines.join("\n"))
}
