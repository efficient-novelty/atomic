# BOUNDARY-AUDIT v1: attachment-rule decision evidence

**Execution date:** 2026-07-19  
**Artifact:** `docs/boundary_audit_v1.json`  
**Schema:** `boundary-audit-v1`  
**Result digest:**
`blake3:76c389766a5c54582d1e2ffe9942db5c53cd3764137cfc62254b9857e1f1c435`

## Outcome

This audit produces decision evidence; it does not select an attachment rule.
Its fail-closed result is:

1. V1's source-bound realizer types incumbent constant-boundary presentations
   whose shapes agree with the supplied S1, S2, and S3 diagrams. The implicit
   cubical `Base` is not derivationally linked to each historical point clause,
   so exact historical-constructor typing remains open even in those rows. V1
   rejects the intended Trunc endpoint diagram with
   `V1RequiresConstantBoundary`; its count of 2 there comes only from a
   separately labelled constant-loop surrogate.
2. V2 structurally accepts all four supplied boundary diagrams. The checker
   validates owners, face inventories, restriction inventories, and pairwise
   overlap labels. It does not elaborate the annotation strings as terms or
   construct boundary-aware transports, so this is not yet a proof that V2
   types any historical constructor.
3. V3 is underdetermined. `PathCon` records a dimension but no boundary, and
   distinct checked diagrams project to the same sealed trace.

Thus no implemented variant currently establishes typed historical fit for
all four constructors. V2 remains a uniform structural proposal whose
term-level proof obligations are explicit.

| variant | historical evidence | general basis result | frozen-kernel derivability |
| --- | --- | --- | --- |
| V1: implicit constant boundary | Typed incumbent presentations have the supplied constant shapes at S1, S2, S3, but lack historical point binding; direct rejection of the intended Trunc boundary | Typed incumbent presentation `1 + d²` in the constant-boundary domain | Neither forced nor forbidden; an explicit conditional sidecar axiom |
| V2: declared boundary proposal | All four supplied diagrams pass structural checks; historical term typing remains open | Raw combinatorial proposal `1 + d²`, conditional on uncharged boundary premises and a future term realizer | Neither forced nor forbidden; boundary syntax is absent from the frozen grammar |
| V3: trace-derived | Underdetermined for every row at the byte level | No trace-derived formula | Not derivable from the dimension-only sealed projection |

No semantic novelty value, threshold comparison, or later-candidate score was
computed.

## B-1: historical constructor audit

The feature-gated diagram checker requires:

- exactly two codimension-one faces per axis;
- owner equality for every point annotation and face annotation;
- the expected remaining-axis context for every face;
- a complete restriction inventory; and
- identical declared restriction labels on every pairwise face overlap.

These are structural checks. They do not parse or type-check the strings used
to name contexts, points, faces, or restrictions.

| constructor | supplied intended boundary | V1 | V2 | V3 |
| --- | --- | --- | --- | --- |
| Step 5 / S1 | `loop(0)=base`, `loop(1)=base` | typed incumbent constant presentation; shape agrees, historical base binding open | structural diagram accepted; term typing open | trace underdetermined |
| Step 6 / Trunc | in `A:Type; x,y:Trunc(A)`, `squash(x,y)(0)=x`, `squash(x,y)(1)=y` | intended diagram directly rejected; constant surrogate replays separately | structural diagram accepted; term typing open | trace underdetermined |
| Step 7 / S2 | all four codimension-one faces are `base` | typed incumbent constant presentation; shape agrees, historical base binding open | structural diagram accepted; term typing open | trace underdetermined |
| Step 8 / S3 | all six codimension-one faces are `base` | typed incumbent constant presentation; shape agrees, historical base binding open | structural diagram accepted; term typing open | trace underdetermined |

The S1/S2/S3 equations are supplied semantic annotations based on their
historical sphere-HIT reading. The Trunc endpoints are the concrete historical
mismatch already exposed by HIST-CERT. None of these boundary payloads occurs
in the sealed `PathCon` bytes. The V1 realizer's `PointExpr::Base` carries the
formed owner but no point-clause identity, which is why constant-shape
agreement is not reported as exact historical typing.

For Trunc, the machine first calls the V1 attachment rule with the intended
endpoint-dependent diagram and records the exact rejection. Only then does it
issue and replay a V1 attachment for a constant diagram so that the raw
historical count can be compared without disguising the semantic mismatch.

For V2, accepting the schematic declarations `x` and `y` proves only that
their labelled diagram is structurally coherent. Naturality of a typed squash
family under substitution is still unproved.

## B-2: basis replay

| constructor | recorded | V1 | V2 | V3 |
| --- | ---: | ---: | ---: | ---: |
| S1 | 2 | 2, typed incumbent presentation | 2 raw keys; typing open | undefined |
| Trunc | 2 | 2 only for the rejected-boundary surrogate | 2 raw keys; typing open | undefined |
| S2 | 5 | 5, typed incumbent presentation | 5 raw keys; typing open | undefined |
| S3 | 10 | 10, typed incumbent presentation | 10 raw keys; typing open | undefined |

The V1 counts are checked against the incumbent source-bound opaque term
tokens. The V2 counts are produced by a boundary-attachment-bound enumeration
of beta, principal-transport, and ordered-naturality keys. V2 has no
variant-specific term tokens in this fragment. Every row also records
`semantic_exhaustiveness_proved = false`.

A single variant, V1, reproduces all four recorded *raw* counts, but its
Trunc row is a surrogate and its other rows lack base-to-point-clause
witnesses. No single implemented variant establishes exact typing for all four
historical constructors. The raw cardinalities therefore do not adjudicate V1
versus V2.

## B-3: general-dimensional formulas

For the incumbent V1 presentation, the machine constructs the disjoint key
classes

[
  {\beta}
  \;\sqcup\;
  {T_a : a\in\mathrm{Fin}(d)}
  \;\sqcup\;
  {N_{a,b} : a,b\in\mathrm{Fin}(d),\ a\ne b},
]

and verifies

[
  1+d+d(d-1)=1+d^2.
]

For V1, these keys are placed in exact bijection with the incumbent typed
tokens. For V2, the same enumeration is only a raw combinatorial proposal:
the present implementation has no term realizer proving that the principal
transport and naturality keys denote well-typed boundary-aware operations.

The V2 proposal is additionally conditional on treating the declared
boundary and future motive-boundary data as premises rather than independent
computation exports. If boundary components are separately charged, the
honest form is

[
  1+d^2+c(b),
]

where `c(b)` depends on the boundary presentation and is not determined by
dimension.

There is no V3 general-dimensional formula. Applying the V2 enumeration to a
finite semantic annotation would be V2 evidence, not a V3 derivation from the
trace.

## B-4: kernel derivability

The frozen grammar represents a path constructor as `Expr::PathCon(u32)`.
Shallow elaboration produces `KernelTy::PathDecl { dimension }`. It stores no
base point, endpoint term, face family, parameter context, or overlap
equation.

- V1 is not derived from that syntax. Its constant-boundary behavior is an
  explicitly versioned, theory-relative attachment axiom.
- V2 is not derived from that syntax. The feature-gated code demonstrates a
  structurally coherent declared-diagram extension, but not its term-level
  soundness.
- V3 is not recoverable from that syntax. For each audited historical owner,
  the machine checks two distinct declared diagrams with the same trace
  projection.

Neither frozen law chooses among the three theories.

## Implementation and preservation

The new code is default-off behind `pen-type/boundary-variants`, forwarded by
`pen-eval/boundary-audit`. The audit invokes the incumbent V1 realizer and
replay APIs directly. Preservation of the pre-existing V1 and HIST-CERT JSON
artifacts is checked externally by byte hashes and replay; the new artifact
does not self-attest that source files were untouched.

The result contains four supplied boundary-diagram audits, twelve
constructor-by-variant rows, three formula audits, three derivability audits,
the comparison table, and a decision-neutral adjudication list.

## Replay and mutation falsifiers

Replay reconstructs the predecessor signatures, formed owners, structural
diagrams, attachments, raw presentations, noninjectivity evidence, and result
digest. The mutation battery rejects:

- every historical fit-verdict flip;
- every formula copy changed in the formula table, historical rows, or
  comparison table;
- a boundary-diagram digest mutation;
- a false promotion of V3 to trace-recoverable;
- an attachment-token hash mutation;
- deletion of a historical row; and
- a semantic mutation followed by recomputation of an outer digest.

Reproduction:

```powershell
cargo test -p pen-type --features boundary-variants boundary_variants --lib
cargo test -p pen-eval --features boundary-audit boundary_audit --lib
cargo run -p pen-eval --features boundary-audit --example boundary_audit -- replay docs/boundary_audit_v1.json
```

## Remaining proof obligations

Before V2 can count as historical typing evidence, it needs:

1. elaborated interval/cofibration contexts and typed point, face, and overlap
   terms;
2. typed `coe`/`hcom` operations and a motive-typed `PathCon` eliminator;
3. weakening, substitution, and naturality for the full parameterized Trunc
   squash family;
4. typed realizers for the proposed beta/transport/naturality keys; and
5. independence and exhaustiveness before the raw presentation is used as a
   semantic classifier or novelty basis.

The charging convention for boundary components must also be fixed before a
dimension-only V2 formula can become a theorem.

Independently, exact V1 historical typing for S1, S2, and S3 requires a
replayable typed witness identifying the implicit cubical base with the
corresponding historical point clause.

## ADJUDICATION REQUIRED

No option is selected or ranked by this audit.

1. **Retain V1.** S1, S2, and S3 keep their source-bound typed incumbent
   presentations and constant-shape agreement, but historical point binding
   remains open; Trunc remains a surrogate, and the TDC/HIST-CERT
   conditionality remains load-bearing.
2. **Develop V2 into a typed rule.** The structural proposal uniformly accepts
   the four supplied diagrams, but term elaboration, typed cubical operations,
   and eliminator evidence must be completed before it can discharge the
   historical exposure. Its current `1+d²` result is a raw conditional
   proposal.
3. **Require a genuinely trace-derived V3.** The current trace is
   insufficient; additional sealed boundary syntax or explicit
   per-constructor semantic axioms are required, and no general-dimensional
   formula is presently derivable.
4. **Defer adjudication.** All affected conclusions remain theory-relative,
   with the Trunc mismatch, the V2 typing gap, and V3 underdetermination
   recorded as open blockers.
