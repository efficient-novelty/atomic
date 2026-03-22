# Autonomous Claim Lane Plan

Last updated: 2026-03-22
Status: active

This file tracks only the remaining operational work for
`desktop_claim_shadow`. Completed rollout work belongs in the skill
references and claim-lane baseline docs.

## Objective

Produce one stored, full-profile `desktop_claim_shadow` bundle from the
disclosed desktop that:

- finishes through step `15` without allocator abort
- preserves auditable failure artifacts if a rerun still stops early
- preserves accepted-hash parity and honest breadth accounting
- passes compare, benchmark, and certification on stored evidence

Until that bundle exists, keep the paper wording at `bounded live recovery`.

## Current Operating Picture

- The active blocker is still runtime stability on
  `configs/desktop_claim_shadow_1h.toml`.
- The latest known full-profile failure is still
  `memory allocation of 1212416 bytes failed`.
- Failure survivability is no longer the issue: claim runs now keep
  `run.json`, step artifacts, frontier snapshots, and narratives on disk.
- The old step-`4` startup RSS cliff has moved materially:
  - `codex-claim-shared-signature-v1` still showed about `3.06 GiB`
  - `codex-claim-frontier-catalog-reuse-v1` removed that startup checkpoint
    from stored evidence and first reported about `66.4 MiB` at step `4`
- That means the queue-side cloned clause-catalog spike is no longer the main
  unknown. The next unknown is what still fails later on the intended full
  profile.

## Working Order

### 1. Re-run The Intended Full Profile

Run `desktop_claim_shadow_1h` on the disclosed desktop with the latest memory
changes and inspect the stored artifacts, not terminal output.

Focus on:

- latest completed step
- observed RSS versus governor-accounted RSS gap
- step `4` / `5` live checkpoints
- whether the run now fails later than the old step-`4` startup cliff

### 2. If It Still Fails, Isolate The Next Real Allocation Site

Use the stored bundle to decide which remaining pressure story is real:

- legality-summary residency
- raw-surface expansion
- later-step frontier retention
- worker scratch / spill / checkpoint pressure
- some still-untracked allocation path

Do not reopen already-landed policy split work unless the stored evidence
forces it.

### 3. Once It Finishes, Re-Earn Stored Claim Evidence

From the stabilized full-profile bundle:

- preserve accepted-hash parity through step `15`
- re-earn early and late breadth gates from stored counts
- keep `full_telescopes_evaluated` inside a moderate certified range
- make fallback, exact-screen reasons, and prune classes impossible to miss

Breadth gates that still must be earned from stored claim evidence:

- step `1` generated raw `= 2144`
- step `10` generated `>= 500`
- step `11` generated `>= 800`
- step `12` generated `>= 1200`
- step `13` generated `>= 2200`
- step `14` generated `>= 3500`
- step `15` generated `>= 5000`

### 4. Freeze Benchmarking And Certification On That Bundle

Use the same stabilized bundle to produce:

- one guarded-vs-claim compare report
- one benchmark bundle
- one passing `claim_certificate.json`
- one certified runtime threshold tied to stored evidence

## Success Condition

This plan is done only when one stored `desktop_claim_shadow` bundle from the
disclosed desktop shows all of the following at the same time:

- full-profile completion through step `15`
- no silent guarded, replay, realistic-shadow, or demo-only fallback
- accepted parity through step `15`
- breadth gates passed honestly from stored evidence
- complete reason / prune accounting
- benchmark and manifest data sufficient for certification
- passing compare and certification outputs

## Immediate Next Slice

1. Rerun `desktop_claim_shadow_1h` and inspect the stored RSS-gap and
   step-live artifacts with the frontier-catalog reuse fix in place.
2. If it still aborts, identify the new dominant allocation site from the
   stored bundle and make the next narrow memory fix there.
3. If it completes, move immediately to compare, benchmark, and certification
   on that same run directory.

## Guardrails

- Preserve existing `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` behavior.
- Prefer narrow fixes driven by stored evidence over broad speculative rewrites.
- Do not lower breadth floors or hide misses to make the claim look cleaner.
- Do not use stronger language such as `unguided` before certification passes.
