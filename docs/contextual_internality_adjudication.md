# Closure adjudication: motive-sensitive contextual internality

**Date:** 2026-07-21. **Status:** **ADOPTED** 2026-07-21 (see ADOPTION
block). Prepared from `docs/SCHEMA2_GLOBAL_E4_V7_RESULT.md`
(surviving witness `[Univ, Lam(App(Var(3), Lib(15)))]`, one ambient
parameter, live ambient dependency in head position; closed-only refusals
from both the ambient-former and dereference tokens; weakening/erasure
inapplicable because the parameter is used).

## The question

All six adopted closure rungs certify derivability of *closed* content.
The v7 witness's clause is an *open judgment*: not derivable over B₁₅,
but derivable over B₁₅ extended by its ambient hypothesis — uniformly,
and type-correctly against that hypothesis's declared motive. Does the
Internal partition include hypothetically derivable families? This rule
answers yes, under the discipline the program has already adopted for
hypotheses elsewhere.

## Rule: `motive-sensitive-contextual-internality-rule-v1`

1. **Open-judgment closure.** A guarded candidate with ambient telescope
   Γ is Internal iff every clause is derivable over B₁₅ ∪ Γ by the
   adopted closure rules, with ambient parameters admitted as typed
   hypotheses — reference-only, usable in any premise position including
   application head, self-certifying *within their scope only*.
2. **Motive-sensitivity.** Every use of an ambient parameter must
   type-check against its declared motive in Γ through frozen
   elaboration and normalization; ill-typed use fails closed. The
   certificate records Γ's motives and the complete typed derivation.
3. **Ambient motives must be B₁₅-formable.** Every type in Γ must itself
   elaborate over B₁₅ (through the adopted closure). A candidate whose
   ambient telescope smuggles a fresh formation or charged structure
   into its hypotheses fails closed at this clause — hypotheses are
   interface, never a side door for content.
4. **Instantiation coherence.** The hypothetical derivation must
   specialize: instantiating Γ at any closed Internal arguments must
   yield a derivation replayable under the closed rules. This is checked
   on registered probe instantiations and is a standing soundness
   obligation, not a one-time test.
5. **Zero credit; standing exceptions.** An open-judgment Internal
   family mints nothing (one uniform hypothetical derivation = one
   derivable family, per the adopted family-versus-instance law). The
   orbit exception and PathCon's charged status are unchanged. The
   existing unused-parameter weakening/erasure theorem remains its own
   evidence type; this rule neither subsumes nor weakens it — a live
   dependency claims hypothetical derivability, never erasure.

**Grounds (non-numeric, non-verdict).** (i) **A3 as written:** the
Guard-Rail's demand schema has contained *clause schemes with holes*
since its first statement — D was judgment-level from birth; the
closed-only tokens implemented a fragment of it, and this rule
implements the rest. (ii) **The endpoint-premise API precedent:**
hypotheses within a sequent are zero-charge, self-certifying interface
(adopted clause 4 of that rule); ambient parameters are hypotheses.
(iii) **T4/family-versus-instance:** the family's instances at Internal
arguments are already Internal under adopted rungs; one uniform
derivation covering all of them is one derivable family. (iv) **The
dereference precedent:** certified reference in head position is already
adopted; a hypothesis is certified by its declaration within its scope.
No count, verdict, pending case, or bar appears in any clause.

**Noted consequence (not a ground).** Open-judgment closure is exactly
the evidence form that C(W)'s demand *schemes* require for their
D-membership decisions — this rung, like its predecessors, builds a
piece of D that Agent D's E-5 phase was always going to need.

## Falsifiers

- **F-C1.** Ill-typed ambient use, non-B₁₅-formable ambient motive,
  forward/cyclic structure → fail closed; classification unchanged.
- **F-C2.** An instantiation of a certified open-judgment derivation
  failing to replay under the closed rules → the hypothetical derivation
  was not uniform; soundness bug event; report verbatim.
- **F-C3.** Any credit, anchor, or orbit minted through a hypothesis →
  invalid on its face (inherits the endpoint-API zero-charge law).
- **F-A5 (standing).** The next assembly rerun finding a further Unknown
  → the ladder continues honestly; no exhaustion license. Forecasts
  about the ladder's end have now been falsified twice and are hereby
  retired from the record; only F-A5 speaks.

## ADOPTION

**Adopted.** Recorded verbatim from the user's adoption message of
2026-07-21 (replay pins this text):

> I adopt `motive-sensitive-contextual-internality-rule-v1`, extending
> the Internal partition to open judgments derivable over B₁₅ plus their
> ambient hypotheses, with motive-sensitive typing, B₁₅-formable ambient
> motives, instantiation coherence, and the zero-credit clause unchanged.
>
> — Halvor Lande, 21 July 2026

Amendment goes through a versioned successor; no silent modification.
