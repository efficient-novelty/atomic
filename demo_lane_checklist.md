# Demo Lane Checklist

Last updated: 2026-03-17

This checklist contains only the still-open work for `demo_breadth_shadow`.

Landed baseline such as config parsing, demo profiles, the phase machine,
persisted narratives/events, demo funnel and closure reporting, proof-close
reserve accounting, compare-tool demo evidence, and repeated
discovery-side reserve retuning is intentionally omitted here.

## 1. Honest Early Breadth

- [ ] Restore step 1 to `2144` generated raw candidates.
- [ ] Show that steps `1` to `4` stay exhaustive or near-exhaustive inside the
      shared `90s` early window on this computer.
- [ ] Keep full candidate-list generation on early steps wherever it remains
      affordable.

Latest stored evidence: `runs/codex-demo-rawcount` raises step `1` to `546`
generated raw surface, up from `288`, but the explicit `2144` target is still
far away.

## 2. Structural Scheduling

- [x] Add a deterministic demo bucket key.
- [x] Track per-bucket stats for generated, admissible, exact-screened,
      pruned, fully scored, and best-overshoot outcomes.
- [x] Add a demo-specific structural priority tuple.
- [x] Keep heuristics structural/runtime-local and acceptance-independent.

## 3. Real Search Widening

- [ ] Widen steps `5` to `9` with more `kappa`, support-form, and bridge-head
      variety.
- [x] Widen steps `10` to `12` with more family unions, reference patterns,
      nested `Pi` and `Sigma`, bridge heads, and reanchor variants.
- [ ] Widen steps `13` to `15` with more operator, Hilbert, and temporal
      mixtures, mixed shells, historical reanchors, clause unions, and
      positional filters.
- [ ] Strengthen exact prefix bounds so widening does not explode full
      terminal work.

Latest stored evidence: fresh `runs/codex-demo-midlate-widening` artifacts keep
accepted parity through step `15` while the landed demo-only step-`10` to `12`
surface widening plus the earlier operator/Hilbert/temporal widening move late
generated counts to `9/15/15/11/12/14` and exact-screened counts to
`6/6/5/3/3/3`, so this workstream is improving but still far short of the
configured floors.

## 4. Floor Attainment

- [ ] Start hitting the configured late-step generated or exact-screened floors
      consistently, beginning with the default `10m` profile.
- [ ] Keep `full_telescopes_evaluated` moderate relative to generated breadth.

Default `10m` signoff targets:

- [ ] Step `10`: `500+`
- [ ] Step `11`: `800+`
- [ ] Step `12`: `1200+`
- [ ] Step `13`: `2200+`
- [ ] Step `14`: `3500+`
- [ ] Step `15`: `5000+`

## 5. Closure-Aware Replanning And Reason Codes

- [ ] Extend the within-step controller beyond current reserve retunes,
      ordering, and handoff logic into stronger closure-aware replanning.
- [ ] Record exact-screen reasons separately:
      partial-prefix bar failure, terminal-prefix completion failure,
      incumbent dominance, and legality/connectivity exact rejection.
- [ ] Label every prune as sound, quotient/dedupe, or heuristic shaping.

## 6. Narrative And Tooling

- [x] Add `--narrative` support in `pen-cli`.
- [x] Render a time bar and a closure bar.
- [x] Keep narrative output inside the intended per-step line budgets.
- [x] Rate-limit repeated live narrative budget pulses using the configured
      interval.
- [ ] Add the remaining mandatory live events.
- [x] Report missing narrative artifacts explicitly in
      `scripts/compare_runs.py`.
- [x] State the honesty boundary explicitly in docs:
      no fake discoveries, no fake breadth, no silent replay fallback, and no
      claims beyond comparison-backed evidence.

Latest code evidence: `pen-cli` now clips demo narrative sections to
`demo.narrative.max_lines_per_step` with explicit omission markers, and live
demo budget-retune pulses now obey `demo.narrative.pulse_interval_millis`
without hiding mandatory phase/milestone pulses. Fresh stored run artifacts are
still needed for the remaining mandatory-event closeout.

## 7. Signoff Gates

- [x] `demo_breadth_shadow` preserves accepted-hash parity with guarded.
- [ ] Step 1 reports `2144`.
- [ ] Steps `1` to `4` fit the shared early `90s` budget honestly.
- [x] The default `10m` profile completes within `600s` on this computer.
- [ ] Late steps show large honest breadth with moderate full evaluations.
- [x] No silent guarded or replay fallback is used when the demo lane misses
      budget or certification.
