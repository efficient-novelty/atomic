# JG2b2b2a-A4-O Ordinary Profile Transcript and Resource Closure

Date: 2026-08-02

Status: **A4-O RELEASE CANDIDATE 1 REJECTED; A4-R1, A4-R2a, A4-R2b, A3-C1,
AND A4-R2c FROZEN; A4-R2d ACTIVE.** Independent
reconstruction verified the two RC1 fixture byte vectors, but the release
audit found unresolved implementation choices in the typed component
transcript, birth-signature/source census, dynamic codecs, failure payloads,
and resource-counting machine. The controlling audit and ordered repair gates
are in `docs/260802_jg2b2b2a_a4_rc1_release_audit.md`. Every use of “freeze” or
“frozen” below describes the rejected RC1 proposal unless the audit explicitly
retains that clause. RC1 is not a selectable profile definition and its
digests cannot identify a verified token.

The rejected candidate is target-neutral. It minted no verified profile token, history,
envelope, action image, pair disposition, occurrence classification, generic
law, cubical bridge, or generative-capacity fact. JG2b2b2b remains blocked
until `R2d -> R3 -> R2e -> R4` closes and independently
regenerates the definition. The two cubical profile contracts remain
open at A2-C and have no authority edge into that ordinary implementation.

This record depends normatively on:

- `docs/260802_jg2b2b2a_adequacy_and_envelope_protocol.md` (A1);
- `docs/260802_jg2b2b2a_indexed_ontology_profiles.md` (A2-O);
- `docs/260802_jg2b2b2a_a3_ordinary_constructor_schemas.md` (A3-O);
- `docs/260801_jg2b2a_substitution_naturality_protocol.md`;
- `docs/260802_jg2b2b0_structural_occurrence_grammar.md`;
- `docs/260802_jg2b2b1_particular_open_typed_substitution.md`; and
- the exact JG1 and JG2a frozen authorities named below.

## 1. Three authority levels and the meaning of selectable

A4-O freezes three distinct types. They must not be collapsed:

```text
OrdinaryDependentEnvelopeProfileDefinitionV1
OrdinaryDependentEnvelopeProfileManifestV1
VerifiedOrdinaryDependentEnvelopeProfileDefinitionV1.
```

The first is the history-free static definition. The second adds exact
upstream identities without containing itself. The third is a future opaque,
remintable token constructed only after a verifier consumes the full opaque
upstream authorities, checks their common dependency graph, reconstructs the
definition and manifest bytes, compares the full bytes, and then checks their
digests. A digest identifies a transcript; it is never evidence that the
transcript or its parents verified.

`selectable` means only that a proposition may name this exact definition
before any history, occurrence, candidate, constructor outcome, Genesis label,
or desired trajectory is inspected. It does not mean executable, theorem-
verified, adequate for a live history, or capable of minting a disposition.

A later, distinct

```text
LawVerifiedOrdinaryDependentEnvelopeProfileV1
```

may exist only after JG2b2b3a proves generic substitution and exact Rust/Agda
correspondence, JG2b2b3b proves the nine functor laws, and JG2b2b3c proves the
seven constructor squares and selector laws. A4-O neither defines a public
constructor for that type nor claims one of those proofs.

The pre-exposure selection function is exact:

```text
SelectProfileV1(OrdinaryDependentDefinitionalComparisonOnly)
  = ordinary-dependent-envelope-v1;

SelectProfileV1(CubicalOpaqueSupportArity)
  = cubical-sealed-trace-v1;                       // A2-C open

SelectProfileV1(CubicalRecentTwoLayerFactorization)
  = cubical-sealed-trace-recent-factorization-v1; // A2-C open.
```

No other claim tag exists in V1. An unavailable selected profile aborts before
history inspection; it is not replaced by the ordinary profile.

## 2. Static typed definition transcript

### 2.1 Exact record fields

After root tag `0xf8`,
`OrdinaryDependentEnvelopeProfileDefinitionV1` encodes these seventeen fields
in this exact order:

1. `definition_schema_version : u16 = 1`;
2. `profile_version : u16 = 1`;
3. `profile_id : text = "ordinary-dependent-envelope-v1"`;
4. `claim_boundary : ClaimBoundaryV1 =
   OrdinaryDependentDefinitionalComparisonOnly`;
5. `profile_state : ProfileStateV1 = DefinitionClosed`;
6. the ordered `ComponentVersionV1` vector;
7. the ordered `ParentSlotV1` vector;
8. the ordered `OrdinaryCarrierTagV1` vector;
9. the ordered `ConstructorBindingV1` vector;
10. the ordered `EnvelopeFieldTagV1` vector;
11. `LevelRelationV1 = RetainedLevelOneInsideCheckedLevelTwo`;
12. `OrdinaryEnvelopeResourcePolicyV1`;
13. `ClosedFailureTaxonomyV1`;
14. the ordered `CodecDomainBindingV1` vector;
15. `CompleteCoverageGrammarV1`;
16. the ordered `OrdinaryProfileRuleV1` vector; and
17. the ordered `UnavailableAuthorityV1` vector.

Every enum discriminant is its zero-based ordinal in the displayed order.
Every vector is encoded with its exact length. There are no ignored fields,
extensions, maps, free strings, caller flags, or implementation-private tags.
Changing a field, tag, order, formula, ceiling, or rule requires a new static
definition and profile version.

The component versions in field 6 are, in order:

| Component | V1 value |
| --- | ---: |
| `EnvelopeProtocol` | 1 |
| `OrdinaryOntology` | 1 |
| `OrdinaryConstructorSchemas` | 1 |
| `LevelRelation` | 1 |
| `CanonicalCodec` | 1 |
| `CoverageGrammar` | 1 |
| `ActionImageGrammar` | 1 |
| `ResourcePolicy` | 1 |

Each component entry encodes its zero-based component tag followed by its
`u16` version.

The A1 parent slots in field 7 are exactly:

```text
JG1, JG2a, JG2b2a, JG2b2b0, JG2b2b1, Profile.
```

The nine carrier tags in field 8 are exactly:

```text
PublicContext, PublicInterface, InterfaceFamily, Substitution,
ComparisonWitness, SealedPublicGrammar, DemandScheme, LiveDemand, Discharge.
```

Each field-9 constructor binding encodes
`(constructor tag, JG1 role tag, action kind, OriginPathV1 tag)`. The exact
vector is:

| Constructor | JG1 role | Action | Principal origin path |
| --- | --- | --- | --- |
| `Formation` | `Formation` | `SubstitutionAction` | `FormationOldestOutputType` |
| `Abstraction` | `Abstraction` | `SubstitutionAction` | `AbstractionOldestFamilyType` |
| `Aggregation` | `Aggregation` | `SubstitutionAction` | `AggregationOldestRawOutputType` |
| `Transport` | `Transport` | `ContextSquareAction` | `TransportOldestRawOutputType` |
| `Comparison` | `Comparison` | `SubstitutionAction` | `ComparisonOldestLeftType` |
| `DemandCompiler` | `Compiler` | `SubstitutionAction` | `DemandCompilerOldestActivationImage` |
| `DischargeTransformer` | `DischargeTransformer` | `ContextSquareAction` | `DischargeTransformerOldestRawOutputImage` |

Constructor, role, action, and origin-path tags are encoded by their respective
declared ordinals. This table is not a classifier result. It binds the seven
A3-O schema definitions in the JG2a/JG1 order.

The seventeen field-10 tags are exactly the A1 field order:

```text
Versions, Parents, OwnedHistory, AnchorIdentity, ImplementationFootprint,
PriorAndCurrentOutputFootprints, ReplayBoundariesAndContexts,
ConstructorSubject, IndexedInputs, IndexedOutput, RawRealization,
DerivedObligations, NormalizationEvidence, StructuralSupportEvidence,
PublicFieldDispositions, ActionAndNaturalitySubjects, BytesAndCommitment.
```

Fields 1--16 of an envelope retain the complete A1/A3 values. Field 17 is
nonrecursive and is closed in section 6.

### 2.2 Exact resource policy

`OrdinaryEnvelopeResourcePolicyV1` begins with root tag `0xfa`, encodes
`resource_policy_version : u16 = 1`, `persistent_ordinal_bits : u16 = 32`,
`cardinality_and_counter_bits : u16 = 64`, and then the following fourteen
`(ResourceCounterV1 tag, u64 ceiling)` entries in this exact order:

| Counter | Ceiling | Exact unit and scope |
| --- | ---: | --- |
| `SourceCensusEntries` | 4,096 | `|U_b| + |Pub_<b(H)|` before one pair search |
| `SyntacticDepth` | 256 | maximum kernel-term, carrier-record, support-path, or theorem-meta-syntax depth |
| `RawSearchNodesPerPair` | 1,048,576 | every visited dependent raw-code prefix node, including rejected prefixes |
| `CompleteRawCodesPerPair` | 262,144 | complete `RawCode_c` leaves evaluated exactly once |
| `MaterialNodesPerEnvelope` | 1,048,576 | every retained scalar/tag/vector slot, term/derivation node, dependency record, obligation, and subject node; repetitions count |
| `MaterialNodesPerPair` | 16,777,216 | search nodes, outcomes, representatives, coverage, and partition records |
| `KernelOperationsPerEnvelope` | 100,000 | the existing `pen-kernel` logical operation unit |
| `NormalizationFuelPerEnvelope` | 50,000 | the existing normalizer fuel unit |
| `KernelOperationsPerPair` | 100,000,000 | sum charged by every build/replay in one `(H,o,c)` search |
| `NormalizationFuelPerPair` | 50,000,000 | analogous aggregate normalization charge |
| `EnvelopeCoreBytes` | 67,108,864 | 64 MiB for canonical A1 fields 1--16 of one candidate or action image |
| `PairTranscriptBytes` | 268,435,456 | 256 MiB for all outcomes, representatives, partition, and coverage evidence for one pair |
| `ProfileManifestBytes` | 1,048,576 | 1 MiB for the static definition or full profile manifest |
| `NormalizedActionSteps` | 4,096 | primitive substitutions or squares in one flattened action |

The bound kernel configuration must be exactly 100,000 operations, depth 256,
and 50,000 normalization-fuel steps and must agree through every parent. A
different configuration requires a different profile definition; a caller
cannot request a larger or smaller theorem domain at evaluation time.

Public fields, support dependencies/classes, closure-worklist steps,
obligation/subject nodes, matches, and quotient classes all charge the material
counters. Matches and classes are also structurally bounded by
`CompleteRawCodesPerPair`. Raw prefix nodes and leaves additionally charge
`MaterialNodesPerPair`. Root tags and length prefixes count toward byte limits.
There is no hidden component-specific limit.

These are engineering ceilings for a bounded partial executable fragment, not
a mathematical adequacy result. In particular, injective ordered telescope
codes grow factorially: the V1 raw-code ceiling can abort apparently small
source censuses. A4-O does not claim that this resource profile is sufficient
for any intended live history. JG2b2b2b must benchmark it pre-exposure; an
outcome-dependent increase is forbidden and any later change requires V2.

The current kernel keeps its operation and normalization counters private to
each public call and does not yet return a usage receipt. Consequently the
first JG2b2b2b implementation obligation is a non-authoritative metered batch
API (or an equivalent shared checked meter) that reports these exact existing
units across one envelope and one pair. The kernel protocol digest and every
dependent parent must then be reminted. Estimating usage, granting each call a
fresh invisible allowance, or treating the number of calls as the operation
count cannot satisfy this V1 profile, and no verified profile token may exist
until the meter is available.

### 2.3 Closed failure taxonomy

`FalseReasonV1` is the following closed tagged sum. The top-level and nested
variants both use their zero-based displayed ordinals:

```text
RawGrammar(
  OutsideSourceCensus | WrongSourceVariant | DuplicateSourceRef |
  LengthOutOfRange | OrdinalOutOfRange | DependentArityMismatch |
  RequiredNonemptyViolation | ConstructorShapeMismatch)

Decode(
  PrefixWeakeningUnavailable | BinderIdentityPrefixMismatch |
  PrefixTypeMismatch | TypeFormationRejected | HasTypeRejected |
  EqualityRejected | DependentShapeMismatch)

Schema(
  OrderedPremiseRejected | ParticularSubstitutionRejected |
  ParticularLiftRejected | RawOutputTypingRejected | RawOutputMismatch |
  ComparisonRejected | EmptyReplacementCertificateMismatch |
  TheoremSubjectTypingRejected)

Support(
  ReferentJudgmentTagMismatch | CurrentOccurrenceNotAdmissible |
  OlderExportNotMember | OlderExportAmbiguous | OlderExportNotPublic |
  PrivateBodyRequested | LaterBirthRequested | FormalDependencyInCandidate |
  OwnerKindMismatch | NonDecreasingDependency | DependencyCycle)

Anchor(
  PrincipalLeafOlderPublic | PrincipalOriginEmpty |
  PrincipalOriginMultiple | AnchorOccurrenceMismatch)

Compatibility(
  RawCodeBindingMismatch | ImplementationProjectionMismatch |
  PriorSupportProjectionMismatch | CurrentOutputProjectionMismatch |
  LocalBoundaryOrBinderMismatch)

Action(
  BaseBindingMismatch | ActionDomainMismatch | ActionCodomainMismatch |
  ActionArityMismatch | ActionTypingRejected | SquareBoundaryMismatch |
  SquareEqualityRejected).
```

`RawGrammar` is available only to a future particular-input decoder; canonical
pair enumeration never generates an ill-shaped raw code. `Action` is available
only before an action image exists. An internally derived checked-action-trace
mismatch is `Abort`, not `False`.

`DefinitionRulePathV1` is a canonical typed ordinal path into fields 6--16 of
the static definition. The first path component names the field, each later
ordinal is range-checked against the selected typed record/vector, and the
terminal node must have the stage named by the reason. It is never caller text.
The false outcome retains the offending raw-field/value path, exact expected
judgment, exact kernel rejection where applicable, and the completed checked
witness. A raw code may return `False` only after the relevant computation
has completed without infrastructure uncertainty.

`AbortReasonV1`, which is outside every coverage certificate and disposition,
has exactly these variants:

```text
OutsideProfileFragment(ProfileBoundaryV1)
ProfileParentMismatch(ProfileParentTagV1)
HistoryParentMismatch(ParentSlotV1)
KernelProtocolMismatch
NormalizerProtocolMismatch
KernelConfigurationMismatch
CompleteHistorySchemaMismatch
CompleteHistoryResourcePolicyMismatch
VerifiedIdentityMissing(DefinitionRulePathV1)
ArithmeticOverflow(Add | Multiply | IndexConversion, DefinitionRulePathV1)
AllocationFailure(ResourceCounterV1, DefinitionRulePathV1)
BudgetExhausted(Operation | Depth | NormalizationFuel | AggregateMaterial |
                RawCodeCount | CanonicalBytes, used, limit)
KernelResourceExhausted(Operations | Depth | Normalization)
NormalizationFailed(PipelineStageV1)
VerifiedParentReplayRejected(PipelineStageV1)
EnumerationCardinalityMismatch(expected,observed)
CoverageMismatch(DefinitionRulePathV1)
CanonicalEncodingMismatch(CodecObjectV1,DefinitionRulePathV1)
QuotientInvariantMismatch(DefinitionRulePathV1)
ActionTraceInvariantMismatch(DefinitionRulePathV1).
```

All subordinate names are closed enums encoded in the static transcript; none
contains platform error text. Unknown or unmapped failure is conservatively
`Abort`, never `False`.

`PipelineStageV1` is, in order, `ProfileRemint`, `HistoryReplay`,
`SourceCensus`, `RawPreflight`, `RawEnumeration`, `Decode`, `Build`, `Fields`,
`Normalization`, `Support`, `Subjects`, `Envelope`, `Anchor`, `Compatibility`,
`Quotient`, `Coverage`, and `Action`. `ProfileParentTagV1` is the five upstream
A1 parent tags followed by `Kernel`, `Normalizer`, and `KernelConfiguration`.
All remaining loci use a validated `DefinitionRulePathV1`; there is no `Other`
or free diagnostic payload.

The complete current `pen-kernel::KernelError` map is fixed:

- `ResourceExhausted(Operations|Depth|Normalization)`, `InvalidLimits`, and
  `UniverseOverflow` map to `Abort`;
- `UnboundVariable`, `InvalidSubstitution`, `SubstitutionArity`,
  `UnknownGlobal`, `ExpectedType`, `ExpectedFunction`, `ExpectedPair`,
  `TypeMismatch`, and `WrongJudgmentForm` map to the enclosing typed-false
  stage only for a candidate-derived judgment; and
- every one of those errors, plus `DuplicateGlobal`, maps to `Abort` when the
  call role is normalization, deterministic reconstruction, or parent/history
  replay. `DuplicateGlobal` is always `Abort`
  because no A3-O raw code can add a global declaration.

Adding a kernel error without versioning this total map prevents profile
reminting.

### 2.4 Complete coverage grammar encoding

Field 15 is the exact record

```text
CompleteCoverageGrammarV1 =
  (coverage_schema_version : u16 = 1,
   raw_ordinal_bits : u16 = 32,
   raw_comparator : StructuralRawOrder,
   outcome_tags : [CheckedFalse,Match],
   class_order : FullQuotientKeyBytesLexicographic,
   member_order : CanonicalRawOrdinalAscending,
   representative_rule : LeastCanonicalRawOrdinal,
   disposition_tags : [CertifiedNoMatch,UniquePositive,TypedAmbiguity],
   abort_is_certificate_member : false).
```

It encodes in displayed field order. Each singleton rule is its tag `0`; each
tag vector has a `u64` count and zero-based `u8` members. `false` is byte `0`.
An abort therefore has no tag in the outcome or disposition vectors.

### 2.5 Ordered semantic rules

`OrdinaryProfileRuleV1` is the following closed 44-rule vector. Each rule
encodes as the `u8` ordinal shown by its position:

1. `ClaimSelectsProfileBeforeHistory`
2. `OrdinaryClaimExcludesCubicalClaims`
3. `DefinitionManifestAndTokenAreNonrecursive`
4. `OpaqueParentsAreEvidenceAndDigestsOnlyIdentify`
5. `CrossParentDependencyGraphIsExact`
6. `SignatureAndHistoryAreLevelTwoInputsOnly`
7. `A1EnvelopeHasExactlySeventeenFields`
8. `A1FieldSeventeenIsNonrecursive`
9. `NineCarriersHaveFrozenOrderAndDefinitions`
10. `SevenConstructorsHaveFrozenOrderAndDefinitions`
11. `ConstructorTagComesOnlyFromInternalIteration`
12. `SourceCensusHasCanonicalOrder`
13. `RawCodeGrammarIsExactlyFiniteAndComplete`
14. `RawCodeEnumerationHasCanonicalOrdinalOrder`
15. `DecodeBuildEvidenceAssemblyIsDeterministic`
16. `ThreeHistoryFootprintsRemainSeparate`
17. `PrincipalSelectorUsesFrozenOriginPath`
18. `SelectorStabilityIsADeferredTheoremSubject`
19. `StructuralSupportTraversalIsComplete`
20. `FormalActionSupportIsEvidenceOnly`
21. `OrdinaryReplacementGrammarIsExactlyEmpty`
22. `WholeEnvelopeQuotientRetainsEveryLoadBearingField`
23. `ResourceAndCheckerReceiptsAreNotQuotientKeys`
24. `ActionImagesAreOpaqueDependentConstructions`
25. `ActionImagesNeverEnterCandidatesOrDispositions`
26. `ActionsFlattenAndRebuildFromTheOriginalBase`
27. `FalseAndAbortAreClosedAndDisjoint`
28. `AnyAbortSuppressesTheWholePairDisposition`
29. `CoverageVisitsEveryRawCodeExactlyOnce`
30. `ClassificationCountsFullQuotientClassesNotRawMatches`
31. `ClassRepresentativeUsesCanonicalRawOrdinal`
32. `CertifiedNoMatchRequiresCompleteAllFalseCoverage`
33. `UniquePositiveRequiresExactlyOneCoveredClass`
34. `TypedAmbiguityRetainsEveryCoveredClass`
35. `EarlyExitAndAmbiguityTieBreaksAreForbidden`
36. `PairAndEnvelopeResourcesAreAggregate`
37. `PreflightArithmeticAndReservationsAreFailClosed`
38. `FullCanonicalBytesAreComparedBeforeDigests`
39. `DigestIdentityNeverReplacesEvidence`
40. `CheckedLevelTwoRetainsCompleteLevelOne`
41. `UniversalLawsRemainSubjectsNotFacts`
42. `CubicalSpecificationLaneIsDisjoint`
43. `ProfileFreezeMintsNoDownstreamAuthority`
44. `AnyByteOrSemanticChangeRequiresVersionChange`

Field 13 encodes, in order, all seven false-family tags with their nested
variant-tag vectors, all twenty abort tags, the seventeen pipeline-stage tags,
the eight profile-parent tags, and a thirteen-row kernel-error table. Each
kernel row contains `(KernelError tag, candidate/action outcome tag,
replay/reconstruction outcome tag)`, with `False=0` and `Abort=1`, exactly as
specified above.

The vector is the typed mutation-sensitive binding to the complete normative
A1, A2-O, and A3-O definitions. It is not a digest of Markdown. Each rule's
meaning is the exact named definition in those frozen records plus the closure
in this record. An implementation that changes such a definition while
retaining the V1 tag fails reminting.

### 2.6 Canonical primitive codec and domains

The codec is the existing `pen-kernel::CanonicalEncoder` grammar, frozen here
as `OrdinaryEnvelopeCanonicalCodecV1`:

- a record or sum root and every enum discriminant is one `u8` tag;
- `u16`, `u32`, and `u64` are little-endian;
- bytes and UTF-8 text are prefixed by a little-endian `u64` byte length;
- a vector is prefixed by a little-endian `u64` element count;
- `None`/`False` is tag `0`, `Some`/`True` is tag `1`;
- a tagged-sum payload follows its tag immediately;
- `Digest` is the length-prefixed lowercase 71-byte ASCII text
  `blake3:` followed by 64 hexadecimal digits, not raw 32 bytes;
- platform `usize`, Serde/JSON encodings, maps, hash iteration order, pointer
  identity, wall time, scheduler order, and platform error strings are
  forbidden; and
- the twelve-form term, declaration, context, judgment, identifier, and digest
  codecs are inherited by exact identity from the bound kernel parent.

Dependent records use declared field order. Set-like records use their frozen
structural comparator, are sorted, and are rejected if duplicate after
canonicalization. No sorting uses a digest.

`CodecDomainBindingV1` is this exact `(object tag, root tag, digest-domain
text)` vector:

| Object | Root | Domain |
| --- | ---: | --- |
| `ProfileDefinition` | `0xf8` | `law-v2/jg2b2b2a/ordinary-profile-definition/v1` |
| `ProfileManifest` | `0xf9` | `law-v2/jg2b2b2a/ordinary-profile-manifest/v1` |
| `ResourcePolicy` | `0xfa` | `law-v2/jg2b2b2a/ordinary-resource-policy/v1` |
| `EnvelopeCore` | `0xfb` | `law-v2/jg2b2b2a/ordinary-envelope-core/v1` |
| `ExactEvidence` | `0xfc` | `law-v2/jg2b2b2a/ordinary-envelope-exact-evidence/v1` |
| `QuotientKey` | `0xfd` | `law-v2/jg2b2b2a/ordinary-envelope-quotient-key/v1` |
| `ActionTrace` | `0xfe` | `law-v2/jg2b2b2a/ordinary-action-trace/v1` |
| `ActionImage` | `0xff` | `law-v2/jg2b2b2a/ordinary-action-image/v1` |
| `PairCoverage` | `0xe8` | `law-v2/jg2b2b2a/ordinary-pair-coverage/v1` |
| `PairDisposition` | `0xe9` | `law-v2/jg2b2b2a/ordinary-pair-disposition/v1` |

BLAKE3 uses exactly `Digest::of_domain_bytes`: prefix
`pen-kernel/domain/v1`, domain length as `u64` little-endian, domain bytes,
payload length as `u64` little-endian, then payload bytes.

The frozen static definition transcript is:

```text
canonical byte length: 1209
domain-separated digest:
blake3:c6ad7291770439b2d1a3055dca862f62680a507f28ce803761777fc823361300
canonical bytes (lowercase hex; concatenate the displayed lines):
f8010001001e000000000000006f7264696e6172792d646570656e64656e742d656e76656c6f70652d76310000080000
000000000000010001010002010003010004010005010006010007010006000000000000000001020304050900000000
000000000102030405060708070000000000000000000000010100010202000203030103040400040505000506060106
1100000000000000000102030405060708090a0b0c0d0e0f1000fa0100200040000e0000000000000000001000000000
000001000100000000000002000010000000000003000004000000000004000010000000000005000000010000000006
a0860100000000000750c30000000000000800e1f505000000000980f0fa02000000000a00000004000000000b000000
10000000000c00001000000000000d001000000000000007000000000000000008000000000000000001020304050607
010700000000000000000102030405060208000000000000000001020304050607030b00000000000000000102030405
060708090a04040000000000000000010203050500000000000000000102030406070000000000000000010203040506
1400000000000000000102030405060708090a0b0c0d0e0f101112131100000000000000000102030405060708090a0b
0c0d0e0f10080000000000000000010203040506070d0000000000000000010101010102010103000104000105000106
00010701010800010900010a00010b00010c00010a0000000000000000f82e000000000000006c61772d76322f6a6732
62326232612f6f7264696e6172792d70726f66696c652d646566696e6974696f6e2f763101f92c000000000000006c61
772d76322f6a673262326232612f6f7264696e6172792d70726f66696c652d6d616e69666573742f763102fa2b000000
000000006c61772d76322f6a673262326232612f6f7264696e6172792d7265736f757263652d706f6c6963792f763103
fb29000000000000006c61772d76322f6a673262326232612f6f7264696e6172792d656e76656c6f70652d636f72652f
763104fc33000000000000006c61772d76322f6a673262326232612f6f7264696e6172792d656e76656c6f70652d6578
6163742d65766964656e63652f763105fd31000000000000006c61772d76322f6a673262326232612f6f7264696e6172
792d656e76656c6f70652d71756f7469656e742d6b65792f763106fe28000000000000006c61772d76322f6a67326232
6232612f6f7264696e6172792d616374696f6e2d74726163652f763107ff28000000000000006c61772d76322f6a6732
62326232612f6f7264696e6172792d616374696f6e2d696d6167652f763108e829000000000000006c61772d76322f6a
673262326232612f6f7264696e6172792d706169722d636f7665726167652f763109e92c000000000000006c61772d76
322f6a673262326232612f6f7264696e6172792d706169722d646973706f736974696f6e2f7631010020000002000000
0000000000010000000300000000000000000102002c00000000000000000102030405060708090a0b0c0d0e0f101112
131415161718191a1b1c1d1e1f202122232425262728292a2b1800000000000000000102030405060708090a0b0c0d0e
0f1011121314151617
```

The byte vector is derived from fields 1--17 above. It is an identity fixture,
not proof or authority.

### 2.7 Explicit unavailable-authority vector

Field 17 of the static definition is the following exact 24-tag vector. Every
entry is false/unavailable at A4-O:

```text
ExecutableIndexedInterface, FactualLevelTwoCarrier, CandidateEnvelope,
FactualActionImage, PairDisposition, OccurrenceClassification, SelectorLaw,
GenericSubstitutionLaw, RustAgdaCorrespondence, NineFunctorLaws,
SevenConstructorNaturalitySquares, ParticularCubicalBridge, CubicalAdequacy,
CubicalSupportArity, CubicalRecentFactorization, ChronologicalWindow,
GCapCarrier, GCapQuotient, GCapWeakening, GCapMarginalOrGamma,
GCapProvenance, GCapBootstrap, DemandOrDebt, SelectiveVerdict.
```

## 3. Nonrecursive manifest, parent closure, and `kappa`

### 3.1 Manifest fields

After root tag `0xf9`, `OrdinaryDependentEnvelopeProfileManifestV1` encodes:

1. `manifest_schema_version : u16 = 1`;
2. the complete `OrdinaryDependentEnvelopeProfileDefinitionV1` by value;
3. `jg1_manifest_digest`;
4. `jg2a_constructor_grammar_manifest_digest`;
5. `jg2a_scope_grammar_digest`;
6. `jg2b2a_protocol_manifest_digest`;
7. `jg2b2b0_structural_grammar_manifest_digest`;
8. `jg2b2b1_particular_substitution_manifest_digest`;
9. `kernel_protocol_digest`;
10. `normalizer_protocol_digest`; and
11. `kernel_configuration_digest`.

There is no profile digest field. The domain-separated digest of this complete
record is the nonrecursive `ProfileBindingV1` identity `P`.

For the independent synthetic fixture, fields 3--11 are respectively the
valid digest texts with payload bytes repeated as `00`, `11`, `22`, `33`,
`44`, `55`, `66`, `77`, and `88`. The frozen fixture is:

```text
canonical byte length: 1923
domain-separated digest:
blake3:8198531645e4e04840d11794a0c3d50f9f393707b9ccc4e8832223be605df113
```

JG2b2b2b must reconstruct both fixtures independently, compare their full
bytes, and pin field-specific mutations. Fixture equality does not mint the
verified token.

### 3.2 Full opaque verification and dependency agreement

The future sole private verifier consumes an untrusted
`OrdinaryDependentEnvelopeProfileManifestV1` proposal, the following full
opaque parents, and one `&Kernel`:

```text
VerifiedPreExposureGenerativeCapabilityGrammarV1,
VerifiedGenerativeCapabilityConstructorGrammarV1,
VerifiedGenerativeSubstitutionNaturalityProtocolV1,
VerifiedGenerativeStructuralOccurrenceGrammarV1,
VerifiedParticularOpenTypedSubstitutionProtocolV1.
```

`kernel.kernel_protocol_digest()` and
`kernel.normalizer_protocol_digest()` supply manifest fields 9 and 10.
Manifest field 11 is the existing JG2a configuration digest under domain
`law-v2/jg2a/kernel-configuration/v1`, over, in order, little-endian
`u32(max_operations)`, `u16(max_depth)`, and
`u32(normalization_fuel)`, with no additional record root. These are exact
projections from the supplied kernel, not three caller-constructible identity
objects.

It independently remints every remintable parent before using it. It then
checks the complete dependency DAG:

1. JG2a's JG1 binding equals the supplied JG1 authority;
2. JG2b2a's JG1, JG2a constructor/scope, kernel, normalizer, and configuration
   bindings agree exactly;
3. JG2b2b0's JG2b2a, JG1, JG2a constructor/scope, kernel, normalizer, and
   configuration bindings agree exactly;
4. JG2b2b1's JG2b2b0, JG2b2a, JG1, JG2a constructor/scope, kernel,
   normalizer, and configuration bindings agree exactly; and
5. the concrete kernel configuration equals A4-O's exact resource triple.

It reconstructs the definition and manifest from those inputs, compares the
definition's full canonical bytes to the frozen static fixture, compares the
proposal's full manifest bytes to that independent reconstruction, checks both
domain-separated identities, and only then may mint
`VerifiedOrdinaryDependentEnvelopeProfileDefinitionV1`. The opaque token owns
the complete parent authorities and manifest, not only their digests. It has no
public unchecked constructor and exposes only read-only exact projections.

The static definition and manifest take no history, signature instance, stage
surface, occurrence, substitution instance, candidate, constructor choice,
expected match, theorem proof, cubical witness, or held-out label.

### 3.3 Exact `kappa` binding and A1 parent order

`kappa` is not an open parameter after A4-O. It is the private projection

```text
kappa : OrdinaryProfileBindingV1 =
  (verified_definition_token,
   exact manifest and ProfileBindingV1 P,
   complete opaque parents,
   OrdinaryEnvelopeResourcePolicyV1,
   OrdinaryEnvelopeCanonicalCodecV1).
```

For every future envelope, A1 field 1 is the exact V1 version vector plus
constructor tag. A1 field 2 is exactly

```text
(JG1, JG2a, JG2b2a, JG2b2b0, JG2b2b1, P)
```

in that order. `JG2a` is the exact verified constructor-grammar authority plus
its retained scope-grammar digest, and `P` is the
verified profile-definition token indexed by the nonrecursive manifest root.
The future `VerifiedEnvelopeCensusV1` owns all six full opaque parents once;
nested envelopes retain exact indices into that owned tuple. A digest, ID, or
caller lookup cannot substitute for the owned content.

## 4. Closed Level-I/Level-II relation

The ordinary Level-I universe is exactly the A2-O mathematical data:

```text
LevelIOrdinaryV1 ::=
  one of the nine fully indexed carrier values
| one of the seven A3-O RawCode_c values
| one of the seven canonical Built_c payloads
| one well-typed formal substitution or context-square action.
```

It contains no history authority and no checked-envelope wrapper.

The Level-II relation is:

```text
CheckedOrdinaryLevelTwoV1(kappa,H,o,c,r,E) iff
  kappa is a reminted VerifiedOrdinaryDependentEnvelopeProfileDefinitionV1,
  H is an owned verified complete-through-head history whose parent DAG agrees,
  o is internally derived by the frozen JG2b2b0 traversal,
  c is supplied by the internal seven-constructor loop,
  r is the exact retained Level-I RawCode_c,
  Eval^kappa_c(H,o,r)=Match(E), and
  E retains every A1 field, complete Level-I value/index, raw proposal,
    derived judgment, normalized result, evidence record, and exact binding.
```

An action image is a separate opaque Level-II theorem-subject object built from
a checked candidate and canonical action. It has no `Derive` proof and is not a
candidate. A pair disposition is a still later coverage object over all
Level-II match results for fixed `(kappa,H,o,c)`.

## 5. Canonical source census, enumeration, quotient, and coverage

### 5.1 Source census order

RC1's source names in this subsection are superseded by A4-R1's exact four-
role signature construction and root-`0xea`--`0xef` records. For fixed
`(H,o,c)`, let `b=Birth(o)`. The exact current vector `U_b` is sorted
lexicographically by the full canonical bytes of
`OccurrenceId14V1`. The
fourteen-field identity includes the JG2b2b0 traversal path, so this order does
not erase declaration-root or child-edge identity. Duplicate exact identities
are an aborting history-replay failure. The anchor `o` receives no priority.

`Pub_<b(H)` is sorted lexicographically by
`(ExportIdV1 canonical bytes,
PublicFieldPathV1::DeclarationField byte)`. It contains exactly one opaque
public declaration binding for each declaration born at an event strictly before
`b`; evaluation uses A4-R1's reverified signature with every such older body
erased. Duplicate exact public identities abort.
The source vector is

```text
V_b = map Current(U_b) ++ map OlderPublic(Pub_<b(H)).
```

Thus `Current` precedes `OlderPublic`; a source encodes its sum tag plus full
exact identity, never only a transient vector index. The source ordinal is
retained separately for coverage.

### 5.2 Raw-code ordinal order

The finite A3-O codec tree is enumerated without caller inputs. Its structural
order is exact:

1. sum/enum alternatives use declared tag order;
2. natural lengths and cuts use numeric ascending order;
3. a `SourceRef` uses its `V_b` ordinal;
4. a vector is ordered by length and then lexicographically by its elements;
5. a product is lexicographic in the displayed A3-O field order; and
6. dependent branches are visited depth first after the preceding *raw shape*
   fixes the next finite arity; no kernel judgment or semantic decoder success
   prunes the raw universe.

Lengths use `0 <= n <= N`; `TelCode+` uses `1 <= n <= N`; source ordinals use
`0 <= i < N`; an operation selector uses `0 <= k < p`, where `p` is the earlier
raw operation-vector length; and an abstraction cut uses
`0 <= cut < |Xi|`. Dynamic section, port, and discharge arities come only from
earlier raw telescope/result lengths. `Empty` precedes `LocalPrefix`, whose
choices use current-source order and then `j=1..BinderDepth(q)`.

The global pairwise-distinct source condition is checked incrementally against
the complete raw code. A prefix that repeats a source is a visited rejected
search node but emits no complete raw code. One used-source bitset spans the
whole enclosing raw code. Every complete `RawCode_c` is
assigned one contiguous `u32` ordinal and evaluated exactly once. The retained
canonical raw bytes and structural comparator must agree on equality; ordinal
or count overflow aborts.

The semantic `Decode_c` function is called exactly once only after the whole
structural raw code exists. A rejected semantic prefix therefore becomes the
one checked `False` outcome of that complete code rather than deleting an
implementation-dependent subtree from the universe.

Before allocating or traversing, a checked structural upper-bound interpreter
evaluates the fixed A3-O codec AST over `N`, using checked `u64` sums/products
and the distinct-source state. If any bound exceeds a resource ceiling, the
pair has no disposition. The bound may conservatively reject a pair whose
typed subbranches would later be false; this is an outside-fragment abort, not
a no-match result and not a truncation of the mathematical grammar.

### 5.3 Complete outcome vector

For every complete raw ordinal, the deterministic A3-O pipeline produces
exactly one of:

```text
CheckedFalseV1(raw_ordinal, raw_code, FalseReasonV1,
               complete_checked_rejection_evidence)

MatchV1(raw_ordinal, raw_code, normalized_envelope,
        QuotientKeyV1_c, exact_evidence_transcript).
```

Within a leaf, first failure is checked in this exact order: decode fields in
declared order; build premises in their displayed A3-O order; `Fields_c`;
normalization; structural support; empty-grammar primitive certificates;
subjects; envelope assembly; anchor; compatibility. Parent/profile replay and
pair preflight occur before the first leaf. An implementation may optimize
only when it produces the identical outcome, locus, counters, and transcript.

`Abort` is deliberately absent. Any abort at a prefix, raw leaf, build,
normalization, support traversal, quotient, encoding, allocation, or coverage
step destroys the proposed complete certificate and yields only an
unprivileged diagnostic `NoDisposition(AbortReasonV1)`. A completed prefix is
never promoted.

`CompleteEnumerationV1` retains, in order:

```text
profile binding, history binding, anchor, constructor,
complete source census, codec/cardinality derivation,
total prefix-node count, total raw-leaf count,
all outcomes in canonical raw ordinal order,
the exact match-to-class partition,
the aggregate deterministic logical resource account,
and complete no-gap/no-duplicate coverage evidence.
```

The logical resource account contains only frozen counters, limits, and
before/after deltas. It contains no wall clock, allocation address, scheduler
state, platform text, or map order.

### 5.4 Quotient classes and representatives

Only `MatchV1` outcomes enter the partition. Two matches share a class exactly
when their complete normalized `QuotientKeyV1_c` values satisfy A3-O's
`approx_(env,c)`. A4-O freezes the canonical key encoder after normalization so
that this relation is equivalent to full key-byte equality. Carrier-normal-
form equality, envelope digest equality, raw-code count, or a first match is
insufficient.

Classes are sorted lexicographically by full canonical quotient-key bytes.
Members of a class are sorted by canonical raw ordinal. The canonical
representative is its least canonical raw ordinal, not the first value found by
an implementation and not a digest minimum. All equivalent member ordinals,
raw codes, and exact representative evidence remain in coverage.

For two distinct sorted classes, `FirstDifferenceV1` retains the first unequal
full-key byte offset, the two bytes, and the decoded `QuotientKeyV1_c` field
paths. Pairwise digest inequality is not an inequivalence witness.

### 5.5 Exact three dispositions

After complete nonaborting enumeration and partitioning:

```text
CertifiedNoMatchV1 =
  complete enumeration with zero Match outcomes and every raw leaf CheckedFalse;

UniquePositiveV1 =
  the sole quotient class, its canonical representative, every equivalent
  member, complete coverage, and a one-class uniqueness certificate;

TypedAmbiguityV1 =
  every sorted quotient class, every member and representative, all pairwise
  FirstDifferenceV1 witnesses, and complete coverage.
```

Zero raw leaves is a lawful completely covered no-match only if census,
cardinality, and empty enumeration all completed within resources. Multiple
raw codes in one class count once. Enumeration never stops at the first or
second match and never chooses among ambiguous classes.

Action images never enter the raw universe, outcome vector, partition, or
disposition. One schema per constructor establishes only closed schema
coverage. It establishes neither that every occurrence matches nor that the
seven predicates are mutually exclusive.

The future occurrence-by-constructor matrix iterates constructors in the exact
JG2a order. A constructor is never caller supplied. Any pair abort suppresses
the later complete matrix/census; it cannot be relabeled as a zero row.

## 6. Envelope, quotient, evidence, and action encodings

### 6.1 Nonrecursive A1 field 17

For a successfully built candidate:

```text
EnvelopeCoreBytesV1(E)
  = encode under root 0xfb exactly A1 fields 1--16 in order;

ExactEvidenceTranscriptV1(E)
  = encode under root 0xfc the exact Decode/Build replay, normalization steps,
    support raw records and coverage, primitive certificates, obligation and
    subject construction, deterministic logical resource accounts, and
    checker receipts retained by fields 11--16;

EvidenceCommitmentV1(E)
  = Digest::of_domain_bytes(ExactEvidence domain,
                            ExactEvidenceTranscriptV1(E)).
```

Neither transcript includes A1 field 17, its own digest, or a commitment to
itself. A1 field 17 is exactly

```text
(EnvelopeCoreBytesV1(E), EvidenceCommitmentV1(E)).
```

The full exact evidence remains owned by the census/envelope. The commitment
does not replace it.

### 6.2 Quotient key versus representative transcript

`QuotientKeyBytesV1_c(E)` encodes under root `0xfd` exactly the complete A3-O
`QuotientKeyV1_c` and nothing else. In particular, normalization derivation
traces, checker proof objects, allocation receipts, resource-counter traces,
envelope core bytes, and evidence commitments are representative evidence and
are not quotient fields. The resource-policy identity and parent/profile
bindings remain quotient fields because they are part of the normalized A1
binding.

Two implementations must compare the complete decoded key and full canonical
key bytes; a key digest may index the comparison but cannot decide equality or
inequality by itself.

### 6.3 Action traces and images

`ActionTraceV1` encodes under root `0xfe` the base candidate binding, exact
unflattened step sequence, flattened canonical action, all sequential/direct
endpoints, and deterministic logical action-resource account. It is
representative evidence outside the quotient key.

`ActionImageV1` encodes under root `0xff` the profile/base binding, schema tag,
base anchor, canonical flattened action, final payload, full final quotient
key, and action-trace commitment. Its private constructor still enforces the
A3-O `CanonicalActionResult_c` equations. Identity uses the literal base
payload/key; composition always rebuilds from the original base. Formal
action dependencies remain `FormalActionSupport`, never historical or paid
owners.

Bounded action construction uses one aggregate account with at most 4,096
flattened steps and all other depth/material/byte ceilings above. Failure
constructs no action image and proves no law. Formal JG2b2b3 theorem subjects
remain unbounded derivations; V1 resource closure does not assert closure of
the Rust fragment under identity, composition, lift, or reindexing.

## 7. Fail-closed accounting order

For a pair or action, the operational order is fixed:

1. verify/remint the profile and parent DAG;
2. preflight every recursive value and checked cardinality formula before
   clone, equality, canonical encoding, hashing, or output allocation;
3. reserve the aggregate fallible buffers needed by the admitted upper bound;
4. start one pair/action ledger and one nested per-envelope ledger;
5. charge every raw prefix, raw leaf, decoder/build step, kernel operation,
   normalization step, field/evidence/support node, quotient comparison,
   encoded byte, coverage record, and action step exactly once at its frozen
   counter; and
6. publish an object only after the complete ledger and replay close.

Per-envelope counters may reset only at the start of a new raw leaf while the
pair totals continue monotonically. No kernel or normalizer call may receive a
fresh invisible pair budget. All addition, multiplication, indexing, byte
lengths, and `u64`/`u32` conversions are checked. No `.take`, partial sort,
lossy reserve, completed-prefix success, or caller ceiling exists.

Exhaustion, overflow, allocation failure, parent disagreement, incomplete
normalization, incomplete support resolution, incomplete enumeration, or
encoding failure yields no disposition. Resource failure is epistemically
neutral and never evidence for `Primitive`, `CertifiedNoMatch`, or
underivability.

## 8. Selector and theorem ownership

A repaired A4 must close the selector *definition* for each schema: the fixed
`OriginPathV1` leaf must be `Current(q)`, checked replay must preserve singleton
lineage, and pair anchoring requires `q=o`. `OlderPublic` at the principal leaf
is `False`; missing or incomplete lineage replay is `Abort`.

The following remain formal subjects retained in A1 field 16:

```text
Eval^kappa_c(H,o,r)=Match(E) ->
  exists! q in U_(Birth(o)).
    origin_(nf(E))(root_c)={q} and q=o and sel_(ord,c)(nf(E))=q,
sel(nf(E)) = sel(E),
E approx_(env,c) E' -> sel(E)=sel(E'),
sel(rename_H(E)) = rename_H(sel(E)),
sel(Re_c(q,E)) = sel(E),
Re_c(id,E) approx_(env,c) E,
Re_c(q1.q2,E) approx_(env,c) Re_c(q2,Re_c(q1,E)),
and all seven Nat_c subjects.
```

The first line is the selector-totality/uniqueness subject; the next four are
its stability subjects. Definition closure does not prove them. JG2b2b2b may implement particular
checks and synthetic mutations only. JG2b2b3a--c own the universal proofs and
combined law-verified token; JG2b2c alone may later apply that token to a
complete factual history census.

## 9. P0 exit-criterion closure

RC1 attempted to close the twelve P0 requirements as follows. The release
audit proves that rows 9--11 remain open, so this table is a rejected-candidate
coverage claim rather than an exit certificate:

| P0 requirement | Frozen binding |
| --- | --- |
| nine dependent sorts | A2-O plus field-8 carrier vector |
| formation/well-formedness | exact A2-O definitions and rule 9 |
| equality/normalization/variance | exact A2-O definitions and rules 9, 22 |
| all actions/orientations | A2-O plus field-9 action tags |
| two substitution actions | A2-O `Substitution` binding |
| seven dependent constructors | A3-O plus field-9 constructor vector |
| derived classifier schemas | A3-O pipeline plus sections 5.1--5.5 |
| nine/seven theorem subjects | A2-O/A3-O, rules 18 and 41 |
| canonical bytes/versions/parents | sections 2, 3, and 6 |
| Level-I/Level-II relation | section 4 |
| fixed resources/evidence retention | sections 2.2, 6, and 7 |
| explicit non-authority | field 17 and section 10 |

RC1 did not close definition choices because rows 9--11 remain open. Even a
repaired A4 will not prove totality, disjointness, operational adequacy,
selector stability, a functor law, or naturality.

## 10. Authority boundary and continuation

A4-O is still attempting to freeze only the target-neutral ordinary profile
definition and exact future remint contract. RC1 minted none of the 24
unavailable authorities in section 2.7 and in particular no factual pair
disposition.

The active repair record is
`docs/260802_jg2b2b2a_a4_r2d_quotient_outcome_coverage_action_schemas.md`.

The ordered continuation is:

1. **JG2b2b2a-A4-R0 -- RC1 release audit -- DISCHARGED 2026-08-02.** RC1 was
   byte-reproducible but failed semantic/representation closure.
2. **JG2b2b2a-A4-R1 -- exact signature and noncircular source census --
   FROZEN 2026-08-02.** The independent audit passed.
3. **JG2b2b2a-A4-R2 -- complete typed schemas and dynamic codecs -- ACTIVE AT
   R2d.** R2a, R2b, A3-C1, and R2c are independently frozen; R2d remains.
4. **JG2b2b2a-A4-R3 -- failure/resources -- BLOCKED BY R2d.** Its closed types
   feed the post-R3 R2e integration join.
5. **JG2b2b2a-A4-R2e -- complete registry/source integration -- BLOCKED BY
   R3.** Its audit unblocks R4.
6. **JG2b2b2a-A4-R4 -- regenerated fixtures -- BLOCKED BY R2e.**
7. **JG2b2b2a-A2-C -- cubical indexed ontologies -- OPEN IN PARALLEL.** The two
   cubical claim IDs remain unavailable until their own nine carriers, actions,
   encodings, finite presentation universes, checkers, and adequacy fields
   close.
8. **JG2b2b2b -- executable ordinary Rust representation and particular
   fail-closed operations -- BLOCKED BY A4-R4.**
9. **JG2b2b3a -- generic substitution and exact Rust/Agda correspondence --
   BLOCKED BY JG2b2b2b.**
10. **JG2b2b3b -- nine functor laws -- BLOCKED.**
11. **JG2b2b3c -- seven constructor squares, selector laws, and combined
   universal authority -- BLOCKED.**
12. **JG2b2c -- factual complete-through-head disposition matrix -- BLOCKED.**

The ordinary A4 lane remains open at active A4-R2d, independently of the open A2-C
cubical specification lane. Full JG2b2b2, JG2b2b, JG2b2, JG2, and every
downstream GCap/selective gate remain open.
