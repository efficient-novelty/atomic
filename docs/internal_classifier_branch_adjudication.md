# Classifier successor adjudication: the Internal branch

**Date:** 2026-07-20. **Status:** **ADOPTED** 2026-07-20 (see ADOPTION
block). Prepared from `docs/SCHEMA2_GLOBAL_E4_ASSEMBLY_RESULT.md`
(live F-G4 witness `[Formation: Univ, Introduction: Lam(Var(1))]`, candidate
digest `blake3:09e281cf…`) and `docs/SCHEMA2_G8_TOTAL_CLASSIFIER_RESULT.md`.

## What this is: a restoration, not an invention

The preregistered Phase-4 partition of `SEMANTIC_NORMALIZATION_PROGRAM.md`
(frozen 2026-07-18) specifies three branches:

```text
internal-by-weakening/erasure | EGP-certified marginal | invalid-or-unclassified
```

The G-8 classifier implemented named-exclusion | adopted-class | Unknown and
omitted the internal branch. The F-G4 witness is a well-typed candidate that
is wholly *derivable* — the grammar enumerated kinds of content and lacked
the kind that is no content. This adjudication restores the preregistered
branch, with proof obligations that make relabeling impossible.

## Rule: `internal-derivability-classifier-branch-v1`

1. **The branch.** `classify_raw_candidate` gains an `Internal` output,
   returnable only with a replayed **derivability certificate**: for every
   clause of the candidate, a typed derivation over the exact predecessor
   closure (B₁₅ for Step-16 candidates), through the frozen elaborator and
   normalization, with weakening/erasure inverse laws where the guarded
   form applies. No certificate, no `Internal` — the branch without proof
   is exactly the relabeling the assembly result forbids.
2. **Zero credit, by inheritance.** An `Internal` candidate contributes no
   marginal family and no ν (T4: derivable families are internal; this
   adds no new valuation clause).
3. **The witness obligation.** The exact F-G4 witness must be run through
   the new branch *first*: prove `Univ` (ambient-arena reference — the
   Stage-1/Step-16 asymmetry under adopted R1: arena-founding was an act
   once; arena-reference is not an act twice) and `Lam(Var(1))` (the
   identity, derivable in the frozen fragment) over B₁₅. Success classifies
   it `Internal` with the certificate attached; failure returns it to
   `Unknown` under its recorded obstruction, and the grammar question
   reopens — in neither case is the witness dropped.
4. **Priority order.** The classifier decision order becomes: surface/typed
   exclusion → `Internal` (certificate-backed) → adopted class → the F-G4
   obstruction branch. `Unknown` remains a live, never-droppable output.

**Grounds (non-numeric, non-verdict).** (i) The Phase-4 partition is
preregistered and frozen — this rule implements it. (ii) T4's law already
assigns derivable content zero marginal credit; the branch gives that
existing law a classifier home. (iii) Operational precedent: the IP-1
candidate join's locally-decisive bare-`Univ` rejection branch handled this
shape's kin at checker level; the typed system now handles it at proof
level. (iv) No count, verdict, bar value, or exhaustion desire appears in
any clause: the branch demands *more* proof, not less, and can fail on the
witness.

## Falsifiers

- **F-I1.** An `Internal` classification without a replayed derivability
  certificate → invalid on its face.
- **F-I2.** The F-G4 witness fails its derivability proof → it returns to
  `Unknown`; the assembly stays fail-closed; report verbatim — that would
  mean the frozen fragment cannot derive the identity, which is a kernel
  discovery, not a classification problem.
- **F-I3.** Any candidate classified `Internal` later exhibits a certified
  marginal family → branch soundness broken; report verbatim; the
  certificate machinery, not the taxonomy, is at fault.
- **F-I4.** Any future candidate found `Unknown` after this branch exists →
  F-G4 fires again on its own terms; this rule is not a license to expect
  exhaustion, only to prove it if it is true.

## Consequences upon adoption

Rerun global E-4 assembly (create-new successor): every raw candidate then
lands in exclusion, `Internal`-with-certificate, an adopted class, or a
named `Unknown`. If no `Unknown` survives, class exhaustion is proved and
the five pending membership verdicts, E-2b, F-Q2, and the Agent A handoff
unlock in the adopted order. If an `Unknown` survives, the program stops
there, honestly, again.

## ADOPTION

**Adopted.** Recorded verbatim from the user's adoption message of
2026-07-20 (replay pins this text):

> I adopt `internal-derivability-classifier-branch-v1`, restoring the
> preregistered internal partition branch with its mandatory derivability
> certificate, the witness obligation, and the retained Unknown branch.
>
> — Halvor Lande, 20 July 2026

Amendment goes through a versioned successor; no silent modification.
