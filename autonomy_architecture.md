# Autonomy Architecture Reading Guide

## Purpose

This guide is for the `desktop_claim_shadow` autonomy workstream. Its scope is
the code that actually computes, selects, persists, and audits a live run that
discovers the full current `15`-step sequence. It is intentionally narrower
than the whole repository.

If you only remember one thing, remember this:

- the authoritative execution spine is
  `configs/desktop_claim_shadow_1h.toml`
  -> `pen-cli run`
  -> `pen-cli/report`
  -> `pen-search/engine`
  -> `pen-type` and `pen-eval`
  -> `pen-store`
  -> `scripts/compare_runs.py` and `scripts/certify_claim_lane.py`

## What "autonomously discover the full 15-step sequence" means here

In this repo, discovery is not just "produce some plausible late telescope."
The full loop is:

1. start from an empty accepted prefix
2. rediscover accepted steps `1..15` in order with live search
3. compute exact `nu`, `rho`, `kappa`, admissibility, connectivity, and
   minimality on the hot path
4. select the accepted winner deterministically
5. persist step artifacts and run metadata as the search advances
6. compare the stored claim run against the guarded baseline
7. certify parity, breadth, honesty, and artifact completeness from disk

That is why the relevant code is not only the search engine. The artifact and
audit layers are part of the autonomy architecture.

## Operational Memory Map

Use the autonomy files by contract, not by habit:

- `autonomous_progress.md`: live snapshot only. Current blocker, active
  diagnosis, latest counters, and the canonical operating position.
- `autonomous_next_steps.md`: current work order only. One active slice, one
  explicit success target, and one "not the next move" fence.
- `autonomous_plan.md`: medium-horizon phases only. Milestones, dependencies,
  and exit criteria.
- `autonomous_checklist.md`: binary signoff gates only. What must be true
  before the claim lane can graduate.
- `autonomous_ledger.md`: append-only probe history. Negative controls,
  tradeoff controls, and the reason earlier branches were ruled out.
- `skills/pen-atomic/SKILL.md`: routing and stable repo context only.
- `skills/pen-atomic/references/13-current-claim-lane.md`: stable claim-lane
  truths, invariants, and reading pointers only.

If a number changes often, it belongs in `autonomous_progress.md`. If a note
starts with "we tried", it belongs in `autonomous_ledger.md`.

## First Reading Order

Read these in order before branching into deeper code:

1. `autonomous_progress.md`
2. `autonomous_next_steps.md`
3. `autonomous_plan.md`
4. `autonomous_checklist.md`
5. `autonomous_ledger.md`
6. `skills/pen-atomic/SKILL.md`
7. `docs/ARCHITECTURE.md`
8. `skills/pen-atomic/references/13-current-claim-lane.md`
9. `skills/pen-atomic/references/02-target-sequence.md`
10. `configs/desktop_claim_shadow_1h.toml`
11. `crates/pen-search/src/config.rs`
12. `crates/pen-cli/src/cmd_run.rs`
13. `crates/pen-cli/src/report.rs`
14. `crates/pen-search/src/engine.rs`

After that, read supporting modules in this order:

1. `crates/pen-search/src/enumerate.rs`
2. `crates/pen-search/src/prefix_memo.rs`
3. `crates/pen-search/src/prefix_cache.rs`
4. `crates/pen-search/src/accept.rs`
5. `crates/pen-search/src/expand.rs`
6. `crates/pen-type/src/admissibility.rs`
7. `crates/pen-type/src/connectivity.rs`
8. `crates/pen-type/src/obligations.rs`
9. `crates/pen-eval/src/bar.rs`
10. `crates/pen-eval/src/nu.rs`
11. `crates/pen-eval/src/minimality.rs`
12. `crates/pen-store/src/manifest.rs`
13. `crates/pen-store/src/frontier.rs`
14. `crates/pen-store/src/checkpoint.rs`
15. `scripts/compare_runs.py`
16. `scripts/certify_claim_lane.py`
17. `scripts/benchmark_claim_lane.py`

## The Smallest Useful Mental Model

The code is split into six layers:

| Layer | Main files | Why it matters |
| --- | --- | --- |
| Target and fixtures | `skills/pen-atomic/references/02-target-sequence.md`, `crates/pen-core/src/telescope.rs` | Defines the current executable canon and the reference telescopes used by tests. |
| Runtime/profile selection | `configs/desktop_claim_shadow_1h.toml`, `crates/pen-search/src/config.rs` | Chooses the claim lane, worker policy, memory thresholds, and demo/claim floors. |
| Run orchestration | `crates/pen-cli/src/cmd_run.rs`, `crates/pen-cli/src/report.rs` | Turns config into a run, feeds the engine, and persists artifacts incrementally. |
| Search engine | `crates/pen-search/src/engine.rs` | Grows the library one step at a time and owns discovery, proof-close, and final acceptance. |
| Structural truth | `crates/pen-type/*`, `crates/pen-eval/*`, `crates/pen-search/src/expand.rs`, `crates/pen-search/src/accept.rs` | Determines whether a candidate is legal, connected, minimal, and bar-clearing. |
| Evidence and signoff | `crates/pen-store/*`, `scripts/compare_runs.py`, `scripts/certify_claim_lane.py` | Decides whether the stored run is honest, complete, and parity-safe. |

## The Canonical Execution Path

### 1. Config selects the claim lane

Start at `configs/desktop_claim_shadow_1h.toml`. That file sets:

- `search_profile = "desktop_claim_shadow"`
- `until_step = 15`
- `window_depth = 2`
- memory/governor limits
- the shared demo/claim floor and narrative settings used by the claim lane

Then read `crates/pen-search/src/config.rs`.

This file defines:

- `RuntimeConfig`
- `SearchProfile`
- `SearchProfile::DesktopClaimShadow`
- worker resolution and all runtime knobs consumed later by `pen-cli` and
  `pen-search`

### 2. `pen-cli run` builds the run and chooses the search policy

Read `crates/pen-cli/src/cmd_run.rs`.

This is the top-level executable entry point for autonomy work:

- `run(...)` loads config, creates the run directory, and wires the observer
  that receives live checkpoints and accepted steps
- `build_run_manifest_base(...)` and `build_run_manifest(...)` create the
  persisted `run.json`
- `search_policy_info(...)` is where the claim lane is recorded honestly as:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`
- `RunArtifactWriter` is responsible for incremental persistence, including
  failure-safe partial artifacts

This is the first place where "claim lane autonomy" becomes durable stored
evidence rather than just in-memory behavior.

### 3. `pen-cli/report` is the bridge from CLI to engine

Read `crates/pen-cli/src/report.rs`.

Important responsibilities:

- `search_atomic_bootstrap_steps_*` wraps the engine and converts engine output
  into persisted `StepReport`s
- `StepReport` is the stable summary surface that downstream tools consume
- `StepSearchStats` records the search-space counts, exact-screen reasons,
  claim step-open diagnostics, root-seeding diagnostics, timing, and frontier
  counts that matter for certification

If you want to know what an autonomous run must leave behind for later audit,
`StepReport` is the answer.

### 4. `pen-search/engine.rs` is the real discovery spine

Read this file before anything else in `pen-search`.

The most important functions are:

- `search_bootstrap_from_prefix_internal(...)`
- `search_next_step_internal(...)`
- `discover_realistic_shadow_candidates(...)`
- `select_acceptance_for_step(...)`
- `build_step_result(...)`

The engine keeps two live structures while moving step by step:

- `library`: the accepted telescopes so far, used as anonymous structural
  inputs for later search
- `history`: the accepted `(step, nu, kappa)` records used to compute the next
  exact bar

The core loop is:

1. evaluate any existing accepted prefix into `library` and `history`
2. for each new step, compute `structural_debt`, `StrictAdmissibility`, and
   the exact objective bar
3. discover candidates
4. dedupe them canonically
5. reject semantically non-minimal candidates
6. select the accepted bar-clearer deterministically
7. return an `AtomicSearchStep` that `pen-cli` can persist

## The Search Engine Internals That Actually Matter

### Structural target state

Read these as the kernel of "what a candidate is":

- `crates/pen-core/src/expr.rs`
- `crates/pen-core/src/clause.rs`
- `crates/pen-core/src/telescope.rs`
- `crates/pen-core/src/canonical.rs`
- `crates/pen-core/src/library.rs`
- `crates/pen-core/src/rational.rs`

This layer defines the anonymous MBTT objects the engine manipulates. It also
defines the canonical hashing keys used later for deterministic dedupe.

`Telescope::reference(step)` in `crates/pen-core/src/telescope.rs` is not the
search implementation, but it is the most compact executable statement of the
current `15`-step target fixtures.

### Exact bar and exact scoring

Read:

- `crates/pen-eval/src/bar.rs`
- `crates/pen-eval/src/nu.rs`
- `crates/pen-eval/src/minimality.rs`
- `crates/pen-search/src/expand.rs`
- `crates/pen-search/src/accept.rs`

These files define the exact score/selection contract:

- `compute_bar(...)` produces the next step bar from accepted history
- `evaluate_candidate(...)` and `evaluate_checked_candidate(...)` compute exact
  `nu`, `rho`, `kappa`, hashes, and structural signals
- `select_acceptance(...)` chooses the minimal positive overshoot winner among
  bar-clearers

Without these files you can generate candidates, but you cannot honestly say
which one wins.

### Admissibility, obligations, and connectivity

Read:

- `crates/pen-type/src/admissibility.rs`
- `crates/pen-type/src/obligations.rs`
- `crates/pen-type/src/connectivity.rs`
- `crates/pen-type/src/check.rs`

This is the structural legality boundary.

For the claim lane, the most important fact is that
`SearchProfile::DesktopClaimShadow` maps to
`AdmissibilityMode::DesktopClaimShadow`, which changes:

- the claim-debt guidance surface
- the late-family surface
- claim anchor behavior
- late-step package requirements and widening behavior

Step-15 work depends especially on `connectivity.rs`, because the late blocker
is no longer raw generation alone; it is qualifier and reanchor evidence on a
connected temporal-shell surface.

## How The Claim Lane Differs From Guarded Search

### The profile switch

The claim path becomes real in three places:

1. `crates/pen-search/src/config.rs`
   - `SearchProfile::DesktopClaimShadow`
2. `crates/pen-search/src/engine.rs`
   - `admissibility_mode_for_profile(...)`
3. `crates/pen-cli/src/cmd_run.rs`
   - `search_policy_info(...)`

Those three together define the runtime identity of the claim lane.

### Enumeration surface and late widening

Read `crates/pen-search/src/enumerate.rs`.

Important types and responsibilities:

- `EnumerationContext`
- `LateFamilySurface`
- `EnumerationSurfaceDiagnostics`
- `build_clause_catalog(...)`
- `enumerate_telescopes(...)`
- `raw_clause_catalog_widths(...)`

For the claim lane, `EnumerationContext::from_admissibility(...)` maps the
late family surface to `LateFamilySurface::ClaimGeneric`.

That is the architectural switch that makes the step-`13` to step-`15`
claim-specific widened surface possible.

### Prefix frontier, legality cache, and exact screening

Read:

- `crates/pen-search/src/prefix_memo.rs`
- `crates/pen-search/src/prefix_cache.rs`
- `crates/pen-search/src/bounds.rs`
- `crates/pen-search/src/branch_bound.rs`

This is the late-step performance and correctness core.

The claim lane does not simply enumerate full late telescopes. It:

1. seeds root prefixes from a clause catalog
2. assigns each prefix a `PrefixSignature`
3. caches legality, connectivity, family-filter, and summary results
4. screens prefixes with exact partial-prefix bounds
5. retains prefix groups for proof-close
6. materializes only the groups that can still matter

The functions that matter most in `engine.rs` are:

- `discover_realistic_shadow_candidates(...)`
- `claim_try_summary_prune_before_materialization(...)`
- `materialize_terminal_prefix_group(...)`
- `materialize_terminal_prefix_group_from_summary(...)`
- `materialize_terminal_prefix_group_from_survivor_sketch(...)`
- `materialize_terminal_prefix_group_compact(...)`
- `load_terminal_prefix_group_for_proof_close(...)`

If autonomy work touches step `4` throughput or late-step breadth, it will
almost always land in this part of the code.

### Claim diagnostics and live checkpoints

Still in `engine.rs`, read these types:

- `ClaimStepOpenDiagnostics`
- `ClaimRootSeedingDiagnostics`
- `StepLiveCheckpoint`

These are the main surfaces that explain:

- which claim band opened
- how wide the raw catalog was
- how many roots were seen, screened, and enqueued
- how large the frontier, prefix cache, and legality cache became during the
  live run

These diagnostics are why claim runs can now be debugged from stored artifacts
instead of terminal memory alone.

## How A Step Becomes Accepted

After discovery and proof-close, `engine.rs` seals the step in this order:

1. full evaluated candidates are collected
2. canonical duplicates are removed by `canonical_hash`
3. semantic minimality is checked with
   `pen_eval::minimality::analyze_semantic_minimality(...)`
4. the accepted candidate is chosen with
   `select_acceptance_for_step(...)`
5. frontier retention is computed for reporting and resume

The baseline acceptance rule is in `crates/pen-search/src/accept.rs`:

- minimal positive overshoot over the exact bar
- then deterministic structural tiebreakers

The claim lane adds important local overrides in
`engine.rs::select_acceptance_for_step(...)`:

- step `11` same-primary tiebreak logic
- step `12` same-primary tiebreak logic
- step `14` same-primary continuation tiebreak logic
- claim viability tiebreaking for tied candidates that keep the next step alive

If parity diverges while generation looks healthy, this function is usually the
first place to inspect.

## Persistence And Audit Are Part Of The Architecture

Read:

- `crates/pen-store/src/manifest.rs`
- `crates/pen-store/src/checkpoint.rs`
- `crates/pen-store/src/frontier.rs`
- `crates/pen-store/src/spill.rs`
- `crates/pen-store/src/sqlite.rs`

These files define the stored autonomy surface:

- `run.json`
- step summaries
- step checkpoints
- frontier manifests and frontier-runtime payloads
- metadata DB rows for stored frontier generations

For autonomy work, the important point is that step checkpoints are the stable
truth unit, while frontier checkpoints are resume accelerators.

Then read:

- `scripts/compare_runs.py`
- `scripts/certify_claim_lane.py`
- `scripts/benchmark_claim_lane.py`

These scripts define what a successful stored claim run must prove:

- accepted-hash parity with guarded
- honest claim search-policy metadata
- no replay or unsafe fallback hiding inside the run
- breadth floors from stored step summaries
- narrative, prune-class, exact-screen, and manifest completeness

This means "architecture required for autonomy" includes the reporting schema,
not just the hot search code.

## The Most Important Tests To Read

Use tests as executable specs. Read these first:

- `crates/pen-search/src/engine.rs`
  - `bootstrap_search_discovers_the_first_fifteen_reference_telescopes`
  - `reference_dct_becomes_admissible_and_connected_after_the_live_hilbert_prefix`
  - `desktop_claim_shadow_late_steps_keep_reference_acceptance`
  - `claim_step_eleven_acceptance_prefers_guarded_leaner_same_primary_shell`
  - `claim_step_twelve_guarded_curvature_shell_survives_screening_and_wins_same_primary_selection`
  - `repaired_claim_step_twelve_late_path_has_scoped_step_thirteen_widening_before_proof_close`
  - `divergent_step_thirteen_acceptance_prefers_step_fourteen_viable_tied_candidate`
  - `current_claim_step_fifteen_exact_prunes_split_into_zero_admitted_families`
  - `current_claim_step_fifteen_zero_admitted_connectivity_surface_is_structurally_connected_but_unqualified`
- `tests/integration/atomic_bootstrap.rs`
  - CLI-level end-to-end assertions for step completion, claim compare output,
    certification output, and benchmark output
- `tests/integration/resume_roundtrip.rs`
  - stored artifact and frontier-resume expectations

If you need the fastest path to the late-step autonomy story, read the
step-`11` to step-`15` tests in `engine.rs` before reading every helper.

## Current Hotspots For Autonomous Claim Discovery

Given the current autonomy docs, the code hotspots are:

- Step `13` widening and residual exact-screen loss
  - start in `enumerate.rs` and `engine.rs`
  - especially `EnumerationContext`, claim surface diagnostics, and
    `discover_realistic_shadow_candidates(...)`
- Step `15` qualifier and historical reanchor evidence
  - start in `connectivity.rs`, `prefix_memo.rs`, and the
    `materialize_terminal_prefix_group*` path
- Stored parity and breadth signoff
  - start in `StepReport`, `run.json`, `compare_runs.py`, and
    `certify_claim_lane.py`

Those are the relevant places for the current "full 15-step autonomous
discovery" gap. Agda export, optional acceleration, and donor Haskell are not
the first places to spend time.

## What Not To Read First

These are real parts of the repo, but they are not on the critical path for
this autonomy slice:

- `crates/pen-cli/src/cmd_export_agda.rs`
- historical donor Haskell as a source of current truth
- placeholder or contract-first storage details that do not participate in the
  live `run` -> `step summary` -> `compare/certify` loop

## Practical Rule Of Thumb

When debugging or extending autonomy behavior, follow this order:

1. config and claim policy metadata
2. `engine.rs` step loop
3. admissibility and enumeration surface
4. prefix legality cache and materialization path
5. acceptance/tiebreak logic
6. persisted `StepReport` and `run.json`
7. compare/certify scripts

If a proposed change cannot be explained all the way through those seven
layers, it is probably not yet a complete autonomy fix.
