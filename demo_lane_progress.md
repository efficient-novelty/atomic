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
- `pen-cli` now renders and persists those phase-aware events directly instead
  of depending only on post-step event synthesis
- targeted `pen-search` and `pen-cli` tests now cover the new phase-aware demo
  narrative path

## Active Gaps

### 1. The Phase Machine Is Now Explicit, But Enforcement Is Still Partial

The lane now has an explicit phase surface and emits real search-side phase
transitions, but `BreadthHarvest`, `Materialize`, and `ProofClose` still mostly
sit on top of the current realistic-shadow work instead of enforcing
phase-specific stop rules, soft-cap overruns, or proof-close-only overrun
reasons.

Next target:

- turn the observational phase machine into phase-specific control logic,
  starting with `Materialize` soft-cap enforcement and proof-close-only
  overrun accounting

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

### 3. Compare Tooling And Closure Reporting Still Lag The New Events

The stored event stream is richer now, but `scripts/compare_runs.py` and the
step narrative header still do not report closure progress, floor hits or
misses, or soft-cap overruns directly from persisted artifacts.

Next target:

- extend step summaries and compare tooling so stored runs can report phase
  progress, floor hits or misses, and cap overruns without rereading debug text

## Immediate Next Steps

1. Turn the new phase observer into real phase-specific enforcement:
   `BreadthHarvest` floor exits, `Materialize` soft caps, and `ProofClose`
   overrun rules.
2. Add the missing honest funnel counters and closure tracking so the demo
   narrative can report floor hits or misses from stored data instead of only
   from derived surface counts.
3. Extend compare tooling and step summaries to consume the new phase/floor/cap
   evidence directly.
