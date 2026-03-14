# Architecture

`pen-atomic` is a deterministic strict-only Rust workspace for rediscovering the
current 15-step PEN genesis sequence from anonymous MBTT structure. The hot path
is entirely Rust: AST handling, checking, admissibility, enumeration, scoring,
selection, manifests, checkpoints, telemetry, and CLI reporting. Cubical Agda is
kept as an export and verification sidecar only.

This document describes the current executable architecture, not the aspirational
end state. For project status and remaining deliverables, see
[`plan_progress.md`](../plan_progress.md).

## Current state

The current supported strict lane is honest and materially useful:

- `pen-cli run` performs live atomic search through step 15.
- `pen-cli resume` regenerates the same strict trajectory deterministically from
  the run config and writes the same frozen artifact surface.
- `pen-cli inspect` reads run directories and frozen manifests without mutating
  them.
- `pen-cli export-agda` and `xtask export-reference-agda` emit manifest-backed
  Agda stubs for accepted steps.
- Debug reporting now shows retained valid candidates, bar comparisons, minimal
  overshoot acceptance, and human-readable telescope translations.

The main incompleteness is below the CLI surface:

- frontier storage is still mostly contract-first rather than fully operational
- the memory governor is not yet enforcing the frozen pressure model
- resume compatibility logic exists, but frontier-backed resume is not yet wired
  into the live CLI path
- replay still exists as an explicit fallback path for requests outside the
  currently supported live bootstrap range

## Design rules

The current workspace follows a few hard architectural rules:

- strict mode only
- exact integer and rational comparisons on the hot path
- no semantic names in core search or evaluation crates
- step checkpoints are the stable artifact unit
- frontier checkpoints are compatibility-gated accelerators, not semantic truth
- Agda export and verification remain observational and cannot influence search

## Crate layout

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

### `pen-core`

`pen-core` defines the frozen anonymous kernel:

- MBTT expressions, clauses, and telescopes
- canonical structural keys and stable hashing inputs
- exact rational arithmetic
- library snapshots and capability extraction
- human-agnostic structural helpers used by the rest of the workspace

This crate is where the repo decides what the search objects actually are.

### `pen-type`

`pen-type` provides conservative structural legality:

- telescope checking
- scope and connectivity checks
- admissibility bands
- obligation tracking over the active window
- structural capability and debt summaries

This crate decides whether a candidate is even allowed to compete at a given
step.

### `pen-eval`

`pen-eval` owns structural scoring and exact bar math:

- native `nu`
- exact `rho` and bar computation
- SCC-based semantic minimality
- structural trace generation

This crate provides the authoritative scoring path for acceptance.

### `pen-search`

`pen-search` is the live search orchestrator:

- telescope enumeration under admissibility constraints
- canonical dedupe
- exact deterministic acceptance
- bounded bootstrap search through step 15
- resume-compatibility decision logic for future frontier resume

At the moment, the live search path is intentionally bounded and complete for
the current 15-step strict corpus rather than broad open-ended exploration.

### `pen-store`

`pen-store` owns frozen artifact contracts and runtime-storage surfaces:

- run and frontier manifest schemas
- step checkpoint types
- telemetry layout
- placeholders and scaffolding for queue, shard, blob, spill, sqlite, and
  migration machinery

The artifact schema is real and already used by the CLI, even though much of the
frontier runtime implementation is still incomplete.

### `pen-cli`

`pen-cli` is the executable integration surface:

- `run`
- `resume`
- `inspect`
- `export-agda`

It translates search results into run directories, reports, checkpoints,
telemetry, and human-readable output. It is also where semantic labels and the
new candidate-level debug presentation are allowed to appear.

### `pen-agda`

`pen-agda` turns accepted telescopes into proof-facing stubs and verification
artifacts. It currently provides export and optional verification support, but
does not participate in search or acceptance.

### `pen-accel`

`pen-accel` is optional and subordinate to CPU truth. It contains early
acceleration scaffolding and must not become the authoritative semantic path.

## Runtime flow

The current strict run path looks like this:

1. `pen-cli` loads a TOML runtime config and decides the target step.
2. `pen-cli::report::generate_steps` chooses live atomic bootstrap search when
   the requested step is within the currently supported range.
3. `pen-search` derives the step bar from accepted history and obtains strict
   admissibility from `pen-type`.
4. `pen-search` enumerates candidate telescopes for the allowed `kappa` band and
   structural lane.
5. `pen-type` filters out inadmissible or disconnected candidates.
6. `pen-eval` computes exact `nu`, `rho`, and semantic minimality for the
   surviving candidates.
7. `pen-search` dedupes by canonical key, applies exact deterministic acceptance
   by minimal positive overshoot, and preserves retained valid candidates for
   evidence reporting.
8. `pen-cli` writes run manifests, step checkpoints, step reports, telemetry,
   and `latest.txt` / `latest.debug.txt`.
9. Optional export paths turn the accepted steps into Agda modules and an export
   manifest.

## Search architecture

The current live engine is best understood as a bounded exact search loop rather
than a frontier-heavy general explorer.

For each step:

- admissibility defines the exact clause band and structural lane that may open
- enumeration generates anonymous MBTT telescopes only within that lane
- candidates are checked and evaluated immediately
- canonical duplicates are removed deterministically
- semantic minimality rejects candidates with admissible bar-clearing terminal
  SCC subbundles
- exact acceptance chooses the candidate with minimal positive overshoot, then
  uses deterministic structural tie-breakers

This is enough to recover the full current strict 15-step corpus. It is not yet
the final anti-junk or memory-safe frontier engine described in the longer-range
project plan.

## Reporting architecture

The reporting surface now has two layers:

- standard reports summarize the current accepted step, `nu`, `kappa`, `rho`,
  bar, and minimal overshoot
- debug reports expose retained valid candidates, candidate-level bar distance,
  human-readable telescope translations, imports from earlier steps, and the
  accepted minimal-overshoot winner

This means the current CLI can explain why a step won within the bounded search
pool, even though broader frontier evidence and prune-surface reporting are not
complete yet.

## Artifact model

Each run directory is the authoritative user-facing artifact surface. Today it
contains:

- `run.json`
- `config.toml`
- `meta.sqlite3`
- `telemetry.ndjson`
- `reports/latest.txt`
- `reports/latest.debug.txt`
- `reports/steps/step-XX-summary.json`
- `checkpoints/steps/step-XX.json`

The `checkpoints/frontier/` directory also exists, but the live CLI does not yet
persist or resume real frontier state there.

## Resume model

There are two resume layers in the current codebase:

- a real compatibility policy for frontier manifests and step reevaluation
- the current CLI resume path, which regenerates the target strict trajectory and
  rewrites the frozen artifact surface

So the architectural contract for richer resume already exists, but the
end-to-end frontier-backed runtime is still future work.

## Agda boundary

Agda remains downstream of accepted Rust artifacts:

- Rust discovers anonymous candidates
- Rust decides admissibility and acceptance
- Rust writes frozen reports and checkpoints
- Agda receives accepted telescopes for export or verification

This split is intentional. The sidecar can validate or present the result, but
it cannot steer the hot path.

## Current honesty boundary

The current repository can honestly claim all of the following:

- live atomic strict discovery through step 15
- exact deterministic acceptance over the real candidate pool used by the
  bounded engine
- frozen late-step executable canon, including step 15 / `DCT` at `nu = 103`
- deterministic run artifacts and CLI-level evidence for the current corpus

It should not yet claim:

- full frontier-backed resume
- finished shard/blob/queue storage
- implemented memory-governor pressure behavior
- open-ended anti-junk retention beyond the current bounded structural lanes
- final proof-facing Agda payload completeness
