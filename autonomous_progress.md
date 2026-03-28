# Autonomous Claim Lane Progress

Last updated: 2026-03-28

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to
signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final
gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-aggregation-v1`.
- The current full-profile baseline is
  `runs/codex-claim-release-full-kernel-aggregation-v1`.
- The authoritative late-surface diagnostic is
  `runs/codex-claim-release-step4-kernel-late-profile-v1`.
- The latest measured slice is
  `runs/codex-claim-release-step4-kernel-competition-hoist-v1`, and it was
  dropped from code after failing keep.
- The lane is still compute-bound in step `4` on the intended profile.
  The old early allocator-failure story is no longer the primary blocker.

## What Stays Landed

- delayed materialization
- the incumbent-primary remaining-one fast path
- the one-pass `structural_nu` summary-build fast path
- the algebraic `nu` ceiling patch
- the family-agnostic claim terminal-admissibility shortcut
- the exact non-allocating connectivity summary scan
- the terminal-only cached parent connectivity decision
- the aggregation-side accept-rank short-circuit that skips full
  `AcceptRank` construction for primary-dominated bar-clearers
- the higher-fidelity late-surface timing accumulation used by the current
  short diagnostic surface

## Baselines That Matter

### 1. Current Short Baseline

- Run:
  `runs/codex-claim-release-step4-kernel-aggregation-v1`
- Honest retained shape:
  `39 groups / 144845 candidates` at `24/43/44/54`
- Matched checkpoints:
  - `24`: `elapsed_millis = 549630`,
    `terminal_summary_build_millis = 492524`
  - `43`: `elapsed_millis = 990480`,
    `terminal_summary_build_millis = 892772`
  - `44`: `elapsed_millis = 1012067`,
    `terminal_summary_build_millis = 912271`
  - `54`: `elapsed_millis = 1247600`,
    `terminal_summary_build_millis = 1126754`

### 2. Current Full-Profile Baseline

- Run:
  `runs/codex-claim-release-full-kernel-aggregation-v1`
- It materially improved the old intended-profile baseline at `24/43/44/54`
  and proved that the winning short kernel shape survives on the real
  `desktop_claim_shadow_1h` profile.
- The retained prefix cache first plateaued at
  `39 groups / 144845 candidates` from `24` through `73`, then reopened to:
  - `40 groups / 147639 candidates` at `74`
  - `41 groups / 154842 candidates` at `140`
- The run never reached step `5`.
- The last stored step-`4` checkpoint was `163` with:
  - `elapsed_millis = 3942636`
  - `observed_process_rss_bytes = 632197120`
  - `frontier_queue_len = 2612`
  - `terminal_summary_build_millis = 3584914`

### 3. Current Late-Surface Diagnostic

- Run:
  `runs/codex-claim-release-step4-kernel-late-profile-v1`
- It reproduces the intended-profile reopens on short evidence and still tracks
  the full-profile baseline closely at `74/76/140`.
- At `76`:
  - `terminal_summary_build_micros = 1839910636`
  - aggregation `= 469431036 us`
  - connectivity `= 416766880 us`
  - clause filtering `= 352203534 us`
  - exact `nu` `= 267574759 us`
  - unexplained tail `= 333934427 us` (`18.15%`)
- Incremental `54 -> 76`:
  - aggregation `+141716373 us`
  - connectivity `+124894828 us`
  - clause filtering `+107776335 us`
  - exact `nu` `+80574865 us`

### 4. Latest Failed Slice

- Run:
  `runs/codex-claim-release-step4-kernel-competition-hoist-v1`
- Hypothesis:
  keep the current compact-summary flow, but hoist the prefix-wide competition
  gate and the same prefix-constant focus-gating read out of the per-admitted
  compact-summary path in
  `compute_terminal_prefix_completion_summary_from_candidates`, so admitted
  remaining-one candidates stop paying that repeated prefix-constant check on
  the reopened surface.
- Outcome:
  preserved the same honest shapes at `24/43/44/54/74/76`, including the
  reopened `40/147639` surface at `74/76`. It slightly improved elapsed time
  and total summary-build versus the late diagnostic on the reopened surface,
  but it materially regressed both wall clock and total
  `terminal_summary_build_*` versus the kept short baseline on the matched
  `24/43/44/54` surface, so it did not earn keep.
- Regressions versus the kept short baseline:
  - `24`: `566548 / 561409` instead of `549630 / 492524`
  - `43`: `1024730 / 1018566` instead of `990480 / 892772`
  - `44`: `1047008 / 1040791` instead of `1012067 / 912271`
  - `54`: `1290162 / 1283403` instead of `1247600 / 1126754`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Reopened-surface read:
  - `74`: `elapsed_millis = 1774558`,
    `terminal_summary_build_micros = 1766685530`
  - `76`: `elapsed_millis = 1830226`,
    `terminal_summary_build_micros = 1822244549`
  This beat the late diagnostic by about `16.1s / 16.0s` at `74` and
  `17.9s / 17.7s` at `76` on `elapsed / total summary-build`, but it still
  trailed the kept full-profile aggregation baseline by about
  `31.3s / 187.5s` at `74` and `32.8s / 193.5s` at `76` on
  `elapsed / total summary-build`.
- Incremental `54 -> 76` still kept aggregation first:
  - aggregation `+136696877 us`
  - connectivity `+119167500 us`
  - clause filtering `+106416113 us`
  - exact `nu` `+78957145 us`
- The run was terminated after storing live evidence through
  `prefix_states_explored = 76`. Because step `4` never sealed, the
  authoritative artifact for this slice is `reports/steps/step-04-live.ndjson`;
  `reports/latest.txt` and `run.json` remained at `step 3 / running`.

## What Stays Dropped

- Ordering and reuse variants:
  `context-equivalence-v1`, `incumbent-ordering-v1`,
  `local-two-step-order-v2`, `proof-close-handoff-v1`,
  `post-plateau-v1`, `post-plateau-materialize-v1`,
  `post-plateau-summary-cache-v3`
  These either never engaged honestly, found no real reuse, or only shifted
  cost without improving the retained-prefix surface.
- Connectivity-side cache and precompute variants:
  `kernel-connectivity-v3`, `kernel-connectivity-v4`
  These regressed despite attacking real work.
- Candidate-prep / remap variants:
  `terminal-candidate-prep-v1`
  This exposed real prep cost but did not improve the kept surface.
- Telemetry-only slice:
  `kernel-filter-profile-v1`
  Informative, but not a keep patch.
- Exact primary-rank bookkeeping rewrite:
  `kernel-rank-bookkeeping-v1`
  Honest shape, slower clock, dropped.
- Bound-merge bookkeeping rewrite:
  `kernel-bound-merge-v1`
  Honest shape, slightly better reopened `74/76` read than the late
  diagnostic, but materially worse than the kept short baseline at
  `24/43/44/54`, so no keep.
- Lazy incumbent-tie `AcceptRank` deferral rewrite:
  `kernel-lazy-acceptrank-v1`
  Honest early and reopened shapes, but materially worse than the kept short
  baseline at `24/43/44/54` and still behind `kernel-bound-merge-v1` at
  `74/76`, so no keep.
- Summary-side batching rewrite:
  `kernel-summary-batching-v1`
  Honest early and reopened shapes, materially better reopened `74/76`
  evidence than the late diagnostic and slightly ahead of
  `kernel-bound-merge-v1` there, but still clearly worse than the kept short
  baseline at `24/43/44/54`, so no keep.
- Summary-side bookkeeping rewrite:
  `kernel-summary-bookkeeping-v1`
  Honest early and reopened shapes, materially better elapsed time than the
  kept short baseline at `24/43/44/54` and materially better reopened `74/76`
  evidence than the late diagnostic plus `kernel-summary-batching-v1`, but it
  still regressed too much on total summary-build at `24/43/44/54` and still
  trailed the kept full-profile aggregation baseline on total summary-build at
  `74/76`, so no keep.
- Prefix-wide competition-gate hoist:
  `kernel-competition-hoist-v1`
  Honest early and reopened shapes, slightly better reopened `74/76`
  `elapsed / terminal_summary_build_*` than the late diagnostic, but
  materially worse than the kept short baseline at `24/43/44/54` and still
  behind the kept full-profile aggregation baseline at `74/76`, so no keep.
- Bar-clear threshold bookkeeping rewrite:
  `kernel-summary-threshold-v1`
  Honest shape, better aggregation microtime than the late diagnostic, but no
  keep because elapsed and total summary-build stayed behind the kept short
  baseline.

## Working Diagnosis

- The old early RSS cliff has already been broken by earlier landed memory and
  frontier compactions.
- The intended-profile blocker is now step-`4` summary throughput, not an early
  memory explosion.
- The earlier `39 groups / 144845 candidates` plateau is real, but it is not
  the terminal full-profile surface. The intended profile later reopens to
  `40/147639` and then `41/154842`.
- The honest late target is therefore the reopened `40/147639` surface, not
  only the early plateau.
- On that reopened surface, aggregation is still the largest measured cost,
  connectivity remains second, clause filtering third, and exact `nu` fourth.
- The latest failed threshold and bound-merge slices show that both the
  per-candidate bar-clear `Rational` gate and the per-candidate `PrefixBound`
  merge churn were real work, but neither was the dominant remaining wall.
- The current compact summary code already skips full `AcceptRank`
  construction for candidates whose primary rank is clearly worse than the
  current group best.
- The lazy incumbent-tie `AcceptRank` deferral slice kept the same honest
  early and reopened shapes, but exact tie-break construction was too small a
  share of the remaining summary-build wall to earn keep.
- The summary-side batching and later summary-bookkeeping slices show that
  compact-summary diagnostics or counter bookkeeping is real work, and they
  can improve elapsed time materially on the reopened `74/76` surface, but the
  bookkeeping-only shapes tried so far still leave too much total
  `terminal_summary_build_*` cost on the matched early `24/43/44/54` surface
  to keep.
- The summary-bookkeeping slice in particular improved elapsed time on both
  the matched early surface and the reopened `74/76` surface, which suggests
  some real overhead lived outside the kept baseline's wall-clock read, but
  because total summary-build still regressed materially on the matched early
  surface, the honest blocker is still inside measured summary-build work, not
  only in downstream discovery-count merging.
- The competition-hoist slice showed that the repeated prefix-wide
  focus-gating lookup is real work on the reopened surface, but it is too
  small on its own: the slice preserved the honest shapes and beat the late
  diagnostic slightly at `74/76`, yet it still regressed too much at
  `24/43/44/54` and stayed well behind the kept full-profile aggregation
  baseline on total summary-build at `74/76`.
- The next aggregation cut should therefore stay inside the compact summary
  path, but move to one different aggregation-side shape that removes a larger
  per-admitted compact-summary constant already charged inside the measured
  summary kernel rather than retrying another threshold-only, bound-only,
  tie-break-only, competition-gate-only, batching-only, or bookkeeping-only
  cleanup first.

## Immediate Next Move

1. Keep the code behind
   `runs/codex-claim-release-step4-kernel-aggregation-v1`,
   `runs/codex-claim-release-full-kernel-aggregation-v1`, and
   `runs/codex-claim-release-step4-kernel-late-profile-v1`.
2. Land one different narrow aggregation-side cut on the winning binary.
   The competition-gate hoist has now been tried and dropped.
   Prefer this next:
   - keep the current primary-rank short-circuit and stored diagnostics, but
     precompute one more compact-summary constant that is still rebuilt for
     every admitted candidate inside
     `compute_terminal_prefix_completion_summary_from_candidates`, so admitted
     candidates stop paying a larger per-admitted invariant on the reopened
     surface
   Do not use:
   - another retry of `kernel-competition-hoist-v1` as the next primary move
   - another retry of `kernel-lazy-acceptrank-v1` as the next primary move
   - another retry of `kernel-summary-batching-v1` as the next primary move
   - another retry of `kernel-summary-bookkeeping-v1` as the next primary move
   - a lossy hash or surrogate key in place of the exact tie-break
   - another bound-only cleanup as the next primary move
3. Re-earn one short release rerun from
   `configs/desktop_claim_shadow_1h.toml` with `--until-step 4` and a run id
   that names the new cut, preferably
   `runs/codex-claim-release-step4-kernel-summary-invariants-v1` next.
4. Capture at least `24/43/44/54/74/76`, and `140` only if it arrives cheaply.
5. Compare the new run against the current short baseline at `24/43/44/54`
   and against the late diagnostic plus full-profile baseline at `74/76`.
6. Only if the short rerun earns keep should the next move branch back to a
   new full-profile rerun. Otherwise stay on step-`4` runtime work.

## Runtime Slice Validation

- Re-run only:
  - `cargo test -p pen-search claim_`
  - `cargo test -p pen-cli claim_run_persists_live_step_memory_checkpoints_before_acceptance`

## Guardrails

- Do not reopen another connectivity-side rewrite first.
- Do not reopen another clause-filter-side rewrite first.
- Do not retry `kernel-rank-bookkeeping-v1` as the next primary move.
- Do not retry `kernel-bound-merge-v1` as the next primary move.
- Do not retry `kernel-lazy-acceptrank-v1` as the next primary move.
- Do not retry `kernel-summary-batching-v1` as the next primary move.
- Do not retry `kernel-summary-bookkeeping-v1` as the next primary move.
- Do not retry `kernel-competition-hoist-v1` as the next primary move.
- Do not retry `kernel-summary-threshold-v1` as the next primary move.
- Do not replace the exact compact-summary tie-break with a lossy hash or
  surrogate ordering key.
- Do not spend another turn on a diagnostic-only slice before an aggregation
  hypothesis is measured.
- Keep user-facing wording at `bounded live recovery`.
- Trust stored artifacts over terminal impressions.
