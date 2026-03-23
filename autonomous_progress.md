# Autonomous Claim Lane Progress

Last updated: 2026-03-24

This file is the live status read for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
execution slice, [autonomous_plan.md](autonomous_plan.md) for the staged path
to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the
final gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- Delayed materialization and the narrow remaining-one incumbent-primary fast
  path are both now kept on the claim lane.
- A new stored intended-profile rerun now exists on that winning binary:
  `runs/codex-claim-release-full-primary-rank-v1`.
- The current full-profile iteration baseline is now
  `runs/codex-claim-release-full-primary-rank-v1`; the older delayed-summary
  full rerun is now the previous full-profile baseline.
- The real intended profile still stalls in step `4`, so the next live blocker
  remains remaining-one summary-side throughput, not the old allocator cliff
  or a later-step pressure story.

## Latest Evidence

- New stored intended-profile rerun:
  `runs/codex-claim-release-full-primary-rank-v1`
- At matching hot step-`4` checkpoints against
  `runs/codex-claim-release-full-delayed-summary-v1`, the new full rerun kept
  the same honest retained-prefix shape and legality growth but reached those
  checkpoints materially earlier:
  - `prefix_states_explored = 43` at `1964.9s` instead of `3309.4s`
  - `prefix_states_explored = 52` at `2406.1s` instead of `4008.9s`
  - `prefix_states_explored = 59` at `2733.2s` instead of `4529.4s`
  - At those same checkpoints, retained prefix cache stayed
    `39 groups / 144845 candidates` and legality summaries stayed
    `205199`, `246986`, and `279487` respectively.
- Against the short stored winner
  `runs/codex-claim-release-step4-primary-rank-v1`, the real intended profile
  tracked the same win closely:
  - `prefix_states_explored = 43` at `1964.9s` instead of `1960.9s`
  - `prefix_states_explored = 52` at `2406.1s` instead of `2392.5s`
  - `prefix_states_explored = 59` at `2733.2s` instead of `2712.3s`
- By the first stored checkpoints after the old delayed-summary full wall
  clocks, the new full rerun had already moved materially farther:
  - after `3309.4s`, it was at `prefix_states_explored = 71` by `3331.9s`
    instead of `43`
  - after `3936.1s`, it was at `prefix_states_explored = 83` by `3950.1s`
    instead of about `52`
  - after `4529.4s`, it was at `prefix_states_explored = 95` by `4544.8s`
    instead of `59`
- At the manual stop after enough stored evidence, the rerun was still active
  in step `4`:
  - frontier queue length `= 2680`
  - legality summaries `= 446635`
  - retained prefix cache `= 40 groups / 147639 candidates`
  - `remaining_one_materialized = 40`
  - `remaining_one_cached_rank_prunes = 441140`
  - `remaining_one_rank_prunes_pre_materialize = 441140`
  - `remaining_one_rank_prunes_post_materialize = 0`
  - `terminal_summary_build_millis = 4314240`
  - `terminal_materialize_millis = 561`
  - observed RSS `~ 343 MiB`

## Current Read

- The old allocator cliff is not the active blocker on the current binary.
- Delayed materialization remains kept baseline behavior on the real intended
  profile, and the incumbent-primary remaining-one fast path also stays kept.
- The stored full-profile rerun re-earned the real intended-profile read and
  confirmed that the short step-`4` win survives the full profile shape.
- Step `4` is still the dominant blocker on the real profile; no later cost
  center has honestly overtaken it yet.
- The hot cost is still `terminal_summary_build_millis` inside remaining-one
  summary construction; materialization remains small and frontier drainage is
  improving only gradually.
- The next action returns to one more narrow remaining-one summary-side,
  summary-reuse, or earlier incumbent-dominance patch, followed by a stored
  release `until_step = 4` rerun.
- Do not reopen split-handoff, memory-compaction, frontier-rewrite, compare,
  benchmark, or certification work yet.

## Immediate Order

1. Land one more narrow remaining-one summary-build, summary-reuse, or earlier
   incumbent-dominance fast path on the hot step-`4` path.
2. Re-earn one stored release rerun with `until_step = 4` and compare its
   stored `reports/steps/step-04-live.ndjson` against
   `runs/codex-claim-release-full-primary-rank-v1` and
   `runs/codex-claim-release-step4-primary-rank-v1`.
3. Keep that next patch only if matching checkpoints cut
   `terminal_summary_build_millis` or move materially farther without
   weakening retained-prefix honesty.
4. Only after that new short rerun earns keep should another real
   `desktop_claim_shadow_1h` full-profile rerun happen.
5. Compare, benchmark, and certification still remain downstream of that next
   step-`4` win.

## Active Baselines

- Full-profile baseline before delayed summary:
  `runs/codex-claim-release-full-v1a`
- Previous full-profile baseline:
  `runs/codex-claim-release-full-delayed-summary-v1`
- Current full-profile iteration baseline:
  `runs/codex-claim-release-full-primary-rank-v1`
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
