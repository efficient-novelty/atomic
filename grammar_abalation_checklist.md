# Grammar Ablation Checklist

Last updated: 2026-04-17
Status: active

This file keeps only the remaining work needed to mark the grammar-ablation
plan as executed. Completed items are removed here and recorded in
`grammar_ablation_progress.md`.

## Pending

- [ ] Thread `grammar_profile` into live search/admissibility decisions so the
  engine can change behavior, not just metadata.
- [ ] Implement `no_temporal` grammar behavior:
  `Next` / `Eventually` unavailable to new search runs and temporal-shell
  clauses unreachable.
- [ ] Add honest halt/report behavior for ablation runs that cannot clear the
  bar under a hostile grammar.
- [ ] Implement `linear_exponential_swap` grammar behavior:
  replacement constructors, audit costs, canonical/human rendering, and late
  shell support.
- [ ] Add hostile-grammar config files only after the corresponding search
  semantics are live.
- [ ] Add targeted tests proving the canonical grammar path is unchanged while
  hostile grammars alter the reachable candidate surface.
- [ ] Extend reporting/inspection so ablation runs expose first divergence step,
  completed step, and halt reason cleanly.
- [ ] Produce one explicit stored canonical baseline control run using
  `configs/grammar_ablation_baseline.toml`.
- [ ] Produce one stored `no_temporal` ablation run and capture its inspected
  trajectory.
- [ ] Produce one stored `linear_exponential_swap` ablation run and capture its
  inspected trajectory.
- [ ] Write `grammar_ablation_report.md` with exact-match, phase-shape, and
  no-match verdicts.
- [ ] Update manuscript/repo claim language to match the observed ablation
  result.
