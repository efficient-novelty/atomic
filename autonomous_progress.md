# Autonomous Claim Lane Progress
Last updated: 2026-03-30

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final gate.

## Current Status
- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current full-profile runtime reference is `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`.
- The previous full-profile runtime reference is `runs/codex-claim-release-full-aggregation-open-band-compact-v1`.
- The earlier full-profile runtime reference is `runs/codex-claim-release-full-connectivity-facts-v1`.
- The latest measured slice is `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`.
- That new intended-profile follow-up preserved the honest retained-prefix story and moved the stored wall again:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `454` read
- It paid only a tiny early retained-surface tax, stayed essentially flat at `74`, then turned into a later-surface win versus the previous runtime reference:
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
  - `454`: `7860534 / 7831399`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The later bucket order stayed aggregation first honestly. The new slice cut aggregation, exact `nu`, and clause-filter handoff again on the later surface while paying only a small reopened-surface connectivity regression:
  - `140`: aggregation `= 726010427 us`, connectivity `= 584150145 us`, exact `nu` `= 504538111 us`, terminal clause-filter handoff `= 15066519 us`
  - `163`: aggregation `= 833688517 us`, connectivity `= 682070208 us`, exact `nu` `= 585495291 us`, terminal clause-filter handoff `= 17723301 us`
  - `228`: aggregation `= 1163303413 us`, connectivity `= 958669397 us`, exact `nu` `= 839079028 us`, terminal clause-filter handoff `= 25826199 us`
  - `229`: aggregation `= 1172258021 us`, connectivity `= 963048280 us`, exact `nu` `= 841806350 us`, terminal clause-filter handoff `= 25948163 us`
  - `332`: aggregation `= 1731818025 us`, connectivity `= 1406954753 us`, exact `nu` `= 1270679374 us`, terminal clause-filter handoff `= 40566147 us`
  - `335`: aggregation `= 1747120766 us`, connectivity `= 1419967511 us`, exact `nu` `= 1283554834 us`, terminal clause-filter handoff `= 41006135 us`
  - `408`: aggregation `= 2108696726 us`, connectivity `= 1738688193 us`, exact `nu` `= 1569842188 us`, terminal clause-filter handoff `= 52588377 us`
  - `437`: aggregation `= 2265681524 us`, connectivity `= 1865546702 us`, exact `nu` `= 1699346967 us`, terminal clause-filter handoff `= 57440082 us`
  - `454`: aggregation `= 2357381452 us`, connectivity `= 1939945849 us`, exact `nu` `= 1774981339 us`, terminal clause-filter handoff `= 60366631 us`
- `terminal_summary_admissibility_checks = 0` and `terminal_summary_fallback_connectivity_checks = 0` through the stored `454` read.
- The first stored plateau activation is still `24`, and `terminal_summary_plateau_activations = 231996` stayed flat through the later surface.
- Observed RSS stayed below the previous runtime reference at every matched stored checkpoint through `437`, then rose to `1485086720` bytes at `454`; that is still well below the old allocator-failure band.
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
- the steady-state scratch-slot `clone_from` reuse on terminal-clause loads inside remaining-one summary/materialization

## Baselines That Matter

### 1. Current Short Baseline
- Run: `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Honest retained shape:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
- The rerun was manually stopped after the stored `76` checkpoint because the decisive short and reopened surfaces had already been captured; one extra stored `77` checkpoint flushed while stopping and kept the same honest `40 groups / 147639 candidates` reopened surface.
- Because the stop was external during step `4`, the authoritative evidence for this short winner is `reports/steps/step-04-live.ndjson`, while `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`.

### 2. Current Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`
- It preserved the honest `39/40/41/42/43` retained-prefix story from the previous runtime reference, paid only a tiny early retained-surface tax, materially improved the later stored checkpoints from `76` onward, and then pushed the stored step-`4` wall farther from `437` to `454`.
- Stored decisive checkpoints:
  - `24`: `385100 / 382267`
  - `43`: `693470 / 689512`
  - `44`: `708134 / 704116`
  - `54`: `872387 / 867757`
  - `74`: `1200035 / 1194139`
  - `76`: `1239059 / 1233043`
  - `140`: `2349156 / 2339322`
  - `163`: `2722673 / 2711471`
  - `228`: `3837038 / 3821952`
  - `229`: `3856643 / 3841498`
  - `332`: `5704569 / 5683113`
  - `335`: `5758074 / 5736396`
  - `408`: `7014570 / 6988332`
  - `437`: `7548400 / 7520336`
  - `454`: `7860534 / 7831399`
- The later retained-prefix surface now reads:
  - `41 groups / 154842 candidates` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `454` read
- Aggregation remains the lead later bucket on the stored `41/42/43` surfaces, with connectivity second, exact `nu` third, and terminal clause-filter handoff still tiny.
- Observed RSS reached `1485086720` bytes at `454`.
- The run never reached step `5` on stored evidence; `reports/steps/step-05-live.ndjson` is absent.

### 3. Previous Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
- It preserved the honest `39/40/41` retained-prefix story from the previous runtime reference, re-earned the `42/157636` and `43/160430` reopens, materially improved every decisive stored matched checkpoint through the previous stored `408` wall, and then pushed the stored step-`4` wall farther to `437`.
- The current scratch-slot `clone_from` follow-up now keeps the same honest retained-prefix story, pays only a tiny early `24/43/44/54/74` tax, wins again from `76` onward, and moves the stored wall from `437` to `454`.

### 4. Earlier Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-connectivity-facts-v1`
- It preserved the honest `39/40/41` retained-prefix story from the old open-band-handoff follow-up, reopened to `42/157636` and `43/160430`, and pushed the prior stored wall to `408`, but the newer compact open-band aggregation reference and the newer scratch-slot `clone_from` reference both now stay ahead of it.

### 5. Comparison Full-Profile Baseline
- Run: `runs/codex-claim-release-full-kernel-aggregation-v1`
- It remains the broader comparison baseline only. The current runtime reference stays materially ahead of it.

### 6. Current Late-Surface Diagnostic
- Run: `runs/codex-claim-release-step4-kernel-late-profile-v1`
- It remains the earlier late diagnostic, but the newer full-profile scratch-slot `clone_from` follow-up is now the authoritative later-surface read.

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
- scratch-slot clause-load reuse first pass: `kernel-clause-load-v1`
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
- The scratch-slot `clone_from` slice engaged honestly on the real intended profile: it preserved the kept `39/40/41/42/43` surface story, stayed effectively flat on the early retained surface, improved the later stored checkpoints from `76` onward, and moved the later wall from `437` to `454`.
- The later wall is still aggregation first on stored evidence, with connectivity second, exact `nu` third, and terminal clause-filter handoff still tiny. The new nuance is that the kept slice buys the wall move mostly by cutting aggregation, exact `nu`, and handoff again while paying only a small connectivity regression on the reopened `42/43` surface.
- Observed RSS is lower than the previous runtime reference at matched stored checkpoints through `437`, then keeps climbing to `1485086720` bytes by `454`; that is still far below the old allocator-failure band, so the lane still reads as throughput-bound rather than allocator-bound on stored evidence.
- The accumulated lesson from the dropped retries is stronger again: the next honest runtime choice is still a later-surface runtime cut on this new winner rather than another plain rerun-only turn, another connectivity-first retry, or another metadata/clause-load-only replay.

## Best Current Inference
The current runtime reference is `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`.

That run is already good enough to answer the latest scratch-slot reuse question from stored evidence:
- it preserved the honest `39/40/41/42/43` retained-prefix story
- it paid only a tiny early retained-surface tax, stayed effectively flat at `74`, and then improved the later stored checkpoints from `76` onward
- it moved materially past the prior stored `437` wall to `454`
- it still did not reach step `5`
- it kept the visible later wall aggregation-first
- it kept observed RSS below the previous reference at matched checkpoints and well below the old allocator-failure band, even though RSS kept climbing on the new later surface

The next honest question is therefore another later-surface runtime cut on this new winner, with the small reopened-surface connectivity regression and the continued late RSS growth treated as explicit guardrails rather than ignored.

## Immediate Next Move
1. Keep `runs/codex-claim-release-step4-kernel-open-band-handoff-v1` as the short step-`4` baseline and `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1` as the current full-profile runtime reference.
2. Do not spend another turn on a plain intended-profile rerun with no code or new runtime question, and do not reopen another unchanged connectivity-first retry, metadata retry, or clause-load-only replay first.
3. Land one narrow later-surface runtime cut on the current winner. Target the post-`332` / post-`335` aggregation-side summary/rank/bookkeeping work that still leads the stored `43 / 160430` surface, while keeping the small reopened-surface connectivity tax and the later RSS growth in view as hard honesty checks.
4. After code changes land, rerun a release claim follow-up with a new run id that states the slice and carry it at least through the stored `332/335/437/454` region and ideally past the new `454` wall or into step `5` on stored evidence.
5. If code changes land before that rerun, rerun only:
   - `cargo test -p pen-search claim_`
   - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`
6. Branch to parity, breadth, compare, benchmark, or certification work only after a later full-profile rerun either reaches step `5` or moves materially past the current stored `454` wall.
