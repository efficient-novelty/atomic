# Autonomous Claim Lane: Next Operational Slice

This note is for the local coding agent working on `desktop_claim_shadow`.
It assumes the latest remaining-one telemetry, bound-only cached bound access,
and cached-rank pre-prune-before-materialization work is already landed.

## Operating Read

The current deadlock is still a step-`4` throughput problem, not a late-band
claim-policy problem.

At step `4`:

- `strict_admissibility_for_mode(..., DesktopClaimShadow)` still uses
  `focus_family = None`
- `min_clause_kappa = 3`, `max_clause_kappa = 3`
- claim widening bands `7/8/9` are still not active at `library_size = 3`

So the hot cost is still the exact remaining-two discovery path, not the later
claim-generic widening surface.

The newest stored full-profile evidence is still `codex-claim-release-full-v1a`,
and the newest code slice now adds the telemetry needed to separate:

- cached-summary misses versus hits
- pre-materialize versus post-materialize rank pruning
- summary build time versus materialization time
- remaining-one work versus frontier drain cost

The next step is to earn one new stored rerun on that updated binary, not to
keep speculating from older artifacts.

## Do Not Spend The Next Slice On

1. Do not reopen memory compaction first. The old early-RSS cliff is no longer
   the main blocker.
2. Do not retune claim widening bands `7/8/9` first. They are still not the
   active step-`4` surface.
3. Do not chase worker-count tuning first. The open question is still inside
   engine-side exact screening and queue drainage.
4. Do not do a broad architecture rewrite before one updated stored rerun
   explains the new hot path honestly.

## Work Order

### 1. Run One Stored `until_step = 4` Release A/B First

Use a local claim config derived from `configs/desktop_claim_shadow_1h.toml`
with:

- `until_step = 4`
- the same claim search profile
- release build
- live checkpoint persistence left on

The point is iteration speed, not weakening the real lane.

### 2. Read The New Step-`4` Telemetry Before Changing More Search Code

Inspect `reports/steps/step-04-live.ndjson` and answer:

- Are cached bound hits still scarce, or is summary reuse no longer the main
  story?
- Are rank prunes now landing mostly before materialization or still after it?
- Is wall time dominated by:
  - `terminal_summary_build_millis`
  - `terminal_materialize_millis`
  - `frontier_sort_pop_millis`
  - or plain exact bound screening?
- Are `remaining_one_unknown_bound_budget_exhaustions` still high enough to
  justify more exact screening reuse before anything else?

### 3. If The Updated Rerun Still Stalls, Pick Exactly One Next Code Path

#### Path A: Partial Exact-Two In-Place Processing

Choose this if:

- materialization and post-materialize prunes are still large
- the incumbent is clearly tightening too late
- or the frontier queue is still broad even after the new pre-prunes

Patch shape:

- replace the current all-or-none exact-two handoff with a stable split
- process the in-order prefix of prepared children that still outrank the
  current frontier head
- exact-bound screen and queue only the remainder

Target tests:

- `exact_two_step_surface_partially_processes_children_that_outrank_frontier_head`
- `exact_two_step_surface_keeps_deterministic_order_after_split_handoff`

#### Path B: Claim Discovery Summary / Delayed Materialization

Choose this if:

- cached-summary misses remain common
- `terminal_summary_build_millis` stays large
- `legality_cache_entries.terminal_prefix_completions` still grows too fast
- or the lane is still paying for payload it does not survive long enough to
  materialize

Patch shape:

- discovery stores only a compact pruning summary first
- full compact materialization happens only when a prefix survives long enough
  to justify it

#### Path C: Frontier Queue Rewrite

Choose this only if:

- `frontier_sort_pop_millis` is a real share of step-`4` wall time

If that timer stays small, do not spend the next slice here.

### 4. Only After The `until_step = 4` Read Improves, Rerun The Real Profile

Once the updated step-`4` A/B looks materially better, rerun the real
`desktop_claim_shadow_1h` profile on the same updated binary and inspect the
stored artifacts again before touching compare/benchmark/certification work.

## Keep/Drop Rule For The Next Patch

Keep a patch only if it does at least one of these on stored step-`4`
artifacts:

- pushes the same checkpoint materially earlier
- lowers `remaining_one_materialized`
- raises cached pre-prunes materially
- flattens the frontier queue sooner
- or makes later step-`4` checkpoints keep moving after the old retained-prefix
  plateau

If the updated rerun does not show one of those, do not stack more speculative
search changes on top of it.

## Regression Set

Run these after each next search-code change:

```bash
cargo test -p pen-search claim_
cargo test -p pen-search online_work_items_
cargo test -p pen-search prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations
cargo test -p pen-search terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity
```

Add targeted tests for any new fast path before trusting live runs.

Do not block on `cargo test -p pen-search --lib` being fully green right now;
the broader tree still stops at
`engine::tests::demo_late_surface_carries_through_live_config_runs`.

## Stop Condition For This Note

This note is stale as soon as one new stored release rerun on the updated
binary shows which of these is true:

- step `4` is no longer the dominant blocker
- partial exact-two handoff is clearly the next win
- discovery-summary work is clearly the next win
- or frontier drain finally shows up as the real cost center

At that point, rewrite this file again from the new stored evidence instead of
keeping old candidate paths around.
