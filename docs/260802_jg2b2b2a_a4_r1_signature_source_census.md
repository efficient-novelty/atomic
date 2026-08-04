# JG2b2b2a-A4-R1 Opaque Birth Signature and Source Census

Date: 2026-08-02

Status: **A4-R1, A4-R2a, AND A4-R2b FROZEN 2026-08-02; A4-R2c ACTIVE.** This independently audited
record removes the first A4
RC1 implementation-time choice by fixing the ordinary evaluation signature,
the full fourteen-field current-occurrence identity, and a noncircular older-
public source census. It mints no occurrence, export, envelope, profile,
classifier, theorem, or factual disposition. A4-R2c is the active typed-schema/
codec frontier; R2d, A4-R3, the R2e integration join, A4-R4, and
JG2b2b2b remain downstream.

Normative parents are the exact JG2b1b2 complete-through-head history,
JG2b2b0 structural occurrence grammar, A1, A2-O, and A3-O records. Existing
`Term`, `Declaration`, `UncheckedSignature`, `DependentContext`, `Digest`,
`GlobalId`, history-event ID, declaration-root, node-kind, and structural-path
codecs are inherited byte-for-byte unless this record explicitly adds a root.

## 0. Construction DAG and parent concordance

The future source-census verifier starts with an independently reminted
`VerifiedOrdinaryDependentEnvelopeProfileDefinitionV1 K`, an independently
reminted `H`, the structural-occurrence and particular-substitution parents
owned by `K`, and the one identity-matching `&Kernel`. Before inspecting any
birth, it remints every remintable parent and requires the complete A4 parent
DAG to agree. In particular, the JG1, JG2a constructor, JG2a scope,
JG2b2a protocol, JG2b2b0 grammar, and JG2b2b1 protocol bindings owned by `K`
must agree with one another, and every JG1/constructor/scope/kernel-protocol/
normalizer-protocol/configuration binding shared with `H` must agree exactly.
Failure constructs no signature, seed, source, census, or disposition.

The only permitted construction order is:

```text
reminted (K,H,kernel,grammar parents)
  -> exact parent concordance and ProfileManifestIdV1
  -> H-only declaration births and birth ordinal b
  -> historical predecessor/successor and strict prefixes
  -> H-only structural pre-occurrences
  -> SigmaAtBirth(H,b)
  -> dual local-context replay
  -> OccurrenceId14V1 values
  -> U_b and Pub_<b(H)
  -> BirthSourceCensusV1
  -> per-occurrence envelope iteration.
```

No caller occurrence, source vector, signature, manifest digest, publicness
flag, or constructor result seeds any arrow in this DAG.

## 1. Four signature roles that must not be collapsed

Let `H` be a reminted
`VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1`. Occurrence
construction begins with an unencoded, non-authoritative `OccurrenceSeedV1`
obtained only by exhaustive traversal of `H`:

```text
OccurrenceSeedV1 =
  (declaration_birth, declaration_root, structural_path, node_kind,
   normalized_structural_subject : Term,
   recomputed_structural_subject_digest : Digest)

b = declaration_birth.event_ordinal : u64.
```

The seed is loop state, not a codec, caller input, occurrence identity, or
authority. The verifier first derives `b` from it, then replays the signatures
below, then verifies the local context under those signatures, and only then
mints `OccurrenceId14V1 o`. The notation `Sigma(H,o)` used downstream is
post-construction shorthand for `SigmaAtBirth(H,b)` after every birth field of
`o` has replayed against that seed; it is never used to discover `b` or the
seed. This order removes any `o -> Sigma -> o` cycle.

The verifier checks `b < H.event_count`, converts `b:u64` to `usize` by checked
conversion, and converts every persistent arena index to `u32` with checked
conversion. Four signature roles are distinct:

1. a strict per-declaration historical prefix, used only to derive occurrence
   provenance and local contexts;
2. the exact JG2a predecessor/full successor at event `b`, used as stage
   replay evidence;
3. the complete terminal signature of `H`, used only for exhaustive-history
   cross-checks; and
4. the common opaque evaluator `Sigma(H,o)`, used by every A2/A3 check.

No one of the first three may be substituted for the fourth.

### 1.1 Full historical birth successor

Let `i=usize::try_from(b)` and let `j=i.checked_add(1)`; conversion or addition
failure aborts. Select exactly `stage=H.reconstructed_stages[i]` and
`event=H.contiguous_chain.events[i]` through the existing private
`reconstructed_stages_for_next_gate()` and `contiguous_chain_for_next_gate()`
projections, plus the privately retained matching kernel. Independently derive:

```text
S_pre   = H.contiguous_chain.replay_boundary_after(kernel,i),
S_event = kernel.verify_extension(S_pre,event.normalized_extension),
S_pref  = H.contiguous_chain.replay_boundary_after(kernel,j),
S_stage = stage.successor_boundary.
```

Before accepting the successor, require full canonical equality of `S_pre`
with `stage.predecessor_boundary`; full canonical equality of
`event.normalized_extension` with `stage.normalized_candidate`; equality of
the event ID and ordinal with the seed's declaration birth; equality of the
event predecessor/successor digests and checked `u64` declaration counts with
`S_pre`/`S_event`; and equality of `event.exact_stage_manifest_digest` with
`stage.manifest_digest`. Then require `S_event`, `S_pref`, and `S_stage` to
have byte-identical complete normalized wires, equal
`VerifiedSignature::digest()` values, and equal checked `u64` declaration
counts. Digest equality never replaces a full-wire comparison. Define their
common value

```text
S^hist_b(H).
```

Finally, let `n` be its checked `usize` declaration count and require its full
normalized wire to equal the first `n` declarations of `H.terminal_boundary`.
The terminal boundary must contain at least `n` declarations. This terminal
prefix is a completeness cross-check only; it is never the evaluator or the
authoritative traversal source.

It contains all declarations born before `b`, the complete extension born at
`b`, and no later declaration. Digest equality never replaces the full-byte
comparisons.

### 1.2 Older-body-erased evaluator

The current kernel unfolds `Global(g)` whenever `g` retains a body. Therefore
using `S^hist_b` would expose older private bodies through normalization.
Construct `OpaqueProjection_b(S^hist_b)` in declaration order by resolving
exactly one exhaustive declaration-birth entry for every `GlobalId`:

```text
Birth(g) < b  -> retain the exact ID and normalized type; set body=None;
Birth(g) = b  -> retain the complete normalized declaration byte-for-byte;
Birth(g) > b, missing, or ambiguous -> Abort.
```

Freshly call `kernel.verify_signature` on the whole projection. Require the
verified normalized bytes to equal the projected input; the projection and
`S^hist_b` must have identical declaration count/order, IDs, and normalized
types; every current-birth body must be byte-identical; every older body must
be absent. Define

```text
SigmaAtBirth(H,b) = verify_kernel(OpaqueProjection_b(S^hist_b(H))),
Sigma(H,o)        = SigmaAtBirth(H,o.birth_event_ordinal)
                    after exact occurrence-seed replay.
```

Every carrier, `TypeView`, `TermView`, decoder, builder, normalizer, support
replay, quotient reconstruction, and action for `(H,o)` uses only this
`Sigma(H,o)`. Same-birth globals may unfold; strictly older globals cannot.

The alternatives are rejected exactly: the predecessor omits current
globals, a strict declaration prefix varies with the declaration and omits
co-born globals, the terminal signature admits later births, and the full
successor exposes older bodies.

## 2. Exact strict-prefix and current-occurrence derivation

For a verified declaration birth `d` with terminal declaration ordinal `t`,
define

```text
StrictPrefix(H,d) = S^hist_b(H).declarations[0..t].
```

The declaration at `t` is excluded. Thus a prefix contains all older-event
declarations and all earlier declarations in the same birth extension. The
verifier requires:

1. `t < |S^hist_b|`;
2. `S^hist_b[t].global_id = d.global_id`;
3. `d` agrees with the unique exhaustive birth entry in event ID/ordinal,
   extension offset, global ID, and terminal ordinal;
4. `S^hist_b[t]` is byte-identical to
   `stage[b].normalized_candidate[d.extension_offset]` and to the corresponding
   declaration in the retained event/terminal history; and
5. independently verifying `StrictPrefix` succeeds.

The resulting verified-prefix signature digest is
`BirthPrefixSignatureDigest`. It is not an unchecked slice digest and is not
merely the whole event predecessor when `d` has earlier same-event siblings.

Traverse the birth-stage normalized declaration, never terminal or caller
syntax, with the exact JG2b2b0 roots and preorder. Derive the oldest-first local
context by appending a parameter type only on `PiBody`, `SigmaBody`, and
`LambdaBody`. Verify that context under `StrictPrefix`; independently reverify
it under `Sigma(H,o)` and require byte equality before it can be used by a raw
decoder. The JG2b2b0 bottom-up structural-subject digest and exact path are
recomputed from the same normalized declaration.

## 3. Current occurrence identities

### 3.1 Exact authority identity

`OccurrenceId14V1` begins with root `0xea`, then
`schema_version:u16=1`, then the exact fourteen JG2b2b0 fields in order:

| # | Field | Exact V1 type/codec |
| ---: | --- | --- |
| 1 | `CompleteThroughHeadCommitment` | `Digest` |
| 2 | `BirthEventId` | `GenerativeHistoryEventIdV1` (its retained `Digest`) |
| 3 | `BirthEventOrdinal` | `u64` |
| 4 | `DeclarationGlobalId` | `GlobalId` (its retained `Digest`) |
| 5 | `BirthExtensionOffset` | `u64` |
| 6 | `TerminalDeclarationOrdinal` | `u64` |
| 7 | `BirthPrefixSignatureDigest` | `Digest` from section 2 |
| 8 | `DeclarationRoot` | inherited tags `DeclarationType=0x00`, `DeclarationBody=0x01` |
| 9 | `BinderDepth` | `u16` |
| 10 | `LocalBinderContextDigest` | `Digest` of the complete context under the domain below |
| 11 | `StructuralPath` | `u64` count then inherited one-byte path tags `0x20..0x2c` |
| 12 | `NodeKind` | inherited one-byte node tags `0x10..0x1b` |
| 13 | `NormalizedStructuralSubjectDigest` | exact JG2b2b0 `Digest` |
| 14 | `ExactStageEvidenceDigest` | birth-stage manifest `Digest` |

The identity domain is

```text
law-v2/jg2b2b2a/ordinary-exact-occurrence-id14/v1
```

and the local-context domain is

```text
law-v2/jg2b2b2a/ordinary-local-binder-context/v1
```

Field 10 is exactly

```text
Digest::of_canonical(
  "law-v2/jg2b2b2a/ordinary-local-binder-context/v1",
  strict_prefix_verified_context.normalized_wire())
```

where the normalized wire has already replayed byte-identically under both the
strict prefix and `SigmaAtBirth(H,b)`. It is not the kernel's separate
`dependent-context-v1` receipt digest, although the checked owner retains that
receipt as evidence.

For path length `p`, the exact identity length is `592+p` bytes. Full identity
bytes decide equality and ordering; an identity digest may only accelerate a
lookup.

The exact stage digest must equal both the reconstructed stage manifest and
the contiguous event's retained exact-stage digest. The declaration root must
exist (`DeclarationBody` is absent when the declaration has no body), and the
path, binder depth, node kind, subject, and local context must all replay.

### 3.2 Normalized structural key

`NormalizedOccurrenceKeyV1` begins with root `0xeb`, then `u16=1`, and encodes
identity fields 2--13 in their relative order, omitting exactly fields 1 and
14. Its domain is

```text
law-v2/jg2b2b2a/ordinary-normalized-occurrence-key/v1
```

Its length is `434+p` bytes. This is the JG2b2b0 normalized comparison domain;
it never replaces the stronger exact authority identity retained by a checked
census.

### 3.3 Checked current entry

The checked census privately owns, for every identity, the full normalized
`Term`, full normalized `DependentContext`, strict-prefix receipt, `Sigma`
context-replay receipt, declaration/stage references, and traversal evidence.
Those are evidence, not additional identity fields. `U_b` contains every node
under the type and each present body of every declaration born at event `b`.
It is sorted lexicographically by complete `OccurrenceId14V1` bytes. Duplicate
bytes abort; they are never deduplicated.

## 4. Noncircular older opaque exports

JG2a has already certified each event boundary as the exact public signature.
That supplies one public binding `g:T` per declaration, but no composite
indexed-interface field census. Ordinary V1 therefore freezes exactly one
public-field path:

```text
PublicFieldPathV1::DeclarationField = 0x00.
```

It denotes the public binding `g:T`, not a declaration AST type-root/body
projection. A richer public-field grammar requires V2 or later authority.

### 4.1 Export identity

`ExportIdV1` begins with root `0xec`, then `u16=1`, and encodes:

```text
(complete_through_head_commitment : Digest,
 birth_event_id                   : GenerativeHistoryEventIdV1,
 birth_event_ordinal              : u64,
 declaration_global_id            : GlobalId,
 birth_extension_offset           : u64,
 terminal_declaration_ordinal     : u64,
 birth_prefix_signature_digest    : Digest,
 exact_stage_evidence_digest      : Digest).
```

Its domain is

```text
law-v2/jg2b2b2a/ordinary-older-public-export-id/v1
```

and its exact length is 422 bytes.

### 4.2 Full export value

`OlderPublicExportV1` begins with root `0xed`, then `u16=1`, and encodes:

```text
(export_id                       : ExportIdV1,
 public_field_path               : DeclarationField,
 public_global_value             : Term::Global(export_id.global_id),
 normalized_public_type          : Term,
 export_birth_successor_digest   : Digest).
```

Its domain is

```text
law-v2/jg2b2b2a/ordinary-older-public-export/v1
```

No body field or body projection exists. The checked object privately owns
`H`, the exact birth stage/prefix, the full `g:T` declaration, and normalized
`TypeFormation(T)` plus `HasType(Global(g),T)` receipts. `T` is metadata for a
`TermView`; it is never a second source term.

For target birth `b`, derive `Pub_<b(H)` as exactly one export for every
exhaustive declaration-birth entry with event ordinal `< b`. Read `g:T` at its
birth-stage successor terminal ordinal and require byte equality with all
retained history copies. Its cardinality must equal the event-`b` predecessor
declaration count; terminal ordinals must be exactly
`0..predecessor_count-1`; every older birth appears once; none at/after `b`
appears. Sort by complete `(ExportIdV1 bytes, DeclarationField byte)`.

This consumes no indexed-interface census, carrier result, classifier,
disposition, caller publicness flag, or caller field path.

## 5. Opaque signature binding and complete census codec

`BirthOpaqueSignatureBindingV1` begins with root `0xee`, then `u16=1`, and
encodes:

```text
(complete_through_head_commitment : Digest,
 target_birth_event_id            : GenerativeHistoryEventIdV1,
 target_birth_event_ordinal       : u64,
 exact_full_successor_digest      : Digest,
 opaque_normalized_signature_wire : UncheckedSignature,
 opaque_verified_signature_digest : Digest).
```

Its domain is

```text
law-v2/jg2b2b2a/ordinary-opaque-birth-signature/v1
```

The opaque checked value privately owns both the full `S^hist_b` and verified
`Sigma(H,o)`, not only these identity projections.

Define

```text
ProfileManifestIdV1 = semantic newtype over Digest
```

with no root, version, or wrapper bytes of its own. It encodes as exactly the
79-byte canonical `Digest` value

```text
Digest::of_canonical(
  "law-v2/jg2b2b2a/ordinary-profile-manifest/v1",
  K.owned_full_profile_manifest()).
```

The manifest includes its `0xf9` root in those canonical bytes. The sole
profile verifier independently reconstructs the complete manifest from its
reminted parents, compares the full bytes before hashing, and requires the
result to equal `K`'s retained manifest identity. A caller digest is never an
input. This construction precedes every history-specific census and is
nonrecursive because the manifest contains no history, runtime signature,
occurrence, export, or census instance.

`BirthSourceCensusV1` begins with root `0xef` and encodes:

```text
(source_census_schema_version : u16 = 1,
 profile_manifest_id          : ProfileManifestIdV1,
 complete_head_commitment     : Digest,
 target_birth_event_id        : GenerativeHistoryEventIdV1,
 target_birth_event_ordinal   : u64,
 opaque_signature             : BirthOpaqueSignatureBindingV1,
 current_sources              : vector<OccurrenceId14V1>,
 older_public_sources         : vector<OlderPublicExportV1>,
 source_refs                  : vector<SourceRefV1>).
```

Its domain is

```text
law-v2/jg2b2b2a/ordinary-birth-source-census/v1
```

`ProfileManifestIdV1` is the exact manifest identity above, not the opaque
profile token. The checked census owner retains `K` and its full manifest/
parents separately and first enforces section 0 parent concordance.

The first two source vectors use the canonical orders above. The final vector
must equal, byte-for-byte,

```text
map Current(U_b) ++ map OlderPublic(Pub_<b(H)).
```

`SourceRefV1` has exact tags and payloads:

```text
Current     = 0x00 followed by full OccurrenceId14V1,
OlderPublic = 0x01 followed by full ExportIdV1 and DeclarationField=0x00.
```

Raw enumeration may iterate checked offsets, but every emitted source leaf
retains the full identity plus a separate checked `u32` source ordinal. No
caller identity, signature, source list, field path, or publicness bit enters
the census.

The source count is measured before sorting, including duplicates, and must
satisfy

```text
|U_b| + |Pub_<b(H)| <= 4096.
```

The lexicographically first adjacent duplicate in either sorted class is an
aborting source-census history-schema failure. Missing history projections,
birth/stage/prefix/traversal/context drift, an opaque-signature replay failure,
or a private-body request constructs no census and no pair disposition.

## 6. Exact decoder consequences

The A3 source grammar is now

```text
SourceRefV1 ::= Current(OccurrenceId14V1)                // tag 0x00
              | OlderPublic(ExportIdV1,DeclarationField) // tag 0x01.
```

`Current` resolves only through the privately owned census to the normalized
birth-stage node and local context. `OlderPublic` resolves only to
`Global(g)` plus checked `T` metadata. All view/check/normalization calls use
`Sigma(H,o)`, never `S^hist_b`, a strict prefix, or the terminal signature.
Before use, a current local context is replayed under both its strict prefix
and `Sigma`; the normalized bytes must agree.

Infrastructure mismatch in birth, stage, prefix, traversal, context, export,
or opaque-signature replay is `Abort`. Only after those prerequisites close
may a candidate-derived `TypeView`/`TermView` judgment be a checked `False`.
An attempt to project an older body is a checked support/source false, but the
internal generator never creates such a projection.

## 7. Level-I/Level-II correction

Level I contains proposed syntax only:

```text
OrdinaryLevelOneProposalV1 ::=
  CarrierProposal(carrier tag, raw indices, raw value)
| RawConstructorProposal(constructor tag, RawCode_c)
| FormalActionProposal(action kind, raw action syntax).
```

Decoded and built records are checked intermediate values, not Level-I
proposals. Level II is the closed sum

```text
CheckedOrdinaryLevelTwoV1 ::=
  CheckedCarrier(Sigma binding, raw carrier proposal, normalized value,
                 full formation/replay evidence)
| CheckedDecoded(Sigma binding, raw code, decoded value, decode evidence)
| CheckedBuilt(Sigma binding, decoded value, Built_c, premise evidence)
| CheckedCandidateEnvelope(kappa,H,o,Sigma(H,o),c,r,E, Match evidence)
| CheckedActionImage(kappa,H,o,Sigma(H,o),c, owned base candidate,
                     normalized action, owned trace, image).
```

Every variant owns or has a checked arena index to the one full
`BirthOpaqueSignatureBindingV1` and its privately retained signatures. A1
field 3's future census owner retains the full binding and source evidence;
field 4 uses `OccurrenceId14V1` plus `NormalizedOccurrenceKeyV1`; field 7
retains the exact predecessor/full successor, strict declaration prefix,
common opaque `Sigma`, and both context-replay receipts. A nested index is
meaningful only inside this owner; canonical identities remain full values.

No Level-II object is thereby a factual occurrence classification or pair
disposition.

## 8. Continuation and non-authority

R1 chooses no expected structure and inspects no live history. It specifies a
future derivation from opaque history; it constructs none now. A4-R2a froze
roots `0xea`--`0xef` and their six domains in the common collision-free root
namespace. The post-R3 R2e join must place their schemas in the complete
registry without changing roots `0xe8`, `0xe9`, or `0xf8`--`0xff`.

A4-R1 froze only after an independent audit rechecked the construction DAG,
parent concordance, full event/stage/terminal replay, body-erasure semantics,
fixed sizes, inherited tags, root collisions, census completeness, and source
ordering. A4-R2a froze the common acyclic schema/path/ownership foundation,
and A4-R2b independently froze Level-I/raw/decoded/built and early-Level-II
payload/tag schemas. Active R2c closes A1/evidence/support/subjects; R2d closes
quotient/outcome/coverage/action layouts plus the joined root `0xe3`. A4-R3
then closes the failure and exact resource machine; the
post-R3 R2e join closes the registry and implementation-protocol identity.
A4-R4 then regenerates the complete
static/manifest fixtures and mutation suite. Only an independently audited R4
may unblock JG2b2b2b.
