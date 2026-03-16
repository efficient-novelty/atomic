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
- demo runs now also persist first-pass per-step narrative artifacts:
  `reports/steps/step-XX-narrative.txt` and
  `reports/steps/step-XX-events.ndjson`
- demo runs and resumes now route the full `RuntimeConfig` into `pen-search`,
  so a real `DemoBudgetController` can enforce the shared early `90s` window,
  late-step baseline budgets, proof-close reserve fractions, and adaptive spill
  toward steps `13` to `15`

## What Landed In This Pass

- `pen-search` now has a real `DemoBudgetController` seeded from runtime config
  and prior completed-step timing when resuming
- the controller applies the shared early `90s` budget across steps `1` to `4`
  instead of treating that metadata as narrative-only
- late steps now receive explicit baseline budgets plus deterministic adaptive
  spill allocation toward steps `13` to `15`, with per-step proof-close reserve
  slices derived from the config fraction
- the realistic-shadow discovery loop now stops widening once the discovery
  slice is exhausted and some real candidate surface has already been
  materialized, keeping the stop condition honest instead of post-hoc
- `pen-cli` run and resume flows now preserve enough prior timing context for
  the demo controller to keep budget accounting consistent across checkpoint
  continuation

## Active Gaps

### 1. The Budget Controller Is Coarse, Not Yet A Full Phase Machine

The engine now enforces real demo budgets during discovery, but it still does
not run an explicit `Scout` / `BreadthHarvest` / `Materialize` / `ProofClose` /
`Seal` state machine. The current reserve split is a coarse search-time cutoff,
not a true phase-aware controller with proof-close-only overrun reasons.

Next target:

- promote the coarse controller into an explicit step-phase machine with
  dedicated proof-close accounting and phase-specific narrative pulses

### 2. The Narrative Is Still Post-Step, Not Yet Live Phase Observation

The new artifacts are useful and honest, but they are still derived from the
completed step summary. They still do not emit live `Scout`,
`BreadthHarvest`, `Materialize`, `ProofClose`, and `Seal` transitions during
the search itself.

Next target:

- replace the current post-step event synthesis with a true search-side
  observer that emits live phase changes and budget pulses

### 3. Honest Funnel Counters Are Still Only Partially Present

The current narrative can show generated surface, exact-screened surface, and
full evaluation usage from existing counters, but the plan-aligned funnel
surface is still missing the explicit demo-lane counters like
`generated_raw_prefixes`, `hard_admissible`, `exact_bound_screened`, and
`closure_percent`.

Next target:

- add the missing funnel and closure counters to `pen-search`, step summaries,
  compare tooling, and the narrative renderer

## Immediate Next Steps

1. Turn the new coarse demo budget controller into a real step-phase machine
   with explicit `Scout`, `BreadthHarvest`, `Materialize`, `ProofClose`, and
   `Seal` transitions.
2. Promote the current post-step narrative stream into a live observer with
   phase changes, budget pulses, and proof-close progress updates emitted from
   the search loop itself.
3. Add the missing honest funnel counters and closure tracking so the demo
   narrative and comparison tooling can report floor hits or misses directly
   from stored artifacts.
