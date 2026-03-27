# Autonomous Claim Lane Plan

Last updated: 2026-03-27
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
  ceiling, the family-agnostic claim terminal-admissibility shortcut, the
  exact non-allocating connectivity summary scan, and the terminal-only cached
  parent connectivity decision.
- The indirect routes around the wall have now been exhausted on stored
  evidence:
  - context-equivalence quotienting found no live reuse
  - frontier-pop incumbent ordering never engaged on the live lane
  - proof-close handoff ordering never engaged on the live lane
  - post-plateau summary-skip only moved cost into direct materialization
  - the narrower post-plateau materialize gate never honestly opened
  - post-plateau summary-cache reuse activated on the plateau, but rebuild
    elisions stayed at `0`
  - the expr-keyed terminal clause hot-path cache regressed
  - the clause-side connectivity profile precompute regressed
  - the terminal-candidate tuple remap cut regressed
- The diagnostic split rerun
  `runs/codex-claim-release-step4-kernel-profile-v2` explained the live
  surface:
  on the honest plateau at `prefix_states_explored = 44`, admissibility was
  the first old wall, followed by connectivity, then aggregation, then exact
  `nu`.
- The follow-up keeps in
  `runs/codex-claim-release-step4-kernel-admissibility-v1`,
  `runs/codex-claim-release-step4-kernel-connectivity-v1`, and
  `runs/codex-claim-release-step4-kernel-connectivity-v2` removed that old
  admissibility wall and then cut the retained connectivity loop twice.
- A newer kept rerun
  `runs/codex-claim-release-step4-kernel-aggregation-v1` then stopped
  building full `AcceptRank` values for bar-clearing claim candidates once a
  strictly better primary `(overshoot, kappa)` rank had already been seen in
  the same summary group.
- That rerun kept the same honest plateau and improved the matched checkpoints
  against `runs/codex-claim-release-step4-kernel-connectivity-v2`:
  - `elapsed_millis = 549630/990480/1012067` instead of
    `551825/998555/1020529`
  - `terminal_summary_build_millis = 492524/892772/912271` instead of
    `495256/901994/921924`
  - `terminal_summary_connectivity_millis = 88989/164940/169227` instead of
    `95969/178000/182453`
  - aggregation also fell slightly to `67567/118700/120643`
- The latest dropped rerun
  `runs/codex-claim-release-step4-terminal-candidate-prep-v1` then added one
  narrow pre-summary candidate-preparation counter and tried one narrow cut in
  the filtered-candidate remap path.
- That rerun kept the same honest plateau and showed:
  - `terminal_summary_candidate_prep_millis = 32904/71577/73974`
    at `24/43/44`
  - but `elapsed_millis = 562457/1017859/1040469` instead of
    baseline `551825/998555/1020529`
  - and `terminal_summary_build_millis = 505516/918924/939406` instead of
    baseline `495256/901994/921924`
- So the new read is now explicit:
  terminal candidate preparation is real and measurable, but this particular
  prep path is not the next keep target.
- The current honest short wall on the kept baseline still sits on the
  already-measured counters:
  connectivity first, aggregation second, then exact `nu`.

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
`runs/codex-claim-release-step4-kernel-aggregation-v1`

Previous short baseline:
`runs/codex-claim-release-step4-kernel-connectivity-v2`

Current full-profile baseline:
`runs/codex-claim-release-full-nu-profile-v1`

Current honest wall:

- measured remaining-one connectivity on the retained `39/144845` plateau
- then aggregation and bound or rank bookkeeping inside the compact summary
  loop
- then exact `nu`

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

### Phase 2. Re-Enter Measured Connectivity Or Aggregation Cuts

Status:

- done via `runs/codex-claim-release-step4-kernel-aggregation-v1`

Goal:

- use the new prep read, keep the failed prep patch dropped, and land one
  narrow connectivity-side or aggregation-side cut that improves the kept
  baseline honestly

Preferred patches:

- stay inside the already-measured `terminal_connectivity` path and cut work
  there without reviving clause-side profile precompute beside the candidate
  list
- or cut bound merge, acceptance-rank bookkeeping, or evaluation-record churn
  inside the admitted summary loop
- only if connectivity and aggregation are both already flat on the next
  candidate, a smaller exact-`nu` cleanup

Reject as primary moves:

- another blind pre-summary candidate-preparation rewrite
- reviving `runs/codex-claim-release-step4-kernel-connectivity-v4`
  in any equivalent profile-precompute shape
- another expr-keyed `HashMap` or `BTreeMap` hot-path cache
- another admissibility-focused patch
- another ordering, reuse, or post-plateau direct-materialize variant

Done when:

- the stored rerun lowers `terminal_summary_build_millis` and wall clock at the
  matched `24/43/44` plateau checkpoints against
  `runs/codex-claim-release-step4-kernel-connectivity-v2`

### Phase 3. Re-Earn The Real Full-Profile Read

Goal:

- prove that the winning short step-`4` sequence helps on the real
  `desktop_claim_shadow_1h` profile

Required output:

- one new stored intended-profile rerun
- a read of its full-profile artifacts, especially `step-04-live.ndjson`

Done when:

- the full-profile rerun either moves materially farther or exposes a later
  blocker honestly

### Phase 4. Finish A Full Claim Bundle

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

### Phase 5. Freeze Signoff Artifacts

Goal:

- turn the finished run into the repo's auditable claim bundle

Required output:

- one compare report against the guarded reference
- one benchmark bundle
- one passing `claim_certificate.json`
- one certified runtime threshold tied to stored evidence

Done when:

- another reviewer can audit the full claim from stored artifacts alone

### Phase 6. Open The Language Gate

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
  `runs/codex-claim-release-step4-kernel-aggregation-v1`
- Previous short step-`4` baseline:
  `runs/codex-claim-release-step4-kernel-connectivity-v2`
- Current full-profile baseline:
  `runs/codex-claim-release-full-nu-profile-v1`
- Diagnostic kernel split:
  `runs/codex-claim-release-step4-kernel-profile-v2`
- Ignore as invalid diagnostic:
  `runs/codex-claim-release-step4-kernel-profile-v1`
- Informative failed short reruns that define the current diagnosis:
  - `runs/codex-claim-release-step4-terminal-candidate-prep-v1`
  - `runs/codex-claim-release-step4-kernel-connectivity-v4`
  - `runs/codex-claim-release-step4-kernel-connectivity-v3`
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
