# Autonomous Claim Lane Checklist

Last updated: 2026-04-05

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready certified bundle.
- The current authoritative stored evidence surface is the completed
  `v9` full-profile run plus its compare, certification, and benchmark
  outputs:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v9`.
- Stored `v9` now passes accepted-hash parity honestly:
  - trajectory and accepted hashes match guarded replay through step `15`
  - compare signoff is `ready`
  - the claim audit is `ready`
- The transient `v7` rerun first re-earned stored step `11` but reopened
  accepted-hash parity at step `12`; clean-tree `v9` now restores that parity
  while keeping the stored step-`11` repair.
- Stored `v9` still fails breadth honestly:
  - early breadth still misses at step `1` (`546` versus target `2144`)
  - late generated floors still miss at step `13` (`123` versus `2200`) and
    step `15` (`1794` versus `5000`)
  - late generated floors now hit at step `10`, step `11`, step `12`, and
    step `14`
- Two newer local step-`13` widened probes are now frozen as negative
  controls only:
  `[3,5,3,3,5,1,1]` and `[5,1,3,3,5,3,3]` can reopen the local floor, but
  neither one yet preserves accepted-hash parity through step `14`; the first
  position-`1` / position-`4` reland is now also frozen as an executable
  regression on the repaired step-`12` chain.
- The stored `v9` certificate now also carries step-level breadth diagnosis
  for the open misses, so step `13` / step `15` catalog widths, root seeding,
  exact-screen pressure, and the full stored step-open pressure signature are
  visible from the bundle itself.
- The current canonical later surfaces are frozen on stored evidence:
  - step `13` reports `[3,1,3,3,1,1,1]` / `27` / `123`
  - step `14` reports `19683` / `12027`
  - step `15` preserves `DCT 103 / 8 / 1794`
- No new per-step claim search-band expansion should land before the stored
  breadth repair proves it is necessary on this parity-clean chain.
- Claim-policy honesty, fallback honesty, narrative/event completeness,
  exact-screen reason completeness, prune-class completeness, manifest
  completeness, and accepted-hash parity are already earned on the stored
  full-profile run.

## 1. Stored Claim Evidence

- [ ] Restore step `1` generated raw count to exactly `2144`.
- [ ] Re-earn the remaining generated-count floors from stored evidence:
      step `13 >= 2200`, step `15 >= 5000`.
- [ ] Keep `full_telescopes_evaluated` within a certified moderate threshold.

Done when:

- breadth passes from stored claim evidence without losing accepted-hash parity
- a reviewer can understand the live claim lane from the stored bundle alone

## 2. Compare, Benchmark, And Certification

- [ ] Store one passing `claim_certificate.json`.

Done when:

- another reviewer can audit the whole claim lane from one stable evidence set

## 3. Language Gate

- [ ] Keep user-facing and paper-facing wording at `bounded live recovery`
      until every section above is closed.
- [ ] Do not use the word `unguided` user-facing before the passing
      certificate exists.
- [ ] Update the stronger sentence only after the certification gate passes.
- [ ] Tie the stronger sentence explicitly to the stored claim certificate and
      disclosed desktop bundle.

Done when:

- stronger language appears only after the technical and evidence gates close
