# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-06

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
  - so the next honest repair target is still the current clause-`0` /
    clause-`1` claim surface, not another broad omitted-side reopening
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
4. Land only a change that improves generated breadth while preserving all of
   the following:
   - accepted step `15` winner stays the canonical `103 / 8`
   - the isolated `single` pocket stays fenced
   - the `small_cluster` does not regress
   - no stronger-than-canonical lifted terminal becomes live
5. Keep the focused guardrail slice green:
   - `current_claim_step_fifteen_partial_prefix_wall_stays_on_four_early_temporal_prefix_families`
   - `current_claim_step_fifteen_remaining_two_partial_prefix_wall_stays_on_nine_clause_zero_one_pairings`
   - `current_claim_step_fifteen_clause_one_demo_eventually_codomain_exact_pocket_reland_stays_a_negative_control`
   - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_exact_pocket_reland_stays_a_negative_control`
   - `current_claim_step_fifteen_pruned_terminal_prefixes_match_direct_exact_assessment`
   - `current_claim_step_fifteen_exact_prunes_split_into_zero_admitted_families`
   - `current_claim_step_fifteen_zero_admitted_prunes_reduce_to_disconnect_and_trivial_derivability`
   - `current_claim_step_fifteen_zero_admitted_connectivity_surface_reports_reanchor_prefix_progress`
   - `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`
   - `current_claim_step_fifteen_small_cluster_relief_clears_summary_prunes_while_three_single_bucket_prunes_remain`
   - `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
   - `current_claim_step_fifteen_residual_single_bucket_incumbent_families_only_keep_reference_terminals_live`
   - `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_still_fence_unsafe_lifted_terminals`
   - `connectivity_accepts_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_only_on_the_exact_anchor_eleven_side_pocket_under_override`
   - `connectivity_keeps_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`
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
- treating the dominant remaining-two wall as an unlabeled early blur again
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
