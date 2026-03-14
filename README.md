# pen-atomic

`pen-atomic` is a deterministic strict-only Rust workspace for rediscovering
the current 15-step PEN genesis sequence from anonymous MBTT structure. Rust
owns the hot path: search, evaluation, storage, resume, telemetry, and
reporting. Cubical Agda remains an observational export and verification
sidecar.

## Project status

- live atomic strict search currently recovers the full 15-step corpus
- the accepted late-step executable canon is fixed at step 15 / `DCT` with
  `nu = 103`
- `pen-cli run`, `resume`, `inspect`, and `export-agda` operate on real stored
  artifacts
- bounded frontier persistence and frontier-backed deterministic resume are real
  for the current lane
- hot-path crates stay structural and name-free
- Agda export is observational only and cannot influence acceptance
- optional acceleration scaffolding is outside the authoritative acceptance
  contract

Progress on final closeout is tracked in
[plan_progress.md](plan_progress.md). Workstream 3 is complete; the only
remaining repo-level closeout item is the final `pen-accel` decision.

## Current executable surface

- `pen-cli run` performs live atomic strict search through step 15 and writes
  `run.json`, step checkpoints, frontier checkpoints, reports, telemetry, and
  metadata
- `pen-cli resume` reuses the latest compatible frontier generation when
  possible and otherwise falls back deterministically to step resume or step
  reevaluation
- `pen-cli inspect` reads run directories and frontier manifests without
  mutating them
- `pen-cli export-agda` exports accepted run artifacts, or a clearly labeled
  reference-replay fallback when no run directory is provided
- `xtask export-reference-agda` emits deterministic reference payloads
- `scripts/compare_runs.py` compares stored cold, rerun, resume, pressure, and
  reference lanes and emits deterministic text and JSON summaries

## Prerequisites

You do not need to know Rust in depth to run this repository, but you do need a
Rust toolchain.

- Install Rust and Cargo: [https://rustup.rs](https://rustup.rs)
- Open a shell in the repository root
- Optional: install Python 3 if you want to use `scripts/compare_runs.py`
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

Intentional limits:

- the authoritative live lane is the bounded strict 15-step corpus, not a claim
  of open-ended anti-junk exploration
- historical alternate late-step totals remain provenance only
- Agda artifacts are observational only
- optional acceleration scaffolding is outside the authoritative acceptance
  contract described here
