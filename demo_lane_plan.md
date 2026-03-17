# Demo Lane Plan

Last updated: 2026-03-17

This plan tracks only the remaining work needed to sign off
`demo_breadth_shadow`.

## Goal

Close the remaining early and late demo-lane gaps without losing accepted-hash
parity, without inflating full telescope evaluations, and without weakening the
comparison-backed honesty boundary.

## Working Baseline

- Early reference: `runs/codex-demo-early-catalog-v2`
- Late reference: `runs/codex-demo-late-surface-v5`
- Guarded comparison baseline: `runs/codex-realistic-late-baseline-v2`
- Current known open deltas:
  - step `1` generated raw: `1296 -> 2144`
  - step `10` exact-screened: `7 -> 120+`
  - remaining late generated gap: step `12` `995 -> 1200+`
- Fresh closures in `v5`:
  - step `12` exact-screened: `749 >= 400`
  - step `14` generated: `5135 >= 3500`

## Execution Order

1. Recover step-`1` generated breadth.
2. Raise step-`10` exact-screened mass.
3. Finish the remaining step-`12` generated breadth.
4. Re-run the default `10m` signoff evidence and refresh docs/tests.

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

### 2. Late Floor Closure

- Treat step `10` exact-screened and step `12` generated as the remaining open
  late targets with separate evidence.
- Keep the newly landed step-`12` exact-screened and step-`14` generated floors
  from regressing while the remaining late work moves.
- Prefer changes that survive live prefix search over raw-catalog-only
  widening.
- Keep accepted parity and `full_telescopes_evaluated` stable while moving the
  open floors.

Done when:

- step `10` exact-screened reaches `120+`
- step `12` generated reaches `1200+`
- step `12` exact-screened stays `>= 400`
- step `14` generated stays `>= 3500`
- the default `10m` profile still preserves accepted parity through step `15`

### 3. Exact-Bound Tightening

- Strengthen exact prefix and terminal-prefix bounds so extra honest breadth
  turns into honest exact-screened mass rather than extra full evaluations.
- Use the remaining step-`10` and step-`12` floors as the scorecard for
  whether a bound change helped.
- Keep exact-screen reasons, prune classes, and stored narrative/event
  coverage stable while the bounds move.

Done when:

- the remaining late floors close without a large rise in
  `full_telescopes_evaluated`
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
