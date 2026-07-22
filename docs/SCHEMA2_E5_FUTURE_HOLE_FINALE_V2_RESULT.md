# E-5 future-hole finale v2 result

**Date:** 2026-07-22. **Status:** **EXECUTED — BLOCKED BY F-FH4.**

The create-new run is recorded in
`docs/schema2_e5_future_hole_finale_v2.json` with result digest
`blake3:9f7f9361a7111f5937156921bef9604d637fe0c23d1ebfccd1e9c9528c9243bf`.
Definition replay succeeds. The certificate deliberately issues no semantic
`O(16)`, F1 verdict, T-BF2 authorization, bridge authorization, or halt
claim.

## Result

The full Stage-16 A3 inventory still contains 89 operational instances.
Future-hole v2 produces:

- 16 of 17 unary registrations with exact declaration, exact-body source,
  and deterministic replay evidence;
- one replayable named registration gap,
  `a3_v2_parametric_internality_failed`, classified as **F-FH4**;
- all 13 historical structural registrations;
- all 13 clause-4-prime structural realizations against the authoritative
  Phase-5b fillers;
- all 13 wrong-provider controls rejected;
- all 13 charge-swap controls rejected;
- the full historical focus projection, including the Stage-3-to-4
  jurisdiction wrinkle, reproduced from the instance grammar.

Consequently F-FH1 passes: every historical structural discharge in the
coarse O-ladder re-derives as specialization of its registered open
judgment. This is a real positive result. It does not suffice to complete
E-5 because the Stage-16 demand partition remains incomplete.

The Stage-16 membership table has 88 rows rather than 89. Seventeen instances
remain underdetermined: the one missing unary row plus the 16 accepted unary
rows whose exact-source evidence has not yet been promoted to a total
specialization theorem. Therefore `D` is not partitioned, semantic `O(16)`
is unset, and F1 is not executed.

## Exact falsifier

The failed family is the historical `eventually_eliminator` presentation

```text
Gamma = (p1 : Type, p2 : Type)
Gamma |- Lam(App(Eventually(Var(1)), Var(2)))
```

The exact contextual audit rejects the first parameter's use as a function
with its declared `Type` motive. This is not safely repairable by merely
allowing a stuck head. The certified assignment

```text
p1 := Lib(14)
p2 := Univ
```

would specialize the body to

```text
Lam(App(Eventually(Lib(14)), Univ))
```

and the frozen kernel rejects that term with `BareUnivArgument`. Thus the
advertised `[Type, Type]` schema is not total under motive-typed
specialization. Accepting it would violate F-M1; silently excluding the
`Univ` image would violate F-M2.

The audit also found the same general defect in the current motive-v2 source
domain: `App(Lib(15), Var(1))` under `Var(1) : Type` is issuable, but its
`Var(1) := Univ` specialization fails. Motive-v2 is fail-closed and forges no
evidence, but it is presently a partial exact-specialization checker, not a
released total theorem over every issued contextual source. The certificate
therefore records all 16 otherwise accepted unary rows as totality
provisional.

## Lawful successor

Two versioned repairs are mathematically coherent; neither is implicit in
the adopted v1 definition:

1. Add dependent ambient telescopes, so the second hypothesis can carry a
   motive such as `El(Eventually(p1))`, together with sequential dependent
   motive substitution and assignment checking.
2. Add a genuinely typed `Eventually` eliminator rule whose premise and
   result motives make the application total.

Before either semantic extension, the existing theorem layer can be made
honest by issuing a replayable stable-source predicate: at every `App(f,a)`,
neither immediate child may be a substituted ambient-parameter leaf. That
restriction also needs a normalization-fuel composition proof. It will
exclude the falsifying family rather than complete it.

## Gate disposition

| gate | result |
| --- | --- |
| F-FH1 historical discharge regression | passed, 13/13 |
| F-FH4 | triggered |
| semantic `O(16) = empty` | not issued |
| F1 | not executed |
| E-5 complete | false |
| T-BF2 authorized | false |
| bridge authorized | false |
| halt claim | not issued |

No bridge or final certificate was run.
