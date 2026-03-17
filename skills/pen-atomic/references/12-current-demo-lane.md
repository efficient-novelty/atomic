# Current Demo Lane

Read this file when the task touches `demo_breadth_shadow`, demo evidence,
budget control, floor attainment, or comparison-backed widening status.

For live targets and signoff criteria, read the repo-level operational docs
first:

- [../../demo_lane_progress.md](../../demo_lane_progress.md)
- [../../demo_lane_plan.md](../../demo_lane_plan.md)
- [../../demo_lane_checklist.md](../../demo_lane_checklist.md)

## Stable Current Behavior

- `demo_breadth_shadow` is comparison-backed and still reuses realistic-shadow
  acceptance truth.
- The per-step phase machine is real:
  `Scout -> BreadthHarvest -> Materialize -> ProofClose -> Seal`.
- Demo runs persist per-step narratives, event streams, reserve/closure
  accounting, and phase handoff reasons.
- Early exhaustive scout narratives now persist the restored raw clause-catalog
  widths for step-audit work.
- Narrative rendering honors the configured line budget and pulse interval.
- The controller rebalances discovery versus proof-close reserve inside a step
  using live throughput and floor projections.
- Materialize and proof-close use deterministic structural bucket ordering and
  closure-aware handoffs.
- Stored step summaries persist exact-screen reason codes and prune classes.
- Demo generated-surface reporting counts raw roots, raw children, forced
  single-continuation collapses, and raw terminal completions.

## Widening That Already Survives Live Search

- Steps `1` to `4` can restore full clause-catalog candidate-list generation
  where affordable.
- Steps `5` to `9` already carry widened demo search mass through the live
  prefix engine.
- Demo prefix-family summaries and active-window clause filtering preserve the
  live demo late-family surface override instead of snapping back to realistic
  family matching before the queue widens.
- Step `10` / `kappa = 4` and step `11` / `kappa = 5` demo override roots now
  selectively skip the cached family-filter summary.
- On those same override surfaces, admitted-but-deprioritized completions do
  not compete for acceptance, so wider live queue exploration does not change
  accepted truth.

## Current Stored Baselines

- Early reference: `runs/codex-demo-early-catalog-v2`
  - step `1` generated raw = `1296`
  - step `1` raw clause widths = `36x36`
  - steps `1` to `4` complete in `140 ms` total (`96/1/1/42 ms`)
- Mid-step carry-through reference:
  `runs/codex-demo-midstep-carrythrough`
  - steps `5` to `9` generated = `27/15/15/45/24`
  - steps `5` to `9` exact-screened = `1/3/2/13/9`
  - `full_telescopes_evaluated = 1` on every step
- Late reference: `runs/codex-demo-late-surface-v4`
  compared against `runs/codex-realistic-late-baseline-v2`
  - steps `10` to `15` generated = `1344/4191/147/3995/2292/22715`
  - steps `10` to `15` exact-screened = `7/253/83/3123/1521/18749`
  - `full_telescopes_evaluated = 1` on every late step
  - accepted parity still holds through step `15`
    (`matches_reference_replay x15`)

## Current Open Gaps

| Area | Current | Target |
| --- | --- | --- |
| Step `1` generated raw | `1296` | `2144` |
| Step `10` exact-screened | `7` | `120+` |
| Step `12` generated raw | `147` | `1200+` |
| Step `12` exact-screened | `83` | `400+` |
| Step `14` generated raw | `2292` | `3500+` |

## Guardrails

- Keep `demo_breadth_shadow` comparison-backed.
- Keep guarded acceptance authoritative.
- Count only honest generated and honest exact-screened mass.
- Prefer stored run evidence over config intent when deciding whether a gap is
  closed.
- Treat changes that alter accepted hashes, materially raise
  `full_telescopes_evaluated`, or lean on silent fallback as regressions.
