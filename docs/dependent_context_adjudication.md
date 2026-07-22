# Versioned extension adjudication: dependent ambient contexts

**Date:** 2026-07-22. **Status:** **ADOPTED** 2026-07-22 (see ADOPTION
block).
Prepared from `docs/SCHEMA2_E5_FUTURE_HOLE_FINALE_V2_RESULT.md`
(F-FH4: `a3_v2_parametric_internality_failed`; the `[Type, Type]`
eventually-eliminator schema is not substitution-total — the certified
assignment p₁ := Lib(14), p₂ := Univ specializes to
`Lam(App(Eventually(Lib(14)), Univ))`, kernel-rejected as
`BareUnivArgument`; 16 accepted unary rows recorded totality-provisional)
and `docs/R_T2_FUTURE_HOLE_CONFLUENCE_V2_RESULT.md` (every branch's
fifth successor scheme requires ambient arity 3 and an `Opaque` third
parameter of sealed kernel type `Function(Neutral, Type)`, which is not
Bₙ-formable as a freestanding motive). One expressivity deficit, three
faces: no dependency between hypotheses, no arity above two, no motive
that *is* a sealed prior clause's type. This is a versioned successor to
`ambient-telescope-candidate-wrapper-v1` and an extension under
`future-hole-hypothesis-definition-v1`; neither adopted document is
modified.

## Rule: `dependent-ambient-context-v1`

1. **Dependent telescopes.** The ambient telescope generalizes from an
   ordered list of independent motives to a genuine dependent context
   Γ = (p₁ : M₁, p₂ : M₂(p₁), …, pₖ : Mₖ(p₁,…,pₖ₋₁)), each Mᵢ formable
   in the context extended by its predecessors. The wrapper discipline
   carries over verbatim: the context is declared at registration,
   enumerated, never inferred, never selected by verdict.
2. **Sequential typed substitution.** Specialization checks assignments
   left to right, each against its motive instantiated at the
   previously checked assignments; discharge remains clause-4′
   instantiation, now under sequential substitution. No simultaneous or
   reordered substitution is defined.
3. **Totality obligation.** A registration is *authoritative* only
   together with a total-specialization theorem: every kernel-admissible
   assignment of the declared context yields a kernel-accepted
   specialized body. Exact-body evidence without the theorem leaves the
   row provisional, exactly as finale-v2 recorded its 16. Totality is
   restored in exactly two lawful ways: (a) a dependent re-declaration
   that states the true type (the eventually eliminator's second
   parameter depends on its first — the honest context says so), or
   (b) a versioned, published narrowing of the declared motive. It is
   never restored by assignment-level filtering: a predicate that
   excludes falsifying specializations by outcome (the stable-source
   route) is admissible as diagnostic instrumentation only, never as a
   gate.
4. **Declared arity.** Context arity is part of the registration,
   finite, and unbounded by rule. The frozen kernel's arity-2 maximum
   is recorded as implementation limit, not law; the exact-context API
   extends to the declared arity. No default cap replaces it.
5. **Opaque prior-clause motives.** A hypothesis may declare as its
   motive *the sealed kernel type of a named prior clause*, referenced
   into the sealed prefix by digest and position — the adopted
   certified-field dereference, used at the type level. Opacity is a
   discipline: the referenced type is used as-is, never decomposed
   (`Neutral` is not analyzed, only matched); a specialization at an
   opaque parameter is accepted exactly when its term is certified to
   inhabit the referenced sealed type. This is reference to what
   exists, not formation of anything new: zero charge, unchanged.
6. **Charging and jurisdiction.** Hypotheses remain zero κ, zero ν, no
   anchors (F-FH3 standing). This rule opens no gate by itself:
   semantic O(16), F1, T-BF2, R-T2 comparison, and the bridge remain
   exactly as the v2 certificates left them until the two experiments
   rerun create-new under this rule.

**The road not taken, retained for the record.** Finale-v2 names a
second coherent repair: a typed `Eventually` eliminator rule making the
application total. Not adopted: it would add former-specific law to the
kernel, one new rule per operator encountered, where the dependent
context states the same truth once, generically, by identification with
adopted machinery (wrapper order, contextual internality, dereference,
clause 4′). It remains available to a future versioned adjudication if
the dependent route hits a stop this document cannot name.

**Grounds (all dated, none numeric, none verdict-shaped).** (i) The
wrapper adoption already declared the telescope *ordered* — order is
what dependency makes meaningful; v1's independence was silence, not
doctrine. (ii) The contextual-internality rule: motive-typed hypotheses
with instantiation coherence are the adopted shape; dependency is that
shape stated without loss. (iii) The certified-field dereference rule:
reading sealed content is lawful and free; clause 5 is its type-level
instance. (iv) The two v2 stops themselves, as exact machine-named
gaps (F-FH4 both times) — cited as the *occasion* of this rule, never
as its justification; the justification is that the registered schemes'
honest types are dependent and opaque, and a language that cannot state
them forces false declarations (the `[Type, Type]` schema is the
exhibit).

## Falsifiers

- **F-DC1 (regression).** The full corpus — the verified O-ladder, the
  13 structural realizations, the 26 rejected controls, the focus
  projection, and finale-v2's 16 provisional rows — must re-derive
  under dependent machinery, with the 16 rows either promoted by
  totality theorems or narrowed by published re-declaration. Any
  historical discharge failing to replay → the extension is wrong;
  fix the machinery, never the history.
- **F-DC2 (no outcome filtering).** Any context, motive, arity, or
  narrowing inferred from, repaired by, or selected because of a
  specialization outcome → invalid on its face. Diagnostics may
  observe outcomes; declarations may not.
- **F-DC3 (no partial credit).** Any row lacking its totality theorem
  contributes nothing: no O(16) issuance, no F1 execution, no
  comparison membership. Provisional means provisional.
- **F-DC4 (sealed reference only).** An opaque motive whose reference
  does not resolve by digest into the sealed prefix, or whose use
  decomposes the referenced type → invalid.
- **F-DC5 (rerun identity).** The two successor experiments rerun as
  the same issuers, create-new, unchanged in scope declaration; any
  silent scope drift from the archived A3-v2 / Global-E4-v10 surface
  beyond what those certificates already record → named gap, not
  repair.
- **F-RT / F-FH (standing).** The tie ladder and future-hole
  falsifiers are unchanged: this rule *enables* the fifth scheme's
  formation on all four branches; it does not predetermine confluence,
  and R-T3 stays closed until R-T2 speaks.

## Consequences upon adoption

The eventually eliminator becomes declarable at its honest dependent
type; the 16 provisional rows acquire their totality obligations; the
17th row forms or names its gap under F-DC2 discipline. The four
Stage-5 branches can form all five successor schemes, making the six
pairwise comparisons lawful for the first time. E-5 finale and R-T2
confluence rerun create-new; only their new certificates — never this
adoption — can issue semantic O(16), execute F1, authorize T-BF2, or
decide confluence. The bridge remains closed until both deliver.

## ADOPTION

**Adopted.** Recorded verbatim from the user's adoption message (replay
pins this text):

> I adopt `dependent-ambient-context-v1`: ambient telescopes are
> dependent contexts with sequential typed substitution; registrations
> are authoritative only with total-specialization theorems, restored
> by honest re-declaration or published narrowing, never by outcome
> filtering; arity is declared, not capped; opaque motives are
> dereferences of sealed prior-clause types, used whole, minting
> nothing.
>
> — Halvor Lande, 22 July 2026

Amendment goes through a versioned successor; no silent modification.
