# Checkpoints

Step checkpoints are the stable resume unit. Frontier checkpoints are resumable only
when compatibility hashes and the frontier record layout match exactly.

## Current artifact surface

Today the live-bootstrap-plus-replay surface writes:

- `run.json` as the top-level run manifest
- `reports/steps/step-XX-summary.json` as deterministic accepted-step reports
- `checkpoints/steps/step-XX.json` as self-contained step checkpoints
- `checkpoints/frontier/step-XX/band-YY/frontier.manifest.json` plus
  deterministic hot/cold shards, dedupe segments, checksum sidecars, and a
  `frontier-runtime.json` cache blob for exact step-4 through step-15 frontier
  evidence
- `meta.sqlite3` as the bounded frontier metadata DB for generations and memory
  events
- `telemetry.ndjson` as the append-only event log

The bounded resumable frontier checkpoint writer is now real. `pen-cli inspect`
and `pen-cli resume` read the same manifest, hot/cold shards, and cache blob
surface when handling a `frontier.manifest.json`.

## Post-hoc comparison

`scripts/compare_runs.py` reads the stored `run.json`, step summaries,
frontier manifests, frontier cache blobs, and telemetry from existing run
directories and emits:

- a human-readable signoff report
- a machine-readable JSON summary

That comparison lane is now the canonical way to compare cold runs, reruns,
frontier-backed resume, compatibility-forced step resume, compatibility-forced
reevaluation, spill-pressure runs, and reference-replay baselines from frozen
artifacts.

## Resume policy

- Same `ast_schema_hash`, `type_rules_hash`, `evaluator_hash`,
  `search_semantics_hash`, and `record_layout_id`: resume from frontier checkpoint.
- Same AST, type, and evaluator hashes but different search semantics: discard
  frontier and resume from the last step checkpoint.
- Same AST and type hashes but different evaluator: resume from the last step
  checkpoint and re-evaluate from there.
- Different AST schema: require migration or replay.

## Current resume behavior

`pen-cli resume` now consumes the latest stored frontier generation when the
frozen compatibility ladder admits an exact match, and otherwise falls back to
the last step checkpoint or reevaluates from there:

- exact-match frontier resume reads the same manifest and hot/cold shards that
  `pen-cli inspect` reads and preserves their stored worker-pressure metadata,
- search-semantics drift drops back to step-checkpoint resume,
- evaluator drift triggers step-checkpoint reevaluation,
- and AST/type drift still requires migration or replay.

For the bounded lane, the frontier surface also carries:

- checksum-verified shard and cache-blob persistence,
- spill generation and dedupe-segment metadata,
- and frozen governor state/action snapshots in both `frontier-runtime.json`
  and `meta.sqlite3`.
