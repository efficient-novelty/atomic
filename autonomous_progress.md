# Autonomous Claim Lane Progress
Last updated: 2026-03-30

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final gate.

## Current Status
- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current full-profile runtime reference is `runs/codex-claim-release-full-aggregation-open-band-compact-v1`.
- The previous full-profile runtime reference is `runs/codex-claim-release-full-connectivity-facts-v1`.
- The latest measured slice is `runs/codex-claim-release-full-aggregation-open-band-compact-v1`.
- That new intended-profile follow-up preserved the honest retained-prefix story and then reopened it twice farther before moving the stored wall again:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
-  - `43 groups / 160430 candidates` from `335` through the stored `437` read
- It materially improved every decisive stored matched checkpoint versus the previous runtime reference:
  - `24`: `382590 / 379801` instead of `385802 / 383018`
  - `43`: `690362 / 686494` instead of `698661 / 694743`
  - `44`: `704899 / 700975` instead of `713483 / 709504`
  - `54`: `868693 / 864182` instead of `880003 / 875413`
  - `74`: `1199861 / 1194114` instead of `1210334 / 1204512`
  - `76`: `1239371 / 1233502` instead of `1250591 / 1244645`
  - `140`: `2376318 / 2366374` instead of `2389805 / 2379752`
  - `163`: `2764278 / 2752823` instead of `2773573 / 2762030`
  - `228`: `3893876 / 3878458` instead of `3924018 / 3908221`
  - `229`: `3913384 / 3897912` instead of `3945205 / 3929337`
  - `332`: `5752765 / 5731419` instead of `5857640 / 5835050`
  - `335`: `5806317 / 5784752` instead of `5912135 / 5889319`
  - `408`: `7058211 / 7032377` instead of `7191639 / 7164178`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The later bucket order stayed aggregation first honestly while also improving the other later buckets versus the previous runtime reference:
  - `140`: aggregation `= 736648605 us`, connectivity `= 584518736 us`, exact `nu` `= 514778949 us`, terminal clause-filter handoff `= 16302812 us`
  - `163`: aggregation `= 848880527 us`, connectivity `= 685516569 us`, exact `nu` `= 599314951 us`, terminal clause-filter handoff `= 19619817 us`
  - `228`: aggregation `= 1181979703 us`, connectivity `= 963171133 us`, exact `nu` `= 859172298 us`, terminal clause-filter handoff `= 28959988 us`
  - `229`: aggregation `= 1190946496 us`, connectivity `= 967419769 us`, exact `nu` `= 861934419 us`, terminal clause-filter handoff `= 29077770 us`
  - `332`: aggregation `= 1750150164 us`, connectivity `= 1399874662 us`, exact `nu` `= 1295406289 us`, terminal clause-filter handoff `= 43827165 us`
  - `335`: aggregation `= 1765342397 us`, connectivity `= 1412824636 us`, exact `nu` `= 1308406106 us`, terminal clause-filter handoff `= 44274555 us`
  - `408`: aggregation `= 2125653593 us`, connectivity `= 1725842418 us`, exact `nu` `= 1597252620 us`, terminal clause-filter handoff `= 55875597 us`
  - `437`: aggregation `= 2281770088 us`, connectivity `= 1848246266 us`, exact `nu` `= 1728738686 us`, terminal clause-filter handoff `= 60344895 us`
- `terminal_summary_admissibility_checks = 0` and `terminal_summary_fallback_connectivity_checks = 0` through the stored `437` read.
- Observed RSS rose materially versus the previous runtime reference, reaching `1440825344` bytes at `437`, but it still stayed well below the old allocator-failure band.
- The lane is still compute-bound, and the later wall remains aggregation-first even after the wall moved from `408` to `437`.
- Because the rerun was manually stopped after the new later blocker was already visible, `reports/latest.txt` still reflects completed step `3`, `run.json` still says `status = "running"`, and `reports/steps/step-05-live.ndjson` is absent; the authoritative evidence for this run lives in `reports/steps/step-04-live.ndjson`.

## What Stays Landed
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
- the higher-fidelity late-surface timing accumulation used by the current short diagnostic surface
- the shared terminal-clause connectivity-facts sidecar on the shared clause catalog used by the claim remaining-one summary/materialization path

## Baselines That Matter

### 1. Current Short Baseline
- Run: `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Honest retained shape:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
- The rerun was manually stopped after the stored `76` checkpoint because the decisive short and reopened surfaces had already been captured; one extra stored `77` checkpoint flushed while stopping and kept the same honest `40 groups / 147639 candidates` reopened surface.
- Because the stop was external during step `4`, the authoritative evidence for this short winner is `reports/steps/step-04-live.ndjson`, while `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`.

### 2. Current Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
- It preserved the honest `39/40/41` retained-prefix story from the previous runtime reference, re-earned the `42/157636` and `43/160430` reopens, materially improved every decisive stored matched checkpoint through the previous stored `408` wall, and then pushed the stored step-`4` wall farther to `437`.
- Stored decisive checkpoints:
  - `24`: `382590 / 379801`
  - `43`: `690362 / 686494`
  - `44`: `704899 / 700975`
  - `54`: `868693 / 864182`
  - `74`: `1199861 / 1194114`
  - `76`: `1239371 / 1233502`
  - `140`: `2376318 / 2366374`
  - `163`: `2764278 / 2752823`
  - `228`: `3893876 / 3878458`
  - `229`: `3913384 / 3897912`
  - `332`: `5752765 / 5731419`
  - `335`: `5806317 / 5784752`
  - `408`: `7058211 / 7032377`
  - `437`: `7587461 / 7560060`
- The later retained-prefix surface now reads:
  - `41 groups / 154842 candidates` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `437` read
- Aggregation remains the lead later bucket on the stored `41/42/43` surfaces, with connectivity second, exact `nu` third, and terminal clause-filter handoff still tiny.
- Observed RSS reached `1440825344` bytes at `437`.
- The run never reached step `5` on stored evidence; `reports/steps/step-05-live.ndjson` is absent.

### 3. Previous Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-connectivity-facts-v1`
- It preserved the honest `39/40/41` retained-prefix story from the old open-band-handoff follow-up, reopened to `42/157636` and `43/160430`, and pushed the prior stored wall to `408`, but the new compact open-band aggregation reference now materially beats it at every decisive stored checkpoint and pushes the wall farther again.
- Honest retained shape:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `408` read

### 4. Comparison Full-Profile Baseline
- Run: `runs/codex-claim-release-full-kernel-aggregation-v1`
- It remains the broader comparison baseline only. The current runtime reference stays materially ahead of it.

### 5. Current Late-Surface Diagnostic
- Run: `runs/codex-claim-release-step4-kernel-late-profile-v1`
- It remains the earlier late diagnostic, but the newer full-profile compact open-band aggregation follow-up is now the authoritative later-surface read.

## What Stays Dropped
- ordering and reuse variants: `context-equivalence-v1`, `incumbent-ordering-v1`, `local-two-step-order-v2`, `proof-close-handoff-v1`, `post-plateau-v1`, `post-plateau-materialize-v1`, `post-plateau-summary-cache-v3`
- connectivity-side cache and precompute variants: `kernel-connectivity-v3`, `kernel-connectivity-v4`
- candidate-prep / remap variants: `terminal-candidate-prep-v1`
- telemetry-only slice: `kernel-filter-profile-v1`
- exact primary-rank bookkeeping rewrite: `kernel-rank-bookkeeping-v1`
- bound-merge bookkeeping rewrite: `kernel-bound-merge-v1`
- lazy incumbent-tie `AcceptRank` deferral rewrite: `kernel-lazy-acceptrank-v1`
- summary-side batching rewrite: `kernel-summary-batching-v1`
- summary-side bookkeeping rewrite: `kernel-summary-bookkeeping-v1`
- prefix-wide competition-gate hoist: `kernel-competition-hoist-v1`
- claim open-band admitted-kernel fusion: `kernel-admitted-kernel-v1`
- direct bound/bookkeeping absorb cleanup: `kernel-bound-bookkeeping-v1`
- scratch-slot clause-load reuse: `kernel-clause-load-v1`
- exact-`nu` high-water gate: `kernel-nu-highwater-v1`
- summary-invariants accept-rank prefix-context rewrite: `kernel-summary-invariants-v1`
- summary-rank-context exact tie-break rewrite: `kernel-rank-context-v1`
- terminal-rank sidecar exact contender-rank rewrite: `kernel-terminal-rank-sidecar-v1`
- primary-rank context exact-threshold rewrite: `kernel-primary-context-v1`
- summary-constant bit-cost hoist: `kernel-summary-constant-v1`
- catalog-backed clause bit-cost sidecar: `kernel-catalog-constant-v1`
- bar-clear threshold bookkeeping rewrite: `kernel-summary-threshold-v1`
- exact rank-metadata pack with last-tie canonical-key finalization: `kernel-rank-metadata-v1`
- compact-summary strict-better-incumbent exact-rank deferral: `kernel-aggregation-tiecut-v1`
- eager terminal-clause metadata pack: `kernel-clause-metadata-v1`
- lazy admitted-only metadata retry: `kernel-admitted-metadata-v1`
- parent-summary connectivity lookup reuse: `kernel-reopened-connectivity-v1`

## Revised Working Diagnosis
- The old early RSS cliff remains broken; this is still a step-`4` throughput problem, not a return of the allocator-failure story.
- The compact open-band aggregation slice engaged honestly on the real intended profile: it preserved the kept `39/40/41/42/43` surface story, improved every stored matched checkpoint through the old `408` wall, and moved the later wall to `437`.
- The later wall is still aggregation first on stored evidence, with connectivity second, exact `nu` third, and terminal clause-filter handoff still tiny, so the new runtime question remains inside later summary throughput rather than earlier admissibility or connectivity work.
- Observed RSS now grows materially higher than it did on the previous runtime reference, but it is still far below the old allocator-failure band; the lane still reads as throughput-bound rather than allocator-bound on stored evidence.
- The accumulated lesson from the dropped retries is stronger again: the next honest runtime choice is still a later-surface runtime cut on this new winner rather than another plain rerun-only turn, another connectivity-first retry, or another metadata/clause-load replay.

## Best Current Inference
The current runtime reference is `runs/codex-claim-release-full-aggregation-open-band-compact-v1`.

That run is already good enough to answer the latest aggregation-side question from stored evidence:
- it preserved the honest `39/40/41/42/43` retained-prefix story
- it materially beat the previous runtime reference at every stored matched checkpoint through `408`
- it moved materially past the prior stored `408` wall to `437`
- it still did not reach step `5`
- it kept the visible later wall aggregation-first
- it kept observed RSS well below the old allocator-failure band, even though RSS grew materially higher than on the previous runtime reference

The next honest question is therefore another later-surface runtime cut on this new winner, with the higher observed RSS growth treated as an explicit guardrail rather than ignored.

## Immediate Next Move
1. Keep `runs/codex-claim-release-step4-kernel-open-band-handoff-v1` as the short step-`4` baseline and `runs/codex-claim-release-full-aggregation-open-band-compact-v1` as the current full-profile runtime reference.
2. Do not spend another turn on a plain intended-profile rerun with no code or new runtime question, and do not reopen another unchanged connectivity-first retry, metadata retry, or clause-load-only replay first.
3. Land one narrow later-surface runtime cut on the current winner. Target the post-`228` aggregation-side summary/rank/bookkeeping work that still leads the stored `41/42/43` surface, while keeping the new higher observed RSS growth in view as a hard honesty check.
4. After code changes land, rerun a release claim follow-up with a new run id that states the slice and carry it at least through the stored `228/229/332/335` region and ideally past the new `437` wall or into step `5` on stored evidence.
5. If code changes land before that rerun, rerun only:
   - `cargo test -p pen-search claim_`
   - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`
6. Branch to parity, breadth, compare, benchmark, or certification work only after a later full-profile rerun either reaches step `5` or moves materially past the current stored `437` wall.
