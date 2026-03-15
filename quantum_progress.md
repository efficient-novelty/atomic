# Quantum Progress

Last updated: 2026-03-15

This file is the operational status snapshot for
[`quantum_improvement_plan.md`](./quantum_improvement_plan.md). It is not a
running journal.

## Objective

Improve the realistic shadow search lane with earlier sound pruning and d = 2
memoization while preserving the current exact acceptance contract and the
guarded 15-step canon.

## Current State

### Authoritative behavior

- `strict_canon_guarded` remains the authoritative executable lane.
- guarded and realistic lanes both preserve the current accepted 15-step
  sequence.
- exact evaluation, semantic minimality, and deterministic acceptance remain
  downstream of the prefix layer.

### What is already implemented

- `realistic_frontier_shadow` already expands prefixes online through
  clause-position catalogs.
- `PrefixBound` exists and is used for exact bound handling and bar pruning at
  the retained prefix frontier.
- `PrefixCache` and `PrefixSignature` are real search primitives, not
  placeholders.
- `PrefixLegalityCache` now reuses incremental legality/connectivity summaries
  keyed by `PrefixSignature` during realistic online prefix expansion.
- `PrefixSignature` now includes:
  - clause position
  - obligation set id
  - active-window hash
  - shape/support summary hashes
  - structural family flags
  - exact serialized-prefix identity
- frontier artifacts, reports, and inspect output already expose the main
  Phase-1 counters:
  - `prefixes_created`
  - `prefix_states_explored`
  - `prefix_states_merged_by_signature`
  - `prefix_states_exact_pruned`
  - `full_telescopes_evaluated`
  - `canonical_dedupe_prunes`
  - `semantic_minimality_prunes`
- reports and frontier manifests now also expose the first Phase-2 memoization
  payoff counters:
  - `incremental_legality_cache_hits`
  - `incremental_connectivity_shortcuts`
  - `incremental_connectivity_fallbacks`
  - `incremental_connectivity_prunes`

## What Is No Longer A Gap

The following work should be treated as complete enough to stop planning around
it as if it were still missing:

- adding any prefix frontier at all
- replacing ad hoc prefix grouping with explicit bounds/signature modules
- making realistic shadow expand prefixes online
- exposing the core Phase-1 counters in reports/manifests
- strengthening `PrefixSignature` beyond exact serialized-prefix identity

## Active Gaps

### 1. Earlier exact pruning on partial prefixes

The realistic lane is online, but its strongest exact pruning is still too late
in the search. We do not yet have the stronger sound partial-prefix bound story
needed to prune earlier with confidence.

### 2. Real memoization on top of the strengthened signature

The strengthened signature is now used for incremental legality/connectivity
summary reuse, but it is not yet used for:

- cached admissibility summaries beyond the current exact check/connectivity
  reuse
- cached late-family structural filter reuse keyed by active-window state

### 3. Phase-2 metrics

The first cache-hit metrics are now live, but we still need the clause-family
filter side of the Phase-2 metric surface.

## Forward Plan

### Now

- define an earlier sound partial-prefix bound
- extend the new legality/connectivity memo layer into admissibility and
  clause-family filter reuse
- keep the strengthened-signature caches exact and deterministic

### Next

- retune prefix priority/order once earlier bounds exist
- add wall-clock and memory high-water metrics next to the new memo counters

### Later

- deterministic continuation profile variants
- restricted interference/normalization sidecar
- declarative late-family grammar

## Next Concrete Steps

1. Audit the current `nu` and legality pipeline and identify the strongest
   partial-prefix exact bound that can be justified without unsound inference.
2. Extend the landed incremental legality/connectivity summary path into exact
   admissibility and late-family filter reuse.
3. Use the memo counters from stored artifacts to retune late-step prefix
   priority/order.
4. Add timing and memory counters next to the new memo metrics.

## Guardrails

- no floats on the hot path
- no stochastic acceptance in guarded mode
- no semantic labels in core search/eval
- no accelerator path that bypasses exact check/eval/minimality/acceptance
- no over-claim that realistic shadow is already an open-ended online frontier
  explorer

## Verification

Latest relevant verification:

- `cargo test -p pen-type -p pen-search -p pen-cli -p pen-store`
