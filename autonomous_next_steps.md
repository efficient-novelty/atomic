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
- the higher-fidelity late-surface timing accumulation

Assume the following were already measured and should stay dropped as standalone next moves:
- ordering and reuse variants
- expr-keyed or clause-side connectivity cache variants
- terminal-candidate prep or remap variants
- telemetry-only filter profiling as a separate slice
- the exact cross-multiplied primary-rank bookkeeping rewrite in `runs/codex-claim-release-step4-kernel-rank-bookkeeping-v1`
- the constant-`kappa` bound-merge rewrite in `runs/codex-claim-release-step4-kernel-bound-merge-v1`
- the lazy incumbent-tie `AcceptRank` rewrite in `runs/codex-claim-release-step4-kernel-lazy-acceptrank-v1`
- the local summary batching rewrite in `runs/codex-claim-release-step4-kernel-summary-batching-v1`
- the shared reason-key summary bookkeeping rewrite in `runs/codex-claim-release-step4-kernel-summary-bookkeeping-v1`
- the prefix-wide competition-gate hoist in `runs/codex-claim-release-step4-kernel-competition-hoist-v1`
- the exact-`nu` high-water gate in `runs/codex-claim-release-step4-kernel-nu-highwater-v1`
- the fixed bar-clear summary-threshold rewrite in `runs/codex-claim-release-step4-kernel-summary-threshold-v1`
- the summary-invariants accept-rank prefix-context rewrite in `runs/codex-claim-release-step4-kernel-summary-invariants-v1`
- the summary-rank-context exact tie-break rewrite in `runs/codex-claim-release-step4-kernel-rank-context-v1`
- the terminal-rank sidecar exact contender-rank rewrite in `runs/codex-claim-release-step4-kernel-terminal-rank-sidecar-v1`
- the primary-rank context exact-threshold rewrite in `runs/codex-claim-release-step4-kernel-primary-context-v1`
- the summary-constant bit-cost hoist in `runs/codex-claim-release-step4-kernel-summary-constant-v1`
- the catalog-backed clause bit-cost sidecar in `runs/codex-claim-release-step4-kernel-catalog-constant-v1`
- the eager clause-filter-wide metadata pack in `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
- the lazy admitted-only metadata retry in `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`
- the parent-summary connectivity lookup reuse in `runs/codex-claim-release-step4-kernel-reopened-connectivity-v1`
- the compact-summary strict-better-incumbent exact-rank deferral in `runs/codex-claim-release-step4-kernel-aggregation-tiecut-v1`
- the exact rank-metadata pack with last-tie canonical-key finalization in `runs/codex-claim-release-step4-kernel-rank-metadata-v1`

## Active Baselines
- Current short baseline: `runs/codex-claim-release-step4-kernel-aggregation-v1`
- Current full-profile baseline: `runs/codex-claim-release-full-kernel-aggregation-v1`
- Current late-surface diagnostic: `runs/codex-claim-release-step4-kernel-late-profile-v1`
- Current informative non-keep rerun: `runs/codex-claim-release-step4-kernel-bound-bookkeeping-v1`

## Revised Working Diagnosis
- The intended profile is still blocked in step `4` by remaining-one compact-summary throughput.
- The honest retained-prefix story is now stable across the kept and dropped reruns:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` at `140` on the kept full-profile baseline
- The reopened connectivity reuse rerun showed that connectivity can fall materially while clause filtering stays cheap, but it still failed keep on `terminal_summary_build_*`.
- The newer compact-summary exact-rank deferral rerun showed that late aggregation really does move:
  - at `76`, aggregation `= 453514026 us` versus `469431036 us` on `runs/codex-claim-release-step4-kernel-late-profile-v1`
  - clause filtering also stayed near the kept late diagnostic: `346694360 us` versus `352203534 us`
  - but it still failed keep because the matched `24/43/44/54` surface regressed on both elapsed and `terminal_summary_build_*`, and the reopened `74/76` read still trailed the kept full-profile baseline
- The newer exact rank-metadata rerun then answered the open ranking question on stored evidence. At the matched `24` checkpoint it preserved the honest `39 groups / 144845 candidates` plateau, but it still failed keep badly:
  - `24`: `1232202 / 872432` instead of the kept short `549630 / 492524`
  - full `AcceptRank` construction was only `543075 us`
  - canonical-key finalization was only `5 us`
  - the broad hot costs at that same checkpoint were still clause load `= 61477661 us`, admissibility bookkeeping `= 22145864 us`, bit-cost recovery `= 11259134 us`, bound update `= 11437024 us`, connectivity `= 137191827 us`, clause filtering `= 135071766 us`, and compact materialize `= 319890 ms`
- The newer direct bound/bookkeeping rerun then answered the next local structural question on stored evidence. At the matched `24` checkpoint it also preserved the honest `39 groups / 144845 candidates` plateau, but it still failed keep:
  - `24`: `549708 / 544700` instead of the kept short `549630 / 492524`
  - the broad hot costs at that same checkpoint were aggregation `= 132199915 us`, connectivity `= 125043669 us`, clause filtering `= 105522189 us`, exact `nu` `= 80087659 us`, and terminal materialize `= 336 ms`
  - the rerun was manually stopped after the decisive `24` read; one extra stored `25` checkpoint flushed while stopping and kept the same `39 groups / 144845 candidates` plateau
- The compact summary wall is therefore still best read as a composite per-admitted kernel, but the stored answer changed: the next blocker is not canonical-tail work first.

## Honest Read
- The reopened wall is no longer connectivity first.
- The compact-summary exact-rank deferral was real but only partial.
- The last-tie canonical-key path is already the rare tail on stored evidence.
- One local bookkeeping/bound cleanup was also not enough to re-earn the early short surface.
- The next winner should therefore not be another canonical-tail, clause-load-only, or bookkeeping/bound-only retry. It should be one kept-binary slice that removes a broader compound clause-load, bookkeeping, bound, connectivity, or clause-filter cost while keeping the retained-prefix shape honest.

## Goal
Land one kept-binary slice that lowers the honest short read on both:
- the matched `24/43/44/54` plateau checkpoints
- the reopened `40/147639` surface at `74/76`

without weakening the retained-prefix shape.

## Do This Next

### 1. Keep The Right Reference Surface
Keep the code behind:
- `runs/codex-claim-release-step4-kernel-aggregation-v1`
- `runs/codex-claim-release-full-kernel-aggregation-v1`
- `runs/codex-claim-release-step4-kernel-late-profile-v1`

Do not reopen first:
- another connectivity-side rewrite
- another clause-filter-side rewrite
- another exact-`nu` cleanup first
- another ordering, reuse, cache, or post-plateau variant
- another retry of the dropped single-axis aggregation slices listed above
- another diagnostic-only slice before a new runtime hypothesis is measured

### 2. Change The Shape Of The Next Runtime Cut
Do **not** treat the next winner as another canonical-tail or rank-metadata retry. The last stored rerun already proved that full `AcceptRank` construction and canonical-key finalization are a rare tail cost.

Preferred primary move:
- keep the winning baseline code
- attack one broader compound structural hot path first:
  - clause load / scratch update together with bookkeeping / bit-cost / bound work
  - or a different aggregation-side fusion that removes multiple per-admitted structural costs at once
- do **not** spend the next slice on:
  - another scratch-slot clause-load-only replay
  - another bookkeeping/bound-only cleanup
  - compact materialization first on the early surface
- if you reuse any exact metadata again, keep it off the compact materialization path unless the stored early surface proves that extra work pays for itself
- keep the metadata exact and structural only; no lossy surrogate keys and no semantic reward signals

The intended effect is to remove broad per-admitted work without paying another early-surface tax just to reconfirm that canonical-tail cost is small.

### 3. Keep The New Telemetry As The Evidence Map
Use `runs/codex-claim-release-step4-kernel-rank-metadata-v1` as the current sub-bucket map and `runs/codex-claim-release-step4-kernel-bound-bookkeeping-v1` as the latest broad checkpoint read.

At the matched `24` checkpoint:
- clause load / scratch update `= 61477661 us`
- admissibility bookkeeping `= 22145864 us`
- exact bit-cost recovery `= 11259134 us`
- bound update `= 11437024 us`
- primary-rank math `= 14524895 us`
- full `AcceptRank` construction `= 543075 us`
- canonical-key finalization `= 5 us`
- latest broad bucket order on the newer structural rerun:
  - aggregation `= 132199915 us`
  - connectivity `= 125043669 us`
  - clause filtering `= 105522189 us`
  - exact `nu` `= 80087659 us`
  - terminal materialize `= 336 ms`

Read that as: the next honest move is not to chase the last tie-break key first, and not to spend another turn on a bookkeeping/bound-only cleanup that still leaves aggregation, connectivity, and clause filtering materially hot.

### 4. Re-Earn The Short Runtime Read
Run a release claim rerun derived from `configs/desktop_claim_shadow_1h.toml` with:
- `--until-step 4`
- one new kept-binary slice above
- live checkpoint persistence left on
- a new run id that states the patch, for example:
  - `runs/codex-claim-release-step4-kernel-aggregation-fusion-v1`
  - `runs/codex-claim-release-step4-kernel-admitted-kernel-v1`
  - `runs/codex-claim-release-step4-kernel-clause-load-bound-fusion-v1`

Let the run go far enough to capture at least:
- matched `24`
- `43/44/54` only if the `24` read still looks plausible
- the reopened `40 groups / 147639 candidates` surface at `74/76` only if the early surface earns that extra runtime

### 5. Read The Stored Artifacts
Open at least:
- `reports/steps/step-04-live.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:
- did the early `24` surface move back under the kept short baseline honestly on both elapsed and `terminal_summary_build_*`?
- if not, which broad buckets dominated there: clause load, bookkeeping, bit-cost/bound work, connectivity, clause filtering, or compact materialization?
- did clause filtering stay near the kept late diagnostic instead of reopening the old clause-filter wall?
- if `24` passes honestly, did the later `43/44/54/74/76` surfaces also hold?

### 6. Re-Earn Only The Validation Needed
If the slice stays claim-only and step-`4` runtime-only, rerun only:
- `cargo test -p pen-search claim_`
- `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

If the new short slice earns keep, then branch back to one new full-profile rerun before parity, breadth, compare, benchmark, or certification work.

## Keep Or Branch Decision
After the next short rerun:
- stay on runtime work if the early or reopened surfaces still regress
- branch back to a new full-profile rerun only if the short rerun beats the kept short baseline honestly and materially improves the reopened `74/76` read against the kept full-profile baseline
- branch to parity, breadth, compare, benchmark, and certification work only after a later full-profile run honestly moves past the step-`4` wall

## Stop Condition For This Note
Rewrite this file as soon as one new stored rerun shows one of these is true:
- a broader kept-binary structural slice earns keep on the early short surface and still looks viable later
- the step-`4` wall moves away from aggregation first
- the new sub-bucket telemetry shows a different exact blocker than expected
- a later full-profile rerun reaches a new blocker honestly
- runtime work is no longer the next honest move
