# Quantum Progress

Last updated: 2026-03-15

This file tracks the repo's actual progress against
[`quantum_improvement_plan.md`](./quantum_improvement_plan.md). It is meant to
prevent us from planning against an out-of-date baseline.

## Current Status

The repo was already ahead of the quantum plan's assumed starting point before
this pass:

- `strict_canon_guarded` already preserved the authoritative deterministic
  15-step sequence.
- `realistic_frontier_shadow` already had real prefix-frontier retention,
  persisted frontier artifacts, pressure accounting, and parity coverage through
  step 15.
- the main remaining Phase-1 gap was not "add any prefix frontier at all," but
  "replace post-hoc terminal-prefix grouping with true online prefix expansion."

This pass landed the first explicit quantum-oriented search primitives:

- `crates/pen-search/src/bounds.rs` now defines a shared `PrefixBound` API for
  exact lower and upper `nu` bounds and exact `rho_upper` bar pruning.
- `crates/pen-search/src/prefix_cache.rs` now defines `PrefixSignature`,
  `PrefixCache`, and `PrefixCacheStats`.
- `crates/pen-search/src/engine.rs` now uses the prefix cache instead of the
  previous ad hoc string-keyed grouping path in realistic shadow mode.
- `crates/pen-search/src/branch_bound.rs` and
  `crates/pen-search/src/worker.rs` now consume `PrefixBound` directly.
- reports, inspect output, and frontier manifests now expose
  `prefix_states_merged_by_signature`.

Acceptance semantics did not change:

- guarded and realistic lanes still preserve the current accepted trajectory.
- exact evaluation, semantic minimality, and deterministic acceptance remain
  downstream of the prefix layer.
- the hot path remains structural and name-free.

## Relevant Learnings

### 1. The quantum plan's baseline was partly stale

The repo no longer matches a pure "enumerate all full telescopes, then think
about frontiers later" baseline. It already has:

- realistic shadow late-step competition
- prefix-frontier persistence
- exact prefix pruning by bar
- bounded frontier pressure and spill behavior

That means new work should be framed as converging the current realistic lane
toward online prefix-first search, not rebuilding prefix retention from
scratch.

### 2. The real current gap is driver control, not just data structures

The realistic lane still works like this:

1. enumerate admissible full telescopes
2. derive terminal prefixes from those full telescopes
3. retain and prune prefix groups
4. evaluate only the retained full candidates

That is already useful, but it is not yet the Phase-1 target from the quantum
plan. The missing step is to make prefixes first-class search states that drive
expansion online.

### 3. Small exact abstractions help without threatening correctness

Pulling bounds and signatures into dedicated modules paid off immediately:

- bar pruning now goes through a shared exact `PrefixBound` contract
- prefix grouping is no longer hidden inside a one-off `BTreeMap<String, ...>`
- merge accounting is visible in reports and manifests

This is a good pattern for the next phases: add explicit exact search
abstractions first, then change control flow around them.

### 4. The d = 2 story is now better grounded in the code

The new `PrefixSignature` and merge metric make the "bounded entanglement"
translation more concrete. We still are not using the full active-window
signature or incremental legality summaries yet, but the code now has a natural
place for them.

### 5. Phase 1 and Phase 2 are now coupled more tightly

Once prefixes become true online states, the next bottleneck will be repeated
checking and repeated per-position clause-family work. That means:

- incremental legality summaries
- cached clause catalogs
- cached family filters

should be treated as the immediate follow-on to online prefix expansion, not as
far-future cleanup.

## What Changed In This Pass

- Added `PrefixBound` and tests in
  `crates/pen-search/src/bounds.rs`.
- Added `PrefixSignature`, `PrefixCache`, and merge-tracking tests in
  `crates/pen-search/src/prefix_cache.rs`.
- Rewired realistic-shadow terminal-prefix grouping in
  `crates/pen-search/src/engine.rs` to use the new cache layer.
- Rewired worker-side exact bar pruning to consume `PrefixBound`.
- Added `prefix_states_merged_by_signature` to:
  - `pen-search` step results
  - CLI reports and inspect output
  - frontier manifests and spill/runtime persistence
  - realistic-shadow integration assertions

## Immediate Next Steps

### Next Step 1: Online Prefix Expansion

Refactor `realistic_frontier_shadow` so it expands prefixes directly instead of
enumerating complete telescopes first and grouping them afterward.

The minimum acceptable shape is:

- frontier pops a retained prefix state
- exact bounds prune prefixes before full completion
- children are generated from the prefix
- only complete surviving prefixes enter full evaluation
- final exact evaluation, semantic minimality, dedupe, and acceptance stay
  unchanged

### Next Step 2: Strengthen PrefixSignature

Extend `PrefixSignature` beyond serialized-prefix identity so it captures the
compact state aspects the quantum plan cares about:

- clause position
- obligation/window summary
- support shape summary
- relevant family flags

The goal is to support sound merging and later memoization without smuggling
semantic labels into the hot path.

### Next Step 3: Add Missing Phase-1 Metrics

Add explicit counters for:

- `prefixes_created`
- `full_telescopes_evaluated`
- `canonical_dedupe_prunes`
- `semantic_minimality_prunes`

Some of this information already exists under different names; the next pass
should align the instrumentation with the quantum plan terminology so progress
is easier to measure.

### Next Step 4: Start Phase-2 Plumbing

After online prefix expansion is in place:

- introduce incremental check summaries
- cache per-position clause catalogs
- cache late-family structural filter results

This is the next likely performance win after online prefix-first control flow.

## Guardrails

The following remain non-negotiable while doing quantum-inspired work:

- no floats on the hot path
- no stochastic acceptance in guarded mode
- no semantic labels in core search/eval
- no accelerator path that can bypass exact evaluation or exact acceptance
- no claim that the realistic lane is already open-ended online prefix search

## Verification Status

Verified after this pass with:

- `cargo test -p pen-search -p pen-cli -p pen-store`
- `cargo test -p pen-search -p pen-cli -p pen-store realistic_shadow_run_preserves_reference_sequence_and_exposes_full_late_competition -- --nocapture`
