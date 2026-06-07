# Grammar Ablation Findings

Last updated: 2026-04-19
Status: hostile evidence is unchanged, and the
`2026-04-19T15:02:20.8495808+02:00` drift sweep found only the
grammar-ablation operational docs plus grammar-ablation skill context newer
than the prior `2026-04-19T14:02:03.8717831+02:00` checkpoint.

## Findings

- `grammar_ablation_report.md` remains the direct comparison verdict: exact
  replay survives through step `14` / `Hilbert` under both hostile profiles,
  but the concrete step-`15` finish is still grammar-sensitive.
- The canonical control
  `runs/grammar-ablation-baseline-v15-initial` still completes through step
  `15` under `grammar_profile: canonical_mbtt_v1` with the canonical `DCT`
  finish.
- `runs/grammar-ablation-no-temporal-v15-initial` still fails inside the
  step-`15` connectivity witness path after a long live window dominated by
  structurally disconnected leaves; downstream candidate-filter counters
  remain `0`.
- `runs/grammar-ablation-linear-exponential-v15-initial` still fails earlier
  on a tiny connected-but-unqualified hostile shell with zero atomic
  candidates.
- Since `2026-04-19T14:02:03.8717831+02:00`, no repo-facing theory or summary
  `.md` / `.tex` surface changed; the only newer files were the operational
  docs and grammar-ablation skill context.
