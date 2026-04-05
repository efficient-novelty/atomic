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
  clean-tree completed `v10`, compare/certification/benchmark outputs now
  exist beside it, the certificate now also surfaces step-level breadth
  diagnosis from stored step summaries plus late-step live checkpoints, now
  including the full stored step-open pressure signature for failing steps,
  and the live claim blocker is no longer step-`4` RSS
  survival or stored step-`11` breadth repair but the remaining stored
  breadth misses on that canonical chain: stored `v10` still misses step `1`
  (`546 / 2144`) and step `15` (`1794 / 5000`), while step `10`, step `11`,
  step `12`, step `13`, and step `14` are now stored hits; the guarded local
  step-`11` breadth repair and the narrow step-`12` selector repair are both
  re-earned on clean stored evidence, the parity-preserving step-`13` repair
  is now also re-earned on stored evidence at `[5,1,3,3,5,3,2]` /
  `1350` / `2320` with canonical acceptance, and the next operational move is
  stored step-`15` diagnosis / repair on top of that `v10` bundle rather than
  another rerun first or another step-`13` theory pass first; the canonical
  `v10` certificate plus the frozen `step-15-live.ndjson` provenance are now
  also pinned by
  `stored_claim_v10_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`,
  so the current breadth miss anatomy is executable in-tree rather than
  notes-only; a new narrow local step-`15` repair now isolates that nearby
  clause-`3` anchor-`11` exact-argument pocket onto the live claim
  clause-`2` variants only, keeps the lifted anchor-`11` neighbors fenced,
  keeps clause `6` as the local safety boundary, and lifts the repaired
  canonical step `15` read from `DCT 103 / 8 / 1794` to
  `DCT 103 / 8 / 3972` while stored `v10` remains frozen at `1794` until the
  next rerun consumes it; user-facing wording stays at `bounded live recovery`
  until stored breadth and
  certification pass
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
  certification outputs now exist for the clean canonical completed `v10`
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
  stored `v10` still misses step `1` (`546 / 2144`) and step `15`
  (`1794 / 5000`), while step `10`, step `11`, step `12`, step `13`, and
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
  canonical `62 / 9 / 12027` continuation; locally, the repaired step-`15`
  read now stays `DCT 103 / 8 / 3972` while stored `v10` remains
  `DCT 103 / 8 / 1794` until the next rerun
- starting from clean-tree `v10` as the canonical stored bundle, but treating
  the next operational move as rerun / audit refresh on top of the new local
  step-`15` repair rather than another diagnosis-only pass
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
  canonical stored bundle is clean-tree completed `v10`, breadth still fails
  honestly at step `1` and step `15`, stored step `13` is now re-earned, and
  the guarded local step-`11` breadth repair plus the narrow step-`12`
  selector repair are now both re-earned on stored evidence.
- The next operational claim-lane work should focus on clean canonical-bundle
  diagnosis at stored step `15`, while keeping step `1` explicit and stored
  step `13` frozen as a hit, rather than on reopening another local
  step-`11` theory slice, another runtime-only step-`4`
  micro-optimization, another rerun first, or new step-`13`
  band/reanchor widening.
- Start with [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md), then load only
  the track-specific references you actually need.
