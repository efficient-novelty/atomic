# Autonomous Claim Lane Progress
Last updated: 2026-03-30

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final gate.

## Current Status
- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is `runs/codex-claim-release-step4-kernel-aggregation-v1`.
- The current full-profile baseline is `runs/codex-claim-release-full-kernel-aggregation-v1`.
- The authoritative late-surface diagnostic is `runs/codex-claim-release-step4-kernel-late-profile-v1`.
- The latest measured slice is `runs/codex-claim-release-step4-kernel-aggregation-tiecut-v1`.
- That rerun was manually stopped after enough stored step-`4` evidence had landed through `prefix_states_explored = 80`; it is dropped from code after failing keep on the matched early short surface and the reopened full-profile surface.
- The intended profile is still blocked in step `4` by remaining-one compact-summary throughput.
- The lane is still compute-bound in step `4`, and the visible reopened wall is still aggregation first. On the newest stored rerun the bucket order stayed aggregation first, clause filtering second, connectivity third, and exact `nu` fourth.

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
- Run: `runs/codex-claim-release-step4-kernel-aggregation-tiecut-v1`
- Hypothesis: keep the aggregation baseline code, keep the reopened connectivity retry dropped, and cut reopened-surface aggregation cost by deferring compact-summary full `AcceptRank` construction whenever a new best primary rank already strictly beats the incumbent.
- Outcome:
  - it preserved the same honest early plateau and the reopened `74/76` shape
  - it kept clause filtering near the kept late diagnostic
  - it cut late aggregation modestly relative to the kept late diagnostic
  - but it still failed keep because both elapsed wall clock and `terminal_summary_build_*` regressed on the matched early short surface, and the reopened `74/76` read still trailed the kept full-profile baseline
- Comparison versus the kept short baseline:
  - `24`: `558655 / 553615` instead of `549630 / 492524`
  - `43`: `1011015 / 1004860` instead of `990480 / 892772`
  - `44`: `1033239 / 1027024` instead of `1012067 / 912271`
  - `54`: `1274068 / 1267275` instead of `1247600 / 1126754`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Reopened-surface read versus the kept full-profile baseline:
  - `74`: `elapsed_millis = 1745950`, `terminal_summary_build_millis = 1738100`
  - `76`: `elapsed_millis = 1800387`, `terminal_summary_build_millis = 1792430`
  - baseline comparison:
    - `74`: kept full-profile `1743244 / 1579138`
    - `76`: kept full-profile `1797441 / 1628768`
- At `76`, the measured bucket order became:
  - aggregation `= 453514026 us`
  - clause filtering `= 346694360 us`
  - connectivity `= 397810359 us`
  - exact `nu` `= 263740192 us`
- Incremental `54 -> 76` became:
  - aggregation `+133894436 us`
  - clause filtering `+103929214 us`
  - connectivity `+113861249 us`
  - exact `nu` `+77298166 us`
- Honest read:
  - the compact-summary exact-rank deferral is real on the late aggregation bucket and on total build time versus `runs/codex-claim-release-step4-kernel-late-profile-v1`
  - it still did not earn keep because the stored short surface regressed by about `1.6-2.1%` on elapsed and about `12.4-12.6%` on `terminal_summary_build_*` at `24/43/44/54`, while the reopened `74/76` surface still regressed by about `0.16%` on elapsed and about `10.0%` on `terminal_summary_build_*` versus the kept full-profile baseline
  - exact accept-rank finalization is therefore part of the reopened aggregation wall, but not enough by itself to re-earn the baseline surfaces

### 5. Earlier Failed Reopened Connectivity Slice
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

### 6. Earlier Failed Admitted-Only Metadata Slice
- Run: `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`
- It preserved the same honest early plateau and reopened `74/76/140` shape, re-earned cheap clause filtering, and improved elapsed time relative to the late diagnostic, but it still failed keep because `terminal_summary_build_*` regressed by about `10-11%` on the matched early short surface.
- At `76`, the measured bucket order was:
  - connectivity `= 414014281 us`
  - aggregation `= 410788615 us`
  - clause filtering `= 355695170 us`
  - exact `nu` `= 263235482 us`

### 7. Earlier Failed Eager Metadata Slice
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
- Compact-summary strict-better-incumbent exact-rank deferral: `kernel-aggregation-tiecut-v1`
- Eager terminal-clause metadata pack: `kernel-clause-metadata-v1`
- Lazy admitted-only metadata retry: `kernel-admitted-metadata-v1`
- Parent-summary connectivity lookup reuse: `kernel-reopened-connectivity-v1`

## Revised Working Diagnosis
- The old early RSS cliff remains broken; this is still a step-`4` throughput problem, not a return of the allocator-failure story.
- The kept baselines still say the intended profile later reopens beyond the early `39/144845` plateau:
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` at `140`
- The reopened connectivity rerun and the compact-summary exact-rank-deferral rerun together now show that the remaining compact-summary wall is a composite per-admitted kernel rather than one missing scalar gate.
- In the current code, each admitted candidate can still pay for multiple exact operations inside `compute_terminal_prefix_completion_summary_from_candidates`: clause load into scratch state, admissibility-diagnostics bookkeeping, exact bit-cost recovery, bound update, primary-rank math, and sometimes full `AcceptRank` construction.
- The full `AcceptRank` path is still real when it fires because it rebuilds full-telescope structural signals, max-var-ref context, and canonical-key context, but the failed keep reads show it is not the only remaining wall.
- The accumulated lesson from the dropped threshold-only, bound-only, bookkeeping-only, competition-gate-only, exact-`nu`-gate-only, bit-cost-only, contender-rank-only, and strict-better-incumbent exact-rank-deferral slices is that another one-constant retry is unlikely to be enough unless it removes several exact per-admitted rescans at once.
- Because `76` still reads aggregation `> clause filtering > connectivity > exact nu`, the honest reopened wall remains aggregation first.

## Best Current Inference
The next honest retry should keep the winning baseline code and keep only the lesson from the newest reruns:

> do not spend the next slice on connectivity first, clause filtering first, or another single-axis aggregation constant first. The next runtime wall to attack is reopened-surface aggregation, and it should be attacked as one exact metadata-pack slice.

That means the next plausible keep slice is not another unchanged metadata retry, not another unchanged connectivity reuse retry, and not another unchanged compact-summary exact-rank deferral retry. It is one different reopened-surface aggregation cut that:
- leaves terminal clause filtering cheap
- preserves the current exact tie-break truth surface
- removes several exact per-admitted rescans together by pairing terminal-clause exact metadata with one prefix-side exact aggregate
- and improves the later `74/76` wall without giving back the matched `24/43/44/54` read

## Immediate Next Move
1. Keep the code behind `runs/codex-claim-release-step4-kernel-aggregation-v1`, `runs/codex-claim-release-full-kernel-aggregation-v1`, and `runs/codex-claim-release-step4-kernel-late-profile-v1`.
2. Do not keep either metadata retry, the reopened connectivity lookup reuse, or the compact-summary strict-better-incumbent exact-rank deferral in code as standalone next moves.
3. Land, if possible, one exact aggregation-side metadata pack that lets the summary loop reuse terminal-clause bit-cost, structural-signal, max-var-ref, and canonical-tail context together with one prefix-side exact aggregate.
4. Inside the same patch, make full `AcceptRank` and exact canonical-key finalization truly last-tie-only, and add fine-grained aggregation telemetry for clause load, diagnostics bookkeeping, bit-cost recovery, bound update, primary-rank math, full `AcceptRank`, and canonical-key finalization.
5. Re-run a short release claim slice to `--until-step 4` and read the stored artifacts at `24/43/44/54/74/76`.
6. Only branch back to a new full-profile rerun if a later short slice earns keep against the matched early plateau and materially improves the reopened `74/76` read against the kept full-profile baseline.

## What Has Not Changed
- Do not branch to compare, benchmark, certification, or stronger language before step `4` moves or a full-profile run finishes.
- Do not reopen allocator-first, frontier-first, or broad early-frontier rewrites as the next primary move.
- Do not replace the current short baseline with an unmeasured or diagnostic-only hypothesis.
