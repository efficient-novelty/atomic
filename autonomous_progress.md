# Autonomous Claim Lane Progress

Last updated: 2026-04-03

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
- The latest finished full-profile run,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v3`,
  reached step `14` and failed with
  `failure_note = "no atomic candidates were generated for step 14"`.
- The same longer rerun reused the preserved release binary hash
  `278c311ddf5e416b09d24923dc392388aaf5817c65f0c60f856ebde7466140a5`
  from repo head `6f3bc1a00fb6ff46e048109b2a5176542105ab73`.
- A fresh full-profile rerun,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v4`,
  was launched on clean-tree repo head
  `140297377964dab9e0333782af3eec370bd784e7` with validated release binary
  hash
  `d3601f87cea1ff639d7c2ed19e604b1a815a65374790f6240910f7bebf3a711f`,
  then manually stopped after it entered step `14`.
- No residual `pen-cli.exe` process remains from that rerun.
- Because the process was killed externally, its persisted `run.json` is now a
  stale last-write snapshot that still says:
  - `status = "running"`
  - `completed_step = 13`
  - `active_step = 14`
  - `active_band = 3`
  - `frontier_epoch = 10`
- Use `reports/steps/step-04-live.ndjson`, `reports/latest.txt`, and
  `reports/steps/step-14-live.ndjson` as the authoritative frozen evidence for
  that stopped rerun.
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
  - the raw structural tie-break on that divergent step-`13` surface is
    therefore the live fork, not a looser step-`14` exact-screen rule
- A new claim-only late-step acceptance repair now lands that fork in code:
  - under the divergent steps `10..12` history, the raw step-`13`
    structural tie-break still picks a dead-end `kappa = 3`, `nu = 23` shell
  - the same step-`13` surface still has exactly one other survivor in the
    same primary acceptance tier (`overshoot` plus `clause_kappa`) that keeps
    step `14` alive
  - `desktop_claim_shadow` now prefers that viable shell before applying the
    secondary structural tie-breakers
- The targeted claim regression slice is green on repo tests:
  - divergent step-`14` reproducer
  - full-sweep exact-prune family split
  - hybrid step-`13` cutover
  - step-`13` structural-delta regression
  - forced local step-`13` operator-band reopen
  - late-step step-`13` viability-tie acceptance regression
- The claim live-checkpoint / step-summary persistence check is also green on
  repo tests.
- The release replay harness is now re-earned on the tracked plateau corpus:
  `target/release/xtask.exe claim-replay-harness benchmark
  tests/fixtures/claim_runtime/remaining_one_plateau_fixtures.json 10`
  completed without any replay mismatch on all `5` stored surfaces.
- The short pre-flight gate was rechecked again this turn on that clean tree:
  - targeted claim regressions green
  - claim live-checkpoint persistence green
  - release replay harness benchmark replays all `5` stored surfaces
- A new capped intended-profile validation read,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-late-accept-capped-v1`,
  now exists on clean-tree repo head `44b9871e65546a210c4ed71dcd31b91f8e6c521c`
  with release binary hash
  `d3601f87cea1ff639d7c2ed19e604b1a815a65374790f6240910f7bebf3a711f`.
- That capped read was intentionally stopped after an external `1260 s` wall,
  so its `run.json` still shows `status = "running"` and the authoritative
  evidence is the stored `reports/steps/step-04-live.ndjson` stream.
- The last stored checkpoint at or before `1200000 ms` on that run is:
  - `elapsed_millis = 1199122`
  - `prefix_states_explored = 141`
  - `prefix_cache_groups = 41`
  - `prefix_cache_candidates = 29249`
  - `frontier_queue_len = 2634`
  - RSS `= 466993152`
  - `terminal_summary_build_millis = 1191657`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- The same capped read re-entered the older `41 groups / 29249 candidates`
  continuation surface by prefix state `140` at `1190118 ms`:
  - `1778 ms` slower than `long-rerun-v1` at the same `140`-state wall
  - `5881 ms` faster than `long-rerun-v2` at the same `140`-state wall
- The stopped `v4` rerun re-earned the old step-`4` honesty gate before
  acceptance:
  - the last stored checkpoint at or before `1200000 ms` reached
    `139` explored prefixes, `40` cache groups, `28438` cached candidates,
    queue `2636`, RSS `459812864`, and kept both
    `terminal_summary_admissibility_checks = 0` and
    `terminal_summary_fallback_connectivity_checks = 0`
  - the first stored checkpoint at `140` explored prefixes reached the older
    `41 groups / 29249 candidates` continuation surface at `1202537 ms` with
    RSS `463085568`
- The same rerun then completed steps `4` through `13`, accepted step `13` at
  `nu = 19`, `kappa = 3`, and entered real step-`14` search before the manual
  stop.
- Its frozen step-`14` root seeding disproves the old `long-rerun-v3`
  zero-candidate opening as the first blocker:
  - `raw_catalog_clause_widths = [168,168,168]`
  - `roots_seen = 90`
  - `roots_rejected_by_insert_root = 5`
  - `roots_enqueued = 85`
  - `generated_raw_surface = 90`
  - `frontier_queue_len = 85`
- The old earliest replay fork is now repaired locally in code and tests:
  - `DesktopClaimShadow` now preserves the guarded structural focus packages
    through steps `4..8` before later claim-debt widening resumes
  - targeted profile-path checks now keep accepted hash plus `nu / kappa`
    parity through steps `4..8`
  - targeted smoke-config checks now keep that same parity through the
    demo-enabled early-exhaustive path
  - the claim live-checkpoint persistence test and the stored replay-harness
    benchmark are still green after the repair
- Stored full-profile replay parity is still open on disk:
  - no fresh clean-start rerun exists yet on the repaired binary
  - the frozen `v1` / `v2` / `v3` / capped / stopped-`v4` artifacts therefore
    remain the pre-repair evidence set

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
- The stored failure is still the admitted `kappa = 9` family that first
  appears once step `13` diverges, but the code now has a local repair for the
  raw step-`13` acceptance fork that used to strand step `14`.
- The stopped `v4` rerun supersedes the old "fresh rerun first" plan:
  step `14` now opens and seeds roots on the repaired path, so the next
  blocker is the earlier replay divergence that already starts at step `4`.

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

### Capped Intended-Profile Validation Read

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-late-accept-capped-v1`
- Launch surface:
  - clean-tree repo head
    `44b9871e65546a210c4ed71dcd31b91f8e6c521c`
  - release binary hash
    `d3601f87cea1ff639d7c2ed19e604b1a815a65374790f6240910f7bebf3a711f`
  - externally stopped after `1260 s`, so use `step-04-live.ndjson` as the
    authoritative record
- Last stored checkpoint at or before `1200000 ms`:
  - `elapsed_millis = 1199122`
  - `prefix_states_explored = 141`
  - `prefix_cache_groups = 41`
  - `prefix_cache_candidates = 29249`
  - `frontier_queue_len = 2634`
  - RSS `= 466993152`
  - `terminal_summary_build_millis = 1191657`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- Matching continuation wall on the same run:
  - prefix state `140` reached at `1190118 ms`
  - `prefix_cache_groups = 41`
  - `prefix_cache_candidates = 29249`
  - RSS `= 463450112`
  - `terminal_summary_build_millis = 1182710`
- Last stored checkpoint before the external stop:
  - `elapsed_millis = 1258330`
  - `prefix_states_explored = 148`
  - `prefix_cache_groups = 41`
  - `prefix_cache_candidates = 29249`
  - `frontier_queue_len = 2627`
  - RSS `= 487518208`
  - `terminal_summary_build_millis = 1250519`

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
- The late-step claim correctness slice still matters, but the stopped `v4`
  rerun moved the first blocker earlier:
  - the old `long-rerun-v3` step-`14` `generated_raw_surface = 0` opening is
    no longer the first failure surface
  - the stopped `v4` rerun already re-earned the `1200000 ms` step-`4` gate,
    completed through step `13`, and entered nonzero step-`14` search
- The early claim-versus-guarded policy split is now closed locally:
  - the repaired claim admissibility keeps the guarded former-eliminator /
    hit / sphere package progression through steps `4..8`
  - that closes the old step-`4` accepted-hash fork on both the plain profile
    path and the smoke-config early-exhaustive path
- The replay / runtime validation surface is still stronger too:
  - the release replay harness cleanly replays the tracked
    `remaining_one_plateau` corpus on all `5` stored surfaces
  - the capped intended-profile read is already past the old `139`-prefix
    short-loop gate by `1200000 ms`
  - that same read is back on the older `41 groups / 29249 candidates`
    continuation surface by prefix state `140`
- The next honest engineering dollar is therefore re-earning stored
  full-profile evidence on the repaired binary:
  - launch a fresh clean-start intended-profile rerun
  - compare early step `4` first against the capped honesty gate and the
    stored continuation walls
  - only then decide whether the surviving blocker is still late-step exact
    screening, runtime, or both

## Forward Direction

- Keep the current short-loop gate, step-`4` continuation references, capped
  intended-profile read, and stopped `v4` bundle frozen as regression checks.
- Keep the landed late-step diagnostics, divergent-prefix reproducer, replay
  harness corpus, step-`13` viability-tie regression, and early parity checks
  green.
- Prioritize a fresh clean-start full-profile rerun on the repaired binary;
  do not spend more local diagnosis time on the old early-step fork unless the
  rerun produces new contradictory evidence.
- Compare early step `4` first against the capped honesty gate and the stored
  continuation walls, then compare late step `14` against `long-rerun-v3`
  only after the early gate still holds.
- Return to step-`4` micro-optimization only if the repaired parity slice
  materially regresses the capped honesty gate on stored rerun evidence.

## Immediate Next Move

1. Freeze
   `clause-accept-rank-facts-v1`,
   `clause-accept-rank-facts-long-rerun-v1`,
   `clause-accept-rank-facts-long-rerun-v2`,
   `clause-accept-rank-facts-long-rerun-v3`,
   `clause-accept-rank-facts-late-accept-capped-v1`, and the stopped
   `clause-accept-rank-facts-long-rerun-v4`
   bundle as the current evidence set.
2. Keep the landed divergent-prefix reproducer, full-sweep exact-prune split,
   hybrid cutover, structural-delta, forced local reopen, late-step claim
   acceptance regressions, claim live-checkpoint persistence, and release
   replay harness green.
3. Launch a fresh clean-start full-profile rerun on the repaired binary; do
   not `resume` the stopped `v4` bundle.
4. On that rerun, compare early step `4` first against the capped
   `1200000 ms` gate plus the stored `v1` / `v2` continuation walls.
5. Only after the early gate still holds, compare late step `14` against
   `long-rerun-v3` and decide whether the surviving blocker is exact
   screening, runtime, or both.
