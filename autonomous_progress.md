# Autonomous Claim Lane Progress
Last updated: 2026-03-31

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final gate.

## Current Status
- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current full-profile runtime reference is `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`.
- The previous full-profile runtime reference is `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`.
- The earlier full-profile runtime reference is `runs/codex-claim-release-full-aggregation-open-band-compact-v1`.
- The earlier full-profile runtime reference is `runs/codex-claim-release-full-connectivity-facts-v1`.
- The latest measured slice is `runs/codex-claim-release-full-aggregation-open-band-accumulator-v1`, and it was dropped after regressing the current runtime reference on stored evidence.
- That later-surface follow-up preserved the honest retained-prefix story only through the current `41` surface:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through the stored `285` read
  It never re-earned the current reference's `42 / 157636` or `43 / 160430` reopens before the stored stop.
- It regressed every decisive matched checkpoint against the current runtime reference:
  - `24`: `336071 / 333226` instead of `327917 / 325321`
  - `43`: `606634 / 602665` instead of `591083 / 587479`
  - `44`: `619470 / 615435` instead of `603570 / 599913`
  - `54`: `762881 / 758237` instead of `743997 / 739797`
  - `74`: `1049197 / 1043309` instead of `1023572 / 1018283`
  - `76`: `1084568 / 1078557` instead of `1057864 / 1052464`
  - `140`: `2063608 / 2053672` instead of `2014043 / 2005230`
  - `163`: `2393159 / 2381788` instead of `2334208 / 2324213`
  - `228`: `3374290 / 3358809` instead of `3287422 / 3274060`
  - `229`: `3392631 / 3377087` instead of `3304737 / 3291324`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The bucket order stayed aggregation first honestly on the stored later surface:
  - `140`: aggregation `= 775992589 us`, connectivity `= 575235886 us`, exact `nu` `= 525904436 us`, terminal clause-filter handoff `= 15264839 us`
  - `228`: aggregation `= 1245498762 us`, connectivity `= 947381690 us`, exact `nu` `= 875269842 us`, terminal clause-filter handoff `= 27189213 us`
  - `285`: aggregation `= 1597950348 us`, connectivity `= 1192428892 us`, exact `nu` `= 1115322732 us`, terminal clause-filter handoff `= 36292013 us`
- `terminal_summary_admissibility_checks = 0` and `terminal_summary_fallback_connectivity_checks = 0` through the stored `285` read.
- The first stored plateau activation is still `24`, and `terminal_summary_plateau_activations = 231996` stayed flat through the stored stop.
- Observed RSS stayed well below the old allocator-failure band:
  - `228`: `828514304` bytes
  - `229`: `831315968` bytes
  - `279`: `973017088` bytes
  - `285`: `990433280` bytes
- Because the rerun was manually stopped after the later-surface regression was already clear, `reports/latest.txt` still reflects completed step `3`, `run.json` still says `status = "running"`, and `reports/steps/step-05-live.ndjson` is absent; the authoritative evidence for this run lives in `reports/steps/step-04-live.ndjson`.

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
- the boundary-timestamp timing pass on the compact claim open-band no-evaluations summary kernel

## Baselines That Matter

### 1. Current Short Baseline
- Run: `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Honest retained shape:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
- The rerun was manually stopped after the stored `76` checkpoint because the decisive short and reopened surfaces had already been captured; one extra stored `77` checkpoint flushed while stopping and kept the same honest `40 groups / 147639 candidates` reopened surface.
- Because the stop was external during step `4`, the authoritative evidence for this short winner is `reports/steps/step-04-live.ndjson`, while `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`.

### 2. Current Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`
- It preserved the honest `39/40/41/42/43` retained-prefix story from the previous runtime reference, materially improved every decisive matched checkpoint through the prior stored `454` wall, and then pushed the stored step-`4` wall farther to `484`.
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
- Aggregation remains the lead later bucket on the stored `41/42/43` surfaces, with connectivity second, exact `nu` third, and terminal clause-filter handoff still tiny.
- Inference from matched checkpoints:
  this new winner mainly collapsed the previously unattributed summary-build remainder rather than reducing measured aggregation itself.
- Observed RSS reached `1581830144` bytes at `484`.
- The run never reached step `5` on stored evidence; `reports/steps/step-05-live.ndjson` is absent.

### 3. Latest Failed Later-Surface Follow-Up
- Run: `runs/codex-claim-release-full-aggregation-open-band-accumulator-v1`
- Hypothesis:
  keep the current stage-timing winner intact, but cut one more per-candidate aggregation constant inside the compact claim open-band no-evaluations summary kernel by specializing the local summary accumulator instead of repeatedly mutating the generic summary fields and `Option` payloads in the hottest loop.
- Outcome:
  - it preserved the honest `39/40/41` retained-prefix story through the stored `285` read
  - it never re-earned the current runtime reference's `42 / 157636` or `43 / 160430` surfaces
  - it regressed every decisive matched checkpoint versus the current runtime reference from `24` through `229`
  - it stayed memory-safe and never hinted at the old allocator-failure story returning
  - the rerun was manually stopped after the regression was already clear on stored evidence
- Comparison versus the current runtime reference:
  - `24`: `336071 / 333226` instead of `327917 / 325321`
  - `43`: `606634 / 602665` instead of `591083 / 587479`
  - `44`: `619470 / 615435` instead of `603570 / 599913`
  - `54`: `762881 / 758237` instead of `743997 / 739797`
  - `74`: `1049197 / 1043309` instead of `1023572 / 1018283`
  - `76`: `1084568 / 1078557` instead of `1057864 / 1052464`
  - `140`: `2063608 / 2053672` instead of `2014043 / 2005230`
  - `163`: `2393159 / 2381788` instead of `2334208 / 2324213`
  - `228`: `3374290 / 3358809` instead of `3287422 / 3274060`
  - `229`: `3392631 / 3377087` instead of `3304737 / 3291324`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Stored later-surface read before stopping:
  - `279`: `41 groups / 154842 candidates`, `4204858 / 4186088`, RSS `= 973017088` bytes
  - `285`: `41 groups / 154842 candidates`, `4291394 / 4272236`, RSS `= 990433280` bytes
- Honest read:
  the accumulator specialization engaged and stayed honest, but it did not move the measured aggregation wall first. The regression stayed visible from the early kept surface through the later stored `41` surface, so this local summary field/`Option` cleanup is now dropped as a standalone next move.

### 4. Previous Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-aggregation-open-band-scratch-clonefrom-v1`
- It preserved the honest `39/40/41/42/43` retained-prefix story from the previous runtime reference, paid only a tiny early retained-surface tax, materially improved the later stored checkpoints from `76` onward, and then pushed the stored step-`4` wall farther from `437` to `454`.
- The current stage-timing follow-up now keeps the same honest retained-prefix story, wins materially at every decisive matched checkpoint, and moves the stored wall again from `454` to `484`.

### 5. Earlier Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
- It preserved the honest `39/40/41` retained-prefix story from the previous runtime reference, re-earned the `42/157636` and `43/160430` reopens, materially improved every decisive stored matched checkpoint through the previous stored `408` wall, and then pushed the stored step-`4` wall farther to `437`.

### 6. Earlier Full-Profile Runtime Reference
- Run: `runs/codex-claim-release-full-connectivity-facts-v1`
- It preserved the honest `39/40/41` retained-prefix story from the old open-band-handoff follow-up, reopened to `42/157636` and `43/160430`, and pushed the prior stored wall to `408`, but the newer compact, scratch-slot, and stage-timing references all now stay ahead of it.

### 7. Comparison Full-Profile Baseline
- Run: `runs/codex-claim-release-full-kernel-aggregation-v1`
- It remains the broader comparison baseline only. The current runtime reference stays materially ahead of it.

### 8. Current Late-Surface Diagnostic
- Run: `runs/codex-claim-release-step4-kernel-late-profile-v1`
- It remains the earlier late diagnostic, but the newer full-profile stage-timing follow-up is now the authoritative later-surface read.

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
- open-band compact-summary accumulator follow-up: `full-aggregation-open-band-accumulator-v1`
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
- The stage-timing slice engaged honestly on the real intended profile: it preserved the kept `39/40/41/42/43` surface story, materially improved every decisive matched checkpoint through the prior stored `454` wall, and moved the later wall again to `484`.
- The newer accumulator follow-up answered the next local bookkeeping question on stored evidence: it preserved the honest `39/40/41` surfaces and stayed memory-safe, but it regressed every decisive matched checkpoint against the stage-timing winner and never re-earned the `42/43` reopens before the stored `285` stop.
- The later wall is still aggregation first on stored evidence, with connectivity second, exact `nu` third, and terminal clause-filter handoff still tiny.
- The new nuance is that the current winner's main gain comes from a much smaller previously unattributed summary-build remainder, while measured aggregation itself remains the lead bucket and is slightly higher at matched later checkpoints. That is an inference from the matched checkpoint totals and bucket sums rather than a separately emitted telemetry field.
- Observed RSS is slightly higher than the previous runtime reference on the matched later surface and then continues rising to `1581830144` bytes by `484`; that is still far below the old allocator-failure band, so the lane still reads as throughput-bound rather than allocator-bound on stored evidence.
- The accumulated lesson from the dropped retries is stronger again: the next honest runtime choice is still a later-surface runtime cut on this new winner rather than another plain rerun-only turn, another connectivity-first retry, another accumulator-only replay, or another metadata/clause-load-only replay.

## Best Current Inference
The current runtime reference is `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`.

That run is already good enough to answer the latest late-surface timing-overhead question from stored evidence:
- it preserved the honest `39/40/41/42/43` retained-prefix story
- it improved every decisive matched checkpoint through the prior stored `454` wall
- it moved materially past the prior stored `454` wall to `484`
- it still did not reach step `5`
- it kept the visible later wall aggregation-first
- it appears to have removed most of the previously unattributed summary-build remainder while keeping the retained-prefix story intact
- it kept observed RSS well below the old allocator-failure band even though late RSS drift is now slightly worse than the previous runtime reference

The newer accumulator follow-up then answered the next local bookkeeping question from stored evidence too:
- it preserved the honest `39/40/41` retained-prefix story through the stored `285` read
- it never re-earned the `42 / 157636` and `43 / 160430` reopens
- it regressed every decisive matched checkpoint against the stage-timing winner from `24` through `229`
- it still stayed well below the old allocator-failure band

The next honest question is therefore still another later-surface runtime cut on this winner, now targeting measured aggregation-side rank/bound/summary work beyond the dropped local accumulator cleanup, with the small late RSS increase treated as an explicit guardrail rather than ignored.

## Immediate Next Move
1. Keep `runs/codex-claim-release-step4-kernel-open-band-handoff-v1` as the short step-`4` baseline and `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1` as the current full-profile runtime reference.
2. Do not spend another turn on a plain intended-profile rerun with no code or new runtime question, and do not reopen another unchanged connectivity-first retry, accumulator-only replay, metadata retry, clause-load-only replay, or timing-only replay first.
3. Land one narrow later-surface runtime cut on the current winner. Target the post-`332` / post-`335` measured aggregation-side summary/rank/bound work that still leads the stored `43 / 160430` surface, while keeping the slight late-surface RSS increase in view as a hard honesty check.
4. After code changes land, rerun a release claim follow-up with a new run id that states the slice and carry it at least through the stored `332/335/408/437/454` region and ideally past the new `484` wall or into step `5` on stored evidence.
5. If code changes land before that rerun, rerun only:
   - `cargo test -p pen-search claim_`
   - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`
6. Branch to parity, breadth, compare, benchmark, or certification work only after a later full-profile rerun either reaches step `5` or moves materially past the current stored `484` wall.
