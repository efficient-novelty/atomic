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
- The current full-profile runtime reference is
  `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`.
- The best current full-profile candidate to beat is
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`.
- The previous full-profile runtime reference is
  `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`.
- The earlier full-profile runtime reference is
  `runs/codex-claim-release-full-aggregation-open-band-compact-v1`.
- The earlier full-profile runtime reference is
  `runs/codex-claim-release-full-connectivity-facts-v1`.
- The latest intended-profile rerun is
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`.
  It carried the already-landed prefix-side single-clause structural `nu`
  context slice past the prior runtime reference, then was manually stopped
  during step `4` after the stored `1038` checkpoint so it can serve as the
  next speed target to beat.
- That stopped rerun re-earned the honest retained-prefix story through the
  farthest stored `43` surface:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `1038` read
  It re-earned and beat the prior runtime reference's stored
  `437/454/484` wall, but it still did not reach step `5`.
- It remains materially ahead of the prior runtime reference on every
  decisive matched later checkpoint it re-earned:
  - `140`: `1849510 / 1839797` instead of `2014043 / 2005230`
  - `163`: `2147103 / 2135876` instead of `2334208 / 2324213`
  - `332`: `4465654 / 4444493` instead of `4885155 / 4866258`
  - `335`: `4506796 / 4485426` instead of `4931017 / 4911930`
  - `408`: `5479859 / 5454313` instead of `6031554 / 6007913`
  - `437`: `5888660 / 5861474` instead of `6494645 / 6469171`
  - `454`: `6125662 / 6097497` instead of `6770742 / 6744132`
  - `484`: `6536061 / 6505941` instead of `7235355 / 7206681`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The same exact-`nu` win is still the source of the improvement, even though
  this stopped rerun is slightly softer than the earlier `v1` proof-of-win rerun
  at matched `140/163`, and the later `42/43` reopens still show the same
  aggregation-first, exact-`nu`-cut shape all the way through the prior
  stored `484` wall:
  - `140`: aggregation `= 787853 ms` instead of `763398 ms`, connectivity
    `= 578640 ms` instead of `557557 ms`, exact `nu` `= 295922 ms` instead of
    `511854 ms`, terminal clause-filter handoff `= 15506 ms` instead of
    `13877 ms`
  - `163`: aggregation `= 906302 ms` instead of `877573 ms`, connectivity
    `= 678215 ms` instead of `651431 ms`, exact `nu` `= 343912 ms` instead of
    `594142 ms`, terminal clause-filter handoff `= 18516 ms` instead of
    `16351 ms`
  - `408`: aggregation `= 2290021 ms` instead of `2218628 ms`,
    connectivity `= 1729598 ms` instead of `1675112 ms`, exact
    `nu` `= 903168 ms` instead of `1596901 ms`, terminal clause-filter
    handoff `= 55743 ms` instead of `54702 ms`
  - `484`: aggregation `= 2722926 ms` instead of `2641777 ms`,
    connectivity `= 2063163 ms` instead of `2010145 ms`, exact
    `nu` `= 1084329 ms` instead of `1902551 ms`, terminal clause-filter
    handoff `= 70144 ms` instead of `65174 ms`
- Cached-summary reopen remains dormant on stored evidence through the stored
  `1038` read:
  - `remaining_one_materialized_from_cached_summary = 0`
  - `remaining_one_prefixes_seen = 0`
  - `remaining_one_materialized_compact_direct = 43`
- `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0` through the stored
  `1038` read.
- The first stored plateau activation is still `24`, and
  `terminal_summary_plateau_activations = 231996` has stayed flat through the
  stored `1038` read.
- The deterministic plateau replay harness is now landed in code:
  `pen_search::engine::claim_replay` can capture compact remaining-one summary
  inputs at targeted retained surfaces, replay only
  `compute_terminal_prefix_completion_summary_from_candidates(...)`, and check
  compact summary parity before reporting local benchmark timings.
- A repo-facing entry point is now landed too:
  `cargo xtask claim-replay-harness <capture|benchmark> ...`.
  The capture path now also reuses an existing fixture file and persists
  newly captured surfaces back to disk during long runs so early plateau
  fixtures are no longer lost when later targets outlast one shell window.
- The harness has honest local validation now:
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`
  both pass with the new replay module and its compact-summary parity test.
- The stored plateau fixture corpus is now complete.
  `tests/fixtures/claim_runtime/remaining_one_plateau_fixtures.json` now
  stores all five stable retained surfaces from the release harness:
  - `39 / 144845` at `24`
  - `40 / 147639` at `74`
  - `41 / 154842` at `140`
  - `42 / 157636` at `332`
  - `43 / 160430` at `335`
- The local replay benchmark read is now refreshed against that full stored
  corpus too:
  `tests/fixtures/claim_runtime/remaining_one_plateau_benchmark.json`.
  On `10` release iterations with compact-summary parity enforced, the current
  timings read:
  - `39 / 144845 @ 24`: avg `3431 us`, best `2742 us`, worst `4103 us`
  - `40 / 147639 @ 74`: avg `5157 us`, best `4420 us`, worst `7276 us`
  - `41 / 154842 @ 140`: avg `2713 us`, best `2611 us`, worst `2959 us`
  - `42 / 157636 @ 332`: avg `2856 us`, best `2463 us`, worst `3594 us`
  - `43 / 160430 @ 335`: avg `2235 us`, best `2024 us`, worst `2934 us`
- The first harness-backed facts-only hot-loop slice is now landed in code:
  remaining-one bound checks, compact/full summary build, compact
  materialization, clause-catalog reuse, filtered active-window clones, and
  replay fixtures can now all stay on clause refs plus predecoded
  connectivity facts plus predecoded structural-`nu` facts instead of
  rebuilding terminal-clause `nu` facts inside the hot loop itself.
- That refreshed local replay read is softer overall than the prior stored
  `2026-03-31` read:
  only the stored `41 / 154842` surface improved, while `39/40/42/43`
  regressed enough to raise the five-surface total from `153124 us` to
  `163951 us` (about `7.07%` slower, and about `5.69%` above the earlier
  pre-facts `155131 us` total) while still keeping compact-summary parity.
- The facts-only slice has honest local validation too:
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`
  - `cargo test -p pen-eval single_clause_context_matches_full_structural_nu`
  all pass after threading the predecoded structural-`nu` facts through the
  hot path and replay harness.
- A fresh intended-profile rerun on that landed slice now exists:
  `runs/codex-claim-release-full-aggregation-open-band-structural-nu-facts-v1`.
  It was manually stopped during step `4` after the stored `229` checkpoint,
  and it preserved the honest retained-prefix story through that read:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` through the stored `229` read
- That stopped rerun materially improved every matched stored checkpoint it
  re-earned versus the current speed target
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`:
  - `24`: `270393 / 267623` instead of `297716 / 295006`
  - `43`: `489831 / 485917` instead of `535068 / 531318`
  - `44`: `500425 / 496452` instead of `546538 / 542733`
  - `54`: `612851 / 608287` instead of `674040 / 669640`
  - `74`: `843984 / 838184` instead of `929910 / 924314`
  - `76`: `873422 / 867487` instead of `961986 / 956271`
  - `140`: `1664957 / 1654897` instead of `1849510 / 1839797`
  - `163`: `1927201 / 1915799` instead of `2147103 / 2135876`
  - `228`: `2687659 / 2672592` instead of `3018308 / 3003118`
  - `229`: `2703503 / 2688385` instead of `3035483 / 3020234`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The same newer rerun kept the visible later wall category unchanged through
  the stored `229` read while cutting active exact `nu` sharply again:
  - `140`: aggregation `= 787553 ms`, connectivity `= 584152 ms`, exact
    `nu` `= 102187 ms`, terminal clause-filter handoff `= 18308 ms`,
    RSS `= 577945600` bytes
  - `163`: aggregation `= 905537 ms`, connectivity `= 681165 ms`, exact
    `nu` `= 118301 ms`, terminal clause-filter handoff `= 21401 ms`,
    RSS `= 645066752` bytes
  - `228`: aggregation `= 1258056 ms`, connectivity `= 953059 ms`, exact
    `nu` `= 166719 ms`, terminal clause-filter handoff `= 30296 ms`,
    RSS `= 832106496` bytes
  - `229`: aggregation `= 1267657 ms`, connectivity `= 957351 ms`, exact
    `nu` `= 167317 ms`, terminal clause-filter handoff `= 30442 ms`,
    RSS `= 834568192` bytes
- Cached-summary reopen still stayed dormant on stored evidence through the
  stored `229` read:
  - `remaining_one_materialized_from_cached_summary = 0`
  - `remaining_one_prefixes_seen = 0`
  - `remaining_one_materialized_compact_direct = 41`
- `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0` through the stored
  `229` read on that newer rerun.
- The later retained surfaces now capture honestly from the same seeded path,
  but the operational command needs to stay in release mode on this repo:
  use `cargo run --release -p xtask -- claim-replay-harness ...` when
  refreshing the stored corpus or benchmark so the late plateau pass does not
  fall back to the much slower default debug build.
- Observed RSS stayed well below the old allocator-failure band, even though
  late drift rose materially after the prior `484` wall:
  - `140`: `572006400` bytes
  - `163`: `638435328` bytes
  - `332`: `938635264` bytes
  - `408`: `1064996864` bytes
  - `484`: `1233252352` bytes
  - `1038`: `2303102976` bytes
- The previous manually stopped proof-of-win rerun remains
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v1`.
  It still provides the deepest stored completed read of this slice through
  `239`.
- Because the latest rerun was manually stopped externally during step `4`
  after the stored `229` checkpoint,
  `reports/latest.txt` still reflects completed step `3`, `run.json` still
  says `status = "running"`, and `reports/steps/step-05-live.ndjson` is
  absent; the authoritative evidence for that rerun lives in
  `reports/steps/step-04-live.ndjson`.

## What Stays Landed
- delayed materialization
- the incumbent-primary remaining-one fast path
- the one-pass `structural_nu` summary-build fast path
- the prefix-side single-clause structural `nu` context reused across
  remaining-one exact scoring, compact-summary build, and compact
  materialization on the claim open-band path
- the algebraic `nu` ceiling patch
- the family-agnostic claim terminal-admissibility shortcut
- the exact non-allocating connectivity summary scan
- the terminal-only cached parent connectivity decision
- the aggregation-side accept-rank short-circuit that skips full
  `AcceptRank` construction for primary-dominated bar-clearers
- the claim open-band terminal-clause handoff fast path that keeps
  exact-admitted open-band surfaces on clause refs instead of per-clause
  admissibility payloads
- the compact claim open-band aggregation fast path that bypasses generic
  admitted-evaluation bookkeeping on the no-evaluations summary kernel
- the higher-fidelity late-surface timing accumulation used by the current
  short diagnostic surface
- the shared terminal-clause connectivity-facts sidecar on the shared clause
  catalog used by the claim remaining-one summary/materialization path
- the shared terminal-clause structural-`nu` facts sidecar threaded through
  the clause catalog, filtered active-window clones, remaining-one
  bound/summary/materialization, and replay fixtures so the hit path can stay
  on predecoded facts end-to-end
- the steady-state scratch-slot `clone_from` reuse on terminal-clause loads
  inside remaining-one summary/materialization
- the boundary-timestamp timing pass on the compact claim open-band
  no-evaluations summary kernel

## Baselines That Matter

### 1. Current Short Baseline
- Run: `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Honest retained shape:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
- The rerun was manually stopped after the stored `76` checkpoint because the
  decisive short and reopened surfaces had already been captured; one extra
  stored `77` checkpoint flushed while stopping and kept the same honest
  `40 groups / 147639 candidates` reopened surface.
- Because the stop was external during step `4`, the authoritative evidence
  for this short winner is `reports/steps/step-04-live.ndjson`, while
  `reports/latest.txt` still reflects completed step `3` and `run.json` still
  says `status = "running"`.

### 2. Current Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`
- It preserved the honest `39/40/41/42/43` retained-prefix story from the
  previous runtime reference, materially improved every decisive matched
  checkpoint through the prior stored `454` wall, and then pushed the stored
  step-`4` wall farther to `484`.
- Stored decisive checkpoints:
  - `24`: `327917 / 325321`
  - `43`: `591083 / 587479`
  - `44`: `603570 / 599913`
  - `54`: `743997 / 739797`
  - `74`: `1023572 / 1018283`
  - `76`: `1057864 / 1052464`
  - `140`: `2014043 / 2005230`
  - `163`: `2334208 / 2324213`
  - `228`: `3287422 / 3274060`
  - `229`: `3304737 / 3291324`
  - `332`: `4885155 / 4866258`
  - `335`: `4931017 / 4911930`
  - `408`: `6031554 / 6007913`
  - `437`: `6494645 / 6469171`
  - `454`: `6770742 / 6744132`
  - `484`: `7235355 / 7206681`
- The later retained-prefix surface now reads:
  - `41 groups / 154842 candidates` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `484` read
- Aggregation remains the lead later bucket on the stored `41/42/43`
  surfaces, with connectivity second, exact `nu` third, and terminal
  clause-filter handoff still tiny.
- Inference from matched checkpoints:
  this new winner mainly collapsed the previously unattributed summary-build
  remainder rather than reducing measured aggregation itself.
- Observed RSS reached `1581830144` bytes at `484`.
- The run never reached step `5` on stored evidence;
  `reports/steps/step-05-live.ndjson` is absent.

### 3. Best Current Candidate
- Run: `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`
- Hypothesis:
  keep the already-landed prefix-`nu` context slice intact and carry the same
  intended-profile rerun deeper before deciding whether it can replace the
  current runtime reference.
- Outcome:
  - it preserved the honest `39/40/41/42/43` retained-prefix story through
    the stored `1038` read before being manually stopped
  - it re-earned the decisive `140/163/332/335/408/437/454/484` region and
    stayed materially ahead of the current runtime reference at every matched
    checkpoint
  - it moved the stored step-`4` wall materially past the previous
    `484` stopper to `1038`
  - it still did not reach step `5`
  - it is slightly slower than the earlier `v1` proof-of-win rerun at matched
    `140/163`, but not enough to erase the win versus the current runtime
    reference, and the later `42/43/484` region is now back with a clear
    wall-clock lead
  - it is still winning by cutting exact `nu`; aggregation remains first,
    connectivity remains second, exact `nu` remains third, and the terminal
    clause-filter handoff remains tiny
  - cached-summary reopen stayed dormant on stored evidence
  - it remained memory-safe, though late RSS drift rose materially by the
    stopped `1038` read
- Comparison versus the current runtime reference:
  - `140`: `1849510 / 1839797` instead of `2014043 / 2005230`
  - `163`: `2147103 / 2135876` instead of `2334208 / 2324213`
  - `332`: `4465654 / 4444493` instead of `4885155 / 4866258`
  - `335`: `4506796 / 4485426` instead of `4931017 / 4911930`
  - `408`: `5479859 / 5454313` instead of `6031554 / 6007913`
  - `437`: `5888660 / 5861474` instead of `6494645 / 6469171`
  - `454`: `6125662 / 6097497` instead of `6770742 / 6744132`
  - `484`: `6536061 / 6505941` instead of `7235355 / 7206681`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Comparison versus the earlier `v1` proof-of-win rerun:
  - `140`: `1849510 / 1839797` instead of `1836533 / 1826733`
  - `163`: `2147103 / 2135876` instead of `2135897 / 2124610`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Stored later-surface read before stopping:
  - `140`: `41 groups / 154842 candidates`, exact `nu` `= 295922 ms`
    instead of `511854 ms`, RSS `= 572006400` bytes
  - `163`: `41 groups / 154842 candidates`, exact `nu` `= 343912 ms`
    instead of `594142 ms`, RSS `= 638435328` bytes
  - `332`: `42 groups / 157636 candidates`, exact `nu` `= 732960 ms`,
    RSS `= 938635264` bytes
  - `408`: `43 groups / 160430 candidates`, exact `nu` `= 903168 ms`,
    RSS `= 1064996864` bytes
  - `484`: `43 groups / 160430 candidates`, exact `nu` `= 1084329 ms`,
    RSS `= 1233252352` bytes
  - `1038`: `43 groups / 160430 candidates`, exact `nu` `= 2406190 ms`,
    RSS `= 2303102976` bytes
- Honest read:
  the deeper rerun has now shown that the same exact-`nu` win survives not
  just the minimum `140/163` re-earn, but also the later `42/43` reopens and
  the full stored `437/454/484` wall, all without waking the dormant
  cached-summary reopen path. This stopped rerun is now the best current
  candidate to beat on speed, even though it still ended inside step `4` and
  late RSS drift stayed visible.
- Because the rerun was manually stopped during step `4`, `reports/latest.txt`
  still reflects completed step `3`, `run.json` still says `status = "running"`,
  and `reports/steps/step-05-live.ndjson` is absent; the authoritative
  evidence lives in `reports/steps/step-04-live.ndjson`.

### 4. Earlier Prefix-`nu` Measured Later-Surface Follow-Up
- Run: `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v1`
- Hypothesis:
  keep the current stage-timing winner intact, but stop rebuilding a full
  structural-`nu` prefix profile for every remaining-one exact candidate by
  precomputing one prefix-side context and folding in only the final clause
  during bound checks, compact-summary construction, and compact
  materialization.
- Outcome:
  - it preserved the honest `39/40/41` retained-prefix story through the
    stored `239` read
  - it has not yet re-earned the current runtime reference's `42 / 157636`
    or `43 / 160430` surfaces
  - it materially improved every decisive matched checkpoint it re-earned
    versus the current runtime reference from `140` through `239`
  - it cut exact `nu` sharply on that retained `41 / 154842` plateau, even
    while aggregation, connectivity, and clause-filter handoff all rose
    slightly
  - it kept aggregation first, connectivity second, exact `nu` third, and the
    clause-filter handoff tiny
  - it stayed memory-safe and never hinted at the old allocator-failure story
    returning
  - the rerun was manually stopped after the stored `239` checkpoint because
    the first later-surface win was already clear but the later `42/43/484`
    region had not yet been re-earned
- Comparison versus the current runtime reference:
  - `140`: `1836533 / 1826733` instead of `2014043 / 2005230`
  - `163`: `2135897 / 2124610` instead of `2334208 / 2324213`
  - `231`: `3069187 / 3053464` instead of `3338028 / 3324509`
  - `239`: `3199123 / 3182864` instead of `3475147 / 3461204`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Stored later-surface read before stopping:
  - `140`: `41 groups / 154842 candidates`, exact `nu` `= 293473 ms`
    instead of `511854 ms`, RSS `= 577634304` bytes
  - `163`: `41 groups / 154842 candidates`, exact `nu` `= 343044 ms`
    instead of `594142 ms`, RSS `= 646213632` bytes
  - `239`: `41 groups / 154842 candidates`, exact `nu` `= 522245 ms`
    instead of `898852 ms`, RSS `= 867381248` bytes
- Honest read:
  this is the first follow-up after the stage-timing winner that wins the
  decisive early later surface honestly on stored evidence. It does not yet
  dislodge the current runtime reference because it has only re-earned the
  `41 / 154842` plateau so far, but the exact-`nu` cut is real and durable
  through the stored `239` read. The next honest move is to carry this same
  slice farther into `332/335/408/437/454/484` or step `5` before inventing
  another code path.

### 5. Latest Failed Later-Surface Follow-Up
- Run: `runs/codex-claim-release-full-aggregation-open-band-compact-summary-reuse-v1`
- Hypothesis:
  keep the current stage-timing winner intact, but let compact claim
  remaining-one summaries retain just enough exact candidate data to reopen
  surviving prefixes directly from cached summaries instead of rescanning
  connectivity and exact `nu` during compact materialization.
- Outcome:
  - it preserved the honest `39/40/41` retained-prefix story through the
    stored `163` read
  - it never re-earned the current runtime reference's `42 / 157636` or
    `43 / 160430` surfaces
  - it regressed every decisive matched checkpoint versus the current runtime
    reference from `24` through `163`
  - its intended cached-summary reopen path never engaged on stored evidence
    through the max stored `165` read
  - it did reduce measured aggregation first on the stored `41 / 154842`
    surface, but connectivity, exact `nu`, and terminal clause-filter handoff
    all rose more than the aggregation win saved
  - it stayed memory-safe and never hinted at the old allocator-failure story
    returning
  - the rerun was manually stopped after the later-surface non-engagement and
    cost-shift regression were already clear on stored evidence
- Comparison versus the current runtime reference:
  - `24`: `333574 / 330822` instead of `327917 / 325321`
  - `43`: `602370 / 598494` instead of `591083 / 587479`
  - `44`: `614946 / 611012` instead of `603570 / 599913`
  - `54`: `758510 / 753973` instead of `743997 / 739797`
  - `74`: `1043186 / 1037421` instead of `1023572 / 1018283`
  - `76`: `1076868 / 1070982` instead of `1057864 / 1052464`
  - `140`: `2054569 / 2044701` instead of `2014043 / 2005230`
  - `163`: `2388800 / 2377464` instead of `2334208 / 2324213`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Stored later-surface read before stopping:
  - `140`: `41 groups / 154842 candidates`, `2054569 / 2044701`,
    RSS `= 577400832` bytes
  - `163`: `41 groups / 154842 candidates`, `2388800 / 2377464`,
    RSS `= 644460544` bytes
- Honest read:
  the cached-summary reopen path stayed dormant on the decisive `39/40/41`
  surfaces, so this slice paid overhead without delivering its intended reuse.
  The small aggregation dip on the first later `41` surface was erased by
  higher connectivity, exact `nu`, and clause-filter time. This compact-
  summary reuse rewrite is now dropped as a standalone next move.

### 6. Previous Failed Later-Surface Follow-Up
- Run: `runs/codex-claim-release-full-aggregation-open-band-rank-scan-v1`
- Hypothesis:
  keep the current stage-timing winner intact, but cut one more
  per-contender measured aggregation cost inside full `AcceptRank`
  construction by fusing the structural signal and max-var-ref scans so
  contender updates stop rebuilding variable-reference sets twice per
  telescope.
- Outcome:
  - it preserved the honest `39/40/41` retained-prefix story through the
    stored `163` read
  - it never re-earned the current runtime reference's `42 / 157636` or
    `43 / 160430` surfaces
  - it regressed every decisive matched checkpoint versus the current runtime
    reference from `24` through `163`
  - it did reduce measured aggregation first on the stored `41 / 154842`
    surface, but connectivity and exact `nu` both rose more than the
    aggregation win saved
  - it stayed memory-safe and never hinted at the old allocator-failure story
    returning
  - the rerun was manually stopped after the later-surface cost shift was
    already clear on stored evidence
- Comparison versus the current runtime reference:
  - `24`: `328941 / 326192` instead of `327917 / 325321`
  - `43`: `595120 / 591267` instead of `591083 / 587479`
  - `44`: `607902 / 603989` instead of `603570 / 599913`
  - `54`: `750755 / 746245` instead of `743997 / 739797`
  - `74`: `1031304 / 1025636` instead of `1023572 / 1018283`
  - `76`: `1064603 / 1058818` instead of `1057864 / 1052464`
  - `140`: `2034303 / 2024681` instead of `2014043 / 2005230`
  - `163`: `2376395 / 2365242` instead of `2334208 / 2324213`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Stored later-surface read before stopping:
  - `140`: `41 groups / 154842 candidates`, `2034303 / 2024681`,
    RSS `= 572178432` bytes
  - `163`: `41 groups / 154842 candidates`, `2376395 / 2365242`,
    RSS `= 639975424` bytes
- Honest read:
  the fused contender-rank scan engaged and honestly lowered measured
  aggregation on the first later `41` surface, but the total wall still
  regressed because connectivity and exact `nu` rose more than the aggregation
  win saved and the unattributed remainder did not shrink enough. This
  contender-rank helper rewrite is now dropped as a standalone next move.

### 7. Previous Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`
- It preserved the honest `39/40/41/42/43` retained-prefix story from the
  previous runtime reference, paid only a tiny early retained-surface tax,
  materially improved the later stored checkpoints from `76` onward, and then
  pushed the stored step-`4` wall farther from `437` to `454`.
- The current stage-timing follow-up now keeps the same honest retained-prefix
  story, wins materially at every decisive matched checkpoint, and moves the
  stored wall again from `454` to `484`.

### 8. Earlier Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
- It preserved the honest `39/40/41` retained-prefix story from the previous
  runtime reference, re-earned the `42/157636` and `43/160430` reopens,
  materially improved every decisive stored matched checkpoint through the
  previous stored `408` wall, and then pushed the stored step-`4` wall farther
  to `437`.

### 9. Earlier Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-connectivity-facts-v1`
- It preserved the honest `39/40/41` retained-prefix story from the old
  open-band-handoff follow-up, reopened to `42/157636` and `43/160430`, and
  pushed the prior stored wall to `408`, but the newer compact, scratch-slot,
  and stage-timing references all now stay ahead of it.

### 10. Comparison Full-Profile Baseline
- Run: `runs/codex-claim-release-full-kernel-aggregation-v1`
- It remains the broader comparison baseline only. The current runtime
  reference stays materially ahead of it.

### 11. Current Late-Surface Diagnostic
- Run: `runs/codex-claim-release-step4-kernel-late-profile-v1`
- It remains the earlier late diagnostic, but the newer full-profile
  stage-timing follow-up is now the authoritative later-surface read.

## What Stays Dropped
- ordering and reuse variants: `context-equivalence-v1`,
  `incumbent-ordering-v1`, `local-two-step-order-v2`,
  `proof-close-handoff-v1`, `post-plateau-v1`,
  `post-plateau-materialize-v1`, `post-plateau-summary-cache-v3`
- connectivity-side cache and precompute variants:
  `kernel-connectivity-v3`, `kernel-connectivity-v4`
- candidate-prep / remap variants: `terminal-candidate-prep-v1`
- telemetry-only slice: `kernel-filter-profile-v1`
- exact primary-rank bookkeeping rewrite: `kernel-rank-bookkeeping-v1`
- bound-merge bookkeeping rewrite: `kernel-bound-merge-v1`
- lazy incumbent-tie `AcceptRank` deferral rewrite:
  `kernel-lazy-acceptrank-v1`
- summary-side batching rewrite: `kernel-summary-batching-v1`
- summary-side bookkeeping rewrite: `kernel-summary-bookkeeping-v1`
- prefix-wide competition-gate hoist: `kernel-competition-hoist-v1`
- claim open-band admitted-kernel fusion: `kernel-admitted-kernel-v1`
- direct bound/bookkeeping absorb cleanup: `kernel-bound-bookkeeping-v1`
- open-band compact-summary accumulator follow-up:
  `full-aggregation-open-band-accumulator-v1`
- open-band compact-summary cached-reuse follow-up:
  `full-aggregation-open-band-compact-summary-reuse-v1`
- open-band contender-rank fused scan follow-up:
  `full-aggregation-open-band-rank-scan-v1`
- scratch-slot clause-load reuse first pass: `kernel-clause-load-v1`
- exact-`nu` high-water gate: `kernel-nu-highwater-v1`
- summary-invariants accept-rank prefix-context rewrite:
  `kernel-summary-invariants-v1`
- summary-rank-context exact tie-break rewrite: `kernel-rank-context-v1`
- terminal-rank sidecar exact contender-rank rewrite:
  `kernel-terminal-rank-sidecar-v1`
- primary-rank context exact-threshold rewrite:
  `kernel-primary-context-v1`
- summary-constant bit-cost hoist: `kernel-summary-constant-v1`
- catalog-backed clause bit-cost sidecar: `kernel-catalog-constant-v1`
- bar-clear threshold bookkeeping rewrite: `kernel-summary-threshold-v1`
- exact rank-metadata pack with last-tie canonical-key finalization:
  `kernel-rank-metadata-v1`
- compact-summary strict-better-incumbent exact-rank deferral:
  `kernel-aggregation-tiecut-v1`
- eager terminal-clause metadata pack: `kernel-clause-metadata-v1`
- lazy admitted-only metadata retry: `kernel-admitted-metadata-v1`
- parent-summary connectivity lookup reuse: `kernel-reopened-connectivity-v1`

## Revised Working Diagnosis
- The old early RSS cliff remains broken; this is still a step-`4`
  throughput problem, not a return of the allocator-failure story.
- The stage-timing slice engaged honestly on the real intended profile: it
  preserved the kept `39/40/41/42/43` surface story, materially improved every
  decisive matched checkpoint through the prior stored `454` wall, and moved
  the later wall again to `484`.
- The newer prefix-`nu`-context follow-up answered the next active exact-`nu`
  question on stored evidence, and the deeper `v2` rerun has now confirmed
  that the same slice still wins honestly well past the re-earned `140/163`
  surface:
  `v1` preserved the honest `39/40/41` surfaces through the stored `239` read
  and materially improved the matched `140/163/231/239` checkpoints, while
  the stopped `v2` rerun re-earned `140/163/332/335/408/437/454/484`,
  reopened `42/43`, and then moved the stored wall to `1038` while staying
  ahead of the current runtime reference at every matched checkpoint.
- That newer follow-up also sharpens the blocker: the first honest win after
  the stage-timing slice comes from collapsing active exact `nu` work on the
  retained `41 / 154842` plateau, not from lowering aggregation or making
  cached-summary reopen engage there.
- The older compact-summary reuse follow-up still exposed the dormant cached
  reuse story clearly enough to keep that replay dropped:
  its intended cached-summary reopen path never engaged on the decisive stored
  `39/40/41` surfaces. Through the max stored `165` read,
  `remaining_one_materialized_from_cached_summary = 0` and
  `remaining_one_prefixes_seen = 0`.
- The later wall is still aggregation first on stored evidence, with
  connectivity second, exact `nu` third, and terminal clause-filter handoff
  still tiny.
- The new nuance is that the retained-prefix plateau after state `24` is real,
  and the first post-stage-timing gain that actually holds on stored evidence
  is an exact-`nu` cut on that plateau. The newer facts-only follow-up now
  pushes that same idea one step farther by keeping the hit path on
  predecoded clause facts; its refreshed local replay read turned softer
  overall on this machine snapshot, but its fresh full-profile rerun still
  materially beat the stopped `v2` speed target through the stored `229`
  checkpoint while keeping the same honest `39/40/41` retained surfaces.
- Observed RSS on the stopped rerun stayed slightly lower than the current
  runtime reference at `140/163`, then rose well above it by the stored
  `1038` read. The lane still reads as throughput-bound rather than
  allocator-bound on stored evidence, but late RSS now needs to stay in view
  as an explicit guardrail while chasing further speed.
- The accumulated lesson is narrower now: do not reopen another dormant
  cached-summary replay or another contender-rank-helper replay first. The
  live hit-path slice is now the landed code under test, and the next move is
  to carry that same rerun shape deeper into the stored `332/335/408/437/454`
  region or step `5`, not to restart code exploration first.

## Best Current Inference
The current full-profile runtime reference is
`runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`, but the
best current candidate to beat is
`runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`.

That stopped `v2` candidate is now the right speed target for the next slice:
- it preserved the honest `39/40/41/42/43` retained-prefix story
- it improved every decisive matched checkpoint through the prior stored `484`
  wall
- it moved materially past that prior stored wall to `1038`
- it still did not reach step `5`
- it kept the visible later wall aggregation-first, with connectivity second
  and exact `nu` third
- it won by cutting active exact `nu` work on the retained plateau while
  cached-summary reopen stayed dormant
- it kept the run memory-safe, even though late RSS drift rose materially by
  the stopped `1038` read

The newer follow-ups then answered three more runtime questions from stored
evidence:
- the rank-scan helper lowered measured aggregation first at `140/163`, but
  connectivity and exact `nu` both rose more than that aggregation win saved
- the compact-summary reuse follow-up preserved the same honest `39/40/41`
  surfaces through the stored `163` read, but it regressed every decisive
  matched checkpoint and never engaged cached-summary reopen on stored
  evidence through the max stored `165` read
- the prefix-`nu`-context follow-up preserved the same honest `39/40/41`
  surfaces through the stored `239` read and materially improved the matched
  `140/163/231/239` checkpoints by cutting active exact `nu`, even though
  aggregation and connectivity both rose slightly
- the deeper stopped rerun on that same prefix-`nu` slice then re-earned the
  decisive `140/163/332/335/408/437/454/484` region on a fresh run id, stayed
  ahead of the current runtime reference there, re-opened `42/43`, moved the
  stored wall to `1038`, and kept cached-summary reopen dormant throughout

The next honest question is now whether the current harness-backed
improvement slice can carry its stored `24/43/44/54/74/76/140/163/228/229`
win into the stopped `v2` candidate's later `332/335/408/437/454/484`
region without giving back honesty. Aggregation and late RSS should stay in
view as explicit guardrails while answering that.

## New Planning Input
- The missing iteration-speed layer is now landed end-to-end:
  the deterministic replay harness and benchmark path are landed, the stored
  plateau fixture corpus now covers `39 / 144845`, `40 / 147639`,
  `41 / 154842`, `42 / 157636`, and `43 / 160430`, and the local replay
  timings are now stored for all five retained surfaces.
- The favored facts-only hot loop is now landed in code:
  the step-`4` hit path can stay on clause refs plus predecoded connectivity
  facts plus predecoded structural-`nu` facts until a prefix survives, rather
  than paying repeated per-clause decode work on the hit path itself.
- The refreshed local replay read on that landed slice is now a caution, not a
  proof of keep on its own:
  only the stored `41 / 154842` surface improved, and the aggregate
  five-surface total rose from `153124 us` to `163951 us`, but the fresh
  full-profile rerun still materially won through the stored `229` read.
- The compact-summary reopen story should stay narrow: do not try to wake the
  full cached-summary reopen path first while it is still dormant on the
  decisive `39/40/41` surfaces. If second-pass duplication still matters after
  the facts-only slice, prefer a tiny survivor sketch that stores only the few
  clause refs/facts that established the best primary rank or remain tie-break
  relevant.
- The next code slice after the in-flight rerun settles should make the local
  continuation cone the primary scoring object:
  use the already-landed
  `SingleClauseStructuralNuContext`,
  `TerminalClauseNuFacts`, and
  `structural_nu_single_clause_upper_bound`
  to carry cheap exact or exact-safe prefix-local interval bounds on
  bar-clearability and overshoot before terminal assembly, so more of the
  decisive step-`4` decision is made from the current prefix rather than from
  full future materialization.
- The hot path should stop reconstructing clause facts on demand:
  `pen-eval/src/nu.rs` still lets `TerminalClauseNuFacts::from_clause(...)`
  collect, sort, and deduplicate library refs dynamically, while
  `structural_nu_with_clause_facts(...)` is the cheaper path that assumes
  those facts already exist. The next honest direction is to make
  `TerminalClauseNuFacts` a mandatory clause-catalog sidecar and keep
  `structural_nu_with_clause(...)` off the winning hot path entirely.
- The late step-`4` remaining-one kernel should split more explicitly into a
  true hit-path plateau kernel and a general fallback kernel:
  stored decisive later surfaces already keep
  `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0`, so the no-miss plateau
  path should be specialized harder around clause refs plus predecoded
  connectivity facts plus predecoded structural-`nu` facts, with fewer
  branches and fewer generic checks.
- A low-risk harness-backed micro-slice remains a dense bitset or fixed
  smallset for `lib_refs` membership inside
  `SingleClauseStructuralNuContext`, since the facts path now pays repeated
  membership checks directly inside the live winning hot loop.
  The preferred shape is now a tiered representation:
  inline small array for the common case, dense bitset once the ref count
  crosses a threshold, and sorted boxed slices only where serialization or
  debug parity actually needs them.
- The replay harness should stay the hard local admission gate for serious
  optimization work:
  every new slice should prove either fewer exact-`nu` evaluations or lower
  measured aggregation time on the stored plateau fixtures before it earns
  another expensive full-profile rerun.
- Deterministic batched parallel reduction of the per-clause summary loop is a
  later, higher-risk swing. Keep it gated behind the replay harness and
  deterministic merge-parity checks first.

## Immediate Next Move
1. Keep `runs/codex-claim-release-step4-kernel-open-band-handoff-v1` as the
   short step-`4` baseline and
   `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`
   as the best current full-profile candidate to beat on speed until a later
   rerun re-earns `332/335/408/437/454/484` or moves past `1038`.
2. Do not spend another turn on a plain new-code exploration first, and do not
   reopen another unchanged connectivity-first retry, accumulator-only replay,
   contender-rank-helper replay, cached-summary-reuse replay, metadata retry,
   clause-load-only replay, or timing-only replay first.
3. Keep the replay harness corpus complete at
   `tests/fixtures/claim_runtime/remaining_one_plateau_fixtures.json` with
   the stored `39@24`, `40@74`, `41@140`, `42@332`, and `43@335` surfaces,
   and refresh it only through the seeded release-harness path instead of
   restarting from zero.
4. Keep the refreshed replay benchmark honest:
   `tests/fixtures/claim_runtime/remaining_one_plateau_benchmark.json` is now
   softer overall on this machine snapshot, so treat it as a caution while
   judging the stored full-profile rerun evidence rather than as a local win.
5. Keep the landed facts-only slice intact for the same full-profile rerun
   shape:
   clause refs plus predecoded connectivity facts plus predecoded
   structural-`nu` facts on the hit path, since stored later surfaces already
   keep `terminal_summary_admissibility_checks = 0` and
   `terminal_summary_fallback_connectivity_checks = 0`.
6. Do not start a new code slice first.
   The next runtime move is to carry
   `runs/codex-claim-release-full-aggregation-open-band-structural-nu-facts-v1`
   or an equivalent continuation of the same binary deeper into
   `332/335/408/437/454/484` or step `5`, because it has already re-earned
   and beaten `24/43/44/54/74/76/140/163/228/229` honestly.
7. When the current rerun settles and the next code slice becomes honest
   again, make the local continuation cone the primary scoring object:
   push cheap exact or exact-safe prefix-local interval pruning through
   `SingleClauseStructuralNuContext`,
   `TerminalClauseNuFacts`, and
   `structural_nu_single_clause_upper_bound`
   so more bar-clearability and overshoot decisions are made before terminal
   assembly.
8. Treat predecoded clause facts as mandatory on the hit path:
   keep `TerminalClauseNuFacts` as a clause-catalog sidecar and do not reopen
   `structural_nu_with_clause(...)` on the winning hot path when
   `structural_nu_with_clause_facts(...)` already has the cheaper shape.
9. Keep the next specialization narrow:
   split the step-`4` remaining-one kernel into a true no-miss plateau
   hit-path kernel and a general fallback kernel, prefer the tiny survivor
   sketch over waking dormant general reopen machinery, and keep the dense
   `lib_refs` representation as the low-risk data-structure follow-up.
10. Require the replay harness to veto weak slices:
    before any future full-profile rerun, the slice should show either fewer
    exact-`nu` evaluations or lower aggregation time on the stored plateau
    fixtures, and deterministic batched parallel reduction should stay gated
    behind that same replay evidence plus merge-parity checks.
11. The current landed slice already passed:
   - `cargo test -p pen-search claim_`
   - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`
   - `cargo test -p pen-eval single_clause_context_matches_full_structural_nu`
   so the next honest move is the intended-profile rerun itself, not another
   local code-only pass first.
12. Compare that rerun against
   `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`
   first. Branch to parity, breadth, compare, benchmark, or certification work
   only after a later full-profile rerun either reaches step `5` or moves
   materially past the current stored `1038` wall.
