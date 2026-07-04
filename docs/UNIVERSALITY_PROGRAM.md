# The Universality Program: From Lane-Conditional Trace to Presentation-Invariant Theorem

**Status:** conjecture stated, decomposed into a lemma ladder (2026-07-02).
U0 proven; U1 proven at sketch (Schema Calculus + L1/L2 + V2b); U2 within
reach and given an empirical twin; U3–U5 scoped. Companion to
`EVALUATOR_DERIVATION.md`.

---

## 1. The Caveat Being Attacked

The Genesis result is currently stated lane-conditionally: *the strict trace
under the disclosed expressive basis, admissibility policy, and bounded
search — not yet a universality theorem across all richer signatures.* The
goal of this program is to replace that caveat with a theorem, and to say
precisely what remains conjectural at every stage in between.

## 2. The Conjecture, Stated in Its Natural Form

The right form of the universality claim is dictated by the framework itself.
The Constitutive Law's second requirement is equivalence-invariance:
structurally equivalent presentations are the same object. A signature (an
expressive basis) is a *presentation* of the constructive order. So the
universality claim is Requirement 2 applied to the Genesis process itself —
the theory predicts the shape of its own universality theorem:

> **Conjecture (Genesis Universality).** Let Σ be any *admissible basis*: a
> signature interpreting the dependent core and satisfying the Constitutive
> Law's four requirements (witness-bearing, univalent, depth-2 dischargeable,
> canonical). Let PEN_Σ be the selection dynamics over Σ with the
> Schema-Calculus evaluator (D1–D3, P1–P6). Then the accepted trace of PEN_Σ
> from the empty context is, up to univalent equivalence of sealed libraries
> and the gauge of Observation B, the canonical fifteen-step trace: there is
> a step-preserving equivalence of libraries B₁₅^Σ ≃ B₁₅ with identical
> (ν, κ) ledgers, terminating at the same Univalent Horizon.

In words: **the Genesis sequence is a presentation-independent invariant of
the two laws.** Univalence is not just the ambient foundation here; it is the
statement's engine — signatures are presentations, and equivalent
presentations must select equivalent structure.

Full mechanization of this conjecture in Cubical Agda is a long-horizon goal
(and explicitly beyond the current philosophical-exploration scope). The
book's Genesis chapter, together with Appendices B/D/J and the L1/L2
derivations, already constitutes an informal proof of the **fixed-basis**
case; what the ladder below adds is the quantifier over bases.

## 3. The Lemma Ladder

The conjecture decomposes along the four axes of lane-conditionality. Each
rung is independently valuable, independently checkable, and several have
cheap empirical twins in the existing engine.

**U0 — Gauge invariance. PROVEN.** Selection is invariant under global
rescaling of the schema-counting unit (Observation B,
`EVALUATOR_DERIVATION.md`). Kills the "units are arbitrary" axis.

**U1 — Evaluator invariance. PROVEN (sketch) + VERIFIED.** The Schema
Calculus derives the counts from constructor *kinds* (D1–D3, P1–P6, Lemmas
L1/L2, the H-space enumeration), not from syntax details; V2b confirmed the
canonical trace hash-identically under the corrected evaluator. Kills the
"evaluator choices" axis modulo the two flagged mechanization steps.

**U2 — Conservative-signature invariance. NEXT TARGET; provable.**
*Claim:* for any admissible extension Σ′ ⊇ Σ by **definable** nodes (new
primitives whose semantics is expressible in Σ), the post-canonicalization
candidate fields coincide and selection is unchanged.
*Proof route:* a definable node's telescope is definitionally equal to its
Σ-expansion; canonical dedup identifies them; charged κ is invariant by D3
(derivability contrast); ν is invariant by U1 (counts depend on kind, not
spelling). This is a finite, self-contained argument over the
canonicalization machinery — the most tractable genuine theorem on the list.
*Empirical twin (cheap, run it first):* the **signature-extension ablation**
— the mirror image of the hostile grammar ablations already run. Add
redundant/definable primitives to the MBTT grammar (a sugared Σ-type, a
defined modality composite, an alias node) and rerun the strict lane:
prediction, trace-invariance. Every such run is a new audited point in the
universality moduli space. Logged as Round-2 action 5c in
`new_run_instructions.md`.

**U3 — Genuinely richer bases. THE HARD CORE; attack by moduli space.**
For non-definable extensions (new modality families, new operator kinds),
the template is Appendix J's Minimal-Extension Theorem, which already proves
the Step-15 instance: at the stall point, grammar growth is forced and the
minimal coherent growth is unique. The general rung is the **stepwise
minimal-extension property**: at every step n, no coherent basis extension
fields a candidate that clears Bar_n before the canonical winner, except
extensions whose winner is univalently equivalent to it.
*Pragmatic reformulation:* do not chase the universal quantifier. Define the
**audited moduli space M** of bases for which the property is verified, and
grow it. M already contains: the three claim lanes (guarded / relaxed /
desktop — convergent per the certified bundles), the two hostile ablation
profiles (no-temporal, linear-exponential: Steps 1–14 replayed, Step 15
stalled exactly as J predicts), and — after 5c — the definable-extension
family. Each new basis family in M converts conjecture into theorem-over-M.
The book can then say precisely: "universality is proven over M = {…},
conjectured in general, in the stated univalent form."

**U4 — Search completeness. SCOPED.** Exact-band enumeration is exhaustive
within bands by construction; the exposure is the node caps and breadth
floors. Rung: show caps are non-binding at the winners (enlarging caps by δ
leaves selection fixed). The v15 breadth-gate certification and the
demo-breadth lanes are the existing empirical points; a cap-perturbation
sweep (±δ on max_expr_nodes, quotas) makes it systematic.

**U5 — Policy necessity. SCOPED.** The admissibility bands and focus
families are computed from the two-layer debt (`obligations.rs`), i.e. the
policy is itself depth-2 output, not curator input. Rung: prove the
debt-to-band map is the unique depth-2-coherent policy, or empirically that
coherent policy perturbations preserve winners (the three-lane convergence
is the existing evidence).

## 4. Sequencing (pragmatic, in order of value-per-effort)

1. **5c signature-extension ablations** (days; engine infrastructure exists)
   — first new points in M, and the cheapest possible falsification exposure
   for U2.
2. **U2 as a written theorem** (the conservative-extension argument over
   canonical dedup; paper-appendix rigor, then Agda target).
3. **Cap/policy perturbation sweeps** (U4/U5 empirical; batch runs).
4. **U3 instances**: prove stepwise minimal-extension for Steps 5–8 rivals
   (ℕ-like, second-circle-like, bare-sphere families are already catalogued
   in the rejected-candidate corpus) — each instance is an Appendix-J-style
   theorem localized at one step.
5. **The Agda horizon**: the fixed-basis theorem first (the trace as a
   formal object — the existing mechanization plus L1/L2's flagged steps),
   the Σ-quantifier last.

## 5. What This Buys the Book, Honestly

After rung 1–2, the Genesis chapter's first caveat can be rewritten from
"not yet a universality theorem" to: *"proven invariant under evaluator
gauge, evaluator derivation, and conservative signature extension; verified
across the audited moduli space of lanes and ablation bases; conjectured
universal in the precise univalent form of the companion program."* That is
no longer a caveat. It is a research frontier with a theorem-shaped boundary.
