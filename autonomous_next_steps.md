# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, the algebraic `nu`
ceiling patch, the step-`4` kernel telemetry split, the family-agnostic
claim terminal-admissibility shortcut, the exact non-allocating
connectivity summary scan, and the terminal-only cached parent connectivity
decision are already landed.
Assume the context-equivalence quotient, the frontier-pop ordering experiment,
the exact-two-step local ordering experiment, the proof-close handoff
experiment, the broad post-plateau summary-skip experiment, the narrower
post-plateau materialize-side gate, the post-plateau summary-cache reuse
experiment, the expr-keyed terminal clause hot-path profile cache
experiment, and the clause-side terminal connectivity profile precompute
experiment were all measured on stored short reruns and then dropped from code
after failing to earn keep.

The current short step-`4` baseline is
`runs/codex-claim-release-step4-kernel-connectivity-v2`.
The previous short baseline is
`runs/codex-claim-release-step4-kernel-connectivity-v1`.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.
The diagnostic kernel split that explained the last kept wall is
`runs/codex-claim-release-step4-kernel-profile-v2`.

Ignore `runs/codex-claim-release-step4-kernel-profile-v1`: that local run used
the flawed per-candidate millisecond accumulation and is not valid evidence.

## Working Diagnosis

- The claim lane is still compute-bound in step `4`, but the measured wall has
  moved again.
- The honest live shape still repeats:
  - frontier queue stays on the same contour
  - legality growth stays on the same contour
  - retained prefix cache still flattens at
    `39 groups / 144845 candidates`
- The kept short baseline
  `runs/codex-claim-release-step4-kernel-connectivity-v2` still defines the
  surface to beat:
  - same honest retained plateau `= 39 groups / 144845 candidates`
  - `elapsed_millis = 551825/998555/1020529` at `24/43/44`
  - `terminal_summary_build_millis = 495256/901994/921924` at `24/43/44`
  - `terminal_summary_connectivity_millis = 95969/178000/182453`
    at `24/43/44`
  - `terminal_summary_aggregation_millis = 68266/119561/121524`
    at `24/43/44`
  - `terminal_summary_exact_nu_millis = 39523/73348/74489`
    at `24/43/44`
  - `terminal_materialize_millis = 327`
  - fallback connectivity stayed `0`
- The latest dropped rerun
  `runs/codex-claim-release-step4-kernel-connectivity-v4` then precomputed a
  clause-side terminal connectivity profile beside the terminal candidate list
  and reused that profile inside the remaining-one connectivity decision.
- That rerun kept the same honest plateau:
  - retained prefix cache `= 39 groups / 144845 candidates` at `24/43/44`
  - `terminal_summary_first_plateau_activation_prefix_state = 24`
  - `terminal_summary_plateau_activations = 97234` at `44`
- But matched against the current short baseline
  `runs/codex-claim-release-step4-kernel-connectivity-v2`, it only moved cost
  instead of removing it:
  - at `prefix_states_explored = 24`:
    `elapsed_millis = 606748` instead of `551825`,
    `terminal_summary_build_millis = 544777` instead of `495256`,
    `terminal_summary_connectivity_millis = 7074` instead of `95969`
  - at `prefix_states_explored = 43`:
    `elapsed_millis = 1091451` instead of `998555`,
    `terminal_summary_build_millis = 981266` instead of `901994`,
    `terminal_summary_connectivity_millis = 12693` instead of `178000`
  - at `prefix_states_explored = 44`:
    `elapsed_millis = 1115276` instead of `1020529`,
    `terminal_summary_build_millis = 1002528` instead of `921924`,
    `terminal_summary_connectivity_millis = 13075` instead of `182453`
- Residual measured work did not justify the regression:
  - aggregation rose to `70228/122321/124289` from `68266/119561/121524`
  - exact `nu` stayed effectively unchanged at
    `39662/73565/74705` versus `39523/73348/74489`
  - `terminal_materialize_millis = 343` instead of `327`
  - fallback connectivity stayed `0`
- So the new rerun proved something useful even though it did not earn keep:
  - the old measured connectivity work can be collapsed dramatically
  - but the total `terminal_summary_build_millis` and wall clock still get
    worse
  - therefore the current honest wall is no longer "connectivity first" in the
    available counters
  - the missing cost is now in pre-summary terminal candidate preparation or
    equivalent clause-side setup work that the current split does not isolate

## Goal

Make the hidden remaining-one pre-summary setup cost measurable, then land one
narrow cut there only if stored evidence shows `terminal_summary_build_millis`
and wall clock both improve against
`runs/codex-claim-release-step4-kernel-connectivity-v2`.

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
- `runs/codex-claim-release-step4-kernel-connectivity-v4`
  - latest dropped rerun
  - same retained plateau `= 39 groups / 144845 candidates`
  - `elapsed_millis = 606748/1091451/1115276`
  - `terminal_summary_build_millis = 544777/981266/1002528`
  - `terminal_summary_connectivity_millis = 7074/12693/13075`
  - `terminal_summary_aggregation_millis = 70228/122321/124289`
  - use this only as proof that the hidden cost now sits outside the measured
    connectivity counter
- `runs/codex-claim-release-step4-kernel-profile-v2`
  - diagnostic-only kept reference for the older measured split
  - use this only as provenance for why admissibility was the earlier wall

## Do This Next

### 1. Add One Missing Counter, Not Another Broad Telemetry Pass

The current split is no longer enough because `v4` made
`terminal_summary_connectivity_millis` tiny while total
`terminal_summary_build_millis` still rose.

Add exactly one narrow counter around the missing pre-summary work, for example:

- terminal candidate preparation before the first connectivity check
- or, if that bundle is still too wide, the clause-side structural profile
  build inside that preparation path

Do not reopen a broad diagnostic patch across the whole kernel.

### 2. Choose One Narrow Patch In The Newly Exposed Pre-Summary Cost

Allowed directions:

- reuse clause-side structural data directly from the shared terminal clause
  catalog when that data is genuinely catalog-stable across prefixes
- reduce per-prefix allocation or copy churn while building the terminal
  candidate list
- avoid rebuilding one clause-side structural summary when the same shared
  last-clause slice is already available unchanged
- only return to connectivity or aggregation edits after the new counter proves
  that candidate preparation is not the new wall

Do not spend this slice on:

- retrying the dropped `v4` profile shape as-is
- another expr-keyed `HashMap` or `BTreeMap` hot-path cache
- another admissibility shortcut
- another exact-`nu` first optimization
- another frontier ordering experiment
- another proof-close handoff experiment
- another context-equivalence or summary-cache reuse experiment
- another post-plateau summary-skip or direct-materialize gate
- memory compaction, certification, or benchmark work

### 3. Re-Earn Targeted Tests

Required tests:

- one targeted regression that the kept reference step-`4` winner still
  survives
- one targeted exactness test for the new pre-summary or summary-prep cut
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

- `runs/codex-claim-release-step4-kernel-prep-v1`
- or `runs/codex-claim-release-step4-terminal-candidate-prep-v1`

### 5. Decide Keep Or Drop From Stored Evidence

Keep the patch only if the stored `reports/steps/step-04-live.ndjson` shows
all of the following:

- the same honest retained `39 groups / 144845 candidates` plateau
- the new pre-summary counter actually falls on the plateau
- `terminal_summary_build_millis` falls at matching `24/43/44` checkpoints
- wall clock reaches at least as far as
  `runs/codex-claim-release-step4-kernel-connectivity-v2`
  at matching checkpoints
- `terminal_materialize_millis` stays near the current kept level
- no new fallback-connectivity or cache-residency blowup appears

Drop the patch if it only moves cost between measured counters, hides work
outside the kept split again, raises `terminal_materialize_millis`, or makes
the same `24/43/44` checkpoints slower than the current short baseline.

## What To Read After The Rerun

Open `reports/steps/step-04-live.ndjson` and answer:

- did the same honest `39/144845` plateau recur?
- did the new pre-summary counter dominate?
- did that new counter actually fall?
- did `terminal_summary_build_millis` fall at `24/43/44`?
- did wall clock beat the current kept baseline?
- did `terminal_materialize_millis` stay controlled?
- after the new counter is visible, is the wall now pre-summary setup,
  aggregation, or something else?

## Stop Condition For This Note

Rewrite this file as soon as one new stored rerun shows one of these is true:

- the remaining plateau wall is no longer the pre-summary setup cost or
  aggregation
- step `4` is no longer the dominant blocker
- the next honest move is a full-profile rerun rather than another short slice
- the real full profile exposes a later honest wall
