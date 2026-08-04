# KW1/KW2 Extension-Authority Protocol

Date: 2026-08-04

Status: DRAFT-NOT-FROZEN (formulation lane, plan binding decision 7)

This document mints no authority; it freezes only at its registered gate —
adversarial review after the KW0 task-language freeze and no later than K1,
and in every case before any profile sees the registered prefix (plan
sections 5.6 and 8.4). Until that gate closes, every construct named here is
formulation-lane text and every value it would certify is `Unknown`.

Plan references (`docs/260804_autonomous_plan_and_status.md`): sections 3.4,
3.5, 5.6, 7.4, 8.2-8.4, 13.1; binding decisions 3, 5, 7, 10.

## 1. Scope

KW1 specifies the six opaque capabilities through which all task-extension
facts are issued. KW2 specifies the generic falsifier suite that must pass
before those capabilities carry weight in any gate. Everything here is
target-neutral: no task, fixture, or falsifier may mention expected
structures, physical labels, expected counts, or any historical continuation.
All fixtures are synthetic and derived from the frozen KW0 manifest only.

Shared discipline for every capability (plan 5.2):

- exactly one private deterministic constructor, reachable only through the
  verifying computation it names;
- no digest, Boolean, tag, count, caller assertion, or matching type name is
  authority; and
- `Unknown` is the outcome whenever evidence is incomplete or resources
  fail; it is never permission to guess and never a silent negative.

## 2. KW1 opaque authority capabilities

### 2.1 VerifiedTaskCarrier

Certifies: the complete finite raw task carrier generated from the frozen
`TaskManifest` (plan 3.3), including the enumeration-completeness fact that
every task expressible under the manifest closure rule is present exactly as
the rule dictates.

Constructor discipline: minted only by the internal enumerator that generates
the carrier from the manifest. A caller-supplied task list, task count,
digest, or "complete" flag cannot mint it (plan 5.6).

Fail-closed: if enumeration exhausts its resource contract or any manifest
check fails, no capability value exists; every downstream consumer observes
`Unknown`. There is no partial carrier authority.

### 2.2 VerifiedTaskQuotient

Certifies: the presentation/equivalence quotient `Tasks_V` over the verified
raw carrier, with the six frozen invariances of plan 3.3 (opaque identifier
permutation, reordering, aliases and definitional extensions, duplicated
presentations, adopted presentation equivalence, transport between proven
equivalent vocabularies).

Constructor discipline: minted only by the quotient decision procedure run
over a capability-bearing `VerifiedTaskCarrier`. Caller-supplied class
representatives, class counts, or equivalence tables cannot mint it.

Fail-closed: an undecidable equivalence pair yields `Unknown` for the whole
quotient; no class-partial quotient is issued.

### 2.3 VerifiedTaskModelRelation

Certifies: for one `(tau, response)` pair, the verified judgment that the
response is a correct disposition under the frozen task contract — a
verified construction, a verified negative, or another outcome the contract
explicitly declares correct (plan 3.4).

Constructor discipline: minted only by checking the response against the
frozen correctness relation. A caller-asserted success bit, score, or typed
wrapper is not authority.

Fail-closed: an unverifiable pair yields an `Unknown` row, never a default
success or default miss.

### 2.4 VerifiedCandidateExtensionSet

Certifies: for a candidate `x` in `D_H`, the complete disposition vector of
`TaskExt_H(x)` over the frozen local carrier `Tasks(H)`: exactly one row per
task, each row verified success, verified non-success, or `Unknown`.

Constructor discipline: minted only by total per-row evaluation of the
counterfactual sealing of `x` against every task, each row passing through
`VerifiedTaskModelRelation`. A caller-supplied success vector, extension
count, or row subset cannot mint it.

Fail-closed: a resource-exhausted row is recorded as `Unknown`; a missing or
duplicated row aborts the mint. Cardinality is derived, never supplied.

### 2.5 VerifiedProfileExtensionSet

Certifies: the same complete disposition vector for a whole profile `P` over
the profile-level carrier named and frozen at KW0 (`TaskExt_E(P)`), under the
branch-aggregation rule registered in the 2x4 preregistration (plan 8.4 KW4).

Constructor discipline and fail-closed behavior: as 2.4, at profile level. No
caller-supplied vector, count, or aggregation shortcut mints it.

### 2.6 VerifiedExtensionDominance

Certifies: one pairwise dominance verdict between two capability-bearing
extension sets: strict subset, equal, incomparable, or `Unknown`, under the
three-valued semantics of section 3 below.

Constructor discipline: minted only from two verified extension sets over
the same verified carrier and quotient. Caller-supplied verdicts,
cardinalities, or set differences cannot mint it.

Fail-closed: any `Unknown`, resource-exhausted, or outside-fragment row on
either side forces the verdict `Unknown`. An `Unknown` verdict never rejects
any candidate or profile.

## 3. Three-valued dominance semantics

Frozen restatement of plan 3.4 as amended:

- A strict-subset rejection is licensed only when every task row of both
  disposition vectors is decided — verified success or verified non-success
  under the frozen task contract.
- Any `Unknown`, resource-exhausted, or outside-fragment row on either side
  makes that pairwise comparison `Unknown`, and the candidate or profile
  survives. Treating an `Unknown` row as a miss would let resource noise act
  as a verified negative, violating plan 5.2.
- Equal and incomparable maximal extensions all survive; no scalar
  averaging, cardinality tie-break, enumeration order, or hash manufactures
  an ordering. Cardinality and weighted priors are diagnostics only.

## 4. The derivation function `H -> Tasks(H)`

Per plan 3.5 as amended and KW3, the object frozen is the derivation
function itself, not any one carrier instance:

- `H -> Tasks(H)` is a frozen component of the shared substrate, identical
  for all eight profiles, reading only the current public grammar and the
  frozen task manifest — never actual future branches (plan 5.6).
- The per-act evaluation of the `|Tasks(H)| x |D_H|` counterfactual sealings
  carries its own registered resource contract.
- If that contract is exhausted, the `S_W` gate returns `Unknown` for the
  act — a gate-level outcome, never a partial rejection — and section 3
  applies to every pairwise comparison touched by the act.

## 5. Extension-completeness theorem (binding decision 3)

Extension-maximal discharge may receive authority only after this theorem is
frozen and verified. The concrete obligations, proved not asserted:

- ECT-1 (row totality): the `TaskExt` computation is total over the frozen
  carrier: every `tau` in `Tasks_V` (respectively `Tasks(H)`) receives
  exactly one disposition — verified success, verified non-success, or
  `Unknown` — with no missing, duplicated, or multiply assigned row.
- ECT-2 (carrier completeness): the enumeration behind `VerifiedTaskCarrier`
  is complete relative to the frozen manifest closure rule, established by an
  internal enumeration theorem; a caller list or count proves nothing.
- ECT-3 (quotient stability): dispositions are constant on quotient classes;
  duplicating a presentation cannot enlarge any extension set.
- ECT-4 (generic proof, not replay): the theorem is a generic mechanized
  proof over the manifest. Fixed-resource replay proves only successful
  instances (plan 5.3) and cannot substitute for ECT-1 through ECT-3.

## 6. KW2 falsifiers

Each falsifier is a concrete executable test over synthetic target-neutral
fixtures. Failure of any falsifier blocks the KW2 freeze. Format: setup;
expected verdict; what failure proves.

1. Equal-length hypotheses, different extensions. Setup: two valid synthetic
   hypotheses with byte-equal encoded length and fully decided, unequal
   extension sets. Expected: the dominance verdict distinguishes them.
   Failure proves the comparator reads syntactic size, not extension.
2. Shorter but strictly more specific. Setup: hypothesis `A` shorter than
   `B`; `TaskExt(A)` a proper subset of `TaskExt(B)`, both fully decided.
   Expected: `A` is strict-subset dominated. Failure proves brevity leaks
   into weakness adjudication (lesson 18).
3. Longer universal-property hypothesis with larger extension. Setup: a
   costlier, longer hypothesis whose decided extension strictly contains a
   cheap specific one. Expected: the longer hypothesis dominates. Failure
   proves cost is acting as a generalization proxy.
4. Broad invalid hypothesis. Setup: a hypothesis failing a `Valid_E` check
   but with a large would-be extension. Expected: excluded before any
   extension comparison (plan 3.2). Failure proves validity does not precede
   weakness (lesson 19).
5. Alias, duplication, presentation invariance. Setup: one task family under
   identifier permutation, reordering, aliases, and duplicated
   presentations. Expected: extension sets and every verdict unchanged.
   Failure proves a quotient leak.
6. Vocabulary-equivalence transport. Setup: two independently proven
   equivalent vocabularies; tasks transported. Expected: transported
   extensions and identical verdicts. Failure proves the quotient is not
   transport-invariant.
7. Incomparable cone retention. Setup: two candidates with fully decided
   incomparable extensions. Expected: both survive; no scalar tie-break.
   Failure proves a manufactured ordering.
8. Prior-ranking reversal. Setup: a frozen weighted prior under which the
   cardinality ranking reverses against the uniform diagnostic. Expected:
   reported as `WeaknessAdjudicationUnderdetermined`, never hidden. Failure
   proves diagnostic suppression.
9. (Deferred: JG9.) Certified `gamma > 0` with no task-extension gain.
   Setup: a candidate with certified positive marginal and unchanged
   extension. Expected: the extension gate is unaffected by `gamma`. Failure
   proves illegal `gamma`/extension coupling (plan 3.6).
10. (Deferred: JG9.) Task-extension gain with certified `gamma = 0`. Setup:
    the converse coupling probe. Expected: extension gain registered with no
    `gamma` inference. Failure proves the same coupling in reverse.
11. No future-branch access. Setup: a fixture exposing a decoy future-trace
    channel to the derivation function and gate. Expected: the channel is
    never read; any read is a fail-closed abort. Failure proves an oracle
    breach (plan 5.6).
12. Resource exhaustion yields `Unknown`. Setup: a task evaluation exceeding
    the per-act contract. Expected: the row is `Unknown`, the gate returns
    `Unknown` for the act, and the candidate survives. Failure proves
    resource noise mints verdicts.
13. Vocabulary sensitivity detection. Setup: two inequivalent admissible
    frozen vocabularies under which the `S_W` survivor set differs.
    Expected: reported as vocabulary sensitivity
    (`VocabularyAdjudicationUnderdetermined`), not absorbed. Failure proves
    sensitivity hiding.
14. Unknown-as-miss gate audit. Setup: a mutant gate that treats an
    `Unknown` row as a miss inside a subset comparison and rejects a
    candidate. Expected: the gate audit rejects the mutant; the candidate
    survives under the frozen gate. Failure proves the three-valued
    semantics of section 3 are not enforced.

Deferral record: falsifiers 9 and 10 consume certified `gamma` and therefore
execute only after JG9 issues it (plan 8.4 as amended). At KW2 freeze time
they are registered as specified-but-deferred with outcome
`ExecutionDeferred(JG9)`; their later failure reopens KW2 as a new version.
All other falsifiers are gamma-independent and execute at KW2 freeze time.

## 7. Freeze gates

- G1: the six KW1 capabilities are constructed, constructor-mutation-tested,
  and frozen before any profile sees the registered prefix (plan 5.6). No
  extension fact reaches any gate earlier.
- G2: every gamma-independent falsifier (1-8, 11-14) passes at KW2 freeze
  time, before Genesis exposure; falsifiers 9-10 are recorded as deferred.
- G3: this document itself freezes only after adversarial review (binding
  decision 7), strictly after the KW0 freeze it quantifies over, and no
  later than K1.
- G4: commit-before-freeze (binding decision 5): every freeze above is
  recorded only against a committed immutable tree hash.

## 8. Open registrations

- OPEN-1: correspondence depth for KW1 authority — full Rust/safe-Agda
  byte-identical transcript agreement (JG8-style) versus safe-Agda generic
  proof with fixed-resource Rust replay of finite instances. Adjudication:
  decided at the KW1 freeze by adversarial review, argued from plan 5.2/5.3;
  until decided, any capability value feeding a K2 gate is `Unknown`.
- OPEN-2: wire representation of `outside-fragment` — a distinct disposition
  tag versus a task-contract-declared outcome per task. Adjudication: fixed
  by the KW0 task-contract grammar at the KW0 freeze; section 3 is invariant
  to the choice, since either representation blocks strict-subset rejection.
- OPEN-3: the forged-capability mutant suite composition. Adjudication:
  enumerated before the KW1 freeze under lesson 13, with at least one mutant
  per capability per forbidden minting path (caller list, caller vector,
  caller count, caller verdict), each rejected only at the KW1 layer.

No open item may be resolved silently; each resolution is a versioned edit
to this document before its freeze gate.
