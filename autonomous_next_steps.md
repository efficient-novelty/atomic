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
- The best remaining leverage is therefore no longer on the clause-`4`
  `reference` side or on the reference clause-`2` sheet of the dominant
  remaining-two clause-`0` / clause-`1` mismatch-`1` surface.
- The next honest slice should stay on the live clause-`4`
  `claim_next_bridge` half or another still-unexplained family outside that
  exhausted reference-side relocalization.
- Do not spend another turn on clause-`5` tail reopenings or exact
  remaining-two clause-`5` bridge-slice reopenings. Those surfaces are
  already exhausted or explicitly ruled out.

## Do This Next

1. Re-read the stored `v12` certificate, compare report, benchmark bundle, and
   `reports/steps/step-15-live.ndjson` before touching search code again.
2. Re-read the new exact claim-pair clause-`4` `reference`-side regressions
   before changing search code again; they prove that the older broader
   clause-`4` `reference`-sheet tradeoff already lives entirely on the exact
   claim clause-`2` pair.
3. Probe only surfaces that differ from both the exact claim-pair
   clause-`4` `claim_next_bridge` tradeoff (`4445 / 539 / 2241`) and the exact
   claim-pair clause-`4` `reference` tradeoff (`4379 / 549 / 2259`).
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
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_stays_a_tradeoff_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_claim_next_bridge_side_on_exact_claim_variant_pair_stays_a_tradeoff_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_side_on_exact_claim_variant_pair_stays_a_tradeoff_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_reference_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_neutral_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_flat_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_tradeoff_control`
- `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_five_claim_next_codomain_on_clause_four_reference_tail_on_exact_claim_variant_pair_stays_a_tradeoff_control`
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
- broader clause-`4` `reference`-sheet reopenings
- claim-safe clause-`4` or clause-`5` reopenings
- raw clause-`3` `anchor-11` widening
- blanket, exact-family, or subset-local same-primary relief

## Success For This Slice

- the clean step-`15` partial-prefix wall is narrower than `553`
- the canonical accepted path stays fixed through step `15`
- the isolated `single` pocket and unsafe lifted shell both stay fenced
- a new clean full-profile rerun beyond `v12` is justified and ready to launch
