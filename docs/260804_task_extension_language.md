# KW0 Task-Extension Language: Vocabulary-Selection Procedure and Manifest Skeleton

Date: 2026-08-04

Status: DRAFT-NOT-FROZEN (formulation lane, plan binding decision 7)

This document mints no authority of any kind; it freezes only at its
registered gate, KW0 (plan §8.4), after adversarial review and before K1.
Until that freeze, every clause below may change freely; after it, changes
require a new version under plan §5.5. The governing plan is
`docs/260804_autonomous_plan_and_status.md` (rev 2), specifically §§1
(binding decisions 5-10), 3.3-3.5, 5.6, 5.7, 7.4, 8.2-8.4, and 13.1.

## 1. Scope

KW0 freezes the finite target-neutral task language over which candidate-act
extension (`TaskExt_H`, plan §3.5) and profile extension (`TaskExt_E`, plan
§3.4) are computed. This document registers, before that freeze:

1. the vocabulary-selection procedure;
2. the KW0-v1 minimality commitment;
3. the vocabulary sensitivity axis and its underdetermination outcome;
4. the `TaskManifest` field skeleton and the task-quotient invariance
   contract;
5. the profile-level task carrier naming rule; and
6. the review and freeze discipline.

Fail-closed discipline applies throughout: wherever a required decision or
check cannot be completed, the outcome is `Unknown`. `Unknown` is an outcome,
never permission to guess (plan §5.2).

## 2. Vocabulary-selection procedure

The task vocabulary is mechanically generated from the frozen public
grammar/bootstrap fragment by a stated closure rule. The generator's inputs
are exactly the sealed public grammar and bootstrap fragment as committed at
the KW0 freeze; no other source may contribute symbols, sorts, or forms.

### 2.1 PROPOSED closure rule CR-1

CR-1 (PROPOSED, not adopted): the task vocabulary is the set of all typed
judgment and realization task forms expressible from the sealed public
grammar's sorts, constructors, and judgment forms, closed under:

- application of each public constructor to well-typed argument slots drawn
  from the same closure;
- formation of each public judgment form over subjects in the closure; and
- the realization form "produce a verified inhabitant, a verified
  certified-negative, or `Unknown` for judgment J", for each judgment J in
  the closure;

up to a stated rank bound `r` on constructor-nesting depth. The closure is
finite because the sealed grammar is finite and `r` is finite.

OPEN-KW0-1: the value of `r` and whether rank is measured by
constructor-nesting depth alone or by a joint (depth, size) bound.
Adjudication procedure: before the KW0 freeze, enumerate candidate bounds,
compute the exact carrier cardinality and the KW0 resource contract's
worst-case evaluation cost for each, and select in a sealed memo the smallest
bound whose carrier is sufficient for clause 3.1 below; if no argued
selection survives adversarial review, KW0 does not freeze.

### 2.2 Hand additions

Any hand-added task form outside the mechanical closure requires an
individually justified sealed memo, written and committed before any profile
run, stating the form, the reason mechanical generation missed it, and an
argument that it is target-neutral under §7. An addition without such a
pre-run memo is invalid and its tasks are excluded from every extension set.

### 2.3 What the procedure forbids

The generator may not consult: historical Genesis output, any expected or
decoded structure, any profile's behavior, any candidate discharger, any
holdout inventory, or any extension result. Choosing the closure rule or
rank bound to make a particular profile or candidate extension-maximal is a
registered violation and voids the freeze.

## 3. KW0-v1 minimality

### 3.1 Binding decision 10 restated

KW0-v1 is deliberately minimal: the first frozen task vocabulary is the
minimal language sufficient to express discharger-realization tasks over the
current public grammar. Sufficiency means: for every constitutively
admissible total discharger expressible in the declared fragment, the
vocabulary can express the realization tasks whose verified correct
realization by free sealing defines `TaskExt_H` (plan §3.5).

### 3.2 Successors

Successor vocabularies are new experiments under the K3 fresh-holdout rule
(plan §8.4 K3). No vocabulary enlargement may be applied retroactively to a
sealed run, and no failed vocabulary is edited in place.

## 4. Vocabulary sensitivity axis

At least two independently motivated admissible vocabularies are
preregistered before the freeze, as a sensitivity axis parallel to the
weighted priors of plan §3.4:

- `V1`: the CR-1 closure at the adopted rank bound (the KW0-v1 primary);
- `V2`: a second admissible vocabulary with an independent motivation
  recorded in its own sealed memo.

OPEN-KW0-2: the concrete definition of `V2`. Candidate constructions, each
independently motivated: (a) CR-1 at a strictly larger rank bound; (b) a
judgment-forms-only restriction of CR-1 (realization tasks over judgment
forms, without nested constructor-application task forms); (c) a closure
generated from the bootstrap fragment alone rather than the full public
grammar. Adjudication procedure: a sealed memo before the KW0 freeze must
argue exactly one choice from motivations that do not reference any profile,
candidate, or extension result; if the memos cannot discriminate, register
the tie and freeze the pair whose two members are provably inequivalent
(distinct task quotients), since an equivalent pair provides no sensitivity.

Both vocabularies receive full `TaskManifest` instantiations and quotients.
If extension-maximality (act-level `S_W` survivor sets or profile-level
maximal sets) reverses between the two frozen vocabularies, the result is
reported as `VocabularyAdjudicationUnderdetermined`, never hidden, never
averaged, and never resolved by picking the vocabulary that yields a winner.
This mirrors `WeaknessAdjudicationUnderdetermined` (plan §3.4) and KW2
falsifier 13.

## 5. TaskManifest skeleton

The frozen manifest instantiates every field of plan §3.3. Placeholders
below are the KW0 freeze obligations; a manifest with any placeholder
unresolved cannot freeze.

| Field | KW0-v1 placeholder (to be fixed at freeze) |
| --- | --- |
| vocabulary | `V1` per §2 (with `V2` as the preregistered sensitivity pair) |
| state/situation grammar | TBD: guarded-history public-grammar states expressible in the vocabulary |
| decision/realization grammar | TBD: realization forms of §2.1 clause 3 |
| correctness relation | TBD: verified construction, verified certified-negative, or a task-contract-declared correct outcome; `Unknown`, resource exhaustion, and outside-fragment never silently count as success (plan §3.4) |
| child/parent relation | TBD: derivation function `H -> Tasks(H)` component, frozen at KW3 as a function, instantiated here as a relation schema |
| task rank and depth | TBD: the adopted CR-1 bound `r` (OPEN-KW0-1) |
| normalization | TBD: the shared substrate's frozen normalization on task presentations |
| presentation/equivalence quotient | TBD: the quotient satisfying the six-clause contract of §5.1 |
| resource contract | TBD: per-task and per-act bounds; exhaustion yields `Unknown` at the gate (plan §3.5) |

`Tasks_V` is the complete finite quotient generated by the manifest.

### 5.1 Task-quotient invariance contract

The task quotient must be invariant under:

- opaque identifier permutation;
- declaration and enumeration reordering;
- transparent aliases and definitional extensions;
- duplicated syntactic presentations of one task;
- adopted presentation equivalence; and
- transport between independently proven equivalent vocabularies.

These six clauses are copied from plan §3.3 and are not renegotiable at the
KW0 freeze; a manifest whose quotient fails any clause cannot freeze, and a
post-freeze counterexample invalidates KW (plan §8.5: if the extension
ranking changes under equivalent task presentations, KW is invalid).

## 6. Profile-level task carrier

KW0 names and freezes the profile-level task carrier that KW4 compares. The
default is `Tasks_V` itself, and this document registers that default as the
KW0-v1 rule. Any other choice must be stated and argued in the frozen KW0
text before K1; absent such an argument, `Tasks_V` is the carrier. The
branch-aggregation rule for `TaskExt_E(P)` over a sealed cone is not decided
here; it is registered in the 2x4 preregistration before K2 (plan §8.4 KW4).

## 7. Negative constraints (plan §5.6 restated)

- No task grammar, vocabulary, situation, or correctness clause may mention
  Genesis stage names, expected mathematical structures, physical labels,
  expected candidate counts, or the historical continuation.
- A task may not encode the expected next Genesis structure, a later
  accepted branch, or a known physical interpretation.
- Duplicating one task presentation may not enlarge extension: duplicated
  presentations are identified by the quotient (§5.1 clause 4).
- A caller-supplied task list or extension count cannot mint completeness;
  the carrier is derived by the frozen generator or it does not exist.
- Uniform-task weakness and weighted-prior weakness are diagnostics over an
  already verified extension set; neither replaces exact extension inclusion.
- `S_W` may not inspect actual future branches; it uses only the frozen
  local parent-task carrier `Tasks(H)`.
- The vocabulary, correctness relation, quotient, priors, and extension
  comparison must all be frozen before any profile sees the registered
  prefix.

Recognizing any sealed anonymous task or realization as familiar mathematics
or physics is a held-out decoding step (plan §2), never an input here.

## 8. Review and freeze discipline

- KW0 freezes only after adversarial review, mirroring K0: a reviewer
  distinct from the author role attacks target-neutrality, closure-rule
  bias, quotient invariance, minimality sufficiency, and the sensitivity
  pair's independence, with written findings resolved or registered before
  freeze.
- Freeze gate: KW0, strictly before K1 (plan §6.4 formulation lane; §8.4).
- Commit-before-freeze (binding decision 5): the frozen KW0 artifact,
  generator, sealed memos, and both vocabulary instantiations are committed
  with immutable tree hashes before any ledger records them as frozen.
- Sub-phase budget (binding decision 6, plan §5.7): obligations discovered
  during KW0 drafting are assigned to KW1/KW2/KW3 unless a short written
  audit shows they block the KW0 freeze itself.
- Open registrations at this draft: OPEN-KW0-1 (rank bound), OPEN-KW0-2
  (second vocabulary). Neither is silently resolved; each closes only by its
  stated adjudication procedure inside the pre-freeze sealed-memo record.
