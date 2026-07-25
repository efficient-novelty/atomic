# XF-0b2 Result — Targeted re-examination of family df4feb52882e

**Campaign:** XF-1. **Plan:** `docs/xf0b2_df4feb52_reexamination_plan.md`.
**Date:** 2026-07-24. **Artifact:** `docs/xf0b2_df4feb52_v1.json`, sealed and
mutation-guarded: `cargo run -p pen-search --example xf0b2_df4feb52_v1 --
replay docs` recomputes the digest, **re-derives every Phase R-0 claim from
the frozen kernel**, enforces the R-1 recording discipline over all five
registered leads, enforces the R-2 stability quorum, and binds the parent
XF-0b concordance by digest. Flipping any verdict, attempt, citation or
quorum row invalidates replay; so does a kernel change that would alter the
elaboration.

## Headline

**Disposition: NO-NAME CONFIRMED. Export WITHDRAWN.**

These are two findings, not one, and they rest on different evidence.

1. **The naming question closes as NO-NAME.** Eight independent contexts —
   five armed lead analysts, two independent armed analysts, one adversarial
   verifier — concur, with no flips. All five registered leads are disposed.
   Roughly thirty-five distinct classical targets were attempted across
   type theory, categorical logic, functional analysis, illative combinatory
   logic and Automath. The concordance's Outcome-U margin therefore holds:
   the register speaks classical mathematics at aspect granularity, with
   **exactly one** family that mathematics does not name.

2. **The export does not follow, and is withdrawn.** A finding made during
   R-1 and confirmed against `git log`: the sealed stage-14 telescope is a
   **hand-authored literal that predates every piece of stage-14 search
   machinery in this repository**, and the certified register is built by
   *reading* it rather than by running a search. A structure that is a
   literal in the code cannot be reported as that code's discovery. Full
   record: `docs/xf0b2_provenance_finding_v2.md`.

Per **F-R5**, a withdrawn export is a success of the discipline, and the
brief's instruction that the re-examination "must be able to go either way"
is satisfied in a direction the brief did not anticipate: not by naming the
object, but by disqualifying the claim that it was found.

## Phase R-0 — what the family actually is (F-R1 discharged)

Re-derived mechanically from the frozen kernel, never from the concordance
gloss. Note: `docs/xf0b2_r0_dossier_v1.md`; post-freeze additions and
corrections: `docs/xf0b2_r0_addendum_v1.md`; raw output:
`docs/xf0b2_r0_probe_output.txt`.

**Elaborated content.** The kernel elaborates stage 14 at ambient arity **1**.
Clause 3's derivation is:

```
pi-form :: Type   [COARSE]
  lam-intro :: Fun(Neutral, Type)   [COARSE]
    ambient-param-1 :: Type
  sigma-form :: Type
    ambient-param-1 :: Type
    field-ref-0 :: Type
```

- **Family form:** `Π( λ_. X , X × Y )`, parameters `[Type, Type]`.
- **In-situ form:** `Π( λ_. X , X × T₀ )`, where `T₀` is the type declared by
  clause 0, `Σ(Π(X,Π(X,U)), X)` — the stage's inner-product-plus-vector datum.

**Four corrections to the concordance gloss** ("a dependent product whose
domain position is a term-level abstraction"):

1. It is **not a dependent product**. Both Π and Σ are non-dependent, stably
   across every admissible ambient arity.
2. The abstraction is **not generic**: it is specifically the *constant family*
   `λ_.X`, whose body ignores its own binder.
3. `Var 2` is **not an unexplained second sort**. At the kernel's own minimal
   ambient arity it resolves to the **field of clause 0**. (A resolution of a
   reference, not a fusion of clauses — the credited family still abstracts it
   to a free `Type` parameter. F-R4 intact.)
4. **A fact the concordance did not record:** the formation is accepted only
   under **two coarse subsumptions** — the highest count at stage 14, against
   1 for clause 8 and 0 for the other seven. The kernel **does not certify
   this Π-formation as well-sorted**; it records the sort mismatch and
   proceeds. The Π-binder's classifier is `El(Lam(Var 1))`: an element of a
   function.

**Uniqueness.** Stage 14 clause 3 is the only clause in all fifteen sealed
telescopes with a `Lam` in a Π/Σ domain position.

### R-0.2 Irreducibility certificate — IRREDUCIBLE

Zero `App` nodes, so no β-redex; NF reached in 0 steps and identical to the
clause. No η rule exists in the kernel, and the sealed signature exports **0**
computation-role clauses across all fifteen entries. No η-redex shape. Bounded
catalog corroboration at scope 4, ≤5 nodes: of **99 064** expressions exactly
**15** are judgmentally equal to `Lam(Var 1)`, and the 7 that are not
syntactically `Lam`-headed are β-redexes reducing to it. **No type-former
presentation of the domain exists.**

The plan's contingency — "if reducible after all, the family's distinctness
collapses into a presentation question" — does not fire.

### R-0.3 Role dossier

**Exactly one occurrence in the entire fifteen-stage register**: stage 14,
clause 3, `axiomatic_introduction_head` / `intrinsic_kernel` / `kernel_head`,
resolution class `proved_family`. No other stage references it. `instances` is
a single `Generator` record — **no specializations anywhere**. Marginality:
`MarginalNoClosurePreimage` against the 37-family predecessor closure of steps
1..13. Family id and predecessor signature digest reproduced byte-identically
from the kernel against the sealed provenance package.

Stage 14 extracts 9 families, 6 marginal, of which 3 are credited (ν₁₄ = 3)
and 3 refused credit. **df4feb52882e is one third of stage 14's entire
semantic payment**, alongside clause 0 (pointed quiver) and clause 2
(coalgebra for the squaring endofunctor).

## Phase R-1 — armed naming attempts (F-R2: arming widened the search, the bar did not move)

| Lead | Subject | Targets | Verdict | Confidence |
|---|---|---|---|---|
| L1 | Very dependent function types (Hickey/NuPRL); Kopylov dependent intersections | 4 | no_name | high |
| L2 | Self-referential record-type presentations | 7 | no_name | high |
| L3 | Comprehension-category / CwF: Π at a section rather than a display map | 5 | no_name | high |
| L4 | Polynomial functors; containers; exponent given by a term | 5 | no_name | high |
| L5 | Open, driven by the role dossier | 7 | no_name | medium |
| A | Independent armed analyst (outward from the type theory) | 10 | no_name | high |
| B | Independent armed analyst (inward from the mathematics) | 13 | no_name | high |
| V | Adversarial verifier | 7 attacks | no_name | high |

**Why everything failed, in one sentence.** Every target's *first* defining
datum is an object — a base, a shape type, an index category, a measure space,
a Hilbert space, a code, a finite cardinal, or the codomain of an arrow — and
the source supplies a family over an **untyped binder**. Analyst B's reading
of that pattern is registered as the study's sharpest interpretive finding:

> Thirteen targets from seven disjoint areas of mathematics failing at the
> identical point is the signature of a defect, not of a structure; a
> genuinely novel object would fail different targets for different reasons.

**Near misses, each killed on a named law.**

- **Coalgebra for `(-) × Y`** (Rutten 2000 §2) — the cheapest tidy naming,
  which would have closed the concordance at 32/32. A coalgebra's structure
  map must have the carrier as its *source*; the recorded source is
  `El(Lam(Var 1))`, not `El(Var 1)`.
- **Illative `Ξ (K X) (K (X × Y))`** (Czajka, JSL 78(3), Def. 3.1) — every
  syntactic node maps. Killed because illative CL is *type-free* and so has
  nothing to receive the source's sort content, and because `Ξ A B` is a
  formula, not a structure with defining data.
- **Simple product / fibrewise exponential** (Jacobs Def. 1.9.1) — requires
  identifying `λ_.X` with `X`, which R-0.2 certifies impossible.
- **`Pi_g` in an LCCC** (Seely 1984) — the only target natively admitting a
  *morphism* where the record puts a term. Killed because `Pi_g`'s argument
  must live in the slice `C/X` and the certified non-dependence means no
  display map exists.

**The Edinburgh LF control.** LF is the mainstream framework built around
families-as-terms, and it *forbids* this shape: its Π-formation demands a type
in the domain, and a family must be *applied* first. Clause 3 has zero `App`
nodes. This is the poverty-of-language rebuttal at the level of the literature
rather than of this grammar.

## Phase R-2 — disposition and quorum

**NO-NAME CONFIRMED.** The plan's stability requirement is met: two
independent armed analysts (A, B) and an adversarial verifier (V) concur, with
all five registered leads disposed. No verdict flipped at any point — in
contrast to the concordance run, where two flipped at this boundary. The
one-family-thin margin that motivated this brief is now eight-context thick.

### What the adversarial verifier changed

The verifier concurred but corrected the record in three substantive ways, all
adopted:

1. **The "η hinge" was a coordinator error.** The claim that an η rule would
   collapse `Lam(Var 1)` to `Var 1` is **false** — η's redex is `λx.(f x)`, and
   this body is `Var 1`, not an application. A campaign ruling that η-freeness
   is a presentation convention would change **nothing**. The verdict is not
   one convention deep; it is as deep as the absence of any rule identifying a
   constant function with its value. See addendum §E.
2. **Analyst B's flip-condition was met and did not flip.** The citation B said
   would make it withdraw — a published rule forming Π with a family-as-term in
   the domain — *exists*: Automath's AUT-QE type inclusion at n=1, k=0, which
   is node-for-node the kernel's own coarse subsumption #2. It does not flip
   the verdict because a *rule* is not a *structure*, and the object it
   licenses is `(A → X) → (X × Y)` with A anonymous. See addendum §G.
3. **One frozen constraint proves too much.** Dossier §5 constraint 1 states
   equation-freeness as a standing bar on all lawful translations; in that form
   it would forbid naming any family whose target carries laws, forever,
   including the 31 XF-0b named. Amended to a per-target observation. No
   verdict rests on it. See addendum §F.

## The provenance finding, and why the export is withdrawn

Full record and chronology: `docs/xf0b2_provenance_finding_v2.md`.

The subject clause's exact shape is a hard-coded literal in the search's
admissibility layer, and the per-position gate admits **exactly one
expression** at clause position 3. More: `late_clause_options` bypasses raw
enumeration entirely at κ=9 and emits the nine-clause telescope verbatim from
a literal table.

The causal direction is **settled**, and both readings the first draft left
open are wrong:

- `git log -L 335,363:crates/pen-core/src/telescope.rs` returns **exactly one
  commit** — `5dd8447`, 2026-03-13, "First rust implementation step". The
  stage-14 telescope has never been modified since.
- The search gates postdate it: `matches_hilbert_functional_shell` 2026-03-14,
  `supports_hilbert_functional_clause_at_position` 2026-03-15.
- The certified register is built by *reading* the literals
  (`t_bi_nu1_regression_v6.rs`: `(1..=15).map(|s| (s, Telescope::reference(s)))`).
- Step 14 "Hilbert, κ=9" was **pre-declared** in
  `skills/pen-atomic/references/02-target-sequence.md`, in commit `4adcf28`,
  the repository's **Initial commit**.

So `Telescope::reference` is a hand-transcribed **design target**, and the
search's stage-14 arms were written afterwards to reproduce it. **No search in
this repository ever found this object.** Whether a prior Haskell engine did is
outside this repository's history and is not claimed either way.

**Effect on the naming verdict: it survives, but the grounds are disturbed.**
Every context leaned somewhere on "the record could have written Y and
declined to" — an inference that presupposes something was choosing. That
inference is now weak. It was auxiliary everywhere and load-bearing nowhere:
every surviving obstruction says a *defining datum has no counterpart in the
typed content*, and a datum the source does not carry cannot be mapped whatever
the reason it is absent.

**`docs/df4feb52_specification_v1.md` is deliberately not written.** The plan
makes it conditional on NO-NAME CONFIRMED, but its purpose is to specify a
*candidate novel object*; writing it would assert exactly the provenance this
finding defeats.

## Consequences registered

1. **The concordance's Outcome U stands, with its meaning changed.** 31 of 32
   families carry lawful classical names; the 32nd does not. But the honest
   description of the 32nd is now *"no classical name, and the best available
   explanation of its shape is an authoring artifact in a hand-transcribed
   reference table"* — not *"a candidate novel mathematical object"*.
2. **The v2 census unit question is unblocked but re-pointed.** XF-0b made a
   principled v2 unit — named classical structures at family granularity —
   conditional on resolving df4feb52882e. It is resolved. But any v2 census must
   now disclose, in addition to XF-0b's contamination and v1's spent rule, that
   **the register's content is read from a hand-authored reference table**.
3. **A campaign-level methods question is opened, wider than this study.** Not
   "is the late stratum a transcription" but: *the whole certified fifteen-stage
   register is read from a hand-authored literal table, and every
   structural-family gate from stage 3 onward is a per-position literal.* This
   bears on XF-0b's headline and on the register's standing as a search result.
   Referred to the internal program.
4. **XF-0 is untouched.** It was blind and never read the register; its 3/11
   mismatch stands exactly as published.
5. **The stage-15 fix orphan** (XF-0b's Outcome-X row) is unaffected and remains
   a named question to the internal program.

## Disclosures

- **Information flow (inherited F-C4).** This study reads the certified
  register directly and is not blind. It contaminates future blind work.
- **Adjudication stability (F-R3).** Eight contexts, zero flips, all five leads
  disposed. Recorded in the sealed artifact with per-lead attempts and
  obstructions.
- **Level vs index (open, disclosed, not load-bearing).** `Telescope::is_connected`
  reads `Var` as a *relative index*; the kernel elaborator reads it as an
  *absolute level*. The credited family is unaffected (parameters `[Type, Type]`
  either way) and no verdict depends on it. But the verifier raised the
  probability that the hand-authored literal was written under `telescope.rs`'s
  own convention — in which case the **in-situ** narrative (`Var 2` = clause 0's
  field) would need re-derivation, though the family and the verdict would not.
  **This must be settled before any document describes the family "in situ".**
- **Negative existential (F-R2 honesty).** Eight contexts sweeping overlapping
  literatures are not eight independent samples. The individual obstructions are
  high-confidence and mechanical; the claim that *nothing* names this family is
  medium-confidence, with residual risk concentrated in the illative /
  Automath corner — which is precisely where the verifier found B's
  flip-citation and showed it still does not flip.
- **Citation provenance.** Four analysts declared sources they could not obtain
  in full (Hickey FOOL 3 numbered rules; Jacobs 1999 definition numbers; Pollack
  FACS 2002; Niu–Spivak numbered propositions) and declined to invent numbers.
  No obstruction depends on any of them. Recorded verbatim in the sealed
  artifact.
