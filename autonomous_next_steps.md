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

## Active Baselines
- Current short baseline: `runs/codex-claim-release-step4-kernel-aggregation-v1`
- Current full-profile baseline: `runs/codex-claim-release-full-kernel-aggregation-v1`
- Current late-surface diagnostic: `runs/codex-claim-release-step4-kernel-late-profile-v1`

## Revised Working Diagnosis
- The eager exact terminal-clause metadata pack engaged and preserved the honest retained-prefix shape:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
- It failed keep badly on runtime:
  - `24`: `1385075 / 1067196` instead of `549630 / 492524`
  - `43`: `2474918 / 1930943` instead of `990480 / 892772`
  - `44`: `2534606 / 1975294` instead of `1012067 / 912271`
  - `54`: `3077025 / 2439389` instead of `1247600 / 1126754`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The reopened surface also regressed hard against both kept references:
  - `74`: `4181583 / 3351564019` versus kept full-profile `1743244 / 1579138`
  - `76`: `4322812 / 3445171148` versus kept full-profile `1797441 / 1628768`
  - `76` also trailed the late diagnostic `1848102 / 1839910636`
- The finer telemetry answers why. On the new slice at `76`:
  - clause filtering `= 2178547522 us`
  - aggregation `= 456894681 us`
  - connectivity `= 412251293 us`
  - exact `nu` `= 269107583 us`
- Inside the new aggregation split, the largest incremental `54 -> 76` sub-bucket is still clause load / scratch update:
  - clause load `+58123032 us`
  - admissibility bookkeeping `+21360371 us`
  - primary-rank math `+13290870 us`
  - bound update `+10799021 us`
  - bit-cost recovery `+10749565 us`
  - full `AcceptRank` construction `+513476 us`
  - canonical-key finalization `+22 us`

## Honest Read
- The numeric-first exact rank idea is real once metadata already exists: full `AcceptRank` assembly and canonical-key finalization became tiny.
- The failure is **where** the metadata was built. Precomputing structural and canonical clause metadata inside `terminal_prefix_clause_candidates` moved the wall to clause filtering first and more than erased the later rank-side savings.
- The next retry should therefore keep the current winning code in place and only revisit metadata if it is built **after** connectivity and admitted-candidate checks, not during terminal clause filtering.

## Goal
Land one narrower exact slice that:
- keeps the kept short baseline code behind `runs/codex-claim-release-step4-kernel-aggregation-v1`
- keeps the honest `39/144845` and reopened `40/147639` shapes
- avoids the clause-filter explosion from `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
- re-tests whether lazy admitted-only metadata can still cut the remaining aggregation work

## Do This Next

### 1. Keep The Right Reference Surface
Keep the code behind:
- `runs/codex-claim-release-step4-kernel-aggregation-v1`
- `runs/codex-claim-release-full-kernel-aggregation-v1`
- `runs/codex-claim-release-step4-kernel-late-profile-v1`

Do not keep in code:
- the eager clause-filter-wide metadata pack from `runs/codex-claim-release-step4-kernel-clause-metadata-v1`

### 2. Retry Metadata Only Behind Admitted Candidates
If metadata is retried, move it behind the hot-path gates:
- do **not** build structural-signal or canonical-key suffix metadata in `terminal_prefix_clause_candidates`
- keep terminal clause filtering cheap again
- build any exact clause metadata only after:
  - connectivity passes
  - admissibility admits the candidate
  - and preferably only once the candidate can still contend after the incumbent-primary short-circuit

The next slice can still keep:
- one prefix-side exact rank context computed once per signature
- numeric-field comparison before canonical-key finalization

But it must delay clause-local metadata until the candidate is already in the admitted summary kernel.

### 3. Keep The Fine-Grained Timing
Keep the new diagnostic lesson, not the eager runtime shape:
- clause filtering must stay separately readable from summary build
- if a lazy metadata retry lands, keep aggregation sub-buckets so one honest rerun shows whether:
  - clause filtering falls back toward the kept references
  - and the admitted-only metadata still lowers aggregation

### 4. Re-Earn The Short Runtime Read
Run a new release claim rerun derived from `configs/desktop_claim_shadow_1h.toml` with:
- `--until-step 4`
- the lazy admitted-only metadata binary above
- live checkpoint persistence left on
- a new run id that states the patch, for example:
  - `runs/codex-claim-release-step4-kernel-lazy-metadata-v1`
  - `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`

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
- did clause filtering fall back toward the kept short or late-diagnostic surfaces?
- did the lazy metadata retry lower elapsed and total `terminal_summary_build_*` at `24/43/44/54` without losing `39/144845`?
- did the reopened `74/76` surface move back toward the kept full-profile aggregation baseline?
- which aggregation sub-bucket is now first at `54 -> 76`?
- is aggregation the honest blocker again, or has the wall moved somewhere else?

### 6. Re-Earn Only The Validation Needed
If the slice stays claim-only and step-`4` runtime-only, rerun only:
- `cargo test -p pen-search claim_`
- `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

If the new short slice earns keep, then branch back to one new full-profile rerun before parity, breadth, compare, benchmark, or certification work.

## Keep Or Branch Decision
After the next short rerun:
- stay on runtime work if the early or reopened surfaces still regress
- branch back to a new full-profile rerun only if the short rerun beats the kept short baseline honestly and materially narrows the reopened `74/76` gap to the kept full-profile aggregation baseline
- branch to parity, breadth, compare, benchmark, and certification work only after a later full-profile run honestly moves past the step-`4` wall

## Stop Condition For This Note
Rewrite this file as soon as one new stored rerun shows one of these is true:
- a lazy admitted-only metadata cut earns keep on the honest short surface
- the step-`4` wall moves away from clause filtering and aggregation first
- a later full-profile rerun reaches a new blocker honestly
- runtime work is no longer the next honest move
