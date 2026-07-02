# The H-Space Enumeration: Settling the `(post+1)/2` Term

**Status:** hand enumeration complete; code updated to match (2026-07-02).
**Verdict:** the term **survives the audit** — it is the canonical-pairing form of
a principled count, not a tuning knob. Its reading is now explicit in code and
here. This closes the last OPEN item of `EVALUATOR_DERIVATION.md` §4.
**Also in this change:** the Lemma L1 correction (`Σ dᵢ²` replacing `max(d)²`)
is applied throughout `pen-eval/src/nu.rs`, including the Hit upper-bound
estimator, whose old max-based form would have been an *unsound* bound under
the corrected rule (it could under-estimate and wrongly prune
multi-constructor candidates).

---

## 1. What Was at Stake

The Hit-class formula charges, for a HIT telescope with formation,
`ν_C = post + (post+1)/2` over the `post` operative clauses following the path
constructor. The chaining term `⌊(post+1)/2⌋ = ⌈post/2⌉` had no derivation and
exactly one load-bearing exposure in the entire strict trace: Step 8, where the
H-space telescope has `post = 2` (multiplication μ, unit coherence), giving
ν_C = 3 inside ν = 18 against a margin of 0.17 — the trace's thinnest
load-bearing use of an unforced choice.

## 2. The Enumeration

Step-8 telescope: formation `S³ : U`, unit point `e : S³`, `PathCon(3)` (the
3-cell), then the operative pair: μ (binary multiplication) and the unit
coherence (μ(e,x) = x, charged as one clause per the appendix's "coherent
unit/multiplicative behavior").

Enumerate the eliminative/cross-interface schemas (ν_C ledger) the operative
pair opens, under P2 and the L2 face principle:

1. **apply-μ** — the multiplication's forward face: combine two S³-data.
   One natural family (partial applications and translations L_x = μ(x,−),
   R_x = μ(−,x) are λ-instances — definitionally complete, free).
   No lifting face: no fibration structure is charged here (the κ=1-carve-out
   logic of Lemma L2, clause-level).
2. **unit-rewrite** — the coherence clause's face: the rewriting family
   μ(e, x) ⇝ x, letting derivations cancel units in compound expressions.
   An equation clause has exactly one face (its rewrite); it introduces
   nothing and lifts nothing.
3. **translate-the-cell** — the interaction of the operation with the path
   payload: μ acts on the 3-cell, making the generator *translatable* around
   the sphere. This is a genuinely new family (before μ, the cell sat at the
   base point; no B-schema moved it), and it is not a λ-instance of apply-μ
   (it is μ's action on *path* data, computed through the cell's Kan
   structure, not on point data). The coherence clause contributes no
   independent cell action — its action on the cell is the restriction of
   μ's (definitionally complete).

Total: **3** = post (two faces) + 1 (one cell action). The engine's
`2 + ⌈2/2⌉ = 3` ✓.

A satisfying cross-check: schema 3 is *exactly* the capability Step 9
consumes. The Hopf construction needs the cell to be movable by the
multiplication — the fibration is built by translating the generator around
the sphere. The ledger opened at Step 8 precisely the door Step 9 walks
through. (Selection and counting cohering again, as in L2's diagonal.)

## 3. The General Form, and What `⌈post/2⌉` Really Is

The principled count is:

    ν_C(Hit, formation) = post + ops

where `ops` = the number of *operation* clauses among the operative entries
(each operation contributes one cell-action schema; each coherence equation
contributes only its rewrite face, already inside `post`).

The engine cannot syntactically distinguish operation from coherence in the
MBTT skeleton (at Step 8 both operative clauses are Lam-headed). The formula
`ops = ⌈post/2⌉` is therefore the **canonical-pairing convention**: charged
operative telescopes are canonicalized as (operation, coherence) pairs, with
an odd remainder read as an unaccompanied operation. Under that convention
the engine's term equals the principled count. The convention is defensible:
a coherence equation among the operative entries necessarily constrains an
operation charged alongside it (an equation constraining only sealed
structure would not belong to this candidate's operative telescope), and one
charged coherence per operation is the minimal canonical packing.

**Where it can break, honestly:** deviant packings. One operation charged with
*two* coherence equations (post = 3, ops = 1) would be counted as ops = 2 by
the convention (principled: 4; engine: 5). Such packings are non-canonical
presentations and should be normalized away by canonical dedup — but that
enforcement is currently implicit. Disposition: the V1 direct enumerator must
distinguish operation from equation clauses semantically and cross-check the
convention across the retained-evidence corpus; if deviant packings occur in
live bands, either canonical dedup must normalize them or the engine needs a
syntactic ops-discriminator. Logged as **V1's second named target** (after
the Möbius anchor).

## 4. Code Changes Applied (this commit)

In `crates/pen-eval/src/nu.rs`:

1. `(post+1)/2` replaced at all three sites (profile method, fast path,
   `hit_nu_c`) by the documented `canonical_operation_count(post)` — **behavior
   identical**, semantics now explicit.
2. Lemma L1 correction: new profile field `path_dim_sq_sum`; ν_H is now
   `path_count + Σ dᵢ²` in all four computation paths (profile method, fast
   path, `compute_nu_h`, and the incremental merge). **Identical on every
   single-path-constructor telescope — i.e., on all fifteen selected steps —
   divergent only for multi-constructor candidates (off-trace).**
3. The Hit upper-bound estimator (`hit_total` and both callers) now bounds the
   square-sum (`path_dim_sq_sum` + at most one addable `max_dim²`) instead of
   using `max(d)²`, restoring soundness of the pruning bound under the
   corrected rule, and routes its chaining term through
   `canonical_operation_count`.

## 5. Required Verification (not runnable in this environment)

No Rust toolchain is available where these edits were made. Before merging:

1. `cargo test -p pen-eval` — expected: all green. Reference-sequence tests
   are unaffected in value (single constructor per selected step); the
   fast-path/legacy consistency tests were updated on both sides.
2. `cargo test` (workspace) — engine/enumeration tests exercise non-reference
   candidates; multi-constructor candidates in live bands may shift ν_H
   upward (Σ ≥ max), which can alter retained-evidence orderings.
3. **V2b: full strict-lane rerun.** The Σ dᵢ² correction and the corrected
   upper bound can, in principle, change which off-trace candidates survive
   bands. Expected outcome: the fifteen accepted shells and all totals
   unchanged (the correction is off-trace at every selected step). If the
   trace *does* shift, that is the derivation speaking, and the new trace is
   the honest one — do not revert the rule to preserve the old fixtures.

## 6. Ledger Impact

With this enumeration, every component of the evaluator is now
**forced, proven (sketch), derived-with-reading, or gauge**:

| Item | Status after 2026-07-02 |
| --- | --- |
| P1, P2, P3, P6, Modal, Axiomatic, Foundation splits | FORCED / proven |
| P4 (d²) | PROVEN (sketch) — L1; mechanization pending |
| P5 record (k−1) | PROVEN (theorem, paper) |
| P5 morphism (2κ + r²) | PROVEN (sketch) — L2; mechanization pending |
| Hit chaining `⌈post/2⌉` | DERIVED with canonical-pairing reading (this doc) |
| Global unit of counting | GAUGE (Observation B) |

The evaluator's degrees-of-freedom problem, as posed, is closed at the
sketch level. What remains is engineering: mechanize L1/L2's flagged steps,
build the V1 direct enumerator, and run V2b.
