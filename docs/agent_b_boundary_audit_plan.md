# Agent B — BOUNDARY-AUDIT: evidence for the attachment-axiom adjudication

> **Addendum 2026-07-19 (post HIST-CERT v1).** `docs/HIST_CERT_RESULT.md`
> is now admissible context and changes the starting position:
> HIST-CERT found a **concrete V1 mismatch** — the Trunc squash constructor
> has endpoint-dependent boundary data, and the frozen constant-boundary
> rule can only type it as a generic-loop surrogate. So B-2 begins with one
> known fixed point: V1 fails historical fit at Trunc; V3 (trace-derived)
> differs from V1 at least there. The audit's central question sharpens to:
> **is there one uniform rule (presumably V2, with constant boundary as a
> special case) that types all four historical constructors** — or does the
> trace force per-constructor rules, which would multiply adjudications?
> Report the general-d basis formula for whichever rule(s) fit; that
> formula replaces 1 + d² in all downstream ceilings, which is precisely
> why it must be derived here from the trace and never evaluated against
> any Step-16 quantity (standing prohibition unchanged).

**Standing:** construction task producing *decision evidence*, not a
decision. The adjudication of the boundary rule is the user's alone; this
artifact must contain no recommendation. Admissible context:
`docs/TDC1_CUBICAL_RESULT.md`, the sealed Genesis trace, `pen-type::cubical`.
Forbidden: the bar value; scoring the Step-16 candidate under any variant;
choosing or tuning variants by their effect on Step-16 certification.

## Mission

The TDC v3 result is conditional on the unadopted rule
`tdc1-pathcon-attachment-implicit-base-constant-boundary-theory-axiom-v1`
(implicit base `a : A`, free `p : I^d → A`, constant boundary
`p|∂I^d = a` — attachments as based d-loops). Convert this from an
arbitrary modeling choice into an evidence-informed adjudication by
implementing the rival boundary theories and testing each against the one
authority that exists: the sealed historical trace.

## Variants to implement (feature-gated module `pen-type::cubical::boundary_variants`; the v1 code path must remain byte-identical)

- **V1 (incumbent):** implicit-base constant boundary, as shipped.
- **V2 (declared boundary):** attachment along an explicitly typed boundary
  map `∂I^d → A`, general position; constant boundary as a special case.
- **V3 (trace-derived):** reverse-engineer what the sealed grammar's
  historical path constructors actually did — S¹'s loop, Trunc's squash
  path, S²'s surface cell, S³'s 3-cell — and formalize *that* as a rule.
  If the trace underdetermines the rule, record the ambiguity precisely;
  do not resolve it by fiat.

## Work items

1. **B-1.** Elaborate the four historical path constructors under each
   variant. Record: type-checks or fails, and why.
2. **B-2.** Basis replay per variant: the presented basis cardinality at
   d = 1, 1, 2, 3, compared with the recorded 2/2/5/10. A variant that
   cannot reproduce the historical alignment is evidence against it —
   record, don't delete.
3. **B-3.** General-d basis cardinality formula per variant (V1 is 1 + d²;
   derive the analogues for V2/V3), with proofs at the level the fragment
   supports. These formulas are *reported*, never evaluated at d = 4
   against any threshold.
4. **B-4.** Kernel-derivability notes: for each variant, what (if
   anything) in the frozen Two-Law kernel or the sealed grammar's PathCon
   semantics forces or forbids it.
5. **B-5.** Artifact `docs/boundary_audit_v1.json` (create-new) +
   `docs/BOUNDARY_AUDIT_RESULT.md`: comparison table
   (variant × {historical fit, basis formula, derivability}), ending with
   an explicit **ADJUDICATION REQUIRED** block listing the options and
   their recorded consequences — no ranking, no recommendation.

## Falsifiers / exit conditions

- If **no** variant reproduces the historical alignment, that is a finding
  about the presented-basis convention itself; report and stop.
- If **V3 ≠ V1** on the trace (the sealed constructors were not based
  loops), the TDC v3 conditionality is *material*, not formal — flag
  loudly; Agent A's per-step conditionality flags become load-bearing.
- Mutation battery: flipping any fit verdict or formula must invalidate
  replay.

## Done

Table complete for V1–V3 over all four historical constructors; artifact
replays; the incumbent code path untouched; no d = 4 evaluation anywhere.
