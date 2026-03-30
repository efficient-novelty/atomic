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
- Current full-profile runtime reference: `runs/codex-claim-release-full-open-band-handoff-followup-v1`
- Earlier same-binary full-profile comparison: `runs/codex-claim-release-full-open-band-handoff-v1`
- Comparison full-profile baseline: `runs/codex-claim-release-full-kernel-aggregation-v1`
- Earlier late-surface diagnostic: `runs/codex-claim-release-step4-kernel-late-profile-v1`

## Revised Working Diagnosis
- The kept open-band handoff code is now re-earned on both short evidence and repeated full-profile evidence.
- The honest retained-prefix story on the current full-profile runtime reference is now:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through the stored `229` read
- The new full-profile follow-up slightly trailed the earlier same-binary reference at `24/43/44/54/74/76`, but it improved the decisive later checkpoints and moved one stored checkpoint farther:
  - `140`: `2571309 / 2561049` instead of `2575049 / 2564601`
  - `163`: `2978288 / 2966621` instead of `2985344 / 2973404`
  - `228`: `4189959 / 4174213` instead of `4209220 / 4192906`
  - it then continued to `229`: `4211079 / 4195271` while still in step `4`
- The later stored wall on the current winner now reads as a connectivity-first step-`4` throughput composite on the post-`140` surface:
  - `140`: connectivity `= 758717336 us`, aggregation `= 751161774 us`, exact `nu` `= 525949191 us`, terminal clause-filter handoff `= 10375779 us`
  - `163`: connectivity `= 884102477 us`, aggregation `= 862160190 us`, exact `nu` `= 609549050 us`, terminal clause-filter handoff `= 12108178 us`
  - `228`: connectivity `= 1238402593 us`, aggregation `= 1199772615 us`, exact `nu` `= 872697561 us`, terminal clause-filter handoff `= 17145752 us`
  - `229`: connectivity `= 1243971258 us`, aggregation `= 1208974002 us`, exact `nu` `= 875496908 us`, terminal clause-filter handoff `= 17225219 us`
  - `terminal_summary_admissibility_checks = 0` through the stored `229` read
- Observed RSS reached only `833540096` bytes at `229`, so the blocker remains throughput rather than a return of the old allocator-failure story.
- The rerun never reached step `5`; `reports/steps/step-05-live.ndjson` is absent.
- Because the stop was external during step `4`, `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`; the authoritative evidence for this rerun lives in `reports/steps/step-04-live.ndjson`.

## Honest Read
- The broader open-band handoff cut remains the short keep winner and is now re-earned again on the real intended profile.
- The intended profile still stalls in step `4`, but plain rerun-only uncertainty is now closed: the post-`140` wall stayed at `41 groups / 154842 candidates` and stayed connectivity-first on fresh stored evidence.
- The later blocker is no longer clause-filter handoff or allocator pressure first; it is the post-`140` step-`4` surface where connectivity now leads, aggregation stays second, exact `nu` stays third, and terminal clause-filter handoff stays tiny.
- The next honest first cut is therefore a later-surface connectivity-side runtime slice on the current winner, not another plain intended-profile rerun and not an aggregation-first retry.

## Goal
Land one narrow later-surface connectivity-side cut on the current winner and re-earn the late step-`4` read honestly.

## Do This Next

### 1. Keep The Current Runtime Reference
Keep the code behind:
- `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- `runs/codex-claim-release-full-open-band-handoff-followup-v1`
- `runs/codex-claim-release-full-open-band-handoff-v1`
- `runs/codex-claim-release-step4-kernel-late-profile-v1`

Keep `runs/codex-claim-release-full-kernel-aggregation-v1` only as the broader comparison baseline.

Do not reopen first:
- another plain intended-profile rerun with no code or new runtime question
- another early short step-`4` micro-slice
- another unchanged reopened-connectivity replay
- another metadata retry
- another admitted-kernel-only replay
- another clause-load-only or bookkeeping/bound-only cleanup
- an aggregation-first slice before a new connectivity-side read exists

### 2. Cut Later Connectivity First
Land one narrow code change that targets the post-`140` connectivity-side exact summary work on the current winner.

If code changes land, use a new run id that states the slice.

Do not reintroduce first:
- the parent-summary connectivity lookup reuse exactly as previously measured
- new clause-filter metadata work
- new accept-rank or aggregation-first bookkeeping work before connectivity moves

### 3. Re-Earn Stored Evidence
After code changes land, rerun release claim with live checkpoint persistence on and let it cover at least the stored `140/163/228` region on the new slice.

Open at least:
- `reports/steps/step-04-live.ndjson`
- `reports/steps/step-05-live.ndjson` if it exists
- `telemetry.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:
- did the new slice preserve the `39 / 144845`, `40 / 147639`, and `41 / 154842` surfaces honestly?
- did the post-`140` connectivity time move materially first?
- did aggregation or exact `nu` become the new lead if connectivity moved?
- did the run move materially past the stored `229` wall or reach step `5`?
- did observed RSS stay well below the old allocator-failure band?

### 4. Re-Earn Only The Validation Needed
If new code changes land before the rerun, rerun only:
- `cargo test -p pen-search claim_`
- `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

If the next turn is doc-only or analysis-only with no code changes, do not reopen extra tests first.

## Keep Or Branch Decision
After the next connectivity-side slice:
- stay on runtime work if the intended profile still stalls in step `4`
- keep `runs/codex-claim-release-full-open-band-handoff-followup-v1` as the runtime reference until a later rerun beats its stored `229` wall honestly
- branch to parity, breadth, compare, benchmark, and certification work only after a later full-profile rerun reaches step `5` or moves materially past the current stored `229` wall

## Stop Condition For This Note
Rewrite this file as soon as one new stored follow-up shows one of these is true:
- the current full-profile runtime reference reaches a new later blocker honestly
- the intended profile finally moves past the step-`4` wall
- the next slice fails for a different structural reason than the current stored evidence predicts
- runtime work is no longer the next honest move
