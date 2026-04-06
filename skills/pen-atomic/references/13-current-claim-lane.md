# Current Claim Lane

Read this file when the task touches `desktop_claim_shadow`, claim-lane
telemetry, claim-lane narratives, or the autonomy-certification roadmap.

## Current Truth

- `desktop_claim_shadow` exists as a separate profile and config family.
- The lane is not yet certified and should still be described with the safer
  `bounded live recovery` wording.
- The current canonical stored claim bundle is clean-tree completed `v11`:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v11`.
- Stored compare, certification, and benchmark outputs now exist beside that
  `v11` bundle, and they already treat accepted-hash parity, claim-policy
  honesty, fallback honesty, narrative/event completeness, exact-screen
  reason coverage, prune-class coverage, and manifest completeness as earned.
- Claim admissibility now uses structural claim debt and anchor hints, without
  named-family focus progression.
- Claim late expansion now uses a claim-specific late surface with structural
  mutators landed for kappa `4-9`, while stored breadth/floor evidence on that
  widened lane is still open.
- Claim bucket scheduling now uses a structural-generic taxonomy derived from
  prefix-local syntax and runtime evidence.
- The current mixed state is recorded explicitly in run metadata:
  - `guidance_style = claim_debt_guided`
  - `late_expansion_policy = claim_generic`
  - `bucket_policy = structural_generic`
- `run.json` now captures CPU, worker-count, build-profile, target, git,
  `Cargo.lock`, and binary fingerprints for claim certification, and the smoke
  certification path now passes the manifest-completeness gate.
- `pen-cli run` and `pen-cli resume` now write `run.json`, step summaries,
  step checkpoints, frontier snapshots, claim narratives/events, and failure
  status incrementally, so failed long claim runs remain auditable from disk.
- claim runs now record observed process RSS alongside governor-accounted RSS
  in stored step pressure data, so the model gap is visible from artifacts.
- claim runs now also emit `step_live_checkpoint` telemetry and
  `reports/steps/step-XX-live.ndjson` artifacts through step `15`, exposing
  observed process RSS, raw catalog widths, frontier queue size, prefix-cache
  size, legality-cache size, and whether late claim widening gates are active
  while the step is still in flight.
- claim auto-worker resolution is now memory-aware on
  `desktop_claim_shadow`, and claim proof-close now drops cached evaluated
  terminal payloads after ranking so the live prefix cache stays smaller.
- claim proof-close now also releases processed retained prefix groups once
  exact certification starts, so the live prefix cache can shrink during
  proof-close instead of carrying already-closed groups to step end.
- claim terminal-prefix materialization now also consumes cached exact
  completion summaries from the legality cache after reuse, so claim runs stop
  holding both the legality-cache payload and the retained prefix-group copy of
  the same exact terminal surface.
- claim terminal-prefix materialization now also has a direct compact path when
  no cached completion summary exists, so claim runs no longer build and then
  immediately re-walk a full terminal evaluation vector just to recover the
  same retained candidates and counts.
- cloned claim prefix signatures now share their serialized exact payload
  across frontier and legality-cache copies, so the same hot-path signature no
  longer duplicates that full prefix string into every cloned cache key.
- claim online frontier work items now reuse the shared clause catalog when no
  prefix-local active-window filter applies, so claim discovery no longer
  clones the full next-clause list into every queued frontier item.
- claim online frontier work items now also reuse that same shared serialized
  prefix key for deterministic ordering, instead of allocating a second copy of
  the serialized prefix string for the queue order key.
- claim terminal-clause filtering now also accepts the shared clause slice
  directly instead of first materializing a fresh `Vec<&ClauseRec>` at every
  terminal-prefix check, so the hot claim release path avoids that per-prefix
  allocation even when the claim lane has no active terminal filter to apply.
- claim exact remaining-two loops now also reuse one scratch terminal
  telescope plus the precomputed prefix bit cost across bound checks,
  completion-summary builds, and compact materialization, so the hot claim
  step-`4` path stops cloning the full prefix telescope for every admitted
  last-clause candidate.
- claim compact discovery now also skips full evaluation for terminal
  candidates that are already below bar or no longer beat the current
  incumbent, so the hot claim step-`4` path stops spending discovery time on
  proof-close work that is already known to be non-winning.
- `scripts/compare_runs.py` now audits claim-policy honesty, exact-screen
  reason coverage, prune-class coverage, narrative artifacts, and whether the
  stored run reaches the step-15 claim signoff surface.
- `scripts/certify_claim_lane.py` now emits a stored pass/fail certificate from
  claim artifacts and currently fails honestly on missing breadth, missing
  generated-floor evidence; the current `v11` certificate still flags
  `early_breadth` plus `late_generated_floors`, and it now also records
  step-level breadth diagnosis for failing steps from the stored summaries
  plus late-step live checkpoints; that diagnosis now also preserves the full
  stored step-open pressure signature, including active widening bands,
  package flags, and claim-debt `path` / `trunc` pressure.
- the canonical `v11` certificate plus the frozen `step-15-live.ndjson`
  provenance are now pinned by
  `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`,
  the refreshed benchmark bundle is pinned by
  `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts`, and the
  current breadth miss anatomy is executable in-tree rather than notes-only
- `scripts/benchmark_claim_lane.py` now aggregates stored claim runs into a
  benchmark bundle with runtime percentiles, parity counts, breadth-floor hit
  counts, and manifest snapshots; it still needs a breadth-clean stored claim
  bundle before those numbers can justify a stronger sentence.
- The current stored breadth snapshot on canonical `v11` is:
  - step `1`: `546 / 2144`
  - step `10`: `1428 / 500`
  - step `11`: `1338 / 800`
  - step `12`: `1338 / 1200`
  - step `13`: `2320 / 2200`
  - step `14`: `12027 / 3500`
  - step `15`: `3972 / 5000`
- The stored certificate now makes the remaining miss split directly visible:
  - step `1` still shows raw catalog widths `[18,36]`, raw catalog `648`,
    seeded roots `18` seen / `16` enqueued, and `435`
    legality/connectivity exact rejections before proof-close
  - step `15` still shows raw catalog `6561`, seeded roots `3`, then
    `468` partial-prefix bar failures plus `242` incumbent-dominance prunes,
    `0` legality/connectivity exact rejections, `243` well-formed
    candidates, and `469` exact-bound-screened candidates, together with the
    `claim_generic` `kappa 8..8` temporal-shell opening, modal anchor ref
    `10`, and active widening bands `7,8,9`
- A fresh stored rerun stack has now consumed the guarded local step-`11`
  breadth repair:
  - the connected claim step-`11` surface now holds
    `kappa 5 = 243`, `kappa 6 = 729` (total `972`)
  - local exact-screen connectivity rejections there are now `0`
  - the guarded accepted step-`11` shell stays fixed locally
  - dirty-tree `v7` first re-earned stored step `11` but reopened accepted-
    hash parity at step `12`
  - the narrow step-`12` same-primary selector repair now lands on top of that
    rerun read
  - clean-tree `v9` restores accepted-hash parity through step `15`
  - clean-tree `v10` then keeps that parity while re-earning stored
    step `13` as a breadth hit beside stored steps `11` and `12`
  - clean-tree `v11` now consumes the isolated anchor-`11` step-`15` repair
    on stored evidence while keeping those stored step `11` / `12` / `13`
    hits intact
- the repo-level autonomy docs now treat claim-policy separation, failed-run
  evidence preservation, and one parity-clean full-profile stored bundle as
  baseline; the live blocker is diagnosing the remaining stored breadth misses
  on the canonical chain, not reopening another step-`4` survival pass first
- the parity-preserving step-`13` repair is now earned on stored evidence:
  - the canonical step-`13` surface now widens to
    `[5,1,3,3,5,3,2]` / raw `1350` / generated `2320`
  - the guarded step-`13` metric shell now stays accepted on that stored
    surface while the canonical step-`14` / step-`15` continuation remains
    `62 / 9 / 12027 -> 103 / 8 / 3972`
  - that stored step-`13` surface now records `576`
    legality/connectivity exact rejections plus `401` heuristic drops before
    proof-close
- two newer non-landed local step-`13` widened probes now frame that next
  diagnosis more tightly:
  - `[3,5,3,3,5,1,1]` / raw `675` / generated `2223` reopens the local floor
    but changes the accepted late path to `45 / 7 -> 61 / 9`
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
    out of the retained pool on that second widened surface, so neither
    widened probe is safe to land raw
- the nearby clause-`3` anchor-`11` exact-argument pocket on the remaining
  step-`15` clause-`2` / clause-`3` blocker is now frozen more tightly too:
  on every current mixed clause-`2` claim/demo replacement, that pocket is a
  full-rank `103 / 8` bar-clearer that matches the canonical `DCT 103 / 8`
  winner on overshoot, clause `kappa`, eliminator/former/density/library/
  binder/closure signals, max var reach, and `nu`, and still loses only on
  higher bit cost `236` versus canonical `229`; the lifted anchor-`11`
  neighbors still reopen the unsafe `88 / 8` rival, so any future repair
  there must isolate the exact-argument qualifier evidence without relanding
  the lifted variants or replacing the canonical accepted path
- the narrow step-`15` anchor-`11` exact-argument repair has now been consumed
  on stored evidence:
  mixed live claim clause-`2` prefixes now expose exactly one additional
  clause-`3` option, the isolated anchor-`11` exact-argument pocket, while
  reference clause-`2` prefixes and the lifted anchor-`11` neighbors stay out
  of the live clause-`3` catalog; the live claim clause-`2` variants now
  regain historical reanchor on that isolated pocket across every repaired-
  side subset of clause positions `0`, `1`, `4`, and `5`, but clause `6`
  still stays the local safety boundary that reopens the unsafe `89 / 8`
  rival; on the stored canonical late chain, step `15` still accepts
  `DCT 103 / 8`, stored generated prefixes now sit at `3972`, the remaining
  stored gap is `1028`, partial-prefix bar failures stay `468`,
  incumbent-dominance prunes now sit at `242`, legality/connectivity exact
  rejections are now `0`, and the surviving temporal terminal cluster now
  widens to `2190` generated / `244` admitted / `244` exact-screened /
  `242` pruned
- a newer local step-`15` clause-`4` side-pocket repair now lands on top of
  that stored-`v11` read:
  - the `demo_sharp_codomain` clause-`4` opening is now relanded only on the
    exact anchor-`11` exact-argument pocket and now counts as historical
    reanchor only there
  - the repaired local late chain still accepts `DCT 103 / 8`, but live
    generated prefixes now lift from `3972` to `4004`
  - the remaining local pressure is now `472` partial-prefix bar failures plus
    `244` incumbent-dominance prunes, with legality/connectivity exact
    rejection still at `0`
  - the local surviving temporal terminal cluster now widens to
    `2208` generated / `246` admitted / `246` exact-screened / `244` pruned
  - the captured zero-admitted surface still stays tightly localized at
    `1956` families / `5868` connected-but-unqualified terminal options, with
    reanchor-prefix progress now split as `1470` clause-`2`-side families and
    `486` clause-`3`-side families
  - the reopened clause-`4` side pocket itself stays tiny and noncanonical:
    only `6` captured prefixes per live claim clause-`2` variant, exact-
    terminal recovered profiles limited to the unsafe `89 / 8` rival plus the
    canonical-primary `103 / 8` tie-clean pocket, and forced reanchor winners
    there still non-reference terminal variants only
- a newer local step-`15` clause-`5` side-pocket repair now lands on top of
  that clause-`4` pocket:
  - the `demo_sharp_domain` clause-`5` opening is now relanded only once the
    exact anchor-`11` clause-`4` side pocket is already present, and it now
    counts as historical reanchor only on that doubly fenced pocket
  - the repaired local late chain still accepts `DCT 103 / 8`, and live
    generated prefixes now lift from `4004` to `4030`
  - the remaining local pressure is now `472` partial-prefix bar failures plus
    `246` incumbent-dominance prunes, with legality/connectivity exact
    rejection still at `0`
  - the local surviving temporal terminal cluster now widens again to
    `2226` generated / `248` admitted / `248` exact-screened / `246` pruned
  - the captured zero-admitted surface still stays tightly localized at
    `1956` families / `5868` connected-but-unqualified terminal options, so
    the clause-`2` / clause-`3` capture remains fenced
- a newer local step-`15` small-cluster relief now lands on top of that
  clause-`5` pocket:
  - the repaired local late chain still accepts `DCT 103 / 8`, and live
    generated prefixes stay fixed at `4030`
  - the remaining local pressure is now `472` partial-prefix bar failures plus
    only `3` incumbent-dominance prunes, with
    legality/connectivity exact rejection still at `0`
  - the local surviving temporal terminal cluster now widens further to
    `2964` generated / `494` admitted / `494` exact-screened / `0` pruned
  - the isolated `single` pocket still stays fenced as one fully scored
    non-winning terminal at overshoot `115657 / 21112`, while the remaining
    incumbent pressure now survives only as `3` residual `single`-bucket
    prunes rather than as the old `246`-candidate `small_cluster` wall
  - the captured zero-admitted surface still stays tightly localized at
    `1956` families / `5868` connected-but-unqualified terminal options, so
    the landed relief changes only the incumbent screen and not the broader
    clause-`2` / clause-`3` capture boundary
- a newer local step-`15` clause-`5` demo-flat-codomain side-pocket repair
  now lands on top of that same exact anchor-`11` clause-`4` pocket plus the
  small-cluster relief:
  - the `demo_flat_codomain` clause-`5` opening now also counts as
    historical reanchor only on that same doubly fenced pocket
  - the repaired local late chain still accepts `DCT 103 / 8`, and live
    generated prefixes now lift from `4030` to `4056`
  - the remaining local gap to the late floor is now `944`, with
    partial-prefix bar failures still at `472`, incumbent-dominance prunes
    still at `3`, and legality/connectivity exact rejection still at `0`
  - the surviving local temporal terminal `small_cluster` now widens again to
    `2988` generated / `498` admitted / `498` exact-screened / `0` pruned,
    while the isolated `single` pocket still remains the only fully scored
    non-winning terminal plus `3` residual single-bucket prunes
  - the captured zero-admitted surface still stays tightly localized at
    `1956` families / `5868` connected-but-unqualified terminal options, so
    the clause-`2` / clause-`3` capture remains fenced
- a newer local step-`15` clause-`4` demo-sharp-bridge side-pocket repair
  now lands on top of that same exact anchor-`11` side-pocket stack:
  - the `demo_sharp_bridge` clause-`4` opening now also counts as
    historical reanchor only on that exact side pocket and only with the
    reference terminal
  - the repaired local late chain still accepts `DCT 103 / 8`, and live
    generated prefixes now lift from `4056` to `4088`
  - the remaining local gap to the late floor is now `912`, with
    partial-prefix bar failures now at `476`, incumbent-dominance prunes
    still at `3`, and legality/connectivity exact rejection still at `0`
  - the surviving local temporal terminal `small_cluster` now widens again to
    `3012` generated / `502` admitted / `502` exact-screened / `0` pruned,
    while the isolated `single` pocket still remains the only fully scored
    non-winning terminal plus `3` residual single-bucket prunes
  - the captured zero-admitted surface now stays tightly localized at
    `1968` families / `5904` connected-but-unqualified terminal options, and
    historical-reanchor prefix progress there now splits as `1482`
    clause-`2`-side families plus `486` clause-`3`-side families
- a newer local step-`15` proof-close incumbent regression now freezes that
  remaining fenced pressure more tightly:
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
    now pins that the remaining `3` incumbent-dominance prunes are all
    `proof_close_group` captures in the fenced
    `k8:structural_generic:temporal_operator:library_backed:single` bucket
  - all `3` still sit on the same-primary `103 / 8` non-winning profile with
    overshoot `115657 / 21112` and bit cost `236`
  - those residual groups now localize to exactly three first-mismatch prefix
    families at clause positions `0`, `2`, and `5`, so the next local repair
    can stay narrower than reopening the whole pocket
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_pin_the_exact_claim_family_labels`
    now freezes those same residual families more concretely as clause-`0`
    `claim_flat_domain`, clause-`2` `claim_flat_domain` plus the
    anchor-`11` exact-argument pocket, and clause-`5`
    `claim_flat_codomain`
- the executable repaired late-path and negative-control guardrails are now
  synced to that same latest local state too:
  - `repaired_claim_step_twelve_late_path_has_scoped_step_thirteen_widening_before_proof_close`
    now freezes the repaired canonical continuation through
    `46 / 7 / 2320 -> 62 / 9 / 12027 -> 103 / 8 / 4088`
  - the local step-`15` guardrails now freeze `4088`, `3`
    incumbent-dominance prunes, and the
    `3012 / 502 / 502 / 0` `small_cluster` surface instead of the older
    clause-`5` `4030` / `246` / `2226 / 248 / 248 / 246` read
  - the frozen `[5,1,3,3,5,3,3]` step-`13` negative control still preserves
    guarded step-`14` / step-`15` hashes while now also observing that same
    repaired local `4088` step-`15` surface
- a new local step-`15` survivor-bucket regression now freezes that
  exact-screened survivor split more tightly:
  - `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`
    now pins exactly two
    `k8:structural_generic:temporal_operator:library_backed` buckets on the
    repaired canonical chain
  - the isolated `single` bucket still carries the one fully scored
    non-winning pocket with overshoot `115657 / 21112` plus `3` residual
    incumbent-dominance prunes
  - the `small_cluster` bucket now widens to `3012` generated /
    `502` admitted / `502` exact-screened / `0` pruned
  - the accepted canonical step-`15` winner still remains the only retained
    candidate there and keeps bit cost `229`
- a new local step-`15` small-cluster-relief regression now freezes that
  landed repair more tightly:
  - `current_claim_step_fifteen_small_cluster_relief_clears_summary_prunes_while_three_single_bucket_prunes_remain`
    now pins that the old `246`-candidate summary-stage `small_cluster`
    incumbent wall has collapsed entirely
  - local incumbent-dominance pressure now sits at `3`
  - the isolated `single` pocket stays fenced even though the
    `small_cluster` wall is no longer dying during exact screening
- a new local step-`15` proof-close incumbent regression now freezes that
  remaining pressure more tightly too:
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
    now keeps the remaining `3` prunes on `proof_close_group`
  - those `3` still stay inside the fenced temporal `single` bucket and stay
    same-primary `103 / 8` non-winners
  - their first mismatches now stay localized to clause positions `0`, `2`,
    and `5`
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_pin_the_exact_claim_family_labels`
    now makes those exact families explicit as clause-`0`
    `claim_flat_domain`, clause-`2` `claim_flat_domain` plus the
    anchor-`11` exact-argument pocket, and clause-`5`
    `claim_flat_codomain`
- a new local step-`15` proof-close surface-count regression now sharpens
  that same blocker without relanding the rejected blanket retention move:
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_each_still_carry_a_three_generated_one_admitted_surface`
    now pins that each of those same three fenced proof-close families is
    already only a tiny local `3`-generated / `1`-admitted / `1`-pruned
    surface
  - the remaining step-`15` pressure is therefore still family-local rather
    than a hidden broad-bucket miss
  - a non-landed proof-close reland that reused those preserved broader
    group-surface counts to grant same-primary relief was checked and
    reverted: it collapsed local incumbent-dominance prunes from `3` to `0`,
    but it also recreated the rejected blanket-retention anatomy by reopening
    `4` retained step-`15` candidates on the same
    `2964 / 494 / 494 / 0` `small_cluster` surface
  - the next landed repair should therefore keep using those preserved counts
    only as diagnosis and stay family-local rather than bucket-global
- a new local step-`15` raw-terminal regression now sharpens that same
  blocker one step further on those exact three families:
  - `current_claim_step_fifteen_residual_single_bucket_incumbent_families_still_hide_two_unsafe_lifted_terminals`
    now pins that each of those same fenced proof-close families still sits
    on the same raw three-terminal shell
  - within each shell, the reference terminal remains the same-primary
    `103 / 8` non-winner at bit cost `236`, while both `next_lift` and
    `eventual_lift` stay connected, locally admissible, and
    stronger-than-canonical `89 / 8` rivals at bit cost `254`
  - the next landed repair therefore cannot blindly reland those residual
    families through broader same-primary retention or raw local terminal
    recovery; it still has to keep those unsafe `89 / 8` lifts fenced
- a new non-landed exact-family same-primary relief probe now sharpens that
  same blocker one step further without reopening the cleared
  `small_cluster`:
  - `current_claim_step_fifteen_exact_family_same_primary_relief_still_unfences_the_isolated_single_pocket`
    now pins that granting same-primary incumbent relief only to those exact
    clause-`0`, clause-`2` + anchor-`11`, and clause-`5` residual families
    keeps live generated prefixes flat at `4088` and keeps partial-prefix bar
    failures at `476`
  - it does collapse the remaining incumbent-dominance prunes from `3` to `0`
  - the cleared `small_cluster` stays unchanged at
    `3012 / 502 / 502 / 0`
  - but the isolated `single` pocket still unfences from `1` to `4`
    fully scored non-winning terminals at the same overshoot
    `115657 / 21112`
  - the next landed repair must therefore stay narrower than same-primary
    relief even at exact-family scope and keep the isolated `single` pocket
    fenced
- a new non-landed blanket step-`15` same-primary retention probe was also
  run and reverted against that same repaired canonical chain:
  - enabling same-primary incumbent relief across the whole local step-`15`
    chain kept live generated prefixes flat at `4030` and kept
    partial-prefix bar failures at `472`
  - it did collapse incumbent-dominance prunes from `246` to `0`, but it also
    unfenced the isolated `single` pocket from `1` to `4` fully scored
    non-winning terminals
  - the surviving temporal `small_cluster` aggregate widened from
    `2226 / 248 / 248 / 246` to `2964 / 494 / 494 / 0`, so that broad
    retention probe changed the local exact-screen anatomy without raising the
    generated floor
  - because it neither lifted the local floor nor preserved the isolated
    `single` fence, it was reverted
- a new local omitted-side-variant regression now sharpens the next safe move
  on that same repaired anchor-`11` pocket without landing another widening
  yet:
  - `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_stay_same_primary_and_non_winning`
    now pins that the omitted demo-only temporal-shell side variants at clause
    positions `0`, `1`, `4`, and `5` all stay structurally connected, locally
    admissible, and same-primary `103 / 8` / `115657 / 21112` on top of the
    current live claim clause-`2` + anchor-`11` exact-argument pocket
  - the clause-`4` `demo_sharp_codomain` and `demo_sharp_bridge` variants now
    regain historical reanchor only on that exact side pocket, while the
    clause-`0`, clause-`1`, and clause-`5` variants still stay outside
    historical reanchor there and still lose only on higher bit cost `243`,
    `245`, or `250`
  - a newer
    `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_still_fence_unsafe_lifted_terminals`
    regression now further pins that every one of those omitted side variants
    stays structurally connected but outside historical reanchor once the
    terminal lifts, so both unsafe `next_lift` and `eventual_lift`
    terminals remain fenced there
  - a non-landed raw position-`0` reland probe briefly lifted the local
    step-`15` generated surface to `4285`, but it also reopened the repaired
    clause-`0` zero-admitted capture to `2835` families / `891`
    reanchor-prefix misses, so that broad catalog reland was reverted
  - that guidance is now partly consumed:
    the clause-`5` `demo_sharp_domain` opening has since been isolated onto
    the existing clause-`4` anchor-`11` side pocket, while the raw
    position-`0` temporal-shell catalog still stays out
- the older raw isolated late-side reland probes are now bounded more
  tightly:
  - a clause-`4`-only demo-sharp-codomain opening, gated on canonical clauses
    `0` and `1` plus the live claim clause-`2` + anchor-`11` exact-argument
    pocket, lifted local step `15` generated prefixes only to `3980`, but it
    also reopened the zero-admitted capture to `1962` families with `1476`
    clause-`2` historical-reanchor-prefix misses
  - a clause-`5`-only demo-sharp-domain opening, gated on canonical clauses
    `0`, `1`, and `4` plus that same pocket, lifted the local surface only to
    `3974` while still reopening the zero-admitted capture to `1950` families
    with `1464` clause-`2` historical-reanchor-prefix misses
  - so clause `4` is now consumed only as a fenced tiny pocket, and the next
    landed repair should not be another raw isolated clause-`4` reland or a
    raw isolated clause-`5` late-side reland either; any future use of those
    openings will need extra qualifier / reanchor evidence or narrower
    residual-incumbent work that keeps them on the repaired path instead of
    the captured clause-`2` / clause-`3` surface, and the new lifted-terminal
    fence regression means any future omitted-side reland there must stay
    reference-terminal-local too; the reverted blanket same-primary-relief
    probe already proved that a full step-`15` incumbent-retention reland is
    too broad

## Current Operational Blockers

- the lane still does not have a signoff-ready certified bundle even though
  stored `v11` now passes accepted-hash parity and the compare/certification/
  benchmark infrastructure is live
- stored breadth still fails honestly on the canonical chain:
  - early breadth still misses at step `1` (`546` versus `2144`)
  - late generated floors still miss only at step `15` (`3972` versus `5000`)
  - stored breadth already hits at step `10`, step `11`, step `12`,
    step `13`, and step `14`
- step `15` is now the remaining stored late-floor miss on the clean
  canonical bundle
- the next rerun or step-`13` theory pass is no longer the first move:
  stored step `13` is already closed and the anchor-`11` step-`15` repair is
  already consumed on canonical `v11`, so the next move is the next local
  step-`15` diagnosis / repair on that residual stored `3972 / 5000` surface
  while keeping the newer local `4088` guardrail surface fenced and while
  keeping the newly frozen unsafe `89 / 8` lifted terminals fenced on the
  three residual single-bucket family shells
- the canonical repaired late chain must stay frozen while breadth is
  re-earned:
  - step `12` should keep the guarded `34 / 6` continuation
  - stored step `13` should stay at `[5,1,3,3,5,3,2]` / `1350` / `2320`
    with the guarded accepted hash
  - step `14` should stay at `19683` / `12027`
  - stored canonical step `15` should now stay on `DCT 103 / 8 / 3972`
- step `1` remains a separate stored early breadth blocker even if late-step
  repairs continue to land
- benchmark evidence is still too weak for a passing claim certificate until a
  fresh stored bundle closes the remaining breadth failures without losing
  accepted-hash parity

## Immediate Next Slice

1. Freeze clean-tree completed `v11` as the current canonical stored claim
   bundle, keep clean `v10` as the pre-anchor-`11` stored baseline, keep clean
   `v6` as the pre-step-`11` breadth baseline, and keep completed `v5` as the
   pre-parity reference surface.
2. Hold the current local and stored guardrails green before reopening any new
   theory: the step-`11` connected surface should stay at `243 + 729 = 972`,
   the guarded step-`11` shell should stay accepted, the repaired step-`12`
   `34 / 6` continuation should stay fixed, the stored step-`13..15`
   canonical surfaces should stay frozen on the repaired branch, and
   `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`,
   `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts`, and
   `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`
   should stay green there too.
3. Treat the residual stored `3972 / 5000` step-`15` gap as the current local
   diagnosis target; use the `v11` certificate and `step-15-live.ndjson`
   first, and do not spend another cycle on rerun setup or step-`13` theory
   before the next parity-preserving local repair is regression-backed; keep
   the isolated `single` survivor bucket fenced while the next repair works
   against the remaining local `3`-incumbent surface on top of the new
   `4088` guardrail now that the old `small_cluster` summary wall is gone.
   The newer clause-`4` `demo_sharp_codomain` plus `demo_sharp_bridge`
   reopenings and the clause-`5` `demo_sharp_domain` and
   `demo_flat_codomain` reopenings are now already consumed narrowly and must
   stay fenced as a tiny noncanonical pocket stack, the reverted raw
   position-`0` reland still proves that broadening that side globally
   reopens the old captured clause-`0` surface, and the next landed repair
   should therefore target those residual `single`-bucket incumbent prunes
   rather than another raw clause-`4`, raw clause-`5`, raw
   position-`0`, or blanket same-primary-retention reland. The new
   proof-close incumbent freeze now further localizes that next move to three
   same-primary `103 / 8` fenced prefix families first diverging at clause
   positions `0`, `2`, and `5`.
   `current_claim_step_fifteen_exact_family_same_primary_relief_still_unfences_the_isolated_single_pocket`
   now further pins that even granting same-primary relief only to those
   exact three families keeps `4088` / `476`, collapses incumbent-dominance
   from `3` to `0`, leaves the `small_cluster` unchanged at
   `3012 / 502 / 502 / 0`, and still unfences the isolated `single` pocket
   from `1` to `4` fully scored non-winning terminals.
4. Keep `[3,5,3,3,5,1,1]` and `[5,1,3,3,5,3,3]` as negative controls only:
   they still prove local breadth can be reopened unsafely, but they are not
   the landed repair.
5. Keep stored step `15 = 3972 / 5000` and step `1 = 546 / 2144` in view
   beside that work; do not reopen another stored step-`11` rerun first now
   that clean-tree `v11` already keeps stored step `11 = 1338 / 800`.
6. If the nearby clause-`3` anchor-`11` neighborhood is touched again, keep
   the live repair boundary explicit: only the exact-argument pocket is
   landed, it is isolated to the current claim clause-`2` variants, it stays
   non-winning by losing to the canonical winner on bit cost `236` versus
   `229`, clause `6` still fences the unsafe `89 / 8` rival, and the lifted
   anchor-`11` variants must stay out.
7. Do not reland the raw global position-`0` temporal-shell widening first:
   the exploratory probe lifted local step `15` only to `4285` and reopened
   the repaired clause-`0` zero-admitted capture, so any next opening there
   has to be isolated onto the live anchor-`11` `small_cluster` path.
8. Do not reland another raw broad clause-`4` opening or a raw isolated
   clause-`5` late-side opening first either:
   clause `4` is already consumed narrowly by the fenced exact anchor-`11`
   side pocket, and the older pocket-gated probes only reached `3980` or
   `3974` while still reopening the captured clause-`2` / clause-`3`
   surface.

## First Reads

- [../../autonomous_plan.md](../../autonomous_plan.md)
- [../../autonomous_next_steps.md](../../autonomous_next_steps.md)
- [../../autonomous_progress.md](../../autonomous_progress.md)
- [../../autonomous_checklist.md](../../autonomous_checklist.md)
- [../../README.md](../../README.md)
- [../../docs/ARCHITECTURE.md](../../docs/ARCHITECTURE.md)

## Do And Do Not

Do:

- treat the current claim lane as a mixed-mode scaffold with honest metadata
- preserve existing guarded, realistic, and demo behavior while the claim lane
  changes
- prefer structural explanations over family-name explanations in new claim
  code
- assume the local shell is Windows PowerShell and avoid shell chaining such
  as `&&`; use separate commands for staging, commit, and push work
- keep claim-lane edits narrow and staged; split very large file updates into
  smaller targeted patches when the first broad patch does not land cleanly
- focus next on diagnosing the remaining stored breadth misses on the clean
  canonical bundle, keeping step `15` / step `1` in view, keeping stored
  step `13` frozen as a hit, and keeping the repaired local
  step-`11` / step-`12` / step-`13..15` guardrails fixed

Do not:

- claim that the whole lane is already family-agnostic end-to-end
- switch `bucket_policy` early
- spend time reopening already-landed claim-policy split work unless a memory
  or evidence bug forces it
- reopen another local step-`11` selector or raw-connectivity theory first:
  the current local breadth repair is already landed and guarded
- reopen another runtime-only step-`4` micro-optimization first unless a fresh
  stored rerun proves the remaining misses are really runtime fallout
- reland the rejected global band-`7` widening or the rejected late reanchor /
  early bridge expansions first while stored breadth is still open
- reland the newer `[3,5,3,3,5,1,1]` or `[5,1,3,3,5,3,3]` step-`13`
  widenings raw; both still leave accepted-hash parity open
- call the lane `unguided` yet
