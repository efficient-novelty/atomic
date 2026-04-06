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
