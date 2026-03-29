# Autonomous Claim Lane Progress
Last updated: 2026-03-29

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final gate.

## Current Status
- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is `runs/codex-claim-release-step4-kernel-aggregation-v1`.
- The current full-profile baseline is `runs/codex-claim-release-full-kernel-aggregation-v1`.
- The authoritative late-surface diagnostic is `runs/codex-claim-release-step4-kernel-late-profile-v1`.
- The latest measured slice is `runs/codex-claim-release-step4-kernel-reopened-connectivity-v1`.
- That rerun was manually stopped after enough stored step-`4` evidence had landed through `prefix_states_explored = 76`; it is dropped from code after failing keep on the matched early short surface and the reopened `terminal_summary_build_*` surface.
- The lane is still compute-bound in step `4`, but the visible reopened wall is no longer connectivity first. On the newest stored rerun it has moved to aggregation first, with clause filtering second and connectivity third.

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
- Stored reopened checkpoints:
  - `74`: `elapsed_millis = 1743244`, `terminal_summary_build_millis = 1579138`
  - `76`: `elapsed_millis = 1797441`, `terminal_summary_build_millis = 1628768`
- The last stored step-`4` checkpoint was `163` with:
  - `elapsed_millis = 3942636`
  - `observed_process_rss_bytes = 632197120`
  - `frontier_queue_len = 2612`
  - `terminal_summary_build_millis = 3584914`

### 3. Current Late-Surface Diagnostic
- Run: `runs/codex-claim-release-step4-kernel-late-profile-v1`
- It reproduced the intended-profile reopens on short evidence and tracked the full-profile baseline closely at `74/76/140`.
- At `76`:
  - `terminal_summary_build_micros = 1839910636`
  - clause filtering `= 352203534 us`
  - aggregation `= 469431036 us`
  - connectivity `= 416766880 us`
  - exact `nu` `= 267574759 us`
- Incremental `54 -> 76`:
  - clause filtering `+107776335 us`
  - aggregation `+141716373 us`
  - connectivity `+124894828 us`
  - exact `nu` `+80574865 us`

### 4. Latest Measured Slice
- Run: `runs/codex-claim-release-step4-kernel-reopened-connectivity-v1`
- Hypothesis: keep the aggregation baseline code, remove the lingering admitted-only rank-metadata path from code, and cut reopened-surface connectivity cost by reusing one parent legality summary across each remaining-one clause scan.
- Outcome:
  - it preserved the same honest early plateau and the reopened `74/76` shape
  - it materially improved elapsed wall clock at `24/43/44/54/74/76`
  - it kept clause filtering near the kept late diagnostic
  - it cut reopened connectivity timing sharply
  - but it still failed keep because `terminal_summary_build_*` regressed on both the matched early surface and the reopened `74/76` build surface
- Comparison versus the kept short baseline:
  - `24`: `523076 / 518007` instead of `549630 / 492524`
  - `43`: `948473 / 942259` instead of `990480 / 892772`
  - `44`: `969247 / 962975` instead of `1012067 / 912271`
  - `54`: `1197195 / 1190324` instead of `1247600 / 1126754`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Reopened-surface read versus the kept full-profile baseline:
  - `74`: `elapsed_millis = 1649766`, `terminal_summary_build_millis = 1641724`
  - `76`: `elapsed_millis = 1702496`, `terminal_summary_build_millis = 1694340`
  - baseline comparison:
    - `74`: kept full-profile `1743244 / 1579138`
    - `76`: kept full-profile `1797441 / 1628768`
- At `76`, the measured bucket order became:
  - aggregation `= 463408834 us`
  - clause filtering `= 356760236 us`
  - connectivity `= 282490143 us`
  - exact `nu` `= 265598332 us`
- Incremental `54 -> 76` became:
  - aggregation `+140197060 us`
  - clause filtering `+108329001 us`
  - connectivity `+80429105 us`
  - exact `nu` `+79520130 us`
- Honest read:
  - the parent-summary connectivity reuse was real on elapsed wall clock and on the connectivity bucket itself
  - it still did not earn keep because the stored summary-build surface regressed by about `5.2-5.6%` at `24/43/44/54` and about `4.0%` at `74/76`
  - the visible reopened blocker is now aggregation first rather than connectivity first

### 5. Earlier Failed Admitted-Only Metadata Slice
- Run: `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`
- It preserved the same honest early plateau and reopened `74/76/140` shape, re-earned cheap clause filtering, and improved elapsed time relative to the late diagnostic, but it still failed keep because `terminal_summary_build_*` regressed by about `10-11%` on the matched early short surface.
- At `76`, the measured bucket order was:
  - connectivity `= 414014281 us`
  - aggregation `= 410788615 us`
  - clause filtering `= 355695170 us`
  - exact `nu` `= 263235482 us`

### 6. Earlier Failed Eager Metadata Slice
- Run: `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
- It preserved the same honest early plateau and reopened `74/76` shape, but regressed catastrophically on runtime and moved the visible wall to clause filtering first, so it did not earn keep.
- At `76`, the measured bucket order was:
  - clause filtering `= 2178547522 us`
  - aggregation `= 456894681 us`
  - connectivity `= 412251293 us`
  - exact `nu` `= 269107583 us`
- Its remaining lesson stays true:
  - eager exact clause metadata inside `terminal_prefix_clause_candidates` is too early and should stay dropped

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
- Eager terminal-clause metadata pack: `kernel-clause-metadata-v1`
- Lazy admitted-only metadata retry: `kernel-admitted-metadata-v1`
- Parent-summary connectivity lookup reuse: `kernel-reopened-connectivity-v1`

## Revised Working Diagnosis
- The old early RSS cliff remains broken; this is still a step-`4` throughput problem, not a return of the allocator-failure story.
- The kept baselines still say the intended profile later reopens beyond the early `39/144845` plateau.
- The newest stored rerun proves three things at once:
  - cutting repeated parent-summary lookups can reduce reopened connectivity cost materially
  - clause filtering can stay near the kept late diagnostic without metadata work
  - but the build surface can still regress even while elapsed wall clock improves
- Because `76` now reads aggregation `> clause filtering > connectivity > exact nu`, the honest reopened wall has shifted again.

## Best Current Inference
The next honest retry should keep the winning baseline code and keep only the lesson from the newest rerun:

> do not spend the next slice on metadata or connectivity first. The next runtime wall to attack is reopened-surface aggregation.

That means the next plausible keep slice is not another metadata retry and not another unchanged connectivity reuse retry. It is one narrower reopened-surface aggregation cut that:
- leaves terminal clause filtering cheap
- preserves the current exact tie-break truth surface
- and improves the later `74/76` wall without giving back the matched `24/43/44/54` read

## Immediate Next Move
1. Keep the code behind `runs/codex-claim-release-step4-kernel-aggregation-v1`, `runs/codex-claim-release-full-kernel-aggregation-v1`, and `runs/codex-claim-release-step4-kernel-late-profile-v1`.
2. Do not keep either metadata retry or the reopened connectivity lookup reuse in code.
3. Land, if possible, one narrow reopened-surface aggregation-side runtime cut rather than another metadata or connectivity pass.
4. Re-run a short release claim slice to `--until-step 4` and read the stored artifacts at `24/43/44/54/74/76`.
5. Only branch back to a new full-profile rerun if that short slice earns keep against the matched early plateau and materially improves the reopened `74/76` read against the kept full-profile baseline.

## What Has Not Changed
- Do not branch to compare, benchmark, certification, or stronger language before step `4` moves or a full-profile run finishes.
- Do not reopen allocator-first, frontier-first, or broad early-frontier rewrites as the next primary move.
- Do not replace the current short baseline with an unmeasured or diagnostic-only hypothesis.
