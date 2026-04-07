# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-07

This file is the exact next work order for `desktop_claim_shadow`.

## Objective

Narrow the remaining clean stored step-`15` partial-prefix wall on top of
canonical `v12` and the matched local `4331` guardrail surface, now that the
real `553`-prune wall split is frozen executable-in-tree.

Do not start with another rerun setup pass.

## Start From

- Canonical stored bundle:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v12`
- Previous stored comparison point:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v11`
- Stored breadth blockers:
  - step `1 = 546 / 2144`
  - step `15 = 4331 / 5000`
- Current canonical late chain:
  - step `13`: accepted `46 / 7`, generated `2320`
  - step `14`: accepted `62 / 9`, generated `12027`
  - step `15`: accepted `103 / 8`, generated `4331`
- Current step-`15` pressure:
  - partial-prefix bar failures: `553`
  - incumbent-dominance prunes: `3`
  - `small_cluster`: `3132 / 522 / 522 / 0`
  - fenced `single` bucket: `1` fully scored non-winner plus `3` residual prunes
- Executable partial-prefix wall split:
  - remaining-two prefixes: `451`
  - remaining-three prefixes: `102`
  - first mismatch positions: clause `0 = 312`, clause `1 = 177`,
    clause `2 = 50`, clause `3 = 14`
  - dominant remaining-two slice: clause `0 = 252`, clause `1 = 145`

## The Real Remaining Problem

- Clean stored and local evidence now align on the same `4331` step-`15`
  surface, so the next honest move is no longer proving the last repair.
- The dominant remaining late miss is now the clean `553` partial-prefix wall
  on the canonical temporal-shell surface.
- That wall is now executable rather than notes-only:
  - `current_claim_step_fifteen_partial_prefix_wall_stays_on_four_early_temporal_prefix_families`
    freezes all `553` actual bound prunes
  - it shows that the live wall now sits entirely on clause positions `0..3`
    and is dominated by remaining-two prefixes, especially clause-`0` and
    clause-`1`
- The dominant remaining-two slice is now executable one layer deeper too:
  - `current_claim_step_fifteen_remaining_two_partial_prefix_wall_stays_on_nine_clause_zero_one_pairings`
    freezes mismatch position `0` as six exact `42`-count pairings:
    clause-`0` `claim_flat_domain` or `claim_eventual_domain` crossed with
    clause-`1` `reference`, `claim_sharp_codomain`, or
    `claim_next_codomain`
  - it freezes mismatch position `1` as clause-`0` `reference` with exactly
    three clause-`1` pairings:
    `claim_sharp_codomain = 42`, `claim_next_codomain = 42`,
    `demo_flat_codomain = 61`
  - the remaining-two mismatch-`2` / mismatch-`3` tail stays only the narrow
    `reference/reference` continuation at `42` and `12`
  - `current_claim_step_fifteen_remaining_two_partial_prefix_wall_sits_on_claim_next_bridge_and_reference_clause_four_families`
    now freezes the same dominant side one layer deeper again:
    mismatch-`0` stays only on clause-`4` `claim_next_bridge = 48 / 48 / 48`
    and clause-`4` `reference = 36 / 36 / 36` against clause-`5`
    `claim_flat_codomain`, `claim_next_codomain`, and `reference`;
    mismatch-`1` stays on the same clause-`4` / clause-`5` grid at
    `27 / 27 / 27` and `22 / 22 / 20`
  - the mismatch-`2` / mismatch-`3` tail stays much smaller and is the only
    place where the old demo-only clause-`4` bridge pockets still appear at
    all
  - so the next honest repair target is still the current clause-`0` /
    clause-`1` claim surface, but now specifically on the live clause-`4`
    `claim_next_bridge` plus `reference` families rather than another broad
    omitted-side reopening
  - `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_four_pressure_on_claim_next_bridge_per_clause_zero_one_pairing`
    now freezes that same live slice per dominant pairing:
    every mismatch-`0` pairing sits at clause-`4`
    `claim_next_bridge = 24` versus `reference = 18`,
    mismatch-`1` keeps the same `24 / 18` split on
    `reference + claim_sharp_codomain` and `reference + claim_next_codomain`,
    and the larger `reference + demo_flat_codomain` side still stays on the
    same live clause-`4` claim families at `33 / 28`
  - `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_two_pressure_on_claim_variants_under_the_live_clause_zero_one_pairs`
    now freezes the same dominant slice across clause `2`:
    the regular mismatch-`0` / mismatch-`1` pairings stay at
    `claim_flat_domain = 15`, `claim_sharp_codomain = 15`,
    `reference = 12`,
    while the larger `reference + demo_flat_codomain` side stays at
    `23 / 23 / 15`
  - so the next honest repair should still work on the live clause-`4`
    `claim_next_bridge` plus `reference` split on the dominant clause-`0` /
    clause-`1` pairings, not on another clause-`3` widening pass or a hidden
    demo-only clause-`2` reopening
- A local clause-`1` `demo_eventually_codomain` exact-pocket reland has now
  been checked, reverted, and pinned by
  `current_claim_step_fifteen_clause_one_demo_eventually_codomain_exact_pocket_reland_stays_a_negative_control`:
  - local step `15` generated breadth lifted to `4466`
  - the `small_cluster` widened to `3156 / 526 / 526 / 0`
  - the isolated `single` pocket and residual `3` incumbent prunes stayed
    fenced
  - but the partial-prefix wall widened to `626` and the zero-admitted
    capture widened to `2562`
  - so that reland is now ruled out as another negative control rather than
    the next repair
- A broader clause-`0` `claim_flat_domain` plus clause-`1`
  `demo_flat_codomain` exact-pocket reland has now also been checked under
  test-only scoped overrides and pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_exact_pocket_reland_stays_a_negative_control`
  plus the matching connectivity override tests:
  - local step `15` generated breadth again lifted to `4466`
  - the isolated `single` pocket and residual `3` incumbent prunes stayed
    fenced
  - but the partial-prefix wall again widened to `626`
  - so that broader flat-pocket reland is also ruled out as another negative
    control rather than the next repair
- A narrower clause-`0` `reference` plus clause-`1` `demo_flat_codomain`
  live-claim-bridge reopening has now also been checked under a scoped
  connectivity-only override and pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_stays_a_tradeoff_control`
  plus the matching connectivity override tests:
  - local step `15` generated breadth lifted to `4523`
  - the clean partial-prefix wall narrowed to `537`
  - zero-admitted capture narrowed to `2223`
  - the larger mismatch-`1` `reference + demo_flat_codomain` branch shrank
    from `61` captured prefixes on clause-`4` `33 / 28` down to `45` on
    clause-`4` `27 / 18`
  - the isolated `single` pocket and residual `3` incumbent prunes stayed
    fenced
  - but the `small_cluster` widened to `3324 / 554 / 554 / 0`
  - so that narrower reopening is now only a tradeoff control rather than
    the landed repair, and the next slice should isolate those escaping `16`
    captures without taking the full `small_cluster` regression
  - that tradeoff branch is now executable one layer deeper too:
    `current_claim_step_fifteen_clause_one_demo_flat_codomain_tradeoff_control_splits_evenly_across_three_clause_two_sheets`
    and
    `current_claim_step_fifteen_clause_one_demo_flat_codomain_tradeoff_control_splits_each_clause_two_sheet_as_three_by_three_and_two_by_two_bridge_cells`
    pin the remaining `45` captured prefixes as three equal clause-`2`
    sheets at `15 / 15 / 15`, and each sheet then splits uniformly as
    clause-`4` `claim_next_bridge = 3 / 3 / 3` plus clause-`4`
    `reference = 2 / 2 / 2` across clause-`5`
    `claim_flat_codomain`, `claim_next_codomain`, and `reference`
  - so the next honest reland can now stay narrower than the whole tradeoff
    reopening: the later exact clause-`2` single-sheet probes, the combined
    claim-pair probe, and the narrower clause-`4` follow-on can all stay
    inside that branch until the remaining off-branch delta is frozen on the
    clause-`4` `reference` remaining-three tail
- A narrower clause-`4` `reference`-sheet reopening on that same
  `reference + demo_flat_codomain` branch has now also been checked under a
  second scoped connectivity-only override and pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_stays_a_tradeoff_control`
  plus the matching connectivity override tests:
  - local step `15` generated breadth lifted only to `4379`
  - the clean partial-prefix wall narrowed only to `549`
  - zero-admitted capture narrowed only to `2259`
  - the larger mismatch-`1` `reference + demo_flat_codomain` branch shrank
    only from clause-`4` `33 / 28` / `61` down to clause-`4` `33 / 24` / `57`
  - the isolated `single` pocket and residual `3` incumbent prunes stayed
    fenced
  - but the `small_cluster` still widened to `3180 / 530 / 530 / 0`
  - the reopened cells stayed narrower than the broader tradeoff control:
    only four clause-`4` `reference` cells reopened, only on the
    `claim_flat_domain` plus `claim_sharp_codomain` clause-`2` sheets, while
    the full `33`-count clause-`4` `claim_next_bridge` side stayed unchanged
  - so that narrower reopening is also only a tradeoff control rather than
    the landed repair, and it motivated the exact clause-`2` sheet probes
    that are now both pinned before any larger clause-`4`
    `claim_next_bridge` reopening
- A narrower exact clause-`2` `claim_flat_domain` sheet reopening on that
  same `reference + demo_flat_codomain` branch has now also been checked
  under a third scoped connectivity-only override and pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_stays_a_tradeoff_control`
  plus the matching connectivity override tests:
  - local step `15` generated breadth lifted to `4412`
  - the clean partial-prefix wall narrowed to `544`
  - zero-admitted capture narrowed to `2250`
  - the larger mismatch-`1` `reference + demo_flat_codomain` branch shrank
    from clause-`4` `33 / 28` / `61` down to clause-`4` `30 / 21` / `51`
  - the isolated `single` pocket and residual `3` incumbent prunes stayed
    fenced
  - but the `small_cluster` still widened to `3216 / 536 / 536 / 0`
  - the reopened bridge cells now reland only the exact
    `claim_flat_domain` clause-`2` sheet at the full `3 / 3 / 3` plus
    `2 / 2 / 2` split, while the sibling `claim_sharp_codomain` and
    `reference` sheets stay higher at `21` and `15`
  - so that exact-sheet reopening is also only a tradeoff control rather
    than the landed repair, but it is stronger than the clause-`4`
    `reference`-sheet probe
- A narrower exact clause-`2` `claim_sharp_codomain` sheet reopening on that
  same `reference + demo_flat_codomain` branch has now also been checked
  under a fourth scoped connectivity-only override and pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_stays_a_tradeoff_control`
  plus the matching connectivity override tests:
  - local step `15` generated breadth also lifted to `4412`
  - the clean partial-prefix wall also narrowed to `544`
  - zero-admitted capture also narrowed to `2250`
  - the larger mismatch-`1` `reference + demo_flat_codomain` branch also
    shrank from clause-`4` `33 / 28` / `61` down to clause-`4`
    `30 / 21` / `51`
  - the isolated `single` pocket and residual `3` incumbent prunes still
    stayed fenced
  - but the `small_cluster` still widened to `3216 / 536 / 536 / 0`
  - the reopened bridge cells now reland only the exact
    `claim_sharp_codomain` clause-`2` sheet at the full `3 / 3 / 3` plus
    `2 / 2 / 2` split, while the sibling `claim_flat_domain` sheet stays at
    `21` and the `reference` sheet stays at `15`
  - so that exact-sheet reopening is also only a tradeoff control rather
    than the landed repair, but it now shows the two claim clause-`2`
    sheets are numerically symmetric while the `reference` sheet is already
    at the `15`-capture floor
- A combined exact clause-`2` claim-variant pair reopening on that same
  `reference + demo_flat_codomain` branch has now also been checked under a
  fifth scoped connectivity-only override and pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_stays_a_tradeoff_control`
  plus the matching connectivity override tests:
  - local step `15` generated breadth also lifted to `4523`
  - the clean partial-prefix wall also narrowed to `537`
  - zero-admitted capture also narrowed to `2223`
  - the larger mismatch-`1` `reference + demo_flat_codomain` branch also
    shrank from clause-`4` `33 / 28` / `61` down to clause-`4`
    `27 / 18` / `45`
  - the isolated `single` pocket and residual `3` incumbent prunes still
    stayed fenced
  - but the `small_cluster` still widened to `3324 / 554 / 554 / 0`
  - the reopened bridge cells now exactly match the broader live-claim-bridge
    tradeoff control: both claim clause-`2` sheets fall to the
    `15`-capture floor, while the `reference` sheet was already there
  - so that combined-pair reopening is also only a tradeoff control rather
    than the landed repair, and it now shows the clause-`2`-only narrowing
    on that mismatch-`1` branch is exhausted
- A narrower exact clause-`4` `claim_next_bridge`-side reopening on that same
  combined claim-variant pair surface has now also been checked under a
  sixth scoped connectivity-only override and pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_variant_pair_stays_a_tradeoff_control`
  plus the matching connectivity override tests:
  - local step `15` generated breadth lifted to `4445`
  - the clean partial-prefix wall narrowed to `539`
  - zero-admitted capture narrowed to `2241`
  - the isolated `single` pocket and residual `3` incumbent prunes still
    stayed fenced
  - but the `small_cluster` still widened to `3252 / 542 / 542 / 0`
  - the captured mismatch-`1` `reference + demo_flat_codomain` branch
    already matches the broader exact claim-pair tradeoff exactly at
    clause-`4` `27 / 18` / `45`, with the same executable
    `3 / 3 / 3` plus `2 / 2 / 2` bridge grid on each clause-`2` sheet
  - so that narrower reopening is also only a tradeoff control rather than
    the landed repair, and a newer executable delta-localization freeze now
    pins the remaining off-branch difference one layer deeper:
    `current_claim_step_fifteen_exact_claim_variant_pair_tradeoff_delta_outside_the_claim_next_bridge_side_probe_lives_on_two_clause_four_reference_remaining_three_prefixes`
  - comparing that narrower probe against the broader exact claim-pair
    tradeoff, the broader side contributes no unique residual
    partial-prefix captures or zero-admitted exact-prune prefixes at all
  - instead the narrower probe alone still keeps exactly two
    remaining-three partial-prefix captures, both at mismatch-`1` on
    `reference + demo_flat_codomain`, with clause `2` on
    `claim_flat_domain` or `claim_sharp_codomain` while clause `4` stays on
    `reference`
  - one layer deeper, that same off-branch delta is the exact `18`
    zero-admitted prefixes: each of those two remaining-three prefixes splits
    evenly across clause `5` `claim_flat_codomain`,
    `claim_next_codomain`, and `reference` at `3 / 3 / 3`
  - so the next honest slice is now the two clause-`4` `reference`
    remaining-three prefixes on the `reference + demo_flat_codomain`
    surface rather than the already-frozen `45`-capture mismatch-`1`
    branch or the larger clause-`4` `claim_next_bridge` half
- A narrower exact clause-`5` `reference` follow-on on that same
  clause-`4` `reference` tail has now also been checked under a seventh
  scoped connectivity-only override and pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_neutral_control`
  plus the matching connectivity override tests:
  - local step `15` generated breadth stays pinned at `4331`
  - the clean partial-prefix wall stays pinned at `553`
  - zero-admitted capture stays pinned at `2271`
  - the larger mismatch-`1` `reference + demo_flat_codomain` branch stays
    pinned at clause-`4` `33 / 28` / `61`
  - the `small_cluster` and isolated `single` pocket stay pinned at
    `3132 / 522 / 522 / 0` and the fenced `1 + 3` split
  - so that exact clause-`5` `reference` cell is already effectively part of
    the current canonical surface rather than the next repair
- A narrower exact clause-`5` `claim_flat_codomain` follow-on on that same
  clause-`4` `reference` tail has now also been checked under an eighth
  scoped connectivity-only override and pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_tradeoff_control`
  plus the matching connectivity override tests:
  - local step `15` generated breadth lifts only to `4355`
  - the clean partial-prefix wall narrows only to `551`
  - zero-admitted capture narrows only to `2265`
  - the larger mismatch-`1` `reference + demo_flat_codomain` branch shrinks
    only to clause-`4` `33 / 26` / `59`
  - the isolated `single` pocket and residual `3` incumbent prunes stay
    fenced
  - but the `small_cluster` still widens to `3156 / 526 / 526 / 0`
  - the reopened bridge grid shows only the exact clause-`5`
    `claim_flat_codomain` cell reopening on the two claim clause-`2` sheets,
    while the sibling `claim_next_codomain` cell stays at the higher
    four-capture level
  - so that probe is also only a smaller tradeoff control rather than the
    landed repair
- A symmetric exact clause-`5` `claim_next_codomain` follow-on on that same
  clause-`4` `reference` tail has now also been checked under a ninth scoped
  connectivity-only override and pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_tradeoff_control`
  plus the matching connectivity override tests:
  - it lands the same `4355 / 551 / 2265` late surface with the same
    `3156 / 526 / 526 / 0` `small_cluster` regression and the same fenced
    `1 + 3` split
  - it also shrinks that same branch only to clause-`4` `33 / 26` / `59`
  - the bridge grid is numerically symmetric with the exact
    clause-`5` `claim_flat_codomain` probe: only the exact
    `claim_next_codomain` cell reopens on each claim clause-`2` sheet while
    the sibling `claim_flat_codomain` cell stays at the higher four-capture
    level
  - so that probe is also only a smaller tradeoff control rather than the
    landed repair
- Together, those two exact single-family clause-`5` probes now show that the
  clause-`4` `reference` remaining-three tail is exhausted as a cleaner local
  repair target:
  - the exact clause-`5` `reference` cell was already neutral on the current
    canonical surface
  - the exact clause-`5` `claim_flat_codomain` and `claim_next_codomain`
    cells are now only two symmetric smaller halves of the already-pinned
    clause-`4` `reference`-sheet tradeoff at `4379 / 549 / 2259`
  - so the next honest slice is no longer another clause-`5` tail reopening;
    it is to step back up to the broader dominant clause-`0` / clause-`1`
    claim surface outside this exhausted exact clause-`4` `reference` tail
- A local clause-`3` `anchor-11` exact-argument widening onto the broader
  clause-`0` / clause-`1` claim surface while clause `2` stayed `reference`
  has now also been checked and reverted:
  - the clean step-`15` partial-prefix wall stayed pinned at `553`
  - the executable remaining-two nine-pair split stayed the same
  - but summary-stage incumbent captures reopened from `0` to `72`
  - so that clause-`3` widening is also ruled out as the next repair rather
    than another live candidate
- A local clause-`5` side-pocket broadening onto the claim-safe clause-`0` /
  clause-`1` surface has now also been checked under scoped overrides and
  pinned by
  `current_claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  plus the matching connectivity override tests:
  - local step `15` generated breadth lifted to `4779`
  - the `small_cluster` widened to `3516 / 586 / 586 / 0`
  - the isolated `single` pocket and residual `3` incumbent prunes stayed
    fenced
  - but the partial-prefix wall widened to `585` and the zero-admitted
    capture widened to `2367`
  - so that broader clause-`5` reland is also ruled out as another negative
    control rather than the next repair
- Clause-`4` claim-safe reopenings have now also been checked under scoped
  overrides and pinned as negative controls:
  - the broad clause-`4` `demo_sharp_codomain` plus `demo_sharp_bridge`
    reopening on the claim-safe clause-`0` / clause-`1` surface is pinned by
    `current_claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
    plus the matching connectivity override tests:
    - local step `15` generated breadth lifted to `4843`
    - the `small_cluster` widened to `3516 / 586 / 586 / 0`
    - the isolated `single` pocket and residual `3` incumbent prunes stayed
      fenced
    - but the partial-prefix wall widened to `617` and the zero-admitted
      capture widened to `2463`
  - the narrower clause-`4` `demo_sharp_codomain`-only reopening is pinned by
    `current_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`,
    and the matching clause-`4` `demo_sharp_bridge`-only reopening is pinned
    by
    `current_claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  - each narrower half still lifted local step `15` generated breadth only to
    `4587`, widened the `small_cluster` to `3324 / 554 / 554 / 0`, kept the
    isolated `single` pocket and residual `3` incumbent prunes fenced, and
    still widened the partial-prefix wall to `585` with zero-admitted capture
    widened to `2367`
  - so even the narrower clause-`4` claim-safe reopenings are also ruled out
    as the next repair rather than new live candidates
- A narrower clause-`5` reland on only the exact remaining-two mismatch-`0`
  bridge slice has now also been checked under scoped overrides and pinned by
  `current_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice_stays_a_negative_control`
  plus the matching injector and connectivity override tests:
  - local step `15` generated breadth lifted to `4691`
  - the `small_cluster` widened to `3420 / 570 / 570 / 0`
  - the isolated `single` pocket and residual `3` incumbent prunes stayed
    fenced
  - but the partial-prefix wall widened to `589`
  - so that narrower mismatch-`0` bridge-slice reland is also ruled out as
    another negative control rather than the next repair
- The old proof-close blocker has been reduced to only `3` fenced
  same-primary `single`-bucket incumbent prunes:
  - clause-`0` `claim_flat_domain`
  - clause-`2` `claim_flat_domain` plus anchor-`11` exact-argument
  - clause-`5` `claim_flat_codomain`
- Those three families still keep only the reference terminal live, while both
  unsafe lifted `89 / 8` terminals remain fenced outside historical reanchor
  and live connectivity.
- Step `1` is still unchanged and separate; it should stay visible, but it is
  not the first slice to reopen from this new bundle.

## Do This Next

1. Re-read the stored `v12` certificate, compare report, benchmark bundle, and
   `reports/steps/step-15-live.ndjson` before touching search code again.
2. Keep step `1` explicit as the separate early breadth blocker, but do not
   reopen it first while step `15` still has the cleaner late-surface
   diagnosis.
3. Start the next repair on the dominant remaining-two clause-`0` / clause-`1`
   side of the clean partial-prefix wall before reopening the smaller
   clause-`2` / clause-`3` tail:
   - remaining-two clause `0 = 252`
   - remaining-two clause `1 = 145`
   - remaining-two clause `2 = 42`
   - remaining-two clause `3 = 12`
   - remaining-three tail across the same four positions = `60 / 32 / 8 / 2`
   - mismatch-`0` current pairings:
     `claim_flat_domain` or `claim_eventual_domain`
     crossed with clause-`1` `reference`, `claim_sharp_codomain`, or
     `claim_next_codomain`, all at `42`
   - mismatch-`1` current pairings:
     clause-`0` `reference` with clause-`1`
     `claim_sharp_codomain = 42`, `claim_next_codomain = 42`,
     `demo_flat_codomain = 61`
   - local capture inspection is now executable too:
     mismatch-`0` lives only on clause-`4` `claim_next_bridge` and
     clause-`4` `reference`, with the clause-`4` `claim_next_bridge` side
     larger at `48 / 48 / 48` versus `36 / 36 / 36` across clause-`5`
     `claim_flat_codomain`, `claim_next_codomain`, and `reference`
   - mismatch-`1` stays on that same clause-`4` / clause-`5` grid at
     `27 / 27 / 27` and `22 / 22 / 20`, while the older demo-only
     clause-`4` bridge pockets survive only on the smaller mismatch-`2` tail
   - the new per-pair clause-`4` freeze now sharpens that live slice again:
     every mismatch-`0` pairing keeps the same `24 / 18`
     clause-`4` `claim_next_bridge` / `reference` split,
     the mismatch-`1` `reference + claim_sharp_codomain` and
     `reference + claim_next_codomain` pairings keep that same `24 / 18`
     split, and the larger `reference + demo_flat_codomain` side still stays
     on the same claim families at `33 / 28`
   - the new clause-`2` freeze now shows that those same dominant pairings
     still sit mostly on the two current claim clause-`2` variants rather
     than on a hidden demo-only clause-`2` reopening:
     `15 / 15 / 12` on the regular pairings and `23 / 23 / 15` on the larger
     mismatch-`1` `demo_flat_codomain` side
   - the newer scoped tradeoff control
     `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_stays_a_tradeoff_control`
     now shows that reopening only that larger mismatch-`1`
     `reference + demo_flat_codomain` branch cuts it from clause-`4`
     `33 / 28` / `61` captured prefixes to clause-`4` `27 / 18` / `45`,
     lifts local breadth to `4523`, narrows the clean wall to `537`, and
     cuts zero-admitted capture to `2223`
   - the newer tradeoff-branch freezes now make that same `45`-capture
     branch executable one layer deeper:
     the captured prefixes split evenly across clause-`2`
     `claim_flat_domain = 15`, `claim_sharp_codomain = 15`,
     `reference = 15`, and each clause-`2` sheet then splits uniformly as
     clause-`4` `claim_next_bridge = 3 / 3 / 3` plus clause-`4`
     `reference = 2 / 2 / 2` across clause-`5`
     `claim_flat_codomain`, `claim_next_codomain`, and `reference`
   - but that same probe still widens the `small_cluster` to
     `3324 / 554 / 554 / 0`, so the next landed repair should isolate those
     escaping `16` captures on that branch without relanding the whole
     reopening; the later exact sheet, claim-pair, and clause-`4`
     follow-ons now pin that residual down to the narrower
     clause-`4` `reference` remaining-three tail
   - the newer clause-`4` `reference`-sheet tradeoff control now shows that
     reopening only that smaller side is real but still not enough:
     it lifts local breadth only to `4379`, narrows the clean wall only to
     `549`, cuts zero-admitted capture only to `2259`, shrinks the larger
     mismatch-`1` branch only to clause-`4` `33 / 24` / `57`, and still
     widens the `small_cluster` to `3180 / 530 / 530 / 0`
   - the newer exact clause-`2` `claim_flat_domain` and
     `claim_sharp_codomain` sheet probes now show that reopening only one
     claim sheet at a time is real and stronger than the clause-`4`
     `reference`-sheet probe, but still not enough:
     each lifts local breadth to `4412`, narrows the clean wall to `544`,
     cuts zero-admitted capture to `2250`, shrinks that same branch to
     clause-`4` `30 / 21` / `51`, and still widens the `small_cluster` to
     `3216 / 536 / 536 / 0`
   - together they now show that the `reference` clause-`2` sheet is already
     at the `15`-capture floor, and the remaining gap to the full
     `45`-capture tradeoff is the shared `21 -> 15` residual on whichever
     claim clause-`2` sheet stays closed
   - the newer combined exact claim-variant clause-`2` pair probe now shows
     what happens when that whole residual is reopened at once:
     it reproduces the broader `4523 / 537 / 2223` tradeoff surface exactly,
     shrinks that same branch to clause-`4` `27 / 18` / `45`, and still
     widens the `small_cluster` to `3324 / 554 / 554 / 0`
   - the newer exact clause-`4` `claim_next_bridge`-side probe on that same
     combined claim-pair surface is now pinned too:
     it lifts local breadth to `4445`, narrows the clean wall to `539`,
     cuts zero-admitted capture to `2241`, and still widens the
     `small_cluster` only to `3252 / 542 / 542 / 0`
   - crucially, that narrower probe already reproduces the same
     mismatch-`1` `reference + demo_flat_codomain` captured branch as the
     broader exact claim-pair tradeoff, including the same executable
     clause-`4` `27 / 18` / `45` split and the same per-sheet
     `3 / 3 / 3` plus `2 / 2 / 2` bridge grid
   - the newer executable delta freeze
     `current_claim_step_fifteen_exact_claim_variant_pair_tradeoff_delta_outside_the_claim_next_bridge_side_probe_lives_on_two_clause_four_reference_remaining_three_prefixes`
     now shows that the broader tradeoff contributes no unique residual
     captures or exact prunes beyond that narrower probe
   - instead the narrower probe alone still keeps exactly two
     remaining-three prefixes on the `reference + demo_flat_codomain`
     mismatch-`1` surface, one on each claim clause-`2` sheet while
     clause-`4` stays `reference`
   - one layer deeper those same two prefixes account for the whole exact
     off-branch delta: `18` zero-admitted prunes split evenly as `3 / 3 / 3`
     across clause-`5` `claim_flat_codomain`, `claim_next_codomain`, and
     `reference` on each sheet
   - so the next honest slice is no longer generic off-branch localization;
     it is to test or repair just those two clause-`4` `reference`
     remaining-three prefixes before any broader clause-`4` or clause-`5`
     reopening
   - the newer exact clause-`5` `reference` follow-on on that same tail is
     now also ruled out as the next repair because it leaves the whole
     `4331 / 553 / 2271` surface unchanged
   - the newer exact clause-`5` `claim_flat_codomain` and
     `claim_next_codomain` follow-ons on that same tail are now also pinned
     as symmetric smaller tradeoff controls:
     each lands only `4355 / 551 / 2265`, trims that same branch only to
     clause-`4` `33 / 26` / `59`, and still widens the `small_cluster` to
     `3156 / 526 / 526 / 0`
   - together those two exact single-family probes merely decompose the
     already-pinned clause-`4` `reference`-sheet tradeoff at
     `4379 / 549 / 2259`, so the clause-`4` `reference` remaining-three tail
     is now exhausted as a cleaner local repair target rather than the next
     repair
   - the reverted clause-`3` `anchor-11` widening with clause `2 = reference`
     left that clause-`4` family split unchanged and only reopened
     summary-stage incumbent pressure
   - the newer claim-safe clause-`4` reopenings also stayed negative
     controls: the narrower single-side variants each widened the wall to
     `585`, and the broader combined clause-`4` reopening widened it further
     to `617`
   - so the next honest slice is still the live clause-`0` / clause-`1`
     claim surface itself, but no longer on another clause-`5`
     remaining-three tail probe; it should step back up outside that
     exhausted exact clause-`4` `reference` tail and relocalize the broader
     dominant clause-`0` / clause-`1` pressure before another raw
     clause-`3` widen-first probe, another demo-bridge reland, a
     claim-safe clause-`4` reopening, or a rerun
4. Land only a change that improves generated breadth while preserving all of
   the following:
   - accepted step `15` winner stays the canonical `103 / 8`
   - the isolated `single` pocket stays fenced
   - the `small_cluster` does not regress
   - no stronger-than-canonical lifted terminal becomes live
5. Keep the focused guardrail slice green:
  - `current_claim_step_fifteen_partial_prefix_wall_stays_on_four_early_temporal_prefix_families`
  - `current_claim_step_fifteen_remaining_two_partial_prefix_wall_stays_on_nine_clause_zero_one_pairings`
  - `current_claim_step_fifteen_remaining_two_partial_prefix_wall_sits_on_claim_next_bridge_and_reference_clause_four_families`
  - `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_four_pressure_on_claim_next_bridge_per_clause_zero_one_pairing`
  - `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_two_pressure_on_claim_variants_under_the_live_clause_zero_one_pairs`
  - `current_claim_step_fifteen_clause_one_demo_eventually_codomain_exact_pocket_reland_stays_a_negative_control`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_exact_pocket_reland_stays_a_negative_control`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_stays_a_tradeoff_control`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_tradeoff_control_splits_evenly_across_three_clause_two_sheets`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_tradeoff_control_splits_each_clause_two_sheet_as_three_by_three_and_two_by_two_bridge_cells`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_stays_a_tradeoff_control`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_stays_a_tradeoff_control`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_stays_a_tradeoff_control`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_stays_a_tradeoff_control`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_variant_pair_stays_a_tradeoff_control`
  - `current_claim_step_fifteen_exact_claim_variant_pair_tradeoff_delta_outside_the_claim_next_bridge_side_probe_lives_on_two_clause_four_reference_remaining_three_prefixes`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_neutral_control`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_tradeoff_control`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_tradeoff_control`
  - `current_claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  - `current_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  - `current_claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  - `current_claim_step_fifteen_clause_five_side_pocket_injects_on_exact_remaining_two_mismatch_zero_bridge_slice`
  - `current_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice_stays_a_negative_control`
  - `current_claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  - `current_claim_step_fifteen_pruned_terminal_prefixes_match_direct_exact_assessment`
  - `current_claim_step_fifteen_exact_prunes_split_into_zero_admitted_families`
  - `current_claim_step_fifteen_zero_admitted_prunes_reduce_to_disconnect_and_trivial_derivability`
  - `current_claim_step_fifteen_zero_admitted_connectivity_surface_reports_reanchor_prefix_progress`
  - `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`
  - `current_claim_step_fifteen_small_cluster_relief_clears_summary_prunes_while_three_single_bucket_prunes_remain`
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_families_only_keep_reference_terminals_live`
  - `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_still_fence_unsafe_lifted_terminals`
  - `connectivity_accepts_clause_four_demo_sharp_codomain_on_claim_safe_clause_zero_one_surface_under_override`
  - `connectivity_accepts_clause_four_demo_sharp_bridge_on_claim_safe_clause_zero_one_surface_under_override`
  - `connectivity_accepts_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_only_on_the_exact_anchor_eleven_side_pocket_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`
  - `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_from_reopening_lifted_terminals_even_under_override`
  - `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_clause_four_reference_sheet_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_reference_terminal_only_even_under_override`
  - `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_flat_clause_two_sheet_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_reference_terminal_only_even_under_override`
  - `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_sharp_clause_two_sheet_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_sharp_sheet_reference_terminal_only_even_under_override`
  - `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_pair_clause_two_sheets_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_variant_pair_reference_terminal_only_even_under_override`
  - `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_variant_pair_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_variant_pair_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_pair_clause_two_sheets_even_under_claim_next_bridge_side_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_even_under_claim_next_bridge_side_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_variant_pair_reference_terminal_only_even_under_override`
  - `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_exact_claim_variant_pair_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_exact_claim_variant_pair_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_pair_clause_two_sheets_even_under_clause_five_reference_on_clause_four_reference_tail_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_families_closed_on_clause_four_reference_tail_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_exact_claim_variant_pair_reference_terminal_only_even_under_override`
  - `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_pair_clause_two_sheets_even_under_clause_five_claim_flat_codomain_on_clause_four_reference_tail_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_closed_on_clause_four_reference_tail_even_under_claim_flat_codomain_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_reference_terminal_only_even_under_override`
  - `connectivity_accepts_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_outside_exact_claim_pair_clause_two_sheets_even_under_clause_five_claim_next_codomain_on_clause_four_reference_tail_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_closed_on_clause_four_reference_tail_even_under_claim_next_codomain_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_reference_terminal_only_even_under_override`
  - `connectivity_accepts_clause_five_demo_sharp_domain_on_the_exact_remaining_two_mismatch_zero_bridge_slice`
  - `connectivity_accepts_clause_five_demo_flat_codomain_on_the_exact_remaining_two_mismatch_zero_bridge_slice`
  - `connectivity_keeps_clause_five_remaining_two_mismatch_one_bridge_slice_outside_historical_reanchor_even_under_override`
  - `connectivity_accepts_clause_five_demo_sharp_domain_on_claim_safe_clause_zero_one_surface_under_override`
  - `connectivity_accepts_clause_five_demo_flat_codomain_on_claim_safe_clause_zero_one_surface_under_override`
6. Only after the local repair is green and parity-clean, launch the next
   clean full-profile rerun beyond `v12`.
7. Immediately refresh compare, benchmark, and certification after that rerun.
8. Reassess whether the remaining `3` incumbent families still need separate
   work after the partial-prefix wall moves.

## Not The Next Move

- another rerun setup pass first
- a step-`1`-first theory slice
- blanket same-primary relief
- exact-family same-primary relief
- subset-local same-primary relief
- clause-`1` `demo_eventually_codomain` exact-pocket reland
- clause-`0` `claim_flat_domain` plus clause-`1` `demo_flat_codomain`
  exact-pocket reland
- clause-`0` `reference` plus clause-`1` `demo_flat_codomain`
  live-claim-bridge reopening
- the narrower clause-`4` `reference`-sheet reopening on that same
  `reference + demo_flat_codomain` branch
- the exact clause-`2` `claim_flat_domain` sheet reopening on that same
  `reference + demo_flat_codomain` branch
- the exact clause-`2` `claim_sharp_codomain` sheet reopening on that same
  `reference + demo_flat_codomain` branch
- the combined exact clause-`2` claim-variant pair reopening on that same
  `reference + demo_flat_codomain` branch
- the already-pinned exact clause-`4` `claim_next_bridge`-side reopening on
  that same combined claim-variant pair surface
- the exact clause-`5` `reference` follow-on on the clause-`4` `reference`
  tail of that same combined claim-variant pair surface
- the exact clause-`5` `claim_flat_codomain` follow-on on the clause-`4`
  `reference` tail of that same combined claim-variant pair surface
- the exact clause-`5` `claim_next_codomain` follow-on on the clause-`4`
  `reference` tail of that same combined claim-variant pair surface
- clause-`3` `anchor-11` exact-argument widening onto the broader
  clause-`0` / clause-`1` claim surface while clause `2` stays `reference`
- clause-`4` `demo_sharp_codomain` reopening onto the claim-safe
  clause-`0` / clause-`1` surface
- clause-`4` `demo_sharp_bridge` reopening onto the claim-safe clause-`0` /
  clause-`1` surface
- the combined broad clause-`4` side-pocket reopening onto the claim-safe
  clause-`0` / clause-`1` surface
- clause-`5` side-pocket broadening onto the claim-safe clause-`0` /
  clause-`1` surface
- the exact remaining-two mismatch-`0` bridge-slice clause-`5` reopening
- treating the dominant remaining-two wall as an unlabeled early blur again
- treating the dominant remaining-two wall as if the old demo-only clause-`4`
  bridge pockets were still the dominant slice
- treating the remaining delta to the broader exact claim-pair tradeoff as if
  it still lived inside the already-captured `45`-prefix mismatch-`1`
  branch instead of the now-frozen two-prefix clause-`4` `reference`
  remaining-three tail
- raw position-`0` reland
- another broad clause-`4` or clause-`5` reopening
- raw reland of the unsafe lifted `89 / 8` shell
- raw reland of the widened step-`13` negative controls
  `[3,5,3,3,5,1,1]` or `[5,1,3,3,5,3,3]`

## Success For This Slice

- the clean step-`15` partial-prefix wall is narrower than `553`
- the canonical accepted path stays fixed through step `15`
- the isolated `single` pocket and unsafe lifted shell both stay fenced
- a new clean full-profile rerun beyond `v12` is justified and ready to launch
