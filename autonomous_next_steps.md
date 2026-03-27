# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, the algebraic `nu`
ceiling patch, the family-agnostic claim terminal-admissibility shortcut, the
exact non-allocating connectivity summary scan, the terminal-only cached
parent connectivity decision, and the aggregation-side accept-rank
short-circuit are already landed.
Assume the context-equivalence quotient, frontier-pop ordering, exact-two-step
local ordering, proof-close handoff ordering, post-plateau summary-skip,
post-plateau materialize-side gating, post-plateau summary-cache reuse,
expr-keyed terminal-clause caching, clause-side connectivity profile
precompute, terminal-candidate prep or remap cuts, and the temporary
clause-filter timing counter were all measured on stored reruns and then
dropped after failing keep.

The current short step-`4` baseline remains
`runs/codex-claim-release-step4-kernel-aggregation-v1`.
The previous short baseline remains
`runs/codex-claim-release-step4-kernel-connectivity-v2`.
The current full-profile baseline is now
`runs/codex-claim-release-full-kernel-aggregation-v1`.
The previous full-profile baseline is now
`runs/codex-claim-release-full-nu-profile-v1`.
The most recent informative late-surface diagnostic is now
`runs/codex-claim-release-step4-kernel-filter-profile-v1`.

## Working Diagnosis

- The intended full-profile rerun
  `runs/codex-claim-release-full-kernel-aggregation-v1`
  moved materially farther than `full-nu-profile-v1` without an RSS failure,
  but it still never reached step `5`.
- That rerun invalidated the old "final plateau" read:
  - `39 groups / 144845 candidates` still first activates at `24`
  - `40 groups / 147639 candidates` first appears at `74`
  - `41 groups / 154842 candidates` first appears at `140`
- The follow-up short diagnostic rerun
  `runs/codex-claim-release-step4-kernel-filter-profile-v1`
  reproduced that first reopen on stored short evidence:
  - `40 groups / 147639 candidates` first appeared at `74`
  - by `76`, observed RSS was `383098880` and `frontier_queue_len = 2699`
- That short rerun also showed the new clause-filter timer was real, but not
  sufficient to explain `terminal_summary_build_millis`:
  - at `76`,
    `terminal_summary_build_millis = 1617942`
  - tracked subphases only summed to
    `148422 + 328562 + 213831 + 133277 = 824092`
- Because the same checkpoint had already accumulated
  `352904` pre-materialization rank prunes, the large remainder is now best
  explained by per-summary millisecond truncation across the current
  remaining-one counter surface, not by one newly isolated hot sub-phase.
- That means the next honest move is not another blind connectivity-side,
  aggregation-side, or clause-filter-side throughput patch first.
- The next honest move is a higher-fidelity late-surface kernel diagnostic on
  the winning binary so the reopened `40/147639` surface becomes measurable
  without per-call millisecond truncation.

## Goal

Re-earn one stored late-surface short step-`4` diagnostic on the winning
binary that measures the reopened `40/147639` retained-prefix surface with
enough fidelity to identify which sub-phase really dominates there.

## Do This Next

### 1. Keep The Winning Runtime Baselines Landed

Keep the code behind:

- `runs/codex-claim-release-step4-kernel-aggregation-v1`
- `runs/codex-claim-release-full-kernel-aggregation-v1`

Keep the temporary clause-filter timing patch dropped.

Do not reopen first:

- another prep-side candidate-remap rewrite
- the dropped clause-side profile-precompute shape
- another expr-keyed hot-path cache
- another ordering or proof-close handoff experiment
- another post-plateau summary-skip or summary-cache-reuse variant
- another throughput patch justified only by the current truncated
  millisecond counters

### 2. Land A Higher-Fidelity Late-Surface Diagnostic

Add one narrow diagnostic-only measurement change that keeps the winning
runtime behavior unchanged but accumulates the remaining-one hot-path
sub-phases without per-summary millisecond truncation.

At minimum, expose accurate late-surface totals for:

- terminal-prefix clause filtering
- connectivity decision time
- aggregation or rank-bookkeeping time
- exact `nu`
- total summary-build time

Prefer accumulating `Duration` or finer-grained totals internally and only
converting for reporting at checkpoint emission time.

### 3. Re-Earn The Short Diagnostic Read

Run a release claim rerun derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `--until-step 4`
- the winning binary
- live checkpoint persistence left on
- a new run id that states the higher-fidelity diagnostic intent, for example:
  `runs/codex-claim-release-step4-kernel-late-profile-v1`

Let the run go far enough to capture at least:

- matched `24/43/44/54` checkpoints
- the first reopened `40 groups / 147639 candidates` surface at `74/76`

If it reaches the later `41/154842` reopen cheaply, keep that evidence too.

### 4. Read The Stored Diagnostic Artifacts

Open at least:

- `reports/steps/step-04-live.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:

- on the reopened `40/147639` surface, which measured sub-phase now dominates
  honestly?
- does the higher-fidelity split close most of the old build-gap read?
- is the next patch connectivity-side, aggregation-side, clause-filter-side,
  or some still-untracked bookkeeping cut?
- does the reopened short surface still track the intended full-profile read?

### 5. Re-Earn Only The Validation Needed For That Diagnostic

Use the already-kept full-profile and short reruns as baseline.
If the next slice stays diagnostic-only, re-run only the focused claim test set
plus the `pen-cli` live-checkpoint persistence test.

## Keep Or Branch Decision

After the higher-fidelity late-surface diagnostic:

- stay on runtime work only if it isolates a real step-`4` blocker that still
  needs a throughput patch
- branch to parity, breadth, compare, benchmark, and certification work only
  if a later rerun honestly moves past the step-`4` wall

## Stop Condition For This Note

Rewrite this file as soon as one new stored late-surface diagnostic shows one
of these is true:

- the reopened `40/147639` surface is now attributed to one dominant
  sub-phase honestly
- the step-`4` wall has moved again
- the next throughput patch target is now explicit from stored evidence
- the lane has moved far enough that runtime work is no longer the next honest
  move
