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
- Current full-profile runtime reference: `runs/codex-claim-release-full-open-band-handoff-v1`
- Comparison full-profile baseline: `runs/codex-claim-release-full-kernel-aggregation-v1`
- Earlier late-surface diagnostic: `runs/codex-claim-release-step4-kernel-late-profile-v1`

## Revised Working Diagnosis
- The kept open-band handoff code is now re-earned on both short and full-profile evidence.
- The honest retained-prefix story on the current full-profile runtime reference is now:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through the stored `228` read
- The new full-profile rerun stayed ahead of the earlier full-profile aggregation baseline at every decisive later checkpoint:
  - `140`: `2575049 / 2564601` instead of `3393970 / 3085651`
  - `163`: `2985344 / 2973404` instead of `3942636 / 3584914`
  - it then continued to `228`: `4209220 / 4192906` while still in step `4`
- The later stored wall on the current winner is a tight step-`4` throughput composite:
  - `228`: connectivity `= 1249033160 us`
  - `228`: aggregation `= 1209742017 us`
  - `228`: exact `nu` `= 866913331 us`
  - `228`: terminal clause-filter handoff `= 17262280 us`
  - `terminal_summary_admissibility_checks = 0` through the stored `228` read
- Observed RSS reached only `837795840` bytes at `228`, so the blocker remains throughput rather than a return of the old allocator-failure story.
- The rerun never reached step `5`; `reports/steps/step-05-live.ndjson` is absent.
- Because the stop was external during step `4`, `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`; the authoritative evidence for this rerun lives in `reports/steps/step-04-live.ndjson`.

## Honest Read
- The broader open-band handoff cut is now the current runtime reference on the real intended profile, not just the short keep winner.
- The intended profile still stalls in step `4`, but the wall moved materially later than the old `163` checkpoint.
- The later blocker is no longer clause-filter handoff or allocator pressure first; it is the post-`140` step-`4` surface where connectivity and aggregation stay nearly tied and exact `nu` remains the third cost.
- The next honest unknown is therefore which later step-`4` runtime cut should move first on this winner.

## Goal
Use one later-surface follow-up on the current full-profile runtime reference to choose the next later step-`4` runtime cut honestly.

## Do This Next

### 1. Keep The Current Runtime Reference
Keep the code behind:
- `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- `runs/codex-claim-release-full-open-band-handoff-v1`
- `runs/codex-claim-release-step4-kernel-late-profile-v1`

Keep `runs/codex-claim-release-full-kernel-aggregation-v1` only as the comparison baseline.

Do not reopen first:
- another early short step-`4` micro-slice
- another plain intended-profile rerun with no new runtime question
- another metadata retry
- another unchanged reopened-connectivity replay
- another admitted-kernel-only replay
- another clause-load-only or bookkeeping/bound-only cleanup

### 2. Re-Earn The Later-Bucket Read
Run one release claim follow-up that keeps live checkpoint persistence on and goes far enough to cover at least the stored `140/163/228` region on the current winner.

If the next turn is diagnostic-first with no code changes, use that rerun to decide from stored evidence whether connectivity or aggregation should be attacked first on the post-`140` surface.

If code changes land first, use a new run id that states the slice.

Let the follow-up go far enough to answer from stored evidence:
- does the later step-`4` surface stay at `41 groups / 154842 candidates` or reopen again?
- is connectivity still first there, or does aggregation reclaim the lead?
- is exact `nu` becoming large enough to challenge that pair?
- does any later structural or memory blocker appear before step `5`?

### 3. Read The Stored Artifacts
Open at least:
- `reports/steps/step-04-live.ndjson`
- `reports/steps/step-05-live.ndjson` if it exists
- `telemetry.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:
- did the follow-up preserve the current full-profile runtime reference honestly?
- which later bucket order dominated after the `41 / 154842` reopen?
- did observed RSS stay well below the old allocator-failure band there too?
- did the run finally reach step `5` or expose a different later blocker honestly?

### 4. Re-Earn Only The Validation Needed
If the next turn is rerun-only with no new code changes, do not reopen extra tests first.

If new code changes land before the follow-up rerun, rerun only:
- `cargo test -p pen-search claim_`
- `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

## Keep Or Branch Decision
After the next later-surface follow-up:
- stay on runtime work if the intended profile still stalls in step `4`
- keep `runs/codex-claim-release-full-open-band-handoff-v1` as the runtime reference until a later rerun beats its stored `228` wall honestly
- branch to parity, breadth, compare, benchmark, and certification work only after a later full-profile rerun reaches step `5` or moves materially past the current stored `228` wall

## Stop Condition For This Note
Rewrite this file as soon as one new stored follow-up shows one of these is true:
- the current full-profile runtime reference reaches a new later blocker honestly
- the intended profile finally moves past the step-`4` wall
- the next follow-up fails for a different structural reason than the current stored evidence predicts
- runtime work is no longer the next honest move
