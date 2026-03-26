# Autonomous Claim Lane Progress

Last updated: 2026-03-26

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
- The step-`4` kernel is now measurable from stored evidence:
  - valid diagnostic split:
    `runs/codex-claim-release-step4-kernel-profile-v2`
  - ignore as invalid local diagnostic:
    `runs/codex-claim-release-step4-kernel-profile-v1`
- A new stored short rerun earned keep in this turn:
  `runs/codex-claim-release-step4-kernel-admissibility-v1`
- The current short step-`4` baseline is now
  `runs/codex-claim-release-step4-kernel-admissibility-v1`.
- The previous short step-`4` baseline is now
  `runs/codex-claim-release-step4-algebraic-v1`.
- The current full-profile iteration baseline remains
  `runs/codex-claim-release-full-nu-profile-v1`.
- The claim lane is still compute-bound in step `4`, but the dominant inner
  wall has moved from admissibility to connectivity.

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
- New kept short baseline:
  `runs/codex-claim-release-step4-kernel-admissibility-v1`
  - It keeps the same honest frontier and retained-prefix shape:
    - `frontier_queue_len = 2731` at `prefix_states_explored = 44`
    - legality summaries `= 209842`
    - retained prefix cache `= 39 groups / 144845 candidates`
    - `terminal_summary_plateau_activations = 97234`
  - It removes the old dominant admissibility work from the plateau:
    - `terminal_summary_admissibility_checks = 0`
    - `terminal_summary_admissibility_millis = 0`
  - Matched against the previous short baseline
    `runs/codex-claim-release-step4-algebraic-v1` at
    `prefix_states_explored = 44`:
    - `elapsed_millis = 1398528` instead of `1662758`
    - `terminal_summary_build_millis = 1292019` instead of `1555470`
    - `terminal_materialize_millis = 388` instead of `466`
  - Matched against the diagnostic split
    `runs/codex-claim-release-step4-kernel-profile-v2` at
    `prefix_states_explored = 44`:
    - `elapsed_millis = 1398528` instead of `1972314`
    - `terminal_summary_build_millis = 1292019` instead of `1872046`
    - connectivity stayed essentially flat:
      `492949` instead of `492575`
    - aggregation stayed close:
      `122571` instead of `118953`
    - exact `nu` stayed close:
      `74395` instead of `74386`
  - So the step-`4` gain is real and exact:
    the same plateau survives, admissibility disappears from the hot loop, and
    wall-clock progress improves by about `15.9%` versus algebraic and
    `29.1%` versus the split diagnostic at the matched `44` checkpoint.
- Re-earned validations in this turn:
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search online_work_items_`
  - `cargo test -p pen-search prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations`
  - `cargo test -p pen-search terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity`
  - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

## Current Read

- The recent loop of failed experiments has been broken by a real keep slice.
- The kernel split confirmed that the remaining wall was inside
  `compute_terminal_prefix_completion_summary_from_candidates`, not in
  ordering, reuse, or post-plateau cache tricks.
- The family-agnostic claim lane was paying for direct admissibility work even
  though `family_filters = 0` and the existing terminal-summary path was
  already exact for that surface.
- Widening that shortcut earned keep on stored evidence without changing the
  honest plateau shape.
- The current dominant plateau cost is now connectivity, with aggregation
  second.
- Exact `nu` is now a smaller tail and should not be the next primary target.
- Memory remains controlled on the short reruns; the wall is still compute,
  not allocator or RSS pressure.

## Immediate Order

1. Patch one narrow connectivity-side or aggregation-side throughput cut inside
   the remaining-one summary builder, using the existing kernel telemetry
   rather than reopening a broad instrumentation pass.
2. Re-earn the targeted remaining-one exactness tests and the standing claim
   regression set.
3. Re-earn one stored release `until_step = 4` rerun and keep the patch only
   if it improves matched plateau checkpoints against
   `runs/codex-claim-release-step4-kernel-admissibility-v1`.
4. Only after another short step-`4` slice earns keep should the next real
   `desktop_claim_shadow_1h` full-profile rerun happen.

## Active Baselines

- Current full-profile baseline:
  `runs/codex-claim-release-full-nu-profile-v1`
- Current short step-`4` baseline:
  `runs/codex-claim-release-step4-kernel-admissibility-v1`
- Previous short step-`4` baseline:
  `runs/codex-claim-release-step4-algebraic-v1`
- Most recent valid diagnostic:
  `runs/codex-claim-release-step4-kernel-profile-v2`
- Ignore as invalid diagnostic:
  `runs/codex-claim-release-step4-kernel-profile-v1`
- Most recent short evidence that did not advance the current short baseline:
  `runs/codex-claim-release-step4-post-plateau-summary-cache-v3`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged.
- Keep user-facing wording at `bounded live recovery`.
- Keep the claim lane honest about still being guided and not yet fully
  unguided.
- Prefer exact structural cuts over semantic shortcuts.
- Trust stored artifacts over terminal impressions.
