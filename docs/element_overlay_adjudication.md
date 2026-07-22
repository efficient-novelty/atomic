# Boundary axiom v3: the historical element-declaration overlay

**Date:** 2026-07-19. **Status:** **ADOPTED** 2026-07-19 (see ADOPTION
block). Prepared from `docs/KERNEL_BRIDGE_RESULT.md` (C-7
obstruction `C-7-historical-point-clause-not-typed-as-owner-element-v1`)
under falsifier F-B1's reporting branch. Successor to
`pathcon-attachment-declared-bound-boundary-theory-axiom-v2`, which it
contains verbatim; the four registered historical diagrams and
`boundary-charge-zero-reference-only-v1` carry over unchanged and remain
in force.

## The obstruction this resolves

The sealed shallow kernel types the historical point clauses (S¹ `base`,
S² `base`, S³ `base`) as `KernelTy::Type`. The adopted v2 diagrams require
binding targets of sort `KernelTy::El(owner)`. The checker correctly
refused to identify them. The sealed record, as recorded, does not
distinguish points from types: its grammar had no element sort to record
the distinction with. Element-hood is therefore interpretation — the third
such supplement (A5: evidence; v2: attachment; v3: sort) — and must be
registered explicitly, never inferred silently.

## Axiom: `pathcon-attachment-declared-bound-boundary-theory-axiom-v3-element-overlay`

Clauses 1–4 are v2 verbatim. Added:

5. **Element-declaration overlay** (`historical-element-declaration-overlay-v1`).
   A registered, versioned overlay assigns to designated sealed clauses an
   element-sort reading: an overlaid clause `c` of package `P` is read as
   `KernelTy::El(A_P)`, where `A_P` is `P`'s formation clause. The overlay
   is interpretation layered over the record; the sealed bytes, and every
   sealed ledger quantity (κ, ν, Σν, Σκ, bar history), are untouched by
   construction.
6. **Eligibility, per entry, machine-checked.** An overlay entry is valid
   only with a replayable *operational-role witness*: the kernel must
   confirm from the sealed telescope alone that the clause occupies a
   point/unit role — it follows its package's formation clause, and the
   package's path constructor's registered diagram binds to it. No witness,
   no entry; entries are never inferred from labels or names.
7. **Registered entries (initial).** Exactly three: the pre-path point
   clauses of Step 5 (S¹ `base`), Step 7 (S² `base`), and Step 8 (S³
   `base`), each read as `El` of its package's formation clause. Trunc
   registers no entry — its diagram binds context variables, and its
   type-checking success under plain v2 is the control showing the overlay
   is targeted interpretation, not blanket promotion.
8. **Conservativity.** Every result proved without the overlay must replay
   unchanged under it. The overlay may enable new typings; it may not
   alter, re-score, or re-classify anything already certified.

**Grounds.** (i) The constructor semantics has always claimed it: a clause
named and used as a base point *is* a point; the flattened sort is a
limitation of the recording grammar, not an assertion of the record.
(ii) Coherence with adopted v2: mandatory binding requires `El(owner)`
targets, so either the four adopted diagrams are wrong or the sort reading
needs this overlay — and the Trunc success plus the audited diagram fits
argue the diagrams are right. (iii) HIST-CERT's rule-driven grammar walk
already assigns these clauses the point/unit *operational* role; the
overlay makes the type-level reading agree with the operational role the
kernel itself computes. (iv) Targetedness: the Trunc control shows the
overlay adds exactly what binding needs and nothing more. None of these
grounds references any Step-16 quantity.

## Falsifiers

- **F-O1.** A registered entry's operational-role witness fails replay →
  the entry is rejected and reported; no fallback reading is substituted.
- **F-O2.** With the overlay in force, the S1/S2/S3 base bindings *still*
  fail term-level typing → the interpretation stack is wrong at a level
  below sorts; report verbatim and stop — that would be a discovery about
  the record's typability, not a bug.
- **F-O3.** Any sealed ledger quantity or previously certified verdict
  changes under the overlay → clause 8 conservativity broken; the overlay
  (not the prior result) is invalid.
- **F-B2 (inherited, sharpened).** The HIST-CERT rerun under v3 must
  reproduce the operational presentations 7/8/10/18 *and* the path
  marginality decisions 2/2/5/10. A marginality flip caused by the sort
  reading is an F-B2 event, reported, never absorbed.

## Consequences upon adoption

1. C-6/C-7 resume under v3: the base-binding witnesses become
   constructible in principle; obstruction
   `C-7-historical-point-clause-not-typed-as-owner-element-v1` is retired
   *only* by a replayed witness, not by the adoption itself.
2. Downstream conditionality flags advance from
   `conditional_on_boundary_axiom_v2` to
   `conditional_on_boundary_axiom_v3`; they clear only when F-B1/F-B2/F-O2
   are discharged by artifacts.
3. Agent A's HIST-CERT create-new rerun remains gated on C's realizer
   APIs under v3, per the standing ownership split.
4. Future overlay entries (if any package beyond 5/7/8 ever needs one) go
   through a versioned overlay successor with the same witness discipline;
   the entry list is closed until then.

## ADOPTION

**Adopted.** Recorded verbatim from the user's adoption message of
2026-07-19 (the A5 pattern; replay should pin this text):

> I adopt
> `pathcon-attachment-declared-bound-boundary-theory-axiom-v3-element-overlay`,
> including `historical-element-declaration-overlay-v1` with its three
> registered entries, as the successor of the v2 boundary axiom, with
> `boundary-charge-zero-reference-only-v1` unchanged and remaining in
> force.
>
> — Halvor Lande, July 19, 2026

Any future amendment goes through a versioned successor document; no
silent or partial modification.
