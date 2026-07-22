# SCHEMA2 enlarged-sub-basis M1 sweep

**Date:** 2026-07-20  
**Status:** sweep complete; no new M1 verdict  
**Artifact:** `docs/schema2_m1_sweep_v1.json`

## Outcome

The v4 enlarged proven sub-basis was swept against the exact registered
Step-8 left-unit target:

```text
Pi x:El(S3). Path El(S3) (mu(base,x)) x
```

No noncircular generating derivation was found. M1 therefore issues no new
`Generated` verdict. This is not an `Independent` verdict: the Step-8
left-unit row remains pending.

```text
Generated before sweep  1
Generated after sweep   1
Pending before sweep    5
Pending after sweep     5
```

The four Stage-1 carrier roles remain pending together with the Step-8
left-unit coherence.

## Basis actually swept

The artifact strictly replays `docs/schema2_v4.json` and binds:

- nine replayed ordinary normalization/naturality tokens;
- all eight publicly typable cubical constructor actions;
- the already-final Step-8 cell-action `Generated` token.

The resulting basis size is 18 and its digest is:

```text
blake3:3fa9fd6d77721526328be804ff859c07996313d1832400ada03a991d2cdb6af5
```

## Why the candidate does not generate

The decisive distinction is between transporting a law and deriving the law.

The `PostPathOperation` naturality token transports `mu` as an operation. It
preserves the outer constructor and produces another operation; it does not
produce a `Path` asserting a unit equation.

The `PostPathCoherence` naturality token proves that an already-supplied
coherence is stable under substitution and normalization. Specialized to the
Step-8 target, its rule has the form:

```text
Step8LeftUnitCoherence -> Step8LeftUnitCoherence
```

It cannot seed the target without assuming the target, so the sweep marks this
route circular.

The public cubical induction likewise acts on already-typed cubical terms. It
preserves their witness provenance and does not introduce a left-unit path
term. The exact registered Step-8 data contains a typed left-unit *type*, but
no registered left-unit term from which cubical action could start.

By contrast, the cell-action verdict remains generated because the existing
parent-cube rule has noncircular premises:

```text
TypedStep8ParentOperation + RegisteredStep8Cell
  -> GeneratedStep8CellAction
```

That rule has no analogue producing the left-unit equality.

## Fixed-point audit

The finite rule closure contains:

```text
typed_step8_parent_operation
registered_step8_cell
generated_step8_cell_action
generic_ordinary_schema
generic_post_path_coherence
public_cubical_term
```

It does not contain `step8_left_unit_coherence`. Of the 19 swept rules, the
only one with that conclusion requires `step8_left_unit_coherence` as a
premise. Mutation tests reject a forged target reachability flag, a forged
second `Generated` verdict, a forged pending-set shrink, and an illicit
`Independent` verdict.

## Interpretation

Naturality of a binary operation does not by itself imply that a chosen point
is its unit. A further noncircular generator is needed, such as an explicitly
typed unit-coherence constructor/term together with a theorem that the
registered Step-8 row is its canonical derived action. Adding such a generator
would be new grammar/construction work, not a consequence of the present v4
sub-basis.

The named remaining obstruction is:

```text
E34_M1_LEFT_UNIT_REQUIRES_A_NONCIRCULAR_PATH_COHERENCE_GENERATOR
```

## Certificate identifiers

```text
schema/result version  schema2-e34-enlarged-sub-basis-m1-sweep-v1
result digest          blake3:aec9896de1fdbc665a4c231fb2365ffc93bf3d08f6829695640b4c9174904838
file SHA-256           bc1817ad9348fd8e7d25ec518822851b673fdc14822a452f9b916ca1de0e5695
```

## Verification

```powershell
cargo test -p pen-schema e34_m1_sweep -- --nocapture
cargo run -p pen-schema --example schema2_e34_class -- replay docs\schema2_v4.json
cargo run -p pen-schema --example schema2_e34_m1_sweep -- replay docs\schema2_m1_sweep_v1.json
```

