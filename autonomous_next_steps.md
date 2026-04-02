# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-02

This note is the exact next work order for `desktop_claim_shadow`.

## Keep Fixed

- Keep the current short baseline:
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`
- Keep the current short-loop gate:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`
- Keep the current later-wall step-`4` continuation reference through `576`:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`
- Keep the current corroborating middle-wall read through `335`:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v2`
- Keep the older farthest stored step-`4` continuation stop:
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
- Keep the replay harness corpus and benchmark files:
  - `tests/fixtures/claim_runtime/remaining_one_plateau_fixtures.json`
  - `tests/fixtures/claim_runtime/remaining_one_plateau_benchmark.json`
- Keep the latest finished full-profile failure for diagnosis:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v3`
- Keep the preserved release binary hash that all three longer reruns reused:
  `278c311ddf5e416b09d24923dc392388aaf5817c65f0c60f856ebde7466140a5`

## Current Read

- `long-rerun-v3` is finished and no longer running.
- Its authoritative `run.json` state is:
  - `status = "failed"`
  - `failure_note = "no atomic candidates were generated for step 14"`
  - `completed_step = 13`
  - `active_step = 14`
- The run therefore answered the old runtime question:
  the current preserved claim binary can escape step `4`, reach step `5`, and
  advance all the way to step `14`.
- Step `13` is not the new blocker. Its stored summary completed in `336 ms`,
  exact-screened `4353` terminals, closed `99%` of the frontier, and accepted
  a `kappa = 3`, `nu = 19` candidate.
- The accepted claim path is already drifting before the failure:
  - step `10`: `nu_delta = -5`, `clause_kappa_delta = -1`
  - step `11`: `nu_delta = -10`, `clause_kappa_delta = -2`
  - step `12`: `nu_delta = -17`, `clause_kappa_delta = -3`
  - step `13`: `nu_delta = -27`, `clause_kappa_delta = -4`
- Step `14` never entered real search on the stored full-profile run. Its only
  live checkpoint there shows:
  - `clause_kappa = 7`
  - `raw_catalog_clause_widths = [3,1,1,1,1,1,1]`
  - `raw_catalog_telescope_count = 3`
  - `generated_raw_surface = 0`
  - `frontier_queue_len = 0`
  - `candidate_pool_len = 0`
- The code now lands the missing late-step diagnostics:
  - `claim_step_open` on claim live checkpoints and step summaries
  - `claim_root_seeding` on claim live checkpoints and step summaries
  - enriched late-step zero-candidate failure notes with claim band plus root
    counts
- The new targeted reproducer now promotes and re-localizes that divergent
  step-`14` opening:
  - claim band `9..9`
  - late-family surface `claim_generic`
  - operator-band opener `= true`
  - hilbert / temporal openers `= false`
  - raw debt axes still `kappa = 7..7`
  - `raw_catalog_clause_widths = [1,3,1,1,1,3,3,3,3]`
  - `roots_seen = 1`
  - `roots_enqueued = 1`
  - `raw_generated_surface = 40`
  - `prefixes_created = 40`
  - `prefix_states_explored = 10`
  - `partial_prefix_bound_checks = 21`
  - `partial_prefix_bound_prunes = 21`
  - `remaining_one_algebraic_prunes = 0`
  - `terminal_summary_build_millis > 0`
- The next blocker is therefore exact partial-prefix screening on the promoted
  step-`14` claim Hilbert band, not claim band selection or root generation.

## Do This Next

### 1. Do Not Launch Another Rerun First

1. Freeze `long-rerun-v1`, `long-rerun-v2`, and `long-rerun-v3` as the current
   stored evidence set.
2. Do not launch `v4` first.
3. Do not prefer `pen-cli resume` first.
   The earlier longer runs still do not expose a resumable step-`4` frontier
   generation, and the immediate question is now late-step failure repair.

### 2. Use The Landed Diagnostics First

1. Keep the landed `claim_step_open` and `claim_root_seeding` payloads in
   place while debugging the fix.
2. Use them to decide whether the step-`14` zero-frontier loss comes from:
   - claim terminal filtering / terminal admissibility
   - exact terminal-summary bound construction
   - `exact_terminal_prefix_bound_decision_from_bound(...)`
   - direct exact bar-clearance on the divergent accepted history
3. Treat the new enriched failure-note shape as part of the regression surface.

### 3. Build A Reproducer Around The Finished Failure

1. Keep the landed reproducer around the finished claim history green:
   reference steps `1..9`, then the stored divergent simple chain on
   steps `10..13`.
2. Keep the promoted reproducer expectations green:
   - `claim_step_open = 9..9`
   - `claim_debt_axes = 7..7`
   - `roots_enqueued = 1`
   - `remaining_one_algebraic_prunes = 0`
3. Capture the first remaining-one step-`14` prefixes that still get
   `CannotClearBar` and compare:
   - the compact terminal-summary bound
   - the direct exact assessment on the same prefix
4. Decide whether the remaining zero-candidate failure comes from:
   - claim terminal filtering / terminal admissibility
   - exact terminal-summary bound construction
   - `exact_terminal_prefix_bound_decision_from_bound(...)`
   - a genuinely non-winning divergent accepted history

### 4. Validation Order After The Fix

1. Run the targeted claim tests needed by the slice.
   Keep at least:
   - the divergent step-`14` reproducer
   - the late-step claim acceptance regression
   - the live-checkpoint / step-summary persistence checks
2. Re-run the replay harness in release mode.
3. If parity holds, spend one capped intended-profile rerun and compare first
   against the current `20`-minute gate from
   `clause-accept-rank-facts-v1`:
   - `elapsed_millis = 1191501`
   - `prefix_states_explored = 139`
   - `prefix_cache_groups = 40`
   - `prefix_cache_candidates = 28438`
   - `frontier_queue_len = 2636`
   - RSS `= 453021696`
   - `terminal_summary_build_millis = 1183915`
   - `terminal_summary_admissibility_checks = 0`
   - `terminal_summary_fallback_connectivity_checks = 0`
4. Only if that capped read stays honest should the lane spend another fresh
   full-profile rerun to check whether step `14` is now viable and whether the
   run reaches step `15`.

### 5. Optimization Order After Step-14 Is Viable

1. First priority: keep the restored non-zero step-`14` search and repair the
   surviving exact-screen blocker on that promoted Hilbert-band surface.
2. Second priority: if the failure is fixed and the lane is back to being
   runtime-limited, return to the compact terminal-summary path, especially its
   connectivity, exact-`nu`, and aggregation costs.
3. Do not revisit step-`4` micro-optimization first while the late-step claim
   path is still broken.

## Do Not Reopen First

- another fresh long rerun
- dormant cached-summary reopen wake-up work
- broad frontier rewrites
- the dropped focus-aligned competition-gate/payload-mode hoist
- the dropped shared compact-bookkeeping fold
- the dropped claim-open-band compact local-state hoist
- contender-rank helper rewrites
- timing-only slices with no new late-step explanation or new step-`14`
  localization

## Keep Or Branch Decision

- Stay code-first until the step-`14` zero-candidate failure is explained and
  fixed.
- Keep the current short-loop gate and stored step-`4` continuation references
  frozen as regression checks.
- Return to long reruns only after a regression-backed late-step fix keeps
  claim replay parity and re-earns the capped intended-profile read honestly.
