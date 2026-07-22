# Successor adjudication: motive-parametric instantiation coherence

**Date:** 2026-07-21. **Status:** **ADOPTED** 2026-07-21 (see ADOPTION
block). Prepared from `docs/SCHEMA2_GLOBAL_E4_V9_RESULT.md`
(surviving wrapped Unknown: v8 clauses under declared motive
`Type -> Element(Univ)`; failure at instantiation coherence, "no
registered closed probes inhabit ambient motive"; named gap
`WRAPPED_CONTEXTUAL_INSTANTIATION_COHERENCE_NOT_PROVED`). This is a
versioned successor to clause 4 of the adopted
`motive-sensitive-contextual-internality-rule-v1`; all other clauses of
that rule are unchanged.

## The defect being repaired

Clause 4 as adopted discharges instantiation coherence by *registered
closed probes* — per-motive test data. The wrapped domain enumerates the
full admissible motive space; per-motive registration cannot cover it,
and restricting the grammar to probed motives would violate the adopted
declaration-independence rule. Probes were the scaffolding; the theorem
is the building.

## Clause 4′: `motive-parametric-instantiation-coherence-v1`

1. **The generic theorem.** Instantiation coherence is discharged by a
   replayed, motive-parametric substitution theorem: for every
   admissible B₁₅-formable motive vector Γ, every contextual-Internal
   derivation over Γ, and every assignment σ of closed Internal terms to
   Γ's parameters (each motive-typed), the specialized derivation
   replays under the closed closure rules. The proof is by structural
   induction over the closure rule inventory (projection, guarded,
   structural, ambient-former, dereference, contextual), quantifying
   over Γ and σ generically — it cannot be tuned to any candidate.
2. **The vacuous case.** A motive with no closed Internal inhabitant has
   no assignments; coherence holds vacuously, and the hypothetical
   derivation stands on its typed structure — standard hypothetical-
   judgment logic (derivability under hypotheses does not presuppose
   their satisfiability). Credit remains zero regardless, so the vacuous
   case carries no valuation risk.
3. **Probes demoted, not deleted.** Registered probes become regression
   spot-checks of the generic theorem, never the evidence. No motive is
   excluded, preferred, or filtered by probe availability.

**Grounds (non-numeric, non-verdict).** (i) The theorem's ingredients
are already proven piecewise: E-1's typed-instance substitution, the v4
induction's genuine non-renaming substitution across every registered
constructor, and F-D2's dereference-invisibility (substitution
commutation in the special case). Clause 4′ assembles adopted parts into
the general lemma they were always converging on. (ii) Generic
quantification over all admissible Γ and σ is the strongest possible
form of verdict-blindness — a theorem about every motive can favor none.
(iii) The vacuous clause is classical hypothetical-judgment logic, with
T4's zero-credit law closing the only conceivable abuse.

## Falsifiers

- **F-M1.** A motive/instance pair on which the generic specialization
  fails to replay closed → a closure rule is not substitution-stable;
  this is a soundness event infecting prior rungs — report verbatim,
  immediately; the certificate machinery, not the taxonomy, is at fault.
- **F-M2.** The theorem provable only by restricting the motive grammar
  → the forbidden filter by other means; stop and report; any grammar
  restriction requires its own adjudication on non-verdict grounds, as
  the v9 artifact itself states.
- **F-A5 (standing).** A surviving Unknown in the v10 rerun → the ladder
  continues honestly; no exhaustion license.

## ADOPTION

**Adopted.** Recorded verbatim from the user's adoption message of
2026-07-21 (replay pins this text):

> I adopt `motive-parametric-instantiation-coherence-v1` as clause 4′ of
> the contextual internality rule, discharging instantiation coherence
> by the generic substitution theorem over all admissible motives and
> closed Internal instances, with the vacuous case as stated and probes
> demoted to regression checks.
>
> — Halvor Lande, July 21, 2026

Amendment goes through a versioned successor; no silent modification.
