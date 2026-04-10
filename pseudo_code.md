# `desktop_claim_shadow`: current pseudocode and blocker map

This is a current-state reading of the live claim lane and the specific problem
it still has today.

Grounded in:

- `autonomous_progress.md`
- `autonomous_next_steps.md`
- `autonomous_plan.md`
- `autonomous_checklist.md`
- `skills/pen-atomic/SKILL.md`
- `skills/pen-atomic/references/13-current-claim-lane.md`
- `configs/desktop_claim_shadow_1h.toml`
- `scripts/certify_claim_lane.py`
- `crates/pen-search/src/engine.rs`
- `crates/pen-search/src/prefix_memo.rs`
- `crates/pen-type/src/admissibility.rs`
- `crates/pen-cli/src/cmd_run.rs`
- `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13/reports/steps/step-15-summary.json`
- `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13/reports/steps/step-15-live.ndjson`

## Current bottom line

The claim lane is no longer blocked by the old step-4 runtime problem. The
current canonical claim bundle reaches step `15`, preserves accepted-hash
parity, and exposes an honest stored failure:

- step `1 = 546 / 2144`
- step `15 = 4331 / 5000`

The active autonomy work is the step-`15` miss, and that miss is already very
specific:

- `553` exact-screen prunes are `partial_prefix_bar_failure`
- `3` are residual `incumbent_dominance`
- `0` are `terminal_prefix_completion_failure`
- `0` are `legality_connectivity_exact_rejection`

So the live wall is a late exact partial-prefix wall on the current canonical
claim-generic temporal surface.

The rerun-backed ordering is already closed on current-head code:

- canonical stored bundle is now
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v13`
- `v11`, `v12`, and `v13` keep the same early breadth miss at
  step `1 = 546 / 2144`
- `v12` and `v13` keep the same late breadth miss at
  step `15 = 4331 / 5000` with the same canonical
  `553` / `3` / `3132` late surface
- step `1` therefore stays explicit but deferred

At local repair scope, the active bottleneck is now narrower than
"some mismatch-zero claim-domain work":

- deeper remaining-one exact-summary relands beneath the representative
  mismatch-`0` pair cell are already exhausted
- the first parent-level historical-reanchor route on each active clause-`5`
  `claim_flat_codomain / reference` family is also exhausted as an unsafe
  matched negative control on `4427 / 545 / 2247` with noncanonical `60 / 8`
- that unsafe route class is now localized too: each route only removes the
  four targeted claim-side remaining-two parent cells and their `24`
  corresponding remaining-one pruned prefixes on the chosen active
  clause-`5` bucket, with no off-target families introduced

So the next code-side repair does not sit on another deeper remaining-one
reland or another route-identity reland. It has to change the parent-level
qualification family above the current remaining-one lattice.

## Current profile

```text
search_profile            = desktop_claim_shadow
strict                    = true
window_depth              = 2
selector                  = minimal_positive_overshoot
exact_clause_policy       = strict
workers                   = auto
max_workers               = 12
frontier_mode             = obligation_guided
learned_motifs            = false

claim metadata recorded in run.json:
  guidance_style          = claim_debt_guided
  late_expansion_policy   = claim_generic
  bucket_policy           = structural_generic
```

Important current honesty notes:

- this is still not the authoritative lane; `strict_canon_guarded` remains the
  authoritative executable truth surface
- this is also still not fully unguided from raw axioms alone
- it is a claim-guided shadow lane with:
  - claim-debt admissibility
  - claim-generic late expansion
  - structural-generic bucket taxonomy
  - optional historical anchor reuse
- user-facing language must still stay at `bounded live recovery` until a
  stored certificate passes

Also important:

- the config still carries a shared `[demo]` block for narrative and floor
  metadata
- `DemoBudgetController::maybe_new(...)` returns `None` unless the search
  profile is `DemoBreadthShadow`
- so `desktop_claim_shadow` does not run the demo breadth-harvest/proof-close
  controller, even though the lane still persists demo-shaped narrative and
  bucket fields for evidence

## Current canonical late surface

The accepted late chain is currently:

```text
step 13 = [5,1,3,3,5,3,2] / 1350 / 2320
step 14 = 62 / 9 / 12027
step 15 = DCT 103 / 8 / 4331
```

The stored step-15 summary currently says:

```text
generated_raw_prefixes      = 4331
canonical_prefix_signatures = 1982
well_formed_terminals       = 261
hard_admissible             = 261
exact_bound_screened        = 554
exact_bound_pruned          = 553
heuristic_dropped           = 257
full_telescopes_evaluated   = 1
retained_candidates         = 1
accepted                    = DCT, nu=103, kappa=8, bit_kappa=229
bar                         = 19520/2639
accepted overshoot          = 115657/21112
```

The stored bucket picture is:

```text
small_cluster = 3132 generated / 522 admitted / 522 exact_screened / 0 pruned
single        = 0 generated / 0 admitted / 0 exact_screened / 3 pruned
fully scored non-winner in single bucket = 1 at overshoot 115657/21112
```

The current step-open diagnostics from `step-15-live.ndjson` are:

```text
raw_catalog_clause_widths   = [3,3,3,3,3,3,3,3]
raw_catalog_telescope_count = 6561
roots_seen                  = 3
roots_enqueued              = 3
kappa band                  = 8..8
late_family_surface         = claim_generic
anchor_policy               = modal
historical_anchor_ref       = 10
active widening bands       = 7,8,9
package_flags               = temporal_shell only
claim_debt_axes             = path0 trunc0 coupling2 support2 modal0 temporal1 reanchor2 closure3
```

## Top-level runtime pseudocode

```text
fn run_desktop_claim_shadow():
    cfg = load_toml("configs/desktop_claim_shadow_1h.toml")
    assert cfg.mode.search_profile == DesktopClaimShadow
    assert cfg.mode.strict

    retention_runtime = frontier_runtime_limits(cfg)
    demo_budget_controller =
        DemoBudgetController::maybe_new(cfg, until_step=15, seed=default)
        // returns None for DesktopClaimShadow

    steps = search_bootstrap_from_prefix_internal(
        accepted_prefix         = [],
        until_step              = 15,
        window_depth            = 2,
        search_profile          = DesktopClaimShadow,
        retention_runtime       = retention_runtime,
        demo_budget_controller  = None,
        progress_observer       = artifact_writer,
    )

    persist run.json, telemetry.ndjson, step summaries, checkpoints,
    frontier manifests, narratives, and step-live ndjson artifacts
```

## Step loop pseudocode

```text
fn search_bootstrap_from_prefix_internal(...):
    library = []
    history = []
    steps   = []

    admissibility_mode = admissibility_mode_for_profile(DesktopClaimShadow)
    // => AdmissibilityMode::DesktopClaimShadow

    for step_index in 1..=15:
        observer.on_step_started(step_index)

        outcome = search_next_step(
            step_index,
            window_depth          = 2,
            library,
            history,
            admissibility_mode    = DesktopClaimShadow,
            retention_runtime,
            demo_step_budget      = None,
            progress_observer,
        )

        observer.on_step_completed(outcome)
        history.push((step_index, outcome.accepted.nu, outcome.accepted.clause_kappa))
        library.push(LibraryEntry::from_telescope(outcome.telescope))
        steps.push(outcome)

    return steps
```

## Step search pseudocode

```text
fn search_next_step(step_index, window_depth, library, history, mode):
    step_start       = now()
    structural_debt  = summarize_structural_debt(library, window_depth)
    admissibility    = strict_admissibility_for_mode(step_index, window_depth, library, mode)
    retention_policy = structural_debt.retention_policy()
    objective_bar    = compute_bar(window_depth, step_index, history).bar
    nu_history       = history.map(|record| (record.step_index, record.nu))

    discovery = discover_realistic_shadow_candidates(
        step_index,
        library,
        history,
        structural_debt,
        admissibility,
        retention_policy,
        objective_bar,
        nu_history,
        demo_step_budget    = None,
        step_start,
        demo_narrative      = None,
        progress_observer,
    )

    prefix_frontier_plan = build_prefix_frontier_plan(
        explored            = discovery.prefix_states_explored,
        step_index,
        admissibility_mode  = DesktopClaimShadow,
        objective_bar,
        retention_policy,
        retention_runtime,
        prefix_cache        = discovery.prefix_cache,
    )

    certify_retained_prefix_groups_after_discovery(...)
    deduped  = canonical_dedupe(discovery.candidates)
    minimal  = semantic_minimality_filter(deduped)
    accepted = select_acceptance(objective_bar, minimal)

    return AtomicSearchStep { ... }
```

## Admissibility pseudocode

The previous version of this doc overstated how uniformly claim-generic the
lane already is. The current code is more specific.

```text
fn strict_admissibility_for_mode(step_index, window_depth, library, DesktopClaimShadow):
    if step_index == 1:
        return frozen bootstrap admissibility for universe
    if step_index == 2:
        return frozen bootstrap admissibility for unit
    if step_index == 3:
        return frozen bootstrap admissibility for witness

    debt         = summarize_structural_debt(library, window_depth)
    loop_anchor  = historical_loop_anchor_ref(library, window_depth)
    modal_anchor = historical_modal_shell_anchor_ref(library, window_depth)

    if step_index in 4..=8:
        if claim_guarded_early_focus_family(step_index, debt, loop_anchor, modal_anchor)
           returns one of:
             FormerEliminator / InitialHit / TruncationHit / HigherHit / SphereLift
        {
            return structural_focus_strict_admissibility(...)
        }

    return claim_strict_admissibility(step_index, DesktopClaimShadow, debt, loop_anchor, modal_anchor)
```

Later claim steps, including the current step-15 blocker, take the
claim-specific generic path:

```text
fn claim_strict_admissibility(step_index, DesktopClaimShadow, debt, loop_anchor, modal_anchor):
    claim_axes = claim_debt_axes_for_step(step_index, debt)

    return StrictAdmissibility {
        mode                     = DesktopClaimShadow
        min_clause_kappa         = claim_axes.kappa_min
        max_clause_kappa         = claim_axes.kappa_max
        ambient_depth            = 2
        max_expr_nodes           = claim_max_expr_nodes(debt, claim_axes)
        max_path_dimension       = claim_max_path_dimension(debt, claim_axes)
        include_trunc            = claim_include_trunc(debt)
        include_modal            = claim_include_modal(debt, claim_axes)
        include_temporal         = claim_include_temporal(claim_axes)
        quota_per_bucket         = debt.quota_per_bucket()
        focus_family             = None
        all package requirements = false
        package_policies         = default
        historical_anchor_ref    = claim_historical_anchor_ref(debt, loop_anchor, modal_anchor)
    }
```

For the current step `15` surface, this concretely resolves to:

```text
kappa_min = 8
kappa_max = 8
include_temporal = true
focus_family = None
historical_anchor_ref = 10
late_family_surface = ClaimGeneric
```

## Discovery pseudocode

`DesktopClaimShadow` runs the realistic prefix-frontier engine with claim
surface diagnostics and claim-specific live checkpoints.

```text
fn discover_realistic_shadow_candidates(..., admissibility.mode = DesktopClaimShadow):
    discovery = default()
    ctx = discovery_enumeration_context(library, admissibility, demo_budget_enabled=false)
    ctx.late_family_surface = ClaimGeneric

    claim_surface = EnumerationSurfaceDiagnostics {
        library_size,
        late_family_surface         = ClaimGeneric,
        claim_widening_band7_active = ...,
        claim_widening_band8_active = ...,
        claim_widening_band9_active = ...,
    }

    claim_step_open = claim_step_open_diagnostics(
        step_index,
        structural_debt,
        admissibility,
        ctx,
    )

    for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa:
        if clause_kappa <= 1:
            use early exhaustive path
            continue

        clause_catalog = build_clause_catalog(ctx, clause_kappa)
        discovery.claim_root_seeding.roots_seen = clause_catalog.clauses_at(0).len()

        emit StepLiveCheckpoint(note="claim_regular_clause_catalog"):
            raw_catalog_clause_widths
            raw_catalog_telescope_count
            claim_surface
            claim_step_open
            claim_root_seeding
            remaining_one_telemetry

        frontier = []
        for root_clause in clause_catalog.clauses_at(0):
            raw_generated_surface += 1
            if !prefix_legality_cache.insert_root(..., late_family_surface = ClaimGeneric):
                roots_rejected_by_insert_root += 1
                continue

            prefixes_created += 1
            work_item = create_online_prefix_work_item(...)
            (decision, performed_exact_check) = screen_prefix_for_frontier(...)

            match decision:
                CanClearBar:
                    roots_enqueued += 1
                    if performed_exact_check: partial_prefix_bound_checks += 1
                    frontier.push(work_item)
                CannotClearBar:
                    roots_rejected_by_exact_screen += 1
                    partial_prefix_bound_prunes += 1
                Unknown:
                    roots_enqueued += 1
                    frontier.push(work_item)

        emit StepLiveCheckpoint(note="claim_root_seeding_summary")

        while frontier not empty:
            emit StepLiveCheckpoint(note="claim_regular_frontier_progress")
            work_item = pop_best_prefix(frontier)
            work_item = collapse_single_continuation_chain(...)

            if prefix_len + 1 == clause_kappa:
                // remaining-one exact screen / materialize path
                ...
                continue

            if work_item.remaining_clause_slots == 2:
                terminal_prefixes = prepare_exact_two_step_terminal_surface(...)
                if can_process_exact_two_step_terminal_surface_now(frontier, terminal_prefixes):
                    process_prepared_exact_two_step_terminal_surface(...)
                else:
                    for terminal_prefix in terminal_prefixes:
                        screen child exact prefix now
                        push only survivors
                continue

            for next_clause in work_item.next_clauses():
                raw_generated_surface += 1
                if !prefix_legality_cache.insert_child(...):
                    continue
                prefixes_created += 1
                child = create_online_prefix_work_item(...)
                (decision, performed_exact_check) = screen_prefix_for_frontier(...)
                keep only CanClearBar or Unknown

    return discovery
```

## Remaining-prefix exact-screen pseudocode

The current blocker lives here.

```text
fn screen_prefix_for_frontier(work_item):
    if cached remaining-one terminal bound exists:
        remaining_one_cached_bound_hits += 1
        return decision_from_cached_bound

    started = now()
    decision = exact_partial_prefix_bound_decision(
        step_index,
        library,
        admissibility,
        objective_bar,
        nu_history,
        clause_catalog,
        work_item,
        prefix_legality_cache,
        budget = EXACT_PARTIAL_PREFIX_BOUND_BUDGET,
        incumbent_rank,
        remaining_one_telemetry,
        remaining_one_summary_kernel_activation,
    )
    remaining_one.exact_partial_prefix_bound_millis += elapsed(started)
    return (decision, performed_exact_check = decision != Unknown)
```

Recursive exact screening:

```text
fn exact_partial_prefix_bound_decision(work_item):
    if cached partial decision exists:
        return cached CanClearBar / CannotClearBar

    if work_item.remaining_clause_slots == 1:
        decision = exact_terminal_prefix_bound_decision(...)
        cache decision if it is cacheable
        return decision

    for child in exact children of this prefix:
        spend budget for child and any collapsed single-continuation chain
        propagated = exact_partial_prefix_bound_decision(child)
        cache propagated decision onto the collapsed signatures too

        if propagated == CanClearBar:
            cache parent as CanClearBar
            return CanClearBar
        if propagated == Unknown:
            return Unknown

    cache parent as CannotClearBar
    return CannotClearBar
```

What this means for the current step-15 wall:

- many remaining-three prefixes recurse into remaining-two prefixes
- many remaining-two prefixes recurse into remaining-one exact terminal checks
- the stored run currently ends with `553` of those prefixes proving
  `CannotClearBar`

What this now means for the active next move:

- the remaining-one lattice below the representative mismatch-`0` claim-side
  shell is already spent
- the first parent-level route class above that lattice is spent too
- the live bottleneck is therefore a parent-level qualification problem, not a
  missing deeper remaining-one child or a missing route-identity split inside
  the same active clause-`5` buckets

## Remaining-one clause preparation pseudocode

The claim lane already landed a meaningful optimization here:

```text
fn terminal_prefix_clause_candidates(...):
    started = now()

    if prefix_legality_cache.filter_claim_open_band_terminal_clauses(...) returns clauses:
        result = ClaimAdmittedOpenBand(
            clauses with:
                cached_admissibility_decision = None
                cached connectivity facts
                cached single-clause nu facts
        )
    else if prefix_legality_cache.filter_terminal_clauses(...) returns clauses:
        result = General(
            clauses with:
                cached_admissibility_decision = Some(decision)
                cached connectivity facts
                cached single-clause nu facts
        )
    else:
        result = General(all filtered_last_clause_options with no cached admissibility)

    remaining_one_telemetry.absorb_terminal_prefix_clause_filter_duration(now() - started)
    return result
```

The open-band claim filter in `prefix_memo.rs` is important:

- it reuses the parent legality summary
- it rejects trivially derivable terminal clauses early
- it enforces exact clause-kappa support early
- it avoids dragging demo-family package policies into the claim path

That is why step `15` can already get from a raw `6561` catalog to only `261`
well-formed admissible terminals.

## Remaining-one exact bound pseudocode

```text
fn exact_terminal_prefix_bound_decision(prefix):
    if cached terminal_prefix_bound_summary exists:
        remaining_one_cached_bound_hits += 1
        return decision_from(summary.bound)

    if claim_remaining_one_algebraic_nu_ceiling_cannot_clear_bar(...):
        remaining_one_algebraic_prunes += 1
        return CannotClearBar

    terminal_clauses = terminal_prefix_clause_candidates(...)

    if exact budget can pay for summary:
        summary = compute_terminal_prefix_completion_summary_from_candidates(
            payload = Compact for DesktopClaimShadow
        )
        remaining_one_telemetry.absorb_terminal_summary_build_duration(...)
        cache.store_terminal_prefix_completion_summary(prefix, summary)
        return decision_from(summary.bound)

    // budget fallback path
    for each terminal clause:
        do exact connectivity/admissibility/nu/bar check directly
        if any one clause can clear the bar:
            return CanClearBar
    return CannotClearBar
```

On the current canonical stored run, almost all of the action is in the summary
path, not in the budget-fallback path.

## Remaining-one summary kernel pseudocode

This is the compact summary builder that runs before most surviving
materialization.

```text
fn compute_terminal_prefix_completion_summary_from_candidates(...):
    summary = TerminalPrefixCompletionSummary::default()
    compact_survivor_sketch = Some(...) if payload == Compact
    scratch_telescope       = terminal_prefix_scratch_telescope(prefix)
    compact_accept_context  = acceptance_rank_context_for_prefix(prefix)
    single_clause_nu_ctx    = SingleClauseStructuralNuContext::from_prefix(...)
    kernel_timing           = RemainingOneSummaryKernelTiming::default()

    match terminal_clauses:
        General(candidates):
            for clause in candidates:
                connectivity = cached terminal_connectivity_with_facts(...)
                summary.generated_candidate_count += 1
                if connectivity == PruneDisconnected:
                    maybe record Disconnected evaluation
                    continue

                if no-miss plateau kernel can score this clause without fallback:
                    if rejected:
                        record admissibility diagnostics
                        continue
                    if admitted:
                        record exact_nu timing
                        update summary.bound
                        update admitted_candidate_count
                        maybe update compact survivor sketch
                        maybe update best primary rank
                        maybe build full AcceptRank
                        continue

                telescope = load_terminal_clause_into_scratch(...)
                if connectivity == NeedsFallback and !passes_connectivity(...):
                    maybe record Disconnected evaluation
                    continue

                admissibility_decision =
                    cached decision or assess_strict_admissibility(...)
                record admissibility diagnostics
                if !is_admitted:
                    maybe record AdmissibilityRejected evaluation
                    continue

                exact_nu = single_clause_structural_nu_total(...)
                bit_kappa_used = terminal_prefix_completion_bit_cost(...)
                competition_allowed =
                    terminal_completion_can_compete_for_acceptance(...)
                primary_rank = terminal_prefix_primary_rank(...)
                maybe update compact sketch
                absorb_terminal_prefix_admitted_clause_summary(...)

        ClaimAdmittedOpenBand(candidates):
            same structure, but admissibility is already known to be
            AdmittedFocusAligned on the open-band claim path

    write kernel timing into remaining_one_telemetry
    return summary
```

What the current stored run tells us about this kernel:

- it is exercised heavily enough to prune `553` prefixes before materialization
- it is not currently losing on connectivity or legality for step `15`
- it is proving too many prefixes below the bar on the canonical surface

## Summary-prune-before-materialization pseudocode

The compact claim path tries to kill prefixes before materialization:

```text
fn claim_try_summary_prune_before_materialization(prefix):
    if algebraic ceiling already proves CannotClearBar:
        cache partial CannotClearBar
        terminal_prefix_bar_prunes += 1
        remaining_one_algebraic_prunes += 1
        remaining_one_bar_prunes_pre_materialize += 1
        return true

    if no cached completion summary:
        terminal_clauses = terminal_prefix_clause_candidates(...)
        summary = compute_terminal_prefix_completion_summary_from_candidates(
            payload = Compact
        )
        cache.store_terminal_prefix_completion_summary(prefix, summary)

    summary = cache.take_terminal_prefix_completion_summary(prefix)
    bucket_key = demo_bucket_key(...)
    record bucket surface counts immediately

    if summary.bound is None:
        record discovery counts
        return true

    if summary.bound cannot clear objective_bar:
        record bucket prune
        record discovery counts
        terminal_prefix_bar_prunes += 1
        remaining_one_bar_prunes_pre_materialize += 1
        return true

    if terminal_prefix_rank_prune_count(summary.best_rank, incumbent_rank, same_primary_relief)
       returns pruned_candidates:
        record bucket prune
        record discovery counts
        terminal_rank_prunes += pruned_candidates
        remaining_one_cached_rank_prunes += 1
        remaining_one_rank_prunes_pre_materialize += 1
        return true

    cache.store_terminal_prefix_completion_summary(prefix, summary)
    return false
```

The current step-15 state after the landed small-cluster repair is:

- summary-stage same-primary incumbent captures are already down to zero on the
  broad `small_cluster`
- the residual `3` incumbent-dominance prunes are confined to the fenced
  single bucket
- the real wall is therefore the `553` bound failures, not the incumbent path

## Materialization pseudocode

If a remaining-one prefix survives summary pruning, the claim lane stays
compact:

```text
fn materialize_terminal_prefix_group(prefix):
    if mode == DesktopClaimShadow:
        if cached summary has full evaluations:
            remaining_one_materialized_from_cached_summary += 1
            return materialize_terminal_prefix_group_from_summary(...)

        if cached summary has compact survivor sketch:
            remaining_one_materialized_from_cached_summary += 1
            return materialize_terminal_prefix_group_from_survivor_sketch(...)

        remaining_one_materialized_compact_direct += 1
        return materialize_terminal_prefix_group_compact(...)

    else:
        return full summary materialization path
```

Compact direct materialization:

```text
fn materialize_terminal_prefix_group_compact(...):
    terminal_clauses = terminal_prefix_clause_candidates(...)
    generated_terminal_candidates = 0
    admitted_terminal_candidates  = 0
    retained_telescopes           = []
    best_accept_rank              = None
    bound                         = None

    for each terminal clause:
        cached connectivity
        generated_terminal_candidates += 1
        if disconnected: continue

        if no-miss plateau kernel can score it:
            update discovery counts, bound, admitted count
            if clause can compete for acceptance:
                build accept rank and retain telescope
            continue

        load clause into scratch telescope
        fallback connectivity if needed
        exact admissibility if needed
        if admitted:
            exact_nu = single_clause_structural_nu_total(...)
            bit_kappa_used = terminal_prefix_completion_bit_cost(...)
            absorb bound
            if clause can compete:
                retain telescope with accept rank

    discovery.raw_generated_surface += generated_terminal_candidates
    return MaterializedTerminalPrefixGroup(...)
```

On the canonical `v13` step-15 run, this path only leads to one fully
evaluated retained telescope.

## Post-materialization certification pseudocode

```text
fn cache_evaluated_terminal_prefix_group_candidates(candidates, discovery):
    sort_terminal_prefix_group_candidates_for_certification(candidates)

    for candidate in candidates:
        if DesktopClaimShadow and candidate already cannot beat incumbent:
            skip full evaluation
            continue

        evaluated = reuse cached evaluated candidate or evaluate_checked_candidate(...)
        rank      = candidate.accept_rank or acceptance_rank(...)
        witness   = analyze_semantic_minimality(...)

        if witness.is_minimal() and rank beats incumbent:
            discovery.terminal_rank_incumbent = rank

        candidate.accept_rank         = rank
        candidate.evaluated_candidate = Some(evaluated)

    if DesktopClaimShadow:
        compact_terminal_prefix_group_candidates(candidates)
        // drop evaluated payloads again to keep memory bounded
```

This is why the current step-15 bottleneck is not a heavy proof-close or full
evaluation problem: there is almost nothing left to evaluate once the exact
screen has finished shrinking the surface.

## Current wall reconstruction pseudocode

The repo now contains test-only analysis helpers that reconstruct the exact wall
from the live runtime rather than from folklore:

```text
fn current_claim_step_fifteen_partial_prefix_wall_summary():
    capture all ExactPartialPrefixBoundDecision::CannotClearBar prefixes
    count by:
        remaining_clause_slots
        matched_clause_count
        first mismatch position
        (remaining_clause_slots, first mismatch position)

fn current_claim_step_fifteen_remaining_two_partial_prefix_clause_zero_one_pair_counts():
    among captured prefixes with remaining_clause_slots == 2,
    label clause 0 and clause 1 against the reference step-15 prefix,
    then count exact pairings

fn current_claim_step_fifteen_remaining_two_partial_prefix_clause_zero_one_clause_four_counts():
    project those same captured prefixes onto clause-4 family labels

fn current_claim_step_fifteen_proof_close_incumbent_prune_summary():
    capture late group-prune families that still lose to the incumbent

fn late_step_zero_admitted_failure_summary(prefix, 15):
    for each captured partial-prefix prune:
        try every last clause directly
        prove that the captured prefixes remain zero-admitted
        count disconnect / trivial-derivability / other rejection reasons
```

Those helpers are what pin the current autonomy diagnosis:

```text
partial-prefix wall size                    = 553
remaining-two captures                     = 451
remaining-three captures                   = 102
first mismatch counts                      = {0:312, 1:177, 2:50, 3:14}
off-branch captures outside frozen ladder  = 390 across 10 pairings
mismatch-zero claim-domain tier            = 252
claim-safe mismatch-one tier               = 84
reference/reference tails                  = 54
residual incumbent families                = 3 tiny single-bucket groups
```

## Current negative and tradeoff controls

The current engine tests and autonomy docs already mark these as exhausted:

```text
exact claim-pair + clause-4 claim_next_bridge side
  => 4445 generated / 539 wall / 2241 zero-admitted captures
  => widens small_cluster to 3252 / 542 / 542 / 0

exact claim-pair clause-4 reference side
  => 4379 generated / 549 wall / 2259 zero-admitted captures
  => widens small_cluster to 3180 / 530 / 530 / 0

exact claim-flat single-sheet and claim-sharp single-sheet splits
  => each 4373 generated / 545 wall / 2259 zero-admitted captures
  => each widens small_cluster to 3180 / 530 / 530 / 0

broad mismatch-zero demo-flat reopening
  => 4985 generated / 667 wall / 2757 zero-admitted captures
  => widens small_cluster to 3564 / 594 / 594 / 0

exact remaining-two mismatch-one clause-5 bridge slice
  => 4511 generated / 571 wall / 2325 zero-admitted captures
  => widens small_cluster to 3276 / 546 / 546 / 0

representative mismatch-zero claim-side parent-route on active clause-5
claim_flat_codomain / reference families
  => each lands unsafe 4427 generated / 545 wall / 2247 zero-admitted captures
  => each displaces canonical acceptance to noncanonical 60/8
  => each contracts only the targeted active clause-5 bucket to 44
  => each removes only four targeted remaining-two parent cells plus 24
     corresponding remaining-one pruned prefixes, with no off-target families
```

So the next slice is not "try those again." The next slice must start below
those already-frozen controls.

## Current success condition pseudocode

```text
fn local_step_fifteen_slice_is_good_enough(candidate_patch_result):
    return (
        candidate_patch_result.generated_raw_prefixes > 4331
        && candidate_patch_result.partial_prefix_wall < 553
        && candidate_patch_result.accepted_step_fifteen == canonical_DCT_103_over_8
        && candidate_patch_result.small_cluster <= (3132, 522, 522, 0)
        && candidate_patch_result.single_bucket_stays_fenced
        && candidate_patch_result.unsafe_89_over_8_lifts_stay_fenced
    )
```

Only after that local success should the workflow return to the stored rerun and
certification surfaces:

```text
compare_runs.py
benchmark_claim_lane.py
certify_claim_lane.py
```

## Short honest summary

```text
Current desktop_claim_shadow is a claim-guided, claim-generic late-step search
that already reaches the canonical DCT step with parity-preserving stored
evidence.

Its active failure is no longer a step-4 runtime wall. It is a step-15 exact
partial-prefix breadth wall on the canonical 4331 generated surface.

The broad incumbent wall is already repaired. The residual single bucket is
already fenced. The representative mismatch-zero remaining-one lattice and the
first parent-route class above it are both now exhausted. The next live work
therefore has to remove some of the 553 partial-prefix bar failures by finding
a different parent-level qualification family, without widening small_cluster,
reopening the wrong tradeoff controls, or unfencing the unsafe 89/8 lifted
shell.
```
