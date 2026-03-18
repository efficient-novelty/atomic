# Demo Lane Plan

Last updated: 2026-03-18

This plan tracked the remaining work needed to sign off
`demo_breadth_shadow`. The tracked signoff work is now complete.

## Goal

Hold the current signed-off demo-lane state without losing accepted-hash
parity, without inflating full telescope evaluations, and without weakening
the comparison-backed honesty boundary.

## Working Baseline

- Early reference: `runs/codex-demo-early-catalog-v3`
- Late reference: `runs/codex-demo-late-surface-v10`
- Guarded comparison baseline: `runs/codex-realistic-late-baseline-v2`
- Current known open deltas:
  - none
- Current stored closures:
  - step `1` generated raw: `2144`
  - steps `1` to `4`: `93 ms` total (`46/1/1/45 ms`)
  - step `10` exact-screened: `638 >= 120`
  - step `12` generated raw: `1330 >= 1200`
  - step `12` exact-screened: `12204 >= 400`
  - step `14` generated: `5135 >= 3500`
  - late phase-event detail alignment: `materialize/proof_close/seal` now use
    the stored exact-screen totals on floor-carrying steps

## Execution Order

1. Preserve the stored step-`1` recovery and early-window evidence.
2. Preserve the default `10m` signoff evidence, accepted parity, and
   comparison-backed accounting on later demo-lane changes.

## Workstreams

### 1. Step-1 Recovery

- Completed in `runs/codex-demo-early-catalog-v3`.
- Step `1` now reports `2144` generated raw candidates.
- The stored scout audit records
  `clause_kappa=2 raw_clause_widths=18x120 raw_telescopes=2144 excluded_exact_clause_echoes=16`.
- Steps `1` to `4` still fit comfortably inside the shared early window.

Done when:

- step `1` reports `2144` generated raw candidates in a stored run
- steps `1` to `4` still fit comfortably inside the shared `90s` early window

### 2. Signoff Refresh

- Completed in `runs/codex-demo-late-surface-v10`.
- The default `10m` profile preserves accepted parity through step `15`.
- `full_telescopes_evaluated = 1` still holds on every late step.
- Exact-screen reason accounting and `materialize/proof_close/seal` phase
  detail alignment remain intact.

Done when:

- step `1` reports `2144` generated raw candidates in a stored run
- the default `10m` profile still preserves accepted parity through step `15`
- the lane still reports exact-screen reasons and prune classes cleanly

## What Not To Reopen

- step-`5` to `9` live carry-through
- generic narrative/tooling cleanup
- raw-catalog-only widening that does not survive the live prefix engine
- changes that improve breadth only by evaluating many more full telescopes

## Likely Touch Points

- `crates/pen-search/src/engine.rs`
- `crates/pen-search/src/prefix_memo.rs`
- `crates/pen-search/src/scheduler.rs`
- `crates/pen-search/src/diversify.rs`
- `crates/pen-search/src/frontier.rs`
