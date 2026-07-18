# SH-1: The Quantized Homotopy Staircase

**Continuation-branch hypothesis, registered for engine testing.**

**Date:** 2026-07-18. **Status:** frozen for the SH-1 engine experiment before
any T1--T4 output was produced. This document
does not withdraw the registered halt claim; both branches remain live until
the adjudication protocol below completes. Naming (`SH-1`) is provisional.

**Firewall.** This document was produced in a context that contains the T1
survivor table and the Step-16 certificate. It is therefore an admissible
input to harness construction (T1–T3) and to the comparison protocol, but it
is a **forbidden input** to the blind valuation derivation (T4). The verbatim
prompt for the T4 thread is in Appendix A and contains no survivor,
staircase, or homotopy-table content beyond what the registered record
(Guard-Rail note, Remark 18) already states. Per the standing rule, the bar
value must not be an input to any valuation law, capacity table, or
admissibility gate produced under this hypothesis.

---

## 0. One-paragraph statement

The two laws' directive content completes at Step 15: nothing after it is
owed (Theorem 12, engine-verified). If anything follows, it is not owed but
*bought*, and the ledger arithmetic sharply constrains what can be bought:
under an honest valuation, every extraction channel (inheritance,
polymorphic instantiation, unanchored fallback credit) collapses below the
bar, leaving exactly one channel — formed path attachment — whose credit
grows with cell dimension. The bar's golden-ratio growth then forces the
continuation into a **quantized staircase of homotopy dimensions**: long
plateaus at fixed d, with jumps forced when the rising bar exhausts the
plateau's margin. Under the shipped credit shape the first rung is forced at
**d = 4**, and its attachment classes are the winding classes π₃ of the
sealed sphere stratum — that is, the first rung is **θ-sector structure**.
The continuation, if real, changes the source of the theory's numbers from
grammar counting (Fibonacci, parametric) to homotopy torsion (ℤ, 2, 2, 12,
24 — arithmetic, physical).

## 1. Fixed premises (not under test)

- **P1.** The two laws as frozen; lane band κ ∈ 2..4; min-overshoot
  selection; identification before scoring.
- **P2.** The Genesis 1–15 record: Σν = 359, Σκ = 64; bar recursion
  Bar_n = Φ_n · Ω_{n−1} with Φ_n = F_n/F_{n−1};
  Bar₁₆ = 354333/39040 ≈ 9.0762.
- **P3.** Theorem 12 (Guard-Rail): O(16) = ∅, engine-verified
  (`docs/o16_emptiness.json`). Step 16 is the open band.
- **P4.** The halt at 15 is **not** assumed. Theorem 17 governs: halt iff
  sup ρ < Bar₁₆ on the open band; continuation at the smallest-margin
  survivor of the honest screens.
- **P5 (placeholder credit shape).** The shipped L1 form at κ = 2: a formed
  path package `[Formation over the window, PathAttach PathCon(d)]` scores
  ν(d) = 5 + d² (engine: `hit_path_d*`). This is a *placeholder* pending T4;
  all quantitative consequences in §3 are conditional on it and are
  recomputed under the blind law before any engine comparison (§5, grading).

## 2. Hypothesis clauses

- **SH-1a (channel selection).** On a debt-free field, any clearing candidate
  whose credit is obligation-free reference — telescopic inheritance,
  per-entry polymorphic instantiation, or the no-formation fallback charge
  κ + |library| — receives law-level ν below the bar under any
  obligation-relative valuation. The shipped minimum-overshoot survivor
  (`hit_no_formation_d1`, ν = 19 = 2 real + 17 fallback) is the boundary
  case: its fallback term is not honest credit; its 2 real units (one new
  1-cell) are the seed of the only channel internality cannot dissolve.

- **SH-1b (capacity law).** d-dimensional path credit presupposes filling
  capacity, made precise: the attachment must land in a **nontrivial class
  of π_{d−1} of a sealed stratum**. Credit for an attachment along a trivial
  or unhosted class is 0 at law level. (This clause is the L1 validity
  domain. It is *assumed here to derive consequences*; whether it is the
  honest law is exactly what T4 must decide, blind.)

- **SH-1c (licensed rungs).** Capacity of B₁₅ is a computable homotopy fact
  through the step-5–8 sphere/HIT strata: π₃(S²) = π₃(S³) = ℤ;
  π₄(S³) = ℤ/2; π₅(S³) = ℤ/2; π₆(S³) = ℤ/12. Hence d = 4 is licensed with
  multiplicity ℤ, d = 5 and d = 6 with multiplicity 2, d = 7 with
  multiplicity 12.

- **SH-1d (staircase).** The ledger recursion plus min-overshoot forces
  quantized plateaus: at fixed d the sealed ρ = (5+d²)/2 is constant while
  Ω_n climbs toward it and Bar_{n+1} ≈ φ·Ω_n climbs past it, forcing a jump
  to the minimal next licensed d. Under P5 exactly: the first rung is forced
  at d = 4 because Bar₁₆ ≈ 9.076 lies in the window (7, 10.5] between the
  d = 3 and d = 4 scores; dimensions 1–3 are *skipped*.

- **SH-1e (theta rung).** The first rung's attachments are classified by
  π₃ = ℤ: winding sectors, with the min-winding tie-break selecting k = 1.
  This is θ-vacuum/instanton structure; the attachment along the Hopf class
  builds ℂP². Physical reading: the first post-completion structure is the
  theta sector.

- **SH-1f (torsion output).** Sector multiplicities of successive rungs are
  the theory's first non-Fibonacci, non-grammar integers: ℤ (d=4), 2 (d=5),
  2 (d=6), 12 (d=7), with the stable 3-stem's 24 adjacent. These are the
  integers that appear in physics as vacuum-sector counts and anomaly
  coefficients (π₄(SU(2)) = ℤ/2 is the Witten anomaly; π₃(G) = ℤ is the
  instanton grading).

## 3. Deterministic consequences under P1–P5 (comparison targets)

Computed 2026-07-18 by exact-rational iteration of the bar recursion,
min-overshoot per step, κ = 2 throughout. To be reproduced in-engine by T2.

**Branch D (formed d²-ladder, ungated or licensed-at-4):**

| rung | steps | plateau length | ν per step | ρ | final-step margin |
| ---: | --- | ---: | ---: | ---: | ---: |
| d=4 | 16–23 | 8 | 21 | 10.5 | ≈ 0.0035 at step 23 |
| d=5 | 24–42 | 19 | 30 | 15 | — |
| d=6 | 43–68 | 26 | 41 | 20.5 | — |
| d=7 | 69–101 | 33 | 54 | 27 | — |

The near-zero margin at step 23 (Bar₂₃ ≈ 10.4965 vs ρ = 10.5) is a sharp
replication diagnostic: any discrepancy in the ledger model shows up there
first.

**Branch S (shipped structural fallback, self-refutation diagnostic):**
resealing `[PathCon(1), Var(1)]` each step gives ν_n = n + 3, ρ_n = (n+3)/2,
with margin over the bar *growing without bound* (0.42, 0.73, 1.03, …) and
the identical clause pair selected forever. Prediction: the shipped
evaluator, run seal-and-continue, produces this echo. An infinite tail of
one syntactic shape with widening margin is not a description of anything;
observing it refutes structural counting as a continuation semantics from
inside.

**Branch D-gated at the raw surface (d ≤ 1):** best ρ = 3 < 9.076 — halt
stands. This is the Theorem 17(2) outcome in ladder form.

## 4. Engine tasks

- **T1 (seal-and-continue probe).** Extend `halting_probe.rs` with a mode
  that seals a designated step-16 survivor, appends it to the reference
  prefix, recomputes the bar from the replayed ledger, and reruns the probe
  at 17, iterating ≥ 4 steps. Output: per-step clause shapes, ν decomposition,
  margins. No interpretation in the artifact.
- **T2 (staircase calculator).** In-engine exact-rational iteration of the
  bar recursion with a pluggable ν(d) law; run for (i) P5's 5 + d²,
  (ii) the fallback 4 + |library|, (iii) the capacity-gated variant reading a
  declared capacity table. Output: jump points, plateau table, the step-23
  margin as an exact rational.
- **T3 (capacity oracle v0).** A *declared* table asserting the nonzero
  π_{d−1} exports of sealed strata (per SH-1c), wired as an admissibility
  gate for `PathCon(d)` candidates. Trust boundary must state: v0 is
  declared, not derived; a typed kernel is required to earn it.
- **T4 (blind valuation derivation — separate thread).** Derive the L1
  validity domain from the frozen record only. Verbatim prompt in
  Appendix A. Deliverable: frozen spec + freeze commit *before* reading this
  document's §3 or any T1/T2 artifact.

## 5. Falsifiers and outcome semantics

Graded in three layers; no retuning at any layer — a burnt layer burns.

- **F-SH1 (qualitative).** T4 returns a capacity-gated law and the derived
  capacity of B₁₅ gives d_max = 1 → nothing clears at 16 → SH-1 burns
  entirely and the halt branch is *strengthened* (record as Theorem 17(2)
  evidence; stop).
- **F-SH2 (qualitative).** T1 does *not* produce the branch-S echo (the
  shipped evaluator yields varied continuation structure) → SH-1a's
  self-refutation diagnostic fails; the structural-counting continuation is
  a live rival and must be interpreted, not dismissed.
- **F-SH3 (quantitative).** With the T4 law substituted into T2, the
  recomputed staircase disagrees with the engine's T1 trajectory at any
  sealed step → the ledger model is falsified even if the qualitative
  staircase survives.
- **F-SH4 (structural).** The capacity oracle, once earned (typed kernel),
  finds π₃ trivial over the sealed strata as the grammar exports them →
  the theta rung is dead regardless of arithmetic.
- **F-SH5 (physical).** D2 resolves θ_QCD *internally* over B₁₅ → the
  first-rung physical interpretation is dead; conversely, if D2 finds θ
  demands winding-sector structure not derivable in B₁₅, that is positive
  evidence for continuation at d = 4. **D2 is hereby promoted from pending
  item to branch discriminator.**

Support semantics: SH-1 is *supported* only if (i) T4's blind law licenses
d ≥ 4 at step 16, (ii) T1/T2 reproduce the recomputed staircase, and
(iii) F-SH4/F-SH5 do not fire. Anything less is recorded as unanchored or
burnt, verbatim.

## 6. Relation to the registered record

- Theorem 12 / Corollary 16 are premises here and survive every outcome.
- SH-1 does not contest the EGP result (typed marginal families 3/0/3/1);
  it contests whether EGP's zero-credit for *licensed* higher cells is the
  honest law — which is precisely T4's question, not this document's.
- If SH-1 burns by F-SH1, the halt claim gains its strongest support to
  date, and the correct book statement becomes: the laws finish at 15, and
  the arithmetic forecloses purchase as well as obligation.

---

## Appendix A: verbatim prompt for the T4 blind thread

Copy exactly the text between the markers into a fresh context. Do not
paste anything else from this document or this conversation.

```
BEGIN T4 PROMPT
Task: derive the validity domain of the L1 path-credit rule.

From the frozen record only — the two-law axioms as frozen,
EVALUATOR_DERIVATION.md (D1–D3, P6), the L1/L2 lemma notes
(docs/LEMMA_L1_D_SQUARED.md, docs/LEMMA_L2_R_SQUARED.md), and the
Guard-Rail theorem note — derive:

1. Whether the credit ν_H = 1 + d² for a d-dimensional path constructor
   is honest as stated, or holds only under a precondition. If a
   precondition, state it exactly and justify it from the frozen axioms.
2. The law-level credit assigned when the precondition fails.
3. The credit's dependence, if any, on how the attachment relates to
   already-sealed structure. Be exact about what must exist in the sealed
   library for a d-cell to earn its credit.

Constraints: do not read docs/CERTIFIED_HALT_15.md, docs/t1_result.md,
docs/staircase_hypothesis.md, docs/certified_halt_verification.json, or
any file describing Step-16 candidates or survivors. Do not consult the
bar value for any purpose. Genesis 1–15 conservativity (Σν = 359, Σκ = 64
under replay) is the regression gate: your law must reproduce the fifteen
historical scores.

Deliverable: a frozen spec (markdown) stating the law, its precondition,
its failure value, and the conservativity check, committed before reading
any Step-16 material.
END T4 PROMPT
```
