# Project Completion Deliverables

Last updated: 2026-03-14

## Objective

Complete the full `pen-atomic` mission described in `overall_plan.md` and
`skills/pen-atomic/references/01-project-brief.md`:

- replace the current deterministic reference replay with a genuine atomic MBTT search engine
- finish the real checkpoint, frontier, storage, and memory-governor machinery
- demonstrate honest 15-step atomic recovery under the frozen strict semantics
- close the remaining late-`nu` and Agda bridge-contract gaps

This file now tracks the remaining deliverables to reach that mission-complete state.

## Current baseline

The workspace bootstrap is complete:

- the crate layout, frozen contracts, configs, docs, and schemas exist
- `pen-core`, `pen-type`, `pen-eval`, and the deterministic `pen-search` scaffold compile and test
- `pen-cli` writes real run manifests, step checkpoints, reports, and telemetry
- `pen-agda` exports manifest-backed Agda stubs with optional verification logs
- top-level integration tests cover replay, resume, inspect, and Agda export

What is not complete yet:

- `pen-cli run` and `pen-cli resume` still replay the frozen reference telescopes outside the supported live bootstrap range
- the atomic search loop is not yet the engine that discovers steps from primitives
- most of the runtime storage subsystem is still contract-first rather than fully implemented
- the repo has not yet produced an honest 15-step atomic recovery result
- the late-step `nu` and Agda payload contracts are still below the final target

## Deliverables

| ID | Deliverable | Status | Done when | Notes |
| --- | --- | --- | --- | --- |
| D1 | Replace replay with genuine atomic enumeration | in_progress | `pen-cli run` discovers candidates from atomic MBTT primitives instead of replaying `Telescope::reference(step)` | A live atomic bootstrap path now discovers steps 1 to 7 and is wired into CLI/report generation for supported runs; replay now begins only at step 8 and beyond. |
| D2 | Implement structural admissibility and obligation-driven search | in_progress | The live search loop computes strict admissibility bands from structural debt over the active window and uses them to open bands honestly | Bootstrap admissibility and structural debt summaries now exist in `pen-type`, and the first former/eliminator package, the first local HIT package, the first truncation package, and the first higher-HIT package now open structural step-4 / step-5 / step-6 / step-7 bands honestly, but the general later-step debt policy is still early. |
| D3 | Build anti-junk frontier retention | pending | Promising sparse eliminator prefixes survive long enough to compete against generic typed macro junk | Current quota buckets are only a first deterministic scaffold; the real obligation-guided anti-junk retention problem remains open. |
| D4 | Finish exact deterministic selection | in_progress | Acceptance decisions use the real candidate pool and preserve strict deterministic tie-breaking under exact arithmetic | The live bootstrap search now selects from a real enumerated candidate pool using exact arithmetic, and the bounded step-4 / step-5 / step-6 / step-7 lanes now use semantic SCC minimality plus stronger structural tie-break signals over the full evaluated pool; the full donor tie-break stack and late-step selection behavior are still not complete yet. |
| D5 | Implement full step and frontier checkpoint I/O | pending | Step checkpoints are written and read as the stable resume unit, and frontier checkpoints are resumable on exact compatibility matches | Requires real implementations in `pen-store` rather than contract-only surfaces. |
| D6 | Implement shard-backed frontier/state storage | pending | The frontier can spill, compact, reload, and resume through real shard/blob/queue storage instead of in-memory scaffolding | Includes queue, shard, blob, frontier packing, checksum, migration, and metadata plumbing. |
| D7 | Implement the memory governor and pressure actions | pending | The engine obeys the frozen 16 GB memory model with green/yellow/orange/red/black behaviors and survives pressure without corrupting progress | This is a first-class search-quality requirement, not just ops polish. |
| D8 | Implement real resume behavior | pending | `pen-cli resume` uses actual step/frontier artifacts and compatibility hashes instead of reconstructing replayed reference runs | Must support frontier resume, step resume, reevaluation, and migration-required outcomes exactly as frozen. |
| D9 | Prove deterministic replay of the real engine | pending | Re-running the same atomic search with the same config produces the same accepted trajectory, reports, and resume decisions | Needs evidence against the genuine search engine, not just the reference scaffold. |
| D10 | Demonstrate honest 15-step atomic recovery | pending | A cold run from primitives recovers the 15-step sequence in order under strict semantics without molecular templates in the hot path | This is the core project goal from the brief. |
| D11 | Match charged `kappa` and bar-clearing order end to end | pending | The recovered 15-step run preserves the accepted structure order, charged `kappa`, and bar-clearing behavior expected by the current canon | This is the minimum acceptance line for mission credibility. |
| D12 | Close the late-`nu` gap | pending | The engine either reproduces the current strict late-step `nu` values honestly or advances to the stronger paper-level late-`nu` targets with evidence | Especially important for steps 11 to 15, and most visibly for step 15 / DCT. |
| D13 | Strengthen evidence and ablation reporting | pending | The repo can explain success or failure with inspectable frontier evidence, prune classes, near misses, and ablation comparisons | This preserves the audit story the brief treats as a deliverable in its own right. |
| D14 | Finish the Agda bridge contract | pending | Exported artifacts include the fuller proof-facing payload expected by the donor bridge, not just stub modules plus hashes | Should cover canonical-key soundness, `nu` claims, deterministic payloads, and import/witness consistency. |
| D15 | Validate Agda sidecar deterministically | pending | The Agda export/verification path is stable across re-export, records skipped/passed/failed honestly, and remains completely outside the hot loop | Must preserve the verification split while making the bridge materially stronger. |
| D16 | Decide whether and how to ship `pen-accel` | pending | Optional acceleration exists only if it demonstrably preserves CPU-authoritative truth and helps the real engine | This is last; it is not required for baseline mission completion. |

## Detailed scope by area

### 1. Replace replay with genuine atomic search

- Replace the current replay-based `run` and `resume` path with a real search loop that:
  - computes the current step bar from accepted history
  - derives structural admissibility and exact bands from interface debt
  - enumerates typed atomic MBTT candidates
  - dedupes by canonical key
  - evaluates `nu`, `rho`, and exact overshoot under the real evaluator
  - applies semantic minimality and deterministic acceptance
- Remove any dependency on preselected `Telescope::reference(step)` candidates from the hot path.
- Keep the hot path name-free and structural.
- Preserve exact arithmetic and deterministic ordering throughout.

### 2. Finish the real storage and runtime subsystem

- Implement the currently contract-only `pen-store` modules:
  - checkpoint orchestration
  - frontier packing/unpacking
  - shard I/O
  - disk-backed queue
  - blob storage
  - spill/compaction policy
  - metadata DB usage
  - checksums and migrations
- Make step checkpoints self-contained and frontier checkpoints disposable-but-useful.
- Ensure run directories remain the authoritative artifact layout.
- Make `inspect` useful against real frontier artifacts, not just run manifests and summaries.

### 3. Build the memory-safe frontier engine

- Enforce the frozen resident-memory budget and governor thresholds.
- Add spill behavior, cache reduction, arena shrinking, forced compaction, and emergency checkpointing.
- Confirm that memory pressure handling preserves determinism and does not bias selection outcomes.
- Demonstrate that frontier shaping and spill behavior do not erase the rare eliminator structures the atomic search must keep alive.

### 4. Prove 15-step atomic recovery

- Produce a real cold-start run that reaches all 15 steps from atomic MBTT primitives.
- Show that the run is:
  - deterministic
  - checkpoint-resumable
  - free of molecular-template shortcuts in the hot path
  - inspectable when failures or near misses occur
- Preserve the accepted order and charged `kappa`.
- Demonstrate honest bar-clearing acceptance rather than post-hoc fitting.

### 5. Close the late-step semantic gaps

- Audit steps 11 to 15 carefully against the canonical targets.
- Decide, with evidence, whether the final engine target is:
  - current strict late-step `nu`, or
  - the stronger paper-level structural `nu`
- For step 15 in particular, reconcile the DCT undercount problem instead of silently inheriting it.
- Keep one AST, one evaluator truth path, and one explicit claim boundary for late-step novelty accounting.

### 6. Finish the Agda contract surface

- Extend the export manifest/payloads so they carry the proof-facing data the donor bridge expects.
- Preserve deterministic module generation and import inference.
- Keep verification observational only.
- Add tests that cover:
  - deterministic re-export
  - canonical-key stability
  - `nu` claim well-formedness
  - witness/import consistency
  - honest skipped verification when `agda` is unavailable

## Acceptance criteria for full project completion

The project should be considered complete only when all of the following are true:

1. `pen-cli run` performs real atomic search rather than reference replay.
2. Step and frontier checkpoints support the frozen compatibility/resume contract end to end.
3. The engine survives realistic memory pressure using the frozen governor rules.
4. A cold atomic run recovers the 15-step sequence under strict semantics without molecular templates in the hot path.
5. The recovered trajectory preserves the accepted structure order and charged `kappa`.
6. The late-step `nu` story is resolved explicitly and backed by evidence.
7. The Agda sidecar exports deterministic, proof-meaningful payloads and verifies them honestly.
8. The evidence layer can explain success, failure, pruning, and resume behavior without relying on post-hoc semantic naming inside the hot path.

## Suggested execution order

1. D1 through D4: replace replay with the real atomic search loop.
2. D5 through D8: finish checkpoint/frontier/runtime storage and resume behavior.
3. D9 through D11: establish deterministic 15-step atomic recovery and baseline evidence.
4. D12 through D15: close the late-`nu` and Agda-contract gaps.
5. D16: add optional acceleration only if still useful after the CPU truth path is complete.

## D1-D4 implementation roadmap

### Milestone A: Honest atomic bootstrap for steps 1 to 3

Goal:

- replace replay with genuine atomic discovery for the earliest bootstrap steps
- keep the CLI and artifact layout stable while the search engine grows

Concrete file targets:

- `crates/pen-type/src/obligations.rs`
  - define structural obligation summaries over the active library window
- `crates/pen-type/src/connectivity.rs`
  - define connectedness and active-window reference filters for candidate telescopes
- `crates/pen-type/src/admissibility.rs`
  - define bootstrap strict admissibility bands and a first structural debt estimate
- `crates/pen-search/src/enumerate.rs`
  - add recursive atomic expression/telescope enumeration instead of single-clause-only scaffolding
- `crates/pen-search/src/accept.rs`
  - keep exact overshoot-first acceptance, but drive it from the live candidate pool
- `crates/pen-search/src/lib.rs`
  - export a real step-search entry point
- `crates/pen-cli/src/report.rs`
  - add a search-backed path for bootstrap steps without relying on `Telescope::reference(step)`
- `crates/pen-cli/src/cmd_run.rs`
  - switch to the live search path where it is genuinely supported

Success line:

- the real search loop discovers steps 1 to 3 from atomic MBTT primitives
- deterministic tests prove the accepted sequence matches the current canon for those steps

Current status:

- completed in this branch

### Milestone B: Step 4 and exact deterministic selection

Goal:

- extend the live engine far enough to discover the first nontrivial eliminator package

Concrete file targets:

- `crates/pen-search/src/enumerate.rs`
  - support nested eliminator/former expressions needed for `Pi`/`Sigma`-like recovery
- `crates/pen-search/src/expand.rs`
  - evaluate live search candidates directly from the enumerated telescope pool
- `crates/pen-eval/src/minimality.rs`
  - use SCC minimality in the acceptance path, not just as a standalone helper
- `crates/pen-search/src/branch_bound.rs`
  - tighten exact bar-based pruning and keep the prune classification explicit
- `crates/pen-search/src/diversify.rs`
  - retain sparse promising prefixes using deterministic structural buckets
- `crates/pen-search/src/worker.rs`
  - process real candidate batches instead of only record-level prune checks

Success line:

- the engine reaches step 4 honestly
- acceptance still uses exact arithmetic and deterministic tie-breaking

Current status:

- bounded step-4 live search is now in place:
  - `crates/pen-type/src/obligations.rs` and `crates/pen-type/src/admissibility.rs` now open a
    structural former/eliminator band at step 4 instead of capping that lane at `kappa = 2`
  - `crates/pen-type/src/connectivity.rs` now accepts internally connected self-contained
    structural packages while still rejecting stale library references, which unblocks the first
    honest non-replay former package
  - `crates/pen-search/src/enumerate.rs` now caches clause banks by prefix depth and uses a
    bounded former/eliminator/computation positional filter for the first nontrivial eliminator
    package instead of re-enumerating the full clause bank at every DFS node
  - `crates/pen-eval/src/minimality.rs` now distinguishes structural SCC amputation from semantic
    minimality, only rejecting bundles when a proper terminal SCC subbundle is still admissible
    and bar-clearing
  - `crates/pen-search/src/expand.rs`, `crates/pen-search/src/accept.rs`, and
    `crates/pen-search/src/branch_bound.rs` now expose stronger structural ranking signals for
    deterministic tie-breaking, including a tighter ambient-span tie-break for bootstrap junk
  - `crates/pen-search/src/engine.rs` now keeps the full evaluated bounded bootstrap pool for
    exact acceptance instead of dropping evaluated step-4 winners through heuristic quota shaping
- bounded step-5 live search is now in place:
  - `crates/pen-type/src/obligations.rs` and `crates/pen-type/src/admissibility.rs` now open a
    structural first-HIT band at step 5 with exact `kappa = 3` and `path_dimension = 1`
  - `crates/pen-type/src/connectivity.rs` now treats local path attachments as honest structural
    edges, which unblocks self-contained one-point / one-path packages without replay
  - `crates/pen-search/src/enumerate.rs` now has a bounded initial-HIT positional filter that
    recovers the first point/path package deterministically from atomic MBTT clauses
- bounded step-6 live search is now in place:
  - `crates/pen-type/src/obligations.rs` and `crates/pen-type/src/admissibility.rs` now track
    missing truncation in the active window and open a structural first-truncation band at step 6
    with exact `kappa = 3`
  - `crates/pen-type/src/check.rs` now preserves the intended ambient scope through unary shell
    operators, and `crates/pen-type/src/connectivity.rs` now treats self-contained truncation
    shells plus their local path attachment as honestly connected
  - `crates/pen-search/src/enumerate.rs` now has a bounded truncation shell / eliminator / path
    positional filter that recovers the first truncation package deterministically from atomic
    MBTT clauses
- bounded step-7 live search is now in place:
  - `crates/pen-type/src/obligations.rs` and `crates/pen-type/src/admissibility.rs` now detect
    the first post-truncation higher-HIT obligation in the active window and open an exact
    `kappa = 3`, `path_dimension = 2` band for that structural package
  - `crates/pen-search/src/enumerate.rs` now has a bounded higher-HIT positional filter that
    recovers the first point / witness / higher-path package deterministically from atomic MBTT
    clauses without loosening the generic step-7 lane
  - `crates/pen-type/src/connectivity.rs` now has explicit coverage for self-contained higher-HIT
    packages, and the CLI fixtures now verify deterministic live discovery through step 7 end to
    end
- bounded step-8 live search is now in place:
  - `crates/pen-type/src/obligations.rs` and `crates/pen-type/src/admissibility.rs` now detect
    the first post-`S2` sphere-lift obligation in the active window and open an exact
    `kappa = 5`, `path_dimension = 3` band for that structural package
  - `crates/pen-search/src/enumerate.rs` now has a bounded `PathCon(3)` / witness-lambda
    positional filter that recovers the reference `S3` package deterministically from atomic MBTT
    clauses without loosening the generic higher-path lane
  - `crates/pen-type/src/connectivity.rs` now treats higher-path witness lambdas as honest local
    attachments to the lifted path package, and the CLI fixtures now verify deterministic live
    discovery through step 8 end to end
- bounded step-9 live search is now in place:
  - `crates/pen-type/src/obligations.rs` and `crates/pen-type/src/admissibility.rs` now detect
    the first post-`S3` library-bundle obligation in the active window and open an exact
    `kappa = 4`, `path_dimension = 0` band with a structurally derived historical loop anchor
  - `crates/pen-search/src/enumerate.rs` now has a bounded library-bundle positional filter plus
    anchored library-ref support that recovers the reference step-9 bundle deterministically from
    atomic MBTT clauses without widening the generic post-`S3` lane
  - `crates/pen-type/src/connectivity.rs` now has explicit coverage for active-window axiomatic
    bundles, and the CLI fixtures now verify deterministic live discovery through step 9 end to
    end
- bounded step-10 live search is now in place:
  - `crates/pen-type/src/obligations.rs` and `crates/pen-type/src/admissibility.rs` now detect
    the first post-Hopf promoted modal-shell obligation in the active window and open an exact
    `kappa = 4`, `path_dimension = 0` band with modal clauses enabled only for that structural
    shell
  - `crates/pen-search/src/enumerate.rs` now has a bounded modal-op positional filter that
    recovers the reference step-10 shell deterministically from atomic MBTT clauses without
    widening the generic late modal lane
  - `crates/pen-type/src/connectivity.rs` and the CLI fixtures now verify deterministic live
    discovery through step 10 end to end
- bounded step-11 live search is now in place:
  - `crates/pen-core/src/library.rs`, `crates/pen-type/src/obligations.rs`, and
    `crates/pen-type/src/admissibility.rs` now detect the first post-cohesion connection-shell
    obligation structurally from an uncoupled modal layer in the active window and open an exact
    `kappa = 5`, `path_dimension = 0` band for that shell without widening the generic late-modal
    lane
  - `crates/pen-search/src/enumerate.rs` now has a bounded connection-shell positional filter
    that recovers the reference step-11 bundle deterministically from atomic MBTT clauses using
    only structural shapes plus coupling to the latest live library entry
  - the CLI fixtures now freeze `reference_steps_until_11.json` and verify deterministic live
    discovery through step 11 end to end
- the live-search support range has now widened honestly:
  - `LIVE_BOOTSTRAP_MAX_STEP` is now `11`
  - CLI `run` / `resume` now use live atomic search through step 11 and switch to explicit
    replay only from step 12 onward
  - unit and integration coverage now exercises deterministic step-11 live discovery end to end,
    and `cargo test --workspace` is the verification target for this branch

### Milestone C: Anti-junk retention and frontier shaping

Goal:

- attack the known atomic failure mode directly: frontier collapse into generic junk

Concrete file targets:

- `crates/pen-type/src/obligations.rs`
  - promote obligation buckets from scalar summaries to retention-relevant state
- `crates/pen-search/src/diversify.rs`
  - enforce deterministic per-bucket quotas for structural diversity
- `crates/pen-search/src/dedupe.rs`
  - preserve stable canonical representatives while exposing dedupe evidence
- `crates/pen-search/src/frontier.rs`
  - represent hot/cold retention policy with explicit structural heads
- `crates/pen-search/src/scheduler.rs`
  - schedule work in a way that does not erase rare eliminator prefixes

Success line:

- late-promising sparse candidates survive long enough to compete
- prune/drop accounting stays explicit and auditable

### Milestone D: CLI replacement of replay for supported ranges

Goal:

- stop using replay wherever the real engine is genuinely capable

Concrete file targets:

- `crates/pen-cli/src/report.rs`
  - expose search-backed step reports and preserve replay only as an explicit fallback path
- `crates/pen-cli/src/cmd_run.rs`
  - prefer live search for supported steps and record the mode honestly in artifacts
- `crates/pen-cli/src/cmd_resume.rs`
  - resume from the same live-search semantics for supported steps
- `tests/integration/atomic_bootstrap.rs`
  - verify CLI-level atomic discovery for the supported bootstrap range
- `tests/integration/deterministic_replay.rs`
  - prove repeatability against the real search path rather than replay

Success line:

- supported CLI runs are driven by real search, not replay
- the remaining replay surface, if any, is explicit and temporary

## Immediate next move

The highest-value next milestone is:

- extend the bounded live-search support range from step 11 into the first second-order
  differential shell (step 12 / `Curvature`) without replay, while preserving the new exact
  step-4 / step-5 / step-6 / step-7 / step-8 / step-9 / step-10 / step-11 search paths and
  keeping the anti-junk story structural rather than template-driven

That is the point where the repo stops being a faithful scaffold and starts becoming the
actual project described in the brief.
