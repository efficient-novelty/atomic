# Demo Lane Progress

Last updated: 2026-03-18

This file is a short operational status page for the remaining
`demo_breadth_shadow` work.

## Current Baseline

- `demo_breadth_shadow` is still comparison-backed. Guarded acceptance remains
  authoritative.
- Steps `5` to `9` already carry widened live search mass through the prefix
  engine and are no longer the active problem.
- The current early reference is `runs/codex-demo-early-catalog-v2`.
- The current late reference is `runs/codex-demo-late-surface-v8`, compared
  against `runs/codex-realistic-late-baseline-v2`.

## Remaining Signoff Gaps

| Area | Current evidence | Target | Status |
| --- | --- | --- | --- |
| Step `1` generated raw | `1296` | `2144` | open |
| Step `12` generated raw | `995` | `1200+` | open |

## What Is Already Good Enough

- Step `10` now hits its exact-screened floor: `638 >= 120`.
- Step `10` generated floor is hit: `1344 >= 500`.
- Step `11` hits both floor families: `4191 >= 800`, `253 >= 220`.
- Step `12` now hits its exact-screened floor: `8629 >= 400`.
- Step `13` and step `15` hit both generated and exact-screened floors.
- Step `14` now hits both late floors: generated `5135 >= 3500`,
  exact-screened `3808 >= 1100`.
- `full_telescopes_evaluated` stays `1` on every late step in the current
  `10m` reference run.
- Steps `1` to `4` finish in `140 ms` total in the current early reference
  run (`96/1/1/42 ms`), so the shared `90s` early window is no longer a
  blocker.
- Step `1` now persists an explicit early audit in the current stored scout
  narrative: `raw_clause_widths=36x36`, `raw_telescopes=1296`.
- Accepted parity still holds through step `15`
  (`matches_reference_replay x15`), and
  `scripts/compare_runs.py` reports `Comparison Signoff: ready` for the
  current late reference against the realistic baseline.
- Late exact-screened totals on steps with an exact-screen floor now count the
  persisted exact-screen reason classes already emitted in the step summary:
  partial-prefix failure, terminal-prefix failure, incumbent dominance, and
  exact legality/connectivity rejection.
- The stored `materialize`, `proof_close`, and `seal` phase details now use
  that same late exact-screen accounting on floor-carrying steps, so the
  phase-event stream no longer lags behind the persisted demo funnel totals.

## Current Read Of The Problem

- The remaining early gap is no longer about budget, reporting, or candidate-
  list restoration. The restored early exhaustive path now shows its current
  ceiling directly: step `1` rebuilds a `36 x 36` raw clause catalog and
  therefore honestly tops out at `1296` generated telescopes, leaving the
  missing `848` surface absent from the live enumerator rather than hidden by
  the budget controller.
- The remaining late gap is no longer step-`5` to `9` carry-through, broad
  late-family exposure, or step-`10` exact-screen mass. The current `v8`
  reference closes step `10` by promoting the already persisted exact-screen
  reason surface into the late exact-screened total, leaving only the last
  `205` raw generated candidates on step `12` still open.
- Further progress likely needs a more selective step-`12` generated widening,
  not another broad late-surface explosion.
- Any change that alters accepted hashes, drives `full_telescopes_evaluated`
  materially upward, or leans on silent fallback is a regression.

## Immediate Next Actions

1. Raise step `1` beyond the current `36 x 36 => 1296` raw clause-catalog
   ceiling without breaking the shared early-window story.
2. Push step `12` generated from `995` to `1200+` without giving back the
   newly landed `8629` exact-screened surface.
3. Keep the default `10m` config-backed test and stored run evidence aligned
   with the current `v8` reference.

## Guardrails

- Keep `demo_breadth_shadow` comparison-backed.
- Keep guarded acceptance authoritative.
- Count only honest generated and honest exact-screened mass.
- Prefer stored run evidence over config intent when deciding whether a gap is
  closed.
