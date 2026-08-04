# Holdout decoder registration, inventory, and burn log

Date: 2026-08-04

Status: DRAFT-NOT-FROZEN (formulation lane, plan binding decision 7)

This document mints no authority: it is formulation-lane text under plan
§13.1, and it becomes binding only at its registered freeze gate — hash-sealed
freeze after adversarial review, no later than K1 (plan §6.4, §8.4 K1). After
that freeze, any decoder change is a new experiment under K3.

Governing artifacts: `docs/260804_autonomous_plan_and_status.md` (§3.7, §5.6,
§8.4 KW5/K3), `docs/BLINDNESS_CONTRACT.md` (§4, §7),
`docs/autonomous_genesis_plan.md` (the external-decoder equivalence-theorem
separation), and `docs/260730_contextual_internalization_frontier.md` (§4).

## 1. Decoder protocol

This protocol extends `BLINDNESS_CONTRACT.md` §7. All four of its conditions
are retained unchanged: the run and all certificates are frozen first; the
blindness manifest verifies; the decoder executes out of process or outside
the lawful dependency closure; and any semantic realization required by a
label has independently passed. The additions below bind KW5.

### 1.1 Match relation

The decoder's match relation is a **verified typed interpretation or
equivalence up to the frozen presentation quotient**. A decoded
identification is a proof artifact: a checked typed translation between the
sealed anonymous output and a candidate external structure, invariant under
the same presentation quotient the run itself adopted. Name resemblance,
syntactic similarity, operator-count coincidence, or shape familiarity is
never a match. This is the equivalence-theorem separation already registered
in `autonomous_genesis_plan.md`: the engine does not know what it discovered;
the decoder must prove, not recognize.

### 1.2 Graded outcomes

Every decoder invocation on a sealed output class returns exactly one of:

- `exact` — a complete verified interpretation/equivalence with the inventory
  item, up to the frozen presentation quotient;
- `partial` — a verified interpretation of a proper typed fragment, with the
  uninterpreted remainder explicitly enumerated; or
- `none` — no verified interpretation within the decoder's frozen resource
  contract. Resource exhaustion during the attempt is reported as
  `none(ResourceExhausted)`, never as evidence of absence.

`Unknown`-style fail-closed semantics apply: an outcome that cannot be
verified is reported at the weaker grade, never promoted.

### 1.3 Decoder inputs and blinding

The decoder receives only:

1. the anonymous sealed run output (post-KW4, per plan §8.4); and
2. the hash-sealed holdout inventory and burn log of this document.

It receives no run internals beyond the sealed artifacts, no formulation
notes, and no unsealed candidate list. The decoder may not be amended,
re-parameterized, or re-run with altered inputs after observing an outcome;
any such change is a new experiment under K3.

### 1.4 OPEN — partial-grade boundary

The exact grammar separating `partial` from `none` (minimum interpreted
fragment, whether the fragment must include the class's typed public
interface) is underdetermined. Adjudication: the freeze-time adversarial
review fixes a decidable fragment criterion before sealing; until then no
partial outcome may be claimed.

## 2. Interpolation versus new prediction

A decoded identification is classified by provenance, decided from the
sealed inventory and burn log alone:

- **Interpolation** — the identified structure or relation appears in the
  development corpus or in the burn log. Interpolation is reported as
  consistency evidence only; it supports no novelty claim.
- **Genuinely novel prediction** — the identified structure or relation was
  in the sealed holdout inventory with status `held-out` at freeze and was
  never used in formulation (no burn-log entry at any date).

Reversals and ambiguity are reported, never adjudicated post hoc. If an item
plausibly falls in both categories, or its burn status is disputed after
outcomes are observed, the report records the dispute and both readings; no
post-outcome reclassification may upgrade an interpolation to a prediction.

## 3. Holdout inventory (DRAFT skeleton)

The concrete item list is deliberately deferred to the freeze. This section
fixes only the schema and the categories, so that no item can be added,
dropped, or re-described after outcomes are observed.

### 3.1 Item schema

Every inventory row carries exactly:

```text
item         opaque description sufficient for the decoder's proof obligation
source       where the item's external formulation is recorded
seal date    date the row entered the sealed inventory
status       held-out | burned | contestable
burn reason  required iff status = burned; cites the formulation use and date
```

A row with status `contestable` must also record both positions and the
registered adjudication procedure.

### 3.2 Categories

1. **Held-out mathematical structures.** External mathematical structures and
   their defining universal properties, reserved for post-sealing
   identification. No item is named in this draft; naming an expected
   structure here before the freeze would itself burn it as a formulation
   input.
2. **Held-out physical structures and relations.** External physical
   structures, relations, and constraints reserved for the KW5 held-out
   physical evaluation. Same deferral discipline as category 1.
3. **The registered C1 internalization prediction.** The held-out prediction
   registered in `docs/260730_contextual_internalization_frontier.md` §4 is
   an inventory row with status `contestable`. Its holdout status is
   genuinely disputed:
   - Position A (held-out): the prediction was registered before any profile
     was formulated or run, is barred from demand compilation, generation,
     acceptance, quotienting, and halting, and therefore qualifies as a
     sealed prediction in the XF-campaign sense.
   - Position B (burned-in-part): the anticipated identification informed
     C1's motivation; the frontier document derives the profile's demand
     shape from the same universal-property family the prediction names, so
     a later match partly confirms the formulation's own input, i.e. is at
     best a constrained interpolation.
   Both positions are recorded now; neither is adopted. Adjudication: the
   freeze-time adversarial review either assigns a single status with written
   rationale or freezes the row as `contestable`, in which case any decoder
   match on it is reported under both readings and claims no novelty.

### 3.3 OPEN — inventory sealing granularity

Whether categories 1 and 2 are sealed as one inventory hash or per-category
hashes is underdetermined. Adjudication: decided at freeze by the reviewer;
the decision affects only audit granularity, not admissibility.

## 4. Burn log (DRAFT initial entries)

A fact used to formulate or repair a law profile ceases to be held out (plan
§3.7). The burn log is append-only; each future entry is dated and
irreversible. A burned fact is never restored; successor experiments must
reserve fresh evidence (K3).

Initial entries, effective at this document's date:

1. **The archived Genesis development corpus.** Every fact of the archived
   fifteen-step Genesis sequence — its stage identities, accepted history,
   and ledger — is development evidence, not holdout. It was the regression
   corpus for the entire programme (plan §5.1). Burn reason: development
   corpus; predates all profiles. Any decoder identification with a member of
   this corpus is interpolation by definition.
2. **All facts cited in law-profile formulation to date.** Every
   mathematical or physical fact cited in the formulation, motivation, or
   repair of `C0`, `C1`, `S0`, `S_W`, `S+`, or `S_up`, or of the shared
   substrate, up to and including this document's date, is burned. The
   itemized citation census is compiled at freeze from the formulation-lane
   documents and their references; a fact whose citation status is unclear is
   recorded `contestable`, not silently retained as holdout.
3. Future entries: one dated row per fact, added at the moment of
   formulation use, before any run consumes the affected profile.

## 5. Hash-sealing rule

At freeze — no later than K1 — the completed inventory and the burn log as
then constituted are hash-sealed, and the seal digests are recorded in the
K1 freeze record and the experiment manifest. From that point:

- the inventory is immutable; the burn log admits only dated appends, and
  each append is re-sealed;
- the decoder protocol of §1, including the frozen partial-grade grammar, is
  immutable; and
- any post-hoc decoder change — match relation, grading, inputs, inventory
  membership, or reclassification — is a new experiment under K3 with a new
  identity and fresh held-out evidence. The old decoder's outcomes stand
  unamended.

## 6. What this document does not do

It names no expected structure, count, or continuation as an input; it
adjudicates no contestable status before the freeze review; it grants the
decoder no authority over run acceptance, quotienting, selection, or halting;
and `Unknown` or `none` outcomes are results to report, never permission to
guess or to rerun until a match appears.
