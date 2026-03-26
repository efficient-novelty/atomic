# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, and the algebraic `nu`
ceiling patch are already landed.
Assume the attempted remaining-one context-equivalence quotient, the frontier-
pop incumbent-ordering experiment, the exact-two-step local ordering
experiment, and the later proof-close handoff ordering experiment were all
implemented, measured on stored short reruns, and then dropped from code after
their stored evidence failed to earn keep.
The current short step-`4` baseline remains
`runs/codex-claim-release-step4-algebraic-v1`.
The most recent short evidence that did not advance that baseline is
`runs/codex-claim-release-step4-proof-close-handoff-v1`.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.

## Goal

Cut discovery-side `terminal_summary_build_millis` after the retained prefix
cache has already flattened, because the proof-close handoff surface never
activates before the hot stored step-`4` checkpoints.

## Why This Is The Next Slice

- On `runs/codex-claim-release-step4-proof-close-handoff-v1`, the new
  proof-close telemetry stayed pinned at zero at every stored checkpoint:
  - `proof_close_handoff_first_activation_prefix_state = 0`
  - `proof_close_handoff_improving_groups_surfaced = 0`
  - `proof_close_handoff_incumbent_priority_promotions = 0`
- The comparable hot checkpoints therefore all still happened in discovery,
  not proof-close:
  - `prefix_states_explored = 1` at `36.5s`
  - `prefix_states_explored = 5` at `201.8s`
  - `prefix_states_explored = 43` at `1623.8s`
  - `prefix_states_explored = 52` at `1968.1s`
  - `prefix_states_explored = 59` at `2230.4s`
- At those same checkpoints, the honest retained-prefix shape stayed exactly on
  the algebraic baseline:
  - frontier queue length `= 2774`, `2770`, `2732`, `2723`, and `2716`
  - legality summaries `= 10193`, `28765`, `205199`, `246986`, and `279487`
  - retained prefix cache `= 13/32520`, `15/38108`, and then `39/144845`
  - `remaining_one_cached_rank_prunes = 4631`, `23205`, `199653`, `241449`,
    and `273957`
  - `remaining_one_materialized = 13`, `15`, `39`, `39`, and `39`
- The measured summary-build time moved only slightly at those same checkpoints:
  - `terminal_summary_build_millis = 33691` instead of `32990`
  - `terminal_summary_build_millis = 189133` instead of `186848`
  - `terminal_summary_build_millis = 1516022` instead of `1524266`
  - `terminal_summary_build_millis = 1839230` instead of `1849248`
  - `terminal_summary_build_millis = 2084626` instead of `2111246`
- The rerun was then manually stopped at `prefix_states_explored = 136` /
  `5388.3s` after enough stored keep/drop evidence:
  - frontier queue length `= 2639`
  - legality summaries `= 636998`
  - retained prefix cache `= 40 groups / 147639 candidates`
  - `remaining_one_cached_rank_prunes = 631544`
  - `terminal_summary_build_millis = 5061269`
  - proof-close telemetry still all zero
  - observed RSS `~ 530.2 MiB`
- The proof-close handoff surface therefore is not on the hot path that is
  still dominating step `4`. The next honest move must happen earlier, while
  discovery is still paying repeated exact-summary cost after the retained
  prefix cache has plateaued.

## Current Comparison Targets

Use these as the numbers to beat:

- `runs/codex-claim-release-step4-algebraic-v1`
  - `prefix_states_explored = 1` at `35.7s`
  - `prefix_states_explored = 5` at `198.5s`
  - `prefix_states_explored = 43` at `1629.3s`
  - `prefix_states_explored = 52` at `1975.0s`
  - `prefix_states_explored = 59` at `2252.6s`
  - retained prefix cache stayed `39 groups / 144845 candidates` at `43/52/59`
  - `terminal_summary_build_millis = 186848` at the `5`-state checkpoint
  - `terminal_summary_build_millis = 2111246` at the `59`-state checkpoint
- `runs/codex-claim-release-step4-proof-close-handoff-v1`
  - proof-close telemetry stayed `0/0/0` at every stored checkpoint
  - the same honest retained-prefix shape still held at `1`, `5`, `43`, `52`,
    and `59`
  - the run was still in discovery at `prefix_states_explored = 136` /
    `5388.3s`
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

### 1. Patch One Narrow Discovery-Side Post-Plateau Summary Experiment

Preferred form:

- target the discovery/materialize hot path after the retained prefix cache has
  already stopped growing at the current honest winner shape
- use already-cached structural bound or rank evidence to avoid rebuilding full
  exact terminal summaries on non-expanding groups
- keep deterministic ordering, retained-prefix honesty, and winner determinism
- keep the write surface inside the discovery/materialize remaining-one summary
  helpers first, not inside proof-close

Reject as first moves:

- more proof-close handoff variants
- more prepared exact-two-step local ordering variants
- more frontier-pop ordering variants
- more context-equivalence variants
- memory compaction
- certification or benchmark work
- widening-band retuning
- a broad frontier rewrite outside the narrow discovery-side summary surface

### 2. Add Telemetry And Tests For The Post-Plateau Discovery Surface

Before trusting live runs, make the new slice measurable and prove it engages
before proof-close.

Preferred telemetry additions:

- one counter for terminal summaries built after the retained group count last
  increased
- one earliest-prefix-state marker for when the retained prefix cache first
  flattens while legality summaries keep rising
- one counter for kept-vs-skipped post-plateau summary builds on that surface

Required tests:

- one targeted test that the new post-plateau fast path does not skip a group
  that can still beat the incumbent
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

- the new post-plateau discovery telemetry activates before proof-close
- at least one real step-`4` win against the current baselines:
  - `terminal_summary_build_millis` falls at matching checkpoints
  - the same wall clock reaches materially farther
  - frontier queue length drains faster without weakening retained prefix-cache
    shape
  - retained-group collapse or cached rank-prune growth improves at the same
    honest retained-prefix shape

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

- did the new discovery-side telemetry activate before proof-close?
- if it activated, how early did the retained prefix cache flatten?
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
- the post-plateau discovery slice is exhausted and the next real move is a
  later honest wall
- the real full profile exposes a later honest wall
