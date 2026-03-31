# Autonomous Claim Lane: Next Operational Slice
Last updated: 2026-03-31

This note is the exact next work order for `desktop_claim_shadow`.

Assume the following are already landed and should stay landed:
- delayed materialization
- the incumbent-primary remaining-one fast path
- the one-pass `structural_nu` summary-build fast path
- the algebraic `nu` ceiling patch
- the family-agnostic claim terminal-admissibility shortcut
- the exact non-allocating connectivity summary scan
- the terminal-only cached parent connectivity decision
- the aggregation-side accept-rank short-circuit that skips full `AcceptRank`
  construction for primary-dominated bar-clearers
- the claim open-band terminal-clause handoff fast path that keeps
  exact-admitted open-band surfaces on clause refs instead of per-clause
  admissibility payloads
- the compact claim open-band aggregation fast path that bypasses generic
  admitted-evaluation bookkeeping on the no-evaluations summary kernel
- the higher-fidelity late-surface timing accumulation
- the shared terminal-clause connectivity-facts sidecar on the shared clause
  catalog used by the claim remaining-one summary/materialization path
- the steady-state scratch-slot `clone_from` reuse on terminal-clause loads
  inside remaining-one summary/materialization
- the boundary-timestamp timing pass on the compact claim open-band
  no-evaluations summary kernel that keeps the late-surface timing read while
  removing most of the previously unattributed timing overhead

Assume the following were already measured and should stay dropped as
standalone next moves:
- ordering and reuse variants
- expr-keyed or clause-side connectivity cache variants
- terminal-candidate prep or remap variants
- telemetry-only filter profiling as a separate slice before a later
  full-profile read exists
- the exact cross-multiplied primary-rank bookkeeping rewrite in
  `runs/codex-claim-release-step4-kernel-rank-bookkeeping-v1`
- the constant-`kappa` bound-merge rewrite in
  `runs/codex-claim-release-step4-kernel-bound-merge-v1`
- the lazy incumbent-tie `AcceptRank` rewrite in
  `runs/codex-claim-release-step4-kernel-lazy-acceptrank-v1`
- the local summary batching rewrite in
  `runs/codex-claim-release-step4-kernel-summary-batching-v1`
- the shared reason-key summary bookkeeping rewrite in
  `runs/codex-claim-release-step4-kernel-summary-bookkeeping-v1`
- the prefix-wide competition-gate hoist in
  `runs/codex-claim-release-step4-kernel-competition-hoist-v1`
- the claim open-band admitted-kernel fusion in
  `runs/codex-claim-release-step4-kernel-admitted-kernel-v1`
- the direct bound/bookkeeping cleanup in
  `runs/codex-claim-release-step4-kernel-bound-bookkeeping-v1`
- the exact rank-metadata pack in
  `runs/codex-claim-release-step4-kernel-rank-metadata-v1`
- the compact-summary strict-better-incumbent exact-rank deferral in
  `runs/codex-claim-release-step4-kernel-aggregation-tiecut-v1`
- the parent-summary connectivity lookup reuse in
  `runs/codex-claim-release-step4-kernel-reopened-connectivity-v1`
- the eager clause-filter-wide metadata pack in
  `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
- the lazy admitted-only metadata retry in
  `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`
- the first scratch-slot clause-load reuse pass in
  `runs/codex-claim-release-step4-kernel-clause-load-v1`
- the helper-level contender-rank fused scan in
  `runs/codex-claim-release-full-aggregation-open-band-rank-scan-v1`
- the cached compact-summary reopen follow-up in
  `runs/codex-claim-release-full-aggregation-open-band-compact-summary-reuse-v1`

## Active Baselines
- Current short baseline:
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Current full-profile runtime reference:
  `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`
- Current live intended-profile rerun:
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`
- Previous proof-of-win intended-profile rerun:
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v1`
- Previous full-profile runtime reference:
  `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`
- Earlier full-profile runtime reference:
  `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
- Earlier full-profile runtime reference:
  `runs/codex-claim-release-full-connectivity-facts-v1`
- Earlier full-profile comparison:
  `runs/codex-claim-release-full-open-band-handoff-followup-v1`
- Comparison full-profile baseline:
  `runs/codex-claim-release-full-kernel-aggregation-v1`
- Earlier late-surface diagnostic:
  `runs/codex-claim-release-step4-kernel-late-profile-v1`

## Revised Working Diagnosis
- The stage-timing slice preserved the kept early and reopened surfaces, then
  moved the stored step-`4` wall again on the intended profile.
- The honest retained-prefix story on the current full-profile runtime
  reference is still:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `484` read
- The current full-profile runtime reference materially improved every decisive
  matched checkpoint from the previous runtime reference and then moved the
  stored wall from `454` to `484`:
  - `24`: `327917 / 325321` instead of `385100 / 382267`
  - `43`: `591083 / 587479` instead of `693470 / 689512`
  - `44`: `603570 / 599913` instead of `708134 / 704116`
  - `54`: `743997 / 739797` instead of `872387 / 867757`
  - `74`: `1023572 / 1018283` instead of `1200035 / 1194139`
  - `76`: `1057864 / 1052464` instead of `1239059 / 1233043`
  - `140`: `2014043 / 2005230` instead of `2349156 / 2339322`
  - `163`: `2334208 / 2324213` instead of `2722673 / 2711471`
  - `228`: `3287422 / 3274060` instead of `3837038 / 3821952`
  - `229`: `3304737 / 3291324` instead of `3856643 / 3841498`
  - `332`: `4885155 / 4866258` instead of `5704569 / 5683113`
  - `335`: `4931017 / 4911930` instead of `5758074 / 5736396`
  - `408`: `6031554 / 6007913` instead of `7014570 / 6988332`
  - `437`: `6494645 / 6469171` instead of `7548400 / 7520336`
  - `454`: `6770742 / 6744132` instead of `7860534 / 7831399`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Aggregation is still the lead bucket on the later surface:
  - `140`: aggregation `= 763398133 us`, connectivity `= 557557788 us`,
    exact `nu` `= 511854832 us`, terminal clause-filter handoff `= 25988115 us`
  - `228`: aggregation `= 1222738289 us`, connectivity `= 916833230 us`,
    exact `nu` `= 851890919 us`, terminal clause-filter handoff `= 35116478 us`
  - `335`: aggregation `= 1833436572 us`, connectivity `= 1357602031 us`,
    exact `nu` `= 1302219954 us`, terminal clause-filter handoff `= 46350411 us`
  - `408`: aggregation `= 2218628958 us`, connectivity `= 1675112606 us`,
    exact `nu` `= 1596901898 us`, terminal clause-filter handoff `= 54702325 us`
  - `454`: aggregation `= 2480479553 us`, connectivity `= 1875448296 us`,
    exact `nu` `= 1808063671 us`, terminal clause-filter handoff `= 58420696 us`
  - `484`: aggregation `= 2641777960 us`, connectivity `= 2010145015 us`,
    exact `nu` `= 1932111468 us`, terminal clause-filter handoff `= 65158646 us`
- The current live intended-profile rerun on the already-landed
  prefix-`nu` context slice has now re-earned the later `42/43` reopens and
  remains ahead of the current runtime reference at the matched
  `140/163/332/335/408` checkpoints, but it has not yet re-earned the stored
  `437/454/484` wall or reached step `5`.
- The biggest missing operational layer is now iteration speed: the repo still
  has no deterministic replay harness for the stable retained plateau
  prefixes `39 / 144845`, `40 / 147639`, `41 / 154842`, `42 / 157636`, and
  `43 / 160430`, even though the stored step-live artifacts are now stable
  enough to replay the hot remaining-one kernel on fixtures instead of waiting
  on multi-hour full reruns.
- The failed rank-scan follow-up on the current winner,
  `runs/codex-claim-release-full-aggregation-open-band-rank-scan-v1`,
  preserved the honest `39/40/41` retained-prefix story through the stored
  `163` read and stayed well below the old allocator-failure band, but it
  still regressed every decisive matched checkpoint through `163` while only
  lowering measured aggregation enough to shift more time into connectivity and
  exact `nu`.
- The latest failed follow-up on the current winner,
  `runs/codex-claim-release-full-aggregation-open-band-compact-summary-reuse-v1`,
  preserved the honest `39/40/41` retained-prefix story through the stored
  `163` read and stayed well below the old allocator-failure band, but it also
  regressed every decisive matched checkpoint through `163`:
  - `24`: `333574 / 330822` instead of `327917 / 325321`
  - `43`: `602370 / 598494` instead of `591083 / 587479`
  - `44`: `614946 / 611012` instead of `603570 / 599913`
  - `54`: `758510 / 753973` instead of `743997 / 739797`
  - `74`: `1043186 / 1037421` instead of `1023572 / 1018283`
  - `76`: `1076868 / 1070982` instead of `1057864 / 1052464`
  - `140`: `2054569 / 2044701` instead of `2014043 / 2005230`
  - `163`: `2388800 / 2377464` instead of `2334208 / 2324213`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- That failed compact-summary-reuse follow-up answered one sharper runtime
  question too: the intended cached-summary reopen path never engaged on the
  decisive stored `39/40/41` surfaces. Through the max stored `165` read:
  - `remaining_one_materialized_from_cached_summary = 0`
  - `remaining_one_prefixes_seen = 0`
  - `remaining_one_materialized_compact_direct = 41`
- The latest failed compact-summary-reuse follow-up did reduce measured
  aggregation first on the stored `41 / 154842` surface, but the total wall
  still regressed because connectivity, exact `nu`, and clause-filter handoff
  rose more than the aggregation win saved:
  - `140`: aggregation `= 749278357 us`, connectivity `= 588117600 us`,
    exact `nu` `= 527205532 us`, terminal clause-filter handoff
    `= 16844349 us`
  - `163`: aggregation `= 864250355 us`, connectivity `= 689039849 us`,
    exact `nu` `= 613184156 us`, terminal clause-filter handoff
    `= 20206734 us`
- `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0` through the stored
  `165` read on the latest failed follow-up.
- Observed RSS stayed well below the old allocator-failure band on the latest
  failed follow-up:
  - `140`: `577400832` bytes
  - `163`: `644460544` bytes
- Because the latest stop was external during step `4`,
  `reports/latest.txt` still reflects completed step `3`, `run.json` still
  says `status = "running"`, and `reports/steps/step-05-live.ndjson` is
  absent; the authoritative evidence for that rerun lives in
  `reports/steps/step-04-live.ndjson`.

## Honest Read
- The stage-timing slice earned keep on stored evidence: it preserved the
  honest `39/40/41/42/43` retained-prefix story, materially improved every
  decisive matched checkpoint through the prior stored `454` wall, and then
  moved the later wall farther to `484`.
- The visible later blocker did not change category. Aggregation is still
  first, connectivity is still second, exact `nu` is still third, and terminal
  clause-filter handoff is still tiny.
- The failed rank-scan follow-up already showed that a helper which only lowers
  measured aggregation on the stored `41` surface can still lose overall
  runtime once connectivity and exact `nu` absorb that win.
- The failed compact-summary-reuse follow-up answered a different local
  question: the retained-prefix plateau after state `24` is real, but this
  specific cached-summary reopen path is dormant on the decisive `39/40/41`
  surfaces, so it cannot explain or relieve the active later cost yet.
- The next honest operational addition is therefore not just another full
  rerun. It is a deterministic replay harness for the retained plateau
  prefixes plus one harness-backed hit-path slice on the current winner.
- Because stored later surfaces already keep
  `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0`, the next code slice
  should bias toward the cost of the hit path itself: clause refs plus
  predecoded connectivity facts plus predecoded structural-`nu` facts, not
  another cache-first miss-path rewrite.

## Goal
While the live prefix-`nu` rerun finishes, add the missing iteration-speed
layer and line up the next harness-backed runtime cut on the current winner:
- build a deterministic replay harness for the stable retained plateau
  prefixes and benchmark only
  `compute_terminal_prefix_completion_summary_from_candidates(...)` on stored
  fixtures
- then land one narrow hit-path slice that pushes the step-`4` loop toward
  clause refs plus predecoded connectivity facts plus predecoded
  structural-`nu` facts without relying on dormant cached-summary reopen logic

Do that without losing the retained-prefix story, deterministic rank parity,
or the later RSS guardrail.

## Do This Next

### 1. Keep The Current Runtime Reference
Keep the code behind:
- `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`
- `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v1`
- `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`
- `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
- `runs/codex-claim-release-full-connectivity-facts-v1`
- `runs/codex-claim-release-full-open-band-handoff-followup-v1`
- `runs/codex-claim-release-step4-kernel-late-profile-v1`

Keep `runs/codex-claim-release-full-kernel-aggregation-v1` only as the broader
comparison baseline.

Keep the current live intended-profile rerun
`runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`
running until it either:
- re-earns the stored `437/454/484` wall
- reaches step `5`
- or stops externally for a new honest reason

Do not reopen first:
- another plain intended-profile rerun with no code or new runtime question
- another unchanged replay of the current winner
- another early short step-`4` micro-slice
- another unchanged contender-rank-helper replay
- another unchanged cached-summary-reuse replay
- another unchanged reopened-connectivity replay
- another metadata retry
- another admitted-kernel-only replay
- another clause-load-only or bookkeeping/bound-only cleanup
- another multi-hour full-profile slice with no harness-backed local replay
- another timing-only rewrite that merely reattributes the same late-surface
  cost with no new runtime question

### 2. Add Iteration-Speed Layer First
Build a deterministic replay harness for the retained plateau prefixes before
cutting the next full-profile runtime slice.

The harness should:
- materialize deterministic fixtures for the stable retained surfaces
  `39 / 144845`, `40 / 147639`, `41 / 154842`, `42 / 157636`, and
  `43 / 160430`
- benchmark only
  `compute_terminal_prefix_completion_summary_from_candidates(...)`
- replay the stored hot kernel from real step-`4` evidence rather than from
  synthetic data
- check deterministic best-rank parity and output shape on every stored
  surface before a full rerun is used to judge the slice

Do not reopen first:
- another later-surface code slice with no harness-backed replay read
- another attempt to wake the full cached-summary reopen mechanism as the
  primary next move
- deterministic batched parallel reduction before the harness exists

### 3. Cut Facts-Only Hot Loop First
After the harness lands, bias the next actual code slice toward the hit path
itself.

Bias toward this shape:
- keep the step-`4` loop on clause refs plus predecoded connectivity facts
  plus predecoded structural-`nu` facts until a prefix survives
- use the existing facts-based helpers directly rather than rebuilding clause
  facts on the hot path
- treat the stored zero
  `terminal_summary_admissibility_checks` and zero
  `terminal_summary_fallback_connectivity_checks` reads as evidence that
  misses are not the main story anymore

If code changes land, use a new run id that states the slice.

Do not reintroduce first:
- another unchanged contender-rank-helper replay with no new runtime question
- another unchanged cached-summary-reuse replay with no new runtime question
- another full cached-summary reopen wake-up attempt on the decisive
  `39/40/41` surfaces
- the parent-summary connectivity lookup reuse exactly as previously measured
- new clause-filter metadata work
- another timing-only replay with no new aggregation-side runtime question

### 4. Keep Secondary Harness-Backed Follow-Ups Narrow
If the facts-only slice wins locally but still leaves too much second-pass
duplication, keep the next follow-ups narrow:
- prefer a tiny survivor sketch that stores only the clause refs/facts needed
  for the best primary rank or remaining tie-break-relevant survivors, rather
  than reopening the full dormant cached-summary reopen mechanism
- prefer a dense bitset or fixed smallset for `lib_refs` membership inside
  `SingleClauseStructuralNuContext` as a low-risk micro-slice
- keep deterministic batched parallel reduction as a later, higher-risk move
  that stays gated behind the replay harness and explicit merge-parity checks

### 5. Re-Earn Stored Evidence
After code changes land, rerun release claim with live checkpoint persistence
on.

At minimum, let it re-earn the stored `140/163` region on the new slice.
Only carry it onward into `228/229` and ideally the stored
`332/335/408/437/454` region or step `5` if it wins the first later surface
honestly.

Open at least:
- `reports/steps/step-04-live.ndjson`
- `reports/steps/step-05-live.ndjson` if it exists
- `telemetry.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:
- did the new slice preserve the `39 / 144845`, `40 / 147639`, and
  `41 / 154842` surfaces honestly, then re-earn `42 / 157636` and
  `43 / 160430` too?
- did the harness-backed slice lower the hit-path cost locally on the replay
  fixtures before the full rerun, and did that local win survive the full
  rerun honestly?
- if cached-summary reopen still stayed dormant, did the new slice still move
  the active post-`140` / post-`163` connectivity and exact `nu` cost enough
  to improve the wall clock on the current winner?
- did aggregation stay the lead bucket, or did a different bucket take the
  lead honestly on the later surface?
- did the run move materially past the stored `484` wall or reach step `5`?
- did observed RSS stay well below the old allocator-failure band, and did its
  later growth flatten or worsen relative to the current runtime reference?

### 6. Re-Earn Only The Validation Needed
If new code changes land before the rerun, rerun only:
- `cargo test -p pen-search claim_`
- `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

If the next turn is doc-only or analysis-only with no code changes, do not
reopen extra tests first.

## Keep Or Branch Decision
After the next later-surface slice:
- stay on runtime work if the intended profile still stalls in step `4`
- keep `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`
  as the runtime reference until a later rerun beats its stored `484` wall
  honestly
- keep the replay harness as a permanent iteration layer for later claim
  runtime slices once it lands
- branch to parity, breadth, compare, benchmark, and certification work only
  after a later full-profile rerun reaches step `5` or moves materially past
  the current stored `484` wall

## Stop Condition For This Note
Rewrite this file as soon as one new stored follow-up shows one of these is
true:
- the current live intended-profile rerun reaches a new later blocker
  honestly
- the current full-profile runtime reference reaches a new later blocker
  honestly
- the intended profile finally moves past the step-`4` wall
- the next slice fails for a different structural reason than the current
  stored evidence predicts
- runtime work is no longer the next honest move
