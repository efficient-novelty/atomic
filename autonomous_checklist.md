# Autonomous Claim Lane Checklist

Last updated: 2026-04-01

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready full-profile bundle.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current full-profile speed winner is
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`.
- The previous full-profile speed winner was
  `runs/codex-claim-release-full-aggregation-open-band-structural-nu-facts-v1`.
- The current deeper continuation target is
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`.
- The explicit no-miss plateau-kernel split is now landed, and its capped
  intended-profile contender is
  `runs/codex-claim-release-full-aggregation-open-band-plateau-kernel-split-v1`.
- The current winner preserved the honest retained-prefix story through the
  stored `576` read, beat `prefix-nu-context-v2` at matched later checkpoints
  through `533`, and still did not reach step `5`.
- The decisive later surfaces are now no-miss surfaces on stored evidence:
  `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0`.
- The lane is still blocked by step-`4` throughput, not by the old allocator
  failure story.

## 1. Runtime Improvement Loop

- [ ] Replace `lib_refs` tree membership on the hot path with the intended
      tiered representation:
      inline small array, dense bitset after threshold, boxed sorted slice
      only where serialization or debug parity needs it.
- [ ] Add a tiny survivor sketch only if second-pass duplication is still
      visible after the kernel split and `lib_refs` work.
- [ ] Keep deterministic batched parallel reduction gated behind replay-harness
      parity evidence and do not open it first.
- [ ] Require every serious runtime slice to prove replay-harness parity plus
      either fewer exact-`nu` evaluations or lower measured aggregation time
      before another full-profile rerun.
- [ ] On a later contender, re-earn `140/163` on the intended profile.
- [ ] After that, re-earn `332/335` on the intended profile.
- [ ] After that, re-earn `408/437/454/484` on the intended profile.
- [ ] Move materially past the current winner's stored `576` wall.
- [ ] Move materially past the deeper continuation target's stored `1038` wall
      or reach step `5`.
- [ ] Finish one intended-profile claim run through step `15`.
- [ ] Explain observed RSS versus governor-accounted RSS honestly from stored
      step artifacts.

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
