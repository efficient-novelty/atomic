# Demo Lane Checklist

Last updated: 2026-03-17

This checklist contains only the still-open work for `demo_breadth_shadow`.

Landed baseline such as config parsing, demo profiles, the phase machine,
persisted narratives/events, demo funnel and closure reporting, proof-close
reserve accounting, compare-tool demo evidence, and repeated
discovery-side reserve retuning is intentionally omitted here.

## 1. Honest Early Breadth

- [ ] Restore step 1 to `2144` generated raw candidates.
- [x] Show that steps `1` to `4` stay exhaustive or near-exhaustive inside the
      shared `90s` early window on this computer.
- [x] Keep full candidate-list generation on early steps wherever it remains
      affordable.

Latest stored evidence: fresh `runs/codex-demo-early-catalog` artifacts
restore full early candidate-list generation through step `4`, raise step `1`
to `1296` generated raw surface, and finish steps `1` to `4` in `122 ms`
total (`95/1/1/25 ms`). The shared early-window story is now backed by stored
evidence, but the explicit `2144` target is still open.

## 2. Structural Scheduling

- [x] Add a deterministic demo bucket key.
- [x] Track per-bucket stats for generated, admissible, exact-screened,
      pruned, fully scored, and best-overshoot outcomes.
- [x] Add a demo-specific structural priority tuple.
- [x] Keep heuristics structural/runtime-local and acceptance-independent.

## 3. Real Search Widening

- [x] Widen steps `5` to `9` with more `kappa`, support-form, and bridge-head
      variety.
- [x] Widen steps `10` to `12` with more family unions, reference patterns,
      nested `Pi` and `Sigma`, bridge heads, and reanchor variants.
- [x] Widen steps `13` to `15` with more operator, Hilbert, and temporal
      mixtures, mixed shells, historical reanchors, clause unions, and
      positional filters.
- [ ] Strengthen exact prefix bounds so widening does not explode full
      terminal work.

Latest stored evidence: fresh paired
`runs/codex-realistic-midstep-baseline` and
`runs/codex-demo-midstep-carrythrough` artifacts keep accepted parity through
step `15`, keep demo narrative artifacts complete (`text=15/15`, `events=15/15`),
and finally turn the landed step-`5` to `9` widening into stored live breadth:
generated raw counts move from realistic `3/3/3/5/4` to demo
`27/15/15/45/24`, exact-screened counts move from `1/1/1/1/1` to
`1/3/2/13/9`, and `full_telescopes_evaluated` stays `1` on every step from
`5` to `9`. Fresh `runs/codex-demo-family-surface` late-step artifacts remain
the current `10m` floor reference, preserving accepted parity through step
`15` while moving generated counts to `36/132/147/291/2292/1007` and
exact-screened counts to `18/82/83/178/1521/707` on steps `10` to `15`. The
remaining widening gap is no longer mid-step carry-through; it is the still-
open late generated and exact-screened floors plus the exact-prefix-bound work.

## 4. Floor Attainment

- [ ] Start hitting the configured late-step generated or exact-screened floors
      consistently, beginning with the default `10m` profile.
- [x] Keep `full_telescopes_evaluated` moderate relative to generated breadth.

Default `10m` signoff targets:

- [ ] Step `10`: `500+`
- [ ] Step `11`: `800+`
- [ ] Step `12`: `1200+`
- [ ] Step `13`: `2200+`
- [ ] Step `14`: `3500+`
- [ ] Step `15`: `5000+`

Latest stored evidence: fresh `runs/codex-demo-family-surface` artifacts still
miss the generated floors, but they now hit the step-`14`
`exact_screened_floor` (`1521 >= 1100`) and keep
`full_telescopes_evaluated = 1` on every late step while generated raw surface
improves to `36/132/147/291/2292/1007`.

## 5. Closure-Aware Replanning And Reason Codes

- [x] Extend the within-step controller beyond current reserve retunes,
      ordering, and handoff logic into stronger closure-aware replanning.
- [x] Record exact-screen reasons separately:
      partial-prefix bar failure, terminal-prefix completion failure,
      incumbent dominance, and legality/connectivity exact rejection.
- [x] Label every prune as sound, quotient/dedupe, or heuristic shaping.

Latest code evidence: the demo controller now promotes `Materialize` into
`ProofClose` with the explicit `closure_pressure_handoff` reason once a live
incumbent turns most pending exact surface into prune-ready certification work,
and the same closure signal now flips proof-close ordering into closure-first
before reserve tightness alone would force it. Stored step summaries still also
persist prune-class totals for quotient/dedupe, sound/minimality, and
heuristic shaping, while CLI debug output, `pen-cli inspect`,
`pen-cli --narrative`, and `scripts/compare_runs.py` all backfill the same
labeled totals from stored search stats when older artifacts predate the
explicit field. The four mandatory exact-screen reason totals remain explicit
and continue to derive backward-compatibly from the underlying incremental
counters when needed.

## 6. Narrative And Tooling

- [x] Add `--narrative` support in `pen-cli`.
- [x] Render a time bar and a closure bar.
- [x] Keep narrative output inside the intended per-step line budgets.
- [x] Rate-limit repeated live narrative budget pulses using the configured
      interval.
- [x] Add the remaining mandatory live events.
- [x] Report missing narrative artifacts explicitly in
      `scripts/compare_runs.py`.
- [x] State the honesty boundary explicitly in docs:
      no fake discoveries, no fake breadth, no silent replay fallback, and no
      claims beyond comparison-backed evidence.

Latest code and stored evidence: `pen-cli` still clips demo narrative sections
to `demo.narrative.max_lines_per_step` with explicit omission markers, and live
demo budget-retune pulses still obey `demo.narrative.pulse_interval_millis`.
`pen-search` now also has regression coverage that forces a `60s` pulse
interval and still emits the mandatory `proof_close` `100%` certification
milestone and final seal overshoot pulses under a step-`15` soft-cap handoff,
while earlier demo tests already keep phase-entry, floor-hit, reserve-
protection, and reserve-exhaustion events explicit. Fresh
`runs/codex-demo-midstep-carrythrough` artifacts also keep per-step narrative
and event artifacts complete across all `15` steps (`text=15/15`,
`events=15/15`).

## 7. Signoff Gates

- [x] `demo_breadth_shadow` preserves accepted-hash parity with guarded.
- [ ] Step 1 reports `2144`.
- [x] Steps `1` to `4` fit the shared early `90s` budget honestly.
- [x] The default `10m` profile completes within `600s` on this computer.
- [ ] Late steps show large honest breadth with moderate full evaluations.
- [x] No silent guarded or replay fallback is used when the demo lane misses
      budget or certification.
