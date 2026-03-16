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
- demo phase accounting now also persists
  `breadth_harvest_exit_reason`, `proof_close_entry_reason`, and
  `proof_close_overrun_reason`, so stored summaries and narratives can
  distinguish floor hits, reserve protection, certification sweeps, and
  soft-cap handoffs directly

## What Landed In This Pass

- `BreadthHarvest` now exits early once a configured generated or exact-screened
  late-step floor is hit after scout sampling, instead of only stopping when
  the raw discovery deadline expires
- demo discovery now also records when widening stops to protect the reserved
  `ProofClose` slice, so late-step breadth exits are no longer a generic
  "closed discovery" story
- `pen-search` step summaries now persist explicit
  `breadth_harvest_exit_reason`, `proof_close_entry_reason`, and
  `proof_close_overrun_reason` values
- `ProofClose` reason tracking now distinguishes reserve-protected handoffs,
  breadth-floor handoffs, certification sweeps, and soft-cap overruns on top
  of the previously landed full-eval split
- `pen-cli` narrative rendering now surfaces those persisted reason codes in the
  demo phase header, and live search-side pulses now say whether breadth
  harvest stopped on a generated floor, an exact-screened floor, or reserve
  protection
- targeted `pen-search` tests now cover generated-floor exits,
  reserve-protection exits, and soft-cap overrun reasoning in addition to the
  existing phase-event and narrative coverage

## Active Gaps

### 1. Honest Funnel Counters Are Still Only Partially Present

The lane can now explain why `BreadthHarvest` stopped and why `ProofClose`
started or overran, but the plan-aligned funnel surface is still missing the
explicit demo counters like `generated_raw_prefixes`, `hard_admissible`,
`exact_bound_screened`, and `closure_percent`.

Next target:

- add the missing funnel and closure counters to `pen-search`, step summaries,
  compare tooling, and the narrative renderer

### 2. Compare Tooling Still Lags The Stored Demo Phase Evidence

The stored narrative header now reports the materialize/proof-close split and
soft-cap overrun directly from persisted step data, and step summaries now also
carry explicit breadth/proof-close reason codes, but
`scripts/compare_runs.py` still does not surface closure progress, floor hits
or misses, or the new demo phase accounting.

Next target:

- extend compare tooling and the remaining summary surface so stored runs can
  report phase progress, floor hits or misses, breadth/proof-close reason codes,
  and cap overruns without rereading debug text

### 3. ProofClose Reserve Usage Is Still Descriptive Rather Than Governed

The lane now records why `ProofClose` started or overran, but it still does not
expose a stronger exact closure-progress or reserve-usage surface beyond those
reason codes and the existing full-eval split.

Next target:

- tie the reserved proof-close slice to explicit closure progress and reserve
  usage evidence, not just handoff reasons

## Immediate Next Steps

1. Add the missing honest funnel counters and closure tracking so the demo
   narrative can report floor hits or misses from stored data instead of only
   from derived surface counts.
2. Extend compare tooling and step summaries to consume the new
   breadth/proof-close reason codes plus the future funnel and floor evidence
   directly.
3. Tighten proof-close reserve accounting from descriptive reason codes into a
   clearer exact closure-progress and reserve-usage surface.
