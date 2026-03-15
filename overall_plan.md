# Overall Plan Reference

This file is the repo-wide reference freeze for the current `pen-atomic`
closeout state. It is not a build-from-scratch blueprint anymore. The active
remaining quantum-search item lives in [`quantum_progress.md`](quantum_progress.md).

## Settled decisions

- `pen-atomic` is strict-only.
- Rust owns the hot path: search, evaluation, storage, resume, reporting, and
  telemetry.
- Cubical Agda is an observational export and verification sidecar only.
- The hot path stays structural and name-free.
- Exact integer and rational arithmetic remain authoritative.
- Step checkpoints are the stable artifact and resume unit.
- Frontier checkpoints are compatibility-gated accelerators, not semantic truth.
- The executable late-step canon is fixed at step 15 / `DCT` with `nu = 103`.
- Historical alternate late-step totals remain provenance only.
- Optional acceleration scaffolding is outside the authoritative acceptance
  contract unless it is separately proven and revalidated on CPU.

## Current executable surface

The current workspace already ships the bounded strict lane:

- `pen-cli run` performs live atomic strict search through step 15.
- `pen-cli resume` performs deterministic frontier resume, step resume, step
  reevaluation, or migration-required failure according to the frozen
  compatibility ladder.
- `pen-cli inspect` reads run directories and frontier manifests without
  mutating them.
- `pen-cli export-agda` exports accepted run artifacts and explicitly labels
  reference-replay fallback exports.
- `xtask export-reference-agda` emits deterministic reference payloads.
- `scripts/compare_runs.py` compares stored cold, rerun, resume, pressure, and
  reference lanes and emits deterministic text and JSON summaries.

## Crate map

```text
pen-core
   ^
pen-type        pen-store
   ^               ^
pen-eval          |
   ^               |
   +---------- pen-search --------+
            ^       ^             |
         pen-accel  |           pen-agda
                \   |            /
                  pen-cli
```

- `pen-core` defines the anonymous MBTT kernel and canonical structural keys.
- `pen-type` enforces structural legality, connectivity, and admissibility.
- `pen-eval` computes authoritative `nu`, `rho`, bars, and minimality.
- `pen-search` performs bounded live strict search and resume decisions.
- `pen-store` owns manifests, checkpoints, frontier persistence, checksums,
  cache blobs, and metadata storage.
- `pen-cli` owns run integration, inspect, reporting, and user-facing output.
- `pen-agda` exports accepted artifacts downstream.
- `pen-accel` remains subordinate to CPU truth.

## Artifact contract

Each run directory is the authoritative surface:

- `run.json`
- `config.toml`
- `meta.sqlite3`
- `telemetry.ndjson`
- `reports/latest.txt`
- `reports/latest.debug.txt`
- `reports/steps/step-XX-summary.json`
- `checkpoints/steps/step-XX.json`
- `checkpoints/frontier/step-XX/band-YY/frontier.manifest.json`
- hot and cold shard files
- dedupe segment files
- `frontier-runtime.json`
- checksum sidecars for persisted frontier payloads

For the bounded lane, frontier persistence is real and deterministic. `inspect`
and compatible `resume` read the same frontier manifests, shard queues, cache
blob, and metadata surface.

## Resume ladder

The compatibility ladder is frozen:

- same AST, type, evaluator, search-semantics, and record-layout hashes:
  frontier resume
- same AST, type, and evaluator hashes but different search semantics: step
  resume
- same AST and type hashes but different evaluator: step reevaluation
- different AST schema: migration or replay required

## Boundaries that remain intentional

- The authoritative live lane is the bounded strict 15-step corpus.
- The repo does not yet claim a broad open-ended anti-junk frontier engine.
- Agda remains observational only and cannot steer acceptance.
- Haskell donor material remains provenance, not executable truth.
- Optional acceleration is outside the authoritative acceptance contract
  described in the current docs.

For the executable architecture, use
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md). For the remaining
quantum-search delta, use [`quantum_progress.md`](quantum_progress.md).
