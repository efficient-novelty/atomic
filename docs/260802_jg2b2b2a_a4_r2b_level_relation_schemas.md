# JG2b2b2a-A4-R2b Level-Relation and Constructor-Intermediate Schemas

Date: 2026-08-02

Status: **A4-R2b FROZEN 2026-08-02; A4-R2c ACTIVE.** This independently
audited record instantiates
the Level-I proposal, raw-constructor, checked-decoder, checked-built, and early
Level-II payload schemas under the frozen A4-R2a metalanguage. It closes the
outer layouts assigned to roots `0xe2`, `0xe4`, `0xe5`, and `0xe6`, and freezes
the five variant tags of root `0xe3`. It does not materialize root `0xe3`, mint
a profile or envelope, inspect a history, or unblock JG2b2b2b.

Normative inputs are:

- `docs/260802_jg2b2b2a_a4_r2a_acyclic_schema_foundation.md`;
- `docs/260802_jg2b2b2a_a4_r1_signature_source_census.md`;
- `docs/260802_jg2b2b2a_indexed_ontology_profiles.md` (A2-O);
- `docs/260802_jg2b2b2a_a3_ordinary_constructor_schemas.md` (A3-O); and
- the exact inherited `pen-kernel`, JG2b2b0, and JG2b2b1 codecs reminted by the
  profile.

Where a name below is an inherited type, its parent codec is used unchanged.
Where a new type is displayed, its sum tags and record fields are zero-based in
the displayed order and use A4-R2a's canonical scalar wire.

## 1. Gate allocation and the root-`0xe3` correction

R1 fixes a three-way Level-I sum and a five-way Level-II sum. The last two
Level-II variants contain a checked candidate envelope and a checked action
image. Their payload types do not close until R2c and R2d. A4-R2a forbids a
forward `TypeIdV1`, a digest-only substitute, and an opaque byte placeholder.
Consequently the exact allocation is:

```text
R2b: roots 0xe2, 0xe4, 0xe5, 0xe6;
     CheckedCarrier / CheckedDecoded / CheckedBuilt payloads;
     root-0xe3 tags 0..4 and their semantic field order.

R2c: candidate-envelope payload types.
R2d: action-image payload types, then the topological root-0xe3 TypeDef.
R2e: final TypeId assignment, operation join, codecs, and source identity.
```

Root object ordinals and root bytes are namespace metadata; they do not impose
`TypeIdV1` order. The final DAG must place every payload type strictly before
the `CheckedOrdinaryLevelTwoV1` sum node.

## 2. Inherited values and semantic newtypes

The following predecessor types use `InheritedCodecBindingV1` without another
wrapper codec:

```text
Term
DependentContext
UncheckedSignature
OpenJudgment
Digest
GlobalId
VerifiedParticularOpenTypedSubstitutionV1
```

The following frozen A4-R1 types are **earlier local schema definitions**, not
`Inherited` nodes:

```text
OccurrenceId14V1
NormalizedOccurrenceKeyV1
ExportIdV1
OlderPublicExportV1
BirthOpaqueSignatureBindingV1
BirthSourceCensusV1
```

Their roots `0xea`--`0xef` and codecs remain exactly R1. R2e must assign them
local TypeIds before every R2b type that refers to them. Classifying one under
a predecessor slot or importing `ParentSlotV1::Profile` is invalid.

All unrooted support nodes of those R1 roots, including
`PublicFieldPathV1`, `ProfileManifestIdV1`, and `SourceRefV1`, and every R2a
path, binder, source-leaf, constructor-tag, and formal-action node used below
are likewise earlier local schema definitions. They are not inherited-codec
bindings merely because R2b consumes rather than redefines them.

The following R2b types are semantic newtypes. A newtype has the identical wire
as its inner type but remains a distinct schema node:

```text
RawContextV1            = newtype vector<Term>
RawTelescopeV1          = newtype vector<Term>
RawSectionV1            = newtype vector<Term>
RawSubstitutionValueV1  = newtype vector<Term>
NormalizedContextV1     = newtype DependentContext
NormalizedTelescopeV1   = newtype vector<Term>
NormalizedSectionV1     = newtype vector<Term>
```

Their validation is type directed. `RawContextV1` is an oldest-first dependent
context proposal. A telescope is oldest-first over its separately retained
base context. A section/substitution vector is oldest-codomain-first. No one of
these types is interchangeable merely because its inner wire is a term vector.
R2a `Newtype` traversal is transparent: typed paths emit no artificial unwrap
step, but the path interpreter changes its current schema node to the newtype's
inner prior type before accepting the next explicit step.

All term/context sizes are bounded by the reminted parent resource policy.
Failure to prove a bound is an R3 abort result, not a different R2b wire.

## 3. Exact raw carrier proposals

### 3.1 Standalone raw compounds

```text
SubstitutionProposalV1 =
  (delta:RawContextV1,
   gamma:RawContextV1,
   images:RawSubstitutionValueV1)

ComparisonRouteV1 ::=
  KernelNormalizedTypeEquality=0

RawGrammarOperationV1 =
  (parameters:RawTelescopeV1,
   results:RawTelescopeV1)

RawGrammarValueV1 =
  (exports:RawTelescopeV1,
   operations:vector<RawGrammarOperationV1>)

RawPortValueV1 =
  (operation_ordinal:u32,
   arguments:RawSectionV1)

RawContractValueV1 =
  (left:Term,
   right:Term,
   ty:Term)

RawSchemeValueV1 =
  (trigger:RawTelescopeV1,
   ports:vector<RawPortValueV1>,
   contracts:vector<RawContractValueV1>)

RawLiveDemandValueV1 =
  (demanded_telescope:RawTelescopeV1,
   inherited_operation_ordinals:vector<u32>,
   specialized_contracts:vector<RawContractValueV1>)

RawDischargeValueV1 =
  (section:RawSectionV1)
```

The raw grammar has no bodies or names. Operation and port ordinals are
contiguous/in-bounds when checked. A port output is derived from its selected
operation and arguments; it is never a raw field. Contract order here is the
A2-O carrier order `(left,right,type)`. Raw A3 source code deliberately uses a
different parse order, fixed in section 5.

`RawLiveDemandValueV1` is the proposed `L=Spec(S,a)` body and is checked against
the unique specialization rebuilt from the separately retained activation
index; it is not accepted as the specialization oracle. Its inherited-operation
vector has exactly one entry per scheme port, in port order, and entry `j`
equals that port's checked `operation_ordinal`. A discharge contains no
derivability or nonexistence bit.

### 3.2 Full carrier-index sum

R2a's `CarrierSlot` expected judgment carries this value, never arbitrary
bytes:

```text
CarrierIndexValueV1 ::=
  PublicContextIndex=0(gamma:NormalizedContextV1)
| PublicInterfaceIndex=1(gamma:NormalizedContextV1)
| InterfaceFamilyIndex=2(gamma:NormalizedContextV1,
                           a:NormalizedTelescopeV1)
| SubstitutionIndex=3(delta:NormalizedContextV1,
                       gamma:NormalizedContextV1)
| ComparisonWitnessIndex=4(gamma:NormalizedContextV1,
                            a:NormalizedTelescopeV1,
                            b:NormalizedTelescopeV1)
| SealedPublicGrammarIndex=5(gamma:NormalizedContextV1)
| DemandSchemeIndex=6(gamma:NormalizedContextV1,
                       grammar:NormalizedGrammarValueV1)
| LiveDemandIndex=7(gamma:NormalizedContextV1,
                     grammar:NormalizedGrammarValueV1,
                     scheme:NormalizedSchemeValueV1,
                     activation:NormalizedSectionV1)
| DischargeIndex=8(gamma:NormalizedContextV1,
                    grammar:NormalizedGrammarValueV1,
                    scheme:NormalizedSchemeValueV1,
                    activation:NormalizedSectionV1,
                    live:NormalizedLiveDemandValueV1)
```

The normalized compound types used above are defined topologically in section
7. A carrier index is a complete value. It is never a hash, serialized byte
string, or owner-local ordinal.

### 3.3 Nine carrier proposal variants

```text
CarrierProposalV1 ::=
  PublicContext=0(gamma:RawContextV1,here:Unit)
| PublicInterface=1(gamma:RawContextV1,a:RawTelescopeV1)
| InterfaceFamily=2(gamma:RawContextV1,
                     a:RawTelescopeV1,
                     b:RawTelescopeV1)
| Substitution=3(delta:RawContextV1,
                  gamma:RawContextV1,
                  images:RawSubstitutionValueV1)
| ComparisonWitness=4(gamma:RawContextV1,
                       a:RawTelescopeV1,
                       b:RawTelescopeV1,
                       route:ComparisonRouteV1)
| SealedPublicGrammar=5(gamma:RawContextV1,
                         grammar:RawGrammarValueV1)
| DemandScheme=6(gamma:RawContextV1,
                  grammar:RawGrammarValueV1,
                  scheme:RawSchemeValueV1)
| LiveDemand=7(gamma:RawContextV1,
                grammar:RawGrammarValueV1,
                scheme:RawSchemeValueV1,
                activation:RawSectionV1,
                live_value:RawLiveDemandValueV1)
| Discharge=8(gamma:RawContextV1,
               grammar:RawGrammarValueV1,
               scheme:RawSchemeValueV1,
               activation:RawSectionV1,
               live_value:RawLiveDemandValueV1,
               discharge:RawDischargeValueV1)
```

Each variant is exactly “raw indices, then raw value” in A2-O order. Repeated
mathematical indices are retained by value because Level II must replay their
agreement. In `LiveDemand`, the activation occurs exactly once as an index and
the following `live_value` is only the proposed specialized body. A checked
`Discharge` also retains the activation that makes its pure `live_value` body
well indexed. The mathematical notation `Discharge[Sigma,Gamma,G,S,L]`
leaves `a` implicit through the dependent formation of `L`; the first-order
wire retains it immediately before `live_value`. The checker independently
rebuilds `S,a -> L` and requires exact agreement before checking the proposed
discharge.

No carrier variant contains another `CarrierProposalV1` or a checked carrier.
The carrier tag is supplied only by this sum and is not repeated as a field.

## 4. Formal action proposals and root `0xe2`

```text
ContextSquareProposalV1 =
  (rho:SubstitutionProposalV1,
   sigma:SubstitutionProposalV1,
   theta_prime:SubstitutionProposalV1)
```

The three substitutions retain all endpoints and images. R2b checks each
substitution and the base-independent shared endpoints. Full square
applicability is relative to the base substitution `theta` projected from the
acted-on payload and is therefore checked only in R2d. There the fourth
pointwise comparison `epsilon` is derived and independently replayed; it is not
raw syntax. Neither action primitive contains `LevelOneProposalV1`.

`FormalActionSyntaxV1`, its two action-kind tags, and its postfix token program
are exactly A4-R2a section 8. The header kind must agree with every primitive
and composition.

Root `0xe2` is:

```text
LevelOneProposalV1 ::=
  CarrierProposal=0(CarrierProposalV1)
| RawConstructorProposal=1(RawConstructorCodeV1)
| FormalActionProposal=2(FormalActionSyntaxV1)
```

Canonical bytes are root `0xe2`, `u16=1`, then the displayed outer tag and
payload. The nested carrier/constructor/action tag is the only subtype header;
an implementation may cache it but may not encode a duplicate.

## 5. Root `0xe4`: exact seven raw constructor products

### 5.1 Source-backed compound types

`SourceRefV1` and `SourceLeafV1` are the frozen R1/R2a types. Every occurrence
below is the full `(source_ordinal:u32,source:SourceRefV1)` value.

```text
CtxCodeV1 ::=
  Empty=0
| LocalPrefix=1(source:SourceLeafV1,prefix_length:u16)

TelCodeV1          = newtype vector<SourceLeafV1>
NonemptyTelCodeV1  = newtype TelCodeV1
SecCodeV1          = newtype vector<SourceLeafV1>
SubCodeV1          = newtype SecCodeV1

FamilyCodeV1 =
  (a:TelCodeV1,b:TelCodeV1)

OperationCodeV1 =
  (parameters:TelCodeV1,results:TelCodeV1)

GrammarCodeV1 =
  (exports:TelCodeV1,
   operations:vector<OperationCodeV1>)

PortCodeV1 =
  (operation_ordinal:u32,
   arguments:SecCodeV1)

RawContractCodeV1 =
  (ty:SourceLeafV1,
   left:SourceLeafV1,
   right:SourceLeafV1)

SchemeCodeV1 =
  (trigger:TelCodeV1,
   ports:vector<PortCodeV1>,
   contracts:vector<RawContractCodeV1>)

DischargeCodeV1 = newtype SecCodeV1
```

`CtxCodeV1::LocalPrefix` requires `source.source=Current(q)` and
`1 <= prefix_length <= min(q.binder_depth,N)`. `Empty` is the sole
empty-context code.
`NonemptyTelCodeV1` has length at least one. Every vector length is at most the
source-census cardinality `N`. The frozen ordinary census limit is exactly
`N <= 4096`; `u32` source-ordinal representability follows and is not a weaker
replacement bound. `PortCodeV1.operation_ordinal` is in bounds for the earlier
grammar operation vector.

The raw contract parse order is exactly `(type,left,right)`, so a decoder can
derive the expected type before checking either term. Successful build stores
the carrier order `(left,right,type)`. Derived port outputs, specialization,
comparison witnesses, normalization, and theorem subjects do not occur in raw
code.

One used-source bitset, keyed by checked `source_ordinal`, spans every nested
leaf of a complete `RawConstructorCodeV1`. Full `SourceRefV1` bytes must be
pairwise distinct. A repeated source is a visited rejected prefix and never a
complete raw code.

### 5.2 Seven variants and scalar widths

```text
RawConstructorCodeV1 ::=
  Formation=0(
    gamma:CtxCodeV1,
    a:NonemptyTelCodeV1)
| Abstraction=1(
    xi:CtxCodeV1,
    cut:u32,
    b:NonemptyTelCodeV1)
| Aggregation=2(
    gamma:CtxCodeV1,
    family:FamilyCodeV1,
    c:NonemptyTelCodeV1)
| Transport=3(
    delta:CtxCodeV1,
    gamma:CtxCodeV1,
    theta:SubCodeV1,
    a:NonemptyTelCodeV1,
    c:NonemptyTelCodeV1)
| Comparison=4(
    gamma:CtxCodeV1,
    a:NonemptyTelCodeV1,
    b:NonemptyTelCodeV1)
| DemandCompiler=5(
    gamma:CtxCodeV1,
    grammar:GrammarCodeV1,
    scheme:SchemeCodeV1,
    activation:SecCodeV1)
| DischargeTransformer=6(
    delta:CtxCodeV1,
    gamma:CtxCodeV1,
    theta:SubCodeV1,
    grammar:GrammarCodeV1,
    scheme:SchemeCodeV1,
    activation:SecCodeV1,
    discharge:DischargeCodeV1,
    discharge_raw:DischargeCodeV1)
```

`cut` is a little-endian `u32` and satisfies `cut < |Xi|`; it leaves the
nonempty suffix `A`. Context-prefix length is `u16`; source, operation, and
path element ordinals are `u32`; vector lengths remain canonical `u64`.
DemandCompiler requires a nonempty trigger and at least one port or contract.
DischargeTransformer requires a nonempty derived demanded telescope. These are
validation predicates over the fixed wire, not optional flags.

Rooted encoding is root `0xe4`, `u16=1`, then the constructor tag and fields.

### 5.3 Exact principal-origin paths

Each `RawFieldPathV1.constructor` equals the displayed sum tag. Its first step
is the byte-identical `SumPayload(tag)` check against root `0xe4`; transparent
newtypes are then entered automatically. The seven `OriginPathV1` bindings are:

```text
FormationOldestOutputType =
  Formation,[SumPayload(0),RecordField(1),VectorElement(0)]

AbstractionOldestFamilyType =
  Abstraction,[SumPayload(1),RecordField(2),VectorElement(0)]

AggregationOldestRawOutputType =
  Aggregation,[SumPayload(2),RecordField(2),VectorElement(0)]

TransportOldestRawOutputType =
  Transport,[SumPayload(3),RecordField(4),VectorElement(0)]

ComparisonOldestLeftType =
  Comparison,[SumPayload(4),RecordField(1),VectorElement(0)]

DemandCompilerOldestActivationImage =
  DemandCompiler,[SumPayload(5),RecordField(3),VectorElement(0)]

DischargeTransformerOldestRawOutputImage =
  DischargeTransformer,[SumPayload(6),RecordField(7),VectorElement(0)]
```

Every path ends at a complete `SourceLeafV1`. The sum tag is runtime checked;
the constructor header alone does not license entry into a different payload.

## 6. Checked decoding compounds

### 6.1 Exact success-replay slots

R3 owns call-role and resource receipts, but R2b fixes their outer positions:

```text
KernelSuccessReplayV1 =
  (call_role:R3::KernelCallRoleV1,
   raw_judgment:OpenJudgment,
   normalized_judgment:OpenJudgment,
   receipt:R3::KernelSuccessReceiptV1)

LengthCheckV1 =
  (path:RawFieldPathV1,
   relation:LengthRelationV1,
   observed:u64,
   bound:u64)

LengthRelationV1 ::=
  Equal=0 | AtMost=1 | NonemptyAtMost=2 | StrictlyLess=3

NormalizedTelescopeFieldComparisonV1 =
  (field_ordinal:u32,
   left_prefix:NormalizedContextV1,
   right_prefix:NormalizedContextV1,
   left_normalized_type:Term,
   right_normalized_type:Term,
   left_formation_replay:KernelSuccessReplayV1,
   right_formation_replay:KernelSuccessReplayV1)

NormalizedTelescopeComparisonV1 =
  (base_context:NormalizedContextV1,
   left:NormalizedTelescopeV1,
   right:NormalizedTelescopeV1,
   left_length:u64,
   right_length:u64,
   fields:vector<NormalizedTelescopeFieldComparisonV1>)

BitVectorV1 =
  (bit_count:u64,byte_count:u64,bytes:bytes)

SourceUseEvidenceV1 =
  (source_count:u64,
   used_sources:BitVectorV1,
   traversal:vector<(path:RawFieldPathV1,leaf:SourceLeafV1)>)
```

`source_count` is byte-equal to the selected census cardinality and equals
`used_sources.bit_count`. The frozen R2a bit-vector codec fixes its exact byte
count, bit order, and zero high padding. `traversal` is the exhaustive typed
raw-field preorder of the complete code; its leaf at each path
is byte-identical to the projected leaf, and a bit is set exactly when that
source ordinal occurs. Because complete raw codes reject repeated sources, the
bit-vector population equals the traversal length. Hence this record cannot
omit a dependency or insert a second occurrence while preserving the checked
raw code.

`NormalizedTelescopeComparisonV1` is not a kernel equality at a fabricated
universe. Both retained lengths equal their telescope lengths and each other;
the field vector has that same length and contiguous checked `u32` ordinals.
Both ordinal-zero prefixes equal `base_context`. Each later prefix is rebuilt
independently by appending the preceding normalized field types on its side;
the two prefix contexts have identical full canonical bytes. Each replay is
exactly `TypeFormation` for its displayed prefix/type, each displayed type is
the corresponding telescope projection, and the two normalized type terms
have identical full canonical bytes. No digest comparison,
`DefinitionallyEqual` call at a synthesized universe, eta rule, or proof-
irrelevance assumption may establish these equalities.

R3 must replace each qualified `R3::` slot with one exact earlier local type
without moving the R2b field. It may not add an error string, optional receipt,
or implementation counter. No R2b replay record contains A1 evidence,
an envelope, support object, quotient key, action image, or enclosing checked
wrapper.

### 6.2 Source views and checked compounds

```text
WeakeningViewV1 =
  (source_binder_keys:vector<BinderKeyV1>,
   target_prefix_binder_keys:vector<BinderKeyV1>,
   shift:u32,
   images:vector<Term>,
   sequential_replay:
     vector<KernelSuccessReplayV1>)

CheckedTypeViewV1 =
  (path:RawFieldPathV1,
   source:SourceLeafV1,
   target_context:NormalizedContextV1,
   weakening:WeakeningViewV1,
   source_term:Term,
   reindexed_term:Term,
   normalized_type:Term,
   formation_replay:KernelSuccessReplayV1)

CheckedTermViewV1 =
  (path:RawFieldPathV1,
   source:SourceLeafV1,
   target_context:NormalizedContextV1,
   weakening:WeakeningViewV1,
   source_term:Term,
   reindexed_term:Term,
   expected_type:Term,
   normalized_term:Term,
   typing_replay:KernelSuccessReplayV1)

CheckedContextV1 =
  (raw_code:CtxCodeV1,
   raw_context:RawContextV1,
   binder_keys:vector<BinderKeyV1>,
   normalized_context:NormalizedContextV1,
   formation_replays:vector<KernelSuccessReplayV1>)

CheckedTelescopeFieldV1 =
  (source:SourceLeafV1,
   path:RawFieldPathV1,
   introduced_binder:BinderKeyV1,
   checked_type:CheckedTypeViewV1)

CheckedTelescopeV1 =
  (raw_code:TelCodeV1,
   base_context:NormalizedContextV1,
   fields:vector<CheckedTelescopeFieldV1>,
   normalized:NormalizedTelescopeV1,
   end_context:NormalizedContextV1)

CheckedSectionFieldV1 =
  (source:SourceLeafV1,
   path:RawFieldPathV1,
   checked_term:CheckedTermViewV1)

CheckedSectionV1 =
  (raw_code:SecCodeV1,
   base_context:NormalizedContextV1,
   target:NormalizedTelescopeV1,
   fields:vector<CheckedSectionFieldV1>,
   normalized:NormalizedSectionV1)

CheckedSubstitutionV1 =
  (raw_code:SubCodeV1,
   section:CheckedSectionV1,
   checked_particular:
     VerifiedParticularOpenTypedSubstitutionV1)

CheckedGrammarOperationV1 =
  (parameters:CheckedTelescopeV1,
   results:CheckedTelescopeV1)

CheckedGrammarV1 =
  (exports:CheckedTelescopeV1,
   operations:vector<CheckedGrammarOperationV1>)

CheckedPortV1 =
  (operation_ordinal:u32,
   arguments:CheckedSectionV1,
   derived_output:NormalizedTelescopeV1)

CheckedContractV1 =
  (raw_code:RawContractCodeV1,
   checked_type:CheckedTypeViewV1,
   checked_left:CheckedTermViewV1,
   checked_right:CheckedTermViewV1,
   carrier_order_value:NormalizedContractValueV1)

CheckedSchemeV1 =
  (trigger:CheckedTelescopeV1,
   ports:vector<CheckedPortV1>,
   contracts:vector<CheckedContractV1>,
   normalized:NormalizedSchemeValueV1)

CheckedDischargeV1 =
  (raw_code:DischargeCodeV1,
   section:CheckedSectionV1,
   instantiated_contract_replays:
     vector<KernelSuccessReplayV1>,
   normalized:NormalizedDischargeValueV1)

ContextSplitReceiptV1 =
  (xi:NormalizedContextV1,
   cut:u32,
   gamma:NormalizedContextV1,
   suffix_a:NormalizedTelescopeV1,
   prefix_binder_keys:vector<BinderKeyV1>,
   suffix_binder_keys:vector<BinderKeyV1>)
```

Every nested raw leaf retains its full typed path. Vector cardinalities must
match the raw code and derived arity exactly. A `CheckedContractV1` explicitly
records the raw parse order and the distinct normalized carrier-order value.
Its checked views prove the derivation of the normalized type/endpoints; raw
and normalized terms need not be byte-identical.

### 6.3 Typed source-resolution transcript

```text
SourceResolutionRequestV1 ::=
  LocalContextPrefix=0(
    path:RawFieldPathV1,
    source:SourceLeafV1,
    prefix_length:u16)
| TypeView=1(
    path:RawFieldPathV1,
    source:SourceLeafV1,
    target_context:NormalizedContextV1)
| TermView=2(
    path:RawFieldPathV1,
    source:SourceLeafV1,
    target_context:NormalizedContextV1,
    expected_type:Term)

SourceResolutionSuccessV1 ::=
  LocalContextPrefix=0(value:CheckedContextV1)
| TypeView=1(value:CheckedTypeViewV1)
| TermView=2(value:CheckedTermViewV1)

SourceResolutionResultV1 ::=
  Success=0(SourceResolutionSuccessV1)
| False=1(R3::CheckedFalseReceiptV1)
| Abort=2(R3::AbortReceiptV1)

SourceResolutionSuccessEntryV1 =
  (call_ordinal:u32,
   request:SourceResolutionRequestV1,
   success:SourceResolutionSuccessV1)

SourceResolutionTranscriptV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   calls:vector<SourceResolutionSuccessEntryV1>)

DecodeEvidenceV1 =
  (constructor:ConstructorTagV1,
   length_checks:vector<LengthCheckV1>,
   source_use:SourceUseEvidenceV1,
   source_resolution:SourceResolutionTranscriptV1,
   kernel_replays:vector<KernelSuccessReplayV1>,
   substitution_replays:
     vector<VerifiedParticularOpenTypedSubstitutionV1>)
```

`LocalContextPrefix` requires `source.source=Current(q)`, its path traverses
the enclosing byte-identical `SourceResolutionInputV1.raw_code` and ends at
that exact leaf inside `CtxCodeV1::LocalPrefix`, and
`1 <= prefix_length <= min(q.binder_depth,N)`. `CtxCodeV1::Empty` makes no
source-resolution call. Type/term paths likewise end at their byte-identical
raw leaf in that same code. Their target context and the term view's expected
type are internal decoder state, never caller-selected expected judgments.
Request and success tags agree, and the checked success repeats the request
path, source, context, and expected type where applicable by full canonical
bytes.

The transcript binding equals its enclosing checked-decoded binding. Call
ordinals are contiguous vector positions in exact raw declared preorder:
oldest-first telescope/section entries, grammar exports then each operation's
parameters/results, and contracts in raw `(type,left,right)` order. Calls and
`source_use.traversal` are one-to-one with identical paths and leaves. A false
or abort terminates decoding and is retained only in its R3 result receipt;
the success transcript admits no omitted, extra, reordered, or duplicate
call. Owner/census drift, forbidden older-body access, or signature replay
failure is `Abort`; a well-scoped but ill-formed view is `False`.

`length_checks`, `kernel_replays`, and `substitution_replays` are likewise
exhaustive and ordered by the same deterministic decoder traversal and the
A3-O field-local checking order. A replay repeated inside a checked view is
required byte-identical at its corresponding transcript position; an
implementation cannot replace the ordered vectors by sets or summaries.

### 6.4 Seven decoded variants and root `0xe5`

```text
DecodedConstructorValueV1 ::=
  Formation=0(
    gamma:CheckedContextV1,
    a_raw:CheckedTelescopeV1)
| Abstraction=1(
    xi:CheckedContextV1,
    cut:u32,
    gamma:NormalizedContextV1,
    a:NormalizedTelescopeV1,
    b_raw:CheckedTelescopeV1,
    split:ContextSplitReceiptV1)
| Aggregation=2(
    gamma:CheckedContextV1,
    a:CheckedTelescopeV1,
    b:CheckedTelescopeV1,
    c_raw:CheckedTelescopeV1)
| Transport=3(
    delta:CheckedContextV1,
    gamma:CheckedContextV1,
    theta:CheckedSubstitutionV1,
    a:CheckedTelescopeV1,
    c_raw:CheckedTelescopeV1)
| Comparison=4(
    gamma:CheckedContextV1,
    a_raw:CheckedTelescopeV1,
    b_raw:CheckedTelescopeV1)
| DemandCompiler=5(
    gamma:CheckedContextV1,
    grammar:CheckedGrammarV1,
    scheme:CheckedSchemeV1,
    activation_raw:CheckedSectionV1)
| DischargeTransformer=6(
    delta:CheckedContextV1,
    gamma:CheckedContextV1,
    theta:CheckedSubstitutionV1,
    grammar:CheckedGrammarV1,
    scheme:CheckedSchemeV1,
    activation_raw:CheckedSectionV1,
    discharge:CheckedDischargeV1,
    discharge_raw:CheckedDischargeV1)
```

The decoded tag must equal the raw constructor tag. Every derived field is
recomputed from earlier fields; it is retained for replay, not trusted.

```text
EvaluationBindingRefV1 =
  (owner_local_index:u32,
   full_identity:BirthSourceCensusV1)

CheckedDecodedPayloadV1 =
  (raw_code:RawConstructorCodeV1,
   decoded:DecodedConstructorValueV1,
   evidence:DecodeEvidenceV1)

CheckedDecodedV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   raw_code:RawConstructorCodeV1,
   decoded:DecodedConstructorValueV1,
   evidence:DecodeEvidenceV1)
```

`EvaluationBindingRefV1` is the specialization of R2a `OwnedRef` with owner
`EnvelopeCensus`; its index must select the byte-identical full
`BirthSourceCensusV1`. That census retains the exact
`BirthOpaqueSignatureBindingV1` and every source selected by a raw ordinal, so
the first field is simultaneously the full checkable `Sigma(H,o)` binding and
the owner required by `SourceLeafV1`. Root `0xe5` encodes `0xe5`, `u16=1`, then
these four fields. A digest cannot replace either binding.

`CheckedDecodedPayloadV1` is the unrooted projection of fields 1--3. It is the
exact A3-O `decoded` component retained by a built record: raw code, decoded
value, and all decoding evidence. Projection from root `0xe5` requires all
three fields to be byte-identical and adds no wrapper bytes.

## 7. Topologically ordered normalized carrier values

```text
NormalizedSubstitutionV1 =
  (delta:NormalizedContextV1,
   gamma:NormalizedContextV1,
   images:NormalizedSectionV1)

NormalizedFamilyValueV1 =
  (gamma:NormalizedContextV1,
   a:NormalizedTelescopeV1,
   b:NormalizedTelescopeV1)

NormalizedComparisonValueV1 =
  (gamma:NormalizedContextV1,
   left:NormalizedTelescopeV1,
   right:NormalizedTelescopeV1,
   route:ComparisonRouteV1)

NormalizedGrammarOperationV1 =
  (parameters:NormalizedTelescopeV1,
   results:NormalizedTelescopeV1)

NormalizedGrammarValueV1 =
  (exports:NormalizedTelescopeV1,
   operations:vector<NormalizedGrammarOperationV1>)

NormalizedPortValueV1 =
  (operation_ordinal:u32,
   arguments:NormalizedSectionV1,
   derived_output:NormalizedTelescopeV1)

NormalizedContractValueV1 =
  (left:Term,right:Term,ty:Term)

NormalizedSchemeValueV1 =
  (trigger:NormalizedTelescopeV1,
   ports:vector<NormalizedPortValueV1>,
   contracts:vector<NormalizedContractValueV1>)

NormalizedLiveDemandValueV1 =
  (demanded_telescope:NormalizedTelescopeV1,
   inherited_operation_ordinals:vector<u32>,
   specialized_contracts:vector<NormalizedContractValueV1>)

NormalizedDischargeValueV1 =
  (section:NormalizedSectionV1)
```

These records are ordered as written in the final schema DAG. They contain no
carrier-proposal wrapper and therefore create no Level-I recursion. A
comparison retains the singleton route and both endpoints but no fabricated
kernel proof term.

```text
NormalizedCarrierValueV1 ::=
  PublicContext=0(gamma:NormalizedContextV1,here:Unit)
| PublicInterface=1(gamma:NormalizedContextV1,
                     a:NormalizedTelescopeV1)
| InterfaceFamily=2(value:NormalizedFamilyValueV1)
| Substitution=3(value:NormalizedSubstitutionV1)
| ComparisonWitness=4(value:NormalizedComparisonValueV1)
| SealedPublicGrammar=5(gamma:NormalizedContextV1,
                         grammar:NormalizedGrammarValueV1)
| DemandScheme=6(gamma:NormalizedContextV1,
                  grammar:NormalizedGrammarValueV1,
                  scheme:NormalizedSchemeValueV1)
| LiveDemand=7(gamma:NormalizedContextV1,
                grammar:NormalizedGrammarValueV1,
                scheme:NormalizedSchemeValueV1,
                activation:NormalizedSectionV1,
                live:NormalizedLiveDemandValueV1)
| Discharge=8(gamma:NormalizedContextV1,
               grammar:NormalizedGrammarValueV1,
               scheme:NormalizedSchemeValueV1,
               activation:NormalizedSectionV1,
               live:NormalizedLiveDemandValueV1,
               discharge:NormalizedDischargeValueV1)
```

`NormalizedGrammarValueV1`, `NormalizedSchemeValueV1`,
`NormalizedLiveDemandValueV1`, and `NormalizedDischargeValueV1` are the pure
carrier bodies `G`, `S`, `L`, and `d`. The full indexed carrier sum immediately
above retains their ambient `Gamma/G/S/a/L` indices exactly once in A2-O order.
Built records may repeat those indices where A3-O explicitly requires them,
but a pure body never silently expands into the full indexed carrier.

## 8. Root `0xe6`: exact checked-built records

### 8.1 Build evidence

```text
SpecializedContractV1 =
  (source_contract_ordinal:u32,
   source_contract:CheckedContractV1,
   substituted_type:Term,
   substituted_left:Term,
   substituted_right:Term,
   substitution_replays:vector<KernelSuccessReplayV1>,
   normalized:NormalizedContractValueV1)

CheckedLiveSpecializationV1 =
  (activation:CheckedSectionV1,
   demanded_telescope:NormalizedTelescopeV1,
   inherited_operation_ordinals:vector<u32>,
   specialized_contracts:vector<SpecializedContractV1>,
   normalized:NormalizedLiveDemandValueV1)

CheckedDischargeTransformationV1 =
  (source:CheckedDischargeV1,
   raw_target:CheckedDischargeV1,
   normalized_output:NormalizedDischargeValueV1,
   contract_replays:vector<KernelSuccessReplayV1>)

BuildDerivedEvidenceV1 ::=
  Formation=0(output:NormalizedTelescopeV1)
| Abstraction=1(split:ContextSplitReceiptV1,
                 output:NormalizedFamilyValueV1)
| Aggregation=2(concatenated:NormalizedTelescopeV1,
                output_comparison:NormalizedTelescopeComparisonV1)
| Transport=3(reindexed:NormalizedTelescopeV1,
              output_comparison:NormalizedTelescopeComparisonV1)
| Comparison=4(witness:NormalizedComparisonValueV1,
               endpoint_comparison:NormalizedTelescopeComparisonV1)
| DemandCompiler=5(specialization:CheckedLiveSpecializationV1)
| DischargeTransformer=6(
    transformation:CheckedDischargeTransformationV1)

BuildPremiseEvidenceV1 =
  (constructor:ConstructorTagV1,
   anchor_path:RawFieldPathV1,
   principal_leaf:SourceLeafV1,
   anchor:OccurrenceId14V1,
   kernel_replays:vector<KernelSuccessReplayV1>,
   substitution_replays:
     vector<VerifiedParticularOpenTypedSubstitutionV1>,
   telescope_comparisons:
     vector<NormalizedTelescopeComparisonV1>,
   derived:BuildDerivedEvidenceV1)
```

The principal leaf must be `Current(anchor)`. `BuildPremiseEvidenceV1` contains
no envelope, A1 evidence, support, quotient key, final action key, action image,
or enclosing `CheckedBuiltV1`. The `derived` tag equals `constructor`; every
normalized result repeated in the built payload is required byte-identical.
A `SpecializedContractV1` retains its source contract and separately substituted
terms, so substitution is never mistaken for raw/normalized byte identity. Its
`source_contract_ordinal` and evidence-vector position agree, one entry exists
per source scheme contract, and the live operation-anchor vector has exactly
one entry per scheme port in port order.

`anchor_path` is exactly the section-5.3 `RawFieldPathV1` selected by the
constructor's frozen `OriginPathV1`; `principal_leaf` is the byte-identical
projection at that path, and `anchor` is the occurrence inside that exact
`Current(anchor)` leaf.

`telescope_comparisons` contains exactly one entry for Aggregation,
Transport, and Comparison and is empty for the other four constructors. Its
singleton is byte-identical to the comparison retained by `derived`. Kernel
`DefinitionallyEqual` replays for discharge contracts remain in their typed
kernel/contract evidence and never substitute for a telescope comparison.
The canonical directions are: Aggregation compares the concatenated `A.B` on
the left with normalized `C_raw` on the right over `Gamma`; Transport compares
reindexed `A[theta]` on the left with normalized `C_raw` on the right over
`Delta`; and Comparison compares normalized `A` on the left with normalized
`B` on the right over `Gamma`.

### 8.2 Built variant records

Every variant has this exact common prefix:

```text
(raw_code,decoded,anchor,indices,nominal_inputs,nominal_output,raw_realization)
```

The sum tag precedes the prefix and supplies `c`. The complete variants are:

```text
BuiltConstructorValueV1 ::=
  Formation=0(
    raw_code:RawConstructorCodeV1,
    decoded:CheckedDecodedPayloadV1,
    anchor:OccurrenceId14V1,
    indices:(gamma:NormalizedContextV1),
    nominal_inputs:(here:Unit),
    nominal_output:NormalizedTelescopeV1,
    raw_realization:CheckedTelescopeV1)
| Abstraction=1(
    raw_code:RawConstructorCodeV1,
    decoded:CheckedDecodedPayloadV1,
    anchor:OccurrenceId14V1,
    indices:(gamma:NormalizedContextV1,
             a:NormalizedTelescopeV1,
             gamma_a:NormalizedContextV1),
    nominal_inputs:(here:Unit,b:NormalizedTelescopeV1),
    nominal_output:NormalizedFamilyValueV1,
    raw_realization:(xi_raw:CheckedContextV1,
                     cut:u32,
                     b_raw:CheckedTelescopeV1))
| Aggregation=2(
    raw_code:RawConstructorCodeV1,
    decoded:CheckedDecodedPayloadV1,
    anchor:OccurrenceId14V1,
    indices:(gamma:NormalizedContextV1,
             a:NormalizedTelescopeV1,
             b:NormalizedTelescopeV1),
    nominal_inputs:(family:NormalizedFamilyValueV1),
    nominal_output:NormalizedTelescopeV1,
    raw_realization:CheckedTelescopeV1)
| Transport=3(
    raw_code:RawConstructorCodeV1,
    decoded:CheckedDecodedPayloadV1,
    anchor:OccurrenceId14V1,
    indices:(delta:NormalizedContextV1,gamma:NormalizedContextV1),
    nominal_inputs:(theta:NormalizedSubstitutionV1,
                    a:NormalizedTelescopeV1),
    nominal_output:NormalizedTelescopeV1,
    raw_realization:CheckedTelescopeV1)
| Comparison=4(
    raw_code:RawConstructorCodeV1,
    decoded:CheckedDecodedPayloadV1,
    anchor:OccurrenceId14V1,
    indices:(gamma:NormalizedContextV1,
             a:NormalizedTelescopeV1,
             b:NormalizedTelescopeV1),
    nominal_inputs:(left:NormalizedTelescopeV1,
                    right:NormalizedTelescopeV1),
    nominal_output:NormalizedComparisonValueV1,
    raw_realization:(a_raw:CheckedTelescopeV1,
                     b_raw:CheckedTelescopeV1))
| DemandCompiler=5(
    raw_code:RawConstructorCodeV1,
    decoded:CheckedDecodedPayloadV1,
    anchor:OccurrenceId14V1,
    indices:(gamma:NormalizedContextV1,
             grammar:NormalizedGrammarValueV1,
             scheme:NormalizedSchemeValueV1,
             activation:NormalizedSectionV1),
    nominal_inputs:(grammar:NormalizedGrammarValueV1,
                    scheme:NormalizedSchemeValueV1),
    nominal_output:NormalizedLiveDemandValueV1,
    raw_realization:CheckedSectionV1)
| DischargeTransformer=6(
    raw_code:RawConstructorCodeV1,
    decoded:CheckedDecodedPayloadV1,
    anchor:OccurrenceId14V1,
    indices:(delta:NormalizedContextV1,
             gamma:NormalizedContextV1,
             grammar:NormalizedGrammarValueV1,
             scheme:NormalizedSchemeValueV1,
             activation:NormalizedSectionV1,
             live:NormalizedLiveDemandValueV1),
    nominal_inputs:(theta:NormalizedSubstitutionV1,
                    discharge:NormalizedDischargeValueV1),
    nominal_output:NormalizedDischargeValueV1,
    raw_realization:CheckedDischargeV1)
```

The built sum tag, `raw_code` tag, and `decoded.decoded` tag must agree. Fields repeated
by the A3-O common-prefix contract are retained byte-identically; they are not
silently deduplicated. In particular, root `0xe6` retains `decoded` once as its
second outer field while `BuiltConstructorValueV1` retains the same complete
checked-decoded payload in its second inner field. The verifier requires full canonical
byte equality. The same rule relates each inner `raw_code` to the raw code
retained by the checked-decoded payload.

```text
CheckedBuiltV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   decoded:CheckedDecodedPayloadV1,
   built:BuiltConstructorValueV1,
   evidence:BuildPremiseEvidenceV1)
```

Root `0xe6` encodes `0xe6`, `u16=1`, then those four fields. No digest or arena
index alone can stand for either repeated value. Its outer
`evaluation_binding` equals the binding inside every repeated decoded
payload's source-resolution transcript and the binding of the
`CheckedDecodedV1` consumed by `BuildDecoded`, all by full canonical bytes.

## 9. Early Level-II payloads and the five-tag contract

```text
CarrierFormationEvidenceV1 =
  (kernel_replays:vector<KernelSuccessReplayV1>,
   substitution_replays:
     vector<VerifiedParticularOpenTypedSubstitutionV1>,
   telescope_comparisons:
     vector<NormalizedTelescopeComparisonV1>,
   normalized_index:CarrierIndexValueV1)

CheckedCarrierPayloadV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   raw_proposal:CarrierProposalV1,
   normalized_value:NormalizedCarrierValueV1,
   evidence:CarrierFormationEvidenceV1)
```

The carrier and index tags must agree. Every normalized index, raw index, and
normalized value is retained and byte-agreement is replayed; none is inferred
from an untyped tag. For `ComparisonWitness`, `telescope_comparisons` contains
exactly one receipt whose base and endpoints agree with the normalized
index/value projections in the direction `base=gamma,left=a,right=b`; the
normalized value's route is exactly the
singleton `KernelNormalizedTypeEquality` tag. The vector is empty for every
other carrier tag. The surrounding kernel vector retains independent
formation calls only; it cannot replace the non-kernel byte comparison.

The three early Level-II payload types, in R1 order, are:

```text
CheckedCarrier       = CheckedCarrierPayloadV1
CheckedDecoded       = CheckedDecodedV1 (also root payload 0xe5)
CheckedBuilt         = CheckedBuiltV1 (also root payload 0xe6)
```

When either latter payload is nested under root `0xe3`, its schema fields are
encoded directly after the Level-II variant tag. The top-level `0xe5` or
`0xe6` root and version header is not duplicated inside root `0xe3`.

R2b freezes the final Level-II variant tags and semantic field order:

```text
CheckedOrdinaryLevelTwoV1 ::=
  CheckedCarrier=0(
    Sigma binding,raw carrier proposal,normalized value,
    full formation/replay evidence)
| CheckedDecoded=1(
    Sigma binding,raw code,decoded value,decode evidence)
| CheckedBuilt=2(
    Sigma binding,decoded value,Built_c,premise evidence)
| CheckedCandidateEnvelope=3(
    kappa,H,o,Sigma(H,o),c,r,E,Match evidence)
| CheckedActionImage=4(
    kappa,H,o,Sigma(H,o),c,owned base candidate,
    normalized action,owned trace,image)
```

The first three variants use the exact payload schemas above. R2c and R2d must
replace every mathematical name in variants 3 and 4 by a complete typed value
or checked owner-local reference carrying its full identity. R2d then creates
the five-way sum node and maps root `0xe3` to it. R2b does not assign a forward
`TypeIdV1` or encode an incomplete `0xe3` object.

No Level-II variant is a pair disposition or occurrence classification.

## 10. R2b result schemas and static operation definitions

R2b supplies these typed runtime result schemas and the corresponding
history-neutral nodes for the final registry operation DAG:

```text
RawEnumerationV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   constructor:ConstructorTagV1,
   codes:vector<RawConstructorCodeV1>,
   cardinality:R3::StructuralCardinalityReceiptV1)

CarrierCheckResultV1 ::=
  Success=0(CheckedCarrierPayloadV1)
| False=1(R3::CheckedFalseReceiptV1)
| Abort=2(R3::AbortReceiptV1)

RawEnumerationResultV1 ::=
  Success=0(RawEnumerationV1)
| Abort=1(R3::AbortReceiptV1)

DecodeRawResultV1 ::=
  Success=0(CheckedDecodedV1)
| False=1(R3::CheckedFalseReceiptV1)
| Abort=2(R3::AbortReceiptV1)

BuildDecodedResultV1 ::=
  Success=0(CheckedBuiltV1)
| False=1(R3::CheckedFalseReceiptV1)
| Abort=2(R3::AbortReceiptV1)

FormalActionSyntaxCheckResultV1 ::=
  Success=0(CheckedFormalActionSyntaxV1)
| False=1(R3::CheckedFalseReceiptV1)
| Abort=2(R3::AbortReceiptV1)

SourceResolutionInputV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   raw_code:RawConstructorCodeV1,
   request:SourceResolutionRequestV1)

CarrierCheckInputV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   proposal:CarrierProposalV1)

RawEnumerationInputV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   constructor:ConstructorTagV1)

DecodeRawInputV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   raw_code:RawConstructorCodeV1)

FormalActionSyntaxCheckInputV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   raw_syntax:FormalActionSyntaxV1)

R2bSemanticOpcodeV1 ::=
  ResolveSource=0
| EnumerateRaw=1
| CheckCarrier=2
| DecodeRaw=3
| BuildDecoded=4
| CheckFormalActionSyntax=5

R2bOperationDefinitionV1 =
  (slot_local_ordinal:u32,
   opcode:R2bSemanticOpcodeV1,
   input_type:TypeIdV1,
   output_type:TypeIdV1,
   predecessor_rules:vector<DefinitionRulePathV1>)

R2bOperationSlotContributionV1 =
  (protocol_slot:ProtocolSlotV1,
   definitions:vector<R2bOperationDefinitionV1>)

R2bOperationDefinitionContributionV1 =
  (slots:vector<R2bOperationSlotContributionV1>)
```

These are history-neutral operation-definition **contributions**, not concrete
evaluation inputs or outputs embedded in root `0xe0`. The `slots` vector has
exactly the five entries `0,1,2,3,10` in that order. Each nested definition
vector has contiguous local ordinals. R2e replaces each named input/output
type below by its exact nonzero, topologically earlier final `TypeIdV1`. The
closed mapping is:

| Protocol slot | Local | Opcode | Input type | Output type | Predecessor |
| --- | ---: | --- | --- | --- | --- |
| `SourceResolution=0` | 0 | `ResolveSource` | `SourceResolutionInputV1` | `SourceResolutionResultV1` | none |
| `RawGeneration=1` | 0 | `EnumerateRaw` | `RawEnumerationInputV1` | `RawEnumerationResultV1` | none |
| `Decode=2` | 0 | `CheckCarrier` | `CarrierCheckInputV1` | `CarrierCheckResultV1` | none |
| `Decode=2` | 1 | `DecodeRaw` | `DecodeRawInputV1` | `DecodeRawResultV1` | `SourceResolution[0]` |
| `Build=3` | 0 | `BuildDecoded` | `CheckedDecodedV1` | `BuildDecodedResultV1` | `Decode[1]` |
| `ActionNormalizationAndRebuild=10` | 0 | `CheckFormalActionSyntax` | `FormalActionSyntaxCheckInputV1` | `FormalActionSyntaxCheckResultV1` | none |

Here `SourceResolution[0]` is exactly
`DefinitionRulePathV1(SourceResolution,[VectorElement(0)])` and `Decode[1]`
is exactly `DefinitionRulePathV1(Decode,[VectorElement(1)])`; the table admits
no implicit dependency edge.

`CheckFormalActionSyntax` is only the slot-10 precursor. R2d must append the
base-relative normalization/rebuild operation at local ordinal 1. After all
R2c/R2d opcodes exist, R2d must close, without yet assigning TypeIds,

```text
JoinedSemanticOpcodeV1 ::=
  R2b=0(R2bSemanticOpcodeV1)
| R2c=1(R2cSemanticOpcodeV1)
| R2d=2(R2dSemanticOpcodeV1)

JoinedOperationDefinitionV1 =
  (slot_local_ordinal:u32,
   opcode:JoinedSemanticOpcodeV1,
   input_type:TypeIdV1,
   output_type:TypeIdV1,
   predecessor_rules:vector<DefinitionRulePathV1>)

JoinedOperationSlotV1 =
  (protocol_slot:ProtocolSlotV1,
   definitions:vector<JoinedOperationDefinitionV1>)

JoinedOperationDefinitionDagV1 =
  (slots:vector<JoinedOperationSlotV1>)
```

The joined `slots` vector has exactly twelve entries and
`slots[i].protocol_slot` has tag `i`. A `DefinitionRulePathV1` tag selects
`slots[tag].definitions`; its first step
`VectorElement(slot_local_ordinal)` therefore addresses the actual local
vector, not a filtered global list. Every predecessor is lexicographically
earlier and validates against that referenced definition. R2e canonically
injects each R2b definition by copying its ordinal, input/output TypeIds, and
predecessors byte-for-byte and wrapping only its opcode as
`JoinedSemanticOpcodeV1::R2b`. It joins later slot contributions in local
ordinal order and may not rename an opcode, move a slot, change an ordinal, or
add a predecessor here.

Concrete success transcripts live inside the checked result schemas above;
false/abort values live in the later outcome machine, never in the static
operation-definition DAG.

`CheckedFormalActionSyntaxV1` is:

```text
(evaluation_binding:EvaluationBindingRefV1,
 action_kind:ActionKindV1,
 raw_syntax:FormalActionSyntaxV1,
 normalized_primitives:vector<CheckedPrimitiveActionSyntaxV1>,
 replay_evidence:vector<KernelSuccessReplayV1>)
```

where

```text
CheckedPrimitiveActionSyntaxV1 ::=
  Substitution=0(VerifiedParticularOpenTypedSubstitutionV1)
| ContextSquare=1(rho:VerifiedParticularOpenTypedSubstitutionV1,
                   sigma:VerifiedParticularOpenTypedSubstitutionV1,
                   theta_prime:VerifiedParticularOpenTypedSubstitutionV1)
```

Each operation uses the exact final local TypeIds assigned in R2e. Its
semantics are the deterministic validation/replay algorithms frozen in
A2-O, A3-O, and this record, not a source-code symbol. `ResolveSource` checks
the request path and leaf against its byte-identical enclosing `raw_code` and
implements exactly section 6.3's result partition. `EnumerateRaw` follows
length-first/product-lexicographic/depth-first A3 order and calls `DecodeRaw`
only after a complete structural raw value exists. Decoder failure cannot
prune a raw subtree. An empty raw universe is `Success`, never `False`;
enumeration has only the displayed success/abort partition. `BuildDecoded`
derives every output and anchor and accepts no expected value.

On enumeration success, `codes` is the complete A3-O structural order, its
length equals the count retained by `cardinality`, is at most the frozen
`CompleteRawCodesPerPair=262,144` ceiling and hence at most `2^32`, and every
raw ordinal is the checked `u32` conversion of its zero-based vector position.
Neither semantic decoder failure nor a resource receipt can delete or reorder
a completed structural code.

`CheckFormalActionSyntax` checks the typed postfix stack, action kind, and each
primitive substitution under the exact `Sigma(H,o)` binding. For a square it
may check the three standalone substitutions and their base-independent shared
endpoints only. It cannot derive `epsilon`, because `epsilon` compares
`theta.rho` with `sigma.theta_prime` and `theta` is projected from the acted-on
Transport or DischargeTransformer payload. R2d's action-rebuild operation must
consume the checked syntax plus the exact typed base-candidate action
projection, verify both remaining endpoints, derive/replay `epsilon`, and only
then construct a checked action trace/image. R2b's syntax result is not a
Level-II action or applicability fact.

The operation DAG contains no history selection, result classifier, theorem
proof, publication step, or action-image constructor.

## 11. Freeze acceptance audit

R2b froze only after an independent audit confirmed:

1. every new record field and sum tag is exactly the displayed order;
2. all seven raw variants match A3-O and use explicit `u16`/`u32` widths;
3. `CtxCodeV1::LocalPrefix` retains a full `SourceLeafV1`, not a bare
   occurrence;
4. one global source bitset covers every nested raw leaf;
5. the seven principal paths traverse a matching runtime sum tag and end at a
   complete `SourceLeafV1`;
6. raw contract order `(type,left,right)` and checked carrier order
   `(left,right,type)` are both retained and cross-checked;
7. all decoded/built tags, repeated raw/decoded values, indices, and normalized
   values agree by full canonical bytes;
8. constructor raw codes accept no derived port output, specialization,
   comparison witness, `epsilon`, constructor tag, or expected judgment; a
   Level-I carrier proposal may propose its complete raw carrier body, but no
   such body is trusted as its own formation or normalization evidence;
9. evaluation references retain owner index plus the full
   `BirthSourceCensusV1`, including its `BirthOpaqueSignatureBindingV1`;
10. evidence contains exact judgments and replay slots, never an unexplained
    byte vector or digest-only proof;
11. no carrier or action primitive recursively embeds `LevelOneProposalV1`;
12. carrier/decode/build/formal-syntax checks retain exact success/false/abort
    partitions, while empty raw enumeration is success and resource failure is
    abort;
13. context-square applicability and `epsilon` are deferred to the R2d
    operation that owns the exact base payload;
14. root `0xe3` is not materialized before the R2c/R2d payload types; and
15. none of the four rooted objects or early Level-II payloads carries profile,
    candidate, disposition, theorem, or publication authority;
16. every telescope comparison is equal-length, prefix-wise formation plus
    canonical normalized-type byte equality, never a fabricated kernel
    equality judgment;
17. source resolution has its typed success/false/abort node and the static
    operation-definition DAG binds every R2b opcode to its exact R2a protocol
    slot without embedding history-specific values; and
18. successful raw enumeration retains the complete structural vector, exact
    cardinality, contiguous checked `u32` position ordinals, and exact census-
    keyed source-use traversal.

The independent audit first rejected loss of the activation that types a
discharge's `L`, misuse of kernel judgments for normalized-telescope byte
comparison, absence of the source-resolution protocol node, a history-specific
operation “DAG,” incomplete raw-ordinal/source-bitset rules, and two mistyped
validation clauses. The repaired record retains the full dependent activation,
introduces exact non-kernel telescope-comparison and typed source-resolution
receipts, separates runtime results from per-slot static operation
contributions, freezes the later three-way opcode/twelve-slot join, and closes
the exact raw enumeration invariants. Two fresh independent whole-file audits
then returned **PASS** on all eighteen conditions. This freezes R2b's layouts
and semantic positions; it does not pre-approve R2c--R2e or fill an
R3-qualified type.

The remaining qualified `R3::KernelCallRoleV1`,
`R3::KernelSuccessReceiptV1`, `R3::StructuralCardinalityReceiptV1`,
`R3::CheckedFalseReceiptV1`, and `R3::AbortReceiptV1` nodes are explicit
R3-owned typed slots whose
outer positions are frozen here. They are not extension bytes. R3 must define
them earlier in the final type DAG without changing an R2b field; only R2e may
then assign final TypeIds and claim complete canonical registry bytes.

R2c may now consume these frozen field boundaries. No downstream gate may
reinterpret them or cite R2b as closure of a later candidate, action, failure,
resource, registry, fixture, or executable-authority gate.

## 12. Continuation and non-authority

With R2b frozen, active R2c must instantiate A1 fields 1--17, exact evidence,
support projections, subject syntax, and roots `0xfb`/`0xfc` without embedding
an enclosing envelope. R2d must then close quotient/outcome/coverage/action
types and materialize root `0xe3`. R3 closes the qualified typed slots above
and the full failure/resource machine. R2e performs the final topological
TypeId assignment, registry reconstruction, codec round trips, reference-source
measurement, and ownership audit before R4 regenerates fixtures.

This record selects no history, occurrence, constructor, raw code, carrier,
action, or outcome. It mints no definition token, checked envelope, action
image, coverage certificate, pair disposition, classifier fact, theorem,
generic law, cubical bridge, `GCap`, `gamma`, or selective authority.
