# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
and the one-pass `structural_nu` summary-build fast path are already landed.
The current full-profile baseline is
`runs/codex-claim-release-full-nu-profile-v1`.

## Goal

Cut remaining-one exact-summary work in step `4` by proving more prefixes dead
in `O(1)` structural time before
`compute_terminal_prefix_completion_summary_from_candidates` runs.

## Why This Is The Next Slice

- The lane has already crossed the old memory wall and is now compute-bound.
- The hot timer is still `terminal_summary_build_millis`.
- `remaining_one_rank_prunes_pre_materialize` is already very high, which
  means the pruning logic is working but still firing too late.
- Materialization time is now small relative to summary build time.
- The retained prefix-cache shape stays flat while legality summaries and exact
  summary work keep growing, so the next win must happen before exact summary
  construction.

## Current Comparison Targets

Use these as the numbers to beat:

- `runs/codex-claim-release-step4-nu-profile-v1`
  - `prefix_states_explored = 43` at `1728.7s`
  - `prefix_states_explored = 52` at `2090.3s`
  - `prefix_states_explored = 59` at `2360.5s`
  - `terminal_summary_build_millis = 2215871` at the `59`-state checkpoint
  - retained prefix cache stayed `39 groups / 144845 candidates`
  - legality summaries stayed `205199`, `246986`, and `279487`
- `runs/codex-claim-release-full-nu-profile-v1`
  - `prefix_states_explored = 43` at `1629.6s`
  - `prefix_states_explored = 52` at `2039.7s`
  - `prefix_states_explored = 59` at `2367.7s`
  - frontier queue length `= 2716`
  - legality summaries `= 279487`
  - retained prefix cache `= 39 groups / 144845 candidates`
  - `remaining_one_rank_prunes_pre_materialize = 273957`
  - `terminal_summary_build_millis = 2221499`
  - `terminal_materialize_millis = 460`
  - observed RSS `~ 316 MiB`

## Do This Next

### 1. Patch One Narrow `O(1)` Structural Pre-Summary Kill Switch

Preferred order:

- algebraic `nu` ceiling for remaining-one prefixes
- deterministic symmetry breaking for independent sibling clauses
- if still narrow and exact, structural-unity forced-bridge pruning

The first patch should be the algebraic `nu` ceiling unless the code says it
cannot be expressed honestly from the existing admissibility caps.

Reject as first moves:

- context-equivalence quotienting
- incumbent-dive queue retuning
- memory compaction
- widening-band retuning
- frontier queue rewrites
- compare, benchmark, or certification work

### 2. Add Telemetry And Tests For The New Prune Surface

Before trusting live runs, make the new slice measurable.

Preferred telemetry additions:

- `remaining_one_algebraic_prunes`
- `remaining_one_symmetry_prunes`
- `remaining_one_structural_unity_prunes`

Required tests:

- one targeted test for each new exact prune rule
- one regression that proves the kept winner still survives

### 3. Re-Earn One Stored Release `until_step = 4` Rerun

Run the same claim profile shape derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `until_step = 4`
- release build
- live checkpoint persistence left on

Read the stored artifacts, not terminal output.

### 4. Decide Keep Or Drop From Stored Telemetry

Keep the patch only if the new `reports/steps/step-04-live.ndjson` shows at
least one of these:

- lower `terminal_summary_build_millis` at matching checkpoints
- fewer legality summaries at matching retained-prefix shape
- materially more `O(1)` pre-summary prune hits
- the same wall clock reaches materially farther
- faster frontier drainage without weakening retained prefix-cache shape

Drop the patch if the rerun does not show one of those.

### 5. Only Then Rerun The Real Profile

Only after the short step-`4` rerun earns keep:

- rerun `desktop_claim_shadow_1h`
- inspect the stored full-profile artifacts
- decide whether step `4` still blocks or whether the next move should be the
  larger context-equivalence quotienting slice

## What To Read After The Rerun

Open `reports/steps/step-04-live.ndjson` and answer:

- did `terminal_summary_build_millis` fall at matching checkpoints?
- did legality summaries grow more slowly at the same retained-prefix shape?
- did the new structural prune counters fire materially?
- did the same wall clock reach farther than the current baseline?
- did frontier queue length drop faster?
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

Add the new targeted prune tests before trusting live runs.

Do not block on `cargo test -p pen-search --lib` being fully green right now;
the broader tree still stops at
`engine::tests::demo_late_surface_carries_through_live_config_runs`.

## Stop Condition For This Note

Rewrite this file as soon as one new stored rerun shows one of these is true:

- exact summary build is no longer the next honest win
- step `4` is no longer the dominant blocker
- the algebraic/symmetry slice is exhausted and the next real move is the
  larger context-equivalence quotienting patch
