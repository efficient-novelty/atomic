# Autonomous Claim Lane Plan

Last updated: 2026-03-23
Status: active

This file is the staged path from the current claim-lane bottleneck to final
signoff. It intentionally omits rollout history that no longer changes the next
decisions.

## Objective

Produce one stored `desktop_claim_shadow` bundle from the disclosed desktop
that:

- finishes through step `15`
- preserves accepted-hash parity and honest breadth accounting
- passes compare, benchmark, and certification on stored evidence

Until that bundle exists, keep the paper wording at `bounded live recovery`.

## Current Operating Read

- The live blocker is still step-`4` throughput on the real
  `desktop_claim_shadow_1h` profile.
- The current full-profile baseline is
  `runs/codex-claim-release-full-delayed-summary-v1`.
- Delayed materialization is already earned and should be treated as baseline.
- The hot cost has shifted from compact materialization into remaining-one
  terminal-summary construction.
- No downstream signoff work matters until that step-`4` throughput improves
  again.

## Execution Order

### Phase 1. Win The Next Step-`4` Throughput Slice

Goal:

- make summary build cheaper or kill more work before summary build starts

Required output:

- one narrow search-code patch
- one stored release rerun with `until_step = 4`
- a telemetry comparison against the current baselines

Done when:

- the rerun shows a real stored win in summary-side step-`4` telemetry

### Phase 2. Re-Earn The Real Full-Profile Read

Goal:

- prove the winning step-`4` patch helps on the real
  `desktop_claim_shadow_1h` profile

Required output:

- one new stored intended-profile rerun
- a read of its full-profile artifacts, especially `step-04-live.ndjson`

Done when:

- the full-profile rerun either moves materially farther or exposes a new later
  blocker honestly

### Phase 3. Finish A Full Claim Bundle

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

### Phase 4. Freeze Signoff Artifacts

Goal:

- turn the finished run into the repo's auditable claim bundle

Required output:

- one compare report against the guarded reference
- one benchmark bundle
- one passing `claim_certificate.json`
- one certified runtime threshold tied to stored evidence

Done when:

- another reviewer can audit the full claim from stored artifacts alone

### Phase 5. Open The Language Gate

Goal:

- allow stronger wording only after the technical gate is closed

Required output:

- user-facing and paper-facing wording updated only after certification passes

Done when:

- the stronger sentence is explicitly tied to the stored claim certificate and
  disclosed desktop bundle

## Non-Goals Until Phase 1 Wins

- reopening split-handoff work
- reopening memory compaction first
- broad frontier rewrites
- benchmark or certification work
- stronger user-facing language

## Baselines To Compare Against

- `runs/codex-claim-release-full-v1a`
- `runs/codex-claim-release-full-delayed-summary-v1`
- `runs/codex-claim-release-step4-delayed-summary-v1`
- `runs/codex-claim-release-step4-telemetry-v1`

## Success Condition

This plan is done only when one stored `desktop_claim_shadow` bundle from the
disclosed desktop shows all of the following at the same time:

- full-profile completion through step `15`
- no silent guarded, replay, realistic-shadow, or demo-only fallback
- accepted parity through step `15`
- breadth gates passed honestly from stored evidence
- complete reason and prune accounting
- benchmark and manifest data sufficient for certification
- passing compare and certification outputs
