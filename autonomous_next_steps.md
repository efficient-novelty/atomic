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
- The first captured surviving exact prunes are now explained:
  - compact summary bound `= direct exact` bound on the terminal-filtered
    surface
  - raw filtered catalog exact walk `= same` bound
  - each of the first three captured prefixes has `3` admitted terminal
    candidates
  - each tops out at `exact_nu = 40` with `clause_kappa = 9`, so
    `rho = 40/9` and `CannotClearBar` is honest
- The full `21`-prune sweep is now also localized on stored test evidence:
  - `9` captured prefixes land in that same honest `40/9` family
  - `2` captured prefixes admit zero terminal candidates and retain no cached
    bound
  - `10` captured prefixes land in a later honest `41/9` family
  - raw and terminal-filtered exact walks agree on all `21` captured prunes
- A new hybrid-prefix cutover regression now localizes the earliest failure:
  - the pure reference step-`13` history still reaches step `14`
  - that winning reference surface already carries `54` zero-admitted
    remaining-one exact prunes
  - replacing only step `13` with the stored divergent acceptance is already
    enough to make step `14` fail
  - that cutover preserves the same `54` zero-admitted baseline but adds `27`
    admitted `kappa = 9` prunes at `50/9` and `51/9`
  - earlier divergences on steps `10..12` only deform that already-failing
    admitted family
- A new step-`13` structural-delta regression now explains that cutover:
  - the reference step-`13` winner stays at `kappa = 7`, `nu = 46`,
    `lib_refs = {11,12}`, and keeps metric capability
  - the step-`13`-only cutover swaps in a non-metric
    `kappa = 3`, `nu = 29`, `lib_refs = {11}` shell
  - step `14` therefore reopens operator-bundle claim debt instead of opening
    the reference Hilbert-functional package
  - the admitted `50/9` and `51/9` families now localize to that reopened
    operator-bundle surface
- A new forced step-`13` operator-band regression now closes the remaining
  diagnosis fork:
  - under the already-divergent steps `10..12`, the natural claim step-`13`
    band is still only `3..3`
  - forcing a local `7..7` reopen there still enumerates roots but
    exact-screen prunes all of them before any root is enqueued
  - that forced local repair yields zero surviving candidates
- A new claim-only late-step acceptance repair now lands the remaining fork:
  - under the divergent steps `10..12` history, the raw step-`13`
    structural tie-break still picks a dead-end `kappa = 3`, `nu = 23` shell
  - that same step-`13` surface still has exactly one other survivor in the
    same primary acceptance tier (`overshoot` plus `clause_kappa`) that keeps
    step `14` alive
  - `desktop_claim_shadow` now prefers that viable shell before the later
    structural tie-break fields
- The targeted claim regression slice is green on repo tests:
  - divergent step-`14` reproducer
  - full-sweep exact-prune family split
  - hybrid step-`13` cutover
  - step-`13` structural-delta regression
  - forced local step-`13` operator-band reopen
  - late-step step-`13` viability-tie acceptance regression
- The next blocker is now replay parity plus one capped intended-profile read
  on the repaired claim path, not more local diagnosis of the same step-`13`
  fork.

## Do This Next

### 1. Do Not Launch Another Rerun First

1. Freeze `long-rerun-v1`, `long-rerun-v2`, and `long-rerun-v3` as the current
   stored evidence set.
2. Do not launch `v4` first.
3. Do not prefer `pen-cli resume` first.
   The earlier longer runs still do not expose a resumable step-`4` frontier
   generation, and the immediate question is now late-step failure repair.

### 2. Use The Landed Diagnostics And Repair First

1. Keep the landed `claim_step_open` and `claim_root_seeding` payloads in
   place while debugging the fix.
2. Treat the new exact-screen regressions as part of the surface.
   They now prove that the first captured `40/9` prunes are not coming from:
   - claim terminal filtering / terminal admissibility
   - exact terminal-summary bound construction
   - `exact_terminal_prefix_bound_decision_from_bound(...)`
3. Treat the same diagnostics as having closed that branch decision:
   the remaining zero-frontier loss is now consistent with a genuinely
   non-winning divergent accepted history, not with a narrower step-`14`
   exact-screen bug on the currently tested surface.
4. Treat the enriched failure-note shape as part of the regression surface.
5. Treat the new late-step acceptance helper as part of the surface too:
   it is now the narrow upstream repair for the divergent step-`13`
   acceptance fork.

### 3. Build A Reproducer Around The Finished Failure

1. Keep the landed reproducer around the finished claim history green:
   reference steps `1..9`, then the stored divergent simple chain on
   steps `10..13`.
2. Keep the promoted reproducer expectations green:
   - `claim_step_open = 9..9`
   - `claim_debt_axes = 7..7`
   - `roots_enqueued = 1`
   - `remaining_one_algebraic_prunes = 0`
3. Keep the new first-prune capture green. For the first three captured
   remaining-one exact prunes it now proves:
   - compact summary bound `= direct exact` bound
   - raw filtered catalog exact walk `= same` bound
   - admitted candidates `= 3`
   - `exact_nu = 40`, `clause_kappa = 9`, so `rho = 40/9`
4. Keep the new full-sweep exact-prune capture green. It now proves that the
   `21` exact prunes split into:
   - `9` prefixes with `3` admitted candidates at `40/9`
   - `2` prefixes with `0` admitted terminal candidates and no cached bound
   - `10` prefixes with `3` admitted candidates at `41/9`
5. Keep the new hybrid cutover regression green. It now proves that:
   - the reference step-`14` winner already carries `54` zero-admitted
     remaining-one prunes
   - step `13` is the first divergence that flips step `14` into failure
   - that cutover adds `27` admitted `kappa = 9` prunes at `50/9` and `51/9`
6. Keep the new step-`13` structural-delta regression green. It now proves
   that:
   - the reference step-`13` winner closes operator-bundle debt with a
     metric-bearing `kappa = 7`, `nu = 46`, `lib_refs = {11,12}` shell
   - the step-`13`-only cutover swaps in a non-metric
     `kappa = 3`, `nu = 29`, `lib_refs = {11}` shell
   - step `14` therefore reopens operator-bundle debt instead of the
     reference Hilbert-functional package
   - the admitted `50/9` and `51/9` families live on that reopened
     operator-bundle surface
7. Keep the new forced local step-`13` operator-band regression green. It now
   proves that even reopening `7..7` locally on the already-divergent
   step-`10..12` history still exact-screens every root before enqueue.
8. Keep the new late-step claim acceptance regression green. It now proves
   that:
   - the raw divergent step-`13` structural tie-break still picks a dead-end
     shell
   - exactly one same-primary-tier survivor keeps step `14` alive
   - `desktop_claim_shadow` now chooses that viable shell
9. Treat the old branch decision as closed:
   the upstream history repair is now landed locally, so the next work is
   validation rather than more local band loosening.

### 4. Validation Order After The Fix

1. Keep the targeted claim tests needed by the slice green.
   The local repo run now covered at least:
   - the divergent step-`14` reproducer
   - the forced local step-`13` operator-band reopen regression
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
- local step-`13` band-widening experiments with no earlier-history repair
- contender-rank helper rewrites
- timing-only slices with no new late-step explanation or new step-`14`
  localization

## Keep Or Branch Decision

- Stay validation-first on the landed step-`13` acceptance repair until replay
  parity and a capped intended-profile rerun either confirm it or localize the
  next defect.
- Keep the current short-loop gate and stored step-`4` continuation references
  frozen as regression checks.
- Return to long reruns only after a regression-backed late-step fix keeps
  claim replay parity and re-earns the capped intended-profile read honestly.
