# Autonomous Claim Lane State

Last updated: 2026-04-17

This file owns the live operational snapshot for `desktop_claim_shadow`.

## Contract

- Keep canonical run IDs, current counters, the current accepted chain, and the
  current diagnosis here.
- Do not store medium-horizon phases here. Use
  [autonomous_plan.md](autonomous_plan.md).
- Do not store the per-turn work order here. Use
  [autonomous_next_steps.md](autonomous_next_steps.md).
- Do not store binary signoff gates or probe history here. Use
  [autonomous_checklist.md](autonomous_checklist.md) and
  [autonomous_ledger.md](autonomous_ledger.md).

## Mission

Preserve the current certified `desktop_claim_shadow` bundle while improving
local claim-lane quality above it without regressing accepted parity, the
stored late-step repair, or the evidence path that now audits the lane end to
end.

## Stored Head And Audited Baseline

- Previous audited stored head:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v14`
- Current certified stored head:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v15`
- Stored `v15` closes the last pre-certification breadth gate while preserving
  the landed late repair:
  - accepted-hash parity is preserved through all `15` steps
  - stored step `1` is now
    `2144 / 1285 / 1 / 475`
    (`generated / well_formed / admitted / legality_connectivity_exact_rejection`)
  - stored step `15` stays
    `7211 / 5000`
    with `partial_prefix_bar_failure = 553`,
    `incumbent_dominance = 3`,
    `small_cluster = 2052 / 522 / 522 / 0`,
    `broad = 3600 / 0 / 0 / 0`,
    and the same fenced `single` pocket
  - the search-policy metadata and compat surface stay honest for the claim
    lane while the repaired step-`15` claim boundary remains consistent
- Stored `v15` now carries refreshed downstream evidence beside the run via
  native `pen-cli` claim-evidence commands:
  - `claim-compare.{json,txt}` reports `Comparison Signoff: ready`,
    accepted-hash parity through step `15`, and `claim lane audit: ready`
  - `claim_benchmark.{json,txt}` covers `v12`, `v13`, `v14`, and `v15` with
    runtime `min = 3976`, `median = 4165`, `p90 = 4642`, `max = 4642` ms,
    parity success `4 / 4`, early-breadth success `1 / 4`,
    late-floor success `2 / 4`, and runtime-threshold success `4 / 4`
  - `claim_certificate.{json,txt}` reports `Claim Certification: ready` with
    stored runtime `4165 ms`; accepted parity, search-policy honesty,
    fallback honesty, narrative artifacts, early breadth, late generated
    floors, exact-screen completeness, prune-class completeness, and manifest
    completeness all pass
- Direct script execution is still blocked here because no runnable
  `python`, `python3`, `py`, or `uv` launcher exists on `PATH`, but that is
  no longer a claim-lane blocker because native
  `pen-cli compare-claim-lane`, `certify-claim-lane`, and
  `benchmark-claim-lane` refresh the audited stored evidence locally.

## Current Late Surface

- Clean stored and local guardrail chain:
  - step `13 = [5,1,3,3,5,3,2] / 1350 / 2320`
  - step `14 = 62 / 9 / 12027`
  - step `15 = DCT 103 / 8 / 7211` on current head and stored `v15`
- Current step-`15` pressure:
  - partial-prefix bar failures: `553`
  - incumbent-dominance prunes: `3`
  - `small_cluster = 2052 / 522 / 522 / 0`
  - `broad = 3600 / 0 / 0 / 0`
  - isolated `single` bucket = canonical accepted `reference(15)` completion
    at overshoot `115657 / 21112` plus `3` residual proof-close incumbent
    prunes
- Executable wall split:
  - remaining-two prefixes: `451`
  - remaining-three prefixes: `102`
  - first mismatch positions: clause `0 = 312`, clause `1 = 177`,
    clause `2 = 50`, clause `3 = 14`
  - dominant remaining-two slice: clause `0 = 252`, clause `1 = 145`
- Repaired-head frontier-remainder localization is now regression-backed too:
  - all `553` clean partial-prefix captures map to exactly queued branches
    `4 .. 11`; queued branches `0 .. 3` carry `0`
  - branch totals are
    `4 / 5 = 7`,
    `6 / 7 = 19`,
    `8 / 9 = 52`,
    `10 = 156`,
    `11 = 241`
  - the largest single live blocker is the direct top-level `reference`
    remainder at `241 = 199` remaining-two plus `42` remaining-three
  - inside that direct `reference` remainder, the wall now localizes one
    layer deeper to a mismatch-`1` surface at
    `177 = 145` remaining-two plus `32` remaining-three, ahead of only a
    `64 = 54 + 10` reference/reference tail on mismatch positions `2 / 3`
  - the repaired-head direct `reference` remainder keeps the same five
    remaining-two pairings explicit, with the largest single pairing still on
    `reference / demo_flat_codomain = 61`; the sibling
    `reference / claim_next_codomain` and
    `reference / claim_sharp_codomain` pairings stay at `42` each
  - inside that repaired-head `reference / demo_flat_codomain` branch, the
    `61` remaining-two captures now split as
    `33` on clause-`4` `claim_next_bridge` versus
    `28` on clause-`4` `reference`
  - that `claim_next_bridge = 33` side is explicit too:
    clause-`2`
    `claim_flat_domain = 12`,
    `claim_sharp_codomain = 12`,
    and
    `reference = 9`
  - the clause-`4` `reference = 28` companion and the remaining-three spill
    are explicit too:
    remaining-two clause-`4` `reference` stays
    `11 / 11 / 6` across clause-`2`
    `claim_flat_domain / claim_sharp_codomain / reference`,
    while all `12` remaining-three captures stay on clause-`4` `reference`,
    evenly `4 / 4 / 4`
  - the broadest repaired-head code-side reopening under that
    `claim_next_bridge = 33` side is now frozen too:
    the exact claim-variant-pair plus clause-`4` `claim_next_bridge`
    reland lands at
    `generated_raw_prefixes = 7485`,
    `partial_prefix_bar_failure = 539`,
    direct `reference` remainder `= 227`,
    `reference / demo_flat_codomain = 45`,
    slot split `27 / 18 / 14`,
    and
    `small_cluster = 2112 / 542 / 542 / 0`,
    so it remains a tradeoff control rather than the landed repair
  - the two repaired-head exact single-sheet reopenings under that same
    `claim_next_bridge = 33` side are frozen too:
    both exact `claim_flat_domain` and exact `claim_sharp_codomain` relands
    land at
    `generated_raw_prefixes = 7317`,
    `partial_prefix_bar_failure = 545`,
    direct `reference` remainder `= 233`,
    `reference / demo_flat_codomain = 51`,
    slot split `30 / 21 / 14`,
    and
    `small_cluster = 2076 / 530 / 530 / 0`,
    so they remain matched smaller tradeoff controls rather than a landing
  - the repaired-head exact clause-`2` `reference` sheet under that same
    clause-`4` `claim_next_bridge` side is now frozen too:
    it lands at
    `generated_raw_prefixes = 7149`,
    `partial_prefix_bar_failure = 551`,
    direct `reference` remainder `= 239`,
    `reference / demo_flat_codomain = 57`,
    slot split `33 / 24 / 14`,
    and
    `small_cluster = 2040 / 518 / 518 / 0`,
    so it is the first non-widening safe side-control below the repaired head
    even though it leaves the live clause-`4` `claim_next_bridge = 33`
    blocker untouched and trims only the clause-`4` `reference` companion
  - the attempted repaired-head exact reference-sheet landing candidate is now
    frozen too:
    reopening that same exact clause-`2` `reference` sheet together with its
    clause-`4` `reference` companion relands the exact same
    `generated_raw_prefixes = 7149`,
    `partial_prefix_bar_failure = 551`,
    direct `reference` remainder `= 239`,
    `reference / demo_flat_codomain = 57`,
    slot split `33 / 24 / 14`,
    and
    `small_cluster = 2040 / 518 / 518 / 0`
  - that neutral exact reference-sheet extension also pins the safe-control
    grid itself:
    remaining-two clause-`4` `claim_next_bridge` stays
    `12 / 12 / 9`,
    remaining-two clause-`4` `reference` stays
    `9 / 9 / 6`,
    and the remaining-three clause-`4` `reference` spill stays
    `5 / 5 / 4`
    across clause-`2`
    `claim_flat_domain / claim_sharp_codomain / reference`
  - the first repaired-head exact claim-sheet reopenings below that broader
    clause-`4` `reference` companion are now frozen too:
    both exact `claim_flat_domain` and exact `claim_sharp_codomain` relands
    land at
    `generated_raw_prefixes = 7236`,
    `partial_prefix_bar_failure = 550`,
    direct `reference` remainder `= 238`,
    `reference / demo_flat_codomain = 57`,
    and
    `small_cluster = 2058 / 524 / 524 / 0`;
    they leave remaining-two clause-`4` `reference` frozen at
    `9 / 9 / 6`,
    keep the branch-local slot split at
    `33 / 24 / 13`,
    and trim only one remaining-three clause-`4` `reference` capture,
    landing
    `4 / 5 / 4`
    or
    `5 / 4 / 4`
    across clause-`2`
    `claim_flat_domain / claim_sharp_codomain / reference`
  - the top-level `claim_eventual_domain` remainder stays second at
    `156 = 126 + 30`, while the six `claim_flat_domain` descendant branches
    carry the other `156`
- Residual incumbent-dominance pressure is still explicit and localized:
  - the lone fully scored `single`-bucket candidate is the canonical accepted
    `reference(15)` completion on the seven-clause reference prefix at
    `103 / 8` with `bit_kappa = 229` and overshoot `115657 / 21112`
  - the remaining `3` proof-close incumbent prunes are same-primary
    `103 / 8` reference-terminal siblings on
    `clause-0 claim_flat_domain`,
    `clause-2 claim_flat_domain plus anchor-11 exact-argument`, and
    `clause-5 claim_flat_codomain`
  - subset-local same-primary relief there is still not the landed repair; it
    only trades those `3` prunes for extra fully scored non-winning reference
    terminals while leaving the repaired `7211 / 553 / 2052` shell unchanged

## Current Verification

- This turn focused verification on the repaired-head mismatch-`1` branch
  slice below the broader clause-`4` `reference` companion:
  - `cargo test -p pen-type clause_four_reference_side -- --nocapture`
  - `cargo test -p pen-search current_claim_step_fifteen_live_reference_ -- --nocapture`
- All of those tests now pass.
- That repaired-head coverage now also pins
  `current_claim_step_fifteen_live_reference_demo_flat_claim_next_bridge_exact_claim_variant_pair_stays_a_tradeoff_control_on_the_repaired_head`
  ,
  `current_claim_step_fifteen_live_reference_demo_flat_claim_next_bridge_exact_reference_sheet_stays_a_safe_side_control_on_the_repaired_head`,
  `current_claim_step_fifteen_live_reference_demo_flat_exact_reference_sheet_clause_four_reference_companion_stays_neutral_on_the_repaired_head`,
  `current_claim_step_fifteen_live_reference_demo_flat_clause_four_reference_exact_claim_sheets_stay_remaining_three_only_tradeoff_controls_on_the_repaired_head`,
  and
  `current_claim_step_fifteen_live_reference_demo_flat_claim_next_bridge_exact_claim_sheets_stay_matched_smaller_tradeoff_controls_on_the_repaired_head`
  inside the `current_claim_step_fifteen_live_reference_` matrix.
- The new `pen-type` coverage now also pins that matcher scope:
  the exact reference-sheet override still admits only the clause-`2`
  `reference` sheet on the clause-`4` `claim_next_bridge` side, while the new
  clause-`4` `reference`-side exact `claim_flat_domain` and exact
  `claim_sharp_codomain` overrides each admit only their own claim sheet,
  still require the exact anchor-`11` side pocket, keep the live clause-`4`
  `claim_next_bridge` side closed, and remain reference-terminal-only.

## Active Diagnosis

- The main pre-certification objective is now complete:
  - `v15` is the first stored claim bundle on current head that preserves
    accepted-hash parity through step `15`
  - stored step `1` is restored to `2144`
  - stored step `15` keeps the repaired
    `7211 / 553 / 2052` late surface
  - compare, benchmark, and certification all pass from stored evidence
- The remaining open quality target is now purely local:
  - the clean step-`15` partial-prefix wall still sits at `553`
  - the residual proof-close incumbent pressure still sits at `3`
  - that wall did not diffuse after the landed `7211` repair; it now stays
    localized to exactly eight queued frontier remainder branches
  - the largest single live blocker is the direct top-level `reference`
    remainder at `241 = 199 + 42`; the top-level
    `claim_eventual_domain` remainder stays next at `156 = 126 + 30`
  - inside that direct `reference` remainder, the first narrower blocker is
    now explicit too: mismatch `1` alone carries
    `177 = 145 + 32`, while the deeper reference/reference tail carries only
    `64 = 54 + 10`; within the remaining-two slice, the largest single pairing
    stays `reference / demo_flat_codomain = 61`
  - inside that `reference / demo_flat_codomain` branch, the next exact lead
    is now narrower too: remaining-two clause-`4`
    `claim_next_bridge = 33` stays slightly larger than the remaining-two
    clause-`4` `reference = 28`, while the branch's remaining-three spill is
    another `12` captures and stays entirely on clause-`4` `reference`
  - the broadest existing code-side reopening under that
    `claim_next_bridge = 33` side is now frozen too:
    the old exact claim-pair plus clause-`4` `claim_next_bridge`
    reland lowers the clean wall from `553` to `539`, the direct
    `reference` remainder from `241` to `227`, and the targeted
    `reference / demo_flat_codomain` pairing from `61` to `45`, but it also
    widens `generated_raw_prefixes` from `7211` to `7485`,
    widens `small_cluster` from
    `2052 / 522 / 522 / 0` to
    `2112 / 542 / 542 / 0`,
    and grows the remaining-three spill from `12` to `14`
  - the tied exact single-sheet reopenings under that same side are now
    frozen too:
    both exact `claim_flat_domain` and exact `claim_sharp_codomain`
    relands lower the clean wall only to `545`, the direct `reference`
    remainder only to `233`, and the targeted
    `reference / demo_flat_codomain` branch only to `51`, while widening the
    repaired-head slot split to `30 / 21 / 14` and the
    `small_cluster` shell to `2076 / 530 / 530 / 0`
  - the exact clause-`2` `reference` sheet below that same side is now
    frozen too as a safe side-control:
    it lowers the clean wall from `553` to `551`, lowers the direct
    `reference` remainder from `241` to `239`, lowers the targeted
    `reference / demo_flat_codomain` branch from `61` to `57`, and tightens
    the repaired-head shell from
    `2052 / 522 / 522 / 0` to
    `2040 / 518 / 518 / 0`,
    but it does that without touching the live clause-`4`
    `claim_next_bridge = 33` blocker; instead it trims only the clause-`4`
    `reference` companion from `28` to `24` while the remaining-three spill
    widens from `12` to `14`
  - the attempted exact reference-sheet landing path is now closed too:
    reopening that same clause-`2` `reference` sheet together with its
    clause-`4` `reference` companion is completely neutral on the repaired
    head, relanding the same
    `7149 / 551 / 2040 / 518 / 518 / 0`
    surface, the same direct `reference` remainder `239`, the same targeted
    `reference / demo_flat_codomain` pairing `57`, and the same
    `33 / 24 / 14` slot split
  - the first exact claim-sheet reopenings under that broader clause-`4`
    `reference` companion are now frozen too as spill-only tradeoff controls:
    both exact `claim_flat_domain` and exact `claim_sharp_codomain` relands
    land at
    `7236 / 550 / 2058 / 524 / 524 / 0`,
    direct `reference` remainder `238`,
    targeted pairing `57`,
    and branch-local slot split
    `33 / 24 / 13`,
    but they leave the remaining-two clause-`4` `reference` row untouched at
    `9 / 9 / 6`
    and trim only one remaining-three spill capture, landing
    `4 / 5 / 4`
    or
    `5 / 4 / 4`
  - consequence: do not reopen step-`1`, stored-evidence, or tooling work
    unless a later change regresses `v15`; the exact reference-sheet family is
    still the last safe inert control below the repaired head, and the tied
    clause-`2` claim-sheet reopenings under the broader clause-`4`
    `reference` companion are now frozen as spill-only tradeoff controls, so
    the next honest move is below them on the smaller exact
    `reference = 6` slice before the whole remaining-three spill, another
    frontier-wide scout, another whole-branch rerun, or the wall-free queued
    branches `0 .. 3`
- Language status changed with the new stored certificate:
  - the lane no longer needs blanket `bounded live recovery` wording for the
    certified bundle itself
  - any stronger wording must still be tied directly to the stored `v15`
    certificate and disclosed desktop bundle
