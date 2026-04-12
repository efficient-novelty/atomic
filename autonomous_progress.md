# Autonomous Claim Lane State

Last updated: 2026-04-12

This file owns the live operational snapshot for `desktop_claim_shadow`.

## Contract

- Keep canonical run IDs, current counters, the current accepted chain, and the
  current diagnosis here.
- Do not store medium-horizon phases here. Use
  [autonomous_plan.md](autonomous_plan.md).
- Do not store the per-turn work order here. Use
  [autonomous_next_steps.md](autonomous_next_steps.md).
- Do not store binary signoff gates or probe history here. Use
  [autonomous_checklist.md](autonomous_checklist.md) and
  [autonomous_ledger.md](autonomous_ledger.md).

## Mission

Produce one stored `desktop_claim_shadow` bundle that:

- completes through step `15`
- keeps accepted-hash parity through step `15`
- passes compare, benchmark, and certification from stored evidence

Until that exists, wording stays at `bounded live recovery`.

## Canonical Baseline

- Canonical stored bundle:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13`
- Stored compare is ready.
- Stored benchmark is refreshed across `v11`, `v12`, and `v13`.
- Stored `v11`, `v12`, and `v13` keep the same early breadth miss:
  - step `1 = 546 / 2144` on all three runs
  - all three keep the same step-`01` surface:
    `generated = 546`, `well_formed = 288`, `admitted = 1`,
    `legality_connectivity_exact_rejection = 435`
- Stored `v12` and `v13` also keep the same late breadth miss:
  - step `15 = 4331 / 5000`
  - both keep step-`15` at `generated = 4331`,
    `partial_prefix_bar_failure = 553`, `incumbent_dominance = 3`, and
    `small_cluster generated = 3132`
- Accepted-hash parity is earned through step `15`.
- Refreshed benchmark keeps all three stored reruns under the runtime
  threshold, with `v13` at `4387 ms`.
- Stored certification still fails only on breadth:
  - step `1 = 546 / 2144`
  - step `15 = 4331 / 5000`
- The canonical stored bundle is now on the newer post-probe codebase, and it
  re-confirms step `15` as the active stored miss rather than reopening
  step `1`.

## Current Late Surface

- Clean stored and local guardrail chain:
  - step `13 = [5,1,3,3,5,3,2] / 1350 / 2320`
  - step `14 = 62 / 9 / 12027`
  - step `15 = DCT 103 / 8 / 4331`
- Current step-`15` pressure:
  - partial-prefix bar failures: `553`
  - incumbent-dominance prunes: `3`
  - `small_cluster = 3132 / 522 / 522 / 0`
  - fenced `single` bucket = `1` fully scored non-winner plus `3` residual
    prunes at overshoot `115657 / 21112`
- Executable wall split:
  - remaining-two prefixes: `451`
  - remaining-three prefixes: `102`
  - first mismatch positions: clause `0 = 312`, clause `1 = 177`,
    clause `2 = 50`, clause `3 = 14`
  - dominant remaining-two slice: clause `0 = 252`, clause `1 = 145`

## Active Diagnosis

- The old broad `small_cluster` incumbent wall is no longer the primary
  blocker. The dominant live miss is the clean step-`15` partial-prefix wall
  on the canonical `4331` surface.
- The rerun-backed step-`15` reset is now confirmed on the newer codebase:
  - `v11`, `v12`, and `v13` keep the same step-`1` fail and the same
    step-`01` surface
  - `v12` and `v13` also keep the same late stored surface on different build
    commits:
    `generated = 4331`,
    `partial_prefix_bar_failure = 553`,
    `incumbent_dominance = 3`, and
    `small_cluster generated = 3132`
  - `v13` keeps accepted parity, claim search policy honesty, narrative
    coverage, compare readiness, benchmark freshness, and runtime-threshold
    pass on the current head
  - consequence: canonical `v13` keeps step `15` as the active stored miss,
    and step `1` stays deferred until a later stored bundle changes the
    diagnosis
- The dominant remaining-two live surface is already executable:
  - exact clause-`0` / clause-`1` pairings are frozen by
    `current_claim_step_fifteen_remaining_two_partial_prefix_wall_stays_on_nine_clause_zero_one_pairings`
  - clause-`4` pressure is frozen to `claim_next_bridge` plus `reference`
  - clause-`2` pressure stays on the current claim variants rather than on a
    hidden demo-only reopening
- The best known clean wall-narrowing tradeoff is now the whole mismatch-`0`
  clause-`4` `claim_next_bridge`-half remaining-one exact-summary relief:
  - it lands `4619 / 529 / 2199`
  - it widens `small_cluster` to `3348 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - each live mismatch-`0` pairing contracts from `42` to `38`
  - the clause-`4` split contracts only the `claim_next_bridge` share, from
    `24 / 18` to `20 / 18`
  - consequence: the first honest wall reduction on the mismatch-`0`
    claim-domain surface now sits on the whole `claim_next_bridge` half, but
    it is still only a tradeoff because the noncanonical `small_cluster`
    shell grows
- That whole-half tradeoff now also splits cleanly one layer deeper across its
  clause-`5` families:
  - the `claim_flat_codomain` cell lands `4475 / 541 / 2235`
  - the `reference` cell lands the same `4475 / 541 / 2235`
  - each smaller tradeoff widens `small_cluster` to `3240 / 522 / 522 / 0`
  - each smaller tradeoff contracts every live mismatch-`0` pairing from
    `42` to `40`
  - each smaller tradeoff contracts only the clause-`4`
    `claim_next_bridge` share from `24 / 18` to `22 / 18`
  - the sibling `claim_next_codomain` cell is a neutral control on the
    untouched `4331 / 553 / 2271` baseline
  - consequence: the whole-half `529` tradeoff is exactly the union of the
    `claim_flat_codomain` and `reference` cells under the mismatch-`0`
    `claim_next_bridge` half, while `claim_next_codomain` is no longer a live
    lead
- The exact claim-pair clause-`4` `reference`-side relocalization is now also
  isolated and exhausted as a smaller tradeoff control:
  - it lands the same `4379 / 549 / 2259` surface as the older broader
    clause-`4` `reference`-sheet probe
  - it still widens `small_cluster` to `3180 / 530 / 530 / 0`
  - the dominant mismatch-`1` `reference + demo_flat_codomain` branch stays at
    `57` captured prefixes with the same clause-`4` split `33 / 24`
  - its bridge grid matches the broader clause-`4` `reference`-sheet tradeoff
    exactly, so that older tradeoff already lived entirely on the exact claim
    clause-`2` pair rather than on the reference clause-`2` sheet
- The exact claim-pair clause-`4` `claim_next_bridge` half is now also
  localized one layer deeper and exhausted at single-sheet scope:
  - the exact claim-flat and exact claim-sharp sub-slices each land the same
    `4373 / 545 / 2259` surface
  - they also widen `small_cluster` to `3180 / 530 / 530 / 0`
  - the dominant mismatch-`1` `reference + demo_flat_codomain` branch drops
    only to `51` captured prefixes with clause-`4` split `30 / 21`
  - each probe only chooses which claim clause-`2` sheet falls to the smaller
    `15`-capture floor while the sibling claim sheet stays at `21` and the
    reference sheet stays at `15`
- The remaining-two wall outside that exhausted mismatch-`1`
  `reference + demo_flat_codomain` ladder is now also executable:
  - excluding the frozen `61`-capture ladder leaves `390` off-branch captures
    across `10` pairings
  - mismatch-`0` claim-domain pairings now own the first priority tier at
    `252`
  - claim-safe mismatch-`1` pairings are the smaller second tier at `84`
  - mismatch-`2` plus mismatch-`3` `reference / reference` tails are only the
    residual `54`
- That promoted `reference / reference` tail is now also localized one layer
  deeper by mismatch position and clause-`4` anatomy:
  - mismatch-`2` is the larger lead at `42`
  - mismatch-`3` is the smaller backup at `12`
  - within mismatch-`2`, clause-`4` stays split across
    `claim_next_bridge = 18`, `reference = 16`,
    `demo_sharp_codomain = 4`, and `demo_sharp_bridge = 4`
  - within mismatch-`3`, clause-`4` stays only on
    `claim_next_bridge = 6` plus `reference = 6`
  - consequence: the lumped `reference / reference` tail is no longer the
    next honest unit of work; the later probes have now spent the whole
    mismatch-`2` clause-`4` anatomy, so the remaining live slice below that
    tail is the smaller mismatch-`3` backup
- The mismatch-`2` `reference / reference` clause-`4`
  `claim_next_bridge` half is now also localized as a smaller tradeoff
  control:
  - a narrow exact-bound override on that half lands `4547 / 535 / 2271`
  - it widens `small_cluster` to `3294 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - the exact mismatch-`2` pair contracts from `42` to `24`
  - the smaller mismatch-`3` backup stays untouched at `12`
  - consequence: the larger mismatch-`2` tail is not safely repairable on its
    whole clause-`4` `claim_next_bridge` half, because the wall win still
    depends on a wider noncanonical `small_cluster`
- The mismatch-`2` `reference / reference` clause-`4` `reference` half is now
  also localized as a sharper tradeoff control:
  - a narrow exact-bound override on that half lands `4835 / 529 / 2271`
  - it widens `small_cluster` to `3492 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - the exact mismatch-`2` pair contracts from `42` to `26`
  - the mismatch-`2` remaining-three spill also contracts by `8`, from
    `102` to `94`
  - the smaller mismatch-`3` backup stays untouched at `12`
  - consequence: the mismatch-`2` wall is not safely repairable on its whole
    clause-`4` `reference` half either; that half buys a larger wall win only
    by widening `small_cluster` even more aggressively than the sibling
    `claim_next_bridge` tradeoff
- The mismatch-`2` `reference / reference` clause-`4`
  `demo_sharp_codomain` pocket is now also localized as a smaller tradeoff
  control:
  - a narrow exact-bound override on that pocket lands `4379 / 549 / 2271`
  - it widens `small_cluster` only to `3168 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - the exact mismatch-`2` pair contracts from `42` to `38`
  - it removes only the tiny `demo_sharp_codomain`
    `claim_flat_codomain / claim_next_codomain` `2 / 2` cells while leaving
    the larger mismatch-`2` `claim_next_bridge` and `reference` halves, the
    sibling `demo_sharp_bridge` pocket, and the mismatch-`3` backup untouched
- The mismatch-`2` `reference / reference` clause-`4`
  `demo_sharp_bridge` pocket is now also localized as the same smaller
  tradeoff control:
  - a narrow exact-bound override on that pocket also lands
    `4379 / 549 / 2271`
  - it widens `small_cluster` only to `3168 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - the exact mismatch-`2` pair also contracts from `42` to `38`
  - it removes only the tiny `demo_sharp_bridge`
    `claim_flat_codomain / claim_next_codomain` `2 / 2` cells while leaving
    the larger mismatch-`2` `claim_next_bridge` and `reference` halves, the
    sibling `demo_sharp_codomain` pocket, and the mismatch-`3` backup
    untouched
- The mismatch-`3` `reference / reference` clause-`4`
  `claim_next_bridge` half is now also localized as a smaller tradeoff
  control:
  - a narrow exact-bound override on that half lands `4403 / 547 / 2271`
  - it widens `small_cluster` only to `3186 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - the exact mismatch-`3` pair contracts from `12` to `6`
  - the larger spent mismatch-`2` tail stays untouched at `42`
  - consequence: the smaller mismatch-`3` backup is not safely repairable on
    its whole clause-`4` `claim_next_bridge` half either; the wall win is
    real, but it still depends on a wider noncanonical `small_cluster`
- The mismatch-`3` `reference / reference` clause-`4` `reference` half is now
  also localized as a sharper tradeoff control:
  - a narrow exact-bound override on that half lands `4481 / 545 / 2271`
  - it widens `small_cluster` to `3240 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - the exact mismatch-`3` pair also contracts from `12` to `6`
  - the mismatch-`3` remaining-three spill also contracts by `2`, from
    `102` to `100`
  - the larger spent mismatch-`2` tail stays untouched at `42`
  - consequence: the smaller mismatch-`3` backup is not safely repairable on
    its whole clause-`4` `reference` half either; that sharper wall win still
    widens `small_cluster` more aggressively than the sibling
    `claim_next_bridge` tradeoff
- Any remaining `reference / reference` leverage is therefore no longer on
  either mismatch-`2` clause-`4` half, either mismatch-`2` demo-side pocket,
  or either mismatch-`3` clause-`4` half. The full tail is now spent as
  tradeoff-only control, so the next honest slice has to compare alternate
  broader backups rather than reopening the same tail again.
- That alternate broader-backup comparison is now explicit:
  - the representative mismatch-`0` claim-side clause-`2` shell remains the
    tighter backup at `4343 / 552 / 2268` with
    `small_cluster = 3141 / 522 / 522 / 0`
  - the representative claim-safe claim-side clause-`2` shell is the looser
    secondary backup at `4347 / 555 / 2277` with
    `small_cluster = 3144 / 522 / 522 / 0`
  - the claim-safe shell buys only `4` extra generated prefixes by spending
    `3` extra clean-wall captures, `3` extra `small_cluster` generated
    terminals, and `9` extra zero-admitted captures
  - the tighter mismatch-`0` shell also contracts the first-mismatch-`0` tier
    from `312` to `311`, while the claim-safe shell leaves mismatch-`0`
    untouched and instead inflates first-mismatch-`1` from `177` to `179`
  - consequence: the representative mismatch-`0` claim-side shell stays
    promoted ahead of the representative claim-safe shell
- The isolated clause-`1` `demo_flat_codomain` exact-suffix side pocket is
  now also exhausted as a looser side-pocket control:
  - a search-only exact-bound override on that lone isolated exact-suffix
    prefix lands `4349 / 556 / 2268`
  - it widens `small_cluster` to `3141 / 522 / 522 / 0`
  - accepted step `15` still stays canonical `103 / 8`
  - `terminal_prefix_completion_failure` stays `0`
  - the isolated `single` pocket stays fenced
  - consequence: that exact-suffix side pocket is not the missing clean-wall
    repair; it only reproduces the same wider mismatch-`0` broader-backup
    shell while also worsening the clean wall from `553` to `556`
- A direct parent-level exact-bound bypass on the whole remaining-two
  mismatch-`0` claim-domain tier is now ruled out:
  - redirecting that tier from `CannotClearBar` to `Unknown` at
    `exact_partial_prefix_bound_decision(...)` dropped the clean wall from
    `553` to `241`, which means the full `312` first-mismatch-`0` surface
    escaped exact pruning
  - `small_cluster` generated-terminal pressure jumped from `3132` to `7020`
  - under the existing mismatch-`0` demo-flat override, generated breadth
    jumped from `4985` to `10349`
  - the isolated `single` pocket stayed fenced
  - consequence: exact-bound relief has to stay below the whole
    mismatch-`0` tier and work one layer deeper inside the live clause-`4` /
    clause-`5` split or the remaining-one exact-summary path
- The broad clause-`1` `demo_flat_codomain` reopening across that
  mismatch-`0` claim-domain tier is now also ruled out as a widening negative
  control:
  - it lands `4985 / 667 / 2757`
  - it widens `small_cluster` to `3564 / 594 / 594 / 0`
  - the isolated `single` pocket stays fenced
  - it adds two new mismatch-`0` pairings at `45 / 45`
  - each new pairing sits on the same clause-`4` split `27 / 18`
  - each new pairing also spreads evenly across the three clause-`2` sheets at
    `15 / 15 / 15`
- The narrower mismatch-`0` clause-`4` `claim_next_bridge`-side relocalization
  under that same demo-flat reopening is now also ruled out as a smaller
  negative control:
  - it lands `4829 / 671 / 2793`
  - it widens `small_cluster` to `3420 / 570 / 570 / 0`
  - the isolated `single` pocket stays fenced
  - it leaves the same two mismatch-`0` pairings at `45 / 45`
  - it leaves the same clause-`4` split `27 / 18`
  - it leaves the same clause-`2` spread `15 / 15 / 15`
  - consequence: the remaining mismatch-`0` leverage is not isolated on a
    connectivity-only relocalization of the clause-`4`
    `claim_next_bridge` half
- The smaller mismatch-`0` clause-`4` `reference`-side relocalization under
  that same demo-flat reopening is now also ruled out as a sharper negative
  control:
  - it lands `4697 / 691 / 2829`
  - it widens `small_cluster` to `3276 / 546 / 546 / 0`
  - the isolated `single` pocket stays fenced
  - it inflates the same two mismatch-`0` pairings to `57 / 57`
  - it widens the clause-`4` split to `33 / 24`
  - it leans the clause-`2` spread toward the two claim sheets at
    `21 / 21 / 15`
  - consequence: the remaining mismatch-`0` leverage is not isolated on
    either clause-`4` half; the next slice should move below clause-`4`
    relocalizations and into remaining-one exact-summary behavior under the
    same live pairings before revisiting smaller claim-safe mismatch-`1`
    surfaces
- The narrower remaining-one exact-summary relief on the existing mismatch-`0`
  clause-`4` `reference` plus clause-`5` `reference` tail is now also ruled
  out as a deeper negative control:
  - a search-only exact-summary override on that live cell lands
    `4547 / 589 / 2235`
  - it widens `small_cluster` to `3240 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - each live mismatch-`0` pairing inflates from `42` to `52`
  - the clause-`4` split leans harder toward `reference`, from `24 / 18` to
    `24 / 28`
  - consequence: even summary-only relief on the narrow reference tail widens
    the mismatch-`0` wall instead of converting that tail into honest breadth
- The whole mismatch-`0` clause-`4` `claim_next_bridge` half is now also
  localized as a real remaining-one exact-summary tradeoff:
  - a search-only exact-summary override on that live half lands
    `4619 / 529 / 2199`
  - it widens `small_cluster` to `3348 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - each live mismatch-`0` pairing contracts from `42` to `38`
  - the clause-`4` split contracts only the `claim_next_bridge` share, from
    `24 / 18` to `20 / 18`
  - consequence: remaining mismatch-`0` leverage is now genuinely localized
    below the `claim_next_bridge` half, but the next slice still has to move
    one layer deeper into its clause-`5` cells to preserve the wall win
    without widening `small_cluster`
- The clause-`5` split under that whole-half tradeoff is now executable too:
  - the `claim_flat_codomain` and `reference` cells are symmetric smaller
    tradeoff controls at `4475 / 541 / 2235`
  - the `claim_next_codomain` cell is neutral on `4331 / 553 / 2271`
  - consequence: the next revisit should move below the two active
    `claim_flat_codomain / reference` cells and leave the inert
    `claim_next_codomain` sibling alone
- Those two active clause-`5` cells are now also exhausted one layer deeper
  at pair-cell scope:
  - every live clause-`0` / clause-`1` pair under either active clause-`5`
    family lands the same `4355 / 551 / 2265` smaller tradeoff
  - `small_cluster` widens only to `3150 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - each probe contracts only its targeted live pair from `42` to `40`
  - each probe contracts only its targeted clause-`4` `claim_next_bridge`
    plus active clause-`5` bucket from `48` to `46`
  - consequence: the clause-`0` / clause-`1` axis and the active
    clause-`5` family choice are now both exhausted at pair-cell scope, so
    the next revisit has to move one layer deeper below a representative pair
    cell rather than comparing more equivalent pair-cell relands
- The representative mismatch-`0` pair cell is now also localized one layer
  deeper across its clause-`2` sheets:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_next_bridge / claim_flat_codomain`,
    the `claim_flat_domain` and `claim_sharp_codomain` sheets each land the
    same smaller tradeoff at `4343 / 552 / 2268`
  - each widens `small_cluster` only to `3141 / 522 / 522 / 0`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline
  - each claim-side probe lowers only its own clause-`2` share from `15` to
    `14`, while the sibling claim sheet stays at `15` and the reference sheet
    stays at `12`
  - the representative pair's clause-`4` split contracts only the
    `claim_next_bridge` share, from `24 / 18` to `23 / 18`
  - the representative clause-`4` `claim_next_bridge` plus clause-`5`
    `claim_flat_codomain` bucket contracts only from `48` to `47`
  - consequence: the representative pair-cell `4355 / 551 / 2265` tradeoff is
    exactly the union of the two claim-side clause-`2` sheets, so the next
    revisit has to move below clause-`2` sheet identity into a finer
    remaining-one exact-summary partition under one representative claim-side
    sheet rather than swapping between the two claim sheets again
- The sibling active clause-`5` `reference` family's representative
  mismatch-`0` pair cell is now also localized one layer deeper across its
  clause-`2` sheets:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_next_bridge / reference`,
    the `claim_flat_domain` and `claim_sharp_codomain` sheets each reland the
    same smaller tradeoff at `4343 / 552 / 2268`
  - each keeps `small_cluster` at `3141 / 522 / 522 / 0`
  - each lowers only its own clause-`2` share from `15` to `14`, while the
    sibling claim sheet stays at `15` and the sibling `reference` sheet stays
    at `12`
  - each contracts the representative pair's clause-`4` split only from
    `24 / 18` to `23 / 18`
  - each contracts the representative clause-`4` `claim_next_bridge` plus
    clause-`5` `reference` bucket only from `48` to `47`
  - the sibling `reference` sheet is a neutral control on
    `4331 / 553 / 2271`
  - consequence: the sibling active clause-`5` `reference` family repeats the
    same claim-side clause-`2` tradeoff one layer deeper too, so clause-`5`
    family identity is now exhausted below representative pair-cell scope as
    well
- The representative `claim_flat_domain` clause-`2` sheet is now also
  localized one layer deeper across its clause-`6` remaining-one
  continuations:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain /
    claim_next_bridge / claim_flat_codomain`,
    the clause-`6` `claim_next_codomain`, `claim_sharp_codomain`, and
    `reference` continuations all land the same matched smaller tradeoff shell
    at `4343 / 552`
  - each widens `small_cluster` only to `3141 / 522 / 522 / 0`
  - each keeps the representative clause-`2` spread at `14 / 15 / 12`
  - each keeps the representative clause-`4` split at `23 / 18`
  - each keeps the representative clause-`4` `claim_next_bridge` plus
    clause-`5` `claim_flat_codomain` bucket at `47`
  - they differ only in the deeper zero-admitted tail:
    `claim_next_codomain = 2270`, `claim_sharp_codomain = 2269`,
    `reference = 2268`
  - consequence: clause-`6` identity is now also exhausted as full-profile
    leverage below that representative claim-flat sheet, so the next revisit
    has to move below the marginally best `reference` clause-`6`
    continuation rather than swapping among the three clause-`6` siblings
- The representative `claim_flat_domain` clause-`2` sheet's marginally best
  clause-`6` `reference` continuation is now also exhausted one layer deeper
  at clause-`3` identity scope:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain /
    claim_next_bridge / claim_flat_codomain / reference`,
    the `claim_flat_argument` and `claim_eventual_argument` clause-`3`
    branches are each individually neutral controls on the untouched
    `4331 / 553 / 2271` baseline
  - each keeps `small_cluster` at `3132 / 522 / 522 / 0`
  - each keeps first-mismatch counts at `312 / 177 / 50 / 14`
  - consequence: the broader clause-`6` `reference` `4343 / 552 / 2268`
    tradeoff does not localize to either clause-`3` branch alone; it appears
    only when both clause-`3` branches reopen together, so the next revisit
    has to move below clause-`3` identity into a finer terminal or
    remaining-one completion partition under that joint continuation
- That broader joint clause-`3` continuation is now also localized one layer
  deeper as a single parent-plus-children shell:
  - relative to either individually neutral clause-`3` branch, the broader
    clause-`6` `reference` tradeoff removes exactly one remaining-two parent
    capture under
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / claim_flat_codomain`
  - the entire zero-admitted delta is exactly that parent's three child
    remaining-one clause-`6` continuations:
    `claim_next_codomain`, `claim_sharp_codomain`, and `reference`
  - no new partial-prefix captures or remaining-one zero-admitted prunes
    appear elsewhere
  - consequence: any remaining leverage below clause-`3` identity is no
    longer on another whole-joint-continuation reopen; it has to partition the
    single remaining-two parent's remaining-one completion / terminal subcells
    beneath that same `claim_flat_codomain` tail
- That representative `claim_flat_domain` parent-plus-children shell is now
  also exhausted one layer deeper at child completion-summary scope:
  - across both clause-`3` branches and all three clause-`6` child labels,
    the six remaining-one child continuations all collapse to the same dead
    completion summary
  - each child keeps only `3` generated terminal candidates and `0` admitted
    candidates
  - each child keeps no bound, no best-rank profile, and no survivor sketch
  - each child exposes the same three local terminal choices only:
    `reference`, `eventual_lift`, and `next_lift`
  - each of those three local choices is still
    `KeepWithoutFallback` plus locally `open_band_structural`, but none passes
    live connectivity on the current claim path
  - consequence: the representative claim-flat parent/child shell no longer
    hides a smaller live terminal pocket, so any remaining mismatch-`0`
    leverage is no longer on claim-flat clause-`3` or clause-`6` child
    identity either; the later claim-sharp probe then moved sideways onto the
    representative `claim_sharp_codomain` clause-`2` sheet rather than
    reopening the same claim-flat shell
- The representative `claim_sharp_codomain` clause-`2` sheet is now also
  exhausted at clause-`6` identity scope:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain /
    claim_next_bridge / claim_flat_codomain`,
    the clause-`6` `claim_next_codomain`, `claim_sharp_codomain`, and
    `reference` continuations all reland the same matched smaller tradeoff
    shell at `4343 / 552`
  - each widens `small_cluster` only to `3141 / 522 / 522 / 0`
  - each keeps the representative clause-`2` spread at `15 / 14 / 12`
  - each keeps the representative clause-`4` split at `23 / 18`
  - each keeps the representative clause-`4` `claim_next_bridge` plus
    clause-`5` `claim_flat_codomain` bucket at `47`
  - they differ only in the deeper zero-admitted tail:
    `claim_next_codomain = 2270`, `claim_sharp_codomain = 2269`,
    `reference = 2268`
  - consequence: clause-`6` identity is now exhausted on the representative
    claim-sharp sheet too, and it only reproduces the same smaller tradeoff
    shell already seen on the representative claim-flat sheet
- The clause-`4` `reference` remaining-three tail is exhausted as a cleaner
  target:
  - exact clause-`5` `reference` is neutral on `4331 / 553 / 2271`
  - exact clause-`5` `claim_flat_codomain` and `claim_next_codomain` are only
    symmetric smaller tradeoff controls at `4355 / 551 / 2265`
- The exact remaining-two mismatch-`1` clause-`5` bridge-slice reopening is
  also ruled out:
  - it lands only `4511 / 571 / 2325`
  - it widens `small_cluster` to `3276 / 546 / 546 / 0`
  - it inflates the three mismatch-`1` pairings to `48 / 48 / 67`
  - it leaves the live clause-`4` mismatch-`1` split unchanged at
    `24 / 18`, `24 / 18`, and `33 / 28`
- The representative claim-safe mismatch-`1` clause-`4`
  `demo_sharp_codomain` side is now also localized one layer deeper across
  its two exact pair cells:
  - the `reference / claim_next_codomain` pair cell lands
    `4363 / 557 / 2283`
  - the `reference / claim_sharp_codomain` pair cell lands the same
    `4363 / 557 / 2283`
  - each widens `small_cluster` only to `3156 / 526 / 526 / 0`
  - the isolated `single` pocket stays fenced
  - each raises only its targeted claim-safe mismatch-`1` pairing from
    `42` to `46`
  - each keeps that pairing's incumbent clause-`4`
    `claim_next_bridge / reference` split at `24 / 18`
  - each adds only a tiny clause-`4` `demo_sharp_codomain` side pocket of `4`
    while leaving the sibling claim-safe pairing frozen at `42`
  - consequence: pair identity is now exhausted below that representative
    claim-safe clause-`4` side too
- The representative claim-safe mismatch-`1` clause-`4`
  `demo_sharp_bridge` side is now also localized one layer deeper across its
  two exact pair cells:
  - the `reference / claim_next_codomain` pair cell also lands
    `4363 / 557 / 2283`
  - the `reference / claim_sharp_codomain` pair cell lands the same
    `4363 / 557 / 2283`
  - each widens `small_cluster` only to `3156 / 526 / 526 / 0`
  - the isolated `single` pocket stays fenced
  - each raises only its targeted claim-safe mismatch-`1` pairing from
    `42` to `46`
  - each keeps that pairing's incumbent clause-`4`
    `claim_next_bridge / reference` split at `24 / 18`
  - each adds only a tiny clause-`4` `demo_sharp_bridge` side pocket of `4`
    while leaving the sibling claim-safe pairing frozen at `42`
  - consequence: clause-`4` side identity is now exhausted below the
    representative claim-safe exact pair scope too, because the
    `demo_sharp_bridge` sibling only reproduces the same smaller
    negative-control shell already seen on `demo_sharp_codomain`
- The representative claim-safe mismatch-`1` clause-`4`
  `demo_sharp_codomain` side's chosen exact pair cell is now also localized
  one layer deeper across clause-`2` identity:
  - under `reference / claim_next_codomain`, `claim_flat_domain` and
    `claim_sharp_codomain` each land the same smaller negative-control shell at
    `4347 / 555 / 2277`
  - each raises only its own clause-`2` share from `15` to `17`, while the
    sibling claim sheet stays at `15` and the reference sheet stays at `12`
  - each preserves the incumbent clause-`4`
    `claim_next_bridge / reference` split at `24 / 18`
  - each adds only a tiny clause-`4` `demo_sharp_codomain` pocket of `2`
  - the sibling `reference` clause-`2` sheet is a neutral search control on the
    untouched `4331 / 553 / 2271` baseline
  - the claim-side sheets lift `small_cluster` generated candidates only to
    `3144`, while the reference sheet stays at `3132`
  - consequence: clause-`2` sheet identity is now exhausted below that
    representative claim-safe exact pair cell too
- The search/connectivity split on that representative claim-safe exact pair is
  now explicit:
  - a hand-built reference-sheet telescope still passes connectivity under the
    exact override
  - live search stays neutral on that reference sheet because only claim
    clause-`2` prefixes expose the anchor-`11` exact-argument pocket on the
    current lane
- The representative claim-safe mismatch-`1` clause-`4`
  `demo_sharp_codomain` side's two claim-side clause-`2` sheets are now also
  localized one layer deeper across their clause-`5` / clause-`6` shells:
  - on both `claim_flat_domain` and `claim_sharp_codomain`, the
    clause-`5` `claim_flat_codomain` and `claim_next_codomain` prefixes each
    combine with all three clause-`6` labels
    `claim_next_codomain / claim_sharp_codomain / reference`
  - each of those six resulting prefixes is captured exactly once on the same
    `4347 / 555` outer shell
  - every one of the six prefixes sits at matched-clause count `2`
  - every terminal continuation on every prefix is `NeedsFallback`
    (`3` terminal candidates each), so none exposes a live historical-reanchor
    continuation
  - consequence: clause-`5` and clause-`6` identity are now exhausted below the
    two representative claim-side sheets too
- The representative claim-safe mismatch-`1`
  `claim_flat_domain` dead shell is now also localized one layer deeper at
  terminal-family scope:
  - beneath
    `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`,
    the six dead clause-`5` / clause-`6` continuations
    `claim_flat_codomain / claim_next_codomain` x
    `claim_next_codomain / claim_sharp_codomain / reference`
    all reland the same dead completion profile
  - every one of those six dead prefixes still exposes exactly `3` terminal
    candidates:
    `reference / eventual_lift / next_lift`
  - every one of those three terminal families still classifies only as
    `NeedsFallback`
  - every one of those six completed telescopes remains structurally connected
    but unqualified on the current claim path with
    `connected = true`, `references_active_window = false`,
    `self_contained = false`, `max_lib_ref = 11`, and
    `historical_reanchor = false`
  - every one of those six dead prefixes stays `0`-admitted with no bound, no
    best-rank profile, and no survivor sketch
  - consequence: terminal-family identity is now exhausted below that
    representative claim-safe dead shell too
- The promoted reason-level pass below that representative claim-safe dead
  shell's `reference` terminal is now also localized:
  - under the selected
    `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`
    exact-pair override, the exact claim-safe reanchor progress reaches `5`
    matched clauses and first fails at clause `5` on the completed
    `reference` terminal
  - the same `5`-matched / first-mismatch-`5` blocker also holds on the
    sibling `eventual_lift` and `next_lift` terminals
  - the same blocker also holds across all six
    `claim_flat_codomain / claim_next_codomain` x
    `claim_next_codomain / claim_sharp_codomain / reference` dead prefixes in
    that shell
  - the hand-built exact claim-safe pair control still reaches the full
    `8`-clause match with no mismatch
  - consequence: the representative claim-safe dead shell is no longer open at
    clause-`6` or terminal-family identity either; the whole completed shell is
    now uniformly blocked one layer earlier by the clause-`5` structural
    reason wall below the exact claim-safe pair
- That representative claim-safe clause-`5` qualification wall is now also
  partitioned one layer deeper on the promoted `reference` terminal:
  - under the same selected
    `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`
    exact-pair override, clause-`5` `reference` is the only label that keeps
    the full exact claim-safe pair match at `8` clauses with no mismatch
  - the four off-reference clause-`5` labels
    `claim_flat_codomain / claim_next_codomain / demo_sharp_domain /
    demo_flat_codomain` all stop at the same `5`-matched /
    first-mismatch-`5` blocker
  - once clause `1` has moved to `claim_next_codomain`, even the demo-side
    clause-`5` `demo_sharp_domain / demo_flat_codomain` labels no longer
    satisfy the clause-`5` side-pocket qualifier
  - the live six-prefix dead shell still uses only the two claim-side dead
    clause-`5` labels `claim_flat_codomain / claim_next_codomain`, one copy
    per clause-`6` sibling
  - consequence: the representative claim-safe clause-`5` wall is now split
    into one exact `reference` control plus four dead off-reference controls,
    so the next claim-safe revisit has to stay below off-reference
    clause-`5` identity rather than reopening demo-side clause-`5` controls,
    clause-`6`, or terminal-family partitions
- Because the earlier reason-level pass already reproduced the same
  `5`-matched / first-mismatch-`5` blocker across all six dead prefixes in
  that shell, the remaining claim-safe branch is now reduced to a short
  fail-fast checkpoint:
  - the next honest revisit is no longer another label-identity comparison
  - it is a finer qualification-reason partition below one live claim-side
    dead clause-`5` label
  - if that first finer reason split simply relands the same blocker while
    preserving the same fences, the branch should be demoted rather than
    spending another turn below the same claim-safe shell
- That first finer reason-level checkpoint is now also exhausted on the live
  claim-safe shell:
  - beneath the representative
    `reference / claim_next_codomain / claim_flat_domain /
    demo_sharp_codomain` dead shell, both live claim-side clause-`5` labels
    `claim_flat_codomain / claim_next_codomain`, all three clause-`6`
    siblings, and all three terminal families
    `reference / eventual_lift / next_lift` reland the same finer reason
    vector
  - every one of those `18` completed telescopes keeps exact-pair progress at
    `5` matched clauses with first mismatch at clause `5`
  - every one of them still fails both the exact claim-safe pair match and the
    clause-`5` side-pocket qualifier
  - every one of them stays structurally connected but outside
    active-window qualification, outside self-containedness, and outside
    historical reanchor, with `max_lib_ref = 11`
  - every one of them therefore still fails live connectivity on the current
    claim path
  - consequence: the claim-safe mismatch-`1` branch is now exhausted at its
    first finer reason-level checkpoint too and should be demoted rather than
    reopened below the same clause-`5` wall
- The first finer mismatch-`0` reason-level split below the representative
  claim-flat dead shell is now also exhausted:
  - beneath
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain /
    claim_next_bridge / claim_flat_codomain`,
    both clause-`3` branches `claim_flat_argument / claim_eventual_argument`,
    all three clause-`6` children
    `claim_next_codomain / claim_sharp_codomain / reference`, and all three
    terminal families `reference / eventual_lift / next_lift` reland the same
    finer reason vector
  - every one of those `18` completed telescopes falls off historical
    reanchor after exactly `2` matched clauses with first mismatch at clause
    `2`
  - every one of them stays structurally connected but outside active-window
    qualification, outside self-containedness, and outside historical
    reanchor, with `max_lib_ref = 10`
  - every one of them therefore still fails live connectivity on the current
    claim path
  - consequence: the promoted mismatch-`0` reason-level backup beneath that
    representative claim-flat shell is now exhausted too and should be
    demoted rather than reopened below the same dead child
- The representative mismatch-`0` claim-sharp dead-child fail-fast checkpoint
  is now also exhausted:
  - beneath
    `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain /
    claim_next_bridge / claim_flat_codomain`,
    both clause-`3` branches `claim_flat_argument / claim_eventual_argument`,
    all three clause-`6` children
    `claim_next_codomain / claim_sharp_codomain / reference`, and all three
    terminal families `reference / eventual_lift / next_lift` reland the same
    dead shell
  - every one of those six child continuations keeps the same
    `3`-generated / `0`-admitted completion summary with no bound, no
    best-rank profile, no survivor sketch, and the same
    `reference / eventual_lift / next_lift` terminal trio
  - every one of those `18` completed telescopes also falls off historical
    reanchor after exactly `2` matched clauses with first mismatch at clause
    `2`
  - every one of them stays structurally connected but outside active-window
    qualification, outside self-containedness, and outside historical
    reanchor, with `max_lib_ref = 10`
  - every one of them therefore still fails live connectivity on the current
    claim path
  - consequence: the tighter representative mismatch-`0` claim-side broader
    backup is now exhausted below both representative claim-side sheets, not
    just beneath the earlier claim-flat shell
- The cross-sheet claim-side union below the representative mismatch-`0`
  `claim_next_bridge / claim_flat_codomain` cell's marginally best
  clause-`6` `reference` continuation is now also exhausted as a
  reconstructive control:
  - reopening both representative claim-side clause-`2` sheets together,
    across both clause-`3` branches, lands `4355 / 551 / 2265`
  - it widens `small_cluster` to `3150 / 522 / 522 / 0`
  - first-mismatch-`0` contracts from `312` to `310`
  - the representative clause-`2` spread becomes `14 / 14 / 12`
  - the representative clause-`4` split contracts from `24 / 18` to
    `22 / 18`
  - the active clause-`4` `claim_next_bridge` plus clause-`5`
    `claim_flat_codomain` bucket contracts from `48` to `46`
  - consequence: that deeper claim-side clause-`6` `reference` union merely
    reconstructs the existing `4355 / 551 / 2265` pair-cell shell instead of
    isolating a safer repair
- That reconstructive claim-side union is now also pinned to the same
  zero-admitted remaining-one dead surface as the representative pair-cell
  tradeoff:
  - the promoted exact-prune family summary stays
    `captured_prefixes = 2265`, `cached_bound_count = 0`, and
    family counts `((0, None, None), 2265)`
  - the promoted terminal-connectivity summary stays
    `generated = 6795`, `NeedsFallback = 6795`,
    `KeepWithoutFallback = 0`, and
    `structurally_connected_but_unqualified = 6795`
  - consequence: there is still no hidden bound-carrying or live
    remaining-one child below that representative mismatch-`0`
    claim-side clause-`6` `reference` shell either
- That same reconstructive union's `small_cluster` widening is now also fully
  localized:
  - relative to the canonical `3132` shell, the widened `3150` shell is
    exactly `+18` generated candidates from six released remaining-one groups
  - those groups are the two claim-side clause-`2` sheets
    `claim_flat_domain / claim_sharp_codomain` crossed with the three
    clause-`6` labels `claim_next_codomain / claim_sharp_codomain / reference`
  - every released group stays in the same
    `k8:structural_generic:temporal_operator:library_backed:small_cluster`
    bucket with `3` generated, `0` admitted, no bound, no bar-clear, and only
    `NeedsFallback` connectivity
  - consequence: the union's `+18` widening carries no hidden qualifying or
    bound-carrying pocket; the next repair has to target the parent-level
    route or qualification that releases those six groups rather than another
    deeper remaining-one reland
- The residual `3` `single`-bucket incumbent prunes remain fenced and are now
  secondary to the partial-prefix wall.

## Current Operating Position

- Keep wording at `bounded live recovery`.
- Treat canonical `v13` as the stored baseline on newer code; do not spend
  another turn on the rerun-vs-step-`1` ordering pass.
- Keep step `1` explicit, but do not reopen it first; `v13` reconfirmed the
  same step-`1` and step-`15` misses on newer code, so another step-`15`
  repair stays ahead of step `1` unless a later stored bundle changes the
  diagnosis.
- When working outside the exhausted mismatch-`1`
  `reference + demo_flat_codomain` ladder, treat both the smaller claim-safe
  mismatch-`1` tier and the promoted representative mismatch-`0`
  claim-side dead-child reason backup as exhausted. The residual
  `reference / reference` tail is now exhausted too, so there is no fresh
  off-branch lead left inside those three surfaces; the claim-safe
  clause-`5` wall, both representative mismatch-`0` dead-child shells, and
  the full `reference / reference` tail are all frozen controls rather than
  fresh leads.
- Do not stop at the representative claim-safe mismatch-`1` clause-`4`
  `demo_sharp_codomain` or `demo_sharp_bridge` side's exact pair-cell
  relands either; each only lifts breadth to `4363` while widening the clean
  wall to `557` and `small_cluster` to `3156`.
- Do not stop at the representative `reference / claim_next_codomain`
  pair cell's clause-`2` split on the clause-`4` `demo_sharp_codomain` side
  either; the `claim_flat_domain` and `claim_sharp_codomain` sheets are
  matched smaller controls at `4347 / 555 / 2277`, while the sibling
  `reference` sheet is a neutral search control on `4331 / 553 / 2271`.
- Because only claim clause-`2` prefixes expose the anchor-`11`
  exact-argument pocket there, the reference clause-`2` sheet is not a fresh
  search lead even though a hand-built telescope can pass connectivity under
  the same override.
- Do not stop at the representative claim-side clause-`5` / clause-`6` shell
  under that `reference / claim_next_codomain / demo_sharp_codomain` cell
  either; both claim-side sheets now reland the same six dead prefixes, and
  every terminal continuation there still needs fallback.
- Do not stop at the representative claim-flat dead prefix's terminal-family
  split under
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`
  either; all six clause-`5` / clause-`6` continuations there now reland the
  same `3`-generated / `0`-admitted profile with the same
  `reference / eventual_lift / next_lift` trio, and every one of those
  completed telescopes is still only structurally connected but outside
  historical reanchor and active-window qualification.
- Do not reopen the representative claim-safe dead prefix through the
  clause-`5` `reference`, `demo_sharp_domain`, or `demo_flat_codomain`
  controls either; under the selected exact pair, `reference` is only the
  exact control and the two demo-side labels are dead off-reference controls
  rather than fresh live search leads.
- The claim-safe mismatch-`1` fail-fast checkpoint is now spent:
  - the first finer reason-level probe below the live claim-side dead
    clause-`5` labels stayed fully uniform across both live clause-`5`
    labels, all three clause-`6` siblings, and all three terminal families
  - do not spend another turn below that same representative claim-safe
    clause-`5` wall unless a later broader-backup comparison changes the
    priority order again
- The representative mismatch-`0` claim-flat dead-child fail-fast checkpoint
  is now spent too:
  - the first finer reason-level probe below that dead shell stayed fully
    uniform across both clause-`3` branches, all three clause-`6` children,
    and all three terminal families
  - do not spend another turn below that same representative mismatch-`0`
    claim-flat dead shell unless a later broader-backup comparison changes
    the priority order again
- The representative mismatch-`0` claim-sharp dead-child fail-fast checkpoint
  is now spent too:
  - the deeper completion pass and the first finer reason-level probe below
    that dead shell both stayed fully uniform across both clause-`3`
    branches, all three clause-`6` children, and all three terminal families
  - do not spend another turn below that same representative mismatch-`0`
    claim-sharp dead shell unless a later broader-backup comparison changes
    the priority order again
- Do not reopen the isolated clause-`1` `demo_flat_codomain` exact-suffix
  side pocket first either; its narrow exact-summary relief only lands
  `4349 / 556 / 2268` with `small_cluster = 3141 / 522 / 522 / 0` while the
  isolated `single` pocket stays fenced, so it is a looser side-pocket
  control rather than a promotable repair.
- If the mismatch-`0` claim-domain surface is revisited, stay below the broad
  clause-`1` `demo_flat_codomain` reopening and below whole-tier parent
  exact-bound relief; the former reaches `4985` only by widening the clean
  wall to `667`, and the latter drops the wall to `241` only by exploding the
  noncanonical `small_cluster`.
- With the claim-safe fail-fast checkpoint, both representative
  mismatch-`0` claim-side dead-child checkpoints, and the full
  `reference / reference` tail now all spent, there is no fresh off-branch
  local lead left inside the currently promoted step-`15` backups.
- That `reference / reference` revisit is now already localized three layers
  deeper:
  - mismatch-`2 = 42` was the earlier larger lead, but its clause-`4`
    `claim_next_bridge = 18`, `reference = 16`,
    `demo_sharp_codomain = 4`, and `demo_sharp_bridge = 4` anatomy is now
    pinned as tradeoff-only control
  - mismatch-`3 = 12` was the smaller backup
  - mismatch-`3` `claim_next_bridge` now lands `4403 / 547 / 2271` with
    `small_cluster = 3186 / 522 / 522 / 0`
  - mismatch-`3` `reference` now lands `4481 / 545 / 2271` with
    `small_cluster = 3240 / 522 / 522 / 0`
- Do not reopen broader mismatch-`0` or claim-safe shells by re-stating the
  same `reference / reference` tail first; the full tail is now frozen
  context rather than the next live lead.
- Do not stop at the mismatch-`2` clause-`4` `claim_next_bridge` half either;
  it narrows the wall only to `535` while widening `small_cluster` to `3294`.
- Do not stop at the mismatch-`2` clause-`4` `reference` half either; it
  narrows the wall further to `529`, but only by widening `small_cluster` to
  `3492`.
- Do not stop at the mismatch-`2` clause-`4` `demo_sharp_codomain` pocket
  either; it narrows the wall only to `549`, contracts the exact mismatch-`2`
  pair only to `38`, and still widens `small_cluster` to `3168`.
- Do not stop at the mismatch-`2` clause-`4` `demo_sharp_bridge` pocket
  either; it lands the same `4379 / 549 / 2271` smaller tradeoff shell and
  the same widened `3168 / 522 / 522 / 0` `small_cluster`.
- Do not stop at the mismatch-`3` clause-`4` `claim_next_bridge` half either;
  it narrows the wall only to `547` while still widening `small_cluster` to
  `3186`.
- Do not stop at the mismatch-`3` clause-`4` `reference` half either; it
  narrows the wall further to `545`, but only by widening `small_cluster` to
  `3240`.
- Because both mismatch-`3` clause-`4` halves are now also exhausted as
  tradeoff controls, the full `reference / reference` tail is frozen context
  rather than the active lead. The next honest revisit should compare
  alternate broader backups rather than reopening the same tail again.
- That broader-backup comparison is now already settled in favor of the
  tighter representative mismatch-`0` claim-side clause-`2` shell:
  - it stays at `4343 / 552 / 2268` with
    `small_cluster = 3141 / 522 / 522 / 0`
  - the secondary representative claim-safe claim-side shell lands the looser
    `4347 / 555 / 2277` with `small_cluster = 3144 / 522 / 522 / 0`
  - do not promote the claim-safe shell ahead of that tighter mismatch-`0`
    backup; its tiny `+4` breadth lift is bought by worse wall, worse
    `small_cluster`, a worse zero-admitted tail, and extra first-mismatch-`1`
    pressure
- The representative mismatch-`0` claim-flat dead shell is now frozen context
  rather than an active lead: its first finer reason-level split already
  relands the same clause-`2` blocker and the same nonqualifying connectivity
  vector across all `18` completed telescopes.
- The representative mismatch-`0` claim-sharp dead shell is now frozen
  context rather than an active lead too: its deeper completion pass and its
  first finer reason-level split already reland the same dead
  `3`-generated / `0`-admitted shell and the same clause-`2` blocker plus
  nonqualifying connectivity vector across all `18` completed telescopes.
- The whole mismatch-`0` clause-`4` `claim_next_bridge`-half remaining-one
  exact-summary relief is now the leading local tradeoff on that surface, but
  it is not the landed repair because it still widens `small_cluster` to
  `3348`; if mismatch-`0` is revisited again, localize one layer deeper into
  the active clause-`5` `claim_flat_codomain / reference` cells under that
  same half first rather than the inert `claim_next_codomain` sibling.
- Do not reopen the mismatch-`0` clause-`4` `claim_next_bridge` half through
  the same connectivity-only relocalization first; it is now a smaller
  negative control rather than the landed repair.
- Do not reopen the mismatch-`0` clause-`4` `reference` half through the same
  connectivity-only relocalization first; it is now an even sharper negative
  control rather than the landed repair.
- Do not reopen remaining-one exact-summary relief on the mismatch-`0`
  clause-`4` `reference` plus clause-`5` `reference` tail first; it lifts
  breadth only to `4547` by widening the clean wall to `589` and
  `small_cluster` to `3240`.
- Do not stop at the whole mismatch-`0` clause-`4` `claim_next_bridge`-half
  remaining-one exact-summary relief either; it narrows the wall to `529`,
  but only by widening `small_cluster` to `3348`.
- Do not reopen the mismatch-`0` clause-`4` `claim_next_bridge` plus
  clause-`5` `claim_next_codomain` exact-summary cell first; it is now a
  neutral control on the untouched `4331 / 553 / 2271` baseline.
- Do not stop at the smaller mismatch-`0` clause-`4` `claim_next_bridge` plus
  clause-`5` `claim_flat_codomain` or `reference` exact-summary cells either;
  each narrows the wall only to `541` while still widening `small_cluster` to
  `3240`.
- Do not stop at the pair-cell subprobes below those two active clause-`5`
  cells either; each only narrows the wall to `551` while still widening
  `small_cluster` to `3150`.
- Because all `12` pair-cell relands were symmetric at that depth, the current
  deep probe already moved below one representative active pair cell; the next
  revisit should stay below that representative cell rather than rechecking the
  other live pairings or swapping between the `claim_flat_codomain` and
  `reference` families again.
- Do not stop at the representative pair-cell clause-`2`
  `claim_flat_domain` or `claim_sharp_codomain` sheet subprobes either; each
  only narrows the wall to `552` while still widening `small_cluster` to
  `3141`.
- Do not reopen the representative pair-cell clause-`2` `reference` sheet
  first; it is now a neutral control on the untouched `4331 / 553 / 2271`
  baseline.
- Because the two representative claim-side clause-`2` sheets are symmetric
  smaller tradeoff controls and the reference sheet is inert, both
  representative claim-side sheets are now frozen context rather than fresh
  mismatch-`0` leads.
- Do not stop at the sibling active clause-`5` `reference` family's
  representative pair-cell clause-`2` split either; its
  `claim_flat_domain` and `claim_sharp_codomain` sheets reland the same
  `4343 / 552 / 2268` smaller tradeoff shell with
  `small_cluster = 3141 / 522 / 522 / 0`, while the sibling `reference`
  sheet is a neutral control on `4331 / 553 / 2271`.
- Because that sibling active clause-`5` `reference` family repeats the same
  representative claim-side clause-`2` tradeoff one layer deeper too,
  clause-`5` family identity is now exhausted below representative pair-cell
  scope as well.
- Do not stop at the representative `claim_flat_domain` clause-`2` sheet's
  clause-`6` `claim_next_codomain`, `claim_sharp_codomain`, or `reference`
  continuations either; all three reland the same `4343 / 552 / 3141`
  smaller tradeoff shell and differ only by a tiny zero-admitted tail delta.
- Do not reopen that marginally best clause-`6` `reference` continuation
  through either individual clause-`3` `claim_flat_argument` or
  `claim_eventual_argument` branch first; each is now a neutral control on
  the untouched `4331 / 553 / 2271` baseline.
- Because those two individual clause-`3` branches are neutral while their
  union is the only clause-`6` `reference` tradeoff, the representative
  claim-flat clause-`6` `reference` shell is now historical frozen context
  rather than the active next slice.
- Do not reassemble both representative claim-side clause-`2` sheets beneath
  that marginally best clause-`6` `reference` continuation either; the
  cross-sheet union now just relands the existing `4355 / 551 / 2265`
  pair-cell smaller tradeoff with `small_cluster = 3150 / 522 / 522 / 0`.
- Because that deeper claim-side union still keeps the exact-prune family on
  `((0, None, None), 2265)` with `cached_bound_count = 0` and every one of its
  `6795` terminal continuations still only `NeedsFallback`, any remaining
  step-`15` leverage now has to move above the current remaining-one
  exact-summary lattice rather than further inside it.
- Do not stop at that representative claim-flat joint clause-`3`
  parent/child shell either; its six child continuations are now all the same
  dead `3`-generated / `0`-admitted completion summary with no bound and no
  survivor sketch.
- Because those claim-flat child continuations only expose the same nonlive
  `reference / eventual_lift / next_lift` open-band structural trio, the next
  honest revisit should now stay off mismatch-`0` claim-flat identity or
  reason relands and move to the broader-backup comparison instead.
- Do not stop at the representative `claim_sharp_codomain` clause-`2` sheet's
  clause-`6` `claim_next_codomain`, `claim_sharp_codomain`, or `reference`
  continuations either; all three reland the same `4343 / 552 / 3141`
  smaller tradeoff shell and differ only by a tiny zero-admitted tail delta.
- Because the representative claim-sharp sheet is now exhausted through both
  clause-`6` identity and its deeper dead-child fail-fast checkpoint, the
  next honest slice no longer sits on another mismatch-`0` claim-side reland.
  With the broader-backup comparison now settled, the rerun-backed reset
  already complete, and both representative claim-side shells frozen, the next
  honest follow-on is now the parent-level route or qualification above the
  current remaining-one lattice that releases the six zero-admitted
  `small_cluster` groups under the representative claim-side clause-`6`
  `reference` union, while step `1` stays deferred.
- The first parent-level route probe above that remaining-one lattice is now
  also spent as an unsafe negative control:
  - a scoped representative mismatch-`0` claim-side historical-reanchor route
    does qualify the targeted claim-side clause-`2` / clause-`3` /
    clause-`6` parent shell
  - it keeps the sibling reference clause-`2` sheet closed
  - it also keeps lifted terminals fenced
  - but the full step-`15` search surface lands noncanonical `60 / 8` with
    `retained = 2`, `generated = 4427`, `partial_prefix_bar_failure = 545`,
    `incumbent_dominance = 117`, and zero-admitted captures `2247`
  - the first-mismatch-`0` tier contracts from `312` to `304`
  - `small_cluster` contracts to `2931 / 455 / 455 / 115`
  - the isolated `single` bucket reopens to `2` fully scored non-winners plus
    `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread contracts to `11 / 11 / 12`
  - the representative clause-`4` split contracts to `20 / 14`
  - the active clause-`4` `claim_next_bridge` plus clause-`5`
    `claim_flat_codomain` bucket contracts to `44`
- Consequence: that first parent-level claim-side route is not the missing
  clean repair. It narrows the wall only by displacing the canonical
  `103 / 8` winner and by reshaping both survivor buckets, so the next honest
  follow-on still has to stay above another reland of that same
  representative mismatch-`0` claim-side parent route while step `1` remains
  deferred.
- The sibling active clause-`5` `reference` family's representative
  mismatch-`0` claim-side parent-route probe is now also spent as the same
  unsafe negative control:
  - it relands the same noncanonical `60 / 8` winner, the same
    `4427 / 545 / 2247` late surface, the same `incumbent_dominance = 117`,
    and the same reopened `single` bucket
  - it relands the same contracted first-mismatch-`0` tier at `304`
  - it relands the same `small_cluster = 2931 / 455 / 455 / 115`
  - it relands the same representative clause-`2` spread `11 / 11 / 12`
  - it relands the same representative clause-`4` split `20 / 14`
  - it contracts only the sibling active clause-`4` `claim_next_bridge` plus
    clause-`5` `reference` bucket to `44`
- Consequence: representative mismatch-`0` claim-side parent-route identity
  is now exhausted across both active clause-`5` families. The next honest
  follow-on has to move to a different parent-level qualification above the
  current remaining-one lattice rather than reopening either claim-side route
  sibling, while step `1` remains deferred.
- That matched unsafe parent-route class is now also localized at delta level:
  - on the chosen active clause-`5` bucket, each route removes exactly four
    remaining-two exact-prune parent cells:
    `claim_flat_domain / claim_sharp_codomain` crossed with clause-`4`
    `claim_next_bridge / reference`
  - those four parent cells contribute exactly `8` removed zero-admitted
    captures and exactly `24` removed remaining-one pruned prefixes
  - neither route introduces any off-target exact-prune or pruned-prefix
    family
- Consequence: the unsafe parent-route class is exhausted not just by the
  landed `4427 / 545 / 2247` surface but by localization too. The next honest
  follow-on has to change the parent-level qualification family itself rather
  than retargeting the same active clause-`5` bucket or re-running another
  route-identity reland, while step `1` remains deferred.
- The first alternate parent-level qualification family above that same
  remaining-one lattice is now also spent across both active clause-`5`
  families:
  - a scoped representative mismatch-`0` claim-side active-window override
    does qualify the targeted claim-side parent shell while keeping the
    sibling reference clause-`2` sheet closed and lifted terminals fenced
  - on either active clause-`5` family, the full step-`15` search surface
    still relands an unsafe negative control with noncanonical `60 / 8`,
    `retained = 2`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`,
    `incumbent_dominance = 110`, zero-admitted captures `2247`, first-mismatch
    distribution `304 / 177 / 50 / 14`,
    `small_cluster = 2952 / 558 / 558 / 108`, and the same reopened `single`
    bucket at best overshoot `545 / 5278`
  - the representative clause-`2` spread stays `11 / 11 / 12`
  - the representative clause-`4` split stays `20 / 14`
  - it contracts only the chosen active clause-`4`
    `claim_next_bridge` plus clause-`5` bucket to `44`
- That alternate active-window family is also localized at the same targeted
  delta shape:
  - on the chosen active clause-`5` bucket, it removes the same four
    remaining-two exact-prune parent cells:
    `claim_flat_domain / claim_sharp_codomain` crossed with clause-`4`
    `claim_next_bridge / reference`
  - it removes the same `8` zero-admitted captures and the same `24`
    remaining-one pruned prefixes
  - it introduces no off-target exact-prune or pruned-prefix family
- Consequence: active-window qualification is not the missing clean repair
  either. It keeps the same targeted four-cell plus `24`-pruned-prefix delta
  as the spent historical-reanchor class, but it still displaces the
  canonical `103 / 8` winner and reopens the `single` bucket, so the next
  honest follow-on has to change parent-level qualification family again
  rather than swapping between the historical-reanchor and active-window
  controls.
- The next alternate parent-level self-contained qualification family above
  that same remaining-one lattice is now also spent across both active
  clause-`5` families:
  - a scoped representative mismatch-`0` claim-side self-contained override
    does qualify the targeted claim-side parent shell while keeping the
    sibling reference clause-`2` sheet closed and lifted terminals fenced
  - on either active clause-`5` family, the full step-`15` search surface
    relands the same unsafe matched control as active-window with
    noncanonical `60 / 8`, `retained = 2`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 110`,
    zero-admitted captures `2247`, first-mismatch distribution
    `304 / 177 / 50 / 14`, `small_cluster = 2952 / 558 / 558 / 108`, and
    the same reopened `single` bucket at best overshoot `545 / 5278`
  - the representative clause-`2` spread stays `11 / 11 / 12`
  - the representative clause-`4` split stays `20 / 14`
  - it contracts only the chosen active clause-`4`
    `claim_next_bridge` plus clause-`5` bucket to `44`
- That alternate self-contained family is also localized at the same targeted
  delta level:
  - on the chosen active clause-`5` bucket, it removes the same four
    remaining-two exact-prune parent cells
  - it removes the same `8` zero-admitted captures and the same `24`
    remaining-one pruned prefixes
  - it introduces no off-target exact-prune or pruned-prefix family
- Consequence: self-contained qualification is not the missing clean repair
  either. It relands the same unsafe shell and the same targeted four-cell
  plus `24`-pruned-prefix delta as the active-window family, so the next
  honest follow-on has to change parent-level qualification family again
  rather than swapping between the historical-reanchor, active-window, and
  self-contained controls.
- The narrower clause-`6` `reference` refinements of those same alternate
  active-window and self-contained families are now also spent:
  - on `claim_flat_codomain`, both refinements reland noncanonical `60 / 8`
    with `retained = 4`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 113`, and
    zero-admitted captures `2247`
  - on `reference`, both sibling refinements reland noncanonical `60 / 8`
    with `retained = 2`, the same `4427 / 545 / 2247`, and
    `incumbent_dominance = 115`
  - both clause-`5` siblings keep first-mismatch distribution
    `304 / 177 / 50 / 14`, clause-`2` spread `11 / 11 / 12`, clause-`4`
    split `20 / 14`, and the chosen active clause-`4`
    `claim_next_bridge` plus clause-`5` bucket at `44`
  - both reland `small_cluster = 2904 / 462 / 462 / 109 / 2` with best
    overshoot `545 / 5278` and no `single` bucket
  - both also keep the same targeted four remaining-two parent cells plus the
    same `24` remaining-one pruned prefixes, with no off-target capture or
    prune family introduced
- Consequence: clause-`6` narrowing does not rescue the alternate
  active-window or self-contained families either. It tightens the broader
  `2952 / 558 / 558 / 108` shell into a narrower unsafe split control, but it
  still displaces the canonical `103 / 8` winner and therefore stays below
  the active frontier rather than becoming the landed repair.
- Narrowing those same alternate active-window and self-contained families one
  layer earlier to only one representative claim-flat clause-`3` argument
  branch on the active clause-`5` `claim_flat_codomain` bucket is now also
  spent:
  - on either `claim_flat_argument` or `claim_eventual_argument`, both
    alternate families reland noncanonical `60 / 8` with `retained = 2`,
    `generated = 4379`, `partial_prefix_bar_failure = 549`,
    `incumbent_dominance = 110`, and zero-admitted captures `2259`
  - both reland the same first-mismatch distribution
    `308 / 177 / 50 / 14`
  - both reland the same
    `small_cluster = 2880 / 486 / 486 / 108`
  - both reopen the same isolated `single` bucket to `2` fully scored
    non-winners plus `2` residual prunes at best overshoot `545 / 5278`
  - on the chosen active clause-`5` bucket, both alternate clause-`3`
    refinements remove the same four targeted remaining-two exact-prune
    parent cells, but now only `1` zero-admitted capture per cell plus `3`
    corresponding remaining-one pruned prefixes per cell, for a narrowed
    targeted delta of `4` removed captures plus `12` removed pruned prefixes
    and no off-target family introduced
- Consequence: clause-`3` narrowing does not rescue the alternate
  active-window or self-contained families either. It relands a matched
  smaller unsafe split control that still displaces the canonical `103 / 8`
  winner and reopens the `single` bucket, so the next honest follow-on has to
  stay above those alternate clause-`3` refinements as well as above their
  broader and clause-`6` siblings.
- Narrowing that same parent-route class to only the representative
  clause-`6` `reference` continuation is now also spent across both active
  clause-`5` families:
  - on either active clause-`5` family, the narrowed route relands a
    different unsafe negative control with noncanonical `74 / 8`,
    `retained = 1`, `generated = 4427`, `partial_prefix_bar_failure = 545`,
    `incumbent_dominance = 111`, and zero-admitted captures `2247`
  - the first-mismatch distribution relands as clause `0 = 304`,
    clause `1 = 177`, clause `2 = 50`, and clause `3 = 14`
  - `small_cluster` contracts further to `2904 / 430 / 430 / 108`
  - the isolated `single` bucket stays fenced at `3` residual prunes plus `1`
    fully scored non-winner at best overshoot `19563 / 10556`
  - the representative clause-`2` spread stays contracted at `11 / 11 / 12`
  - the representative clause-`4` split stays contracted at `20 / 14`
  - it contracts only the chosen active clause-`4`
    `claim_next_bridge` plus clause-`5` bucket to `44`
- Consequence: the marginally narrower `reference` clause-`6` parent-route
  refinement is not the missing clean repair either. It preserves the tighter
  targeted delta and keeps the `single` pocket fenced, but it still displaces
  the canonical `103 / 8` winner and remains an unsafe negative control
  rather than a landed repair.
- That narrower parent-route refinement is also localized to the same
  targeted delta shape:
  - on the chosen active clause-`5` bucket, it removes the same four
    remaining-two exact-prune parent cells
  - it removes the same `8` zero-admitted captures and the same `24`
    remaining-one pruned prefixes
  - it introduces no off-target exact-prune or pruned-prefix family
- Consequence: the next honest follow-on has to stay above not just the broad
  representative claim-side parent-route class, but also the narrower
  `reference` clause-`6` refinement on both active clause-`5` families.
- Narrowing that same representative mismatch-`0` claim-side parent-route
  class on the active clause-`5` `claim_flat_codomain` family to only one
  clause-`3` argument branch is now also spent as a smaller unsafe negative
  control:
  - on either `claim_flat_argument` or `claim_eventual_argument`, the
    narrowed route relands noncanonical `60 / 8` with `retained = 2`,
    `generated = 4379`, `partial_prefix_bar_failure = 549`,
    `incumbent_dominance = 113`, zero-admitted captures `2259`, and
    `small_cluster = 2871 / 435 / 435 / 111`
  - the first-mismatch distribution relands as clause `0 = 308`,
    clause `1 = 177`, clause `2 = 50`, and clause `3 = 14`
  - the isolated `single` bucket reopens to `2` fully scored non-winners plus
    `2` residual prunes at best overshoot `545 / 5278`
  - consequence: representative claim-flat clause-`3` identity is now
    exhausted inside that same parent-route family too; even the tighter
    `4379 / 549 / 2259` shell still displaces the canonical `103 / 8` winner
    and reopens the `single` bucket
- The first recombined parent-level qualification family above that same
  remaining-one lattice is now also spent across both active clause-`5`
  families:
  - a scoped representative mismatch-`0` claim-side historical-reanchor plus
    active-window override still qualifies the targeted claim-side parent shell
    on `claim_flat_codomain` and `reference` while keeping lifted terminals
    fenced
  - on either active clause-`5` family, that hybrid relands the exact same
    unsafe shell as the already-spent active-window family:
    noncanonical `60 / 8`, `retained = 2`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 110`,
    zero-admitted captures `2247`, first-mismatch distribution
    `304 / 177 / 50 / 14`, `small_cluster = 2952 / 558 / 558 / 108`, the same
    reopened `single` bucket at best overshoot `545 / 5278`, the same
    representative clause-`2` spread `11 / 11 / 12`, the same
    representative clause-`4` split `20 / 14`, and the same chosen active
    clause-`4` plus clause-`5` bucket at `44`
- Consequence: adding historical reanchor on top of the representative
  mismatch-`0` claim-side active-window family does not create a fresh
  parent-level repair class. The next honest follow-on has to stay above that
  recombined hybrid too rather than recombining already-spent parent-route and
  active-window controls.
- The sibling recombined parent-level qualification family above that same
  remaining-one lattice is now also spent across both active clause-`5`
  families:
  - a scoped representative mismatch-`0` claim-side historical-reanchor plus
    self-contained override still qualifies the targeted claim-side parent
    shell on `claim_flat_codomain` and `reference` while keeping lifted
    terminals fenced
  - on either active clause-`5` family, that hybrid relands the exact same
    unsafe shell as the already-spent self-contained family:
    noncanonical `60 / 8`, `retained = 2`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 110`,
    zero-admitted captures `2247`, first-mismatch distribution
    `304 / 177 / 50 / 14`, `small_cluster = 2952 / 558 / 558 / 108`, the same
    reopened `single` bucket at best overshoot `545 / 5278`, the same
    representative clause-`2` spread `11 / 11 / 12`, the same
    representative clause-`4` split `20 / 14`, and the same chosen active
    clause-`4` plus clause-`5` bucket at `44`
- Consequence: adding historical reanchor on top of the representative
  mismatch-`0` claim-side self-contained family does not create a fresh
  parent-level repair class either. The next honest follow-on has to stay
  above both recombined hybrids and move to a looser recombined parent-level
  qualification family rather than reopening narrower clause-`3` /
  clause-`6` selectors.
- The first looser recombined parent-level qualification family above that
  same remaining-one lattice is now also spent across both active clause-`5`
  families:
  - a scoped representative mismatch-`0` claim-side active-window plus
    self-contained override still qualifies the targeted claim-side parent
    shell on `claim_flat_codomain` and `reference` while keeping lifted
    terminals fenced
  - on either active clause-`5` family, that looser recombination relands the
    exact same unsafe shell as the already-spent active-window and
    self-contained families:
    noncanonical `60 / 8`, `retained = 2`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 110`,
    zero-admitted captures `2247`, first-mismatch distribution
    `304 / 177 / 50 / 14`, `small_cluster = 2952 / 558 / 558 / 108`, the same
    reopened `single` bucket at best overshoot `545 / 5278`, the same
    representative clause-`2` spread `11 / 11 / 12`, the same
    representative clause-`4` split `20 / 14`, and the same chosen active
    clause-`4` plus clause-`5` bucket at `44`
- That looser recombined family is also localized at the same targeted delta
  level:
  - on the chosen active clause-`5` bucket, each probe removes the same four
    remaining-two exact-prune parent cells
  - it removes the same `8` zero-admitted captures and the same `24`
    remaining-one pruned prefixes
  - it introduces no off-target exact-prune or pruned-prefix family
- Consequence: combining active-window and self-contained does not create a
  fresh parent-level repair class either. The next honest follow-on has to
  stay above both recombined historical-reanchor hybrids and above this first
  looser recombined family too, moving to the next looser recombined
  parent-level qualification family rather than reopening narrower
  clause-`3` / clause-`6` selectors.
- The next looser recombined parent-level qualification family above that
  same remaining-one lattice is now also spent across both active clause-`5`
  families:
  - a scoped representative mismatch-`0` claim-side historical-reanchor plus
    active-window plus self-contained override still qualifies the targeted
    claim-side parent shell on `claim_flat_codomain` and `reference` while
    keeping lifted terminals fenced
  - on either active clause-`5` family, that full qualification triad
    relands the exact same unsafe shell as the already-spent active-window
    and self-contained families:
    noncanonical `60 / 8`, `retained = 2`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 110`,
    zero-admitted captures `2247`, first-mismatch distribution
    `304 / 177 / 50 / 14`, `small_cluster = 2952 / 558 / 558 / 108`, the same
    reopened `single` bucket at best overshoot `545 / 5278`, the same
    representative clause-`2` spread `11 / 11 / 12`, the same
    representative clause-`4` split `20 / 14`, and the same chosen active
    clause-`4` plus clause-`5` bucket at `44`
- That full qualification triad is also localized at the same targeted delta
  level:
  - on the chosen active clause-`5` bucket, each probe removes the same four
    remaining-two exact-prune parent cells
  - it removes the same `8` zero-admitted captures and the same `24`
    remaining-one pruned prefixes
  - it introduces no off-target exact-prune or pruned-prefix family
- Consequence: exhausting historical-reanchor, active-window, and
  self-contained together does not create a fresh parent-level repair class
  either. The full parent-level qualification-family lattice above the
  current remaining-one shell is now spent, so the next honest follow-on has
  to move to a different repair class above that lattice rather than trying
  another recombination or reopening narrower clause-`3` / clause-`6`
  selectors from the same exhausted shell.
- The first clause-`4` split above that exhausted parent-level
  qualification-family lattice is now also spent on the representative
  mismatch-`0` claim-side parent-route shell across both active
  clause-`5` families:
  - a scoped clause-`4` `reference` override does qualify the targeted
    claim-side parent shell while keeping the sibling
    `claim_next_bridge` branch, the sibling reference clause-`2` sheet, and
    lifted terminals fenced
  - on either active clause-`5` family, the full step-`15` search surface
    relands a different unsafe matched control with noncanonical `60 / 8`,
    `retained = 2`, `generated = 4391`,
    `partial_prefix_bar_failure = 557`, `incumbent_dominance = 113`,
    zero-admitted captures `2259`, first-mismatch distribution
    `316 / 177 / 50 / 14`, `small_cluster = 2871 / 435 / 435 / 111`, and
    the same reopened `single` bucket at best overshoot `545 / 5278`
  - the representative clause-`2` spread shifts to `19 / 19 / 12`
  - the representative clause-`4` split shifts to `24 / 26`
  - it contracts only the chosen clause-`4` `reference` plus active
    clause-`5` bucket to `36`
- That narrow clause-`4` parent-route split is also localized at delta level:
  - on the chosen active clause-`5` bucket, it removes only the two
    remaining-three clause-`4` `reference` parent captures
  - it removes `4` zero-admitted captures in total and the matching `12`
    remaining-one pruned prefixes
  - it introduces four targeted remaining-two clause-`4` `reference`
    capture families and no introduced pruned-prefix family
- Consequence: even above the exhausted parent-level qualification-family
  lattice, the representative parent-route clause-`4` `reference` split is
  not the missing clean repair either. It only converts remaining-three
  pressure into a narrower remaining-two clause-`4` `reference` pocket while
  still displacing the canonical `103 / 8` winner and reopening the
  `single` bucket, so the next honest follow-on has to move to a different
  code-side repair class again rather than retrying this route split or
  dropping back to narrower clause-`3` / clause-`6` selectors.
- The sibling clause-`4` `reference` splits inside the alternate
  active-window and self-contained qualification families are now also spent
  across both active clause-`5` families:
  - a scoped clause-`4` `reference` override on either alternate family does
    qualify the targeted claim-side parent shell while keeping the sibling
    `claim_next_bridge` branch, the sibling reference clause-`2` sheet, and
    lifted terminals fenced
  - on either active clause-`5` family, both alternate clause-`4` splits
    reland the same smaller unsafe control with noncanonical `60 / 8`,
    `retained = 2`, `generated = 4391`,
    `partial_prefix_bar_failure = 557`, `incumbent_dominance = 110`,
    zero-admitted captures `2271`, first-mismatch distribution
    `312 / 177 / 50 / 14`, `small_cluster = 2880 / 486 / 486 / 108`, and
    the same reopened `single` bucket at best overshoot `545 / 5278`
  - the representative clause-`2` spread stays `15 / 15 / 12`
  - the representative clause-`4` split stays `24 / 18`
  - each contracts only the chosen clause-`4` `reference` plus active
    clause-`5` bucket to `36`
- Those alternate clause-`4` splits are also localized at delta level:
  - on the chosen active clause-`5` bucket, each probe removes only the two
    remaining-three clause-`4` `reference` parent captures
  - it removes `4` zero-admitted captures in total and the matching `12`
    remaining-one pruned prefixes
  - it introduces four targeted remaining-two clause-`4` `reference`
    capture families and no introduced pruned-prefix family
- Consequence: clause-`4` `reference` narrowing inside the alternate
  active-window or self-contained families is not the missing clean repair
  either. It stays matched across both alternate families and both active
  clause-`5` buckets, only reconstructing a smaller unsafe split control, so
  the next honest follow-on has to stay above those alternate clause-`4`
  splits too rather than cycling among clause-`4` families or dropping back
  to narrower clause-`3` / clause-`6` selectors.
- The sibling clause-`4` `claim_next_bridge` split above that exhausted
  parent-level qualification-family lattice is now also spent on the
  representative mismatch-`0` claim-side parent-route shell across both
  active clause-`5` families:
  - on either active clause-`5` family, the narrowed route relands a broader
    unsafe control with noncanonical `60 / 8`, `retained = 2`,
    `generated = 4427`, `partial_prefix_bar_failure = 545`,
    `incumbent_dominance = 117`, and zero-admitted captures `2271`
  - the first-mismatch distribution stays `312 / 177 / 50 / 14`
  - `small_cluster` relands as `2931 / 455 / 455 / 115`
  - the isolated `single` bucket reopens to `2` fully scored non-winners plus
    `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread stays `15 / 15 / 12`
  - the representative clause-`4` split stays `24 / 18`
  - it keeps the chosen clause-`4` `claim_next_bridge` plus active
    clause-`5` bucket at `48`
- That narrow clause-`4` `claim_next_bridge` parent-route split is also
  localized at delta level:
  - on the chosen active clause-`5` bucket, it removes the same four targeted
    remaining-two parent cells
  - it removes `8` zero-admitted captures in total and the matching `24`
    remaining-one pruned prefixes
  - it introduces no capture family and no pruned-prefix family
- Consequence: even above the exhausted parent-level qualification-family
  lattice, the representative parent-route clause-`4`
  `claim_next_bridge` split is not the missing clean repair either. It keeps
  the same unsafe `4427 / 545` shell and the same four-cell plus
  `24`-pruned-prefix delta while failing to contract clause-`2` or
  clause-`4` pressure, so the next honest follow-on has to stay above both
  clause-`4` branch splits on that parent-route shell.
- The matched clause-`4` `claim_next_bridge` splits inside the alternate
  active-window and self-contained qualification families are now also spent
  across both active clause-`5` families:
  - on either active clause-`5` family, both alternate clause-`4`
    `claim_next_bridge` splits reland the same broader unsafe control with
    noncanonical `60 / 8`, `retained = 2`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 110`,
    zero-admitted captures `2271`, first-mismatch distribution
    `312 / 177 / 50 / 14`, `small_cluster = 2952 / 558 / 558 / 108`, and the
    same reopened `single` bucket at best overshoot `545 / 5278`
  - the representative clause-`2` spread stays `15 / 15 / 12`
  - the representative clause-`4` split stays `24 / 18`
  - each keeps the chosen clause-`4` `claim_next_bridge` plus active
    clause-`5` bucket at `48`
- Those alternate clause-`4` `claim_next_bridge` splits are also localized at
  delta level:
  - on the chosen active clause-`5` bucket, each probe removes the same four
    targeted remaining-two parent cells
  - it removes `8` zero-admitted captures in total and the matching `24`
    remaining-one pruned prefixes
  - it introduces no capture family and no pruned-prefix family
- Consequence: clause-`4` `claim_next_bridge` narrowing inside the alternate
  active-window or self-contained families is not the missing clean repair
  either. It stays matched across both alternate families and both active
  clause-`5` buckets, only relanding broader unsafe controls, so the next
  honest follow-on has to stay above both clause-`4` branch splits across the
  parent-route and alternate families rather than cycling among clause-`4`
  families or dropping back to narrower clause-`3` / clause-`6` selectors.
- The representative mismatch-`0` claim-side clause-`2` sheet splits above
  that exhausted parent-level qualification-family lattice and above both
  clause-`4` branch splits are now also spent across both active clause-`5`
  families:
  - on the representative parent-route shell, the `claim_flat_domain` and
    `claim_sharp_codomain` clause-`2` overrides each reland unsafe
    noncanonical `60 / 8` with `retained = 1`, `generated = 4379`,
    `partial_prefix_bar_failure = 549`, and
    `small_cluster = 2871 / 435 / 435 / 111`, while the sibling
    clause-`2` `reference` override relands unsafe noncanonical `74 / 8` with
    `retained = 1`, the same `4379 / 549`, and
    `small_cluster = 2844 / 426 / 426 / 120`
  - every parent-route clause-`2` split keeps the `single` pocket fenced at
    `3` residual prunes plus `1` fully scored non-winner and localizes its
    delta to only two targeted remaining-two parent cells plus `12` removed
    remaining-one pruned prefixes on the chosen active clause-`5` bucket,
    with no off-target family introduced
- The sibling clause-`2` sheet splits inside the alternate active-window and
  self-contained qualification families are now also spent across both active
  clause-`5` families:
  - on either alternate family, the `claim_flat_domain` and
    `claim_sharp_codomain` clause-`2` overrides reland the same smaller
    unsafe `60 / 8` with `retained = 1`, `generated = 4379`,
    `partial_prefix_bar_failure = 549`, and
    `small_cluster = 2880 / 486 / 486 / 108`
  - the sibling clause-`2` `reference` override on either alternate family
    relands a different unsafe matched control with noncanonical `60 / 8`,
    `retained = 2`, the same `4379 / 549`, and
    `small_cluster = 2844 / 468 / 468 / 127`
  - every alternate-family clause-`2` split localizes to the same two
    targeted remaining-two parent cells plus `12` removed remaining-one
    pruned prefixes on the chosen active clause-`5` bucket, with no
    off-target family introduced
- Consequence: even above the exhausted parent-level qualification-family
  lattice and above both clause-`4` branch splits, clause-`2` identity on
  the representative parent-route, active-window, and self-contained shells
  is now spent as unsafe control rather than the missing clean repair. The
  next honest follow-on has to stay above these clause-`2` sheet splits too
  and move to a different code-side repair class rather than reopening
  another selector inside the same parent shell.
- Stable repo context should now mirror that ordering too:
  once the representative mismatch-`0` parent-route, qualification-family,
  clause-`4`, and clause-`2` lattice is frozen, the first fresh lever moves
  back to the step-`15` exact-screen engine path rather than to another
  connectivity-family retry.
- The first exact-screen engine probe above that spent connectivity lattice is
  now also spent as a broader tradeoff control:
  - relaxing remaining-two `exact_partial_prefix_bound_decision(...)` on the
    whole mismatch-`0` clause-`4` `claim_next_bridge` half lands
    `4763 / 517 / 2271`
  - accepted step `15` stays canonical `103 / 8`
  - `incumbent_dominance` stays `3` and the isolated `single` pocket stays
    fenced
  - `small_cluster` widens to `3456 / 522 / 522 / 0`
  - every live mismatch-`0` clause-`0` / clause-`1` pair contracts from `42`
    to `36`
  - both clause-`4` branches equalize at `18` per live pair
  - all six live clause-`4` / clause-`5` cells equalize at `36`
  - zero-admitted captures stay `2271`
- Consequence: broad remaining-two exact partial-prefix relief on the whole
  mismatch-`0` `claim_next_bridge` half is not the landed repair either. It
  converts more of the same zero-admitted shell from exact-pruned wall pressure
  into generated survivor mass, so the next honest follow-on has to stay below
  that broad release and split the remaining-two exact-screen class by active
  clause-`5` cell or another narrower remaining-two selector rather than
  reopening connectivity or deeper remaining-one relands.
- That broader exact-screen release now also splits cleanly one layer deeper
  across its clause-`5` cells:
  - the `claim_flat_codomain` cell lands `4475 / 541 / 2271`
  - the `claim_next_codomain` cell lands the same `4475 / 541 / 2271`
  - the `reference` cell lands the same `4475 / 541 / 2271`
  - all three keep accepted step `15` canonical `103 / 8`,
    `incumbent_dominance = 3`, and the isolated `single` pocket fenced
  - all three widen `small_cluster` only to `3240 / 522 / 522 / 0`
  - all three contract every live mismatch-`0` pair from `42` to `40`
  - all three contract the clause-`4` split only from `24 / 18` to
    `22 / 18`
  - all three keep zero-admitted captures fixed at `2271`
  - each single-cell probe localizes to only its chosen
    `claim_next_bridge` clause-`5` family while leaving the two sibling
    `claim_next_bridge` cells at `48` and the whole `reference` half at `36`
- Consequence: clause-`5` identity on the remaining-two exact-screen path is
  now spent too. The broad `4763 / 517 / 2271` release is exactly the union of
  three symmetric smaller tradeoffs, and `claim_next_codomain` is now live on
  this remaining-two exact-screen path even though it stayed inert on the
  remaining-one exact-summary ladder.
- That newly live `claim_next_codomain` exact-screen cell is now also spent
  one layer deeper across its clause-`0` / clause-`1` pairs:
  - every live pair under that cell lands the same smaller
    `4355 / 551 / 2271` tradeoff
  - `small_cluster` widens only to `3150 / 522 / 522 / 0`
  - accepted step `15` stays canonical `103 / 8` and the isolated `single`
    pocket stays fenced
  - each probe contracts only its targeted live pair from `42` to `40`
  - each probe contracts only its targeted clause-`4`
    `claim_next_bridge` share from `24` to `22` while leaving the sibling
    `reference` share at `18`
  - each probe contracts only the chosen clause-`4` `claim_next_bridge` plus
    clause-`5` `claim_next_codomain` bucket from `48` to `46`
  - zero-admitted captures stay fixed at `2271`
- Consequence: clause-`0` / clause-`1` identity inside that exact-screen
  `claim_next_codomain` cell is now spent too. It only reconstructs a uniform
  smaller exact-screen tradeoff above the same zero-admitted shell, so the
  next honest follow-on has to stay below that spent exact-screen pair-cell
  lattice and localize a representative claim-side clause-`2` sheet inside
  that same `claim_next_bridge / claim_next_codomain` pair cell rather than
  reopening sibling exact-screen pairs or dropping into deeper remaining-one
  relands.
- That representative exact-screen pair-cell clause-`2` split is now also
  spent:
  - on `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
    claim_next_codomain`, the `claim_flat_domain` and
    `claim_sharp_codomain` sheets each land the same smaller
    `4343 / 552 / 2271` tradeoff
  - both keep accepted step `15` canonical `103 / 8` and keep
    `incumbent_dominance = 3`
  - both shift first-mismatch counts only to `311 / 177 / 50 / 14`
  - both widen `small_cluster` only to generated `3141`
  - each cuts only its own claim-side clause-`2` share from `15` to `14`,
    shifts the representative clause-`2` spread to `14 / 15 / 12` or
    `15 / 14 / 12`, contracts the representative clause-`4`
    `claim_next_bridge / reference` split only to `23 / 18`, and contracts
    the representative `claim_next_bridge` plus clause-`5`
    `claim_next_codomain` bucket only from `48` to `47`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with first-mismatch counts
    `312 / 177 / 50 / 14` and `small_cluster generated = 3132`
- Consequence: clause-`2` identity inside that representative exact-screen
  pair cell is now spent too. The pair-cell tradeoff is exactly the union of
  the two claim-side sheets while the sibling reference sheet stays inert, so
  the next honest follow-on has to stay below that spent clause-`2` split and
  localize the released representative claim-flat parent beneath the
  remaining-two exact-screen boundary rather than reopening the sibling
  claim-side sheet, the neutral reference sheet, or deeper remaining-one
  relands indiscriminately.
- That representative exact-screen pair-cell `claim_flat_domain` sheet is now
  also localized at delta scope:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / claim_next_codomain`,
    the claim-flat clause-`2` tradeoff removes exactly one remaining-two
    exact-prune capture on that six-clause parent shell
  - that removed capture still has only six fixed clauses, so clause `6`
    stays out of scope at this exact-screen boundary
  - the tradeoff introduces no new exact-prune capture family and removes no
    pruned-terminal family either
- Consequence: exact-screen clause-`6` identity on that representative
  claim-flat sheet is not available at this layer. The next honest follow-on
  has to move below the remaining-two exact-screen boundary and localize that
  single released six-clause parent at remaining-one scope rather than
  retrying another same-layer clause-`6` split, reopening the sibling
  claim-sharp sheet, or bouncing back to sibling exact-screen pairs.
- That same released exact-screen `claim_flat_domain` plus clause-`5`
  `claim_next_codomain` parent is now also spent on remaining-one
  exact-summary relief:
  - stacking remaining-one exact-summary relief beneath the exact-screen
    representative claim-flat tradeoff keeps the full step-`15` surface on
    the same `4343 / 552 / 2271`
  - it keeps the same `311 / 177 / 50 / 14` first-mismatch counts
  - it keeps the same `small_cluster generated = 3141`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)` and the same remaining-one terminal
    connectivity summary with `6813` structurally connected but unqualified
    generated candidates
  - it releases no remaining-one pruned-terminal group at all
- Consequence: remaining-one exact-summary relief on that released
  `claim_next_codomain` parent is only a neutral control. The next honest
  follow-on therefore has to move below summary-prune relief and localize the
  same six-clause parent at direct remaining-one completion or terminal scope
  rather than retrying the same summary override, reopening the sibling
  claim-sharp sheet, or bouncing back to sibling exact-screen pairs.
- That same released exact-screen `claim_flat_domain` plus clause-`5`
  `claim_next_codomain` parent is now also spent at direct remaining-one
  completion and terminal scope:
  - its three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary
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
- Consequence: direct remaining-one completion and terminal identity on that
  released exact-screen representative claim-flat parent are now spent too.
  The next honest follow-on has to move sideways to the sibling exact-screen
  representative `claim_sharp_codomain` sheet on the same pair cell rather
  than retrying claim-flat clause-`6` identity, the same summary override, or
  sibling exact-screen pairs.
- That sibling exact-screen pair-cell `claim_sharp_codomain` sheet is now
  also localized at delta scope:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`,
    the claim-sharp clause-`2` tradeoff removes exactly one remaining-two
    exact-prune capture on that six-clause parent shell
  - that removed capture still has only six fixed clauses, so clause `6`
    stays out of scope at this exact-screen boundary
  - the tradeoff introduces no new exact-prune capture family and removes no
    pruned-terminal family either
- Consequence: exact-screen clause-`6` identity on that representative
  claim-sharp sheet is not available at this layer either. The next honest
  follow-on has to move below the remaining-two exact-screen boundary and
  localize that single released six-clause parent at remaining-one scope
  rather than reopening the spent claim-flat sheet, the neutral `reference`
  sheet, or sibling exact-screen pairs.
- That same released exact-screen `claim_sharp_codomain` plus clause-`5`
  `claim_next_codomain` parent is now also spent on remaining-one
  exact-summary relief:
  - stacking remaining-one exact-summary relief beneath the exact-screen
    representative claim-sharp tradeoff keeps the full step-`15` surface on
    the same `4343 / 552 / 2271`
  - it keeps the same `311 / 177 / 50 / 14` first-mismatch counts
  - it keeps the same `small_cluster generated = 3141`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)` and the same remaining-one terminal
    connectivity summary with `6813` structurally connected but unqualified
    generated candidates
  - it releases no remaining-one pruned-terminal group at all
- Consequence: remaining-one exact-summary relief on that released
  `claim_next_codomain` parent is only a neutral control on the claim-sharp
  sheet too. The next honest follow-on therefore has to move below
  summary-prune relief and localize the same six-clause parent at direct
  remaining-one completion or terminal scope rather than retrying the same
  summary override, reopening the spent claim-flat sheet, the neutral
  `reference` sheet, or sibling exact-screen pairs.
- That same released exact-screen `claim_sharp_codomain` plus clause-`5`
  `claim_next_codomain` parent is now also spent at direct remaining-one
  completion and terminal scope:
  - its three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary
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
- Consequence: both representative exact-screen claim-side sheets on that
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge / claim_next_codomain`
  pair cell are now spent too. The next honest follow-on has to stop below
  that spent representative pair and move sideways to a sibling exact-screen
  pair on the same `claim_next_codomain` cell rather than reopening either
  claim-side sheet, the neutral `reference` sheet, or the same remaining-one
  follow-ons.
- That first sibling exact-screen pair on the same `claim_next_codomain`
  cell is now also spent at clause-`2` identity scope:
  - on `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge /
    claim_next_codomain`, the `claim_flat_domain` and
    `claim_sharp_codomain` sheets each land the same smaller
    `4343 / 552 / 2271` tradeoff
  - both keep accepted step `15` canonical `103 / 8` and keep
    `incumbent_dominance = 3`
  - both shift first-mismatch counts only to `311 / 177 / 50 / 14`
  - both widen `small_cluster` only to generated `3141`
  - each cuts only its own claim-side clause-`2` share from `15` to `14`,
    leaving the sibling claim-side sheet at `15` and the sibling
    `reference` sheet at `12`
  - each contracts the targeted exact-screen pair only from `42` to `41`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with first-mismatch counts
    `312 / 177 / 50 / 14`, `small_cluster generated = 3132`, and the same
    targeted pair still at `42`
- Consequence: clause-`2` identity on that first sibling exact-screen pair is
  not the missing repair either. The next honest follow-on has to move below
  the representative `claim_flat_domain` sheet on
  `claim_eventual_domain / claim_sharp_codomain / claim_next_bridge /
  claim_next_codomain` rather than reopening the sibling
  `claim_sharp_codomain` sheet, the neutral `reference` sheet, or any
  remaining-one relands on the spent representative pair.
- That first sibling exact-screen `claim_flat_domain` sheet is now also
  localized at delta scope:
  - under
    `claim_eventual_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / claim_next_codomain`,
    the claim-flat clause-`2` tradeoff removes exactly one remaining-two
    exact-prune capture on that six-clause parent shell
  - that removed capture still has only six fixed clauses, so clause `6`
    stays out of scope at this exact-screen boundary
  - the tradeoff introduces no new exact-prune capture family and removes no
    pruned-terminal family either
- Consequence: exact-screen clause-`6` identity on that first sibling
  claim-flat sheet is not available at this layer either. The next honest
  follow-on has to move below the remaining-two exact-screen boundary and
  localize that single released six-clause parent at remaining-one scope
  rather than reopening the sibling claim-sharp sheet, the neutral
  `reference` sheet, or the same remaining-one relands on the spent
  representative pair.
- That same released first sibling exact-screen `claim_flat_domain` plus
  clause-`5` `claim_next_codomain` parent is now also spent on remaining-one
  exact-summary relief:
  - stacking remaining-one exact-summary relief beneath the first sibling
    exact-screen claim-flat tradeoff keeps the full step-`15` surface on the
    same `4343 / 552 / 2271`
  - it keeps the same `311 / 177 / 50 / 14` first-mismatch counts
  - it keeps the same `small_cluster generated = 3141`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)` and the same remaining-one terminal
    connectivity summary with `6813` structurally connected but unqualified
    generated candidates
  - it releases no remaining-one pruned-terminal group at all
- Consequence: remaining-one exact-summary relief on that released first
  sibling exact-screen claim-flat parent is only a neutral control too. The
  next honest follow-on therefore has to move below summary-prune relief and
  localize the same six-clause parent at direct remaining-one completion or
  terminal scope rather than retrying the same summary override, reopening
  the sibling claim-sharp sheet, the neutral `reference` sheet, or bouncing
  back to the spent representative pair.
- That same released first sibling exact-screen `claim_flat_domain` plus
  clause-`5` `claim_next_codomain` parent is now also spent at direct
  remaining-one completion and terminal scope:
  - its three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary
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
- Consequence: direct remaining-one completion and terminal identity on that
  released first sibling exact-screen claim-flat parent are now spent too.
  The next honest follow-on has to move sideways to the sibling exact-screen
  `claim_sharp_codomain` sheet on the same pair cell rather than retrying
  claim-flat clause-`6` identity, the same summary override, the neutral
  `reference` sheet, or the spent representative pair.
- That same first sibling exact-screen `claim_sharp_codomain` sheet is now
  also localized at delta scope:
  - under
    `claim_eventual_domain / claim_sharp_codomain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`,
    the claim-sharp clause-`2` tradeoff removes exactly one remaining-two
    exact-prune capture on that six-clause parent shell
  - that removed capture still has only six fixed clauses, so clause `6`
    stays out of scope at this exact-screen boundary
  - the tradeoff introduces no new exact-prune capture family and removes no
    pruned-terminal family either
- Consequence: exact-screen clause-`6` identity on that first sibling
  claim-sharp sheet is not available at this layer either. The next honest
  follow-on therefore has to move below the remaining-two exact-screen
  boundary and localize that single released six-clause parent at
  remaining-one scope rather than reopening the spent first sibling
  claim-flat sheet, the neutral `reference` sheet there, or the spent
  representative pair.
- That same released first sibling exact-screen `claim_sharp_codomain` plus
  clause-`5` `claim_next_codomain` parent is now also spent on remaining-one
  exact-summary relief:
  - stacking remaining-one exact-summary relief beneath the first sibling
    exact-screen claim-sharp tradeoff keeps the full step-`15` surface on the
    same `4343 / 552 / 2271`
  - it keeps the same `311 / 177 / 50 / 14` first-mismatch counts
  - it keeps the same `small_cluster generated = 3141`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)` and the same remaining-one terminal
    connectivity summary with `6813` structurally connected but unqualified
    generated candidates
  - it releases no remaining-one pruned-terminal group at all
- Consequence: remaining-one exact-summary relief on that released first
  sibling exact-screen claim-sharp parent is only a neutral control too. The
  next honest follow-on therefore has to move below summary-prune relief and
  localize the same six-clause parent at direct remaining-one completion or
  terminal scope rather than retrying the same summary override, reopening
  the spent first sibling claim-flat sheet, the neutral `reference` sheet, or
  bouncing back to the spent representative pair.
- That same released first sibling exact-screen `claim_sharp_codomain` plus
  clause-`5` `claim_next_codomain` parent is now also spent at direct
  remaining-one completion and terminal scope:
  - its three clause-`6` continuations `claim_next_codomain`,
    `claim_sharp_codomain`, and `reference` each reland the same dead
    `3`-generated / `0`-admitted completion summary
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
- Consequence: both claim-side sheets on that first sibling exact-screen pair
  are now spent too. The next honest follow-on therefore had to move sideways
  to the next sibling exact-screen pair on the same `claim_next_codomain`
  cell, namely
  `claim_eventual_domain / reference / claim_next_bridge / claim_next_codomain`,
  rather than reopening the spent first sibling claim-flat or claim-sharp
  sheets, the same remaining-one follow-ons, or the already-spent
  representative pair.
- That next sibling exact-screen pair on the same `claim_next_codomain` cell
  is now also spent at clause-`2` identity scope:
  - on `claim_eventual_domain / reference / claim_next_bridge /
    claim_next_codomain`, the `claim_flat_domain` and
    `claim_sharp_codomain` sheets each land the same smaller
    `4343 / 552 / 2271` tradeoff
  - both keep accepted step `15` canonical `103 / 8` and keep
    `incumbent_dominance = 3`
  - both shift first-mismatch counts only to `311 / 177 / 50 / 14`
  - both widen `small_cluster` only to generated `3141`
  - each cuts only its own claim-side clause-`2` share from `15` to `14`,
    leaving the sibling claim-side sheet at `15` and the sibling
    `reference` sheet at `12`
  - each contracts the targeted exact-screen pair only from `42` to `41`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with first-mismatch counts
    `312 / 177 / 50 / 14`, `small_cluster generated = 3132`, and the same
    targeted pair still at `42`
- Both next-sibling exact-screen claim-side sheets on that same pair are now
  also exhausted through the same remaining-two boundary and remaining-one
  sequence:
  - each exact-screen delta removes exactly one remaining-two exact-prune
    capture on its released six-clause parent shell, still leaves clause `6`
    out of scope at the exact-screen boundary, and introduces no new
    exact-prune or pruned-terminal family
  - stacking remaining-one exact-summary relief beneath either released
    parent stays completely neutral on the same `4343 / 552 / 2271`, the same
    `311 / 177 / 50 / 14`, the same `small_cluster = 3141 / 522 / 522 / 0`,
    the same zero-admitted exact-prune family `((0, None, None), 2271)`, and
    the same `6813` structurally connected but unqualified generated
    candidates, with no released remaining-one pruned-terminal group
  - each released parent's three clause-`6` continuations reland the same
    dead `3`-generated / `0`-admitted completion summary with no bound, no
    best-rank profile, no survivor sketch, and the same
    `reference / eventual_lift / next_lift` `NeedsFallback` trio
  - every one of those clause-`6` continuations stays uniformly blocked at
    clause `2` with `matched_clause_count = 2` and
    `first_mismatch_position = 2`
- Consequence: all three `claim_eventual_domain` exact-screen pairs on the
  active clause-`5` `claim_next_codomain` cell are now spent too. The next
  honest follow-on has to stop below that exhausted clause-`0` family and
  move sideways to the first clause-`0` sibling exact-screen pair on the same
  cell, namely
  `claim_flat_domain / claim_next_codomain / claim_next_bridge / claim_next_codomain`,
  rather than reopening the spent representative, first sibling, or next
  sibling pairs, their neutral `reference` sheets, or the same remaining-one
  follow-ons.
- That first clause-`0` sibling exact-screen pair on the same
  `claim_next_codomain` cell is now also spent at clause-`2` identity scope:
  - on `claim_flat_domain / claim_next_codomain / claim_next_bridge /
    claim_next_codomain`, the `claim_flat_domain` and
    `claim_sharp_codomain` sheets each land the same smaller
    `4343 / 552 / 2271` tradeoff
  - both keep accepted step `15` canonical `103 / 8` and keep
    `incumbent_dominance = 3`
  - both shift first-mismatch counts only to `311 / 177 / 50 / 14`
  - both widen `small_cluster` only to generated `3141`
  - each cuts only its own claim-side clause-`2` share from `15` to `14`,
    leaving the sibling claim-side sheet at `15` and the sibling
    `reference` sheet at `12`
  - each contracts the targeted exact-screen pair only from `42` to `41`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with first-mismatch counts
    `312 / 177 / 50 / 14`, `small_cluster generated = 3132`, and the same
    targeted pair still at `42`
- Both first-clause-`0`-sibling exact-screen claim-side sheets on that same
  pair are now also exhausted through the same remaining-two boundary and
  remaining-one sequence:
  - each exact-screen delta removes exactly one remaining-two exact-prune
    capture on its released six-clause parent shell, still leaves clause `6`
    out of scope at the exact-screen boundary, and introduces no new
    exact-prune or pruned-terminal family
  - stacking remaining-one exact-summary relief beneath either released
    parent stays completely neutral on the same `4343 / 552 / 2271`, the same
    `311 / 177 / 50 / 14`, the same `small_cluster = 3141 / 522 / 522 / 0`,
    the same zero-admitted exact-prune family `((0, None, None), 2271)`, and
    the same `6813` structurally connected but unqualified generated
    candidates, with no released remaining-one pruned-terminal group
  - each released parent's three clause-`6` continuations reland the same
    dead `3`-generated / `0`-admitted completion summary with no bound, no
    best-rank profile, no survivor sketch, and the same
    `reference / eventual_lift / next_lift` `NeedsFallback` trio
  - every one of those clause-`6` continuations stays uniformly blocked at
    clause `2` with `matched_clause_count = 2` and
    `first_mismatch_position = 2`
- Consequence: the first clause-`0` sibling exact-screen pair on the active
  clause-`5` `claim_next_codomain` cell is now spent too. The next honest
  follow-on has to move sideways to the first clause-`1` sibling exact-screen
  pair on that same clause-`0` `claim_flat_domain` family, namely
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`,
  rather than reopening the spent `claim_eventual_domain` representative,
  first-sibling, or next-sibling pairs, their neutral `reference` sheets, or
  the spent first clause-`0` sibling pair.
- That first clause-`1` sibling exact-screen pair on the same
  `claim_flat_domain` clause-`0` family is now also spent at clause-`2`
  identity scope:
  - on `claim_flat_domain / claim_sharp_codomain / claim_next_bridge /
    claim_next_codomain`, the `claim_flat_domain` and
    `claim_sharp_codomain` sheets each land the same smaller
    `4343 / 552 / 2271` tradeoff
  - both keep accepted step `15` canonical `103 / 8` and keep
    `incumbent_dominance = 3`
  - both shift first-mismatch counts only to `311 / 177 / 50 / 14`
  - both widen `small_cluster` only to generated `3141`
  - each cuts only its own claim-side clause-`2` share from `15` to `14`,
    leaving the sibling claim-side sheet at `15` and the sibling
    `reference` sheet at `12`
  - each contracts the targeted exact-screen pair only from `42` to `41`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with first-mismatch counts
    `312 / 177 / 50 / 14`, `small_cluster generated = 3132`, and the same
    targeted pair still at `42`
- Consequence: clause-`2` identity on that first clause-`1` sibling
  exact-screen pair is not the missing repair either. The next honest
  follow-on has to move below the representative `claim_flat_domain` sheet on
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`
  rather than reopening the sibling `claim_sharp_codomain` sheet, the
  neutral `reference` sheet, or any remaining-one relands on the spent
  representative, first-sibling, next-sibling, or first-clause-`0`-sibling
  pairs.
- That first clause-`1` sibling exact-screen `claim_flat_domain` sheet is now
  also localized at delta scope:
  - under
    `claim_flat_domain / claim_sharp_codomain / claim_flat_domain / claim_next_bridge / claim_next_codomain`,
    the claim-flat clause-`2` tradeoff removes exactly one remaining-two
    exact-prune capture on that six-clause parent shell
  - that removed capture still has only six fixed clauses, so clause `6`
    stays out of scope at this exact-screen boundary
  - the tradeoff introduces no new exact-prune capture family and removes no
    pruned-terminal family either
- Consequence: exact-screen clause-`6` identity on that first clause-`1`
  sibling claim-flat sheet is not available at this layer either. The next
  honest follow-on has to move below the remaining-two exact-screen boundary
  and localize that single released six-clause parent at remaining-one
  exact-summary scope rather than reopening the sibling claim-sharp sheet,
  the neutral `reference` sheet, or the spent representative, first-sibling,
  next-sibling, or first-clause-`0`-sibling pairs.
- That same released first clause-`1` sibling exact-screen
  `claim_flat_domain` plus clause-`5` `claim_next_codomain` parent is now
  also spent on remaining-one exact-summary relief:
  - stacking remaining-one exact-summary relief beneath the first clause-`1`
    sibling exact-screen claim-flat tradeoff keeps the full step-`15`
    surface on the same `4343 / 552 / 2271`
  - it keeps the same `311 / 177 / 50 / 14` first-mismatch counts
  - it keeps the same `small_cluster generated = 3141`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)` and the same remaining-one terminal
    connectivity summary with `6813` structurally connected but unqualified
    generated candidates
  - it releases no remaining-one pruned-terminal group at all
- Consequence: remaining-one exact-summary relief on that released first
  clause-`1` sibling exact-screen claim-flat parent is only a neutral control
  too. The next honest follow-on therefore has to move below summary-prune
  relief and localize the same six-clause parent at direct remaining-one
  completion or terminal scope rather than retrying the same summary
  override, reopening the sibling claim-sharp sheet, the neutral
  `reference` sheet, or bouncing back to the spent representative,
  first-sibling, next-sibling, or first-clause-`0`-sibling pairs.
- That same released first clause-`1` sibling exact-screen
  `claim_flat_domain` plus clause-`5` `claim_next_codomain` parent is now
  also spent at direct remaining-one completion / terminal scope:
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
- Consequence: the released first clause-`1` sibling exact-screen
  `claim_flat_domain` parent is now spent through direct remaining-one
  completion too. The next honest follow-on has to move sideways to the
  sibling exact-screen `claim_sharp_codomain` sheet on
  `claim_flat_domain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`
  rather than retrying the spent claim-flat parent, the same summary
  override, the neutral `reference` sheet, or the spent representative,
  first-sibling, next-sibling, and first-clause-`0`-sibling pairs.
- That first clause-`1` sibling exact-screen `claim_sharp_codomain` sheet on
  that same pair is now also localized at delta scope:
  - under
    `claim_flat_domain / claim_sharp_codomain / claim_sharp_codomain / claim_next_bridge / claim_next_codomain`,
    the claim-sharp clause-`2` tradeoff removes exactly one remaining-two
    exact-prune capture on that six-clause parent shell
  - that removed capture still has only six fixed clauses, so clause `6`
    stays out of scope at this exact-screen boundary
  - the tradeoff introduces no new exact-prune capture family and removes no
    pruned-terminal family either
- Consequence: exact-screen clause-`6` identity on that first clause-`1`
  sibling claim-sharp sheet is not available at this layer either. The next
  honest follow-on therefore has to move below the remaining-two exact-screen
  boundary and localize the released six-clause parent at remaining-one scope
  rather than reopening the spent claim-flat sheet, the neutral `reference`
  sheet there, or the spent representative, first-sibling, next-sibling, or
  first-clause-`0`-sibling pairs.
- That same released first clause-`1` sibling exact-screen
  `claim_sharp_codomain` plus clause-`5` `claim_next_codomain` parent is now
  also spent on remaining-one exact-summary relief:
  - stacking remaining-one exact-summary relief beneath the first clause-`1`
    sibling exact-screen claim-sharp tradeoff keeps the full step-`15`
    surface on the same `4343 / 552 / 2271`
  - it keeps the same `311 / 177 / 50 / 14` first-mismatch counts
  - it keeps the same `small_cluster generated = 3141`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)` and the same remaining-one terminal
    connectivity summary with `6813` structurally connected but unqualified
    generated candidates
  - it releases no remaining-one pruned-terminal group at all
- Consequence: remaining-one exact-summary relief on that released first
  clause-`1` sibling exact-screen claim-sharp parent is only a neutral
  control too. The next honest follow-on therefore has to move below
  summary-prune relief and localize the same six-clause parent at direct
  remaining-one completion or terminal scope rather than retrying the same
  summary override, reopening the spent claim-flat sheet, the neutral
  `reference` sheet there, or bouncing back to the spent representative,
  first-sibling, next-sibling, or first-clause-`0`-sibling pairs.
- That same released first clause-`1` sibling exact-screen
  `claim_sharp_codomain` plus clause-`5` `claim_next_codomain` parent is now
  also spent at direct remaining-one completion / terminal scope:
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
- Consequence: both claim-side sheets on that first clause-`1` sibling
  exact-screen pair are now spent too. The next honest follow-on has to move
  sideways to the next clause-`1` sibling exact-screen pair on the same
  clause-`0` `claim_flat_domain` family, namely
  `claim_flat_domain / reference / claim_next_bridge / claim_next_codomain`,
  rather than reopening the spent claim-flat or claim-sharp sheets, the
  neutral `reference` sheet inside that spent pair, or the spent
  representative, first-sibling, next-sibling, or first-clause-`0`-sibling
  pairs.
- That next clause-`1` sibling exact-screen pair on the same
  `claim_flat_domain` clause-`0` family is now also spent at clause-`2`
  identity scope:
  - on `claim_flat_domain / reference / claim_next_bridge /
    claim_next_codomain`, the `claim_flat_domain` and
    `claim_sharp_codomain` sheets each land the same smaller
    `4343 / 552 / 2271` tradeoff
  - both keep accepted step `15` canonical `103 / 8` and keep
    `incumbent_dominance = 3`
  - both shift first-mismatch counts only to `311 / 177 / 50 / 14`
  - both widen `small_cluster` only to generated `3141`
  - each cuts only its own claim-side clause-`2` share from `15` to `14`,
    leaving the sibling claim-side sheet at `15` and the sibling
    `reference` sheet at `12`
  - each contracts the targeted exact-screen pair only from `42` to `41`
  - the sibling `reference` sheet is a neutral control on the untouched
    `4331 / 553 / 2271` baseline with first-mismatch counts
    `312 / 177 / 50 / 14`, `small_cluster generated = 3132`, and the same
    targeted pair still at `42`
- Consequence: clause-`2` identity on that next clause-`1` sibling
  exact-screen pair is now spent too. The next honest follow-on has to move
  below the representative `claim_flat_domain` sheet on
  `claim_flat_domain / reference / claim_next_bridge / claim_next_codomain`
  rather than reopening the sibling `claim_sharp_codomain` sheet, the
  neutral `reference` sheet, or any remaining-one relands on the spent
  representative, first-sibling, next-sibling, first-clause-`0`-sibling, or
  first-clause-`1`-sibling pairs.
- That representative next clause-`1` sibling exact-screen
  `claim_flat_domain` sheet on that same pair is now also localized at delta
  scope:
  - under
    `claim_flat_domain / reference / claim_flat_domain / claim_next_bridge / claim_next_codomain`,
    the claim-flat clause-`2` tradeoff removes exactly one remaining-two
    exact-prune capture on that released six-clause parent shell
  - that removed capture still has only six fixed clauses, so clause `6`
    stays out of scope at this exact-screen boundary
  - the tradeoff introduces no new exact-prune capture family and removes no
    pruned-terminal family either
- Consequence: exact-screen clause-`6` identity on that next clause-`1`
  sibling claim-flat sheet is not available at this layer either. The next
  honest follow-on therefore has to move below the remaining-two exact-screen
  boundary and localize the released six-clause parent at remaining-one scope
  rather than reopening the spent pair-cell split, the sibling
  `claim_sharp_codomain` sheet, the neutral `reference` sheet there, or the
  spent representative, first-sibling, next-sibling, first-clause-`0`-
  sibling, or first-clause-`1`-sibling pairs.
- That same released next clause-`1` sibling exact-screen
  `claim_flat_domain` plus clause-`5` `claim_next_codomain` parent is now
  also spent on remaining-one exact-summary relief:
  - stacking remaining-one exact-summary relief beneath that next
    clause-`1` sibling exact-screen claim-flat tradeoff keeps the full
    step-`15` surface on the same `4343 / 552 / 2271`
  - it keeps the same `311 / 177 / 50 / 14` first-mismatch counts
  - it keeps the same `small_cluster generated = 3141`
  - it keeps the same zero-admitted exact-prune family
    `((0, None, None), 2271)` and the same remaining-one terminal
    connectivity summary with `6813` structurally connected but unqualified
    generated candidates
  - it releases no remaining-one pruned-terminal group at all
- Consequence: remaining-one exact-summary relief on that released next
  clause-`1` sibling exact-screen claim-flat parent is only a neutral control
  too. The next honest follow-on therefore has to move below summary-prune
  relief and localize the same six-clause parent at direct remaining-one
  completion or terminal scope rather than retrying the same summary
  override, reopening the spent pair-cell split, the sibling
  `claim_sharp_codomain` sheet, the neutral `reference` sheet there, or
  bouncing back to the spent representative, first-sibling, next-sibling,
  first-clause-`0`-sibling, or first-clause-`1`-sibling pairs.
- That same released next clause-`1` sibling exact-screen
  `claim_flat_domain` plus clause-`5` `claim_next_codomain` parent is now
  also spent at direct remaining-one completion / terminal scope:
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
- Consequence: the representative claim-flat sheet on that next clause-`1`
  sibling exact-screen pair is now spent too. The next honest follow-on has
  to move sideways to the sibling exact-screen `claim_sharp_codomain` sheet
  on that same pair,
  `claim_flat_domain / reference / claim_next_bridge / claim_next_codomain`,
  rather than reopening the spent representative claim-flat sheet there, the
  neutral `reference` sheet on that pair, or the spent representative,
  first-sibling, next-sibling, first-clause-`0`-sibling, or
  first-clause-`1`-sibling pairs.
- Do not reopen representative claim-side clause-`2` sheet identity first on
  the parent-route, active-window, or self-contained shells; those clause-`2`
  splits now belong to the live autonomy docs and ledger rather than as fresh
  leads.
- Do not reopen the exact claim-flat or claim-sharp single-sheet splits on the
  clause-`4` `claim_next_bridge` half first; they are now smaller tradeoff
  controls rather than the landed repair.
- Keep step `1` explicit but deferred:
  - both stored reruns still pin it at `546 / 2144`
  - do not promote a step-`1` theory slice unless the newer rerun changes the
    diagnosis instead of simply re-confirming the step-`15` miss
- Use a later rerun beyond `v13` to refresh compare, benchmark, and
  certification before reopening step `1`.
