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

The repository is being bootstrapped in the frozen order:

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
