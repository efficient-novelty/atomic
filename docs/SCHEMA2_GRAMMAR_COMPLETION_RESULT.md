# Schema2 grammar completion result (G-2 through G-7)

**Date:** 2026-07-20. **Status:** complete and replay-clean under the adopted
`steps-9-15-trace-faithful-signature-batch-v1`.

## Result

The create-new artifact is `docs/schema2_grammar_completion_v1.json`, with
result digest
`blake3:26ccc358c25ace6501e81125363064730c1642cfe775b0a931e8298bc5d60215`.
Definition replay succeeds.

- G-2 registers the four Step-10 modal trace signatures.
- G-3 registers exactly the eight Step-15 J3 roles: two operator formers,
  comparison, interaction, two exchanges, eliminator, and composition.
- G-4 registers the 31 Step-9/11/12/13/14 Map/Axiomatic signatures.
- G-5 treats P6/combinatorial synthesis as derived closure over the eight
  Step-15 primitives. It exports zero additional primitives.
- All 43 adopted signatures elaborate against their exact predecessor
  signatures, normalize stably, remain support-local, and carry a naturality
  witness under the genuine non-renaming action `A |-> Trunc(B)`. Coarse
  premises are retained rather than erased.
- G-7 combines nine ordinary naturality constructors, eight previously public
  cubical constructors, and four endpoint-premise constructors. The public
  cubical induction therefore has 12 forms. The endpoint publication is an
  opaque replay-only projection of the registered Trunc computation audit: it
  exposes no premise ledger and accepts no caller-supplied premise.
- `agda/Schema2GrammarCompletion.agda` checks with Agda 2.8.0 under
  `--safe --without-K`, with no postulates.

## Separate G-6 decision

No unit-coherence generator is issued. The decision uses the sealed trace and
frozen specification only. Step 8 clause 4 is `Lam(Var(2))`: the intended
left-unit type can be registered, but this raw clause is neither `Id` nor path
syntax and the adopted sources provide no noncircular typed inhabitant or
generator rule. The named gap is
`G6_STEP8_TRACE_HAS_COHERENCE_SIGNATURE_BUT_NO_TYPED_NONCIRCULAR_PATH_TERM_RULE`.

The M1 sweep is replayed only as an archival predecessor. Its pending verdict,
counts, score effects, and desired outcome are explicitly not decision grounds.

## Firewall

This phase issues no candidate membership verdict, independent-family verdict,
E-2b result, stage count, F-Q2 score, or halt/continuation conclusion. It only
authorizes G-8.
