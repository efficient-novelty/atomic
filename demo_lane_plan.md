# Demo Lane Plan

Last updated: 2026-03-16

This plan now covers only the remaining work for `demo_breadth_shadow`.

Landed baseline such as config profiles, the phase machine, demo budget
control, stored demo funnel/closure evidence, narrative artifacts, compare-tool
support, and repeated discovery-side reserve retunes is no longer restated
here.

## Remaining Objective

Turn `demo_breadth_shadow` from an honest demo evidence surface into an
honestly broader live search lane that still preserves guarded acceptance
parity.

## Fixed Constraints

- `strict_canon_guarded` stays unchanged and authoritative
- `demo_breadth_shadow` stays comparison-backed only
- exact rational `rho`, `bar`, and overshoot comparisons remain authoritative
- semantic minimality and deterministic tie-breaking remain unchanged
- no semantic names or target IDs enter hot-path search or evaluation
- no stochastic acceptance or float-based ranking is introduced
- no fake breadth, fake discoveries, or silent guarded fallback
- if the lane misses a floor, budget, or certification target, it must say so
  plainly and persist partial evidence

## Remaining Success Criteria

- step 1 reports the current `2144` generated raw candidates
- steps `1` to `4` stay exhaustive or near-exhaustive inside the shared `90s`
  early window on this computer
- `demo_breadth_shadow` preserves accepted-hash parity with guarded
- the default `10m` profile completes within `600s` on this computer
- late steps show materially broader honest generated or exact-screened search
  mass while `full_telescopes_evaluated` stays moderate
- every prune is labeled by type and exact-screen reasons are separated where
  applicable

## Remaining Workstreams

### 1. Early Breadth Recovery

- restore step-1 generated breadth to `2144`
- keep full candidate-list generation on steps `1` to `4` wherever that still
  fits the shared early window
- validate the early-window behavior from stored run evidence rather than from
  config alone

### 2. Demo-Specific Structural Scheduling

- add a deterministic bucket key and per-bucket stats
- prioritize exact survivability, obligation closure, bridge/completion
  potential, underexplored buckets, and low redundancy
- keep all heuristics structural and runtime-local so they influence ordering
  only, never acceptance

### 3. Real Search-Surface Widening

- steps `5` to `9`: widen `kappa`, support-form, and bridge-head variety while
  keeping near-exhaustive behavior where cheap
- steps `10` to `12`: widen family unions, reference patterns, nested `Pi` and
  `Sigma`, bridge heads, and reanchor variants
- steps `13` to `15`: widen operator, Hilbert, and temporal mixtures, mixed
  shells, historical reanchors, clause unions, and positional filters
- strengthen exact prefix bounds so widening increases honest search mass
  without exploding full terminal work

### 4. Closure-Aware Replanning

- build on the landed scout and `BreadthHarvest` reserve retunes
- let live closure progress reshape the `Materialize` versus `ProofClose`
  split, not only retained-group ordering and handoff timing
- keep reserve usage, closure progress, and replanning reasons explicit in
  stored artifacts

### 5. Reporting Closeout

- label prune classes as sound, quotient/dedupe, or heuristic shaping
- persist explicit exact-screen reason codes
- keep `--narrative` output inside the intended line budgets
- finish the remaining mandatory live-event and pulse-rate-limiting story

## Primary Touch Points

- `crates/pen-search/src/engine.rs`
- `crates/pen-search/src/scheduler.rs`
- `crates/pen-search/src/diversify.rs`
- `crates/pen-search/src/frontier.rs`
- `crates/pen-cli/src/narrative.rs`
- `crates/pen-cli/src/report.rs`
- `scripts/compare_runs.py`
