# Search Contract

The search engine remains strict-only, deterministic, and CPU-first. With
Workstream 4 closed, the repo still ships explicit rollout profiles instead of
pretending there is only one live claim surface.

## Active profiles

- `strict_canon_guarded`: authoritative live lane for the current executable
  15-step corpus
- `relaxed_shadow`: comparison lane for the earlier admissibility-widening
  deltas through step 12
- `realistic_frontier_shadow`: broader comparison-backed lane with generative
  late enumeration and live prefix-frontier retention through step 15
- `demo_breadth_shadow`: experimental comparison-backed demo lane scaffold with
  explicit 5-minute, 10-minute, and 15-minute budget metadata; in the current
  milestone it still reuses realistic-shadow search semantics while the
  dedicated demo scheduler, funnel counters, and live phase observer land;
  demo runs now also persist first-pass narrative text and event artifacts, and
  `pen-cli run|resume --narrative` can append that per-step narrative to the
  terminal report

The realistic lane is real runtime behavior, not a paper placeholder, but it is
still rollout-gated by parity evidence rather than promoted to default truth.

## Current executable scope

The current CLI path can honestly claim all of the following:

- live search is used through step 15 in both the guarded and realistic lanes
- the CLI can also run the experimental `demo_breadth_shadow` profile configs,
  which currently carry demo budget metadata, persist demo narrative artifacts,
  and still reuse realistic-shadow search semantics for the actual search work
- deterministic reference replay is no longer needed for the current 15-step
  corpus, except when the CLI is explicitly asked to extend beyond the live
  range
- bounded resume consumes stored step/frontier artifacts instead of rebuilding
  a config-driven prefix
- exact step-4 through step-15 search emits deterministic frontier evidence
  while preserving the hot-path invariants the broader frontier engine must keep

## Evidence comparison lane

`scripts/compare_runs.py` is the canonical post-hoc evidence tool for the
bounded rollout surface. It reads existing run directories and compares:

- accepted trajectory equality or deltas
- accepted-hash equality or deltas
- per-step search-space count deltas
- admissibility-diagnostic deltas
- late-step competition deltas
- per-step provenance sequences
- replay-ablation summaries
- prune sample totals
- frontier-retention deltas
- governor state and pressure-action sequences
- missing demo narrative and event artifacts when a demo lane should have
  written them
- step-15 claim-boundary consistency

The script also emits a Workstream 4 rollout view:

- parity set: realistic lanes must preserve guarded trajectory, accepted hashes,
  and step-15 claim boundary while still showing broader late-step competition
- resume set: realistic frontier-resume, step-resume, and reevaluate lanes must
  each preserve the same guarded parity surface
- pressure set: pressure-backed realistic lanes must preserve the same parity
  while exercising non-neutral governor or spill behavior

The script emits a human-readable signoff report plus a machine-readable JSON
summary without moving truth out of the stored Rust artifacts.

## Non-negotiable invariants

1. No semantic labels or target IDs in hot-path crates.
2. No floating-point ranking or threshold checks in the hot path.
3. No mutable AST trees inside frontier states.
4. Every prune is classified as sound, quotient or dedupe, or heuristic shaping.
5. GPU acceleration is advisory only and must be rechecked on CPU before sound prune or acceptance.
6. The guarded lane remains authoritative until rollout parity and the docs agree on a wider honesty boundary.

## Honesty boundary

- no fake discoveries
- no fake breadth
- no silent guarded or replay fallback when a comparison-backed lane misses a
  floor, budget, or certification target
- no claim beyond persisted run evidence and comparison-backed summaries

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
- the guarded executable canon ending at step 15 / `DCT` with `nu = 103`
- canonical comparison of persisted evidence lanes via `scripts/compare_runs.py`
