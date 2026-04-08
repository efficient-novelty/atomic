# Autonomous Claim Lane Plan

Last updated: 2026-04-08
Status: active

This file owns the medium-horizon path from the current canonical claim bundle
to final signoff.

## Contract

- Keep phases, dependencies, and exit criteria here.
- Do not store live counters here. Use
  [autonomous_progress.md](autonomous_progress.md).
- Do not store the exact current work order here. Use
  [autonomous_next_steps.md](autonomous_next_steps.md).
- Do not store probe history here. Use
  [autonomous_ledger.md](autonomous_ledger.md).

## Goal

Produce one stored `desktop_claim_shadow` bundle that:

- completes through step `15`
- preserves accepted-hash parity through step `15`
- passes compare, benchmark, and certification from stored evidence

Until then, wording stays at `bounded live recovery`.

## Phase 1: Land One Step-15 Repair

Goal:

- improve the clean step-`15` miss on top of canonical `v12`

Must keep true:

- accepted step `15` winner stays canonical `103 / 8`
- the isolated `single` pocket stays fenced
- `small_cluster` does not regress
- stronger-than-canonical lifted `89 / 8` terminals do not become live

Do not use:

- blanket, exact-family, or subset-local same-primary relief
- raw lifted-shell relands
- raw clause-`3` widening
- claim-safe clause-`4` or clause-`5` reopenings
- exact remaining-two mismatch-`0` or mismatch-`1` clause-`5`
  bridge-slice reopenings
- a rerun-first or step-`1`-first pass

Exit criterion:

- one focused local repair is regression-backed and clearly better than the
  current `4331 / 553` local surface

## Phase 2: Re-Earn Stored Breadth

Goal:

- consume the landed local repair in one new clean full-profile bundle beyond
  `v12`

Required output:

- one canonical stored run beyond `v12`
- full-profile completion through step `15`
- accepted-hash parity preserved through step `15`
- refreshed compare, benchmark, and certification outputs

Exit criterion:

- stored step `15` either passes or the new stored miss is narrower and
  cleaner than `v12`

## Phase 3: Close Remaining Breadth Gates

Goal:

- close the remaining stored breadth blockers

Open gates:

- step `15 >= 5000`
- step `1 = 2144`
- `full_telescopes_evaluated` stays within a certified moderate threshold

Decision rule:

- keep step `1` explicit, but do not reopen it before the next stored
  step-`15` rerun unless that rerun itself changes the diagnosis

## Phase 4: Certification

Goal:

- store one passing certification surface

Required output:

- refreshed compare report
- refreshed benchmark bundle
- passing `claim_certificate.json`

Exit criterion:

- another reviewer can audit the lane from one stable stored bundle

## Phase 5: Language Gate

Goal:

- allow stronger wording only after the evidence gate closes

Rules:

- keep user-facing and paper-facing wording at `bounded live recovery` until
  certification passes
- do not use `unguided` user-facing before a passing certificate exists
- tie any stronger sentence directly to the stored certificate and disclosed
  desktop bundle

## Operating Rules

- Trust stored evidence over terminal impressions.
- Treat `v12` as canonical until a newer stored bundle clearly replaces it.
- Keep live numbers in [autonomous_progress.md](autonomous_progress.md).
- Keep current execution instructions in
  [autonomous_next_steps.md](autonomous_next_steps.md).
- Keep binary gates in [autonomous_checklist.md](autonomous_checklist.md).
- Keep experiment history in [autonomous_ledger.md](autonomous_ledger.md).
