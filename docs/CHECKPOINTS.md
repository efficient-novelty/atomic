# Checkpoints

Step checkpoints are the stable resume unit. Frontier checkpoints are resumable only
when compatibility hashes and the frontier record layout match exactly.

## Current artifact surface

Today the live-bootstrap-plus-replay surface writes:

- `run.json` as the top-level run manifest
- `reports/steps/step-XX-summary.json` as deterministic accepted-step reports
- `checkpoints/steps/step-XX.json` as self-contained step checkpoints
- `telemetry.ndjson` as the append-only event log

The full frontier checkpoint writer is still future work, but the frozen compatibility
decision surface already exists and is what `pen-cli inspect` uses when reading a
`frontier.manifest.json`.

## Resume policy

- Same `ast_schema_hash`, `type_rules_hash`, `evaluator_hash`,
  `search_semantics_hash`, and `record_layout_id`: resume from frontier checkpoint.
- Same AST, type, and evaluator hashes but different search semantics: discard
  frontier and resume from the last step checkpoint.
- Same AST and type hashes but different evaluator: resume from the last step
  checkpoint and re-evaluate from there.
- Different AST schema: require migration or replay.

## Scaffold behavior

`pen-cli resume` currently rebuilds the accepted trajectory from the stored config:

- it uses live atomic bootstrap search through step 15,
- it no longer needs deterministic reference replay for the current 15-step corpus,
- and it keeps the run artifact layout stable while the real frontier engine is still
  being built behind the same checkpoint contract.
