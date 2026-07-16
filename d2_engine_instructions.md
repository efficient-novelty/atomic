# Engine Instructions: The ε-Closure Project

**Filename note.** Filed as `d2_engine_instructions.md` per request; in the book's
forward program (`pen/book/appendices/app_o_predictions_program.tex`) this target
is listed as **D1: the ε-closure lemma**. Same project.

**Date:** 2026-07-16 (v2; supersedes the 07-12 draft after pre-freeze analysis
found a scoring vulnerability and a stronger derivation route — see §6).
**Status:** specification for freeze. **Upstream records:**
`pen/book/note_weinberg_matching.md` §8, `docs/CONSTANTS_PROGRAM.md` §3 addendum,
`pen/book/chapters/ch_open_problems.tex` Problem Four.
**Protocol:** freeze before evaluation, one blind run, misses reportable, no
retuning. Every numerical identity below was verified in exact arithmetic
(sympy, 2026-07-16); the engine re-derives all of them from scratch as gates.

---

## 1. Mission, in engine terms only

Small typed records ("charge assignments"): rational numbers attached to named
slots, subject to exact polynomial constraints. Three theorems to establish by
construction in exact arithmetic, one canonicalization, one scoring pass, one
post-run characterization. No floats anywhere. No measured physics values
anywhere upstream of the verdict (§9).

You do not need to know any physics. §4 defines every object as data and every
constraint as a polynomial. §2 tells you why the project exists, in one page.

## 2. Why this project exists

The book derives a claim of the form "the matter content forces the number 3/8"
(a normalization of a coupling). An external adversarial review (July 2026)
observed that if the full matter content — including a slot called ν^c that the
framework independently requires — is charged *all at once*, the constraint
system admits a one-parameter family of solutions (parameter ε), and the claimed
derivation silently assumed ε = 0. Pre-freeze analysis (this document, v2) then
found the situation is sharper in both directions:

- **Sharper FOR the framework:** its own architecture is *sequential*: the
  original matter content is sealed first (call it Stage A), and the ν^c slot
  arrives later as an extension (Stage B). Theorem A below shows the Stage-A
  system has a *unique* solution up to rescaling — no ε exists at Stage A. And
  under the framework's monotone-growth rule (sealed values cannot be rewritten),
  Theorem B shows the extension's only new value is forced to zero by a single
  linear gate. On the sequential reading, the gap closes in two lines.
- **Sharper AGAINST naive repair:** if one instead re-derives everything
  simultaneously and asks the evaluator to pick ε by clause-minimality, the
  family contains a booby trap: the point ε = −1/2, at which *fewer* slots are
  charged than at ε = 0 (5 vs 6). A least-clause scorer selects the trap, not
  the target. So the "PEN enumeration picks ε = 0 by counting" argument, as
  registered on 07-12, is **wrong as stated**, and this project's blind outcomes
  have been re-registered accordingly (§8) before any code is frozen.

The engine's job is therefore: prove Theorems A and B (the sequential closure),
prove the family-completeness theorem (the honest record of what the
simultaneous reading admits), canonicalize the family under the presentation
quotient, and *report* — not resolve by fiat — what the existing clause
semantics does on the simultaneous reading, T₃R trap and all.

## 3. Glossary (all you need)

- **Slot:** a named record with tags (triality t ∈ {0,+1,−1}, duality d ∈ {0,1},
  multiplicity m) and one unknown rational y (its "hypercharge").
- **Assignment:** a choice of y for every slot.
- **Gates:** the polynomial constraints of §4.2–4.3. Assignments violating a
  gate are inadmissible, full stop.
- **Sterile slot:** one with y = 0 (it has no interface to the U(1) structure;
  in clause terms, nothing to specify).
- **Sealing / monotone growth:** the framework's core rule that once a value is
  integrated into history it cannot be rewritten by later extensions.
- **ε:** the family parameter of the simultaneous solution space (§4.5).
- **Post-run invariants:** derived numbers/groups computed from the winning
  assignment *after* the verdict (§10). Never inputs.

## 4. The mathematics (complete, self-contained)

### 4.1 Slot tables

**Stage-A content** (the originally sealed matter; one generation):

| slot | t | d | multiplicity m | unknown |
|------|---|---|:---:|---|
| Q    | +1 | 1 | 6 | y_Q |
| u^c  | −1 | 0 | 3 | y_u |
| d^c  | −1 | 0 | 3 | y_d |
| L    | 0 | 1 | 2 | y_L |
| e^c  | 0 | 0 | 1 | y_e |
| H (scalar) | 0 | 1 | 2 | y_H |

**Stage-B extension:** one more slot, ν^c (t=0, d=0, m=1, unknown y_ν), plus one
more gate (the "neutrino Yukawa" line in §4.3).

### 4.2 Anomaly gates (fermion slots only; H exempt)

```
A331 :  2·y_Q + y_u + y_d                          = 0
A221 :  3·y_Q + y_L                                = 0
Agrav:  Σ m_i·y_i   over fermion slots             = 0
A111 :  Σ m_i·y_i³  over fermion slots             = 0
```

Two further conditions are automatic for these fixed slot lists (triality
balance +1,−1,−1; doublet count even: 3+1 with H exempt... assert: fermion
doublet count is 3 at Stage A — Q counts 3 by color — and 3 at Stage B, odd;
the actual "Witten" condition counts doublets including color multiplicity:
3(Q) + 1(L) = 4, even). Encode both as fixed assertions with the multiplicities
spelled out; they contain no unknowns.

### 4.3 Yukawa gates (standard two-component convention)

Stage A (three lines):

```
up:        y_Q + y_u + y_H = 0        (term Q·u^c·H)
down:      y_Q + y_d − y_H = 0        (term Q·d^c·H~)
electron:  y_L + y_e − y_H = 0        (term L·e^c·H~)
```

Stage B adds:

```
neutrino:  y_L + y_ν + y_H = 0        (term L·ν^c·H)
```

(H~ denotes the conjugate of H, hypercharge −y_H; which lines carry the tilde
is convention, and the M3 move of §5 makes the choice presentation-level.)

### 4.4 Theorem A (Stage-A uniqueness) — verified, engine must re-prove

Solve the three Stage-A Yukawa gates: y_u = −y_H−y_Q, y_d = y_H−y_Q,
y_e = y_H−y_L. Then over the Stage-A slots:

- A331 vanishes identically;
- A221 forces y_L = −3·y_Q;
- Agrav reduces to **y_H − 3·y_Q = 0**, forcing y_H = 3·y_Q  ← the key line;
- after substituting y_H = 3y_Q, A111 is the **zero polynomial** in y_Q.

**Theorem A.** The Stage-A system's solution space is one-dimensional: the
single ray through (y_Q, y_u, y_d, y_L, y_e, y_H) = (1/6, −2/3, 1/3, −1/2, 1, 1/2),
unique up to global rescaling (which is pure convention, move M1). There is no
ε at Stage A.

This *strengthens* the book's original claim for its own content and is a
publishable output of the run on its own.

### 4.5 Theorem B (sealed extension forcing) — verified, engine must re-prove

Fix the Stage-A values (any point on the ray; use the displayed one). Add ν^c
with unknown y_ν and the neutrino Yukawa gate. Then:

- Yukawa gate: y_L + y_ν + y_H = −1/2 + y_ν + 1/2 = y_ν = 0. **One line.**
- Agrav over extended content: (sealed sum = 0) + y_ν = 0 → y_ν = 0.
- A111: y_ν³ = 0 → y_ν = 0. (Three independent forcings; any one suffices.)

**Theorem B.** Under sealing, the extension's unique admissible charge is
y_ν = 0: the new slot is sterile, exactly. The "ε-family" is unreachable
without rewriting sealed values, which monotone growth forbids.

### 4.6 Theorem C (simultaneous family completeness) — verified, engine must re-prove

If instead all seven slots are charged at once (Stage A + B gates together,
nothing sealed), solve the four Yukawa gates, then:

- A331 vanishes identically; A221 forces y_L = −3y_Q;
- Agrav and A111 then vanish **identically** in the two remaining unknowns
  (y_Q, y_H). (The Agrav line that forced y_H = 3y_Q at Stage A is cancelled by
  the ν^c contribution: this is precisely how the extension opens the family.)

**Theorem C.** The simultaneous solution space is the 2-parameter family with
basis points **Y** = (y_Q, y_H) = (1/6, 1/2) and **B−L** = (1/3, 0); general
member α·(Y + ε·(B−L)), α ∈ ℚ*, ε ∈ ℚ ∪ {∞}. Rank check: 7 unknowns − 4 Yukawa
− 1 (A221) = 2. Nothing outside the family solves the system.

Charges of the general member (α gauge-fixed to 1 by M1):

```
y_Q = 1/6 + ε/3      y_u = −2/3 − ε/3     y_d = 1/3 − ε/3
y_L = −1/2 − ε       y_e = 1 + ε          y_ν = ε·(−1)·...   — compute: y_ν = −ε... 
```

Do NOT copy that last line: derive all seven from the substitutions; the
sign conventions must come out of your own algebra. (For the record the
verified derived charges give per-component electric charges
q_ν = −ε, q_e = −1−ε, q_u = 2/3+ε/3, q_d = −1/3+ε/3; but q's are post-run
quantities, §10.)

### 4.7 The κ-stratification of the family (verified; the trap lives here)

A slot decharges (y = 0) at exactly one ε each:

```
ν^c at ε = 0      e^c at ε = −1      d^c at ε = 1
u^c at ε = −2     Q AND L at ε = −1/2      (H at no finite ε in this gauge)
```

Charged-slot counts: generic ε → 7; ε ∈ {0, −1, 1, −2} → 6; **ε = −1/2 → 5**
(both Q and L decharge simultaneously since y_L = −3y_Q). The ε = −1/2 point is
the involution fixed point of §5 and corresponds to a known symmetric structure
(the U(1) acting only on the d = 0 slots). Any scorer that rewards fewer charged
slots, fewer clauses, or shorter descriptions selects ε = −1/2 over ε = 0.
This is the registered trap: it is why the simultaneous reading, scored by
naive minimality, does not deliver the lemma, and why Theorems A + B carry the
primary weight.

## 5. The presentation quotient (frozen move list)

- **M1 — global rescale:** y → λ·y on every slot, λ ∈ ℚ*. Gauge fix: y_H = 1/2
  when y_H ≠ 0; the y_H = 0 branch (pure B−L, "ε = ∞") is a separate branch —
  never silently dropped.
- **M2 — global conjugation:** y → −y on every slot, with doublet component
  labels swapped (T₃ → −T₃).
- **M3 — conjugate-Higgs relabel:** y_H → −y_H together with swapping the
  Yukawa roles (u^c ↔ d^c) and (e^c ↔ ν^c).
- **M4 — vev-component relabel:** which doublet component is "upper" is
  presentation.
- **M5 — same-record slot permutations** (identical t, d, y, Yukawa role after
  M3 bookkeeping).

**Expected theorem (verify as blind outcome B2):** M2∘M3 acts on the family as
**ε ↦ −1−ε** (in coordinates: (1/6+ε/3, 1/2) → (−1/6−ε/3, −1/2) → (−1/6−ε/3, 1/2),
and −1/6−ε/3 = 1/6+ε′/3 gives ε′ = −1−ε). Equivalence classes {ε, −1−ε};
canonical representative ε ≥ −1/2; fixed point ε = −1/2. Corollaries the
canonicalizer must exhibit: ε = −1 ≡ ε = 0 (resolving the review's flagged
loose end — the two points that both reproduce the ε = 0 check values are the
same candidate), and ε = 1 ≡ ε = −2 (the two quark-singlet-sterile points).
After the quotient, the κ = 6 stratum has exactly TWO classes ({0 ≡ −1} and
{1 ≡ −2}) and the κ = 5 stratum one ({−1/2}).

## 6. What changed from the 07-12 draft (pre-freeze disclosure)

The 07-12 draft registered "the evaluator selects ε = 0 by least κ at equal ν"
as the primary blind outcome. Pre-freeze verification found the κ-stratification
of §4.7: ε = −1/2 undercuts ε = 0 on every naive minimality count (5 charged
slots vs 6; shorter description; more sterile structure). The draft's primary
outcome would have failed its own run for a reason discovered before freezing —
so, per the formalization-error provision (which applies before freeze, freely),
the project is re-architected: the sequential derivation (Theorems A + B) is
primary and carries the lemma; the simultaneous scoring pass is retained as a
*diagnostic*, expected to expose the trap rather than dodge it. Nothing about
this re-architecture uses observed physics values; it uses only the algebra of
§4, which the engine re-derives. This section must ship in the frozen document
unchanged, so the record shows the near-miss.

## 7. What the engine actually runs

- **Run 1 — Theorem A:** symbolic proof pass (exact polynomial expansion over
  ℚ; assert the identities and the forcings of §4.4; assert solution-space
  dimension 1). Emit the Stage-A ray.
- **Run 2 — Theorem B:** seal the ray's values; add ν^c; assert all three
  independent forcings of y_ν = 0. Emit the extended assignment.
- **Run 3 — Theorem C + quotient:** simultaneous system; assert family
  completeness and rank; enumerate the decharging branch points exactly;
  canonicalize under M1–M5; assert the ε ↦ −1−ε involution and the class
  structure of §5.
- **Run 4 — diagnostic scoring (simultaneous reading):** score the finite class
  list {generic, {0≡−1}, {1≡−2}, {−1/2}, {ε=∞}} with the EXISTING clause
  semantics under three frozen accountings, reported side by side, no winner
  declared by the module: (a) per-charged-slot clauses; (b) per-embedding-
  direction clause + description-length tie-break; (c) the sealed-dial reading
  (marginal ε-shift over a sealed ε=0 completion: ν = 0 at κ ≥ 1 →
  inadmissible). Expected per §4.7: (a) and (b) prefer ε = −1/2; (c) forbids
  any shift. The artifact records all three verdicts as data.

## 8. Blind outcomes (register verbatim; adjudicate after the frozen run)

- **B1:** Theorem A holds as stated (Stage-A uniqueness up to M1; the forcing
  line is Agrav → y_H = 3y_Q; A111 collapses identically).
- **B2:** Theorem C holds; the quotient acts as ε ↦ −1−ε; class structure of §5
  (in particular ε = −1 ≡ ε = 0) confirmed by the canonicalizer.
- **B3:** Theorem B holds: under sealing, y_ν = 0 is forced independently by
  the Yukawa gate, Agrav, and A111. **This is the ε-closure lemma.** Its scope
  is explicitly conditional on the sealing-order premise, which is the
  framework's own cumulative-growth axiom, not a new assumption — but the
  conditionality is declared, and the run cannot and does not test the axiom.
- **B4 (diagnostic):** the simultaneous scoring pass reproduces the trap:
  accountings (a)/(b) prefer the ε = −1/2 class, (c) blocks all shifts. If
  instead (a) or (b) prefers ε = 0, that is a *surprise finding about the
  clause semantics* — reportable, not celebratable, since it would contradict
  the §4.7 hand analysis and one of the two is then wrong.
- **B5 (post-run):** the winning assignment (Theorem B's) has the ν^c slot
  sterile; stabilizer group (§10) of order 6; check-quantity table matches the
  registered formulas at ε = 0.

**Kill conditions.** B1 or B (Theorem C) failing = the algebra registered here
is wrong — escalate immediately, both to the book side and against this
document. B3 failing = the ε-closure lemma is false even sequentially; the
book's 3/8 claim stays permanently conditional; report as a real miss, no
retuning. B4 surprising = freeze the surprise, do not "fix" either side.

## 9. Firewalls

1. The scoring and theorem modules may see: slot tags, rationals (zero-tests,
   equality-tests, arithmetic), clause counts, the move list. They may NOT see
   or compute: per-component electric charges, trace sums, sin²θ_W, stabilizer
   groups, or any measured value. Grep gate on forbidden identifiers
   (`charge`, `sin2`, `3/8` as a literal, `neutrino`, `Z6`, `stabilizer`) in
   those modules.
2. Exact rationals only (`num-rational` over `i128`; assert no overflow).
3. No new evaluator dials, weights, or thresholds in Run 4; existing semantics
   only, or a reported inability.
4. Every branch (ε = ∞ included) appears in the artifact with a verdict row.
5. One frozen run; misses and surprises ship as-is.

## 10. Post-run module (physics-facing outputs; computed last, never imported)

From the Run-2 winner: per-component electric charges q = T₃ + y (doublet
components T₃ = ±1/2, singlets 0); the four registered check formulas
(q_ν = −ε, q_e = −1−ε, q_u = 2/3+ε/3, q_d = −1/3+ε/3 — all evaluated at the
winner's ε = 0); trace sums Tr T₃² = 2, Tr Y² = 10/3, Tr Q² = 16/3 and the
normalization 3/8 = 2/(16/3); and the **stabilizer computation**: the finite
group of triples (a ∈ ℤ₃, b ∈ ℤ₂, θ ∈ ℚ/ℤ) with a·t/3 + b·d/2 + θ·(y/y_unit) ≡ 0
(mod 1) on every slot (y_unit = the lattice normalizer: smallest positive λ
with all y/λ ∈ ℤ). Expected order 6, cyclic; the per-slot integrality witness
at the winner is t/3 + d/2 + y ∈ ℤ with values (1, −1, 0, 0, 1, 0, 1) for
(Q, u^c, d^c, L, e^c, ν^c, H). Also emit the same computation for the ε = −1/2
class, for the record (its lattice differs) — this is characterization, not
scoring.

## 11. Artifacts and deliverables

- `crates/pen-eval/src/epsilon_closure.rs` (pattern: `lambda_trigger_v2.rs`;
  theorems as `#[test]`s + one artifact emitter), or sibling module.
- `docs/epsilon_closure_run.json`: `{stageA: {forcing_line, ray, identities},
  stageB: {forcings, winner}, simultaneous: {family_basis, rank, branch_points,
  classes, involution}, scoring_diagnostic: {accounting_a, accounting_b,
  accounting_c}, post_run: {q_table, traces, normalization, stabilizer_winner,
  stabilizer_minus_half}}`.
- `docs/epsilon_closure_results.md`: B1–B5 adjudication in the established
  results format, misses and surprises included.
- One-line status handback for `pen/book/note_weinberg_matching.md` §8 with
  artifact hash.

## 12. Checklist

- [ ] §4 algebra as exact-arithmetic tests (Theorems A, B, C; branch points;
      automatic-condition assertions with multiplicities)
- [ ] Canonicalizer + quotient tests (involution, idempotence, ε=−1 ≡ ε=0,
      ε=1 ≡ ε=−2, fixed point −1/2)
- [ ] Diagnostic scorer, three accountings, purity-gated
- [ ] Freeze commit including this document unmodified (§6 intact)
- [ ] Single blind run → artifacts; B1–B5 adjudication; handback line
