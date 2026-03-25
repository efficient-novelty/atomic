# Autonomous Claim Lane Checklist

Last updated: 2026-03-25

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready full-profile bundle.
- The current full-profile baseline is
  `runs/codex-claim-release-full-nu-profile-v1`.
- The next concrete task is one narrow deterministic proof-close handoff patch
  on the exact-two-step remaining-one surface, then a stored release
  `until_step = 4` rerun.
- The current hot bottleneck is `terminal_summary_build_millis` in step `4`,
  not compact materialization.

## 1. Runtime Completion

- [ ] Land one narrow step-`4` summary-side throughput patch that earns keep on
      stored telemetry.
- [ ] Re-earn one release `until_step = 4` rerun and confirm that summary-side
      telemetry improves without weakening retained prefix-cache shape.
- [ ] Re-earn one full `desktop_claim_shadow_1h` rerun on the winning binary.
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
