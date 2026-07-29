# Law V2 semantic-audit lambda/unit adjudication V2

**Date:** 2026-07-29

**Status:** `DRAFT_PROPOSED_NOT_ADOPTED`

**Proposed audit profile:** `gf2-semantic-audit-lambda-unit-v2`

**Preserved predecessor proposal:** `gf2-semantic-audit-lambda-unit-v1`

**Broader predecessor proposal:** `gf2-semantic-audit-core-v1`

**Companion cost proposal:** `gf2-kernel-cost-lambda-unit-v2`

## Authority boundary

This document proposes a semantic successor that corrects the V1 substitution
carrier. It does not edit, reinterpret, supersede, adopt, or freeze the V1
proposal or its prototype result. It changes no broader proposal, issued H3/H4
artifact, Profile Registry entry, Window Audit V1 result, or registered
`UndefinedAudit` outcome.

This proposal has no adopted manifest or digest. Its manifest constructor and
verifier define and recognize only the exact generic candidate wire. Verifying
that wire does not prove any theorem listed below and grants no normalization,
rewrite, quotient, cost, or Profile A authority.

No Profile A history, archived semantic vector, Stage-4 candidate, desired
ratio, or desired continuation result may be consumed while developing this
profile. This document authorizes no Acts 1--4 calculation and issues no live
\(\kappa\), \(\nu\), marginal-family, SR2, ratio, threshold, or Selective-Law
value.

## Decision

The V1 proposal incorrectly treated the closure of finitely many substitution
generators under unrestricted composition as a finite carrier. That closure
need not be finite. If an opaque \(f\) permits

\[
\sigma(x)=f(x),
\]

then the composites can contain all pairwise distinct images
\(f^n(x)\). A resource bound, truncation, bounded search, or quotient by action
on an incomplete test set cannot establish completeness of this monoid.

V2 separates two obligations:

1. a finite census of direct substitution witnesses consumed by the finite
   rank-0/1/2 construction rules; and
2. a generic metatheorem governing every well-typed simultaneous
   substitution.

The binding semantic clause is:

> The raw-carrier substitution census is not the category of all typed
> substitutions. It is the finite inventory of direct substitution witnesses
> consumed by the manifest's finite derivation constructors. Identity,
> composition, lifting, typing preservation, and naturality for arbitrary
> typed substitutions are supplied by a generic metatheorem rather than
> exhaustive enumeration.

There is no global `Composite` construction-witness variant and no fixed-point
closure over compatible substitutions.

## Preserved lambda/unit fragment

V2 retains the V1 projection-free term surface:

```text
Sort(0)
Sort(1)
UnitType
Unit
Variable(index)
Global(id)
Pi(parameter_type, body_type)
Lambda(parameter_type, body)
Apply(function, argument)
```

The public surface may contain verified transparent definitions and aliases,
verified opaque bodyless operation heads, separately sealed public equations,
finite declaration groups, finite dependent contexts, rank-at-most-two raw
family derivations, and fresh rules that meet the exact adopted fresh-rule
grammar.

Records, record descriptors, record fields, projections, projection clauses,
and projection reductions remain outside this profile.
`DescriptorForcedProjection` is absent from its Q0 rule manifest. A projection
term, projection payload, nonempty forced-projection inventory, or claimed
projection reconstruction returns exactly:

```text
OutsideFragment(DescriptorProjection)
```

This is not `Unknown`, a certified absence, or a zero-cost disposition.

## Finite construction-substitution witnesses

Only the following witness kinds are enumerated during raw-carrier
construction:

```text
ContextEmbedding
ForcedNewestArgument
OneHoleInstantiation
```

A context-embedding witness supplies one dependency-respecting leg of a
registered context amalgamation. A forced-newest witness maps
\(\Gamma,A\) to \(\Gamma\), fixes the old variables, and maps the newest
variable to the exact typed family argument selected by the source tuple. A
one-hole witness instantiates one registered typed hole with the exact filler
family selected by that tuple.

For each finite source tuple, the verifier reconstructs the complete
oldest-first image vector required by that derivation and checks its dependent
typing sequentially. If reconstructing this vector internally combines an
embedding with a filler, the resulting vector and its composition derivation
are bound to that raw-family node only. They are not inserted into a global
substitution monoid.

The construction inventory is finite because it is bounded by the finite:

- context-witness inventory;
- context-amalgamation inventory;
- raw-family source-tuple inventory;
- one-hole-context inventory; and
- rank-two source-tuple inventory.

The raw-carrier completeness proof must proceed by derivation rank:

- rank 0 contains every verified semantic seed exactly once;
- rank 1 gives a total disposition to every ordered seed tuple, constructor,
  context witness, one-hole witness, and directly reconstructed substitution;
  and
- rank 2 gives the same complete treatment to every ordered lower-rank tuple.

The theorem must establish:

> Every substitution consumed by a rank-at-most-two family derivation is
> reconstructed directly from that derivation's source tuple and registered
> witness data.

It must not claim that every typed substitution between inventoried contexts
has been enumerated.

## Generic simultaneous-substitution metatheory

An arbitrary typed simultaneous substitution
\(\sigma:\Gamma\Rightarrow\Delta\) consists of one term in \(\Delta\) for each
variable of \(\Gamma\), checked in oldest-first dependent order. Such
substitutions are mathematical and verifier objects, but they are not
exhaustively enumerated.

A pinned safe-Agda theorem package, with matching Rust operations, must prove
the following exact surface:

### Substitution algebra

- identity;
- composition;
- associativity;
- lifting under `Pi`;
- lifting under `Lambda`;
- weakening as a substitution;
- commutation of lifting and composition; and
- capture avoidance under the adopted de Bruijn convention.

### Typing preservation

The substitution lemma must cover sorts, unit type, unit, variables, globals,
dependent products, lambdas, and applications.

### Q0 reduction stability

For every adopted Q0 step and arbitrary well-typed \(\sigma\),

\[
t\longrightarrow u
\quad\Longrightarrow\quad
t[\sigma]\longrightarrow^{*}u[\sigma].
\]

The theorem must cover ordinary beta, provenance-preserving public delta, unit
computation, and every inventoried fresh equation.

### Family-constructor naturality

The theorem must prove structurally:

\[
\begin{aligned}
\mathsf{GenericPublicApplication}(F,G)[\sigma]
&=
\mathsf{GenericPublicApplication}(F[\sigma],G[\sigma]),\\
\mathsf{GenericEquationAction}(E,C)[\sigma]
&=
\mathsf{GenericEquationAction}(E[\sigma],C[\sigma]).
\end{aligned}
\]

Authority must be carried by a private, non-deserializable verifier-minted
capability bound to the exact semantic manifest, term grammar, context
grammar, de Bruijn protocol, substitution protocol, Q0 inventory, family
constructor inventory, Agda source digest, and checker-transcript digest. A
caller-supplied enum tag, Boolean, digest, theorem name, or finite test suite
cannot mint this capability.

Missing checked authority returns:

```text
Unknown(MissingSubstitutionMetatheory)
```

## Complete semantic-seed authority

The rank-0 input must be a complete verifier-minted semantic-seed census. It
must reconstruct, rather than trust, public clause identity, source identity,
public and event support, local role, and any demand-output anchor.

A public-clause census derives clause identities from exact inventory
subjects, origins, normalized public content, group membership, ownership,
and dependency support. Clause identity exists before and independently of
cost classification.

A port association is not a demand anchor. The demand authority must replay
the generic demand judgment, the candidate family judgment, the exact typed
specialization or reindexing, kernel typing after specialization, and
membership in the correct demand orbit.

Missing orbit, realization, or complete seed authority returns respectively:

```text
Unknown(MissingDemandOrbitAuthority)
Unknown(MissingDemandRealizationAuthority)
Unknown(MissingSemanticSeedCensus)
```

## Binder-local typed occurrence census

Every root judgment and every reduct introduced into the finite rewrite
universe receives a recursively constructed occurrence census. Each occurrence
binds:

- its root rewrite node;
- an exact syntax path;
- its binder-extended local context;
- the focused subterm;
- its expected type or type-formation judgment; and
- an exact kernel replay digest.

The traversal includes both terms and the types appearing in judgments. For
`Pi(A,B)` and `Lambda(A,b)`, the parameter is checked in \(\Gamma\) and the
body is visited in \(\Gamma,A\). For `Apply(f,a)`, both children are checked in
\(\Gamma\), including the dependent result type. Variables and globals replay
their exact lookup derivations.

Paths distinguish function, argument, binder parameter, binder body, judgment
type, and equation side. Shifting an outer-context instance is not accepted as
a substitute for reconstructing the binder-local judgment.

An incomplete occurrence tree returns:

```text
Unknown(MissingTypedOccurrenceCensus)
```

## Public-inventory compatibility boundary

The current `VerifiedPublicAuditInventoryV1` capability is minted against the
candidate digest of an exact `VerifiedSemanticAuditManifestV1`. It is not
authority for this V2 profile, even though the lambda/unit kernel term grammar
is intentionally preserved.

The V2 candidate digest is distinct from the V1 candidate digest. No cast,
digest substitution, profile-name comparison, or permissive conversion may
rebind a V1 inventory to V2. Any attempt to consume an existing V1 inventory
without an exact verifier-minted compatibility capability returns:

```text
Unknown(MissingVerifiedPublicInventory)
```

The prototype implements the second of these two lawful transport designs:

1. replay the complete inventory under V2 and mint a
   `VerifiedPublicAuditInventoryV2` bound directly to the exact V2 candidate
   digest; or
2. mint a private compatibility certificate that binds the exact V1 inventory
   digest, its V1 manifest digest, the V2 candidate digest, and the identical
   kernel term, context, typing, de Bruijn, projection-boundary, public-history,
   and Q3 protocols.

`VerifiedPublicInventoryCompatibilityV2` takes the second route. Its verifier
requires the exact lambda/unit V1/V2 profile pair, the inventory-to-V1
manifest binding, recursive lambda/unit syntax, a positively empty projection
and Q3 boundary, equal universe/context/Q0/eta protocols, complete inventory
coverage, exact history and boundaries, exact extension, and the bound
normalizer protocol.

The compatibility theorem transports only this closed set of preserved
kernel-fragment inventory facts. It does not transport the rejected V1 global
substitution closure or erase the V2 seed, substitution-metatheory,
rank-inductive carrier, occurrence, edge-matching, historical-rewrite,
overlap, and conservativity obligations. The compatibility capability alone
grants no rewrite or Q0 authority.

## Edge-local fresh-rule matching

Fresh rewrite edges do not carry an identifier into a global substitution
inventory. For every rewrite node, typed occurrence, and inventoried fresh
rule, the verifier:

1. matches the left-linear rule pattern directly against the focus;
2. reconstructs the unique match substitution, if it exists;
3. checks every image in the occurrence's binder-local context;
4. instantiates both rule sides;
5. checks exact equality of the instantiated left side and the focus; and
6. checks the instantiated right side and result type.

No match and one verified match are total dispositions. Multiple inequivalent
matches return `Unknown`; the verifier never selects one by iteration order.
This edge census is finite because the rewrite-node, typed-position, and
inventoried-rule inventories are finite. It does not enumerate unrelated
substitutions.

## Independent historical and successor subjects

The predecessor and successor audit subjects are constructed independently.
An equation-free predecessor requires a positive empty historical
rewrite-system certificate proving the complete predecessor equation census,
zero public rewrite equations, and the exact base Q0 inventory. A predecessor
with equations requires the previously issued rewrite-system theorem for its
exact history and boundary.

Historical rule authority is never inferred from stored equation syntax, and
the predecessor graph is never reconstructed by filtering the successor graph
for old global identifiers.

Missing historical authority returns:

```text
Unknown(MissingHistoricalRewriteAuthority)
```

## Weakening-image conservativity

Conservativity is stated only on the weakening image of the independently
constructed predecessor subject:

\[
G_{\mathrm{succ}}\!\restriction_{\operatorname{im}(\mathrm{wk})}
=
\mathrm{wk}(G_{\mathrm{pred}}).
\]

For every predecessor node \(t\), immediate outgoing edges and normal forms
must transport exactly:

\[
\operatorname{Out}_{G_{\mathrm{succ}}}(\mathrm{wk}(t))
=
\mathrm{wk}\!\left(\operatorname{Out}_{G_{\mathrm{pred}}}(t)\right),
\]

\[
\operatorname{nf}_{\mathrm{succ}}(\mathrm{wk}(t))
=
\mathrm{wk}\!\left(\operatorname{nf}_{\mathrm{pred}}(t)\right).
\]

A successor-only derivation that happens to mention only old globals but is
not in the weakening image does not belong to this equality. Missing any
required square returns:

```text
Unknown(MissingWeakeningImageConservativity)
```

## Finite rewrite graph and overlap census

The finite graph is built from the complete manifest-generated audit subject,
its binder-local occurrences, and edge-local beta, delta, unit, and fresh-rule
matches. It may mint rewrite authority only after verifying:

1. type preservation for every edge;
2. acyclicity and an explicit decreasing topological rank;
3. exactly one reachable normal form for every node;
4. an exhaustive overlap-pair census; and
5. weakening-image conservativity against the independently built
   predecessor graph.

For every source, every unordered pair of immediate outgoing edges is
classified as same-position, left-nested-in-right, right-nested-in-left, or
disjoint. Both immediate reducts and actual paths to one common normal form
are recorded. Unique normal forms establish joinability of present edges but
do not replace explicit proof that no edge pair was omitted.

Missing pair coverage returns:

```text
Unknown(MissingOverlapCensus)
```

The finite graph proves termination and confluence only for the exact finite
manifest-generated audit subject. The generic substitution metatheory
separately proves that the rule schemas are stable under arbitrary well-typed
substitution.

## Q0, Q1, Q2, and empty Q3

The V2 Q0 grammar contains de Bruijn normalization, sequential substitution,
ordinary beta, provenance-preserving public delta, unit computation,
telescope flattening, and admitted fresh nonrecursive constructor
computations. It contains no descriptor projection rule. An inventoried
equation is candidate reduction data until exact rewrite authority admits it.

After authorized Q0 normalization, Q1 preserves originating-family identity
and Q2 exhaustively decides the fixed presentation-witness inventory. V2
retains exactly:

```text
VerifiedEmptyOriginCutoffRegistry
```

as its Q3 rule. The origin-cutoff Q3 census must therefore be positively
verified empty. Q3 adds no quotient edge. A nonempty registry remains outside
the fragment.

## Exact authority chain and fail-closed boundary

The intended theorem dependency is:

```text
verified public inventory
+ verified public-clause census
+ verified demand-anchor census
+ verified historical rewrite system
+ verified substitution metatheory
        |
        v
complete semantic-seed census
        |
        v
finite pre-Q0 carrier with direct construction substitutions
        |
        v
binder-local typed occurrence universe
        |
        v
edge-local beta/delta/unit/fresh matching
        |
        v
finite reduction graph
        |
        v
termination + unique normal forms
+ explicit overlap joins
+ weakening-image conservativity
        |
        v
future VerifiedRewriteSystemV2
```

Any missing semantic seed, direct construction witness, generic substitution
theorem, typed occurrence, edge-local match, historical rewrite certificate,
edge, overlap pair, termination rank, unique normal form, or conservativity
square returns `Unknown` and grants no Q0 authority. Resource exhaustion does
not authorize truncation.

## Companion kernel-cost semantics

This semantic correction does not change
`gf2-kernel-cost-lambda-unit-v2`. A separately sealed equation for a bodyless
head remains a separately paid public clause unless it is predecessor-public
or proved to be a duplicate presentation. Rewrite admissibility and kernel
cost remain separate judgments. A later cost-verifier revision may bind this
semantic V2 protocol without changing that cost rule.

## Rust--Agda agreement

After the theorem stack exists, independent Rust and safe-Agda
implementations must consume the same canonical generic inputs and compute the
semantic seed census, direct construction witnesses, raw carrier, typed
occurrences, edge-local matches, predecessor and successor graphs,
termination ranks, normal forms, overlap joins, and conservativity result.

Agda may not contain Rust-produced expected answers or caller-supplied theorem
code. Typechecking a fixed reference module alone is not transcript
agreement.

## Adoption and freeze boundary

This adjudication does not claim that the required Agda package, private
substitution capability, semantic-seed census, typed occurrence census,
historical rewrite authority, overlap proof, or rewrite-system V2 capability
already exists.

Only after theorem completion, independent transcript agreement, an explicit
adoption packet, human adoption, and manifest freeze may a separately
isolated Profile A adapter be proposed. This document does not create that
adapter and does not authorize access to Profile A data.

## Preserved results

The V1 semantic proposal and prototype result remain unchanged. So do all
broader proposals, cost adjudications, issued H3/H4 results, the Profile
Registry, and Window Audit V1. This proposal does not assume
\(\kappa_4=2\), \(\nu_4=2\), create a bootstrap density, authorize a
Selective-Law benchmark, or predict whether a future frozen audit will be
defined.
