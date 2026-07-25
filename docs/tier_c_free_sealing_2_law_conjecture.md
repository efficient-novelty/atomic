# The Tier-C Free-Sealing Clause: Derivation Attempt and Conjecture

**Status:** partially derived. One premise remains open.
**Date:** 2026-07-25.
**Context:** `docs/XF1_HOPF_BRIDGE_AUDIT_V1_RESULT.md` (Hopf shell census, T2);
`docs/the_answer_is_two_theory_short.tex` (Hopf shell census theorem and the
Local alphabet theorem); ledger events 008 / 008a.
**Verification:** every representation-theoretic claim below was computed, not
quoted; see §8 for the reproduction.

---

## 1. Is this still worth doing?

**Yes — and the case is stronger than the audit stated, for a reason the audit
missed.**

The audit recorded tier-C as "declared rather than derived" and offered a
fallback: if it cannot be derived, the census returns an `argmin` over
`{4, 6, 7, 8, …}` and *"a monotone cost `κ(H_n)` must be justified
independently."*

**That fallback is not available.** The Selective Law's two-register acceptance
reads:

> at guarded stages (nonempty windowed obligation profile), acceptance is total
> discharge of the live demands with typed receipts, **and no value quantity
> legislates**

The census is scored against a nonempty obligation profile O1–O8. It is
therefore a **guarded stage**, and a monotone cost `κ(H_n)` is exactly a value
quantity legislating at a guarded stage. The Two Laws forbid it. The same
passage continues:

> where genuinely distinct total payments coexist, the law's silence is
> certified rather than broken, and history is lawfully a cone

So the trichotomy is sharper than the audit allowed:

| | outcome |
|---|---|
| tier-C derivable from the Laws | `n★ = 4` is a **theorem**; `S¹→S⁹→CP⁴` is selected outright |
| tier-C not derivable, and `n ∈ {4,6,7,8,…}` all totally discharge | the census **lawfully branches** into a cone, exactly like the Step-4 four-branch cone |
| tier-C replaced by a cost `κ(H_n)` | **forbidden** — value legislation at a guarded stage |

There is no "argmin" branch. Either tier-C is part of what discharge *means*,
or the census produces a certified cone. Both are publishable Two-Law results;
the middle option the audit assumed is ruled out by the Law itself.

This raises the value of the derivation considerably. It is no longer a
tidying-up exercise — it decides whether the shell is **selected** or whether
the theory must accept a second lawful cone in its physics layer.

---

## 2. What is to be derived

**Tier-C clause (as declared in the census).** The defining representation of
the fibration-preserving group `G(H_n) = U(n+1)` decomposes into the alphabet's
*defining* representations, each exactly once, with

- no composite `3 ⊗ 2` summand,
- no higher `SU(2)` spin,
- no spectator singlet.

Equivalently: `C^(n+1) ≅ 3 ⊕ 2`, forcing `n + 1 = 5` and `n★ = 4`.

**Target.** Derive this clause from the Selective Law's realization face
(free sealing) applied to the API that the Local alphabet theorem has already
selected — *not* from a cost, a dimension count, or any measured constant.

---

## 3. What the Two Laws supply

**Free sealing**, verbatim:

> once an API is selected, the new layer is its least term-model completion,
> containing **no unforced generators or equations** and **initial among exact
> realizations of that selected API**

with the standing warning:

> This relative initiality must not be confused with the false stronger claim
> that PEN always selects the initial completion of a weaker requirement fixed
> in advance.

Two things follow immediately, and both matter.

1. **Free sealing is a universal property, not a value quantity.** Initiality is
   structural: it is preserved under equivalence, it does not compare sizes, and
   it does not rank candidates on a scale. It is therefore *legitimate at a
   guarded stage* in a way that `argmin κ` is not. This is the hinge of the
   whole attempt.
2. **The initiality is relative to the selected API.** So the derivation must
   identify the API precisely and take initiality relative to *that*, never
   relative to the weaker profile O1–O8. Getting this backwards is the exact
   error the warning names.

---

## 4. The derivation

### 4.1 The API

The Local alphabet theorem selects `U(1)`, `SU(2)`, `SU(3)` along the
division-algebra ladder, and it selects them *with their carriers*:

| rung | group | carrier | role as stated in the theory |
|---|---|---|---|
| `ℂ` | `U(1)` | `C` | phase symmetry for complex amplitudes |
| `ℍ` | `SU(2)` | `ℍ ≅ C²` | smallest rich non-abelian internal orientation language |
| `𝕆` | `SU(3)` | `C³ ⊂ 𝕆` | stabilizer of the complex phase in the octonionic structure |

Write the carriers `V₁ = C`, `V₂ = C²`, `V₃ = C³`.

**Premise P1 (carrier-bearing alphabet).** The selected API is the alphabet
*with* these carriers, not the three abstract groups.

P1 is a reading of the Local alphabet theorem, not a new physical assumption:
the ladder does not produce abstract groups and then hunt for representations.
It produces *matrix groups acting on division algebras* — unit quaternions
acting on `ℍ`, the phase-stabilizer acting on the `C³` orthogonal to `ℂ` inside
`𝕆`. On that reading the carrier is constitutive of the selection, exactly as
`SU(n)` is defined as a subgroup of `GL(n,C)` rather than presented abstractly
and later represented. §5 records what would have to be added to the theorem to
make P1 a citation rather than a reading.

### 4.2 The arena, and why `U(1)` costs nothing

The frozen obligations O2 and O3 demand an `SU(2)` action and an `SU(3)` action
*on the shell*. Since the fibration-preserving group is `U(n+1)` and its
defining representation is `C^(n+1)`, the realization arena is `C^(n+1)`. No
extra premise is needed here — this is O2/O3 as already frozen.

O1 demands **a primitive central complex phase, freely acting**. On the Hopf
shell `S^(2n+1) ⊂ C^(n+1)` the fiber `U(1)` acts by `v ↦ zv`: by *scalars on the
whole space*. Central means acting by scalars; acting by scalars means **not
occupying a summand**. So `V₁` is discharged by the ambient complex structure of
the arena itself.

This is not a convenience. It is the structural reason the Hopf family is the
right candidate set at all: it is the arena in which a central `U(1)` is
realized freely *without consuming a generator*. Had `V₁` required its own
summand, the coproduct below would be `C⁶` and `n★ = 5`.

### 4.3 Exact realizations, and the theorem

**Definition.** Let `G = SU(3) × SU(2)`. An **exact realization of the API** is a
triple `(W, ι₃, ι₂)` where `W` is a finite-dimensional complex vector space
carrying a `G`-action commuting with the ambient scalars, and

```
ι₃ : V₃ ↪ W    is SU(3)-equivariant
ι₂ : V₂ ↪ W    is SU(2)-equivariant
```

exhibit the two selected carriers inside `W`. A **morphism**
`(W, ι₃, ι₂) → (W′, ι′₃, ι′₂)` is a `G`-equivariant linear `f : W → W′` with
`f ∘ ι₃ = ι′₃` and `f ∘ ι₂ = ι′₂`. Call this category `E`.

`E` is the category of realizations **under** the two carriers — precisely the
shape free sealing refers to when it says "initial among exact realizations of
that selected API".

> **Theorem (free sealing selects `C⁵`).**
> `(V₃ ⊕ V₂, can₃, can₂)` is an initial object of `E`. Consequently the free
> seal of the carrier-bearing alphabet on a Hopf shell arena is `C⁵`, whence
> `n + 1 = 5` and
> ```
> n★ = 4,   S¹ → S⁹ → CP⁴,   G(H₄) = U(5).
> ```

**Proof.**
*Existence.* `V₃ = (3,1)` and `V₂ = (1,2)` are non-isomorphic irreducibles of
`G`, so for any `(W, ι₃, ι₂)` the images `im ι₃` and `im ι₂` intersect trivially
(a nonzero intersection would be a common subrepresentation of two
non-isomorphic irreducibles). Hence `f := ι₃ ⊕ ι₂ : V₃ ⊕ V₂ → W` is a
well-defined `G`-equivariant map, and it satisfies the two triangle conditions
by construction.

*Uniqueness.* Any morphism `g` out of `(V₃ ⊕ V₂, can₃, can₂)` must satisfy
`g|_{V₃} = ι₃` and `g|_{V₂} = ι₂`. Since `V₃ ⊕ V₂` is spanned by those two
subspaces, `g = f`. ∎

The direct sum is the coproduct in `Rep(G)`, so the theorem says exactly:
**the free seal of the API is the coproduct of its carriers.** That is
"no unforced generators" in its sharpest form — a generator is unforced precisely
when it is not the image of a carrier under a coproduct injection.

### 4.4 Every tier-C exclusion is now a computed carrier-multiplicity statement

The clause's three prohibitions are not three stipulations. They are three
instances of one condition — *exactly one copy of each carrier and nothing else*
— and each failure is a count:

| candidate | `dim` | copies of `V₃` | copies of `V₂` | spectator | verdict |
|---|---|---|---|---|---|
| `C⁵ = (3,1) ⊕ (1,2)`  `[n=4]` | 5 | **1** | **1** | no | **free-sealed** |
| `C⁶ = (3,1) ⊕ (1,2) ⊕ (1,1)`  `[n=5]` | 6 | 1 | 1 | **yes** | unforced |
| `C⁶ = (3,2)`  `[n=5]` | 6 | 2 | 3 | no | unforced |
| `C⁷ = (1,1) ⊕ (3,2)`  `[n=6]` | 7 | 2 | 3 | **yes** | unforced |
| `C⁷ = (3,1) ⊕ (1,4)`  `[n=6]` | 7 | 1 | **0** | no | unforced |
| `C⁸ = (1,2) ⊕ (3,2)`  `[n=7]` | 8 | 2 | 4 | no | unforced |

The composite dies by branching, not by fiat:

```
(3 ⊗ 2) ↓ SU(3)  =  3 ⊕ 3          two copies of the SU(3) carrier
(3 ⊗ 2) ↓ SU(2)  =  2 ⊕ 2 ⊕ 2      three copies of the SU(2) carrier
```

five carrier copies where the API demands two. And the higher spin dies because
`4 = Sym³(C²)` is not the carrier at all — it contains **zero** copies of `V₂`,
so `(3,1) ⊕ (1,4)` is not even an exact realization: `ι₂` does not exist.

---

## 5. The remaining gap

Exactly one premise is doing work that the theory does not currently state.

> **Open premise P1.** The API selected by the Local alphabet theorem is the
> alphabet *with its division-algebra carriers* `(C, C², C³)`, not the three
> abstract groups.

Everything else is either already in the theory (free sealing; the guarded-stage
prohibition on value legislation; O1–O3 as frozen) or is proved above.

**Why P1 is not free.** If the API were the abstract groups, "exact realization"
would mean any faithful representation, and the initial object of the resulting
category does not exist: there is no universal faithful representation. One could
retreat to "minimal faithful representation" — which is `3 ⊕ 2` at dimension 5,
since faithfulness needs `dim ≥ 3` for `SU(3)` and `dim ≥ 2` for `SU(2)`, and no
4-dimensional option is faithful on both. **But that retreat is illegal**: it is
a dimension minimum, hence a value quantity, hence forbidden at a guarded stage
by exactly the rule that killed the `argmin`. The carrier route survives
precisely because it replaces the minimum with a universal property.

So P1 is not a convenience — it is the only known way to reach `C⁵` without
value legislation.

**A robustness note that sharpens what P1 actually buys.** Enumerating all
representations of `SU(3) × SU(2)` faithful on both factors, dimensions 1–4
admit **none**, and dimension 5 admits **exactly two**: `(3,1) ⊕ (1,2)` and its
conjugate `(3̄,1) ⊕ (1,2)` — the same pair the census returns. So the illegal
dimension-minimum route and the legal coproduct route **agree on the answer**.
What P1 buys is therefore not the value `C⁵` but the *right to assert it*: it
converts a forbidden minimization into a universal property. The value is
robust; only its legitimacy depends on P1. This is worth stating plainly,
because it means a failure of P1 does not change `n★` — it changes whether the
theory is entitled to claim `n★` at all.

**What would close it.** A lemma inside the Local alphabet theorem of the form:

> *The ladder selects each factor together with the division-algebra module on
> which it is realized: `U(1)` on `ℂ`, `SU(2)` on `ℍ`, and `SU(3)` on the
> `C³ ⊂ 𝕆` orthogonal to the preserved complex phase. The carrier is part of the
> selection, not a subsequent choice.*

The material for this lemma is already in the theorem's proof sketch — the
groups are produced *as* automorphism/stabilizer groups of those modules. What
is missing is the explicit statement that the module travels with the group into
the API. That is a paragraph of work in the theory, not a research programme.

---

## 6. Secondary open items (do not block P1)

1. **The `n = 5, 6` enumeration discrepancy** recorded in the audit — two
   independent enumerations of the O1–O4 + O7 survivor set disagree on how
   "exactly one commutant `U(1)`" is encoded. Immaterial to the theorem above,
   which never uses the tier-B survivor set, but it should be settled.
2. **O5 (three-generation capacity) is still unadjudicated by the shell.**
   `χ(CP⁴) = 5` and every Betti number is 1; nothing in the base equals three.
   The generation floor continues to rest where it already rested. The theorem
   above does not change this and must not be read as touching it.
3. **O8 (one common extractor)** is not used in the derivation. Whether the
   shell can supply a single extractor is a separate question.

---

## 7. Falsification

The derivation is wrong if any of the following holds.

- **`V₃ ⊕ V₂` is not initial in `E`.** Kill by exhibiting an exact realization
  admitting two distinct morphisms from `V₃ ⊕ V₂`, or none.
- **P1 is false**: the Local alphabet theorem can be shown to select the groups
  *without* their carriers, in a way that survives the guarded-stage prohibition.
  Then tier-C is not derivable by this route and the census branches into a cone.
- **Initiality is itself a value quantity** under the Selective Law's intended
  reading. This would be surprising — initiality is invariant under equivalence
  and compares nothing — but it is the interpretive claim the whole attempt
  rests on, and it is the theory's own to adjudicate.
- **The central-phase argument fails**: if O1 required `V₁` to occupy a summand,
  the coproduct is `C⁶` and `n★ = 5`, giving `S¹ → S¹¹ → CP⁵` and a different
  gauge structure entirely. This is the sharpest quantitative fork in the
  argument.

---

## 8. Reproduction

Representation-theoretic content of §4.4 and the initiality Hom-computations:

```bash
python scripts/xf1_hopf_bridge_audit/a2_census.py
```

for the census enumeration and the commutant computations, together with the
branching and carrier-multiplicity table computed inline for this note. The
theorem in §4.3 is a two-line categorical argument and requires no computation;
the Hom-space vanishings it uses (`Hom((3,1),(1,2)) = 0` and its transpose) are
Schur's lemma for non-isomorphic irreducibles.

---

## 9. Summary

| item | status |
|---|---|
| Is the derivation still worth doing? | **Yes, more than the audit recorded** — the `argmin` fallback is forbidden by the Law, so the alternative is a certified cone, not a cheap ranking |
| Free sealing legitimate at a guarded stage? | **Yes** — it is a universal property, not a value quantity |
| `V₃ ⊕ V₂ = C⁵` initial among exact realizations? | **Proved** (§4.3) |
| Tier-C's three prohibitions reduced to one condition? | **Yes** — exactly one copy of each carrier; all three are computed carrier counts (§4.4) |
| `n★ = 4` follows? | **Yes, given P1** |
| P1 itself | **Open.** Reading of the Local alphabet theorem; needs one lemma stating that the carrier travels with the group into the API |
| Net effect | tier-C moves from *declared clause with an illegal fallback* to *theorem modulo one statable lemma* |

The honest one-line verdict: **this is no longer an open problem, it is an open
paragraph.**
