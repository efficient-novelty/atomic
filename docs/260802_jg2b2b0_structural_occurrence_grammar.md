# JG2b2b0 Structural Occurrence Grammar

Date: 2026-08-02

Status: DISCHARGED as a grammar-only subgate. JG2b2b1 is now also discharged as
a particular-instance subgate, and the JG2b2b2-P0 ontology audit is discharged.
A0 is discharged; the A1 envelope protocol, A2-O nine-sort carrier/action
substrate, A3-O constructor realization/classifier specification, and A4-O
RC1 ordinary complete-profile candidate was rejected; A4-R1 signature/source
closure, A4-R2a acyclic foundation, and A4-R2b level-relation schemas are frozen, A4-R2c is active, and A2-C remains open. JG2b2b2b is blocked by A4-R4 and no executable indexed-interface
authority exists yet. JG2b2c factual
complete-through-head enumeration remains
blocked until all of JG2b2b0--JG2b2b3c have combined into universal authority.

## Scope and authority boundary

JG2b2b0 freezes the structural language in which later history-bound public
occurrences can be named. The implemented opaque public token is
`VerifiedGenerativeStructuralOccurrenceGrammarV1`. It is remintable grammar
authority, not linear factual evidence.

The verifier consumes the already verified JG2b2a protocol, JG1 grammar, JG2a
constructor grammar, a matching kernel, and the closed
`GenerativeStructuralOccurrenceGrammarManifestV1`. It accepts no history, stage
vector, declaration, term, occurrence list, path, context, substitution,
constructor tag, classifier, theorem, expected count, or Boolean claim. In
particular, neither a
`VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1` nor any digest
derived from one is an input to this gate.

The token freezes only:

- the declaration-root vocabulary and root order;
- the normalized-term node vocabulary;
- the parent/child edge vocabulary, compatibility relation, and child order;
- deterministic declaration-root and term preorder;
- the three binder-entering edges and the derivation of binder paths;
- the fourteen-field vocabulary and ordering required of a later factual
  occurrence identity;
- canonical domains/codecs and the matching kernel, normalizer, and resource
  profile identities; and
- the rule that normalized structural jurisdiction and exact-source evidence
  remain distinct.

It mints no history, event, birth, declaration, occurrence, local context,
typing judgment, substitution, interface realization, classifier disposition,
theorem subject, theorem application, functor law, naturality equation, census,
support, carrier, quotient, `gamma`, bootstrap, or selection fact.

The implemented authority metadata boundary is correspondingly exact:

```text
jg2b2b0_structural_occurrence_grammar = true
derived_public_occurrence_authority = false
particular_open_typed_substitution_authority = false
complete_admissible_substitution_universe_authority = false
full_kernel_typed_substitution_metatheory = false
executable_indexed_interface_authority = false
universal_naturality_authority = false
```

## Exact declaration-root vocabulary

`GenerativeStructuralOccurrenceRootV1` contains exactly these possible roots,
in this order:

1. `DeclarationType`, always present and rooted at `Declaration.ty`;
2. `DeclarationBody`, present exactly when `Declaration.body` is `Some`.

The type root is completely traversed before the body root begins. Equal terms
in the two fields remain distinct because the root tag is identity-bearing.
The grammar verifier/token does not consume or traverse a declaration. The
public `proposed_generative_structural_declaration_traversal_v1` helper can run
this preorder over caller-supplied syntax, but its result is explicitly
unverified and carries no declaration provenance or occurrence authority.

## Exact term-node vocabulary

`GenerativeStructuralOccurrenceNodeKindV1` contains exactly the twelve current
`Term` forms exposed by `pen-kernel`, in this order:

1. `Sort`
2. `Var`
3. `Global`
4. `Pi`
5. `Sigma`
6. `Lambda`
7. `Apply`
8. `Pair`
9. `First`
10. `Second`
11. `UnitType`
12. `Unit`

This is a closed grammar. There is no `Other`, caller-defined node tag, or
digest-only extension point. A future kernel term form requires a versioned
grammar change rather than silent omission. The same tags describe either raw
caller syntax or normalized syntax; only a later authority can establish that
a traversed declaration is the normalized declaration derived from history.

## Exact edge vocabulary and child order

`GenerativeStructuralOccurrencePathStepV1` contains exactly these thirteen
parent/child edges, in this global order:

1. `PiParameter`
2. `PiBody`
3. `SigmaParameter`
4. `SigmaBody`
5. `LambdaParameterType`
6. `LambdaBody`
7. `ApplyFunction`
8. `ApplyArgument`
9. `PairSigmaType`
10. `PairFirst`
11. `PairSecond`
12. `FirstPair`
13. `SecondPair`

Their compatibility and per-parent child order are closed:

| Parent node | Children, in traversal order |
| --- | --- |
| `Pi` | `PiParameter`, `PiBody` |
| `Sigma` | `SigmaParameter`, `SigmaBody` |
| `Lambda` | `LambdaParameterType`, `LambdaBody` |
| `Apply` | `ApplyFunction`, `ApplyArgument` |
| `Pair` | `PairSigmaType`, `PairFirst`, `PairSecond` |
| `First` | `FirstPair` |
| `Second` | `SecondPair` |
| `Sort`, `Var`, `Global`, `UnitType`, `Unit` | no children |

An edge attached to any other parent is ill-formed. No child can be skipped,
duplicated, reordered, or represented by an ordinal without its frozen edge
tag.

## Deterministic preorder

The implemented preorder over one supplied declaration is:

1. `DeclarationType`, followed by optional `DeclarationBody`;
2. at each root, visit the current term node before its children;
3. visit children in the compatibility-table order above.

JG2b2b0 does not order birth events or declarations across a history. JG2b2c
must derive those outer orders from later opaque history/birth authority before
applying this within-declaration preorder. Repeated equal subterms at distinct
paths produce distinct positions.

## Structural paths and binder paths

A structural path is the pair

```text
(declaration_root, ordered_edge_word).
```

The empty edge word names the term at the selected declaration root. Every
nonempty path must be derivable one compatible edge at a time from the node at
its parent prefix. A proposed path is interpreted over exactly the caller term
supplied to the unverified helper. JG2b2c must instead reconstruct and traverse
normalized declarations from opaque history evidence; it may not assert that a
normalized path points into pre-normalization raw syntax.

Exactly three edges enter a binder body:

- `PiBody`, whose binder type is the sibling selected by `PiParameter`;
- `SigmaBody`, whose binder type is the sibling selected by `SigmaParameter`;
- `LambdaBody`, whose binder type is the sibling selected by
  `LambdaParameterType`.

All other edges preserve binder depth. Binder ancestry is derived, never caller
supplied: scan the structural path from root to node and retain the prefixes
whose incoming step is one of those three binder-entering edges, outermost
first. The unverified traversal records only the derived `binder_depth`; a later
factual verifier must recompute both that depth and the local binder context
from the path before binding `LocalBinderContextDigest`. Dependent contexts are
ordered oldest binder first; de Bruijn `Var(0)` denotes the newest binder.

This rule is structural only. It does not assert that a derived binder type is
well formed or that the focused term is well typed; the later classifier and
factual census bridge must supply that node-local evidence.

## Frozen identity vocabularies

`GenerativeStructuralOccurrenceIdentityDomainV1` freezes exactly two comparison
domains, in this order:

1. `NormalizedStructuralOccurrence`
2. `ExactOccurrenceAuthority`

`GenerativeStructuralOccurrenceIdentityFieldV1` freezes exactly fourteen fields
that every later factual occurrence identity must retain, in this order:

1. `CompleteThroughHeadCommitment`
2. `BirthEventId`
3. `BirthEventOrdinal`
4. `DeclarationGlobalId`
5. `BirthExtensionOffset`
6. `TerminalDeclarationOrdinal`
7. `BirthPrefixSignatureDigest`
8. `DeclarationRoot`
9. `BinderDepth`
10. `LocalBinderContextDigest`
11. `StructuralPath`
12. `NodeKind`
13. `NormalizedStructuralSubjectDigest`
14. `ExactStageEvidenceDigest`

The two domain tags distinguish normalized structural comparison from exact
authority comparison, while the ordered field enum freezes everything a later
factual identity must retain. Neither enum is a factual identity instance. The
separation policy also remains visible within the fields:
`NormalizedStructuralSubjectDigest` records the normalized structural subject,
while `ExactStageEvidenceDigest` and `CompleteThroughHeadCommitment` bind exact
provenance. Neither may replace the other.

The normalized domain excludes `CompleteThroughHeadCommitment` and
`ExactStageEvidenceDigest`. The exact-authority domain extends the normalized
identity with precisely those two fields. This permits normalized structural
comparison without erasing the stronger exact provenance retained by the
factual authority.

`DeclarationRoot` plus `StructuralPath` keeps equal subjects at distinct roots
or paths distinct. `BinderDepth` and `LocalBinderContextDigest` must be
recomputed from that path rather than accepted independently. The birth event,
global, offsets, terminal ordinal, and strict birth-prefix signature field keep
the later node attached to its exact declaration birth without treating a
trusted-scope `history_digest` as evidence.

JG2b2b0 fills none of these fields and intentionally exports no factual
occurrence or occurrence-ID type. A digest or unverified traversal without the
later verifier-retained history, typing, and exact-stage evidence cannot mint an
occurrence.

## Exact semantic-rule vocabulary

`GenerativeStructuralOccurrenceGrammarRuleV1` freezes these eighteen rules, in
this order:

1. `GrammarIsHistoryIndependent`
2. `DeclarationRootsAreTypeThenPresentBody`
3. `TraversalIsDeterministicPreorder`
4. `ChildStepsAreClosedFieldSpecificAndParentChecked`
5. `OnlyDependentBodiesEnterBinders`
6. `ContextEntriesAreOldestFirst`
7. `DeBruijnZeroNamesNewestBinder`
8. `BinderDepthAndContextDeriveFromPath`
9. `EqualSubjectsAtDistinctPathsRemainDistinct`
10. `NormalizedSubjectAndExactEvidenceRemainSeparate`
11. `NormalizedIdentityExcludesCompleteHeadAndExactStageEvidence`
12. `ExactAuthorityIdentityExtendsNormalizedWithCompleteHeadAndExactStageEvidence`
13. `StructuralSubjectDigestIsBottomUpFieldOrderedMerkle`
14. `FutureOccurrenceIdentityRetainsEveryFrozenField`
15. `CallerSyntaxTraversalMintsNoFactualOccurrence`
16. `NestedTypingAndClassificationRequireLaterAuthority`
17. `ResourceExhaustionMintsNoPartialFact`
18. `GrammarMintsNoSubstitutionNaturalityOrDownstreamAuthority`

## Explicitly unverified traversal helper

The public
`proposed_generative_structural_declaration_traversal_v1` helper takes the
verified grammar token, an identity-matching kernel, and one caller-supplied
`Declaration`. It returns
`ProposedGenerativeStructuralDeclarationTraversalV1`, containing a declaration-
input digest and an ordered vector of `GenerativeStructuralTraversalNodeV1`.
Each proposed node exposes its root, optional parent ordinal, optional incoming
field step, structural depth, derived binder depth, node kind, and structural-
subject digest; `structural_path` reconstructs its edge word from the private
parent arena.

The `Proposed` name marks the non-authoritative boundary. The helper does not
verify that the declaration came from any signature or history, does not normalize it, and does
not type its nodes. Even a traversal of caller syntax that happens to equal a
verified normalized declaration mints no provenance or occurrence fact.
JG2b2c must derive its own normalized declarations from opaque complete-through-
head evidence and then re-establish every frozen identity field.

After traversal has passed its depth and operation checks, structural-subject
digests are computed once per node in reverse preorder. Each digest is the
domain-separated Merkle image of the exact local `Term` tag, any local scalar
or global payload, and already-computed child digests in frozen field order.
The declaration-input digest separately composes the declaration global ID,
type-root digest, body-presence tag, and optional body-root digest. No unchecked
caller subtree is recursively encoded, and successful hashing is linear in the
number of retained nodes.

## Resource and failure semantics

The implemented proposal helper is iterative and checks the grammar-bound
kernel configuration, `max_operations`, `max_depth`, binder-depth overflow,
`u32` node-ordinal overflow, allocation failure, and path lookup bounds. Any
failure returns no partial proposal. Parent/edge compatibility and child order
come from exhaustive private matching over `Term`; callers cannot inject an
edge.

Later factual traversals must preserve the same fail-closed behavior and apply
all additional matching kernel limits required by normalization and typing
before any fact is minted. Counts and products use checked arithmetic. There is
no truncating `.take`, caller cap, partial positive/negative fact, or expected-
count input.

Paths and binder contexts may be represented persistently so enumeration stays
linear in node count plus retained path/context structure. Any factual JG2b2c
matrix must separately check the exact `occurrence_count * 7` product.

## Frozen transcript identity

The static definition transcript binds the exact root, node, path-step,
binder-step, two-domain identity, fourteen-field future identity, and eighteen-
rule vocabularies. It is exactly 123 bytes under domain
`law-v2/jg2b2b0/structural-occurrence-grammar-definition/v1`, with digest:

```text
blake3:bb514d85e64f3f530633de7f0f5d48c17d61d4e6156b364061cd740accd94b80
```

`GenerativeStructuralOccurrenceDefinitionManifestV1` carries that static
definition. `GenerativeStructuralOccurrenceGrammarManifestV1` additionally
binds the exact JG2b2a protocol, JG1, constructor/scope grammar, kernel,
normalizer, and kernel-configuration identities under the distinct full-
manifest domain `law-v2/jg2b2b0/structural-occurrence-grammar/v1`.
Its independent 677-byte synthetic full-manifest codec fixture has digest
`blake3:5e25ebf83319b1d687a95e3a8720892f22ec669230ea1dd414eae891c3f5c798`,
pinning the full root tag and all seven upstream-binding field positions.

The static digest is an identifier, not evidence. Only successful closed
verification may mint `VerifiedGenerativeStructuralOccurrenceGrammarV1`.

The resulting crate passes 64 unit tests and 16 compile-fail doctests, strict
all-target clippy, the generative isolation checker and its 11 mutation tests,
and all three reviewed kernel dependency graphs. The adversarial traversal
suite includes a 10,000-node caller tree under a depth-one kernel profile; it
returns `DepthBudgetExhausted` before any recursive subtree encoding.

## Ordered continuation

The combined JG2b2b gate now proceeds as follows:

1. **JG2b2b0 -- structural occurrence grammar -- DISCHARGED 2026-08-02.** This
   document's history-free grammar-only authority.
2. **JG2b2b1 -- particular open typed substitution/context-morphism substrate
   -- DISCHARGED 2026-08-02.** Two aggregate kernel batches separately check the
   exact raw proposal and the rebuilt normalized proposal. Identity,
   composition, binder lift, and judgment reindexing derive and recheck only
   particular outputs; all generic-law authority remains false. See
   `docs/260802_jg2b2b1_particular_open_typed_substitution.md`.
3. **JG2b2b2-P0 -- indexed-interface ontology audit -- DISCHARGED
   2026-08-02.** Correct nine-sort/seven-constructor ownership without minting
   authority. See
   `docs/260802_jg2b2b2_indexed_interface_ontology_audit.md`.
4. **JG2b2b2a -- target-neutral ontology/representation freeze -- ORDINARY
   LANE FROZEN THROUGH A4-R2b; A4-R2c ACTIVE; A2-C OPEN.** A0 is discharged; A1 and the A2-O ordinary carrier/action substrate
   are frozen; A3-O has frozen the seven ordinary constructor telescopes, slot
   grammars, checked realization/classifier predicates, principal-root
   selectors, and naturality subjects. The cubical A2-C ontologies remain open.
   A4-O RC1 was rejected; `R2c -> R2d -> R3 -> R2e -> R4` must close the complete selectable profile
   transcript/resource contract. This classifies no occurrence.
5. **JG2b2b2b -- closed executable Rust grammar and particular fail-closed
   operations -- BLOCKED BY A4-R4.** Implement only the independently frozen
   repaired A4 representation.
6. **JG2b2b3a -- generic substitution proof and exact Rust/Agda
   correspondence.** Prove the unbounded substitution/typing/equality/
   normalization laws and bind them to the exact production Rust calculus.
7. **JG2b2b3b -- nine functor laws.** Prove identity and composition for each
   frozen indexed reindexing action, including the bi-indexed substitution
   actions.
8. **JG2b2b3c -- seven constructor squares and combined universal authority.**
   Prove every generic constructor naturality square and mint the combined
   package only after all earlier theorem and correspondence authorities agree.
   The theorem quantifies over formal derivations; each bounded Rust application
   remains conditional on successful replay, with exhaustion producing no fact.
9. **JG2b2c -- factual complete-through-head census.** Only then consume opaque
   history, enumerate occurrences, derive the exact occurrence-by-constructor
   matrix, and retain concrete theorem-application evidence.

No finite set of successful JG2b2b1 checks, including exhaustive checks within
a configured resource bound, substitutes for JG2b2b3a--JG2b2b3c.
