# Search Realism Plan

Last updated: 2026-03-15

This file replaces the old closeout-only tracker. The active planning target is
to make the live search meaningfully more realistic while preserving the
current deterministic 15-step realization.

## Fixed starting point

Treat the following as current executable truth:

- `pen-cli run` and `pen-cli resume` recover the current 15-step sequence.
- The accepted late-step executable canon remains step 15 / `DCT` at `nu = 103`.
- The current live path is deterministic, exact, and name-free on the hot path.
- The current engine evaluates the full surviving candidate pool for each step
  before post-hoc retention annotations are attached.
- The current live lane is still heavily constrained by step-locked
  admissibility and fixed late-shell enumeration.
- The current `runs/debug-full` evidence shows that step 4 has a small real
  pool: `enumerated=4`, `evaluated=4`, `retained=4`.
- The current `runs/debug-full` evidence shows that steps 5 to 15 collapse to
  singleton late-step search: `enumerated=1`, `evaluated=1`, `retained=1`.
- Current frontier retention is mostly candidate-level evidence on already
  scored full telescopes, not a broad prefix frontier that shapes discovery.

## Problem statement

The current repo can honestly claim successful bounded live recovery through
step 15, but it cannot honestly claim that late discovery is already a
realistic wide structural search. The main blockers are:

- hard step-locked admissibility and package gates
- fixed late-shell constructors that collapse late enumeration to one path
- frontier retention that is not yet connected to a genuinely competing prefix
  search

That means "widening the beam" is not the first move. The correct order is:

1. relax the step-locked admissibility and package gates
2. replace fixed late-shell constructors with genuinely generative late
   enumeration
3. only then wire and tune frontier retention on a search surface that actually
   contains competing prefixes and candidates

## Main goal

Transform the current bounded exact-shell lane into a more realistic
structural-search lane that:

- still deterministically realizes the current 15-step sequence
- still keeps exact arithmetic and name-free acceptance
- generates real late-step competition instead of singleton shell replay
- makes frontier retention materially affect search behavior rather than only
  reporting metadata

## Non-goals

The following are explicitly out of scope for this plan:

- changing the accepted 15-step canon
- moving semantic names into core crates
- replacing exact arithmetic with heuristic floating-point scoring
- claiming open-ended anti-junk frontier search before the evidence exists
- using fixed late-step shell lists in the final target lane

## Hard preservation rules

Every workstream below must preserve these invariants:

- The current 15-step accepted telescope sequence remains the reference truth.
- `rho`, bar comparison, and acceptance ordering remain exact and deterministic.
- The accepted candidate must still be selected from the full semantically
  minimal pool that survives the current lane for that mode.
- Any relaxed or generative mode must ship behind a comparison and preservation
  harness before it becomes the default live path.
- Reporting and Agda remain downstream and observational only.
- The hot path must remain structural and name-free.

## Success criteria

This effort is only complete when all of the following are true:

- guarded mode still recovers the exact current 15-step sequence
- a relaxed shadow lane recovers the same 15-step sequence from genuine
  multi-candidate late-step search
- late steps 10 to 15 no longer rely on fixed singleton shell constructors
- late steps 10 to 15 each exhibit more than one evaluated candidate in at
  least one realistic shadow profile
- frontier retention influences real prefix survival and exploration order,
  not just below-bar reporting metadata
- the docs and reports describe the new honesty boundary correctly

## Execution order

The order below is deliberate and should not be inverted:

1. Workstream 0: add preservation harnesses and baseline search-space evidence
2. Workstream 1: relax hard admissibility and package gates into staged
   structural preference
3. Workstream 2: replace fixed late-shell constructors with generative late
   enumeration
4. Workstream 3: wire and tune actual frontier retention on the now-broader
   search surface
5. Workstream 4: compare, harden, and only then consider making the realistic
   lane the default

Workstream 3 should not start in earnest until Workstream 2 produces real late
candidate competition. Otherwise "beam widening" will mostly be moving zeros and
ones around.

## Current status snapshot

Status as of 2026-03-15:

- Workstream 0 is materially in place for the current repo scope.
- Workstream 1 is complete and its exit gate is now covered by executable
  evidence.
- Workstream 2 is complete and its exit gate is now covered by executable
  evidence.
- Workstream 3 is complete and its exit gate is now covered by executable
  evidence.
- Workstream 4 is complete and its exit gate is now covered by executable
  evidence.

What is already accomplished in the repo:

- explicit search-profile labeling now exists in runtime config, reports,
  inspect output, telemetry, and integration coverage
- shadow config profiles now exist for:
- `strict_canon_guarded`
- `relaxed_shadow`
- `realistic_frontier_shadow`
- preservation and comparison scaffolding from Workstream 0 now exists in the
  repo:
- guarded search-space fixtures
- run-comparison support
- integration coverage for guarded cold run, replay, resume, and resume
  compatibility behavior
- Workstream 1 has a first executable pass:
- admissibility now has an explicit mode split between guarded and relaxed
  shadow behavior
- package-family handling is no longer only booleans; there is now a staged
  policy surface:
- `forbid`
- `allow`
- `prefer`
- `require`
- the engine now records admissibility diagnostics by decision class and reason
- the CLI report JSON and debug output now surface admissibility diagnostics
- `run` and `resume` now route the selected search profile into the actual
  search engine, not just into labels
- regression coverage now asserts:
- guarded behavior still holds for the current canon
- relaxed shadow demotes modal step 10 from hard `require` to `prefer`
- relaxed shadow late steps 10 to 12 now expose genuine multi-candidate
  evaluation while preserving the accepted reference trajectory
- bounded relaxed late-family clause unions now prevent the generic late-grammar
  explosion that previously collapsed the shadow lane into runaway enumeration
- guarded-versus-relaxed comparison artifacts now report admissibility
  diagnostics alongside accepted-hash and search-space deltas
- integration coverage now freezes the guarded-versus-relaxed delta surface
  through step 12 and asserts that the deltas occur only where intended
- Workstream 2 has an executable closeout:
- the primary realistic lane no longer uses fixed late-constructor early
  returns for steps 10 to 15
- operator, Hilbert, and temporal late families now run through bounded
  generative clause families plus positional structural filters on the general
  late DFS lane
- temporal historical reanchor connectivity now accepts the realistic shadow
  step-15 family variant rather than only the exact reference DCT shell
- realistic shadow late steps 13, 14, and 15 now expose genuine
  multi-candidate evaluation while preserving accepted-hash parity with the
  reference replay
- integration and unit coverage now freeze realistic shadow late competition
  through step 15

What remains after Workstream 4:

- broader historical-anchor demotion beyond the step-10-to-12 closeout target
  remains later realism work where it is still justified
- any future default-lane promotion still requires new evidence beyond the
  completed Workstream 4 boundary

## Workstream 0: Baseline and preservation harness

Goal:

- make every later search-space change measurable, comparable, and reversible

Deliverables:

- add machine-readable baseline fixtures for per-step search-space stats:
  `enumerated`, `well_formed`, `admissibility_rejected`, `evaluated`,
  `canonical`, `semantically_minimal`, `retained`
- add a comparison helper that can diff two run directories specifically on:
  accepted hashes, per-step counts, and late-step candidate competition
- add an explicit search-mode label to reports so guarded and relaxed runs can
  be compared without ambiguity
- add a preservation test matrix for:
- current guarded live search through step 15
- step resume from stored reports
- reevaluation from stored reports
- frontier-backed resume compatibility behavior
- add shadow-run config profiles, for example:
- `strict_canon_guarded`
- `relaxed_shadow`
- `realistic_frontier_shadow`

Primary file targets:

- `crates/pen-cli/src/report.rs`
- `crates/pen-search/src/config.rs`
- `tests/integration/atomic_bootstrap.rs`
- `tests/integration/deterministic_replay.rs`
- `tests/integration/resume_roundtrip.rs`
- `scripts/compare_runs.py`
- `tests/fixtures/`

Exit gate:

- every future search change can be run in guarded mode and shadow mode and
  compared automatically
- the exact current 15-step sequence is locked by tests and fixtures before any
  relaxation work begins

## Workstream 1: Relax step-locked admissibility and package gates

Goal:

- stop collapsing late search to a single admissible family before enumeration
  has a chance to compete

Status:

- complete on 2026-03-14

Current constraints to change:

- `StrictAdmissibility` hard-selects one package family at a time
- late steps often force a single exact `kappa` band
- late structural debt currently turns "this family should be preferred" into
  "only this family may exist"

Planned changes:

- split admissibility into two layers:
- hard legality:
  well-formedness, connectivity, exact `kappa` bounds, anti-derivability,
  safety invariants
- soft structural preference:
  former, HIT, modal, connection, curvature, operator, Hilbert, temporal
  obligations
- replace boolean `require_*_package` gates with a staged policy model such as:
- `forbid`
- `allow`
- `prefer`
- `require`
- keep `require` only where the repo has a true semantic reason to do so, not
  just because the current late reference step is known
- widen late `kappa` handling from exact-singleton bands to narrow structurally
  justified bands whenever doing so does not break the reference recovery gate
- demote historical anchor refs from hard family locks into structural hints
  wherever possible
- add diagnostics for each admissibility decision so the engine can report why a
  candidate was excluded:
- rejected by exact legality
- rejected by structural debt cap
- admitted but de-prioritized
- admitted and focus-aligned

Implementation sequence:

1. keep steps 1 to 4 essentially unchanged so the early bootstrap remains a
   stable reference base
2. relax steps 5 to 8 carefully while maintaining current package inclusion
   tests
3. relax steps 9 to 15 from hard family locks to soft preference plus exact
   legality
4. only then widen `kappa` bands step by step where evidence shows singleton
   collapse is still dominated by hard admissibility rather than later ranking

Primary file targets:

- `crates/pen-type/src/admissibility.rs`
- `crates/pen-type/src/obligations.rs`
- `crates/pen-search/src/engine.rs`
- `crates/pen-cli/src/report.rs`
- `tests/`

Evidence to add:

- per-step admissibility diagnostics in JSON reports
- regression tests asserting that the reference telescope remains admissible in
  both guarded and relaxed shadow modes
- comparison artifacts showing where candidate counts increased and why

Exit gate:

- the guarded lane still recovers the exact 15-step sequence
- the relaxed shadow lane still contains the reference candidate at every step
- at least three late steps beyond step 9 now have more than one evaluated
  candidate without using fixed singleton shell constructors

Accomplished so far:

- introduced explicit admissibility modes so guarded and relaxed shadow lanes
  can differ semantically while preserving the same report surface
- split admissibility outcomes into:
- rejected by exact legality
- rejected by structural debt cap
- admitted but de-prioritized
- admitted and focus-aligned
- introduced staged package-policy machinery and focus-family tracking in
  `StrictAdmissibility`
- kept the current guarded lane exact by still translating guarded focus into
  `require` where the current executable canon depends on it
- wired search profile through `run`, `resume`, reevaluation, and step-report
  generation so relaxed shadow is a real engine path
- added admissibility diagnostics to step JSON and debug reporting
- added tests for:
- guarded exactness preservation
- relaxed modal step-10 policy demotion from `require` to `prefer`

Closeout evidence:

- the guarded lane still recovers the exact 15-step sequence through the full
  preservation suite
- the relaxed shadow lane keeps the reference telescope admissible through step
  15 and preserves the accepted reference trajectory through step 12
- late relaxed competition is now frozen at steps 10, 11, and 12, each with
  more than one evaluated candidate
- the guarded-versus-relaxed comparison artifacts now show accepted-hash parity
  with intentional deltas only at steps 10, 11, and 12

Exit gate status:

- met for the Workstream 1 target boundary
- guarded exact parity holds through step 15
- relaxed reference containment holds through step 15 admissibility checks
- steps 10, 11, and 12 each evaluate more than one candidate in relaxed shadow
- those late-step deltas are captured by fixtures and comparison artifacts

## Workstream 2: Replace fixed late-shell constructors with generative late enumeration

Goal:

- remove the late singleton constructor lists and recover the late steps from
  real grammar-driven competition

Status:

- complete on 2026-03-14

Current constraints to remove:

- dedicated early returns that bypass general enumeration for late shell modes
- fixed clause vectors for operator, Hilbert, and temporal bundles
- late-shell construction that behaves more like exact shell realization than
  structural discovery

Planned changes:

- remove the special-case late enumerators as primary search machinery:
- `enumerate_operator_bundle_telescopes`
- `enumerate_hilbert_functional_telescopes`
- `enumerate_temporal_shell_telescopes`
- convert late shell logic from constructor lists into generative positional
  filters and obligation-guided clause families
- make late enumeration grammar-driven again:
- clause positions should accept families of expressions
- filters should prune structurally illegal or off-target forms
- the reference telescope should survive because it is structurally included,
  not because its exact clause list was hard-coded
- keep family-specific clause-shape tests as filters, not as single-output
  constructors
- extend expression generation where needed for late competition:
- more library-reference patterns
- more higher-order binder patterns
- more modal and temporal interaction forms
- better support for nested `Pi`, `Sigma`, and library applications in late
  bundles

Migration order:

1. modal shell competition at step 10
2. connection shell competition at step 11
3. curvature shell competition at step 12
4. operator bundle competition at step 13
5. Hilbert functional competition at step 14
6. temporal shell competition at step 15

For each migrated family:

- keep a reference containment test
- add a competition test or artifact that proves more than one candidate was
  actually considered
- compare guarded and relaxed shadow acceptance results
- do not delete the old constructor path until the relaxed generative path has
  matched the accepted reference step repeatedly

Primary file targets:

- `crates/pen-search/src/enumerate.rs`
- `crates/pen-search/src/expand.rs`
- `crates/pen-search/src/engine.rs`
- `crates/pen-type/src/admissibility.rs`
- `tests/`

Evidence to add:

- per-step late-family candidate-count snapshots before and after each migration
- tests that the reference late step is generated by the general lane
- artifacts proving that accepted late steps were selected from a real late pool
  rather than a singleton exact shell

Exit gate:

- no fixed singleton late constructor lists remain on the primary realistic lane
- late steps 10 to 15 each have genuine multi-candidate evaluation in at least
  one realistic shadow profile
- the exact current 15-step sequence still wins under guarded mode and also
  survives in the realistic shadow lane

Accomplished so far:

- removed the remaining fixed late-constructor early returns from the primary
  late enumeration path
- steps 13 to 15 now use bounded generative clause families plus positional
  family filters on the same DFS lane already used by the earlier
  Workstream-2 migrations
- realistic shadow package matching and temporal reanchor connectivity now
  admit the same bounded temporal family used by enumeration, so the step-15
  competitor survives real evaluation instead of being pruned as a stale
  historical ref shell
- added unit coverage for:
- realistic shadow late competition at steps 13, 14, and 15
- temporal reanchor connectivity for the realistic shadow step-15 variant
- added integration coverage that runs `realistic_frontier_shadow` through step
  15 and asserts replay parity plus multi-candidate evaluation at every late
  step from 10 to 15

Closeout evidence:

- `cargo test -p pen-type connectivity_accepts_realistic_temporal_reanchor_variants -- --nocapture`
  passes
- `cargo test -p pen-search realistic_shadow_step_ -- --nocapture` passes
- `cargo test --test atomic_bootstrap realistic_shadow_run_preserves_reference_sequence_and_exposes_full_late_competition -- --nocapture`
  passes
- the pre-Workstream-3 realistic-shadow closeout run preserved the accepted
  reference trajectory and reported:
- step 10 `enumerated=7`, `evaluated=2`
- step 11 `enumerated=24`, `evaluated=2`
- step 12 `enumerated=24`, `evaluated=2`
- step 13 `enumerated=2`, `evaluated=2`
- step 14 `enumerated=2`, `evaluated=2`
- step 15 `enumerated=2`, `evaluated=2`
- the realistic shadow run still closes with
  `replay_ablation: matches_reference_replay x15` and step 15 / `DCT` at
  `nu = 103`

Exit gate status:

- met for the Workstream 2 target boundary
- no fixed singleton late constructor lists remain on the primary realistic lane
- late steps 10 to 15 each had more than one evaluated candidate on the
  Workstream-2 realistic-shadow baseline
- guarded and realistic shadow runs both preserve the current 15-step accepted
  trajectory

## Workstream 3: Wire actual frontier retention and then tune it

Goal:

- make frontier retention affect real search behavior only after the search
  surface contains meaningful competition

Status:

- complete on 2026-03-15

Current constraints to change:

- live bootstrap search currently enumerates whole candidate telescopes directly
- frontier retention is attached after scoring full candidates
- the existing frontier, scheduler, worker, and prefix-state modules are not yet
  the driver of the live bootstrap path

Planned changes:

- move the live engine from "enumerate complete telescopes, then annotate
  retention" toward "expand and rank partial prefix states, then complete the
  strongest survivors"
- wire `FrontierWindow`, `SchedulerPlan`, and worker batches into the live
  search path
- define exact lower and upper bounds for partial prefixes so branch-and-bound
  can prune safely without changing acceptance truth
- make retention classes operate on partial states as well as full candidates
- retain diversity across:
- obligation focus
- bridge heads
- support forms
- macro-heavy generic shells
- library-reference patterns
- modal and temporal interaction prefixes
- distinguish reporting counters for:
- prefix states explored
- prefix states pruned by exact bounds
- prefix states dropped by heuristic shaping
- full candidates evaluated
- full candidates accepted or retained
- keep cold spill and governor pressure logic, but only after prefix retention is
  connected to real search traffic

Tuning sequence:

1. wire prefix-state frontier in shadow mode only
2. verify exact parity with guarded acceptance on the current 15-step corpus
3. increase frontier diversity quotas and band breadth gradually
4. test pressure behavior with intentionally constrained runtime limits
5. only after parity and pressure tests pass, consider promoting the realistic
   frontier lane out of shadow mode

Primary file targets:

- `crates/pen-search/src/engine.rs`
- `crates/pen-search/src/frontier.rs`
- `crates/pen-search/src/scheduler.rs`
- `crates/pen-search/src/worker.rs`
- `crates/pen-search/src/branch_bound.rs`
- `crates/pen-search/src/state.rs`
- `crates/pen-search/src/diversify.rs`
- `crates/pen-store/src/manifest.rs`
- `crates/pen-cli/src/report.rs`
- `tests/`

Evidence to add:

- prefix-frontier stats in reports and checkpoint manifests
- tests showing that changing frontier quotas changes explored prefixes and
  pressure behavior
- parity tests proving that guarded acceptance remains unchanged on the current
  15-step sequence

Exit gate:

- frontier retention now changes real search work rather than only report
  metadata
- realistic frontier shadow runs preserve the accepted 15-step sequence
- pressure tests show spill and drop behavior on real competing prefixes

Accomplished so far:

- realistic shadow now groups admissible full telescopes by live terminal
  prefix before full-candidate expansion
- exact per-prefix lower and upper `nu` bounds are now recorded from the
  admissible completion set, and worker batches now sound-prune prefixes whose
  exact upper bound cannot clear the step bar
- prefix retention now changes real search work:
- only full candidates under retained prefix states are expanded and scored
- exact-pruned and heuristic-dropped prefixes no longer flow into full-candidate
  evaluation
- persisted frontier checkpoints are now built from the live prefix frontier
  itself rather than from retained full-candidate annotations
- frontier pressure and spill behavior for realistic shadow now reflect the
  prefix-frontier runtime rather than the candidate-level cold-retention report
  surface
- step reports, debug output, inspect output, comparison artifacts, and frontier
  manifests now surface:
- `prefix_states_explored`
- `prefix_states_exact_pruned`
- `prefix_states_heuristic_dropped`
- `prefix_frontier_hot_states`
- `prefix_frontier_cold_states`
- frontier manifest counts now also record:
- `prefix_states_explored`
- `prefix_states_exact_pruned`
- `prefix_states_heuristic_dropped`
- the preserved realistic shadow lane still closes with replay parity through
  step 15 while exposing nonzero prefix-frontier traffic across the late steps

Closeout evidence:

- `cargo test -p pen-search -p pen-cli -p pen-store` passes
- `pen-cli run --config configs/realistic_frontier_shadow.toml --until-step 15`
  still preserves the accepted reference trajectory through step 15 / `DCT` at
  `nu = 103`
- the realistic frontier shadow lane now accounts for each explored late prefix
  state as:
- retained hot
- retained cold
- exact-pruned by bound
- heuristic-dropped by shaping
- step summaries and frontier manifests now agree on the persisted prefix
  counters for the live realistic-shadow frontier
- constrained-memory pressure runs still exercise spill-backed frontier
  retention while preserving replay parity

Exit gate status:

- met for the Workstream 3 target boundary
- frontier retention now changes real search work rather than only report
  metadata
- realistic frontier shadow runs preserve the accepted 15-step sequence
- pressure tests now show spill and drop behavior on real competing prefixes

## Workstream 4: Rollout, parity, and honesty-boundary rewrite

Goal:

- ship the broader search honestly and only after parity is proven

Status:

- complete on 2026-03-15
- the guarded-versus-realistic rollout matrix is now frozen across cold,
  resume, reevaluate, and pressure lanes
- guarded remains authoritative; realistic frontier shadow remains
  comparison-backed

Planned changes:

- keep at least two explicit modes during rollout:
- `strict_canon_guarded`
- `realistic_frontier_shadow`
- optionally add `realistic_live` only after the shadow lane is stable
- run repeated comparison matrices:
- cold guarded run to step 15
- cold realistic shadow run to step 15
- resume and reevaluate lanes in both modes where meaningful
- constrained-memory pressure runs
- update docs so they state exactly what has changed:
- which lane is authoritative
- whether late steps are now genuinely competitive
- how frontier retention now participates in search
- keep historical donor and molecular provenance as audit material, not as
  hidden search shortcuts

Primary file targets:

- `README.md`
- `docs/ARCHITECTURE.md`
- `docs/SEARCH_CONTRACT.md`
- `plan_progress.md`
- `scripts/compare_runs.py`

Accomplished so far:

- `scripts/compare_runs.py` now freezes the full Workstream-4 rollout view on
  top of the existing lane diffing surface:
- parity-set status for `realistic_frontier_shadow` lanes against the guarded
  baseline
- resume-set status for realistic frontier-resume, compatibility-forced
  step-resume, and compatibility-forced reevaluate lanes
- pressure-set status for realistic lanes that exercise non-neutral governor or
  spill behavior
- realistic resume integration coverage now freezes:
- exact-match frontier resume
- compatibility-forced step resume
- compatibility-forced reevaluation
- each of the realistic resume paths preserves the accepted 15-step reference
  trajectory and retains the realistic search-profile evidence surface
- the README, architecture doc, and search-contract doc now agree on the
  current honesty boundary:
- `strict_canon_guarded` is authoritative
- `realistic_frontier_shadow` is broader but comparison-backed
- default-lane promotion is still future work, not current truth
- integration coverage now freezes the completed guarded-versus-realistic
  parity/resume/pressure comparison matrix

Closeout evidence:

- `python -m py_compile scripts/compare_runs.py` passes
- `cargo test --test atomic_bootstrap compare_runs_reports_workstream4_rollout_parity_and_pressure_sets -- --nocapture`
  passes
- `cargo test --test resume_roundtrip realistic_resume_roundtrip -- --nocapture`
  passes
- `cargo test --test deterministic_replay compare_runs_script_emits_a_deterministic_evidence_signoff -- --nocapture`
  passes
- the completed rollout matrix now covers:
- guarded cold baseline
- realistic cold baseline
- realistic frontier resume
- realistic compatibility-forced step resume
- realistic compatibility-forced reevaluate
- realistic constrained-memory pressure
- the compare output now reports Workstream-4 `ready` only when all of the
  following are present and parity-clean:
- realistic cold parity set
- realistic resume/reevaluate set
- realistic pressure set

Exit gate:

- the repo can honestly claim genuine multi-candidate late-step structural
  search through step 15
- the realistic lane has reproducible parity evidence against the current canon
- docs no longer describe the late live lane as broader or narrower than it
  really is

Exit gate status:

- met for the Workstream 4 target boundary
- genuine late-step multi-candidate search and live prefix-frontier retention
  were already established by Workstreams 2 and 3 and are now covered by the
  final rollout evidence matrix
- realistic cold, realistic resume, realistic reevaluate, and realistic
  pressure lanes now all preserve guarded accepted-hash parity through step 15
- guarded remains authoritative and the docs now say so consistently

## Risks and mitigations

Risk: relaxed admissibility breaks the current canon early.

Mitigation:

- keep guarded mode unchanged until relaxed shadow parity is demonstrated
- compare accepted hashes on every run

Risk: generative late enumeration explodes combinatorially.

Mitigation:

- widen one family at a time
- add exact partial bounds before turning on true frontier retention
- measure per-family growth before advancing to the next late step

Risk: hidden templates survive under new names.

Mitigation:

- remove fixed late constructor vectors from the realistic lane
- require that late reference telescopes are generated by grammar plus filters

Risk: frontier tuning happens before there is anything real to retain.

Mitigation:

- do not prioritize Workstream 3 until Workstream 2 produces real late
  competition

## Definition of done

This plan is complete only when the repo demonstrates all of the following at
the same time:

- exact recovery of the current 15-step sequence
- genuine multi-candidate late search
- no fixed late singleton constructors on the realistic lane
- frontier retention that shapes real prefix exploration
- documentation and reports that describe the new lane honestly
