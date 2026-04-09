# Autonomous Claim Lane State

Last updated: 2026-04-09

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
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v12`
- Stored compare is ready.
- Stored benchmark is refreshed across `v11` and `v12`.
- Accepted-hash parity is earned through step `15`.
- Stored certification still fails only on breadth:
  - step `1 = 546 / 2144`
  - step `15 = 4331 / 5000`

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
    identity either; the next honest revisit should move sideways to the
    representative `claim_sharp_codomain` clause-`2` sheet unless a narrower
    connectivity-failure reason partition under one dead claim-flat child is
    explicitly promoted
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
- The residual `3` `single`-bucket incumbent prunes remain fenced and are now
  secondary to the partial-prefix wall.

## Current Operating Position

- Keep wording at `bounded live recovery`.
- Keep step `1` explicit, but do not reopen it first unless a newer stored
  rerun changes the diagnosis.
- When working outside the exhausted mismatch-`1`
  `reference + demo_flat_codomain` ladder, treat the mismatch-`0`
  claim-domain surface as exhausted through the representative claim-side
  clause-`6` identity passes and move to the smaller claim-safe mismatch-`1`
  tier before the `reference / reference` tails. The promoted representative
  claim-safe dead-prefix reason partition is now uniform at clause `5`, so
  clause-`6` and terminal-family identity are frozen controls rather than
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
- If the claim-safe mismatch-`1` tier is revisited next, move below one
  representative dead prefix's now-uniform clause-`5` reason wall, starting
  with
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain /
  claim_flat_codomain / reference`; use the already-exhausted clause-`6` and
  terminal-family siblings plus the other five dead prefixes only as matched
  controls, and partition by narrower clause-`5` qualification behavior rather
  than by another claim-side sheet, clause-`6`, or terminal-family reland.
- If the mismatch-`0` claim-domain surface is revisited, stay below the broad
  clause-`1` `demo_flat_codomain` reopening and below whole-tier parent
  exact-bound relief; the former reaches `4985` only by widening the clean
  wall to `667`, and the latter drops the wall to `241` only by exploding the
  noncanonical `small_cluster`.
- When revisiting mismatch-`0`, work one layer deeper below the now-exhausted
  clause-`4` `claim_next_bridge / reference` split, starting with
  remaining-one exact-summary behavior under the same live pairings before any
  narrower aligned clause-`5` reopening, rather than on a broad parent-level
  release of the whole tier.
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
- Do not stop at that representative claim-flat joint clause-`3`
  parent/child shell either; its six child continuations are now all the same
  dead `3`-generated / `0`-admitted completion summary with no bound and no
  survivor sketch.
- Because those claim-flat child continuations only expose the same nonlive
  `reference / eventual_lift / next_lift` open-band structural trio, the next
  honest revisit should now move off mismatch-`0` identity partitions and onto
  the smaller claim-safe mismatch-`1` tier before the `reference / reference`
  tails, unless a reason-level connectivity partition under one frozen
  representative claim-side shell is explicitly promoted.
- Do not stop at the representative `claim_sharp_codomain` clause-`2` sheet's
  clause-`6` `claim_next_codomain`, `claim_sharp_codomain`, or `reference`
  continuations either; all three reland the same `4343 / 552 / 3141`
  smaller tradeoff shell and differ only by a tiny zero-admitted tail delta.
- Because the representative claim-sharp sheet is now exhausted at clause-`6`
  identity scope too, the next honest slice no longer sits on another
  mismatch-`0` claim-side identity reland; it should move to the smaller
  claim-safe mismatch-`1` tier unless a narrower reason-level connectivity
  partition under one frozen representative claim-side shell is explicitly
  promoted.
- Do not reopen the exact claim-flat or claim-sharp single-sheet splits on the
  clause-`4` `claim_next_bridge` half first; they are now smaller tradeoff
  controls rather than the landed repair.
- After a real local repair lands, immediately rerun beyond `v12` and refresh
  compare, benchmark, and certification.
