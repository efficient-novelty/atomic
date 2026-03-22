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
- The optimized release reruns now add a sharper read:
  - `codex-claim-release-step5-v1` stayed under about `167.1 MiB` observed RSS
    after `1777.1s` on step `4`, so the early release build is no longer
    showing the old allocator cliff there
  - the new direct compact claim materialization fast path plus shared
    work-item order key improved the same hot step-`4` checkpoints by about
    `12-14%` on `codex-claim-release-step4-fastpath-v2`
  - a follow-up slice-based terminal-clause filter path that reuses the shared
    clause slice when claim filters are inactive improved those same hot
    checkpoints by about `18-20%` versus `codex-claim-release-step4-fastpath-v2`
    on `codex-claim-release-filter-slice-v1a` while keeping observed RSS below
    about `84.0 MiB` through prefix state `7`
  - a follow-up intended-profile rerun on that newer binary
    (`codex-claim-release-full-v1a`) did not re-hit the old allocator abort
    before an external timeout after `3844.7s`; by then step `4` had explored
    `43` prefix states, enumerated `848047359` candidates, kept the frontier
    queue at `2732`, and held observed RSS to about `278.2 MiB`
  - the same stored step-live stream also showed the retained prefix cache
    flattening after prefix state `24`: later checkpoints stayed at `39`
    groups / `144845` retained candidates while legality summaries kept rising
    from `140197` to `205199`, so much of the remaining step-`4` cost was
    still repeated exact terminal completion on surfaces that were no longer
    adding new retained groups
  - a follow-up throughput pass now reuses one scratch terminal telescope plus
    the precomputed prefix bit cost across claim exact terminal bound checks,
    completion summaries, and compact materialization, so the hot remaining-two
    path stops cloning the full prefix telescope for every admitted last-clause
    candidate
  - a newer claim-only throughput pass now skips discovery-time full
    evaluation for compact terminal candidates that are already below bar or
    no longer beat the current incumbent; a fresh single-worker smoke rerun
    (`codex-claim-scratch-smoke-v2`) then reached `prefix_states_explored = 5`
    at `499.9s` versus `519.4s` on `codex-claim-scratch-smoke-v1`, reached
    `prefix_states_explored = 6` at `572.7s`, and stayed below about
    `82.0 MiB` observed RSS through that checkpoint before manual stop
- That means the queue-side cloned clause-catalog spike is no longer the main
  blocker, and the old allocator abort is no longer the first visible failure
  mode on the new binary. The next blocker is still step-`4` exact
  remaining-two throughput on the optimized claim lane, but the newest binary
  shape now needs a comparable intended-profile rerun before reopening a
  broader rewrite.

## Working Order

### 1. Keep Pushing The Optimized Claim Run Farther

Run `desktop_claim_shadow_1h` on the disclosed desktop with the latest release
binary and inspect the stored artifacts, not terminal output.

Focus on:

- latest completed step
- observed RSS versus governor-accounted RSS gap
- step `4` / `5` live checkpoints
- whether step `4` now moves materially farther inside the same budget window
- whether the run now fails later than the old step-`4` startup cliff

### 2. If It Still Stalls, Isolate The Next Real Cost Center

Use the stored bundle to decide which remaining pressure story is real:

- step-`4` exact remaining-two throughput
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

1. Rerun the intended `desktop_claim_shadow_1h` profile on the newer release
   binary that now reuses a scratch terminal telescope and skips
   discovery-time full evaluation for compact claim candidates that are
   already below bar or incumbent-dominated on the hot remaining-two path,
   then inspect the stored step-live artifacts rather than terminal output.
2. If that rerun still stalls in step `4`, use the new stored evidence to
   decide whether the next narrow fix belongs in earlier incumbent pruning,
   broader exact bound screening, or some later-step pressure story that still
   is not visible.
3. If a rerun completes, move immediately to compare, benchmark, and
   certification on that same run directory.

## Guardrails

- Preserve existing `strict_canon_guarded`, `realistic_frontier_shadow`, and
  `demo_breadth_shadow` behavior.
- Prefer narrow fixes driven by stored evidence over broad speculative rewrites.
- Do not lower breadth floors or hide misses to make the claim look cleaner.
- Do not use stronger language such as `unguided` before certification passes.
