# Autonomous Claim Lane Ledger

Last updated: 2026-04-08

This file is the append-only history for `desktop_claim_shadow`.

## Contract

- Append landed probes, negative controls, and major decisions here.
- Record each entry as `scope -> result -> consequence`.
- Keep live counters in [autonomous_progress.md](autonomous_progress.md).
- Keep the current work order in [autonomous_next_steps.md](autonomous_next_steps.md).

## Entry Format

- `Date`
- `Scope`
- `Result`
- `Consequence`

## 2026-04-06

- Scope: establish the next clean canonical stored bundle.
  Result: `v12` became the canonical clean stored bundle, with compare ready,
  benchmark refreshed across `v11` and `v12`, accepted-hash parity earned
  through step `15`, and certification still breadth-only at step `1`
  (`546 / 2144`) and step `15` (`4331 / 5000`).
  Consequence: `v12` is the baseline for all later claim-lane work.
- Scope: localize the real step-`15` wall on the canonical `4331` surface.
  Result: the clean partial-prefix wall was frozen at `553`, split as
  `451` remaining-two plus `102` remaining-three prefixes, with first mismatch
  pressure at clause positions `0 / 1 / 2 / 3 = 312 / 177 / 50 / 14`.
  Consequence: the remaining blocker became executable evidence rather than
  notes-only diagnosis.
- Scope: deepen localization of the dominant remaining-two wall.
  Result: the live surface was frozen as nine exact clause-`0` / clause-`1`
  pairings, then onto clause-`4` `claim_next_bridge` plus `reference`, and
  then onto clause-`2` claim variants rather than a hidden demo-only branch.
  Consequence: later probes could target named live families instead of broad
  early-wall widenings.

## 2026-04-07

- Scope: probe the `reference + demo_flat_codomain` mismatch-`1` tradeoff
  stack.
  Result: the broad control landed at `4523 / 537 / 2223` with
  `small_cluster = 3324 / 554 / 554 / 0`; the narrower clause-`4`
  `reference`-sheet tradeoff landed at `4379 / 549 / 2259`; the exact
  clause-`2` `claim_flat_domain` and `claim_sharp_codomain` sheets each landed
  at `4412 / 544 / 2250`; and the combined exact claim-pair reopening simply
  reproduced the broad `4523 / 537 / 2223` tradeoff.
  Consequence: clause-`2`-only narrowing on that branch was exhausted.
- Scope: narrow the tradeoff one layer deeper.
  Result: the exact clause-`4` `claim_next_bridge`-side probe landed at
  `4445 / 539 / 2241` with `small_cluster = 3252 / 542 / 542 / 0`, and the
  exact-claim-pair delta freeze localized the remaining off-branch miss to two
  clause-`4` `reference` remaining-three prefixes.
  Consequence: the broader tradeoff no longer hid an unlabeled residual.
- Scope: exhaust the clause-`4` `reference` remaining-three tail.
  Result: exact clause-`5` `reference` was a neutral control that left
  `4331 / 553 / 2271` unchanged; exact clause-`5`
  `claim_flat_codomain` and `claim_next_codomain` were only symmetric smaller
  tradeoff controls at `4355 / 551 / 2265` with
  `small_cluster = 3156 / 526 / 526 / 0`.
  Consequence: the clause-`4` `reference` tail was not the landed repair.
- Scope: rule out broader reopenings around the same surface.
  Result:
  - clause-`1` `demo_eventually_codomain` and clause-`0`
    `claim_flat_domain` plus clause-`1` `demo_flat_codomain` exact-pocket
    relands both lifted local breadth to `4466` but widened the wall to `626`
  - raw clause-`3` `anchor-11` widening left the wall at `553` but reopened
    `72` summary-stage incumbent captures
  - claim-safe clause-`5` side-pocket broadening widened the wall to `585`
  - claim-safe clause-`4` reopenings widened the wall to `585` per half and
    `617` combined
  - the exact remaining-two mismatch-`0` clause-`5` bridge-slice reopening
    landed only `4691 / 589`
  Consequence: the next slice had to stay narrower than clause-`4` /
  clause-`5` reopenings and narrower than raw clause-`3` widening.

## 2026-04-08

- Scope: test the exact remaining-two mismatch-`1` clause-`5` bridge slice.
  Result: the injector and connectivity override tests landed, and the local
  surface moved only to `4511 / 571 / 2325` with
  `small_cluster = 3276 / 546 / 546 / 0`; the three mismatch-`1` pairings
  widened to `48 / 48 / 67`, while the live clause-`4` split stayed fixed at
  `24 / 18`, `24 / 18`, and `33 / 28`.
  Consequence: the next slice should stay above clause-`5` bridge-side
  reopenings and work directly on the live clause-`4` families under the
  dominant mismatch-`1` pairings.
- Scope: refactor autonomous memory surfaces into non-overlapping roles.
  Result:
  - [autonomous_progress.md](autonomous_progress.md) now owns only the live
    operational snapshot
  - [autonomous_plan.md](autonomous_plan.md) now owns medium-horizon phases
  - [autonomous_next_steps.md](autonomous_next_steps.md) now owns the single
    active work order
  - [autonomous_checklist.md](autonomous_checklist.md) now owns only binary
    gates
  - [autonomous_ledger.md](autonomous_ledger.md) now owns experiment history
  Consequence: live counts should be edited in one place, and probe history
  should be appended only here.
- Scope: isolate the exact claim-pair clause-`4` `reference` side under the
  dominant mismatch-`1` `reference + demo_flat_codomain` branch.
  Result: new connectivity and search regressions pinned that narrower probe
  at `4379 / 549 / 2259` with `small_cluster = 3180 / 530 / 530 / 0`; the
  dominant branch stayed at `57` captured prefixes with the same clause-`4`
  split `33 / 24`, and the bridge-family grid matched the older broader
  clause-`4` `reference`-sheet tradeoff exactly.
  Consequence: the broader clause-`4` `reference`-sheet tradeoff already lives
  entirely on the exact claim clause-`2` pair, so exact clause-`4`
  `reference`-side relocalization is exhausted rather than the landed repair.
