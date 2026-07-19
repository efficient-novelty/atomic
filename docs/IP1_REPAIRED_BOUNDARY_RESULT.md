# IP-1 repaired candidate-boundary result

**Date:** 2026-07-19. **Artifact:** `docs/ip1_candidate_verdict_join_v3.json`, schema 3, digest `blake3:e4cb89fe83476fb8a0eee43bcc976053bcbbff0bd6db35607d7e8154a3ef858b`.

## Result

The full frozen Step-16 raw product was streamed into a finite operational quotient and replayed successfully. With A5 explicitly recorded as **Adopted**, every positive-count row has one of two operational outcomes: a locally decisive bare-`Univ` kernel rejection, or absence of a full-candidate typed score certificate, which A5 rejects as unrankable. Optional H, P5, and synthesis amplification routes are also fail-closed.

Therefore the current checker has no certifiably clearing Step-16 candidate under adopted A5. This is an **operational checker result**, not an exact intended typed-semantic halt theorem.

| kappa | per-position raw widths | raw tuples | quotient rows | maximum conditional clause-local bound | first clearing integer |
| ---: | --- | ---: | ---: | ---: | ---: |
| 2 | 662184 x 910182 | 602707957488 | 63 | 8 | 19 |
| 3 | 662184 x 910182 x 1207872 | 727994066026945536 | 72 | 10 | 28 |
| 4 | 662184 x 910182 x 1207872 x 1559142 | 1135046124093383916890112 | 78 | 12 | 37 |

For every stratum, quotient-row counts sum exactly to the raw Cartesian-product count. No clause-local unclassified outcome or raw P5 issuer-audit residual remains. The displayed maxima are conditional diagnostics only: they are not promoted to implemented candidate scores and do not discharge A5.

The certificate carries the adopted premise and its provenance rather than inferring it from a field name:

> On a debt-free field, acceptance requires a certified clearing; unrankable candidates cannot be accepted.

Its recorded source is the user's explicit adoption: “Yes I adopt A5 as an axiom in the context of this plan.” Replay rejects the operational theorem flags if that adjudication is changed, even when the payload digest is recomputed.

## Exact limits

The finite quotient records a `clause_local_extraction_proxy`; it does not claim that this proxy equals the full candidate extractor. Accordingly every row keeps these full-candidate fields false or absent:

- `implemented_egp_bridge_returns_bound` is false;
- `implemented_exact_egp_nu`, `implemented_egp_nu_upper_bound`, `implemented_route_nu_upper_bound`, and `intended_semantic_nu_upper_bound` are null;
- `full_candidate_extraction_join_proved`, `whole_telescope_fuel_compositionality_proved`, `typed_instance_sort_preservation_proved`, `complete_naturality_basis_proved`, and `intended_semantic_candidate_join_proved` are false;
- `exact_family_count_distribution_claimed` is false.

The only numeric score diagnostics are `conditional_on_successful_full_egp_bridge_nu_upper_bound` and `conditional_clause_local_route_nu_upper_bound`. They state what would follow after the missing candidate-level bridge premises; replay never treats them as certified scores.

Consequently this artifact does not classify intended depth-two semantic schemas by support-local normal forms, prove exact marginal-family counts, or establish the global semantic halt at fifteen. Those claims still require a sort-preserving typed substitution/instance theorem, a complete naturality basis, and a whole-telescope elaboration/fuel argument.

## P5 boundary after the token repair

The old premise “no public token constructors exist” is false as written. A genuine public non-vacuous lift witness exists: its first clause is `App(Lib(14), Lib(13))`; the kernel issues the lift token and the public replay adapter accepts it. That witness is outside the frozen raw Step-16 leaf surface because `Lib(13)` is unavailable there.

Lift replay is not full P5 evidence. Independent record-internality capability remains unavailable, so the witness cannot construct full P5 amplification. The raw product has only `Lib(14)`/`Lib(15)` leaves; the exported transportable domains are `{Lib(11), Lib(12), Lib(13)}` for `Lib(14)` and empty for `Lib(15)`. Thus every raw P5 route receives a named fail-closed outcome, while the public partial lift route is reported explicitly.

H and synthesis amplification also remain fail-closed because their required public sidecar capabilities are unavailable.

## Replay and artifact immutability

Candidate-join replay always performs full definition recomputation; there is no public weak-replay mode. Tests mutate row scores, A5 adoption, the signature digest, P5 domains, the public P5 witness, and capability flags, recompute the payload digest, and confirm rejection.

The schema-3 generator uses create-new semantics and refuses to overwrite an existing artifact. The earlier `docs/ip1_certification_boundary.json` schema-1 burn remains byte-for-byte archival and is replay-pinned to commit `96c8bec`; the current legacy runner also uses create-new semantics so it cannot overwrite that burn. Schema 3 is a new repaired-boundary record, not a retroactive rewrite.

## Verification

- `cargo test -p pen-search candidate_join --lib`: 8 passed, including exhaustive concrete bisimulation for kappa 2 at expression-node cap 3 and all kappa 2 through 4 at cap 1.
- Full schema-3 generation completed at kappa 2 through 4 and expression-node cap 6.
- `target/release/examples/ip1_candidate_join.exe replay docs/ip1_candidate_verdict_join_v3.json`: valid, with all operational A5 conclusions true and intended semantic completeness false.
- A second generation attempt against the schema-3 path was refused before enumeration.
