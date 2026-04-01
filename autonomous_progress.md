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
- The current winner wins by cutting active exact `nu` work on the retained
  plateau while preserving the honest retained-prefix story.
- Aggregation is still the lead measured bucket, so exact-`nu` cuts alone are
  not enough; future wins need to keep shaving the hit path without shifting
  too much cost back into connectivity or generic aggregation bookkeeping.
- The replay harness is now a permanent iteration layer, but the latest local
  benchmark is a caution rather than a keep-proof on its own:
  the five-surface total rose from `153124 us` to `163951 us` even though the
  full-profile winner still improved materially on real later surfaces.

## Forward Direction

- Make the local continuation cone the primary scoring object.
  Use
  `SingleClauseStructuralNuContext`,
  `TerminalClauseNuFacts`, and
  `structural_nu_single_clause_upper_bound`
  to carry cheap exact or exact-safe prefix-local interval bounds on
  bar-clearability and overshoot before terminal assembly.
- Stop reconstructing clause facts on the winning path.
  `TerminalClauseNuFacts` should be a mandatory clause-catalog sidecar on the
  hot path, and `structural_nu_with_clause(...)` should stay off that path.
- Split the step-`4` remaining-one kernel into a true no-miss hit-path kernel
  and a general fallback kernel.
- Compress `lib_refs` more aggressively inside
  `SingleClauseStructuralNuContext`.
  Preferred shape:
  inline small array first, dense bitset after a threshold, boxed sorted
  slices only where serialization or debug parity needs them.
- Prefer a tiny survivor sketch over waking dormant general reopen machinery if
  second-pass duplication still matters after the facts-only slice.
- Require the replay harness to veto weak slices:
  before any future full-profile rerun, the slice should show either fewer
  exact-`nu` evaluations or lower measured aggregation time on the stored
  plateau fixtures.

## Immediate Next Move

1. Keep the current winner frozen as the speed target through the stored
   `576` wall.
2. Do not spend another turn on a plain rerun with no code change.
3. Reopen code work step by step in this order:
   - prefix-local interval pruning on the local continuation cone
   - mandatory predecoded clause-facts sidecar on the hot path
   - explicit hit-path plateau kernel versus fallback kernel split
   - tiered `lib_refs` representation
   - tiny survivor sketch only if second-pass duplication still matters
4. After each code slice:
   - rerun only the claim-focused tests needed by the change
   - require replay-harness parity plus either fewer exact-`nu` evaluations or
     lower measured aggregation time on the stored fixtures
   - then rerun the full profile and re-earn `140/163`, then
     `332/335/408/437/454/484`, and ideally move materially past `576` toward
     `1038` or step `5`
5. Keep these categories dropped until a new stored read says otherwise:
   cached-summary reopen wake-up work, contender-rank helper rewrites,
   connectivity-first/cache-first rewrites, deterministic batched parallel
   reduction, and broad metadata or bookkeeping cleanup.
