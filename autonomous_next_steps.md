# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-08

This file owns the single active work order for `desktop_claim_shadow`.

## Contract

- Keep exactly one active objective and one current hypothesis here.
- Move completed or ruled-out probes to
  [autonomous_ledger.md](autonomous_ledger.md).
- Do not restate medium-horizon phases or historical context here. Use
  [autonomous_plan.md](autonomous_plan.md) and
  [autonomous_progress.md](autonomous_progress.md).

## Objective

Narrow the remaining clean stored step-`15` partial-prefix wall on top of
canonical `v12` and the matched local `4331` guardrail surface.

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
- The next honest slice should therefore start with the mismatch-`0`
  claim-domain surface before revisiting the smaller claim-safe mismatch-`1`
  tier or the `reference / reference` tails.
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
- Any remaining mismatch-`0` leverage therefore has to stay below that broad
  demo-flat reopening rather than relanding either of the clause-`4` halves
  again.
- Any remaining mismatch-`0` leverage also has to stay below the whole-tier
  parent exact-bound decision. The next repair has to live one layer deeper,
  inside the existing clause-`4` / clause-`5` split or remaining-one exact
  summary behavior on those live mismatch-`0` pairings.
- Any remaining mismatch-`0` leverage is therefore no longer on a
  connectivity-only relocalization of either clause-`4` half. The next honest
  slice should move below clause-`4` relocalizations and test remaining-one
  exact-summary behavior under the same live pairings before reopening
  clause-`5` families.
- Do not spend another turn on clause-`5` tail reopenings or exact
  remaining-two clause-`5` bridge-slice reopenings or on another exact
  claim-flat / claim-sharp single-sheet restatement. Those surfaces are
  already exhausted or explicitly ruled out.

## Do This Next

1. Start from the stored `v12` certificate, compare report, benchmark bundle,
   and `reports/steps/step-15-live.ndjson`, but treat the exact-claim mismatch-`1`
   tradeoff ladder as already frozen at `57 -> 51 -> 45`.
2. Probe only surfaces outside the exact claim-pair mismatch-`1`
   `reference + demo_flat_codomain` branch, or at minimum outside any reland
   that only swaps which claim clause-`2` sheet keeps the `21`-capture share
   of the same `51`-capture tradeoff.
3. Prefer the mismatch-`0` claim-domain surface first:
   - six pairings totaling `252`
   - do not redirect the whole remaining-two mismatch-`0` claim-domain tier
     from `CannotClearBar` to `Unknown` at the parent
     `exact_partial_prefix_bound_decision(...)` level; that exploratory relief
     only releases the full `312` first-mismatch-`0` family and explodes
     `small_cluster`
   - do not reopen broad clause-`1` `demo_flat_codomain` across both
     mismatch-`0` claim-domain pairings again; that control is now frozen at
     `4985 / 667 / 2757`
   - do not reland the mismatch-`0` clause-`4` `claim_next_bridge`-half
     connectivity-only relocalization again; that smaller control is now
     frozen at `4829 / 671 / 2793`
   - do not reland the mismatch-`0` clause-`4` `reference` half
     connectivity-only relocalization again; that sharper control is now
     frozen at `4697 / 691 / 2829`
   - if mismatch-`0` is revisited, localize one layer deeper beneath the now
     exhausted clause-`4` `claim_next_bridge / reference` split, starting with
     remaining-one exact-summary behavior under those live pairings before any
     narrower clause-`5` `reference / claim_flat_codomain / claim_next_codomain`
     family reopening
   - both mismatch-`0` clause-`4` halves are now ruled out as negative
     controls, so the next revisit should start with remaining-one exact-summary
     behavior under the same live pairings
   - only after that, consider the smaller claim-safe mismatch-`1` tier at
     `84`
   - leave the `reference / reference` tails at `54` below both of those
     unless a narrower probe changes the ordering
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
- broad clause-`1` `demo_flat_codomain` reopenings across the full
  mismatch-`0` claim-domain surface
- another connectivity-only relocalization of the mismatch-`0` clause-`4`
  `claim_next_bridge` half
- another connectivity-only relocalization of the mismatch-`0` clause-`4`
  `reference` half
- whole-tier remaining-two mismatch-`0` claim-domain
  `CannotClearBar -> Unknown` relief at the parent
  `exact_partial_prefix_bound_decision(...)` layer
- broader clause-`4` `reference`-sheet reopenings
- claim-safe clause-`4` or clause-`5` reopenings
- raw clause-`3` `anchor-11` widening
- blanket, exact-family, or subset-local same-primary relief

## Success For This Slice

- the clean step-`15` partial-prefix wall is narrower than `553`
- the canonical accepted path stays fixed through step `15`
- the isolated `single` pocket and unsafe lifted shell both stay fenced
- a new clean full-profile rerun beyond `v12` is justified and ready to launch
