# Schema2 transparent ambient-former internality result

**Date:** 2026-07-21. **Status:** all adjudicated transparent former
obligations replay at term level with zero marginal credit.

The create-new artifact is
`docs/schema2_ambient_former_internality_v1.json`, with result digest
`blake3:41b5451dc892b7d131230524df3ca8a5df1efe9fee49ea812c9c1de7944756c4`.
Definition replay succeeds.

## Term-level closure

The opaque constructor token elaborates and normalizes the selected clause,
then recursively replays every term premise. Its evidence records the exact
derivation rule, normal form, sealed-library or certified-prior-field
provenance, typed-result preservation, and the complete premise tree.

The registered matrix has 17 obligations: universe, variable, sealed library
constant, lambda, application, Pi, Sigma, identity, reflexivity, suspension,
truncation, the four Step-10 modal formers, and the two temporal formers. Every
matrix witness replays with `nu = 0`.

The rule fails closed for charged `PathCon`, candidate-declared formation,
uncertified field premises, guarded candidates without the separate inverse
law, and candidate-fresh application heads. Thus the rule proves only
transparent closure over premises that have already earned `Internal`.

## Resolved witness

For

```text
[Univ, Lam(App(Lib(14), Lib(15)))]
```

clause 0 inherits `Internal`. Clause 1 replays `lam-intro` over an `app-stuck`
body whose function and argument are separately sealed-library premises.
Stuckness is accepted only as a computation fact; it creates no novelty and
earns no credit. The whole candidate therefore enters `Internal` at `nu = 0`.

This certificate authorizes global E-4 v6 create-new. It does not authorize
the pending memberships or any later domino.
