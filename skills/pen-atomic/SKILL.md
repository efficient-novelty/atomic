---
name: pen-atomic
description: Current-state architecture and donor guide for the `pen-atomic` Rust workspace. Use when working on live strict search, `realistic_frontier_shadow`, `demo_breadth_shadow`, `desktop_claim_shadow`, MBTT/kernel design, admissibility, exact selection, reporting, checkpoints, Agda export, or when you need to reconcile current Rust behavior with donor theory and Haskell provenance.
---

# PEN Atomic

## Overview

Use this skill as a routing and stable-context guide for the repository as it
exists today, not as a live experiment log.

Treat these as stable repo truths:

- `pen-cli run` and `pen-cli resume` perform live atomic strict search through
  step `15`.
- `strict_canon_guarded` remains the authoritative executable lane.
- `realistic_frontier_shadow` is a live comparison-backed lane with real
  online prefix search, persisted frontier evidence, and detailed reporting.
- `demo_breadth_shadow` is a comparison-backed child of realistic shadow with
  runnable `5m`, `10m`, and `15m` profiles plus stored narrative/event
  artifacts.
- `desktop_claim_shadow` is a separate claim-lane scaffold with its own
  configs, narratives, policy metadata, compare/certification/benchmark
  tooling, manifest provenance, and failure-safe persistence.
- Claim-lane mechanisms such as claim-debt admissibility, a claim-generic late
  surface, structural-generic bucket scheduling, later `kappa 7-9` mutators,
  and claim-path exactness rechecks are landed in code and tests.
- Claim artifacts, manifests, live checkpoints, and certification surfaces are
  real; the lane must stay at `bounded live recovery` until a stored
  certificate passes.
- Current post-probe stored rerun evidence already keeps step-`15` repair
  ahead of any step-`1` reopening on newer code.
- Live claim-lane counters, active hypotheses, current blockers, and probe
  history no longer live in this skill file. Read the autonomous docs for that
  operating state:
  [../../autonomous_progress.md](../../autonomous_progress.md),
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md),
  [../../autonomous_plan.md](../../autonomous_plan.md),
  [../../autonomous_checklist.md](../../autonomous_checklist.md), and
  [../../autonomous_ledger.md](../../autonomous_ledger.md).
- Keep this skill file focused on stable repository truths; current probe
  outcomes belong in the autonomous docs rather than here.
- User-facing wording stays at `bounded live recovery` until stored breadth
  and certification pass.
- The accepted executable late-step canon is the current Rust truth, including
  step `15` / `DCT` at `nu = 103`

Treat these as still incomplete:

- `pen-store` is still partly contract-first
- the anti-junk frontier engine is not yet the full long-range design
- the Agda bridge is still lighter than the final proof-facing target

The current architecture focus is split between three active tracks:

- stronger exact late-step pruning and ordering on
  `realistic_frontier_shadow`
- honest breadth, budget, and evidence surfacing on `demo_breadth_shadow`
- rerun-confirmed step-`15` repair, stored breadth repair, and certification
  work on
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
  [../../autonomous_progress.md](../../autonomous_progress.md) for the live
  snapshot,
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md) for the
  active work order,
  [../../autonomous_plan.md](../../autonomous_plan.md) for medium-horizon
  phases,
  [../../autonomous_checklist.md](../../autonomous_checklist.md) for binary
  signoff gates, and
  [../../autonomous_ledger.md](../../autonomous_ledger.md) for append-only
  probe history.

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
6. [../../autonomous_progress.md](../../autonomous_progress.md),
   [../../autonomous_next_steps.md](../../autonomous_next_steps.md),
   [../../autonomous_plan.md](../../autonomous_plan.md), and
   [../../autonomous_ledger.md](../../autonomous_ledger.md) when the task
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
  plus [../../autonomous_progress.md](../../autonomous_progress.md),
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md),
  [../../autonomous_plan.md](../../autonomous_plan.md),
  [../../autonomous_checklist.md](../../autonomous_checklist.md), and
  [../../autonomous_ledger.md](../../autonomous_ledger.md) for the current
  claim-lane mixed state, active work order, signoff gates, and probe
  history.
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
- [../../autonomous_progress.md](../../autonomous_progress.md)
- [../../autonomous_next_steps.md](../../autonomous_next_steps.md)
- [../../autonomous_plan.md](../../autonomous_plan.md)
- [../../autonomous_checklist.md](../../autonomous_checklist.md)
- [../../autonomous_ledger.md](../../autonomous_ledger.md)
- [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)
- [references/08-evidence-and-invariants.md](references/08-evidence-and-invariants.md)

Focus on:

- keeping the claim lane separate from demo-only behavior
- use [../../autonomous_progress.md](../../autonomous_progress.md) as the
  canonical home for live counters, blocker anatomy, and the current
  operating position
- use [../../autonomous_next_steps.md](../../autonomous_next_steps.md) as the
  only home for the active slice and explicit "not the next move" guidance
- use [../../autonomous_ledger.md](../../autonomous_ledger.md) for probe
  history, negative controls, and why earlier ideas were ruled out
- treat the exhausted mismatch-`1` `reference + demo_flat_codomain` ladder as
  a live autonomy detail owned by the autonomous docs rather than by this
  skill file; read [../../autonomous_progress.md](../../autonomous_progress.md)
  and [../../autonomous_next_steps.md](../../autonomous_next_steps.md) for the
  current off-branch priority order before opening another late-step probe
- use [../../autonomous_plan.md](../../autonomous_plan.md) for
  medium-horizon sequencing and exit criteria rather than re-explaining the
  current slice
- keep certification language honest: the claim lane remains at
  `bounded live recovery` until a stored certificate passes
- use `scripts/compare_runs.py`, `scripts/benchmark_claim_lane.py`, and
  `scripts/certify_claim_lane.py` as the evidence surfaces before moving the
  search code again
- use the current canonical rerun plus earlier stored evidence and
  `run.json` build fingerprints to keep step-`15` repair ahead of step-`1`
  theory work; do not spend another turn on rerun-vs-step-`1` ordering
  unless a later stored bundle changes the diagnosis
- treat the step-`15` partial-prefix wall, the dominant clause-`0` / clause-`1`
  pairings, and the live clause-`4` split as live diagnosis owned by
  [../../autonomous_progress.md](../../autonomous_progress.md)
- treat the active slice and the exact next move as owned by
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md), not by
  this skill file
- keep the next repair above exhausted clause-`5` tail probes and above
  demo-only reopenings and above another exact claim-pair clause-`4`
  `reference`-side relocalization pass; that narrower reference-side tradeoff
  now only reproduces the broader clause-`4` `reference`-sheet result, so the
  skill should route you to the live docs rather than restating their full
  diagnostic ledger
- do not bypass `exact_partial_prefix_bound_decision(...)` for the whole
  remaining-two mismatch-`0` claim-domain tier; that parent-level release is
  now a ruled-out widening control owned by the autonomous docs and ledger, so
  read those files before touching exact-bound relief on the current step-`15`
  wall
- keep the next repair above the broad clause-`1` `demo_flat_codomain`
  reopening across the full mismatch-`0` claim-domain surface; that control is
  now a widening negative result owned by
  [../../autonomous_ledger.md](../../autonomous_ledger.md), so route to the
  live docs before reopening that surface again
- keep the next repair above the narrower mismatch-`0` clause-`4`
  `claim_next_bridge`-side relocalization too; that smaller negative control
  now also belongs to [../../autonomous_ledger.md](../../autonomous_ledger.md),
  so route to the live docs before retrying that same connectivity-only slice
- keep the next repair above the narrower mismatch-`0` clause-`4`
  `reference`-side relocalization too; that sharper negative control now also
  belongs to [../../autonomous_ledger.md](../../autonomous_ledger.md), so
  route to the live docs before retrying that same connectivity-only slice
- keep the next repair above remaining-one exact-summary relief on the
  mismatch-`0` clause-`4` `reference` plus clause-`5` `reference` tail too;
  that deeper negative control now also belongs to
  [../../autonomous_ledger.md](../../autonomous_ledger.md), so route to the
  live docs before retrying that same narrow reference-tail slice
- treat the whole mismatch-`0` clause-`4` `claim_next_bridge`-half
  remaining-one exact-summary tradeoff as live autonomy detail too; it now
  narrows the wall but still widens `small_cluster`, so route to the live
  docs before retrying that whole half and localize within its clause-`5`
  cells if that is still the active slice
- treat the narrower mismatch-`0` clause-`4` `claim_next_bridge` plus
  clause-`5` `claim_flat_codomain` and clause-`5` `reference` exact-summary
  tradeoffs as live autonomy detail too; they are smaller tradeoff controls,
  while the sibling clause-`5` `claim_next_codomain` cell is inert, so route
  to the live docs before retrying any of those whole-cell slices
- treat the deeper pair-cell subprobes below those active mismatch-`0`
  clause-`5` `claim_flat_codomain / reference` cells as live autonomy detail
  too; they are now symmetric smaller tradeoff controls across all `12`
  live pairings, so route to the live docs before retrying that pair-cell
  scope instead of assuming one pairing or active clause-`5` family is newly
  privileged
- treat the representative clause-`2` sheet subprobes below that pair-cell as
  live autonomy detail too; the two claim-side sheets are now matching smaller
  tradeoff controls while the reference sheet is neutral, so route to the live
  docs before retrying clause-`2` identity instead of assuming one sheet is
  newly privileged
- treat the sibling active mismatch-`0` clause-`5` `reference` family's
  representative clause-`2` split as live autonomy detail too; if the live
  docs already localize it to the same `4343 / 552 / 2268` smaller tradeoff
  shell with `small_cluster = 3141 / 522 / 522 / 0` on the two claim-side
  sheets and a neutral `4331 / 553 / 2271` sibling `reference` sheet, route
  there instead of retrying clause-`5` family identity one layer deeper as if
  it were fresh leverage
- treat the representative `claim_flat_domain` clause-`2` sheet's clause-`6`
  continuations as live autonomy detail too; the `claim_next_codomain`,
  `claim_sharp_codomain`, and `reference` continuations now all reland the
  same matched smaller tradeoff shell and differ only by a tiny deeper
  zero-admitted tail delta, so route to the live docs before retrying
  clause-`6` identity as if it were fresh leverage
- treat the deeper representative mismatch-`0` claim-side clause-`6`
  `reference` union as live autonomy detail too; if the live docs already pin
  that cross-sheet union at the same `4355 / 551 / 2265` pair-cell shell with
  `small_cluster = 3150 / 522 / 522 / 0`, plus the same zero-admitted
  `((0, None, None), 2265)` remaining-one family, the full `+18`
  `small_cluster` widening localized to six released `3`-generated /
  `0`-admitted `NeedsFallback` groups, and no cached bound or hidden live
  pocket, route there instead of reopening a claim-side union as if it were a
  fresh repair class
- treat the representative mismatch-`0` claim-side parent-route probes as
  live autonomy detail too; if the live docs already pin both scoped
  historical-reanchor routes on the active clause-`5`
  `claim_flat_codomain / reference` families at the same unsafe
  `4427 / 545 / 2247` negative-control shell with noncanonical `60 / 8`,
  `incumbent_dominance = 117`,
  `small_cluster = 2931 / 455 / 455 / 115`, a reopened `single` bucket, and a
  delta localized only to the four targeted claim-side remaining-two parent
  cells plus their `24` matching remaining-one pruned prefixes on the chosen
  active clause-`5` bucket, route there instead of reopening either
  parent-route sibling as if it were a fresh repair class
- treat the first alternate representative mismatch-`0` claim-side
  active-window qualification family as live autonomy detail too; if the live
  docs already pin both active clause-`5`
  `claim_flat_codomain / reference` probes at the same unsafe
  `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four-cell plus `24`-pruned-prefix targeted delta,
  route there instead of reopening that alternate qualification family as if
  it were a fresh repair class
- treat the next alternate representative mismatch-`0` claim-side
  self-contained qualification family as live autonomy detail too; if the
  live docs already pin both active clause-`5`
  `claim_flat_codomain / reference` probes at the same unsafe
  `4427 / 545 / 2247` shell with noncanonical `60 / 8`,
  `incumbent_dominance = 110`,
  `small_cluster = 2952 / 558 / 558 / 108`, the same reopened `single`
  bucket, and the same four-cell plus `24`-pruned-prefix targeted delta,
  route there instead of reopening that alternate qualification family as if
  it were a fresh repair class
- treat the narrower clause-`6` `reference` refinement of that same
  representative mismatch-`0` claim-side parent-route class as live autonomy
  detail too; if the live docs already pin both active clause-`5`
  `claim_flat_codomain / reference` refinements at unsafe noncanonical
  `74 / 8` with `retained = 1`, the same `4427 / 545 / 2247`,
  `incumbent_dominance = 111`,
  `small_cluster = 2904 / 430 / 430 / 108`, a still-fenced `single` bucket,
  and the same four-cell plus `24`-pruned-prefix delta, route there instead
  of reopening that marginally narrower clause-`6` selector as if it were a
  fresh repair class
- treat the representative claim-flat clause-`3` refinement inside that same
  mismatch-`0` claim-side parent-route family as live autonomy detail too; if
  the live docs already pin both `claim_flat_argument /
  claim_eventual_argument` branches on the active `claim_flat_codomain`
  bucket at the same smaller unsafe `4379 / 549 / 2259` shell with
  noncanonical `60 / 8`, `incumbent_dominance = 113`,
  `small_cluster = 2871 / 435 / 435 / 111`, and the same reopened `single`
  bucket, route there instead of reopening clause-`3` identity as if it were
  a fresh repair class
- treat the representative `claim_flat_domain` clause-`2` sheet's marginally
  best clause-`6` `reference` continuation's clause-`3` split as live
  autonomy detail too; the `claim_flat_argument` and
  `claim_eventual_argument` branches are now individually neutral controls on
  the untouched `4331 / 553 / 2271` baseline, and the broader
  `4343 / 552 / 2268` clause-`6` `reference` tradeoff only appears when both
  clause-`3` branches reopen together, so route to the live docs before
  retrying clause-`3` identity as if one branch were newly privileged
- treat that broader joint clause-`3` continuation below the representative
  `claim_flat_domain` clause-`2` sheet's marginally best clause-`6`
  `reference` shell as live autonomy detail too; it is now localized to one
  remaining-two parent capture plus its three clause-`6` remaining-one child
  continuations, so route to the live docs before retrying the whole joint
  continuation and instead localize the next probe to a finer terminal or
  remaining-one completion partition inside that parent/child shell
- treat that representative claim-flat parent/child shell as live autonomy
  detail too; its six clause-`3` / clause-`6` child continuations now all
  collapse to matched dead `3`-generated / `0`-admitted completion summaries
  with the same nonlive open-band structural terminal trio, so route to the
  live docs before spending another turn on the exhausted claim-flat shell
- treat the representative mismatch-`0` claim-flat dead-child reason split as
  live autonomy detail too; if the live docs have already localized that
  shell's first finer reason pass to one uniform clause-`2` blocker plus the
  same nonqualifying connectivity vector across all clause-`3` / clause-`6` /
  terminal continuations, route there instead of reopening that dead child as
  if another mismatch-`0` identity or reason reland were still fresh
- treat the representative mismatch-`0` claim-sharp dead-child completion and
  reason split as live autonomy detail too; if the live docs have already
  localized that shell to the same dead `3`-generated / `0`-admitted child
  summaries and the same uniform clause-`2` blocker plus nonqualifying
  connectivity vector, route there instead of reopening that sharper dead
  child as if another mismatch-`0` identity or reason reland were still fresh
- treat the isolated clause-`1` `demo_flat_codomain` exact-suffix side
  pocket as live autonomy detail too; if the live docs have already pinned
  that lone reland at `4349 / 556 / 2268` with
  `small_cluster = 3141 / 522 / 522 / 0` and the isolated `single` pocket
  still fenced, route there instead of reopening that side pocket as if it
  were still a fresh repair lead
- treat the promoted `reference / reference` tail split as live autonomy
  detail too; if the live docs have already localized that tail to mismatch
  `2 = 42` versus mismatch `3 = 12` with mismatch-`2` clause-`4` pressure
  still concentrated on `claim_next_bridge = 18` and `reference = 16`, route
  there instead of reopening broader mismatch-`0` or claim-safe shells as if
  the whole tail were still one undifferentiated lead
- treat the larger mismatch-`2` `reference / reference` clause-`4`
  `claim_next_bridge` and `reference` halves as live autonomy detail too; if
  the live docs have already pinned them as wider-`small_cluster` tradeoff
  controls, route there instead of reopening either larger half as if it were
  still a fresh safe repair lead
- treat the tiny mismatch-`2` `reference / reference` clause-`4`
  `demo_sharp_codomain` and `demo_sharp_bridge` pockets as live autonomy
  detail too; if the live docs have already pinned them as matched smaller
  tradeoff controls, route there instead of reopening either demo-side pocket
  as if it were still a fresh repair lead
- treat the smaller mismatch-`3` `reference / reference` clause-`4`
  `claim_next_bridge` and `reference` halves as live autonomy detail too; if
  the live docs have already pinned those halves as smaller or sharper
  tradeoff controls, route there instead of reopening either mismatch-`3`
  half as if it were still a fresh repair lead
- once the live docs say those two mismatch-`2` demo-side pockets and both
  mismatch-`3` halves are spent too, move to the next broader-backup
  comparison before reopening broader mismatch-`0` or claim-safe shells by
  restating the same `reference / reference` tail
- if that broader-backup comparison is already explicit in the live docs,
  keep the tighter representative mismatch-`0` claim-side clause-`2` shell
  ahead of the looser representative claim-safe claim-side shell; do not
  promote the claim-safe backup first unless the mismatch-`0` backup has been
  ruled out again by a later broader comparison rather than by another
  representative dead-shell reland
- treat the representative `claim_sharp_codomain` clause-`2` sheet's
  clause-`6` continuations as live autonomy detail too; the
  `claim_next_codomain`, `claim_sharp_codomain`, and `reference`
  continuations now also reland the same matched smaller tradeoff shell and
  differ only by a tiny deeper zero-admitted tail delta, so route to the
  live docs before retrying clause-`6` identity on the representative
  claim-sharp sheet as if it were fresh leverage
- treat the representative claim-safe mismatch-`1` clause-`4`
  `demo_sharp_codomain` and `demo_sharp_bridge` sides' exact pair-cell
  relands as live autonomy detail too; both sides now reland the same smaller
  negative-control shell on the `reference / claim_next_codomain` and
  `reference / claim_sharp_codomain` pairings, so route to the live docs
  before spending another turn swapping between those claim-safe pairings or
  clause-`4` side siblings as if one were newly privileged
- treat the representative claim-safe exact pair cell's clause-`2` split as
  live autonomy detail too; on the chosen
  `reference / claim_next_codomain / demo_sharp_codomain` cell, the two
  claim-side sheets are matched smaller negative controls while the sibling
  `reference` sheet is only a search-neutral control, so route to the live
  docs before retrying that clause-`2` identity split as if one sheet were
  newly privileged
- treat the representative claim-safe claim-side sheets' clause-`5` /
  clause-`6` shell as live autonomy detail too; beneath that same chosen cell,
  both claim-side sheets now reland the same six dead prefixes and every
  terminal continuation still needs fallback, so route to the live docs before
  retrying clause-`5` or clause-`6` identity there as if one sibling were a
  fresh lead
- treat the representative claim-safe `claim_flat_domain` dead prefix's
  terminal-family partition as live autonomy detail too; beneath
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`,
  all six dead prefixes now reland the same `3`-generated / `0`-admitted
  profile with the same `reference / eventual_lift / next_lift` trio, and
  every one of those completed telescopes is still structurally connected but
  outside historical reanchor and active-window qualification, so route to the
  live docs before retrying terminal-family identity there as if one terminal
  were newly privileged
- treat the representative claim-safe dead prefix's clause-`5`
  qualification split as live autonomy detail too; under the selected
  `reference / claim_next_codomain / claim_flat_domain / demo_sharp_codomain`
  exact pair, clause-`5` `reference` is the only exact control while
  `claim_flat_codomain / claim_next_codomain / demo_sharp_domain /
  demo_flat_codomain` all stop at `5 / mismatch 5`, and the live dead shell
  itself still uses only the two claim-side dead labels, so route to the live
  docs before retrying clause-`5` identity there as if a demo-side control
  were still live
- treat any deeper reason-level pass below that representative claim-safe dead
  prefix as live autonomy detail too; if the live docs have already localized
  the completed shell to the clause-`5` qualification wall, route there
  instead of reopening clause-`6` or terminal-family identity as if those
  controls were newly privileged
- keep the next slice above another mismatch-`0` representative claim-side
  reland; with both claim-side sheets now frozen through their deeper
  completion and first finer reason passes, and with the deeper
  clause-`6` `reference` union now also pinned as a reconstructive control,
  and with that union now shown to stay inside the same zero-admitted
  remaining-one dead surface as the pair-cell tradeoff, route to the live
  docs before spending another turn below the representative mismatch-`0`
  pair unless a later broader-backup comparison explicitly promotes a new
  branch
- treat negative controls, tradeoff controls, and probe outcomes as ledger
  entries owned by [../../autonomous_ledger.md](../../autonomous_ledger.md)
- keep the remaining local guardrails green while moving toward
  family-agnostic structural evidence and honest certification language
- keep nearby step-`13` widenings and anchor-`11` side pockets fenced unless a
  new active slice explicitly promotes them in
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md)

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
- The claim lane has real configs, artifacts, compare/certify/benchmark
  tooling, and landed claim-specific mechanics, but live counters and probe
  history now live in the autonomous docs rather than in this skill file.
- For current claim-lane status, read
  [references/13-current-claim-lane.md](references/13-current-claim-lane.md),
  [../../autonomous_progress.md](../../autonomous_progress.md),
  [../../autonomous_next_steps.md](../../autonomous_next_steps.md), and
  [../../autonomous_ledger.md](../../autonomous_ledger.md).
- Start with [docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md), then load only
  the track-specific references you actually need.
