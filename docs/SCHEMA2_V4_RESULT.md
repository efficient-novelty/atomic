# SCHEMA2 E3/E4 class-indexed induction result (v4)

**Date:** 2026-07-20  
**Status:** operational induction complete; global intended-Schema2 E3/E4 incomplete  
**Certificate:** `docs/schema2_v4.json`

## Result

The requested induction was executed over the entire constructor inventory
that is presently defined by the operational kernels:

- all **9/9 registered ordinary schema constructors** have replayed typed
  normalization/naturality tokens under a genuine non-identity ambient
  substitution;
- all **8/8 cubical constructors accepted by the public cubical context** are
  covered by recursive dimension-action induction;
- the cubical checks include genuinely dependent `coe`, `hcom`, two typed tube
  faces, and one pairwise tube-overlap obligation;
- the registered Trunc endpoint bundle replays with its exact two-element
  basis;
- a new Agda mirror checks under `--safe --without-K` with no postulates.

This is not the global E3/E4 completeness theorem. Four cubical syntax forms
use the kernel's private endpoint-premise context, and the intended grammar
still lacks adjudicated constructors/rules for Modal, Synthesis, several Map
and Axiomatic forms, and total raw-candidate classification. P1 therefore
continues to forbid an `Independent` verdict, E2b, stage counts, and a halt
claim.

## Machine result

The strict replay returned:

```text
valid                                      true
predecessor_replayed                       true
ordinary_constructor_count                 9
ordinary_registry_complete                 true
public_cubical_constructor_count            8
public_cubical_induction_complete           true
private_endpoint_premise_constructor_count  4
every_class_complete                        false
full_e4_complete                            false
e2b_executed                               false
forbidden_outputs_withheld                 true
```

The certificate identifiers are:

```text
schema        schema2-e3-e4-class-induction-successor-v4
result digest blake3:c5b598eba49f66cb7f845fdab43ab217a40b7d9ce2ec971a91989712711fcc4a
file SHA-256  ffe5e195e136aeb3f142ed4b30b6cabcbb36988347b71f9c1deae841d675e56a
```

The predecessor `docs/schema2_v3.json` was replayed before the v4 token was
issued. Its byte-pinned sources remain valid.

## Ordinary normalization/naturality theorem

The Rust issuer forms a source schema for each registered constructor, checks
it, normalizes it, applies the same genuine typed substitution to every type
and term field of its semantic interpretation, forms and normalizes the target
schema, and compares the two paths. It also checks constructor identity,
family presentation/disposition, and target support reconstruction.

The tested substitution is not a renaming:

```text
A |-> Trunc(B)
B |-> B
a |-> |b|
```

The complete operational inventory is:

1. fresh formation;
2. point/unit introduction;
3. path-constructor introduction;
4. recursor;
5. inductor;
6. Trunc parametric action;
7. post-path operation;
8. post-path coherence;
9. cell action.

All nine tokens replay. The aggregate ordinary derivation hash is:

```text
blake3:c4e7d0683248b9da761d1e5452fc7f105ee70f759e36851a94c1702195f8f4dc
```

This proves exhaustiveness only relative to `OrdinarySchemaKind::ALL`. It does
not prove that this nine-kind registry exhausts the intended semantic grammar
from the frozen open-problem specification.

## Dependent cubical-action induction

The Rust proof traverses the kernel's `CubicalTerm` enum by an exhaustive match.
For every interval variable in each typed exemplar it tests both faces and all
in-context variable images. On every action it:

1. substitutes the cubical context, source type, and source term;
2. re-infers the substituted source type;
3. normalizes after substitution;
4. substitutes the already-normalized term and re-infers its type;
5. normalizes that path;
6. requires equal expected types and equal normal forms.

The public inventory covered is:

```text
Point
MotiveBase
PathMethod
PathElim
DimLambda
DimApp
Coe
Hcom
```

The four exemplars produced 17 dimension-substitution witnesses. The coverage
metrics are:

```text
coe nodes                 1
hcom nodes                1
typed tube faces          2
checked tube overlaps     1
registered Trunc basis    2
```

The remaining four enum forms are:

```text
EndpointEvaluationHypothesis
EndpointMethodHypothesis
EndpointPathElim
EndpointElimNeutral
```

They are deliberately not accepted by the public context. The registered
Trunc issuer can type them only through a private premise context. Replaying
that registered bundle is evidence for the two sealed Trunc endpoints, not a
generic induction theorem and not transport of an arbitrary registered bundle
under an arbitrary E1 substitution.

## Class-indexed audit

| Class | Registered ordinary constructors | What v4 proves | Why the class is not complete |
| --- | --- | --- | --- |
| Foundation | formation, point/unit | both normalization-natural | intended grammar and total raw-candidate classifier not exhaustive |
| Former | recursor, inductor | both normalization-natural | intended grammar and total raw-candidate classifier not exhaustive |
| Map | Trunc action, post-path operation | both normalization-natural | natural-transformation and exported-support-action grammar incomplete |
| Axiomatic | post-path coherence | normalization-natural | adjoint/mate/interchange grammar incomplete |
| Modal | none | no constructor can be claimed | typed modal constructors/rules absent |
| HIT/V2 | path constructor, cell action | both normalization-natural; public cubical induction complete | generic endpoint-premise induction and bundle transport absent |
| Synthesis | none | no constructor can be claimed | typed synthesis constructors/rules absent |
| Unknown | none | no constructor can be claimed | rows require typed classification before elimination |

Every class also carries the common named gap
`E34_TOTAL_TYPED_RAW_CANDIDATE_TO_SCHEMA2_CLASSIFIER_NOT_PROVED`.

## Agda theorem boundary

`agda/Schema2E3E4ClassInduction.agda` supplies constructive, postulate-free
proofs for:

- normalization/substitution naturality by induction over all nine ordinary
  constructors and an explicit redex wrapper;
- preservation of the operational constructor kind and class index;
- absence of registered Modal and Synthesis normal constructors;
- dimension action under binders;
- normalization/action commutation for every public cubical constructor,
  including `coe`, `hcom`, and lists of tubes;
- exhaustiveness of the public cubical constructor induction.

The Agda file is a safe proof mirror of the operational syntax. A formal
Rust-to-Agda datatype/semantics correspondence theorem is not claimed. Its
`PublicEndpointPremiseContext` is intentionally empty, recording rather than
hiding the missing public endpoint-premise API.

## Falsifier and phase-order outcome

F-E1 fires as a partial-delivery condition: every intended class remains
incomplete with named gaps. F-P1 does not fire because no forbidden output was
issued. Specifically, v4 issues none of:

- an `Independent` membership verdict;
- an ordinary-family credit;
- a stage count;
- E2b quotient closure;
- global E3 or E4 completion;
- a Genesis halt conclusion.

The next sound step is not to run E2b. It is to expose/adjudicate the generic
endpoint-premise API and finish the missing intended class grammar, followed by
the total raw-candidate exhaustiveness theorem. Only that result can turn the
current per-constructor proofs into global E4 completeness.

## Verification commands

```powershell
cargo test -p pen-schema e34_class -- --nocapture
agda -i agda agda\Schema2E3E4ClassInduction.agda
cargo run -p pen-schema --example schema2_e34 -- replay docs\schema2_v3.json
cargo run -p pen-schema --example schema2_e34_class -- replay docs\schema2_v4.json
```

