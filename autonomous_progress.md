# Autonomous Claim Lane Progress

Last updated: 2026-04-06

This file is the live operating brief for `desktop_claim_shadow`.
It is intentionally short. Use [autonomous_next_steps.md](autonomous_next_steps.md)
for the exact slice to execute, [autonomous_plan.md](autonomous_plan.md) for the
staged path to signoff, and [autonomous_checklist.md](autonomous_checklist.md)
for the remaining gates.

## Mission

Produce one stored `desktop_claim_shadow` bundle that:

- completes through step `15`
- keeps accepted-hash parity through step `15`
- passes compare, benchmark, and certification from stored evidence

Until that exists, wording stays at `bounded live recovery`.

## Canonical Stored Baseline

- Canonical stored bundle:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v12`
- Clean-tree repo head:
  `d9a1b45a72f4ca2fcfb0a325d1bd6100124b8778`
- Release binary hash:
  `4b6c68ef394ae8528452571375e530dad7841c91ccd4edf20fab0b83dcdea38e`
- Stored run status:
  - `status = "completed"`
  - `completed_step = 15`
  - `active_step = 16`
  - `active_band = 8`
  - `frontier_epoch = 12`
  - `dirty_tree = false`

## What Is Already Earned

- Stored `v12` compare is ready.
- Stored `v12` accepted hashes still match guarded through step `15`.
- Stored benchmark is refreshed across `v11` and `v12`.
- Stored `v12` certification still fails only on breadth:
  - step `1 = 546 / 2144`
  - step `15 = 4331 / 5000`
- Stored step `15` has now lifted cleanly from `3972` on `v11` to `4331` on
  `v12`, while stored steps `10` through `14` remain hits.

## Current Canonical Late Surface

- The clean stored late chain and the local guardrail now match:
  - step `13`: `[5,1,3,3,5,3,2]` / raw `1350` / generated `2320`
  - step `14`: accepted `62 / 9`, generated `12027`
  - step `15`: accepted `DCT 103 / 8`, generated `4331`
- Current step-`15` pressure is:
  - partial-prefix bar failures: `553`
  - incumbent-dominance prunes: `3`
  - `small_cluster`: `3132 / 522 / 522 / 0`
  - fenced `single` bucket: `1` fully scored non-winner plus `3` residual prunes
    at overshoot `115657 / 21112`
  - executable partial-prefix wall split:
    - remaining-two prefixes: `451`
    - remaining-three prefixes: `102`
    - first mismatch positions: clause `0 = 312`, clause `1 = 177`,
      clause `2 = 50`, clause `3 = 14`
    - dominant remaining-two slice: clause `0 = 252`, clause `1 = 145`

## Residual Step-15 Diagnosis

- The old broad `small_cluster` incumbent wall is still closed; the remaining
  late miss is now dominated by the clean step-`15` partial-prefix wall, and
  that wall is no longer notes-only:
  - `current_claim_step_fifteen_partial_prefix_wall_stays_on_four_early_temporal_prefix_families`
    now freezes the real `553`-prune split as `451` remaining-two plus `102`
    remaining-three prefixes
  - it localizes first mismatch pressure to clause positions `0..3` only,
    with the dominant live slice now on remaining-two clause-`0` / clause-`1`
    prefixes at `252 / 145`
- The dominant remaining-two side of that wall is now pinned one layer more
  concretely by
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_stays_on_nine_clause_zero_one_pairings`:
  - mismatch position `0` is exactly six current claim-generic clause-`0` /
    clause-`1` pairings, all at `42` captures each:
    `claim_flat_domain` or `claim_eventual_domain` crossed with clause-`1`
    `reference`, `claim_sharp_codomain`, or `claim_next_codomain`
  - mismatch position `1` keeps clause `0` at `reference` and is exactly
    three clause-`1` pairings:
    `claim_sharp_codomain = 42`, `claim_next_codomain = 42`,
    `demo_flat_codomain = 61`
  - the remaining-two tail at mismatch positions `2` and `3` stays only the
    narrow `reference/reference` continuation at `42` and `12`
  - so the main blocker is still the current claim-generic clause-`0` /
    clause-`1` surface rather than an undiscovered broad demo-only reopening
- A local clause-`1` `demo_eventually_codomain` exact-pocket reland was
  tested, reverted, and is now pinned by
  `current_claim_step_fifteen_clause_one_demo_eventually_codomain_exact_pocket_reland_stays_a_negative_control`:
  - it lifted the local late surface from `4331` to `4466`
  - it widened the `small_cluster` from `3132 / 522 / 522 / 0` to
    `3156 / 526 / 526 / 0`
  - it kept the isolated `single` pocket and the residual `3`
    incumbent-dominance prunes unchanged
  - but it widened the clean partial-prefix wall from `553` to `626` and the
    zero-admitted exact-prune capture from `2271` to `2562`
  - so it is now a negative control, not the landed repair
- A broader clause-`0` / clause-`1` flat-pocket reland was also tested under
  scoped test-only overrides and is now pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_exact_pocket_reland_stays_a_negative_control`
  plus the matching connectivity override tests:
  - it broadened the landed clause-`1` `demo_flat_codomain` side pocket onto
    clause-`0` `claim_flat_domain`
  - it again lifted the local late surface from `4331` to `4466`
  - it kept the isolated `single` pocket and the residual `3`
    incumbent-dominance prunes unchanged
  - but it widened the clean partial-prefix wall from `553` to `626`
  - so that broader flat-pocket reland is now ruled out too
- The residual proof-close pressure is still localized to three same-primary
  `103 / 8` families:
  - clause-`0` `claim_flat_domain`
  - clause-`2` `claim_flat_domain` plus anchor-`11` exact-argument
  - clause-`5` `claim_flat_codomain`
- Each of those three families still keeps only the reference terminal live;
  both unsafe lifted `89 / 8` terminals remain structurally connected but
  fenced outside historical reanchor and live connectivity.
- Step `1` remains unchanged and separate from this late-step diagnosis.

## Boundaries That Still Matter

- Do not reopen step `1` first just because `v12` exists; keep it explicit,
  but keep pushing on the cleaner step-`15` evidence first unless a newer
  rerun changes that diagnosis.
- Do not use blanket, exact-family, or subset-local same-primary relief.
  Those probes still unfence extra non-winning `236`-bit terminals.
- Do not reopen the raw lifted `89 / 8` shell.
- Do not reland the broader clause-`0` `claim_flat_domain` plus clause-`1`
  `demo_flat_codomain` exact-pocket reopening.
- Do not reland the raw position-`0`, broad clause-`4`, or broad clause-`5`
  widenings.
- Do not reland the raw step-`13` widened controls
  `[3,5,3,3,5,1,1]` or `[5,1,3,3,5,3,3]`.

## Keep Green

- Stored freezes:
  - `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`
  - `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts`
- Current step-`15` guardrails:
  - `current_claim_step_fifteen_partial_prefix_wall_stays_on_four_early_temporal_prefix_families`
  - `current_claim_step_fifteen_remaining_two_partial_prefix_wall_stays_on_nine_clause_zero_one_pairings`
  - `current_claim_step_fifteen_clause_one_demo_eventually_codomain_exact_pocket_reland_stays_a_negative_control`
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_exact_pocket_reland_stays_a_negative_control`
  - `current_claim_step_fifteen_pruned_terminal_prefixes_match_direct_exact_assessment`
  - `current_claim_step_fifteen_exact_prunes_split_into_zero_admitted_families`
  - `current_claim_step_fifteen_zero_admitted_connectivity_surface_reports_reanchor_prefix_progress`
  - `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_families_only_keep_reference_terminals_live`
  - `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_still_fence_unsafe_lifted_terminals`
  - `connectivity_accepts_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_only_on_the_exact_anchor_eleven_side_pocket_under_override`
  - `connectivity_keeps_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_outside_historical_reanchor_without_the_exact_anchor_eleven_side_pocket_even_under_override`

## Immediate Direction

Use the new executable wall split to attack the clean `v12` step-`15`
partial-prefix wall on top of the matched `4331` surface, start with the
dominant remaining-two clause-`0` / clause-`1` side of that wall before
reopening the smaller clause-`2` / clause-`3` tail, keep the next change
prefix-local and reference-terminal-safe, use the new nine-pair executable
split to work first on the six mismatch-`0` current claim-generic
clause-`0` / clause-`1` pairings and the three mismatch-`1` clause-`1`
pairings before spending another cycle on the smaller reference/reference
tail, do not reland the reverted clause-`1` `demo_eventually_codomain`
exact-pocket probe or the broader clause-`0` `claim_flat_domain` plus
clause-`1` `demo_flat_codomain` exact-pocket reland that are now pinned as
negative controls and widened the wall to `626`, rerun only after the new
local slice is green, and keep step `1` explicit unless that next stored
bundle changes it directly.
