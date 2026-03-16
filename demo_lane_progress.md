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
- demo phase data now also persists live proof-close reserve and closure
  accounting via `proof_close_reserved_millis`,
  `proof_close_elapsed_millis`, `proof_close_remaining_millis`,
  `proof_close_reserve_overrun_millis`, `proof_close_reserve_exhausted`,
  `proof_close_frontier_total_groups`, `proof_close_frontier_groups_closed`,
  `proof_close_frontier_groups_remaining`, and
  `proof_close_closure_percent`
- late-step demo planning now also feeds stored floor misses plus proof-close
  reserve pressure or slack back into the standard `25` to `40` percent
  reserve profiles, frontloading more spill and discovery toward the next
  late step after misses while preserving explicit out-of-band reserve override
  configs such as `0.00` and `1.00` literally
- the demo controller now also uses current-step scout throughput to retune
  discovery versus proof-close reserve inside the same step, borrowing up to
  half of the reserved certification slice when projected breadth floors are
  still out of reach and surfacing that rebalance in the stored narrative
- demo materialize and proof-close now also spend the retained exact surface in
  a reserve-aware deterministic order: with healthy reserve they prioritize
  groups that can tighten the incumbent earliest, and under tight reserve they
  prioritize prune-ready or low-cost groups that can close the remaining
  frontier faster
- demo materialize can now also yield into `ProofClose` with an explicit
  `materialize_reserve_handoff` reason once an incumbent exists and the
  remaining exact surface is already in closure-first reserve-pressure mode,
  instead of waiting only for the soft cap or the end of materialize
- candidates inside each retained prefix group are now also processed in exact
  accept-rank order before stable structural tiebreaks, so later dominated
  candidates can be skipped once an earlier exact incumbent wins cleanly
- `pen-cli` narrative rendering, debug reporting, and
  `scripts/compare_runs.py` now consume that stored demo phase, funnel, and
  closure evidence directly instead of inferring the story from debug text
- demo proof-close narratives now also emit live closure milestone and
  reserve-exhaustion pulses from the search loop itself, so the reserved
  certification slice is no longer only a final report surface
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
- `pen-search` now also tracks proof-close reserve usage and remaining-group
  closure live while exact certification runs, instead of leaving proof-close
  reserve consumption implicit
- `pen-search` now uses that live reserve signal to reorder retained
  certification groups deterministically, switching from incumbent-improvement
  order to faster closure/prune order when the remaining reserved slice gets
  tight relative to the pending exact surface
- the demo budget controller now also uses stored late-step floor misses plus
  reserve exhaustion or slack to retune later spill and effective proof-close
  reserve sizing inside the standard `25` to `40` percent profiles, instead of
  treating every late step as the same fixed spill split
- `pen-search` now also performs a first scout-driven within-step rebalance of
  discovery versus proof-close reserve, borrowing from the reserved
  certification slice when the sampled generated or exact-screened throughput
  still projects a breadth-floor miss under the original split
- `pen-search` can now also enter `ProofClose` from `Materialize` with the new
  `materialize_reserve_handoff` reason when a tightened incumbent already
  exists and the pending exact surface has flipped into closure-first reserve
  pressure, rather than waiting only for a soft cap or the end of materialize
- retained-group candidate order is now exact accept-rank first with stable
  structural tiebreaks, so materialize and proof-close tighten incumbents
  earlier before spending later exact evaluations
- demo proof-close narratives now emit budget pulses when closure crosses the
  stored frontier milestones and when certification overruns the reserved slice
- `pen-cli` narrative output now renders generated, exact-screened, and
  full-eval progress from those stored demo counters, adds explicit closure and
  proof-close reserve lines, and prints a compact stored demo-funnel line with
  winner overshoot
- debug report rendering now also exposes the stored demo funnel and closure
  counters plus the persisted floor, proof-close reason, reserve, and closure
  progress fields
- `scripts/compare_runs.py` now emits lane-level demo phase evidence, including
  floor hits or misses, closure percent, soft-cap status, breadth/proof-close
  reason codes, proof-close reserve usage, proof-close closure progress, and
  latest-step demo funnel summaries in both text and JSON
- targeted tests now also cover adaptive spill and reserve retuning, the new
  materialize-to-proof-close handoff trigger, zero-discovery reserve
  protection, the new narrative rendering, compare-tool demo evidence surface,
  reserve-exhaustion accounting, and the compare-summary schema bump

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

### 2. Current-Step Retuning Exists Now, But The Feedback Loop Is Still Shallow

The lane no longer treats current-step feedback as report-only evidence:
late-step spill still reacts to stored floor misses and reserve pressure or
slack, scout throughput can now borrow time from proof-close reserve inside the
same step when projected breadth floors are still underwater, retained groups
and within-group candidates reorder under live reserve pressure so tighter
incumbents and cheap exact prunes happen earlier, and materialize can hand off
early once the pending exact surface has already become closure-first. The
remaining gap is that this is still a first scout-time rebalance plus later
proof-close ordering and one reserve-pressure handoff, not a richer repeated
within-step loop driven by live closure progress and evolving discovery
throughput.

Next target:

- let live discovery pressure and proof-close closure keep retuning the
  discovery, materialize, and certification split repeatedly throughout the
  step, not just once from scout throughput and later from a single
  reserve-pressure handoff

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
2. Extend the new scout-driven within-step rebalance into a richer live
   controller so discovery pressure and proof-close closure keep moving budget
   while the step is still running.
3. Keep widening the actual demo search surface, especially on steps `10` to
   `15`, so the newly surfaced generated and exact-screened counters grow for
   real.
