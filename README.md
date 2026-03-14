# pen-atomic

`pen-atomic` is a Rust-first rewrite of the current PEN strict lane.

The hot path lives entirely in Rust: search, evaluation, storage, resume, telemetry,
and reporting. Cubical Agda remains a sidecar for export and verification only.

## Frozen v1 commitments

- strict mode only
- deterministic CPU-first search
- exact integer or rational hot-path comparisons
- no semantic names in hot-path crates
- step checkpoints are the stable resume unit
- frontier checkpoints are compatibility-gated speed artifacts

## Current status

The repository now has a real integration surface for the frozen v1 contracts:

- `pen-cli run` writes `run.json`, step checkpoints, step reports, and telemetry.
- `pen-cli resume` extends the same run directory deterministically.
- `pen-cli inspect` reports on run directories and frozen artifact files.
- `pen-cli export-agda` and `xtask export-reference-agda` emit manifest-backed Agda stubs.

The important honesty note is that `run` and `resume` are now hybrid:

- they perform real live atomic bootstrap search through step 15,
- they no longer need deterministic reference replay for the current 15-step corpus,
- and they still write the same frozen artifacts the future frontier engine will consume.

The workspace is still being filled in according to the frozen order:

1. `pen-core`
2. `pen-store`
3. schemas and checkpoint tests
4. `pen-type`
5. `pen-eval`
6. `pen-search`
7. `pen-cli`
8. `pen-agda`
9. `pen-accel`

Progress is tracked in [plan_progress.md](plan_progress.md).

## Current commands

- `cargo run -p pen-cli -- run --config configs/debug.toml --until-step 15`
- `cargo run -p pen-cli -- resume runs/<run-id>`
- `cargo run -p pen-cli -- inspect runs/<run-id>`
- `cargo run -p pen-cli -- export-agda --run-dir runs/<run-id>`
- `cargo run -p xtask -- generate-schemas`
- `cargo run -p xtask -- export-reference-agda 15`

## Verification

- `cargo test --workspace`
- `cargo run -p pen-cli -- run --config configs/debug.toml --root %TEMP%\\pen-cli-smoke --run-id smoke-run --until-step 15`
- `cargo run -p pen-cli -- export-agda --until-step 2 --output-dir %TEMP%\\pen-agda-smoke`
- `cargo run -p xtask -- export-reference-agda 2`
