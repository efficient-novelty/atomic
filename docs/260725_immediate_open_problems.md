# Immediate Open Problems — Free Environment Completion

**Date:** 2026-07-25.
**Status:** working problem statement. Nothing here is frozen, sealed, or
registered.
**Parent:** `docs/free_environment_completion_attempt_v1.md`, §5 and §7.
**Scope:** the two items that block target 3 of the Genesis
operational-completion package (*No fundamental mixing*). Problem A is the
blocker. Problem B is the loose end. They are independent and can be worked in
either order, though A is worth more.

---

## 0. Where these sit

The attempt derives purification as free sealing: the mixed layer is the free
environment completion of the sealed pure layer, so every morphism *is* a
dilation and purification is constructional rather than postulated. Two things
stand between that and a result.

| | problem | grade | blocks |
|---|---|---|---|
| **A** | Derive Q1(b), ledger-invariance under isometric enlargement of the environment | genuine research; one real lemma | the whole derivation — without it, choosing isometries is reverse-engineering to dodge the collapse |
| **B** | Prove initiality of `P_⌐` for the abstract sealed layer `P`, not just for the cited concrete instance | probably routine; must actually be written | the claim's grade, not its truth |

Closing both moves target 3 from *bridge premise* to *derived*, which drops the
reconstruction package from four declared premises to three and — with the
dagger dependency already found — replaces "four coordinate targets" with an
ordered structure. Closing A alone is most of the value.

---

## Problem A — ledger-invariance under isometric enlargement

### A.1 Statement

**Informal.** Enlarging the environment by an embedding that loses nothing must
not change what the public ledger reads.

**Formal.** Let `P` be the sealed pure layer, `⌐_A : A → I` the discarding
family of the public interface, and `u : E → E′` an isometry of `P`
(`u† u = id_E`). Prove:

```
⌐_{E′} ∘ u  =  ⌐_E                                            (Q1b)
```

equivalently, in dilation form: for every `f : A → B ⊗ E`, the pairs `(E, f)`
and `(E′, (id_B ⊗ u) ∘ f)` name the same public process.

**What may be used.** Internal Record Actualization; Public Confluence; free
sealing; the sealed pure layer's own structure. **What may not:** any fact about
Hilbert spaces, density operators, or partial traces (see A.6, trap 1).

### A.2 Why it is load-bearing

The unrestricted completion — environments ranging over all of `P`, dilations
identified by *any* environment map — collapses to a single morphism per
hom-set whenever `P` has a zero object with `B ⊗ 0 ≅ 0`. Proof in the parent
note, §4.2, two lines.

So the construction has exactly two possible standings:

- **Q1(b) derived** — isometries are the identification class *because the
  ledger cannot see the difference*, and the completion is forced.
- **Q1(b) not derived** — isometries are the identification class *because any
  larger class collapses the construction*. That is choosing the premise to make
  the conclusion come out, which is precisely what F-XF2 exists to catch. The
  item would then have to be filed as a declared bridge, not a derivation, and
  target 3 stays a premise.

There is no third reading. This is why A is the blocker rather than a
refinement.

### A.3 The gap, stated precisely

Public Confluence, as the synthesis states it:

> compatible public records must admit a joint nondemolition refinement;
> finite-capacity failure is "common seal" or "no common seal," never
> contradictory sealed facts on one overlap.

Internal Record Actualization:

> a formal seal becomes a public event only through a stable decodable internal
> record.

The mismatch is one of **quantification shape**, and it should be named exactly
rather than glossed:

| | Public Confluence | Q1(b) |
|---|---|---|
| ranges over | pairs of records, over a shared overlap | one record, under enlargement of the *unread* part |
| asserts | existence of a joint refinement | equality of two discards |
| failure mode | "no common seal" | a different public fact |

Public Confluence constrains how two records *relate*. Q1(b) constrains how one
record behaves under an operation on the part no record reads. Neither
statement contains the other. The lemma is the bridge between them.

### A.4 Attack routes

**Route 1 — contrapositive through a two-observer construction.** The intended
argument, and the one the parent note gestures at.

Suppose `⌐_{E′} ∘ u ≠ ⌐_E`. Construct two observers sealing the same pure
process `f : A → B ⊗ E`, one recording the environment as `E`, the other as
`E′ ⊇ u(E)`. Show (i) their records are *compatible* in Public Confluence's
sense, (ii) they assign different public facts to the shared `B`-overlap, hence
(iii) no joint nondemolition refinement exists, contradicting PC.

Three sub-lemmas, each of which must be discharged against the theory's own
definitions and not against intuition:

- **A-1 (compatibility).** Are two records of one process into environments of
  different size *compatible*? This requires PC's compatibility relation to be
  pinned down. It is currently used more than it is defined, and this
  sub-lemma is where the route will either work or reveal that PC is
  underspecified. **Do this one first** — it is diagnostic for the whole route.
- **A-2 (verdict is the discard).** A difference in `⌐` must be a difference in
  public fact. Near-definitional if the public process *is* the discard
  composite, but it must be stated, because if the ledger's verdict is some
  coarser function of the discard the argument fails at exactly this step.
- **A-3 (the added subspace carries nothing).** The orthogonal complement of
  `u(E)` in `E′` is never occupied, so by Internal Record Actualization it
  supports no stable decodable record and cannot contribute to a public record.

*Assessment:* the most likely to succeed, and the most likely to produce a
useful negative — if A-1 fails, the finding is that Public Confluence is
underspecified, which is worth publishing on its own.

**Route 2 — directly from Internal Record Actualization, bypassing PC.**
Possibly cleaner, and currently underrated.

A public event exists only via a *stable decodable* internal record. An isometry
is invertible on its image: `u† u = id`. So any decoder `d` for a record held in
`E` yields a decoder `d ∘ u†` for the corresponding record in `E′`, recovering
the same content, and conversely. Decodability is therefore invariant under `u`,
and if the public event is a function of the decodable content, so is the
ledger's verdict.

*Assessment:* shorter and uses only one clause. The soft spot is the last step —
"the public event is a function of the decodable content" — which is close to
what IRA says but may be exactly the same gap relocated rather than closed.
Worth an hour before committing to Route 1, because if it works it is much
cleaner.

**Route 3 — no-unforced-generator argument.** If `⌐` were *not* invariant under
isometric enlargement, then `⌐` would carry data about the environment beyond
what the pure layer's structure supplies — an unforced generator on the public
interface. Free sealing admits none, so `⌐` is determined by the pure structure.

*Assessment:* attractive, and it would keep the whole derivation inside the
realization face. **But check for circularity before investing**: the step from
"determined by the pure structure" to "invariant under isometries" presumes the
pure structure sees environments only up to isometry, which is close to the
conclusion. If that presumption can be discharged from the dagger alone, the
route works; if not, it is question-begging in a way that would be easy to miss
in a write-up.

### A.5 What a solution must satisfy

A proof of Q1(b) counts only if all five hold.

1. **Verdict-blind (F-XF2).** No step consults a measured quantity, and no step
   is motivated by the fact that quantum theory satisfies Stinespring
   uniqueness. The premise must be defensible by someone who has never seen the
   intended model.
2. **Legal at a guarded stage.** Q1(b) is an *invariance*, not a preference. Any
   reformulation that reads "the smallest sufficient environment is selected"
   has silently become a minimality condition and is barred. Check the wording
   of the final statement against this specifically.
3. **No new primitive role.** The clause must specialize Internal Record
   Actualization, Public Confluence, or free sealing. A new selective principle
   is forbidden by the two-role ontology.
4. **Isometries, not unitaries.** See A.6, trap 2.
5. **Stated before it is used.** The lemma is written and frozen before any
   claim about target 3's status changes.

### A.6 Traps

**Trap 1 — importing Stinespring.** "Dilations of a channel are unique up to
isometry on the environment, therefore `⌐` is isometry-invariant." This is a
theorem *about finite-dimensional Hilbert spaces*. Using it here derives the
abstract premise from the intended model — the exact inversion the campaign
constitution exists to prevent, and the most natural wrong move available. The
Huot–Staton concordance recorded in the parent note is legitimate precisely
because the isometry restriction was reached from the collapse argument first;
it must not be retro-fitted into the derivation as an input.

**Trap 2 — proving invariance under unitaries only.** Unitary invariance is
strictly weaker and does not suffice. Composition in `P_⌐` tensors environments,
so environment dimension grows along a composite; comparing dilations of a
single channel across different environment sizes requires non-unitary
isometries. A unitary-only identification still yields a category, but one in
which the same channel with a padded environment is a *different* morphism —
purification uniqueness fails, and the completion is not the intended one. This
near-miss will typecheck and will look like a result.

**Trap 3 — proving it for the intended model and calling it general.** `FdHilb`
satisfying Q1(b) is not evidence that the sealed layer does. State the
hypotheses on `P` the proof actually uses; that list is also the answer to
Problem B.

**Trap 4 — quietly redefining `⌐` so Q1(b) holds by construction.** If the
discard is *defined* as an isometry-invariant assignment, Q1(b) is vacuous and
the API premise has absorbed the content. Q1(a) and Q1(b) must remain
separable, with (a) supplying a discard and (b) constraining it.

### A.7 Negative resolution and what it would mean

If Q1(b) is shown *not* to follow — Public Confluence and Internal Record
Actualization are jointly silent about discard behavior under environment
enlargement — the honest outcomes are, in order of preference:

1. **The clause is filed as a declared bridge premise**, counted, and target 3
   remains a premise rather than a derivation. The reconstruction package keeps
   its four declared premises. This is a publishable negative and costs the
   theory nothing it currently claims.
2. **Public Confluence is found underspecified** (the A-1 outcome), which is a
   finding about the synthesis itself and outranks the purification question.

What is *not* available is retreating to "the isometry class is chosen because
larger classes collapse the construction." That is a stipulation dressed as a
derivation, and it should be refused explicitly if anyone proposes it,
including a future session of this project.

---

## Problem B — initiality of `P_⌐` for general `P`

### B.1 Statement

The parent note constructs `P_⌐` and verifies it is a well-defined monoidal
category with discarding. It **asserts** initiality on the strength of Huot and
Staton's completion theorem, which is proved for a specific concrete instance:
finite-dimensional vector spaces and isometries, completing to CPTP maps between
finite-dimensional C\*-algebras.

Prove, for the abstract sealed layer:

> `η : P → P_⌐` is initial in the category of exact realizations of the Q1 API —
> triples `(M, F, ⌐^M)` with `M` symmetric monoidal, `⌐^M` a monoidal discarding
> family, `F : P → M` strong symmetric monoidal, and `F` sending isometries of
> `P` to `⌐^M`-causal maps of `M`.

and identify the minimal hypotheses on `P` under which it holds.

### B.2 Why this is the loose end and not the blocker

The proof shape is visible and looks routine. Define

```
G(A) := F(A),        G[E, f] := (id_{FB} ⊗ ⌐^M_{FE}) ∘ F(f).
```

- **Well-defined.** If `f′ = (id_B ⊗ u) ∘ f` for an isometry `u`, then
  `G[E′, f′] = (id ⊗ (⌐^M_{FE′} ∘ F u)) ∘ F(f) = (id ⊗ ⌐^M_{FE}) ∘ F(f) = G[E, f]`,
  using exactly the realization category's causality clause on `F u`. Note this
  is **Q1(b) transported into `M`** — the two problems meet here, which is worth
  noticing but does not make them the same problem.
- **Functorial.** Needs monoidality of `⌐^M` and of `F`; the composite
  `[F, g] ∘ [E, f] = [F ⊗ E, (g ⊗ id_E) ∘ f]` should discharge against
  `⌐^M_{F ⊗ E} = ⌐^M_F ⊗ ⌐^M_E`.
- **Unique.** Every morphism of `P_⌐` is a discard composite of an `η`-image, so
  any comparison functor agreeing with `F` on `η` and preserving `⌐` agrees
  everywhere.

So B is a page of diagram-checking plus a hypothesis audit, not a research
programme. It is listed because it is currently *asserted*, and an asserted step
in a chain whose whole point is derivation-versus-declaration is not acceptable
at this project's standard.

### B.3 The hypothesis audit — what `P` must supply

Each to be checked against what Genesis actually sealed, not assumed:

1. **A dagger**, so that "isometry" is definable at all. Already known to be
   required (parent note §4.2). This is the target 1 dependency.
2. **Symmetric monoidal structure**, for the environment swap used in the
   tensor of two classes.
3. **Existence of the quotient.** The congruence check is done; what remains is
   whether the colimit is well-behaved for the relevant `P` — in particular
   whether the environment diagram is filtered enough that the classes compose
   without further identification.
4. **Finite coproducts / distributivity.** The cited completion is stated over
   distributive monoidal categories. Either the sealed layer supplies direct
   sums, or the completion must be shown to add them freely, or the abstract
   theorem must be proved without that hypothesis.
5. **Zero object interaction.** The unrestricted completion collapses via the
   zero object. Verify explicitly that the isometric completion does not: there
   is no isometry from a nonzero `E` into `0`, so the collapse argument has no
   purchase — but this should be written down rather than assumed, since it is
   the hinge that distinguishes the two constructions.

### B.4 Two structural facts to record while proving it

- **`P_⌐` is not a dagger category.** The completion destroys the dagger — its
  morphisms are not reversible. This is expected and correct (channels are not
  unitaries), but it means the dagger is consumed at this rung and is not
  available downstream in the completed layer. Worth stating explicitly, since
  the reconstruction package's remaining targets may want it.
- **The fork from the parent note §4.3 is decided here.** Whether the completion
  lands on the sub-causal category (unit not terminal, effects = positive
  operators) or on the causal one (unit terminal, channels) depends on whether
  the pure layer's sealed morphisms are all pure maps or only the causal ones.
  This is answerable by reading the Genesis record, and should be looked up
  rather than argued.

---

## 1. Order of work

1. **Route 2 first** (A, from Internal Record Actualization alone) — cheapest,
   and if it lands, the shortest possible derivation.
2. **Sub-lemma A-1** (is PC's compatibility relation defined sharply enough to
   cover records into different-sized environments?) — diagnostic. Its failure
   is itself a result.
3. **Route 1** in full if Route 2 stalls.
4. **Problem B** — write the diagram-checking and the hypothesis audit. Do this
   even if A is unresolved: B's hypothesis list is independently useful, since
   it is the specification of what the sealed pure layer must supply, and item 1
   of that list is the target-1 dependency already found.
5. **Route 3** only after checking it is not circular.

## 2. Standing constraints on all of the above

- Nothing produces a number, so the work is verdict-blind by construction; but
  the *motivation* for each premise must still be defensible without reference
  to the intended model.
- Every clause must specialize an existing principle. No new primitive role.
- No minimality, no cost, no ranking — at any step, including in the informal
  motivation. The whole point of routing purification through free sealing is
  that initiality is admissible at a guarded stage where minimization is not.
- If an item fails, it is published as a failure and the corresponding target
  reverts to a counted bridge premise. Silence is the one unavailable outcome.
