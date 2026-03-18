# Demo Lane Checklist

Last updated: 2026-03-18

This checklist covers the tracked tasks for `demo_breadth_shadow` signoff.
The current checklist is fully closed.

## Current Open Numbers

- None. The current stored signoff references are
  `runs/codex-demo-early-catalog-v3` and
  `runs/codex-demo-late-surface-v10`.

## 1. Step-1 Recovery

- [x] Explain where the missing `848` generated raw surface went.
- [x] Raise step `1` generated raw from `1296` to `2144`.
- [x] Reconfirm that steps `1` to `4` still fit honestly inside the shared
      `90s` early window after the step-`1` change.

Current audit evidence:
`runs/codex-demo-early-catalog-v3` now shows step `1` generated raw `2144`,
with scout detail
`clause_kappa=2 raw_clause_widths=18x120 raw_telescopes=2144 excluded_exact_clause_echoes=16`,
and steps `1` to `4` complete in `93 ms` total (`46/1/1/45 ms`).

Done when:

- a stored early run shows step `1 = 2144`
- the same run keeps steps `1` to `4` comfortably inside the early window

## 2. Step-10 Exact-Screened Floor

- [x] Raise step `10` exact-screened surface from `7` to `120+`.
- [x] Keep the already-landed step-`10` generated floor hit.
- [x] Preserve accepted parity and keep `full_telescopes_evaluated` moderate.

Current evidence:
`runs/codex-demo-late-surface-v10` now shows step `10` generated `1344`,
exact-screened `638`, `full_telescopes_evaluated = 1`, accepted parity
through step `15`, and phase-detail exact-screened values of
`637 -> 638` across `materialize -> proof_close/seal`.

Done when:

- a stored default `10m` run shows step `10` generated `>= 500`
- the same run shows step `10` exact-screened `>= 120`
- accepted parity still holds

## 3. Step-12 Floor Closure

- [x] Raise step `12` generated raw from `995` to `1200+`.
- [x] Raise step `12` exact-screened surface from `83` to `400+`.
- [x] Preserve the reference acceptance while doing it.

Current evidence:
`runs/codex-demo-late-surface-v10` now shows step `12` generated `1330`,
exact-screened `12204`, `full_telescopes_evaluated = 1`, accepted parity
through step `15`, and phase-detail exact-screened values of
`12203 -> 12204` across `materialize -> proof_close/seal`.

Done when:

- a stored default `10m` run shows step `12` generated `>= 1200`
- the same run shows step `12` exact-screened `>= 400`
- accepted parity still holds

## 4. Step-14 Generated Floor

- [x] Raise step `14` generated raw from `2292` to `3500+`.
- [x] Keep step `14` exact-screened at `1100+`.
- [x] Preserve accepted parity and keep `full_telescopes_evaluated` moderate.

Current evidence:
`runs/codex-demo-late-surface-v10` now shows step `14` generated `5135`,
exact-screened `3808`, `full_telescopes_evaluated = 1`, and accepted parity
through step `15`.

Done when:

- a stored default `10m` run shows step `14` generated `>= 3500`
- the same run still shows step `14` exact-screened `>= 1100`
- accepted parity still holds

## 5. Exact-Bound Tightening

- [x] Add or retune exact prefix / terminal-prefix bounds that convert widened
      honest breadth into honest exact-screened mass.
- [x] Preserve exact-screen reasons, prune classes, and narrative/event
      artifacts while the bounds move.
- [x] Avoid solving the open late floors by evaluating many more full
      telescopes.

Current evidence:
`runs/codex-demo-late-surface-v10` keeps the closed late floors on steps
`10`, `12`, `14`, and `15`, carries the stored exact-screen reason classes
through summaries and phase details, and still holds
`full_telescopes_evaluated = 1` on every late step.

Done when:

- the remaining late floors close without a large rise in
  `full_telescopes_evaluated`

## 6. Final Signoff Package

- [x] The default `10m` run finishes within `600s`.
- [x] Compare output still reports accepted parity through step `15`.
- [x] Config-backed tests cover the currently closed floors.
- [x] `demo_lane_progress.md`, `demo_lane_plan.md`, and
      `demo_lane_checklist.md` reflect the latest stored evidence.

Current evidence:
`runs/codex-demo-early-catalog-v3` closes the final early gap,
`runs/codex-demo-late-surface-v10` preserves accepted parity through step
`15` (`matches_reference_replay x15`), `scripts/compare_runs.py` reports
`Comparison Signoff: ready`, and the config-backed floor tests now match the
stored early and late references.
