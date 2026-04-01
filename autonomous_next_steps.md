# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-01

This note is the exact next work order for `desktop_claim_shadow`.

## Keep Fixed

- Keep the current short baseline:
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Keep the current full-profile speed winner to beat:
  `runs/codex-claim-release-full-aggregation-open-band-structural-nu-facts-v1`
- Keep the current deeper continuation target:
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`
- Keep the current in-flight contender under direct read:
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
- Keep the replay harness corpus and benchmark files:
  - `tests/fixtures/claim_runtime/remaining_one_plateau_fixtures.json`
  - `tests/fixtures/claim_runtime/remaining_one_plateau_benchmark.json`

## Current Read

- The first continuation-cone slice is now landed on top of the current
  winner.
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
- A new full-profile rerun is now in flight:
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
  `run.json` currently reports `status = "running"`, `completed_step = 3`,
  and `active_step = 4`.
- The current full-profile speed winner is still
  `runs/codex-claim-release-full-aggregation-open-band-structural-nu-facts-v1`
  until the in-flight rerun re-earns the later stored checkpoints honestly.

## Do This Next

### 1. Read The In-Flight Full Rerun First

Do not open another code slice first while
`prefix-local-score-v1`
is the live read.

Inspect its stored step-`4` checkpoints in this order:

- `140/163`
- `332/335`
- `408/437/454/484`
- `533/576`
- then push toward `1038` or step `5`

### 2. Promotion Rule

Replace the current speed winner only if
`prefix-local-score-v1`

- preserves the honest retained-prefix story
- beats `structural-nu-facts-v1` honestly through the stored
  `484/533/576` region
- and does not reintroduce the old allocator-failure band

Replace the deeper continuation target only if the same run keeps the speed win
through the current winner's region and then moves materially past `576`
toward `1038` or reaches step `5`.

### 3. If The In-Flight Rerun Stalls, Open The Next Code Slice In This Order

1. Make `TerminalClauseNuFacts` a mandatory clause-catalog sidecar on every
   winning-path remaining-one evaluation.
   Keep `structural_nu_with_clause_facts(...)` on the hot path and keep
   `structural_nu_with_clause(...)` off it.

2. Split the step-`4` remaining-one kernel into two kernels.
   - true no-miss hit-path plateau kernel
   - general fallback kernel

3. Compress `lib_refs` inside `SingleClauseStructuralNuContext`.
   Preferred order:
   - inline small array for the common case
   - dense bitset after a threshold
   - sorted boxed slices only where serialization or debug parity needs them

4. If second-pass duplication still matters, prefer a tiny survivor sketch.
   Carry only the clause refs and facts needed for the best primary rank and
   tie-break-relevant survivors.
   Do not wake the dormant general cached-summary reopen machinery first.

### 4. Validation Loop For Any Further Code Slice

If another code slice lands after the in-flight rerun reads out, validate in
this order:

1. Rerun only the claim-focused tests needed by the changed code.
2. Refresh the replay-harness read in release mode.
3. Confirm parity plus either fewer exact-`nu` evaluations or lower measured
   replay time on the stored plateau fixtures.
4. Launch the next full-profile rerun only after that local gate passes.
5. Re-earn the later checkpoints in the same order:
   - `140/163`
   - `332/335`
   - `408/437/454/484`
   - `533/576`
   - then `1038` or step `5`

## Do Not Reopen First

- another fresh rerun while `prefix-local-score-v1` is still the live read
- cached-summary reopen wake-up work
- contender-rank helper rewrites
- connectivity-first or cache-first rewrites
- deterministic batched parallel reduction
- broad metadata-only or bookkeeping-only cleanup
- another timing-only slice with no new runtime question

## Keep Or Branch Decision

- Stay on runtime work while the intended profile still stalls in step `4`.
- Keep `structural-nu-facts-v1` as the current speed winner until the in-flight
  rerun or a later contender beats it honestly through the stored
  `484/533/576` region.
- Keep `prefix-nu-context-v2` as the deeper continuation target until a later
  full rerun moves materially past `576` toward `1038` or reaches step `5`.
- Keep `prefix-local-score-v1` as the active read until its stored artifacts
  say honest win, honest loss, or non-engagement.
- Branch to parity, breadth, compare, benchmark, and certification work only
  after a later full-profile rerun reaches step `5` or materially surpasses
  the current step-`4` wall.
