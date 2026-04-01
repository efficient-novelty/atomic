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
  on top of the tiered prefix-side structural-`nu` `lib_refs` work, the
  explicit plateau/fallback kernel split, and the borrowed-primary-rank reuse,
  while also caching each terminal clause's bit cost inside
  `TerminalClauseNuFacts` and backfilling older replay fixtures on load.
  The hot remaining-one summary/materialization path therefore stops
  recursively re-walking the same last-clause expr just to recover
  `bit_kappa_used` on every admitted candidate, while compact remaining-one
  summaries still keep a survivor sketch for incumbent-relevant
  competition-allowed bar-clearers even when multiple primary ranks are live,
  and materialization still reuses that cached sketch without waking the
  dormant general cached-summary reopen path.
- A fresh follow-on slice is now in the repo but still unbenchmarked:
  compact remaining-one no-miss branches now build full accept ranks from a
  prefix-local accept-rank context plus the terminal clause instead of
  reloading a scratch telescope and rescanning the whole terminal telescope
  when they need to break a primary-rank tie.
  That follow-on slice has only cleared claim-focused tests so far and still
  needs replay timing/parity before it earns another intended-profile rerun.

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
- The newly earned capped rerun was then spent on
  `runs/codex-claim-release-full-aggregation-open-band-clause-bit-cost-sidecar-v1`.
  Its nearest stored step-`4` checkpoint to `1200000 ms` landed at:
  - `elapsed_millis = 1192222`
  - `prefix_states_explored = 122`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 109690`
  - `frontier_queue_len = 2653`
  - RSS `= 489394176` bytes
  - `terminal_summary_build_millis = 1184060`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- That rerun did not replace the current gate honestly:
  it kept the same retained-prefix surface and lower RSS, but explored two
  fewer prefixes than `plateau-kernel-split-v1`, left one more frontier item,
  and spent slightly more time in summary build.
- Near-term validation should now try to beat `124` explored prefixes by
  `20` minutes while keeping that no-miss shape honest.

## New Local Reads

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
- Two later local-only compact-summary bookkeeping slices then stayed
  parity-clean in the same claim-focused tests and replay parity, but both
  also landed worse and were dropped instead of kept:
  - a shared compact sketch/best-primary bookkeeping fold across the compact
    summary paths
  - a claim-open-band compact local-state hoist for
    `bound / admitted count / best primary / best rank / sketch` bookkeeping
- The shared compact-bookkeeping fold rereads landed `140129 us`,
  `140565 us`, and `145083 us` total:
  - `140129`: `27648 / 50765 / 20329 / 20751 / 20636`
  - `140565`: `26793 / 58976 / 18071 / 18207 / 18518`
  - `145083`: `31456 / 56870 / 19093 / 18553 / 19111`
- The claim-open-band compact local-state hoist rereads landed `129362 us`,
  `133001 us`, and `164098 us` total:
  - `129362`: `26190 / 45671 / 18214 / 18308 / 20979`
  - `133001`: `24988 / 49889 / 19082 / 18876 / 20166`
  - `164098`: `47503 / 54272 / 21662 / 19399 / 21262`
- These are `24 / 74 / 140 / 332 / 335` in order for each total above.
- Neither later slice re-earned the checked-in `123544 us` replay gate, so
  both code paths were reverted, the benchmark artifact stayed unchanged, and
  no new intended-profile rerun was launched.
- A later narrow slice then cached terminal-clause bit cost inside
  `TerminalClauseNuFacts`, backfilled older replay fixtures on load, and
  reused that sidecar across the remaining-one plateau-kernel and compact
  admitted bit-`kappa` bookkeeping paths instead of recursively walking the
  same clause expr on every admitted candidate.
- Claim-focused validation stayed green while testing the slice:
  - `cargo test -p pen-eval terminal_clause_nu_facts_backfill_missing_bit_cost`
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search cached_terminal_prefix_rank_summary_prunes_without_reopening_completion_summary`
  - `cargo test -p pen-search take_terminal_prefix_completion_summary_removes_cached_payload_after_reuse`
- The release replay harness stayed parity-clean on the stored plateau
  fixtures while the slice was in place.
- The immediate pre-slice local release reread was `134660 us` total:
  - `24`: `30164`
  - `74`: `48407`
  - `140`: `18293`
  - `332`: `18854`
  - `335`: `18942`
- Warm rereads with the bit-cost sidecar reuse landed `133228 us`,
  `124028 us`, `122830 us`, and `122493 us` total:
  - `133228`: `26828 / 49463 / 18923 / 18241 / 19773`
  - `124028`: `24874 / 45415 / 17527 / 18007 / 18205`
  - `122830`: `26271 / 43593 / 17247 / 17245 / 18474`
  - `122493`: `25108 / 43553 / 18100 / 17228 / 18504`
- These are `24 / 74 / 140 / 332 / 335` in order for each total above.
- The refreshed stored benchmark artifact was then rewritten from a later
  under-gate reread at `123148 us` total:
  - `123148`: `25589 / 43365 / 18070 / 17325 / 18799`
- Additional spot-check rereads still bounced around the gate at `124012 us`
  and `126456 us`, so the win is narrow and noisy, but the slice re-earned the
  checked-in `123544 us` replay gate more than once and also beat the
  immediate pre-slice local release reread honestly, so the code stayed
  landed and the lane then spent that earned capped intended-profile rerun.
- That capped intended-profile rerun landed the stored `1192222 ms` /
  `122`-prefix checkpoint on
  `runs/codex-claim-release-full-aggregation-open-band-clause-bit-cost-sidecar-v1`,
  which kept the same retained-prefix story and lower RSS but did not beat the
  current `124`-prefix short-loop gate honestly.
- A fresh local-only follow-on slice now precomputes a prefix-local
  accept-rank context and reuses it in the compact remaining-one no-miss
  branches so the hot best-rank updates stop loading a scratch telescope and
  rescanning the full terminal telescope just to break primary-rank ties.
- Claim-focused validation stayed green for the follow-on slice:
  - `cargo test -p pen-search prefix_clause_acceptance_rank_matches_full_telescope_rank`
  - `cargo test -p pen-search claim_replay_fixture_replays_compact_summary_with_parity`
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search cached_terminal_prefix_rank_summary_prunes_without_reopening_completion_summary`
  - `cargo test -p pen-search take_terminal_prefix_completion_summary_removes_cached_payload_after_reuse`
- Release replay parity and warmed rereads for the new prefix-local
  accept-rank slice are still pending, so no new benchmark artifact or
  intended-profile rerun has been launched on top of it yet.

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
- the terminal-clause bit-cost sidecar cached inside
  `TerminalClauseNuFacts`, with replay-loader backfill for older fixtures, now
  reused across the remaining-one plateau-kernel and compact admitted
  bit-`kappa` bookkeeping paths
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
- The new local evidence is now better but still narrow:
  the landed clause-bit-cost sidecar reuse re-earned the checked-in replay
  gate on warmed release rereads and beat the immediate pre-slice local read,
  while the earlier dropped focus-aligned competition-gate hoist, shared
  compact-bookkeeping fold, and claim-open-band compact local-state hoist all
  stayed parity-clean but landed worse on the stored plateau fixtures.
- The newly spent capped rerun then lost narrowly at `122` explored prefixes
  by `1192222 ms`, so the lane is back in code work and the stored
  `124`-prefix gate still stands.
- The fresh prefix-local accept-rank follow-on slice has only cleared
  claim-focused tests so far, so replay timing/parity must now decide whether
  it deserves another `20`-minute intended-profile rerun.
- The plateau-kernel split still owns the current stored short-loop run to
  beat.
  The lane should keep treating `prefix-local-score-v1` as the long-run
  reference until a later capped rerun replaces the `124`-prefix checkpoint
  honestly.
- The optimization loop now needs shorter, more repeatable intended-profile
  reads.
  We no longer expect the very next slice to beat the full `1095`-prefix stop.
  Instead, each new slice should first try to beat the current short-loop gate
  at `124` explored prefixes.

## Forward Direction

- Keep `prefix-local-score-v1` as the long-run run to beat and
  `plateau-kernel-split-v1` as the current short-loop checkpoint to beat.
- Use a hard 20-minute max intended-profile rerun for the next attempts.
- The newly spent bit-cost-sidecar rerun lost narrowly, so validate the new
  prefix-local accept-rank compact-summary slice on the release replay harness
  before spending another intended-profile rerun.
- Only launch another capped intended-profile rerun if that new slice stays
  replay-parity clean and re-earns the checked-in replay gate honestly.
- If the new slice loses, drop it and stay on another narrow per-admitted
  compact-summary cost slice inside
  `compute_terminal_prefix_completion_summary_from_candidates(...)` rather
  than retrying the dropped focus-aligned competition-gate/payload-mode hoist,
  the dropped shared compact-bookkeeping fold, or the dropped claim-open-band
  compact local-state hoist.
- Do not wake the dormant general cached-summary reopen machinery first.
- Only reopen longer full-profile continuation reads after repeated 20-minute
  wins show that the lane has materially improved.

## Immediate Next Move

1. Keep `prefix-local-score-v1` frozen as the long-run run to beat.
2. Validate the new prefix-local accept-rank compact-summary slice on the
   stored plateau fixtures in release mode first.
3. Keep the slice only if it preserves replay parity and re-earns or beats the
   current checked-in replay total of `123148 us` honestly on warmed rereads.
4. Only then launch one new intended-profile rerun on the current binary for
   `20` minutes max.
5. Record the nearest stored step-`4` checkpoint to `1200000 ms` and compare it
   first against the current short-loop gate:
   `1191562 ms`, `124` explored prefixes, `40 groups / 109690 candidates`,
   `frontier_queue_len = 2651`, RSS `= 495325184`,
   `terminal_summary_build_millis = 1183359`,
   `terminal_summary_admissibility_checks = 0`, and
   `terminal_summary_fallback_connectivity_checks = 0`.
6. If the new slice does not re-earn replay gate honestly, or if the later
   rerun still does not beat that checkpoint honestly, drop back to another
   narrow compact-summary per-admitted cost slice while keeping the dropped
   focus-aligned competition-gate/payload-mode hoist, the dropped shared
   compact-bookkeeping fold, and the dropped claim-open-band compact local-
   state hoist off the table first.
7. Keep broad cleanup, cached-summary reopen wake-up work, contender-rank
   rewrites, connectivity-first/cache-first rewrites, and deterministic
   batched parallel reduction dropped until the short-loop runtime wins become
   strong enough to justify a longer read again.
