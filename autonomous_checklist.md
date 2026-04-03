# Autonomous Claim Lane Checklist

Last updated: 2026-04-03

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
- The accepted claim path is still not replay-parity correct on stored
  evidence:
  - steps `1..7` still match guarded `nu / kappa`
  - step `8` is the first visible `nu / kappa` mismatch:
    claim `11 / 3` versus guarded `18 / 5`
  - `replay_ablation = diverges_from_reference_replay` already starts at
    step `4`, so the structural replay fork predates the visible
    `nu / kappa` drift
- The stored late-step reproducer still matters, but it is no longer the first
  blocker. The stored `v3` run opened step `14` at `clause_kappa = 7` with
  `raw_catalog_clause_widths = [3,1,1,1,1,1,1]`, while the current reproducer
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
- A new forced local step-`13` operator-band regression now closes the old
  fork in the diagnosis:
  - under the already-divergent steps `10..12`, the natural claim step-`13`
    band is still `3..3`
  - forcing a local `7..7` reopen there still exact-screens every root before
    enqueue, leaving zero surviving candidates
  - the raw step-`13` structural tie-break was therefore the live fork, not a
    looser step-`14` exact-screen rule
- A new claim-only late-step acceptance repair now lands that fork in code:
  - under the divergent steps `10..12` history, the raw step-`13`
    structural tie-break still picks a dead-end `kappa = 3`, `nu = 23` shell
  - exactly one same-primary-tier survivor keeps step `14` alive
  - `desktop_claim_shadow` now prefers that viable shell before the later
    structural tie-break fields
- The release replay harness is now re-earned on the tracked
  `remaining_one_plateau` corpus, and the claim live-checkpoint persistence
  check is green.
- A new capped intended-profile validation read,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-late-accept-capped-v1`,
  is now past the old short-loop gate by `1200000 ms` with:
  - `prefix_states_explored = 141`
  - `prefix_cache_groups = 41`
  - `prefix_cache_candidates = 29249`
  - `frontier_queue_len = 2634`
  - RSS `= 466993152`
  - `terminal_summary_build_millis = 1191657`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`
- The stopped `v4` rerun now adds stronger frozen evidence:
  - it was manually stopped after entering step `14` on clean-tree repo head
    `140297377964dab9e0333782af3eec370bd784e7` with release binary hash
    `d3601f87cea1ff639d7c2ed19e604b1a815a65374790f6240910f7bebf3a711f`
  - no residual `pen-cli.exe` process remains, and its persisted `run.json`
    is now a stale last-write snapshot that still says
    `status = "running"`, `completed_step = 13`, `active_step = 14`,
    `active_band = 3`, `frontier_epoch = 10`
  - by `1194315 ms`, it re-earned the old step-`4` gate at
    `139` explored prefixes, `40` cache groups, `28438` cached candidates,
    queue `2636`, RSS `459812864`, with both zero-count summary checks still
    at `0`
  - by `1202537 ms`, it re-entered the older
    `41 groups / 29249 candidates` continuation surface at `140` explored
    prefixes
  - it then completed steps `4` through `13` and entered real step-`14`
    search with `raw_catalog_clause_widths = [168,168,168]`,
    `roots_seen = 90`, `roots_rejected_by_insert_root = 5`,
    `roots_enqueued = 85`, `generated_raw_surface = 90`, and
    `frontier_queue_len = 85`
- The old zero-candidate step-`14` opening is therefore no longer the first
  blocker.
- The current blocker is the earlier step-`4` replay-ablation fork:
  - claim step `4` uses open-band claim admissibility instead of the guarded
    former-eliminator focus gate
  - with `[demo] enabled = true`, claim step `4` also uses the early
    exhaustive discovery branch
  - the stopped claim step-`4` summary retains `7` candidates after an
    `open_band_structural` surface, while guarded step `4` keeps only `4`
    `focus_former_eliminator` survivors

## 1. Runtime Improvement Loop

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
