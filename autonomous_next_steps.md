# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, the algebraic `nu`
ceiling patch, the family-agnostic claim terminal-admissibility shortcut, the
exact non-allocating connectivity summary scan, and the terminal-only cached
parent connectivity decision are already landed.
Assume the context-equivalence quotient, the frontier-pop ordering experiment,
the exact-two-step local ordering experiment, the proof-close handoff
experiment, the broad post-plateau summary-skip experiment, the narrower
post-plateau materialize-side gate, the post-plateau summary-cache reuse
experiment, the expr-keyed terminal clause hot-path profile cache experiment,
the clause-side terminal connectivity profile precompute experiment, and the
terminal-candidate tuple-remap prep experiment were all measured on stored
short reruns and then dropped from code after failing to earn keep.

The current short step-`4` baseline remains
`runs/codex-claim-release-step4-kernel-connectivity-v2`.
The previous short baseline remains
`runs/codex-claim-release-step4-kernel-connectivity-v1`.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.
The latest dropped short rerun is now
`runs/codex-claim-release-step4-terminal-candidate-prep-v1`.
The diagnostic kernel split that explained the last kept wall is still
`runs/codex-claim-release-step4-kernel-profile-v2`.

Ignore `runs/codex-claim-release-step4-kernel-profile-v1`: that local run used
the flawed per-candidate millisecond accumulation and is not valid evidence.

## Working Diagnosis

- The claim lane is still compute-bound in step `4`, and the honest live shape
  still repeats:
  - frontier queue stays on the same contour
  - legality growth stays on the same contour
  - retained prefix cache still flattens at
    `39 groups / 144845 candidates`
- The kept short baseline
  `runs/codex-claim-release-step4-kernel-connectivity-v2` still defines the
  surface to beat:
  - same honest retained plateau `= 39 groups / 144845 candidates`
  - `elapsed_millis = 551825/998555/1020529` at `24/43/44`
  - `terminal_summary_build_millis = 495256/901994/921924`
    at `24/43/44`
  - `terminal_summary_connectivity_millis = 95969/178000/182453`
    at `24/43/44`
  - `terminal_summary_aggregation_millis = 68266/119561/121524`
    at `24/43/44`
  - `terminal_summary_exact_nu_millis = 39523/73348/74489`
    at `24/43/44`
  - `terminal_materialize_millis = 327`
  - fallback connectivity stayed `0`
- The latest dropped rerun
  `runs/codex-claim-release-step4-terminal-candidate-prep-v1` then added one
  narrow candidate-preparation counter and removed the extra filtered-candidate
  remap before the first connectivity check.
- That rerun kept the same honest plateau:
  - retained prefix cache `= 39 groups / 144845 candidates` at `24/43/44`
  - `terminal_summary_first_plateau_activation_prefix_state = 24`
  - `terminal_summary_plateau_activations = 97234` at `44`
- It also exposed the previously hidden prep bucket on that plateau:
  - `terminal_summary_candidate_prep_millis = 32904/71577/73974`
    at `24/43/44`
- But matched against the kept short baseline, it still regressed:
  - at `prefix_states_explored = 24`:
    `elapsed_millis = 562457` instead of `551825`,
    `terminal_summary_build_millis = 505516` instead of `495256`,
    `terminal_summary_connectivity_millis = 99484` instead of `95969`
  - at `prefix_states_explored = 43`:
    `elapsed_millis = 1017859` instead of `998555`,
    `terminal_summary_build_millis = 918924` instead of `901994`,
    `terminal_summary_connectivity_millis = 183265` instead of `178000`
  - at `prefix_states_explored = 44`:
    `elapsed_millis = 1040469` instead of `1020529`,
    `terminal_summary_build_millis = 939406` instead of `921924`,
    `terminal_summary_connectivity_millis = 187753` instead of `182453`
- Residual measured work also moved slightly the wrong way:
  - aggregation rose to `68588/120729/122966` from `68266/119561/121524`
  - exact `nu` rose slightly to `39558/73405/74546` from
    `39523/73348/74489`
  - `terminal_materialize_millis = 338` instead of `327`
- So the new rerun proved something useful even though it did not earn keep:
  - the hidden prep bucket is now visible on the honest plateau
  - but the tuple-remap candidate-prep path is not the keep target
  - the short baseline is still more honestly blocked by the already-measured
    connectivity bucket first and aggregation second
  - the next slice should return to one narrow measured connectivity-side or
    aggregation-side cut, not another blind prep-side rewrite

## Goal

Land one narrow connectivity-side or aggregation-side cut on top of
`runs/codex-claim-release-step4-kernel-connectivity-v2`, then keep it only if
stored evidence shows both `terminal_summary_build_millis` and wall clock
improve at matched `24/43/44` checkpoints without blowing up
`terminal_materialize_millis` or moving cost into another hidden bucket.

## Current Comparison Targets

Use these as the numbers to beat:

- `runs/codex-claim-release-step4-kernel-connectivity-v2`
  - honest retained plateau active from `prefix_states_explored = 24`
  - `prefix_states_explored = 24` at `551.8s`
  - `prefix_states_explored = 43` at `998.6s`
  - `prefix_states_explored = 44` at `1020.5s`
  - retained prefix cache stayed `39 groups / 144845 candidates` at `24/43/44`
  - `terminal_summary_build_millis = 495256/901994/921924`
  - `terminal_summary_connectivity_millis = 95969/178000/182453`
  - `terminal_summary_aggregation_millis = 68266/119561/121524`
  - `terminal_summary_exact_nu_millis = 39523/73348/74489`
  - `terminal_materialize_millis = 327`
  - fallback connectivity stayed `0`
- `runs/codex-claim-release-step4-terminal-candidate-prep-v1`
  - latest dropped rerun
  - same retained plateau `= 39 groups / 144845 candidates`
  - `terminal_summary_candidate_prep_millis = 32904/71577/73974`
  - `elapsed_millis = 562457/1017859/1040469`
  - `terminal_summary_build_millis = 505516/918924/939406`
  - `terminal_summary_connectivity_millis = 99484/183265/187753`
  - `terminal_summary_aggregation_millis = 68588/120729/122966`
  - use this only as proof that the tuple-remap prep path is not the next keep
    target
- `runs/codex-claim-release-step4-kernel-connectivity-v4`
  - informative earlier drop showing that clause-side profile precompute can
    collapse the measured connectivity counter while still making total build
    and wall slower
  - use this only as a warning not to revive that profile-precompute shape

## Do This Next

### 1. Keep The Prep Experiment Dropped

Do not carry forward the code from
`runs/codex-claim-release-step4-terminal-candidate-prep-v1`.

The new prep counter did its job, but the actual throughput cut still made the
matched plateau checkpoints slower than the kept short baseline.

### 2. Choose One Narrow Cut Inside Measured Connectivity Or Aggregation

Preferred direction for this slice:

- stay inside the already-measured connectivity or aggregation loop
- do not add another broad telemetry pass first

Allowed directions:

- reduce work inside the current `terminal_connectivity` path using only the
  existing parent summary data already on hand
- reduce historical reanchor or equivalent connectivity-side bookkeeping only
  if it stays inside the measured connectivity bucket
- reduce bound merge, acceptance-rank bookkeeping, or evaluation-record churn
  inside the admitted summary aggregation path
- only touch exact `nu` first if the chosen patch already leaves connectivity
  and aggregation flat on the new candidate

Do not spend this slice on:

- another pre-summary terminal-candidate preparation patch
- reviving the dropped clause-side profile precompute from
  `runs/codex-claim-release-step4-kernel-connectivity-v4`
- another expr-keyed hot-path cache
- another admissibility shortcut
- another frontier ordering experiment
- another proof-close handoff experiment
- another context-equivalence or summary-cache reuse experiment
- another post-plateau summary-skip or direct-materialize gate
- memory compaction, certification, or benchmark work

### 3. Re-Earn Targeted Tests

Required tests:

- one targeted regression that the kept reference step-`4` winner still
  survives
- one targeted exactness test for the new connectivity-side or aggregation-side
  cut
- one representative remaining-one summary test proving admitted or pruned
  counts stay exact

Then rerun the standing validation set:

```bash
cargo test -p pen-search claim_
cargo test -p pen-search online_work_items_
cargo test -p pen-search prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations
cargo test -p pen-search terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity
cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance
```

### 4. Re-Earn One Stored Release `until_step = 4` Rerun

Run the claim profile derived from `configs/desktop_claim_shadow_1h.toml`
with:

- `until_step = 4`
- release build
- live checkpoint persistence left on

Use a new run id that states the new target, for example:

- `runs/codex-claim-release-step4-kernel-connectivity-v5`
- or `runs/codex-claim-release-step4-kernel-aggregation-v1`

### 5. Decide Keep Or Drop From Stored Evidence

Keep the patch only if the stored `reports/steps/step-04-live.ndjson` shows
all of the following:

- the same honest retained `39 groups / 144845 candidates` plateau
- `terminal_summary_build_millis` falls at matching `24/43/44` checkpoints
- wall clock reaches at least as far as
  `runs/codex-claim-release-step4-kernel-connectivity-v2`
  at matching checkpoints
- the targeted measured bucket actually falls
- `terminal_materialize_millis` stays near the current kept level
- no new fallback-connectivity or cache-residency blowup appears

Drop the patch if it only moves cost into another measured bucket, hides work
outside the kept split again, raises `terminal_materialize_millis`, or makes
the same `24/43/44` checkpoints slower than the current short baseline.

## What To Read After The Rerun

Open `reports/steps/step-04-live.ndjson` and answer:

- did the same honest `39/144845` plateau recur?
- did the targeted measured bucket actually fall?
- did `terminal_summary_build_millis` fall at `24/43/44`?
- did wall clock beat the current kept baseline?
- did `terminal_materialize_millis` stay controlled?
- after the new cut, is the wall now still connectivity, aggregation, exact
  `nu`, or something else?

## Stop Condition For This Note

Rewrite this file as soon as one new stored rerun shows one of these is true:

- the remaining plateau wall is no longer the measured connectivity or
  aggregation buckets
- step `4` is no longer the dominant blocker
- the next honest move is a full-profile rerun rather than another short slice
- the real full profile exposes a later honest wall
