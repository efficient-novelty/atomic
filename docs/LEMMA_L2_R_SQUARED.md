# Lemma L2: The r² Transport Matrix

**Status:** proof sketch complete (2026-07-02), same rigor level as L1
(`LEMMA_L1_D_SQUARED.md`) and the Telescopic Elimination theorem. Three steps
flagged for mechanization, one of them genuinely delicate (§5.1). The κ = 1
carve-out in the engine is *explained* rather than anomalous (§3). Companion to
`EVALUATOR_DERIVATION.md` (principle P5-morphism).

**Machine-check clarification (2026-07-18):** the fibration argument remains a
proof sketch. The finite basis `(Fin 2 x Fin kappa) + (Fin r x Fin r)` is
checked conditionally in Agda and explicitly enumerated in Rust. Rust's
semantic fields are trusted caller assertions, not proof objects, while Agda
requires a complete schema/basis isomorphism. Face/transport disjointness,
transport staging, nontrivial monodromy, and the propositional-inverse
obstruction remain unmechanized.

---

## 1. Statement

**Lemma L2.** Let B be a library of sealed strata and let X be a *weave*: a
Map-class extension of κ ≥ 2 charged morphism-type clauses whose reference set
R = {L₁,…,L_r} (r distinct sealed strata) is strongly connected by the weave's
reference graph, charged as a single fibration-style bundle (structural unity),
and definitionally nontrivial (not trivializable over B — else it is redundant
or composite and never reaches scoring). Then the new eliminative and
cross-interface schemas (the ν_C ledger, per D1–D3) number exactly

    ν_C(X) = 2κ + r²

consisting of:

- **Part A — the face pairs** (2κ): for each charged clause, its *forward face*
  (the clause as used: project, include, classify) and its *lifting face* (the
  pullback/lifting action the clause acquires by participating in a fibration);
- **Part B — the transport matrix** (r²): for each ordered pair
  (i, j) ∈ R × R, the interface-level transport family T_{i,j} carrying
  Lᵢ-indexed data to Lⱼ-indexed data through the sealed weave; diagonal entries
  T_{i,i} are the weave's *monodromy families*, off-diagonal entries its
  *directed couplings*.

Moreover ν_G(X) = 0: a weave declares no new type former — every clause
inhabits an arrow type formed from sealed strata by sealed formers, so no new
formation schema exists; the clauses' usable introduction content is exactly
their forward faces, already counted in Part A. (Counting a per-clause
"declaration" schema in addition would double-count the forward face.)

**Verification against the trace:** Step 9 (Hopf): κ = 4, R = {S¹, S², S³} so
r = 3; ν_C = 8 + 9 = 17 = ν ✓, with ν_G = ν_H = 0 ✓ — the pure-interaction row.

## 2. Proof

**Part A: the face pairs (2 per clause; FORCED given the fibration reading).**

*Availability.* Each charged clause f participates in the bundle in two
directions. Forward: the clause as a usable map — its application family
(project total-space data to the base; classify base data; include fiber data).
Lifting: because the weave is charged as a *fibration structure* rather than a
bag of arrows, each clause acquires a transposed action — path lifting against
the projection, pullback of families along the classifying map, fiberwise
extension along the inclusion. The lifting face is not λ-definable from
application: lifting and pullback are Kan-theoretic content supplied by the
fibration clauses jointly, which is precisely what the candidate's charged
structural unity buys. Both faces are new over B (before the weave, the strata
were mutually inert: no B-schema carried data across them).

*Independence.* Forward and lifting faces have different variances and
different judgment forms (map application vs. family pullback); no definitional
rule interderives them (an inverse or section is nowhere charged).

*Exhaustiveness per clause.* Any further per-clause action is λ-composite of
the two faces with sealed structure: pre- and post-composition are λ-definable
from application; iterated lifts stage. Two faces per clause, no more.

**The κ = 1 carve-out, explained.** A single bare arrow (κ = 1) is not a weave:
no bundle structure is charged, so there is no lifting face — the only
eliminative family is application, and the declaration itself is the one
introduction. The engine's special case (ν_G, ν_C) = (1, 1) for κ = 1 is
therefore not an anomaly but the degenerate limit of the same principle: faces
exist pairwise only where fibration structure pays for lifting. That the
carve-out and the general formula cohere under one reading is evidence the
formula was tracking a principle before the principle was articulated.

**Part B: the transport matrix (r²).**

*Availability.* The sealed weave exports, at the interface level, transport of
strata-indexed data through the whole bundle — families that use projection,
lifting, and identification *jointly* and are therefore not per-clause content:

- Diagonal T_{i,i} (r monodromy families): strong connectivity provides, for
  each stratum Lᵢ, a weave-cycle through the other strata and back; the induced
  self-transport on Lᵢ-indexed data is the weave's holonomy at i (for Hopf: the
  action of base loops on fibers — the monodromy that *is* the fibration's
  nontriviality). Nontriviality of the weave guarantees these are not
  definitionally the identity: a weave with definitionally trivial
  self-transport is trivializable, hence definitionally composite or redundant,
  hence inadmissible — **admissibility itself certifies the diagonal's
  availability.** Selection and counting cohere.
- Off-diagonal T_{i,j}, i ≠ j (r(r−1) directed couplings): transport of
  Lᵢ-indexed data to Lⱼ-indexed data through the bundle (lift base data to the
  total space; push fiber data along inclusion-then-projection routes; classify
  total data over the base). Strong connectivity provides a route for every
  ordered pair; the induced family is new because the strata were inert in B.

*Independence.* Distinct ordered pairs have distinct source/target judgment
forms. The directed pairs (i,j) and (j,i) are related only by inverting
fibration transport, and **fibration transport has no definitional inverse** —
lifts are unique only up to a propositional path. This is the L1 transpose
obstruction in fibration dress: ordered pairs do not collapse. Diagonal
families are automorphism-valued, off-diagonal are map-valued: different
arities. Independence of Part B from Part A is the delicate step (flagged
§5.1): Part A families are *clause-indexed*, natural in ambient terms with one
clause as parameter; Part B families are *stratum-pair-indexed*, with the whole
sealed interface as parameter and no single clause sufficient to express them
(each T_{i,j} uses at least two faces jointly, or the fibration identification);
their judgment forms differ (clause action vs. interface transport).

*Exhaustiveness.* Any interface-level schema of the weave involving strata
(i₁, …, i_k) with k ≥ 3 factors through pairwise transports by staging:
transport composes, and composites of the T-families are definitional once the
pairwise ones are sealed (the union-staging analogue; D2's pairs-at-most
realized by the transport algebra itself). Schemas involving a non-referenced
stratum factor through the reference set — the weave's public surface speaks
only its references. Schemas not involving the weave are old. Hence exactly
r² interface families. ∎

## 3. Rival Counts Refuted

- **r(r−1) (no diagonal):** denies the monodromy families — but the diagonal is
  the weave's nontriviality certificate. Any weave that *survives admissibility*
  has live diagonals (§2.B); r(r−1) undercounts exactly the schemas that make
  the candidate selectable rather than redundant. Refuted by the admissibility
  architecture itself, independently of the trace.
- **C(r,2) or C(r,2)+r (undirected):** requires collapsing (i,j) with (j,i),
  i.e. treating fibration transport as definitionally invertible. It is
  propositionally invertible at best (and for general charged maps, not at
  all). Same obstruction class as L1's transpose step.
- **κ·r or 2κ·r (per-clause-per-stratum):** counts *instances* rather than
  natural families — each clause face is already one schema natural over all
  ambient data (D1); multiplying by strata violates the schema/instance
  distinction that the whole ledger rests on.

## 4. The Unified P5, and the Quadratic Shadow Again

**Unified connectivity principle (P5, final form).** *An extension's
cross-interface surface is the depth-2 closure of its reference structure under
the extension's own operations.* A record/telescope's operation is *access*:
fields do not compose, so the closure of k references is a spanning fusion —
k − 1 bridges plus the inherited deepest surface (the proven Telescopic
Elimination theorem). A weave's operations are *transport and lifting*: arrows
compose, so the closure of r strongly-connected references is the full ordered
matrix — r². One principle, two operation algebras, two counts; the
record/weave dichotomy is the constructor kind (P5's case split), not a choice.

And the quadratic shadow acquires its fourth incarnation. The table from
`LEMMA_L1_D_SQUARED.md` §4 extends by one column: directed object = the
reference set R under weave transport; pairing = source stratum × target
stratum; diagonal = monodromies (again — the same word is forced in both
lemmas, which is not an accident: both diagonals are "carry it around and see
what returns"); off-diagonal = directed couplings; higher orders freed by
transport staging; quadratic signature = r². Born's exponent 2, the d² Kan
matrix, the r² transport matrix, and the two-term Fibonacci recurrence: four
ordered self-pairings, one depth-2 window. At this point D2 (pairs-at-most
counting) is no longer a proposed sharpening of the definitions; it is the
load-bearing wall that four independent structures lean on.

## 5. Flagged for Mechanization

1. **Part A / Part B disjointness (the delicate step).** No definitional
   interderivability between clause-indexed face families and stratum-pair
   transport families. The informal argument (different judgment forms; T_{i,j}
   requires ≥ 2 faces jointly) must be made exact in MBTT. This is L2's
   analogue of L1's transpose obstruction, and it is *harder*: a referee's
   sharpest version is "isn't T_{i,j}, for a directly-linked pair, just the
   forward face of the linking clause?" The answer (interface transport uses
   the fibration identification, not bare application) must be a theorem, not
   a distinction.
2. **Propositional-inverse obstruction:** verify that no charged weave clause
   family acquires a definitional inverse in MBTT (equivalence-typed clauses
   would partially collapse the matrix and the count would drop; the selected
   Hopf AST charges maps and a witness, not equivalences — state as a
   hypothesis of the lemma, parallel to L1 §5.1).
3. **The small anchor (honest status: incomplete).** L1 had the circle — a
   trace-independent hand count where the rivals disagree. L2's cleanest
   candidate anchor is a Möbius-style two-stratum bundle (base S¹, fiber
   stratum F; κ = 2: projection + twist clause; formula predicts
   2·2 + 2² = 8). Hand enumeration finds the four faces and plausibly the four
   transports (base monodromy, fiber flip, base-on-fiber action, fiber-descent)
   — but the independence of T_{F,F} (fiber flip) from T_{B,F} (monodromy
   action evaluated at the generator) is exactly the §5.1 disjointness in
   miniature. The anchor therefore *localizes* the delicate step rather than
   discharging it, which is itself useful: mechanize the Möbius case first,
   and the general lemma follows the same seam.

## 6. Consequences for the Ledger

- `EVALUATOR_DERIVATION.md` P5-morphism upgrades to **PROVEN (sketch),
  mechanization pending**, with the three flags above; P5 is restated in its
  unified closure form (§4).
- ν_G = 0 for weaves and the κ = 1 carve-out move from GAUGE-adjacent to
  explained-by-principle.
- The evaluator's remaining genuinely open item is now exactly one: the
  **`(post+1)/2` H-space chaining term** (one load-bearing exposure, Step 8,
  margin 0.17). It should be settled not by lemma but by hand enumeration of
  the H-space coherence obligations under P2, with the code changed to match
  whatever the enumeration says, followed by the strict-lane rerun (V2c).
- V-program additions: V2c (H-space enumeration + rerun), and the Möbius
  anchor joins V1's direct-enumerator test set as its first target.

## 7. Machine-Checked Finite Layer (2026-07-18)

The Rust checker verifies `kappa >= 2`, distinct nonempty references,
well-formed submitted edges, and strong connectivity. Conditional on explicit
semantic assertions, it checks the disjoint face/diagonal/off-diagonal basis
and total `2 * kappa + r^2`. The Möbius total 8 and Hopf total 17 tests are
conditional arithmetic anchors only. In particular, the Hopf test submits a
cycle over extracted reference names; it does not derive a fibration graph or
charged semantics from the telescope.

The Agda theorem `l2-conditional-cardinality` makes the complete boundary
precise: callers must provide classifier and realizer maps with both inverse
laws. The current AST has no `coe`, `hcom`, fibration witness, or definitional
equality quotient from which to construct those maps, so the obligations in
Section 5 remain open.
