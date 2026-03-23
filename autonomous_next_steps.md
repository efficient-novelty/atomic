# Autonomous Claim Lane: Next Operational Slice

This note is for the local coding agent working on `desktop_claim_shadow`.
It assumes the delayed-materialization / cached-rank pre-prune work is already
landed and that the newest stored intended-profile rerun is
`codex-claim-release-full-delayed-summary-v1`.

## Operating Read

The newest stored intended-profile rerun on the updated binary is now
`codex-claim-release-full-delayed-summary-v1`, and it changes the next move.

Against the prior intended-profile baseline (`codex-claim-release-full-v1a`),
the new rerun kept the same frontier and retained-prefix shape but reached the
same hot step-`4` checkpoints materially earlier:

- at `prefix_states_explored = 24`, elapsed time fell to `1854.5s` from
  `2000.8s`, and observed RSS fell to about `218.7 MiB` from about `250.0 MiB`
- at `prefix_states_explored = 39`, elapsed time fell to `3010.8s` from
  `3440.6s`, and observed RSS fell to about `237.8 MiB` from about `275.0 MiB`
- at `prefix_states_explored = 43`, elapsed time fell to `3309.4s` from
  `3844.7s`, while frontier queue length, legality summaries, and retained
  prefix-cache shape stayed identical at `2732`, `205199`, and `39 / 144845`

At about the same wall clock as the old run's last stored checkpoint
(`3936.1s`), the new rerun had already reached `prefix_states_explored = 52`
instead of `44`, with:

- frontier queue length down to `2723` instead of `2731`
- legality summaries up to `246986`
- retained prefix-cache groups still flat at `39`
- observed RSS at about `252.7 MiB` instead of about `268.0 MiB`

At the manual stop checkpoint on `codex-claim-release-full-delayed-summary-v1`
(`prefix_states_explored = 59`, `4529.4s`):

- frontier queue length was `2716`
- legality summaries had risen to `279487`
- retained prefix-cache groups and candidates were still flat at `39 / 144845`
- `remaining_one_materialized` was still only `39`
- pre-materialization rank prunes had risen to `273957`
- post-materialization rank prunes were still `0`
- cached bound hits were still `0`
- `terminal_summary_build_millis` had reached `4387822 ms`
- `terminal_materialize_millis` was still only `527 ms`

That means the deadlock is no longer "should we delay materialization?" The
lane is already killing almost all of the old dead remaining-one work before
materialization. The hot cost has moved into building the compact pruning
summaries themselves.

## Do Not Spend The Next Slice On

1. Do not reopen the rejected partial exact-two split-handoff path first. It
   still did not earn keep.
2. Do not reopen memory compaction first. The new full rerun stayed around
   `252-266 MiB` observed RSS while step `4` remained honest.
3. Do not chase later-step frontier or certification work first. The real full
   profile is still honestly blocked in step `4`.
4. Do not treat cached bound reuse as the immediate next win. Bound hits are
   still `0` on the new full-profile evidence.

## Work Order

### 1. Patch One Narrow Remaining-One Summary-Build Path

Choose the next slice around the new hot cost center:

- make remaining-one terminal-summary construction itself cheaper
- or add one still-narrow incumbent-based early kill that avoids paying full
  summary-build cost for non-improving surfaces
- keep the patch narrow enough that a new stored step-`4` rerun can answer
  whether `terminal_summary_build_millis` truly falls without weakening the
  retained-prefix and frontier shape

### 2. Re-Earn One Stored `until_step = 4` Release Rerun On That Patch

Use the same local claim profile shape derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `until_step = 4`
- the same claim search profile
- release build
- live checkpoint persistence left on

The point is still iteration speed, not weakening the real lane. Use
`codex-claim-release-full-delayed-summary-v1` and
`codex-claim-release-step4-delayed-summary-v1` as the comparison baselines.

### 3. Read The New Step-`4` Telemetry Before Changing More Search Code

Inspect `reports/steps/step-04-live.ndjson` and answer:

- Does `terminal_summary_build_millis` fall materially at matching checkpoints?
- Does the same wall clock now reach farther than
  `codex-claim-release-full-delayed-summary-v1`?
- Do legality-summary counts or frontier drainage improve without weakening the
  retained prefix-cache shape?
- Do pre-summary or pre-materialization rank prunes move early enough to show
  that the lane is now rejecting more work before summary build?

### 4. Only After The `until_step = 4` Read Improves, Rerun The Real Profile

Once the updated step-`4` rerun shows a real summary-side win, rerun the real
`desktop_claim_shadow_1h` profile on that same updated binary and inspect the
stored artifacts again before touching compare/benchmark/certification work.

## Keep/Drop Rule For The Next Patch

Keep a patch only if it does at least one of these on stored step-`4`
artifacts:

- pushes the same checkpoint materially earlier
- reaches materially farther at the same wall clock
- lowers `terminal_summary_build_millis`
- lowers legality-summary growth or frontier breadth without weakening retained
  prefix shape
- lowers pre-summary or pre-materialization work materially
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

- terminal-summary construction is clearly no longer the next win
- step `4` is no longer the dominant blocker
- or some newly visible cost center overtakes summary build honestly

At that point, rewrite this file again from the new stored evidence instead of
keeping old rejected paths around.
