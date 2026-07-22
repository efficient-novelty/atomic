# Schema2 global E-4 v7 assembly result

**Date:** 2026-07-21. **Status:** certified-field indirection is resolved; an
exact genuinely guarded `Unknown` survives; exhaustion remains unproved.

The create-new artifact is `docs/schema2_global_e4_assembly_v7.json`, with
result digest
`blake3:2a4509f58c9f79b70fbebc158688eb35f1414ab42ec236fd1f96fe31ab0b61fa`.
Definition replay succeeds.

## Resolved predecessor witness

The v7 assembly rechecks

```text
[Univ, Lam(App(Var(1), Lib(15)))]
```

as an exact raw-catalog member. Its referent's complete `Internal` derivation
replays, its direct judgment agrees after certified refinement, and it enters
the certified-field `Internal` branch at marginal `nu = 0`.

## Next exact fail-fast witness

The rerun then tests

```text
[Univ, Lam(App(Var(3), Lib(15)))].
```

It is an exact typed raw-catalog member with one ambient parameter. Unlike the
v6 survivor, `Var(3)` is not a candidate field: it is a live ambient dependency
inside the lambda body. Consequently neither prior-field certification nor
dereference applies.

The ambient-former token reports

```text
ambient-former closure is closed-only; candidate ambient arity is 1
```

and the dereference token independently reports the corresponding closed-only
refusal. The candidate is also distinct from the registered guarded identity
weakening image: there the inserted parameter is unused, while here it is the
application function. The classifier therefore retains F-G4 and
`INDUCTIVE_INTERNALITY_GUARDED_WEAKENING_ERASURE_INVERSE_LAWS_MISSING`.

## Lawful stopping point

The v7 lower bound on surviving `Unknown`s is one. Thus
`no_unknown_survives`, class exhaustion, global E-4 completion, and the five
membership authorizations are all false. F-Q2, certified historical totals,
F1, the bridge, and the fork were not executed.

The experiment falsifies the claim that indirection was the final generic
mechanism in the current frozen surface: genuinely ambient-dependent families
remain. A positive next rung would require a new motive-sensitive guarded
internality principle; the existing weakening/erasure inverse theorem cannot
apply because the ambient parameter is used.
