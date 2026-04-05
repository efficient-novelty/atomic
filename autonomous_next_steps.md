# Autonomous Claim Lane: Next Operational Slice

Last updated: 2026-04-05

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
  `629b59861d521555cfd214a602bedec4f44c67d0` with release binary hash
  `bdcbf9b0c6386a5fd2e0e7388c2637a15486c962f47041530075dff3e50a5385`
  and is green:
  - targeted claim regressions
  - claim live-checkpoint persistence
  - release replay harness on all `5` stored plateau surfaces
- The current repaired-chain local guardrails were rerun on clean-tree repo
  head `67c26eca02cb5546745bdd5ca5b31468e6807f42`:
  - guarded step-`11` / step-`12` / step-`15` claim regressions green
  - repaired step-`12` same-primary selector regression green
  - repaired step-`12 -> 15` continuation guardrails green
- A fresh clean-start full-profile rerun,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v9`,
  now exists on clean-tree repo head
  `67c26eca02cb5546745bdd5ca5b31468e6807f42` with release binary hash
  `2023ea693e72403b98448ab1bece5048b739a2cb115aafcd2b1580cb941a59bf`.
- Its authoritative `run.json` state is:
  - `status = "completed"`
  - `completed_step = 15`
  - `active_step = 16`
  - `active_band = 8`
  - `frontier_epoch = 12`
  - `dirty_tree = false`
- The older completed `v5` bundle remains frozen as the pre-parity reference
  surface.
- The earlier clean-tree completed `v6` bundle remains frozen as the
  pre-step-`11`-breadth stored baseline.
- The dirty-tree `v7` and `v8` reruns remain transient selector diagnostics:
  - `v7` first re-earned stored step `11` but reopened accepted-hash parity at
    step `12`
  - `v8` restored parity and the step-`11` floor, but it was recorded with
    `dirty_tree = true`
- The new compare report against `runs/codex-guarded-claim-cert-v1` is ready:
  - trajectory matches guarded through step `15`
  - accepted hashes match guarded through step `15`
  - replay ablation now matches reference replay on `01..15`
  - search-space counts still diverge at step `4`, step `9`, step `10`,
    step `11`, step `12`, step `13`, step `14`, and step `15`
  - admissibility diagnostics still diverge at step `9..15`
  - late-step competition still diverges at step `10..15`
- The new certification result is:
  - status `= "attention"`
  - failing checks:
    - `early_breadth`
    - `late_generated_floors`
  - passing checks:
    - `accepted_hash_parity`
    - claim search-policy honesty
    - fallback honesty
    - narrative completeness
    - runtime threshold
    - exact-screen reason completeness
    - prune class completeness
    - manifest completeness
  - the certificate now also emits step-level breadth diagnosis from stored
    step summaries plus `step-XX-live.ndjson`, so the open miss anatomy is
    directly visible on canonical `v9`
  - that stored diagnosis now also preserves the full claim-step-open
    pressure signature for failing steps, including active widening bands,
    package flags, and claim-debt `path` / `trunc` pressure
- The stored breadth snapshot on the parity-clean bundle is:
  - step `1`: `546` versus target `2144` (`miss`)
  - step `10`: `1428` versus target `500` (`hit`)
  - step `11`: `1338` versus target `800` (`hit`)
  - step `12`: `1338` versus target `1200` (`hit`)
  - step `13`: `123` versus target `2200` (`miss`)
  - step `14`: `12027` versus target `3500` (`hit`)
  - step `15`: `1794` versus target `5000` (`miss`)
- The benchmark bundle proves runtime and stored parity are no longer open
  blockers on the current stored bundle:
  - claim run count `= 1`
  - completed step-`15` count `= 1`
  - runtime `= 2848 ms`
  - parity success count `= 1`
  - full early breadth hit count `= 0`
  - full late floor hit count `= 0`
- The current repo head `67c26eca02cb5546745bdd5ca5b31468e6807f42`
  has now revalidated the targeted local guardrails before the next breadth
  repair lands:
  - current step-`9` / step-`11` / step-`12` selector regressions green
  - repaired step-`12` same-primary selector regression green
  - current step-`13` / step-`14` / step-`15` guardrails green
  - claim live-checkpoint persistence green
  - stored `v5` compare / certification / benchmark freeze regressions green
- The current stored step-`13` / step-`14` / step-`15` canonical surfaces are
  now corroborated end to end:
  - step `13` holds `[3,1,3,3,1,1,1]` / `27` / `123`
  - step `14` holds `19683` / `12027`
  - step `15` keeps the canonical `DCT 103 / 8 / 1794` continuation
- The next repair should therefore move off ordinal parity and onto stored
  breadth on the canonical chain, starting at step `13` as the earliest
  remaining late-floor miss.
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
- A new isolated clause-`2` / clause-`3` context regression now sharpens that
  same late blocker further:
  - every remaining isolated clause-`2` or clause-`3` claim-only surface now
    already spans all `32` repaired-side / clause-`6` context masks on the
    captured step-`15` surface
  - forced clause-local reanchor still never restores the exact reference
    terminal anywhere on that isolated context set
  - exact-terminal-only isolated recovery now also sharpens that same read:
    clause-`3` claim-only contexts still collapse only onto unsafe `88 / 8`
    profiles with exact clause `6` and unsafe `74 / 8` profiles with
    claim-only clause `6`, while clause-`2` claim-only contexts still
    collapse only onto unsafe `89 / 8` and `75 / 8` profiles across that same
    clause-`6` split
  - the next repair should therefore stay off isolated clause-`2` /
    clause-`3` relands even behind repaired-side exactness or clause-`6`
    context and keep targeting narrower qualifier evidence on the canonical
    branch itself
- A new exact-suffix clause-`2` / clause-`3` catalog regression now sharpens
  that same late blocker further:
  - on the otherwise exact suffix, the live step-`15` claim catalog at clause
    position `2` still exposes only the two known non-reference claim
    variants
  - both of those clause-`2` catalog variants stay structurally connected,
    locally admissible, outside historical reanchor, and pinned to the unsafe
    `89 / 8` exact-terminal profile
  - clause position `3` likewise still exposes only the two known
    non-reference claim variants, and both stay structurally connected,
    locally admissible, outside historical reanchor, and pinned to the unsafe
    `88 / 8` exact-terminal profile
  - the next repair should therefore stay off any undiscovered exact-suffix
    clause-catalog reland too and keep targeting narrower qualifier evidence
    on the canonical branch itself
- A new nearby clause-`3` anchor-swap regression now sharpens that same late
  blocker further:
  - on the otherwise exact suffix, swapping clause `3` from the exact
    historical anchor `10` to nearby library refs `9` or `11` still leaves
    every tested clause structurally connected, locally admissible, outside
    historical reanchor, and first mismatching the temporal-shell prefix
    exactly at clause `3`
  - the nearby anchor-`9` exact-argument swap still reopens an unsafe
    stronger-than-canonical `65 / 8` profile while the lifted anchor-`9`
    neighbors stay below bar
  - the nearby anchor-`11` exact-argument swap reaches only a noncanonical
    `117 / 8` bar-clearer without restoring qualifier evidence, while the
    lifted anchor-`11` neighbors all reopen unsafe stronger-than-canonical
    `102 / 8` profiles
  - the next repair should therefore stay off any simple nearby clause-`3`
    anchor retarget too and keep targeting narrower qualifier evidence on the
    canonical branch itself
- A new mixed clause-`2` replacement plus nearby clause-`3` anchor-swap
  regression now sharpens that same late blocker further again:
  - pairing any current clause-`2` claim/demo replacement with the nearby
    clause-`3` anchor-`9` neighborhood still leaves all `5` tested
    exact-suffix variants structurally connected, locally admissible, outside
    historical reanchor, and below bar
  - pairing those same clause-`2` replacements with the nearby clause-`3`
    anchor-`11` exact-argument swap now reaches only a
    canonical-primary-but-still-unqualified `103 / 8` bar-clearer, while the
    lifted anchor-`11` neighbors on those same clause-`2` replacement
    contexts still reopen the unsafe stronger-than-canonical `88 / 8`
    profile
  - the next repair should therefore stay off any simple nearby clause-`3`
    anchor retarget or direct clause-`2` replacement reland too; if that
    anchor-`11` neighborhood matters, it has to be isolated without relanding
    the lifted anchor-`11` variants
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
- Two newer non-landed local step-`13` breadth probes now sharpen the next
  safe move further:
  - widening only operator-bundle formation positions `1` and `4` to
    demo-like variants lifts the repaired local read to
    `[3,5,3,3,5,1,1]` / raw `675` / generated `2223`, but it also changes the
    accepted late path to `45 / 7 -> 61 / 9` before step `15`, so that reland
    is not safe
  - that first unsafe position-`1` / position-`4` reland is now frozen as an
    executable negative-control regression on the repaired step-`12` chain:
    the mixed custom step-`13` catalog reproduces raw widths
    `[3,5,3,3,5,1,1]`, raw `675`, live generated `2223`, accepted
    `45 / 7`, and the shifted repaired step-`14` winner profile `61 / 9`
  - widening positions `0`, `4`, `5`, and `6` while keeping position `1`
    exact lifts the repaired local read to
    `[5,1,3,3,5,3,3]` / raw `2025` / generated `2995` and preserves the
    guarded accepted hashes at steps `14` and `15`, but the accepted
    step-`13` hash still flips to a non-reference `46 / 7` shell
  - on that second widened surface, the exact guarded step-`13` telescope
    still evaluates and clears bar locally, but continuing from it no longer
    reproduces the guarded step-`14` accepted hash
  - a naive guarded step-`13` retention/selection reland on top of that
    second widened surface was also explored and reverted:
    it recovers the guarded step-`13` shell into the retained pool but still
    does not close full step-`13 -> step-14` accepted-hash parity
  - the next repair should therefore target a parity-preserving clause-local
    step-`13` widening/retention change rather than relanding either new
    exploratory widened surface raw
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
- A new local step-`11` breadth repair now sharpens the earliest remaining
  late-floor miss before any new stored rerun:
  - claim structural-shell seal variants now count as connected when the
    adjacent step-`11` `kappa = 6` closure clause stays on the existing
    library-backed claim surface
  - the direct connected claim step-`11` surface now widens from
    `kappa 5 = 243`, `kappa 6 = 243` (total `486`) to
    `kappa 5 = 243`, `kappa 6 = 729` (total `972`)
  - live claim search now also keeps that existing step-`11` `kappa = 6`
    surface out of exact frontier screening, so the local step-`11`
    generated-floor regression is now green while the guarded accepted
    step-`11` shell and the repaired step-`12` `34 / 6` continuation stay
    fixed
  - local step-`11` exact-screen connectivity rejections are now `0`
  - dirty-tree `v7` first consumed that repair on stored evidence but reopened
    accepted-hash parity at step `12`
  - the narrow step-`12` same-primary selector repair now lands on top of that
    rerun read
  - clean-tree `v9` now restores accepted-hash parity through step `15` and
    re-earns stored step `11 = 1338 / 800` while keeping guarded
    step `12 = 1338 / 1200`
  - `skills/pen-atomic` is now synced to that same stored-`v9` / step-`13`
    posture, so future sessions should not use the older step-`11` rerun plan
    or the older step-`4` throughput-first claim summary as their starting
    point

## Do This Next

### 1. Freeze `v9` As The Canonical Stored Bundle

1. Freeze `v1`, `v2`, `v3`, capped `v1`, stopped `v4`, completed `v5`, and
   clean canonical completed `v9` as the current stored claim evidence set.
2. Keep clean `v6` frozen as the pre-step-`11` breadth baseline and dirty-tree
   `v7` / `v8` frozen as selector diagnostics.
3. Freeze the new compare / certification / benchmark outputs under `v9`.
4. Keep the replay-harness corpus and benchmark files frozen; do not recapture
   fixtures first.

### 2. Treat Stored Parity And Audit Infrastructure As Earned

1. Treat the pre-flight gate as earned on repo head
   `629b59861d521555cfd214a602bedec4f44c67d0`.
2. Treat one parity-clean clean-start full-profile completion through
   step `15` as earned on the current claim lane.
3. Treat accepted-hash parity, claim-policy honesty, fallback honesty,
   narrative/event completeness, exact-screen reason completeness,
   prune-class completeness, manifest completeness, and compare-ready claim
   audit as earned on stored `v9` evidence.
4. Treat the old step-`9` / step-`11` / step-`12` parity fork, the old
   step-`14` zero-candidate opening, and the old step-`4` runtime wall as no
   longer being the first blockers.

### 3. Hold The Canonical Chain Stable

1. Keep the step-`9` retained-pool / selector regression green, including the
   raw-structural read that still prefers the cheaper non-guarded same-primary
   survivor.
2. Keep the step-`11` retained-pool / selector / continuation-collapse and
   new breadth regressions green on the repaired local chain:
   the connected step-`11` claim surface should stay at
   `kappa 5 = 243`, `kappa 6 = 729` (total `972`), the guarded step-`11`
   shell should stay accepted, and the stored step-`11` floor should stay
   re-earned at `1338 / 800` without reopening the old step-`12` drop.
3. Keep the step-`12` exact-screen / minimality / cache-key / selector /
   continuation-collapse regressions green.
4. Keep the current step-`13` / step-`14` / step-`15` regressions green and
   keep the stored canonical counts frozen at step `13`
   `[3,1,3,3,1,1,1]` / `27` / `123`, step `14` `19683` / `12027`, and
   step `15` `DCT 103 / 8 / 1794`.
5. Keep the new executable step-`13` negative-control regression green:
   widening only operator-bundle formation positions `1` and `4` to the
   demo-like custom surface should still reopen the local floor to
   `[3,5,3,3,5,1,1]` / raw `675` / generated `2223`, but it must also keep
   the unsafe `45 / 7 -> 61 / 9` repaired late path so it remains diagnosis,
   not a landed repair.
6. Keep the stored certification / benchmark snapshot green for the current
   breadth surface:
   step `1` `546 / 2144`, step `10` `1428 / 500`, step `11` `1338 / 800`,
   step `12` `1338 / 1200`, step `13` `123 / 2200`, step `14`
   `12027 / 3500`, and step `15` `1794 / 5000`.
7. Do not reland the rejected global band-`7` widening or the rejected late
   reanchor / early bridge expansions while breadth is still open.

### 4. Diagnose The Remaining Stored Breadth Evidence

1. Start from clean-tree `v9` as the canonical stored bundle; do not reopen
   another stored rerun first unless the next diagnosis explicitly requires
   new evidence capture.
2. Resume diagnosis from stored step `13` as the earliest remaining late-floor
   miss on that clean canonical bundle, using the enriched stored certificate
   first and `step-13-live.ndjson` only when checkpoint-level timing or note
   provenance is needed:
   the current stored read is `[3,1,3,3,1,1,1]` / `27`, `3` seeded roots,
   `0` exact-screen losses there, and a `claim_generic` `kappa 7..7`
   operator-bundle opening with active widening bands `7,8`.
   Keep the two newer local widened probes as negative controls only:
   `[3,5,3,3,5,1,1]` proves the floor can be reopened at the cost of an unsafe
   `45 / 7 -> 61 / 9` path and is now frozen as an executable regression,
   while `[5,1,3,3,5,3,3]` proves the floor can be reopened while preserving
   guarded step-`14` / step-`15` hashes but still loses the guarded
   step-`13` accepted hash.
3. Keep the stored step-`15` miss in view beside that work:
   `1794 / 5000`; the current stored read there is broad `6561` catalog-open
   with `468` partial-prefix bar failures plus `80` incumbent-dominance
   prunes.
4. Keep step `11`, step `12`, and step `14` frozen as positive stored floor
   hits so the next fix cannot silently reshuffle the canonical branch.
5. Keep step `1` `546 / 2144` on the checklist as a separate stored early
   breadth blocker; do not let a late-step repair paper over it.
6. Only treat the certification gate as newly in reach once breadth passes on
   stored evidence without losing accepted-hash parity through step `15`.

## Do Not Reopen First

- a `resume`-based restart of the stopped `v4` run
- another runtime-only step-`4` micro-optimization slice first
- any new step-`13` / step-`14` / step-`15` search-band expansion first:
  hold the current widened later surfaces fixed until the canonical path is
  stable and correct through step `15`
- replay-fixture recapture or benchmark-file churn before the breadth fix
- stronger claim wording or runtime-threshold freeze before a passing
  certificate exists
- another raw step-`9` enumeration or terminal-clause-filter theory first:
  the guarded step-`9` telescope is already present on both of those
  diagnostic surfaces
- another local step-`11` selector or raw-connectivity theory first:
  the local step-`11` breadth repair is already landed and guarded
- another stored/local step-`11` breadth rerun or diagnosis first:
  clean-tree `v9` already re-earns step `11 = 1338 / 800`
- a direct temporal-reanchor matcher reland first:
  the exploratory late branches already disturbed the canonical step-`15`
  continuation
- a raw reland of the newer `[3,5,3,3,5,1,1]` or `[5,1,3,3,5,3,3]`
  step-`13` widenings first:
  those probes reopen local breadth but still leave accepted-hash parity open
- a direct early clause-`2` / clause-`3` reanchor-bridge matcher reland
  first:
  those exploratory branches already displaced the canonical step-`15`
  continuation and remain downstream of the earlier parity repair anyway

## Keep Or Branch Decision

- Keep the current lane on stored breadth repair using parity-clean completed
  `v9` as the canonical stored claim bundle, with clean `v6` frozen as the
  pre-step-`11` breadth baseline and completed `v5` frozen as the pre-parity
  reference surface.
- Keep the new local step-`11` breadth repair and the narrow step-`12`
  selector repair as guarded landed changes while the next work starts at
  stored step `13`, not step `11`.
- Keep the current short-loop gate, stored step-`4` continuation references,
  the capped intended-profile read, stopped `v4`, and completed
  `v5` / `v6` / `v9` frozen as regression checks.
- Return to another runtime-only slice only if the stored targeted parity/floor
  regressions prove the remaining misses are just accounting bugs rather than
  real breadth loss on the canonical chain.
