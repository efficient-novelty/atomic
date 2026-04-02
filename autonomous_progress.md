# Autonomous Claim Lane Progress

Last updated: 2026-04-02

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to
signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final
gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current later-wall continuation winner through the stored `576` read is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`.
- The current farthest stored step-`4` continuation to beat is still
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`.
- The previous full-profile speed winner was
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`.
- The earlier full-profile speed winner before that was
  `runs/codex-claim-release-full-aggregation-open-band-structural-nu-facts-v1`.
- The previous deeper continuation target was
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`.
- The current winning stack now keeps the broadened cached survivor sketch on
  top of the tiered prefix-side structural-`nu` `lib_refs` work, the explicit
  plateau/fallback kernel split, the borrowed-primary-rank reuse, the cached
  terminal-clause bit-cost sidecar inside `TerminalClauseNuFacts`, the newer
  prefix-local accept-rank context, and the newer clause-side accept-rank
  facts reuse for compact remaining-one no-miss branches.
  The hot remaining-one summary/materialization path therefore avoids both
  recursively re-walking the same last-clause expr just to recover
  `bit_kappa_used`, reloading a scratch telescope plus the full terminal
  telescope just to break a primary-rank tie, and rescanning the same
  terminal clause expr for structural signals and ref sets after the compact
  path has already decided the clause is worth a full accept-rank build,
  while cached multi-primary survivor-sketch materialization still stays
  landed without waking the dormant general cached-summary reopen path.

## Current Run To Beat

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`
- Status:
  manually stopped during step `4`.
  `run.json` still says `status = "running"`,
  `reports/latest.txt` still reflects completed step `3`,
  and `reports/steps/step-05-live.ndjson` is absent.
  The authoritative evidence is
  `reports/steps/step-04-live.ndjson`.
- Final stored read before stopping:
  - `prefix_states_explored = 578`
  - `prefix_cache_groups = 43`
  - `prefix_cache_candidates = 29809`
  - `elapsed_millis = 5015917`
  - `terminal_summary_build_millis = 4987295`
  - RSS `= 1744551936` bytes
  - `frontier_queue_len = 2197`
- This run materially passed every earlier stored wall through `576`:
  - at `140`:
    `1188340 / 1181188` with `41 groups / 29249 candidates`
  - at `163`:
    `1381425 / 1373168` with `41 groups / 29249 candidates`
  - at `332`:
    `2853118 / 2836752` with `42 groups / 29529 candidates`
  - at `335`:
    `2879368 / 2862850` with `43 groups / 29809 candidates`
  - at `408`:
    `3511930 / 3491831` with `43 groups / 29809 candidates`
  - at `437`:
    `3770964 / 3749419` with `43 groups / 29809 candidates`
  - at `454`:
    `3922561 / 3900177` with `43 groups / 29809 candidates`
  - at `484`:
    `4183978 / 4160100` with `43 groups / 29809 candidates`
  - at `576`:
    `4997082 / 4968579` with `43 groups / 29809 candidates`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The later-surface shape still stayed no-miss on stored evidence:
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- Cached-summary reopen stayed dormant:
  - `remaining_one_materialized_from_cached_summary = 43`
  - `remaining_one_prefixes_seen = 0`
  - `remaining_one_materialized_compact_direct = 0`
- `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
  still remains the farthest stored step-`4` continuation behind it:
  it stopped at `1095` explored prefixes after `10815742 ms`.

## 20-Minute Validation Gate

- Capped intended-profile validation reruns still stop after `20` minutes max
  unless the lane explicitly spends another earned longer continuation.
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
- The plateau-kernel split remains the first honest short-loop win over
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
- The newest landed slice now owns the best short-loop stored read:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`
  - `elapsed_millis = 1191501`
  - `prefix_states_explored = 139`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 28438`
  - `frontier_queue_len = 2636`
  - RSS `= 453021696` bytes
  - `terminal_summary_build_millis = 1183915`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- That rerun is the new honest short-loop gate:
  four more explored prefixes than `prefix-accept-rank-context-v1` by the
  same 20-minute window, the same retained-prefix surface, a shorter frontier,
  and still no fallback connectivity or admissibility work, although RSS and
  summary-build time both rose slightly.
- Near-term validation should now treat `139` explored prefixes by `20`
  minutes as the current short-loop checkpoint while treating
  `clause-accept-rank-facts-long-rerun-v1` as the current later-wall
  continuation reference through `576`.

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
- A fresh follow-on slice then precomputed a prefix-local
  accept-rank context and reused it in the compact remaining-one no-miss
  branches so the hot best-rank updates stop loading a scratch telescope and
  rescanning the full terminal telescope just to break primary-rank ties.
- Claim-focused validation stayed green for that slice:
  - `cargo test -p pen-search prefix_clause_acceptance_rank_matches_full_telescope_rank`
  - `cargo test -p pen-search claim_replay_fixture_replays_compact_summary_with_parity`
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search cached_terminal_prefix_rank_summary_prunes_without_reopening_completion_summary`
  - `cargo test -p pen-search take_terminal_prefix_completion_summary_removes_cached_payload_after_reuse`
- The release replay harness stayed parity-clean on the stored plateau
  fixtures while the slice was in place.
- The checked-in replay gate before the slice was `123148 us` total.
- The first release build-and-read landed `100403 us` total:
  - `100403`: `19999 / 26839 / 17899 / 19201 / 16465`
- Warm rereads with the prefix-local accept-rank context landed `98385 us`,
  `103447 us`, `111079 us`, `94576 us`, and `96905 us` total:
  - `98385`: `20078 / 27629 / 17790 / 17559 / 15329`
  - `103447`: `22038 / 28752 / 17325 / 17328 / 18004`
  - `111079`: `19201 / 41528 / 17876 / 17231 / 15243`
  - `94576`: `17963 / 26255 / 17118 / 17131 / 16109`
  - `96905`: `19095 / 28145 / 17243 / 17226 / 15196`
- These are `24 / 74 / 140 / 332 / 335` in order.
- Every warmed reread beat the checked-in `123148 us` replay gate honestly, so
  the slice stayed landed and the refreshed stored benchmark artifact was
  rewritten from a later under-gate reread at `102513 us` total:
  - `102513`: `23164 / 27019 / 17767 / 19272 / 15291`
- The earned capped intended-profile rerun was then spent on
  `runs/codex-claim-release-full-aggregation-open-band-prefix-accept-rank-context-v1`.
- Its nearest stored step-`4` checkpoint to `1200000 ms` landed at:
  - `elapsed_millis = 1190946`
  - `prefix_states_explored = 135`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 28438`
  - `frontier_queue_len = 2640`
  - RSS `= 445005824` bytes
  - `terminal_summary_build_millis = 1183014`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- That rerun replaced the current short-loop gate honestly:
  same retained-group count, eleven more explored prefixes than
  `plateau-kernel-split-v1`, a much smaller retained candidate surface, a
  shorter frontier, lower RSS, slightly lower summary-build time, and still
  no fallback connectivity or admissibility work.
- Longer continuation reads still remain gated behind repeated 20-minute wins,
  so the loop is back in code work with
  `prefix-accept-rank-context-v1` as the short-loop checkpoint to beat.
- A fresh narrow slice then precomputed clause-side accept-rank facts inside
  `TerminalClauseNuFacts` and reused them in the compact remaining-one no-miss
  branches so hot best-rank tie updates stop rescanning the same terminal
  clause expr for structural signals and ref sets once the compact path has
  already decided the clause is worth a full accept-rank build.
- Claim-focused validation stayed green for that slice:
  - `cargo test -p pen-eval terminal_clause_nu_facts_backfill_missing_bit_cost`
  - `cargo test -p pen-search prefix_clause_acceptance_rank_matches_full_telescope_rank`
  - `cargo test -p pen-search claim_replay_fixture_replays_compact_summary_with_parity`
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search cached_terminal_prefix_rank_summary_prunes_without_reopening_completion_summary`
  - `cargo test -p pen-search take_terminal_prefix_completion_summary_removes_cached_payload_after_reuse`
- The release replay harness stayed parity-clean on the stored plateau
  fixtures while the slice was in place.
- The checked-in replay gate before the slice was `102513 us` total.
- The first release build-and-read landed `90263 us` total:
  - `90263`: `16928 / 22945 / 17849 / 17759 / 14782`
- Warm rereads with the clause-side accept-rank facts landed `90614 us`,
  `98305 us`, and `88197 us` total:
  - `90614`: `16795 / 24703 / 17110 / 17247 / 14759`
  - `98305`: `20715 / 26426 / 18860 / 17740 / 14564`
  - `88197`: `16135 / 22658 / 17127 / 17735 / 14542`
- These are `24 / 74 / 140 / 332 / 335` in order.
- Every warmed reread beat the checked-in `102513 us` replay gate honestly, so
  the slice stayed landed and the refreshed stored benchmark artifact was
  rewritten from a later under-gate reread at `88197 us` total:
  - `88197`: `16135 / 22658 / 17127 / 17735 / 14542`
- The earned capped intended-profile rerun was then spent on
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`.
- Its nearest stored step-`4` checkpoint to `1200000 ms` landed at:
  - `elapsed_millis = 1191501`
  - `prefix_states_explored = 139`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 28438`
  - `frontier_queue_len = 2636`
  - RSS `= 453021696` bytes
  - `terminal_summary_build_millis = 1183915`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- That rerun replaced the current short-loop gate honestly:
  same retained-prefix surface, four more explored prefixes than
  `prefix-accept-rank-context-v1`, a shorter frontier, and still no fallback
  connectivity or admissibility work, although RSS and summary-build time both
  rose slightly.
- The lane now has repeated honest 20-minute wins on the same retained-prefix
  surface (`135` then `139`), so a longer intended-profile continuation is no
  longer blocked on short-loop proof alone.
- A fresh longer intended-profile rerun was then launched as
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`
  because neither `clause-accept-rank-facts-v1` nor the stored inspect surface
  exposed a resumable step-`4` frontier generation, so `pen-cli resume` would
  only have replayed step `4` from the step-`3` checkpoint instead of
  continuing the in-flight frontier honestly.
- The rerun reused the same stored config and the same release binary hash
  (`278c311ddf5e416b09d24923dc392388aaf5817c65f0c60f856ebde7466140a5`)
  as the earlier short-loop winner.
- It then beat `prefix-local-score-v1` honestly at every matched later
  checkpoint through `576` while keeping the same no-miss shape and the same
  frontier queue lengths at the matched prefix counts, but on a much smaller
  retained candidate surface:
  - `140`: `1188340 / 1181188` with `41 groups / 29249 candidates`,
    `frontier_queue_len = 2635`, RSS `= 459980800`
  - `163`: `1381425 / 1373168` with `41 groups / 29249 candidates`,
    `frontier_queue_len = 2612`, RSS `= 528904192`
  - `332`: `2853118 / 2836752` with `42 groups / 29529 candidates`,
    `frontier_queue_len = 2443`, RSS `= 1022218240`
  - `335`: `2879368 / 2862850` with `43 groups / 29809 candidates`,
    `frontier_queue_len = 2440`, RSS `= 1030688768`
  - `408`: `3511930 / 3491831` with `43 groups / 29809 candidates`,
    `frontier_queue_len = 2367`, RSS `= 1247543296`
  - `437`: `3770964 / 3749419` with `43 groups / 29809 candidates`,
    `frontier_queue_len = 2338`, RSS `= 1330483200`
  - `454`: `3922561 / 3900177` with `43 groups / 29809 candidates`,
    `frontier_queue_len = 2321`, RSS `= 1379835904`
  - `484`: `4183978 / 4160100` with `43 groups / 29809 candidates`,
    `frontier_queue_len = 2291`, RSS `= 1468563456`
  - `576`: `4997082 / 4968579` with `43 groups / 29809 candidates`,
    `frontier_queue_len = 2199`, RSS `= 1738993664`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The rerun was then manually stopped at `578` explored prefixes with
  `elapsed_millis = 5015917`, `terminal_summary_build_millis = 4987295`,
  `frontier_queue_len = 2197`, RSS `= 1744551936`, and still
  `0` fallback/admissibility checks.
- No step-`4` frontier artifacts were persisted on that rerun either, so a
  future longer continuation still needs another fresh rerun rather than a
  true frontier-backed resume.

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
- the prefix-local accept-rank context reused across compact remaining-one
  no-miss branches so best-rank tie updates stop rebuilding a scratch
  telescope and rescanning the full terminal telescope
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
- The latest stopped rerun proved that the newer short-loop gains do move the
  later intended-profile walls honestly through `576`, but the lane still did
  not reach the older `1038` wall and still did not finish step `4`.
- The decisive later surfaces remain effectively no-miss surfaces:
  stored later reads keep
  `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0`.
- Aggregation is still the lead measured bucket.
- The old "second primary rank disables the sketch" limitation is now gone:
  multi-primary incumbent-relevant surfaces now reuse cached sketch
  materialization in tests.
- The new local evidence is now materially better:
  the landed prefix-local accept-rank context re-earned the replay gate on
  every warmed release reread, refreshed the checked-in replay benchmark to
  `102513 us`, and then produced a stronger stored 20-minute checkpoint, while
  the newer clause-side accept-rank facts reuse then pushed the refreshed
  replay benchmark down again to `88197 us` and replaced the short-loop gate
  at `139` explored prefixes by the same 20-minute window.
- The newest capped rerun therefore adds a second honest short-loop win on the
  same retained-prefix surface even though the lane still did not finish
  step `4`.
- Repeated 20-minute wins now exist, and the reopened longer rerun has now
  shown that those gains also move the later intended-profile walls honestly
  through `576`.
- The clause-accept-rank-facts slice now owns the current stored short-loop
  run to beat, while
  `clause-accept-rank-facts-long-rerun-v1` now owns the nearer later-wall
  continuation reference through `576`.
  `prefix-local-score-v1` still remains the farthest stored step-`4`
  continuation to beat at `1095`, so the next question is whether the newer
  compact retained surface can also re-earn `1038/1095` honestly or reach
  step `5`.
- Because neither the original short-loop winner nor the newer longer rerun
  persisted a resumable step-`4` frontier generation, the next continuation
  still needs another fresh intended-profile rerun rather than `pen-cli resume`
  first.
- The optimization loop now needs shorter, more repeatable intended-profile
  reads.
  The next slice no longer needs to prove `140/163`, `332/335`,
  `408/437/454/484`, or `576` again from scratch.
  Instead, near-term runtime work should keep the `139`-prefix short-loop gate
  honest while pushing the later continuation farther toward `1038/1095` or
  step `5`.

## Forward Direction

- Keep `clause-accept-rank-facts-v1` as the current short-loop checkpoint to
  beat.
- Keep `clause-accept-rank-facts-long-rerun-v1` as the current later-wall
  continuation reference through `576`.
- Keep `prefix-local-score-v1` as the farthest stored step-`4` continuation to
  beat at `1038/1095` until a later rerun replaces it honestly or reaches
  step `5`.
- The clause-side accept-rank-facts slice has now produced repeated honest
  20-minute wins and one honest longer rerun through `576`, so the next move
  is another fresh longer intended-profile rerun on the same binary before
  spending another code slice.
- Keep the refreshed replay benchmark gate at `88197 us` for later code work.
- If that next fresh rerun cannot re-earn the new `576` continuation surface
  honestly or loses the no-miss retained-prefix story before `1038`, drop back
  to another narrow per-admitted compact-summary cost slice inside
  `compute_terminal_prefix_completion_summary_from_candidates(...)` rather
  than retrying the dropped focus-aligned competition-gate/payload-mode hoist,
  the dropped shared compact-bookkeeping fold, or the dropped claim-open-band
  compact local-state hoist.
- Do not wake the dormant general cached-summary reopen machinery first.
- A further longer full-profile rerun is now justified because the first fresh
  longer continuation already moved the later stored walls honestly through
  `576`.

## Immediate Next Move

1. Keep `clause-accept-rank-facts-v1` frozen as the current short-loop gate
   to beat.
2. Keep
   `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`
   frozen as the current later-wall continuation reference through `576`.
3. Keep `prefix-local-score-v1` frozen as the farthest stored step-`4`
   continuation target to beat at `1038/1095`.
4. Launch one fresh intended rerun on the same release binary and config
   rather than using `pen-cli resume` first, because neither the earlier
   short-loop winner nor the newer longer rerun stored a resumable step-`4`
   frontier generation.
5. Let that fresh rerun run materially past the new `576` reference and then
   inspect matched stored step-`4` checkpoints in this order:
   `576`, then `1038`, then `1095`, then step `5`.
6. Keep comparing the current `20`-minute gate first against
   `1191501 ms`, `139` explored prefixes, `40 groups / 28438 candidates`,
   `frontier_queue_len = 2636`, RSS `= 453021696`,
   `terminal_summary_build_millis = 1183915`,
   `terminal_summary_admissibility_checks = 0`, and
   `terminal_summary_fallback_connectivity_checks = 0`.
7. Keep comparing the later continuation first against the new `576` read from
   `clause-accept-rank-facts-long-rerun-v1`
   (`4997082 / 4968579`, `43 groups / 29809 candidates`,
   `frontier_queue_len = 2199`, RSS `= 1738993664`, and still
   `0` fallback/admissibility checks), then against the older
   `prefix-local-score-v1` `1038/1095` walls behind it.
8. If the next fresh rerun cannot keep that `576` surface honestly or cannot
   move materially toward `1038`, drop back to another narrow compact-summary
   per-admitted cost slice while keeping the dropped
   focus-aligned competition-gate/payload-mode hoist, the dropped shared
   compact-bookkeeping fold, and the dropped claim-open-band compact local-
   state hoist off the table first.
9. Keep broad cleanup, cached-summary reopen wake-up work, contender-rank
   rewrites, connectivity-first/cache-first rewrites, and deterministic
   batched parallel reduction dropped until the longer continuation proves the
   newer short-loop win scales past the next stored walls.
