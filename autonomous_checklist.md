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
- Local step-`11` and step-`12` repairs are landed, but no stored rerun has
  consumed them yet.
- The remaining local late blocker is now split cleanly:
  - a scoped claim-only widening plus follow-up structural-connectivity plus
    repaired-chain incumbent-relief repairs are now landed at step `13`:
    claim-open still widens to raw widths `[3,1,3,3,1,1,1]`, raw catalog `27`,
    and the guarded accepted metric shell, but the repaired live surface now
    reaches `123` generated prefixes on the same observed `step-13 -> step-15`
    continuation
  - that repaired widened step-`13` surface no longer loses prefixes to exact
    connectivity or partial-prefix bar screening:
    connectivity rejections, partial-prefix bar failures, and captured
    zero-admitted exact prunes are all now `0` on the repaired branch
  - that repaired widened step-`13` surface now also clears live
    terminal-rank pruning (`0`) on the repaired guarded step-`12` `34 / 6`
    chain while the divergent step-`13` viability-split guardrail stays
    intact, so step `13` is now a guardrail rather than the next open late
    blocker
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
    disconnect before admissibility, and the new follow-up connectivity
    classification now proves those options are all structurally connected but
    still unqualified `NeedsFallback` candidates with `0` historical reanchor
    hits; the new reanchor-prefix regression now also proves those exact-pruned
    families already fall off the temporal-shell prefix by clause `5` or
    earlier with no full seven-clause reanchor-prefix matches; the new
    clause-`6` boundary regression now also proves that once those first six
    temporal-shell clauses are fixed, the captured exact-prune surface has
    already stopped and only `3` clause-`6` variants remain outside it, with
    only the exact reference continuation exposing a
    `KeepWithoutFallback` terminal path; the new isolated-prefix regression now
    also proves each of the `12` single early claim-only deviations on the
    otherwise exact seven-clause prefix still leaves all `3` terminal
    continuations at `NeedsFallback` and zero-admitted, and the follow-up
    variant-product regression now proves each of those `12` local variants
    stays captured across its full later claim suffix fan-out too, so the open
    step-`15` work is earlier clause-local qualifier evidence on the canonical
    branch through clause `5` or earlier, not just the clause-`6` / terminal
    slot, a multi-deviation interaction, or a later-suffix interaction
- A naive global claim band-`7` widening is now ruled out as the direct next
  reland:
  - it can lift the repaired local step-`13` read to raw `2187` /
    generated `615`
  - but it also disturbs claim prefix-memo, realistic-shadow, demo-lane, and
    divergent late-step guardrails, so the remaining open work is the
    step-`15` exact-screen path rather than another blind step-`13` reland
- A direct temporal-reanchor matcher reland is also now ruled out as the
  direct next step-`15` fix:
  - a broad reland displaced the canonical step-`15` continuation to
    `60 / 8 / 9840`
  - a narrower late-shell-only reland still displaced it to `89 / 8 / 780`
  - both variants were reverted, so the remaining open work is narrower
    qualifier / reanchor evidence on the connected captured surface rather
    than a generic temporal-shell matcher expansion
- A direct early clause-`2` / clause-`3` reanchor-bridge matcher reland is
  now also ruled out as the direct next step-`15` fix:
  - a clause-`3`-only reland displaced the canonical step-`15` continuation to
    `88 / 8 / 795`
  - a clause-`2` plus clause-`3` reland displaced it further to
    `74 / 8 / 828`
  - both variants were reverted and the baseline step-`15`
    `103 / 8 / 780` surface was revalidated afterwards, so the remaining open
    work is narrower qualifier evidence on the current connected captured
    surface rather than direct early bridge-matcher expansion
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
