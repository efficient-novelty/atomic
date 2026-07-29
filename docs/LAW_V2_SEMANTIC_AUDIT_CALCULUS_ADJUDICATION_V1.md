# Law V2 semantic-audit calculus adjudication V1

**Date:** 2026-07-29

**Status:** `DRAFT_PROPOSED_NOT_ADOPTED`

**Proposed audit profile:** `gf2-semantic-audit-core-v1`

## Authority boundary

This document fixes the normative proposal that must be reviewed before the
semantic-audit verifier is frozen. It is not an adopted audit manifest, has no
semantic digest, issues no value, and authorizes no Acts 1--4 calculation.

The completed Profile A experiment, its H3/H4 manifests and artifacts, and
[`LAW_V2_WINDOW_REGISTER_AUDIT_V1.md`](LAW_V2_WINDOW_REGISTER_AUDIT_V1.md)
remain unchanged. In particular, the issued V1 outcome remains
`UndefinedAudit`.

The proposal consumes no archived semantic vector, structural novelty total,
historical Stage-4 candidate, later Genesis act, desired ratio, or desired
continuation result.

## Decision

The first executable semantic register is a finite, manifest-indexed calculus,
not the set of arbitrary well-typed terms of the ambient GF2 language.

For an audit manifest \(\mathcal A\), define

\[
  \mathsf{RawSch}^{\mathcal A}_2(H)
\]

to be the finite set of canonical schema-derivation trees generated from the
verified public inventory of \(H\) by the exact seed and rank-two rule
inventories below. Define

\[
  \mathsf{Sch}^{\mathcal A}_2(H)
  =
  \pi_0\!\left(
    \mathrm{NF}_{\mathcal A}
      (\mathsf{RawSch}^{\mathcal A}_2(H)),
    E_{\mathcal A}
  \right).
\]

Here \(\mathrm{NF}_{\mathcal A}\) is Q0 normalization and
\(E_{\mathcal A}\) is the least equivalence closure of the exhaustively
decided Q1, Q2, and Q3 edges on that fixed normalized vertex set. Q0 equality
and Q1--Q3 connected-component equality are separate decisions.

For an exact conservative extension \(x\),

\[
  \nu^{\mathcal A}_H(x)
  =
  \left|
    \mathsf{Sch}^{\mathcal A}_2(I(H,x))
    \setminus
    \operatorname{im}(\mathrm{wk}^{\mathcal A}_x)
  \right|.
\]

No unindexed \(\mathsf{Sch}_2\) or \(\nu\) value may be issued by this
profile.

## Bound audit identity

Every issued semantic value must bind:

- the semantic-audit manifest digest;
- the separate kernel-cost manifest digest;
- the GF2 kernel protocol digest;
- the GF2 normalizer protocol digest;
- the exact public history and boundary-chain digest; and
- the origin-cutoff Q3 registry digest.

V1 values may enter one ratio or comparison only when their semantic-audit,
cost, kernel, and normalizer protocol identities match exactly. Their
act-local history digests are necessarily different: they must instead form a
certified contiguous boundary chain in which each successor boundary is the
next act's predecessor boundary. Their origin-cutoff Q3 bindings must be
identical empty-registry contracts or be related by an explicit compatible
cutoff/restriction theorem. A later audit profile may transport an older value
only through an explicit conservativity and value-preservation theorem. A
successor may not silently reinterpret an earlier value.

## Public availability is not ambient formability

The kernel's ability to check an expression is an ambient typing fact:

\[
  \mathrm{AmbForm}_K(t:T).
\]

Public availability is a different, provenance-bearing judgment:

\[
  \mathrm{PubAvail}^{\mathcal A}_H(t:T).
\]

It requires a typed derivation from predecessor public exports and the frozen
structural rules. Every nonstructural leaf must have public support. Bare
ambient object primitives are not public support merely because the verifier
implements them.

For `gf2-semantic-audit-core-v1`, `Sort`, `UnitType`, and `Unit` receive no
always-public exemption. They remain available to the checker for typing and
normalization, but their first verified public export is eligible to create a
new public-head family. Normalization must retain source provenance so that
unfolding a transparent global cannot turn a predecessor-supported reference
into an indistinguishable ambient-only term.

A later transparent alias is removable only when its target was already
available through the predecessor boundary or an earlier public export in the
same extension. Ambient formability alone never satisfies that condition.

This decision concerns eligibility and quotienting. It does not by itself
establish that any particular act has a marginal family; that still requires
the complete carrier, weakening, quotient, and SR2 audit.

## Core public inventory

An input history is inside the core only when it supplies a verified, finite,
canonically ordered inventory containing:

- public declaration groups and every declaration they cover;
- paired source identities, original source types/bodies, normalized public
  types/bodies, and checked source-to-normal-form derivations;
- public head and equation identifiers;
- declaration and equation origins;
- closed-inductive former and constructor descriptors;
- bodyless-operation role descriptors;
- exact-extension and conservativity certificates;
- predecessor and successor public-boundary digests; and
- the complete origin-cutoff Q3 registry.

Every public group and equation must receive exactly one supported
disposition. A missing, duplicate, or unregistered disposition returns
`Unknown(MissingTupleDisposition)` or `OutsideFragment`, as applicable; it
never denotes absence.

## Raw family seeds

The V1 seed grammar is:

```text
PublicHead {
    declaration,
    source_identity,
    source_type_and_body,
    generic_context,
    normalized_conclusion,
    source_to_normal_derivation,
    role,
    public_support,
}

PublicEquation {
    equation,
    source_identity,
    source_left_right_and_type,
    generic_context,
    normalized_left,
    normalized_right,
    normalized_type,
    source_to_normal_derivation,
    role,
    public_support,
}

PublicUniversalInterface {
    interface,
    generic_context,
    normalized_conclusion,
    role,
    public_support,
}
```

`PublicHead` and `PublicEquation` are supported in V1.
`PublicUniversalInterface` is a reserved tagged constructor whose V1 source
inventory is empty; encountering one returns `OutsideFragment` rather than
silently treating it as a head. It is reserved for a separately adopted
contextual audit profile.

Each verified seed compiles deterministically to one generic parameterized
judgment. A malformed or untypeable seed fails closed.

## Finite auxiliary carriers

The rule grammar does not infer arbitrary substitutions, unifiers, shared
contexts, or one-hole terms. The manifest supplies four additional finite
carriers:

1. `GenericContext`: seed telescopes, identity amalgamations of equal
   telescopes, and every dependency-respecting disjoint interleaving of at
   most three seed telescopes, represented by explicit order-preserving
   embeddings. Partial sharing other than telescope identity is outside V1.
2. `GenericSubstitution`: the finite lifting substitutions induced by those
   embeddings plus the single argument-for-newest-variable substitution
   forced by an applicable `GenericPublicApplication`. No other term search is
   permitted.
3. `ContextAmalgamationWitness`: one target `GenericContext` plus the exact
   embeddings and substitutions from each source context.
4. `RegisteredOneHoleContext`: a normalized lower-rank public application
   term plus one explicitly indexed and typed syntax position.

Every context merge, component vector, embedding, substitution, and hole
position in these bounded carriers is enumerated. If several witnesses apply,
all resulting derivations enter the raw carrier and are left to Q1--Q3; no
order or hash chooses one. A required witness outside these finite carriers is
`OutsideFragment`, and an undecided typing or compatibility check is
`Unknown`.

## Rank-two derivation grammar

The raw carrier is generated by exactly these constructors:

1. `Seed`: include each verified seed once in canonical source order.
2. `GenericPublicApplication`: from two lower-rank derivations and each
   applicable enumerated `ContextAmalgamationWitness` and
   `GenericSubstitution`, whose normalized conclusions form a typed
   function/argument pair, form their application over the witness's target
   telescope.
3. `GenericEquationAction`: from a lower-rank public-equation derivation and a
   lower-rank `RegisteredOneHoleContext`, together with each applicable
   context and substitution witness, form the corresponding congruence family.

Ranks are:

\[
\operatorname{rank}(\mathsf{Seed})=0,\qquad
\operatorname{rank}(R(D_1,\ldots,D_k))
=1+\max_i\operatorname{rank}(D_i).
\]

Only derivations of rank at most two are admitted. The manifest enumerator
must visit every ordered source tuple for every rule, and must issue one of:

```text
Applicable(canonical_derivation)
CertifiedInapplicable(typed_reason)
OutsideFragment(reason)
Unknown(reason)
```

for every tuple. `Applicable` derivations are deduplicated only after Q0--Q3;
an enumeration order or digest order may not suppress a derivation.
Only `CertifiedInapplicable` excludes a source/witness tuple. One
`OutsideFragment` or `Unknown` disposition aborts the entire carrier and leaves
the value undefined.

No arbitrary term constructor, free composition loop, theorem search, or
unbounded application syntax is part of the carrier. Lambda and application
inside a seed's canonical generic judgment are syntax checked by Q0; they do
not authorize free enumeration of lambda terms.

## Family versus instance

Families are generic objects at birth.

A head with type

\[
  \prod_{\gamma:\Gamma}T(\gamma)
\]

defines one family over \(\Gamma\). Its well-typed specializations are
instances of that same family. Likewise, a generic equation

\[
  \Gamma\vdash \ell=r:T
\]

is one family, and every closed substitution is an instance.

Every derivation first receives an immutable raw-family identity containing:

- the audit-manifest digest;
- the source seed or canonical source tuple;
- the normalized generic parameter telescope;
- the normalized generic conclusion or equation;
- the typed local role;
- exact public support; and
- the canonical substitution action.

Q0--Q3 then assigns a separate quotient-class identity. That identity contains
the manifest, canonical normalized generic judgment, role, substitution
action, and Q2-canonical public support. When Q2 deletes a prior-public
transparent alias, its raw family is transported to the target's existing
class and canonical support; the quotient witness retains the raw source
provenance but does not mint a second class. A first ambient export has no
prior public target and therefore cannot use this transport.

Alpha-renaming, de Bruijn reindexing, and specialization preserve the
quotient-class identity. No clustering of concrete terms is permitted.

## Q0--Q3 decision

### Q0: typed judgmental normalization

Q0 contains only:

- de Bruijn binder normalization;
- left-to-right sequential substitution;
- \(\beta\)-reduction for lambda/application;
- provenance-preserving \(\delta\)-unfolding of verified transparent public
  globals;
- the core `UnitType`/`Unit` rules;
- canonical telescope flattening;
- descriptor-generated forced projection reduction; and
- verified fresh, nonrecursive, nonoverlapping constructor-computation rules.

The V1 eta registry is empty. Zeta and arbitrary iota rules are unsupported
unless they arise from one of the explicitly listed descriptor-generated
rules.

Ordinary normalization must be implemented by hereditary substitution or
normalization by evaluation indexed by the finite core's type rank, with a
reducibility/strong-normalization proof. The fresh-rule layer must separately
decrease the frozen fresh-head dependency rank and the designated constructor
scrutinee measure. The combined proof must establish all beta/projection/delta/
fresh critical pairs, confluence, type preservation, and soundness.
Delta-unfolding carries an origin trace, so a normalized ambient primitive
still records whether it arose from a predecessor-public global or bare
ambient syntax.

### Q1: structural family identity

Q1 identifies only alpha/de Bruijn variants and substitutions carrying the
same originating family identity. Specialization never creates a family.

### Q2: finite public-presentation equivalence

Q2 contains only:

- binder renaming;
- legal exchange of dependency-independent declarations;
- typed currying/uncurrying carrying an explicit isomorphism between two
  `GenericContext` telescopes from the finite auxiliary carrier;
- insertion/deletion of a transparent alias whose target is predecessor-
  public or dependency-prior in the same extension; and
- insertion/deletion of duplicate transparent public fields with the same
  public-supported target.

First public exposure of an ambient primitive is not alias deletion.
Equality after ambient-only unfolding is insufficient.

Q2 is evaluated only on the fixed normalized vertices generated before
quotienting. For every ordered vertex pair and every finite witness code, the
verifier records `Applicable`, `CertifiedInapplicable`, `OutsideFragment`, or
`Unknown`; the latter two abort the audit. No Q2 constructor inserts a new
alias, field, term, or vertex. Every applicable telescope, alias, or duplicate
witness must preserve typing, generic conclusion, substitution action, public
support, and source provenance in both directions.

### Q3: registered equivalence edges

The core accepts only an input carrying a verified-empty origin-cutoff Q3
registry. Empty-registry completeness is a positive generic theorem: no Q3
edge may be consulted or inferred. The later Profile A adapter must prove that
its bound registry satisfies this generic input contract. Paths, univalence,
cubical transport, and contextual adjoints remain outside this profile.

## Total typed role derivation

The caller's `role` field is a claim, not evidence. V1 reconstructs it through
this total table:

```text
Seed(PublicHead)                 -> KernelHead
Seed(PublicEquation)             -> Coherence
Seed(PublicUniversalInterface)   -> OutsideFragment
GenericPublicApplication         -> SupportAction
GenericEquationAction            -> Coherence
AdjointMate                      -> uninhabited in V1
```

Every role certificate binds the constructor, typed sources, and generic
conclusion. A claimed mismatch aborts the audit. Demand-output anchoring is a
separate SR2 alternative and does not change this local-role derivation.

## Exact supported fragment

The proposed core supports exactly:

- the explicit finite universe-level set \(\{0,1\}\);
- `Sort`, `UnitType`, and `Unit`;
- dependent \(\Pi\);
- variables, globals, lambda, and application;
- verified public declaration groups;
- closed-inductive former and constructor descriptors;
- bodyless public operation heads;
- fresh nonrecursive constructor-computation equations;
- descriptor-generated forced projections;
- conservative exact extensions;
- the Q0, Q1, and finite Q2 grammars above; and
- any verified-empty origin-cutoff Q3 registry satisfying the generic
  contract.

It returns `OutsideFragment` for:

- paths and path constructors;
- univalence;
- nontrivial cubical transport;
- contextual adjoints or universal interfaces;
- recursive or overlapping rewrite systems;
- arbitrary Sigma/record syntax not represented by the forced-projection
  descriptor;
- an unregistered family constructor; and
- arbitrary theorem search.

## Required completeness theorems

The verifier may be frozen only after it proves both layers for this finite
manifest.

First, for Q0 judgmental equivalence \(\equiv^0_{\mathcal A,H}\):

\[
  \mathrm{nf}_{\mathcal A}(D)
  =
  \mathrm{nf}_{\mathcal A}(E)
  \quad\Longleftrightarrow\quad
  D\equiv^0_{\mathcal A,H}E.
\]

Second, let \(E_{\mathcal A,H}\) be the least equivalence relation generated
by every exhaustively decided Q1, Q2, and Q3 edge on the fixed Q0-normalized
vertex set. Then:

\[
  \operatorname{component}_{\mathcal A,H}(D)
  =
  \operatorname{component}_{\mathcal A,H}(E)
  \quad\Longleftrightarrow\quad
  D\;E_{\mathcal A,H}\;E.
\]

The proof package must also establish structural finiteness, normalization
termination, typing preservation, soundness, exhaustive auxiliary-carrier and
source-tuple disposition, finite graph-component completeness, and
family-versus-instance completeness.

This is a theorem only about the frozen finite carrier. It makes no claim
about arbitrary GF2 syntax or arbitrary univalent equivalence.

## Weakening, restriction, and marginals

Every admitted conservative exact extension must induce structural weakening

\[
  \mathrm{wk}^{\mathcal A}_x:
  \mathsf{Sch}^{\mathcal A}_2(H)
  \longrightarrow
  \mathsf{Sch}^{\mathcal A}_2(I(H,x)).
\]

The implementation must prove typing preservation, normalization commutation,
Q1/Q2/Q3 transport, family-identity preservation, and public-support
preservation. It must also construct a restriction map on old-support
families satisfying

\[
  \mathrm{res}_x\circ\mathrm{wk}_x=\mathrm{id}.
\]

The exact set difference is the complete definition of marginality. For every
class outside the weakening image, the verifier must additionally prove that
its canonical support touches the new event or a newly discharged
pre-existing output. Failure of that theorem reveals an incomplete
weakening/restriction/support audit and makes the whole act `UndefinedAudit`;
it never filters the class out of the marginal set. Digest inequality is not
a marginality proof.

## SR2 output

Every marginal family must receive exactly one typed, injective provenance
tag:

\[
  (c,r)
  \quad\text{or}\quad
  (o,p),
\]

where \(c\) is an exact first-irreducible clause, \(r\) is one of the frozen
four local roles, and \((o,p)\) is an exact pre-existing live demand output.
Role assignment comes from the derivation constructor and typed source, not a
string.

For each marginal family, the verifier first computes its complete
candidate-local dependency support. It then enumerates every typed eligible
tag:

1. if the support contains new first-irreducible clauses, only dependency-
   minimal such clauses are eligible; the role comes from the verified family
   constructor;
2. when several minimal clauses remain, a tag is eligible only with a
   presentation-invariant typed principal-source proof showing that one clause
   is the family's source and the others are premises;
3. only when there is no eligible new irreducible clause may a demand tag be
   used, and then the proof must show that the family realizes that exact
   pre-existing \((o,p)\) output; and
4. the eligible tags must contain exactly one equivalence class. No traversal,
   declaration order, or hash may choose among ambiguous tags.

No eligible tag or several inequivalent eligible tags returns
`UndefinedAudit(AmbiguousOrMissingProvenance)`. After total assignment, the
verifier exhaustively checks injection and presentation transport.

A cost-free computation family may therefore use its exact pre-existing
computation-demand output as the anchor. Two families receiving one tag is
`UndefinedAudit(ProvenanceCollision)`, not permission to rename families or
expand the role set.

## Generic pre-freeze vectors

Before any registered act is exposed, Rust and safe Agda must agree on:

1. first public export of an ambient primitive;
2. transparent alias to an already public term;
3. a bodyful definition with ordinary beta;
4. a bodyless head plus a fresh nonrecursive computation equation;
5. a descriptor-forced projection;
6. a duplicate transparent field;
7. two specializations retaining one originating family identity;
8. equivalent binder and supported telescope presentations;
9. conservative weakening and its restriction retraction;
10. one first-irreducible clause producing two distinct typed roles;
11. a deliberate SR2 provenance collision;
12. a verified-empty Q3 registry; and
13. unsupported path and univalence inputs.

The suite must exercise successful and fail-closed paths. In particular, the
collision is a certified failure, the empty registry succeeds without an
edge, and path/univalence return `OutsideFragment`.

## Adoption and freeze gate

Before this profile can issue a live value:

1. independently review and adopt this semantic decision;
2. encode the complete finite manifest;
3. prove the carrier, Q0--Q3, weakening/restriction, marginal, and SR2
   obligations;
4. obtain matching Rust and safe-Agda results on the prescribed generic,
   non-Genesis vectors;
5. freeze the semantic-audit, cost, kernel, normalizer, history-binding, Q3,
   verifier, and reference-agreement digests; and
6. only then expose Acts 1--4 to the frozen verifier.

Until those gates close, `gf2-semantic-audit-core-v1` remains
`proposed_not_adopted`.

## Source consistency

This proposal makes executable the finite-fragment milestone and semantic
register in
[`app_a_two_laws_formal_axioms.tex`](app_a_two_laws_formal_axioms.tex),
especially its finite boundary, schema-family/instance distinction,
weakening-based marginal definition, and SR1--SR3 requirements. It narrows the
executable carrier; it does not weaken the ambient mathematical semantics.
