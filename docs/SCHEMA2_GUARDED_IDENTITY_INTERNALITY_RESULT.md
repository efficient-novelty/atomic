# Schema2 guarded identity internality result

**Date:** 2026-07-20. **Status:** the one-ambient-parameter identity
family earns `Internal`; global E-4 may rerun create-new.

The create-new artifact is
`docs/schema2_guarded_identity_internality_v1.json`, with result digest
`blake3:3fee6bb232dff5d5c854c74a7467548aa8fa0cda128db44ade4a8614711a8b0f`.
Definition replay succeeds.

## Exported evidence

The existing guarded weakening/erasure theorem is exported at the shared
`pen-type` boundary as an opaque, replayable token for each candidate clause.
Each token is bound to the signature, visible library, base and guarded
candidate hashes, both elaboration hashes, clause index, expression, and
scope. Its public projection records four checked equalities:

1. weakening the base clause gives the guarded clause;
2. erasing the guarded clause gives the base clause;
3. erasure after weakening is the identity on the base clause; and
4. weakening after erasure is the identity on the guarded clause.

The token's proof fields are private, so deserializing a claimed proof is not
an issuance path. Replay reissues the token from the bound inputs and compares
its derivation hash. Mutation tests cover the clause projection and both
candidate directions.

## Classifier consequence

The base family

```text
[Univ, Lam(Var(2))]
```

already earns `Internal`. Under the frozen absolute-level convention, its
weakening through one unused ambient parameter is

```text
[Univ, Lam(Var(3))].
```

Both telescope clauses now carry replayed weakening/erasure tokens and both
inverse laws. The guarded family therefore earns `Internal` with marginal
`nu = 0`; it creates no independent schema credit.

This result discharges only the guarded identity witness identified by global
E-4 v3. It authorizes a create-new E-4 rerun but does not itself authorize the
five membership verdicts or any later stage.
