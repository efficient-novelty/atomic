# Law V2 kernel-cost adjudication V1

**Date:** 2026-07-29

**Status:** `DRAFT_PROPOSED_NOT_ADOPTED`

**Proposed cost profile:** `gf2-kernel-cost-core-v1`

**Companion audit profile:** `gf2-semantic-audit-core-v1`

## Authority boundary

This document proposes the cost semantics that must be frozen independently
of semantic-family extraction. It is not an adopted cost manifest, has no
digest, and computes no Acts 1--4 cost.

The existing Profile A artifacts and the issued
[`LAW_V2_WINDOW_REGISTER_AUDIT_V1.md`](LAW_V2_WINDOW_REGISTER_AUDIT_V1.md)
remain unchanged. Visible declaration counts and compiler output counts are
not cost certificates.

## Decision

Kernel cost is the cardinality of a unique, exact, predecessor-relative basis
of first-irreducible public clauses. It is not the number of declarations,
AST nodes, generated files, or visible equations.

Let \(\mathrm{ExactAPI}(H,x)\) contain all of:

- source-level public exports;
- normalized boundary declarations;
- sealed public computation/rewrite clauses; and
- registered public-interface and operation-role descriptors.

Every source export or equation is bound to its normalized view as one raw
clause: source identity, original syntax, normalized syntax, and the checked
source-to-normal-form derivation are all retained. Source and normal forms are
not charged as two clauses.

V1 role and former descriptors are equation-free immutable metadata attached
to their owning declaration clause, not separately charged clauses. Their
closed field grammar contains only descriptor kind, owner/source identifiers,
typed local role, former/constructor references, computation-mode tag, and
public-support identifiers. It contains no term payload, equation payload,
equation identifier/digest, opaque bytes, or caller-defined extension field.
If metadata itself adds a public operation, field, equation, or behavioral
choice, that addition must appear as a separate raw public clause. This
anti-smuggling boundary is part of the cost manifest.

After Q0/Q2 duplicate and presentation quotienting, a set \(S\) is an
admissible basis exactly when:

\[
  \mathrm{Free}^{\mathcal K}
    (\mathrm{Pub}(H),\mathrm{LiveSpec}(H);S)
  \simeq
  \mathrm{ExactAPI}(H,x),
\]

and all of the following hold:

1. **Exactness:** the free completion reconstructs every and only public
   generator, descriptor, and equation of the act.
2. **Independence:** removing any \(s\in S\) prevents exact reconstruction.
3. **Complete negative evidence:** irreducibility is established by the
   complete finite calculus or a registered separating model, never by failed
   search.
4. **Uniqueness:** every admissible basis is the same up to the adopted
   presentation equivalence.
5. **Order invariance:** equivalent declaration orders yield the same basis.

Then:

\[
  \mathsf K^{\mathcal K}_H(x)=S,
  \qquad
  \kappa^{\mathcal K}_H(x)=|S|.
\]

Multiple inequivalent bases, an incomplete disposition, or a failed negative
decision returns `Unknown` and leaves the act `UndefinedAudit`. Canonical
hash, traversal, or greedy SCC order may not choose a basis.

## Public availability judgment

The cost verifier distinguishes:

```text
PredecessorPublicExport
DerivedFromPublicInterface
AmbientOnly
OutsideFragment
Unknown
```

`PredecessorPublicExport` and `DerivedFromPublicInterface` require
provenance-preserving typed derivations whose nonstructural leaves are
predecessor or earlier-in-extension public exports.

`AmbientOnly` means the kernel can form or normalize the expression but no
such public derivation exists. For this profile, bare `Sort`, `UnitType`, and
`Unit` are ambient-only until publicly exported.

Consequently:

- first public export of an ambient primitive is charged as
  `FirstIrreducible` when it belongs to every admissible basis after the full
  frozen free closure is considered;
- a later transparent alias is free only when its exact target is already
  predecessor-public or dependency-prior in the same extension; and
- kernel normalization must retain provenance, because ambient unfolding
  alone cannot certify a public alias.

“Dependency-prior” is defined by the canonical acyclic public dependency DAG,
not serialized declaration order. Independent declaration exchange therefore
cannot change availability or cost.

Structural binder formation, lambda, application, and other rules explicitly
listed in the cost manifest may be used without first exporting those rules as
object-level public constants. This structural-rule allowance does not create
an ambient-object exemption.

## Raw public clause grammar

V1 recognizes these raw clause kinds:

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

A bodyful public definition is one declaration clause. Its ordinary beta
behavior is not a second charged clause. A bodyless head and a separately
sealed public equation are two raw clauses before free-completion
classification.

Every raw clause must receive exactly one disposition:

```text
FirstIrreducible
TransparentAlias
ForcedDefinitionalCompletion
ForcedProjection
DuplicatePresentation
OutsideFragment
Unknown
```

`OutsideFragment` and `Unknown` are failures of the complete cost audit, not
zero-cost classifications.

## Dependency graph and basis calculation

The verifier constructs the exact public-clause dependency graph. An edge
\(c\to d\) records that reconstruction of \(c\) uses \(d\). It collapses SCCs
and evaluates the resulting DAG in canonical dependency order.

SCC decomposition is an exhaustive implementation method, not a greedy
definition of cost. A mutually dependent SCC may contain several
first-irreducible clauses. Every candidate basis for the SCC must be tested for
exact reconstruction and independence, and uniqueness must be proved up to Q2.

For every nonbasis clause, the certificate records the exact reconstruction
derivation and its disposition. For every basis clause \(s\), it records the
complete non-reconstruction certificate against the predecessor, the full
candidate basis \(S\setminus\{s\}\) (including other members of the same SCC),
and every allowed free-completion rule.

## Exact free-completion rules

The V1 free-completion manifest contains only:

1. **Ordinary definitional completion.** Beta behavior of a bodyful
   transparent definition follows by ordinary typed reduction.
2. **Prior-public transparent alias.** A declaration whose target is certified
   predecessor-public or dependency-prior adds no new irreducible clause.
3. **Descriptor-forced projection.** A projection is free only when a frozen
   record descriptor uniquely reconstructs its type and reduction rule.
4. **Fresh constructor computation completion.** A fresh nonrecursive
   computation equation is free only under the theorem below.
5. **Duplicate presentation deletion.** A duplicate transparent field with
   the same public-supported target is free.

No theorem-search result, arbitrary eta principle, univalence edge, later
history, or candidate-specific compiler convention is a free-completion rule.

## Decision for compiler-generated fresh equations

Compiler generation alone does not make an equation free.

The bodyless head and each computation equation begin as distinct raw public
clauses. A generated equation is classified
`ForcedDefinitionalCompletion` exactly when a generic certificate reconstructs
the full exact rewrite API from:

- the predecessor public API;
- the frozen cost manifest;
- already-public closed-former and constructor information;
- the pre-existing typed computation-demand contract; and
- the bodyless head as the only new seed.

The certificate must prove:

- deterministic reconstruction by a rule frozen before the act is inspected;
- typing and substitution stability;
- termination, nonoverlap, confluence, and conservativity;
- deletion/regeneration round trip for the complete exact API;
- no extra generator or equation;
- initiality, or an equivalent unique-extension universal property; and
- invariance under Q0/Q2/Q3 transport.

The concrete candidate equation may not be smuggled into a post-hoc role
descriptor or used as a premise to prove its own reconstruction.

If an exact realization of those inputs can omit the equation or choose
inequivalent behavior, the equation is separately `FirstIrreducible`. If the
finite theorem procedure decides neither condition, its disposition is
`Unknown`.

This is the V1 normative decision: the classification is theorem-conditional,
not compiler-label-conditional. It deliberately does not decide whether the
already sealed Act-4 equation satisfies the theorem. That is a later blind
audit result, not an input to this adjudication.

Even when an equation is cost-free, its semantic family may remain marginal
and may be SR2-anchored to the exact pre-existing computation-demand output.
Cost and semantic-family provenance are separate judgments.

## Generic pre-freeze vectors

Before any Acts 1--4 input is exposed, Rust and safe Agda must agree on:

1. first public export of an ambient primitive;
2. transparent alias to an already public term;
3. bodyful definition whose beta follows by ordinary reduction;
4. bodyless head plus a generated computation equation with a successful
   exact free-completion theorem;
5. the same raw shape with a complete separating
   non-reconstruction certificate;
6. descriptor-forced record projection;
7. duplicate transparent field;
8. two mutually dependent irreducible fields;
9. equivalent independent declaration orders; and
10. a deliberately nonunique irreducible basis.

Expected dispositions are fixed by this document, not by a desired aggregate
cost. In particular, vector 4 marks the equation
`ForcedDefinitionalCompletion`, vector 5 marks it `FirstIrreducible`, and
vector 10 returns `Unknown(NonUniqueBasis)`. A missing free-completion theorem
without vector 5's complete non-reconstruction certificate returns `Unknown`;
failure to prove completion is never itself proof of irreducibility.

## Adoption gate

Before this cost profile can issue \(\kappa\):

1. independently review and adopt the public/ambient distinction and the
   free-completion criterion;
2. encode and digest the exact clause, dependency, reconstruction, negative
   decision, and separating-model grammars;
3. prove complete disposition and basis uniqueness for the finite fragment;
4. prove the generic fresh-equation free-completion theorem rather than
   recording a Boolean or digest in its place;
5. obtain Rust/safe-Agda agreement on every generic vector; and
6. freeze the cost manifest before any live Profile A audit.

Until those gates close, `gf2-kernel-cost-core-v1` remains
`proposed_not_adopted`.

## Source consistency

The formal appendix charges irreducible public clauses once at first
irreducibility and makes aliases, definitional completion, and forced
projections free. It also states that the repository's current exact-extension
verifier does not prove generic free-sealing initiality. This proposal turns
both statements into an exact finite proof obligation.

The older PEN cost discussion likewise distinguishes derivability relative to
the prior operational library from a new neutral head or operational clause,
and explicitly charges ambient vocabulary when it becomes a local public
commitment. The proposal retains that predecessor-relative distinction while
removing historical score and selection behavior from the cost semantics.
