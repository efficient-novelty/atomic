# Schema2 global E-4 v5 assembly result

**Date:** 2026-07-20. **Status:** structural lambda witness resolved; an exact
application-body Unknown survives; class exhaustion remains unproved.

The create-new artifact is `docs/schema2_global_e4_assembly_v5.json`, with
result digest
`blake3:7519a0e7e06432b6265e268de5efd5d95059ffa12e4a952766716051bb4ffbd0`.
Definition replay succeeds.

## Resolved predecessor witness

The v5 assembly rechecks

```text
[Univ, Lam(Univ)]
```

as an exact raw-catalog member. It now lands in the structural-lambda
`Internal` branch: one clause is inherited, one clause carries constructor
evidence, the body and lambda constructor both replay, and marginal `nu = 0`.

## Next exact fail-fast witness

The rerun then tests

```text
[Univ, Lam(App(Lib(14), Lib(15)))].
```

It is an exact closed raw-catalog member and elaborates. Its lambda body,
however, ends in the `app-stuck` derivation. The structural-lambda token
refuses it with

```text
unsupported structural body App(Lib(14), Lib(15)) under derivation rule app-stuck
```

The existing classifier therefore retains F-G4 and the named
`INDUCTIVE_TELESCOPE_INTERNALITY_NOT_PROVED_FOR_CANDIDATE` gap.

## Lawful stopping point

Because an exact Unknown survives, class exhaustion and global E-4
completeness remain false. The five membership verdicts, E-2b/F-Q2, F-T1,
E-5/F1, classifier bridge, and fork remain unexecuted.

The next decision is whether application belongs to the zero-credit structural
`Internal` closure. A sound positive rule would need separately replayable
`Internal` premises for the function and argument, application congruence or
naturality, preservation of the typed result, and no credit unless the result
is an independently exported demand orbit. E-4 must rerun create-new after
that decision or proof.
