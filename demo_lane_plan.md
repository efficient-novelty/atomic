# Demo Lane Plan

Last updated: 2026-03-18

This plan tracks only the remaining work needed to sign off
`demo_breadth_shadow`.

## Goal

Close the remaining early demo-lane gap without losing accepted-hash parity,
without inflating full telescope evaluations, and without weakening the
comparison-backed honesty boundary.

## Working Baseline

- Early reference: `runs/codex-demo-early-catalog-v2`
- Late reference: `runs/codex-demo-late-surface-v9`
- Guarded comparison baseline: `runs/codex-realistic-late-baseline-v2`
- Current known open deltas:
  - step `1` generated raw: `1296 -> 2144`
- Fresh closures in `v9`:
  - step `10` exact-screened: `638 >= 120`
  - step `12` generated raw: `1330 >= 1200`
  - step `12` exact-screened: `12204 >= 400`
  - step `14` generated: `5135 >= 3500`
  - late phase-event detail alignment: `materialize/proof_close/seal` now use
    the stored exact-screen totals on floor-carrying steps

## Execution Order

1. Recover step-`1` generated breadth.
2. Re-run the default `10m` signoff evidence and refresh docs/tests once the
   step-`1` change lands.

## Workstreams

### 1. Step-1 Recovery

- Audit which families or prefixes still fail to appear between the current
  `1296` and the target `2144`.
- Preserve full early candidate-list generation through step `4`.
- Re-validate the shared early-window story from stored run evidence, not just
  from config settings.

Done when:

- step `1` reports `2144` generated raw candidates in a stored run
- steps `1` to `4` still fit comfortably inside the shared `90s` early window

### 2. Signoff Refresh

- Keep the landed late floors from regressing while the remaining step-`1`
  work moves.
- Preserve accepted parity, `full_telescopes_evaluated = 1` on the late
  steps, and the current exact-screen reason / phase-detail accounting.
- Prefer changes that survive live prefix search over raw-catalog-only
  widening.

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
