# Autonomous Claim Lane: Next Operational Slice

This note is the exact next work order for `desktop_claim_shadow`.

Assume the following are already landed and should stay landed:

- delayed materialization
- the incumbent-primary remaining-one fast path
- the one-pass `structural_nu` summary-build fast path
- the algebraic `nu` ceiling patch
- the family-agnostic claim terminal-admissibility shortcut
- the exact non-allocating connectivity summary scan
- the terminal-only cached parent connectivity decision
- the aggregation-side accept-rank short-circuit
- the higher-fidelity late-surface timing accumulation

Assume the following were already measured and should stay dropped:

- ordering and reuse variants
- expr-keyed or clause-side connectivity cache variants
- terminal-candidate prep or remap cuts
- telemetry-only filter profiling
- the exact cross-multiplied primary-rank bookkeeping rewrite in
  `runs/codex-claim-release-step4-kernel-rank-bookkeeping-v1`
- the constant-`kappa` bound-merge rewrite in
  `runs/codex-claim-release-step4-kernel-bound-merge-v1`
- the lazy incumbent-tie `AcceptRank` rewrite in
  `runs/codex-claim-release-step4-kernel-lazy-acceptrank-v1`
- the local summary batching rewrite in
  `runs/codex-claim-release-step4-kernel-summary-batching-v1`
- the shared reason-key summary bookkeeping rewrite in
  `runs/codex-claim-release-step4-kernel-summary-bookkeeping-v1`
- the fixed bar-clear summary-threshold rewrite in
  `runs/codex-claim-release-step4-kernel-summary-threshold-v1`

## Active Baselines

- Current short baseline:
  `runs/codex-claim-release-step4-kernel-aggregation-v1`
- Current full-profile baseline:
  `runs/codex-claim-release-full-kernel-aggregation-v1`
- Current late-surface diagnostic:
  `runs/codex-claim-release-step4-kernel-late-profile-v1`

## Working Diagnosis

- The intended profile is still blocked in step `4`, not by the old early RSS
  story, but by remaining-one summary throughput.
- The early retained-prefix plateau
  `39 groups / 144845 candidates` is real, but it is not terminal on the real
  profile. The intended run later reopens to `40/147639` at `74` and
  `41/154842` at `140`.
- The honest late target is therefore the reopened `40/147639` surface.
- On that surface, the current diagnostic still shows aggregation first,
  connectivity second, clause filtering third, and exact `nu` fourth.
- The latest summary-bookkeeping rewrite kept the same honest early and
  reopened shapes, improved elapsed materially on both surfaces, and beat the
  late diagnostic strongly at `74/76`, but it still regressed too much on
  total `terminal_summary_build_*` at `24/43/44/54` to keep.
- The current code already skips full `AcceptRank` construction for compact
  summary candidates whose primary rank is clearly worse than the current
  group best, so the next move should stay aggregation-side but go after
  loop-invariant or per-admitted work inside the measured summary kernel, not
  retry another threshold-only, bound-only, exact rank-bookkeeping,
  batching-only, or bookkeeping-only shape first.

## Goal

Land one different narrow aggregation-side cut that lowers the honest short
read on both:

- the matched `24/43/44/54` plateau checkpoints
- the reopened `40/147639` surface at `74/76`

without weakening the retained-prefix shape.

## Do This Next

### 1. Keep The Right Reference Surface

Keep the code behind:

- `runs/codex-claim-release-step4-kernel-aggregation-v1`
- `runs/codex-claim-release-full-kernel-aggregation-v1`
- `runs/codex-claim-release-step4-kernel-late-profile-v1`

Do not reopen first:

- another connectivity-side rewrite
- another clause-filter-side rewrite
- another exact-`nu` cleanup first
- another ordering, reuse, cache, or post-plateau variant
- another retry of `kernel-rank-bookkeeping-v1`
- another retry of `kernel-bound-merge-v1`
- another retry of `kernel-lazy-acceptrank-v1`
- another retry of `kernel-summary-batching-v1`
- another retry of `kernel-summary-bookkeeping-v1`
- another retry of `kernel-summary-threshold-v1`
- another diagnostic-only slice before a new aggregation hypothesis is measured

### 2. Pick One Aggregation Hypothesis

Stay inside the remaining-one compact summary path after connectivity and after
exact `nu`.

Prefer this first:

- keep the current primary-rank short-circuit and stored diagnostics, but
  hoist prefix-wide competition gating and other loop-invariant aggregation
  work out of the per-admitted compact-summary path in
  `compute_terminal_prefix_completion_summary_from_candidates`, so admitted
  candidates stop paying repeated prefix-constant checks on the reopened
  surface

Fallback after that:

- precompute one more compact-summary constant that is currently rebuilt for
  every admitted candidate inside the aggregation block, but do not reopen the
  old terminal-candidate-prep or remap shapes first

Do not pick next:

- another bound-only cleanup; `kernel-bound-merge-v1` already showed that
  per-candidate bound churn was real but not dominant enough to keep
- another batching-only or bookkeeping-only cleanup; `kernel-summary-batching-v1`
  and `kernel-summary-bookkeeping-v1` already showed that those shapes can
  help elapsed on the reopened surface without fixing the honest early
  summary-build wall

If two candidate cuts still look equally plausible, prefer the one that
removes expensive work from every admitted candidate on the reopened surface
rather than the one that only improves rare late tie cases.

### 3. Re-Earn The Short Runtime Read

Run a release claim rerun derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `--until-step 4`
- the winning binary plus the new aggregation-side cut
- live checkpoint persistence left on
- a new run id that states the patch, for example:
  - `runs/codex-claim-release-step4-kernel-summary-loop-hoist-v1`
  - `runs/codex-claim-release-step4-kernel-competition-hoist-v1`
  - `runs/codex-claim-release-step4-kernel-summary-invariants-v1`

Let the run go far enough to capture at least:

- matched `24/43/44/54`
- the reopened `40 groups / 147639 candidates` surface at `74/76`
- `140` only if it still arrives cheaply

### 4. Read The Stored Artifacts

Open at least:

- `reports/steps/step-04-live.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:

- did the new cut lower elapsed and `terminal_summary_build_*` at
  `24/43/44/54` without losing `39/144845`?
- did the reopened `74/76` surface improve enough to justify keep?
- is aggregation still first on `54 -> 76`, or has the wall moved honestly?
- does a later full-profile rerun become the next gate, or is step `4` still
  the next honest runtime target?

### 5. Re-Earn Only The Validation Needed

If the slice stays claim-only and step-`4` runtime-only, rerun only:

- `cargo test -p pen-search claim_`
- `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

If the short runtime slice earns keep, then branch back to one new full-profile
rerun before parity, breadth, compare, benchmark, or certification work.

## Keep Or Branch Decision

After the next short rerun:

- stay on runtime work if step `4` still blocks the intended profile
- branch back to a new full-profile rerun only if the short rerun earns keep
- branch to parity, breadth, compare, benchmark, and certification work only
  after a later full-profile run honestly moves past the step-`4` wall

## Stop Condition For This Note

Rewrite this file as soon as one new stored rerun shows one of these is true:

- a new aggregation-side cut earns keep on the honest short surface
- the step-`4` wall moves away from aggregation first
- a later full-profile rerun reaches a new blocker honestly
- runtime work is no longer the next honest move
