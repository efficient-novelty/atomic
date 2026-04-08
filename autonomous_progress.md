# Autonomous Claim Lane State

Last updated: 2026-04-08

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

Produce one stored `desktop_claim_shadow` bundle that:

- completes through step `15`
- keeps accepted-hash parity through step `15`
- passes compare, benchmark, and certification from stored evidence

Until that exists, wording stays at `bounded live recovery`.

## Canonical Baseline

- Canonical stored bundle:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v12`
- Stored compare is ready.
- Stored benchmark is refreshed across `v11` and `v12`.
- Accepted-hash parity is earned through step `15`.
- Stored certification still fails only on breadth:
  - step `1 = 546 / 2144`
  - step `15 = 4331 / 5000`

## Current Late Surface

- Clean stored and local guardrail chain:
  - step `13 = [5,1,3,3,5,3,2] / 1350 / 2320`
  - step `14 = 62 / 9 / 12027`
  - step `15 = DCT 103 / 8 / 4331`
- Current step-`15` pressure:
  - partial-prefix bar failures: `553`
  - incumbent-dominance prunes: `3`
  - `small_cluster = 3132 / 522 / 522 / 0`
  - fenced `single` bucket = `1` fully scored non-winner plus `3` residual
    prunes at overshoot `115657 / 21112`
- Executable wall split:
  - remaining-two prefixes: `451`
  - remaining-three prefixes: `102`
  - first mismatch positions: clause `0 = 312`, clause `1 = 177`,
    clause `2 = 50`, clause `3 = 14`
  - dominant remaining-two slice: clause `0 = 252`, clause `1 = 145`

## Active Diagnosis

- The old broad `small_cluster` incumbent wall is no longer the primary
  blocker. The dominant live miss is the clean step-`15` partial-prefix wall
  on the canonical `4331` surface.
- The dominant remaining-two live surface is already executable:
  - exact clause-`0` / clause-`1` pairings are frozen by
    `current_claim_step_fifteen_remaining_two_partial_prefix_wall_stays_on_nine_clause_zero_one_pairings`
  - clause-`4` pressure is frozen to `claim_next_bridge` plus `reference`
  - clause-`2` pressure stays on the current claim variants rather than on a
    hidden demo-only reopening
- The best known narrowing tradeoff is still the exact claim-pair plus
  clause-`4` `claim_next_bridge`-side probe at `4445 / 539 / 2241`, but it
  still widens `small_cluster` to `3252 / 542 / 542 / 0`.
- The exact claim-pair clause-`4` `reference`-side relocalization is now also
  isolated and exhausted as a smaller tradeoff control:
  - it lands the same `4379 / 549 / 2259` surface as the older broader
    clause-`4` `reference`-sheet probe
  - it still widens `small_cluster` to `3180 / 530 / 530 / 0`
  - the dominant mismatch-`1` `reference + demo_flat_codomain` branch stays at
    `57` captured prefixes with the same clause-`4` split `33 / 24`
  - its bridge grid matches the broader clause-`4` `reference`-sheet tradeoff
    exactly, so that older tradeoff already lived entirely on the exact claim
    clause-`2` pair rather than on the reference clause-`2` sheet
- The clause-`4` `reference` remaining-three tail is exhausted as a cleaner
  target:
  - exact clause-`5` `reference` is neutral on `4331 / 553 / 2271`
  - exact clause-`5` `claim_flat_codomain` and `claim_next_codomain` are only
    symmetric smaller tradeoff controls at `4355 / 551 / 2265`
- The exact remaining-two mismatch-`1` clause-`5` bridge-slice reopening is
  also ruled out:
  - it lands only `4511 / 571 / 2325`
  - it widens `small_cluster` to `3276 / 546 / 546 / 0`
  - it inflates the three mismatch-`1` pairings to `48 / 48 / 67`
  - it leaves the live clause-`4` mismatch-`1` split unchanged at
    `24 / 18`, `24 / 18`, and `33 / 28`
- The residual `3` `single`-bucket incumbent prunes remain fenced and are now
  secondary to the partial-prefix wall.

## Current Operating Position

- Keep wording at `bounded live recovery`.
- Keep step `1` explicit, but do not reopen it first unless a newer stored
  rerun changes the diagnosis.
- After a real local repair lands, immediately rerun beyond `v12` and refresh
  compare, benchmark, and certification.
