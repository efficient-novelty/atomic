# Autonomous Claim Lane Checklist

Last updated: 2026-04-03

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
- Local step-`11` and step-`12` repairs are landed, but no stored rerun has
  consumed them yet.
- The remaining local late blocker is now split cleanly:
  - a scoped claim-only widening is now landed at step `13`:
    claim-open widens to raw widths `[3,1,3,3,1,1,1]`, raw catalog `27`, and
    live generated prefixes `33` while preserving the guarded accepted metric
    shell and the same observed `step-13 -> step-15` continuation
  - the remaining step-`13` loss is now residual exact screening on that
    widened surface:
    legality/connectivity rejection `24`, partial-prefix bar failure `12`,
    and incumbent dominance `2`
  - step `14` is now a guardrail rather than the first blocker:
    raw `19683`, `3` surviving roots, `12027` live generated prefixes, and a
    selector that preserves the canonical step-`15` continuation
  - step `15` still opens a raw `6561`-telescope catalog but loses `512`
    prefixes to exact partial-prefix bar failure before proof-close; the new
    local exact-prune family read now shows that captured surface as `2184`
    zero-admitted terminal families with no cached compact bounds, so the
    remaining open work there is connected terminal exposure and exact
    connectivity on the canonical temporal-shell path rather than generic
    bound accounting or claim admissibility; the new connectivity regression
    now proves all `6552` generated terminal options on that captured surface
    disconnect before admissibility
- A naive global claim band-`7` widening is now ruled out as the direct next
  reland:
  - it can lift the repaired local step-`13` read to raw `2187` /
    generated `615`
  - but it also disturbs claim prefix-memo, realistic-shadow, demo-lane, and
    divergent late-step guardrails, so the remaining open work is the residual
    exact-screen losses on the landed scoped widening plus the step-`15`
    exact-screen path
- Claim-policy honesty, fallback honesty, narrative/event completeness,
  exact-screen reason completeness, prune-class completeness, and manifest
  completeness are already earned on the stored full-profile run.

## 1. Stored Claim Evidence

- [ ] Preserve accepted-hash parity through step `15` on a stored full-profile
      bundle.
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
