# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, and the algebraic `nu`
ceiling patch are already landed.
Assume the attempted remaining-one context-equivalence quotient, the
frontier-pop incumbent-ordering experiment, the exact-two-step local ordering
experiment, the proof-close handoff ordering experiment, the broad
post-plateau discovery summary-skip experiment, the narrower post-plateau
materialize-side one-clause gate experiment, and the later post-plateau
summary-cache reuse experiment were all implemented, measured on stored short
reruns, and then dropped from code after their stored evidence failed to earn
keep.
The current short step-`4` baseline remains
`runs/codex-claim-release-step4-algebraic-v1`.
The most recent short evidence that did not advance that baseline is
`runs/codex-claim-release-step4-post-plateau-summary-cache-v3`.
Ignore the two earlier attempts at that same run-id family,
`runs/codex-claim-release-step4-post-plateau-summary-cache-v1` and
`runs/codex-claim-release-step4-post-plateau-summary-cache-v2`: those local
reruns were stopped once their stored checkpoints showed the plateau gate
activating before the honest retained `39 groups / 144845 candidates` flat
zone.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.

## Goal

Cut remaining-one `terminal_summary_build_millis` on the honest retained
`39/144845` plateau surface by making each exact summary build cheaper, not by
retaining whole compact pruning summaries across prefixes.

## Why This Is The Next Slice

- The later stored summary-cache rerun,
  `runs/codex-claim-release-step4-post-plateau-summary-cache-v3`, did prove
  that the post-plateau trigger can engage on the honest retained surface:
  - `post_plateau_summary_first_activation_prefix_state = 24`
  - retained prefix cache had reached `39 groups / 144845 candidates`
- But that rerun also proved the hoped-for live duplicate-summary reuse is not
  the honest wall:
  - `post_plateau_summary_rebuild_elisions = 0` at `24`, `43`, and `44`
  - `post_plateau_summary_rebuilds_kept` still rose to
    `4353`, `92589`, and `97233`
  - live `terminal_prefix_completions` rose with it to
    `4353`, `92589`, and `97233`
- The wall-clock and summary-build cost then moved the wrong direction while
  the honest retained shape stayed the same:
  - `prefix_states_explored = 43` at `1708.3s` instead of `1629.3s`
  - frontier queue length still stayed `2732`
  - legality summaries still stayed `205199`
  - retained prefix cache still stayed `39 groups / 144845 candidates`
  - `terminal_summary_build_millis = 1599435` instead of `1524266`
  - `terminal_materialize_millis = 481` instead of `466`
- So the exhausted idea is now specific and narrow:
  exact-prefix retained-summary caching does not buy real reuse on the live
  claim lane. The honest step-`4` wall is still the per-prefix exact
  completion builder itself on the retained plateau surface.

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
- `runs/codex-claim-release-step4-post-plateau-summary-cache-v3`
  - first honest plateau activation at `24`
  - `post_plateau_summary_rebuild_elisions` still stayed `0`
  - `terminal_summary_build_millis` and wall clock both worsened while
    retained shape stayed honest
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

### 1. Patch One Narrow Per-Summary Builder Throughput Experiment

Preferred form:

- target `compute_terminal_prefix_completion_summary_from_candidates` and the
  claim remaining-one exact helper path immediately around it
- cut the cost of each exact summary build on the retained `39/144845` surface
  without retaining compact pruning summaries across prefixes
- preserve the algebraic baseline's materialize behavior; do not reopen the
  broad discovery-side summary-skip surface or another direct-compact
  materialize gate
- prefer a cut that attacks per-candidate exact work directly:
  - cheaper bound/rank aggregation bookkeeping
  - less repeated temporary object churn inside the compact summary loop
  - narrower exact-`nu` work after already-known connectivity/admissibility
    screens
- keep deterministic ordering, retained-prefix honesty, and winner determinism
- keep the write surface inside the claim remaining-one summary-build helpers
  first, not inside proof-close or the broad frontier loop

Reject as first moves:

- more post-plateau summary-cache retention or reuse variants
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

### 2. Add Telemetry And Tests For The Inner Summary-Builder Surface

Before trusting live runs, make the next slice measurable.

Preferred telemetry additions:

- split `terminal_summary_build_millis` enough to show whether the win lands in
  exact-`nu` work or in the surrounding non-`nu` summary bookkeeping
- one counter or earliest-prefix-state marker that proves the new inner-loop
  cut engages on the retained `39/144845` plateau surface
- do not keep the dropped post-plateau summary-cache counters in code unless
  the new slice genuinely still needs them

Required tests:

- one targeted regression that the kept reference step-`4` winner still
  survives on the retained plateau surface
- one targeted deterministic-order test if the new patch touches candidate
  ordering or tie handling
- one targeted test that the new helper cut preserves exact admitted/pruned
  counts for a representative remaining-one prefix

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

- the new inner summary-builder telemetry activates on the honest retained
  plateau surface
- at least one real step-`4` win against the current baselines:
  - wall clock reaches materially farther at matching checkpoints
  - `terminal_summary_build_millis` falls
  - `terminal_materialize_millis` stays near the algebraic baseline instead of
    reopening the old blowup
  - frontier queue length drains faster without weakening retained prefix-cache
    shape
  - cached rank-prune growth or retained-group collapse improves at the same
    honest retained-prefix shape

Drop the patch if the rerun only adds telemetry overhead, only grows resident
summary/cache state, or still leaves the same `43/52/59` checkpoints slower
than the algebraic baseline.

### 5. Only Then Rerun The Real Profile

Only after the short step-`4` rerun earns keep:

- rerun `desktop_claim_shadow_1h`
- inspect the stored full-profile artifacts
- decide whether step `4` still blocks or whether a later phase becomes the
  honest next wall

## What To Read After The Rerun

Open `reports/steps/step-04-live.ndjson` and answer:

- did the new inner-loop telemetry activate on the retained `39/144845`
  plateau surface?
- did `terminal_summary_build_millis` fall at matching `43/52/59` checkpoints?
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
- the refined per-summary builder slice is exhausted and the next real move is
  a different honest wall
- the real full profile exposes a later honest wall
