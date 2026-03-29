# Agda Sidecar

Agda is a verification sidecar only.

The Rust hot path may export accepted steps into generated Agda modules, but Agda
must never influence search ranking, sound prune decisions, or acceptance.

## Current bridge surface

`pen-agda` now exports:

- support modules for the bridge contract and abstraction barrier
- one `StepXX.agda` module per accepted step
- one `PayloadXX.agda` module per accepted step
- `manifest.json` describing the exported modules and hashes
- optional `StepXX.verify.log` files when verification is requested

The default generated output lives under `agda/Generated/`, and `xtask
export-reference-agda` can refresh the reference bridge directly from the frozen
reference sequence.

The support modules currently mirrored into each export bundle are:

- `BridgePayload.agda`
- `AbstractionBarrier.agda`
- `StepWitness.agda`

## Invariants

- Agda module rendering is deterministic from accepted-step artifacts.
- Each exported bundle is self-contained enough for `agda` to typecheck the
  generated step and payload modules directly.
- Verification status is recorded in the export manifest only after the export step.
- Missing `agda` executables are reported as skipped verification, never as hidden
  success.
- Agda results are observational only and never change selection outcomes.
