# Current Realistic Shadow

Read this file when the task touches `realistic_frontier_shadow`, online prefix
search, realistic late-step pruning, memoization, bounds, or evidence counters.

## Stable Current Behavior

- `realistic_frontier_shadow` has real prefix-frontier retention and persisted
  frontier evidence.
- `pen-search` expands prefixes online through clause-position catalogs instead
  of enumerating all full telescopes first.
- `PrefixSignature` carries active-window hashing, shape/support summaries, and
  structural family flags.
- `PrefixLegalityCache` is live and keyed by `PrefixSignature`.

## Landed Reuse And Pruning

### Legality And Connectivity

- Reuse incremental legality and connectivity summaries.
- Reuse an exact historical-reanchor summary to shortcut temporal-shell
  terminal connectivity.

### Family And Admissibility Filtering

- Reuse exact clause-family feasibility summaries.
- Reuse exact active-window clause filtering.
- Reuse terminal trivial-derivability summaries.
- Reuse cached terminal admissibility decisions.
- Reuse explicit terminal-clause filtering before fallback telescope assembly.

### Terminal-Prefix Reuse

- Memoize exact terminal-prefix completion summaries.
- Carry exact late terminal accept-rank summaries on retained prefix groups.
- Store exact terminal-prefix best accept-rank summaries.
- Materialize retained terminal-prefix groups eagerly and cache their exact full
  candidates.

### Exact Bounds

- Compute exact terminal-prefix completion bounds.
- Run a budgeted exact small-tree completion bound on new root and child
  prefixes before queue entry.
- Memoize exact multi-step partial-prefix bar decisions.
- Store exact one-clause-short terminal-prefix bar decisions in the same
  partial-prefix bound cache.

### Surface Collapse And Ordering

- Materialize isolated exact remaining-two late surfaces when no competing work
  can interleave.
- Collapse exact single-continuation late-family suffixes in-place.
- Cache next-position active-window clause surfaces on work items.
- Order online prefix work by remaining distance to terminal and exact
  continuation width.

## Current Evidence Surface

- Reports and frontier manifests expose:
  `prefixes_created`, `full_telescopes_evaluated`,
  `canonical_dedupe_prunes`, and `semantic_minimality_prunes`.
- Reports, manifests, and inspect output expose memo counters including
  legality/connectivity, clause-family, active-window, terminal-clause,
  trivial-derivability, terminal-admissibility, terminal-prefix completion,
  terminal-rank, and partial-prefix bound reuse/prune counts.
- Step summaries persist search timing.
- Inspect output exposes frontier memory high-water metrics including RSS, hot
  frontier, cold frontier, dedupe, and persisted memory snapshot bytes.

## Current Evidence Snapshot

- Step `10` artifacts show `incremental_partial_prefix_bound_prunes = 1`,
  confirming an exact partial-prefix prune before queue entry.
- Step `11` and `12` artifacts show `incremental_partial_prefix_bound_hits = 1`,
  confirming multi-step partial-prefix bar decisions are reused.
- Step `13` artifacts show `prefix_states_explored = 1`,
  `incremental_terminal_prefix_completion_hits = 1`, and
  `incremental_terminal_prefix_rank_hits = 1`, confirming isolated remaining-
  two collapse avoids the later queued child-bound replay.
- Step `14` and `15` artifacts show `incremental_terminal_rank_prunes = 1` and
  `full_telescopes_evaluated = 1`, confirming dominated late retained surfaces
  are skipped before final candidate evaluation.
- Step `15` artifacts still show `prefix_states_explored = 2`,
  `incremental_partial_prefix_bound_hits = 1`,
  `incremental_partial_prefix_bound_checks = 3`, and
  `incremental_terminal_prefix_completion_hits = 2`, so the surviving
  temporal-shell root lane is still the main remaining late exact surface.
- Step `15` artifacts also show `incremental_legality_cache_hits = 19`,
  `incremental_active_window_clause_filter_hits = 18`,
  `incremental_connectivity_shortcuts = 2`, and
  `incremental_connectivity_fallbacks = 0`.

## Current Remaining Gap

- Add stronger sound bound pruning beyond the already-landed family,
  active-window, terminal-clause, terminal-admissibility, terminal-prefix, and
  partial-prefix reuse path.
- Add broader non-family admissibility/filter reuse before terminal completion
  summaries are built.
- Continue using timing and memory evidence to retune realistic late-step order.
