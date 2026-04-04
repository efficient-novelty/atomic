# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-04

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
    repaired step-`10..12` chain, so step `9` should not be treated as a
    closed local selection bug while the late step-`13` / step-`15` breadth
    story remains open
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
  - a follow-up cache-key repair now also keeps that guarded curvature shell
    alive into the retained claim candidate pool by keying exact multi-step
    partial-prefix bounds on `(prefix_signature, clause_kappa)`
  - a third follow-up selector repair now also makes live claim step `12`
    prefer the guarded same-primary curvature shell over richer raw
    structural winners by preferring the leaner former/binder shell first,
    then the stronger introduction application spine within that lean tier,
    then the shallower formation-clause var reach, then lower bit cost
  - every current same-primary tied step-`12` survivor still collapses onto
    one observed step-`13..15` continuation and the same late generated counts
    `123`, `12027`, and `1794`
  - the step-`12` accepted-hash fork is therefore now closed locally and does
    not explain the late generated-floor collapse by itself
- A new local late-surface repair now sharpens that remaining blocker:
  - on the repaired step-`12` history, a new scoped claim-only step-`13`
    widening now lifts claim-open from singleton-heavy widths
    `[3,1,1,1,1,1,1]` / raw `3` / generated `9` to
    `[3,1,3,3,1,1,1]` / raw `27` / generated `33` without changing the
    accepted guarded metric shell or the observed
    `step-13 -> step-15` continuation
  - that widened step-`13` surface now keeps all `3` roots alive at step-open
    and shifts the remaining local loss mostly into exact
    legality/connectivity rejection (`24`), partial-prefix bar failure (`12`),
    and incumbent dominance (`2`) before proof-close
  - step `14` no longer shares that thin-path profile locally:
    the widened claim `kappa = 9` catalog now opens `19683` raw telescopes,
    keeps `3` roots alive, and lifts live generated prefixes to `12027`
    before proof-close on the repaired step-`12` chain
  - that widened step-`14` surface now exposes a `4`-way same-primary
    `62 / 9` continuation fork; live claim acceptance now prefers the one
    same-primary survivor that restores the canonical step-`15`
    `DCT 103 / 8` continuation while the broader `78 / 9 / 12027`
    branch remains the alternate local path
  - step `15` on the restored canonical branch still opens a broader
    `6561`-telescope claim catalog, but the repaired live surface now narrows
    to `1794` generated prefixes before proof-close on the canonical
    `DCT 103 / 8` continuation after paying `468` exact partial-prefix bar
    failures plus `80` incumbent-dominance prunes
- A new narrow clause-`0` / clause-`1` historical-reanchor repair now sharpens
  that late blocker further:
  - the captured repaired-canonical step-`15` exact-prune surface now
    consists of `2160` zero-admitted terminal families with no cached compact
    bounds
  - all `6480` generated terminal options across that captured surface are now
    structurally connected but still fail as `NeedsFallback` candidates, with
    `0` structural disconnections, `0` admissibility rejections, and `0`
    historical reanchor hits
  - the surviving temporal terminal cluster on the repaired canonical branch
    has widened to `84` generated / `10` admitted / `10` exact-screened /
    `8` pruned while preserving the canonical `DCT 103 / 8` continuation
  - across those captured `2160` exact-prune families, historical-reanchor
    prefix progress now first breaks only at clause positions `2..5` with
    counts `1458`, `486`, `162`, and `54`; clause positions `0` and `1` are
    now repaired out of the captured isolated-prefix surface
  - the clause-`6` boundary remains downstream of that capture:
    once the first six temporal-shell clauses are fixed, the repaired claim
    lane still keeps exactly `3` clause-`6` prefixes outside the captured
    surface, with only the exact reference continuation exposing a
    `KeepWithoutFallback` terminal path while the two claim-only clause-`6`
    variants remain pure `NeedsFallback`
  - the isolated-prefix and variant-product regressions now localize the
    remaining miss to clause positions `2..5` themselves:
    each remaining position contributes exactly `2` isolated claim-only
    deviations on the otherwise exact seven-clause prefix, with per-variant
    later-suffix counts `81`, `27`, `9`, and `3`, and all resulting
    continuations still remain zero-admitted `NeedsFallback`
  - the forced-reanchor recovery regression now proves each remaining isolated
    local variant would recover all `3` terminal continuations as
    `KeepWithoutFallback`, admitted, and bar-clearing on the otherwise exact
    suffix, but the forced-reanchor winner regression also proves a direct
    local reanchor flip still never restores the canonical reference terminal
    clause: the remaining local best-rank profiles stay `75 / 8` at clause
    `2`, `74 / 8` at clause `3`, and `89 / 8` at clauses `4` and `5`
  - the next repair should therefore stay on clause-local qualifier /
    reanchor evidence at positions `2..5` while preserving the exact
    reference terminal continuation; do not broaden the repaired clause-`0` /
    clause-`1` prefix acceptance to lifted terminal closures
- A new local exact-terminal-only isolated recovery regression now sharpens
  that same late blocker again:
  - on the otherwise exact suffix, granting isolated exact-terminal recovery
    to the remaining clause-`2..5` temporal deviations would still create
    stronger-than-canonical local recovered profiles only at clause positions
    `2` and `3`
  - those unsafe exact-terminal recovered profiles are now fixed at
    `89 / 8` for clause `2` and `88 / 8` for clause `3`
  - the same exact-terminal-only recovery now stays on canonical-primary
    `103 / 8` profiles at clause positions `4` and `5`
  - that exact-terminal-only branch was not landed; the later clause-`4` /
    clause-`5` opening has now been consumed by the narrower historical-
    reanchor repair below, so the next repair should keep clause-`2` /
    clause-`3` isolated recovery fenced while the canonical
    `DCT 103 / 8 / 1794` continuation stays frozen
- A new narrow clause-`4` / clause-`5` historical-reanchor repair now sharpens
  that late blocker further:
  - claim-only temporal-shell variants at clause positions `4` and `5` now
    count as historical reanchor evidence only when the terminal clause stays
    on the exact reference continuation
  - lifted non-reference terminal closures on those repaired later-clause
    prefixes still stay outside historical reanchor, so the broader unsafe
    terminal reland remains out
  - the canonical step-`15` continuation stays at `DCT 103 / 8` while the
    live generated surface lifts from `930` to `1794`
  - the repaired canonical branch now pays `468` partial-prefix bar failures
    plus `80` incumbent-dominance prunes, with a widened surviving temporal
    terminal cluster of `732` generated / `82` admitted / `82` exact-screened /
    `80` pruned
  - the captured exact-prune surface shrinks from `2160` to `1944` families,
    the connected `NeedsFallback` surface shrinks from `6480` to `5832`
    terminal options, and the remaining historical-reanchor mismatch now
    localizes to clause positions `2..3` with counts `1458` and `486`
  - clause positions `0`, `1`, `4`, and `5` are now repaired out of the
    captured isolated-prefix surface, clause `6` still stays downstream of the
    capture boundary, and the remaining local repair is therefore clause-local
    qualifier evidence at positions `2..3` while preserving the exact
    reference terminal continuation
  - the forced-reanchor recovery and winner regressions now apply only to
    clause positions `2` and `3`: each remaining isolated local variant would
    still recover all `3` terminal continuations as `KeepWithoutFallback`,
    admitted, and bar-clearing on the otherwise exact suffix if clause-local
    qualifier evidence were restored, but a direct local reanchor flip still
    never restores the canonical reference terminal clause and instead stays on
    the noncanonical `75 / 8` and `74 / 8` winner profiles
  - the exact-terminal-only isolated recovery regression now also sharpens the
    remaining blocker honestly: clause-`4` / clause-`5` are no longer present
    on the captured isolated surface, while clause-`2` / clause-`3` still
    reopen the stronger local `89 / 8` and `88 / 8` rivals if isolated
    recovery is granted
  - the next repair should therefore stay on clause-local qualifier /
    reanchor evidence at positions `2..3` while preserving the exact
    reference terminal continuation; do not broaden the repaired clause-`0` /
    clause-`1` / clause-`4` / clause-`5` prefix acceptance to lifted terminal
    closures
- A new local clause-`2` / clause-`3` pair-surface regression now sharpens
  that same late blocker further:
  - all `8` remaining claim-only clause-`2` / clause-`3` pairings stay
    present across the captured repaired-canonical surface with `243`
    prefixes each through the later-suffix fan-out
  - exact-terminal-only local recovery on any such pair still admits and
    bar-clears all `243` prefixes, but every recovered profile still outranks
    the canonical `DCT 103 / 8` continuation
  - the clause-`2` exact / clause-`3` claim-only pairs stay pinned to unsafe
    exact-terminal profiles `74 / 8` and `88 / 8`
  - the clause-`2` claim-only / clause-`3` exact pairs stay pinned to unsafe
    exact-terminal profiles `75 / 8` and `89 / 8`
  - the mixed claim-only clause-`2` / clause-`3` pair surfaces stay pinned to
    unsafe exact-terminal profiles `60 / 8` and `74 / 8`
  - forced clause-local reanchor across those mixed pair surfaces now admits
    all `729` terminal continuations but keeps only `405` bar-clearing
    winners, with the exact reference terminal winning on `162 / 243`
    prefixes there while the rest still split to the two non-reference
    closures
  - a new mixed-pair reference-terminal winner regression now sharpens that
    same read further:
    even when forced clause-local reanchor on those mixed pair surfaces does
    restore the exact reference terminal, those wins still occur only on the
    unsafe `60 / 8` profile and still all outrank the canonical
    `DCT 103 / 8` continuation
  - the next repair should therefore stay off any direct clause-`2` /
    clause-`3` matcher or paired reanchor reland and instead target narrower
    qualifier evidence on the canonical branch without reopening those
    stronger local rivals
- A new nearby clause-`2` / clause-`3` temporal-replacement regression now
  sharpens that same late blocker further:
  - swapping the current claim-only clause-`2` variants for nearby demo-style
    temporal neighbors still leaves every exact-terminal-only isolated recovery
    on the same unsafe `89 / 8` profile
  - swapping the current claim-only clause-`3` variants for nearby demo-style
    temporal neighbors still leaves every exact-terminal-only isolated recovery
    on the same unsafe `88 / 8` profile
  - those nearby replacements also stay structurally connected, locally
    admissible, and outside historical reanchor on the otherwise exact suffix,
    so a simple clause-catalog swap is not enough to repair the missing
    clause-local qualifier evidence
- A new nearby clause-`2` / clause-`3` pair-replacement regression now
  sharpens that same late blocker further:
  - pairing any nearby claim-or-demo temporal replacement at clause positions
    `2` and `3` on the otherwise exact suffix still leaves all `16`
    non-reference pairs structurally connected, locally admissible, and
    outside historical reanchor
  - every such exact-terminal-only paired recovery now collapses onto the same
    unsafe `74 / 8` profile
  - broadening that local replacement neighborhood therefore still does not
    supply safe qualifier evidence on the canonical branch
- A new mixed clause-`2` / clause-`3` reference-terminal context regression
  now sharpens that same late blocker further:
  - every mixed claim-only clause-`2` / clause-`3` surface that restores the
    exact reference terminal under forced reanchor still does so only when
    clause `6` also deviates
  - those unsafe `60 / 8` reference-terminal wins already span every
    repaired-side subset of clause positions `0`, `1`, `4`, and `5` on top
    of that clause-`6` deviation, including the minimal clause-`6`-only
    context
  - the next repair should therefore stay off any clause-`6`-mediated reland
    too and keep targeting narrower clause-`2` / clause-`3` qualifier
    evidence on the canonical branch itself
- New stored-audit regressions now freeze that `v5` evidence surface:
  - compare locks the step-`9`, step-`11`, and step-`12` parity forks
  - certification locks the step-`1` breadth miss and the step-`10..15`
    generated-floor snapshot
  - benchmark locks the single-run `408 ms` / parity-`0` / breadth-hit-`0`
    aggregate
- A new local step-`13` catalog regression now freezes the repaired
  breadth read more precisely:
  claim-open now sits at `kappa = 7..7` on `LateFamilySurface::ClaimGeneric`
  with scoped widths `[3,1,3,3,1,1,1]`, raw catalog `27`, and live generated
  prefixes `33` before proof-close while the guarded step-`13` metric shell
  stays accepted, so the next late read is no longer pure catalog-open
  starvation but the residual exact-screen losses on that widened surface.
- A new follow-up step-`13` exact-prune/connectivity regression now freezes
  that residual widened-surface loss more precisely:
  - the widened step-`13` exact-screen split is now fixed at
    `24` connectivity prunes, `0` terminal-clause-filter prunes,
    `12` partial-prefix bar prunes, and `2` incumbent prunes
  - the captured widened step-`13` exact-prune surface now consists of
    `24` zero-admitted remaining-one prefixes with no cached compact bounds
  - all `24` generated terminal options on that captured surface are now
    frozen as structurally disconnected before fallback, with `0`
    `NeedsFallback` candidates and `0` exact-legality rejections
  - the next step-`13` repair should therefore target structural
    connectivity on that widened operator-band surface rather than terminal
    clause filtering, qualifier fallback, or another blind catalog reland
- A new follow-up step-`13` structural-connectivity repair now updates that
  local read again:
  - claim operator-bundle seed and action variants now satisfy structural
    connectivity on the scoped widened band-`7` surface
  - the guarded step-`13` metric shell stays accepted on the repaired live
    chain
  - live generated prefixes on that repaired step-`13` surface now lift from
    `33` to `123`
  - exact connectivity rejections, partial-prefix bar failures, and captured
    zero-admitted exact prunes on that repaired step-`13` surface are now all
    `0`
- A new follow-up step-`13` repaired-chain incumbent-relief repair now closes
  the last widened-surface step-`13` blocker on that same repaired chain:
  - claim proof-close now preserves same-primary incumbent ties on step `13`
    when the repaired live history reaches guarded step-`12` `34 / 6`
  - the guarded step-`13` metric shell and the observed
    `step-13 -> step-15` continuation stay unchanged on that repaired chain
  - the widened repaired step-`13` surface now also clears live terminal-rank
    pruning entirely (`0`) while keeping exact connectivity rejections,
    partial-prefix bar failures, and captured zero-admitted exact prunes at `0`
  - the divergent step-`13` viability-split guardrail stays intact on the
    unrepaired branch, so this relief remains scoped to the repaired
    `34 / 6` history rather than reopening the broader divergent surface
- A follow-up exploratory global step-`13` widening was also run locally but
  not landed:
  - widening the claim-generic band-`7` widths from `[3,1,1,1,1,1,1]` to
    `[3,3,3,3,3,3,3]` lifted the repaired step-`13` read from raw `3` /
    generated `9` to raw `2187` / generated `615`
  - root seeding on that branch widened to `roots_seen = 3`,
    `roots_rejected_by_exact_screen = 0`, and `roots_enqueued = 3`
  - most remaining loss there shifted into exact legality/connectivity
    rejection (`1954`), partial-prefix bar failure (`168`), and incumbent
    dominance (`236`)
  - the late local path on that branch became
    `(13,45,7,615) -> (14,61,9,12027) -> (15,103,8,780)`
  - but that same reland disturbed claim prefix-memo, realistic-shadow,
    demo-lane, and divergent step-`13` / step-`14` guardrails, so it was
    reverted and should not be relanded directly
- A follow-up exploratory step-`15` temporal-reanchor broadening was also run
  locally but not landed:
  - a broad temporal-shell matcher expansion collapsed the captured exact-prune
    surface, but it also displaced the canonical step-`15` continuation from
    `103 / 8 / 780` to `60 / 8 / 9840`
  - a narrower late-shell-only reanchor reland still displaced the canonical
    continuation to `89 / 8 / 780`
  - both variants were reverted, so the next step-`15` repair should not be a
    direct matcher broadening reland
- A second follow-up exploratory early-bridge reanchor pass was also run
  locally but not landed:
  - broadening only the clause-`3` historical-reanchor anchor to accept the
    flat-bridge argument displaced the canonical step-`15` continuation from
    `103 / 8 / 780` to `88 / 8 / 795`
  - widening the earlier clause-`2` plus clause-`3` bridge pair displaced the
    canonical continuation further to `74 / 8 / 828`
  - both variants were reverted, and the then-current step-`15`
    `103 / 8 / 780` baseline plus the exact-prune / connectivity /
    reanchor-prefix regressions were revalidated afterwards
  - the next step-`15` repair should therefore not directly broaden the early
    clause-`2` / clause-`3` reanchor bridge matcher either

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
   candidates still share the same observed repaired step-`10..12` chain.
6. Keep the new step-`12` selector read fixed:
   live claim step `12` now prefers the guarded same-primary curvature shell
   over richer raw structural winners inside the retained `34 / 6` tier.
7. Keep the new step-`12` continuation-collapse read fixed:
   all current same-primary tied step-`12` survivors still collapse onto one
   observed step-`13..15` continuation and the same late generated counts
   `123`, `12027`, and `780`.
8. Keep the new late-surface diagnostic read fixed:
   - repaired step `13` now opens a scoped widened claim surface with raw
     widths `[3,1,3,3,1,1,1]`, raw catalog `27`, `3` surviving roots, and
     live generated prefixes `123` while preserving the guarded accepted
     metric shell
   - the repaired widened step-`13` surface now clears exact connectivity and
     partial-prefix bar screening entirely on that branch
   - a follow-up repaired-chain incumbent-relief pass now also clears live
     terminal-rank pruning there (`0`) on the repaired guarded
     step-`12` `34 / 6` chain while preserving the same observed
     `step-13 -> step-15` continuation and leaving the divergent step-`13`
     guardrail intact
   - repaired step `14` now opens `19683` raw claim telescopes on the promoted
     `kappa = 9` band, keeps `3` roots alive, and lifts live generated
     prefixes to `12027` before proof-close
   - that widened step-`14` surface now exposes a `4`-way same-primary
     `62 / 9` continuation fork, and live claim acceptance now prefers the
     one survivor that restores the canonical step-`15` `DCT 103 / 8`
     continuation
   - the restored canonical step `15` branch still opens `6561` raw claim
     telescopes but still loses `512` prefixes to partial-prefix bar failure
     before proof-close
9. Split the common late floor collapse into its actual subproblems:
   - step `13` is no longer blocked by a pure singleton-heavy catalog-open:
     the landed scoped widening plus follow-up structural-connectivity repair
     now reach raw widths `[3,1,3,3,1,1,1]`, raw catalog `27`, and live
     generated prefixes `123`, and the follow-up repaired-chain
     incumbent-relief pass now also keeps exact connectivity rejections,
     partial-prefix bar failures, captured zero-admitted exact prunes, and
     live terminal-rank prunes all at `0` on the repaired guarded
     `34 / 6` chain while the rejected global branch still bounds the much
     larger reachable ceiling (`2187` / `615`); step `13` should therefore
     now stay on the guard rail as a repaired local surface rather than reopen
     as the next late blocker
   - step `14` is now locally widened enough that it should stay on the guard
     rail as a regression rather than reopening as the first breadth blocker
   - step `15` remains the first late floor, but the updated captured
     exact-prune regression now shows that surface localizes to `1944`
     zero-admitted families with no cached compact bounds while the live
     canonical branch stays fixed at `DCT 103 / 8 / 1794`; the connectivity
     regressions now prove all `5832` generated terminal options there are
     structurally connected but still unqualified `NeedsFallback` candidates
     with `0` historical reanchor hits and `0` admissibility rejections, and
     the follow-up reanchor-prefix regression now proves those captured
     families already fall off the temporal-shell prefix only at clause
     positions `2..3` with no full seven-clause matches while clause positions
     `0`, `1`, `4`, and `5` stay repaired out of the captured isolated-prefix
     surface; the clause-`6` boundary regression still proves only `3`
     clause-`6` variants remain outside the captured surface once the first
     six clauses are fixed, while the updated isolated-prefix / variant-
     product regressions now localize the remaining miss to `4` isolated
     claim-only deviations, two each at clause positions `2` and `3`, each
     blocked across its later-suffix fan-out; the forced-reanchor recovery
     regression still proves each remaining isolated local variant would
     recover all `3` terminal continuations as `KeepWithoutFallback`,
     admitted, and bar-clearing on the otherwise exact suffix, so inspect
     clause-local qualifier / reanchor evidence at positions `2..3` on that
     connected surface rather than generic partial-prefix bar arithmetic, raw
     dependency-edge generation, claim admissibility, downstream admissibility
     / bar bookkeeping, a multi-deviation-only interaction, a later-suffix
     interaction, or the clause-`6` / terminal slot alone; the new
     exact-terminal-only isolated recovery regression now further fences the
     earlier bridge positions by proving clause `2` and clause `3` still
     reopen stronger local `89 / 8` and `88 / 8` recovered rivals even
     without lifted terminal closures
10. Keep the step-`1` breadth miss on the checklist, but do not confuse that
    longstanding signoff floor with the new claim-specific mid/late parity
    fork.

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
7. Keep the new step-`12` retained-pool regression green so the guarded
   curvature shell stays present in the retained candidate pool after the
   cross-band cache-key repair.
8. Keep the new step-`12` cache-key regression green so a modal `5`-clause
   `CannotClearBar` result cannot poison the guarded `6`-clause continuation
   on the same preterminal prefix.
9. Keep the new step-`12` selector regression green so live claim acceptance
   continues to choose the guarded same-primary curvature shell over richer
   raw structural winners inside the repaired `34 / 6` tier.
 10. Keep the new step-`12` continuation-collapse regression green so the tied
     same-primary step-`12` set stays pinned to one observed step-`13..15`
     continuation with generated counts `123`, `12027`, and `1794` while the
     remaining late floor collapse is repaired.
11. Keep the new step-`13` scoped-widening plus structural-connectivity repair
    regressions green so the repaired step-`12` chain keeps reporting
    claim-open `kappa = 7..7` with raw widths `[3,1,3,3,1,1,1]`, raw
    catalog `27`, live generated prefixes `123`, and the guarded accepted
    metric shell before proof-close.
    Keep the paired repaired step-`13` exact-screen-clear plus incumbent-
    relief regression green too: the same repaired chain should still report
    `0` connectivity prunes, `0` terminal-clause-filter prunes,
    `0` partial-prefix bar prunes, `0` captured zero-admitted exact prunes,
    and `0` live terminal-rank prunes while the divergent step-`13`
    viability-split guardrail stays intact.
12. Keep the step-`4` claim prefix-memo, realistic-shadow, demo-lane, and
    divergent step-`13` / step-`14` guardrails green while exploring any
    step-`13` widening path:
    the rejected global branch already showed raw `2187` / generated `615`
    is not enough to justify waking those unrelated surfaces.
13. Keep the new widened step-`14` regression green so the repaired step-`12`
    chain keeps reporting step `14` raw catalog `19683` with `3` surviving
    roots and `12027` live generated prefixes before proof-close.
14. Keep the new step-`14` same-primary continuation selector regression green
    so live claim acceptance continues to choose the one same-primary
    `62 / 9` survivor that restores the canonical step-`15`
    `DCT 103 / 8` continuation while the broader `78 / 9 / 12027`
    branch remains the alternate local path.
15. Keep the new step-`15` exact-prune family regression green so the
    repaired canonical temporal-shell path continues to report `1944`
    captured zero-admitted exact prunes with no cached compact bounds while
    the live canonical continuation stays fixed at `DCT 103 / 8 / 1794`.
16. Keep the new step-`15` zero-admitted connectivity regressions green so the
    repaired canonical temporal-shell path continues to report `5832`
    structurally connected but still unqualified `NeedsFallback` terminal
    options, `0` structural disconnections, `0` historical reanchor hits, and
    `0` admissibility rejections across those captured `1944` exact prunes
    while the next repair targets qualifier / reanchor evidence there.
17. Keep the new step-`15` reanchor-prefix regression green so the repaired
    canonical temporal-shell path continues to report no full seven-clause
    reanchor-prefix matches across those captured `1944` exact prunes and the
    current mismatch split stays localized to clause positions `2..3` with
    counts `1458` and `486`, while clause positions `0`, `1`, `4`, and `5`
    stay repaired out of the captured surface.
18. Keep the new step-`15` clause-`6` boundary regression green so once the
    first six temporal-shell clauses are fixed, the repaired claim lane still
    keeps exactly `3` clause-`6` prefixes outside the captured exact-prune
    surface, with only the exact reference continuation exposing a
    `KeepWithoutFallback` terminal path while the two claim-only clause-`6`
    variants remain pure `NeedsFallback`.
19. Keep the new step-`15` isolated-prefix plus variant-product regressions
    green so each remaining clause position `2..3` still contributes exactly
    `2` isolated claim-only deviations on the otherwise exact seven-clause
    prefix, while clause positions `0`, `1`, `4`, and `5` stay absent from
    that captured surface and all resulting continuations still leave `0`
    admitted candidates.
    Keep the new forced-reanchor recovery regression green too: each remaining
    isolated local variant should still recover all `3` terminal
    continuations as `KeepWithoutFallback`, admitted, and bar-clearing on the
    otherwise exact suffix once clause-local qualifier evidence is supplied.
    Keep the new forced-reanchor winner regression green too: that same direct
    local reanchor flip should still never restore the canonical reference
    terminal clause and should instead stay pinned to the fixed remaining
    noncanonical winner profiles `75 / 8` and `74 / 8`.
    Keep the new exact-terminal-only isolated recovery regression green too:
    even without lifted terminal closures, clause positions `2` and `3`
    should still reopen stronger local `89 / 8` and `88 / 8` recovered
    rivals, so direct clause-`2` / clause-`3` isolated recovery remains
    fenced while the repaired clause-`4` / clause-`5` qualifier work stays on
    the guardrail.
    Keep the new clause-`2` / clause-`3` pair-surface regression green too:
    all `8` remaining pairings should stay pinned at `243` captured prefixes
    each, exact-terminal-only recovery on every pairing should still admit
    and bar-clear all `243` prefixes while staying stronger than the
    canonical `DCT 103 / 8` profile, and forced clause-local reanchor on the
    mixed pair surfaces should still admit all `729` terminal continuations,
    keep only `405` bar clearers, and split winners with the exact reference
    terminal winning on `162 / 243` prefixes there.
    Keep the new mixed-pair reference-terminal winner regression green too:
    those mixed-pair reference-terminal wins should still stay pinned to the
    unsafe `60 / 8` profile and still all outrank the canonical
    `DCT 103 / 8` continuation, so both isolated and paired clause-`2` /
    clause-`3` direct recovery remain fenced.
    Keep the new mixed clause-`2` / clause-`3` reference-terminal context
    regression green too: every mixed claim-only clause-`2` / clause-`3`
    surface that restores the exact reference terminal under forced reanchor
    should still require a claim-only clause-`6` deviation and should
    already span every repaired-side subset of positions `0`, `1`, `4`, and
    `5` on top of that clause-`6` deviation, so no safe reference-terminal
    pocket is hiding behind repaired-side exactness or a clause-`6`-mediated
    reland.
    Keep the new nearby clause-`2` / clause-`3` temporal-replacement
    regression green too: swapping the current claim-only variants for nearby
    demo-style temporal neighbors should still stay outside historical
    reanchor and still collapse onto the same unsafe isolated `89 / 8` and
    `88 / 8` profiles, so a simple clause-catalog swap remains fenced.
    Keep the new nearby clause-`2` / clause-`3` pair-replacement regression
    green too: broadening both positions together across that same nearby
    claim/demo neighborhood on the otherwise exact suffix should still stay
    outside historical reanchor and still collapse all `16`
    exact-terminal-only paired recoveries onto the same unsafe `74 / 8`
    profile, so broader local replacement-neighborhood relands remain fenced
    too.
20. Keep the stored compare regression green for the step-`12`
    guarded `34 / 6` versus stored-`v5` claim `33 / 5` drop.
21. Keep the stored certification / benchmark assertions green for the
    step-`1` breadth miss and the late generated-floor snapshot at
    steps `10..15` so the next fix cannot silently reshuffle the stored
    failure surface.
22. Keep the existing pre-flight gate green while the local repair lands.
23. Do not reland either exploratory temporal-reanchor matcher variant
    directly:
    the broad reland displaced the canonical step-`15` continuation to
    `60 / 8 / 9840`, and the narrower late-shell-only reland still displaced
    it to `89 / 8 / 780`.
24. Do not reland the exploratory early clause-`2` / clause-`3`
    reanchor-bridge matcher variants directly:
    the clause-`3`-only branch displaced the canonical step-`15`
    continuation to `88 / 8 / 795`, and the clause-`2` plus clause-`3`
    branch displaced it to `74 / 8 / 828`, and the new clause-`2` /
    clause-`3` pair-surface regression now shows the remaining paired claim
    surfaces are still unsafe to reland directly too.

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
- a naive global claim band-`7` widening reland:
  the exploratory branch widened step `13` locally but broke unrelated claim
  prefix-memo, realistic-shadow, demo-lane, and divergent late-step
  guardrails
- a step-`13` terminal-clause-filter or fallback/reanchor theory first:
  the new widened-surface regression now shows the captured step-`13`
  loss is `24` structurally disconnected zero-admitted prefixes with `0`
  terminal-clause-filter and `0` `NeedsFallback` traffic
- replay-fixture recapture or benchmark-file churn before the parity/floor fix
- stronger claim wording or runtime-threshold freeze before a passing
  certificate exists
- another raw step-`9` enumeration or terminal-clause-filter theory first:
  the guarded step-`9` telescope is already present on both of those
  diagnostic surfaces
- a direct temporal-reanchor matcher reland first:
  both the broad `60 / 8 / 9840` branch and the narrower
  `89 / 8 / 780` branch disturbed the canonical step-`15` continuation
- a terminal-clause-only step-`15` qualifier tweak first:
  the new reanchor-prefix regression now proves the captured exact-prune
  surface already falls off the temporal-shell prefix by clause `5` or earlier
  and never reaches a full seven-clause reanchor prefix
- a simple historical-reanchor bool flip on the isolated early claim-only
  temporal variants first:
  the new forced-reanchor winner regression now shows that even on the
  otherwise exact suffix such a direct reland still never restores the
  canonical reference terminal clause and instead splits into the two
  non-reference terminal closures
- a direct early clause-`2` / clause-`3` reanchor-bridge matcher reland
  first:
  the clause-`3`-only branch already displaced the canonical step-`15`
  continuation to `88 / 8 / 795`, and the clause-`2` plus clause-`3`
  branch displaced it further to `74 / 8 / 828`
- a direct paired clause-`2` / clause-`3` matcher reland first:
  the new pair-surface regression now shows every remaining pairing still
  recovers only through stronger-than-canonical exact-terminal profiles even
  when mixed pair surfaces sometimes restore the exact reference terminal
  under forced reanchor
- a direct isolated exact-terminal clause-`2` / clause-`3` recovery reland
  first:
  the new local regression now shows that even without lifted terminal
  closures those earlier isolated recoveries still reopen stronger local
  `89 / 8` and `88 / 8` rivals on the otherwise exact suffix
- a simple nearby clause-`2` / clause-`3` temporal clause-catalog swap first:
  the new local regression now shows that demo-style neighboring replacements
  still stay outside historical reanchor and still collapse onto those same
  unsafe isolated `89 / 8` and `88 / 8` profiles on the otherwise exact
  suffix

## Keep Or Branch Decision

- Keep the current lane on parity-plus-breadth repair using completed `v5` as
  the canonical stored claim bundle.
- Keep the current short-loop gate, stored step-`4` continuation references,
  the capped intended-profile read, stopped `v4`, and completed `v5` frozen as
  regression checks.
- Return to another runtime-only slice only if the stored targeted parity/floor
  regressions prove the stored failure is just an accounting bug rather than a
  real search divergence.
