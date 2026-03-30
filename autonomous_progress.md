# Autonomous Claim Lane Progress
Last updated: 2026-03-30

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final gate.

## Current Status
- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current full-profile runtime reference is `runs/codex-claim-release-full-connectivity-facts-v1`.
- The previous full-profile runtime reference is `runs/codex-claim-release-full-open-band-handoff-followup-v1`.
- The latest measured slice is `runs/codex-claim-release-full-connectivity-facts-v1`.
- That new intended-profile follow-up preserved the honest retained-prefix story and then reopened it twice farther:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` from `335` through the stored `408` read
- It materially improved every decisive stored matched checkpoint versus the previous runtime reference:
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
- The later bucket order changed honestly after the connectivity-side cut landed:
  - `140`: aggregation `= 744641415 us`, connectivity `= 586579312 us`, exact `nu` `= 520449833 us`, terminal clause-filter handoff `= 15799216 us`
  - `163`: aggregation `= 857070798 us`, connectivity `= 685837174 us`, exact `nu` `= 604333257 us`, terminal clause-filter handoff `= 18798149 us`
  - `228`: aggregation `= 1200334910 us`, connectivity `= 968606567 us`, exact `nu` `= 869713548 us`, terminal clause-filter handoff `= 28170861 us`
  - `229`: aggregation `= 1210184906 us`, connectivity `= 973284301 us`, exact `nu` `= 872719297 us`, terminal clause-filter handoff `= 28348740 us`
  - `332`: aggregation `= 1799313029 us`, connectivity `= 1424660906 us`, exact `nu` `= 1323961772 us`, terminal clause-filter handoff `= 45034612 us`
  - `335`: aggregation `= 1814967597 us`, connectivity `= 1437778259 us`, exact `nu` `= 1337241409 us`, terminal clause-filter handoff `= 45526234 us`
  - `408`: aggregation `= 2185774562 us`, connectivity `= 1756872641 us`, exact `nu` `= 1633848702 us`, terminal clause-filter handoff `= 58019282 us`
- `terminal_summary_admissibility_checks = 0` and `terminal_summary_fallback_connectivity_checks = 0` through the stored `408` read.
- Observed RSS stayed below `964218880` bytes through the stored `408` checkpoint, still far below the old allocator-failure band.
- The lane is still compute-bound, and the later wall is now aggregation-first rather than connectivity-first.
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
- Run: `runs/codex-claim-release-full-connectivity-facts-v1`
- It preserved the honest `39/40/41` retained-prefix story from the previous runtime reference, then reopened to `42/157636` and `43/160430`, while materially improving all decisive stored matched checkpoints and pushing the stored step-`4` wall from `229` to `408`.
- Stored decisive checkpoints:
  - `24`: `385802 / 383018`
  - `43`: `698661 / 694743`
  - `44`: `713483 / 709504`
  - `54`: `880003 / 875413`
  - `74`: `1210334 / 1204512`
  - `76`: `1250591 / 1244645`
  - `140`: `2389805 / 2379752`
  - `163`: `2773573 / 2762030`
  - `228`: `3924018 / 3908221`
  - `229`: `3945205 / 3929337`
  - `332`: `5857640 / 5835050`
  - `335`: `5912135 / 5889319`
  - `408`: `7191639 / 7164178`
- The later retained-prefix surface now reads:
  - `41 groups / 154842 candidates` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `408` read
- Aggregation is now the lead later bucket on the stored `41/42/43` surfaces, with connectivity second, exact `nu` third, and terminal clause-filter handoff still tiny.
- Observed RSS reached `964218880` bytes at `408`.
- The run never reached step `5` on stored evidence; `reports/steps/step-05-live.ndjson` is absent.

### 3. Previous Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-open-band-handoff-followup-v1`
- It re-earned the kept short winner again on the real intended profile and pushed the prior stored wall to `229`, but the new connectivity-facts reference now materially beats it at every decisive stored checkpoint and pushes the wall much farther.
- Honest retained shape:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through the stored `229` read

### 4. Comparison Full-Profile Baseline
- Run: `runs/codex-claim-release-full-kernel-aggregation-v1`
- It remains the broader comparison baseline only. The current runtime reference stays materially ahead of it.

### 5. Current Late-Surface Diagnostic
- Run: `runs/codex-claim-release-step4-kernel-late-profile-v1`
- It remains the earlier late diagnostic, but the newer full-profile connectivity-facts follow-up is now the authoritative later-surface read.

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
- The connectivity-facts slice answered the old later question honestly: the post-`140` connectivity wall moved materially first on stored evidence.
- Once that happened, the later wall reopened twice more and aggregation became the new lead bucket on the real intended profile.
- The accumulated lesson from the dropped retries is now stronger again: the next honest runtime choice is a later-surface aggregation-side cut first on this winner rather than another connectivity-first retry, another early short micro-slice, or another plain rerun-only turn.

## Best Current Inference
The current runtime reference is no longer the old open-band-handoff follow-up. It is `runs/codex-claim-release-full-connectivity-facts-v1`.

That run is already good enough to answer the connectivity-side question from stored evidence:
- it preserved the honest `39/40/41` retained-prefix story
- it reopened farther to `42/157636` and `43/160430`
- it moved materially past the prior stored `229` wall to `408`
- it kept observed RSS far below the old allocator-failure band
- it changed the visible later wall from connectivity-first to aggregation-first

The next honest question is therefore aggregation-side runtime work on the current winner.

## Immediate Next Move
1. Keep `runs/codex-claim-release-step4-kernel-open-band-handoff-v1` as the short step-`4` baseline and `runs/codex-claim-release-full-connectivity-facts-v1` as the current full-profile runtime reference.
2. Do not spend another turn on a plain intended-profile rerun with no code or new runtime question, and do not reopen another unchanged connectivity-first retry first.
3. Land one narrow later-surface aggregation-side runtime cut on the current winner. Target the post-`140` aggregation-side summary/rank/bookkeeping work that now leads the stored `41/42/43` surface.
4. After code changes land, rerun a release claim follow-up with a new run id that states the slice and carry it at least through the stored `140/163/228` region and the current `42/43` reopen on stored evidence.
5. If code changes land before that rerun, rerun only:
   - `cargo test -p pen-search claim_`
   - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`
6. Branch to parity, breadth, compare, benchmark, or certification work only after a later full-profile rerun either reaches step `5` or moves materially past the current stored `408` wall.
