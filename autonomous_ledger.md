# Autonomous Claim Lane Ledger

Last updated: 2026-04-10

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
