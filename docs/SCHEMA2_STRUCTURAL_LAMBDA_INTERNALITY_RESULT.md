# Schema2 structural lambda internality result

**Date:** 2026-07-20. **Status:** structural lambda introduction closes over
an already `Internal` body with zero marginal credit.

The create-new artifact is
`docs/schema2_structural_lambda_internality_v1.json`, with result digest
`blake3:1fabce65fe81b17bed17f26b565244f895251a74b9481ff847cc1a824589441b`.
Definition replay succeeds.

## Constructor rule

The kernel export is an opaque token whose replay projection is bound to the
sealed signature, visible library, candidate hash, whole-telescope elaboration
hash, clause index, scopes, expression, normal form, and exact elaboration
subtree. It recognizes only the following closed structural derivations:

1. `Univ` by `univ-form`;
2. a visible sealed predecessor constant by `library-constant`; and
3. `Lam(body)` by a one-child `lam-intro`, provided the child recursively
   replays one of these structural derivations.

Applications, variables, candidate-field references, ambient parameters, and
all other schema constructors fail closed. Guarded candidates must still use
the separate weakening/erasure theorem.

Every accepted node and every lambda constructor records marginal `nu = 0`.
The persisted projection is not a token issuance path: replay reissues the
opaque token from the bound inputs and compares the complete projection and
derivation hash.

## Candidate consequence

For

```text
[Univ, Lam(Univ)]
```

clause 0 inherits its previously earned inductive `Internal` certificate.
The body of clause 1 replays `Univ` as a structural base derivation, after
which the enclosing `lam-intro` constructor replays. The two clauses cover the
candidate exactly once, so the candidate earns `Internal` with `nu = 0`.

This result resolves only the registered structural-lambda witness and
authorizes a create-new global E-4 rerun. It does not by itself authorize the
five pending membership verdicts or later stages.
