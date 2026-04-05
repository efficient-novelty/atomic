# Autonomous Claim Lane Progress

Last updated: 2026-04-05

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
  `629b59861d521555cfd214a602bedec4f44c67d0` with release binary hash
  `bdcbf9b0c6386a5fd2e0e7388c2637a15486c962f47041530075dff3e50a5385`:
  - targeted `desktop_claim_shadow` regressions green
  - claim live-checkpoint persistence green
  - release replay harness benchmark replayed all `5` stored
    `remaining_one_plateau` surfaces without mismatch
- The current repaired-chain local guardrails were rerun on clean-tree repo
  head `67c26eca02cb5546745bdd5ca5b31468e6807f42`:
  - guarded step-`11` / step-`12` / step-`15` claim regressions green
  - repaired step-`12` same-primary selector regression green
  - repaired step-`12 -> 15` continuation guardrails green
- The older completed `v5` bundle remains frozen as the pre-parity completed
  evidence surface.
- The earlier clean-tree completed `v6` bundle remains frozen as the
  pre-step-`11`-breadth stored baseline.
- The dirty-tree `v7` and `v8` reruns remain transient selector diagnostics:
  - `v7` first consumed the step-`11` breadth repair but reopened
    accepted-hash parity at step `12`
  - `v8` restored accepted-hash parity and the stored step-`11` floor, but it
    was recorded with `dirty_tree = true`
- The current authoritative stored full-profile bundle is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v11`
  on clean-tree repo head `4e38f2463b429bcbebe16cc6d7c5eac7ef2de050` with
  release binary hash
  `64535f9e6c2e2a77c1bdeeb1f848abbeb0312f9b454bce42f27c68b3b852271c`.
- Its authoritative `run.json` state is:
  - `status = "completed"`
  - `completed_step = 15`
  - `active_step = 16`
  - `active_band = 8`
  - `frontier_epoch = 12`
  - `resolved_worker_count = 8`
  - `dirty_tree = false`
- The new `v11` bundle now stores the full claim artifact surface through
  step `15`:
  - run manifest with complete host/build/git/binary provenance
  - step summaries through `step-15-summary.json`
  - live-checkpoint streams through `step-15-live.ndjson`
  - per-step narratives and event streams through step `15`
  - frontier and step checkpoints for the completed run
- Stored audit outputs now also exist under that `v11` run directory:
  - `claim-compare.txt` / `claim-compare.json`
  - `claim_certificate.txt` / `claim_certificate.json`
  - `claim_benchmark.txt` / `claim_benchmark.json`
  - the certificate now also records per-step breadth diagnosis from stored
    step summaries plus `step-XX-live.ndjson` checkpoints, so the open
    step-`1` / step-`15` miss anatomy is queryable without another rerun
  - that stored diagnosis now preserves the full claim-step-open pressure
    signature for failing steps too, including active widening bands,
    package flags, and claim-debt `path` / `trunc` pressure alongside the
    existing catalog, root-seeding, and exact-screen read
- Those audits use the guarded baseline
  `runs/codex-guarded-claim-cert-v1`.
- The clean-tree stored `v11` compare audit is ready:
  - trajectory matches guarded through step `15`
  - accepted hashes match guarded through step `15`
  - replay ablation now matches reference replay on `01..15`
  - search-space counts still diverge at step `4`, step `9`, step `10`,
    step `11`, step `12`, step `13`, step `14`, and step `15`
  - admissibility diagnostics still diverge at step `9..15`
  - late-step competition still diverges at step `10..15`
- The clean-tree stored `v11` certification result is honest and narrower:
  - status `= "attention"`
  - failing checks:
    - `early_breadth`
    - `late_generated_floors`
  - passing checks:
    - `accepted_hash_parity`
    - `search_policy`
    - `fallback_honesty`
    - `narrative_artifacts`
    - `runtime_threshold`
    - `exact_screen_reason_completeness`
    - `prune_class_completeness`
    - `manifest_completeness`
  - the stored certificate now makes the current miss split explicit:
    - step `1` still opens catalog `648` with widths `[18, 36]`, roots
      `18` seen / `16` enqueued, and `435`
      legality/connectivity exact rejections before proof-close
    - step `15` still opens `6561`, seeds `3` roots, and then loses stored
      breadth under `468` partial-prefix bar failures plus
      `242` incumbent-dominance prunes, with `0`
      legality/connectivity exact rejections, and now prints the
      `claim_generic` `kappa 8..8` temporal-shell opening with modal anchor
      ref `10` and active widening bands `7,8,9`
- New stored-evidence regressions now freeze that canonical `v11`
  diagnosis in-tree:
  - `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`
    now pins the stored step-`1` miss, the full late-floor table on
    `10..15`, the fact that only step `15` still misses there, and the
    frozen `step-15-live.ndjson` provenance note sequence
    `claim_regular_clause_catalog -> claim_root_seeding_summary`
  - `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts`
    now pins the refreshed single-run `3965 ms` / parity-`1` / early-`0` /
    late-`0` benchmark bundle
- A new local step-`15` survivor-bucket regression now freezes the
  exact-screened survivor split more tightly on the repaired canonical chain:
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
  freezes that remaining exact-screened pressure more tightly too:
  - `current_claim_step_fifteen_small_cluster_incumbent_surface_stays_same_primary_and_non_winning`
    now pins that all `242` remaining `small_cluster` prunes happen during
    summary-stage exact screening rather than later proof-close materialization
  - every one of those remaining pruned candidates still sits on the same
    primary `103 / 8` tier with overshoot `115657 / 21112`
  - the remaining non-winning spread there is now only secondary bit cost,
    from `236` through `290`, above the canonical winner's `229`
  - the earliest first-mismatch positions across that still-pruned surface are
    now frozen at clause positions `0`, `1`, `2`, `4`, and `5` with counts
    `162`, `54`, `18`, `6`, and `2`
- A new local step-`15` omitted-side-variant regression now sharpens the next
  safe opening on that same repaired anchor-`11` pocket without relanding any
  new widening yet:
  - `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_stay_same_primary_and_non_winning`
    now pins that the omitted demo-only temporal-shell side variants at clause
    positions `0`, `1`, `4`, and `5` all stay structurally connected, locally
    admissible, and same-primary `103 / 8` / `115657 / 21112` on top of the
    current live claim clause-`2` + anchor-`11` exact-argument pocket
  - those omitted side variants still stay outside historical reanchor there
    and still lose only on higher bit cost `243`, `245`, or `250`, so they are
    candidate-safe surface openings rather than hidden stronger winners
  - a non-landed raw position-`0` reland probe briefly lifted the local
    step-`15` generated surface from `3972` to `4285`, but it also reopened the
    old clause-`0` zero-admitted capture to `2835` families and restored
    `891` clause-`0` historical-reanchor misses, so that broad catalog reland
    was reverted
  - the next repair should therefore isolate one of those omitted side
    openings onto the existing anchor-`11` `small_cluster` path instead of
    broadening the raw position-`0` temporal-shell catalog first
- The remaining stored breadth snapshot on the parity-clean bundle is:
  - step `1`: `546` versus target `2144` (`miss`)
  - step `10`: `1428` versus target `500` (`hit`)
  - step `11`: `1338` versus target `800` (`hit`)
  - step `12`: `1338` versus target `1200` (`hit`)
  - step `13`: `2320` versus target `2200` (`hit`)
  - step `14`: `12027` versus target `3500` (`hit`)
  - step `15`: `3972` versus target `5000` (`miss`)
- The new benchmark bundle proves runtime and stored parity are now earned on
  the current bundle:
  - claim run count `= 1`
  - completed step-`15` count `= 1`
  - runtime `= 3965 ms`
  - parity success count `= 1`
  - full early breadth hit count `= 0`
  - full late floor hit count `= 0`
- The current repo head `4e38f2463b429bcbebe16cc6d7c5eac7ef2de050`
  now stores the canonical rerun and refreshed audit bundle that frame the
  remaining breadth repair:
  - the step-`9` / step-`11` / step-`12` claim selector regressions are green
  - the repaired step-`12` same-primary selector regression is green
  - the current step-`13` / step-`14` / step-`15` guardrails are green
  - claim live-checkpoint persistence is green
  - the new stored `v11` certificate / benchmark freeze regressions are green
  - the stored `v5` compare / certification / benchmark freeze regressions
    remain green
- The current local step-`15` diagnostic suite was rerun green after adding
  that survivor-bucket and small-cluster incumbent freezes:
  - all current `current_claim_step_fifteen_*` engine regressions are green
  - `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`
    is green
  - `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts` is green
- The parity-preserving step-`13` breadth repair is now earned on stored
  evidence:
  - on the canonical repaired chain, claim-open now widens to
    `[5,1,3,3,5,3,2]` / raw `1350` / generated `2320`
  - the guarded step-`13` metric shell now stays accepted on that stored
    surface, with the same observed canonical continuation
    `46 / 7 / 2320 -> 62 / 9 / 12027 -> 103 / 8 / 3972`
  - step-`13` root seeding now rises to `5` seen / `5` enqueued on the
    stored canonical branch
  - the stored step-`13` surface now records `576`
    legality/connectivity exact rejections before proof-close, alongside
    `774` well-formed candidates, `8` exact-screened / retained candidates,
    and `401` heuristic drops
  - the two frozen executable negative controls remain unchanged:
    `[3,5,3,3,5,1,1]` still reopens local breadth only on the unsafe
    `45 / 7 -> 61 / 9` path, and `[5,1,3,3,5,3,3]` still preserves guarded
    step-`14` / step-`15` hashes while keeping step-`13` parity open
- A new narrow step-`9` same-primary selector repair now lands on top of that
  stored `v5` evidence surface:
  - the guarded step-`9` telescope still stays retained inside the local
    same-primary `17 / 4` tie set on the guarded step-`8` prefix
  - exactly one tied step-`9` survivor now matches the exact historical-anchor
    axiomatic bundle under claim admissibility on that prefix
  - live claim step-`9` acceptance now prefers that guarded exact-bundle
    survivor over the cheaper same-primary structural winner, while the raw
    structural selector still picks the old cheaper `70`-bit variant
  - the new step-`9` selector regression, the downstream step-`11` /
    step-`12` / step-`13` / step-`14` / step-`15` claim guardrails, the late
    divergent step-`13` / step-`14` guardrails, and the stored `v5` compare
    regression were all rerun green after this repair
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
- A new follow-up step-`15` forced-reanchor winner regression now sharpens
  that same blocker one layer further:
  - on that same otherwise exact suffix, forcing clause-local reanchor on each
    isolated early claim-only temporal-shell deviation still never restores the
    canonical reference terminal clause
  - those `12` local forced-recovery surfaces now split `6 / 6` between the
    two non-reference claim terminal closures, with `0` reference-terminal
    winners
  - the position-local best-rank profiles are now fixed at `89 / 8` for
    positions `0`, `1`, `4`, and `5`, `75 / 8` for position `2`, and
    `74 / 8` for position `3`
  - the next step-`15` repair therefore cannot be a simple historical-reanchor
    bool flip on those local variants; it must restore clause-local qualifier
    evidence while still preserving the canonical terminal continuation
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
  - both variants were reverted, and the then-current step-`15`
    `103 / 8 / 780` baseline plus the exact-prune / connectivity /
    reanchor-prefix regressions were revalidated afterwards
  - the next step-`15` repair should therefore not directly broaden the early
    clause-`2` / clause-`3` reanchor bridge matcher either
- A new narrow step-`15` clause-`0` / clause-`1` historical-reanchor repair
  now lands:
  - claim-only temporal-shell variants at clause positions `0` and `1` now
    count as historical reanchor evidence only when the terminal clause stays
    on the exact reference continuation
  - lifted non-reference terminal closures on those repaired prefixes still
    stay outside historical reanchor, so the broader unsafe terminal reland
    remains out
  - the canonical step-`15` continuation stays at `DCT 103 / 8` while the
    live generated surface lifts from `780` to `930`
  - the repaired canonical branch now pays `540` partial-prefix bar failures
    plus `8` incumbent-dominance prunes, with a widened surviving temporal
    terminal cluster of `84` generated / `10` admitted / `10` exact-screened /
    `8` pruned
  - the captured exact-prune surface shrinks from `2184` to `2160` families,
    the connected `NeedsFallback` surface shrinks from `6552` to `6480`
    terminal options, and the remaining historical-reanchor mismatch now
    localizes to clause positions `2..5` with counts `1458`, `486`, `162`,
    and `54`
  - clause positions `0` and `1` are no longer present on the captured
    isolated-prefix surface, so the remaining local repair is now clause-
    local qualifier evidence at positions `2..5` while preserving the exact
    reference terminal continuation
- A new local exact-terminal-only isolated-recovery regression now sharpens
  that same step-`15` blocker further:
  - on the otherwise exact suffix, granting isolated exact-terminal recovery
    to the remaining clause-`2..5` temporal deviations would still create
    stronger-than-canonical local recovered profiles only at clause positions
    `2` and `3`
  - those unsafe exact-terminal recovered profiles are now fixed at
    `89 / 8` for clause `2` and `88 / 8` for clause `3`
  - the same exact-terminal-only recovery now stays on canonical-primary
    `103 / 8` profiles at clause positions `4` and `5`
  - that exact-terminal-only branch was not landed, and the canonical
    `DCT 103 / 8 / 930` continuation plus the current step-`15`
    exact-prune / connectivity / isolated-variant guardrails were
    revalidated afterwards
- A new narrow step-`15` clause-`4` / clause-`5` historical-reanchor repair
  now lands:
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
  - clause positions `0`, `1`, `4`, and `5` are no longer present on the
    captured isolated-prefix surface, clause `6` still stays downstream of the
    capture boundary, and the remaining local repair is now clause-local
    qualifier / reanchor evidence at positions `2..3` while preserving the
    exact reference terminal continuation
  - the forced-reanchor recovery and winner reads now apply only to clause
    positions `2` and `3`: each remaining isolated local variant would still
    recover all `3` terminal continuations as `KeepWithoutFallback`, admitted,
    and bar-clearing on the otherwise exact suffix if clause-local qualifier
    evidence were restored, but a direct local reanchor flip still never
    restores the canonical reference terminal clause and instead stays on the
    noncanonical `75 / 8` and `74 / 8` winner profiles
  - the earlier exact-terminal-only regression now also sharpens the new open
    read honestly: clause-`4` / clause-`5` are no longer present on the
    captured isolated surface, while clause-`2` / clause-`3` still reopen the
    stronger local `89 / 8` and `88 / 8` rivals if isolated recovery is
    granted
- A new local clause-`2` / clause-`3` pair-surface regression now sharpens
  that same step-`15` blocker further:
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
  - the next step-`15` repair should therefore stay off any direct
    clause-`2` / clause-`3` matcher or paired reanchor reland and instead
    target narrower qualifier evidence on the canonical branch without
    reopening those stronger local rivals
- A new nearby clause-`2` / clause-`3` temporal-replacement regression now
  sharpens that same step-`15` blocker further:
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
  sharpens that same step-`15` blocker further:
  - pairing any nearby claim-or-demo temporal replacement at clause positions
    `2` and `3` on the otherwise exact suffix still leaves all `16`
    non-reference pairs structurally connected, locally admissible, and
    outside historical reanchor
  - every such exact-terminal-only paired recovery now collapses onto the same
    unsafe `74 / 8` profile
  - broadening that local replacement neighborhood therefore still does not
    supply safe qualifier evidence on the canonical branch
  - a new mixed clause-`2` / clause-`3` reference-terminal context regression
    now narrows that same step-`15` blocker further:
    every mixed claim-only clause-`2` / clause-`3` surface that restores the
    exact reference terminal under forced reanchor still does so only when
    clause `6` also deviates, and those unsafe `60 / 8` reference-terminal
    wins already span every repaired-side subset of clause positions `0`,
    `1`, `4`, and `5` on top of that clause-`6` deviation, including the
    minimal clause-`6`-only context, so the remaining open work is not hiding
    behind repaired-side exactness or a clause-`6`-mediated reland either
- A new isolated clause-`2` / clause-`3` context regression now narrows that
  same step-`15` blocker further:
  every remaining isolated clause-`2` or clause-`3` claim-only surface now
  already spans all `32` repaired-side / clause-`6` context masks on the
  captured step-`15` surface, yet forced clause-local reanchor still never
  restores the exact reference terminal anywhere on that isolated context set;
  exact-terminal-only isolated recovery also still collapses only onto unsafe
  clause-`3` profiles `88 / 8` with exact clause `6` and `74 / 8` with
  claim-only clause `6`, plus unsafe clause-`2` profiles `89 / 8` with exact
  clause `6` and `75 / 8` with claim-only clause `6`, so no safe isolated
  clause-`2` / clause-`3` pocket is hiding behind repaired-side exactness or
  clause-`6` context either
- A new exact-suffix clause-`2` / clause-`3` catalog regression now narrows
  that same step-`15` blocker further:
  on the otherwise exact suffix, the live step-`15` claim catalog at clause
  position `2` still exposes only the two known non-reference claim variants,
  and both stay structurally connected, locally admissible, outside
  historical reanchor, and pinned to the unsafe `89 / 8` exact-terminal
  profile; clause position `3` likewise still exposes only the two known
  non-reference claim variants, and both stay structurally connected, locally
  admissible, outside historical reanchor, and pinned to the unsafe
  `88 / 8` exact-terminal profile, so the remaining open work is not hiding
  behind an undiscovered exact-suffix clause-catalog branch either
- A new nearby clause-`3` anchor-swap regression now narrows that same
  step-`15` blocker further:
  on the otherwise exact suffix, swapping clause `3` from the exact
  historical anchor `10` to nearby library refs `9` or `11` still leaves
  every tested clause structurally connected, locally admissible, outside
  historical reanchor, and first mismatching the temporal-shell prefix
  exactly at clause `3`; the nearby anchor-`9` exact-argument swap still
  reopens an unsafe stronger-than-canonical `65 / 8` profile while the
  lifted anchor-`9` neighbors stay below bar, and the nearby anchor-`11`
  exact-argument swap reaches only a noncanonical `117 / 8` bar-clearer
  without restoring qualifier evidence while the lifted anchor-`11`
  neighbors all reopen unsafe stronger-than-canonical `102 / 8` profiles,
  so the remaining open work is not a simple nearby clause-`3` anchor
  retarget either
- A new mixed clause-`2` replacement plus nearby clause-`3` anchor-swap
  regression now narrows that same step-`15` blocker further again:
  pairing any current clause-`2` claim/demo replacement with the nearby
  clause-`3` anchor-`9` neighborhood still leaves all `5` tested exact-suffix
  variants structurally connected, locally admissible, outside historical
  reanchor, and below bar; pairing those same clause-`2` replacements with
  the nearby clause-`3` anchor-`11` exact-argument swap now reaches only a
  canonical-primary-but-still-unqualified `103 / 8` bar-clearer, while the
  lifted anchor-`11` neighbors on those same clause-`2` replacement contexts
  still reopen the unsafe stronger-than-canonical `88 / 8` profile, so the
  remaining open work is not a simple nearby clause-`3` anchor retarget or a
  direct clause-`2` replacement reland either; if that anchor-`11`
  neighborhood matters, the next repair must isolate the exact-argument
  qualifier evidence without relanding the lifted anchor-`11` variants
- A new anchor-`11` exact-argument rank regression now narrows that same
  step-`15` blocker further again:
  on every current mixed clause-`2` claim/demo replacement, the nearby
  clause-`3` anchor-`11` exact-argument pocket now freezes as a full-rank
  `103 / 8` bar-clearer that matches the canonical `DCT 103 / 8` winner on
  overshoot, clause `kappa`, eliminator/former/density/library/binder/
  closure signals, max var reach, and `nu`, but it still loses cleanly on
  higher bit cost `236` versus canonical `229`; the lifted anchor-`11`
  neighbors still reopen the unsafe `88 / 8` rival, so any future repair
  that touches that neighborhood must isolate the exact-argument qualifier
  evidence without relanding the lifted variants or replacing the canonical
  accepted path
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

- The clean `v10` rerun is the current canonical stored claim bundle:
  `desktop_claim_shadow` now has one newer clean-start full-profile bundle
  from the disclosed desktop that both finishes through step `15` and
  preserves accepted-hash parity through step `15`.
- Its terminal summary still shows the accepted executable canon at step `15`
  / `DCT` with `nu = 103`, `kappa = 8`.
- The old `v3` step-`14` zero-candidate opening, the old stored
  step-`9` / step-`11` / step-`12` parity fork, and the old stored
  step-`13` floor miss are no longer live stored blockers:
  - `v10` reaches step `14`, seeds roots, accepts a survivor, and advances to
    step `15`
  - compare now reports trajectory and accepted hashes matching guarded
    through step `15`
  - replay ablation now records `matches_reference_replay` on every step
    `01..15`
- The transient stored reruns between `v6` and `v10` also sharpen what changed:
  - dirty-tree `v7` first re-earned stored step `11` but diverged at accepted
    hash parity on step `12`
  - dirty-tree `v8` restored parity and the stored step-`11` floor, but it
    stayed outside clean-tree certification because `dirty_tree = true`
  - clean-tree `v9` first restored stored parity through step `15`
  - clean-tree `v10` then re-earns stored step `13` while preserving that
    same parity-clean later continuation
- The new compare audit stays explicit about what still differs while
  remaining signoff-ready:
  - search-space counts diverge at step `4`, step `9`, step `10`,
    step `11`, step `12`, step `13`, step `14`, and step `15`
  - admissibility diagnostics diverge at step `9..15`
  - late-step competition diverges at step `10..15`
- The certification audit is now explicit and narrower:
  - status `= "attention"`
  - failing checks:
    - `early_breadth`
    - `late_generated_floors`
  - passing checks:
    - `accepted_hash_parity`
    - `search_policy`
    - `fallback_honesty`
    - `narrative_artifacts`
    - `runtime_threshold`
    - `exact_screen_reason_completeness`
    - `prune_class_completeness`
    - `manifest_completeness`
- The new completed run proves the claim artifacts are now strong enough for
  parity-safe auditing on stored evidence:
  - claim policy metadata is honest
  - fallback honesty is clear
  - narrative/event artifacts are complete
  - exact-screen reason counts are complete through step `15`
  - prune class counts are complete through step `15`
  - manifest provenance is complete on the stored release build
- Runtime is no longer the live blocker on the stored full-profile slice:
  - the benchmark bundle records one completed step-`15` claim run
  - stored runtime for `v10` is `3849 ms`
  - parity success count is now `1`
  - the audit passes the provisional `600000 ms` runtime threshold
- The remaining breadth/floor failures are now concrete on the parity-clean
  stored bundle:
  - early breadth:
    - step `1` generated `546`, target `2144`
  - late generated floors:
    - step `10`: `1428` against target `500` (`hit`)
    - step `11`: `1338` against target `800` (`hit`)
    - step `12`: `1338` against target `1200` (`hit`)
    - step `13`: `2320` against target `2200` (`hit`)
    - step `14`: `12027` against target `3500` (`hit`)
    - step `15`: `1794` against target `5000` (`miss`)

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
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v10`
- Launch surface:
  - clean-tree repo head
    `6939b71063e4aec9598b293560c0c233981bc169`
  - release binary hash
    `d003dd29c599ba86b7ef410b1c7849f89a0fe45d33dc42508e368f4c2a7c473a`
- Full-profile outcome:
  - `status = "completed"`
  - `completed_step = 15`
  - `active_step = 16`
  - step `15` accepted `DCT` with `nu = 103`, `kappa = 8`
  - `replay_ablation = matches_reference_replay x15`
- Stored audits under the same run directory:
  - compare report against `runs/codex-guarded-claim-cert-v1` (`ready`)
  - claim certificate (`attention`: `early_breadth`, `late_generated_floors`)
  - single-run benchmark bundle (`3849 ms`, parity `1`, early `0`, late `0`)

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
  the clean-tree stored `v11` run completes the full claim profile in one
  clean-start pass.
- The old `v3` step-`14` zero-candidate opening is also no longer the first
  blocker:
  `v11` reaches step `14`, seeds roots, accepts a survivor there, and advances
  to step `15`.
- Stored ordinal parity is now re-earned on the clean canonical bundle after
  the transient `v7` step-`12` fork and the dirty-tree `v8` confirmation pass:
  - compare signoff is `ready`
  - accepted hashes match guarded through step `15`
  - replay ablation now matches reference replay on every step `01..15`
  - the claim audit is `ready`, and claim-policy / fallback / narrative /
    exact-screen / prune / manifest coverage remain complete
- The current blocker is now stored breadth on the canonical chain:
  - early breadth still misses at step `1` (`546` versus `2144`)
  - late generated floors now hit at steps `10`, `11`, `12`, `13`, and `14`
  - late generated floors still miss only at step `15` with stored count
    `3972`
  - the stored certificate now exposes the miss anatomy directly:
    step `15` now loses the remaining breadth later under `468`
    partial-prefix bar failures and `242` incumbent-dominance prunes after
    connectivity exact rejection has already fallen to `0`
- Search-space, admissibility, and late-step competition deltas remain honest
  comparison-backed differences, but they are no longer parity blockers as
  long as accepted hashes stay aligned.
- A new local step-`9` diagnostic plus selector repair now closes the earliest
  local fork:
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
  - the tied step-`9` candidates still collapse onto the same observed
    repaired step-`10..12` chain, so the final local fix stays narrowly on
    step-`9` same-primary final selection rather than on generation or later
    viability
  - exactly one tied step-`9` survivor matches the guarded historical-anchor
    axiomatic bundle, and live claim step `9` now accepts that survivor
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
- A new follow-up local step-`11` breadth repair now sharpens the earliest
  late-floor miss further:
  - claim structural-shell seal variants now count as connected when the
    adjacent step-`11` `kappa = 6` closure clause stays on the existing
    library-backed claim surface
  - the direct connected claim step-`11` surface now widens from
    `kappa 5 = 243`, `kappa 6 = 243` (total `486`) to
    `kappa 5 = 243`, `kappa 6 = 729` (total `972`) without changing the
    guarded accepted step-`11` shell
  - live claim search now also keeps that existing step-`11` `kappa = 6`
    surface out of exact frontier screening, so the local generated-floor
    regression is now green while the guarded step-`11` winner and the
    repaired step-`12` `34 / 6` continuation stay fixed
  - local step-`11` exact-screen connectivity rejections are now `0`
  - dirty-tree `v7` first consumed that repair on stored evidence but reopened
    accepted-hash parity at step `12`
  - the narrow step-`12` same-primary selector repair now lands on top of that
    rerun read
  - clean-tree `v9` now restores accepted-hash parity through step `15` and
    re-earns stored step `11 = 1338 / 800` while keeping guarded
    step `12 = 1338 / 1200`
- The `skills/pen-atomic` claim-lane guidance is now synced to that same
  stored-`v9` / step-`13` posture, so future sessions should start from the
  clean canonical rerun and the remaining stored step-`13` / step-`15`
  breadth work instead of the older step-`11` rerun plan.
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
    and `1794`
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
    its raw claim catalog is still broad at `6561`, but the repaired live
    canonical branch now narrows to `1794` generated prefixes before
    proof-close on the same `DCT 103 / 8` continuation after paying `468`
    exact partial-prefix bar failures plus `80` incumbent-dominance prunes
  - the captured zero-admitted exact-prune surface under that branch is now
    localized more tightly after the narrow clause-`0` / clause-`1` and
    clause-`4` / clause-`5` historical-reanchor repairs:
    it now consists of `1944` captured families with no cached compact bounds
    while all `5832` generated terminal options on that captured surface stay
    structurally connected but still unqualified (`NeedsFallback`) with `0`
    historical reanchor hits
  - the surviving temporal terminal cluster has widened to `732` generated /
    `82` admitted / `82` exact-screened / `80` pruned, and the remaining
    captured families now first fall off the historical reanchor prefix only
    at clause positions `2..3` with counts `1458` and `486`
  - clause positions `0`, `1`, `4`, and `5` are now repaired out of the captured
    isolated-prefix surface, clause `6` still stays downstream of the capture
    boundary, and the remaining local repair is therefore clause-local
    qualifier / reanchor evidence at positions `2..3` while preserving the
    exact reference terminal continuation
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
- Two newer non-landed local step-`13` breadth probes now sharpen the parity
  side of that same blocker further:
  - widening only the repaired operator-bundle formation positions `1` and `4`
    to demo-like variants lifts the local read to
    `[3,5,3,3,5,1,1]` / raw `675` / generated `2223`, but it also shifts the
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
  - the next honest local repair is therefore a parity-preserving clause-local
    step-`13` widening/retention change, not a raw reland of either new
    exploratory widened surface
- The clean canonical `v10` bundle also changes what counts as the next honest
  engineering dollar:
  - do not reopen runtime-only step-`4` surgery first
  - do not reopen late-step zero-candidate diagnosis first
  - do not reopen another step-`13` theory slice first
  - do not reopen another rerun first; start from stored step `15` as the
    remaining late-floor miss on the clean canonical bundle
- No residual `pen-cli.exe` process remains from the `v11` rerun.

## Forward Direction

- Freeze the current evidence set:
  `v1`, `v2`, `v3`, capped `v1`, stopped `v4`, completed `v5`, clean
  baseline `v6`, transient diagnostics `v7` / `v8`, clean canonical `v9`,
  clean canonical `v10`, and clean canonical `v11`.
- Treat the pre-flight gate, the clean canonical full-profile rerun, the
  stored `v11` compare readiness, and the `v11` claim audit readiness as
  earned.
- Treat the new local step-`9` selector regression and its downstream
  step-`11` / step-`12` / step-`13` / step-`14` / step-`15` guardrail reruns
  as the current local proof that parity is repaired and must now stay closed
  while breadth is fixed.
- Keep the stored `v5` audit-freeze regressions green as the pre-parity
  comparison surface, keep stored `v6` frozen as the pre-step-`11` breadth
  baseline, keep stored `v10` frozen as the pre-anchor-`11` stored baseline,
  and keep the stored `v11` compare / certification /
  benchmark outputs green as the current breadth guardrail.
- Keep the claim-policy metadata, narrative/event artifacts, exact-screen
  reason counts, prune-class counts, manifest provenance, and runtime-threshold
  pass green.
- Keep the new local step-`11` breadth guardrails and the repaired step-`12`
  selector green:
  the connected step-`11` claim surface should stay at
  `kappa 5 = 243`, `kappa 6 = 729` (total `972`), the guarded step-`11`
  shell should stay accepted, and the guarded step-`12` same-primary winner
  should stay fixed without reopening the old step-`12` drop.
- Keep the current local step-`13` / step-`14` / step-`15` surfaces green as
  guardrails:
  step `13` should now stay at `[5,1,3,3,5,3,2]` / `1350` / `2320` with the
  guarded accepted hash, while step `14` stays `19683` / `12027` and stored
  canonical step `15` now stays `DCT 103 / 8 / 3972`.
- Use the stored `v11` certificate and the late-step live checkpoints as the
  first diagnosis surface for the remaining misses; they now expose raw
  catalog widths, root seeding, exact-screen pressure, and the full stored
  step-open pressure envelope for the open steps.
- Treat stored `v11` step `15 = 3972 / 5000` as the remaining stored
  late-floor miss; do not reopen step `11` or step `13` first now that stored
  step `11` re-earns `1338 / 800` and stored step `13` re-earns
  `2320 / 2200` on the clean canonical bundle.
- Treat the new `[3,5,3,3,5,1,1]` and `[5,1,3,3,5,3,3]` local step-`13`
  widenings as negative controls only:
  they prove missing breadth can be re-opened locally, but neither one yet
  preserves accepted-hash parity through step `14`; both the
  position-`1` / position-`4` reland and the
  position-`0` / position-`4` / position-`5` / position-`6` reland are now
  frozen as executable regressions rather than doc-only guidance.
- Keep step `1` on the checklist as a separate stored early breadth blocker.
- Keep step `15` on the checklist as the remaining later stored floor miss on
  the canonical branch.

## Immediate Next Move

1. Freeze `long-rerun-v11` as the current canonical stored claim bundle, keep
   clean `v10` as the pre-anchor-`11` stored baseline, keep `v6` as the clean
   pre-step-`11` breadth baseline, and keep `v5` as the pre-parity completed
   reference; do not resume the stopped `v4` run and do not recapture replay
   fixtures first.
2. Hold the current pre-flight gate, the stored `v11` compare readiness, the
   new local step-`11` breadth guardrails, the repaired step-`12`
   same-primary selector, the stored step-`13`
   `[5,1,3,3,5,3,2]` / `1350` / `2320` accepted surface, the stored
   `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`
   and `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts`
   regressions, and the current step-`14` / step-`15` later-surface guardrails
   green.
3. Use the stored `v11` certificate and late-step live checkpoints as the
   first diagnosis surface for the residual stored `3972 / 5000` miss at
   step `15`; the next move is another parity-preserving local step-`15`
   repair on the remaining summary-stage `small_cluster` same-primary
   `103 / 8` incumbent surface, not another rerun setup pass and not a
   proof-close reland; the new omitted-side-variant regression now proves that
   several demo-only side openings around the live anchor-`11` pocket are
   already safe on rank, but the reverted raw position-`0` reland also proves
   that widening that side globally reopens the old captured clause-`0`
   surface, so the next landed repair must isolate one of those openings onto
   the current `small_cluster` path instead of broadening the whole catalog.
4. Launch the next clean full-profile claim rerun only after that next local
   step-`15` repair is regression-backed and parity-clean, then refresh
   compare / benchmark / certification immediately afterwards and keep
   step `1 = 546 / 2144` explicit as the separate stored early breadth blocker
   unless the rerun itself changes it.
5. Keep the two frozen step-`13` negative controls and the rejected global
   band-`7` / temporal-reanchor / early bridge expansions out of the landed
   path while the residual stored step-`15` diagnosis is pending.
