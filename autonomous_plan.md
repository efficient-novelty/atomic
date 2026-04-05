# Autonomous Claim Lane Plan

Last updated: 2026-04-05
Status: active

This file is the staged path from the current parity-clean-but-not-certified
claim-lane state to final signoff. It is strategic rather than diary-like.

## Objective

Produce one stored `desktop_claim_shadow` bundle from the disclosed desktop
that:

- finishes through step `15`
- preserves accepted-hash parity and honest breadth accounting
- passes compare, benchmark, and certification on stored evidence

Until that bundle exists, keep the paper wording at `bounded live recovery`.

## Current Strategic Position

- The current canonical stored claim bundle is clean-tree completed `v10`:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v10`
  on repo head `6939b71063e4aec9598b293560c0c233981bc169` with release binary
  hash `d003dd29c599ba86b7ef410b1c7849f89a0fe45d33dc42508e368f4c2a7c473a`.
- Stored compare, certification, and benchmark outputs now exist beside that
  `v10` run and are the current authoritative evidence surface.
- The stored `v10` certificate plus the frozen `step-15-live.ndjson`
  provenance are now also pinned by
  `stored_claim_v10_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`,
  so the current canonical breadth diagnosis is executable in-tree.
- Stored accepted-hash parity is earned on `v10` through step `15`.
- Runtime, manifest completeness, fallback honesty, narrative/event
  completeness, exact-screen reason completeness, and prune-class completeness
  are all earned on stored evidence.
- Stored step `13` is now re-earned on the canonical chain:
  - the stored claim-open surface now widens to
    `[5,1,3,3,5,3,2]` / raw `1350` / generated `2320`
  - the guarded step-`13` metric shell stays accepted there and keeps the
    canonical `62 / 9 / 12027 -> 103 / 8 / 1794` continuation through
    steps `14` and `15`
  - the stored step-`13` surface now seeds `5` roots and pays
    `576` legality/connectivity exact rejections plus `401` heuristic drops
    before proof-close
- The remaining blockers are stored breadth misses on the canonical chain:
  step `1 = 546 / 2144` and step `15 = 1794 / 5000`.
- Step `15` is now the remaining stored late-floor miss, and the next honest
  local engineering dollar is stored step-`15` diagnosis / repair on top of
  `v10`, not another rerun first and not another step-`13` theory pass.
- Two new non-landed local step-`13` probes now sharpen the safe search space:
  - widening only operator-bundle formation positions `1` and `4` to
    demo-like variants lifts the repaired local read to
    `[3,5,3,3,5,1,1]` / raw `675` / generated `2223`, but it changes the
    accepted late path to `45 / 7 -> 61 / 9`
  - that first unsafe position-`1` / position-`4` reland is now frozen as an
    executable negative-control regression on the repaired step-`12` chain:
    the mixed custom step-`13` catalog reproduces raw widths
    `[3,5,3,3,5,1,1]`, raw `675`, live generated `2223`, accepted
    `45 / 7`, and the shifted repaired step-`14` winner profile `61 / 9`
  - that second unsafe position-`0` / position-`4` / position-`5` /
    position-`6` reland is now also frozen as an executable negative-control
    regression on the repaired step-`12` chain:
    the mixed custom step-`13` catalog uses the full demo slices at positions
    `0` and `4` plus the original three-option demo slice at positions `5`
    and `6`, reproducing raw widths `[5,1,3,3,5,3,3]`, raw `2025`,
    live generated `2995`, accepted `46 / 7`, and the guarded accepted hashes
    at steps `14` and `15`
  - the current reverted code still keeps the exact guarded step-`13` shell
    out of the retained pool on that second widened surface, so it remains
    diagnosis only and not a landed parity repair
- Those probes were reverted. They are diagnosis, not landed repairs, and the
  position-`1` / position-`4` reland plus the
  position-`0` / position-`4` / position-`5` / position-`6` reland are now
  both covered by executable regressions instead of doc-only memory.

## Optimization Thesis

The next cycle should spend engineering time on the remaining stored
step-`15` floor while keeping step `1` explicit, not on another rerun first
and not on another round of step-`13` theory.

The highest-value work is:

1. keep the current step-`11` / step-`12` / step-`13..15` local guardrails
   green on the repaired chain
2. use stored `v10` certificate plus `step-15-live.ndjson` as the primary
   diagnosis surface
3. keep the stored step-`13` hit frozen and step `1` explicit on the checklist
4. land a parity-preserving step-`15` repair, then rerun and refresh compare /
   benchmark / certification

Treat `v10` plus its stored audit bundle as the current canonical guardrail.
Keep the replay harness corpus and benchmark inputs frozen until real stored
behavior changes.

## Decision Rules

- Trust stored artifacts over terminal impressions.
- Treat clean-tree completed `v10` as the canonical stored claim bundle until a
  newer parity-and-breadth candidate exists.
- Require targeted claim regressions plus replay-harness parity before any new
  full-profile rerun.
- Keep step `1` explicit as the separate early breadth blocker and repair the
  remaining stored late-floor miss at step `15` before spending the next cycle
  on broader late-step theory or another rerun that does not close it.
- Prefer narrow, regression-backed fixes over broad frontier rewrites.
- Do not reland the exploratory `[3,5,3,3,5,1,1]` or `[5,1,3,3,5,3,3]`
  step-`13` widenings directly; both still leave accepted-hash parity open.
- Do not accept a "fix" that only wakes guarded, replay, realistic-shadow, or
  demo-only behavior.
- Keep user-facing and paper-facing wording at `bounded live recovery` until a
  passing certificate exists.

## Active Phase: Stored Step-`15` Breadth Repair

Goal:

- turn the parity-clean-but-breadth-failing `v10` lane into a rerun-ready
  candidate by repairing the remaining stored late-floor miss at step `15`
  while preserving accepted-hash parity through step `15` and keeping stored
  step `13` closed

Loop:

1. keep the stored `v10` compare / certification / benchmark regressions green
2. keep the local step-`11` breadth and step-`12` selector guardrails green
3. keep the current step-`13` / step-`14` / step-`15` canonical guardrails
   green until a replacement is explicitly proved
4. use the stored `v10` certificate and `step-15-live.ndjson` first
5. land a parity-preserving step-`15` repair
6. rerun targeted claim tests plus replay parity on that repair
7. launch the next clean full-profile rerun
8. refresh compare / benchmark / certification on that new stored bundle
9. only treat certification as newly in reach if the rerun keeps step-`15`
   completion while closing the remaining breadth failures

Current slice order:

1. hold the stored step-`13` hit and the frozen negative controls green
2. diagnose stored step `15` on top of `v10`
3. land and validate a local step-`15` repair
4. rerun and refresh stored compare / benchmark / certification
5. revisit stored step `1` from the new evidence bundle once step `15` moves

Do not reopen first:

- another fresh full-profile rerun before a local step-`15` repair is landed
- another fresh step-`13` widening theory pass before step `15` is diagnosed
- a `resume`-based restart of stopped `v4`
- another runtime-only step-`4` micro-optimization slice first
- another stored/local step-`11` rerun first
- a raw reland of the exploratory `[3,5,3,3,5,1,1]` or
  `[5,1,3,3,5,3,3]` step-`13` widenings
- replay-fixture recapture or benchmark-file churn first
- stronger wording or runtime-threshold freeze before a passing certificate
  exists

## Phase 2: Re-Earn One Full-Profile Claim Run

Goal:

- produce one new stored full-profile bundle beyond `v10` that consumes the
  remaining stored breadth repair

Required output:

- one canonical run directory from the disclosed desktop
- full-profile completion through step `15`
- accepted-hash parity through step `15`
- early and late breadth floors passed from stored evidence
- complete reason, prune, narrative, and manifest surfaces preserved

## Phase 3: Freeze Signoff Artifacts

Goal:

- turn the repaired bundle into the auditable claim signoff surface

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
- stronger sentence tied explicitly to the stored claim certificate and
  disclosed desktop bundle

## Non-Goals Until The Step-13 Repair Is Real

- another runtime-only slice first
- another broad later-step band-expansion slice
- another raw step-`13` exploratory widening reland
- metadata-only cleanup in place of breadth repair
- stronger user-facing language before a passing certificate exists

## Success Condition

This plan is done only when one stored `desktop_claim_shadow` bundle from the
disclosed desktop shows all of the following at the same time:

- full-profile completion through step `15`
- no silent guarded, replay, realistic-shadow, or demo-only fallback
- accepted parity through step `15`
- early and late breadth gates passed from stored evidence
- the accepted path is canonical in ordinal order from step `9` through
  step `15`, and any remaining breadth repair is earned on that path
- complete reason, prune, narrative, and manifest data
- passing compare, benchmark, and certification outputs
