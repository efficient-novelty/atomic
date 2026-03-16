# Quantum Improvement Plan

Last updated: 2026-03-16

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
  retained prefix frontier, exact small-tree completion-bound pruning on newly
  created prefixes, and exact one-clause-short terminal-prefix bar pruning.
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
- realistic shadow now also keeps an exact incremental clause-family
  feasibility summary and uses it for active-window clause filtering keyed by
  `PrefixSignature`, and stored artifacts expose:
  - `incremental_clause_family_filter_hits`
  - `incremental_clause_family_prunes`
  - `incremental_active_window_clause_filter_hits`
  - `incremental_active_window_clause_filter_prunes`
- realistic shadow now also reuses an exact terminal admissibility summary as
  an explicit terminal-clause filter keyed by `PrefixSignature`, and stored
  artifacts expose:
  - `incremental_terminal_clause_filter_hits`
  - `incremental_terminal_clause_filter_prunes`
- realistic shadow now also keeps an exact incremental terminal
  trivial-derivability summary keyed by `PrefixSignature`, and stored artifacts
  expose:
  - `incremental_trivial_derivability_hits`
  - `incremental_trivial_derivability_prunes`
- realistic shadow now also memoizes exact multi-step partial-prefix
  bar-clearability decisions keyed by `PrefixSignature`, and stored artifacts
  expose:
  - `incremental_partial_prefix_bound_hits`
- realistic shadow now also exposes the first exact partial-prefix
  completion-bound payoff counters from stored artifacts:
  - `incremental_partial_prefix_bound_checks`
  - `incremental_partial_prefix_bound_prunes`
- realistic shadow now also exposes the exact one-clause-short terminal-prefix
  frontier prune counter from stored artifacts:
  - `incremental_terminal_prefix_bar_prunes`
- realistic shadow now also collapses exact single-continuation late-family
  suffixes in-place, caches the next-position active-window clause surface on
  each work item, and uses that continuation state to order online prefix work
  deterministically by remaining distance to terminal and exact continuation
  width
- per-step timing telemetry is now also persisted in stored step summaries and
  rendered by inspect output next to frontier memory bytes

## Main Gap

The remaining high-value gap is earlier sound pruning on partial prefixes.

Today the realistic shadow lane has real online prefix control flow, but its
strongest general exact pruning is still concentrated in tiny exact
continuation trees plus the retained terminal-prefix frontier. The next step is
to move more sound bound reasoning and memoized legality/filter reuse earlier
across broader late-step continuation surfaces without unsound inference.

## Workstreams

### 1. Earlier Partial-Prefix Bounds

Goal:
move exact pruning earlier than the current small-tree and terminal-prefix
frontier cases.

Deliverables:

- a sound broader partial-prefix bound story that can justify earlier bar
  pruning beyond exact tiny-subtree expansion
- prefix priority/order that reflects those stronger bounds
- no change to final exact evaluation, minimality, dedupe, or acceptance

Exit criteria:

- realistic shadow preserves the current accepted sequence
- `full_telescopes_evaluated` drops materially at late steps
- earlier exact-prune counters increase for realistic late steps beyond the
  already-landed small-tree and terminal-prefix prune cases

### 2. d = 2 Memoization

Goal:
reuse more exact non-family admissibility work keyed by the strengthened prefix
signature.

Deliverables:

- stronger exact admissibility summaries keyed by `PrefixSignature` beyond the
  landed legality/connectivity/family/active-window/terminal-clause/trivial-
  derivability/terminal-admissibility layer
- broader per-position exact filter reuse for late-step clause surfaces that
  cannot be discharged by the current family-shaped summaries alone
- cache-hit and prune metrics visible in run artifacts

Exit criteria:

- cache hits are measurable in stored artifacts
- repeated non-family admissibility/filter work drops at steps 10 to 15
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
- `incremental_partial_prefix_bound_hits`
- `incremental_partial_prefix_bound_checks`
- `incremental_partial_prefix_bound_prunes`
- `incremental_terminal_prefix_bar_prunes`
- per-step timing telemetry
- frontier memory high-water bytes

Next metrics to add:

- using the now-persisted timing and memory evidence to retune late-step
  prefix order

## Non-Negotiables

- no floats on the hot path
- no stochastic acceptance in guarded mode
- no semantic labels in core search/eval
- no accelerator path that bypasses exact check/eval/minimality/acceptance
- no regression of the guarded 15-step canon
- no claim that realistic shadow is already an open-ended anti-junk explorer

## Immediate Next Steps

1. Define the strongest sound partial-prefix bound that can be justified from
   the existing scoring/checking pipeline beyond the landed budgeted
   small-tree completion-bound prune and the landed one-clause-short
   terminal-prefix bar prune, without inventing unsound `nu_upper` logic.
2. Extend the landed legality/connectivity/family/active-window/terminal-
   clause/trivial-derivability/terminal-admissibility/multi-step partial-
   prefix bar-decision memo layer into further exact admissibility and late-
   family filter reuse keyed by strengthened `PrefixSignature` state.
3. Use `incremental_partial_prefix_bound_hits`,
   `incremental_partial_prefix_bound_checks`,
   `incremental_partial_prefix_bound_prunes`,
   `incremental_terminal_prefix_bar_prunes`, and the persisted timing/memory
   telemetry to retune late-step prefix priority/order.
4. Only after 1 to 3 are stable, revisit deterministic continuation profiles or
   interference-sidecar work.

## Definition Of Done For The Next Pass

The next meaningful search pass should do all of the following:

- broaden earlier exact partial-prefix pruning beyond the already-landed
  small-tree and terminal-prefix cases
- extend the new memoized legality/connectivity/family/active-window/terminal-
  clause/trivial-derivability reuse into stronger exact admissibility or
  late-family filter summaries
- expose the payoff in stored metrics
- preserve guarded and realistic accepted sequences
