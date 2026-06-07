# Grammar Ablation Report

Last updated: 2026-04-19
Status: stored comparison report plus broader claim-language sweep landed; current
evidence preserves the canonical corridor through step `14` / `Hilbert` under
both hostile grammars, but neither hostile profile recovers a stored
step-`15` terminal, so the concrete step-`15` finish is grammar-sensitive on
current head.

## Evidence Base

- `runs/grammar-ablation-baseline-v15-initial/run.json` and
  `runs/grammar-ablation-baseline-v15-initial/reports/latest.txt` provide the
  executable control: `grammar_profile: canonical_mbtt_v1`,
  `first_divergence_step: none_through_step_15`, and
  `replay_ablation: matches_reference_replay x15`.
- `runs/grammar-ablation-no-temporal-v15-initial/run.json`,
  `runs/grammar-ablation-no-temporal-v15-initial/reports/latest.txt`, and
  `runs/grammar-ablation-no-temporal-v15-initial/reports/steps/step-15-live.ndjson`
  provide the first hostile boundary: exact replay still holds through
  `Hilbert`, and the stored step-`15` live window localizes a long
  connectivity-heavy wall even though the manifest itself ended as a stale
  owner failure.
- `runs/grammar-ablation-linear-exponential-v15-initial/run.json`,
  `runs/grammar-ablation-linear-exponential-v15-initial/reports/latest.txt`,
  and
  `runs/grammar-ablation-linear-exponential-v15-initial/reports/steps/step-15-live.ndjson`
  provide the first stored swapped hostile boundary: exact replay still holds
  through `Hilbert`, and the step-`15` failure is an immediate no-candidate
  collapse on a tiny hostile shell.

## Exact-Match Surface

- The canonical control remains the stored baseline rerun
  `grammar-ablation-baseline-v15-initial`, which completes through step `15`
  with the current `DCT` finish at `nu = 103`, `kappa = 8`, and `rho = 103/8`.
- Both hostile profiles still report `latest: step 14 (Hilbert)` and
  `replay_ablation: matches_reference_replay x14`.
- The current hostile evidence therefore preserves exact accepted-step parity
  through bootstrap, geometric ascent, and framework abstraction up to
  `Hilbert`; the first empirical break is at step `15`, not earlier.

## Step-15 Failure Classes

- `no_temporal` does not recover any stored step-`15` terminal. Its manifest
  now reports `status: failed` only because the older owner disappeared before
  persisting a terminal result, so the honest step-`15` evidence comes from the
  fixed-binary live segment. That segment reaches
  `generated_raw_surface = 30789`,
  `enumerated_candidates = 12478`,
  `prefix_states_explored = 29034`,
  `dfs_leaf_connectivity_rejections = 16548`, split into
  `dfs_leaf_disconnected_rejections = 15730` and
  `dfs_leaf_connected_unqualified_rejections = 818`, while downstream
  candidate-filter counters still remain `0`. This is a long
  connectivity-local failure, dominated by structural disconnection.
- `linear_exponential_swap` also fails to recover any stored step-`15`
  terminal, but its surface is much smaller and cleaner:
  `failure_note: no atomic candidates were generated for step 15`. The live
  window reaches only `raw_catalog_telescope_count = 1`,
  `generated_raw_surface = 8`,
  `prefix_states_explored = 8`,
  `prefixes_created = 1`, and one
  `dfs_leaf_connected_unqualified_rejections = 1` with
  `dfs_leaf_disconnected_rejections = 0`; `strict_telescope_enumeration_ready`
  then lands with `raw_catalog_telescope_count = 0`, and downstream
  candidate-filter counters still remain `0`. This is a tiny
  connected-unqualified collapse, not a broad search wall.

## Verdict

- Exact match: both hostile profiles keep the canonical replay through step
  `14` / `Hilbert`.
- Phase-shape match: the current
  `bootstrap -> geometric ascent -> framework abstraction` corridor still
  survives under both hostile grammars.
- No terminal match: neither hostile profile recovers a stored step-`15`
  analogue of the canonical `DCT` finish.
- Honest claim boundary: current evidence supports robustness through the
  step-`14` framework corridor, but it does not support grammar independence
  for the concrete step-`15` temporal-cohesive finish. At the current scale,
  the executable MBTT grammar is load-bearing for that final step.

## Claim-Language Updates Landed

- `tex/automated_theory_synthesis_jar_draft.tex`: the abstract and conclusion
  now distinguish canonical recovery of the temporal-cohesive endpoint from
  hostile-grammar evidence that only stays exact through `Hilbert`.
- `tex/synthetic_framework_abstraction_mscs_draft.tex`: the abstract, core
  thesis, and late theorem / conclusion language now scope the step-`15`
  endpoint to the fixed canonical MBTT late vocabulary and state the
  hostile-ablation boundary explicitly.
- `tex/pen_paper.tex`: the `Kolmogorov invariance of the encoding bias` remark
  plus the remaining result-summary / fixed-point / conclusion shorthand now
  keep step `15` explicitly inside the disclosed canonical MBTT lane.
- `tex/pen_lmcs.tex` and `tex/constructive_idealism.tex`: the companion
  abstract / theorem-summary / conclusion passages now treat the step-`15`
  endpoint as a canonical-grammar result rather than as grammar-independent
  executable evidence.
- Any follow-on wording work should start with remaining theory-side /
  companion-summary docs rather than new hostile-profile plumbing.
