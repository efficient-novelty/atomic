# Boundary adjudication: V2-declared-bound + zero boundary charge

**Date:** 2026-07-19. **Status:** **ADOPTED** 2026-07-19 (see ADOPTION
block). Prepared from `docs/BOUNDARY_AUDIT_RESULT.md`
(blake3:76c389…) and `docs/HIST_CERT_RESULT.md`. Supersedes
`tdc1-pathcon-attachment-implicit-base-constant-boundary-theory-axiom-v1`,
whose realizer and tokens remain valid as the constant special case.

## Axiom: `pathcon-attachment-declared-bound-boundary-theory-axiom-v2`

1. **Declared diagram.** A `PathCon(d)` attachment on a formed owner `A`
   elaborates to a declared boundary diagram: a typed term on each of the
   2d codimension-one faces of `I^d`, with identical declared restrictions
   on every pairwise face overlap, in a declared parameter context.
2. **Mandatory binding.** Every point and face term in the diagram must
   resolve to a sealed clause of the owner's package/predecessor library or
   to a declared context variable. No unbound symbols. (This closes the
   V1 base-binding gap for S1/S2/S3 by construction.)
3. **Constant case.** The everywhere-`base` diagram is the degenerate
   special case; all v1 source-bound tokens and the proven key-class
   bijection 1 + d + d(d−1) = 1 + d² carry over unchanged there.
4. **Registered interpretation.** The four historical diagrams audited in
   BOUNDARY-AUDIT B-1 are adopted as part of this axiom, verbatim:
   - S¹: `loop(0) = base`, `loop(1) = base`;
   - Trunc: in `A : Type; x, y : Trunc(A)`: `squash(x,y)(0) = x`,
     `squash(x,y)(1) = y`;
   - S²: all four faces `base`; S³: all six faces `base`.
   These are the explicit form of the constructors' standing HIT readings;
   the audit proved the sealed bytes cannot supply them (V3
   underdetermination), so they are registered as interpretation, not
   derived as fact.

**Grounds.** (i) Uniformity: the only implemented rule whose structural
checks accept all four historical constructors. (ii) It types the intended
Trunc squash, which V1 rejects by rule — under V1 the semantic re-audit of
Step 6 is permanently impossible. (iii) Conservativity: the constant case
reproduces V1's proven presentation exactly. (iv) Minimality: it declares
only what the sealed constructor names have always claimed. None of these
grounds references any Step-16 quantity.

## Charging convention: `boundary-charge-zero-reference-only-v1`

1. **Zero glue charge.** Declared boundary data contributes no novelty
   credit: the path-basis ceiling remains 1 + d², c(b) = 0.
2. **Reference-only boundaries.** Boundary terms may only refer — to
   sealed clauses or context variables. A component not so expressible is
   not boundary data; it must be hoisted to its own telescope clause,
   where it is charged, anchored, and audited as a family like any other.
3. **No double charge.** A hoisted clause's family credit is ordinary
   clause credit; its subsequent citation in a diagram adds nothing.

**Grounds.** (i) **Trace-derived:** the sealed record already prices
boundaries at zero — Trunc's endpoint-dependent squash, the richest
boundary in the trace, has recorded count 2 = 1 + 1², identical to a
constant loop; any c(b) > 0 retroactively falsifies the recorded totals.
The convention thus has the trace-derivability the boundary rule itself
provably lacks. (ii) **T4 coherence:** a declaration is a premise of the
attachment, not a family produced by it; charging mentions of existing
structure is the extraction pattern already burned three times (fallback
|library| charge, P5 self-import chain, P6 per-entry credit). (iii)
**Completeness without surcharge:** the hoisting rule ensures genuinely
new boundary content is charged — as a clause. Slogan of record: *charge
lives on clauses, never on the glue.* Noted as consequence, not
motivation: c(b) = 0 keeps all path ceilings dimension-determined, so
existing zone arithmetic survives V2 unchanged.

## Falsifiers (registered with the adoption)

- **F-B1.** A registered historical diagram fails term-level typing once
  Agent C's realizers exist → the *interpretation* is wrong; revise the
  diagram in a versioned successor axiom; never weaken the checker.
- **F-B2.** HIST-CERT rerun under this axiom fails to reproduce the raw
  presentations 7/8/10/18 or path marginality 2/2/5/10 → the diagrams or
  the convention misread the trace; obstruction exit, report verbatim.
- **F-B3.** A historical boundary component found inexpressible
  reference-only, whose hoisting would alter a sealed κ → the convention
  conflicts with the sealed ledger; report — that is a discovery about
  the record, not a bug to absorb.

## Consequences upon adoption

1. Agent C's brief gains the five V2 term-level obligations
   (BOUNDARY-AUDIT "Remaining proof obligations") plus the S1/S2/S3
   binding witnesses; TDC v3's coe/hcom/eliminator work is the starting
   stock.
2. HIST-CERT reruns create-new under this axiom once the historical
   fragment's realizers exist; its rows drop
   `conditional_on_boundary_axiom_v1` for
   `conditional_on_boundary_axiom_v2` until F-B1/F-B2 are discharged,
   after which the flag clears.
3. All downstream basis formulas cite 1 + d² with c(b) = 0 by this
   convention's name and version.

## ADOPTION

**Adopted.** Recorded per the A5 pattern; replay should pin this text:

> I adopt `pathcon-attachment-declared-bound-boundary-theory-axiom-v2`,
> including its four registered historical diagrams, and
> `boundary-charge-zero-reference-only-v1`, as axioms in the context of
> this program.
>
> — Halvor, 2026-07-19

Provenance: the user's adoption message of 2026-07-19, verbatim: "Yes, I
adopt these recommendations." Any future amendment goes through a
versioned successor document; no silent or partial modification.
