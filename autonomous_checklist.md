# Autonomous Claim Lane Checklist

Last updated: 2026-04-04

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready certified bundle.
- The current authoritative stored evidence surface is the completed
  `v5` full-profile run plus its compare, certification, and benchmark
  outputs:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v5`.
- Stored `v5` still fails parity and breadth honestly:
  - accepted hashes diverge from guarded replay at steps `9`, `11`, and `12`
  - early breadth still misses at step `1` (`546` versus target `2144`)
  - late generated floors still miss at steps `11..15`
- A local step-`9` same-primary selector repair is now landed and guarded by
  tests, but the stored `v5` bundle still records the old step-`9` /
  step-`11` / step-`12` parity forks until `v6` is rerun.
- Local step-`11` and step-`12` repairs have now been rerun green on top of
  that repaired local step-`9` selector and should stay frozen as downstream
  guardrails until stored evidence catches up.
- Local step-`13`, step-`14`, and narrow step-`15` repairs are also landed and
  should stay frozen as guardrails:
  - step `13` currently reports `[3,1,3,3,1,1,1]` / `27` / `123`
  - step `14` currently reports `19683` / `12027`
  - step `15` currently preserves `DCT 103 / 8 / 1794`
- No new per-step claim search-band expansion should land before the canonical
  accepted path is stable and correct through step `15`.
- Keep the step-`1` breadth miss on the checklist, but do not confuse that
  signoff floor with the ordinal parity repair.
- Claim-policy honesty, fallback honesty, narrative/event completeness,
  exact-screen reason completeness, prune-class completeness, and manifest
  completeness are already earned on the stored full-profile run.

## 1. Stored Claim Evidence

- [ ] Restore accepted-hash parity first at step `9` on a stored full-profile
      bundle.
- [ ] Re-earn accepted-hash parity through steps `10..15` in ordinal order on
      top of the repaired step-`9` prefix.
- [ ] Defer any new per-step claim search-band expansion until the canonical
      accepted path through step `15` is stable and correct.
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
