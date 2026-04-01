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
- The broadened incumbent-relevant survivor sketch is now landed, the
  follow-on compact-summary bookkeeping slice now reuses borrowed
  primary-rank refs while preserving cached multi-primary sketch
  materialization in tests, and a newer clause-bit-cost sidecar reuse now
  keeps terminal clause bit cost inside `TerminalClauseNuFacts` while
  backfilling older replay fixtures on load, so the remaining-one plateau
  kernel and compact admitted bookkeeping stop recursively re-walking the
  same clause expr just to recover `bit_kappa_used`.
- The clause-bit-cost sidecar slice stayed parity-clean in the claim-focused
  tests and release replay parity, beat the immediate pre-slice local release
  reread of `134660 us`, and re-earned the checked-in `123544 us` replay gate
  on warmed rereads at `122830 us` and `122493 us`; the refreshed stored
  benchmark artifact was rewritten from a later under-gate reread at
  `123148 us`, although extra spot-check rereads still bounced around the
  gate at `124012 us` and `126456 us`, so the replay win is real but narrow.
- That earned capped intended-profile rerun was then spent on
  `runs/codex-claim-release-full-aggregation-open-band-clause-bit-cost-sidecar-v1`,
  whose nearest stored step-`4` checkpoint to `1200000 ms` landed at
  `1192222 ms`, `122` explored prefixes, `40 groups / 109690 candidates`,
  `frontier_queue_len = 2653`, RSS `= 489394176`,
  `terminal_summary_build_millis = 1184060`, and still
  `0` fallback/admissibility checks, so the current stored short-loop gate to
  beat still remains `plateau-kernel-split-v1` at `124` explored prefixes.
- A fresh follow-on slice now precomputes a prefix-local accept-rank context
  and reuses it in the compact remaining-one no-miss branches so those hot
  best-rank updates stop loading a scratch telescope and rescanning the full
  terminal telescope just to break primary-rank ties.
- That follow-on slice has already cleared the same claim-focused tests plus
  `claim_replay_fixture_replays_compact_summary_with_parity`, but its release
  replay parity and warmed rereads are still pending before another intended-
  profile rerun is justified.
- A later local-only focus-aligned competition-gate/payload-mode hoist stayed
  parity-clean in tests and replay parity, but its warm rereads landed
  `136040 us`, `137054 us`, and `140843 us` total after a `130405 us`
  immediate pre-slice local reread, so that code was dropped.
- Two later local-only compact-summary bookkeeping slices also stayed
  parity-clean in tests and replay parity, but both regressed and were
  dropped:
  the shared compact sketch/best-primary bookkeeping fold landed
  `140129 us`, `140565 us`, and `145083 us`, while the claim-open-band
  compact local-state hoist landed `129362 us`, `133001 us`, and `164098 us`
  total across `24 / 74 / 140 / 332 / 335`.
- The current winner preserved the honest retained-prefix story through the
  stored `576` read, beat `prefix-nu-context-v2` at matched later checkpoints
  through `533`, and still did not reach step `5`.
- The decisive later surfaces are now no-miss surfaces on stored evidence:
  `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0`.
- The lane is still blocked by step-`4` throughput, not by the old allocator
  failure story.

## 1. Runtime Improvement Loop
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
