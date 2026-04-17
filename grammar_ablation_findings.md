# Grammar Ablation Findings

Last updated: 2026-04-17
Status: preliminary implementation findings only

## Findings

- The grammar is currently hardcoded across the AST, bit-cost audit, typed
  enumeration, admissibility, and reporting layers rather than modeled as a
  swappable runtime object.
- Temporal availability is presently exposed through search/admissibility
  switches such as `include_temporal` plus temporal-shell clause-family logic.
  That makes `no_temporal` the cleanest first hostile grammar to implement.
- Historical `run.json` artifacts do not record a grammar profile. The new
  `inspect` path handles that honestly by printing `grammar_profile: unknown`
  for old runs.
- The canonical path still parses and tests as before after adding
  `grammar_profile`; the new field is additive so far, not behavioral.
- Regenerating schemas after the manifest change refreshed
  `schemas/run_manifest_v1.schema.json` as expected and also refreshed
  `schemas/frontier_manifest_v1.schema.json`, revealing existing schema drift
  relative to the current code-generated manifest model.

## Non-Findings Yet

- No hostile-grammar search run has been executed yet.
- No empirical independence verdict exists yet.
- No divergence step or halt step has been measured yet.
