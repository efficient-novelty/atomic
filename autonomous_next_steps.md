# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-09

This file owns the single active work order for `desktop_claim_shadow`.

## Contract

- Keep exactly one active objective and one current hypothesis here.
- Move completed or ruled-out probes to
  [autonomous_ledger.md](autonomous_ledger.md).
- Do not restate medium-horizon phases or historical context here. Use
  [autonomous_plan.md](autonomous_plan.md) and
  [autonomous_progress.md](autonomous_progress.md).

## Objective

Keep the active work order off the exhausted mismatch-`0` identity relands and
below the now-exhausted representative claim-safe mismatch-`1` dead prefix's
clause-`6` and terminal-family controls, so the next slice starts from the
newly localized clause-`5` reason wall instead of reopening another identity
reland on top of canonical `v12` and the matched local `4331` guardrail
surface.

Do not start with another rerun setup pass.

## Start From

- Canonical stored bundle:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v12`
- Stored breadth blockers:
  - step `1 = 546 / 2144`
  - step `15 = 4331 / 5000`
- Current clean late chain:
  - step `13 = [5,1,3,3,5,3,2] / 1350 / 2320`
  - step `14 = 62 / 9 / 12027`
  - step `15 = DCT 103 / 8 / 4331`
- Current step-`15` pressure:
  - partial-prefix bar failures: `553`
  - incumbent-dominance prunes: `3`
  - `small_cluster = 3132 / 522 / 522 / 0`
  - fenced `single` bucket = `1` fully scored non-winner plus `3` residual
    prunes

## Active Hypothesis

- The exact claim-pair clause-`4` `reference` side is now exhausted as a
  smaller tradeoff control: it only reproduces the older
  `4379 / 549 / 2259` clause-`4` `reference`-sheet tradeoff and still widens
  `small_cluster` to `3180 / 530 / 530 / 0`.
- The exact claim-flat and exact claim-sharp single-sheet relocalizations on
  the live clause-`4` `claim_next_bridge` half are now also exhausted as
  smaller tradeoff controls: each lands `4373 / 545 / 2259`, widens
  `small_cluster` to `3180 / 530 / 530 / 0`, and only repartitions the same
  `51`-capture dominant mismatch-`1` branch between the two claim clause-`2`
  sheets.
- The best remaining leverage is therefore no longer on the clause-`4`
  `reference` side, the reference clause-`2` sheet, or the exact claim-flat
  versus claim-sharp single-sheet split inside the dominant remaining-two
  clause-`0` / clause-`1` mismatch-`1` surface.
- The exhausted mismatch-`1` `reference + demo_flat_codomain` ladder is now
  cleanly separated from the rest of the wall:
  - excluding that frozen `61`-capture ladder leaves `390` off-branch
    remaining-two captures across `10` pairings
  - the next live priority tier is the mismatch-`0` claim-domain surface at
    `252`
  - the smaller claim-safe mismatch-`1` tier is `84`
  - mismatch-`2` plus mismatch-`3` `reference / reference` tails are only `54`
- That earlier ordering promoted the mismatch-`0` claim-domain surface first;
  with the representative claim-side clause-`6` identity passes now frozen on
  both claim sheets, the next honest slice moves to the smaller claim-safe
  mismatch-`1` tier before the `reference / reference` tails.
- A direct parent-level exact-bound bypass on that whole remaining-two
  mismatch-`0` claim-domain tier is now ruled out as a widening negative
  control:
  - redirecting the full tier from `CannotClearBar` to `Unknown` inside
    `exact_partial_prefix_bound_decision(...)` dropped the clean wall only by
    releasing the entire first-mismatch-`0` family, from `553` to `241`
  - `small_cluster` generated-terminal pressure exploded from `3132` to
    `7020`
  - under the existing mismatch-`0` demo-flat negative control, generated
    breadth jumped from `4985` to `10349`
  - the isolated `single` pocket stayed fenced, so the failure mode is the
    noncanonical `small_cluster`, not a reopened single-pocket rival
- The broad clause-`1` `demo_flat_codomain` reopening across the full
  mismatch-`0` claim-domain surface is now also exhausted as a widening
  negative control:
  - it lands `4985 / 667 / 2757`
  - it widens `small_cluster` to `3564 / 594 / 594 / 0`
  - it adds two new mismatch-`0` pairings at `45 / 45`
  - each new pairing sits on the same clause-`4` split `27 / 18`
  - each new pairing also spreads evenly across the three clause-`2` sheets at
    `15 / 15 / 15`
- The narrower mismatch-`0` clause-`4` `claim_next_bridge`-side relocalization
  under that same demo-flat reopening is now also exhausted as a smaller
  negative control:
  - it lands `4829 / 671 / 2793`
  - it widens `small_cluster` to `3420 / 570 / 570 / 0`
  - it leaves the same two mismatch-`0` pairings at `45 / 45`
  - it leaves the same clause-`4` split `27 / 18`
  - it leaves the same clause-`2` spread `15 / 15 / 15`
- The smaller mismatch-`0` clause-`4` `reference`-side relocalization under
  that same demo-flat reopening is now also exhausted as a sharper negative
  control:
  - it lands `4697 / 691 / 2829`
  - it widens `small_cluster` to `3276 / 546 / 546 / 0`
  - it inflates the same two mismatch-`0` pairings to `57 / 57`
  - it widens the clause-`4` split to `33 / 24`
  - it leans the clause-`2` spread toward the two claim sheets at
    `21 / 21 / 15`
- The narrower remaining-one exact-summary relief on the existing mismatch-`0`
  clause-`4` `reference` plus clause-`5` `reference` tail is now also
  exhausted as a deeper negative control:
  - it lands `4547 / 589 / 2235`
  - it widens `small_cluster` to `3240 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - each live mismatch-`0` pairing inflates from `42` to `52`
  - the clause-`4` split leans harder toward `reference`, from `24 / 18` to
    `24 / 28`
- The whole mismatch-`0` clause-`4` `claim_next_bridge` half is now the first
  honest remaining-one exact-summary tradeoff on that claim-domain surface:
  - it lands `4619 / 529 / 2199`
  - it widens `small_cluster` to `3348 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - each live mismatch-`0` pairing contracts from `42` to `38`
  - the clause-`4` split contracts only the `claim_next_bridge` share, from
    `24 / 18` to `20 / 18`
- That whole-half tradeoff now splits cleanly one layer deeper:
  - the `claim_flat_codomain` cell under the same mismatch-`0`
    `claim_next_bridge` half lands `4475 / 541 / 2235`
  - the `reference` cell under that same half lands the same
    `4475 / 541 / 2235`
  - each smaller tradeoff widens `small_cluster` to `3240 / 522 / 522 / 0`
  - each smaller tradeoff contracts every live mismatch-`0` pairing from
    `42` to `40`
  - each smaller tradeoff contracts only the clause-`4`
    `claim_next_bridge` share from `24 / 18` to `22 / 18`
  - the sibling clause-`5` `claim_next_codomain` cell is a neutral control on
    `4331 / 553 / 2271`
- Those two active clause-`5` cells are now also exhausted at pair-cell scope:
  - every one of the `12` live pair cells below them lands the same
    `4355 / 551 / 2265` smaller tradeoff
  - `small_cluster` widens only to `3150 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - each probe contracts only its targeted live mismatch-`0` pair from
    `42` to `40`
  - each probe contracts only its targeted clause-`4` `claim_next_bridge`
    plus active clause-`5` bucket from `48` to `46`
- Any remaining mismatch-`0` leverage therefore has to stay below that broad
  demo-flat reopening rather than relanding either of the clause-`4` halves
  again.
- Any remaining mismatch-`0` leverage also has to stay below the whole-tier
  parent exact-bound decision. The next repair has to live one layer deeper,
  inside the existing clause-`4` / clause-`5` split or remaining-one exact
  summary behavior on those live mismatch-`0` pairings.
- Any remaining mismatch-`0` leverage is therefore no longer on a
  connectivity-only relocalization of either clause-`4` half. The next honest
  slice should move below the whole `claim_next_bridge`-half exact-summary
  tradeoff and below its two active clause-`5`
  `claim_flat_codomain / reference` subcells before trying any broader
  clause-`5` family reopening.
- Any remaining mismatch-`0` leverage is now also no longer on choosing among
  the six live clause-`0` / clause-`1` pairings or between those two active
  clause-`5` families at pair-cell scope. The next honest slice should move
  one layer deeper below a single representative pair cell instead of spending
  another turn on equivalent pair-cell relands.
- That representative pair cell now also splits cleanly one layer deeper
  across its clause-`2` sheets:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_next_bridge / claim_flat_codomain`,
    the `claim_flat_domain` sheet lands `4343 / 552 / 2268`
  - the `claim_sharp_codomain` sheet lands the same `4343 / 552 / 2268`
  - each smaller tradeoff widens `small_cluster` only to
    `3141 / 522 / 522 / 0`
  - each smaller tradeoff lowers only its own clause-`2` share from `15` to
    `14`, while the sibling claim sheet stays at `15` and the reference sheet
    stays at `12`
  - each smaller tradeoff contracts the representative pair's clause-`4`
    split only from `24 / 18` to `23 / 18`
  - each smaller tradeoff contracts the representative clause-`4`
    `claim_next_bridge` plus clause-`5` `claim_flat_codomain` bucket only from
    `48` to `47`
  - the sibling `reference` sheet is a neutral control on
    `4331 / 553 / 2271`
- Any remaining mismatch-`0` leverage below that representative pair cell is
  therefore no longer on clause-`2` sheet identity either. The next honest
  slice should move below one representative claim-side clause-`2` sheet into
  a finer remaining-one exact-summary partition before considering the smaller
  claim-safe mismatch-`1` tier.
- That representative `claim_flat_domain` clause-`2` sheet is now also
  exhausted at clause-`6` identity scope:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / claim_flat_codomain`,
    the clause-`6` `claim_next_codomain`, `claim_sharp_codomain`, and
    `reference` continuations all land the same matched smaller tradeoff shell
    at `4343 / 552`
  - each keeps `small_cluster` at `3141 / 522 / 522 / 0`
  - each keeps the representative clause-`2` spread at `14 / 15 / 12`
  - each keeps the representative clause-`4` split at `23 / 18`
  - each keeps the representative clause-`4` `claim_next_bridge` plus
    clause-`5` `claim_flat_codomain` bucket at `47`
  - they differ only in the deeper zero-admitted tail:
    `claim_next_codomain = 2270`, `claim_sharp_codomain = 2269`,
    `reference = 2268`
- Any remaining mismatch-`0` leverage below that representative claim-flat
  sheet is therefore no longer on clause-`6` identity either; the later
  claim-flat probe followed the marginally best clause-`6` `reference`
  continuation before the mismatch-`0` slice moved sideways.
- That marginally best clause-`6` `reference` continuation is now also
  exhausted one layer deeper at clause-`3` identity scope:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain /
    claim_next_bridge / claim_flat_codomain / reference`,
    the `claim_flat_argument` and `claim_eventual_argument` clause-`3`
    branches are each individually neutral controls on
    `4331 / 553 / 2271`
  - each keeps `small_cluster` at `3132 / 522 / 522 / 0`
  - each keeps first-mismatch counts at `312 / 177 / 50 / 14`
  - the broader clause-`6` `reference` `4343 / 552 / 2268` tradeoff only
    appears when both clause-`3` branches reopen together
- Any remaining mismatch-`0` leverage below that marginally best clause-`6`
  `reference` continuation is therefore no longer on individual clause-`3`
  identity either; the later claim-flat probe moved below those two
  clause-`3` branches into the finer joint-continuation shell.
- That joint clause-`3` continuation is now also localized one layer deeper:
  - relative to either individually neutral clause-`3` branch, the broader
    clause-`6` `reference` tradeoff removes exactly one remaining-two parent
    capture under
    `claim_eventual_domain / claim_next_codomain / claim_flat_domain / claim_next_bridge / claim_flat_codomain`
  - its entire zero-admitted delta is exactly the three sibling remaining-one
    clause-`6` continuations below that same parent:
    `claim_next_codomain`, `claim_sharp_codomain`, and `reference`
  - it introduces no new captures or remaining-one prunes elsewhere
- Any remaining mismatch-`0` leverage below that joint clause-`3`
  continuation is therefore no longer on another whole-joint-continuation
  reopen either; the later claim-flat probe partitioned that single
  remaining-two parent's remaining-one completion / terminal subcells
  instead.
- That representative `claim_flat_domain` parent/child shell is now also
  exhausted at child completion-summary scope:
  - across both clause-`3` branches and all three clause-`6` child labels,
    the six remaining-one child continuations all collapse to the same dead
    `3`-generated / `0`-admitted completion summary
  - each keeps no bound, no best-rank profile, and no survivor sketch
  - each exposes only the same three local terminal choices:
    `reference`, `eventual_lift`, and `next_lift`
  - each of those three choices is still
    `KeepWithoutFallback` plus locally `open_band_structural`, but none passes
    live connectivity on the current claim path
- Any remaining mismatch-`0` leverage is therefore no longer on the
  representative claim-flat clause-`3` or clause-`6` child identity either,
  nor on a hidden live terminal pocket inside that claim-flat shell.
- The representative `claim_sharp_codomain` clause-`2` sheet is now also
  exhausted at clause-`6` identity scope:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain / claim_next_bridge / claim_flat_codomain`,
    the clause-`6` `claim_next_codomain`, `claim_sharp_codomain`, and
    `reference` continuations all reland the same matched smaller tradeoff
    shell at `4343 / 552`
  - each keeps `small_cluster` at `3141 / 522 / 522 / 0`
  - each keeps the representative clause-`2` spread at `15 / 14 / 12`
  - each keeps the representative clause-`4` split at `23 / 18`
  - each keeps the representative clause-`4` `claim_next_bridge` plus
    clause-`5` `claim_flat_codomain` bucket at `47`
  - they differ only in the deeper zero-admitted tail:
    `claim_next_codomain = 2270`, `claim_sharp_codomain = 2269`,
    `reference = 2268`
- Any remaining mismatch-`0` leverage is therefore no longer on either
  representative claim-side clause-`6` identity surface, nor on another
  mismatch-`0` sheet-identity reland below the current representative pair.
- The next honest slice should therefore move off mismatch-`0` identity
  partitions and onto the smaller claim-safe mismatch-`1` tier before the
  `reference / reference` tails, unless a narrower reason-level connectivity
  partition under one frozen representative claim-side shell is explicitly
  promoted.
- That representative claim-safe mismatch-`1` clause-`4`
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
- That representative claim-safe mismatch-`1` clause-`4`
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
- Any remaining claim-safe mismatch-`1` leverage is therefore no longer on
  choosing between the `claim_next_codomain` and `claim_sharp_codomain`
  pairings or between the clause-`4`
  `demo_sharp_codomain / demo_sharp_bridge` sides below that representative
  claim-safe exact pair scope. The next honest slice should move below one
  representative claim-safe exact pair cell instead of reopening another
  clause-`4` sibling or pair-cell reland.
- The chosen representative claim-safe mismatch-`1` exact pair cell on the
  clause-`4` `demo_sharp_codomain` side is now also localized one layer deeper
  across its clause-`2` sheets:
  - under `reference / claim_next_codomain`, `claim_flat_domain` and
    `claim_sharp_codomain` each land the same smaller negative-control shell at
    `4347 / 555 / 2277`
  - each raises only its own clause-`2` share from `15` to `17`, while the
    sibling claim sheet stays at `15` and the reference sheet stays at `12`
  - each keeps the incumbent clause-`4`
    `claim_next_bridge / reference` split at `24 / 18`
  - each adds only a tiny clause-`4` `demo_sharp_codomain` pocket of `2`
  - the sibling `reference` clause-`2` sheet is a neutral search control on
    `4331 / 553 / 2271`
  - the claim-side sheets lift `small_cluster` generated candidates only to
    `3144`, while the reference sheet stays at `3132`
- The search/connectivity split on that representative exact pair cell is now
  explicit:
  - a hand-built reference-sheet telescope still passes connectivity under the
    same override
  - live search stays neutral there because only claim clause-`2` prefixes
    expose the anchor-`11` exact-argument pocket on the current claim lane
- Any remaining claim-safe mismatch-`1` leverage is therefore no longer on
  clause-`2` sheet identity inside that representative exact pair cell. The
  next honest slice should move below one representative claim-side clause-`2`
  sheet instead of reopening the sibling claim sheet, the reference sheet, or
  another exact pair cell first.
- The two representative claim-side sheets under that chosen exact pair cell
  are now also localized one layer deeper across their clause-`5` / clause-`6`
  shells:
  - on both `claim_flat_domain` and `claim_sharp_codomain`, the
    clause-`5` `claim_flat_codomain` and `claim_next_codomain` prefixes each
    combine with all three clause-`6` labels
    `claim_next_codomain / claim_sharp_codomain / reference`
  - each of those six resulting prefixes is captured exactly once on the same
    `4347 / 555` outer shell
  - every one of the six prefixes sits at matched-clause count `2`
  - every terminal continuation on every prefix is `NeedsFallback`
    (`3` terminal candidates each)
- The representative claim-safe `claim_flat_domain` dead shell under that same
  chosen exact pair is now also localized one layer deeper at terminal-family
  scope:
  - all six clause-`5` / clause-`6` continuations
    `claim_flat_codomain / claim_next_codomain` x
    `claim_next_codomain / claim_sharp_codomain / reference`
    reland the same dead `3`-generated / `0`-admitted completion profile
  - every one of those six dead prefixes still exposes the same three terminal
    families only:
    `reference / eventual_lift / next_lift`
  - every one of those terminal families still classifies only as
    `NeedsFallback`
  - every one of those completed telescopes remains structurally connected but
    unqualified on the current claim path with
    `connected = true`, `references_active_window = false`,
    `self_contained = false`, `max_lib_ref = 11`, and
    `historical_reanchor = false`
- The promoted reason-level pass below that representative dead prefix's
  `reference` terminal is now also localized:
  - under the selected exact claim-safe pair, the completed `reference`
    terminal reaches `5` matched clauses on the exact-pair reanchor path and
    first fails at clause `5`
  - the same `5`-matched / first-mismatch-`5` blocker also holds on the
    sibling `eventual_lift` and `next_lift` terminals
  - the same blocker also holds across all six dead prefixes in that shell
  - the exact claim-safe pair control still reaches the full `8`-clause match
    with no mismatch
- Any remaining claim-safe mismatch-`1` leverage is therefore no longer on
  claim-side clause-`6` or terminal-family identity below that representative
  exact pair cell either. The next honest slice should stay below the now
  uniform clause-`5` reason wall rather than reopening another claim-side
  sheet, clause-`6`, or terminal-family sibling.
- Do not spend another turn on clause-`5` tail reopenings or exact
  remaining-two clause-`5` bridge-slice reopenings or on another exact
  claim-flat / claim-sharp single-sheet restatement. Those surfaces are
  already exhausted or explicitly ruled out.

## Do This Next

1. Start from the stored `v12` certificate, compare report, benchmark bundle,
   and `reports/steps/step-15-live.ndjson`, but treat the exact-claim mismatch-`1`
   tradeoff ladder as already frozen at `57 -> 51 -> 45`.
2. Probe only surfaces outside the exact claim-pair mismatch-`1`
   `reference + demo_flat_codomain` branch, and within the smaller claim-safe
   mismatch-`1` tier stay outside any reland that only swaps exact pair cell,
   clause-`4` side sibling, or clause-`2` sheet identity.
3. Prefer the smaller claim-safe mismatch-`1` tier next:
   - total remaining-two pressure there is `84`
   - keep the mismatch-`0` claim-domain surface frozen as exhausted through
     the whole `claim_next_bridge`-half tradeoff, its active clause-`5`
     cells, the representative pair-cell clause-`2` split, and the matched
     representative claim-side clause-`6` identity passes
   - do not reopen the broad mismatch-`0` negative controls again:
     whole-tier parent `CannotClearBar -> Unknown` relief, broad clause-`1`
     `demo_flat_codomain`, or either clause-`4`
     `claim_next_bridge / reference` relocalization
   - do not reopen the mismatch-`0` remaining-one exact-summary tradeoff
     cells again either: the whole `claim_next_bridge` half, the active
     clause-`5` `claim_flat_codomain / reference` cells, the symmetric
     pair-cell relands, the representative claim-flat joint clause-`3`
     shell, or the representative claim-sharp clause-`6` identity relands
   - treat the representative `claim_flat_domain` and
     `claim_sharp_codomain` clause-`2` sheets as frozen smaller tradeoffs at
     `4343 / 552 / 2268`
   - treat both representative claim-side clause-`6` identity passes as
     frozen matched smaller tradeoff shells at `4343 / 552`, differing only
     by the tiny deeper zero-admitted tail at `2270 / 2269 / 2268`
   - treat the representative claim-safe mismatch-`1` clause-`4`
     `demo_sharp_codomain` and `demo_sharp_bridge` sides' exact pair cells as
     frozen matched smaller negative controls at `4363 / 557 / 2283`
   - on either representative claim-safe clause-`4` side, each exact
     pair-cell reland only raises its targeted pairing from `42` to `46`,
     keeps the incumbent clause-`4` `24 / 18` split, and adds only a tiny
     `demo_sharp_codomain = 4` or `demo_sharp_bridge = 4` side pocket while
     leaving the sibling pairing at `42`
   - inside the chosen representative
     `reference / claim_next_codomain / demo_sharp_codomain` cell, treat the
     `claim_flat_domain` and `claim_sharp_codomain` clause-`2` sheets as
     frozen matched smaller negative controls at `4347 / 555 / 2277`, and the
     sibling `reference` sheet as a neutral search control at
     `4331 / 553 / 2271`
   - because only claim clause-`2` prefixes expose the anchor-`11`
     exact-argument pocket there, do not promote the reference sheet from the
     connectivity witness into the next live search slice
   - inside the chosen representative claim-side sheets, treat the
     clause-`5` `claim_flat_codomain / claim_next_codomain` and
     clause-`6` `claim_next_codomain / claim_sharp_codomain / reference`
     identities as frozen matched dead shells too; every one of those six
     prefixes still has only `NeedsFallback` terminals
   - if the claim-safe mismatch-`1` tier is revisited next, stay below the
     representative dead prefix's now-uniform clause-`5` reason wall at
     `reference / claim_next_codomain / claim_flat_domain /
     demo_sharp_codomain / claim_flat_codomain / reference`;
     use the already-exhausted `eventual_lift` and `next_lift` terminals plus
     the other five dead prefixes in that shell only as matched controls, and
     partition by narrower clause-`5` qualification behavior rather than by
     another clause-`6` or terminal-family reland
   - if mismatch-`0` is reopened at all, do so only through a newly promoted
     reason-level connectivity partition beneath one frozen representative
     claim-side shell; otherwise leave all `12` live pair cells alone
   - leave the `reference / reference` tails at `54` below the claim-safe
     mismatch-`1` tier unless a narrower probe changes the ordering
4. Land only a change that improves generated breadth while preserving all of
   the following:
   - accepted step `15` winner stays canonical `103 / 8`
   - the isolated `single` pocket stays fenced
   - `small_cluster` is no worse than `3132 / 522 / 522 / 0`
   - stronger-than-canonical lifted `89 / 8` terminals stay fenced

## Keep Green

- `current_claim_step_fifteen_partial_prefix_wall_stays_on_four_early_temporal_prefix_families`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_stays_on_nine_clause_zero_one_pairings`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_sits_on_claim_next_bridge_and_reference_clause_four_families`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_four_pressure_on_claim_next_bridge_per_clause_zero_one_pairing`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_two_pressure_on_claim_variants_under_the_live_clause_zero_one_pairs`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_outside_reference_demo_flat_tradeoff_ladder_stays_on_ten_off_branch_pairings`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_outside_reference_demo_flat_tradeoff_ladder_still_prioritizes_mismatch_zero_claim_domain_surface`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_stays_a_tradeoff_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_variant_pair_stays_a_tradeoff_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_flat_sheet_stays_a_smaller_tradeoff_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_sharp_sheet_stays_a_smaller_tradeoff_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_claim_variant_pair_stays_a_tradeoff_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_neutral_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_tradeoff_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_tradeoff_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_mismatch_zero_claim_domain_surface_stays_a_negative_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_mismatch_zero_claim_domain_clause_four_claim_next_bridge_side_stays_a_smaller_negative_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_mismatch_zero_claim_domain_clause_four_reference_side_stays_a_negative_control`
- `current_claim_step_fifteen_clause_four_sharp_codomain_on_exact_claim_safe_pair_cells_stay_matched_smaller_negative_controls`
- `current_claim_step_fifteen_clause_four_sharp_bridge_on_exact_claim_safe_pair_cells_stay_matched_smaller_negative_controls`
- `connectivity_accepts_clause_four_demo_sharp_codomain_on_exact_claim_safe_pair_under_override`
- `connectivity_keeps_clause_four_demo_sharp_codomain_on_sibling_claim_safe_pair_closed_even_under_override`
- `connectivity_keeps_clause_four_demo_sharp_codomain_on_exact_claim_safe_pair_reference_terminal_only_even_under_override`
- `current_claim_step_fifteen_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_clause_two_sheets_split_into_two_claim_side_smaller_negative_controls_plus_one_reference_neutral_control`
- `current_claim_step_fifteen_clause_four_demo_sharp_codomain_on_representative_claim_safe_claim_side_sheets_localize_to_matching_dead_clause_five_six_shells`
- `current_claim_step_fifteen_representative_claim_safe_dead_prefixes_stay_on_six_matched_dead_completion_summaries`
- `current_claim_step_fifteen_representative_claim_safe_dead_prefixes_keep_only_uniform_nonlive_terminal_families`
- `current_claim_step_fifteen_representative_claim_safe_dead_prefix_reason_progress_stays_uniformly_blocked_at_clause_five`
- `current_claim_step_fifteen_claim_clause_two_prefixes_expose_only_anchor_eleven_exact_argument_pocket`
- `connectivity_accepts_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_claim_clause_two_under_override`
- `connectivity_keeps_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_sibling_claim_clause_two_closed_even_under_override`
- `connectivity_accepts_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_reference_clause_two_under_override`
- `connectivity_tracks_claim_safe_pair_reason_progress_below_the_representative_dead_prefix`
- `connectivity_keeps_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_claim_clause_two_reference_terminal_only_even_under_override`
- `connectivity_accepts_clause_four_demo_sharp_bridge_on_exact_claim_safe_pair_under_override`
- `connectivity_keeps_clause_four_demo_sharp_bridge_on_sibling_claim_safe_pair_closed_even_under_override`
- `connectivity_keeps_clause_four_demo_sharp_bridge_on_exact_claim_safe_pair_reference_terminal_only_even_under_override`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_mismatch_zero_reference_tail_stays_a_negative_control`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_mismatch_zero_claim_next_bridge_half_stays_a_tradeoff_control`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_mismatch_zero_claim_next_bridge_clause_five_claim_flat_codomain_stays_a_tradeoff_control`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_mismatch_zero_claim_next_bridge_clause_five_claim_next_codomain_stays_a_neutral_control`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_mismatch_zero_claim_next_bridge_clause_five_reference_stays_a_tradeoff_control`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_pair_cells_below_the_active_mismatch_zero_clause_five_cells_stay_uniform_smaller_tradeoff_controls`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_a_representative_mismatch_zero_pair_cell_stays_a_smaller_tradeoff_control`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_representative_mismatch_zero_pair_cell_clause_two_sheets_split_into_two_claim_side_smaller_tradeoff_controls_plus_one_reference_neutral_control`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_representative_mismatch_zero_claim_flat_sheet_clause_six_sheets_stay_matched_smaller_tradeoff_controls`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_representative_mismatch_zero_claim_sharp_sheet_clause_six_sheets_stay_matched_smaller_tradeoff_controls`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_representative_mismatch_zero_claim_flat_sheet_clause_six_reference_clause_three_sheets_stay_individually_neutral_controls`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_representative_mismatch_zero_claim_flat_sheet_clause_six_reference_tradeoff_delta_below_joint_clause_three_continuation_probe`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_below_the_representative_claim_flat_joint_clause_three_shell_stays_on_six_matched_dead_completion_summaries`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_below_the_representative_claim_flat_joint_clause_three_shell_keeps_only_uniform_nonlive_open_band_terminal_choices`
- `current_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice_stays_a_negative_control`
- `current_claim_step_fifteen_clause_five_remaining_two_mismatch_one_bridge_slice_stays_a_negative_control`
- `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`
- `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
- the matching connectivity override tests for any touched override slice

## Not The Next Move

- another rerun setup pass first
- a step-`1`-first theory slice
- clause-`5` `reference` / `claim_flat_codomain` / `claim_next_codomain`
  tail reopenings
- exact remaining-two mismatch-`0` or mismatch-`1` clause-`5`
  bridge-slice reopenings
- another exact claim-pair clause-`4` `reference`-side relocalization pass
- another exact claim-flat or claim-sharp single-sheet relocalization pass on
  the clause-`4` `claim_next_bridge` half
- another exact claim-safe mismatch-`1` pair-cell reland on the
  representative clause-`4` `demo_sharp_codomain` or
  `demo_sharp_bridge` side for either the
  `reference / claim_next_codomain` or
  `reference / claim_sharp_codomain` pairing
- another representative claim-safe exact-pair clause-`2` identity pass on the
  `claim_flat_domain`, `claim_sharp_codomain`, or `reference` sheet under
  `reference / claim_next_codomain / demo_sharp_codomain`
- another representative claim-safe claim-side clause-`5` / clause-`6`
  identity pass inside
  `reference / claim_next_codomain / demo_sharp_codomain`
- another representative claim-safe dead-prefix terminal-family reland under
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`
- another representative claim-safe dead-prefix clause-`6` or terminal-family
  reason reland after the exact-pair blocker has already been localized
  uniformly at clause `5`
- broad clause-`1` `demo_flat_codomain` reopenings across the full
  mismatch-`0` claim-domain surface
- another connectivity-only relocalization of the mismatch-`0` clause-`4`
  `claim_next_bridge` half
- another connectivity-only relocalization of the mismatch-`0` clause-`4`
  `reference` half
- another whole-half remaining-one exact-summary relief pass on the
  mismatch-`0` clause-`4` `claim_next_bridge` half
- another whole-cell remaining-one exact-summary relief pass on the
  mismatch-`0` clause-`4` `claim_next_bridge` plus clause-`5`
  `claim_flat_codomain` cell
- another whole-cell remaining-one exact-summary relief pass on the
  mismatch-`0` clause-`4` `claim_next_bridge` plus clause-`5`
  `reference` cell
- another pair-cell remaining-one exact-summary relief pass on any of the
  `12` mismatch-`0` live clause-`0` / clause-`1` cells under those two active
  clause-`5` families
- another representative mismatch-`0` pair-cell clause-`2` identity pass on
  the `claim_flat_domain`, `claim_sharp_codomain`, or `reference` sheet under
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_flat_codomain`
- another representative `claim_flat_domain` clause-`2` clause-`6` identity
  pass on the `claim_next_codomain`, `claim_sharp_codomain`, or `reference`
  continuation under `claim_eventual_domain / claim_next_codomain /
  claim_flat_domain / claim_next_bridge / claim_flat_codomain`
- another representative `claim_sharp_codomain` clause-`2` clause-`6`
  identity pass on the `claim_next_codomain`, `claim_sharp_codomain`, or
  `reference` continuation under `claim_eventual_domain /
  claim_next_codomain / claim_sharp_codomain / claim_next_bridge /
  claim_flat_codomain`
- another representative `claim_flat_domain` clause-`2` marginally best
  clause-`6` `reference` clause-`3` identity pass on the
  `claim_flat_argument` or `claim_eventual_argument` branch under
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain /
  claim_next_bridge / claim_flat_codomain / reference`
- another whole joint clause-`3` continuation reopen under that same
  representative clause-`6` `reference` shell
- another representative claim-flat parent/child-shell completion pass that
  only rechecks the same six dead `3`-generated / `0`-admitted child
  continuations or re-partitions them by clause-`3` / clause-`6` identity
- another whole-cell remaining-one exact-summary relief pass on the
  mismatch-`0` clause-`4` `claim_next_bridge` plus clause-`5`
  `claim_next_codomain` cell
- another remaining-one exact-summary relief pass on the mismatch-`0`
  clause-`4` `reference` plus clause-`5` `reference` tail
- whole-tier remaining-two mismatch-`0` claim-domain
  `CannotClearBar -> Unknown` relief at the parent
  `exact_partial_prefix_bound_decision(...)` layer
- broader clause-`4` `reference`-sheet reopenings
- claim-safe clause-`4` or clause-`5` reopenings
- raw clause-`3` `anchor-11` widening
- blanket, exact-family, or subset-local same-primary relief

## Success For This Slice

- the clean step-`15` partial-prefix wall is narrower than `552`
- the canonical accepted path stays fixed through step `15`
- the isolated `single` pocket and unsafe lifted shell both stay fenced
- a new clean full-profile rerun beyond `v12` is justified and ready to launch
