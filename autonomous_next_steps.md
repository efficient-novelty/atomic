# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, and the algebraic `nu`
ceiling patch are already landed.
Assume the attempted remaining-one context-equivalence quotient was tried,
measured on a stored short rerun, and then dropped from code after it failed
to earn keep.
Assume one additional narrow frontier-pop incumbent-arrival ordering experiment
was also implemented, measured on a stored short rerun, and then dropped from
code after the stored evidence showed that surface never activated on the live
claim lane.
Assume one further narrow claim-only exact-two-step local ordering experiment
was also implemented, measured on a stored short rerun, and then dropped from
code after the stored evidence showed that it activated immediately but still
regressed the hot `1/5` step-`4` checkpoints.
The current short step-`4` baseline remains
`runs/codex-claim-release-step4-algebraic-v1`.
The most recent short evidence that did not advance that baseline is
`runs/codex-claim-release-step4-local-two-step-order-v2`.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.

## Goal

Raise the honest step-`4` incumbent sooner at the first proof-close handoff on
the retained exact-two-step surface, because the prepared local ordering pass
did activate but still left `compute_terminal_prefix_completion_summary_from_candidates`
as the dominant wall-clock cost.

## Why This Is The Next Slice

- The stored exact-two-step local-order rerun was the first post-algebraic
  slice that actually activated on the live claim lane.
- On `runs/codex-claim-release-step4-local-two-step-order-v2`, the new local
  telemetry turned on immediately:
  - `local_exact_two_step_first_activation_prefix_state = 1`
  - `local_exact_two_step_improving_prefixes_surfaced = 4644` then `4653`
  - `local_exact_two_step_incumbent_priority_promotions = 4629` then `4638`
- But those same comparable checkpoints still moved the wrong direction:
  - `prefix_states_explored = 1` at `61.7s` instead of `35.7s`
  - `prefix_states_explored = 5` at `249.6s` instead of `198.5s`
- At those same checkpoints, frontier queue length and legality summaries still
  matched the baseline exactly:
  - frontier queue length `= 2774` and `2770`
  - legality summaries `= 10193` and `28765`
- At those same checkpoints, the retained prefix cache stayed honest but moved
  materially behind the baseline:
  - `1 group / 2794 candidates` instead of `13 / 32520`
  - `2 groups / 5588 candidates` instead of `15 / 38108`
- At those same checkpoints, `remaining_one_cached_rank_prunes` rose only
  slightly to `4643` and `23218` instead of `4631` and `23205`, while
  `remaining_one_materialized` fell to `1` and `2` instead of `13` and `15`.
- The hot cost stayed in the same place and got worse early:
  - `terminal_summary_build_millis = 58966` instead of `32990`
  - `terminal_summary_build_millis = 237791` instead of `186848`
- The rerun was then manually stopped at `prefix_states_explored = 5` /
  `249.6s` after enough stored keep/drop evidence:
  - frontier queue length `= 2770`
  - legality summaries `= 28765`
  - retained prefix cache `= 2 groups / 5588 candidates`
  - `remaining_one_cached_rank_prunes = 23218`
  - `terminal_summary_build_millis = 237791`
  - observed RSS `~ 46.8 MiB`
- The prepared exact-two-step local ordering surface therefore engaged but did
  not earn keep. The next honest move is the immediate proof-close handoff on
  the same cached exact or primary-rank evidence, not another prepared-surface
  ordering pass, another frontier-order variant, more context-key work, more
  memory work, or a blind full-profile rerun.

## Current Comparison Targets

Use these as the numbers to beat:

- `runs/codex-claim-release-step4-algebraic-v1`
  - `prefix_states_explored = 1` at `35.7s`
  - `prefix_states_explored = 5` at `198.5s`
  - `prefix_states_explored = 43` at `1629.3s`
  - `prefix_states_explored = 52` at `1975.0s`
  - `prefix_states_explored = 59` at `2252.6s`
  - `terminal_summary_build_millis = 186848` at the `5`-state checkpoint
  - `terminal_summary_build_millis = 2111246` at the `59`-state checkpoint
  - retained prefix cache stayed `15 groups / 38108 candidates` at `5`
  - retained prefix cache stayed `39 groups / 144845 candidates` at `43/52/59`
  - legality summaries stayed `28765`, `205199`, `246986`, and `279487`
  - `remaining_one_cached_rank_prunes` stayed `23205`, `199653`, `241449`,
    and `273957`
- `runs/codex-claim-release-step4-local-two-step-order-v2`
  - `prefix_states_explored = 1` at `61.7s` instead of `35.7s`
  - `prefix_states_explored = 5` at `249.6s` instead of `198.5s`
  - frontier queue length stayed `2774` and `2770`
  - legality summaries stayed `10193` and `28765`
  - retained prefix cache only reached `1/2794` and `2/5588`
  - `remaining_one_cached_rank_prunes` only rose to `4643` and `23218`
  - `remaining_one_materialized` fell to `1` and `2`
  - `terminal_summary_build_millis` rose to `58966` and `237791`
  - the local ordering telemetry activated immediately but still did not earn
    keep
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

### 1. Patch One Narrow Deterministic Proof-Close Handoff Experiment

Preferred form:

- bias the first proof-close handoff or immediate proof-close group selection
  toward retained groups that can raise the incumbent sooner or collapse faster
  using already-cached exact or primary-rank evidence
- keep deterministic ordering inside equal-priority buckets
- preserve retained-prefix honesty and winner determinism
- keep the write surface inside the immediate proof-close handoff and local
  proof-close group-order helpers first

Reject as first moves:

- more prepared exact-two-step local ordering variants
- more frontier-pop ordering variants
- more context-equivalence variants
- memory compaction
- certification or benchmark work
- widening-band retuning
- a broad frontier rewrite outside the narrow proof-close handoff surface

### 2. Add Telemetry And Tests For The Local Proof-Close Handoff Surface

Before trusting live runs, make the new slice measurable and prove it actually
activates on the live claim lane.

Preferred telemetry additions:

- one counter for proof-close handoff incumbent-priority promotions
- one counter for proof-close handoff improving groups surfaced
- one earliest-prefix-state marker for when that proof-close handoff first
  activates

Required tests:

- one targeted test that a retained claim proof-close group with better cached
  coarse accept potential is certified before a weaker sibling
- one targeted deterministic-tie test that equal proof-close ordering buckets
  still process in stable exact order
- one regression that proves the kept reference step-`4` winner is not delayed
  behind a worse cached retained group

### 3. Re-Earn One Stored Release `until_step = 4` Rerun

Run the same claim profile shape derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `until_step = 4`
- release build
- live checkpoint persistence left on

Read the stored artifacts, not terminal output.

### 4. Decide Keep Or Drop From Stored Telemetry

Keep the proof-close handoff patch only if the new
`reports/steps/step-04-live.ndjson` shows both of these:

- the new proof-close handoff telemetry actually activates on the stored rerun
- at least one real step-`4` win against the current baselines:
  - the same honest winner arrives materially earlier
  - `remaining_one_rank_prunes_pre_materialize` or retained-group collapse
    grows materially faster at the comparable retained-prefix shape
  - `terminal_summary_build_millis` falls at matching checkpoints
  - the same wall clock reaches materially farther
  - frontier queue length drains faster without weakening retained prefix-cache
    shape

Drop the patch if the activation counters stay zero or the rerun does not show
one of those wins.

### 5. Only Then Rerun The Real Profile

Only after the short step-`4` rerun earns keep:

- rerun `desktop_claim_shadow_1h`
- inspect the stored full-profile artifacts
- decide whether step `4` still blocks or whether a later phase becomes the
  honest next wall

## What To Read After The Rerun

Open `reports/steps/step-04-live.ndjson` and answer:

- did the new proof-close handoff telemetry activate at all?
- if it activated, how early did it first activate?
- did the same honest winner arrive earlier?
- did retained-group collapse or pre-materialize rank pruning climb faster?
- did `terminal_summary_build_millis` fall at matching checkpoints?
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
- the proof-close handoff slice is exhausted and the next real move is a later
  honest wall
- the real full profile exposes a later honest wall
