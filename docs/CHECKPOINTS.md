# Checkpoints

Step checkpoints are the stable resume unit. Frontier checkpoints are resumable only
when compatibility hashes and the frontier record layout match exactly.

## Resume policy

- Same `ast_schema_hash`, `type_rules_hash`, `evaluator_hash`,
  `search_semantics_hash`, and `record_layout_id`: resume from frontier checkpoint.
- Same AST, type, and evaluator hashes but different search semantics: discard
  frontier and resume from the last step checkpoint.
- Same AST and type hashes but different evaluator: resume from the last step
  checkpoint and re-evaluate from there.
- Different AST schema: require migration or replay.
