# Demo Lane Progress

Last updated: 2026-03-16

This file is the forward-facing status snapshot for
[`demo_lane_plan.md`](./demo_lane_plan.md). It should answer two questions:

- what the demo lane can honestly claim today
- what the next implementation pass should land next

## Current Position

- `demo_breadth_shadow` remains comparison-backed and still reuses
  `realistic_frontier_shadow` search semantics for the actual search work
- demo config parsing, `5m` / `10m` / `15m` config files, and doc scaffolding
  are landed
- demo runs now persist phase-aware per-step narrative artifacts:
  `reports/steps/step-XX-narrative.txt` and
  `reports/steps/step-XX-events.ndjson`
- demo runs and resumes now route the full `RuntimeConfig` into `pen-search`,
  so a real `DemoBudgetController` can enforce the shared early `90s` window,
  late-step baseline budgets, proof-close reserve fractions, and adaptive spill
  toward steps `13` to `15`
- the demo controller now also plans explicit `Scout` and
  `BreadthHarvest` slices inside the discovery budget, and the stored event
  stream records `Scout`, `BreadthHarvest`, `Materialize`, `ProofClose`, and
  `Seal` transitions from the search loop itself rather than synthesizing only
  post-step summaries
- demo steps now persist phase-level full-evaluation accounting, including the
  configured soft cap plus the `Materialize` versus `ProofClose` split and any
  proof-close-only overrun after the cap handoff

## What Landed In This Pass

- `pen-search` now has an explicit `StepPhase` surface for
  `Scout`, `BreadthHarvest`, `Materialize`, `ProofClose`, and `Seal`
- the demo budget controller now derives per-step scout and
  breadth-harvest slices on top of the existing shared early window,
  late-step baselines, proof-close reserve, and adaptive spill accounting
- the realistic-shadow search loop now emits search-side phase changes and
  budget pulses while the step is running, including a real scout throughput
  sample for prefix generation, admissibility checks, exact-bound checks, and
  full evaluations on this computer
- the demo materialization loop now enforces the configured
  `full_eval_soft_cap` as a real `Materialize` to `ProofClose` handoff instead
  of treating the cap as narrative-only metadata
- any exact candidates reopened after that handoff are now counted as
  proof-close-only overrun work and persisted on the step summary as demo phase
  accounting
- `pen-cli` now renders and persists those phase-aware events directly instead
  of depending only on post-step event synthesis, and the demo narrative header
  now reports the materialize/proof-close split, the configured cap, and the
  overrun count directly from stored step data
- targeted `pen-search` and `pen-cli` tests now cover both the original
  phase-aware narrative path and the new late-step soft-cap handoff into
  `ProofClose`

## Active Gaps

### 1. BreadthHarvest And ProofClose Budget Rules Are Still Partial

The lane now enforces the `Materialize` soft cap and tracks proof-close-only
overrun honestly, but `BreadthHarvest` still does not exit on floor success or
reserve pressure, and `ProofClose` still lacks stronger budget and
certification-state reasons beyond the new soft-cap handoff.

Next target:

- add floor-aware `BreadthHarvest` exits plus stronger `ProofClose` reserve and
  certification overrun reasons on top of the landed soft-cap handoff

### 2. Honest Funnel Counters Are Still Only Partially Present

The phase-aware narrative can now show generated surface, exact-screened
surface, and full-evaluation usage at real phase boundaries, but the
plan-aligned funnel surface is still missing the explicit demo-lane counters
like
`generated_raw_prefixes`, `hard_admissible`, `exact_bound_screened`, and
`closure_percent`.

Next target:

- add the missing funnel and closure counters to `pen-search`, step summaries,
  compare tooling, and the narrative renderer

### 3. Compare Tooling Still Lags The Stored Demo Phase Evidence

The stored narrative header now reports the materialize/proof-close split and
soft-cap overrun directly from persisted step data, but `scripts/compare_runs.py`
still does not surface closure progress, floor hits or misses, or the new demo
phase accounting.

Next target:

- extend compare tooling and the remaining summary surface so stored runs can
  report phase progress, floor hits or misses, and cap overruns without
  rereading debug text

## Immediate Next Steps

1. Turn `BreadthHarvest` into a real floor-aware exit stage that protects the
   proof-close reserve instead of only observing discovery progress.
2. Add the missing honest funnel counters and closure tracking so the demo
   narrative can report floor hits or misses from stored data instead of only
   from derived surface counts.
3. Extend compare tooling and step summaries to consume the new
   materialize/proof-close cap accounting plus the future floor evidence
   directly.
