# Autonomous Claim Lane: Next Operational Slice

This note is for the local coding agent working on `desktop_claim_shadow`.
It assumes the latest remaining-one telemetry, bound-only cached bound access,
and cached-rank pre-prune-before-materialization work is already landed.

## Operating Read

The newest stored step-`4` rerun on the updated binary is now
`codex-claim-release-step4-telemetry-v1`, and it changes the next move.

At the hot early step-`4` checkpoints:

- claim widening bands `7/8/9` are still not active at `library_size = 3`
- cached bound hits are still `0`
- exact bound screening time is still `0 ms`
- terminal summary build time is still `0 ms`
- frontier pop/sort time is still `0 ms`
- direct compact remaining-one materialization is the dominant timer
- almost every materialized remaining-one group is still dying only after
  materialization

At `prefix_states_explored = 5` on `codex-claim-release-step4-telemetry-v1`:

- elapsed time was `423.8s`
- frontier queue length was `2770`
- legality summaries had risen to `28765`
- remaining-one groups materialized had reached `23220`
- post-materialization rank prunes had reached `23205`
- terminal materialization time had reached `393661 ms`

That means the deadlock is no longer "which exact screening branch should we
use first?" The hot cost is still payload that gets fully compact-materialized
before the lane learns it will die.

The follow-up stored rerun on a narrow partial exact-two split patch
(`codex-claim-release-step4-split-handoff-v1`) did not improve those same
checkpoints, and its exact-bound timer stayed at `0 ms` through
`prefix_states_explored = 6`. So partial exact-two split handoff is not the
next win on the current hot surface and should not be kept as the next patch.

## Do Not Spend The Next Slice On

1. Do not reopen memory compaction first. The old early-RSS cliff is still not
   the main blocker.
2. Do not retune claim widening bands `7/8/9` first. They are still not the
   active step-`4` surface.
3. Do not chase frontier queue rewrites first. The frontier pop timer is still
   `0 ms` on the hot early checkpoints.
4. Do not keep or reopen the partial exact-two split-handoff patch first. The
   stored rerun did not earn it.

## Work Order

### 1. Patch One Narrow Discovery-Summary / Delayed-Materialization Path

Choose the next slice around claim remaining-one discovery itself:

- discovery should store a compact pruning summary first
- full compact materialization should happen only when a prefix survives long
  enough to justify it
- keep the patch narrow enough that the new stored step-`4` rerun can answer
  whether the lane is truly materializing less work

### 2. Re-Earn One Stored `until_step = 4` Release Rerun On That Patch

Use the same local claim profile shape derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `until_step = 4`
- the same claim search profile
- release build
- live checkpoint persistence left on

The point is still iteration speed, not weakening the real lane.

### 3. Read The New Step-`4` Telemetry Before Changing More Search Code

Inspect `reports/steps/step-04-live.ndjson` and answer:

- Does `remaining_one_materialized` fall materially at matching checkpoints?
- Does `terminal_materialize_millis` stop dominating as early?
- Do legality-summary counts grow in a way that now justifies consuming cached
  summaries later instead of paying full materialization early?
- Do post-materialization rank prunes drop enough to show that the lane is now
  pruning earlier?

### 4. Only After The `until_step = 4` Read Improves, Rerun The Real Profile

Once the updated step-`4` rerun shows a real materialization-side win, rerun
the real `desktop_claim_shadow_1h` profile on that same updated binary and
inspect the stored artifacts again before touching compare/benchmark/
certification work.

## Keep/Drop Rule For The Next Patch

Keep a patch only if it does at least one of these on stored step-`4`
artifacts:

- pushes the same checkpoint materially earlier
- lowers `remaining_one_materialized`
- lowers `terminal_materialize_millis`
- lowers post-materialization rank-prune volume materially
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

- delayed materialization is clearly the next win
- step `4` is no longer the dominant blocker
- or some newly visible cost center overtakes materialization honestly

At that point, rewrite this file again from the new stored evidence instead of
keeping old rejected paths around.
