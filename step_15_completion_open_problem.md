# Step 15 completion: the support-local classification open problem

**Date:** 2026-07-18

**Status:** open. A proof was attempted. The finite combinatorial decomposition is
provable, but the proposed exact semantic bijection is not. As currently stated in
the earlier version of `agda/CertifiedHalt.agda`, it is false for sparse candidates
and is not defined by the repository's present shallow syntax. The Agda boundary
has now been corrected to an injective `AtMost` theorem, and the strengthened
Selective Law now states an anchored four-role provenance axiom. Constructing that
anchored classifier from the intended typed semantics is still unproved.

**Implementation update:** `crates/pen-eval/src/semantic_provenance.rs` checks the
finite provenance bookkeeping conditionally, and `agda/ProvenanceBound.agda`
checks that a valid injective certificate implies `AtMost Marginal (4*kappa)`.
The generated `docs/semantic_provenance_audit.json` replays all fifteen legacy
totals but reports every revised semantic score as unavailable. It does not close
J2/J3 from focus-family labels, and it does not reuse the old Bar16 until the
historical candidate cones and winners have been re-audited.

This document is self-contained. It states the mathematical objects that must be
defined, records the part that can already be proved, gives counterexamples to the
current exact statement, and specifies a proof and machine-checking target that
would actually close the global-halt argument.

## 1. Executive conclusion

The current proposed normal-form code is

\[
  \operatorname{Tag}(H,R)
  := H
     \;\sqcup\;
     (\{\mathsf{in},\mathsf{out}\}\times H\times R)
     \;\sqcup\;
     (H\times H),
\]

where `H` is the set of fresh heads and `R` is the set of old support probes. If
`|H|=k` and `|R|=r`, then

\[
  |\operatorname{Tag}(H,R)|=k+2kr+k^2.
\]

That arithmetic is correct. It is exactly the incidence decomposition of a
complete directed graph on old and new vertices after deleting old/old edges.

What is not proved is that the intended quotient of marginal, depth-two
derivation schemas is that complete graph. There are two independent gaps:

1. **Normalization and uniqueness:** every marginal schema must reduce to one
   and only one tag, and two schemas with the same tag must be definitionally the
   same schema.
2. **Saturation:** every possible tag must actually be realized by a semantic
   schema.

The second assertion is unnecessary for an upper bound and is generally false.
For example, a candidate may introduce two fresh heads without introducing all
four ordered interactions between them. The current Agda record nevertheless
requires a realizer for every one of those four interactions.

The theorem needed for the halt is therefore not an isomorphism with the full
tag code. It is an injection

\[
  \operatorname{Marg}_2(B;X)
  \hookrightarrow
  \operatorname{Tag}(H_X,R_X).
\]

Even this weaker theorem needs a real typed operational semantics, a schema
quotient, a support-compression theorem, and a thinness/uniqueness theorem. None
of those is presently supplied by the AST or by D1--D3.

## 2. The intended setting

Fix a typed base library `B` and an extension telescope `X`.

The following objects are needed before the classification proposition is a
well-formed mathematical statement.

### 2.1 Schema classes

Let

\[
  \operatorname{Sch}_2(T)
\]

be the set of depth-at-most-two **derivation schema classes** over a typed theory
`T`. A schema is intended to be a natural family of derivable judgments, not a
single closed term and not merely an AST pattern. The equivalence relation must
at least identify:

- alpha-equivalent presentations;
- beta/eta and declared computation equality;
- admissible weakening and substitution presentations of the same natural
  family;
- the adjoint mates that D1 says are counted as one schema;
- any definitional completion declared free by D2.

The phrase "depth at most two" also needs a formal filtration. It must say
whether depth counts syntax constructors, dependency edges, primitive
interaction stages, support cardinality, or historical layers. These notions
are not interchangeable.

### 2.2 Weakening and the marginal quotient

There should be a weakening map

\[
  w_X:\operatorname{Sch}_2(B)\longrightarrow
      \operatorname{Sch}_2(B+X).
\]

The extension must be conservative enough that `w_X` is injective. Only then is
the set of genuinely new schema classes unambiguously represented by

\[
  \operatorname{Marg}_2(B;X)
  := \operatorname{Sch}_2(B+X)\setminus\operatorname{im}(w_X).
\]

If conservativity is not available, novelty cannot simply be treated as a set
difference; a quotient or homotopy cofiber must be specified instead.

### 2.3 Fresh heads and old probes

Let `H_X` be the finite set of *effective fresh semantic heads* remaining after
transparent declarations have been erased. Its size is denoted `q`. It is not
automatically the raw telescope length `kappa`: a telescope may mix transparent
aliases with opaque declarations, and one clause may export zero, one, or more
semantic operations.

Let `R_X` be the finite set of *old semantic probes* against which the extension
can have primitive depth-two interactions. Its size is denoted `r`.

This definition is deliberately semantic. A set of syntactic `Lib(n)` leaves is
not sufficient:

- a universe-polymorphic operator can act on old entries without containing a
  literal `Lib(n)` leaf;
- one imported interface may expose many independent eliminators or transports;
- repeated occurrences or distinct argument positions may matter;
- two imported interfaces may share transitive ancestry and hence may not be
  independent probes.

The Step-16 arithmetic needs separate theorems `q <= kappa <= 4` and `r <= 2`.
The latter must bound semantic probes, not merely the number of allowed library
identifiers in the raw expression grammar.

## 3. The exact theorem currently encoded

The Agda module defines a finite code equivalent to

\[
  \operatorname{Tag}(H_X,R_X)
  = H_X
  \sqcup
  (2\times H_X\times R_X)
  \sqcup
  (H_X\times H_X).
\]

It then asks for maps

\[
\begin{aligned}
  \mathsf{classify}&:\operatorname{Marg}_2(B;X)
      \longrightarrow \operatorname{Tag}(H_X,R_X),\\
  \mathsf{realize}&:\operatorname{Tag}(H_X,R_X)
      \longrightarrow \operatorname{Marg}_2(B;X)
\end{aligned}
\]

and both inverse laws. Thus it asks for the exact equivalence

\[
  \operatorname{Marg}_2(B;X)
  \simeq \operatorname{Tag}(H_X,R_X). \tag{Exact}
\]

The three constructors are intended to mean:

1. a unary schema whose distinguished support is one fresh head `h`;
2. an incoming or outgoing interaction between `h` and one old probe `r`;
3. an ordered interaction between two fresh heads `h_1,h_2`.

Old/old interactions are omitted because they should be weakening images from
`B`. Interactions with three or more relevant endpoints are omitted because D2
claims they normalize to composites of depth-two forms.

This is stronger than a normal-form upper-bound theorem. In particular,
`classify(realize(tag)) = tag` asserts that **every** incoming probe, outgoing
probe, and ordered fresh/fresh pair exists and remains distinct.

## 4. The part that can be proved now

There is an unconditional finite graph lemma.

Let `N` be a set of new vertices and `O` a disjoint set of old vertices. Define

\[
 \operatorname{Inc}(N,O)
 := N\;\sqcup\;
    \{(x,y)\in(O\sqcup N)^2\mid x\in N\text{ or }y\in N\}.
\]

Then

\[
 \operatorname{Inc}(N,O)
 \simeq
 N\sqcup(2\times N\times O)\sqcup(N\times N). \tag{Graph}
\]

The classifier is the exhaustive coproduct case split:

- a new vertex `n` goes to the unary summand;
- an edge `(o,n)` goes to `(in,n,o)`;
- an edge `(n,o)` goes to `(out,n,o)`;
- an edge `(n,m)` goes to `(n,m)` in the new/new summand;
- `(o,o')` is excluded by the defining predicate.

The inverse performs the opposite four constructor cases. The inverse laws
follow from no-confusion for the old/new coproduct constructors. Consequently,

\[
 |\operatorname{Inc}(N,O)|=|N|+2|N||O|+|N|^2.
\]

This proves that `supportLocalCode` correctly represents a *complete directed
incidence graph*. It does not identify semantic derivation schemas with that
graph. That identification is precisely the open problem.

## 5. Attempted semantic proof and where it stops

The natural proof attempt follows D1's three clauses.

### Case A: one new generator

Normalize a marginal schema generated by a single new generator, choose its
first fresh head `h`, and map it to the unary tag `h`.

This requires proofs that:

- the generator survives in normal form;
- a marginal schema has a fresh head at all;
- the chosen head is invariant under definitional equality, substitution, and
  permutation of independent telescope fields;
- all formation, introduction, projection, computation, and adjoint-mate
  presentations assigned to `h` are one schema class, or else receive additional
  tags.

None follows merely from saying that the schema was generated by one new
generator.

### Case B: one adjoint mate

D1 says mates are counted as one schema. One would therefore map a mate to the
same unary tag as its generator.

This needs an actual mateship equivalence on schema derivations and a theorem
that each relevant orbit has one distinguished fresh generator. Constructors
with multiple premise-sensitive eliminators, record fields with several
projections, or computation and naturality laws at different sites can otherwise
give several inequivalent schema families with the same fresh head.

### Case C: one depth-two interaction

Normalize the interaction and inspect its relevant endpoints.

- fresh/old and old/fresh endpoints map to the two mixed directions;
- fresh/fresh endpoints map to an ordered pair;
- old/old endpoints should be inherited and hence non-marginal;
- larger support should factor definitionally through binary schemas.

This is the intended classification, but D2 only states an arity policy. It does
not prove a normalization/factorization result, does not show that a composite
has a unique binary presentation, and does not show that one support pair admits
at most one schema class.

### Inverse laws

Even granting the three cases, they establish only that each schema has a tag.
To prove the first inverse law one needs **tag invariance and thinness**: schemas
with the same tag are equal. To prove the other inverse law one needs
**saturation**: every tag is inhabited. Saturation is not a consequence of the
case analysis and is contradicted by sparse candidates.

## 6. Why the exact equivalence is false as stated

### 6.1 A minimal logical countermodel to D1--D3

Take one old probe `o`, one fresh head `h`, and one marginal schema `G_h`. Let
`G_h` be a context-natural generator schema with a genuinely fresh neutral head.
There are no interaction schemas.

- D1 holds: the only schema is generated by one new generator.
- D2 holds: there are no interactions of arity greater than two.
- D3 holds: realizing `G_h` requires the new head.

But the marginal schema set has cardinality `1`, whereas the proposed code has

\[
  1+2\cdot1\cdot1+1^2=4
\]

tags. Therefore D1--D3 do not imply (Exact). This countermodel does not claim to
model an additional maximal-coupling axiom; it proves that such an axiom would
have to be stated and used.

There is a dual underdetermination. D1--D3 do not forbid two inequivalent
computation or naturality schemas with the same oriented pair `(h,o)`. Hence the
prose axioms also do not imply the injective upper bound without an additional
thinness theorem.

### 6.2 Sparse candidates in the corrected calculus

The sidecar calculus already treats the support-local quantities as ceilings,
not as exact schema presentations. At `k=2,r=0`, the full code has six tags, but
representative local calculations are sparse:

- a two-head Foundation package receives only its two kernel heads;
- a formed dimension-one HIT receives two kernel heads and two beta/Kan items;
- a two-modality package receives two heads and one modal pair.

Those counts are respectively `2`, `4`, and `3`, not `6`. Likewise, the class
ceiling function sets the Foundation ceiling to `k`, while the proposed exact
code at `r=0` has `k+k^2` elements. A total realizer satisfying
`classify(realize(tag))=tag` would have to invent interactions that the candidate
does not export.

These examples do not refute use of the six-element code as an upper envelope.
They refute its use as the exact semantic schema set of every candidate.

### 6.3 Implicit and repeated support can undercount

The opposite failure is possible if `R_X` is read as the set of syntactic
`Lib(n)` leaves.

- A universe-polymorphic temporal operator can have no literal library leaf but
  still act on every old type. P6 currently counts one schema per old entry in
  this situation.
- A binary head may use the same imported interface in two different argument
  positions. A set of identifiers collapses the positions.
- A new clause may depend on two earlier fresh heads, so recording only a first
  fresh head loses semantic support.
- A term such as an old operation applied to a new term can remain old-headed
  while being marginal. A fresh-head theorem must explain why it is an instance
  of an inherited schema or give it a tag.
- A nested expression using two old interfaces and one new head requires a real
  fusion theorem before it can be replaced by unary mixed probes.

Thus the proposed code can both overcount nonexistent complete-graph edges and
undercount semantic sites hidden by the shallow support representation.

## 7. Conflicting meanings of "depth two" and "support"

The repository contains several mathematically different counting stories.
They must be reconciled before a proof can choose its domain.

### 7.1 Pairwise D1/D2 story

D1 describes generator, mate, and depth-at-most-two interactions. D2 says
ordered pairs are the largest charged interaction shape and declares longer
composites free. Read literally, these are formation rules or an arity cap. They
do not force one and only one schema for every allowed pair.

### 7.2 Whole-library polymorphism

P6 says a universe-polymorphic operator acts on every sealed library entry and
contributes one schema per entry. This makes support semantic and potentially
whole-library, even when the candidate has no direct `Lib` reference. It is not
compatible with deriving `r <= 2` solely from the fact that the raw Step-16
grammar exposes `L14` and `L15` as leaves.

### 7.3 Combinatorial schema synthesis

The LMCS manuscript's combinatorial synthesis theorem constructs

\[
  E^{(1)}_{i,j}=F_i(A_j),\qquad
  E^{(t+1)}_{i,j_1,\ldots,j_{t+1}}
    =\Sigma_{x:E^{(t)}_{i,j_1,\ldots,j_t}}A_{j_{t+1}},
\]

and claims `Omega(k N^d)` distinct depth-`d` schemas. At depth two this retains
one fresh former and two old atoms. The support-local code has no
fresh/old/old constructor and, with `r <= 2`, is independent of the total
library size `N`. Both claims cannot govern the same schema quotient without a
new collapse theorem or a revision of one of them.

### 7.4 Historical-interface density

The coherence-depth draft defines the depth-two historical interface as the
tagged coproduct of **all exported schemas** from the latest two layers. Its
maximal-density condition assigns one datum per exported generator. Here
"two" counts layers, not two probes. Compressing those entire interfaces to the
two labels `L14` and `L15` is a substantive opacity/density theorem, not a
consequence of coherence depth.

### 7.5 P5 telescopic inheritance

The Telescopic Elimination theorem says every schema of a unique maximal direct
import lifts into a minimal complete API. The number of inherited schemas can be
far larger than one per fresh head or one per imported layer. To use the local
code, one must prove that these lifts are instances of a smaller natural family,
or restrict the Step-16 domain so that historical P5 amplification is unavailable.

These interpretations may support different calculi. They cannot be silently
mixed in a single exact-cardinality proof.

## 8. The corrected theorem that is sufficient for halting

Define a support-local embedding to consist of

\[
  \mathsf{classify}_X:
  \operatorname{Marg}_2(B;X)\to\operatorname{Tag}(H_X,R_X)
\]

together with

\[
  \mathsf{classify}_X(s)=\mathsf{classify}_X(t)
  \Longrightarrow s=t. \tag{Injective}
\]

No global realizer and no saturation law are required. Then finite-set
cardinality gives

\[
  |\operatorname{Marg}_2(B;X)|
  \le q+2qr+q^2. \tag{Bound}
\]

This is the theorem the Agda boundary should expose, for example schematically:

```agda
record SupportLocalEmbedding (Schema : Set) (k r : Nat) : Set where
  field
    classify : Schema -> El (supportLocalCode k r)
    injective : {s t : Schema} -> classify s ≡ classify t -> s ≡ t
```

An `AtMost Schema n` cardinality record should be derived from this embedding.
The existing `Presented Schema n` record denotes exact cardinality and is too
strong for the halt argument.

If an exact normal-form theorem is desired, use a candidate-dependent admissible
subcode

\[
  \operatorname{RealTag}_X
  :=\{\tau\in\operatorname{Tag}(H_X,R_X)mid
       \tau\text{ is semantically realized by }X\}.
\]

The plausible exact statement is then

\[
  \operatorname{Marg}_2(B;X)\simeq\operatorname{RealTag}_X,
\]

plus the inclusion `RealTag_X -> Tag(H_X,R_X)`. This cleanly separates unique
normalization from the false assertion that every possible interaction exists.

### 8.1 Arithmetic consequence at Step 16

Suppose every raw-admitted Step-16 extension satisfies:

1. it is transparent, so its marginal novelty is zero; or it has an injective
   support-local classifier;
2. `q <= kappa` and `kappa <= 4`;
3. `r <= 2` for the **semantic** old-probe set;
4. novelty `nu` is exactly the cardinality of the marginal schema quotient.

Then

\[
\begin{aligned}
  \nu
  &\le q+2qr+q^2\\
  &\le q+4q+q^2\\
  &\le 9q\\
  &\le 9\kappa.
\end{aligned}
\]

The third line uses `q <= 4`. Since

\[
  \operatorname{Bar}_{16}
  =\frac{354333}{39040}
  =9+\frac{2973}{39040}>9,
\]

every such candidate has `rho = nu/kappa <= 9 < Bar_16` and fails. Surjectivity
onto all tags would add no useful fact.

## 9. Precise theorem package still to be proved

The recommended global statement is the following.

### Support-local embedding theorem

For the fixed accepted library `B_15`, every telescope `X` in the complete
raw-admitted Step-16 domain has a checked typed elaboration such that exactly one
of the following holds:

1. **Transparent case.** Every declaration of `X` is definitionally realizable
   over `B_15`, after per-clause decomposition, and
   `Marg_2(B_15;X)` is empty.
2. **Opaque case.** There are finite sets `H_X` and `R_X`, with
   `|H_X| <= kappa(X) <= 4` and `|R_X| <= 2`, and an injective classifier

   \[
     \operatorname{Marg}_2(B_{15};X)
       \hookrightarrow
     H_X\sqcup(2\times H_X\times R_X)\sqcup(H_X\times H_X).
   \]

The dichotomy must work per clause or per strongly connected component; an
extension containing one transparent alias and one opaque declaration must not
fall outside both cases.

### Optional exact theorem

For each opaque `X`, define a checked admissibility predicate `A_X` on tags and
prove

\[
  \operatorname{Marg}_2(B_{15};X)
  \simeq \sum_{\tau:\operatorname{Tag}(H_X,R_X)} A_X(\tau),
\]

where each `A_X(tau)` is proof-irrelevant. Exact equivalence with the *full* tag
set is valid only under the additional saturation theorem

\[
  \forall\tau,\;A_X(\tau).
\]

Saturation should not be assumed for ordinary candidates and is not needed for
the halt.

## 10. Semantic foundations required by the proof

The embedding theorem depends on a kernel with the following explicit data and
metatheorems.

### S1. Typed elaboration

There must be judgments for contexts, types, terms, telescope extensions, and
schema families. Every raw-admitted candidate in the theorem's domain must
elaborate, or raw admission must be replaced by typed admission. Private Boolean
tokens asserting that elaboration occurred are not a construction of these
judgments.

### S2. Operational and definitional equality

Specify reduction rules for all constructors that affect the fifteen-step
library, including eliminators, modalities, paths, coercion/composition if they
are semantically counted, record projections, and temporal operators. Prove
enough confluence and normalization to give equality classes canonical
representatives. If full strong normalization is unavailable, a terminating and
complete normalization procedure for the finite depth-two fragment is enough.

### S3. Natural schema families

Define what parameters a schema ranges over and what naturality means. Distinguish:

- one polymorphic schema from its many library instances;
- a primitive computation rule from a derived term using it;
- an adjoint mate from an independently counted operation;
- an inherited schema from a genuinely marginal lift.

Without this distinction, P6 can count `N` instances while the support-local
argument counts one family, producing incompatible cardinalities.

### S4. Conservative weakening

Prove that old schema classes remain distinct after extension and characterize
the image of weakening. In particular, prove that an old-headed stuck term
depending on new arguments is either an instance of an old natural schema or is
correctly retained as marginal.

### S5. Semantic support

Define support on normalized schema derivations, not only on raw expressions.
Prove invariance under definitional equality and completeness for binders,
polymorphism, implicit parameters, repeated occurrences, and transitive imports.

### S6. Effective fresh-head extraction

After transparent erasure, construct `H_X` and prove that each marginal normal
form has a stable distinguished fresh head. Prove that independent telescope
permutations and beta/eta transformations do not change its tag.

### S7. Depth-two factorization

Prove, rather than stipulate, that every schema with three or more relevant
support endpoints factors through charged unary/binary schemas and contributes
no additional marginal class. Prove coherence: two factorization orders yield
the same schema class.

### S8. Thinness and no-confusion

For a fixed schema kind and tag, prove that the fiber of `classify` is a
subsingleton. Also prove that distinct tags cannot become equal through
definitional equality, adjoint mateship, inherited weakening, or support
renaming. If several schema kinds can share one endpoint pair, either prove they
coincide or add the schema kind/site to the tag code.

### S9. Interface compression

Prove the actual Step-16 bound `|R_X| <= 2`. It must explain why all operations
exported by `L14` and `L15`, their common ancestry, and any implicit universe
action amount to at most two independent primitive probes. If that statement is
false, replace `R_X` with the real probe set and recompute the bound.

### S10. Domain exhaustiveness

Prove that every typed, raw-admitted, semantically minimal Step-16 extension is
covered. A finite expression-signature automaton that is not an exhaustive
telescope quotient cannot supply this premise. Symmetry reduction, clause roles,
dependency order, mixed transparency, and semantic admission gates must all be
included or proved conservative.

## 11. Lemmas for a constructive proof

A successful proof can be organized into the following named lemmas.

1. **Elaboration soundness.** Raw candidates accepted into the theorem domain
   elaborate to well-typed conservative extensions.
2. **Depth-two normalization.** Every schema derivation has a canonical
   depth-two normal derivation representing its equivalence class.
3. **Weakening recognition.** A canonical schema is in the old weakening image
   iff its semantic support and primitive heads are old.
4. **Fresh-head existence.** Every remaining marginal canonical schema contains
   an effective fresh head.
5. **Fresh-head stability.** The selected head is invariant under all quotient
   equalities and allowed telescope reorderings.
6. **Support factorization.** A marginal canonical schema has either one fresh
   endpoint, one fresh and one old endpoint, or two fresh endpoints.
7. **Probe compression.** Every old endpoint maps to `R_X`, with `|R_X| <= 2`,
   without identifying distinguishable schema classes.
8. **Shape classifier.** The preceding data construct a total map to `Tag`.
9. **Tag determinacy.** Two marginal canonical schemas with the same tag are
   definitionally the same natural schema family.
10. **Classifier injectivity.** Shape classification followed by tag
    determinacy gives (Injective).
11. **Finite cardinality.** The graph decomposition and the injection give
    (Bound).
12. **Step-16 arithmetic.** The surface caps imply `nu <= 9*kappa`, strictly
    below the frozen bar.
13. **Exhaustive halt.** The typed candidate-domain theorem applies the bound to
    every possible next step.

Only after Lemmas 1--10 are proved does the existing finite arithmetic become a
semantic theorem. For the optional full equivalence, add:

14. **Tag availability.** Every tag satisfying `A_X` has a semantic realizer.
15. **Saturation, if claimed.** Every tag satisfies `A_X`.

Lemma 15 is expected to fail for sparse candidates.

## 12. Required adversarial examples

Any proposed definition and proof should be tested against at least these cases.

1. Two fresh heads and no old support, with no fresh/fresh interaction.
2. A formed one-dimensional HIT with separate formation, path, beta, and Kan
   presentations.
3. Two modalities with a mateship relation that may be directed or undirected.
4. A P5 record with one imported interface and several inherited exported
   schemas.
5. A universe-polymorphic temporal operator with no literal `Lib` leaf.
6. A repeated import in two argument positions.
7. A clause depending simultaneously on two earlier fresh heads.
8. An old-headed eliminator applied to new data.
9. A nested old/old/new expression whose two factorization orders must agree.
10. A mixed telescope with one transparent and one opaque clause.
11. Direct imports `L14` and `L15`, which share transitive ancestry at `L10`.
12. Beta/eta-equal presentations and permutations of independent fields.
13. Two inequivalent schema kinds at the same oriented support pair.
14. A sparse Foundation candidate, to ensure the proof never assumes saturation.

For every example, the machine artifact should print the normalized schema
classes, their tags, and checked evidence that no two classes collide.

## 13. Recommended machine-checking sequence

The implementation should proceed in this order so that later finite checks do
not hide an earlier semantic assumption.

1. **Define a typed semantic IR.** Include derivation objects and reductions for
   the constructors in scope. Do not infer semantic irreducibility merely from
   expression shape.
2. **Define schemas and the quotient.** Implement the natural-family index and a
   checked equality/normalization procedure for the depth-two fragment.
3. **Implement weakening recognition.** Enumerate or decide the old image and
   construct the marginal set.
4. **Implement semantic support.** Track fresh heads, old probes, argument sites,
   binders, and implicit polymorphic support through normalization.
5. **Produce proof-carrying classifications.** Each marginal normal form should
   emit its tag plus a derivation of the relevant shape lemma.
6. **Check injectivity directly.** Group normalized schemas by tag and reject any
   fiber with two inequivalent members. Do not silently deduplicate collisions.
7. **Prove probe compression.** Export a certificate explaining every map from an
   old semantic observation to one of at most two probes.
8. **Complete the typed candidate domain.** Enumerate typed telescopes or prove
   that a smaller symbolic automaton is a quotient preserving all possible
   schema counts.
9. **Export only the injection to Agda.** Prove `AtMost`, the `9*kappa` envelope,
   and the rational bar comparison independently.
10. **Replay falsifiers.** Mutating a tag, support proof, reduction trace, or
    completeness flag must invalidate the certificate.

The bar value must not be an input to normalization, classification, support
compression, candidate admission, or the derivation of the coefficient `9`.

## 14. Decision required before continuing

One of the following semantic choices must be made explicitly.

### Route A: support-local upper-bound semantics (recommended for the halt)

Treat the complete tag code as an envelope. Prove an injection and semantic
`r <= 2`; do not require all tags to be realized. Reconcile or restrict P6,
whole-interface inheritance, and combinatorial synthesis so that they cannot
produce additional independent marginal schemas at Step 16.

### Route B: maximally coupled exact graph semantics

Add an axiom that every allowable unary and ordered binary incidence has exactly
one primitive schema. Then (Exact) follows from normalization, thinness, and
saturation. In this route `R_X` must be the full active exported interface unless
an independent compression theorem is proved; it cannot simply mean two layer
labels by definition.

### Route C: global amplification semantics

Retain P6's one-per-library-entry action, `Omega(kN^2)` synthesis, and full P5
inheritance as independent schemas. Then the present support-local code and its
`r <= 2` bound are not the appropriate global model. A different halt argument
or a much larger normal-form code is required.

### Route D: candidate-class-specific normal forms

Give Foundation, HIT, Modal, P5, Synthesis, and other classes distinct finite
codes and prove typed classification for each. This may yield the sharper
`4*kappa` sidecar bound, but it moves the main burden to proving that the class
partition is exhaustive and that every class-specific formula is a theorem of
the same semantic quotient.

## 15. Completion criteria

The Step-15 global-halt claim is closed by this route only when all of the
following are present and machine checked:

- a precise typed definition of `Sch_2`, its equality, weakening, and marginal
  quotient;
- a complete typed Step-16 candidate domain;
- constructed effective fresh-head and semantic old-probe sets for every
  candidate;
- independent proofs `q <= kappa <= 4` and `r <= 2`;
- a total classifier for every marginal schema;
- a checked proof that the classifier is injective;
- a proof that schema novelty equals the marginal quotient cardinality;
- the finite cardinality and rational arithmetic proof;
- an end theorem quantifying over every possible Step-16 candidate, not merely
  the candidates observed by the current evaluator;
- a certificate whose trust boundary names the elaboration/reduction kernel and
  whose completeness flags are all true.

If exact equivalence with the full code is still claimed, add a proof that every
tag is realized. Sparse candidates above must then be excluded by a stated
admissibility/maximal-density theorem, not by an implementation convention.

## 16. Consequence for the current halt claim

The current evidence supports three separate statements:

1. the shipped structural evaluator has concrete Step-16 clearing witnesses;
2. the new proof-carrying sidecar calculus is below the bar on its defined finite
   surface;
3. the intended Genesis sequence halts at Step 15 only conditionally.

The graph lemma and Agda arithmetic do not turn statement 2 into statement 3.
The missing bridge should now be stated as the support-local **embedding** theorem
plus semantic support compression and candidate-domain exhaustiveness. The
stronger full-code bijection should be withdrawn unless the intended semantics
is explicitly changed to a maximally coupled, saturated one.

## 17. Repository source map

The relevant existing artifacts are:

- `agda/CertifiedHalt.agda`: the finite code, exact `SupportLocalSemantic`
  isomorphism record, and Step-16 arithmetic;
- `agda/CountingLemmas.agda`: finite-code cardinality infrastructure;
- `crates/pen-eval/src/certified_novelty.rs`: corrected sidecar normalizer,
  certificate boundary, direct-support matrix, and class ceilings;
- `crates/pen-search/src/certified_halt.rs`: serialized distinction between
  shipped SAT, sidecar UNSAT, and conditional global halt;
- `docs/CERTIFIED_HALT_15.md`: current dual-certificate status and the previously
  stated exact-isomorphism obligation;
- `docs/EVALUATOR_DERIVATION.md`: D1--D3, P6, the direct-enumerator proposal, and
  the explicit note that no schema classifier is derived from the AST;
- `docs/LEMMA_L1_D_SQUARED.md` and `docs/LEMMA_L2_R_SQUARED.md`: existing
  semantic staging, transport, disjointness, and mechanization obligations for
  two earlier pair-counting lemmas;
- `tex/pen_lmcs.tex`: combinatorial schema synthesis and the informal external
  linear bound;
- `tex/pen_paper.tex`: Telescopic Elimination and P5 inherited capacity;
- `tex/coherence_depth_fibonacci_lmcs_draft.tex`: historical-interface and
  maximal-density interpretation of coherence depth;
- `crates/pen-type/src/check.rs`: current shallow well-scopedness/type checks;
- `crates/pen-type/src/infer.rs`, `crates/pen-type/src/normalize.rs`, and
  `crates/pen-type/src/equality.rs`: currently empty semantic placeholders;
- `crates/pen-core/src/canonical.rs`: presentation-level AST canonicalization.

## 18. Glossary

- **Schema:** a natural family of derivable judgments, modulo the chosen semantic
  equality.
- **Instance:** a particular specialization of a schema; whether instances count
  separately must be fixed once and used consistently.
- **Marginal:** not in the weakening image of the prior library.
- **Fresh head:** a primitive semantic head introduced by the extension after
  transparent erasure.
- **Old probe:** an independently observable old interface operation relevant to
  a primitive interaction.
- **Support-local:** determined by at most one fresh head alone, one oriented
  fresh/old pair, or one ordered fresh/fresh pair.
- **Thinness:** at most one schema class for a fixed tag.
- **Saturation:** every syntactically possible tag is semantically realized.
- **Exact classification:** equivalence with the realized normal-form subcode;
  equivalence with the full code additionally requires saturation.
- **Embedding classification:** unique tagging of actual schemas, sufficient for
  a cardinality upper bound.
