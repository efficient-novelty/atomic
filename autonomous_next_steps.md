# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, the algebraic `nu`
ceiling patch, the step-`4` kernel telemetry split, the family-agnostic
claim terminal-admissibility shortcut, and the exact non-allocating
connectivity summary scan are already landed.
Assume the context-equivalence quotient, the frontier-pop ordering experiment,
the exact-two-step local ordering experiment, the proof-close handoff
experiment, the broad post-plateau summary-skip experiment, the narrower
post-plateau materialize-side gate, and the post-plateau summary-cache reuse
experiment were all measured on stored short reruns and then dropped from code
after failing to earn keep.

The current short step-`4` baseline is
`runs/codex-claim-release-step4-kernel-connectivity-v1`.
The previous short baseline is
`runs/codex-claim-release-step4-kernel-admissibility-v1`.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.
The diagnostic kernel split that explained the last wall is
`runs/codex-claim-release-step4-kernel-profile-v2`.

Ignore `runs/codex-claim-release-step4-kernel-profile-v1`: that local run used
the flawed per-candidate millisecond accumulation and is not valid evidence.

## Working Diagnosis

- The claim lane is still compute-bound in step `4`, but the wall has moved.
- The honest live shape still repeats:
  - frontier queue stays on the same contour
  - legality growth stays on the same contour
  - retained prefix cache still flattens at
    `39 groups / 144845 candidates`
- The kernel split rerun
  `runs/codex-claim-release-step4-kernel-profile-v2` showed the dominant
  plateau cost at `prefix_states_explored = 44` was:
  - admissibility `= 679889 ms`
  - connectivity `= 492575 ms`
  - aggregation `= 118953 ms`
  - exact `nu` `= 74386 ms`
- The follow-up keep rerun
  `runs/codex-claim-release-step4-kernel-admissibility-v1` widened the existing
  terminal-summary admissibility shortcut onto the family-agnostic claim lane
  and removed that dominant admissibility cost on the same plateau.
- The next keep rerun
  `runs/codex-claim-release-step4-kernel-connectivity-v1` then replaced the old
  allocating `lib_refs` / `var_refs` scans inside
  `ConnectivitySummary::extend` with exact non-allocating scans and cut the
  remaining plateau wall further on the same surface:
  - same honest retained plateau `= 39 groups / 144845 candidates`
  - `terminal_summary_connectivity_millis = 222604/399280/408582`
    at `24/43/44` instead of `269953/481062/492949`
  - `terminal_summary_build_millis = 635477/1145519/1170875`
    at `24/43/44` instead of `695759/1263393/1292019`
  - `elapsed_millis = 692343/1245950/1273659`
    at `24/43/44` instead of `756279/1367539/1398528`
  - `terminal_materialize_millis = 382` instead of `388`
  - fallback connectivity stayed `0`
- That means the current honest wall is still:
  - connectivity first
  - aggregation second
  - exact `nu` remains smaller and did not materially move
- So the next slice should stay on connectivity-side work unless new stored
  evidence changes the shape again.

## Goal

Land one more narrow connectivity-side cut inside the retained remaining-one
summary path, using the existing kernel telemetry to prove the dominant
plateau cost falls again without shifting the wall back into aggregation or
materialization.

## Current Comparison Targets

Use these as the numbers to beat:

- `runs/codex-claim-release-step4-kernel-connectivity-v1`
  - honest retained plateau active from `prefix_states_explored = 24`
  - `prefix_states_explored = 24` at `692.3s`
  - `prefix_states_explored = 43` at `1246.0s`
  - `prefix_states_explored = 44` at `1273.7s`
  - retained prefix cache stayed `39 groups / 144845 candidates` at `24/43/44`
  - `terminal_summary_build_millis = 635477` at `24`
  - `terminal_summary_build_millis = 1145519` at `43`
  - `terminal_summary_build_millis = 1170875` at `44`
  - `terminal_summary_connectivity_millis = 222604` at `24`,
    `399280` at `43`, `408582` at `44`
  - `terminal_summary_aggregation_millis = 69544` at `24`,
    `121941` at `43`, `123884` at `44`
  - `terminal_summary_exact_nu_millis = 39556` at `24`,
    `73468` at `43`, `74610` at `44`
  - `terminal_summary_admissibility_millis = 0` at `24/43/44`
  - `terminal_materialize_millis = 382` at `24/43/44`
- `runs/codex-claim-release-step4-kernel-admissibility-v1`
  - previous kept baseline before the connectivity cut
  - `prefix_states_explored = 24/43/44`
    at `756.3s / 1367.5s / 1398.5s`
  - retained prefix cache `= 39 groups / 144845 candidates`
  - `terminal_summary_connectivity_millis = 269953/481062/492949`
  - `terminal_summary_build_millis = 695759/1263393/1292019`
- `runs/codex-claim-release-step4-kernel-profile-v2`
  - diagnostic-only run that proved admissibility was the old dominant wall
  - `terminal_summary_admissibility_millis = 363234/668084/679889`
    at `24/43/44`

## Do This Next

### 1. Reuse The Existing Kernel Split

Do not start by adding another broad telemetry patch.

The current split is already enough:

- connectivity work
- fallback connectivity checks
- admissibility work
- exact `nu` work
- aggregation and bound/rank bookkeeping
- honest plateau activation marker

Only add telemetry if one missing counter is strictly necessary to decide
between two concrete connectivity or aggregation cuts.

### 2. Choose One Narrow Patch In The Still-Dominant Connectivity Cost

Allowed directions:

- reduce one more repeated connectivity-summary extension cost that still sits
  on the retained plateau
- or move one exact structural disconnection rejection earlier if it can be
  proven from the existing summary state without reopening broad heuristics
- only pivot to aggregation if one smaller connectivity edit is no longer
  obvious from the stored evidence

Do not spend this slice on:

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
- one targeted exactness test for the new connectivity or aggregation cut
- one representative remaining-one summary test proving admitted/pruned counts
  stay exact

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

Use a new run id that states the actual target, for example:

- `runs/codex-claim-release-step4-kernel-connectivity-v2`
- or `runs/codex-claim-release-step4-kernel-aggregation-v1`

### 5. Decide Keep Or Drop From Stored Evidence

Keep the patch only if the stored `reports/steps/step-04-live.ndjson` shows
all of the following:

- the same honest retained `39 groups / 144845 candidates` plateau
- the targeted dominant plateau sub-phase actually improves against
  `runs/codex-claim-release-step4-kernel-connectivity-v1`
- `terminal_summary_build_millis` falls at matching plateau checkpoints
- wall clock reaches at least as far as the new short baseline at matching
  checkpoints
- `terminal_materialize_millis` stays near the current kept level
- no new fallback-connectivity or cache-residency blowup appears

Drop the patch if it only shifts cost between connectivity and aggregation,
reopens materialization, or makes the same `24/43/44` checkpoints slower than
the current short baseline.

## What To Read After The Rerun

Open `reports/steps/step-04-live.ndjson` and answer:

- did the same honest `39/144845` plateau recur?
- which plateau sub-phase now dominates?
- did the targeted connectivity or aggregation counter actually fall?
- did `terminal_summary_build_millis` fall at `24/43/44`?
- did wall clock beat the current kept baseline?
- did `terminal_materialize_millis` stay controlled?
- did the hot cost move again, or is connectivity still the wall?

## Stop Condition For This Note

Rewrite this file as soon as one new stored rerun shows one of these is true:

- the remaining plateau wall is no longer connectivity or aggregation
- step `4` is no longer the dominant blocker
- the next honest move is a full-profile rerun rather than another short slice
- the real full profile exposes a later honest wall
