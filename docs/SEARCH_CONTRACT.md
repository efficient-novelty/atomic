# Search Contract

The search engine is strict-only, deterministic, and CPU-first.

## Current executable scope

The current CLI path is an honest hybrid bootstrap surface:

- live atomic search is used through step 15,
- deterministic reference replay is no longer needed for the current 15-step corpus,
- and both paths write the same real run artifacts while preserving the hot-path
  invariants the later frontier engine must keep.

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
- AST schema change -> no automatic frontier resume

## What is already frozen

- exact rational `bar` and `rho` comparisons
- deterministic candidate hashing and canonical hashing
- step checkpoint compatibility hashes
- live atomic bootstrap search through step 15
- reference replay over the frozen 15-step telescope corpus
