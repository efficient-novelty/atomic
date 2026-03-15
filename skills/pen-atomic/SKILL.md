---
name: pen-atomic
description: Current-state architecture and donor guide for the `pen-atomic` Rust workspace. Use when working on live strict search, MBTT/kernel design, admissibility, exact selection, reporting, checkpoints, Agda export, or when you need to reconcile the current Rust implementation with older donor theory and Haskell provenance.
---

# PEN Atomic

## Overview

Use this skill as the current working memory pack for the repository as it exists
today, not as a description of an aspirational future rewrite.

The repo now has a real bounded live-search lane:

- `pen-cli run` and `pen-cli resume` perform live atomic strict search through
  step 15
- the accepted executable late-step canon is now the current Rust truth,
  including step 15 / `DCT` at `nu = 103`
- the CLI writes real run manifests, step checkpoints, reports, and telemetry
- debug reporting now includes retained valid candidates and human-readable
  telescope translations

What is still incomplete:

- `pen-store` is still partly contract-first
- the anti-junk frontier engine is not yet the full long-range design
- the Agda bridge is still lighter than the final proof-facing target

The current search-architecture focus has shifted again:

- `realistic_frontier_shadow` already has real prefix-frontier retention and
  persisted frontier evidence
- `pen-search` now has explicit `bounds.rs` and `prefix_cache.rs` primitives
  for quantum-inspired prefix work
- `realistic_frontier_shadow` now expands prefixes online through
  clause-position catalogs instead of enumerating all full telescopes first
- `PrefixSignature` now carries active-window hashing, shape/support summaries,
  and structural family flags for future d = 2 memoization
- realistic shadow now has a real `PrefixLegalityCache` that reuses
  incremental legality/connectivity summaries keyed by `PrefixSignature`
- realistic shadow now also keeps an exact incremental clause-family
  feasibility summary keyed by `PrefixSignature`, pruning mixed late-family
  prefixes before they reach terminal admissibility
- reports and frontier manifests now expose the plan-aligned counters
  `prefixes_created`, `full_telescopes_evaluated`,
  `canonical_dedupe_prunes`, and `semantic_minimality_prunes`
- reports, manifests, and inspect output now also expose the first memoization
  payoff counters `incremental_legality_cache_hits`,
  `incremental_connectivity_shortcuts`,
  `incremental_connectivity_fallbacks`, and
  `incremental_connectivity_prunes`, `incremental_clause_family_filter_hits`,
  and `incremental_clause_family_prunes`
- the next gap is stronger sound bound pruning beyond the landed exact
  clause-family impossibility prunes plus extending the new memo layer into
  cached terminal admissibility decisions, not "add a frontier for the first
  time"

Start with the current architecture doc before diving into donor material:

- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [../../overall_plan.md](../../overall_plan.md)
- [../../quantum_progress.md](../../quantum_progress.md)

## Working Rules

1. Treat the current Rust workspace and its tests as the source of truth for
   present executable behavior.
2. Use [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md) first when you need the
   current crate boundaries, runtime flow, reporting surface, or honesty
   boundary.
3. Treat `engine/src/*.hs` as donor provenance for unresolved semantics, not as
   automatically newer or truer than the current Rust implementation.
4. Keep the hot path name-free. Semantic labels belong in CLI reporting and Agda
   export layers only.
5. Separate three targets the repo still risks conflating:
   - the current strict executable Rust canon
   - paper-facing or aspirational targets
   - older historical 16-step or alternate late-shell variants
6. Do not mistake the existence of compatibility structs, manifest schemas, or
   placeholder storage modules for fully implemented runtime behavior.
7. Do not regress the repo back toward replay-or-template logic when the current
   bounded live atomic lane already works through step 15.

## First Reads

For most tasks, read in this order:

1. [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
2. [../../overall_plan.md](../../overall_plan.md)
3. [../../quantum_progress.md](../../quantum_progress.md) when the task touches
   prefix search, bounds, caching, or the quantum plan
4. [theory/README.md](theory/README.md)

Then branch based on the task.

## Reference Map

- Read [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md) for the current crate
  graph, runtime flow, reporting architecture, artifact model, and current
  honesty boundary.
- Read [../../README.md](../../README.md) for the current user-facing commands
  and smoke-test commands.
- Read [../../overall_plan.md](../../overall_plan.md) for current completion
  status, deliverables, and immediate next priorities.
- Read [../../quantum_progress.md](../../quantum_progress.md) for the current
  delta between the quantum improvement plan and the live Rust codebase.
- Read [theory/README.md](theory/README.md) when you need the theorem map or
  manuscript map.
- Read [theory/genesis.md](theory/genesis.md) when you need the exact strict
  15-step target.
- Read [theory/pen-model.md](theory/pen-model.md),
  [theory/coherence-and-scaling.md](theory/coherence-and-scaling.md), and
  [theory/novelty-selection-and-rejection.md](theory/novelty-selection-and-rejection.md)
  for the mathematical contract behind the search objective.
- Read [theory/late-framework-abstraction.md](theory/late-framework-abstraction.md)
  and [theory/terminal-dct.md](theory/terminal-dct.md) before making claims
  about steps 10 to 15.
- Read [references/01-project-brief.md](references/01-project-brief.md) for
  scope, donor priorities, and stale-doc warnings.
- Read [references/02-target-sequence.md](references/02-target-sequence.md) for
  the canonical 15-step target, current strict values, and bar arithmetic.
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
  when you need test or CI contracts, evidence requirements, or non-interference
  rules.
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
- [theory/genesis.md](theory/genesis.md)
- [theory/novelty-selection-and-rejection.md](theory/novelty-selection-and-rejection.md)
- [references/05-search-and-selection.md](references/05-search-and-selection.md)
- [references/06-atomic-research-lessons.md](references/06-atomic-research-lessons.md)
- [references/02-target-sequence.md](references/02-target-sequence.md)

Focus on:

- exact-band search and bar semantics
- admissibility from structural debt, not names
- deterministic dedupe and SCC minimality
- the remaining difference between the current realistic online prefix engine,
  the landed legality/connectivity memo path, and the still-missing earlier
  partial-prefix bound pruning
- using strengthened `PrefixSignature` state, the landed memo counters, and the
  remaining bound/admissibility gaps past the landed clause-family pruning as
  the starting point for further quantum-inspired search work

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
  strict 15-step lane.
- The repo now has real live atomic search through step 15, exact deterministic
  selection, and a richer candidate-level evidence surface.
- The current quantum-focused search gap is stronger partial-prefix bound
  pruning beyond the landed exact clause-family impossibility prunes plus
  extending the prefix-signature memo layer from legality/connectivity/family
  reuse into cached terminal admissibility decisions.
- Other big unfinished areas remain broader anti-junk frontier design,
  storage/runtime hardening beyond the current bounded resume lanes, the memory
  governor, and the stronger Agda contract.
- Start with [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md) for current
  behavior, then use the theory and donor references only as needed.
