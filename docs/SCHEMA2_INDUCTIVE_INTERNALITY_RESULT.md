# Schema2 inductive telescope-internality result

**Date:** 2026-07-20. **Status:** adopted ordered dependency rule proved for
the exact witness; projection and identity remain distinct.

The create-new artifact is `docs/schema2_inductive_internality_v1.json`, with
result digest
`blake3:267ff195edb832ee250a422aab483fcde9db019b0d5154e3d3253f056253b3d7`.
Definition replay succeeds.

## Ordered proof

The successor checks the telescope left-to-right and carries an explicit map
from certified clause indices to their Internal derivation hashes.

For the exact witness:

1. Clause 0, `Univ`, earns direct Internal status as an ambient-arena
   reference.
2. Only after clause 0 enters the certified prefix does clause 1 run.
3. Clause 1, `Lam(Var(1))`, derives as `lam-intro(field-ref-0)`. Its field
   dependency is strictly prior, clause 0 is already certified, and the
   certificate retains clause 0's derivation hash.
4. Clause 1 therefore earns `inductive_constant_projection`, explicitly
   marked not identity.

The whole telescope receives a certificate-backed `Internal` classification
with marginal `nu = 0`. The projection-clause derivation is
`blake3:4a7f986bb232a44acb238af78b7f32223d717ea513a5169cf0479da0b0f26441`.

The control `Lam(Var(2))` independently remains
`direct_structural_identity`, with leaf `local-var-1`, no field dependency,
and derivation
`blake3:1db76cbe661883fc84f1e397e8b84b815ceb35ad48edfff6f6bf5ed91fe050fc`.

## Versioning and firewall

The predecessor F-I2 certificate remains correct and unchanged under its
stricter direct-B15 rule. This successor records the new ordered derivation;
it does not reinterpret the predecessor artifact.

Forward, missing, cyclic, or uncertified field dependencies fail the ordered
prefix check. Ambient guarded clauses still require weakening/erasure inverse
laws. No global assembly, membership verdict, count, score, or halt conclusion
is emitted by this certificate.

