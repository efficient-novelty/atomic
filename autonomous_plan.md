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
  `runs/codex-claim-release-step4-kernel-aggregation-v1`
  keeps the honest `39 groups / 144845 candidates` retained-prefix shape
  through `24/43/44/54`.
- The current full-profile baseline
  `runs/codex-claim-release-full-kernel-aggregation-v1`
  proves that the intended profile later reopens to `40/147639` at `74` and
  `41/154842` at `140`, while still never reaching step `5`.
- The current late diagnostic
  `runs/codex-claim-release-step4-kernel-late-profile-v1`
  showed aggregation first on the kept reopened surface.
- The eager clause-metadata rerun
  `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
  preserved the honest short and reopened shapes, but it failed keep badly:
  wall clock and total `terminal_summary_build_*` regressed hard at
  `24/43/44/54`, it trailed both kept reopened references at `74/76`, and it
  moved the visible wall to clause filtering first.
- The newer lazy admitted-only metadata rerun
  `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`
  re-earned cheap clause filtering and kept the honest early and reopened
  shapes, but it still failed keep on the matched early short surface because
  `terminal_summary_build_*` regressed by about `10-11%`.
- The newer reopened connectivity rerun
  `runs/codex-claim-release-step4-kernel-reopened-connectivity-v1`
  then kept the honest early and reopened shapes, cut reopened connectivity
  timing materially, and improved elapsed wall clock at `24/43/44/54/74/76`,
  but it still failed keep because `terminal_summary_build_*` regressed by
  about `5.2-5.6%` on the matched early short surface and about `4.0%` at
  `74/76` versus the kept full-profile baseline.
- That newest rerun changed the honest reopened read again: at `76`, the
  stored bucket order became aggregation first, clause filtering second,
  connectivity third, and exact `nu` fourth.
- The newer direct bound/bookkeeping rerun
  `runs/codex-claim-release-step4-kernel-bound-bookkeeping-v1`
  then preserved the same honest early plateau at `24` and kept aggregation
  first, connectivity second, clause filtering third, and exact `nu` fourth,
  but it still failed keep because the matched early short surface read
  `549708 / 544700` instead of the kept `549630 / 492524`.
- The newer admitted-kernel rerun
  `runs/codex-claim-release-step4-kernel-admitted-kernel-v1`
  then preserved the same honest early plateau at `24/25`, materially
  improved elapsed at the matched `24` checkpoint, and lowered the measured
  aggregation bucket there, but it still failed keep because
  `terminal_summary_build_*` read `514192` instead of the kept `492524` while
  the broad early bucket order moved to connectivity first, aggregation
  second, clause filtering third, and exact `nu` fourth.
- The next step should therefore keep the current winning binary in code,
  leave both metadata retries, the dropped connectivity reuse, and the
  admitted-kernel-only replay out of code, and target one broader claim
  open-band handoff cut rather than another metadata, connectivity,
  clause-load-only, bookkeeping/bound-only, or admitted-kernel-only pass.

## Decision Rules

- Trust stored artifacts over terminal impressions.
- Treat "same honest shape, slower clock" as regression, not progress.
- Replace the short baseline only if the new rerun:
  - keeps the same honest shape at `24/43/44/54`
  - improves wall clock and `terminal_summary_build_*` at those checkpoints
  - and does not lose the reopened `74/76` read honestly
- Retire a hypothesis after one honest stored rerun if it shows:
  - non-engagement
  - a pure cost shift
  - or a clear regression against the kept baseline
- Do not branch to compare, benchmark, certification, or stronger language
  before step `4` moves or a full-profile run finishes.

## Active Phase: Break The Step-`4` Wall

Goal:

- land one broader compound claim open-band handoff cut on the winning binary
  and re-earn the short read on the reopened surface without reintroducing a
  clause-filter wall or metadata cost on the early surface

Working baselines:

- short:
  `runs/codex-claim-release-step4-kernel-aggregation-v1`
- full profile:
  `runs/codex-claim-release-full-kernel-aggregation-v1`
- late diagnostic:
  `runs/codex-claim-release-step4-kernel-late-profile-v1`

Preferred cuts:

- keep terminal clause filtering cheap
- keep the current exact tie-break truth surface intact
- keep the admitted-kernel aggregation win while lowering the newer
  connectivity / aggregation / clause-filter composite
- keep aggregation, connectivity, clause filtering, and exact `nu` separately
  readable in stored evidence

Reject as the next primary move:

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

- one new short rerun lowers matched `24/43/44/54` wall clock and
  `terminal_summary_build_*` honestly, keeps clause filtering near the kept
  references, and improves the reopened `74/76` read against the kept
  full-profile baseline

## Phase 2: Re-Earn The Intended-Profile Read

Goal:

- prove that the new short winner helps on the real
  `desktop_claim_shadow_1h` profile

Required output:

- one new stored full-profile rerun
- a read of its `step-04-live.ndjson`, `run.json`, and `reports/latest.txt`

Done when:

- the run either moves materially farther, exposes a later blocker honestly,
  or finishes through step `15`

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
