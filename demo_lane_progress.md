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
- demo step summaries now also persist a first plan-aligned stored funnel
  surface via `demo_funnel` plus a first closure surface via `demo_closure`,
  covering `generated_raw_prefixes`, `canonical_prefix_signatures`,
  `hard_admissible`, `exact_bound_screened`, `exact_bound_pruned`,
  `bar_clearers`, `semantically_minimal_clearers`, `winner_overshoot`,
  `frontier_total_seen`, `frontier_certified_nonwinning`, and
  `closure_percent`
- stored demo phase data now also carries the configured
  `generated_floor` and `exact_screened_floor`, so floor targets travel with
  the step summary rather than only living in config files
- `pen-cli` narrative rendering, debug reporting, and
  `scripts/compare_runs.py` now consume that stored demo phase, funnel, and
  closure evidence directly instead of inferring the story from debug text
- the newly stored surface makes an important current gap explicit: the demo
  lane still misses the step-1 `2144` generated floor on the current search
  path, and the compare tool now reports that miss honestly instead of hiding
  it behind generic breadth counts

## What Landed In This Pass

- `pen-search` now builds and persists the first explicit demo-lane funnel and
  closure structs on every step summary, instead of leaving that surface
  implicit inside generic search counters
- demo phase summaries now persist `generated_floor` and
  `exact_screened_floor` alongside the already-landed proof-close reason codes
- `pen-cli` narrative output now renders generated, exact-screened, and
  full-eval progress from those stored demo counters, adds an explicit closure
  line, and prints a compact stored demo-funnel line with winner overshoot
- debug report rendering now also exposes the stored demo funnel and closure
  counters plus the persisted floor and proof-close reason fields
- `scripts/compare_runs.py` now emits lane-level demo phase evidence, including
  floor hits or misses, closure percent, soft-cap status, breadth/proof-close
  reason codes, and latest-step demo funnel summaries in both text and JSON
- targeted tests now cover the new narrative rendering, compare-tool demo
  evidence surface, and the compare-summary schema bump

## Active Gaps

### 1. Honest Counters Now Exist, But The Breadth Story Still Misses Key Floors

The lane now persists the planned funnel and closure counters, and the compare
tool can surface floor hits or misses directly from stored data. The remaining
problem is no longer "missing counters"; it is that the current search path
still misses some of the intended breadth targets, most visibly the explicit
step-1 `2144` generated floor.

Next target:

- use the new stored evidence to drive actual early-step and late-step
  widening work until the demo lane can satisfy more of the planned breadth
  floors honestly

### 2. ProofClose Closure Evidence Is Still Post-Hoc Rather Than Governing Runtime

The step summary now exposes `frontier_total_seen`,
`frontier_certified_nonwinning`, and `closure_percent`, but those numbers are
still a post-step evidence surface. They do not yet govern how much of the
reserved `ProofClose` slice is spent or when closure is considered sufficient
mid-step.

Next target:

- tie the reserved proof-close slice to live closure progress and explicit
  reserve-usage accounting, not just stored after-the-fact counters

### 3. Demo Widening Is Still Mostly A Reporting Surface, Not Yet A Broader Search Surface

The repo can now report the demo lane more honestly, but the underlying search
semantics are still largely inherited from `realistic_frontier_shadow`. The
larger unfinished task remains turning the reported generated and exact-screened
surfaces into genuinely broader live search mass, especially on later steps.

Next target:

- widen the actual demo search schedule and exact-screen pipeline so late-step
  floors improve because the lane is doing more real search work, not because
  the reporting surface got richer

## Immediate Next Steps

1. Use the new stored funnel and floor evidence to close the explicit step-1
   and early-step breadth gap, rather than leaving the compare tool to report a
   permanent miss.
2. Turn the new closure counters into live proof-close governance and reserve
   usage accounting so the reserved slice is explained and enforced mid-step.
3. Keep widening the actual demo search surface, especially on steps `10` to
   `15`, so the newly surfaced generated and exact-screened counters grow for
   real.
