---
name: pen-atomic
description: Current-state architecture and donor guide for the `pen-atomic` Rust workspace. Use when working on live strict search, `realistic_frontier_shadow`, `demo_breadth_shadow`, `desktop_claim_shadow`, MBTT/kernel design, admissibility, exact selection, reporting, checkpoints, Agda export, or when you need to reconcile current Rust behavior with donor theory and Haskell provenance.
---

# PEN Atomic

## Overview

Use this skill as the current working memory pack for the repository as it
exists today, not as a description of an aspirational rewrite.

Treat these as current repo truths:

- `pen-cli run` and `pen-cli resume` perform live atomic strict search through
  step `15`
- `strict_canon_guarded` remains the authoritative executable lane
- `realistic_frontier_shadow` is a live comparison-backed lane with real online
  prefix search, persisted frontier evidence, and detailed reporting
- `demo_breadth_shadow` is a comparison-backed child of realistic shadow with
  runnable `5m`, `10m`, and `15m` profiles plus stored narrative/event
  artifacts
- `desktop_claim_shadow` now exists as a separate claim-lane scaffold with its
  own configs, narratives, and policy metadata; it uses claim-debt
  admissibility, a claim-generic late surface, and a structural-generic claim
  bucket taxonomy, and the later `kappa 7-9` mutator packs plus claim-path
  exactness rechecks are landed in code/tests; the repo also has
  claim-specific compare/certification/benchmark tooling, richer
  CPU/build/git/binary manifest fingerprints, and incremental failed-run
  artifact persistence; claim runs now record observed-versus-accounted RSS
  gap data, claim auto-worker resolution is memory-aware, claim proof-close
  drops cached evaluated terminal payloads and releases processed retained
  prefix groups once exact certification starts, claim terminal-prefix
  materialization compacts legality-cache reuse plus uncached direct
  materialization, cloned prefix signatures share one serialized exact payload
  allocation, and claim frontier items reuse the shared clause catalog plus
  serialized prefix order key; the current stored canonical bundle is
  clean-tree completed `v12`, compare/certification/benchmark outputs now
  exist beside it, the certificate now also surfaces step-level breadth
  diagnosis from stored step summaries plus late-step live checkpoints, now
  including the full stored step-open pressure signature for failing steps,
  and the live claim blocker is the remaining stored breadth misses on that
  canonical chain: stored `v12` still misses step `1` (`546 / 2144`) and
  step `15` (`4331 / 5000`), while step `10`, step `11`, step `12`,
  step `13`, and step `14` are stored hits; the guarded local step-`11`
  breadth repair and the narrow step-`12` selector repair are both re-earned
  on clean stored evidence, the parity-preserving step-`13` repair is now
  also re-earned on stored evidence at `[5,1,3,3,5,3,2]` / `1350` / `2320`
  with canonical acceptance, and the next operational move is no longer
  notes-only diagnosis but a prefix-local repair against the now-frozen clean
  step-`15` partial-prefix wall on top of that `v12` bundle: a newer
  executable
  `current_claim_step_fifteen_partial_prefix_wall_stays_on_four_early_temporal_prefix_families`
  regression now pins the real `553` bound-prune wall as `451`
  remaining-two plus `102` remaining-three prefixes, with first mismatch
  pressure only at clause positions `0..3` (`312 / 177 / 50 / 14`) and the
  dominant live slice now on remaining-two clause `0` / clause `1`
  prefixes (`252 / 145`), and a newer
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_stays_on_nine_clause_zero_one_pairings`
  regression now sharpens that slice into six exact mismatch-`0`
  current-claim pairings at `42` each plus three mismatch-`1` clause-`1`
  pairings at `42`, `42`, and `61`, and the newer
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_sits_on_claim_next_bridge_and_reference_clause_four_families`
  regression now freezes that same dominant side onto clause-`4`
  `claim_next_bridge` plus clause-`4` `reference`, with clause-`5`
  staying split only across `reference`, `claim_next_codomain`, and
  `claim_flat_codomain`, and the newer
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_four_pressure_on_claim_next_bridge_per_clause_zero_one_pairing`
  plus
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_two_pressure_on_claim_variants_under_the_live_clause_zero_one_pairs`
  regressions now freeze that same dominant side per pairing too: the regular
  mismatch-`0` / mismatch-`1` pairs keep clause-`4`
  `claim_next_bridge` ahead of `reference` at `24 / 18`, the larger
  `reference + demo_flat_codomain` side still stays on the same live
  clause-`4` claim families at `33 / 28`, while a newer scoped tradeoff
  control now shows that reopening only that branch lifts local breadth to
  `4523`, narrows the clean wall to `537`, and shrinks that branch to
  clause-`4` `27 / 18` / `45` captured prefixes, but only by widening the
  `small_cluster` to `3324 / 554 / 554 / 0`; the clause-`2` split keeps the
  pressure on the two current claim variants at `15 / 15 / 12` or
  `23 / 23 / 15` rather than on a hidden demo-only clause-`2` reopening, so
  the next landed repair should start by isolating that narrower
  `reference + demo_flat_codomain` branch rather than with another rerun
  setup pass or a step-`1`-first theory slice; the canonical
  `v11` certificate plus the frozen `step-15-live.ndjson` provenance are now
  pinned by
  `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`,
  and the refreshed benchmark bundle is pinned by
  `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts`,
  so the current breadth miss anatomy is executable in-tree rather than
  notes-only; the narrow step-`15` anchor-`11` repair has now been consumed
  on stored evidence: that nearby clause-`3` exact-argument pocket is isolated
  onto the live claim
  clause-`2` variants only, keeps the lifted anchor-`11` neighbors fenced,
  keeps clause `6` as the local safety boundary, and lifts the stored
  canonical step `15` read from `DCT 103 / 8 / 1794` to
  `DCT 103 / 8 / 3972`; the remaining stored step-`15` miss on canonical
  `v11` is still the residual `468` partial-prefix bar / `242`
  incumbent-dominance surface on top of that repaired chain, but a newer
  local post-`v11` clause-`4` side-pocket repair now reopens only the
  `demo_sharp_codomain` clause-`4` option on the exact anchor-`11`
  exact-argument pocket and counts it as historical reanchor only there,
  lifting the current local guardrail surface again to
  `DCT 103 / 8 / 4004`; a newer local post-`v11` clause-`5` side-pocket
  repair now reopens only the `demo_sharp_domain` clause-`5` option once that
  exact clause-`4` side pocket is already present and counts it as historical
  reanchor only there, lifting the current local guardrail surface again to
  `DCT 103 / 8 / 4030`; a newer local post-`v11` small-cluster relief now
  lands on top of that clause-`5` pocket, keeps live generated prefixes fixed
  at `4030`, collapses local incumbent-dominance pressure from `246` to `3`,
  widens the local `small_cluster` to `2964 / 494 / 494 / 0`, and keeps the
  isolated `single` pocket fenced as one fully scored non-winning terminal
  plus `3` residual single-bucket prunes; a newer local post-`v11`
  clause-`5` demo-flat-codomain side-pocket repair now also consumes only
  that same doubly fenced pocket, lifts the current local guardrail surface
  again to `DCT 103 / 8 / 4056`, keeps local incumbent-dominance pressure at
  `3`, widens the local `small_cluster` again to
  `2988 / 498 / 498 / 0`, and keeps the isolated `single` pocket fenced; a
  newer local post-`v11` clause-`4` demo-sharp-bridge side-pocket repair now
  also consumes only that same exact anchor-`11` pocket stack, lifts the
  current local guardrail surface again to `DCT 103 / 8 / 4088`, raises the
  remaining partial-prefix wall to `476`, widens the local `small_cluster`
  again to `3012 / 502 / 502 / 0`, and keeps the isolated `single` pocket
  fenced; a newer local post-`v11` clause-`5` bridge-pocket stack repair now
  also consumes that same exact anchor-`11` clause-`4` bridge pocket, lifts
  the current local guardrail surface again to `DCT 103 / 8 / 4140`, keeps
  the remaining partial-prefix wall at `476`, widens the local
  `small_cluster` again to `3060 / 510 / 510 / 0`, and keeps the isolated
  `single` pocket fenced; a newer local post-`v11` clause-`1`
  `demo_flat_codomain` side-pocket repair now also consumes that same exact
  anchor-`11` bridge-pocket stack, lifts the current local guardrail surface
  again to `DCT 103 / 8 / 4275`, raises the remaining partial-prefix wall to
  `549`, widens the local `small_cluster` again to
  `3084 / 514 / 514 / 0`, and keeps the isolated `single` pocket fenced; the
  newer exact-pocket clause-`0` / clause-`5` follow-on then lifts the current
  local guardrail again to `DCT 103 / 8 / 4331`, raises the remaining
  partial-prefix wall to `553`, widens the local `small_cluster` to
  `3132 / 522 / 522 / 0`, and still keeps the isolated `single` pocket
  fenced; a newer exploratory clause-`1`
  `demo_eventually_codomain` exact-pocket reland was also tested locally and
  reverted: it lifted the local surface to `DCT 103 / 8 / 4466`, widened the
  `small_cluster` to `3156 / 526 / 526 / 0`, and kept the isolated `single`
  pocket plus residual incumbent pressure unchanged, but it widened the
  partial-prefix wall to `626` and the zero-admitted exact-prune capture to
  `2562`, so it is now another negative control rather than the landed path,
  and that reland is now pinned by
  `current_claim_step_fifteen_clause_one_demo_eventually_codomain_exact_pocket_reland_stays_a_negative_control`;
  a second scoped negative control that broadened the landed clause-`1`
  `demo_flat_codomain` side pocket onto clause-`0` `claim_flat_domain` was
  also tested locally and reverted: it again lifted the local surface to
  `DCT 103 / 8 / 4466`, kept the isolated `single` pocket plus residual
  incumbent pressure unchanged, but it again widened the partial-prefix wall
  to `626`, so it is also ruled out rather than the landed path, and that
  reland is now pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_exact_pocket_reland_stays_a_negative_control`
  plus the matching connectivity override tests; a newer scoped tradeoff
  control that broadens the same clause-`1` `demo_flat_codomain` qualifier
  only across clause-`0` `reference` and the live clause-`4` / clause-`5`
  claim families is now pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_stays_a_tradeoff_control`
  plus the matching connectivity override tests: it lifts the local surface
  to `DCT 103 / 8 / 4523`, narrows the clean partial-prefix wall to `537`,
  cuts zero-admitted capture to `2223`, and shrinks the larger mismatch-`1`
  `reference + demo_flat_codomain` branch from clause-`4` `33 / 28` / `61`
  captured prefixes to clause-`4` `27 / 18` / `45`, but it also widens the
  `small_cluster` to `3324 / 554 / 554 / 0`, so it is only a tradeoff
  control and not the landed path; a third local exploratory
  clause-`3` `anchor-11` exact-argument widening onto the broader
  clause-`0` / clause-`1` claim surface while clause `2` stayed `reference`
  was also checked and reverted: it left the clean `553` partial-prefix wall
  and the executable remaining-two nine-pair split unchanged, but it reopened
  `72` summary-stage incumbent captures, so that clause-`3` widening is also
  ruled out rather than the next landed path; a fourth scoped negative
  control that broadened the clause-`5` side pocket onto the claim-safe
  clause-`0` / clause-`1` surface has now also been checked under override
  and pinned by
  `current_claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  plus the matching connectivity override tests: it lifted the local surface
  to `DCT 103 / 8 / 4779`, widened the `small_cluster` to
  `3516 / 586 / 586 / 0`, and kept the isolated `single` pocket plus
  residual incumbent pressure unchanged, but it widened the partial-prefix
  wall to `585` and the zero-admitted exact-prune capture to `2367`, so it
  is also ruled out rather than the landed path; another scoped negative
  control that reopened the clause-`4` `demo_sharp_codomain` plus
  `demo_sharp_bridge` side pockets on the claim-safe clause-`0` /
  clause-`1` surface is now also pinned by
  `current_claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  plus the matching connectivity override tests: it lifted the local surface
  to `DCT 103 / 8 / 4843`, widened the `small_cluster` to
  `3516 / 586 / 586 / 0`, and kept the isolated `single` pocket plus
  residual incumbent pressure unchanged, but it widened the partial-prefix
  wall to `617` and the zero-admitted exact-prune capture to `2463`, so that
  broader clause-`4` reopening is also ruled out rather than the landed path;
  matching narrower scoped negative controls that reopened only the
  clause-`4` `demo_sharp_codomain` side pocket or only the
  clause-`4` `demo_sharp_bridge` side pocket on that same claim-safe
  clause-`0` / clause-`1` surface are now pinned by
  `current_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
  and
  `current_claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`:
  each lifted the local surface only to `DCT 103 / 8 / 4587`, widened the
  `small_cluster` to `3324 / 554 / 554 / 0`, and kept the isolated `single`
  pocket plus residual incumbent pressure unchanged, but still widened the
  partial-prefix wall to `585` and the zero-admitted exact-prune capture to
  `2367`, so even the narrower claim-safe clause-`4` reopenings are also
  ruled out rather than the landed path; another scoped negative
  control that reopened the clause-`5` side pocket only on the exact
  remaining-two mismatch-`0` bridge slice is now also pinned by
  `current_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice_stays_a_negative_control`
  plus the matching injector and connectivity override tests: it lifted the
  local surface to `DCT 103 / 8 / 4691`, widened the `small_cluster` to
  `3420 / 570 / 570 / 0`, and kept the isolated `single` pocket plus
  residual incumbent pressure unchanged, but it still widened the
  partial-prefix wall to `589`, so that narrower mismatch-`0` bridge-slice
  reland is also ruled out rather than the landed path;
  the
  executable repaired late-path and negative-control guardrails are now also
  synced to that same `4331` / `3` / `3132 / 522 / 522 / 0` local surface
  end to end; the current executable
  step-`15` survivor-bucket
  regression now freezes that latest exact-screened split as one
  library-backed temporal-operator `small_cluster` bucket at
  `3132` generated / `522` admitted / `522` exact-screened / `0` pruned
  plus the isolated non-winning `single` pocket at overshoot
  `115657 / 21112` and the residual `3` single-bucket incumbent prunes; a
  newer executable
  `current_claim_step_fifteen_small_cluster_relief_clears_summary_prunes_while_three_single_bucket_prunes_remain`
  regression now pins that the old `246`-candidate summary-stage
  `small_cluster` incumbent wall has collapsed entirely while the isolated
  pocket stays fenced, so the next repair now targets the residual
  `single`-bucket incumbent pressure rather than reopening the old
  small-cluster summary wall first; a newer executable
  `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
  regression now pins that those remaining `3` incumbent-dominance prunes are
  all `proof_close_group` captures in the fenced temporal `single` bucket,
  that they stay same-primary `103 / 8` non-winners at overshoot
  `115657 / 21112` and bit cost `236`, and that they first diverge only at
  clause positions `0`, `2`, and `5`, so the next repair can stay narrower
  than reopening the whole pocket; a newer
  `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_pin_the_exact_claim_family_labels`
  regression now further freezes those three residual families as clause-`0`
  `claim_flat_domain`, clause-`2` `claim_flat_domain` plus the anchor-`11`
  exact-argument pocket, and clause-`5` `claim_flat_codomain`; a newer
  `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_each_still_carry_a_three_generated_one_admitted_surface`
  regression now also pins that each of those same three fenced proof-close
  families is already only a tiny local `3`-generated / `1`-admitted /
  `1`-pruned surface, so the next repair should stay family-local rather than
  relanding bucket-global same-primary relief from those preserved broader
  group-surface counts; a newer
  `current_claim_step_fifteen_residual_single_bucket_incumbent_families_still_hide_two_unsafe_lifted_terminals`
  regression now also pins that each of those same three families still sits
  on the same raw three-terminal shell, with the reference terminal staying a
  same-primary `103 / 8` non-winner at bit cost `236` while the neighboring
  `next_lift` and `eventual_lift` terminals remain connected, locally
  admissible, and stronger-than-canonical `89 / 8` rivals at bit cost `254`;
  a newer
  `current_claim_step_fifteen_residual_single_bucket_incumbent_families_only_keep_reference_terminals_live`
  regression now further pins that those same three families still keep only
  the reference terminal live on the current claim path while both unsafe
  lifts stay structurally connected but outside historical reanchor and live
  connectivity there, so the next repair still has to stay
  reference-terminal-local and keep those unsafe lifted terminals fenced;
  a newer
  `current_claim_step_fifteen_exact_family_same_primary_relief_still_unfences_the_isolated_single_pocket`
  regression now also proves that even granting same-primary relief only to
  those exact clause-`0`, clause-`2` + anchor-`11`, and clause-`5` residual
  families keeps the repaired local surface at `4331` generated prefixes with
  `553` partial-prefix bar failures, collapses incumbent-dominance from `3`
  to `0`, leaves the `small_cluster` unchanged at `3132 / 522 / 522 / 0`,
  and still unfences the isolated `single` pocket from `1` to `4`
  fully scored non-winning terminals, so the next landed repair must stay
  narrower than same-primary relief even at exact-family scope; a newer
  `current_claim_step_fifteen_subset_local_same_primary_relief_only_trades_single_prunes_for_non_winners`
  regression now further proves that every non-empty strict subset of those
  same three families still keeps `4331` / `553` and the cleared
  `3132 / 522 / 522 / 0` `small_cluster` unchanged, keeps the canonical
  winner unchanged, and only trades `n` residual incumbent prunes for `n`
  extra fully scored non-winning terminals in the isolated `single` pocket,
  with those extra candidates all staying same-primary `103 / 8`,
  bit-cost-`236` reference-terminal completions from the selected residual
  families, so the next landed repair must stay narrower than proof-close
  same-primary relief even on a strict subset of those families;
  a newer
  `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_stay_same_primary_and_non_winning`
  regression now also proves that omitted demo-only side variants at clause
  positions `0`, `1`, `4`, and `5` are already structurally connected,
  locally admissible, same-primary `103 / 8` non-winners around the live
  anchor-`11` exact-argument pocket, with the clause-`4`
  `demo_sharp_codomain` and `demo_sharp_bridge` variants now regaining
  historical reanchor only on that exact side pocket, the clause-`0`
  `demo_sharp_domain` and clause-`1` `demo_flat_codomain` variants now also
  regaining historical reanchor only on that same exact side pocket and only
  with the reference terminal, while the clause-`0`,
  clause-`1` `demo_eventually_codomain`, and clause-`5` variants stay outside
  historical reanchor there; that newer clause-`0` exact-pocket reland plus
  the follow-on exact-pocket clause-`5` repairs now leave the live canonical
  guardrail at `4331` / `553` / `3` with the same
  `3132 / 522 / 522 / 0` `small_cluster`; a reverted raw
  position-`0` reland that lifted local step `15` only to `4285` also
  reopened the repaired clause-`0` zero-admitted capture, and the older raw
  clause-`4` / clause-`5` probes only reached `3980` or `3974` while
  reopening the captured clause-`2` / clause-`3` surface; the newer
  `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_still_fence_unsafe_lifted_terminals`
  regression now also pins that every one of those omitted side variants
  stays structurally connected but outside historical reanchor once the
  terminal lifts, so both unsafe lifted terminals remain fenced there; the newer
  clause-`5` `demo_sharp_domain` and `demo_flat_codomain` pockets are now
  also consumed narrowly on top of that clause-`4` repair, and the landed
  small-cluster relief has now consumed that summary-stage move too, so the
  next repair should target the residual `3` `single`-bucket incumbent prunes
  on top of the fenced `4331` local surface rather than broadening raw
  position `0` or relanding another broad clause-`4` / clause-`5`
  injection; a reverted blanket step-`15` same-primary relief
  probe kept generated breadth flat at `4030`, collapsed incumbent-dominance
  to `0`, widened the `small_cluster` aggregate to `2964 / 494 / 494 / 0`,
  and unfenced the isolated `single` pocket to `4` fully scored non-winning
  terminals, and the newer strict-subset same-primary probe now also proves
  that even any non-empty subset of the three residual families only trades
  `n` incumbent prunes for `n` extra fully scored non-winning
  reference-terminal completions while keeping `4331` / `553` and the
  cleared `3132 / 522 / 522 / 0` `small_cluster` unchanged, so that entire
  same-primary-retention family of relands is now outside the landed path;
  user-facing wording stays at
  `bounded live recovery` until stored breadth and certification pass
- the accepted executable late-step canon is the current Rust truth, including
  step `15` / `DCT` at `nu = 103`

Treat these as still incomplete:

- `pen-store` is still partly contract-first
- the anti-junk frontier engine is not yet the full long-range design
- the Agda bridge is still lighter than the final proof-facing target

The current architecture focus is split between three active tracks:

- stronger exact late-step pruning and ordering on
  `realistic_frontier_shadow`
- honest breadth, budget, and evidence surfacing on `demo_breadth_shadow`
- stored breadth repair, rerun evidence, and certification work on
  `desktop_claim_shadow`

## Current-State References

Read only the track-specific detail you need:

- For the current realistic-shadow capability inventory, evidence snapshot, and
  remaining search gap, read
  [references/11-current-realistic-shadow.md](references/11-current-realistic-shadow.md).
- For the current demo-lane stable mechanisms, evidence baselines, and signoff
  status, read
  [references/12-current-demo-lane.md](references/12-current-demo-lane.md).
- For the current claim-lane scaffold state, read
  [references/13-current-claim-lane.md](references/13-current-claim-lane.md).
- For live demo-lane targets and signoff criteria, read
  [../../demo_lane_progress.md](../../demo_lane_progress.md),
  [../../demo_lane_plan.md](../../demo_lane_plan.md), and
  [../../demo_lane_checklist.md](../../demo_lane_checklist.md).
- For the active autonomy workstream, read
  [../../autonomous_plan.md](../../autonomous_plan.md),
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md), and
  [../../autonomous_progress.md](../../autonomous_progress.md), plus
  [../../autonomous_checklist.md](../../autonomous_checklist.md).
  These are the operational docs for remaining claim-lane work and
  intentionally omit old rollout history.

Start with the current architecture doc before diving into donor material:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [../../overall_plan.md](../../overall_plan.md)
- [../../quantum_progress.md](../../quantum_progress.md)

## Working Rules

1. Treat the current Rust workspace and its tests as the source of truth for
   present executable behavior.
2. Use [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md) first when you need
   current crate boundaries, runtime flow, reporting surface, or the honesty
   boundary.
3. Treat `engine/src/*.hs` as donor provenance for unresolved semantics, not as
   automatically newer or truer than the current Rust implementation.
4. Keep the hot path name-free. Semantic labels belong in CLI reporting and
   Agda export layers only.
5. Separate three targets the repo still risks conflating:
   - the current strict executable Rust canon
   - paper-facing or aspirational targets
   - older historical 16-step or alternate late-shell variants
6. Do not mistake compatibility structs, manifest schemas, or placeholder
   storage modules for fully implemented runtime behavior.
7. Do not regress the repo back toward replay-or-template logic when the
   current bounded live atomic lane already works through step `15`.
8. Use the operational repo docs for live open work and use the bundled
   references for stable current-state detail.
9. Assume the local shell is Windows PowerShell unless the current task proves
   otherwise. Prefer one command per shell call and avoid POSIX-style chaining
   such as `&&`, especially for `git add` / `git commit` / `git push`.
10. Prefer narrow, staged edits over oversized multi-file or multi-hunk
    patches. If a large doc or instruction update starts failing to apply,
    split it into smaller targeted changes instead of forcing one huge patch.

## Session Cleanup

Before ending a session:

1. Check for residual processes started during the session, especially
   `cargo`, `pen-cli.exe`, long-running `cargo test`, `cargo run`, benchmark,
   export, and validation commands.
2. Do not assume a timed-out Codex command killed its child processes.
   Inspect the live process table and confirm whether the spawned workload is
   still running.
3. If a residual process was started for the current session and is no longer
   needed, terminate both the child workload and its wrapper process.
4. Do not terminate unrelated user-owned workloads. If ownership is unclear,
   inspect the command line, start time, parent process, and output path before
   deciding.
5. Report any residual processes found or terminated in the final handoff so
   the session closes with an explicit cleanup status.

## First Reads

For most tasks, read in this order:

1. [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
2. [../../overall_plan.md](../../overall_plan.md)
3. One of these track-specific current-state references, depending on the task:
   - [references/11-current-realistic-shadow.md](references/11-current-realistic-shadow.md)
   - [references/12-current-demo-lane.md](references/12-current-demo-lane.md)
   - [references/13-current-claim-lane.md](references/13-current-claim-lane.md)
4. [../../quantum_progress.md](../../quantum_progress.md) when the task
   touches prefix search, bounds, caching, or realistic late-step tuning
5. [../../demo_lane_progress.md](../../demo_lane_progress.md) and
   [../../demo_lane_plan.md](../../demo_lane_plan.md) when the task touches
   `demo_breadth_shadow`
6. [../../autonomous_progress.md](../../autonomous_progress.md) and
   [../../autonomous_next_steps.md](../../autonomous_next_steps.md) when the
   task touches `desktop_claim_shadow`
7. [theory/README.md](theory/README.md) when you need the theorem or manuscript
   map

Then branch based on the task.

## Reference Map

- Read [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md) for the current crate
  graph, runtime flow, reporting architecture, artifact model, and honesty
  boundary.
- Read [../../README.md](../../README.md) for the current user-facing commands
  and smoke-test commands.
- Read [../../overall_plan.md](../../overall_plan.md) for current completion
  status, deliverables, and immediate next priorities.
- Read [../../quantum_progress.md](../../quantum_progress.md) for the current
  delta between the quantum improvement plan and the live Rust codebase.
- Read [references/11-current-realistic-shadow.md](references/11-current-realistic-shadow.md)
  for the current realistic-shadow capability inventory, evidence snapshot, and
  remaining search gap.
- Read [../../demo_lane_progress.md](../../demo_lane_progress.md),
  [../../demo_lane_plan.md](../../demo_lane_plan.md), and
  [../../demo_lane_checklist.md](../../demo_lane_checklist.md) for live
  `demo_breadth_shadow` status, targets, and signoff tasks.
- Read [references/12-current-demo-lane.md](references/12-current-demo-lane.md)
  for the stable current demo-lane mechanisms and evidence baselines that
  should remain true while later demo-lane changes move.
- Read [references/13-current-claim-lane.md](references/13-current-claim-lane.md)
  plus [../../autonomous_plan.md](../../autonomous_plan.md),
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md), and
  [../../autonomous_progress.md](../../autonomous_progress.md), plus
  [../../autonomous_checklist.md](../../autonomous_checklist.md) for the
  current claim-lane mixed state, honesty boundary, and remaining autonomy
  work.
- Read [theory/README.md](theory/README.md) when you need the theorem map or
  manuscript map.
- Read [theory/genesis.md](theory/genesis.md) when you need the exact strict
  `15`-step target.
- Read [theory/pen-model.md](theory/pen-model.md),
  [theory/coherence-and-scaling.md](theory/coherence-and-scaling.md), and
  [theory/novelty-selection-and-rejection.md](theory/novelty-selection-and-rejection.md)
  for the mathematical contract behind the search objective.
- Read [theory/late-framework-abstraction.md](theory/late-framework-abstraction.md)
  and [theory/terminal-dct.md](theory/terminal-dct.md) before making claims
  about steps `10` to `15`.
- Read [references/01-project-brief.md](references/01-project-brief.md) for
  scope, donor priorities, and stale-doc warnings.
- Read [references/02-target-sequence.md](references/02-target-sequence.md) for
  the canonical `15`-step target, current strict values, and bar arithmetic.
- Read [references/03-repo-donor-map.md](references/03-repo-donor-map.md) when
  deciding what to port, what to demote to reporting, and what not to copy.
- Read [references/04-mbtt-kernel.md](references/04-mbtt-kernel.md) before
  touching ASTs, encoding, canonicalization, telescope storage, or capability
  flags.
- Read [references/05-search-and-selection.md](references/05-search-and-selection.md)
  before implementing enumeration, admissibility, ranking, minimality, or
  resume behavior.
- Read [references/06-atomic-research-lessons.md](references/06-atomic-research-lessons.md)
  before designing any wider anti-junk or frontier-retention story.
- Read [references/07-agda-validation.md](references/07-agda-validation.md)
  before building export payloads, witness modules, or verification scripts.
- Read [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)
  when you need test or CI contracts, evidence requirements, or
  non-interference rules.
- Read [references/09-rust-rewrite-blueprint.md](references/09-rust-rewrite-blueprint.md)
  when working on schemas, checkpoints, memory limits, or unfinished runtime
  infrastructure.
- Read [references/10-open-questions.md](references/10-open-questions.md)
  before making decisions that could accidentally lock in the wrong semantics.

## Task Routing

### If you are working on current runtime architecture

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [../../overall_plan.md](../../overall_plan.md)
- [references/09-rust-rewrite-blueprint.md](references/09-rust-rewrite-blueprint.md)

Focus on:

- crate boundaries
- runtime and artifact flow
- what is already real versus still scaffolded
- how not to over-claim unfinished resume/storage/governor features

### If you are defining or changing the Rust core

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [theory/pen-model.md](theory/pen-model.md)
- [theory/coherence-and-scaling.md](theory/coherence-and-scaling.md)
- [references/04-mbtt-kernel.md](references/04-mbtt-kernel.md)
- [references/03-repo-donor-map.md](references/03-repo-donor-map.md)

Focus on:

- frozen atom schema
- telescope and library representation
- exact `kappa` and canonicalization contracts
- stable IDs, hashes, and checkpoint compatibility

### If you are implementing the search loop

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [../../quantum_progress.md](../../quantum_progress.md)
- [references/11-current-realistic-shadow.md](references/11-current-realistic-shadow.md)
- [theory/genesis.md](theory/genesis.md)
- [theory/novelty-selection-and-rejection.md](theory/novelty-selection-and-rejection.md)
- [references/05-search-and-selection.md](references/05-search-and-selection.md)
- [references/06-atomic-research-lessons.md](references/06-atomic-research-lessons.md)
- [references/02-target-sequence.md](references/02-target-sequence.md)

Focus on:

- exact-band search and bar semantics
- admissibility from structural debt, not names
- deterministic dedupe and SCC minimality
- the remaining difference between the current realistic online prefix engine
  and the still-missing stronger sound bounds plus broader non-family
  admissibility reuse
- using `PrefixSignature`, memo counters, timing telemetry, and frontier
  memory evidence to drive the next realistic late-step improvements

### If you are working on `demo_breadth_shadow`

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [references/12-current-demo-lane.md](references/12-current-demo-lane.md)
- [../../demo_lane_progress.md](../../demo_lane_progress.md)
- [../../demo_lane_plan.md](../../demo_lane_plan.md)
- [../../demo_lane_checklist.md](../../demo_lane_checklist.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

Focus on:

- keeping `demo_breadth_shadow` comparison-backed rather than authoritative
- treating breadth as real generated or exactly screened search mass, not
  inflated full-evaluation counts
- preserving the explicit phase machine
  `Scout -> BreadthHarvest -> Materialize -> ProofClose -> Seal`
- persisting honest demo evidence in step summaries, narratives, and event
  streams rather than reconstructing it from debug text
- holding the closed early and late signoff baselines without regressing
  accepted parity, narrative/event coverage, or the honesty boundary

### If you are working on `desktop_claim_shadow`

Read:

- [references/13-current-claim-lane.md](references/13-current-claim-lane.md)
- [../../autonomous_plan.md](../../autonomous_plan.md)
- [../../autonomous_progress.md](../../autonomous_progress.md)
- [../../autonomous_checklist.md](../../autonomous_checklist.md)
- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

Focus on:

- keeping the claim lane separate from demo-only behavior
- recording the mixed current state honestly in policy metadata:
  claim-debt admissibility, claim-generic late expansion, and
  structural-generic bucket scheduling are real; `kappa 7-9` mutators and
  claim-path exactness rechecks are landed; stored compare, benchmark, and
  certification outputs now exist for the clean canonical completed `v12`
  bundle,
  but certification still fails honestly on stored breadth misses and the lane
  must stay at `bounded live recovery`; use `scripts/compare_runs.py`,
  `scripts/benchmark_claim_lane.py`, and `scripts/certify_claim_lane.py` as
  the current evidence surfaces before changing more search code, and prefer
  the certificate first when you need the stored step-`1` / step-`15` miss
  anatomy because it now reports catalog widths, root seeding, exact-screen
  pressure, active widening bands, package flags, and claim-debt
  `path` / `trunc` pressure for failing steps
- treat the latest claim-cache work as operational memory work, not just
  metadata work: claim proof-close already drops evaluated terminal payloads
  after ranking and now also releases processed retained prefix groups once
  certification starts, and claim materialization now also consumes cached
  exact completion summaries from the legality cache after reuse, while the
  newer direct compact claim materialization path avoids rebuilding and
  re-walking the same uncached terminal evaluation vector on the hot step-4
  path; the latest full-profile rerun then showed the retained prefix cache
  flattening after prefix state `24`, and the next landed throughput pass now
  reuses one scratch terminal telescope plus the precomputed prefix bit cost
  across that same remaining-two loop, while the newest claim-only discovery
  pass now skips full evaluation for compact terminal candidates that are
  already below bar or incumbent-dominated and showed a modest early smoke
  gain; do not reopen that step-`4` throughput story first unless a fresh
  stored rerun proves the remaining breadth misses are really runtime fallout
- treating the current blocker as stored breadth on the canonical chain:
  stored `v12` still misses step `1` (`546 / 2144`) and step `15`
  (`4331 / 5000`), while step `10`, step `11`, step `12`, step `13`, and
  step `14` are already stored hits
- keeping the new guarded local step-`11` breadth repair and the narrow
  step-`12` selector repair green:
  the connected claim surface should stay at `kappa 5 = 243`,
  `kappa 6 = 729` (total `972`), exact-screen connectivity rejections there
  should stay at `0`, the guarded step-`11` shell should stay accepted, and
  the repaired guarded step-`12` winner plus the later
  step-`13..15` guardrails should stay fixed
- treating the stored step-`13` repair as the current late-step truth:
  the canonical repaired branch now stays at
  `[5,1,3,3,5,3,2]` / `1350` / `2320` with the guarded accepted hash and the
  canonical `62 / 9 / 12027 -> DCT 103 / 8 / 4331` continuation; the clean
  stored and local step-`15` surfaces now match
- treating clean-tree `v12` as the canonical stored bundle, but treating the
  next operational move as diagnosis / repair on the residual stored
  step-`15` `4331 / 5000` gap rather than another rerun setup pass, while
  keeping that same `4331` surface explicit as the guardrail that must not
  regress
- treating the clean step-`15` partial-prefix wall as newly executable rather
  than notes-only:
  `current_claim_step_fifteen_partial_prefix_wall_stays_on_four_early_temporal_prefix_families`
  now pins all `553` actual bound prunes as `451` remaining-two plus `102`
  remaining-three prefixes, with first mismatch pressure only at clause
  positions `0..3` and dominant remaining-two pressure at clause `0` / clause
  `1`, so the next landed repair should start there before reopening the
  smaller clause-`2` / clause-`3` tail
- treating the dominant remaining-two clause-`0` / clause-`1` wall as
  executable exact pairings rather than a generic early blur:
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_stays_on_nine_clause_zero_one_pairings`
  now freezes mismatch-`0` as the six exact
  `claim_flat_domain` / `claim_eventual_domain` crossed with
  `reference` / `claim_sharp_codomain` / `claim_next_codomain` pairings at
  `42` each, and mismatch-`1` as the
  `reference` + `claim_sharp_codomain` / `claim_next_codomain` /
  `demo_flat_codomain` pairings at `42`, `42`, and `61`
- treating that same dominant remaining-two wall as executable one layer
  deeper again too:
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_sits_on_claim_next_bridge_and_reference_clause_four_families`
  now freezes mismatch-`0` and mismatch-`1` onto clause-`4`
  `claim_next_bridge` plus clause-`4` `reference`, with clause-`5`
  staying split only across `reference`, `claim_next_codomain`, and
  `claim_flat_codomain`; the old demo-only clause-`4` bridge pockets now
  survive only on the smaller mismatch-`2` tail, so the next landed repair
  should target the live clause-`4` claim families rather than another
  demo-bridge reopening
- treating that same dominant remaining-two wall as executable per dominant
  pairing too:
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_four_pressure_on_claim_next_bridge_per_clause_zero_one_pairing`
  now freezes the regular mismatch-`0` / mismatch-`1` pairings at
  clause-`4` `claim_next_bridge = 24` versus `reference = 18`, while the
  larger `reference + demo_flat_codomain` side still stays on the same live
  clause-`4` claim families at `33 / 28`; the mismatch-`2` tail is still the
  only place the old clause-`4` demo pockets appear at all
- treating that same dominant remaining-two wall as executable across clause
  `2` too:
  `current_claim_step_fifteen_remaining_two_partial_prefix_wall_keeps_clause_two_pressure_on_claim_variants_under_the_live_clause_zero_one_pairs`
  now freezes the regular mismatch-`0` / mismatch-`1` pairings at
  `claim_flat_domain = 15`, `claim_sharp_codomain = 15`,
  `reference = 12`, with the larger mismatch-`1`
  `reference + demo_flat_codomain` side at `23 / 23 / 15`, so the next move
  should still target the live clause-`0` / clause-`1` plus clause-`4`
  claim-family split rather than reopening clause-`3` widening or another
  hidden demo-only clause-`2` branch
- treating the newer clause-`0` `reference` plus clause-`1`
  `demo_flat_codomain` live-claim-bridge reopening as a tradeoff control
  only:
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_stays_a_tradeoff_control`
  now shows that reopening only that larger mismatch-`1` branch cuts it from
  clause-`4` `33 / 28` / `61` captured prefixes to clause-`4`
  `27 / 18` / `45`, lifts local breadth to `4523`, and narrows the clean wall
  to `537`, but it also widens the `small_cluster` to
  `3324 / 554 / 554 / 0`, so the next landed repair should isolate those
  escaping `16` captures rather than reland the whole reopening
- treating the two reverted clause-`1` side-pocket broadenings as negative
  controls only:
  the clause-`1` `demo_eventually_codomain` exact-pocket reland and the
  broader clause-`0` `claim_flat_domain` plus clause-`1`
  `demo_flat_codomain` exact-pocket reland both lift local generated breadth
  to `4466` but also widen the clean partial-prefix wall to `626`, so neither
  is the next move
- treating the reverted clause-`3` `anchor-11` exact-argument widening onto
  the broader clause-`0` / clause-`1` claim surface as another negative
  control only:
  it leaves the clean `553` wall and the executable remaining-two nine-pair
  split unchanged while reopening `72` summary-stage incumbent captures, so
  it is also not the next move
- treating the reverted clause-`5` side-pocket broadening onto the claim-safe
  clause-`0` / clause-`1` surface as another negative control only:
  it lifts local generated breadth to `4779` and widens the `small_cluster`
  to `3516 / 586 / 586 / 0`, but it also widens the clean partial-prefix
  wall to `585`, so it is also not the next move
- treating the reverted claim-safe clause-`4` reopenings as negative controls
  only:
  reopening either the clause-`4` `demo_sharp_codomain` side pocket or the
  clause-`4` `demo_sharp_bridge` side pocket on the claim-safe clause-`0` /
  clause-`1` surface lifts local generated breadth only to `4587` but still
  widens the clean partial-prefix wall to `585`, and reopening both together
  lifts local breadth to `4843` but widens the wall further to `617`, so none
  of those clause-`4` claim-safe reopenings is the next move
- treating the next honest slice as narrower than any claim-safe clause-`4`
  or clause-`5` reopening or the full reference-clause-`0` tradeoff control:
  the dominant remaining-two wall still lives on the clause-`0` /
  clause-`1` claim surface and the clause-`4` `claim_next_bridge` plus
  `reference` families, but the new claim-safe clause-`4` controls show that
  reopening those side pockets directly is still too broad, and the newer
  `reference + demo_flat_codomain` tradeoff control shows that the next
  landed repair should isolate its escaping `16` captures without taking the
  accompanying `small_cluster` reopening
- treating the remaining step-`15` `small_cluster` incumbent surface as a
  landed local relief rather than the live blocker:
  the old summary-stage same-primary `103 / 8` bit-cost spread is now closed,
  and the remaining local incumbent pressure is down to `3`
  fenced `single`-bucket prunes
- moving admissibility, mutation, scheduling, and certification toward
  family-agnostic structural evidence
- not using stronger words like `unguided` before the certification gate lands
- treating the two newer local step-`13` widened probes,
  `[3,5,3,3,5,1,1]` and `[5,1,3,3,5,3,3]`, as negative controls only:
  they still show how local breadth can be reopened unsafely, but they are
  not the landed repair; both the
  position-`1` / position-`4` reland and the
  position-`0` / position-`4` / position-`5` / position-`6` reland are now
  frozen as executable regressions on the repaired step-`12` chain
- treating the nearby clause-`3` anchor-`11` neighborhood as narrowly landed:
  only the exact-argument pocket is now live, it is isolated to the current
  claim clause-`2` variants, it stays non-winning by losing to the canonical
  step-`15` winner on bit cost `236` versus `229`, clause `6` still fences
  the unsafe `89 / 8` rival, and the lifted anchor-`11` variants must stay
  out

### If you are working on reporting or evidence

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [../../overall_plan.md](../../overall_plan.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

Focus on:

- keeping semantic names out of the hot path
- surfacing accepted candidates, near misses, and retained valid candidates
- making inspect/debug output explain current behavior honestly
- not pretending frontier evidence already exists when it does not

### If you are implementing resume or storage

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [references/09-rust-rewrite-blueprint.md](references/09-rust-rewrite-blueprint.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

Focus on:

- step checkpoints as the stable unit
- frontier checkpoints as disposable speed artifacts
- compatibility hashes and migration behavior
- the gap between current manifest contracts and real shard/blob/queue runtime

### If you are implementing Agda verification

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [theory/late-framework-abstraction.md](theory/late-framework-abstraction.md)
- [theory/terminal-dct.md](theory/terminal-dct.md)
- [references/07-agda-validation.md](references/07-agda-validation.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

Focus on:

- deterministic export payloads
- witness and import consistency
- keeping Agda completely outside the hot loop
- strengthening the bridge without changing acceptance truth

### If you are checking whether a design choice is honest

Read:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [references/01-project-brief.md](references/01-project-brief.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)
- [references/10-open-questions.md](references/10-open-questions.md)

Reject designs that:

- smuggle semantic names or target IDs into core crates
- let reporting or decoding influence selection
- hide replay or template shortcuts behind supposedly atomic enumeration
- treat placeholder runtime modules as completed storage or resume behavior
- rely on floating-point comparisons for core admissibility or ranking

## Quick Summary

- The current Rust workspace is already the primary executable truth for the
  strict `15`-step lane.
- The main realistic-search gap is stronger sound bound pruning plus broader
  non-family admissibility reuse on top of the already-landed memo/bound path.
- The tracked demo-lane signoff set is currently closed; use the repo-level
  demo-lane docs and `references/12-current-demo-lane.md` as the regression
  baseline for future work.
- The claim lane now exists as a separate scaffold, but its policy metadata
  now honestly reports that claim-debt admissibility, a claim-generic late
  surface, and structural-generic bucket scheduling are landed; later `kappa
  7-9` mutators and claim-path exactness rechecks are now also landed; the
  repo also now has a claim-lane compare audit, a failing-until-earned
  certification script, and richer manifest provenance/build fingerprints,
  while failed-run evidence preservation is now landed, claim proof-close now
  both drops evaluated terminal payloads and releases processed retained prefix
  groups more aggressively, claim materialization now also compacts duplicated
  legality-cache terminal payloads plus streams uncached terminal
  materialization directly, and claim frontier items now reuse both the shared
  clause catalog and the shared serialized prefix order key; the latest smoke
  and release reruns removed the old step-4 startup RSS cliff, then sped up
  the hot release step-4 path by about `12-14%` and another about `18-20%`,
  and the newest step-4 throughput pass now reuses one scratch terminal
  telescope plus the precomputed prefix bit cost after the stored full-profile
  rerun showed a retained-prefix plateau inside step `4`; the current
  canonical stored bundle is clean-tree completed `v12`, breadth still fails
  honestly at step `1` and step `15`, but stored step `15` has now lifted to
  `4331 / 5000`, stored step `13` is re-earned, and the clean stored/local
  guardrail surface now holds `4331` after the fenced clause-`4` /
  clause-`5` side-pocket relands plus the landed small-cluster relief, the
  narrower clause-`5` `demo_flat_codomain` extension, the newer
  clause-`4` `demo_sharp_bridge` reopening, the newer clause-`5`
  bridge-pocket stack extension, and the newer clause-`1`
  `demo_flat_codomain` side-pocket reland; the newer clause-`0`
  `demo_next_domain` exact-pocket reland plus the clause-`5`
  exact-pocket follow-on now carry that surface to `4331` / `553` / `3`; the old
  `246`-candidate `small_cluster` wall is collapsed to `3` residual fenced
  `single`-bucket incumbent prunes on top of a
  `3132 / 522 / 522 / 0` `small_cluster` surface, and the guarded local
  step-`11` breadth
  repair plus the narrow step-`12` selector repair are now both re-earned on
  stored evidence.
- The next operational claim-lane work should focus on clean canonical-bundle
  diagnosis at stored step `15`, while keeping step `1` explicit, stored
  step `13` frozen as a hit, and the matched `4331` clean late surface fenced,
  rather than on reopening another local step-`11` theory slice,
  another runtime-only step-`4` micro-optimization, another rerun first, or
  new step-`13` band/reanchor widening.
- Start with [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md), then load only
  the track-specific references you actually need.
