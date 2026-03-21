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
  own configs, narratives, and policy metadata; it now uses claim-debt
  admissibility plus a claim-generic late surface, and it now uses a
  structural-generic claim bucket taxonomy; later `kappa 7-9` mutator packs
  and claim-path exactness rechecks are now landed in code/tests; the repo now
  also has claim-specific compare/certification tooling plus richer
  CPU/build/git/binary manifest fingerprints, while breadth evidence, parity
  signoff, runtime stability on the intended `claim-1h` auto-worker profile,
  and certification pass status still remain open
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
- remaining claim-band widening, breadth evidence, and certification work on
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
  [../../autonomous_plan.md](../../autonomous_plan.md) and
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
6. [../../autonomous_progress.md](../../autonomous_progress.md) when the task
   touches `desktop_claim_shadow`
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
  plus [../../autonomous_plan.md](../../autonomous_plan.md) and
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
  claim-path exactness rechecks are landed, while breadth evidence, stored
  parity signoff, and certification pass status are still open; manifest
  completeness is now landed in code/tests, but the intended
  `desktop_claim_shadow_1h` auto-worker profile still aborts before artifact
  flush; use `scripts/compare_runs.py` and `scripts/certify_claim_lane.py` as
  the current evidence surfaces before changing more search code
- treating the remaining breadth evidence, stored parity signoff, and
  certification work as the next real bottlenecks rather than continuing to tune
  already-landed admissibility, claim-path exactness tests, or bucket labels
- moving admissibility, mutation, scheduling, and certification toward
  family-agnostic structural evidence
- not using stronger words like `unguided` before the certification gate lands

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
  while breadth evidence, stored parity signoff, runtime stability on the
  intended auto-worker claim profile, and certification pass status are still
  open.
- The next operational claim-lane work should focus on stored breadth
  evidence, stored parity signoff, and stabilizing a full-profile stored claim
  bundle on the disclosed machine rather than on already-landed profile,
  manifest-field, admissibility, bucket scheduling separation, or the
  now-covered claim-path exactness rechecks.
- Start with [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md), then load only
  the track-specific references you actually need.
