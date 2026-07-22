# Procedural adjudication P1: E-2 split + monotone membership clause

**Date:** 2026-07-19. **Status:** **ADOPTED** 2026-07-20 (see ADOPTION
block). Prepared from `docs/SCHEMA2_V2_RESULT.md` (F-Q4 triggered;
dependency cycle: E-2's adopted R1/R2 quotients require E-3 frozen equality
and E-4 generator membership, while the strict order forbids E-3/E-4 before
E-2 completes). This amends the *procedure* of
`docs/agent_e_schema2_plan.md` only; no adopted rule (R1, R2, or any prior
axiom) is modified.

## Amendment A: the E-2 split

The phase order becomes:

```text
E-1  →  E-2a (typing)  →  E-3  →  E-4  →  E-2b (quotient closure)  →  E-5 … E-8
```

- **E-2a — typing milestone.** Registered signatures elaborated, ordinary
  constructor registry typed, package families issued where no membership
  test is required. **Declared complete** by `docs/schema2_v2.json`
  (Stage-1 completed package family; Step-8 μ/left-unit/cell-action typed;
  nine-kind registry with depth proofs).
- **E-2b — quotient-closure milestone**, executable only after E-4's
  completeness theorem: decide the four R1 carrier-exception role cases and
  the two R2 membership cases; mint the resulting individual
  ordinary-family tokens; run the revised Steps 5–8 count-as-output
  regression; evaluate F-Q2; issue the Agent A handoff. F-Q2 and all stage
  counts remain pinned to E-2b — no count finalizes earlier under any
  clause of this adjudication.

## Amendment B: the monotone membership clause (M1)

Generator membership is monotone in the basis: if a *proven sub-basis* of
the eventual E-4 basis generates a row's derivation, the full basis does.
Therefore:

1. As E-4 develops, any proven sub-basis may immediately issue **final
   verdicts of "generated"** for pending rows, each carrying the sub-basis
   digest and the generating derivation.
2. **"Independent" verdicts require the E-4 completeness theorem.** No
   sub-basis, however large, may issue one.
3. Interim "generated" verdicts are final by monotonicity. If E-4's
   completed basis ever appears to contradict one, that is not a
   revision event but a soundness bug in the sub-basis proof — a
   report-verbatim kernel event (F-P2).

M1 is count-blind: it accelerates only the direction that needs no
completeness, and it can only shrink the pending set, never decide a stage
count (counts wait for E-2b).

## Rejected alternative (recorded)

Option 1 of the v2 result — a minimal E-3/E-4 prerequisite kernel built
solely to decide E-2's six named cases — is declined: a bespoke sub-kernel
whose basis is chosen with the six cases in view risks prejudging the full
E-4 basis and creates verdicts requiring later reconciliation. M1 provides
the same early payout soundly, from the real E-4 as it grows.

## Falsifiers

- **F-P1.** Any "independent" verdict, ordinary-family token, stage count,
  or F-Q2 evaluation issued before the E-4 completeness theorem →
  certificate invalid.
- **F-P2.** A completed E-4 basis contradicting an M1 "generated" verdict →
  sub-basis soundness bug; report verbatim; the kernel proof, not the
  verdict rule, is at fault.
- **F-P3.** The E-2b count regression (F-Q2) is skipped or run against
  anything other than the completed membership verdicts → invalid.

## ADOPTION

**Adopted.** Recorded verbatim from the user's adoption message of
2026-07-20 (replay pins this text):

> I adopt procedural adjudication P1: the E-2a/E-2b phase split with
> quotient closure after E-4, and the monotone membership clause M1, as
> amendments to the Schema2 execution order. All adopted rules and axioms
> are unchanged.
>
> — Halvor Lande, 20 July, 2026

Amendment goes through a versioned successor; no silent modification.
