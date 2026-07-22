# UC-1: The Univalent Collapse Hypothesis

**Registered hypothesis, frozen before the BI-1 artifacts are read.**

**Date:** 2026-07-22. **Status:** frozen. Naming (`UC-1`) provisional.
Registered from the user's stated intuition, pinned verbatim for
provenance:

> My (weak) intuition is that these four structures are actually
> equivalent/univalent. The reason that they appear as 4 different
> structures is that the grammar/language we are using is a bit too
> fine grained. […] not only are the per stage vectors identical, the
> structures from stage 5 and onwards are the exact same (or
> equivalent/univalent).
>
> — Halvor Lande, 22 July 2026

**Epistemic state at registration (firewall).** Known at freeze time:
the R-T1 four-class verdict, the R-T2 refutation (matchings 3,2,3,3,2,3
of 5; 2+2 arity-3 key classes), the four-branch preliminary BI report
at halt granularity only ("each root has exactly one typed A3
discharger at stages 5–15, reaches debt-free 15 → 16, and each
branch-local finale proves semantic O(16) = ∅ with F1 excluded").
**Not known:** BI-0 regression validity; the per-stage (κ, ν) vectors
of the non-enacted branches; the winner telescopes at stages 5–15 per
branch; which scheme carries each branch's live demand. Predictions
P1–P3 below are registered against exactly this ignorance. This
document is a forbidden input to all BI computations (F-BI1 unchanged)
and to any blind derivation.

---

## 0. One-paragraph statement

The Stage-4 cone is one world seen four ways. The frozen equality is a
syntactic under-approximation of univalent equivalence — the same
disease, one rung higher, that made 89 singletons of 8 families before
naturality transport was carried. The four acts are pairwise
equivalent at act level (the level of the library extension they
enact, not the level of their clauses), the continuations from Stage 5
onward are the same structures up to references into the Stage-4
content, and R-T2's divergent schemes live entirely in the unenacted
margin — possibility that never becomes world. If UC-1 holds in full,
branching is gauge, the R-T3 Option-B adoption self-simplifies (a cone
of equivalent branches is exactly the gauge-freedom reading R-T2's
clause anticipated), Genesis is effectively deterministic again, and
the theory's one free moment dissolves into presentation.

## 1. Grounds (dated before the tie; the tie is occasion, never ground)

- **G-1.** The ambient theory has been cubical HoTT with univalence
  from Genesis step 1. The frozen beta-normal equality was always a
  conservative syntactic approximation of the theory's own identity.
- **G-2.** The 89-vs-8 precedent: the machinery has erred too fine
  before, and the lawful cure was carried transport, not relaxed
  discipline.
- **G-3.** The tie protocol's rung-1 principle, adopted: presentation-
  multiplicity is not a tie.
- **G-4.** Constructive idealism as recorded: what exists is the act
  of world-extension; act-identity may be coarser than clause-identity.

Not grounds (F-UC5): the BI preliminary report, the attractiveness of
a deterministic Genesis, the desire to simplify Option B's obligations.

## 2. Hypothesis clauses (separable; each burns independently)

- **UC-1a (order gauge).** Within each former, the two application
  orders are exchange-equivalent: a typed kernel equivalence swaps the
  two parameters (they are mutually non-dependent) and transports one
  eliminator package to the other. Collapses the cone 4 → 2.
- **UC-1b (act-level former collapse).** Π-package and Σ-package are
  equivalent *as acts*: there is a library equivalence B₄^Π ≃ B₄^Σ
  under which the `former_eliminator` discharge witnesses correspond.
  Explicitly NOT claimed: Π ≃ Σ as types (false in general in
  univalent foundations; the frozen equality's refusal at type level
  is correct). Collapses 2 → 1.
- **UC-1c (healing).** The continuations coincide above the fork:
  stage-5–15 winners are identical modulo the induced relabeling of
  references into Stage-4 content; the enacted demand chain is common
  to all branches; every R-T2 scheme mismatch is confined to the
  unenacted margin.
- **UC-1d (full collapse).** UC-1a + UC-1b + UC-1c: the cone is one
  world in four presentations; branching is gauge; the induced
  transport matches all successor scheme sets 5/5, lawfully superseding
  the R-T2 comparison by theorem (never by relaxation).

## 3. Pre-registered predictions (decidable from BI artifacts on arrival)

- **P1 (ledger identity).** The per-stage (κ, ν) vectors of all four
  branches are identical at stages 5–15. Equivalence transports
  ledgers; any divergence anywhere refutes UC-1c and UC-1d outright.
- **P2 (winner identity modulo fork shadow).** The canonical winner
  telescopes at stages 5–15 agree across branches after relabeling
  Stage-4 references along the branch correspondence; all divergence
  between branch continuations factors through the Stage-4 clauses.
- **P3 (margin confinement).** For every stage and every branch pair,
  the live demand lies in the pairwise-matched scheme subset. No
  enacted obligation ever falls in a mismatched scheme.

P1–P3 are read off the sealed BI artifacts; no new runs are required
to score them.

## 4. Construction tasks (only if P1–P3 survive)

- **U-T1 (exchange equivalence).** Prove UC-1a in the kernel: the
  non-dependence of the two parameters, the swap equivalence, the
  package transport. Create-new, Agda-mirrored where the pattern
  requires.
- **U-T2 (act equivalence).** Construct B₄^Π ≃ B₄^Σ with witness
  correspondence, or exhibit the obstruction as a named certified
  invariant. This is the hard clause and the hypothesis's center of
  gravity.
- **U-T3 (transport of futures).** Lift the U-T1/U-T2 equivalences to
  B₁₅ along the healed continuations and show induced 5/5 scheme
  matching — the theorem that lawfully supersedes the R-T2 verdict.
- **U-T4 (gauge certificate).** The cone-is-one-world certificate:
  branch index = gauge, with replay and mutation falsifiers.

## 5. Outcome zones (registered before any scoring)

- **Z-COLLAPSE.** P1–P3 pass, U-T1..U-T4 land → the cone is a point up
  to gauge; Option B self-simplifies via versioned successor; the
  Stage-4 stop of T-BF1 dissolves by certified equivalence (the exact
  consequence-option 1 the T-BF1 record names); the bar-free program
  unblocks in full.
- **Z-HEAL.** P1–P3 pass but U-T2 finds a certified obstruction → the
  fork is real but consequence-free: genuinely inequivalent acts with
  identical futures. The world forgets its choice. Nondeterminism
  stands at Stage 4 and is confined to it; recorded, not repaired.
- **Z-LEDGER.** P1 passes, P2 or P3 fails → branches share numbers,
  not structures (the BI Z-ISO reading at act level); UC-1c burns;
  UC-1a/b remain testable but lose their strongest consequence.
- **Z-REFUTED.** P1 fails, or a certified distinguishing invariant
  lands → the hypothesis burns; the four branches are different
  worlds; the branch index is physics.

## 6. Falsifiers

- **F-UC1.** Any per-stage (κ, ν) divergence at stages 5–15 across
  branches → UC-1c and UC-1d burned.
- **F-UC2.** Any enacted live demand in a pairwise-mismatched scheme →
  UC-1c burned as stated; no "approximately healed" reformulation.
- **F-UC3.** Kernel refutation of the exchange equivalence (a genuine
  dependence between the two parameters, or transport failure) →
  UC-1a burned; the order axis is structure, not gauge.
- **F-UC4.** A certified invariant, preserved by every equivalence in
  the adopted class, that differs across the four acts → UC-1b and
  UC-1d burned.
- **F-UC5 (discipline).** Any coarsening, amendment, or bypass of an
  adopted comparison justified by BI results, by this hypothesis's
  attractiveness, or by the desire for determinacy → invalid on its
  face. Adopted comparisons are superseded only by proven theorems
  whose grounds are G-1..G-4.
- **F-UC6 (no post-hoc equivalences).** If UC-1 burns at any clause,
  no new equivalence notion may be registered to rescue the collapse
  unless it derives from grounds independent of the burn record.
  Successors state what changed and why the change predates the data.

## 7. Relation to the standing program

UC-1 gates nothing. BI-1 completes and BI-4 issues regardless; the
branch indices remain governed by F-R3-B1 until BI-4 speaks, whatever
P1–P3 say. R-T3's Option-B adoption is not contingent on UC-1: in
Z-COLLAPSE it self-simplifies, in Z-HEAL it is vindicated as stated,
in Z-REFUTED its price sheet was already accepted. T-BF1's Stage-4
stop resolves only in Z-COLLAPSE, and only by the certified
equivalence rerunning rung R-T1 — never by this document. The bridge
waits on BI-4 in every zone.
