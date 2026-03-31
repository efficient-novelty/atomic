# Autonomous Claim Lane Progress
Last updated: 2026-03-31

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
- The previous full-profile runtime reference is
  `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`.
- The earlier full-profile runtime reference is
  `runs/codex-claim-release-full-aggregation-open-band-compact-v1`.
- The earlier full-profile runtime reference is
  `runs/codex-claim-release-full-connectivity-facts-v1`.
- The latest measured slice is
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v1`,
  and it landed a prefix-side single-clause structural `nu` context for the
  remaining-one exact scoring, compact-summary build, and compact
  materialization hot path on the current winner.
- That later-surface follow-up preserved the honest retained-prefix story
  through the current `41` surface:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through the stored `239` read
  It has not yet re-earned the current reference's `42 / 157636` or
  `43 / 160430` reopens.
- It materially improved the decisive matched later checkpoints it re-earned
  against the current runtime reference:
  - `140`: `1836533 / 1826733` instead of `2014043 / 2005230`
  - `163`: `2135897 / 2124610` instead of `2334208 / 2324213`
  - `231`: `3069187 / 3053464` instead of `3338028 / 3324509`
  - `239`: `3199123 / 3182864` instead of `3475147 / 3461204`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The win is coming from exact `nu`, not from lower aggregation or
  connectivity:
  - `140`: aggregation `= 787415 ms` instead of `763398 ms`, connectivity
    `= 569913 ms` instead of `557557 ms`, exact `nu` `= 293473 ms` instead of
    `511854 ms`, terminal clause-filter handoff `= 14855 ms` instead of
    `13877 ms`
  - `163`: aggregation `= 907945 ms` instead of `877573 ms`, connectivity
    `= 667954 ms` instead of `651431 ms`, exact `nu` `= 343044 ms` instead of
    `594142 ms`, terminal clause-filter handoff `= 17645 ms` instead of
    `16351 ms`
  - `239`: aggregation `= 1361981 ms` instead of `1303826 ms`, connectivity
    `= 993382 ms` instead of `962017 ms`, exact `nu` `= 522245 ms` instead of
    `898852 ms`, terminal clause-filter handoff `= 27858 ms` instead of
    `25090 ms`
- Cached-summary reopen remained dormant on stored evidence through the max
  stored `239` read:
  - `remaining_one_materialized_from_cached_summary = 0`
  - `remaining_one_prefixes_seen = 0`
  - `remaining_one_materialized_compact_direct = 41`
- `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0` through the stored
  `239` read.
- The first stored plateau activation is still `24`, and
  `terminal_summary_plateau_activations = 231996` stayed flat through the
  stored stop.
- Observed RSS stayed well below the old allocator-failure band:
  - `140`: `577634304` bytes
  - `163`: `646213632` bytes
  - `239`: `867381248` bytes
- Because the rerun was manually stopped after the exact-`nu` win through the
  first later surface was already clear, `reports/latest.txt` still reflects
  completed step `3`, `run.json` still says `status = "running"`, and
  `reports/steps/step-05-live.ndjson` is absent; the authoritative evidence
  for this run lives in `reports/steps/step-04-live.ndjson`.

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

### 3. Latest Measured Later-Surface Follow-Up
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

### 4. Latest Failed Later-Surface Follow-Up
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

### 5. Previous Failed Later-Surface Follow-Up
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

### 6. Previous Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`
- It preserved the honest `39/40/41/42/43` retained-prefix story from the
  previous runtime reference, paid only a tiny early retained-surface tax,
  materially improved the later stored checkpoints from `76` onward, and then
  pushed the stored step-`4` wall farther from `437` to `454`.
- The current stage-timing follow-up now keeps the same honest retained-prefix
  story, wins materially at every decisive matched checkpoint, and moves the
  stored wall again from `454` to `484`.

### 7. Earlier Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
- It preserved the honest `39/40/41` retained-prefix story from the previous
  runtime reference, re-earned the `42/157636` and `43/160430` reopens,
  materially improved every decisive stored matched checkpoint through the
  previous stored `408` wall, and then pushed the stored step-`4` wall farther
  to `437`.

### 8. Earlier Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-connectivity-facts-v1`
- It preserved the honest `39/40/41` retained-prefix story from the old
  open-band-handoff follow-up, reopened to `42/157636` and `43/160430`, and
  pushed the prior stored wall to `408`, but the newer compact, scratch-slot,
  and stage-timing references all now stay ahead of it.

### 9. Comparison Full-Profile Baseline
- Run: `runs/codex-claim-release-full-kernel-aggregation-v1`
- It remains the broader comparison baseline only. The current runtime
  reference stays materially ahead of it.

### 10. Current Late-Surface Diagnostic
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
  question on stored evidence: it preserved the honest `39/40/41` surfaces
  through the stored `239` read and materially improved the matched
  `140/163/231/239` checkpoints, but it has not yet re-earned the later
  `42/43/484` region.
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
  is an exact-`nu` cut on that plateau. The open question is no longer whether
  that work is active; it is whether the win survives once the later `42/43`
  reopens come back.
- Observed RSS is slightly lower than the current runtime reference at
  `140/163`, then slightly higher by the stored `239` read; that is still far
  below the old allocator-failure band, so the lane still reads as
  throughput-bound rather than allocator-bound on stored evidence.
- The accumulated lesson is narrower now: do not reopen another dormant
  cached-summary replay or another contender-rank-helper replay first, and do
  not invent another new code slice before this exact-`nu` win is carried far
  enough to prove whether it survives the later wall.

## Best Current Inference
The current runtime reference is
`runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`.

That run is already good enough to answer the latest late-surface
timing-overhead question from stored evidence:
- it preserved the honest `39/40/41/42/43` retained-prefix story
- it improved every decisive matched checkpoint through the prior stored `454`
  wall
- it moved materially past the prior stored `454` wall to `484`
- it still did not reach step `5`
- it kept the visible later wall aggregation-first
- it appears to have removed most of the previously unattributed summary-build
  remainder while keeping the retained-prefix story intact
- it kept observed RSS well below the old allocator-failure band even though
  late RSS drift is now slightly worse than the previous runtime reference

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

The next honest question is now whether that exact-`nu` win survives once the
later `42/43/484` region is re-earned. Aggregation and late RSS should stay in
view as explicit guardrails while answering that.

## Immediate Next Move
1. Keep `runs/codex-claim-release-step4-kernel-open-band-handoff-v1` as the
   short step-`4` baseline and
   `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1` as
   the current full-profile runtime reference. Keep the prefix-`nu`-context
   slice landed, but treat it as provisional until it re-earns the later wall.
2. Do not spend another turn on a plain new-code exploration first, and do not
   reopen another unchanged connectivity-first retry, accumulator-only replay,
   contender-rank-helper replay, cached-summary-reuse replay, metadata retry,
   clause-load-only replay, or timing-only replay first.
3. Re-run the same exact-`nu` slice with a fresh run id and carry it past the
   already-won `140/163/231/239` region into the current
   `332/335/408/437/454/484` band or into step `5`.
4. If that deeper rerun keeps winning once `42 / 157636` and `43 / 160430`
   reopen, promote it to the new runtime reference. If it gives the win back
   there, only then cut the next code slice.
5. If code changes land before that deeper rerun, rerun only:
   - `cargo test -p pen-search claim_`
   - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`
   If the next turn is rerun-only on this already-landed slice, do not reopen
   extra tests first.
6. Branch to parity, breadth, compare, benchmark, or certification work only
   after a later full-profile rerun either reaches step `5` or moves
   materially past the current stored `484` wall.
