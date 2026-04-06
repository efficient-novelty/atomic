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
- That same dominant remaining-two wall is now frozen one layer deeper again by
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_sits_on_claim_next_bridge_and_reference_clause_four_families`:
  - mismatch position `0` now splits only across clause-`4`
    `claim_next_bridge = 48 / 48 / 48` and clause-`4` `reference = 36 / 36 / 36`
    against clause-`5` `claim_flat_codomain`, `claim_next_codomain`, and
    `reference`
  - mismatch position `1` now stays on the same clause-`4` / clause-`5`
    family grid at `27 / 27 / 27` for clause-`4` `claim_next_bridge` and
    `22 / 22 / 20` for clause-`4` `reference`
  - the mismatch-`2` / mismatch-`3` tail stays much smaller and is the only
    place where the old demo-only clause-`4` bridge pockets still appear at
    all
  - so the next honest slice is no longer a generic early claim wall or
    another demo-bridge reland, but the live clause-`4`
    `claim_next_bridge` plus `reference` families on top of the dominant
    clause-`0` / clause-`1` claim surface
- That same dominant remaining-two wall is now frozen one layer more concretely
  again by
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_four_pressure_on_claim_next_bridge_per_clause_zero_one_pairing`
  plus
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_two_pressure_on_claim_variants_under_the_live_clause_zero_one_pairs`:
  - every mismatch-`0` clause-`0` / clause-`1` pairing now keeps the same
    clause-`4` split, `claim_next_bridge = 24` versus `reference = 18`
  - mismatch-`1` keeps that same `24 / 18` split on
    `reference + claim_sharp_codomain` and `reference + claim_next_codomain`,
    while the larger `reference + demo_flat_codomain` side still stays on the
    same live claim families at `33 / 28`
  - the older clause-`4` demo pockets now stay confined to the mismatch-`2`
    tail at only `18 / 4 / 4 / 16`, and mismatch-`3` stays only `6 / 6`
    across `claim_next_bridge` and `reference`
  - the same dominant mismatch-`0` / mismatch-`1` pairings now also keep
    clause `2` on the two current claim variants ahead of the reference
    continuation:
    `15 / 15 / 12` on the regular pairings and `23 / 23 / 15` on the larger
    `reference + demo_flat_codomain` side
  - so the next repair still should not reopen clause-`3` widening, hidden
    demo-only clause-`2` drift, or another broad clause-`4` omitted-side
    reland; it should work directly against the live clause-`4`
    `claim_next_bridge` plus `reference` split on the dominant clause-`0` /
    clause-`1` pairings
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
- A narrower clause-`0` `reference` plus clause-`1` `demo_flat_codomain`
  live-claim-bridge reopening was also checked under a scoped
  connectivity-only override and is now pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_stays_a_tradeoff_control`
  plus the matching connectivity override tests:
  - it broadens the landed clause-`1` `demo_flat_codomain` qualifier only
    across the live clause-`4` / clause-`5` claim families beneath
    clause-`0` `reference`
  - it lifts the local late surface from `4331` to `4523`
  - it narrows the clean partial-prefix wall from `553` to `537` and the
    zero-admitted exact-prune capture from `2271` to `2223`
  - it shrinks the larger mismatch-`1`
    `reference + demo_flat_codomain` branch from `61` captured prefixes on
    clause-`4` `33 / 28` down to `45` on clause-`4` `27 / 18`
  - it keeps the isolated `single` pocket and the residual `3`
    incumbent-dominance prunes unchanged
  - but it widens the `small_cluster` from `3132 / 522 / 522 / 0` to
    `3324 / 554 / 554 / 0`
  - newer executable tradeoff-branch freezes now pin that same `45`-capture
    branch one layer deeper:
    `current_claim_step_fifteen_clause_one_demo_flat_codomain_tradeoff_control_splits_evenly_across_three_clause_two_sheets`
    and
    `current_claim_step_fifteen_clause_one_demo_flat_codomain_tradeoff_control_splits_each_clause_two_sheet_as_three_by_three_and_two_by_two_bridge_cells`
  - the branch now splits evenly across clause-`2`
    `claim_flat_domain = 15`, `claim_sharp_codomain = 15`,
    `reference = 15`
  - each clause-`2` sheet then splits uniformly as clause-`4`
    `claim_next_bridge = 3 / 3 / 3` plus clause-`4`
    `reference = 2 / 2 / 2` across clause-`5`
    `claim_flat_codomain`, `claim_next_codomain`, and `reference`
  - so it is a tradeoff control, not the landed repair; the next slice should
    isolate that narrower `reference + demo_flat_codomain` branch without
    taking the full `small_cluster` reopening, starting with one exact
    `15`-count clause-`2` sheet or the exact `18`-count clause-`4`
    `reference` sheet before touching the larger `27`-count clause-`4`
    `claim_next_bridge` side
- A narrower clause-`4` `reference`-sheet reopening on that same
  `reference + demo_flat_codomain` branch was then checked under a second
  scoped connectivity-only override and is now pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_four_reference_sheet_stays_a_tradeoff_control`
  plus the matching connectivity override tests:
  - it broadens that same branch only while clause `4` stays on the exact
    reference sheet and clause `5` ranges across `reference`,
    `claim_flat_codomain`, and `claim_next_codomain`
  - it lifts the local late surface from `4331` to `4379`
  - it narrows the clean partial-prefix wall from `553` to `549` and the
    zero-admitted exact-prune capture from `2271` to `2259`
  - it shrinks the larger mismatch-`1`
    `reference + demo_flat_codomain` branch from `61` captured prefixes on
    clause-`4` `33 / 28` down only to clause-`4` `33 / 24` / `57`
  - it keeps the isolated `single` pocket and the residual `3`
    incumbent-dominance prunes unchanged
  - but it still widens the `small_cluster` from `3132 / 522 / 522 / 0` to
    `3180 / 530 / 530 / 0`
  - the reopened cells stay even narrower than the broader tradeoff control:
    only four captured clause-`4` `reference` cells reopen, and only on the
    `claim_flat_domain` plus `claim_sharp_codomain` clause-`2` sheets, while
    the full `33`-count clause-`4` `claim_next_bridge` side stays unchanged
  - so it is also only a tradeoff control rather than the landed repair, and
    the next honest slice should now move to one exact `15`-count
    clause-`2` sheet before touching the larger clause-`4`
    `claim_next_bridge` side again
- A narrower exact clause-`2` `claim_flat_domain` sheet reopening on that
  same `reference + demo_flat_codomain` branch was then checked under a third
  scoped connectivity-only override and is now pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_clause_two_claim_flat_sheet_stays_a_tradeoff_control`
  plus the matching connectivity override tests:
  - it broadens that same branch only while clause `2` stays on the exact
    `claim_flat_domain` sheet and clauses `4` and `5` stay on the live claim
    bridge families
  - it lifts the local late surface from `4331` to `4412`
  - it narrows the clean partial-prefix wall from `553` to `544` and the
    zero-admitted exact-prune capture from `2271` to `2250`
  - it shrinks the larger mismatch-`1`
    `reference + demo_flat_codomain` branch from `61` captured prefixes on
    clause-`4` `33 / 28` down to clause-`4` `30 / 21` / `51`
  - it keeps the isolated `single` pocket and the residual `3`
    incumbent-dominance prunes unchanged
  - but it still widens the `small_cluster` from `3132 / 522 / 522 / 0` to
    `3216 / 536 / 536 / 0`
  - the reopened bridge cells now reland only the exact
    `claim_flat_domain` clause-`2` sheet at the full `3 / 3 / 3` plus
    `2 / 2 / 2` split, while the sibling `claim_sharp_codomain` and
    `reference` sheets stay higher at `21` and `15`
  - so it is also only a tradeoff control rather than the landed repair, but
    it is a stronger narrowing than the clause-`4` `reference`-sheet probe;
    the next honest slice should now move to the sibling exact
    clause-`2` `claim_sharp_codomain` sheet before touching the
    `reference` sheet or the larger clause-`4` `claim_next_bridge` side again
- A local clause-`3` `anchor-11` exact-argument widening onto the broader
  clause-`0` / clause-`1` claim surface while clause `2` stayed `reference`
  was also checked and reverted:
  - the clean step-`15` partial-prefix wall stayed pinned at `553`, including
    the same executable remaining-two nine-pair split
  - but summary-stage incumbent captures reopened from `0` to `72`
  - so widening the clause-`3` exact pocket onto that broader early claim
    surface is also ruled out as the next repair
- A local clause-`5` side-pocket broadening onto the claim-safe clause-`0` /
  clause-`1` surface was also checked under scoped overrides and is now pinned
  by
  `current_claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  plus the matching connectivity override tests:
  - it lifted the local late surface from `4331` to `4779`
  - it widened the `small_cluster` from `3132 / 522 / 522 / 0` to
    `3516 / 586 / 586 / 0`
  - it kept the isolated `single` pocket and the residual `3`
    incumbent-dominance prunes unchanged
  - but it widened the clean partial-prefix wall from `553` to `585` and the
    zero-admitted exact-prune capture from `2271` to `2367`
  - so that broader clause-`5` reland is also ruled out as the next move
- Clause-`4` claim-safe reopenings have now also been checked under scoped
  overrides and are now pinned as negative controls:
  - the broad clause-`4` `demo_sharp_codomain` plus `demo_sharp_bridge`
    reopening on the claim-safe clause-`0` / clause-`1` surface is now pinned
    by
    `current_claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
    plus the matching connectivity override tests:
    - it lifted the local late surface from `4331` to `4843`
    - it widened the `small_cluster` from `3132 / 522 / 522 / 0` to
      `3516 / 586 / 586 / 0`
    - it kept the isolated `single` pocket and the residual `3`
      incumbent-dominance prunes unchanged
    - but it widened the clean partial-prefix wall from `553` to `617` and
      the zero-admitted exact-prune capture from `2271` to `2463`
  - the narrower clause-`4` `demo_sharp_codomain`-only reopening is now pinned
    by
    `current_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`,
    and the matching clause-`4` `demo_sharp_bridge`-only reopening is now
    pinned by
    `current_claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  - each narrower half still lifted the local late surface only to `4587`,
    widened the `small_cluster` to `3324 / 554 / 554 / 0`, kept the isolated
    `single` pocket and residual `3` incumbent prunes unchanged, and still
    widened the clean partial-prefix wall to `585` with zero-admitted capture
    widened to `2367`
  - so even the narrower clause-`4` claim-safe reopenings are now ruled out;
    the next landed repair must stay narrower than claim-safe clause-`4` or
    clause-`5` side-pocket broadening
- A narrower clause-`5` reland on only the exact remaining-two mismatch-`0`
  bridge slice was also checked under scoped overrides and is now pinned by
  `current_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice_stays_a_negative_control`
  plus the matching injector and connectivity override tests:
  - it reopened the clause-`4` `demo_sharp_bridge` prerequisite only across
    the six mismatch-`0` clause-`0` / clause-`1` pairings before admitting the
    two existing clause-`5` side clauses there
  - it lifted the local late surface from `4331` to `4691`
  - it widened the `small_cluster` from `3132 / 522 / 522 / 0` to
    `3420 / 570 / 570 / 0`
  - it kept the isolated `single` pocket and the residual `3`
    incumbent-dominance prunes unchanged
  - but it widened the clean partial-prefix wall from `553` to `589`
  - so that narrower mismatch-`0` bridge-slice reland is also ruled out as
    the next move
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
- Do not reland the narrower clause-`0` `reference` plus clause-`1`
  `demo_flat_codomain` live-claim-bridge reopening; that probe lifted local
  breadth to `4523` and narrowed the clean wall to `537`, but it also
  widened the `small_cluster` to `3324 / 554 / 554 / 0`.
- Do not reland the narrower clause-`4` `reference`-sheet reopening on that
  same `reference + demo_flat_codomain` branch first; that probe lifted local
  breadth only to `4379`, narrowed the clean wall only to `549`, left the
  larger clause-`4` `claim_next_bridge` side untouched at `33`, and still
  widened the `small_cluster` to `3180 / 530 / 530 / 0`.
- Do not reland the exact clause-`2` `claim_flat_domain` sheet reopening on
  that same `reference + demo_flat_codomain` branch first; that probe lifted
  local breadth to `4412`, narrowed the clean wall to `544`, and cut
  zero-admitted capture to `2250`, but it still widened the `small_cluster`
  to `3216 / 536 / 536 / 0`.
- Do not reland the raw clause-`3` `anchor-11` exact-argument widening onto
  the broader clause-`0` / clause-`1` claim surface while clause `2` stays
  `reference`; that probe left the `553` wall unchanged and reopened `72`
  summary-stage incumbent captures.
- Do not reland the clause-`5` side-pocket broadening onto the claim-safe
  clause-`0` / clause-`1` surface; that probe lifted local breadth to `4779`
  but widened the clean wall to `585`.
- Do not reland the clause-`4` `demo_sharp_codomain` or
  `demo_sharp_bridge` reopenings onto the claim-safe clause-`0` / clause-`1`
  surface; each narrower half only lifted local breadth to `4587` while
  widening the clean wall to `585`, and the combined reopening widened it to
  `617`.
- Do not reland the exact remaining-two mismatch-`0` bridge-slice clause-`5`
  reopening; that narrower probe still lifted local breadth to `4691` but
  widened the clean wall to `589`.
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
  - `current_claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  - `current_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  - `current_claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  - `current_claim_step_fifteen_clause_five_side_pocket_injects_on_exact_remaining_two_mismatch_zero_bridge_slice`
  - `current_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice_stays_a_negative_control`
  - `current_claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  - `current_claim_step_fifteen_pruned_terminal_prefixes_match_direct_exact_assessment`
  - `current_claim_step_fifteen_exact_prunes_split_into_zero_admitted_families`
  - `current_claim_step_fifteen_zero_admitted_connectivity_surface_reports_reanchor_prefix_progress`
  - `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`
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
  - `connectivity_accepts_clause_five_demo_sharp_domain_on_the_exact_remaining_two_mismatch_zero_bridge_slice`
  - `connectivity_accepts_clause_five_demo_flat_codomain_on_the_exact_remaining_two_mismatch_zero_bridge_slice`
  - `connectivity_keeps_clause_five_remaining_two_mismatch_one_bridge_slice_outside_historical_reanchor_even_under_override`
  - `connectivity_accepts_clause_five_demo_sharp_domain_on_claim_safe_clause_zero_one_surface_under_override`
  - `connectivity_accepts_clause_five_demo_flat_codomain_on_claim_safe_clause_zero_one_surface_under_override`

## Immediate Direction

Use the executable wall split plus the new
`reference + demo_flat_codomain` tradeoff control to attack the clean `v12`
step-`15` partial-prefix wall on top of the matched `4331` surface. Start
with the dominant remaining-two clause-`0` / clause-`1` side before
reopening the smaller clause-`2` / clause-`3` tail, keep the next change
prefix-local and reference-terminal-safe, and work first on the six
mismatch-`0` current claim-generic clause-`0` / clause-`1` pairings and the
three mismatch-`1` clause-`1` pairs. The regular pairings still sit at
clause-`4` `24 / 18`, the larger `reference + demo_flat_codomain` side still
sits at `33 / 28` on the canonical surface, and the clause-`2` split still
keeps the pressure on the two current claim variants rather than on a hidden
demo-only clause-`2` reopening, but the new tradeoff control now shows that
reopening only that larger mismatch-`1` branch can cut clause-`4` pressure
there to `27 / 18`, shrink the clean wall to `537`, cut zero-admitted capture
to `2223`, and lift local breadth to `4523` while leaving the isolated
`single` pocket and residual `3` prunes unchanged. Because that same probe
also widens the `small_cluster` to `3324 / 554 / 554 / 0`, the next landed
repair should isolate those escaping `16` captures on the
`reference + demo_flat_codomain` branch without relanding the full reopening.
The new executable tradeoff-branch freezes now show exactly how to do that
more narrowly: the `45` captured prefixes break into three equal clause-`2`
sheets at `15` each, and each sheet then splits uniformly as clause-`4`
`claim_next_bridge = 3 / 3 / 3` plus clause-`4` `reference = 2 / 2 / 2`
across the three clause-`5` families. The narrower clause-`4`
`reference`-sheet tradeoff is now also executable and pinned: it lifts local
breadth only to `4379`, narrows the clean wall only to `549`, cuts
zero-admitted capture only to `2259`, shrinks the larger mismatch-`1`
branch only to clause-`4` `33 / 24` / `57`, and still widens the
`small_cluster` to `3180 / 530 / 530 / 0`, so it is useful evidence but not
the landed repair. A newer exact clause-`2` `claim_flat_domain` sheet probe
is now pinned too: it lifts local breadth to `4412`, narrows the clean wall
to `544`, cuts zero-admitted capture to `2250`, shrinks that same branch to
clause-`4` `30 / 21` / `51`, and still widens the `small_cluster` to
`3216 / 536 / 536 / 0`, so it is also only a tradeoff control. The next live
slice should therefore move to the sibling exact
clause-`2` `claim_sharp_codomain` sheet rather than another clause-`4`
`reference`-sheet-only reopening,
and should stay narrower than another clause-`3` widen-first probe, the
reverted clause-`1` broadenings at `4466 / 626`, the claim-safe
clause-`4` reopenings at `4587 / 585` or `4843 / 617`, the broader
clause-`5` reopening at `4779 / 585`, or the narrower mismatch-`0`
clause-`5` reopening at `4691 / 589`. Rerun only after the new local slice
is green, and keep step `1` explicit unless that next stored bundle changes
it directly.
