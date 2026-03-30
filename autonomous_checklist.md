# Autonomous Claim Lane Checklist

Last updated: 2026-03-30

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready full-profile bundle.
- The current full-profile runtime reference is
  `runs/codex-claim-release-full-open-band-handoff-followup-v1`.
- The current short step-`4` baseline is now
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The most recent informative late-surface diagnostic is now
  `runs/codex-claim-release-step4-kernel-late-profile-v1`.
- The latest intended-profile follow-up preserved the honest retained-prefix
  shape on stored evidence:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through the stored `229` read
- That follow-up slightly trailed the earlier same-binary full-profile
  reference at `24/43/44/54/74/76`, but it improved the decisive later
  checkpoints and moved one stored checkpoint farther:
  - `140`: `2571309 / 2561049` instead of `2575049 / 2564601`
  - `163`: `2978288 / 2966621` instead of `2985344 / 2973404`
  - `228`: `4189959 / 4174213` instead of `4209220 / 4192906`
  - it then continued to `229`: `4211079 / 4195271`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- At `140/163/228/229` on that follow-up, stored step-live telemetry kept the
  later bucket order connectivity first, aggregation second, exact `nu`
  third, and terminal clause-filter handoff tiny:
  - `140`: connectivity `= 758717336 us`, aggregation `= 751161774 us`,
    exact `nu` `= 525949191 us`, handoff `= 10375779 us`
  - `163`: connectivity `= 884102477 us`, aggregation `= 862160190 us`,
    exact `nu` `= 609549050 us`, handoff `= 12108178 us`
  - `228`: connectivity `= 1238402593 us`, aggregation `= 1199772615 us`,
    exact `nu` `= 872697561 us`, handoff `= 17145752 us`
  - `229`: connectivity `= 1243971258 us`, aggregation `= 1208974002 us`,
    exact `nu` `= 875496908 us`, handoff `= 17225219 us`
  - `terminal_summary_admissibility_checks = 0` through the stored `229` read
- Observed RSS stayed below `833540096` bytes through the stored `229`
  checkpoint, so the intended profile is still throughput-bound rather than
  allocator-bound on the current winner.
- That rerun was manually stopped after the decisive stored `229` checkpoint.
  Because the stop was external during step `4`, `reports/latest.txt` still
  reflects completed step `3`, `run.json` still says `status = "running"`,
  and `reports/steps/step-05-live.ndjson` is absent; the authoritative
  evidence for that full-profile reference lives in
  `reports/steps/step-04-live.ndjson`.
- The eager metadata rerun
  `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
  preserved the same honest early and reopened shapes, but it failed keep
  badly on runtime and moved the visible wall to clause filtering first.
- At `76` on that slice, stored telemetry read:
  clause filtering `= 2178547522 us`, aggregation `= 456894681 us`,
  connectivity `= 412251293 us`, exact `nu` `= 269107583 us`.
- The same rerun also showed that once clause metadata already exists, full
  `AcceptRank` construction and canonical-key finalization become small, so
  the remaining lesson is to keep any metadata retry lazy and admitted-only
  rather than building it in terminal clause filtering.
- The newer admitted-only metadata rerun
  `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`
  then re-earned cheap clause filtering and kept the same honest early and
  reopened shapes, but it still failed keep on the matched early short
  surface because `terminal_summary_build_*` regressed by about `10-11%`.
- At `76` on that newer slice, stored telemetry read:
  connectivity `= 414014281 us`, aggregation `= 410788615 us`,
  clause filtering `= 355695170 us`, exact `nu` `= 263235482 us`.
- The newer reopened connectivity rerun
  `runs/codex-claim-release-step4-kernel-reopened-connectivity-v1`
  then kept the same honest early and reopened shapes, improved elapsed wall
  clock at `24/43/44/54/74/76`, and cut reopened connectivity timing
  materially, but it still failed keep because `terminal_summary_build_*`
  regressed by about `5.2-5.6%` on the matched early short surface and about
  `4.0%` at `74/76` versus the kept full-profile baseline.
- At `76` on that newest slice, stored telemetry read:
  aggregation `= 463408834 us`, clause filtering `= 356760236 us`,
  connectivity `= 282490143 us`, exact `nu` `= 265598332 us`.
- The newer direct bound/bookkeeping rerun
  `runs/codex-claim-release-step4-kernel-bound-bookkeeping-v1`
  then preserved the same honest early plateau at `24` and kept aggregation
  first, connectivity second, clause filtering third, and exact `nu` fourth,
  but it still failed keep because the matched early short surface read
  `549708 / 544700` instead of the kept `549630 / 492524`, while terminal
  materialization still stayed tiny at `336 ms`.
- The newer admitted-kernel rerun
  `runs/codex-claim-release-step4-kernel-admitted-kernel-v1`
  then preserved the same honest early plateau at `24/25`, improved elapsed
  materially at the matched `24` checkpoint, and lowered the measured
  aggregation bucket there, but it still failed keep because the matched
  early short surface read `519065 / 514192` instead of the kept
  `549630 / 492524`, while the broad early bucket order moved to connectivity
  first, aggregation second, clause filtering third, and exact `nu` fourth.
- The next honest move is therefore not another short step-`4` micro-slice or
  another plain rerun-only turn first. It is one narrow later-surface
  connectivity-side cut on the current winner so the post-`140` wall can move
  honestly on stored evidence.

## 1. Runtime Completion

- [x] Land one narrow step-`4` connectivity-side or aggregation-side
      throughput patch that earns keep on stored telemetry.
- [x] Re-earn one release `until_step = 4` rerun and confirm that the measured
      summary-side telemetry improves without weakening retained prefix-cache
      shape.
- [x] Re-earn one full `desktop_claim_shadow_1h` rerun on the winning binary.
- [ ] Finish one intended-profile claim run through step `15`.
- [x] Confirm from stored artifacts that the run no longer depends on the old
      allocator-failure story.
- [ ] Explain the observed RSS versus governor-accounted RSS behavior honestly
      from stored step artifacts.

Done when:

- one intended-profile claim run completes through step `15`
- the stored artifacts honestly explain the runtime pressure story

## 2. Stored Claim Evidence

- [ ] Preserve accepted-hash parity through step `15`.
- [ ] Restore step `1` generated raw count to exactly `2144`.
- [ ] Re-earn the required generated-count floors from stored evidence:
      step `10 >= 500`, step `11 >= 800`, step `12 >= 1200`,
      step `13 >= 2200`, step `14 >= 3500`, step `15 >= 5000`.
- [ ] Ensure early and late breadth claims come from stored generated counts,
      not from config intent.
- [ ] Keep `full_telescopes_evaluated` within a certified moderate threshold.
- [ ] Persist complete `exact_screen_reason_counts`.
- [ ] Persist complete `prune_class_counts`.
- [ ] Ensure late-step summaries expose generated, hard-admissible,
      exact-screened, exact-screen-pruned by reason, heuristic-dropped, and
      fully-evaluated totals.
- [ ] Make guarded, replay, realistic-shadow, or demo-only fallback impossible
      to miss in claim artifacts.

Done when:

- parity and breadth both pass from stored claim evidence
- a reviewer can understand the live claim lane from the stored bundle alone

## 3. Compare, Benchmark, And Certification

- [ ] Store one canonical claim-lane run directory from the disclosed desktop.
- [ ] Store one compare report against the guarded reference for that run.
- [ ] Store one benchmark bundle for that same run.
- [ ] Record benchmark median wall time.
- [ ] Record benchmark p90 wall time.
- [ ] Record benchmark max wall time.
- [ ] Record parity success count.
- [ ] Record floor-hit count.
- [ ] Record manifest snapshot alongside the benchmark outputs.
- [ ] Freeze one certified runtime threshold for the claim sentence.
- [ ] Store one passing `claim_certificate.json`.

Done when:

- another reviewer can audit the whole claim lane from one stable evidence set

## 4. Language Gate

- [ ] Keep user-facing and paper-facing wording at `bounded live recovery`
      until every section above is closed.
- [ ] Do not use the word `unguided` user-facing before the passing
      certificate exists.
- [ ] Update the stronger sentence only after the certification gate passes.
- [ ] Tie the stronger sentence explicitly to the stored claim certificate and
      disclosed desktop bundle.

Done when:

- stronger language appears only after the technical and evidence gates close
