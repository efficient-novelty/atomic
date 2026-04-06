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

## Residual Step-15 Diagnosis

- The old broad `small_cluster` incumbent wall is still closed; the remaining
  late miss is now dominated by the clean step-`15` partial-prefix wall.
- A local clause-`1` `demo_eventually_codomain` exact-pocket reland was
  tested and reverted:
  - it lifted the local late surface from `4331` to `4466`
  - it widened the `small_cluster` from `3132 / 522 / 522 / 0` to
    `3156 / 526 / 526 / 0`
  - it kept the isolated `single` pocket and the residual `3`
    incumbent-dominance prunes unchanged
  - but it widened the clean partial-prefix wall from `553` to `626` and the
    zero-admitted exact-prune capture from `2271` to `2562`
  - so it is now a negative control, not the landed repair
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
- Do not reland the raw position-`0`, broad clause-`4`, or broad clause-`5`
  widenings.
- Do not reland the raw step-`13` widened controls
  `[3,5,3,3,5,1,1]` or `[5,1,3,3,5,3,3]`.

## Keep Green

- Stored freezes:
  - `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`
  - `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts`
- Current step-`15` guardrails:
  - `current_claim_step_fifteen_pruned_terminal_prefixes_match_direct_exact_assessment`
  - `current_claim_step_fifteen_exact_prunes_split_into_zero_admitted_families`
  - `current_claim_step_fifteen_zero_admitted_connectivity_surface_reports_reanchor_prefix_progress`
  - `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_families_only_keep_reference_terminals_live`
  - `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_still_fence_unsafe_lifted_terminals`

## Immediate Direction

Diagnose the clean `v12` step-`15` partial-prefix wall on top of the matched
`4331` surface, keep the next change prefix-local and reference-terminal-safe,
do not reland the reverted clause-`1` `demo_eventually_codomain`
exact-pocket probe that widened the wall to `626`, rerun only after the new
local slice is green, and keep step `1` explicit unless that next stored
bundle changes it directly.
