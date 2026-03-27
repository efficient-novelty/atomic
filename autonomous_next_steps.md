# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, the algebraic `nu`
ceiling patch, the family-agnostic claim terminal-admissibility shortcut, the
exact non-allocating connectivity summary scan, the terminal-only cached
parent connectivity decision, and the new aggregation-side accept-rank
short-circuit are already landed.
Assume the context-equivalence quotient, frontier-pop ordering, exact-two-step
local ordering, proof-close handoff ordering, post-plateau summary-skip,
post-plateau materialize-side gating, post-plateau summary-cache reuse,
expr-keyed terminal-clause caching, clause-side connectivity profile
precompute, and terminal-candidate prep/remap cuts were all measured on stored
short reruns and then dropped after failing keep.

The current short step-`4` baseline is now
`runs/codex-claim-release-step4-kernel-aggregation-v1`.
The previous short baseline is now
`runs/codex-claim-release-step4-kernel-connectivity-v2`.
The current full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.
The valid diagnostic split that still explains the hot wall is
`runs/codex-claim-release-step4-kernel-profile-v2`.

## Working Diagnosis

- One more narrow short step-`4` slice earned keep.
- The new kept rerun
  `runs/codex-claim-release-step4-kernel-aggregation-v1`
  preserved the same honest retained plateau:
  - retained prefix cache `= 39 groups / 144845 candidates`
  - first plateau activation at `prefix_states_explored = 24`
  - same plateau activations `= 97234` at `44`
- It improved the matched kept checkpoints against
  `runs/codex-claim-release-step4-kernel-connectivity-v2`:
  - at `24`: `elapsed_millis = 549630` instead of `551825`,
    `terminal_summary_build_millis = 492524` instead of `495256`,
    `terminal_summary_connectivity_millis = 88989` instead of `95969`
  - at `43`: `elapsed_millis = 990480` instead of `998555`,
    `terminal_summary_build_millis = 892772` instead of `901994`,
    `terminal_summary_connectivity_millis = 164940` instead of `178000`
  - at `44`: `elapsed_millis = 1012067` instead of `1020529`,
    `terminal_summary_build_millis = 912271` instead of `921924`,
    `terminal_summary_connectivity_millis = 169227` instead of `182453`
- Residual measured costs also stayed controlled:
  - aggregation fell to `67567/118700/120643`
    from `68266/119561/121524`
  - exact `nu` stayed flat at `39527/73331/74471`
    versus `39523/73348/74489`
  - `terminal_materialize_millis = 325` instead of `327`
  - fallback connectivity stayed `0`
- The honest short wall is therefore still step `4` summary compute,
  especially connectivity first and aggregation second, but the short-slice
  sequence has earned another real keep.
- That means the next honest move is now the intended full-profile rerun on
  the winning binary, not another short slice first.

## Goal

Re-earn one stored intended-profile `desktop_claim_shadow_1h` rerun on top of
the winning short baseline, then use its artifacts to decide whether:

- step `4` is still the real blocker on the intended profile,
- a later honest blocker is now exposed,
- or the run has moved far enough that parity, breadth, compare, benchmark,
  and certification work can resume.

## Do This Next

### 1. Keep The New Short Baseline Landed

Keep the code behind
`runs/codex-claim-release-step4-kernel-aggregation-v1`.

Do not reopen, in the next slice:

- another short step-`4` prep-side rewrite first
- the dropped clause-side profile-precompute shape from
  `runs/codex-claim-release-step4-kernel-connectivity-v4`
- another expr-keyed hot-path cache
- another ordering or proof-close handoff experiment
- another post-plateau summary-skip or summary-cache-reuse variant

### 2. Re-Earn The Intended Full-Profile Read

Run the claim profile derived from `configs/desktop_claim_shadow_1h.toml`
with:

- release build
- the intended auto-worker profile
- live checkpoint persistence left on
- a new run id that states the winning short baseline, for example:
  `runs/codex-claim-release-full-kernel-aggregation-v1`

### 3. Read The Stored Full-Profile Artifacts

Open at least:

- `reports/steps/step-04-live.ndjson`
- `reports/steps/step-05-live.ndjson` if the run reaches step `5`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:

- did the intended profile move materially farther than
  `runs/codex-claim-release-full-nu-profile-v1`?
- did step `4` still flatten onto the same honest `39/144845` plateau?
- if so, did the shorter `kernel-aggregation-v1` shape survive on the real
  full profile?
- if not, what later honest wall is now exposed?
- did observed RSS remain controlled on the full profile?
- is the next move now another short slice, or compare/benchmark/certify work?

### 4. Re-Earn Only The Validation Needed For That Full Read

Use the already-kept short rerun and already-re-earned test set as baseline.
If code is unchanged in the next slice, do not spend time re-running another
short release `until_step = 4` slice first.

## Keep Or Branch Decision

After the full-profile rerun:

- stay on runtime work only if the intended profile still exposes a real
  completion blocker
- move to compare, benchmark, certification, and breadth/parity closeout only
  if the stored full-profile bundle honestly supports that next step

## Stop Condition For This Note

Rewrite this file as soon as one new stored intended-profile rerun shows one
of these is true:

- step `4` is no longer the dominant blocker
- the full profile exposes a later honest wall
- the full profile finishes far enough that compare/benchmark/certification is
  now the next honest move
- the intended-profile read proves another short step-`4` slice is still
  necessary
