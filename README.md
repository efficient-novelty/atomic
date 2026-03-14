# pen-atomic

`pen-atomic` is a deterministic strict-only Rust workspace for rediscovering
the current 15-step PEN genesis sequence from anonymous MBTT structure. Rust
owns the hot path: search, evaluation, storage, resume, telemetry, and
reporting. Cubical Agda remains an observational export and verification
sidecar.

## Claim boundary

- strict mode only
- CPU-authoritative exact integer and rational comparisons
- hot-path crates stay structural and name-free
- step checkpoints are the stable artifact and resume unit
- frontier checkpoints are compatibility-gated accelerators, not semantic truth
- Agda cannot influence search or acceptance
- the executable late-step canon is fixed at step 15 / `DCT` with `nu = 103`

## Current executable surface

- `pen-cli run` performs live atomic strict search through step 15 and writes
  `run.json`, step checkpoints, frontier checkpoints, reports, telemetry, and
  metadata.
- `pen-cli resume` reuses the latest compatible frontier generation when the
  frozen compatibility hashes and record layout match, otherwise it falls back
  deterministically to step resume or step reevaluation.
- `pen-cli inspect` reads run directories and frontier manifests without
  mutating them.
- `pen-cli export-agda` exports accepted run artifacts, or a clearly labeled
  reference-replay fallback when no run directory is provided.
- `xtask export-reference-agda` emits deterministic reference payloads.
- `scripts/compare_runs.py` compares stored cold, rerun, resume, pressure, and
  reference lanes and emits deterministic text and JSON summaries.

## Intentional limits

- The authoritative live lane is the bounded strict 15-step corpus, not a claim
  of open-ended anti-junk exploration.
- Historical alternate late-step totals remain provenance only.
- Agda artifacts are observational only.
- Optional acceleration scaffolding is outside the authoritative acceptance
  contract described here.

Progress and the remaining closeout item are tracked in
[plan_progress.md](plan_progress.md).

## Current commands

- `cargo run -p pen-cli -- run --config configs/debug.toml --root runs --run-id demo --until-step 15`
- `cargo run -p pen-cli -- resume runs/demo --until-step 15`
- `cargo run -p pen-cli -- inspect runs/demo`
- `cargo run -p pen-cli -- export-agda --run-dir runs/demo`
- `cargo run -p xtask -- export-reference-agda 15`

## Verification

- `cargo test --workspace`
- `cargo run -p pen-cli -- run --config configs/debug.toml --root $env:TEMP\pen-cli-smoke --run-id smoke-run --until-step 15`
- `cargo run -p pen-cli -- resume $env:TEMP\pen-cli-smoke\smoke-run --until-step 15`
- `cargo run -p pen-cli -- export-agda --run-dir $env:TEMP\pen-cli-smoke\smoke-run --output-dir $env:TEMP\pen-agda-smoke`
