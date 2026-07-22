# A3 rule-inventory exhaustiveness v2 result

**Date:** 2026-07-21  
**Schema:** `a3-rule-inventory-exhaustiveness-v2`  
**Status:** **relative exhaustiveness proved; broader semantic-language scope not claimed**

## The theorem proved

The successor theorem leaves the sealed A3 v1 generator and all v1 artifacts
unchanged.  It reconstructs the intended rule domain independently and then
performs a two-way join:

1. the DEMAND-COMPLETE seed grammar supplies every unary action, chronological
   binary comparison, and higher open-box seed from the raw two-entry window;
2. `summarize_structural_debt` plus `required_packages_for` independently
   supplies the structural future-hole seeds;
3. every independent seed receives exactly one typed disposition: one A3
   instance/scheme/orbit, or a replayed typed eligibility rejection; and
4. every operational A3 instance, scheme, and orbit is consumed by exactly one
   independent preimage.

The structural branch additionally joins every positive raw debt predicate to
one A3 constructor-evidence token, one structural scheme, one singleton
instance, one orbit, and one motive-typed, zero-charge future-hole open
judgment.  Negative predicates have none of those outputs.

Across stages 1--16 the theorem classifies 443 independent base seeds and all
192 structural-family candidates (12 at each stage).  All sixteen window
proofs pass.  At Stage 16 the partition is 90 raw base seeds: 89 promoted and
the higher singleton rejected for absence of a typed path witness; no
structural predicate is live.

## Branch-safe API

`prove_a3_window_inventory_for_exact_prefix(signature, stage)` accepts only an
exact sealed prefix `1..stage`.  It accepts no focus label, expected count,
score, bar, winner, or next candidate.  The A3 prefix validator rejects longer,
shorter, and non-contiguous inputs.  This is the integration surface for tied
or revised histories.

The historical issuer, replay, JSON, and create-new surfaces are:

- `issue_historical_a3_rule_inventory_exhaustiveness`;
- `replay_historical_a3_rule_inventory_exhaustiveness`;
- `historical_a3_rule_inventory_exhaustiveness_json_pretty`;
- `emit_historical_a3_rule_inventory_exhaustiveness_create_new`.

## Schema2 archive disclosure

The frozen G2--G7 grammar-completion artifact verifies its own result digest,
and all 43 registered rows join to current A3 typed source parameters with the
same kernel role, type, and normal form.  Its *live definition replay* is
false: the archived M1 predecessor pins the former
`e4_generator_basis.rs` bytes (`82053`, `fb19...`), while the current source is
`82321`, `149d...`.  This exact drift is serialized under
`A3_COMPLETED_SCHEMA2_ARCHIVE_LIVE_REPLAY_SOURCE_DRIFT` and is not used as a
premise of the independently reconstructed A3 rule enumeration.

The source-bound Global E-4 v10 archive scopes the relative theorem to the
completed wrapped domain: its archived digest is bound, wrapped class
exhaustion is true, and no `Unknown` survives.  It does not generate or filter
the four A3 rule shapes; the seed grammar and raw structural predicates remain
the only rule-domain generators.

The theorem is relative to the adopted/frozen operational depth-two rule
language.  It does not claim that this language exhausts every conceivable
future semantic schema formalism.  That deliberately unclaimed extension is
named
`A3_ABSOLUTE_RULE_SHAPE_EXHAUSTIVENESS_BEYOND_ADOPTED_DEPTH_TWO_GRAMMAR_NOT_PROVED`.

## Artifact and replay

- Certificate: `docs/a3_rule_inventory_exhaustiveness_v2.json`
- Issuer: `crates/pen-eval/src/a3_rule_inventory_exhaustiveness.rs`
- Driver: `crates/pen-eval/examples/a3_rule_inventory_exhaustiveness.rs`
- Result digest:
  `blake3:bd3f33ed7a15b6f50c421d2e42ed7aaea710fbcab750d0a98a990a1dad3b30db`

Disk replay returns `valid = true`, 16 historical windows, 443 base seeds,
192 structural candidates, relative exhaustiveness `true`, and absolute
semantic exhaustiveness `false`.
