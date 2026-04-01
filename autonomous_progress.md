# Autonomous Claim Lane Progress

Last updated: 2026-04-01

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to
signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final
gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current full-profile speed winner to beat is
  `runs/codex-claim-release-full-aggregation-open-band-structural-nu-facts-v1`.
- The current deeper continuation target is
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`,
  because it still owns the farthest stored wall at `1038` even though it is
  slower than the new winner at matched later checkpoints.
- The newest landed code slice is
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`:
  a prefix-local continuation-cone scoring pass on the remaining-one hot path
  that now uses
  `SingleClauseStructuralNuContext` plus
  `TerminalClauseNuFacts`
  to make more bar-clearability and primary-overshoot decisions before scratch
  telescope assembly whenever connectivity needs no fallback and admissibility
  is already known.
- That same run id is now the active full-profile contender.
  `run.json` currently reports `status = "running"`,
  `completed_step = 3`, and `active_step = 4`.
- The previous broader full-profile runtime reference is
  `runs/codex-claim-release-full-aggregation-open-band-stage-timing-v1`.

## Current Winner

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-structural-nu-facts-v1`
- Status:
  manually stopped during step `4`; `reports/latest.txt` still reflects
  completed step `3`, `run.json` still says `status = "running"`, and
  `reports/steps/step-05-live.ndjson` is absent. The authoritative evidence is
  `reports/steps/step-04-live.ndjson`.
- Honest retained-prefix story through the stored `576` read:
  - `39 groups / 144845 candidates` at `24/43/44/54`
  - `40 groups / 147639 candidates` at `74/76`
  - `41 groups / 154842 candidates` from `140` through `331`
  - `42 groups / 157636 candidates` at `332/333/334`
  - `43 groups / 160430 candidates` from `335` through the stored `576` read
- Decisive matched later checkpoints versus
  `runs/codex-claim-release-full-aggregation-open-band-prefix-nu-context-v2`:
  - `140`: `1656717 / 1647017` instead of `1849510 / 1839797`
  - `163`: `1925819 / 1914639` instead of `2147103 / 2135876`
  - `332`: `4011498 / 3989370` instead of `4465654 / 4444493`
  - `335`: `4047111 / 4024786` instead of `4506796 / 4485426`
  - `408`: `4899116 / 4872728` instead of `5479859 / 5454313`
  - `437`: `5256846 / 5228815` instead of `5888660 / 5861474`
  - `454`: `5469278 / 5440222` instead of `6125662 / 6097497`
  - `484`: `5823948 / 5793152` instead of `6536061 / 6505941`
  - `533`: `6446054 / 6412428` instead of `7246571 / 7213520`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.
- Stored wall after stopping:
  - `576`: `6934404 / 6898343`, `43 groups / 160430 candidates`,
    RSS `= 1423429632` bytes
- Late-surface shape stayed the same at the current winner:
  aggregation first, connectivity second, exact `nu` third, terminal
  clause-filter handoff tiny.
- At the stored `533` read:
  - aggregation `= 3007387 ms`
  - connectivity `= 2281593 ms`
  - exact `nu` `= 410409 ms`
  - terminal clause-filter handoff `= 83505 ms`
  - RSS `= 1423474688` bytes
- Cached-summary reopen stayed dormant through the stored `576` read:
  - `remaining_one_materialized_from_cached_summary = 0`
  - `remaining_one_prefixes_seen = 0`
  - `remaining_one_materialized_compact_direct = 43`
- `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0` through the stored
  `576` read.

## New Local Read

- The first continuation-cone slice is now landed in code.
  Remaining-one exact bound checks, compact summary build, and compact
  materialization now fast-path prefix-local exact scoring whenever
  connectivity needs no fallback and admissibility is already known from the
  prefix-local surface.
- Claim-focused parity checks stayed green after the slice:
  - `claim_remaining_one_algebraic_ceiling_keeps_reference_step_four_winner_prefix`
  - `claim_terminal_prefix_completion_summary_matches_direct_exact_assessment`
  - `claim_replay_fixture_replays_compact_summary_with_parity`
  - `claim_hoisted_terminal_competition_gate_matches_direct_check`
  - `claim_materialization_reopens_after_cached_pruning_summary`
- The checked-in release replay benchmark on the stored plateau fixtures is now
  `150440 us` total across the five stored surfaces, down from `163951 us`.
  Surface deltas versus the prior checked-in read:
  - `24`: `34319 -> 29887`
  - `74`: `51575 -> 58992`
  - `140`: `27139 -> 20863`
  - `332`: `28566 -> 19680`
  - `335`: `22352 -> 21018`
- The replay read is therefore a real overall local improvement, but not a
  uniform per-surface win yet because `40 groups / 147639 candidates`
  regressed.
- A fresh full-profile contender is now running:
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`.
  The stored run directory already exists, and the run is currently in
  step `4`.

## What Stays Landed

- delayed materialization
- the incumbent-primary remaining-one fast path
- the prefix-side single-clause structural-`nu` context reused across
  remaining-one exact scoring, compact-summary build, and compact
  materialization
- the shared terminal-clause connectivity-facts sidecar on the clause catalog
- the shared terminal-clause structural-`nu` facts sidecar threaded through
  the clause catalog, filtered active-window clones, remaining-one
  bound/summary/materialization, and replay fixtures
- the prefix-local continuation-cone score fast path on remaining-one exact
  bound checks, compact summary build, and compact materialization when
  fallback connectivity is not needed
- the compact claim open-band aggregation fast path on the no-evaluations
  kernel
- the aggregation-side accept-rank short-circuit for primary-dominated
  bar-clearers
- the claim open-band terminal-clause handoff fast path on clause refs
- steady-state scratch-slot `clone_from` reuse on terminal-clause loads
- the boundary-timestamp timing pass on the compact summary kernel
- the deterministic replay harness plus stored retained plateau fixtures and
  benchmark

## Current Diagnosis

- The old early RSS cliff is still gone. This remains a step-`4` throughput
  problem, not a return of the allocator-failure story.
- The retained-prefix plateau after state `24` is real.
- The decisive later surfaces are now effectively no-miss surfaces:
  stored later reads keep
  `terminal_summary_admissibility_checks = 0` and
  `terminal_summary_fallback_connectivity_checks = 0`.
- The new continuation-cone slice is pointed at the right cost center:
  it makes more bar-clearability and primary-overshoot decisions from the
  current prefix before assembly, and the replay harness now shows an overall
  local win rather than the previous cautionary regression.
- Aggregation is still the lead measured bucket, so this is not done:
  the replay win is mixed at one stored surface, and the real question is
  whether the in-flight full rerun re-earns the stored later checkpoints
  through `576`.

## Forward Direction

- Read the in-flight full rerun before opening another code slice.
- If `prefix-local-score-v1` wins honestly through `484/533/576`, promote it
  and keep pushing that same run toward `1038` or step `5`.
- If it stalls before then, the next code slice is:
  make `TerminalClauseNuFacts` mandatory on every winning-path remaining-one
  evaluation so `structural_nu_with_clause_facts(...)` stays hot-path-only and
  `structural_nu_with_clause(...)` stays off that path.
- After that, split the explicit no-miss hit-path plateau kernel from the
  general fallback kernel.
- Keep `lib_refs` compression and any tiny survivor sketch work after those two
  slices, not before.

## Immediate Next Move

1. Keep the current winner frozen as the speed target through the stored
   `576` wall until the in-flight contender proves otherwise.
2. Read
   `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
   at `140/163`, then `332/335`, then `408/437/454/484`, then `533/576`.
3. If it wins honestly through `576`, keep driving that run toward `1038` or
   step `5`.
4. If it stalls or loses honestly before then, reopen code in this order:
   - mandatory `TerminalClauseNuFacts` sidecar on the hot path
   - explicit no-miss hit-path kernel versus fallback kernel split
   - tiered `lib_refs` representation
   - tiny survivor sketch only if second-pass duplication still matters
5. Keep these categories dropped until a new stored read says otherwise:
   cached-summary reopen wake-up work, contender-rank helper rewrites,
   connectivity-first/cache-first rewrites, deterministic batched parallel
   reduction, and broad metadata or bookkeeping cleanup.
