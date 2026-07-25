# XF-0b2 Phase R-0 — Internal characterization of family df4feb52882e

**Campaign:** XF-1. **Plan:** `docs/xf0b2_df4feb52_reexamination_plan.md`.
**Date:** 2026-07-24. **Status:** versioned note; this is the R-0.1
*gloss correction* the plan mandates, plus the R-0.2 irreducibility
certificate and the R-0.3 role dossier.

Everything below is re-derived mechanically from the frozen kernel by
`cargo run -p pen-search --example xf0b2_r0_probe` (raw output:
`docs/xf0b2_r0_probe_output.txt`). **No statement here rests on the
concordance's informal gloss** (falsifier F-R1). Where the elaboration
and the gloss differ, the elaboration governs and the difference is
recorded in §1.4.

---

## 0. Frozen conventions actually in force

Pulled from the kernel sources, not from memory:

- **Binding** (`crates/pen-type/src/normalize.rs`, tag
  `kernel-v1-debruijn-levels`): `Var` carries a **1-based de Bruijn
  LEVEL** into the unified scope `[ambient parameters, prior telescope
  fields, enclosing binders]`. **Binders are `Lam` bodies and `Pi`/`Sigma`
  codomains only**; `Pi`/`Sigma` *domains* are not under the binder, and
  unary operators (modal, temporal, `Refl`, `Susp`, `Trunc`) bind nothing.
- **Reduction** (same file): generic beta `App(Lam b, a) → b[binder := a]`
  is the **only** oriented computation clause. There is **no eta rule**.
- **Judgmental equality** (`crates/pen-type/src/equality.rs`): fuel-bounded
  beta-normal-form equality with a serializable witness.
- **Ambient discipline** (`crates/pen-type/src/elaborate.rs`): each
  telescope is elaborated in the **smallest ambient context that scopes
  it**, capped at 2.
- **Field classifiers**: a field reference to a clause whose *kernel*
  role is `Formation` has kernel type `Type` (the declared type as an
  opaque object); every other field reference is `Neutral`.
- **Coarse subsumption**: accepting a non-`Type` classifier where `Type`
  is demanded is *not* an error — it is counted and recorded in the
  derivation (`as_type`, `elaborate.rs`).

## 1. R-0.1 — The elaboration in situ

### 1.1 The sealed subject

Stage 14 ("Hilbert-functional shell"), candidate hash
`blake3:1ff4820f2272c022a5fece1032aa9647e41167a3dd8f62765e84e22f5d90b19c`,
from `Telescope::reference(14)` (`crates/pen-core/src/telescope.rs`).
The nine clauses, 0-based:

| # | Clause expression | Kernel role | Coarse |
|---|---|---|---|
| 0 | `Sigma(Pi(Var 1, Pi(Var 1, Univ)), Var 1)` | Formation | 0 |
| 1 | `Pi(Var 1, Var 1)` | Formation | 0 |
| 2 | `Pi(Var 1, Sigma(Var 1, Var 1))` | Formation | 0 |
| **3** | **`Pi(Lam(Var 1), Sigma(Var 1, Var 2))`** | **Formation** | **2** |
| 4 | `Sigma(Pi(Var 1, Var 1), Pi(Var 1, Var 1))` | Formation | 0 |
| 5 | `Pi(Lib 13, Var 1)` | Formation | 0 |
| 6 | `Pi(Lib 12, Var 1)` | Formation | 0 |
| 7 | `Pi(Lib 11, Var 1)` | Formation | 0 |
| 8 | `Lam(Pi(Var 1, Univ))` | Introduction | 1 |

**The kernel's minimal ambient arity for stage 14 is exactly 1** (computed,
not assumed). Call that single ambient type parameter **X** — the carrier.

### 1.2 The derivation of clause 3, verbatim

```
pi-form :: Type   [COARSE]
  lam-intro :: Fun(Neutral, Type)   [COARSE]
    ambient-param-1 :: Type
  sigma-form :: Type
    ambient-param-1 :: Type
    field-ref-0 :: Type
```

Scope arithmetic at clause 3: base scope = ambient (1) + prior fields
(3) = **4**. The `Lam` binder occupies level **5**; the `Pi` binder
occupies level **5**; the `Sigma` binder occupies level **6**.

Every `Var` resolved:

| Occurrence | Level | Resolves to | Classifier |
|---|---|---|---|
| body of `Lam` | 1 | **ambient parameter #1** (= X) | `Type` |
| domain of `Sigma` | 1 | **ambient parameter #1** (= X) | `Type` |
| codomain of `Sigma` | 2 | **field of clause 0** | `Type` |

**Both `Var`s sit strictly below every binder level.** This is stable
across every admissible ambient arity A ∈ {0,1,2} (all three checked
mechanically), so the conclusion does not depend on the ambient reading:

- A=0 → binders at 4,5; `Var 1` = field of clause 0; `Var 2` = field of clause 1
- A=1 → binders at 5,6; `Var 1` = ambient #1; `Var 2` = field of clause 0 ← **the kernel's own reading**
- A=2 → binders at 6,7; `Var 1` = ambient #1; `Var 2` = ambient #2

### 1.3 What the clause therefore says

**In situ, at the kernel's ambient arity 1:**

> `clause 3  :  Π( λ_. X , Σ( X , T₀ ) )`   where `T₀ = Σ(Π(X, Π(X, U)), X)` is the type declared by clause 0.

- The `Π` is **non-dependent**: its bound variable occurs nowhere in the
  codomain.
- The `Σ` is **non-dependent**: its bound variable occurs nowhere in its
  own codomain. So `Σ(X, T₀)` is the plain product `X × T₀`.
- The domain `Lam(Var 1)` is a `Lam` whose body **does not mention its own
  binder**. It is therefore the **constant family `λ_. X`** — not an
  arbitrary term-level abstraction.
- The `Π`-bound variable receives the kernel classifier
  **`El(Lam(Var 1))`** — "an element of `λ_. X`", i.e. the kernel is
  asked to quantify over the inhabitants of a *function*, not a type.

**As the credited family** (canonical presentation = clause normal form
abstracted over first-use free levels):

> `c : Π( λ_. X , Σ( X , Y ) )`  with parameters `[Type, Type]`, X ↦ level 1, Y ↦ level 2.

Canonical normal form: `Pi(Lam(Var 1), Sigma(Var 1, Var 2))`, identical
to the raw clause. Renaming map `free_scope_len: 4, forward: [(1,1),(2,2)]`.
Naturality square (transposition (1 2)) closes judgmentally in 0 steps.

### 1.4 Where this differs from the concordance gloss (F-R1 discharge)

The XF-0b concordance glossed the family as *"a dependent product whose
domain position is a term-level abstraction"*. Four corrections:

1. **It is not a dependent product.** It is doubly non-dependent. (The
   XF-0b dossier notes reached the same conclusion by a hedged level
   argument; the kernel settles it directly, and the sealed *result
   document's* one-line gloss — "a dependent product whose domain
   position is a term-level abstraction" — is wrong as stated. Corrected
   here.)
2. **The abstraction is not generic.** It is specifically the *constant
   family at the carrier*, `λ_. X`. Any naming attempt that treats the
   domain as an arbitrary function/term is attacking a shape the record
   does not contain.
3. **`Var 2` is not an unexplained second output sort.** The concordance
   recorded it as "an ambient sort or a prior clause field" and called
   the resulting factor "an unexplained second output sort Y (the unique
   use of context level 2 anywhere in the sealed stage-14 candidate)". At
   the kernel's own minimal ambient arity, it resolves to **the field of
   clause 0** — the stage's inner-product-plus-vector datum
   `Σ(Π(X,Π(X,U)), X)`. This is a *resolution of a reference*, not a
   fusion of two clauses: the credited family still abstracts it to a
   free `Type` parameter, and no clause has been merged (F-R4 / F-C3
   intact). Both readings are on the table for R-1; neither may be
   swapped for the other mid-argument.
4. **A decisive fact the concordance did not record: the formation is
   accepted only under two coarse subsumptions.** Clause 3 carries
   `coarse = 2`, the highest at stage 14 (clause 8 has 1; the other seven
   have 0). The two uses are (i) the unannotated `Lam` binder and
   (ii) `as_type` coercing the domain's classifier `Fun(Neutral, Type)`
   into the `Type` position demanded by `pi-form`. **The kernel does not
   certify this Π-formation as well-sorted.** It records the sort
   mismatch and proceeds. That is the exact, mechanical content of
   "abstraction in domain position".

### 1.5 Uniqueness in the corpus

Scanning all nine clauses of all fifteen sealed telescopes: **stage 14
clause 3 is the only clause in the entire sealed signature with a `Lam`
in a `Pi`/`Sigma` domain position.** There is no second instance to
compare it against.

Related but distinct: stage 14 clause 8 `Lam(Pi(Var 1, Univ))` is also a
constant family (at `X → U`), but it stands as a clause in its own right,
not in a domain position, and its family is **internal** (it has a
predecessor preimage), whereas clause 3's family is marginal.

## 2. R-0.2 — Irreducibility certificate

**Verdict: IRREDUCIBLE. The domain abstraction is not β/η-eliminable
under the frozen kernel rules.**

Grounds, each mechanically checked:

1. **No β-redex exists.** Clause 3 contains **zero `App` nodes**. Its
   normal form is computed in **0 beta steps** and is syntactically
   identical to the raw clause. Likewise `Lam(Var 1)` in isolation: 0
   steps, NF identical.
2. **No η rule exists to apply.** Beta is the only oriented computation
   clause in the kernel. Independently confirmed on the sealed data:
   across all fifteen sealed entries the count of exported
   `Computation`-role clauses is **0**, and the only β-redex clause in
   the entire sealed corpus is step 4 clause 2. There is no
   library-supplied conversion that could act here.
3. **No η-redex shape either.** An η-redex at scope 4 would be
   `Lam(App(f, Var 5))` with the binder not free in `f`. The body is
   `Var 1`. Not an η-redex.
4. **Nothing outside the `Lam`-headed class can be judgmentally equal to
   it.** Judgmental equality is β-NF equality; `Lam(Var 1)` is its own
   NF; so any expression equal to it has NF `Lam(Var 1)`, which is
   `Lam`-headed. Bounded corroboration over the generator catalog at
   scope 4, ≤ 5 nodes: of **99 064** expressions, exactly **15** are
   judgmentally equal to `Lam(Var 1)`; the **7** that are not
   syntactically `Lam`-headed are all β-redexes `App(Lam(Lam(Var 1)), a)`
   that reduce to it. **No type-former presentation of the domain
   exists.** Spot checks: `Lam(Var 1)` is judgmentally distinct from
   `Var 1`, `Univ`, `Lib 13`, `Pi(Var 1, Var 1)`, `Sigma(Var 1, Var 1)`.

**Consequence for the study.** The family's survival of the quotient is
*not* a presentation artifact. The plan's contingency — "if reducible
after all, the family's distinctness collapses into a presentation
question and the quotient machinery owns the next move" — does not fire.
Naming remains the live question, and R-1 proceeds.

## 3. R-0.3 — Role dossier

### 3.1 Where the family acts

**Exactly one occurrence in the entire fifteen-stage register.** From the
sealed semantic ledger v6 (`docs/t_bi_nu1_semantic_provenance_v6.json`),
scanning all 15 packages' role rows:

| Field | Value |
|---|---|
| stage | 14 |
| owner clause | 3 |
| occurrence kind | `axiomatic_introduction_head` |
| mechanism | `intrinsic_kernel` |
| local role | `kernel_head` |
| resolution class | `proved_family` |
| direct rule | `direct_kernel_family` |
| coordinates | `{}` (none) |
| uniform specialization | false |

No other stage — including stage 15, the temporal shell that sits on top
of it — references this family. Its `instances` list contains a single
record, `{clause_index: 3, kind: Generator}`: **the family has no
specializations anywhere in the record.** It is generated once and never
instantiated.

### 3.2 Marginality and credit

- Marginality: `MarginalNoClosurePreimage` against the typed predecessor
  closure of steps 1..13 (**37 families**, digest
  `blake3:963875e7d8eff61ce694f8d539a0ffd5fe8d8aff1d2d8404985232443fcc89ef`).
  Re-derived here; matches the sealed
  `fresh_marginality: marginal_no_preimage`.
- Family id re-derived from the kernel as
  `blake3:df4feb52882ec5b6a56472960c340c6a16be3b8c48dd25d389f9b5dca8e9e62b`,
  minted against the predecessor signature digest
  `blake3:098799481f6344c06c33e505ccac86eaf3a8fd3f5cb0843c37d186fc5538faac`
  — byte-identical to the sealed provenance package's
  `predecessor_signature_digest` and `credited_family_ids` entry. Full
  provenance chain reproduced.
- Anchor: `credited_local_role`, clause 3, role `kernel_head`.

### 3.3 What the hilbert-functional payment uses it for

Stage 14 extracts **9 families**, of which **6 are marginal**. Of those
six, **3 are credited** (ν₁₄ = 3) and **3 are refused credit** under
`theorem_impossible_no_relation` (theorem
`T-BI-B2-exact-family-local-role-relation-v1`). The three credited:

| Family | Clause | Concordance verdict |
|---|---|---|
| `67134aea1691` | 0 | named — rooted proof-relevant directed graph (pointed quiver) |
| `4efc2e4dac7a` | 2 | named — coalgebra for the squaring endofunctor |
| **`df4feb52882e`** | **3** | **NO-NAME** |

So df4feb52882e is **one third of stage 14's entire semantic payment**.
Note the company it keeps: its two credited siblings are clause 0
(`Σ(Π(X,Π(X,U)), X)`) and clause 2 (`Π(X, Σ(X,X))`) — and clause 0 is
precisely the field that the target's `Var 2` points at in situ.

### 3.4 The stage read as a whole

With X the carrier and `T₀` the clause-0 type:

| # | Reading |
|---|---|
| 0 | `(X → X → U) × X` — a binary universe-valued form on the carrier, with a distinguished vector |
| 1 | `X → X` — an operator |
| 2 | `X → X × X` — a copy/split map |
| **3** | **`(λ_. X) → (X × T₀)`** — the target |
| 4 | `(X → X) × (X → X)` — an operator pair (the adjoint-pair shape) |
| 5,6,7 | `L13 → X`, `L12 → X`, `L11 → X` — imports landing in the carrier |
| 8 | `λ_. (X → U)` — a constant family at the universe-valued predicate shape |

Two facts R-1 should hold onto:

- **Clause 3 is the only clause at stage 14 that references any other
  clause's field.** `Var 2` occurs nowhere else in the telescope; every
  other clause mentions only `Var 1`. Under the kernel's level semantics
  that reference is to clause 0.
- **The scalar/`Univ`-valued functional shape is available in this very
  grammar and clause 3 does not use it** (clause 8 is
  `Lam(Pi(Var 1, Univ))`). So a failed dual-space / functional reading of
  clause 3 is **not** a poverty-of-language artifact. This rebuttal, first
  made in the concordance, survives re-derivation and is re-registered.

## 4. Disclosures

- **Contamination (inherited F-C4).** This note reads the certified
  register directly and is not blind; it contaminates future blind work,
  exactly as XF-0b already declared.
- **Level vs index in the search layer, not load-bearing.**
  `Telescope::is_connected` (`crates/pen-core/src/telescope.rs`) tests
  clause connectivity with a *relative index* reading (`Var(j−i)` in
  clause `j` means clause `i`), which is a different notion from the
  frozen kernel's *absolute level* reading. The semantic register's
  family extraction runs entirely through `elaborate_telescope`, so the
  level reading is the one that governs everything in this note. The
  discrepancy is recorded for honesty and is used for nothing.
- **Granularity (inherited F-C3 / F-R4).** No clause was split, merged,
  or reinterpreted. §1.4 item 3 resolves a reference the kernel itself
  resolves; it does not fuse clause 0 into clause 3, and the credited
  family's canonical presentation still carries `Var 2` as a free `Type`
  parameter.

## 5. What R-1 is armed with

The armed naming attempts proceed on **the elaborated content of §1.3**,
not the gloss. In one line, twice:

- **Family form:** `Π( λ_. X , X × Y )` for type parameters X, Y — a
  Π-formation whose domain slot holds the constant family at X rather
  than a type, with a plain product codomain.
- **In-situ form:** `Π( λ_. X , X × T₀ )` at stage 14, where `T₀` is the
  carrier's inner-product-plus-vector datum.

Plus the three hard constraints any lawful translation must respect:

1. The clause is **equation-free**: a singleton formation clause carrying
   no laws. A target whose defining data include equations has no source
   counterpart for them.
2. The formation is **not well-sorted** in the kernel: two coarse
   subsumptions, `El(Lam(Var 1))` as the binder's classifier.
3. The family is **irreducible and un-instantiated**: no β/η route out,
   no specializations, one occurrence.
