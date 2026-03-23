# Autonomous Claim Lane Progress

Last updated: 2026-03-23

This file is the live status read for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
execution slice, [autonomous_plan.md](autonomous_plan.md) for the staged path
to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the
final gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- One new narrow remaining-one incumbent-primary fast path is now landed in
  code and has earned keep on a stored release `until_step = 4` rerun.
- The current full-profile baseline is still
  `runs/codex-claim-release-full-delayed-summary-v1` until the real intended
  profile is rerun on the winning binary.
- The next live blocker is re-earning the real `desktop_claim_shadow_1h`
  full-profile read on that winning binary.
- The delayed-materialization patch is kept. It is no longer speculative.

## Latest Evidence

- New stored step-`4` rerun:
  `runs/codex-claim-release-step4-primary-rank-v1`
- At matching hot step-`4` checkpoints against
  `runs/codex-claim-release-full-delayed-summary-v1`, the new rerun kept the
  same honest retained-prefix shape and legality growth but reached those
  checkpoints materially earlier:
  - `prefix_states_explored = 43` at `1960.9s` instead of `3309.4s`
  - `prefix_states_explored = 52` at `2392.5s` instead of `4008.9s`
  - `prefix_states_explored = 59` at `2712.3s` instead of `4529.4s`
  - At those same checkpoints, retained prefix cache stayed
    `39 groups / 144845 candidates` and legality summaries stayed
    `205199`, `246986`, and `279487` respectively.
- At the old full-profile baseline wall clocks, the new rerun had already
  moved materially farther:
  - by `3309.4s`, it had reached `prefix_states_explored = 72` instead of `43`
  - by `3936.1s`, it had reached `prefix_states_explored = 84` instead of `51`
  - by about `4494.4s`, it had reached `prefix_states_explored = 96`
    instead of `59`
- At that manual stop after enough stored evidence:
  - frontier queue length `= 2679`
  - legality summaries `= 451278`
  - retained prefix cache `= 40 groups / 147639 candidates`
  - `remaining_one_materialized = 40`
  - `remaining_one_cached_rank_prunes = 445784`
  - `remaining_one_rank_prunes_pre_materialize = 445784`
  - `remaining_one_rank_prunes_post_materialize = 0`
  - `terminal_summary_build_millis = 4262989`
  - `terminal_materialize_millis = 545`
  - observed RSS `~ 259 MiB`

## Current Read

- The old allocator cliff is not the active blocker on the current binary.
- Delayed materialization remains kept baseline behavior on the real intended
  profile.
- The new incumbent-primary remaining-one fast path also earned keep on stored
  release step-`4` telemetry.
- The stored rerun shows lower `terminal_summary_build_millis` at matching
  checkpoints and much farther progress at the same wall clock without
  weakening retained-prefix honesty.
- The next action is now the real `desktop_claim_shadow_1h` full-profile rerun
  on this winning binary, not another step-`4` micro-patch first.
- Do not reopen split-handoff, memory-compaction, frontier-rewrite, compare,
  benchmark, or certification work before that full-profile read exists.

## Immediate Order

1. Rerun the real `desktop_claim_shadow_1h` profile on the winning binary from
   `runs/codex-claim-release-step4-primary-rank-v1`.
2. Inspect the stored full-profile artifacts, especially
   `reports/steps/step-04-live.ndjson`, and decide whether step `4` is still
   the blocker or a later cost center has honestly taken over.
3. Only after that better full-profile rerun exists should compare,
   benchmark, and certification move again.

## Active Baselines

- Full-profile baseline before delayed summary:
  `runs/codex-claim-release-full-v1a`
- Current full-profile baseline:
  `runs/codex-claim-release-full-delayed-summary-v1`
- Previous short step-`4` baseline:
  `runs/codex-claim-release-step4-delayed-summary-v1`
- Current short step-`4` iteration baseline:
  `runs/codex-claim-release-step4-primary-rank-v1`
- Pre-delay materialization baseline:
  `runs/codex-claim-release-step4-telemetry-v1`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged.
- Keep user-facing wording at `bounded live recovery`.
- Trust stored artifacts over terminal impressions.
