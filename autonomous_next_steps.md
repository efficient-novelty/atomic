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
- the compact claim open-band aggregation fast path that bypasses generic admitted-evaluation bookkeeping on the no-evaluations summary kernel
- the higher-fidelity late-surface timing accumulation
- the shared terminal-clause connectivity-facts sidecar on the shared clause catalog used by the claim remaining-one summary/materialization path
- the steady-state scratch-slot `clone_from` reuse on terminal-clause loads inside remaining-one summary/materialization

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
- Current full-profile runtime reference: `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`
- Previous full-profile runtime reference: `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
- Earlier full-profile runtime reference: `runs/codex-claim-release-full-connectivity-facts-v1`
- Earlier full-profile comparison: `runs/codex-claim-release-full-open-band-handoff-followup-v1`
- Comparison full-profile baseline: `runs/codex-claim-release-full-kernel-aggregation-v1`
- Earlier late-surface diagnostic: `runs/codex-claim-release-step4-kernel-late-profile-v1`

## Revised Working Diagnosis
- The scratch-slot `clone_from` slice preserved the kept early and reopened surfaces and then moved the stored step-`4` wall again on the intended profile.
- The honest retained-prefix story on the new full-profile runtime reference is now:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `454` read
- The new full-profile follow-up pays only a tiny early retained-surface tax, stays effectively flat at `74`, improves the later stored checkpoints from `76` onward, and then moves the stored wall from `437` to `454`:
  - `24`: `385100 / 382267` instead of `382590 / 379801`
  - `43`: `693470 / 689512` instead of `690362 / 686494`
  - `44`: `708134 / 704116` instead of `704899 / 700975`
  - `54`: `872387 / 867757` instead of `868693 / 864182`
  - `74`: `1200035 / 1194139` instead of `1199861 / 1194114`
  - `76`: `1239059 / 1233043` instead of `1239371 / 1233502`
  - `140`: `2349156 / 2339322` instead of `2376318 / 2366374`
  - `163`: `2722673 / 2711471` instead of `2764278 / 2752823`
  - `228`: `3837038 / 3821952` instead of `3893876 / 3878458`
  - `229`: `3856643 / 3841498` instead of `3913384 / 3897912`
  - `332`: `5704569 / 5683113` instead of `5752765 / 5731419`
  - `335`: `5758074 / 5736396` instead of `5806317 / 5784752`
  - `408`: `7014570 / 6988332` instead of `7058211 / 7032377`
  - `437`: `7548400 / 7520336` instead of `7587461 / 7560060`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Aggregation is still the lead bucket on the later surface:
  - `140`: aggregation `= 726010427 us`, connectivity `= 584150145 us`, exact `nu` `= 504538111 us`, terminal clause-filter handoff `= 15066519 us`
  - `228`: aggregation `= 1163303413 us`, connectivity `= 958669397 us`, exact `nu` `= 839079028 us`, terminal clause-filter handoff `= 25826199 us`
  - `335`: aggregation `= 1747120766 us`, connectivity `= 1419967511 us`, exact `nu` `= 1283554834 us`, terminal clause-filter handoff `= 41006135 us`
  - `408`: aggregation `= 2108696726 us`, connectivity `= 1738688193 us`, exact `nu` `= 1569842188 us`, terminal clause-filter handoff `= 52588377 us`
  - `437`: aggregation `= 2265681524 us`, connectivity `= 1865546702 us`, exact `nu` `= 1699346967 us`, terminal clause-filter handoff `= 57440082 us`
  - `454`: aggregation `= 2357381452 us`, connectivity `= 1939945849 us`, exact `nu` `= 1774981339 us`, terminal clause-filter handoff `= 60366631 us`
- The kept slice buys that later win mainly by cutting aggregation, exact `nu`, and clause-filter handoff again while paying only a small connectivity regression on the reopened `42/43` surface.
- `terminal_summary_admissibility_checks = 0` and `terminal_summary_fallback_connectivity_checks = 0` through the stored `454` read.
- Observed RSS stays below the previous runtime reference at matched checkpoints through `437`, then rises to `1485086720` bytes at `454`; that is still well below the old allocator-failure band.
- The rerun never reached step `5`; `reports/steps/step-05-live.ndjson` is absent.
- Because the stop was external during step `4`, `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`; the authoritative evidence for this rerun lives in `reports/steps/step-04-live.ndjson`.

## Honest Read
- The scratch-slot `clone_from` slice earned keep on stored evidence: it preserved the honest `39/40/41/42/43` retained-prefix story, stayed effectively flat on the early retained surface, improved the later stored checkpoints from `76` onward, and pushed the later wall from `437` to `454`.
- The visible later blocker did not change category. Aggregation is still first, connectivity is still second, exact `nu` is still third, and terminal clause-filter handoff is still tiny.
- The new secondary pressure signals are a small connectivity regression on the reopened `42/43` surface and continued RSS growth on the later surface. Both remain secondary to the aggregation-led throughput wall on stored evidence, but both now deserve explicit guardrails on the next slice.
- The next honest first cut is therefore another narrow later-surface runtime slice on the current winner, not another plain rerun-only turn and not another connectivity-first, metadata-first, or clause-load-only retry.

## Goal
Land one narrow later-surface runtime cut on the current winner that keeps the new `454` wall gain and either reduces aggregation again or reveals a new later blocker honestly without losing the retained-prefix story.

## Do This Next

### 1. Keep The Current Runtime Reference
Keep the code behind:
- `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
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

### 2. Cut Later Runtime First
Land one narrow code change that targets the post-`332` / post-`335` later-surface summary/rank/bookkeeping work on the current winner.

Bias toward reductions in aggregation-side summary work first, but keep the small reopened-surface connectivity regression and the continued later RSS climb visible while you cut.

If code changes land, use a new run id that states the slice.

Do not reintroduce first:
- the parent-summary connectivity lookup reuse exactly as previously measured
- new clause-filter metadata work
- another connectivity-first slice before a new later runtime read exists

### 3. Re-Earn Stored Evidence
After code changes land, rerun release claim with live checkpoint persistence on and let it cover at least the stored `332/335/437/454` region and ideally move past the stored `454` wall or reach step `5` on the new slice.

Open at least:
- `reports/steps/step-04-live.ndjson`
- `reports/steps/step-05-live.ndjson` if it exists
- `telemetry.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:
- did the new slice preserve the `39 / 144845`, `40 / 147639`, and `41 / 154842` surfaces honestly?
- did it preserve or move the `42 / 157636` and `43 / 160430` surfaces honestly?
- did the post-`332` aggregation time move materially first again?
- did connectivity or exact `nu` become the new lead if aggregation moved?
- did the run move materially past the stored `454` wall or reach step `5`?
- did observed RSS stay well below the old allocator-failure band, and did its later growth flatten or worsen relative to the current runtime reference?

### 4. Re-Earn Only The Validation Needed
If new code changes land before the rerun, rerun only:
- `cargo test -p pen-search claim_`
- `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

If the next turn is doc-only or analysis-only with no code changes, do not reopen extra tests first.

## Keep Or Branch Decision
After the next later-surface slice:
- stay on runtime work if the intended profile still stalls in step `4`
- keep `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1` as the runtime reference until a later rerun beats its stored `454` wall honestly
- branch to parity, breadth, compare, benchmark, and certification work only after a later full-profile rerun reaches step `5` or moves materially past the current stored `454` wall

## Stop Condition For This Note
Rewrite this file as soon as one new stored follow-up shows one of these is true:
- the current full-profile runtime reference reaches a new later blocker honestly
- the intended profile finally moves past the step-`4` wall
- the next slice fails for a different structural reason than the current stored evidence predicts
- runtime work is no longer the next honest move
