# Lemma L1: The d² Law

**Status:** proof sketch complete (2026-07-02), at the rigor level of the paper's
Telescopic Elimination theorem. Two steps flagged for mechanization. One new
off-trace discrepancy discovered in the engine (§6). Companion to
`EVALUATOR_DERIVATION.md` (principle P4).

**Machine-check clarification (2026-07-18):** the cubical argument remains a
proof sketch. `agda/CountingLemmas.agda` checks the cardinality theorem from an
explicit schema/basis isomorphism, and
`crates/pen-eval/src/counting_lemmas.rs` enumerates the finite basis tags. No
MBTT instance of that isomorphism is supplied. Availability, the transpose
obstruction, exhaustiveness, and union-staging therefore remain semantic proof
obligations; this work does not prove that Genesis halts after Step 15.

---

## 1. Statement

**Lemma L1.** Let B be a library containing the dependent core (Π/Σ), path
structure with Kan operations (coercion `coe` and homogeneous composition
`hcom` over a cofibration lattice supporting union-staging, as in CCHM-style
cubical type theory), and let X extend B by a **single path constructor p of
dimension d ≥ 1** — a generator of the d-fold loop space Ω^d(A, a) with constant
boundary, where A is the type being declared and no nontrivial d-cell of that
boundary type is derivable in B. Then the new *computation schemas* (the ν_H
ledger, per definitions D1–D3 of `EVALUATOR_DERIVATION.md`) number exactly

    ν_H(X) = 1 + d²

consisting of:

- **β** — the eliminator's computation rule on p (1 schema);
- **K_a** for a ∈ {1,…,d} — the *monodromy schemas*: the transport automorphism
  across p in principal direction a (d schemas);
- **K_{a,b}** for ordered pairs a ≠ b — the *variation schemas*: the
  b-directional variation of the a-monodromy, i.e. the naturality filler of
  K_a across the cell in direction b (d(d−1) schemas).

d monodromies + d(d−1) variations = d² Kan schemas; plus β.

## 2. Proof

**(i) Availability — each listed schema exists and is new.**

Fix a ∈ {1,…,d}. Holding the other d−1 interval variables fixed, the constant
boundary makes p a *loop family* in direction a: endpoints definitionally agree.
Hence for any dependent family E over the ambient type, `coe` along that loop is
an automorphism of the fiber — the monodromy K_a. It is new over B: by
hypothesis every d-cell of p's boundary type derivable in B is definitionally
degenerate, so every B-derivable transport across such cells is definitionally
the identity, while K_a with cap p is not (it is the first nontrivial holonomy
the library possesses in this type — for d = 1 this is exactly the winding
transport of the circle, the schema whose existence nobody doubts).

For b ≠ a: K_a is a family over the b-th interval; its dependence on b is a
path of automorphisms, whose filler — the naturality square of transport
across the cell — is the schema K_{a,b}. New for the same reason.

β is the defining computation of the eliminator applied to p — a schema of a
different judgment form (reduction of an eliminator application, not a Kan
filler), present for every constructor by the universal property (principle P3).

**(ii) Independence — no two are definitionally interderivable.**

β vs. any K: different judgment forms. K_a vs. K_{a′}: distinct principal
directions; the only candidate identification is the interval swap, and this is
the load-bearing step:

> **The transpose obstruction.** The swap of interval variables is a symmetry
> of the cube category, but it acts on the *generator* as transposition
> p ↦ pᵀ, and for a declared HIT constructor pᵀ is not definitionally p —
> their identification (e.g. orientation reversal for sphere generators) is at
> best a propositional path. Definitional completion (D3) makes only
> *definitionally* interderivable schemas free. Hence schemas indexed by
> distinct ordered pairs remain independent: K_{a,b} and K_{b,a} vary
> different monodromies and are related only through the propositional
> transposition.

K_{a,b} vs. K_a: different judgment arities (automorphism vs. path of
automorphisms). ∎(ii)

**(iii) Exhaustiveness — nothing else at depth 2.**

Let S be any new computation schema involving p. Cases:

1. *Kan fillers with tube constraints on two or more directions* (the higher
   fillers a referee expects to enlarge the count for d ≥ 3): by
   **cofibration union-staging** — `hcom` over a tube φ₁ ∨ φ₂ decomposes into
   staged compositions over φ₁ then φ₂ in CCHM-style cofibration lattices —
   every multi-tube filler is a definitional composite of single-tube fillers.
   Free by definitional completion. Note what this means structurally: the
   pairs-at-most discipline (D2) is not *imposed* on the count here; the
   cubical algebra itself stages every higher interaction through pairwise
   ones. Depth-2 counting is the fixed point of the Kan algebra's own
   compositionality.
2. *Functorial images* ap_f(p): reduce via β (for f defined by the eliminator)
   or via the definitional action of ap on cubical terms. Composite; free.
3. *Inversion, whiskering, self-composites* p ⋆ p: definitional `hcom`
   composites of listed schemas. Free.
4. *Schemas pairing p with other library generators*: cross-interface content —
   the ν_C ledger by the ν_H/ν_C split (self/computation vs. interaction),
   counted there and only there. No leakage in either direction.
5. *Schemas not involving p*: old. ∎(iii)

**(iv) Conclusion.** Availability gives ≥ 1 + d²; independence keeps them
distinct; exhaustiveness gives ≤ 1 + d². ∎

## 3. Refutation of the Rival Counts

The two counts a referee would propose fall to the same principles, and —
crucially — fall *independently of the audited trace*, at d = 1, where the three
candidates disagree (1+1² = 2, vs. 1+2d = 3, vs. 1+C(1,2) = 1):

- **2d (face counting, or op-kind × direction):** predicts a third independent
  schema for the circle beyond {β, monodromy}. Enumerate by hand: the candidate
  third schemas (inversion, reversed transport, hcom with degenerate tube) are
  all definitional composites. The circle has exactly two. Moreover the
  (coe, hcom) op-pair is not an independent axis per direction: for a
  constant-boundary generator the degenerate-tube hcom and the coe along the
  same direction are definitionally related; the correct parameterization is
  (principal direction, probe direction) — the matrix, not the face list.
- **C(d,2) (unordered planes):** predicts ν_H = 1 for the circle — denying the
  monodromy schema, which manifestly exists (it *is* the circle's computational
  content: the transport that winds). And for d ≥ 2 it requires identifying
  K_{a,b} with K_{b,a}, i.e. treating the transposition as definitional, which
  the transpose obstruction blocks.

The d = 1 anchor is decided by inspection of the circle, with no reference to
the Genesis trace; the general-d matrix form then follows from the
direction/probe parameterization, the transpose obstruction, and staging.

## 4. The Born-Rule Analogy (assessed: structural, not superficial)

The suspicion that L1 rhymes with Appendix H is correct, and the rhyme has a
precise common source. Appendix H's theorem: the least phase-neutral scalar
honestly extracted from a directed proof object is its **conjugate
self-pairing** — pairing routes against reversed routes; diagonal terms are the
|ψᵢ|², off-diagonal pairs are the interference cross terms; higher even
exponents are killed by constructive primeness. Lemma L1's theorem: the
complete depth-2 computational surface of a directed generator is its **ordered
direction self-pairing** — diagonal pairs are the monodromies, off-diagonal
pairs are the variation fillers; higher tubes are killed by union-staging.

Same theorem-schema, different categories:

| | App H (Born) | L1 (Kan payload) | Fibonacci debt (App D) |
| --- | --- | --- | --- |
| Directed object | proof route ψ | d-cell p | interface layer |
| Pairing | route × reversed route | direction × probe direction | layer × predecessor layer |
| Diagonal | \|ψᵢ\|² weights | monodromies | self-burden |
| Off-diagonal | interference terms | variation fillers | cross-layer obligation |
| Higher orders freed by | constructive primeness | cofibration staging | window truncation |
| Quadratic signature | exponent 2 | d² | two-term recurrence |

All three are the depth-2 window casting the same shadow: *at coherence depth
two, the invariant content of a directed thing is the matrix of its ordered
self-pairings, and everything beyond pairs is composite.* Neither result
derives the others — the uniqueness arguments live in different categories
(monoidal scalar extraction vs. schema enumeration vs. recurrence algebra) —
but the triple occurrence of the quadratic signature from one window is strong
evidence that D2 (pairs-at-most counting) is the correct sharpening of the
philosophical definitions rather than a convenience. It also upgrades the
framework's rhetoric honestly: "the answer is 2" now has three independent
technical incarnations traceable to one source.

## 5. Flagged for Mechanization

1. **The transpose obstruction** (§2.ii): verify in the target cubical theory
   that generator transposition is propositional, not definitional. If a theory
   with definitional cube symmetries plus symmetric HIT declarations were used
   instead, ordered pairs would collapse to unordered and the count would drop
   toward 1 + d + C(d,2); the lemma is theory-relative in exactly this one
   place, and the engine's MBTT grammar (no symmetry equations on declared
   constructors) sits on the ordered side. State this as a hypothesis of the
   lemma.
2. **Union-staging** (§2.iii.1): confirm the hcom decomposition over ∨ in the
   exact cofibration lattice used. Standard in CCHM-style systems; must be
   checked, not assumed.

## 6. New Finding: the Multi-Constructor Corner (off-trace)

The engine computes `ν_H = path_count + max_dim²` per *telescope*; the
per-constructor form proven here gives Σᵢ (1 + dᵢ²) = path_count + Σᵢ dᵢ².
These agree on every selected Genesis step (each has at most one path
constructor) and diverge for multi-path-constructor telescopes — i.e. only on
*rejected or unexplored* candidates. This is a newly identified OPEN corner:
if L1 is accepted, the engine should move to the Σ dᵢ² form and the strict lane
should be rerun (expected trace-neutral, since the divergence is off-trace —
but "expected" is not "verified," and a rejected candidate whose ν_H rises
under the correction could in principle re-enter a band). Added to the
verification program as **V2b**.

## 7. Consequences for the Ledger

- `EVALUATOR_DERIVATION.md` P4 upgrades from DERIVABLE to
  **PROVEN (sketch), mechanization pending**, with the two flagged steps and
  the theory-relativity hypothesis of §5.1.
- The rival-rule reruns in V2 remain worth running, now as *consistency
  checks* of the proof rather than as open model selection: 2d and C(d,2)
  are refuted at d = 1 by inspection, so if either preserved the trace better
  than d², that would indict the trace, not rescue the rivals.
- Remaining open items from the original audit: **L2** (the r² transport
  matrix — the monodromy/variation decomposition here suggests the analogous
  route: r self-transports + r(r−1) directed cross-transports = r², via the
  same ordered-pairing principle; the transpose-obstruction analogue is that
  a weave's (i,j) and (j,i) transports are related only propositionally),
  and the **(post+1)/2** H-space chaining term (unchanged; still the sharpest
  exposure).

## 8. Machine-Checked Finite Layer (2026-07-18)

The Rust checker enumerates one beta tag and every ordered
`(principal, probe)` tag for a single positive dimension, then checks the
beta/diagonal/off-diagonal partition and total `1 + d^2`. The Agda theorem
`l1-conditional-cardinality` proves `Presented Schema (1 + d * d)` only from
an `L1Semantic Schema d` classifier/realizer isomorphism. Constructing that
isomorphism for computation schemas modulo definitional interderivability is
exactly the remaining cubical-semantic burden in Section 5.

The multi-constructor evaluator discrepancy in Section 6 was subsequently
closed by V2b: the evaluator now sums the per-constructor squares and the
strict fifteen-step trace was unchanged. That engine correction is not a proof
that cross-constructor semantic surfaces form a disjoint sum, so the formal L1
checker intentionally remains the single-constructor statement above.
