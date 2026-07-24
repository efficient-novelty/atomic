# XF-B0 — Library Membership Criterion (LMC) v1

**Campaign:** XF-1 (`docs/xf1_external_falsifier_campaign.md`). **Item:**
XF-B0, the build that operationalizes the cone-certified halt. **Date:**
2026-07-24. **Status:** FROZEN BEFORE ANY CANDIDATE EVALUATION (F-XF5).
This document defines the test and evaluates no proposal. The first
evaluation of any specific structure occurs in a versioned successor
artifact (`xfb0_lmc_evaluation_<candidate>_vN.md`) and is published whatever
its verdict (F-XF4). Amendment of this criterion goes through a versioned
successor; no silent modification. Before freezing, this text passed a
three-lens adversarial stress panel; the confirmed findings and their
repairs are recorded in Section 8.

## 0. Foundation and register eligibility (F-XF1)

Everything this criterion consumes is bound in MS-1
(`docs/milestone_certificate_v1.json`, result digest
`blake3:37d96619734afb37125f3cc28fc3cac87b8b0b81c8c0a9945156bdba6c43bb0b`):

- the sealed alphabet **B₁₅** — the fifteen sealed shells of the certified
  Genesis Sequence (`SealedSignature::genesis_del_h15()`, the frozen
  telescope 1..15 verified against the fifteen checkpoint hashes);
- the Phase-1 kernel conventions (level binding, beta-only reduction,
  capture-avoiding substitution, fuel-bounded judgmental equality);
- the adopted closure law: definitional derivability, the semantic-family
  predecessor closure, the adopted A3/ambient-former partition, the EGP
  anchor mechanism table, and the four certified falsifier shapes;
- the certified halt package: semantic O(16) = ∅ at full instance
  granularity, cone-level on all four branches; internal continuations
  score ν = 0 by univalent closure; the act-local provenance ceiling
  ν ≤ 4κ forecloses open-band earning;
- the certified representation fact: the point-field action has exact
  kernel ℤ₆ with faithful image G₀/ℤ₆.

Derivation-basis note: LMC-0/LMC-1 classification uses the sealed alphabet
and adopted closure rules only; the LMC-2 foreclosure conclusion
additionally rests on the certified halt package and provenance ceiling,
both lawful register under F-XF1. No structural-register quantity — the
sealed engine-testimony quantities enumerated in F-XF1, cited here by
rule-id and deliberately not restated — is an input anywhere in this
pipeline; an evaluation found resting on one is void, not weakened.

## 1. What the LMC decides

Input: a proposed structure P — a particle, force, carrier, or sector
proposed as *fundamental* (generic categories only; this document names no
specific model). Output: exactly one of four verdicts:

- `NotFormalizable` — fail-closed abstention; no claim about the world
  (but see LMC-3c: persistent abstention on a *confirmed* structure is a
  tracked adequacy failure, never silent).
- `RecombinationOfB15` — P is recombination/behavior of library content.
- `SixteenthEntryForeclosed` — P would require a sixteenth library entry,
  which the certified halt forecloses; the theory predicts no such
  fundamental structure exists.
- `Burned` — issued only through LMC-3 (Section 5): recorded against the
  prior `SixteenthEntryForeclosed` evaluation in the versioned
  adjudication artifact and mirrored in `docs/XF_LEDGER.md`.

## 2. The dossier (input protocol)

An evaluation begins from a two-part submission, each part separately
digest-pinned:

**Part A — classification dossier** (the only input to LMC-0/1/2):

1. P's proposed fundamental carriers, operations, and laws as a typed API
   (clauses in the sealed grammar's terms);
2. a declared translation of that API over B₁₅'s exported interfaces, or a
   declaration that no translation is claimed;
3. the physical claim: fundamental sector, or composite/effective content.

**Part B — sealed empirical annex** (consumed only by LMC-3):

4. the empirical identification: which measured behavior — observables,
   quantum numbers, coupling structure — the API is answerable to.

The classification verdict must be a replayable function of Part A alone;
the Part-B digest is recorded in the evaluation artifact but its content
is quarantined from every classification step. The dossier is authored
blind to the pipeline's verdict machinery: it may not be tuned against
trial runs of the classifier (F-XF2 applied to membership).

## 3. The decision pipeline (each step replayable, published)

- **LMC-0, formalizability gate.** If Part A cannot be presented in the
  sealed grammar at all — its clauses do not parse or type as terms of the
  grammar — verdict `NotFormalizable`. Fail-closed: the LMC does not
  adjudicate what it cannot type, and abstention is never evidence for or
  against the theory.
- **LMC-1, recombination test.** Elaborate each Part-A clause against
  B₁₅'s exported interfaces; compute each clause's canonical semantic
  family and classify it against the adopted predecessor closure:
  **internal** (inside the closure), **anchored-marginal** (decided by
  exact lookup in the pinned EGP mechanism table, nothing more), or
  **new-primitive** — exactly one of the four certified falsifier shapes
  (stuck fresh head; missing formation clause; untyped lift; undominated
  import), with no generalization: a clause that fails to classify as
  internal, anchored-marginal, or one of the four certified shapes falls
  the whole evaluation to `NotFormalizable` (fail-closed, tracked). If
  every clause is internal or anchored-marginal, verdict
  `RecombinationOfB15`: P is behavior of library content; its experimental
  discovery would not be a new fundamental sector and does not touch the
  halt.
- **LMC-2, sixteenth-entry test.** If any clause is new-primitive, P
  demands a sixteenth library entry. The certified halt then binds, on
  every branch of the Stage-4 cone: the guarded register is empty
  (semantic O(16) = ∅ at full instance granularity — there is no live
  demand P could pay), so acceptance could only be open-band certified
  earned novelty, which the act-local provenance ceiling ν ≤ 4κ
  forecloses. Verdict `SixteenthEntryForeclosed`.

## 4. Gauge-sector corollary (feeds XF-B3)

A proposed additional *gauge factor* is a sixteenth-entry candidate unless
its declared action factors through the sealed alphabet's faithful image
(the exact-ℤ₆-kernel representation fact: G₀/ℤ₆). Each proposed extension
is adjudicated through this pipeline individually; no blanket verdict is
issued here (that would be pre-evaluation).

## 5. LMC-3 — standing veto semantics (burn discipline, F-XF3)

**"Carried by", defined.** A dossier *carries* a measured behavior only if
(a) its Part A classifies `RecombinationOfB15` under this pipeline, and
(b) its Part-B empirical identification covers the confirming
observation's published discriminators (observables, quantum numbers,
coupling structure, within the experiment's stated uncertainties), as
adjudicated in a versioned, published adequacy artifact whose adequacy
standard is declared per-discovery **before** any candidate dossier is
examined (verdict-blind). Mere typeability never carries anything;
adequacy without lawful classification never carries anything.

- **LMC-3a, presumptive veto.** Upon confirmed measurement of a
  fundamental structure previously classified `SixteenthEntryForeclosed`,
  the veto is recorded in `docs/XF_LEDGER.md` **by default**: the
  observation is published verbatim and the prior evaluation is marked
  `Burned`. The record may be lifted only if, within the registered bound
  below, a dossier carrying the measured behavior (as defined above) is
  exhibited inside the versioned survival adjudication — whose grounds may
  not cite the desire to survive. The burden of production sits entirely
  with the theory.
- **LMC-3b, registered bound.** The survival adjudication permits at most
  **three** versioned dossier attempts, each published whatever its
  verdict, all logged in `docs/XF_LEDGER.md`. Exhaustion of the bound
  without a carrying dossier makes the `Burned` record permanent,
  automatically. Kill-stays-killed is the default state, not the appeal
  outcome.
- **LMC-3c, coverage.** A confirmed fundamental structure with no prior
  LMC classification obligates a pipeline evaluation in a versioned
  artifact within the same registered bound; a resulting
  `SixteenthEntryForeclosed` is an immediate LMC-3a veto. A persistent
  `NotFormalizable` on a confirmed fundamental structure is recorded in
  the ledger as an **adequacy failure of the criterion** — a distinct,
  tracked outcome, published, never silent abstention. The theory does
  not survive by declining to evaluate.
- **Corroboration.** If the confirmed structure is carried (in the defined
  sense) by a recombination dossier, the observation corroborates
  completeness and is logged as such.

## 6. Replayability contract

An evaluation artifact must pin:

- the Part-A digest and Part-B digest (separately);
- **content digests of the classification machinery**: blake3 over the
  canonical source of the elaborator and of the family-computation /
  closure-classification implementation (the pen-type and pen-eval
  sources actually compiled), recorded alongside the hand-versioned
  compat tags (`elaborator_hash`, `token_rules_hash`) — the tags alone
  are version labels, not machinery pins, and are insufficient by
  themselves;
- the per-clause table in a canonical serialization: clause order = Part-A
  order, fixed schema (clause, elaboration verdict, canonical family,
  closure classification, anchor-table row if any), with the schema's own
  digest recorded;
- the verdict; and the digests of every consumed certificate.

Replay = re-elaboration and re-classification under the pinned content
digests reproducing the canonical table byte-for-byte. A drift between
replay and record is disclosed, never repaired silently (the MS-1
drift-manifest discipline applies).

## 7. What this build claims and refuses to claim

With the LMC, every search for new fundamental structure becomes a test of
this theory: each confirmed fundamental discovery ends as corroboration
(a carrying recombination dossier), a recorded veto (`Burned`), or a
tracked adequacy failure of the criterion itself — never silence. Without
prejudice to that, this criterion claims nothing about any specific
proposed structure, fits no curve, and inherits the campaign's hazard
fence: no φ/Fibonacci/ledger-integer pattern-spotting, and no path to the
fine-structure constant is asserted or sought.

## 8. Pre-freeze adversarial stress record

Three isolated critic lenses (register-leak, replayability, prejudgment)
audited the pre-freeze draft; every substantive finding was independently
re-verified by refutation-first checkers. Six findings were confirmed and
repaired before the freeze; the full panel record lives in the campaign
session's workflow journal (run `wf_280902ea-c89`):

1. LMC-3's veto trigger was a universally quantified negative over an
   unbounded dossier space — undischargeable, so a burn could stall
   forever. Repaired: presumptive veto with burden of production on the
   theory (LMC-3a) and a registered three-attempt bound (LMC-3b).
2. "Carried by" was undefined, making the load-bearing predicate of every
   future veto unadjudicable. Repaired: the two-condition definition with
   a per-discovery, verdict-blind adequacy standard (Section 5).
3. Confirmed structures never evaluated, or abstained on, escaped both
   branches of LMC-3. Repaired: LMC-3c coverage clause; persistent
   abstention on a confirmed structure is a tracked adequacy failure.
4. The compat tags pin version labels, not machinery. Repaired: content
   digests of the actually-compiled elaborator and classifier sources
   required in every evaluation artifact (Section 6).
5. The classifier had no pinned identity and "the four certified falsifier
   shapes, generalized" invoked an uncertified generalization. Repaired:
   classifier content digest required; "generalized" deleted — the four
   shapes exactly, all else fail-closed (Section 3, LMC-1).
6. The classification/empirical boundary was asserted but not structural.
   Repaired: two-part dossier with separate digests; the verdict is a
   replayable function of Part A alone (Section 2).
