# Demo Lane Progress

Last updated: 2026-03-17

This file tracks only the open status of `demo_breadth_shadow`.

Landed baseline is no longer repeated here in detail. The repo already has the
demo config profiles, phase machine, budget controller, persisted
narratives/events, demo funnel and closure stats, proof-close reserve
accounting, compare-tool demo evidence, and repeated discovery-side reserve
retuning during `BreadthHarvest`.

## Current Status

- `demo_breadth_shadow` remains comparison-backed and still reuses
  `realistic_frontier_shadow` for the underlying search semantics.
- `demo_breadth_shadow` now widens the step-`10` to `12`
  modal-shell/axiomatic-bridge and connection/curvature clause surfaces plus
  the operator-bundle, Hilbert-functional, and temporal-shell late-family
  clause surfaces during demo discovery while still reusing realistic-shadow
  admissibility for acceptance parity.
- The lane now reports its misses honestly instead of hiding them behind
  generic breadth counts or debug text.
- Demo generated-surface reporting and live narrative progress now count raw
  root prefixes, raw child prefixes, forced single-continuation collapses, and
  raw terminal completions instead of falling back to
  `max(prefixes_created, enumerated_candidates)`.
- Demo steps `1` to `4` now also restore full clause-catalog candidate-list
  generation where it remains affordable instead of only exposing the current
  realistic prefix-frontier shadow on those early steps.
- The lane now has a deterministic demo bucket scheduler keyed only by
  structural/runtime-local evidence, with stored per-bucket generated,
  admissible, exact-screened, pruned, fully scored, and best-overshoot stats.
- CLI debug output, `--narrative`, and `scripts/compare_runs.py` now surface
  compact bucket summaries so the ordering story is visible in stored evidence.
- Stored step summaries, CLI debug output, and `pen-cli --narrative` now also
  separate the mandatory exact-screen reason codes into partial-prefix bar
  failure, terminal-prefix completion failure, incumbent dominance, and
  legality/connectivity exact rejection, with backward-compatible derivation
  from older incremental counters when needed.
- Stored step summaries now also persist prune-class totals for
  quotient/dedupe, sound/minimality, and heuristic shaping, while CLI debug
  output, `pen-cli inspect`, `pen-cli --narrative`, and
  `scripts/compare_runs.py` backfill the same labeled totals from stored
  search stats when older artifacts predate the explicit field.
- `pen-cli --narrative` now enforces the configured per-step line budget by
  trimming low-priority event, retained-candidate, prune, and trace tails with
  explicit omission markers instead of letting long sections grow without
  bound.
- Repeated live scout and breadth-harvest budget-retune pulses now honor the
  configured `pulse_interval_millis` while mandatory phase-entry, milestone,
  reserve-exhaustion, and seal pulses still emit immediately.
- Demo materialize can now also hand off into `ProofClose` with the explicit
  `closure_pressure_handoff` reason once a live incumbent makes most pending
  retained exact surface prune-ready, and the same closure-pressure summary now
  flips proof-close ordering into closure-first even before reserve tightness
  alone would force that mode change.
- Fresh stored evidence in `runs/codex-demo-early-catalog` now shows that
  restored early candidate-list generation directly: step `1` reports
  `generated_raw_prefixes = 1296`, steps `1` to `4` finish in `122 ms` total
  (`95/1/1/25 ms`), and the stored step summaries plus appended narrative
  progress both reflect those raw early counts.
- Fresh stored evidence in `runs/codex-demo-midlate-widening` still preserves
  accepted parity through step `15` (`matches_reference_replay x15`) and
  finishes far under the default `600s` ceiling on this computer.
- Exhaustive `pen-search` coverage now confirms that the demo-only late-family
  enumerators beat realistic-shadow enumeration on steps `13` to `15`, and the
  step-`15` demo lane now includes an extra temporal exchange bridge variant
  that still preserves the reference acceptance under exact comparison.
- Demo prefix-family summaries and active-window clause filtering now preserve
  the live demo late-family surface override instead of snapping those prefixes
  back to realistic-shadow family matching before the online queue can widen
  honestly, while demo terminal admissibility still falls back to the direct
  realistic-shadow check whenever that override is active so accepted parity
  stays authoritative.
- Fresh stored evidence in `runs/codex-demo-family-surface` still preserves
  accepted parity through step `15` (`matches_reference_replay x15`) and now
  turns the landed late-family widening into materially broader stored live
  breadth: generated raw surface moves to `36`, `132`, `147`, `291`, `2292`,
  and `1007` on steps `10` to `15`, exact-screened counts move to
  `18`, `82`, `83`, `178`, `1521`, and `707`, every late step still keeps
  `full_telescopes_evaluated = 1`, and step `14` now hits the configured
  exact-screened floor (`1521 >= 1100`).
- The main remaining problem is no longer "missing demo evidence." The main
  problem is now pushing the remaining surfaced late floors from this much
  stronger base while the mandatory live-event closeout is still unfinished.

## What Still Blocks Signoff

### 1. Early Breadth Still Misses The Step-1 Floor

- step 1 now reports `1296` generated raw surface in
  `runs/codex-demo-early-catalog`, up from `546` in
  `runs/codex-demo-rawcount` and `288` before honest raw counting, but it
  still misses the explicit `2144` floor
- the same stored run now restores full early candidate-list generation
  through step `4` where affordable and finishes steps `1` to `4` in `122 ms`
  total, so the remaining early-breadth gap is now the step-`1` floor itself
  rather than missing shared-window evidence

### 2. Real Widening Is Still Missing

- steps `5` to `9` still inherit too much of the current realistic-shadow
  surface
- configured late-step generated and exact-screened floors are now stored and
  reported, but they are not yet being hit consistently
- the latest stored `10m` run now raises generated raw surface to
  `36`, `132`, `147`, `291`, `2292`, and `1007` on steps `10` to `15`, with
  exact-screened counts of `18`, `82`, `83`, `178`, `1521`, and `707`, so the
  live late breadth story is no longer "barely above realistic shadow" even
  though most configured generated floors and all but one exact-screened floor
  are still open
- the landed demo-specific structural bucket scheduler plus the preserved demo
  family surface now have enough honest breadth underneath them to show real
  widening, but the next gap is pushing that wider late surface through the
  remaining configured floors, especially step `13` and step `15`
- the new closure-pressure handoff improves the within-step split, but it still
  needs the remaining widening and bound work to hit the last late floors

### 3. Reporting Closeout Is Still Incomplete

- `pen-cli` now supports `--narrative` for appending stored per-step demo
  narrative output, including explicit time and closure bars
- `scripts/compare_runs.py` now flags missing step narrative and event
  artifacts explicitly for demo lanes
- the remaining narrative/tooling gap is now the still-missing mandatory live
  events rather than line-budget discipline or pulse-rate limiting

## Next Priorities

1. Push the honest early breadth story from the current step-`1` `1296`
   toward `2144` without regressing the newly restored full candidate-list
   generation and shared early-window evidence on steps `1` to `4`.
2. Build on the landed demo-only step-`10` to `12` widening plus the earlier
   late-family widening, the preserved demo family-surface override, and the
   honest raw-surface counting to keep pushing steps `10` to `15` toward the
   remaining generated and exact-screened floors, especially step `13` and
   step `15`.
3. Finish the remaining mandatory live-event closeout.
