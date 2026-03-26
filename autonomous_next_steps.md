# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, and the algebraic `nu`
ceiling patch are already landed.
Assume the attempted remaining-one context-equivalence quotient, the
frontier-pop incumbent-ordering experiment, the exact-two-step local ordering
experiment, the proof-close handoff ordering experiment, and the broad
post-plateau discovery summary-skip experiment were all implemented, measured
on stored short reruns, and then dropped from code after their stored evidence
failed to earn keep.
The current short step-`4` baseline remains
`runs/codex-claim-release-step4-algebraic-v1`.
The most recent short evidence that did not advance that baseline is
`runs/codex-claim-release-step4-post-plateau-v1`.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.

## Goal

Cut post-plateau `terminal_materialize_millis` on the retained remaining-one
surface without reopening the broad discovery-side summary-skip surface that
already made the step-`4` wall worse.

## Why This Is The Next Slice

- On `runs/codex-claim-release-step4-post-plateau-v1`, the new post-plateau
  telemetry activated before proof-close exactly as intended:
  - `post_plateau_terminal_summary_builds = 0`
  - `post_plateau_terminal_summary_builds_skipped = 14018` at `5` states,
    `158608` at `43`, `200404` at `52`, and `232912` at `59`
  - `post_plateau_flatten_first_prefix_state = 25` on the first long plateau,
    then `75` after the later `40`th retained group landed
- At those same matching checkpoints, the retained-prefix shape stayed honest
  against the algebraic baseline:
  - frontier queue length `= 2774`, `2770`, `2732`, `2723`, and `2716`
  - legality summaries `= 10193`, `28765`, `205199`, `246986`, and `279487`
  - retained prefix cache `= 13/32520`, `15/38108`, and then `39/144845`
- The intended summary-side cut was real, but it did not help the real wall:
  - `terminal_summary_build_millis = 67319` instead of `186848` at `5`
  - `terminal_summary_build_millis = 317305` instead of `1524266` at `43`
  - `terminal_summary_build_millis = 317305` instead of `2111246` at `59`
- The saved summary work was immediately re-spent in the wrong place:
  - `terminal_materialize_millis = 261792` instead of `104` at `5`
  - `terminal_materialize_millis = 2591286` instead of `466` at `43`
  - `terminal_materialize_millis = 3797292` instead of `466` at `59`
  - `remaining_one_materialized = 14032`, `158639`, `200435`, and `232943`
    instead of `15`, `39`, `39`, and `39`
  - every one of those extra groups stayed on
    `remaining_one_materialized_compact_direct`
- Wall-clock progress therefore moved the wrong direction even though the new
  telemetry activated:
  - `prefix_states_explored = 5` at `354.4s` instead of `198.5s`
  - `prefix_states_explored = 43` at `3113.3s` instead of `1629.3s`
  - `prefix_states_explored = 52` at `3853.5s` instead of `1975.0s`
  - `prefix_states_explored = 59` at `4403.0s` instead of `2252.6s`
- The rerun was then manually stopped at `prefix_states_explored = 92` /
  `7187.0s` after enough stored evidence:
  - frontier queue length `= 2683`
  - legality summaries `= 432706`
  - retained prefix cache `= 40 groups / 147639 candidates`
  - `terminal_summary_build_millis = 352986`
  - `terminal_materialize_millis = 6367302`
  - `remaining_one_materialized = 381637`
  - `remaining_one_materialized_compact_direct = 381637`
  - observed RSS `~ 421.5 MiB`
- The broad post-plateau discovery summary-skip surface is therefore exhausted.
  The next honest move must stay narrower: preserve summary-side pruning, but
  stop reopening so many non-expanding post-plateau groups into direct compact
  materialization.

## Current Comparison Targets

Use these as the numbers to beat:

- `runs/codex-claim-release-step4-algebraic-v1`
  - `prefix_states_explored = 5` at `198.5s`
  - `prefix_states_explored = 43` at `1629.3s`
  - `prefix_states_explored = 52` at `1975.0s`
  - `prefix_states_explored = 59` at `2252.6s`
  - retained prefix cache stayed `39 groups / 144845 candidates` at `43/52/59`
  - `terminal_summary_build_millis = 186848` at the `5`-state checkpoint
  - `terminal_summary_build_millis = 2111246` at the `59`-state checkpoint
  - `terminal_materialize_millis = 104` at `5`, `466` at `43`, and `466` at
    `59`
  - `remaining_one_materialized = 15` at `5` and `39` at `43/52/59`
- `runs/codex-claim-release-step4-post-plateau-v1`
  - post-plateau telemetry activated and summary-build time collapsed
  - the same honest retained-prefix shape still held at `5`, `43`, `52`, and
    `59`
  - wall-clock progress became materially worse because the work moved into
    `terminal_materialize_millis`
  - the run was still in discovery at `prefix_states_explored = 92` /
    `7187.0s`
- `runs/codex-claim-release-full-nu-profile-v1`
  - `prefix_states_explored = 43` at `1629.6s`
  - `prefix_states_explored = 52` at `2039.7s`
  - `prefix_states_explored = 59` at `2367.7s`
  - frontier queue length `= 2716`
  - legality summaries `= 279487`
  - retained prefix cache `= 39 groups / 144845 candidates`
  - `remaining_one_rank_prunes_pre_materialize = 273957`
  - `terminal_summary_build_millis = 2221499`
  - observed RSS `~ 316 MiB`

## Do This Next

### 1. Patch One Narrow Post-Plateau Materialize-Side Experiment

Preferred form:

- target the retained remaining-one materialize helpers after the retained
  prefix cache has already flattened at the current honest winner shape
- preserve summary-side bound and rank pruning; do not reopen the broad
  discovery-side summary-skip surface
- use already-cached exact bound or rank evidence to avoid direct compact
  materialization of groups that are not expanding the honest retained surface
- keep deterministic ordering, retained-prefix honesty, and winner determinism
- keep the write surface inside the claim remaining-one materialize / cached
  rank helpers first, not inside proof-close or the broad frontier loop

Reject as first moves:

- more blind post-plateau summary-skip variants
- more proof-close handoff variants
- more prepared exact-two-step local ordering variants
- more frontier-pop ordering variants
- more context-equivalence variants
- memory compaction
- certification or benchmark work
- widening-band retuning
- a broad frontier rewrite outside the narrow remaining-one materialize surface

### 2. Add Telemetry And Tests For The Materialize-Side Post-Plateau Surface

Before trusting live runs, make the new slice measurable and prove it engages
before proof-close.

Preferred telemetry additions:

- one counter for post-plateau direct compact materializations on that surface
- one earliest-prefix-state marker for when that materialize-side fast path
  first engages after the retained cache is already flat
- one counter pair for kept-vs-skipped post-plateau materializations on that
  surface

Required tests:

- one targeted test that the new post-plateau materialize fast path does not
  skip a group that can still beat the incumbent
- one targeted deterministic-tie test that equal discovery-side buckets still
  process in stable exact order
- one regression that proves the kept reference step-`4` winner still survives
  when the retained cache is already flat

### 3. Re-Earn One Stored Release `until_step = 4` Rerun

Run the same claim profile shape derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `until_step = 4`
- release build
- live checkpoint persistence left on

Read the stored artifacts, not terminal output.

### 4. Decide Keep Or Drop From Stored Telemetry

Keep the patch only if the new `reports/steps/step-04-live.ndjson` shows both
of these:

- the new materialize-side telemetry activates before proof-close
- at least one real step-`4` win against the current baselines:
  - wall clock reaches materially farther at matching checkpoints
  - `terminal_materialize_millis` falls without giving back the old
    `terminal_summary_build_millis` win
  - frontier queue length drains faster without weakening retained prefix-cache
    shape
  - retained-group collapse or cached rank-prune growth improves at the same
    honest retained-prefix shape

Drop the patch if the activation counters stay zero or the rerun only swaps one
large cost for another without improving real step-`4` progress.

### 5. Only Then Rerun The Real Profile

Only after the short step-`4` rerun earns keep:

- rerun `desktop_claim_shadow_1h`
- inspect the stored full-profile artifacts
- decide whether step `4` still blocks or whether a later phase becomes the
  honest next wall

## What To Read After The Rerun

Open `reports/steps/step-04-live.ndjson` and answer:

- did the new materialize-side telemetry activate before proof-close?
- if it activated, how early did the retained prefix cache flatten?
- did `terminal_materialize_millis` fall at matching checkpoints?
- did `terminal_summary_build_millis` stay controlled instead of bouncing back?
- did frontier queue length drop faster?
- did the same wall clock reach farther than the current baseline?
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

Do not block on `cargo test -p pen-search --lib` being fully green right now;
the broader tree still stops at
`engine::tests::demo_late_surface_carries_through_live_config_runs`.

## Stop Condition For This Note

Rewrite this file as soon as one new stored rerun shows one of these is true:

- step `4` is no longer the dominant blocker
- the refined post-plateau remaining-one slice is exhausted and the next real
  move is a different honest wall
- the real full profile exposes a later honest wall
