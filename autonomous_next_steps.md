# Autonomous Claim Lane: Next Operational Slice
Last updated: 2026-03-29

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
- the eager terminal-clause metadata pack in `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
- the lazy admitted-only metadata retry in `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`
- the parent-summary connectivity lookup reuse in `runs/codex-claim-release-step4-kernel-reopened-connectivity-v1`

## Active Baselines
- Current short baseline: `runs/codex-claim-release-step4-kernel-aggregation-v1`
- Current full-profile baseline: `runs/codex-claim-release-full-kernel-aggregation-v1`
- Current late-surface diagnostic: `runs/codex-claim-release-step4-kernel-late-profile-v1`
- Current informative non-keep rerun: `runs/codex-claim-release-step4-kernel-reopened-connectivity-v1`

## Revised Working Diagnosis
- The reopened connectivity reuse rerun preserved the honest retained-prefix shape:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
- It also re-earned cheap clause filtering relative to the kept late diagnostic:
  - `76` clause filtering `= 356760236 us` versus `352203534 us` on `runs/codex-claim-release-step4-kernel-late-profile-v1`
  - this stayed near the dropped admitted-only metadata retry `= 355695170 us`
- It materially improved elapsed wall clock on both the matched early short surface and the reopened `74/76` surface:
  - `24`: `523076 / 518007` versus kept short `549630 / 492524`
  - `43`: `948473 / 942259` versus kept short `990480 / 892772`
  - `44`: `969247 / 962975` versus kept short `1012067 / 912271`
  - `54`: `1197195 / 1190324` versus kept short `1247600 / 1126754`
  - `74`: `1649766 / 1641724` versus kept full-profile `1743244 / 1579138`
  - `76`: `1702496 / 1694340` versus kept full-profile `1797441 / 1628768`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- But it still failed keep because `terminal_summary_build_*` regressed on the same surfaces:
  - about `5.2-5.6%` worse than the kept short baseline at `24/43/44/54`
  - about `4.0%` worse than the kept full-profile baseline at `74/76`
- The visible reopened wall moved away from connectivity first. At `76` on the new rerun:
  - aggregation `= 463408834 us`
  - clause filtering `= 356760236 us`
  - connectivity `= 282490143 us`
  - exact `nu` `= 265598332 us`
- Incremental `54 -> 76` now also leads with aggregation rather than connectivity:
  - aggregation `+140197060 us`
  - clause filtering `+108329001 us`
  - connectivity `+80429105 us`
  - exact `nu` `+79520130 us`

## Honest Read
- Reusing one parent legality summary across each remaining-one clause scan did cut reopened-surface connectivity cost materially and improved elapsed wall clock.
- It did **not** earn keep because the stored `terminal_summary_build_*` surface still regressed on both the matched early checkpoints and the reopened `74/76` surface.
- The new lesson is therefore different from the prior note:
  - metadata is still not the next move
  - connectivity is no longer the first visible reopened wall
  - the reopened step-`4` wall is now aggregation first, with clause filtering second and connectivity third

## Goal
Land one narrower reopened-surface runtime slice that:
- keeps the kept short baseline code behind `runs/codex-claim-release-step4-kernel-aggregation-v1`
- keeps the honest `39/144845` and reopened `40/147639` shapes
- keeps clause filtering near `runs/codex-claim-release-step4-kernel-late-profile-v1`
- attacks the now-honest aggregation-first reopened wall without reopening the dropped metadata or connectivity rewrites

## Do This Next

### 1. Keep The Right Reference Surface
Keep the code behind:
- `runs/codex-claim-release-step4-kernel-aggregation-v1`
- `runs/codex-claim-release-full-kernel-aggregation-v1`
- `runs/codex-claim-release-step4-kernel-late-profile-v1`

Do not keep in code:
- the eager clause-filter-wide metadata pack from `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
- the lazy admitted-only metadata path from `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`
- the parent-summary connectivity lookup reuse from `runs/codex-claim-release-step4-kernel-reopened-connectivity-v1`

### 2. Pivot From Connectivity To The Reopened Aggregation Wall
The next retry should not be another metadata slice and should not be another connectivity-first slice.

Instead:
- keep terminal clause filtering cheap
- keep the current exact tie-break truth surface as-is
- target one narrow exact runtime cut on the reopened aggregation wall
- prefer work that engages only after the retained-prefix plateau or reopened `74/76` surface, not a broad early-frontier rewrite

The next slice may still touch remaining-one runtime code, but it should avoid:
- rebuilding clause-local exact metadata as a primary move
- reopening eager or lazy canonical-key work unchanged
- reopening the dropped connectivity lookup reuse unchanged
- reopening the earlier dropped rank-context or bookkeeping rewrites unchanged

### 3. Keep The Timing Surface Honest
Keep these stored reads available on the next retry:
- clause filtering separate from total summary build
- aggregation separate from connectivity
- exact `nu` separate from the rest

If one new aggregation retry lands, keep any extra split narrowly scoped and only if it stays cheap enough to measure:
- aggregation bookkeeping versus accept-rank/canonical-key finalization
- late reopened prefixes versus the earlier `39/144845` plateau

### 4. Re-Earn The Short Runtime Read
Run a new release claim rerun derived from `configs/desktop_claim_shadow_1h.toml` with:
- `--until-step 4`
- the aggregation-side binary above
- live checkpoint persistence left on
- a new run id that states the patch, for example:
  - `runs/codex-claim-release-step4-kernel-aggregation-late-v1`
  - `runs/codex-claim-release-step4-kernel-reopened-aggregation-v1`

Let the run go far enough to capture at least:
- matched `24/43/44/54`
- the reopened `40 groups / 147639 candidates` surface at `74/76`
- `140` only if it still arrives cheaply

### 5. Read The Stored Artifacts
Open at least:
- `reports/steps/step-04-live.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:
- did the early `24/43/44/54` surface move back under the kept short baseline honestly on both elapsed and `terminal_summary_build_*`?
- did clause filtering stay near the kept late diagnostic instead of reopening the old clause-filter wall?
- did the reopened `74/76` wall stay aggregation-first, or did it move again?
- did `74/76` move past the kept full-profile aggregation baseline honestly?
- is aggregation still first on the `54 -> 76` increment, or did another bucket retake the lead?

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
- a reopened-surface aggregation cut earns keep on both the early and reopened short surfaces
- the step-`4` wall moves away from aggregation first
- a later full-profile rerun reaches a new blocker honestly
- runtime work is no longer the next honest move
