# pen-atomic

`pen-atomic` is a deterministic strict-only Rust workspace for rediscovering
the current 15-step PEN genesis sequence from anonymous MBTT structure. Rust
owns the hot path: search, evaluation, storage, resume, telemetry, and
reporting. Cubical Agda remains an observational export and verification
sidecar.

## Project status

- `strict_canon_guarded` remains the authoritative live lane and recovers the
  full 15-step corpus
- `realistic_frontier_shadow` is a broader comparison-backed lane that
  preserves the same accepted 15-step sequence while exposing genuine late-step
  competition and prefix-frontier retention
- `demo_breadth_shadow` now has an experimental config/runtime scaffold for
  5-minute, 10-minute, and 15-minute comparison-backed demo runs; this first
  milestone still reuses realistic-shadow search semantics while the dedicated
  demo breadth controller and narrative pipeline land
- the accepted late-step executable canon is fixed at step 15 / `DCT` with
  `nu = 103`
- `pen-cli run`, `resume`, `inspect`, and `export-agda` operate on real stored
  artifacts
- bounded frontier persistence and frontier-backed deterministic resume are real
  for the current rollout lanes
- hot-path crates stay structural and name-free
- Agda export is observational only and cannot influence acceptance
- optional acceleration scaffolding is outside the authoritative acceptance
  contract

Rollout, parity hardening, and honesty-boundary closeout are tracked in
[overall_plan.md](overall_plan.md). Workstream 4 is complete. The guarded
lane remains authoritative while `realistic_frontier_shadow` stays
comparison-backed rather than default.

## Current executable surface

- `pen-cli run` performs live search through step 15 under explicit search
  profiles and writes `run.json`, step checkpoints, frontier checkpoints,
  reports, telemetry, and metadata
- `pen-cli resume` reuses the latest compatible frontier generation when
  possible and otherwise falls back deterministically to step resume or step
  reevaluation
- `pen-cli inspect` reads run directories and frontier manifests without
  mutating them
- `pen-cli export-agda` exports accepted run artifacts, or a clearly labeled
  reference-replay fallback when no run directory is provided
- `xtask export-reference-agda` emits deterministic reference payloads
- `scripts/compare_runs.py` compares stored guarded, realistic, rerun, resume,
  pressure, and reference lanes and emits deterministic text and JSON summaries
  including the current Workstream 4 parity and pressure rollout view
- `scripts/certify_claim_lane.py` audits a stored `desktop_claim_shadow` run
  against a guarded baseline and emits a claim-certificate summary plus JSON

## Prerequisites

You do not need to know Rust in depth to run this repository, but you do need a
Rust toolchain.

- Install Rust and Cargo: [https://rustup.rs](https://rustup.rs)
- Open a shell in the repository root
- Optional: install Python 3 if you want to use `scripts/compare_runs.py` or
  `scripts/certify_claim_lane.py`
- Optional: install Agda if you want `pen-cli export-agda --verify` to invoke
  the external `agda` executable

On the first `cargo` command, Cargo may download dependencies and build the
workspace. That first run is slower; later runs usually reuse the existing build
artifacts.

## If You're New To Cargo

The command shape used in this repo is:

```powershell
cargo run -p pen-cli -- run --config configs/debug.toml --run-id demo
```

Read it like this:

- `cargo run` means "build this Rust program if needed, then run it"
- `-p pen-cli` tells Cargo which workspace package to run
- the first `--` separates Cargo's own flags from the flags for `pen-cli`
- `run` is the `pen-cli` subcommand
- everything after `run` is an argument to `pen-cli`

By default, `cargo run` uses Rust's dev build profile and produces a binary in
`target/debug/`. If you want an optimized build, use:

```powershell
cargo run --release -p pen-cli -- run --config configs/cpu_only.toml --run-id full-cpu
```

That builds the optimized binary in `target/release/`.

If you prefer, you can build first and run the binary directly:

```powershell
cargo build -p pen-cli
.\target\debug\pen-cli.exe run --config configs/debug.toml --run-id demo
```

## The Three "Debug" Knobs

This repository has three different things that can look like "debug." They are
not the same.

1. Rust dev build profile
   `cargo run` and `cargo build` default to Rust's dev profile. That affects how
   the program itself is compiled and usually means faster compile times and
   slower runtime than `--release`.

2. `configs/debug.toml`
   This is a runtime configuration file. It sets `[mode].debug = true`, keeps
   search single-threaded, uses smaller memory limits, and defaults to
   `until_step = 6`. It is a good configuration for understanding the system,
   inspecting artifacts, and doing a quick local smoke run.

3. `pen-cli ... --debug`
   This is a CLI output flag. It changes what is printed to your terminal for
   `run` and `resume`, showing the verbose debug report instead of the standard
   short report. It does not change the accepted result. The run directory gets
   both `reports/latest.txt` and `reports/latest.debug.txt` either way.

Important: `configs/debug.toml` defaults to step 6, not step 15. If you use the
debug config and want to continue farther, pass `--until-step 15` explicitly.

## Runtime Configs

The repository ships a few useful runtime profiles:

| Config | What it is for | Important defaults |
| --- | --- | --- |
| `configs/debug.toml` | Easiest profile to inspect locally | single worker, lower memory, `until_step = 6`, CPU backend |
| `configs/default.toml` | General-purpose default | auto workers, `until_step = 8` |
| `configs/cpu_only.toml` | Full step-15 run with explicit CPU backend | auto workers, `until_step = 15`, CPU backend |
| `configs/desktop_16gb.toml` | Full step-15 run for a larger desktop | auto workers, `until_step = 15`, larger memory budget |

If you are unsure where to start:

- use `configs/debug.toml` for a first run you can inspect easily
- use `configs/cpu_only.toml` for the clearest full 15-step CPU-authoritative
  run

For rollout and comparison work, use the dedicated search-profile configs:

- `configs/strict_canon_guarded.toml`: authoritative guarded baseline
- `configs/relaxed_shadow.toml`: earlier widening lane for admissibility and
  competition deltas through step 12
- `configs/realistic_frontier_shadow.toml`: broader comparison-backed lane with
  generative late enumeration and prefix-frontier retention through step 15
- `configs/demo_breadth_shadow_5m.toml`: experimental narrow demo profile with
  a shared 90-second early exhaustive window and a 5-minute total budget
- `configs/demo_breadth_shadow_10m.toml`: experimental default demo profile
  with a shared 90-second early exhaustive window and a 10-minute total budget
- `configs/demo_breadth_shadow_15m.toml`: experimental broad demo profile with
  a shared 90-second early exhaustive window and a 15-minute total budget

Demo runs now also write first-pass per-step narrative artifacts in
`reports/steps/step-XX-narrative.txt` and `reports/steps/step-XX-events.ndjson`.
Add `--narrative` to `pen-cli run` or `pen-cli resume` if you want that stored
per-step demo narrative appended to the terminal report as well.

## Quick Start

### 1. Verify the workspace builds

```powershell
cargo test --workspace
```

### 2. Run a small first example

This uses the debug config exactly as written, so it stops at step 6.

```powershell
cargo run -p pen-cli -- run --config configs/debug.toml --root runs --run-id quick-start
```

If you want the verbose terminal report for the same run, add `--debug`:

```powershell
cargo run -p pen-cli -- run --config configs/debug.toml --root runs --run-id quick-start-debug --debug
```

### 3. Inspect what was written

```powershell
cargo run -p pen-cli -- inspect runs/quick-start
```

### 4. Resume that run to step 15

Because the saved config for `quick-start` defaults to step 6, we override it
here:

```powershell
cargo run -p pen-cli -- resume runs/quick-start --until-step 15 --debug
```

### 5. Export Agda artifacts from that run

```powershell
cargo run -p pen-cli -- export-agda --run-dir runs/quick-start
```

## Full Step-15 Runs

If you want the full current executable canon from scratch in one command, use a
full-step config.

Explicit CPU profile:

```powershell
cargo run -p pen-cli -- run --config configs/cpu_only.toml --root runs --run-id full-cpu
```

Larger desktop profile:

```powershell
cargo run -p pen-cli -- run --config configs/desktop_16gb.toml --root runs --run-id desktop-full
```

Single-worker debug-style run all the way to step 15:

```powershell
cargo run -p pen-cli -- run --config configs/debug.toml --root runs --run-id debug-full --until-step 15 --debug
```

## Guarded vs Realistic Comparison

The completed Workstream 4 rollout keeps the guarded lane authoritative and
uses the realistic lane as a comparison-backed rollout surface.

Authoritative guarded baseline:

```powershell
cargo run -p pen-cli -- run --config configs/strict_canon_guarded.toml --root runs --run-id guarded --until-step 15
```

Broader realistic shadow lane:

```powershell
cargo run -p pen-cli -- run --config configs/realistic_frontier_shadow.toml --root runs --run-id realistic --until-step 15
```

Experimental demo breadth scaffold:

```powershell
cargo run -p pen-cli -- run --config configs/demo_breadth_shadow_10m.toml --root runs --run-id demo-10m --until-step 15 --narrative
```

Compare the stored evidence:

```powershell
python scripts/compare_runs.py --baseline guarded --lane guarded=runs/guarded --lane realistic=runs/realistic
```

For demo lanes, the compare output now also calls out missing
`step-XX-narrative.txt` and `step-XX-events.ndjson` artifacts explicitly.

For claim lanes, the compare output now also includes a claim-lane audit that
checks claim-policy honesty, exact-screen reason coverage, prune-class
coverage, fallback evidence, and whether the stored run reaches the step-15
claim signoff surface.

Claim runs now also append `step_live_checkpoint` telemetry events and
`reports/steps/step-XX-live.ndjson` artifacts for the in-flight claim-path
memory diagnostics around steps 4 and 5, including observed process RSS,
prefix-cache/legality-cache sizes, frontier queue size, raw catalog widths, and
claim-surface widening flags.

To audit a stored claim run directly against a guarded baseline and emit a
certificate:

```powershell
python scripts/certify_claim_lane.py --guarded-run runs/guarded --claim-run runs/claim --runtime-threshold-ms 600000 --json-out runs/claim/claim_certificate.json --text-out runs/claim/claim_certificate.txt
```

The current smoke claim config is expected to fail that certification honestly
until the repo has a full step-15 claim bundle, breadth floors, and the
remaining manifest provenance fields.

To aggregate multiple stored claim runs into a repeatable benchmark bundle with
median/p90/max runtime, parity success counts, breadth-floor hit counts, and
manifest snapshots:

```powershell
python scripts/benchmark_claim_lane.py --guarded-run runs/guarded --claim-run runs/claim-a --claim-run runs/claim-b --runtime-threshold-ms 600000 --json-out runs/claim-benchmark.json --text-out runs/claim-benchmark.txt
```

The benchmark script summarizes stored claim-lane evidence only. It does not
hide incomplete breadth or parity evidence; instead it records how many stored
runs actually satisfy those gates.

For the full Workstream 4 rollout matrix, also compare:

- a realistic frontier-resume lane
- a realistic compatibility-forced step-resume lane
- a realistic compatibility-forced reevaluate lane
- a realistic pressure-backed lane

The comparison output now reports a Workstream 4 rollout section showing:

- cold realistic lanes that preserve guarded parity
- realistic resume and reevaluate lanes that preserve guarded parity
- realistic pressure lanes that exercise non-neutral governor behavior without
  losing parity

## What A Run Writes

Each run creates a directory under `--root/<run-id>`. For example, with
`--root runs --run-id quick-start`, the run directory is `runs/quick-start`.

Important files and directories:

- `run.json`
- `config.toml`
- `meta.sqlite3`
- `telemetry.ndjson`
- `reports/latest.txt`
- `reports/latest.debug.txt`
- `reports/steps/step-XX-summary.json`
- `checkpoints/steps/step-XX.json`
- `checkpoints/frontier/step-XX/band-YY/...`

Step checkpoints are the stable resume unit. Frontier checkpoints are
compatibility-gated accelerators, not semantic truth.

## Common Commands

Show CLI help:

```powershell
cargo run -p pen-cli -- --help
cargo run -p pen-cli -- run --help
cargo run -p pen-cli -- resume --help
cargo run -p pen-cli -- inspect --help
cargo run -p pen-cli -- export-agda --help
```

Export deterministic reference Agda payloads without a run directory:

```powershell
cargo run -p xtask -- export-reference-agda 15
```

If the `agda` executable is installed, ask the export command to verify too:

```powershell
cargo run -p pen-cli -- export-agda --run-dir runs/quick-start --verify
```

## Troubleshooting

- If `cargo run` says the run directory already exists, pick a new `--run-id` or
  remove the old directory first.
- If you use `configs/debug.toml` and forget `--until-step 15`, the command will
  stop at step 6 because that is the config default.
- If `export-agda --verify` says verification was skipped, that usually means
  the external `agda` executable is not installed or not on your `PATH`.
- If you want the simplest fully CPU-authoritative full run, prefer
  `configs/cpu_only.toml`.

## Claim Boundary

The repository can honestly claim all of the following today:

- strict mode only
- CPU-authoritative exact integer and rational comparisons
- hot-path crates stay structural and name-free
- step checkpoints are the stable artifact and resume unit
- frontier checkpoints are compatibility-gated accelerators, not semantic truth
- Agda cannot influence search or acceptance
- the executable late-step canon is fixed at step 15 / `DCT` with `nu = 103`
- `strict_canon_guarded` is the authoritative lane
- `realistic_frontier_shadow` is a broader comparison-backed lane, not the
  default truth surface

Intentional limits:

- the authoritative live lane remains the bounded guarded 15-step corpus, not a
  claim of open-ended anti-junk exploration
- `realistic_frontier_shadow` still requires comparison-backed parity evidence
  and pressure evidence before any default-lane promotion
- comparison-backed demo/reporting surfaces do not invent fake discoveries or
  fake breadth to look broader than the stored search work really was
- when a comparison-backed lane misses a floor, budget, or certification
  target, it must report the miss in stored artifacts instead of silently
  falling back to guarded or replay behavior
- comparison-backed docs and summaries only claim what persisted run evidence
  and compare output actually support
- historical alternate late-step totals remain provenance only
- Agda artifacts are observational only
- optional acceleration scaffolding is outside the authoritative acceptance
  contract described here
