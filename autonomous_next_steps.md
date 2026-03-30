# Autonomous Claim Lane: Next Operational Slice
Last updated: 2026-03-30

This note is the exact next work order for `desktop_claim_shadow`.

Assume the following are already landed and should stay landed:
- delayed materialization
- the incumbent-primary remaining-one fast path
- the one-pass `structural_nu` summary-build fast path
- the algebraic `nu` ceiling patch
- the family-agnostic claim terminal-admissibility shortcut
- the exact non-allocating connectivity summary scan
- the terminal-only cached parent connectivity decision
- the aggregation-side accept-rank short-circuit that skips full `AcceptRank` construction for primary-dominated bar-clearers
- the claim open-band terminal-clause handoff fast path that keeps exact-admitted open-band surfaces on clause refs instead of per-clause admissibility payloads
- the higher-fidelity late-surface timing accumulation

Assume the following were already measured and should stay dropped as standalone next moves:
- ordering and reuse variants
- expr-keyed or clause-side connectivity cache variants
- terminal-candidate prep or remap variants
- telemetry-only filter profiling as a separate slice
- the exact cross-multiplied primary-rank bookkeeping rewrite in `runs/codex-claim-release-step4-kernel-rank-bookkeeping-v1`
- the constant-`kappa` bound-merge rewrite in `runs/codex-claim-release-step4-kernel-bound-merge-v1`
- the lazy incumbent-tie `AcceptRank` rewrite in `runs/codex-claim-release-step4-kernel-lazy-acceptrank-v1`
- the local summary batching rewrite in `runs/codex-claim-release-step4-kernel-summary-batching-v1`
- the shared reason-key summary bookkeeping rewrite in `runs/codex-claim-release-step4-kernel-summary-bookkeeping-v1`
- the prefix-wide competition-gate hoist in `runs/codex-claim-release-step4-kernel-competition-hoist-v1`
- the claim open-band admitted-kernel fusion in `runs/codex-claim-release-step4-kernel-admitted-kernel-v1`
- the direct bound/bookkeeping cleanup in `runs/codex-claim-release-step4-kernel-bound-bookkeeping-v1`
- the exact rank-metadata pack in `runs/codex-claim-release-step4-kernel-rank-metadata-v1`
- the compact-summary strict-better-incumbent exact-rank deferral in `runs/codex-claim-release-step4-kernel-aggregation-tiecut-v1`
- the parent-summary connectivity lookup reuse in `runs/codex-claim-release-step4-kernel-reopened-connectivity-v1`
- the eager clause-filter-wide metadata pack in `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
- the lazy admitted-only metadata retry in `runs/codex-claim-release-step4-kernel-admitted-metadata-v1`

## Active Baselines
- Current short baseline: `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Current full-profile baseline: `runs/codex-claim-release-full-kernel-aggregation-v1`
- Current late-surface diagnostic: `runs/codex-claim-release-step4-kernel-late-profile-v1`

## Revised Working Diagnosis
- The broader claim open-band handoff cut earned keep on stored step-`4` evidence.
- The honest retained-prefix story is now:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
- The new short winner beat the prior kept short baseline on both elapsed and `terminal_summary_build_*` at every matched checkpoint:
  - `24`: `417756 / 414838` instead of `549630 / 492524`
  - `43`: `760135 / 755953` instead of `990480 / 892772`
  - `44`: `777287 / 773037` instead of `1012067 / 912271`
  - `54`: `962821 / 957858` instead of `1247600 / 1126754`
- It also beat the kept full-profile reopened surface on both elapsed and `terminal_summary_build_*`:
  - `74`: `1315892 / 1309667` instead of `1743244 / 1579138`
  - `76`: `1358533 / 1352182` instead of `1797441 / 1628768`
- Stored step-live kernel telemetry says the targeted handoff cost is now tiny on the keep slice:
  - `24`: terminal clause-filter handoff `= 1716095 us`
  - `76`: terminal clause-filter handoff `= 5572740 us`
  - `terminal_summary_admissibility_checks = 0` through the stored `76` read
- The rerun was manually stopped after the decisive stored `76` checkpoint; one extra stored `77` checkpoint flushed while stopping and kept the same honest reopened surface.
- Because that stop was external during step `4`, `reports/latest.txt` still reflects completed step `3` and `run.json` still says `status = "running"`; the authoritative evidence for this winner lives in `reports/steps/step-04-live.ndjson`.

## Honest Read
- The broader open-band handoff cut is real and is now the short keep winner.
- The next honest unknown is no longer another short step-`4` micro-slice first.
- The lane is still blocked on the intended profile until one new full-profile rerun shows what later blocker remains on this winner.

## Goal
Run one full-profile intended `desktop_claim_shadow_1h` rerun on the new short winner and read the later blocker honestly from stored artifacts.

## Do This Next

### 1. Keep The Winning Short Binary
Keep the code behind:
- `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- `runs/codex-claim-release-full-kernel-aggregation-v1`
- `runs/codex-claim-release-step4-kernel-late-profile-v1`

Do not reopen first:
- another short step-`4` micro-slice
- another metadata retry
- another connectivity reuse retry
- another admitted-kernel-only replay
- another clause-load-only or bookkeeping/bound-only cleanup
- another diagnostic-only slice before the full-profile rerun is measured

### 2. Re-Earn The Intended-Profile Read
Run one release claim rerun derived from `configs/desktop_claim_shadow_1h.toml` with:
- the kept open-band handoff code above
- the profile's intended full horizon
- live checkpoint persistence left on
- a new run id that states the kept patch, for example:
  - `runs/codex-claim-release-full-open-band-handoff-v1`

Let the rerun go far enough to answer from stored evidence:
- does it preserve the new `24/43/44/54` win on the real full profile?
- does it preserve the reopened `74/76` win there too?
- does it move materially farther than `runs/codex-claim-release-full-kernel-aggregation-v1`?
- does it finally reach step `5` or expose a different later blocker honestly?

### 3. Read The Stored Artifacts
Open at least:
- `reports/steps/step-04-live.ndjson`
- `reports/steps/step-05-live.ndjson` if it exists
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:
- did the full-profile rerun keep the new early and reopened short wins?
- if it still stalled in step `4`, which later bucket order dominated there?
- if it reached step `5` or later, what new blocker became visible?
- did the observed-versus-accounted RSS story stay honest on the kept short winner?

### 4. Re-Earn Only The Validation Needed
If the next turn is rerun-only with no new code changes, do not reopen extra tests first.

If new code changes land before the full-profile rerun, rerun only:
- `cargo test -p pen-search claim_`
- `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

## Keep Or Branch Decision
After the next full-profile rerun:
- stay on runtime work if it still stalls in step `4` or exposes a later runtime blocker
- keep this short winner as the runtime reference if the full-profile rerun preserves the early and reopened surfaces honestly
- branch to parity, breadth, compare, benchmark, and certification work only after a later full-profile rerun honestly moves past the step-`4` wall

## Stop Condition For This Note
Rewrite this file as soon as one new stored full-profile rerun shows one of these is true:
- the kept open-band handoff winner reaches a new later blocker honestly
- the intended profile finally moves past the step-`4` wall
- the full-profile rerun fails for a different structural reason than the short evidence predicted
- runtime work is no longer the next honest move
