# `desktop_claim_shadow`: current step-15 breadth problem statement

Reviewed inputs:

- `autonomous_next_steps.md`
- `autonomous_progress.md`
- `autonomous_plan.md`
- `autonomous_checklist.md`
- `skills/pen-atomic/SKILL.md`
- `skills/pen-atomic/references/13-current-claim-lane.md`
- `docs/ARCHITECTURE.md`
- `configs/desktop_claim_shadow_1h.toml`
- `scripts/certify_claim_lane.py`
- `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13/claim_certificate.txt`
- `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13/claim-compare.txt`
- `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13/claim_benchmark.txt`
- `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13/reports/steps/step-15-summary.json`
- `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13/reports/steps/step-15-live.ndjson`
- `crates/pen-search/src/engine.rs`
- `crates/pen-search/src/prefix_memo.rs`
- `crates/pen-type/src/admissibility.rs`
- `crates/pen-cli/src/cmd_run.rs`

## Executive summary

The active `desktop_claim_shadow` blocker is no longer the old step-4 runtime
kernel. The current honest blocker is a stored step-15 breadth miss on the
canonical `v13` claim bundle.

The repo already has all of the following on
`runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13`:

- completion through step `15`
- accepted-hash parity with the guarded baseline through step `15`
- a ready compare report
- a refreshed benchmark bundle across `v11`, `v12`, and `v13`
- a narrow, reproducible failing certificate

What still fails is breadth:

- step `1 = 546 / 2144`
- step `15 = 4331 / 5000`

The current autonomy work is explicitly not a step-`1`-first pass. The active
problem is the clean step-`15` partial-prefix wall sitting on top of the
canonical `4331` generated surface. That wall is not a vague "search is too
small" story. It is a very specific exact-screen surface:

- `553` exact-screen prunes are `partial_prefix_bar_failure`
- `3` exact-screen prunes are residual `incumbent_dominance`
- `0` are `terminal_prefix_completion_failure`
- `0` are `legality_connectivity_exact_rejection`

So the present step-`15` miss is not primarily a connectivity miss, not a
terminal-clause legality miss, and not a proof-close budgeting miss. It is a
remaining-prefix exact-bound problem.

At the current local-repair level, that exact-bound problem is no longer
"anything under mismatch-`0`". The representative mismatch-`0`
remaining-one exact-summary lattice is already exhausted, and the first
parent-level historical-reanchor route on each active clause-`5`
`claim_flat_codomain / reference` family is also exhausted as a matched unsafe
negative control. The active bottleneck is now finding a different
parent-level qualification family above that lattice.

## Canonical stored evidence

### Certification and compare state

The canonical stored run is:

- `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13`

The stored compare report says:

- trajectory matches guarded
- accepted hashes match guarded
- step-15 claim boundary is still `DCT` at `nu = 103`
- the claim lane remains honest about its metadata:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`

The stored certificate says:

- parity passes through step `15`
- fallback honesty is clear
- narrative artifacts are complete
- manifest completeness passes
- runtime threshold passes
- the only failing checks are:
  - early breadth at step `1`
  - late generated floor at step `15`

The benchmark bundle shows the same breadth outcome on all recent stored runs:

- `v11`: parity pass, early breadth fail, late breadth fail
- `v12`: parity pass, early breadth fail, late breadth fail
- `v13`: parity pass, early breadth fail, late breadth fail at `4387 ms`

This is why the wording still stays at `bounded live recovery`.

### Current accepted late chain

The current accepted late chain, reflected in the autonomous docs and the
stored run, is:

- step `13 = [5,1,3,3,5,3,2] / 1350 / 2320`
- step `14 = 62 / 9 / 12027`
- step `15 = DCT 103 / 8 / 4331`

The accepted step-15 artifact in `step-15-summary.json` is still the canonical
reference telescope:

- label = `DCT`
- `nu = 103`
- `clause_kappa = 8`
- `bit_kappa = 229`
- `rho = 103/8`
- overshoot over bar `19520/2639` is `115657/21112`

### Current step-15 funnel numbers

From the stored `step-15-summary.json`:

- `generated_raw_prefixes = 4331`
- `canonical_prefix_signatures = 1982`
- `well_formed_terminals = 261`
- `hard_admissible = 261`
- `exact_bound_screened = 554`
- `exact_bound_pruned = 553`
- `heuristic_dropped = 257`
- `full_telescopes_evaluated = 1`
- `bar_clearers = 1`
- `semantically_minimal_clearers = 1`

The exact-screen reason totals for step `15` are:

- `partial_prefix_bar_failure = 553`
- `terminal_prefix_completion_failure = 0`
- `incumbent_dominance = 3`
- `legality_connectivity_exact_rejection = 0`

The step-15 bucket surface is also already sharply localized:

- `small_cluster = 3132 generated / 522 admitted / 522 exact_screened / 0 pruned`
- `single = 0 generated / 0 admitted / 0 exact_screened / 3 pruned / 1 fully scored non-winner`

The single bucket is not a broad search mass problem. It is a tiny fenced
residual pocket. The broad miss lives in the `small_cluster`-dominated partial
prefix wall.

### It is now above the exhausted representative mismatch-`0` lattice

The current autonomy docs now pin two deeper facts that matter for the next
repair:

- the representative mismatch-`0` claim-side clause-`6` `reference` union is
  spent and still stays on the same zero-admitted remaining-one family
- the first parent-level claim-side historical-reanchor route on each active
  clause-`5` `claim_flat_codomain / reference` family is also spent

Those two parent-route probes are not fresh search leads:

- each lands the same unsafe `4427 / 545 / 2247` surface
- each displaces canonical acceptance to noncanonical `60 / 8`
- each contracts `small_cluster` only by reopening the wrong survivor shape
- each reopens the isolated `single` bucket
- each is now localized to only four targeted claim-side remaining-two parent
  cells plus their `24` corresponding remaining-one pruned prefixes on the
  chosen active clause-`5` bucket, with no off-target families introduced

So the current bottleneck is not another deeper reland inside that same
remaining-one lattice, and not another claim-side parent-route identity pass.
It is the missing parent-level qualification family above those already spent
surfaces.

### Current live step-open anatomy

From `step-15-live.ndjson` and `scripts/certify_claim_lane.py`:

- raw clause catalog widths are `[3,3,3,3,3,3,3,3]`
- raw catalog telescope count is `6561`
- claim root seeding is:
  - `roots_seen = 3`
  - `roots_rejected_by_insert_root = 0`
  - `roots_rejected_by_exact_screen = 0`
  - `roots_enqueued = 3`
- claim step-open diagnostics are:
  - `kappa = 8..8`
  - `late_family_surface = claim_generic`
  - `anchor_policy = modal`
  - `historical_anchor_ref = 10`
  - widening bands `7`, `8`, and `9` are all active
  - package flags:
    - `operator_bundle = false`
    - `hilbert_functional = false`
    - `temporal_shell = true`
  - claim debt axes:
    - `path_pressure = 0`
    - `trunc_pressure = 0`
    - `coupling_pressure = 2`
    - `support_pressure = 2`
    - `modal_pressure = 0`
    - `temporal_pressure = 1`
    - `reanchor_pressure = 2`
    - `closure_pressure = 3`

This means the step-15 miss is happening on the intended live claim-open band,
not on a stale or mismatched configuration.

## What the current problem actually is

### It is a late exact-screen breadth failure, not a runtime failure

The current code path reaches step `15` quickly enough for stored evidence:

- step `15` wall clock in the stored summary is only `132 ms`
- stored total runtime passes the certificate threshold
- `DemoBudgetController::maybe_new(...)` is disabled for
  `DesktopClaimShadow`, so the shared `[demo]` config block does not create a
  real demo breadth-harvest/proof-close controller for this lane

That is why the modern problem statement must not describe a step-4 runtime
kernel bottleneck. The certification failure is now a correctness-of-breadth
problem in the live claim search surface, not a wall-clock stall.

### It is a partial-prefix wall before materialization

The step-15 miss is concentrated in exact pruning before the lane ever needs to
fully materialize or evaluate most terminal candidates.

The strongest evidence is the step-15 reason split:

- `553` partial-prefix bar failures
- only `3` incumbent-dominance prunes
- no terminal-prefix completion failures
- no legality/connectivity exact rejections

In other words:

- the dominant miss is "this prefix cannot clear the bar even after exact
  remaining work"
- not "this last clause is illegal"
- not "this last clause disconnects"
- not "we materialized many candidates and later lost them"

The code agrees with that diagnosis:

- `screen_prefix_for_frontier(...)` asks the exact bound machinery before
  frontier retention
- `exact_partial_prefix_bound_decision(...)` recursively screens unfinished
  prefixes
- `exact_terminal_prefix_bound_decision(...)` handles remaining-one prefixes
- `claim_try_summary_prune_before_materialization(...)` kills compact claim
  groups before materialization when the cached summary bound or best rank says
  they cannot matter

The stored miss is therefore fundamentally about the exact-screen frontier
surface.

### It is already localized to a narrow late-family surface

The autonomous docs and the engine tests now freeze the wall anatomy:

- remaining-two prefixes: `451`
- remaining-three prefixes: `102`
- first mismatch positions:
  - clause `0 = 312`
  - clause `1 = 177`
  - clause `2 = 50`
  - clause `3 = 14`

Inside the remaining-two wall:

- the dominant clause-`0` slice is `252`
- the dominant clause-`1` slice is `145`

After separating the exhausted mismatch-`1`
`reference + demo_flat_codomain` tradeoff ladder, the off-branch wall becomes:

- `390` captured prefixes across `10` pairings
- priority tier `1`: mismatch-`0` claim-domain surface = `252`
- priority tier `2`: claim-safe mismatch-`1` surface = `84`
- residual tails: mismatch-`2` plus mismatch-`3` `reference/reference` = `54`

This is why the current work order says the next honest slice should start with
the mismatch-`0` claim-domain surface, not with another replay of the older
reference-side or clause-`5` controls.

## Where the current code puts the wall

### Top-level runtime path

The current claim lane is driven by:

- `search_bootstrap_from_prefix_internal(...)`
- `search_next_step_internal_with_clause_catalog_override(...)`
- `discover_realistic_shadow_candidates(...)`

At step `15`, that runtime:

1. builds the library and accepted history through step `14`
2. derives `StrictAdmissibility` via
   `strict_admissibility_for_mode(..., DesktopClaimShadow)`
3. opens `LateFamilySurface::ClaimGeneric`
4. builds the clause catalog for `clause_kappa = 8`
5. seeds three exact roots
6. drives a best-first prefix frontier
7. exact-screens prefixes before materialization whenever possible
8. materializes only the few surviving remaining-one groups
9. evaluates only the acceptance-relevant candidates

### Admissibility path

The claim lane is not demo-only, but it is also not the authoritative guarded
lane:

- early bootstrap steps `1..3` use frozen bootstrap admissibility
- claim steps `4..8` may still borrow guarded-family focus for the five early
  former/initial/trunc/higher/sphere surfaces via
  `claim_guarded_early_focus_family(...)`
- later claim steps, including step `15`, use `claim_strict_admissibility(...)`
  with:
  - no required package family
  - `focus_family = None`
  - claim-debt-driven clause band and syntax allowances
  - optional historical anchor reuse

For the current step-15 surface, the admissibility layer is therefore already
the broader claim-generic late path, but still honestly claim-guided.

### Prefix-screen path

The relevant exact-screen pipeline is:

1. `screen_prefix_for_frontier(...)`
2. `exact_partial_prefix_bound_decision(...)`
3. for remaining-two prefixes:
   - `prepare_exact_two_step_terminal_surface(...)`
   - then recursive exact screening of the prepared child prefixes
4. for remaining-one prefixes:
   - `cached_terminal_prefix_queue_entry_bound_decision(...)`
   - `claim_remaining_one_algebraic_nu_ceiling_cannot_clear_bar(...)`
   - `terminal_prefix_clause_candidates(...)`
   - `compute_terminal_prefix_completion_summary_from_candidates(...)`

The important part is that step `15` is already using the landed compact claim
machinery:

- open-band claim terminal filtering from `PrefixLegalityCache`
- cached terminal connectivity facts
- cached or derived admissibility reuse
- single-clause `nu` facts
- scratch-telescope clause reuse
- compact survivor sketches when full payloads are not needed
- direct compact materialization for surviving groups

So the present problem is not "the claim lane still needs the basic compact
remaining-one machinery." That machinery is already real and already being
exercised. The problem is that the live exact screen still refuses too many
late prefixes on the current canonical surface.

### Summary and materialization split

The compact claim path now works like this:

- `claim_try_summary_prune_before_materialization(...)`
  - reuses or builds a compact `TerminalPrefixCompletionSummary`
  - records bucket surface counts immediately
  - prunes on:
    - missing bound
    - bound cannot clear the bar
    - best reachable rank already loses to incumbent
- only if that summary survives does
  `materialize_terminal_prefix_group(...)` continue
- compact claim mode then prefers:
  - `materialize_terminal_prefix_group_from_survivor_sketch(...)`, or
  - `materialize_terminal_prefix_group_compact(...)`
- after materialization,
  `cache_evaluated_terminal_prefix_group_candidates(...)` evaluates only the
  candidates that still need discovery-time minimality/rank work and then
  drops evaluated payloads again to keep memory bounded

This matters because the current step-15 wall is not caused by broad late
full-evaluation cost. The run evaluates only one terminal telescope. The miss
happens much earlier.

## What is already repaired, and what still remains fenced

### Summary-stage small-cluster incumbent pressure is already fixed

The current engine tests freeze an important repair that has already landed:

- the temporal `small_cluster` is no longer dying under summary-stage
  same-primary incumbent pruning
- summary-stage incumbent prune capture count is now `0`
- the remaining `3` incumbent-dominance prunes live only in the fenced single
  bucket

That is why the autonomy docs describe the old broad incumbent wall as no
longer the lead blocker.

### The residual single pocket is intentionally fenced

The remaining single-bucket pressure is tiny and explicit:

- `3` residual prunes
- each family is `3 generated / 1 admitted / 1 pruned`
- the three exact family labels are:
  - `clause-0 claim_flat_domain`
  - `clause-2 claim_flat_domain plus anchor-11 exact-argument`
  - `clause-5 claim_flat_codomain`

Each of those tiny families still sits beside two stronger-than-canonical
lifted terminals with `89 / 8` and better overshoot than the canonical
reference terminal, but those lifted terminals remain fenced by connectivity on
the live claim path.

So the current problem statement must preserve both truths at once:

- the single bucket is not the main breadth wall
- broadening same-primary relief there is unsafe because it risks relanding the
  noncanonical `89 / 8` lifts

## Why the obvious reopenings are already ruled out

The autonomous docs and current engine tests now freeze multiple late-step
controls as exhausted or negative.

### Frozen tradeoff controls

These do narrow the wall, but they still widen the wrong noncanonical surface:

- exact claim-pair plus clause-`4` `claim_next_bridge` side:
  - `generated = 4445`
  - `partial-prefix wall = 539`
  - zero-admitted captured prefixes = `2241`
  - `small_cluster = 3252 / 542 / 542 / 0`
- exact claim-pair clause-`4` `reference` side:
  - `generated = 4379`
  - `partial-prefix wall = 549`
  - zero-admitted captured prefixes = `2259`
  - `small_cluster = 3180 / 530 / 530 / 0`
- exact claim-flat single-sheet relocalization on the live clause-`4`
  `claim_next_bridge` half:
  - `generated = 4373`
  - `partial-prefix wall = 545`
  - zero-admitted captured prefixes = `2259`
  - `small_cluster = 3180 / 530 / 530 / 0`
- exact claim-sharp single-sheet relocalization on the same half:
  - same outcome as the exact claim-flat single-sheet probe

The docs therefore need to say plainly that these are now control surfaces, not
the next landed repair.

### Frozen negative controls

These push breadth upward only by reopening more bad surface:

- broad clause-`1` `demo_flat_codomain` reopening across the full mismatch-`0`
  claim-domain surface:
  - `generated = 4985`
  - `partial-prefix wall = 667`
  - zero-admitted captured prefixes = `2757`
  - `small_cluster = 3564 / 594 / 594 / 0`
  - two new mismatch-`0` pairings at `45 / 45`
  - each pairing splits clause `4` as `27 / 18`
  - each pairing spreads clause `2` evenly as `15 / 15 / 15`
- exact remaining-two mismatch-`0` clause-`5` bridge-slice reopening:
  - explicitly frozen as a widening negative control
- exact remaining-two mismatch-`1` clause-`5` bridge-slice reopening:
  - `generated = 4511`
  - `partial-prefix wall = 571`
  - zero-admitted captured prefixes = `2325`
  - `small_cluster = 3276 / 546 / 546 / 0`

### Exhausted clause-`5` tails

The clause-`4` reference remaining-three tail is already classified:

- exact clause-`5` `reference` is neutral on `4331 / 553 / 2271`
- exact clause-`5` `claim_flat_codomain` and `claim_next_codomain` are only
  smaller tradeoff controls on `4355 / 551 / 2265`

That is why `autonomous_next_steps.md` explicitly says clause-`5` tail
reopenings are not the next move.

### Exhausted parent-route class

The representative mismatch-`0` claim-side parent-route class is now frozen as
an unsafe negative control across both active clause-`5` families:

- `claim_flat_codomain` route:
  - `4427 generated / 545 wall / 2247 zero-admitted captures`
  - noncanonical accepted `60 / 8`
  - `incumbent_dominance = 117`
  - `small_cluster = 2931 / 455 / 455 / 115`
  - reopened `single` bucket
- `reference` route:
  - the same `4427 / 545 / 2247` surface
  - the same noncanonical `60 / 8`
  - the same `incumbent_dominance = 117`
  - the same `small_cluster = 2931 / 455 / 455 / 115`
  - the same reopened `single` bucket

And the new localization result matters:

- each route only removes four targeted remaining-two parent cells:
  `claim_flat_domain / claim_sharp_codomain` crossed with clause-`4`
  `claim_next_bridge / reference`
- each route removes exactly `8` zero-admitted captures and exactly `24`
  remaining-one pruned prefixes on its chosen active clause-`5` bucket
- neither route introduces any off-target capture or prune family

That is why the next slice has to change qualification family rather than
retrying route identity.

## Formal problem statement

We need one narrow change in the live step-`15` claim exact-screen path that:

- improves the clean stored step-`15` breadth miss on top of canonical `v13`
- increases the honest generated surface beyond `4331`
- narrows the clean partial-prefix wall below `553`
- does not regress the accepted late path
- does not widen the noncanonical `small_cluster`
- does not unfence the isolated `single` pocket
- does not reland the stronger-than-canonical lifted `89 / 8` terminals

More concretely, the next accepted repair must preserve all of these:

- accepted step `15` winner stays canonical `DCT 103 / 8`
- accepted-hash parity through step `15` stays intact
- `small_cluster` is no worse than `3132 / 522 / 522 / 0`
- the `single` bucket stays exactly one fully scored non-winner plus three
  residual incumbent prunes
- the best current canonical claim-open diagnosis still points at the
  mismatch-`0` claim-domain surface before the smaller claim-safe mismatch-`1`
  tier or the `reference/reference` tails
- the active local repair stays above the exhausted representative
  mismatch-`0` remaining-one lattice and above the matched unsafe
  claim-side parent-route class, so it changes parent-level qualification
  rather than relanding route identity

And the next attempt must not "solve" the problem by any of the already ruled
out moves:

- not by reopening broad mismatch-`0` demo-flat controls
- not by replaying exhausted clause-`4` reference-side or single-sheet
  relocalizations
- not by replaying clause-`5` tails
- not by broad same-primary relief
- not by reopening the unsafe lifted shell
- not by turning back to a rerun-first or step-`1`-first theory pass

## Success criteria for the current slice

The immediate slice succeeds if it produces a local step-`15` repair that:

- lowers the clean partial-prefix wall below `553`
- preserves canonical acceptance at `103 / 8`
- keeps the isolated single pocket fenced
- keeps the stronger-than-canonical lifted shell fenced
- keeps `small_cluster` at or below `3132 / 522 / 522 / 0`
- justifies a new clean stored rerun beyond `v13`

The broader lane does not finish until all of these later gates also close:

- stored step `15 >= 5000`
- stored step `1 = 2144`
- compare, benchmark, and certificate refreshed on the new canonical bundle
- certified moderate `full_telescopes_evaluated`
- one passing `claim_certificate.json`

## Current "keep green" invariants

The autonomy work order already identifies the regression fence that the next
repair must keep green:

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
- `current_claim_step_fifteen_clause_five_remaining_two_mismatch_zero_bridge_slice_stays_a_negative_control`
- `current_claim_step_fifteen_clause_five_remaining_two_mismatch_one_bridge_slice_stays_a_negative_control`
- `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`
- `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
- the matching connectivity override tests for any touched override slice

## Bottom line

The current `desktop_claim_shadow` problem is highly specific:

- the lane already reaches canonical step `15`
- the lane already preserves accepted parity and honest metadata
- the lane already exposes the failing breadth anatomy from stored artifacts
- the live miss is a step-`15` partial-prefix exact-screen wall on the current
  claim-generic temporal surface

So the next repair is not "make the claim lane faster" and not "make the claim
lane broader somehow." It is:

- remove or redirect the wrong exact partial-prefix bar failures on the
  canonical `v13` step-`15` surface,
- starting below the broad frozen mismatch-`1`
  `reference + demo_flat_codomain` ladder,
- above the exhausted representative mismatch-`0` remaining-one lattice and
  above the matched unsafe claim-side parent-route class,
- by changing the parent-level qualification family on the live mismatch-`0`
  claim-domain tier rather than replaying route identity,
- while preserving the current canonical winner and every existing safety
  fence.
