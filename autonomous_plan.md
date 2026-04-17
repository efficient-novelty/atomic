# Autonomous Claim Lane Plan

Last updated: 2026-04-17
Status: post-certification hardening active

This file owns the medium-horizon path from the current canonical claim bundle
through the remaining post-certification quality work.

## Contract

- Keep phases, dependencies, and exit criteria here.
- Do not store live counters here. Use
  [autonomous_progress.md](autonomous_progress.md).
- Do not store the exact current work order here. Use
  [autonomous_next_steps.md](autonomous_next_steps.md).
- Do not store probe history here. Use
  [autonomous_ledger.md](autonomous_ledger.md).

## Goal

Preserve the current certified `desktop_claim_shadow` bundle while improving
the remaining local step-`15` quality gate above it.

## Phase 1: Confirm Post-Probe Stored Baseline

Status:

- complete via canonical `v13`

Completed output:

- one canonical stored run beyond `v12`
- full-profile completion through step `15`
- accepted-hash parity preserved through step `15`
- refreshed compare, benchmark, and certification outputs
- diagnosis reconfirmed on newer code as breadth-only at
  step `1 = 546 / 2144` and step `15 = 4331 / 5000`

## Phase 2: Land One Step-15 Repair

Status:

- complete locally on the current head: the post-reference exact-two-step
  repair is production-active and keeps the repaired
  `7211 / 553 / 2052` late surface while preserving canonical accepted
  `103 / 8`, the fenced `single` pocket, and zero fully scored lifted `89 / 8`

## Phase 3: Re-Earn Stored Breadth

Status:

- complete via stored `v14`: the landed late repair reached stored evidence,
  preserved accepted-hash parity through step `15`, and refreshed native
  compare / benchmark / certification artifacts locally

## Phase 4: Close Remaining Breadth Gates

Status:

- complete via stored `v15`: the first step-`1` follow-on is no longer only
  local evidence; stored step `1` now reaches
  `2144 / 1285 / 1 / 475`
  while stored step `15` keeps the repaired
  `7211 / 553 / 2052` surface intact

## Phase 5: Certification

Status:

- complete via stored `v15`

Completed output:

- refreshed compare report
- refreshed benchmark bundle
- passing `claim_certificate.json`

Exit criterion:

- another reviewer can audit the lane from one stable stored bundle

## Phase 6: Language Gate

Status:

- complete for the certified bundle

Rule that still stays active:

- any stronger sentence must be tied directly to the stored certificate and
  disclosed desktop bundle

## Phase 7: Post-Certification Hardening

Goal:

- improve the remaining clean step-`15` local wall on top of the certified
  stored head

Current status:

- active
- certified stored `v15` now closes the step-`1` gate and passes compare,
  benchmark, and certification from stored evidence
- the remaining open binary gate is the clean step-`15`
  `partial_prefix_bar_failure = 553` wall on current head
- that unchanged wall is now localized below exactly eight queued frontier
  remainder branches, with the direct top-level `reference` remainder at
  `241 = 199 + 42` as the current largest single blocker
- inside that direct `reference` remainder, the next narrowed blocker is the
  mismatch-`1` surface at `177 = 145 + 32`, while the deeper
  reference/reference tail now sits at only `64 = 54 + 10`
- inside the mismatch-`1` `reference / demo_flat_codomain` branch, the next
  exact slice is now narrower too: remaining-two clause-`4`
  `claim_next_bridge = 33`, remaining-two clause-`4` `reference = 28`, and
  the whole remaining-three spill is another `12` captures on clause-`4`
  `reference`
- the tied exact `claim_flat_domain` and exact `claim_sharp_codomain`
  single-sheet reopenings under that `claim_next_bridge = 33` side are now
  frozen as matched smaller tradeoff controls at
  `generated_raw_prefixes = 7317`,
  `partial_prefix_bar_failure = 545`,
  direct `reference` remainder `= 233`,
  `reference / demo_flat_codomain = 51`,
  slot split `30 / 21 / 14`,
  and
  `small_cluster = 2076 / 530 / 530 / 0`
- the exact clause-`2` `reference` sheet under that same
  `claim_next_bridge = 33` side is now frozen too as the first non-widening
  safe side-control at
  `generated_raw_prefixes = 7149`,
  `partial_prefix_bar_failure = 551`,
  direct `reference` remainder `= 239`,
  `reference / demo_flat_codomain = 57`,
  slot split `33 / 24 / 14`,
  and
  `small_cluster = 2040 / 518 / 518 / 0`
- the attempted exact reference-sheet landing candidate is frozen too:
  adding its clause-`4` `reference` companion is completely neutral and
  relands the same
  `7149 / 551 / 2040 / 518 / 518 / 0`
  surface, with the same direct `reference` remainder `239`, the same
  targeted pairing `57`, and the same `33 / 24 / 14` slot split
- the first exact `claim_flat_domain` and exact `claim_sharp_codomain`
  reopenings under that broader clause-`4` `reference` companion are frozen
  too as spill-only tradeoff controls at
  `generated_raw_prefixes = 7236`,
  `partial_prefix_bar_failure = 550`,
  direct `reference` remainder `= 238`,
  `reference / demo_flat_codomain = 57`,
  branch-local slot split `33 / 24 / 13`,
  and
  `small_cluster = 2058 / 524 / 524 / 0`;
  they leave the remaining-two clause-`4` `reference` row pinned at
  `9 / 9 / 6`
  and trim only one remaining-three spill capture to
  `4 / 5 / 4`
  or
  `5 / 4 / 4`
- the next honest move in Phase 7 is therefore no longer another exact
  reference-sheet probe; it is the smaller exact `reference = 6` slice below
  that same broader clause-`4` `reference` companion, rather than revisiting
  the already-frozen pair-level, spill-only claim-sheet, or exact-reference-
  sheet controls

Must keep true:

- accepted step `15` winner stays canonical `103 / 8`
- stored step-`15` stays no worse than
  `7211 / 553 / 2052`
- the isolated `single` pocket stays fenced
- stronger-than-canonical lifted `89 / 8` terminals do not become fully scored
  or live
- the stored evidence path remains refreshable through native `pen-cli`
  commands

Do not use:

- another step-`1`-first theory slice unless stored `v15` regresses
- another storage / evidence rerun with no material code change
- reopening tooling work while native `pen-cli` evidence remains sufficient
- downgrading the certified bundle back to pre-certification language

Exit criterion:

- one focused local repair or tightened diagnosis drops the clean step-`15`
  partial-prefix wall below `553` without regressing the certified `v15`
  surface

## Operating Rules

- Trust stored evidence over terminal impressions.
- Treat `v15` as the current certified stored head and `v14` as the previous
  audited stored head.
- Keep live numbers in [autonomous_progress.md](autonomous_progress.md).
- Keep current execution instructions in
  [autonomous_next_steps.md](autonomous_next_steps.md).
- Keep binary gates in [autonomous_checklist.md](autonomous_checklist.md).
- Keep experiment history in [autonomous_ledger.md](autonomous_ledger.md).
