# JG2b2b2a-A4-R2c Envelope, Evidence, Support, and Subject Schemas

Date: 2026-08-02

Status: **A4-R2c FROZEN 2026-08-02 WITH NARROW C2 SUBJECT-TYPE CORRIGENDUM
2026-08-03; A4-R2d ACTIVE AT QG3.** This record closes the
candidate-envelope half of the repaired ordinary representation: the exact A1
fields 1--17, the nonrecursive roots `0xfb` and `0xfc`, public-field lineage,
normalization, candidate structural support, the ordinary empty replacement
certificate, the theorem-subject language, and the candidate payload reserved
by root `0xe3`; its slot `8[2]` output is intentionally rank-free. The original
base freeze passed two independent audits against candidate SHA-256
`5786B95E2820570212401CC9BE4E0B47CCFCF1F409182FB2F9CB19FDDF22247F`.
That digest is retained as base-freeze provenance, not asserted as the hash of
this post-corrigendum file. The C2 corrigendum in subsection 5.3.0a closes
formal subject leaf types and exact output paths and consumes R2a-C2's one
append-only dependent-discharge proposition opcode. It has passed its own
independent dependency, typing, count, and route audits.
It does not close
the complete per-raw outcome pass, the candidate arena or final tag-3 payload
instance, quotient classes, pair coverage, dispositions, action traces/images,
root `0xe3`, R3 failure/resource payloads, the final registry, or executable
authority.

Normative inputs are:

- `docs/260802_jg2b2b2a_adequacy_and_envelope_protocol.md` (A1);
- `docs/260802_jg2b2b2a_indexed_ontology_profiles.md` (A2-O);
- `docs/260802_jg2b2b2a_a3_ordinary_constructor_schemas.md` (A3-O);
- `docs/260802_jg2b2b2a_a3_c1_expected_judgment_corrigendum.md`
  (post-R2b A3-C1);
- `docs/260802_jg2b2b2a_a4_ordinary_profile_closure.md`;
- `docs/260802_jg2b2b2a_a4_r1_signature_source_census.md`;
- `docs/260802_jg2b2b2a_a4_r2a_acyclic_schema_foundation.md`; and
- `docs/260802_jg2b2b2a_a4_r2b_level_relation_schemas.md`.

Every record field and sum variant below is zero-based in displayed order.
Every ordinal has the displayed fixed width. Every nested value uses its local
schema directly: a nested root object omits its top-level root and version
header unless this record explicitly says that canonical root bytes are
retained as `Bytes`. R2e assigns final `TypeIdV1` values only after R3 closes
the qualified typed slots; no name below is an opaque byte placeholder.

## 1. Gate allocation and acyclic construction order

R2c owns only candidate material. The exact split is:

```text
R2c:
  A1 fields 1--17;
  EnvelopeCore root 0xfb;
  ExactEvidence root 0xfc;
  candidate public leaves/lineage, normalization, support, dispositions,
    obligations, and subjects;
  rank-free CandidateMatch output and the reserved final
    CheckedCandidateEnvelope payload schema for future root 0xe3 tag 3;
  ProtocolSlotV1 contributions 4--8.

R2d:
  complete per-raw outcome pass, PairCandidateArena materialization, final
    candidate/tag-3 values, QuotientKey 0xfd, CheckedLeafOutcome 0xe7,
    PairCoverage 0xe8, PairDisposition 0xe9, ActionTrace 0xfe, ActionImage 0xff;
  action-image payload for future root 0xe3 tag 4;
  joined five-way root 0xe3 and slots 9--11 completion.

R3:
  closed false/abort, kernel-call, structural-cardinality, resource,
  allocation, budget, and meter receipt types already qualified below.
```

The candidate construction order is fixed:

```text
reminted (K,H,kernel) and BirthSourceCensusV1
  -> CheckedBuiltV1
  -> public leaves, all field/output lineage, footprints, boundary replay,
     obligations, and SubjectRootProjectionV1
  -> normalization evidence
  -> candidate-only support shape, owner resolution, closure, quotient,
     coverage, and pure SupportProjectionV1
  -> empty-grammar field dispositions
  -> full SubjectSyntaxV1 with byte-identical retained root projection
  -> A1 fields 1--16
  -> root-0xfb core bytes and root-0xfc exact-evidence bytes
  -> nonrecursive A1 field 17
  -> rank-free checked candidate and Match evidence
  -> [R2d] receipt-free quotient projection
  -> [R2d] complete per-raw outcome pass and rank-bearing candidate arena.
```

The type DAG follows the same dependencies except that the nonrecursive
`SubjectSyntaxV1` metalanguage is defined before its runtime instances. The
upstream correction order is `A3-O base -> R2a+C2 -> R2b -> A3-C1 -> R2c+C2`;
C1's two
payload types are therefore already acyclic here. A
subject literal may name a pure support-*function call descriptor*; it cannot
contain `OrdinarySupportV1` or its computed result. Thus normalization may
retain a support-projection proposition, support may later retain the same
proposition by value, and neither points back to an enclosing evidence or
envelope object.

### 1.1 Fixed R3 account scopes

Every displayed `R3::EnvelopeResourceAccountV1` position is R3-qualified but
is not a free receipt slot. Throughout this record, `Acct_R3(scope,events)`
denotes the unique canonical account for the named scope and exact ordered
event stream. `Aggregate_R3(scope,accounts,aggregate_only_events)` denotes the
unique ordered, scope-tagged, overflow-checked conservation aggregate: each
listed child account contributes exactly once and only the final event stream
is charged directly by the aggregate. The symbolic scope names used below are
closed distinct inputs that R3 must encode without changing these outer field
positions.

Every `*Events` compiler below is a deterministic schema-DAG preorder with no
caller-selected branch. It never descends into the counters or value bytes of
a nested `EnvelopeResourceAccountV1`, including the account being constructed;
where canonical encoding requires a length, it treats a finalized account as
one opaque typed byte span of its R3-closed encoded length. It otherwise retains
the exact receipt-validation, structural, canonical-encoding, and operation
events explicitly named for its scope. Consequently neither `Acct_R3` nor
`Aggregate_R3` is recursive. R3 closes their internal counter/limit/meter
layout and proves conservation; it may not change the scopes, event order,
child-account order, or opaque-account treatment fixed here.

## 2. Owner-bound identities and common candidate binding

The verified census owns the opaque authorities. Canonical candidate records
therefore retain checked owner-local references plus complete typed identities:

```text
EnvelopeProfileRefV1 =
  (owner_local_index:u32,
   full_identity:ProfileManifestIdV1)

CompleteHistoryIdentityV1 =
  (complete_through_head_commitment:Digest)

EnvelopeHistoryRefV1 =
  (owner_local_index:u32,
   full_identity:CompleteHistoryIdentityV1)

CheckedBuiltRefV1 =
  (owner_local_index:u32,
   full_identity:CheckedBuiltV1).
```

These are exact specializations of R2a `OwnedRef` with owner kind
`EnvelopeCensus`. Their object types are respectively the full profile
manifest privately authenticated by `K`, the complete-through-head object
privately authenticated by `H`, and `CheckedBuiltV1`. R2e assigns their earlier
local object/identity TypeIds; the opaque Rust authority type itself is never
encoded. The two `PairCandidateArena` specializations are deliberately defined
only after their identity types in sections 9.3 and 10; a forward type
reference is not licensed by this summary.

For `EnvelopeHistoryRefV1`, the enclosed digest is exactly the privately
owned token's `complete_through_head_commitment_digest`; it is not caller
input and is not evidence in place of `H`. For `CheckedBuiltRefV1`, the full
typed identity is the complete value, not a digest.

Owner-local indices are canonical data, not insertion order. The unique
profile and unique complete history owned by an ordinary envelope census both
have index `0`. Evaluation-census objects are sorted by
`(target_birth_event_ordinal, full BirthSourceCensusV1 bytes)` and reject a
duplicate full identity. Checked-built objects are sorted by
`(full EvaluationBindingRefV1 bytes, ConstructorTagV1 tag,
reconstructed raw_ordinal)` and likewise reject a duplicate complete value;
`CheckedBuiltRefV1.owner_local_index` is the zero-based rank in that order.
Every displayed index is recomputed from the privately owned complete arena
before encoding.

The six A1 parent identities have an exact closed sum:

```text
EnvelopeParentIdentityV1 ::=
  JG1=0(manifest_digest:Digest)
| JG2a=1(constructor_manifest_digest:Digest,
          scope_grammar_digest:Digest)
| JG2b2a=2(protocol_manifest_digest:Digest)
| JG2b2b0=3(structural_grammar_manifest_digest:Digest)
| JG2b2b1=4(particular_substitution_manifest_digest:Digest)
| Profile=5(profile_manifest_id:ProfileManifestIdV1)

ParentBindingRefV1 =
  (owner_local_index:u32,
   full_identity:EnvelopeParentIdentityV1)

EnvelopeParentBindingsV1 =
  (jg1:ParentBindingRefV1,
   jg2a:ParentBindingRefV1,
   jg2b2a:ParentBindingRefV1,
   jg2b2b0:ParentBindingRefV1,
   jg2b2b1:ParentBindingRefV1,
   profile:ParentBindingRefV1).
```

Each field's parent tag and `owner_local_index` equal its zero-based field
position. Every identity is freshly projected from the reminted authority
owned by the verified census. The profile tag is the same full
`ProfileManifestIdV1` as the enclosing `EnvelopeProfileRefV1`. A digest
identifies the privately owned parent; it never replaces reminting or parent-
DAG comparison. Field 2 therefore has exactly six entries and no extra owner
field.

Field 1 uses:

```text
EnvelopeComponentTagV1 ::=
  EnvelopeProtocol=0
| OrdinaryOntology=1
| OrdinaryConstructorSchemas=2
| LevelRelation=3
| CanonicalCodec=4
| CoverageGrammar=5
| ActionImageGrammar=6
| ResourcePolicy=7

ComponentVersionV1 =
  (component:EnvelopeComponentTagV1,
   schema_version:u16)

ParentVersionBindingV1 =
  (parent_slot:ParentSlotV1, schema_version:u16)

EnvelopeVersionBindingsV1 =
  (envelope_protocol_version:u16=1,
   envelope_schema_version:u16=1,
   profile_version:u16=1,
   component_versions:vector<ComponentVersionV1>,
   parent_versions:vector<ParentVersionBindingV1>,
   constructor:ConstructorTagV1).
```

`component_versions` has exactly eight entries, entry `i` has component tag
`i` in the displayed A4-O order and `schema_version=1`, with no name or free
tag on the wire. `parent_versions` is exactly the six A1
parent slots in tag order, with the schema version freshly read from each
bound parent; ordinary V1 requires every displayed version to be `1`. The
constructor comes only from the internal seven-constructor iteration.

Field 3 uses:

```text
OwnedHistoryAndEvaluationV1 =
  (history:EnvelopeHistoryRefV1,
   evaluation_binding:EvaluationBindingRefV1).
```

`history.full_identity.complete_through_head_commitment` equals
`evaluation_binding.full_identity.complete_head_commitment`; the profile
identity and target birth in the census agree with fields 1--4. The full
`BirthOpaqueSignatureBindingV1`, `U_b`, `Pub_<b(H)`, and source vector inside
that evaluation binding are the sole `Sigma(H,o)` and source authority.

An assembly seed contains no caller-selected result:

```text
CandidateAssemblySeedV1 =
  (profile:EnvelopeProfileRefV1,
   parents:EnvelopeParentBindingsV1,
   history_and_evaluation:OwnedHistoryAndEvaluationV1,
   anchor:OccurrenceId14V1,
   constructor:ConstructorTagV1,
   raw_ordinal:u32,
   raw_code:RawConstructorCodeV1,
   checked_built:CheckedBuiltRefV1).
```

The raw ordinal is its checked zero-based position in the complete structural
enumeration. Every repeated constructor, raw code, anchor, evaluation binding,
and built value agrees by full canonical bytes. The seed is an internal
operation input, not a Level-II object.

## 3. Exact constructor projections used by A1 fields 8--11

The action-neutral schema payload is the exact A3-O projection:

```text
CanonicalSchemaPayloadV1 ::=
  Formation=0(gamma:NormalizedContextV1,
              a:NormalizedTelescopeV1,
              a_raw:CheckedTelescopeV1)
| Abstraction=1(gamma:NormalizedContextV1,
                a:NormalizedTelescopeV1,
                b:NormalizedTelescopeV1,
                xi_raw:CheckedContextV1,
                cut:u32,
                b_raw:CheckedTelescopeV1)
| Aggregation=2(gamma:NormalizedContextV1,
                a:NormalizedTelescopeV1,
                b:NormalizedTelescopeV1,
                c_raw:CheckedTelescopeV1)
| Transport=3(delta:NormalizedContextV1,
              gamma:NormalizedContextV1,
              theta:NormalizedSubstitutionV1,
              a:NormalizedTelescopeV1,
              c_raw:CheckedTelescopeV1)
| Comparison=4(gamma:NormalizedContextV1,
               a:NormalizedTelescopeV1,
               b:NormalizedTelescopeV1,
               comparison:NormalizedComparisonValueV1,
               a_raw:CheckedTelescopeV1,
               b_raw:CheckedTelescopeV1)
| DemandCompiler=5(gamma:NormalizedContextV1,
                   grammar:NormalizedGrammarValueV1,
                   scheme:NormalizedSchemeValueV1,
                   activation:NormalizedSectionV1,
                   live:NormalizedLiveDemandValueV1,
                   activation_raw:CheckedSectionV1)
| DischargeTransformer=6(delta:NormalizedContextV1,
                         gamma:NormalizedContextV1,
                         theta:NormalizedSubstitutionV1,
                         grammar:NormalizedGrammarValueV1,
                         scheme:NormalizedSchemeValueV1,
                         activation:NormalizedSectionV1,
                         live:NormalizedLiveDemandValueV1,
                         discharge:NormalizedDischargeValueV1,
                         discharge_raw:CheckedDischargeV1).
```

Each tag and field is projected from the byte-identical
`BuiltConstructorValueV1`; no action result or quotient key occurs here.

Field 8 uses the checked owner binding:

```text
ConstructorSubjectBindingV1 =
  (constructor:ConstructorTagV1,
   checked_built:CheckedBuiltRefV1).
```

The constructor tag equals the referenced checked-built, built-sum, raw-code,
decoded-sum, build-evidence, and fields 9--11 tags. Because the reference's
full identity is the complete `CheckedBuiltV1`, this retains the full tagged
`Built_c` constructor subject without a digest-only shortcut.

Field 9 retains two separately tagged projections so the A3-O `Indices` and
`Inputs` roots remain independently addressable. Repeated values remain
repeated where A3-O requires them:

```text
ConstructorIndicesV1 ::=
  Formation=0(gamma:NormalizedContextV1)
| Abstraction=1(gamma:NormalizedContextV1,
                a:NormalizedTelescopeV1,
                gamma_a:NormalizedContextV1)
| Aggregation=2(gamma:NormalizedContextV1,
                a:NormalizedTelescopeV1,
                b:NormalizedTelescopeV1)
| Transport=3(delta:NormalizedContextV1,
              gamma:NormalizedContextV1)
| Comparison=4(gamma:NormalizedContextV1,
               a:NormalizedTelescopeV1,
               b:NormalizedTelescopeV1)
| DemandCompiler=5(gamma:NormalizedContextV1,
                   grammar:NormalizedGrammarValueV1,
                   scheme:NormalizedSchemeValueV1,
                   activation:NormalizedSectionV1)
| DischargeTransformer=6(delta:NormalizedContextV1,
                         gamma:NormalizedContextV1,
                         grammar:NormalizedGrammarValueV1,
                         scheme:NormalizedSchemeValueV1,
                         activation:NormalizedSectionV1,
                         live:NormalizedLiveDemandValueV1)

NominalInputsV1 ::=
  Formation=0(here:Unit)
| Abstraction=1(here:Unit,b:NormalizedTelescopeV1)
| Aggregation=2(family:NormalizedFamilyValueV1)
| Transport=3(theta:NormalizedSubstitutionV1,
              a:NormalizedTelescopeV1)
| Comparison=4(left:NormalizedTelescopeV1,
               right:NormalizedTelescopeV1)
| DemandCompiler=5(grammar:NormalizedGrammarValueV1,
                   scheme:NormalizedSchemeValueV1)
| DischargeTransformer=6(theta:NormalizedSubstitutionV1,
                         discharge:NormalizedDischargeValueV1)

IndexedInputsV1 =
  (indices:ConstructorIndicesV1,
   nominal_inputs:NominalInputsV1).
```

Both inner tags equal the constructor tag. This two-field record is the exact
field-9 wire: indices first, nominal inputs second.

Field 10 is:

```text
IndexedOutputV1 ::=
  Formation=0(NormalizedTelescopeV1)
| Abstraction=1(NormalizedFamilyValueV1)
| Aggregation=2(NormalizedTelescopeV1)
| Transport=3(NormalizedTelescopeV1)
| Comparison=4(NormalizedComparisonValueV1)
| DemandCompiler=5(NormalizedLiveDemandValueV1)
| DischargeTransformer=6(NormalizedDischargeValueV1).
```

The raw-realization projection is:

```text
RawRealizationValueV1 ::=
  Formation=0(CheckedTelescopeV1)
| Abstraction=1(xi_raw:CheckedContextV1,
                cut:u32,
                b_raw:CheckedTelescopeV1)
| Aggregation=2(CheckedTelescopeV1)
| Transport=3(CheckedTelescopeV1)
| Comparison=4(a_raw:CheckedTelescopeV1,
               b_raw:CheckedTelescopeV1)
| DemandCompiler=5(CheckedSectionV1)
| DischargeTransformer=6(CheckedDischargeV1).

RawInterfaceAndImplementationV1 =
  (raw_code:RawConstructorCodeV1,
   decoded:CheckedDecodedPayloadV1,
   raw_realization:RawRealizationValueV1).
```

The field-8 constructor subject is `ConstructorSubjectBindingV1`, whose full
checked-built identity contains the tagged `BuiltConstructorValueV1`; it is
not `CanonicalSchemaPayloadV1`. Fields 9--11 are the exact projections above
and intentionally repeat their corresponding built components. Every tag and
repeated value must be byte-identical.

## 4. Public fields, lineage, footprints, and exact history replay

```text
NormalizedPublicFieldV1 =
  (field_ordinal:u32,
   path:EnvelopeFieldPathV1,
   normalized_judgment:OpenJudgment)

FieldLineageEntryV1 =
  (raw_leaf_path:RawFieldPathV1,
   source:SourceLeafV1,
   derivation_edge:RawDependencyEdgeV1)

FieldLineageV1 =
  (field_ordinal:u32,
   field_path:EnvelopeFieldPathV1,
   entries:vector<FieldLineageEntryV1>)

OutputLineageEntryV1 =
  (output_ordinal:u32,
   field_path:EnvelopeFieldPathV1,
   raw_leaf_path:RawFieldPathV1,
   occurrence:OccurrenceId14V1)

ExactOccurrenceFootprintV1 =
  newtype vector<OccurrenceId14V1>

OlderPublicFootprintEntryV1 =
  (export_id:ExportIdV1,
   public_field_path:PublicFieldPathV1::DeclarationField)

ExactOlderPublicFootprintV1 =
  newtype vector<OlderPublicFootprintEntryV1>

PriorAndCurrentOutputFootprintsV1 =
  (prior_support:ExactOlderPublicFootprintV1,
   current_output:ExactOccurrenceFootprintV1).
```

`fields` is exactly `Fields_c(b)` and follows A3-O
`Indices ++ Inputs ++ Output ++ RawRealization` public-leaf preorder. A
mathematically repeated field at a different typed root/path is retained
again. Field ordinals and output ordinals are contiguous checked `u32`
positions. A public-field judgment is `TypeFormation` for a type leaf or
`HasType` for an image/term leaf; its subject is exactly the normalized value
at the retained path, with the internally derived expected type where needed.
The field and lineage vectors have equal length and byte-identical paths at
each ordinal. Each lineage vector is the complete backwards replay through the fixed
`Decode_c;Build_c` DAG. Repeated derivation edges remain repeated.

All three footprint vectors sort by full canonical identity bytes and reject
duplicates after their defining set projection. They are exactly:

```text
implementation = sort_unique(Cl_b({anchor} union Use_b(raw_code)))
prior_support   = Old_b(raw_code)
current_output  = sort_unique(Cl_b(OutputUse_c(raw_code))).
```

The output-lineage paths are exactly the seven A3-O source positions. The
anchor belongs to `implementation` and `current_output`; `current_output` is
nonempty. No formal action dependency exists in a candidate footprint.

Field 7 retains both endpoints and the exact R3-qualified replay receipts:

```text
CheckedSignatureReplayV1 =
  (call_role:R3::KernelCallRoleV1,
   raw_wire:UncheckedSignature,
   normalized_wire:UncheckedSignature,
   normalized_digest:Digest,
   declaration_count:u64,
   receipt:R3::KernelSuccessReceiptV1)

CheckedContextReplayV1 =
  (call_role:R3::KernelCallRoleV1,
   signature_digest:Digest,
   raw_context:RawContextV1,
   normalized_context:DependentContext,
   receipt:R3::KernelSuccessReceiptV1)

HistoricalBirthBoundaryReplayV1 =
  (birth_event_id:GenerativeHistoryEventIdV1,
   birth_event_ordinal:u64,
   normalized_birth_extension:UncheckedSignature,
   predecessor:CheckedSignatureReplayV1,
   full_successor:CheckedSignatureReplayV1,
   exact_stage_evidence_digest:Digest)

CurrentSourceHistoryReplayV1 =
  (source:SourceLeafV1,
   occurrence:OccurrenceId14V1,
   birth_boundary:HistoricalBirthBoundaryReplayV1,
   strict_prefix:CheckedSignatureReplayV1,
   local_context:DependentContext,
   strict_prefix_context:CheckedContextReplayV1,
   opaque_sigma_context:CheckedContextReplayV1)

OlderSourceHistoryReplayV1 =
  (source:SourceLeafV1,
   export:OlderPublicExportV1,
   birth_boundary:HistoricalBirthBoundaryReplayV1,
   strict_prefix:CheckedSignatureReplayV1,
   public_type_formation:KernelSuccessReplayV1,
   public_global_typing:KernelSuccessReplayV1)

SourceHistoryReplayV1 ::=
  Current=0(CurrentSourceHistoryReplayV1)
| OlderPublic=1(OlderSourceHistoryReplayV1)

ReplayBoundariesAndContextsV1 =
  (target_birth_boundary:HistoricalBirthBoundaryReplayV1,
   opaque_signature:BirthOpaqueSignatureBindingV1,
   selected_source_replays:vector<SourceHistoryReplayV1>).
```

`selected_source_replays` contains exactly one entry for every unique current
or older source selected by fields 5--6, including the anchor through the
implementation footprint. It follows the enclosing `BirthSourceCensusV1`
source order, not footprint sort order; no unselected census member occurs.
Every retained source ordinal selects the byte-identical source identity.
Predecessor, birth extension, successor, strict prefix, opaque signature,
stage evidence, and context values are freshly projected/replayed from the
privately owned `H`. For a current occurrence, the local context and the two
replay outputs have byte-identical normalized wires. An older export retains
public type/global replay and has no body or local-variable projection.

The complete slot-4 success payload is:

```text
PublicLeavesAndLineageV1 =
  (seed:CandidateAssemblySeedV1,
   fields:vector<NormalizedPublicFieldV1>,
   field_lineage:vector<FieldLineageV1>,
   output_lineage:vector<OutputLineageEntryV1>,
   implementation:ExactOccurrenceFootprintV1,
   prior_and_current:PriorAndCurrentOutputFootprintsV1,
   replay_boundaries:ReplayBoundariesAndContextsV1,
   obligations:DerivedObligationsV1,
   subject_root_projection:SubjectRootProjectionV1).
```

`DerivedObligationsV1` and `SubjectRootProjectionV1` are defined below. Their
types are topologically earlier than this runtime record in the final DAG even
though their normative algorithms are presented later for readability.

## 5. Derived obligations and the pre-support theorem projection

### 5.1 Exact obligation grammar

The eight A3-O realization groups retain their displayed order:

```text
ObligationGroupTagV1 ::=
  ParentReplay=0
| RawMembershipAndDecode=1
| CarrierFormationAndCompatibility=2
| ParticularSubstitutionsAndLifts=3
| IndependentRawOutputCheck=4
| NormalizationSupportAndDisposition=5
| PrincipalCurrentOrigin=6
| TheoremEndpointConstruction=7.
```

An obligation is a typed proposition or checked construction descriptor, never
free prose:

```text
ObligationSourceRootV1 ::=
  AssemblySeed=0
| CheckedDecodedPayload=1
| BuiltConstructorValue=2
| DecodeEvidence=3
| BuildPremiseEvidence=4
| DerivedVersionBindings=5

ObligationSourcePathV1 =
  (root:ObligationSourceRootV1,
   steps:vector<TypedPathStepV1>,
   result_type:TypeIdV1)

ObligationProjectionRelationV1 ::=
  ExactBytes=0
| RawMembership=1
| KernelJudgment=2
| SourceResolution=3
| ParticularSubstitution=4
| TelescopeComparison=5
| CarrierFormation=6
| CarrierCompatibility=7
| LengthConstraint=8(LengthRelationV1)
| Nonempty=9
| RawOutputAgreement=10

ObligationProjectionPredicateV1 =
  (relation:ObligationProjectionRelationV1,
   sources:vector<ObligationSourcePathV1>)

OriginSingletonObligationV1 =
  (origin_path:OriginPathV1,
   principal_leaf_path:RawFieldPathV1,
   expected_anchor:OccurrenceId14V1)

SubjectWellFormedObligationV1 =
  (subject_slot:u16, expected_root_kind:SubjectRootKindV1)

DerivedObligationPredicateV1 ::=
  Projection=0(ObligationProjectionPredicateV1)
| OriginSingleton=1(OriginSingletonObligationV1)
| SupportProjectionRequirement=2(SupportProjectionRequirementV1)
| SubjectWellFormed=3(SubjectWellFormedObligationV1)
| ProfileRule=4(OrdinaryProfileRuleV1)
| ConstructionRequirement=5(DefinitionRulePathV1).

DerivedObligationV1 =
  (obligation_ordinal:u32,
   group:ObligationGroupTagV1,
   group_local_ordinal:u32,
   rule_path:DefinitionRulePathV1,
   support_path:FullLocalPathV1,
   expected_judgment:ExpectedJudgmentV1,
   predicate:DerivedObligationPredicateV1)

DerivedObligationGroupV1 =
  (group:ObligationGroupTagV1,
   entries:vector<DerivedObligationV1>)

DerivedObligationsV1 =
  (constructor:ConstructorTagV1,
   groups:vector<DerivedObligationGroupV1>).
```

The six source roots are interpreted against, respectively, the assembly
seed, `checked_built.full_identity.decoded`,
`checked_built.full_identity.built`, its decode evidence, and its build-premise
evidence, followed by the uniquely reconstructed `EnvelopeVersionBindingsV1`.
A source path stores only a root, typed steps, and final nonzero
`TypeIdV1`; it does not copy the reached value or a success receipt into field
12.

The compiler is finite and exact. `groups` has length eight, tag `i` at
position `i`, and no omitted empty group. It emits these rows:

`DerivedObligationsV1.constructor` is byte-identical to the assembly seed,
checked-built, built-payload, and every selected schema tag; it is never
inferred independently from a caller-supplied group.

| Group | Exact entry sequence | Count |
| ---: | --- | ---: |
| 0 | `Projection(ExactBytes)` for derived versions, profile ref, each of the six parent refs, history ref, evaluation ref, and constructor, in that order | 11 |
| 1 | raw-code membership/equality; every decode `LengthCheck`; every `SourceUse` leaf; every source-resolution call; then each decode kernel replay's raw/normalized judgment pair | `1 + lengths + source leaves + calls + kernel calls` |
| 2 | every maximal normalized carrier/index value in the selected `BuiltConstructorValueV1`, followed by each dependent compatibility edge, all in the frozen A3 public-carrier preorder | `carrier values + compatibility edges` |
| 3 | decode substitution replays, build-premise substitution replays, then nested lift/substitution descriptors in build-derived structural preorder | their exact concatenated lengths |
| 4 | build kernel raw/normalized judgment pairs, build telescope comparisons, then the selected constructor's raw-output agreement components in build-derived preorder | their exact concatenated lengths |
| 5 | `ConstructionRequirement` for slots `5[0]`, `6[0]`, `6[1]`, `6[2]`, `7[0]`, then the normalization, renaming, and reindexing `SupportProjectionRequirement` descriptors | 8 |
| 6 | the one `OriginSingleton` built from the selected A3 origin path and principal leaf | 1 |
| 7 | one `SubjectWellFormed` for each subject slot in ascending order | 8 for tags 0,1,2,4,5; 11 for tags 3,6 |

In group 0, “derived versions” is the empty-step `DerivedVersionBindings`
projection. It is a read-only compiler source, not another stored seed field.
All other source paths are
the unique schema-DAG paths to the named earlier values. A kernel row points to
the replay's two judgment fields, never its role or receipt. A source-resolution
row points to request and semantic success fields, never nested replay receipt
fields. Group 2's carrier walk and group 4's raw-output components are exactly
the selected constructor row of A3-O sections 3 and 4.1; no receipt, resource
account, or later evidence value is reached.

`Projection` relation tags are, in table order: `ExactBytes` in group 0;
`RawMembership`, `LengthConstraint(check.relation)`, `ExactBytes`, `SourceResolution`, and
`KernelJudgment` for group 1's five segments; `CarrierFormation` then
`CarrierCompatibility` in group 2; `ParticularSubstitution` in group 3; and
`KernelJudgment`, `TelescopeComparison`, then `RawOutputAgreement` in group 4.
Within an entry, `sources` follows left-before-right and raw-before-normalized
display order and is never sorted or deduplicated.

`rule_path` is `PublicLeavesAndLineage[0]` for group 0; `Build[0]` for groups
2, 4, and 6; `Decode[1]` for group 1;
the originating `Decode[1]` or `Build[0]` path for each group-3 row; the exact
required rule itself for group 5; and `SubjectConstruction[0]` for group 7.
Group 5's exact eight-path vector is
`[Normalization[0], SupportAndOwnerResolution[0],
SupportAndOwnerResolution[1], SupportAndOwnerResolution[2],
FieldDisposition[0], Normalization[0],
SupportAndOwnerResolution[2], SupportAndOwnerResolution[2]]`.
The final three entries are respectively normalization, admissible-renaming,
and reindexing preservation. Each notation expands to the one-step
`DefinitionRulePathV1` fixed in section 11. `support_path` is exactly
`Obligations/groups[i]/entries[j]/predicate` under the tagged
`FullLocalPathV1`, using record/vector/sum steps with the displayed ordinals.
Global and group-local ordinals are contiguous.
Every group-5 entry has
`ExpectedJudgmentV1::ConstructionRule(rule_path)`;
the path stored inside a `ConstructionRequirement` or
`SupportProjectionRequirement` is byte-identical to that entry's `rule_path`.

The expected judgment is copied from an `OpenJudgment` source when present;
otherwise it is derived by the reached static type: type/carrier values use
`TypeFormation` or their exact `CarrierSlot`, term images use `HasType` with
the companion projected normalized type, substitutions use the complete
codomain `ContextPrefix`, occurrence rows use `StructuralOccurrence`, and a
pure record/path descriptor uses `StructuralRecord` with its exact nonzero
type, while a required operation uses `ConstructionRule` with its exact
definition path. This
type-directed function has no caller branch. Every term-bearing predicate is
traversed by support. Receipts produced while checking these descriptors occur
only in `ObligationConstructionTranscriptV1`; none is embedded in a
`DerivedObligationPredicateV1`.

This uses the A3-C1 appended tags 8 and 9. During support traversal every
obligation record emits one leading `ObligationPremise` slot with its complete
expected judgment, followed by the slots of its selected term-bearing sources.

The `SupportProjectionRequirement` payload is the receipt-free seed contract
defined in section 5.4, not proposition syntax, a full support-projection
subject, or a source bundle. This is the required acyclic cut:
`DerivedObligationsV1` may occur in a subject source bundle, while no obligation
points back to that bundle. The final schema DAG places the closed source-path,
formal-action-path, support-kind, and requirement descriptors,
as well as `ExpectedJudgmentV1` and `SubjectRootKindV1`, before
`DerivedObligationPredicateV1`. Forward textual presentation does not permit a
forward `TypeIdV1`.

### 5.2 Descriptor-only subject literals

Subject syntax describes propositions relative to the checked candidate
assembly input. It does not copy a candidate payload or support object into a
literal. The exact descriptor types are:

```text
PrincipalRootLineageV1 =
  (origin_path:OriginPathV1,
   raw_field_path:RawFieldPathV1,
   principal_leaf:SourceLeafV1,
   pre_expansion_occurrence:OccurrenceId14V1,
   exact_anchor:OccurrenceId14V1)

FootprintProjectionV1 =
  (implementation:ExactOccurrenceFootprintV1,
   current_output:ExactOccurrenceFootprintV1,
   prior_support:ExactOlderPublicFootprintV1,
   output_lineage:vector<OutputLineageEntryV1>)

CandidateMatchPremiseV1 =
  (profile:EnvelopeProfileRefV1,
   history:EnvelopeHistoryRefV1,
   evaluation:EvaluationBindingRefV1,
   anchor:OccurrenceId14V1,
   constructor:ConstructorTagV1,
   raw_ordinal:u32,
   raw_code:RawConstructorCodeV1)

SelectorSubjectInputsV1 =
  (match_premise:CandidateMatchPremiseV1,
   current_domain:vector<OccurrenceId14V1>,
   origin_path:OriginPathV1,
   anchor:OccurrenceId14V1)

SubjectActionConstantsV1 =
  (substitution_identity:FormalActionSyntaxV1,
   context_square_identity:FormalActionSyntaxV1)

EnvelopeObjectDescriptorV1 =
  (match_premise:CandidateMatchPremiseV1,
   evaluation_binding:EvaluationBindingRefV1,
   constructor:ConstructorTagV1,
   canonical_payload:CanonicalSchemaPayloadV1,
   principal_lineage:PrincipalRootLineageV1,
   footprints:FootprintProjectionV1)

SubjectProjectionSourceBundleV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   schema_payload:CanonicalSchemaPayloadV1,
   public_fields:vector<NormalizedPublicFieldV1>,
   principal_lineage:PrincipalRootLineageV1,
   field_lineages:vector<FieldLineageV1>,
   footprints:FootprintProjectionV1,
   obligations:DerivedObligationsV1,
   selector_inputs:SelectorSubjectInputsV1,
   action_constants:SubjectActionConstantsV1,
   source_candidate:EnvelopeObjectDescriptorV1)

SubjectProjectionSourceRootV1 ::=
  EvaluationBinding=0
| SchemaPayload=1
| PublicFields=2
| PrincipalLineage=3
| FieldLineages=4
| Footprints=5
| Obligations=6
| SelectorInputs=7
| ActionConstants=8
| SourceCandidate=9

SubjectProjectionSourcePathV1 =
  (root:SubjectProjectionSourceRootV1,
   steps:vector<TypedPathStepV1>,
   result_type:TypeIdV1)

FormalActionComponentRefV1 ::=
  SourceProjection=0(SubjectProjectionSourcePathV1)
| PrimitiveArgument=1(primitive_ordinal:u32)
| EarlierDerived=2(derived_component_ordinal:u32)

FormalActionDerivedComponentV1 ::=
  IteratedLift=0(primitive_ordinal:u32,
                 target:SubjectProjectionSourcePathV1,
                 lift_depth:u16)
| OrderedComposite=1(left:FormalActionComponentRefV1,
                     right:FormalActionComponentRefV1)
| ReconstructedEpsilon=2
| ReindexedPayloadComponent=3(source:SubjectProjectionSourcePathV1)
| EpsilonCongruenceEndpoint=4

FormalActionProjectionRootV1 ::=
  NormalizedSyntax=0
| PrimitiveArgument=1(primitive_ordinal:u32)
| DerivedComponent=2(component_ordinal:u32,
                     component:FormalActionDerivedComponentV1)

FormalActionProjectionPathV1 =
  (action_binder:u32,
   root:FormalActionProjectionRootV1,
   steps:vector<TypedPathStepV1>,
   result_type:TypeIdV1)

SupportFunctionProjectionV1 ::=
  Full=0 | Current=1 | Older=2 | OutputRoot=3

CoreProjectionLiteralV1 ::=
  CorePayloadProjection=0(path:SubjectProjectionSourcePathV1)
| CarrierPayloadProjection=1(path:SubjectProjectionSourcePathV1,
                             carrier_index:CarrierIndexValueV1,
                             field_ordinal:u32)
| ExactOccurrenceProjection=2(path:SubjectProjectionSourcePathV1)
| NormalizedOccurrenceProjection=3(path:SubjectProjectionSourcePathV1)
| FormalActionArgumentProjection=4(path:FormalActionProjectionPathV1)
| PureSupportFunctionProjection=5(support_binder:u32,
                                   projection:SupportFunctionProjectionV1,
                                   result_type:TypeIdV1).
```

Within every source bundle, the source-candidate descriptor is canonical:

```text
source_candidate.match_premise       = selector_inputs.match_premise
source_candidate.evaluation_binding  = evaluation_binding
source_candidate.constructor         = schema_payload.variant_tag
source_candidate.canonical_payload   = schema_payload
source_candidate.principal_lineage   = principal_lineage
source_candidate.footprints          = footprints
obligations.constructor              = source_candidate.constructor
selector_inputs.match_premise.constructor
                                      = source_candidate.constructor.
```

All equations are full bytes; the payload variant is converted to the
byte-identical `ConstructorTagV1` by the fixed tag correspondence.

The principal `origin_path` is the selected constructor's frozen A3 tag.
`raw_field_path` is byte-identical to
`BuildPremiseEvidenceV1.anchor_path` and resolves that origin tag;
`principal_leaf` is byte-identical to the source leaf at that path and to
`BuildPremiseEvidenceV1.principal_leaf`. It must select
`Current(pre_expansion_occurrence)` at the same census ordinal. The
`pre_expansion_occurrence` is the exact source identity before `Cl_b`, while
`exact_anchor` is byte-identical to it, the checked-build anchor, the assembly
seed anchor, and A1 field 4. `FieldLineageV1` remains the distinct exhaustive
lineage for all public leaves; it is not duplicated inside this record.

`CandidateMatchPremiseV1` is a proposition descriptor, not a successful
envelope or receipt. Relative to the enclosing subject's source-candidate
symbol `E`, it denotes exactly
`Eval^kappa_c(H,o,r)=Match(E)` with every parameter projected from the
assembly seed and repeated here by full bytes. It deliberately omits the
checked-built reference, later evidence, and `E` itself.
`SelectorSubjectInputsV1.current_domain` is byte-identical to
`match_premise.evaluation.full_identity.current_sources`; its `origin_path`
and `anchor` are byte-identical to the principal-lineage fields. The two action constants
are respectively `(SubstitutionAction,[Identity])` and
`(ContextSquareAction,[Identity])`, with no alternate postfix spelling.

Concrete tags 0--3 are interpreted only against the enclosing
`SubjectProjectionSourceBundleV1`. The root fixes an exact schema-DAG type;
the typed steps must terminate at `result_type`, which R2e fills with the
actual nonzero local `TypeIdV1`. The projected value is the reached subvalue;
it is not copied into the literal and no opaque bytes or digest authenticate
it. A carrier projection additionally validates the complete carrier index.
Exact and normalized occurrence projections must end at their named exact
types.

A formal-action path is rooted at its earlier formal-action binder, never at a
raw/envelope/support path. `NormalizedSyntax` exposes the normalized formal
action; `PrimitiveArgument(i)` exposes checked primitive `i`; and
`DerivedComponent(i,d)` exposes only the exact virtual component `d` at
derived-component ordinal `i`. Any `EarlierDerived(j)` in `d` has `j<i`.
Substitution actions have primitive 0=`theta`. Formation and Comparison have
an empty derived vector; Abstraction and Aggregation have exactly
`theta^A`; DemandCompiler has exactly `theta^E`. Each is one
`IteratedLift(0,target,full binder count)` component, with the target path and
lift depth fixed by the selected payload. The full binder count must admit the
exact checked `u16` binder-depth conversion before this component is allocated;
an out-of-profile raw value is `False`, while failure to convert an already
checked internal value is `Abort`. Square actions have primitives
0=`rho`, 1=`sigma`, and 2=`theta_prime`. Transport's derived vector is exactly
reconstructed `epsilon`, `theta.rho`, `sigma.theta_prime`, then the
`epsilon`-congruence endpoint. DischargeTransformer inserts between the two
composites and final endpoint the reindexed `G`, `S`, `a`, `L`, and every
retained contract, in that exact payload/contract order. A component absent
from that constructor/action row is invalid. The root therefore selects the virtual
record before `steps` selects a typed subvalue; generic steps cannot invent a
new lift, composite, reindexed component, or endpoint.

For both square rows, the two composite descriptors are exactly
`OrderedComposite(SourceProjection(theta),PrimitiveArgument(0))` and
`OrderedComposite(PrimitiveArgument(1),PrimitiveArgument(2))`. Each reindexed
component stores the byte-identical source-bundle path to its unreindexed
payload component. `ReconstructedEpsilon` and
`EpsilonCongruenceEndpoint` occur exactly once at the positions above. A pure
support projection is authenticated by its earlier support-projection binder.
Neither embeds a checked action image, computed support value, final key, or
unchecked caller object. `QuotientField` is invalid in every R2c subject.

### 5.3 Binder, slot, and root-projection closure

```text
SubjectCarrierIndexV1 ::=
  PublicContext=0(gamma:u32)
| PublicInterface=1(gamma:u32)
| InterfaceFamily=2(gamma:u32,a:u32)
| Substitution=3(delta:u32,gamma:u32)
| ComparisonWitness=4(gamma:u32,a:u32,b:u32)
| SealedPublicGrammar=5(gamma:u32)
| DemandScheme=6(gamma:u32,grammar:u32)
| LiveDemand=7(gamma:u32,grammar:u32,scheme:u32,activation:u32)
| Discharge=8(gamma:u32,grammar:u32,scheme:u32,
               activation:u32,live:u32)

SubjectIndexRefV1 ::=
  SourceProjection=0(SubjectProjectionSourcePathV1)
| EarlierBinder=1(binder_ordinal:u32)

SubjectActionBaseV1 ::=
  SourceCandidate=0
| PriorActionResult=1(action_binder:u32)

FormalActionCardinalityV1 ::=
  Identity=0
| ExactlyOnePrimitive=1
| ArbitraryNormalized=2

SubjectBinderSortV1 ::=
  CorePayload=0(constructor:ConstructorTagV1,
                evaluation_binding:EvaluationBindingRefV1)
| DependentContext=1
| Telescope=2(base_context:SubjectIndexRefV1)
| Section=3(base_context:SubjectIndexRefV1,
            telescope:SubjectIndexRefV1)
| Substitution=4(domain:SubjectIndexRefV1,
                 codomain:SubjectIndexRefV1)
| Carrier=5(index:SubjectCarrierIndexV1)
| Occurrence=6
| HistoryRenaming=7
| FormalAction=8(action_kind:ActionKindV1,
                  cardinality:FormalActionCardinalityV1,
                  base:SubjectActionBaseV1)
| SupportProjection=9
| PresentedEnvelope=10(constructor:ConstructorTagV1,
                       evaluation_binding:EvaluationBindingRefV1)
| NonemptyPublicInterface=11(context:SubjectIndexRefV1)
| SchemeActivation=12(context:SubjectIndexRefV1,
                      grammar_binder:u32,
                      scheme_binder:u32)
| DischargeAtSpecialization=13(context:SubjectIndexRefV1,
                               grammar_binder:u32,
                               scheme_binder:u32,
                               activation_binder:u32)

SubjectBinderV1 =
  (sort:SubjectBinderSortV1)

SubjectRootKindV1 ::=
  Identity=0
| Composition=1
| SelectorExistenceAndAgreement=2
| SelectorNormalization=3
| SelectorPresentation=4
| SelectorRenaming=5
| SelectorAction=6
| Naturality=7
| ExtraCoherenceFirst=8
| ExtraCoherenceSecond=9
| ExtraCoherenceThird=10

SubjectEndpointProgramV1 =
  (subject_kind:SubjectRootKindV1,
   binders:vector<SubjectBinderV1>,
   value_nodes:vector<SubjectNodeV1>,
   value_node_types:vector<TypeIdV1>)

SubjectRootProjectionV1 =
  (constructor:ConstructorTagV1,
   source_bundle:SubjectProjectionSourceBundleV1,
   subjects:vector<SubjectEndpointProgramV1>).
```

#### 5.3.0a Narrow subject-type closure corrigendum

The R2d item-4 audit found places where the original R2c prose fixed a
semantic operation but left its exact source-side type target implicit behind
`TypeIdV1` or an ellipsis. This corrigendum closes those targets and appends one
dependent-discharge proposition opcode. It changes no binder, slot, path
occurrence, clause count, node count, or theorem statement.

First, a history-renaming binder denotes a formal variable, not an encoded
bijection or a proof object:

```text
SubjectHistoryRenamingRefV1 = newtype u32
SubjectPropositionSortV1    = newtype Unit
```

For `Binder(h)` at global binder ordinal `b`, the synthesized value is exactly
`SubjectHistoryRenamingRefV1(checked_u32(b))`.  Validation requires `b` to
select the byte-identical earlier `HistoryRenaming` binder after checked
relocation.  This reference has no literal or projection constructor and may
be consumed only as the second operand of `RenameHistory`.  It cannot be
composed, compared, owned, stored as an envelope field, or executed as a
renaming map.  Semantically the enclosing theorem binder still ranges over all
admissible bijections of exact identifiers in the same history evidence, as
fixed by A3; the reference is only the syntax-level handle for that universally
quantified variable.  A later executable renaming witness, if required, is a
different type and cannot be smuggled into this handle.

`SubjectPropositionSortV1` is likewise a type-checker marker, not a proposition
value or proof.  Proposition content is the node expression itself.  It is the
result type of node tags `9..16`; it is never emitted by a literal or project
path.  The two newtypes are distinct local schema nodes even though their
underlying scalar codecs are inherited.

Second, the previously abbreviated envelope paths are exact.  For every
constructor `c`:

```text
Out_c = TypedProjectionPathV1::EnvelopeField(
  EnvelopeFieldPathV1(c,Output,[SumPayload(c)])).
```

The only emitted `RawOut_c` paths are:

| Constructor | Exact `EnvelopeFieldPathV1` |
| --- | --- |
| Transport | `(Transport,RawRealization,[SumPayload(3)])` |
| DischargeTransformer | `(DischargeTransformer,RawRealization,[SumPayload(6)])` |

The unused Formation, Abstraction, Aggregation, Comparison, and
DemandCompiler `RawOut_c` cases are invalid in the primary subject compiler;
no generic descent may invent one.  On a validated descriptor the Transport
path reaches its complete `CheckedTelescopeV1` realization and the
DischargeTransformer path reaches its complete `CheckedDischargeV1`
realization.  Their following `Normalize` nodes reach
`NormalizedTelescopeV1` and `NormalizedDischargeValueV1`, respectively.

Third, the exact receipt-free result schemas for the two virtual square
components are neutral normalized subject leaves owned by this corrigendum:

```text
SubjectNormalizedSubstitutionEqualityV1 =
  (left:NormalizedSubstitutionV1,
   right:NormalizedSubstitutionV1)

SubjectNormalizedTelescopeEqualityV1 =
  (base_context:NormalizedContextV1,
   left:NormalizedTelescopeV1,
   right:NormalizedTelescopeV1)

SubjectIndexedDischargeEndpointV1 =
  (index:CarrierIndexValueV1,
   discharge:NormalizedDischargeValueV1)

SubjectCarrierIndexEqualityV1 =
  (left:CarrierIndexValueV1,
   right:CarrierIndexValueV1)

SubjectHeterogeneousDischargeEqualityV1 =
  (left:SubjectIndexedDischargeEndpointV1,
   right:SubjectIndexedDischargeEndpointV1,
   index_equality:SubjectCarrierIndexEqualityV1)

SubjectTransportSquareEndpointV1 =
  (epsilon:SubjectNormalizedSubstitutionEqualityV1,
   reindexed_implementation:NormalizedTelescopeV1,
   nested_left:NormalizedTelescopeV1,
   composite_left:NormalizedTelescopeV1,
   composite_right:NormalizedTelescopeV1,
   nested_right:NormalizedTelescopeV1,
   implementation_agreement:SubjectNormalizedTelescopeEqualityV1,
   left_association:SubjectNormalizedTelescopeEqualityV1,
   epsilon_congruence:SubjectNormalizedTelescopeEqualityV1,
   right_association:SubjectNormalizedTelescopeEqualityV1,
   naturality:SubjectNormalizedTelescopeEqualityV1)

SubjectDischargeSquareEndpointV1 =
  (epsilon:SubjectNormalizedSubstitutionEqualityV1,
   reindexed_implementation:SubjectIndexedDischargeEndpointV1,
   nested_left:SubjectIndexedDischargeEndpointV1,
   composite_left:SubjectIndexedDischargeEndpointV1,
   composite_right:SubjectIndexedDischargeEndpointV1,
   nested_right:SubjectIndexedDischargeEndpointV1,
   implementation_agreement:SubjectHeterogeneousDischargeEqualityV1,
   left_association:SubjectHeterogeneousDischargeEqualityV1,
   epsilon_congruence:SubjectHeterogeneousDischargeEqualityV1,
   right_association:SubjectHeterogeneousDischargeEqualityV1,
   naturality:SubjectHeterogeneousDischargeEqualityV1)

ReconstructedEpsilon
  -> SubjectNormalizedSubstitutionEqualityV1

EpsilonCongruenceEndpoint in Transport
  -> SubjectTransportSquareEndpointV1

EpsilonCongruenceEndpoint in DischargeTransformer
  -> SubjectDischargeSquareEndpointV1.
```

The five endpoint positions are, in order, reindexed implementation, nested
action on the left, action by the left composite, action by the right composite,
and nested action on the right. The following four fields merely describe the
implementation-agreement, left-association, epsilon-congruence, and right-
association propositions between adjacent positions. They retain no equality
proof. The final `naturality` field describes the end-to-end proposition from
the first position to the fifth. A discharge endpoint retains its complete
dependent `G/S/a/L` index,
including the canonical contract set, so a body cannot be detached from its
target. `SubjectHeterogeneousDischargeEqualityV1` is the formal dependent
proposition that its two indices are equal and, under that equality, the two
discharge bodies are equal. Its `index_equality` repeats exactly the two endpoint
indices. It specializes to A2-O's homogeneous discharge equality when those
indices are byte-identical; otherwise it remains an unproved heterogeneous
subject and may never be evaluated as `ApproxOrd` on mismatched indices. These
types depend only on the frozen A2-O normalized carriers and A3-O square-subject
contract. They contain no R2d quotient root, subject program, evidence, or
receipt and therefore preserve the explicit upstream order
`A3-O base -> R2a+C2 -> R2b -> A3-C1 -> R2c+C2`. R2e must assign their exact numerical
IDs; R2c does not guess them.

R2c consumes the append-only logical extension owned by R2a-C2:

```text
SubjectNodeV1 +=
  DependentDischargeEquality=16(left_node:u32,right_node:u32).
```

Both operands must be normalized discharge expressions whose typing derivation
reconstructs an exact `SubjectIndexedDischargeEndpointV1`. The node denotes the
corresponding `SubjectHeterogeneousDischargeEqualityV1`, with
`index_equality.left/right` copied from those two derived endpoint indices, and
has result sort `SubjectPropositionSortV1`. It asserts no index or body equality.
It is invalid for non-discharge operands. When the indices are byte-identical it
specializes to homogeneous `ApproxOrd`; a validator may not demand that
specialization in order to admit the heterogeneous subject.

Finally, endpoint `value_node_types` uses the exact reached leaf schema for each
value node, including `SubjectHistoryRenamingRefV1` for the direct formal
renaming-binder node and the three square leaves above for their virtual action
components. Proposition-node results in the full `SubjectSyntaxV1` are inferred
as `SubjectPropositionSortV1`; they are not stored in the endpoint value-type
vector. Zero, a copied unrelated type ID, or an opaque byte schema is invalid.
This is a subject-typing corrigendum: it changes only the exact
DischargeTransformer clause opcode from homogeneous `ApproxOrd` to the R2a-C2
tag while preserving every binder/value expression, slot, clause/node count,
strict-postorder position, duplicate occurrence, relocation rule, and field-16
copy theorem below.

Every `EarlierBinder`, carrier-index ordinal, action-base ordinal, and direct
binder ordinal is strictly smaller than the containing binder's global
position and has the required earlier sort. A `SourceProjection` is decoded
against the enclosing source bundle and must end at the required index type.
`PriorActionResult(q1)` is legal only in a later action binder in the same slot
and denotes the exact reindexed payload after `q1`; it is what types
composition's second action. `SubjectNodeV1::Binder` uses the same global
position. Each endpoint program admits only node tags 0--8; all operands point
to earlier global value-node ordinals. `value_node_types` has the exact
value-node length and records the reached nonzero schema type. Binder
declarations themselves are not support dependencies.

`NonemptyPublicInterface` carries its positive-length premise.
`SchemeActivation` is indexed by the earlier grammar/scheme binders.
`DischargeAtSpecialization` deterministically derives `M=Spec(T,b)`, checks its
demanded telescope nonempty, and binds a discharge at exactly that
specialization; `M` is not an independent caller binder.

The empty `SourceCandidate` literal path and every `PresentedEnvelope` binder
have the same nonzero `EnvelopeObjectDescriptorV1` TypeId. `Normalize`,
`Reindex`, `Selector`, and `ApproxEnvelope` consume that descriptor type; they
never consume `CanonicalSchemaPayloadV1` as if it were an envelope object.
The descriptor is an ambient handle, not a standalone quotient identity. Its
`match_premise`, evaluation binding, constructor, payload, lineage, and
footprints are byte-identical to the corresponding fields of the enclosing
source bundle. In that bundle, `SourceCandidate` denotes the unique envelope
obtained by running the closed deterministic R2c stages from the complete
bundle; `PresentedEnvelope` denotes another value of the same full semantic
type. Thus two byte-distinct candidate assemblies cannot alias merely because
their payload and footprints agree.

For Formation, Abstraction, Aggregation, Comparison, and DemandCompiler,
subject slots are exactly tags `0..7`. For Transport and
DischargeTransformer they are exactly tags `0..10`; slots 8--10 are the three
ordered `T0--T3` or `D0--D3` edges. No empty placeholder slots are encoded for
the five single-base schemas.

`subjects[i].subject_kind` has tag `i`. The following compiler, rather than an
implementation-selected expression DAG, fixes every program.

#### 5.3.1 Closed endpoint operations and common symbols

For constructor `c`, let `a_c` be `SubstitutionAction` for tags 0, 1, 2, 4,
and 5, and `ContextSquareAction` for tags 3 and 6. The compiler uses these
unique source literals:

```text
E      = CorePayloadProjection(SourceCandidate,[])
MatchE = CorePayloadProjection(SelectorInputs.match_premise)
U      = CorePayloadProjection(SelectorInputs.current_domain)
o      = ExactOccurrenceProjection(SelectorInputs.anchor)
id_c   = CorePayloadProjection(ActionConstants.substitution_identity)
         or CorePayloadProjection(ActionConstants.context_square_identity)
```

`Out_c(X)` is `Project(X,EnvelopeField(Output,...))` to the complete selected
constructor output. `RawOut_c(X)` is the corresponding selected A3 raw-output
position: `A_raw`, `B_raw`, `C_raw`, `C_raw`, not used for Comparison,
`a_raw`, or `d_raw`. `Arg(q,root,p)` is
`FormalActionArgumentProjection(FormalActionProjectionPath(q,root,p,result_type))`,
where `root` includes the exact primitive ordinal or
`(derived-component ordinal,component descriptor)` payload fixed above.
Every ellipsis here means
the unique typed `RecordField`/`SumPayload` path in the frozen R2b constructor
record, not a stored wildcard.

`SubjectNodeV1::Compose` is type-directed and has exactly these admitted input
and output triples:

| Left, right | Result | Meaning |
| --- | --- | --- |
| context, telescope | context | context extension |
| telescope, telescope | interface family | `Fam` |
| telescope, telescope | public interface | dependent concatenation |
| telescope, telescope | comparison witness | canonical `q` |
| demand scheme, section | live demand | `Spec` |
| substitution, substitution | substitution | ordered action composition |
| context square, context square | context square | ordered square composition/pasting |

Input/output `TypeIdV1` values select exactly one row; every other triple is ill
typed. `Reindex` is likewise type-directed by its value and action types and
uses the A2-O capture-safe recipe. A formal action exposes `rho`, `sigma`,
`theta_prime`, `epsilon`, and every derived lift only through fixed typed
`Arg` paths. Thus identity, `Fam`, concatenation, comparison, `Spec`, action
composition, and square pasting have one encodable construction.

At the designated principal root, `Origin(Normalize(E),root_c)` has result type
`OccurrenceId14V1`: it is the singleton-eliminated occurrence licensed by the
byte-identical group-6 `OriginSingleton` obligation. It is not a set or an
unchecked selector. `U` remains the domain of `ExistsUnique`.

#### 5.3.2 Exact common-slot templates

Binder vectors and clauses are exactly:

| Slot | Exact binder vector | Exact clause |
| ---: | --- | --- |
| 0 | empty | `Reindex(E,id_c) approx_env E` |
| 1 | `q1:FormalAction(a_c,ArbitraryNormalized,SourceCandidate)`, `q2:FormalAction(a_c,ArbitraryNormalized,PriorActionResult(q1))` | `Reindex(E,Compose(q1,q2)) approx_env Reindex(Reindex(E,q1),q2)` |
| 2 | `q:Occurrence` | `MatchE -> exists! q in U. Origin(Normalize(E),root_c)=q and q=o and Selector(Normalize(E))=q` |
| 3 | empty | `Selector(Normalize(E))=Selector(E)` |
| 4 | `E':PresentedEnvelope(c,same evaluation binding)` | `E approx_env E' -> Selector(E)=Selector(E')` |
| 5 | `h:HistoryRenaming` | `Selector(RenameHistory(E,h))=RenameHistory(Selector(E),h)` |
| 6 | `q:FormalAction(a_c,ArbitraryNormalized,SourceCandidate)` | `Selector(Reindex(E,q))=Selector(E)` |
| 7 | `q:FormalAction(a_c,ExactlyOnePrimitive,SourceCandidate)` | the constructor row in section 5.3.3 |

The slot-2 predicate conjunction order is exactly origin agreement, anchor
agreement, selector agreement. `ExistsUnique` supplies membership in `U` and
the unique-binder quantifier; no redundant `Membership` node is emitted.
`MatchE` is the atomic proposition descriptor defined in section 5.2. Slot 4's
peer carries the same constructor and full evaluation binding but no embedded
envelope value.

#### 5.3.3 Exact naturality templates

Writing `N`, `R`, `O`, and `W` for `Normalize`, `Reindex`, `Out_c`, and
`RawOut_c` node emission, slot 7 is exactly:

| Constructor | Left endpoint | Right endpoint |
| --- | --- | --- |
| Formation | `N(O(R(E,q)))` | `N(R(O(E),q))` |
| Abstraction | `N(R(O(E),q))` | `N(O(R(E,q)))` |
| Aggregation | `N(R(O(E),q))` | `N(O(R(E,q)))` |
| Transport | `N(W(R(E,q)))` | `N(O(R(E,q)))` |
| Comparison | `N(R(O(E),q))` | `N(O(R(E,q)))` |
| DemandCompiler | `N(R(O(E),q))` | `N(O(R(E,q)))` |
| DischargeTransformer | `N(W(R(E,q)))` | `N(O(R(E,q)))` |

The clause root is `ApproxOrd(left,right)` except for DischargeTransformer,
whose root is `DependentDischargeEquality(left,right)`. These rows are
definitionally the
A3 displays: Formation's sides are `Out(reindex_theta(E))` and `A[theta]`;
Abstraction/Aggregation/Comparison/DemandCompiler expand the right `O(R(E,q))`
through the unique `Fam`/concatenation/`q`/`Spec` `Compose` row; and the two
square rows expand to `C_raw[rho]` versus `(A[sigma])[theta_prime]`, or
`rho^*d_raw` versus `theta_prime^*(sigma^*d)`.

Before those two endpoints, slot 7 emits action-argument projections in this
exact order, retaining even an argument subsequently reconstructed by
`Reindex`:

| Constructor | Ordered `Arg` projections |
| --- | --- |
| Formation | `theta` |
| Abstraction | `theta`, `theta^A` |
| Aggregation | `theta`, `theta^A` |
| Comparison | `theta` |
| DemandCompiler | `theta`, `theta^E` |
| Transport | `rho`, `sigma`, `theta_prime`, `epsilon`, `theta.rho`, `sigma.theta_prime`, the `epsilon`-congruence endpoint |
| DischargeTransformer | `rho`, `sigma`, `theta_prime`, `epsilon`, `theta.rho`, `sigma.theta_prime`, reindexed `G`, `S`, `a`, `L`, every retained contract, and the `epsilon`-congruence endpoint |

Within the final DischargeTransformer row, `G`, `S`, `a`, `L` and contracts follow
their A3 payload order. These formal auxiliaries are retained typed endpoint
nodes, but universally bound declarations emit no candidate dependency slot.

`theta`, or `rho/sigma/theta_prime`, use the `PrimitiveArgument` root.
Every lift, composition, `epsilon`, congruence endpoint, reindexed index, and
contract uses `DerivedComponent`; in particular no path claims that R2b's
three-field checked square primitive stores `epsilon`. The latter is freshly
reconstructed from those three fields under the square-checking rule.

#### 5.3.4 Exact two-whisker templates

Slots 8--10 exist only for Transport and DischargeTransformer. Each slot
independently repeats its complete binder vector; no binder is shared across
slots. Transport binds:

```text
Omega, Gamma, Delta, Xi : DependentContext
sigma : Substitution(Gamma,Omega)
theta : Substitution(Delta,Gamma)
rho   : Substitution(Xi,Delta)
Z     : NonemptyPublicInterface(Omega)
```

DischargeTransformer binds:

```text
Omega, Gamma, Delta, Xi : DependentContext
sigma : Substitution(Gamma,Omega)
theta : Substitution(Delta,Gamma)
rho   : Substitution(Xi,Delta)
K     : Carrier(SealedPublicGrammar(Omega))
T     : Carrier(DemandScheme(Omega,K))
b     : SchemeActivation(Omega,K,T)
e     : DischargeAtSpecialization(Omega,K,T,b)
```

All references are `EarlierBinder` values in the displayed order. For
Transport the compiler forms:

```text
T0=N(R(R(R(Z,sigma),theta),rho))
T1=N(R(R(Z,sigma),Compose(theta,rho)))
T2=N(R(Z,Compose(sigma,Compose(theta,rho))))
T3=N(R(Z,Compose(Compose(sigma,theta),rho))).
```

For DischargeTransformer it replaces `Z` by `e` and names the resulting four
values `D0..D3`; type-directed `Reindex` is the discharge pullback and retains
the derived `K/T/b/M` indices. Slot 8 emits endpoints 0 and 1, slot 9 emits 1
and 2, and slot 10 emits 2 and 3. Transport appends one `ApproxOrd` clause;
DischargeTransformer appends one `DependentDischargeEquality` clause. An
intermediate repeated in adjacent slots is re-emitted in full.

#### 5.3.5 Canonical node emission

For each slot, expressions above are expanded by strict left-to-right
postorder. Every syntactic occurrence emits a fresh `Binder`, `Literal`,
`Project`, `Normalize`, `Reindex`, `Compose`, `RenameHistory`, `Selector`, or
`Origin` node; each applicable clause emits its one fresh
`DependentDischargeEquality` node in the proposition suffix. There is no
common-subexpression elimination, even for `E`, a
binder, or a repeated endpoint. Slot value blocks are concatenated in ascending
slot order. Only after every value block, proposition suffixes are emitted in
slot order by the same postorder rule, reusing the already emitted endpoint
root ordinals and emitting no value node again. Finally one `Conjunction` is
appended with the clause roots in slot order.

Binder vectors are concatenated in slot order before node emission. All binder
and node references in a template are relocated by that slot's checked global
offset; cross-slot references are forbidden. `value_node_types` records one
nonzero result type per emitted value node. This algorithm fixes sharing,
repetition, operand order, clause suffixes, and the final root, so two valid
encoders cannot choose byte-distinct DAGs.

The source bundle is byte-identical to the corresponding slot-4 projections.
The projection is constructed before support and is one support root; it
contains only the value programs, not proposition suffixes or a field-16
wrapper.

R2a tags `0..15` remain byte-for-byte; C2 appends exactly tag 16:

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
| DependentDischargeEquality=16(left_node:u32,right_node:u32).
```

Every node operand is strictly smaller than its node ordinal. The final root
is a `Conjunction` whose child vector is exactly the clause roots in slot
order. The full syntax's binder vector is the concatenation of the endpoint
program binder vectors. Its node vector begins with the byte-identical
concatenation of every endpoint program's value nodes, followed only by the
proposition nodes for slots in order and the final conjunction. All binder,
node, and path ordinals are range checked with `u32` conversion before
allocation or traversal. Counts use the universal `u64` count wire; each span
addition is checked without overflow and its exclusive end must admit an exact
`u32` conversion into the corresponding ordinal domain.

Exact construction evidence uses:

```text
SubjectSlotSpanV1 =
  (subject_slot:u16,
   root_kind:SubjectRootKindV1,
   first_binder:u32,
   binder_count:u64,
   first_value_node:u32,
   value_node_count:u64,
   first_clause_node:u32,
   clause_node_count:u64,
   clause_root_node:u32)

SubjectConstructionTranscriptV1 =
  (slot_spans:vector<SubjectSlotSpanV1>,
   final_conjunction_node:u32,
   emitted_projection:SubjectRootProjectionV1,
   node_type_replays:vector<R3::KernelSuccessReceiptV1>,
   resource_account:R3::EnvelopeResourceAccountV1)

EnvelopeSubjectsV1 =
  (syntax:SubjectSyntaxV1,
   root_projection:SubjectRootProjectionV1).
```

Binder spans are contiguous, nonoverlapping, and cover every binder. Value spans
are contiguous and cover exactly the initial value-node block in slot order;
clause spans are contiguous and cover exactly the following proposition-node
block in the same order. The one final conjunction follows every clause span,
is selected by both `final_conjunction_node` and `syntax.root_node`, and is not
charged to an individual slot. Re-extracting endpoint literals from the value
spans must produce bytes identical to both `emitted_projection` and the
pre-support projection retained by `PublicLeavesAndLineageV1`. Field 16 is
exactly `EnvelopeSubjectsV1`; the construction transcript belongs to root
`0xfc` and does not add a third field-16 component.

For each span, `first_* + *_count` is checked in `u64`, is at most the exact
corresponding vector length, and its exact conversion to `u32` is the exclusive
end ordinal. Consequently a count is never smuggled onto the narrower ordinal
wire, while every stored start/root remains a genuine `u32` ordinal.

`node_type_replays` has exactly the full syntax node count and follows node
ordinal order, including the trailing conjunction. Receipt `i` is the R3
success receipt for independently checking node `i` at its uniquely compiled
type; no aggregate receipt substitutes for a row.

For `u:EnvelopeSubjectsV1` and its transcript
`t:SubjectConstructionTranscriptV1`, the account bytes are fixed by

```text
SubjectConstructionEvents(u,t) =
  BinderAndNodeCompilationEvents(u.syntax)
  ++ NodeTypeCheckEvents(t.node_type_replays)
  ++ SpanAndProjectionBindingEvents(
       t.slot_spans,t.final_conjunction_node,
       t.emitted_projection,u.root_projection)
  ++ SubjectTranscriptReconstructionEvents(t)

t.resource_account =
  Acct_R3(SubjectConstruction,SubjectConstructionEvents(u,t)).
```

The reconstruction stream omits `t.resource_account` itself. It includes the
exact span cardinalities, backward-reference/root checks, projection byte
equality, and transcript encoding, but does not repeat the node kernel calls
already charged by `NodeTypeCheckEvents`.

The forbidden-literal list is absolute: no `RawCapabilityEnvelopeV1`, checked
candidate, `OrdinarySupportV1`, computed `SupportProjectionV1`, quotient/final
key, action trace/image, pair transcript, or pair disposition can occur in a
subject binder, literal, node, or projection.

### 5.4 Pure support-projection subjects

```text
SupportProjectionSubjectKindV1 ::=
  NormalizationPreservation=0
| AdmissibleRenamingPreservation=1
| ReindexingPreservation=2

SupportProjectionRequirementV1 =
  (kind:SupportProjectionSubjectKindV1,
   evaluation_binding:EvaluationBindingRefV1,
   raw_code:RawConstructorCodeV1,
   subject_endpoint_paths:vector<SubjectProjectionSourcePathV1>,
   construction_rule:DefinitionRulePathV1)

SupportProjectionSeedV1 =
  (evaluation_binding:EvaluationBindingRefV1,
   raw_code:RawConstructorCodeV1,
   source_bundle:SubjectProjectionSourceBundleV1,
   subject_endpoint_paths:vector<SubjectProjectionSourcePathV1>)

SupportProjectionExpressionV1 ::=
  ProjectSeed=0(SupportProjectionSeedV1)
| NormalizedSeed=1(SupportProjectionSeedV1)
| RenamedSeed=2(SupportProjectionSeedV1,renaming_binder:u32)
| ReindexedSeed=3(SupportProjectionSeedV1,action_binder:u32)

SupportProjectionSubjectV1 =
  (kind:SupportProjectionSubjectKindV1,
   binders:vector<SubjectBinderV1>,
   left:SupportProjectionExpressionV1,
   right:SupportProjectionExpressionV1,
   proposition:SubjectSyntaxV1).
```

The proposition has exactly two
`PureSupportFunctionProjection` endpoint literals whose binder/function tags
match `left` and `right`, and an `Equality` root. A seed contains the exact
evaluation/raw/source/endpoint descriptors, but no action-component census,
`SubjectRootProjectionV1`, computed `SupportProjectionV1`, normalization
evidence, or `OrdinarySupportV1`. In the reindexing row the complete arbitrary
normalized postfix action is represented only by the formal-action binder and
the `ReindexedSeed(seed,q)` expression. Its variable finite primitive count is
not serialized into the seed. Primitive-by-primitive action instantiation is
an R2d action-trace concern, not an input to this parametric preservation
subject. The normalization instance is byte-identical wherever it is repeated
in normalization and support evidence. These subjects make no claim that a
proof has been supplied.

For every seed, `evaluation_binding=source_bundle.evaluation_binding` and
`raw_code=source_bundle.selector_inputs.match_premise.raw_code` by full bytes.
`subject_endpoint_paths` is the exact scan
of every physically encoded `SubjectProjectionSourcePathV1` in the enclosing
endpoint programs, in subject-slot then value-node order and literal-field
order, preserving repeated paths. A non-source literal contributes no row;
there is no sorting or deduplication.

Let `SeedOf(e)` be the first payload of any of the four
`SupportProjectionExpressionV1` variants. In every realized subject,
`SeedOf(left)=SeedOf(right)=seed` by full bytes. Each group-5
`SupportProjectionRequirementV1` is then constructed field by field, rather
than by an ill-typed record erasure:

```text
requirement.kind                   = subject.kind
requirement.evaluation_binding     = seed.evaluation_binding
requirement.raw_code               = seed.raw_code
requirement.subject_endpoint_paths = seed.subject_endpoint_paths
requirement.construction_rule      = RuleFor(subject.kind)
```

Here `RuleFor(NormalizationPreservation)=Normalization[0]` and
`RuleFor(AdmissibleRenamingPreservation)=RuleFor(ReindexingPreservation)=
SupportAndOwnerResolution[2]`. The seed has no `kind` or `construction_rule`
field from which either value could be copied. Its `source_bundle` is
independently required to be the enclosing bundle containing that exact
obligation, and the first two seed equations above bind it to that bundle.
This construction binds every requirement byte without making the obligation
graph cyclic.

The three instances have no binder freedom:

| Kind | Binders | Left | Right |
| ---: | --- | --- | --- |
| 0 | one `SupportProjection` binder | `ProjectSeed(seed)` | `NormalizedSeed(seed)` |
| 1 | `SupportProjection`, then `HistoryRenaming` | `ProjectSeed(seed)` | `RenamedSeed(seed,h)` |
| 2 | `SupportProjection`, then `FormalAction(a_c,ArbitraryNormalized,SourceCandidate)` | `ProjectSeed(seed)` | `ReindexedSeed(seed,q)` |

In every row the unique support-projection binder has ordinal `b=0`. Put
`T_support = type_id(SupportProjectionV1)`, meaning the exact nonzero local
`TypeIdV1` that R2e assigns to the complete support-projection result type. The
proposition bytes are fixed exactly by:

```text
proposition.binders = binders
proposition.nodes = [
  Literal(PureSupportFunctionProjection(
    support_binder=0,projection=Full,result_type=T_support)),
  Literal(PureSupportFunctionProjection(
    support_binder=0,projection=Full,result_type=T_support)),
  Equality(left_node=0,right_node=1)
]
proposition.root_node = 2.
```

Node 0 is interpreted only as the full pure support-function application to
the outer `left` expression; node 1 is interpreted only as the corresponding
application to the outer `right` expression. This positional association is
part of `SupportProjectionSubjectV1` validation and cannot be swapped or
redirected to another expression. `Current`, `Older`, and `OutputRoot` are
invalid in all three preservation propositions, and no alternate result type
is admitted. Those other projection tags remain reserved for explicitly typed
future subjects; their mere presence in the closed enum creates no choice
here.

In the final type DAG, the already acyclic `ExactReferentV1`,
`ExpectedJudgmentV1`, `SupportKeyV1`, and `SupportProjectionV1` type nodes are
placed before `CoreProjectionLiteralV1`, even though their runtime support
compiler is presented later. The literal carries only that earlier TypeId and
never a computed support value.

The proposition binder vector is byte-identical to the outer binder vector.
Its nodes are exactly left literal at ordinal 0, right literal at ordinal 1,
and `Equality(0,1)` at ordinal 2, with `root_node=2`; there is no conjunction,
sharing choice, or extra endpoint. Kind, expression tags, binder ordinals, and
the shared seed must agree with the displayed row. The reindexing expression's
`action_binder` is exactly the ordinal of that row's sole `FormalAction`
binder. No `FormalActionProjectionPathV1`, redundant `FormalActionIdV1`,
`FormalGlobal` identity, checked action value, or action support root is
encoded in a pure-support seed.

## 6. Exact normalization evidence

Normalization starts from checked built material, not an envelope:

```text
AnchorIdentityV1 =
  (exact:OccurrenceId14V1,
   normalized:NormalizedOccurrenceKeyV1)

RawNormalizationEndpointV1 =
  (seed:CandidateAssemblySeedV1,
   versions:EnvelopeVersionBindingsV1,
   anchor:AnchorIdentityV1,
   constructor_subject:ConstructorSubjectBindingV1,
   indexed_inputs:IndexedInputsV1,
   indexed_output:IndexedOutputV1,
   raw_interface_and_implementation:RawInterfaceAndImplementationV1,
   fields:vector<NormalizedPublicFieldV1>,
   field_lineages:vector<FieldLineageV1>,
   output_lineage:vector<OutputLineageEntryV1>,
   implementation:ExactOccurrenceFootprintV1,
   prior_and_current:PriorAndCurrentOutputFootprintsV1,
   replay_boundaries:ReplayBoundariesAndContextsV1,
   obligations:DerivedObligationsV1,
   subject_root_projection:SubjectRootProjectionV1)

NormalizedEnvelopeEndpointV1 =
  (versions:EnvelopeVersionBindingsV1,
   parents:EnvelopeParentBindingsV1,
   owned_history:OwnedHistoryAndEvaluationV1,
   anchor:AnchorIdentityV1,
   constructor:ConstructorTagV1,
   canonical_payload:CanonicalSchemaPayloadV1,
   indexed_inputs:IndexedInputsV1,
   indexed_output:IndexedOutputV1,
   normalized_public_fields:vector<NormalizedPublicFieldV1>,
   field_lineages:vector<FieldLineageV1>,
   output_lineage:vector<OutputLineageEntryV1>,
   implementation:ExactOccurrenceFootprintV1,
   prior_and_current:PriorAndCurrentOutputFootprintsV1,
   replay_boundaries:ReplayBoundariesAndContextsV1,
   obligations:DerivedObligationsV1,
   subject_root_projection:SubjectRootProjectionV1).
```

Although `RawNormalizationEndpointV1.fields` already carries normalized leaf
values produced by the independently checked built record, the endpoint still
retains every raw telescope, section, interface, and implementation value
through its complete checked built and field-11 projections. It is therefore
the A1 typed pre-evidence endpoint, not a normalized-output-only shortcut.
Its versions, anchor, field/output lineages, and replay boundaries are the
byte-identical slot-4 projections. The normalized endpoint retains the
identity, provenance, boundary, footprint, obligation, and pre-support subject
projections needed by the construction equations. Excluded metadata fields are
preserved by direct byte-equality and anchor equations; the closed
presentation-atom frontier below is intentionally limited to payload,
interface, public-field, and implementation material.

```text
PresentationStepKindV1 ::=
  KernelNormalization=0(NormalizationRuleV1)
| CaptureSafeSubstitution=1
| IteratedBinderLift=2
| RecordReconstruction=3(ReconstructionTagV1)

NormalizationMaterialValueV1 ::=
  TermValue=0(Term)
| ContextValue=1(NormalizedContextV1)
| TelescopeValue=2(NormalizedTelescopeV1)
| SectionValue=3(NormalizedSectionV1)
| SubstitutionValue=4(NormalizedSubstitutionV1)
| CarrierValue=5(NormalizedCarrierValueV1)
| CheckedContextValue=6(CheckedContextV1)
| CheckedTelescopeValue=7(CheckedTelescopeV1)
| CheckedSectionValue=8(CheckedSectionV1)
| CheckedDischargeValue=9(CheckedDischargeV1)
| SchemaPayloadValue=10(CanonicalSchemaPayloadV1)
| IndexedInputsValue=11(IndexedInputsV1)
| IndexedOutputValue=12(IndexedOutputV1)
| PublicFieldValue=13(NormalizedPublicFieldV1)
| OccurrenceFootprintValue=14(ExactOccurrenceFootprintV1)
| PriorAndCurrentFootprintsValue=15(PriorAndCurrentOutputFootprintsV1)

NormalizationStepInputRootV1 ::=
  RawEndpoint=0
| NormalizedEndpoint=1
| EarlierStep=2(step_ordinal:u32)

NormalizationStepInputPathV1 =
  (root:NormalizationStepInputRootV1,
   steps:vector<TypedPathStepV1>,
   result_type:TypeIdV1)

NormalizationStepOutputRootV1 ::=
  IntermediateResult=0
| NormalizedEndpoint=1

NormalizationStepOutputPathV1 =
  (root:NormalizationStepOutputRootV1,
   steps:vector<TypedPathStepV1>,
   result_type:TypeIdV1)

NormalizationStepEvidenceV1 ::=
  KernelNormalization=0(KernelSuccessReplayV1)
| CaptureSafeSubstitution=1(VerifiedParticularOpenTypedSubstitutionV1)
| IteratedBinderLift=2(VerifiedParticularOpenTypedSubstitutionV1)
| RecordReconstruction=3(
    reconstruction:ReconstructionTagV1,
    kernel_replays:vector<KernelSuccessReplayV1>,
    telescope_comparisons:vector<NormalizedTelescopeComparisonV1>)

NormalizationStepV1 =
  (step_ordinal:u32,
   kind:PresentationStepKindV1,
   predecessor_steps:vector<u32>,
   input_paths:vector<NormalizationStepInputPathV1>,
   output_path:NormalizationStepOutputPathV1,
   before:NormalizationMaterialValueV1,
   after:NormalizationMaterialValueV1,
   evidence:NormalizationStepEvidenceV1)

RawNormalizationEndpointReplayV1 =
  (endpoint:RawNormalizationEndpointV1,
   kernel_replays:vector<KernelSuccessReplayV1>,
   telescope_comparisons:vector<NormalizedTelescopeComparisonV1>)

NormalizedEnvelopeEndpointReplayV1 =
  (endpoint:NormalizedEnvelopeEndpointV1,
   kernel_replays:vector<KernelSuccessReplayV1>,
   telescope_comparisons:vector<NormalizedTelescopeComparisonV1>)

NormalizationIdempotenceEvidenceV1 =
  (first:NormalizedEnvelopeEndpointV1,
   second:NormalizedEnvelopeEndpointV1,
   second_pass_steps:vector<NormalizationStepV1>,
   field_comparisons:vector<NormalizedTelescopeComparisonV1>)

RawNormalizationEndpointPathV1 =
  (steps:vector<TypedPathStepV1>,
   result_type:TypeIdV1)

NormalizedEnvelopeEndpointPathV1 =
  (steps:vector<TypedPathStepV1>,
   result_type:TypeIdV1)

NormalizationMaterialClassV1 ::=
  CanonicalPayloadComponent=0(component_ordinal:u32)
| IndexedInputComponent=1(component_ordinal:u32)
| IndexedOutputComponent=2(component_ordinal:u32)
| PublicField=3(field_ordinal:u32)
| ImplementationFootprint=4
| PriorSupportFootprint=5
| CurrentOutputFootprint=6

EnvelopePresentationAtomV1 =
  (material_ordinal:u32,
   material_class:NormalizationMaterialClassV1,
   left:RawNormalizationEndpointPathV1,
   right:NormalizedEnvelopeEndpointPathV1,
   relation:PresentationRelationV1)

PresentationRelationV1 ::=
  ExactBytes=0
| OrdinaryNormalizedTerm=1
| OrdinaryNormalizedTelescope=2

EnvelopePresentationWitnessV1 =
  (raw_endpoint:RawNormalizationEndpointV1,
   normalized_endpoint:NormalizedEnvelopeEndpointV1,
   atoms:vector<EnvelopePresentationAtomV1>,
   kernel_replays:vector<KernelSuccessReplayV1>,
   substitution_replays:
     vector<VerifiedParticularOpenTypedSubstitutionV1>,
   telescope_comparisons:vector<NormalizedTelescopeComparisonV1>)

AnchorNormalizationEvidenceV1 =
  (origin_path:OriginPathV1,
   before:PrincipalRootLineageV1,
   after:PrincipalRootLineageV1)

EnvelopeNormalizationEvidenceV1 =
  (raw_endpoint:RawNormalizationEndpointV1,
   ordered_steps:vector<NormalizationStepV1>,
   normalized_endpoint:NormalizedEnvelopeEndpointV1,
   raw_endpoint_replay:RawNormalizationEndpointReplayV1,
   normalized_endpoint_replay:NormalizedEnvelopeEndpointReplayV1,
   idempotence:NormalizationIdempotenceEvidenceV1,
   whole_envelope_presentation:EnvelopePresentationWitnessV1,
   anchor_preservation:AnchorNormalizationEvidenceV1,
   support_projection_subject:SupportProjectionSubjectV1,
   substitution_compatibility_subject:SubjectSyntaxV1,
   resource_account:R3::EnvelopeResourceAccountV1).
```

For `n:EnvelopeNormalizationEvidenceV1`, the normalization account is exactly

```text
NormalizationEvents(n) =
  FirstPassStepEvents(n.ordered_steps)
  ++ EndpointReplayCensusBindingEvents(
       n.raw_endpoint,n.raw_endpoint_replay)
  ++ EndpointReplayCensusBindingEvents(
       n.normalized_endpoint,n.normalized_endpoint_replay)
  ++ IdempotenceSecondPassEvents(n.idempotence)
  ++ PresentationBindingEvents(n.whole_envelope_presentation)
  ++ AnchorPreservationBindingEvents(n.anchor_preservation)
  ++ SupportProjectionSubjectConstructionEvents(
       n.support_projection_subject)
  ++ SubstitutionSubjectConstructionEvents(
       n.substitution_compatibility_subject)
  ++ NormalizationRecordReconstructionEvents(n)

n.resource_account =
  Acct_R3(Normalization,NormalizationEvents(n)).
```

`FirstPassStepEvents` and `IdempotenceSecondPassEvents` charge each displayed
kernel, substitution, lift, comparison, and reconstruction operation exactly
at its step occurrence. The two replay-census streams and
`PresentationBindingEvents` charge only path decoding, cardinality, and byte-
equality/reconciliation of already charged replay records; they do not execute
or charge those calls again. The final reconstruction stream excludes every
nested receipt account and `n.resource_account` itself.

For any endpoint value `v`, `StoredReplayCensus(v)` is the deterministic
schema-DAG preorder that appends a subvalue when its exact type is
`KernelSuccessReplayV1` or `NormalizedTelescopeComparisonV1` and does not
descend inside an appended replay. It otherwise visits sum payloads, record
fields, and vector elements in canonical field/element order. The endpoint
replay equations are exactly

```text
raw_endpoint_replay.endpoint = raw_endpoint
raw_endpoint_replay.kernel_replays
  = StoredReplayCensus(raw_endpoint).kernel
raw_endpoint_replay.telescope_comparisons
  = StoredReplayCensus(raw_endpoint).telescope

normalized_endpoint_replay.endpoint = normalized_endpoint
normalized_endpoint_replay.kernel_replays
  = StoredReplayCensus(normalized_endpoint).kernel
normalized_endpoint_replay.telescope_comparisons
  = StoredReplayCensus(normalized_endpoint).telescope

idempotence.first = normalized_endpoint
whole_envelope_presentation.raw_endpoint = raw_endpoint
whole_envelope_presentation.normalized_endpoint = normalized_endpoint.
```

All are full-byte equalities. This census is replay metadata validation, not
normalization material, and has no caller-selected inclusion or ordering.

Define the closed virtual list `NormalizationMaterialPaths_c` by the following
tagged concatenation. In the first three segments, `component_ordinal` is the
contiguous ordinal of the emitted atom within that class (not merely its outer
record-field ordinal):

1. `CanonicalPayloadComponent`: the selected constructor's payload fields in
   the display order of `CanonicalSchemaPayloadV1`, with left paths into the
   corresponding selected `BuiltConstructorValueV1` under
   `raw_endpoint.constructor_subject.checked_built.full_identity` and right
   paths into `normalized_endpoint.canonical_payload`;
2. `IndexedInputComponent`: indices first and nominal inputs second, each in
   its selected constructor field order, pairing the two endpoint copies;
3. `IndexedOutputComponent`: the selected output in its frozen carrier order;
4. `PublicField`: one segment per exact field ordinal, pairing `fields[i]` with
   `normalized_public_fields[i]`; and
5. `ImplementationFootprint`, `PriorSupportFootprint`, and
   `CurrentOutputFootprint`, in that order.

Traversal recursively uses the closed A2 carrier-field order only through
normalized carrier types and ordinary product/sum/vector structure. A checked
wrapper is never traversed generically. At a pair of checked-wrapper fields the
compiler must apply exactly one row of this closed semantic-projection table:

| Encountered pair | Semantic path appended on both sides | Material rule | Fields excluded from semantic descent |
| --- | --- | --- | --- |
| `(CheckedContextV1,CheckedContextV1)` | `.normalized_context` | descend through the reached `NormalizedContextV1` | `raw_code`, `raw_context`, `binder_keys`, `formation_replays` |
| `(CheckedTelescopeV1,CheckedTelescopeV1)` | `.normalized` | emit one `OrdinaryNormalizedTelescope` atom | `raw_code`, `base_context`, `fields`, `end_context` |
| `(CheckedSectionV1,CheckedSectionV1)` | `.normalized` | descend through the reached `NormalizedSectionV1` | `raw_code`, `base_context`, `target`, `fields` |
| `(CheckedDischargeV1,CheckedDischargeV1)` | `.normalized` | descend through the reached `NormalizedDischargeValueV1` and its `section` | `raw_code`, checked `section`, `instantiated_contract_replays` |

Before taking a row, the two wrapper tags and record/vector shapes must agree,
and every listed excluded field must be byte-identical under the canonical
payload projection equation. The table therefore suppresses those fields from
the atom census without leaving any of their bytes free. In particular, nested
`CheckedTypeViewV1`, `CheckedTermViewV1`, weakening data, source/path/binder
annotations, raw code, formation/typing/kernel replays, checked contracts, and
receipt metadata are reached only inside an excluded field and emit no atom.
Encountering a mismatched pair or any checked-wrapper type not in the table at
a material location is an invariant failure; there is no fallback recursion.

After the table projection, each maximal `NormalizedTelescopeV1` pair emits
one `OrdinaryNormalizedTelescope` atom, each maximal `Term` pair emits one
`OrdinaryNormalizedTerm` atom, and every remaining scalar or non-term leaf
emits one `ExactBytes` atom. Tags, vector lengths, and ordinary record wrappers
are checked while descending and do not create caller-selectable atoms. This
exact list is the normalization material frontier: owner/profile/history
bindings, anchor and lineage metadata, raw ordinals/codes, duplicated
decoded/build evidence, replay boundaries, obligation and subject descriptors,
receipts, and resource accounts are deliberately excluded. Their endpoint
copies are bound independently by the field-projection, checked-wrapper, and
anchor equations; their term-bearing receipt subfields are not normalization
material.

The excluded-field equations are exactly

```text
normalized.versions                = raw.versions
normalized.parents                 = raw.seed.parents
normalized.owned_history           = raw.seed.history_and_evaluation
normalized.anchor                  = raw.anchor
normalized.constructor             = raw.seed.constructor
                                   = raw.constructor_subject.constructor
normalized.field_lineages          = raw.field_lineages
normalized.output_lineage          = raw.output_lineage
normalized.replay_boundaries       = raw.replay_boundaries
normalized.obligations             = raw.obligations
normalized.subject_root_projection = raw.subject_root_projection.
```

Each equality is full canonical bytes. The corresponding versions,
evaluation binding, raw code, and constructor repeated inside every selected
subrecord also agree; there is no metadata erasure or second projection source.

`ordered_steps` is compiled only from the left paths of this list, in material
order, by the frozen A2 capture-safe traversal. At each material value the
compiler visits children left-to-right, emits one capture-safe-substitution
event for each particular substitution application, one iterated-lift event
for each individual binder lift, one kernel event for each deterministic
kernel reduction, and one record-reconstruction event after each reconstructed
record node. No event batches two reductions, substitutions, lifts, or record
nodes. Step ordinals are contiguous.

For every step, `predecessor_steps` is the sorted duplicate-free set of exactly
the earlier producer ordinals named by its `EarlierStep` input paths, while
`input_paths` is the unsorted semantic operand order. First-pass paths may use
`RawEndpoint` or an earlier step and never `NormalizedEndpoint`; second-pass
paths may use `NormalizedEndpoint` or an earlier step and never `RawEndpoint`.
An earlier-step path is decoded against that step's `after` value. The first
input value is byte-identical to `before`. An `IntermediateResult` output has
an empty step vector and denotes the complete `after` value; a
`NormalizedEndpoint` output is decoded against the pass destination endpoint
and its reached value is byte-identical to `after`. Every intermediate result
is consumed by a later step, and the last producer for every non-exact material
atom uses that atom's right endpoint path. Result types are equal, nonzero, and
match the displayed material-value variant.

Kind and evidence tags agree. A kernel, substitution, or lift event contains
exactly one replay value. A reconstruction event contains exactly the complete
local kernel replays and telescope comparisons in child order and has the same
`ReconstructionTagV1` in kind and evidence. The nested
`NormalizationRuleV1` is the four-way kernel sum `BetaApply`,
`PermittedGlobal(GlobalId)`, `FirstPairProjection`, and
`SecondPairProjection`. A global reduction is present only for a global whose
body is available under the exact opaque `Sigma(H,o)`; older bodies cannot be
unfolded. Every replay has the exact R3 call role.

The raw replay repeats the top-level raw endpoint byte-for-byte, and the
normalized replay does the same for the top-level normalized endpoint. The
normalized endpoint is complete and canonical. The idempotence record's
`first` and `second` values are byte-identical; `second_pass_steps` is the
output of this identical compiler with `first` as its
`NormalizedEndpoint` source and `second` as destination. It may be empty only
when the deterministic event census is empty. `field_comparisons` is exactly
the material-order vector obtained from every
`NormalizationMaterialPaths_c` atom tagged
`OrdinaryNormalizedTelescope`, comparing that atom's right path in `first`
with the byte-identical right path in `second`; there is one comparison per
such atom, with the idempotence call role, and no other row. An atom's left path is decoded only against the witness's
`raw_endpoint`; its right path only against `normalized_endpoint`. Both paths
terminate at nonzero, relation-admissible types. The exact admissibility table
is `ExactBytes: (T,T)`, `OrdinaryNormalizedTerm: (Term,Term)`, and
`OrdinaryNormalizedTelescope:
(NormalizedTelescopeV1,NormalizedTelescopeV1)`; no checked wrapper can be an
atom endpoint and no other type pair is legal. `atoms` is
byte-for-byte `NormalizationMaterialPaths_c`, including contiguous
`material_ordinal`, exact class, paths, and relation; it is neither sorted nor
deduplicated afterward. The presentation witness's `kernel_replays` is the
step-order concatenation of each kernel event's one replay and each
reconstruction event's local kernel vector; `substitution_replays` is the
step-order vector of the one replay from every capture-safe-substitution and
iterated-lift event; and `telescope_comparisons` is the step-order
concatenation of reconstruction comparison vectors. No other replay occurs in
those three vectors. Thus every and only quotient-relevant material projection
occurs once, with no caller-selected omission. The two principal lineage
records and every excluded metadata projection are byte-identical under their
independent construction equations.

The anchor record has no free path or copied lineage:

```text
anchor_preservation.before =
  raw_endpoint.subject_root_projection.source_bundle.principal_lineage
anchor_preservation.after =
  normalized_endpoint.subject_root_projection.source_bundle.principal_lineage
anchor_preservation.origin_path
  = anchor_preservation.before.origin_path
  = anchor_preservation.after.origin_path.
```

The two complete lineage records are byte-identical, and their exact anchor is
the endpoint anchor and assembly-seed anchor by the section-5 equations.

The support subject is
`NormalizationPreservation` over the raw and normalized pure projection
seeds. The substitution subject is exactly
`nf(E[theta]) approx_env nf(E)[theta]`, expressed only through descriptor
literals and one `FormalAction(a_c,ArbitraryNormalized,SourceCandidate)` binder. Its nodes are
exactly `E`, `theta`, `Reindex`, `Normalize`, then a fresh `E`, `Normalize`, a
fresh `theta`, `Reindex`, and `ApproxEnvelope(left,right)` at ordinals `0..8`,
with root 8 and no conjunction or sharing. It contains no `E` by value, envelope core,
support wrapper, quotient key, action image, or proof.
Every `SubjectProjectionSourcePathV1` in this substitution subject is resolved
against `raw_endpoint.subject_root_projection.source_bundle`; no implicit
normalized-endpoint or later-envelope bundle is available. The resource account is
an R3-owned exact slot under root `0xfb` and is copied into root `0xfc`; it is
not an estimate or a caller limit.

## 7. Exact candidate structural support

### 7.1 Support roots and dependency records

The candidate root-value sum is closed:

```text
CheckedActionArgumentEntryV1 =
  (action_argument_ordinal:u32,
   formal_id:FormalActionIdV1,
   checked_primitive:CheckedPrimitiveActionSyntaxV1)

CheckedActionArgumentRootV1 =
  (arguments:vector<CheckedActionArgumentEntryV1>)

SupportRootValueV1 ::=
  Indices=0(ConstructorIndicesV1)
| Inputs=1(NominalInputsV1)
| Output=2(IndexedOutputV1)
| RawRealization=3(RawRealizationValueV1)
| RawCode=4(RawConstructorCodeV1)
| Obligations=5(DerivedObligationsV1)
| SubjectProjection=6(SubjectRootProjectionV1)
| ActionArgument=7(CheckedActionArgumentRootV1).

SupportRootV1 =
  (root:SupportRootTagV1,
   value:SupportRootValueV1).
```

The two tags agree. A candidate has exactly the seven roots `0..6` in that
order and no tag-7 root. R2d action support appends exactly one tag-7 root after
rebuilding the first seven roots; it cannot change this candidate type or
order for every nonempty action. That root's nonempty `arguments` vector, not
repeated tag-7 roots, contains every checked primitive of the complete flattened
normalized action. Its entries have `action_argument_ordinal=i` at vector
position `i` and preserve primitive order. The empty identity action appends no
segment and therefore has no tag-7 root, making its root/tag/path stream
byte-identical to the base candidate as A3-O requires. R2d binds each `formal_id` and
`checked_primitive` byte-for-byte to the final action trace and instantiated
subject locus; R2c admits no caller-selected duplicate root or ordinal.

For `x:SupportShapeInputV1`, let `B` be
`x.public_leaves.seed.checked_built.full_identity.built`. The root vector is
not caller supplied; it is exactly

```text
[
  SupportRoot(Indices,           ProjectIndices(B)),
  SupportRoot(Inputs,            ProjectNominalInputs(B)),
  SupportRoot(Output,            ProjectIndexedOutput(B)),
  SupportRoot(RawRealization,    ProjectRawRealization(B)),
  SupportRoot(RawCode,           x.public_leaves.seed.raw_code),
  SupportRoot(Obligations,       x.public_leaves.obligations),
  SupportRoot(SubjectProjection, x.public_leaves.subject_root_projection)
].
```

The four `Project*` functions are exactly the tag-preserving projections in
section 3. All seven value tags, `B`'s tag, the seed constructor, the
normalization endpoint constructor, and the outer root tags agree by full
bytes.

Support traversal is a tagged virtual traversal, not blind descent through the
by-value records:

| Root tag | Exact traversal |
| ---: | --- |
| 0--3 | A3-O `PublicLeaves` over the selected indices, inputs, output, or raw-realization value in declared carrier-field order |
| 4 | exhaustive A3-O `SourceLeaves` over the raw code in raw-field preorder |
| 5 | complete typed obligation predicates in field-12 group/entry order |
| 6 | endpoint-program value nodes in subject-slot then node order |
| 7 | the sole action-argument root's vector elements in `action_argument_ordinal` order; absent for a candidate |

For tag 6, `source_bundle` is resolution context only and is not recursively
walked as a record. Binder declarations emit nothing. A source-bundle literal
or project follows exactly its selected path and traverses the reached typed
value; repeated path occurrences remain repeated. A binder-derived
`FormalActionArgumentProjection` or `PureSupportFunctionProjection` counts as
a typed endpoint node but emits no candidate dependency slot. Concrete checked
action arguments are traversed only under action-image root 7 in R2d. No
unselected bundle field, receipt, proof metadata, scalar tag, or ordinal emits
a support slot merely by being visited as a scalar. The explicit
enclosing-record `OperationAnchor` incidence below is the sole ordinal
exception and emits from the operation, port, or live-demand record even though
the selected `u32` itself has no independent `Visit`. Per-root
`node_count` counts nodes visited by this virtual traversal, not all nodes in
the stored record representation.

More precisely, `SupportShapeEvents_c(root,value)` is the unique stream over
`Visit(path,type_id)` and `Slot(shape_slot_without_ordinal)` produced by this
table. Entering a selected sum payload, record, vector container, vector
element, A2 carrier node, or JG2b2b0 term constructor emits one `Visit` before
any children; primitive scalar/tag/length bytes emit none. At a visited carrier
or term node, the A3 edge rules emit all local `Slot` events in the displayed
29-edge order before recursively visiting child values in frozen field/term
edge order. Obligation traversal emits one `ObligationPremise` slot for every
obligation record before visiting its term-bearing predicate sources. Subject
traversal enters only endpoint-program value nodes and then values reached by
their literals/projections. These dispatch clauses are exhaustive for every
local variant and have no default branch.

The complete shape event stream is the concatenation for root tags `0..6`.
`node_count` for a root is its number of `Visit` events; its
`shape_slot_count` is its number of `Slot` events. `shape_slots` is exactly the
global `Slot` subsequence with contiguous ordinals attached, preserving event
order. This fixes wrapper counting, repeated-path behavior, and slot emission;
an implementation cannot count serialized metadata or skip a selected typed
node.

The missing incidence direction is fixed by
`LocalEdgeIncidence_c(visited node,role)`. At every `Visit`, the compiler emits
exactly this vector (sorted by the 29-edge order, preserving the stated local
order among repeated tags) before visiting children:

| Visited node/role | Exact local edge vector and multiplicity |
| --- | --- |
| selected `SourceLeafV1` | one `SourceLeaf` |
| JG2b2b0 `Var` | one `LocalBinderUse` |
| JG2b2b0 `Global` | one `GlobalUse` |
| `Pi` | `TermChild(PiParameter)`, `TermChild(PiBody)` |
| `Sigma` | `TermChild(SigmaParameter)`, `TermChild(SigmaBody)` |
| `Lambda` | `TermChild(LambdaParameterType)`, `TermChild(LambdaBody)` |
| `Apply` | `TermChild(ApplyFunction)`, `TermChild(ApplyArgument)` |
| `Pair` | `TermChild(PairSigmaType)`, `TermChild(PairFirst)`, `TermChild(PairSecond)` |
| `First` | one `TermChild(FirstPair)` |
| `Second` | one `TermChild(SecondPair)` |
| JG2b2b0 `Sort`, `UnitType`, or `Unit` | empty |
| telescope prefix | one `TelescopePrefix` per prefix ordinal, oldest first |
| section expected-type field | one `SectionExpectedType` |
| substitution | one `SubstitutionImage` per image ordinal |
| family index | one `FamilyIndex` per indexed field |
| grammar | one `GrammarExport` per export |
| grammar operation `operations[k]` | `OperationAnchor` once for its derived ordinal `k`, then one `OperationParameter` per parameter field, then one `OperationResult` per result field |
| scheme | one `SchemeTrigger` per trigger |
| scheme port | `OperationAnchor` once for its retained `operation_ordinal`, then one `PortArgument` per argument, then one `PortDerivedOutput` per derived output |
| live-demand record | one `OperationAnchor` per `inherited_operation_ordinals[j]`, in port order |
| activation | one `ActivationImage` per image |
| contract | `ContractType`, `ContractLeft`, `ContractRight`, once each |
| discharge | one `DischargeImage` per image |
| obligation record | one `ObligationPremise` before its selected predicate-source traversal |
| source-backed endpoint occurrence (`CoreProjectionLiteralV1` tag `0..3`) | one `TheoremEndpoint` |
| binder-derived endpoint occurrence (`FormalActionArgumentProjection` or `PureSupportFunctionProjection`, tags `4..5`) | empty |
| record labeled `ExpectedType`, `CaptureSafeSubstitution`, `IteratedBinderLift`, `TelescopeConcatenation`, `ComparisonReplay`, `GrammarReindex`, `SchemeSpecialization`, or `DischargeContractReplay` by the frozen build recipe | exactly one `RecordReconstruction(the same tag)` |
| normalization event labeled by rule `r` | exactly one `NormalizationStep(r)` |
| sum/record/vector wrapper or primitive metadata field with none of the closed roles above | empty |

If one visited value carries several displayed roles, its vector is the stable
29-edge-order merge of those rows; multiplicities and the within-row orders are
unchanged. The wrapper-empty row is restricted to the closed schema variants
whose fields are exhaustively listed above; it is not an implementation
default. In particular, every compatible JG2b2b0 child emits exactly one
`TermChild` slot even though that child is also recursively visited, and every
recipe-labeled reconstruction emits exactly one reconstruction slot. Emit-zero
and emit-two alternatives are invalid.

The full carrier index used by a `CarrierSlot` judgment is also derived by a
closed companion rule; a pure body never supplies missing ambient indices.
Write `cindex(p)` for the index at a visited carrier-field path `p`. If `p` is
inside a complete `NormalizedCarrierValueV1`, `cindex(p)` is the unique
`CarrierIndexValueV1` constructed tag-for-tag from that wrapper's displayed
ambient index fields by the R2b table. Otherwise the only admissible pure-body
cases are:

| Pure body containing `p` | Exact `cindex(p)` |
| --- | --- |
| `NormalizedFamilyValueV1 f` | `InterfaceFamilyIndex(f.gamma,f.a)` |
| `NormalizedGrammarValueV1 G` | `SealedPublicGrammarIndex(I.gamma)` |
| `NormalizedSchemeValueV1 S` | `DemandSchemeIndex(I.gamma,I.grammar)` |

Here `I=ProjectIndices(B)` for the same built candidate that owns the support
root. Every pure `G` or `S` occurrence is required byte-identical to the
corresponding companion component selected from `I` or from that candidate's
frozen indexed input projection. Thus a standalone grammar or scheme under
`Inputs`, or any repeated source projection of either body, uses the one
canonical `Indices` companion, independent of its copy path. A constructor
whose selected `I` lacks any required component cannot contain that pure body.
Pure `NormalizedSubstitutionV1`, `NormalizedComparisonValueV1`,
`NormalizedLiveDemandValueV1`, and `NormalizedDischargeValueV1` bodies admit no
row because none of the five `CarrierSlot` edge roles occurs in those bodies;
their image/endpoint roles use `StaticExpected`. The table is exhaustive for
`CarrierSlot`; implementation search or reconstruction from a repeated body is
forbidden.

For every such edge, `field_ordinal` is the zero-based ordinal of the selected
leaf in that complete indexed carrier's frozen A3-O `PublicLeaves` flattening,
not a record-local field ordinal or a vector-element-local ordinal. In
particular, scheme trigger leaves, every port argument, and every port derived-
output leaf occupy distinct positions in the one flattened
`DemandSchemeIndex` carrier order; their local port/field coordinates cannot
collide in `CarrierSlot(cindex(p),field_ordinal)`.

Each `Slot` event is filled by the following total dispatch. `p` is its exact
full local path; `StaticExpected(p,role)` is the unique judgment obtained from
the path's nonzero `TypeIdV1`, enclosing carrier index, field/operation ordinal,
and companion normalized type. Its closed cases are: type-valued term
`-> TypeFormation`; typed term `-> HasType(companion type)`; comparison
endpoint `-> DefinitionallyEqualAt(companion type)`; prefix
`-> ContextPrefix(full context,prefix ordinal)`; carrier field
`-> CarrierSlot(full index,field ordinal)`; operation field
`-> OperationOrdinal(grammar path,operation ordinal)`; occurrence
`-> StructuralOccurrence(full occurrence)`; normalization node
`-> NormalizationSubject(rule)`; other typed record
`-> StructuralRecord(type_id)`; and required definition
`-> ConstructionRule(full rule path)`. Exactly one case applies.

| Edge | Exact referent | Exact expected judgment |
| --- | --- | --- |
| `SourceLeaf` | `CurrentOccurrence(q)` or `OlderExport(e,path)` from the complete selected `SourceLeafV1` | `StaticExpected(p,source-role)` |
| `LocalBinderUse` | `Binder(the retained BinderKeyV1)` | `HasType(the complete binder type)` |
| `GlobalUse` | `KernelGlobal(g)` | `HasType(the complete expected type)` |
| `TelescopePrefix` | `RecordSlot(p)` | `ContextPrefix(the complete prefix context,prefix ordinal)` |
| `SectionExpectedType`, `SubstitutionImage`, `ActivationImage`, `ContractType`, `ContractLeft`, `ContractRight`, `DischargeImage` | `RecordSlot(p)` | `StaticExpected(p,the named edge role)` |
| `FamilyIndex`, `GrammarExport`, `SchemeTrigger`, `PortArgument`, `PortDerivedOutput` | `RecordSlot(p)` | `CarrierSlot(cindex(p),the exact field ordinal)` |
| `OperationAnchor` | `RecordSlot(opath)` for the exact selected grammar operation record | `OperationOrdinal(gpath,k)` |
| `OperationParameter` | `OperationSlot(grammar path,operation ordinal,Parameter,field ordinal)` | `OperationOrdinal(the same grammar path and operation ordinal)` |
| `OperationResult` | `OperationSlot(grammar path,operation ordinal,Result,field ordinal)` | `OperationOrdinal(the same grammar path and operation ordinal)` |
| `ObligationPremise` | `RecordSlot(the obligation support_path)` | byte-identical `obligation.expected_judgment` |
| `TheoremEndpoint` | `RecordSlot(p)` | `StaticExpected(p,the endpoint role)` |
| `TermChild(j)` | `RecordSlot(the exact child path)` | `StaticExpected(the child path,j)` |
| `RecordReconstruction(k)` | `RecordSlot(p)` | `StructuralRecord(the reconstructed record TypeIdV1)` |
| `NormalizationStep(r)` | `NormalizationNode(p,r)` | `NormalizationSubject(r)` |
| closure tags `24..26` | no shape event; emitted only by the closure compiler | `StructuralOccurrence(added occurrence)` |
| action tags `27..28` | no candidate shape event; R2d action-root dispatch only | action-owned R2d judgment |

This table determines `expected_referent_kind` from the displayed referent
constructor and stores the complete displayed judgment, never only its tag.
It is exhaustive over all 29 edge variants and all ten A3-C1 expected-judgment
variants; a missing static companion or multiply applicable case is false.

The `OperationAnchor` row has no connection to the candidate's historical
occurrence anchor. For each occurrence, `k` is the checked retained operation
ordinal, `gpath` is the exact `FullLocalPathV1` of its complete companion
`NormalizedGrammarValueV1`, and
`opath = gpath / operations[k]`. Its emitted dependency keeps
`full_local_path=locus`, where `locus=opath` for the grammar declaration and is
the exact scalar field path for a scheme-port selector or live-demand inherited
ordinal. The exact referent is always `RecordSlot(opath)` and the judgment is
always `OperationOrdinal(gpath,k)`. Thus one edge is emitted for every grammar
operation declaration, every scheme-port selector, and every live-demand
inherited-ordinal occurrence; repeats remain repeated raw dependencies.

For every scheme-port or live-demand selector occurrence whose containing
value does not embed its complete companion grammar--including a standalone
`NormalizedSchemeValueV1` under `Inputs` or a standalone
`NormalizedLiveDemandValueV1` under `Output`--`gpath` is the byte-identical
companion grammar under that same built candidate's selected `Indices` root.
For an indexed carrier that embeds its companion grammar, that exact local
grammar path is used. A direct operation declaration always uses its enclosing
grammar path. These cases are exhaustive and no implementation-selected search
path is admitted.
`OperationParameter` and `OperationResult` alone use
`OperationSlot(gpath,k,Parameter|Result,field_ordinal)`. An
`OperationAnchor` owner path is the single `StructuralTerminal` self-hop
`RecordSlot(opath) -> RecordSlot(opath)` with `via=opath`, yielding
`StructuralOwner(opath,RecordSlot(opath))`; selector validation occurs while
constructing the exact referent and is not encoded as a `RecordOrdinal` hop.

The exact edge and referent grammars are:

```text
RawDependencyEdgeV1 ::=
  SourceLeaf=0
| LocalBinderUse=1
| GlobalUse=2
| TelescopePrefix=3
| SectionExpectedType=4
| SubstitutionImage=5
| FamilyIndex=6
| GrammarExport=7
| OperationAnchor=8
| OperationParameter=9
| OperationResult=10
| SchemeTrigger=11
| PortArgument=12
| PortDerivedOutput=13
| ActivationImage=14
| ContractType=15
| ContractLeft=16
| ContractRight=17
| DischargeImage=18
| ObligationPremise=19
| TheoremEndpoint=20
| TermChild=21(jg2b2b0_edge_tag:u8)
| RecordReconstruction=22(ReconstructionTagV1)
| NormalizationStep=23(NormalizationRuleV1)
| ClosureAncestor=24
| ClosureBinderSibling=25
| ClosureSameBirth=26
| ActionIntroduced=27
| ActionArgument=28.

ReconstructionTagV1 ::=
  ExpectedType=0
| CaptureSafeSubstitution=1
| IteratedBinderLift=2
| TelescopeConcatenation=3
| ComparisonReplay=4
| GrammarReindex=5
| SchemeSpecialization=6
| DischargeContractReplay=7

NormalizationRuleV1 ::=
  BetaApply=0
| PermittedGlobal=1(GlobalId)
| FirstPairProjection=2
| SecondPairProjection=3

ParameterOrResultV1 ::= Parameter=0 | Result=1

ExactReferentV1 ::=
  CurrentOccurrence=0(OccurrenceId14V1)
| OlderExport=1(ExportIdV1,PublicFieldPathV1::DeclarationField)
| KernelGlobal=2(GlobalId)
| Binder=3(BinderKeyV1)
| RecordSlot=4(FullLocalPathV1)
| OperationSlot=5(grammar_path:FullLocalPathV1,
                  operation_ordinal:u32,
                  parameter_or_result:ParameterOrResultV1,
                  field_ordinal:u32)
| NormalizationNode=6(FullLocalPathV1,NormalizationRuleV1)
| FormalAction=7(FormalActionIdV1)

ExpectedJudgmentV1 ::=
  TypeFormation=0
| HasType=1(normalized_type:Term)
| DefinitionallyEqualAt=2(normalized_type:Term)
| ContextPrefix=3(normalized_context:NormalizedContextV1,prefix_ordinal:u32)
| CarrierSlot=4(carrier_index:CarrierIndexValueV1,field_ordinal:u32)
| OperationOrdinal=5(grammar_path:FullLocalPathV1,
                      operation_ordinal:u32)
| StructuralOccurrence=6(OccurrenceId14V1)
| NormalizationSubject=7(NormalizationRuleV1)
| StructuralRecord=8(type_id:TypeIdV1)
| ConstructionRule=9(rule_path:DefinitionRulePathV1).
```

The 29 edge variants are exactly the A3-O display, hence tags `0..28`.
`ExpectedJudgmentV1` is the A3-C1-corrected sum with unchanged tags `0..7`
and appended tags `StructuralRecord=8` and `ConstructionRule=9`.
`ContextPrefix` retains the complete normalized context
value; a digest-like context ID or arbitrary bytes cannot supply the judgment.
`StructuralRecord` carries the reached descriptor's exact nonzero type ID;
`ConstructionRule` carries the complete required definition path.

The previously open owner-resolution path is closed here:

```text
CurrentOwnerLocationV1 ::=
  CandidateField=0(EnvelopeFieldPathV1)
| AuxiliaryRoot=1(FullLocalPathV1)

SupportOwnerV1 ::=
  CurrentOwner=0(OccurrenceId14V1,CurrentOwnerLocationV1)
| OlderOwner=1(ExportIdV1,PublicFieldPathV1::DeclarationField)
| StructuralOwner=2(FullLocalPathV1,ExactReferentV1)
| FormalActionOwner=3(FormalActionIdV1)

OwnerResolutionHopKindV1 ::=
  CurrentSource=0
| GlobalBirthDeclaration=1
| GlobalPermittedBody=2
| OlderPublicExport=3
| HistoricalBinder=4
| SyntheticBinderLineage=5
| FieldLineage=6
| RecordOrdinal=7
| StructuralTerminal=8
| FormalTerminal=9

OwnerResolutionHopV1 =
  (hop_ordinal:u32,
   kind:OwnerResolutionHopKindV1,
   from:ExactReferentV1,
   to:ExactReferentV1,
   via:FullLocalPathV1,
   predecessor_hop:Option<u32>)

OwnerResolutionPathV1 =
  (hops:vector<OwnerResolutionHopV1>,
   terminal_owner:SupportOwnerV1)

RawDependencyV1 =
  (raw_ordinal:u32,
   support_root_tag:SupportRootTagV1,
   full_local_path:FullLocalPathV1,
   edge:RawDependencyEdgeV1,
   exact_referent:ExactReferentV1,
   expected_judgment:ExpectedJudgmentV1,
   owner_resolution_path:OwnerResolutionPathV1).
```

An owner-resolution path is nonempty. For a raw dependency `d`, hop 0 has
`from=d.exact_referent` and no predecessor; its `via` is the exact
transition-specific witness path in the table below. The source locus remains
separately and byte-identically recorded as `d.full_local_path`. For
`j>0`, `predecessor_hop=Some(j-1)` and `hops[j-1].to=hops[j].from` by full
bytes; no branch or skipped predecessor is legal. Hop ordinals are contiguous.
The final pair is exact:

```text
CurrentOwner(q,location)  <-> final.to=CurrentOccurrence(q)
OlderOwner(e,path)        <-> final.to=OlderExport(e,path)
StructuralOwner(path,r)   <-> final.to=r and final.via=path
FormalActionOwner(a)      <-> final.to=FormalAction(a).
```

The final hop kind is respectively `CurrentSource`, `OlderPublicExport`,
`StructuralTerminal`, or `FormalTerminal`, and `terminal_owner` is
byte-identical to `owner_map[d.raw_ordinal].owner`.
Each hop is the unique replay of the fixed Decode/Build/normalization DAG. The
closed transition table is:

| Hop kind | Required `from -> to` transition and `via` source |
| --- | --- |
| `CurrentSource` | `CurrentOccurrence(q) -> CurrentOccurrence(q)`; at hop 0, `via=d.full_local_path`; after another hop, `via` is byte-identical to the preceding hop's exact target-witness path |
| `GlobalBirthDeclaration` | `KernelGlobal(g) -> CurrentOccurrence(q)` where the owned census and birth declaration map `g` to the unique same-birth declaration occurrence `q`; `via` is the originating raw dependency's `d.full_local_path` |
| `GlobalPermittedBody` | `NormalizationNode(p,PermittedGlobal(g)) -> CurrentOccurrence(q_body)` where `Sigma(H,o)` exposes the unique same-birth body occurrence; `via=p`, which is byte-identical to the originating normalization-node local path. The ordinary `GlobalUse` record separately resolves the declaration occurrence; older bodies are forbidden |
| `OlderPublicExport` | either `KernelGlobal(g) -> OlderExport(e,p)` by the unique `Pub_<b(H)>` declaration export, with `via` equal to the originating raw dependency's local path, or terminal `OlderExport(e,p) -> OlderExport(e,p)`; the terminal branch uses `via=d.full_local_path` at hop 0 and otherwise copies the preceding hop's `via` byte-for-byte |
| `HistoricalBinder` | `Binder(HistoricalBinder(parent,edge)) -> CurrentOccurrence(q_enter(parent,edge))`, where owned JG2b2b0 replay uniquely derives the binder-entering occurrence from both `parent` and `edge`; `via=d.full_local_path` byte-for-byte |
| `SyntheticBinderLineage` | `Binder(SyntheticBinder(v,c,p)) -> RecordSlot(l)` where `l` is the unique earlier raw-field lineage slot selected by `(v,c,p)` and `via=l` byte-for-byte |
| `FieldLineage` | `RecordSlot(l) -> CurrentOccurrence(q)` or `RecordSlot(l) -> OlderExport(e,p)` according to the exact `SourceLeafV1` selected at `l`; `via` is the full local path of that exact lineage entry |
| `RecordOrdinal` | a `RecordSlot` or `OperationSlot` maps to the unique exact referent selected by its declared field/operation ordinal and typed path; `via` is that child path |
| `StructuralTerminal` | `r -> r` for `r` equal to `RecordSlot`, `OperationSlot`, or `NormalizationNode`, iff this edge-indexed referent has no admissible underlying `FieldLineage`, `RecordOrdinal`, or `GlobalPermittedBody` transition; `via` is the same structural path. Every `OperationAnchor` referent is in this structural-only class |
| `FormalTerminal` | `FormalAction(a) -> FormalAction(a)` at the selected formal-action path; candidate use is invalid |

No other pair of referent tags is admitted. Every semantic lookup in the table
must return exactly one full byte value; zero or multiple results is false at a
candidate locus. The guards are disjoint: an exact lineage-entry slot requires
`FieldLineage`; another declared child selected by its typed ordinal requires
`RecordOrdinal`; and a `NormalizationNode(p,PermittedGlobal(g))` for which the
normalization event exists requires `GlobalPermittedBody`. Any such admissible
route forbids `StructuralTerminal`. Conversely, a structural record,
operation-anchor, or normalization endpoint with none of those underlying
routes requires `StructuralTerminal`; an implementation may not choose the
self-hop merely because its referent tag admits one.

The current-owner location is also derived, not selected. For a direct record
whose emitted `full_local_path` is inside one particular public field's
`FieldLineageV1` traversal, it is
`CandidateField(that exact EnvelopeFieldPathV1)`. Every other direct current
owner uses `AuxiliaryRoot(d.full_local_path)`. Every closure-suffix record uses
`AuxiliaryRoot(inherited_path)` even when the forcing record came from a public
field. The terminal owner's location must equal this function byte-for-byte. A
candidate cannot contain `FormalTerminal`, `FormalActionOwner`, `FormalAction`,
`ExpectedReferentKindV1::FormalAction`, `ActionIntroduced`, or
`ActionArgument`.

### 7.2 Owner and irreducible-key closure

```text
SupportKeyV1 ::=
  CurrentSupport=0(OccurrenceId14V1,
                   CurrentOwnerLocationV1,
                   ExpectedJudgmentV1)
| OlderSupport=1(ExportIdV1,
                 PublicFieldPathV1::DeclarationField,
                 ExpectedJudgmentV1)
| StructuralSupport=2(FullLocalPathV1,
                      ExactReferentV1,
                      ExpectedJudgmentV1)
| FormalActionSupport=3(FormalActionIdV1,ExpectedJudgmentV1).
```

Owner variants and support-key variants correspond tag-for-tag. Candidate
validation rejects tag 3 in both sums. Two dependencies share a class exactly
when their complete `SupportKeyV1` bytes agree. Class order is lexicographic
over the full tagged key bytes. No digest, normalized-term equality, or owner
erasure participates.

For every raw ordinal `i`, the irreducible key is constructed from both the
resolved owner and the raw dependency's full expected judgment:

```text
key(CurrentOwner(q,location), J) = CurrentSupport(q,location,J)
key(OlderOwner(e,path), J)       = OlderSupport(e,path,J)
key(StructuralOwner(path,r), J)  = StructuralSupport(path,r,J)
key(FormalActionOwner(a), J)     = FormalActionSupport(a,J).
```

Current/global/binder/record resolution is exactly A3-O section 4.2. Concrete
globals first check under `Sigma(H,o)` and resolve to a same-birth exact
occurrence or the unique older `DeclarationField`; no `ExternalGlobal` exists.
Historical and synthetic binders follow their exact retained keys. Structural
slots terminate as evidence-only owners. The lexicographic well-founded
measure is `(birth ordinal, declaration ordinal, carrier-prefix ordinal,
derivation-DAG height)`.

### 7.3 Two-pass coverage and pure projection

```text
ExpectedReferentKindV1 ::=
  CurrentOccurrence=0
| OlderExport=1
| KernelGlobal=2
| Binder=3
| RecordSlot=4
| OperationSlot=5
| NormalizationNode=6
| FormalAction=7

SupportShapeSlotV1 =
  (slot_ordinal:u32,
   support_root_tag:SupportRootTagV1,
   full_local_path:FullLocalPathV1,
   edge:RawDependencyEdgeV1,
   expected_referent_kind:ExpectedReferentKindV1,
   expected_judgment:ExpectedJudgmentV1)

ResolvedDirectSlotV1 =
  (slot_ordinal:u32,raw_ordinal:u32)

SupportShapeRootCountV1 =
  (support_root_tag:SupportRootTagV1,
   node_count:u64,
   shape_slot_count:u64)

SupportRootCountV1 =
  (support_root_tag:SupportRootTagV1,
   node_count:u64,
   shape_slot_count:u64,
   resolved_slot_count:u64)

SupportClosureWorkItemV1 =
  (work_ordinal:u32,
   forcing_raw_ordinal:u32,
   emitted_raw_ordinal:u32,
   added_occurrence:OccurrenceId14V1,
   closure_edge:RawDependencyEdgeV1)

SupportOwnerMapEntryV1 =
  (raw_ordinal:u32,owner:SupportOwnerV1)

CanonicalBitVectorV1 =
  (bit_count:u64,
   byte_count:u64,
   packed_lsb_first:Bytes)

SupportCoverageV1 =
  (shape_slots:vector<SupportShapeSlotV1>,
   resolved_direct_slots:vector<ResolvedDirectSlotV1>,
   root_counts:vector<SupportRootCountV1>,
   raw_dependencies:vector<RawDependencyV1>,
   closure_worklist:vector<SupportClosureWorkItemV1>,
   owner_map:vector<SupportOwnerMapEntryV1>,
   support_classes:vector<SupportKeyV1>,
   raw_to_class:vector<u32>,
   all_resolved:CanonicalBitVectorV1,
   shape_cardinality:R3::StructuralCardinalityReceiptV1,
   resolution_cardinality:R3::StructuralCardinalityReceiptV1,
   resource_account:R3::EnvelopeResourceAccountV1)

SupportProjectionV1 =
  (keys:vector<SupportKeyV1>)

SupportFootprintAgreementV1 =
  (current_projection:ExactOccurrenceFootprintV1,
   older_projection:ExactOlderPublicFootprintV1,
   output_projection:ExactOccurrenceFootprintV1).
```

The shape pass traverses all seven candidate roots and emits exactly the
direct dependency slots, with contiguous `slot_ordinal`.
`SupportShapeTraversalV1.root_counts` uses `SupportShapeRootCountV1` and has
exactly tags `0..7`; the candidate `ActionArgument` row has both counts zero.
The final `SupportCoverageV1.root_counts` uses `SupportRootCountV1`, repeats the
shape-pass node/shape counts, and adds the completed per-root resolved counts;
its tag-7 row has all three counts zero. The resolution pass emits one raw
dependency per shape slot in the same
prefix order and records the identity map in `resolved_direct_slots`. It then appends every
forced `Cl_b` record in canonical full-occurrence byte order, with ancestor,
binder-sibling, and same-birth edge order. No other raw record is legal.

For every shape slot `s`, `s.support_root_tag=s.full_local_path.root`. For every
direct ordinal `i`, `resolved_direct_slots[i]=(i,i)` and raw dependency `i`
repeats `s[i]`'s root, full path, edge, and expected judgment byte-for-byte;
its exact-referent variant tag equals `s[i].expected_referent_kind`.
Consequently the direct prefix length equals `shape_slots.len`, and there is no
resolution reorder or omitted shape slot.

The three stage values are byte-bound, not merely extensionally related. For
input `x`, shape result `S`, owner-resolution result `R`, and closure result
`O`, validation requires

```text
S.input                         = x
R.shape                         = S
O.support.constructor           = S.input.public_leaves.seed.constructor
O.support.roots                 = S.roots
O.support.coverage.shape_slots  = S.shape_slots
O.support.coverage.shape_cardinality
                                = S.cardinality
O.support.coverage.resolution_cardinality
                                = R.resolution_cardinality
O.support.coverage.resolved_direct_slots
                                = [(i,i) | 0 <= i < d]
prefix_d(O.support.coverage.raw_dependencies)
                                = R.direct_dependencies
prefix_d(O.support.coverage.owner_map)
                                = R.direct_owner_map,
where d = S.shape_slots.len.
```

All equalities are full canonical-byte equalities. `R.direct_dependencies` and
`R.direct_owner_map` both have length `d`; their ordinals are `0..d-1`. The
shape-cardinality receipt certifies the exact `Visit` and `Slot` totals of the
shape event stream. The resolution-cardinality receipt certifies exactly `d`
attempted and `d` successful direct resolutions in slot order. For root tag
`t`, final `root_counts[t].node_count` and `shape_slot_count` equal the two
fields of `S.root_counts[t]`, while `resolved_slot_count` is exactly the number
of direct slots with tag `t`. Neither closure additions nor deduplicated
classes alter these direct per-root counts.

The two support resource accounts have disjoint, exact scopes under section
1.1's specification functions. For the same `(S,R,O)` above:

```text
CoverageEvents(S,R,O) =
  ShapeVisitAndSlotEvents(S)
  ++ DirectOwnerResolutionEvents(R)
  ++ ClosureWorklistEvents(O.support.coverage.closure_worklist)
  ++ OwnerClassSortAndMapEvents(O.support.coverage)
  ++ CoverageCardinalityAndBitVectorEvents(O.support.coverage)

SupportOuterEvents(O.support) =
  ProjectionConstructionEvents(O.support.projection)
  ++ FootprintAgreementEvents(O.support.footprint_agreement)
  ++ SupportPreservationBindingEvents(
       O.support.preservation_subjects)
  ++ OuterSupportReconstructionEvents(O.support)

O.support.coverage.resource_account =
  Acct_R3(SupportCoverage,CoverageEvents(S,R,O))

O.support.resource_account =
  Aggregate_R3(SupportTotal,[
    O.support.coverage.resource_account,
    Acct_R3(SupportOuter,SupportOuterEvents(O.support))
  ],[]).
```

The outer reconstruction stream excludes traversal and encoding of the nested
coverage bytes already charged by `CoverageEvents` and excludes the outer
`resource_account` field itself; the coverage account occurs exactly once in
the aggregate. Every displayed `*Events` function ignores every nested
`EnvelopeResourceAccountV1` field and no account under construction is ever an
event input. Conversely, projection, footprint-agreement, and preservation-
subject binding events occur only in the outer stream. R3 must close the
account schemas and counter conservation with these exact scope tags and event
orders; it may not merge, duplicate, omit, or caller-size either stream.

`SupportPreservationBindingEvents` charges only the byte-equality binding of
the normalization row to the already constructed
`normalization.support_projection_subject`; it constructs and charges the
admissible-renaming and reindexing rows. It therefore does not recharge the
normalization subject construction already present in `NormalizationEvents`.

Let `d=shape_slots.len`. The closure suffix length is
`raw_dependencies.len-d=closure_worklist.len`. Work item `j` has
`work_ordinal=j` and `emitted_raw_ordinal=d+j`; its forcing record is any
already resolved record related by one exact `Cl_b` edge under the frozen
semantic well-founded measure, not necessarily a smaller raw ordinal. The
emitted raw record inherits the forcing record's
root and full local path, has exact referent
`CurrentOccurrence(added_occurrence)`, expected judgment
`StructuralOccurrence(added_occurrence)`, and one of the edges
`ClosureAncestor`, `ClosureBinderSibling`, or `ClosureSameBirth`. Its terminal
owner is `CurrentOwner(added_occurrence,AuxiliaryRoot(inherited_path))` and its
owner path satisfies the linear rules above. Its edge is byte-identical to the
work item's `closure_edge`.

Let the terminal owner of the forcing record be
`CurrentOwner(q_force,_)`. The work item is valid exactly when the owned
history replay establishes one immediate
`ClStep_b(q_force,closure_edge,added_occurrence)`: the named retained ancestor
for `ClosureAncestor`, the named binder sibling for `ClosureBinderSibling`, or
the named same-birth companion for `ClosureSameBirth`. A forcing record with a
non-current terminal owner cannot emit a closure item. Taking the least fixed
point of these one-step events is exactly `Cl_b`; a transitive jump, reversed
edge, or edge label that disagrees with the history relation is invalid.

All possible additions are first collected under the lexicographic semantic
measure and emitted by full added-occurrence bytes. An occurrence already
present in the direct prefix or an earlier closure item emits no duplicate. If
multiple witnesses force the same addition, the least pair
`(forcing_raw_ordinal,closure_edge tag)` is retained. These equations, not
work-queue insertion order, fix the complete suffix.

`raw_dependencies[i].raw_ordinal=i`. `owner_map[i].raw_ordinal=i`.
`owner_map`, `raw_to_class`, and the checked `u64` bit count of `all_resolved`
have exactly the raw-dependency length. `byte_count=ceil(bit_count/8)` and is
also the exact nested `Bytes` length; bit `i` is the little-endian bit within
byte `i/8`; every in-range bit is one and every unused high bit of the final
byte is zero. This is byte-for-byte the R2a/R2b universal bit-vector codec.
Class indices are in bounds. For `s:OrdinarySupportV1`,
`s.coverage.support_classes` is sorted, duplicate-free, and exactly the set of
`key(s.coverage.owner_map[i].owner,
s.coverage.raw_dependencies[i].expected_judgment)` values; it equals
`s.projection.keys` byte-for-byte.
For every `i`,
`s.coverage.support_classes[s.coverage.raw_to_class[i]] =
 key(s.coverage.owner_map[i].owner,
     s.coverage.raw_dependencies[i].expected_judgment)` by full bytes.

The complete support record is:

```text
SupportPreservationSubjectsV1 =
  (normalization:SupportProjectionSubjectV1,
   admissible_renaming:SupportProjectionSubjectV1,
   reindexing:SupportProjectionSubjectV1)

OrdinarySupportV1 =
  (constructor:ConstructorTagV1,
   subject_root_projection:SubjectRootProjectionV1,
   principal_lineage:PrincipalRootLineageV1,
   field_lineages:vector<FieldLineageV1>,
   roots:vector<SupportRootV1>,
   coverage:SupportCoverageV1,
   projection:SupportProjectionV1,
   footprint_agreement:SupportFootprintAgreementV1,
   preservation_subjects:SupportPreservationSubjectsV1,
   resource_account:R3::EnvelopeResourceAccountV1).
```

The subject projection, principal lineage, and field lineages are
byte-identical to the pre-evidence values. The preservation kinds are
respectively tags 0, 1, and 2; the first subject is byte-identical to
`normalization.support_projection_subject`. They are propositions, not proofs.

Candidate acceptance requires:

```text
footprint_agreement.current_projection = implementation
footprint_agreement.older_projection   = prior_support
footprint_agreement.output_projection  = current_output.
```

The left sides are recomputed from the paired
`(raw_dependencies[i],owner_map[i])` vectors, not copied from fields 5--6.
`current_projection` erases every `CurrentOwner(q,location)` in the complete
`owner_map`, then sorts and deduplicates the exact `q` bytes; it does not filter
by support root or location. `older_projection` analogously erases every
`OlderOwner(e,path)` in the complete map and sorts/deduplicates the full
`(e,path)` bytes. Consequently an occurrence/export owner introduced through
an obligation or subject root is visible to these equalities rather than
silently discarded.

`output_projection` is recomputed independently of global support-suffix
deduplication. The exact `output_lineage` raw-field source paths are traversed
by JG2b2b0 to obtain `OutputUse_c(r)`; the result is
`sort_unique(Cl_b(OutputUse_c(r)))`. It is not obtained by filtering direct
dependencies and then attaching whichever global closure records happen to
inherit that root. Structural and formal owners remain in `projection.keys`
but in neither history projection.

All three displayed equalities are over full sorted vectors. No support record
asserts a paid clause, prior live output, public provenance, or theorem fact.

## 8. Complete ordinary field dispositions

The common A1 two-way sum is represented even though the ordinary profile
admits only its primitive branch:

```text
LowerPublicBoundaryV1 =
  (field_path:EnvelopeFieldPathV1,
   earlier_fields:vector<NormalizedPublicFieldV1>)

ReplacementRecipeV1 =
  (rule_ordinal:u32,
   replacement_term:Term)

AllowedDependencyEvidenceV1 =
  (replacement_support:SupportProjectionSeedV1,
   allowed_support:SupportProjectionSeedV1,
   inclusion_subject:SubjectSyntaxV1)

ReplacementEvidenceV1 =
  (field:NormalizedPublicFieldV1,
   lower_public_boundary:LowerPublicBoundaryV1,
   recipe:ReplacementRecipeV1,
   typing_replays:vector<KernelSuccessReplayV1>,
   boundary_equations:vector<KernelSuccessReplayV1>,
   semantic_preservation_subject:SubjectSyntaxV1,
   allowed_dependencies:AllowedDependencyEvidenceV1,
   substitution_stability_subject:SubjectSyntaxV1,
   substitution_stability_replays:vector<KernelSuccessReplayV1>,
   public_normalization_compatibility:SubjectSyntaxV1)

OrdinaryEmptyReplacementTranscriptV1 =
  (replacement_grammar_version:u16=1,
   profile_manifest_id:ProfileManifestIdV1,
   rule_count:u64=0,
   attempted_rule_ordinals:vector<u32>,
   search_completed:Bool,
   empty_enumeration_cardinality:R3::StructuralCardinalityReceiptV1)

CompleteNoReplacementCertificateV1 =
  (field:NormalizedPublicFieldV1,
   lower_public_boundary:LowerPublicBoundaryV1,
   transcript:OrdinaryEmptyReplacementTranscriptV1,
   completeness_rule:OrdinaryProfileRuleV1,
   resource_account:R3::EnvelopeResourceAccountV1)

PublicFieldDispositionV1 ::=
  Derived=0(ReplacementEvidenceV1)
| Primitive=1(CompleteNoReplacementCertificateV1)

PublicFieldDispositionEntryV1 =
  (field:NormalizedPublicFieldV1,
   disposition:PublicFieldDispositionV1)

PublicFieldDispositionsV1 =
  (entries:vector<PublicFieldDispositionEntryV1>).
```

The ordinary replacement grammar has exactly zero rules. Therefore a checked
ordinary candidate rejects every `Derived` value: no `rule_ordinal` can be in
bounds. For every `Primitive` certificate, `rule_count=0`,
`attempted_rule_ordinals` is empty, and `search_completed=True`; the profile
identity is the candidate's exact profile identity. `completeness_rule`
is exactly zero-based `OrdinaryProfileRuleV1` tag 20,
`OrdinaryReplacementGrammarIsExactlyEmpty`, and is bound to that candidate's
full `ProfileManifestIdV1`. `DefinitionRulePathV1` is reserved for paths in the
operation-definition vectors and cannot select a profile-definition rule.

`entries` has exactly the same length and order as `Fields_c(b)`. Both copies
of each full `NormalizedPublicFieldV1` are byte-identical. The lower boundary
is exactly the earlier public fields whose dependent prefix types the selected
field, in original field order; it is not every lexicographically earlier
path. Exhaustion, allocation failure, an incomplete search, or absence of the
R3 resource receipt constructs no certificate and no candidate.

The exact construction transcript is:

```text
FieldDispositionConstructionTranscriptV1 =
  (fields:vector<NormalizedPublicFieldV1>,
   dispositions:PublicFieldDispositionsV1,
   empty_searches:vector<OrdinaryEmptyReplacementTranscriptV1>,
   typing_replays:vector<KernelSuccessReplayV1>,
   resource_account:R3::EnvelopeResourceAccountV1).
```

For field-disposition input `x` and output `(D,T)`, the transcript equations are
exactly

```text
T.fields             = x.public_leaves.fields = Fields_c
T.dispositions       = D
T.empty_searches[i]  = D.entries[i].Primitive.transcript
T.empty_searches.len = D.entries.len
T.typing_replays     = [].
```

All entries are `Primitive`, so the projection in the third equation is total,
and both vectors follow field order. The already checked
`NormalizedPublicFieldV1` judgments are not rechecked by the zero-rule search;
therefore no typing call is made and the replay vector is canonically empty.
Put `C_i=D.entries[i].Primitive`. The leaf and aggregate account equations are
exactly

```text
EmptyReplacementEvents(i,C_i) =
  ZeroRuleEnumerationEvents(C_i.transcript)
  ++ EmptyEnumerationCardinalityEvents(
       C_i.transcript.empty_enumeration_cardinality)
  ++ PrimitiveCertificateReconstructionEvents(i,C_i)

C_i.resource_account =
  Acct_R3(EmptyReplacementField(i),
          EmptyReplacementEvents(i,C_i))

DispositionTranscriptEvents(T) =
  FieldAndDispositionBindingEvents(T.fields,T.dispositions)
  ++ EmptySearchProjectionBindingEvents(T.empty_searches)
  ++ EmptyTypingReplayVectorEvents(T.typing_replays)
  ++ DispositionTranscriptReconstructionEvents(T)

T.resource_account =
  Aggregate_R3(FieldDispositionTotal,
    [C_i.resource_account | 0 <= i < D.entries.len],
    DispositionTranscriptEvents(T)).
```

The child-account vector is in field order. Transcript events validate the
byte-identical projected searches and vector/cardinality structure but never
recharge a zero-rule enumeration or its receipt. Both reconstruction streams
exclude the resource-account field under construction. This transcript is
part of root `0xfc`; field 15 retains only the byte-identical typed disposition
vector.

## 9. A1 fields 1--17 and roots `0xfb`/`0xfc`

### 9.1 Exact field map and core root

```text
EnvelopeCoreV1 =
  (versions:EnvelopeVersionBindingsV1,                         // A1  1
   parents:EnvelopeParentBindingsV1,                           // A1  2
   owned_history:OwnedHistoryAndEvaluationV1,                  // A1  3
   anchor:AnchorIdentityV1,                                    // A1  4
   implementation:ExactOccurrenceFootprintV1,                 // A1  5
   prior_and_current:PriorAndCurrentOutputFootprintsV1,        // A1  6
   replay_boundaries:ReplayBoundariesAndContextsV1,            // A1  7
   constructor_subject:ConstructorSubjectBindingV1,             // A1  8
   indexed_inputs:IndexedInputsV1,                             // A1  9
   indexed_output:IndexedOutputV1,                             // A1 10
   raw_interface_and_implementation:RawInterfaceAndImplementationV1,
                                                                  // A1 11
   obligations:DerivedObligationsV1,                           // A1 12
   normalization:EnvelopeNormalizationEvidenceV1,              // A1 13
   structural_support:OrdinarySupportV1,                       // A1 14
   field_dispositions:PublicFieldDispositionsV1,               // A1 15
   subjects:EnvelopeSubjectsV1).                               // A1 16
```

Root `0xfb` follows the frozen R2a universal rooted-object wire: byte `0xfb`,
little-endian `u16=1`, then these sixteen fields in order. The root-local schema
version is distinct from the protocol/profile/component versions retained in
field 1. `EnvelopeCoreV1` is the unrooted local type of those sixteen fields;
nested use omits the root byte and root-local version word.

The field map is exact:

| A1 field | R2c type and invariant |
| ---: | --- |
| 1 | exact component/parent versions and internal constructor tag |
| 2 | six reminted parent identities in A1 order, owned through `K` |
| 3 | full-history owner binding plus the complete `Sigma(H,o)` source census |
| 4 | exact and normalized anchor identities; normalized is the exact R1 projection |
| 5 | `Implementation_c(r)` |
| 6 | `PriorSupport_c(r)`, then `CurrentOutput_c(r)` |
| 7 | exact current/export birth boundaries, strict prefixes, opaque signature, and dual context endpoints |
| 8 | constructor tag plus checked-built owner reference carrying the complete `CheckedBuiltV1` identity |
| 9 | separately tagged indices, then nominal inputs |
| 10 | complete indexed nominal output |
| 11 | raw code, complete checked decoded payload, and raw realization |
| 12 | eight ordered obligation groups |
| 13 | complete normalization evidence and aggregate account |
| 14 | complete two-pass support and field lineage |
| 15 | one empty-grammar primitive disposition per public field |
| 16 | full theorem DAG and byte-identical pre-support endpoint projection |

Every repeated tag/value across fields 3--16 is compared by full canonical
bytes. The core contains no field 17, exact-evidence root wrapper, quotient
key, outcome, coverage, action image, or pair disposition.

### 9.2 Exact-evidence root

The R2c construction-only records are:

```text
FootprintClosurePredecessorV1 =
  (forcing_occurrence:OccurrenceId14V1,
   predecessor_occurrence:OccurrenceId14V1,
   edge:RawDependencyEdgeV1)

PublicLeavesConstructionTranscriptV1 =
  (public_leaves_and_lineage:PublicLeavesAndLineageV1,
   closure_predecessors:vector<FootprintClosurePredecessorV1>,
   boundary_kernel_replays:vector<KernelSuccessReplayV1>,
   resource_account:R3::EnvelopeResourceAccountV1)

ObligationConstructionTranscriptV1 =
  (obligations:DerivedObligationsV1,
   premise_source_paths:vector<ObligationSourcePathV1>,
   kernel_replays:vector<KernelSuccessReplayV1>,
   substitution_replays:
     vector<VerifiedParticularOpenTypedSubstitutionV1>,
   telescope_comparisons:vector<NormalizedTelescopeComparisonV1>,
   resource_account:R3::EnvelopeResourceAccountV1)

EnvelopeAssemblyTranscriptV1 =
  (core:EnvelopeCoreV1,
   checked_field_count:R3::StructuralCardinalityReceiptV1,
   checker_receipts:vector<R3::KernelSuccessReceiptV1>,
   resource_account:R3::EnvelopeResourceAccountV1)

ExactEvidenceTranscriptV1 =
  (seed:CandidateAssemblySeedV1,
   decode_evidence:DecodeEvidenceV1,
   build_evidence:BuildPremiseEvidenceV1,
   public_leaves:PublicLeavesConstructionTranscriptV1,
   obligations:ObligationConstructionTranscriptV1,
   normalization:EnvelopeNormalizationEvidenceV1,
   support:OrdinarySupportV1,
   dispositions:FieldDispositionConstructionTranscriptV1,
   subjects:SubjectConstructionTranscriptV1,
   assembly:EnvelopeAssemblyTranscriptV1,
   aggregate_resource_account:R3::EnvelopeResourceAccountV1).
```

`FootprintClosurePredecessorV1` retains every `Cl_b` edge used for footprint
construction, not only support-closure edges. Every component repeated from
fields 11--16 is byte-identical. `decode_evidence` and `build_evidence` are the
exact values in the checked built record. Checker/resource receipt vectors
are exhaustive in deterministic pipeline/call order and are reconciled with
the one aggregate account; R3 closes their internal schemas and conservation
rules without moving any field above.

`closure_predecessors` is the exact concatenation of two independently
compiled sections: first the `implementation` closure from
`sort_unique({anchor} union Use_b(raw_code))`, then the `current_output` closure
from `sort_unique(OutputUse_c(raw_code))`. In each section, seeds use full
occurrence-byte order; nonseed additions are emitted in full added-occurrence
byte order. A row for addition `q_add` is
`(q_force,q_add,edge)` for the immediate
`ClStep_b(q_force,edge,q_add)` witness with least
`(q_force full bytes,edge tag)` among all witnesses in that section. Seeds emit
no row. There is no omitted or extra addition and no duplicate within a
section; the same row is retained twice if independently used by both
sections. Thus section order and cross-section multiplicity are canonical.

`boundary_kernel_replays` is the schema-preorder vector of every complete
`KernelSuccessReplayV1` nested in
`public_leaves_and_lineage.replay_boundaries`, stopping descent at each such
replay. No other replay is present. The transcript's
`public_leaves_and_lineage` is byte-identical to the slot-4 success payload.

`assembly.core` is byte-identical to A1 fields 1--16 in their displayed order,
and `checked_field_count` is the R3 structural-cardinality receipt for exactly
16 visited and 16 successfully mapped fields. Assembly performs only typed
byte-equality, canonical encoding, and commitment operations over already
checked stage values; it makes no new kernel call. Consequently
`assembly.checker_receipts=[]` exactly. Its resource account covers those
structural/encoding operations and is included once in the aggregate account.

The obligation transcript's source paths are the concatenation of every
`Projection.sources` vector in obligation order. Its kernel, substitution, and
telescope vectors are exactly the successful checks required by those rows,
in the same obligation order and then their internal left-to-right order; a
descriptor with no call contributes no entry. Thus field 12 remains
receipt-free while root `0xfc` retains exhaustive construction replay.

For a successful assembly pair `(E,X)`, the remaining stage accounts and the
one exact-evidence aggregate obey this closed hierarchy:

```text
PublicLeavesConstructionEvents(X.public_leaves) =
  PublicFieldProjectionEvents(
    X.public_leaves.public_leaves_and_lineage.fields)
  ++ FieldAndOutputLineageEvents(
       X.public_leaves.public_leaves_and_lineage)
  ++ FootprintSeedUseProjectionAndClosureEvents(
       X.public_leaves.public_leaves_and_lineage,
       X.public_leaves.closure_predecessors)
  ++ BoundaryKernelCallEvents(
       X.public_leaves.boundary_kernel_replays)
  ++ SubjectRootProjectionConstructionEvents(
       X.public_leaves.public_leaves_and_lineage.subject_root_projection)
  ++ PublicLeavesTranscriptReconstructionEvents(X.public_leaves)

X.public_leaves.resource_account =
  Acct_R3(PublicLeavesConstruction,
          PublicLeavesConstructionEvents(X.public_leaves))

ObligationConstructionEvents(X.obligations) =
  ObligationPredicateCompilationEvents(X.obligations.obligations)
  ++ ObligationSourcePathBindingEvents(
       X.obligations.premise_source_paths)
  ++ ObligationKernelCallEvents(X.obligations.kernel_replays)
  ++ ObligationSubstitutionCallEvents(
       X.obligations.substitution_replays)
  ++ ObligationTelescopeComparisonEvents(
       X.obligations.telescope_comparisons)
  ++ ObligationTranscriptReconstructionEvents(X.obligations)

X.obligations.resource_account =
  Acct_R3(ObligationConstruction,
          ObligationConstructionEvents(X.obligations))

CoreCanonicalEncodingEvents(C) =
  CanonicalEncodingEvents(
    root=0xfb,root_schema_version=u16(1),payload=C.fields_1_through_16)

EnvelopeAssemblyEvents(X) =
  CoreFieldMappingEvents(X.assembly.core)
  ++ CoreFieldCardinalityEvents(X.assembly.checked_field_count)
  ++ CoreCanonicalEncodingEvents(X.assembly.core)
  ++ ExactEvidenceCommitmentReservationEvents(
       ExactEvidenceCommitmentDescriptor(
         domain="law-v2/jg2b2b2a/ordinary-envelope-exact-evidence/v1",
         canonical_encoded_length=
           CanonicalEncodedLengthFromTypedShape(root=0xfc,value=X)))
  ++ AssemblyTranscriptReconstructionEvents(X.assembly)

X.assembly.resource_account =
  Acct_R3(EnvelopeAssembly,EnvelopeAssemblyEvents(X))

ExactEvidenceAggregateEvents(X) =
  SeedDecodeBuildBindingEvents(
    X.seed,X.decode_evidence,X.build_evidence)
  ++ ExactEvidenceTranscriptReconstructionEvents(X)

X.aggregate_resource_account =
  Aggregate_R3(ExactEvidenceTotal,[
    X.public_leaves.resource_account,
    X.obligations.resource_account,
    X.normalization.resource_account,
    X.support.resource_account,
    X.dispositions.resource_account,
    X.subjects.resource_account,
    X.assembly.resource_account
  ],ExactEvidenceAggregateEvents(X)).
```

The seven child accounts occur exactly once and in displayed exact-evidence
component order.
The support-coverage child is already included once in the support total, and
the per-field empty-replacement children are already included once in the
disposition total, so neither appears again here. Decode/build replay execution
belongs to the earlier frozen R2b stages; the aggregate-only stream charges
only their exact byte binding into this transcript. In contrast, the boundary
replays are R2c `ProjectPreEvidence` calls and are executed and charged exactly
once by `BoundaryKernelCallEvents`. The footprint stream includes construction
of the implementation/current-output seed and use projections, every immediate
closure addition, and the two final sort/dedup projections; it is not merely a
walk over retained closure rows. The subject-root stream charges the descriptor-
only source-bundle and endpoint-program projection compiled after the exact
obligation value is available. An event-census compiler treats every nested account as an opaque
finalized byte span: it may use the span's schema-derived encoded length but
never descends into its counters or makes an event depend on their values. The
actual canonical encoders still include every account byte.
`ExactEvidenceCommitmentReservationEvents` reserves exactly one future domain-
separated commitment call from the displayed domain and canonical encoded
length. `CanonicalEncodedLengthFromTypedShape` observes only type tags, vector/
byte-string lengths, the leading `0xfc,u16=1` root/version header, and the
R3-closed encoded lengths of account fields; it
does not inspect an account value or a digest output. After the seven-child
aggregate is fixed, section 9.3 executes that one call over the complete `X`
bytes and R3 requires its actual domain, input length, and meter delta to equal
the reservation before `E` can be returned. Thus the account is constructed
before field 17, while the eventual commitment still hashes every account byte
without a value-level cycle.

Root `0xfc` encodes byte `0xfc`, little-endian `u16=1`, then the displayed
transcript fields. The version word is the universal rooted-object header, not
a field of nested `ExactEvidenceTranscriptV1`. Root `0xfc` does not contain A1
field 17, an `EnvelopeCoreBytesV1`, an `EvidenceCommitmentV1`, its own digest,
or another exact-evidence transcript.
The typed `assembly.core` is permitted because it is exactly fields 1--16 and
contains no field 17.

### 9.3 Nonrecursive field 17

```text
EnvelopeCoreBytesV1 = semantic newtype over Bytes
EvidenceCommitmentV1 = semantic newtype over Digest

ExactEvidenceRefV1 =
  (owner_local_index:u32,
   full_identity:EvidenceCommitmentV1)

BytesAndCommitmentV1 =
  (envelope_core_bytes:EnvelopeCoreBytesV1,
   evidence_commitment:EvidenceCommitmentV1).
```

For core `C` and exact transcript `X`:

```text
EnvelopeCoreBytesV1(C)
  = encode(0xfb || u16(1) || C.fields_1_through_16)

EvidenceCommitmentV1(X)
  = Digest::of_domain_bytes(
      "law-v2/jg2b2b2a/ordinary-envelope-exact-evidence/v1",
      encode(0xfc || u16(1) || X.fields)).
```

The bytes value includes its canonical `u64` byte-length prefix when nested as
a field, but the bytes it contains begin with exactly `0xfb`. The digest uses
the exact A4-O domain and includes the canonical root-`0xfc` bytes as payload.
Full bytes are compared and decoded before any digest check is used as an
identity shortcut.

`ExactEvidenceRefV1` is the exact R2a `OwnedRef` specialization with owner
`PairCandidateArena`, object type `ExactEvidenceTranscriptV1`, and identity
type `EvidenceCommitmentV1`. The owner resolves the commitment to one full
root-`0xfc` transcript, re-encodes it, and recomputes the commitment before the
reference is usable. The full transcript remains owned. Within one enclosing
`(profile,H,o,c)` pair evaluation, exact-evidence objects and checked-candidate
objects use the same order: ascending successful raw ordinal, with the
owner-local index equal to the zero-based rank in that filtered order (not the
possibly gapped raw ordinal). The complete pair-outcome stage in R2d is the
only lawful constructor of either rank-bearing reference; R2c per-leaf matching
is deliberately rank-free.

The full A1 value is a separate seventeen-field record:

```text
RawCapabilityEnvelopeV1 =
  (versions:EnvelopeVersionBindingsV1,
   parents:EnvelopeParentBindingsV1,
   owned_history:OwnedHistoryAndEvaluationV1,
   anchor:AnchorIdentityV1,
   implementation:ExactOccurrenceFootprintV1,
   prior_and_current:PriorAndCurrentOutputFootprintsV1,
   replay_boundaries:ReplayBoundariesAndContextsV1,
   constructor_subject:ConstructorSubjectBindingV1,
   indexed_inputs:IndexedInputsV1,
   indexed_output:IndexedOutputV1,
   raw_interface_and_implementation:RawInterfaceAndImplementationV1,
   obligations:DerivedObligationsV1,
   normalization:EnvelopeNormalizationEvidenceV1,
   structural_support:OrdinarySupportV1,
   field_dispositions:PublicFieldDispositionsV1,
   subjects:EnvelopeSubjectsV1,
   bytes_and_commitment:BytesAndCommitmentV1).
```

Fields 1--16 are byte-identical to one `EnvelopeCoreV1 C`. Field 17 is exactly
the pair derived above. `RawCapabilityEnvelopeV1` has no top-level root and no
hidden eighteenth field. Neither root transcript contains field 17, so the
construction is well founded.

The corrected R2a root-containment inventory includes `0xfb`: field 7 directly
contains R3-qualified kernel success receipts; fields 8 and 11 contain them
transitively through checked-built/decode evidence; fields 13--15 directly
retain normalization, support, disposition, cardinality, and resource
receipts; and field 16 contains them transitively through the checked raw
carrier values inside `CanonicalSchemaPayloadV1`. Field 12's projection-only
obligation descriptors add no receipt bytes. This is an outer-position
inventory corrigendum only; it changes neither the R2a schema metalanguage nor
an R2b field.

## 10. Checked candidate and future root-`0xe3` tag 3

```text
RankFreeCandidateDeriveReplayV1 =
  (raw_ordinal:u32,
   raw_code:RawConstructorCodeV1,
   checked_built:CheckedBuiltRefV1,
   core:EnvelopeCoreV1,
   exact_evidence_identity:EvidenceCommitmentV1)

AnchorMatchEvidenceV1 =
  (principal_lineage:PrincipalRootLineageV1,
   normalized_origin_candidates:vector<OccurrenceId14V1>,
   selected_anchor:OccurrenceId14V1,
   current_output:ExactOccurrenceFootprintV1,
   selector_subject_slots:vector<u16>)

CompatibilityMatchEvidenceV1 =
  (enumerated_raw_code:RawConstructorCodeV1,
   retained_raw_code:RawConstructorCodeV1,
   derived_implementation:ExactOccurrenceFootprintV1,
   retained_implementation:ExactOccurrenceFootprintV1,
   derived_prior_and_current:PriorAndCurrentOutputFootprintsV1,
   retained_prior_and_current:PriorAndCurrentOutputFootprintsV1,
   derived_replay_boundaries:ReplayBoundariesAndContextsV1,
   retained_replay_boundaries:ReplayBoundariesAndContextsV1)

RankFreeCandidateMatchEvidenceV1 =
  (derive:RankFreeCandidateDeriveReplayV1,
   anchor:AnchorMatchEvidenceV1,
   compatibility:CompatibilityMatchEvidenceV1)

RankFreeCheckedCandidateEnvelopeV1 =
  (envelope:RawCapabilityEnvelopeV1,
   exact_evidence:ExactEvidenceTranscriptV1,
   match_evidence:RankFreeCandidateMatchEvidenceV1)

CandidateDeriveReplayV1 =
  (raw_ordinal:u32,
   raw_code:RawConstructorCodeV1,
   checked_built:CheckedBuiltRefV1,
   core:EnvelopeCoreV1,
   exact_evidence:ExactEvidenceRefV1)

CandidateMatchEvidenceV1 =
  (derive:CandidateDeriveReplayV1,
   anchor:AnchorMatchEvidenceV1,
   compatibility:CompatibilityMatchEvidenceV1)

CheckedCandidateEnvelopeV1 =
  (envelope:RawCapabilityEnvelopeV1,
   exact_evidence:ExactEvidenceTranscriptV1,
   match_evidence:CandidateMatchEvidenceV1).
```

For one candidate-match input `assembly=(E,X)`, put `s=X.seed`, and write
`M_0` for the successful rank-free match evidence. The derive record is exact
replay data, not a proof token. Its complete R2c source equations are:

```text
M_0.derive.raw_ordinal = s.raw_ordinal
M_0.derive.raw_code    = s.raw_code
M_0.derive.checked_built = s.checked_built

M_0.derive.core = X.assembly.core
              = (E.versions,
                 E.parents,
                 E.owned_history,
                 E.anchor,
                 E.implementation,
                 E.prior_and_current,
                 E.replay_boundaries,
                 E.constructor_subject,
                 E.indexed_inputs,
                 E.indexed_output,
                 E.raw_interface_and_implementation,
                 E.obligations,
                 E.normalization,
                 E.structural_support,
                 E.field_dispositions,
                 E.subjects)

M_0.derive.exact_evidence_identity = EvidenceCommitmentV1(X)
E.bytes_and_commitment.envelope_core_bytes
  = EnvelopeCoreBytesV1(X.assembly.core)
E.bytes_and_commitment.evidence_commitment
  = EvidenceCommitmentV1(X).
```

The rank-free identity in the last group is recomputed from the full `X`; a
caller digest is insufficient. It is not an `OwnedRef` and cannot be used to
resolve evidence. The anchor evidence is fixed by:

```text
M_0.anchor.principal_lineage
  = X.public_leaves.public_leaves_and_lineage
      .subject_root_projection.source_bundle.principal_lineage
  = E.subjects.root_projection.source_bundle.principal_lineage

M_0.anchor.normalized_origin_candidates
  = [M_0.anchor.principal_lineage.pre_expansion_occurrence]
M_0.anchor.selected_anchor
  = M_0.anchor.principal_lineage.pre_expansion_occurrence
  = M_0.anchor.principal_lineage.exact_anchor
  = E.anchor.exact
  = s.anchor
M_0.anchor.current_output = E.prior_and_current.current_output
M_0.anchor.selector_subject_slots = [2,3,4,5,6].
```

Thus the normalized-origin vector is the complete singleton provenance set at
the frozen principal position, not an arbitrary singleton. Its member is the
principal leaf's current occurrence and belongs to the independently derived
`current_output` footprint.

Let `ReplayBoundariesAndContexts_c(H,o,r)` mean a fresh execution of the exact
section-4 boundary/source-replay compiler: it derives the implementation,
prior-support, and current-output source set from `(H,o,r)`, then emits the
target boundary and exactly one selected-source replay per unique selected
source in the enclosing birth-source-census order. Compatibility evidence is
source-bound by:

```text
M_0.compatibility.enumerated_raw_code = s.raw_code
M_0.compatibility.retained_raw_code
  = E.raw_interface_and_implementation.raw_code

M_0.compatibility.derived_implementation
  = sort_unique(Cl_b^H({s.anchor} union Use_b^H(s.raw_code)))
M_0.compatibility.retained_implementation = E.implementation

M_0.compatibility.derived_prior_and_current
  = (Old_b^H(s.raw_code),
     sort_unique(Cl_b^H(OutputUse_c^H(s.raw_code))))
M_0.compatibility.retained_prior_and_current = E.prior_and_current

M_0.compatibility.derived_replay_boundaries
  = ReplayBoundariesAndContexts_c(H,s.anchor,s.raw_code)
M_0.compatibility.retained_replay_boundaries = E.replay_boundaries.
```

Here `H` is the complete owner-resolved history named by
`s.history_and_evaluation.history`; every superscript-`H` operation is replayed
against that authority. Candidate acceptance additionally requires each
displayed derived/retained pair, and the two raw codes, to be byte-identical.
Pairwise equality alone is not sufficient: both sides must equal their
independently specified sources above. Missing replay is not represented by
omitting a pair. A successful rank-free match also requires:

```text
Derive_(ord,c)^kappa(H,E)
Anchor_(ord,c)^kappa(H,E,o)
Compat_(ord,c)^kappa(H,o,Implementation_c(r),r,E).
```

These are the deterministic replay predicates over retained data; they are
not generic theorem proofs. The selector stability nodes in field 16 remain
unproved subjects.

The per-leaf R2c operation stores only `RankFreeCheckedCandidateEnvelopeV1`.
It does not yet own a `PairCandidateArena`, and therefore cannot construct an
`ExactEvidenceRefV1`, `CandidateMatchEvidenceV1`, or
`CheckedCandidateEnvelopeV1`. Those final schema types are nevertheless closed
here so that R2d may only fill, never redesign, the reserved Level-II payload.
After `CheckedCandidateEnvelopeV1` is defined, the final DAG places:

```text
CandidateEnvelopeRefV1 =
  (owner_local_index:u32,
   full_identity:RawCapabilityEnvelopeV1)
```

as the exact `OwnedRef(PairCandidateArena,CheckedCandidateEnvelopeV1,
RawCapabilityEnvelopeV1)` specialization. The referenced object must have a
byte-identical `envelope` and an exact-evidence transcript whose recomputed
commitment equals field 17. Candidate arena order is ascending raw ordinal.
The encoded owner-local index is the successful-raw-ordinal rank fixed in
section 9.3.

The schema reserved by R2c for the already frozen Level-II tag 3 is:

```text
CheckedCandidateEnvelopePayloadV1 =
  (profile_binding:EnvelopeProfileRefV1,             // kappa
   history_binding:EnvelopeHistoryRefV1,             // H
   anchor:OccurrenceId14V1,                          // o
   evaluation_binding:EvaluationBindingRefV1,        // Sigma(H,o)
   constructor:ConstructorTagV1,                     // c
   raw_code:RawConstructorCodeV1,                    // r
   envelope:CandidateEnvelopeRefV1,                  // E
   match_evidence:CandidateMatchEvidenceV1).          // Match evidence

CandidateMatchOutputV1 =
  (candidate:CheckedCandidateEnvelopeV1,
   level_two_payload:CheckedCandidateEnvelopePayloadV1).
```

This is exactly the eight semantic components frozen by R2b, in that order.
All repeated profile/history/evaluation/anchor/constructor/raw/envelope values
agree with the referenced candidate by full bytes. `match_evidence` is
byte-identical to the `match_evidence` of the owner-resolved
`CheckedCandidateEnvelopeV1` selected by `envelope`. Both fields of the final
`CandidateMatchOutputV1` are the candidate and payload from that same match
event. Nested under future root
`0xe3`, the payload is encoded directly after variant tag 3; no candidate root
or version header is inserted. R2d must use this type unchanged when it closes
the five-way Level-II sum.

R2d first projects the receipt-free quotient key directly from each successful
rank-free match; that projection contains neither reference below. R2d
materializes this final pair only while finalizing its complete per-raw outcome
pass after those keys exist. The pass is indexed by every raw ordinal in the
frozen A3 generation for the fixed `(profile,H,o,c)` pair, including a checked-
false terminal from whichever earlier pipeline operation first rejected the
raw leaf; any abort suppresses the entire pair. Let the successful rank-free
`CandidateMatch` leaves in that complete pass be ordered by raw ordinal and let
`rho(j)` be the number of preceding successful rank-free `CandidateMatch`
leaves. For rank-free match `(E,X,M_0)` at leaf `j`, the only lawful conversion
is:

```text
M.derive.raw_ordinal          = M_0.derive.raw_ordinal
M.derive.raw_code             = M_0.derive.raw_code
M.derive.checked_built        = M_0.derive.checked_built
M.derive.core                 = M_0.derive.core
M.derive.exact_evidence       =
  ExactEvidenceRefV1(rho(j), M_0.derive.exact_evidence_identity)
M.anchor                      = M_0.anchor
M.compatibility               = M_0.compatibility

Q.envelope                    = E
Q.exact_evidence              = X
Q.match_evidence              = M

L.profile_binding             = X.seed.profile
L.history_binding             = X.seed.history_and_evaluation.history
L.anchor                      = X.seed.anchor
L.evaluation_binding          = X.seed.history_and_evaluation.evaluation_binding
L.constructor                 = X.seed.constructor
L.raw_code                    = X.seed.raw_code
L.envelope                    = CandidateEnvelopeRefV1(rho(j), E)
L.match_evidence              = M

resolve_PairCandidateArena(M.derive.exact_evidence) = X
resolve_PairCandidateArena(L.envelope) = Q.
```

The `PairCandidateArena` owns exactly these `X` and `Q` values in the same
ascending-match-leaf order. Thus both references share `rho(j)`. No success-
only caller vector, ambient insertion order, claimed prefix count, or digest-
only lookup may choose the rank. This is a strict R2d construction obligation,
not authority for R2c to anticipate coverage.

No checked candidate is a pair disposition or an occurrence classification.
It is one successful raw-code leaf before quotienting and complete coverage.

## 11. R2c operation definitions for protocol slots 4--8

### 11.1 Runtime inputs, outputs, and partitions

```text
PreEvidenceProjectionInputV1 =
  (seed:CandidateAssemblySeedV1)

NormalizationInputV1 =
  (public_leaves:PublicLeavesAndLineageV1)

NormalizationOutputV1 =
  (normalization:EnvelopeNormalizationEvidenceV1)

SupportShapeInputV1 =
  (public_leaves:PublicLeavesAndLineageV1,
   normalization:EnvelopeNormalizationEvidenceV1)

SupportShapeTraversalV1 =
  (input:SupportShapeInputV1,
   roots:vector<SupportRootV1>,
   shape_slots:vector<SupportShapeSlotV1>,
   root_counts:vector<SupportShapeRootCountV1>,
   cardinality:R3::StructuralCardinalityReceiptV1)

SupportOwnerResolutionInputV1 =
  (shape:SupportShapeTraversalV1)

SupportOwnerResolutionV1 =
  (shape:SupportShapeTraversalV1,
   direct_dependencies:vector<RawDependencyV1>,
   direct_owner_map:vector<SupportOwnerMapEntryV1>,
   resolution_cardinality:R3::StructuralCardinalityReceiptV1)

SupportClosureInputV1 =
  (resolved:SupportOwnerResolutionV1)

SupportClosureOutputV1 =
  (support:OrdinarySupportV1)

FieldDispositionInputV1 =
  (public_leaves:PublicLeavesAndLineageV1,
   normalization:EnvelopeNormalizationEvidenceV1,
   support:OrdinarySupportV1)

FieldDispositionOutputV1 =
  (dispositions:PublicFieldDispositionsV1,
   transcript:FieldDispositionConstructionTranscriptV1)

SubjectConstructionInputV1 =
  (public_leaves:PublicLeavesAndLineageV1,
   normalization:EnvelopeNormalizationEvidenceV1,
   support:OrdinarySupportV1,
   dispositions:PublicFieldDispositionsV1)

SubjectConstructionOutputV1 =
  (subjects:EnvelopeSubjectsV1,
   transcript:SubjectConstructionTranscriptV1)

EnvelopeAssemblyInputV1 =
  (public_leaves:PublicLeavesAndLineageV1,
   normalization:EnvelopeNormalizationEvidenceV1,
   support:OrdinarySupportV1,
   dispositions:FieldDispositionConstructionTranscriptV1,
   subjects:SubjectConstructionOutputV1)

EnvelopeAssemblyOutputV1 =
  (envelope:RawCapabilityEnvelopeV1,
   exact_evidence:ExactEvidenceTranscriptV1)

CandidateMatchInputV1 =
  (assembly:EnvelopeAssemblyOutputV1)

RankFreeCandidateMatchOutputV1 =
  (candidate:RankFreeCheckedCandidateEnvelopeV1).
```

Every success payload repeats the exact previous-stage value it consumes only
where displayed. The nine operation results use the closed partitions:

```text
PreEvidenceProjectionResultV1 ::=
  Success=0(PublicLeavesAndLineageV1)
| Abort=1(R3::AbortReceiptV1)

NormalizationResultV1 ::=
  Success=0(NormalizationOutputV1)
| Abort=1(R3::AbortReceiptV1)

SupportShapeResultV1 ::=
  Success=0(SupportShapeTraversalV1)
| Abort=1(R3::AbortReceiptV1)

SupportOwnerResolutionResultV1 ::=
  Success=0(SupportOwnerResolutionV1)
| False=1(R3::CheckedFalseReceiptV1)
| Abort=2(R3::AbortReceiptV1)

SupportClosureResultV1 ::=
  Success=0(SupportClosureOutputV1)
| False=1(R3::CheckedFalseReceiptV1)
| Abort=2(R3::AbortReceiptV1)

FieldDispositionResultV1 ::=
  Success=0(FieldDispositionOutputV1)
| False=1(R3::CheckedFalseReceiptV1)
| Abort=2(R3::AbortReceiptV1)

SubjectConstructionResultV1 ::=
  Success=0(SubjectConstructionOutputV1)
| False=1(R3::CheckedFalseReceiptV1)
| Abort=2(R3::AbortReceiptV1)

EnvelopeAssemblyResultV1 ::=
  Success=0(EnvelopeAssemblyOutputV1)
| Abort=1(R3::AbortReceiptV1)

CandidateMatchResultV1 ::=
  Success=0(RankFreeCandidateMatchOutputV1)
| False=1(R3::CheckedFalseReceiptV1)
| Abort=2(R3::AbortReceiptV1).
```

### 11.2 Exact cross-stage equations

Success never licenses an operation to replace its input by another internally
consistent candidate. For `x.seed=s`, put `c=s.constructor`, `r=s.raw_code`,
`o=s.anchor`, `B=s.checked_built.full_identity.built`, and let `H` be the
complete owner-resolved history named by `s.history_and_evaluation.history`.
The successful `ProjectPreEvidence(x)=P` equations are exactly:

```text
P.seed                = s
P.fields              = Fields_c(B)
P.field_lineage       = FieldLineage_c(H,s)
P.output_lineage      = OutputLineage_c(H,s)
P.implementation      = sort_unique(Cl_b^H({o} union Use_b^H(r)))
P.prior_and_current   = (Old_b^H(r),
                         sort_unique(Cl_b^H(OutputUse_c^H(r))))
P.replay_boundaries   = ReplayBoundariesAndContexts_c(H,o,r)
P.obligations         = CompileObligations_c(s)
P.subject_root_projection
  = CompileSubjectRootProjection_c(
      P.seed,P.fields,P.field_lineage,P.output_lineage,
      P.implementation,P.prior_and_current,P.replay_boundaries,P.obligations).
```

The named compilers are precisely the finite algorithms and canonical orders
of sections 3--5; they are notation, not additional wire operations. In the
last equation the source bundle uses the eight already fixed preceding
components of `P`, so it is not a recursive caller input. In particular,
`P.seed=x.seed` is a full-byte equality: the `Build[0]` predecessor cannot be
ignored in favor of a different checked-built value.

For any such `P`, define the semantic constructor `RawEndpoint(P)` by the
following complete field equations:

```text
RawEndpoint(P).seed       = P.seed
RawEndpoint(P).versions   = Versions(P.seed.profile,
                                     P.seed.parents,c)
RawEndpoint(P).anchor     = (o, R1NormalizedOccurrenceKey(H,o))
RawEndpoint(P).constructor_subject = (c,P.seed.checked_built)
RawEndpoint(P).indexed_inputs
  = (ProjectIndices(B),ProjectNominalInputs(B))
RawEndpoint(P).indexed_output = ProjectIndexedOutput(B)
RawEndpoint(P).raw_interface_and_implementation
  = (r,P.seed.checked_built.full_identity.decoded,
       ProjectRawRealization(B))
RawEndpoint(P).fields                  = P.fields
RawEndpoint(P).field_lineages          = P.field_lineage
RawEndpoint(P).output_lineage          = P.output_lineage
RawEndpoint(P).implementation          = P.implementation
RawEndpoint(P).prior_and_current       = P.prior_and_current
RawEndpoint(P).replay_boundaries       = P.replay_boundaries
RawEndpoint(P).obligations             = P.obligations
RawEndpoint(P).subject_root_projection = P.subject_root_projection.
```

`Versions` and `R1NormalizedOccurrenceKey` are the unique projections fixed in
sections 2 and 6 and by frozen R1; the four `Project*` terms are the selected
tag-preserving section-3 projections. A successful
`NormalizePreEvidence((P))=(N)` satisfies

```text
N.normalization.raw_endpoint = RawEndpoint(P)
```

and every other field of `N.normalization` is the unique section-6 compilation
from that endpoint. It cannot normalize an unrelated public-leaves value.

The three support calls retain the already displayed section-7 equations. In
operation notation they begin with the additional exact input bindings:

```text
EnumerateSupportShape((P,N.normalization)) = S
  => S.input = (P,N.normalization)
ResolveSupportOwners((S)) = R          => R.shape = S
CloseSupportAndCoverage((R)) = O       =>
  O.support.subject_root_projection = P.subject_root_projection
  O.support.principal_lineage
    = P.subject_root_projection.source_bundle.principal_lineage
  O.support.field_lineages = P.field_lineage
```

Here `P` and `N` in the closure row are recovered without ambiguity from
`R.shape.input`; all remaining roots, direct-prefix, owner, closure, coverage,
projection, footprint, preservation-subject, and resource fields obey the
exhaustive equations in section 7. In every downstream call,
`N.normalization.raw_endpoint=RawEndpoint(P)` and every repeated support field
must agree with `P` by full bytes.

For `CertifyPrimitiveFields((P,N.normalization,O.support))=(D,T_D)`, the section-8 equations
hold with `x.public_leaves=P`, `x.normalization=N.normalization`, and
`x.support=O.support`; in particular `T_D.dispositions=D` and its field vector
is exactly `P.fields`. For
`ConstructSubjects((P,N.normalization,O.support,D))=(U,T_U)`, the complete bindings are:

```text
U.root_projection       = P.subject_root_projection
T_U.emitted_projection  = P.subject_root_projection
T_U                     = CompileSubjectTranscript_c(
                            P.subject_root_projection)
U.syntax                = CompileSubjectSyntax_c(
                            P.subject_root_projection).
```

The two `CompileSubject*` terms are exactly the closed slot/binder/node compiler
and transcript equations of section 5.3. The call additionally checks the
normalization, support, and disposition input equalities just stated; those
predecessors cannot be replaced or ignored merely because the compiled syntax
is determined by `P`.

For assembly input `(P,N.normalization,O.support,T_D,(U,T_U))`, define the unrooted core
`Core(P,N,O,T_D,U)` to have these sixteen fields in order:

```text
(N.normalization.normalized_endpoint.versions,
 P.seed.parents,
 P.seed.history_and_evaluation,
 N.normalization.normalized_endpoint.anchor,
 P.implementation,
 P.prior_and_current,
 P.replay_boundaries,
 RawEndpoint(P).constructor_subject,
 RawEndpoint(P).indexed_inputs,
 RawEndpoint(P).indexed_output,
 RawEndpoint(P).raw_interface_and_implementation,
 P.obligations,
 N.normalization,
 O.support,
 T_D.dispositions,
 U).
```

A successful `AssembleEnvelope` output `(E,X)` is bound by:

```text
X.seed                                      = P.seed
X.decode_evidence = P.seed.checked_built.full_identity.decoded.evidence
X.build_evidence  = P.seed.checked_built.full_identity.evidence
X.public_leaves.public_leaves_and_lineage   = P
X.obligations.obligations                   = P.obligations
X.normalization                             = N.normalization
X.support                                   = O.support
X.dispositions                              = T_D
X.subjects                                  = T_U
X.assembly.core                             = Core(P,N,O,T_D,U)

(E.fields_1_through_16)                     = X.assembly.core
E.bytes_and_commitment.envelope_core_bytes  = EnvelopeCoreBytesV1(X.assembly.core)
E.bytes_and_commitment.evidence_commitment  = EvidenceCommitmentV1(X).
```

The remaining construction-transcript fields and accounts are exactly the
section-9.2 compilers from these bound values. Assembly makes no replacement
kernel call and introduces no alternate evidence source.

Finally, for `CheckCandidateMatch((E,X))=(Q_0)`, let `M_0` be the unique
section-10 rank-free match evidence. Success requires:

```text
Q_0.envelope       = E
Q_0.exact_evidence = X
Q_0.match_evidence = M_0.
```

Thus the sole field of `RankFreeCandidateMatchOutputV1`, including its envelope
and full exact-evidence transcript, is byte-bound to the input assembly. It
contains no `PairCandidateArena` owner-local index, candidate reference, or
final Level-II payload. The section-10 R2d conversion equations are the only
route from this output to those rank-bearing values.

A profile-disallowed owner, subject, disposition, or match predicate is
`False`; parent/history drift,
deterministic reconstruction drift, incomplete traversal, arithmetic or
allocation failure, resource exhaustion, canonical encoding failure, or an
internal checked-value mismatch is `Abort`. No result stores platform text.
R3 closes the exact reason and receipt payloads. Any abort later suppresses the
whole pair disposition.

### 11.3 Static contribution

```text
R2cSemanticOpcodeV1 ::=
  ProjectPreEvidence=0
| NormalizePreEvidence=1
| EnumerateSupportShape=2
| ResolveSupportOwners=3
| CloseSupportAndCoverage=4
| CertifyPrimitiveFields=5
| ConstructSubjects=6
| AssembleEnvelope=7
| CheckCandidateMatch=8

R2cOperationDefinitionV1 =
  (slot_local_ordinal:u32,
   opcode:R2cSemanticOpcodeV1,
   input_type:TypeIdV1,
   output_type:TypeIdV1,
   predecessor_rules:vector<DefinitionRulePathV1>)

R2cOperationSlotContributionV1 =
  (protocol_slot:ProtocolSlotV1,
   definitions:vector<R2cOperationDefinitionV1>)

R2cOperationDefinitionContributionV1 =
  (slots:vector<R2cOperationSlotContributionV1>).
```

The outer vector has exactly slots `4,5,6,7,8` in that order. Local ordinals
are contiguous. The complete map is:

| Protocol slot | Local | Opcode | Input | Output | Exact predecessors |
| --- | ---: | --- | --- | --- | --- |
| `PublicLeavesAndLineage=4` | 0 | `ProjectPreEvidence` | `PreEvidenceProjectionInputV1` | `PreEvidenceProjectionResultV1` | `Build[0]` |
| `Normalization=5` | 0 | `NormalizePreEvidence` | `NormalizationInputV1` | `NormalizationResultV1` | `PublicLeavesAndLineage[0]` |
| `SupportAndOwnerResolution=6` | 0 | `EnumerateSupportShape` | `SupportShapeInputV1` | `SupportShapeResultV1` | `PublicLeavesAndLineage[0]`, `Normalization[0]` |
| `SupportAndOwnerResolution=6` | 1 | `ResolveSupportOwners` | `SupportOwnerResolutionInputV1` | `SupportOwnerResolutionResultV1` | `SourceResolution[0]`, `SupportAndOwnerResolution[0]` |
| `SupportAndOwnerResolution=6` | 2 | `CloseSupportAndCoverage` | `SupportClosureInputV1` | `SupportClosureResultV1` | `SupportAndOwnerResolution[1]` |
| `FieldDisposition=7` | 0 | `CertifyPrimitiveFields` | `FieldDispositionInputV1` | `FieldDispositionResultV1` | `PublicLeavesAndLineage[0]`, `Normalization[0]`, `SupportAndOwnerResolution[2]` |
| `SubjectConstruction=8` | 0 | `ConstructSubjects` | `SubjectConstructionInputV1` | `SubjectConstructionResultV1` | slots `4[0]`, `5[0]`, `6[2]`, `7[0]` |
| `SubjectConstruction=8` | 1 | `AssembleEnvelope` | `EnvelopeAssemblyInputV1` | `EnvelopeAssemblyResultV1` | `Build[0]`, slots `4[0]`, `5[0]`, `6[2]`, `7[0]`, `8[0]` |
| `SubjectConstruction=8` | 2 | `CheckCandidateMatch` | `CandidateMatchInputV1` | `CandidateMatchResultV1` | `SubjectConstruction[1]` |

Each notation `Slot[n]` is encoded as
`DefinitionRulePathV1(protocol_slot=Slot,
steps=[VectorElement(n)])`. Predecessors appear in the order shown. `Build[0]`
is the frozen R2b `BuildDecoded` definition at slot 3. R2e substitutes exact
nonzero topologically earlier `TypeIdV1` values and wraps each opcode only as
`JoinedSemanticOpcodeV1::R2c`; it may not move, rename, reorder, or add an
edge.

`ProjectPreEvidence` constructs public leaves/lineage, obligations, and the descriptor-
only `SubjectRootProjectionV1`, because support must consume that projection
before the full subject DAG exists. Slot 8 reconstructs the full DAG and
requires projection byte equality; it does not retroactively change support.
`AssembleEnvelope` performs field mapping, exact-evidence projection, and root
encoding/commitment. `CheckCandidateMatch` alone performs `Derive`, `Anchor`,
and `Compat` replay and stores only the rank-free checked candidate. It obtains
the byte-identical slot-4 value from
`assembly.exact_evidence.public_leaves.public_leaves_and_lineage`; no hidden
second input or predecessor is admitted. It does not assign a successful-leaf
rank, construct either `PairCandidateArena` reference, or emit the future tag-3
payload. Neither operation performs raw search, quotient comparison, complete
outcome enumeration, class counting, or publication.

## 12. R2c freeze audit

R2c froze only after two independent audits confirmed all of the following:

1. A1 fields 1--17 occur exactly once and in the frozen order;
2. root `0xfb` is exactly `0xfb,u16=1` followed by fields 1--16 and no field 17;
3. root `0xfc` is exactly `0xfc,u16=1` followed by the complete exact transcript
   fields, but no field 17, evidence commitment, own digest, or recursive
   evidence value;
4. field 17 is exactly root-`0xfb` bytes plus the root-`0xfc` domain-separated
   commitment, while the full evidence remains owned;
5. fields 8--11 are exact tag-agreeing projections of frozen R2b `Built_c`;
6. public leaves and lineages use typed paths, preserve repeats, and follow the
   exact A3-O preorder;
7. implementation, prior-support, and current-output footprints remain three
   distinct full-identity projections;
8. field 7 covers the exact union of all current identities and every older
   export in fields 4--6, including both local-context replay endpoints;
9. normalization retains both endpoints, every step/replay, idempotence,
    presentation, anchor preservation, pure support subject, substitution
    subject, and aggregate account; its material traversal uses the closed four-
    wrapper semantic-projection table and never descends into checked proof or
    receipt metadata;
10. candidate support has roots `0..6`, the exact node-role incidence and 29
    edge variants, all ten A3-C1 expected judgments, one typed owner per raw
    record, the guarded unique owner-resolution path, all grammar/port/live
    `OperationAnchor` occurrences, source-backed-only theorem endpoints, exact
    local-or-canonical-companion carrier indices, and no formal/action
    dependency;
11. the two support passes, closure segment, owner vector, quotient map,
    all-resolved bit vector, class sort, and three history projections are
    exhaustive and mutually consistent;
12. every ordinary public field receives the complete zero-rule `Primitive`
    certificate with its exact leaf account, the transcript aggregates those
    accounts once in field order, and no `Derived` value validates;
13. subject binders/literals are descriptor-only, all node references point
    backward, pure-support requirements are compiled field by field from the
    subject kind and shared seed, arbitrary reindexing actions have no finite
    component census, and forbidden enclosing/key/action/pair values are absent;
14. the pre-support and field-16 subject-root projections are byte-identical;
15. the five single-base schemas have slots `0..7`, the two square schemas
    have slots `0..10`, and endpoint/extra-coherence order is exact;
16. the reserved candidate payload has exactly the eight frozen tag-3 semantic
    fields and every owner reference carries its full typed identity;
17. all success/false/abort results are disjoint, every one of the ten resource-
    account fields is bound by the closed leaf/nested/aggregate hierarchy, the
    exact-evidence total has its seven children once in component order, field-
    17 metering uses the acyclic domain/rooted-length reservation, and all
    R3-qualified slots retain fixed outer positions;
18. slots 4--8 have exactly the nine operation definitions and predecessor
    paths displayed above;
19. every successful operation output is byte-bound to its exact input, the
    candidate-match replay fields are independently derived from the assembly,
    and slot `8[2]` contains no rank-bearing reference or tag-3 payload;
20. every new scalar follows the R2a universal wire, including `u64` span
    counts, `u16` binder depth, and checked conversion into `u32` ordinal
    domains;
21. every local schema reference admits the explicit acyclic order
    `A3-O base -> R2a+C2 -> R2b -> A3-C1 -> R2c+C2`;
22. A3-C1 changes only the two appended expected-judgment tags and every pure
    obligation record emits its required leading support slot;
23. R2d alone may materialize the two shared-rank arena references, final
    checked candidate, and tag-3 payload, and only from its complete per-raw
    outcome pass rather than a success-only vector or ambient arena state;
24. a candidate has no action root, while the R2d reservation permits exactly
    one nonempty tag-7 vector root for a nonidentity flattened action and none
    for identity; and
25. no candidate, pair disposition, occurrence classification, theorem,
    profile-selection, or executable authority is minted by this record.

Both base audits passed the candidate hash recorded above. The 2026-08-03 C2
dependency, typing, count, and route audits also passed; that corrigendum is not
covered by the retained base hash. R2d may treat the resulting layouts as
frozen; changing one now requires an explicit corrigendum and a new audit.

## 13. Continuation and non-authority

With R2c+C2 and R2d QG1--QG2 frozen, the active QG3 cut must close receipt-free
support/disposition mirrors, support-preservation/action subjects, action-
introduced lineage, and the shared candidate/action eight-group shape. Root
`0xfd` remains unmaterialized. The later complete per-raw checked-leaf outcome
pass over the frozen A3 generation may then assign the shared dense successful-
leaf ranks, materialize the final checked candidate and tag-3 payload fixed in
section 10, and create the `PairCandidateArena`. R2d then closes quotient
classes, pair coverage/dispositions, base-relative
action normalization/rebuild, action trace/image roots, the action-image
Level-II payload, and finally the joined root `0xe3`. It must
recompute action support from final roots and may use the action-only variants
reserved here, but it cannot reinterpret a candidate support owner or mutate
roots `0xfb`/`0xfc`. An R2d action subject retains this record's
`SubjectProjectionSourceBundleV1` byte-identically as its base-candidate
environment and denotes the final image only through a formal-action-bound
reindexing node; it may not reinterpret `SourceCandidate` or replace the
bundle's `schema_payload` with final action-image bytes.

R3 then fills the qualified receipt/resource nodes without moving their outer
positions. R2e alone assigns final TypeIds, joins all twelve operation slots,
measures the isolated reference source, reconstructs the registry, and closes
canonical round trips. R4 alone regenerates fixtures and mutation tests.

This record selects no history, occurrence, constructor, raw code, match,
class, action, or result. It mints no verified profile token, executable
indexed-interface authority, envelope census, pair disposition, occurrence
classification, generic law, cubical bridge, `GCap`, `gamma`, or selective
authority.
