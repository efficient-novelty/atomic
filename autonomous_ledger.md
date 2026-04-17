# Autonomous Claim Lane Ledger

Last updated: 2026-04-17

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
- Scope: isolate the exact claim-flat and exact claim-sharp single-sheet
  sub-slices on the live clause-`4` `claim_next_bridge` half under the same
  dominant mismatch-`1` `reference + demo_flat_codomain` branch.
  Result: new connectivity and search regressions pinned both single-sheet
  probes at the same `4373 / 545 / 2259` surface with
  `small_cluster = 3180 / 530 / 530 / 0`; the dominant branch fell only to
  `51` captured prefixes with clause-`4` split `30 / 21`, and each probe only
  changed which claim clause-`2` sheet dropped to the smaller `15`-capture
  floor while the sibling claim sheet stayed at `21` and the reference sheet
  stayed at `15`.
  Consequence: the exact claim-pair clause-`4` `claim_next_bridge` half is now
  also exhausted at single-sheet scope, so the next slice should move above
  the `57 -> 51 -> 45` mismatch-`1` tradeoff ladder rather than splitting the
  claim-flat and claim-sharp clause-`2` sheets again.
- Scope: freeze the remaining-two wall outside the exhausted mismatch-`1`
  `reference + demo_flat_codomain` ladder.
  Result: new step-`15` search regressions pinned the off-branch surface at
  `390` captures across `10` pairings, split as mismatch-`0` claim-domain
  surface `252`, claim-safe mismatch-`1` surface `84`, and
  mismatch-`2` plus mismatch-`3` `reference / reference` tails `54`.
  Consequence: the next slice should prioritize mismatch-`0` claim-domain
  pairings before the smaller claim-safe mismatch-`1` tier or the
  `reference / reference` tails.
- Scope: probe the broad clause-`1` `demo_flat_codomain` reopening across the
  mismatch-`0` claim-domain surface.
  Result: new connectivity and search regressions pinned that reopening as a
  widening negative control at `4985 / 667 / 2757` with
  `small_cluster = 3564 / 594 / 594 / 0`; it added two new `45`-capture
  mismatch-`0` pairings on `claim_eventual_domain` and `claim_flat_domain`,
  each split `27 / 18` across clause-`4` `claim_next_bridge / reference` and
  evenly `15 / 15 / 15` across the two claim clause-`2` sheets plus the
  reference sheet, while leaving the isolated `single` pocket unchanged.
  Consequence: the next slice should stay above broad mismatch-`0`
  `demo_flat_codomain` reopenings and only revisit that priority tier through
  a narrower localization, or else move on to the smaller claim-safe
  mismatch-`1` surface.
- Scope: test whether broad exact-bound relief on the whole remaining-two
  mismatch-`0` claim-domain tier can safely turn the step-`15` wall into
  honest frontier breadth.
  Result: a temporary parent-level `exact_partial_prefix_bound_decision(...)`
  override redirected that whole tier from `CannotClearBar` to `Unknown`; the
  clean wall fell from `553` to `241`, which means the full `312`
  first-mismatch-`0` family escaped exact pruning, but `small_cluster`
  generated-terminal pressure jumped from `3132` to `7020`. Under the existing
  mismatch-`0` demo-flat override, the negative control also ballooned from
  `4985` to `10349`. The isolated `single` pocket stayed fenced, and the
  exploratory change was reverted immediately after the read.
  Consequence: whole-tier parent-level mismatch-`0` exact-bound relief is now
  ruled out. The next repair has to stay one layer deeper, inside the existing
  mismatch-`0` clause-`4` / clause-`5` split or the remaining-one
  exact-summary path, rather than releasing the entire tier at once.
- Scope: localize the broad mismatch-`0` `demo_flat_codomain` reopening onto
  the clause-`4` `claim_next_bridge` half.
  Result: new connectivity and search regressions pinned that narrower reland
  as a smaller negative control at `4829 / 671 / 2793` with
  `small_cluster = 3420 / 570 / 570 / 0`; the isolated `single` pocket stayed
  fenced, and the same two mismatch-`0` pairings remained live at `45 / 45`
  with the same clause-`4` split `27 / 18`, the same clause-`2` spread
  `15 / 15 / 15`, and the same `3 / 3 / 3` versus `2 / 2 / 2` bridge grid.
  Consequence: the remaining mismatch-`0` leverage is not isolated on a
  connectivity-only relocalization of the larger clause-`4`
  `claim_next_bridge` half; the next slice should move to the smaller
  clause-`4` `reference` half or to remaining-one exact-summary behavior
  under the same live pairings.
- Scope: localize that same broad mismatch-`0` `demo_flat_codomain` reopening
  onto the smaller clause-`4` `reference` half.
  Result: new connectivity and search regressions pinned that sharper reland
  as a negative control at `4697 / 691 / 2829` with
  `small_cluster = 3276 / 546 / 546 / 0`; the isolated `single` pocket stayed
  fenced, the same two mismatch-`0` pairings widened further to `57 / 57`,
  the clause-`4` split widened to `33 / 24`, and the clause-`2` spread leaned
  toward the two claim sheets at `21 / 21 / 15`.
  Consequence: the remaining mismatch-`0` leverage is not isolated on either
  clause-`4` half. The next slice should move below clause-`4`
  relocalizations and into remaining-one exact-summary behavior under the same
  live pairings before revisiting the smaller claim-safe mismatch-`1` tier.
- Scope: probe remaining-one exact-summary relief on the narrow mismatch-`0`
  clause-`4` `reference` plus clause-`5` `reference` tail.
  Result: a search-only exact-summary override pinned that deeper reland at
  `4547 / 589 / 2235` with `small_cluster = 3240 / 522 / 522 / 0`; the
  isolated `single` pocket stayed fenced, each live mismatch-`0` pairing
  widened from `42` to `52`, and the clause-`4` split leaned further toward
  `reference`, from `24 / 18` to `24 / 28`.
  Consequence: even summary-only relief on that existing reference tail
  widens the mismatch-`0` wall, so the next revisit should stay away from
  that tail and probe another live mismatch-`0` cell first.
- Scope: probe remaining-one exact-summary relief on the whole mismatch-`0`
  clause-`4` `claim_next_bridge` half.
  Result: a search-only exact-summary override pinned the first real
  wall-narrowing tradeoff on that surface at `4619 / 529 / 2199` with
  `small_cluster = 3348 / 522 / 522 / 0`; the isolated `single` pocket stayed
  fenced, each live mismatch-`0` pairing contracted from `42` to `38`, and
  the clause-`4` split contracted only the `claim_next_bridge` share, from
  `24 / 18` to `20 / 18`.
  Consequence: the next revisit should stay below the whole-half tradeoff and
  localize into the non-reference clause-`5` cells under that same half,
  because the wall improvement is real but the wider `small_cluster` shell
  still blocks a safe landing.
- Scope: split that whole mismatch-`0` clause-`4` `claim_next_bridge`-half
  tradeoff across its clause-`5` families.
  Result: the narrower clause-`5` `claim_flat_codomain` cell and clause-`5`
  `reference` cell each landed the same smaller tradeoff at
  `4475 / 541 / 2235` with `small_cluster = 3240 / 522 / 522 / 0`, each
  contracting every live mismatch-`0` pairing from `42` to `40` and reducing
  only the clause-`4` `claim_next_bridge` share from `24` to `22`; the
  sibling clause-`5` `claim_next_codomain` cell was a neutral control on the
  untouched `4331 / 553 / 2271` baseline.
  Consequence: the whole-half `4619 / 529 / 2199` tradeoff is exactly the
  union of the clause-`5` `claim_flat_codomain` and clause-`5` `reference`
  subcells under the mismatch-`0` `claim_next_bridge` half, so the next
  revisit should localize below those two active cells and ignore the inert
  `claim_next_codomain` sibling.
- Scope: localize below those two active mismatch-`0` clause-`5`
  `claim_flat_codomain / reference` cells onto individual live clause-`0` /
  clause-`1` pair cells.
  Result: all `12` pair-cell overrides landed the same smaller tradeoff at
  `4355 / 551 / 2265` with `small_cluster = 3150 / 522 / 522 / 0`; each
  probe kept the isolated `single` pocket fenced, contracted only its
  targeted live mismatch-`0` pair from `42` to `40`, and contracted only its
  targeted clause-`4` `claim_next_bridge` plus active clause-`5` bucket from
  `48` to `46`.
  Consequence: both active clause-`5` families and all six live
  clause-`0` / clause-`1` pairings are now exhausted at pair-cell scope. The
  next revisit has to move one layer deeper below a representative pair cell,
  most naturally into its clause-`2` sheet split or an equivalently fine
  remaining-one exact-summary partition, rather than spending another turn on
  equivalent pair-cell relands.
- Scope: localize below the representative mismatch-`0` pair cell onto its
  clause-`2` sheets.
  Result: under
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge / claim_flat_codomain`,
  the `claim_flat_domain` and `claim_sharp_codomain` sheets each landed the
  same smaller tradeoff at `4343 / 552 / 2268` with
  `small_cluster = 3141 / 522 / 522 / 0`, while the sibling `reference` sheet
  stayed neutral on `4331 / 553 / 2271`; each claim-side probe lowered only
  its own clause-`2` share from `15` to `14`, left the sibling claim sheet at
  `15` and the reference sheet at `12`, and contracted the representative
  clause-`4` split only from `24 / 18` to `23 / 18`.
  Consequence: the representative pair-cell `4355 / 551 / 2265` tradeoff is
  exactly the union of the two claim-side clause-`2` sheets, so the next
  revisit has to move below clause-`2` sheet identity into a finer
  remaining-one exact-summary partition under one representative claim-side
  sheet rather than swapping between the two claim sheets again.
- Scope: localize one layer deeper below the representative mismatch-`0`
  `claim_flat_domain` clause-`2` sheet onto its clause-`6` remaining-one
  exact-summary continuations.
  Result: the three clause-`6` continuations below
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain /
  claim_next_bridge / claim_flat_codomain` all landed the same matched
  smaller tradeoff shell at `4343 / 552` with
  `small_cluster = 3141 / 522 / 522 / 0`; `claim_next_codomain`,
  `claim_sharp_codomain`, and `reference` differed only in the deeper
  zero-admitted tail at `2270 / 2269 / 2268`, while the representative
  clause-`2` share stayed `14`, the representative clause-`4` split stayed
  `23 / 18`, and the active clause-`4` plus clause-`5` bucket stayed `47`.
  Consequence: clause-`6` identity is now also exhausted as full-profile
  leverage below that representative claim-flat sheet, so the next revisit
  has to move below the marginally best `reference` continuation rather than
  spending another turn swapping among the three clause-`6` siblings.
- Scope: localize one layer deeper below that representative
  `claim_flat_domain` clause-`2` sheet's marginally best clause-`6`
  `reference` continuation onto its clause-`3` branches.
  Result: the `claim_flat_argument` and `claim_eventual_argument`
  clause-`3` branches each stayed individually neutral on the untouched
  `4331 / 553 / 2271` baseline, each kept
  `small_cluster = 3132 / 522 / 522 / 0`, and each preserved first-mismatch
  counts at `312 / 177 / 50 / 14`.
  Consequence: the broader clause-`6` `reference` `4343 / 552 / 2268`
  tradeoff does not localize to either clause-`3` branch alone; it only
  appears when both clause-`3` branches reopen together, so the next revisit
  has to move below clause-`3` identity into a finer terminal or remaining-one
  completion partition under that joint continuation.
- Scope: localize that broader representative `claim_flat_domain`
  clause-`2` plus clause-`6` `reference` tradeoff one layer deeper beneath the
  joint clause-`3` continuation.
  Result: new step-`15` regressions pinned the whole delta relative to either
  individually neutral clause-`3` branch at exactly one remaining-two parent
  capture plus its three child remaining-one exact-prune prefixes:
  - the parent delta is one
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / claim_flat_codomain`
    capture
  - the zero-admitted delta is exactly the three sibling clause-`6`
    continuations below that same parent:
    `claim_next_codomain`, `claim_sharp_codomain`, and `reference`
  - no new partial-prefix captures or zero-admitted remaining-one prunes
    appear elsewhere
  Consequence: the joint clause-`3` continuation is now localized as a single
  remaining-two parent shell plus its three clause-`6` child continuations, so
  the next revisit has to partition that one parent's remaining-one completion
  / terminal subcells rather than reopening either individual clause-`3`
  branch or the whole joint continuation again.

## 2026-04-09

- Scope: partition the representative `claim_flat_domain` joint clause-`3`
  parent/child shell one layer deeper at child completion-summary scope.
  Result: beneath
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / claim_flat_codomain`,
  all six remaining-one child continuations across the two clause-`3` branches
  and the three clause-`6` labels
  `claim_next_codomain / claim_sharp_codomain / reference` now collapse to the
  same dead completion summary:
  - `3` generated terminal candidates
  - `0` admitted candidates
  - no bound, no best-rank profile, and no survivor sketch
  - the same three local terminal choices only:
    `reference`, `eventual_lift`, and `next_lift`
  - each of those three choices is still
    `KeepWithoutFallback` plus locally `open_band_structural`, but none passes
    live connectivity on the current claim path
  Consequence: the representative claim-flat parent/child shell is now
  exhausted at full completion-profile scope. The next honest mismatch-`0`
  revisit should move sideways to the representative
  `claim_sharp_codomain` clause-`2` sheet unless a narrower
  connectivity-failure reason partition under one dead claim-flat child is
  explicitly promoted.
- Scope: localize one layer deeper below the representative
  `claim_sharp_codomain` clause-`2` sheet at clause-`6` identity scope.
  Result: under
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / claim_flat_codomain`,
  the clause-`6` `claim_next_codomain`, `claim_sharp_codomain`, and
  `reference` continuations all relanded the same matched smaller tradeoff
  shell at `4343 / 552` with `small_cluster = 3141 / 522 / 522 / 0`; each
  kept the representative clause-`2` spread at `15 / 14 / 12`, the
  representative clause-`4` split at `23 / 18`, and the representative
  clause-`4` `claim_next_bridge` plus clause-`5` `claim_flat_codomain`
  bucket at `47`, differing only in the deeper zero-admitted tail at
  `2270 / 2269 / 2268`.
  Consequence: clause-`6` identity is now exhausted on the representative
  claim-sharp sheet too. Because it only reproduces the same shell already
  seen on the representative claim-flat sheet, the next honest slice should
  move off mismatch-`0` identity partitions and onto the smaller claim-safe
  mismatch-`1` tier unless a narrower reason-level connectivity partition
  beneath one frozen representative claim-side shell is explicitly promoted.
- Scope: localize the representative claim-safe mismatch-`1` clause-`4`
  `demo_sharp_codomain` side onto its two exact pair cells.
  Result: the
  `reference / claim_next_codomain` and
  `reference / claim_sharp_codomain` pair-cell relands now both land the same
  smaller negative-control shell at `4363 / 557 / 2283` with
  `small_cluster = 3156 / 526 / 526 / 0`; each keeps the isolated `single`
  pocket fenced, raises only its targeted claim-safe mismatch-`1` pairing
  from `42` to `46`, preserves that pairing's incumbent clause-`4`
  `claim_next_bridge / reference` split at `24 / 18`, and adds only a tiny
  clause-`4` `demo_sharp_codomain` side pocket of `4` while leaving the
  sibling claim-safe pairing frozen at `42`.
  Consequence: pair identity below the representative claim-safe
  `demo_sharp_codomain` side is now exhausted too. Any remaining claim-safe
  mismatch-`1` leverage is no longer on choosing between the
  `claim_next_codomain` and `claim_sharp_codomain` pairings under that side;
  the next honest slice should move below one representative claim-safe pair
  cell or only move sideways if the clause-`4` `demo_sharp_bridge` sibling is
  explicitly promoted.
- Scope: localize the representative claim-safe mismatch-`1` clause-`4`
  `demo_sharp_bridge` side onto its two exact pair cells.
  Result: the
  `reference / claim_next_codomain` and
  `reference / claim_sharp_codomain` pair-cell relands also both land the same
  smaller negative-control shell at `4363 / 557 / 2283` with
  `small_cluster = 3156 / 526 / 526 / 0`; each keeps the isolated `single`
  pocket fenced, raises only its targeted claim-safe mismatch-`1` pairing
  from `42` to `46`, preserves that pairing's incumbent clause-`4`
  `claim_next_bridge / reference` split at `24 / 18`, and adds only a tiny
  clause-`4` `demo_sharp_bridge` side pocket of `4` while leaving the sibling
  claim-safe pairing frozen at `42`.
  Consequence: clause-`4` side identity below the representative claim-safe
  exact pair scope is now exhausted too. Any remaining claim-safe mismatch-`1`
  leverage is no longer on choosing between the
  `demo_sharp_codomain` and `demo_sharp_bridge` siblings or between the two
  claim-safe pairings under them; the next honest slice should move below one
  representative exact pair cell instead of reopening another clause-`4`
  sibling or pair-cell reland.
- Scope: localize one layer deeper below the representative claim-safe
  mismatch-`1` exact pair cell
  `reference / claim_next_codomain` on the clause-`4`
  `demo_sharp_codomain` side at clause-`2` identity scope.
  Result: the `claim_flat_domain` and `claim_sharp_codomain` sheets each land
  the same smaller negative-control shell at `4347 / 555 / 2277`; each lifts
  only its own clause-`2` share from `15` to `17`, preserves the incumbent
  clause-`4` `claim_next_bridge / reference` split at `24 / 18`, adds only a
  tiny clause-`4` `demo_sharp_codomain` pocket of `2`, and lifts
  `small_cluster` generated candidates only to `3144`. The sibling
  `reference` clause-`2` sheet is a neutral control on the untouched
  `4331 / 553 / 2271` baseline with `small_cluster` generated candidates still
  at `3132` and no `demo_sharp_codomain` clause-`4` pocket.
  Consequence: clause-`2` sheet identity is now exhausted below that
  representative claim-safe exact pair cell too. The next honest claim-safe
  revisit should move below one representative claim-side sheet rather than
  swapping between the two claim sheets or reopening the reference sheet.
- Scope: explain the search/connectivity split on that representative
  claim-safe exact pair cell's `reference` clause-`2` sheet.
  Result: a hand-built reference-sheet telescope still passes connectivity
  under the exact override, but live search remains neutral there because only
  claim clause-`2` prefixes expose the anchor-`11` exact-argument pocket on
  the current lane.
  Consequence: the representative `reference` clause-`2` sheet is a control,
  not the next live search slice.
- Scope: localize one layer deeper below the representative claim-safe
  `reference / claim_next_codomain / demo_sharp_codomain` cell across the two
  claim-side clause-`2` sheets' clause-`5` / clause-`6` shells.
  Result: on both the `claim_flat_domain` and `claim_sharp_codomain` sheets,
  the clause-`5` `claim_flat_codomain` and `claim_next_codomain` prefixes each
  combine with all three clause-`6` labels
  `claim_next_codomain / claim_sharp_codomain / reference`, yielding the same
  six captured prefixes on the same `4347 / 555` outer shell. Every one of the
  six prefixes appears exactly once, sits at matched-clause count `2`, and has
  only `NeedsFallback` terminal continuations (`3` candidates each).
  Consequence: clause-`5` and clause-`6` identity are now exhausted below the
  two representative claim-side sheets too. The next honest claim-safe revisit
  should move below one representative dead prefix in that six-prefix shell,
  using the other five as matched controls rather than reopening sheet,
  clause-`5`, or clause-`6` identity.
- Scope: localize the representative claim-safe mismatch-`1`
  `claim_flat_domain` dead shell one layer deeper at terminal-family /
  completion-summary scope.
  Result: beneath
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`,
  all six dead clause-`5` / clause-`6` continuations
  `claim_flat_codomain / claim_next_codomain` x
  `claim_next_codomain / claim_sharp_codomain / reference` collapse to the
  same dead completion profile:
  - `3` generated terminal candidates
  - `0` admitted candidates
  - no bound, no best-rank profile, and no survivor sketch
  - the same three terminal families only:
    `reference`, `eventual_lift`, and `next_lift`
  - each of those three terminal families still classifies only as
    `NeedsFallback`
  - each full telescope remains structurally connected but unqualified on the
    current claim path:
    `connected = true`, `references_active_window = false`,
    `self_contained = false`, `max_lib_ref = 11`,
    `historical_reanchor = false`
  Consequence: terminal-family identity is now exhausted below that
  representative claim-safe dead shell too. The next honest claim-safe revisit
  should move below one representative terminal family, starting with the
  `reference` terminal under
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain /
  claim_flat_codomain / reference`, and partition by specific
  reason-level connectivity behavior while using the other two terminal
  families and the other five dead prefixes as matched controls.
- Scope: localize that promoted representative claim-safe dead-prefix
  `reference`-terminal reason pass one layer deeper against the exact
  claim-safe pair matcher itself.
  Result: under the selected
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`
  exact-pair override, the hand-built exact pair still reaches the full
  `8`-clause exact-pair historical-reanchor match with no mismatch, but every
  completed telescope in the six-prefix dead shell lands the same earlier
  blocker:
  - the completed `reference` terminal reaches only `5` matched clauses and
    first fails at clause `5`
  - the same `5`-matched / first-mismatch-`5` blocker also holds on the
    `eventual_lift` and `next_lift` terminals
  - the same blocker also holds across all six dead prefixes
    `claim_flat_codomain / claim_next_codomain` x
    `claim_next_codomain / claim_sharp_codomain / reference`
  Consequence: the promoted reason-level claim-safe dead-shell slice is now
  exhausted one layer deeper too. Clause-`6` and terminal-family identity are
  frozen controls, and any further claim-safe revisit has to stay below the
  clause-`5` qualification wall rather than reopening another clause-`6` or
  terminal-family reland.
- Scope: partition that representative claim-safe dead-shell clause-`5`
  qualification wall one layer deeper on the promoted `reference` terminal.
  Result: under the same selected
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`
  exact-pair override, clause-`5` `reference` is the only label that keeps
  the full exact-pair reanchor match (`8` clauses, no mismatch). All four
  off-reference clause-`5` labels
  `claim_flat_codomain / claim_next_codomain / demo_sharp_domain /
  demo_flat_codomain` stop at the same `5`-matched / first-mismatch-`5`
  blocker, fail the exact claim-safe pair, and also fail the clause-`5`
  side-pocket qualifier once clause `1` has moved to `claim_next_codomain`.
  The live dead shell itself still uses only the two claim-side dead labels
  `claim_flat_codomain / claim_next_codomain`, one copy per clause-`6`
  sibling.
  Consequence: the representative claim-safe clause-`5` wall is now sharper
  than the earlier exact anchor-`11` pocket controls. Even the demo-side
  clause-`5` variants are dead controls under the selected exact pair, so the
  next honest revisit has to stay below off-reference clause-`5` identity
  rather than retrying demo-side clause-`5` controls, clause-`6` identity, or
  terminal-family relands.
- Scope: run the first finer reason-level checkpoint below the representative
  claim-safe dead-shell clause-`5` wall.
  Result: beneath
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`,
  both live claim-side clause-`5` labels
  `claim_flat_codomain / claim_next_codomain`, all three clause-`6` siblings,
  and all three terminal families `reference / eventual_lift / next_lift`
  relanded the same finer reason vector:
  - exact-pair progress stayed fixed at `5` matched clauses with first
    mismatch at clause `5`
  - exact claim-safe pair match stayed false
  - clause-`5` side-pocket qualification stayed false
  - every completed telescope stayed structurally connected but outside
    active-window qualification, outside self-containedness, and outside
    historical reanchor, with `max_lib_ref = 11`
  - every completed telescope therefore still failed live connectivity on the
    current claim path
  Consequence: the claim-safe mismatch-`1` branch is now exhausted at its
  first finer reason-level checkpoint too. The branch should be demoted, and
  the next promoted backup should come from a mismatch-`0` reason-level
  connectivity partition beneath one frozen representative claim-side shell
  rather than from another claim-safe clause-`5` retry.
- Scope: run the first finer reason-level checkpoint below the representative
  mismatch-`0` claim-flat dead child.
  Result: beneath
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain /
  claim_next_bridge / claim_flat_codomain`,
  both clause-`3` branches `claim_flat_argument / claim_eventual_argument`,
  all three clause-`6` children
  `claim_next_codomain / claim_sharp_codomain / reference`, and all three
  terminal families `reference / eventual_lift / next_lift` relanded the same
  finer reason vector:
  - historical-reanchor progress stayed fixed at `2` matched clauses with
    first mismatch at clause `2`
  - every completed telescope stayed structurally connected but outside
    active-window qualification, outside self-containedness, and outside
    historical reanchor, with `max_lib_ref = 10`
  - every completed telescope therefore still failed live connectivity on the
    current claim path
  Consequence: the representative mismatch-`0` claim-flat dead-child backup is
  now exhausted at its first finer reason-level checkpoint too. The branch
  should be demoted, and the next promoted backup should move to the residual
  `reference / reference` tails rather than to another mismatch-`0`
  clause-`3` / clause-`6` / terminal-family or reason reland.
- Scope: localize the promoted `reference / reference` tail one layer deeper
  by mismatch position and clause-`4` anatomy.
  Result: the residual tail now splits cleanly into mismatch positions
  `2 = 42` and `3 = 12`. Within mismatch-`2`, clause-`4` stays on
  `claim_next_bridge = 18`, `reference = 16`,
  `demo_sharp_codomain = 4`, and `demo_sharp_bridge = 4`; mismatch-`3` stays
  only on `claim_next_bridge = 6` plus `reference = 6`.
  Consequence: the lumped `reference / reference` tail is no longer the live
  unit of work. The next honest slice should move below the larger
  mismatch-`2` tail first, comparing its clause-`4`
  `claim_next_bridge` and `reference` halves before the tiny demo-side
  pockets or the smaller mismatch-`3` backup.
- Scope: localize the promoted mismatch-`2` `reference / reference` tail onto
  its clause-`4` `claim_next_bridge` half.
  Result: a narrow exact-bound override on that half lands
  `4547 / 535 / 2271` with `small_cluster = 3294 / 522 / 522 / 0`; the
  isolated `single` pocket stays fenced, the exact mismatch-`2` pair
  contracts from `42` to `24`, and the smaller mismatch-`3` backup stays
  untouched at `12`.
  Consequence: the larger mismatch-`2` tail is not safely repairable on its
  whole clause-`4` `claim_next_bridge` half. The wall win is real, but it is
  only a tradeoff because the noncanonical `small_cluster` shell widens.
- Scope: localize the promoted mismatch-`2` `reference / reference` tail onto
  its clause-`4` `reference` half.
  Result: a narrow exact-bound override on that half lands
  `4835 / 529 / 2271` with `small_cluster = 3492 / 522 / 522 / 0`; the
  isolated `single` pocket stays fenced, the exact mismatch-`2` pair
  contracts from `42` to `26`, and the mismatch-`2` remaining-three spill
  also contracts by `8`, while the smaller mismatch-`3` backup stays
  untouched at `12`.
  Consequence: the mismatch-`2` wall is not safely repairable on its whole
  clause-`4` `reference` half either. Because that sharper wall win widens
  `small_cluster` even more aggressively than the sibling
  `claim_next_bridge` tradeoff, the next honest slice should move to the two
  tiny mismatch-`2` clause-`4` demo-side pockets before promoting mismatch-`3`
  or reopening broader mismatch-`0` or claim-safe shells again.
- Scope: localize the promoted mismatch-`2` `reference / reference` tail onto
  its clause-`4` `demo_sharp_codomain` pocket.
  Result: a narrow exact-bound override on that pocket lands
  `4379 / 549 / 2271` with `small_cluster = 3168 / 522 / 522 / 0`; the
  isolated `single` pocket stays fenced, the exact mismatch-`2` pair
  contracts from `42` to `38`, and the probe removes only the
  `demo_sharp_codomain` `2 / 2` clause-`5`
  `claim_flat_codomain / claim_next_codomain` cells while leaving the larger
  mismatch-`2` `claim_next_bridge` and `reference` halves, the sibling
  `demo_sharp_bridge` pocket, and the mismatch-`3` backup untouched.
  Consequence: the tiny mismatch-`2` `demo_sharp_codomain` pocket is also only
  a smaller tradeoff control. It buys a real but narrow wall win only by
  widening `small_cluster`, so it is not the landed repair.
- Scope: localize the promoted mismatch-`2` `reference / reference` tail onto
  its clause-`4` `demo_sharp_bridge` pocket.
  Result: a narrow exact-bound override on that pocket also lands
  `4379 / 549 / 2271` with `small_cluster = 3168 / 522 / 522 / 0`; the
  isolated `single` pocket stays fenced, the exact mismatch-`2` pair
  contracts from `42` to `38`, and the probe removes only the
  `demo_sharp_bridge` `2 / 2` clause-`5`
  `claim_flat_codomain / claim_next_codomain` cells while leaving the larger
  mismatch-`2` `claim_next_bridge` and `reference` halves, the sibling
  `demo_sharp_codomain` pocket, and the mismatch-`3` backup untouched.
  Consequence: the tiny mismatch-`2` `demo_sharp_bridge` pocket is the same
  matched smaller tradeoff control. With both demo-side pockets now exhausted,
  the whole mismatch-`2` clause-`4` anatomy is spent, so the next honest slice
  should move to the smaller mismatch-`3` backup before reopening broader
  mismatch-`0` or claim-safe shells again.
- Scope: localize the promoted mismatch-`3` `reference / reference` tail onto
  its clause-`4` `claim_next_bridge` half.
  Result: a narrow exact-bound override on that half lands
  `4403 / 547 / 2271` with `small_cluster = 3186 / 522 / 522 / 0`; the
  isolated `single` pocket stays fenced, the exact mismatch-`3` pair
  contracts from `12` to `6`, and the larger spent mismatch-`2` tail stays
  untouched at `42`.
  Consequence: the smaller mismatch-`3` backup is not safely repairable on
  its whole clause-`4` `claim_next_bridge` half either. The wall win is real,
  but it still depends on a wider noncanonical `small_cluster`.
- Scope: localize the promoted mismatch-`3` `reference / reference` tail onto
  its clause-`4` `reference` half.
  Result: a narrow exact-bound override on that half lands
  `4481 / 545 / 2271` with `small_cluster = 3240 / 522 / 522 / 0`; the
  isolated `single` pocket stays fenced, the exact mismatch-`3` pair also
  contracts from `12` to `6`, the mismatch-`3` remaining-three spill
  contracts by `2`, from `102` to `100`, and the larger spent mismatch-`2`
  tail stays untouched at `42`.
  Consequence: the smaller mismatch-`3` backup is not safely repairable on
  its whole clause-`4` `reference` half either. With both mismatch-`3` halves
  now exhausted as tradeoff controls, the full `reference / reference` tail is
  spent and the next honest slice has to compare alternate broader backups
  rather than reopening that tail again.
- Scope: compare the tightest remaining broader backups after the full
  `reference / reference` tail was spent.
  Result: a new explicit regression comparison kept the representative
  mismatch-`0` claim-side clause-`2` shell as the tighter backup at
  `4343 / 552 / 2268` with `small_cluster = 3141 / 522 / 522 / 0`, while the
  representative claim-safe claim-side clause-`2` shell stayed looser at
  `4347 / 555 / 2277` with `small_cluster = 3144 / 522 / 522 / 0`. The
  claim-safe shell bought only `4` extra generated prefixes by spending
  `3` extra clean-wall captures, `3` extra `small_cluster` generated
  terminals, and `9` extra zero-admitted captures; it also left the
  first-mismatch-`0` tier untouched at `312` and instead inflated
  first-mismatch-`1` from `177` to `179`, while the tighter mismatch-`0`
  shell cut first-mismatch-`0` to `311`.
  Consequence: the representative mismatch-`0` claim-side shell stays
  promoted ahead of the representative claim-safe shell. The claim-safe shell
  is now a secondary broader backup that should not compete again unless the
  tighter mismatch-`0` branch is ruled out below its remaining untouched
  representative claim-sharp continuation.
- Scope: localize the representative mismatch-`0`
  `claim_sharp_codomain` branch one layer deeper below its marginally best
  clause-`6` `reference` continuation.
  Result: beneath
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain /
  claim_next_bridge / claim_flat_codomain`, both clause-`3` branches
  `claim_flat_argument / claim_eventual_argument`, all three clause-`6`
  children `claim_next_codomain / claim_sharp_codomain / reference`, and all
  three terminal families `reference / eventual_lift / next_lift` relanded
  the same dead shell:
  - the six clause-`3` / clause-`6` child continuations all collapse to the
    same `3`-generated / `0`-admitted completion summary
  - none keeps a bound, a best-rank profile, or a survivor sketch
  - every completed telescope keeps the same
    `KeepWithoutFallback + open_band_structural` terminal trio while still
    failing live connectivity
  - every one of those `18` completed telescopes also keeps the same finer
    reason vector: historical reanchor progress `2`, first mismatch at clause
    `2`, `connected = true`, `references_active_window = false`,
    `self_contained = false`, `max_lib_ref = 10`,
    `historical_reanchor = false`, and no live connectivity
  Consequence: the representative mismatch-`0` claim-sharp dead-child backup
  is now exhausted too. The tighter representative mismatch-`0` claim-side
  broader backup is therefore spent beneath both claim-side sheets rather than
  only beneath claim-flat, so the next honest move has to stop below another
  representative claim-side dead-shell reland and switch to the first
  post-local-probe fallback decision.

## 2026-04-10

- Scope: settle the first post-local-probe fallback from stored evidence.
  Result: the stored `v11` and `v12` certificates and step summaries keep
  step `1` fixed at `546 / 2144` with the same step-`01` surface
  (`generated = 546`, `well_formed = 288`, `admitted = 1`,
  `legality_connectivity_exact_rejection = 435`), while the late stored
  surface moved materially on the same claim policy from `v11` to `v12`
  (`generated = 3972 -> 4331`,
  `partial_prefix_bar_failure = 468 -> 553`,
  `incumbent_dominance = 242 -> 3`,
  `small_cluster generated = 2190 -> 3132`). At decision time, the local
  probe stack was already far ahead of the `v12` build commit.
  Consequence: the first honest follow-on is a rerun-backed step-`15` reset
  on newer code, and a step-`1` reopening stays deferred unless that rerun
  changes the diagnosis.
- Scope: run the first post-local-probe stored rerun on newer code and refresh
  claim-lane evidence.
  Result: canonical `v13` on commit
  `ffbbd34abfbdef0abcf40e4ce8eddb259276dec6` reproduced the same breadth-only
  miss as `v12`: step `1 = 546 / 2144` with the same step-`01` surface and
  step `15 = 4331 / 5000` with the same canonical `553` partial-prefix wall,
  `3` incumbent prunes, and `small_cluster = 3132 / 522 / 522 / 0`. Compare
  stayed ready, benchmark refreshed across `v11` / `v12` / `v13`,
  certification still failed only on breadth, and runtime stayed under the
  threshold at `4387 ms`.
  Consequence: `v13` becomes the canonical stored bundle, the rerun-backed
  step-`15` reset is confirmed on newer code, and the next honest slice is no
  longer another rerun-ordering pass but a fresh code-side step-`15` repair
  while step `1` stays deferred.
- Scope: localize the sibling active mismatch-`0` clause-`5` `reference`
  family one layer deeper below its representative pair-cell clause-`2`
  split.
  Result: beneath
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge / reference`,
  the `claim_flat_domain` and `claim_sharp_codomain` sheets each reland the
  same smaller tradeoff shell at `4343 / 552 / 2268` with
  `small_cluster = 3141 / 522 / 522 / 0`, while the sibling `reference`
  sheet stays neutral on `4331 / 553 / 2271` with
  `small_cluster = 3132 / 522 / 522 / 0`. Each claim-side probe lowers only
  its own clause-`2` share from `15` to `14`, contracts the clause-`4` split
  only from `24 / 18` to `23 / 18`, and lowers the active clause-`4`
  `claim_next_bridge` plus clause-`5` `reference` bucket only from `48` to
  `47`.
  Consequence: the sibling active clause-`5` `reference` family is exhausted
  one layer deeper too, so clause-`5` family identity is no longer a fresh
  mismatch-`0` lead below representative pair-cell scope.
- Scope: test whether the lone isolated clause-`1` `demo_flat_codomain`
  exact-suffix side pocket can clear the canonical step-`15` wall without
  reopening the fenced pockets.
  Result: a search-only exact-bound override on that single isolated
  exact-suffix prefix landed `4349 / 556 / 2268` with
  `small_cluster = 3141 / 522 / 522 / 0`, accepted step `15` still
  canonical at `103 / 8`, `terminal_prefix_completion_failure = 0`, and the
  isolated `single` pocket still fenced at `3` residual prunes plus `1`
  fully scored non-winner.
  Consequence: the isolated clause-`1` exact-suffix side pocket is now spent
  as a looser side-pocket control. It only reproduces the same wider
  mismatch-`0` broader-backup shell while worsening the clean wall from
  `553` to `556`, so the next repair should stay above another isolated
  clause-`1` exact-suffix reland.
- Scope: test whether reopening both representative mismatch-`0` claim-side
  clause-`2` sheets together, but only beneath the marginally best
  clause-`6` `reference` continuation on the active
  `claim_next_bridge / claim_flat_codomain` cell, isolates a safer
  step-`15` repair.
  Result: that cross-sheet exact-summary override lands
  `4355 / 551 / 2265` with `small_cluster = 3150 / 522 / 522 / 0`.
  First-mismatch-`0` contracts from `312` to `310`, the representative
  clause-`2` spread becomes `14 / 14 / 12`, the representative clause-`4`
  split contracts from `24 / 18` to `22 / 18`, and the active clause-`4`
  `claim_next_bridge` plus clause-`5` `claim_flat_codomain` bucket contracts
  from `48` to `46`.
  Consequence: the deeper representative claim-side clause-`6`
  `reference` union is now also spent as a reconstructive control. It simply
  relands the existing `4355 / 551 / 2265` pair-cell smaller tradeoff rather
  than isolating a safer repair, so the next slice should stay above another
  claim-side union reassembly inside that same exact-summary ladder.
- Scope: determine whether that reconstructive representative mismatch-`0`
  claim-side clause-`6` `reference` union actually opens any new
  bound-carrying or live remaining-one surface beyond the older pair-cell
  tradeoff.
  Result: under the union override, the promoted exact-prune family summary
  stays exactly `captured_prefixes = 2265`, `cached_bound_count = 0`, and
  family counts `((0, None, None), 2265)`, while the promoted
  terminal-connectivity summary stays exactly `generated = 6795`,
  `NeedsFallback = 6795`, `KeepWithoutFallback = 0`, and
  `structurally_connected_but_unqualified = 6795`. Those summaries match the
  older representative pair-cell `4355 / 551 / 2265` tradeoff exactly.
  Consequence: the representative claim-side clause-`6` `reference` union is
  now closed not just as a reconstructive tradeoff control, but as proof that
  the current mismatch-`0` exact-summary ladder remains trapped inside the
  same zero-admitted remaining-one dead surface. The next repair has to move
  above that remaining-one lattice rather than another deeper reland inside
  it.
- Scope: localize the widened `small_cluster` shell inside that same
  reconstructive representative mismatch-`0` claim-side clause-`6`
  `reference` union to see whether any hidden qualifying or bound-carrying
  pocket remains below the existing pair-cell tradeoff.
  Result: the union's widened `small_cluster = 3150 / 522 / 522 / 0` shell is
  exactly `+18` generated candidates relative to the canonical `3132` shell,
  and all `18` candidates come from exactly six released remaining-one groups:
  `(claim_flat_domain, claim_next_codomain)`,
  `(claim_flat_domain, claim_sharp_codomain)`,
  `(claim_flat_domain, reference)`,
  `(claim_sharp_codomain, claim_next_codomain)`,
  `(claim_sharp_codomain, claim_sharp_codomain)`, and
  `(claim_sharp_codomain, reference)`. Every released group stays in the same
  `k8:structural_generic:temporal_operator:library_backed:small_cluster`
  bucket with `3` generated candidates, `0` admitted candidates, no bound, no
  bar-clear, `PruneDisconnected = 0`, `NeedsFallback = 3`, and
  `KeepWithoutFallback = 0`.
  Consequence: the reconstructive union's `+18` widening carries no hidden
  live or bound-carrying pocket at child scope. The next honest repair class
  has to target the parent-level route or qualification that releases those
  six groups rather than another deeper remaining-one reland inside the same
  exact-summary lattice.
- Scope: test the first parent-level route probe above that exhausted
  representative mismatch-`0` exact-summary lattice by adding a narrowly
  scoped claim-side historical-reanchor route for the representative
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_flat_codomain` shell.
  Result: the connectivity override does qualify the targeted claim-side
  parent shell while keeping the sibling reference clause-`2` sheet closed and
  keeping lifted terminals fenced, but the full step-`15` search surface lands
  an unsafe negative control:
  - accepted step `15` shifts off the canonical winner to noncanonical
    `60 / 8`
  - retained breadth becomes `4427`
  - partial-prefix bar failures contract only to `545`
  - incumbent-dominance pressure jumps from `3` to `117`
  - zero-admitted captures contract to `2247`
  - the first-mismatch-`0` tier contracts from `312` to `304`
  - `small_cluster` contracts to `2931 / 455 / 455 / 115`
  - the isolated `single` bucket reopens to `2` fully scored non-winners plus
    `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread contracts to `11 / 11 / 12`
  - the representative clause-`4` split contracts to `20 / 14`
  - the active clause-`4` `claim_next_bridge` plus clause-`5`
    `claim_flat_codomain` bucket contracts to `44`
  Consequence: the first parent-level claim-side route probe is now spent as
  an unsafe negative control. It narrows the wall only by displacing the
  canonical `103 / 8` winner and by reshaping both survivor buckets, so the
  next repair still has to stay above another reland of that same
  representative mismatch-`0` claim-side parent route.
- Scope: test the sibling active clause-`5` `reference` family's
  representative mismatch-`0` claim-side historical-reanchor route above that
  same exhausted remaining-one lattice.
  Result: the sibling route relands the same unsafe negative-control surface
  exactly:
  - accepted step `15` again shifts to noncanonical `60 / 8`
  - retained breadth again becomes `4427`
  - partial-prefix bar failures again land at `545`
  - incumbent-dominance again jumps to `117`
  - zero-admitted captures again land at `2247`
  - the first-mismatch-`0` tier again contracts to `304`
  - `small_cluster` again lands at `2931 / 455 / 455 / 115`
  - the isolated `single` bucket again reopens to `2` fully scored non-winners
    plus `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread again contracts to `11 / 11 / 12`
  - the representative clause-`4` split again contracts to `20 / 14`
  - the sibling active clause-`4` `claim_next_bridge` plus clause-`5`
    `reference` bucket contracts to `44`
  Consequence: representative mismatch-`0` claim-side parent-route identity is
  now exhausted across both active clause-`5` families. The next honest repair
  has to move to a different parent-level qualification above the current
  remaining-one lattice rather than reopening either claim-side route sibling.
- Scope: localize that matched unsafe parent-route class against the canonical
  baseline to see whether either active clause-`5` route hides any off-target
  capture or prune family above the current remaining-one lattice.
  Result: both active clause-`5` parent-route probes are now localized at the
  same narrow delta shape:
  - on the chosen active clause-`5` bucket, each route removes exactly four
    remaining-two exact-prune parent cells:
    `claim_flat_domain / claim_sharp_codomain` crossed with clause-`4`
    `claim_next_bridge / reference`
  - each removed parent cell contributes `2` zero-admitted captures, so each
    route removes exactly `8` captured prefixes in total and introduces none
  - each removed parent cell also contributes `6` remaining-one pruned
    prefixes, so each route removes exactly `24` pruned prefixes in total and
    introduces none
  - no off-target exact-prune or pruned-prefix family appears under either
    active clause-`5` route
  Consequence: the unsafe parent-route class is now exhausted at delta scope
  too. It is not a broader qualification family hiding relief elsewhere; it
  only requalifies the same targeted claim-side bucket on whichever active
  clause-`5` family is chosen. The next honest repair therefore has to change
  the parent-level qualification family itself rather than merely retrying
  route identity or another delta relocalization on those same active buckets.
- Scope: test whether narrowing that spent representative mismatch-`0`
  claim-side parent-route class to only the representative clause-`6`
  `reference` continuation on the active clause-`5`
  `claim_flat_codomain` bucket isolates a safer repair.
  Result: the narrower connectivity override still qualifies the targeted
  claim-side parent shell while keeping sibling claim clause-`6` variants, the
  reference clause-`2` sheet, and lifted terminals fenced, but the full
  step-`15` search surface still lands an unsafe negative control:
  - accepted step `15` shifts to noncanonical `74 / 8`
  - retained breadth becomes `4427` with `retained = 1`
  - partial-prefix bar failures stay at `545`
  - incumbent-dominance contracts only to `111`
  - zero-admitted captures stay at `2247`
  - the first-mismatch distribution stays `304 / 177 / 50 / 14`
  - `small_cluster` contracts further to `2904 / 430 / 430 / 108`
  - the isolated `single` bucket stays fenced at `3` residual prunes plus `1`
    fully scored non-winner at best overshoot `19563 / 10556`
  - the representative clause-`2` spread stays at `11 / 11 / 12`
  - the representative clause-`4` split stays at `20 / 14`
  - the active clause-`4` `claim_next_bridge` plus clause-`5`
    `claim_flat_codomain` bucket contracts to `44`
  Consequence: the narrower clause-`6` `reference` refinement is still unsafe.
  It preserves the targeted contraction and keeps the `single` pocket fenced,
  but it still displaces the canonical `103 / 8` winner and is not the
  missing repair.
- Scope: test the sibling active clause-`5` `reference` family's same narrow
  clause-`6` `reference` refinement above the current remaining-one lattice.
  Result: the sibling refinement relands the same unsafe negative-control
  surface exactly:
  - accepted step `15` again shifts to noncanonical `74 / 8`
  - retained breadth again becomes `4427` with `retained = 1`
  - partial-prefix bar failures again land at `545`
  - incumbent-dominance again lands at `111`
  - zero-admitted captures again land at `2247`
  - the first-mismatch distribution again stays `304 / 177 / 50 / 14`
  - `small_cluster` again lands at `2904 / 430 / 430 / 108`
  - the isolated `single` bucket again stays fenced at `3` residual prunes
    plus `1` fully scored non-winner at best overshoot `19563 / 10556`
  - the representative clause-`2` spread again stays `11 / 11 / 12`
  - the representative clause-`4` split again stays `20 / 14`
  - the sibling active clause-`4` `claim_next_bridge` plus clause-`5`
    `reference` bucket contracts to `44`
  Consequence: the narrower clause-`6` `reference` refinement is matched
  across both active clause-`5` families too, so clause-`5` identity remains
  exhausted at that parent-route depth.
- Scope: localize that narrower clause-`6` `reference` parent-route class
  against the canonical baseline to see whether it hides any off-target delta
  above the current remaining-one lattice.
  Result: both active clause-`5` narrow refinements are now localized at the
  same delta shape:
  - on the chosen active clause-`5` bucket, each refinement removes exactly
    four remaining-two exact-prune parent cells:
    `claim_flat_domain / claim_sharp_codomain` crossed with clause-`4`
    `claim_next_bridge / reference`
  - each refinement removes exactly `8` zero-admitted captures in total and
    introduces none
  - each refinement removes exactly `24` remaining-one pruned prefixes in
    total and introduces none
  - no off-target exact-prune or pruned-prefix family appears under either
    active clause-`5` refinement
  Consequence: even the marginally better narrow reference-only parent-route
  refinement is fully localized and spent. The next honest repair has to
  change the parent-level qualification family rather than retargeting the
  same parent shell with another clause-`6` reland.
- Scope: test whether narrowing that same representative mismatch-`0`
  claim-side parent-route class one layer earlier, to only one claim-side
  clause-`3` argument branch on the active clause-`5`
  `claim_flat_codomain` bucket, isolates a safer repair.
  Result: both clause-`3` branches reland the same smaller but still unsafe
  negative control exactly:
  - `claim_flat_argument` and `claim_eventual_argument` both reland
    noncanonical `60 / 8` with `retained = 2`, `generated = 4379`,
    `partial_prefix_bar_failure = 549`, `incumbent_dominance = 113`, and
    zero-admitted captures `2259`
  - both reland the same first-mismatch distribution
    `308 / 177 / 50 / 14`
  - both reland the same `small_cluster = 2871 / 435 / 435 / 111`
  - both reopen the same isolated `single` bucket to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  Consequence: representative claim-flat clause-`3` identity is now spent
  inside that same parent-route family too. The route contracts further than
  the broader `4427 / 545 / 2247` and the narrower clause-`6` `reference`
  refinement, but it still displaces the canonical `103 / 8` winner and
  reopens the `single` bucket, so the next honest repair still has to change
  parent-level qualification family rather than narrowing the same route by
  clause-`3` identity.
- Scope: test the first alternate parent-level qualification family above the
  exhausted representative mismatch-`0` remaining-one lattice by granting the
  targeted claim-side parent shell active-window qualification instead of
  historical reanchor.
  Result: on both active clause-`5` families
  `claim_flat_codomain / reference`, the active-window override does qualify
  the targeted claim-side parent shell while keeping the sibling reference
  clause-`2` sheet closed and lifted terminals fenced, but the full
  step-`15` search surface still relands an unsafe matched control:
  - accepted step `15` again shifts to noncanonical `60 / 8`
  - retained breadth again becomes `4427` with `retained = 2`
  - partial-prefix bar failures again land at `545`
  - incumbent-dominance relands at `110`
  - zero-admitted captures again land at `2247`
  - the first-mismatch distribution again stays `304 / 177 / 50 / 14`
  - `small_cluster` now relands as `2952 / 558 / 558 / 108`
  - the isolated `single` bucket again reopens to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread again stays `11 / 11 / 12`
  - the representative clause-`4` split again stays `20 / 14`
  - the active clause-`4` `claim_next_bridge` plus active clause-`5` bucket
    again contracts to `44`
  Consequence: active-window qualification is now spent too. It is not the
  missing clean repair; it keeps the same noncanonical `60 / 8` winner and
  the same `4427 / 545 / 2247` shell as the broader historical-reanchor class
  while reopening the `single` bucket, so the next honest repair still has to
  move to a different parent-level qualification family rather than swapping
  between historical-reanchor and active-window.
- Scope: localize that matched representative mismatch-`0` claim-side
  active-window class against the canonical baseline to see whether it hides
  any off-target delta above the current remaining-one lattice.
  Result: both active clause-`5` active-window probes are localized at the
  same narrow delta shape:
  - on the chosen active clause-`5` bucket, each probe removes exactly four
    remaining-two exact-prune parent cells:
    `claim_flat_domain / claim_sharp_codomain` crossed with clause-`4`
    `claim_next_bridge / reference`
  - each probe removes exactly `8` zero-admitted captures in total and
    introduces none
  - each probe removes exactly `24` remaining-one pruned prefixes in total
    and introduces none
  - no off-target exact-prune or pruned-prefix family appears under either
    active clause-`5` active-window probe
  Consequence: the active-window family is exhausted at delta scope too. It
  changes survivor pressure inside the same targeted claim-side bucket, but
  it does not reveal a broader hidden repair class above the current
  remaining-one lattice.
- Scope: test the next alternate parent-level qualification family above that
  same remaining-one lattice by granting the targeted representative
  mismatch-`0` claim-side parent shell self-contained qualification instead of
  historical reanchor or active-window qualification.
  Result: on both active clause-`5` families
  `claim_flat_codomain / reference`, the self-contained override does qualify
  the targeted claim-side parent shell while keeping the sibling reference
  clause-`2` sheet closed and lifted terminals fenced, but the full
  step-`15` search surface still relands the same unsafe matched control as
  the active-window family:
  - accepted step `15` again shifts to noncanonical `60 / 8`
  - retained breadth again becomes `4427` with `retained = 2`
  - partial-prefix bar failures again land at `545`
  - incumbent-dominance again lands at `110`
  - zero-admitted captures again land at `2247`
  - the first-mismatch distribution again stays `304 / 177 / 50 / 14`
  - `small_cluster` again relands as `2952 / 558 / 558 / 108`
  - the isolated `single` bucket again reopens to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread again stays `11 / 11 / 12`
  - the representative clause-`4` split again stays `20 / 14`
  - the active clause-`4` `claim_next_bridge` plus active clause-`5` bucket
    again contracts to `44`
  Consequence: self-contained qualification is not the missing clean repair
  either. It relands the same unsafe `4427 / 545 / 2247` shell as the
  active-window family rather than exposing a new repair class above the
  current remaining-one lattice.
- Scope: localize that matched representative mismatch-`0` claim-side
  self-contained class against the canonical baseline to see whether it hides
  any off-target delta above the current remaining-one lattice.
  Result: both active clause-`5` self-contained probes are localized at the
  same narrow delta shape as the active-window family:
  - on the chosen active clause-`5` bucket, each probe removes exactly four
    remaining-two exact-prune parent cells:
    `claim_flat_domain / claim_sharp_codomain` crossed with clause-`4`
    `claim_next_bridge / reference`
  - each probe removes exactly `8` zero-admitted captures in total and
    introduces none
  - each probe removes exactly `24` remaining-one pruned prefixes in total
    and introduces none
  - no off-target exact-prune or pruned-prefix family appears under either
    active clause-`5` self-contained probe
  Consequence: the self-contained family is exhausted at delta scope too. The
  next honest repair has to change parent-level qualification family again
  rather than swapping between historical-reanchor, active-window, and
  self-contained controls.
- Scope: narrow the alternate active-window qualification family one layer
  deeper, to only the representative clause-`6` `reference` continuation on
  each active clause-`5` bucket above the current remaining-one lattice.
  Result: the narrower active-window refinements are still unsafe split
  controls rather than clean repairs:
  - on `claim_flat_codomain`, the refinement relands noncanonical `60 / 8`
    with `retained = 4`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 113`, and
    zero-admitted captures `2247`
  - on `reference`, the sibling refinement relands noncanonical `60 / 8`
    with `retained = 2`, the same `4427 / 545 / 2247`, and
    `incumbent_dominance = 115`
  - both clause-`5` siblings keep first-mismatch distribution
    `304 / 177 / 50 / 14`, clause-`2` spread `11 / 11 / 12`, clause-`4`
    split `20 / 14`, and the chosen active clause-`4`
    `claim_next_bridge` plus clause-`5` bucket at `44`
  - both also reland `small_cluster = 2904 / 462 / 462 / 109 / 2` with best
    overshoot `545 / 5278` and no `single` bucket
  - both keep the same targeted four remaining-two parent cells plus the same
    `24` remaining-one pruned prefixes, with no off-target capture or prune
    family introduced
  Consequence: narrowing active-window to clause-`6` `reference` does not
  expose a safe repair class. It improves the bucket shape relative to the
  broader `2952 / 558 / 558 / 108` shell, but it still displaces the
  canonical `103 / 8` winner and only repartitions survivor pressure inside
  the same targeted claim-side bucket.
- Scope: narrow the alternate self-contained qualification family one layer
  deeper, to only the representative clause-`6` `reference` continuation on
  each active clause-`5` bucket above the current remaining-one lattice.
  Result: the narrower self-contained refinements reland the exact same unsafe
  clause-`5` split control as the active-window refinements:
  - `claim_flat_codomain` again lands noncanonical `60 / 8` with
    `retained = 4`, `generated = 4427`, `partial_prefix_bar_failure = 545`,
    `incumbent_dominance = 113`, zero-admitted captures `2247`, and
    `small_cluster = 2904 / 462 / 462 / 109 / 2`
  - `reference` again lands noncanonical `60 / 8` with `retained = 2`, the
    same `4427 / 545 / 2247`, `incumbent_dominance = 115`, and the same
    `2904 / 462 / 462 / 109 / 2` `small_cluster`
  - both siblings keep best overshoot `545 / 5278`, no `single` bucket, the
    same `304 / 177 / 50 / 14` first-mismatch distribution, the same
    `11 / 11 / 12` clause-`2` spread, the same `20 / 14` clause-`4` split,
    the same chosen active clause-`4` plus clause-`5` bucket at `44`, and
    the same four-cell plus `24`-pruned-prefix targeted delta
  Consequence: self-contained clause-`6` narrowing is now spent too. It is not
  a distinct recovery class hiding below the broad self-contained family;
  below that narrower selector it still just relands the active-window split
  control. The next honest repair therefore has to stay above both alternate
  clause-`6` refinements as well as above their broader parent families.
- Scope: narrow those same alternate active-window and self-contained
  qualification families one layer earlier, to only one representative
  claim-flat clause-`3` argument branch on the active clause-`5`
  `claim_flat_codomain` bucket above the current remaining-one lattice.
  Result: both alternate families reland the same smaller but still unsafe
  split control on either `claim_flat_argument` or `claim_eventual_argument`:
  - both families reland noncanonical `60 / 8` with `retained = 2`,
    `generated = 4379`, `partial_prefix_bar_failure = 549`,
    `incumbent_dominance = 110`, and zero-admitted captures `2259`
  - both reland the same first-mismatch distribution
    `308 / 177 / 50 / 14`
  - both reland the same
    `small_cluster = 2880 / 486 / 486 / 108`
  - both reopen the same isolated `single` bucket to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - on the chosen active clause-`5` bucket, both alternate clause-`3`
    refinements remove the same four targeted remaining-two exact-prune parent
    cells, but now only one captured prefix per cell and three remaining-one
    pruned prefixes per cell, for a narrowed targeted delta of `4`
    zero-admitted captures plus `12` pruned prefixes and no off-target family
    introduced
  Consequence: alternate-family clause-`3` narrowing is now spent too.
  Active-window and self-contained stay matched even under clause-`3`
  identity, and this smaller `4379 / 549 / 2259` split control still
  displaces the canonical `103 / 8` winner, so the next honest repair has to
  stay above those narrower alternate clause-`3` refinements as well as above
  their broader and clause-`6` siblings.
- Scope: test the first recombined parent-level qualification family above
  that same remaining-one lattice by granting the targeted representative
  mismatch-`0` claim-side parent shell both historical-reanchor and
  active-window qualification at once.
  Result: on both active clause-`5` families
  `claim_flat_codomain / reference`, the recombined historical-reanchor plus
  active-window override still qualifies the targeted claim-side parent shell
  while keeping lifted terminals fenced, but the full step-`15` search
  surface simply relands the exact same unsafe shell as the already-spent
  active-window family:
  - accepted step `15` again shifts to noncanonical `60 / 8`
  - retained breadth again becomes `4427` with `retained = 2`
  - partial-prefix bar failures again land at `545`
  - incumbent-dominance again lands at `110`
  - zero-admitted captures again land at `2247`
  - the first-mismatch distribution again stays `304 / 177 / 50 / 14`
  - `small_cluster` again relands as `2952 / 558 / 558 / 108`
  - the isolated `single` bucket again reopens to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread again stays `11 / 11 / 12`
  - the representative clause-`4` split again stays `20 / 14`
  - the active clause-`4` `claim_next_bridge` plus active clause-`5` bucket
    again contracts to `44`
  Consequence: adding historical reanchor on top of the already-spent
  active-window family does not create a fresh parent-level repair class
  either. The next honest repair therefore has to stay above that recombined
  hybrid rather than recombining already-spent parent-route and active-window
  controls.
- Scope: test the sibling recombined parent-level qualification family above
  that same remaining-one lattice by granting the targeted representative
  mismatch-`0` claim-side parent shell both historical-reanchor and
  self-contained qualification at once.
  Result: on both active clause-`5` families
  `claim_flat_codomain / reference`, the recombined historical-reanchor plus
  self-contained override still qualifies the targeted claim-side parent shell
  while keeping lifted terminals fenced, but the full step-`15` search
  surface simply relands the exact same unsafe shell as the already-spent
  self-contained family:
  - accepted step `15` again shifts to noncanonical `60 / 8`
  - retained breadth again becomes `4427` with `retained = 2`
  - partial-prefix bar failures again land at `545`
  - incumbent-dominance again lands at `110`
  - zero-admitted captures again land at `2247`
  - the first-mismatch distribution again stays `304 / 177 / 50 / 14`
  - `small_cluster` again relands as `2952 / 558 / 558 / 108`
  - the isolated `single` bucket again reopens to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread again stays `11 / 11 / 12`
  - the representative clause-`4` split again stays `20 / 14`
  - the active clause-`4` `claim_next_bridge` plus active clause-`5` bucket
    again contracts to `44`
  Consequence: adding historical reanchor on top of the already-spent
  self-contained family does not create a fresh parent-level repair class
  either. The next honest repair therefore has to stay above both recombined
  hybrids and move to a looser recombined parent-level qualification family
  rather than reopening narrower clause-`3` / clause-`6` selectors or
  reassembling already-spent parent-route plus self-contained controls.
- Scope: test the first looser recombined parent-level qualification family
  above that same remaining-one lattice by granting the targeted
  representative mismatch-`0` claim-side parent shell both active-window and
  self-contained qualification at once.
  Result: on both active clause-`5` families
  `claim_flat_codomain / reference`, the looser recombined active-window plus
  self-contained override still qualifies the targeted claim-side parent shell
  while keeping lifted terminals fenced, but the full step-`15` search
  surface simply relands the exact same unsafe shell as the already-spent
  active-window and self-contained families:
  - accepted step `15` again shifts to noncanonical `60 / 8`
  - retained breadth again becomes `4427` with `retained = 2`
  - partial-prefix bar failures again land at `545`
  - incumbent-dominance again lands at `110`
  - zero-admitted captures again land at `2247`
  - the first-mismatch distribution again stays `304 / 177 / 50 / 14`
  - `small_cluster` again relands as `2952 / 558 / 558 / 108`
  - the isolated `single` bucket again reopens to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread again stays `11 / 11 / 12`
  - the representative clause-`4` split again stays `20 / 14`
  - the active clause-`4` `claim_next_bridge` plus active clause-`5` bucket
    again contracts to `44`
  Consequence: combining active-window and self-contained does not create a
  fresh parent-level repair class either. The next honest repair therefore
  has to stay above that first looser recombined family too and move to the
  next looser recombined parent-level qualification family rather than
  reopening narrower clause-`3` / clause-`6` selectors or restating
  already-spent hybrids.
- Scope: localize that first looser recombined active-window plus
  self-contained family against the canonical baseline to see whether it hides
  any off-target delta above the current remaining-one lattice.
  Result: both active clause-`5` looser recombined probes are localized at
  the same narrow delta shape:
  - on the chosen active clause-`5` bucket, each probe removes exactly four
    remaining-two exact-prune parent cells:
    `claim_flat_domain / claim_sharp_codomain` crossed with clause-`4`
    `claim_next_bridge / reference`
  - each probe removes exactly `8` zero-admitted captures in total and
    introduces none
  - each probe removes exactly `24` remaining-one pruned prefixes in total
    and introduces none
  - no off-target exact-prune or pruned-prefix family appears under either
    active clause-`5` looser recombined probe
  Consequence: the first looser recombined family is exhausted at delta scope
  too. It relands the same unsafe shell and the same targeted four-cell plus
  `24`-pruned-prefix delta as the already-spent alternate families, so the
  next honest repair has to move above it rather than re-running the same
  recombination on either active clause-`5` bucket.
- Scope: test the next looser recombined parent-level qualification family
  above that same remaining-one lattice by granting the targeted
  representative mismatch-`0` claim-side parent shell historical-reanchor,
  active-window, and self-contained qualification all at once.
  Result: on both active clause-`5` families
  `claim_flat_codomain / reference`, the full qualification triad still
  qualifies the targeted claim-side parent shell while keeping lifted
  terminals fenced, but the full step-`15` search surface again relands the
  exact same unsafe shell as the already-spent active-window and
  self-contained families:
  - accepted step `15` again shifts to noncanonical `60 / 8`
  - retained breadth again becomes `4427` with `retained = 2`
  - partial-prefix bar failures again land at `545`
  - incumbent-dominance again lands at `110`
  - zero-admitted captures again land at `2247`
  - the first-mismatch distribution again stays `304 / 177 / 50 / 14`
  - `small_cluster` again relands as `2952 / 558 / 558 / 108`
  - the isolated `single` bucket again reopens to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread again stays `11 / 11 / 12`
  - the representative clause-`4` split again stays `20 / 14`
  - the active clause-`4` `claim_next_bridge` plus active clause-`5` bucket
    again contracts to `44`
  Consequence: exhausting historical-reanchor, active-window, and
  self-contained together does not create a fresh parent-level repair class
  either. The next honest repair therefore has to move above the full
  parent-level qualification-family lattice rather than reopening another
  recombination of the same three qualifiers.
- Scope: localize that full parent-route plus active-window plus
  self-contained qualification triad against the canonical baseline to see
  whether it hides any off-target delta above the current remaining-one
  lattice.
  Result: both active clause-`5` full-triad probes are localized at the same
  narrow delta shape:
  - on the chosen active clause-`5` bucket, each probe removes exactly four
    remaining-two exact-prune parent cells:
    `claim_flat_domain / claim_sharp_codomain` crossed with clause-`4`
    `claim_next_bridge / reference`
  - each probe removes exactly `8` zero-admitted captures in total and
    introduces none
  - each probe removes exactly `24` remaining-one pruned prefixes in total
    and introduces none
  - no off-target exact-prune or pruned-prefix family appears under either
    active clause-`5` full-triad probe
  Consequence: the full qualification triad is exhausted at delta scope too.
  It relands the same unsafe shell and the same targeted four-cell plus
  `24`-pruned-prefix delta as every other spent parent-level qualification
  family on this representative shell, so the next honest repair has to move
  above the entire qualification-family lattice rather than retrying another
  recombination on either active clause-`5` bucket.
- Scope: test the first clause-`4` split above that exhausted parent-level
  qualification-family lattice by narrowing the representative mismatch-`0`
  claim-side parent-route shell to the clause-`4` `reference` branch on each
  active clause-`5` bucket.
  Result: the narrow clause-`4` `reference` route split does qualify the
  targeted claim-side parent shell while keeping the sibling
  `claim_next_bridge` branch, the sibling reference clause-`2` sheet, and
  lifted terminals fenced, but on both active clause-`5` families it still
  relands a matched unsafe shell:
  - both `claim_flat_codomain` and `reference` reland noncanonical `60 / 8`
    with `retained = 2`, `generated = 4391`,
    `partial_prefix_bar_failure = 557`, `incumbent_dominance = 113`, and
    zero-admitted captures `2259`
  - both reland the same first-mismatch distribution
    `316 / 177 / 50 / 14`
  - both reland the same
    `small_cluster = 2871 / 435 / 435 / 111`
  - both reopen the same isolated `single` bucket to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - both shift the representative clause-`2` spread to `19 / 19 / 12`
  - both shift the representative clause-`4` split to `24 / 26`
  - each contracts only the chosen clause-`4` `reference` plus active
    clause-`5` bucket to `36`
  - on the chosen active clause-`5` bucket, each probe removes only the two
    remaining-three clause-`4` `reference` parent captures, removes `4`
    zero-admitted captures plus `12` remaining-one pruned prefixes in total,
    introduces four targeted remaining-two clause-`4` `reference` capture
    families, and introduces no pruned-prefix family
  Consequence: even above the exhausted parent-level qualification-family
  lattice, that clause-`4` `reference` route split is not a fresh clean
  repair class. It only converts remaining-three pressure into a narrower
  remaining-two clause-`4` `reference` pocket while still displacing the
  canonical `103 / 8` winner and reopening the `single` bucket, so the next
  honest probe has to move to a different code-side repair class again
  rather than retrying this route split on either active clause-`5` bucket.
- Scope: test the sibling clause-`4` `reference` split above that exhausted
  parent-level qualification-family lattice by narrowing the representative
  mismatch-`0` claim-side active-window and self-contained families to the
  clause-`4` `reference` branch on each active clause-`5` bucket.
  Result: the alternate clause-`4` `reference` splits do qualify the targeted
  claim-side parent shell while keeping the sibling `claim_next_bridge`
  branch, the sibling reference clause-`2` sheet, and lifted terminals
  fenced, but on both active clause-`5` families and on both alternate
  qualification families they still reland the same matched smaller unsafe
  shell:
  - accepted step `15` again shifts to noncanonical `60 / 8`
  - retained breadth again becomes `4391` with `retained = 2`
  - partial-prefix bar failures again land at `557`
  - incumbent-dominance again lands at `110`
  - zero-admitted captures again land at `2271`
  - the first-mismatch distribution again stays `312 / 177 / 50 / 14`
  - `small_cluster` again relands as `2880 / 486 / 486 / 108`
  - the isolated `single` bucket again reopens to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread again stays `15 / 15 / 12`
  - the representative clause-`4` split again stays `24 / 18`
  - the chosen clause-`4` `reference` plus active clause-`5` bucket again
    contracts to `36`
  - on the chosen active clause-`5` bucket, each probe removes only the two
    remaining-three clause-`4` `reference` parent captures, removes `4`
    zero-admitted captures plus `12` remaining-one pruned prefixes in total,
    introduces four targeted remaining-two clause-`4` `reference` capture
    families, and introduces no pruned-prefix family
  Consequence: the alternate active-window and self-contained clause-`4`
  `reference` splits are now spent too. They stay matched across both
  alternate families and both active clause-`5` buckets, only reconstructing
  a smaller unsafe split control rather than a clean repair, so the next
  honest probe has to move above those alternate clause-`4` splits too
  rather than cycling among clause-`4` families or dropping back to narrower
  clause-`3` / clause-`6` selectors.
- Scope: test the sibling clause-`4` `claim_next_bridge` split above that
  exhausted parent-level qualification-family lattice by narrowing the
  representative mismatch-`0` claim-side parent-route shell to the
  clause-`4` `claim_next_bridge` branch on each active clause-`5` bucket.
  Result: on both active clause-`5` families it relands a broader unsafe
  negative control:
  - both `claim_flat_codomain` and `reference` reland noncanonical `60 / 8`
    with `retained = 2`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 117`, and
    zero-admitted captures `2271`
  - both keep the same first-mismatch distribution
    `312 / 177 / 50 / 14`
  - both reland the same
    `small_cluster = 2931 / 455 / 455 / 115`
  - both reopen the same isolated `single` bucket to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - both keep the representative clause-`2` spread at `15 / 15 / 12`
  - both keep the representative clause-`4` split at `24 / 18`
  - each keeps the chosen clause-`4` `claim_next_bridge` plus active
    clause-`5` bucket at `48`
  - on the chosen active clause-`5` bucket, each probe removes the same four
    targeted remaining-two parent cells, removes `8` zero-admitted captures
    plus `24` remaining-one pruned prefixes in total, and introduces no
    capture or pruned-prefix family
  Consequence: the parent-route clause-`4` `claim_next_bridge` split is not a
  fresh clean repair class either. It preserves the unsafe `4427 / 545`
  shell and the same four-cell plus `24`-pruned-prefix delta while failing
  to contract clause-`2` or clause-`4` pressure, so the next honest probe
  has to stay above both clause-`4` branch splits on that parent-route shell.
- Scope: test the matched clause-`4` `claim_next_bridge` split above that
  exhausted parent-level qualification-family lattice by narrowing the
  representative mismatch-`0` claim-side active-window and self-contained
  families to the clause-`4` `claim_next_bridge` branch on each active
  clause-`5` bucket.
  Result: on both active clause-`5` families and on both alternate
  qualification families they reland the same broader unsafe control:
  - accepted step `15` again shifts to noncanonical `60 / 8`
  - retained breadth again becomes `4427` with `retained = 2`
  - partial-prefix bar failures again land at `545`
  - incumbent-dominance again lands at `110`
  - zero-admitted captures again land at `2271`
  - the first-mismatch distribution again stays `312 / 177 / 50 / 14`
  - `small_cluster` again relands as `2952 / 558 / 558 / 108`
  - the isolated `single` bucket again reopens to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread again stays `15 / 15 / 12`
  - the representative clause-`4` split again stays `24 / 18`
  - the chosen clause-`4` `claim_next_bridge` plus active clause-`5` bucket
    again stays at `48`
  - on the chosen active clause-`5` bucket, each probe removes the same four
    targeted remaining-two parent cells, removes `8` zero-admitted captures
    plus `24` remaining-one pruned prefixes in total, and introduces no
    capture or pruned-prefix family
  Consequence: the alternate active-window and self-contained clause-`4`
  `claim_next_bridge` splits are now spent too. They stay matched across
  both alternate families and both active clause-`5` buckets, only relanding
  broader unsafe controls rather than a clean repair, so the next honest
  probe has to move above both clause-`4` branch splits across the
  parent-route and alternate families rather than cycling among
  clause-`4` families or dropping back to narrower clause-`3` / clause-`6`
  selectors.

## 2026-04-11

- Scope: test the representative mismatch-`0` claim-side clause-`2` sheet
  splits above the exhausted parent-level qualification-family lattice and
  above both clause-`4` branch splits by narrowing the parent-route shell to
  each clause-`2` sheet on both active clause-`5` buckets.
  Result: on both active clause-`5` families, the
  `claim_flat_domain` and `claim_sharp_codomain` clause-`2` sheets each
  reland unsafe noncanonical `60 / 8` with `retained = 1`,
  `generated = 4379`, `partial_prefix_bar_failure = 549`, and
  `small_cluster = 2871 / 435 / 435 / 111`, while the sibling
  clause-`2` `reference` sheet relands unsafe noncanonical `74 / 8` with
  `retained = 1`, the same `4379 / 549`, and
  `small_cluster = 2844 / 426 / 426 / 120`. Each split keeps the `single`
  pocket fenced and localizes its delta to only two targeted remaining-two
  parent cells plus `12` removed remaining-one pruned prefixes on the chosen
  active clause-`5` bucket, with no off-target family introduced.
  Consequence: clause-`2` identity on the representative parent-route shell
  is now spent too. It only relands unsafe controls rather than a fresh clean
  repair, so the next honest probe has to stay above those clause-`2`
  selectors.
- Scope: test the sibling clause-`2` sheet splits above that same
  clause-`4` / parent-qualification lattice inside the alternate
  active-window and self-contained qualification families on both active
  clause-`5` buckets.
  Result: on either alternate family, the `claim_flat_domain` and
  `claim_sharp_codomain` clause-`2` sheets reland the same smaller unsafe
  `60 / 8` with `retained = 1`, `generated = 4379`,
  `partial_prefix_bar_failure = 549`, and
  `small_cluster = 2880 / 486 / 486 / 108`, while the sibling
  clause-`2` `reference` sheet relands a different unsafe matched control
  with noncanonical `60 / 8`, `retained = 2`, the same `4379 / 549`, and
  `small_cluster = 2844 / 468 / 468 / 127`. Each split removes the same two
  targeted remaining-two parent cells plus `12` removed remaining-one pruned
  prefixes on the chosen active clause-`5` bucket and introduces no
  off-target family.
  Consequence: the alternate active-window and self-contained clause-`2`
  sheet splits are spent too. They stay matched across both alternate
  families and both active clause-`5` buckets, so the next honest probe has
  to move above those clause-`2` selectors as well rather than reopening
  another narrower split inside the same representative parent shell.
- Scope: decide the next honest repair class after exhausting the
  representative mismatch-`0` claim-side clause-`2` sheet splits above the
  clause-`4` and parent-level qualification lattices.
  Result: the parent-route, active-window, and self-contained clause-`2`
  selectors all reland unsafe controls rather than the canonical
  `103 / 8` winner.
  Consequence: the active slice must now move to a different code-side repair
  class above both clause-`4` branch splits and above these clause-`2` sheet
  splits instead of reopening another selector inside the same parent shell.
- Scope: test the first broader exact-screen follow-on above that spent
  connectivity lattice by relaxing remaining-two
  `exact_partial_prefix_bound_decision(...)` on the whole mismatch-`0`
  clause-`4` `claim_next_bridge` half.
  Result: the broader exact-screen release keeps accepted step `15`
  canonical at `103 / 8` and keeps the isolated `single` pocket fenced, but
  it still lands only a broader tradeoff control:
  - generated breadth rises to `4763`
  - partial-prefix bar failures contract to `517`
  - `incumbent_dominance` stays `3`
  - zero-admitted captures stay `2271`
  - `small_cluster` widens to `3456 / 522 / 522 / 0`
  - every live mismatch-`0` clause-`0` / clause-`1` pair contracts from `42`
    to `36`
  - both clause-`4` branches equalize at `18` per live pair
  - all six live clause-`4` / clause-`5` cells equalize at `36`
  Consequence: broad remaining-two exact partial-prefix relief on the whole
  mismatch-`0` `claim_next_bridge` half is spent too. It narrows the wall
  only by converting more of the same zero-admitted shell into generated
  survivor mass, so the next honest probe has to stay below that broad
  release and split the remaining-two exact-screen class by active
  clause-`5` cell or another narrower remaining-two selector rather than
  reopening connectivity or deeper remaining-one relands.
- Scope: split that broader remaining-two exact-screen release one layer
  deeper across the mismatch-`0` clause-`4` `claim_next_bridge` clause-`5`
  cells.
  Result: the `claim_flat_codomain`, `claim_next_codomain`, and `reference`
  clause-`5` cells each land the same smaller tradeoff control:
  - generated breadth rises to `4475`
  - partial-prefix bar failures contract to `541`
  - `incumbent_dominance` stays `3`
  - zero-admitted captures stay `2271`
  - `small_cluster` widens to `3240 / 522 / 522 / 0`
  - every live mismatch-`0` clause-`0` / clause-`1` pair contracts from `42`
    to `40`
  - the clause-`4` split contracts only from `24 / 18` to `22 / 18`
  - each probe localizes to only its chosen `claim_next_bridge`
    clause-`5` family, leaving the two sibling `claim_next_bridge` cells at
    `48` and the whole `reference` half at `36`
  Consequence: clause-`5` identity on the remaining-two exact-screen path is
  spent too. The broader `4763 / 517 / 2271` release is exactly the union of
  these three symmetric smaller tradeoffs, and the previously inert
  `claim_next_codomain` cell is now live on the remaining-two exact-screen
  path. The next honest probe has to move below that spent clause-`5` lattice
  to a narrower remaining-two selector, most naturally a representative
  clause-`0` / clause-`1` pair inside one chosen exact-screen cell, rather
  than reopening connectivity or deeper remaining-one relands.
- Scope: split the newly live mismatch-`0` clause-`4` `claim_next_bridge`
  plus clause-`5` `claim_next_codomain` exact-screen cell one layer deeper
  across its clause-`0` / clause-`1` pairs.
  Result: all six live pair probes reland the same smaller exact-screen
  tradeoff:
  - generated breadth rises to `4355`
  - partial-prefix bar failures contract to `551`
  - `incumbent_dominance` stays `3`
  - zero-admitted captures stay fixed at `2271`
  - `small_cluster` widens only to `3150 / 522 / 522 / 0`
  - each probe keeps accepted step `15` canonical `103 / 8` and the isolated
    `single` pocket fenced
  - each probe contracts only its targeted live pair from `42` to `40`
  - each probe contracts only its targeted clause-`4`
    `claim_next_bridge` share from `24` to `22` while leaving the sibling
    `reference` share at `18`
  - each probe contracts only the targeted clause-`4` `claim_next_bridge`
    plus clause-`5` `claim_next_codomain` bucket from `48` to `46`
  Consequence: clause-`0` / clause-`1` identity inside that exact-screen
  `claim_next_codomain` cell is now spent too. It only reconstructs a
  uniform smaller exact-screen tradeoff above the same zero-admitted shell,
  so the next honest probe has to move below that exact-screen pair lattice
  rather than reopening sibling exact-screen pairs.
- Scope: localize one representative exact-screen pair inside that spent
  `claim_next_codomain` cell before dropping into deeper remaining-one
  relands.
  Result: the representative
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_next_codomain` pair lands the same `4355 / 551 / 2271` tradeoff, the
  same `3150 / 522 / 522 / 0` `small_cluster`, remaining-clause-slot counts
  `449 / 102`, first-mismatch counts `310 / 177 / 50 / 14`, and the same
  targeted `40`-capture pair plus `22 / 18` clause-`4` split.
  Consequence: the next honest probe has to stay below the exact-screen
  pair-cell lattice and localize a representative claim-side clause-`2`
  sheet inside that same exact-screen pair rather than re-running pair-cell
  identity.
- Scope: split the representative exact-screen pair cell
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_next_codomain` one layer deeper across its clause-`2` sheets.
  Result: the two claim-side sheets are matching smaller tradeoff controls
  while the sibling `reference` sheet is neutral:
  - `claim_flat_domain` lands `4343 / 552 / 2271`, keeps accepted step `15`
    canonical `103 / 8`, keeps `incumbent_dominance = 3`, shifts
    first-mismatch counts only to `311 / 177 / 50 / 14`, widens
    `small_cluster` only to generated `3141`, shifts the representative
    clause-`2` spread to `14 / 15 / 12`, contracts the representative
    clause-`4` split only to `23 / 18`, and contracts the representative
    `claim_next_bridge` plus clause-`5` `claim_next_codomain` bucket only to
    `47`
  - `claim_sharp_codomain` lands the same `4343 / 552 / 2271` tradeoff with
    the same fences, the same `311 / 177 / 50 / 14`, the same
    `small_cluster generated = 3141`, the mirrored representative
    clause-`2` spread `15 / 14 / 12`, the same `23 / 18` clause-`4` split,
    and the same `47` active clause-`5` bucket
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with first-mismatch counts
    `312 / 177 / 50 / 14` and `small_cluster generated = 3132`
  Consequence: clause-`2` identity inside that representative exact-screen
  pair cell is spent too. The pair-cell tradeoff is exactly the union of the
  two claim-side sheets while the sibling `reference` sheet stays inert, so
  the next honest probe has to stay below this clause-`2` split and localize
  a representative clause-`6` continuation inside the `claim_flat_domain`
  sheet rather than reopening the sibling claim-side sheet, the neutral
  `reference` sheet, sibling exact-screen pairs, or deeper remaining-one
  relands.
- Scope: split the representative exact-screen `claim_flat_domain` sheet
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain /
  claim_next_bridge / claim_next_codomain` one layer deeper across its
  clause-`6` continuations.
  Result: the attempted clause-`6` split does not exist at this layer. The
  representative claim-flat clause-`2` tradeoff removes exactly one
  remaining-two exact-prune capture on
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain /
  claim_next_bridge / claim_next_codomain`, that removed capture still has
  only six fixed clauses, and clause `6` is therefore still out of scope at
  the exact-screen boundary. The tradeoff introduces no new exact-prune
  capture family and removes no pruned-terminal family either.
  Consequence: same-layer exact-screen clause-`6` identity is unavailable on
  that representative claim-flat sheet. The next honest probe has to move
  below the remaining-two exact-screen boundary and localize the single
  released six-clause parent at remaining-one scope rather than retrying
  another clause-`6` split, reopening the sibling claim-sharp sheet, or
  bouncing back to sibling exact-screen pairs.
- Scope: test the first remaining-one follow-on beneath that released
  exact-screen representative `claim_flat_domain` plus clause-`5`
  `claim_next_codomain` parent by stacking remaining-one exact-summary relief
  under the same exact-screen representative claim-flat tradeoff.
  Result: the stacked follow-on is a neutral control rather than a new
  tradeoff.
  - the full step-`15` surface stays on the same `4343 / 552 / 2271`
  - it keeps accepted step `15` canonical `103 / 8`
  - it keeps the same `311 / 177 / 50 / 14` first-mismatch distribution
  - it keeps the same `small_cluster = 3141 / 522 / 522 / 0`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)`
  - it keeps the same remaining-one terminal connectivity summary with
    `6813` structurally connected but unqualified generated candidates
  - it releases no remaining-one pruned-terminal prefixes and introduces no
    new remaining-one pruned-terminal group either
  Consequence: remaining-one exact-summary relief is not the missing lever on
  that released exact-screen representative claim-flat parent either. The
  next honest probe has to move below summary-prune relief and localize the
  same six-clause parent at direct remaining-one completion or terminal scope
  rather than retrying the same summary override, reopening the sibling
  claim-sharp sheet, or bouncing back to sibling exact-screen pairs.
- Scope: localize that same released exact-screen representative
  `claim_flat_domain` plus clause-`5` `claim_next_codomain` parent one layer
  deeper at direct remaining-one completion and terminal scope by splitting
  its clause-`6` continuations.
  Result: all three clause-`6` continuations reland the same dead shell:
  - `claim_next_codomain`, `claim_sharp_codomain`, and `reference` each land
    the same `3`-generated / `0`-admitted completion summary
  - none carries a bound, a best-accept primary rank, a best-accept rank, or
    a survivor sketch
  - each keeps only the same `reference / eventual_lift / next_lift` terminal
    trio, and every one of those terminal continuations still classifies as
    `NeedsFallback`
  - every completed telescope stays structurally connected but outside the
    active window, outside self-containedness, and outside historical
    reanchor with `max_lib_ref = 10`
  - all three clause-`6` continuations stay uniformly blocked at clause `2`
    with `matched_clause_count = 2` and `first_mismatch_position = 2`
  Consequence: direct remaining-one completion and terminal identity on that
  released exact-screen representative claim-flat parent are now spent too.
  The next honest probe has to move sideways to the sibling exact-screen
  representative `claim_sharp_codomain` sheet on the same pair cell rather
  than retrying claim-flat clause-`6` identity, the same summary override, or
  sibling exact-screen pairs.
- Scope: split the sibling exact-screen `claim_sharp_codomain` sheet
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain /
  claim_next_bridge / claim_next_codomain` one layer deeper across its
  remaining-two boundary.
  Result: the attempted clause-`6` split does not exist at this layer on the
  claim-sharp sheet either. The representative claim-sharp clause-`2`
  tradeoff removes exactly one remaining-two exact-prune capture on
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain /
  claim_next_bridge / claim_next_codomain`, that removed capture still has
  only six fixed clauses, and clause `6` is therefore still out of scope at
  the exact-screen boundary. The tradeoff introduces no new exact-prune
  capture family and removes no pruned-terminal family either.
  Consequence: same-layer exact-screen clause-`6` identity is unavailable on
  that representative claim-sharp sheet too. The next honest probe has to
  move below the remaining-two exact-screen boundary and localize that single
  released six-clause parent at remaining-one scope rather than reopening the
  spent claim-flat sheet, the neutral `reference` sheet, or sibling
  exact-screen pairs.
- Scope: test the first remaining-one follow-on beneath that released
  exact-screen representative `claim_sharp_codomain` plus clause-`5`
  `claim_next_codomain` parent by stacking remaining-one exact-summary relief
  under the same exact-screen representative claim-sharp tradeoff.
  Result: the stacked follow-on is a neutral control rather than a new
  tradeoff.
  - the full step-`15` surface stays on the same `4343 / 552 / 2271`
  - it keeps accepted step `15` canonical `103 / 8`
  - it keeps the same `311 / 177 / 50 / 14` first-mismatch distribution
  - it keeps the same `small_cluster = 3141 / 522 / 522 / 0`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)`
  - it keeps the same remaining-one terminal connectivity summary with
    `6813` structurally connected but unqualified generated candidates
  - it releases no remaining-one pruned-terminal prefixes and introduces no
    new remaining-one pruned-terminal group either
  Consequence: remaining-one exact-summary relief is not the missing lever on
  that released exact-screen representative claim-sharp parent either. The
  next honest probe has to move below summary-prune relief and localize the
  same six-clause parent at direct remaining-one completion or terminal scope
  rather than retrying the same summary override, reopening the spent
  claim-flat sheet, the neutral `reference` sheet, or sibling exact-screen
  pairs.
- Scope: localize that same released exact-screen representative
  `claim_sharp_codomain` plus clause-`5` `claim_next_codomain` parent one
  layer deeper at direct remaining-one completion and terminal scope by
  splitting its clause-`6` continuations.
  Result: all three clause-`6` continuations reland the same dead shell on
  the claim-sharp side too:
  - `claim_next_codomain`, `claim_sharp_codomain`, and `reference` each land
    the same `3`-generated / `0`-admitted completion summary
  - none carries a bound, a best-accept primary rank, a best-accept rank, or
    a survivor sketch
  - each keeps only the same `reference / eventual_lift / next_lift` terminal
    trio, and every one of those terminal continuations still classifies as
    `NeedsFallback`
  - every completed telescope stays structurally connected but outside the
    active window, outside self-containedness, and outside historical
    reanchor with `max_lib_ref = 10`
  - all three clause-`6` continuations stay uniformly blocked at clause `2`
    with `matched_clause_count = 2` and `first_mismatch_position = 2`
  Consequence: both representative exact-screen claim-side sheets on
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_next_codomain` are now exhausted below the clause-`2` split. The
  next honest probe has to stop below that spent representative pair and move
  sideways to a sibling exact-screen pair on the same `claim_next_codomain`
  cell rather than reopening either claim-side sheet, the neutral
  `reference` sheet, or the same remaining-one follow-ons.
- Scope: split the first sibling exact-screen pair's clause-`2` identity on
  `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge /
  claim_next_codomain`.
  Result: the sibling pair relands the same smaller exact-screen clause-`2`
  lattice as the spent representative pair rather than exposing fresh
  leverage.
  - the `claim_flat_domain` and `claim_sharp_codomain` sheets each land the
    same `4343 / 552 / 2271` tradeoff
  - both keep accepted step `15` canonical `103 / 8`
  - both keep `incumbent_dominance = 3`
  - both shift first-mismatch counts only to `311 / 177 / 50 / 14`
  - both widen `small_cluster` only to `3141 / 522 / 522 / 0`
  - each cuts only its own claim-side clause-`2` share from `15` to `14`,
    leaving the sibling claim-side sheet at `15` and the sibling
    `reference` sheet at `12`
  - each contracts the targeted exact-screen pair only from `42` to `41`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with the same targeted pair still at `42`
  Consequence: clause-`2` identity on that first sibling exact-screen pair is
  spent too. The next honest probe has to move below the representative
  `claim_flat_domain` sheet on
  `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge /
  claim_next_codomain` rather than reopening the sibling
  `claim_sharp_codomain` sheet, the neutral `reference` sheet, or any
  remaining-one relands on the spent representative pair.
- Scope: split the first sibling exact-screen `claim_flat_domain` sheet
  `claim_eventual_domain / claim_sharp_codomain / claim_flat_domain /
  claim_next_bridge / claim_next_codomain` one layer deeper across its
  remaining-two boundary.
  Result: the attempted clause-`6` split does not exist at this layer on the
  first sibling claim-flat sheet either. The claim-flat clause-`2` tradeoff
  removes exactly one remaining-two exact-prune capture on that six-clause
  parent shell, that removed capture still has only six fixed clauses, and
  clause `6` is therefore still out of scope at the exact-screen boundary.
  The tradeoff introduces no new exact-prune capture family and removes no
  pruned-terminal family either.
  Consequence: same-layer exact-screen clause-`6` identity is unavailable on
  that first sibling claim-flat sheet too. The next honest probe has to move
  below the remaining-two exact-screen boundary and localize that single
  released six-clause parent at remaining-one scope rather than reopening the
  sibling claim-sharp sheet, the neutral `reference` sheet, or the same
  remaining-one relands on the spent representative pair.
- Scope: test the first remaining-one follow-on beneath that released first
  sibling exact-screen `claim_flat_domain` plus clause-`5`
  `claim_next_codomain` parent by stacking remaining-one exact-summary relief
  under the same exact-screen claim-flat tradeoff.
  Result: the stacked follow-on is a neutral control rather than a new
  tradeoff.
  - the full step-`15` surface stays on the same `4343 / 552 / 2271`
  - it keeps accepted step `15` canonical `103 / 8`
  - it keeps the same `311 / 177 / 50 / 14` first-mismatch distribution
  - it keeps the same `small_cluster = 3141 / 522 / 522 / 0`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)`
  - it keeps the same remaining-one terminal connectivity summary with
    `6813` structurally connected but unqualified generated candidates
  - it releases no remaining-one pruned-terminal prefixes and introduces no
    new remaining-one pruned-terminal group either
  Consequence: remaining-one exact-summary relief is not the missing lever on
  that released first sibling exact-screen claim-flat parent either. The next
  honest probe has to move below summary-prune relief and localize the same
  six-clause parent at direct remaining-one completion or terminal scope
  rather than retrying the same summary override, reopening the sibling
  claim-sharp sheet, the neutral `reference` sheet, or bouncing back to the
  spent representative pair.
- Scope: localize that same released first sibling exact-screen
  `claim_flat_domain` plus clause-`5` `claim_next_codomain` parent one layer
  deeper at direct remaining-one completion and terminal scope by splitting
  its clause-`6` continuations.
  Result: all three clause-`6` continuations reland the same dead shell on
  the first sibling claim-flat side too:
  - `claim_next_codomain`, `claim_sharp_codomain`, and `reference` each land
    the same `3`-generated / `0`-admitted completion summary
  - none carries a bound, a best-accept primary rank, a best-accept rank, or
    a survivor sketch
  - each keeps only the same `reference / eventual_lift / next_lift` terminal
    trio, and every one of those terminal continuations still classifies as
    `NeedsFallback`
  - every completed telescope stays structurally connected but outside the
    active window, outside self-containedness, and outside historical
    reanchor with `max_lib_ref = 10`
  - all three clause-`6` continuations stay uniformly blocked at clause `2`
    with `matched_clause_count = 2` and `first_mismatch_position = 2`
  Consequence: direct remaining-one completion and terminal identity on that
  released first sibling exact-screen claim-flat parent are now spent too.
  The next honest probe has to move sideways to the sibling exact-screen
  `claim_sharp_codomain` sheet on the same pair cell rather than retrying
  claim-flat clause-`6` identity, the same summary override, the neutral
  `reference` sheet, or the spent representative pair.
- Scope: split the sibling exact-screen `claim_sharp_codomain` sheet
  `claim_eventual_domain / claim_sharp_codomain / claim_sharp_codomain /
  claim_next_bridge / claim_next_codomain` one layer deeper across its
  remaining-two boundary.
  Result: the attempted clause-`6` split does not exist at this layer on the
  first sibling claim-sharp sheet either. The claim-sharp clause-`2`
  tradeoff removes exactly one remaining-two exact-prune capture on that
  six-clause parent shell, that removed capture still has only six fixed
  clauses, and clause `6` is therefore still out of scope at the exact-screen
  boundary. The tradeoff introduces no new exact-prune capture family and
  removes no pruned-terminal family either.
  Consequence: same-layer exact-screen clause-`6` identity is unavailable on
  that first sibling claim-sharp sheet too. The next honest probe has to move
  below the remaining-two exact-screen boundary and localize that single
  released six-clause parent at remaining-one scope rather than reopening the
  spent first sibling claim-flat sheet, the neutral `reference` sheet there,
  or the spent representative pair.
- Scope: test the first remaining-one follow-on beneath that released first
  sibling exact-screen `claim_sharp_codomain` plus clause-`5`
  `claim_next_codomain` parent by stacking remaining-one exact-summary relief
  under the same exact-screen claim-sharp tradeoff.
  Result: the stacked follow-on is a neutral control rather than a new
  tradeoff.
  - the full step-`15` surface stays on the same `4343 / 552 / 2271`
  - it keeps accepted step `15` canonical `103 / 8`
  - it keeps the same `311 / 177 / 50 / 14` first-mismatch distribution
  - it keeps the same `small_cluster = 3141 / 522 / 522 / 0`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)`
  - it keeps the same remaining-one terminal connectivity summary with
    `6813` structurally connected but unqualified generated candidates
  - it releases no remaining-one pruned-terminal prefixes and introduces no
    new remaining-one pruned-terminal group either
  Consequence: remaining-one exact-summary relief is not the missing lever on
  that released first sibling exact-screen claim-sharp parent either. The
  next honest probe has to move below summary-prune relief and localize the
  same six-clause parent at direct remaining-one completion or terminal scope
  rather than retrying the same summary override, reopening the spent first
  sibling claim-flat sheet, the neutral `reference` sheet, or bouncing back
  to the spent representative pair.
- Scope: localize that same released first sibling exact-screen
  `claim_sharp_codomain` plus clause-`5` `claim_next_codomain` parent one
  layer deeper at direct remaining-one completion and terminal scope by
  splitting its clause-`6` continuations.
  Result: all three clause-`6` continuations reland the same dead shell on
  the first sibling claim-sharp side too:
  - `claim_next_codomain`, `claim_sharp_codomain`, and `reference` each land
    the same `3`-generated / `0`-admitted completion summary
  - none carries a bound, a best-accept primary rank, a best-accept rank, or
    a survivor sketch
  - each keeps only the same `reference / eventual_lift / next_lift` terminal
    trio, and every one of those terminal continuations still classifies as
    `NeedsFallback`
  - every completed telescope stays structurally connected but outside the
    active window, outside self-containedness, and outside historical
    reanchor with `max_lib_ref = 10`
  - all three clause-`6` continuations stay uniformly blocked at clause `2`
    with `matched_clause_count = 2` and `first_mismatch_position = 2`
  Consequence: both claim-side sheets on that first sibling exact-screen pair
  are now spent too. The next honest probe has to stop below that spent first
  sibling pair and move sideways to the next sibling exact-screen pair on the
  same `claim_next_codomain` cell, namely
  `claim_eventual_domain / reference / claim_next_bridge / claim_next_codomain`,
  rather than reopening the spent first sibling claim-flat or claim-sharp
  sheets, the same remaining-one follow-ons, or the already-spent
  representative pair.
- Scope: split the next sibling exact-screen pair's clause-`2` identity on
  `claim_eventual_domain / reference / claim_next_bridge /
  claim_next_codomain`.
  Result: the next sibling pair relands the same smaller exact-screen
  clause-`2` lattice as the spent representative and first sibling pairs
  rather than exposing fresh leverage.
  - the `claim_flat_domain` and `claim_sharp_codomain` sheets each land the
    same `4343 / 552 / 2271` tradeoff
  - both keep accepted step `15` canonical `103 / 8`
  - both keep `incumbent_dominance = 3`
  - both shift first-mismatch counts only to `311 / 177 / 50 / 14`
  - both widen `small_cluster` only to `3141 / 522 / 522 / 0`
  - each cuts only its own claim-side clause-`2` share from `15` to `14`,
    leaving the sibling claim-side sheet at `15` and the sibling
    `reference` sheet at `12`
  - each contracts the targeted exact-screen pair only from `42` to `41`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with the targeted pair still at `42`
  Consequence: clause-`2` identity on that next sibling exact-screen pair is
  spent too. The next honest probe has to move below the representative
  `claim_flat_domain` sheet on
  `claim_eventual_domain / reference / claim_next_bridge /
  claim_next_codomain` rather than reopening the sibling
  `claim_sharp_codomain` sheet, the neutral `reference` sheet, or any
  remaining-one relands on the spent `claim_eventual_domain` pairs.
- Scope: exhaust the next sibling exact-screen `claim_flat_domain` sheet
  `claim_eventual_domain / reference / claim_flat_domain / claim_next_bridge /
  claim_next_codomain` through its exact-screen boundary and first
  remaining-one follow-on.
  Result: the claim-flat side repeats the same spent pattern as the earlier
  exact-screen pairs.
  - the exact-screen delta removes exactly one remaining-two exact-prune
    capture on that released six-clause parent shell
  - clause `6` stays out of scope at the exact-screen boundary
  - the delta introduces no new exact-prune capture family and removes no
    pruned-terminal family
  - stacking remaining-one exact-summary relief stays completely neutral on
    the same `4343 / 552 / 2271`, the same `311 / 177 / 50 / 14`, the same
    `small_cluster = 3141 / 522 / 522 / 0`, the same
    `((0, None, None), 2271)` exact-prune family, and the same `6813`
    structurally connected but unqualified generated candidates
  - the stacked follow-on releases no remaining-one pruned-terminal group
  - the three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary with no bound, no
    best-rank profile, no survivor sketch, and the same
    `reference / eventual_lift / next_lift` `NeedsFallback` trio
  - all three clause-`6` continuations stay uniformly blocked at clause `2`
    with `matched_clause_count = 2` and `first_mismatch_position = 2`
  Consequence: the next sibling exact-screen claim-flat branch is spent too.
  The next honest probe has to move sideways to the sibling
  `claim_sharp_codomain` sheet on that same pair rather than reopening the
  neutral `reference` sheet, the same claim-flat follow-ons, or the spent
  representative and first sibling pairs.
- Scope: exhaust the next sibling exact-screen `claim_sharp_codomain` sheet
  `claim_eventual_domain / reference / claim_sharp_codomain /
  claim_next_bridge / claim_next_codomain` through its exact-screen boundary
  and first remaining-one follow-on.
  Result: the claim-sharp side also repeats the same spent pattern.
  - the exact-screen delta removes exactly one remaining-two exact-prune
    capture on that released six-clause parent shell
  - clause `6` stays out of scope at the exact-screen boundary
  - the delta introduces no new exact-prune capture family and removes no
    pruned-terminal family
  - stacking remaining-one exact-summary relief stays completely neutral on
    the same `4343 / 552 / 2271`, the same `311 / 177 / 50 / 14`, the same
    `small_cluster = 3141 / 522 / 522 / 0`, the same
    `((0, None, None), 2271)` exact-prune family, and the same `6813`
    structurally connected but unqualified generated candidates
  - the stacked follow-on releases no remaining-one pruned-terminal group
  - the three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary with no bound, no
    best-rank profile, no survivor sketch, and the same
    `reference / eventual_lift / next_lift` `NeedsFallback` trio
  - all three clause-`6` continuations stay uniformly blocked at clause `2`
    with `matched_clause_count = 2` and `first_mismatch_position = 2`
  Consequence: the whole `claim_eventual_domain` exact-screen trio on the
  active clause-`5` `claim_next_codomain` cell is now spent. The next honest
  probe has to move sideways to the first clause-`0` sibling exact-screen
  pair on that same cell,
  `claim_flat_domain / claim_next_codomain / claim_next_bridge / claim_next_codomain`,
  rather than reopening the spent representative, first sibling, or next
  sibling pairs, their neutral `reference` sheets, or the same
  remaining-one follow-ons.
- Scope: split the first clause-`0` sibling exact-screen pair's clause-`2`
  identity on `claim_flat_domain / claim_next_codomain / claim_next_bridge /
  claim_next_codomain`.
  Result: the first clause-`0` sibling pair relands the same smaller
  exact-screen clause-`2` lattice as the spent `claim_eventual_domain` trio
  rather than exposing fresh leverage.
  - the `claim_flat_domain` and `claim_sharp_codomain` sheets each land the
    same `4343 / 552 / 2271` tradeoff
  - both keep accepted step `15` canonical `103 / 8`
  - both keep `incumbent_dominance = 3`
  - both shift first-mismatch counts only to `311 / 177 / 50 / 14`
  - both widen `small_cluster` only to `3141 / 522 / 522 / 0`
  - each cuts only its own claim-side clause-`2` share from `15` to `14`,
    leaving the sibling claim-side sheet at `15` and the sibling
    `reference` sheet at `12`
  - each contracts the targeted exact-screen pair only from `42` to `41`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with the targeted pair still at `42`
  Consequence: clause-`2` identity on that first clause-`0` sibling
  exact-screen pair is spent too. The next honest probe has to move below the
  representative `claim_flat_domain` sheet on
  `claim_flat_domain / claim_next_codomain / claim_next_bridge / claim_next_codomain`
  rather than reopening the sibling `claim_sharp_codomain` sheet, the
  neutral `reference` sheet, or any remaining-one relands on the spent
  `claim_eventual_domain` or first-clause-`0`-sibling pairs.
- Scope: exhaust the first clause-`0` sibling exact-screen `claim_flat_domain`
  sheet `claim_flat_domain / claim_next_codomain / claim_flat_domain /
  claim_next_bridge / claim_next_codomain` through its exact-screen boundary
  and first remaining-one follow-on.
  Result: the claim-flat side repeats the same spent pattern as the earlier
  exact-screen pairs.
  - the exact-screen delta removes exactly one remaining-two exact-prune
    capture on that released six-clause parent shell
  - clause `6` stays out of scope at the exact-screen boundary
  - the delta introduces no new exact-prune capture family and removes no
    pruned-terminal family
  - stacking remaining-one exact-summary relief stays completely neutral on
    the same `4343 / 552 / 2271`, the same `311 / 177 / 50 / 14`, the same
    `small_cluster = 3141 / 522 / 522 / 0`, the same
    `((0, None, None), 2271)` exact-prune family, and the same `6813`
    structurally connected but unqualified generated candidates
  - the stacked follow-on releases no remaining-one pruned-terminal group
  - the three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary with no bound, no
    best-rank profile, no survivor sketch, and the same
    `reference / eventual_lift / next_lift` `NeedsFallback` trio
  - all three clause-`6` continuations stay uniformly blocked at clause `2`
    with `matched_clause_count = 2` and `first_mismatch_position = 2`
  Consequence: the first clause-`0` sibling exact-screen claim-flat branch is
  spent too. The next honest probe has to move sideways to the sibling
  `claim_sharp_codomain` sheet on that same pair rather than reopening the
  neutral `reference` sheet, the same claim-flat follow-ons, or the spent
  `claim_eventual_domain` pairs.
- Scope: exhaust the first clause-`0` sibling exact-screen
  `claim_sharp_codomain` sheet `claim_flat_domain / claim_next_codomain /
  claim_sharp_codomain / claim_next_bridge / claim_next_codomain` through its
  exact-screen boundary and first remaining-one follow-on.
  Result: the claim-sharp side also repeats the same spent pattern.
  - the exact-screen delta removes exactly one remaining-two exact-prune
    capture on that released six-clause parent shell
  - clause `6` stays out of scope at the exact-screen boundary
  - the delta introduces no new exact-prune capture family and removes no
    pruned-terminal family
  - stacking remaining-one exact-summary relief stays completely neutral on
    the same `4343 / 552 / 2271`, the same `311 / 177 / 50 / 14`, the same
    `small_cluster = 3141 / 522 / 522 / 0`, the same
    `((0, None, None), 2271)` exact-prune family, and the same `6813`
    structurally connected but unqualified generated candidates
  - the stacked follow-on releases no remaining-one pruned-terminal group
  - the three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary with no bound, no
    best-rank profile, no survivor sketch, and the same
    `reference / eventual_lift / next_lift` `NeedsFallback` trio
  - all three clause-`6` continuations stay uniformly blocked at clause `2`
    with `matched_clause_count = 2` and `first_mismatch_position = 2`
  Consequence: the first clause-`0` sibling exact-screen pair on the active
  clause-`5` `claim_next_codomain` cell is now spent too. The next honest
  probe has to move sideways to the first clause-`1` sibling exact-screen
  pair on the same clause-`0` `claim_flat_domain` family,
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`,
  rather than reopening the spent `claim_eventual_domain` representative,
  first-sibling, or next-sibling pairs, their neutral `reference` sheets, or
  the same remaining-one follow-ons on the spent first clause-`0` sibling
  pair.
- Scope: split the first clause-`1` sibling exact-screen pair's clause-`2`
  identity on `claim_flat_domain / claim_sharp_codomain / claim_next_bridge /
  claim_next_codomain`.
  Result: the first clause-`1` sibling pair relands the same smaller
  exact-screen clause-`2` lattice as the spent `claim_eventual_domain` trio
  and first clause-`0` sibling pair rather than exposing fresh leverage.
  - the `claim_flat_domain` and `claim_sharp_codomain` sheets each land the
    same `4343 / 552 / 2271` tradeoff
  - both keep accepted step `15` canonical `103 / 8`
  - both keep `incumbent_dominance = 3`
  - both shift first-mismatch counts only to `311 / 177 / 50 / 14`
  - both widen `small_cluster` only to `3141 / 522 / 522 / 0`
  - each cuts only its own claim-side clause-`2` share from `15` to `14`,
    leaving the sibling claim-side sheet at `15` and the sibling
    `reference` sheet at `12`
  - each contracts the targeted exact-screen pair only from `42` to `41`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with the targeted pair still at `42`
  Consequence: clause-`2` identity on that first clause-`1` sibling
  exact-screen pair is spent too. The next honest probe has to move below the
  representative `claim_flat_domain` sheet on
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`
  rather than reopening the sibling `claim_sharp_codomain` sheet, the
  neutral `reference` sheet, or any remaining-one relands on the spent
  representative, first-sibling, next-sibling, or first-clause-`0`-sibling
  pairs.
- Scope: split the first clause-`1` sibling exact-screen `claim_flat_domain`
  sheet `claim_flat_domain / claim_sharp_codomain / claim_flat_domain /
  claim_next_bridge / claim_next_codomain` one layer deeper across its
  remaining-two boundary.
  Result: the attempted clause-`6` split does not exist at this layer on the
  first clause-`1` sibling claim-flat sheet either. The claim-flat
  clause-`2` tradeoff removes exactly one remaining-two exact-prune capture
  on that six-clause parent shell, that removed capture still has only six
  fixed clauses, and clause `6` is therefore still out of scope at the
  exact-screen boundary. The tradeoff introduces no new exact-prune capture
  family and removes no pruned-terminal family either.
  Consequence: same-layer exact-screen clause-`6` identity is unavailable on
  that first clause-`1` sibling claim-flat sheet too. The next honest probe
  has to move below the remaining-two exact-screen boundary and test the
  released six-clause parent at remaining-one exact-summary scope rather than
  reopening the sibling `claim_sharp_codomain` sheet, the neutral
  `reference` sheet there, or the spent representative, first-sibling,
  next-sibling, or first-clause-`0`-sibling pairs.
- Scope: test the first remaining-one follow-on beneath that released first
  clause-`1` sibling exact-screen `claim_flat_domain` plus clause-`5`
  `claim_next_codomain` parent by stacking remaining-one exact-summary relief
  under
  `claim_flat_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / claim_next_codomain`.
  Result: remaining-one exact-summary relief on that first clause-`1`
  sibling claim-flat parent is only a neutral control too.
  - it keeps the same `4343 / 552 / 2271`
  - it keeps the same `311 / 177 / 50 / 14`
  - it keeps the same `small_cluster = 3141 / 522 / 522 / 0`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)` and the same remaining-one terminal
    connectivity summary with `6813` structurally connected but unqualified
    generated candidates
  - it releases no remaining-one pruned-terminal group
  Consequence: remaining-one exact-summary relief on that released first
  clause-`1` sibling exact-screen claim-flat parent is spent too. The next
  honest probe has to move below summary-prune relief and localize the same
  six-clause parent at direct remaining-one completion or terminal scope
  rather than retrying the same summary override, reopening the sibling
  `claim_sharp_codomain` sheet, the neutral `reference` sheet there, or
  bouncing back to the spent representative, first-sibling, next-sibling, or
  first-clause-`0`-sibling pairs.
- Scope: localize that same released first clause-`1` sibling exact-screen
  `claim_flat_domain` plus clause-`5` `claim_next_codomain` parent at direct
  remaining-one completion / terminal scope under
  `claim_flat_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / claim_next_codomain`.
  Result: the released first clause-`1` sibling claim-flat parent is spent at
  direct completion too rather than hiding a live child.
  - its three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary
  - each keeps `matched_clause_count = 2` and
    `first_mismatch_position = 2`
  - none carries a bound, a best-rank profile, or a survivor sketch
  - each keeps only the same structurally connected but unqualified
    `reference / eventual_lift / next_lift` terminal trio with
    `NeedsFallback`, no active-window support, no self-containedness, no
    historical reanchor, and `max_lib_ref = 10`
  Consequence: the next honest probe has to move sideways to the sibling
  exact-screen `claim_sharp_codomain` sheet on
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`
  rather than retrying the spent claim-flat parent, the same summary
  override, the neutral `reference` sheet, or the spent representative,
  first-sibling, next-sibling, and first-clause-`0`-sibling pairs.
- Scope: exhaust the first clause-`1` sibling exact-screen
  `claim_sharp_codomain` sheet `claim_flat_domain / claim_sharp_codomain /
  claim_sharp_codomain / claim_next_bridge / claim_next_codomain` through its
  exact-screen boundary, stacked remaining-one exact-summary follow-on, and
  direct remaining-one completion / terminal scope.
  Result: the claim-sharp side also repeats the same spent pattern.
  - the exact-screen delta removes exactly one remaining-two exact-prune
    capture on that released six-clause parent shell
  - clause `6` stays out of scope at the exact-screen boundary
  - the delta introduces no new exact-prune capture family and removes no
    pruned-terminal family
  - stacking remaining-one exact-summary relief stays completely neutral on
    the same `4343 / 552 / 2271`, the same `311 / 177 / 50 / 14`, the same
    `small_cluster = 3141 / 522 / 522 / 0`, the same
    `((0, None, None), 2271)` exact-prune family, and the same `6813`
    structurally connected but unqualified generated candidates
  - the stacked follow-on releases no remaining-one pruned-terminal group
  - the three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary with no bound, no
    best-rank profile, no survivor sketch, and the same
    `reference / eventual_lift / next_lift` `NeedsFallback` trio
  - all three clause-`6` continuations stay uniformly blocked at clause `2`
    with `matched_clause_count = 2` and `first_mismatch_position = 2`
  Consequence: both claim-side sheets on that first clause-`1` sibling
  exact-screen pair are now spent too. The next honest probe has to move
  sideways to the next clause-`1` sibling exact-screen pair on the same
  clause-`0` `claim_flat_domain` family,
  `claim_flat_domain / reference / claim_next_bridge / claim_next_codomain`,
  rather than reopening the spent claim-flat or claim-sharp sheets, the
  neutral `reference` sheet inside that spent pair, or the spent
  representative, first-sibling, next-sibling, or first-clause-`0`-sibling
  pairs.
- Scope: split the next clause-`1` sibling exact-screen pair's clause-`2`
  identity on `claim_flat_domain / reference / claim_next_bridge /
  claim_next_codomain`.
  Result: the next clause-`1` sibling pair relands the same smaller
  exact-screen clause-`2` lattice as the spent representative,
  first-sibling, next-sibling, first-clause-`0`, and first-clause-`1`
  sibling pairs rather than exposing fresh leverage.
  - the `claim_flat_domain` and `claim_sharp_codomain` sheets each land the
    same `4343 / 552 / 2271` tradeoff
  - both keep accepted step `15` canonical `103 / 8`
  - both keep `incumbent_dominance = 3`
  - both shift first-mismatch counts only to `311 / 177 / 50 / 14`
  - both widen `small_cluster` only to `3141 / 522 / 522 / 0`
  - each cuts only its own claim-side clause-`2` share from `15` to `14`,
    leaving the sibling claim-side sheet at `15` and the sibling
    `reference` sheet at `12`
  - each contracts the targeted exact-screen pair only from `42` to `41`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with the targeted pair still at `42`
  Consequence: clause-`2` identity on that next clause-`1` sibling
  exact-screen pair is spent too. The next honest probe has to move below the
  representative `claim_flat_domain` sheet on
  `claim_flat_domain / reference / claim_next_bridge / claim_next_codomain`
  rather than reopening the sibling `claim_sharp_codomain` sheet, the
  neutral `reference` sheet, or the spent representative, first-sibling,
  next-sibling, first-clause-`0`-sibling, or first-clause-`1`-sibling pairs.
- Scope: exhaust the next clause-`1` sibling exact-screen representative
  `claim_flat_domain` sheet `claim_flat_domain / reference /
  claim_flat_domain / claim_next_bridge / claim_next_codomain` through its
  exact-screen boundary, stacked remaining-one exact-summary follow-on, and
  direct remaining-one completion / terminal scope.
  Result: the representative claim-flat side also repeats the same spent
  pattern.
  - the exact-screen delta removes exactly one remaining-two exact-prune
    capture on that released six-clause parent shell
  - clause `6` stays out of scope at the exact-screen boundary
  - the delta introduces no new exact-prune capture family and removes no
    pruned-terminal family
  - stacking remaining-one exact-summary relief stays completely neutral on
    the same `4343 / 552 / 2271`, the same `311 / 177 / 50 / 14`, the same
    `small_cluster = 3141 / 522 / 522 / 0`, the same
    `((0, None, None), 2271)` exact-prune family, and the same `6813`
    structurally connected but unqualified generated candidates
  - the stacked follow-on releases no remaining-one pruned-terminal group
  - the three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary with no bound, no
    best-rank profile, no survivor sketch, and the same
    `reference / eventual_lift / next_lift` `NeedsFallback` trio
  - all three clause-`6` continuations stay uniformly blocked at clause `2`
    with `matched_clause_count = 2` and `first_mismatch_position = 2`
  Consequence: the representative claim-flat sheet on that next clause-`1`
  sibling exact-screen pair is now spent too. The next honest probe has to
  move sideways to the sibling exact-screen `claim_sharp_codomain` sheet on
  that same pair,
  `claim_flat_domain / reference / claim_next_bridge / claim_next_codomain`,
  rather than reopening the spent representative claim-flat sheet there, the
  neutral `reference` sheet on that pair, or the spent representative,
  first-sibling, next-sibling, first-clause-`0`-sibling, or
  first-clause-`1`-sibling pairs.
- Scope: exhaust the sibling next clause-`1` exact-screen
  `claim_sharp_codomain` sheet
  `claim_flat_domain / reference / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`
  through its exact-screen boundary, stacked remaining-one exact-summary
  follow-on, and direct remaining-one completion / terminal scope.
  Result: the last live claim-side branch on the exact-screen
  `claim_next_codomain` cell also repeats the same spent pattern.
  - the exact-screen delta removes exactly one remaining-two exact-prune
    capture on that released six-clause parent shell
  - clause `6` stays out of scope at the exact-screen boundary
  - the delta introduces no new exact-prune capture family and removes no
    pruned-terminal family
  - stacking remaining-one exact-summary relief stays completely neutral on
    the same `4343 / 552 / 2271`, the same `311 / 177 / 50 / 14`, the same
    `small_cluster = 3141 / 522 / 522 / 0`, the same
    `((0, None, None), 2271)` exact-prune family, and the same `6813`
    structurally connected but unqualified generated candidates
  - the stacked follow-on releases no remaining-one pruned-terminal group
  - the three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary with no bound, no
    best-rank profile, no survivor sketch, and the same
    `reference / eventual_lift / next_lift` `NeedsFallback` trio
  - all three clause-`6` continuations stay uniformly blocked at clause `2`
    with `matched_clause_count = 2` and `first_mismatch_position = 2`
  Consequence: both claim-side sheets on all six exact-screen pair cells
  under the active clause-`5` `claim_next_codomain` family are now spent, so
  the next honest probe has to move sideways to the sibling active
  mismatch-`0` clause-`5` `reference` family, below its representative
  `claim_flat_domain` clause-`2` sheet at finer remaining-one exact-summary
  scope under
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`,
  rather than reopening any spent exact-screen pair-cell split, claim-side
  sheet, or neutral `reference` sheet on the exhausted exact-screen cell.
- Scope: localize the sibling active mismatch-`0` clause-`5` `reference`
  family's representative `claim_flat_domain` clause-`2` sheet one layer
  deeper across its clause-`6` remaining-one continuations.
  Result: beneath
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`,
  the `claim_next_codomain`, `claim_sharp_codomain`, and `reference`
  continuations all reland the same matched smaller `4343 / 552` shell with
  `small_cluster = 3141 / 522 / 522 / 0`, the representative clause-`2`
  spread fixed at `14 / 15 / 12`, the representative clause-`4` split fixed
  at `23 / 18`, and the active clause-`4` `claim_next_bridge` plus
  clause-`5` `reference` bucket fixed at `47`, differing only in the deeper
  zero-admitted tail `2270 / 2269 / 2268`.
  Consequence: clause-`6` identity is now spent on that sibling active
  `reference`-family representative claim-flat sheet too. The next honest
  probe has to move below the marginally best clause-`6` `reference`
  continuation on that same sheet rather than swapping among the spent
  clause-`6` siblings, reopening the representative clause-`2` split, or
  bouncing back to the exhausted `claim_next_codomain` exact-screen cell.
- Scope: split that same sibling active mismatch-`0` clause-`5` `reference`
  family's representative `claim_flat_domain` marginally best clause-`6`
  `reference` continuation one layer deeper across its clause-`3` branches.
  Result: the two clause-`3` branches are individually neutral controls.
  Beneath
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`,
  both `claim_flat_argument` and `claim_eventual_argument` reland the
  untouched `4331 / 553 / 2271` baseline, keep first-mismatch counts
  `312 / 177 / 50 / 14`, and keep
  `small_cluster = 3132 / 522 / 522 / 0`.
  Consequence: clause-`3` identity is not the missing repair on that sibling
  active `reference`-family shell either. The next honest probe has to move
  below the broader joint clause-`3` continuation on that same marginally
  best clause-`6` `reference` shell rather than privileging either neutral
  clause-`3` branch, swapping among the spent clause-`6` siblings, reopening
  the spent representative clause-`2` split, or bouncing back to the
  exhausted `claim_next_codomain` exact-screen cell.
- Scope: localize that same sibling active mismatch-`0` clause-`5`
  `reference` family's representative `claim_flat_domain` marginally best
  clause-`6` `reference` continuation one layer deeper at joint clause-`3`
  delta scope.
  Result: relative to either individually neutral clause-`3` branch, the
  broader clause-`6` `reference` tradeoff on
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  differs only by removing one remaining-two exact-prune capture on that same
  parent shell and by reopening the same three clause-`6` remaining-one
  continuations `claim_next_codomain / claim_sharp_codomain / reference`
  beneath it; it introduces no new partial-prefix capture family and no new
  pruned-terminal family.
  Consequence: the sibling active `reference`-family joint clause-`3`
  continuation is now localized to one remaining-two parent plus its three
  clause-`6` remaining-one child continuations. The next honest probe has to
  move below that parent/child shell and localize a finer remaining-one
  completion or terminal partition there rather than reopening either neutral
  clause-`3` branch, rerunning the whole joint clause-`3` continuation,
  swapping among spent clause-`6` siblings, reopening the representative
  clause-`2` split, or bouncing back to the exhausted
  `claim_next_codomain` exact-screen cell.
- Scope: localize that same sibling active mismatch-`0` clause-`5`
  `reference` family's representative `claim_flat_domain` parent/child shell
  one layer deeper at remaining-one completion / terminal scope.
  Result: beneath
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`,
  all six clause-`3` / clause-`6` child continuations across
  `claim_flat_argument / claim_eventual_argument` and
  `claim_next_codomain / claim_sharp_codomain / reference` reland the same
  dead `3`-generated / `0`-admitted completion summary with no bound, no
  best-rank profile, no survivor sketch, and only the same
  `reference / eventual_lift / next_lift` terminal trio; each of those three
  terminal choices stays locally `KeepWithoutFallback` plus
  `open_band_structural`, yet none passes live connectivity.
  Consequence: the sibling active `reference`-family representative
  `claim_flat_domain` shell is not hiding a finer completion or terminal
  partition either, so the next honest probe had to localize a reason split
  beneath that same dead shell rather than reopening the same clause-`3`
  branches, the spent clause-`6` siblings, or the broader joint
  continuation again.
- Scope: localize that same sibling active mismatch-`0` clause-`5`
  `reference` family's representative `claim_flat_domain` dead shell one
  layer deeper at first finer reason scope.
  Result: across all `18` clause-`3` / clause-`6` / terminal continuations
  beneath
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`,
  every completed telescope keeps `matched_clause_count = 2` and
  `first_mismatch_position = 2`, remains structurally connected, stays
  outside active-window qualification, stays outside self-containedness,
  stays outside historical reanchor with `max_lib_ref = 10`, and still fails
  connectivity.
  Consequence: the sibling active `reference`-family representative
  `claim_flat_domain` shell is now spent through both completion and the
  first finer reason split, so the next honest probe has to move sideways to
  the sibling active clause-`5` `reference` family's representative
  `claim_sharp_codomain` clause-`2` sheet under
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`
  rather than reopening the spent claim-flat shell or bouncing back to the
  exhausted `claim_next_codomain` exact-screen cell.
- Scope: localize that same sibling active mismatch-`0` clause-`5`
  `reference` family's representative `claim_sharp_codomain` clause-`2`
  sheet one layer deeper across its clause-`6` remaining-one continuations.
  Result: beneath
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`,
  the `claim_next_codomain`, `claim_sharp_codomain`, and `reference`
  continuations all reland the same matched smaller `4343 / 552` shell, keep
  `small_cluster = 3141 / 522 / 522 / 0`, keep the representative
  clause-`2` spread at `15 / 14 / 12`, keep the representative
  clause-`4` split at `23 / 18`, and keep the active
  `claim_next_bridge` plus clause-`5` `reference` bucket at `47`, differing
  only in the deeper zero-admitted tail `2270 / 2269 / 2268`.
  Consequence: clause-`6` identity is now exhausted on that sibling active
  `reference`-family claim-sharp sheet too. The next honest probe had to
  move below the marginally best clause-`6` `reference` continuation on that
  same sheet rather than swapping among the spent clause-`6` siblings there,
  reopening the spent representative clause-`2` split, or bouncing back to
  the exhausted `claim_next_codomain` exact-screen cell.
- Scope: split that same sibling active mismatch-`0` clause-`5` `reference`
  family's representative `claim_sharp_codomain` marginally best
  clause-`6` `reference` continuation one layer deeper across its clause-`3`
  branches.
  Result: the two clause-`3` branches are individually neutral controls.
  Beneath
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`,
  both `claim_flat_argument` and `claim_eventual_argument` reland the
  untouched `4331 / 553 / 2271` baseline, keep first-mismatch counts
  `312 / 177 / 50 / 14`, and keep
  `small_cluster = 3132 / 522 / 522 / 0`.
  Consequence: clause-`3` identity is not the missing repair on that sibling
  active `reference`-family claim-sharp shell either. The next honest probe
  had to move below the broader joint clause-`3` continuation on that same
  marginally best clause-`6` `reference` shell rather than privileging
  either neutral clause-`3` branch, swapping among spent clause-`6`
  siblings, reopening the representative clause-`2` split, or bouncing back
  to the exhausted `claim_next_codomain` exact-screen cell.
- Scope: localize that same sibling active mismatch-`0` clause-`5`
  `reference` family's representative `claim_sharp_codomain` marginally best
  clause-`6` `reference` continuation one layer deeper at joint clause-`3`
  delta scope.
  Result: relative to either individually neutral clause-`3` branch, the
  broader clause-`6` `reference` tradeoff on
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`
  differs only by removing one remaining-two exact-prune capture on that same
  parent shell and by reopening the same three clause-`6` remaining-one
  continuations `claim_next_codomain / claim_sharp_codomain / reference`
  beneath it; it introduces no new partial-prefix capture family and no new
  pruned-terminal family.
  Consequence: the sibling active `reference`-family claim-sharp joint
  clause-`3` continuation is now localized to one remaining-two parent plus
  its three clause-`6` remaining-one child continuations. The next honest
  probe has to move below that parent/child shell and localize a finer
  remaining-one completion or terminal partition there rather than reopening
  either neutral clause-`3` branch, rerunning the whole joint clause-`3`
  continuation, swapping among spent clause-`6` siblings, reopening the
  representative clause-`2` split, or bouncing back to the exhausted
  `claim_next_codomain` exact-screen cell.
- Scope: localize that same sibling active mismatch-`0` clause-`5`
  `reference` family's representative `claim_sharp_codomain` parent/child
  shell one layer deeper at remaining-one completion / terminal scope.
  Result: beneath
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`,
  all six clause-`3` / clause-`6` child continuations across
  `claim_flat_argument / claim_eventual_argument` and
  `claim_next_codomain / claim_sharp_codomain / reference` reland the same
  dead `3`-generated / `0`-admitted completion summary with no bound, no
  best-rank profile, no survivor sketch, and only the same
  `reference / eventual_lift / next_lift` terminal trio; each of those three
  terminal choices stays locally `KeepWithoutFallback` plus
  `open_band_structural`, yet none passes live connectivity.
  Consequence: the sibling active `reference`-family representative
  `claim_sharp_codomain` shell is not hiding a finer completion or terminal
  partition either, so the next honest probe had to localize a reason split
  beneath that same dead shell rather than reopening the same clause-`3`
  branches, the spent clause-`6` siblings, or the broader joint
  continuation again.
- Scope: localize that same sibling active mismatch-`0` clause-`5`
  `reference` family's representative `claim_sharp_codomain` dead shell one
  layer deeper at first finer reason scope.
  Result: across all `18` clause-`3` / clause-`6` / terminal continuations
  beneath
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`,
  every completed telescope keeps `matched_clause_count = 2` and
  `first_mismatch_position = 2`, remains structurally connected, stays
  outside active-window qualification, stays outside self-containedness,
  stays outside historical reanchor with `max_lib_ref = 10`, and still fails
  connectivity.
  Consequence: the sibling active `reference`-family representative
  `claim_sharp_codomain` shell is now spent through both completion and the
  first finer reason split, so the next honest probe has to move sideways to
  the first sibling exact-screen pair on that same active clause-`5`
  `reference` family, starting with the representative `claim_flat_domain`
  clause-`2` sheet under
  `claim_eventual_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / reference`,
  rather than reopening the spent representative claim-sharp shell,
  rerunning the whole joint clause-`3` continuation there, reopening the
  spent representative clause-`2` split on that same reference family, or
  bouncing back to the exhausted `claim_next_codomain` exact-screen cell.
- Scope: split the first sibling exact-screen pair on that same active
  mismatch-`0` clause-`5` `reference` family one layer deeper across its
  clause-`2` sheets.
  Result: on
  `claim_eventual_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / reference`,
  the `claim_flat_domain` and `claim_sharp_codomain` sheets each land the
  same smaller `4343 / 552 / 2271` tradeoff with first-mismatch counts
  `311 / 177 / 50 / 14`, `small_cluster = 3141 / 522 / 522 / 0`, and the
  targeted exact pair contracted only from `42` to `41`, while the sibling
  `reference` sheet stays neutral on the untouched `4331 / 553 / 2271`
  baseline with the pair still at `42`.
  Consequence: that first sibling exact-screen pair on the active
  `reference` family again lives only on the two claim-side sheets, so the
  next honest probe had to move below the representative `claim_flat_domain`
  sheet rather than reopening the sibling `claim_sharp_codomain` or neutral
  `reference` sheets.
- Scope: localize that same first sibling exact-screen
  `claim_flat_domain` sheet on the active mismatch-`0` clause-`5`
  `reference` family one layer deeper at the exact-screen boundary.
  Result: it removes exactly one six-clause exact-prune capture on
  `claim_eventual_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / reference`,
  introduces no new exact-prune capture family, and removes no deeper
  pruned-terminal family.
  Consequence: clause `6` stays out of scope at that exact-screen boundary
  too, so the next honest probe had to stay below the released
  six-clause `reference` parent rather than reopening the same clause-`2`
  split or jumping sideways to the sibling `claim_sharp_codomain` sheet.
- Scope: stack remaining-one exact-summary relief beneath that same released
  first sibling exact-screen `claim_flat_domain` plus clause-`5`
  `reference` parent.
  Result: it keeps the same `4343 / 552`, the same
  `311 / 177 / 50 / 14`, and the same
  `small_cluster = 3141 / 522 / 522 / 0`, but lowers the zero-admitted tail
  from `2271` to `2268`, keeps a single zero-admitted exact-prune family now
  at `((0, None, None), 2268)`, lowers structurally connected but
  unqualified generated candidates from `6813` to `6804`, and does so by
  shaving exactly three remaining-one pruned prefixes beneath that same
  released `reference` parent while introducing no new remaining-one pruned
  prefix.
  Consequence: this first sibling exact-screen claim-flat follow-on is not a
  neutral control, so the next honest probe has to stay below the sharper
  `4343 / 552 / 2268` shell and localize its clause-`6` continuations rather
  than moving sideways to the sibling `claim_sharp_codomain` sheet or
  bouncing back to the spent higher-level `reference`-family shell.
- Scope: localize that same first sibling exact-screen `claim_flat_domain`
  plus clause-`5` `reference` shell one layer deeper across its clause-`6`
  remaining-one continuations.
  Result: beneath
  `claim_eventual_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / reference`,
  the `claim_next_codomain`, `claim_sharp_codomain`, and `reference`
  continuations all reland the same matched smaller `4343 / 552` shell, keep
  first-mismatch counts `311 / 177 / 50 / 14`, keep
  `small_cluster = 3141 / 522 / 522 / 0`, keep the localized clause-`2`
  spread at `14 / 15 / 12`, keep the localized clause-`4` split at `23 / 18`,
  keep the active clause-`4` `claim_next_bridge` plus clause-`5`
  `reference` bucket at `47`, and differ only in the deeper zero-admitted
  tail `2270 / 2269 / 2268`.
  Consequence: clause-`6` identity is now exhausted on that first sibling
  exact-screen claim-flat shell too, so the next honest probe has to move
  below the marginally best clause-`6` `reference` continuation there and
  split its clause-`3` branches rather than swapping among the spent
  `claim_next_codomain` or `claim_sharp_codomain` siblings, moving sideways
  to the sibling exact-screen `claim_sharp_codomain` sheet, or bouncing back
  to the spent higher-level `reference`-family shell.
- Scope: split that same first sibling exact-screen mismatch-`0`
  clause-`5` `reference` family `claim_flat_domain` shell's marginally best
  clause-`6` `reference` continuation one layer deeper across its clause-`3`
  branches.
  Result: beneath
  `claim_eventual_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / reference`,
  both `claim_flat_argument` and `claim_eventual_argument` land the same
  matched smaller `4343 / 552 / 2271` shell, keep first-mismatch counts
  `311 / 177 / 50 / 14`, and keep
  `small_cluster = 3141 / 522 / 522 / 0`.
  Consequence: clause-`3` identity is already exhausted there as matched
  smaller controls, so the next honest probe had to compare those matched
  branches against the broader joint clause-`3` continuation instead of
  privileging either branch as a fresh repair class.
- Scope: localize that same first sibling exact-screen mismatch-`0`
  clause-`5` `reference` family `claim_flat_domain` shell's broader joint
  clause-`3` continuation at delta scope.
  Result: relative to either matched clause-`3` branch, the broader joint
  clause-`3` continuation changes no remaining-two partial-prefix capture
  and differs only by reopening the same three clause-`6` remaining-one
  continuations `claim_next_codomain / claim_sharp_codomain / reference`,
  with no new remaining-one pruned-terminal family beyond those three
  reopened continuations.
  Consequence: the next honest probe has to move below that broader joint
  clause-`3` continuation and localize a finer remaining-one completion or
  terminal partition beneath those same three clause-`6` continuations
  rather than reopening either spent clause-`3` branch, the spent
  clause-`6` siblings, or the higher-level exact-screen shell.
- Scope: localize the sibling first exact-screen `claim_sharp_codomain`
  sheet on that same active mismatch-`0` clause-`5` `reference` family one
  layer deeper through the exact-screen boundary and stacked remaining-one
  exact-summary relief.
  Result: the exact-screen delta removes exactly one six-clause exact-prune
  capture on
  `claim_eventual_domain / claim_sharp_codomain / claim_sharp_codomain / claim_next_bridge / reference`
  with no introduced exact-prune or pruned-terminal family, while the
  stacked follow-on keeps the same `4343 / 552`,
  `311 / 177 / 50 / 14`, and `small_cluster = 3141 / 522 / 522 / 0`, but
  sharpens the deeper zero-admitted tail from `2271` to `2268`, keeps the
  single zero-admitted exact-prune family at `((0, None, None), 2268)`,
  lowers structurally connected but unqualified generated candidates from
  `6813` to `6804`, and does so by shaving exactly three remaining-one
  pruned prefixes.
  Consequence: this sibling exact-screen claim-sharp follow-on is another
  sharper smaller tradeoff rather than a neutral control, so the live branch
  had to stay below that `4343 / 552 / 2268` shell instead of reopening the
  spent claim-flat shell, the neutral `reference` sheet on the same first
  sibling pair, or the higher-level `reference`-family shell.
- Scope: localize that same first sibling exact-screen
  `claim_sharp_codomain` plus clause-`5` `reference` shell one layer deeper
  across its clause-`6` continuations and the marginally best clause-`6`
  `reference` continuation's clause-`3` split.
  Result: the `claim_next_codomain`, `claim_sharp_codomain`, and `reference`
  clause-`6` continuations all reland the same matched smaller
  `4343 / 552` shell, keep first-mismatch counts `311 / 177 / 50 / 14`,
  keep `small_cluster = 3141 / 522 / 522 / 0`, keep the localized
  clause-`2` spread at `15 / 14 / 12`, keep the localized clause-`4` split
  at `23 / 18`, keep the active clause-`4` `claim_next_bridge` plus
  clause-`5` `reference` bucket at `47`, and differ only in the deeper
  zero-admitted tail `2270 / 2269 / 2268`; beneath the marginally best
  clause-`6` `reference` continuation, both `claim_flat_argument` and
  `claim_eventual_argument` clause-`3` branches reland the same matched
  smaller `4343 / 552 / 2271` shell with the same
  `311 / 177 / 50 / 14` and `small_cluster = 3141 / 522 / 522 / 0`.
  Consequence: clause-`6` and clause-`3` identity are both exhausted on that
  sibling exact-screen claim-sharp shell; neither clause-`3` branch is
  newly privileged.
- Scope: localize that same first sibling exact-screen
  `claim_sharp_codomain` plus clause-`5` `reference` shell's broader joint
  clause-`3` continuation and then localize the resulting parent/child shell
  through completion and first finer reason scope.
  Result: relative to either matched clause-`3` branch, the broader joint
  clause-`3` continuation changes no remaining-two partial-prefix capture
  and differs only by reopening the same three clause-`6` remaining-one
  continuations `claim_next_codomain / claim_sharp_codomain / reference`;
  beneath that joint shell, all six clause-`3` / clause-`6` child
  continuations collapse to matched dead `3`-generated / `0`-admitted`
  completion summaries with no bound, no survivor sketch, and only the same
  `reference / eventual_lift / next_lift` open-band structural terminal trio;
  across all `18` clause-`3` / clause-`6` / terminal continuations, every
  completed telescope keeps `matched_clause_count = 2`,
  `first_mismatch_position = 2`, remains structurally connected, stays
  outside active-window qualification, stays outside self-containedness,
  stays outside historical reanchor with `max_lib_ref = 10`, and still fails
  connectivity.
  Consequence: that first sibling exact-screen active `reference`-family
  claim-sharp shell is now spent through both completion and the first finer
  reason split, so the next honest live branch moves sideways to the next
  sibling exact-screen pair on the same active clause-`5` `reference`
  family, starting with
  `claim_eventual_domain / reference / claim_flat_domain / claim_next_bridge / reference`,
  instead of reopening the spent first sibling pair again.
- Scope: split the next sibling exact-screen pair on that same active
  mismatch-`0` clause-`5` `reference` family one layer deeper across its
  clause-`2` sheets.
  Result: on
  `claim_eventual_domain / reference / claim_flat_domain / claim_next_bridge / reference`,
  the `claim_flat_domain` and `claim_sharp_codomain` sheets each land the
  same smaller `4343 / 552 / 2271` tradeoff with first-mismatch counts
  `311 / 177 / 50 / 14`, `small_cluster = 3141 / 522 / 522 / 0`, and the
  targeted exact pair contracted only from `42` to `41`, while the sibling
  `reference` sheet stays neutral on the untouched `4331 / 553 / 2271`
  baseline with the pair still at `42`.
  Consequence: that next sibling exact-screen pair on the active
  `reference` family again lives only on the two claim-side sheets, so the
  next honest probe had to move below the representative `claim_flat_domain`
  sheet rather than reopening the sibling `claim_sharp_codomain` or neutral
  `reference` sheets.
- Scope: localize that same next sibling exact-screen `claim_flat_domain`
  sheet on the active mismatch-`0` clause-`5` `reference` family through
  the exact-screen boundary and stacked remaining-one exact-summary relief.
  Result: the exact-screen delta removes exactly one six-clause exact-prune
  capture on
  `claim_eventual_domain / reference / claim_flat_domain / claim_next_bridge / reference`
  with no introduced exact-prune or pruned-terminal family, while the
  stacked follow-on keeps the same `4343 / 552`,
  `311 / 177 / 50 / 14`, and `small_cluster = 3141 / 522 / 522 / 0`, but
  sharpens the deeper zero-admitted tail from `2271` to `2268`, keeps the
  single zero-admitted exact-prune family at `((0, None, None), 2268)`,
  lowers structurally connected but unqualified generated candidates from
  `6813` to `6804`, and does so without introducing any new deeper
  pruned-terminal family.
  Consequence: this next sibling exact-screen claim-flat follow-on is another
  sharper smaller tradeoff rather than a neutral control, so the live branch
  had to stay below that `4343 / 552 / 2268` shell instead of moving
  sideways immediately to the sibling `claim_sharp_codomain` sheet or the
  neutral `reference` sheet.
- Scope: localize that same next sibling exact-screen
  `claim_flat_domain` plus clause-`5` `reference` shell one layer deeper
  across its clause-`6` continuations and the marginally best clause-`6`
  `reference` continuation's clause-`3` split.
  Result: the `claim_next_codomain`, `claim_sharp_codomain`, and `reference`
  clause-`6` continuations all reland the same matched smaller
  `4343 / 552` shell, keep first-mismatch counts `311 / 177 / 50 / 14`,
  keep `small_cluster = 3141 / 522 / 522 / 0`, and differ only in the
  deeper zero-admitted tail `2270 / 2269 / 2268`; beneath the marginally
  best clause-`6` `reference` continuation, both `claim_flat_argument` and
  `claim_eventual_argument` clause-`3` branches reland the same matched
  smaller `4343 / 552 / 2271` shell with the same
  `311 / 177 / 50 / 14` and `small_cluster = 3141 / 522 / 522 / 0`.
  Consequence: clause-`6` and clause-`3` identity are both exhausted on that
  next sibling exact-screen claim-flat shell; neither clause-`3` branch is
  newly privileged.
- Scope: localize the released next sibling active `reference`-family
  `claim_flat_domain` parent one layer deeper across its clause-`6`
  completion profiles.
  Result: each of `claim_next_codomain / claim_sharp_codomain / reference`
  collapses to the same dead `3`-generated / `0`-admitted completion
  summary with no bound, no best-rank profile, and no survivor sketch; each
  keeps only the same `reference / eventual_lift / next_lift`
  `NeedsFallback` trio; and each keeps `matched_clause_count = 2` with
  `first_mismatch_position = 2`.
  Consequence: that next sibling exact-screen active `reference`-family
  claim-flat shell is now spent through the same smaller-tradeoff ladder and
  dead completion profile as the prior first sibling claim-flat shell, so
  the next honest live branch moves sideways to the sibling
  `claim_sharp_codomain` sheet on that same pair under
  `claim_eventual_domain / reference / claim_sharp_codomain / claim_next_bridge / reference`.
- Scope: localize that same next sibling exact-screen
  `claim_sharp_codomain` sheet on the active mismatch-`0` clause-`5`
  `reference` family through the exact-screen boundary, stacked
  remaining-one exact-summary relief, the matched clause-`6` /
  clause-`3` relands, and the released parent's clause-`6` completion
  profiles.
  Result: beneath
  `claim_eventual_domain / reference / claim_sharp_codomain / claim_next_bridge / reference`,
  the exact-screen delta removes exactly one six-clause exact-prune capture
  with no introduced exact-prune or pruned-terminal family; stacking
  remaining-one exact-summary relief beneath that released parent keeps the
  same `4343 / 552`, the same `311 / 177 / 50 / 14`, and the same
  `small_cluster = 3141 / 522 / 522 / 0`, but sharpens the deeper
  zero-admitted tail from `2271` to `2268`; its three clause-`6`
  continuations again reland the same matched smaller `4343 / 552` shell and
  differ only in the deeper zero-admitted tail `2270 / 2269 / 2268`; beneath
  the marginally best clause-`6` `reference` continuation, both
  `claim_flat_argument` and `claim_eventual_argument` reland the same matched
  smaller `4343 / 552 / 2271` shell; and each of
  `claim_next_codomain / claim_sharp_codomain / reference` again collapses
  to the same dead `3`-generated / `0`-admitted completion summary with no
  bound, no best-rank profile, no survivor sketch, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  `matched_clause_count = 2` plus `first_mismatch_position = 2`.
  Consequence: that next sibling exact-screen active `reference`-family
  claim-sharp shell is now also spent through the same smaller-tradeoff
  ladder and dead completion profile as the prior first sibling claim-sharp
  shell, so the next honest live branch has to move below that dead shell and
  localize a first finer reason split there rather than reopening the spent
  claim-flat sibling, privileging the neutral `reference` sheet, or bouncing
  back to the higher-level `reference`-family shell.
- Scope: localize that same next sibling exact-screen active `reference`-
  family claim-sharp dead shell one layer deeper at first finer reason scope.
  Result: across all `18` clause-`3` / clause-`6` / terminal continuations
  beneath
  `claim_eventual_domain / reference / claim_sharp_codomain / claim_next_bridge / reference`,
  every completed telescope keeps `matched_clause_count = 2` and
  `first_mismatch_position = 2`, remains structurally connected, stays
  outside active-window qualification, stays outside self-containedness,
  stays outside historical reanchor with `max_lib_ref = 10`, and still fails
  connectivity.
  Consequence: the full `claim_eventual_domain / reference` row on the active
  clause-`5` `reference` family is now spent through both completion and the
  first finer reason split, so the next honest live branch has to move
  sideways to the first clause-`0` sibling exact-screen pair on that same
  active clause-`5` `reference` family, starting with the representative
  `claim_flat_domain` sheet under
  `claim_flat_domain / claim_next_codomain / claim_next_bridge / reference`,
  rather than reopening the exhausted `claim_eventual_domain / reference`
  row, privileging the neutral `reference` sheet, or bouncing back to
  claim-safe controls.
- Scope: split the first clause-`0` sibling exact-screen pair on that same
  active mismatch-`0` clause-`5` `reference` family one layer deeper across
  its clause-`2` sheets.
  Result: on
  `claim_flat_domain / claim_next_codomain / claim_next_bridge / reference`,
  the `claim_flat_domain` and `claim_sharp_codomain` sheets each land the
  same smaller `4343 / 552 / 2271` tradeoff with first-mismatch counts
  `311 / 177 / 50 / 14`, `small_cluster = 3141 / 522 / 522 / 0`, and the
  targeted exact pair contracted only from `42` to `41`, while the sibling
  `reference` sheet stays neutral on the untouched `4331 / 553 / 2271`
  baseline with the pair still at `42`.
  Consequence: that first clause-`0` sibling exact-screen pair on the active
  `reference` family again lives only on the two claim-side sheets, so the
  next honest probe had to move below the representative `claim_flat_domain`
  sheet rather than reopening the sibling `claim_sharp_codomain` or neutral
  `reference` sheets, rerunning the spent `claim_eventual_domain / reference`
  row, or bouncing back to claim-safe controls.
- Scope: localize that same first clause-`0` sibling exact-screen
  `claim_flat_domain` sheet on the active mismatch-`0` clause-`5`
  `reference` family through the exact-screen boundary, stacked
  remaining-one exact-summary relief, the matched clause-`6` /
  clause-`3` relands, and the released parent's clause-`6` completion
  profiles.
  Result: beneath
  `claim_flat_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`,
  the exact-screen delta removes exactly one six-clause exact-prune capture
  with no introduced exact-prune or pruned-terminal family; stacking
  remaining-one exact-summary relief beneath that released parent keeps the
  same `4343 / 552`, the same `311 / 177 / 50 / 14`, and the same
  `small_cluster = 3141 / 522 / 522 / 0`, but sharpens the deeper
  zero-admitted tail from `2271` to `2268`; its three clause-`6`
  continuations reland the same matched smaller `4343 / 552` shell and
  differ only in the deeper zero-admitted tail `2270 / 2269 / 2268`;
  beneath the marginally best clause-`6` `reference` continuation, both
  `claim_flat_argument` and `claim_eventual_argument` reland the same matched
  smaller `4343 / 552 / 2271` shell; and each of
  `claim_next_codomain / claim_sharp_codomain / reference` collapses to the
  same dead `3`-generated / `0`-admitted completion summary with no bound,
  no best-rank profile, no survivor sketch, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  `matched_clause_count = 2` plus `first_mismatch_position = 2`.
  Consequence: that first clause-`0` sibling exact-screen active
  `reference`-family claim-flat shell is now spent through the same
  smaller-tradeoff ladder and dead completion profile as the earlier
  reference-family claim-flat shells, so the next honest probe has to move
  sideways to the sibling `claim_sharp_codomain` sheet on that same pair
  under
  `claim_flat_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`,
  rather than reopening the spent claim-flat shell, privileging the neutral
  `reference` sheet, or bouncing back to the exhausted
  `claim_eventual_domain / reference` row.
- Scope: localize that same first clause-`0` sibling exact-screen
  `claim_sharp_codomain` sheet on the active mismatch-`0` clause-`5`
  `reference` family through the exact-screen boundary, stacked
  remaining-one exact-summary relief, the matched clause-`6` /
  clause-`3` relands, and the released parent's clause-`6` completion
  profiles.
  Result: beneath
  `claim_flat_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`,
  the exact-screen delta removes exactly one six-clause exact-prune capture
  with no introduced exact-prune or pruned-terminal family; stacking
  remaining-one exact-summary relief beneath that released parent keeps the
  same `4343 / 552`, the same `311 / 177 / 50 / 14`, and the same
  `small_cluster = 3141 / 522 / 522 / 0`, but sharpens the deeper
  zero-admitted tail from `2271` to `2268`; its three clause-`6`
  continuations reland the same matched smaller `4343 / 552` shell and
  differ only in the deeper zero-admitted tail `2270 / 2269 / 2268`;
  beneath the marginally best clause-`6` `reference` continuation, both
  `claim_flat_argument` and `claim_eventual_argument` reland the same matched
  smaller `4343 / 552 / 2271` shell; and each of
  `claim_next_codomain / claim_sharp_codomain / reference` collapses to the
  same dead `3`-generated / `0`-admitted completion summary with no bound,
  no best-rank profile, no survivor sketch, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  `matched_clause_count = 2` plus `first_mismatch_position = 2`.
  Consequence: that first clause-`0` sibling exact-screen active
  `reference`-family claim-sharp shell is now also spent through the same
  smaller-tradeoff ladder and dead completion profile as the earlier
  reference-family claim-sharp shells, so the next honest probe has to move
  below that dead shell and localize a first finer reason split there rather
  than reopening the spent claim-flat sibling, privileging the neutral
  `reference` sheet, or bouncing back to the exhausted
  `claim_eventual_domain / reference` row.
- Scope: localize the first finer reason split beneath that same first
  clause-`0` sibling exact-screen `claim_sharp_codomain` dead shell on the
  active mismatch-`0` clause-`5` `reference` family.
  Result: across all `18` clause-`3` / clause-`6` / terminal continuations
  beneath
  `claim_flat_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`,
  every completed telescope keeps `matched_clause_count = 2`,
  `first_mismatch_position = 2`, remains structurally connected, stays
  outside active-window qualification, stays outside self-containedness,
  stays outside historical reanchor with `max_lib_ref = 10`, and still fails
  connectivity.
  Consequence: the whole first clause-`0` sibling exact-screen pair on that
  active `reference` family is now frozen through both claim-side shells'
  deeper completion and first finer reason passes, so the next honest slice
  has to stay above another representative mismatch-`0` claim-side reland
  unless a later broader-backup comparison explicitly promotes a new branch.
- Scope: make the first finer reason split beneath the first clause-`0`
  sibling exact-screen `claim_flat_domain` dead shell on the active
  mismatch-`0` clause-`5` `reference` family explicit in the live docs and
  regression surface.
  Result: across all `18` clause-`3` / clause-`6` / terminal continuations
  beneath
  `claim_flat_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`,
  every completed telescope keeps `matched_clause_count = 2`,
  `first_mismatch_position = 2`, remains structurally connected, stays
  outside active-window qualification, stays outside self-containedness,
  stays outside historical reanchor with `max_lib_ref = 10`, and still fails
  connectivity.
  Consequence: the first clause-`0` sibling active `reference`-family
  claim-flat shell is now also explicitly frozen through first finer reason
  scope, which matches the already-frozen pair-level conclusion and leaves
  the next honest slice unchanged: stay above another representative
  mismatch-`0` claim-side reland unless a later broader-backup comparison
  explicitly promotes a new branch.
- Scope: split the first clause-`1` sibling exact-screen pair on that same
  active mismatch-`0` clause-`5` `reference` family one layer deeper across
  its clause-`2` sheets.
  Result: on
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / reference`,
  the `claim_flat_domain` and `claim_sharp_codomain` sheets each land the
  same smaller `4343 / 552 / 2271` tradeoff with first-mismatch counts
  `311 / 177 / 50 / 14`, `small_cluster = 3141 / 522 / 522 / 0`, and the
  targeted exact pair contracted only from `42` to `41`; the localized
  clause-`2` split is `14 / 15 / 12` on the claim-flat sheet and
  `15 / 14 / 12` on the claim-sharp sheet, while the sibling `reference`
  sheet stays neutral on the untouched `4331 / 553 / 2271` baseline with
  first-mismatch `312 / 177 / 50 / 14`,
  `small_cluster = 3132 / 522 / 522 / 0`, and the pair still at `42`.
  Consequence: that first clause-`1` sibling exact-screen pair on the active
  `reference` family again lives only on the two claim-side sheets, so the
  next honest probe had to move below the representative
  `claim_flat_domain` sheet rather than reopening the sibling
  `claim_sharp_codomain` or neutral `reference` sheets, rerunning the spent
  first clause-`0` sibling pair, or bouncing back to the exhausted
  `claim_eventual_domain / reference` row.
- Scope: localize that same first clause-`1` sibling exact-screen
  `claim_flat_domain` sheet on the active mismatch-`0` clause-`5`
  `reference` family through the exact-screen boundary, stacked
  remaining-one exact-summary relief, the matched clause-`6` /
  clause-`3` relands, and the released parent's clause-`6` completion
  profiles.
  Result: beneath
  `claim_flat_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / reference`,
  the exact-screen delta removes exactly one six-clause exact-prune capture
  with no introduced exact-prune or pruned-terminal family; stacking
  remaining-one exact-summary relief beneath that released parent keeps the
  same `4343 / 552`, the same `311 / 177 / 50 / 14`, and the same
  `small_cluster = 3141 / 522 / 522 / 0`, but sharpens the deeper
  zero-admitted tail from `2271` to `2268` while shaving exactly `3`
  remaining-one pruned prefixes; its three clause-`6` continuations reland
  the same matched smaller `4343 / 552` shell and differ only in the deeper
  zero-admitted tail `2270 / 2269 / 2268`; beneath the marginally best
  clause-`6` `reference` continuation, both `claim_flat_argument` and
  `claim_eventual_argument` reland the same matched smaller
  `4343 / 552 / 2271` shell; and each of
  `claim_next_codomain / claim_sharp_codomain / reference` collapses to the
  same dead `3`-generated / `0`-admitted completion summary with no bound,
  no best-rank profile, no survivor sketch, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  `matched_clause_count = 2` plus `first_mismatch_position = 2`.
  Consequence: that first clause-`1` sibling exact-screen active
  `reference`-family claim-flat shell is now spent through the same
  smaller-tradeoff ladder and dead completion profile as the earlier
  reference-family claim-flat shells, so the next honest probe had to move
  below that dead shell and localize a first finer reason split there rather
  than reopening the spent claim-flat shell, privileging the neutral
  `reference` sheet, rerunning the exhausted first clause-`0` sibling pair,
  or bouncing back to the exhausted `claim_eventual_domain / reference`
  row.
- Scope: localize the first finer reason split beneath that same first
  clause-`1` sibling exact-screen `claim_flat_domain` dead shell on the
  active mismatch-`0` clause-`5` `reference` family.
  Result: across all `18` clause-`3` / clause-`6` / terminal continuations
  beneath
  `claim_flat_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / reference`,
  every completed telescope keeps `matched_clause_count = 2`,
  `first_mismatch_position = 2`, remains structurally connected, stays
  outside active-window qualification, stays outside self-containedness,
  stays outside historical reanchor with `max_lib_ref = 10`, still fails
  connectivity, and keeps the same finer reason vector
  `(2, Some(2), true, false, false, 10, false, false)`.
  Consequence: that first clause-`1` sibling active `reference`-family
  claim-flat shell is now also explicitly frozen through first finer reason
  scope, so the next honest probe had to move sideways to the sibling
  `claim_sharp_codomain` sheet on that same pair rather than reopening the
  spent claim-flat shell, privileging the neutral `reference` sheet, or
  bouncing back to the exhausted earlier rows.
- Scope: localize that same first clause-`1` sibling exact-screen
  `claim_sharp_codomain` sheet on the active mismatch-`0` clause-`5`
  `reference` family through the exact-screen boundary, stacked
  remaining-one exact-summary relief, the matched clause-`6` /
  clause-`3` relands, and the released parent's clause-`6` completion
  profiles.
  Result: beneath
  `claim_flat_domain / claim_sharp_codomain / claim_sharp_codomain / claim_next_bridge / reference`,
  the exact-screen delta removes exactly one six-clause exact-prune capture
  with no introduced exact-prune or pruned-terminal family; stacking
  remaining-one exact-summary relief beneath that released parent keeps the
  same `4343 / 552`, the same `311 / 177 / 50 / 14`, and the same
  `small_cluster = 3141 / 522 / 522 / 0`, but sharpens the deeper
  zero-admitted tail from `2271` to `2268` while shaving exactly `3`
  remaining-one pruned prefixes; its three clause-`6` continuations reland
  the same matched smaller `4343 / 552` shell and differ only in the deeper
  zero-admitted tail `2270 / 2269 / 2268`; beneath the marginally best
  clause-`6` `reference` continuation, both `claim_flat_argument` and
  `claim_eventual_argument` reland the same matched smaller
  `4343 / 552 / 2271` shell; and each of
  `claim_next_codomain / claim_sharp_codomain / reference` collapses to the
  same dead `3`-generated / `0`-admitted completion summary with no bound,
  no best-rank profile, no survivor sketch, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  `matched_clause_count = 2` plus `first_mismatch_position = 2`.
  Consequence: that first clause-`1` sibling exact-screen active
  `reference`-family claim-sharp shell is now also spent through the same
  smaller-tradeoff ladder and dead completion profile as the earlier
  reference-family claim-sharp shells, so the next honest probe had to move
  below that dead shell and localize a first finer reason split there rather
  than reopening the spent claim-flat sibling, privileging the neutral
  `reference` sheet, rerunning the exhausted first clause-`0` sibling pair,
  or bouncing back to the exhausted `claim_eventual_domain / reference`
  row.
- Scope: localize the first finer reason split beneath that same first
  clause-`1` sibling exact-screen `claim_sharp_codomain` dead shell on the
  active mismatch-`0` clause-`5` `reference` family.
  Result: across all `18` clause-`3` / clause-`6` / terminal continuations
  beneath
  `claim_flat_domain / claim_sharp_codomain / claim_sharp_codomain / claim_next_bridge / reference`,
  every completed telescope keeps `matched_clause_count = 2`,
  `first_mismatch_position = 2`, remains structurally connected, stays
  outside active-window qualification, stays outside self-containedness,
  stays outside historical reanchor with `max_lib_ref = 10`, still fails
  connectivity, and keeps the same finer reason vector
  `(2, Some(2), true, false, false, 10, false, false)`.
  Consequence: the whole first clause-`1` sibling exact-screen pair on that
  active `reference` family is now frozen through both claim-side shells'
  deeper completion and first finer reason passes, so the next honest slice
  has to move sideways to the next clause-`1` sibling exact-screen pair on
  that same family, starting with
  `claim_flat_domain / reference / claim_next_bridge / reference`, rather
  than reopening the spent first clause-`1` pair, its neutral `reference`
  sheet, the exhausted first clause-`0` sibling pair, the exhausted
  `claim_eventual_domain / reference` row, or claim-safe controls.
- Scope: split the next clause-`1` sibling exact-screen pair on that same
  active mismatch-`0` clause-`5` `reference` family one layer deeper across
  its clause-`2` sheets.
  Result: on
  `claim_flat_domain / reference / claim_next_bridge / reference`, the
  `claim_flat_domain` and `claim_sharp_codomain` sheets each land the same
  smaller `4343 / 552 / 2271` tradeoff with first-mismatch counts
  `311 / 177 / 50 / 14`, `small_cluster = 3141 / 522 / 522 / 0`, and the
  targeted exact pair contracted only from `42` to `41`; the localized
  clause-`2` split is `14 / 15 / 12` on the claim-flat sheet and
  `15 / 14 / 12` on the claim-sharp sheet, while the sibling `reference`
  sheet stays neutral on the untouched `4331 / 553 / 2271` baseline with
  first-mismatch `312 / 177 / 50 / 14`,
  `small_cluster = 3132 / 522 / 522 / 0`, and the pair still at `42`.
  Consequence: that next clause-`1` sibling exact-screen pair on the active
  `reference` family again lives only on the two claim-side sheets, so the
  next honest probe had to move below the representative
  `claim_flat_domain` sheet rather than reopening the sibling
  `claim_sharp_codomain` or neutral `reference` sheets, rerunning the spent
  first clause-`1` sibling pair, the exhausted first clause-`0` sibling
  pair, or the exhausted `claim_eventual_domain / reference` row.
- Scope: localize that same next clause-`1` sibling exact-screen
  `claim_flat_domain` sheet on the active mismatch-`0` clause-`5`
  `reference` family through the exact-screen boundary, stacked
  remaining-one exact-summary relief, the matched clause-`6` /
  clause-`3` relands, the released parent's clause-`6` completion profiles,
  and the first finer reason split.
  Result: beneath
  `claim_flat_domain / reference / claim_flat_domain / claim_next_bridge / reference`,
  the exact-screen delta removes exactly one six-clause exact-prune capture
  with no introduced exact-prune or pruned-terminal family; stacking
  remaining-one exact-summary relief beneath that released parent keeps the
  same `4343 / 552`, the same `311 / 177 / 50 / 14`, and the same
  `small_cluster = 3141 / 522 / 522 / 0`, but sharpens the deeper
  zero-admitted tail from `2271` to `2268` while shaving exactly `3`
  remaining-one pruned prefixes; its three clause-`6` continuations reland
  the same matched smaller `4343 / 552` shell and differ only in the deeper
  zero-admitted tail `2270 / 2269 / 2268`; beneath the marginally best
  clause-`6` `reference` continuation, both `claim_flat_argument` and
  `claim_eventual_argument` reland the same matched smaller
  `4343 / 552 / 2271` shell; each of
  `claim_next_codomain / claim_sharp_codomain / reference` collapses to the
  same dead `3`-generated / `0`-admitted completion summary with no bound,
  no best-rank profile, no survivor sketch, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  `matched_clause_count = 2` plus `first_mismatch_position = 2`; and across
  all `18` clause-`3` / clause-`6` / terminal continuations, every deeper
  telescope keeps the same finer reason vector
  `(2, Some(2), true, false, false, 10, false, false)`.
  Consequence: that next clause-`1` sibling active `reference`-family
  claim-flat shell is now frozen through the same smaller-tradeoff ladder,
  dead completion profile, and first finer reason pass as the earlier
  reference-family claim-flat shells, so the next honest probe had to move
  sideways to the sibling `claim_sharp_codomain` sheet on that same pair
  rather than reopening the spent claim-flat shell, privileging the neutral
  `reference` sheet, rerunning the exhausted first clause-`1` sibling pair,
  the exhausted first clause-`0` sibling pair, or the exhausted
  `claim_eventual_domain / reference` row.
- Scope: localize that same next clause-`1` sibling exact-screen
  `claim_sharp_codomain` sheet on the active mismatch-`0` clause-`5`
  `reference` family through the exact-screen boundary, stacked
  remaining-one exact-summary relief, the matched clause-`6` /
  clause-`3` relands, the released parent's clause-`6` completion profiles,
  and the first finer reason split.
  Result: beneath
  `claim_flat_domain / reference / claim_sharp_codomain / claim_next_bridge / reference`,
  the exact-screen delta removes exactly one six-clause exact-prune capture
  with no introduced exact-prune or pruned-terminal family; stacking
  remaining-one exact-summary relief beneath that released parent keeps the
  same `4343 / 552`, the same `311 / 177 / 50 / 14`, and the same
  `small_cluster = 3141 / 522 / 522 / 0`, but sharpens the deeper
  zero-admitted tail from `2271` to `2268` while shaving exactly `3`
  remaining-one pruned prefixes; its three clause-`6` continuations reland
  the same matched smaller `4343 / 552` shell and differ only in the deeper
  zero-admitted tail `2270 / 2269 / 2268`; beneath the marginally best
  clause-`6` `reference` continuation, both `claim_flat_argument` and
  `claim_eventual_argument` reland the same matched smaller
  `4343 / 552 / 2271` shell; each of
  `claim_next_codomain / claim_sharp_codomain / reference` collapses to the
  same dead `3`-generated / `0`-admitted completion summary with no bound,
  no best-rank profile, no survivor sketch, the same
  `reference / eventual_lift / next_lift` `NeedsFallback` trio, and the same
  `matched_clause_count = 2` plus `first_mismatch_position = 2`; and across
  all `18` clause-`3` / clause-`6` / terminal continuations, every deeper
  telescope keeps the same finer reason vector
  `(2, Some(2), true, false, false, 10, false, false)`.
  Consequence: the whole next clause-`1` sibling exact-screen pair on that
  active `reference` family is now frozen through both claim-side shells'
  deeper completion and first finer reason passes, so the next honest slice
  has to stay above that fully exhausted active clause-`5`
  `reference` family and compare the next broader-backup branch rather than
  reopening the spent next clause-`1` pair, the spent first clause-`1`
  pair, the exhausted first clause-`0` sibling pair, the exhausted
  `claim_eventual_domain / reference` row, or looser claim-safe controls
  unless that broader comparison explicitly reranks them.
- Scope: compare that newly exhausted next clause-`1` sibling exact-screen
  `reference` family directly against the earlier tighter mismatch-`0`
  broader backup and the looser representative claim-safe backup.
  Result: both claim-side sheets on
  `claim_flat_domain / reference / claim_next_bridge / reference` reland the
  same tighter `4343 / 552 / 2268` shell with first-mismatch
  `311 / 177 / 50 / 14` and `small_cluster = 3141 / 522 / 522 / 0`, exactly
  matching the earlier representative mismatch-`0` claim-side broader backup;
  the looser claim-safe backup still lands only `4347 / 555 / 2277`, spending
  `+4` generated prefixes, `+3` clean-wall captures, `+3`
  `small_cluster` generated terminals, and `+9` zero-admitted captures while
  leaving first mismatch `0` at `312` and inflating first mismatch `1` to
  `179`.
  Consequence: exhausting the next clause-`1` sibling active `reference`
  family does not rerank the backup ordering. The next honest slice still has
  to stay on the next unresolved mismatch-`0` broader-backup branch rather
  than reopening looser claim-safe controls or any spent exact-screen
  `reference`-family sibling.
- Scope: compare the first unresolved mismatch-`0` broader-backup sibling on
  the active clause-`5` `claim_flat_codomain` family directly against the
  earlier tighter mismatch-`0` broader backup and the looser representative
  claim-safe backup.
  Result: both claim-side sheets on
  `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge / claim_flat_codomain`
  reland the same tighter `4343 / 552 / 2268` shell with first-mismatch
  `311 / 177 / 50 / 14` and `small_cluster = 3141 / 522 / 522 / 0`, exactly
  matching the earlier representative mismatch-`0` claim-side broader
  backup; the looser claim-safe backup still lands only
  `4347 / 555 / 2277`, spending `+4` generated prefixes, `+3` clean-wall
  captures, `+3` `small_cluster` generated terminals, and `+9`
  zero-admitted captures while leaving first mismatch `0` at `312` and
  inflating first mismatch `1` to `179`.
  Consequence: that first unresolved active `claim_flat_codomain` sibling
  does not rerank the backup ordering either. The next honest slice should
  move sideways to
  `claim_eventual_domain / reference / claim_next_bridge / claim_flat_codomain`
  rather than reopening looser claim-safe controls or any spent exact-screen
  `reference`-family sibling.
- Scope: compare the next unresolved mismatch-`0` broader-backup sibling on
  that same active clause-`5` `claim_flat_codomain` family directly against
  the earlier tighter mismatch-`0` broader backup and the looser
  representative claim-safe backup.
  Result: both claim-side sheets on
  `claim_eventual_domain / reference / claim_next_bridge / claim_flat_codomain`
  reland the same tighter `4343 / 552 / 2268` shell with first-mismatch
  `311 / 177 / 50 / 14` and `small_cluster = 3141 / 522 / 522 / 0`, exactly
  matching the earlier representative mismatch-`0` claim-side broader
  backup; the looser claim-safe backup still lands only
  `4347 / 555 / 2277`, spending `+4` generated prefixes, `+3` clean-wall
  captures, `+3` `small_cluster` generated terminals, and `+9`
  zero-admitted captures while leaving first mismatch `0` at `312` and
  inflating first mismatch `1` to `179`.
  Consequence: that second active `claim_flat_codomain` sibling does not
  rerank the backup ordering either. The next honest slice should move
  sideways to
  `claim_flat_domain / claim_next_codomain / claim_next_bridge / claim_flat_codomain`
  rather than reopening looser claim-safe controls, either spent
  `claim_eventual_domain` sibling on the active `claim_flat_codomain`
  family, or any spent exact-screen `reference`-family sibling.
- Scope: compare the first clause-`0` broader-backup sibling on that same
  active clause-`5` `claim_flat_codomain` family directly against the
  earlier tighter mismatch-`0` broader backup and the looser representative
  claim-safe backup.
  Result: both claim-side sheets on
  `claim_flat_domain / claim_next_codomain / claim_next_bridge / claim_flat_codomain`
  reland the same tighter `4343 / 552 / 2268` shell with first-mismatch
  `311 / 177 / 50 / 14` and `small_cluster = 3141 / 522 / 522 / 0`, exactly
  matching the earlier representative mismatch-`0` claim-side broader
  backup; the looser claim-safe backup still lands only
  `4347 / 555 / 2277`, spending `+4` generated prefixes, `+3` clean-wall
  captures, `+3` `small_cluster` generated terminals, and `+9`
  zero-admitted captures while leaving first mismatch `0` at `312` and
  inflating first mismatch `1` to `179`.
  Consequence: that first clause-`0` active `claim_flat_codomain` sibling
  does not rerank the backup ordering either. The next honest slice should
  move sideways to
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_flat_codomain`
  rather than reopening looser claim-safe controls, the spent first
  clause-`0` sibling on that active `claim_flat_codomain` family, either
  spent `claim_eventual_domain` sibling on that family, or any spent
  exact-screen `reference`-family sibling.
- Scope: compare the first clause-`1` broader-backup sibling on that same
  active clause-`5` `claim_flat_codomain` family directly against the
  earlier tighter mismatch-`0` broader backup and the looser representative
  claim-safe backup.
  Result: both claim-side sheets on
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_flat_codomain`
  reland the same tighter `4343 / 552 / 2268` shell with first-mismatch
  `311 / 177 / 50 / 14` and `small_cluster = 3141 / 522 / 522 / 0`, exactly
  matching the earlier representative mismatch-`0` claim-side broader
  backup; the looser claim-safe backup still lands only
  `4347 / 555 / 2277`, spending `+4` generated prefixes, `+3` clean-wall
  captures, `+3` `small_cluster` generated terminals, and `+9`
  zero-admitted captures while leaving first mismatch `0` at `312` and
  inflating first mismatch `1` to `179`.
  Consequence: that first clause-`1` active `claim_flat_codomain` sibling
  does not rerank the backup ordering either. The next honest slice should
  move sideways to
  `claim_flat_domain / reference / claim_next_bridge / claim_flat_codomain`
  rather than reopening looser claim-safe controls, the spent first
  clause-`1` sibling on that active `claim_flat_codomain` family, the spent
  first clause-`0` sibling on that active `claim_flat_codomain` family,
  either spent `claim_eventual_domain` sibling on that family, or any spent
  exact-screen `reference`-family sibling.
- Scope: compare the next clause-`1` broader-backup sibling on that same
  active clause-`5` `claim_flat_codomain` family directly against the
  earlier tighter mismatch-`0` broader backup and the looser representative
  claim-safe backup.
  Result: both claim-side sheets on
  `claim_flat_domain / reference / claim_next_bridge / claim_flat_codomain`
  reland the same tighter `4343 / 552 / 2268` shell with first-mismatch
  `311 / 177 / 50 / 14` and `small_cluster = 3141 / 522 / 522 / 0`, exactly
  matching the earlier representative mismatch-`0` claim-side broader
  backup; the looser claim-safe backup still lands only
  `4347 / 555 / 2277`, spending `+4` generated prefixes, `+3` clean-wall
  captures, `+3` `small_cluster` generated terminals, and `+9`
  zero-admitted captures while leaving first mismatch `0` at `312` and
  inflating first mismatch `1` to `179`.
  Consequence: that final active `claim_flat_codomain` sibling does not
  rerank the backup ordering either. The whole active clause-`5`
  `claim_flat_codomain` family is now frozen through all five
  nonrepresentative siblings, so the next honest slice has to move above
  that fully exhausted broader-backup lattice rather than reopening any
  spent `claim_flat_codomain` sibling, the spent active `reference` family,
  or looser claim-safe controls.
- Scope: combine the first alternate active-window qualification family with
  the representative tighter broader-backup shell on the same representative
  mismatch-`0` claim-side branch across both active clause-`5` families.
  Result: on both `claim_flat_codomain` and `reference`, and on both
  claim-side clause-`2` sheets, that combined family relands unsafe
  noncanonical `60 / 8` with `retained = 2`, `generated = 4439`,
  `partial_prefix_bar_failure = 544`, `incumbent_dominance = 110`,
  zero-admitted captures `2244`, first-mismatch `303 / 177 / 50 / 14`,
  `small_cluster = 2961 / 558 / 558 / 108`, and the same reopened `single`
  bucket at best overshoot `545 / 5278`. The claim-flat and claim-sharp
  sheets stay symmetric, only swapping the localized clause-`2` split
  `10 / 11 / 12` versus `11 / 10 / 12`, and the targeted active
  clause-`4` plus clause-`5` bucket contracts only from `44` to `43`.
  Consequence: stacking the tighter broader-backup shell under active-window
  sharpens the older unsafe active-window control by `+12 / -1 / -3` on
  generated / clean-wall / zero-admitted counts, but it still displaces the
  canonical `103 / 8` winner and keeps the `single` bucket reopened. The
  next honest slice should move sideways to the sibling self-contained plus
  tighter broader-backup combination rather than reopening the exhausted
  broader-backup lattice or the broader active-window family alone.
- Scope: combine the sibling self-contained qualification family with the
  representative tighter broader-backup shell on the same representative
  mismatch-`0` claim-side branch across both active clause-`5` families.
  Result: on both `claim_flat_codomain` and `reference`, and on both
  claim-side clause-`2` sheets, that combined family relands the same unsafe
  noncanonical `60 / 8` with `retained = 2`, `generated = 4439`,
  `partial_prefix_bar_failure = 544`, `incumbent_dominance = 110`,
  zero-admitted captures `2244`, first-mismatch `303 / 177 / 50 / 14`,
  `small_cluster = 2961 / 558 / 558 / 108`, and the same reopened `single`
  bucket at best overshoot `545 / 5278`. The claim-flat and claim-sharp
  sheets stay symmetric there too, only swapping the localized clause-`2`
  split `10 / 11 / 12` versus `11 / 10 / 12`, and the targeted active
  clause-`4` plus clause-`5` bucket again contracts only from `44` to `43`.
  Consequence: stacking the tighter broader-backup shell under
  self-containedness collapses to the same sharpened unsafe shell as the
  combined active-window family rather than opening a distinct repair class.
  Both combined alternate-qualification plus exact-shell families are now
  frozen together, so the next honest slice should move above that combined
  exact-shell lattice to the looser recombined active-window plus
  self-contained plus exact-shell family rather than reopening the exhausted
  broader-backup lattice or either combined family alone.
- Scope: combine the looser recombined active-window plus self-contained
  qualification family with the same representative tighter broader-backup
  shell across both active clause-`5` families.
  Result: on both `claim_flat_codomain` and `reference`, and on both
  claim-side clause-`2` sheets, that recombined exact-shell family relands
  the exact same unsafe noncanonical `60 / 8` with `retained = 2`,
  `generated = 4439`, `partial_prefix_bar_failure = 544`,
  `incumbent_dominance = 110`, zero-admitted captures `2244`,
  first-mismatch `303 / 177 / 50 / 14`,
  `small_cluster = 2961 / 558 / 558 / 108`, and the same reopened `single`
  bucket at best overshoot `545 / 5278`. The claim-flat and claim-sharp
  sheets stay symmetric there too, only swapping the localized clause-`2`
  split `10 / 11 / 12` versus `11 / 10 / 12`, and the targeted active
  clause-`4` plus clause-`5` bucket again contracts only from `44` to `43`.
  Consequence: the recombined exact-shell family does not open a fresh repair
  class either; it collapses to the same sharpened unsafe shell as the
  combined active-window and combined self-contained families. The whole
  combined exact-shell lattice is now frozen, so the next honest slice should
  move one layer broader to the full historical-reanchor plus active-window
  plus self-contained plus exact-shell family rather than reopening the
  exhausted broader-backup lattice or any narrower combined family alone.
- Scope: combine the full historical-reanchor plus active-window plus
  self-contained qualification triad with that same representative tighter
  broader-backup shell across both active clause-`5` families and both
  claim-side clause-`2` sheets.
  Result: on both `claim_flat_codomain` and `reference`, and on both
  claim-side clause-`2` sheets, that full qualification-triad exact-shell
  family relands the exact same unsafe noncanonical `60 / 8` with
  `retained = 2`, `generated = 4439`,
  `partial_prefix_bar_failure = 544`, `incumbent_dominance = 110`,
  zero-admitted captures `2244`, first-mismatch `303 / 177 / 50 / 14`,
  `small_cluster = 2961 / 558 / 558 / 108`, and the same reopened `single`
  bucket at best overshoot `545 / 5278`. The claim-flat and claim-sharp
  sheets stay symmetric there too, only swapping the localized clause-`2`
  split `10 / 11 / 12` versus `11 / 10 / 12`, and the targeted active
  clause-`4` plus clause-`5` bucket again contracts only from `44` to `43`.
  Consequence: adding historical reanchor on top of the sharpened exact-shell
  stack does not open a fresh repair class either; the full
  qualification-triad exact-shell family collapses to the same unsafe shell
  as the narrower combined exact-shell families. The next honest slice
  therefore has to move above that whole qualification-triad exact-shell
  lattice rather than reopening the spent broader-backup lattice or any
  narrower combined family alone.
- Scope: localize that full historical-reanchor plus active-window plus
  self-contained plus exact-shell family against the canonical baseline to see
  whether it hides any off-target delta.
  Result: under either active clause-`5` family, the focused claim-flat
  sheet removes the same four targeted remaining-two exact-prune cells at
  `3 / 2 / 2 / 2` and the matching remaining-one pruned-prefix families at
  `9 / 6 / 6 / 6`, while the focused claim-sharp sheet keeps the same
  localized shape but swaps the heavier `3`-capture / `9`-pruned share onto
  the claim-sharp `claim_next_bridge` cell instead. No off-target exact-prune
  capture family appears, and no off-target pruned-prefix family appears,
  under either active clause-`5` family.
  Consequence: the full qualification-triad exact-shell family is spent at
  delta scope too. It stays confined to the same targeted active bucket as
  the narrower combined exact-shell families, so the next honest slice has to
  move above that exhausted lattice rather than retrying another
  recombination of the same representative exact-shell shell.
- Scope: localize the first pre-materialization summary/materialization
  follow-on above that full qualification-triad exact-shell lattice on the
  representative active clause-`5` `reference` claim-flat shell, using
  `claim_try_summary_prune_before_materialization(...)` plus immediate
  terminal-group materialization on every remaining-one pruned prefix removed
  by the full-triad opening.
  Result: the full-triad opening removes `27` baseline remaining-one
  pruned prefixes. Only `3` of the removed
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes pre-prune before materialization, and those pre-prunes register
  `0` terminal-prefix-bar prunes and `0` terminal-rank prunes. The other `24`
  removed prefixes survive summary pruning and materialize the same unsafe
  clause-kappa `8` rank at best overshoot `545 / 5278`: `6` each on
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  and
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / reference / reference`
  and
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / reference`
  and
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / reference / reference`.
  Consequence: `claim_try_summary_prune_before_materialization(...)` does not
  restore canonical `103 / 8` above the qualification-triad exact-shell
  lattice; it mostly carries the same unsafe sharpened shell forward. Freeze
  that first pre-materialization follow-on too, and move the next honest slice
  above it rather than reopening the spent broader-backup lattice, any
  narrower combined exact-shell family, or this pre-materialization branch.
- Scope: localize the first exact-terminal follow-on above that same
  pre-materialization branch on the representative active clause-`5`
  `reference` claim-flat shell by replaying
  `exact_terminal_prefix_bound_decision(...)` on each of the `27`
  remaining-one prefixes removed by the full qualification-triad exact-shell
  opening.
  Result: `24` of those removed prefixes still cache `CanClearBar` terminal
  summaries across the same four `6`-prefix labels that later materialize the
  unsafe clause-kappa `8` / overshoot `545 / 5278` shell. Only the focused
  `3`
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes cache no terminal bound, and the live exact-terminal override
  leaves them at `Unknown` rather than closing them as clean
  `CannotClearBar` proofs.
  Consequence: `exact_terminal_prefix_bound_decision(...)` does not isolate a
  fresh repair above the qualification-triad exact-shell lattice either. Freeze
  that exact-terminal follow-on too, and move the next honest slice above it
  rather than reopening the spent broader-backup lattice, any narrower
  combined exact-shell family, the pre-materialization branch, or this
  exact-terminal branch itself.
- Scope: localize the first remaining-one rank-profile follow-on above that
  same exact-terminal branch on the representative active clause-`5`
  `reference` claim-flat shell by inspecting the cached compact summaries that
  survive the exact-terminal replay on those same `27` removed
  remaining-one prefixes.
  Result: the surviving `24` `CanClearBar` groups all already share the same
  summary-stage rank profile: `generated = 3`, `admitted = 3`, a present
  terminal bound, and the same
  `best_accept_rank = best_accept_primary_rank = 8 / 545 / 5278`.
  `terminal_prefix_rank_prune_count(...)` stays `0` on those survivors even
  when same-primary relief is disabled, so the live branch is not hanging on a
  same-primary exception. The only remaining split is compact survivor-sketch
  cardinality: within each of the same four live `6`-prefix labels, `4`
  cached summaries keep exactly `1` survivor while the other `2` keep `3`.
  Consequence: summary-stage rank pruning and same-primary relief are not the
  missing lever above the exact-terminal branch either. Freeze that
  rank-profile follow-on too, and move the next honest slice to compact
  survivor-sketch / materialization behavior on those same `24` `CanClearBar`
  groups rather than reopening the pre-materialization branch, same-primary
  relief, rank-prune logic, or the exact-terminal branch itself.
- Scope: localize the first compact survivor-sketch / materialization
  follow-on above that same rank-profile branch on the representative active
  clause-`5` `reference` claim-flat shell by comparing
  `materialize_terminal_prefix_group(...)` against the live no-cache compact
  path on the same `24` `CanClearBar` groups.
  Result: all `24` `CanClearBar` groups materialize the same retained
  candidates from cached survivor sketches as they do on the live no-cache
  compact path. Every one of those groups keeps `generated = 3`,
  `admitted = 3`, and `best_accept_rank = 8 / 545 / 5278`; within each of the
  same four live `6`-prefix labels, `4` groups materialize `1` retained
  candidate while the other `2` materialize `3`; cached-summary reuse
  accounts for all `24` of those materializations, and the no-cache
  comparison reroutes the same `24` groups through the live direct compact
  path.
  Consequence: compact survivor-sketch reuse and live compact materialization
  are not the missing lever above the exact-terminal branch either. Freeze
  that materialization follow-on too, and move the next honest slice above it
  to retained-candidate / terminal-family identity inside the remaining
  `1`-versus-`3` split rather than reopening the pre-materialization branch,
  same-primary relief, rank-prune logic, the exact-terminal branch, or
  materialization parity itself.
- Scope: localize the retained-candidate / terminal-family identity inside the
  remaining `1`-versus-`3` split above that same materialization branch on
  the representative active clause-`5` `reference` claim-flat shell.
  Result: across each of the same four live `6`-prefix labels, the two
  non-reference clause-`6` continuations `claim_next_codomain` and
  `claim_sharp_codomain` each keep only the retained `reference` terminal,
  while the clause-`6` `reference` continuation keeps the full retained
  `reference + eventual_lift + next_lift` family. The split stays symmetric
  across both clause-`3` branches, so each live label now breaks cleanly as
  `2` one-survivor `claim_next_codomain` continuations, `2` one-survivor
  `claim_sharp_codomain` continuations, and `2` three-survivor
  `reference` continuations.
  Consequence: the remaining `1`-versus-`3` split is now localized exactly to
  clause-`6` identity rather than to cached-vs-live materialization behavior
  or clause-`3` identity. The next honest slice has to move above candidate
  identity to the retained accept-rank / ordering profile inside the
  clause-`6` `reference` three-survivor shell rather than reopening the
  non-reference clause-`6` controls, the exact-terminal branch, or
  materialization parity.
- Scope: localize the residual proof-close `single` bucket above the now-spent
  qualification-triad exact-shell remaining-one lattice by capturing the fully
  scored step-`15` `single`-bucket candidate and comparing it against the
  already-localized proof-close incumbent-pruned groups.
  Result: the lone fully scored `single`-bucket candidate is the canonical
  accepted `reference(15)` completion on the seven-clause reference prefix at
  `103 / 8` with `bit_kappa = 229` and overshoot `115657 / 21112`. The
  remaining three proof-close incumbent-pruned groups stay on weaker
  same-primary `103 / 8` reference-terminal siblings at `bit_kappa = 236` on
  `clause-0 claim_flat_domain`,
  `clause-2 claim_flat_domain plus anchor-11 exact-argument`, and
  `clause-5 claim_flat_codomain`, each on a local `3`-generated /
  `1`-admitted surface.
  Consequence: the residual `single`-bucket proof-close family is now fully
  explicit. Exact-family or subset-local same-primary relief there only
  converts those three prunes into extra fully scored non-winning reference
  terminals, grows the `single` bucket from `1` to `4`, and leaves
  `4331 / 553` plus `small_cluster = 3132 / 522 / 522 / 0` unchanged. Freeze
  that proof-close family too and move the next honest slice above it rather
  than reopening same-primary relief on the isolated single pocket.

## 2026-04-14

- Scope: test the first fresh code-side family above the frozen
  qualification-triad exact-shell remaining-one lattice and the residual
  proof-close `single` pocket by checking whether the unsafe surviving groups
  route through the generic focus-aligned / deprioritized competition gate.
  Result: on the representative active clause-`5` `reference` claim-flat
  shell, the surviving `24` unsafe remaining-one groups bypass
  `terminal_completion_can_compete_for_acceptance(...)` entirely. All `24`
  arrive through `TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand` as
  the full `reference + eventual_lift + next_lift` trio, while only the
  focused `3`
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes still pre-prune earlier.
  Consequence: the generic focus-aligned / deprioritized competition gate is
  not the next repair either. The next honest slice should move to
  claim-open-band terminal filtering / competition on those exact `24`
  unsafe groups rather than reopening same-primary relief, the generic
  competition gate, or the spent remaining-one branches.
- Scope: split that claim-open-band family by whole terminal-label subsets on
  the same representative active clause-`5` `reference` claim-flat shell.
  Result:
  - any subset that still keeps `reference` relands unsafe noncanonical
    `60 / 8` reference-terminal winners on the two
    `claim_eventual_domain / claim_next_codomain / {claim_flat_domain, claim_sharp_codomain} / reference / reference`
    shells
  - `reference` alone lands `2408 / 270 / 283` and reopens the isolated
    `single` bucket to `814 / 289 / 289 / 283 / 2` at best overshoot
    `545 / 5278`
  - `reference + eventual_lift` and `reference + next_lift` each reland the
    same broader unsafe control at `3221 / 272 / 305`
  - the full `reference + eventual_lift + next_lift` trio stays on the
    original `4439 / 544 / 2244` shell with
    `small_cluster = 2961 / 558 / 558 / 108`
  - the tighter lift-only single-label subsets are still unsafe controls too:
    `eventual_lift` alone and `next_lift` alone each collapse only to
    `568 / 347 / 7`, leaving one retained noncanonical
    `74 / 8 / 270 / 19563 / 10556` reference winner on the
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain / reference / reference`
    shell plus a smaller reopened `single` bucket
  - the lift-only pair `eventual_lift + next_lift` is looser again at
    `616 / 361 / 14`, retaining the same weaker `74 / 8 / 270 / 19563 / 10556`
    reference nonwinner plus an unsafe
    `60 / 8 / 288 / 545 / 5278` `eventual_lift` winner on that same
    `claim_flat_domain / reference / reference` shell
  Consequence: whole-label claim-open-band terminal filtering is spent as an
  unsafe control too. The next honest slice has to move below that subset
  lattice to prefix-local claim-open-band filtering / competition on the
  surviving `reference / reference` shells rather than reopening the generic
  competition gate or any whole-label subset.
- Scope: localize that claim-open-band family below the whole-label subset
  lattice by filtering only the two surviving
  `claim_eventual_domain / claim_next_codomain / {claim_flat_domain, claim_sharp_codomain} / reference / reference`
  shells on the same representative active clause-`5` `reference`
  claim-flat shell.
  Result:
  - local single-label subsets `reference`, `eventual_lift`, and `next_lift`
    plus the lift-only pair `eventual_lift + next_lift` all stay unsafe, now
    rerouting the noncanonical winner onto the sibling
    `claim_eventual_domain / claim_next_codomain / {claim_flat_domain, claim_sharp_codomain} / claim_next_bridge / reference`
    shells at `60 / 8 / 295 / 545 / 5278`
  - those rerouted local controls land `4415 / 544` with
    `incumbent_dominance = 113` or `119` on the single-label subsets and
    `4427 / 544 / 116` on the lift-only pair
  - the reopened `single` bucket there ranges from
    `13 / 13 / 13 / 13 / 2` through `15 / 15 / 15 / 11 / 2`
  - the rerouted local controls also keep the smaller unsafe
    `small_cluster = 2889 / 486 / 486 / 108`
  - local subsets that keep `reference` together with either lift still
    reland the original unsafe
    `claim_eventual_domain / claim_next_codomain / {claim_flat_domain, claim_sharp_codomain} / reference / reference`
    winners at `60 / 8 / 277 / 545 / 5278`, landing `4427 / 544 / 128` for
    either mixed pair and the original `4439 / 544 / 110` for the full trio
  Consequence: the first targeted-shell prefix-local family is spent as an
  unsafe control too. The next honest slice has to move below it to
  prefix-local claim-open-band filtering / competition on the sibling
  `claim_next_bridge / reference` shells rather than reopening the whole-label
  subset lattice or the now-spent targeted `reference / reference` shells.
- Scope: localize that claim-open-band family one layer deeper below the
  spent targeted `reference / reference` shells by filtering only the sibling
  `claim_eventual_domain / claim_next_codomain / {claim_flat_domain, claim_sharp_codomain} / claim_next_bridge / reference`
  shells on the same representative active clause-`5` `reference`
  claim-flat shell.
  Result:
  - the targeted sibling shell is asymmetric before filtering at `15`
    removed prefixes, split as `9` flat and `6` sharp, because the focused
    `3` no-bound
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
    groups still live only on the flat branch while the sharp sibling stays on
    the `6` cached `CanClearBar` groups
  - every local subset still reroutes back onto the same unsafe
    `60 / 8 / 277 / 545 / 5278` reference-terminal winners on the two
    `claim_eventual_domain / claim_next_codomain / {claim_flat_domain, claim_sharp_codomain} / reference / reference`
    shells
  - single-label subsets `reference`, `eventual_lift`, and `next_lift` land
    `4409 / 544` with `incumbent_dominance = 122 / 114 / 114`,
    `small_cluster = 2880 / 486 / 486 / 108`, and the reopened
    `single = 15 / 12 / 12 / 14 / 2`
  - the lift-only pair `eventual_lift + next_lift` lands `4424 / 544 / 118`
    on that same unsafe `reference / reference` shell, while
    `reference + eventual_lift` and `reference + next_lift` each land
    `4424 / 544 / 134`; only the full trio returns the original
    `4439 / 544 / 110` shell with
    `small_cluster = 2961 / 558 / 558 / 108`
  Consequence: the sibling `claim_next_bridge / reference` shell is spent as
  another unsafe control too. The next honest slice has to move below that
  full `15`-prefix shell to the focused `3` no-bound flat-shell prefixes on
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  rather than reopening the spent targeted `reference / reference` shells,
  the full sibling shell, or the whole-label subset lattice.
- Scope: move below that focused-three subset lattice to the earlier
  pre-prune / exact-terminal path on those same
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes.
  Result:
  - the same focused `3` prefixes reland a single dead
    `3`-generated / `0`-admitted no-bound control
  - pre-materialization consumes that summary directly with zero
    terminal-prefix-bar prunes, zero terminal-rank prunes, and no cached
    survivor sketch left resident
  - direct terminal assessment on those same prefixes matches the same dead
    shell too: exactly `3` generated candidates, `0` admitted candidates, and
    no bound that can clear the bar
  - live exact-terminal replay stays `Unknown` with the same
    `3`-generated / `0`-admitted no-bound summary and no retained
    accept-rank or primary-rank profile
  Consequence: the focused-three pre-prune / exact-terminal branch is another
  spent unsafe control. The next honest slice should move below it to direct
  connectivity / exact-admissibility makeup on those same `3` prefixes rather
  than reopening any terminal-label subset, the full sibling shell, the spent
  targeted `reference / reference` shells, or the broader exact-shell lattice.
- Scope: move below that focused-three pre-prune / exact-terminal control to
  direct connectivity / exact-admissibility makeup on those same
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes.
  Result:
  - each of those same `3` exact-terminal-`Unknown` prefixes still generates
    the same `reference + eventual_lift + next_lift` terminal trio under a
    full direct terminal summary
  - all `9` generated terminal candidates die at connectivity before exact
    admissibility; each prefix stays `3` disconnected,
    `0` admissibility-rejected, and `0` admitted
  - none of those prefixes produces a bound, a best accept rank, or a best
    primary rank
  Consequence: direct exact-admissibility is another spent dead control on the
  focused-three flat-shell branch. The next honest slice should move below it
  to terminal connectivity identity / fallback makeup on those same `3`
  prefixes rather than reopening any terminal-label subset, the full sibling
  shell, the spent targeted `reference / reference` shells, or the broader
  exact-shell lattice.
- Scope: move below that focused-three direct-connectivity control to terminal
  connectivity identity / fallback makeup on those same
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes.
  Result:
  - each of those same `3` exact-terminal-`Unknown` prefixes still exposes
    only the same `reference + eventual_lift + next_lift` trio
  - all `9` generated terminal candidates first classify as `NeedsFallback`,
    while none classify as `PruneDisconnected` or `KeepWithoutFallback`
  - every fallback then fails with the same connectivity witness:
    `connected = true`, `references_active_window = false`,
    `self_contained = false`, `max_lib_ref = 10`, and
    `historical_reanchor = false`
  Consequence: terminal connectivity identity is another spent dead control on
  the focused-three flat-shell branch. The next honest slice should move below
  it to the historical-reanchor blocker / first-mismatch reason makeup on
  those same `3` prefixes rather than reopening any terminal-label subset, the
  full sibling shell, the spent targeted `reference / reference` shells, or
  the broader exact-shell lattice.
- Scope: move below that focused-three terminal-connectivity control to the
  historical-reanchor blocker / first-mismatch reason makeup on those same
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
  prefixes.
  Result:
  - all `9` completed telescopes across those same `3` exact-terminal-`Unknown`
    prefixes and their `reference + eventual_lift + next_lift` terminal
    families share `matched_clause_count = 2` and
    `first_mismatch_position = 2`
  - every completed telescope remains structurally connected, stays outside
    active-window qualification, stays outside self-containedness, stays
    outside historical reanchor with `max_lib_ref = 10`, and still fails
    connectivity
  Consequence: the focused-three flat-shell residue is another spent dead
  control too. Its blocker sits entirely above later continuation identity, so
  the next honest slice should move above that fully spent branch to a
  different code-side step-`15` repair family rather than reopening
  terminal-label subsets, exact-terminal replay, direct-connectivity,
  historical-reanchor reason splits, the full sibling shell, the spent
  targeted `reference / reference` shells, or the broader exact-shell
  lattice.
- Scope: move above that spent claim-open-band terminal-filter branch to the
  route selector feeding `TerminalPrefixClauseCandidates::ClaimAdmittedOpenBand(...)`.
  Result:
  - the route selector is now pinned as a caller-level admissibility/surface
    gate rather than a prefix-local repair family
  - `PrefixLegalityCache::filter_claim_open_band_terminal_clauses(...)` only
    opens the open-band dispatch while the live path stays on default
    `DesktopClaimShadow` with `focus_family = None`,
    `package_policies = PackagePolicies::default()`, and no late
    family-surface override away from `ClaimGeneric`
  - adding any family focus, any nondefault package policy, or any late
    family-surface override closes that route before
    `ClaimAdmittedOpenBand(...)` can fire
  Consequence: the route selector is now another spent mechanism-level
  control. The next honest slice should move above it to the caller-side
  admissibility/surface setup that keeps the surviving `24` unsafe groups on
  that default claim-generic path rather than reopening any spent subset,
  prefix-local filter shell, or focused-three continuation beneath it.
- Scope: move above that route selector to the live caller-side
  admissibility/surface setup on the actual step-`15` claim path.
  Result:
  - on the actual `14`-step claim prefix,
    `strict_admissibility_for_mode(15, 2, ..., DesktopClaimShadow)` keeps
    `focus_family = None`,
    `package_policies = PackagePolicies::default()`, and
    `historical_anchor_ref = Some(10)`
  - `discovery_enumeration_context(...)` and `claim_step_open_diagnostics(...)`
    keep `LateFamilySurface::ClaimGeneric` on that same live step-`15` path,
    with `kappa = 8 .. 8` and all three claim widening bands active
  - the existing route-selector regression and removed-prefix open-band
    regression still pass unchanged under that pinned caller setup
  Consequence: the caller-side setup is not hiding a stray family focus,
  package-policy bump, or late-surface override either. The next honest slice
  should move above it to the derivation that keeps step `15` on that default
  claim-generic path rather than reopening any spent selector-level or
  continuation-level control beneath it.
- Scope: move above that pinned caller-side setup to the underlying
  claim-step-open debt snapshot and the live default claim-generic
  band-`8` terminal trio.
  Result:
  - on the actual `14`-step claim prefix, `claim_step_open_diagnostics(...)`
    now explicitly pins
    `anchor_policy = Modal`,
    `claim_debt_axes = 8 / 8` with
    `path = 0`,
    `trunc = 0`,
    `coupling = 2`,
    `support = 2`,
    `modal = 0`,
    `temporal = 1`,
    `reanchor = 2`,
    `closure = 3`, and
    `package_flags = operator_bundle false / hilbert_functional false / temporal_shell true`
  - on the canonical seven-clause reference prefix, the live default
    `filter_claim_open_band_terminal_clauses(...)` path keeps the full
    `reference + eventual_lift + next_lift` claim-generic `kappa = 8`
    terminal trio rather than a narrower subset
  Consequence: the live caller path is now explicit through both its temporal
  debt window and its full band-`8` trio. The next honest slice should move
  above that stack to the derivation that keeps the surviving `24` unsafe
  groups on that default claim-generic trio rather than reopening any spent
  selector-level or continuation-level control beneath it.
- Scope: move above that caller path to the admissibility derivation that
  keeps step `15` on the default claim-generic route.
  Result:
  - on the actual `14`-step claim library, the raw structural-debt route still
    resolves to `TemporalShell` with modal anchor `Some(10)`
  - a same-mode structural-focus replay would keep
    `focus_family = TemporalShell` with
    `package_policies.temporal_shell = Prefer`
  - `claim_guarded_early_focus_family(15, ...)` returns `None` because the
    guarded structural-focus shortcut only applies on steps `4 ..= 8`
  - `strict_admissibility_for_mode(15, 2, ..., DesktopClaimShadow)` therefore
    regression-matches `claim_strict_admissibility(...)` rather than
    `structural_focus_strict_admissibility(...)`, while keeping the same
    `kappa = 8 .. 8`, the same modal/temporal budget, and the same anchor
  Consequence: the live step-`15` claim route is not hiding a late structural
  temporal-shell reroute either. The next honest slice should move above this
  dispatch to the claim-profile helpers that still produce the live no-focus
  `ClaimGeneric` surface, starting with `claim_debt_axes_for_step(...)` and
  `claim_historical_anchor_ref(...)` rather than reopening any spent
  selector-level or continuation-level control beneath it.
- Scope: move above that admissibility dispatch to the first claim-profile
  helper pair inside `claim_strict_admissibility(...)`.
  Result:
  - on the divergent step-`13` operator-bundle claim library,
    `claim_debt_axes_for_step(14, ...)` now regression-backs the helper-side
    band promotion directly: the raw claim axes stay at `7 / 7`, the helper
    promotes them to `9 / 9`, and the underlying debt pressures stay
    unchanged
  - on the actual step-`14` claim library, `claim_debt_axes_for_step(15, ...)`
    now regression-backs the live temporal-shell helper output directly too:
    it leaves the raw claim axes unchanged at `8 / 8` with
    `path = 0`,
    `trunc = 0`,
    `coupling = 2`,
    `support = 2`,
    `modal = 0`,
    `temporal = 1`,
    `reanchor = 2`, and
    `closure = 3`
  - on that same live step-`14` claim library,
    `claim_historical_anchor_ref(...)` now regression-backs the live anchor
    choice directly too: `ClaimAnchorPolicy::Modal` keeps the modal anchor at
    `Some(10)`
  - targeted `pen-type` regressions now pass for the step-`14` helper
    promotion, the step-`15` helper outputs, and the existing step-`15`
    dispatch boundary
  Consequence: the live no-focus step-`15` claim-generic route is not hiding a
  helper-local band promotion or anchor override either. The next honest
  slice should move above that helper pair to the remaining claim-profile
  helper stack inside `claim_strict_admissibility(...)`, starting with
  `claim_include_modal(...)` and `claim_include_temporal(...)` rather than
  reopening any spent selector-level, continuation-level, or axes/anchor
  explanation beneath it.
- Scope: move above that first claim-profile helper pair to the
  modal/temporal inclusion layer inside `claim_strict_admissibility(...)`.
  Result:
  - on the divergent step-`13` operator-bundle claim library, after
    `claim_debt_axes_for_step(14, ...)` promotes the claim band to `9 / 9`,
    `claim_include_modal(...)` and `claim_include_temporal(...)` both stay
    closed with `modal_pressure = 0` and `temporal_pressure = 0`
  - on the actual live step-`14` claim library, the step-`15`
    temporal-shell snapshot opens both helpers directly:
    `claim_include_modal(...)` and `claim_include_temporal(...)` both return
    `true` on the same `8 / 8` axes because `temporal_pressure = 1` even
    though `modal_pressure = 0`
  - targeted `pen-type` regressions now pass for the step-`14`
    operator-bundle helper control, the live step-`15` modal/temporal helper
    outputs, and the existing step-`15` helper/dispatch boundary
  Consequence: the live no-focus step-`15` claim-generic route is not hiding a
  helper-local modal or temporal widening override beyond the already-pinned
  temporal debt pressure either. The next honest slice should move above that
  helper layer to the remaining claim-profile sizing surface inside
  `claim_strict_admissibility(...)`, starting with `claim_max_expr_nodes(...)`
  and `claim_max_path_dimension(...)` rather than reopening any spent
  selector-level, continuation-level, axes/anchor, or modal/temporal
  explanation beneath it.
- Scope: move above that modal/temporal helper layer to the claim-profile
  sizing surface inside `claim_strict_admissibility(...)`.
  Result:
  - on the divergent step-`13` operator-bundle claim library, after
    `claim_debt_axes_for_step(14, ...)` promotes the claim band to `9 / 9`,
    `claim_max_expr_nodes(...)` now regression-backs the wide seven-node
    envelope directly while `claim_max_path_dimension(...)` still stays
    closed at `0`
  - on the actual live step-`14` claim library, the step-`15`
    temporal-shell `8 / 8` claim profile now regression-backs that same
    claim-size surface directly too:
    `claim_max_expr_nodes(...) = 7` and
    `claim_max_path_dimension(...) = 0`
  - targeted `pen-type` regressions now pass for the divergent step-`14`
    claim-size helper control, the live step-`15` claim-size helper outputs,
    and the existing step-`15` helper/dispatch boundary
  Consequence: the live no-focus step-`15` claim-generic route is not hiding a
  helper-local expression-budget or path-budget split either. The next honest
  slice should move above that sizing layer to the residual claim-profile
  envelope inside `claim_strict_admissibility(...)`, starting with
  `claim_include_trunc(...)` and `quota_per_bucket()` rather than reopening
  any spent selector-level, continuation-level, axes/anchor, modal/temporal,
  or sizing explanation beneath it.
- Scope: move above that sizing layer to the claim-profile truncation/quota
  surface inside `claim_strict_admissibility(...)`.
  Result:
  - on the divergent step-`13` operator-bundle claim library, after
    `claim_debt_axes_for_step(14, ...)` promotes the claim band to `9 / 9`,
    `claim_include_trunc(...)` now regression-backs the closed truncation gate
    directly with `max_path_dimension = 0` and `truncated_entries = 0`, while
    `quota_per_bucket()` still stays wide at `64` by inheriting the raw
    operator-bundle package pressure
  - on the actual live step-`14` claim library, the step-`15`
    temporal-shell `8 / 8` claim profile now regression-backs that same
    helper surface directly too:
    `claim_include_trunc(...) = false` with
    `max_path_dimension = 0` and `truncated_entries = 0`, while
    `quota_per_bucket() = 64` by inheriting the raw temporal-shell package
    pressure
  - targeted `pen-type` regressions now pass for the divergent step-`14`
    trunc/quota helper control, the live step-`15` trunc/quota helper
    outputs, and the existing step-`15` helper/dispatch boundary
  Consequence: the live no-focus step-`15` claim-generic route is not hiding a
  helper-local truncation reopening or quota split either. The next honest
  slice should move above that trunc/quota layer to the remaining package-
  requirement surface inside `claim_strict_admissibility(...)`, starting with
  the `require_*_package` flags rather than reopening any spent selector-
  level, continuation-level, axes/anchor, modal/temporal, sizing, or
  trunc/quota explanation beneath it.
- Scope: move above that trunc/quota layer to the helper-local package-
  requirement surface inside `claim_strict_admissibility(...)`.
  Result:
  - on the divergent step-`13` operator-bundle claim library, after
    `claim_debt_axes_for_step(14, ...)` promotes the claim band to `9 / 9`,
    the raw debt snapshot still requires only the operator-bundle package,
    while `claim_strict_admissibility(...)` keeps
    `PackagePolicies::default()` and every `require_*_package` flag closed
  - on the actual live step-`14` claim library, the step-`15`
    temporal-shell `8 / 8` claim profile still requires only the
    temporal-shell package at the raw debt layer, while
    `claim_strict_admissibility(...)` again keeps
    `PackagePolicies::default()` and every `require_*_package` flag closed
  - targeted `pen-type` regressions now pass for the divergent step-`14`
    package-requirement control, the live step-`15` package-requirement
    control, and the earlier step-`14`/step-`15` helper stack
  Consequence: the live no-focus step-`15` claim-generic route is not hiding a
  helper-local hard package requirement either; raw package pressure stays
  diagnostic-only on that route. The next honest slice should move beyond the
  full helper-local package surface to the downstream package-policy consumer
  view on the assembled claim profile, starting with
  `required_focus_family()` and `policy_for(...)` rather than reopening any
  spent selector-level, continuation-level, axes/anchor, modal/temporal,
  sizing, trunc/quota, or helper-local package-requirement explanation
  beneath it.
- Scope: move above that helper-local package-requirement surface to the
  downstream package-policy consumer accessors on the assembled claim
  profile.
  Result:
  - on the divergent step-`13` operator-bundle claim library after
    `claim_debt_axes_for_step(14, ...)` promotes the claim band to `9 / 9`,
    `required_focus_family()` stays `None` and `policy_for(...)` stays
    `Allow` for every structural family
  - on the actual live step-`14` claim library, the step-`15`
    temporal-shell `8 / 8` claim profile keeps that same no-focus consumer
    view too: `required_focus_family()` stays `None` and
    `policy_for(...)` stays `Allow` for every structural family
  - on the bypassed step-`15` structural temporal-shell route,
    `focus_family = Some(TemporalShell)` stays visible, but
    `required_focus_family()` still stays `None` because the only nondefault
    downstream policy is `policy_for(TemporalShell) = Prefer`
  - targeted `pen-type` regressions now pass for both no-focus claim
    profiles and the bypassed structural temporal-shell control
  Consequence: the assembled DesktopClaimShadow claim profile is not hiding a
  downstream required-family or forbidden-family gate either. The next honest
  slice should move above those accessors to the first actual search-side
  consumer of that package-policy view, starting with
  `PrefixFamilySummary::for_admissibility(...)` rather than reopening any
  spent selector-level, continuation-level, axes/anchor, modal/temporal,
  sizing, trunc/quota, helper-local package-requirement, or accessor-level
  explanation beneath it.
- Scope: move above those package-policy consumer accessors to the first
  actual search-side family-summary / work-item layer on the assembled
  claim profile.
  Result:
  - on the actual live step-`15` claim prefix,
    `PrefixFamilySummary::for_admissibility(...)` short-circuits on
    `focus_family = None`, so `PrefixLegalityCache::insert_root(...)` stores
    no family filter, `family_option_count(...)` stays absent, and
    `filter_active_window_clauses(...)` stays disabled on the default claim-
    generic route
  - on that same seven-clause reference prefix replayed through the bypassed
    temporal-shell focused route with
    `focus_family = Some(TemporalShell)` and
    `policy_for(TemporalShell) = Prefer`, the same search-side layer
    re-enables one cached family option and the active-window consumer path,
    even though the exact step-`15` terminal catalog still stays fully
    compatible with that focused replay
  - on the live default route, `create_online_prefix_work_item(...)` keeps
    `remaining_family_options = u8::MAX`, `filtered_next_clauses = None`, and
    the full step-`8` catalog before later claim-open-band filtering
  - targeted `pen-search` regressions now pass for the low-level family-
    summary short-circuit, the live step-`15` work-item consumer, the
    existing claim-open-band route-selector baseline, and the existing live
    caller-setup baseline
  Consequence: the assembled no-focus claim profile is not hiding a search-
  side family summary or early active-window narrowing either. The next
  honest slice should move above this first work-item layer to the frontier
  ordering / queue-pop consumer, starting with
  `prefix_frontier_work_key(...)` and `pop_best_prefix(...)` rather than
  reopening any spent selector-level, continuation-level, axes/anchor,
  modal/temporal, sizing, trunc/quota, helper-local package-requirement,
  accessor-level, or first-work-item explanation beneath it.
- Scope: move above that first search-side family-summary / work-item layer to
  the frontier ordering / queue-pop consumer on the assembled no-focus claim
  profile.
  Result:
  - on the live seven-clause reference prefix and the already-spent focused
    three no-bound flat-shell residues used only as comparison geometry,
    `prefix_frontier_work_key(...)` ties first on
    `remaining_clause_slots = 1`,
    `next_clause_count = 3`,
    `remaining_family_options = u8::MAX`, and
    `clause_count = 7`
  - the first differing field is `bit_cost`, where the live reference prefix
    stays at `191` while every focused-three residue is heavier, so
    `pop_best_prefix(...)` still pops the live reference work item before the
    focused claim-side residues
  - targeted `pen-search` regressions now pass for the live step-`15`
    work-item baseline and the new frontier-queue ordering slice
  Consequence: frontier ordering is not secretly promoting the spent focused-
  three dead branch ahead of the live no-focus reference trio either. The
  next honest slice should move above queue-pop to the post-pop remaining-one
  consumer on the live reference prefix, starting with
  `collapse_single_continuation_chain(...)` and then
  `claim_try_summary_prune_before_materialization(...)` rather than reopening
  any spent selector-level, continuation-level, axes/anchor, modal/temporal,
  sizing, trunc/quota, helper-local package-requirement, accessor-level,
  work-item, or frontier-ordering explanation beneath it.
- Scope: move above that frontier ordering / queue-pop consumer to the
  post-pop remaining-one handoff on the live seven-clause reference prefix.
  Result:
  - `collapse_single_continuation_chain(...)` is a strict no-op there because
    queue-pop already hands over the live reference work item at
    `remaining_clause_slots = 1` with `next_clause_count = 3`; no child
    prefix is synthesized and the legality-cache entry counts stay unchanged
  - `claim_try_summary_prune_before_materialization(...)` does not pre-prune
    that live reference prefix either; instead it caches a compact
    claim-open-band shell with `generated = 3`, `admitted = 1`, a
    bar-clearing bound, `claim_open_band_structural = 1`, and a single
    survivor whose best accept rank already matches the canonical
    `reference(15)` `103 / 8` completion
  - targeted `pen-search` regressions now pass for the new collapse no-op
    slice, the new summary-stage slice, the frontier-ordering baseline, and
    the nearby canonical single-bucket baseline
  Consequence: the live post-pop remaining-one handoff is not hiding a
  single-continuation collapse or a fresh summary-stage prune lever on the
  live reference prefix either. The next honest slice should move above both
  to the remaining-one materialization handoff on that same live reference
  prefix, starting with `materialize_remaining_one_prefix_group(...)` and
  `materialize_terminal_prefix_group(...)` rather than reopening any spent
  selector-level, continuation-level, axes/anchor, modal/temporal, sizing,
  trunc/quota, helper-local package-requirement, accessor-level, work-item,
  frontier-ordering, collapse, or summary-pre-prune explanation beneath it.
- Scope: move above that post-pop remaining-one handoff to the remaining-one
  materialization handoff on the live seven-clause reference prefix.
  Result:
  - `materialize_remaining_one_prefix_group(...)` increments
    `remaining_one_materialized` and delegates to
    `materialize_terminal_prefix_group(...)`
  - because the cached compact summary there carries no full evaluations and
    only a single-survivor sketch,
    `materialize_terminal_prefix_group(...)` reuses the cached summary through
    `materialize_terminal_prefix_group_from_survivor_sketch(...)` rather than
    reopening direct compact materialization
  - the handoff relands `generated = 3`, `admitted = 1`, the same
    bar-clearing bound, and the lone canonical `reference(15)` `103 / 8`
    survivor on the live seven-clause reference prefix
  - targeted `pen-search` regressions now pass for the new remaining-one
    materialization slice, the earlier collapse and summary-stage slices, and
    the downstream canonical single-bucket baseline
  Consequence: the live remaining-one materialization handoff is not hiding a
  fresh survivor-expansion or direct-compact replay lever either. The next
  honest slice should move above it to the first post-materialize retained-
  bound / incumbent-rank consumer on the same live reference prefix, starting
  with `record_demo_bucket_surface(...)`,
  `record_demo_bucket_exact_screened(...)`,
  `terminal_prefix_rank_prune_count(...)`, and
  `best_prefix_group_accept_rank(...)`.
- Scope: move above that remaining-one materialization handoff to the first
  post-materialize retained-bound / incumbent-rank consumer on the live
  seven-clause reference prefix.
  Result:
  - summary pre-prune has already recorded the live
    `k8:structural_generic:temporal_operator:library_backed:small_cluster`
    shell once at `generated = 3`, `admitted = 1`, and
    `exact_screened = 1`
  - `record_demo_bucket_surface(...)` and
    `record_demo_bucket_exact_screened(...)` then add a second `3 / 1 / 1`
    pass for that same retained group, lifting the running prefix-local bucket
    total to `6 / 2 / 2` while still leaving zero prunes and zero fully scored
    terminals at that stage
  - the retained bound still clears the bar, `discovery.terminal_rank_incumbent`
    is still absent there, `terminal_prefix_rank_prune_count(...)` returns
    `None`, and `best_prefix_group_accept_rank(...)` relands the canonical
    `reference(15)` `103 / 8` rank from the lone retained candidate
  - targeted `pen-search` regressions now pass for the new post-materialize
    bucket/rank slice together with the earlier collapse, summary-stage, and
    remaining-one materialization slices
  Consequence: the first post-materialize bucket/rank consumer is not hiding a
  fresh prune or rank-order lever either. The next honest slice should move
  above it to the candidate-cache handoff on that same live reference prefix,
  starting with `cache_evaluated_terminal_prefix_group_candidates(...)` and
  `PrefixCache::record_group_with_surface_counts(...)`.
- Scope: localize that same live seven-clause reference prefix one layer
  deeper at the candidate-cache handoff above the first post-materialize
  bucket/rank consumer.
  Result:
  `cache_evaluated_terminal_prefix_group_candidates(...)` sorts the lone
  retained candidate, evaluates it once because no incumbent rank exists yet,
  seeds `discovery.terminal_rank_incumbent` with the canonical
  `reference(15)` `103 / 8` survivor, and then compacts away the evaluated
  payload because `DesktopClaimShadow` keeps the compact claim path live;
  `PrefixCache::record_group_with_surface_counts(...)` then records that same
  live seven-clause reference prefix as a one-survivor compact cache group
  with `generated = 3`, `admitted = 1`, the same bar-clearing bound, the
  same canonical best accept rank, and no signature merge.
  Consequence: the candidate-cache handoff is not hiding a replay-only
  incumbent shift or a wider retained family either. The next honest slice
  should move above it to the first proof-close intake on that same live
  reference prefix, starting with
  `load_terminal_prefix_group_for_proof_close(...)`.
- Scope: localize that same live seven-clause reference prefix one layer
  deeper at the first proof-close intake above the candidate-cache handoff.
  Result:
  - `load_terminal_prefix_group_for_proof_close(...)` immediately takes the
    compact live reference group back out of `PrefixCache`, so claim-mode
    proof-close releases the resident cache copy as soon as intake begins
  - the reloaded group still carries `generated = 3`, `admitted = 1`, the
    same bar-clearing retained bound, and only the canonical
    `reference(15)` `103 / 8` survivor, still without a retained evaluated
    payload
  - `demo_bucket_key_for_group(...)` rebuckets that compact one-survivor
    intake onto
    `k8:structural_generic:temporal_operator:library_backed:single`, so the
    earlier step-`15` small-cluster same-primary relief no longer applies at
    this group gate
  - proof-close does not reuse the earlier candidate-cache incumbent there
    either: its local `incumbent_terminal_rank` starts empty, so the first
    live reference intake stays open because the proof-close gate is still
    unseeded, not because the rebucketed single survivor inherits the earlier
    same-primary carveout
  - targeted `pen-search` regressions now pass for the new proof-close intake
    slice together with the earlier candidate-cache, first post-materialize
    bucket/rank, and generic claim proof-close cache-release slices
  Consequence: the first proof-close intake is not hiding a stale-cache replay
  bug, a preserved small-cluster relief exemption, or an already-seeded local
  incumbent either. The next honest slice should move above it to the
  proof-close candidate-order / candidate-level incumbent gate on that same
  live reference prefix, starting with
  `sort_terminal_prefix_group_candidates_for_certification(...)` and the
  candidate-level `accept_rank_can_survive_incumbent(...)` check.
- Scope: localize that same live seven-clause reference prefix one layer
  deeper at the proof-close candidate evaluation / rank-recomputation /
  minimality path above the candidate-order gate.
  Result:
  - `evaluate_checked_candidate(...)` re-evaluates the compact canonical
    `reference(15)` survivor because proof-close reloads that live reference
    group with no retained evaluated payload
  - that fully scored replay relands `nu = 103`, `clause_kappa = 8`,
    `bit_kappa = 229`, and `rho = 103 / 8`
  - `acceptance_rank(...)` recomputes the same canonical
    `reference(15)` `103 / 8` rank from the fully scored survivor
  - `analyze_semantic_minimality(...)` keeps the survivor semantically
    minimal with no admissible bar-clearing detachable subbundle
  - proof-close seeds its fresh local `incumbent_terminal_rank` with that
    canonical rank only after the re-evaluation / minimality confirmation,
    not by reusing `discovery.terminal_rank_incumbent`
  - targeted `pen-search` regressions now pass for the new proof-close
    evaluation / rank / minimality slice together with the earlier
    candidate-cache, proof-close intake, and candidate-gate layers
  Consequence: the proof-close evaluation layer is not hiding a compact-replay
  score drift, a minimality-only drop, or an implicit incumbent reuse bug
  either. The next honest slice should move above it to the first
  post-evaluation fully scored bucket / proof-close full-eval accounting
  consumer on that same live reference prefix, starting with
  `bucket_stats.fully_scored_terminal_candidates`,
  `bucket_stats.best_overshoot`, and `demo_phase.proof_close_full_evals`.
- Scope: localize that same live seven-clause reference prefix one layer
  deeper at the first post-evaluation fully scored bucket / proof-close
  full-eval accounting consumer above the proof-close evaluation layer.
  Result:
  - the rebucketed
    `k8:structural_generic:temporal_operator:library_backed:single` pocket
    records `fully_scored_terminal_candidates = 1` and
    `best_overshoot = 115657 / 21112` on the canonical `reference(15)` replay,
    while the earlier `small_cluster` shell stays at
    `fully_scored_terminal_candidates = 0` with no best overshoot
  - the live non-budgeted `DesktopClaimShadow` path keeps
    `demo_phase.materialize_full_evals = 0` and
    `demo_phase.proof_close_full_evals = 0`; those demo counters stay dormant
    because no step budget is active on this path
  - the lone fully scored replay is instead carried only by top-level step
    accounting, with `full_telescopes_evaluated = 1`,
    `demo_funnel.full_telescopes_evaluated = 1`, and the same single-bucket
    capture count
  - targeted `pen-search` regressions now pass for the new post-evaluation
    accounting slice together with the earlier proof-close evaluation,
    candidate-gate, intake, and candidate-cache layers
  Consequence: the first post-evaluation accounting consumer is not hiding a
  missing demo proof-close increment or a rebucket drift either. The next
  honest slice should move above it to the top-level full-telescope /
  demo-funnel accounting consumer on that same live reference prefix,
  starting with `full_telescopes_evaluated`,
  `demo_funnel.full_telescopes_evaluated`, and
  `build_demo_funnel_stats(...)`.
- Scope: localize that same live seven-clause reference prefix one layer
  deeper at the top-level full-telescope / demo-funnel accounting consumer
  above the post-evaluation bucket layer.
  Result:
  - rebuilding the public funnel with `build_demo_funnel_stats(...)` relands
    the same live public surface on that canonical reference prefix, keeping
    `generated_raw_prefixes = 4331`,
    `full_telescopes_evaluated = 1`,
    `exact_bound_screened = exact_bound_pruned + 1`,
    `bar_clearers = 1`,
    `semantically_minimal_clearers = 1`, and canonical
    `winner_overshoot = 115657 / 21112`
  - the rebuilt funnel still matches the live step-level
    `well_formed_terminals`, `hard_admissible`, and `heuristic_dropped`
    accounting together with the lone retained bar-clearing survivor on the
    canonical `reference(15)` path
  - targeted `pen-search` regressions now pass for the new top-level
    demo-funnel slice together with the earlier proof-close full-eval,
    evaluation, candidate-gate, intake, and candidate-cache layers
  Consequence: the public demo-funnel builder is not hiding a late replay-
  accounting drift, a missing exact-screen carry, or a missing
  clearer/minimality projection either. The next honest slice should move
  above it to the top-level demo-closure consumer on that same live reference
  prefix, starting with `build_demo_closure_stats(...)`,
  `demo_closure.frontier_total_seen`,
  `demo_closure.frontier_certified_nonwinning`, and
  `demo_closure.closure_percent`.
- Scope: localize that same live seven-clause reference prefix one layer
  deeper at the top-level demo-closure consumer above the public funnel
  builder.
  Result:
  - rebuilding the public closure with `build_demo_closure_stats(...)`
    relands the same live public surface on that canonical reference prefix,
    keeping `frontier_total_seen = exact_bound_screened`,
    `frontier_certified_nonwinning = exact_bound_pruned`, and
    `closure_percent = 99`
  - the rebuilt closure still leaves exactly the lone fully scored canonical
    `reference(15)` replay outside the certified-nonwinner count, so
    `frontier_total_seen = frontier_certified_nonwinning + 1`
  - targeted `pen-search` regressions now pass for the new top-level
    demo-closure slice together with the earlier top-level funnel,
    proof-close full-eval, evaluation, candidate-gate, intake, and
    candidate-cache layers
  Consequence: the public demo-closure builder is not hiding a frontier-total
  drift, a certified-nonwinner drift, or a closure-percent rounding bug
  either. The next honest slice should move above it to the report-side
  demo-closure handoff on that same live reference prefix, starting with
  `step.demo_closure`,
  `StepReport.search_stats.demo_closure`, and
  `stored_demo_closure(...)`.
- Scope: localize that same live seven-clause reference prefix one layer
  deeper at the report-side demo-closure handoff above the public closure
  builder.
  Result:
  - `step_to_report(...)` copies `AtomicSearchStep.demo_closure` directly into
    `StepReport.search_stats.demo_closure`; on the live step-`15` claim step,
    the stored closure stays
    `frontier_total_seen = 554`,
    `frontier_certified_nonwinning = 553`,
    `closure_percent = 99`
  - serializing the report keeps that same `demo_closure` payload in the step
    summary
  - `stored_demo_closure(...)` prefers the stored closure over a mutated
    `demo_funnel` fallback, so the report-side path keeps the exact live
    public closure surface instead of recomputing it
  - targeted `pen-cli` regressions now pass for
    `current_claim_step_fifteen_step_report_preserves_the_live_demo_closure_surface`
    and
    `current_claim_step_fifteen_stored_demo_closure_prefers_the_reported_surface`
  Consequence: the report-side demo-closure handoff is not hiding a
  step-to-report copy drift, a serialization drift, or a stored-versus-
  fallback closure drift either. The next honest slice should move above it
  to the first report/narrative closure render consumer on that same live
  reference prefix, starting with `closure_line(...)`,
  `render_step_narrative(...)`, and the `demo closure:` line inside
  `render_debug_report(...)`.
- Scope: localize that same live seven-clause reference prefix one layer
  deeper at the first report/narrative closure render consumer above the
  report-side handoff.
  Result:
  - `render_step_narrative(...)` feeds `closure_line(...)` from
    `stored_demo_closure(...)`; on the live step-`15` claim report, even
    after mutating `demo_funnel.exact_bound_screened = 7` and
    `demo_funnel.exact_bound_pruned = 1`, the rendered narrative still keeps
    the stored `99% frontier_total = 554 certified_nonwinning = 553`
    closure line
  - `render_debug_report(...)` keeps that same stored `demo closure:` line on
    the same drifted report instead of rendering the fallback
    `frontier_total_seen = 7`,
    `frontier_certified_nonwinning = 1`,
    `closure_percent = 14`
  - targeted `pen-cli` regressions now pass for
    `current_claim_step_fifteen_narrative_render_uses_the_stored_demo_closure_surface`
    and
    `current_claim_step_fifteen_debug_report_demo_closure_line_uses_the_reported_surface`
  Consequence: the first report/narrative closure render layer is not hiding a
  narrative-line drift or a debug-report text drift either. The next honest
  slice should move above it to the caller/output emission layer on that same
  live reference prefix, starting with `render_run_narrative(...)`,
  `write_step_narrative_artifact(...)`,
  `render_run_output(...)`, and `write_latest_reports(...)`.
- Scope: localize that same live seven-clause reference prefix one layer
  deeper at the first caller/output emission layer above the
  report/narrative closure render consumer.
  Result:
  - `render_run_narrative(...)` keeps the same stored
    `99% frontier_total = 554 certified_nonwinning = 553` closure line on the
    drifted step-`15` claim run because it simply joins the per-step
    `render_step_narrative(...)` output rather than recomputing from the
    mutated `demo_funnel`
  - `write_step_narrative_artifact(...)` persists that same stored closure
    line into `reports/steps/step-15-narrative.txt`
  - `render_run_output(...)` keeps the base standard/debug report intact and
    appends that same stored run narrative when narrative output is requested
  - `write_latest_reports(...)` persists `latest.txt` and `latest.debug.txt`
    straight from `render_standard_report(...)` and
    `render_debug_report(...)`, so the drifted step-`15` claim latest debug
    artifact still keeps
    `frontier_total_seen = 554`,
    `frontier_certified_nonwinning = 553`,
    `closure_percent = 99`
  - targeted `pen-cli` regressions now pass for
    `current_claim_step_fifteen_run_narrative_uses_the_stored_demo_closure_surface`,
    `current_claim_step_fifteen_narrative_artifact_uses_the_stored_demo_closure_surface`,
    `current_claim_step_fifteen_run_output_with_narrative_uses_the_stored_demo_closure_surface`,
    and
    `current_claim_step_fifteen_latest_reports_preserve_the_live_reported_surface`
  Consequence: the first caller/output emission layer is not hiding a
  run-narrative join drift, narrative-artifact write drift, combined
  terminal-output drift, or latest-report persistence drift either. The next
  honest slice should move above it to the caller-side narrative toggle /
  emission-dispatch layer on that same live reference prefix, starting with
  `terminal_narrative_config(...)`,
  `run(...)`,
  `resume(...)`,
  `RunArtifactWriter::persist_step(...)`,
  `RunArtifactWriter::finalize_success(...)`, and
  `finalize_failed_run(...)` / `RunArtifactWriter::finalize_failed(...)`.
- Scope: localize that same live seven-clause reference prefix one layer
  deeper at the caller-side narrative toggle / emission-dispatch layer above
  the first caller/output emission consumer.
  Result:
  - `terminal_narrative_config(...)` now has a direct claim regression too:
    it opens narrative output only when the caller explicitly requests it on a
    narrative-capable demo profile, so the claim run/resume path stays opt-in
  - on a drifted stored step-`15` claim report,
    `RunArtifactWriter::persist_step(...)` preserves the stored
    `frontier_total_seen = 554`,
    `frontier_certified_nonwinning = 553`,
    `closure_percent = 99` surface in both `step-15-summary.json` and
    `step-15-narrative.txt`
  - `RunArtifactWriter::finalize_success(...)` plus
    `finalize_failed_run(...)` / `RunArtifactWriter::finalize_failed(...)`
    keep that same stored closure surface in `latest.debug.txt` rather than
    drifting to the mutated `7 / 1 / 14` fallback
  - a completed `resume(...)` call on that same drifted stored run reuses the
    stored closure surface in both the debug `demo closure:` line and the
    appended claim narrative
  - targeted `pen-cli` regressions now pass for
    `current_claim_step_fifteen_terminal_narrative_config_only_opens_for_requested_claim_demo_runs`,
    `current_claim_step_fifteen_persist_step_and_finalize_success_preserve_the_stored_demo_closure_surface`,
    `current_claim_step_fifteen_failed_run_finalization_preserves_the_stored_demo_closure_surface`,
    and
    `current_claim_step_fifteen_completed_resume_with_narrative_uses_the_stored_demo_closure_surface`
  Consequence: the caller-side dispatch layer is not hiding an opt-in toggle
  drift, step-artifact persistence drift, latest-debug finalization drift,
  failed-run finalization drift, or completed-resume readback drift either.
  The next honest slice should move above it to the first downstream stored-
  artifact consumer in `scripts/compare_runs.py`, starting with
  `demo_phase_entry(...)`, `has_demo_phase_evidence(...)`, and the
  `demo phase latest:` / `demo funnel latest:` lane-summary lines.
- Scope: localize the first downstream stored-artifact consumer in
  `scripts/compare_runs.py` above the caller-side narrative toggle /
  emission-dispatch layer.
  Result:
  - `has_demo_phase_evidence(...)` now has direct claim regression coverage
    too: it keeps compare-side demo-phase evidence live when stored
    `demo_closure` is the only surviving proof of breadth
  - on a drifted stored step-`15` claim report, `demo_phase_entry(...)`
    preserves the mixed surface of funnel-side
    `4331 / 7 / 1 / 1 / 115657 / 21112` plus stored closure
    `554 / 553 / 99`
  - `render_lane_summary(...)` keeps the `demo phase latest:` line on
    `proof_close_closure = 99%` and the `demo funnel latest:` line on
    `closure = 99%` instead of the drifted `14%`
  - added direct `compare_runs.py` unit coverage:
    `test_claim_demo_phase_evidence_stays_present_for_stored_closure_only`,
    `test_claim_demo_phase_entry_prefers_the_stored_demo_closure_surface`,
    and `test_claim_lane_summary_uses_the_stored_demo_closure_surface`
  - verification execution is currently blocked in this shell because no
    `python`, `python3`, or `py` launcher is available
  Consequence: the first compare-side stored-artifact consumer is not hiding a
  closure-gating drift, a `demo_phase_entry(...)` fallback drift, or a lane-
  summary render drift either. The next honest slice should move above it to
  the remaining compare-side audit consumer in `scripts/compare_runs.py`,
  starting with
  `narrative_artifact_entry(...)`,
  `build_claim_lane_audit(...)`,
  `summarize_claim_lane_audit(...)`, and the
  `narrative artifacts:` / `claim audit:` / `search policy:` /
  `fallback honesty:` lane-summary lines.
- Scope: localize the remaining compare-side audit consumer in
  `scripts/compare_runs.py` above that compare-side demo-phase consumer.
  Result:
  - `narrative_artifact_entry(...)` now has direct claim regression coverage
    too: a clean step-`15` claim artifact pair relands
    `status = complete`, `text = 1 / 1`, and `events = 1 / 1`
  - `build_claim_lane_audit(...)` and
    `summarize_claim_lane_audit(...)` keep a complete honest claim lane at
    `ready` with zero reasons while preserving accepted-hash parity,
    narrative-artifact completeness, exact-screen coverage, prune-class
    coverage, honest claim search policy, and clear fallback honesty
  - `render_lane_summary(...)` keeps
    `narrative artifacts: complete (text=1/1, events=1/1)`,
    `claim audit: ready (none)`, and the honest `search policy:` plus clear
    `fallback honesty:` lines on that ready audit surface
  - added direct `compare_runs.py` unit coverage:
    `test_claim_narrative_artifact_entry_tracks_complete_claim_step_artifacts`,
    `test_claim_lane_audit_ready_for_complete_honest_claim_lane`, and
    `test_claim_lane_summary_renders_claim_audit_lines_from_ready_audit_surface`
  - verification execution is currently blocked in this shell because no
    `python`, `python3`, or `py` launcher is available
  Consequence: the remaining compare-side audit consumer is not hiding a
  narrative-artifact completeness drift, a claim-audit readiness drift, a
  search-policy render drift, or a fallback-honesty render drift either. The
  next honest slice should move above it to the top-level compare summary /
  signoff consumer in `scripts/compare_runs.py`, starting with
  `build_summary(...)`,
  `signoff_summary(...)`,
  `render_claim_lane_audit(...)`,
  `render_text_summary(...)`, and the top-level
  `Comparison Signoff:` / `claim lane audit:` lines.
- Scope: localize the top-level compare summary / signoff consumer in
  `scripts/compare_runs.py` above that compare-side audit consumer.
  Result:
  - `build_summary(...)` and `signoff_summary(...)` now have direct claim
    regression coverage on a ready two-lane comparison: they keep
    `signoff.status = ready`,
    `signoff.summary = all 2 lanes preserve baseline claim-baseline and the
    step-15 reference claim boundary`, a consistent step-`15` claim boundary,
    and a ready top-level claim-lane audit on
    `claim-baseline / claim-current`
  - `render_claim_lane_audit(...)` now has direct coverage too: it keeps
    `ready (claim-baseline, claim-current)` on that same ready audit surface
  - `render_text_summary(...)` now surfaces the stored signoff explanation
    directly in the top-level `Comparison Signoff:` line while keeping the
    top-level `claim lane audit:` line on that ready claim-audit surface,
    even when the comparison geometry still uses the known drifted
    `7 / 1 / 14` funnel against stored closure `554 / 553 / 99`
  - direct `compare_runs.py` unit coverage now includes
    `test_claim_comparison_build_summary_keeps_ready_signoff_and_audit_surface`,
    `test_claim_top_level_audit_renderer_reports_ready_claim_lanes`, and
    `test_claim_text_summary_top_level_lines_keep_ready_signoff_and_audit_surface`
  - verification execution is currently blocked in this shell because no
    `python`, `python3`, or `py` launcher is available
  Consequence: the top-level compare summary / signoff consumer is not hiding
  a signoff-summary drift, a top-level claim-audit rollup drift, or a
  text-summary render drift either. The next honest slice should move above it
  to the compare script caller/output layer in `scripts/compare_runs.py`,
  starting with `main()`, `--text-out`, `--json-out`, `write_text(...)`, and
  stdout emission of the top-level summary.
- Scope: localize the compare script caller/output layer in
  `scripts/compare_runs.py` above that top-level compare summary / signoff
  consumer.
  Result:
  - direct `compare_runs.py` unit coverage now drives `main()` through real
    `--lane`, `--baseline`, `--text-out`, and `--json-out` parsing while
    patching `load_lane(...)` onto the ready
    `claim-baseline / claim-current` comparison surface
  - nested `write_text(...)` output creation now lands both `summary.txt` and
    `summary.json`; stdout matches `--text-out` on the ready top-level
    `Comparison Signoff:` / `claim lane audit:` summary
  - `--json-out` preserves `baseline_lane = claim-baseline`,
    `signoff.status = ready`, and `claim_lane_audit.status = ready`
  - direct `compare_runs.py` unit coverage now includes
    `test_claim_compare_main_writes_text_json_and_stdout_from_ready_summary`
  - verification execution is currently blocked in this shell because no
    `python`, `python3`, or `py` launcher is available
  Consequence: the compare script caller/output layer is not hiding CLI parse
  drift, nested artifact-write drift, JSON summary emission drift, or stdout
  drift either. The next honest slice should move above the whole compare
  evidence surface to the first downstream certification consumer in
  `scripts/certify_claim_lane.py`, starting with
  `accepted_hash_parity_check(...)`, `search_policy_check(...)`,
  `fallback_honesty_check(...)`, `narrative_artifact_check(...)`, and the
  top-level `checks` / `failing_checks` surface.
- Scope: localize the first downstream certification consumer in
  `scripts/certify_claim_lane.py` above that whole compare evidence surface.
  Result:
  - direct `certify_claim_lane.py` unit coverage now pins the ready
    claim-lane surface for `accepted_hash_parity_check(...)`,
    `search_policy_check(...)`, `fallback_honesty_check(...)`, and
    `narrative_artifact_check(...)`; on the ready claim lane built from the
    stored-closure compare fixture, all four stay `pass` with the expected
    ready detail
  - direct `certify_claim_lane.py` unit coverage now drives `main()` through
    real `--claim-run`, `--guarded-run`, `--runtime-threshold-ms`,
    `--json-out`, and `--text-out` parsing while patching the
    summary/manifest/step inputs onto a certificate surface where only those
    first four certification consumers fail
  - the emitted certificate keeps `status = attention`, isolates
    `failing_checks` to
    `accepted_hash_parity / search_policy / fallback_honesty / narrative_artifacts`,
    and leaves early breadth, late generated floors, runtime threshold,
    exact-screen completeness, prune-class completeness, and manifest
    completeness green
  - direct `certify_claim_lane.py` unit coverage now includes
    `test_claim_certification_first_downstream_checks_pass_for_ready_claim_lane`
    and
    `test_claim_certification_main_surfaces_first_downstream_failures_in_checks`
  - verification execution is currently blocked in this shell because no
    `python`, `python3`, or `py` launcher is available
  Consequence: the first certification consumer is not hiding parity-detail
  drift, policy-detail drift, fallback-detail drift, narrative-artifact
  drift, or top-level check-aggregation / failing-check routing drift
  either. The next honest slice should move above that first certification
  check surface to the stored breadth/runtime/completeness consumers in
  `scripts/certify_claim_lane.py`, starting with
  `breadth_check(...)`, `breadth_diagnosis(...)`,
  `runtime_threshold_check(...)`, `completeness_check(...)`, and
  `manifest_completeness_check(...)`.
- Scope: re-verify the currently pinned live-reference compact-cache /
  proof-close / report stack on this machine and sync the autonomy docs to
  that verified state.
  Result:
  - reran `cargo test -p pen-type desktop_claim_shadow_step_`
  - reran `cargo test -p pen-search current_claim_step_fifteen_live_reference_`
  - reran
    `cargo test -p pen-search current_claim_step_fifteen_live_caller_setup_stays_on_default_claim_generic_profile`
  - reran
    `cargo test -p pen-search current_claim_step_fifteen_live_work_item_keeps_the_default_claim_profile_family_agnostic`
  - reran
    `cargo test -p pen-search current_claim_step_fifteen_frontier_queue_pops_the_live_reference_work_item_before_the_tighter_focused_three_prefixes_on_bit_cost`
  - reran `cargo test -p pen-cli current_claim_step_fifteen_`
  - all focused Rust checks passed without code changes
  - confirmed again that no `python`, `python3`, `py`, or `uv` launcher is
    available locally, so direct `scripts/compare_runs.py` and
    `scripts/certify_claim_lane.py` execution remains blocked
  Consequence: the live docs now freeze the whole live-reference compact-cache
  / proof-close / report / compare / certification evidence stack as verified
  on this machine, and the next slice stays above it to a different code-side
  step-`15` repair family.
- Scope: localize the focused-three no-bound flat-shell exact partial-prefix
  gate above the already-pinned dead exact-terminal branch.
  Result:
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_focused_three_no_bound_flat_shell_prefixes_default_exact_partial_prefix_gate_stays_cannot_clear_bar`
    and
    `current_claim_step_fifteen_focused_three_no_bound_flat_shell_pair_cell_summary_override_only_reclassifies_the_exact_partial_prefix_gate_to_unknown`
  - on those same `3`
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / reference`
    prefixes, direct `exact_partial_prefix_bound_decision(...)` replay now
    stays explicit as `CannotClearBar` by default while caching the same
    compact dead `3`-generated / `0`-admitted no-bound summary with no best
    accept primary rank and no best accept rank
  - enabling the pair-cell remaining-one exact-summary override on that same
    focused-three residue now stays explicit too: it only reclassifies the
    gate to `Unknown`, leaves the same compact dead summary in cache, and does
    not cache any stable partial-prefix decision
  - focused current-machine verification passed via
    `cargo test -p pen-search focused_three_no_bound_flat_shell -- --nocapture`
  Consequence: the focused-three exact-bound gate is not hiding a fresh repair
  either; it is only the handoff into the already-pinned dead exact-terminal
  path, so the next honest slice still has to move above that whole branch to
  a different code-side step-`15` repair family.
- Scope: localize the first reintroduced late temporal-focus replay on the
  live seven-clause reference prefix above the fully pinned compare /
  certification evidence surface.
  Result:
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_live_reference_reintroduced_temporal_focus_closes_claim_open_band_but_keeps_the_same_single_survivor_summary`
  - reran
    `cargo test -p pen-search live_reference_reintroduced_temporal_focus -- --nocapture`,
    `cargo test -p pen-search claim_step_fifteen_family_summary_stays_disabled_until_a_focus_family_is_reintroduced -- --nocapture`,
    and `cargo test -p pen-search current_claim_step_fifteen_live_reference_ -- --nocapture`;
    all passed
  - replaying the live step-`15` reference prefix with
    `focus_family = TemporalShell` and
    `package_policies.temporal_shell = Prefer` reactivates one remaining
    family option on the live work item, but still keeps no filtered terminal
    clause list resident there and keeps the same three-clause terminal
    catalog live
  - `claim_try_summary_prune_before_materialization(...)` still does not
    pre-prune that focused replay; closing `ClaimAdmittedOpenBand` only reroutes
    the live reference prefix through the general admissibility path, while the
    cached compact summary stays on `generated = 3`, `admitted = 1`, a
    bar-clearing bound, and a single canonical `103 / 8` survivor with zero
    terminal-prefix-bar and zero terminal-rank prunes
  Consequence: pure temporal-focus reintroduction on the live reference prefix
  is another neutral control rather than the next repair. The next honest
  slice should move above it to the caller-level late-family-surface override
  replay on that same live reference prefix rather than reopening pure
  temporal focus or the pinned compare/certification stack.

- Scope: localize the caller-level late-family-surface override replay on that
  same live seven-clause reference prefix above the newly frozen pure
  temporal-focus control and the pinned compare / certification evidence
  surface.
  Result:
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_live_reference_late_family_surface_override_closes_claim_open_band_and_widens_the_terminal_catalog_without_displacing_the_canonical_survivor`
  - reran
    `cargo test -p pen-search live_reference_late_family_surface_override -- --nocapture`,
    `cargo test -p pen-search claim_open_band_terminal_clause_filter_requires_default_claim_generic_route -- --nocapture`,
    and `cargo test -p pen-search current_claim_step_fifteen_live_reference_ -- --nocapture`;
    all passed
  - keeping the same live `DesktopClaimShadow` admissibility profile but
    overriding the caller-side `LateFamilySurface` from `ClaimGeneric` to
    `DemoBreadthShadow` keeps `kappa = 8 .. 8` and
    `historical_anchor_ref = Some(10)`, closes `ClaimAdmittedOpenBand`, and
    still leaves no cached family summary plus no filtered next-clause list on
    the live work item
  - that same replay widens the live remaining-one terminal catalog from `3`
    clauses to `5`, reroutes through the widened general terminal path, still
    avoids pre-materialization prune, and still caches a compact summary at
    `generated = 5`, `admitted = 1`, `open_band_structural = 1`, a
    bar-clearing bound, a single canonical `103 / 8` survivor sketch, and zero
    terminal-prefix-bar plus zero terminal-rank prunes
  Consequence: the caller-level late-family-surface override is another
  widening control rather than the next repair. The next honest slice should
  move above it to the derivation that keeps step `15` on the default
  `ClaimGeneric` path rather than reopening pure temporal focus, the
  late-surface override, or the pinned compare/certification stack.

- Scope: localize the enumeration-context constructor above that live
  late-family-surface override replay.
  Result:
  - added direct `pen-search` regression coverage for
    `desktop_claim_shadow_step_fifteen_enumeration_context_derivation_stays_claim_generic_until_the_mode_changes`
  - reran
    `cargo test -p pen-search desktop_claim_shadow_step_fifteen_enumeration_context_derivation_stays_claim_generic_until_the_mode_changes -- --nocapture`
    and
    `cargo test -p pen-search claim_generic_kappa_eight_catalog_adds_modal_temporal_exchange_variants -- --nocapture`;
    both passed
  - on the live step-`15` claim admissibility,
    `EnumerationContext::from_admissibility(...)` still derives
    `LateFamilySurface::ClaimGeneric` directly from
    `AdmissibilityMode::DesktopClaimShadow`
  - replaying that same live admissibility with a bypassed synthetic
    `focus_family = TemporalShell` plus
    `package_policies.temporal_shell = Prefer` still yields an identical
    enumeration context and the same live
    `reference + eventual_lift + next_lift` remaining-one catalog
  - only an explicit mode change to `DemoBreadthShadow` leaves the default
    claim-generic surface at constructor time
  Consequence: the live step-`15` route is not hiding a focus-family or
  package-policy-driven surface reroute at constructor time either. The next
  honest slice should move above it to the raw claim-generic band-`8` family
  emitter / late clause selector rather than reopening pure temporal focus,
  the late-surface override, or the pinned compare/certification stack.

- Scope: localize the first in-flight discovery checkpoint above the live
  step-`15` root-seeding summary in
  [`C:\DEV\atomic\crates\pen-search\src\engine.rs`](C:\DEV\atomic\crates\pen-search\src\engine.rs).
  Result:
  - force-emitted the first `claim_regular_frontier_progress` checkpoint
    inside
    `discover_realistic_shadow_candidates_with_clause_catalog_override(...)`
    so fast runs now preserve the first in-loop shell while later loop
    checkpoints still respect the existing elapsed-time throttle
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_live_regular_frontier_progress_checkpoint_keeps_the_same_three_root_shell_before_queue_pop`
  - reran
    `cargo test -p pen-search current_claim_step_fifteen_live_regular_frontier_progress_checkpoint_keeps_the_same_three_root_shell_before_queue_pop -- --nocapture`,
    `cargo test -p pen-search current_claim_step_fifteen_live_ -- --nocapture`,
    and
    `cargo test -p pen-search current_claim_step_fifteen_frontier_queue_pops_the_live_reference_work_item_before_the_tighter_focused_three_prefixes_on_bit_cost -- --nocapture`;
    all passed
  - the first in-loop checkpoint now stays explicit on the same
    `clause_kappa = 8`, raw widths `3 / 3 / 3 / 3 / 3 / 3 / 3 / 3`, raw
    telescopes `6561`, zero enumerated/well-formed/admissibility drift, zero
    prefix-state exploration, zero exact-screen prunes, and the same public
    root seeding `3 / 0 / 0 / 3` before any queue-pop or remaining-one
    telemetry movement
  Consequence: the first in-loop discovery checkpoint is now frozen too, so
  the next honest slice moves above it to the first queue-pop consumer on the
  live reference prefix.

- Scope: land the real ancestry-gated post-reference exact-two-step repair
  above the pinned queued `12`-item remainder and the combined dominant-pair
  scout.
  Result:
  - kept the blanket and scoped `DemoBreadthShadow` selectors as test-only
    scout controls, but widened the non-test `DesktopClaimShadow` step-`15`
    path whenever the post-reference exact-two-step prefix descends from the
    top-level queued `reference` or top-level queued
    `claim_eventual_domain` root
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_post_reference_exact_two_step_live_repair_matches_the_combined_dominant_top_level_remainder_scout`
  - reran
    `cargo test -p pen-search current_claim_step_fifteen_post_reference_exact_two_step_ -- --nocapture`,
    `cargo test -p pen-search current_claim_step_fifteen_live_ -- --nocapture`,
    `cargo test -p pen-cli current_claim_step_fifteen_ -- --nocapture`, and
    `cargo test -p pen-type desktop_claim_shadow_step_ -- --nocapture`;
    all passed
  - the landed live repair now matches the combined dominant-pair scout at
    `generated_raw_prefixes = 7211` with `partial_prefix_bar_failure = 553`,
    accepted `103 / 8`, `small_cluster = 2052 / 522 / 522 / 0`, the same
    fenced `single` pocket, and zero fully scored lifted `89 / 8`
  Consequence: the current-head Phase 2 local repair is satisfied. The next
  honest blocker is a new stored full-profile bundle beyond `v13`, not
  another pre-storage scout pass on the same remainder lattice.

- Scope: store the landed ancestry-gated post-reference exact-two-step repair
  in one clean full-profile bundle beyond `v13`.
  Result:
  - ran
    `cargo run --release -p pen-cli -- run --config configs/desktop_claim_shadow_10h.toml --root runs --run-id codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v14 --until-step 15`
  - stored `v14` completed through step `15` on the current dirty-tree head
  - manual stored step-summary comparison against `v13` keeps accepted-hash
    parity through all `15` steps, keeps step `1` at `546 / 288 / 1 / 435`,
    and moves step `15` from `4331 / 553 / 3132` to
    `7211 / 553 / 2052` with `broad = 3600 / 0 / 0 / 0`
  - `v13 -> v14` keeps the same search-policy metadata, compat hashes, and
    resolved worker count `8`
  - direct `scripts/compare_runs.py`, `scripts/benchmark_claim_lane.py`, and
    `scripts/certify_claim_lane.py` refresh remains blocked because
    `python`, `python3`, `py`, and `uv` are unavailable here
  Consequence: `v14` is now the stored head on current code, while `v13`
  remains the last fully audited compare/benchmark/cert bundle. The next
  honest blocker is downstream evidence freshness and then the remaining
  stored step-`1` gate if that audited evidence still points there.
- Scope: consume the landed step-`1` repair in one fresh full-profile stored
  rerun and refresh native evidence on current head.
  Result:
  - ran
    `cargo run --release -p pen-cli -- run --config configs/desktop_claim_shadow_10h.toml --root runs --run-id codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15 --until-step 15`
  - refreshed
    `claim-compare.{json,txt}`,
    `claim_benchmark.{json,txt}`,
    and
    `claim_certificate.{json,txt}`
    beside stored `v15` through native `pen-cli` commands
  - stored `v15` preserves accepted-hash parity through all `15` steps,
    closes step `1` to `2144 / 1285 / 1 / 475`,
    keeps stored step `15` at `7211 / 553 / 2052` with
    `broad = 3600 / 0 / 0 / 0`,
    and records `Claim Certification: ready` at stored runtime `4165 ms`
  - the refreshed benchmark across `v12` / `v13` / `v14` / `v15` now records
    parity success `4 / 4`, early-breadth success `1 / 4`, and late-floor
    success `2 / 4`
  Consequence: `v15` is now the current certified stored head. The remaining
  open gate is local step-`15` hardening below the clean
  `partial_prefix_bar_failure = 553` wall, not more stored step-`1`,
  certification, or tooling work.
- Scope: align the repo-native claim-evidence regression with the now-certified
  step-`1` surface.
  Result:
  - updated
    `breadth_and_runtime_checks_surface_claim_smoke_readiness`
    in `crates/pen-cli/src/claim_evidence.rs`
    so the claim smoke run now expects the stored step-`1` breadth check to
    pass instead of fail
  - reran `cargo test -p pen-cli claim_evidence -- --nocapture`; it passed
  Consequence: the local claim-evidence test surface now matches the certified
  stored claim-lane state instead of asserting the old pre-certification gap.
- Scope: localize the repaired-head clean step-`15` partial-prefix wall below
  the queued frontier remainder branches.
  Result:
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_live_repair_partial_prefix_wall_localizes_to_eight_frontier_remainder_branches_on_the_repaired_head`
  - reran
    `cargo test -p pen-search current_claim_step_fifteen_live_repair_partial_prefix_wall_localizes_to_eight_frontier_remainder_branches_on_the_repaired_head -- --nocapture`,
    `cargo test -p pen-search current_claim_step_fifteen_post_reference_exact_two_step_live_repair_matches_the_combined_dominant_top_level_remainder_scout -- --nocapture`,
    `cargo test -p pen-search current_claim_step_fifteen_live_frontier_remainder_items_record_the_twelve_queued_labels_and_work_keys -- --nocapture`,
    `cargo test -p pen-search current_claim_step_fifteen_live_frontier_remainder_checkpoint_exposes_reference_pop_and_remaining_one_telemetry_movement -- --nocapture`,
    and
    `cargo test -p pen-search current_claim_step_fifteen_live_ -- --nocapture`;
    all passed
  - with live repair enabled, all `553` clean partial-prefix captures now map
    to exactly queued branches `4 .. 11`, preserving the known
    `451` remaining-two plus `102` remaining-three split
  - branch totals are
    `4 / 5 = 7`,
    `6 / 7 = 19`,
    `8 / 9 = 52`,
    `10 = 156`,
    and
    `11 = 241`; queued branches `0 .. 3` contribute `0`
  Consequence: the next honest local repair target is now the direct
  top-level `reference` remainder first, then the top-level
  `claim_eventual_domain` remainder, not another frontier-wide scout or the
  already wall-free queued branches `0 .. 3`.

- Scope: localize the repaired-head direct top-level `reference` remainder
  one layer deeper below the frontier-remainder split.
  Result:
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_live_reference_remainder_partial_prefix_wall_localizes_to_the_mismatch_one_surface_before_the_reference_reference_tail`
    and
    `current_claim_step_fifteen_live_reference_remainder_remaining_two_wall_stays_on_the_same_five_pairings`
  - reran
    `cargo test -p pen-search current_claim_step_fifteen_live_reference_remainder_ -- --nocapture`
    and
    `cargo test -p pen-search current_claim_step_fifteen_live_repair_partial_prefix_wall_localizes_to_eight_frontier_remainder_branches_on_the_repaired_head -- --nocapture`;
    both passed
  - on the repaired `7211 / 553 / 2052` head, the direct top-level
    `reference` remainder now decomposes cleanly to mismatch
    `1 = 177 = 145` remaining-two plus `32` remaining-three,
    mismatch `2 = 50 = 42 + 8`,
    and mismatch `3 = 14 = 12 + 2`
  - the remaining-two slice inside that direct `reference` remainder keeps
    the same five pairings explicit:
    `reference / claim_next_codomain = 42`,
    `reference / claim_sharp_codomain = 42`,
    `reference / demo_flat_codomain = 61`,
    plus the mismatch-`2` and mismatch-`3` `reference / reference` tail at
    `42` and `12`
  Consequence: the next honest claim-lane slice is now below the repaired-head
  mismatch-`1` surface inside the direct top-level `reference` remainder, with
  the `reference / demo_flat_codomain` remaining-two pairing as the first
  narrow blocker to probe before reopening the smaller sibling pairings, the
  reference/reference tail, or the whole top-level
  `claim_eventual_domain` branch.

- Scope: localize the repaired-head `reference / demo_flat_codomain` blocker
  one layer deeper below the mismatch-`1` direct-`reference` split.
  Result:
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_live_reference_demo_flat_remainder_splits_the_next_blocker_between_a_33_capture_remaining_two_bridge_side_and_a_12_capture_remaining_three_reference_spill`
  - reran
    `cargo test -p pen-search current_claim_step_fifteen_live_reference_ -- --nocapture`
    and
    `cargo test -p pen-search current_claim_step_fifteen_live_repair_partial_prefix_wall_localizes_to_eight_frontier_remainder_branches_on_the_repaired_head -- --nocapture`;
    both passed
  - on the repaired `7211 / 553 / 2052` head, the
    `reference / demo_flat_codomain` branch now totals `73` captures:
    `61` remaining-two plus `12` remaining-three
  - inside that branch, the remaining-two half splits first as
    clause-`4` `claim_next_bridge = 33` versus
    clause-`4` `reference = 28`
  - the remaining-three spill is fully localized too: all `12` captures stay
    on clause-`4` `reference`, evenly `4 / 4 / 4` across clause-`2`
    `claim_flat_domain / claim_sharp_codomain / reference`
  Consequence: the next honest local repair lead is no longer the whole
  `reference / demo_flat_codomain = 61` pairing; it is the remaining-two
  clause-`4` `claim_next_bridge = 33` side first, with the remaining-two
  clause-`4` `reference = 28` companion and the `12`-capture remaining-three
  reference spill as the immediate follow-ons before reopening sibling
  mismatch-`1` pairings or the whole top-level
  `claim_eventual_domain` branch.

- Scope: test the first repaired-head exact claim-pair reopening under the
  direct `reference / demo_flat_codomain` clause-`4`
  `claim_next_bridge` side.
  Result:
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_live_reference_demo_flat_claim_next_bridge_exact_claim_variant_pair_stays_a_tradeoff_control_on_the_repaired_head`
  - reran
    `cargo test -p pen-search current_claim_step_fifteen_live_reference_ -- --nocapture`
    and
    `cargo test -p pen-search current_claim_step_fifteen_live_repair_partial_prefix_wall_localizes_to_eight_frontier_remainder_branches_on_the_repaired_head -- --nocapture`;
    both passed
  - on top of the repaired
    `7211 / 553 / 2052` head, the exact claim-pair plus clause-`4`
    `claim_next_bridge` reland moves the live surface to
    `generated_raw_prefixes = 7485`,
    `partial_prefix_bar_failure = 539`,
    and
    `small_cluster = 2112 / 542 / 542 / 0`
  - the direct top-level `reference` remainder contracts from `241` to `227`,
    the targeted `reference / demo_flat_codomain` pairing contracts from `61`
    to `45`, and the branch-local slot split becomes `27 / 18 / 14`
  - the repaired-head branch grid stays explicit too:
    the remaining-two side becomes a symmetric `9 / 9 / 9`
    clause-`4` `claim_next_bridge` row above a `6 / 6 / 6`
    clause-`4` `reference` row, while the remaining-three clause-`4`
    `reference` spill widens from `4 / 4 / 4` to `5 / 5 / 4`
  Consequence: keep that repaired-head exact claim-pair family frozen as a
  tradeoff control and move below it to one tied clause-`2` claim sheet
  (`claim_flat_domain = 12` or `claim_sharp_codomain = 12`) inside the
  remaining-two clause-`4` `claim_next_bridge = 33` side before reopening the
  smaller clause-`2` `reference = 9` sheet, the clause-`4`
  `reference = 28` companion, or the remaining-three spill.

- Scope: test the tied repaired-head exact single-sheet reopenings under the
  direct `reference / demo_flat_codomain` clause-`4`
  `claim_next_bridge` side.
  Result:
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_live_reference_demo_flat_claim_next_bridge_exact_claim_sheets_stay_matched_smaller_tradeoff_controls_on_the_repaired_head`
  - reran
    `cargo test -p pen-search current_claim_step_fifteen_live_reference_ -- --nocapture`
    and
    `cargo test -p pen-search current_claim_step_fifteen_live_repair_partial_prefix_wall_localizes_to_eight_frontier_remainder_branches_on_the_repaired_head -- --nocapture`;
    both passed
  - on top of the repaired `7211 / 553 / 2052` head, both exact
    `claim_flat_domain` and exact `claim_sharp_codomain` relands land at
    `generated_raw_prefixes = 7317`,
    `partial_prefix_bar_failure = 545`,
    and
    `small_cluster = 2076 / 530 / 530 / 0`
  - the direct top-level `reference` remainder contracts from `241` to `233`,
    the targeted `reference / demo_flat_codomain` branch contracts from `73`
    to `65` total captures (`51` remaining-two plus `14` remaining-three),
    and the branch-local slot split becomes `30 / 21 / 14`
  Consequence: keep both tied repaired-head claim-sheet families frozen as
  matched smaller tradeoff controls and move below them to the smaller
  clause-`2` `reference = 9` sheet before reopening the clause-`4`
  `reference = 28` companion or the remaining-three spill.

- Scope: test the repaired-head exact clause-`2` `reference` sheet under the
  direct `reference / demo_flat_codomain` clause-`4`
  `claim_next_bridge` side.
  Result:
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_live_reference_demo_flat_claim_next_bridge_exact_reference_sheet_stays_a_safe_side_control_on_the_repaired_head`
  - added direct `pen-type` matcher coverage for
    `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_reference_sheet_under_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_reference_sheet_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_reference_clause_two_sheet_even_under_claim_next_bridge_side_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_even_under_claim_next_bridge_side_on_exact_reference_sheet_override`,
    and
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_reference_sheet_reference_terminal_only_even_under_override`
  - reran
    `cargo test -p pen-search current_claim_step_fifteen_live_reference_ -- --nocapture`,
    `cargo test -p pen-type claim_next_bridge_side_on_exact_reference_sheet -- --nocapture`,
    and
    `cargo test -p pen-type outside_exact_reference_clause_two_sheet -- --nocapture`;
    all passed
  - on top of the repaired `7211 / 553 / 2052` head, the exact
    clause-`2` `reference` reland lands at
    `generated_raw_prefixes = 7149`,
    `partial_prefix_bar_failure = 551`,
    `small_cluster = 2040 / 518 / 518 / 0`,
    direct `reference` remainder `= 239`,
    `reference / demo_flat_codomain = 57`,
    and branch-local slot split `33 / 24 / 14`
  - that control stays reference-terminal-only, still requires the exact
    anchor-`11` side pocket, keeps the actual clause-`4` `reference` sheet
    closed, and keeps the claim-flat / claim-sharp clause-`2` sheets fenced
  Consequence: freeze the exact clause-`2` `reference` reland as the first
  non-widening safe side-control below the repaired head. It drops the wall
  below `553`, but it does so by trimming only the clause-`4`
  `reference` companion while leaving the live clause-`4`
  `claim_next_bridge = 33` blocker untouched, so the next honest move is to
  see whether this safe side-control can be turned into a real repaired-head
  landing candidate before reopening broader clause-`4` or sibling mismatch
  families.

- Scope: test whether the repaired-head exact clause-`2` `reference` sheet can
  become a real landing candidate by reopening its clause-`4`
  `reference` companion under the same exact reference-sheet control.
  Result:
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_live_reference_demo_flat_exact_reference_sheet_clause_four_reference_companion_stays_neutral_on_the_repaired_head`
  - added direct `pen-type` matcher coverage for
    `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_reference_sheet_under_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_reference_sheet_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_reference_clause_two_sheet_even_under_clause_four_reference_side_on_exact_reference_sheet_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_closed_even_under_clause_four_reference_side_on_exact_reference_sheet_override`,
    and
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_reference_sheet_reference_terminal_only_even_under_override`
  - reran
    `cargo test -p pen-type exact_reference_sheet -- --nocapture`
    and
    `cargo test -p pen-search current_claim_step_fifteen_live_reference_ -- --nocapture`;
    both passed
  - on top of the repaired
    `7211 / 553 / 2052` head, the combined exact reference-sheet candidate is
    completely neutral:
    it relands the same
    `generated_raw_prefixes = 7149`,
    `partial_prefix_bar_failure = 551`,
    `small_cluster = 2040 / 518 / 518 / 0`,
    direct `reference` remainder `= 239`,
    `reference / demo_flat_codomain = 57`,
    and branch-local slot split `33 / 24 / 14`
  - that neutral result also pins the exact-reference-sheet grid one layer
    deeper:
    remaining-two clause-`4` `claim_next_bridge` stays
    `12 / 12 / 9`,
    remaining-two clause-`4` `reference` stays
    `9 / 9 / 6`,
    and the remaining-three clause-`4` `reference` spill stays
    `5 / 5 / 4`
    across clause-`2`
    `claim_flat_domain / claim_sharp_codomain / reference`
  Consequence: freeze the whole exact-reference-sheet family as the last safe
  inert control below the repaired head. The next honest move is now to step
  past it and reopen the broader remaining-two clause-`4`
  `reference = 28` companion itself, starting from one tied clause-`2`
  claim sheet at `9` before the smaller exact `reference = 6` slice and the
  `5 / 5 / 4` remaining-three spill.

- Scope: test the first repaired-head exact `claim_flat_domain` and exact
  `claim_sharp_codomain` reopenings under the broader clause-`4`
  `reference` companion below the inert exact-reference-sheet family.
  Result:
  - added direct `pen-search` regression coverage for
    `current_claim_step_fifteen_live_reference_demo_flat_clause_four_reference_exact_claim_sheets_stay_remaining_three_only_tradeoff_controls_on_the_repaired_head`
  - added direct `pen-type` matcher coverage for
    `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_claim_flat_sheet_under_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_claim_flat_sheet_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_flat_clause_two_sheet_even_under_clause_four_reference_side_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_closed_even_under_clause_four_reference_side_on_exact_claim_flat_sheet_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_claim_flat_sheet_reference_terminal_only_even_under_override`,
    `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_claim_sharp_sheet_under_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_claim_sharp_sheet_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_sharp_clause_two_sheet_even_under_clause_four_reference_side_override`,
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_closed_even_under_clause_four_reference_side_on_exact_claim_sharp_sheet_override`,
    and
    `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_claim_sharp_sheet_reference_terminal_only_even_under_override`
  - reran
    `cargo test -p pen-type clause_four_reference_side -- --nocapture`
    and
    `cargo test -p pen-search current_claim_step_fifteen_live_reference_ -- --nocapture`;
    both passed
  - on top of the repaired
    `7211 / 553 / 2052` head, both exact claim-sheet relands land at
    `generated_raw_prefixes = 7236`,
    `partial_prefix_bar_failure = 550`,
    `small_cluster = 2058 / 524 / 524 / 0`,
    direct `reference` remainder `= 238`,
    `reference / demo_flat_codomain = 57`,
    and branch-local slot split `33 / 24 / 13`
  - both controls leave the remaining-two clause-`4` `reference` row pinned
    at
    `9 / 9 / 6`
    and trim only one remaining-three clause-`4` `reference` capture:
    the exact `claim_flat_domain` reland lands
    `4 / 5 / 4`,
    while the exact `claim_sharp_codomain` reland lands
    `5 / 4 / 4`
    across clause-`2`
    `claim_flat_domain / claim_sharp_codomain / reference`
  Consequence: freeze both exact claim-sheet reopenings under the broader
  clause-`4` `reference` companion as spill-only tradeoff controls. They do
  not touch the remaining-two `9 / 9 / 6` lead, so the next honest move is
  now below them on the smaller exact clause-`2` `reference = 6` slice rather
  than back through the already-frozen claim sheets, the inert exact-
  reference-sheet family, or the whole remaining-three spill.
