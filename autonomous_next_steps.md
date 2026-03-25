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
The current short step-`4` baseline remains
`runs/codex-claim-release-step4-algebraic-v1`.
The most recent short evidence that did not advance that baseline is
`runs/codex-claim-release-step4-incumbent-ordering-v1`.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.

## Goal

Move the hot step-`4` cost inside the in-place exact-two-step remaining-one
screening surface, because the attempted frontier-pop ordering bias never
engaged and `compute_terminal_prefix_completion_summary_from_candidates`
still dominates the wall clock.

## Why This Is The Next Slice

- The stored frontier-order rerun kept the same honest queue, legality, retained
  prefix-cache shape, and cached rank-prune counts at the comparable
  `1/5/10/43/52/59` checkpoints, but it was slower than the current short
  baseline at every one of them.
- On `runs/codex-claim-release-step4-incumbent-ordering-v1`, the comparable
  checkpoint times moved the wrong direction:
  - `prefix_states_explored = 1` at `39.3s` instead of `35.7s`
  - `prefix_states_explored = 5` at `223.5s` instead of `198.5s`
  - `prefix_states_explored = 10` at `427.3s` instead of `380.4s`
  - `prefix_states_explored = 43` at `1835.9s` instead of `1629.3s`
  - `prefix_states_explored = 52` at `2224.0s` instead of `1975.0s`
  - `prefix_states_explored = 59` at `2519.5s` instead of `2252.6s`
- At those same checkpoints, frontier queue length, legality summaries,
  retained prefix-cache shape, and `remaining_one_cached_rank_prunes` matched
  the baseline exactly, which means the rerun did not surface the winner or the
  prune story any earlier.
- At those same checkpoints, `terminal_summary_build_millis` rose to
  `36947`, `213448`, `407816`, `1748516`, `2119645`, and `2402376` instead of
  `32990`, `186848`, `355887`, `1524266`, `1849248`, and `2111246`.
- Through the manual stop at `prefix_states_explored = 161` / `7221.1s`,
  the new frontier-order telemetry stayed completely inactive:
  - `remaining_one_prefixes_seen = 0`
  - `remaining_one_incumbent_priority_promotions = 0`
  - `remaining_one_incumbent_improving_groups_seen = 0`
  - `remaining_one_incumbent_improvement_prefix_state = 0`
- That same manual-stop checkpoint still showed the hot cost sitting in the
  same place:
  - frontier queue length `= 2614`
  - legality summaries `= 753073`
  - retained prefix cache `= 41 groups / 154842 candidates`
  - `remaining_one_cached_rank_prunes = 747643`
  - `terminal_summary_build_millis = 6883748`
  - observed RSS `~ 610.7 MiB`
- The frontier-pop incumbent-arrival slice is therefore exhausted on stored
  evidence. The next honest move is inside the prepared exact-two-step
  remaining-one surface or its proof-close handoff, not another frontier-order
  variant, more context-key work, more memory work, or a blind full-profile
  rerun.

## Current Comparison Targets

Use these as the numbers to beat:

- `runs/codex-claim-release-step4-algebraic-v1`
  - `prefix_states_explored = 43` at `1629.3s`
  - `prefix_states_explored = 52` at `1975.0s`
  - `prefix_states_explored = 59` at `2252.6s`
  - `terminal_summary_build_millis = 2111246` at the `59`-state checkpoint
  - retained prefix cache stayed `39 groups / 144845 candidates`
  - legality summaries stayed `205199`, `246986`, and `279487`
  - `remaining_one_cached_rank_prunes` stayed `199653`, `241449`, and `273957`
- `runs/codex-claim-release-step4-incumbent-ordering-v1`
  - `prefix_states_explored = 1` at `39.3s` instead of `35.7s`
  - `prefix_states_explored = 5` at `223.5s` instead of `198.5s`
  - `prefix_states_explored = 10` at `427.3s` instead of `380.4s`
  - `prefix_states_explored = 43` at `1835.9s` instead of `1629.3s`
  - `prefix_states_explored = 52` at `2224.0s` instead of `1975.0s`
  - `prefix_states_explored = 59` at `2519.5s` instead of `2252.6s`
  - frontier queue length stayed `2774`, `2770`, `2765`, `2732`, `2723`,
    and `2716`
  - legality summaries stayed `10193`, `28765`, `51980`, `205199`, `246986`,
    and `279487`
  - retained prefix cache stayed `13/32520`, `15/38108`, `19/53693`,
    `39/144845`, `39/144845`, and `39/144845`
  - `remaining_one_cached_rank_prunes` stayed `4631`, `23205`, `46421`,
    `199653`, `241449`, and `273957`
  - `terminal_summary_build_millis` rose to `36947`, `213448`, `407816`,
    `1748516`, `2119645`, and `2402376`
  - the frontier-order telemetry stayed all-zero through the manual stop
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

### 1. Patch One Narrow Deterministic Exact-Two-Step Screening Experiment

Preferred form:

- bias the in-place prepared exact-two-step terminal-prefix surface toward
  prefixes that can raise the incumbent sooner or collapse faster using
  already-cached exact or primary-rank evidence
- if the prepared surface never exposes that evidence early enough, bias the
  first proof-close handoff on the same narrow deterministic evidence instead
- keep deterministic ordering inside equal-priority buckets
- preserve retained-prefix honesty and winner determinism
- keep the write surface inside
  `prepare_exact_two_step_terminal_surface`,
  `process_prepared_exact_two_step_terminal_surface`, or the immediate
  proof-close handoff first

Reject as first moves:

- more frontier-pop ordering variants
- more context-equivalence variants
- memory compaction
- certification or benchmark work
- widening-band retuning
- a broad frontier rewrite outside the narrow exact-two-step or proof-close
  surface

### 2. Add Telemetry And Tests For The Local Exact-Two-Step Surface

Before trusting live runs, make the new slice measurable and prove it actually
activates on the live claim lane.

Preferred telemetry additions:

- one counter for local exact-two-step incumbent-priority promotions
- one counter for local exact-two-step improving prefixes surfaced
- one earliest-prefix-state marker for when that local surface first activates

Required tests:

- one targeted test that a prepared remaining-one claim prefix with better
  cached coarse accept potential is processed before a weaker sibling
- one targeted deterministic-tie test that equal local ordering buckets still
  process in stable exact order
- one regression that proves the kept reference step-`4` winner is not delayed
  behind a worse cached prepared prefix

### 3. Re-Earn One Stored Release `until_step = 4` Rerun

Run the same claim profile shape derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `until_step = 4`
- release build
- live checkpoint persistence left on

Read the stored artifacts, not terminal output.

### 4. Decide Keep Or Drop From Stored Telemetry

Keep the exact-two-step patch only if the new
`reports/steps/step-04-live.ndjson` shows both of these:

- the new local-surface telemetry actually activates on the stored rerun
- at least one real step-`4` win against the current baselines:
  - the same honest winner arrives materially earlier
  - `remaining_one_rank_prunes_pre_materialize` grows materially faster at the
    comparable retained-prefix shape
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

- did the new exact-two-step telemetry activate at all?
- if it activated, how early did it first activate?
- did the same honest winner arrive earlier?
- did `remaining_one_rank_prunes_pre_materialize` climb faster?
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
- the exact-two-step or proof-close slice is exhausted and the next real move is
  a later honest wall
- the real full profile exposes a later honest wall
