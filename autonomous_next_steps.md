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
- Keep the replay harness corpus and benchmark files:
  - `tests/fixtures/claim_runtime/remaining_one_plateau_fixtures.json`
  - `tests/fixtures/claim_runtime/remaining_one_plateau_benchmark.json`

## Current Read

- The current winner is the facts-only hit-path rerun, not the older
  prefix-`nu` context rerun.
- It preserved the honest retained-prefix story through the stored `576` read,
  beat `prefix-nu-context-v2` at every matched later checkpoint through `533`,
  and still did not reach step `5`.
- The later decisive surfaces are now no-miss surfaces on stored evidence:
  `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0`.
- Aggregation remains first, connectivity remains second, exact `nu` remains
  third, and terminal clause-filter handoff remains small.
- Cached-summary reopen remains dormant on decisive stored surfaces and is not
  the next honest move.
- The replay harness stays necessary, but the latest local benchmark is only a
  gate, not proof of keep on its own.

## Do This Next

### 1. Do Not Rerun First

Do not start another plain rerun first.
The next move is the next code slice on top of the current winner.

### 2. Use The Replay Harness As A Hard Gate

Before any new full-profile rerun, require all of the following:

- deterministic parity on the stored plateau fixtures
- either fewer exact-`nu` evaluations or lower measured aggregation time on
  the stored plateau fixtures
- no regression severe enough to erase the stored later-surface wall-clock win

If the slice fails that local gate, drop or revise it before spending another
full-profile run.

### 3. Implement The Next Slices In This Order

1. Make the local continuation cone the primary scoring object.
   Push cheap exact or exact-safe prefix-local interval pruning through
   `SingleClauseStructuralNuContext`,
   `TerminalClauseNuFacts`, and
   `structural_nu_single_clause_upper_bound`
   so more bar-clearability and overshoot decisions are made from the current
   prefix before terminal assembly.

2. Remove fact reconstruction from every winning-path evaluation.
   Treat `TerminalClauseNuFacts` as a mandatory clause-catalog sidecar on the
   hot path.
   Keep `structural_nu_with_clause_facts(...)` on the hot path and keep
   `structural_nu_with_clause(...)` off it.

3. Split the step-`4` remaining-one kernel into two kernels.
   - true no-miss hit-path plateau kernel
   - general fallback kernel

   The no-miss kernel should assume clause refs plus predecoded connectivity
   facts plus predecoded structural-`nu` facts and carry fewer branches and
   fewer generic checks.

4. Compress `lib_refs` inside `SingleClauseStructuralNuContext`.
   Preferred order:
   - inline small array for the common case
   - dense bitset after a threshold
   - sorted boxed slices only where serialization or debug parity needs them

5. If second-pass duplication still matters, prefer a tiny survivor sketch.
   Carry only the clause refs and facts needed for the best primary rank and
   tie-break-relevant survivors.
   Do not wake the dormant general cached-summary reopen machinery first.

### 4. Validation Loop For Each Slice

If a code slice lands, validate in this order:

1. Rerun only the claim-focused tests needed by the changed code.
2. Refresh the replay-harness read in release mode.
3. Confirm parity plus either lower aggregation time or fewer exact-`nu`
   evaluations on the stored plateau fixtures.
4. Start a new full-profile rerun only after that local gate passes.
5. On the full rerun, re-earn these regions in order:
   - `140/163`
   - `332/335`
   - `408/437/454/484`
   - then move materially past `576`
   - then chase `1038` or step `5`

## Do Not Reopen First

- another plain intended-profile rerun with no code or new runtime question
- cached-summary reopen wake-up work
- contender-rank helper rewrites
- connectivity-first or cache-first rewrites
- deterministic batched parallel reduction
- broad metadata-only or bookkeeping-only cleanup
- another timing-only slice with no new runtime question

## Keep Or Branch Decision

- Stay on runtime work while the intended profile still stalls in step `4`.
- Keep `structural-nu-facts-v1` as the current speed winner until a later
  slice beats it honestly through the stored `484/533/576` region.
- Keep `prefix-nu-context-v2` as the deeper continuation target until a later
  slice moves materially past `576` toward `1038` or reaches step `5`.
- Branch to parity, breadth, compare, benchmark, and certification work only
  after a later full-profile rerun reaches step `5` or materially surpasses
  the current step-`4` wall.
