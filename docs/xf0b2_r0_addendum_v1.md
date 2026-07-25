# XF-0b2 — R-0 addendum (post-freeze)

**Date:** 2026-07-24. **Status:** additive to `docs/xf0b2_r0_dossier_v1.md`,
which is NOT edited (the R-1 analysts were armed from it and it stays as
they read it). Every item below was produced *after* the dossier froze, in
response to questions the armed analysts raised. Raw output:
`docs/xf0b2_r0_probe_output.txt`, sections ADDENDUM A and ADDENDUM B.

## A. Could the record have written a dependent version at this exact position?

Asked by the L1 analyst, who rested a non-poverty rebuttal on the claim
that `Pi(Lam(Var 1), Sigma(Var 1, Var 5))` would bind the codomain to the
Π binder, but had not run it through the kernel. Run now, in the real
stage-14 context (ambient 1, prior kernel roles `[Formation; 3]`, visible
library 13):

| Variant | Result | Coarse |
|---|---|---|
| `Pi(Lam(Var 1), Sigma(Var 1, Var 2))` — **the sealed clause** | elaborates, `Type` | **2** |
| `Pi(Lam(Var 1), Sigma(Var 1, Var 5))` — codomain bound to the Π binder | elaborates, `Type` | 3 |
| `Pi(Lam(Var 1), Sigma(Var 1, Var 6))` — codomain bound to the Σ binder | elaborates, `Type` | 3 |
| `Pi(Var 1, Sigma(Var 1, Var 5))` — type domain *and* dependent codomain | elaborates, `Type` | 1 |

**The rebuttal holds, and is stronger than the analyst claimed.** A
genuinely dependent product at this exact position is not merely writable
in the grammar — it *elaborates*, in this context, at this clause index.
The record wrote the non-dependent one. Further, the fourth row shows the
well-sorted dependent form `Π(X, Σ(X, ·))` elaborates at **coarse 1**,
strictly cheaper than the sealed clause's 2: the sealed clause is not the
kernel's cheapest way to say anything nearby.

## B. Does the sealed corpus ever bind a `Var` to a local binder?

Also asked by the L1 analyst, who could not settle it. Swept all 15 sealed
telescopes, resolving every `Var` occurrence at its own binder depth under
the frozen convention (binders = `Lam` bodies and `Pi`/`Sigma` codomains
only), each step at its own minimal ambient arity:

| Step | Clause | Expression | Binder-bound levels |
|---|---|---|---|
| 11 | 0 | `Pi(Lib 10, Pi(Var 1, Var 1))` | 1, 1 |
| 11 | 1 | `Lam(Pi(Var 1, Var 2))` | 2 |
| 12 | 0 | `Pi(Lib 11, Pi(Var 1, Var 1))` | 1, 1 |

**Total binder-bound `Var` occurrences in the entire sealed corpus: 5,
across 3 clauses.** So binder reference is a device the record uses — it
is rare, but it is not absent, and it is therefore available. Stage 14
uses it nowhere: every `Var` in all nine of its clauses is a free
reference to the ambient parameter or to a prior field.

**Consequence for the study.** Both of L1's non-poverty rebuttals are
confirmed mechanically rather than by inspection. The failure of the
dependent readings across leads L1–L5 is a fact about what the record
*says*, not about what its grammar *can* say.

## C. Clarification of a dossier line (phrasing, not content)

Raised by the L3 analyst. Dossier §1.3 says "Naturality square
(transposition (1 2)) closes judgmentally in 0 steps". Per the doc comment
at `crates/pen-eval/src/typed_families.rs`, that square certifies that
**canonicalization is stable under bijective renaming of the parameter
telescope** — it does *not* say the family is symmetric in X and Y. It is
not: X occurs twice (in the domain abstraction and in the product's left
factor), Y once. `Pi(Lam(Var 1), Sigma(Var 1, Var 2))` and
`Pi(Lam(Var 2), Sigma(Var 2, Var 1))` are distinct raw expressions.

**Any naming attempt that treats X and Y as interchangeable on the
strength of that line is void.** No attempt in this study did so; the
clarification is registered so none later does.

## D. New mechanical fact contributed by the L3 analyst, verified

`elaborate.rs`'s `app-el-pi` rule fires only when the `El`'d expression is
an `Expr::Pi`. The Π-binder of clause 3 carries classifier
`El(Lam(Var 1))`, which is not a `Pi`. Therefore **an inhabitant of this
Π's domain has no elimination behaviour in the kernel at all**:
applications on it fall through to `app-stuck`. This is an independent,
mechanical confirmation that the domain is not functioning as a type —
nothing can be computed with its elements.

The converse must not be claimed and is not: the Π *itself* would be
eliminable, since `El(Pi(...))` does match the rule. The record simply
contains no clause that eliminates it.

## E. Correction: "the η hinge" was a coordinator error, and there is no hinge

Three analysts (L3, L4, A) and the coordinator's own verifier brief treated
the verdict as resting on the kernel's η-freeness — analyst A's phrase, "the
verdict is exactly one frozen convention deep". The brief stated it as *"Under
an eta rule, `Lam(Var 1)` collapses to `Var 1`."*

**That sentence is false and it was the coordinator's, not an analyst's.** The
η rule for Π contracts `λx.(f x)` to `f`; its redex shape requires the body to
be an *application of the bound variable*. The body here is `Var 1`, the
ambient parameter. `λ_.X` is not an η-redex, and adding η to the kernel would
not collapse it. No published type theory identifies a constant function with
its value: `λ_.X : A → Type` and `X : Type` are of different types.

The frozen dossier had this right at §2 item 3 ("An η-redex at scope 4 would
be `Lam(App(f, Var 5))` … The body is `Var 1`. Not an η-redex"); the analysts
and the brief drifted from it. What the corpus calls "the η collapse" is
**binder erasure**, not η — either the CwF reinterpretation of the `Lam`'s
binder as the ambient context (which the kernel refuses: it pushes a fresh
local classified `Neutral`), or Automath's identification of λ with Π. The
adversarial verifier ran the latter and found that even there `[x:A]X` read as
a type is `A → X`, not `X` — so the one historical system licensing the shape
yields `(A → X) → (X × Y)`, not `X → X × Y`.

**Consequence:** a campaign ruling that η-freeness is a mere presentation
convention would change **nothing**. The verdict is not one convention deep;
it is as deep as the absence of any rule identifying a constant function with
its value. Three analysts reached the right disposition for an understated
reason.

## F. Amendment: dossier §5 hard constraint 1 proves too much

Frozen dossier §5 states as a standing constraint on all lawful translations:
*"The clause is equation-free … A target whose defining data include equations
has no source counterpart for them."*

**As a general constraint this proves too much.** The register exports **zero**
Computation-role clauses across all fifteen entries, so equation-freeness is a
property of the whole record; as a standing bar it would forbid ever naming any
family whose target carries laws — permanently, for the entire census —
including the 31 families XF-0b *did* name.

**Amended to a per-target observation:** the admissible form of the objection
is never "the source has no equations" but "**this** target is *defined by* its
equations" (Beck–Chevalley, the adjunction bijection, the decoding equation,
the reproducing property, dot-reduction). The adversarial verifier audited all
eight R-1/R-2 reports target by target and found **no surviving obstruction
resting on equation-freeness alone**; every row carries an independent
structural or sort kill. So no verdict changes. The constraint is nonetheless
amended here so no later study inherits it in the over-strong form.

Registered with it: L2's "two independent kill-shots per target" upgrade of
equation-freeness into a general anti-poverty rebuttal is the weakest argument
in the corpus and must not be cited on its own.

## G. Analyst B's flip-condition was met, and did not flip

Analyst B named the single citation that would make it withdraw its verdict: *a
published formation rule of the form "Γ ⊢ a : A → Type implies Γ ⊢ Π(a, B)
type"* — a system forming Π with a family-as-term in the domain slot, with no
intervening decoding, index, or display map.

**The citation exists.** Automath's AUT-QE type inclusion (Wiedijk, JAR
29(3-4):365-387, 2002, §3.1, flag `-q`; de Bruijn, LNM 125, 1970;
Kamareddine–Wells–Ventura, CSR 2015) states that an expression of category
`[x1,A1]…[xn,An]TYPE` is also correct for `[x1,A1]…[xk,Ak]TYPE` for k < n. At
n=1, k=0 that is exactly the rule B asked for — and it is, node for node, the
kernel's own coarse subsumption #2. L5 had already found it and filed it under
a different heading.

**It does not flip the verdict**, for three reasons the verifier established:
a formation *rule* is not a *structure* and has no defining data for clauses to
map onto; the object the rule licenses is `(A → X) → (X × Y)` with **A
anonymous**, which is not a parameter of the credited family and cannot be
supplied without changing it (F-R4); and the lift depends on Automath's λ/Π
identification, which the frozen kernel does not make.

**B's flip-condition was mis-specified**: it asked for a *rule*, and the
translation standard asks for a *structure*. Registered so the campaign does
not treat the condition as still open.
