# Autonomous Claim Lane Checklist

Last updated: 2026-04-01

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready full-profile bundle.
- The current full-profile runtime reference is
  `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The most recent informative late-surface diagnostic is
  `runs/codex-claim-release-step4-kernel-late-profile-v1`.
- The latest intended-profile follow-up preserved the honest retained-prefix
  shape on stored evidence, re-earned the later reopens, and then moved the
  stored step-`4` wall farther:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` through the stored `484` read
- That follow-up materially improved every decisive stored matched checkpoint
  through the previous stored `454` wall:
  - `24`: `327917 / 325321` instead of `385100 / 382267`
  - `43`: `591083 / 587479` instead of `693470 / 689512`
  - `44`: `603570 / 599913` instead of `708134 / 704116`
  - `54`: `743997 / 739797` instead of `872387 / 867757`
  - `74`: `1023572 / 1018283` instead of `1200035 / 1194139`
  - `76`: `1057864 / 1052464` instead of `1239059 / 1233043`
  - `140`: `2014043 / 2005230` instead of `2349156 / 2339322`
  - `163`: `2334208 / 2324213` instead of `2722673 / 2711471`
  - `228`: `3287422 / 3274060` instead of `3837038 / 3821952`
  - `229`: `3304737 / 3291324` instead of `3856643 / 3841498`
  - `332`: `4885155 / 4866258` instead of `5704569 / 5683113`
  - `335`: `4931017 / 4911930` instead of `5758074 / 5736396`
  - `408`: `6031554 / 6007913` instead of `7014570 / 6988332`
  - `437`: `6494645 / 6469171` instead of `7548400 / 7520336`
  - `454`: `6770742 / 6744132` instead of `7860534 / 7831399`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- At `140/163/228/229/332/335/408/454/484`, stored step-live telemetry still
  shows aggregation first, connectivity second, exact `nu` third, and
  terminal clause-filter handoff tiny:
  - `140`: aggregation `= 763398133 us`, connectivity `= 557557788 us`,
    exact `nu` `= 511854832 us`, handoff `= 25988115 us`
  - `163`: aggregation `= 877573120 us`, connectivity `= 651431110 us`,
    exact `nu` `= 594142794 us`, handoff `= 29787220 us`
  - `228`: aggregation `= 1222738289 us`, connectivity `= 916833230 us`,
    exact `nu` `= 851890919 us`, handoff `= 35116478 us`
  - `229`: aggregation `= 1231827955 us`, connectivity `= 920978899 us`,
    exact `nu` `= 854659192 us`, handoff `= 35324213 us`
  - `332`: aggregation `= 1817428261 us`, connectivity `= 1345141843 us`,
    exact `nu` `= 1288881640 us`, handoff `= 46243052 us`
  - `335`: aggregation `= 1833436572 us`, connectivity `= 1357602031 us`,
    exact `nu` `= 1302219954 us`, handoff `= 46350411 us`
  - `408`: aggregation `= 2218628958 us`, connectivity `= 1675112606 us`,
    exact `nu` `= 1596901898 us`, handoff `= 54702325 us`
  - `454`: aggregation `= 2480479553 us`, connectivity `= 1875448296 us`,
    exact `nu` `= 1808063671 us`, handoff `= 58420696 us`
  - `484`: aggregation `= 2641777960 us`, connectivity `= 2010145015 us`,
    exact `nu` `= 1932111468 us`, handoff `= 65158646 us`
  - `terminal_summary_admissibility_checks = 0` and
    `terminal_summary_fallback_connectivity_checks = 0` through the stored
    `484` read
- Inference from the matched checkpoint totals:
  the new winner's main gain comes from shrinking the previously unattributed
  summary-build tail rather than from lower measured aggregation itself.
- Observed RSS reached `1581830144` bytes by the stored `484` checkpoint.
  That is slightly above the previous runtime reference on the matched later
  surface, but it still stays well below the old allocator-failure band, so
  the intended profile is still throughput-bound rather than allocator-bound
  on the current winner.
- That rerun was manually stopped after the decisive stored `484` checkpoint.
  Because the stop was external during step `4`, `reports/latest.txt` still
  reflects completed step `3`, `run.json` still says `status = "running"`,
  and `reports/steps/step-05-live.ndjson` is absent; the authoritative
  evidence for that full-profile reference lives in
  `reports/steps/step-04-live.ndjson`.
- The replay harness layer is now landed end-to-end:
  the repo now stores deterministic plateau fixtures for
  `39/144845`, `40/147639`, `41/154842`, `42/157636`, and `43/160430`, and
  the refreshed release benchmark now reads `3326`, `4564`, `2782`, `2553`,
  and `2085 us` average on those five retained surfaces.
- The first harness-backed facts-only hit-path slice is now landed in code:
  the claim remaining-one hot loop can now stay on clause refs plus
  predecoded connectivity facts plus predecoded structural-`nu` facts through
  bound checks, summary build, compact materialization, clause-catalog reuse,
  filtered active-window clones, and replay fixtures.
- That local replay read is mixed but net-positive on the stored corpus:
  `39/40` regressed slightly, `41/42/43` improved clearly, and the aggregate
  five-surface total fell from `155131 us` to `153124 us` while keeping
  compact-summary parity.
- The next honest move is therefore not another short step-`4` micro-slice,
  another code-only exploration pass, or another connectivity-first retry. It
  is the intended-profile rerun on the landed facts-only slice so the
  post-`140` / post-`228` wall can be judged from stored step-`4` evidence.
- A tiny survivor sketch and a dense `lib_refs` membership set remain
  reasonable harness-backed follow-ups if the first facts-only slice still
  leaves too much second-pass duplication. Deterministic batched parallel
  reduction stays gated behind the harness and parity checks.

## 1. Runtime Completion

- [x] Build a deterministic replay harness for retained plateau prefixes from
      stored `step-04-live.ndjson` evidence.
- [x] Persist deterministic plateau fixtures for `39/144845`, `40/147639`,
      `41/154842`, `42/157636`, and `43/160430`.
- [x] Benchmark only
      `compute_terminal_prefix_completion_summary_from_candidates(...)` on
      those stored fixtures before another expensive later-surface slice lands.
- [x] Use that harness to validate the next facts-only hot-loop slice before
      judging it from another multi-hour full-profile rerun.
- [ ] Keep deterministic batched parallel reduction gated behind harness-backed
      parity evidence.
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
