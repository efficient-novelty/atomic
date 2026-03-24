# Autonomous Claim Lane Plan

Last updated: 2026-03-24
Status: active

This file is the staged path from the current compute-bound claim-lane wall to
final signoff. It intentionally omits older rollout history that no longer
changes the next decisions.

## Objective

Produce one stored `desktop_claim_shadow` bundle from the disclosed desktop
that:

- finishes through step `15`
- preserves accepted-hash parity and honest breadth accounting
- passes compare, benchmark, and certification on stored evidence

Until that bundle exists, keep the paper wording at `bounded live recovery`.

## Current Operating Read

- The claim lane has already crossed the old memory wall.
- The current binary is now stably compute-bound in step `4`.
- The current full-profile iteration baseline is
  `runs/codex-claim-release-full-nu-profile-v1`.
- Delayed materialization, the incumbent-primary remaining-one fast path, and
  the one-pass `structural_nu` summary build fast path are all baseline.
- The hot cost is still remaining-one exact terminal-summary construction.
- The high pre-materialize rank-prune count is good news: pruning works, but it
  still triggers too late.
- The next meaningful wins must happen before
  `compute_terminal_prefix_completion_summary_from_candidates`, not in another
  memory-first rewrite.

## Execution Order

### Phase 1. Shift Step-`4` Pruning Ahead Of Exact Summary Build

Goal:

- kill more doomed remaining-one prefixes in `O(1)` structural time before the
  hot exact summary builder runs

Preferred first patches:

- algebraic `nu` ceiling derived from the current admissibility caps
- deterministic symmetry breaking for independent sibling clauses
- if still narrow and low risk, a structural-unity forced-bridge prune

Required output:

- one narrow search-code patch or tightly coupled patch pair
- new telemetry counters for the new prune surface
- one stored release rerun with `until_step = 4`
- a stored artifact comparison against the current short/full baselines

Done when:

- the rerun shows a real stored win in step-`4` summary-side telemetry without
  weakening retained-prefix honesty

### Phase 2. Collapse Duplicate Remaining-One States If Phase 1 Is Not Enough

Goal:

- stop paying exact-summary cost once per AST prefix when the surviving work is
  really a small number of repeated typing-context shapes

Preferred patch:

- context-equivalence quotienting keyed by normalized exported type context
  rather than exact prefix AST history

Required output:

- one scoped cache-key experiment
- one stored release rerun with `until_step = 4`
- evidence that legality/summary counts collapse materially without parity or
  honesty regressions

Done when:

- the rerun shows materially fewer repeated exact summary builds at matching
  checkpoints

### Phase 3. Improve Incumbent Arrival Only If Structural Culling Still Leaves Waste

Goal:

- find the winning dense `Pi`/`Sigma` package earlier so the incumbent rank
  rises sooner and structural ceilings can prune more of the queue

Preferred patch:

- deterministic incumbent-first or density-biased frontier ordering that keeps
  the exact lane honest

Required output:

- one narrow queue-order experiment
- one stored release rerun with `until_step = 4`

Done when:

- the rerun finds the same honest winner earlier and that earlier incumbent
  materially reduces the surviving exact-summary load

### Phase 4. Re-Earn The Real Full-Profile Read

Goal:

- prove the winning step-`4` patch helps on the real
  `desktop_claim_shadow_1h` profile

Required output:

- one new stored intended-profile rerun
- a read of its full-profile artifacts, especially `step-04-live.ndjson`

Done when:

- the full-profile rerun either moves materially farther or exposes a later
  blocker honestly

### Phase 5. Finish A Full Claim Bundle

Goal:

- complete one intended-profile run through step `15`

Required output:

- one canonical claim-lane run directory from the disclosed desktop
- accepted parity through step `15`
- honest breadth evidence through the required floors
- complete reason and prune accounting

Done when:

- one stored run finishes through step `15` and still satisfies the claim-lane
  honesty boundary

### Phase 6. Freeze Signoff Artifacts

Goal:

- turn the finished run into the repo's auditable claim bundle

Required output:

- one compare report against the guarded reference
- one benchmark bundle
- one passing `claim_certificate.json`
- one certified runtime threshold tied to stored evidence

Done when:

- another reviewer can audit the full claim from stored artifacts alone

### Phase 7. Open The Language Gate

Goal:

- allow stronger wording only after the technical gate is closed

Required output:

- user-facing and paper-facing wording updated only after certification passes

Done when:

- the stronger sentence is explicitly tied to the stored claim certificate and
  disclosed desktop bundle

## Non-Goals Until Phase 1 Wins

- reopening allocator or memory-compaction work first
- broad frontier rewrites
- widening-band retuning
- compare, benchmark, or certification work
- stronger user-facing language

## Baselines To Compare Against

- `runs/codex-claim-release-full-v1a`
- `runs/codex-claim-release-full-delayed-summary-v1`
- `runs/codex-claim-release-full-primary-rank-v1`
- `runs/codex-claim-release-full-nu-profile-v1`
- `runs/codex-claim-release-step4-delayed-summary-v1`
- `runs/codex-claim-release-step4-primary-rank-v1`
- `runs/codex-claim-release-step4-nu-profile-v1`
- `runs/codex-claim-release-step4-telemetry-v1`

## Success Condition

This plan is done only when one stored `desktop_claim_shadow` bundle from the
disclosed desktop shows all of the following at the same time:

- full-profile completion through step `15`
- no silent guarded, replay, realistic-shadow, or demo-only fallback
- accepted parity through step `15`
- breadth gates passed honestly from stored evidence
- complete reason and prune accounting
- the step-`4` compute wall is no longer the blocker on the intended profile
- benchmark and manifest data sufficient for certification
- passing compare and certification outputs
