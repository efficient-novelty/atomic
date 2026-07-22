# Versioned extension adjudication: support-comprehension contexts

**Date:** 2026-07-22. **Status:** **ADOPTED** 2026-07-22 (see ADOPTION
block).
Prepared from `docs/NU_REGISTER_EXECUTION_RESULT.md` and
`docs/CHRONOLOGICAL_INTERFACE_SLOT_MAP_V4_RESULT.md`
(`blake3:1b00718c…`): T-SM1b derives 17/18; the unique residual is
instance `blake3:4c9c4272…` (older Step-14 clause 3, newest Step-15
clause 6), blocked as `T_SM1B_DEPENDENT_TARGET_CYCLE` — "motive for
parameter 2 depends on non-predecessors [1, 2]." The diagnosis: the
arity-2 presentation `(p1 : Type, p2 : Type)` is a flattening of a
comprehension. The instance's honest dependent context is
`(q1 : Type, q2 : Type, x : El(Eventually(F(q1, q2))))`, with the
interface slots realized as p1 ↦ F(q1, q2) and p2 ↦ x. The apparent
self-dependence factors through the composite; nothing is cyclic in
the honest type. This is a versioned successor to
`chronological-interface-slot-map-v1` and an extension under
`dependent-ambient-context-v1`; neither adopted document is modified.

## Rule: `support-comprehension-context-v1`

1. **Support contexts.** A chronological registration may declare,
   beside its interface, a *support context* Γ_s — a dependent ambient
   context under the adopted rules (sequential substitution, declared
   arity, totality obligation) whose parameters are the instance's
   independent generators.
2. **Realized slot map.** Interface slots map, in sealing order, to
   declared *realizers*: expressions over Γ_s, formable, zero-charge,
   consumed whole. The adopted identity map is the degenerate case in
   which every realizer is a bare parameter; this rule strictly
   extends it and changes no instance the identity map already
   derives.
3. **Canonicity, not choice.** The support context is not free: it is
   computed by canonical dependency analysis of the sealed instance —
   parameters are the maximal independent generators of the motive
   graph; realizers are the canonical decomposition of the interface
   sorts over them. One instance, one honest context. Any hand-shaped
   context, and any context selected because a derivation succeeds
   under it, is invalid on its face (the F-DC2 line, unchanged).
4. **Well-foundedness restored, and proved.** Motives over Γ_s
   reference predecessors only; the registration is valid only with
   the acyclicity proof of its motive graph. A dependency that
   survives canonical decomposition still cyclic is a named
   impossibility, not a candidate for further flattening or padding.
5. **Charging.** Support parameters and realizers are hypotheses and
   references: zero κ, zero ν, no anchors (standing law, unchanged).

**Grounds (all dated, none outcome-shaped).** (i) The
dependent-context adoption already established the repair class:
totality and coherence are restored by *honest re-declaration* of the
true type, never by outcome filtering — this rule is that clause
applied to a context that was flattened, not merely under-declared.
(ii) The wrapper discipline (adopted): interpretation data is
declared, enumerated, never inferred from verdicts; clause 3's
canonicity requirement is that discipline made checkable. (iii) The
certified-field dereference and future-hole definitions (adopted):
references to sealed content and typed hypotheses are zero-charge
interface. (iv) The v4 cycle diagnostic as the exact stop — occasion,
never ground.

## Falsifiers

- **F-SC1 (regression, non-negotiable).** The unchanged positive gate
  reruns in full: 72/72 sealed chronological discharges derived, 9/9
  former-gap members recovered, zero named gaps — and every one of the
  71 previously derived cases must re-derive *unchanged*. Any
  previously derived case shifting under this rule → the extension is
  wrong; versioned successor; the history never moves.
- **F-SC2.** Any support context not reproducible by the canonical
  dependency analysis from the sealed instance alone → invalid. The
  analysis procedure itself is part of the create-new artifact and
  replays.
- **F-SC3.** Any credit, anchor, or orbit minted through a support
  parameter or realizer → invalid (standing zero-charge law).
- **F-SC4.** A registration lacking its acyclicity proof → not
  derived; residual stands; no partial credit (F-NR5 pattern).
- **F-SC5 (scope).** This rule touches chronological realization
  only. The three T-BI construction obligations — the prefix-local
  `Expr → Schema2` bridge, the exact family-to-role anchor theorem,
  and transitive intrinsic-isolation — proceed under already-adopted
  law and acquire no new semantics from this adoption; any of them
  requiring more → its own named gap and versioned adjudication.

## Registered construction obligations (no new law; named for tracking)

- **T-BI-B1:** candidate-and-prefix-local bridge from `pen_core::Expr`
  normalization to the ordinary `Schema2` family calculus.
- **T-BI-B2:** exact family-to-local-role anchor relation theorem —
  proves a family or proves its collision impossible; labels are
  search candidates, never verdicts.
- **T-BI-B3:** transitive intrinsic-isolation/capability theorem for
  the pre-seal call graph; source scans remain diagnostics.

## Consequences upon adoption

The cyclic instance re-registers at its honest comprehension type and
derives or fails under F-SC1's full-gate rerun; T-SM1b's surface can
reach 18/18 and the fixed positive gate 72/72 with 9/9. Together with
T-BI-B1..B3, that is the complete named path back to a BI-0 attempt
under F-AL1′. Nothing in this adoption issues a ledger, a divergence
table, a BI verdict, or a branch: F-NR3/F-NR4 remain in force until
extraction completeness is real.

## ADOPTION

**Adopted.** Recorded verbatim from the user's adoption message (replay
pins this text):

> I adopt `support-comprehension-context-v1`: chronological interface
> slots may be realized by canonical expressions over a declared
> support context computed from the sealed instance's dependency
> structure — honest comprehension, never flattening; acyclicity
> proved, canonicity checkable, nothing minted; the identity map
> stands as the degenerate case and every previously derived discharge
> must re-derive unchanged.
>
> — Halvor Lande, 22 July 2026

Amendment goes through a versioned successor; no silent modification.
