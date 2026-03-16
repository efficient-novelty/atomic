# Demo Lane Progress

Last updated: 2026-03-16

This file tracks only the open status of `demo_breadth_shadow`.

Landed baseline is no longer repeated here in detail. The repo already has the
demo config profiles, phase machine, budget controller, persisted
narratives/events, demo funnel and closure stats, proof-close reserve
accounting, compare-tool demo evidence, and repeated discovery-side reserve
retuning during `BreadthHarvest`.

## Current Status

- `demo_breadth_shadow` remains comparison-backed and still reuses
  `realistic_frontier_shadow` for the underlying search semantics.
- The lane now reports its misses honestly instead of hiding them behind
  generic breadth counts or debug text.
- The lane now has a deterministic demo bucket scheduler keyed only by
  structural/runtime-local evidence, with stored per-bucket generated,
  admissible, exact-screened, pruned, fully scored, and best-overshoot stats.
- CLI debug output, `--narrative`, and `scripts/compare_runs.py` now surface
  compact bucket summaries so the ordering story is visible in stored evidence.
- The main remaining problem is no longer "missing demo evidence." The main
  problem is that the live search surface is still not broad enough in the
  places the plan cares about.

## What Still Blocks Signoff

### 1. Early Breadth Is Still Short

- step 1 still misses the explicit `2144` generated floor on the current path
- steps 1 to 4 are not yet shown to stay exhaustive or near-exhaustive inside
  the shared `90s` window on this computer
- early full candidate-list generation is still not restored wherever it is
  supposed to remain affordable

### 2. Real Widening Is Still Missing

- steps `5` to `15` still inherit too much of the current realistic-shadow
  surface
- configured late-step generated and exact-screened floors are now stored and
  reported, but they are not yet being hit consistently
- the landed demo-specific structural bucket scheduler still needs broader real
  widening underneath it so late buckets actually reach their honest floors

### 3. The Within-Step Controller Still Needs Closure-Aware Replanning

- discovery-side reserve retunes now repeat during `BreadthHarvest`
- proof-close still mostly changes ordering and handoff timing rather than
  replanning the earlier widening and materialize split from live closure
  state

### 4. Reporting Closeout Is Still Incomplete

- prune labeling is not yet fully separated into sound, quotient/dedupe, and
  heuristic shaping
- exact-screen reason codes are not yet fully persisted and surfaced
- `pen-cli` now supports `--narrative` for appending stored per-step demo
  narrative output, including explicit time and closure bars
- `scripts/compare_runs.py` now flags missing step narrative and event
  artifacts explicitly for demo lanes
- the remaining narrative/tooling gap is line-budget discipline plus the still-
  missing mandatory live events and pulse-rate closeout

## Next Priorities

1. Restore the honest early breadth story, starting with step 1 at `2144` and
   the shared early `90s` window.
2. Use the landed demo-specific structural scheduling to widen the real search
   surface on steps `10` to `15`.
3. Extend the current reserve retunes into stronger closure-aware replanning
   across `Materialize` and `ProofClose`.
4. Finish prune taxonomy, exact-screen reason reporting, and remaining
   narrative/tooling gaps.
