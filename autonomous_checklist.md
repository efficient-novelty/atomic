# Autonomous Claim Lane Checklist

Last updated: 2026-04-02

This checklist is the live signoff gate for `desktop_claim_shadow`.
It lists only work that is still open.

## Current Open Read

- The claim lane still does not have a signoff-ready full-profile bundle.
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
- The latest full-profile run,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v3`,
  answered the old runtime question by reaching step `14`, but it failed there
  with `no atomic candidates were generated for step 14`.
- Steps `10` through `13` already diverge from reference replay on stored
  evidence. The stored run opened step `14` at `clause_kappa = 7` with
  `raw_catalog_clause_widths = [3,1,1,1,1,1,1]`, but the current reproducer
  now promotes that divergent prefix to `claim_band = 9..9`, enqueues one
  root, reaches exact terminal-summary work, and still dies with exact
  partial-prefix pruning.
- The full `21`-prune remaining-one exact sweep is now explained on test
  evidence:
  - `9` prefixes keep the honest `40/9` ceiling with `3` admitted candidates
  - `2` prefixes admit zero terminal candidates and retain no cached bound
  - `10` prefixes keep an honest later `41/9` ceiling with `3` admitted
    candidates
- A new hybrid cutover regression now proves that the zero-admitted family is
  not the blocker:
  - the reference step-`14` winner already coexists with `54` zero-admitted
    remaining-one prunes
  - step `13` is the first divergence that flips step `14` into failure
  - that cutover adds `27` admitted `kappa = 9` prunes at `50/9` and `51/9`
- A new step-`13` structural-delta regression now identifies that cutover:
  - the reference step-`13` winner closes operator-bundle debt with a
    metric-bearing `kappa = 7`, `nu = 46`, `lib_refs = {11,12}` shell
  - the step-`13`-only cutover swaps in a non-metric
    `kappa = 3`, `nu = 29`, `lib_refs = {11}` shell
  - step `14` therefore reopens operator-bundle claim debt instead of opening
    the reference Hilbert-functional package
  - the admitted `50/9` and `51/9` families localize to that reopened
    operator-bundle surface
- The current blocker is late-step claim viability/correctness, not inability
  to escape step `4`.

## 1. Runtime Improvement Loop

- [ ] Use the landed step-`13` structural-delta regression to decide whether
      the narrow honest fix belongs in backtracking the divergent accepted
      step-`13` shell or in tightening the reopened operator-bundle step-`14`
      surface / exact-screen path it unlocks.
- [ ] Require every serious runtime slice to prove replay-harness parity plus
      either fewer exact-`nu` evaluations or lower measured aggregation time
      before another full-profile rerun.
- [ ] Finish one intended-profile claim run through step `15`.
- [ ] Explain observed RSS versus governor-accounted RSS honestly from stored
      step artifacts.

Done when:

- one intended-profile claim run completes through step `15`
- the stored artifacts honestly explain both the runtime and late-step failure
  story

## 2. Stored Claim Evidence

- [ ] Preserve accepted-hash parity through step `15`.
- [ ] Restore step `1` generated raw count to exactly `2144`.
- [ ] Re-earn the required generated-count floors from stored evidence:
      step `10 >= 500`, step `11 >= 800`, step `12 >= 1200`,
      step `13 >= 2200`, step `14 >= 3500`, step `15 >= 5000`.
- [ ] Ensure early and late breadth claims come from stored generated counts,
      not from config intent.
- [ ] Keep `full_telescopes_evaluated` within a certified moderate threshold.
- [ ] Persist complete `exact_screen_reason_counts`.
- [ ] Persist complete `prune_class_counts`.
- [ ] Ensure late-step summaries expose generated, hard-admissible,
      exact-screened, exact-screen-pruned by reason, heuristic-dropped, and
      fully-evaluated totals.
- [ ] Make guarded, replay, realistic-shadow, or demo-only fallback impossible
      to miss in claim artifacts.

Done when:

- parity and breadth both pass from stored claim evidence
- a reviewer can understand the live claim lane from the stored bundle alone

## 3. Compare, Benchmark, And Certification

- [ ] Store one canonical claim-lane run directory from the disclosed desktop.
- [ ] Store one compare report against the guarded reference for that run.
- [ ] Store one benchmark bundle for that same run.
- [ ] Record benchmark median wall time.
- [ ] Record benchmark p90 wall time.
- [ ] Record benchmark max wall time.
- [ ] Record parity success count.
- [ ] Record floor-hit count.
- [ ] Record manifest snapshot alongside the benchmark outputs.
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
