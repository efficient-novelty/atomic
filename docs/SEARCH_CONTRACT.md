# Search Contract

The search engine is strict-only, deterministic, and CPU-first.

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
