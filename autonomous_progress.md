# Autonomous Claim Lane Progress

Last updated: 2026-03-24

This file is the live status read for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
execution slice, [autonomous_plan.md](autonomous_plan.md) for the staged path
to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the
final gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- Delayed materialization, the narrow remaining-one incumbent-primary fast
  path, and the new one-pass `structural_nu` summary-build fast path are all
  now kept on the claim lane.
- A new stored short rerun now exists on that winning binary:
  `runs/codex-claim-release-step4-nu-profile-v1`.
- A new stored intended-profile rerun now exists on that same winning binary:
  `runs/codex-claim-release-full-nu-profile-v1`.
- The current full-profile iteration baseline is now
  `runs/codex-claim-release-full-nu-profile-v1`; the previous full-profile
  baseline is now `runs/codex-claim-release-full-primary-rank-v1`.
- The real intended profile still stalls in step `4`, so the next live blocker
  remains remaining-one summary-side throughput, not delayed materialization,
  not the old allocator cliff, and not a later-step pressure story.

## Latest Evidence

- New stored short rerun:
  `runs/codex-claim-release-step4-nu-profile-v1`
- At matching hot step-`4` checkpoints against
  `runs/codex-claim-release-step4-primary-rank-v1`, the new short rerun kept
  the same honest retained-prefix shape and legality growth but reached those
  checkpoints materially earlier:
  - `prefix_states_explored = 43` at `1728.7s` instead of `1960.9s`
  - `prefix_states_explored = 52` at `2090.3s` instead of `2392.5s`
  - `prefix_states_explored = 59` at `2360.5s` instead of `2712.3s`
  - At those same checkpoints, retained prefix cache stayed
    `39 groups / 144845 candidates` and legality summaries stayed
    `205199`, `246986`, and `279487` respectively.
  - At those same checkpoints, `terminal_summary_build_millis` fell to
    `1622973`, `1963028`, and `2215871` instead of
    `1854223`, `2264051`, and `2567993`.
- New stored intended-profile rerun:
  `runs/codex-claim-release-full-nu-profile-v1`
- At matching hot step-`4` checkpoints against
  `runs/codex-claim-release-full-primary-rank-v1`, the new full rerun kept the
  same honest retained-prefix shape and legality growth but reached those
  checkpoints materially earlier:
  - `prefix_states_explored = 43` at `1629.6s` instead of `1964.9s`
  - `prefix_states_explored = 52` at `2039.7s` instead of `2406.1s`
  - `prefix_states_explored = 59` at `2367.7s` instead of `2733.2s`
  - At those same checkpoints, retained prefix cache stayed
    `39 groups / 144845 candidates` and legality summaries stayed
    `205199`, `246986`, and `279487` respectively.
  - At those same checkpoints, `terminal_summary_build_millis` fell to
    `1520539`, `1910369`, and `2221499` instead of
    `1857581`, `2277413`, and `2587129`.
- By the old full-profile `43`-state wall clock, the new full rerun had
  already moved materially farther:
  - after `1964.9s`, it was already at `prefix_states_explored = 51` by
    `1977.6s` instead of `43`
  - before the old `52`-state wall clock of `2406.1s`, it had already reached
    `prefix_states_explored = 59` by `2367.7s`
- At the manual stop after enough stored evidence, the new full rerun was
  still active in step `4`:
  - frontier queue length `= 2716`
  - legality summaries `= 279487`
  - retained prefix cache `= 39 groups / 144845 candidates`
  - `remaining_one_materialized = 39`
  - `remaining_one_cached_rank_prunes = 273957`
  - `remaining_one_rank_prunes_pre_materialize = 273957`
  - `remaining_one_rank_prunes_post_materialize = 0`
  - `terminal_summary_build_millis = 2221499`
  - `terminal_materialize_millis = 460`
  - observed RSS `~ 316 MiB`

## Current Read

- The new one-pass `structural_nu` fast path earned keep on both the short
  stored rerun and the real intended-profile stored rerun.
- The current binary no longer needs the old allocator-failure story to explain
  the hot lane.
- Step `4` is still the dominant blocker on the real profile; no later cost
  center has honestly overtaken it yet.
- The hot cost is still `terminal_summary_build_millis` inside remaining-one
  summary construction; materialization remains small and frontier drainage is
  improving only gradually.
- The new full rerun re-earned the real intended-profile read and confirmed
  that the new short step-`4` win survives the full profile shape.
- The next action returns to one more narrow remaining-one summary-side,
  summary-reuse, or earlier incumbent-dominance patch, followed by one more
  stored release `until_step = 4` rerun against the new baselines below.
- Do not reopen split-handoff, memory-compaction, frontier-rewrite, compare,
  benchmark, or certification work yet.

## Immediate Order

1. Land one more narrow remaining-one summary-build, summary-reuse, or earlier
   incumbent-dominance fast path on the hot step-`4` path.
2. Re-earn one stored release rerun with `until_step = 4` and compare its
   stored `reports/steps/step-04-live.ndjson` against
   `runs/codex-claim-release-full-nu-profile-v1` and
   `runs/codex-claim-release-step4-nu-profile-v1`.
3. Keep that next patch only if matching checkpoints cut
   `terminal_summary_build_millis` again or move materially farther without
   weakening retained-prefix honesty.
4. Only after that next short rerun earns keep should another real
   `desktop_claim_shadow_1h` full-profile rerun happen.
5. Compare, benchmark, and certification still remain downstream of that next
   step-`4` win.

## Active Baselines

- Full-profile baseline before delayed summary:
  `runs/codex-claim-release-full-v1a`
- Earlier full-profile baseline:
  `runs/codex-claim-release-full-delayed-summary-v1`
- Previous full-profile baseline:
  `runs/codex-claim-release-full-primary-rank-v1`
- Current full-profile iteration baseline:
  `runs/codex-claim-release-full-nu-profile-v1`
- Earlier short step-`4` baseline:
  `runs/codex-claim-release-step4-delayed-summary-v1`
- Previous short step-`4` baseline:
  `runs/codex-claim-release-step4-primary-rank-v1`
- Current short step-`4` iteration baseline:
  `runs/codex-claim-release-step4-nu-profile-v1`
- Pre-delay materialization baseline:
  `runs/codex-claim-release-step4-telemetry-v1`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged.
- Keep user-facing wording at `bounded live recovery`.
- Trust stored artifacts over terminal impressions.
