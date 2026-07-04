# The Constants Program: What the Framework Can and Cannot Calculate

**Status:** program stated, first result identified, discipline pre-registered
(2026-07-02). Companion to `UNIVERSALITY_PROGRAM.md` and the ch14
slots-versus-values refiling.

---

## 1. The Criticism, Steelmanned

*"Predicting the absence of new particles is a safe bet given the LHC status
quo. It conveniently masks the theory's inability to calculate actual runtime
values — the electron mass, coupling constants, cosmological parameters —
which it files away as un-derivable."*

This is the strongest remaining empirical criticism, and its core is fair:
negative predictions are shared with plain conservatism, and a refiling
("values are runtime facts") could in principle be an excuse rather than an
insight. Two things blunt it but do not dissolve it: the refiling is
*principled* (the slot count is a theorem of the derived grammar; the
categories are argued, not ad hoc), and "runtime" does not mean "underivable
in principle" — it means "not fixed by the grammar," leaving open dynamical,
attractor, or boundary-condition derivations (the Higgs-criticality reading is
exactly such a candidate). But the only full answer is a number. This
document is the program for producing one.

## 2. The Reframe That Comes First (already earned)

The criticism says "the theory derives no constants of nature." That is false
as stated, and the response should begin by saying so precisely: the framework
derives the **integer** constants of nature —

| Constant | Value | Derivation |
| --- | --- | --- |
| Spacetime dimension | 3+1 | ch13: pincer (Ehrenfest/Weyl floor, associativity ceiling, Hodge dividend) |
| Time directions | 1 | signature trichotomy (modal collapse / ultrahyperbolic / Lorentzian) |
| Number of colors | 3 | ladder + anomaly cancellation (charge thirds ↔ N_c = 3) |
| Number of generations | 3 | CP floor: ½(N−1)(N−2) ≥ 1 |
| Gauge group factors | U(1)×SU(2)×SU(3) | division-algebra rungs; primeness keeps the product |
| θ_QCD | 0 | declined clause (conjectured) |
| Free-parameter count | 19 (26) | slot count = dimension of the value-space the grammar leaves open |

Integer constants are constants. They are, moreover, exactly the constants a
*structural* theory should fix, on the framework's own division: grammar fixes
what is countable; runtime fills in the continuum. The criticism, correctly
narrowed, is: *derive one non-integer dimensionless constant.* (Dimensionless
is non-negotiable — dimensional constants like G, c, ħ are unit conventions,
a point as old as Planck.)

## 3. Tier 1 — Available Now: the Weak Mixing Angle's Boundary Value

**Result (derivable today, one bridge principle short of a theorem):** over
the matter content that ch14's anomaly argument fixes (charges Q = T₃ + Y
with Y ∈ {1/6, 2/3, −1/3, −1/2, −1} per generation), the charge traces are

    Tr T₃² = 2,   Tr Y² = 10/3,   Tr Q² = 16/3   (per generation)

giving the canonical normalization Tr Y²/Tr T₃² = **5/3** and

    sin²θ_W(structural) = Tr T₃² / Tr Q² = **3/8**.

This is the celebrated grand-unification boundary value — but note what the
computation used: *only* the representation content and hypercharges that the
framework already derives from coherence (anomaly freedom + Yukawa
existence). No SU(5), no unification hypothesis. The 3/8 is a property of the
derived matter alphabet itself, exposed whenever the two electroweak rungs
are normalized on equal footing.

**The bridge principle (the honest open half):** "normalized on equal
footing" must be earned, not assumed. The framework's natural version: at the
structural scale — where the rungs' shells are sealed under one ledger
normalization (Observation B's single counting gauge) — trace-normalized
coupling strengths coincide. Deriving *which physical scale* that is, is the
open problem, and everything sharp depends on it:

- **If the scale is derived** (candidate: the ignition/firewall scale), the
  prediction becomes crisp, because the framework's own empty-desert
  prediction makes the running calculation clean: pure SM running, no
  thresholds. Then honesty bites: non-SUSY running of 3/8 from ~10¹⁶ GeV
  gives sin²θ_W(M_Z) ≈ 0.21 versus the measured 0.2312 — the same
  percent-level failure that killed minimal SU(5). The framework would then
  need the discrepancy to come out of its own UV story (the asymptotic-safety
  gravitational dressing near the boundary) or stand falsified at the percent
  level. **That risk should be embraced, not hidden: it is what makes this a
  prediction rather than a retrodiction.**
- **If no scale is derivable**, the claim retreats to: the *boundary form*
  3/8 and the normalization 5/3 are theorems of the derived content — a
  genuine derived non-integer constant, but at an unspecified scale (worth
  publishing as such, clearly bounded).

**Work items:** (a) state the equal-trace bridge principle within the Schema
Calculus (candidate: equal ν-per-schema weighting across sealed gauge shells);
(b) attempt a structural derivation of the matching scale from the firewall;
(c) run the desert RG (textbook) and publish the number with its error
budget, whatever it is.

## 4. Tier 2 — The Framework's Fingerprint: φ in the Sky

The most *distinctive* target is not a single number but a functional form
with a zero-free-parameter frequency. The framework's deepest quantitative
signature is the golden ratio: Φₙ → φ is forced by the depth-2 window, and
discrete self-similar growth with ratio φ generically imprints
**log-periodic structure with period ln φ ≈ 0.4812**.

Two committed exposure points:

1. **Dark energy:** ch16's horizon-saturation mechanism owes w_eff(z). If the
   saturation response inherits the bar's discrete φ-scaling, the deviation
   w(z) + 1 should carry log-periodic modulation in ln(1+z) with angular
   frequency 2π/ln φ ≈ **13.06** — the period is *forced*, only amplitude and
   phase are fit. DESI DR2/DR3 and Euclid can test a two-parameter template
   against a three-parameter generic (w₀, wₐ) one. A detected modulation at
   the predicted frequency would be the framework's single most convincing
   quantitative result; a clean w₀wₐ fit with no room for the modulation
   bounds the amplitude and weakens the mechanism.
2. **Ringdown comb:** the already-registered log-periodic comb at spacing
   ln φ in black-hole ringdown residuals (LIGO/Virgo/KAGRA now; LISA later).

Same fingerprint, two independent instruments. Deriving the w_eff(z) template
from the saturation model (bar dynamics → horizon-growth equation) is the
concrete work item, already named as the companion paper's acceptance test.

## 5. Tier 3 — Moonshots (named so ambition has an address)

- **The Koide relation.** The charged-lepton masses satisfy
  (mₑ+m_μ+m_τ)/(√mₑ+√m_μ+√m_τ)² = 2/3 to five decimal places — an exact-looking
  dimensionless fact with no accepted explanation. If Yukawas are runtime
  eigenvalues, a minimal-overshoot constraint on the three-generation mass
  matrix (democratic base + minimal hierarchy-generating deformation) is the
  kind of structure that could yield 2/3 exactly. High risk; sensational if it
  lands; harmless as a stated target.
- **Λ's magnitude in Planck units (10⁻¹²²).** The saturation reading makes Λ a
  residual indexed to the structure-formation epoch, which converts the
  "why so small" into "why now" — already dissolved qualitatively (trigger,
  not tuning). A quantitative residual estimate (saturation imbalance ×
  horizon count) is conceivable but currently underdetermined.
- **The baryon asymmetry** (η ~ 6×10⁻¹⁰) via the CP floor: the framework fixes
  that CP violation *exists* (N=3); the magnitude is a runtime integral.
  Unlikely to be structural. Listed to be honest about not attempting it.

## 6. The Numerology Firewall (pre-registered discipline)

This program sits one careless step from Eddington's grave — the great
astrophysicist spent his last years "deriving" 1/α = 136 from structural
counting, then, when measurement said 137, found a reason for the extra 1
(and is remembered for it as "Sir Arthur Adding-One"). The ledger is full of
crisp numbers (φ, 103/8, Σκ = 64, 1060 bits), and the temptation to pattern-
match them onto physical constants must be blocked by rule:

1. **Bridge principle first.** No ledger quantity may be identified with a
   physical observable without a stated, independently motivated bridge
   principle, written down *before* the numerical comparison (the equal-trace
   normalization of §3 and the φ-scaling inheritance of §4 are the two
   currently licensed bridges).
2. **Running is not optional.** Continuous couplings run; any "derivation" of
   a low-energy value that ignores renormalization is wrong on arrival. The
   framework may only supply boundary data (a value at a scale, or a
   scale-free functional form); standard physics does the rest.
3. **Fine-structure abstinence.** No attempt on 1/137 except as the RG image
   of independently derived boundary data. The constant that ruined the most
   careful numerologist of the twentieth century is not this program's entry
   point.
4. **Pre-registration.** Each attempt gets a dated target statement
   (quantity, bridge, predicted value or form, error budget, falsifier)
   before the fit — the same convention as every other program in this
   repository.

## 7. Summary for the Critic

The framework has derived the integer constants of nature and the count of
the continuous ones; it presently derives one continuous boundary value
(sin²θ_W = 3/8, from its own matter content, no unification assumed) modulo
one stated bridge principle; it owes, and has pre-registered, a
zero-free-parameter functional form (φ-log-periodic w(z), plus the ringdown
comb) that living experiments will grade this decade; and it has named its
moonshots and bound itself to a numerology discipline with teeth. That is not
a completed calculation of the electron mass. It is also not "filing
everything away": it is a falsifiable program with its first result already
on the table and its failure modes stated in advance.
