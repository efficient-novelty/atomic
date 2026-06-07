# Current Grammar Ablation

Use this note when the task is about hostile grammar profiles, grammar-choice
independence, or whether the current MBTT grammar is carrying too much of the
15-step result.

## Stable Repo Truths

- The preserved controls remain the current canonical strict lane plus the
  current claim-lane evidence bundle. Grammar-ablation work must not rewrite or
  retroactively reinterpret those stored artifacts.
- Repo-root planning for this lane lives in
  [../../grammar_ablation_gap.md](../../grammar_ablation_gap.md),
  [../../grammar_ablation_progress.md](../../grammar_ablation_progress.md),
  [../../grammar_ablation_findings.md](../../grammar_ablation_findings.md),
  [../../grammar_ablation_next_steps.md](../../grammar_ablation_next_steps.md),
  and [../../grammar_abalation_checklist.md](../../grammar_abalation_checklist.md).
- `grammar_profile` is now a real config / manifest / inspect field.
  Historical runs that predate the field should be reported as
  `grammar_profile: unknown`.
- `canonical_mbtt_v1` is still the executable control grammar.
- The fresh stored canonical control run
  `runs/grammar-ablation-baseline-v15-initial` now completes through step `15`
  under `canonical_mbtt_v1`. Current-head
  `pen-cli inspect runs/grammar-ablation-baseline-v15-initial/run.json`
  reports `status: completed`,
  `active_step: 16`,
  `active_band: 8`,
  `frontier_epoch: 12`,
  `updated_utc: 2026-04-18T19:06:10.8931279Z`,
  `first_divergence_step: none_through_step_15`, and
  `grammar_profile: canonical_mbtt_v1`; `reports/latest.txt` reproduces the
  current `DCT` finish at step `15` with `nu = 103`, `kappa = 8`,
  `rho = 103/8`, `bar = 19520/2639`, and
  `replay_ablation: matches_reference_replay x15`.
- The first hostile slice, `no_temporal`, is behaviorally live in code:
  runtime admissibility closes temporal syntax, forbids the temporal-shell
  package, and runtime-driven enumeration stops generating `Next` /
  `Eventually`.
- The first stored hostile run,
  `runs/grammar-ablation-no-temporal-v15-initial`, still matches the canonical
  replay through step `14`. Current-head `pen-cli inspect` now reports
  `status: failed`, `active_step: 15`,
  `updated_utc: 2026-04-18T18:37:26.7553791Z`,
  `first_divergence_step: none_through_step_14`,
  `grammar_profile: no_temporal`, and `failure_note: stale running manifest:
  owner pid 19812 disappeared on current host before step 15 persisted a
  terminal status`.
- The first retry that consumed the new sort boundary reached
  `strict_clause_materialization_position_0_sort_started_clause_count_3016662`
  at `163942 ms` and panicked because the previous
  `sort_unstable_by(compare_clause_sort_order)` path was not a valid total
  order on the live clause set.
- Current head now keeps the explicit
  `strict_clause_materialization_position_*_sort_started_clause_count_*`
  checkpoint but uses the safe cached-key sorter
  `sort_by_cached_key(clause_sort_key)` again.
- In the latest fixed-binary run segment (start at the last
  `resume_prefix_replay_progress_1_of_14` line in
  `runs/grammar-ablation-no-temporal-v15-initial/reports/steps/step-15-live.ndjson`),
  step `15` now clears materialization positions `4 .. 7` to ready at
  `358938 / 453308 / 590071 / 779492 ms`.
- Current head now also emits the explicit post-ready
  `strict_telescope_enumeration_handoff_started` checkpoint at `976407 ms`;
  the first emitted `strict_telescope_enumeration_progress` checkpoint follows
  at `976417 ms`, so the old blind handoff gap remains closed.
- Current head now carries the split DFS-local leaf counters in the step live
  checkpoint surface:
  `dfs_prefix_rejections`,
  `dfs_leaf_rejections`,
  `dfs_leaf_check_rejections`, and
  `dfs_leaf_connectivity_rejections`, plus the deeper
  `dfs_leaf_disconnected_rejections` and
  `dfs_leaf_connected_unqualified_rejections` split.
- In the latest `31`-minute rerun segment, the first structurally disconnected
  leaf lands at `976653 ms` with
  `dfs_leaf_rejections = 1`,
  `dfs_leaf_connectivity_rejections = 1`,
  `dfs_leaf_disconnected_rejections = 1`, and
  `dfs_leaf_connected_unqualified_rejections = 0`.
- Structurally connected but unqualified leaves do exist, but they start later:
  the first one lands at `987366 ms` with
  `generated_raw_surface = 408`,
  `enumerated_candidates = 188`,
  `dfs_leaf_connectivity_rejections = 168`,
  `dfs_leaf_disconnected_rejections = 167`, and
  `dfs_leaf_connected_unqualified_rejections = 1`.
- The same post-ready window still stays entirely inside
  `strict_telescope_enumeration_progress` through `1858587 ms`, ending at
  `generated_raw_surface = 30789`,
  `enumerated_candidates = 12478`,
  `prefix_states_explored = 29034`,
  `prefixes_created = 1`,
  `dfs_prefix_rejections = 1755`,
  `dfs_leaf_rejections = 16548`,
  `dfs_leaf_check_rejections = 0`,
  `dfs_leaf_connectivity_rejections = 16548`,
  `dfs_leaf_disconnected_rejections = 15730`,
  `dfs_leaf_connected_unqualified_rejections = 818`, and
  `observed_process_rss_bytes = 1844666368`, while the emitted downstream
  counters (`well_formed_candidates`, exact-screen / admissibility,
  candidate-pool, and prefix-cache fields) still remain `0`.
- Code read now explains that the downstream zeroes are stage-local rather than
  suspicious: `enumerate_telescopes_dfs(...)` emits the live progress stream
  and increments completed telescopes during the DFS, while
  `well_formed_candidates` only begins later in the engine's candidate-filter
  loop after `strict_telescope_enumeration_ready`.
- Code read in `crates/pen-type/src/connectivity.rs` now sharpens the remaining
  boundary too: `passes_connectivity(...)` only accepts witnesses with
  `connected = true` and at least one of
  `references_active_window`,
  `self_contained`, or
  `historical_reanchor`.
- The current observed live wall is therefore inside the connectivity witness
  path after the explicit handoff, not clause sort, per-position `nu`, the
  post-ready transition, the prefix-side pre-recursion gate, the
  `check_telescope(...)` gate, or the downstream filter loop itself; within
  that witness path the wall is mostly structural disconnection rather than a
  connected-but-unqualified qualification failure.
- Current head still streams huge terminal exact-expression buckets directly
  into clause materialization and only uses raw clause-width preallocation
  hints at the terminal position, because earlier width probes are upper bounds
  rather than exact widths.
- Current-head `pen-cli inspect` surfaces stored run `status`, `active_step`,
  `active_band`, `frontier_epoch`, `updated_utc`, `first_divergence_step`, and
  `failure_note` directly from the manifest and keeps `reports/latest.txt`
  aligned when it reconciles a stale owner.
- `linear_exponential_swap` is now both behaviorally live and empirically
  stored: current-head `pen-cli inspect` reports run
  `grammar-ablation-linear-exponential-v15-initial` as `status: failed`,
  `active_step: 15`,
  `updated_utc: 2026-04-18T21:03:52.2947225Z`,
  `first_divergence_step: none_through_step_14`,
  `grammar_profile: linear_exponential_swap`, and
  `failure_note: no atomic candidates were generated for step 15`.
- The swapped step-`15` live surface is much narrower than the `no_temporal`
  wall: it builds a single raw hostile shell with
  `raw_catalog_telescope_count = 1`,
  `generated_raw_surface = 8`,
  `prefixes_created = 1`, and one
  `dfs_leaf_connected_unqualified_rejections = 1` with
  `dfs_leaf_disconnected_rejections = 0`; downstream candidate-filter
  counters still remain `0`.
- `grammar_ablation_report.md` now exists and gives the first direct
  comparison verdict: both hostile profiles preserve exact replay through step
  `14` / `Hilbert`, but neither hostile profile currently recovers a stored
  step-`15` terminal, so the executable MBTT grammar is load-bearing for the
  concrete step-`15` finish at this scale.
- The remaining theory-side companion surfaces now match that same boundary
  too: `skills/pen-atomic/theory/README.md` and
  `skills/pen-atomic/theory/terminal-dct.md` both scope the step-`15`
  temporal shell to the canonical control grammar and point grammar-robustness
  questions back to the repo-root ablation docs.
- The latest repo-wide `.md` / `.tex` drift sweep at
  `2026-04-19T15:02:20.8495808+02:00` found only the grammar-ablation
  operational docs plus this skill context newer than the prior
  `2026-04-19T14:02:03.8717831+02:00` checkpoint; the keyword hits stayed
  inside that set, so no repo-facing manuscript, theory, or summary surface
  reopened wording work.
- `epistemic_swap` is still only a named profile in config and manifests; it
  is not behaviorally live yet, and it is no longer the immediate next slice.

## First Reads

1. [../../grammar_ablation_gap.md](../../grammar_ablation_gap.md)
2. [../../grammar_ablation_progress.md](../../grammar_ablation_progress.md)
3. [../../grammar_ablation_findings.md](../../grammar_ablation_findings.md)
4. [../../grammar_ablation_next_steps.md](../../grammar_ablation_next_steps.md)
5. [04-mbtt-kernel.md](04-mbtt-kernel.md)
6. [05-search-and-selection.md](05-search-and-selection.md)
7. [theory/README.md](../theory/README.md)
8. [theory/genesis.md](../theory/genesis.md)

## Working Rules

- Preserve the canonical and claim-lane evidence as controls; add new configs,
  run IDs, manifests, and reports for ablation work instead of mutating old
  evidence.
- Do not claim grammar independence from metadata-only plumbing. A profile only
  counts as live once it changes search/admissibility behavior in code and has
  tests.
- Do not claim empirical divergence, convergence, or completed-step results
  until a stored hostile run exists and its step-`15` state is a meaningful
  terminal outcome rather than a stale-owner failure.
- If a live hostile run was written by an older binary, run current-head
  `pen-cli inspect` once before trusting `reports/latest.txt`; current-head
  inspect now refreshes a stale `running` manifest into a failed latest-report
  surface.
- If a resumed hostile run clears `strict_clause_materialization_position_7_ready`
  and the explicit `strict_telescope_enumeration_handoff_started` note lands,
  treat the handoff as solved; once a longer rerun still stays inside
  `strict_telescope_enumeration_progress` with rising counters, stop blind
  reruns and add the next witness-local split before reopening sort or
  per-clause fact work; once the connectivity-local split itself is landed and
  honestly localizes the wall, store one fresh canonical baseline control run;
  with that control, the first stored `linear_exponential_swap` run, and the
  report now in hand, the wording sweep now covers
  `tex/automated_theory_synthesis_jar_draft.tex`,
  `tex/synthetic_framework_abstraction_mscs_draft.tex`,
  `tex/pen_paper.tex`, the companion summaries in `tex/pen_lmcs.tex` and
  `tex/constructive_idealism.tex`, and the theory-side entry points
  `skills/pen-atomic/theory/README.md` plus
  `skills/pen-atomic/theory/terminal-dct.md`. Repeated repo-facing doc sweeps,
  most recently the 2026-04-19T15:02:20.8495808+02:00 pass, again found only
  the grammar-ablation working docs and skill context newer than the prior
  `2026-04-19T14:02:03.8717831+02:00` checkpoint and no new grammar-robust
  step-`15` overclaim or newer repo-facing claim surface, so the lane remains
  in hold-mode monitoring: only reopen wording work if a later changed
  repo-facing claim surface reintroduces grammar-robust step-`15` language,
  and do not reopen deeper `no_temporal` instrumentation or start
  `epistemic_swap` first unless new hostile evidence changes the current
  boundary.
- If a new sort helper trips Rust's total-order assertion on live step-`15`
  data, revert to the safe cached-key clause sorter before making more timing
  claims about the post-ready path.
- If raw clause-width preallocation is touched again, keep those hints
  terminal-only unless the width probe becomes exact for earlier positions.
- Treat the `strict_clause_catalog_ready` `raw_catalog_telescope_count` as a
  saturating upper bound, not an exact post-catalog telescope count; add a
  later honest checkpoint if you need to know whether actual clause
  materialization, the telescope DFS, or the post-enumeration filtering loop
  is the real wall.
- If a hostile grammar cannot clear the bar, report that as a result rather
  than smoothing it into the canonical narrative.
