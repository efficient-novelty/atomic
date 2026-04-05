# Autonomous Claim Lane Checklist

Last updated: 2026-04-06

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready certified bundle.
- The current authoritative stored evidence surface is the completed
  `v11` full-profile run plus its compare, certification, and benchmark
  outputs:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v11`.
- The stored `v11` certificate and frozen `step-15-live.ndjson` provenance
  are now pinned by
  `stored_claim_v11_certificate_and_step_15_live_checkpoint_freeze_current_canonical_diagnosis`,
  and the refreshed benchmark bundle is pinned by
  `stored_claim_v11_benchmark_freezes_runtime_and_floor_counts`,
  so the current open miss anatomy is executable in-tree.
- Stored `v11` now passes accepted-hash parity honestly:
  - trajectory and accepted hashes match guarded replay through step `15`
  - compare signoff is `ready`
  - the claim audit is `ready`
- The transient `v7` rerun first re-earned stored step `11` but reopened
  accepted-hash parity at step `12`; clean-tree `v11` now keeps that repaired
  parity chain while consuming the isolated anchor-`11` step-`15` repair on
  stored evidence.
- Stored `v11` still fails breadth honestly:
  - early breadth still misses at step `1` (`546` versus target `2144`)
  - late generated floors still miss only at step `15` (`3972` versus `5000`)
  - late generated floors now hit at step `10`, step `11`, step `12`,
    step `13`, and step `14`
- Two newer local step-`13` widened probes are now frozen as negative
  controls only:
  `[3,5,3,3,5,1,1]` and `[5,1,3,3,5,3,3]` can reopen the local floor, but
  neither one yet preserves accepted-hash parity through step `14`; both the
  position-`1` / position-`4` reland and the
  position-`0` / position-`4` / position-`5` / position-`6` reland are now
  frozen as executable regressions on the repaired step-`12` chain.
- The stored `v11` certificate now also carries step-level breadth diagnosis
  for the open misses, so step `1` / step `15` catalog widths, root seeding,
  exact-screen pressure, and the full stored step-open pressure signature are
  visible from the bundle itself.
- The current canonical later surfaces are frozen on stored evidence:
  - step `13` reports `[5,1,3,3,5,3,2]` / `1350` / `2320`
  - step `14` reports `19683` / `12027`
  - step `15` now preserves `DCT 103 / 8 / 3972`
- The remaining stored step-`15` miss is now narrower and later-surface-local:
  the canonical `v11` certificate reports `468` partial-prefix bar failures,
  `242` incumbent-dominance prunes, `0`
  legality/connectivity exact rejections, `243` well-formed candidates, and
  `469` exact-bound-screened candidates on that `6561`-catalog temporal-shell
  surface.
- The current exact-screened step-`15` survivor surface is now frozen by
  `current_claim_step_fifteen_survivor_buckets_stay_on_one_small_cluster_plus_one_single_pocket`:
  one library-backed temporal-operator `single` bucket carries the isolated
  non-winning pocket at overshoot `115657 / 21112` plus the residual `3`
  incumbent-dominance prunes, and one library-backed temporal-operator
  `small_cluster` bucket now carries `2964 / 494 / 494 / 0`.
- A newer local step-`15` small-cluster relief is now landed and frozen by
  `current_claim_step_fifteen_small_cluster_relief_clears_summary_prunes_while_three_single_bucket_prunes_remain`:
  the old `246`-candidate summary-stage `small_cluster` incumbent wall is now
  gone, local incumbent-dominance pressure has fallen to `3`, live generated
  prefixes still stay at `4030` with `472` partial-prefix bar failures, and
  the isolated fully scored `single` pocket stays fenced.
- A newer local step-`15` proof-close incumbent regression now freezes that
  remaining fenced pressure more tightly:
  `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_stay_on_three_fenced_prefix_families`
  now pins that the remaining `3` incumbent-dominance prunes are all
  `proof_close_group` captures in the fenced temporal `single` bucket, all
  stay same-primary `103 / 8` non-winners at overshoot `115657 / 21112` and
  bit cost `236`, and first diverge only at clause positions `0`, `2`, and
  `5`.
- A newer exact-family regression now freezes those same residual
  `proof_close_group` captures as three concrete claim-side families:
  `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_pin_the_exact_claim_family_labels`
  now pins clause-`0` `claim_flat_domain`, clause-`2`
  `claim_flat_domain` plus the anchor-`11` exact-argument pocket, and
  clause-`5` `claim_flat_codomain`.
- A newer surface-count regression now sharpens that same residual blocker:
  `current_claim_step_fifteen_residual_single_bucket_incumbent_groups_each_still_carry_a_three_generated_one_admitted_surface`
  now pins that each of those three fenced proof-close families is already
  only a tiny `3`-generated / `1`-admitted / `1`-pruned local surface, so the
  next landed repair still needs to stay family-local rather than reopen
  bucket-global same-primary relief.
- A newer raw-terminal regression now sharpens that same residual blocker
  further:
  `current_claim_step_fifteen_residual_single_bucket_incumbent_families_still_hide_two_unsafe_lifted_terminals`
  now pins that each of those same three fenced families still sits on the
  same raw three-terminal shell, with the reference terminal staying the
  same-primary `103 / 8` non-winner at bit cost `236` while both
  `next_lift` and `eventual_lift` stay connected, locally admissible, and
  stronger-than-canonical `89 / 8` rivals at bit cost `254`.
- A new local omitted-side-variant regression now proves the next safe opening
  is narrower than a raw catalog reland:
  omitted demo-only temporal-shell side variants at clause positions `0`,
  `1`, `4`, and `5` all stay structurally connected, locally admissible, and
  same-primary `103 / 8` / `115657 / 21112` non-winners around the live
  anchor-`11` exact-argument pocket; the clause-`4`
  `demo_sharp_codomain` variant now regains historical reanchor only on that
  exact side pocket, while the clause-`0`, clause-`1`, and clause-`5` side
  variants still stay outside historical reanchor there. A reverted raw
  position-`0` probe that lifted local step `15` only to `4285` also reopened
  the repaired clause-`0` zero-admitted capture to `2835` families / `891`
  reanchor-prefix misses, so the next landed repair must stay isolated onto
  the existing `small_cluster` path instead of broadening the whole
  position-`0` catalog. A newer
  `current_claim_step_fifteen_demo_only_side_variants_around_anchor_eleven_pocket_still_fence_unsafe_lifted_terminals`
  regression now also proves that every one of those omitted side variants
  keeps both unsafe lifted terminals fenced once the terminal moves, so any
  future reland there has to stay reference-terminal-local.
- The older raw isolated late-side probes are no longer the live boundary by
  themselves:
  the clause-`4`-only pocket-gated probe only reached `3980` and reopened the
  zero-admitted capture to `1962` families / `1476` clause-`2`
  reanchor-prefix misses, and the clause-`5`-only pocket-gated probe only
  reached `3974` and still reopened the capture to `1950` families / `1464`
  clause-`2` misses.
- A newer narrower clause-`5` side-pocket reland remains landed locally on top
  of the clause-`4` pocket:
  it lifts the repaired local step-`15` guardrail surface to `4030`,
  keeps the captured zero-admitted surface tightly localized at
  `1956` families / `5868` connected-but-unqualified terminal options, and
  keeps the clause-`4` / clause-`5` side pocket tiny and noncanonical.
- The next landed repair should therefore not be another raw clause-`4`
  reland or another blanket same-primary-retention pass:
  clause `4` plus the local clause-`5` `demo_sharp_domain` opening are now
  consumed as fenced local pockets, the small-cluster summary wall is already
  cleared, and the next gain now needs to target the residual `3`
  single-bucket incumbent prunes on top of the current `4030` local surface,
  starting from those clause-`0` / clause-`2` / clause-`5` fenced prefix
  families rather than reopening the whole side-pocket neighborhood, while
  also keeping the newly frozen unsafe `89 / 8` lifted terminals fenced on
  each of those same family shells.
- No broader per-step claim search-band expansion should land beyond that
  isolated anchor-`11` pocket before a new local repair proves more is
  necessary on this parity-clean stored chain.
- Claim-policy honesty, fallback honesty, narrative/event completeness,
  exact-screen reason completeness, prune-class completeness, manifest
  completeness, and accepted-hash parity are already earned on the stored
  full-profile run.

## 1. Stored Claim Evidence

- [ ] Restore step `1` generated raw count to exactly `2144`.
- [ ] Re-earn the remaining generated-count floor from stored evidence:
      step `15 >= 5000`.
- [ ] Keep `full_telescopes_evaluated` within a certified moderate threshold.

Done when:

- breadth passes from stored claim evidence without losing accepted-hash parity
- a reviewer can understand the live claim lane from the stored bundle alone

## 2. Compare, Benchmark, And Certification

- [ ] Store one passing `claim_certificate.json`.

Done when:

- another reviewer can audit the whole claim lane from one stable evidence set

## 3. Language Gate

- [ ] Keep user-facing and paper-facing wording at `bounded live recovery`
      until every section above is closed.
- [ ] Do not use the word `unguided` user-facing before the passing
      certificate exists.
- [ ] Update the stronger sentence only after the certification gate passes.
- [ ] Tie the stronger sentence explicitly to the stored claim certificate and
      disclosed desktop bundle.

Done when:

- stronger language appears only after the technical and evidence gates close
