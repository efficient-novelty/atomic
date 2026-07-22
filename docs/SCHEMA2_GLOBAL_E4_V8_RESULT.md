# Schema2 global E-4 v8 assembly result

**Date:** 2026-07-21. **Status:** v7's witness is resolved, but an exact live
context candidate exposes a raw-surface declaration gap. Exhaustion remains
unproved.

The create-new artifact is `docs/schema2_global_e4_assembly_v8.json`, with
result digest
`blake3:76ccd16c1f9318b5705650d0279d5681ead849d3d0fe05b959c39f5e56d681fd`.
Definition replay succeeds.

## Resolved predecessor

The v8 assembly re-elaborates the previous witness rather than trusting its
informal label. Its application head is `local-var-1`; the inferred ambient
parameter has zero occurrences. The candidate is the weakening image of a
closed v7-Internal base, both inverse laws replay for both clauses, and it
earns `Internal` at `nu = 0`.

## Next exact fail-fast witness

The raw-catalog member

```text
[Univ, Lam(App(Var(3), App(Var(1), Lib(15))))]
```

has exactly one ambient parameter. Here the derivation genuinely contains
both `ambient-param-1` and `local-var-1`. With an explicit B15-formable
`Type -> Type` motive declaration, the adopted contextual proof and both
closed specialization probes replay successfully.

Without that separate declaration, the raw classifier retains the named
gap

```text
RAW_TELESCOPE_AMBIENT_MOTIVE_DECLARATION_MISSING
```

because the enumerated `Telescope` value has no place to store Gamma's
motive. Supplying or inferring one inside the classifier would make the
classification depend on evidence absent from the raw candidate.

## Lawful stopping point

The v8 lower bound on surviving `Unknown`s is one. Therefore
`no_unknown_survives`, class exhaustion, global E-4 completion, and the five
membership authorizations remain false. F-Q2, certified historical totals,
F1, the bridge, and the fork were not executed.

The next versioned step is representational: introduce a raw-candidate
wrapper carrying an ordered, B15-formable ambient telescope; bind enumeration,
hashing, and classifier replay to that wrapper; then rerun create-new. Motives
must be declared independently of classification outcomes.
