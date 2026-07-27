# Target 1 and Ledger Invariance, Jointly

**Date:** 2026-07-25.
**Status:** derivation route found. One new premise (R1), one debt reassigned to
target 1, and the counted premise of ledger event 009 becomes dischargeable
conditional on both.
**Context:** ledger events 009 (ledger invariance counted as a bridge premise)
and 010 (standing provenance disclosure);
`docs/free_environment_completion_attempt_v1.md`;
`docs/260725_immediate_open_problems_solution.md` (the countermodel).
**Register independence:** this item consumes the Selective Law's realization
face and the already-sealed Born self-pairing. It does not consume the certified
fifteen-stage register, so the standing disclosure does not bear on it and PA-1
will not re-grade it.
**Verification:** the model computation in §4 was run, not quoted.

---

## 1. Result

Event 009 registered **ledger invariance** — `ε_{E′} u = ε_E` for every isometry
`u` — as a counted bridge premise, on the strength of a countermodel showing it
independent of Internal Record Actualization, Public Confluence, monoidal
discarding and free sealing.

That independence result stands. What follows is not a refutation of it but the
outcome its own scoping clause anticipated: the abstract clauses do not force
ledger invariance, and the *specific structure* does.

> **Claim.** Suppose (R1) the public layer is the self-paired layer, (R2) the
> pure layer carries a dagger under which self-paired objects are self-dual, and
> (R3) the discard is free-sealed rather than independently posited. Then the
> discard is the canonical dagger pairing, monoidal discarding Q1(a) is
> *derived* rather than assumed, and
> ```
> Causal(P, ε)  =  Isom(P)
> ```
> — an equality, not an inclusion. Ledger invariance is then a theorem.

Three things follow that are worth more than the equality itself.

- **Both halves of the Q1 API come from one source.** The attempt note treated
  Q1(a) (there is a discard) and Q1(b) (it is isometry-invariant) as two clauses
  of a selected API. Under R1–R3 they are one clause: the dagger's canonical
  pairing, which is monoidal and isometry-invariant because it is a pairing.
- **The dependency refines from targets to fragments.** Target 3 does not
  require target 1. It requires exactly target 1's *dagger / state–effect
  correspondence* fragment, and nothing of its spectral, frame-symmetry, filter
  completeness, or Jordan content. The reconstruction package's ordering is a
  partial order on fragments, not on whole targets.
- **The countermodel is diagnosed rather than merely blocked.** §2.

---

## 2. What the countermodel was actually exploiting

`Mat_{ℝ≥0}` with `ε_n = (1,…,1)` is a dagger symmetric monoidal category with a
monoidal discard that is invariant under every unitary and not under every
isometry. The diagnosis in the solution document — an ℓ²-dagger against an
ℓ¹-discard — is right, and it can be made structurally exact:

> **The countermodel's discard is not the dagger's pairing. It is an
> independently chosen effect on an undoubled layer.**

In a compact dagger category the canonical pairing is the cap
`A* ⊗ A → I`. It is a map *out of a self-paired object*. The countermodel's
`ε_n : n → 1` is a map out of `n`. These live on different objects — verified in
§4: for `n = 3` the cap has domain dimension `9`, the discard has domain
dimension `3`. There was never a mismatch between two candidate discards on one
object; there was a discard on an object that carries no canonical one.

That is the whole of the failure, and it tells you exactly what to repair: put
the public layer where the canonical pairing lives.

---

## 3. The derivation

### 3.1 R1 — the public layer is the self-paired layer

> **Premise R1.** The objects the public interface discards over are the
> self-paired objects `Â = A* ⊗ A`, not the pure carriers `A`.

R1 is new, and it is where this attempt is exposed. But it is argued from
content the theory has already sealed rather than from the intended model.

Public weights are phase-blind: the synthesis derives them by pairing each proof
amplitude with its conjugate, and XF-A's registered derivation rests on
conjugate self-pairing alone. A phase-blind layer is a self-paired layer — the
invariants of the global phase action on pure carriers are exactly the doubled
objects. So R1 says only that the interface which records phase-blind facts is
defined on the objects that carry phase-blind facts.

What R1 is *not*: it is not a claim that mixed states are primitive, and it is
not a minimality condition. It is a statement about which objects the discard is
typed over.

### 3.2 R2 — the dagger, and where the debt sits

> **Premise R2.** The pure layer carries a dagger under which self-paired
> objects are self-dual, so the cap `ε_Â : Â → I` exists.

R2 is not new debt. It is target 1's existing debt, now precisely scoped: of
everything target 1 promises — spectral decomposition, positive dagger /
state–effect correspondence, frame symmetry, constructive filter completeness,
homogeneous self-duality, Euclidean Jordan structure — target 3 needs only the
dagger with self-duality. Anyone working target 1 can deliver that fragment
first and unblock target 3 without finishing Jordan.

### 3.3 R3 — free sealing chooses the discard

> **Premise R3.** The discard is free-sealed against the selected API rather
> than posited independently.

Given R1 and R2 the pairing already exists, supplied by structure the pure layer
has. Positing a *different* effect as the ledger's discard adds a generator that
no clause of the API forces and no live obligation discharges. Free sealing
admits no unforced generators, so the canonical pairing is the only lawful
discard.

This is the same move as the repaired empty-desert constraint, and it is legal
at a guarded stage for the same reason: it is a universal-property argument, not
a comparison of costs. Note what it is not — it is not "the canonical discard is
simpler" or "cheaper". It is that the alternative is unforced.

### 3.4 Q1(a) is derived, not assumed

The cap is monoidal: `ε_{Â ⊗ B̂} = ε_Â ⊗ ε_B̂` and `ε_I = id_I`, given that
doubling is monoidal (`Â ⊗ B̂ ≅ (A ⊗ B)^`). So the discarding clause the attempt
note took as premise Q1(a) is a consequence of R1–R3 rather than an axiom.

### 3.5 The theorem

> **Theorem.** Under R1–R3, for a pure morphism `f : A → B` with doubling
> `f̂ : Â → B̂`,
> ```
> ε_B̂ ∘ f̂  =  ε_Â     ⟺     f† f = id_A .
> ```
> Hence `Causal(P, ε) = Isom(P)`, and ledger invariance holds.

**Proof.** `ε_B̂(f̂ ρ) = ⟨pairing⟩(f ρ f†) = ⟨pairing⟩(f† f ρ)` by cyclicity of
the canonical pairing, which holds in any compact closed category. If
`f† f = id` this is `ε_Â(ρ)`, giving causality. Conversely, if
`⟨pairing⟩((f† f − id) ρ) = 0` for every state `ρ`, then `f† f = id` by
nondegeneracy of the pairing on the positive cone. ∎

The converse direction is the part worth noticing. It is not merely that
isometries are causal; **the two classes coincide**, so the class `J` the
completion needs is not a choice at all — it is determined by the discard, which
is determined by the dagger.

### 3.6 Corollary — target 3 closes

By event 009's realization theorem, `Env_J(P)` is initial among exact
`J`-realizations for any wide symmetric monoidal class `J`. Under R1–R3 the class
is forced to be `Isom(P) = Causal(P, ε)`. So the environment completion is the
free seal outright, and purification is derived rather than conditional.

---

## 4. Verification in the intended model

Computed, not quoted (`numpy`, complex finite dimensions):

| check | result |
|---|---|
| `V : C² → C³` an isometry, `ρ` a random state | `Tr(VρV†) = 1.0000000` against `Tr ρ = 1` |
| `M : C² → C³` not an isometry | `Tr(MρM†) = 1.362` against `Tr ρ = 1` — causality fails |
| nondegeneracy, i.e. causal ⇒ isometry | `sup over 200 random states of |Tr(M†Mρ) − Tr ρ| = 1.058`, bounded away from zero |
| the countermodel's discard is not the cap | cap on `n = 3` has domain dimension `9`; `ε₃` has domain dimension `3` |

The third row is the converse direction: a non-isometry is not merely
occasionally non-causal, it is uniformly separated, which is what nondegeneracy
of the pairing delivers.

---

## 5. Relation to the independence result of event 009

No contradiction, and the relationship should be stated precisely because it is
easy to misreport.

Event 009 proved: ledger invariance does not follow from Internal Record
Actualization, Public Confluence, monoidal discarding and free sealing **as
abstractly stated**. That proof is untouched. Its scoping clause said: whether
the specific sealed layer satisfies ledger invariance by a theorem using its own
structure is open.

This note supplies such a theorem, under three added hypotheses. The countermodel
fails **R3**: its `ε` is an independently chosen effect where a canonical pairing
was available on the doubled layer, so it is not a model of a free-sealed
discard. It also fails R1, being undoubled.

So the honest ledger reading is not "the premise is discharged" but:

> Ledger invariance remains a counted premise. A derivation route now exists that
> discharges it against R1 plus target 1's dagger fragment, replacing one
> unstructured premise with one structured premise (R1) and one existing debt.

Whether that is progress depends entirely on whether R1 is more defensible than
Q1(b), which is §6's question and should not be assumed.

---

## 6. Remaining gaps

**R1 is the new exposure, and it is the item to attack.** The argument from
phase-blindness to self-pairing is good in the intended model — the phase
invariants of pure carriers are exactly the doubled objects — but the abstract
step "public records are phase-blind, therefore the public layer is self-paired"
needs the phase group and its invariants pinned in the sealed layer. If the
sealed layer's phase structure is itself target 2's business, R1 imports a second
fragment dependency and the ordering claim in §1 needs amending.

> **Resolved — `docs/r1_public_layer_typing_v1.md`.** R1 is discharged, and the
> route above is *not* the one that works: the phase labeling `θ_x → U(1)` is
> posited in the synthesis's definition of a preparation, which is exactly what
> target 2 owes, so the phase route would have imported target 2. The route that
> works runs the other way — the dagger is the only structure converting a state
> into a scalar, and it converts it into `ψ†ψ`, so the public layer is
> self-paired and **phase-blindness is a consequence** (relative to the group of
> unitary scalars; identifying that group as `U(1)` remains target 2's).
> Consequently R1 and R3 are one clause, not two: **S1 — the public interface
> introduces no scalar-producing structure beyond what the dagger supplies.**
> The ordering claim in §1 stands unamended, and the premise ledger for target 3
> reduces to S1 plus target 1's dagger fragment.

**R2 is target 1's, and is now scoped.** Deliver the dagger with self-duality on
the doubled objects; the spectral and Jordan content is not needed here.

**Cyclicity is a hypothesis, not free.** The proof uses cyclicity and
nondegeneracy of the canonical pairing. These hold in compact closed categories.
Whether the sealed layer is compact closed — as opposed to merely monoidal with a
dagger — must be checked and not assumed. If it is not, the pairing may exist
without cyclicity and the theorem fails at its first line.

**One trap, recorded.** The proof in §3.5 is written in the intended model's
notation (`Tr`, `f† f`). It must be rewritten in the abstract graphical calculus
before it counts, because as written it is vulnerable to exactly the objection
raised against Stinespring in the parent problem statement: importing the model
to derive the abstract premise. The *content* is abstract — cyclicity and
nondegeneracy — but the *presentation* is not, and the distinction has already
cost this program once.

---

## 7. Falsification

- **R1 false** — the public interface is typed over pure carriers, not doubled
  objects. Then no canonical discard exists, R3 has nothing to select, and the
  countermodel is not repaired. Kill by exhibiting sealed content that types the
  ledger over undoubled carriers.
- **The sealed layer is not compact closed.** Then cyclicity is unavailable and
  §3.5 fails immediately.
- **R3 is circular** — if "free-sealed discard" is read so that only an
  isometry-invariant discard counts as free-sealed, the argument assumes its
  conclusion. Check: the argument must run from *existence of a canonical
  pairing* to *uniqueness of the lawful discard*, never from invariance to
  invariance.
- **The converse direction fails abstractly.** Nondegeneracy of the pairing on
  the positive cone is used to get causal ⇒ isometry. In a layer where the
  pairing degenerates, `Causal ⊋ Isom`, the class `J` is larger than the
  isometries, and the completion changes — though note it would still be *a*
  wide monoidal class, so event 009's realization theorem survives and only the
  identification of `J` fails.

---

## 8. Summary

| item | status |
|---|---|
| Ledger invariance derivable from structure? | **Yes, under R1–R3** — and as an equality `Causal = Isom`, not an inclusion |
| Q1(a), monoidal discarding | **derived**, not assumed, from the same source |
| Does this contradict event 009? | **No.** That independence is from the abstract clauses; the countermodel fails R1 and R3 |
| Net effect on the premise count | one unstructured premise (Q1(b)) → one structured premise (R1) + one existing debt (target 1's dagger fragment) |
| Ordering finding, refined | target 3 needs only target 1's **dagger / state–effect fragment**, not its Jordan content — the package orders on fragments |
| New exposure | **R1**, and the compact-closedness hypothesis behind cyclicity |
| Presentation debt | §3.5 must be rewritten in the abstract calculus before it counts |

The one-line verdict: **ledger invariance is not an independent bridge premise —
it is the dagger's pairing seen from the ledger side — provided the ledger is
typed over the self-paired layer, which is now the thing to prove.**
