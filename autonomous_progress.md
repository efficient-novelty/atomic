# Autonomous Claim Lane Progress

Last updated: 2026-04-04

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
- The stored failure
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v3`
  remains the finished pre-repair late-step reference:
  it reached step `14` and failed with
  `failure_note = "no atomic candidates were generated for step 14"`.
- The capped intended-profile read
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-late-accept-capped-v1`
  and the stopped rerun
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v4`
  remain frozen as the pre-repair early-runtime evidence set.
- The short pre-flight gate was rerun on clean-tree repo head
  `c1fbb51d4fc9a620cd2ce95c9c3eadfe1a54fc65` with release binary hash
  `c42758f96c8171900651503d7f2a0ffe9915966c41edea98d8f1e296fc772a4e`:
  - targeted `desktop_claim_shadow` regressions green
  - claim live-checkpoint persistence green
  - release replay harness benchmark replayed all `5` stored
    `remaining_one_plateau` surfaces without mismatch
- A fresh clean-start full-profile rerun,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v5`,
  now exists on that same clean-tree head and binary.
- Its authoritative `run.json` state is:
  - `status = "completed"`
  - `completed_step = 15`
  - `active_step = 16`
  - `active_band = 8`
  - `frontier_epoch = 12`
  - `resolved_worker_count = 8`
  - `dirty_tree = false`
- The new `v5` bundle now stores the full claim artifact surface through
  step `15`:
  - run manifest with complete host/build/git/binary provenance
  - step summaries through `step-15-summary.json`
  - live-checkpoint streams through `step-15-live.ndjson`
  - per-step narratives and event streams through step `15`
  - frontier and step checkpoints for the completed run
- Stored audit outputs now also exist under that `v5` run directory:
  - `claim-compare.txt` / `claim-compare.json`
  - `claim_certificate.txt` / `claim_certificate.json`
  - `claim_benchmark.txt` / `claim_benchmark.json`
- Those audits use the guarded baseline
  `runs/codex-guarded-claim-cert-v1`.
- A narrow claim-lane code change now lands on top of that stored `v5`
  evidence surface:
  claim incumbent pruning preserves same-primary accepted-rank ties on
  steps `9..12`, so mid-claim parity candidates are no longer discarded
  before final selection on secondary structural tiebreaks alone.
- A new unit regression now proves the guarded step-`11` completion:
  - is still present in the claim remaining-one terminal-clause catalog on
    the live divergent history
  - still survives the full remaining-one summary and the compact survivor
    sketch even with an incumbent present
  - now survives into the retained claim candidate pool instead of being
    dropped by incumbent pruning
- A follow-up claim-lane code change now repairs the local step-`11`
  same-primary selector:
  live claim acceptance now prefers the guarded step-`11` survivor over the
  richer raw structural winner by preferring the leaner same-primary
  former/binder shell first, then stronger dependent density within that lean
  tier, then lower bit cost.
- New unit regressions now freeze the post-repair step-`11` read:
  - the raw structural same-primary winner still does not pick the guarded
    step-`11` hash
  - live claim step-`11` acceptance now does pick the guarded same-primary
    survivor
  - every current same-primary tied step-`11` survivor still keeps step `12`
    alive
  - those tied step-`11` survivors now collapse onto one observed
    step-`12` accepted hash / `nu` / `kappa` at guarded `34 / 6`, not the old
    `33 / 5` drop
  - so the step-`11` local selection repair is landed, and the old
    step-`12` `33 / 5` drop is no longer the remaining local blocker
- A new follow-up claim-lane code change now narrows the local step-`12`
  blocker further:
  claim open-band semantic minimality no longer lets worse-ranked detachable
  subbundles prune the guarded step-`12` curvature shell, and the claim
  remaining-one algebraic ceiling no longer screens that shell out early on
  the modal `5..6` slice.
- A second follow-up claim-lane code change now repairs the local step-`12`
  retained-pool miss:
  exact multi-step partial-prefix bound memoization now keys on
  `(prefix_signature, clause_kappa)`, so the modal `5`-clause
  `CannotClearBar` result no longer poisons the guarded `6`-clause
  continuation on the same preterminal prefix.
- A third follow-up claim-lane code change now repairs the local step-`12`
  same-primary selector:
  live claim acceptance now prefers the guarded curvature shell over richer
  same-primary `34 / 6` survivors by preferring the leaner former/binder
  shell first, then the stronger introduction application spine within that
  lean tier, then the shallower formation-clause var reach, then lower bit
  cost.
- New unit regressions now freeze the post-repair step-`12` read:
  - every current same-primary tied step-`11` survivor still collapses onto
    one observed live claim step-`12` continuation
  - that continuation now recovers guarded `nu = 34`, `kappa = 6`
  - the guarded step-`12` curvature shell now survives preterminal clause
    exposure, full remaining-one summary, compact survivor sketch, semantic
    minimality, and exact remaining-one screening on the repaired live history
  - it now also survives into the retained candidate pool on that repaired
    live history
  - the raw structural same-primary winner still does not pick the guarded
    step-`12` hash
  - live claim step-`12` acceptance now does pick the guarded same-primary
    curvature shell
  - every current same-primary tied step-`12` survivor still collapses onto
    one observed step-`13..15` continuation and the same late generated counts
    `33`, `12027`, and `780`
  - the step-`12` accepted-hash fork is therefore now closed locally and does
    not explain the late generated-floor collapse by itself
- A new local late-surface repair now sharpens the remaining blocker further:
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
  - step `15` on the restored canonical branch still opens a `6561`-telescope
    claim catalog, but `512` exact partial-prefix bar failures still shrink
    that live generated surface to `780` before proof-close
- A new local step-`15` exact-prune family regression now sharpens that late
  blocker further:
  - the captured repaired-canonical step-`15` exact-prune surface currently
    consists of `2184` zero-admitted terminal families with no cached compact
    bounds
  - those captured prefixes therefore are not hiding a bar-clearer behind the
    compact summary path; they currently expose no admitted terminal
    completions at all on the canonical temporal-shell branch
- A new follow-up step-`15` zero-admitted connectivity regression now sharpens
  that same late blocker further:
  - all `6552` generated terminal options across those captured `2184`
    exact-pruned families currently disconnect before admissibility
  - there are `0` trivially-derivable, structural-debt-cap, or other
    exact-legality rejections anywhere on that captured surface
  - step `15` remains the first late floor miss, but the next repair is now
    more specific than generic partial-prefix bar accounting: inspect terminal
    clause exposure and exact connectivity on that canonical temporal-shell
    surface
- A new follow-up step-`15` connectivity-classification regression now sharpens
  that same blocker one layer further:
  - those `6552` generated terminal options are not structurally disconnected
  - all `6552` are instead structurally connected but still fail as
    `NeedsFallback` candidates
  - none currently qualify active-window connectivity, self-contained
    connectivity, or historical reanchor on that captured canonical
    temporal-shell surface
  - the remaining local step-`15` problem is therefore qualifier / reanchor
    evidence on the existing connected surface, not raw dependency-edge
    generation
- A new follow-up step-`15` reanchor-prefix regression now sharpens that same
  blocker one layer further:
  - across the captured `2184` zero-admitted exact-prune families, historical
    reanchor prefix progress now breaks at clause positions `0..5` with counts
    `1458`, `486`, `162`, `54`, `18`, and `6`
  - none of those captured families preserve a full seven-clause temporal-shell
    prefix, so the current missed qualifier surface never even reaches the last
    two temporal-shell clause slots intact
  - the next step-`15` repair should therefore inspect earlier temporal-shell
    prefix qualification on the canonical branch, not just the terminal clause
    matcher in isolation
- A new follow-up step-`15` clause-`6` boundary regression now sharpens that
  same blocker one layer further:
  - once the first six temporal-shell clauses are fixed, the current claim lane
    keeps exactly `3` clause-`6` prefixes outside the captured exact-prune
    surface: the exact historical-reanchor continuation plus `2` non-reanchor
    claim variants
  - none of those `3` clause-`6` prefixes belong to the captured `2184`
    zero-admitted exact-prune families
  - only the exact clause-`6` prefix still exposes a
    `KeepWithoutFallback` terminal continuation; the two claim-only clause-`6`
    variants remain pure `NeedsFallback`
  - the remaining captured exact-prune blocker is therefore earlier qualifier
    evidence through clause `5` or earlier; clause `6` is now downstream of
    that capture boundary
- A new follow-up step-`15` isolated-prefix regression now sharpens that same
  blocker one layer further:
  - on the otherwise exact seven-clause temporal-shell prefix, the captured
    exact-prune surface now contains exactly `2` isolated claim-only variants
    at each early clause position `0..5`
  - all `12` of those single early deviations still classify all `3` terminal
    continuations as `NeedsFallback` and remain zero-admitted
  - the remaining step-`15` repair is therefore clause-local qualifier /
    reanchor evidence at positions `0..5` themselves, not a multi-deviation
    interaction, clause-`6`, or terminal-slot problem
- A new follow-up step-`15` variant-product regression now sharpens that same
  blocker one layer further:
  - for each early clause position `0..5`, those `2` isolated claim-only
    local variants each stay captured across every later claim suffix
    combination before the clause-`6` boundary, with per-variant suffix counts
    `729`, `243`, `81`, `27`, `9`, and `3`
  - the remaining step-`15` repair is therefore not hiding in later-suffix
    interaction on that captured surface; it is clause-local qualifier /
    reanchor evidence on those exact local variants themselves
- A new follow-up step-`15` forced-reanchor recovery regression now sharpens
  that same blocker one layer further:
  - on each of those `12` isolated early claim-only temporal-shell deviations,
    restoring clause-local reanchor evidence would flip all `3` terminal
    continuations to `KeepWithoutFallback`
  - all `36` of those forced-reanchor local continuations are admitted and
    bar-clearing on the otherwise exact suffix
  - the remaining step-`15` repair is therefore a pure clause-local qualifier /
    reanchor evidence miss at positions `0..5` themselves, not downstream
    admissibility or bar arithmetic once the suffix is fixed
- A follow-up exploratory step-`15` temporal-reanchor broadening pass was also
  run locally but was not landed:
  - a broad temporal-shell matcher expansion collapsed the captured exact-prune
    surface, but it also displaced the canonical step-`15` continuation from
    `103 / 8 / 780` to `60 / 8 / 9840`
  - a narrower late-shell-only reanchor reland still displaced the canonical
    continuation to `89 / 8 / 780`
  - both variants were reverted, so the next step-`15` repair should not be a
    direct matcher broadening reland
- A second follow-up exploratory early-bridge reanchor pass was also run
  locally but was not landed:
  - broadening only the clause-`3` historical-reanchor anchor to accept the
    flat-bridge argument displaced the canonical step-`15` continuation from
    `103 / 8 / 780` to `88 / 8 / 795`
  - widening the earlier clause-`2` plus clause-`3` bridge pair displaced the
    canonical continuation further to `74 / 8 / 828`
  - both variants were reverted, and the current step-`15`
    `103 / 8 / 780` baseline plus the exact-prune / connectivity /
    reanchor-prefix regressions were revalidated afterwards
  - the next step-`15` repair should therefore not directly broaden the early
    clause-`2` / clause-`3` reanchor bridge matcher either
- No fresh full-profile rerun or stored audit refresh has consumed those local
  repairs yet, so the frozen `v5` run, compare report, claim certificate, and
  benchmark bundle remain the authoritative stored evidence surface.
- New stored-evidence regressions now freeze that `v5` audit surface in-tree:
  - compare assertions pin the step-`9`, step-`11`, and step-`12`
    accepted-hash / trajectory forks
  - certification assertions pin the step-`1` breadth miss plus the
    step-`10..15` late generated-floor snapshot
  - benchmark assertions pin the single-run `408 ms` / parity-`0` /
    breadth-hit-`0` aggregate bundle
- A new local step-`13` catalog regression now freezes the repaired
  breadth read more precisely:
  claim-open now sits at `kappa = 7..7` on `LateFamilySurface::ClaimGeneric`
  with scoped widths `[3,1,3,3,1,1,1]`, raw catalog `27`, and live generated
  prefixes `33` before proof-close while the guarded step-`13` metric shell
  stays accepted, so the next late read is no longer pure catalog-open
  starvation but the residual exact-screen losses on that widened surface.
- A new follow-up step-`13` exact-prune/connectivity regression now sharpens
  that same widened-surface blocker further:
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
- A new follow-up step-`13` structural-connectivity repair now closes that
  widened-surface blocker on the repaired step-`12` chain:
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
- A follow-up exploratory step-`13` widening pass was also run locally but was
  not landed:
  - a naive global claim-generic band-`7` widening lifted the repaired
    claim-open read from raw widths `[3,1,1,1,1,1,1]` / raw `3` /
    generated `9` to `[3,3,3,3,3,3,3]` / raw `2187` / generated `615`
  - root seeding on that branch widened cleanly to `roots_seen = 3`,
    `roots_rejected_by_exact_screen = 0`, and `roots_enqueued = 3`
  - the remaining step-`13` loss there then shifted mostly into exact
    legality/connectivity rejection (`1954`), partial-prefix bar failure
    (`168`), and incumbent dominance (`236`)
  - the late local path on that branch became
    `(13,45,7,615) -> (14,61,9,12027) -> (15,103,8,780)`
  - but that same global reland disturbed unrelated claim prefix-memo,
    realistic-shadow, demo-lane, and divergent step-`13` / step-`14`
    guardrails, so it was reverted and should not be treated as the next
    landable fix by itself

## Latest Full-Profile Outcome

- The new `v5` rerun closes the old completion blocker:
  `desktop_claim_shadow` now has one fresh clean-start full-profile bundle
  from the disclosed desktop that finishes through step `15`.
- Its terminal summary shows the accepted executable canon is still step `15`
  / `DCT` at `nu = 103`, `kappa = 8`.
- The old `v3` step-`14` zero-candidate opening is no longer the first live
  blocker:
  - `v5` reaches step `14`
  - step `14` opens at claim band `9..9`
  - step `14` records `roots_seen = 1` and `roots_enqueued = 1`
  - step `14` accepts a real survivor and then advances to step `15`
- The new compare audit against `codex-guarded-claim-cert-v1` shows the
  completed run is still not parity-clean:
  - trajectory diverges at step `9`, step `11`, step `12`, step `13`,
    step `14`, and step `15`
  - accepted hashes diverge at step `9`, step `11`, and step `12`
  - search-space counts diverge at step `4`, step `9`, step `10`,
    step `11`, step `12`, and step `14`
  - admissibility diagnostics diverge at step `9`, step `10`, step `11`,
    step `12`, step `13`, step `14`, and step `15`
- The earliest stored claim-specific accepted-hash fork is now localized:
  - step `9` keeps `nu = 17`, `kappa = 4`, but accepts a different hash than
    guarded replay
  - step `10` temporarily realigns on the guarded accepted hash
  - step `11` again keeps `nu = 26`, `kappa = 5`, but accepts a different hash
  - step `12` then falls to `nu = 33`, `kappa = 5` versus guarded
    `nu = 34`, `kappa = 6`
  - step `13` and step `14` reuse the guarded accepted hashes again, but on
    lower `nu` than guarded replay (`45` vs `46`, then `61` vs `62`)
  - step `15` returns to the guarded accepted hash / `nu` / `kappa`
    while replay ablation still records path divergence
- The certification audit is now explicit and honest:
  - status `= "attention"`
  - failing checks:
    - `accepted_hash_parity`
    - `early_breadth`
    - `late_generated_floors`
  - passing checks:
    - `search_policy`
    - `fallback_honesty`
    - `narrative_artifacts`
    - `runtime_threshold`
    - `exact_screen_reason_completeness`
    - `prune_class_completeness`
    - `manifest_completeness`
- The new completed run proves the claim artifacts are now operationally rich
  enough for certification-style auditing even though the lane still fails the
  content gates:
  - claim policy metadata is honest
  - fallback honesty is clear
  - narrative/event artifacts are complete
  - exact-screen reason counts are complete through step `15`
  - prune class counts are complete through step `15`
  - manifest provenance is complete on the stored release build
- Runtime is no longer the live blocker on the stored full-profile slice:
  - the benchmark bundle records one completed step-`15` claim run
  - stored runtime for `v5` is `408 ms`
  - the audit passes the provisional `600000 ms` runtime threshold
- The remaining breadth/floor failures are now concrete on stored evidence:
  - early breadth:
    - step `1` generated `546`, target `2144`
  - late generated floors:
    - step `10`: `1428` against target `500` (`hit`)
    - step `11`: `546` against target `800` (`miss`)
    - step `12`: `930` against target `1200` (`miss`)
    - step `13`: `9` against target `2200` (`miss`)
    - step `14`: `157` against target `3500` (`miss`)
    - step `15`: `780` against target `5000` (`miss`)

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

### Fresh Completed Full-Profile Claim Bundle

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v5`
- Launch surface:
  - clean-tree repo head
    `c1fbb51d4fc9a620cd2ce95c9c3eadfe1a54fc65`
  - release binary hash
    `c42758f96c8171900651503d7f2a0ffe9915966c41edea98d8f1e296fc772a4e`
- Full-profile outcome:
  - `status = "completed"`
  - `completed_step = 15`
  - `active_step = 16`
  - step `15` accepted `DCT` with `nu = 103`, `kappa = 8`
- Stored audits under the same run directory:
  - compare report against `runs/codex-guarded-claim-cert-v1`
  - failing-but-honest claim certificate
  - single-run benchmark bundle

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
- late-step `claim_step_open` and `claim_root_seeding` diagnostics
- late-step step-`13` viability-tie acceptance repair
- early guarded-package parity repair through steps `4..8`
- stored compare / certification / benchmark outputs on one completed
  full-profile claim bundle

## Current Diagnosis

- The old early RSS cliff remains gone.
- Step `4` throughput is no longer the blocker that determines the next slice:
  the stored `v5` run completes the full claim profile in one clean-start pass.
- The old `v3` step-`14` zero-candidate opening is also no longer the first
  blocker:
  `v5` reaches step `14`, seeds roots, accepts a survivor there, and advances
  to step `15`.
- The current blocker is now stored parity plus stored breadth:
  - accepted-hash parity is still open through step `15`
  - the earliest claim-specific accepted-hash divergence is step `9`
  - the largest generated-floor collapse now localizes at steps `11..15`,
    with the local step-`14` catalog miss now repaired and the remaining live
    floor pressure concentrated at step `13` plus the canonical step-`15`
    exact-screen path
  - step `15` still returns the accepted executable canon, so the lane is
    now failing by taking a too-thin or wrong route to the same endpoint
- A new local step-`9` diagnostic pass narrows the earliest fork further:
  - the guarded step-`9` telescope is still present in late claim
    enumeration (`81` `kappa = 4` claim telescopes)
  - it still passes claim open-band admissibility and connectivity on the
    guarded step-`8` prefix
  - the guarded step-`9` three-clause prefix still exposes the guarded
    closing clause through the claim remaining-one terminal-clause filter
  - a local no-drop frontier experiment can carry that guarded step-`9`
    telescope into the retained claim candidate set, so the earliest fork is
    no longer best explained as raw generation loss or terminal-clause
    filtering loss
  - however, that same experiment shows the tied step-`9` candidates still
    collapse onto the same observed repaired step-`10..12` chain, so step `9`
    still should not be treated as a closed local selection bug while the late
    step-`13` / step-`15` breadth story remains open
- A new local step-`11` diagnostic plus narrow incumbent-pruning and selector
  repairs now refine that blocker:
  - the guarded step-`11` completion is still present in the claim
    remaining-one clause catalog on the divergent claim history
  - it still survives the full remaining-one summary and the compact
    survivor sketch even with an incumbent present
  - it now survives into the retained candidate pool on steps `9..12`
  - live claim step-`11` acceptance now prefers the guarded same-primary
    survivor over the richer raw structural winner
  - every current same-primary tied step-`11` survivor now collapses onto one
    observed step-`12` `34 / 6` continuation
  - so step `11` is no longer best described as raw generation loss,
    compact-summary loss, a simple one-step viability miss, or an unresolved
    local same-primary selector; the old step-`12` `33 / 5` drop is no longer
    the remaining local blocker on that repaired step-`11` history
- A new local step-`12` repair now refines that blocker further:
  - the guarded step-`12` telescope is still enumerated and exact-admitted on
    the repaired live claim history
  - it now also survives claim open-band semantic minimality and the
    remaining-one exact screen on that history
  - live claim step `12` now recovers guarded `nu = 34`, `kappa = 6`
    instead of the old `33 / 5` drop
  - exact multi-step partial-prefix caching now also keeps that guarded shell
    alive into the retained candidate pool
  - live claim step `12` now also accepts the guarded same-primary curvature
    shell instead of a richer local rival
  - every current same-primary tied step-`12` survivor still collapses onto
    one observed step-`13..15` path with late generated counts `123`, `12027`,
    and `780`
  - so the remaining local blocker is no longer a live step-`12`
    `nu / kappa`, retained-pool, or accepted-hash miss; it is the late
    generated-floor collapse at steps `13..15` on top of the repaired
    step-`9` / `11` / `12` chain
- A new scoped step-`13` widening now explains where that collapse still
  starts:
  - the repaired claim lane no longer stalls at a singleton-heavy
    `3`-telescope step-open:
    step `13` now widens to raw widths `[3,1,3,3,1,1,1]`, raw catalog `27`,
    `3` surviving roots, and live generated prefixes `123` while preserving
    the guarded accepted metric shell
  - that means the remaining step-`13` loss is now visibly downstream of
    catalog-open and no longer an exact-screen connectivity problem on the
    repaired branch:
    exact connectivity rejection, partial-prefix bar failure, and captured
    zero-admitted exact prunes are all now `0` there
  - a follow-up repaired-chain incumbent-relief pass now also clears the last
    live step-`13` terminal-rank pressure on that repaired `34 / 6` history:
    live terminal-rank prunes are now `0` there too while the guarded metric
    shell and the same observed `step-13 -> step-15` continuation stay fixed
  - step `14` is no longer thin on the repaired local chain:
    widening the claim `kappa = 9` catalog now yields `19683` raw telescopes,
    `3` surviving roots, and `12027` live generated prefixes before
    proof-close
  - step `15` is different:
    its raw claim catalog is still broad at `6561`, but exact
    partial-prefix bar failures dominate there (`512`) before proof-close
  - the captured zero-admitted exact-prune surface under that branch is now
    also localized more tightly:
    all `2184` captured families lose all `6552` generated terminal options to
    connectivity before admissibility, and the new connectivity-classification
    regression now proves those options are all structurally connected but
    still unqualified (`NeedsFallback`) with `0` historical reanchor hits, so
    the remaining local repair is qualifier / reanchor evidence on that
    connected terminal-clause surface rather than generic claim admissibility;
    the new reanchor-prefix regression now also proves those captured families
    already fall off the temporal-shell prefix by clause `5` or earlier, so
    the next repair is not just the last-clause slot in isolation; the new
    clause-`6` boundary regression now also proves that once those first six
    clauses are fixed, the captured exact-prune surface has already stopped, so
    the remaining repair should stay focused on clause `5` or earlier rather
    than clause `6`
- A follow-up exploratory global step-`13` widening now bounds the next repair
  more honestly:
  - naive band-`7` widening can lift the repaired step-`13` local read to raw
    `2187` / generated `615` with `3` seeded roots and no immediate root
    exact-screen rejection
  - but once that happens, most remaining loss shifts into exact
    legality/connectivity rejection, partial-prefix bar failure, and
    incumbent dominance instead of disappearing
  - and that same reland breaks unrelated claim prefix-memo,
    realistic-shadow, demo-lane, and divergent late-step guardrails, so the
    narrower next fix had to stay claim-only and repaired-history-specific;
    that scoped widening is now landed, so the remaining work is the residual
    exact-screen losses on that widened step-`13` surface
- The new stored full-profile bundle also changes what counts as the next
  honest engineering dollar:
  - do not reopen runtime-only step-`4` surgery first
  - do not reopen late-step zero-candidate diagnosis first
  - first localize the earliest parity fork and the late generated-floor
    collapse that remain on the completed run
- No residual `pen-cli.exe` process remains from the `v5` rerun.

## Forward Direction

- Freeze the current evidence set:
  `v1`, `v2`, `v3`, capped `v1`, stopped `v4`, and completed `v5`.
- Treat the pre-flight gate, the completed full-profile rerun, and the stored
  compare / certification / benchmark outputs as earned.
- Keep the new stored `v5` compare / certification / benchmark regressions
  green as the local parity-plus-breadth guardrail.
- Keep the claim-policy metadata, narrative/event artifacts, exact-screen
  reason counts, prune-class counts, and manifest provenance green.
- Prioritize targeted local diagnosis and repair for:
  - the step-`9` accepted-hash fork
  - the landed scoped claim-only step-`13` widening plus follow-up
    structural-connectivity plus repaired-chain incumbent-relief repairs:
    claim-open widths `[3,1,3,3,1,1,1]`, raw catalog `27`, live generated
    prefixes `123`, guarded accepted metric shell preserved, and the same
    observed `step-13 -> step-15` continuation preserved
  - treating step `13` as a repaired local guardrail rather than the next
    live blocker:
    exact connectivity rejections, partial-prefix bar failures, captured
    zero-admitted exact prunes, and live terminal-rank prunes are now all `0`
    on the repaired `34 / 6` chain, while the divergent step-`13` guardrail
    remains intact
  - keeping the widened step-`14` catalog plus same-primary continuation
    selector stable until a stored rerun consumes it
  - the step-`15` canonical temporal-shell exact-prune surface, which now
    freezes as `2184` captured zero-admitted families with no cached compact
    bounds, with all `6552` generated terminal options on that captured
    surface now frozen as structurally connected but still unqualified
    `NeedsFallback` candidates with `0` historical reanchor hits while the raw
    `6561`-telescope catalog still narrows to `780` generated prefixes before
    proof-close; those captured families now also fall off the historical
    reanchor prefix by clause `5` or earlier, and once those first six clauses
    are fixed the claim lane now keeps exactly `3` clause-`6` variants outside
    captured exact prune with only the exact reference continuation exposing a
    `KeepWithoutFallback` terminal path; the new isolated-prefix regression now
    also proves each of the `12` single early claim-only deviations on the
    otherwise exact seven-clause prefix still leaves all `3` terminal
    continuations at `NeedsFallback` and zero-admitted, and the follow-up
    variant-product regression now proves each of those `12` local variants
    stays captured across its full later claim suffix fan-out too; the new
    forced-reanchor recovery regression now also proves each such isolated
    local variant would recover all `3` terminal continuations as
    `KeepWithoutFallback`, admitted, and bar-clearing on the otherwise exact
    suffix, so the next repair should inspect clause-local earlier
    temporal-shell qualifier evidence through clause `5` or earlier rather
    than only the clause-`6` / terminal slot, downstream admissibility / bar
    arithmetic, or a multi-early-deviation interaction
  - avoiding a direct temporal-reanchor matcher reland first:
    both the broad `60 / 8 / 9840` branch and the narrower `89 / 8 / 780`
    branch disturbed the canonical step-`15` continuation and were reverted
  - avoiding a direct early clause-`2` / clause-`3` reanchor-bridge matcher
    reland first:
    the clause-`3`-only branch displaced the canonical continuation to
    `88 / 8 / 795`, and the clause-`2` plus clause-`3` branch displaced it to
    `74 / 8 / 828`, so both were reverted as well
- Launch the next clean-start full-profile rerun only after the local repair
  is green against those stored parity/floor regressions.

## Immediate Next Move

1. Keep the new step-`11` retained-pool regression green:
   the guarded completion must stay visible through terminal-clause
   generation, compact summary, survivor sketch, and retained-pool
   preservation on the live claim history.
2. Keep the new step-`11` selector regression green:
   live claim acceptance must keep preferring the guarded same-primary
   survivor over richer raw structural winners on the live divergent history.
3. Keep the new step-`11` continuation-collapse regression green:
   all current same-primary tied step-`11` survivors still keep step `12`
   alive and now collapse onto one observed step-`12` `34 / 6`
   continuation.
4. Keep the new step-`12` exact-screen / minimality regression green:
   the guarded curvature shell must keep surviving preterminal clause
   exposure, full summary, compact survivor sketch, semantic minimality, and
   exact remaining-one screening on the repaired live history.
5. Keep the new step-`12` retained-pool regression green:
   the guarded curvature shell must stay present in the retained candidate
   pool once the cross-band partial-prefix cache is keyed by `clause_kappa`.
6. Keep the new step-`12` cache-key regression green:
   a modal `5`-clause `CannotClearBar` result must not poison the guarded
   `6`-clause continuation on the same preterminal prefix.
7. Keep the new step-`12` selector regression green:
   live claim acceptance must keep preferring the guarded same-primary
   curvature shell over richer raw structural winners inside the repaired
   `34 / 6` tier.
8. Keep the new step-`12` continuation-collapse regression green:
   every current same-primary tied step-`12` survivor must still collapse onto
   one observed step-`13..15` continuation with generated counts
   `123`, `12027`, and `780`.
9. Keep the new widened step-`14` regression green:
   the repaired step-`12` chain must keep reporting a step-`14` raw catalog
   of `19683`, `3` surviving roots, and `12027` live generated prefixes
   before proof-close on the widened claim `kappa = 9` surface.
10. Keep the new step-`14` same-primary continuation selector regression green:
    live claim acceptance must keep choosing the one same-primary
    `62 / 9` survivor that restores the canonical step-`15`
    `DCT 103 / 8` continuation while the broader `78 / 9 / 12027`
    branch remains the alternate local path.
11. Do not reland a global band-`7` widening directly:
    the exploratory branch widened repaired step `13` to raw `2187` /
    generated `615`, but it also disturbed claim prefix-memo,
    realistic-shadow, demo-lane, and divergent late-step guardrails.
12. Keep the new scoped claim-only step-`13` widening plus structural-
    connectivity plus repaired-chain incumbent-relief regressions green:
    the repaired step-`12` chain must keep reporting claim-open widths
    `[3,1,3,3,1,1,1]`, raw catalog `27`, live generated prefixes `123`, and
    the guarded accepted metric shell at step `13`.
13. Keep the new repaired step-`13` exact-screen-clear regression green:
    the repaired widened surface must keep reporting `0` connectivity prunes,
    `0` terminal-clause-filter prunes, `0` partial-prefix bar prunes,
    `0` captured zero-admitted exact prunes, and `0` live terminal-rank
    prunes on the surviving repaired `34 / 6` surface.
14. Treat step `13` as a repaired guardrail rather than the next late blocker:
    do not reopen structural connectivity, terminal filtering,
    incumbent-dominance relief, or another blind catalog widening target
    first while step `15` remains open.
15. Treat step `15` as the separate remaining exact-screen problem:
    inspect why the restored canonical temporal-shell path still collapses
    into `2184` captured zero-admitted exact-prune families and `512`
    partial-prefix bar failures before proof-close; the new regression shows
    those captured families are now a pure qualifier / reanchor miss on an
    already connected surface (`6552 / 6552` `NeedsFallback`, `0` structural
    disconnections, `0` historical reanchor hits, `0` admissibility
    rejections), and the follow-up reanchor-prefix regression now proves those
    captured families already fall off the temporal-shell prefix by clause `5`
    or earlier with no full seven-clause prefix matches; the isolated-prefix
    regression now also proves every single early claim-only deviation at
    clause positions `0..5` already keeps all `3` terminal continuations at
    `NeedsFallback` and zero-admitted on the otherwise exact suffix, and the
    follow-up variant-product regression now proves each such local variant
    stays blocked across every later claim suffix combination before the
    clause-`6` boundary, while the new forced-reanchor recovery regression
    proves each isolated local variant would recover all `3` terminal
    continuations as `KeepWithoutFallback`, admitted, and bar-clearing on the
    otherwise exact suffix, so do not treat that remaining loss as generic bar
    bookkeeping, raw dependency-edge generation, a claim admissibility miss, a
    terminal-clause-only matcher bug, a multi-deviation-only interaction, a
    later-suffix interaction, or a direct early clause-`2` / clause-`3`
    bridge-matcher expansion.
16. Do not reland either exploratory temporal-reanchor matcher variant
    directly:
    the broad reland displaced the canonical step-`15` continuation to
    `60 / 8 / 9840`, and the narrower late-shell-only reland still displaced
    it to `89 / 8 / 780`.
17. Do not reland the exploratory early clause-`2` / clause-`3`
    reanchor-bridge matcher variants directly:
    the clause-`3`-only branch displaced the canonical step-`15`
    continuation to `88 / 8 / 795`, and the clause-`2` plus clause-`3`
    branch displaced it to `74 / 8 / 828`.
18. Re-run the step-`9` tie diagnostic only after the step-`13` breadth and
    step-`15` exact-screen stories are better understood, since the current
    tied step-`9` set still shares the same observed repaired
    step-`10..12` chain.
19. Only then land the narrowest honest parity/floor fix, and only then launch
    `long-rerun-v6` and re-run compare, certification, and benchmark against
    the repaired bundle.
