# Autonomous Claim Lane Plan

Last updated: 2026-04-06
Status: active

This file is the staged path from the current parity-clean-but-not-certified
claim-lane state to final signoff. It is strategic rather than diary-like.

## Objective

Produce one stored `desktop_claim_shadow` bundle from the disclosed desktop
that:

- finishes through step `15`
- preserves accepted-hash parity and honest breadth accounting
- passes compare, benchmark, and certification on stored evidence

Until that bundle exists, keep the paper wording at `bounded live recovery`.

## Current Strategic Position

- The current canonical stored claim bundle is clean-tree completed `v11`:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v11`
  on repo head `4e38f2463b429bcbebe16cc6d7c5eac7ef2de050` with release binary
  hash `64535f9e6c2e2a77c1bdeeb1f848abbeb0312f9b454bce42f27c68b3b852271c`.
- Stored compare, certification, and benchmark outputs now exist beside that
  `v11` run and are the current authoritative evidence surface.
- The stored `v11` certificate plus the frozen `step-15-live.ndjson`
  provenance are now pinned by
  `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`,
  and the refreshed `v11` benchmark bundle is pinned by
  `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts`,
  so the current canonical breadth diagnosis is executable in-tree.
- Stored accepted-hash parity is earned on `v11` through step `15`.
- Runtime, manifest completeness, fallback honesty, narrative/event
  completeness, exact-screen reason completeness, and prune-class completeness
  are all earned on stored evidence.
- Stored step `13` is now re-earned on the canonical chain:
  - the stored claim-open surface now widens to
    `[5,1,3,3,5,3,2]` / raw `1350` / generated `2320`
  - the guarded step-`13` metric shell stays accepted there and keeps the
    canonical `62 / 9 / 12027 -> 103 / 8 / 3972` continuation through
    steps `14` and `15`
  - the stored step-`13` surface now seeds `5` roots and pays
    `576` legality/connectivity exact rejections plus `401` heuristic drops
    before proof-close
- The remaining blockers are stored breadth misses on the canonical chain:
  step `1 = 546 / 2144` and step `15 = 3972 / 5000`.
- Step `15` is now the remaining stored late-floor miss, and the next honest
  local engineering dollar is stored step-`15` diagnosis / repair on top of
  `v11`, not another rerun first and not another step-`13` theory pass.
- The narrow local step-`15` anchor-`11` repair has now been consumed on
  stored evidence:
  - mixed live claim clause-`2` prefixes now expose exactly one additional
    clause-`3` option, the isolated anchor-`11` exact-argument pocket, while
    reference clause-`2` prefixes and the lifted anchor-`11` neighbors stay
    out of the live clause-`3` catalog
  - the live claim clause-`2` variants now regain historical reanchor on that
    isolated pocket across every repaired-side subset of clause positions
    `0`, `1`, `4`, and `5`, while clause `6` still stays the safety boundary
    that reopens the unsafe `89 / 8` rival
  - on the stored canonical late chain, step `15` still accepts
    `DCT 103 / 8`, and stored generated prefixes have now lifted from `1794`
    to `3972`
  - the remaining stored gap is now `1028`, with step-`15`
    partial-prefix bar failures still at `468`, incumbent-dominance prunes
    now at `242`, and legality/connectivity exact rejections now at `0`
  - the stored surviving temporal terminal cluster now widens to
    `2190` generated / `244` admitted / `244` exact-screened / `242` pruned,
    and one isolated fully scored non-winning anchor-`11` terminal pocket is
    now frozen with overshoot `115657 / 21112`
- A newer local step-`15` clause-`4` side-pocket repair now lands on top of
  that stored-`v11` diagnosis:
  - the `demo_sharp_codomain` clause-`4` opening is now relanded only on the
    exact anchor-`11` exact-argument pocket and now counts as historical
    reanchor only there
  - the repaired local late chain still accepts `DCT 103 / 8`, but live
    generated prefixes now lift from `3972` to `4004`
  - the remaining local gap to the late floor is now `996`, with
    step-`15` partial-prefix bar failures at `472`, incumbent-dominance
    prunes at `244`, and legality/connectivity exact rejections still at `0`
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
- A newer local step-`15` clause-`5` side-pocket repair now lands on top of
  that clause-`4` pocket:
  - the `demo_sharp_domain` clause-`5` opening is now relanded only once the
    exact anchor-`11` clause-`4` side pocket is already present, and it now
    counts as historical reanchor only on that doubly fenced pocket
  - the repaired local late chain still accepts `DCT 103 / 8`, and live
    generated prefixes now lift from `4004` to `4030`
  - the remaining local gap to the late floor is now `970`, with
    step-`15` partial-prefix bar failures still at `472`,
    incumbent-dominance prunes now at `246`, and
    legality/connectivity exact rejections still at `0`
  - the local surviving temporal terminal cluster now widens again to
    `2226` generated / `248` admitted / `248` exact-screened / `246` pruned
  - the captured zero-admitted surface still stays tightly localized at
    `1956` families / `5868` connected-but-unqualified terminal options, so
    the clause-`2` / clause-`3` capture remains fenced
- A newer local step-`15` small-cluster relief now lands on top of that
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
- A newer local step-`15` clause-`5` demo-flat-codomain side-pocket repair
  now lands on top of that same exact anchor-`11` clause-`4` pocket plus the
  small-cluster relief:
  - the `demo_flat_codomain` clause-`5` opening now also counts as
    historical reanchor only on that same doubly fenced pocket
  - the repaired local late chain still accepts `DCT 103 / 8`, and live
    generated prefixes now lift from `4030` to `4056`
  - the remaining local gap to the late floor is now `944`, with
    step-`15` partial-prefix bar failures still at `472`,
    incumbent-dominance prunes still at `3`, and
    legality/connectivity exact rejections still at `0`
  - the local surviving temporal terminal cluster now widens again to
    `2988` generated / `498` admitted / `498` exact-screened / `0` pruned,
    while the isolated `single` pocket still remains the only fully scored
    non-winning terminal plus `3` residual single-bucket prunes
  - the captured zero-admitted surface still stays tightly localized at
    `1956` families / `5868` connected-but-unqualified terminal options, so
    the landed repair still leaves the broader clause-`2` / clause-`3`
    capture boundary untouched
- A newer local step-`15` clause-`4` demo-sharp-bridge side-pocket repair now
  lands on top of that same exact anchor-`11` pocket stack:
  - the `demo_sharp_bridge` clause-`4` opening now also counts as historical
    reanchor only on that exact side pocket and only with the reference
    terminal
  - the repaired local late chain still accepts `DCT 103 / 8`, and live
    generated prefixes now lift from `4056` to `4088`
  - the remaining local gap to the late floor is now `912`, with step-`15`
    partial-prefix bar failures now at `476`, incumbent-dominance prunes still
    at `3`, and legality/connectivity exact rejections still at `0`
  - the local surviving temporal terminal cluster now widens again to
    `3012` generated / `502` admitted / `502` exact-screened / `0` pruned,
    while the isolated `single` pocket still remains the only fully scored
    non-winning terminal plus `3` residual single-bucket prunes
  - the captured zero-admitted surface still stays tightly localized at
    `1968` families / `5904` connected-but-unqualified terminal options, so
    the landed repair still leaves the broader clause-`2` / clause-`3`
    capture boundary untouched
- A new executable step-`15` survivor-bucket regression now freezes that
  post-repair split more tightly:
  - exactly two
    `k8:structural_generic:temporal_operator:library_backed` buckets remain
    on the canonical repaired surface
  - the isolated `single` bucket carries one fully scored non-winning pocket
    at overshoot `115657 / 21112` plus the residual `3`
    incumbent-dominance prunes
  - the `small_cluster` bucket is now no longer the incumbent wall:
    it holds `2988` generated / `498` admitted / `498` exact-screened /
    `0` pruned
- A newer executable
  `current_claim_step_fifteen_small_cluster_relief_clears_summary_prunes_while_three_single_bucket_prunes_remain`
  regression now freezes that new local state more tightly:
  - summary-stage incumbent capture on the `small_cluster` surface is now `0`
  - local incumbent-dominance pressure is now only `3`
  - the isolated `single` pocket stays fenced even though the
    `small_cluster` wall is no longer dying during exact screening
- A newer executable
  `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
  regression now freezes that remaining proof-close pressure more tightly:
  - the remaining `3` incumbent-dominance prunes are all
    `proof_close_group` captures in the fenced temporal `single` bucket
  - all `3` stay same-primary `103 / 8` non-winners at overshoot
    `115657 / 21112` with bit cost `236`
  - those residual groups now localize to exactly three first-mismatch prefix
    families at clause positions `0`, `2`, and `5`
- A newer executable
  `current_claim_step_fifteen_residual_single_bucket_incumbent_families_still_hide_two_unsafe_lifted_terminals`
  regression now sharpens that same local blocker further:
  - each of those same three fenced families still sits on the same raw
    three-terminal shell
  - the reference terminal there stays a same-primary `103 / 8` non-winner
    at bit cost `236`
  - the neighboring `next_lift` and `eventual_lift` terminals there still
    stay connected, locally admissible, and stronger-than-canonical
    `89 / 8` rivals at bit cost `254`
  - the next landed repair therefore still has to keep those unsafe lifted
    terminals fenced rather than blindly relanding broader same-primary or raw
    local-terminal retention on that family shell
- A newer executable
  `current_claim_step_fifteen_exact_family_same_primary_relief_still_unfences_the_isolated_single_pocket`
  regression now sharpens that same blocker one step further:
  - granting same-primary incumbent relief only to those exact
    clause-`0`, clause-`2` + anchor-`11`, and clause-`5` residual families
    still keeps the repaired local surface at `4088` generated prefixes with
    `476` partial-prefix bar failures
  - it does collapse the remaining incumbent-dominance prunes from `3` to `0`
  - the cleared `small_cluster` stays unchanged at
    `3012 / 502 / 502 / 0`
  - but the isolated `single` pocket still unfences from `1` to `4`
  fully scored non-winning terminals, so the next landed repair must stay
  narrower than same-primary relief even at exact-family scope
- A newer executable
  `current_claim_step_fifteen_subset_local_same_primary_relief_only_trades_single_prunes_for_non_winners`
  regression now sharpens that same blocker further again:
  - every non-empty strict subset of those same three residual families still
    keeps the repaired local surface at `4088` generated prefixes with `476`
    partial-prefix bar failures
  - the cleared `small_cluster` still stays unchanged at
    `3012 / 502 / 502 / 0`, and the repaired canonical winner also stays
    unchanged
  - instead, opening any strict subset only trades `n` of the remaining `3`
    incumbent-dominance prunes for `n` extra fully scored non-winning
    terminals in the isolated `single` pocket, and those extra candidates are
    all same-primary `103 / 8`, bit-cost-`236` reference-terminal completions
    from the selected residual families
  - the next landed repair must therefore stay narrower than proof-close
    same-primary relief even on a strict subset of those three families,
    because even reference-terminal-only same-primary retention there is still
    too broad
- A newer executable omitted-side-variant regression now sharpens that same
  `small_cluster` target without landing another widening yet:
  - the omitted demo-only temporal-shell side variants at clause positions
    `0`, `1`, `4`, and `5` all stay structurally connected, locally
    admissible, and same-primary `103 / 8` / `115657 / 21112` on top of the
    current live claim clause-`2` + anchor-`11` exact-argument pocket
  - the clause-`4` `demo_sharp_codomain` and `demo_sharp_bridge` variants now
    regain historical reanchor only on that exact side pocket, while the clause-`0`,
    clause-`1`, and clause-`5` variants still stay outside historical
    reanchor there and still lose only on higher bit cost `243`, `245`, or
    `250`
  - a non-landed raw position-`0` reland probe briefly lifted the local
    step-`15` generated surface to `4285`, but it also reopened the repaired
    clause-`0` zero-admitted capture to `2835` families / `891`
    reanchor-prefix misses, so that broad catalog reland was reverted
  - a newer executable
    `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_still_fence_unsafe_lifted_terminals`
    regression now sharpens that same opening further:
    every omitted demo-only side variant around the exact anchor-`11` pocket
    still stays structurally connected but outside historical reanchor once
    the terminal lifts, so both unsafe `89 / 8` lifted terminals remain
    fenced there
  - that guidance is now partly consumed:
    the clause-`5` `demo_sharp_domain` opening has since been isolated onto
    the existing clause-`4` anchor-`11` side pocket, while the raw
    position-`0` temporal-shell catalog still stays out
- The older raw isolated late-side reland probes are now bounded more
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
    landed repair can no longer be another raw isolated clause-`4` reland or
    a raw isolated clause-`5` reland either; any further gain will need
    additional qualifier / reanchor evidence or residual-incumbent work that
    keeps the captured clause-`2` / clause-`3` surface fenced
- Two new non-landed local step-`13` probes now sharpen the safe search space:
  - widening only operator-bundle formation positions `1` and `4` to
    demo-like variants lifts the repaired local read to
    `[3,5,3,3,5,1,1]` / raw `675` / generated `2223`, but it changes the
    accepted late path to `45 / 7 -> 61 / 9`
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
- Those probes were reverted. They are diagnosis, not landed repairs, and the
  position-`1` / position-`4` reland plus the
  position-`0` / position-`4` / position-`5` / position-`6` reland are now
  both covered by executable regressions instead of doc-only memory.

## Optimization Thesis

The next cycle should spend engineering time on the residual stored step-`15`
gap on canonical `v11` while keeping step `1` explicit, not on another rerun
setup pass or another round of step-`13` theory first.

The highest-value work is:

1. keep the current step-`11` / step-`12` / step-`13..15` local guardrails
   green on the repaired chain
2. keep the stored step-`13` hit, the new `v11` stored diagnosis, and step
   `1` explicit on the checklist
3. use the `v11` certificate and `step-15-live.ndjson` to land the next
   parity-preserving local step-`15` repair against the current post-relief
   `476`-bar / `3`-incumbent surface, keeping the new `4088` guardrail
   stable, treating the raw position-`0` probe as the explicit "do not reland
   this globally" boundary, and treating clause `4` plus the local
   clause-`5` `demo_sharp_domain` and `demo_flat_codomain` openings as
   already consumed by a fenced tiny side pocket so the next gain now comes
   from the residual `single`-bucket incumbent pressure rather than from the
   old `small_cluster` summary wall; the new proof-close incumbent freeze now
   further localizes that next move to three fenced same-primary `103 / 8`
   prefix families first diverging at clause positions `0`, `2`, and `5`, and
   the newer strict-subset same-primary probe now proves the next repair must
   stay narrower than proof-close same-primary relief even on any strict
   subset of those same families, because those probes already only unfence
   same-primary `103 / 8`, bit-cost-`236` reference-terminal completions
4. launch the next clean full-profile rerun only after that next local repair
   is regression-backed
5. refresh compare / benchmark / certification on the next stored bundle

Treat `v11` plus its stored audit bundle as the current canonical guardrail.
Keep the replay harness corpus and benchmark inputs frozen until real stored
behavior changes.

## Decision Rules

- Trust stored artifacts over terminal impressions.
- Treat clean-tree completed `v11` as the canonical stored claim bundle until a
  newer parity-and-breadth candidate exists.
- Require targeted claim regressions plus replay-harness parity before any new
  full-profile rerun.
- Keep step `1` explicit as the separate early breadth blocker and repair the
  remaining stored late-floor miss at step `15` before spending the next cycle
  on broader late-step theory or another rerun that does not close it.
- Prefer narrow, regression-backed fixes over broad frontier rewrites.
- Do not reland the exploratory `[3,5,3,3,5,1,1]` or `[5,1,3,3,5,3,3]`
  step-`13` widenings directly; both still leave accepted-hash parity open.
- Do not accept a "fix" that only wakes guarded, replay, realistic-shadow, or
  demo-only behavior.
- Keep user-facing and paper-facing wording at `bounded live recovery` until a
  passing certificate exists.

## Active Phase: Stored Step-`15` Breadth Repair

Goal:

- turn the parity-clean-but-breadth-failing `v11` lane into the next
  rerun-ready
  candidate by repairing the remaining stored late-floor miss at step `15`
  while preserving accepted-hash parity through step `15` and keeping stored
  step `13` closed

Loop:

1. keep the stored `v11` compare / certification / benchmark regressions green
2. keep the local step-`11` breadth and step-`12` selector guardrails green
3. keep the current step-`13` / step-`14` / step-`15` canonical guardrails
   green until a replacement is explicitly proved
4. use the stored `v11` certificate and `step-15-live.ndjson` first
5. land a parity-preserving next step-`15` repair
6. rerun targeted claim tests plus replay parity on that repair
7. launch the next clean full-profile rerun
8. refresh compare / benchmark / certification on that new stored bundle
9. only treat certification as newly in reach if the rerun keeps step-`15`
   completion while closing the remaining breadth failures

Current slice order:

1. hold the stored step-`13` hit, the stored `v11` diagnosis freeze, and the
   frozen negative controls green
2. use the stored `v11` certificate and live checkpoints to localize the next
   step-`15` repair against the remaining local `476` partial-prefix bar
   failures and `3` incumbent-dominance prunes, keeping the isolated
   `single` survivor bucket fenced, keeping the newly reopened clause-`4`
   side pocket fenced as tiny and noncanonical, and working against the
   residual `single`-bucket pressure without broadening the raw
   position-`0` catalog; keep
   `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_pin_the_exact_claim_family_labels`
   green so that the exact residual surface stays frozen as clause-`0`
   `claim_flat_domain`, clause-`2` `claim_flat_domain` plus the anchor-`11`
   exact-argument pocket, and clause-`5` `claim_flat_codomain`; keep
   `current_claim_step_fifteen_residual_single_bucket_incumbent_families_still_hide_two_unsafe_lifted_terminals`
   green too so the next step-`15` repair still respects that each of those
   three families sits beside two unsafe stronger-than-canonical `89 / 8`
   lifted terminals, keep
   `current_claim_step_fifteen_exact_family_same_primary_relief_still_unfences_the_isolated_single_pocket`
   green too so the next step-`15` repair still respects that even
   exact-family same-primary relief collapses `3 -> 0` incumbent-dominance
   prunes only by unfencing the isolated `single` pocket from `1` to `4`
   fully scored non-winning terminals while leaving the `small_cluster`
   unchanged at `3012 / 502 / 502 / 0`, keep
   `current_claim_step_fifteen_subset_local_same_primary_relief_only_trades_single_prunes_for_non_winners`
   green too so the next step-`15` repair still respects that even any strict
   subset of those same families only trades `n` incumbent prunes for `n`
   extra fully scored non-winning reference-terminal completions while keeping
   `4088` / `476` and the cleared `small_cluster` unchanged, and keep
   `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_still_fence_unsafe_lifted_terminals`
   green so any future omitted-side opening stays reference-terminal-local
   instead of reopening that lifted shell
3. rerun only after that next local repair is regression-backed
4. revisit stored step `1` from the next evidence bundle once step `15` moves

Do not reopen first:

- another fresh step-`13` widening theory pass before the rerun consumes the
  next landed local step-`15` repair
- a `resume`-based restart of stopped `v4`
- another runtime-only step-`4` micro-optimization slice first
- another stored/local step-`11` rerun first
- a raw reland of the exploratory `[3,5,3,3,5,1,1]` or
  `[5,1,3,3,5,3,3]` step-`13` widenings
- a raw global position-`0` temporal-shell reland on step `15`
- replay-fixture recapture or benchmark-file churn first
- stronger wording or runtime-threshold freeze before a passing certificate
  exists

## Phase 2: Re-Earn One Full-Profile Claim Run

Goal:

- produce one new stored full-profile bundle beyond `v11` that consumes the
  remaining stored breadth repair

Required output:

- one canonical run directory from the disclosed desktop
- full-profile completion through step `15`
- accepted-hash parity through step `15`
- early and late breadth floors passed from stored evidence
- complete reason, prune, narrative, and manifest surfaces preserved

## Phase 3: Freeze Signoff Artifacts

Goal:

- turn the repaired bundle into the auditable claim signoff surface

Required output:

- one compare report against the guarded reference
- one benchmark bundle
- one passing `claim_certificate.json`
- one certified runtime threshold tied to stored evidence

## Phase 4: Open The Language Gate

Goal:

- allow stronger wording only after the technical gate is closed

Required output:

- user-facing and paper-facing wording updated only after certification passes
- stronger sentence tied explicitly to the stored claim certificate and
  disclosed desktop bundle

## Non-Goals Until The Step-13 Repair Is Real

- another runtime-only slice first
- another broad later-step band-expansion slice
- another raw step-`13` exploratory widening reland
- metadata-only cleanup in place of breadth repair
- stronger user-facing language before a passing certificate exists

## Success Condition

This plan is done only when one stored `desktop_claim_shadow` bundle from the
disclosed desktop shows all of the following at the same time:

- full-profile completion through step `15`
- no silent guarded, replay, realistic-shadow, or demo-only fallback
- accepted parity through step `15`
- early and late breadth gates passed from stored evidence
- the accepted path is canonical in ordinal order from step `9` through
  step `15`, and any remaining breadth repair is earned on that path
- complete reason, prune, narrative, and manifest data
- passing compare, benchmark, and certification outputs
