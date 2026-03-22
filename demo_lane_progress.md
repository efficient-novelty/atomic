# Demo Lane Progress

Last updated: 2026-03-22

This file tracks only the live operational state of `demo_breadth_shadow`.
Historical rollout detail belongs in `demo_lane_plan.md`,
`demo_lane_checklist.md`, and the bundled demo-lane reference docs.

## Current Status

- Tracked `demo_breadth_shadow` signoff work is closed.
- `demo_breadth_shadow` remains comparison-backed. Guarded / realistic
  acceptance remains authoritative.
- The current stored signoff references are:
  - early: `runs/codex-demo-early-catalog-v3`
  - late: `runs/codex-demo-late-surface-v10`
  - guarded comparison baseline: `runs/codex-realistic-late-baseline-v2`

## Why The Lane Stays Closed

- The early reference already closes the last early gap:
  - step `1` generated raw = `2144`
  - steps `1` to `4` complete in `93 ms` total
- The late reference already closes the tracked late signoff set:
  - accepted parity holds through step `15`
  - late floors are closed from stored evidence
  - `full_telescopes_evaluated = 1` on every late step
  - `scripts/compare_runs.py` reports `Comparison Signoff: ready`

## Current Read

- The demo lane is not the active repo bottleneck. The live issue is the
  claim-lane runtime/certification path, not `demo_breadth_shadow`.
- Reopen this lane only for new scope or if one of the stored signoff
  references regresses.
- Treat any change that alters accepted hashes, weakens comparison-backed
  accounting, materially raises `full_telescopes_evaluated`, or leans on
  silent fallback as a regression first, not as new progress.

## Immediate Next Action

1. Keep using `runs/codex-demo-early-catalog-v3` and
   `runs/codex-demo-late-surface-v10` as the stored demo-lane references.
2. Preserve accepted parity, honest generated / exact-screened accounting, and
   late `full_telescopes_evaluated = 1` on any future demo-lane edits.
3. Leave this lane closed unless the stored references regress or the scope
   changes.
