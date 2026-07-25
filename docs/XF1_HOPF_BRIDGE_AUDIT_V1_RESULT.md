# XF-1 Hopf-Bridge Audit v1 — Result

**Campaign:** XF-1, the External Falsifier Campaign (`docs/XF_LEDGER.md`).
**Date:** 2026-07-25.
**Subject:** *The Complex Hopf Fibration as the Canonical Space for
Gauge–Gravity Unification*, SSRN 6975959, 18 June 2026
(`docs/ssrn-6975959.pdf`, 78 pp.).
**Commission:** a frozen audit of the candidate Hopf-spectral bridge model on
three tests — (T1) the exact Koide cross-check, (T2) the S⁹ minimality census,
(T3) a complete four-dimensional EFT matching calculation — to decide whether
the paper supplies the missing parameter layer or gives a numerically
impressive but underdetermined realization.

**Reproduction:** `scripts/xf1_hopf_bridge_audit/` — five self-contained Python
scripts (`a1`–`a5`), mpmath at 40–60 digits, no empirical input except where
explicitly labelled. Every number below is computed, none is quoted.

**Revision:** body = v1. A **v1.1 addendum** is appended at the end, recording
findings from a 36-agent adversarial verification pass that ran after the body
was frozen. Three v1 findings were refuted — all on *status*, none on
arithmetic — and those corrections are folded into T2.3, T2.4 and T2.6 in
place, marked as such. No v1 verdict changed.

---

## Headline

**The paper does not supply the missing parameter layer. Two of the three tests
return decisive negatives against the paper; the third returns a decisive
positive that the paper does not itself make.**

1. **T1 — Koide: the test is vacuous as posed, and the paper's charged-lepton
   sector has zero predictive content.** The three coefficients `D(1), D(2),
   D(3)` are not derivable from the paper's own Eqs. (63)–(65), and they are
   numerically identical — to one part in 10⁹ — to the values obtained by
   *inverting* the mass formula on the measured lepton masses. Consequently
   `Q_Hopf − Q_exp = −9.9 × 10⁻¹¹`. The advertised "−9.23 ppm near-miss of
   2/3" is not two mechanisms nearly meeting; it is the experimental Koide
   deviation, restated. There is no residual for a missing spectral correction
   to explain.

2. **T2 — the S⁹ census succeeds, verdict-blind, and yields more than S⁹.**
   Under a frozen, constant-free obligation profile the census returns
   `n★ = 4` **uniquely** (not merely as an argmin) at its strongest tier,
   with structure group `U(5)` and minimal SM-carrying subgroup
   `S(U(3)×U(2))`. That group's standard presentation is
   `(SU(3)×SU(2)×U(1))/ℤ₆` — the same group the Two-Law theory reaches by
   sequential representation sealing, so two independent routes land on one
   endpoint. The `5/3` normalization and `sin²θ_W = 3/8` follow, but they need
   obligation **O4** (`q_ν = 0`) to fix the hypercharge scale and they use the
   ambient `su(5)` trace form; they are not pure subgroup indices. **This is
   the Two-Law theory's derivation, not the paper's** — and the gap is worse
   than a bad argument for a right answer. The paper's route to S⁹ is
   `Spin(10)`-framed (wrong covariance group for a U(1)-winding decomposition)
   and circular against α; and its **nested** shell tower puts `SU(2)` *inside*
   `SU(3)` (Thm 6: `S⁵ ≅ SU(3)/SU(2)`), so it never contains `SU(3) × SU(2)` as
   a product at all — verified commutant dimension 5 and non-abelian, versus 1
   for the census's block embedding. The two constructions share a sphere and
   nothing else.

3. **T3 — the EFT matching has no solution, and the factor-3.7 anomaly is a
   placement artifact.** The paper's `g` and `g′` cannot be MS-bar couplings at
   any single scale: they require scales **5.08 decades apart** (45.4 TeV and
   0.382 GeV) because they must run in opposite directions. Read correctly,
   the pair `(α⁻¹ = 137.036, sin²θ_W = 3/4π)` is the *measured Q²→0 pair* and
   carries no information beyond it. Redone with correct placement, the
   `g₁ = g₂` crossing sits at **1.031 × 10¹³ GeV** (one loop) and
   **1.089 × 10¹³ GeV** (two loop), reproducing the Two-Law audit's
   `1.1 × 10¹³ GeV` to under 1 %. The commissioning factor 3.7 resolves
   exactly: `3.9599 = 2.70014 × 1.46655` — reference-scale shift times
   coupling offset. Nothing physical was being measured by it.

A fourth result, not commissioned but decisive for the merged package: the
**S⁹ neutrino sector is the paper's one genuinely parameter-free prediction,
and it survives verification of every coefficient except one — which is
discarded illegally, and the discarding is load-bearing at 21σ.**

---

## What was frozen before any computation

- The **obligation profile** for T2 (O1–O8 below), stated in structural terms
  only, containing no measured constant. Frozen before the paper's S⁹ sections
  were read.
- The **inputs** for T1: the paper's own symbolic expressions for `a`, `σ₃`,
  `τ₃`, `α`, `Λ_L`, and the printed `D(n)`, taken verbatim from Eqs. (53),
  (46)/(69), (70), Thm 46, p. 41, and Eq. (66).
- The **hazard fence** of the XF constitution: no φ, Fibonacci or ledger
  integer is used as a physical parameter anywhere in this audit; α is treated
  as out of reach and no curve is fit toward it.

---

## T1 — Exact Koide cross-check

### T1.1 The formula and its exact inputs

Eq. (62), p. 38:

```
m_n = Λ_Hopf · (n+1) · exp[ a·n − D(n) + n·α/6 + σ₃·ln τ₃(K_n) ],  n = 1,2,3
```

Computed at 60 digits from the paper's own closed forms:

| quantity | paper's closed form | computed value |
|---|---|---|
| `a` | `6√2 · exp(ζ(3)/24π²)` | `8.52845144101100416590206338175` |
| `σ₃` | `ζ(3)/(4π²)` | `0.03044845705839327078025153047` |
| `τ₃` | (unknot, Hopf link, trefoil) | `(1, 1, √3)` |
| `α` | Thm 46, sphere-volume ratio | `α⁻¹ = 137.036082448164337440176169` |
| `Λ_L` | `√(2π)·v·κ⁶` | `1.68065448092136 × 10⁻⁴ MeV` |

`α` from Thm 46 is fully symbolic. The volume ratio collapses to a closed form,
verified identical to 45 digits:

```
1/α  =  16 · 120^(1/4) · π^(11/4) / 9  =  137.036082448164337440176169169
CODATA                                  =  137.035999177
deviation                               =  +0.607659 ppm
```

This part is not in dispute — it is Wyler's constant, exactly computable, and
the paper's derivation of it is a genuine (if premise-laden) construction. Note
that "agreement to six significant figures" is a fixed pure number agreeing with
a measured one to 0.6 ppm, where the measurement's own uncertainty is far
smaller; it is a near-coincidence, not a confirmed prediction.

### T1.2 D(n) is not derivable from the paper's own equations

The paper states `D(n)` is "the quadratic-in-n piece of `−ζ′_n(0)`", extracted
from Eq. (63) `ζ_n(s) = ζ_H(s−2, n+1) − ζ_H(s, n+1)` with Eqs. (64)–(65).
Those equations are internally correct and were verified:

```
ζ′_1(0) = −ζ(3)/(4π²) + ½ln(2π) = 0.888490076146279471   ✓ matches the paper's printed 0.888490076
```

Evaluating Eq. (65) gives the object the paper says `D(n)` is extracted from:

| n | `−ζ′_n(0)` = `ln det′ B_n` | `D(n)` printed |
|---|---|---|
| 1 | `−0.888490076146` | `1.203011392` |
| 2 | `−2.96793161783` | `4.806545406` |
| 3 | `−11.7568299272` | `10.818228646` |

`−ζ′_n(0)` is **negative and monotone decreasing**; `D(n)` is **positive and
increasing**. No stated operation maps one to the other. The only closed form
the paper itself offers is Eq. (61), `D(n) = ζ(3)n²`, and the printed values
are not that either:

| n | `ζ(3)n²` | `D(n)` printed | residual |
|---|---|---|---|
| 1 | `1.20205690316` | `1.203011392` | `+9.54489 × 10⁻⁴` |
| 2 | `4.80822761264` | `4.806545406` | `−1.68221 × 10⁻³` |
| 3 | `10.8185121284` | `10.818228646` | `−2.83482 × 10⁻⁴` |

The residuals have no common sign and no monotone pattern. **`D(n)` is stated,
not derived.** Status: **FITTED**, pending the inverse test below.

### T1.3 The inverse problem — D(n) *is* the measured masses

Solving Eq. (62) for `D(n)` given the measured masses and the paper's own
derived `Λ_L, a, α, σ₃`:

```
D(n) = ln Λ_L + ln(n+1) + a·n + n·α/6 + σ₃·ln τ₃(K_n) − ln m_n
```

with `m_e = 0.51099895069`, `m_μ = 105.6583755`, `m_τ = 1776.86 MeV`:

| n | `D_required(n)` | `D_printed(n)` | difference |
|---|---|---|---|
| 1 | `1.20304550604` | `1.203011392` | `+3.41140 × 10⁻⁵` |
| 2 | `4.80657952079` | `4.806545406` | `+3.41148 × 10⁻⁵` |
| 3 | `10.8182627597` | `10.818228646` | `+3.41137 × 10⁻⁵` |

**The difference is one constant, to six significant figures, across all three
n.** A constant shift in `D` is exactly degenerate with a rescaling of
`Λ_Hopf`. Removing that degeneracy by working with differences only:

| | required | printed | difference |
|---|---|---|---|
| `D(2) − D(1)` | `3.60353401475` | `3.603534014` | `7.5 × 10⁻¹⁰` |
| `D(3) − D(1)` | `9.61521725370` | `9.615217254` | `−3.0 × 10⁻¹⁰` |
| `D(3) − D(2)` | `6.01168323895` | `6.011683240` | `−1.1 × 10⁻⁹` |

**The printed `D(n)` reproduce the measured lepton mass ratios to one part in
10⁹.** They are the measured masses, re-expressed. The residual constant
`3.4114 × 10⁻⁵` is the 34 ppm by which the paper's derived `Λ_L` misses the
scale that would make `m_e` exact — which is why the paper's own predicted
`m_e = 0.511016383` MeV sits 34 ppm above the measured value.

Parameter accounting: Eq. (62) contains `{Λ_Hopf, D(1), D(2), D(3)}` with one
exact degeneracy, hence **3 free quantities for 3 masses.** Exactly saturated.
**Residual predictive content of the charged-lepton sector: zero.**

`D(3)` as printed encodes `m_τ = 1776.86 MeV` specifically. Feeding any other
τ mass into the inverse breaks the constant-offset pattern — e.g. at
`m_τ = 1776.93` the offsets become `(+3.4114, +3.4115, −0.528) × 10⁻⁵`, no
longer constant. The coefficient is pinned to one PDG edition.

### T1.4 The Koide values

Scale-free, so `Λ_Hopf` cancels:

```
Q_Hopf = 0.666660511366486377811738499908727898392
2/3    = 0.666666666666666666666666666666666666667
Q_Hopf − 2/3 = −6.15530018 × 10⁻⁶  =  −9.23295 ppm
```

This reproduces the commissioning estimate (`0.6666605114`, `−9.23 ppm`)
exactly. But:

```
Q_exp (m_τ = 1776.86) = 0.666660511465521990306145728539
Q_Hopf − Q_exp        = −9.90356 × 10⁻¹¹   =  −1.5 × 10⁻⁴ ppm
```

**`Q_Hopf` equals `Q_exp` to one part in 10¹⁰.** The −9.23 ppm is the
experimental Koide deviation and nothing else. The proposed "joint test of two
frameworks meeting on a scale-free observable" has only one framework in it:
the data appears on both sides.

### T1.5 The `a_Koide − a_Hopf` residual is a gauge artifact

Holding the printed `D` fixed, the `a` that forces `Q = 2/3` exactly is

```
a_Koide = 8.52850785862198374309347779451
a_Hopf  = 8.52845144101100416590206338175
difference = 5.6417611 × 10⁻⁵  = 6.61522 ppm
```

confirming the commissioning arithmetic. **But this quantity is not physical.**
In the exponent `a` appears as `a·n` and `D` as `−D(n)`; the shift
`a → a + δ`, `D(n) → D(n) + n·δ` leaves the spectrum exactly invariant. Since
`D(n)` is not independently determined, `a` is not independently testable, and
the 6.62 ppm is a reparameterization, not a residual awaiting explanation.

The paper's Remark 23 (p. 37) states that "a constant **and a linear-in-n
piece**" of `−ζ′_n(0)` are "absorbed into the overall scale `Λ_Hopf`". An
n-independent scale cannot absorb a linear-in-n term. **ERROR as written**, and
it is exactly the degeneracy above.

For the record, the exact-Koide τ roots from the measured `m_e, m_μ`:

```
high root = 1776.96902729 MeV        (Two-Law tex: 1776.969027 — agrees)
low  root =    3.31735654442 MeV     (Two-Law tex: 3.3173565 — agrees)
paper's m_τ = 1776.86 MeV;  high root − paper = 0.109027 MeV
```

### T1 verdict

> The exact Koide cross-check **cannot be run** against this paper, because the
> paper's charged-lepton spectrum is not an independent computation of the
> masses. It is the masses. `Q_Hopf` is `Q_exp` to 10⁻¹⁰. The Two-Law
> conditional target `Q = 2/3` is neither corroborated nor burned by this
> paper: no evidence flows in either direction. **XF-C is untouched.**

---

## T2 — S⁹ minimality census (verdict-blind)

### T2.1 The frozen obligation profile

| | obligation |
|---|---|
| O1 | a primitive central complex phase (a freely acting U(1)) |
| O2 | an SU(2) action |
| O3 | an SU(3) action |
| O4 | a neutral chiral sector |
| O5 | three-generation flavor capacity |
| O6 | four-dimensional public descent |
| O7 | no additional fundamental gauge factor beyond the alphabet |
| O8 | one common spectral/public-record extractor |

Candidates `H_n : S¹ → S^(2n+1) → CP^n`, `n = 1, 2, 3, …`.

**The load-bearing correction to the census as commissioned:** for the
obligations that reference the fibration — O1–O3 and O7 — the relevant group is
*not* the isometry group `O(2n+2)` of the round sphere but the subgroup
preserving the Hopf fibration, i.e. commuting with the free U(1) fiber action
and descending to `CP^n`, which is **`U(n+1)`**. Those obligations are then
decidable by pure representation theory on `C^(n+1)`, and this decides the
census.

*Scoping (added after adversarial review):* the correction is not that
`O(2n+2)` is always the wrong group. Statements referencing **only** the round
metric and the full coexact form bundle — for instance the paper's Corollary 5
on the uniqueness of the Beltrami operator on every shell — are legitimately
`O(2n+2)`-covariant and are untouched here. The distinction bites exactly where
the **winding decomposition** is in play, because that is `U(1)`-equivariant.

### T2.2 The census, scored at three obligation tiers

| tier | content | discharging set | verdict |
|---|---|---|---|
| A | `rank U(n+1) ≥ rank(SU(3)×SU(2)×U(1)) = 4` | `{3,4,5,6,7,8,…}` | argmin 3 |
| B | A + commuting faithful **3** and faithful **2** on `C^(n+1)`; commutant supplies exactly one U(1) | `{4, 6, 7, 8, …}` | argmin 4 |
| C | B + free sealing / no unforced generators: `C^(n+1)` decomposes into the alphabet's **defining** representations, each once, with no composite `3⊗2`, no higher SU(2) spin, no spectator singlet | **`{4}`** | **unique** |

Obstructions, verified computationally:

- `n = 1, 2` (`C²`, `C³`): a faithful **3** needs 3 dimensions and a faithful
  **2** needs 2 more; `3 + 2 = 5 > n+1`. Fails O2/O3.
- `n = 3` (`C⁴`, group `U(4)`): rank passes, but the SU(3) irreps of dimension
  ≤ 4 are `1, 3, 3̄`, so a faithful **3** forces `C⁴ = 3 ⊕ 1`, whose commutant
  is abelian by Schur (`U(1)×U(1)`). An abelian group cannot contain SU(2).
  **Fails.**
- `n = 4` (`C⁵`, group `U(5)`): `C⁵ = 3 ⊕ 2` — and this is the *only*
  decomposition with each defining representation exactly once. **Passes at
  every tier.**
- `n = 5` (`C⁶`): `3 ⊗ 2` leaves no U(1) for hypercharge; `3 ⊕ 2 ⊕ 1` supplies
  one U(1) too many. **Fails O7.**
- `n ≥ 6`: passes tier B but always via a composite `3⊗2`, a higher SU(2) spin,
  or a spectator summand. **Fails tier C.**

```
n★ = 4   →   S¹ → S⁹ → CP⁴,  structure group U(5)
```

**Honest scoping:** at tier B the answer is only an argmin over `{4,6,7,8,…}`
and a monotone cost `κ(H_n)` must be declared to select 4. At tier C it is
unique and no cost function is needed. Tier C's extra clause is exactly the
Two-Law free-sealing condition (no unforced generators), so the census is
decisive *within* the Two-Law framework and merely minimal outside it. This
should be recorded as the census's one premise.

### T2.3 What the winning candidate forces

At `n = 4`, the subgroup of `U(5)` preserving `C⁵ = 3 ⊕ 2` with unit total
determinant is `S(U(3)×U(2))`. The homomorphism

```
SU(3) × SU(2) × U(1) → S(U(3)×U(2)),   (g, h, z) ↦ (z²g, z⁻³h)
```

has `det(z²g)·det(z⁻³h) = z⁶·z⁻⁶ = 1`, and kernel
`{(z⁻²I₃, z³I₂, z) : z⁶ = 1}`, verified by brute force over the sixth roots of
unity. Hence

```
S(U(3)×U(2)) ≅ ( SU(3) × SU(2) × U(1) ) / ℤ₆
```

Confirmed independently by a center argument: `Z(S(U(3)×U(2)))` is a
**connected** circle (it is the kernel of the character `(3,2)` on `T²`, and
`gcd(3,2) = 1`), whereas `Z(SU(3)×SU(2)×U(1))` has **6** components. Both
groups have dimension 12, so dimension does not separate them — the component
count does. `S(U(3)×U(2))` is therefore *not* the direct product.

**Scoping (corrected after adversarial review — this claim was initially
overstated).** What the census forces is `C⁵ = 3 ⊕ 2` and hence the structure
subgroup `S(U(3)×U(2))`. That this group *is* `(SU(3)×SU(2)×U(1))/ℤ₆` is a
**standard fact**, not a new derivation — it is Baez–Huerta (2010), which is the
paper's own reference [16]. The correct statement is therefore:

> Two independent routes — sequential representation sealing in the Two-Law
> theory, Hopf-shell minimality here — land on the **same group**. The novelty
> is the census reaching `S(U(3)×U(2))` from shell minimality alone; the ℤ₆
> presentation of that group is textbook.

That is still a real and useful concordance. It is not a second independent
discovery of the ℤ₆, and the ledger records it at that strength.

### T2.4 The 5/3 and 3/8, as a trace identity

The embedding fixes the U(1) normalization against SU(2) through the same
invariant trace form. On the **5**:

```
Y  = (−1/3, −1/3, −1/3, 1/2, 1/2)      T₃ = (0, 0, 0, 1/2, −1/2)
Tr Y² = 5/6      Tr T₃² = 1/2      Tr Q² = 4/3
Tr Y² / Tr T₃² = 5/3
sin²θ_W = Tr T₃² / (Tr T₃² + Tr Y²) = 3/8
```

Over a full SM generation the same computation gives `Tr T₃² = 2`,
`Tr Y² = 10/3`, `Tr Q² = 16/3` — **exactly the three traces the Two-Law tex
states** (line 1566) — with the same `5/3` and `3/8`.

**Two premises are load-bearing here, and adversarial review corrected an
overstatement in the first draft of this section.**

*(i) Group theory fixes the direction of `Y`, not its scale.* The commutant of
`SU(3)×SU(2)` in `su(5)` is one-dimensional, so `Y` is forced only up to
normalization. Parametrizing `Y(y) = diag(2y, 2y, 2y, −3y, −3y)`:

```
Tr Y² = 30y²      Tr T₃² = 1/2      Tr(T₃Y) = 0 identically
κ(y)  = Tr Y²/Tr T₃² = 60y²
sin²θ_W(y) = 1/(1 + 60y²)
```

| `y` | `κ` | `sin²θ_W` |
|---|---|---|
| `1/12` | `5/12` | `12/17` |
| **`1/6`** | **`5/3`** | **`3/8`** |
| `1/3` | `20/3` | `3/23` |

`κ = 5/3` and `sin²θ_W = 3/8` hold **only** at `y = ±1/6`. What selects it is
the colour-singlet neutral chiral state — **obligation O4** — via
`q_ν = 1/2 − 3y = 0 ⇒ y = 1/6` (which simultaneously gives the coloured charge
`2y = 1/3`). **O4 is a matter-content obligation, not a subgroup index.** So the
`5/3` and the `3/8` are *not* pure group theory; they are group theory plus
`q_ν = 0`. Since O4 is in the frozen profile the census result stands, but the
attribution must be correct.

*(ii) `κ = 5/3` is meaningful only through the ambient **simple** `su(5)`.*
Rep-independence — `5 → 5/6 over 1/2`, `10 → 5/2 over 3/2`,
`15 → 35/6 over 7/2`, `24 → 25/3 over 5`, all giving `5/3` — is exactly what a
simple ambient algebra buys. If the ambient algebra is not simple, each factor
carries an independent scale and `κ` is arbitrary. So the normalization
**borrows `su(5)`** even though O7 keeps `SU(5)` out of the gauge sector.

The defensible version of "no GUT" is therefore narrower than first stated:
`SU(5)` is never *gauged* — there are no `X, Y` gauge bosons and no
GUT-mediated proton decay, consistent with both frameworks — but its trace form
is used, supplied by the shell's structure group `U(5)` rather than by a gauged
unification. That is arguably the interesting content: the *geometry* delivers
the normalization without the *dynamics*. It should be declared, not elided.

### T2.5 The paper's own S⁹ argument does not coincide with this

- The paper sets its S⁹ "framing number" `ℓ₉ = 2^(8/2) = 2⁴ = 16` from the
  **Spin(10)** chiral spinor dimension (p. 49), and Lemma 6 (p. 61) requires
  `SO(10)` invariance. *(The number 16 is fine; only the justification fails —
  see the v1.1 addendum, A5.7.)* But the fiber-winding decomposition the whole construction rests
  on is `U(1)`-equivariant, hence covariant under
  `U(5) = O(10) ∩ GL(5,ℂ)` (dim 25), **not** under `SO(10)` (dim 45). A generic
  `SO(10)` element does not commute with the free `U(1)` fiber action and
  therefore does not preserve the Hopf fibration or map winding sectors to
  winding sectors. Using `Spin(10)` representation theory to normalize a
  `U(5)`-covariant winding decomposition is a category error, and it is the one
  place where the paper's S⁹ argument would have had to reproduce the census.
- Thm 46 derives **α from S⁹**. If S⁹ were selected because it reproduces α,
  the derivation is circular. The paper offers no independent selection
  argument, so on the paper's own presentation S⁹ is a **BRIDGE PREMISE**.

### T2.6 The paper's own shell tower does not realize `SU(3) × SU(2)` at all

This is the sharpest structural finding of the audit, and it came out of
adversarial review of the census rather than of the paper.

The census uses the **block** embedding `C⁵ = 3 ⊕ 2`, in which `SU(3)` and
`SU(2)` **commute**. The paper builds its shells by the **standard nested
inclusion** `(z₁,…,z_n) ↦ (z₁,…,z_n,0)`, so `S³ ⊂ S⁵` puts `SU(2)` *inside*
`SU(3)`. Thm 6 (p. 8) says so verbatim: *"The unique compact Lie group `G`
containing `SU(2)` such that `G/SU(2) ≅ S⁵` is `G = SU(3)`."* `SU(2)` is the
**stabilizer**, not a factor.

Verified by explicit `5×5` anti-hermitian generators and commutant ranks:

| embedding | `su(3), su(2)` commute? | `dim` commutant in `u(5)` | in `su(5)` |
|---|---|---|---|
| **block** `3 ⊕ 2` (census) | **yes** | `2` | `1` — the single hypercharge |
| **nested** (paper, Thm 5/6) | **no** — `su(2) ⊂ su(3)` | `5` | — |

Under the paper's own nesting there is **no `SU(3) × SU(2)` product subgroup**,
no rank-2 abelian commutant, and hence no hypercharge direction of the required
kind; the 5-dimensional commutant is non-abelian.

**Consequence.** The census does not retroactively license the paper's
construction, as this audit's first draft concluded. The census's `S⁹` and the
paper's `S⁹` are *structurally different objects*: same sphere, incompatible
group actions. The paper's shell tower `S³ ⊂ S⁵ ⊂ S⁷ ⊂ S⁹` realizes a **nested
chain** `SU(2) ⊂ SU(3) ⊂ … ⊂ SU(5)`, which is not the Standard Model gauge
group. That is a defect in the paper independent of every numerical finding
above, and it is upstream of all of them.

### T2 verdict

> The census **succeeds and is verdict-blind**, returning `n★ = 4` uniquely at
> the free-sealing tier, together with `U(5)` and `S(U(3)×U(2))` — no measured
> constant used anywhere. **This is a genuine new Two-Law derivation**, subject
> to three declared premises (tier-C free sealing; O4 fixing the hypercharge
> scale; the ambient `su(5)` trace form). The `ℤ₆` presentation and the `5/3`
> and `3/8` that follow are correct but standard, not new discoveries.
>
> It is emphatically **not** the paper's derivation, and the relationship is
> worse than "the paper argues for the right answer badly". The paper's route to
> S⁹ is `Spin(10)`-framed — the wrong covariance group for a `U(1)`-winding
> decomposition — and circular against α. More seriously, **its nested shell
> tower puts `SU(2)` inside `SU(3)`, so it never contains `SU(3) × SU(2)` as a
> product at all** (T2.6). The census licenses a *different* use of `S⁹` than
> the one the paper builds.

---

## T3 — Complete four-dimensional EFT matching

### T3.1 The paper's own arithmetic

From the paper's `α_H` and `sin²θ_H = 3/(4π) = 0.238732414638`:

| | computed | paper prints | error |
|---|---|---|---|
| `e = √(4πα_H)` | `0.3028220288` | — | — |
| `g = e/sinθ_W` | `0.6197717396` | `0.6205` (Eq. 76) | **+0.1175 %** |
| `g′ = e/cosθ_W` | `0.3470713745` | `0.3469` (Eq. 77) | **−0.0494 %** |

Both printed values are arithmetically wrong at the quoted precision. The
commissioning calculation used the correct `0.6197717` and `0.3470714`.

### T3.2 The decisive test — there is no single matching scale

Treat the paper's `g` and `g′` as MS-bar couplings of the unbroken SM at some
common `μ₀`, anchored on the measured values at `M_Z = 91.1876 GeV`
(`α⁻¹ = 127.951`, `sin²θ_W = 0.23122` ⇒ `g₂ = 0.6517340`, `g_Y = 0.3574226`),
and solve for `μ₀` from each separately:

```
g₂ = 0.6197717  requires  μ = 4.54151 × 10⁴ GeV     (b₂ = −19/6 < 0 ⇒ μ > M_Z)
g_Y = 0.3470714 requires  μ = 3.81895 × 10⁻¹ GeV    (b_Y = +41/6 > 0 ⇒ μ < M_Z)

ratio of required scales = 1.1892 × 10⁵   =  5.075 decades
```

**No single matching scale exists.** The two couplings must run in *opposite*
directions to reach the paper's values, so no threshold prescription, no
scheme choice and no finite spectral dressing of common origin can reconcile
them. The `0.382 GeV` figure is doubly unphysical — it lies below every
electroweak threshold, where unbroken-SM running does not apply at all — which
strengthens rather than weakens the conclusion.

*Fairness note:* the paper does not itself assert that `g` and `g′` are MS-bar
couplings at a common scale; it calls them "undressed geometric-scale values"
subject to unspecified "finite spectral corrections". That framing is
unfalsifiable as written, since no `Δ_spec(Q²)` is ever given. The test above is
the natural EFT reading, stated as a hypothesis and rejected.

### T3.3 What the paper's boundary data actually is

| | paper | measured | deviation |
|---|---|---|---|
| `α⁻¹` | `137.036082448` | `137.035999177` (Q²→0) | `+0.61 ppm` |
| `sin²θ_W` | `0.238732414638` | `≈ 0.23863` (Q²→0, MS-bar) | `+0.043 %` |
| `sin²θ_W` | `0.238732414638` | `0.23122` (M_Z, MS-bar) | `+3.25 %` |

**Both numbers sit on the measured Q²→0 pair.** The paper's electroweak
"predictions" carry no information beyond the low-energy pair that has been
known for decades. This also explains the paper's own `Δ_spec(M_Z²) ≈ −0.0075`:
that is exactly the known SM running of the weak mixing angle from `Q² → 0` to
`M_Z`, re-imported under a different name and never computed.

### T3.4 The g₁ = g₂ crossing under each placement

One loop, `(b₁, b₂, b₃) = (41/10, −19/6, −7)`, one Higgs doublet:

| placement | `μ_X` | `g_X` |
|---|---|---|
| measured `M_Z` anchors — **the correct EFT completion** | `1.03089 × 10¹³ GeV` | `0.544328` |
| paper's data at `μ₀ = M_Z` | `1.51186 × 10¹³ GeV` | `0.524180` |
| paper's data at `μ₀ = v = 246.21965 GeV` (the commissioning hybrid) | `4.08224 × 10¹³ GeV` | `0.524180` |

Two loop (SM `B_ij` matrix, top Yukawa `y_t(M_Z) ≈ 0.95` evolved with the
gauge couplings), measured anchors:

```
μ_X = 1.08939 × 10¹³ GeV     g_X = 0.545701     g₃(μ_X) = 0.580413
```

**The two-loop crossing reproduces the Two-Law audit's `1.1 × 10¹³ GeV` to
better than 1 %.** The Two-Law placement audit is independently confirmed here.

*Precision caveat:* `μ_X` depends exponentially on the input couplings, so the
two-loop figure is quoted to three digits but is good to roughly ±10 % against
plausible variation in `α_s(M_Z)`, `sin²θ_W(M_Z)` and the top Yukawa
(`y_t(M_Z) ≈ 0.95` used here). The robust statements are the *order* — about
`1 × 10¹³` GeV, deep in the desert — and the *decomposition* of the
commissioning factor below, which is exact and input-independent.

Decomposition of the commissioning factor:

```
hybrid / correct = 3.9599
      of which  v / M_Z            = 2.70014     (reference-scale shift)
      of which  coupling offset    = 1.46655     (undressed vs measured)
      product                      = 3.9599      ✓
```

**The factor is entirely a placement artifact.** It is not a diagnostic of
two-loop effects, thresholds, spectral dressing, or failure of the Hopf
normalization — the four candidate explanations offered at commissioning. It is
the choice to place low-energy boundary data at `v`.

### T3.5 `sin²θ_W = 3/8` at the crossing is an identity

With `g₁² = (5/3)g_Y²`, at any `g₁ = g₂` point `g_Y² = (3/5)g₂²`, so

```
sin²θ_W = g_Y²/(g₂² + g_Y²) = (3/5)/(1 + 3/5) = 3/8
```

Verified numerically as exactly `0.375` in all three placements above. It is
therefore **independent of `μ_X`** and carries no information about the
matching scale. Both frameworks' `3/8` is the normalization identity of
T2.4, not a dynamical coincidence.

### T3.6 The strong coupling, Eq. (78)

```
α_s^geom = (dim SU(3)/dim SU(2))·(N₃/N₅)·α_eff = (8/3)·(4π²/8π³)·α_eff = (4/3π)·α_eff
prefactor 4/(3π) = 0.424413181578
```

| `α_eff` | `α_s^geom` |
|---|---|
| `α_H = 0.00729735` | `0.0030971` |
| `α(M_Z) = 1/127.951` | `0.0033170` |
| `1` (maximal defensible) | `0.4244132` |

Remark 24 claims `α_s ≈ 1` at the geometric scale, running to `0.118` at `M_Z`.
Eq. (78) would need `α_eff = 2.3562` for the first and `α_eff = 0.27780` for
the second. With any electromagnetic-strength `α_eff`, Eq. (78) gives
`α_s ≈ 3 × 10⁻³` — a factor **322.9** below the claimed geometric-scale value,
and *below* the measured `M_Z` value, so the asserted "decreasing at higher
spectral levels" behaviour has **the wrong sign as well as the wrong
magnitude**. Status: **ERROR as written.**

### T3.7 The UV-finiteness contradiction

Thm 49 (p. 63) claims exact UV finiteness because every shell action is
Gaussian and therefore has no higher loops. Thm 19 (pp. 17–18) gives the
projected four-dimensional action

```
S_4D = Σ_k ∫ [ φ†_k(□ + m²_k)φ_k + g_klm φ_k φ_l φ_m δ_{k+l+m,0} + … ]
```

with an explicit **cubic vertex**, sourced (item iii) from the
`γ α ∧ F ∧ (dα)^{n−1}` term, which is trilinear in the connection because
`F = da + a∧a`. A theory with a cubic vertex is not a Gaussian determinant. The
one-loop self-energy it generates,
`∫ d⁴k /[(k² + m²)((k+p)² + m²)]`, is logarithmically divergent in four
dimensions, and compact odd-dimensionality of the *shell* does not remove it:
compactness discretizes the spatial spectrum on `S³`, turning the loop integral
into a divergent mode sum, but the four-dimensional loop remains.
**The two theorems are incompatible as written.** This is also where the paper
collides with the Two-Law theory, which preserves the ordinary one-loop
coefficients `(41/10, −19/6, −7)` — and those coefficients are what produce the
`1.089 × 10¹³ GeV` crossing that the paper's own boundary data, correctly
placed, requires.

### T3.8 The boson sector does not reproduce the paper's own table

Eq. (71), `m_B(n) = Λ_B·(n+1)·e^{nα/6}·T_B(n)` with `r = k+2 = 8` and
`Λ_B = v·√(2/r)·sin(π/r)·e^{−2α}`, evaluated with the paper's own inputs:

| | intermediate | paper |
|---|---|---|
| `v·√(2/r)·sin(π/r)` | `47112.157 MeV` | `47112` ✓ |
| `Λ_B` | `46429.563 MeV` | `46429` ✓ |
| `T_W` | `0.8649233596` | — |
| `r_f = r + √3α/2π` | `8.002011619` | `8.002012` ✓ |
| `T_Z`, `T_H` | `0.6534371696`, `0.6721892101` | — |

Every intermediate reproduces. The masses do not:

| | Eq. (71) as written | Table 1 | offset | pull vs PDG | paper claims |
|---|---|---|---|---|---|
| `m_W` | `80413.769 MeV` | `80369.5` | `+0.05508 %` | **`+3.44σ`** | `+0.04σ` |
| `m_Z` | `91238.069 MeV` | `91187.8` | `+0.05513 %` | **`+24.0σ`** | `+0.11σ` |
| `m_H` | `125294.129 MeV` | `125225` | `+0.05520 %` | `+0.86σ` | `+0.23σ` |

The three overshoot ratios are `1.0005508, 1.0005513, 1.0005520` — **one
constant**, hence a pure `Λ_B` normalization gap. Table 1 requires
`Λ_B = 46404.003 MeV` against the `46429.563 MeV` the paper derives: an
**undisclosed rescaling by 0.99944949**.

The paper's own p. 41 text concedes this ("reproducing the observed W mass to
within 0.054 %; the residual is `O(α²)`") but Table 1 nevertheless prints
`+0.04σ`. And the excuse does not hold: `α² = 0.0053 %`, an order of magnitude
smaller than the `0.055 %` gap. Taken at face value, Eq. (71) puts the Z mass
**24σ** from PDG, not `+0.11σ`.

This is the same failure mode as `D(n)` in the lepton sector: a stated
normalization that does not produce the tabulated numbers, with the difference
absorbed silently. It does mean, however, that the boson sector — unlike the
lepton sector — has only **one** absorbed quantity rather than three, so it
retains two genuine mass *ratio* predictions. Those ratios are the part of the
electroweak sector worth testing further.

### T3.9 On-shell matching parameters from the paper's predicted masses

The commissioning calculation is confirmed exactly (using `v = 246.21965 GeV`
and the Table 1 masses):

```
g_2            = 0.6528276683      commissioned 0.6528277   ✓
g_Y            = 0.3499382355      commissioned 0.3499382   ✓
sin²θ_W (OS)   = 0.2232002862      commissioned 0.2232003   ✓
λ              = 0.1293322002      commissioned 0.1293322   ✓
y_e            = 2.9350286 × 10⁻⁶
y_μ            = 6.0686861 × 10⁻⁴
y_τ            = 1.0205763 × 10⁻²
```

These are correct arithmetic. Their **status**, however, is now fixed by T3.8:
they are on-shell parameters derived from a table that the paper's own formula
does not generate. They should be recorded as *conditional low-energy matching
results of the tabulated masses*, not of the paper's mechanism.

### T3 verdict

> A complete 4D EFT matching **cannot be constructed**, because the paper's
> boundary data is not simultaneously realizable at any scale, and because the
> paper supplies none of the required ingredients: no matching scale, no
> threshold prescription, no scheme, and no formula for `Δ_spec(Q²)`. What the
> calculation *does* deliver is the correct completion: measured anchors,
> two-loop SM running, crossing at `1.089 × 10¹³ GeV`, `sin²θ_W = 3/8`
> identically at the crossing — i.e. **the Two-Law result, with the paper
> contributing nothing that survives**.

---

## Supplementary — the S⁹ neutrino sector

This was not commissioned, but it is the paper's **only** genuinely
parameter-free mass prediction and it decides the "sharply specified neutrino
package".

**What verifies.** Every closed-form coefficient reproduces:

| quantity | recomputed | paper |
|---|---|---|
| eigenvalues `λ_k = (k+2)(k+6)` | matches Ikeda–Taniguchi for coexact 2-forms on `S⁹` | Eq. (116) ✓ |
| `d(1)` | `120` = `dim Λ³(10)` for `SO(10)` `[0,0,1,0,0]` | Eq. (117) ✓ |
| `ζ′_Δ2(0)` | **`−0.41364466`** (independently recomputed from Eqs. 116–117 via Hurwitz zeta derivatives) | `−0.41364` ✓ |
| `C₉` | `−0.1567077415` | `−0.15671` ✓ |
| `β₉` | `0.001330635242` | `1.331 × 10⁻³` ✓ |
| `σ₉` | `0.01522422853` | `1.522 × 10⁻²` ✓ |

With `a₉ = √5` the full Eq. (127) reproduces the paper's own table:
`m_ν = 0.000969545, 0.00870805, 0.049604 eV`, `Δm²₂₁ = 7.48902 × 10⁻⁵`,
`Δm²₃₁ = 2.45962 × 10⁻³ eV²`, `Σm_ν = 0.0592816 eV`.

**This is a real asymmetry worth recording: the neutrino sector's spectral
coefficient is independently reproducible, while the charged-lepton sector's
`D(n)` is not.**

**What breaks.** Eqs. (119)–(121) *derive*
`a₉ = exp(ζ′/16)·√5 = 2.179000872`, then discard the exponential factor,
declare `a₉ = √5 = 2.236067977` "exactly", and absorb the difference "into the
`O(1)` prefactor of the determinant". But `a₉` multiplies `n`; the shift
`δ = 0.057067105` multiplies `m_n` by `exp(nδ)`, which is n-dependent and
**cannot** be absorbed into an n-independent prefactor. With the derived `a₉`:

| observable | `a₉ = √5` (declared) | `a₉ = exp(ζ′/16)√5` (derived) | PDG | miss |
|---|---|---|---|---|
| `Δm²₂₁` | `7.489 × 10⁻⁵` | `5.9515 × 10⁻⁵` | `(7.53 ± 0.18) × 10⁻⁵` | **8.8σ** |
| `Δm²₃₁` | `2.4596 × 10⁻³` | `1.7463 × 10⁻³` | `(2.453 ± 0.033) × 10⁻³` | **21.4σ** |
| `Σm_ν` | `0.05928 eV` | `0.05048 eV` | — | — |

The illegal discarding is the difference between a fit and a 21σ miss. It is
**load-bearing**, not cosmetic.

**A second inconsistency:** the knot torsions are `τ₃ = (1, 1, √3)` on `S³`
(Eq. 70) but `τ₉ = (1, 4, 3)` on `S⁹` (p. 51), for the *same three knots*
(unknot, Hopf link, trefoil), both described as "the universal knot torsion
normalizations". At most one assignment can be right.

**Implied Dirac Yukawas** at the Two-Law minimality tier
(`y = √2 m/v`), from the paper's quoted spectrum:
`y_ν1 = 5.5714 × 10⁻¹⁵`, `y_ν2 = 5.00162 × 10⁻¹⁴`, `y_ν3 = 2.84911 × 10⁻¹³`,
`Σm_ν = 0.059282 eV` — confirming the commissioning values to the quoted
precision.

---

## Dependency record

| tier | items |
|---|---|
| **Two-Law derived** (verdict-blind, no measured constant) | Hopf/connection grammar; local `G₀`; exact `ℤ₆` kernel by sequential sealing; three-generation floor; one-Higgs and Dirac minimality targets; **`n★ = 4 ⇒ S⁹ → CP⁴` with structure group `U(5)` and minimal SM-carrying subgroup `S(U(3)×U(2))` (new, this audit)**; the equal-trace crossing at `1.089 × 10¹³ GeV` at two loops **(confirmed, this audit)** |
| **Derived, but standard** (correct, not novel) | `S(U(3)×U(2)) ≅ (SU(3)×SU(2)×U(1))/ℤ₆` — Baez–Huerta 2010, the paper's own ref [16]. The census *reaches* this group independently; it does not newly derive the `ℤ₆` |
| **Two-Law premises inside the census** (declared, not derived) | tier-C free-sealing clause (selects `n★ = 4` uniquely rather than as an argmin); **obligation O4 (`q_ν = 0`) fixing the hypercharge scale `y = 1/6`** — without it `κ(y) = 60y²` and `sin²θ_W = 1/(1+60y²)` are free; use of the **ambient simple `su(5)` trace form** to make `κ = 5/3` rep-independent |
| **Paper bridge premise** (adopted, not derived) | single universal bundle; finite `S⁹` shell *as the paper argues it*; soldering; Beltrami mass extractor; knot-sector identification; spectral normalization; four-dimensional descent |
| **Conditional output** | `α` from Thm 46 (symbolic, `+0.61 ppm`, but the shell it is computed on is selected circularly in the paper); the S⁹ neutrino spectrum **conditional on the illegal `a₉` truncation**; effective couplings; mixing; gravity |
| **Error as written** | `D(n)` presented as derived when it is the measured masses (Eq. 66 vs Eqs. 63–65); Remark 23's absorption of a linear-in-n term into an n-independent scale; `a₉` truncation (Eqs. 119–121); `α_s` Eq. (78) vs Remark 24, off by `322.9×` with the wrong sign; printed `g = 0.6205` and `g′ = 0.3469`; **Eq. (71) overshoots Table 1 by a uniform `+0.055 %`, putting `m_Z` at `24σ` rather than `+0.11σ`**; Thm 49 (UV finiteness) vs Thm 19 (cubic vertex); `τ₃` vs `τ₉` for the same knots |

---

## Registered consequences

1. **XF-C (Koide `Q = 2/3`) is untouched.** The paper supplies no independent
   determination of the charged-lepton spectrum, so it can neither corroborate
   nor burn the prediction. The "−9.23 ppm near-miss" must not be recorded as
   external evidence in either direction.
2. **A new verdict-blind derivation is available for registration:** the S⁹
   minimality census, yielding `n★ = 4`, `U(5)`, and `S(U(3)×U(2))`. It carries
   **three** declared premises, all of which must be frozen before anything is
   claimed: the tier-C free-sealing clause (without which `n★ = 4` is an argmin
   over `{4,6,7,8,…}`, not unique); obligation O4 (`q_ν = 0`) fixing the
   hypercharge scale; and use of the ambient simple `su(5)` trace form for
   `κ = 5/3`.
3. **The `ℤ₆` is reached by a second independent route, but not newly
   derived.** The census independently arrives at `S(U(3)×U(2))`; the
   identification of that group with `(SU(3)×SU(2)×U(1))/ℤ₆` is textbook
   (Baez–Huerta 2010). This is a concordance of endpoints and mildly
   strengthens XF-B3; it does not close it and must not be recorded as a second
   discovery.
4. **The two cosmological bridges remain competing alternatives**, as
   commissioned: the paper's `w = −1` exactly, versus the Two-Law synthesis's
   late interval with `w_eff < −1`. Neither is forced by the core; they must
   not be combined.
5. **The paper's status in the record:** *Candidate Hopf-Spectral Physical
   Realization — external bridge model, not part of the sealed core*, with the
   error tier above attached. Its α formula and its neutrino spectral
   machinery are the parts worth retaining; its charged-lepton sector is not a
   derivation and should not be cited as one.

## Answer to the commissioning question

> **Does the paper supply the missing parameter layer, or a numerically
> impressive but underdetermined realization?**

Neither, exactly — the accounting is worse than "underdetermined" in one sector
and better than expected in another.

- **Charged leptons: not underdetermined but exactly saturated.** 3 free
  quantities, 3 masses, zero residual. This is not a parameter layer; it is a
  parameterization of the data.
- **Neutrinos: genuinely overdetermined and predictive** — every coefficient
  closed-form, one free scale `Λ₉` itself derived from `v` — and it survives
  verification except for one illegal truncation whose repair costs 21σ. This
  *is* a parameter layer, and it is falsified in its derived form.
- **Electroweak: not a parameter layer at all.** The boundary data is the
  measured `Q² → 0` pair; no single matching scale exists; the strong-coupling
  formula is off by 322.9× with the wrong sign.
- **The one thing that does supply structure is the geometry, not the
  spectrum** — and the Two-Law census, not the paper, is what extracts it.
  `n★ = 4`, `U(5)`, `ℤ₆`, `5/3`, `3/8` are all derivable verdict-blind, and
  none of them needed the paper's mass machinery.

**The sharpest remaining open question:** whether the tier-C free-sealing
clause of the census can be derived from the Two Laws rather than declared. If
it can, `S⁹ → CP⁴` becomes a theorem and the paper's geometry acquires the
derivation its own argument fails to give. If it cannot, the census returns
only an argmin over `{4, 6, 7, 8, …}` and a monotone cost must be justified
independently. That single question is worth more than every mass formula in
the paper.

---

# Addendum v1.1 — post-synthesis pass

**Date:** 2026-07-25, same day, after the audit body above was frozen.
**Provenance:** five independent derivation lanes with 25 adversarial
verification agents (36 agents total, 0 errors) ran against the v1 findings.
Three findings were refuted — all on *status*, none on arithmetic — and those
corrections are already folded into T2.3, T2.4 and T2.6 above. The items below
are **new** findings that the lanes surfaced and that I re-derived from scratch
before recording. Reproduction: `scripts/xf1_hopf_bridge_audit/a5_addendum.py`.

Nothing in the v1 verdicts changes. Every item below strengthens them.

### A5.1 The "quadratic piece" reading of D(n) is closed off exactly

Any three-point quadratic fit to `−ζ′_n(0)` has its curvature fixed by the
second difference, which has a closed form:

```
2nd difference of −ζ′_n(0) = −6.7094567676650416029 = −(8 ln 3 − 3 ln 2)   exactly
⇒ c₂ = −3.3547283838325208015
ζ(3) required               = +1.2020569031595942854
ratio = −2.7908233
```

**Wrong sign and 2.79× too large.** No quadratic extracted from Eq. (65) can be
`ζ(3)n²`. T1.2's conclusion is now closed rather than merely unmet.

### A5.2 The paper's own asymptotic claim is false

Eq. (66)'s note reads *"For large n, `D(n) ∼ ζ(3)n²`."* The Eq. (65) object
grows like `−(1/3)n³ ln n`:

| `n` | `−ζ′_n(0)/(ζ(3)n²)` | `−ζ′_n(0)/(−⅓n³ln n)` |
|---|---|---|
| 10 | `−6.3393982` | `0.99283853` |
| 100 | `−120.35152` | `0.94243666` |
| 1000 | `−1825.9728` | `0.95324303` |

### A5.3 Thm 46 names a coset whose volume it does not use

p. 61 verbatim: *"two copies of `Vol(S⁴)` (the coset volume
`Vol(SU(3)/SU(2))`)"*. But `SU(3)/SU(2) ≅ S⁵`, and the paper says so itself on
p. 23.

```
Vol(S⁴) = 8π²/3 = 26.3189450696      Vol(S⁵) = π³ = 31.0062766803
1/α using Vol(S⁴)  (as PRINTED) = 137.036082448164
1/α using Vol(S⁵)  (as NAMED)   = 190.194176592907
```

The recipe works only under the printed symbol, not under the named object.
Further, splitting `ln α`:

```
n-independent prefactor 9/(8π⁴) = 0.0115492300365199   → 90.669 % of ln α
n-dependent N_B = (Vol(S⁹)/160)^(1/4) = 0.631847154   →  9.331 %
```

**~91 % of `ln α` is carried by a factor independent of the shell**, which
substantially weakens "α is derived *from* S⁹". This tempers the one thing T1.1
credited to the paper: the arithmetic is exact and equals Wyler's constant, but
the geometric attribution is thinner than presented.

### A5.4 Table 1 is incompatible with the paper's own `g` — at 5 %, not 0.055 %

T3.8 found a uniform `+0.055 %` gap between Eq. (71) and Table 1. A larger
inconsistency sits one level up:

```
Thm 35:  g = 0.619771739575        Axiom 1:  v = 246.21965 GeV
tree-level m_W = g·v/2 = 76.299990399 GeV
Table 1    m_W         = 80.3695 GeV
discrepancy = −5.0635 %
```

and the two `sin²θ_W` values disagree likewise:

```
Table-1 on-shell  1 − m_W²/m_Z² = 0.223200286206
Thm 35 / Eq. (75)  3/(4π)       = 0.238732414638
   relative to 3/(4π):     −6.50608 %
   relative to on-shell:   −6.95875 %
```

The boson-mass table and the gauge-coupling theorem are **two disconnected
chains disagreeing at the 5–7 % level about the same two observables.**

### A5.5 Obligation O8 fails inside the paper's own summary table

The census's O8 (one common spectral/public-record extractor) is violated by
p. 52 as printed:

| | `S³` | `S⁵` | `S⁹` |
|---|---|---|---|
| action type | CS | CS | `L²` torsion |
| operator | `B = ⋆d` on `Ω¹` | `B = ⋆d` on `Ω²` | `Δ₂` on `Ω²` (order 1 → 2) |
| framing `ℓ` | 6 (knot) | 6 (knot) | 16 (contact chirality) |
| det exponent | `−1/2` | `−1/2` | `+1/2` |

A single extractor cannot change operator order, framing rule and determinant
sign between sectors of one theory. Separately, Eq. (112)
`S₉[T] = γ₉∫T ∧ ⋆T` is a positive-definite quadratic form in a **commuting**
2-form field; Gaussian integration gives `(det)^{−1/2}`. The paper's
`Z = (det Δ₂)^{+1/2}` "fermionic sign" (p. 49) has no spinor field to justify
it — and that sign propagates into `κ₉`, hence into `Λ₉`, hence into the whole
neutrino spectrum.

### A5.6 What the paper does and does not say (full-text search, 78 pp.)

```
'koide'         0 occurrences
'ppm'           0
'proton decay'  0
'georgi'        0
'grand unif'    1  — bibliography entry [16] (Baez–Huerta) only
```

Three consequences. **(i)** The paper makes no Koide claim at all; T1's verdict
that XF-C is untouched is stronger than stated — the paper is not a failed
external test of XF-C, it is silent on it. **(ii)** T2.4's remark that the
census is "consistent with the paper's *no intermediate GUT group*" is
**vacuous as a statement about the paper**, which never raises the issue; the
consistency is with the Two-Law theory alone. **(iii)** The paper's single ℤ₆
mention is imported from Baez–Huerta [16], reinforcing T2.3's scoping.

### A5.7 Correction to the v1 quotation of `ℓ₉`

The v1 body quoted `ℓ₉ = 2⁸/2 = 16`, which is a misreading of flattened PDF
superscripts (`2⁸/2 = 128`). The paper's actual reading is
`dim_C S± = 2^((10−2)/2) = 2⁴ = 16` and `ℓ₉ = 2^(8/2) = 16`. **The number is
fine; only the justification fails**: `S⁹ = Spin(10)/Spin(9)` is
odd-dimensional and `Spin(9)` has a *unique* 16-dimensional spinor irrep, so
there is no chirality split to count. Noted in passing: 16 is available
`U(5)`-covariantly as `dim Λ^even(C⁵) = 1 + 10 + 5 = 16` — the right number by
a route the paper's own structure does not support. The body text is corrected.

### A5.8 Census enumeration — one unresolved discrepancy, immaterial

An independent lane's enumeration of `O1–O4 + O7` survivors gives
`n ∈ {4, 7, 8, …}`; mine (tier B) gives `{4, 6, 7, 8}`. The two disagree at
`n = 5, 6` on how "exactly one commutant `U(1)`" is encoded. **This changes
neither result**: both give argmin 4, and tier C — which excludes composites,
higher spins and spectators — gives `{4}` uniquely under either enumeration.
Recorded as open rather than silently resolved. The lane also notes that at
`n = 7` the same trace computation yields `sin²θ_W = 3/4` and at `n = 8`
`7/16`; preferring `n = 4` because it gives `3/8` would break verdict-blindness
and is **not** done here — tier C excludes `n = 7, 8` structurally, before any
value is computed.

### A5.9 The falsifier this audit can publish

T1's inverse problem pins the target from both sides, which turns the open
question into a usable test. Any future derivation of `D(n)` — from
Nash–O'Connor on `L(n,1)`, Cheeger–Müller, or anything else — must reproduce

```
D_required(n) − D(n) = 3.411418677 × 10⁻⁵ ± 6 × 10⁻¹⁰   simultaneously for n = 1,2,3
```

with the constant free (degenerate with `Λ_Hopf`) and the shape fixed:
`D(2) − D(1) = 3.603534014`, `D(3) − D(1) = 9.615217254`. **The test is that it
must do so without being told `m_τ`.** The cited route is closed: the Eq. (63)
object has forced second difference `−6.7094567676650416029` where `+2.4041138`
is needed (A5.1), and grows as `−(1/3)n³ln n` (A5.2).

This is the single calculation that would flip the audit. If such a
construction exists and reproduces the shape to `~10⁻⁹`, the paper contains a
real theorem with a broken proof and the charged-lepton sector becomes a
genuine three-mass prediction. Given that `D(3)` encodes a human-rounded
six-figure PDG literal to ten digits, that outcome is unlikely — but it is now
sharply testable rather than merely doubted.
