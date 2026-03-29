# Autonomous Claim Lane Progress
Last updated: 2026-03-29

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final gate.

## Current Status
- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is `runs/codex-claim-release-step4-kernel-aggregation-v1`.
- The current full-profile baseline is `runs/codex-claim-release-full-kernel-aggregation-v1`.
- The authoritative late-surface diagnostic is `runs/codex-claim-release-step4-kernel-late-profile-v1`.
- The latest measured slice is `runs/codex-claim-release-step4-kernel-terminal-rank-sidecar-v1`, and it was dropped from code after failing keep.
- The lane is still compute-bound in step `4` on the intended profile.

## What Stays Landed
- delayed materialization
- the incumbent-primary remaining-one fast path
- the one-pass `structural_nu` summary-build fast path
- the algebraic `nu` ceiling patch
- the family-agnostic claim terminal-admissibility shortcut
- the exact non-allocating connectivity summary scan
- the terminal-only cached parent connectivity decision
- the aggregation-side accept-rank short-circuit that skips full `AcceptRank` construction for primary-dominated bar-clearers
- the higher-fidelity late-surface timing accumulation used by the current short diagnostic surface

## Baselines That Matter
### 1. Current Short Baseline
- Run: `runs/codex-claim-release-step4-kernel-aggregation-v1`
- Honest retained shape: `39 groups / 144845 candidates` at `24/43/44/54`
- Matched checkpoints:
  - `24`: `elapsed_millis = 549630`, `terminal_summary_build_millis = 492524`
  - `43`: `elapsed_millis = 990480`, `terminal_summary_build_millis = 892772`
  - `44`: `elapsed_millis = 1012067`, `terminal_summary_build_millis = 912271`
  - `54`: `elapsed_millis = 1247600`, `terminal_summary_build_millis = 1126754`

### 2. Current Full-Profile Baseline
- Run: `runs/codex-claim-release-full-kernel-aggregation-v1`
- It materially improved the old intended-profile baseline at `24/43/44/54` and proved that the winning short kernel shape survives on the real `desktop_claim_shadow_1h` profile.
- The retained prefix cache first plateaued at `39 groups / 144845 candidates` from `24` through `73`, then reopened to:
  - `40 groups / 147639 candidates` at `74`
  - `41 groups / 154842 candidates` at `140`
- The run never reached step `5`.
- The last stored step-`4` checkpoint was `163` with:
  - `elapsed_millis = 3942636`
  - `observed_process_rss_bytes = 632197120`
  - `frontier_queue_len = 2612`
  - `terminal_summary_build_millis = 3584914`

### 3. Current Late-Surface Diagnostic
- Run: `runs/codex-claim-release-step4-kernel-late-profile-v1`
- It reproduces the intended-profile reopens on short evidence and still tracks the full-profile baseline closely at `74/76/140`.
- At `76`:
  - `terminal_summary_build_micros = 1839910636`
  - aggregation `= 469431036 us`
  - connectivity `= 416766880 us`
  - clause filtering `= 352203534 us`
  - exact `nu` `= 267574759 us`
  - unexplained tail `= 333934427 us` (`18.15%`)
- Incremental `54 -> 76`:
  - aggregation `+141716373 us`
  - connectivity `+124894828 us`
  - clause filtering `+107776335 us`
  - exact `nu` `+80574865 us`

### 4. Latest Failed Slice
- Run: `runs/codex-claim-release-step4-kernel-terminal-rank-sidecar-v1`
- Hypothesis: keep the current primary-rank short-circuit, but remove whole-prefix exact contender-rank rescans by combining one prefix-side exact rank context with exact terminal-clause rank metadata.
- Outcome: preserved the same honest plateau and reopened shapes at `24/43/44/54/74/76`, materially beat the late diagnostic on the reopened surface, but still materially regressed the matched early `24/43/44/54` surface and still trailed the kept full-profile aggregation baseline on total `terminal_summary_build_*` at `74/76`, so it did not earn keep.
- Comparison versus the kept short baseline:
  - `24`: `550360 / 545601` instead of `549630 / 492524`
  - `43`: `1000033 / 994244` instead of `990480 / 892772`
  - `44`: `1022008 / 1016165` instead of `1012067 / 912271`
  - `54`: `1267127 / 1260682` instead of `1247600 / 1126754`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Reopened-surface read:
  - `74`: `elapsed_millis = 1752569`, `terminal_summary_build_micros = 1744964766`
  - `76`: `elapsed_millis = 1803362`, `terminal_summary_build_micros = 1795650034`
- Honest read: exact contender-rank rescans are real work, but they are not the whole remaining wall.

## What Stays Dropped
- Ordering and reuse variants: `context-equivalence-v1`, `incumbent-ordering-v1`, `local-two-step-order-v2`, `proof-close-handoff-v1`, `post-plateau-v1`, `post-plateau-materialize-v1`, `post-plateau-summary-cache-v3`
- Connectivity-side cache and precompute variants: `kernel-connectivity-v3`, `kernel-connectivity-v4`
- Candidate-prep / remap variants: `terminal-candidate-prep-v1`
- Telemetry-only slice: `kernel-filter-profile-v1`
- Exact primary-rank bookkeeping rewrite: `kernel-rank-bookkeeping-v1`
- Bound-merge bookkeeping rewrite: `kernel-bound-merge-v1`
- Lazy incumbent-tie `AcceptRank` deferral rewrite: `kernel-lazy-acceptrank-v1`
- Summary-side batching rewrite: `kernel-summary-batching-v1`
- Summary-side bookkeeping rewrite: `kernel-summary-bookkeeping-v1`
- Prefix-wide competition-gate hoist: `kernel-competition-hoist-v1`
- Exact-`nu` high-water gate: `kernel-nu-highwater-v1`
- Summary-invariants accept-rank prefix-context rewrite: `kernel-summary-invariants-v1`
- Summary-rank-context exact tie-break rewrite: `kernel-rank-context-v1`
- Terminal-rank sidecar exact contender-rank rewrite: `kernel-terminal-rank-sidecar-v1`
- Primary-rank context exact-threshold rewrite: `kernel-primary-context-v1`
- Summary-constant bit-cost hoist: `kernel-summary-constant-v1`
- Catalog-backed clause bit-cost sidecar: `kernel-catalog-constant-v1`
- Bar-clear threshold bookkeeping rewrite: `kernel-summary-threshold-v1`

## Revised Working Diagnosis
- The old early RSS cliff has already been broken by earlier landed memory and frontier compactions.
- The intended-profile blocker is now step-`4` summary throughput, not early memory explosion.
- The earlier `39 groups / 144845 candidates` plateau is real, but it is not the terminal intended-profile surface. The real profile later reopens to `40/147639` and `41/154842`.
- On that reopened surface, aggregation is still the largest measured bucket, but the code path shows that this bucket is a **bundle of multiple exact per-admitted operations**, not one isolated missing constant.

### Code-level read of the current hot loop
In the current `compute_terminal_prefix_completion_summary_from_candidates` path, every admitted candidate in compact-summary mode still pays for all of the following inside the measured summary kernel:
- clause load into scratch telescope state
- admissibility-diagnostics bookkeeping
- exact bit-cost recovery for the terminal clause
- bound absorption
- primary-rank math
- optional full `AcceptRank` construction when the candidate can still contend

When full `AcceptRank` does fire, the current code still rebuilds whole-telescope structural signals, max-var-ref context, and canonical-key context from the telescope itself. That makes the remaining wall look more like an accumulation of exact clause-local and whole-telescope rescans than like one missing gate.

### What the failed slices now imply
- The threshold-only, bound-only, competition-gate-only, exact-`nu`-gate-only, bookkeeping-only, bit-cost-only, and contender-rank-only slices all found real work, but none removed enough of the total measured step-`4` wall to earn keep.
- The terminal-rank-sidecar result is especially useful: it improved the reopened surface honestly, so exact contender-rank rescans are real, but because the early plateau still regressed materially and the kept full-profile aggregation baseline still wins at `74/76`, contender-rank rescans are also not dominant enough on their own.
- The present evidence therefore argues against another single-axis slice as the next primary move.

## Best Current Inference
The next winning move should stay inside the compact summary path, but it should no longer be framed as "one more scalar constant." The stronger hypothesis is:

> land one exact terminal-clause metadata pack that removes several per-admitted rescans at once and reuses the same exact metadata in both compact summary and rare full-rank comparison.

Concretely, the most plausible next keep slice is one that bundles, in one narrow change:
- exact terminal-clause bit-cost reuse
- exact terminal-clause structural-signal reuse
- exact terminal-clause max-var-ref reuse
- exact canonical-key suffix or equivalent exact rolling key context so canonical-key finalization becomes last-tie only rather than ordinary contender work

This is a code-reading inference, not yet a measured win. The baselines above stay authoritative until a stored rerun proves otherwise.

## Immediate Next Move
1. Keep the code behind `runs/codex-claim-release-step4-kernel-aggregation-v1`, `runs/codex-claim-release-full-kernel-aggregation-v1`, and `runs/codex-claim-release-step4-kernel-late-profile-v1`.
2. Land one exact terminal-clause metadata-pack slice in the remaining-one compact-summary kernel.
3. Add fine-grained aggregation sub-bucket timing inside the same patch rather than as a separate diagnostic-only branch.
4. Re-run a short release claim slice to `--until-step 4` and read the stored artifacts at `24/43/44/54/74/76`.
5. Only branch back to a new full-profile rerun if that short slice earns keep against the matched early plateau and materially narrows the reopened `74/76` gap to the kept full-profile aggregation baseline.

## What Has Not Changed
- Do not branch to compare, benchmark, certification, or stronger language before step `4` moves or a full-profile run finishes.
- Do not reopen allocator-first, frontier-first, or broad connectivity-first work as the next primary move.
- Do not replace the current short baseline with an unmeasured or diagnostic-only hypothesis.
