# Law V2 kernel-cost adjudication V2

**Date:** 2026-07-29

**Status:** `DRAFT_PROPOSED_NOT_ADOPTED`

**Proposed cost profile:** `gf2-kernel-cost-core-v2`

**Companion audit profile:** `gf2-semantic-audit-core-v1`

**Proposed predecessor:** `gf2-kernel-cost-core-v1`

## Authority boundary

This document proposes a successor to
[`LAW_V2_KERNEL_COST_ADJUDICATION_V1.md`](LAW_V2_KERNEL_COST_ADJUDICATION_V1.md).
It supersedes V1 only as the current cost proposal; it does not amend V1,
adopt either profile, or transport any result issued under another profile.
V1 remains an immutable historical proposal and its
[`prototype result`](LAW_V2_SEMANTIC_AUDIT_CORE_V1_PROTOTYPE_RESULT.md)
remains unchanged.

This proposal has no adopted manifest digest and computes no Acts 1--4 cost.
The registered Profile A H3/H4 artifacts and
[`LAW_V2_WINDOW_REGISTER_AUDIT_V1.md`](LAW_V2_WINDOW_REGISTER_AUDIT_V1.md)
remain unchanged. In particular, this adjudication issues no provisional
\(\kappa_i\), \(\nu_i\), ratio, threshold, or Selective-Law result.

## V2 decision

Kernel cost remains the cardinality of a unique, exact,
predecessor-relative basis of first-irreducible public clauses. It is not the
number of declarations, syntax nodes, generated files, demand entries, or
visible equations.

V2 changes one proposed V1 rule:

> A separately sealed public equation owned by a bodyless fresh head is a
> separate paid public clause unless the exact normalized equation is
> predecessor-public or it is a proved duplicate presentation.

Being specified by an obligation is not the same as being freely derivable
from the existing public API. Consequently, none of the following makes such
an equation cost-free:

- compiler generation;
- a pre-existing demand that predicts its shape;
- role or provenance metadata;
- a descriptor naming its constructor;
- the equation being necessary for total demand discharge; or
- the equation being admissible as a Q0 rewrite.

The type of a demanded filler specifies what payment can discharge the
obligation; it does not provide the filler. The same rule applies to a
demanded computation witness. A bodyless head and its separately sealed
computation equation are distinct public commitments.

The fresh-equation rule in V1,
`CertifiedFreshConstructorComputation`, is therefore absent from the V2
cost-free completion manifest. This removes the exact fresh
free-completion/initiality theorem from the V2 cost critical path without
weakening any rewrite-safety obligation.

## Exact public API and basis

Let \(\mathrm{ExactAPI}(H,x)\) contain all of:

- source-level public exports;
- normalized boundary declarations;
- sealed public computation/rewrite clauses; and
- registered public-interface and operation-role descriptors.

Every source export or equation is bound to its normalized view as one raw
clause. Source identity, original syntax, normalized syntax, and the checked
source-to-normal derivation are retained; source and normalized presentations
are not charged separately.

Role and former descriptors are equation-free immutable metadata attached to
their owning declaration clause. Their closed grammar contains only the
frozen descriptor kind, owner/source identifiers, typed local role,
former/constructor references, computation-mode tag, and public-support
identifiers. It cannot contain a term payload, equation payload, equation
identifier or digest, opaque bytes, or caller-defined extension fields. Any
metadata that adds an operation, field, equation, or behavioral choice must
be represented as its own raw public clause.

After Q0/Q2 duplicate and presentation quotienting, \(S\) is an admissible
basis exactly when

\[
  \mathrm{Free}^{\mathcal K_2}
    (\mathrm{Pub}(H),\mathrm{LiveSpec}(H);S)
  \simeq
  \mathrm{ExactAPI}(H,x),
\]

and:

1. **Exactness:** completion reconstructs every and only public generator,
   descriptor, and equation of the act.
2. **Independence:** removing any \(s\in S\) prevents exact reconstruction.
3. **Complete disposition:** every public clause is classified by the
   complete finite V2 calculus.
4. **Complete negative evidence:** first irreducibility follows from
   exhaustion of every applicable V2 free rule or a registered separating
   model, never failed search.
5. **Uniqueness:** every admissible basis agrees up to the adopted
   presentation equivalence.
6. **Order invariance:** equivalent independent declaration orders give the
   same basis.

Then

\[
  \mathsf K^{\mathcal K_2}_H(x)=S,
  \qquad
  \kappa^{\mathcal K_2}_H(x)=|S|.
\]

Multiple inequivalent bases, incomplete inventory or disposition, missing
negative evidence, or failed uniqueness returns `Unknown` and leaves the act
`UndefinedAudit`. No traversal, hash, declaration order, or greedy SCC choice
may select a basis.

## Verified public inventory requirement

Cost classification consumes a verifier-minted public inventory, not an
arbitrary caller-provided clause slice. The required capability binds:

- the exact predecessor history and boundary;
- the exact successor boundary and conservative exact extension;
- complete normalized public group, declaration, and equation coverage;
- declaration and equation origins;
- source-to-normal derivations;
- predecessor-visible typed demand contracts with exact `PortKey`s;
- public availability re-derived from the predecessor boundary;
- the canonical public dependency DAG;
- a positively verified origin-cutoff Q3 registry; and
- exhaustive no-omission and no-duplicate coverage.

Verified fields are private and are minted only by replay from a separate
unchecked wire representation. Serialized labels, digest claims, or
`complete = true` assertions confer no authority. In particular, a
candidate-local descriptor cannot retroactively establish predecessor
availability or supply an omitted public equation.

## Public availability judgment

The verifier distinguishes:

```text
PredecessorPublicExport
DependencyPriorExport
DerivedFromPublicInterface
AmbientOnly
OutsideFragment
Unknown
```

`PredecessorPublicExport` is a direct export at the predecessor boundary.
`DependencyPriorExport` is a direct export from a canonical strict-prior
dependency. `DerivedFromPublicInterface` requires a provenance-preserving
typed derivation whose nonstructural leaves are predecessor or
dependency-prior public exports. Dependency priority is defined by the
canonical acyclic public dependency DAG, never serialized declaration order.

`AmbientOnly` means that the kernel can form or normalize an expression but
the public derivation does not exist. For this profile, bare `Sort`,
`UnitType`, and `Unit` are ambient-only until publicly exported. Their first
public export is charged when complete basis verification establishes first
irreducibility.

A declaration alias is free only when its exact normalized target is
predecessor-public or dependency-prior. An equation replay is a
`TransparentAlias` only when the exact normalized equation is
predecessor-public; an equation already represented dependency-prior in the
same extension is instead decided by the duplicate-presentation rule. Ambient
reduction alone is not evidence of public availability. Structural rules
explicitly named by the manifest may be used without object-level public
exports, but this does not create an ambient-object exemption.

## Raw public clause grammar

V2 retains the V1 raw grammar:

```text
PublicDeclaration {
    head,
    source_identity,
    source_type,
    source_body,
    normalized_type,
    normalized_body,
    source_to_normal_derivation,
    public_group,
    equation_free_descriptor_metadata,
}

PublicEquation {
    equation_id,
    source_identity,
    source_left,
    source_right,
    source_type,
    normalized_left,
    normalized_right,
    normalized_type,
    source_to_normal_derivation,
    owner_head,
}

ForcedProjectionClause {
    projection_id,
    registered_record_descriptor,
}
```

A bodyful public definition is one declaration clause; its ordinary beta
behavior is not a second clause. A bodyless head and every separately sealed
public equation owned by it are separate raw clauses before classification.

Every clause receives exactly one disposition:

```text
FirstIrreducible
TransparentAlias
ForcedDefinitionalCompletion
ForcedProjection
DuplicatePresentation
OutsideFragment
Unknown
```

`OutsideFragment` and `Unknown` fail the complete cost audit; neither is a
zero-cost classification.

## V2 free-completion rules

The complete V2 cost-free completion rule set is:

```rust
enum FreeCompletionRuleV2 {
    OrdinaryBetaOfBodyfulDefinition,
    PriorPublicTransparentAlias,
    DescriptorForcedProjection,
    DuplicatePresentationDeletion,
}
```

The rules mean:

1. **Ordinary beta of a bodyful definition.** The ordinary typed beta
   behavior of a transparent definition follows from its body and is
   `ForcedDefinitionalCompletion`.
2. **Prior-public transparent alias or replay.** A declaration whose exact
   target is proved predecessor-public or dependency-prior is
   `TransparentAlias`. An exact equation replay receives this disposition
   only when it is predecessor-public.
3. **Descriptor-forced projection.** A projection is `ForcedProjection` only
   when a frozen record descriptor uniquely reconstructs its type and
   reduction rule.
4. **Duplicate presentation deletion.** A clause with the same normalized
   public content and public-supported provenance as an already represented
   clause is `DuplicatePresentation`.

There is no `CertifiedFreshConstructorComputation` rule in V2. Arbitrary eta
principles, unregistered theorem search, later history, demand prediction,
candidate-specific compiler conventions, and post-hoc descriptors are not
free-completion rules.

## Bodyless fresh equations

For each separately sealed public equation owned by a bodyless fresh head,
the verifier applies this total decision:

1. if the exact normalized equation is already public at the verified
   predecessor boundary, classify the new presentation as
   `TransparentAlias`;
2. otherwise, if a complete Q2 witness proves that it duplicates an already
   represented normalized public equation, classify it
   `DuplicatePresentation`;
3. otherwise, after complete inventory coverage and exhaustion of all four
   V2 free rules, classify it `FirstIrreducible`; and
4. if any coverage, availability, duplicate, or negative decision is
   incomplete, return `Unknown`.

This decision is clause-local but basis membership remains global: exact
reconstruction, independence, basis uniqueness up to Q2, and order invariance
must still be proved for the complete public API.

The head's type and a demand contract may be necessary inputs to type-check
the equation. They do not reconstruct its behavior. A compiler-produced
equation and a hand-authored equation with the same verified public identity
receive the same cost disposition.

## Rewrite admissibility is separate

Cost-free completion and rewrite admissibility answer different questions:

\[
\begin{array}{ll}
\text{Rewrite admissibility:}
  & \text{May the sealed equation safely compute?}\\[0.3em]
\text{Kernel cost:}
  & \text{Was that public equation already derivable for free?}
\end{array}
\]

For a bodyless fresh head, rewrite admissibility may be proved while the
equation remains `FirstIrreducible`.

After public sealing, a fresh nonrecursive computation equation may enter Q0
only through a separate `VerifiedRewriteSystemV1`-style capability proving:

- typing;
- substitution stability;
- left-linearity and non-recursion;
- absence of critical overlaps;
- termination and confluence; and
- conservativity over predecessor terms.

Those theorems authorize computation. They do not establish a zero-cost
completion, deletion/regeneration initiality, or any cost disposition. A
rewrite-safety failure leaves Q0 and the semantic audit undefined; a
cost-basis failure independently leaves \(\kappa\) undefined.

Semantic-family provenance is likewise independent of cost. A paid
first-irreducible public equation may seed a marginal semantic family and
provides its direct SR2 `Coherence` anchor.

## Dependency graph and basis calculation

The verifier constructs the exact public-clause dependency graph. An edge
\(c\to d\) records that reconstruction of \(c\) uses \(d\). It collapses SCCs
and evaluates the resulting DAG in canonical dependency order.

SCC decomposition is an exhaustive implementation device, not a greedy cost
definition. A mutually dependent SCC may contain multiple irreducible
clauses. Every candidate basis must pass exact reconstruction and
independence, and uniqueness must be proved up to Q2.

For each nonbasis clause, the certificate records its exact reconstruction
derivation and disposition. For each basis clause \(s\), it records complete
non-reconstruction evidence against the predecessor, the full candidate basis
\(S\setminus\{s\}\), and every V2 free-completion rule.

## Generic pre-freeze vectors

Before any Acts 1--4 input is exposed, Rust and safe Agda must agree on:

1. first public export of an ambient primitive;
2. transparent alias to an already public term;
3. bodyful definition whose beta follows by ordinary reduction;
4. bodyless fresh head plus a separately sealed fresh computation equation
   for which neither clause is predecessor-public or a duplicate;
5. an exact bodyless-head equation replay that is predecessor-public;
6. descriptor-forced record projection;
7. duplicate transparent field or equation presentation;
8. two mutually dependent irreducible fields;
9. equivalent independent declaration orders; and
10. a deliberately nonunique irreducible basis.

Expected dispositions are fixed by this document, not by an aggregate target.
In vector 4, complete V2 negative evidence classifies both the bodyless head
and its separately sealed equation `FirstIrreducible`. Vector 5 classifies
the replay `TransparentAlias`; vector 6 is `ForcedProjection`; vector 7 is
`DuplicatePresentation`; and vector 10 returns
`Unknown(NonUniqueBasis)`. Incomplete inventory, availability, or negative
evidence returns `Unknown`, never an inferred first-irreducible clause.

The vectors must also demonstrate that accepting the vector-4 equation into
Q0 after a rewrite-admissibility proof does not change its cost disposition.

## Adoption and freeze gate

Before `gf2-kernel-cost-core-v2` can issue \(\kappa\):

1. independently review and adopt the public/ambient distinction and the V2
   separately paid bodyless-equation rule;
2. encode and digest the exact clause, dependency, reconstruction, negative
   decision, separating-model, and presentation grammars;
3. mint the verified public inventory from exact boundary replay and prove
   complete declaration/equation coverage;
4. implement and prove the complete four-rule V2 disposition procedure;
5. prove exact API reconstruction, clause independence, basis uniqueness up
   to Q2, and order invariance for the finite fragment;
6. prove fresh rewrite admissibility independently of cost;
7. obtain transcript-level Rust/safe-Agda agreement on every generic vector;
   and
8. freeze the cost manifest before any live Profile A audit.

The V1 fresh free-completion theorem is not an adoption gate for V2. No
implementation may replace the removed rule with a renamed compiler,
demand-contract, descriptor, or provenance exception.

Until all V2 gates close, `gf2-kernel-cost-core-v2` remains
`proposed_not_adopted`. Profile A must not be exposed to this profile and no
live act may receive a V2 cost value.

## Source consistency

The proposed manifest-indexed appendix charges irreducible public clauses
once at first irreducibility and permits only enumerated, verified free
completion. V2 resolves the previously open fresh-equation case by treating a
separately sealed equation for a bodyless head as its own public commitment.

This is consistent with predecessor-relative cost: kernel formability,
compiler generation, and demand specificity do not make a new public
behavioral clause available from the prior operational library. It also
preserves anti-packing and treats the demanded term filler and demanded
equation filler symmetrically.

The companion
[`semantic-audit calculus proposal`](LAW_V2_SEMANTIC_AUDIT_CALCULUS_ADJUDICATION_V1.md)
continues to govern whether an already sealed fresh equation may participate
in Q0 and semantic-family extraction. V2 changes only its proposed kernel-cost
classification; it neither adopts nor modifies that semantic profile.
