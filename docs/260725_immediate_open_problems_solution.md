# Resolution of the Immediate Open Problems: Free Environment Completion

**Date:** 2026-07-25  
**Context:** `260725_immediate_open_problems.md` and `free_environment_completion_attempt_v1.md`  
**Verdict:** Problem A has a rigorous **negative** solution (independence/countermodel). Problem B has a rigorous **positive** solution after one necessary normalization is made explicit in the definition of an exact realization.

---

## 1. Executive result

| problem | resolution | consequence |
|---|---|---|
| **A: derive Q1(b)** | **Not derivable from the presently stated Internal Record Actualization, Public Confluence, Q1(a), and free sealing.** An explicit non-vacuous countermodel satisfies those clauses and violates Q1(b), even while being invariant under every unitary recoding. | Q1(b) must remain a declared bridge/API equation, or the record axioms must be strengthened by an explicit record-extensionality link to discarding. |
| **B: initiality of `P_⌐`** | **Proved.** More generally, the construction is initial for any chosen wide symmetric-monoidal class `J` of environment embeddings. | Once Q1(b) (or the corresponding `J`-causality API) is selected, free sealing produces the environment completion without further bridge content. |

Thus the realization half of target 3 is closed, but the API-selection half is not. The exact status is:

> **Q1(b) is independent bridge content; conditional on Q1(b), the environment completion is the free seal.**

---

# Part A — Independence of ledger-invariance

## 2. Statement to be tested

Let `P` be the pure dagger symmetric monoidal layer, let

\[
\varepsilon_A:A\longrightarrow I
\]

be the monoidal discarding family supplied by Q1(a), and let
`u:E\to E'` be an isometry, `u^\dagger u=\mathrm{id}_E`. Problem A asks for

\[
\varepsilon_{E'}\,u=\varepsilon_E. \tag{Q1b}
\]

The current record clauses do not mention this equation. Internal Record Actualization constrains the existence, decoding, persistence, and comparison of record states. Public Confluence constrains compatible pairs of stable records by requiring a common nondemolition refinement. Neither clause defines the public denotation of a pure morphism as a discard composite, and neither says that every isometry in `P` is an admissible change of record code.

That gap is not merely expositional. It has a countermodel.

## 3. Theorem A — Q1(b) is independent

### Theorem

Consider the two-sorted theory consisting of:

1. a dagger symmetric monoidal category `P`;
2. a monoidal family `ε_A:A→I` satisfying Q1(a);
3. an Internal Record Actualization layer;
4. a Public Confluence layer; and
5. free sealing, understood as imposing only the equations selected by the API.

Without an additional axiom connecting the record layer to the composites
`(id_B⊗ε_E)f`, this theory does **not** entail Q1(b).

### Proof by countermodel

#### 3.1 The pure categorical layer

Let

\[
P=\operatorname{Mat}_{\mathbb R_{\ge 0}}.
\]

Its objects are natural numbers. A morphism `n→m` is an `m×n` matrix with nonnegative real entries. Composition is matrix multiplication, tensor product is the Kronecker product, the tensor unit is `1`, and the dagger is transpose.

For each `n`, define

\[
\varepsilon_n=(1,1,\ldots,1):n\longrightarrow 1,
\]

with the unique map for `n=0`. Then

\[
\varepsilon_1=\mathrm{id}_1,
\qquad
\varepsilon_{mn}=\varepsilon_m\otimes\varepsilon_n.
\]

Hence `ε` satisfies Q1(a).

This family is also invariant under every unitary of `P`. A square nonnegative matrix `v` satisfying `v^\dagger v=vv^\dagger=I` is a permutation matrix, and therefore

\[
\varepsilon_n v=\varepsilon_n.
\]

So the countermodel does not exploit the weaker “unitaries versus isometries” near-miss.

Now take

\[
u=\frac1{\sqrt2}\begin{pmatrix}1\\1\end{pmatrix}:1\longrightarrow2.
\]

Then

\[
u^\dagger u=1,
\]

so `u` is an isometry, but

\[
\varepsilon_2u
=(1,1)\frac1{\sqrt2}\begin{pmatrix}1\\1\end{pmatrix}
=\sqrt2
\ne1
=\varepsilon_1.
\]

Thus Q1(b) fails.

#### 3.2 A non-vacuous public-record layer

Let a public record be a finite partial function

\[
r:S\rightharpoonup\{0,1\}
\]

on a countable set `S` of public addresses.

* A record is copied by copying the partial function.
* It is future-stable because later histories extend it without changing already assigned values.
* Two records are operationally compatible exactly when they agree on the intersection of their domains.
* Their joint nondemolition refinement is their union `r∪s`.
* The comparison maps are restrictions to the two original domains.

The union is unique, preserves both inputs literally, and supplies a non-vacuous model of Public Confluence.

For Internal Record Actualization, an event at a fresh address `a` with label `b` is recorded by extending `r` to `r∪{a↦b}`. The decoder evaluates at `a`; deterministic internal dynamics gives the recorded alternative weight one; the decoding tolerance is zero; lock-in is monotone extension; and inter-observer comparisons are restrictions or address-preserving relabellings. Thus the four record requirements are satisfied.

Each finite stage of this record system can be internalized in `P`: take the possible partial records as a distinguished basis, use the basis-copying isometry `e_r↦e_r⊗e_r`, and represent decoding, restriction, and compatible union by nonnegative matrices. Thus the countermodel need not place the record layer outside the pure category. Nothing in the construction refers to `ε`, so the record axioms remain true while Q1(b) fails.

#### 3.3 Free sealing does not repair the failure

Free sealing contains no unforced equations. Since the record signature contains no equation identifying

\[
\varepsilon_{E'}u
\quad\text{with}\quad
\varepsilon_E,
\]

and the countermodel distinguishes them, soundness forbids a derivation of that equality. Adding it during completion would be precisely an unselected equation, contrary to free sealing.

Conceptually, Q1(b) is the compatibility law `Isom(P)⊆Causal(P,ε)`: it identifies dagger-losslessness with ledger-losslessness. The countermodel separates these notions—`u` is an `ℓ²`-isometry but is not causal for the `ℓ¹` discard. This proves the independence claim at the abstraction level of the open problem. It rules out a general implication from Q1(a), Internal Record Actualization, Public Confluence, and free sealing. A later, more tightly specified Genesis category could still satisfy Q1(b) by a genuinely new theorem using additional structure of that particular `P`; no such structure-to-discard theorem is presently stated. ∎

## 4. Diagnosis of the three proposed routes

### Route 1: two observers and Public Confluence

This route stops at A-1 and A-2.

* “Operationally compatible” is a relation on record states, but the current axioms do not declare two environment presentations related by an arbitrary pure isometry to be compatible record presentations.
* More importantly, no axiom says that inequality of discard composites is inequality of public records. The countermodel makes the public record layer ignore the chosen `ε` entirely.

Therefore a difference

\[
\varepsilon_{E'}u\ne\varepsilon_E
\]

does not presently produce contradictory public facts, so Public Confluence is never triggered.

### Route 2: decoder transport along `u^†`

From `u^†u=id_E`, a decoder on `E` can indeed be transported to the image of `u` in `E'`. This proves preservation of **recoverability of encoded content**. It does not prove equality of two categorical effects. The missing implication is:

> same recoverable content ⇒ same public denotation ⇒ same discard composite.

Neither implication is currently an axiom.

### Route 3: no-unforced-generator argument

The direction reverses. Free sealing says not to add equations that the selected API does not force. Once Q1(a) has selected only a monoidal family of discards, Q1(b) is an additional equation. The countermodel proves that monoidality does not force it. Therefore free sealing preserves the distinction rather than erasing it.

## 5. Exact minimal repair

A positive proof becomes immediate after adding an explicit **record-code extensionality bridge** with two clauses:

1. **Discard adequacy.** The public denotation of a dilation `f:A→B⊗E` is represented by
   \[
   \operatorname{Pub}_E(f)=(\mathrm{id}_B\otimes\varepsilon_E)f.
   \]
2. **Lossless recoding invariance.** If `u:E→E'` is an admissible lossless record encoder with decoder `u^†`, then
   \[
   \operatorname{Pub}_{E'}((\mathrm{id}_B\otimes u)f)
   =\operatorname{Pub}_E(f).
   \]

Taking `A=E`, `B=I`, and `f=id_E` gives

\[
\varepsilon_{E'}u=\varepsilon_E.
\]

This is a valid proof, but it is not a derivation from the current clauses: the two displayed links are exactly the missing bridge. To avoid hiding Q1(b) in prose, the honest registration is either Q1(b) itself or the more semantic record-code extensionality package above.

## 6. Resolution of Problem A

Problem A is therefore solved in the negative:

> **Internal Record Actualization plus Public Confluence do not presently derive ledger-invariance under isometric environment enlargement.**

This is the negative outcome already contemplated by A.7. Q1(b) remains counted bridge content, and target 3 cannot yet be reclassified as wholly derived.

---

# Part B — Initiality of the isometric environment completion

## 7. One necessary correction to the statement

The formula in Problem B suppresses all monoidal coherence maps. It is literally correct for strict symmetric monoidal categories and strict symmetric monoidal realization functors.

For general **strong** symmetric monoidal functors, the definition of an exact realization must additionally require the functor's tensorator and unit map to be causal. If

\[
\phi_{A,B}:F(A)\otimes F(B)\xrightarrow{\cong}F(A\otimes B),
\qquad
\phi_0:I_M\xrightarrow{\cong}F(I_P),
\]

then require

\[
\varepsilon^M_{F(A\otimes B)}\phi_{A,B}
=\varepsilon^M_{F(A)}\otimes\varepsilon^M_{F(B)},
\qquad
\varepsilon^M_{F(I_P)}\phi_0=\mathrm{id}_{I_M}. \tag{N}
\]

Equivalently, work in the strict/normal lane obtained by monoidal coherence. Without (N), the statement as written is false; a counterexample is given in Section 12 below.

## 8. General construction

It is useful to separate the dagger from the universal property.

Let `P` be an essentially small strict symmetric monoidal category and let `J⊆P` be a wide symmetric monoidal subcategory. Think of `J` as the selected class of lossless environment embeddings. In the intended application,

\[
J=\operatorname{Isom}(P)
=\{u\mid u^\dagger u=\mathrm{id}\}.
\]

Define `Env_J(P)` as follows.

* Objects are the objects of `P`.
* A representative of a morphism `A→B` is a pair
  \[
  (E,f),\qquad f:A\longrightarrow B\otimes E.
  \]
* Generate an equivalence relation by
  \[
  (E,f)\sim(E',(\mathrm{id}_B\otimes u)f)
  \qquad(u:E\to E'\text{ in }J).
  \]
* Composition is
  \[
  [D,g]\,[E,f]
  :=[D\otimes E,(g\otimes\mathrm{id}_E)f].
  \]
* Tensor product is induced from that of `P`, using the symmetry to move the two environment factors to the right.

Write

\[
\eta:P\longrightarrow\operatorname{Env}_J(P),
\qquad
\eta(h)=[I,h],
\]

and define discard by

\[
\varepsilon^{\operatorname{Env}}_A=[A,\mathrm{id}_A]:A\longrightarrow I.
\]

Because `J` contains identities and is closed under composition and tensor, the generated relation is a monoidal congruence. No filteredness assumption is required.

## 9. Exact realizations

For a strict symmetric monoidal category `M` with monoidal discard `ε^M`, call a morphism `h:X→Y` **causal** when

\[
\varepsilon^M_Yh=\varepsilon^M_X.
\]

An exact `J`-realization of `P` is a strict symmetric monoidal functor

\[
F:P\longrightarrow M
\]

such that `F(u)` is causal for every `u` in `J`.

A comparison from `Env_J(P)` to `(M,F,ε^M)` is a discard-preserving strict symmetric monoidal functor `G` satisfying `Gη=F`.

## 10. Theorem B — universal property

### Theorem

For every exact `J`-realization `(M,F,ε^M)`, there exists a unique discard-preserving strict symmetric monoidal functor

\[
G:\operatorname{Env}_J(P)\longrightarrow M
\]

such that `Gη=F`. Hence

\[
(\operatorname{Env}_J(P),\eta,\varepsilon^{\operatorname{Env}})
\]

is initial among exact `J`-realizations.

For `J=Isom(P)`, this is Problem B.

### Proof

Define `G(A)=F(A)` on objects and

\[
G[E,f]
:=(\mathrm{id}_{F(B)}\otimes\varepsilon^M_{F(E)})F(f). \tag{1}
\]

#### 10.1 Well-definedness

Suppose

\[
f'=(\mathrm{id}_B\otimes u)f,
\qquad u:E\to E'\text{ in }J.
\]

Then

\[
\begin{aligned}
G[E',f']
&=(\mathrm{id}_{F(B)}\otimes\varepsilon^M_{F(E')})
  (\mathrm{id}_{F(B)}\otimes F(u))F(f)\\
&=(\mathrm{id}_{F(B)}\otimes
   (\varepsilon^M_{F(E')}F(u)))F(f)\\
&=(\mathrm{id}_{F(B)}\otimes\varepsilon^M_{F(E)})F(f)\\
&=G[E,f],
\end{aligned}
\]

because `F(u)` is causal. Hence (1) is constant on every generating relation and therefore on its equivalence closure.

#### 10.2 Identities

The identity of `A` is `[I,id_A]`. Since `F(I)=I` and `ε^M_I=id_I`,

\[
G[I,\mathrm{id}_A]=\mathrm{id}_{F(A)}.
\]

#### 10.3 Composition

Let

\[
f:A\to B\otimes E,
\qquad
g:B\to C\otimes D.
\]

Using strict monoidality of `F` and monoidality of discard,

\[
\begin{aligned}
G([D,g]\,[E,f])
&=(\mathrm{id}_{F(C)}\otimes
   \varepsilon^M_{F(D)}\otimes\varepsilon^M_{F(E)})
  (F(g)\otimes\mathrm{id}_{F(E)})F(f)\\
&=(\mathrm{id}_{F(C)}\otimes\varepsilon^M_{F(D)})F(g)
  (\mathrm{id}_{F(B)}\otimes\varepsilon^M_{F(E)})F(f)\\
&=G[D,g]G[E,f].
\end{aligned}
\]

#### 10.4 Tensor product

For representatives `(E,f)` and `(D,g)`, the tensor in `Env_J(P)` merely swaps the environment factors past the public output factors. Symmetry and

\[
\varepsilon^M_{F(E\otimes D)}
=\varepsilon^M_{F(E)}\otimes\varepsilon^M_{F(D)}
\]

give

\[
G([E,f]\otimes[D,g])=G[E,f]\otimes G[D,g].
\]

Thus `G` is strict symmetric monoidal.

#### 10.5 Extension of `F`

For `h:A→B`,

\[
G\eta(h)
=G[I,h]
=(\mathrm{id}_{F(B)}\otimes\varepsilon^M_I)F(h)
=F(h).
\]

#### 10.6 Preservation of discard

By definition,

\[
G(\varepsilon^{\operatorname{Env}}_A)
=G[A,\mathrm{id}_A]
=\varepsilon^M_{F(A)}.
\]

#### 10.7 Uniqueness

Every morphism has the canonical factorization

\[
[E,f]
=(\mathrm{id}_B\otimes\varepsilon^{\operatorname{Env}}_E)\,\eta(f). \tag{2}
\]

Let `H` be any discard-preserving strict symmetric monoidal functor with `Hη=F`. Applying `H` to (2) gives

\[
H[E,f]
=(\mathrm{id}_{F(B)}\otimes\varepsilon^M_{F(E)})F(f)
=G[E,f].
\]

Hence `H=G`. ∎

## 11. The initial object itself is an exact realization

For `u:E→E'` in `J`,

\[
\varepsilon^{\operatorname{Env}}_{E'}\eta(u)
=[E',u]
=[E,\mathrm{id}_E]
=\varepsilon^{\operatorname{Env}}_E,
\]

where the middle equality is exactly one generating relation. Thus `η(u)` is causal, and the constructed object belongs to the realization category whose initiality was proved.

## 12. Why normalization (N) is necessary for strong monoidal functors

Let `P` be the terminal strict symmetric monoidal category. Let `M` be the one-object strict symmetric monoidal category whose scalar group is the positive reals under multiplication, with discard scalar `1`.

Choose a strong symmetric monoidal functor `F:P→M` whose tensorator is a scalar `a≠1` and whose unit map is `a^{-1}`. The monoidal coherence equations hold because their product is `1`. The only morphism of `P` is the identity, so `F` sends every isometry to a causal map.

But `Env(P)=P`, whose discard is its identity. A discard-preserving comparison extending `F` would force the unit map of `F` to be causal, hence `a^{-1}=1`, a contradiction.

Therefore the literal condition “`F` sends isometries to causal maps” is insufficient in the non-strict strong-monoidal setting. Requiring (N), or working with strict/normal realization functors, is necessary.

## 13. Minimal hypothesis audit

| proposed hypothesis | needed for the theorem? | exact role |
|---|---:|---|
| Essentially small `P` (or a fixed universe) | **Yes** | Makes each quotient hom-class a set. |
| Symmetric monoidal structure | **Yes** | Defines composition and moves environment factors to the right in tensor products. |
| Wide monoidal embedding class `J` | **Yes** | Ensures the quotient relation is a monoidal congruence. |
| Dagger | **Only for `J=Isom(P)`** | The universal theorem needs `J`, not a dagger. The dagger supplies the intended choice `u†u=id`. |
| Filtered environment diagram | **No** | A quotient by the generated equivalence relation, or an ordinary small colimit in `Set`, exists without filteredness. Filteredness is needed only to replace zigzags by a single common enlargement. |
| Finite coproducts/distributivity | **No** | Not needed for the environment quotient or its initiality. They are needed for stronger completions that also freely add classical direct-sum structure. |
| Zero object | **No** | Irrelevant to the universal proof. |
| No nonzero isometry into zero | **Not a theorem hypothesis** | It blocks the particular zero-object collapse argument but does not by itself guarantee global nontriviality. |
| Causal/normal monoidal coherence of a strong `F` | **Yes outside the strict lane** | Required for a discard-preserving monoidal extension to exist. |

## 14. Structural corrections to the parent note

### 14.1 The dagger is not “destroyed” as an absolute theorem

The construction does not, in general, induce a canonical dagger on `Env_J(P)`. That is the precise statement. A special completed category might admit some dagger for unrelated reasons, so “`P_⌐` is not a dagger category” is stronger than the construction proves.

### 14.2 The environmental equivalence is a generated zigzag

For general `P`, equality of dilation classes means a zigzag generated by `J`-embeddings. It need not be represented by one reversible map or even by one common enlargement. A filtered/common-cocone hypothesis is needed for that stronger normal form. Therefore “unique up to isometric environmental equivalence” is accurate; “unique up to reversible environmental equivalence” needs qualification.

### 14.3 The causal fork has three standard levels

The completion depends on which pure maps are present:

* all pure linear maps give unrestricted completely positive maps;
* pure contractions give trace-nonincreasing completely positive maps;
* pure isometries give causal channels.

The universal-property proof does not choose among these. It only completes the already sealed pure morphism class.

### 14.4 Distributivity belongs to the stronger endpoint

The quotient theorem above freely adds discarding relative to `J`. It does not automatically add all classical direct sums or all finite-dimensional C*-algebra objects. Distributivity is relevant to that stronger completion, not to Problem B's abstract initiality.

---

# 15. Combined conclusion for target 3

The two open problems separate cleanly into API selection and API realization.

1. **API selection remains open/bridged.** The current physical record clauses do not force isometry-invariant discard. Problem A has a countermodel, so Q1(b) must be declared or the physical record API strengthened.
2. **API realization is complete.** Once the class of environment embeddings and its causality equation are selected, `Env_J(P)` is initial among exact realizations. Problem B is proved without coproducts, distributivity, filteredness, or a zero-object assumption.
3. **The dagger dependency remains genuine.** A dagger is needed only to derive the intended embedding class `J=Isom(P)` from the pure layer; the free-completion theorem then uses that class.

The resulting honest ledger entry is:

> **Target 3 is conditionally derived:** Q1(b) is one counted bridge premise; conditional on it, purification/environment completion is a theorem of free sealing.
