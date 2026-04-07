# Autonomous Claim Lane Plan

Last updated: 2026-04-07
Status: active

This file is the staged path from the current `v12` state to final signoff.
It is deliberately operational rather than historical.

## Goal

Produce one stored `desktop_claim_shadow` bundle that:

- completes through step `15`
- preserves accepted-hash parity through step `15`
- passes compare, benchmark, and certification from stored evidence

Until then, wording stays at `bounded live recovery`.

## Current Position

- Canonical stored evidence is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v12`.
- Stored parity is already earned.
- Stored compare is ready and the refreshed benchmark now covers `v11` plus
  `v12`.
- Stored certification still fails only on breadth:
  - step `1 = 546 / 2144`
  - step `15 = 4331 / 5000`
- The clean stored late surface and the local guardrail now match:
  - step `13 = 2320`
  - step `14 = 12027`
  - step `15 = 4331`
- The remaining local blocker is no longer the old broad `small_cluster`
  incumbent wall. It is the clean step-`15` residual of `553`
  partial-prefix bar failures plus `3` fenced `single`-bucket incumbent
  prunes.
- The exact clause-`5` `reference` follow-on on the clause-`4` `reference`
  remaining-three tail is now pinned as a neutral control on the canonical
  `4331 / 553 / 2271` surface.
- The exact clause-`5` `claim_flat_codomain` and `claim_next_codomain`
  single-family follow-ons on that same tail are now both pinned as symmetric
  smaller tradeoff controls at `4355 / 551 / 2265`, with the larger
  `reference + demo_flat_codomain` branch only shrinking to
  clause-`4` `33 / 26` / `59` while `small_cluster` still widens to
  `3156 / 526 / 526 / 0`.
- Together those two single-family probes now show the clause-`4`
  `reference` remaining-three tail is exhausted as a cleaner local repair
  target, so Phase 1 should step back up to the broader dominant
  clause-`0` / clause-`1` claim surface outside that tail before another
  rerun.

## Phase 1: Narrow The Remaining Step-15 Wall

Goal:

- improve the clean step-`15` late-surface miss on top of `v12` without
  changing the canonical accepted path

Must keep true:

- accepted step `15` winner stays canonical `103 / 8`
- isolated `single` pocket stays fenced
- `small_cluster` does not regress
- unsafe lifted `89 / 8` terminals do not become live

Do not use:

- blanket same-primary relief
- exact-family same-primary relief
- subset-local same-primary relief
- raw lifted-shell relands
- claim-safe clause-`4` `demo_sharp_codomain` or `demo_sharp_bridge`
  reopenings, whether single-side or combined
- raw position-`0`, broad clause-`4`, or broad clause-`5` relands
- raw step-`13` widened controls

Exit criterion:

- one focused local repair is regression-backed and sets up the next clean
  rerun beyond `v12`

## Phase 2: Re-Earn Stored Breadth

Goal:

- run one new clean full-profile bundle that consumes the next landed local
  repair

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

- close the breadth blockers from stored evidence

Open gates:

- step `15 >= 5000`
- step `1 = 2144`
- runtime remains within a certified moderate threshold

Decision rule:

- keep step `1` explicit, but do not reopen it before the next stored
  step-`15` rerun unless that rerun itself changes the diagnosis

## Phase 4: Certification

Goal:

- produce one passing stored certification surface

Required output:

- ready compare report
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

## Working Rules

- Trust stored evidence over terminal impressions.
- Treat `v12` as canonical until a newer stored bundle clearly replaces it.
- Prefer narrow, regression-backed fixes over broader rewrites.
- Do not spend another cycle on rerun setup before the next local step-`15`
  repair is real.
- Do not reopen already-closed step-`11` through step-`14` work unless a new
  regression forces it.

## Success Condition

This plan is complete only when one stored `desktop_claim_shadow` bundle shows
all of the following at once:

- full-profile completion through step `15`
- accepted-hash parity through step `15`
- breadth gates passed from stored evidence
- compare, benchmark, and certification all pass
- the accepted late path is still canonical
- the evidence is strong enough to support stronger wording
