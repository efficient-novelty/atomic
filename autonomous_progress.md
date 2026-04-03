# Autonomous Claim Lane Progress

Last updated: 2026-04-03

This file is the live operating brief for `desktop_claim_shadow`.
Use [autonomous_next_steps.md](autonomous_next_steps.md) for the exact next
slice, [autonomous_plan.md](autonomous_plan.md) for the staged path to
signoff, and [autonomous_checklist.md](autonomous_checklist.md) for the final
gate.

## Current Status

- `desktop_claim_shadow` is not signoff-ready.
- The current short step-`4` baseline is
  `runs/codex-claim-release-step4-kernel-open-band-handoff-v1`.
- The current short-loop gate is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`.
- The current later-wall step-`4` continuation reference through `576` is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`.
- The current corroborating middle-wall read through `335` is
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v2`.
- The older farthest stored step-`4` continuation stop remains
  `runs/codex-claim-release-full-aggregation-open-band-prefix-local-score-v1`
  at `1095` explored prefixes.
- The stored failure
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v3`
  remains the finished pre-repair late-step reference:
  it reached step `14` and failed with
  `failure_note = "no atomic candidates were generated for step 14"`.
- The capped intended-profile read
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-late-accept-capped-v1`
  and the stopped rerun
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v4`
  remain frozen as the pre-repair early-runtime evidence set.
- The short pre-flight gate was rerun this turn on clean-tree repo head
  `c1fbb51d4fc9a620cd2ce95c9c3eadfe1a54fc65` with release binary hash
  `c42758f96c8171900651503d7f2a0ffe9915966c41edea98d8f1e296fc772a4e`:
  - targeted `desktop_claim_shadow` regressions green
  - claim live-checkpoint persistence green
  - release replay harness benchmark replayed all `5` stored
    `remaining_one_plateau` surfaces without mismatch
- A fresh clean-start full-profile rerun,
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v5`,
  now exists on that same clean-tree head and binary.
- Its authoritative `run.json` state is:
  - `status = "completed"`
  - `completed_step = 15`
  - `active_step = 16`
  - `active_band = 8`
  - `frontier_epoch = 12`
  - `resolved_worker_count = 8`
  - `dirty_tree = false`
- The new `v5` bundle now stores the full claim artifact surface through
  step `15`:
  - run manifest with complete host/build/git/binary provenance
  - step summaries through `step-15-summary.json`
  - live-checkpoint streams through `step-15-live.ndjson`
  - per-step narratives and event streams through step `15`
  - frontier and step checkpoints for the completed run
- Stored audit outputs now also exist under that `v5` run directory:
  - `claim-compare.txt` / `claim-compare.json`
  - `claim_certificate.txt` / `claim_certificate.json`
  - `claim_benchmark.txt` / `claim_benchmark.json`
- Those audits use the guarded baseline
  `runs/codex-guarded-claim-cert-v1`.
- New stored-evidence regressions now freeze that `v5` audit surface in-tree:
  - compare assertions pin the step-`9`, step-`11`, and step-`12`
    accepted-hash / trajectory forks
  - certification assertions pin the step-`1` breadth miss plus the
    step-`10..15` late generated-floor snapshot
  - benchmark assertions pin the single-run `408 ms` / parity-`0` /
    breadth-hit-`0` aggregate bundle

## Latest Full-Profile Outcome

- The new `v5` rerun closes the old completion blocker:
  `desktop_claim_shadow` now has one fresh clean-start full-profile bundle
  from the disclosed desktop that finishes through step `15`.
- Its terminal summary shows the accepted executable canon is still step `15`
  / `DCT` at `nu = 103`, `kappa = 8`.
- The old `v3` step-`14` zero-candidate opening is no longer the first live
  blocker:
  - `v5` reaches step `14`
  - step `14` opens at claim band `9..9`
  - step `14` records `roots_seen = 1` and `roots_enqueued = 1`
  - step `14` accepts a real survivor and then advances to step `15`
- The new compare audit against `codex-guarded-claim-cert-v1` shows the
  completed run is still not parity-clean:
  - trajectory diverges at step `9`, step `11`, step `12`, step `13`,
    step `14`, and step `15`
  - accepted hashes diverge at step `9`, step `11`, and step `12`
  - search-space counts diverge at step `4`, step `9`, step `10`,
    step `11`, step `12`, and step `14`
  - admissibility diagnostics diverge at step `9`, step `10`, step `11`,
    step `12`, step `13`, step `14`, and step `15`
- The earliest stored claim-specific accepted-hash fork is now localized:
  - step `9` keeps `nu = 17`, `kappa = 4`, but accepts a different hash than
    guarded replay
  - step `10` temporarily realigns on the guarded accepted hash
  - step `11` again keeps `nu = 26`, `kappa = 5`, but accepts a different hash
  - step `12` then falls to `nu = 33`, `kappa = 5` versus guarded
    `nu = 34`, `kappa = 6`
  - step `13` and step `14` reuse the guarded accepted hashes again, but on
    lower `nu` than guarded replay (`45` vs `46`, then `61` vs `62`)
  - step `15` returns to the guarded accepted hash / `nu` / `kappa`
    while replay ablation still records path divergence
- The certification audit is now explicit and honest:
  - status `= "attention"`
  - failing checks:
    - `accepted_hash_parity`
    - `early_breadth`
    - `late_generated_floors`
  - passing checks:
    - `search_policy`
    - `fallback_honesty`
    - `narrative_artifacts`
    - `runtime_threshold`
    - `exact_screen_reason_completeness`
    - `prune_class_completeness`
    - `manifest_completeness`
- The new completed run proves the claim artifacts are now operationally rich
  enough for certification-style auditing even though the lane still fails the
  content gates:
  - claim policy metadata is honest
  - fallback honesty is clear
  - narrative/event artifacts are complete
  - exact-screen reason counts are complete through step `15`
  - prune class counts are complete through step `15`
  - manifest provenance is complete on the stored release build
- Runtime is no longer the live blocker on the stored full-profile slice:
  - the benchmark bundle records one completed step-`15` claim run
  - stored runtime for `v5` is `408 ms`
  - the audit passes the provisional `600000 ms` runtime threshold
- The remaining breadth/floor failures are now concrete on stored evidence:
  - early breadth:
    - step `1` generated `546`, target `2144`
  - late generated floors:
    - step `10`: `1428` against target `500` (`hit`)
    - step `11`: `546` against target `800` (`miss`)
    - step `12`: `930` against target `1200` (`miss`)
    - step `13`: `9` against target `2200` (`miss`)
    - step `14`: `157` against target `3500` (`miss`)
    - step `15`: `780` against target `5000` (`miss`)

## Current Reference Runs

### Short-Loop Gate

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-v1`
- Nearest stored checkpoint to `1200000 ms`:
  - `elapsed_millis = 1191501`
  - `prefix_states_explored = 139`
  - `prefix_cache_groups = 40`
  - `prefix_cache_candidates = 28438`
  - `frontier_queue_len = 2636`
  - RSS `= 453021696`
  - `terminal_summary_build_millis = 1183915`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`

### Capped Intended-Profile Validation Read

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-late-accept-capped-v1`
- Launch surface:
  - clean-tree repo head
    `44b9871e65546a210c4ed71dcd31b91f8e6c521c`
  - release binary hash
    `d3601f87cea1ff639d7c2ed19e604b1a815a65374790f6240910f7bebf3a711f`
  - externally stopped after `1260 s`, so use `step-04-live.ndjson` as the
    authoritative record
- Last stored checkpoint at or before `1200000 ms`:
  - `elapsed_millis = 1199122`
  - `prefix_states_explored = 141`
  - `prefix_cache_groups = 41`
  - `prefix_cache_candidates = 29249`
  - `frontier_queue_len = 2634`
  - RSS `= 466993152`
  - `terminal_summary_build_millis = 1191657`
  - `terminal_summary_admissibility_checks = 0`
  - `terminal_summary_fallback_connectivity_checks = 0`

### Later-Wall Step-4 Reference Through `576`

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v1`
- Key stored later walls:
  - `140`: `1188340 / 1181188` with `41 groups / 29249 candidates`
  - `163`: `1381425 / 1373168` with `41 groups / 29249 candidates`
  - `332`: `2853118 / 2836752` with `42 groups / 29529 candidates`
  - `335`: `2879368 / 2862850` with `43 groups / 29809 candidates`
  - `408`: `3511930 / 3491831` with `43 groups / 29809 candidates`
  - `437`: `3770964 / 3749419` with `43 groups / 29809 candidates`
  - `454`: `3922561 / 3900177` with `43 groups / 29809 candidates`
  - `484`: `4183978 / 4160100` with `43 groups / 29809 candidates`
  - `576`: `4997082 / 4968579` with `43 groups / 29809 candidates`
  These pairs are `elapsed_millis / terminal_summary_build_millis`.

### Corroborating Middle-Wall Read Through `335`

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v2`
- Matched corroborating walls:
  - `140`: `1195999 / 1188793` with `41 groups / 29249 candidates`
  - `163`: `1390249 / 1381921` with `41 groups / 29249 candidates`
  - `332`: `2874771 / 2858216` with `42 groups / 29529 candidates`
  - `335`: `2901198 / 2884494` with `43 groups / 29809 candidates`

### Fresh Completed Full-Profile Claim Bundle

- Run:
  `runs/codex-claim-release-full-aggregation-open-band-clause-accept-rank-facts-long-rerun-v5`
- Launch surface:
  - clean-tree repo head
    `c1fbb51d4fc9a620cd2ce95c9c3eadfe1a54fc65`
  - release binary hash
    `c42758f96c8171900651503d7f2a0ffe9915966c41edea98d8f1e296fc772a4e`
- Full-profile outcome:
  - `status = "completed"`
  - `completed_step = 15`
  - `active_step = 16`
  - step `15` accepted `DCT` with `nu = 103`, `kappa = 8`
- Stored audits under the same run directory:
  - compare report against `runs/codex-guarded-claim-cert-v1`
  - failing-but-honest claim certificate
  - single-run benchmark bundle

## Landed Winning Stack

- delayed materialization
- incumbent-primary remaining-one fast path
- prefix-side single-clause structural-`nu` context with tiered `lib_refs`
- shared terminal-clause connectivity-facts sidecar on the clause catalog
- shared terminal-clause structural-`nu` facts sidecar threaded through the
  remaining-one bound, summary, and materialization paths
- terminal-clause bit-cost sidecar cached inside `TerminalClauseNuFacts`
- explicit remaining-one no-miss plateau kernel
- compact claim open-band aggregation fast path
- aggregation-side accept-rank short-circuit for primary-dominated bar-clearers
- prefix-local accept-rank context reused across compact remaining-one no-miss
  branches
- clause-side accept-rank facts reused across compact remaining-one no-miss
  branches
- broadened compact remaining-one survivor sketch with cached materialization
  reuse, while the dormant general cached-summary reopen path stays off
- deterministic replay harness plus stored retained plateau fixtures and
  benchmark
- late-step `claim_step_open` and `claim_root_seeding` diagnostics
- late-step step-`13` viability-tie acceptance repair
- early guarded-package parity repair through steps `4..8`
- stored compare / certification / benchmark outputs on one completed
  full-profile claim bundle

## Current Diagnosis

- The old early RSS cliff remains gone.
- Step `4` throughput is no longer the blocker that determines the next slice:
  the stored `v5` run completes the full claim profile in one clean-start pass.
- The old `v3` step-`14` zero-candidate opening is also no longer the first
  blocker:
  `v5` reaches step `14`, seeds roots, accepts a survivor there, and advances
  to step `15`.
- The current blocker is now stored parity plus stored breadth:
  - accepted-hash parity is still open through step `15`
  - the earliest claim-specific accepted-hash divergence is step `9`
  - the largest generated-floor collapse now localizes at steps `11..15`,
    especially steps `13..15`
  - step `15` still returns the accepted executable canon, so the lane is
    now failing by taking a too-thin or wrong route to the same endpoint
- The new stored full-profile bundle also changes what counts as the next
  honest engineering dollar:
  - do not reopen runtime-only step-`4` surgery first
  - do not reopen late-step zero-candidate diagnosis first
  - first localize the earliest parity fork and the late generated-floor
    collapse that remain on the completed run
- No residual `pen-cli.exe` process remains from the `v5` rerun.

## Forward Direction

- Freeze the current evidence set:
  `v1`, `v2`, `v3`, capped `v1`, stopped `v4`, and completed `v5`.
- Treat the pre-flight gate, the completed full-profile rerun, and the stored
  compare / certification / benchmark outputs as earned.
- Keep the new stored `v5` compare / certification / benchmark regressions
  green as the local parity-plus-breadth guardrail.
- Keep the claim-policy metadata, narrative/event artifacts, exact-screen
  reason counts, prune-class counts, and manifest provenance green.
- Prioritize targeted local diagnosis and repair for:
  - the step-`9` accepted-hash fork
  - the step-`11` accepted-hash fork
  - the step-`12` `nu / kappa` drop
  - the late generated-floor collapse at steps `11..15`
- Launch the next clean-start full-profile rerun only after the local repair
  is green against those stored parity/floor regressions.

## Immediate Next Move

1. Use the new stored compare regression to localize the step-`9`
   same-`nu` / same-`kappa` winner fork in code.
2. Extend that local diagnosis to the step-`11` same-`nu` / same-`kappa`
   fork and the step-`12` `33/5` versus `34/6` drop.
3. Explain why the completed claim path still misses the generated-floor
   targets at steps `11..15`, especially the `9 / 157 / 780` collapse across
   steps `13..15`.
4. Land the narrowest honest fix that restores stored parity/floor evidence
   without regressing the new step-`15` completion, while keeping the new
   stored-audit regressions green.
5. Only then launch `long-rerun-v6` and re-run compare, certification, and
   benchmark against the repaired bundle.
