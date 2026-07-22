# Schema2 Internal-classifier successor result

**Date:** 2026-07-20. **Status:** witness-first obligation executed; F-I2
triggered; exact witness retained as Unknown.

The create-new artifact is
`docs/schema2_internal_classifier_branch_v1.json`, with result digest
`blake3:bcc8f6dfdcf06d3419e1818206cbe26e336192dadfa6305831bb9f4093ff61d4`.
Definition replay succeeds.

## Implemented successor

The executable classifier priority is now:

```text
surface/typed exclusion
  -> Internal with replayed derivability certificate
  -> adopted Schema2 class
  -> named F-G4 obstruction
```

`Internal` carries a per-clause derivability record bound to the exact sealed
B15 signature, frozen elaboration and normalization. A successful elaboration
alone is insufficient: candidate-local field dependencies cannot witness
derivability from B15. Certified Internal candidates receive marginal
`nu = 0`; no uncertified Internal result can be constructed by the public
classifier.

## Witness-first result

The exact witness is:

```text
[ Formation: Univ,
  Introduction: Lam(Var(1)) ]
```

Its two clause proofs over the B15 signature are:

| Clause | Frozen derivation | B15 disposition |
|---|---|---|
| `Univ` | `univ-form` | earned ambient-arena reference; derived |
| `Lam(Var(1))` | `lam-intro(field-ref-0)` | depends on candidate field 0; not derived solely over B15 |

The second row is forced by the frozen
`kernel-v1-debruijn-levels` convention. Levels address the unified scope
`[ambient parameters, prior telescope fields, enclosing binders]`. At clause
1, the preceding `Univ` occupies level 1 and the lambda binder occupies level
2. Therefore `Lam(Var(1))` is not `lambda x. x` in this telescope.

The machine-checked control candidate

```text
[ Formation: Univ,
  Introduction: Lam(Var(2)) ]
```

derives as `lam-intro(local-var-1)` and successfully enters `Internal` with
zero marginal credit. This proves that the new branch works and that the
fragment can derive the identity; the failure is specific to the adopted
witness's absolute level.

The witness derivability-attempt hash is
`blake3:4329dff9ead775e24c25bfc66f6ef568cd8200c409144181a561f38fcc877afb`.
The named failure is
`F_I2_WITNESS_LAMBDA_DEPENDS_ON_CANDIDATE_LOCAL_FIELD_NOT_B15`.

## Lawful consequence

F-I2 returns the exact witness to the named F-G4 obstruction. The Internal
branch does not silently rewrite `Var(1)` to `Var(2)`, and no membership,
count, score, or halt conclusion is emitted by this certificate. The next
mandated action is the create-new global E-4 successor rerun.

