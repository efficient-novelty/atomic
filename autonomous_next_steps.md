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

## Active Baselines
- Current short baseline: `runs/codex-claim-release-step4-kernel-aggregation-v1`
- Current full-profile baseline: `runs/codex-claim-release-full-kernel-aggregation-v1`
- Current late-surface diagnostic: `runs/codex-claim-release-step4-kernel-late-profile-v1`
- Current informative non-keep rerun: `runs/codex-claim-release-step4-kernel-aggregation-tiecut-v1`

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
- The compact summary wall is now best read as a composite per-admitted kernel, not as one missing scalar gate. In the current code, every admitted candidate still pays for multiple exact operations inside `compute_terminal_prefix_completion_summary_from_candidates`: clause load into scratch state, admissibility diagnostics bookkeeping, exact bit-cost recovery, bound update, primary-rank math, and sometimes full `AcceptRank` construction.
- The full `AcceptRank` path is still expensive when it fires because it rebuilds full-telescope structural signals, max-var-ref context, and canonical-key context. The dropped contender-rank sidecar proved that this work is real, but the keep failures show it is not the only remaining wall.
- The lesson from the dropped threshold-only, bound-only, bookkeeping-only, competition-gate-only, exact-`nu`-gate-only, bit-cost-only, contender-rank-only, and strict-better-incumbent exact-rank-deferral slices is now that another one-constant slice is unlikely to be enough unless it removes several exact per-admitted rescans at once.

## Honest Read
- The reopened wall is no longer connectivity first.
- The compact-summary exact-rank deferral was real but only partial.
- The next winner should therefore not be another standalone scalar tweak. It should be one exact aggregation-side slice that removes several exact per-admitted rescans at once while keeping the retained-prefix shape honest.

## Goal
Land one exact aggregation-side slice that lowers the honest short read on both:
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

### 2. Change The Shape Of The Next Aggregation Cut
Do **not** treat the next winner as "one more scalar constant". Treat it as one exact terminal-clause metadata pack landed in one narrow slice.

Preferred primary move:
- extend `terminal_prefix_clause_candidates` / `FilteredTerminalClause` so the summary loop can reuse exact terminal-clause metadata that is currently rebuilt per admitted candidate
- keep the metadata exact and structural only; no lossy surrogate keys and no semantic reward signals
- target a pack that can be reused in both the compact summary path and the rare full-rank path, for example:
  - terminal-clause bit-cost delta
  - terminal-clause structural-signal delta
  - terminal-clause max-var-ref contribution
  - exact canonical-key suffix or rolling exact-key context needed to avoid rebuilding the whole telescope key on ordinary contenders
- pair that with one prefix-side exact aggregate computed once per signature, so the compact summary path can recover most ranking and bound inputs without rescanning the whole telescope

The intended effect is to remove several exact per-admitted rescans in one landed slice rather than chasing one small constant at a time.

### 3. Make Full `AcceptRank` Truly Last-Tie Only
Inside the same slice:
- keep the existing primary-rank short-circuit
- compare contenders first on the exact numeric fields that can be assembled from prefix aggregates plus terminal-clause metadata
- only materialize the exact canonical key when those earlier exact tie-break fields still tie

This keeps exact tie-break truth while moving the most allocation-heavy work to the rarest path.

### 4. Add Fine-Grained Aggregation Telemetry Inside The Same Patch
Do not run a separate diagnostic-only slice first. Instead, add sub-bucket timing to the same candidate patch so one honest rerun answers where the remaining aggregation time actually sits.

At minimum, split aggregation into:
- clause load / scratch update
- admissibility-diagnostics bookkeeping
- exact bit-cost recovery
- bound update
- primary-rank math
- full `AcceptRank` construction
- canonical-key finalization

This is only to read the stored artifact from the same runtime slice; it is not a separate branch of work.

### 5. Re-Earn The Short Runtime Read
Run a release claim rerun derived from `configs/desktop_claim_shadow_1h.toml` with:
- `--until-step 4`
- the metadata-pack binary above
- live checkpoint persistence left on
- a new run id that states the patch, for example:
  - `runs/codex-claim-release-step4-kernel-clause-metadata-v2`
  - `runs/codex-claim-release-step4-kernel-rank-metadata-v1`
  - `runs/codex-claim-release-step4-kernel-canonical-tail-v1`

Let the run go far enough to capture at least:
- matched `24/43/44/54`
- the reopened `40 groups / 147639 candidates` surface at `74/76`
- `140` only if it still arrives cheaply

### 6. Read The Stored Artifacts
Open at least:
- `reports/steps/step-04-live.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:
- did the early `24/43/44/54` surface move back under the kept short baseline honestly on both elapsed and `terminal_summary_build_*`?
- did clause filtering stay near the kept late diagnostic instead of reopening the old clause-filter wall?
- did the reopened `74/76` wall stay aggregation-first, or did it move again?
- which aggregation sub-buckets dominated at `74/76` and on the `54 -> 76` increment?
- did full `AcceptRank` construction and canonical-key finalization become the rare tail rather than the common admitted-candidate path?
- did `74/76` move past the kept full-profile aggregation baseline honestly?

### 7. Re-Earn Only The Validation Needed
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
- a composite aggregation-side metadata pack earns keep on both the early and reopened short surfaces
- the step-`4` wall moves away from aggregation first
- the new sub-bucket telemetry shows a different exact blocker than expected
- a later full-profile rerun reaches a new blocker honestly
- runtime work is no longer the next honest move
