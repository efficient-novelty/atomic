# `desktop_claim_shadow`: pseudocode + current status

This is a compact reading of the current autonomous claim-lane attempt, grounded in:

- `skills/pen-atomic/theory/genesis.md`
- `configs/desktop_claim_shadow_1h.toml`
- `crates/pen-search/src/engine.rs`
- `crates/pen-type/src/admissibility.rs`
- `crates/pen-search/src/enumerate.rs`
- `crates/pen-search/src/prefix_memo.rs`
- `crates/pen-cli/src/cmd_run.rs`
- `autonomous_progress.md`
- `autonomous_next_steps.md`

The goal is still the 15-step executable genesis canon:

```text
1  U_0                                  tau=1    nu=1    kappa=2
2  1                                    tau=2    nu=1    kappa=1
3  star : 1                             tau=4    nu=2    kappa=1
4  Pi/Sigma                             tau=7    nu=5    kappa=3
5  S^1                                  tau=12   nu=7    kappa=3
6  propositional truncation             tau=20   nu=8    kappa=3
7  S^2                                  tau=33   nu=10   kappa=3
8  H-space / S^3                        tau=54   nu=18   kappa=5
9  Hopf fibration                       tau=88   nu=17   kappa=4
10 modal shell                          tau=143  nu=19   kappa=4
11 connection shell                     tau=232  nu=26   kappa=5
12 curvature shell                      tau=376  nu=34   kappa=6
13 operator bundle / metric reading     tau=609  nu=46   kappa=7
14 Hilbert-functional shell             tau=986  nu=62   kappa=9
15 temporal-cohesive shell / DCT        tau=1596 nu=103  kappa=8
```

## What the current claim lane actually is

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

Important honesty note:

- this lane is not yet "unguided from axioms" in the strong sense
- it is structurally guided by claim debt, anchor hints, claim-specific late mutators, and claim-specific bucket taxonomy
- it is a mixed-mode shadow lane, not the authoritative canon lane
- the authoritative lane is still `strict_canon_guarded`

Also:

- the config has a shared `[demo]` block for narrative/floor metadata
- but the current `DemoBudgetController` only activates for `DemoBreadthShadow`
- so `desktop_claim_shadow` uses the realistic-shadow style live search, not the demo-budgeted breadth harvest controller

## Top-level pseudocode

```text
fn run_desktop_claim_shadow():
    cfg = load_toml("configs/desktop_claim_shadow_1h.toml")
    assert cfg.mode.strict

    until_step   = 15
    profile      = DesktopClaimShadow
    window_depth = 2

    workers = resolve_workers_auto_for_claim(cfg, host_cpus, host_ram):
        cpu_based    = min(host_cpus, cfg.search.max_workers)
        if cfg.search.workers != auto or profile != DesktopClaimShadow:
            return cpu_based
        host_claim_budget = host_ram - reserve_for_os_gib
        steady_target     = min(target_rss_gib, host_claim_budget)
        soft_target       = min(soft_rss_gib, host_claim_budget)
        worker_headroom   = soft_target - steady_target
        memory_based      = floor(worker_headroom / worker_arena_bytes)
        return min(cpu_based, clamp(memory_based, 1, max_workers))

    writer = start_run_artifacts():
        create run dir
        write config.toml
        write initial run.json
        append run_started telemetry
        create sqlite metadata db

    steps = search_bootstrap_from_prefix_internal(
        accepted_prefix = [],
        until_step      = 15,
        window_depth    = 2,
        search_profile  = DesktopClaimShadow,
        retention_runtime = frontier_runtime_limits(cfg, workers),
        demo_budget_controller = None,   // claim lane does not use the demo controller
        progress_observer = writer,
    )

    writer.finalize_success():
        write final run.json
        write latest reports
        rewrite telemetry summary
```

## Step loop pseudocode

```text
fn search_bootstrap_from_prefix_internal(prefix, until_step, window_depth, profile, runtime):
    library = []
    history = []
    steps   = []

    admissibility_mode = DesktopClaimShadow

    for step_index in 1..=min(until_step, 15):
        observer.on_step_started(step_index)

        outcome = search_next_step(
            step_index,
            window_depth,
            library,
            history,
            admissibility_mode = DesktopClaimShadow,
            retention_runtime  = runtime,
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
    step_start      = now()
    structural_debt = summarize_structural_debt(library, window_depth)
    admissibility   = strict_admissibility_for_mode(step_index, window_depth, library, mode)
    retention_policy = structural_debt.retention_policy()
    objective_bar    = compute_bar(window_depth, step_index, history).bar
    nu_history       = history.map(|r| (r.step_index, r.nu))

    discovery = discover_realistic_shadow_candidates(
        step_index,
        library,
        history,
        admissibility,
        retention_policy,
        objective_bar,
        nu_history,
    )

    prefix_cache = discovery.prefix_cache
    candidates   = discovery.candidates

    prefix_frontier = build_prefix_frontier_plan(
        prefix_states_explored = discovery.prefix_states_explored,
        step_index,
        objective_bar,
        retention_policy,
        retention_runtime,
        prefix_cache,
    )

    proof_close(prefix_cache, prefix_frontier, candidates, admissibility, objective_bar)

    if candidates.is_empty():
        fail("no atomic candidates")

    deduped = canonical_dedupe(candidates)
    minimal = semantic_minimality_filter(deduped)

    accepted = select_acceptance(objective_bar, minimal):
        keep only bar-clearers
        choose minimal positive overshoot
        break ties deterministically by structural ranking

    frontier_retention = build_frontier_retention(objective_bar, retention_policy, minimal)

    return AtomicSearchStep {
        accepted,
        retained_candidates = minimal,
        telemetry,
        frontier snapshots,
        bucket stats,
        near misses,
    }
```

## Claim-specific admissibility pseudocode

This is the main reason the lane is still not fully unguided.

```text
fn strict_admissibility_for_mode(step_index, window_depth, library, DesktopClaimShadow):
    if step_index in {1,2,3}:
        return frozen bootstrap admissibility

    debt         = summarize_structural_debt(library, window_depth)
    loop_anchor  = historical_loop_anchor_ref(library, window_depth)
    modal_anchor = historical_modal_shell_anchor_ref(library, window_depth)

    return claim_strict_admissibility(debt, loop_anchor, modal_anchor)

fn claim_strict_admissibility(debt, loop_anchor, modal_anchor):
    claim_axes = debt.claim_debt_axes()

    return StrictAdmissibility {
        mode                = DesktopClaimShadow
        min_clause_kappa    = claim_axes.kappa_min
        max_clause_kappa    = claim_axes.kappa_max
        ambient_depth       = 2
        max_expr_nodes      = claim_max_expr_nodes(debt, claim_axes)
        max_path_dimension  = claim_max_path_dimension(debt, claim_axes)
        include_trunc       = (debt.max_path_dimension == 1 and debt.truncated_entries == 0)
        include_modal       = claim_include_modal(debt, claim_axes)
        include_temporal    = (claim_axes.temporal_pressure > 0)
        quota_per_bucket    = debt.quota_per_bucket()

        // unlike guarded / realistic family-guided lanes:
        focus_family        = None
        package_policies    = default
        all package requirements = false

        historical_anchor_ref = match debt.claim_anchor_policy():
            None  -> None
            Loop  -> loop_anchor
            Modal -> modal_anchor
    }
```

Interpretation:

- the lane no longer progresses by named family focus
- but it still uses structural debt summarization to pick the kappa band and syntax allowances
- and it can still import historical anchor hints

## Claim discovery pseudocode

`DesktopClaimShadow` uses `LateFamilySurface::ClaimGeneric` and `SearchBucketTaxonomy::StructuralGeneric`.

```text
fn discover_realistic_shadow_candidates(..., admissibility.mode = DesktopClaimShadow):
    discovery = default()
    ctx = EnumerationContext::from_admissibility(library, admissibility)
    ctx.late_family_surface = ClaimGeneric

    claim_surface_diagnostics = {
        library_size,
        late_family_surface = ClaimGeneric,
        widening_band7_active = (library_size >= 11),
        widening_band8_active = (library_size >= 11),
        widening_band9_active = (library_size >= 13),
    }

    for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa:
        if clause_kappa <= 1:
            exhaustively enumerate telescopes
            check well-formedness
            check admissibility
            evaluate survivors
            continue

        clause_catalog = build_clause_catalog(ctx, clause_kappa)
        emit live checkpoint for claim steps 4-5

        frontier = []

        for first_clause in clause_catalog.clauses_at(0):
            prefix = [first_clause]
            signature = PrefixSignature(step_index, prefix)
            if legality_cache.insert_root(signature, ...):
                work_item = create_online_prefix_work_item(...)
                decision  = screen_prefix_for_frontier(...)
                push to frontier unless exact-screen pruned

        while frontier not empty:
            emit live checkpoint for claim steps 4-5

            work_item = pop_best_prefix(frontier)
            work_item = collapse_single_continuation_chain_if_only_one_child(...)
            prefix_len = len(work_item.prefix_telescope)

            if prefix_len + 1 == clause_kappa:
                handle_remaining_one_prefix(work_item)
                continue

            if work_item.remaining_clause_slots == 2:
                terminal_prefixes = prepare_exact_two_step_terminal_surface(work_item)
                if terminal children still outrank current frontier head:
                    process_prepared_exact_two_step_terminal_surface_now(terminal_prefixes)
                else:
                    for terminal_prefix in terminal_prefixes:
                        decision = screen_prefix_for_frontier(terminal_prefix)
                        push unless exact-screen pruned
                continue

            for next_clause in work_item.next_clauses():
                child_prefix    = append(work_item.prefix_telescope, next_clause)
                child_signature = PrefixSignature(step_index, child_prefix)
                if legality_cache.insert_child(...):
                    child_work_item = create_online_prefix_work_item(...)
                    decision = screen_prefix_for_frontier(child_work_item)
                    push unless exact-screen pruned

    return discovery
```

## Remaining-one hot path pseudocode

This is the current bottleneck.

```text
fn screen_prefix_for_frontier(work_item):
    if work_item.remaining_clause_slots == 1:
        remaining_one_telemetry.remaining_one_prefixes_seen += 1

    if cached terminal-prefix bound exists:
        remaining_one_cached_bound_hits += 1
        return CanClearBar or CannotClearBar from cached bound

    decision = exact_partial_prefix_bound_decision(work_item, bounded_budget)
    if remaining one and decision == CannotClearBar:
        remaining_one_bar_prunes_pre_materialize += 1
    if remaining one and decision == Unknown:
        remaining_one_unknown_bound_budget_exhaustions += 1
    return decision
```

```text
fn exact_partial_prefix_bound_decision(work_item):
    if work_item.remaining_clause_slots == 1:
        return exact_terminal_prefix_bound_decision(...)

    recursively try children:
        collapse deterministic single-child chains
        propagate cached CanClearBar / CannotClearBar decisions upward
        return Unknown if budget runs out
```

```text
fn exact_terminal_prefix_bound_decision(prefix):
    if cached bound exists:
        return from bound

    terminal_clauses = terminal_prefix_clause_candidates(...)

    if budget can pay for full terminal summary:
        summary = compute_terminal_prefix_completion_summary_from_candidates(
            payload = Compact,   // claim mode
        )
        telemetry.terminal_summary_build_millis += elapsed
        legality_cache.store_terminal_prefix_completion_summary(prefix, summary)
        return decision_from(summary.bound)

    else:
        fall back to clause-by-clause exact bound scan with a scratch telescope
        return CanClearBar / CannotClearBar / Unknown
```

## Remaining-one prune/materialize split pseudocode

```text
fn handle_remaining_one_prefix(work_item):
    if claim_try_summary_prune_before_materialization(work_item):
        return

    group = materialize_remaining_one_prefix_group(work_item)

    if group.bound is None:
        return
    if group.bound cannot clear objective_bar:
        terminal_prefix_bar_prunes += 1
        return
    if cached or computed best rank cannot beat incumbent:
        terminal_rank_prunes += pruned_count
        remaining_one_rank_prunes_post_materialize += 1
        return

    prefix_states_explored += 1

    cache_evaluated_terminal_prefix_group_candidates(group):
        sort deterministically
        in claim mode, only evaluate candidates that still might beat incumbent
        run semantic minimality only when candidate is still acceptance-relevant
        update incumbent rank when a better semantically minimal candidate appears
        then drop evaluated payloads again to keep memory bounded

    prefix_cache.record_group_with_bound(group)
```

```text
fn claim_try_summary_prune_before_materialization(prefix):
    if no cached completion summary:
        summary = compute_terminal_prefix_completion_summary_from_candidates(
            payload = Compact
        )
        telemetry.terminal_summary_build_millis += elapsed
        legality_cache.store_terminal_prefix_completion_summary(prefix, summary)

    summary = legality_cache.take_terminal_prefix_completion_summary(prefix)

    if summary.bound is None:
        record discovery counts
        return true   // nothing useful survives

    if summary.bound cannot clear bar:
        terminal_prefix_bar_prunes += 1
        remaining_one_bar_prunes_pre_materialize += 1
        record discovery counts
        return true

    if summary.best_accept_rank / primary_rank already loses to incumbent:
        terminal_rank_prunes += pruned_candidates
        remaining_one_cached_rank_prunes += 1
        remaining_one_rank_prunes_pre_materialize += 1
        record discovery counts
        return true

    legality_cache.store_terminal_prefix_completion_summary(prefix, summary)
    return false
```

```text
fn materialize_terminal_prefix_group(prefix):
    if mode == DesktopClaimShadow:
        if cached summary also carries full evaluations:
            remaining_one_materialized_from_cached_summary += 1
            return materialize_from_summary(...)

        remaining_one_materialized_compact_direct += 1
        return materialize_terminal_prefix_group_compact(...)

    else:
        return full materialization path
```

Interpretation:

- delayed materialization is already landed
- the lane now tries hard to kill remaining-one prefixes before full group materialization
- but when a prefix survives, it still pays heavily for compact summary construction

## Proof-close pseudocode

```text
fn proof_close(prefix_cache, prefix_frontier, candidates, admissibility, objective_bar):
    pending_group_signatures = prefix_frontier.retained_prefix_signatures
    incumbent_terminal_rank  = None

    while pending_group_signatures not empty:
        signature = select next proof-close group deterministically

        group = load_terminal_prefix_group_for_proof_close(signature):
            if mode == DesktopClaimShadow:
                return prefix_cache.take(signature)   // free group as soon as proof-close starts
            else:
                return prefix_cache.get(signature).clone()

        if group.best_accept_rank already loses to incumbent:
            prune whole group
            continue

        sort group candidates deterministically

        for group_candidate in group:
            if candidate rank already loses to incumbent:
                prune candidate
                continue

            evaluated = reuse cached evaluated payload or evaluate_checked_candidate(...)
            witness   = analyze_semantic_minimality(...)
            if witness minimal and rank beats incumbent:
                incumbent_terminal_rank = rank
            candidates.push(evaluated)
```

## Acceptance / persistence pseudocode

```text
fn canonical_dedupe(candidates):
    keep first candidate per canonical_hash

fn semantic_minimality_filter(candidates):
    reject candidate if it contains an admissible bar-clearing SCC subbundle

fn select_acceptance(objective_bar, retained):
    keep bar-clearers
    choose minimal positive overshoot
    tie-break deterministically by the acceptance rank

fn persist_claim_run(step):
    write run.json incrementally
    write reports/steps/step-XX-summary.json
    write checkpoints/steps/step-XX.json
    write frontier manifests + hot/cold shards + dedupe segments + sqlite metadata
    append telemetry.ndjson

    if profile == DesktopClaimShadow:
        annotate observed process RSS on stored step pressure
        for steps 4-5, append:
            reports/steps/step-04-live.ndjson
            reports/steps/step-05-live.ndjson
        each live row includes:
            observed_process_rss_bytes
            frontier_queue_len
            prefix_cache_groups / candidates
            legality_cache entry counts
            exact_screen_prunes
            remaining_one telemetry
```

## Current status

As of `autonomous_progress.md` dated `2026-03-24`:

- `desktop_claim_shadow` is still not signoff-ready
- the current full-profile iteration baseline is `runs/codex-claim-release-full-primary-rank-v1`
- delayed materialization is kept
- the narrow remaining-one incumbent-primary fast path is also kept
- the old allocator cliff is no longer the active blocker
- the real intended profile still stalls in step 4
- the dominant cost is now remaining-one summary-side throughput, especially `terminal_summary_build_millis`

The latest stored intended-profile evidence says:

```text
against previous full baseline runs/codex-claim-release-full-delayed-summary-v1:

prefix_states_explored = 43 at 1964.9s instead of 3309.4s
prefix_states_explored = 52 at 2406.1s instead of 4008.9s
prefix_states_explored = 59 at 2733.2s instead of 4529.4s

after the old wall clocks, the new run was already farther:
by 3331.9s -> 71 instead of 43
by 3950.1s -> 83 instead of about 52
by 4544.8s -> 95 instead of 59
```

Manual-stop snapshot from that newer full run:

```text
still in step 4
frontier queue length                     = 2680
legality summaries                       = 446635
retained prefix cache                    = 40 groups / 147639 candidates
remaining_one_materialized               = 40
remaining_one_cached_rank_prunes         = 441140
remaining_one_rank_prunes_pre_materialize= 441140
remaining_one_rank_prunes_post_materialize= 0
terminal_summary_build_millis            = 4314240
terminal_materialize_millis              = 561
observed RSS                             ~ 343 MiB
```

The honest read is:

- the lane is much faster than the older full baseline
- it is also far more memory-stable than the earlier allocator-failure era
- but it is still spending most of its serious wall-clock time inside compact remaining-one summary construction
- so it still does not reach the later genesis steps on the real desktop profile

## Current challenges relative to the "unguided 15-step genesis" goal

### 1. It is still guided

The lane is currently:

- `claim_debt_guided`, not axiom-only unguided
- `claim_generic` on late expansion, not completely generic anonymous closure
- `structural_generic` in bucket taxonomy, which is broader than family-named scheduling but still a handcrafted search bias
- still allowed to use historical anchor hints

So the current lane is best described as:

```text
anonymous structural search with claim-specific guidance
```

not:

```text
fully unguided rediscovery from raw axioms alone
```

### 2. Full-profile progress is still blocked at step 4

The target sequence needs stable rediscovery through step 15, but the current full claim profile:

- has not honestly cleared step 4 on the intended desktop profile
- therefore has not yet shown end-to-end autonomous rediscovery of steps 5-15 under this lane
- still spends most search time in exact remaining-one screening before proof-close can matter much

### 3. The bottleneck has narrowed but not disappeared

The active hot path is no longer:

- allocator collapse
- compact materialization itself
- or obvious frontier cloning waste

It is now:

- repeated compact terminal summary construction
- repeated remaining-one exact completion reasoning on surfaces that often do not change retained-prefix shape
- slower-than-needed frontier drainage while legality summaries keep growing

### 4. Evidence/certification is still incomplete

Even if the code is better, the lane still lacks the stored evidence needed for the stronger claim:

- no step-15 full-profile stored claim bundle on the intended desktop config
- claim parity / signoff evidence still incomplete
- widened claim band 9 still needs stored breadth/floor evidence on the claim lane itself
- compare, benchmark, and certification remain downstream of one more real step-4 win

### 5. The repo itself says not to call the lane unguided yet

The current claim-lane notes explicitly say:

- keep the safer wording `bounded live recovery`
- treat the lane as a mixed-mode scaffold with honest metadata
- do not claim it is already family-agnostic end-to-end
- do not claim it is already unguided

## Immediate next slice

From `autonomous_next_steps.md`, the current work order is:

```text
1. Land one more narrow remaining-one fast path:
   - cheaper terminal-summary construction
   - or earlier incumbent-dominance pruning
   - or summary-side reuse

2. Re-earn one stored release rerun with until_step = 4
   - same claim profile shape as desktop_claim_shadow_1h
   - read stored reports/steps/step-04-live.ndjson
   - trust artifacts, not terminal impressions

3. Keep the patch only if it really improves one of:
   - lower terminal_summary_build_millis at matching checkpoints
   - same wall clock reaches farther
   - earlier pre-summary or pre-materialization pruning
   - faster frontier drainage without weakening retained-prefix honesty
   - slower legality-summary growth without weakening retained-prefix honesty

4. Only then rerun the real full profile
```

## Short honest summary

```text
Current `desktop_claim_shadow` is a structurally guided, claim-specific,
memory-aware prefix-frontier search that is now much faster and more stable
than the earlier claim lane, but it still stalls in step 4 because compact
remaining-one summary construction dominates wall clock.

It is therefore not yet an unguided end-to-end rediscovery of the 15-step
genesis sequence on this desktop.
```
