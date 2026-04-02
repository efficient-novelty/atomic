# Autonomous Claim Lane Plan

Last updated: 2026-04-02
Status: active

This file is the staged path from the current mixed claim-lane state to final
claim signoff. It is intentionally strategic and forward-looking rather than a
run diary.

## Objective

Produce one stored `desktop_claim_shadow` bundle from the disclosed desktop
that:

- finishes through step `15`
- preserves accepted-hash parity and honest breadth accounting
- passes compare, benchmark, and certification on stored evidence

Until that bundle exists, keep the paper wording at `bounded live recovery`.

## Current Strategic Position

- `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v3`
  proved that the current preserved claim binary can escape step `4`, reach
  step `13`, and open step `14`. The old "can this lane reach step `5`?"
  question is now closed on stored evidence.
- The same run failed at step `14` with
  `failure_note = "no atomic candidates were generated for step 14"`.
- Step `13` itself is not the new blocker. Its stored summary completed in
  `336 ms`, exact-screened `4353` terminals, closed `99%` of the frontier,
  and accepted a `kappa = 3`, `nu = 19` candidate.
- The accepted claim path is already drifting before the failure:
  steps `10` through `13` all diverge from reference replay, with widening
  `nu` and `clause_kappa` deltas by step `13`.
- Step `14` never entered real search on the failed run. Its only live
  checkpoint shows `clause_kappa = 7`,
  `raw_catalog_clause_widths = [3,1,1,1,1,1,1]`,
  `raw_catalog_telescope_count = 3`,
  `generated_raw_surface = 0`,
  `frontier_queue_len = 0`, and `candidate_pool_len = 0`.
- The new divergent-prefix reproducer plus current claim code now promote that
  same late-step opening into `claim_step_open = 9..9`, enqueue one root, and
  reach exact terminal-summary work, but the reproducer still dies with exact
  partial-prefix pruning after summary build starts.
- The compact terminal-summary path is still the main measured cost inside
  step `4`, but step-`4` throughput is no longer the immediate blocker for the
  next cycle. The immediate blocker is late-step claim viability and the
  step-`14` zero-candidate failure.
- Keep the current short-loop/runtime references for regression checks:
  - short-loop gate:
    `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`
  - later-wall step-`4` continuation reference through `576`:
    `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`
  - corroborating middle-wall read through `335`:
    `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v2`
  - older farthest stored step-`4` stop at `1095`:
    `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`

## Optimization Thesis

The next cycle should spend engineering time on late-step diagnosis and repair,
not on another long rerun first.

The highest-value work is:

1. explain why the promoted step-`14` Hilbert-band prefixes still all get
   `CannotClearBar` in exact partial-prefix screening after terminal summary
   begins
2. compare the compact terminal-summary bound against a direct exact
   assessment on the same divergent remaining-one prefixes
3. turn that surviving exact-screen failure into a regression-backed fix
   before spending another
   intended-profile rerun

The compact terminal-summary path should be optimized again only after late-step
viability is restored. Do not delete or bypass that summary path outright; it is
part of the claim-lane exact prune/materialization algorithm, not just
diagnostic scaffolding.

## Decision Rules

- Trust stored artifacts over terminal impressions.
- Treat `long-rerun-v3` as closing the old step-`4` escape question; do not
  launch another long rerun just to reconfirm that the lane can leave
  step `4`.
- Require claim-focused tests plus replay-harness parity before any new
  intended-profile rerun.
- Prefer narrow instrumentation and regression-backed fixes over broad frontier
  rewrites.
- Do not accept a "fix" that only hides the failure by waking guarded, replay,
  realistic-shadow, or demo-only fallback behavior.
- Keep the current short-loop gate and stored step-`4` continuation references
  as regression checks, not as the primary open question.

## Active Phase: Step-14 Failure Diagnosis And Repair

Goal:

- turn `no atomic candidates were generated for step 14` into a reproducible,
  test-backed bug and land the narrowest honest fix

Loop:

1. capture the late-step debt/band/package state from the finished claim
   history
2. add accounting between `claim_regular_clause_catalog` and the first
   frontier/candidate creation
3. reproduce the step-`14` failure in tests on the divergent claim history
4. patch the narrowest band-selection, clause-catalog, or root-screen path
   that restores non-zero step-`14` search
5. rerun targeted claim tests plus replay parity
6. only then spend a capped intended-profile rerun
7. only spend another full long rerun if the capped read stays honest

Current slice order:

1. emit claim debt axes, min/max `kappa`, late-family surface, historical
   anchor, and package flags at late step open
2. emit root-seeding counts after clause catalog construction:
   `roots_seen`, `roots_rejected_by_insert_root`,
   `roots_rejected_by_exact_screen`, and `roots_enqueued`
3. keep the divergent reproducer green with
   `claim_step_open = 9..9`, `claim_debt_axes = 7..7`,
   `roots_enqueued = 1`, and `remaining_one_algebraic_prunes = 0`
4. explain why the surviving promoted step-`14` prefixes still get exact
   partial-prefix `CannotClearBar`
5. revisit step-`4` compact-summary cost only after step-`14` viability is
   restored

Do not reopen first:

- another fresh long rerun
- more step-`4` summary-build micro-optimizations first
- dormant cached-summary reopen wake-up work
- broad frontier rewrites
- compare, benchmark, or certification work before the late-step repair lands

## Phase 2: Re-Earn One Full-Profile Claim Run

Goal:

- complete one intended-profile claim run through step `15` after the
  step-`14` failure is fixed

Required output:

- one canonical run directory from the disclosed desktop
- accepted parity through step `15`
- honest breadth evidence through the required floors
- complete reason and prune accounting

## Phase 3: Freeze Signoff Artifacts

Goal:

- turn the finished run into the auditable claim bundle

Required output:

- one compare report against the guarded reference
- one benchmark bundle
- one passing `claim_certificate.json`
- one certified runtime threshold tied to stored evidence

## Phase 4: Open The Language Gate

Goal:

- allow stronger wording only after the technical gate is closed

Required output:

- user-facing and paper-facing wording updated only after certification passes

## Non-Goals Until Step-14 Is Fixed

- another `v4` rerun first
- claiming step-`4` runtime is the current blocker
- timing-only slices with no new late-step explanation
- broad metadata-only cleanup
- stronger user-facing language

## Success Condition

This plan is done only when one stored `desktop_claim_shadow` bundle from the
disclosed desktop shows all of the following at the same time:

- full-profile completion through step `15`
- no silent guarded, replay, realistic-shadow, or demo-only fallback
- accepted parity through step `15`
- breadth gates passed honestly from stored evidence
- complete reason and prune accounting
- the step-`14` zero-candidate failure is gone on the winning claim path
- benchmark and manifest data sufficient for certification
- passing compare and certification outputs
