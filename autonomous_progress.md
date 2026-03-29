# Autonomous Claim Lane Progress
Last updated: 2026-03-29

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final gate.

## Current Status
- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is `runs/codex-claim-release-step4-kernel-aggregation-v1`.
- The current full-profile baseline is `runs/codex-claim-release-full-kernel-aggregation-v1`.
- The authoritative late-surface diagnostic is `runs/codex-claim-release-step4-kernel-late-profile-v1`.
- The latest measured slice is `runs/codex-claim-release-step4-kernel-clause-metadata-v1`, and it was dropped from code after failing keep.
- The lane is still compute-bound in step `4`, but the eager clause-metadata retry moved the visible wall to clause filtering first.

## What Stays Landed
- delayed materialization
- the incumbent-primary remaining-one fast path
- the one-pass `structural_nu` summary-build fast path
- the algebraic `nu` ceiling patch
- the family-agnostic claim terminal-admissibility shortcut
- the exact non-allocating connectivity summary scan
- the terminal-only cached parent connectivity decision
- the aggregation-side accept-rank short-circuit that skips full `AcceptRank` construction for primary-dominated bar-clearers
- the higher-fidelity late-surface timing accumulation used by the current short diagnostic surface

## Baselines That Matter
### 1. Current Short Baseline
- Run: `runs/codex-claim-release-step4-kernel-aggregation-v1`
- Honest retained shape: `39 groups / 144845 candidates` at `24/43/44/54`
- Matched checkpoints:
  - `24`: `elapsed_millis = 549630`, `terminal_summary_build_millis = 492524`
  - `43`: `elapsed_millis = 990480`, `terminal_summary_build_millis = 892772`
  - `44`: `elapsed_millis = 1012067`, `terminal_summary_build_millis = 912271`
  - `54`: `elapsed_millis = 1247600`, `terminal_summary_build_millis = 1126754`

### 2. Current Full-Profile Baseline
- Run: `runs/codex-claim-release-full-kernel-aggregation-v1`
- It materially improved the old intended-profile baseline at `24/43/44/54` and proved that the winning short kernel shape survives on the real `desktop_claim_shadow_1h` profile.
- The retained prefix cache first plateaued at `39 groups / 144845 candidates` from `24` through `73`, then reopened to:
  - `40 groups / 147639 candidates` at `74`
  - `41 groups / 154842 candidates` at `140`
- The run never reached step `5`.
- Stored reopened checkpoints:
  - `74`: `elapsed_millis = 1743244`, `terminal_summary_build_millis = 1579138`
  - `76`: `elapsed_millis = 1797441`, `terminal_summary_build_millis = 1628768`
- The last stored step-`4` checkpoint was `163` with:
  - `elapsed_millis = 3942636`
  - `observed_process_rss_bytes = 632197120`
  - `frontier_queue_len = 2612`
  - `terminal_summary_build_millis = 3584914`

### 3. Current Late-Surface Diagnostic
- Run: `runs/codex-claim-release-step4-kernel-late-profile-v1`
- It reproduces the intended-profile reopens on short evidence and still tracks the full-profile baseline closely at `74/76/140`.
- At `76`:
  - `terminal_summary_build_micros = 1839910636`
  - clause filtering `= 352203534 us`
  - aggregation `= 469431036 us`
  - connectivity `= 416766880 us`
  - exact `nu` `= 267574759 us`
- Incremental `54 -> 76`:
  - clause filtering `+107776335 us`
  - aggregation `+141716373 us`
  - connectivity `+124894828 us`
  - exact `nu` `+80574865 us`

### 4. Latest Failed Slice
- Run: `runs/codex-claim-release-step4-kernel-clause-metadata-v1`
- Hypothesis: land one exact terminal-clause metadata pack plus one prefix-side exact rank context so the summary loop can reuse bit-cost, structural-signal, max-var-ref, and canonical-key inputs and make canonical finalization last-tie only.
- Outcome: preserved the same honest early plateau and reopened `74/76` shape, but regressed catastrophically on runtime and moved the visible wall to clause filtering first, so it did not earn keep.
- Comparison versus the kept short baseline:
  - `24`: `1385075 / 1067196` instead of `549630 / 492524`
  - `43`: `2474918 / 1930943` instead of `990480 / 892772`
  - `44`: `2534606 / 1975294` instead of `1012067 / 912271`
  - `54`: `3077025 / 2439389` instead of `1247600 / 1126754`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Reopened-surface read:
  - `74`: `elapsed_millis = 4181583`, `terminal_summary_build_micros = 3351564019`
  - `76`: `elapsed_millis = 4322812`, `terminal_summary_build_micros = 3445171148`
- At `76`, the measured bucket order became:
  - clause filtering `= 2178547522 us`
  - aggregation `= 456894681 us`
  - connectivity `= 412251293 us`
  - exact `nu` `= 269107583 us`
- Inside the new aggregation split, incremental `54 -> 76` was led by:
  - clause load / scratch update `+58123032 us`
  - admissibility bookkeeping `+21360371 us`
  - primary-rank math `+13290870 us`
  - bound update `+10799021 us`
  - bit-cost recovery `+10749565 us`
  - full `AcceptRank` construction `+513476 us`
  - canonical-key finalization `+22 us`
- Honest read: the metadata itself was not free. Building it inside `terminal_prefix_clause_candidates` made clause filtering explode and more than erased the later accept-rank savings.

## What Stays Dropped
- Ordering and reuse variants: `context-equivalence-v1`, `incumbent-ordering-v1`, `local-two-step-order-v2`, `proof-close-handoff-v1`, `post-plateau-v1`, `post-plateau-materialize-v1`, `post-plateau-summary-cache-v3`
- Connectivity-side cache and precompute variants: `kernel-connectivity-v3`, `kernel-connectivity-v4`
- Candidate-prep / remap variants: `terminal-candidate-prep-v1`
- Telemetry-only slice: `kernel-filter-profile-v1`
- Exact primary-rank bookkeeping rewrite: `kernel-rank-bookkeeping-v1`
- Bound-merge bookkeeping rewrite: `kernel-bound-merge-v1`
- Lazy incumbent-tie `AcceptRank` deferral rewrite: `kernel-lazy-acceptrank-v1`
- Summary-side batching rewrite: `kernel-summary-batching-v1`
- Summary-side bookkeeping rewrite: `kernel-summary-bookkeeping-v1`
- Prefix-wide competition-gate hoist: `kernel-competition-hoist-v1`
- Exact-`nu` high-water gate: `kernel-nu-highwater-v1`
- Summary-invariants accept-rank prefix-context rewrite: `kernel-summary-invariants-v1`
- Summary-rank-context exact tie-break rewrite: `kernel-rank-context-v1`
- Terminal-rank sidecar exact contender-rank rewrite: `kernel-terminal-rank-sidecar-v1`
- Primary-rank context exact-threshold rewrite: `kernel-primary-context-v1`
- Summary-constant bit-cost hoist: `kernel-summary-constant-v1`
- Catalog-backed clause bit-cost sidecar: `kernel-catalog-constant-v1`
- Bar-clear threshold bookkeeping rewrite: `kernel-summary-threshold-v1`
- Eager terminal-clause metadata pack: `kernel-clause-metadata-v1`

## Revised Working Diagnosis
- The old early RSS cliff remains broken; this is still a step-`4` throughput problem, not a return of the allocator-failure story.
- The kept baselines still say the intended profile later reopens beyond the early `39/144845` plateau.
- The eager metadata-pack rerun proved that the full-telescope accept-rank rebuild is not the dominant wall once exact clause metadata already exists:
  - canonical-key finalization became almost invisible
  - full `AcceptRank` construction also became small
- But the same rerun also proved that moving exact clause metadata construction into terminal clause filtering is too early. That shift made clause filtering the first measured cost and swamped the remaining summary kernel.

## Best Current Inference
The next honest retry should keep the winning baseline code and keep only the lesson from the failed slice:

> if exact terminal-clause metadata is revisited, it must be built lazily behind connectivity and admitted-candidate checks, not in `terminal_prefix_clause_candidates`.

That means the next plausible keep slice is not another eager clause-filter-wide metadata pack. It is a narrower admitted-only metadata path that:
- leaves terminal clause filtering cheap
- still allows numeric-first exact rank comparison
- and only finalizes canonical keys on true last-tie contenders

## Immediate Next Move
1. Keep the code behind `runs/codex-claim-release-step4-kernel-aggregation-v1`, `runs/codex-claim-release-full-kernel-aggregation-v1`, and `runs/codex-claim-release-step4-kernel-late-profile-v1`.
2. Do not keep the eager clause-filter-wide metadata patch in code.
3. Land, if possible, one lazy admitted-only metadata retry inside the summary kernel rather than inside terminal clause filtering.
4. Re-run a short release claim slice to `--until-step 4` and read the stored artifacts at `24/43/44/54/74/76`.
5. Only branch back to a new full-profile rerun if that short slice earns keep against the matched early plateau and materially narrows the reopened `74/76` gap to the kept full-profile aggregation baseline.

## What Has Not Changed
- Do not branch to compare, benchmark, certification, or stronger language before step `4` moves or a full-profile run finishes.
- Do not reopen allocator-first, frontier-first, or broad connectivity-first work as the next primary move.
- Do not replace the current short baseline with an unmeasured or diagnostic-only hypothesis.
