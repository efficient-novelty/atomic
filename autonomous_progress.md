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
  path, and the one-pass `structural_nu` summary-build fast path are all now
  kept on the claim lane.
- A new stored short rerun now exists on that winning binary:
  `runs/codex-claim-release-step4-nu-profile-v1`.
- A new stored intended-profile rerun now exists on that same winning binary:
  `runs/codex-claim-release-full-nu-profile-v1`.
- The current full-profile iteration baseline is now
  `runs/codex-claim-release-full-nu-profile-v1`; the previous full-profile
  baseline is now `runs/codex-claim-release-full-primary-rank-v1`.
- The claim lane has already crossed the memory wall; the real intended profile
  is now compute-bound in step `4`.
- The next likely leverage is not more memory work. It is earlier structural
  pruning before exact remaining-one summary construction starts.
- A local algebraic `nu` ceiling remaining-one pre-summary prune is now landed
  in code together with `remaining_one_algebraic_prunes` telemetry, but no
  stored step-`4` rerun has earned keep on that patch yet.
- The broad `cargo test -p pen-search claim_` sweep was not re-earned in this
  turn on the disclosed desktop: a full-surface claim test run exhausted
  virtual memory hard enough to crash Windows, so current local validation is
  limited to compile coverage plus exact single-test regressions.

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
- Earlier full-profile evidence had already isolated the same phase transition
  more starkly:
  - legality summaries `= 446635`
  - `terminal_summary_build_millis = 4314240`
  - `remaining_one_rank_prunes_pre_materialize = 441140`
  - observed RSS `~ 343 MiB`
  - That older read and the newer `nu`-profile read tell the same story:
    memory is stable, pruning is real, but the expensive proof that a prefix is
    dead still happens too late.
- The new algebraic-ceiling slice is now wired locally in search code:
  - `remaining_one_algebraic_prunes` is recorded in
    `RemainingOneTelemetry` and in claim live-checkpoint telemetry fixtures.
  - The new engine regressions now avoid full step-surface enumeration and
    pass as exact single-test runs:
    `engine::tests::claim_remaining_one_algebraic_ceiling_prunes_before_summary_build`
    and
    `engine::tests::claim_remaining_one_algebraic_ceiling_keeps_reference_step_four_winner_prefix`.
  - The named exact regressions also passed as low-memory single-test runs:
    `engine::tests::online_work_items_cache_exact_filtered_next_clauses`,
    `engine::tests::online_work_items_reuse_full_catalog_when_no_filter_applies`,
    `engine::tests::prefix_queue_prefers_nearer_terminal_and_tighter_cached_continuations`,
    `prefix_memo::tests::terminal_clause_filter_skips_inadmissible_last_clauses_before_connectivity`,
    and
    `cmd_run::tests::claim_run_persists_live_step_memory_checkpoints_before_acceptance`.
- A 2026-03-25 attempted broad claim-test sweep is not trustworthy evidence and
  should not be repeated casually on this machine:
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

- The lane is now in a classic algorithmic phase transition: the old allocator
  cliff is gone and the remaining wall is compute.
- The current pruning logic is working, but the hot telemetry says it still
  fires after too many exact summary builds have already been paid for.
- The high pre-materialize rank-prune totals are therefore good evidence of the
  next design move: shift more killing power ahead of
  `compute_terminal_prefix_completion_summary_from_candidates`.
- The new algebraic ceiling slice is now locally implemented and cheaply
  testable, but it still needs one stored release rerun before it can count as
  a kept claim-lane win.
- The most promising next structural patches are:
  - an algebraic `nu` ceiling for remaining-one prefixes
  - deterministic symmetry breaking for independent sibling clauses
  - if still narrow and honest, a structural-unity forced-bridge prune
- Context-equivalence quotienting and incumbent-first queue ordering remain
  plausible follow-on moves, but they are larger and should wait until the
  first structural culling slice is measured.
- Do not reopen allocator, memory-compaction, frontier-rewrite, compare,
  benchmark, or certification work yet.

## Immediate Order

1. Re-earn one stored release rerun with `until_step = 4` on the locally
   landed algebraic `nu` ceiling patch and compare its
   stored `reports/steps/step-04-live.ndjson` against
   `runs/codex-claim-release-full-nu-profile-v1` and
   `runs/codex-claim-release-step4-nu-profile-v1`.
2. Keep that patch only if matching checkpoints cut
   `terminal_summary_build_millis`, cut repeated legality-summary growth, or
   move materially farther without weakening retained-prefix honesty.
3. Only after that short rerun earns keep should another real
   `desktop_claim_shadow_1h` full-profile rerun happen.
4. If the short rerun still leaves step `4` compute-bound, escalate next to
   context-equivalence quotienting rather than back to memory work.
5. Do not rerun the broad `claim_` unit-test filter as a first validation step
   on this desktop again; keep local validation to compile coverage plus exact
   named tests unless a narrower claim-test shard is introduced.

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
- Keep the claim lane honest about still being guided and not yet fully
  unguided.
- Prefer structural exact bounds over semantic shortcuts.
- Trust stored artifacts over terminal impressions.
