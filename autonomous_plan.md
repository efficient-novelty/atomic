# Autonomous Claim Lane Plan

Last updated: 2026-03-28
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
  showed aggregation first on the kept reopened surface, but the newest eager
  metadata rerun changed that read.
- The latest eager clause-metadata rerun
  `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
  preserved the honest short and reopened shapes, but it failed keep badly:
  wall clock and total `terminal_summary_build_*` regressed hard at
  `24/43/44/54`, it trailed both kept reopened references at `74/76`, and it
  moved the visible wall to clause filtering first.
- The lesson is now more specific: exact rank metadata is only promising if it
  is built lazily behind connectivity and admitted-candidate checks. Building
  structural/canonical clause metadata in terminal clause filtering is too
  early and swamps the later accept-rank savings.
- The next step should keep the current winning binary in code and retry only
  a narrower admitted-only metadata path rather than another eager
  clause-filter-wide metadata pack.

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

- land one narrower admitted-only metadata cut on the winning binary and
  re-earn the short read on the reopened surface without reintroducing a
  clause-filter wall

Working baselines:

- short:
  `runs/codex-claim-release-step4-kernel-aggregation-v1`
- full profile:
  `runs/codex-claim-release-full-kernel-aggregation-v1`
- late diagnostic:
  `runs/codex-claim-release-step4-kernel-late-profile-v1`

Preferred cuts:

- keep terminal clause filtering cheap
- if metadata is revisited, build it only after connectivity and admitted
  checks inside the summary kernel
- keep exact tie-break truth; do not swap it for a lossy hash or surrogate key
- preserve the numeric-first rank comparison idea only if it stays behind the
  incumbent-primary short-circuit

Reject as the next primary move:

- another connectivity-side rewrite
- another eager clause-filter-wide metadata rewrite
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
  references, and still looks good at `74/76`

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
