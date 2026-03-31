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
- the aggregation-side accept-rank short-circuit that skips full `AcceptRank` construction for primary-dominated bar-clearers
- the claim open-band terminal-clause handoff fast path that keeps exact-admitted open-band surfaces on clause refs instead of per-clause admissibility payloads
- the compact claim open-band aggregation fast path that bypasses generic admitted-evaluation bookkeeping on the no-evaluations summary kernel
- the higher-fidelity late-surface timing accumulation
- the shared terminal-clause connectivity-facts sidecar on the shared clause catalog used by the claim remaining-one summary/materialization path
- the steady-state scratch-slot `clone_from` reuse on terminal-clause loads inside remaining-one summary/materialization
- the boundary-timestamp timing pass on the compact claim open-band no-evaluations summary kernel that keeps the late-surface timing read while removing most of the previously unattributed timing overhead

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
- the first scratch-slot clause-load reuse pass in `runs/codex-claim-release-step4-kernel-clause-load-v1`

## Active Baselines
- Current short baseline: `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Current full-profile runtime reference: `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`
- Previous full-profile runtime reference: `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`
- Earlier full-profile runtime reference: `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
- Earlier full-profile runtime reference: `runs/codex-claim-release-full-connectivity-facts-v1`
- Earlier full-profile comparison: `runs/codex-claim-release-full-open-band-handoff-followup-v1`
- Comparison full-profile baseline: `runs/codex-claim-release-full-kernel-aggregation-v1`
- Earlier late-surface diagnostic: `runs/codex-claim-release-step4-kernel-late-profile-v1`

## Revised Working Diagnosis
- The stage-timing slice preserved the kept early and reopened surfaces, then moved the stored step-`4` wall again on the intended profile.
- The honest retained-prefix story on the new full-profile runtime reference is now:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `484` read
- The new full-profile follow-up materially improved every decisive matched checkpoint from the previous runtime reference and then moved the stored wall from `454` to `484`:
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
  - `140`: aggregation `= 763398133 us`, connectivity `= 557557788 us`, exact `nu` `= 511854832 us`, terminal clause-filter handoff `= 25988115 us`
  - `228`: aggregation `= 1222738289 us`, connectivity `= 916833230 us`, exact `nu` `= 851890919 us`, terminal clause-filter handoff `= 35116478 us`
  - `335`: aggregation `= 1833436572 us`, connectivity `= 1357602031 us`, exact `nu` `= 1302219954 us`, terminal clause-filter handoff `= 46350411 us`
  - `408`: aggregation `= 2218628958 us`, connectivity `= 1675112606 us`, exact `nu` `= 1596901898 us`, terminal clause-filter handoff `= 54702325 us`
  - `454`: aggregation `= 2480479553 us`, connectivity `= 1875448296 us`, exact `nu` `= 1808063671 us`, terminal clause-filter handoff `= 58420696 us`
  - `484`: aggregation `= 2641777960 us`, connectivity `= 2010145015 us`, exact `nu` `= 1932111468 us`, terminal clause-filter handoff `= 65158646 us`
- Inference from matched checkpoint totals:
  the new slice's main win is not lower measured aggregation. Measured aggregation and exact `nu` are slightly higher at matched late checkpoints, but the previously unattributed summary-build tail collapsed sharply. At `454`, the unattributed remainder fell from about `1698726 ms` on the previous runtime reference to about `521722 ms` on the new one.
- `terminal_summary_admissibility_checks = 0` and `terminal_summary_fallback_connectivity_checks = 0` through the stored `484` read.
- Observed RSS is slightly higher than the previous runtime reference on the matched later surface:
  - `408`: `1361702912` bytes instead of `1355993088`
  - `437`: `1444425728` bytes instead of `1436966912`
  - `454`: `1494106112` bytes instead of `1485086720`
  It then rises to `1581830144` bytes at `484`. That is still well below the old allocator-failure band.
- The rerun never reached step `5`; `reports/steps/step-05-live.ndjson` is absent.
- Because the stop was external during step `4`, `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`; the authoritative evidence for this rerun lives in `reports/steps/step-04-live.ndjson`.

## Honest Read
- The stage-timing slice earned keep on stored evidence: it preserved the honest `39/40/41/42/43` retained-prefix story, materially improved every decisive matched checkpoint through the prior stored `454` wall, and then moved the later wall farther to `484`.
- The visible later blocker did not change category. Aggregation is still first, connectivity is still second, exact `nu` is still third, and terminal clause-filter handoff is still tiny.
- The key new nuance is that the new win comes mainly from collapsing previously unattributed summary-build overhead while keeping the retained-prefix story intact. That is an inference from the matched checkpoint totals and bucket sums, not a separately recorded telemetry field.
- The new guardrail is a small but consistent later-surface RSS increase versus the previous runtime reference. It remains secondary to the later aggregation-led throughput wall, but it should stay visible on the next slice.
- The next honest first cut is therefore another narrow later-surface runtime slice on the new winner, now aimed at measured aggregation-side rank/bound/summary work directly rather than another timing-only rewrite or a plain rerun-only turn.

## Goal
Land one narrow later-surface runtime cut on the current winner that keeps the new `484` wall gain and either reduces measured aggregation first or reveals a new lead bucket honestly without losing the retained-prefix story.

## Do This Next

### 1. Keep The Current Runtime Reference
Keep the code behind:
- `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`
- `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`
- `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
- `runs/codex-claim-release-full-connectivity-facts-v1`
- `runs/codex-claim-release-full-open-band-handoff-followup-v1`
- `runs/codex-claim-release-step4-kernel-late-profile-v1`

Keep `runs/codex-claim-release-full-kernel-aggregation-v1` only as the broader comparison baseline.

Do not reopen first:
- another plain intended-profile rerun with no code or new runtime question
- another unchanged replay of the current winner
- another early short step-`4` micro-slice
- another unchanged reopened-connectivity replay
- another metadata retry
- another admitted-kernel-only replay
- another clause-load-only or bookkeeping/bound-only cleanup
- another timing-only rewrite that merely reattributes the same late-surface cost with no new runtime question

### 2. Cut Later Runtime First
Land one narrow code change that targets the post-`332` / post-`335` measured aggregation-side summary/rank/bound work on the current winner.

Bias toward reductions in measured aggregation first. Keep the small later-surface RSS increase visible while you cut, and treat connectivity and exact `nu` as secondary follow-up buckets unless a new stored read shows one of them taking the lead honestly.

If code changes land, use a new run id that states the slice.

Do not reintroduce first:
- the parent-summary connectivity lookup reuse exactly as previously measured
- new clause-filter metadata work
- another connectivity-first slice before a new later runtime read exists
- another timing-only replay with no new aggregation-side runtime question

### 3. Re-Earn Stored Evidence
After code changes land, rerun release claim with live checkpoint persistence on and let it cover at least the stored `332/335/408/437/454` region and ideally move past the stored `484` wall or reach step `5` on the new slice.

Open at least:
- `reports/steps/step-04-live.ndjson`
- `reports/steps/step-05-live.ndjson` if it exists
- `telemetry.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:
- did the new slice preserve the `39 / 144845`, `40 / 147639`, and `41 / 154842` surfaces honestly?
- did it preserve or move the `42 / 157636` and `43 / 160430` surfaces honestly?
- did the post-`332` measured aggregation time finally move materially first?
- did connectivity or exact `nu` become the new lead if aggregation moved?
- did the run move materially past the stored `484` wall or reach step `5`?
- did observed RSS stay well below the old allocator-failure band, and did its later growth flatten or worsen relative to the current runtime reference?

### 4. Re-Earn Only The Validation Needed
If new code changes land before the rerun, rerun only:
- `cargo test -p pen-search claim_`
- `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

If the next turn is doc-only or analysis-only with no code changes, do not reopen extra tests first.

## Keep Or Branch Decision
After the next later-surface slice:
- stay on runtime work if the intended profile still stalls in step `4`
- keep `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1` as the runtime reference until a later rerun beats its stored `484` wall honestly
- branch to parity, breadth, compare, benchmark, and certification work only after a later full-profile rerun reaches step `5` or moves materially past the current stored `484` wall

## Stop Condition For This Note
Rewrite this file as soon as one new stored follow-up shows one of these is true:
- the current full-profile runtime reference reaches a new later blocker honestly
- the intended profile finally moves past the step-`4` wall
- the next slice fails for a different structural reason than the current stored evidence predicts
- runtime work is no longer the next honest move
