# Schema2 motive-parametric coherence result

**Date:** 2026-07-21. **Status:** the adopted
`motive-parametric-instantiation-coherence-v1` theorem is implemented,
source-bound, emitted create-new, and replayed. It resolves the exact global
E-4 v9 survivor with zero marginal credit.

The create-new artifact is
`docs/schema2_motive_parametric_coherence_v1.json`, with result digest
`blake3:bc365d38580fe3f62a44e87eee76169d7429255869a6320361b19682a01c466d`.
Definition replay succeeds.

## Generic substitution theorem

The theorem is a structural substitution induction over the six adopted
Internal closure rule kinds:

1. inductive projection;
2. guarded weakening/erasure;
3. structural lambda;
4. transparent ambient former;
5. certified-field dereference; and
6. motive-sensitive contextual internality.

It is generic over B15-formable ambient contexts, motives, and motive-typed
closed assignments. The implementation traverses the full expression
constructor inventory and includes the empty assignment and vacuous
uninhabited-motive cases. Adding a closure-rule variant without adding its
specialization case is a compile-time non-exhaustive match; a mutated proof
projection is rejected by replay.

Registered probes remain regression examples only. They are not premises of
the theorem, and no inhabitedness test or motive-grammar filter is applied.
Consequently F-M2 does not shrink the v9 domain: its finite six-node motive
grammar retains the unfiltered syntactic upper bound of 914,612.

## v9 survivor

The exact v9 witness with motive

```text
Type -> Element(Univ)
```

passes the existing motive-typing audit. Its predecessor failure was exactly
the absence of registered probes, not a typed mismatch. Generic substitution
therefore discharges the remaining instantiation-coherence obligation and the
successor classifier returns `Internal` with `nu = 0`.

Other contextual failures are not blanket-relabelled. In particular, motive
mismatches still fail as named typed exclusions. This passes F-M1: no
substitution-unstable closure rule was found.

## Verification

```text
cargo test -p pen-type motive_parametric_coherence -- --nocapture
cargo test -p pen-schema internal_classifier_branch_v9 -- --nocapture
cargo test -p pen-schema motive_parametric_coherence_certificate -- --nocapture
cargo run -q -p pen-schema --example schema2_motive_parametric_coherence -- replay docs/schema2_motive_parametric_coherence_v1.json
```

All focused tests and artifact replay pass.
