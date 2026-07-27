# The Problem of Two: Negative Resolution by an Arity–Degree Countermodel

**Date:** 2026-07-25  
**Status:** solved negatively, relative to the permitted basis stated in
`260725_the_problem_of_two.md` §4.  
**Result:** Side A does **not** imply Side B. The obstruction is not merely that
the two sides currently use different index sets. Even after strengthening Side
A to an arity-two statement on the alternative interface itself, a nonlinear
public extractor can turn unary/binary primitive data into nonzero Möbius/Sorkin
coefficients of arbitrarily high order.

---

## 1. Verdict

Let `T_perm` denote exactly the permitted basis of the problem:

1. the full depth theorem `d_obl = 2`, including Level-1 existence, Level-2
   structural identity, and contractibility/computability at Level 3 and above;
2. the width-two chronological window;
3. the Fibonacci debt recurrence;
4. no Born-form premise, no engine-register `d²` or `r²`, no telescopic
   inheritance, and no register-to-degrees-of-freedom identification.

Assume, as the manuscript does, that the constitutive theory in `T_perm` has a
model. Then

\[
T_{\mathrm{perm}} \not\models
\bigl( I_T=0\text{ for every }|T|\ge 3\bigr).
\]

More strongly, the implication remains false after adding the proposed
R-alpha strengthening

> every primitive public scalar generator on the alternative interface has
> support arity at most two.

Thus no sound proof of XF-A from the permitted basis alone exists. A positive
proof must add an operational premise controlling the **algebraic degree of the
public extractor**—for example orthogonal additivity, affine dependence on the
pair decomposition, or the phase-neutral constructively-prime self-pairing used
in the independent Born derivation.

This is a theorem of non-entailment from the stated premises. It is not a claim
that no enlarged theory can derive XF-A.

---

## 2. The invariant mismatch

Side A and Side B bound different invariants.

### Side A: primitive-generator arity

Side A says that, after canonical computational replacement, no irreducible
primitive historical trace field has support on three or more earlier sealed
layers. It bounds the arity of the data from which public structure is built.

### Side B: Boolean polynomial degree

For a set function

\[
\mu:2^A\longrightarrow \mathbb R,
\]

the Möbius coefficients

\[
I_T=\sum_{S\subseteq T}(-1)^{|T|-|S|}\mu(S)
\]

are its coefficients in the multilinear Boolean expansion. Therefore

\[
I_T=0\quad (|T|\ge 3)
\]

is exactly the assertion that `mu` has Boolean polynomial degree at most two.
It bounds the algebraic degree of the **composite readout**, not the arity of
its primitive inputs.

A nonlinear map can raise algebraic degree without introducing a new primitive
high-arity input. That elementary fact is the decisive obstruction.

---

## 3. Countermodel theorem

### Theorem (Arity–Möbius separation)

Let `A` be a finite alternative set of cardinality `N >= 3`. Suppose the only
nonconstant primitive scalar datum on alternatives is the binary field

\[
g:A\times A\longrightarrow \mathbb R_{\ge 0},
\qquad g(i,j)=1.
\]

For an integer `m >= 1`, define the derived pair aggregate and normalized public
measure

\[
q(S):=\sum_{i,j\in S}g(i,j)=|S|^2,
\]

\[
\mu_m(S):=\left(\frac{q(S)}{q(A)}\right)^m
          =\left(\frac{|S|}{N}\right)^{2m}.
\]

Then:

1. `mu_m` is computed entirely from one binary primitive field and ordinary
   scalar operations;
2. `mu_m(emptyset)=0`, `mu_m(A)=1`, and `mu_m` is nonnegative, monotone, and
   permutation-invariant;
3. for every `T subseteq A` with `k=|T|`,

\[
I_T(\mu_m)
 =\frac{1}{N^{2m}}
   \sum_{j=0}^{k}(-1)^{k-j}{k\choose j}j^{2m}
 =\frac{k!}{N^{2m}}
   {2m\brace k},
\]

where `{n brace k}` is the Stirling number of the second kind;
4. consequently,

\[
I_T(\mu_m)>0
\qquad\text{whenever}\qquad 1\le k\le 2m.
\]

In particular, binary primitive data can generate nonzero Sorkin coefficients
of arbitrarily high order.

### Proof

The first two assertions follow immediately from the definitions. Since
`mu_m(S)` depends only on `|S|`, every `j`-element subset of a fixed `k`-element
set `T` contributes the same value `(j/N)^(2m)`. Hence

\[
I_T(\mu_m)
 =N^{-2m}\sum_{j=0}^{k}(-1)^{k-j}{k\choose j}j^{2m}.
\]

The finite-difference identity

\[
\sum_{j=0}^{k}(-1)^{k-j}{k\choose j}j^n
 = k!{n\brace k}
\]

now gives the displayed formula. The Stirling number `{2m brace k}` is strictly
positive for `1 <= k <= 2m`. Therefore `I_T(mu_m)` is nonzero throughout that
range. No primitive field of arity `k` has been introduced: the high-order
Möbius term is created solely by applying the nonlinear map `x -> x^m` to an
aggregate of binary data. QED.

---

## 4. The three-alternative witness

Take `A={1,2,3}` and `m=2`. Then

\[
\mu(S)=\frac{|S|^4}{3^4}.
\]

Thus

\[
\mu(A)=1,\qquad
\mu(\{i,j\})=\frac{16}{81},\qquad
\mu(\{i\})=\frac{1}{81},\qquad
\mu(\varnothing)=0.
\]

The third-order coefficient is

\[
\begin{aligned}
I_{123}
 &=1-3\frac{16}{81}+3\frac{1}{81}-0\\
 &=\frac{36}{81}\\
 &=\frac49\ne0.
\end{aligned}
\]

This is the normalized form of the manuscript's `I_3=36` counterexample. It
also satisfies natural regularity properties not required by the problem:
normalization, monotonicity, positivity, and full permutation symmetry.

---

## 5. Relative non-entailment

### Corollary (No proof from the permitted basis)

Assume the constitutive Side-A theory is consistent, equivalently that it has a
model `C`. Expand `C` by adjoining the alternative interface and derived measure
of the theorem above. The constitutive component is unchanged, so

- `d_obl=2` still holds;
- the width-two historical window still holds;
- the Fibonacci debt recurrence still holds;
- no barred engine-register law has been used;
- even the strengthened assertion “all alternative-interface primitive scalar
  generators have arity at most two” holds.

But `I_{123}=4/9`, so XF-A is false in the expansion. Therefore XF-A is not a
logical consequence of the permitted basis. By soundness, no valid derivation
from only those premises can exist. QED.

The consistency qualification is the standard one: an inconsistent premise set
entails every formula by explosion. The book treats Side A as a proved,
mechanized theorem with intended models, so relative non-entailment is the
relevant verdict.

---

## 6. Why R-alpha cannot solve the problem

Appendix C's actual theorem is historical: support is a set of earlier sealed
layers, higher depth is introduced as one more remote horn factor, and the final
factorization is through the previous two normalized signatures. The proof is
therefore not presently a theorem over arbitrary alternative index sets.

The core Kan statement—contractibility of a horn-extension fibre over a fixed
boundary—might admit an index-neutral abstraction. But even granting the full
R-alpha generalization does not help: the countermodel above already has only
binary primitive data on the **alternative** index set itself. What fails is not
only transport of the index set. What fails is transport from

> primitive support arity at most two

to

> composite scalar extractor of Boolean degree at most two.

R-alpha addresses the first phrase and says nothing about the second.

---

## 7. Internal confirmation from the Born section

The book's independent Born argument already displays the missing premise. It
states that phase-blind monoidal scalars built from one amplitude fibre have the
family

\[
|\psi|^{2k},\qquad k\ge0,
\]

and selects `k=1` only by **constructive primeness**, which rejects repeated
copies of the same proof/conjugate-proof pair.

The countermodel family is exactly the coherent-alternative version of the
unselected cases:

\[
\mu_m(S)
 = \frac{\left|\sum_{i\in S}1\right|^{2m}}
        {\left|\sum_{i\in A}1\right|^{2m}}.
\]

For `m>1`, the primitive amplitude/comparison data remain unary or binary, but
the Möbius degree rises to `2m`. Thus the book itself confirms that binary
self-pairing does not select the square without an additional minimality or
primality principle. That additional principle is not in the permitted basis of
this problem.

---

## 8. Route assessment after the theorem

### R-alpha — generalized interface arity

**Insufficient.** It may remove the historical/simultaneous index mismatch, but
not the generator-arity/extractor-degree mismatch. The countermodel survives
the proposed generalization.

### R-beta — realize alternatives as layers

**Insufficient even if constructible.** Reindexing the same binary data as
successive layers does not stop a nonlinear extractor from generating higher
Möbius degree.

### R-gamma — binarity plus independently derived additivity

**Viable.** If B.5 independently proves that public weight is affine/additive
over the unary and pair decomposition, then grade-2 follows. But the resulting
theorem has at least two inputs:

1. Side A supplies a unary/binary generating interface;
2. the operational reconstruction supplies degree-preserving additivity.

This is not the requested single-root implication from `d_obl=2` alone.

### R-delta — equivalence rather than implication

**Also needs an additional faithfulness premise.** Grade-2 constrains one public
measure. By itself it cannot rule out an independent ternary primitive field in
some other part of the public signature. Conversely, arity-two primitive data
does not constrain nonlinear measure degree. An equivalence theorem would need
the measure extractor to be exhaustive and faithful for all primitive public
content.

---

## 9. The exact positive theorem available after widening the basis

### Proposition (Affine pair extractor implies XF-A)

Suppose there are constants `c`, unary coefficients `u_i`, and binary
coefficients `v_ij` such that for every event `S subseteq A`,

\[
\mu(S)
 =c+\sum_{i\in S}u_i
   +\sum_{\{i,j\}\subseteq S}v_{ij}.
\]

Then

\[
I_T=0\qquad\text{for every }|T|\ge3.
\]

### Proof

In the Möbius sum for a fixed `T`, the constant term has coefficient
`(1-1)^|T|`, each unary term has coefficient `(1-1)^(|T|-1)`, and each binary
term has coefficient `(1-1)^(|T|-2)`. All vanish when `|T|>=3`. QED.

This proposition identifies the exact missing bridge: the public extractor must
be affine in the unary/pair generators, or must be derived from independent
axioms that force that affine form. Merely saying that its inputs are unary and
binary is not enough.

---

## 10. Required record amendments

### `260725_the_problem_of_two.md`

Change the status from **open** to:

> **Solved negatively.** Side A does not imply Side B from the permitted basis.
> A normalized family computed solely from binary primitive data has nonzero
> Möbius coefficients of arbitrarily high order. The missing condition is a
> degree-preserving operational extractor, not merely a common arity bound.

Replace the last one-line summary with:

> **The two statements resemble one another, but they bound different
> invariants: Side A bounds primitive-generator arity, while Side B bounds the
> Boolean polynomial degree of a composite measure. Nonlinear extraction
> separates them.**

### XF-A derivation obligation

The obligation “derive XF-A only from width-two structure and `d_obl=2`” should
be marked **impossible as stated**, subject to the standard consistency
assumption. It can be replaced by either:

1. the existing independent conjugate-self-pairing derivation; or
2. an amended obligation allowing an explicitly counted operational premise
   such as orthogonal additivity/constructive primeness.

### Book language

- Chapter 5's “meant to be echoes” is the correct theorem-safe grade for this
  link.
- Any claim that the Born two and coherence-depth two have a single logical
  source should be qualified.
- It remains defensible to say both are selected within the broader Two-Law
  architecture, provided the distinct derivation inputs are named.

---

## 11. What is unaffected

This negative result does **not** refute either side separately.

- Side A remains the depth-two theorem for the fixed cubical extension calculus.
- Side B retains its independent derivation from conjugate self-pairing/Born
  extraction.
- The chain

\[
d_{\mathrm{obl}}=2
\Longrightarrow
\text{two-step historical window}
\Longrightarrow
\text{affine Fibonacci recurrence}
\]

is untouched.
- The result rejects only the claim that XF-A follows from that historical depth
  theorem without an additional operational extractor principle.

---

## 12. Final theorem statement

> **Negative Resolution of the Problem of Two.** Assuming the consistency of
> the fixed Side-A constitutive calculus, the depth-two theorem, width-two
> chronological window, and Fibonacci debt recurrence do not entail vanishing
> Möbius/Sorkin coefficients above order two. This remains true even if the
> depth theorem is strengthened to forbid primitive fields of arity above two
> directly on the alternative interface. For every desired interference order
> `k`, binary primitive data admit a normalized positive derived measure with a
> nonzero `k`th Möbius coefficient. Therefore a degree-preserving operational
> extractor—such as independently derived affine/orthogonal additivity or the
> constructively-prime conjugate self-pairing—is a logically independent
> requirement.
