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
- [ ] Widen steps `10` to `12` with more family unions, reference patterns,
      nested `Pi` and `Sigma`, bridge heads, and reanchor variants.
- [ ] Widen steps `13` to `15` with more operator, Hilbert, and temporal
      mixtures, mixed shells, historical reanchors, clause unions, and
      positional filters.
- [ ] Strengthen exact prefix bounds so widening does not explode full
      terminal work.

Latest stored evidence: the landed demo-only operator/Hilbert/temporal surface
widening plus raw generated-surface counting moves steps `10` to `15` only to
generated counts `5/11/11/11/12/14` and exact-screened counts `2/2/1/3/3/3`,
so this workstream remains open.

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
- [ ] Keep narrative output inside the intended per-step line budgets.
- [ ] Add the remaining mandatory live events and pulse rate limiting.
- [x] Report missing narrative artifacts explicitly in
      `scripts/compare_runs.py`.
- [x] State the honesty boundary explicitly in docs:
      no fake discoveries, no fake breadth, no silent replay fallback, and no
      claims beyond comparison-backed evidence.

## 7. Signoff Gates

- [x] `demo_breadth_shadow` preserves accepted-hash parity with guarded.
- [ ] Step 1 reports `2144`.
- [ ] Steps `1` to `4` fit the shared early `90s` budget honestly.
- [x] The default `10m` profile completes within `600s` on this computer.
- [ ] Late steps show large honest breadth with moderate full evaluations.
- [x] No silent guarded or replay fallback is used when the demo lane misses
      budget or certification.


## 8. Immediate Execution Slice

- [ ] Run an early-window recovery pass that first restores step `1` toward
      `2144`, then revalidates steps `2` to `4` inside the shared `90s` window
      with parity preserved.
- [ ] Run a late-step widening pass for steps `10` to `15` that increases both
      generated and exact-screened breadth before further scheduler retuning.
- [ ] Close controller/reporting integration by emitting closure-aware
      replanning decisions plus exact-screen reason and prune-taxonomy labels
      in the same stored evidence consumed by `--narrative` and
      `scripts/compare_runs.py`.
