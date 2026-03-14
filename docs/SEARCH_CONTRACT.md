# Search Contract

The search engine is strict-only, deterministic, and CPU-first.

## Current executable scope

The current CLI path is an honest hybrid bootstrap surface:

- live atomic search is used through step 15,
- deterministic reference replay is no longer needed for the current 15-step corpus,
- bounded resume now consumes stored step/frontier artifacts instead of
  rebuilding a full config-driven prefix,
- and the exact step-4 through step-15 path now emits deterministic frontier
  evidence while preserving the hot-path invariants the later frontier engine
  must keep.

## Evidence comparison lane

`scripts/compare_runs.py` is the canonical post-hoc evidence tool for the
current bounded surface. It reads existing run directories and compares:

- accepted trajectory equality or deltas
- per-step provenance sequences
- replay-ablation summaries
- prune sample totals
- frontier-retention deltas
- governor state and pressure-action sequences
- step-15 claim-boundary consistency

The script emits a human-readable signoff report plus a machine-readable JSON
summary without moving truth out of the stored Rust artifacts.

## Non-negotiable invariants

1. No semantic labels or target IDs in hot-path crates.
2. No floating-point ranking or threshold checks in the hot path.
3. No mutable AST trees inside frontier states.
4. Every prune is classified as sound, quotient or dedupe, or heuristic shaping.
5. GPU acceleration is advisory only and must be rechecked on CPU before sound prune
   or acceptance.

## Resume rules

- exact compatibility match -> frontier resume allowed
- same AST, type, and evaluator but different search semantics -> resume from step
- same AST and type but different evaluator -> resume from step and reevaluate
- AST schema change -> no automatic frontier resume

## What is already frozen

- exact rational `bar` and `rho` comparisons
- deterministic candidate hashing and canonical hashing
- step checkpoint compatibility hashes
- live atomic bootstrap search through step 15
- reference replay over the frozen 15-step telescope corpus
- canonical comparison of persisted evidence lanes via `scripts/compare_runs.py`
