# Autonomous Claim Lane Progress

Last updated: 2026-03-27

This file is the live status read for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
execution slice, [autonomous_plan.md](autonomous_plan.md) for the staged path
to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the
final gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- The following claim-lane wins are landed in code:
  - delayed materialization
  - the incumbent-primary remaining-one fast path
  - the one-pass `structural_nu` summary-build fast path
  - the algebraic `nu` ceiling patch
  - the family-agnostic claim terminal-admissibility shortcut
  - the exact non-allocating connectivity summary scan
  - the terminal-only cached parent connectivity decision
- The following claim-only slices were implemented, measured on stored short
  reruns, and then dropped from code after failing keep:
  - context-equivalence quotient:
    `runs/codex-claim-release-step4-context-equivalence-v1`
  - frontier-pop incumbent ordering:
    `runs/codex-claim-release-step4-incumbent-ordering-v1`
  - exact-two-step local ordering:
    `runs/codex-claim-release-step4-local-two-step-order-v2`
  - proof-close handoff ordering:
    `runs/codex-claim-release-step4-proof-close-handoff-v1`
  - post-plateau summary-skip:
    `runs/codex-claim-release-step4-post-plateau-v1`
  - post-plateau materialize-side gate:
    `runs/codex-claim-release-step4-post-plateau-materialize-v1`
  - post-plateau summary-cache reuse:
    `runs/codex-claim-release-step4-post-plateau-summary-cache-v3`
  - expr-keyed terminal clause hot-path profile cache:
    `runs/codex-claim-release-step4-kernel-connectivity-v3`
  - clause-side terminal connectivity profile precompute:
    `runs/codex-claim-release-step4-kernel-connectivity-v4`
  - terminal-candidate tuple remap and allocation cut:
    `runs/codex-claim-release-step4-terminal-candidate-prep-v1`
- The step-`4` kernel is still measurable from stored evidence:
  - valid diagnostic split:
    `runs/codex-claim-release-step4-kernel-profile-v2`
  - ignore as invalid local diagnostic:
    `runs/codex-claim-release-step4-kernel-profile-v1`
- The most recent stored short rerun in this turn did not earn keep:
  `runs/codex-claim-release-step4-terminal-candidate-prep-v1`
- The current short step-`4` baseline remains
  `runs/codex-claim-release-step4-kernel-connectivity-v2`.
- The previous short step-`4` baseline remains
  `runs/codex-claim-release-step4-kernel-connectivity-v1`.
- The current full-profile iteration baseline remains
  `runs/codex-claim-release-full-nu-profile-v1`.
- The claim lane is still compute-bound in step `4`, but the latest rerun
  moved the hidden pre-summary cost into view and showed that this particular
  candidate-preparation cut is not the keep target:
  the same plateau survived, the new prep counter landed honestly on it, and
  yet both `terminal_summary_build_millis` and wall clock still regressed
  against the kept short baseline.

## Latest Evidence

- Diagnostic kernel split:
  `runs/codex-claim-release-step4-kernel-profile-v2`
  - At the honest retained plateau `39 groups / 144845 candidates`, the
    measured hot step-`4` sub-phases at `prefix_states_explored = 44` were:
    - admissibility `= 679889 ms`
    - connectivity `= 492575 ms`
    - aggregation `= 118953 ms`
    - exact `nu` `= 74386 ms`
  - That rerun was diagnostic-only and slower overall, but it identified the
    real dominant cost on the live plateau.
- Current kept short baseline:
  `runs/codex-claim-release-step4-kernel-connectivity-v2`
  - It keeps the same honest retained plateau:
    - retained prefix cache `= 39 groups / 144845 candidates` at `24/43/44`
    - `terminal_summary_first_plateau_activation_prefix_state = 24`
    - `terminal_summary_plateau_activations = 97234` at `44`
  - It reuses the cached parent legality summary for the terminal-only
    connectivity or reanchor decision instead of cloning and extending the
    full child legality summary for each last-clause check.
  - Matched against the previous short baseline
    `runs/codex-claim-release-step4-kernel-connectivity-v1`:
    - at `prefix_states_explored = 24`:
      `elapsed_millis = 551825` instead of `692343`,
      `terminal_summary_build_millis = 495256` instead of `635477`,
      `terminal_summary_connectivity_millis = 95969` instead of `222604`
    - at `prefix_states_explored = 43`:
      `elapsed_millis = 998555` instead of `1245950`,
      `terminal_summary_build_millis = 901994` instead of `1145519`,
      `terminal_summary_connectivity_millis = 178000` instead of `399280`
    - at `prefix_states_explored = 44`:
      `elapsed_millis = 1020529` instead of `1273659`,
      `terminal_summary_build_millis = 921924` instead of `1170875`,
      `terminal_summary_connectivity_millis = 182453` instead of `408582`
  - Residual costs stayed controlled at the same matched checkpoints:
    - aggregation stayed slightly lower:
      `68266/119561/121524` instead of `69544/121941/123884`
    - exact `nu` stayed essentially unchanged:
      `39523/73348/74489` instead of `39556/73468/74610`
    - `terminal_materialize_millis = 327` instead of `382`
    - fallback connectivity stayed `0`
  - The stored rerun was manually stopped once the stored
    `step-04-live.ndjson` reached the matched `24/43/44` plateau checkpoints;
    the keep decision is based on those stored checkpoints.
- Dropped short rerun:
  `runs/codex-claim-release-step4-kernel-connectivity-v3`
  - It tried caching per-clause hot-path terminal
    check or connectivity profiles keyed by clause `expr`.
  - It kept the same honest plateau, but at the matched `44` checkpoint it
    slowed wall clock to `1081589` from `1020529` and raised
    `terminal_summary_build_millis` to `978181` from `921924`, so it was
    dropped from code.
- Dropped short rerun:
  `runs/codex-claim-release-step4-kernel-connectivity-v4`
  - It tried precomputing a clause-side terminal connectivity profile beside
    the terminal candidate list and reusing that profile inside the
    remaining-one connectivity decision.
  - It kept the same honest plateau and collapsed the measured connectivity
    counter, but still regressed at the matched `44` checkpoint to
    `elapsed_millis = 1115276` and
    `terminal_summary_build_millis = 1002528`, so it was dropped from code.
- New dropped short rerun:
  `runs/codex-claim-release-step4-terminal-candidate-prep-v1`
  - It added one narrow pre-summary candidate-preparation counter and tried one
    narrow throughput cut that removed the extra filtered-candidate remap and
    tuple allocation before the first connectivity check.
  - It kept the same honest retained plateau:
    - retained prefix cache `= 39 groups / 144845 candidates` at `24/43/44`
    - `terminal_summary_first_plateau_activation_prefix_state = 24`
    - `terminal_summary_plateau_activations = 97234` at `44`
  - The new counter did land honestly on the plateau:
    - `terminal_summary_candidate_prep_millis = 32904/71577/73974`
      at `24/43/44`
  - But matched against the current short baseline
    `runs/codex-claim-release-step4-kernel-connectivity-v2`, it still missed
    keep:
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
  - Residual measured costs also moved slightly the wrong way:
    - aggregation rose to `68588/120729/122966` from `68266/119561/121524`
    - exact `nu` rose slightly to `39558/73405/74546` from
      `39523/73348/74489`
    - `terminal_materialize_millis = 338` instead of `327`
  - The stored rerun was manually stopped once the stored
    `step-04-live.ndjson` reached the matched `24/43/44` plateau checkpoints;
    the drop decision is based on those stored checkpoints.
  - So the new read is useful even though the patch failed:
    terminal candidate preparation is now visible on the plateau, but this
    tuple-remap path is only about `33/72/74` seconds there and is not the
    next keep target; the short baseline is still more honestly blocked by the
    already-measured connectivity bucket first and aggregation second.
- Re-earned validations in this turn:
  - `cargo test -p pen-search claim_remaining_one_algebraic_ceiling_keeps_reference_step_four_winner_prefix`
  - `cargo test -p pen-search claim_terminal_prefix_completion_summary_matches_direct_exact_assessment`
  - `cargo test -p pen-search claim_remaining_one_algebraic_ceiling_prunes_before_summary_build`
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search online_work_items_`
  - `cargo test -p pen-search prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations`
  - `cargo test -p pen-search terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity`
  - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

## Current Read

- The recent loop of failed experiments is still anchored by one real keep
  slice in `runs/codex-claim-release-step4-kernel-connectivity-v2`.
- The kernel split confirmed that the remaining wall stayed inside
  `compute_terminal_prefix_completion_summary_from_candidates`, not in
  ordering, cache-reuse, or post-plateau gating.
- The exact non-allocating connectivity summary scan and the terminal-only
  cached parent legality summary reuse were both real step-`4` wins on the
  honest retained plateau.
- The expr-keyed terminal clause cache, the clause-side connectivity profile
  precompute, and now the terminal-candidate tuple remap cut all failed keep on
  stored evidence and should stay dropped from code.
- The new prep counter proved something useful:
  pre-summary candidate preparation is measurable on the plateau, but on the
  kept surface it is smaller than the already-measured connectivity and
  aggregation buckets and does not justify another blind prep-side rewrite.
- So the current honest wall on the kept short baseline has moved back to the
  measured counters:
  connectivity first, aggregation second, with exact `nu` and candidate prep
  behind them.
- Memory remains controlled on the short reruns; the live blocker is still
  compute, not allocator or RSS pressure.

## Immediate Order

1. Keep `runs/codex-claim-release-step4-terminal-candidate-prep-v1` as stored
   evidence only and keep its code patch dropped.
2. Return to one narrow measured step-`4` cut inside connectivity or
   aggregation; do not reopen another broad pre-summary or clause-profile
   experiment.
3. Re-earn the standing claim regression set and one more stored release
   `until_step = 4` rerun; keep the next patch only if it improves matched
   `24/43/44` checkpoints against
   `runs/codex-claim-release-step4-kernel-connectivity-v2`.
4. Only after another short step-`4` slice earns keep should the next real
   `desktop_claim_shadow_1h` full-profile rerun happen.

## Active Baselines

- Current full-profile baseline:
  `runs/codex-claim-release-full-nu-profile-v1`
- Current short step-`4` baseline:
  `runs/codex-claim-release-step4-kernel-connectivity-v2`
- Previous short step-`4` baseline:
  `runs/codex-claim-release-step4-kernel-connectivity-v1`
- Most recent valid diagnostic:
  `runs/codex-claim-release-step4-kernel-profile-v2`
- Ignore as invalid diagnostic:
  `runs/codex-claim-release-step4-kernel-profile-v1`
- Most recent short evidence that did not advance the current short baseline:
  `runs/codex-claim-release-step4-terminal-candidate-prep-v1`
- Previous short evidence that did not advance the current short baseline:
  `runs/codex-claim-release-step4-kernel-connectivity-v4`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged.
- Keep user-facing wording at `bounded live recovery`.
- Keep the claim lane honest about still being guided and not yet fully
  unguided.
- Prefer exact structural cuts over semantic shortcuts.
- Trust stored artifacts over terminal impressions.
