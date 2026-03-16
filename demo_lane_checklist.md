# Demo Lane Checklist

Last updated: 2026-03-16

This checklist turns the demo-lane proposal into a repo-ready implementation
sequence. The default signoff target is the 10-minute profile on this computer,
with 5-minute and 15-minute variants using the same lane.

## Progress

- [x] Added `demo_breadth_shadow` to the runtime search-profile surface.
- [x] Added optional `[demo]` budget and narrative config parsing to
      `RuntimeConfig`.
- [x] Added runnable `5m`, `10m`, and `15m` demo config files.
- [x] Set the default demo metadata profile to `10m`.
- [x] Wired the new profile to the current realistic-shadow search semantics as
      an honest first milestone while the dedicated demo scheduler lands.
- [x] Updated [`README.md`](./README.md) and
      [`docs/SEARCH_CONTRACT.md`](./docs/SEARCH_CONTRACT.md) to document the
      current scaffold and its honesty boundary.
- [x] Added first-pass structured demo narrative events to live step reports.
- [x] Added persisted demo narrative artifacts in `reports/steps/` for demo
      runs.

## 0. Invariants

- [ ] Keep `strict_canon_guarded` unchanged and authoritative.
- [ ] Add `demo_breadth_shadow` as a comparison-backed child of
      `realistic_frontier_shadow`.
- [ ] Preserve exact rational `rho`, `bar`, and overshoot comparisons.
- [ ] Preserve semantic minimality and deterministic tie-breaking.
- [ ] Keep hot-path search and eval name-free.
- [ ] Do not add stochastic acceptance or float-based ranking.
- [ ] Keep step 1 at the current `2144` raw candidates.
- [ ] Keep steps 1 to 4 exhaustive or near-exhaustive inside a shared
      90-second window on this computer.
- [ ] Treat late-step "large" as real generated, admissible, and exactly
      screened search mass, not theatrical full-evaluation counts.
- [ ] Do not generate random noise for demo purposes.
- [ ] If the lane misses budget or certification, report it plainly and persist
      partial evidence. No silent fallback.

## 1. Config Surface

- [x] Add `SearchProfile::DemoBreadthShadow` in
      `crates/pen-search/src/config.rs`.
- [x] Add `configs/demo_breadth_shadow_5m.toml`.
- [x] Add `configs/demo_breadth_shadow_10m.toml`.
- [x] Add `configs/demo_breadth_shadow_15m.toml`.
- [x] Add a `[demo]` config section covering:
      `profile`, `total_budget_sec`, `early_exhaustive_budget_sec`,
      `adaptive_reserve_sec`, `proof_close_reserve_fraction`,
      `scout_fraction`, `narrative`, `floors`, and `caps`.
- [x] Add config parsing tests for the new search profile and all three demo
      configs.
- [x] Route `demo_breadth_shadow` through the current realistic-shadow engine
      until the dedicated demo breadth controller and scheduler land.

## 2. Budget Model

- [x] Make the default target the 10-minute profile.
- [x] Use a shared `90s` exhaustive or near-exhaustive early window for steps 1
      to 4.
- [ ] Keep `2144` as the explicit step-1 raw breadth floor.
- [x] Reserve 25 to 40 percent of each step's usable budget for proof-close.
- [x] Add an adaptive reserve that can spill toward steps 13 to 15.

### 5-Minute Profile

- [x] Total budget: `300s`
- [x] Early exhaustive window: `90s`
- [x] Step budgets:
      `5=8s`, `6=10s`, `7=11s`, `8=12s`, `9=14s`, `10=10s`, `11=12s`,
      `12=16s`, `13=22s`, `14=28s`, `15=37s`
- [x] Adaptive reserve: `30s`

### 10-Minute Profile

- [x] Total budget: `600s`
- [x] Early exhaustive window: `90s`
- [x] Step budgets:
      `5=12s`, `6=14s`, `7=18s`, `8=24s`, `9=32s`, `10=15s`, `11=25s`,
      `12=35s`, `13=55s`, `14=70s`, `15=90s`
- [x] Adaptive reserve: `120s`

### 15-Minute Profile

- [x] Total budget: `900s`
- [x] Early exhaustive window: `90s`
- [x] Keep the proposed late-step floors:
      `10=10s`, `11=20s`, `12=30s`, `13=50s`, `14=80s`, `15=130s`
- [x] Use baseline middle-step floors:
      `5=20s`, `6=24s`, `7=28s`, `8=34s`, `9=44s`
- [x] Adaptive reserve: `340s`

## 3. Step Phase Machine

- [x] Add `StepPhase::{Scout, BreadthHarvest, Materialize, ProofClose, Seal}`.
- [x] Add a `DemoBudgetController` in `crates/pen-search/src/engine.rs`.
- [x] During `Scout`, measure:
      prefix generation rate, admissibility rate, exact-bound rate, and full
      evaluation rate on this computer.
- [x] After `Scout`, let sampled throughput retune discovery versus
      proof-close reserve when the current-step floor projection is still under
      pressure.
- [x] During `BreadthHarvest`, keep widening until breadth floors are met or
      proof-close reserve must be protected.
- [x] During `Materialize`, enforce a soft cap on full terminal evaluations.
- [ ] During `ProofClose`, spend reserved budget on exact branch-and-bound
      closure.
- [ ] During `Seal`, apply semantic minimality and minimal-positive-overshoot
      winner selection.

## 4. Honest Funnel Counters

- [ ] Add funnel counters to `pen-search` and step reports:
      `generated_raw_prefixes`, `canonical_prefix_signatures`,
      `well_formed_terminals`, `hard_admissible`, `exact_bound_screened`,
      `exact_bound_pruned`, `heuristic_dropped`,
      `full_telescopes_evaluated`, `bar_clearers`,
      `semantically_minimal_clearers`, and `winner_overshoot`.
- [ ] Add closure tracking:
      `frontier_total_seen`, `frontier_certified_nonwinning`,
      and `closure_percent`.
- [ ] Distinguish sound prune, quotient or dedupe, and heuristic shaping in
      stored artifacts and console output.
- [ ] Keep existing counters; do not replace them.

## 5. Structural Scheduling

- [ ] Add a deterministic bucket key in
      `crates/pen-search/src/diversify.rs` or `scheduler.rs`.
- [ ] Track per-bucket stats:
      `generated`, `well_formed`, `admissible`, `exact_bound_hits`,
      `exact_bound_prunes`, `full_scored`, `bar_clearers`,
      and `best_overshoot`.
- [ ] Add a demo-specific deterministic priority tuple that favors:
      exact survivability, obligation closure, bridge potential,
      underexplored buckets, low redundancy, and stable tie-breaking.
- [ ] Use theory-guided and quantum-inspired heuristics only in ordering and
      scheduling, never in acceptance.
- [ ] Keep the heuristics structural and runtime-local so the lane assumes
      little or no canon knowledge.

## 6. Search-Surface Widening

### Steps 1 To 4

- [ ] Preserve current exhaustive behavior wherever it still fits inside the
      shared `90s` window.
- [ ] Preserve the current step-1 `2144` raw-candidate count.
- [ ] Keep full candidate-list generation for the early steps where it is still
      reasonable on this computer.

### Steps 5 To 9

- [ ] Slightly widen `kappa` bands.
- [ ] Add more support-form variety.
- [ ] Add more bridge-head choices.
- [ ] Keep near-exhaustive enumeration where it remains cheap.

### Steps 10 To 12

- [ ] Widen family unions.
- [ ] Widen library-reference patterns.
- [ ] Add more nested `Pi` and `Sigma` patterns.
- [ ] Add more bridge-head alternatives.
- [ ] Add more reanchor variants.
- [ ] Rely on exact screening before full materialization.

### Steps 13 To 15

- [ ] Widen operator, Hilbert, and temporal mixtures.
- [ ] Admit more mixed-shell bridge shapes.
- [ ] Keep multiple historical reanchor variants alive.
- [ ] Widen clause unions and positional filters.
- [ ] Strengthen exact prefix bounds so the frontier can stay broad without
      exploding full terminal work.

## 7. Breadth Floors

- [ ] Treat the following as generated or exactly screened late-step floors.

### 5-Minute Floors

- [ ] Step 10: `300+`
- [ ] Step 11: `500+`
- [ ] Step 12: `800+`
- [ ] Step 13: `1500+`
- [ ] Step 14: `2500+`
- [ ] Step 15: `3500+`

### 10-Minute Floors

- [ ] Step 10: `500+`
- [ ] Step 11: `800+`
- [ ] Step 12: `1200+`
- [ ] Step 13: `2200+`
- [ ] Step 14: `3500+`
- [ ] Step 15: `5000+`

### 15-Minute Floors

- [ ] Step 10: `700+`
- [ ] Step 11: `1100+`
- [ ] Step 12: `1700+`
- [ ] Step 13: `3000+`
- [ ] Step 14: `4500+`
- [ ] Step 15: `6500+`

- [ ] Keep `full_telescopes_evaluated` moderate relative to generated breadth.

## 8. Exact Screening And Soft Caps

- [ ] Promote exact screening into an explicit decision stage.
- [ ] Record exact-screen reasons separately:
      partial-prefix bar failure, terminal-prefix completion failure,
      incumbent dominance, and legality or connectivity exact rejection.
- [x] Add per-step `full_eval_soft_cap` settings.
- [x] Permit cap overrun only during `ProofClose` when certification requires
      it.
- [x] Surface cap overruns in the narrative with the reason.

## 9. Narrative And Progress

- [x] Add a search-side observer and a `NarrativeEvent` stream.
- [x] Add `crates/pen-cli/src/narrative.rs`.
- [x] Add `crates/pen-cli/src/progress.rs`.
- [ ] Add `--narrative` support in `pen-cli`.
- [ ] Render a time bar and a closure bar.
- [ ] Keep narrative output between 30 and 200 lines per step.
- [ ] Print mandatory events:
      step open, phase changes, first admissible bucket in a new class,
      first bar-clearer, incumbent improvements, prune waves, and final
      ranked clearer list.
- [ ] Rate-limit live pulses to roughly 250 to 1000 ms depending on step
      class.
- [x] Persist:
      `reports/steps/step-XX-narrative.txt` and
      `reports/steps/step-XX-events.ndjson`.

## 10. Comparison And Reporting

- [ ] Extend `scripts/compare_runs.py` for `demo_breadth_shadow`.
- [ ] Compare accepted-hash parity against guarded.
- [ ] Compare funnel counters and closure percent per step.
- [ ] Report breadth-floor hits or misses explicitly.
- [ ] Report full-eval soft-cap overruns explicitly.
- [ ] Report missing narrative artifacts explicitly.

## 11. Docs

- [x] Update `docs/SEARCH_CONTRACT.md` with the demo lane as a
      comparison-backed profile.
- [x] Update `README.md` with the new configs and run commands.
- [x] Link this checklist from [`demo_lane_plan.md`](./demo_lane_plan.md).
- [ ] State the honesty boundary explicitly:
      no random noise, no fake discovery lines, no silent replay fallback,
      and no claims beyond comparison-backed evidence.

## 12. Recommended PR Order

- [ ] PR 1: config surface and new profile enum
- [ ] PR 2: funnel counters and report schema
- [ ] PR 3: observer pipeline and narrative renderer
- [ ] PR 4: budget controller and step phase machine
- [ ] PR 5: bucketed scheduler and structural priority
- [ ] PR 6: late-surface widening plus exact screening
- [ ] PR 7: proof-close hardening and parity gates
- [ ] PR 8: docs, compare tooling, and signoff notes

## 13. Acceptance

- [ ] `strict_canon_guarded` still reproduces the current 15-step sequence.
- [ ] `demo_breadth_shadow` preserves accepted-hash parity with guarded.
- [ ] Steps 1 to 4 stay exhaustive or near-exhaustive inside the shared
      `90s` early budget.
- [ ] Step 1 still reports `2144` raw candidates.
- [ ] The 10-minute profile completes within `600s` on this computer.
- [ ] Late steps show large honest generated or exactly screened breadth.
- [ ] `full_telescopes_evaluated` remains moderate.
- [ ] Every prune is labeled by type.
- [ ] No silent guarded or replay fallback is used when the demo lane misses
      budget or certification.
