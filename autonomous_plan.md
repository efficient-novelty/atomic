# Autonomous Claim Lane Plan

Last updated: 2026-04-03
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
- Local step-`11` and step-`12` retained-pool, selector, minimality, and
  cache-key repairs are now landed and guarded by tests, but no stored rerun
  has consumed them yet.
- The remaining late local blocker is now split into two concrete subproblems:
  - step `13` still opens on a singleton-heavy claim-generic band-`7` catalog
    with raw widths `[3,1,1,1,1,1,1]`, only `3` raw telescopes, and
    `2 / 3` roots exact-pruned before proof-close
  - step `15` still opens a broad raw `6561`-telescope catalog on the restored
    canonical branch, but exact partial-prefix bar failure still removes `512`
    prefixes before proof-close
- Step `14` is now a local guardrail, not the first blocker:
  the repaired chain keeps a widened `kappa = 9` catalog with raw `19683`,
  `3` surviving roots, `12027` live generated prefixes, and a selector that
  preserves the canonical step-`15` continuation.
- A follow-up exploratory global step-`13` widening was useful but is not
  landable as-is:
  it can lift the repaired local step-`13` read to raw `2187` /
  generated `615`, but it also disturbs claim prefix-memo,
  realistic-shadow, demo-lane, and divergent late-step guardrails.
- Accepted-hash parity is still open on stored `v5`, with the earliest stored
  fork at step `9`; that fork should stay secondary until the step-`13` and
  step-`15` breadth story is repaired honestly.

## Optimization Thesis

The next cycle should spend engineering time on local parity-plus-breadth
repair, not on another full-profile rerun first.

The highest-value work is:

1. introduce a scoped claim-only step-`13` widening path, or an equivalent
   mechanism, that can improve the repaired band-`7` catalog without waking
   claim prefix-memo, realistic-only, demo-only, or divergent late-step
   guardrails
2. inspect and repair the canonical step-`15` exact partial-prefix bar path as
   its own problem
3. revisit step `9` final selection only if parity still diverges after the
   late-surface repair lands locally

Treat the stored `v5` audit bundle plus the landed local regressions as the
current guardrails. Keep the replay harness corpus and benchmark inputs frozen
until real stored behavior changes.

## Decision Rules

- Trust stored artifacts over terminal impressions.
- Treat completed `v5` as the canonical stored claim bundle until `v6` exists.
- Require targeted claim regressions plus replay-harness parity before any new
  full-profile rerun.
- Prefer narrow, regression-backed fixes over broad frontier rewrites.
- Do not reland the rejected global step-`13` band-`7` widening directly.
- Do not accept a "fix" that only wakes guarded, replay, realistic-shadow, or
  demo-only behavior.
- Keep user-facing and paper-facing wording at `bounded live recovery` until a
  passing certificate exists.

## Active Phase: Stored Parity And Breadth Repair

Goal:

- turn the completed-but-failing `v5` lane into a locally repaired,
  rerun-ready parity-plus-breadth candidate

Loop:

1. keep the stored compare / certification / benchmark regressions green
2. keep the landed local step-`11` / step-`12` parity repairs green
3. land a scoped claim-only widening, or equivalent, at step `13` and then
   re-measure the residual exact-screen losses there
4. inspect and repair the step-`15` exact partial-prefix bar path on the
   canonical branch
5. re-evaluate step `9` only after the late breadth story is clearer
6. rerun targeted claim tests plus replay parity
7. only then launch `long-rerun-v6`
8. only treat certification as newly in reach if `v6` keeps step-`15`
   completion while closing parity and breadth failures

Current slice order:

1. step-`13` scoped claim-only widening plus residual exact-screen read
2. step-`15` exact partial-prefix bar path
3. step-`9` final selection if it still matters after the late repair
4. compare / benchmark / certification refresh only after `v6` exists

Do not reopen first:

- another clean-start full-profile rerun before the local repair is green
- a `resume`-based restart of stopped `v4`
- another runtime-only step-`4` micro-optimization slice first
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
- raw step-`9` enumeration or terminal-clause-filter theories before the late
  breadth story is clearer
- metadata-only cleanup in place of parity / breadth repair
- stronger user-facing language before a passing certificate exists

## Success Condition

This plan is done only when one stored `desktop_claim_shadow` bundle from the
disclosed desktop shows all of the following at the same time:

- full-profile completion through step `15`
- no silent guarded, replay, realistic-shadow, or demo-only fallback
- accepted parity through step `15`
- early and late breadth gates passed from stored evidence
- the step-`13` and step-`15` floor pressure is repaired on the winning claim
  path
- complete reason, prune, narrative, and manifest data
- passing compare, benchmark, and certification outputs
