# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume the delayed-materialization patch is already landed and the current
full-profile baseline is `runs/codex-claim-release-full-delayed-summary-v1`.

## Goal

Reduce the remaining-one summary-build cost in step `4` without weakening the
real claim lane.

## Why This Is The Next Slice

- The current full-profile baseline already proved that delayed
  materialization works.
- The lane is no longer spending meaningful time in compact materialization
  itself.
- The hot timer is now `terminal_summary_build_millis`.
- The retained prefix-cache shape is flat while summary-build time keeps
  climbing, so the next win must come before or inside summary construction.

## Current Comparison Targets

Use these as the numbers to beat:

- `runs/codex-claim-release-step4-delayed-summary-v1`
  - early step-`4` probe on the kept patch
- `runs/codex-claim-release-full-delayed-summary-v1`
  - `prefix_states_explored = 43` at `3309.4s`
  - `prefix_states_explored = 52` by `3936.1s`
  - `prefix_states_explored = 59` at `4529.4s`
  - `terminal_summary_build_millis = 4387822`
  - `terminal_materialize_millis = 527`
  - frontier queue length `= 2716`
  - legality summaries `= 279487`
  - retained prefix cache `= 39 groups / 144845 candidates`

## Do This Next

### 1. Patch One Narrow Remaining-One Fast Path

Acceptable targets:

- make terminal-summary construction itself cheaper
- avoid full summary build for clearly incumbent-dominated surfaces
- reuse summary-side work that is currently rebuilt per surviving surface

Reject as first moves:

- partial exact-two split handoff
- memory compaction
- widening-band retuning
- frontier queue rewrites
- compare, benchmark, or certification work

### 2. Re-Earn One Stored Release `until_step = 4` Rerun

Run the same claim profile shape derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `until_step = 4`
- release build
- live checkpoint persistence left on

Read the stored artifacts, not terminal output.

### 3. Decide Keep Or Drop From Stored Telemetry

Keep the patch only if the new `reports/steps/step-04-live.ndjson` shows at
least one of these:

- lower `terminal_summary_build_millis` at matching checkpoints
- the same wall clock reaches materially farther
- earlier pre-summary or pre-materialization pruning
- faster frontier drainage without weakening retained prefix-cache shape
- slower legality-summary growth without weakening retained prefix-cache shape

Drop the patch if the rerun does not show one of those.

### 4. Only Then Rerun The Real Profile

Only after the short step-`4` rerun earns keep:

- rerun `desktop_claim_shadow_1h`
- inspect the stored full-profile artifacts
- decide whether step `4` still blocks or a later cost center has taken over

## What To Read After The Rerun

Open `reports/steps/step-04-live.ndjson` and answer:

- did `terminal_summary_build_millis` fall at matching checkpoints?
- did the same wall clock reach farther than the current baseline?
- did frontier queue length drop faster?
- did legality summaries grow more slowly?
- did retained prefix-cache shape stay honest?
- did the hot cost move somewhere new?

## Regression Set

Run these after each search-code change:

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

Rewrite this file as soon as one new stored rerun shows one of these is true:

- summary-build cost is no longer the next win
- step `4` is no longer the dominant blocker
- or a different cost center has honestly overtaken summary build
