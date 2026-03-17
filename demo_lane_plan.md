# Demo Lane Plan

Last updated: 2026-03-17

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

### 2. Late Search-Surface Widening

- build on the now-stored step-`5` to `9` live carry-through rather than
  re-opening raw-catalog-only widening
- keep pushing steps `10` to `12` toward their configured generated and
  exact-screened floors
- keep pushing steps `13` to `15` toward their configured generated and
  exact-screened floors, especially step `13` and step `15`
- preserve accepted parity, moderate `full_telescopes_evaluated`, the landed
  bucket scheduler, and the landed closure-aware proof-close ordering while
  widening further

### 3. Exact-Bound Tightening

- strengthen exact prefix bounds so widening increases honest search mass
  without exploding full terminal work
- keep exact-screen reasons, prune classes, narrative artifacts, and mandatory
  live-event coverage stable while the late bounds move

## Primary Touch Points

- `crates/pen-search/src/engine.rs`
- `crates/pen-search/src/scheduler.rs`
- `crates/pen-search/src/diversify.rs`
- `crates/pen-search/src/frontier.rs`
- `crates/pen-cli/src/narrative.rs`
- `crates/pen-cli/src/report.rs`
- `scripts/compare_runs.py`
