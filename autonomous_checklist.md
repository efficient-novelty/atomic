# Autonomous Claim Lane Checklist

Last updated: 2026-04-03

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready certified bundle.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current short-loop gate is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`.
- The current later-wall step-`4` continuation winner through `576` is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`.
- The current corroborating middle-wall read through `335` is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v2`.
- The older farthest stored step-`4` continuation stop remains
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
  at `1095` explored prefixes.
- A fresh clean-start full-profile rerun,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v5`,
  now completes through step `15` on clean-tree repo head
  `c1fbb51d4fc9a620cd2ce95c9c3eadfe1a54fc65` with release binary hash
  `c42758f96c8171900651503d7f2a0ffe9915966c41edea98d8f1e296fc772a4e`.
- A compare report, claim certificate, and benchmark bundle now also exist
  beside that `v5` run and make the remaining signoff gaps explicit.
- Stored `v5` compare / certification / benchmark regression tests now pin
  those current parity and breadth failures in-tree, so the next change has to
  move real stored behavior rather than only the narrative around it.
- Local step-`11` retained-pool and selector repairs are now landed:
  the guarded same-primary completion survives the current claim
  terminal-clause catalog, full remaining-one summary, compact survivor
  sketch, retained candidate pool, and final live selection on the divergent
  history; no stored rerun has consumed that repair yet, so the signoff
  surface is still the frozen `v5` evidence set.
- A follow-up local step-`12` minimality and exact-screen repair now keeps
  the guarded curvature shell alive through preterminal clause exposure, full
  remaining-one summary, compact survivor sketch, semantic minimality, and
  exact remaining-one screening on that repaired live history.
- A second follow-up local step-`12` cache-key repair now also keeps that
  guarded curvature shell alive into the retained claim candidate pool by
  keying exact multi-step partial-prefix bounds on
  `(prefix_signature, clause_kappa)`.
- A third follow-up local step-`12` selector repair now also makes live claim
  step `12` accept the guarded same-primary curvature shell inside the
  repaired `34 / 6` tier.
- All current same-primary step-`12` survivors, including that guarded shell,
  still collapse onto one observed local step-`13..15` path with generated
  counts `9`, `12027`, and `780`, so the closed local step-`12` hash fork does
  not explain the remaining step-`13` / step-`15` floor misses by itself.
- A new local late-surface repair now changes that read:
  - step `13` still opens only `3` raw claim telescopes, exact-prunes
    `2 / 3` roots, and reaches only one `2`-terminal bucket before acceptance;
    the repaired claim-open regression now pins that bottleneck more sharply
    at singleton-heavy raw widths `[3,1,1,1,1,1,1]` on the claim-generic
    band-`7` catalog
  - step `14` no longer shares that thin-path shape locally:
    the widened claim `kappa = 9` catalog now opens `19683` raw telescopes,
    keeps `3` roots alive, and lifts live generated prefixes to `12027`
    before proof-close
  - that widened step-`14` surface now exposes a `4`-way same-primary
    `62 / 9` continuation fork, and live claim acceptance now chooses the one
    same-primary survivor that restores the canonical step-`15`
    `DCT 103 / 8` continuation
  - step `15` on that restored canonical branch still opens `6561` raw
    claim telescopes, but `512` partial-prefix bar failures still cut that
    live generated surface to `780` before proof-close
- The old `v3` step-`14` zero-candidate failure is therefore no longer the
  first blocker.
- The current blocker is stored parity plus stored breadth on the completed
  run:
  - compare says accepted hashes diverge from guarded replay at steps `9`,
    `11`, and `12`
  - step `9` still keeps guarded `nu / kappa` but accepts a different winner
    on the stored `v5` run
  - step `11` still diverges on stored `v5`, but the live local selector now
    picks the guarded same-primary survivor and no stored rerun has consumed
    that repair yet
  - step `12` still drops to `nu = 33`, `kappa = 5` versus guarded `34 / 6`,
    while the repaired live local path now reaches guarded `34 / 6`, retains
    the guarded curvature shell, and now also accepts that guarded shell
  - step `13` and step `14` recover the guarded accepted hashes but stay one
    `nu` low on stored `v5`; locally, step `14` now has a widened
    same-primary repair that preserves the canonical step-`15` continuation
    without a fresh stored rerun yet
  - step `15` returns the guarded `DCT` hash / `nu` / `kappa`
- The stored claim certificate is still `attention` because:
  - accepted-hash parity fails
  - early breadth fails at step `1` (`546` versus target `2144`)
  - late generated floors fail at steps `11..15`
- The stored benchmark bundle says:
  - step-`15` completion count `= 1`
  - runtime `= 408 ms`
  - parity success count `= 0`
  - full early breadth hit count `= 0`
  - full late floor hit count `= 0`
- Claim-policy honesty, fallback honesty, narrative/event completeness,
  exact-screen reason completeness, prune-class completeness, and manifest
  completeness are now all earned on the stored full-profile run.

## 1. Runtime Improvement Loop

- [ ] Require every serious runtime slice to prove replay-harness parity plus
      either fewer exact-`nu` evaluations or lower measured aggregation time
      before another full-profile rerun.
- [ ] Explain observed RSS versus governor-accounted RSS honestly from stored
      step artifacts.

Done when:

- the stored artifacts honestly explain the runtime story and no longer leave
  the claim sentence blocked on missing runtime evidence

## 2. Stored Claim Evidence

- [ ] Preserve accepted-hash parity through step `15`.
- [ ] Restore step `1` generated raw count to exactly `2144`.
- [ ] Re-earn the required generated-count floors from stored evidence:
      step `10 >= 500`, step `11 >= 800`, step `12 >= 1200`,
      step `13 >= 2200`, step `14 >= 3500`, step `15 >= 5000`.
- [ ] Ensure early and late breadth claims come from stored generated counts,
      not from config intent.
- [ ] Keep `full_telescopes_evaluated` within a certified moderate threshold.

Done when:

- parity and breadth both pass from stored claim evidence
- a reviewer can understand the live claim lane from the stored bundle alone

## 3. Compare, Benchmark, And Certification

- [ ] Freeze one certified runtime threshold for the claim sentence.
- [ ] Store one passing `claim_certificate.json`.

Done when:

- another reviewer can audit the whole claim lane from one stable evidence set

## 4. Language Gate

- [ ] Keep user-facing and paper-facing wording at `bounded live recovery`
      until every section above is closed.
- [ ] Do not use the word `unguided` user-facing before the passing
      certificate exists.
- [ ] Update the stronger sentence only after the certification gate passes.
- [ ] Tie the stronger sentence explicitly to the stored claim certificate and
      disclosed desktop bundle.

Done when:

- stronger language appears only after the technical and evidence gates close
