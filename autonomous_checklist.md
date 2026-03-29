# Autonomous Claim Lane Checklist

Last updated: 2026-03-29

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready full-profile bundle.
- The current full-profile baseline is
  `runs/codex-claim-release-full-kernel-aggregation-v1`.
- The current short step-`4` baseline is now
  `runs/codex-claim-release-step4-kernel-aggregation-v1`.
- The most recent informative late-surface diagnostic is now
  `runs/codex-claim-release-step4-kernel-late-profile-v1`.
- That diagnostic rerun kept the same honest `39 groups / 144845 candidates`
  plateau through `24/43/44/54`, then reopened to `40/147639` at `74/76`
  and `41/154842` at `140` while still tracking the intended full-profile
  baseline closely on elapsed time, RSS, and frontier queue length.
- The newest eager metadata rerun
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
- The next honest move is therefore not another eager metadata pack and not a
  full-profile rerun first. It is one narrower admitted-only metadata retry on
  the kept short baseline code.

## 1. Runtime Completion

- [x] Land one narrow step-`4` connectivity-side or aggregation-side
      throughput patch that earns keep on stored telemetry.
- [x] Re-earn one release `until_step = 4` rerun and confirm that the measured
      summary-side telemetry improves without weakening retained prefix-cache
      shape.
- [x] Re-earn one full `desktop_claim_shadow_1h` rerun on the winning binary.
- [ ] Finish one intended-profile claim run through step `15`.
- [ ] Confirm from stored artifacts that the run no longer depends on the old
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
