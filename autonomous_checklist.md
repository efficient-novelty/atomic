# Autonomous Claim Lane Checklist

Last updated: 2026-04-02

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready full-profile bundle.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current later-wall continuation winner through the stored `576` wall is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`.
- The current farthest stored step-`4` continuation to beat is still
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`.
- The previous full-profile speed winner before that was
  `runs/codex-claim-release-full-aggregation-open-band-structural-nu-facts-v1`.
- The current deeper continuation target is
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`.
- The explicit no-miss plateau-kernel split is now landed, and the current
  short-loop contender to beat is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`.
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
  reread of `134660 us`, and earned the prior capped intended-profile rerun,
  but its stored `1192222 ms` / `122`-prefix checkpoint still lost honestly to
  `plateau-kernel-split-v1`.
- The newer prefix-local accept-rank-context slice then stayed parity-clean in
  the targeted claim tests and release replay harness, beat the checked-in
  `123148 us` replay total on every warmed release reread (`98385 us`,
  `103447 us`, `111079 us`, `94576 us`, and `96905 us`), and refreshed the
  stored benchmark artifact from a later under-gate reread at `102513 us`.
- Its earned capped intended-profile rerun on
  `runs/codex-claim-release-full-aggregation-open-band-prefix-accept-rank-context-v1`
  then landed the previous short-loop gate:
  `1190946 ms`, `135` explored prefixes, `40 groups / 28438 candidates`,
  `frontier_queue_len = 2640`, RSS `= 445005824`,
  `terminal_summary_build_millis = 1183014`, and still
  `0` fallback/admissibility checks.
- A newer clause-side accept-rank-facts slice then stayed parity-clean in the
  targeted claim tests and release replay harness, beat the checked-in
  `102513 us` replay total on every warmed release reread (`90614 us`,
  `98305 us`, and `88197 us`), and refreshed the stored benchmark artifact
  from a later under-gate reread at `88197 us`.
- Its earned capped intended-profile rerun on
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`
  then landed the current stored short-loop gate:
  `1191501 ms`, `139` explored prefixes, `40 groups / 28438 candidates`,
  `frontier_queue_len = 2636`, RSS `= 453021696`,
  `terminal_summary_build_millis = 1183915`, and still
  `0` fallback/admissibility checks.
- The lane now has repeated honest 20-minute wins on the same retained-prefix
  surface, and a fresh longer intended-profile rerun on the same binary/config
  then moved the later stored step-`4` walls honestly through `576` while
  keeping the same no-miss shape:
  - `140`: `1188340 / 1181188` with `41 groups / 29249 candidates`
  - `163`: `1381425 / 1373168` with `41 groups / 29249 candidates`
  - `332`: `2853118 / 2836752` with `42 groups / 29529 candidates`
  - `335`: `2879368 / 2862850` with `43 groups / 29809 candidates`
  - `408`: `3511930 / 3491831` with `43 groups / 29809 candidates`
  - `437`: `3770964 / 3749419` with `43 groups / 29809 candidates`
  - `454`: `3922561 / 3900177` with `43 groups / 29809 candidates`
  - `484`: `4183978 / 4160100` with `43 groups / 29809 candidates`
  - `576`: `4997082 / 4968579` with `43 groups / 29809 candidates`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- That rerun was manually stopped at `578` explored prefixes with
  `frontier_queue_len = 2197`, RSS `= 1744551936`, and still
  `0` fallback/admissibility checks.
- A second fresh intended-profile rerun on the same binary/config then
  re-earned the matched `140/163/332/335` walls while keeping the same
  no-miss shape:
  - `140`: `1195999 / 1188793` with `41 groups / 29249 candidates`
  - `163`: `1390249 / 1381921` with `41 groups / 29249 candidates`
  - `332`: `2874771 / 2858216` with `42 groups / 29529 candidates`
  - `335`: `2901198 / 2884494` with `43 groups / 29809 candidates`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- That second rerun was slightly slower than
  `clause-accept-rank-facts-long-rerun-v1` at each matched wall and was then
  manually stopped at `345` explored prefixes with
  `frontier_queue_len = 2430`, RSS `= 1056485376`, and still
  `0` fallback/admissibility checks, so `long-rerun-v1` remains the current
  later-wall winner through `576`.
- The next open runtime question is now whether another fresh rerun can carry
  that same smaller retained surface honestly back through `576`, then move
  materially past the older `1038` wall, then the older `1095` stop, or reach
  step `5`.
- Neither `clause-accept-rank-facts-v1` nor either longer rerun persisted a
  resumable step-`4` frontier generation, so the next continuation still needs
  another fresh intended rerun rather than `pen-cli resume`.
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
