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

- The current canonical stored claim bundle is clean-tree completed `v9`:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v9`
  on repo head `67c26eca02cb5546745bdd5ca5b31468e6807f42` with release binary
  hash `2023ea693e72403b98448ab1bece5048b739a2cb115aafcd2b1580cb941a59bf`.
- Stored compare, certification, and benchmark outputs now exist beside that
  `v9` run and are the current authoritative evidence surface.
- Stored accepted-hash parity is earned on `v9` through step `15`.
- Runtime, manifest completeness, fallback honesty, narrative/event
  completeness, exact-screen reason completeness, and prune-class completeness
  are all earned on stored evidence.
- The remaining blockers are stored breadth misses on the canonical chain:
  step `1 = 546 / 2144`, step `13 = 123 / 2200`, step `15 = 1794 / 5000`.
- Step `13` is now the earliest remaining stored late-floor miss and is the
  next honest local engineering dollar.
- Two new non-landed local step-`13` probes now sharpen the safe search space:
  - widening only operator-bundle formation positions `1` and `4` to
    demo-like variants lifts the repaired local read to
    `[3,5,3,3,5,1,1]` / raw `675` / generated `2223`, but it changes the
    accepted late path to `45 / 7 -> 61 / 9`
  - widening positions `0`, `4`, `5`, and `6` while keeping position `1`
    exact lifts the repaired local read to
    `[5,1,3,3,5,3,3]` / raw `2025` / generated `2995` and preserves the
    guarded step-`14` / step-`15` accepted hashes, but it still replaces the
    guarded step-`13` accepted hash with a non-reference `46 / 7` shell
  - on that second widened surface, the exact guarded step-`13` telescope
    still evaluates and clears the bar locally, but continuing from it no
    longer reproduces the guarded step-`14` accepted hash
- Those probes were reverted. They are diagnosis, not landed repairs.

## Optimization Thesis

The next cycle should spend engineering time on parity-preserving stored
breadth repair at step `13`, not on another stored rerun first and not on a
raw reland of any exploratory widened step-`13` surface.

The highest-value work is:

1. keep the current step-`11` / step-`12` / step-`13..15` local guardrails
   green on the repaired chain
2. use stored `v9` certificate plus `step-13-live.ndjson` as the primary
   diagnosis surface
3. land a parity-preserving step-`13` breadth repair on top of the canonical
   chain
4. only then launch the next full-profile stored rerun and refresh compare /
   benchmark / certification

Treat `v9` plus its stored audit bundle as the current canonical guardrail.
Keep the replay harness corpus and benchmark inputs frozen until real stored
behavior changes.

## Decision Rules

- Trust stored artifacts over terminal impressions.
- Treat clean-tree completed `v9` as the canonical stored claim bundle until a
  newer parity-and-breadth candidate exists.
- Require targeted claim regressions plus replay-harness parity before any new
  full-profile rerun.
- Repair the earliest remaining stored breadth miss before spending the next
  cycle on later-step theory that does not close that miss.
- Prefer narrow, regression-backed fixes over broad frontier rewrites.
- Do not reland the exploratory `[3,5,3,3,5,1,1]` or `[5,1,3,3,5,3,3]`
  step-`13` widenings directly; both still leave accepted-hash parity open.
- Do not accept a "fix" that only wakes guarded, replay, realistic-shadow, or
  demo-only behavior.
- Keep user-facing and paper-facing wording at `bounded live recovery` until a
  passing certificate exists.

## Active Phase: Stored Breadth Repair

Goal:

- turn the parity-clean-but-breadth-failing `v9` lane into a rerun-ready
  candidate by repairing the earliest stored breadth miss at step `13` while
  preserving accepted-hash parity through step `15`

Loop:

1. keep the stored `v9` compare / certification / benchmark regressions green
2. keep the local step-`11` breadth and step-`12` selector guardrails green
3. keep the current step-`13` / step-`14` / step-`15` canonical guardrails
   green until a replacement is explicitly proved
4. use the stored certificate and late-step live checkpoints first
5. land a parity-preserving step-`13` breadth repair
6. rerun targeted claim tests plus replay parity
7. launch the next clean full-profile rerun only after that local repair is
   stable
8. refresh compare / benchmark / certification on the new stored bundle
9. only treat certification as newly in reach if the rerun keeps step-`15`
   completion while closing the remaining breadth failures

Current slice order:

1. diagnose step `13` on top of clean-tree `v9`
2. turn the new exploratory step-`13` probes into a narrower
   parity-preserving repair
3. rerun once that repair is local-green
4. revisit stored step `15` and step `1` only after the step-`13` canonical
   surface is fixed

Do not reopen first:

- another clean-start full-profile rerun before the local step-`13` repair is
  green
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

- produce one new stored full-profile bundle beyond `v9` that consumes the
  parity-preserving breadth repair

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
