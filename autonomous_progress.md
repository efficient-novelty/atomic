# Autonomous Claim Lane Progress

Last updated: 2026-03-23

This file is the live status read for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
execution slice, [autonomous_plan.md](autonomous_plan.md) for the staged path
to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the
final gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- The current blocker is still step-`4` throughput on the real
  `desktop_claim_shadow_1h` profile.
- The latest intended-profile evidence is
  `runs/codex-claim-release-full-delayed-summary-v1`.
- The delayed-materialization patch is kept. It is no longer speculative.

## Latest Evidence

- Compared with `runs/codex-claim-release-full-v1a`, the current intended
  profile baseline reached the same hot step-`4` checkpoints materially
  earlier:
  - `prefix_states_explored = 24` at `1854.5s` instead of `2000.8s`
  - `prefix_states_explored = 39` at `3010.8s` instead of `3440.6s`
  - `prefix_states_explored = 43` at `3309.4s` instead of `3844.7s`
- At about the old run's last stored wall clock (`3936.1s`), the new rerun had
  already reached `prefix_states_explored = 52` instead of `44`.
- At the manual stop checkpoint on
  `runs/codex-claim-release-full-delayed-summary-v1`:
  - `prefix_states_explored = 59`
  - elapsed time `= 4529.4s`
  - frontier queue length `= 2716`
  - legality summaries `= 279487`
  - retained prefix cache `= 39 groups / 144845 candidates`
  - `remaining_one_materialized = 39`
  - `remaining_one_rank_prunes_pre_materialize = 273957`
  - `remaining_one_rank_prunes_post_materialize = 0`
  - `remaining_one_cached_bound_hits = 0`
  - `terminal_summary_build_millis = 4387822`
  - `terminal_materialize_millis = 527`
  - observed RSS `~ 266 MiB`

## Current Read

- The old allocator cliff is not the active blocker on the current binary.
- Delayed materialization earned keep on the real intended profile, not only
  on a short step-`4` probe.
- Materialization itself is no longer the dominant cost center.
- The dominant step-`4` cost is now remaining-one terminal-summary
  construction.
- The next patch should target cheaper summary build or an earlier
  incumbent-based kill before summary build starts.
- Do not reopen split-handoff, memory-compaction, frontier-rewrite, compare,
  benchmark, or certification work first.

## Immediate Order

1. Patch one narrow remaining-one summary-build or earlier-incumbent path.
2. Re-earn one stored release `until_step = 4` rerun on that patch.
3. Compare its `reports/steps/step-04-live.ndjson` against:
   `runs/codex-claim-release-full-delayed-summary-v1`,
   `runs/codex-claim-release-step4-delayed-summary-v1`, and
   `runs/codex-claim-release-step4-telemetry-v1`.
4. Only if the stored step-`4` telemetry improves should the real
   `desktop_claim_shadow_1h` profile run again.
5. Only after a better full-profile rerun exists should compare, benchmark,
   and certification move again.

## Active Baselines

- Full-profile baseline before delayed summary:
  `runs/codex-claim-release-full-v1a`
- Current full-profile baseline:
  `runs/codex-claim-release-full-delayed-summary-v1`
- Current short step-`4` iteration baseline:
  `runs/codex-claim-release-step4-delayed-summary-v1`
- Pre-delay materialization baseline:
  `runs/codex-claim-release-step4-telemetry-v1`

## Guardrails

- Keep `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` unchanged.
- Keep user-facing wording at `bounded live recovery`.
- Trust stored artifacts over terminal impressions.
