# Autonomous Claim Lane Progress

Last updated: 2026-03-25

This file is the live status read for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
execution slice, [autonomous_plan.md](autonomous_plan.md) for the staged path
to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the
final gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- Delayed materialization, the narrow remaining-one incumbent-primary fast
  path, the one-pass `structural_nu` summary-build fast path, and the
  algebraic `nu` ceiling patch are all now landed in code.
- A new stored short rerun now exists on that algebraic binary:
  `runs/codex-claim-release-step4-algebraic-v1`.
- A new stored intended-profile rerun now also exists on that same binary:
  `runs/codex-claim-release-full-algebraic-v1`.
- The current short step-`4` iteration baseline now advances to
  `runs/codex-claim-release-step4-algebraic-v1`.
- The current full-profile iteration baseline remains
  `runs/codex-claim-release-full-nu-profile-v1`; the new full algebraic rerun
  did not earn replacement.
- The claim lane is still compute-bound in step `4`.
- The new `remaining_one_algebraic_prunes` counter did not record a single
  live hit on either stored rerun, so the new ceiling is presently inert on
  the intended profile.
- The broad `cargo test -p pen-search claim_` sweep still was not re-earned on
  this desktop. Keep local validation to compile coverage plus exact named
  tests.

## Latest Evidence

- New stored short rerun:
  `runs/codex-claim-release-step4-algebraic-v1`
- Against `runs/codex-claim-release-step4-nu-profile-v1`, the new short rerun
  kept the same honest retained-prefix shape and legality growth but reached
  the matching hot step-`4` checkpoints materially earlier:
  - `prefix_states_explored = 43` at `1629.3s` instead of `1728.7s`
  - `prefix_states_explored = 52` at `1975.0s` instead of `2090.3s`
  - `prefix_states_explored = 59` at `2252.6s` instead of `2360.5s`
  - At those same checkpoints, retained prefix cache stayed
    `39 groups / 144845 candidates` and legality summaries stayed
    `205199`, `246986`, and `279487` respectively.
  - At those same checkpoints, `terminal_summary_build_millis` fell to
    `1524266`, `1849248`, and `2111246` instead of
    `1622973`, `1963028`, and `2215871`.
- That short rerun was then manually stopped after enough stored evidence:
  - `prefix_states_explored = 180` at `7275.7s`
  - frontier queue length `= 2595`
  - legality summaries `= 841290`
  - retained prefix cache `= 41 groups / 154842 candidates`
  - `remaining_one_algebraic_prunes = 0`
  - `remaining_one_rank_prunes_pre_materialize = 835879`
  - `terminal_summary_build_millis = 6854843`
  - `terminal_materialize_millis = 503`
  - observed RSS `~ 667 MiB`
- New stored intended-profile rerun:
  `runs/codex-claim-release-full-algebraic-v1`
- Against the current full baseline
  `runs/codex-claim-release-full-nu-profile-v1`, the new full rerun kept the
  same honest retained-prefix shape and legality growth at the matching
  checkpoints, but it did not improve them:
  - `prefix_states_explored = 43` at `1703.4s` instead of `1629.6s`
  - `prefix_states_explored = 52` at `2081.4s` instead of `2039.7s`
  - `prefix_states_explored = 59` at `2369.7s` instead of `2367.7s`
  - At those same checkpoints, retained prefix cache stayed
    `39 groups / 144845 candidates` and legality summaries stayed
    `205199`, `246986`, and `279487` respectively.
  - At those same checkpoints, `terminal_summary_build_millis` rose to
    `1602527`, `1962557`, and `2237096` instead of
    `1520539`, `1910369`, and `2221499`.
- At the old full-profile `43`-state wall clock of `1629.6s`, the new full
  rerun was only at `prefix_states_explored = 41` with:
  - frontier queue length `= 2734`
  - legality summaries `= 195913`
  - `remaining_one_algebraic_prunes = 0`
- That full rerun was then manually stopped after enough stored evidence:
  - `prefix_states_explored = 105` at `4271.3s`
  - frontier queue length `= 2670`
  - legality summaries `= 493065`
  - retained prefix cache `= 40 groups / 147639 candidates`
  - `remaining_one_algebraic_prunes = 0`
  - `remaining_one_rank_prunes_pre_materialize = 487580`
  - `terminal_summary_build_millis = 4027549`
  - `terminal_materialize_millis = 490`
  - observed RSS `~ 451 MiB`
- The exact named regressions were re-earned on this desktop in this turn:
  - `engine::tests::claim_remaining_one_algebraic_ceiling_prunes_before_summary_build`
  - `engine::tests::claim_remaining_one_algebraic_ceiling_keeps_reference_step_four_winner_prefix`
  - `engine::tests::online_work_items_cache_exact_filtered_next_clauses`
  - `engine::tests::online_work_items_reuse_full_catalog_when_no_filter_applies`
  - `engine::tests::prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations`
  - `prefix_memo::tests::terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity`
  - `cmd_run::tests::claim_run_persists_live_step_memory_checkpoints_before_acceptance`
- A 2026-03-25 attempted broad claim-test sweep is still not trustworthy
  evidence and should not be repeated casually on this machine:
  - Windows logged `Resource-Exhaustion-Detector` warnings at `2026-03-25
    06:41:02` and `2026-03-25 06:46:00` showing
    `pen_search-28bcbd274e85075a.exe` at about `55.8 GiB` and `57.2 GiB` of
    virtual memory respectively.
  - Windows then logged `BugCheck 0x0000003b` at `2026-03-25 06:25:20` and a
    later unexpected restart at `2026-03-25 06:47:17`.
  - The trigger was the old test shape that bootstrapped and enumerated a full
    step-`4` prefix surface just to find one algebraic-prune witness; that
    setup is now removed from the new targeted regressions.

## Current Read

- The short step-`4` rerun says the algebraic binary can move the narrow
  `until_step = 4` profile faster.
- The intended-profile rerun does not reproduce that win and shows zero live
  algebraic prune hits through `prefix_states_explored = 105`.
- Step `4` exact summary build therefore remains the dominant honest blocker on
  the real profile.
- Memory remains controlled. The new full rerun stayed near `451 MiB` observed
  RSS when manually stopped, far below the old allocator-failure band.
- The algebraic ceiling patch can stay landed as a narrow local slice, but it
  does not currently count as a new full-profile claim-lane baseline.
- The next honest move is context-equivalence quotienting keyed by normalized
  exported type context, not more memory compaction or another blind
  algebraic-only variant.

## Immediate Order

1. Land one narrow context-equivalence quotienting patch for remaining-one
   prefixes before exact summary build.
2. Add quotient-hit telemetry and exact regressions proving equivalent
   contexts collapse without killing the kept winner.
3. Re-earn one stored release `until_step = 4` rerun and compare its stored
   `reports/steps/step-04-live.ndjson` against
   `runs/codex-claim-release-step4-algebraic-v1` and
   `runs/codex-claim-release-full-nu-profile-v1`.
4. Only if that short rerun shows a real stored win should another real
   `desktop_claim_shadow_1h` full-profile rerun happen.
5. Do not rerun the broad `claim_` unit-test filter as a first validation step
   on this desktop again.

## Active Baselines

- Full-profile baseline before delayed summary:
  `runs/codex-claim-release-full-v1a`
- Earlier full-profile baseline:
  `runs/codex-claim-release-full-delayed-summary-v1`
- Current full-profile iteration baseline:
  `runs/codex-claim-release-full-nu-profile-v1`
- Most recent intended-profile evidence that did not advance that baseline:
  `runs/codex-claim-release-full-algebraic-v1`
- Earlier short step-`4` baseline:
  `runs/codex-claim-release-step4-delayed-summary-v1`
- Previous short step-`4` baseline:
  `runs/codex-claim-release-step4-nu-profile-v1`
- Current short step-`4` iteration baseline:
  `runs/codex-claim-release-step4-algebraic-v1`
- Pre-delay materialization baseline:
  `runs/codex-claim-release-step4-telemetry-v1`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged.
- Keep user-facing wording at `bounded live recovery`.
- Keep the claim lane honest about still being guided and not yet fully
  unguided.
- Prefer structural exact bounds over semantic shortcuts.
- Trust stored artifacts over terminal impressions.
