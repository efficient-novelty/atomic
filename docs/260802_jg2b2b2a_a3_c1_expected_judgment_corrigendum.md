# JG2b2b2a-A3-C1 Expected-Judgment Wire Corrigendum

Date: 2026-08-02

Status: **A3-C1 FROZEN 2026-08-02; A4-R2d ACTIVE.** This corrigendum closes one
representation omission in the frozen A3-O support contract and was
independently audited with A4-R2c. It
does not change a constructor family, carrier, edge tag, theorem subject,
realization predicate, or executable authority.

## 1. Dependency cut

The normative order is

```text
A3-O base -> A4-R2a -> A4-R2b -> A3-C1 -> A4-R2c.
```

A4-R2a consumes only the base A3-O contract and defines `TypeIdV1` and
`DefinitionRulePathV1`; A4-R2b closes the base tag-4
`CarrierIndexValueV1`. A3-C1 then uses those already defined acyclic types to
close the expected-judgment wire. Reading A3-C1 back into either R2a or R2b is
forbidden; there is no A3/A4 type cycle.

## 2. Omission and exact correction

A3-O section 4.2 requires dependent-prefix, operation, contract, and
obligation records to emit a structural dependency even when no nested
`Global` or `Var` exists. Base `ExpectedJudgmentV1` tags `0..7` describe term,
carrier, operation-ordinal, occurrence, and normalization judgments, but do
not encode either a pure typed record or the complete identity of a required
static construction rule. Reusing `TypeFormation` or `OperationOrdinal` would
misstate that judgment.

The effective post-R2b V1 sum is therefore exactly:

```text
ExpectedJudgmentV1 ::=
  TypeFormation=0
| HasType=1(normalized_type:Term)
| DefinitionallyEqualAt=2(normalized_type:Term)
| ContextPrefix=3(normalized_context:NormalizedContextV1,
                  prefix_ordinal:u32)
| CarrierSlot=4(carrier_index:CarrierIndexValueV1,
                field_ordinal:u32)
| OperationOrdinal=5(grammar_path:FullLocalPathV1,
                     operation_ordinal:u32)
| StructuralOccurrence=6(OccurrenceId14V1)
| NormalizationSubject=7(NormalizationRuleV1)
| StructuralRecord=8(type_id:TypeIdV1)
| ConstructionRule=9(rule_path:DefinitionRulePathV1).
```

Tags `0..7` and their payload bytes are unchanged. Tags 8 and 9 are appended;
there is no renumbering and no eleventh variant. `StructuralRecord` requires a
nonzero type ID equal to the reached record's exact schema type.
`ConstructionRule` retains the full definition path; a slot-local ordinal or
an obligation path cannot substitute for it.

## 3. Support use

For every A3-required record edge, the exact typed dispatch selects one of the
ten judgments above. In particular, R2c obligation traversal emits one leading
`ObligationPremise` slot for every obligation record:

- a projected term/carrier/occurrence row uses the applicable unchanged tag
  `0..7`;
- a pure descriptor row uses `StructuralRecord` with its exact type; and
- a construction requirement uses `ConstructionRule` with the byte-identical
  `DefinitionRulePathV1` stored by that requirement.

Nested term-bearing values then emit their own ordinary dependency slots in
the frozen A3 order. The leading record slot is never suppressed merely
because that nested traversal is empty.

## 4. Authority boundary and audit

A3-C1 changes only the expected-judgment sum and the structural-obligation
dispatch made necessary by the already frozen A3 requirement. A4-R2c froze
this corrigendum only after its two independent release audits confirmed:

1. the dependency order above is acyclic;
2. tags `0..7` are byte-for-byte the A3-O base variants;
3. tags 8 and 9 have exactly the displayed payloads;
4. every obligation record emits exactly one leading structural slot; and
5. no proof, receipt, classifier, profile selection, or execution authority is
   introduced.
