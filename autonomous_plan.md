# Autonomous Claim Lane Plan

Last updated: 2026-04-04
Status: active

This file is the staged path from the current completed-but-not-certified
claim-lane state to final signoff. It is intentionally strategic and
forward-looking rather than a run diary.

## Objective

Produce one stored `desktop_claim_shadow` bundle from the disclosed desktop
that:

- finishes through step `15`
- preserves accepted-hash parity and honest breadth accounting
- passes compare, benchmark, and certification on stored evidence

Until that bundle exists, keep the paper wording at `bounded live recovery`.

## Current Strategic Position

- A completed clean-start full-profile bundle now exists:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v5`
  finishes through step `15` on clean-tree repo head
  `c1fbb51d4fc9a620cd2ce95c9c3eadfe1a54fc65` with release binary hash
  `c42758f96c8171900651503d7f2a0ffe9915966c41edea98d8f1e296fc772a4e`.
- Stored compare, certification, and benchmark outputs now exist beside that
  `v5` run and are the current authoritative evidence surface.
- The pre-flight gate, one fresh full-profile completion through step `15`,
  claim-policy honesty, fallback honesty, narrative/event completeness,
  exact-screen reason completeness, prune-class completeness, and manifest
  completeness are now earned on stored evidence.
- Runtime is no longer the first blocker on the stored slice:
  the current benchmark bundle records `408 ms` and passes the provisional
  runtime threshold.
- The old `v3` step-`14` zero-candidate opening and the old step-`4` runtime
  wall are no longer the first blockers.
- Accepted-hash parity is still open on stored `v5`, and the earliest stored
  fork is step `9`.
- Because each accepted step seeds the next library layer, that step-`9`
  fork is now the first unresolved canonical blocker and should be repaired
  before more downstream search work lands.
- A new local step-`9` same-primary selector repair is now landed and guarded
  by tests on the guarded step-`8` prefix, but no stored rerun has consumed it
  yet; `v5` therefore still carries the old step-`9` fork and the next cycle
  must re-earn stored evidence on `v6`.
- Local step-`11` and step-`12` retained-pool, selector, minimality, and
  cache-key repairs have now been rerun green on top of that repaired local
  step-`9` selector; treat them as downstream guardrails until `v6`
  re-consumes the canonical chain.
- Local step-`13`, step-`14`, and narrow step-`15` repairs remain valuable,
  but they should now be treated as guardrails rather than as the next
  widening targets.
- No new per-step claim search-band expansion should be the next engineering
  dollar. Hold the current widened late surfaces fixed until the accepted
  path is stable and correct through step `15`.

## Optimization Thesis

The next cycle should spend engineering time on re-earning stored evidence for
the landed ordinal-parity repairs, not on another pre-rerun selector theory.

The highest-value work is:

1. launch `long-rerun-v6` on the repaired local tree
2. refresh compare, benchmark, and certification on that new stored bundle
3. keep the current step-`13` / step-`14` / step-`15` local surfaces green as
   guardrails, but do not widen those bands further yet
4. revisit late breadth and late exact-screen pressure only after `v6`
   reveals what still fails on the now-repaired canonical chain

Treat the stored `v5` audit bundle plus the landed local regressions as the
current guardrails. Keep the replay harness corpus and benchmark inputs frozen
until real stored behavior changes.

## Decision Rules

- Trust stored artifacts over terminal impressions.
- Treat completed `v5` as the canonical stored claim bundle until `v6` exists.
- Require targeted claim regressions plus replay-harness parity before any new
  full-profile rerun.
- Repair the earliest divergent accepted step before spending the next cycle on
  later-step repairs.
- Prefer narrow, regression-backed fixes over broad frontier rewrites.
- Do not broaden per-step claim search bands again until the canonical
  step-`9` through step-`15` path is stable and correct.
- Do not accept a "fix" that only wakes guarded, replay, realistic-shadow, or
  demo-only behavior.
- Keep user-facing and paper-facing wording at `bounded live recovery` until a
  passing certificate exists.

## Active Phase: Stored Ordinal Parity Repair

Goal:

- turn the completed-but-failing `v5` lane into a locally repaired,
  rerun-ready canonical candidate by repairing the earliest accepted-hash fork
  first and then walking downstream in sequence order

Loop:

1. keep the stored compare / certification / benchmark regressions green
2. keep the landed local step-`9` selector regression green
3. keep the landed step-`11` and step-`12` repairs green on that repaired
   local prefix
4. keep the landed step-`13` / step-`14` / step-`15` later-surface regressions
   green as guardrails without widening bands further
5. rerun targeted claim tests plus replay parity
6. launch `long-rerun-v6`
7. only then re-measure whether any late breadth or late exact-screen repair
   is still needed on the now-canonical chain
9. only treat certification as newly in reach if `v6` keeps step-`15`
   completion while closing parity and breadth failures

Current slice order:

1. launch `long-rerun-v6`
2. refresh compare / benchmark / certification on `v6`
3. if parity still diverges, resume ordinal local diagnosis from the earliest
   stored miss on the repaired chain
4. keep step-`13` / step-`14` / step-`15` guardrails frozen without new band
   expansion

Do not reopen first:

- another clean-start full-profile rerun before the local repair is green
- a `resume`-based restart of stopped `v4`
- another runtime-only step-`4` micro-optimization slice first
- another step-`13` / step-`14` / step-`15` widening slice before the
  step-`9` / step-`11` / step-`12` canonical chain is stable
- replay-fixture recapture or benchmark-file churn first
- stronger wording or runtime-threshold freeze before a passing certificate
  exists

## Phase 2: Re-Earn One Full-Profile Claim Run

Goal:

- produce one new stored full-profile bundle (`v6`) that consumes the local
  repairs

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

## Non-Goals Until `v6` Is Real

- another runtime-only slice first
- another later-step band-expansion slice before the step-`9` through
  step-`12` canonical path is stable
- metadata-only cleanup in place of parity / breadth repair
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
