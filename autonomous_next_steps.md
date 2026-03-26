# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, and the algebraic `nu`
ceiling patch are already landed.
Assume the attempted remaining-one context-equivalence quotient, the
frontier-pop incumbent-ordering experiment, the exact-two-step local ordering
experiment, the proof-close handoff ordering experiment, the broad
post-plateau discovery summary-skip experiment, and the narrower post-plateau
materialize-side one-clause gate experiment were all implemented, measured on
stored short reruns, and then dropped from code after their stored evidence
failed to earn keep.
The current short step-`4` baseline remains
`runs/codex-claim-release-step4-algebraic-v1`.
The most recent short evidence that did not advance that baseline is
`runs/codex-claim-release-step4-post-plateau-materialize-v1`.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.

## Goal

Cut post-plateau `terminal_summary_build_millis` on the retained remaining-one
surface without reopening either the broad discovery-side summary-skip surface
or another direct-compact materialize-side gate that does not actually engage
on the live claim lane.

## Why This Is The Next Slice

- On `runs/codex-claim-release-step4-post-plateau-v1`, the broad
  post-plateau summary-skip surface did activate before proof-close and it did
  collapse stored summary-build time, but it failed honestly because it moved
  the wall into direct compact materialization:
  - `terminal_summary_build_millis = 67319` instead of `186848` at `5`
  - `terminal_materialize_millis = 261792` instead of `104` at `5`
  - `remaining_one_materialized = 14032` instead of `15` at `5`
  - wall-clock progress then fell materially behind the algebraic baseline
- The follow-up narrow materialize-side rerun,
  `runs/codex-claim-release-step4-post-plateau-materialize-v1`, then showed
  that the intended retained-surface direct-compact gate still does not
  actually engage on the live claim lane:
  - `prefix_states_explored = 1` at `36.3s` instead of `35.7s`
  - `prefix_states_explored = 5` at `202.0s` instead of `198.5s`
  - `prefix_states_explored = 9` at `337.2s` instead of `332.5s`
  - retained prefix cache still stayed `13/32520`, `15/38108`, and `19/53693`
  - `terminal_summary_build_millis` still rose to
    `33516`, `189616`, and `314814`
  - `terminal_materialize_millis` stayed small at `85`, `103`, and `155`
  - but `post_plateau_materialize_fast_path_kept = 0` and
    `post_plateau_materialized_compact_direct = 0` at those same checkpoints,
    while `post_plateau_materialize_fast_path_skipped` only rose to
    `4630`, `23204`, and `41776`
- That means the honest post-plateau wall is still summary-side exact
  completion work on the retained remaining-one surface after the retained
  prefix cache has already flattened, not a materialize-side direct-compact
  reopening.

## Current Comparison Targets

Use these as the numbers to beat:

- `runs/codex-claim-release-step4-algebraic-v1`
  - `prefix_states_explored = 5` at `198.5s`
  - `prefix_states_explored = 43` at `1629.3s`
  - `prefix_states_explored = 52` at `1975.0s`
  - `prefix_states_explored = 59` at `2252.6s`
  - retained prefix cache stayed `39 groups / 144845 candidates` at `43/52/59`
  - `terminal_summary_build_millis = 186848` at `5`
  - `terminal_summary_build_millis = 2111246` at `59`
  - `terminal_materialize_millis = 104` at `5`, `466` at `43`, and `466` at
    `59`
  - `remaining_one_materialized = 15` at `5` and `39` at `43/52/59`
- `runs/codex-claim-release-step4-post-plateau-materialize-v1`
  - plateau-side telemetry observed the retained-surface flat zone
  - the intended direct-compact materialize counter still stayed `0`
  - the run was manually stopped at `prefix_states_explored = 9` / `337.2s`
    after enough stored non-engagement evidence
- `runs/codex-claim-release-step4-post-plateau-v1`
  - summary-build time collapsed
  - wall-clock progress became materially worse because the work moved into
    `terminal_materialize_millis`
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

### 1. Patch One Narrow Post-Plateau Summary-Side Experiment

Preferred form:

- target the retained remaining-one summary-build helpers after the retained
  prefix cache has already flattened at the current honest winner shape
- preserve the algebraic baseline's materialize behavior; do not reopen the
  broad discovery-side summary-skip surface or another non-engaging
  direct-compact gate
- use already-cached exact bound, admissibility, or rank evidence to avoid
  rebuilding compact terminal summaries that are not changing the honest
  retained surface
- keep deterministic ordering, retained-prefix honesty, and winner determinism
- keep the write surface inside the claim remaining-one summary-build / cached
  rank helpers first, not inside proof-close or the broad frontier loop

Reject as first moves:

- more post-plateau direct-compact materialize gates
- more blind post-plateau summary-skip variants
- more proof-close handoff variants
- more prepared exact-two-step local ordering variants
- more frontier-pop ordering variants
- more context-equivalence variants
- memory compaction
- certification or benchmark work
- widening-band retuning
- a broad frontier rewrite outside the narrow remaining-one summary surface

### 2. Add Telemetry And Tests For The Summary-Side Post-Plateau Surface

Before trusting live runs, make the new slice measurable and prove it engages
before proof-close.

Preferred telemetry additions:

- one counter for post-plateau retained-surface summary reuses or rebuild
  elisions on that surface
- one earliest-prefix-state marker for when that summary-side fast path first
  engages after the retained cache is already flat
- one counter pair for kept-vs-skipped post-plateau summary rebuilds on that
  surface

Required tests:

- one targeted test that the new post-plateau summary-side fast path does not
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

- the new summary-side telemetry activates before proof-close
- at least one real step-`4` win against the current baselines:
  - wall clock reaches materially farther at matching checkpoints
  - `terminal_summary_build_millis` falls
  - `terminal_materialize_millis` stays near the algebraic baseline instead of
    reopening the old blowup
  - frontier queue length drains faster without weakening retained prefix-cache
    shape
  - cached rank-prune growth or retained-group collapse improves at the same
    honest retained-prefix shape

Drop the patch if the activation counters stay zero or the rerun only adds
telemetry overhead without improving real step-`4` progress.

### 5. Only Then Rerun The Real Profile

Only after the short step-`4` rerun earns keep:

- rerun `desktop_claim_shadow_1h`
- inspect the stored full-profile artifacts
- decide whether step `4` still blocks or whether a later phase becomes the
  honest next wall

## What To Read After The Rerun

Open `reports/steps/step-04-live.ndjson` and answer:

- did the new summary-side telemetry activate before proof-close?
- if it activated, how early did the retained prefix cache flatten?
- did `terminal_summary_build_millis` fall at matching checkpoints?
- did `terminal_materialize_millis` stay controlled?
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
- the refined post-plateau summary-side slice is exhausted and the next real
  move is a different honest wall
- the real full profile exposes a later honest wall
