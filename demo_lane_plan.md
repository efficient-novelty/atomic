# Demo Lane Plan

Last updated: 2026-03-16

This file is the working plan for a new comparison-backed demo lane. The
primary target is a truthful run that fits within 10 minutes on this computer,
with narrower 5-minute and broader 15-minute profiles using the same search
logic. The lane must keep early exhaustive behavior honest, retain the current
step-1 raw breadth story, and broaden late-step search mass without changing
acceptance truth.

Implementation sequencing lives in
[`demo_lane_checklist.md`](./demo_lane_checklist.md). Current forward status
lives in [`demo_lane_progress.md`](./demo_lane_progress.md).

## Objective

Introduce `demo_breadth_shadow` as a child of `realistic_frontier_shadow`, not
as a replacement for `strict_canon_guarded`.

The lane should show a visibly broad live search that:

- keeps step 1 at the current `2144` raw candidates
- keeps generating the full list of potential candidates for the first few
  steps whenever that remains reasonable inside a shared 90-second exhaustive
  window on this computer
- gradually focuses the lane using theory-guided and quantum-inspired
  scheduling heuristics
- still generates and evaluates a large honest late-step surface
- assumes little or no knowledge of what the canon actually is

The goal is not to make the lane look broad. The goal is to make it broad in a
way that is structurally real, comparison-backed, and exactly certified.

## Non-Negotiables

- `strict_canon_guarded` stays unchanged and authoritative.
- `demo_breadth_shadow` is comparison-backed only.
- Exact rational `rho`, `bar`, and overshoot comparisons stay authoritative.
- Semantic minimality and deterministic tie-breaking stay unchanged.
- No semantic names or target IDs enter hot-path search or evaluation.
- No stochastic acceptance or float-based hot-path ranking is introduced.
- Every reported candidate, prune, and discovery must come from real search
  work, not random noise or scripted theater.
- If the lane misses a budget or cannot certify a winner inside budget, it must
  say so plainly and persist partial evidence. No silent fallback.

## Search Shape

The demo lane should widen and focus in stages.

- Steps 1 to 4 use a shared 90-second exhaustive or near-exhaustive window.
- Step 1 must keep the current `2144` raw candidates.
- Early steps should keep full candidate-list generation where it is still
  reasonable inside the shared 90-second window.
- Steps 5 to 9 should still explore broadly, but begin using structural
  prioritization to focus the lane.
- Steps 10 to 15 should keep a large honest candidate funnel while exact
  screening, admissibility reuse, and proof-close work keep full terminal
  evaluation counts moderate.

This lane should search the probability space strategically, not cosmetically.
The heuristics should be structural and runtime-derived, not target-aware.

## Honest Breadth Model

Late-step "large" should mean real search mass in the funnel, not wasteful full
exact evaluation counts. The console and stored artifacts should report a
funnel with distinct meanings:

```text
generated_raw_prefixes
canonical_prefix_signatures
well_formed_terminals
hard_admissible
exact_bound_screened
exact_bound_pruned
heuristic_dropped
full_telescopes_evaluated
bar_clearers
semantically_minimal_clearers
winner_overshoot
```

The primary breadth story should be:

- large `generated_raw_prefixes`
- large `hard_admissible`
- large `exact_bound_screened`
- moderate `full_telescopes_evaluated`

That keeps the lane honest and aligned with the current exact-bound direction.

## Step Phase Machine

Each step should run through an explicit state machine:

1. `Scout`
2. `BreadthHarvest`
3. `Materialize`
4. `ProofClose`
5. `Seal`

### Scout

Sample the whole admissible surface broadly for a short fixed time or work
quantum. Measure throughput on this computer:

- prefix generation per second
- hard-admissibility checks per second
- exact prefix-bound checks per second
- full terminal evaluations per second

### BreadthHarvest

Expand prefixes under diversity quotas and bucket budgets until either:

- the step's breadth floor is met, or
- the proof-close reserve must be protected

### Materialize

Turn only the strongest surviving prefixes into terminal candidates and run
exact evaluation under a soft full-evaluation cap.

### ProofClose

Spend the reserved budget on exact branch-and-bound closure until the incumbent
cannot be beaten by any surviving prefix under sound bounds.

### Seal

Apply semantic minimality, rank bar-clearers, and accept the winner by minimal
positive overshoot.

## Quantum-Inspired Heuristics

The quantum language belongs in scheduling and ordering, not in truth or
acceptance.

- amplitude = structural promise
- constructive interference = one prefix satisfying several live obligations
- destructive interference = down-ranking overrepresented or dominated buckets
- collapse = exact proof-close onto the certified winner

These heuristics must stay structural and runtime-local. They should assume no
or minimal canon knowledge.

## Structural Scheduling

Use a deterministic bucketed scheduler keyed only by structural information
already available during search. A representative bucket key is:

```rust
struct PrefixBucketKey {
    kappa_band: u8,
    focus_family: FocusFamily,
    bridge_head: BridgeHeadClass,
    support_form: SupportForm,
    library_ref_pattern: RefPattern,
    modal_temporal_mix: MixClass,
}
```

Per-bucket stats should track:

```rust
struct BucketStats {
    generated: u64,
    well_formed: u64,
    admissible: u64,
    exact_bound_hits: u64,
    exact_bound_prunes: u64,
    full_scored: u64,
    bar_clearers: u64,
    best_overshoot: Option<ExactRational>,
}
```

The deterministic priority tuple should prefer:

- hard exact survivability
- obligation-closure gain
- bridge and completion potential
- underexplored buckets
- low redundancy
- stable tie-breaking

## Time Budgets

The 10-minute profile is the default demo target on this computer. The 5-minute
and 15-minute profiles use the same lane and telemetry surface, with narrower
and broader search pressure respectively.

All three profiles share the same early rule:

- steps 1 to 4 share a 90-second exhaustive or near-exhaustive window
- step 1 must still report the current `2144` raw candidates

### Profile Summary

| Profile | Total budget | Early exhaustive window | Baseline step budgets | Adaptive reserve |
| --- | --- | --- | --- | --- |
| `5m` | `300s` | `90s` | `210s` across steps 5 to 15 | `30s` |
| `10m` | `600s` | `90s` | `390s` across steps 5 to 15 | `120s` |
| `15m` | `900s` | `90s` | `470s` across steps 5 to 15 | `340s` |

Reserve 25 to 40 percent of each step's usable budget for proof-close.

### 5-Minute Profile

Use this as the narrower demo pass.

| Step | Budget |
| --- | --- |
| 5 | `8s` |
| 6 | `10s` |
| 7 | `11s` |
| 8 | `12s` |
| 9 | `14s` |
| 10 | `10s` |
| 11 | `12s` |
| 12 | `16s` |
| 13 | `22s` |
| 14 | `28s` |
| 15 | `37s` |

### 10-Minute Profile

Use this as the default demo pass.

| Step | Budget |
| --- | --- |
| 5 | `12s` |
| 6 | `14s` |
| 7 | `18s` |
| 8 | `24s` |
| 9 | `32s` |
| 10 | `15s` |
| 11 | `25s` |
| 12 | `35s` |
| 13 | `55s` |
| 14 | `70s` |
| 15 | `90s` |

### 15-Minute Profile

Use this as the broader comparison pass. Keep the late-step floors that were
already proposed for steps 10 to 15.

| Step | Budget floor |
| --- | --- |
| 5 | `20s` |
| 6 | `24s` |
| 7 | `28s` |
| 8 | `34s` |
| 9 | `44s` |
| 10 | `10s` |
| 11 | `20s` |
| 12 | `30s` |
| 13 | `50s` |
| 14 | `80s` |
| 15 | `130s` |

The 15-minute profile should spend most of its adaptive reserve on steps 13 to
15 proof-close and spill behavior.

## Breadth Floors

The budget controller should calibrate online on this computer, but the lane
should still carry explicit floor targets for late steps.

Generated or exactly screened late-step targets:

| Step | `5m` floor | `10m` floor | `15m` floor |
| --- | --- | --- | --- |
| 10 | `300+` | `500+` | `700+` |
| 11 | `500+` | `800+` | `1100+` |
| 12 | `800+` | `1200+` | `1700+` |
| 13 | `1500+` | `2200+` | `3000+` |
| 14 | `2500+` | `3500+` | `4500+` |
| 15 | `3500+` | `5000+` | `6500+` |

These floors refer to real generated or exactly screened search mass, not
inflated terminal evaluation counts.

## Hardware-Adaptive Budget Control

Because this is explicitly "on this computer," the lane should calibrate
online.

During `Scout`, measure:

- prefix expansions per second
- admissibility checks per second
- exact prefix-bound checks per second
- full evaluations per second

Then compute feasible breadth floors for the rest of the step, widen until the
floor is met or proof-close reserve is endangered, and carry unused budget
forward to later difficult steps.

## Narrative And Progress

The lane should emit structured events, not worker-side print statements.

- `pen-search` emits `NarrativeEvent`s
- `pen-cli` renders the console story and persists the event stream
- step summaries keep funnel counters, closure progress, and ranked clearers

The narrative should:

- stay between 30 and 200 lines per step
- show a time bar and a closure bar
- distinguish sound prune, quotient or dedupe, and heuristic shaping
- print discoveries, incumbent improvements, prune waves, and final clearers

Recommended line targets:

- steps 1 to 4: 30 to 50 lines
- steps 5 to 9: 40 to 80 lines
- steps 10 to 12: 80 to 140 lines
- steps 13 to 15: 120 to 200 lines

## Widening Order

Widen the search surface in this order.

### Steps 1 To 4

Keep current exhaustive behavior as much as possible inside the shared
90-second window. Preserve the current step-1 `2144` raw-candidate story.

### Steps 5 To 9

Do modest widening:

- slightly broader `kappa` bands
- more support-form variety
- more bridge-head choices
- more near-exhaustive enumeration where still cheap

### Steps 10 To 12

Broaden more aggressively:

- more family unions
- more library-reference patterns
- more nested `Pi` and `Sigma` patterns
- more bridge-head alternatives
- more reanchor variants

### Steps 13 To 15

Diverge most strongly here:

- more operator, Hilbert, and temporal mixtures
- more mixed-shell bridge shapes
- more historical reanchor variants
- wider clause unions and positional filters
- stronger exact prefix bounds so the frontier can stay broad without wasting
  full terminal work

## Success Criteria

This plan is successful only if all of the following remain true:

- `strict_canon_guarded` still reproduces the current 15-step sequence
- `demo_breadth_shadow` preserves accepted-hash parity with the guarded lane
- steps 1 to 4 stay exhaustive or near-exhaustive inside the shared
  90-second early budget
- step 1 still keeps the current `2144` raw candidates
- the 10-minute profile completes within `600s` on this computer
- late steps show large honest generated or exactly screened search mass
- `full_telescopes_evaluated` stays moderate relative to generated breadth
- every prune is labeled as sound, quotient or dedupe, or heuristic shaping
- there is no silent replay or guarded fallback when the demo lane misses
  budget or certification

## Primary Touch Points

Expected implementation work should center on:

- `crates/pen-search/src/config.rs`
- `crates/pen-search/src/engine.rs`
- `crates/pen-search/src/frontier.rs`
- `crates/pen-search/src/scheduler.rs`
- `crates/pen-search/src/state.rs`
- `crates/pen-search/src/branch_bound.rs`
- `crates/pen-search/src/diversify.rs`
- `crates/pen-search/src/enumerate.rs`
- `crates/pen-search/src/expand.rs`
- `crates/pen-type/src/admissibility.rs`
- `crates/pen-type/src/obligations.rs`
- `crates/pen-store/src/manifest.rs`
- `crates/pen-cli/src/report.rs`
- `crates/pen-cli/src/narrative.rs`
- `crates/pen-cli/src/progress.rs`
- `scripts/compare_runs.py`

Persisted artifacts should include:

- `reports/steps/step-XX-summary.json`
- `reports/steps/step-XX-narrative.txt`
- `reports/steps/step-XX-events.ndjson`
