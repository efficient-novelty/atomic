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
- The newest landed code slice now keeps the broadened cached survivor sketch
  on top of the tiered prefix-side structural-`nu` `lib_refs` work and the
  explicit plateau/fallback kernel split, while threading borrowed
  primary-rank refs through compact summary bookkeeping so the hot path stops
  cloning the same primary rank into both best-rank tracking and
  survivor-sketch append on every bar-clearer.
  Compact remaining-one summaries still keep a survivor sketch for
  incumbent-relevant competition-allowed bar-clearers even when multiple
  primary ranks are live, and materialization still reuses that cached sketch
  without waking the dormant general cached-summary reopen path.

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

- A follow-on local-only slice then tried hoisting the focus-aligned
  competition gate plus the compact/full payload-mode branch once per
  `compute_terminal_prefix_completion_summary_from_candidates(...)` call on top
  of the landed survivor-sketch, tiered-`lib_refs`, and plateau-kernel work,
  but that code was dropped instead of landed.
- Claim-focused validation stayed green while testing the slice:
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search cached_terminal_prefix_rank_summary_prunes_without_reopening_completion_summary`
  - `cargo test -p pen-search take_terminal_prefix_completion_summary_removes_cached_payload_after_reuse`
- The release replay harness stayed parity-clean on the stored plateau
  fixtures while the hoist was in place.
- The immediate pre-slice local reread was `130405 us` total across the five
  stored surfaces:
  - `24`: `28907`
  - `74`: `47093`
  - `140`: `17896`
  - `332`: `18018`
  - `335`: `18491`
- Warm rereads with the hoist stayed worse at `136040 us`, `137054 us`, and
  `140843 us` total:
  - `136040`: `27713 / 50699 / 18734 / 19345 / 19549`
  - `137054`: `25619 / 50318 / 22239 / 19997 / 18881`
  - `140843`: `28737 / 49206 / 20016 / 20591 / 22293`
  These are `24 / 74 / 140 / 332 / 335` in order.
- The hoist therefore did not re-earn the checked-in `123544 us` replay gate
  and also did not beat the immediate pre-slice local reread honestly, so the
  code was reverted, the benchmark artifact stayed unchanged, and no new
  intended-profile rerun was launched.

## What Stays Landed

- delayed materialization
- the incumbent-primary remaining-one fast path
- the prefix-side single-clause structural-`nu` context reused across
  remaining-one exact scoring, compact-summary build, and compact
  materialization, now with tiered `lib_refs` storage on the hot path
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
- the broadened compact remaining-one survivor sketch on cached summaries for
  incumbent-relevant bar-clearers across both single-primary and multi-primary
  surfaces, now with borrowed primary-rank reuse across compact best-rank
  bookkeeping and survivor-sketch append, while direct compact reopen still
  stays preserved when no cached sketch is available
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
- The old "second primary rank disables the sketch" limitation is now gone:
  multi-primary incumbent-relevant surfaces now reuse cached sketch
  materialization in tests.
- The new local evidence is still mixed:
  the landed borrowed-primary-rank survivor-sketch bookkeeping reuse still
  owns the last directionally-better replay read, while the latest attempted
  focus-aligned competition-gate hoist stayed parity-clean but landed worse
  than the immediate pre-slice local reread on the stored plateau fixtures and
  was therefore dropped.
- Because that replay gate did not improve, the lane has not yet earned
  another `20`-minute intended-profile rerun.
- The plateau-kernel split therefore still owns the only honest short-loop win
  so far.
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
  keep the new multi-primary/incumbent-relevant survivor-sketch coverage plus
  the borrowed-primary-rank bookkeeping reuse, but try a different narrow
  per-admitted compact-summary cost inside
  `compute_terminal_prefix_completion_summary_from_candidates(...)` instead of
  retrying the dropped focus-aligned competition-gate/payload-mode hoist, so
  the open-band replay path can honestly re-earn the checked-in `123544 us`
  replay gate or otherwise reduce exact-`nu` work on the stored plateau
  fixtures before another intended-profile rerun.
- Do not wake the dormant general cached-summary reopen machinery first.
- Only reopen longer full-profile continuation reads after repeated 20-minute
  wins show that the lane has materially improved.

## Immediate Next Move

1. Keep `prefix-local-score-v1` frozen as the long-run run to beat.
2. Reopen code work with the next slice:
   keep the new multi-primary/incumbent-relevant sketch coverage and the new
   borrowed-primary-rank fast path, but cut a different remaining
   compact-summary per-admitted cost on top of the tiered-`lib_refs` and
   plateau/fallback kernel work; do not reopen the dropped focus-aligned
   competition-gate/payload-mode hoist first.
3. After that slice:
   - rerun only the claim-focused tests touched by the change
   - rerun the replay harness in release mode
   - only then launch a new intended-profile rerun for `20` minutes max if
     parity stays clean and the replay gate improves
4. Judge the next local slice first against the checked-in replay gate, then
   against the current short-loop gate:
   `123544 us` total across the five stored replay surfaces, then
   `1191562 ms`, `124` explored prefixes, `40 groups / 109690 candidates`
5. Keep broad cleanup, cached-summary reopen wake-up work, contender-rank
   rewrites, connectivity-first/cache-first rewrites, and deterministic
   batched parallel reduction dropped until the short-loop runtime wins become
   strong enough to justify a longer read again.
