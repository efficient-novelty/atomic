# Autonomous Claim Lane Progress
Last updated: 2026-03-30

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final gate.

## Current Status
- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current full-profile baseline is `runs/codex-claim-release-full-open-band-handoff-v1`.
- The authoritative late-surface diagnostic is `runs/codex-claim-release-step4-kernel-late-profile-v1`.
- The latest measured slice is `runs/codex-claim-release-full-open-band-handoff-v1`.
- That intended-profile rerun re-earned the full-profile read on stored evidence:
  - it preserved the honest `39 groups / 144845 candidates` plateau at `24/43/44/54`
  - it preserved the honest reopened `40 groups / 147639 candidates` surface at `74/76`
  - it reopened again to `41 groups / 154842 candidates` by `140`
  - it then moved materially beyond the old full-profile wall to a stored `228` checkpoint while still in step `4`
- Decisive stored checkpoints on that rerun are:
  - `24`: `417512 / 414611`
  - `43`: `747903 / 743859`
  - `44`: `763654 / 759550`
  - `54`: `941234 / 936486`
  - `74`: `1292196 / 1286212`
  - `76`: `1336730 / 1330612`
  - `140`: `2575049 / 2564601`
  - `163`: `2985344 / 2973404`
  - `228`: `4209220 / 4192906`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- The later stored bucket order on that rerun stayed a step-`4` throughput composite rather than a clause-filter or allocator relapse:
  - `228`: connectivity `= 1249033160 us`, aggregation `= 1209742017 us`, exact `nu` `= 866913331 us`, terminal clause-filter handoff `= 17262280 us`
  - `terminal_summary_admissibility_checks = 0` through the stored `228` read
- Observed RSS on that rerun stayed below `837795840` bytes through the stored `228` checkpoint, well below the old allocator-failure band.
- The completed step summaries still keep the observed-versus-accounted RSS story explicit: step `1` stored green pressure with `rss_bytes = 1342177376`, `observed_process_rss_bytes = 18309120`, and `rss_gap_bytes = -1323868256`, while steps `2` and `3` stored zero frontier pressure; the stopped step-`4` rerun never sealed a step summary, so the late blocker is still read from `reports/steps/step-04-live.ndjson`.
- Because the rerun was manually stopped after the new later blocker was already visible, `reports/latest.txt` still reflects completed step `3`, `run.json` still says `status = "running"`, and `reports/steps/step-05-live.ndjson` is absent; the authoritative evidence for that rerun lives in `reports/steps/step-04-live.ndjson`.
- The lane is still compute-bound, but the blocker has moved materially later inside step `4`.

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

## Baselines That Matter
### 1. Current Short Baseline
- Run: `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Honest retained shape:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
- The rerun was manually stopped after the stored `76` checkpoint because the decisive short and reopened surfaces had already been captured; one extra stored `77` checkpoint flushed while stopping and kept the same honest `40 groups / 147639 candidates` reopened surface.
- Because the stop was external during step `4`, the authoritative evidence for this short winner is `reports/steps/step-04-live.ndjson`, while `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`.
- Matched checkpoints:
  - `24`: `elapsed_millis = 417756`, `terminal_summary_build_millis = 414838`
  - `43`: `elapsed_millis = 760135`, `terminal_summary_build_millis = 755953`
  - `44`: `elapsed_millis = 777287`, `terminal_summary_build_millis = 773037`
  - `54`: `elapsed_millis = 962821`, `terminal_summary_build_millis = 957858`
  - `74`: `elapsed_millis = 1315892`, `terminal_summary_build_millis = 1309667`
  - `76`: `elapsed_millis = 1358533`, `terminal_summary_build_millis = 1352182`

### 2. Current Full-Profile Baseline
- Run: `runs/codex-claim-release-full-open-band-handoff-v1`
- It re-earned the kept short winner on the real intended profile and moved the full-profile step-`4` wall materially later than `runs/codex-claim-release-full-kernel-aggregation-v1`.
- Honest retained shape:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through the stored `228` read
- The run never reached step `5` on stored evidence; `reports/steps/step-05-live.ndjson` is absent.
- Stored checkpoints:
  - `24`: `elapsed_millis = 417512`, `terminal_summary_build_millis = 414611`
  - `43`: `elapsed_millis = 747903`, `terminal_summary_build_millis = 743859`
  - `44`: `elapsed_millis = 763654`, `terminal_summary_build_millis = 759550`
  - `54`: `elapsed_millis = 941234`, `terminal_summary_build_millis = 936486`
  - `74`: `elapsed_millis = 1292196`, `terminal_summary_build_millis = 1286212`
  - `76`: `elapsed_millis = 1336730`, `terminal_summary_build_millis = 1330612`
  - `140`: `elapsed_millis = 2575049`, `terminal_summary_build_millis = 2564601`
  - `163`: `elapsed_millis = 2985344`, `terminal_summary_build_millis = 2973404`
  - `228`: `elapsed_millis = 4209220`, `terminal_summary_build_millis = 4192906`
- It stayed materially ahead of the earlier full-profile aggregation baseline at every decisive later checkpoint:
  - `140`: `2575049 / 2564601` instead of `3393970 / 3085651`
  - `163`: `2985344 / 2973404` instead of `3942636 / 3584914`
- Later stored bucket order at `228` was:
  - connectivity `= 1249033160 us`
  - aggregation `= 1209742017 us`
  - exact `nu` `= 866913331 us`
  - terminal clause-filter handoff `= 17262280 us`
- Observed RSS reached `837795840` bytes at `228`, still far below the old allocator-failure band.
- Because the rerun was manually stopped during step `4` after the later blocker was already visible, `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`; the authoritative evidence for this run is `reports/steps/step-04-live.ndjson`.

### 3. Earlier Full-Profile Aggregation Baseline
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

### 4. Current Late-Surface Diagnostic
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

### 5. Latest Kept Open-Band Handoff Slice
- Run: `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Hypothesis: keep the aggregation baseline code plus the admitted-kernel win, and cut the remaining claim open-band clause-filter handoff cost by letting claim terminal clause filtering hand the summary/materialize kernels only exact-admitted clause refs rather than per-clause admissibility payloads that the hot path immediately discards.
- Outcome:
  - it preserved the same honest early plateau at `24/43/44/54`: `39 groups / 144845 candidates`
  - it preserved the honest reopened `40 groups / 147639 candidates` surface at `74/76`
  - it materially improved both elapsed and `terminal_summary_build_*` at every stored matched checkpoint and at the reopened `74/76` surface
  - the rerun was manually stopped after the stored `76` checkpoint because the decisive short and reopened surfaces had already been captured; one extra stored `77` checkpoint flushed while stopping and kept the same `40 groups / 147639 candidates` reopened surface
  - because the stop was external during step `4`, `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`; the authoritative evidence for this slice lives in `reports/steps/step-04-live.ndjson`
- Comparison versus the prior kept short baseline and kept full-profile reopened surface:
  - `24`: `417756 / 414838` instead of `549630 / 492524`
  - `43`: `760135 / 755953` instead of `990480 / 892772`
  - `44`: `777287 / 773037` instead of `1012067 / 912271`
  - `54`: `962821 / 957858` instead of `1247600 / 1126754`
  - `74`: `1315892 / 1309667` instead of `1743244 / 1579138`
  - `76`: `1358533 / 1352182` instead of `1797441 / 1628768`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Stored step-live kernel telemetry says the targeted handoff cost is now tiny on the keep slice:
  - `24`: terminal clause-filter handoff `= 1716095 us`, connectivity `= 125969317 us`, aggregation `= 122046408 us`, exact `nu` `= 80279671 us`, terminal materialize `= 298 ms`
  - `76`: terminal clause-filter handoff `= 5572740 us`, connectivity `= 410676521 us`, aggregation `= 391673461 us`, exact `nu` `= 269118565 us`, terminal materialize `= 302 ms`
  - `terminal_summary_admissibility_checks = 0` through the stored `76` read
- Honest read:
  - the broader open-band handoff cut engaged without weakening the retained-prefix shape
  - the early short surface moved back under the kept baseline honestly on both elapsed and `terminal_summary_build_*`
  - the reopened `74/76` surface also moved materially under the kept full-profile baseline on both elapsed and `terminal_summary_build_*`
  - the next honest move is therefore a full-profile rerun on this winning binary rather than another short step-`4` micro-slice first

### 6. Earlier Failed Admitted-Kernel Slice
- Run: `runs/codex-claim-release-step4-kernel-admitted-kernel-v1`
- Hypothesis: keep the aggregation baseline code, specialize the claim open-band admitted surface so the remaining-one summary and direct compact materialization kernels stop carrying full per-clause admissibility payloads, stop doing per-clause open-band reason-map bookkeeping, and stop rechecking the competition gate on a surface that is already exact-admitted and focus-aligned.
- Outcome:
  - it preserved the same honest early plateau at `24/25`: `39 groups / 144845 candidates`
  - it materially improved elapsed at the matched `24` checkpoint
  - it materially lowered the measured aggregation bucket there
  - but it still failed keep immediately because the matched early short surface did not move back under the kept baseline on `terminal_summary_build_*`
  - the rerun was manually stopped after the stored `24` checkpoint because that early build failure was already decisive; one extra `25` checkpoint flushed while stopping and kept the same `39 groups / 144845 candidates` plateau
- Comparison versus the kept short baseline:
  - `24`: `519065 / 514192` instead of `549630 / 492524`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- At `24`, the measured early bucket read was:
  - connectivity `= 121124566 us`
  - aggregation `= 119212285 us`
  - clause filtering `= 113491706 us`
  - exact `nu` `= 78051194 us`
  - terminal materialize `= 324 ms`
- Honest read:
  - the admitted-kernel fast path engaged without weakening the retained-prefix shape
  - the slice really did remove aggregation-side work and improved elapsed by about `5.6%` at `24`
  - it still failed keep because `terminal_summary_build_*` stayed about `4.4%` above the kept short baseline at that same checkpoint
  - the stored early wall is no longer aggregation-first on this slice; it is now a tighter connectivity / aggregation / clause-filter composite
  - another admitted-kernel-only retry is therefore not the next honest standalone move

### 7. Earlier Failed Bound/Bookkeeping Slice
- Run: `runs/codex-claim-release-step4-kernel-bound-bookkeeping-v1`
- Hypothesis: keep the aggregation baseline code, cut broader per-admitted structural work by avoiding redundant reason-string clones inside `AdmissibilityDiagnostics::record`, and absorb exact completion bounds in place inside the remaining-one summary kernel and direct compact materialization path instead of building a temporary singleton bound for every admitted candidate.
- Outcome:
  - it preserved the same honest early plateau at `24`: `39 groups / 144845 candidates`
  - it kept the same broad early bucket order at that checkpoint: aggregation first, connectivity second, clause filtering third, exact `nu` fourth
  - but it still failed keep immediately because the matched early short surface did not move back under the kept baseline on either elapsed or `terminal_summary_build_*`
  - the rerun was manually stopped after the stored `24` checkpoint because the early failure was already decisive; one extra `25` checkpoint flushed while stopping and kept the same `39 groups / 144845 candidates` plateau
- Comparison versus the kept short baseline:
  - `24`: `549708 / 544700` instead of `549630 / 492524`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- At `24`, the measured early bucket read was:
  - aggregation `= 132199915 us`
  - connectivity `= 125043669 us`
  - clause filtering `= 105522189 us`
  - exact `nu` `= 80087659 us`
  - terminal materialize `= 336 ms`
- Honest read:
  - the local bookkeeping/bound cleanup engaged without weakening the retained-prefix shape
  - the slice still failed keep because elapsed was only effectively flat-to-slightly-worse at `24` while `terminal_summary_build_*` still regressed by about `10.6%`
  - aggregation remained the largest visible bucket, connectivity and clause filtering stayed materially hot, and compact materialization stayed tiny
  - another bookkeeping/bound-only cleanup is therefore not the next honest standalone move

### 8. Earlier Failed Clause-Load Slice
- Run: `runs/codex-claim-release-step4-kernel-clause-load-v1`
- Hypothesis: restore the kept aggregation baseline code, remove the already-dropped competition-gate hoist from the summary kernel, and cut broader remaining-one structural cost by reusing the terminal scratch clause slot with `clone_from` instead of cloning a fresh clause payload into that slot on every scratch overwrite.
- Outcome:
  - it preserved the same honest early plateau at `24`: `39 groups / 144845 candidates`
  - it kept the same broad early bucket order at that checkpoint: aggregation first, connectivity second, clause filtering third, exact `nu` fourth
  - but it still failed keep immediately because the matched early short surface regressed on both elapsed and `terminal_summary_build_*`
  - the rerun was manually stopped after the stored `24` checkpoint because the early failure was already decisive
- Comparison versus the kept short baseline:
  - `24`: `562592 / 557559` instead of `549630 / 492524`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- At `24`, the measured early bucket read was:
  - aggregation `= 143755610 us`
  - connectivity `= 126724055 us`
  - clause filtering `= 105192716 us`
  - exact `nu` `= 79702207 us`
  - terminal materialize `= 344 ms`
- Honest read:
  - the scratch-slot clause-load reuse engaged without weakening the retained-prefix shape
  - the slice still regressed the matched early surface by about `2.4%` on elapsed and about `13.2%` on `terminal_summary_build_*` at `24`
  - one narrower scratch-slot clause-load reuse is therefore not enough by itself to re-earn the kept baseline, and the next runtime cut still needs a broader win inside the structural kernel

### 9. Earlier Failed Exact Rank-Metadata Slice
- Run: `runs/codex-claim-release-step4-kernel-rank-metadata-v1`
- Hypothesis: keep the aggregation baseline code, carry one lazy exact terminal-clause metadata pack plus one prefix-side exact rank context through the compact summary loop, compare contenders first on exact numeric fields, and finalize canonical keys only on true numeric ties while recording fine-grained aggregation telemetry.
- Outcome:
  - it preserved the same honest early plateau at `24`: `39 groups / 144845 candidates`
  - it proved the intended tail behavior on stored evidence: at `24`, full accept-rank construction was only `543075 us` and canonical-key finalization was only `5 us`
  - but it failed keep immediately because the matched early short surface regressed badly on both elapsed and `terminal_summary_build_*`
  - the rerun was manually stopped after the stored `24` checkpoint because the early failure was already decisive
- Comparison versus the kept short baseline:
  - `24`: `1232202 / 872432` instead of `549630 / 492524`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- At `24`, the measured early bucket read was:
  - aggregation `= 140971173 us`
  - connectivity `= 137191827 us`
  - clause filtering `= 135071766 us`
  - exact `nu` `= 79983349 us`
  - compact materialize `= 319890 ms`
- Aggregation sub-buckets at `24` were:
  - clause load / scratch update `= 61477661 us`
  - admissibility bookkeeping `= 22145864 us`
  - bit-cost recovery `= 11259134 us`
  - bound update `= 11437024 us`
  - primary-rank math `= 14524895 us`
  - full `AcceptRank` construction `= 543075 us`
  - canonical-key finalization `= 5 us`
- Honest read:
  - canonical-key finalization really is the rare tail now, and numeric accept-rank construction is also small
  - the slice still regressed the matched early surface by about `124.2%` on elapsed and about `77.1%` on `terminal_summary_build_*` at `24`
  - the next blocker is therefore not canonical-tail work first; the broad remaining cost still sits in clause load, bookkeeping, bound, connectivity, clause filtering, and compact materialization

### 10. Earlier Failed Exact-Rank Deferral Slice
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

### 11. Earlier Failed Reopened Connectivity Slice
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

### 12. Earlier Failed Admitted-Only Metadata Slice
- Run: `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`
- It preserved the same honest early plateau and reopened `74/76/140` shape, re-earned cheap clause filtering, and improved elapsed time relative to the late diagnostic, but it still failed keep because `terminal_summary_build_*` regressed by about `10-11%` on the matched early short surface.
- At `76`, the measured bucket order was:
  - connectivity `= 414014281 us`
  - aggregation `= 410788615 us`
  - clause filtering `= 355695170 us`
  - exact `nu` `= 263235482 us`

### 13. Earlier Failed Eager Metadata Slice
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
- Claim open-band admitted-kernel fusion: `kernel-admitted-kernel-v1`
- Direct bound/bookkeeping absorb cleanup: `kernel-bound-bookkeeping-v1`
- Scratch-slot clause-load reuse: `kernel-clause-load-v1`
- Exact-`nu` high-water gate: `kernel-nu-highwater-v1`
- Summary-invariants accept-rank prefix-context rewrite: `kernel-summary-invariants-v1`
- Summary-rank-context exact tie-break rewrite: `kernel-rank-context-v1`
- Terminal-rank sidecar exact contender-rank rewrite: `kernel-terminal-rank-sidecar-v1`
- Primary-rank context exact-threshold rewrite: `kernel-primary-context-v1`
- Summary-constant bit-cost hoist: `kernel-summary-constant-v1`
- Catalog-backed clause bit-cost sidecar: `kernel-catalog-constant-v1`
- Bar-clear threshold bookkeeping rewrite: `kernel-summary-threshold-v1`
- Exact rank-metadata pack with last-tie canonical-key finalization: `kernel-rank-metadata-v1`
- Compact-summary strict-better-incumbent exact-rank deferral: `kernel-aggregation-tiecut-v1`
- Eager terminal-clause metadata pack: `kernel-clause-metadata-v1`
- Lazy admitted-only metadata retry: `kernel-admitted-metadata-v1`
- Parent-summary connectivity lookup reuse: `kernel-reopened-connectivity-v1`

## Revised Working Diagnosis
- The old early RSS cliff remains broken; this is still a step-`4` throughput problem, not a return of the allocator-failure story.
- The new full-profile rerun on the open-band-handoff winner preserved the decisive early and reopened surfaces on the real `desktop_claim_shadow_1h` profile:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through the stored `228` read
- That rerun also moved materially farther than the prior full-profile baseline:
  - `140`: `2575049 / 2564601` instead of `3393970 / 3085651`
  - `163`: `2985344 / 2973404` instead of `3942636 / 3584914`
  - it then continued to `228` at `4209220 / 4192906` before the external stop
- The later stored wall on the current winner is no longer the old early plateau or the old `163` stop point; it is the post-`140` step-`4` surface where connectivity and aggregation stay nearly tied, exact `nu` remains the third cost, and terminal clause-filter handoff stays tiny.
- Completed step summaries still keep the observed-versus-accounted RSS story explicit, and the live step-`4` stream kept observed RSS below about `0.84 GiB`; the current blocker is therefore later step-`4` runtime, not a revived memory cliff.
- The accumulated lesson from the dropped retries is now stronger: the open-band handoff cut won on both short and full-profile evidence, so the next honest runtime choice is no longer "rerun or not?" but "which later connectivity-versus-aggregation cut should move first on this winner?"

## Best Current Inference
The broader claim open-band handoff cut is now both the short keep winner and the current full-profile runtime reference.

The next honest question is no longer whether this winner survives the intended profile. It does. The next honest question is which later step-`4` throughput component after the `41 / 154842` reopen should move first:
- connectivity-side exact summary work
- aggregation-side per-admitted bookkeeping and ranking work
- or a still-later structural effect that only appears beyond the current stored `228` read

The stored evidence is already good enough to stop reopening plain rerun-only turns or new early-surface micro-slices first:
- the real full-profile run preserved the short `24/43/44/54` win
- it preserved the reopened `74/76` win and the later `41/154842` widen
- it moved materially beyond the old `163` wall
- clause-filter handoff and admissibility rechecks remain negligible on the current winner

## Immediate Next Move
1. Keep the code behind `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`, treat `runs/codex-claim-release-full-open-band-handoff-v1` as the current full-profile runtime reference, and keep `runs/codex-claim-release-full-kernel-aggregation-v1` only as the comparison baseline.
2. Do not reopen another early short step-`4` micro-slice first, and do not spend another turn on a plain intended-profile rerun with no new runtime question.
3. Use the stored late surface on `runs/codex-claim-release-full-open-band-handoff-v1` to pick one later-runtime follow-up that targets the post-`140` wall, with connectivity-side and aggregation-side work as the only live first-cut candidates.
4. If the next turn is diagnostic-first, rerun a release claim follow-up that goes far enough to cover at least the `140/163/228` region and keep live checkpoint persistence on so the later bucket order is re-earned on stored evidence.
5. If code changes land before that rerun, rerun only:
   - `cargo test -p pen-search claim_`
   - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`
6. Branch to parity, breadth, compare, benchmark, or certification work only after a later full-profile rerun either reaches step `5` or moves materially past the current stored `228` wall.

## What Has Not Changed
- Do not branch to compare, benchmark, certification, or stronger language before step `4` moves or a full-profile run finishes.
- Do not reopen allocator-first, frontier-first, or broad early-frontier rewrites as the next primary move.
- Do not replace the current short or full-profile baseline with an unmeasured or diagnostic-only hypothesis.
