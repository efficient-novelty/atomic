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

## What Landed In This Pass

- `pen-search` now attaches a structured `NarrativeEvent` stream to each live
  `AtomicSearchStep`
- `pen-cli` now renders and persists per-step demo narrative text plus NDJSON
  event streams for `demo_breadth_shadow` runs
- the persisted narrative now surfaces current timing, generated-surface
  progress, exact-screen progress, and full-evaluation soft-cap progress using
  the existing demo budget metadata
- the new narrative remains honest about the current milestone: it summarizes
  completed step evidence after search rather than pretending the full demo
  step-phase observer already exists

## Active Gaps

### 1. The Budget Controller Still Is Not Enforced

The configs now carry the right budget metadata and the narrative renders
progress against it, but the engine still does not enforce the shared early
`90s` window, step-level reserves, or adaptive reserve spill.

Next target:

- add a real `DemoBudgetController` and wire the budget model into search-time
  control flow

### 2. The Narrative Is Post-Step, Not Yet Live Phase Observation

The new artifacts are useful and honest, but they are still derived from the
completed step summary. They do not yet emit live `Scout`, `BreadthHarvest`,
`Materialize`, `ProofClose`, and `Seal` transitions during the search itself.

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

1. Land the real demo budget controller with the shared early `90s` window and
   late adaptive reserve spill.
2. Promote the current post-step narrative stream into a live observer with
   explicit phase transitions and pulse cadence.
3. Add the missing honest funnel counters and closure tracking so the new
   narrative and comparison tooling can report floor hits or misses directly
   from stored artifacts.
