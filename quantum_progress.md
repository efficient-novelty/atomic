# Quantum Progress

Last updated: 2026-03-16

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
- realistic shadow now also keeps an exact incremental clause-family
  feasibility summary keyed by `PrefixSignature`, pruning mixed late-family
  prefixes as soon as no admissible structural family remains.
- realistic shadow now also uses that strengthened family summary for exact
  active-window clause filtering keyed by `PrefixSignature`, skipping child and
  terminal clause options that cannot match any still-admissible structural
  family before child-prefix legality or terminal assembly runs.
- realistic shadow now also reuses that exact family summary for cached
  terminal admissibility decisions keyed by `PrefixSignature`, avoiding a full
  late-family package-match recomputation whenever the summary is available.
- realistic shadow now also reuses that exact terminal admissibility summary as
  an explicit terminal-clause filter keyed by `PrefixSignature`, so last-clause
  options can be rejected before terminal connectivity work or fallback
  telescope assembly runs.
- realistic shadow now also keeps an exact incremental trivial-derivability
  summary keyed by `PrefixSignature`, rejecting terminal continuations that are
  provably trivially derivable before full telescope assembly and before any
  direct admissibility fallback.
- realistic shadow now also computes an exact terminal-prefix completion bound
  per retained one-clause-short prefix and prunes any such prefix group that
  cannot clear the current bar before retained-prefix frontier planning runs.
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
  - `incremental_clause_family_filter_hits`
  - `incremental_clause_family_prunes`
  - `incremental_active_window_clause_filter_hits`
  - `incremental_active_window_clause_filter_prunes`
  - `incremental_terminal_clause_filter_hits`
  - `incremental_terminal_clause_filter_prunes`
  - `incremental_trivial_derivability_hits`
  - `incremental_trivial_derivability_prunes`
  - `incremental_terminal_admissibility_hits`
  - `incremental_terminal_admissibility_rejections`
  - `incremental_terminal_prefix_bar_prunes`
- step telemetry now also carries the first Phase-2 timing counters:
  - `step_wall_clock_millis`
  - `candidate_discovery_wall_clock_millis`
  - `prefix_frontier_planning_wall_clock_millis`
  - `selection_wall_clock_millis`
- stored step summaries now also persist `search_timing`, and `pen-cli
  inspect` step output now renders those per-step timing counters alongside
  the existing frontier-pressure memory bytes so late-step order retunes can
  be driven directly from stored run artifacts.
- deterministic reports, step summaries, and frontier inspect output now also
  surface the first memory high-water metrics for the retained prefix frontier:
  - `rss_bytes`
  - `hot_frontier_bytes`
  - `cold_frontier_bytes`
  - `dedupe_bytes`
  - plus persisted frontier `memory_snapshot` bytes in frontier manifests
- stored realistic-shadow step-11 artifacts now show the first landed earlier
  exact partial-prefix bar prune via
  `incremental_terminal_prefix_bar_prunes = 1`.

## What Is No Longer A Gap

The following work should be treated as complete enough to stop planning around
it as if it were still missing:

- adding any prefix frontier at all
- replacing ad hoc prefix grouping with explicit bounds/signature modules
- making realistic shadow expand prefixes online
- exposing the core Phase-1 counters in reports/manifests
- strengthening `PrefixSignature` beyond exact serialized-prefix identity
- extending the strengthened-signature memo layer into broader exact
  active-window clause filtering before child-prefix legality and terminal
  assembly
- extending the strengthened-signature memo layer into exact terminal trivial-
  derivability reuse before full telescope assembly
- extending the strengthened-signature memo layer into cached terminal
  admissibility decisions for late-family realistic shadow work
- moving that cached terminal admissibility reuse forward into an explicit
  terminal-clause filter before terminal connectivity or fallback telescope
  assembly
- landing an earlier exact terminal-prefix bar prune before retained-prefix
  frontier planning
- adding the first timing telemetry and deterministic frontier memory
  high-water metrics for the memoized realistic-shadow path
- persisting and rendering the per-step timing telemetry in stored step
  summaries and inspect output for realistic-shadow artifact analysis

## Active Gaps

### 1. Stronger partial-prefix bounds beyond family-based filtering and the landed terminal-prefix bar prune

The realistic lane now has an earlier exact prune for impossible mixed-family
prefixes, exact active-window clause filtering on child and terminal clause
options, and an exact one-clause-short terminal-prefix bar prune, but its
strongest `nu`/bar reasoning still only fires at the terminal-prefix frontier.
We do not yet have the stronger sound partial-prefix bound story needed to
prune confidently earlier than that one-clause-short point.

### 2. Broader non-family admissibility reuse beyond the landed filter layer

The strengthened signature is now used for incremental legality/connectivity
reuse, exact clause-family feasibility pruning, active-window clause filtering,
an explicit terminal-clause admissibility filter, terminal trivial-
derivability pruning, and cached terminal admissibility decisions, but it is
not yet used for:

- stronger exact reuse of non-family admissibility structure before the full
  terminal telescope is assembled
- a more explicit partial-prefix admissibility summary that goes beyond the
  landed family-shaped filter surface plus trivial-derivability reuse

### 3. Using the new timing and memory evidence to retune search order

The timing telemetry and deterministic frontier memory high-water metrics are
now live, but we have not yet used them to retune realistic late-step prefix
priority/order or to explain which retained-prefix paths are still too
expensive relative to their payoff.

## Forward Plan

### Now

- define a stronger sound partial-prefix bound beyond exact clause-family
  impossibility pruning, the landed active-window clause filtering, and the
  landed terminal-prefix bar prune
- use the new active-window, terminal-clause, trivial-derivability, and
  terminal-admissibility payoff counters together with
  `incremental_terminal_prefix_bar_prunes` to target broader exact
  admissibility/filter reuse
- use the now-persisted timing telemetry and deterministic frontier memory
  high-water metrics from stored step summaries and inspect output to identify
  the late-step prefix lanes that still do redundant work
- keep the strengthened-signature caches exact and deterministic

### Next

- retune prefix priority/order once earlier bounds exist and the new timing/
  memory evidence is stable

### Later

- deterministic continuation profile variants
- restricted interference/normalization sidecar
- declarative late-family grammar

## Next Concrete Steps

1. Audit the current `nu` and legality pipeline and identify the strongest
   partial-prefix exact bound that goes beyond the landed clause-family
   impossibility prunes, active-window clause filtering, and the one-clause-
   short terminal-prefix bar prune without unsound inference.
2. Extend the landed legality/connectivity/family/active-window/terminal-
   clause/trivial-derivability/terminal-admissibility memo path into another
   exact admissibility summary that fires before the full terminal telescope is
   assembled.
3. Use the expanded memo counters together with the now-persisted timing
   telemetry and frontier memory high-water metrics from stored artifacts to
   retune late-step prefix priority/order.

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
