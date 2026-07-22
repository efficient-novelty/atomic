# Classifier successor adjudication: inductive telescope internality

**Date:** 2026-07-20. **Status:** **ADOPTED** 2026-07-20 (see ADOPTION
block). Versioned successor to
`internal-derivability-classifier-branch-v1`, prepared from
`docs/SCHEMA2_INTERNAL_CLASSIFIER_BRANCH_RESULT.md`.

## Problem exposed by the witness-first run

The first Internal branch required each clause to derive directly over B15
and therefore rejected every candidate-local field reference. That condition
was sufficient to prevent proof-free relabelling, but it was too strict for a
dependent telescope: a later clause may lawfully use an earlier declaration
whose own Internal proof has already established that it adds no marginal
content.

For the exact witness, the frozen absolute-level checker proves:

```text
clause 0: Univ        -> univ-form
clause 1: Lam(Var 1) -> lam-intro(field-ref-0)
```

`Lam(Var(1))` is therefore not the identity in this two-clause scope. It is a
constant projection of the already introduced arena field. The actual closed
identity is `Lam(Var(2))`, whose leaf rule is `local-var-1`. The successor
must preserve this distinction.

## Rule: `inductive-telescope-internality-v1`

1. **Ordered judgment.** For a telescope `c_0, ..., c_n`, Internal
   derivability is checked from left to right. At position `i`, the admissible
   context is B15 extended only by earlier clauses `c_j` (`j < i`) that have
   already earned replayed Internal certificates in this same run.
2. **Dependency gate.** A `field-ref-j` leaf may be used in the proof of
   `c_i` only when `j < i` and clause `j` is present in the ordered certified
   Internal prefix. Forward, missing, cyclic, or merely well-typed field
   references do not pass.
3. **Projection/identity separation.** `lam-intro(field-ref-j)` is recorded
   as an inductive constant projection through certified field `j`.
   `lam-intro(local-var-1)` is recorded separately as structural identity.
   Neither proof may be reported under the other's name.
4. **Transitive provenance.** The certificate for a later clause retains the
   derivation hashes of every certified earlier clause it uses. Replay rebuilds
   the prefix and rechecks the dependencies; a boolean claim or an unbound
   field number is not evidence.
5. **Zero credit.** A telescope enters `Internal` only if every clause passes
   the ordered judgment. It then inherits `nu = 0`; the rule creates no new
   valuation clause.
6. **Guards remain mandatory.** This successor changes only candidate-local
   telescope dependencies. Ambient weakening/erasure obligations remain
   exactly as in the predecessor rule. An open/guarded clause without both
   inverse laws remains unproved and therefore cannot enter `Internal`.
7. **Classifier order remains frozen.** Surface/typed exclusion ->
   certificate-backed Internal -> adopted class -> named F-G4 obstruction.
   Unknown remains a live output.

## Falsifiers

- **F-IT1.** A field dependency is accepted before its target clause has an
  Internal certificate -> successor unsound; report verbatim.
- **F-IT2.** A forward, missing, or cyclic dependency replays -> ordered
  prefix checker unsound; report verbatim.
- **F-IT3.** `Lam(Var(1))` is labelled identity, or `Lam(Var(2))` is labelled
  projection, under the frozen witness scope -> binding/provenance mismatch;
  report verbatim.
- **F-IT4.** The predecessor F-I2 artifact is overwritten or reinterpreted ->
  invalid versioning. It remains a correct result under its stricter rule;
  this rule issues a successor disposition.
- **F-IT5.** A guarded term enters Internal without replayed weakening and
  erasure inverse laws -> invalid on its face.
- **F-IT6.** A future exact raw candidate remains Unknown -> F-G4 fires and
  global E-4 remains incomplete; this rule may not hide the survivor.

## Consequence upon adoption

Run the exact witness first. If clause 0 earns the arena-reference proof and
clause 1's only field dependency is certified clause 0, issue a successor
Internal certificate naming clause 1 a constant projection. Recheck
`Lam(Var(2))` independently as identity. Then rerun global E-4 from a
create-new successor artifact. Downstream membership and count work remains
conditional on the absence of every Unknown, not merely resolution of this
one witness.

## ADOPTION

**Adopted.** Recorded verbatim from the user's message of 2026-07-20:

> Yes, do the versioned adjudication for inductive telescope internality:
> permit a later clause to depend on an earlier clause only after that
> earlier clause has itself earned Internal. That could classify Lam(Var(1))
> honestly as a constant projection from the internally certified arena—not
> as identity. Lam(Var(2)) should remain the separately verified identity.
>
> — Halvor Lande, 20 July 2026

The amendment is implemented only by create-new successor artifacts; no
predecessor source or certificate is silently modified.

