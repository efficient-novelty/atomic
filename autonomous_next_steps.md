# Autonomous Claim Lane: Next Operational Slice
Last updated: 2026-03-30

This note is the exact next work order for `desktop_claim_shadow`.

Assume the following are already landed and should stay landed:
- delayed materialization
- the incumbent-primary remaining-one fast path
- the one-pass `structural_nu` summary-build fast path
- the algebraic `nu` ceiling patch
- the family-agnostic claim terminal-admissibility shortcut
- the exact non-allocating connectivity summary scan
- the terminal-only cached parent connectivity decision
- the aggregation-side accept-rank short-circuit that skips full `AcceptRank` construction for primary-dominated bar-clearers
- the claim open-band terminal-clause handoff fast path that keeps exact-admitted open-band surfaces on clause refs instead of per-clause admissibility payloads
- the higher-fidelity late-surface timing accumulation
- the shared terminal-clause connectivity-facts sidecar on the shared clause catalog used by the claim remaining-one summary/materialization path

Assume the following were already measured and should stay dropped as standalone next moves:
- ordering and reuse variants
- expr-keyed or clause-side connectivity cache variants
- terminal-candidate prep or remap variants
- telemetry-only filter profiling as a separate slice before a later full-profile read exists
- the exact cross-multiplied primary-rank bookkeeping rewrite in `runs/codex-claim-release-step4-kernel-rank-bookkeeping-v1`
- the constant-`kappa` bound-merge rewrite in `runs/codex-claim-release-step4-kernel-bound-merge-v1`
- the lazy incumbent-tie `AcceptRank` rewrite in `runs/codex-claim-release-step4-kernel-lazy-acceptrank-v1`
- the local summary batching rewrite in `runs/codex-claim-release-step4-kernel-summary-batching-v1`
- the shared reason-key summary bookkeeping rewrite in `runs/codex-claim-release-step4-kernel-summary-bookkeeping-v1`
- the prefix-wide competition-gate hoist in `runs/codex-claim-release-step4-kernel-competition-hoist-v1`
- the claim open-band admitted-kernel fusion in `runs/codex-claim-release-step4-kernel-admitted-kernel-v1`
- the direct bound/bookkeeping cleanup in `runs/codex-claim-release-step4-kernel-bound-bookkeeping-v1`
- the exact rank-metadata pack in `runs/codex-claim-release-step4-kernel-rank-metadata-v1`
- the compact-summary strict-better-incumbent exact-rank deferral in `runs/codex-claim-release-step4-kernel-aggregation-tiecut-v1`
- the parent-summary connectivity lookup reuse in `runs/codex-claim-release-step4-kernel-reopened-connectivity-v1`
- the eager clause-filter-wide metadata pack in `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
- the lazy admitted-only metadata retry in `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`

## Active Baselines
- Current short baseline: `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Current full-profile runtime reference: `runs/codex-claim-release-full-connectivity-facts-v1`
- Previous full-profile runtime reference: `runs/codex-claim-release-full-open-band-handoff-followup-v1`
- Earlier same-binary full-profile comparison: `runs/codex-claim-release-full-open-band-handoff-v1`
- Comparison full-profile baseline: `runs/codex-claim-release-full-kernel-aggregation-v1`
- Earlier late-surface diagnostic: `runs/codex-claim-release-step4-kernel-late-profile-v1`

## Revised Working Diagnosis
- The new connectivity-facts slice preserved the kept early and reopened surfaces and materially improved every stored matched checkpoint through the old `229` wall on the intended profile.
- The honest retained-prefix story on the new full-profile runtime reference is now:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` from `335` through the stored `408` read
- The new full-profile follow-up materially beat the previous runtime reference at every decisive stored checkpoint:
  - `24`: `385802 / 383018` instead of `417678 / 414814`
  - `43`: `698661 / 694743` instead of `760980 / 756851`
  - `44`: `713483 / 709504` instead of `777344 / 773151`
  - `54`: `880003 / 875413` instead of `961793 / 956919`
  - `74`: `1210334 / 1204512` instead of `1325033 / 1318766`
  - `76`: `1250591 / 1244645` instead of `1368485 / 1362079`
  - `140`: `2389805 / 2379752` instead of `2571309 / 2561049`
  - `163`: `2773573 / 2762030` instead of `2978288 / 2966621`
  - `228`: `3924018 / 3908221` instead of `4189959 / 4174213`
  - `229`: `3945205 / 3929337` instead of `4211079 / 4195271`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The old post-`140` connectivity-first wall moved materially first:
  - `140`: connectivity `= 586579312 us` instead of `758717336 us`
  - `163`: connectivity `= 685837174 us` instead of `884102477 us`
  - `228`: connectivity `= 968606567 us` instead of `1238402593 us`
  - `229`: connectivity `= 973284301 us` instead of `1243971258 us`
- But once connectivity moved, the visible later blocker changed honestly. Aggregation is now the lead bucket on the reopened later surface:
  - `140`: aggregation `= 744641415 us`, connectivity `= 586579312 us`, exact `nu` `= 520449833 us`, terminal clause-filter handoff `= 15799216 us`
  - `163`: aggregation `= 857070798 us`, connectivity `= 685837174 us`, exact `nu` `= 604333257 us`, terminal clause-filter handoff `= 18798149 us`
  - `228`: aggregation `= 1200334910 us`, connectivity `= 968606567 us`, exact `nu` `= 869713548 us`, terminal clause-filter handoff `= 28170861 us`
  - `229`: aggregation `= 1210184906 us`, connectivity `= 973284301 us`, exact `nu` `= 872719297 us`, terminal clause-filter handoff `= 28348740 us`
  - `332`: aggregation `= 1799313029 us`, connectivity `= 1424660906 us`, exact `nu` `= 1323961772 us`, terminal clause-filter handoff `= 45034612 us`
  - `335`: aggregation `= 1814967597 us`, connectivity `= 1437778259 us`, exact `nu` `= 1337241409 us`, terminal clause-filter handoff `= 45526234 us`
  - `408`: aggregation `= 2185774562 us`, connectivity `= 1756872641 us`, exact `nu` `= 1633848702 us`, terminal clause-filter handoff `= 58019282 us`
- `terminal_summary_admissibility_checks = 0` and `terminal_summary_fallback_connectivity_checks = 0` through the stored `408` read.
- Observed RSS reached only `964218880` bytes at `408`, so the blocker remains throughput rather than allocator pressure.
- The rerun never reached step `5`; `reports/steps/step-05-live.ndjson` is absent.
- Because the stop was external during step `4`, `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`; the authoritative evidence for this rerun lives in `reports/steps/step-04-live.ndjson`.

## Honest Read
- The later-surface connectivity-side cut engaged, preserved the honest `39/40/41` retained-prefix story, and reopened the intended profile twice more to `42/157636` and `43/160430`.
- The old connectivity-first question is now answered on stored evidence: connectivity moved materially first, and aggregation became the new later lead bucket.
- The next honest first cut is therefore a later-surface aggregation-side runtime slice on the current winner, not another connectivity-first retry and not another plain rerun-only turn.

## Goal
Land one narrow later-surface aggregation-side cut on the current winner and re-earn the new `42/43` step-`4` read honestly.

## Do This Next

### 1. Keep The Current Runtime Reference
Keep the code behind:
- `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- `runs/codex-claim-release-full-connectivity-facts-v1`
- `runs/codex-claim-release-full-open-band-handoff-followup-v1`
- `runs/codex-claim-release-full-open-band-handoff-v1`
- `runs/codex-claim-release-step4-kernel-late-profile-v1`

Keep `runs/codex-claim-release-full-kernel-aggregation-v1` only as the broader comparison baseline.

Do not reopen first:
- another plain intended-profile rerun with no code or new runtime question
- another unchanged connectivity-facts replay
- another early short step-`4` micro-slice
- another unchanged reopened-connectivity replay
- another metadata retry
- another admitted-kernel-only replay
- another clause-load-only or bookkeeping/bound-only cleanup

### 2. Cut Later Aggregation First
Land one narrow code change that targets the post-`140` aggregation-side summary/rank/bookkeeping work on the current winner.

If code changes land, use a new run id that states the slice.

Do not reintroduce first:
- the parent-summary connectivity lookup reuse exactly as previously measured
- new clause-filter metadata work
- another connectivity-first slice before a new aggregation-side read exists

### 3. Re-Earn Stored Evidence
After code changes land, rerun release claim with live checkpoint persistence on and let it cover at least the stored `140/163/228` region and the new `332/335` reopen on the new slice.

Open at least:
- `reports/steps/step-04-live.ndjson`
- `reports/steps/step-05-live.ndjson` if it exists
- `telemetry.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:
- did the new slice preserve the `39 / 144845`, `40 / 147639`, and `41 / 154842` surfaces honestly?
- did it preserve or move the `42 / 157636` and `43 / 160430` surfaces honestly?
- did the post-`140` aggregation time move materially first?
- did connectivity or exact `nu` become the new lead if aggregation moved?
- did the run move materially past the stored `408` wall or reach step `5`?
- did observed RSS stay well below the old allocator-failure band?

### 4. Re-Earn Only The Validation Needed
If new code changes land before the rerun, rerun only:
- `cargo test -p pen-search claim_`
- `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

If the next turn is doc-only or analysis-only with no code changes, do not reopen extra tests first.

## Keep Or Branch Decision
After the next aggregation-side slice:
- stay on runtime work if the intended profile still stalls in step `4`
- keep `runs/codex-claim-release-full-connectivity-facts-v1` as the runtime reference until a later rerun beats its stored `408` wall honestly
- branch to parity, breadth, compare, benchmark, and certification work only after a later full-profile rerun reaches step `5` or moves materially past the current stored `408` wall

## Stop Condition For This Note
Rewrite this file as soon as one new stored follow-up shows one of these is true:
- the current full-profile runtime reference reaches a new later blocker honestly
- the intended profile finally moves past the step-`4` wall
- the next slice fails for a different structural reason than the current stored evidence predicts
- runtime work is no longer the next honest move
