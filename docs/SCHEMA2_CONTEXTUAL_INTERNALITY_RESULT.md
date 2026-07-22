# Schema2 contextual internality result

**Date:** 2026-07-21. **Status:** the adopted motive-sensitive contextual
rule is implemented at term level for an explicitly declared ambient
telescope. The frozen raw-candidate payload does not yet carry that
declaration.

The create-new artifact is
`docs/schema2_contextual_internality_v1.json`, with result digest
`blake3:298a9e705419c8aac9cbb567c81c2dd22e2df986aeee1c1905935cd052386367`.
Definition replay succeeds.

## Term-level rule

An opaque declaration token binds an ordered ambient telescope to an exact
candidate hash. Every motive is checked B15-formable. The contextual token
then replays the frozen derivation, checks each ambient occurrence against
its declared motive, rejects uncertified or non-prior fields and charged
`PathCon`, and specializes the judgment at two registered closed Internal
probes. Both specializations must replay under the closed ambient-former
rules. The token mints no credit, anchor, orbit, or family.

The declared control

```text
[Univ, Lam(App(Var(3), App(Var(1), Lib(15))))]
```

has one inferred ambient parameter. Its derivation records `Var(1)` as
`ambient-param-1` and `Var(3)` as `local-var-1`. With the explicit motive
`Type -> Type`, it earns contextual `Internal` with `nu = 0`; both closed
specializations replay.

## Correction to the v7 witness

For the sealed v7 witness

```text
[Univ, Lam(App(Var(3), Lib(15)))]
```

the frozen derivation records `Var(3)` as `local-var-1`, not an ambient
parameter. The inferred `ambient-param-1` is unused. Consequently this
witness earns `Internal` by the existing per-clause weakening/erasure
inverse theorem, with erased base

```text
[Univ, Lam(App(Var(2), Lib(15)))]
```

and `nu = 0`. The contextual issuer correctly refuses to relabel it as a
live contextual dependency.

## Remaining interface boundary

`pen_core::telescope::Telescope` serializes only clauses. It contains no
ordered ambient-motive field. The same control therefore remains a named
obstruction when passed through the raw classifier without its separate
declaration token. This is recorded rather than repaired by inferring a
motive from a desired classification.
