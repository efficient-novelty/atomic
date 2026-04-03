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
- Keep the finished pre-repair late-step failure:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v3`
- Keep the capped intended-profile validation read:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-late-accept-capped-v1`
- Keep the stopped fresh full-profile rerun evidence path:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v4`
- Keep the fresh completed full-profile rerun:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v5`
- Keep the stored `v5` audit outputs frozen beside that run:
  - `claim-compare.txt` / `claim-compare.json`
  - `claim_certificate.txt` / `claim_certificate.json`
  - `claim_benchmark.txt` / `claim_benchmark.json`
- Keep the preserved `v3` release binary hash:
  `278c311ddf5e416b09d24923dc392388aaf5817c65f0c60f856ebde7466140a5`
- Keep the validated capped-read / stopped-`v4` release binary hash:
  `d3601f87cea1ff639d7c2ed19e604b1a815a65374790f6240910f7bebf3a711f`
- Keep the fresh completed `v5` release binary hash:
  `c42758f96c8171900651503d7f2a0ffe9915966c41edea98d8f1e296fc772a4e`

## Current Read

- The short pre-flight gate was rerun on clean-tree repo head
  `c1fbb51d4fc9a620cd2ce95c9c3eadfe1a54fc65` and is green:
  - targeted claim regressions
  - claim live-checkpoint persistence
  - release replay harness on all `5` stored plateau surfaces
- A fresh clean-start full-profile rerun,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v5`,
  now exists on that same clean-tree head and release binary hash
  `c42758f96c8171900651503d7f2a0ffe9915966c41edea98d8f1e296fc772a4e`.
- Its authoritative `run.json` state is:
  - `status = "completed"`
  - `completed_step = 15`
  - `active_step = 16`
  - `active_band = 8`
  - `frontier_epoch = 12`
  - `dirty_tree = false`
- The old `v3` step-`14` zero-candidate opening is no longer the first
  blocker:
  `v5` reaches step `14`, seeds roots, accepts a survivor there, and then
  completes step `15`.
- The new compare report against `runs/codex-guarded-claim-cert-v1` is
  explicit:
  - trajectory diverges at step `9`, step `11`, step `12`, step `13`,
    step `14`, and step `15`
  - accepted hashes diverge at step `9`, step `11`, and step `12`
  - search-space counts diverge at step `4`, step `9`, step `10`,
    step `11`, step `12`, and step `14`
- The earliest accepted-hash fork is now localized:
  - step `9` keeps guarded `nu = 17`, `kappa = 4`, but accepts a different
    hash
  - step `10` realigns on the guarded accepted hash
  - step `11` keeps guarded `nu = 26`, `kappa = 5`, but accepts a different
    hash
  - step `12` drops to `nu = 33`, `kappa = 5` versus guarded `34 / 6`
  - step `13` and step `14` recover the guarded accepted hashes but stay one
    `nu` low
  - step `15` returns the guarded accepted `DCT` hash / `nu` / `kappa`
- The new certification result is:
  - status `= "attention"`
  - failing checks:
    - `accepted_hash_parity`
    - `early_breadth`
    - `late_generated_floors`
  - passing checks:
    - claim search-policy honesty
    - fallback honesty
    - narrative completeness
    - runtime threshold
    - exact-screen reason completeness
    - prune class completeness
    - manifest completeness
- The stored generated-floor misses are now concrete:
  - step `1`: `546` versus target `2144`
  - step `10`: `1428` versus target `500` (`hit`)
  - step `11`: `546` versus target `800` (`miss`)
  - step `12`: `930` versus target `1200` (`miss`)
  - step `13`: `9` versus target `2200` (`miss`)
  - step `14`: `157` versus target `3500` (`miss`)
  - step `15`: `780` versus target `5000` (`miss`)
- The benchmark bundle now proves runtime is not the open blocker on the
  stored completed run:
  - claim run count `= 1`
  - completed step-`15` count `= 1`
  - runtime `= 408 ms`
  - parity success count `= 0`
  - full early breadth hit count `= 0`
  - full late floor hit count `= 0`
- A new local step-`9` diagnostic pass narrows the earliest fork further:
  - the guarded step-`9` telescope is still present in claim late
    enumeration (`81` `kappa = 4` claim telescopes)
  - it still passes claim open-band admissibility and connectivity on the
    guarded step-`8` prefix
  - the guarded step-`9` three-clause prefix still exposes the guarded
    closing clause through the claim remaining-one terminal-clause filter
  - a local no-drop frontier experiment can carry that guarded step-`9`
    telescope into the retained claim candidate set, so the earliest fork is
    no longer best explained as raw generation loss or terminal-clause
    filtering loss
  - but the tied step-`9` candidates still collapse onto the same observed
    step-`10` / repaired-step-`11` continuation while step `12` remains
    unresolved, so step `9` should not be treated as a closed local selection
    bug yet
- A new local step-`11` diagnostic and narrow incumbent-pruning plus selector
  repairs now extend that localization:
  - the guarded step-`11` closing clause is still present in the claim
    remaining-one terminal-clause catalog on the divergent step-`10` history
  - the guarded step-`11` completion still survives the full remaining-one
    summary and the compact survivor sketch even with an incumbent present
  - claim incumbent pruning now preserves same-primary accepted-rank ties on
    steps `9..12`, so that guarded step-`11` completion now survives into the
    retained claim candidate pool
  - live claim step-`11` acceptance now prefers the guarded same-primary
    survivor over the richer raw structural winner by preferring the leaner
    former/binder shell first, then stronger dependent density within that
    lean tier, then lower bit cost
  - every current same-primary tied step-`11` survivor now collapses onto one
    observed step-`12` accepted hash / `nu` / `kappa` at guarded `34 / 6`, not
    the old `33 / 5` drop
  - step `11` is therefore no longer best explained as raw generation loss,
    compact-summary loss, a simple one-step viability miss, or an unresolved
    local selector; the old step-`12` `33 / 5` drop is no longer the next
    blocker on that repaired step-`11` history
- A new local step-`12` minimality and exact-screen repair now extends that
  localization:
  - the guarded step-`12` curvature shell is still enumerated and
    exact-admitted on the repaired live claim history
  - it now also survives preterminal clause exposure, full remaining-one
    summary, compact survivor sketch, semantic minimality, and exact
    remaining-one screening on that history
  - live claim step `12` now recovers guarded `nu = 34`, `kappa = 6`
    instead of the old `33 / 5` drop
  - but live claim step `12` still accepts a different same-primary
    `34 / 6` hash, and the guarded curvature shell still does not survive
    into the retained claim candidate pool
  - the remaining local blocker is therefore now the step-`12`
    same-primary retained-pool / accepted-hash fork inside `34 / 6`
- New stored-audit regressions now freeze that `v5` evidence surface:
  - compare locks the step-`9`, step-`11`, and step-`12` parity forks
  - certification locks the step-`1` breadth miss and the step-`10..15`
    generated-floor snapshot
  - benchmark locks the single-run `408 ms` / parity-`0` / breadth-hit-`0`
    aggregate

## Do This Next

### 1. Freeze The New Completed Evidence Set

1. Freeze `v1`, `v2`, `v3`, capped `v1`, stopped `v4`, and completed `v5` as
   the current stored claim evidence set.
2. Freeze the new compare / certification / benchmark outputs under `v5`.
3. Keep the replay-harness corpus and benchmark files frozen; do not recapture
   fixtures first.

### 2. Treat Completion And Audit Infrastructure As Earned

1. Treat the pre-flight gate as earned on repo head
   `c1fbb51d4fc9a620cd2ce95c9c3eadfe1a54fc65`.
2. Treat one fresh clean-start full-profile completion through step `15` as
   earned on the current claim lane.
3. Treat claim-policy honesty, fallback honesty, narrative/event completeness,
   exact-screen reason completeness, prune-class completeness, and manifest
   completeness as earned on stored full-profile evidence.
4. Treat the old step-`14` zero-candidate opening and the old step-`4`
   runtime wall as no longer being the first blockers.

### 3. Diagnose The Earliest Remaining Parity And Breadth Forks

1. Keep the new step-`9` diagnostic read fixed:
   the guarded telescope is still present in claim enumeration and still
   survives direct claim admissibility / remaining-one clause filtering.
2. Keep the new step-`11` retained-pool diagnostic read fixed:
   the guarded step-`11` completion now survives terminal-clause generation,
   full summary, compact survivor sketch, and retained-pool preservation on
   the live claim history.
3. Keep the new step-`11` selector read fixed:
   live claim acceptance now prefers the guarded same-primary survivor over
   the richer raw structural winner on the live divergent history.
4. Keep the new step-`11` continuation-collapse read fixed:
   all current same-primary tied step-`11` survivors still keep step `12`
   alive and now collapse onto one observed claim step-`12`
   accepted hash / `nu` / `kappa` at `34 / 6`.
5. Keep step `9` final selection deferred while the current tied step-`9`
   candidates still share the same observed step-`10` / repaired-step-`11`
   continuation.
6. Move the active repair target to the step-`12` same-primary
   retained-pool / accepted-hash fork inside guarded `34 / 6`:
   inspect why the guarded curvature shell now survives exact screening but
   still does not survive into the retained claim candidate pool, and why a
   different same-primary `34 / 6` hash is accepted instead.
7. Use the repaired step-`9` / step-`11` / step-`12` chain to explain the
   late generated-floor collapse at
   steps `11..15`, especially:
   - step `13`: `9`
   - step `14`: `157`
   - step `15`: `780`
8. Keep the step-`1` breadth miss on the checklist, but do not confuse that
   longstanding signoff floor with the new claim-specific mid/late parity fork.

### 4. Use The New Regressions To Drive The Local Repair

1. Keep the stored compare regression green for the step-`9`
   same-`nu` / same-`kappa` accepted-hash fork.
2. Keep the new step-`11` retained-pool regression green so same-primary
   claim ties are not silently pruned before final selection on steps `9..12`.
3. Keep the new step-`11` selector regression green so live claim acceptance
   continues to choose the guarded same-primary survivor over richer raw
   structural winners on the live divergent history.
4. Keep the new step-`11` continuation-collapse regression green so the
   current tied same-primary set stays pinned to one observed step-`12`
   `34 / 6` continuation while the step-`12` same-primary hash fork is
   repaired.
5. Keep the stored compare regression green for the step-`11`
   same-`nu` / same-`kappa` accepted-hash fork.
6. Keep the new step-`12` exact-screen / minimality regression green so the
   guarded curvature shell keeps surviving preterminal clause exposure, full
   summary, compact survivor sketch, semantic minimality, and exact
   remaining-one screening on the repaired live history.
7. Keep the stored compare regression green for the step-`12`
   guarded `34 / 6` versus stored-`v5` claim `33 / 5` drop.
8. Keep the stored certification / benchmark assertions green for the
   step-`1` breadth miss and the late generated-floor snapshot at
   steps `10..15` so the next fix cannot silently reshuffle the stored
   failure surface.
9. Keep the existing pre-flight gate green while the local repair lands.

### 5. Only Rerun After The Local Repair Exists

1. Launch `long-rerun-v6` only after the new parity/floor regressions are
   green locally.
2. Re-run compare, certification, and benchmark on that new full-profile
   bundle.
3. Only treat the certification gate as newly in reach if `v6` closes
   `accepted_hash_parity` and the late generated floors while preserving the
   new step-`15` completion.

## Do Not Reopen First

- a `resume`-based restart of the stopped `v4` run
- another runtime-only step-`4` micro-optimization slice first
- another late-step zero-candidate diagnosis slice first
- another clean-start full-profile rerun before the local repair is green
  against the new step-`9` / `11` / `12` regressions
- replay-fixture recapture or benchmark-file churn before the parity/floor fix
- stronger claim wording or runtime-threshold freeze before a passing
  certificate exists
- another raw step-`9` enumeration or terminal-clause-filter theory first:
  the guarded step-`9` telescope is already present on both of those
  diagnostic surfaces

## Keep Or Branch Decision

- Keep the current lane on parity-plus-breadth repair using completed `v5` as
  the canonical stored claim bundle.
- Keep the current short-loop gate, stored step-`4` continuation references,
  the capped intended-profile read, stopped `v4`, and completed `v5` frozen as
  regression checks.
- Return to another runtime-only slice only if the stored targeted parity/floor
  regressions prove the stored failure is just an accounting bug rather than a
  real search divergence.
