# Autonomous Claim Lane Checklist

Last updated: 2026-04-06

This checklist tracks only work that is still open for `desktop_claim_shadow`.

## Keep Explicit While Working

- Canonical stored baseline is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v12`.
- Stored parity is earned; stored breadth is not.
- Refreshed compare and benchmark already exist beside `v12`.
- Refreshed `v12` certification still fails only on breadth:
  - step `1 = 546 / 2144`
  - step `15 = 4331 / 5000`
- The current clean canonical late surface is:
  - step `13 = 2320`
  - step `14 = 12027`
  - step `15 = 4331`
- The remaining clean step-`15` pressure is:
  - partial-prefix bar failures `553`
  - incumbent-dominance prunes `3`
  - `small_cluster = 3132 / 522 / 522 / 0`
  - fenced `single` bucket = `1` fully scored non-winner plus `3` residual prunes
- The executable partial-prefix wall split is now explicit:
  - remaining-two prefixes `451`
  - remaining-three prefixes `102`
  - first mismatch positions `clause 0 = 312`, `clause 1 = 177`,
    `clause 2 = 50`, `clause 3 = 14`
  - dominant remaining-two slice `clause 0 = 252`, `clause 1 = 145`
- The dominant remaining-two clause-`0` / clause-`1` slice is now explicit:
  - mismatch `0` is six exact `42`-count pairings:
    clause-`0` `claim_flat_domain` or `claim_eventual_domain`
    crossed with clause-`1` `reference`, `claim_sharp_codomain`, or
    `claim_next_codomain`
  - mismatch `1` is clause-`0` `reference` with clause-`1`
    `claim_sharp_codomain = 42`, `claim_next_codomain = 42`,
    `demo_flat_codomain = 61`
  - the remaining-two mismatch-`2` / mismatch-`3` tail stays only the narrow
    `reference/reference` continuation at `42` and `12`
- The dominant remaining-two wall is now explicit one layer deeper too:
  - mismatch `0` stays only on clause-`4` `claim_next_bridge` and clause-`4`
    `reference`, with the clause-`4` `claim_next_bridge` side larger at
    `48 / 48 / 48` versus `36 / 36 / 36` across clause-`5`
    `claim_flat_codomain`, `claim_next_codomain`, and `reference`
  - mismatch `1` stays on the same clause-`4` / clause-`5` grid at
    `27 / 27 / 27` and `22 / 22 / 20`
  - the old demo-only clause-`4` bridge pockets survive only on the smaller
    mismatch-`2` tail, so they are not the dominant live blocker anymore
- The dominant remaining-two wall is now explicit per dominant pairing too:
  - every mismatch `0` clause-`0` / clause-`1` pairing keeps the same
    clause-`4` split, `claim_next_bridge = 24` versus `reference = 18`
  - mismatch `1` keeps that same `24 / 18` split on
    `reference + claim_sharp_codomain` and `reference + claim_next_codomain`,
    while the larger `reference + demo_flat_codomain` side still stays on the
    same live clause-`4` claim families at `33 / 28`
  - the mismatch-`2` tail is the only place the old clause-`4` demo pockets
    still appear, at `18 / 4 / 4 / 16`
- The dominant remaining-two wall is now explicit across clause `2` too:
  - the regular mismatch-`0` / mismatch-`1` pairings stay on
    `claim_flat_domain = 15`, `claim_sharp_codomain = 15`,
    `reference = 12`
  - the larger `reference + demo_flat_codomain` side stays at `23 / 23 / 15`
  - the mismatch-`2` tail stays only on the two claim clause-`2` variants at
    `21 / 21`, while mismatch-`3` stays only `reference = 12`
- A narrower `reference + demo_flat_codomain` tradeoff control is now
  explicit too:
  - `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_reference_clause_zero_live_claim_bridge_surface_stays_a_tradeoff_control`
  - it lifted local step `15` generated breadth to `4523`
  - it narrowed the clean partial-prefix wall to `537` and zero-admitted
    capture to `2223`
  - it shrank the mismatch `1` `reference + demo_flat_codomain` branch from
    `61` captured prefixes on clause-`4` `33 / 28` down to `45` on
    clause-`4` `27 / 18`
  - it kept the isolated `single` pocket and residual `3` incumbent prunes
    unchanged
  - but it widened `small_cluster` to `3324 / 554 / 554 / 0`, so it is not
    the landed repair
- A reverted clause-`1` `demo_eventually_codomain` exact-pocket probe is now
  also part of the live diagnosis and is pinned by
  `current_claim_step_fifteen_clause_one_demo_eventually_codomain_exact_pocket_reland_stays_a_negative_control`:
  - it lifted local step `15` generated breadth to `4466`
  - it widened `small_cluster` to `3156 / 526 / 526 / 0`
  - it kept the isolated `single` pocket and residual `3` incumbent prunes
    unchanged
  - but it widened the partial-prefix wall to `626`, so it is not the landed
    repair
- A reverted broader clause-`0` `claim_flat_domain` plus clause-`1`
  `demo_flat_codomain` exact-pocket reland is now also part of the live
  diagnosis and is pinned by
  `current_claim_step_fifteen_clause_one_demo_flat_codomain_on_clause_zero_claim_flat_exact_pocket_reland_stays_a_negative_control`:
  - it also lifted local step `15` generated breadth to `4466`
  - it kept the isolated `single` pocket and residual `3` incumbent prunes
    unchanged
  - but it also widened the partial-prefix wall to `626`, so it is not the
    landed repair
- A reverted clause-`3` `anchor-11` exact-argument widening onto the broader
  clause-`0` / clause-`1` claim surface while clause `2` stayed `reference`
  is now also part of the live diagnosis:
  - it left the clean partial-prefix wall pinned at `553`
  - it left the executable remaining-two nine-pair split unchanged
  - but it reopened `72` summary-stage incumbent captures, so it is also not
    the landed repair
- A reverted clause-`5` side-pocket broadening onto the claim-safe
  clause-`0` / clause-`1` surface is now also part of the live diagnosis and
  is pinned by
  `current_claim_step_fifteen_clause_five_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`:
  - it lifted local step `15` generated breadth to `4779`
  - it widened `small_cluster` to `3516 / 586 / 586 / 0`
  - it kept the isolated `single` pocket and residual `3` incumbent prunes
    unchanged
  - but it widened the partial-prefix wall to `585`, so it is not the landed
    repair
- Reverted clause-`4` claim-safe reopenings are now also part of the live
  diagnosis:
  - the broad clause-`4` `demo_sharp_codomain` plus `demo_sharp_bridge`
    reopening is pinned by
    `current_claim_step_fifteen_clause_four_side_pocket_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`:
    it lifted local step `15` generated breadth to `4843`, widened
    `small_cluster` to `3516 / 586 / 586 / 0`, kept the isolated `single`
    pocket and residual `3` incumbent prunes unchanged, but widened the
    partial-prefix wall to `617`, so it is not the landed repair
  - the narrower clause-`4` `demo_sharp_codomain`-only and
    `demo_sharp_bridge`-only reopenings are pinned by
    `current_claim_step_fifteen_clause_four_sharp_codomain_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`
    and
    `current_claim_step_fifteen_clause_four_sharp_bridge_on_claim_safe_clause_zero_one_surface_stays_a_negative_control`:
    each lifted local step `15` generated breadth only to `4587`, widened
    `small_cluster` to `3324 / 554 / 554 / 0`, kept the isolated `single`
    pocket and residual `3` incumbent prunes unchanged, but still widened the
    partial-prefix wall to `585`, so neither is the landed repair

## Late-Step Gate

- [ ] Narrow clean stored step-`15` partial-prefix bar failures below `553` on
      top of canonical `v12`.
- [ ] Keep the accepted step-`15` winner fixed at the canonical `103 / 8`.
- [ ] Keep the isolated `single` pocket fenced.
- [ ] Keep both unsafe lifted `89 / 8` terminals fenced.

## Stored Evidence Gate

- [ ] Re-earn stored step `15 >= 5000` without losing accepted-hash parity.
- [ ] Restore stored step `1` generated count to exactly `2144`.
- [ ] Keep `full_telescopes_evaluated` within a certified moderate threshold.

## Audit Gate

- [ ] Store one passing `claim_certificate.json`.

## Language Gate

- [ ] Keep user-facing and paper-facing wording at `bounded live recovery`
      until the certification gate passes.
- [ ] Do not use `unguided` user-facing before a passing certificate exists.
- [ ] Update any stronger sentence only after certification passes.
- [ ] Tie any stronger sentence directly to the stored certificate and
      disclosed desktop bundle.
