# Autonomous Claim Lane Plan

Last updated: 2026-03-30
Status: active

This file is the staged path from the current step-`4` wall to final claim
signoff. It is intentionally forward-looking and operational rather than a
full experiment diary.

## Objective

Produce one stored `desktop_claim_shadow` bundle from the disclosed desktop
that:

- finishes through step `15`
- preserves accepted-hash parity and honest breadth accounting
- passes compare, benchmark, and certification on stored evidence

Until that bundle exists, keep the paper wording at `bounded live recovery`.

## Current Strategy

- The claim lane has already crossed the old early RSS wall.
- The intended profile is still blocked inside step `4`.
- The current short baseline
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
  keeps the honest `39/144845` retained-prefix shape through `24/43/44/54`
  and the honest reopened `40/147639` surface at `74/76`.
- The current full-profile runtime reference
  `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
  preserved the honest `39/144845`, `40/147639`, and `41/154842` story,
  re-earned `42/157636` and `43/160430`, and moved the stored
  step-`4` wall from `408` to `437`.
- That new full-profile reference materially improved every decisive stored
  checkpoint through the previous stored `408` wall, then exposed a new later
  blocker honestly: on the real intended profile, aggregation still leads,
  connectivity is second, exact `nu` is third, and terminal clause-filter
  handoff remains tiny on the `41/42/43` surface.
- Observed RSS on the new reference rose to about `1.44 GiB` by the stored
  `437` read, materially above the previous runtime reference but still far
  below the old allocator-failure band, so the blocker remains later step-`4`
  throughput rather than allocator pressure.
- The next step should therefore keep the short winner and current full-profile
  runtime reference in code, leave the dropped metadata, reopened-connectivity,
  clause-load-only, bookkeeping/bound-only, admitted-kernel-only, and
  connectivity-retry moves out of code as standalone next moves, and land one
  narrow later-surface runtime cut before reopening another plain rerun-only
  turn.

## Decision Rules

- Trust stored artifacts over terminal impressions.
- Treat "same honest shape, slower clock" as regression, not progress.
- Replace the short baseline only if the new rerun:
  - keeps the same honest shape at `24/43/44/54`
  - improves wall clock and `terminal_summary_build_*` at those checkpoints
  - and does not lose the reopened `74/76` read honestly
- Replace the full-profile runtime reference only if the new rerun:
  - preserves the honest retained-prefix story already visible on the current
    runtime reference
  - moves materially past the current stored `437` wall or reaches step `5`
  - and keeps RSS well below the old allocator-failure band
- Retire a hypothesis after one honest stored rerun if it shows:
  - non-engagement
  - a pure cost shift
  - or a clear regression against the kept baseline
- Do not branch to compare, benchmark, certification, or stronger language
  before step `4` moves or a full-profile run finishes.

## Active Phase: Move The Later Aggregation Wall

Goal:

- cut the post-`140` aggregation-side step-`4` wall on the current winner on
  the real `desktop_claim_shadow_1h` profile

Working baselines:

- short:
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- full-profile runtime reference:
  `runs/codex-claim-release-full-aggregation-open-band-compact-v1`
- previous full-profile runtime reference:
  `runs/codex-claim-release-full-connectivity-facts-v1`
- broader full-profile baseline:
  `runs/codex-claim-release-full-kernel-aggregation-v1`
- late diagnostic:
  `runs/codex-claim-release-step4-kernel-late-profile-v1`

Required output:

- one narrow later-surface aggregation-side code slice on the current winner
- one new stored full-profile rerun on that slice with live checkpoint
  persistence through at least the `140/163/228` region and ideally through the
  current `42/43` reopen
- a read of its `step-04-live.ndjson`, `run.json`, and `reports/latest.txt`
- `step-05-live.ndjson` too if the rerun reaches step `5`

Reject as the next primary move:

- another plain intended-profile rerun with no code or new runtime question
- another connectivity-first slice before a new aggregation-side read exists
- another eager clause-filter-wide metadata rewrite
- another lazy admitted-only metadata rewrite
- another unchanged reopened connectivity reuse retry
- another narrow clause-load-only replay
- another narrow bookkeeping/bound-only replay
- another admitted-kernel-only replay
- another exact-`nu` cleanup first
- another diagnostic-only slice first
- another retry of `kernel-rank-bookkeeping-v1`
- another retry of `kernel-bound-merge-v1`
- another retry of `kernel-lazy-acceptrank-v1`
- another retry of `kernel-summary-batching-v1`
- another retry of `kernel-summary-bookkeeping-v1`
- another retry of `kernel-competition-hoist-v1`
- another retry of `kernel-nu-highwater-v1`
- another retry of `kernel-summary-threshold-v1`
- reopening old ordering, reuse, cache, or post-plateau variants

Done when:

- the next aggregation-side rerun either:
  - materially reduces the post-`140` aggregation wall
  - exposes a new later blocker honestly
  - or finishes through step `15`

## Phase 2: Re-Earn The New Runtime Reference

Goal:

- prove that the aggregation-side slice moves the current later step-`4` wall
  on the real `desktop_claim_shadow_1h` profile

Required output:

- one new stored full-profile rerun on the aggregation-side slice
- a read of its `step-04-live.ndjson`, `run.json`, and `reports/latest.txt`

Done when:

- the run either moves materially past
  `runs/codex-claim-release-full-aggregation-open-band-compact-v1`, exposes a
  later blocker honestly, or finishes through step `15`

## Phase 3: Finish A Full Claim Bundle

Goal:

- complete one intended-profile claim run through step `15`

Required output:

- one canonical run directory from the disclosed desktop
- accepted parity through step `15`
- honest breadth evidence through the required floors
- complete reason and prune accounting

## Phase 4: Freeze Signoff Artifacts

Goal:

- turn the finished run into the auditable claim bundle

Required output:

- one compare report against the guarded reference
- one benchmark bundle
- one passing `claim_certificate.json`
- one certified runtime threshold tied to stored evidence

## Phase 5: Open The Language Gate

Goal:

- allow stronger wording only after the technical gate is closed

Required output:

- user-facing and paper-facing wording updated only after certification passes

## Non-Goals Until The Step-`4` Wall Moves

- reopening allocator or memory-compaction work first
- broad frontier rewrites
- widening-band retuning
- more ordering, cache-reuse, or summary-skip variants
- compare, benchmark, or certification work before a runtime winner exists
- stronger user-facing language

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
