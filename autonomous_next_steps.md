# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-03

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
- Keep the latest capped intended-profile validation read:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-late-accept-capped-v1`
- Keep the stopped fresh full-profile rerun evidence path:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v4`
- Keep the preserved `long-rerun-v3` release binary hash:
  `278c311ddf5e416b09d24923dc392388aaf5817c65f0c60f856ebde7466140a5`
- Keep the validated capped-read / stopped-`v4` release binary hash:
  `d3601f87cea1ff639d7c2ed19e604b1a815a65374790f6240910f7bebf3a711f`

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
- The claim live-checkpoint / step-summary persistence check is also green on
  repo tests.
- The release replay harness is now re-earned on the tracked plateau corpus:
  `target/release/xtask.exe claim-replay-harness benchmark
  tests/fixtures/claim_runtime/remaining_one_plateau_fixtures.json 10`
  completed without any replay mismatch on all `5` stored surfaces.
- A new capped intended-profile validation read,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-late-accept-capped-v1`,
  now exists on clean-tree repo head
  `44b9871e65546a210c4ed71dcd31b91f8e6c521c`
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
- The short pre-flight gate was rerun this turn on clean-tree repo head
  `140297377964dab9e0333782af3eec370bd784e7`:
  - targeted claim regressions green
  - claim live-checkpoint persistence green
  - release replay harness benchmark replays all `5` stored surfaces
- A fresh clean-start full-profile rerun,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v4`,
  was manually stopped after it entered step `14`.
- No residual `pen-cli.exe` process remains from that rerun.
- Because the process was killed externally, its persisted `run.json` is now a
  stale last-write snapshot that still says:
  - `status = "running"`
  - `completed_step = 13`
  - `active_step = 14`
  - `active_band = 3`
  - `frontier_epoch = 10`
- Its frozen step-`4` evidence re-earned the old honesty/continuation gate:
  - the last stored checkpoint at or before `1200000 ms` reached
    `139` explored prefixes, `40` cache groups, `28438` cached candidates,
    queue `2636`, RSS `459812864`, and kept both zero-count summary checks at
    `0`
  - the first stored `140`-state checkpoint then re-entered the older
    `41 groups / 29249 candidates` continuation surface at `1202537 ms`
- The same stopped rerun then completed steps `4` through `13`, accepted
  step `13` at `nu = 19`, `kappa = 3`, and entered real step-`14` search.
- Its frozen step-`14` root seeding disproves the old zero-candidate opening
  as the first blocker:
  - `raw_catalog_clause_widths = [168,168,168]`
  - `roots_seen = 90`
  - `roots_rejected_by_insert_root = 5`
  - `roots_enqueued = 85`
  - `generated_raw_surface = 90`
  - `frontier_queue_len = 85`
- Replay parity is still not restored end-to-end:
  - steps `1..7` still match guarded `nu / kappa`
  - step `8` is the first visible `nu / kappa` mismatch:
    claim `11 / 3` versus guarded `18 / 5`
  - `replay_ablation = diverges_from_reference_replay` already starts at
    step `4`, so the structural fork predates the visible `nu / kappa` drift
- The replay-ablation root cause is now localized earlier than the late-step
  repair:
  - claim step `4` uses open-band claim admissibility instead of the guarded
    former-eliminator focus gate
  - with `[demo] enabled = true`, claim step `4` also runs the early
    exhaustive discovery branch
  - the claim step-`4` winner therefore comes from a wider structural surface
    and takes a different accepted hash before late-step divergence appears
- Replay parity plus the capped intended-profile read remain valuable local
  guards, but the current blocker is now fixing that early step-`4` replay
  fork before spending another full rerun.

## Do This Next

### 1. Freeze The Validation Evidence

1. Freeze `long-rerun-v1`, `long-rerun-v2`, and `long-rerun-v3` as the current
   stored evidence set.
2. Add
   `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-late-accept-capped-v1`
   and the stopped
   `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v4`
   bundle to that frozen evidence set.
3. Keep the replay harness corpus and benchmark files frozen; do not recapture
   fixtures first.

### 2. Treat The Validation Gate As Earned

1. Treat the targeted claim regressions, live-checkpoint persistence test, and
   release replay harness as the pre-flight gate for the next rerun.
2. Treat the capped intended-profile read as the new early honesty gate:
   - checkpoint at or before `1200000 ms` now reaches
     `141` explored prefixes
   - the run is already back on the older `41 groups / 29249 candidates`
     continuation surface by prefix state `140`
3. Treat `terminal_summary_admissibility_checks = 0` and
   `terminal_summary_fallback_connectivity_checks = 0` as part of that gate.
4. Treat the new release binary hash
   `d3601f87cea1ff639d7c2ed19e604b1a815a65374790f6240910f7bebf3a711f`
   as the validated binary for the next full rerun.

### 3. Repair The Earliest Replay Fork

1. Localize and repair the step-`4` claim-versus-guarded admissibility split.
2. Keep the demo-enabled early-exhaustive claim step-`4` path under the same
   parity check; do not treat the landed late-step viability regressions as
   sufficient proof of replay parity.
3. Re-run targeted parity checks that compare accepted hash plus `nu / kappa`
   through at least steps `4..8`.

### 4. Relaunch Only After Early Parity Holds

1. Start a fresh clean-start full-profile rerun only after that targeted
   parity slice is green.
2. Do not `resume` the stopped `v4` bundle.
3. On the next rerun, compare early step `4` first against the capped
   validation gate, then compare step `14` against `long-rerun-v3` only after
   early replay parity holds.

### 5. Optimization Order After Replay Parity

1. First priority: keep restored guarded replay parity through steps `4..8`
   while preserving the re-earned `1200000 ms` honesty gate.
2. Second priority: once parity holds, use the next full rerun to decide
   whether the surviving blocker is still late-step exact screening, runtime,
   or both.
3. Do not spend more time on late-step-only diagnosis before the early
   step-`4` fork is closed.

## Do Not Reopen First

- a `resume`-based restart of the stopped `v4` run
- another clean-start full rerun before the early step-`4` replay divergence
  is fixed or explicitly justified
- dormant cached-summary reopen wake-up work
- broad frontier rewrites
- the dropped focus-aligned competition-gate/payload-mode hoist
- the dropped shared compact-bookkeeping fold
- the dropped claim-open-band compact local-state hoist
- local step-`13` band-widening experiments with no new failed full-profile
  evidence
- contender-rank helper rewrites
- timing-only slices with no new late-step explanation or new step-`14`
  localization
- replay-fixture recapture or benchmark-file churn before a new full rerun
- compare / benchmark / certification work before a new full-profile run
  reaches step `15`

## Keep Or Branch Decision

- Branch back to local early-step parity repair before another full rerun:
  the stopped `v4` evidence already disproved the old step-`14`
  zero-candidate opening but still diverges structurally at step `4`.
- Keep the current short-loop gate, stored step-`4` continuation references,
  the capped intended-profile read, and the stopped `v4` bundle frozen as
  regression checks.
- Return to another full rerun only after targeted steps `4..8` parity checks
  are green.
