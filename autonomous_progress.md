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
  `runs/codex-claim-release-step4-kernel-connectivity-v1`
- The current short step-`4` baseline is now
  `runs/codex-claim-release-step4-kernel-connectivity-v1`.
- The previous short step-`4` baseline is now
  `runs/codex-claim-release-step4-kernel-admissibility-v1`.
- The current full-profile iteration baseline remains
  `runs/codex-claim-release-full-nu-profile-v1`.
- The claim lane is still compute-bound in step `4`, but the dominant inner
  wall remains connectivity, with aggregation still second.

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
- Previous kept short baseline:
  `runs/codex-claim-release-step4-kernel-admissibility-v1`
  - It kept the same honest frontier and retained-prefix shape while removing
    direct admissibility work from the plateau.
- New kept short baseline:
  `runs/codex-claim-release-step4-kernel-connectivity-v1`
  - It keeps the same honest retained plateau:
    - retained prefix cache `= 39 groups / 144845 candidates` at `24/43/44`
    - `terminal_summary_first_plateau_activation_prefix_state = 24`
    - `terminal_summary_plateau_activations = 97234` at `44`
  - It replaces the old allocating `lib_refs` / `var_refs` scans inside the
    connectivity summary extension with exact non-allocating scans.
  - Matched against the previous short baseline
    `runs/codex-claim-release-step4-kernel-admissibility-v1`:
    - at `prefix_states_explored = 24`:
      `elapsed_millis = 692343` instead of `756279`,
      `terminal_summary_build_millis = 635477` instead of `695759`,
      `terminal_summary_connectivity_millis = 222604` instead of `269953`
    - at `prefix_states_explored = 43`:
      `elapsed_millis = 1245950` instead of `1367539`,
      `terminal_summary_build_millis = 1145519` instead of `1263393`,
      `terminal_summary_connectivity_millis = 399280` instead of `481062`
    - at `prefix_states_explored = 44`:
      `elapsed_millis = 1273659` instead of `1398528`,
      `terminal_summary_build_millis = 1170875` instead of `1292019`,
      `terminal_summary_connectivity_millis = 408582` instead of `492949`
  - Residual costs stayed controlled at the same matched checkpoints:
    - aggregation rose only slightly:
      `69544/121941/123884` instead of `66625/120646/122571`
    - exact `nu` stayed close:
      `39556/73468/74610` instead of `39462/73255/74395`
    - `terminal_materialize_millis = 382` instead of `388`
    - fallback connectivity stayed `0`
  - The stored rerun was manually stopped after enough matched plateau evidence
    had been recorded beyond `prefix_states_explored = 44`; the keep decision
    is based on the stored `step-04-live.ndjson` checkpoints.
  - So the step-`4` gain is real and exact:
    the same plateau survives, connectivity falls by about `17%`, total
    summary-build time falls by about `9%`, and wall-clock progress improves by
    about `8.9%` at the matched `44` checkpoint.
- Re-earned validations in this turn:
  - `cargo test -p pen-search claim_terminal_connectivity_keeps_reference_step_four_winner_clause`
  - `cargo test -p pen-search claim_terminal_connectivity_matches_direct_step_four_assessment`
  - `cargo test -p pen-search claim_terminal_prefix_completion_summary_matches_direct_exact_assessment`
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
- The old allocating `lib_refs` / `var_refs` scans inside
  `ConnectivitySummary::extend` were real hot-loop overhead on the retained
  plateau.
- Replacing those scans with exact non-allocating walks earned keep on stored
  evidence without changing the honest plateau shape or reopening
  materialization.
- Connectivity is still the dominant plateau cost, aggregation is still
  second, and exact `nu` remains a smaller tail.
- Because the same `39 / 144845` plateau still repeats, the next honest move
  is another narrow connectivity-side cut rather than a full-profile rerun.
- Memory remains controlled on the short reruns; the wall is still compute,
  not allocator or RSS pressure.

## Immediate Order

1. Patch one more narrow connectivity-side throughput cut inside the
   remaining-one summary builder, using the existing kernel telemetry rather
   than reopening a broad instrumentation pass.
2. Re-earn the targeted remaining-one exactness tests and the standing claim
   regression set.
3. Re-earn one stored release `until_step = 4` rerun and keep the patch only
   if it improves matched plateau checkpoints against
   `runs/codex-claim-release-step4-kernel-connectivity-v1`.
4. Only after another short step-`4` slice earns keep should the next real
   `desktop_claim_shadow_1h` full-profile rerun happen.

## Active Baselines

- Current full-profile baseline:
  `runs/codex-claim-release-full-nu-profile-v1`
- Current short step-`4` baseline:
  `runs/codex-claim-release-step4-kernel-connectivity-v1`
- Previous short step-`4` baseline:
  `runs/codex-claim-release-step4-kernel-admissibility-v1`
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
