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
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v11`,
  now exists on clean-tree repo head
  `4e38f2463b429bcbebe16cc6d7c5eac7ef2de050` with release binary hash
  `64535f9e6c2e2a77c1bdeeb1f848abbeb0312f9b454bce42f27c68b3b852271c`.
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
- The earlier clean-tree completed `v10` bundle remains frozen as the
  pre-anchor-`11` stored baseline for the remaining step-`15` repair.
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
    directly visible on canonical `v11`
  - that stored diagnosis now also preserves the full claim-step-open
    pressure signature for failing steps, including active widening bands,
    package flags, and claim-debt `path` / `trunc` pressure
- A new local step-`15` survivor-bucket freeze now localizes the remaining
  exact-screened survivor surface more tightly on the repaired canonical
  chain:
  - `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`
    now pins exactly two
    `k8:structural_generic:temporal_operator:library_backed` buckets at
    step `15`
  - the isolated `single` bucket still carries just one fully scored
    non-winning pocket with overshoot `115657 / 21112`
  - the remaining exact-screened pressure still stays concentrated in the
    `small_cluster` bucket with `2190` generated / `244` admitted /
    `244` exact-screened / `242` pruned
  - the accepted canonical step-`15` winner still remains the only retained
    candidate there and keeps bit cost `229`
- A new local step-`15` small-cluster incumbent-surface regression now
  sharpens that same remaining blocker further:
  - `current_claim_step_fifteen_small_cluster_incumbent_surface_stays_same_primary_and_non_winning`
    now pins that all `242` remaining `small_cluster` prunes happen during
    summary-stage exact screening rather than later proof-close materialization
  - every one of those remaining pruned candidates still stays on the same
    primary `103 / 8` profile with overshoot `115657 / 21112`
  - the remaining non-winning spread there is now only secondary bit cost,
    from `236` through `290`, above the canonical winner's `229`
  - the earliest first-mismatch positions across that still-pruned surface are
    now frozen at clause positions `0`, `1`, `2`, `4`, and `5` with counts
    `162`, `54`, `18`, `6`, and `2`
  - the next repair should therefore stay on that narrow summary-stage
    same-primary `small_cluster` incumbent surface rather than reopening
    proof-close ordering or the isolated `single` pocket first
- A new local step-`15` omitted-side-variant regression now sharpens that same
  next move again without landing another widening yet:
  - `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_stay_same_primary_and_non_winning`
    now pins that the omitted demo-only temporal-shell side variants at clause
    positions `0`, `1`, `4`, and `5` all stay structurally connected, locally
    admissible, and same-primary `103 / 8` / `115657 / 21112` on top of the
    current live claim clause-`2` + anchor-`11` exact-argument pocket
  - those omitted side variants still stay outside historical reanchor there
    and still lose only on higher bit cost `243`, `245`, or `250`
  - a non-landed raw position-`0` reland probe briefly lifted the local
    step-`15` generated surface to `4285`, but it also reopened the old
    clause-`0` zero-admitted capture to `2835` families with
    `891` clause-`0` historical-reanchor misses, so that broad catalog reland
    was reverted
  - the next repair should therefore isolate one of those omitted side
    openings onto the existing anchor-`11` `small_cluster` path instead of
    broadening the raw position-`0` temporal-shell catalog first
- Two follow-up isolated late-side reland probes were then tried and reverted:
  - a clause-`4`-only demo-sharp-codomain opening, gated on canonical clauses
    `0` and `1` plus the live claim clause-`2` + anchor-`11` exact-argument
    pocket, lifted local step `15` generated prefixes only to `3980`, but it
    also reopened the zero-admitted capture to `1962` families with `1476`
    clause-`2` historical-reanchor-prefix misses
  - a clause-`5`-only demo-sharp-domain opening, gated on canonical clauses
    `0`, `1`, and `4` plus that same pocket, lifted the local surface only to
    `3974` while still reopening the zero-admitted capture to `1950` families
    with `1464` clause-`2` historical-reanchor-prefix misses
  - the next repair should therefore not be a raw isolated clause-`4` or
    clause-`5` late-side reland either; any future use of those openings will
    need extra qualifier/reanchor evidence or summary-stage retention changes
    that keep them on the `small_cluster` path instead of feeding the
    captured clause-`2` / clause-`3` surface
- New stored-evidence regressions now freeze that canonical `v11`
  diagnosis in-tree:
  - `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`
    now pins the stored step-`1` miss, the full late-floor table on `10..15`,
    the fact that only step `15` still misses there, and the frozen
    `step-15-live.ndjson` provenance note sequence
    `claim_regular_clause_catalog -> claim_root_seeding_summary`
  - `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts` now pins the
    refreshed single-run `3965 ms` / parity-`1` / early-`0` / late-`0`
    stored benchmark bundle
- The stored breadth snapshot on the parity-clean bundle is:
  - step `1`: `546` versus target `2144` (`miss`)
  - step `10`: `1428` versus target `500` (`hit`)
  - step `11`: `1338` versus target `800` (`hit`)
  - step `12`: `1338` versus target `1200` (`hit`)
  - step `13`: `2320` versus target `2200` (`hit`)
  - step `14`: `12027` versus target `3500` (`hit`)
  - step `15`: `3972` versus target `5000` (`miss`)
- The benchmark bundle proves runtime and stored parity are no longer open
  blockers on the current stored bundle:
  - claim run count `= 1`
  - completed step-`15` count `= 1`
  - runtime `= 3965 ms`
  - parity success count `= 1`
  - full early breadth hit count `= 0`
  - full late floor hit count `= 0`
- The current repo head `4e38f2463b429bcbebe16cc6d7c5eac7ef2de050`
  has now revalidated the targeted local guardrails and stored rerun before
  the next breadth repair lands:
  - current step-`9` / step-`11` / step-`12` selector regressions green
  - repaired step-`12` same-primary selector regression green
  - current step-`13` / step-`14` / step-`15` guardrails green
  - claim live-checkpoint persistence green
  - stored `v11` certificate / benchmark freeze regressions green
  - stored `v5` compare / certification / benchmark freeze regressions green
- The parity-preserving step-`13` breadth repair is now earned on stored
  evidence:
  - on the canonical repaired chain, claim-open now widens to
    `[5,1,3,3,5,3,2]` / raw `1350` / generated `2320`
  - the guarded step-`13` metric shell stays accepted there while the
    canonical later continuation remains
    `46 / 7 / 2320 -> 62 / 9 / 12027 -> 103 / 8 / 3972`
  - step-`13` root seeding now rises to `5` seen / `5` enqueued on the
    stored canonical branch
  - the stored step-`13` surface now pays `576`
    legality/connectivity exact rejections and `401` heuristic drops before
    proof-close
  - the two executable step-`13` negative controls remain frozen unchanged:
    `[3,5,3,3,5,1,1]` still reopens only the unsafe `45 / 7 -> 61 / 9` path,
    and `[5,1,3,3,5,3,3]` still preserves guarded step-`14` / step-`15`
    hashes while keeping step-`13` parity open
- The next engineering dollar is therefore no longer another rerun first and
  no longer another step-`13` clause-local theory pass first; it is stored
  step-`15` diagnosis / repair on top of this new canonical `v11` bundle,
  while keeping step `1` explicit as the separate early breadth blocker.
- The current stored step-`13` / step-`14` / step-`15` canonical surfaces are
  now corroborated end to end:
  - step `13` holds `[5,1,3,3,5,3,2]` / `1350` / `2320`
  - step `14` holds `19683` / `12027`
  - step `15` keeps the canonical `DCT 103 / 8 / 3972` continuation
- The next repair should therefore move off ordinal parity and onto stored
  breadth on the canonical chain, starting at step `15` as the remaining
  late-floor miss while keeping step `1` explicit on the checklist.
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
- A new anchor-`11` exact-argument rank regression now sharpens that same
  late blocker further again:
  - on every current mixed clause-`2` claim/demo replacement, the nearby
    clause-`3` anchor-`11` exact-argument pocket now freezes as a full-rank
    `103 / 8` bar-clearer
  - that pocket now matches the canonical `DCT 103 / 8` winner on overshoot,
    clause `kappa`, eliminator/former/density/library/binder/closure signals,
    max var reach, and `nu`, and still loses only on higher bit cost
    `236` versus canonical `229`
  - the lifted nearby anchor-`11` neighbors still reopen the unsafe `88 / 8`
    rival, so the next repair should only touch that neighborhood if it can
    isolate the exact-argument qualifier evidence without relanding the
    lifted variants or replacing the canonical accepted path
- A new narrow local step-`15` anchor-`11` exact-argument repair now lands on
  top of that diagnosis:
  - mixed live claim clause-`2` prefixes now expose exactly one additional
    clause-`3` option, the isolated anchor-`11` exact-argument pocket, while
    reference clause-`2` prefixes and the lifted anchor-`11` neighbors stay
    out of the live clause-`3` catalog
  - the live claim clause-`2` variants now regain historical reanchor on that
    isolated pocket across every repaired-side subset of clause positions
    `0`, `1`, `4`, and `5`, but clause `6` still stays the local safety
    boundary: once clause `6` also deviates, the same pocket reopens the
    unsafe `89 / 8` rival and remains fenced
  - on the repaired canonical late chain, step `15` still accepts
    `DCT 103 / 8`, but live generated prefixes now lift from `1794` to
    `3972`
  - step-`15` partial-prefix bar failures stay `468` while incumbent-
    dominance prunes rise from `80` to `242`
  - the surviving temporal terminal cluster now widens to
    `2190` generated / `244` admitted / `244` exact-screened / `242` pruned,
    and one isolated fully scored non-winning anchor-`11` terminal pocket is
    now frozen with overshoot `115657 / 21112`
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
  - that second unsafe position-`0` / position-`4` / position-`5` /
    position-`6` reland is now also frozen as an executable negative-control
    regression on the repaired step-`12` chain:
    the mixed custom step-`13` catalog uses the full demo slices at positions
    `0` and `4` plus the original three-option demo slice at positions `5`
    and `6`, reproducing raw widths `[5,1,3,3,5,3,3]`, raw `2025`,
    live generated `2995`, accepted `46 / 7`, and the guarded accepted hashes
    at steps `14` and `15`
  - the current reverted code still keeps the exact guarded step-`13` shell
    out of the retained pool on that second widened surface, so it remains
    diagnosis only and not a landed parity repair
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

### 1. Freeze `v11` As The Canonical Stored Bundle

1. Freeze `v1`, `v2`, `v3`, capped `v1`, stopped `v4`, completed `v5`, clean
   canonical completed `v10`, and clean canonical completed `v11` as the
   current stored claim evidence set.
2. Keep clean `v6` frozen as the pre-step-`11` breadth baseline and dirty-tree
   `v7` / `v8` frozen as selector diagnostics.
3. Freeze the new compare / certification / benchmark outputs under `v11`.
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
   audit as earned on stored `v11` evidence.
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
4. Keep the current local step-`13` / step-`14` / step-`15` regressions green
   and keep both the local and stored repaired surfaces straight:
   - local repaired step `13` should stay
     `[5,1,3,3,5,3,2]` / `1350` / `2320` with the guarded accepted hash
   - stored canonical `v11` step `13` now matches that same
     `[5,1,3,3,5,3,2]` / `1350` / `2320` surface
   - step `14` should stay `19683` / `12027`
   - stored canonical step `15` should now stay `DCT 103 / 8 / 3972`
   - the exact-screened step-`15` survivor surface should stay frozen as one
     library-backed temporal-operator `single` bucket with one fully scored
     non-winning pocket at overshoot `115657 / 21112` plus one
     library-backed temporal-operator `small_cluster` bucket at
     `2190` generated / `244` admitted / `244` exact-screened / `242`
     pruned
   - the remaining `small_cluster` incumbent surface should also stay frozen
     as `242` summary-stage same-primary `103 / 8` prunes with bit costs
     `236..290` and earliest first-mismatch counts
     `162 / 54 / 18 / 6 / 2` at clause positions `0 / 1 / 2 / 4 / 5`
5. Keep the two executable step-`13` negative-control regressions green:
   widening only operator-bundle formation positions `1` and `4` to the
   demo-like custom surface should still reopen the local floor to
   `[3,5,3,3,5,1,1]` / raw `675` / generated `2223`, but it must also keep
   the unsafe `45 / 7 -> 61 / 9` repaired late path so it remains diagnosis,
   not a landed repair;
   widening positions `0`, `4`, `5`, and `6` via the mixed custom surface
   (full demo slices at positions `0` and `4`, original three-option demo
   slices at positions `5` and `6`) should still reopen the local floor to
   `[5,1,3,3,5,3,3]` / raw `2025` / generated `2995` while preserving the
   guarded step-`14` / step-`15` accepted hashes but displacing step `13` to
   a non-reference `46 / 7` shell.
6. Keep the stored certification / benchmark snapshot green for the current
   breadth surface:
   step `1` `546 / 2144`, step `10` `1428 / 500`, step `11` `1338 / 800`,
   step `12` `1338 / 1200`, step `13` `2320 / 2200`, step `14`
   `12027 / 3500`, and step `15` `3972 / 5000`; keep
   `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`
   plus `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts` green
   as the executable freeze for that evidence surface.
7. Do not reland the rejected global band-`7` widening or the rejected late
   reanchor / early bridge expansions while breadth is still open.

### 4. Repair The Residual Stored Step-`15` Gap On Canonical `v11`

1. Start from clean-tree stored `v11` as the canonical reference surface; do
   not spend another cycle on rerun setup or step-`13` diagnosis first now
   that stored step `13` is closed there and the anchor-`11` repair is already
   consumed on stored evidence.
2. Treat the residual stored `3972 / 5000` step-`15` gap as the current
   diagnosis target; keep the stored `v11` certificate/live-note regression
   green while the next local repair is isolated instead of reopening another
   theory pass first.
3. Keep the stored step-`13` hit frozen while doing that work, and keep the
   repaired local late chain explicit:
   `[5,1,3,3,5,3,2]` / `1350` / `2320`, `5` seeded roots, and the guarded
   `46 / 7 -> 62 / 9 -> 103 / 8` continuation should remain the canonical
   breadth-repaired late chain; stored step `15` now carries `3972`
   generated prefixes there, and the next repair should work against the
   remaining `468` partial-prefix bar / `242` incumbent-dominance surface
   rather than the older `1794` read, with the new survivor-bucket freeze
   keeping the isolated `single` pocket fenced while the next repair targets
   the `small_cluster` pressure; that pressure is now pinned more tightly as
   `242` summary-stage same-primary `103 / 8` incumbents with bit costs
   `236..290` and earliest first mismatches at clause positions
   `0`, `1`, `2`, `4`, and `5`; the new omitted-side-variant regression now
   also proves that several demo-only side openings around the live
   anchor-`11` pocket are already rank-safe, while the reverted raw
   position-`0` reland proves that broadening that side globally reopens the
   old captured clause-`0` surface, and the reverted isolated clause-`4` /
   clause-`5` relands prove that a raw late-side next-clause injection still
   feeds the captured clause-`2` / clause-`3` surface, so the next landed
   repair must stay narrower than all three of those raw relands.
4. Keep step `11`, step `12`, and step `14` frozen as positive stored floor
   hits so the next repair cannot silently reshuffle the canonical branch.
5. Keep step `1 = 546 / 2144` on the checklist as the separate stored early
   breadth blocker; do not let stored step-`13` closure paper over it.
6. Launch the next clean full-profile rerun only after the next local
   step-`15` repair is regression-backed and parity-clean, then refresh
   compare / benchmark / certification immediately afterwards; only treat the
   certification gate as newly in reach once that rerun proves broader stored
   breadth without losing accepted-hash parity through step `15`.

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
- a raw global position-`0` temporal-shell reland first:
  the exploratory probe lifted local step `15` only to `4285` but also
  reopened the repaired clause-`0` zero-admitted capture
- a raw isolated clause-`4` or clause-`5` late-side reland first:
  the reverted pocket-gated probes only lifted local step `15` to `3980` or
  `3974` and still reopened the captured clause-`2` / clause-`3` surface
- a direct early clause-`2` / clause-`3` reanchor-bridge matcher reland
  first:
  those exploratory branches already displaced the canonical step-`15`
  continuation and remain downstream of the earlier parity repair anyway

## Keep Or Branch Decision

- Keep the current lane on stored breadth repair using parity-clean completed
  `v11` as the canonical stored claim bundle, with clean `v10` frozen as the
  pre-anchor-`11` stored baseline, clean `v6` frozen as the pre-step-`11`
  breadth baseline, and completed `v5` frozen as the pre-parity reference
  surface.
- Keep the new local step-`11` breadth repair and the narrow step-`12`
  selector repair as guarded landed changes while the next work starts at
  stored step `15`, not step `11` and not stored step `13`.
- Keep the current short-loop gate, stored step-`4` continuation references,
  the capped intended-profile read, stopped `v4`, and completed
  `v5` / `v6` / `v10` / `v11` frozen as regression checks.
- Return to another runtime-only slice only if the stored targeted parity/floor
  regressions prove the remaining misses are just accounting bugs rather than
  real breadth loss on the canonical chain.
