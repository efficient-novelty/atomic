# Autonomous Claim Lane Progress

Last updated: 2026-04-01

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to
signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final
gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current run to beat is now
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`.
- The previous full-profile speed winner was
  `runs/codex-claim-release-full-aggregation-open-band-structural-nu-facts-v1`.
- The previous deeper continuation target was
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`.
- The newest landed code slice is the prefix-local continuation-cone scoring
  pass on the remaining-one hot path.
  It now uses
  `SingleClauseStructuralNuContext` plus
  `TerminalClauseNuFacts`
  to make more bar-clearability and primary-overshoot decisions before scratch
  telescope assembly whenever connectivity needs no fallback and admissibility
  is already known.

## Current Run To Beat

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
- Status:
  manually stopped during step `4`.
  `run.json` still says `status = "running"`,
  `reports/latest.txt` still reflects completed step `3`,
  and `reports/steps/step-05-live.ndjson` is absent.
  The authoritative evidence is
  `reports/steps/step-04-live.ndjson`.
- Final stored read before stopping:
  - `prefix_states_explored = 1095`
  - `prefix_cache_groups = 43`
  - `prefix_cache_candidates = 122481`
  - `elapsed_millis = 10815742`
  - `terminal_summary_build_millis = 10751697`
  - RSS `= 3175555072` bytes
  - `frontier_queue_len = 1680`
- This run materially passed both earlier stored walls:
  - at `576`:
    `5632051 / 5598470` with `43 groups / 122481 candidates`
  - at `1038`:
    `10238225 / 10177832` with `43 groups / 122481 candidates`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The later-surface shape still stayed no-miss on stored evidence:
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- Cached-summary reopen stayed dormant:
  - `remaining_one_materialized_from_cached_summary = 0`
  - `remaining_one_prefixes_seen = 0`
  - `remaining_one_materialized_compact_direct = 43`

## 20-Minute Validation Baseline

- Future intended-profile attempts now stop after `20` minutes max.
- The stored comparison point is the nearest step-`4` checkpoint to
  `1200000 ms` on `prefix-local-score-v1`.
- Current 20-minute baseline:
  - `elapsed_millis = 1203991`
  - `prefix_states_explored = 123`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 109690`
  - `frontier_queue_len = 2652`
  - RSS `= 491208704` bytes
  - `terminal_summary_build_millis = 1196362`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- Near-term validation is now about beating that 20-minute stored read, not
  about matching the full `1095`-prefix stop immediately.

## New Local Read

- The first continuation-cone slice is now landed in code.
  Remaining-one exact bound checks, compact summary build, and compact
  materialization now fast-path prefix-local exact scoring whenever
  connectivity needs no fallback and admissibility is already known from the
  prefix-local surface.
- Claim-focused parity checks stayed green after the slice:
  - `claim_remaining_one_algebraic_ceiling_keeps_reference_step_four_winner_prefix`
  - `claim_terminal_prefix_completion_summary_matches_direct_exact_assessment`
  - `claim_replay_fixture_replays_compact_summary_with_parity`
  - `claim_hoisted_terminal_competition_gate_matches_direct_check`
  - `claim_materialization_reopens_after_cached_pruning_summary`
- The checked-in release replay benchmark on the stored plateau fixtures is now
  `150440 us` total across the five stored surfaces, down from `163951 us`.
  Surface deltas versus the prior checked-in read:
  - `24`: `34319 -> 29887`
  - `74`: `51575 -> 58992`
  - `140`: `27139 -> 20863`
  - `332`: `28566 -> 19680`
  - `335`: `22352 -> 21018`
- The replay read is therefore a real overall local improvement, but not a
  uniform per-surface win yet because `40 groups / 147639 candidates`
  regressed.

## What Stays Landed

- delayed materialization
- the incumbent-primary remaining-one fast path
- the prefix-side single-clause structural-`nu` context reused across
  remaining-one exact scoring, compact-summary build, and compact
  materialization
- the shared terminal-clause connectivity-facts sidecar on the clause catalog
- the shared terminal-clause structural-`nu` facts sidecar threaded through
  the clause catalog, filtered active-window clones, remaining-one
  bound/summary/materialization, and replay fixtures
- the prefix-local continuation-cone score fast path on remaining-one exact
  bound checks, compact summary build, and compact materialization when
  fallback connectivity is not needed
- the compact claim open-band aggregation fast path on the no-evaluations
- the aggregation-side accept-rank short-circuit for primary-dominated
  bar-clearers
- the claim open-band terminal-clause handoff fast path on clause refs
- steady-state scratch-slot `clone_from` reuse on terminal-clause loads
- the boundary-timestamp timing pass on the compact summary kernel
- the deterministic replay harness plus stored retained plateau fixtures and
  benchmark

## Current Diagnosis

- The old early RSS cliff is still gone. This remains a step-`4` throughput
  problem, not a return of the allocator-failure story.
- The latest stopped run proved that the lane can move materially past both the
  old `576` wall and the old `1038` wall, but it still did not finish step `4`.
- The decisive later surfaces remain effectively no-miss surfaces:
  stored later reads keep
  `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0`.
- Aggregation is still the lead measured bucket.
- The optimization loop now needs shorter, more repeatable intended-profile
  reads.
  We no longer expect the very next slice to beat the full `1095`-prefix stop.
  Instead, each new slice should first try to beat the 20-minute stored read at
  `123` explored prefixes.

## Forward Direction

- Keep `prefix-local-score-v1` as the current run to beat.
- Use a hard 20-minute max intended-profile rerun for the next attempts.
- The next code slice is still:
  make `TerminalClauseNuFacts` a mandatory clause-catalog sidecar on every
  winning-path remaining-one evaluation so
  `structural_nu_with_clause_facts(...)`
  stays on the hot path and
  `structural_nu_with_clause(...)`
  stays off it.
- After that, split the explicit no-miss hit-path plateau kernel from the
  general fallback kernel.
- Keep `lib_refs` compression and any tiny survivor sketch work after those two
  slices, not before.
- Only reopen longer full-profile continuation reads after repeated 20-minute
  wins show that the lane has materially improved.

## Immediate Next Move

1. Keep `prefix-local-score-v1` frozen as the current run to beat.
2. Reopen code work with the next slice:
   make `TerminalClauseNuFacts` mandatory on the winning remaining-one hot
   path.
3. After that slice:
   - rerun only the claim-focused tests touched by the change
   - rerun the replay harness in release mode
   - then launch a new intended-profile rerun for `20` minutes max
4. Judge that rerun first against the stored 20-minute baseline:
   `1203991 ms`, `123` explored prefixes, `40 groups / 109690 candidates`
5. Keep broad cleanup, cached-summary reopen wake-up work, contender-rank
   rewrites, connectivity-first/cache-first rewrites, and deterministic
   batched parallel reduction dropped until the short-loop runtime wins become
   strong enough to justify a longer read again.
