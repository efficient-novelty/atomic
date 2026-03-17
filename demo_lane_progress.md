# Demo Lane Progress

Last updated: 2026-03-17

This file is a short operational status page for the remaining
`demo_breadth_shadow` work.

## Current Baseline

- `demo_breadth_shadow` is still comparison-backed. Guarded acceptance remains
  authoritative.
- Steps `5` to `9` already carry widened live search mass through the prefix
  engine and are no longer the active problem.
- The current early reference is `runs/codex-demo-early-catalog-v2`.
- The current late reference is `runs/codex-demo-late-surface-v4`, compared
  against `runs/codex-realistic-late-baseline-v2`.

## Remaining Signoff Gaps

| Area | Current evidence | Target | Status |
| --- | --- | --- | --- |
| Step `1` generated raw | `1296` | `2144` | open |
| Step `10` exact-screened | `7` | `120+` | open |
| Step `12` generated raw | `147` | `1200+` | open |
| Step `12` exact-screened | `83` | `400+` | open |
| Step `14` generated raw | `2292` | `3500+` | open |

## What Is Already Good Enough

- Step `10` generated floor is hit: `1344 >= 500`.
- Step `11` hits both floor families: `4191 >= 800`, `253 >= 220`.
- Step `13` and step `15` hit both generated and exact-screened floors.
- Step `14` already hits its exact-screened floor: `1521 >= 1100`.
- `full_telescopes_evaluated` stays `1` on every late step in the current
  `10m` reference run.
- Steps `1` to `4` finish in `140 ms` total in the current early reference
  run (`96/1/1/42 ms`), so the shared `90s` early window is no longer a
  blocker.
- Step `1` now persists an explicit early audit in the current stored scout
  narrative: `raw_clause_widths=36x36`, `raw_telescopes=1296`.
- Accepted parity still holds through step `15`
  (`matches_reference_replay x15`).

## Current Read Of The Problem

- The remaining early gap is no longer about budget, reporting, or candidate-
  list restoration. The restored early exhaustive path now shows its current
  ceiling directly: step `1` rebuilds a `36 x 36` raw clause catalog and
  therefore honestly tops out at `1296` generated telescopes, leaving the
  missing `848` surface absent from the live enumerator rather than hidden by
  the budget controller.
- The remaining late gap is no longer step-`5` to `9` carry-through or broad
  late-family exposure. It is concentrated in step `10` exact-screen mass,
  step `12` generated and exact-screened mass, and step `14` generated mass.
- Further progress likely needs tighter exact prefix and terminal-prefix
  bounds, not just more raw catalog widening.
- Any change that alters accepted hashes, drives `full_telescopes_evaluated`
  materially upward, or leans on silent fallback is a regression.

## Immediate Next Actions

1. Raise step `1` beyond the current `36 x 36 => 1296` raw clause-catalog
   ceiling without breaking the shared early-window story.
2. Instrument step `10` to find where honest exact-screen opportunities are
   being lost before the `120` floor.
3. Re-open step `12` widening only with parity checks in place, since the last
   broader attempt either changed acceptance or collapsed generation entirely.
4. Push step `14` generated breadth upward without disturbing the already-good
   exact-screened surface.
5. Keep the default `10m` config-backed test and stored run evidence aligned
   with the latest accepted reference.

## Guardrails

- Keep `demo_breadth_shadow` comparison-backed.
- Keep guarded acceptance authoritative.
- Count only honest generated and honest exact-screened mass.
- Prefer stored run evidence over config intent when deciding whether a gap is
  closed.
