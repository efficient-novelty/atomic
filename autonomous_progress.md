# Autonomous Claim Lane State

Last updated: 2026-04-08

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
- The residual `3` `single`-bucket incumbent prunes remain fenced and are now
  secondary to the partial-prefix wall.

## Current Operating Position

- Keep wording at `bounded live recovery`.
- Keep step `1` explicit, but do not reopen it first unless a newer stored
  rerun changes the diagnosis.
- When working outside the exhausted mismatch-`1`
  `reference + demo_flat_codomain` ladder, start with the mismatch-`0`
  claim-domain surface before the smaller claim-safe mismatch-`1` tier or the
  `reference / reference` tails.
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
- Do not reopen the exact claim-flat or claim-sharp single-sheet splits on the
  clause-`4` `claim_next_bridge` half first; they are now smaller tradeoff
  controls rather than the landed repair.
- After a real local repair lands, immediately rerun beyond `v12` and refresh
  compare, benchmark, and certification.
