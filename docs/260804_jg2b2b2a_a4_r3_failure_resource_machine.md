# JG2b2b2a-A4-R3 Failure, Resource, and Meter Machine

Date: 2026-08-04

Status: **A4-R3 COMPLETE 2026-08-05 — §1 total reservation census, §2
failure-receipt lane, §3 kernel-role/replay and cardinality lane, and §4
accounts, meter, and cut-global closure all closed; every ledger line
LF.1–LF.13 checked by the final audit (independent two-notation census
re-verification included); fourteen registered decisions RD-R3-A-1 through
RD-R3-C-6; next lawful move A4-R2e**

This record executes the A4-R3 cut under the enumerated closure ledger of
plan §13.4 (`docs/260804_autonomous_plan_and_status.md`, lines LF.1–LF.13)
in the pre-registered sub-cut order F-A -> F-B -> F-C, deciding the §13.4a
underdeterminations assigned to each sub-cut (F-A: items 1, 2, 11, 12 as
RD-R3-A-1, RD-R3-A-2, RD-R3-A-4, RD-R3-A-3; F-B: items 3, 4, 5, 6 as
RD-R3-B-1, RD-R3-B-2, RD-R3-B-3, RD-R3-B-4; F-C: items 7, 8, 9, 10, 13,
14 as RD-R3-C-1, RD-R3-C-2, RD-R3-C-3, RD-R3-C-4, RD-R3-C-5,
RD-R3-C-6). R3 fills only the already
qualified reserved `R3::` positions of the frozen records under their
frozen outer tags: it mints no new root, no new R3-qualified position, and
changes no closed grammar byte, tag, field order, ownership edge, scalar
width, opcode, predecessor vector, or registered decision (RD-RT-A-1
through RD-RT-C-4 and every earlier registered decision). Immutable
read-only inputs: the complete R2d record
(`docs/260802_jg2b2b2a_a4_r2d_quotient_outcome_coverage_action_schemas.md`),
R2a, R2b, R2c, A4-O (`260802_jg2b2b2a_a4_ordinary_profile_closure.md`),
RC1 (`260802_jg2b2b2a_a4_rc1_release_audit.md`), R1, and all QG1/QG2/QG3
bytes. The work is compiler-plan level only: no numerical `TypeIdV1`, no
registry/source join (R2e), no fixture or mutation regeneration (R4), no
metered batch API, executable authority, or verified profile token
(JG2b2b2b; the meter is unavailable), and no theorem; the record-§10.19.6
truth-audit gate persists unweakened. The work stops fail-closed on any
grammar, topology, or audit failure; exhaustion, overflow, and allocation
failure yield no disposition and are never evidence (lessons 1 and 5).

## 1. Total reservation census (LF.1)

### 1.1 Census discipline and the totality rule

The census below was re-derived from the frozen records, not copied from
any prior inventory. Two sweeps were run and reconciled:

1. an exact-token sweep for the `R3::` qualifier over R2b, R2c, and the
   R2d record (R2a, A4-O, and RC1 contain zero `R3::` tokens — their
   binding content is prose-named, per the explicit caution registered at
   plan §13.4 LF.1); and
2. a prose sweep over R2a §12 (the six R3-owned families), A4-O §§2.2–2.3,
   5.3, 6.1, 6.3, 7, and RC1 §§2.5–2.6 (the repair-rule families).

Totality rule, binding on every later section of this record and on F-B
and F-C: every position filled by this cut is a row of this census; no
position is invented; none is missed; a fill at any non-census position,
and a census row left neither filled nor explicitly deferred to a named
F-B/F-C section, is an abort of the cut. Counts are symbolic: the number
of qualified positions is the row count of the tables below, never a
separately stored numeral.

### 1.2 R2b qualified positions

Document: `260802_jg2b2b2a_a4_r2b_level_relation_schemas.md`.

| Row | Location | Containing type and position | Owning family |
| --- | --- | --- | --- |
| B1 | §6.1 (line 435) | `KernelSuccessReplayV1.call_role` | `R3::KernelCallRoleV1` |
| B2 | §6.1 (line 438) | `KernelSuccessReplayV1.receipt` | `R3::KernelSuccessReceiptV1` |
| B3 | §6.3 (line 646) | `SourceResolutionResultV1.False=1` payload | `R3::CheckedFalseReceiptV1` |
| B4 | §6.3 (line 647) | `SourceResolutionResultV1.Abort=2` payload | `R3::AbortReceiptV1` |
| B5 | §10 (line 1129) | `RawEnumerationV1.cardinality` | `R3::StructuralCardinalityReceiptV1` |
| B6 | §10 (line 1133) | `CarrierCheckResultV1.False=1` payload | `R3::CheckedFalseReceiptV1` |
| B7 | §10 (line 1134) | `CarrierCheckResultV1.Abort=2` payload | `R3::AbortReceiptV1` |
| B8 | §10 (line 1138) | `RawEnumerationResultV1.Abort=1` payload | `R3::AbortReceiptV1` |
| B9 | §10 (line 1142) | `DecodeRawResultV1.False=1` payload | `R3::CheckedFalseReceiptV1` |
| B10 | §10 (line 1143) | `DecodeRawResultV1.Abort=2` payload | `R3::AbortReceiptV1` |
| B11 | §10 (line 1147) | `BuildDecodedResultV1.False=1` payload | `R3::CheckedFalseReceiptV1` |
| B12 | §10 (line 1148) | `BuildDecodedResultV1.Abort=2` payload | `R3::AbortReceiptV1` |
| B13 | §10 (line 1152) | `FormalActionSyntaxCheckResultV1.False=1` payload | `R3::CheckedFalseReceiptV1` |
| B14 | §10 (line 1153) | `FormalActionSyntaxCheckResultV1.Abort=2` payload | `R3::AbortReceiptV1` |

R2b §6.1 additionally freezes the replacement discipline for every row of
this census: "R3 must replace each qualified `R3::` slot with one exact
earlier local type without moving the R2b field. It may not add an error
string, optional receipt, or implementation counter." R2b §11 confirms the
five R2b-visible family names as "explicit R3-owned typed slots whose
outer positions are frozen here. They are not extension bytes."

### 1.3 R2c qualified positions

Document: `260802_jg2b2b2a_a4_r2c_envelope_evidence_support_subject_schemas.md`.
The bracketed index `[k/10]` marks the ten `EnvelopeResourceAccountV1`
fields whose count is frozen by R2c acceptance condition 17 ("every one of
the ten resource-account fields is bound by the closed
leaf/nested/aggregate hierarchy").

| Row | Location | Containing type and position | Owning family |
| --- | --- | --- | --- |
| C1 | §4 (line 494) | `CheckedSignatureReplayV1.call_role` | `R3::KernelCallRoleV1` |
| C2 | §4 (line 499) | `CheckedSignatureReplayV1.receipt` | `R3::KernelSuccessReceiptV1` |
| C3 | §4 (line 502) | `CheckedContextReplayV1.call_role` | `R3::KernelCallRoleV1` |
| C4 | §4 (line 506) | `CheckedContextReplayV1.receipt` | `R3::KernelSuccessReceiptV1` |
| C5 | §5.3.5 (line 1490) | `SubjectConstructionTranscriptV1.node_type_replays` (vector element) | `R3::KernelSuccessReceiptV1` |
| C6 | §5.3.5 (line 1491) | `SubjectConstructionTranscriptV1.resource_account` [1/10] | `R3::EnvelopeResourceAccountV1` |
| C7 | §6 (line 1867) | `EnvelopeNormalizationEvidenceV1.resource_account` [2/10] | `R3::EnvelopeResourceAccountV1` |
| C8 | §7.3 (line 2643) | `SupportCoverageV1.shape_cardinality` | `R3::StructuralCardinalityReceiptV1` |
| C9 | §7.3 (line 2644) | `SupportCoverageV1.resolution_cardinality` | `R3::StructuralCardinalityReceiptV1` |
| C10 | §7.3 (line 2645) | `SupportCoverageV1.resource_account` [3/10] | `R3::EnvelopeResourceAccountV1` |
| C11 | §7.3 (line 2817) | `OrdinarySupportV1.resource_account` [4/10] | `R3::EnvelopeResourceAccountV1` |
| C12 | §8 (line 2891) | `OrdinaryEmptyReplacementTranscriptV1.empty_enumeration_cardinality` | `R3::StructuralCardinalityReceiptV1` |
| C13 | §8 (line 2898) | `CompleteNoReplacementCertificateV1.resource_account` [5/10] | `R3::EnvelopeResourceAccountV1` |
| C14 | §8 (line 2937) | `FieldDispositionConstructionTranscriptV1.resource_account` [6/10] | `R3::EnvelopeResourceAccountV1` |
| C15 | §9.2 (line 3058) | `PublicLeavesConstructionTranscriptV1.resource_account` [7/10] | `R3::EnvelopeResourceAccountV1` |
| C16 | §9.2 (line 3067) | `ObligationConstructionTranscriptV1.resource_account` [8/10] | `R3::EnvelopeResourceAccountV1` |
| C17 | §9.2 (line 3071) | `EnvelopeAssemblyTranscriptV1.checked_field_count` | `R3::StructuralCardinalityReceiptV1` |
| C18 | §9.2 (line 3072) | `EnvelopeAssemblyTranscriptV1.checker_receipts` (vector element) | `R3::KernelSuccessReceiptV1` |
| C19 | §9.2 (line 3073) | `EnvelopeAssemblyTranscriptV1.resource_account` [9/10] | `R3::EnvelopeResourceAccountV1` |
| C20 | §9.2 (line 3086) | `ExactEvidenceTranscriptV1.aggregate_resource_account` [10/10] | `R3::EnvelopeResourceAccountV1` |
| C21 | §11.1 (line 3602) | `SupportShapeTraversalV1.cardinality` | `R3::StructuralCardinalityReceiptV1` |
| C22 | §11.1 (line 3611) | `SupportOwnerResolutionV1.resolution_cardinality` | `R3::StructuralCardinalityReceiptV1` |
| C23 | §11.1 (line 3662) | `PreEvidenceProjectionResultV1.Abort=1` payload | `R3::AbortReceiptV1` |
| C24 | §11.1 (line 3666) | `NormalizationResultV1.Abort=1` payload | `R3::AbortReceiptV1` |
| C25 | §11.1 (line 3670) | `SupportShapeResultV1.Abort=1` payload | `R3::AbortReceiptV1` |
| C26 | §11.1 (line 3674) | `SupportOwnerResolutionResultV1.False=1` payload | `R3::CheckedFalseReceiptV1` |
| C27 | §11.1 (line 3675) | `SupportOwnerResolutionResultV1.Abort=2` payload | `R3::AbortReceiptV1` |
| C28 | §11.1 (line 3679) | `SupportClosureResultV1.False=1` payload | `R3::CheckedFalseReceiptV1` |
| C29 | §11.1 (line 3680) | `SupportClosureResultV1.Abort=2` payload | `R3::AbortReceiptV1` |
| C30 | §11.1 (line 3684) | `FieldDispositionResultV1.False=1` payload | `R3::CheckedFalseReceiptV1` |
| C31 | §11.1 (line 3685) | `FieldDispositionResultV1.Abort=2` payload | `R3::AbortReceiptV1` |
| C32 | §11.1 (line 3689) | `SubjectConstructionResultV1.False=1` payload | `R3::CheckedFalseReceiptV1` |
| C33 | §11.1 (line 3690) | `SubjectConstructionResultV1.Abort=2` payload | `R3::AbortReceiptV1` |
| C34 | §11.1 (line 3694) | `EnvelopeAssemblyResultV1.Abort=1` payload | `R3::AbortReceiptV1` |
| C35 | §11.1 (line 3698) | `CandidateMatchResultV1.False=1` payload | `R3::CheckedFalseReceiptV1` |
| C36 | §11.1 (line 3699) | `CandidateMatchResultV1.Abort=2` payload | `R3::AbortReceiptV1` |

R2c §1.1 (lines 102–123) is the binding scope contract over rows C6, C7,
C10, C11, C13, C14, C15, C16, C19, C20: `Acct_R3(scope,events)` and
`Aggregate_R3(scope,accounts,aggregate_only_events)` with frozen scope
tags, event orders, child-account orders, and opaque-span treatment; "R3
closes their internal counter/limit/meter layout and proves conservation;
it may not change the scopes, event order, child-account order, or
opaque-account treatment fixed here." R2c §9.2 (lines 3092–3095) binds
rows C18/C20: "Checker/resource receipt vectors are exhaustive in
deterministic pipeline/call order and are reconciled with the one
aggregate account; R3 closes their internal schemas and conservation rules
without moving any field above."

### 1.4 R2d-record qualified positions

Document: `260802_jg2b2b2a_a4_r2d_quotient_outcome_coverage_action_schemas.md`.

| Row | Location | Containing type and position | Owning family |
| --- | --- | --- | --- |
| D1 | §4 (line 196) | `RankFreeCheckedLeafOutcomeV1.CheckedFalse=1.receipt` (internal construction partition, unrooted) | `R3::CheckedFalseReceiptV1` |
| D2 | §6 (line 334), finalized §14.2.1 (line 15684) | `CheckedLeafOutcomeV1.CheckedFalse=1.receipt` (root `0xe7`) | `R3::CheckedFalseReceiptV1` |
| D3 | §14.1.4 (line 15566) | `QuotientProjectionResultV1.Abort=1` payload | `R3::AbortReceiptV1` |
| D4 | §14.1.4 (line 15574) | `QuotientKeyComparisonResultV1.Abort=1` payload | `R3::AbortReceiptV1` |
| D5 | §14.2.4.3 (line 16080) | `PairCoverageV1.logical_account` (root `0xe8`) | `R3::PairResourceAccountV1` |
| D6 | §14.2.6 (line 16298) | `PairCoverageResultV1.Abort=1` payload | `R3::AbortReceiptV1` |
| D7 | §14.2.6 (line 16308) | `PairDispositionResultV1.Abort=1` payload — the frozen `NoDisposition` realization of §14.2.5 | `R3::AbortReceiptV1` |
| D8 | §14.3.2.1 (line 16714) | `ActionTraceV1.logical_account` (root `0xfe`, last position) | `R3::ActionResourceAccountV1` |
| D9 | §14.3.4.2 (line 17275) | `ActionRebuildResultV1.False=1` payload | `R3::CheckedFalseReceiptV1` |
| D10 | §14.3.4.2 (line 17276) | `ActionRebuildResultV1.Abort=2` payload | `R3::AbortReceiptV1` |

D1 and D2 are two frozen display occurrences of the same leaf-outcome
receipt position: the record-§4 internal partition is bound to the rooted
§14.2.1 sum by the frozen section-4/5 equations, and the §14.2.6 presence
law constructs D2 values exactly from D1 values.

### 1.5 Prose-named families and inherited transitive channels

The R2a §12 R3-owned family list (lines 707–723) is the authority; each
family is realized by census rows and by the schemas of this record:

1. "`FalseReasonV1`, `AbortReasonV1`, their loci and payloads" — closed in
   §2.1 of this record (A4-O §2.3 sums; RC1 §2.5 payload repair rules).
   These are by-value constituents of the two receipt types; they occupy
   no separate outer position.
2. "`KernelCallRoleV1` and exact success/error receipts" — rows B1, B2,
   C1–C5, C18; contents close in F-B (§13.4a items 3, 4, 5). Per A4-O
   §2.2 (lines 219–227) the current kernel "does not yet return a usage
   receipt"; no verified profile token may exist until the meter is
   available. There is no `R3::KernelErrorReceiptV1` position anywhere in
   the frozen records (verified by sweep): error receipts are realized
   inside the false/abort families' optional kernel-error field (§2.3.2,
   §2.4.1), whose exact form is the F-B registered decision on §13.4a
   item 5.
3. "`CheckedFalseReceiptV1` and unprivileged `NoDisposition` diagnostics"
   — rows B3, B6, B9, B11, B13, C26, C28, C30, C32, C35, D1, D2, D9
   (false receipts, closed in §2.3) and row D7 with every other
   `AbortReceiptV1` row (the diagnostic lane, closed in §2.4).
4. "pair/envelope/action resource accounts and aggregate meter receipts"
   — the ten R2c account rows, D5, and D8; internal layout, meter, and
   conservation close in F-C (§13.4a items 7, 8, 9, 10) under the frozen
   R2c §1.1 contract, including the field-17 commitment-metering
   reservation of R2c §9.2 (lines 3220–3229).
5. "structural cardinality and count-only traversal evidence" — rows B5,
   C8, C9, C12, C17, C21, C22 plus the count-only traversal evidence
   family of RC1 §2.6, which the R2d §14.2.4.2 owner table assigns the
   A4-O §5.3 "total prefix-node count" item; per LF.7 it is realized
   inside a reserved position without a new outer position. Closes in F-B
   (§13.4a item 6).
6. "material/depth recurrence and allocation/budget receipts" — the
   material-size recursion and fourteen-counter mapping of RC1 §2.6
   (items 3–4) and the allocation/budget payloads of A4-O §2.3. The
   `AllocationFailure` and `BudgetExhausted` payload schemas close in
   §2.1 of this record; the recurrence realization and the exact
   fourteen-counter mapping close in F-C (§13.4a item 13; LF.8).

Inherited transitive channels (positions already counted above, reached
through frozen containing values; the census adds no row for them):

- root `0xfb` per the R2c §9.3 corrigendum (lines 3313–3321): A1
  field 7 directly contains kernel success receipts (C2, C4); fields 8
  and 11 contain them transitively through checked-built/decode evidence
  (B1, B2); fields 13–15 directly retain normalization, support,
  disposition, cardinality, and resource receipts (C7–C14); field 16
  contains them transitively through checked raw carrier values;
- root `0xfc` per R2c §9.2: the root payload is
  `ExactEvidenceTranscriptV1`, whose construction-only transcripts hold
  the direct rows C15–C20, whose `decode_evidence`/`build_evidence` are
  the exact checked-built values carrying B1/B2, and whose every
  component repeated from fields 11–16 is byte-identical (R2c lines
  3090–3092), so the field-11–16 positions recur without a new row;
- root `0xfe` per R2d §14.3.2.1: exactly three inherited channels —
  `base_binding.base_candidate.full_identity`, each `unflattened_steps`
  element's `replay_evidence` vector of `KernelSuccessReplayV1` values,
  and each endpoint's `canonical_schema_payload` checked raw carriers —
  plus the directly declared `logical_account` (D8);
- root `0xe3` per R2d §14.3.3.2: the R2b success receipts inside the
  tag-0–2 checked compounds, the four tag-3 channels, and the tag-4
  `base_candidate.full_identity` channel;
- roots `0xe5`/`0xe6` per the R2a §9 corrigendum (lines 575–582), whose
  explicit clause is that "the earlier five-root inventory omitted the
  frozen R2b success receipts under `0xe5`/`0xe6`": the standalone
  checked-decoded (`CheckedDecodedV1`) and checked-built
  (`CheckedBuiltV1`) root payloads contain rows B1, B2 through the
  `KernelSuccessReplayV1` values of their decode/build evidence;
- roots `0xe0`, `0xf8`, `0xf9`, `0xfa`: the static failure/resource
  definitions nested by value (R2a §9 corrigendum); their reconciliation
  with the frozen A4-O field-13 tag tables is F-C's §13.4a item 14.

Root `0xfd` remains receipt-free by the frozen quotient stripping rule;
roots `0xe9` and `0xff` remain receipt-free by the registered decisions
RD-RT-B-2 and RD-RT-C-2(i). No census row nests under any of these three.

### 1.6 Agreement checks

1. Against R2a §12: every one of the six prose families maps onto census
   rows or F-A/F-B/F-C schema obligations as listed in §1.5; no family is
   unrepresented; no census row falls outside a family.
2. Against the R2d tail enumeration (§14.3.4.4, lines 17413–17418): the
   R2d closing statement names `R3::CheckedFalseReceiptV1`,
   `R3::AbortReceiptV1`, `R3::StructuralCardinalityReceiptV1`,
   `R3::PairResourceAccountV1`, `R3::ActionResourceAccountV1`, "and the
   positions inherited through frozen R2b/R2c values". The first five
   names cover exactly the D-rows plus the B/C rows of those families;
   the inherited clause covers `KernelCallRoleV1`,
   `KernelSuccessReceiptV1`, and `EnvelopeResourceAccountV1`, whose
   defining occurrences are B/C rows. Exact agreement.
3. Against the plan §13.4 LF.1 line: the eight named families at their
   exact R2b/R2c/R2d positions are `KernelCallRoleV1`,
   `KernelSuccessReceiptV1`, `StructuralCardinalityReceiptV1`,
   `CheckedFalseReceiptV1`, `AbortReceiptV1`,
   `EnvelopeResourceAccountV1` (ten R2c positions),
   `PairResourceAccountV1` (D5), `ActionResourceAccountV1` (D8); the
   census sweeps both notations and adds the R2a §12
   count-only-traversal and loci/payload families. Exact agreement.

## 2. F-A: the failure-receipt lane

### 2.1 The closed reason taxonomy (LF.2)

#### 2.1.1 `FalseReasonV1`

`FalseReasonV1` closes as exactly the A4-O §2.3 closed tagged sum, with no
variant added, removed, renamed, or reordered. The top-level and nested
variants both use their zero-based displayed ordinals (the frozen A4-O
convention), each encoded as one `u8` tag under the frozen R2a wire; a
`FalseReasonV1` value encodes as the family tag followed by the nested
tag, and the nested variants carry no payload:

```text
FalseReasonV1 ::=
  RawGrammar=0(
    OutsideSourceCensus=0 | WrongSourceVariant=1 | DuplicateSourceRef=2 |
    LengthOutOfRange=3 | OrdinalOutOfRange=4 | DependentArityMismatch=5 |
    RequiredNonemptyViolation=6 | ConstructorShapeMismatch=7)
| Decode=1(
    PrefixWeakeningUnavailable=0 | BinderIdentityPrefixMismatch=1 |
    PrefixTypeMismatch=2 | TypeFormationRejected=3 | HasTypeRejected=4 |
    EqualityRejected=5 | DependentShapeMismatch=6)
| Schema=2(
    OrderedPremiseRejected=0 | ParticularSubstitutionRejected=1 |
    ParticularLiftRejected=2 | RawOutputTypingRejected=3 |
    RawOutputMismatch=4 | ComparisonRejected=5 |
    EmptyReplacementCertificateMismatch=6 | TheoremSubjectTypingRejected=7)
| Support=3(
    ReferentJudgmentTagMismatch=0 | CurrentOccurrenceNotAdmissible=1 |
    OlderExportNotMember=2 | OlderExportAmbiguous=3 |
    OlderExportNotPublic=4 | PrivateBodyRequested=5 |
    LaterBirthRequested=6 | FormalDependencyInCandidate=7 |
    OwnerKindMismatch=8 | NonDecreasingDependency=9 | DependencyCycle=10)
| Anchor=4(
    PrincipalLeafOlderPublic=0 | PrincipalOriginEmpty=1 |
    PrincipalOriginMultiple=2 | AnchorOccurrenceMismatch=3)
| Compatibility=5(
    RawCodeBindingMismatch=0 | ImplementationProjectionMismatch=1 |
    PriorSupportProjectionMismatch=2 | CurrentOutputProjectionMismatch=3 |
    LocalBoundaryOrBinderMismatch=4)
| Action=6(
    BaseBindingMismatch=0 | ActionDomainMismatch=1 |
    ActionCodomainMismatch=2 | ActionArityMismatch=3 |
    ActionTypingRejected=4 | SquareBoundaryMismatch=5 |
    SquareEqualityRejected=6)
```

The frozen availability rules hold unchanged: `RawGrammar` is available
only to a future particular-input decoder — canonical pair enumeration
never generates an ill-shaped raw code; `Action` is available only before
an action image exists — an internally derived checked-action-trace
mismatch is `Abort`, not `False`. There is no `Other`, no free diagnostic
payload, and no platform text anywhere in this sum. Unknown or unmapped
failure is conservatively `Abort`, never `False`.

#### 2.1.2 `AbortReasonV1`

`AbortReasonV1`, which is outside every coverage certificate and
disposition, closes with exactly the A4-O §2.3 variants in their
zero-based displayed order, each payload now realized by an exact typed
schema (subordinate schemas in §2.1.3; payload field names are minted by
this record, payload contents and order are the frozen A4-O display):

```text
AbortReasonV1 ::=
  OutsideProfileFragment=0(boundary:ProfileBoundaryV1)
| ProfileParentMismatch=1(parent:ProfileParentTagV1)
| HistoryParentMismatch=2(parent:ParentSlotV1)
| KernelProtocolMismatch=3
| NormalizerProtocolMismatch=4
| KernelConfigurationMismatch=5
| CompleteHistorySchemaMismatch=6
| CompleteHistoryResourcePolicyMismatch=7
| VerifiedIdentityMissing=8(site:DefinitionRulePathV1)
| ArithmeticOverflow=9(op:ArithmeticOpV1,site:DefinitionRulePathV1)
| AllocationFailure=10(counter:ResourceCounterV1,site:DefinitionRulePathV1)
| BudgetExhausted=11(counter:ResourceCounterV1,used:u64,limit:u64)
| KernelResourceExhausted=12(kind:KernelResourceKindV1)
| NormalizationFailed=13(stage:PipelineStageV1)
| VerifiedParentReplayRejected=14(stage:PipelineStageV1)
| EnumerationCardinalityMismatch=15(expected:u64,observed:u64)
| CoverageMismatch=16(site:DefinitionRulePathV1)
| CanonicalEncodingMismatch=17(object:CodecObjectV1,site:DefinitionRulePathV1)
| QuotientInvariantMismatch=18(site:DefinitionRulePathV1)
| ActionTraceInvariantMismatch=19(site:DefinitionRulePathV1)
```

`ParentSlotV1` and `DefinitionRulePathV1` are the frozen earlier types
(R1/A4-O field 7 and R2a §6 respectively) and are consumed unchanged;
`DefinitionRulePathV1` is a canonical typed ordinal path into fields 6–16
of the static definition, validated per R2a §6 (every step kind, tag, and
ordinal checked against the schema node and runtime value; the path must
end at the declared expected type and re-encode byte-identically). It is
never caller text. All subordinate names are closed enums encoded in the
static transcript; none contains platform error text.

#### 2.1.3 Subordinate locus and payload schemas — REGISTERED DECISION RD-R3-A-3 (§13.4a item 12)

The underdetermination, catalogued at plan-§13.4a registration and
assigned to F-A: the exact widths of the profile-boundary, codec-object,
and arithmetic payloads; whether optional judgments and relevant input
bytes in false receipts are complete or ceiling-bounded; and their
charging counter.

Decision (registered): the subordinate schemas close as follows, with
exact widths, per the RC1 §2.5 repair rules ("It omitted the subordinate
`ProfileBoundary`, arithmetic, resource, codec-object, and path schemas
and their payload widths"; the path schema was subsequently closed by the
frozen R2a §6 and is consumed, not redefined):

```text
ArithmeticOpV1 ::= Add=0 | Multiply=1 | IndexConversion=2

KernelResourceKindV1 ::= Operations=0 | Depth=1 | Normalization=2

PipelineStageV1 ::=
  ProfileRemint=0 | HistoryReplay=1 | SourceCensus=2 | RawPreflight=3 |
  RawEnumeration=4 | Decode=5 | Build=6 | Fields=7 | Normalization=8 |
  Support=9 | Subjects=10 | Envelope=11 | Anchor=12 | Compatibility=13 |
  Quotient=14 | Coverage=15 | Action=16

ProfileParentTagV1 ::=
  JG1=0 | JG2a=1 | JG2b2a=2 | JG2b2b0=3 | JG2b2b1=4 |
  Kernel=5 | Normalizer=6 | KernelConfiguration=7

ResourceCounterV1 ::=
  SourceCensusEntries=0 | SyntacticDepth=1 | RawSearchNodesPerPair=2 |
  CompleteRawCodesPerPair=3 | MaterialNodesPerEnvelope=4 |
  MaterialNodesPerPair=5 | KernelOperationsPerEnvelope=6 |
  NormalizationFuelPerEnvelope=7 | KernelOperationsPerPair=8 |
  NormalizationFuelPerPair=9 | EnvelopeCoreBytes=10 |
  PairTranscriptBytes=11 | ProfileManifestBytes=12 |
  NormalizedActionSteps=13

CodecObjectV1 ::=
  ProfileDefinition=0 | ProfileManifest=1 | ResourcePolicy=2 |
  EnvelopeCore=3 | ExactEvidence=4 | QuotientKey=5 | ActionTrace=6 |
  ActionImage=7 | PairCoverage=8 | PairDisposition=9 |
  ExactOccurrenceId14=10 | NormalizedOccurrenceKey=11 |
  OlderPublicExportId=12 | OlderPublicExport=13 | BirthOpaqueSignature=14 |
  BirthSourceCensus=15 | DynamicSchemaRegistry=16 | LevelOneProposal=17 |
  CheckedLevelTwo=18 | RawConstructorCode=19 | CheckedDecoded=20 |
  CheckedBuilt=21 | CheckedLeafOutcome=22

ProfileBoundaryV1 =
  (exceeded_counter:ResourceCounterV1,
   computed_bound:u64,
   ceiling:u64,
   formula_site:DefinitionRulePathV1)
```

Clause by clause, with the argument from frozen text:

1. Every closed enum above encodes as one `u8` tag in zero-based
   displayed order. The variant lists are frozen verbatim:
   `ArithmeticOpV1` and `KernelResourceKindV1` are exactly the displayed
   A4-O §2.3 argument alternatives of `ArithmeticOverflow` and
   `KernelResourceExhausted`; `PipelineStageV1` is exactly the A4-O §2.3
   "in order" stage list; `ProfileParentTagV1` is exactly "the five
   upstream A1 parent tags followed by `Kernel`, `Normalizer`, and
   `KernelConfiguration`"; `ResourceCounterV1` is exactly the fourteen
   `(ResourceCounterV1 tag, u64 ceiling)` rows of A4-O §2.2 in their
   exact displayed order; `CodecObjectV1` is exactly the object column of
   the R2a §9 `CodecDomainBindingV1` table at its frozen object ordinals
   (RC1 objects 0–9, R1 objects 10–15, R2 objects 16–22). The `u8` width
   is the frozen R2a/R2d wire rule ("enum and sum tags are `u8` in
   displayed zero-based order"); no wider or split encoding is lawful.
2. `ProfileBoundaryV1` (the payload of `OutsideProfileFragment`) closes
   as the conservative preflight rejection record of A4-O §5.2: "a
   checked structural upper-bound interpreter evaluates the fixed A3-O
   codec AST over `N`, using checked `u64` sums/products"; "If any bound
   exceeds a resource ceiling, the pair has no disposition"; "this is an
   outside-fragment abort, not a no-match result". It therefore retains
   exactly which frozen ceiling was exceeded (`exceeded_counter`), the
   checked computed bound (`computed_bound:u64` — the interpreter's
   checked `u64` arithmetic), the frozen ceiling value (`ceiling:u64` —
   byte-equal to the profile's §2.2 row for that counter), and the
   checked cardinality-formula site (`formula_site` — the A4-O §7 step-2
   preflight names "every recursive value and checked cardinality
   formula"; the site is a validated `DefinitionRulePathV1`). All scalar
   widths are forced by the frozen
   `cardinality_and_counter_bits : u16 = 64`.
3. The arithmetic payload retains no operand values: A4-O §2.3 displays
   exactly `(Add | Multiply | IndexConversion, DefinitionRulePathV1)`,
   and adding operand fields would extend a frozen variant sum (LF.2
   forbids it) and constitute a free diagnostic payload. The `site` path
   identifies the checked operation; nothing else is retained.
4. The resource payload of `BudgetExhausted` closes per the RC1 §2.5
   repair rule mandated verbatim by ledger lines LF.2 and LF.5:
   "`BudgetExhausted` carries the exact `ResourceCounterV1` tag directly
   plus `used:u64` and `limit:u64`". The six displayed A4-O §2.3
   category names (`Operation | Depth | NormalizationFuel |
   AggregateMaterial | RawCodeCount | CanonicalBytes`) remain the frozen
   prose grouping of the fourteen counters and are not a stored byte:
   storing a category byte would re-introduce the audited RC1 defect
   ("It also used six generic budget categories for fourteen resource
   counters") and store a value derivable from the exact tag.
   `EnumerationCardinalityMismatch` closes with `expected:u64,
   observed:u64`, forced by the same frozen counter width.
5. Optional expected/actual checked judgments and the exact relevant
   input bytes in false receipts are COMPLETE, never truncated,
   digest-substituted, or ceiling-clipped. The frozen text is exact on
   each: A4-O §2.3 "the false outcome retains the offending
   raw-field/value path, exact expected judgment, exact kernel rejection
   where applicable, and the completed checked witness"; RC1 §2.5
   "optional expected and actual checked judgments" and "exact relevant
   input bytes". Truncation would substitute a lossy identity for
   evidence (rules 38/39: full canonical bytes are compared before
   digests; digest identity never replaces evidence) and would require an
   invented truncation numeral, which no frozen location supplies. The
   retained values are nevertheless already resource-bounded by the
   frozen ceilings alone: judgments are normalized kernel terms bounded
   by `SyntacticDepth` and the material counters; the relevant input
   bytes are the canonical bytes of the exact value selected by the
   receipt's validated locus path, bounded by the frozen byte ceilings.
6. Charging counter: a checked-false or abort receipt is an outcome
   record of its pair; every retained node of its judgments, paths, and
   input bytes charges `MaterialNodesPerPair` (frozen A4-O §2.2 scope:
   "search nodes, outcomes, representatives, coverage, and partition
   records"; repetitions count), and its canonical encoded bytes charge
   `PairTranscriptBytes` (frozen scope: "all outcomes, representatives,
   partition, and coverage evidence for one pair"), each charged exactly
   once at its frozen counter per the A4-O §7 accounting order. No fresh
   invisible allowance and no uncharged retention exists; the receipt
   rows of the total fourteen-counter object mapping close in F-C with
   LF.8 without altering these two counter assignments.

#### 2.1.4 The total kernel-error map, unchanged

The complete current `pen-kernel::KernelError` map is consumed exactly as
frozen in A4-O §2.3:

- `ResourceExhausted(Operations|Depth|Normalization)`, `InvalidLimits`,
  and `UniverseOverflow` map to `Abort`;
- `UnboundVariable`, `InvalidSubstitution`, `SubstitutionArity`,
  `UnknownGlobal`, `ExpectedType`, `ExpectedFunction`, `ExpectedPair`,
  `TypeMismatch`, and `WrongJudgmentForm` map to the enclosing
  typed-false stage only for a candidate-derived judgment; and
- every one of those errors, plus `DuplicateGlobal`, maps to `Abort` when
  the call role is normalization, deterministic reconstruction, or
  parent/history replay. `DuplicateGlobal` is always `Abort` because no
  A3-O raw code can add a global declaration.

Adding a kernel error without versioning this total map prevents profile
reminting. The closed call-role enum realizing the role side of this map
(RC1 §2.5: "kernel call roles are a closed enum; logical kernel errors
may be false only at explicitly listed proposal-check roles and are abort
at every remint, history, normalization, deterministic reconstruction,
quotient, coverage, and action-rebuild role") is `KernelCallRoleV1`,
whose variant inventory is the F-B registered decision on §13.4a item 3;
this subsection fixes only that the map is total, that its false rows are
reachable exclusively at proposal-check roles, and that no F-A schema
adds, removes, or reorders a row.

#### 2.1.5 Aborts of this subsection

Any added, removed, renamed, or reordered variant in either sum or any
subordinate enum; a stored budget-category byte; an operand value in the
arithmetic payload; a truncated, digest-substituted, or ceiling-clipped
judgment or input-byte retention; an invented truncation or width
numeral; an `Other` variant, free diagnostic payload, or platform text; a
`False` classification of an unknown or unmapped failure; a kernel-error
map row change without a version change.

### 2.2 Lesson-11 completeness re-audit (LF.3)

Lesson 11 ("Negative classifications are completeness claims. Every new
input family must re-audit the checker's full failure taxonomy") is
discharged against the closed taxonomy for every input family of every
slot-9/10/11 operation and every R2b/R2c result partition. The audit
table is keyed by protocol slot and frozen opcode; the `False` column
lists the only `FalseReasonV1` families that may inhabit that partition's
`False` arm (a genuine caller-proposal failure of the proposed raw code,
carrier, or action), and every internal, replay, infrastructure, or
resource failure of the same operation is an abort:

| Slot[local], opcode | Result partition | `False` arm | Lawful `False` families | Internal/replay failure |
| --- | --- | --- | --- | --- |
| `SourceResolution[0]` `ResolveSource` | `SourceResolutionResultV1` | yes | `Support` | `Abort` |
| `RawGeneration[0]` `EnumerateRaw` | `RawEnumerationResultV1` | none | — (enumeration is internal; no proposal exists) | `Abort` (`EnumerationCardinalityMismatch`; ordinal or count overflow aborts per A4-O §5.2) |
| `Decode[0]` `CheckCarrier` | `CarrierCheckResultV1` | yes | `Decode`; `RawGrammar` exactly under its frozen availability rule (an ill-shaped particular-input proposal; never generated by canonical enumeration) | `Abort` |
| `Decode[1]` `DecodeRaw` | `DecodeRawResultV1` | yes | `Decode`; `RawGrammar` only for a future particular-input decoder | `Abort` |
| `Build[0]` `BuildDecoded` | `BuildDecodedResultV1` | yes | `Schema`; plus the two disjoint `Anchor` false cases (see below) | `Abort` |
| `PublicLeavesAndLineage[0]` `ProjectPreEvidence` | `PreEvidenceProjectionResultV1` | none | — (pure projection of already-checked values) | `Abort` |
| `Normalization[0]` `NormalizePreEvidence` | `NormalizationResultV1` | none | — (normalization is internal; kernel errors at the normalization role are abort by the total map) | `Abort` (`NormalizationFailed`) |
| `SupportAndOwnerResolution[0]` `EnumerateSupportShape` | `SupportShapeResultV1` | none | — (structural traversal is complete and internal, rule 19) | `Abort` |
| `SupportAndOwnerResolution[1]` `ResolveSupportOwners` | `SupportOwnerResolutionResultV1` | yes | `Support` | `Abort` |
| `SupportAndOwnerResolution[2]` `CloseSupportAndCoverage` | `SupportClosureResultV1` | yes | `Support` | `Abort` |
| `FieldDisposition[0]` `CertifyPrimitiveFields` | `FieldDispositionResultV1` | yes | `Schema` (`EmptyReplacementCertificateMismatch`; every `Derived` value is false — no `rule_ordinal` can be in bounds, R2c §8) | `Abort` (exhaustion, allocation failure, an incomplete search, or absence of the R3 resource receipt constructs no certificate) |
| `SubjectConstruction[0]` `ConstructSubjects` | `SubjectConstructionResultV1` | yes | `Schema` (`TheoremSubjectTypingRejected`) | `Abort` |
| `SubjectConstruction[1]` `AssembleEnvelope` | `EnvelopeAssemblyResultV1` | none | — (assembly and canonical encoding are internal) | `Abort` (`CanonicalEncodingMismatch`, allocation, budget) |
| `SubjectConstruction[2]` `CheckCandidateMatch` | `CandidateMatchResultV1` | yes | `Compatibility` | `Abort` (late anchor-stage lineage failure is an invariant abort, see below) |
| `QuotientProjectionAndComparison[0]` `ProjectCandidateQuotient` | `QuotientProjectionResultV1` | none — proved uninhabitable | — | `Abort` suppresses the pair |
| `QuotientProjectionAndComparison[1]` `CompareQuotientKeys` | `QuotientKeyComparisonResultV1` | none — proved uninhabitable | — | `Abort` suppresses the pair |
| `ActionNormalizationAndRebuild[0]` `CheckFormalActionSyntax` | `FormalActionSyntaxCheckResultV1` | yes | `Action` (available only before an action image exists) | `Abort` |
| `ActionNormalizationAndRebuild[1]` `NormalizeAndRebuildAction` | `ActionRebuildResultV1` | yes | exactly the `FalseReasonV1::Action` family (caller-proposed base/domain/codomain/arity/typing/square failures, R2d §14.3.4.2) | `Abort` (an internally derived checked-action-trace mismatch is `Abort`, not `False`; `ActionTraceInvariantMismatch`) |
| `OutcomePartitionAndCoverage[0]` `EvaluatePairCoverage` | `PairCoverageResultV1` | none — proved uninhabitable | — | `Abort` suppresses the pair |
| `OutcomePartitionAndCoverage[1]` `ClassifyPairDisposition` | `PairDispositionResultV1` | none — proved uninhabitable | — | `Abort` is the `NoDisposition` realization (§2.4.4) |

Audit notes, each from frozen text:

1. Slot-9/11 no-`False` partitions are uninhabitable as recorded, not by
   fiat: R2d §14.1.4 — record §4 routes every quotient-projection,
   encoding, and comparison failure as an abort that suppresses the
   entire pair, and `9[0]`'s domain is already-successful rank-free
   matches; R2d §14.2.6 — a leaf-level false is a `CheckedFalse` outcome
   (data inside a successful pass), and every failure of the pass or
   classification itself is an abort, so a checked false position at pair
   level would be uninhabitable by construction. The closed taxonomy
   preserves both proofs: no `FalseReasonV1` family describes an
   internal projection, comparison, coverage, or classification failure.
2. Anchor placement per the RC1 §2.5 repair rules: immediately after
   successful decode and before build, the ordinary anchor stage reads
   the one fixed raw `OriginPathV1` leaf; `OlderPublic` and a
   `Current(q)` with `q != o` are the two disjoint anchor-false cases
   (`Anchor::PrincipalLeafOlderPublic` and
   `Anchor::AnchorOccurrenceMismatch`). Because `Decode[1]` has already
   returned `Success` at that point and partitions are per-operation,
   these two false cases surface at the next operation boundary,
   `Build[0]`'s `False` arm — consistent with the frozen R2c retention of
   `BuildPremiseEvidenceV1.anchor_path` and `principal_leaf`. Failure
   after a valid `Current(o)` principal leaf to replay exactly that one
   lineage through normalization is an invariant abort, not a second
   empty or multiple-origin false case; consequently
   `Anchor::PrincipalOriginEmpty` and `Anchor::PrincipalOriginMultiple`
   remain in the closed sum (LF.2 forbids removal) but are uninhabited by
   ordinary V1 canonical evaluation. This is recorded as an inhabitation
   constraint, not a grammar change.
3. Every input family is covered: the slot-9/10/11 inputs
   (`QuotientProjectionInputV1`, `QuotientKeyComparisonInputV1`,
   `ActionRebuildInputV1`, `FormalActionSyntaxCheckInputV1`,
   `PairCoverageInputV1`, `PairDispositionInputV1`) consume only
   already-successful frozen payloads, wire scalars, checked positions,
   and — for the two action operations — the caller-proposed action
   syntax whose failures are exactly the `Action` family. An `Abort` arm
   anywhere inside `PairCoverageInputV1.leaves` or `.comparisons` is an
   abort of `11[0]` and mints nothing (R2d §14.2.6). The R2b/R2c input
   families (`SourceResolutionInputV1`, `CarrierCheckInputV1`,
   `RawEnumerationInputV1`, `DecodeRawInputV1`,
   `FormalActionSyntaxCheckInputV1`, and the R2c §11.1 stage inputs) are
   audited by their rows above.
4. First-failure order: within a leaf the first failure is checked in the
   exact A4-O §5.3 order (decode fields in declared order; build premises
   in displayed A3-O order; `Fields_c`; normalization; structural
   support; empty-grammar primitive certificates; subjects; envelope
   assembly; anchor; compatibility), with the RC1 anchor-read repair of
   note 2; the first checked false excludes only that raw leaf and
   evaluation continues; any abort suppresses the entire pair.

### 2.3 `CheckedFalseReceiptV1` (LF.4)

#### 2.3.1 REGISTERED DECISION RD-R3-A-1 (§13.4a item 1): one shared type per receipt family, locus-discriminated internally

The underdetermination: shared versus per-position receipt types for
`CheckedFalseReceiptV1`/`AbortReceiptV1` across heterogeneous loci, and
the locus-discrimination mechanism.

Decision (registered): there is exactly one `CheckedFalseReceiptV1` type
and exactly one `AbortReceiptV1` type. Every census row of either family
consumes the same schema-DAG node; no per-position, per-slot, or per-lane
receipt type exists. Locus discrimination is internal and closed: a
closed subject sum (`FalseSubjectV1`, §2.3.2) mirroring the frozen
root-`0xe2` `LevelOneProposalV1` tag order, plus the closed
`stage:PipelineStageV1`, the validated typed path
`locus:TypedProjectionPathV1`, and the closed `reason` sums of §2.1.

Argument from frozen text:

1. Every qualified position in R2b, R2c, and the R2d record spells the
   identical name — `R3::CheckedFalseReceiptV1` or `R3::AbortReceiptV1`
   (census §§1.2–1.4). Under the frozen R2a acyclic schema DAG a name
   denotes exactly one topologically earlier node, and R2b §6.1 requires
   R3 to "replace each qualified `R3::` slot with one exact earlier
   local type without moving the R2b field": one exact type per name.
   Splitting into per-position types would retype frozen fields, which
   LF.11 forbids.
2. Heterogeneous inhabitation is already governed by the frozen partition
   grammars (which operations possess a `False` arm at all — §2.2) and
   by the frozen availability rules of the taxonomy (`RawGrammar`,
   `Action`); a shared type therefore admits no unlawful inhabitant,
   because receipts are constructed only by the operation that failed
   and are never decision inputs (LF.9, rule 23).
3. The discrimination mechanism introduces no new outer position and no
   new enum where a frozen one exists: `PipelineStageV1` (A4-O §2.3),
   `TypedProjectionPathV1` (R2a §6, the closed four-form validated path
   sum), and the `LevelOneProposalV1` tag order (R2b §4) are consumed;
   only the subject sum and the record itself are minted, by value,
   inside already-reserved positions (LF.10).

#### 2.3.2 The record

`CheckedFalseReceiptV1` closes as the exact RC1 §2.5 record — "a
checked-false receipt has an exact record containing the complete raw
code, constructor/raw ordinal, closed stage/locus/reason, optional
expected and actual checked judgments, optional kernel error, exact
relevant input bytes, and deterministic resource deltas" — in that
display order, with the first two items realized by the closed subject
sum required by RD-R3-A-1 for the non-raw-leaf lanes:

```text
FalseSubjectV1 ::=
  CarrierProposal=0(proposal:CarrierProposalV1)
| RawConstructorProposal=1(constructor:ConstructorTagV1,
                           raw_ordinal:u32,
                           raw_code:RawConstructorCodeV1)
| FormalActionProposal=2(raw_syntax:FormalActionSyntaxV1)

CheckedFalseReceiptV1 =
  (subject:FalseSubjectV1,
   stage:PipelineStageV1,
   locus:TypedProjectionPathV1,
   reason:FalseReasonV1,
   expected_judgment:Option<OpenJudgment>,
   actual_judgment:Option<OpenJudgment>,
   kernel_error:Option<KernelErrorEvidenceV1>,
   relevant_input_bytes:bytes,
   resource_deltas:ResourceDeltaVectorV1)
```

Field by field:

- `subject` — the three variants mirror the frozen `LevelOneProposalV1`
  tags (`CarrierProposal=0`, `RawConstructorProposal=1`,
  `FormalActionProposal=2`) so that every proposal lane with a `False`
  arm has exactly one subject form; the `RawConstructorProposal` arm's
  triple `(constructor:ConstructorTagV1, raw_ordinal:u32,
  raw_code:RawConstructorCodeV1)` is byte-for-byte the frozen R2c
  `CandidateMatchPremiseV1` retention triple, realizing RC1's "complete
  raw code, constructor/raw ordinal" with the frozen names and widths
  (`u32` per `persistent_ordinal_bits : u16 = 32`).
- `stage`/`locus`/`reason` — the closed stage/locus/reason: `stage` is
  the exact `PipelineStageV1` position at which the A4-O §5.3
  first-failure order (with the RC1 anchor-read repair) terminated;
  `locus` is the offending raw-field/value path (A4-O §2.3), a validated
  `TypedProjectionPathV1` that must end at the declared expected type
  and re-encode byte-identically (R2a §6) — for a raw-leaf false it is
  typically the `RawField` arm; `reason` is the §2.1.1 sum. The
  `(stage, reason-family)` pair must be a lawful row of the §2.2 audit
  table for the failing operation; any other pair is an abort. There is
  no `Other` and no free text.
- `expected_judgment`/`actual_judgment` — `Some` exactly when the failing
  check required or produced a kernel judgment ("exact expected
  judgment ... and the completed checked witness", "where applicable" —
  A4-O §2.3); complete, never truncated (RD-R3-A-3 clause 5). The
  payload type is the frozen inherited `OpenJudgment`.
- `kernel_error` — `Some` exactly when the failure is a logical kernel
  rejection at a proposal-check role per the total map of §2.1.4.
  `KernelErrorEvidenceV1` is a name minted here whose exact schema is
  the F-B registered decision on §13.4a item 5 (no
  `R3::KernelErrorReceiptV1` position exists; error receipts live only
  inside this field and its §2.4.1 twin).
- `relevant_input_bytes` — the complete canonical bytes of the exact
  value selected by `locus`, complete and deterministic (RD-R3-A-3
  clause 5); encoded as the frozen `bytes` wire form with its `u64`
  length.
- `resource_deltas` — RC1's "deterministic resource deltas".
  `ResourceDeltaVectorV1` is a name minted here; F-A fixes its
  position, presence, and content discipline — frozen counters and
  checked `u64` before/after deltas only, no wall clock, allocation
  address, scheduler state, platform text, or map order (A4-O §5.3) —
  and its internal counter layout closes in F-C under §13.4a items 7–8
  together with the account family.

The record encodes as the in-order concatenation of its fields' canonical
values under the frozen R2a wire (sum tags `u8`; ordinals `u32`; counts
and byte lengths `u64`); it is never a rooted object — no R2a §9 root
maps a receipt — and appears only nested at census positions.

#### 2.3.3 REGISTERED DECISION RD-R3-A-2 (§13.4a item 2): duplication with checked binding equations

The underdetermination: false-receipt/outer-variant redundancy —
duplication versus checked binding equations for
`raw_ordinal`/`raw_code` carried by both the `0xe7` variant and the
RC1-prescribed receipt.

Decision (registered): both copies exist — the receipt is fully
self-contained per RC1 §2.5, and the frozen `0xe7`/section-4 outer
positions are unchanged — and the redundancy is closed by registered
checked binding equations, in the exact style of the frozen section-4/5
equations. At the two outer-variant positions (census rows D1, D2), for
the fixed pair with binding constructor `c` and enumeration `R`:

```text
P[j] = CheckedFalse(j_u32, r, f)  requires:

f.subject = RawConstructorProposal(
              constructor = binding.constructor,
              raw_ordinal = j_u32,
              raw_code    = r)

f.subject.raw_ordinal = outer raw_ordinal = checked_u32(j)
f.subject.raw_code    = outer raw_code    = R[j]     (full canonical bytes)
f.subject.constructor = the pair's fixed ConstructorTagV1
                        (byte-equal to PairEvaluationBindingV1.constructor
                         and to the constructor of R[j])
```

checked at construction and re-checked at every decode; any disagreement
is an abort of the enclosing operation (the re-encode-disagreement family
of the R2d §14.2.1 abort list).

Argument from frozen text:

1. Omission from the receipt is unlawful: RC1 §2.5 and ledger line LF.4
   mandate that the receipt itself contain the complete raw code and
   constructor/raw ordinal, and by RD-R3-A-1 the same single type serves
   census positions that have no outer copy at all (rows B3, B6, B9,
   B11, B13, C26, C28, C30, C32, C35, D9 — the §1.5 item-3 false family
   minus the two outer-variant rows D1, D2), where the receipt is the
   sole carrier.
2. Omission from the outer variant is unlawful: the `0xe7`
   `CheckedFalse=1` positions `raw_ordinal`/`raw_code` are frozen R2d
   grammar bytes (LF.11; R2d §14.2.1 forbids any removed or reordered
   field).
3. Therefore both copies necessarily exist, and the only lawful closure
   is a checked binding equation — the exact device the frozen records
   already use for this data (`P[j].raw_ordinal = checked_u32(j)`,
   `P[j].raw_code = R[j]`, and R2d §14.2.1's "bound by the section-4/5
   equations").
4. The duplication creates no second authority: receipts are never
   decision inputs (LF.9); no certificate tag, density check, class
   membership, count, or disposition reads the receipt copy — density is
   checked against `enumeration.codes.len`, never against receipt bytes
   — and the binding equations make a disagreeing pair unrepresentable
   in any accepted value.

#### 2.3.4 Minting rule and aborts

`False` is minted only after the relevant computation completed without
infrastructure uncertainty (A4-O §2.3): a raw code may return `False`
only when the failing check itself ran to a definite rejection within
budget; exhaustion, overflow, allocation failure, or any incomplete
computation at the same site is an abort, never a false. A completed
prefix is never promoted.

Aborts of this subsection: a per-position or per-lane receipt type; a
subject arm added, removed, or reordered against the `LevelOneProposalV1`
tag order; a `RawConstructorProposal` subject violating any binding
equation of 2.3.3; a `(stage, reason-family)` pair outside the §2.2
table; an unvalidated or non-re-encoding locus path; a truncated
judgment or input-byte field; a receipt minted from an incomplete
computation; a rooted receipt object; an error string, optional receipt,
or implementation counter added to any frozen field (R2b §6.1); trailing
bytes, truncation, or re-encode disagreement on the closed positions.

### 2.4 `AbortReceiptV1` (LF.5)

#### 2.4.1 The record

```text
AbortReceiptV1 =
  (stage:PipelineStageV1,
   reason:AbortReasonV1,
   duplicate_census:Option<DuplicateCensusIdentityV1>,
   kernel_error:Option<KernelErrorEvidenceV1>,
   resource_deltas:ResourceDeltaVectorV1)
```

- `stage` — the exact pipeline position at abort; deterministic per the
  A4-O §5.3 identical-outcome rule ("identical outcome, locus, counters,
  and transcript").
- `reason` — the closed §2.1.2 sum; abort loci travel inside the reason
  payloads (`DefinitionRulePathV1` and the subordinate schemas of
  RD-R3-A-3), so no separate locus field exists.
- `duplicate_census` — the dedicated duplicate-census diagnostic:
  `Some(d)` exactly when `stage = SourceCensus`,
  `reason = VerifiedParentReplayRejected(SourceCensus)`, and the abort
  is the A4-O §5.1 duplicate-exact-census-identity failure; `None`
  otherwise (§2.4.3). The `(stage, reason)` byte pair alone does not
  decide presence — R1 §5 (lines 448–451) names non-duplicate
  census-stage replay failures — so the third conjunct is a
  construction obligation, and a duplicate-identity abort minted with
  `duplicate_census = None` is an abort (§2.4.6).
- `kernel_error` — `Some` exactly when the abort was caused by a kernel
  error under the total map of §2.1.4 (always for
  `KernelResourceExhausted`; for logical kernel errors at every remint,
  history, normalization, deterministic reconstruction, quotient,
  coverage, and action-rebuild role); exact form is the F-B decision on
  §13.4a item 5, shared with §2.3.2.
- `resource_deltas` — as §2.3.2: frozen counters and checked `u64`
  before/after deltas only; internal layout closes in F-C.

The same shared-type and no-rooted-object rules hold as for the false
receipt (RD-R3-A-1): one `AbortReceiptV1` node serves every abort-family
census row; no root maps it; it appears only nested at census positions.
Abort receipts stay outside every coverage certificate and disposition:
`abort_is_certificate_member : false` is a frozen row of
`CompleteCoverageGrammarV1`, an abort has no tag in the outcome or
disposition vectors (A4-O §2.4), root `0xe9` is receipt-free by
RD-RT-B-2, and `AbortReasonV1` "is outside every coverage certificate and
disposition" (A4-O §2.3).

#### 2.4.2 `BudgetExhausted` realization

Per RC1 §2.5 and ledger LF.5, the `BudgetExhausted` payload is exactly
`(counter:ResourceCounterV1, used:u64, limit:u64)` (§2.1.2/§2.1.3 clause
4): `counter` is the exact fourteen-tag `ResourceCounterV1` value in the
frozen A4-O §2.2 order; `limit` is byte-equal to the frozen ceiling row
of the governing resource policy for that counter; `used` is the checked
would-be total whose charge first strictly exceeded `limit` under the
A4-O §7 accounting order (every unit charged exactly once at its frozen
counter; per-envelope counters reset only at the start of a new raw leaf
while pair totals continue monotonically; no kernel or normalizer call
receives a fresh invisible pair budget). `KernelResourceExhausted`
remains the distinct kernel-side exhaustion variant with its closed
three-tag payload; neither is ever evidence.

#### 2.4.3 The dedicated duplicate-census receipt

Per RC1 §2.5 ("duplicate current/older census identities have one
dedicated abort receipt with source class and the two checked `u32`
ordinals") and LF.5, the dedicated receipt closes as the shared
`AbortReceiptV1` carrying the dedicated diagnostic payload:

```text
SourceClassV1 ::= Current=0 | OlderPublic=1

DuplicateCensusIdentityV1 =
  (source_class:SourceClassV1,
   first_ordinal:u32,
   second_ordinal:u32)
```

with the fixed presence equation: `duplicate_census = Some(d)` exactly
when `stage = SourceCensus` and
`reason = VerifiedParentReplayRejected(SourceCensus)` arising from a
duplicate exact census identity, and `None` otherwise. The reason
assignment is from frozen text: "Duplicate exact identities are an
aborting history-replay failure" and "Duplicate exact public identities
abort" (A4-O §5.1) — a history-replay failure surfacing at the source
census stage. `SourceClassV1` mirrors the frozen R1 `SourceRefV1` tags
(`Current` tag `0x00`, `OlderPublic` tag `0x01`); the two ordinals are
positions in the one frozen sorted census vector of that class (`U_b` or
`Pub_<b(H)`, A4-O §5.1), `u32` per `persistent_ordinal_bits : u16 = 32`
and the frozen `SourceLeafV1.source_ordinal:u32`, with
`first_ordinal < second_ordinal` checked against the frozen census
order. No other reason may carry this payload, and no
duplicate-exact-identity abort may omit it: because R1 §5 (lines
448–451) also names non-duplicate census-stage replay failures
("Missing history projections, birth/stage/prefix/traversal/context
drift, an opaque-signature replay failure, or a private-body request
constructs no census"), presence is not a function of the
`(stage, reason)` bytes alone; the presence law of §2.4.1 is this one
law restated, checked at construction, with both violation directions
in the §2.4.6 abort list.

#### 2.4.4 `NoDisposition` realization, unchanged

The A4-O §5.3 unprivileged diagnostic `NoDisposition(AbortReasonV1)` is
realized exactly as frozen by R2d §14.2.5: it is the `Abort=1` arm of the
slot-`11[1]` result partition `PairDispositionResultV1` (census row D7),
whose payload is this `AbortReceiptV1` carrying the `AbortReasonV1`. It
is never a rooted `0xe9` value, never a fourth disposition variant, and
never a promotion of a completed prefix. F-A changes nothing about this
realization; it supplies only the receipt schema that inhabits the frozen
arm. Any abort anywhere in the pair suppresses the whole pair
disposition (rule 28); on abort no complete census, class partition,
pair coverage, or disposition exists, and a partially filled buffer has
no canonical status (R2d §4).

#### 2.4.5 REGISTERED DECISION RD-R3-A-4 (§13.4a item 11): Unknown is the absence of a disposition plus the unprivileged diagnostic

The underdetermination: the registered statement that Unknown is exactly
the absence of a disposition plus the unprivileged abort diagnostic — no
Unknown marker, fourth variant, or rooted diagnostic at any position.

Decision (registered): in ordinary V1, "Unknown" names exactly the state
in which no `PairDispositionV1` value exists for the pair and the only
artifact is the unprivileged `AbortReceiptV1` diagnostic at the frozen
result-partition `Abort` arm (`NoDisposition` realization, §2.4.4).
There is no Unknown marker byte, no Unknown outcome tag, no fourth
disposition variant, and no rooted diagnostic object at any position, and
R3 introduces none.

Argument from frozen text:

1. The frozen grammars already exclude every marker form: the field-15
   `CompleteCoverageGrammarV1` fixes
   `outcome_tags : [CheckedFalse,Match]`,
   `disposition_tags : [CertifiedNoMatch,UniquePositive,TypedAmbiguity]`,
   and `abort_is_certificate_member : false` — "An abort therefore has
   no tag in the outcome or disposition vectors" (A4-O §2.4); `Abort` is
   deliberately absent from the §5.3 outcome vector; R2d §14.2.5
   excludes an `Abort` or `NoDisposition` variant inside the rooted
   `0xe9` sum; and no R2a §9 root maps any receipt. Minting a marker,
   fourth variant, or rooted diagnostic would change closed grammar
   bytes, which LF.11 forbids.
2. The frozen epistemics require exactly this reading: `Unknown` is
   successful safety — it identifies the missing proof or resource
   without manufacturing authority (lesson 1); finite replay is not a
   universal theorem (lesson 5); "Resource failure is epistemically
   neutral and never evidence for `Primitive`, `CertifiedNoMatch`, or
   underivability" (A4-O §7). A representable Unknown value inside any
   certificate or disposition would convert the absence of evidence into
   an evidence-bearing byte, exactly the promotion the frozen records
   forbid ("A completed prefix is never promoted", A4-O §5.3; "it cannot
   be relabeled as a zero row", A4-O §5.5).

#### 2.4.6 Aborts of this subsection

A `BudgetExhausted` payload without the exact `ResourceCounterV1` tag or
with a stored category byte; a `limit` differing by any byte from the
frozen policy ceiling; a duplicate-census diagnostic under any other
reason or stage, with a non-census ordinal, or with
`first_ordinal >= second_ordinal`; a duplicate-exact-identity census
abort whose `duplicate_census` is `None`; an abort receipt inside a
coverage certificate, disposition, quotient key, or any rooted object;
an Unknown
marker, fourth variant, or rooted diagnostic; a `NoDisposition`
realization anywhere other than the frozen `11[1]` abort arm; an
exhaustion, overflow, or allocation failure classified as `False` or as
evidence; trailing bytes, truncation, or re-encode disagreement on the
closed positions.

### 2.5 F-A closing statement

F-A fixes exactly: the total reservation census over both notations with
its totality rule and the three agreement checks (LF.1, §1); the closed
reason taxonomy — `FalseReasonV1` and `AbortReasonV1` with exactly the
A4-O §2.3 variant sums and orders, every subordinate locus/payload schema
with exact widths under registered decision RD-R3-A-3, every locus a
validated typed path, no `Other`, no free diagnostic payload, no platform
text, unknown or unmapped failure `Abort` never `False`, and the total
kernel-error map unchanged (LF.2, §2.1); the lesson-11 completeness
re-audit of every input family of every slot-9/10/11 operation and every
R2b/R2c result partition, with each `False` arm inhabited only by a
genuine caller-proposal failure, every internal/replay failure an abort,
the slot-9/11 no-`False` partitions uninhabitable as recorded, `10[1]`
`False` exactly the `FalseReasonV1::Action` family, and the anchor-read
inhabitation constraint recorded (LF.3, §2.2); `CheckedFalseReceiptV1` as
the exact RC1 §2.5 record under registered decisions RD-R3-A-1 (one
shared type per receipt family with internal closed locus discrimination)
and RD-R3-A-2 (duplication closed by checked outer-variant binding
equations), with the false-minting rule (LF.4, §2.3); and
`AbortReceiptV1` with the exact `BudgetExhausted` realization, the
dedicated duplicate-census receipt, the unchanged `NoDisposition`
realization at the frozen `11[1]` abort arm, and registered decision
RD-R3-A-4 (Unknown is the absence of a disposition plus the unprivileged
diagnostic; no marker, fourth variant, or rooted diagnostic) (LF.5,
§2.4).

Global-line checks for F-A's material:

- LF.9 — no F-A schema makes any receipt or account a decision input: no
  quotient key, tag vector, class, count, disposition, flattening,
  rebuild, or opcode value depends on receipt bytes; the RD-R3-A-2
  binding equations are consistency checks on retained evidence, not
  decision edges; the frozen 10.22.4/RT-A/RT-B/RT-C independence results
  extend verbatim — holding pair inputs fixed while varying only
  receipt/account material leaves every quotient key, tag vector, class
  row, witness, and disposition byte-unchanged; receipts and accounts
  remain non-quotient fields (rule 23).
- LF.10 — containment unchanged: F-A mints zero new roots and zero new
  R3-qualified positions; every F-A type is defined by value inside
  already-reserved census positions; the filled material nests only under
  the twelve R2a §9 corrigendum roots (`0xe0`, `0xe3`, `0xe5`, `0xe6`,
  `0xe7`, `0xe8`, `0xf8`, `0xf9`, `0xfa`, `0xfb`, `0xfc`, `0xfe`);
  `0xfd`, `0xe9`, and `0xff` remain receipt-free; the three inherited
  `0xfe` channels and the per-variant root-`0xe3` containment are
  unchanged.
- LF.11 — no closed grammar byte, tag, field order, ownership edge,
  scalar width, opcode, predecessor vector, or registered decision
  changed; every F-A type (`FalseReasonV1`, `AbortReasonV1`,
  `ArithmeticOpV1`, `KernelResourceKindV1`, `PipelineStageV1`,
  `ProfileParentTagV1`, `ResourceCounterV1`, `CodecObjectV1`,
  `ProfileBoundaryV1`, `FalseSubjectV1`, `CheckedFalseReceiptV1`,
  `SourceClassV1`, `DuplicateCensusIdentityV1`, `AbortReceiptV1`) is
  defined topologically earlier under the R2 scalar codec than every
  frozen position consuming it, with leaf fields drawn only from frozen
  earlier types (`ConstructorTagV1`, `RawConstructorCodeV1`,
  `CarrierProposalV1`, `FormalActionSyntaxV1`, `OpenJudgment`,
  `ParentSlotV1`, `DefinitionRulePathV1`, `TypedProjectionPathV1`, wire
  scalars) and the two F-A-named, F-B/F-C-closed nodes
  (`KernelErrorEvidenceV1`, `ResourceDeltaVectorV1`). Because those two
  nodes and the F-B/F-C families remain open, no root holding a
  qualified position — and no `trace_commitment` value — has complete
  canonical bytes at F-A close.
- LF.12 — stop boundary held: no numerical `TypeIdV1`, no registry or
  source join, no fixture or mutation bytes, no metered batch API (the
  meter remains unavailable; `resource_deltas` is a logical schema
  position, not a meter), no executable authority, no verified profile
  token, no theorem; the three preservation descriptors remain
  uninhabited typed syntax and the record-§10.19.6 truth-audit gate
  persists unweakened.
- LF.13 — the transitive forbidden-input scan over §§1–2 of this record
  ran at F-A close and passes: no F-A definition consumes a
  representative, copied ordinal (every `raw_ordinal`/census-ordinal
  field is a checked position bound by registered equations, the RT-B
  pattern), construction transcript, replay-as-input, semantic SR2
  result, `nu`, `GCap`/`gamma` or other semantic-novelty datum, QG2
  stored route vector, source or
  syntax copy outside the frozen retention positions, or numerical
  `TypeIdV1`; nor any remaining family of the frozen R2d §10.21.6
  union — a support-to-lineage, footprint-to-owner, or
  lineage-to-stream back-edge, an evidence target, or an R3 account or
  receipt consumed as input — zero hits in each; zero new admission
  values beyond QG2's two.

F-A does NOT close: F-B — the `KernelCallRoleV1` variant inventory and
its alignment with the kernel-error map rows (§13.4a item 3),
`KernelSuccessReceiptV1` content under the meter tension (item 4), the
exact `KernelErrorEvidenceV1` form inside the two optional kernel-error
fields (item 5), `StructuralCardinalityReceiptV1`, the count-only
traversal evidence family, and the prefix-node-count residence (item 6;
LF.6–LF.7); F-C — the internal layout of `EnvelopeResourceAccountV1`,
`PairResourceAccountV1`, and `ActionResourceAccountV1` with
`ResourceDeltaVectorV1` (item 7), the meter unit/granularity and charge
schema with the per-envelope reset (item 8), the exact `Aggregate_R3`
conservation equations and child orders (item 9), the closed scope-tag
enum and `EmptyReplacementField(i)` encoding (item 10), the
material/depth recurrence realization (item 13), the static
failure/resource definitions nested under `0xe0`/`0xf8`/`0xf9` and their
field-13 reconciliation (item 14), the fourteen-counter mapping and
field-17 metering equality (LF.8); and, per LF.12, numerical `TypeIdV1`
assignment and the registry/source join (R2e), fixture and mutation
regeneration (R4), any executable JG2b2b2b authority, and any theorem.
This sub-cut mints no verified profile, executable interface, envelope
census, pair disposition, occurrence classification, action image
instance, theorem, generic law, cubical bridge, `GCap`, `gamma`, or
selective authority. The next lawful move is F-B, the kernel-role/replay
and cardinality-receipt lane, under this same census.

## 3. F-B: the kernel-role, replay, and cardinality lane

This sub-cut closes ledger lines LF.6 and LF.7 under the §1 census and
the §13.4a assignments (items 3, 4, 5, 6 as RD-R3-B-1, RD-R3-B-2,
RD-R3-B-3, RD-R3-B-4). It consumes §§1–2 unchanged and stays consistent
with RD-R3-A-1 through RD-R3-A-4. The census rows it fills are exactly
the kernel-role rows B1, C1, C3; the kernel-success-receipt rows B2, C2,
C4, C5, C18; and the structural-cardinality rows B5, C8, C9, C12, C17,
C21, C22 — plus the closure of `KernelErrorEvidenceV1`, the F-A-named
node inside the two optional kernel-error fields of the already closed
false/abort receipts (§2.3.2, §2.4.1), which occupies no census row of
its own. Every remaining unfilled census row (the ten
`EnvelopeResourceAccountV1` rows C6, C7, C10, C11, C13, C14, C15, C16,
C19, C20, and the account rows D5, D8) is explicitly deferred to the
named F-C sections per the §1.1 totality rule. The inherited transitive
channels of §1.5 (roots `0xe3`, `0xe5`, `0xe6`, `0xfb`, `0xfc`, `0xfe`)
reuse these defining occurrences and add no position.

### 3.1 `KernelCallRoleV1` (LF.6)

#### 3.1.1 REGISTERED DECISION RD-R3-B-1 (§13.4a item 3): the exact variant inventory, aligned with the kernel-error map rows

The underdetermination, catalogued at plan-§13.4a registration and
assigned to F-B: the exact `KernelCallRoleV1` variant inventory and its
alignment with the A4-O kernel-error map rows.

Decision (registered): `KernelCallRoleV1` closes as a closed
payload-free enum — RC1 §2.5's "kernel call roles are a closed enum" —
encoded as one `u8` tag in zero-based displayed order, with exactly
these variants in two registered groups:

```text
KernelCallRoleV1 ::=
  ProposalDecodeCheck=0
| ProposalBuildCheck=1
| ProposalSubjectsCheck=2
| ProposalActionCheck=3
| ProfileRemint=4
| HistoryReplay=5
| Normalization=6
| DeterministicReconstruction=7
| Quotient=8
| Coverage=9
| ActionRebuild=10
```

Variants 0–3 are the explicitly listed proposal-check roles that RC1
§2.5 demands ("logical kernel errors may be false only at explicitly
listed proposal-check roles"); variants 4–10 are the abort-only roles,
spelled and ordered verbatim from the same frozen sentence ("abort at
every remint, history, normalization, deterministic reconstruction,
quotient, coverage, and action-rebuild role"). The variant names are
minted by this record (R3 owns the type; R2b §6.1: "R3 owns call-role
and resource receipts, but R2b fixes their outer positions"); every
name component is a frozen spelling — the four proposal-check names are
`Proposal` + the frozen `PipelineStageV1` stage name + `Check`, and the
proposal-check group is displayed in frozen `PipelineStageV1` stage
order (`Decode=5`, `Build=6`, `Subjects=10`, `Action=16` in the §2.1.3
display).

Argument from frozen text:

1. The proposal-check list is derived, not chosen: A4-O §2.3 maps a
   logical kernel error "to the enclosing typed-false stage only for a
   candidate-derived judgment", and the exact kernel rejections
   representable in `FalseReasonV1` are its `*Rejected` variants ("exact
   kernel rejection where applicable", A4-O §2.3) — `Decode` family
   (`TypeFormationRejected`, `HasTypeRejected`, `EqualityRejected`),
   `Schema` family (`OrderedPremiseRejected`,
   `ParticularSubstitutionRejected`, `ParticularLiftRejected`,
   `RawOutputTypingRejected`, `ComparisonRejected`,
   `TheoremSubjectTypingRejected`), and `Action` family
   (`ActionTypingRejected`, `SquareEqualityRejected`); every
   `*Mismatch`/`*Unavailable`/structural variant is a non-kernel byte or
   shape comparison. Under the §2.2 audit table of this record (LF.3) a
   `*Rejected` variant is lawful at exactly the operations whose stages
   are `Decode` (`Decode[0]`, `Decode[1]`), `Build` (`Build[0]`),
   `Subjects` (`SubjectConstruction[0]`), and `Action` (`10[0]`,
   `10[1]`). Stage `Fields` is excluded even though `FieldDisposition[0]`
   has a `Schema`-family `False` arm: the only variant lawful there is
   `EmptyReplacementCertificateMismatch` — a shape comparison, not a
   kernel rejection ("a checked ordinary candidate rejects every
   `Derived` value: no `rule_ordinal` can be in bounds", R2c §8) — and
   the frozen field-disposition transcript makes no kernel call at all
   (`T.typing_replays = []`; "no typing call is made and the replay
   vector is canonically empty", R2c §8), so no candidate-derived kernel
   rejection can surface as `False` at `Fields` and no
   `ProposalFieldsCheck` role exists to list. Those four stages are the
   complete proposal-check role list; no fifth stage can lawfully
   surface a kernel rejection as `False`, and a vacuous fifth variant
   would be a pre-drilled hole in a V1 enum — the device argument 2 of
   RD-R3-B-2 rejects — while a replacement grammar with rules is a rule
   44 (`AnyByteOrSemanticChangeRequiresVersionChange`) version boundary,
   never a reserved role.
2. The abort-only list is verbatim RC1 §2.5 and refines, without
   contradicting, the coarser A4-O §2.3 third row ("when the call role
   is normalization, deterministic reconstruction, or parent/history
   replay"): `ProfileRemint` and `HistoryReplay` jointly realize
   "parent/history replay" as the two frozen remint/history duties, and
   `Quotient`, `Coverage`, and `ActionRebuild` are abort-only because
   no call at those roles checks a candidate-derived proposal judgment
   (A4-O §2.3 row two grants `False` "only for a candidate-derived
   judgment"; RC1 names all seven explicitly).
3. Roles are per-call duties, not per-operation labels. Within `10[1]`
   (`NormalizeAndRebuildAction`) a call that type-checks the newly
   appended caller-proposed action carries `ProposalActionCheck` — this
   is how the frozen R2d §14.3.4.2 caller-proposed
   "base/domain/codomain/arity/typing/square failures" remain `False`
   at `10[1]` — while every deterministic-rebuild constructor
   invocation carries `ActionRebuild` and its kernel errors abort, as
   RC1 requires. Likewise the R1 §6 source-view checks ("Only after
   those prerequisites close may a candidate-derived
   `TypeView`/`TermView` judgment be a checked `False`") carry
   `ProposalDecodeCheck` and surface at the `Decode`-stage `False` arms
   of the §2.2 table, while `SourceResolution[0]`'s own `False` arm
   remains the structural `Support` family with no kernel rejection —
   exactly as F-A recorded.

#### 3.1.2 The total role map

The alignment demanded by item 3 is this total two-axis map, which
realizes the three rows of the frozen A4-O §2.3 kernel-error map
(consumed unchanged in §2.1.4) over the closed role enum; every
`(kernel error, role)` cell is decided:

```text
| Kernel-error map row (A4-O §2.3)          | roles 0-3 (proposal-check) | roles 4-10 (abort-only) |
| ----------------------------------------- | -------------------------- | ----------------------- |
| ResourceExhausted(Operations|Depth|       | Abort                      | Abort                   |
|   Normalization), InvalidLimits,          |                            |                         |
|   UniverseOverflow                        |                            |                         |
| UnboundVariable, InvalidSubstitution,     | False at the enclosing     | Abort                   |
|   SubstitutionArity, UnknownGlobal,       |   typed-false stage (the   |                         |
|   ExpectedType, ExpectedFunction,         |   role's own stage, 3.1.3) |                         |
|   ExpectedPair, TypeMismatch,             |                            |                         |
|   WrongJudgmentForm                       |                            |                         |
| DuplicateGlobal                           | Abort (always Abort:       | Abort                   |
|                                           |   no A3-O raw code can add |                         |
|                                           |   a global declaration)    |                         |
```

The map is total, its `False` cells are reachable exclusively at
proposal-check roles, and no row is added, removed, or reordered —
exactly the three properties F-A fixed in §2.1.4 and deferred here.
Adding a kernel error without versioning this total map prevents
profile reminting (A4-O §2.3, unchanged).

#### 3.1.3 Coherence equations and inhabitation constraints

All checked at construction and re-checked at every decode:

1. Role-stage equation: a receipt or error evidence carrying
   `ProposalDecodeCheck` has stage `Decode`; `ProposalBuildCheck` —
   `Build`; `ProposalSubjectsCheck` — `Subjects`;
   `ProposalActionCheck` — `Action`. Abort-only roles carry the stage
   at which the call was issued, unconstrained by this equation (a
   `Normalization`-role call may lawfully occur at stage `Decode`: R1
   §6's "view/check/normalization calls"). Inhabitation constraint: no
   role carries stage `Fields` in ordinary-V1 canonical evaluation —
   the frozen field-disposition transcript retains `typing_replays =
   []` and no other kernel vector ("no typing call is made and the
   replay vector is canonically empty", R2c §8), and receipt vectors
   are exhaustive per scope (3.2.2) — so no lawful success receipt,
   replay, or kernel-error evidence carries stage `Fields`; recorded as
   an inhabitation constraint in the exact §2.2 note-2 device, not a
   grammar change.
2. `CheckedSignatureReplayV1.call_role` (row C1) is `ProfileRemint` or
   `HistoryReplay` only; `CheckedContextReplayV1.call_role` (row C3) is
   `HistoryReplay` only — both records are the frozen A1 field-7
   parent/history replay evidence (R2c §4), and R1 §6 fixes that a
   current local context "is replayed under both its strict prefix and
   `Sigma`" before use.
3. `NormalizationStepEvidenceV1.KernelNormalization` replays, the
   endpoint replays (`RawNormalizationEndpointReplayV1`,
   `NormalizedEnvelopeEndpointReplayV1`), and every second-pass
   idempotence replay carry `Normalization`.
4. `boundary_kernel_replays` (`ProjectPreEvidence` calls, R2c §9.2)
   carry `DeterministicReconstruction`: `PreEvidenceProjectionResultV1`
   has no `False` arm (§2.2) — a pure projection of already-checked
   values proposes nothing.
5. `SubjectConstructionTranscriptV1.node_type_replays` elements (row
   C5) carry `ProposalSubjectsCheck`; `CheckedFormalActionSyntaxV1`
   `replay_evidence` elements carry `ProposalActionCheck`.
6. General law: a `Proposal*` role is lawful only on a call whose
   checked judgment is candidate/proposal-derived inside an operation
   whose §2.2 row admits the corresponding `*Rejected`-bearing family;
   every call that replays an already-checked judgment carries the
   replay/reconstruction role of its duty. A `Proposal*` role violating
   this law is an abort.

`KernelCallRoleV1` is representative evidence, never a quotient field:
the frozen R2d quotient projections "erase `call_role`, `raw_wire`,
`raw_context`, every receipt" (R2d line 1123), and rule 23 holds
unchanged.

#### 3.1.4 Aborts of this subsection

Any added, removed, renamed, reordered, or payload-bearing variant; a
wider or split tag encoding; a `Proposal*` role whose stage violates
the role-stage equation; a success receipt, replay, or kernel-error
evidence carrying stage `Fields` (constraint 1: the frozen
field-disposition transcript makes no kernel call, R2c §8); a logical
kernel error classified `False` at
any role 4–10; a `DuplicateGlobal`, `InvalidLimits`,
`UniverseOverflow`, or `ResourceExhausted` value classified `False` at
any role; a C1/C3 `call_role` outside constraint 2; a `Proposal*` role
on a replay of an already-checked judgment; a kernel-error map row
change without a version change; a role consumed as a quotient key,
tag vector, class, count, or disposition input.

### 3.2 `KernelSuccessReceiptV1` (LF.6)

#### 3.2.1 REGISTERED DECISION RD-R3-B-2 (§13.4a item 4): call identity only — no per-call meter deltas, no reserved meter fields

The underdetermination: `KernelSuccessReceiptV1` content under the
meter tension — per-call meter deltas versus call identity only versus
reserved meter fields (RC1 demands deterministic resource deltas; R2b
forbids implementation counters; A4-O records that the current kernel
returns no usage receipt).

Decision (registered): the receipt carries call identity only:

```text
KernelSuccessReceiptV1 =
  (call_role:KernelCallRoleV1,
   stage:PipelineStageV1,
   call_ordinal:u32)
```

No per-call operation count, normalization-fuel count, delta, duration,
or reserved/optional meter field exists in this record, and none may be
added without a version change.

Argument from frozen text:

1. Per-call meter deltas are unlawful now: A4-O §2.2 (lines 219–227)
   fixes that "the current kernel keeps its operation and normalization
   counters private to each public call and does not yet return a usage
   receipt", that the first JG2b2b2b implementation obligation is the
   non-authoritative metered batch API, and that "Estimating usage,
   granting each call a fresh invisible allowance, or treating the
   number of calls as the operation count cannot satisfy this V1
   profile"; RC1 §2.5 confirms "The current kernel exposes no exact
   usage receipt across multiple public calls." A per-call delta field
   could therefore be inhabited only by an estimate or an invented
   numeral — both defects — and the meter is unavailable at this cut
   (LF.12).
2. Reserved meter fields are unlawful too: R2b §6.1 forbids adding "an
   error string, optional receipt, or implementation counter" to any
   frozen field (restated by LF.6); R2a §12 forbids faking an R3 value
   "with arbitrary bytes", which is what an uninhabitable reserved
   width would be; and when the meter arrives, A4-O §2.2 already
   mandates that "the kernel protocol digest and every dependent parent
   must then be reminted" — a schema evolution owned by rule 44
   (`AnyByteOrSemanticChangeRequiresVersionChange`) at a version
   boundary, never by a pre-drilled hole in a V1 record.
3. RC1's deterministic-resource-deltas demand is discharged at its own
   frozen positions, not here: the RC1 §2.5 receipt sentence attaches
   "deterministic resource deltas" to the checked-false receipt —
   realized by the F-A `resource_deltas:ResourceDeltaVectorV1` fields
   of §2.3.2/§2.4.1 — and pair/envelope resources are aggregate by rule
   36 (`PairAndEnvelopeResourcesAreAggregate`), charged once at their
   frozen counters under the A4-O §7 accounting order into the
   `Acct_R3`/`Aggregate_R3` accounts. Kernel operations and
   normalization fuel are charged at the aggregate ledgers; a per-call
   success receipt is the wrong scope for any counter.
4. The three identity fields are each forced. `call_ordinal:u32` is the
   frozen call-identity device — `SourceResolutionSuccessEntryV1`
   spells `call_ordinal:u32`, and contiguous checked `u32` ordinals are
   the frozen R2b comparison discipline — recomputed as a checked
   position, never copied (the RT-B pattern, LF.13). `call_role` and
   `stage` make the receipt self-describing at the two frozen
   receipt-only positions (rows C5, C18), which carry bare
   `R3::KernelSuccessReceiptV1` vectors with no sibling `call_role`
   field; because the same single schema node serves every position of
   the family (the RD-R3-A-1 same-name/one-node rule applies verbatim:
   every position spells `R3::KernelSuccessReceiptV1`), the role must
   live inside the receipt. At the replay positions (rows B2, C2, C4)
   the enclosing record's frozen `call_role` field then coexists with
   the receipt copy, and the redundancy is closed by a checked binding
   equation in the exact RD-R3-A-2 style:

   ```text
   KernelSuccessReplayV1.call_role       = receipt.call_role
   CheckedSignatureReplayV1.call_role    = receipt.call_role
   CheckedContextReplayV1.call_role      = receipt.call_role
   ```

   checked at construction and re-checked at every decode; any
   disagreement is an abort of the enclosing operation.
5. The receipt retains no judgment, no digest, and no kernel
   configuration copy: the exact raw/normalized judgments live at the
   frozen `KernelSuccessReplayV1` fields (R2b §6.1) and are compared as
   full canonical bytes (rules 38/39: digests never replace evidence);
   the kernel protocol and configuration identities are already bound
   once, at manifest fields 9–11 (A4-O §3.1), with disagreement aborted
   by `KernelProtocolMismatch`/`KernelConfigurationMismatch` — a stored
   per-call copy would retain a value derivable from an identity
   already bound once, forbidden by the derivable-stored-value
   inference registered at §2.1.3 clause 4 from the audited RC1 §2.5
   category defect.

#### 3.2.2 Receipt-vector exhaustiveness and reconciliation (R2c §9.2)

The frozen rule (R2c lines 3092–3095) — "Checker/resource receipt
vectors are exhaustive in deterministic pipeline/call order and are
reconciled with the one aggregate account; R3 closes their internal
schemas and conservation rules without moving any field above" — closes
as follows:

1. Order and totality: `call_ordinal` is the zero-based position of the
   call in the deterministic kernel-call stream of its enclosing scope
   — the exact stream the frozen R2c event compilers fix
   (`BoundaryKernelCallEvents`, `ObligationKernelCallEvents`, and the
   per-scope streams of §1.1) and, for the R2b decode/build replays,
   the frozen R2b evidence order. Within every frozen vector that
   retains a scope's calls exhaustively, ordinals are contiguous
   checked `u32` positions (`receipts[k].call_ordinal =
   checked_u32(k)`); ordinal or count overflow aborts (A4-O §5.2).
2. Exhaustiveness instances, from frozen text: `boundary_kernel_replays`
   is "the schema-preorder vector of every complete
   `KernelSuccessReplayV1` nested in
   `public_leaves_and_lineage.replay_boundaries`, stopping descent at
   each such replay. No other replay is present" (R2c §9.2); the
   obligation transcript's vectors are "exactly the successful checks
   required by those rows, in the same obligation order and then their
   internal left-to-right order"; `assembly.checker_receipts=[]`
   exactly, because assembly "makes no new kernel call" (R2c §9.2) —
   row C18 is inhabited by the empty vector of this closed type, never
   by a nonempty one.
3. Reconciliation: each receipt corresponds one-to-one and
   order-preservingly to the kernel-call event at the same position of
   its scope's frozen event stream, and the count equality (receipt
   count = kernel-call event count, per scope) is checked. A receipt
   with no event, an event with no receipt, or an order disagreement is
   an abort. The conservation equations of the accounts themselves —
   what each event charges and how child accounts aggregate — close in
   F-C (§13.4a items 8, 9) without altering this correspondence law.

The receipt is never a rooted object — no R2a §9 root maps it — and
appears only nested at census positions and through the §1.5 inherited
channels. Wire form: the in-order concatenation `u8` role tag, `u8`
stage tag, `u32` ordinal under the frozen R2a scalar codec.

#### 3.2.3 Aborts of this subsection

A per-call operation, fuel, delta, duration, or reserved meter field; a
receipt inhabited by an estimated or invented numeral; a binding
equation of 3.2.1 clause 4 violated; a noncontiguous, copied, or
overflowed `call_ordinal`; a receipt vector that omits a call of its
scope, contains a foreign call, or disagrees with its account event
stream in count or order; a nonempty `assembly.checker_receipts`; a
judgment, digest, or configuration copy inside the receipt; an error
string, optional receipt, or implementation counter added to any
frozen field (R2b §6.1, LF.6); a rooted receipt object; trailing
bytes, truncation, or re-encode disagreement on the closed positions.

### 3.3 REGISTERED DECISION RD-R3-B-3 (§13.4a item 5): the error-receipt realization inside the optional kernel-error fields

The underdetermination: no `R3::KernelErrorReceiptV1` position exists
anywhere in the frozen records (verified by the §1 sweep), so error
receipts must be realized inside the false/abort families' "optional
kernel error" field — the exact form is to be registered.

Decision (registered): kernel-error evidence closes as the following
pair of types, inhabiting exactly the two F-A optional fields
(`CheckedFalseReceiptV1.kernel_error` and `AbortReceiptV1.kernel_error`,
§2.3.2/§2.4.1) and no outer position anywhere:

```text
KernelErrorTagV1 ::=
  ResourceExhausted=0(kind:KernelResourceKindV1)
| InvalidLimits=1
| UniverseOverflow=2
| UnboundVariable=3
| InvalidSubstitution=4
| SubstitutionArity=5
| UnknownGlobal=6
| ExpectedType=7
| ExpectedFunction=8
| ExpectedPair=9
| TypeMismatch=10
| WrongJudgmentForm=11
| DuplicateGlobal=12

KernelErrorEvidenceV1 =
  (call_role:KernelCallRoleV1,
   error:KernelErrorTagV1)
```

Clause by clause, with the argument from frozen text:

1. The variant list of `KernelErrorTagV1` is exactly the error names of
   the total A4-O §2.3 `pen-kernel::KernelError` map, in displayed
   map-row order (first row, then second row, then the one error the
   third row adds), with no error added, removed, renamed, or
   reordered; adding one without versioning the map prevents profile
   reminting (A4-O §2.3). `ResourceExhausted` carries the closed F-A
   `KernelResourceKindV1` (`Operations=0 | Depth=1 | Normalization=2`)
   — no new enum is minted where a closed one exists (the RD-R3-A-1
   clause-3 discipline). Tags are `u8` in zero-based displayed order.
2. `KernelErrorEvidenceV1` retains the role and the exact error
   identity, nothing else: no platform error text exists anywhere in
   the sum (A4-O §2.3: "none contains platform error text"); no
   kernel-side term or judgment copy exists, because the exact expected
   and actual checked judgments already live at the §2.3.2
   `expected_judgment`/`actual_judgment` fields of the false receipt —
   this record's realization of RC1 §2.5's "optional expected and
   actual checked judgments" — and duplicating them here would create a
   second copy; no `stage` field exists, because the enclosing receipt already
   carries `stage` (§2.3.2/§2.4.1); and no failing-call ordinal exists,
   because the deterministic A4-O §5.3 first-failure order plus the
   receipt's `stage`/`locus` already determine the failing call — a
   stored ordinal would retain a derivable value, forbidden by the
   §2.1.3 clause-4 derivable-stored-value inference and by the RT-B
   checked-position-never-copied pattern (LF.13).
3. Presence and coherence equations, all checked at construction and
   every decode, jointly realizing the total map of 3.1.2:

   ```text
   CheckedFalseReceiptV1.kernel_error = Some(e)  requires:
     e.call_role in {ProposalDecodeCheck..ProposalActionCheck}
     e.call_role = the proposal-check role of the receipt's stage
                   (role-stage equation, 3.1.3)
     e.error in the second map row (logical errors only)
     reason is the matching *Rejected variant lawful for that
       (stage, family) row of the §2.2 audit table of this record

   AbortReceiptV1.kernel_error = Some(e)  requires:
     map(e.error, e.call_role) = Abort in the 3.1.2 table
     — always satisfied for the first row and DuplicateGlobal;
       for a second-row error it requires a role 4-10

   reason = KernelResourceExhausted(kind)  iff
     kernel_error = Some(e) with e.error = ResourceExhausted(kind'),
     kind byte-equal to kind'    (the §2.4.1 "always Some" law)
   ```

   The map fixes classification (`False` versus `Abort`) only; no
   frozen text binds a bijection between kernel errors and
   `AbortReasonV1` variants beyond the `KernelResourceExhausted`
   equation above, so none is minted — the abort reason remains the
   enclosing operation's §2.1.2 reason under the F-A rules, with the
   exact kernel identity retained here.
4. No new outer position: both fields are `Option` values inside the
   two F-A receipt records, which themselves inhabit only census
   positions; no root maps kernel-error evidence, and the §1.5
   containment is unchanged (LF.10).

Aborts of this subsection: any added, removed, renamed, or reordered
error variant; a wider or split tag; platform text, a term/judgment
copy, a stage or ordinal field, or any free payload; a `Some`
kernel-error on a false receipt at a role 4–10 or with a first-row or
`DuplicateGlobal` error; a `Some` kernel-error violating any presence
or byte-equality equation above; a `KernelResourceExhausted` abort
whose `kernel_error` is `None` or kind-disagreeing; a kernel-error
value at any position other than the two frozen optional fields.

### 3.4 `StructuralCardinalityReceiptV1` and the count-only traversal family (LF.7)

#### 3.4.1 REGISTERED DECISION RD-R3-B-4 (§13.4a item 6): one scope-tagged type; the prefix-node count lives in the row-B5 receipt

The underdetermination: the `StructuralCardinalityReceiptV1` field
inventory, the count-only realization, where the total prefix-node
count lives, and whether one scope-tagged type serves all five
positions.

Decision (registered): there is exactly one
`StructuralCardinalityReceiptV1` type, scope-tagged internally, serving
all five scope-distinct positions; the A4-O §5.3 "total prefix-node
count" lives in the `visited_nodes` field of the receipt at the
reserved row-B5 position `RawEnumerationV1.cardinality`, with no new
outer position:

```text
CardinalityScopeV1 ::=
  RawEnumeration=0
| SupportShape=1
| SupportOwnerResolution=2
| EmptyEnumeration=3
| CoreFieldCardinality=4

StructuralCardinalityReceiptV1 =
  (scope:CardinalityScopeV1,
   visited_nodes:u64,
   completed_leaves:u64,
   admitted_bound:Option<u64>)
```

The scope-position law binds each frozen position to one scope tag:

```text
| Scope tag                | Census rows           | Frozen position                                            |
| ------------------------ | --------------------- | ---------------------------------------------------------- |
| RawEnumeration=0         | B5                    | RawEnumerationV1.cardinality                               |
| SupportShape=1           | C21; retained copy C8 | SupportShapeTraversalV1.cardinality;                       |
|                          |                       |   SupportCoverageV1.shape_cardinality                      |
| SupportOwnerResolution=2 | C22; retained copy C9 | SupportOwnerResolutionV1.resolution_cardinality;           |
|                          |                       |   SupportCoverageV1.resolution_cardinality                 |
| EmptyEnumeration=3       | C12 (one per field)   | OrdinaryEmptyReplacementTranscriptV1                       |
|                          |                       |   .empty_enumeration_cardinality                           |
| CoreFieldCardinality=4   | C17                   | EnvelopeAssemblyTranscriptV1.checked_field_count           |
```

The plan-§13.4a item-6 phrase "all five positions" denotes exactly
these five scope-distinct positions; the census's seven R2b/R2c rows
include the two frozen byte-identical retained copies
(`O.support.coverage.shape_cardinality = S.cardinality` and
`O.support.coverage.resolution_cardinality = R.resolution_cardinality`,
R2c §7.3 lines 2685–2688), checked as byte equalities, never
independently reconstructed. Scope names are minted from frozen
spellings (`RawEnumerationV1`, `SupportShapeTraversalV1`,
`SupportOwnerResolutionV1`, `empty_enumeration_cardinality` /
`EmptyEnumerationCardinalityEvents`, `CoreFieldCardinalityEvents`).

Per-scope field equations, all checked:

```text
scope = RawEnumeration:
  visited_nodes    = the count-only total of the initial-empty-code
                     event plus every attempted encoded
                     scalar/tag/vector-length/source decision,
                     including every rejected duplicate source
                     (RC1 §2.6 rule 1) — the A4-O §5.3
                     "total prefix-node count"
  completed_leaves = the count-only total of completed raw codes
                   = enumeration.codes.len       (checked recomputation)
  admitted_bound   = Some(bound): the checked u64 result of the A4-O
                     §5.2 structural upper-bound interpreter for this
                     pair and constructor, with
                     completed_leaves <= bound

scope = SupportShape:
  visited_nodes    = every attempted structural decision of the
                     complete support-shape traversal (rule 19,
                     StructuralSupportTraversalIsComplete) — the exact
                     Visit total of the shape event stream (R2c §7.3)
  completed_leaves = shape_slots.len              (checked) — the
                     exact Slot total (R2c §7.3)
  admitted_bound   = None

scope = SupportOwnerResolution:
  visited_nodes    = every attempted owner-resolution decision
                     — exactly d attempted (R2c §7.3), d =
                     S.shape_slots.len
  completed_leaves = direct_dependencies.len
                   = direct_owner_map.len         (checked) — exactly
                     d successful, in slot order (R2c §7.3)
  admitted_bound   = None

scope = EmptyEnumeration:
  visited_nodes    = the one initial-empty-grammar event; no rule
                     decision exists to attempt
                     (attempted_rule_ordinals is empty, R2c §8)
  completed_leaves = rule_count = 0   (byte-equal to the frozen
                                       rule_count:u64=0)
  admitted_bound   = None

scope = CoreFieldCardinality:
  visited_nodes    = the frozen R2c §9.2 "exactly 16 visited"
  completed_leaves = the frozen "16 successfully mapped fields"
                     (A1 fields 1--16)
  admitted_bound   = None
```

Argument from frozen text:

1. One shared type: every one of the seven positions spells the
   identical name `R3::StructuralCardinalityReceiptV1`, and under the
   frozen R2a acyclic schema DAG plus the R2b §6.1 "one exact earlier
   local type" rule that name denotes exactly one node — the RD-R3-A-1
   argument verbatim. Scope discrimination is internal because no
   frozen outer position discriminates.
2. The field inventory realizes the frozen counting demands and
   nothing else: `visited_nodes`/`completed_leaves` is exactly the
   attempted/completed split of RC1 §2.6 rule 1 (events for decisions
   attempted versus completed raw codes), of the A4-O §2.2 counter
   pair `RawSearchNodesPerPair` ("every visited dependent raw-code
   prefix node, including rejected prefixes") versus
   `CompleteRawCodesPerPair` ("complete `RawCode_c` leaves evaluated
   exactly once"), and of all three frozen positions whose receipt
   contents are displayed: the R2c §9.2 field-count wording "exactly
   16 visited and 16 successfully mapped fields", and the R2c §7.3
   pair (lines 2700–2702) — "the exact `Visit` and `Slot` totals of
   the shape event stream"; "exactly `d` attempted and `d` successful
   direct resolutions in slot order" — each confirms the two-count
   shape. Widths are forced by
   `cardinality_and_counter_bits : u16 = 64`; the scope tag is `u8` in
   zero-based displayed order.
3. Prefix-node residence: the R2d §14.2.4.2 owner table assigns "total
   prefix-node count" to the "R3 count-only traversal evidence family
   (R2a §12)" and assigns "codec/cardinality derivation" to
   `RawEnumerationV1` "and its R3-qualified
   `R3::StructuralCardinalityReceiptV1`"; LF.7 requires the count
   "realized inside a reserved position without a new outer position".
   The only reserved position of this family on the raw-enumeration
   path is row B5; therefore the total prefix-node count is
   `visited_nodes` there, and the codec/cardinality derivation is
   retained as `admitted_bound` — the checked `u64` sums/products
   result of the A4-O §5.2 preflight interpreter, whose failure case
   F-A already closed as `ProfileBoundaryV1` (§2.1.3 clause 2). The
   presence law (`Some` exactly at scope `RawEnumeration`) is the
   §2.4.3 presence-law device.
4. The total raw-leaf count is never stored twice: `completed_leaves`
   at row B5 is a checked recomputation of `enumeration.codes.len`
   (the frozen R2d owner table: "`n` as the element count of
   `enumeration.codes` and of `leaf_outcome_tags`, never stored
   twice"), so no claimed count can disagree with the vector it
   describes; counts remain symbolic (§1.1).

#### 3.4.2 The count-only traversal family and the RC1 §2.6 rules, restated as construction law

The R2a §12 "structural cardinality and count-only traversal evidence"
family is realized entirely by the shared receipt above at its census
rows; there is no separate traversal-evidence record, no outer position
for one, and no retained per-event log — an event log would be a
construction transcript, which no R3 definition may consume (LF.13)
and no frozen position retains. The family's whole evidence is the
counts plus these checked laws, binding per LF.7 on every scope:

1. One event per attempted decision (RC1 §2.6 rule 1): the count-only
   traversal runs "over the same structural generator" with one event
   for the initial empty code, one for every encoded
   scalar/tag/vector-length/source decision attempted — including a
   rejected duplicate source — and one for every completed raw code.
2. Count-only equals emitting (RC1 §2.6 rule 2): the count-only totals
   must equal the emitting traversal's totals exactly; a disagreement
   is the abort `EnumerationCardinalityMismatch(expected, observed)`
   with `expected` the count-only total and `observed` the emitting
   total (§2.1.2; §2.2 row `RawGeneration[0]`).
3. No semantic decoder may run during the count-only pass (RC1 §2.6
   rule 1; LF.7): `Decode_c` is called "exactly once only after the
   whole structural raw code exists" (A4-O §5.2), and no kernel
   judgment or semantic decoder success prunes the raw universe.
4. Root-`0xe8` density is never read from the receipt: "density is
   checked against `enumeration.codes.len`, never against the
   cardinality receipt" (R2d §14.2.6, line 16390); the certificate's
   density evidence is the tag vector's own element count and the
   checked equations of R2d §14.2.4.3, and no count `n`, `m`, or `c`
   is stored as a separate certificate field. The receipt is evidence,
   never a decision input (LF.9, rule 23).
5. Charging correspondence: the row-B5 `visited_nodes` units are
   exactly the units charged at `RawSearchNodesPerPair` and its
   `completed_leaves` units exactly those charged at
   `CompleteRawCodesPerPair`; the receipt itself is retained material
   under the F-A clause-6 charging rules. The charge events, the
   fourteen-counter object mapping, and every account equation close
   in F-C (LF.8) without altering this correspondence.

#### 3.4.3 Aborts of this subsection

A per-position or per-lane receipt type; a scope tag disagreeing with
the scope-position law; a count-only/emitting disagreement; a semantic
decoder invocation during the count-only pass; a density, coverage,
class, count, or disposition check reading receipt bytes; a
`Some(admitted_bound)` outside scope `RawEnumeration` or a `None` at
it; `completed_leaves` exceeding `admitted_bound`; a row-C8/C9 copy
not byte-identical to its C21/C22 original; an `EmptyEnumeration`
receipt with a nonzero `completed_leaves` or a nonempty attempted-rule
vector; a `CoreFieldCardinality` receipt differing from the frozen
16/16 values; a retained per-event log or traversal transcript;
ordinal or count overflow; a stored duplicate of a vector's own
element count at a certificate position; trailing bytes, truncation,
or re-encode disagreement on the closed positions.

### 3.5 F-B closing statement

F-B fixes exactly: `KernelCallRoleV1` as the closed eleven-position
role enum realizing the total A4-O false/abort role map — four derived
proposal-check roles (stage `Fields` excluded: its only lawful `False`
variant is the non-kernel `EmptyReplacementCertificateMismatch` and
the frozen field-disposition transcript makes no kernel call, R2c §8),
seven abort-only roles verbatim RC1 §2.5, the
total role map with `False` cells reachable exclusively at
proposal-check roles, the role-stage coherence equations, and the
registered inhabitation constraints — under registered decision RD-R3-B-1
(LF.6, §3.1); `KernelSuccessReceiptV1` as call identity only —
role/stage/checked contiguous `u32` call ordinal, no per-call meter
delta and no reserved meter field, the outer/inner role redundancy
closed by checked binding equations, and the R2c §9.2
exhaustiveness/reconciliation rule closed as an order-preserving
one-to-one correspondence with the frozen per-scope kernel-call event
streams — under registered decision RD-R3-B-2, with no error string,
optional receipt, or implementation counter added to any frozen field
(LF.6, §3.2); the kernel-error realization as
`KernelErrorTagV1`/`KernelErrorEvidenceV1` inside exactly the two F-A
optional kernel-error fields, with the total-map presence and
byte-equality equations and no new outer position, under registered
decision RD-R3-B-3 (§3.3), completing the §2.3.2/§2.4.1 payloads; and
`StructuralCardinalityReceiptV1` as one shared scope-tagged type over
the five scope-distinct positions with the visited/completed/bound
inventory, the total prefix-node count resident in the row-B5
`visited_nodes`, the count-only traversal family realized with the
RC1 §2.6 rules as construction law, and the root-`0xe8`
density-never-from-receipt rule restated, under registered decision
RD-R3-B-4 (LF.7, §3.4).

Global-line checks for F-B's material:

- LF.9 — no F-B schema makes any role, receipt, or count a decision
  input: the quotient projections erase `call_role` and every receipt
  (R2d line 1123); root-`0xe8` density is checked against
  `enumeration.codes.len`, never against the cardinality receipt (R2d
  §14.2.6); the 3.1.3/3.2.1/3.3/3.4.1 equations are consistency checks
  on retained evidence, not decision edges; holding pair inputs fixed
  while varying only role/receipt/count material leaves every quotient
  key, tag vector, class row, witness, and disposition byte-unchanged;
  rule 23 extends verbatim.
- LF.10 — containment unchanged: F-B mints zero new roots and zero new
  R3-qualified positions; every F-B type is defined by value inside
  already-reserved census positions; the filled material nests only
  under the twelve R2a §9 corrigendum roots; `0xfd`, `0xe9`, and
  `0xff` remain receipt-free; the three inherited `0xfe` channels and
  the per-variant root-`0xe3` containment are unchanged.
- LF.11 — no closed grammar byte, tag, field order, ownership edge,
  scalar width, opcode, predecessor vector, or registered decision
  changed; every F-B type (`KernelCallRoleV1`,
  `KernelSuccessReceiptV1`, `KernelErrorTagV1`,
  `KernelErrorEvidenceV1`, `CardinalityScopeV1`,
  `StructuralCardinalityReceiptV1`) is defined topologically earlier
  under the R2 scalar codec than every frozen position consuming it,
  with leaf fields drawn only from frozen earlier types
  (`PipelineStageV1`, `KernelResourceKindV1`, wire scalars). Of the
  two F-A-named open nodes, `KernelErrorEvidenceV1` is now closed and
  `ResourceDeltaVectorV1` remains open with the three account
  families; consequently no root holding a qualified position — and no
  `trace_commitment` value — has complete canonical bytes at F-B
  close.
- LF.12 — stop boundary held: no numerical `TypeIdV1`, no registry or
  source join, no fixture or mutation bytes, no metered batch API and
  no per-call usage value of any kind (the meter remains unavailable;
  the success receipt is call identity only), no executable authority,
  no verified profile token, no theorem; the record-§10.19.6
  truth-audit gate persists unweakened.
- LF.13 — the transitive forbidden-input scan over §3 ran at F-B close
  and passes: every `call_ordinal` and count is a checked position or
  checked recomputation bound by registered equations (the RT-B
  pattern), never a copied ordinal; no F-B definition consumes a
  representative, construction transcript or per-event log,
  replay-as-input, semantic SR2 result, `nu`, `GCap`/`gamma` or other
  semantic-novelty datum,
  QG2 stored route vector, source or syntax copy outside the frozen
  retention positions, or numerical `TypeIdV1`; nor any remaining
  family of the frozen R2d §10.21.6 union — a support-to-lineage,
  footprint-to-owner, or lineage-to-stream back-edge, an evidence
  target, or an R3 account or receipt consumed as input — zero hits
  in each; zero new admission values beyond QG2's two.

F-B does NOT close: F-C — the internal layout of
`EnvelopeResourceAccountV1`, `PairResourceAccountV1`, and
`ActionResourceAccountV1` with `ResourceDeltaVectorV1` (§13.4a item
7), the meter unit/granularity and charge schema per event kind with
the per-envelope reset (item 8), the exact `Aggregate_R3` conservation
equations and child orders (item 9), the closed account scope-tag enum
and the `EmptyReplacementField(i)` encoding (item 10 — distinct from
and not constrained by `CardinalityScopeV1`, which tags receipts, not
accounts), the material/depth recurrence realization (item 13), the
static failure/resource definitions nested under `0xe0`/`0xf8`/`0xf9`
and their field-13 reconciliation (item 14), and the fourteen-counter
mapping with the field-17 metering equality (LF.8); and, per LF.12,
numerical `TypeIdV1` assignment and the registry/source join (R2e),
fixture and mutation regeneration (R4), any executable JG2b2b2b
authority, and any theorem. This sub-cut mints no verified profile,
executable interface, envelope census, pair disposition, occurrence
classification, action image instance, theorem, generic law, cubical
bridge, `GCap`, `gamma`, or selective authority. The next lawful move
is F-C, the accounts, meter, and cut-global closure lane, under this
same census.

## 4. F-C: accounts, meter, and cut-global closure

This sub-cut closes ledger line LF.8 and the cut-global lines LF.9–LF.13
under the §1 census and the §13.4a assignments (items 7, 8, 9, 10, 13, 14
as RD-R3-C-1, RD-R3-C-2, RD-R3-C-3, RD-R3-C-4, RD-R3-C-5, RD-R3-C-6). It
consumes §§1–3 unchanged and stays consistent with RD-R3-A-1 through
RD-R3-B-4. The census rows it fills are exactly the rows F-A and F-B
deferred here: the ten `EnvelopeResourceAccountV1` rows C6, C7, C10, C11,
C13, C14, C15, C16, C19, C20; the pair-account row D5; and the
action-account row D8. With these rows filled, every row of the §1 census
is filled by a named section of this record and the §1.1 totality rule is
discharged with zero residue. The F-A-named open node
`ResourceDeltaVectorV1` (§2.3.2, §2.4.1) closes in this section together
with the account families, exactly as F-A and F-B reserved. The inherited
transitive channels of §1.5 reuse these defining occurrences and add no
position.

### 4.1 The three account families and the meter (LF.8)

#### 4.1.1 REGISTERED DECISION RD-R3-C-1 (§13.4a item 7): full fourteen-row before/after windows; limits by `0xfa` policy reference; one fixed opaque-span length

The underdetermination, catalogued at plan-§13.4a registration and
assigned to F-C: account internal layout — full fourteen-counter vector
versus scope-relevant subset; deltas versus totals; embedded limits
versus `0xfa` policy references; the fixed encoded length of a finalized
opaque account span.

Decision (registered): the three account families close with one shared
internal shape over the full fourteen-counter window vector:

```text
AccountScopeV1 ::=
  SubjectConstruction=0 | Normalization=1 | SupportCoverage=2 |
  SupportTotal=3 | SupportOuter=4 | EmptyReplacementField=5 |
  FieldDispositionTotal=6 | PublicLeavesConstruction=7 |
  ObligationConstruction=8 | EnvelopeAssembly=9 | ExactEvidenceTotal=10 |
  PairTotal=11 | ActionTotal=12

CounterWindowV1 = (before:u64, after:u64)

ResourceDeltaVectorV1 =
  (source_census_entries:CounterWindowV1,
   syntactic_depth:CounterWindowV1,
   raw_search_nodes_per_pair:CounterWindowV1,
   complete_raw_codes_per_pair:CounterWindowV1,
   material_nodes_per_envelope:CounterWindowV1,
   material_nodes_per_pair:CounterWindowV1,
   kernel_operations_per_envelope:CounterWindowV1,
   normalization_fuel_per_envelope:CounterWindowV1,
   kernel_operations_per_pair:CounterWindowV1,
   normalization_fuel_per_pair:CounterWindowV1,
   envelope_core_bytes:CounterWindowV1,
   pair_transcript_bytes:CounterWindowV1,
   profile_manifest_bytes:CounterWindowV1,
   normalized_action_steps:CounterWindowV1)

EnvelopeResourceAccountV1 =
  (scope:AccountScopeV1,
   scope_parameter:u32,
   deltas:ResourceDeltaVectorV1)

PairResourceAccountV1 =
  (scope:AccountScopeV1,          -- PairTotal only (4.2.2)
   scope_parameter:u32,           -- 0, checked
   deltas:ResourceDeltaVectorV1)

ActionResourceAccountV1 =
  (scope:AccountScopeV1,          -- ActionTotal only (4.2.2)
   scope_parameter:u32,           -- 0, checked
   deltas:ResourceDeltaVectorV1)
```

Clause by clause, with the argument from frozen text:

1. Full fourteen-row vector, never a scope-relevant subset. The
   fourteen-row inventory is the frozen A4-O §2.2 table ("the following
   fourteen `(ResourceCounterV1 tag, u64 ceiling)` entries in this exact
   order"), whose field names above are the exact fourteen
   `ResourceCounterV1` variants of §2.1.3 in that frozen order; RC1 §2.6
   demands "one exact mapping from every retained object and byte
   encoding to the fourteen counters", and the audited RC1 defect was
   precisely a coarser projection of the fourteen ("It also used six
   generic budget categories for fourteen resource counters") — a
   per-scope subset would re-introduce that defect class as a per-scope
   re-partition. A row outside its scope's charging set is lawful only
   as a flat window (`after = before`), checked; scope relevance is a
   set of equations (4.1.2, 4.2.1), never a layout variation.
2. Before/after windows, not bare deltas and not bare totals. "Frozen
   counters, limits, and before/after deltas" is the verbatim A4-O §5.3
   content law, restated by LF.8; the window form realizes both derived
   readings at once (the delta is the checked `u64` difference
   `after - before`; the total is `after` at ledger close) and is the
   only form under which the frozen A4-O §7 monotone-continuation law
   ("per-envelope counters may reset only at the start of a new raw
   leaf while the pair totals continue monotonically") is a checkable
   byte property of retained values (the chaining equations of 4.1.2
   clause 5).
3. Limits close as `0xfa` policy references, never embedded copies. The
   fourteen ceilings are frozen bytes of the one governing
   `OrdinaryEnvelopeResourcePolicyV1` (root `0xfa`), reached through the
   frozen profile bindings of every object enclosing an account; the
   resource-policy identity is itself a quotient field ("The
   resource-policy identity and parent/profile bindings remain quotient
   fields", A4-O §6.2), so every account is evaluated under exactly one
   frozen policy with no second copy. Embedding fourteen limit words in
   every account would store values derivable from a single frozen
   table — forbidden by the derivable-stored-value inference this
   record registered at §2.1.3 clause 4 from the audited RC1 §2.5
   category defect ("It also used six generic budget categories for
   fourteen resource counters"), in the pattern of the frozen R2d
   stored-count prohibition ("No count `n`, `m`, or `c` is stored as a
   separate certificate field", R2d §14.2.4.3) — and would re-bind an
   identity already bound once (the
   F-B 3.2.1 clause-5 manifest device). The A4-O §5.3 whitelist is
   discharged as: counters by the fixed fourteen-row order, limits by
   the policy reference enforced at every charge (4.1.2 clause 2),
   deltas by the stored windows; the account's only stored words are
   the windows. `BudgetExhausted` keeps its embedded byte-equal
   `limit:u64` unchanged (§2.4.2): a failure receipt is self-contained
   evidence of the exceeded bound; a finalized account never repeats
   the policy.
4. One fixed encoded length per finalized span. R2c §1.1 requires every
   event compiler to treat "a finalized account as one opaque typed
   byte span of its R3-closed encoded length" without descending into
   its value bytes, and the frozen field-17 length function
   `CanonicalEncodedLengthFromTypedShape` "observes only type tags,
   vector/byte-string lengths, the leading `0xfc,u16=1` root/version
   header, and the R3-closed encoded lengths of account fields; it does
   not inspect an account value" (R2c §9.2). The account encoding is
   therefore value-independent: the one `u8` scope tag, the one `u32`
   scope parameter, then the fourteen rows of two `u64` scalars each,
   concatenated in displayed order under the frozen R2a wire, with no
   optional field, no wire vector, no count prefix, and no
   scope-dependent payload anywhere inside an account. The fixed
   encoded length of a finalized span is the schema-derived constant of
   exactly that shape — identical for all three families and for every
   scope value — and is never a separately stored numeral.
   `ResourceDeltaVectorV1` is a fixed fourteen-field record, not a wire
   vector: no `u64` element count is stored, and no per-row counter tag
   is stored (the row position in the frozen order is the tag; a
   stored tag would duplicate a derivable value).
5. Three distinct nodes, one shared shape, one shared window node. The
   three family names are distinct frozen spellings at distinct census
   positions, so by the RD-R3-A-1 name/node rule each closes as its own
   schema-DAG node; their internal shape is deliberately identical, and
   the receipts' `resource_deltas` fields (§2.3.2, §2.4.1) consume the
   same single `ResourceDeltaVectorV1` node without a scope word — the
   receipt's `stage`/`locus` already discriminate, and a receipt is not
   an account and mints no scope authority. A receipt's window is the
   fourteen-row window of its charging span: the per-envelope rows from
   the raw-leaf reset, the pair rows the monotone pair-ledger window
   over the same span; for a pair-scope abort before any leaf the
   per-envelope rows are flat.
6. Accounts are themselves retained material under the F-A clause-6
   charging rules: their nodes charge the material counter of their
   scope and their canonical bytes charge the byte counter of their
   enclosing rooted object; no uncharged retention exists, and no event
   of any compiler depends on the counter values being charged (R2c
   §1.1: the compilers "never descend into the counters"; "no account
   under construction is ever an event input").

#### 4.1.2 REGISTERED DECISION RD-R3-C-2 (§13.4a item 8): the meter — frozen per-counter units, one checked charge per event, the per-envelope reset

The underdetermination: meter unit and granularity and the charge schema
per event kind, including the per-envelope counter reset at raw-leaf
start.

Decision (registered), clause by clause from frozen text:

1. Unit: there is no common meter unit and no invented sub-unit. Each
   counter's unit is its own frozen A4-O §2.2 unit column — the existing
   `pen-kernel` logical operation unit, the existing normalizer fuel
   unit, retained material slots/nodes, visited prefix nodes, complete
   raw leaves, canonical bytes (root tags and length prefixes count),
   census entries, syntactic depth, and flattened action steps. "Pure
   finite control work need not acquire an invented 'CPU step' counter"
   (RC1 §2.6); estimating usage or treating the number of calls as the
   operation count "cannot satisfy this V1 profile" (A4-O §2.2).
2. Granularity: one checked `u64` charge per event, charged exactly
   once at its frozen counter in the deterministic event order of the
   frozen R2c compilers and the A4-O §7 accounting order, with checked
   arithmetic, no silent wrap, and the frozen ceiling of the governing
   `0xfa` policy row enforced at every charge (the first charge whose
   would-be total strictly exceeds its ceiling is the §2.4.2
   `BudgetExhausted` abort). Accounting is deterministic: an
   implementation may optimize only when it produces the "identical
   outcome, locus, counters, and transcript" (A4-O §5.3).
3. Charge schema per event kind. The event kinds are exactly the four
   the frozen R2c §1.1 contract names — "the exact receipt-validation,
   structural, canonical-encoding, and operation events explicitly
   named for its scope" — plus the pair enumeration and action-step
   events of the A4-O §7 charge list:

   ```text
   | Event kind             | Charged counters (frozen A4-O §2.2 rows)      |
   | ---------------------- | --------------------------------------------- |
   | structural             | material_nodes_per_envelope /                 |
   |                        |   material_nodes_per_pair, by the             |
   |                        |   material-size recurrence (4.3.1);           |
   |                        |   raw_search_nodes_per_pair for every         |
   |                        |   visited prefix node incl. rejected;         |
   |                        |   complete_raw_codes_per_pair for every       |
   |                        |   complete leaf; source_census_entries at     |
   |                        |   the one census binding before one pair      |
   |                        |   search; syntactic_depth as the high-water   |
   |                        |   check of the depth recurrence (4.3.1)       |
   | operation              | kernel_operations_per_envelope and            |
   |                        |   kernel_operations_per_pair (kernel logical  |
   |                        |   operation unit); normalization_fuel_per_    |
   |                        |   envelope and normalization_fuel_per_pair    |
   |                        |   (normalizer fuel unit)                      |
   | canonical-encoding     | envelope_core_bytes / pair_transcript_bytes / |
   |                        |   profile_manifest_bytes; root tags and       |
   |                        |   length prefixes count                       |
   | receipt-validation     | the material counters of the scope: path      |
   |                        |   decoding, cardinality, and byte-equality    |
   |                        |   over already charged records; never         |
   |                        |   re-executes or recharges the validated call |
   | action step            | normalized_action_steps (primitive            |
   |                        |   substitutions or squares in one flattened   |
   |                        |   action)                                     |
   ```

   A unit with both a per-envelope and a per-pair frozen counter is one
   event reflected once in each governing ledger's row for its scope —
   the pair row is the rule-36 aggregate of the envelope charges plus
   pair-direct charges — and is never charged twice within one ledger;
   "no kernel or normalizer call may receive a fresh invisible pair
   budget" (A4-O §7).
4. The one maximum-valued row: `SyntacticDepth` is the only counter
   whose frozen unit is a maximum ("maximum kernel-term,
   carrier-record, support-path, or theorem-meta-syntax depth"); its
   windows are high-water marks (`after = max(before, observed)`),
   aggregated by checked maximum (4.2.1); every other row is additive.
5. The per-envelope reset, consumed verbatim from A4-O §7:
   "Per-envelope counters may reset only at the start of a new raw leaf
   while the pair totals continue monotonically." Realization: the four
   per-envelope rows (`material_nodes_per_envelope`,
   `kernel_operations_per_envelope`, `normalization_fuel_per_envelope`,
   `envelope_core_bytes`) open every raw leaf's ledger with
   `before = 0`; every pair-scope row opens with `before` equal to the
   pair ledger's current value; across the per-leaf windows in
   canonical raw ordinal order the pair-scope rows chain contiguously
   (`window[j+1].before = window[j].after`), and the retained per-leaf
   windows plus the pair-direct spans tile the pair ledger with no gap
   and no overlap. In the action lane no raw leaf exists and no reset
   occurs: the per-envelope rows of the one action account window the
   single bounded construction from `before = 0` at action-ledger
   start (A4-O §7 fixes the same operational order "for a pair or
   action").
6. Meter availability boundary: this is a compiler-plan charge schema
   only. The current kernel "does not yet return a usage receipt"
   (A4-O §2.2); executing the operation-event rows requires the
   JG2b2b2b non-authoritative metered batch API, no verified profile
   token may exist until the meter is available, and nothing in this
   section is an executable meter (LF.12).
7. Publication: an account is finalized and published "only after the
   complete ledger and replay close" (A4-O §7 step 6). Exhaustion,
   overflow, or allocation failure finalizes no account: the only
   artifact is the abort receipt with its `resource_deltas` window, and
   it is never evidence (lessons 1 and 5; A4-O §7 exhaustion
   neutrality).

#### 4.1.3 Aborts of this subsection

A scope-relevant-subset, reordered, or variable-length account layout; a
wire-vector count prefix or stored per-row counter tag; an embedded
limit word in any account; a stored bare delta or total without its
window endpoints; unchecked arithmetic or silent wrap; a wall clock,
allocation address, scheduler state, platform text, or map order; a
non-flat window at a scope-irrelevant row; a per-family internal-shape
divergence; an uncharged retained account node or byte; an event
depending on a counter value under construction; an account finalized
from an incomplete ledger; an estimated or invented operation or fuel
numeral (the meter is unavailable); a fresh invisible allowance; a
charge event charged twice within one ledger or at a foreign counter; a
per-envelope row not reset at raw-leaf start; a pair-scope row reset
anywhere; a gap or overlap in the window tiling; trailing bytes,
truncation, or re-encode disagreement on the closed positions.

### 4.2 Aggregation, scope tags, and the field-17 metering equality (LF.8)

#### 4.2.1 REGISTERED DECISION RD-R3-C-3 (§13.4a item 9): the conservation equations, the rule-36 pair aggregation, and the single action aggregate

The underdetermination: the exact `Aggregate_R3` conservation equation;
whether `PairResourceAccountV1` aggregates per-envelope accounts (rule
36) with which child list and order; the aggregate structure of
`ActionResourceAccountV1` over its bounded steps.

Decision (registered): the two R2c §1.1 specification functions close as
the following checked equations (all arithmetic checked `u64`, no silent
wrap; `Charge(c,events)` is the checked sum of 4.1.2 charges at counter
`c` over the ordered events; `DepthHighWater(events)` is the checked
maximum depth observed by the structural events):

```text
a = Acct_R3(scope,events)  requires, per row:

  additive c:        a.deltas[c].after
                       = a.deltas[c].before + Charge(c,events)
  syntactic_depth:   a.deltas[depth].after
                       = max(a.deltas[depth].before,
                             DepthHighWater(events))

g = Aggregate_R3(scope,[A_1..A_k],E)  requires, per row:

  additive c:        g.deltas[c].after - g.deltas[c].before
                       = sum over j of
                           (A_j.deltas[c].after - A_j.deltas[c].before)
                         + Charge(c,E)
  syntactic_depth:   g.deltas[depth].after
                       = max(g.deltas[depth].before,
                             max over j of A_j.deltas[depth].after,
                             DepthHighWater(E))

  every listed child contributes exactly once, in the frozen child
  order; no aggregate embeds a child account value; the aggregate is
  scope-tagged and overflow-checked (R2c §1.1).
```

This is the `Acct_R3`/`Aggregate_R3` conservation obligation of R2c
§1.1 stated as byte equations, checked at construction and re-checked
at every decode. The frozen R2c child lists and event streams are
consumed unchanged and may not be merged, duplicated, omitted, or
caller-sized (R2c §7.3): `SupportTotal` aggregates exactly the
coverage account and the `SupportOuter` account
(`Acct_R3(SupportOuter, SupportOuterEvents(O.support))`) with an empty
aggregate-only stream; `FieldDispositionTotal` aggregates the
per-field `EmptyReplacementField(i)` children in field order with
`DispositionTranscriptEvents(T)`; `ExactEvidenceTotal` aggregates the
seven children exactly once in displayed exact-evidence component
order with `ExactEvidenceAggregateEvents(X)` (the support-coverage
child is already inside the support total and the per-field children
inside the disposition total, so neither appears again — R2c §9.2).
The `SupportOuter` child account is constructed and checked but
retained at no frozen position; at decode its window is
deterministically recomputed from `O.support` under the
identical-outcome rule, so the `SupportTotal` conservation equation
stays decidable without a new position.

`PairTotal` (census row D5): `PairResourceAccountV1` DOES aggregate
the per-envelope accounts, per rule 36
(`PairAndEnvelopeResourcesAreAggregate`) and A4-O §7 step 4 ("start
one pair/action ledger and one nested per-envelope ledger").
Registered child list and order: for each raw ordinal `j` in canonical
raw ordinal order (the frozen arena order, R2a §10), leaf `j`
contributes exactly one finalized per-leaf window —

- for a `CandidateMatch` leaf: the envelope's one exact-evidence
  aggregate account `X_j.aggregate_resource_account`
  (`ExactEvidenceTotal`), which already contains its seven children
  exactly once; and
- for a `CheckedFalse` leaf: the leaf receipt's `resource_deltas`
  window (§2.3.2) — the leaf built no transcript accounts, and the
  uniform window shape of RD-R3-C-1 clause 5 is what makes this
  contribution well-typed in the same conservation equation.

The aggregate-only stream is the deterministic pair-direct event
stream outside every per-leaf window, compiled in the frozen A4-O §7
order: the census-cardinality binding (the `source_census_entries`
window realizes the frozen "before one pair search" scope, with
`after - before = |U_b| + |Pub_<b(H)|` checked against the frozen
4,096 ceiling), structural raw enumeration (prefix nodes and completed
leaves), cardinality-receipt validation, quotient projection and
comparison, class partition/sort and first-difference witnesses (the
A4-O §5.4 `FirstDifferenceV1` items, realized as the frozen R2d
`ClassKeyFirstDifferenceV1`),
the coverage tag vector with its checked density equations, canonical
`0xe7`/`0xe8` encoding, and pair-transcript reconstruction. The pair
account's rows open at the pair-ledger start values of A4-O §7 step 4.
This realizes the A4-O §5.3 retention item "the aggregate
deterministic logical resource account" at its R2d-assigned owner, the
R3-qualified `logical_account` position under `0xe8`.

`ActionTotal` (census row D8): `ActionResourceAccountV1` closes as ONE
aggregate account over its bounded steps with an empty child list and
one direct stream:
`Aggregate_R3(ActionTotal, [], ActionDirectEvents(trace))`. Argument:
A4-O §6.3 fixes "one aggregate account with at most 4,096 flattened
steps and all other depth/material/byte ceilings above"; the frozen
root-`0xfe` trace field list retains no child-account position (R2d
§14.3.2.1: `logical_account` is the single newly declared R3-qualified
position under the root; the three inherited channels retain the base
candidate's and checked steps' already finalized evidence — including,
transitively, the base envelope's own R2c accounts — never a per-step
child account of this aggregate), so no child list exists to aggregate,
and minting per-step
retained accounts would mint new positions (LF.10 forbids it). The
nested per-envelope enforcement ledger of A4-O §7 step 4 enforces the
envelope-scale ceilings at every charge over the single action-lane
window (no per-step reset exists; 4.1.2 clause 5) but finalizes no
retained child.
`ActionDirectEvents` is a name minted here for the deterministic
stream, in frozen step order: per-step checked-syntax and
replay-evidence validation (receipt-validation events over the
`unflattened_steps`), flattening, every endpoint rebuild invocation of
`CanonicalActionResult_c` (operation events; the sequential and direct
endpoints of R2d §14.3.2.1), canonical `0xfe` encoding, and trace
reconstruction, with every flattened step charged once at
`normalized_action_steps` against the frozen 4,096 ceiling.

#### 4.2.2 REGISTERED DECISION RD-R3-C-4 (§13.4a item 10): the closed scope-tag enum and the `EmptyReplacementField(i)` encoding

The underdetermination: the closed scope-tag enum and the encoding of
the parameterized `EmptyReplacementField(i)`.

Decision (registered): `AccountScopeV1` (displayed in 4.1.1) is the one
closed payload-free scope enum shared by all three account families,
encoded as one `u8` tag in zero-based displayed order — distinct from
and unconstrained by `CardinalityScopeV1`, which tags receipts, not
accounts (§3.4). Variants 0–10 are exactly the eleven R2c §1.1 symbolic
scope names ("closed distinct inputs that R3 must encode without
changing these outer field positions"), ordered by each scope's first
`Acct_R3`/`Aggregate_R3` display in R2c — the zero-based
displayed-order device of the frozen A4-O convention applied to the
document that displays them (`SubjectConstruction` §5.3.5,
`Normalization` §6, `SupportCoverage` then `SupportTotal` then
`SupportOuter` §7.3, `EmptyReplacementField` then
`FieldDispositionTotal` §8, `PublicLeavesConstruction` then
`ObligationConstruction` then `EnvelopeAssembly` then
`ExactEvidenceTotal` §9.2). Variants 11–12 are the two scopes minted
here for the two frozen R2d-lane positions, named `PairTotal` and
`ActionTotal` by the frozen aggregate naming device of the sibling
scopes (`SupportTotal`, `FieldDispositionTotal`, `ExactEvidenceTotal`),
in R2d position order (root `0xe8` D5 before root `0xfe` D8).

The parameterized scope encodes through the uniform `scope_parameter`
field, never a variant payload: `EmptyReplacementField(i)` is
`scope = EmptyReplacementField` with
`scope_parameter = checked_u32(i)`, where `i` is the public-field
ordinal of the certificate's field, range-checked against
`D.entries.len` and byte-equal to the child's position in the frozen
field-order child vector of `FieldDispositionTotal` (R2c §8); every
other scope value carries `scope_parameter = 0`, checked (the
fixed-filler device of the frozen tag-`0` `None` encoding). A variant
payload is unlawful because it would give scope-dependent encoded
lengths and break the single opaque-span length of RD-R3-C-1 clause 4;
the `u32` width is the frozen `persistent_ordinal_bits : u16 = 32`,
and the ordinal is a checked position, never a copied one (the RT-B
pattern, LF.13).

The scope-position law binds each frozen position to one scope tag,
and the family-scope law binds each family to its lawful scopes; both
are checked at construction and every decode:

```text
| Scope tag                  | Census row / residence                      |
| -------------------------- | ------------------------------------------- |
| SubjectConstruction=0      | C6  SubjectConstructionTranscriptV1         |
| Normalization=1            | C7  EnvelopeNormalizationEvidenceV1         |
| SupportCoverage=2          | C10 SupportCoverageV1                       |
| SupportTotal=3             | C11 OrdinarySupportV1                       |
| SupportOuter=4             | child value of the SupportTotal aggregate   |
|                            |   only; retained at no position             |
| EmptyReplacementField=5    | C13 CompleteNoReplacementCertificateV1      |
|                            |   (one per field ordinal i)                 |
| FieldDispositionTotal=6    | C14 FieldDispositionConstructionTranscriptV1|
| PublicLeavesConstruction=7 | C15 PublicLeavesConstructionTranscriptV1    |
| ObligationConstruction=8   | C16 ObligationConstructionTranscriptV1      |
| EnvelopeAssembly=9         | C19 EnvelopeAssemblyTranscriptV1            |
| ExactEvidenceTotal=10      | C20 ExactEvidenceTranscriptV1               |
| PairTotal=11               | D5  PairCoverageV1.logical_account (0xe8)   |
| ActionTotal=12             | D8  ActionTraceV1.logical_account (0xfe)    |
```

`EnvelopeResourceAccountV1` carries exactly scopes 0–10;
`PairResourceAccountV1` carries exactly `PairTotal`;
`ActionResourceAccountV1` carries exactly `ActionTotal`. Any other
pairing is an abort.

#### 4.2.3 The field-17 commitment metering equality (R2c §9.2)

The frozen reservation (R2c lines 3220–3229) closes as follows.
`ExactEvidenceCommitmentReservationEvents` charges exactly one reserved
future domain-separated commitment call from the displayed descriptor:
the frozen domain `law-v2/jg2b2b2a/ordinary-envelope-exact-evidence/v1`
and
`canonical_encoded_length = CanonicalEncodedLengthFromTypedShape(root=0xfc,value=X)`
— a value computable at reservation time exactly because RD-R3-C-1
clause 4 fixes the account-span lengths that function observes. The
reservation triple is `(domain, input length, meter delta)`, where the
meter delta is the checked canonical-encoding and operation charge of
one `Digest::of_domain_bytes` call over that length under the 4.1.2
charge schema, charged once inside `EnvelopeAssemblyEvents`. R2c §9.3
then executes that one call over the complete `X` bytes after the
seven-child aggregate is fixed, and — as the frozen text demands ("R3
requires its actual domain, input length, and meter delta to equal the
reservation before `E` can be returned") — the actual domain, actual
input length, and actual meter delta must each be byte-equal to the
reservation. Any disagreement, and any second or unreserved commitment
call, is the abort `CanonicalEncodingMismatch(ExactEvidence, site)` of
envelope assembly (the reservation is a canonical-encoding contract;
`ExactEvidence` is its frozen `CodecObjectV1` ordinal). Thus the
account is constructed before field 17 while the eventual commitment
still hashes every account byte without a value-level cycle, and the
field-17 commitment metering equals its frozen reservation —
discharging the LF.8 clause and R2c acceptance condition 17
("field-17 metering uses the acyclic domain/rooted-length
reservation").

#### 4.2.4 Aborts of this subsection

A conservation equation violated at any row; a child omitted,
duplicated, reordered, embedded, or caller-sized; a merged,
duplicated, omitted, or caller-sized event stream; a nonempty
aggregate-only stream for `SupportTotal`; a nonempty child list for
`ActionTotal`; a pair child list out of canonical raw ordinal order or
missing a leaf window; a flattened action exceeding the frozen step
ceiling; a scope tag violating the scope-position or family-scope law;
a nonzero `scope_parameter` outside `EmptyReplacementField`, or an
`EmptyReplacementField` parameter out of range or disagreeing with its
field-order child position; a reservation/actual disagreement in
domain, input length, or meter delta at field 17; a second or
unreserved commitment call; trailing bytes, truncation, or re-encode
disagreement on the closed positions.

### 4.3 Recurrences and the static failure/resource definitions (LF.8)

#### 4.3.1 REGISTERED DECISION RD-R3-C-5 (§13.4a item 13): the material/depth recurrences are account-internal charging law, not receipt types

The underdetermination: whether the material/depth recurrence family
becomes distinct receipt types or account-internal fields.

Decision (registered): the family closes as account-internal charging
law — the two static total recurrences below plus the account and
receipt-delta rows they inhabit — and NO distinct material or depth
receipt type exists anywhere. The recurrences realize RC1 §2.6 rule 3
("a recursive material-size function over scalars, sums/options,
records/tuples, vectors/sets, terms, judgments, and retained
references, with repeated occurrences charged repeatedly") clause by
clause:

```text
material_size(v), one charge unit per retained slot/node:
  wire scalar or enum/sum tag        = 1
  option/sum value                   = 1 + material_size(payload)
                                       (payload-free arm: 1)
  record/tuple                       = 1 + sum over declared fields
  vector/set                         = 1 + sum over elements
  bytes/text                         = 1 (the bytes themselves charge
                                       the byte counters of the
                                       enclosing encoding, never the
                                       material counters)
  term/declaration/context/judgment  = 1 + sum over immediate subnodes
                                       (the twelve-form codec inherited
                                       by exact identity from the bound
                                       kernel parent, A4-O §2.6)
  retained reference                 = 1 + material_size(the full typed
                                       identity retained after the
                                       checked owner-local index,
                                       R2a §10)

depth(v):
  wire scalar or enum/sum tag        = 1
  composite value                    = 1 + max over immediate children
                                       (empty composite: 1)
```

Repeated occurrences are charged repeatedly — no structure-sharing
discount exists (A4-O §2.2: "repetitions count"). `material_size`
charges `MaterialNodesPerEnvelope`/`MaterialNodesPerPair` per the
4.1.2 schema; `depth` is checked against the frozen 256 ceiling at
every constructor of the four frozen depth domains ("maximum
kernel-term, carrier-record, support-path, or theorem-meta-syntax
depth", A4-O §2.2) and surfaces in retained bytes only as the
`syntactic_depth` high-water windows. Together with the 4.1.2 table
this closes the RC1 §2.6 rule-4 obligation: one exact mapping from
every retained object and byte encoding to the fourteen counters.

Argument from frozen text:

1. A distinct receipt type is unmintable: the §1 census is total
   (LF.1), reserves no position for a material or depth receipt
   anywhere in the frozen records, and a fill at any non-census
   position is an abort of the cut (§1.1); a new outer position is
   forbidden outright (LF.10). The RD-R3-B-4 precedent does not
   transfer: the cardinality family HAS reserved positions (rows B5,
   C8, C9, C12, C17, C21, C22); this family has none.
2. R2a §12 family 6 couples "material/depth recurrence" with
   "allocation/budget receipts"; the receipts of that family are the
   F-A `AllocationFailure` and `BudgetExhausted` payloads (§2.1),
   which already retain the exact counter tag and the checked
   `used`/`limit` words. The recurrence is the law that computes those
   words and the account windows — not a second evidence object.
3. RC1 §2.6 demands "a recursive material-size function" and "one
   exact mapping ... to the fourteen counters" — a function and a
   mapping, not a stored artifact. Storing per-value recurrence
   results would retain values derivable from the value bytes
   themselves — forbidden by the derivable-stored-value inference
   registered at §2.1.3 clause 4 from the audited RC1 §2.5 category
   defect, in the pattern of the frozen R2d §14.2.4.3 stored-count
   prohibition — and an
   event log of the recursion would be a construction transcript,
   which no R3 definition may retain or consume (LF.13, §3.4.2).

#### 4.3.2 REGISTERED DECISION RD-R3-C-6 (§13.4a item 14): the static definitions under `0xe0`/`0xf8`/`0xf9` and the field-13 reconciliation

The underdetermination: the exact form of the static failure/resource
definitions nested by value under `0xe0`/`0xf8`/`0xf9` and their
reconciliation with the frozen A4-O field-13 tag tables without
changing any A4-O byte.

Decision (registered):

1. Form: every R3 type minted by this record — the §2.1 reason sums
   and subordinate enums, the §2.3/§2.4 receipt records, the §3 role,
   receipt, error-evidence, and cardinality types, and the §4 account
   types (`AccountScopeV1`, `CounterWindowV1`,
   `ResourceDeltaVectorV1`, the three account families) — closes as
   one static schema-DAG node definition per type in the frozen R2a
   schema metalanguage, appended by value to the complete `0xe0`
   dynamic schema registry, which the `0xf8` profile definition owns
   by value ("The completed profile definition at `0xf8` must own the
   source-set contract and the complete `0xe0` registry by value",
   R2a §11) and whose identity the `0xf9` manifest binds. This is
   exactly the nesting the R2a §9 corrigendum records ("the static
   failure/resource definitions nested by value under `0xe0`, `0xf8`,
   and `0xf9`"); no other root nests a static R3 definition.
   Numerical `TypeIdV1` numerals for the new registry rows are the
   named R2e boundary (LF.12); both questions this item registers —
   form and reconciliation — are decided here.
2. Reconciliation without changing any A4-O byte: the frozen
   static-definition transcript (canonical byte length 1209, digest
   `blake3:c6ad7291770439b2d1a3055dca862f62680a507f28ce803761777fc823361300`,
   displayed at A4-O §2.6) already encodes field 13 as "all seven
   false-family tags with their nested variant-tag vectors, all twenty
   abort tags, the seventeen pipeline-stage tags, the eight
   profile-parent tags, and a thirteen-row kernel-error table", and
   embeds the complete `0xfa` resource policy by value. The
   reconciliation is the following checked projection-equation set, in
   the direction R3 type -> frozen bytes, checked at profile remint:
   - the tag-table projection of `FalseReasonV1` (§2.1.1) is
     byte-identical to the seven family tags with their nested
     variant-tag vectors;
   - the projection of `AbortReasonV1` (§2.1.2) is byte-identical to
     the twenty abort tags;
   - `PipelineStageV1` projects to the seventeen stage tags and
     `ProfileParentTagV1` to the eight parent tags (§2.1.3);
   - the thirteen `KernelErrorTagV1` variants (§3.3) with the two
     outcome columns of the §3.1.2 total role map — "(KernelError tag,
     candidate/action outcome tag, replay/reconstruction outcome tag),
     with `False=0` and `Abort=1`" — are byte-identical to the frozen
     thirteen-row kernel-error table: the candidate/action column is
     the proposal-check-role column and the replay/reconstruction
     column the abort-only-role column of the §3.1.2 map; and
   - `ResourceCounterV1`'s fourteen tags in order, with the fourteen
     ceilings referenced by RD-R3-C-1 clause 3, are byte-identical to
     the embedded policy rows of the same frozen transcript (root
     `0xfa`, `resource_policy_version : u16 = 1`,
     `persistent_ordinal_bits : u16 = 32`,
     `cardinality_and_counter_bits : u16 = 64`, then the fourteen
     `(ResourceCounterV1 tag, u64 ceiling)` entries).
3. No second authority: the frozen A4-O bytes remain the byte
   authority and the R3 schema-DAG nodes the typed authority, bound
   one-to-one by the checked projections — the exact RD-R3-A-2
   binding-equation device. The R3 registry rows ADD definitions; no
   A4-O byte changes, because field 13 is "the typed
   mutation-sensitive binding to the complete normative A1, A2-O, and
   A3-O definitions" and "An implementation that changes such a
   definition while retaining the V1 tag fails reminting" (A4-O §2.5)
   — a projection disagreement is therefore a reminting failure of the
   R3 side, never an edit of the frozen side.

#### 4.3.3 Aborts of this subsection

A distinct material or depth receipt type or any new outer position; a
stored per-value recurrence result or recursion event log; a
structure-sharing discount; an uncharged retained node; a depth check
omitted at a constructor of the four frozen depth domains; an invented
CPU-step or sub-unit counter; a static R3 definition nested under any
root other than `0xe0`/`0xf8`/`0xf9`; a registry row whose projection
disagrees with any frozen field-13 or embedded-policy byte; any A4-O
byte edit; a numerical `TypeIdV1` value (R2e); trailing bytes,
truncation, or re-encode disagreement on the closed positions.

### 4.4 Cut-global closure (LF.9–LF.13)

#### 4.4.1 Receipts are never decision inputs, with the independence extension (LF.9)

No schema of §§2–4 makes any receipt, account, role, count, or window
a decision input: no quotient key, tag vector, class, class order,
count, disposition, flattening, rebuild, or opcode value depends on
receipt or account bytes. The frozen guards all hold with the filled
schemas: the quotient projections erase `call_role`, `raw_wire`,
`raw_context`, and every receipt (R2d line 1123); root-`0xe8` density
is checked against `enumeration.codes.len`, never against receipt
bytes (R2d §14.2.6); the event compilers never descend into account
counters and "no account under construction is ever an event input"
(R2c §1.1, §7.3) — accounts are constructed FROM decision-bearing
values, never the reverse; and every 4.1/4.2 conservation, chaining,
scope, and reservation equation is a consistency check on retained
evidence, not a decision edge. The independence extension: the frozen
10.22.4/RT-A/RT-B/RT-C independence results extend verbatim to the
filled schemas — holding the pair/action inputs and the applicable
formal semantic environment fixed while varying only receipt/account
material leaves every quotient key, tag vector, class row, class
order, witness, count, disposition, flattened action, opcode, and
comparison value byte-unchanged, and changes a receipt-carrying value
only at the frozen reserved positions embedding the varied bytes
verbatim (and, downstream of them, the `trace_commitment` and
field-17 commitment digests of the complete rooted bytes). Receipts
and accounts remain non-quotient fields (rule 23,
`ResourceAndCheckerReceiptsAreNotQuotientKeys`).

#### 4.4.2 Containment unchanged; zero new positions (LF.10)

The completed cut mints zero new roots and zero new R3-qualified
positions. Every F-A/F-B/F-C type is defined by value inside
already-reserved census positions; the filled material nests exactly
under the twelve R2a §9 corrigendum roots (`0xe0`, `0xe3`, `0xe5`,
`0xe6`, `0xe7`, `0xe8`, `0xf8`, `0xf9`, `0xfa`, `0xfb`, `0xfc`,
`0xfe`). Root `0xfd` remains receipt-free by the frozen quotient
stripping rule; roots `0xe9` and `0xff` remain receipt-free by the
registered decisions RD-RT-B-2 and RD-RT-C-2(i). The three inherited
`0xfe` channels and the per-variant root-`0xe3` containment are
unchanged (§1.5). The `SupportOuter` child account is a construction
value retained at no position and adds none. Every census row of §1
is filled by §§2–4; none was invented; none was missed.

#### 4.4.3 No-change audit; topological earliness and byte-completeness (LF.11)

No closed grammar byte, tag, field order, ownership edge, scalar
width, opcode, predecessor vector, or registered decision (RD-RT-A-1
through RD-RT-C-4, every earlier registered decision, and the cut's
own RD-R3-A-1 through RD-R3-A-4, RD-R3-B-1 through RD-R3-B-4, and
RD-R3-C-1 through RD-R3-C-6, each immutable from its registration)
changed anywhere in this cut. Every F-C type
(`AccountScopeV1`, `CounterWindowV1`, `ResourceDeltaVectorV1`,
`EnvelopeResourceAccountV1`, `PairResourceAccountV1`,
`ActionResourceAccountV1`) is defined topologically earlier under the
R2 scalar codec than every frozen position consuming it, with leaf
fields drawn only from wire scalars and the closed enums of this
record. With `ResourceDeltaVectorV1` and the three account families
closed, both F-A-named open nodes are now closed
(`KernelErrorEvidenceV1` in F-B, `ResourceDeltaVectorV1` here) and no
reserved qualifier remains anywhere in the schema DAG: per the LF.11
line, the roots holding qualified positions — including every
`trace_commitment` value and complete canonical `0xff` bytes — now
become canonically byte-complete at the schema level: every position
of every root has a complete schema-DAG definition and the canonical
byte grammar is total. What remains uncomputable is inhabitants, at
their named boundaries: numerical `TypeIdV1` registry numerals (R2e),
fixture bytes (R4), and every executed evaluation (JG2b2b2b).

#### 4.4.4 Stop boundary (LF.12)

Held for the whole cut: no numerical `TypeIdV1` and no
registry/source join (R2e); no fixture or mutation regeneration (R4);
no metered batch API and no executable meter — the 4.1.2 charge
schema is compiler-plan only and the meter remains unavailable; no
executable authority and no verified profile token (JG2b2b2b); no
theorem — the three preservation descriptors remain uninhabited typed
syntax, and the record-§10.19.6 truth-audit gate persists unweakened.

#### 4.4.5 Transitive forbidden-input scan with recorded result (LF.13)

The scan ran at F-C close over §4 and once more, cumulatively over
§§1–4, at cut close, against the full frozen union of forbidden
families: the plan §13.4 LF.13 parenthetical and the R2d §10.21.6
list that the LR.13 restatements bind on every R2d equation the
filled schemas participate in. Families checked against every
definition and equation: representatives; copied ordinals — every
ordinal of §4 (the
`scope_parameter` field ordinal, the child positions of every child
list, the canonical-raw-ordinal child order) is a checked position
bound by registered equations, the RT-B pattern; construction
transcripts and per-event logs — no account retains an event log,
only windows, and every event stream is a deterministic compilation,
not a retained value; replay-as-input — receipt-validation events
validate already charged records and never re-execute a call;
support-to-lineage, footprint-to-owner, and lineage-to-stream
back-edges; evidence targets; R3 accounts or receipts consumed as
input (the LF.9 attestation of §4.4.1, re-run here as a scanned
family); semantic SR2 results; `nu`; `GCap`/`gamma` or any other
semantic-novelty data; QG2 stored route
vectors; source or syntax copies outside the frozen retention
positions; numerical `TypeIdV1`. Recorded result: PASS at F-C close
and PASS at cut close — zero hits in every family, including the
three back-edge families and evidence targets, and zero new
admission values beyond QG2's two.

### 4.5 Record closing statement: what A4-R3 fixes and what it does not

A4-R3 fixes exactly, at the compiler-plan boundary: the total
reservation census over both notations with its totality rule and
agreement checks (§1; LF.1); the closed reason taxonomy with every
subordinate locus/payload schema at exact widths and the total
kernel-error map unchanged (§2.1; LF.2); the lesson-11 completeness
re-audit over every input family and result partition (§2.2; LF.3);
`CheckedFalseReceiptV1` and `AbortReceiptV1` as the two shared,
locus-discriminated receipt records with checked binding equations,
the dedicated duplicate-census receipt, the exact `BudgetExhausted`
realization, the unchanged `NoDisposition` realization, and Unknown
as the absence of a disposition plus the unprivileged diagnostic
(§§2.3–2.4; LF.4–LF.5); `KernelCallRoleV1`, `KernelSuccessReceiptV1`,
and `KernelErrorEvidenceV1` with the total role map, binding
equations, and exhaustiveness/reconciliation law (§§3.1–3.3; LF.6);
`StructuralCardinalityReceiptV1` and the count-only traversal family
(§3.4; LF.7); and the resource lane of this section (LF.8) — the
three account families over the full fourteen-row before/after window
vector with policy-referenced limits and one fixed opaque-span length
(RD-R3-C-1), the meter charge schema with frozen per-counter units,
one checked charge per event, and the per-envelope reset at raw-leaf
start (RD-R3-C-2), the `Acct_R3`/`Aggregate_R3` conservation
equations with the frozen child orders, the rule-36 pair aggregation
in canonical raw ordinal order, and the single bounded action
aggregate (RD-R3-C-3), the closed `AccountScopeV1` enum with the
uniform `EmptyReplacementField(i)` parameter encoding (RD-R3-C-4),
the material/depth recurrences as account-internal charging law with
the exact fourteen-counter mapping (RD-R3-C-5), the static-definition
form and field-13/policy reconciliation without an A4-O byte change
(RD-R3-C-6), and the field-17 commitment metering equality (§4.2.3) —
with the cut-global lines LF.9–LF.13 checked in §4.4. Fourteen
registered decisions (RD-R3-A-1 through RD-R3-A-4, RD-R3-B-1 through
RD-R3-B-4, RD-R3-C-1 through RD-R3-C-6) discharge the fourteen
catalogued §13.4a underdeterminations; every ledger line LF.1–LF.13
of plan §13.4 checks; the A4-R3 cut is complete.

A4-R3 does NOT close: numerical `TypeIdV1` assignment and the
complete registry/source integration join (R2e, which may not change
an edge, reorder a predecessor, add a hidden operation, or choose a
name); fixture and mutation regeneration (R4, which must bind the
repaired `0xe7` order explicitly and may not copy RC1 coverage
fixtures); any executable JG2b2b2b authority — including the
non-authoritative metered batch API that first makes the 4.1.2
operation rows executable, and any verified profile token, none of
which may exist until the meter is available; and any theorem — the
three preservation descriptors remain unproved typed proposition
syntax, the sequential-versus-direct and full-projection equalities
remain unminted, and the record-§10.19.6 truth-audit gate persists
unweakened as a frozen input to the mandatory separately versioned
theory corrigendum. This cut mints no verified profile, executable
interface, envelope census, pair disposition, occurrence
classification, action image instance, theorem, generic law, cubical
bridge, `GCap`, `gamma`, or selective authority. The next lawful move
is A4-R2e, the registry/source join, per the plan §6.4
implementation-lane chain (`R2d roots and runtime closure -> A4-R3
failure/resource machine -> A4-R2e registry/source join -> A4-R4
fixtures and mutations`).
