# Autonomous Claim Lane Checklist

Last updated: 2026-04-03

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready certified bundle.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current short-loop gate is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`.
- The current later-wall step-`4` continuation winner through `576` is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`.
- The current corroborating middle-wall read through `335` is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v2`.
- The older farthest stored step-`4` continuation stop remains
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
  at `1095` explored prefixes.
- A fresh clean-start full-profile rerun,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v5`,
  now completes through step `15` on clean-tree repo head
  `c1fbb51d4fc9a620cd2ce95c9c3eadfe1a54fc65` with release binary hash
  `c42758f96c8171900651503d7f2a0ffe9915966c41edea98d8f1e296fc772a4e`.
- A compare report, claim certificate, and benchmark bundle now also exist
  beside that `v5` run and make the remaining signoff gaps explicit.
- Stored `v5` compare / certification / benchmark regression tests now pin
  those current parity and breadth failures in-tree, so the next change has to
  move real stored behavior rather than only the narrative around it.
- A new local claim regression now proves the guarded step-`11` completion
  survives the current claim terminal-clause catalog, full remaining-one
  summary, compact survivor sketch, and retained candidate pool on the live
  divergent history; no stored rerun has consumed that repair yet, so the
  signoff surface is still the frozen `v5` evidence set.
- The old `v3` step-`14` zero-candidate failure is therefore no longer the
  first blocker.
- The current blocker is stored parity plus stored breadth on the completed
  run:
  - compare says accepted hashes diverge from guarded replay at steps `9`,
    `11`, and `12`
  - step `9` and step `11` both keep guarded `nu / kappa` but still accept a
    different winner
  - step `12` drops to `nu = 33`, `kappa = 5` versus guarded `34 / 6`
  - step `13` and step `14` recover the guarded accepted hashes but stay one
    `nu` low
  - step `15` returns the guarded `DCT` hash / `nu` / `kappa`
- The stored claim certificate is still `attention` because:
  - accepted-hash parity fails
  - early breadth fails at step `1` (`546` versus target `2144`)
  - late generated floors fail at steps `11..15`
- The stored benchmark bundle says:
  - step-`15` completion count `= 1`
  - runtime `= 408 ms`
  - parity success count `= 0`
  - full early breadth hit count `= 0`
  - full late floor hit count `= 0`
- Claim-policy honesty, fallback honesty, narrative/event completeness,
  exact-screen reason completeness, prune-class completeness, and manifest
  completeness are now all earned on the stored full-profile run.

## 1. Runtime Improvement Loop

- [ ] Require every serious runtime slice to prove replay-harness parity plus
      either fewer exact-`nu` evaluations or lower measured aggregation time
      before another full-profile rerun.
- [ ] Explain observed RSS versus governor-accounted RSS honestly from stored
      step artifacts.

Done when:

- the stored artifacts honestly explain the runtime story and no longer leave
  the claim sentence blocked on missing runtime evidence

## 2. Stored Claim Evidence

- [ ] Preserve accepted-hash parity through step `15`.
- [ ] Restore step `1` generated raw count to exactly `2144`.
- [ ] Re-earn the required generated-count floors from stored evidence:
      step `10 >= 500`, step `11 >= 800`, step `12 >= 1200`,
      step `13 >= 2200`, step `14 >= 3500`, step `15 >= 5000`.
- [ ] Ensure early and late breadth claims come from stored generated counts,
      not from config intent.
- [ ] Keep `full_telescopes_evaluated` within a certified moderate threshold.

Done when:

- parity and breadth both pass from stored claim evidence
- a reviewer can understand the live claim lane from the stored bundle alone

## 3. Compare, Benchmark, And Certification

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
