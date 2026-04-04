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
- Local step-`11`, step-`12`, step-`13`, step-`14`, and narrow step-`15`
  clause-`0` / clause-`1` plus clause-`4` / clause-`5` repairs are landed,
  but no stored rerun has consumed them yet.
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
  - step `15` still opens a raw `6561`-telescope catalog, but the repaired
    canonical branch now narrows to `1794` generated prefixes on the same
    `DCT 103 / 8` continuation after paying `468` exact partial-prefix bar
    failures plus `80` incumbent-dominance prunes before proof-close; the
    captured exact-prune surface is now `1944` zero-admitted terminal
    families with no cached compact bounds, and all `5832` generated terminal
    options on that surface are structurally connected but still unqualified
    `NeedsFallback` candidates with `0` historical reanchor hits and `0`
    admissibility rejections
  - the remaining step-`15` mismatch is now localized more tightly:
    historical-reanchor prefix progress first breaks only at clause positions
    `2..3` with counts `1458` and `486`, while clause positions `0`, `1`,
    `4`, and `5` are repaired out of the captured isolated-prefix surface and
    clause `6` stays downstream of that capture boundary
  - the remaining isolated deviations are now `4` total, with exactly `2`
    claim-only variants at each clause position `2` and `3`;
    those local variants still stay blocked across every later claim suffix
    combination before the clause-`6` boundary and still leave all `3`
    terminal continuations zero-admitted `NeedsFallback`
  - the forced-reanchor recovery and winner reads now apply only to clause
    positions `2..3`: each remaining isolated local variant would recover all
    `3` terminal continuations as `KeepWithoutFallback`, admitted, and
    bar-clearing on the otherwise exact suffix if clause-local qualifier
    evidence were restored, but a direct local reanchor flip still never
    restores the canonical reference terminal clause and instead stays on the
    noncanonical `75 / 8` and `74 / 8` winner profiles, so the
    open step-`15` work is still clause-local qualifier evidence on the
    canonical branch at positions `2..3` while preserving the exact reference
    terminal continuation
  - a new exact-terminal-only isolated recovery regression now narrows that
    open step-`15` work further:
    clause positions `2` and `3` still reopen stronger local exact-terminal
    recovered profiles at `89 / 8` and `88 / 8`, so direct clause-`2` /
    clause-`3` isolated recovery remains fenced while the repaired
    clause-`4` / clause-`5` qualifier work stays on the guardrail
  - a new clause-`2` / clause-`3` pair-surface regression now narrows that
    same open step-`15` work further:
    all `8` remaining clause-`2` / clause-`3` claim-only pairings stay
    present with `243` captured prefixes each through the later suffix fan-out,
    exact-terminal-only recovery on every pairing still admits and bar-clears
    all `243` prefixes but always outranks the canonical `DCT 103 / 8`
    continuation, and forced clause-local reanchor on the mixed pair surfaces
    still admits all `729` terminal continuations while keeping only `405`
    bar clearers and splitting winners with the exact reference terminal
    winning on `162 / 243` prefixes there, so the remaining open work is not
    any direct paired clause-`2` / clause-`3` reland either
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
    `103 / 8 / 1794` surface was revalidated afterwards, so the remaining open
    work is narrower qualifier evidence on the current connected captured
    surface rather than direct early bridge-matcher expansion
- A direct paired clause-`2` / clause-`3` reland is now also ruled out as
  the direct next step-`15` fix:
  - the new pair-surface regression shows every remaining pairing still
    recovers only through stronger-than-canonical exact-terminal profiles
  - mixed pair surfaces sometimes restore the exact reference terminal under
    forced reanchor, but only on an unsafe surface that still outranks the
    canonical continuation overall
- A direct isolated exact-terminal clause-`2` / clause-`3` recovery reland
  is now also ruled out as the direct next step-`15` fix:
  - the new local regression shows those earlier isolated recoveries still
    reopen stronger local `89 / 8` and `88 / 8` rivals even without lifted
    terminal closures
  - clause positions `4` and `5` stay on canonical-primary `103 / 8`, so the
    remaining open work can stay later-clause-scoped before revisiting the
    earlier bridge slots
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
