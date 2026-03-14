# Agda Sidecar

Agda is a verification sidecar only.

The Rust hot path may export accepted steps into generated Agda modules, but Agda
must never influence search ranking, sound prune decisions, or acceptance.

## Current bridge surface

`pen-agda` now exports:

- one `StepXX.agda` module per accepted step
- `manifest.json` describing the exported modules and hashes
- optional `StepXX.verify.log` files when verification is requested

The default generated output lives under `agda/Generated/`, and `xtask
export-reference-agda` can refresh the reference bridge directly from the frozen
reference sequence.

## Invariants

- Agda module rendering is deterministic from accepted-step artifacts.
- Verification status is recorded in the export manifest only after the export step.
- Missing `agda` executables are reported as skipped verification, never as hidden
  success.
- Agda results are observational only and never change selection outcomes.
