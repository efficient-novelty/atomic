# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-10

This file owns the single active work order for `desktop_claim_shadow`.

## Contract

- Keep exactly one active objective and one current hypothesis here.
- Move completed or ruled-out probes to
  [autonomous_ledger.md](autonomous_ledger.md).
- Do not restate medium-horizon phases or historical context here. Use
  [autonomous_plan.md](autonomous_plan.md) and
  [autonomous_progress.md](autonomous_progress.md).

## Objective

Keep the active work order on the rerun-confirmed post-local-probe
step-`15` miss: use canonical `v13` as the stored baseline for the next
code-side step-`15` repair search above the exhausted mismatch-`0`
remaining-one exact-summary lattice, above both representative mismatch-`0`
claim-side parent-route probes, and above their narrower clause-`6`
`reference` refinements, and above the matched representative mismatch-`0`
claim-side active-window and self-contained qualification families on those
same active clause-`5` buckets, above their narrower clause-`6`
`reference` refinements too, and above the representative claim-flat
clause-`3` `claim_flat_argument / claim_eventual_argument` refinements too.
The scoped historical-reanchor route on each
active clause-`5` `claim_flat_codomain / reference` family qualified its
targeted claim-side parent shell, yet the broad full surfaces relanded unsafe
noncanonical `60 / 8` at `4427 / 545 / 2247` with
`incumbent_dominance = 117`,
`small_cluster = 2931 / 455 / 455 / 115`, and a reopened `single` bucket,
while narrowing that same route to clause-`6` `reference` only relanded a
different unsafe noncanonical `74 / 8` at the same `4427 / 545 / 2247` with
`incumbent_dominance = 111`,
`small_cluster = 2904 / 430 / 430 / 108`, and a still-fenced `single`
bucket. The first alternate active-window qualification family on those same
active clause-`5` buckets also qualifies the targeted claim-side parent
shell, yet both broad active-window surfaces reland a different unsafe
matched control on the same noncanonical `60 / 8` and the same
`4427 / 545 / 2247`, now with `incumbent_dominance = 110`,
`small_cluster = 2952 / 558 / 558 / 108`, and the same reopened `single`
bucket. The next alternate self-contained qualification family on those same
active clause-`5` buckets also qualifies the targeted claim-side parent
shell, yet both broad self-contained surfaces reland that same unsafe matched
control with the same noncanonical `60 / 8`, the same
`4427 / 545 / 2247`, `incumbent_dominance = 110`,
`small_cluster = 2952 / 558 / 558 / 108`, and the same reopened `single`
bucket. Narrowing those same alternate active-window and self-contained
families to clause-`6` `reference` also stays unsafe rather than exposing a
repair: on `claim_flat_codomain`, both refinements reland noncanonical
`60 / 8` with `retained = 4`, the same `4427 / 545 / 2247`,
`incumbent_dominance = 113`, and
`small_cluster = 2904 / 462 / 462 / 109 / 2` with best overshoot
`545 / 5278`; on `reference`, both refinements reland the same
noncanonical `60 / 8` with `retained = 2`, the same `4427 / 545 / 2247`,
`incumbent_dominance = 115`, and the same
`2904 / 462 / 462 / 109 / 2` `small_cluster`, still with no `single`
bucket. The parent-route plus active-window plus self-contained family is
therefore frozen as an unsafe negative-control lattice, and their narrower
alternate clause-`6` refinements are frozen as a split unsafe control rather
than the landed repair. Keep the demoted claim-safe mismatch-`1`
checkpoint, both representative
mismatch-`0` dead-child checkpoints, the spent `reference / reference` tail,
the isolated clause-`1` `demo_flat_codomain` exact-suffix side pocket, the
reconstructive representative mismatch-`0` claim-side clause-`6`
`reference` union under `claim_next_bridge / claim_flat_codomain`, both
representative claim-side parent-route probes, their narrower clause-`6`
`reference` refinements, the matched representative claim-side active-window
and self-contained families, their narrower clause-`6` `reference`
refinements too, and the representative claim-flat clause-`3` refinements
inside that same parent-route family, plus the looser representative
claim-safe backup frozen as controls rather than fresh leads.

Do not spend another turn re-running the rerun-vs-step-`1` decision on the
same code. The newer stored rerun has already shown that current-head code
reproduces the same breadth-only miss, so step `1` stays deferred unless a
later stored bundle changes the diagnosis.

## Start From

- Canonical stored bundle:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13`
- Earlier stored comparison points:
  - `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v12`
  - `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v11`
- Stored breadth blockers:
  - step `1 = 546 / 2144`
  - step `15 = 4331 / 5000`
- Stored cross-rerun step-`1` stability:
  - `v11 = 546 / 2144`
  - `v12 = 546 / 2144`
  - `v13 = 546 / 2144`
  - all three keep step-`01` at `generated = 546`, `well_formed = 288`,
    `admitted = 1`, `legality_connectivity_exact_rejection = 435`
- Stored cross-rerun step-`15` stability on the newer codebase:
  - `v12 = 4331 / 5000`
  - `v13 = 4331 / 5000`
  - both keep step-`15` at `generated = 4331`,
    `partial_prefix_bar_failure = 553`, `incumbent_dominance = 3`, and
    `small_cluster generated = 3132`
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
- Stored late-surface movement between reruns:
  - `v11`: `generated = 3972`, `partial_prefix_bar_failure = 468`,
    `incumbent_dominance = 242`, `small_cluster generated = 2190`
  - `v12`: `generated = 4331`, `partial_prefix_bar_failure = 553`,
    `incumbent_dominance = 3`, `small_cluster generated = 3132`
  - `v13`: `generated = 4331`, `partial_prefix_bar_failure = 553`,
    `incumbent_dominance = 3`, `small_cluster generated = 3132`

## Active Hypothesis

- The rerun-backed step-`15` reset is now confirmed on the current
  post-probe codebase:
  - `v11`, `v12`, and `v13` keep the same step-`1` fail at `546 / 2144`
    with the same step-`01` surface
  - `v12` and `v13` also keep the same step-`15` fail at `4331 / 5000`
    with the same canonical `553` / `3` / `3132` late surface
  - `v13` keeps accepted parity, claim policy honesty, narrative coverage,
    compare readiness, benchmark freshness, and runtime-threshold pass on the
    newer build commit
- The rerun-vs-step-`1` ordering is therefore closed on current-head code:
  step `15` remains the active stored miss, step `1` remains secondary, and
  another same-code rerun or fallback-decision pass would add no new
  evidence.
- The newer stored rerun has now done its job: it confirmed the step-`15`
  reset rather than changing the diagnosis enough to justify reopening step
  `1`.
- The first fresh post-rerun code-side repair probe on the mismatch-`0`
  exact-summary ladder is now also spent:
  - reopening both representative claim-side clause-`2` sheets together, but
    only beneath the marginally best clause-`6` `reference` continuation on
    the active `claim_next_bridge / claim_flat_codomain` cell, lands
    `4355 / 551 / 2265`
  - it widens `small_cluster` to `3150 / 522 / 522 / 0`
  - first-mismatch-`0` contracts from `312` to `310`
  - the representative clause-`2` spread becomes `14 / 14 / 12`
  - the representative clause-`4` split contracts from `24 / 18` to
    `22 / 18`
  - the active clause-`4` `claim_next_bridge` plus clause-`5`
    `claim_flat_codomain` bucket contracts from `48` to `46`
- Any remaining step-`15` leverage is therefore no longer on reassembling the
  representative mismatch-`0` claim-side clause-`2` sheets beneath that
  marginally best clause-`6` `reference` continuation either; that deeper
  union just reconstructs the existing `4355 / 551 / 2265` pair-cell smaller
  tradeoff rather than isolating a safer repair.
- That reconstructive claim-side union is now also pinned to the same
  zero-admitted remaining-one dead surface as the representative pair-cell
  tradeoff:
  - exact-prune family summary stays on
    `captured_prefixes = 2265`, `cached_bound_count = 0`, and the single
    family `((0, None, None), 2265)`
  - terminal-connectivity summary stays on `6795` generated candidates, all
    `NeedsFallback`, all structurally connected but unqualified, and none on
    historical reanchor
- That reconstructive claim-side union's `small_cluster` widening is now also
  fully localized:
  - the widened `3150` shell is exactly `+18` generated candidates from six
    released remaining-one groups
  - those groups are the two claim-side clause-`2` sheets
    `claim_flat_domain / claim_sharp_codomain` crossed with the three
    clause-`6` labels `claim_next_codomain / claim_sharp_codomain / reference`
  - every released group is the same dead `small_cluster` shell:
    `3` generated, `0` admitted, no bound, cannot clear bar, and only
    `NeedsFallback`
- Any remaining step-`15` leverage is therefore no longer inside that
  representative mismatch-`0` remaining-one exact-summary lattice either; the
  next repair class has to move above those dead-shell relands instead of
  reopening another deeper remaining-one tradeoff.
- The next repair class therefore has to live above that remaining-one
  exact-summary lattice:
  - the next honest probe should target the parent-level route or
  qualification that releases those six groups
  - the goal is to preserve the current wall gain without reopening that same
    `+18` `small_cluster` shell
- The first parent-level route probe above that lattice is now also spent as
  an unsafe negative control:
  - the scoped representative mismatch-`0` claim-side historical-reanchor
    route does qualify the targeted claim-side clause-`2` / clause-`3` /
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
- Any remaining step-`15` leverage is therefore no longer on that
  representative mismatch-`0` claim-side parent-route reopening either; it
  narrows the wall only by displacing the canonical `103 / 8` winner and by
  reshaping both survivor buckets.
- The sibling active clause-`5` `reference` family's representative
  mismatch-`0` claim-side parent-route probe is now also spent as the same
  unsafe negative control:
  - it relands the same noncanonical `60 / 8` winner, the same
    `4427 / 545 / 2247` late surface, and the same
    `incumbent_dominance = 117`
  - it relands the same contracted first-mismatch-`0` tier at `304`
  - it relands the same `small_cluster = 2931 / 455 / 455 / 115`
  - it relands the same reopened `single` bucket
  - it relands the same representative clause-`2` spread `11 / 11 / 12`
  - it relands the same representative clause-`4` split `20 / 14`
  - it contracts only the sibling active clause-`4` `claim_next_bridge` plus
    clause-`5` `reference` bucket to `44`
- Any remaining step-`15` leverage is therefore no longer on representative
  mismatch-`0` claim-side parent-route identity across either active
  clause-`5` family. The next honest probe has to move to a different
  parent-level qualification above the current remaining-one lattice.
- That matched unsafe parent-route class is now also localized at delta level:
  - on the chosen active clause-`5` bucket, each route removes exactly four
    remaining-two exact-prune parent cells:
    `claim_flat_domain / claim_sharp_codomain` crossed with clause-`4`
    `claim_next_bridge / reference`
  - those four parent cells contribute exactly `8` removed zero-admitted
    captures and exactly `24` removed remaining-one pruned prefixes
  - neither route introduces any off-target exact-prune or pruned-prefix
    family
- Any remaining step-`15` leverage is therefore no longer on another
  route-identity reland or delta relocalization of those same active
  clause-`5` buckets either. The next honest probe has to swap to a different
  parent-level qualification family above the current remaining-one lattice.
- The first alternate parent-level qualification family above that same
  remaining-one lattice is now also spent across both active clause-`5`
  families:
  - a scoped representative mismatch-`0` claim-side active-window override
    does qualify the targeted claim-side parent shell while keeping the
    sibling reference clause-`2` sheet closed and lifted terminals fenced
  - on `claim_flat_codomain` and `reference`, that alternate family still
    relands unsafe noncanonical `60 / 8` with `retained = 2`,
    `generated = 4427`, `partial_prefix_bar_failure = 545`,
    `incumbent_dominance = 110`, zero-admitted captures `2247`,
    first-mismatch distribution `304 / 177 / 50 / 14`, and
    `small_cluster = 2952 / 558 / 558 / 108`
  - the isolated `single` bucket reopens to `2` fully scored non-winners plus
    `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread stays `11 / 11 / 12`
  - the representative clause-`4` split stays `20 / 14`
  - it contracts only the chosen active clause-`4`
    `claim_next_bridge` plus active clause-`5` bucket to `44`
- That alternate active-window family is also already localized at the same
  delta level:
  - on the chosen active clause-`5` bucket, it removes the same four
    remaining-two exact-prune parent cells
  - it removes the same `8` zero-admitted captures and the same `24`
    remaining-one pruned prefixes
  - it introduces no off-target exact-prune or pruned-prefix family
- Any remaining step-`15` leverage is therefore no longer on the broad
  representative mismatch-`0` claim-side parent-route class or on the first
  alternate active-window qualification family across either active
  clause-`5` bucket. The next honest probe has to change parent-level
  qualification family again rather than swapping between those two matched
  controls.
- The next alternate parent-level self-contained qualification family above
  that same remaining-one lattice is now also spent across both active
  clause-`5` families:
  - a scoped representative mismatch-`0` claim-side self-contained override
    does qualify the targeted claim-side parent shell while keeping the
    sibling reference clause-`2` sheet closed and lifted terminals fenced
  - on `claim_flat_codomain` and `reference`, that alternate family relands
    the same unsafe matched control as active-window with noncanonical
    `60 / 8`, `retained = 2`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 110`,
    zero-admitted captures `2247`, first-mismatch distribution
    `304 / 177 / 50 / 14`, and
    `small_cluster = 2952 / 558 / 558 / 108`
  - the isolated `single` bucket reopens to `2` fully scored non-winners plus
    `2` residual prunes at best overshoot `545 / 5278`
  - the representative clause-`2` spread stays `11 / 11 / 12`
  - the representative clause-`4` split stays `20 / 14`
  - it contracts only the chosen active clause-`4`
    `claim_next_bridge` plus active clause-`5` bucket to `44`
- That alternate self-contained family is also already localized at the same
  delta level:
  - on the chosen active clause-`5` bucket, it removes the same four
    remaining-two exact-prune parent cells
  - it removes the same `8` zero-admitted captures and the same `24`
    remaining-one pruned prefixes
  - it introduces no off-target exact-prune or pruned-prefix family
- Any remaining step-`15` leverage is therefore no longer on the broad
  representative mismatch-`0` claim-side parent-route class, on the first
  alternate active-window qualification family, or on the matched
  self-contained qualification family across either active clause-`5`
  bucket. The next honest probe has to change parent-level qualification
  family again rather than swapping between those three matched controls.
- Narrowing those same alternate active-window and self-contained families to
  only the representative clause-`6` `reference` continuation is now also
  spent:
  - on `claim_flat_codomain`, both alternate-family refinements reland
    noncanonical `60 / 8` with `retained = 4`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 113`,
    zero-admitted captures `2247`, and
    `small_cluster = 2904 / 462 / 462 / 109 / 2`
  - on `reference`, both sibling refinements reland noncanonical `60 / 8`
    with `retained = 2`, the same `4427 / 545 / 2247`,
    `incumbent_dominance = 115`, and the same
    `2904 / 462 / 462 / 109 / 2` `small_cluster`
  - both clause-`5` siblings keep best overshoot `545 / 5278`, no `single`
    bucket, the same `304 / 177 / 50 / 14` first-mismatch distribution, the
    same `11 / 11 / 12` clause-`2` spread, the same `20 / 14` clause-`4`
    split, the same chosen active clause-`4` plus clause-`5` bucket at `44`,
    and the same four-cell plus `24`-pruned-prefix targeted delta
- Any remaining step-`15` leverage is therefore no longer on the narrower
  alternate active-window or self-contained clause-`6` `reference`
  refinements either. Those alternates no longer reland the broader
  `2952 / 558 / 558 / 108` shell, but they still only reshape survivor
  pressure inside the same targeted claim-side bucket while displacing the
  canonical `103 / 8` winner.
- Narrowing that same parent-route class to only the representative
  clause-`6` `reference` continuation is now also spent across both active
  clause-`5` families:
  - on `claim_flat_codomain` and `reference`, the narrowed route relands
    noncanonical `74 / 8` with `retained = 1`, `generated = 4427`,
    `partial_prefix_bar_failure = 545`, `incumbent_dominance = 111`,
    zero-admitted captures `2247`, and
    `small_cluster = 2904 / 430 / 430 / 108`
  - the isolated `single` bucket stays on `3` residual prunes plus `1` fully
    scored non-winner at best overshoot `19563 / 10556`
  - the representative clause-`2` spread stays `11 / 11 / 12`
  - the representative clause-`4` split stays `20 / 14`
  - it contracts only the chosen active clause-`4`
    `claim_next_bridge` plus active clause-`5` bucket to `44`
- That narrower clause-`6` `reference` parent-route refinement is also
  already localized at the same delta level:
  - on the chosen active clause-`5` bucket, it removes the same four
    remaining-two exact-prune parent cells
  - it removes the same `8` zero-admitted captures and the same `24`
    remaining-one pruned prefixes
  - it introduces no off-target exact-prune or pruned-prefix family
- Any remaining step-`15` leverage is therefore no longer on the broad
  representative mismatch-`0` claim-side parent-route class or on its
  marginally narrower clause-`6` `reference` refinement across either active
  clause-`5` family. The next honest probe has to change parent-level
  qualification family again rather than retargeting the same route with a
  narrower clause-`6` selector.
- Narrowing that same representative mismatch-`0` claim-side parent-route
  class on the active clause-`5` `claim_flat_codomain` family to only one
  clause-`3` argument branch is now also spent:
  - on either `claim_flat_argument` or `claim_eventual_argument`, the
    narrowed route relands the same smaller unsafe negative control with
    noncanonical `60 / 8`, `retained = 2`, `generated = 4379`,
    `partial_prefix_bar_failure = 549`, `incumbent_dominance = 113`,
    zero-admitted captures `2259`, and
    `small_cluster = 2871 / 435 / 435 / 111`
  - the first-mismatch distribution relands as clause `0 = 308`,
    clause `1 = 177`, clause `2 = 50`, and clause `3 = 14`
  - the isolated `single` bucket reopens to `2` fully scored non-winners plus
    `2` residual prunes at best overshoot `545 / 5278`
- Any remaining step-`15` leverage is therefore no longer on representative
  claim-flat clause-`3` identity inside that same parent-route family
  either. Even that tighter `4379 / 549 / 2259` shell still displaces the
  canonical `103 / 8` winner and reopens the `single` bucket, so the next
  honest probe has to stay above another claim-flat clause-`3` reland as well
  as above the broader route class and the clause-`6` `reference`
  refinement.
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
- That earlier ordering had promoted the smaller claim-safe mismatch-`1` tier
  ahead of the `reference / reference` tails once the representative
  mismatch-`0` claim-side clause-`6` identity passes froze on both claim
  sheets, but both of those backup surfaces are now spent and no longer
  compete with the rerun-backed fallback.
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
- The sibling active clause-`5` `reference` family's representative
  mismatch-`0` pair cell now relands the same clause-`2` split one layer
  deeper:
  - under
    `claim_eventual_domain / claim_next_codomain / claim_next_bridge / reference`,
    the `claim_flat_domain` and `claim_sharp_codomain` sheets each land
    `4343 / 552 / 2268`
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
- Any remaining mismatch-`0` leverage below that representative pair cell is
  therefore no longer on clause-`2` sheet identity either, including on the
  sibling active clause-`5` `reference` family. The next honest slice should
  move below one representative claim-side clause-`2` sheet into a finer
  remaining-one exact-summary partition before considering the smaller
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
- That representative claim-sharp shell is now also exhausted one layer deeper
  at child completion-summary and first finer reason scope:
  - beneath
    `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain /
    claim_next_bridge / claim_flat_codomain`,
    both clause-`3` branches and all three clause-`6` children collapse to the
    same dead `3`-generated / `0`-admitted completion summary
  - each keeps no bound, no best-rank profile, and no survivor sketch
  - each keeps only the same three local terminal choices:
    `reference`, `eventual_lift`, and `next_lift`
  - each of those three choices is still
    `KeepWithoutFallback` plus locally `open_band_structural`, but none passes
    live connectivity on the current claim path
  - every one of those `18` completed telescopes also relands the same finer
    reason vector:
    `matched = 2`, `first_mismatch = 2`, `connected = true`,
    `references_active_window = false`, `self_contained = false`,
    `max_lib_ref = 10`, `historical_reanchor = false`, and no live
    connectivity
- Any remaining mismatch-`0` leverage is therefore no longer on either
  representative claim-side clause-`6` identity surface, on either
  representative claim-side dead-child completion shell, or on another
  representative mismatch-`0` reason reland below the current pair.
- With the smaller claim-safe mismatch-`1` fail-fast checkpoint and the full
  `reference / reference` tail already spent too, no fresh off-branch local
  lead remains inside the currently promoted step-`15` backups.
- The next honest slice should therefore stop below another representative
  mismatch-`0` dead-shell reland and move to the first post-local-probe
  fallback decision, while still keeping the looser claim-safe shell demoted
  unless a later comparison changes the ordering again.
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
  exact pair cell either. The next honest slice should stay below the
  clause-`5` qualification wall rather than reopening another claim-side
  sheet, clause-`6`, or terminal-family sibling.
- That representative claim-safe clause-`5` wall is now also partitioned one
  layer deeper:
  - under the same selected exact pair, clause-`5` `reference` is the only
    label that keeps the full exact-pair match and historical reanchor
  - the four off-reference clause-`5` labels
    `claim_flat_codomain / claim_next_codomain / demo_sharp_domain /
    demo_flat_codomain` all stop at the same `5`-matched /
    first-mismatch-`5` blocker
  - once clause `1` has moved to `claim_next_codomain`, even the two
    demo-side clause-`5` labels no longer satisfy the clause-`5`
    side-pocket qualifier there
  - the live dead shell itself still sits only on the two claim-side dead
    labels `claim_flat_codomain / claim_next_codomain`, one copy per
    clause-`6` sibling
- Any remaining claim-safe mismatch-`1` leverage is therefore no longer on
  reopening clause-`5` `reference` as a fresh search lead or on retrying the
  demo-side clause-`5` controls inside that representative exact pair cell.
  The next honest slice should stay below the two live claim-side dead
  clause-`5` labels instead of spending another turn on clause-`6` or
  terminal-family siblings.
- The live strategic question is now narrower than another label reland:
  - the earlier reason-level pass already reproduced the same
    `5`-matched / first-mismatch-`5` blocker across all six dead prefixes in
    that shell
  - the next claim-safe revisit should therefore ask whether any finer
    qualification reason below one live claim-side dead clause-`5` label can
    break that uniform blocker
  - if the first finer reason split simply relands the same blocker with the
    same fences, the claim-safe branch should be demoted quickly and the next
    promoted backup should come from mismatch-`0` reason-level connectivity
    below one frozen representative claim-side shell
- That first finer reason-level split is now also exhausted:
  - beneath the representative claim-safe dead shell, both live claim-side
    clause-`5` labels, all three clause-`6` siblings, and all three terminal
    families reland the same finer reason vector
  - every one of those completed telescopes stays structurally connected but
    outside active-window qualification, outside self-containedness, outside
    clause-`5` side-pocket qualification, and outside historical reanchor,
    while exact-pair progress stays fixed at `5 / mismatch 5`
  - the claim-safe branch is therefore now demoted at its first fail-fast
    checkpoint and is no longer the active next slice
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
  - the representative mismatch-`0` claim-flat backup is therefore now
    demoted at its first fail-fast checkpoint too and is no longer the active
    next slice
- The promoted `reference / reference` tail is now also localized one layer
  deeper by mismatch position and clause-`4` anatomy:
  - mismatch-`2` is the larger lead at `42`
  - mismatch-`3` is the smaller backup at `12`
  - mismatch-`2` keeps clause-`4` pressure on
    `claim_next_bridge = 18`, `reference = 16`,
    `demo_sharp_codomain = 4`, and `demo_sharp_bridge = 4`
  - mismatch-`3` stays only on the smaller
    `claim_next_bridge = 6` plus `reference = 6` tail
- Any remaining `reference / reference` leverage is therefore no longer on
  another lumped tail restatement. The later probes have now spent the whole
  mismatch-`2` clause-`4` anatomy, so the remaining live slice moved to the
  smaller mismatch-`3` backup before another broader shell could compete.
- The mismatch-`2` `reference / reference` clause-`4`
  `claim_next_bridge` half is now also exhausted as a smaller tradeoff
  control:
  - it lands `4547 / 535 / 2271`
  - it widens `small_cluster` to `3294 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - it contracts only the exact mismatch-`2` pair from `42` to `24`
  - it leaves the smaller mismatch-`3` backup untouched at `12`
- The mismatch-`2` `reference / reference` clause-`4` `reference` half is now
  also exhausted as a sharper tradeoff control:
  - it lands `4835 / 529 / 2271`
  - it widens `small_cluster` to `3492 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - it contracts the exact mismatch-`2` pair from `42` to `26`
  - it also removes the `8`-capture mismatch-`2` remaining-three spill while
    leaving the smaller mismatch-`3` backup untouched at `12`
- The mismatch-`2` `reference / reference` clause-`4`
  `demo_sharp_codomain` pocket is now also exhausted as a smaller tradeoff
  control:
  - it lands `4379 / 549 / 2271`
  - it widens `small_cluster` only to `3168 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - it contracts only the exact mismatch-`2` pair from `42` to `38`
  - it removes only the tiny `demo_sharp_codomain`
    `claim_flat_codomain / claim_next_codomain` `2 / 2` cells while leaving
    the larger mismatch-`2` `claim_next_bridge` and `reference` halves, the
    sibling `demo_sharp_bridge` pocket, and the smaller mismatch-`3` backup
    untouched
- The mismatch-`2` `reference / reference` clause-`4` `demo_sharp_bridge`
  pocket is now also exhausted as the same smaller tradeoff control:
  - it also lands `4379 / 549 / 2271`
  - it also widens `small_cluster` only to `3168 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - it also contracts only the exact mismatch-`2` pair from `42` to `38`
  - it removes only the tiny `demo_sharp_bridge`
    `claim_flat_codomain / claim_next_codomain` `2 / 2` cells while leaving
    the larger mismatch-`2` `claim_next_bridge` and `reference` halves, the
    sibling `demo_sharp_codomain` pocket, and the smaller mismatch-`3` backup
    untouched
- Any remaining mismatch-`2` leverage is therefore no longer on either of
  those two larger clause-`4` halves or either tiny demo-side pocket.
- The mismatch-`3` `reference / reference` clause-`4`
  `claim_next_bridge` half is now also exhausted as a smaller tradeoff
  control:
  - it lands `4403 / 547 / 2271`
  - it widens `small_cluster` only to `3186 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - it contracts only the exact mismatch-`3` pair from `12` to `6`
  - it leaves the larger spent mismatch-`2` tail untouched at `42`
- The mismatch-`3` `reference / reference` clause-`4` `reference` half is now
  also exhausted as a sharper tradeoff control:
  - it lands `4481 / 545 / 2271`
  - it widens `small_cluster` to `3240 / 522 / 522 / 0`
  - the isolated `single` pocket stays fenced
  - it also contracts only the exact mismatch-`3` pair from `12` to `6`
  - it also removes the `2`-capture mismatch-`3` remaining-three spill while
    leaving the larger spent mismatch-`2` tail untouched at `42`
- Any remaining `reference / reference` leverage is therefore no longer on
  either mismatch-`2` clause-`4` half, either mismatch-`2` demo-side pocket,
  or either mismatch-`3` clause-`4` half. The full tail is now spent as
  tradeoff-only control, so the next honest slice has to compare alternate
  broader backups rather than reopening the same tail again.
- That alternate broader-backup comparison is now also explicit:
  - the representative mismatch-`0` claim-side clause-`2` shell remains the
    tighter broader backup at `4343 / 552 / 2268` with
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
    promoted, and the representative claim-safe shell should not compete for
    promotion again unless that tighter mismatch-`0` backup is ruled out
    by a later broader comparison rather than by another representative
    dead-shell reland
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
- Do not spend another turn on clause-`5` tail reopenings or exact
  remaining-two clause-`5` bridge-slice reopenings or on another exact
  claim-flat / claim-sharp single-sheet restatement. Those surfaces are
  already exhausted or explicitly ruled out.

## Do This Next

1. Start from canonical `v13`:
   - use its certificate, compare, benchmark, `run.json`,
     `step-01-summary.json`, `step-15-summary.json`, and
     `step-15-live.ndjson`
   - use `v11` and `v12` only as earlier comparison points rather than as the
     current baseline
2. Treat rerun-vs-step-`1` ordering as closed on the current head:
   - `v13` reproduced the `v12` step-`1` and step-`15` breadth misses on
     newer code
   - compare, benchmark, and certification were refreshed and stayed honest
   - consequence: do not spend another turn on another same-code rerun or
     another fallback-decision restatement
3. Keep step `1` explicit but deferred:
   - do not promote a step-`1` theory slice while canonical `v13`
     still re-confirms step `15` as the active stored miss
   - only reopen step `1` if a later stored bundle changes the diagnosis
     rather than reproducing `4331 / 553` again
4. Spend the next execution turn on the first fresh code-side step-`15`
   repair above the exhausted controls:
   - keep the claim-safe mismatch-`1` clause-`5` wall, both representative
     mismatch-`0` dead shells, the spent `reference / reference` tail, the
     isolated clause-`1` `demo_flat_codomain` exact-suffix side pocket, the
     reconstructive representative mismatch-`0` claim-side clause-`6`
     `reference` union, and the looser representative claim-safe backup
     frozen as controls rather than fresh leads
   - use the known mismatch-`0` claim-`next_bridge` exact-summary tradeoff
     ladder only as geometry for choosing a new repair class; do not simply
     reland the same whole-half, whole-cell, pair-cell, or representative
     dead-shell controls verbatim
   - stay above another claim-side union reassembly inside the current
     representative pair-cell exact-summary selector lattice; the next repair
     class has to move beyond a deeper restatement of the same `4355 / 551`
     smaller tradeoff
   - keep the next repair above the current remaining-one exact-summary
     lattice on that representative mismatch-`0` shell entirely; the latest
     reconstructive control still leaves `cached_bound_count = 0`, exact-prune
     family `((0, None, None), 2265)`, and `6795` structurally connected but
     unqualified `NeedsFallback` terminal continuations
5. Treat a new local or stored candidate as promotable only if all of the
   following stay true:
   - accepted step `15` winner stays canonical `103 / 8`
   - the isolated `single` pocket stays fenced
   - `small_cluster` is no worse than `3132 / 522 / 522 / 0`
   - stronger-than-canonical lifted `89 / 8` terminals stay fenced
   - and the surfaced result improves beyond `4331` generated or `553`
     partial-prefix bar failures without worsening step `1`

## Keep Green

- `current_claim_step_fifteen_partial_prefix_wall_stays_on_four_early_temporal_prefix_families`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_stays_on_nine_clause_zero_one_pairings`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_sits_on_claim_next_bridge_and_reference_clause_four_families`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_four_pressure_on_claim_next_bridge_per_clause_zero_one_pairing`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_two_pressure_on_claim_variants_under_the_live_clause_zero_one_pairs`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_outside_reference_demo_flat_tradeoff_ladder_stays_on_ten_off_branch_pairings`
- `current_claim_step_fifteen_remaining_two_partial_prefix_wall_outside_reference_demo_flat_tradeoff_ladder_still_prioritizes_mismatch_zero_claim_domain_surface`
- `current_claim_step_fifteen_reference_reference_tail_localizes_to_mismatch_two_then_mismatch_three`
- `current_claim_step_fifteen_reference_reference_tail_keeps_clause_four_anatomy_explicit_per_mismatch_position`
- `current_claim_step_fifteen_reference_reference_tail_mismatch_two_clause_four_claim_next_bridge_half_stays_a_tradeoff_control`
- `current_claim_step_fifteen_reference_reference_tail_mismatch_two_clause_four_reference_half_stays_a_tradeoff_control`
- `current_claim_step_fifteen_reference_reference_tail_mismatch_two_clause_four_demo_sharp_codomain_pocket_stays_a_smaller_tradeoff_control`
- `current_claim_step_fifteen_reference_reference_tail_mismatch_two_clause_four_demo_sharp_bridge_pocket_stays_a_smaller_tradeoff_control`
- `current_claim_step_fifteen_reference_reference_tail_mismatch_three_clause_four_claim_next_bridge_half_stays_a_smaller_tradeoff_control`
- `current_claim_step_fifteen_reference_reference_tail_mismatch_three_clause_four_reference_half_stays_a_sharper_tradeoff_control`
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
- `current_claim_step_fifteen_representative_claim_safe_dead_prefix_clause_five_labels_stay_on_two_nonqualifying_claim_variants`
- `current_claim_step_fifteen_claim_clause_two_prefixes_expose_only_anchor_eleven_exact_argument_pocket`
- `connectivity_accepts_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_claim_clause_two_under_override`
- `connectivity_keeps_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_sibling_claim_clause_two_closed_even_under_override`
- `connectivity_accepts_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_reference_clause_two_under_override`
- `connectivity_tracks_claim_safe_pair_reason_progress_below_the_representative_dead_prefix`
- `connectivity_keeps_clause_four_demo_sharp_codomain_on_representative_claim_safe_pair_claim_clause_two_reference_terminal_only_even_under_override`
- `current_claim_step_fifteen_representative_claim_safe_first_finer_reason_split_stays_uniform_below_live_clause_five_labels`
- `connectivity_splits_representative_claim_safe_clause_five_labels_into_one_exact_pair_match_plus_four_dead_controls`
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
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_representative_mismatch_zero_reference_clause_five_pair_cell_clause_two_sheets_split_into_two_claim_side_smaller_tradeoff_controls_plus_one_reference_neutral_control`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_representative_mismatch_zero_claim_flat_sheet_clause_six_sheets_stay_matched_smaller_tradeoff_controls`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_representative_mismatch_zero_claim_sharp_sheet_clause_six_sheets_stay_matched_smaller_tradeoff_controls`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_representative_mismatch_zero_claim_flat_sheet_clause_six_reference_clause_three_sheets_stay_individually_neutral_controls`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_representative_mismatch_zero_claim_flat_sheet_clause_six_reference_tradeoff_delta_below_joint_clause_three_continuation_probe`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_below_the_representative_claim_flat_joint_clause_three_shell_stays_on_six_matched_dead_completion_summaries`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_below_the_representative_claim_flat_joint_clause_three_shell_keeps_only_uniform_nonlive_open_band_terminal_choices`
- `current_claim_step_fifteen_representative_mismatch_zero_claim_flat_dead_child_reason_progress_stays_uniformly_blocked_at_clause_two`
- `current_claim_step_fifteen_representative_mismatch_zero_claim_flat_first_finer_reason_split_stays_uniform_below_the_dead_child_shell`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_on_representative_mismatch_zero_claim_side_clause_six_reference_union_relands_the_pair_cell_tradeoff`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_below_the_representative_claim_sharp_joint_clause_three_shell_stays_on_six_matched_dead_completion_summaries`
- `current_claim_step_fifteen_remaining_one_exact_summary_relief_below_the_representative_claim_sharp_joint_clause_three_shell_keeps_only_uniform_nonlive_open_band_terminal_choices`
- `current_claim_step_fifteen_representative_mismatch_zero_claim_sharp_dead_child_reason_progress_stays_uniformly_blocked_at_clause_two`
- `current_claim_step_fifteen_representative_mismatch_zero_claim_sharp_first_finer_reason_split_stays_uniform_below_the_dead_child_shell`
- `current_claim_step_fifteen_alternate_broader_backup_comparison_keeps_the_tighter_representative_mismatch_zero_claim_side_shell_ahead_of_the_claim_safe_shell`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_exact_suffix_relief_stays_a_looser_side_pocket_control`
- `current_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice_stays_a_negative_control`
- `current_claim_step_fifteen_clause_five_remaining_two_mismatch_one_bridge_slice_stays_a_negative_control`
- `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`
- `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
- the matching connectivity override tests for any touched override slice

## Not The Next Move

- another stored-evidence-only fallback-decision pass instead of the
  rerun-backed step-`15` reset, or another same-code rerun-backed step-`15`
  reset now that canonical `v13` already closed that ordering
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
- another representative claim-safe dead-prefix clause-`5` reland on
  `reference`, `demo_sharp_domain`, or `demo_flat_codomain` inside
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`
- another finer reason-level split below that same representative claim-safe
  clause-`5` wall before a later broader-backup comparison changes the
  ordering again
- another representative claim-safe dead-prefix terminal-family reland under
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`
- another isolated clause-`1` `demo_flat_codomain` exact-suffix relief pass
  on the lone exact-suffix side pocket
- another representative claim-safe dead-prefix clause-`6` or terminal-family
  reason reland after the exact-pair blocker has already been localized
  to the clause-`5` wall and its off-reference dead controls
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
- another representative mismatch-`0` pair-cell clause-`2` identity pass on
  the `claim_flat_domain`, `claim_sharp_codomain`, or `reference` sheet under
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  reference`
- promoting the representative claim-safe claim-side clause-`2` shell ahead
  of the tighter representative mismatch-`0` claim-side shell after the
  broader-backup comparison has already settled that ordering
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
- another representative mismatch-`0` claim-side clause-`6` `reference`
  union reland under `claim_eventual_domain / claim_next_codomain /
  claim_next_bridge / claim_flat_codomain`; it now only reconstructs the
  existing `4355 / 551 / 2265` pair-cell smaller tradeoff with
  `small_cluster = 3150 / 522 / 522 / 0`
- another deeper remaining-one exact-summary reland inside that same
  representative mismatch-`0` pair-cell shell now that the latest
  reconstructive control still keeps the promoted surface on one
  zero-admitted `((0, None, None), 2265)` family, with the entire widened
  `small_cluster` shell localized to six released `3`-generated /
  `0`-admitted `NeedsFallback` groups rather than to a hidden live pocket
- another representative mismatch-`0` claim-side parent-route reland on
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_flat_codomain`; that scoped historical-reanchor route now only lands
  the unsafe `4427 / 545 / 2247` negative control with noncanonical
  `60 / 8`, `incumbent_dominance = 117`,
  `small_cluster = 2931 / 455 / 455 / 115`, a reopened `single` bucket, and a
  delta localized only to the four targeted claim-side remaining-two parent
  cells plus their `24` matching remaining-one pruned prefixes on the active
  `claim_flat_codomain` bucket
- another representative mismatch-`0` claim-side parent-route reland on
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  reference`; that sibling scoped historical-reanchor route now relands the
  same unsafe `4427 / 545 / 2247` negative control with the same noncanonical
  `60 / 8`, the same `incumbent_dominance = 117`,
  the same `small_cluster = 2931 / 455 / 455 / 115`, the same reopened
  `single` bucket, and the same targeted four-cell plus `24`-pruned-prefix
  delta on the active `reference` bucket
- another representative mismatch-`0` claim-side active-window reland on
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_flat_codomain`; that alternate qualification family now only relands
  the unsafe `4427 / 545 / 2247` shell with the same noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same targeted four-cell plus `24`-pruned-prefix delta on
  the active `claim_flat_codomain` bucket
- another representative mismatch-`0` claim-side active-window reland on
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  reference`; that sibling alternate qualification family now relands the
  same unsafe `4427 / 545 / 2247` shell with the same noncanonical `60 / 8`,
  the same `incumbent_dominance = 110`, the same
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same targeted four-cell plus `24`-pruned-prefix delta on
  the active `reference` bucket
- another representative mismatch-`0` claim-side self-contained reland on
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_flat_codomain`; that alternate qualification family now also relands
  the same unsafe `4427 / 545 / 2247` shell with the same noncanonical
  `60 / 8`, the same `incumbent_dominance = 110`, the same
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same targeted four-cell plus `24`-pruned-prefix delta on
  the active `claim_flat_codomain` bucket
- another representative mismatch-`0` claim-side self-contained reland on
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  reference`; that sibling alternate qualification family now relands the
  same unsafe `4427 / 545 / 2247` shell with the same noncanonical `60 / 8`,
  the same `incumbent_dominance = 110`, the same
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same targeted four-cell plus `24`-pruned-prefix delta on
  the active `reference` bucket
- another representative mismatch-`0` claim-side active-window or
  self-contained reland narrowed to clause-`6` `reference` on
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_flat_codomain`; those narrower alternate refinements now only reland
  unsafe noncanonical `60 / 8` with `retained = 4`, the same
  `4427 / 545 / 2247`, `incumbent_dominance = 113`,
  `small_cluster = 2904 / 462 / 462 / 109 / 2`, no `single` bucket, best
  overshoot `545 / 5278`, and the same targeted four-cell plus
  `24`-pruned-prefix delta on the active `claim_flat_codomain` bucket
- another representative mismatch-`0` claim-side active-window or
  self-contained reland narrowed to clause-`6` `reference` on
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  reference`; those sibling narrower alternate refinements now only reland
  unsafe noncanonical `60 / 8` with `retained = 2`, the same
  `4427 / 545 / 2247`, `incumbent_dominance = 115`,
  `small_cluster = 2904 / 462 / 462 / 109 / 2`, no `single` bucket, best
  overshoot `545 / 5278`, and the same targeted four-cell plus
  `24`-pruned-prefix delta on the active `reference` bucket
- another representative mismatch-`0` claim-side parent-route reland narrowed
  to clause-`6` `reference` on `claim_eventual_domain / claim_next_codomain /
  claim_next_bridge / claim_flat_codomain`; that refinement now relands the
  unsafe `4427 / 545 / 2247` shell on a different noncanonical `74 / 8`
  winner with `retained = 1`, `incumbent_dominance = 111`,
  `small_cluster = 2904 / 430 / 430 / 108`, a still-fenced `single` bucket,
  and the same targeted four-cell plus `24`-pruned-prefix delta on the active
  `claim_flat_codomain` bucket
- another representative mismatch-`0` claim-side parent-route reland narrowed
  to clause-`6` `reference` on `claim_eventual_domain / claim_next_codomain /
  claim_next_bridge / reference`; that sibling refinement now relands the
  same unsafe `4427 / 545 / 2247` shell with the same noncanonical `74 / 8`,
  the same `retained = 1`, the same `incumbent_dominance = 111`, the same
  `small_cluster = 2904 / 430 / 430 / 108`, the same fenced `single` bucket,
  and the same targeted four-cell plus `24`-pruned-prefix delta on the active
  `reference` bucket
- another representative mismatch-`0` claim-side parent-route reland narrowed
  to only `claim_flat_argument` on
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_flat_codomain`; that refinement now relands the smaller but still
  unsafe `4379 / 549 / 2259` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 113`,
  `small_cluster = 2871 / 435 / 435 / 111`, and the same reopened `single`
  bucket
- another representative mismatch-`0` claim-side parent-route reland narrowed
  to only `claim_eventual_argument` on
  `claim_eventual_domain / claim_next_codomain / claim_next_bridge /
  claim_flat_codomain`; that sibling refinement now relands the same smaller
  unsafe `4379 / 549 / 2259` shell with the same noncanonical `60 / 8`, the
  same `incumbent_dominance = 113`, the same
  `small_cluster = 2871 / 435 / 435 / 111`, and the same reopened `single`
  bucket
- another whole joint clause-`3` continuation reopen under that same
  representative clause-`6` `reference` shell
- another representative claim-flat parent/child-shell completion pass that
  only rechecks the same six dead `3`-generated / `0`-admitted child
  continuations or re-partitions them by clause-`3` / clause-`6` identity
- another finer reason-level split below that same representative
  `claim_eventual_domain / claim_next_codomain / claim_flat_domain /
  claim_next_bridge / claim_flat_codomain` dead child before the
  alternate broader backups have been compared
- another representative claim-sharp parent/child-shell completion pass that
  only rechecks the same six dead `3`-generated / `0`-admitted child
  continuations or re-partitions them by clause-`3` / clause-`6` identity
- another finer reason-level split below that same representative
  `claim_eventual_domain / claim_next_codomain / claim_sharp_codomain /
  claim_next_bridge / claim_flat_codomain` dead child before the
  rerun-backed step-`15` reset has been run
- another lumped `reference / reference` tail restatement now that the full
  mismatch-`2` and mismatch-`3` tail is spent
- another mismatch-`2` `reference / reference` clause-`4`
  `claim_next_bridge`-half reland now that it is pinned as a `4547 / 535`
  tradeoff control with a wider `small_cluster`
- another mismatch-`2` `reference / reference` clause-`4`
  `reference`-half reland now that it is pinned as a sharper
  `4835 / 529` tradeoff control with an even wider `small_cluster`
- another mismatch-`2` `reference / reference` clause-`4`
  `demo_sharp_codomain`-pocket reland now that it is pinned as a
  `4379 / 549` smaller tradeoff control with
  `small_cluster = 3168 / 522 / 522 / 0`
- another mismatch-`2` `reference / reference` clause-`4`
  `demo_sharp_bridge`-pocket reland now that it is pinned as the same
  `4379 / 549` smaller tradeoff control with the same widened
  `3168 / 522 / 522 / 0` `small_cluster`
- another mismatch-`3` `reference / reference` clause-`4`
  `claim_next_bridge`-half reland now that it is pinned as a
  `4403 / 547` smaller tradeoff control with
  `small_cluster = 3186 / 522 / 522 / 0`
- another mismatch-`3` `reference / reference` clause-`4`
  `reference`-half reland now that it is pinned as a sharper
  `4481 / 545` tradeoff control with
  `small_cluster = 3240 / 522 / 522 / 0`
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

- canonical `v13` stands as the newer stored baseline on current-head code
- step `1` stays explicit but deferred because `v13` re-confirms the same
  breadth-only step-`15` miss
- both representative mismatch-`0` claim-side parent-route probes on the
  active clause-`5` `claim_flat_codomain / reference` families are pinned as
  the same unsafe negative control on `4427 / 545 / 2247` with noncanonical
  `60 / 8`, `incumbent_dominance = 117`,
  `small_cluster = 2931 / 455 / 455 / 115`, and a reopened `single` bucket
- that matched unsafe route class is also localized to only the four targeted
  claim-side remaining-two parent cells plus their `24` corresponding
  remaining-one pruned prefixes on the chosen active clause-`5` bucket, with
  no off-target capture or prune families introduced
- the first alternate active-window qualification family on those same active
  clause-`5` `claim_flat_codomain / reference` buckets is also pinned as a
  matched unsafe negative control on the same `4427 / 545 / 2247` with
  noncanonical `60 / 8`, `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four-cell plus `24`-pruned-prefix targeted delta
- the next alternate self-contained qualification family on those same active
  clause-`5` `claim_flat_codomain / reference` buckets is also pinned as the
  same matched unsafe negative control on the same `4427 / 545 / 2247` with
  noncanonical `60 / 8`, `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four-cell plus `24`-pruned-prefix targeted delta
- the narrower clause-`6` `reference` refinements of those same alternate
  active-window and self-contained families are also pinned as unsafe split
  controls rather than fresh repairs:
  `claim_flat_codomain` relands noncanonical `60 / 8` with `retained = 4`,
  `incumbent_dominance = 113`, and
  `small_cluster = 2904 / 462 / 462 / 109 / 2`,
  while `reference` relands noncanonical `60 / 8` with `retained = 2`,
  `incumbent_dominance = 115`, and the same
  `2904 / 462 / 462 / 109 / 2` `small_cluster`
- those narrower alternate-family clause-`6` refinements also keep best
  overshoot `545 / 5278`, no `single` bucket, the same
  `304 / 177 / 50 / 14` first-mismatch distribution, the same
  `11 / 11 / 12` clause-`2` spread, the same `20 / 14` clause-`4` split, and
  the same four targeted remaining-two parent cells plus their `24`
  corresponding remaining-one pruned prefixes
- the narrower clause-`6` `reference` refinement of that same parent-route
  class is also pinned across both active clause-`5` families at unsafe
  noncanonical `74 / 8` with `retained = 1`, the same `4427 / 545 / 2247`
  surface, `incumbent_dominance = 111`,
  `small_cluster = 2904 / 430 / 430 / 108`, a still-fenced `single` bucket,
  and the same four-cell plus `24`-pruned-prefix targeted delta
- the representative claim-flat clause-`3`
  `claim_flat_argument / claim_eventual_argument` refinements inside that
  same parent-route family are also pinned as the same smaller unsafe
  negative control at noncanonical `60 / 8` with `retained = 2`,
  `generated = 4379`, `partial_prefix_bar_failure = 549`,
  `incumbent_dominance = 113`, zero-admitted captures `2259`,
  `small_cluster = 2871 / 435 / 435 / 111`, and the same reopened
  `single` bucket
- the next operational slice stays above the exhausted claim-safe,
  representative mismatch-`0`, `reference / reference`, isolated
  clause-`1` exact-suffix, representative claim-side clause-`6`
  `reference` union, both representative claim-side parent-route controls,
  the matched representative claim-side active-window and self-contained
  families, their narrower clause-`6` `reference` refinements, the narrower
  alternate active-window and self-contained clause-`6` `reference`
  refinements too, and the representative claim-flat clause-`3`
  refinements while now targeting a different parent-level qualification
  above the current remaining-one lattice
- any new surface is judged against the unchanged fences:
  canonical `103 / 8`, fenced `single`, fenced lifted `89 / 8`, and
  `small_cluster <= 3132 / 522 / 522 / 0`
