# Autonomous Claim Lane Plan

Last updated: 2026-03-26
Status: active

This file is the staged path from the current claim-lane wall to final
signoff. It records the current strategy and decision rules, not every older
experiment.

## Objective

Produce one stored `desktop_claim_shadow` bundle from the disclosed desktop
that:

- finishes through step `15`
- preserves accepted-hash parity and honest breadth accounting
- passes compare, benchmark, and certification on stored evidence

Until that bundle exists, keep the paper wording at `bounded live recovery`.

## Strategic Diagnosis

- The claim lane has already crossed the old memory wall.
- The current binary is still compute-bound in step `4`.
- The honest live shape still repeats across stored short reruns:
  - the frontier queue drains along the same contour
  - legality growth stays on the same contour
  - the retained prefix cache still flattens at
    `39 groups / 144845 candidates`
- The landed wins so far were the cheap early ones:
  delayed materialization, the incumbent-primary remaining-one fast path, the
  one-pass `structural_nu` summary-build fast path, the algebraic `nu`
  ceiling, and now the family-agnostic claim terminal-admissibility shortcut.
- The indirect routes around the wall have now been exhausted on stored
  evidence:
  - context-equivalence quotienting found no live reuse
  - frontier-pop incumbent ordering never engaged on the live lane
  - proof-close handoff ordering never engaged on the live lane
  - post-plateau summary-skip only moved cost into direct materialization
  - the narrower post-plateau materialize gate never honestly opened
  - post-plateau summary-cache reuse activated on the plateau, but rebuild
    elisions stayed at `0`
- The kernel split rerun
  `runs/codex-claim-release-step4-kernel-profile-v2` then narrowed the wall
  further: on the honest plateau at `prefix_states_explored = 44`,
  admissibility was the dominant measured sub-phase
  (`679889 ms`), ahead of connectivity (`492575 ms`), aggregation
  (`118953 ms`), and exact `nu` (`74386 ms`).
- The follow-up keep rerun
  `runs/codex-claim-release-step4-kernel-admissibility-v1` widened the existing
  terminal-summary admissibility shortcut onto the family-agnostic claim lane
  and removed that cost on the same plateau:
  - `elapsed_millis` fell to `1398528` from algebraic `1662758`
  - `terminal_summary_build_millis` fell to `1292019` from algebraic
    `1555470`
  - `terminal_summary_admissibility_millis` fell to `0` from diagnostic
    `679889`
- So the honest wall has moved again, but not out of the same kernel:
  remaining-one connectivity is now the dominant plateau cost, aggregation is
  second, and exact `nu` is no longer the first target.

## Strategic Rules

- Trust stored artifacts over terminal impressions.
- Treat "same honest shape, slower clock" as overhead, not progress.
- Prefer exact structural cuts over broad heuristic rewrites.
- Require telemetry for any newly claimed activation surface.
- Retire a hypothesis after one honest stored rerun if it shows
  non-engagement, zero reuse, or a pure cost shift.
- Keep step-`4` work narrow until a rerun proves the wall has moved again.
- Do not replace the short baseline unless the new rerun keeps the same honest
  plateau shape and improves matched checkpoints.

## Current Phase

Current short baseline:
`runs/codex-claim-release-step4-kernel-admissibility-v1`

Previous short baseline:
`runs/codex-claim-release-step4-algebraic-v1`

Current full-profile baseline:
`runs/codex-claim-release-full-nu-profile-v1`

Current honest wall:

- remaining-one connectivity on the retained `39/144845` plateau
- then aggregation and bound/rank bookkeeping

## Execution Order

### Phase 1. Make The Step-`4` Kernel Measurable

Status:

- done

Stored evidence:

- diagnostic split:
  `runs/codex-claim-release-step4-kernel-profile-v2`
- keep rerun using that read:
  `runs/codex-claim-release-step4-kernel-admissibility-v1`

What this phase proved:

- the dominant plateau cost was real kernel admissibility work, not a cache or
  ordering issue
- the family-agnostic claim lane could reuse terminal-summary admissibility
  exactly even with `family_filters = 0`
- removing that work improved the honest plateau without reopening the old
  materialize blowup

### Phase 2. Reduce Work Before Or Inside Connectivity

Goal:

- cut the new dominant plateau cost without changing retained-prefix honesty

Preferred patches:

- cheaper cached structural connectivity decisions
- one exact-preserving disconnection prune moved earlier
- less repeated summary-extension churn in `terminal_connectivity`

Reject as primary moves:

- another admissibility-focused patch
- another exact-`nu` first optimization
- another ordering, reuse, or post-plateau direct-materialize variant

Done when:

- the stored rerun shows lower connectivity-side cost and lower
  `terminal_summary_build_millis` on matched plateau checkpoints

### Phase 3. Cut Aggregation And Residual Exact Work

Goal:

- shrink the next most expensive remaining kernel work after connectivity

Preferred patches:

- less bound merge or rank aggregation churn
- less evaluation-record churn inside the compact summary loop
- only then, if connectivity is already flat, a smaller exact-`nu` cleanup

Done when:

- `terminal_summary_build_millis` falls again at matched plateau checkpoints
  while `terminal_materialize_millis` stays controlled and winner determinism
  remains intact

### Phase 4. Re-Earn The Real Full-Profile Read

Goal:

- prove that the winning short step-`4` sequence helps on the real
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

## Non-Goals Until The Step-`4` Wall Moves Again

- reopening allocator or memory-compaction work first
- broad frontier rewrites
- widening-band retuning
- more ordering, cache-reuse, or post-plateau summary-skip variants
- compare, benchmark, or certification work
- stronger user-facing language

## Baselines And Informative Evidence

- Current short step-`4` baseline:
  `runs/codex-claim-release-step4-kernel-admissibility-v1`
- Previous short step-`4` baseline:
  `runs/codex-claim-release-step4-algebraic-v1`
- Current full-profile baseline:
  `runs/codex-claim-release-full-nu-profile-v1`
- Diagnostic kernel split:
  `runs/codex-claim-release-step4-kernel-profile-v2`
- Ignore as invalid diagnostic:
  `runs/codex-claim-release-step4-kernel-profile-v1`
- Informative failed short reruns that define the current diagnosis:
  - `runs/codex-claim-release-step4-context-equivalence-v1`
  - `runs/codex-claim-release-step4-incumbent-ordering-v1`
  - `runs/codex-claim-release-step4-local-two-step-order-v2`
  - `runs/codex-claim-release-step4-proof-close-handoff-v1`
  - `runs/codex-claim-release-step4-post-plateau-v1`
  - `runs/codex-claim-release-step4-post-plateau-materialize-v1`
  - `runs/codex-claim-release-step4-post-plateau-summary-cache-v3`

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
