# Demo Lane Progress

Last updated: 2026-03-18

This file is a short operational status page for the tracked
`demo_breadth_shadow` signoff state.

## Current Baseline

- `demo_breadth_shadow` is still comparison-backed. Guarded acceptance remains
  authoritative.
- Steps `5` to `9` already carry widened live search mass through the prefix
  engine and are no longer the active problem.
- The current early reference is `runs/codex-demo-early-catalog-v3`.
- The current late reference is `runs/codex-demo-late-surface-v10`, compared
  against `runs/codex-realistic-late-baseline-v2`.

## Remaining Signoff Gaps

- None. `runs/codex-demo-early-catalog-v3` and
  `runs/codex-demo-late-surface-v10` now close every tracked
  `demo_breadth_shadow` signoff target.

## What Is Already Good Enough

- Step `1` now hits its generated-raw target: `2144`.
- Step `10` now hits its exact-screened floor: `638 >= 120`.
- Step `10` generated floor is hit: `1344 >= 500`.
- Step `11` hits both floor families: `4191 >= 800`, `1423 >= 220`.
- Step `12` now hits both late floors: generated `1330 >= 1200`,
  exact-screened `12204 >= 400`.
- Step `13` and step `15` hit both generated and exact-screened floors.
- Step `14` now hits both late floors: generated `5135 >= 3500`,
  exact-screened `3808 >= 1100`.
- `full_telescopes_evaluated` stays `1` on every late step in the current
  `10m` reference run.
- Steps `1` to `4` finish in `93 ms` total in the current early reference
  run (`46/1/1/45 ms`), so the shared `90s` early window is no longer a
  blocker.
- Step `1` now persists an explicit early audit in the current stored scout
  narrative:
  `clause_kappa=2 raw_clause_widths=18x120 raw_telescopes=2144 excluded_exact_clause_echoes=16`.
- Accepted parity still holds through step `15`
  (`matches_reference_replay x15`), and
  `scripts/compare_runs.py` reports `Comparison Signoff: ready` for
  `runs/codex-demo-late-surface-v10` against the realistic baseline.
- Late exact-screened totals on steps with an exact-screen floor now count the
  persisted exact-screen reason classes already emitted in the step summary:
  partial-prefix failure, terminal-prefix failure, incumbent dominance, and
  exact legality/connectivity rejection.
- The stored `materialize`, `proof_close`, and `seal` phase details now use
  that same late exact-screen accounting on floor-carrying steps, so the
  phase-event stream no longer lags behind the persisted demo funnel totals.

## Current Read Of The Problem

- There is no remaining tracked signoff gap in `demo_breadth_shadow`.
- The restored early exhaustive path no longer tops out at `36 x 36`. The
  current stored early reference uses a step-`1`-specific
  `18 x 120 => 2144` raw surface with `16` excluded exact-clause echo pairs,
  and it does so without threatening the shared early-window story.
- The late lane keeps every previously closed floor in the current `v10`
  reference while preserving accepted parity and
  `full_telescopes_evaluated = 1` on every late step.
- Any change that alters accepted hashes, drives `full_telescopes_evaluated`
  materially upward, or leans on silent fallback is a regression.

## Immediate Next Actions

1. Treat `runs/codex-demo-early-catalog-v3` and
   `runs/codex-demo-late-surface-v10` as the current stored signoff
   references.
2. Keep the comparison-backed guardrails, accepted parity, and late
   `full_telescopes_evaluated = 1` invariant intact on any further
   `demo_breadth_shadow` changes.
3. Reopen this lane only for new scope or if one of the stored signoff
   references regresses.

## Guardrails

- Keep `demo_breadth_shadow` comparison-backed.
- Keep guarded acceptance authoritative.
- Count only honest generated and honest exact-screened mass.
- Prefer stored run evidence over config intent when deciding whether a gap is
  closed.
