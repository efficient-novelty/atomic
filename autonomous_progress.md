# Autonomous Claim Lane Progress

Last updated: 2026-04-02

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to
signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final
gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current short-loop gate is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`.
- The current later-wall step-`4` continuation reference through `576` is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`.
- The current corroborating middle-wall read through `335` is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v2`.
- The older farthest stored step-`4` continuation stop remains
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
  at `1095` explored prefixes.
- The latest full-profile run,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v3`,
  reached step `14` and failed with
  `failure_note = "no atomic candidates were generated for step 14"`.
- The same longer rerun reused the preserved release binary hash
  `278c311ddf5e416b09d24923dc392388aaf5817c65f0c60f856ebde7466140a5`
  from repo head `6f3bc1a00fb6ff46e048109b2a5176542105ab73`.
- The claim lane now records late-step `claim_step_open` diagnostics in live
  checkpoints and persisted step summaries:
  - `kappa_min` / `kappa_max`
  - claim debt axes / pressures
  - late-family surface
  - historical anchor
  - package flags for operator / hilbert / temporal-band openers
- The claim lane now records `claim_root_seeding` diagnostics between
  `claim_regular_clause_catalog` and the first frontier push:
  - `roots_seen`
  - `roots_rejected_by_insert_root`
  - `roots_rejected_by_exact_screen`
  - `roots_enqueued`
- Late-step zero-candidate failures now enrich the thrown failure note with the
  claim band plus those root-seeding counts, so the next failed run will not
  collapse to a bare `no atomic candidates` string.
- A targeted regression now reproduces the stored `long-rerun-v3` step-`14`
  failure from the divergent accepted prefix:
  reference steps `1..9`, then the stored simple chain on steps `10..13`.

## Latest Full-Profile Outcome

- `long-rerun-v3` is finished and no longer active.
- Its authoritative `run.json` state is:
  - `status = "failed"`
  - `completed_step = 13`
  - `active_step = 14`
  - `active_band = 3`
  - `frontier_epoch = 10`
- The run answered the old runtime question:
  the current preserved claim binary can escape step `4`, reach step `5`, and
  advance to step `14`.
- Step `13` itself completed quickly on stored evidence:
  - `step_wall_clock_millis = 336`
  - `exact_bound_screened = 4353`
  - `frontier_certified_nonwinning = 4351`
  - `closure_percent = 99`
  - accepted `kappa = 3`, `nu = 19`
- The accepted claim path is already diverging before the failure:
  - step `10`: `nu_delta = -5`, `clause_kappa_delta = -1`
  - step `11`: `nu_delta = -10`, `clause_kappa_delta = -2`
  - step `12`: `nu_delta = -17`, `clause_kappa_delta = -3`
  - step `13`: `nu_delta = -27`, `clause_kappa_delta = -4`
- Step `14` never entered real search. Its only live checkpoint shows:
  - `clause_kappa = 7`
  - `raw_catalog_clause_widths = [3,1,1,1,1,1,1]`
  - `raw_catalog_telescope_count = 3`
  - `generated_raw_surface = 0`
  - `frontier_queue_len = 0`
  - `candidate_pool_len = 0`
- The new reproducer now confirms the same divergent late-step opening in test:
  - claim band `7..7`
  - late-family surface `claim_generic`
  - operator-band opener `= true`
  - hilbert / temporal openers `= false`
  - `roots_seen = 3`
  - `roots_enqueued = 0`
- The immediate blocker is therefore late-step claim viability and root
  generation, not another step-`4` continuation read.

## Current Reference Runs

### Short-Loop Gate

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`
- Nearest stored checkpoint to `1200000 ms`:
  - `elapsed_millis = 1191501`
  - `prefix_states_explored = 139`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 28438`
  - `frontier_queue_len = 2636`
  - RSS `= 453021696`
  - `terminal_summary_build_millis = 1183915`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`

### Later-Wall Step-4 Reference Through `576`

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`
- Key stored later walls:
  - `140`: `1188340 / 1181188` with `41 groups / 29249 candidates`
  - `163`: `1381425 / 1373168` with `41 groups / 29249 candidates`
  - `332`: `2853118 / 2836752` with `42 groups / 29529 candidates`
  - `335`: `2879368 / 2862850` with `43 groups / 29809 candidates`
  - `408`: `3511930 / 3491831` with `43 groups / 29809 candidates`
  - `437`: `3770964 / 3749419` with `43 groups / 29809 candidates`
  - `454`: `3922561 / 3900177` with `43 groups / 29809 candidates`
  - `484`: `4183978 / 4160100` with `43 groups / 29809 candidates`
  - `576`: `4997082 / 4968579` with `43 groups / 29809 candidates`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.

### Corroborating Middle-Wall Read Through `335`

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v2`
- Matched corroborating walls:
  - `140`: `1195999 / 1188793` with `41 groups / 29249 candidates`
  - `163`: `1390249 / 1381921` with `41 groups / 29249 candidates`
  - `332`: `2874771 / 2858216` with `42 groups / 29529 candidates`
  - `335`: `2901198 / 2884494` with `43 groups / 29809 candidates`

## Landed Winning Stack

- delayed materialization
- incumbent-primary remaining-one fast path
- prefix-side single-clause structural-`nu` context with tiered `lib_refs`
- shared terminal-clause connectivity-facts sidecar on the clause catalog
- shared terminal-clause structural-`nu` facts sidecar threaded through the
  remaining-one bound, summary, and materialization paths
- terminal-clause bit-cost sidecar cached inside `TerminalClauseNuFacts`
- explicit remaining-one no-miss plateau kernel
- compact claim open-band aggregation fast path
- aggregation-side accept-rank short-circuit for primary-dominated bar-clearers
- prefix-local accept-rank context reused across compact remaining-one no-miss
  branches
- clause-side accept-rank facts reused across compact remaining-one no-miss
  branches
- broadened compact remaining-one survivor sketch with cached materialization
  reuse, while the dormant general cached-summary reopen path stays off
- deterministic replay harness plus stored retained plateau fixtures and
  benchmark

## Current Diagnosis

- The old early RSS cliff remains gone.
- Step `4` summary build is still the main measured runtime bucket when the run
  is inside step `4`, but `long-rerun-v3` proved that the current preserved
  binary can clear that wall and advance into late steps.
- The next blocker is late-step claim correctness/viability, not inability to
  escape step `4`.
- The strongest current clues are:
  - widening replay divergence on steps `10` through `13`
  - the step-`14` opening band/candidate mismatch
  - the collapse from a raw clause catalog to zero generated roots before any
    frontier work begins
  - the new claim-open diagnostics show that the divergent history still
    carries the operator-band opener but never reaches the reference-style
    Hilbert `kappa = 9` band
- The likely diagnosis surface is the path between:
  - claim debt / admissibility band selection
  - late-step clause-catalog construction
  - `PrefixLegalityCache::insert_root(...)`
  - `screen_prefix_for_frontier(...)`
- The compact terminal-summary path remains worth optimizing later, but it is
  no longer the first engineering dollar to spend.

## Forward Direction

- Keep the current short-loop gate and step-`4` continuation references frozen
  as regression checks.
- Do not launch `v4` first.
- Do not prioritize another step-`4` micro-optimization before the step-`14`
  failure is explained.
- Keep the landed late-step diagnostics and divergent-prefix reproducer green
  while localizing the zero-frontier loss.
- Use the new claim-open and root-seeding payloads to decide whether the next
  narrow fix belongs in band selection, clause-catalog construction,
  `insert_root(...)`, or `screen_prefix_for_frontier(...)`.
- After that fix lands and replay parity holds, spend one capped intended
  rerun before committing to another full long rerun.

## Immediate Next Move

1. Freeze
   `clause-accept-rank-facts-v1`,
   `clause-accept-rank-facts-long-rerun-v1`,
   `clause-accept-rank-facts-long-rerun-v2`, and
   `clause-accept-rank-facts-long-rerun-v3`
   as the current evidence set.
2. Run the landed divergent-prefix reproducer and inspect the new
   `claim_step_open` plus `claim_root_seeding` payloads together.
3. Localize whether the three lost step-`14` roots die in
   `insert_root(...)`, exact-screen rejection, or earlier band/catalog
   selection.
4. Patch the narrowest honest late-step path that restores non-zero step-`14`
   search without waking guarded, replay, realistic-shadow, or demo-only
   fallback behavior.
5. Run the targeted claim tests for the reproducer plus the late-step claim
   acceptance surface.
6. Only if that stays clean, spend one capped intended-profile rerun against
   the `139`-prefix short-loop gate before authorizing another full rerun.
