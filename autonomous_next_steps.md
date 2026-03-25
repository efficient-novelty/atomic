# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, and the algebraic `nu`
ceiling patch are already landed.
Assume the attempted remaining-one context-equivalence quotient was tried,
measured on a stored short rerun, and then dropped from code after it failed
to earn keep.
The current short step-`4` baseline remains
`runs/codex-claim-release-step4-algebraic-v1`.
The most recent short evidence that did not advance that baseline is
`runs/codex-claim-release-step4-context-equivalence-v1`.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.

## Goal

Bring the same honest winner to the front earlier in step `4` so the
incumbent rank rises sooner and more remaining-one exact-summary work gets
pruned before `compute_terminal_prefix_completion_summary_from_candidates`
dominates the wall clock.

## Why This Is The Next Slice

- The context-equivalence rerun kept the same honest queue, legality, and
  retained-prefix shape at the comparable `1/5/10` checkpoints, but it was
  slower than the current short baseline at each one.
- On `runs/codex-claim-release-step4-context-equivalence-v1`,
  `remaining_one_context_equivalence_hits` and
  `remaining_one_context_equivalence_reused_summaries` both stayed `0`, while
  `remaining_one_context_equivalence_misses` jumped to `2775` at the first
  stored frontier-progress checkpoint and then stayed flat through the manual
  stop.
- At the matching early checkpoints, `terminal_summary_build_millis` rose to
  `34532`, `191121`, and `362946` instead of `32990`, `186848`, and `355887`.
- The quotient slice is therefore exhausted on stored evidence; the hot cost is
  still remaining-one exact summary build, but another cache-key variant is not
  the next honest move.
- The next honest experiment is earlier incumbent arrival or a narrow
  deterministic queue-order bias, not more memory work, more quotienting, or
  another blind rerun on the same ordering surface.

## Current Comparison Targets

Use these as the numbers to beat:

- `runs/codex-claim-release-step4-algebraic-v1`
  - `prefix_states_explored = 43` at `1629.3s`
  - `prefix_states_explored = 52` at `1975.0s`
  - `prefix_states_explored = 59` at `2252.6s`
  - `terminal_summary_build_millis = 2111246` at the `59`-state checkpoint
  - retained prefix cache stayed `39 groups / 144845 candidates`
  - legality summaries stayed `205199`, `246986`, and `279487`
- `runs/codex-claim-release-step4-context-equivalence-v1`
  - `prefix_states_explored = 1` at `37.4s` instead of `35.7s`
  - `prefix_states_explored = 5` at `204.7s` instead of `198.5s`
  - `prefix_states_explored = 10` at `389.8s` instead of `380.4s`
  - queue length stayed `2774`, `2770`, and `2765`
  - legality summaries stayed `10193`, `28765`, and `51980`
  - retained prefix cache stayed `13/32520`, `15/38108`, and `19/53693`
  - `remaining_one_context_equivalence_hits = 0`
  - `remaining_one_context_equivalence_reused_summaries = 0`
  - `remaining_one_context_equivalence_misses = 2775` from the first
    frontier-progress checkpoint through the manual stop
  - manual stop at `prefix_states_explored = 13` / `507.6s`
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

### 1. Patch One Narrow Deterministic Incumbent-Arrival Experiment

Preferred form:

- bias step-`4` frontier pop order toward near-terminal claim prefixes that can
  raise the incumbent sooner using already-cached exact or primary-rank
  evidence
- keep deterministic ordering inside equal-priority buckets
- preserve retained-prefix honesty and winner determinism
- keep the write surface inside the step-`4` ordering path first

Reject as first moves:

- more context-equivalence variants
- memory compaction
- certification or benchmark work
- widening-band retuning
- a broad frontier rewrite outside the narrow step-`4` ordering surface

### 2. Add Telemetry And Tests For The Ordering Surface

Before trusting live runs, make the new slice measurable.

Preferred telemetry additions:

- `remaining_one_incumbent_priority_promotions`
- `remaining_one_incumbent_improving_groups_seen`
- `remaining_one_incumbent_improvement_prefix_state`

Required tests:

- one targeted test that a remaining-one claim work item with better cached
  coarse accept potential is popped before a weaker sibling
- one targeted deterministic-tie test that equal ordering buckets still pop in
  stable exact order
- one regression that proves the kept reference step-`4` winner is not delayed
  behind a worse cached item

### 3. Re-Earn One Stored Release `until_step = 4` Rerun

Run the same claim profile shape derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `until_step = 4`
- release build
- live checkpoint persistence left on

Read the stored artifacts, not terminal output.

### 4. Decide Keep Or Drop From Stored Telemetry

Keep the ordering patch only if the new `reports/steps/step-04-live.ndjson`
shows at least one of these against the current baselines:

- the same honest winner arrives materially earlier
- `remaining_one_rank_prunes_pre_materialize` grows materially faster at the
  comparable retained-prefix shape
- `terminal_summary_build_millis` falls at matching checkpoints
- the same wall clock reaches materially farther
- frontier queue length drains faster without weakening retained prefix-cache
  shape

Drop the patch if the rerun does not show one of those.

### 5. Only Then Rerun The Real Profile

Only after the short step-`4` rerun earns keep:

- rerun `desktop_claim_shadow_1h`
- inspect the stored full-profile artifacts
- decide whether step `4` still blocks or whether a later phase becomes the
  honest next wall

## What To Read After The Rerun

Open `reports/steps/step-04-live.ndjson` and answer:

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
- the incumbent-arrival slice is exhausted and the next real move is a later
  exact-screening or proof-close surface
- the real full profile exposes a later honest wall
