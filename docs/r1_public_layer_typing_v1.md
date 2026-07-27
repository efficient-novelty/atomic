# R1: Is the Public Layer the Self-Paired Layer?

**Date:** 2026-07-25.
**Status:** discharged, and it collapses into an existing premise rather than
standing beside it. The feared target-2 dependency is **avoided**, and
phase-blindness comes out as a consequence instead of an input.
**Context:** `docs/target1_ledger_invariance_joint_v1.md` §3.1 and §6, which
raised R1 and named the target-2 risk; ledger event 009.
**Register independence:** unaffected by the standing provenance disclosure —
nothing here consumes the certified fifteen-stage register.

---

## 1. Result

R1 asked: are the objects the public interface discards over the self-paired
objects `Â = A* ⊗ A`, rather than the pure carriers `A`?

> **Yes — and R1 is not an independent premise. It merges with R3 into a single
> free-sealing clause.** The consolidated premise is
>
> **S1 (single source).** The public interface introduces no scalar-producing
> structure beyond what the dagger already supplies.
>
> Given S1 and target 1's dagger fragment, the following are consequences rather
> than assumptions: the public layer is self-paired (R1); the discard is the
> canonical pairing (R3); monoidal discarding (Q1a); ledger invariance as the
> equality `Causal = Isom` (Q1b); and **phase-blindness of public scalars**.

The premise ledger for target 3 therefore goes

| | before | after |
|---|---|---|
| API clauses | Q1(a) discarding **+** Q1(b) ledger invariance | — both derived |
| structural premises | R1 self-paired layer **+** R3 free-sealed discard | **S1** (one clause) |
| outstanding debt | target 1's dagger fragment | target 1's dagger fragment |

Two API clauses and two structural premises become one clause and one existing
debt, with phase-blindness thrown in.

---

## 2. The bad branch, and why it is real

The route named in the parent note was: public weights are phase-blind, a
phase-blind layer is a self-paired layer, therefore R1.

**That route imports target 2, and the import is not cosmetic.** The synthesis
introduces the phase in the *definition* of a preparation:

> a phase labeling `θ_x : π₀(Ψ_x) → U(1)`, constant on isomorphism classes

and it uses phase-neutrality as the *input* to the Born argument — "the public
scalar support for `x` must be phase-neutral", from which the conjugate
self-pairing is then selected. Meanwhile reconstruction target 2 is precisely
the obligation to *derive* "a faithful primitive central `U(1)` action on
pure-process carriers".

So the phase route would justify R1 by a `U(1)` that is posited where target 2
says it must be earned. Taking it would make target 3 depend on targets 1 **and**
2, and would do so through an assumption the package has already flagged as
owed. This is the branch the parent note warned about, and it is not available.

---

## 3. The good branch: the dagger already makes scalars

Turn the dependency around. Ask not "what does phase-blindness force?" but
"given a dagger, what scalar can the public interface even form?"

A state is a morphism `ψ : I → A`. It is not a scalar. The dagger converts it to
an effect `ψ† : A → I`, and the only scalar functorially available from a single
state and the dagger is

```
ψ† ψ : I → I .
```

That is the self-pairing. So the public value of a state is computed on the
self-paired object, **because the dagger is the structure that makes values out
of states**, and self-pairing is what it does. No phase group is consulted.

**Phase-blindness is then a theorem, not a premise.** For any scalar `u` with
`u† u = 1`,

```
(uψ)† (uψ)  =  u† u · ψ† ψ  =  ψ† ψ .
```

So the public scalar is invariant under the group of unitary scalars, *whatever
that group turns out to be*. This is the correctly scoped statement and it is
worth stating carefully: it derives that public scalars are blind to the phase
group, and it does **not** derive that the group is `U(1)`. Identifying the group
remains target 2's, untouched. What has changed is the direction of dependence —
blindness now follows from the dagger rather than being assumed ahead of it.

**Where S1 does the work.** Nothing above forbids the public interface from
carrying its *own* scalar-producing structure, unrelated to the dagger. S1 is
exactly the clause that forbids it, and it is free sealing in its ordinary form:
a second scalar-former is a generator that no clause of the API demands and no
live obligation discharges. This is the same move as the repaired empty-desert
constraint, and it is legal at a guarded stage for the same reason — a universal
property, not a comparison of costs.

Note that S1 and R3 are then the same clause seen at two depths, which is why
they collapse. R3 said "don't posit a discard beside the canonical pairing"; S1
says "don't posit a scalar-former beside the dagger". The second entails the
first.

---

## 4. The theory already types public extraction this way

R1 turns out to be, like tier-C's P1, **a reading of sealed content rather than
a new assumption** — and the reading is close to explicit.

The synthesis defines

```
D := Ψ ×_X Ψ̄ ,        D_x ≃ Ψ_x × Ψ̄_x
```

and calls it, in its own words, *the decohered self-pairing over outcomes*. The
Born weight is then computed as `w(x) = |D_x|`, on `D`, not on `Ψ`. The notation
index lists `D` as "decohered conjugate self-pairing over public outcomes".

So the object public extraction is typed over is already the self-paired one.
What was missing was not the typing but the *reason* — and the reason on offer
ran through phase-neutrality, i.e. through the bad branch. §3 supplies a reason
that does not.

The parallel with tier-C is exact and worth recording, because it is now twice
that the same shape has appeared:

| | tier-C | here |
|---|---|---|
| open premise | P1: the API is the alphabet **with its carriers** | R1: the public API is typed over **self-paired** objects |
| status found | a reading of the Local alphabet theorem | a reading of the Born apparatus |
| one-line content | *the carrier travels with the group into the API* | *the conjugate travels with the amplitude into the public API* |
| closure | one lemma in the synthesis | one lemma in the synthesis (§7) |

---

## 5. The countermodel, re-diagnosed

`Mat_{ℝ≥0}` with `ε_n = (1,…,1)` fails S1, and the failure is now visible in one
line: for a state `ψ : 1 → n`, that category offers **two** ways to make a
scalar — `ψ†ψ` from the dagger, and `ε_n ψ` from the posited discard. The second
is a scalar-producing structure beyond the dagger. S1 forbids it.

This sharpens the earlier diagnosis rather than replacing it. The parent note
said the countermodel's discard "lives on a different object" than the canonical
pairing; that is true, and the reason it can is that the countermodel was
allowed a second source of scalars in the first place.

The independence result of ledger event 009 is untouched: it showed ledger
invariance does not follow from the clauses *as abstractly stated*, and S1 is not
among them.

---

## 6. What is left

**S1 is the whole of the remaining structural premise, and it should be attacked
as free sealing rather than as physics.** The question is not "is the world like
this" but "does the selected public API force a second scalar-former". If the
public interface's selected clauses can be shown to mention only records and
their comparison — never a valuation — then S1 is immediate. If some sealed
clause of Internal Record Actualization independently supplies a valuation, S1
fails and the countermodel returns.

> **Audited — `docs/s1_single_source_audit_v1.md`. S1 is FALSE on the current
> text**, and by the second disjunct: Internal Record Actualization clause 1
> requires the record to be correlated "by the **Born-weighted** internal
> dynamics", and the public-event extractor requires the record to be
> "**Born-supported**". Those are valuations, supplied independently of the
> dagger. Five neighbouring clauses were audited and cleared, so the hit count
> is exactly two, and Public Confluence is valuation-free throughout.
>
> The failure is **textual, not structural**: both clauses can be restated
> weight-neutrally, and the theory has an independent reason to do so, since as
> written an input to its own dependency chain
> (`DCT + Closed(U) + IRA + PC + Q_p ⇒ observer-indexed public physics`)
> presupposes the Born weight that the public-physics layer derives. The repair
> is an **axiom change** and has not been made.

**The dagger remains target 1's**, scoped as before to the dagger /
state–effect fragment. Nothing here reduces that debt; it is now the only debt.

**Two inherited items, unchanged and still owed.** The compact-closedness
hypothesis behind cyclicity in the parent note's §3.5, and that section's
presentation debt — it is written in `Tr` / `f†f` notation and must be redone in
the abstract calculus before it counts. §3 above has the same exposure in
milder form: `ψ†ψ` is model notation for an abstract statement.

**One question I could not settle.** Whether the Born apparatus that supplies
the reading in §4 is itself downstream of the public-event extractor `Q_p`,
which the synthesis explicitly says the Two Laws do not determine. The discard
is more primitive than a choice of POVM — it is the trivial effect, not a
measurement — so I do not think R1 inherits the `Q_p` bridge. But I have not
proved that, and if it does inherit it, R1's status drops from *reading of
sealed content* to *reading of a bridged layer*.

---

## 7. What would close it in the synthesis

One lemma, in the Born section, of the form:

> *The public interface is typed over the decohered self-pairing. The dagger is
> the only structure the public API selects that converts a state into a scalar,
> and it converts it into `ψ†ψ`; the conjugate therefore travels with the
> amplitude into the public API, and no further scalar-former is sealed. Public
> phase-blindness follows for the group of unitary scalars, whatever that group
> is; identifying it as `U(1)` is not claimed here.*

The material is present — `D` is already defined as the self-pairing and the
weight is already computed on it. What is missing is the sentence saying that
the self-pairing is *forced by the dagger under free sealing*, rather than
selected by a phase-neutrality requirement that presupposes the phase.

---

## 8. Falsification

- **S1 false** — a sealed clause of the public interface independently supplies
  a valuation. Then two scalar-formers coexist lawfully, `Mat_{ℝ≥0}` is a model
  again, and ledger invariance reverts to a counted premise with no route.
- **The dagger is not the only state-to-scalar structure** in the sealed pure
  layer, even before the public interface is added. Then §3's uniqueness step
  fails at its first move.
- **`D` is not what public extraction is typed over** — the Born section's use
  of `D` is presentational and the operative typing is over `Ψ`. Kill by
  exhibiting a public extraction in the sealed record defined on `Ψ` directly.
- **R1 inherits the `Q_p` bridge** (§6). This would not make R1 false, but it
  would re-grade it.

---

## 9. Byproduct finding, outside the scope of R1

While tracing the phase input I found a stale claim of the same class as the
ones cleared earlier today, and it is recorded here rather than silently fixed.

The synthesis carries **Proposition (Uniqueness of the Born square)** — every
phase-blind monoidal scalar has the form `|ψ(x)|^{2k}`, with `k > 1` rejected
because it "bundles `k` copies of the same proof/conjugate-proof pair" and
"constructive primeness rejects this duplication."

That is the tensor-word/primeness argument, and XF-1 campaign amendment 1, item
B.5, **retired it**: "vulnerable — it excludes `|z|^p` only within its chosen
language, and historical depth-two does not constrain unary algebraic degree."
The amendment registered a replacement route (Jordan/Hilbert reconstruction plus
frame invariance plus orthogonal additivity over public channels plus continuity
and normalization, giving Cauchy additivity and hence the square). The synthesis
still states the retired argument unqualified, and does not carry the
replacement.

Not repaired here — it is a change to the theory's Born derivation and belongs
to whoever owns that section. Note that R1 does **not** depend on it: §3 needs
only that the public scalar is the self-pairing, not that the exponent is
exactly two.

---

## 10. Summary

| item | status |
|---|---|
| R1 as posed | **discharged** — the public layer is the self-paired layer |
| R1 as an independent premise | **dissolved** — it merges with R3 into S1 |
| Target-2 dependency | **avoided.** The phase route would have imported it; the dagger route does not |
| Phase-blindness | **derived**, relative to the group of unitary scalars; identifying that group is still target 2's |
| Net premise ledger for target 3 | **S1 (free sealing) + target 1's dagger fragment.** Q1(a), Q1(b), R1, R3 are all consequences |
| Ordering claim from the parent note | **stands, unamended** — target 3 needs a fragment of target 1 and nothing of target 2 |
| New exposure | S1, to be attacked as free sealing; plus the inherited cyclicity and presentation debts |

One line: **the conjugate travels with the amplitude into the public API for the
same reason the carrier travels with the group — free sealing adds nothing the
selected structure already supplies.**
