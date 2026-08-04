# JG2b2b1 Particular Open Typed Substitution

Date: 2026-08-02

Status: DISCHARGED as a particular-instance subgate. The aggregate JG2b2b gate
remains open. JG2b2b2-P0 and A0 are discharged; the A1 envelope protocol,
A2-O nine-sort carrier/action substrate, A3-O constructor realization/
classifier specification, A4-R1 signature/source closure, and A4-R2a acyclic
foundation and A4-R2b level-relation schemas are frozen; A4-O RC1 was rejected, A4-R2c is active, A2-C remains
open, and JG2b2b2b is blocked
by A4-R4.
JG2b2b3a--JG2b2b3c remain responsible for the
generic substitution metatheory, indexed functor laws, constructor naturality,
and combined universal authority. JG2b2c remains blocked behind that complete
package.

## Scope and authority boundary

JG2b2b1 implements evidence for one exact open typed substitution, or for one
exact result derived from already verified particular evidence. It does not
turn bounded computation into quantification. Its authority is deliberately
limited to the proposition that the complete retained instance was derived and
independently accepted by the matching kernel under the matching finite
resource profile.

The subgate can establish these particular facts:

- one exact dependently typed simultaneous substitution/context morphism;
- one internally derived identity substitution;
- one internally derived composite of two composable verified substitutions;
- one internally derived binder lift of a verified substitution; and
- one internally derived reindexing of one verified open judgment.

Each constructor produces a fresh, flat instance result. A derived token does
not contain a proof program whose unchecked recursive interpretation could
manufacture further facts, and a digest never substitutes for retained replay
evidence.

The implemented authority metadata boundary is:

```text
particular_open_typed_substitution_authority = true
complete_admissible_substitution_universe_authority = false
full_kernel_typed_substitution_metatheory = false
generic_identity_composition_lifting_authority = false
generic_typing_equality_preservation_authority = false
normalization_reindexing_compatibility_authority = false
executable_indexed_interface_authority = false
indexed_interface_functor_law_authority = false
universal_naturality_authority = false
derived_public_occurrence_authority = false
```

The first `true` flag says only that successful calls can mint particular replay
evidence. Every generic, universal, indexed-interface, occurrence, and
naturality flag remains false.

## Frozen protocol authority

`ParticularOpenTypedSubstitutionProtocolRuleV1` is the closed sixteen-rule
vocabulary, in this order:

1. `EvidenceIsParticularNotUniversal`
2. `MorphismOrientationIsDomainDeltaToCodomainGamma`
3. `ImagesAreCompleteAndOldestCodomainEntryFirst`
4. `ExpectedImageTypesDeriveFromPriorImages`
5. `RawAndNormalizedProposalsReceiveSeparateAggregateKernelReplays`
6. `AllContextsImagesAndJudgmentsAreKernelChecked`
7. `ExactInputsAndNormalizedReplayEvidenceRemainSeparate`
8. `IdentityCompositeLiftAndReindexOutputsAreInternallyDerived`
9. `EveryDerivedOutputIsIndependentlyRechecked`
10. `CallerExpectedTypesOutputsDigestsAndBooleansAreForbidden`
11. `AggregateResourcesFailClosedWithoutPartialEvidence`
12. `KernelLimitsDoNotExceedReviewedHardCeilings`
13. `ExhaustionMintsNoPositiveOrNegativeFact`
14. `ParticularEvidenceNeverEstablishesGenericLaws`
15. `CallerMintedParticularEvidenceIsNeverCensusInput`
16. `ProtocolMintsNoFactualCensusInterfaceFunctorNaturalityOrDownstreamAuthority`

`ParticularOpenTypedSubstitutionProtocolDefinitionV1` freezes the V1 schema,
resource policy, reviewed hard ceilings of 100,000 operations, depth 256, and
50,000 normalization-fuel steps, and the ordered rules. Its canonical
definition transcript is 39 bytes under
`law-v2/jg2b2b1/particular-open-typed-substitution-definition/v1`, with digest:

```text
blake3:460193d192ebab1a0c1d30227026c2c6e1cf23c069449939a6f4e3b3645518cb
```

`ParticularOpenTypedSubstitutionProtocolManifestV1` additionally binds the
exact structural-occurrence grammar, JG2b2a protocol, JG1 and constructor/scope
grammars, kernel, normalizer, and kernel configuration. Only successful closed
`verify_particular_open_typed_substitution_protocol_v1` verification mints the
opaque remintable
`VerifiedParticularOpenTypedSubstitutionProtocolV1`.

The independent synthetic full-manifest codec fixture is 672 bytes under
`law-v2/jg2b2b1/particular-open-typed-substitution-protocol/v1`, with digest:

```text
blake3:3a0c35e6c930450326cd4129d61db8cb60601a6cbab723ff0cd93cddd624d436
```

This fixture pins the full root and upstream-binding field order; its digest is
an identity fixture, not an authority token.

That protocol token freezes how later instances must be checked; it is not
itself a substitution or factual replay. Particular evidence separately binds
the exact protocol manifest and all instance inputs/results.

## Exact orientation and image order

Fix a verified signature `Sigma`, a domain/source dependent context `Delta`,
and a codomain/target dependent context

```text
Gamma = (A_0, A_1, ..., A_{n-1}),
```

where context entries are written oldest first and de Bruijn `Var(0)` names the
newest entry. A context morphism

```text
theta : Delta -> Gamma
```

is represented by exactly `n` images

```text
theta = (theta_0, theta_1, ..., theta_{n-1})
```

in oldest-codomain-entry-first order. Its dependent typing obligations are
derived, not supplied:

```text
Sigma ; Delta |- theta_i : A_i[theta_0, ..., theta_{i-1}]
```

for every `i < n`. Thus `|theta| = |Gamma|`; the domain length does not
determine the image count. The expected type for image `i` is computed from the
codomain entry and the already derived prefix of images. There is no caller
expected-type vector, caller image-order tag, or caller assertion that the
substitution is complete.

The empty codomain case `n = 0` is lawful. Both aggregate batches still include
the codomain and domain context checks (using the kernel-checked `UnitType`
formation sentinels), so an empty image vector never bypasses context replay.

This orientation is frozen throughout identity, composition, lifting, and
judgment reindexing. If

```text
theta : Delta -> Gamma
rho   : Epsilon -> Delta,
```

the internally derived composite is a particular morphism

```text
theta[rho] : Epsilon -> Gamma
```

whose images are obtained by reindexing every image of `theta` by `rho`. This
notation records the implemented direction only; it does not assert a generic
composition law.

## The two-aggregate-batch minting boundary

The base substitution verifier performs two complete aggregate kernel replay
batches. Local bounded preflight and internal transformation precede each
batch; that local work is not itself either replay pass and mints no authority.

### Batch 1: exact raw proposal

The verifier first validates the matching frozen protocol/configuration
identities, checks all caller-controlled material with checked size/depth
preflight, fallibly reserves the aggregate `Vec` storage, and derives every raw
expected image type from the exact raw codomain and the preceding exact raw
images. It then submits one aggregate
kernel batch, with one shared kernel budget, containing the exact raw source/
codomain context obligations and every exact raw image at its raw internally
derived type. It does not reset a fresh budget by calling the single-judgment
verifier once per image.

There is no caller expected-type vector. Batch 1 either checks the complete raw
proposal or returns no evidence. Its normalized contexts, images, expected
types, and judgments are intermediate verified outputs, not caller inputs and
not yet a minted substitution token.

### Batch 2: rebuilt normalized proposal

The verifier rebuilds the entire proposal from batch 1's normalized contexts
and images. It repeats the bounded size/depth/transform preflight and fallible
aggregate-buffer reservation, derives
the normalized dependent image obligations from those normalized values, and
submits a second aggregate kernel batch for independent checking of the
complete rebuilt normalized proposal.

Only agreement between the complete first and second batches permits minting.
The result retains the exact signature, verifier profile, raw contexts and
images, and exact input/evidence binding separately from the normalized
contexts, images, judgments, and replay binding. A successful prefix is not
partial evidence; exhaustion or any mismatch in either batch produces neither
a positive fact nor a negative theorem.

Each `VerifiedParticularOpenTypedSubstitutionImageV1` makes the separation
inspectable without making it forgeable: `raw_judgment` is the batch-one input,
`raw_normalized_judgment` its kernel result, `canonical_input_judgment` the
rebuilt batch-two input, and `canonical_normalized_judgment` the second kernel
result. The enclosing substitution separately retains the corresponding raw,
normalized, and rebuilt-canonical domain/codomain context judgments plus
`raw_proposal_digest`, `canonical_digest`, and the stronger `evidence_digest`.

Identity and composition derive their complete output internally and invoke
this base two-batch verifier. Binder lift additionally reindexes and jointly
replays the codomain parameter's exact source `TypeFormation` and the derived
domain `TypeFormation` before invoking the base verifier.
Judgment reindexing is a separate particular operation: after deriving the
target internally, it aggregate-batches the exact source judgment together
with that derived target judgment. In no case does internal construction
certify its own output.

## Particular evidence objects

### Direct substitution

`ProposedParticularOpenTypedSubstitutionV1` contains only the exact domain,
codomain, and ordered image vector. Successful direct verification returns
`VerifiedParticularOpenTypedSubstitutionV1` through
`verify_particular_open_typed_substitution_v1`; its per-image records are
`VerifiedParticularOpenTypedSubstitutionImageV1`.

Direct verification binds the exact signature, matching kernel/normalizer/
configuration, exact domain and codomain contexts, exact ordered images,
internally derived expected types, and the complete normalized kernel replay.
It retains normalized replay identity separately from exact input/evidence
identity. Definitionally equal presentations may therefore agree in the
normalized domain without erasing which exact instance was checked.

### Identity

`verify_particular_identity_open_typed_substitution_v1` derives the full de
Bruijn image vector from the exact caller context, then sends both that context
and the derived vector through the base two-batch verifier. The resulting
`VerifiedParticularIdentityOpenTypedSubstitutionV1` proves that particular
vector is a checked endomorphism of that context. It does not prove, for
arbitrary terms or judgments, `t[id] = t`, nor either
unit law for substitution composition.

### Composition

`verify_particular_composite_open_typed_substitution_v1` checks that the
signature and protocol identities agree and that the outer domain equals the
inner codomain at the canonical normalized boundary. It retains both exact
operand tokens, derives every composite image internally, and sends the
resulting substitution through the base two-batch verifier. Its result proves
only that exact composite is well typed and is retained in
`VerifiedParticularCompositeOpenTypedSubstitutionV1`. It does not prove
associativity, left/right unit, equality congruence, or closure under every resource-bounded
composition.

### Binder lift

`verify_particular_binder_lift_open_typed_substitution_v1` derives the domain
extension, weakened prior images, and newest bound-variable image from the
exact substitution and binder data. It first jointly kernel-replays the
codomain parameter's exact source
`TypeFormation` and derived domain `TypeFormation`, then sends the entire
lifted substitution through the base two-batch verifier. A
successful `VerifiedParticularBinderLiftOpenTypedSubstitutionV1` proves one lift.
It does not prove generic lift totality, iteration, composition, weakening,
typing-preservation, or commutation laws. In particular, fixed resource sets
need not be closed under lifting.

### Reindexed open judgment

`verify_particular_reindexed_open_judgment_v1` derives the complete substituted
form of one `TypeFormation`, `HasType`, or `DefinitionallyEqual` judgment and independently
kernel-checks both the required input and derived output evidence in its
aggregate batch. `VerifiedParticularReindexedOpenJudgmentV1` proves only that
exact output judgment. A reindexed
`DefinitionallyEqual` result is one kernel-replayed equality instance, not a
generic equality-preservation theorem.

## Exact non-theorems

No JG2b2b1 token, list of tokens, or exhaustive enumeration under a configured
finite bound proves any of the following:

```text
t[id] = t
(t[theta])[rho] = t[theta[rho]]
(theta[rho])[sigma] = theta[rho[sigma]]
id[theta] = theta = theta[id]
lift(theta) is generically total or commutes with composition
typing is preserved by every typed substitution
typed definitional equality is preserved by every typed substitution
normalization commutes with reindexing
any of the nine indexed-interface functor laws
any of the seven constructor naturality squares
```

Nor does the verifier establish a complete admissible substitution universe.
It also establishes no equality between independently minted instance tokens,
even when their normalized replay fields happen to agree.
These are statements over arbitrary formal derivations and substitutions, not
properties obtained by counting successful bounded examples. JG2b2b3a must
prove the full twelve-form kernel substitution metatheory over unbounded formal
derivations and bind it to the exact Rust representation. JG2b2b3b and
JG2b2b3c must then separately establish the indexed functor laws and naturality
squares. Bounded Rust replay remains conditional even after those proofs;
resource exhaustion still mints no fact.

## Failure and resource semantics

All counts, products, index conversions, and budget accumulations are checked.
The bound kernel configuration may be stricter than, but may not exceed, the
reviewed protocol ceilings of 100,000 operations, depth 256, and 50,000
normalization-fuel steps. The implementation must preflight all caller-
controlled syntax and fallibly
reserve its aggregate `Vec` output/replay buffers before replay and minting. An
allocation failure in those fallibly reserved buffers yields no token. This is
not a claim that Rust's recursive `Box<Term>` reconstruction has a fallible
allocator. The verifier must reject malformed context directions, incomplete
or reordered image vectors, mismatched signatures/configurations,
non-composable premises,
incorrect internally derived expected types, or any replay disagreement.

Resource exhaustion is epistemically neutral:

```text
success       -> one particular positive replay fact
type mismatch -> verifier failure, not a general negative theorem
exhaustion    -> no positive fact and no negative fact
```

There is no truncating enumeration, `.take`-style partial success, caller cap
that changes the theorem domain, or promotion of a completed prefix.

## Downstream consumption firewall

JG2b2b2a may use the existence and exact representation of the particular-
substitution substrate in its target-neutral ontology/representation freeze.
Frozen A2-O gives each of nine ordered sorts its dependent indices,
carrier/value syntax, well-formedness, typed/intensional equality,
normalization, variance, reindex recipe, and constructor-parametric finite
proposal operator. Frozen A3-O instantiates that operator with each of seven
constructors' dependent input/output telescope, finite slot grammar, checked
realization/classifier predicate, principal-root selector, and formal
naturality subject. Those schemas prove neither totality nor disjointness and
 classify no occurrence. A4-O RC1 was rejected; the chain
 `R2c -> R2d -> R3 -> R2e -> R4` must still bind the
 complete selectable ordinary profile.
JG2b2b2a may not cite a successful b1 instance as a functor or naturality law,
and it mints no executable authority; JG2b2b2b remains blocked by A4-R4.

JG2b2c has the stronger factual boundary. It must consume the opaque
complete-through-head history and the combined JG2b2b0--JG2b2b3c grammar/
metatheory/naturality authority, derive every occurrence, local context,
substitution, reindexing endpoint, and theorem application internally, and
then retain the concrete successful replay evidence for each positive row.
Caller-minted JG2b2b1 tokens are never a substitution-universe input, an
occurrence list, a naturality oracle, or a shortcut to matrix coverage.

Consequently, even after JG2b2b3a proves generic substitution, the metadata flag
`complete_admissible_substitution_universe_authority` remains false: the
universal result comes from a theorem over formal derivations, not from finite
enumeration. Only the later combined theorem package may set the distinct
metatheory/functor/naturality authorities justified by its proofs.

## Verification status

The resulting `pen-generative-audit` crate passes 83 unit tests and 40
compile-fail doctests, strict all-target clippy, and all 17 current isolation-
checker tests. The
b1 fixtures cover both aggregate replays, raw/normalized separation, dependent
image order, empty codomain, aggregate material exhaustion, all twelve term
forms, all three open-judgment forms, particular identity/composition/lift/
reindex receipts, profile/signature mismatch, and deep-input preflight.

## Ordered continuation

The combined JG2b2b gate now proceeds as follows:

1. **JG2b2b0 -- structural occurrence grammar -- DISCHARGED 2026-08-02.**
2. **JG2b2b1 -- particular open typed substitution/context morphisms --
   DISCHARGED 2026-08-02.** This document's two-aggregate-batch,
   particular-instance authority.
3. **JG2b2b2-P0 -- indexed-interface ontology audit -- DISCHARGED
   2026-08-02.** Separate nine sort-owned structures from seven constructor-
   owned schemas and split target-neutral freeze from executable Rust work.
   See `docs/260802_jg2b2b2_indexed_interface_ontology_audit.md`.
4. **JG2b2b2a -- target-neutral indexed-interface ontology/representation
   freeze -- ORDINARY LANE FROZEN THROUGH A4-R2b; A4-R2c ACTIVE; A2-C OPEN.** A0 is discharged; A1 and the A2-O ordinary
   carrier/action substrate are frozen; A3-O has frozen each ordinary
   constructor's telescope, finite slot grammar, checked realization/
   classifier predicate, principal-root selector, and formal naturality
   subject. The cubical A2-C ontologies remain open. A4-O RC1 was rejected;
   `R2c -> R2d -> R3 -> R2e -> R4` must close the complete selectable ordinary profile transcript/
   resource contract. This
   mints no occurrence-classification or executable authority.
5. **JG2b2b2b -- closed executable Rust grammar and particular fail-closed
   operations -- BLOCKED BY A4-R4.** Implement the exact independently frozen
   representation and checked particular operations after JG2b2b2a.
6. **JG2b2b3a -- generic substitution proof and exact Rust/Agda
   correspondence.** Prove the unbounded twelve-form term-calculus
   substitution, identity, composition, lift, typing, equality, and
   normalization/reindexing results, then prove exact representation agreement.
7. **JG2b2b3b -- nine functor laws.** Prove identity and composition for every
   frozen indexed action.
8. **JG2b2b3c -- seven constructor squares and combined universal authority.**
   Prove all seven generic naturality squares and combine the complete package.
9. **JG2b2c -- factual complete-through-head census.** Derive every particular
   application internally; do not consume caller-minted b1 evidence.

No finite family of JG2b2b1 successes substitutes for JG2b2b3a--JG2b2b3c.
