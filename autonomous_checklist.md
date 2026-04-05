# Autonomous Claim Lane Checklist

Last updated: 2026-04-05

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready certified bundle.
- The current authoritative stored evidence surface is the completed
  `v6` full-profile run plus its compare, certification, and benchmark
  outputs:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v6`.
- Stored `v6` now passes accepted-hash parity honestly:
  - trajectory and accepted hashes match guarded replay through step `15`
  - compare signoff is `ready`
  - the claim audit is `ready`
- Stored `v6` still fails breadth honestly:
  - early breadth still misses at step `1` (`546` versus target `2144`)
  - late generated floors still miss at step `11` (`330` versus `800`),
    step `13` (`123` versus `2200`), and step `15` (`1794` versus `5000`)
  - late generated floors now hit at step `10`, step `12`, and step `14`
- A new local step-`11` breadth repair is now landed and guarded, but no
  stored rerun has consumed it yet:
  - the connected claim step-`11` surface now holds
    `kappa 5 = 243`, `kappa 6 = 729` (total `972`)
  - the guarded step-`11` shell still stays accepted locally
  - stored `v6` remains the authoritative breadth read until a fresh rerun
    re-earns step `11` on disk
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
      step `11 >= 800`, step `13 >= 2200`, step `15 >= 5000`.
- [ ] Keep `full_telescopes_evaluated` within a certified moderate threshold.

Done when:

- breadth passes from stored claim evidence without losing accepted-hash parity
- a reviewer can understand the live claim lane from the stored bundle alone

## 2. Compare, Benchmark, And Certification

- [ ] Freeze one certified runtime threshold for the claim sentence.
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
