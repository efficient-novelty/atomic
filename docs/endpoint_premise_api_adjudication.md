# Endpoint-premise API adjudication: the generic elimination ledger

**Date:** 2026-07-20. **Status:** **ADOPTED** 2026-07-20 (see ADOPTION
block). Prepared from `docs/SCHEMA2_V4_RESULT.md` (four private
endpoint-premise forms outside the public induction),
`docs/TRUNC_ENDPOINT_REALIZER_RESULT.md` (the proven instance), and
`docs/SCHEMA2_M1_SWEEP_RESULT.md` (contamination hazard, §Firewall).

## What is being generalized

TRUNC-ER proved one instance: the registered Trunc bundle types its
endpoint-dependent eliminator *as a sequent* under an opaque, source-bound
premise ledger — motive `P`, endpoint evaluations `e0 : P(x)`, `e1 : P(y)`,
and a PathP method `q` with recorded restriction equations — with every
premise derived from the replayed typed-boundary token by exact context
lookups, and ordinary inference rejecting all premise references. Four
cubical forms (`EndpointEvaluationHypothesis`, `EndpointMethodHypothesis`,
`EndpointPathElim`, `EndpointElimNeutral`) currently exist only inside that
private context. This adjudication fixes the *generic* rule those forms
instantiate.

## Rule: `endpoint-premise-ledger-api-rule-v1`

1. **Diagram-determined signature.** For every registered declared-boundary
   bundle (under the adopted v2/v3 boundary axioms), the diagram determines
   a canonical elimination-premise signature: a typed motive over the
   owner; exactly one endpoint evaluation per declared boundary binding;
   method(s) of the diagram's PathP/face shape, with one restriction
   equation per face and one coherence obligation per registered pairwise
   overlap. Nothing more (minimality) and nothing less (sufficiency): the
   ledger is a function of the diagram, never of the caller.
2. **Sequent discipline.** Endpoint-aware judgments hold under exactly this
   ledger. The two-checker separation is retained: ordinary context-free
   inference must reject every premise reference; only the endpoint-aware
   checker consumes the ledger, and only against the exact replayed bundle.
3. **Provenance.** Every premise instance must be derived from the replayed
   bundle token by exact context lookups. A caller-supplied premise never
   types — the anti-forgery pattern already validated twice (TRUNC-ER
   endpoints; the v2 `MapCube` derived faces) becomes the API's law.
4. **Charging.** Premises are hypotheses: zero κ, zero ν, no anchors. A
   premise can never mint, seed, or anchor a family. This extends the
   adopted charging convention to the elimination interface — charge lives
   on clauses, never on glue, and never on hypotheses.
5. **Publication.** The four private forms become public exactly as
   instances of this signature, extending the public cubical induction
   from 8 to 12 constructors. The Trunc bundle is the mandatory regression
   instance: the generic API applied to the registered Trunc diagram must
   reproduce the TRUNC-ER tokens (byte-compatibly, or as certified
   equivalents with both directions replayed).

**Grounds (non-numeric, non-verdict).** (i) The signature is the standard
eliminator discipline for cells with declared boundaries: given the adopted
boundary axioms, the elimination data is *determined* by the diagram — this
rule registers that determination rather than inventing content. (ii) A
proven instance exists and is sealed (TRUNC-ER, replayed in H15 and exact
B5). (iii) Clause 4 is forced by the same double-charge asymmetry that
grounded R1: hypotheses are the interface through which content is
consumed, not content. (iv) No clause of this rule references any count,
verdict, bar value, or pending membership case.

## Firewall: verdict-blindness for future generators

The M1 sweep names what would flip the pending left-unit verdict — a typed
unit-coherence generator. This API is deliberately **neutral** with respect
to that question: it supplies typing infrastructure, not generators, and
adopting it changes no membership verdict. Binding requirement on all
successor grammar work: any proposed new generator or constructor must be
justified from the sealed trace and the frozen open-problem specification
alone, with an explicit statement that no pending membership outcome,
count, or sweep result was grounds. A generator whose recorded
justification cites a desired verdict is invalid on its face (the F-E4
rule extended from counts to verdicts).

## Falsifiers

- **F-EP1.** A registered bundle whose diagram-determined signature fails
  to type its own sealed eliminator → the interpretation is revised in a
  versioned successor (F-B1 pattern); the checker is never loosened.
- **F-EP2.** Any route by which a caller-supplied premise yields a token →
  forgery hole; the API implementation is invalid regardless of test
  status.
- **F-EP3.** A premise ledger found to influence any κ, ν, anchor, or
  count → clause-4 violation; certificate invalid.
- **F-EP4.** The generic API fails to reproduce the TRUNC-ER regression
  instance → the generalization is wrong; revise the rule, never the
  sealed instance.

## Consequences upon adoption

The public cubical induction extends to all 12 forms; the named E-3 gap
`E3_REGISTERED_BOUNDARY_BUNDLE_PROVENANCE_TRANSPORT_UNDER_SUBSTITUTION`
becomes addressable (bundle transport under E-1 substitution requires the
public signature); endpoint-dependent constructors in the grammar-completion
work (Map/Axiomatic remainder, and any adjudicated unit-coherence
constructor) acquire their typing discipline in advance, so that grammar
decisions and typing infrastructure stay separate concerns.

## ADOPTION

**Adopted.** Recorded verbatim from the user's adoption message of
2026-07-20 (replay pins this text):

> I adopt `endpoint-premise-ledger-api-rule-v1`, including its
> diagram-determined signature, sequent discipline, provenance law,
> zero-charge clause, and the verdict-blindness firewall for future
> generators, as the public form of the endpoint-premise context.
>
> — Halvor Lande, 20 July 2026

Amendment goes through a versioned successor; no silent modification.
