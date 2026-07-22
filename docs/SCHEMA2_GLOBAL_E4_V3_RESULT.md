# Schema2 global E-4 v3 assembly result

**Date:** 2026-07-20. **Status:** projection witness resolved; next exact
guarded Unknown found; class exhaustion remains unproved.

The create-new artifact is `docs/schema2_global_e4_assembly_v3.json`, with
result digest
`blake3:2c71ad9af7cd8934e05941961247c76f217ac1386546d8dba90f44ef79cabb24`.
Definition replay succeeds.

## Progress

The v3 assembly replays the inductive successor and rechecks

```text
[Univ, Lam(Var(1))]
```

as an exact raw-catalog member. It now lands in `Internal` with zero credit:
clause 1 is a constant projection through already certified clause 0. The
predecessor F-I2/F-G4 witness is therefore resolved in this successor.

## Next exact Unknown

Fail-fast enumeration then reaches

```text
[Univ, Lam(Var(3))]
```

Under the frozen absolute-level convention this is the identity in a context
with one ambient parameter. It is an exact raw-catalog member and elaborates,
but the whole telescope is guarded by ambient arity 1. Neither clause carries
the required weakening/erasure inverse certificate. The Internal successor
therefore returns the named gap

```text
INDUCTIVE_INTERNALITY_GUARDED_WEAKENING_ERASURE_INVERSE_LAWS_MISSING
```

and the classifier retains F-G4. Its survivor derivation is
`blake3:67e9047e57caa642e55f51311c282905005401990a3cc32c3d12576fffa427d3`.

## Lawful stopping point

Because at least one exact Unknown survives, the certificate records class
exhaustion, global E-4 completeness, and authorization of the five pending
memberships as false. The membership verdicts, E-2b/F-Q2, F-T1, E-5/F1,
bridge, and fork remain unexecuted.

The next proof obligation is candidate-clause-level guarded internality for
the one-ambient-parameter identity family: construct and replay weakening and
erasure maps and both inverse laws. Only a successful successor proof licenses
another create-new global E-4 rerun.

