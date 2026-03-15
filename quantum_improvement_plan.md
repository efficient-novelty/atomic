# Quantum Improvement Plan

Last updated: 2026-03-15

This file is the operational plan for the repo's quantum-inspired search work.
It is intentionally focused on the live Rust codebase, not on older proposal
text or paper context.

## Objective

Improve `realistic_frontier_shadow` so it does less redundant work while
preserving the current exact acceptance contract:

- same accepted 15-step sequence in guarded mode
- same accepted sequence in realistic shadow mode
- exact integer/rational comparisons on the hot path
- deterministic ordering and acceptance
- no semantic labels in core search/eval

The target outcome is a stronger online prefix engine that prunes earlier,
reuses more local state, and measures its own payoff from stored artifacts.

## Current Baseline

The repo is already past the original "just add a prefix frontier" stage.

- `strict_canon_guarded` remains the authoritative executable lane.
- `realistic_frontier_shadow` already expands prefixes online through
  clause-position catalogs.
- `PrefixBound` exists and is already used for exact bound handling at the
  retained prefix frontier.
- `PrefixSignature` now carries:
  - clause position
  - obligation set id
  - active-window hash
  - shape/support summary hashes
  - structural family flags
  - exact serialized-prefix identity
- reports and frontier manifests already expose:
  - `prefixes_created`
  - `prefix_states_explored`
  - `prefix_states_merged_by_signature`
  - `prefix_states_exact_pruned`
  - `full_telescopes_evaluated`
  - `canonical_dedupe_prunes`
  - `semantic_minimality_prunes`
- realistic shadow now also reuses incremental legality/connectivity summaries
  keyed by `PrefixSignature`, and stored artifacts expose:
  - `incremental_legality_cache_hits`
  - `incremental_connectivity_shortcuts`
  - `incremental_connectivity_fallbacks`
  - `incremental_connectivity_prunes`

## Main Gap

The remaining high-value gap is earlier sound pruning on partial prefixes.

Today the realistic shadow lane has real online prefix control flow, but its
strongest exact pruning is still concentrated near the retained terminal-prefix
frontier. The next step is to move more sound bound reasoning and memoized
legality/filter reuse earlier in the search.

## Workstreams

### 1. Earlier Partial-Prefix Bounds

Goal:
move exact pruning earlier than the terminal-prefix frontier.

Deliverables:

- a sound partial-prefix bound story that can justify earlier bar pruning
- prefix priority/order that reflects those stronger bounds
- no change to final exact evaluation, minimality, dedupe, or acceptance

Exit criteria:

- realistic shadow preserves the current accepted sequence
- `full_telescopes_evaluated` drops materially at late steps
- earlier exact-prune counters increase for realistic late steps

### 2. d = 2 Memoization

Goal:
reuse local legality and clause-family work keyed by the strengthened prefix
signature.

Deliverables:

- incremental legality/check summary type
- cache of legality/connectivity summaries by prefix signature
- cache of admissibility summaries by prefix signature
- cache of per-position clause-family filters keyed by active-window state
- cache-hit metrics visible in run artifacts

Exit criteria:

- cache hits are measurable in stored artifacts
- repeated legality/filter work drops at steps 10 to 15
- accepted canon does not change

### 3. Search-Order Improvements

Goal:
improve deterministic search order without changing correctness.

Deliverables:

- better prefix priority inputs once stronger bounds exist
- optional deterministic continuation/beam variants behind shadow profiles
- no stochastic acceptance or relaxed correctness gates

Exit criteria:

- same accepted outputs
- reduced work or better retained-prefix coverage in realistic late steps

### 4. Restricted Interference Layer

Goal:
reduce redundant late-family exploration outside the authoritative path.

Deliverables:

- small audited normalization/canonicalization layer
- optional structural sharing or rewrite sidecar for late families only
- extracted candidates still pass exact check/eval/minimality/acceptance

Exit criteria:

- no correctness-path bypasses
- measurable reduction in redundant complete candidates

### 5. Declarative Late-Family Grammar

Goal:
replace hand-shaped late-family constructors with declarative clause-shape
grammars after the prefix engine and memoization are stronger.

Status:
deferred until workstreams 1 and 2 are stable.

## Metrics To Watch

Primary:

- `prefixes_created`
- `prefix_states_explored`
- `prefix_states_merged_by_signature`
- `prefix_states_exact_pruned`
- `full_telescopes_evaluated`
- `canonical_dedupe_prunes`
- `semantic_minimality_prunes`
- per-step timing telemetry
- frontier memory high-water bytes

Next metrics to add:

- using the new timing and memory evidence to retune late-step prefix order

## Non-Negotiables

- no floats on the hot path
- no stochastic acceptance in guarded mode
- no semantic labels in core search/eval
- no accelerator path that bypasses exact check/eval/minimality/acceptance
- no regression of the guarded 15-step canon
- no claim that realistic shadow is already an open-ended anti-junk explorer

## Immediate Next Steps

1. Define the strongest sound partial-prefix bound that can be justified from
   the existing scoring/checking pipeline without inventing unsound `nu_upper`
   logic.
2. Introduce an incremental legality summary type and thread it through online
   prefix expansion.
3. Extend the landed legality/connectivity memo layer into exact admissibility
   and late-family filter reuse keyed by strengthened `PrefixSignature` state.
4. Use the new timing telemetry and frontier memory high-water metrics to
   retune late-step prefix priority/order.
5. Only after 1 to 4 are stable, revisit deterministic continuation profiles or
   interference-sidecar work.

## Definition Of Done For The Next Pass

The next meaningful search pass should do all of the following:

- land at least one earlier exact partial-prefix prune
- extend the new memoized legality/connectivity reuse into admissibility or
  clause-family filtering
- expose the payoff in stored metrics
- preserve guarded and realistic accepted sequences
