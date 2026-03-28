# Autonomous Claim Lane: Next Operational Slice

This note is the exact work order for the next `desktop_claim_shadow` slice.
Assume delayed materialization, the incumbent-primary remaining-one fast path,
the one-pass `structural_nu` summary-build fast path, the algebraic `nu`
ceiling patch, the family-agnostic claim terminal-admissibility shortcut, the
exact non-allocating connectivity summary scan, the terminal-only cached
parent connectivity decision, the aggregation-side accept-rank short-circuit,
and the higher-fidelity late-surface timing accumulation are already landed.
Assume the context-equivalence quotient, frontier-pop ordering, exact-two-step
local ordering, proof-close handoff ordering, post-plateau summary-skip,
post-plateau materialize-side gating, post-plateau summary-cache reuse,
expr-keyed terminal-clause caching, clause-side connectivity profile
precompute, terminal-candidate prep or remap cuts, and the temporary
filter-only timing patch were all measured on stored reruns and then dropped
after failing keep.

The current short step-`4` baseline remains
`runs/codex-claim-release-step4-kernel-aggregation-v1`.
The previous short baseline remains
`runs/codex-claim-release-step4-kernel-connectivity-v2`.
The current full-profile baseline remains
`runs/codex-claim-release-full-kernel-aggregation-v1`.
The previous full-profile baseline remains
`runs/codex-claim-release-full-nu-profile-v1`.
The most recent informative late-surface diagnostic is now
`runs/codex-claim-release-step4-kernel-late-profile-v1`.
The previous informative late-surface diagnostic remains
`runs/codex-claim-release-step4-kernel-filter-profile-v1`.

## Working Diagnosis

- The new short diagnostic rerun
  `runs/codex-claim-release-step4-kernel-late-profile-v1`
  kept the same honest early retained plateau, reopened
  `40 groups / 147639 candidates` at `74`, and then reached
  `41 groups / 154842 candidates` at `140`.
- That rerun still tracks the intended full-profile baseline closely on the
  reopened surfaces:
  - at `74`, short `elapsed_millis = 1790703`,
    observed RSS `= 378155008`, queue `= 2701`;
    full baseline `= 1743244 / 374198272 / 2701`
  - at `76`, short `elapsed_millis = 1848102`,
    observed RSS `= 382369792`, queue `= 2699`;
    full baseline `= 1797441 / 379269120 / 2699`
  - at `140`, short `elapsed_millis = 3535425`,
    observed RSS `= 570712064`, queue `= 2635`;
    full baseline `= 3393970 / 566460416 / 2635`
- The higher-fidelity split closes most of the old reopened-surface build-gap
  read:
  - at `76`,
    `terminal_summary_build_micros = 1839910636`
  - tracked subphases now sum to
    `352203534 + 416766880 + 469431036 + 267574759 = 1505976209`
  - leaving only `333934427` microseconds unexplained, about `18.15%` of
    build time instead of the older roughly `49.07%` gap
- On the reopened `40/147639` surface, aggregation or rank-bookkeeping is now
  the honest dominant measured phase:
  - cumulative at `76`:
    aggregation `= 469431036 us`,
    connectivity `= 416766880 us`,
    clause filtering `= 352203534 us`,
    exact `nu` `= 267574759 us`
  - incremental `54 -> 76`:
    aggregation `+141716373 us`,
    connectivity `+124894828 us`,
    clause filtering `+107776335 us`,
    exact `nu` `+80574865 us`
- That means the next honest move is no longer another diagnostic-only slice
  first.
- The next honest move is one narrow aggregation-side or rank-bookkeeping
  throughput cut on the winning binary, while treating the smaller remaining
  build gap as secondary untracked bookkeeping rather than the primary target.

## Goal

Land one narrow remaining-one aggregation or rank-bookkeeping cut that lowers
the reopened `40/147639` retained-prefix surface honestly on stored short
evidence without weakening the retained-prefix shape.

## Do This Next

### 1. Keep The Winning Baselines And Diagnostic Surface Landed

Keep the code behind:

- `runs/codex-claim-release-step4-kernel-aggregation-v1`
- `runs/codex-claim-release-full-kernel-aggregation-v1`
- `runs/codex-claim-release-step4-kernel-late-profile-v1`

Keep the temporary filter-only timing patch dropped.

Do not reopen first:

- another connectivity-side rewrite
- another clause-filter-side rewrite
- another prep-side candidate-remap rewrite
- the dropped clause-side profile-precompute shape
- another expr-keyed hot-path cache
- another ordering or proof-close handoff experiment
- another post-plateau summary-skip or summary-cache-reuse variant
- another telemetry-only slice before an aggregation hypothesis is measured

### 2. Land One Narrow Aggregation-Side Cut

Stay inside the remaining-one summary loop after connectivity and around the
compact bound or rank bookkeeping.

Prefer cuts that reduce:

- primary-rank or best-rank bookkeeping churn
- bound absorption or merge overhead
- summary-side evaluation or record construction that remains non-winning
  after the primary-rank check
- other compact summary aggregation work exposed by
  `runs/codex-claim-release-step4-kernel-late-profile-v1`

Reject as primary moves:

- another connectivity-side cut first
- another clause-filter-side cut first
- another exact-`nu` cleanup first unless aggregation proves flat on the next
  candidate
- another diagnostic-only pass first

### 3. Re-Earn The Short Runtime Read

Run a release claim rerun derived from
`configs/desktop_claim_shadow_1h.toml` with:

- `--until-step 4`
- the winning binary plus the new aggregation-side cut
- live checkpoint persistence left on
- a new run id that states the patch, for example:
  `runs/codex-claim-release-step4-kernel-aggregation-v2`
  or
  `runs/codex-claim-release-step4-kernel-rank-bookkeeping-v1`

Let the run go far enough to capture at least:

- matched `24/43/44/54` checkpoints
- the reopened `40 groups / 147639 candidates` surface at `74/76`
- the later `41/154842` reopen at `140` if it still arrives cheaply

### 4. Read The Stored Runtime Artifacts

Open at least:

- `reports/steps/step-04-live.ndjson`
- `reports/latest.txt`
- `run.json`

Answer from stored evidence:

- does the new cut lower elapsed and summary-build time at the matched
  `24/43/44/54` checkpoints without losing `39/144845`?
- does the reopened `40/147639` surface improve enough to justify keep?
- is aggregation still first on `54 -> 76`, or has connectivity,
  clause filtering, or the smaller bookkeeping tail become the next target?
- does a later full-profile rerun still remain the next runtime gate after the
  short read?

### 5. Re-Earn Only The Validation Needed For That Runtime Slice

Use the already-kept full-profile and short reruns as baseline.
If the next slice stays claim-only and step-`4` runtime-only, re-run only the
focused claim test set plus the `pen-cli` live-checkpoint persistence test.
If the new runtime cut earns keep, then re-enter one full
`desktop_claim_shadow_1h` rerun before parity, breadth, compare, benchmark,
or certification work.

## Keep Or Branch Decision

After the next short aggregation-side rerun:

- stay on runtime work only if step `4` still blocks the intended profile
- branch back to full-profile completion only if the new short rerun earns
  keep honestly
- branch to parity, breadth, compare, benchmark, and certification work only
  after a later full-profile rerun honestly moves past the step-`4` wall

## Stop Condition For This Note

Rewrite this file as soon as one new stored short rerun shows one of these is
true:

- an aggregation-side or rank-bookkeeping cut earns keep on the reopened short
  surface
- the step-`4` wall moves away from aggregation first
- the full-profile rerun reaches a later blocker honestly
- runtime work is no longer the next honest move
