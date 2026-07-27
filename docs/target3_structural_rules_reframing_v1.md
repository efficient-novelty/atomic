# Target 3 in Structural-Rule Vocabulary: One Echo and Three Leads

**Date:** 2026-07-25.
**Grade: leads, not results.** Nothing here is proved. One item is a certified
theorem in the wrong register and is deliberately *not* cashed in.
**Context:** `docs/s1_single_source_audit_v1.md` (S1 audited, repair now applied
to the synthesis), `docs/r1_public_layer_typing_v1.md`,
`docs/target1_ledger_invariance_joint_v1.md`, ledger event 009.
**Occasion:** the theory is a type theory, and the target-3 package had been
stated entirely in operational-category vocabulary. Restating it in the
calculus's own vocabulary is free, and it turned out not to be neutral.

---

## 1. The reframing

| operational statement | structural-rule statement |
|---|---|
| discarding `ε_A : A → I` | **weakening** — adjoining what the process does not use |
| the free environment completion | the **free affine completion**: adding weakening, not contraction |
| no-cloning (sealed proposition) | **no contraction** |
| ledger invariance `ε_{E′}u = ε_E` | **the ledger is stable under weakening** |
| S1 (single source) | **one normalization**, the dagger's, not two |

Two of these are worth spelling out.

**Affine, not cartesian.** A monoidal category with a monoidal discard is exactly
a semicartesian one, and semicartesian is exactly affine — linear structure plus
weakening, without contraction. Adding contraction gives cartesian, which is
classical. So the completion's output is affine; whether it is *also* cartesian
depends entirely on its input. The sealed no-cloning proposition says the pure
layer has no natural diagonal, so the completion cannot land on classical. This
is the same fact recorded from the other side in the parent note — free-completing
a pure deterministic layer returns deterministic maps with garbage discarded,
Bennett rather than Bayes — and it explains that fact rather than merely
reporting it. `Set` is already cartesian, hence already affine, hence its own
completion.

**Ledger invariance is a structural rule.** "Enlarging the environment by an
isometry does not change what the ledger reads" is, in this vocabulary, "the
ledger is stable under weakening." That is about as canonical a shape as a claim
about a type theory can have — which is a reason to expect it to be derivable
from the calculus rather than declared as physics, and a reason to be suspicious
that it currently is not.

The countermodel reads cleanly here too. `u = 2^{-1/2}(1,1)ᵀ` adjoins an ancilla
that is **dagger-normalized but not ledger-normalized**: `σ†σ = 1` while
`ε₂σ = √2`. Two normalizations, which is precisely what S1 forbids.

---

## 2. The echo — and why it is not an argument

Searching the kernel for this structure found it, already implemented and
certified.

`crates/pen-eval/src/typed_families.rs`, in the frozen kernel-v1 operational
definitions:

> The parameter telescope of a clause family is the ordered list of free scope
> references (ambient parameters and prior fields) its normal form **actually
> uses**, renumbered by first use; unused scope entries are dropped **(that IS
> the checked weakening quotient)**.

`crates/pen-eval/src/internality.rs` proves the two inverse laws

```
erase (weaken f) = f        weaken (erase g) = g
```

as typed equalities through the same normalization and univalent equality, with
serialized, replayed witnesses.

So in the **formal register** the statement *the ledger does not count what is
not used* is a certified theorem of the kernel. In the **physical register** the
same statement is ledger invariance — the premise counted in ledger event 009,
currently undischarged.

**This must not be used as an argument, and I am recording it rather than using
it.** The XF-0 census returned blind 3/11 against exactly this kind of
identification, and its standing consequence is that no later item may lean on a
claimed register↔degrees-of-freedom correspondence. The two-register discipline
puts formal-register quantities in sealed testimony, not in physical law, and
amendment F's automated fence would reject a physics derivation that referenced
them without a bridge certificate. An echo between registers is precisely what
that fence exists to stop being cashed.

What the echo legitimately supplies is a **specification of the bridge that would
be needed**, with both sides already formally stated — which is unusual, and is
what would make this a candidate for a *derived* bridge certificate rather than
a declared one. It also supplies the argument shape, which does transport
because it is an argument and not a quantity:

> Two clause families that differ only by unused context are the same family,
> because family identity is what the normal form *does*.
>
> Two records that differ only by unread environment are the same record,
> because record identity is what the record *decodes* — Internal Record
> Actualization clause 2.

That is Route 2 of the original problem statement, and the solution document's
objection to it stands unchanged: *same recoverable content ⇒ same public
denotation* is still not an axiom. The repair to the sealing clauses did not add
it. But the objection is now narrower than it was, because the competing
denotation the countermodel used — an independently posited Born weighting — is
no longer named by the clauses.

---

## 3. Lead: the dagger from path reversal

The philosophical motivation for the whole Born construction is that a proof is
a path *segment* and what is observable is the round trip, out and back. The
formal counterpart of "walking back" is the dagger. So it is worth asking whether
the sealed cubical layer already supplies it: paths have inverses, and `p ↦ p⁻¹`
is contravariant, involutive, and identity-on-objects — a dagger.

**The gap is real and it is instructive.** Path inversion makes every morphism
invertible, so it yields a *groupoid*: a dagger category in which everything is
unitary and `Isom` is everything. That is too much. The completion needs
isometries proper — embeddings that increase the environment — and those are not
invertible.

But note what generates them: an isometry `A → A ⊗ E` is a unitary composed with
an ancilla adjunction, and ancilla adjunction is weakening. So

```
isometries  =  ⟨ groupoid (path reversal) , weakening (ancilla adjunction) ⟩
```

— exactly the two structures already in play in this package, and nothing else.
That is suggestive enough to record.

**The circularity to check before investing.** Ancilla adjunction is an isometry
only for a *normalized* ancilla, and normalization is a pairing condition, which
is what the dagger was supposed to supply. Whether this is a genuine circle or
merely two facets of one structure is the first question, and it should be
settled before any of the above is written down as a route to target 1.

---

## 4. Discipline note: which 2's are earned

Several 2's now sit close together and the temptation to identify them should be
declined explicitly rather than left to be noticed later.

- **Earned.** Grade-2 exactness — every Sorkin coefficient `I_T = 0` for
  `|T| ≥ 3` — follows from conjugate self-pairing alone, as amendment B.1
  records. The 2 of the round trip and the 2 of the interference hierarchy are
  the same 2, and that identity is derived.
- **Earned.** The arity of a round trip is 2 because a circuit is (out, back).
  This is a better account of the Born exponent's origin than the retired
  primeness argument, since it makes 2 an arity rather than a chosen power. It
  does **not** exclude `(ψ†ψ)^k`; `k` independent circuits are also countable,
  and `(ψ ⊗ ψ)†(ψ ⊗ ψ) = (ψ†ψ)²` is reachable. Fixing the function remains
  amendment B.5's additivity route.
- ~~**Barred.** `d_obl = 2` … identifying it with the round-trip 2 is exactly
  the register-crossing the hazard fence prohibits. Noted and declined.~~
  **WITHDRAWN — see `docs/the_two_2s_arity_bridge_v1.md`.** This ruling was
  wrong. `d_obl = 2` is not a fenced quantity (the fence bars φ, Fibonacci
  coefficients, cumulative bars, engine ν totals, and the engine `d²`/`r²`
  laws), and XF-A's caution names width-two and `d_obl = 2` as the *permitted*
  basis for its derivation — so the identification is not a transgression but
  XF-A's own undischarged obligation. Reading the book's `d=2` appendix shows
  the correspondence is also tighter than a coincidence of magnitudes: the
  theorem is already an **arity** bound — no irreducible primitive historical
  field of arity `k ≥ 3` in the μ-minimal public signature — which is the Sorkin
  bound's shape and location in the other register.

---

## 5. What would make any of this real

1. **Settle the circularity in §3** — is ancilla normalization independent of
   the dagger, or the same structure twice? Cheap, and it gates the target-1
   lead entirely.
2. **Attempt the bridge certificate specified in §2.** Both sides are formally
   stated, which is rare. A derived bridge here would discharge ledger
   invariance *and* be the first register-crossing certificate the campaign has
   earned rather than declared. A failure would be worth as much, since it would
   show the two registers' weakening notions are genuinely different.
3. **Re-attempt Route 2** under the repaired clauses. The objection stands, but
   the field is narrower: with no posited weighting in the sealing clauses, the
   remaining question is whether record identity fixes public denotation.

None of these is started. This note exists so that the echo in §2 is on the
record as an echo, before anyone is tempted to write it down as an argument.
