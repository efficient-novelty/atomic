# Autonomous Claim Lane Checklist

Last updated: 2026-03-30

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready full-profile bundle.
- The current full-profile runtime reference is
  `runs/codex-claim-release-full-connectivity-facts-v1`.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The most recent informative late-surface diagnostic is
  `runs/codex-claim-release-step4-kernel-late-profile-v1`.
- The latest intended-profile follow-up preserved the honest retained-prefix
  shape on stored evidence and then reopened twice farther:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `408` read
- That follow-up materially improved every decisive stored matched checkpoint
  through `229`:
  - `24`: `385802 / 383018` instead of `417678 / 414814`
  - `43`: `698661 / 694743` instead of `760980 / 756851`
  - `44`: `713483 / 709504` instead of `777344 / 773151`
  - `54`: `880003 / 875413` instead of `961793 / 956919`
  - `74`: `1210334 / 1204512` instead of `1325033 / 1318766`
  - `76`: `1250591 / 1244645` instead of `1368485 / 1362079`
  - `140`: `2389805 / 2379752` instead of `2571309 / 2561049`
  - `163`: `2773573 / 2762030` instead of `2978288 / 2966621`
  - `228`: `3924018 / 3908221` instead of `4189959 / 4174213`
  - `229`: `3945205 / 3929337` instead of `4211079 / 4195271`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- At `140/163/228/229/332/335/408`, stored step-live telemetry now shows
  aggregation first, connectivity second, exact `nu` third, and terminal
  clause-filter handoff tiny:
  - `140`: aggregation `= 744641415 us`, connectivity `= 586579312 us`,
    exact `nu` `= 520449833 us`, handoff `= 15799216 us`
  - `163`: aggregation `= 857070798 us`, connectivity `= 685837174 us`,
    exact `nu` `= 604333257 us`, handoff `= 18798149 us`
  - `228`: aggregation `= 1200334910 us`, connectivity `= 968606567 us`,
    exact `nu` `= 869713548 us`, handoff `= 28170861 us`
  - `229`: aggregation `= 1210184906 us`, connectivity `= 973284301 us`,
    exact `nu` `= 872719297 us`, handoff `= 28348740 us`
  - `332`: aggregation `= 1799313029 us`, connectivity `= 1424660906 us`,
    exact `nu` `= 1323961772 us`, handoff `= 45034612 us`
  - `335`: aggregation `= 1814967597 us`, connectivity `= 1437778259 us`,
    exact `nu` `= 1337241409 us`, handoff `= 45526234 us`
  - `408`: aggregation `= 2185774562 us`, connectivity `= 1756872641 us`,
    exact `nu` `= 1633848702 us`, handoff `= 58019282 us`
  - `terminal_summary_admissibility_checks = 0` and
    `terminal_summary_fallback_connectivity_checks = 0` through the stored
    `408` read
- Observed RSS stayed below `964218880` bytes through the stored `408`
  checkpoint, so the intended profile is still throughput-bound rather than
  allocator-bound on the current winner.
- That rerun was manually stopped after the decisive stored `408` checkpoint.
  Because the stop was external during step `4`, `reports/latest.txt` still
  reflects completed step `3`, `run.json` still says `status = "running"`,
  and `reports/steps/step-05-live.ndjson` is absent; the authoritative
  evidence for that full-profile reference lives in
  `reports/steps/step-04-live.ndjson`.
- The next honest move is therefore not another short step-`4` micro-slice, a
  plain rerun-only turn, or another connectivity-first retry. It is one
  narrow later-surface aggregation-side cut on the current winner so the
  post-`140` aggregation wall can move honestly on stored evidence.

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
