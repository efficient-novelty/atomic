# Architecture

`pen-atomic` is a deterministic strict-only Rust workspace for rediscovering
the current 15-step PEN genesis sequence from anonymous MBTT structure. This
document records the current executable architecture, not a superseded
scaffolding plan. For the remaining closeout item, see
[`plan_progress.md`](../plan_progress.md).

## Current executable surface

The repository can currently do all of the following:

- `pen-cli run` performs live atomic strict search through step 15.
- `pen-cli resume` consumes the latest compatible frontier generation when the
  frozen compatibility ladder admits it and otherwise falls back deterministically
  to step resume or step reevaluation.
- `pen-cli inspect` reads run directories and frontier manifests without
  mutating them.
- `pen-cli export-agda` exports accepted run artifacts, and it labels
  reference-replay fallback exports explicitly when no run directory is present.
- `xtask export-reference-agda` emits deterministic reference payloads.
- `scripts/compare_runs.py` emits deterministic text and JSON summaries across
  cold, rerun, resume, pressure, and reference lanes from stored artifacts.
- The accepted late-step executable canon is fixed at step 15 / `DCT` with
  `nu = 103`.

This executable surface is intentionally bounded. The live authoritative lane is
the current strict 15-step corpus, not a claim that the repo already ships a
broad open-ended anti-junk frontier engine.

## Design rules

The current workspace follows these hard rules:

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

`pen-core` defines the anonymous kernel:

- MBTT expressions, clauses, and telescopes
- canonical structural keys and stable hashing inputs
- exact rational arithmetic
- library snapshots and structural capability extraction

### `pen-type`

`pen-type` provides conservative structural legality:

- telescope checking
- scope and connectivity checks
- admissibility bands
- obligation tracking over the active window

### `pen-eval`

`pen-eval` owns authoritative structural scoring:

- native `nu`
- exact `rho` and bar computation
- SCC-based semantic minimality
- structural trace generation

### `pen-search`

`pen-search` is the live orchestrator:

- anonymous MBTT enumeration under admissibility constraints
- canonical dedupe
- exact deterministic acceptance by minimal positive overshoot
- bounded live bootstrap search through step 15
- resume-compatibility decisions for frontier and step artifacts

### `pen-store`

`pen-store` owns the frozen artifact contracts and bounded frontier persistence:

- run, step, and frontier manifests
- telemetry layout
- hot and cold shard persistence
- dedupe segments, checksum sidecars, cache blobs, and metadata storage
- governor evaluation and spill metadata for bounded frontier generations

### `pen-cli`

`pen-cli` is the executable integration surface:

- `run`
- `resume`
- `inspect`
- `export-agda`

It translates search results into reports, checkpoints, telemetry, and
human-readable output. Semantic labels appear here, not in the hot path.

### `pen-agda`

`pen-agda` renders accepted telescopes into proof-facing export artifacts. It is
downstream of accepted Rust artifacts and never participates in acceptance.

### `pen-accel`

`pen-accel` is optional and subordinate to CPU truth. Any acceleration
scaffolding is outside the authoritative acceptance contract described here.

## Runtime flow

The current strict run path looks like this:

1. `pen-cli` loads a TOML runtime config and decides the target step.
2. For targets within the supported live range, `pen-cli` selects live atomic
   bootstrap search; beyond that range it uses explicit reference replay.
3. `pen-search` derives the step bar from accepted history and obtains strict
   admissibility from `pen-type`.
4. `pen-search` enumerates candidate telescopes for the allowed `kappa` band and
   structural lane.
5. `pen-type` filters out inadmissible or disconnected candidates.
6. `pen-eval` computes exact `nu`, `rho`, and semantic minimality for surviving
   candidates.
7. `pen-search` removes canonical duplicates deterministically, accepts the
   minimal positive overshoot winner, and retains bounded frontier evidence for
   reporting and resume.
8. `pen-cli` writes run manifests, step checkpoints, frontier checkpoints, step
   reports, telemetry, and latest summaries.
9. Optional export paths turn accepted steps into Agda modules and an export
   manifest.

## Search and frontier model

The current live engine is a bounded exact strict lane, not a speculative
template or replay shortcut:

- admissibility defines the exact clause band and structural lane that may open
- enumeration generates anonymous MBTT telescopes only within that lane
- candidates are checked and evaluated immediately
- canonical duplicates are removed deterministically
- semantic minimality rejects candidates with admissible bar-clearing terminal
  SCC subbundles
- exact acceptance chooses the candidate with minimal positive overshoot, then
  applies deterministic structural tie-breakers
- retained valid candidates, prune samples, and frontier pressure metadata are
  persisted for the bounded lane

The bounded frontier runtime is real for this lane: hot and cold shards, dedupe
segments, checksum sidecars, `frontier-runtime.json`, and `meta.sqlite3` are all
written and reused by `inspect` and compatible `resume` paths.

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
- `checkpoints/frontier/step-XX/band-YY/frontier.manifest.json`
- `checkpoints/frontier/step-XX/band-YY/hot-*.bin`
- `checkpoints/frontier/step-XX/band-YY/cold-*.bin`
- `checkpoints/frontier/step-XX/band-YY/dedupe-*.txt`
- `checkpoints/frontier/step-XX/band-YY/frontier-runtime.json`
- checksum sidecars for persisted frontier payloads

Step checkpoints are the stable resume unit. Frontier checkpoints are disposable
speed artifacts that become resumable only when compatibility hashes and the
record layout match exactly.

## Resume model

There are two resume layers in the current codebase:

- a frozen compatibility ladder that chooses frontier resume, step resume, step
  reevaluation, or migration-required outcomes from stored artifacts
- a CLI resume path that consumes the latest compatible frontier generation when
  available and otherwise falls back to stored step artifacts

`pen-cli inspect` reads the same frontier manifests, shard files, and cache blob
surface that `pen-cli resume` uses for exact-match frontier resume.

## Reporting and Agda boundary

The reporting surface now explains the bounded lane at candidate level:

- standard output summarizes the accepted step, `nu`, `kappa`, `rho`, bar
  distance, overshoot, and provenance
- debug output adds retained valid candidates, prune classes, frontier
  retention, replay-ablation summaries, and pressure-state reporting
- step 15 explicitly carries the executable canon claim `DCT` at `nu = 103`

Agda remains strictly downstream:

- Rust discovers anonymous candidates
- Rust decides admissibility and acceptance
- Rust writes frozen reports and checkpoints
- Agda receives accepted telescopes for export or verification only

## Honesty boundary

The current repository can honestly claim:

- live atomic strict discovery through step 15
- exact deterministic acceptance over the bounded live candidate pool
- frozen late-step executable canon, including step 15 / `DCT` at `nu = 103`
- deterministic run artifacts and CLI-level evidence for the current corpus
- real bounded frontier persistence and frontier-backed deterministic resume
- deterministic comparison artifacts from stored run directories
- deterministic Agda export from accepted run artifacts with explicit fallback
  labeling

It should not claim:

- open-ended anti-junk retention beyond the current bounded strict lanes
- that the live search loop is already a broad frontier-driven explorer under
  sustained pressure
- that Agda participates in or proves acceptance truth
- that optional acceleration scaffolding is part of the authoritative acceptance
  contract
