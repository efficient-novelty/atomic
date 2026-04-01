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
- The current long-run run to beat is
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`.
- The previous full-profile speed winner was
  `runs/codex-claim-release-full-aggregation-open-band-structural-nu-facts-v1`.
- The previous deeper continuation target was
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`.
- The newest landed code slice now splits the step-`4` remaining-one kernel
  into an explicit true no-miss hit-path plateau kernel and a general fallback
  kernel.
  Exact bound screening, compact summary build, and compact materialization now
  share that split while keeping the mandatory
  `TerminalClauseNuFacts`
  sidecar on the winning path.

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

## 20-Minute Validation Gate

- Future intended-profile attempts now stop after `20` minutes max.
- `prefix-local-score-v1` remains the long-run reference.
  Its nearest stored step-`4` checkpoint to `1200000 ms` is still the original
  short-loop gate that later slices had to beat:
  - `elapsed_millis = 1203991`
  - `prefix_states_explored = 123`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 109690`
  - `frontier_queue_len = 2652`
  - RSS `= 491208704` bytes
  - `terminal_summary_build_millis = 1196362`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- The newest landed slice now owns the best short-loop stored read:
  `runs/codex-claim-release-full-aggregation-open-band-plateau-kernel-split-v1`
  - `elapsed_millis = 1191562`
  - `prefix_states_explored = 124`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 109690`
  - `frontier_queue_len = 2651`
  - RSS `= 495325184` bytes
  - `terminal_summary_build_millis = 1183359`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- That run is the first honest short-loop win over
  `prefix-local-score-v1`:
  same retained-prefix surface, one more explored prefix, one shorter frontier
  queue, lower summary-build time, and still no fallback connectivity or
  admissibility work.
- Near-term validation should now try to beat `124` explored prefixes by
  `20` minutes while keeping that no-miss shape honest.

## New Local Read

- The explicit no-miss plateau-kernel split is now landed on top of the
  mandatory-`TerminalClauseNuFacts` winner.
  Remaining-one exact bound checks, compact summary build, and compact
  materialization now route candidates that stay on cached
  `KeepWithoutFallback`
  evidence through one shared plateau kernel and reserve the general fallback
  kernel for candidates that really need extra connectivity or admissibility
  work.
- Claim-focused parity checks stayed green after the slice:
  - `cargo test -p pen-search claim_`
- The checked-in release replay benchmark on the stored plateau fixtures is now
  `145248 us` total across the five stored surfaces, down from `147912 us`.
  Surface deltas versus the prior checked-in read:
  - `24`: `29557 -> 33376`
  - `74`: `54615 -> 50232`
  - `140`: `21011 -> 21316`
  - `332`: `20884 -> 20210`
  - `335`: `21845 -> 20114`
- The replay read is therefore a real overall local improvement, but not a
  uniform per-surface win yet because surfaces `24` and `140` regressed.
- The capped intended-profile contender
  `runs/codex-claim-release-full-aggregation-open-band-plateau-kernel-split-v1`
  was manually stopped at the `20` minute cap during step `4`.
  `run.json` still says `status = "running"` and
  `reports/latest.txt` still reflects completed step `3`;
  the authoritative evidence is
  `reports/steps/step-04-live.ndjson`.
- Its nearest stored read to `20` minutes was:
  - `elapsed_millis = 1191562`
  - `prefix_states_explored = 124`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 109690`
  - `frontier_queue_len = 2651`
  - RSS `= 495325184` bytes
  - `terminal_summary_build_millis = 1183359`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- That read kept the retained-prefix story honest and is now the first
  short-loop win, because it explored `124` prefixes instead of the baseline
  `123` while keeping the same `40 groups / 109690 candidates` surface.

## What Stays Landed

- delayed materialization
- the incumbent-primary remaining-one fast path
- the prefix-side single-clause structural-`nu` context reused across
  remaining-one exact scoring, compact-summary build, and compact
  materialization
- the shared terminal-clause connectivity-facts sidecar on the clause catalog
- the shared terminal-clause structural-`nu` facts sidecar threaded through
  the clause catalog, filtered active-window clones, remaining-one
  bound/summary/materialization, and replay fixtures, now mandatory on the
  winning remaining-one path
- the prefix-local continuation-cone score fast path on remaining-one exact
  bound checks, compact summary build, and compact materialization when
  fallback connectivity is not needed
- the explicit remaining-one no-miss hit-path plateau kernel shared across
  bound screening, compact summary build, and compact materialization, with the
  general fallback kernel reserved for true connectivity or admissibility
  misses
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
- The latest plateau-kernel contender preserved the same `40 groups / 109690`
  retained-prefix surface, improved overall replay time again, and produced the
  first honest short-loop win at `124` explored prefixes by `20` minutes.
- That win is still only one short-loop beat.
  The lane should keep treating `prefix-local-score-v1` as the long-run
  reference until later slices repeat the 20-minute win and then reopen the
  deeper continuation honestly.
- The optimization loop now needs shorter, more repeatable intended-profile
  reads.
  We no longer expect the very next slice to beat the full `1095`-prefix stop.
  Instead, each new slice should first try to beat the current short-loop gate
  at `124` explored prefixes.

## Forward Direction

- Keep `prefix-local-score-v1` as the long-run run to beat and
  `plateau-kernel-split-v1` as the current short-loop checkpoint to beat.
- Use a hard 20-minute max intended-profile rerun for the next attempts.
- The next code slice is now:
  compress `lib_refs` inside
  `SingleClauseStructuralNuContext`
  while keeping the new plateau/fallback split and the mandatory
  `TerminalClauseNuFacts` sidecar on the winning path.
- Keep any tiny survivor sketch work after the `lib_refs` slice, not before.
- Only reopen longer full-profile continuation reads after repeated 20-minute
  wins show that the lane has materially improved.

## Immediate Next Move

1. Keep `prefix-local-score-v1` frozen as the long-run run to beat.
2. Reopen code work with the next slice:
   compress `lib_refs` inside
   `SingleClauseStructuralNuContext`
   without undoing the new plateau/fallback kernel split.
3. After that slice:
   - rerun only the claim-focused tests touched by the change
   - rerun the replay harness in release mode
   - then launch a new intended-profile rerun for `20` minutes max
4. Judge that rerun first against the current short-loop gate:
   `1191562 ms`, `124` explored prefixes, `40 groups / 109690 candidates`
5. Keep broad cleanup, cached-summary reopen wake-up work, contender-rank
   rewrites, connectivity-first/cache-first rewrites, and deterministic
   batched parallel reduction dropped until the short-loop runtime wins become
   strong enough to justify a longer read again.
