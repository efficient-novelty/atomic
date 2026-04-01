# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-01

This note is the exact next work order for `desktop_claim_shadow`.

## Keep Fixed

- Keep the current short baseline:
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Keep the current run to beat:
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
- Keep the replay harness corpus and benchmark files:
  - `tests/fixtures/claim_runtime/remaining_one_plateau_fixtures.json`
  - `tests/fixtures/claim_runtime/remaining_one_plateau_benchmark.json`
- Keep the current 20-minute intended-profile validation target from the stored
  step-`4` stream of `prefix-local-score-v1`:
  - `elapsed_millis = 1203991`
  - `prefix_states_explored = 123`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 109690`
  - `frontier_queue_len = 2652`
  - RSS `= 491208704` bytes

## Current Read

- The first continuation-cone slice is now landed on top of the older winner.
  Remaining-one exact bound checks, compact summary build, and compact
  materialization now use
  `SingleClauseStructuralNuContext` plus
  `TerminalClauseNuFacts`
  to make more bar-clearability and primary-overshoot decisions before scratch
  telescope assembly whenever connectivity needs no fallback and admissibility
  is already known from the local prefix surface.
- The release replay harness stayed parity-clean on the stored plateau
  fixtures.
- The checked-in five-surface replay benchmark improved overall from
  `163951 us` to `150440 us`.
  It improved surfaces `24`, `140`, `332`, and `335`, but regressed surface
  `74/147639`, so the slice is a real local win overall, not a uniform win at
  every stored surface.
- The full intended-profile rerun
  `prefix-local-score-v1`
  was then manually stopped during step `4`.
  The authoritative stored read is
  `reports/steps/step-04-live.ndjson`.
- That run materially passed the old stored `576` wall and the old stored
  `1038` wall, then stopped at:
  - `prefix_states_explored = 1095`
  - `prefix_cache_groups = 43`
  - `prefix_cache_candidates = 122481`
  - `elapsed_millis = 10815742`
  - `terminal_summary_build_millis = 10751697`
  - RSS `= 3175555072` bytes
- The next loop is no longer "let every contender run as long as possible."
  The next loop is now a short intended-profile optimization loop with a hard
  `20` minute cap.

## Do This Next

### 1. Reopen Code Work Now

Do not run another uncapped intended-profile continuation first.
The next move is the next code slice on top of
`prefix-local-score-v1`.

### 2. Implement The Next Slice In This Order

1. Make `TerminalClauseNuFacts` a mandatory clause-catalog sidecar on every
   winning-path remaining-one evaluation.
   Keep `structural_nu_with_clause_facts(...)` on the hot path and keep
   `structural_nu_with_clause(...)` off it.

2. After that slice lands, split the step-`4` remaining-one kernel into two
   kernels.
   - true no-miss hit-path plateau kernel
   - general fallback kernel

3. After that, compress `lib_refs` inside
   `SingleClauseStructuralNuContext`.
   Preferred order:
   - inline small array for the common case
   - dense bitset after a threshold
   - sorted boxed slices only where serialization or debug parity needs them

4. If second-pass duplication still matters after those slices, prefer a tiny
   survivor sketch.
   Carry only the clause refs and facts needed for the best primary rank and
   tie-break-relevant survivors.
   Do not wake the dormant general cached-summary reopen machinery first.

### 3. Validation Loop For Each New Slice

For the next attempts, validate in this order:

1. Rerun only the claim-focused tests needed by the changed code.
2. Refresh the replay-harness read in release mode.
3. Confirm replay parity plus either fewer exact-`nu` evaluations or lower
   measured replay time on the stored plateau fixtures.
4. Launch a new intended-profile rerun and stop it after `20` minutes max.
5. Compare the nearest stored step-`4` checkpoint to the current 20-minute
   target from `prefix-local-score-v1`.

### 4. Short-Loop Beat Rule

Treat a new slice as a real short-loop win only if its 20-minute stored read
beats the current target honestly.

Minimum comparison fields:

- `prefix_states_explored`
- `prefix_cache_groups`
- `prefix_cache_candidates`
- `frontier_queue_len`
- `observed_process_rss_bytes`
- `terminal_summary_build_millis`
- `terminal_summary_admissibility_checks`
- `terminal_summary_fallback_connectivity_checks`

Interpretation rule:

- More explored prefixes by `20` minutes is the main signal.
- Keep the retained-prefix story honest.
- Do not accept a "win" that only comes from waking fallback connectivity or
  admissibility work on what should still be no-miss surfaces.
- Do not expect the next few slices to beat the full `1095`-prefix stop yet.
  The immediate goal is repeated honest improvement over the `20` minute
  baseline.

### 5. When To Allow Another Long Read

Only allow a longer intended-profile continuation again after repeated
20-minute wins show that the lane has materially improved.

Until then:

- stop new intended-profile contenders at `20` minutes
- record their nearest stored 20-minute checkpoint
- keep `prefix-local-score-v1` as the long-run reference to beat

## Do Not Reopen First

- another long intended-profile rerun with no new code
- cached-summary reopen wake-up work
- contender-rank helper rewrites
- connectivity-first or cache-first rewrites
- deterministic batched parallel reduction
- broad metadata-only or bookkeeping-only cleanup
- another timing-only slice with no new runtime question

## Keep Or Branch Decision

- Stay on runtime work while the intended profile still stalls in step `4`.
- Keep `prefix-local-score-v1` as the current run to beat until a later slice
  beats its 20-minute stored read repeatedly and then eventually beats its
  longer stored continuation honestly.
- Treat the stopped `1095`-prefix read as the current long-run continuation
  reference, but treat the `20` minute checkpoint as the next validation gate.
- Branch to parity, breadth, compare, benchmark, and certification work only
  after later runtime slices move the intended profile far enough that longer
  reads become worth reopening.
