# Completion Closeout Plan

Last updated: 2026-03-14

This file now tracks the final closeout residue only. Workstream 3 is complete.
The only remaining repo-level closeout item is Workstream 2.

## Fixed starting point

Treat the following as settled context, not open roadmap items:

- live atomic strict search recovers the full current 15-step corpus
- the accepted late-step executable canon is frozen as `26, 34, 46, 62, 103`
- step 15 is explicitly `DCT` at `nu = 103`
- exact deterministic acceptance is in place through step 15
- `pen-cli run`, `resume`, `inspect`, and `export-agda` all operate on real
  artifacts rather than contract-only placeholders
- the bounded frontier runtime persists manifests, hot/cold shards, dedupe
  segments, cache blobs, spill metadata, checksums, and `meta.sqlite3`
- frontier-backed resume, step resume, reevaluate, and migration-required
  fallback paths are real and deterministic
- reports, inspect output, and telemetry already surface provenance, canon
  evidence, frontier pressure, replay-ablation summaries, and sampled
  prune/frontier evidence for the bounded lane
- `scripts/compare_runs.py` emits deterministic text and JSON comparison
  summaries across cold live, rerun, frontier resume, compatibility-forced step
  resume, compatibility-forced reevaluate, spill-pressure, and reference-replay
  lanes from stored artifacts
- the Agda sidecar is deterministic and remains observational only
- the current CPU-authoritative step-15 discovery path already runs in under
  two seconds

## Workstream 3: Final claim-boundary sweep

Status:

- complete on 2026-03-14

Delivered:

- `README.md`, `docs/ARCHITECTURE.md`, `overall_plan.md`, and this file now
  tell the same current-state story instead of mixing executable truth with old
  scaffolding language
- stale top-level implementation-order and blueprint claims have been removed
- the core honesty boundary remains explicit:
  - hot path stays structural and name-free
  - exact arithmetic remains authoritative
  - step checkpoints remain the stable artifact unit
  - frontier checkpoints remain compatibility-gated accelerators
  - Agda remains observational only
  - step 15 remains explicitly `DCT` at `nu = 103`
- optional acceleration is excluded from the authoritative acceptance contract
  described by the docs, while its final ship-or-drop decision remains
  Workstream 2

Verification completed on 2026-03-14:

- `cargo test --workspace`
- a fresh step-15 smoke run passed from `pen-cli run`
- a fresh frontier-backed resume smoke run passed from step 12 to step 15
- a fresh `pen-cli export-agda` smoke run passed from accepted run artifacts

## Remaining open item

### Workstream 2: `pen-accel` final decision

Goal:

- remove ambiguity around acceleration from the completion path

Current reality:

- `pen-accel` is still placeholder-only
- the authoritative CPU path is already fast enough for the current strict
  completion target
- shipping acceleration without parity proof would weaken the repo's honesty
  boundary more than it would help users

Default closeout path:

- mark `pen-accel` non-shipping or experimental and explicitly out of the
  completion gate

Only choose the ship path if all of the following become true:

- it produces a measurable wall-clock improvement on the real authoritative lane
- it cannot change authoritative prune or acceptance outcomes
- every accelerated advisory result is exactly revalidated on CPU
- deterministic tests and docs prove the above

Concrete file targets:

- `Cargo.toml`
- `crates/pen-search/src/config.rs`
- `crates/pen-accel/*`
- `README.md`
- `docs/ARCHITECTURE.md`

Recommendation:

- take the non-shipping or experimental path unless fresh benchmark evidence
  proves acceleration is worth the added contract surface

Done when either:

- `pen-accel` is clearly documented as non-shipping or experimental and no
  longer part of the completion claim, or
- it ships with exact CPU-authoritative parity plus benchmark and regression
  evidence

## Completion gate

The repo closeout is now blocked only on Workstream 2. Workstream 3 is already
done:

- docs no longer contradict the actual executable surface
- `cargo test --workspace` passes
- the final step-15 run smoke passed
- the final resume smoke passed
- the final Agda export smoke passed

`plan_progress.md` can be retired once the `pen-accel` decision is made
explicitly enough to close Workstream 2.
