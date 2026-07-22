# Closure adjudication: ambient formers over Internal premises

**Date:** 2026-07-20. **Status:** **ADOPTED — Option A** 2026-07-21 (see
ADOPTION block). Prepared from `docs/SCHEMA2_GLOBAL_E4_V5_RESULT.md`
(surviving witness `[Univ, Lam(App(Lib(14), Lib(15)))]`, app-stuck body)
and the internality ladder (inductive-projection, guarded, structural-
lambda rules). Two options are offered: the general rule (recommended)
and the application-only rule (conservative). They are separable.

## The question

Does application — and more generally, each ambient former — belong to
the zero-credit Internal closure when its premises are Internal? This is
the frozen spec's S4 question (old-headed stuck terms) posed at
constructor level, and it is the question whose answer completes the
certified construction of the closure operator D(B₁₅) that A2/A3, R2,
and C(W) all cite.

## Option A (recommended): `ambient-former-closure-internal-rule-v1`

1. **The closure.** A term formed by a **transparent ambient former**
   over premises that each carry replayable Internal certificates earns
   structural Internal with marginal ν = 0, provided: the former belongs
   to the frozen expression grammar (ambient, not candidate-exported);
   the typed result is preserved through frozen elaboration and
   normalization; and full provenance is retained. Stuck results are
   acceptable: stuckness is a computation fact, not a novelty fact
   (app-stuck bodies included).
2. **The registered former partition.** The frozen former inventory is
   partitioned, once, by Definition 6 applied at former level:
   - **Transparent (law-like) formers** — their action on sealed content
     is generated uniformly by D: `Lam`, `App`, the four Step-10 modal
     formers, the two temporal unary formers, and the base/binary
     formers of the frozen grammar. Grounds: J3 (temporal action is
     law-like), the adopted Step-10 trace signatures (four formers, no
     witness clauses), and A2 (D is closed under ambient formation).
   - **Charged (witness-bearing) formers** — they assert inhabitants or
     cells not determined by the laws: `PathCon` (and any
     candidate-declared formation). These are **excluded** from the
     closure: a PathCon over Internal premises is a HIT/V2 candidate,
     never Internal by this rule. The classifier priority already routes
     it to its adopted content class.
3. **Zero credit; the orbit exception stands.** No closure result mints
   or anchors a family. If a formed term is an independently exported
   required output of a live demand orbit, that credit path runs through
   the orbit machinery exclusively (the standing EGP rule), never
   through this closure.
4. **Fail-closed scope.** Any former application involving a
   candidate-fresh head, a premise without a replayable Internal
   certificate, or a former outside the registered transparent list
   fails closed and stays where the classifier puts it — adopted class
   or named F-G4. Forward, cyclic, and uncertified dependencies inherit
   the ordered-prefix discipline of the inductive rule.

**Grounds (non-numeric, non-verdict).** (i) A2: D is extensive and
monotone — closure under ambient formation *is* its definition; this
rule is the last constructor-level installment of building D as a
certified object. (ii) The E-3 provenance principle and its mirror:
normalization cannot launder reference into novelty, and formation
cannot compound reference into novelty — a term whose provenance is
wholly sealed references is reference. (iii) Definition 6 supplies the
transparent/charged partition at former level, with J3 and the adopted
Step-10 signatures grounding the specific memberships. (iv) T4:
derivable content earns zero. No clause consults a count, verdict,
pending case, or the bar.

**Noted consequence (not a ground).** This rule's application case also
renders the C-5 P5 lift witness (`App(Lib(14), Lib(13))`) decidable
under the same discipline, converging two open items on one rule.

## Option B (conservative): `application-closure-internal-rule-v1`

Clause 1 restricted to `App` alone, with identical premises, charging,
orbit exception, and fail-closed scope. Each remaining former (modal,
temporal, binary/base) would then require its own successor adjudication
as fail-fast witnesses reach it. Sound but iterative: expect several
more Unknown→rule→rerun cycles.

## Falsifiers (both options)

- **F-A1.** A closure classification with any premise lacking a replayed
  Internal certificate → invalid on its face.
- **F-A2.** A closure-certified term later exhibits a certified marginal
  family → soundness event; report verbatim; the rule, not the taxonomy,
  is at fault (F-I3 pattern).
- **F-A3.** Any route admitting a candidate-fresh head through the
  closure → invalid.
- **F-A4.** A former on the transparent list fails its law-like proof
  obligations at term level → partition revision in a versioned
  successor; the failing former moves to charged; no in-place edits.
- **F-A5.** The next assembly rerun finds a further Unknown → F-G4
  continues on its own terms; neither option is an exhaustion license.

## ADOPTION

**Option A adopted.** Recorded verbatim from the user's adoption message
of 2026-07-21 (replay pins this text):

> I adopt `ambient-former-closure-internal-rule-v1`, including the
> registered transparent/charged former partition with PathCon excluded,
> the zero-credit clause with the standing orbit exception, and the
> fail-closed scope.
>
> — Halvor Lande, 21 July 2026

Option B is not adopted and is retained above for the record only.
Amendment goes through a versioned successor; no silent modification.
