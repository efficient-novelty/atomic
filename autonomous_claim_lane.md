# Autonomous Claim Lane

Compiled on 2026-06-12 from the current repository docs and executable code.

This document describes the `desktop_claim_shadow` autonomous claim lane: what
it is, how it runs, how it is certified, the current pseudocode for the live
path, and the main results currently recorded by the project.

## Source Map

Primary state and planning files:

- [`autonomous_progress.md`](autonomous_progress.md)
- [`autonomous_next_steps.md`](autonomous_next_steps.md)
- [`autonomous_plan.md`](autonomous_plan.md)
- [`autonomous_checklist.md`](autonomous_checklist.md)
- [`skills/pen-atomic/references/13-current-claim-lane.md`](skills/pen-atomic/references/13-current-claim-lane.md)

Executable surfaces:

- [`configs/desktop_claim_shadow_smoke.toml`](configs/desktop_claim_shadow_smoke.toml)
- [`configs/desktop_claim_shadow_1h.toml`](configs/desktop_claim_shadow_1h.toml)
- [`configs/desktop_claim_shadow_10h.toml`](configs/desktop_claim_shadow_10h.toml)
- [`crates/pen-type/src/admissibility.rs`](crates/pen-type/src/admissibility.rs)
- [`crates/pen-search/src/engine.rs`](crates/pen-search/src/engine.rs)
- [`crates/pen-search/src/enumerate.rs`](crates/pen-search/src/enumerate.rs)
- [`crates/pen-search/src/expand.rs`](crates/pen-search/src/expand.rs)
- [`crates/pen-search/src/accept.rs`](crates/pen-search/src/accept.rs)
- [`crates/pen-eval/src/nu.rs`](crates/pen-eval/src/nu.rs)
- [`crates/pen-eval/src/bar.rs`](crates/pen-eval/src/bar.rs)
- [`crates/pen-core/src/expr.rs`](crates/pen-core/src/expr.rs)
- [`crates/pen-core/src/clause.rs`](crates/pen-core/src/clause.rs)
- [`crates/pen-core/src/telescope.rs`](crates/pen-core/src/telescope.rs)
- [`crates/pen-cli/src/cmd_run.rs`](crates/pen-cli/src/cmd_run.rs)
- [`crates/pen-cli/src/claim_evidence.rs`](crates/pen-cli/src/claim_evidence.rs)
- [`crates/pen-cli/src/cmd_compare_claim_lane.rs`](crates/pen-cli/src/cmd_compare_claim_lane.rs)
- [`crates/pen-cli/src/cmd_certify_claim_lane.rs`](crates/pen-cli/src/cmd_certify_claim_lane.rs)
- [`crates/pen-cli/src/cmd_benchmark_claim_lane.rs`](crates/pen-cli/src/cmd_benchmark_claim_lane.rs)
- [`scripts/compare_runs.py`](scripts/compare_runs.py)
- [`scripts/certify_claim_lane.py`](scripts/certify_claim_lane.py)
- [`scripts/benchmark_claim_lane.py`](scripts/benchmark_claim_lane.py)

The numerical stored-run results below come from the live autonomous state
files. The named run directories are recorded there as the canonical local
artifacts; this checkout's `runs/` directory only contains the placeholder file.

## Executive Summary

`desktop_claim_shadow` is the repository's autonomous claim-grade shadow lane.
It is not the authoritative truth lane; `strict_canon_guarded` remains the
authoritative executable corpus. The claim lane is a stricter evidence path for
testing whether the current 15-step accepted sequence can be recovered under a
claim-specific, comparison-backed profile with honest metadata, narrative
artifacts, breadth floors, runtime accounting, and downstream certification.

The lane has a certified stored head:

```text
runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15
```

That stored head preserves accepted-hash parity through step `15`, restores the
stored step-`1` breadth surface, preserves the repaired step-`15` late surface,
and carries ready compare, benchmark, and certificate artifacts.

The remaining active work is no longer certification. It is post-certification
local hardening of the clean step-`15` partial-prefix wall, currently still at
`553`.

## Lane Identity

Runtime profile:

```text
search_profile          = desktop_claim_shadow
strict                  = true
window_depth            = 2
selector                = minimal_positive_overshoot
exact_clause_policy     = strict
frontier_mode           = obligation_guided
learned_motifs          = false
```

Claim-policy metadata written to `run.json`:

```text
guidance_style          = claim_debt_guided
late_expansion_policy   = claim_generic
bucket_policy           = structural_generic
```

The important boundary is semantic:

- `strict_canon_guarded` is still the authoritative accepted sequence.
- `desktop_claim_shadow` is a certified shadow lane.
- The lane is bounded and claim-guided, not an unguided exhaustive theorem
  discovery claim.
- Stronger language must be tied to the stored `v15` certificate and disclosed
  desktop bundle.

## Architecture

### 0. Core Calculus / AST Definitions

The core calculus lives in `pen-core`. It is deliberately anonymous: the search
engine sees structural constructors, de Bruijn variables, library references,
roles, hashes, and telescopes, not semantic theorem names.

The main expression enum is `Expr` in `crates/pen-core/src/expr.rs`:

```text
Expr =
  App(function, argument)
  Lam(body)
  Pi(domain, codomain)
  Sigma(domain, codomain)
  Univ
  Var(index)
  Lib(index)
  Id(type, left, right)
  Refl(expr)
  Susp(expr)
  Trunc(expr)
  PathCon(dimension)
  Flat(expr)
  Sharp(expr)
  Disc(expr)
  Shape(expr)
  Next(expr)
  Eventually(expr)
  Bang(expr)
  WhyNot(expr)
```

How to read that in the current lane:

- Types are represented by the same AST as terms. `Univ`, `Pi`, `Sigma`,
  `Id`, modal constructors, truncation/suspension constructors, and temporal
  constructors are all structural `Expr` nodes.
- Terms are also `Expr` nodes: `Lam`, `App`, `Var`, `Lib`, `Refl`, and the
  constructor applications built from them.
- Variables are de Bruijn-style `Var(u32)` references into the local telescope.
- Earlier accepted structures are addressed through `Lib(u32)`.
- Cubical/path structure is represented by `Id(type, left, right)`,
  `Refl(expr)`, and explicit path constructors `PathCon(dimension)`.
  The dimension payload is the cubical/path dimension used by connectivity,
  admissibility, and novelty scoring.
- Modal and cohesive structure is represented by `Flat`, `Sharp`, `Disc`, and
  `Shape`.
- Temporal structure is represented by `Next` and `Eventually`.
- `Bang` and `WhyNot` are present for grammar-variant experiments; the frozen
  canonical v1 atom set uses the first eighteen constructors through
  `Eventually`.

Each expression also has an internable `ExprNode` form with child IDs and an
optional integer payload for `Var`, `Lib`, and `PathCon`. That is the stable
node-shaped view used by canonicalization and structural accounting.

Clauses are records in `crates/pen-core/src/clause.rs`:

```text
ClauseRec {
    role: ClauseRole,
    expr: Expr,
}

ClauseRole =
  Formation | Introduction | Elimination | PathAttach | Computation
```

A candidate structure is a telescope in
`crates/pen-core/src/telescope.rs`:

```text
Telescope {
    clauses: Vec<ClauseRec>
}
```

The primary `kappa` used by accepted candidates is the clause count:

```text
Telescope::kappa() = clauses.len()
```

The same type also exposes:

- `bit_cost()`: encoded AST bit cost, reported as `bit_kappa`
- `lib_refs()` and `var_refs()`: structural dependency sets
- `path_dimensions()`: every `PathCon(dimension)` in the telescope
- `is_connected()`: a conservative telescope connectivity predicate
- `classify(...)`: structural class such as `Foundation`, `Hit`, `Modal`,
  `Axiomatic`, or `Synthesis`
- `reference(step)`: deterministic reference telescopes used by tests,
  exported artifacts, and comparison surfaces

### 0.1. PEN Evaluator

The evaluator surface is split between `pen-eval` and `pen-search`.

Novelty `nu` is counted in `crates/pen-eval/src/nu.rs`:

```text
structural_nu(telescope, library, nu_history) -> StructuralNuResult {
    profile = TelescopeNuProfile::from_telescope(telescope)

    if profile.trivially_derivable:
        return nu_g = 0, nu_h = 0, nu_c = 0, total = 0

    nu_g = profile.base_nu_g()
    base_nu_h = profile.base_nu_h()
    base_nu_c = profile.base_nu_c(library, nu_history)

    if profile.class == Synthesis:
        distributive_law_bonus = profile.distributive_law_bonus(...)
        universe_polymorphism_bonus = profile.universe_polymorphism_bonus(...)
        infinitesimal_shift_bonus = profile.infinitesimal_shift_bonus(...)

    nu_h = base_nu_h + infinitesimal_shift_bonus
    nu_c = base_nu_c + distributive_law_bonus + universe_polymorphism_bonus
    total = nu_g + nu_h + nu_c

    return { nu_g, nu_h, nu_c, total, bonuses }
}
```

`compute_native_nu(...)` wraps `structural_nu(...)` and adds trace lines for
reports. The three components are:

- `nu_g`: grammar/geometric structural contribution
- `nu_h`: historical/interface contribution
- `nu_c`: coherence/composition contribution

Candidate evaluation happens in `crates/pen-search/src/expand.rs`:

```text
evaluate_candidate(library, history, telescope):
    check_telescope(library, telescope)
    return evaluate_checked_candidate(library, history, telescope)

evaluate_checked_candidate(library, history, telescope):
    native = compute_native_nu(telescope, library, history_nu_pairs)
    return build_candidate(telescope, native, library)

build_candidate(telescope, native, library):
    clause_kappa = telescope.kappa()
    bit_kappa = telescope_bit_cost(telescope)
    nu = native.total
    rho = compute_rho(nu, clause_kappa)
    candidate_hash = blake3(serialized_telescope)
    canonical_hash = blake3(canonical_key_telescope(telescope))
    signals = structural_signals(telescope)
    return ExpandedCandidate(...)
```

So the accepted `kappa` in reports is the clause count, while `bit_kappa` is a
separate encoded-size diagnostic. The current accepted step-`15` result is
therefore `nu = 103`, `clause_kappa = 8`, and `rho = 103 / 8`.

Bar and density arithmetic live in `crates/pen-eval/src/bar.rs`:

```text
compute_rho(nu, kappa) = Rational(nu, kappa)
```

```text
compute_bar(window_depth, step_index, history):
    if step_index <= 2:
        phi = 1
        omega = 1 / 2
        bar = 1 / 2
    else:
        phi = d_bonacci_delta(d, n) / d_bonacci_delta(d, n - 1)
        omega = sum(history.nu) / sum(history.kappa)
        bar = phi * omega
```

All of these are exact rational computations. There is no floating-point
ranking in the acceptance path.

Acceptance ranking is in `crates/pen-search/src/accept.rs`:

```text
select_acceptance(bar, candidates):
    eligible = candidates where candidate.rho >= bar
    rank each eligible by:
        minimal positive overshoot
        clause_kappa
        structural signal tie-breakers
        max_var_ref
        bit_kappa
        descending nu
        canonical key
    choose the minimum rank deterministically
```

### 0.2. Main Generation & Selection Loop

The `S_0 -> S_15` loop is driven by
`search_bootstrap_from_prefix_internal(...)` in
`crates/pen-search/src/engine.rs`.

Conceptually:

```text
S_0:
    library = []
    history = []
    steps = []

for n in 1..=15:
    admissibility_mode = mode_for_profile(search_profile)
    outcome = search_next_step_with_grammar_profile(
        step_index = n,
        window_depth = 2,
        library = S_{n-1},
        history = accepted novelty/kappa history,
        admissibility_mode,
        grammar_profile,
        retention_runtime,
        optional demo/claim budget,
        progress_observer,
    )

    history.push((n, outcome.accepted.nu, outcome.accepted.clause_kappa))
    library.push(LibraryEntry::from_telescope(outcome.telescope, library))
    steps.push(outcome)

return steps
```

The per-step search function then performs the actual generate/evaluate/select
cycle:

```text
search_next_step_internal(...):
    structural_debt = summarize_structural_debt(library, window_depth)
    admissibility = runtime_admissibility_for_profile(...)
    objective_bar = compute_bar(window_depth, step_index, history).bar
    retention_policy = structural_debt.retention_policy()

    if mode is DesktopClaimShadow / RealisticShadow / DemoBreadthShadow:
        discovery = discover_realistic_shadow_candidates_with_clause_catalog_override(...)
    else:
        enumerate strict telescopes by admissible kappa band
        check_telescope(...)
        assess_strict_admissibility(...)
        evaluate_checked_candidate(...)

    candidates = discovery.candidates
    dedupe by canonical structural key
    apply semantic minimality / SCC checks
    retained = build_frontier_retention(...)
    accepted = select_acceptance(objective_bar, candidates)
    write search stats, frontier stats, claim diagnostics, and accepted result
```

For `desktop_claim_shadow`, this loop uses the claim-specific admissibility
mode and claim-generic late expansion described below. The accepted output of
each step becomes the next library state, so `S_n` is literally the library
after appending the selected telescope from step `n`.

### 1. Configuration Layer

The claim profile lives in three TOML configs:

- `desktop_claim_shadow_smoke`: step-`6` smoke profile, one worker, short
  budgets.
- `desktop_claim_shadow_1h`: full step-`15` desktop profile with a one-hour
  total claim/demo evidence budget.
- `desktop_claim_shadow_10h`: full step-`15` desktop profile with a ten-hour
  budget envelope.

All three share the same core search identity:

- strict mode
- `search_profile = "desktop_claim_shadow"`
- `until_step = 15` for the full profiles
- `window_depth = 2`
- `minimal_positive_overshoot`
- checkpointing under `runs`
- CPU verification for acceleration
- memory ceilings for desktop operation

The configs still use a shared `[demo]` block for narrative and floor metadata.
For the claim lane, that block is not a claim that the lane is the demo lane.
It supplies current reporting/floor fields until the reporting surface is
renamed. In the current code, `DesktopClaimShadow` borrows demo-style budget
planning only for step `1`; later claim steps run the non-budgeted claim path.

### 2. CLI And Artifact Layer

`pen-cli run` loads the config, resolves worker count, writes `run.json`, and
persists every accepted step through the run writer in
`crates/pen-cli/src/cmd_run.rs`.

The run writer persists:

- `run.json`
- `config.toml`
- `telemetry.ndjson`
- `reports/latest.txt`
- `reports/latest.debug.txt`
- `reports/steps/step-XX-summary.json`
- `reports/steps/step-XX-narrative.txt`
- `reports/steps/step-XX-events.ndjson`
- `reports/steps/step-XX-live.ndjson`
- step checkpoints
- frontier snapshots

For `DesktopClaimShadow`, the writer also annotates observed process RSS in the
frontier pressure surface and writes claim live checkpoints as both telemetry
events and per-step live NDJSON.

### 3. Profile Dispatch

The search profile is mapped in `pen-search` as:

```text
SearchProfile::DesktopClaimShadow -> AdmissibilityMode::DesktopClaimShadow
```

The candidate discovery path is shared with the broader realistic frontier
engine:

```text
RealisticShadow | DemoBreadthShadow | DesktopClaimShadow
    -> discover_realistic_shadow_candidates_with_clause_catalog_override(...)
```

The claim lane differs through:

- `AdmissibilityMode::DesktopClaimShadow`
- claim-debt admissibility
- `LateFamilySurface::ClaimGeneric`
- structural-generic bucket taxonomy
- claim live checkpoints
- claim-specific evidence/certification checks

### 4. Claim Admissibility

`strict_admissibility_for_mode(...)` handles the claim lane in two stages.

Steps `1..=3` are frozen bootstrap cases.

For later steps, the code computes:

- structural debt over the active window
- loop anchor
- modal-shell anchor

Then `DesktopClaimShadow` does this:

- For steps `4..=8`, it may reuse an early guarded structural focus when the
  focus family is one of the early foundational families:
  `former_eliminator`, `initial_hit`, `truncation_hit`, `higher_hit`, or
  `sphere_lift`.
- Otherwise it dispatches to `claim_strict_admissibility(...)`.

The claim-generic admissibility route intentionally keeps the package surface
open:

```text
focus_family             = None
package_policies         = PackagePolicies::default()
all require_*_package    = false
historical_anchor_ref    = claim_historical_anchor_ref(...)
```

It sizes the open band from claim debt axes:

- `kappa_min` and `kappa_max`
- path pressure
- truncation pressure
- coupling pressure
- support pressure
- modal pressure
- temporal pressure
- reanchor pressure
- closure pressure

For the live step-`15` temporal-shell surface, the docs record:

```text
kappa band              = 8..8
anchor_policy           = Modal
historical_anchor_ref   = 10
late_family_surface     = claim_generic
package pressure        = temporal_shell only
claim_debt_axes         = coupling2 support2 temporal1 reanchor2 closure3
include_temporal        = true
focus_family            = None
```

### 5. Enumeration And Discovery

`EnumerationContext::from_admissibility(...)` turns strict admissibility into a
clause-generation context. `LateFamilySurface` marks the late-family surface:

```text
None
RealisticShadow
ClaimGeneric
DemoBreadthShadow
```

For the claim lane, the regular late path uses `ClaimGeneric`.

The discovery engine builds a clause catalog for each supported `kappa` band,
then performs online prefix-frontier expansion:

1. Build the clause catalog and raw catalog widths.
2. Insert root clauses into the prefix legality cache.
3. Screen root prefixes with exact partial-prefix bounds when possible.
4. Maintain a sorted frontier of live prefix work items.
5. Pop the best prefix deterministically.
6. Collapse single-continuation chains.
7. For remaining-one prefixes, materialize and evaluate terminal groups.
8. For remaining-two prefixes, prepare exact two-step terminal surfaces.
9. For deeper prefixes, insert child clauses, screen them, and retain only
   prefixes that can still clear the bar or remain unknown.

The claim lane emits live checkpoints with stable notes, including:

```text
claim_early_exhaustive_catalog
claim_early_exhaustive_progress
claim_regular_clause_catalog
claim_root_seeding_summary
claim_regular_frontier_progress
```

These checkpoints are now part of the certification evidence path. They expose
catalog widths, raw catalog telescope counts, root seeding, prefix-cache size,
legality-cache size, frontier size, process RSS, exact-screen prunes, claim
surface diagnostics, and claim-step-open diagnostics.

### 6. Exact Screening And Acceptance

The live bottleneck is in exact partial-prefix screening:

- `screen_prefix_for_frontier(...)`
- `exact_partial_prefix_bound_decision(...)`
- `exact_terminal_prefix_bound_decision(...)`
- `claim_try_summary_prune_before_materialization(...)`
- `materialize_remaining_one_prefix_group(...)`

The exact screen asks whether any completion of a partial prefix can clear the
current objective bar. If all exact completions fail, the prefix is counted as a
`partial_prefix_bar_failure`. If the answer cannot be established within the
screening path, the prefix remains in the frontier.

Full terminal candidates then pass through:

- type checking
- connectivity
- strict admissibility
- exact `nu`, `rho`, and bar comparison
- canonical deduplication
- semantic minimality
- deterministic acceptance by minimal positive overshoot
- deterministic tie-breaking

The step-`15` accepted candidate remains the canonical `DCT` candidate:

```text
nu = 103
clause_kappa = 8
```

### 7. Memory And Proof-Close Behavior

The claim lane includes memory hardening specifically for late proof-close:

- evaluated terminal payloads are dropped after ranking
- processed retained prefix groups are released once certification starts
- legality-cache completion summaries are reused
- uncached compact materialization is direct rather than rebuild-and-rewalk
- frontier items reuse the shared clause catalog and serialized prefix key
- worker count for auto mode is capped by claim memory headroom, not only CPU
  count

The goal is to preserve deterministic evidence while keeping the local desktop
run inside the configured RSS envelope.

### 8. Evidence Layer

There are two equivalent evidence surfaces:

- Native Rust commands:
  - `pen-cli compare-claim-lane`
  - `pen-cli benchmark-claim-lane`
  - `pen-cli certify-claim-lane`
- Python scripts:
  - `scripts/compare_runs.py`
  - `scripts/benchmark_claim_lane.py`
  - `scripts/certify_claim_lane.py`

On this machine, direct Python script execution is recorded as blocked because
there is no runnable `python`, `python3`, `py`, or `uv` launcher on `PATH`.
That is no longer a lane blocker because the native `pen-cli` commands cover
local evidence refresh.

The certification layer checks:

- accepted-hash parity through step `15`
- claim search-policy honesty
- fallback/replay honesty
- narrative artifact completeness
- early breadth target, currently exact step `1 = 2144`
- late generated floors for steps `10..=15`
- runtime threshold
- exact-screen reason completeness
- prune-class completeness
- manifest provenance completeness

## Pseudocode

### Top-Level Run

```text
fn run_desktop_claim_shadow(config_path, run_id):
    cfg = load_toml(config_path)
    assert cfg.mode.strict == true
    assert cfg.mode.search_profile == DesktopClaimShadow

    host = inspect_host()
    workers = resolve_worker_count_for_host(cfg, host.logical_cpus, host.ram)
    retention_runtime = frontier_runtime_limits(cfg)
    writer = RunArtifactWriter::start(run_id, cfg, workers, host)

    demo_budget_controller =
        if cfg.mode.search_profile supports demo/claim narrative:
            DemoBudgetController::maybe_new(cfg)
        else:
            None

    steps = search_bootstrap_from_prefix_internal(
        accepted_prefix        = [],
        until_step             = cfg.search.until_step,
        window_depth           = cfg.objective.window_depth,
        search_profile         = DesktopClaimShadow,
        grammar_profile        = cfg.mode.grammar_profile,
        retention_runtime      = retention_runtime,
        demo_budget_controller = demo_budget_controller,
        progress_observer      = writer,
    )

    writer.finalize_success()
    return steps
```

### Bootstrap Step Loop

```text
fn search_bootstrap_from_prefix_internal(...):
    library = []
    history = []
    steps = []

    mode = admissibility_mode_for_profile(DesktopClaimShadow)
    assert mode == DesktopClaimShadow

    for step_index in 1..=until_step:
        observer.on_step_started(step_index)

        demo_step_budget =
            if search_profile == DemoBreadthShadow:
                plan_demo_budget(step_index)
            else if search_profile == DesktopClaimShadow and step_index == 1:
                plan_demo_budget(step_index)
            else:
                None

        outcome = search_next_step_with_grammar_profile(
            step_index,
            window_depth,
            library,
            history,
            mode,
            grammar_profile,
            retention_runtime,
            demo_step_budget,
            observer,
        )

        observer.on_step_completed(outcome)
        history.push((step_index, outcome.accepted.nu, outcome.accepted.kappa))
        library.push(LibraryEntry::from_telescope(outcome.telescope))
        steps.push(outcome)

    return steps
```

### Claim Admissibility

```text
fn strict_admissibility_for_mode(step_index, window_depth, library, DesktopClaimShadow):
    if step_index in {1, 2, 3}:
        return frozen_bootstrap_admissibility(step_index)

    debt = summarize_structural_debt(library, window_depth)
    loop_anchor = historical_loop_anchor_ref(library, window_depth)
    modal_anchor = historical_modal_shell_anchor_ref(library, window_depth)

    if step_index in 4..=8:
        focus = claim_guarded_early_focus_family(
            step_index,
            debt,
            loop_anchor,
            modal_anchor,
        )
        if focus is an early foundational family:
            return structural_focus_strict_admissibility(
                DesktopClaimShadow,
                debt,
                focus,
                loop_anchor,
                modal_anchor,
            )

    return claim_strict_admissibility(
        step_index,
        DesktopClaimShadow,
        debt,
        loop_anchor,
        modal_anchor,
    )
```

```text
fn claim_strict_admissibility(step_index, mode, debt, loop_anchor, modal_anchor):
    axes = claim_debt_axes_for_step(step_index, debt)

    return StrictAdmissibility {
        mode = DesktopClaimShadow
        min_clause_kappa = axes.kappa_min
        max_clause_kappa = axes.kappa_max
        ambient_depth = 2
        max_expr_nodes = claim_max_expr_nodes(debt, axes)
        max_path_dimension = claim_max_path_dimension(debt, axes)
        include_trunc = claim_include_trunc(debt)
        include_modal = claim_include_modal(debt, axes)
        include_temporal = claim_include_temporal(axes)
        include_linear_exponential = false
        quota_per_bucket = debt.quota_per_bucket()
        focus_family = None
        package_policies = Allow all
        require_*_package = false for every package
        historical_anchor_ref = claim_historical_anchor_ref(
            debt,
            loop_anchor,
            modal_anchor,
        )
    }
```

### Claim Candidate Discovery

```text
fn discover_claim_candidates(step_index, library, history, admissibility):
    discovery = RealisticShadowDiscovery::default()
    ctx = EnumerationContext::from_admissibility(library, admissibility)
    ctx.late_family_surface = ClaimGeneric

    claim_surface = ctx.surface_diagnostics()
    claim_step_open = claim_step_open_diagnostics(
        step_index,
        structural_debt,
        admissibility,
        ctx,
    )

    for clause_kappa in admissibility.min_clause_kappa..=admissibility.max_clause_kappa:
        if step_uses_early_exhaustive_demo_path(step_index):
            discover_demo_early_exhaustive_candidates(...)
            continue

        catalog = build_clause_catalog(ctx, clause_kappa)
        emit_live_checkpoint("claim_regular_clause_catalog", catalog, claim_surface)

        frontier = []
        for root_clause in catalog.clauses_at(0):
            raw_generated_surface += 1
            root_prefix = Telescope([root_clause])

            if !prefix_legality_cache.insert_root(root_prefix, ClaimGeneric):
                roots_rejected_by_insert_root += 1
                continue

            work_item = create_online_prefix_work_item(root_prefix)
            decision = screen_prefix_for_frontier(work_item)

            if decision == CanClearBar or decision == Unknown:
                roots_enqueued += 1
                frontier.push(work_item)
            else:
                roots_rejected_by_exact_screen += 1
                partial_prefix_bound_prunes += 1

        emit_live_checkpoint("claim_root_seeding_summary", frontier)

        while frontier is not empty:
            emit_live_checkpoint("claim_regular_frontier_progress", frontier)

            work_item = pop_best_prefix(frontier)
            work_item = collapse_single_continuation_chain(work_item)
            if work_item was fully pruned:
                continue

            if work_item.remaining_clause_slots == 1:
                group = materialize_remaining_one_prefix_group(work_item)
                if group.bound cannot clear objective_bar:
                    terminal_prefix_bar_prunes += 1
                    continue
                if incumbent_rank prunes group:
                    terminal_rank_prunes += group.pruned_count
                    continue
                evaluate_and_cache_terminal_group(group)
                continue

            if work_item.remaining_clause_slots == 2:
                terminal_prefixes = prepare_exact_two_step_terminal_surface(work_item)
                if can_process_exact_two_step_terminal_surface_now(frontier, terminal_prefixes):
                    process_prepared_exact_two_step_terminal_surface(terminal_prefixes)
                else:
                    screen each terminal_prefix and push survivors
                continue

            for clause in work_item.next_clauses(catalog):
                child = work_item.extend(clause)
                if !prefix_legality_cache.insert_child(child):
                    continue
                decision = screen_prefix_for_frontier(child)
                if decision == CanClearBar or decision == Unknown:
                    frontier.push(child)
                else:
                    partial_prefix_bound_prunes += 1

    return discovery
```

### Exact Partial-Prefix Screening

```text
fn screen_prefix_for_frontier(work_item):
    if cached exact decision exists:
        return cached decision

    decision = exact_partial_prefix_bound_decision(
        step_index,
        library,
        admissibility,
        objective_bar,
        nu_history,
        clause_catalog,
        work_item,
        prefix_legality_cache,
        budget,
        current_incumbent_rank,
    )

    if decision is cacheable:
        cache decision for work_item.signature

    return decision
```

```text
fn exact_partial_prefix_bound_decision(prefix):
    if prefix.remaining_clause_slots == 1:
        return exact_terminal_prefix_bound_decision(prefix)

    saw_unknown = false
    for child in exact_children(prefix):
        child = collapse_single_continuation_chain(child)
        child_decision = exact_partial_prefix_bound_decision(child)

        if child_decision == CanClearBar:
            cache prefix as CanClearBar
            return CanClearBar

        if child_decision == Unknown:
            saw_unknown = true

    if saw_unknown:
        return Unknown

    cache prefix as CannotClearBar
    return CannotClearBar
```

### Acceptance And Persistence

```text
fn finish_step(discovery):
    candidates = discovery.candidates

    candidates = canonical_dedupe(candidates)
    candidates = semantic_minimality_filter(candidates)
    retained = build_frontier_retention(candidates)

    accepted = candidates
        .filter(candidate.rho > objective_bar)
        .min_by(acceptance_rank(
            overshoot = candidate.rho - objective_bar,
            clause_kappa,
            nu,
            canonical_hash,
        ))

    step = AtomicSearchStep {
        accepted,
        retained,
        search_stats,
        claim_surface,
        claim_step_open,
        claim_root_seeding,
        live_checkpoint_summary,
    }

    persist_step_report(step)
    persist_step_checkpoint(step)
    persist_frontier_snapshot(step)
    append_telemetry(step_accepted)
    update_latest_reports()

    return step
```

### Claim Evidence Refresh

```text
fn compare_claim_lane(guarded_run, claim_run):
    guarded = load_run(guarded_run)
    claim = load_run(claim_run)

    audit = build_claim_lane_audit(guarded, claim)
    compare trajectory, accepted hashes, search-space counts,
            admissibility diagnostics, late-step competition
    compare step-15 claim boundary

    signoff = ready iff:
        trajectory matches
        accepted hashes match
        step-15 boundary is consistent
        audit is ready

    write text/json summary
```

```text
fn certify_claim_lane(guarded_run, claim_run, runtime_threshold):
    guarded = load_run(guarded_run)
    claim = load_run(claim_run)
    audit = build_claim_lane_audit(guarded, claim)

    checks = {
        accepted_hash_parity,
        search_policy,
        fallback_honesty,
        narrative_artifacts,
        early_breadth(step 1 == 2144),
        late_generated_floors(steps 10..15),
        runtime_threshold,
        exact_screen_reason_completeness,
        prune_class_completeness,
        manifest_completeness,
    }

    status = ready iff all checks pass
    write claim_certificate.{txt,json}
```

```text
fn benchmark_claim_lane(guarded_run, claim_runs, runtime_threshold):
    for claim_run in claim_runs:
        load run
        compute parity check
        compute early breadth check
        compute late generated floor check
        compute runtime threshold check
        compute manifest completeness

    aggregate:
        min / median / p90 / max runtime
        step-15 completion count
        parity success count
        early breadth success count
        late floor success count
        runtime threshold pass count

    write claim_benchmark.{txt,json}
```

## Main Results

### Certified Stored Head

Current certified stored head:

```text
runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15
```

Previous audited stored head:

```text
runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v14
```

The current stored head is the first current-head claim bundle recorded in the
autonomous docs that simultaneously:

- preserves accepted-hash parity through step `15`
- restores stored step `1`
- preserves the repaired stored step-`15` surface
- carries ready compare, benchmark, and certification artifacts

### Accepted Canon

The accepted late chain remains:

```text
step 13 = [5,1,3,3,5,3,2] / 1350 / 2320
step 14 = 62 / 9 / 12027
step 15 = DCT / 103 / 8 / 7211
```

Step `15` remains the canonical accepted `DCT` candidate with:

```text
nu           = 103
clause_kappa = 8
```

### Stored Step 1

Stored `v15` restores step `1` to:

```text
generated / well_formed / admitted / legality_connectivity_exact_rejection
= 2144 / 1285 / 1 / 475
```

This closes the earlier stored step-`1` breadth gap for the certified bundle.

### Stored Step 15 Surface

Stored `v15` keeps the repaired step-`15` late surface:

```text
generated_raw_prefixes       = 7211
late generated floor         = 5000
partial_prefix_bar_failure   = 553
incumbent_dominance          = 3
small_cluster                = 2052 / 522 / 522 / 0
broad                        = 3600 / 0 / 0 / 0
```

The isolated `single` pocket remains fenced around the canonical accepted
`reference(15)` completion at `103 / 8`, with residual proof-close incumbent
pressure recorded as `3`.

### Compare Results

The stored `claim-compare.{json,txt}` evidence reports:

```text
Comparison Signoff: ready
accepted-hash parity through step 15
claim lane audit: ready
```

The compare surface checks trajectory, accepted hashes, search-space counts,
admissibility diagnostics, late-step competition, step-`15` claim boundary,
claim-policy honesty, narrative artifacts, and fallback honesty.

### Benchmark Results

The stored `claim_benchmark.{json,txt}` bundle covers `v12`, `v13`, `v14`, and
`v15`:

```text
runtime min      = 3976 ms
runtime median   = 4165 ms
runtime p90      = 4642 ms
runtime max      = 4642 ms
parity success   = 4 / 4
late-floor pass  = 2 / 4
early-breadth pass = 1 / 4
runtime pass     = 4 / 4
```

This is intentionally an aggregate over stored evidence. It does not hide older
samples that missed breadth gates.

### Certification Results

The stored `claim_certificate.{json,txt}` reports:

```text
Claim Certification: ready
stored runtime: 4165 ms
```

Passing checks:

- accepted parity
- search-policy honesty
- fallback honesty
- narrative completeness
- early breadth
- late generated floors
- exact-screen completeness
- prune-class completeness
- manifest completeness

### Current Post-Certification Hardening Target

Certification is closed. The remaining quality target is local step-`15`
hardening:

```text
clean partial-prefix wall = 553
residual incumbent pressure = 3
```

The repaired-head wall localizes to exactly queued frontier remainder branches
`4..=11`; queued branches `0..=3` contribute none.

Branch totals:

```text
4 / 5  = 7
6 / 7  = 19
8 / 9  = 52
10     = 156
11     = 241
```

Largest live blocker:

```text
direct top-level reference remainder = 241 = 199 remaining-two + 42 remaining-three
```

Inside that branch:

```text
mismatch 1 = 177 = 145 + 32
mismatch 2 = 50  = 42 + 8
mismatch 3 = 14  = 12 + 2
```

Inside the dominant mismatch-`1` remaining-two surface:

```text
largest pairing = reference / demo_flat_codomain = 61
clause-4 claim_next_bridge side = 33
clause-4 reference side         = 28
remaining-three spill           = 12 on clause-4 reference
```

Frozen controls:

```text
claim-pair + clause-4 claim_next_bridge:
  7485 / 539 / 2112 / 542 / 542 / 0

exact claim_flat_domain sheet:
  7317 / 545 / 2076 / 530 / 530 / 0

exact claim_sharp_codomain sheet:
  7317 / 545 / 2076 / 530 / 530 / 0

exact clause-2 reference sheet:
  7149 / 551 / 2040 / 518 / 518 / 0

exact clause-2 reference sheet plus clause-4 reference companion:
  neutral, same 7149 / 551 / 2040 / 518 / 518 / 0

first exact claim-flat / claim-sharp reopenings under clause-4 reference companion:
  7236 / 550 / 2058 / 524 / 524 / 0
```

The next active slice is the smaller exact clause-`2` `reference = 6` slice
below the broader clause-`4` `reference` companion, before reopening the whole
remaining-three spill or higher branches.

### Regression Coverage Recorded In The Live Docs

Recent targeted verification is recorded as green for:

```text
cargo test -p pen-type clause_four_reference_side -- --nocapture
cargo test -p pen-search current_claim_step_fifteen_live_reference_ -- --nocapture
cargo test -p pen-search desktop_claim_shadow_smoke_config_ -- --nocapture
cargo test -p pen-search demo_early_steps_restore_full_clause_catalog_generation -- --nocapture
cargo test -p pen-search desktop_claim_shadow_step_five_keeps_claim_generic_surface_under_demo_budget -- --nocapture
cargo test -p pen-search current_claim_step_fifteen_post_reference_exact_two_step_ -- --nocapture
cargo test -p pen-search current_claim_step_fifteen_live_repair_partial_prefix_wall_localizes_to_eight_frontier_remainder_branches_on_the_repaired_head -- --nocapture
cargo test -p pen-type claim_next_bridge_side_on_exact_reference_sheet -- --nocapture
cargo test -p pen-type outside_exact_reference_clause_two_sheet -- --nocapture
cargo test -p pen-cli claim_evidence -- --nocapture
```

## Invariants To Preserve

- `strict_canon_guarded` remains authoritative.
- `desktop_claim_shadow` stays a certified shadow/evidence lane.
- Accepted step `15` remains canonical `DCT 103 / 8`.
- Stored step `1` stays at `2144 / 1285 / 1 / 475` unless intentionally
  reopened.
- Stored step `15` stays no worse than `7211 / 553 / 2052` unless a stronger
  audited bundle replaces `v15`.
- The isolated `single` pocket stays fenced.
- Stronger lifted `89 / 8` terminals do not become fully scored/live.
- Compare, benchmark, and certificate artifacts remain refreshable through
  native `pen-cli`.
- Claim-lane behavior remains separate from demo-only behavior.
- Any stronger external wording remains tied to the stored certificate and
  disclosed desktop bundle.

## Honest Claim Boundary

The repository can currently say that the `desktop_claim_shadow` claim lane has
a stored compare/benchmark/certify-passing bundle that preserves the guarded
accepted sequence through step `15` and exposes the remaining local step-`15`
quality wall honestly.

It should not say that the claim lane is now the authoritative lane, that the
system has performed unbounded autonomous discovery, or that the remaining
step-`15` wall is solved. The live open gate is still:

```text
clean step-15 partial-prefix wall below 553
```
