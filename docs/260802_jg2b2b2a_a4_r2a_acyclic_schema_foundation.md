# JG2b2b2a-A4-R2a Acyclic Schema, Path, and Ownership Foundation

Date: 2026-08-02

Status: **A4-R2a+C2 AND A4-R2b FROZEN; A4-R2d ACTIVE AT QG3.** The R2a base
froze on 2026-08-02; the append-only theorem-node C2 corrigendum froze on
2026-08-03. This independently audited record repairs the
representation cycles exposed after the A4-R1 freeze and fixes the common
typed-schema metalanguage, scalar codec, path language, root namespace,
ownership discipline, and Rust source-identity input used by the remaining
A4-R2 subgates. It does not yet close the concrete Level-I/Level-II registry,
mint a profile token, or unblock JG2b2b2b.

This record depends normatively on:

- `docs/260802_jg2b2b2a_a4_rc1_release_audit.md`;
- `docs/260802_jg2b2b2a_a4_r1_signature_source_census.md`;
- `docs/260802_jg2b2b2a_adequacy_and_envelope_protocol.md` (A1);
- `docs/260802_jg2b2b2a_indexed_ontology_profiles.md` (A2-O); and
- `docs/260802_jg2b2b2a_a3_ordinary_constructor_schemas.md` (A3-O).

Where this record gives a narrower representation rule than A1, A3-O, or the
rejected A4 RC1, this record controls the ordinary V1 repair. No mathematical
carrier or constructor family is changed.

## 1. The defect and its exact repair

An exact type inventory found three representation cycles:

1. A3-O's quotient key retained theorem subjects that could name
   `FinalActionKey`, although `FinalActionKey` is itself a quotient key.
2. normalization evidence retained preservation of full support while full
   support retained normalization-preservation subjects; and
3. support traversed instantiated field-16 subjects even though those subjects
   were constructed after support.

It also found two non-cyclic but authority-relevant holes:

- `ExpectedJudgmentV1::CarrierSlot(...,canonical_index_bytes,...)` admitted an
  untyped byte string where a full carrier-index value is required; and
- `FormalActionIdV1::ExternalGlobal(GlobalId)` could name a global absent from
  `Sigma(H,o)`, although every bounded action check is required to use that
  exact signature.

Ordinary V1 repairs these defects by projection stratification:

```text
theorem subject syntax -> typed core/projection literals only
normalization          -> SupportProjectionSubjectV1, never OrdinarySupportV1
support traversal      -> SubjectRootProjectionV1, never field-16 wrappers
quotient key           -> subject syntax/projections, never FinalActionKey
action trace endpoint  -> payload/key projection, never ActionImageV1
action image owner     -> one original CandidateEnvelopeV1, never EnvelopeObject
carrier-slot judgment  -> full CarrierIndexValueV1, never arbitrary bytes
bounded action global  -> must resolve in Sigma(H,o), never ExternalGlobal
```

`FinalActionKeyV1` remains convenient mathematical notation for the quotient
key deterministically rebuilt from an original candidate and a normalized
action. It is not a node or literal in the canonical subject syntax.

## 2. Auditable subdivision of A4-R2

A4-R2 is divided without weakening its completion criterion:

1. **A4-R2a -- acyclic schema/path/ownership foundation -- FROZEN
   2026-08-02.** Freeze
   the strata, scalar and composite wire rules, schema DAG, typed paths, root
   namespace, nonrecursive reference discipline, subject-DAG boundary, and
   implementation-source measurement contract.
2. **A4-R2b -- exact Level-I, raw, decoded, built, and early Level-II payload
   schemas.** Fill the complete types rooted at `0xe2`, `0xe4`, `0xe5`, and
   `0xe6`, including every nine-carrier proposal and all seven decoded/built
   variants. Freeze the three early Level-II payload records and the exact
   five-tag `0xe3` sum contract, but do not materialize root `0xe3` yet.
3. **A4-R2c -- exact A1 core/evidence/support/subject schemas.** Fill A1 fields
   1--17, the projection interfaces above, and root `0xfb`/`0xfc` layouts.
4. **A4-R2d -- quotient, outcome, coverage, disposition, action, and joined
   Level-II schemas.** Fill roots `0xfd`, `0xe7`--`0xe9`, `0xfe`, and `0xff`
   without a back edge, then materialize root `0xe3` after its candidate and
   action-image payload types are topologically earlier.
5. **A4-R3 -- failure/resource parameter closure after R2d.** Fill the exact
   typed slots whose outer positions and scalar widths R2a--R2d freeze.
6. **A4-R2e -- complete registry/source integration join after R3.** Assemble
   root `0xe0` with the R3 types, create the non-authoritative reference
   interpreter slice, measure its protocol identity, and independently audit
   every type-directed round trip and ownership edge.
7. **A4-R4 -- regenerated fixtures and mutations after R2e.** Regenerate the
   definition and manifest only from the complete joined registry.

Completion of R2d, not an impossible opaque R3 placeholder, unblocks A4-R3.
Only the post-R3 R2e integration audit freezes the complete A4-R2 registry.
The labels record which defect owns each artifact; they do not impose the
incorrect linear order `R2e before R3`. A4-R4 remains the owner of regenerated
definition/manifest fixtures and mutation rejection.

Post-freeze scheduling audit found one analogous forward-reference hazard in
the original subdivision wording: R1's five-way `CheckedLevelTwo` sum contains
`CheckedCandidateEnvelope` and `CheckedActionImage`, whose concrete payload
types close only in R2c/R2d. Under section 5, neither a sum nor an `OwnedRef`
may point to a later `TypeIdV1`; an opaque identity or byte placeholder is also
forbidden. Therefore R2b freezes roots `0xe2`, `0xe4`, `0xe5`, `0xe6`, the
three early checked payloads, and the five exact `0xe3` variant tags. R2d
materializes the root-`0xe3` sum only after the two late payload types exist.
This scheduling corrigendum changes no R2a wire rule, root assignment, or
ownership edge; it prevents a downstream gate label from requiring a forbidden
forward reference. An independent reread returned **PASS** on the corrected
allocation: root object ordinal 18 remains metadata, not a `TypeIdV1`, and the
root-`0xe3` definition is now materialized only after all five payload types are
topologically earlier.

## 3. Representation strata and well-foundedness

Every ordinary representation type belongs to exactly one stratum:

| Stratum | Contents |
| --- | --- |
| `S0` | exact inherited kernel/JG codecs and frozen A4-R1 values |
| `S1` | paths, raw proposals, formal action syntax, and subject syntax |
| `S2` | checked carrier/decode/build intermediates |
| `S3` | pre-evidence envelope semantic payload and pure projections |
| `S4` | normalization, support, primitive, obligation, and subject evidence |
| `S5` | envelope core/evidence, quotient key, outcomes, coverage, disposition |
| `S6` | checked action trace and action image |
| `S7` | private authority-bearing owners and deterministic arenas |

A type in `Si` may contain by value only a type in `Sj` with `j<i` or a type
with a strictly earlier local `TypeIdV1` in the same stratum, except for exact
inherited recursive types in `S0`. A value may refer to another value in the
same owning arena only through a checked owner-local reference whose target was
constructed earlier in the arena's frozen total order. No ordinary V1 schema
definition refers to itself, directly or transitively.

The only admitted recursive wire types are inherited kernel/JG types such as
`Term`; their exact codec identities and recursion limits are parent bindings,
not redefined A4 types. A future implementation must reject a schema DAG with
a local back edge before allocating or decoding any instance.

## 4. Universal canonical wire

The following rules apply to every new ordinary V1 type unless an inherited
parent codec is named explicitly:

| Value | Canonical bytes |
| --- | --- |
| sum or enum tag | one `u8`, zero-based displayed order |
| Boolean | `0x00=false`, `0x01=true`; every other byte rejected |
| schema/component version | little-endian `u16` |
| binder depth | little-endian `u16` |
| type or record-field ordinal | little-endian `u16` |
| source/raw/path/class/member/arena ordinal | little-endian `u32` |
| history/event/declaration ordinal | little-endian `u64` |
| count, byte offset, or length | little-endian `u64` |
| byte vector | `u64` byte length, then those bytes |
| canonical text | `u64` byte length, then valid UTF-8 bytes |
| vector | `u64` element count, then elements in order |
| option | `0x00` or `0x01` followed by the `Some` payload |
| record or tuple | fields in declared order, without padding |
| rooted object | root `u8`, schema version `u16=1`, then fields |

Schema names, domain strings, fixed source paths, and protocol symbols are
restricted to nonempty ASCII strings over `[A-Za-z0-9_./-]`. They receive no
Unicode, locale, path-case, or separator normalization. Ordinary mathematical
text values, where a parent type admits them, retain the exact parent codec.

A bit vector encodes `(bit_count:u64,byte_count:u64,bytes)`, requires
`byte_count=ceil(bit_count/8)`, uses least-significant-bit-first order within a
byte, and requires every unused high bit of the last byte to be zero.

Set-like vectors are sorted lexicographically by complete canonical element
bytes and reject adjacent duplicates. Digests may index a comparison but never
decide equality, inequality, order, or ownership. Decoding consumes the whole
input; trailing bytes, nonminimal variants, invalid tags, overflow, and a
failed decode/re-encode byte-equality check are errors.

`Digest`, `GlobalId`, `GenerativeHistoryEventIdV1`, kernel terms, declarations,
contexts, signatures, and judgments retain their exact parent codecs. In
particular, `Digest` remains the 79-byte canonical value frozen by A4-R1.

## 5. The closed typed-schema DAG

The future root-`0xe0` registry contains a by-value
`OrdinarySchemaDagV1`. Its metalanguage is fixed here.

```text
TypeIdV1 = u16

ScalarKindV1 ::=
  Unit=0 | Bool=1 | U8=2 | U16=3 | U32=4 | U64=5
| Bytes=6 | Text=7 | Digest=8

SchemaTypeNodeV1 ::=
  Scalar=0(ScalarKindV1)
| Inherited=1(inherited_binding_ordinal:u16)
| Newtype=2(prior_type:TypeIdV1)
| Option=3(element_type:TypeIdV1)
| Vector=4(element_type:TypeIdV1)
| Record=5(fields:vector<FieldDefV1>)
| Sum=6(variants:vector<VariantDefV1>)
| OwnedRef=7(owner_kind:OwnerKindV1,
             object_type:TypeIdV1,
             identity_type:TypeIdV1)
| MetaCodecRef=8(meta_codec:MetaCodecKindV1)

FieldDefV1 =
  (field_ordinal:u16, field_type:TypeIdV1)

VariantDefV1 =
  (variant_tag:u8, payload_type:Option<TypeIdV1>)

SchemaTypeDefV1 =
  (type_id:TypeIdV1, node:SchemaTypeNodeV1)

OrdinarySchemaDagV1 =
  (schema_dag_version:u16=1,
   inherited_bindings:vector<InheritedCodecBindingV1>,
   type_definitions:vector<SchemaTypeDefV1>)
```

```text
OwnerKindV1 ::=
  VerifiedProfile=0
| EnvelopeCensus=1
| SourceCensus=2
| PairCandidateArena=3
| PairClassArena=4
| CheckedActionImage=5

MetaCodecKindV1 ::=
  RegistryMetaCodecV1=0
```

`type_definitions.len <= 65,536`, `type_definitions[i].type_id=i`, and every
referenced local `TypeIdV1` must be strictly less than the containing
definition's ID. A record has at most 65,536 fields and a sum at most 256
variants. Field ordinals and variant tags must equal their vector positions.
Empty records and empty sums are forbidden; `Unit` supplies a payload-free
value. A `Newtype` has the identical inner codec but remains a distinct typed
schema node.

```text
InheritedCodecBindingV1 =
  (binding_ordinal:u16,
   parent_slot:PredecessorCodecSlotV1,
   parent_type_tag:u16,
   parent_codec_identity:Digest)

PredecessorCodecSlotV1 ::=
  JG1=0 | JG2a=1 | JG2b2a=2 | JG2b2b0=3 | JG2b2b1=4
```

Binding ordinals are contiguous. Each binding is checked against the full
reminted parent owned by `K`; a caller digest cannot introduce an inherited
type. These tags are exactly the first five frozen `ParentSlotV1` tags.
`ParentSlotV1::Profile=5` is deliberately unrepresentable here: importing a
codec from the profile that owns this registry would be a back edge. An
inherited recursive codec is allowed only at this predecessor boundary.

`OwnedRef` has the canonical value

```text
(owner_local_index:u32, full_typed_identity:identity_type).
```

The index is range checked against the enclosing deterministic owner and the
indexed object's freshly encoded identity must equal the following full value.
An index never substitutes for that value. An `OwnedRef` is invalid outside
the root object that owns its arena.

The schema DAG binds layouts. The completed registry must additionally bind a
typed, topologically ordered operation transcript for source resolution, raw
generation, decoding, building, lineage, normalization, support, field
dispositions, subjects, quotient projection, action rebuild, and coverage.
Rule labels or source-code names alone do not bind those semantics. R2b--R2d
must instantiate those operation records; R2a does not claim they are filled.

The root-`0xe0` registry's own outer codec is this R2a metalanguage. Exactly
`type_definitions[0]` is the payload-free
`MetaCodecRef(RegistryMetaCodecV1)` node, and root `0xe0` maps to exactly
`TypeIdV1=0`. No other `MetaCodecRef` node is valid. Within the entire local
DAG, `TypeIdV1=0` may occur as a field type exactly once: as zero-based field
ordinal 17, `dynamic_schema_registry`, of the root-`0xf8` profile-definition
record. It is forbidden as a newtype target, option/vector element, sum
payload, owned-reference component, operation input/output, or any other
record field. Thus the single hard-coded meta-codec closes the designated
profile-to-registry containment edge and cannot be used as a general escape
hatch or placed inside a registry operation value. Every other dynamic root
maps to its ordinary local type ID. Describing the root-`0xf8` record in the
registry is metadata; its runtime registry field is encoded through this one
external meta-codec and is not recursively embedded in the registry value.

## 6. Typed path language

R1's `PublicFieldPathV1::DeclarationField` is the singleton path of an older
history export. It is not reused for an envelope field. Ordinary V1 uses the
following distinct path types:

```text
TypedPathStepV1 ::=
  RecordField=0(field_ordinal:u16)
| VectorElement=1(element_ordinal:u32)
| SumPayload=2(variant_tag:u8)
| OptionPayload=3
| TermChild=4(inherited JG2b2b0 child-edge tag:u8)

RawFieldPathV1 =
  (constructor:ConstructorTagV1,
   steps:vector<TypedPathStepV1>)

EnvelopeFieldRootV1 ::=
  Indices=0 | Inputs=1 | Output=2 | RawRealization=3

EnvelopeFieldPathV1 =
  (constructor:ConstructorTagV1,
   root:EnvelopeFieldRootV1,
   steps:vector<TypedPathStepV1>)

SupportRootTagV1 ::=
  Indices=0 | Inputs=1 | Output=2 | RawRealization=3
| RawCode=4 | Obligations=5 | SubjectProjection=6 | ActionArgument=7

FullLocalPathV1 =
  (root:SupportRootTagV1,
   steps:vector<TypedPathStepV1>)

QuotientFieldPathV1 =
  (quotient_field_ordinal:u16,
   steps:vector<TypedPathStepV1>)

DefinitionRulePathV1 =
  (protocol_slot:ProtocolSlotV1,
   steps:vector<TypedPathStepV1>)

SubjectPathV1 =
  (subject_slot:u16, node_ordinal:u32)
```

`ConstructorTagV1` remains the A3-O order `Formation=0` through
`DischargeTransformer=6`. The operation-transcript slot order is already
closed:

```text
ProtocolSlotV1 ::=
  SourceResolution=0
| RawGeneration=1
| Decode=2
| Build=3
| PublicLeavesAndLineage=4
| Normalization=5
| SupportAndOwnerResolution=6
| FieldDisposition=7
| SubjectConstruction=8
| QuotientProjectionAndComparison=9
| ActionNormalizationAndRebuild=10
| OutcomePartitionAndCoverage=11

TypedProjectionPathV1 ::=
  RawField=0(RawFieldPathV1)
| EnvelopeField=1(EnvelopeFieldPathV1)
| SupportLocal=2(FullLocalPathV1)
| QuotientField=3(QuotientFieldPathV1)

OriginPathV1 ::=
  FormationOldestOutputType=0
| AbstractionOldestFamilyType=1
| AggregationOldestRawOutputType=2
| TransportOldestRawOutputType=3
| ComparisonOldestLeftType=4
| DemandCompilerOldestActivationImage=5
| DischargeTransformerOldestRawOutputImage=6
```

R2b must bind each `OriginPathV1` tag to one exact `RawFieldPathV1`; the enum
itself and its wire are fixed here.

Every path is decoded relative to an exact root type in the schema DAG and
traversed against the corresponding runtime value. `SumPayload(tag)` is valid
only when the current value has that exact tag and the bound variant has a
payload; `OptionPayload` is valid only for `Some`; and a vector ordinal must be
in bounds. A step whose kind, tag, or ordinal is invalid for the current schema
node/value is rejected. A path must end at the declared expected type and must
re-encode byte-identically. There is no free `GrammarPath`, `MetaSyntaxPath`,
string path, or untyped local path in ordinary V1; those earlier names are
aliases for one of the typed forms above only after its exact root type is
known.

The binder identity sum is:

```text
HistoricalBodyEdgeV1 ::= PiBody=0 | SigmaBody=1 | LambdaBody=2

BinderKeyV1 ::=
  HistoricalBinder=0(parent:OccurrenceId14V1,
                       body_edge:HistoricalBodyEdgeV1)
| SyntheticBinder=1(profile_version:u16,
                      constructor:ConstructorTagV1,
                      raw_field_path:RawFieldPathV1)
| FormalBinder=2(subject_path:SubjectPathV1,
                  binder_ordinal:u32)
```

The formal variant is forbidden in raw-code candidate decoding and exists only
in checked formal-action contexts. Historical and synthetic keys retain the
A3-O construction rules; binder-type equality alone never identifies them.

R1 also requires every emitted raw source leaf to retain its checked ordinal:

```text
SourceLeafV1 =
  (source_ordinal:u32, source:SourceRefV1).
```

The ordinal must select a byte-identical full `SourceRefV1` in the enclosing
`BirthSourceCensusV1`. It is not a replacement identity and cannot be supplied
without the census owner.

## 7. Nonrecursive theorem-subject boundary

The canonical theorem language is a topologically ordered DAG, never a
recursive expression tree:

```text
SubjectSyntaxV1 =
  (binders:vector<SubjectBinderV1>,
   nodes:vector<SubjectNodeV1>,
   root_node:u32)

SubjectNodeV1 ::=
  Binder=0(binder_ordinal:u32)
| Literal=1(CoreProjectionLiteralV1)
| Project=2(source_node:u32,path:TypedProjectionPathV1)
| Normalize=3(source_node:u32)
| Reindex=4(value_node:u32,action_node:u32)
| Compose=5(left_node:u32,right_node:u32)
| RenameHistory=6(value_node:u32,renaming_node:u32)
| Selector=7(value_node:u32)
| Origin=8(value_node:u32,origin_path:OriginPathV1)
| ApproxOrd=9(left_node:u32,right_node:u32)
| ApproxEnvelope=10(left_node:u32,right_node:u32)
| Equality=11(left_node:u32,right_node:u32)
| Membership=12(element_node:u32,set_node:u32)
| Conjunction=13(conjunct_nodes:vector<u32>)
| Implication=14(premise_node:u32,conclusion_node:u32)
| ExistsUnique=15(binder_ordinal:u32,
                   domain_node:u32,predicate_node:u32)
| DependentDischargeEquality=16(left_node:u32,right_node:u32)
```

R2a-C2 appends only tag 16; tags `0..15` and all their payloads remain byte-for-
byte unchanged. The new tag is the structural two-operand opcode needed when a
dependent discharge equality must retain potentially distinct carrier indices.
It stores only two backward node ordinals. R2c+C2 owns its exact normalized
typing rule and heterogeneous subject leaf; the R2a syntax node itself contains
no index, body, proof, receipt, path, or later schema reference. This division
preserves the upstream type DAG while preventing a downstream record from
silently extending an R2a-owned sum.

Every node operand must be strictly less than that node's ordinal; binders are
contiguous and the root must exist. R2b--R2d must close
`SubjectBinderV1`, `CoreProjectionLiteralV1`, `TypedProjectionPathV1`, and the
seven `OriginPathV1` instantiations as finite schema-DAG types before R2
freezes. This gate fixes their admissible boundary:

```text
admitted literals:
  core payload projection, carrier payload projection,
  exact/normalized occurrence identity, checked formal-action argument,
  or pure support-function projection

forbidden literals:
  RawCapabilityEnvelopeV1, CandidateEnvelopeV1, OrdinarySupportV1,
  QuotientKeyV1, FinalActionKeyV1, ActionTraceV1, ActionImageV1,
  PairTranscriptV1, or PairDispositionV1.
```

Before support traversal, the verifier deterministically derives
`SubjectRootProjectionV1`: the ordered typed endpoint literals instantiated by
the constructor schema, without the containing subject wrapper, quotient key,
or action image. Support traverses this projection. A1 field 16 later retains
the full subject DAG plus the byte-identical projection. Consequently support
can cover every subject endpoint without referring to a record that contains
support.

Normalization records a `SupportProjectionSubjectV1`, a proposition over the
pure support-projection function. It never embeds `OrdinarySupportV1`. Full
support may later retain that same subject by value without a reverse edge.

## 8. Bounded action correction

Canonical raw action syntax is a nonrecursive postfix program:

```text
FormalActionTokenV1 ::=
  Identity=0
| PrimitiveSubstitution=1(SubstitutionProposalV1)
| PrimitiveContextSquare=2(ContextSquareProposalV1)
| Compose=3

ActionKindV1 ::=
  SubstitutionAction=0 | ContextSquareAction=1

FormalActionSyntaxV1 =
  (action_kind:ActionKindV1,
   postfix_tokens:vector<FormalActionTokenV1>)
```

Validation scans left to right with a checked `u32` stack of typed actions.
`Identity` pushes an identity of the header `action_kind`.
`PrimitiveSubstitution` is admissible only under `SubstitutionAction`, and
`PrimitiveContextSquare` only under `ContextSquareAction`; each pushes that
kind. `Compose` consumes two actions, requires both to have the header kind,
and pushes their ordered composite of the same kind. A mixed-kind program is
rejected even when its untyped stack shape would be valid. Exactly one final
action of the header kind is required. The original postfix vector preserves
association. Normalization removes identities and emits the exact
left-to-right vector of checked primitive actions; an empty vector is identity
of the header kind.

A context-square proposal supplies `rho`, `sigma`, and `theta_prime`; its
pointwise `epsilon` equality is internally derived and checked. No caller proof
payload is a raw field.

Every term/global in a bounded action must check under A4-R1's exact
`Sigma(H,o)`. A global then resolves to its exact current or older owner.
Ordinary V1 removes `FormalActionIdV1::ExternalGlobal`; an unknown global is a
checked action false at a caller proposal locus and an invariant abort if it
appears in an internally rebuilt trace. Introducing an action-owned signature
extension would require a new profile version.

Action sequential and direct endpoints have the nonrecursive shape

```text
ActionEndpointV1 =
  (canonical_schema_payload:CanonicalSchemaPayloadV1,
   semantic_key_projection:QuotientSemanticProjectionV1).
```

They never contain an action image. Root `0xff` may contain the final full
quotient key because the quotient key contains no action image or final-key
literal; root `0xfe` owns the representative route and endpoints.

## 9. Root and domain namespace

Root `0xe1` is already the frozen complete-through-head commitment. Repository
inventory leaves `0xe0` and `0xe2`--`0xe7` for the missing R2 objects. The final
`CodecDomainBindingV1` preserves RC1 object ordinals 0--9, appends frozen R1
objects as 10--15, and appends these R2 objects as 16--22:

| Object ordinal | Object | Root | Domain |
| ---: | --- | ---: | --- |
| 0 | `ProfileDefinition` | `0xf8` | `law-v2/jg2b2b2a/ordinary-profile-definition/v1` |
| 1 | `ProfileManifest` | `0xf9` | `law-v2/jg2b2b2a/ordinary-profile-manifest/v1` |
| 2 | `ResourcePolicy` | `0xfa` | `law-v2/jg2b2b2a/ordinary-resource-policy/v1` |
| 3 | `EnvelopeCore` | `0xfb` | `law-v2/jg2b2b2a/ordinary-envelope-core/v1` |
| 4 | `ExactEvidence` | `0xfc` | `law-v2/jg2b2b2a/ordinary-envelope-exact-evidence/v1` |
| 5 | `QuotientKey` | `0xfd` | `law-v2/jg2b2b2a/ordinary-envelope-quotient-key/v1` |
| 6 | `ActionTrace` | `0xfe` | `law-v2/jg2b2b2a/ordinary-action-trace/v1` |
| 7 | `ActionImage` | `0xff` | `law-v2/jg2b2b2a/ordinary-action-image/v1` |
| 8 | `PairCoverage` | `0xe8` | `law-v2/jg2b2b2a/ordinary-pair-coverage/v1` |
| 9 | `PairDisposition` | `0xe9` | `law-v2/jg2b2b2a/ordinary-pair-disposition/v1` |
| 10 | `ExactOccurrenceId14` | `0xea` | `law-v2/jg2b2b2a/ordinary-exact-occurrence-id14/v1` |
| 11 | `NormalizedOccurrenceKey` | `0xeb` | `law-v2/jg2b2b2a/ordinary-normalized-occurrence-key/v1` |
| 12 | `OlderPublicExportId` | `0xec` | `law-v2/jg2b2b2a/ordinary-older-public-export-id/v1` |
| 13 | `OlderPublicExport` | `0xed` | `law-v2/jg2b2b2a/ordinary-older-public-export/v1` |
| 14 | `BirthOpaqueSignature` | `0xee` | `law-v2/jg2b2b2a/ordinary-opaque-birth-signature/v1` |
| 15 | `BirthSourceCensus` | `0xef` | `law-v2/jg2b2b2a/ordinary-birth-source-census/v1` |
| 16 | `DynamicSchemaRegistry` | `0xe0` | `law-v2/jg2b2b2a/ordinary-dynamic-schema-registry/v1` |
| 17 | `LevelOneProposal` | `0xe2` | `law-v2/jg2b2b2a/ordinary-level-one-proposal/v1` |
| 18 | `CheckedLevelTwo` | `0xe3` | `law-v2/jg2b2b2a/ordinary-checked-level-two/v1` |
| 19 | `RawConstructorCode` | `0xe4` | `law-v2/jg2b2b2a/ordinary-raw-constructor-code/v1` |
| 20 | `CheckedDecoded` | `0xe5` | `law-v2/jg2b2b2a/ordinary-checked-decoded/v1` |
| 21 | `CheckedBuilt` | `0xe6` | `law-v2/jg2b2b2a/ordinary-checked-built/v1` |
| 22 | `CheckedLeafOutcome` | `0xe7` | `law-v2/jg2b2b2a/ordinary-checked-leaf-outcome/v1` |

R3 introduces no new root. Its values or by-value static definitions are
definitely nested at the exact R2 field slots under `0xe0`, `0xe3`, `0xe5`, `0xe6`,
`0xe7`, `0xe8`, `0xf8`, `0xf9`, `0xfa`, `0xfb`, `0xfc`, and `0xfe`. This is
the 2026-08-02 root-containment corrigendum: the earlier five-root inventory
omitted the frozen R2b success receipts under `0xe5`/`0xe6` (and therefore
future `0xe3`), the A1 replay/normalization/resource evidence under `0xfb`, and
the static failure/resource definitions nested by value under `0xe0`, `0xf8`,
and `0xf9`.
The corrigendum changes no root, outer field, type, tag, or semantic rule. R3
may fill those qualified types but may not change an R2 outer tag, field
position, ownership edge, or scalar width.

Root `0xfd` remains receipt-free by the frozen quotient stripping rule. R2d
must make the still-open `0xe9` and `0xff` containment choice explicit: a
checked owner-local reference with a complete typed receipt-free identity may
keep R3 evidence only in its owner, whereas embedding a full receipt-bearing
coverage/candidate value places R3 transitively under that root. A bare digest,
opaque bytes, or an unstated choice is forbidden.

## 10. Private ownership graph

The only authority-bearing ownership edges are:

```text
VerifiedOrdinaryProfile K
  owns full definition, manifest, dynamic registry,
       implementation-source input, and every reminted parent

VerifiedEnvelopeCensus
  owns K, full H, matching kernel, opaque Sigma bindings,
       complete BirthSourceCensus values,
       checked carrier/decoded/built arenas,
       candidate core/evidence/key arena,
       complete pair transcripts and dispositions

CheckedActionImage
  owns exactly one original checked candidate,
       one full ActionTraceV1, and one ActionImageV1
```

Candidate arena order is ascending raw ordinal. Quotient-class order is full
quotient-key byte order. Members are ascending raw ordinal. Source order is the
frozen R1 order. These orders make every owner-local index deterministic.

No candidate owns a pair transcript; no quotient key owns evidence or an
action trace; no action trace owns an action image; no action image owns another
action image. Acting on an action image first projects its original candidate,
concatenates the normalized action, and rebuilds once from that original base.

Canonical references always retain their full typed identity after the checked
owner-local index. Rust arena layout, allocation addresses, map order, and
shared-pointer identity never enter canonical bytes.

## 11. Reproducible Rust implementation-protocol input

R2 freezes the source-identity algorithm now, but its final digest cannot be
pinned before the narrow non-authoritative reference source slice exists.
Deferring the input to the blocked JG2b2b2b evaluator would be circular.

```text
CanonicalSourceChunkV1 =
  (chunk_tag:u8,
   fixed_path:text,
   canonical_utf8_lf_bytes:bytes)

ImplementationProtocolIdentityInputV1 =
  (source_input_schema_version:u16=1,
   chunks:vector<CanonicalSourceChunkV1>)
```

For every chunk, bytes must be valid UTF-8; each CRLF pair becomes LF; a bare
carriage return is rejected. No other byte transformation occurs. The path is
encoded with the universal canonical-text codec and is part of the identity.
The exact tags, paths, and order are:

```text
 0 crates/pen-ordinary-profile-fixture/src/lib.rs
 1 crates/pen-ordinary-profile-fixture/src/schema.rs
 2 crates/pen-ordinary-profile-fixture/src/canonical.rs
 3 crates/pen-ordinary-profile-fixture/src/protocol.rs
 4 crates/pen-ordinary-profile-fixture/src/source_identity.rs
 5 crates/pen-ordinary-profile-fixture/src/registry.rs
 6 crates/pen-ordinary-profile-fixture/src/remint.rs
 7 crates/pen-ordinary-profile-fixture/src/conformance.rs
 8 crates/pen-ordinary-profile-fixture/Cargo.toml
 9 crates/pen-ordinary-profile-fixture/production-dependency-graph.lock
10 rust-toolchain.toml
11 .cargo/config.toml
```

Chunk 9 is the reviewed canonical resolved **production** dependency graph,
not an unchecked copy of the workspace lockfile. Its generator must parse the
active lock and manifest, derive the reachable production graph, require exact
equality with the reviewed file, and then bind the reviewed bytes, following
the existing kernel protocol-identity pattern. The crate root may declare only
the seven sibling modules listed above. Source measurement parses every
production `mod` declaration reachable from `src/lib.rs`, requires the
reachable relative-path set to equal chunks 0--7 exactly, and rejects an
unlisted module, build script, generated include, macro-expanded source file,
or path override. Local dependency implementations are not silently folded
into this set: their exact reminted parent protocol identities are separate
profile inputs and the reviewed production graph binds the dependency edges.

The identity is

```text
Digest::of_domain_bytes(
  "law-v2/jg2b2b2a/ordinary-rust-implementation-protocol/v1",
  encode(ImplementationProtocolIdentityInputV1)).
```

The input contains no digest of itself. Included source files may not embed the
expected implementation digest or regenerated A4 fixture. Build output, Git
metadata, timestamps, absolute paths, environment variables, target triples,
and dev-only dependency resolution are excluded.

The completed profile definition at `0xf8` must own the source-set contract and
the complete `0xe0` registry by value. The `0xf9` manifest adds one
`implementation_protocol_digest` field. `K` privately owns the complete source
input and recomputes the digest; a caller-supplied digest is never evidence.

These files implement the generic schema/protocol validator, canonical codec,
registry reconstruction, source measurement, remint comparison, and a
reference interpreter for every closed operation-DAG node, including raw
generation, decode/build, quotient/coverage, and action-rebuild nodes on
unprivileged fixture values. They contain no constructor for `K`, checked
history, candidate, coverage, disposition, or law tokens; cannot inspect live
history; and cannot publish a semantic result. Creating this isolated slice
during R2e is A4 fixture infrastructure, not the JG2b2b2b evaluator. A later
production evaluator requires its own source identity and Rust/Agda
correspondence; changing it cannot redefine the by-value profile semantics.

## 12. R3-owned typed slots

R2b--R2d must place, but must not fake with arbitrary bytes, the following
R3-owned values:

- `FalseReasonV1`, `AbortReasonV1`, their loci and payloads;
- `KernelCallRoleV1` and exact success/error receipts;
- `CheckedFalseReceiptV1` and unprivileged `NoDisposition` diagnostics;
- pair/envelope/action resource accounts and aggregate meter receipts;
- structural cardinality and count-only traversal evidence; and
- material/depth recurrence and allocation/budget receipts.

Until R3 supplies their complete schema-DAG definitions, no final root
containing one has complete canonical bytes. This is the explicit
`R2d -> R3 -> R2e` integration dependency, not an opaque extension point: R3
types must use the R2 scalar codec and the already frozen outer field
positions.

## 13. R2a acceptance conditions

R2a froze only after an independent audit confirmed all of the following:

1. every local schema reference is topologically earlier;
2. the path types decode only against the bound schema DAG;
3. support consumes `SubjectRootProjectionV1`, not field 16;
4. normalization names only the pure support projection subject;
5. no subject literal contains a quotient/final key or enclosing object;
6. no action endpoint or trace contains an action image;
7. `ExternalGlobal` and untyped carrier-index bytes are absent from ordinary
   V1;
8. the exact A4 set `0xe0`, `0xe2`--`0xef`, and `0xf8`--`0xff` is
   collision-free, with `0xe1` and unrelated `0xf0`--`0xf7` roots preserved;
9. every checked reference contains both deterministic index and full identity;
10. the implementation-source input has exact chunks, canonicalization,
    domain, and anti-self-reference rules; and
11. no profile, candidate, disposition, or theorem authority is minted; and
12. C2 appends exactly payload `(left_node:u32,right_node:u32)` at tag 16,
    preserves tags `0..15`, and introduces no downstream schema reference.

The independent base audit completed on 2026-08-02. Its first pass rejected a
profile-owned inherited-codec back edge, a general-purpose meta-codec escape,
an untagged sum-payload path, and an action program checked only by stack shape.
The repaired record admits only predecessor codec slots, reserves exactly
`TypeIdV1=0` for the one root-`0xe0` meta-codec use, carries the runtime variant
tag in every sum path, and type-checks every action token and composition against
the exact header kind. The second pass returned **PASS** on all eleven
base conditions. An independent 2026-08-03 C2 audit confirmed contiguous tags
`0..16`, unchanged encodings for tags `0..15`, backward-only operands, and no
schema back-edge. This freezes R2a+C2 only; it does not pre-approve an R2b--R2e
schema or fixture.

## 14. Continuation and non-authority

R2b has now independently frozen the nine carrier proposals, seven raw
products, all checked decoder compounds, seven concrete built records, three
early Level-II payload records, and the exact tag/field contract for the later
five-way Level-II sum in
`docs/260802_jg2b2b2a_a4_r2b_level_relation_schemas.md`. It closes roots
`0xe2`, `0xe4`, `0xe5`, and `0xe6`; R2d closes root `0xe3` after the
candidate-envelope and action-image types exist. Frozen R2c+C2 uses
`SourceLeafV1`, `EnvelopeFieldPathV1`, the exact R1 `Sigma(H,o)`, and the full
typed carrier-index sum fixed by A2-O.

This record selects no history, occurrence, constructor, raw code, carrier,
action, or desired outcome. It mints no definition token, envelope, action
image, coverage certificate, disposition, occurrence classification, generic
law, cubical bridge, `GCap`, `gamma`, or selective authority.
