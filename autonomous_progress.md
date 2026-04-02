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
- Claim admissibility now promotes the divergent step-`14` reproducer from the
  raw operator-stage debt band (`claim_debt_axes = 7..7`) into the effective
  claim Hilbert band (`claim_step_open = 9..9`) once the late-step structural
  pressure is saturated.
- The claim Hilbert-band remaining-one algebraic ceiling now stands down on
  that promoted step-`14` surface, so exact terminal summary work can run
  instead of pruning the reproducer before summary build.
- A targeted regression now reproduces the stored `long-rerun-v3` step-`14`
  failure from the divergent accepted prefix and now localizes the surviving
  blocker:
  reference steps `1..9`, then the stored simple chain on steps `10..13`.
- New exact-screen regressions now cover both the first captured
  remaining-one step-`14` prefixes and the full `21`-prefix divergent sweep:
  - the first captured prefixes still compare cached compact summary against
    direct exact checks on the same prefix surface
  - the full sweep now proves the `21` exact prunes split into three honest
    direct-exact families:
    - `9` prefixes with `3` admitted candidates at `exact_nu = 40`,
      `clause_kappa = 9`
    - `2` prefixes with `0` admitted terminal candidates and no cached bound
    - `10` prefixes with `3` admitted candidates at `exact_nu = 41`,
      `clause_kappa = 9`
- A new hybrid-prefix regression now traces the earliest late-step cutover:
  - the pure reference step-`13` history still reaches step `14`
  - that winning reference surface already carries `54` zero-admitted
    remaining-one exact prunes and no cached compact bounds
  - replacing only step `13` with the stored divergent acceptance is already
    enough to make step `14` fail
  - that step-`13` cutover preserves the same `54` zero-admitted baseline but
    adds `27` admitted `kappa = 9` prunes:
    - `9` at `exact_nu = 50`
    - `18` at `exact_nu = 51`
  - earlier divergences on steps `10..12` only deform that already-failing
    admitted family; they are not needed to trigger the failure
- A new step-`13` structural-delta regression now explains that cutover:
  - the reference step-`13` winner stays at `kappa = 7`, `nu = 46`,
    `lib_refs = {11,12}`, and keeps metric capability
  - replacing only step `13` with the stored divergent acceptance drops that
    slot to `kappa = 3`, `nu = 29`, `lib_refs = {11}`, and no metric
    capability
  - step `14` therefore reopens operator-bundle claim debt instead of opening
    the reference Hilbert-functional package
  - the admitted `50/9` and `51/9` families now localize to that reopened
    operator-bundle surface
- A new forced step-`13` operator-band regression now closes the remaining
  diagnosis fork:
  - under the already-divergent steps `10..12`, the natural claim step-`13`
    band is still only `3..3`
  - forcing a local `7..7` reopen at step `13` still enumerates roots but
    exact-screen prunes all of them before any root is enqueued
  - that forced local repair yields zero surviving candidates
  - the honest next fix therefore belongs in upstream history repair /
    backtracking, not in loosening the current step-`14` exact-screen path
- The targeted claim regression slice is green on repo tests:
  - divergent step-`14` reproducer
  - full-sweep exact-prune family split
  - hybrid step-`13` cutover
  - step-`13` structural-delta regression
  - forced local step-`13` operator-band reopen

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
- Step `14` never entered real search on the stored full-profile run. Its only
  live checkpoint there shows:
  - `clause_kappa = 7`
  - `raw_catalog_clause_widths = [3,1,1,1,1,1,1]`
  - `raw_catalog_telescope_count = 3`
  - `generated_raw_surface = 0`
  - `frontier_queue_len = 0`
  - `candidate_pool_len = 0`
- The new reproducer now advances that same divergent late-step opening farther
  than the stored run ever reached:
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
- The captured surviving exact prunes are now explained on stored test
  evidence:
  - the first `9` captured prefixes keep cached compact bounds that match
    direct exact, with `3` admitted terminal candidates at
    `exact_nu = 40`, `clause_kappa = 9`
  - the next `2` captured prefixes admit zero terminal candidates and
    therefore carry no cached bound
  - the final `10` captured prefixes again keep cached compact bounds that
    match direct exact, but top out at `exact_nu = 41`,
    `clause_kappa = 9`
  - all `21` remain honest `CannotClearBar` outcomes; none hide a bar clearer
    behind claim terminal filtering
- The immediate blocker is now the admitted `kappa = 9` failure family that
  first appears once step `13` diverges, not claim band selection, root
  generation, or the zero-admitted family that the reference path already
  survives.

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
  - the step-`14` band mismatch is now locally repaired in test:
    `claim_step_open = 9..9` while `claim_debt_axes` still report `7..7`
  - root generation is no longer the live blocker on the reproducer:
    one promoted Hilbert-band root survives `insert_root(...)` and enters the
    frontier
  - the claim Hilbert-band algebraic ceiling is no longer the live blocker on
    that reproducer:
    `remaining_one_algebraic_prunes = 0` and terminal summary build now runs
  - the reproducer still dies with `21` exact partial-prefix prunes, zero
    terminal bar prunes, and zero terminal rank prunes
- The captured remaining-one exact prunes are no longer ambiguous:
  - `19` of the `21` captured prefixes retain cached compact bounds that
    match direct exact assessment
  - raw and terminal-filtered exact walks agree on all `21` captured prunes
  - the sweep now splits into three honest families:
    - `9` prefixes with `3` admitted candidates at `exact_nu = 40`,
      `clause_kappa = 9`
    - `2` prefixes with `0` admitted terminal candidates and no bound
    - `10` prefixes with `3` admitted candidates at `exact_nu = 41`,
      `clause_kappa = 9`
- The hybrid cutover is now no longer ambiguous either:
  - the reference step-`14` winner already coexists with a `54`-prefix
    zero-admitted remaining-one prune family
  - step `13` is the earliest divergence that flips step `14` into failure
  - the new step-`13` structural-delta regression now shows why:
    - the reference winner closes operator-bundle debt with a metric-bearing
      `kappa = 7`, `nu = 46`, `lib_refs = {11,12}` shell
    - the step-`13`-only cutover swaps in a non-metric
      `kappa = 3`, `nu = 29`, `lib_refs = {11}` shell
    - step `14` therefore reopens operator-bundle debt instead of the
      reference Hilbert-functional package
    - the admitted `50/9` and `51/9` families live on that reopened
      operator-bundle surface
- The remaining branch decision is now closed too:
  - a forced local step-`13` `7..7` reopen on the already-divergent
    step-`10..12` history still exact-screens every root before enqueue
  - no local step-`13` repair candidate survives that divergent history
  - the current step-`14` exact-screen path therefore still looks honest on
    stored test evidence
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
- Treat the new forced step-`13` operator-band regression as closing that
  branch decision:
  the remaining narrow fix is upstream history repair / backtracking, not a
  looser step-`14` exact-screen rule.
- After that fix lands and replay parity holds, spend one capped intended
  rerun before committing to another full long rerun.

## Immediate Next Move

1. Freeze
   `clause-accept-rank-facts-v1`,
   `clause-accept-rank-facts-long-rerun-v1`,
   `clause-accept-rank-facts-long-rerun-v2`, and
   `clause-accept-rank-facts-long-rerun-v3`
   as the current evidence set.
2. Keep the landed divergent-prefix reproducer green with the promoted
   `claim_step_open = 9..9`, `roots_enqueued = 1`, and
   `remaining_one_algebraic_prunes = 0` state.
3. Keep the new full-sweep exact-prune regression green. It now proves that
   the `21` captured remaining-one exact prunes split into:
   - `9` prefixes with `3` admitted candidates at `40/9`
   - `2` prefixes with `0` admitted terminal candidates and no bound
   - `10` prefixes with `3` admitted candidates at `41/9`
4. Keep the new hybrid cutover regression green. It now proves that:
   - the reference step-`14` winner already carries `54` zero-admitted prunes
   - step `13` is the first divergence that flips step `14` into failure
   - that cutover adds `27` admitted `kappa = 9` prunes at `50/9` and `51/9`
5. Keep the new step-`13` structural-delta regression green. It now proves
   that:
   - the reference step-`13` winner closes operator-bundle debt with a
     metric-bearing `kappa = 7`, `nu = 46`, `lib_refs = {11,12}` shell
   - the step-`13`-only cutover swaps in a non-metric
     `kappa = 3`, `nu = 29`, `lib_refs = {11}` shell
   - step `14` therefore reopens operator-bundle debt instead of the
     reference Hilbert-functional package
   - the admitted `50/9` and `51/9` families live on that reopened
     operator-bundle surface
6. Keep the new forced local step-`13` operator-band regression green. It now
   proves that even reopening `7..7` locally on the already-divergent
   step-`10..12` history still exact-screens every root before enqueue.
7. Treat that branch decision as closed:
   the honest narrow fix now belongs in upstream history repair /
   backtracking, not in loosening the current step-`14` exact-screen path.
8. Only after that fix lands, rerun the targeted claim tests plus replay
   parity for the reproducer and late-step claim surface.
9. Only if that stays clean, spend one capped intended-profile rerun against
   the `139`-prefix short-loop gate before authorizing another full rerun.
