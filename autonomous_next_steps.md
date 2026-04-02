# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-02

This note is the exact next work order for `desktop_claim_shadow`.

## Keep Fixed

- Keep the current short baseline:
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Keep the current later-wall continuation reference through `576`:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`
- Keep the current farthest stored step-`4` continuation to beat:
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
- Keep the current best short-loop checkpoint to beat:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`
- Keep the replay harness corpus and benchmark files:
  - `tests/fixtures/claim_runtime/remaining_one_plateau_fixtures.json`
  - `tests/fixtures/claim_runtime/remaining_one_plateau_benchmark.json`
- Keep the current 20-minute intended-profile validation target from the stored
  step-`4` stream of `clause-accept-rank-facts-v1`:
  - `elapsed_millis = 1191501`
  - `prefix_states_explored = 139`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 28438`
  - `frontier_queue_len = 2636`
  - RSS `= 453021696` bytes
  - `terminal_summary_build_millis = 1183915`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- Keep the current matched later-wall continuation target from the stored
  step-`4` stream of `clause-accept-rank-facts-long-rerun-v1`:
  - `elapsed_millis = 4997082`
  - `prefix_states_explored = 576`
  - `prefix_cache_groups = 43`
  - `prefix_cache_candidates = 29809`
  - `frontier_queue_len = 2199`
  - RSS `= 1738993664` bytes
  - `terminal_summary_build_millis = 4968579`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`

## Current Read

- The follow-on survivor-sketch bookkeeping slice remains landed on top of the
  broadened incumbent-relevant sketch coverage, the tiered `lib_refs` work,
  the explicit no-miss plateau-kernel split, and the mandatory
  `TerminalClauseNuFacts` winner.
  Compact claim summaries still keep a survivor sketch for competition-allowed
  bar-clearers that can still beat the current incumbent even when multiple
  primary ranks are live, and still reuse it during materialization without
  waking the dormant general cached-summary reopen path, while the hot compact
  summary path still threads borrowed primary-rank refs through best-rank
  tracking and sketch append instead of cloning the same primary-rank payload
  twice per bar-clearer.
- Claim-focused validation stayed green:
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-search cached_terminal_prefix_rank_summary_prunes_without_reopening_completion_summary`
  - `cargo test -p pen-search take_terminal_prefix_completion_summary_removes_cached_payload_after_reuse`
- The release replay harness stayed parity-clean on the stored plateau
  fixtures.
- A follow-on local-only attempt then hoisted the focus-aligned competition
  gate plus the compact/full payload-mode branch once per
  `compute_terminal_prefix_completion_summary_from_candidates(...)` call, but
  that code was dropped after replay validation.
- The immediate pre-slice local reread was `130405 us` total:
  - `24`: `28907`
  - `74`: `47093`
  - `140`: `17896`
  - `332`: `18018`
  - `335`: `18491`
- Warm rereads with the hoist landed worse at `136040 us`, `137054 us`, and
  `140843 us` total:
  - `136040`: `27713 / 50699 / 18734 / 19345 / 19549`
  - `137054`: `25619 / 50318 / 22239 / 19997 / 18881`
  - `140843`: `28737 / 49206 / 20016 / 20591 / 22293`
  These are `24 / 74 / 140 / 332 / 335` in order.
- The dropped hoist therefore did not re-earn the checked-in `123544 us`
  total and did not beat the immediate pre-slice local reread honestly, so the
  benchmark artifact stayed unchanged and no new intended-profile rerun was
  launched.
- Two later local-only compact-summary bookkeeping slices then also stayed
  parity-clean in the same claim-focused tests and replay parity, but both
  landed worse and were dropped:
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
  These are `24 / 74 / 140 / 332 / 335` in order for each total above.
- Neither later slice re-earned the checked-in `123544 us` total, so both
  code paths were reverted, the benchmark artifact stayed unchanged, and no
  new intended-profile rerun was launched.
- A later narrow slice then cached terminal-clause bit cost inside
  `TerminalClauseNuFacts`, backfilled older replay fixtures on load, and
  reused that sidecar across the remaining-one plateau-kernel and compact
  admitted bit-`kappa` bookkeeping paths instead of recursively walking the
  same clause expr on every admitted candidate.
- Claim-focused validation stayed green:
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
  These are `24 / 74 / 140 / 332 / 335` in order.
- The refreshed stored benchmark artifact was then rewritten from a later
  under-gate reread at `123148 us` total:
  - `123148`: `25589 / 43365 / 18070 / 17325 / 18799`
- Additional spot-check rereads still bounced around the gate at `124012 us`
  and `126456 us`, so the win is narrow and noisy, but the slice re-earned the
  checked-in `123544 us` total more than once and also beat the immediate
  pre-slice local release reread honestly.
- The earned capped intended-profile rerun was then spent on
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
- That rerun did not beat the current `124`-prefix short-loop target honestly:
  it kept the same retained-prefix surface and lower RSS, but explored two
  fewer prefixes than `plateau-kernel-split-v1`, left one more frontier item,
  and spent slightly more time in summary build.
- The loop has therefore already spent the re-earned capped rerun and is back
  in code work.
- A fresh local-only slice now precomputes a prefix-local accept-rank context
  and reuses it in the compact remaining-one no-miss branches so those hot
  best-rank updates stop loading a scratch telescope and rescanning the full
  terminal telescope just to break primary-rank ties.
- Claim-focused validation stayed green for that follow-on slice:
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
- The loop has therefore already spent the earned capped rerun and is back in
  code work with `prefix-accept-rank-context-v1` as the short-loop checkpoint
  to beat.
- A fresh narrow slice now precomputes clause-side accept-rank facts inside
  `TerminalClauseNuFacts` and reuses them in the compact remaining-one no-miss
  branches, so hot best-rank tie updates stop rescanning the same terminal
  clause expr for structural signals and ref sets once the compact path has
  already decided the clause is worth a full accept-rank build.
- Claim-focused validation stayed green for that follow-on slice:
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
- The lane therefore now has repeated honest 20-minute wins on the current
  retained-prefix surface (`135` then `139`), so longer continuation reads are
  back on the table.
- A fresh longer intended-profile rerun was then launched as
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`
  because neither `clause-accept-rank-facts-v1` nor the stored inspect surface
  exposed a resumable step-`4` frontier generation, so `pen-cli resume` would
  only have replayed step `4` from the step-`3` checkpoint instead of
  continuing the in-flight frontier honestly.
- That rerun again reused the same release binary hash
  (`278c311ddf5e416b09d24923dc392388aaf5817c65f0c60f856ebde7466140a5`)
  and the same stored config as the earlier short-loop winner.
- It then beat `prefix-local-score-v1` honestly at every matched later
  checkpoint through `576` while keeping the same no-miss shape and the same
  frontier queue lengths at the matched prefix counts, but on a much smaller
  retained surface:
  - `140`: `1188340 / 1181188` with `41 groups / 29249 candidates`
  - `163`: `1381425 / 1373168` with `41 groups / 29249 candidates`
  - `332`: `2853118 / 2836752` with `42 groups / 29529 candidates`
  - `335`: `2879368 / 2862850` with `43 groups / 29809 candidates`
  - `408`: `3511930 / 3491831` with `43 groups / 29809 candidates`
  - `437`: `3770964 / 3749419` with `43 groups / 29809 candidates`
  - `454`: `3922561 / 3900177` with `43 groups / 29809 candidates`
  - `484`: `4183978 / 4160100` with `43 groups / 29809 candidates`
  - `576`: `4997082 / 4968579` with `43 groups / 29809 candidates`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The rerun was then manually stopped at `578` explored prefixes with
  `elapsed_millis = 5015917`, `terminal_summary_build_millis = 4987295`,
  `frontier_queue_len = 2197`, RSS `= 1744551936`, and still
  `0` fallback/admissibility checks.
- No step-`4` frontier artifacts were persisted on that rerun either, so the
  next longer continuation still needs another fresh intended rerun rather
  than `pen-cli resume`.
- A second fresh intended-profile rerun was then launched as
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v2`
  on the same release binary and config because neither the short-loop winner
  nor the first longer rerun stored a resumable step-`4` frontier generation.
- That rerun re-earned the matched later walls through `335` while keeping the
  same no-miss shape and retained surfaces:
  - `140`: `1195999 / 1188793` with `41 groups / 29249 candidates`
  - `163`: `1390249 / 1381921` with `41 groups / 29249 candidates`
  - `332`: `2874771 / 2858216` with `42 groups / 29529 candidates`
  - `335`: `2901198 / 2884494` with `43 groups / 29809 candidates`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Each matched wall landed slightly slower than
  `clause-accept-rank-facts-long-rerun-v1`, so the rerun did not replace the
  current later-wall reference even though it corroborated the same compact
  retained surface through `335`.
- The rerun was then manually stopped at `345` explored prefixes with
  `elapsed_millis = 2990691`, `terminal_summary_build_millis = 2973493`,
  `frontier_queue_len = 2430`, RSS `= 1056485376`, and still
  `0` fallback/admissibility checks.
- No step-`4` frontier artifacts were persisted on that rerun either, so the
  next longer continuation still needs another fresh intended rerun rather
  than `pen-cli resume`.

## Do This Next

### 1. Spend The Next Slice On Yet Another Longer Continuation, Not A Code Change

Keep the clause-side accept-rank-facts slice landed.
It already re-earned replay gate comfortably, spent one capped rerun, then won
the short-loop checkpoint again at `139` explored prefixes, then owned one
honest longer rerun through `576`, and now also has a second fresh rerun that
re-earned the same `140/163/332/335` middle walls without replacing the
current `576` winner.
The next move is still not another fresh code slice first.
Reopen one more longer intended-profile continuation on the current binary and
see whether the lane can now carry that corroborated `335` surface back
through `576` and then toward `1038/1095` or step `5`.

### 2. Run The Next Validation Slice In This Order

1. Keep the current landed code and the refreshed replay benchmark artifact
   unchanged.
2. Do not prefer `pen-cli resume` first on either
   `clause-accept-rank-facts-v1` or
   `clause-accept-rank-facts-long-rerun-v1` or
   `clause-accept-rank-facts-long-rerun-v2`.
   None stored a resumable step-`4` frontier generation, so launch one fresh
   intended `desktop_claim_shadow` rerun on the same release binary and config
   instead.
3. Let that continuation run materially past the `20`-minute window and past
   the corroborated `335` surface.
4. Re-earn the later stored step-`4` checkpoints in this order:
   `408`, `437`, `454`, `484`, `576`, then `1038`, then `1095`, then step `5`.
5. Keep comparing the `20`-minute checkpoint first against the current
   short-loop target from `clause-accept-rank-facts-v1`.
   Then compare later checkpoints first against
   `clause-accept-rank-facts-long-rerun-v2` through `335`, then against
   `clause-accept-rank-facts-long-rerun-v1` through `576`, and keep
   `prefix-local-score-v1` as the farther continuation reference behind them
   at `1038/1095`.
6. If the next fresh rerun cannot keep the corroborated `335` surface
   honestly, cannot keep the new `576` continuation surface honestly, or if
   later stored reads lose the retained-prefix story before `1038`, return to
   code work with another narrow per-admitted compact-summary cost slice inside
   `compute_terminal_prefix_completion_summary_from_candidates(...)`, but do
   not retry the dropped focus-aligned competition-gate/payload-mode hoist,
   the dropped shared compact-bookkeeping fold, or the dropped
   claim-open-band compact local-state hoist first.
7. Do not wake the dormant general cached-summary reopen machinery first.

### 3. Keep Rule For The Next Longer Read

Treat the reopened continuation as real progress only if the stored later
step-`4` checkpoints keep the same no-miss shape, re-earn
`408/437/454/484/576` after the corroborated `335` surface, and then move
materially beyond the new `576` wall toward `1038/1095` or step `5`.

### 4. Short-Loop Beat Rule

Treat a later slice as a real short-loop win only if its `20`-minute stored
read beats the current target honestly.

Minimum comparison fields:

- `prefix_states_explored`
- `prefix_cache_groups`
- `prefix_cache_candidates`
- `frontier_queue_len`
- `observed_process_rss_bytes`
- `terminal_summary_build_millis`
- `terminal_summary_admissibility_checks`
- `terminal_summary_fallback_connectivity_checks`

Interpretation rule:

- More explored prefixes by `20` minutes is the main signal.
- Keep the retained-prefix story honest.
- Do not accept a "win" that only comes from waking fallback connectivity or
  admissibility work on what should still be no-miss surfaces.
- Do not expect the next few slices to beat the full `1095`-prefix stop yet.
- The immediate goal is to turn the repeated `20`-minute wins plus the first
  honest `576` continuation win into `1038/1095` progress or a step-`5`
  arrival.

### 5. When To Allow Another Long Read

Repeated `20`-minute wins now exist, and one longer continuation has already
cleared `576` honestly, so another longer continuation is allowed on the
current landed binary.

After that continuation:

- keep `clause-accept-rank-facts-long-rerun-v1` as the current later-wall
  reference through `576` unless the next rerun replaces it honestly
- keep `clause-accept-rank-facts-long-rerun-v2` as the current corroborating
  middle-wall read through `335` unless the next rerun replaces it honestly
- keep `prefix-local-score-v1` as the farther stored continuation reference to
  beat at `1038/1095`
- keep `clause-accept-rank-facts-v1` as the current short-loop gate until a
  later contender replaces it honestly
- return to code work first if the longer continuation fails to move the later
  stored walls honestly beyond `576`

## Do Not Reopen First

- cached-summary reopen wake-up work
- the dropped focus-aligned competition-gate/payload-mode hoist
- the dropped shared compact-bookkeeping fold
- the dropped claim-open-band compact local-state hoist
- contender-rank helper rewrites
- connectivity-first or cache-first rewrites
- deterministic batched parallel reduction
- broad metadata-only or bookkeeping-only cleanup
- another timing-only slice with no new runtime question

## Keep Or Branch Decision

- Stay on runtime work while the intended profile still stalls in step `4`.
- Keep `clause-accept-rank-facts-long-rerun-v1` as the nearer continuation
  reference through `576` until a later slice replaces it honestly.
- Keep `clause-accept-rank-facts-long-rerun-v2` as a corroborating middle-wall
  read through `335`, but do not treat it as the later-wall winner.
- Keep the stopped `1095`-prefix read from `prefix-local-score-v1` as the
  farther continuation reference until a later rerun reaches `1038/1095`
  honestly or reaches step `5`.
- Treat the `clause-accept-rank-facts-v1` `20`-minute checkpoint as the next
  short-loop validation gate until a later capped rerun replaces it honestly.
- Branch to parity, breadth, compare, benchmark, and certification work only
  after the reopened longer continuation proves that the newer short-loop wins
  also move the later intended-profile walls beyond `576`.
