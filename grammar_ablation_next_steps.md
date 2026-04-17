# Grammar Ablation Next Steps

Last updated: 2026-04-17

This file owns the single active next slice for the grammar-ablation lane.

## Objective

Implement an honest `no_temporal` grammar path in the live search engine while
keeping `canonical_mbtt_v1` behavior unchanged.

## Hypothesis

The first real leverage is to make `grammar_profile` affect enumeration and
admissibility before touching more invasive AST surgery. In practice that means
gating temporal constructor generation and temporal-shell clause families on
the selected grammar profile, then adding a clear halt/report path when the run
can no longer clear the bar.

## Start From

- `crates/pen-search/src/enumerate.rs`
- `crates/pen-type/src/admissibility.rs`
- `crates/pen-search/src/engine.rs`
- `configs/grammar_ablation_baseline.toml`
- `grammar_ablation_findings.md`

## Immediate Tasks

- Add the minimal helper(s) that answer whether temporal constructors are
  allowed under the active `grammar_profile`.
- Use those helpers to suppress `Next` / `Eventually` generation for
  `no_temporal`.
- Prevent temporal-shell clause-family opening under `no_temporal`.
- Add focused tests:
  canonical profile still emits the current temporal surface,
  `no_temporal` does not.
- Decide the exact halt/report contract for a hostile run that cannot advance.

## Stop Condition For The Next Slice

- A `no_temporal` profile changes the reachable search space in code and tests,
  while the canonical profile remains regression-green.
