# Agent E — SCHEMA2: the intended depth-two schema grammar (terminal long pole)

**Date:** 2026-07-19. **Frozen requirements document:**
`docs/step_15_completion_open_problem.md` (the rewritten spec). This brief
is the execution plan for that spec; where they differ, the spec governs.

**Standing:** construction task. Admissible context: the spec; all archival
results (KERNEL-BRIDGE v3/v4/v5, TRUNC-ER, HIST-CERT v1/v3, BOUNDARY-AUDIT,
DEMAND-COMPLETE, TDC cubical v3); the adopted axiom stack (A5; boundary v3
element-overlay; `boundary-charge-zero-reference-only-v1`); the frozen T4
law; `SEMANTIC_NORMALIZATION_PROGRAM.md` and both burned EGP results as
*diagnostic data*. Forbidden: the bar value in any derivation path; any
Step-16 acceptance computation; any EGP-v3 reselection (separate
preregistration only); **any constructor decision made by retrospective
reference to a desired count** (the spec's own prohibition — recorded
counts are regression outputs, never definitional inputs).

**Ownership:** new crate `pen-schema` + `agda/Schema2.agda` (Agda-first,
intrinsically typed, `--safe`, no postulates, no K). Read-only use of
`pen-type::{cubical, substitution, fuel_composition}`. No mutation of the
shipped evaluator, kernel/token-rule hashes, or any archival artifact.

## Obstructions this brief retires (each only by replayed theorem tokens)

| obstruction | source | phase |
| --- | --- | --- |
| `C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS` | KERNEL-BRIDGE | E-4 |
| `C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION` | KERNEL-BRIDGE | E-1 |
| `D4_A3_CW_INSTANCE_GRAMMAR_ORBIT_QUOTIENT_D_MEMBERSHIP_UNDEFINED` | DEMAND-COMPLETE | E-5 |
| HIST-CERT ordinary remainder `5/6/5/8` (→ F-T1 path) | HIST-CERT v3 | E-2 |
| `C6_V3_GENERAL_HISTORICAL_TERM_LEVEL_COMPLETION` | KERNEL-BRIDGE v3 | E-3/E-7 |
| `C8_CANDIDATE_BOUNDARY_PROVENANCE_JOIN` + domain-wide C-3 | KERNEL-BRIDGE | E-8 |

## Phases (strict order; each phase has a grader; no phase is done without it)

- **E-1 (contexts and substitution).** Spec §1.1/§1.4: typed schema
  contexts (universe/type params, opaque element params, library refs,
  interval vars, cofibrations; dependency allowed), with formation, lookup,
  legal exchange, weakening, and substitution as judgments. The dependent
  typed-instance judgment extends restricted C-1 to genuine typed
  expression images. Grader: Agda mirror extends `KernelBridge.agda`;
  identity/composition/preservation property tests; the four TRUNC-ER
  endpoint maps re-derive as instances.
- **E-2 (the grammar + ordinary-family realizers).** Spec §1.2: the
  inductive `Schema2(W)` with every constructor carrying formation/typing
  rules, a depth proof, a support window, and a semantic interpretation.
  Deliver first the ordinary-family constructors and their typed
  realizers — formation, point/unit, recursor/inductor, Trunc parametric
  action, post-path operation/coherence/cell families — because they close
  HIST-CERT's `5/6/5/8` remainder and unblock Agent A's final rerun as an
  **early handoff, before E-4..E-8 complete**. Definitional inputs already
  frozen (cite, do not re-decide): family-vs-instance and orbit-creation
  rules per T4 and `SEMANTIC_NORMALIZATION_PROGRAM.md` Phase-2 definitions.
  Any grammar point the frozen record does not decide is an
  **ADJUDICATION REQUIRED** entry — halt that constructor and surface it;
  never choose silently (the interpretation-stack discipline).
  Graders: (i) typed instances at steps 5–8 reproduce the operational
  inventories `7/8/10/18` with the path sub-bundles landing exactly on the
  registered `2/2/5/10` keys; (ii) **the Stage-1 divergence diagnostic**:
  locate, at family granularity, why EGP-v1 scored Stage 1 as `2` where
  the sealed record says `1` — the grammar must *explain* the divergence
  (which construction was double- or under-counted and under which
  definition), producing the first concrete datum for the Phase-5b
  bridge-versus-rederive fork. The explanation is a deliverable, not a fix.
- **E-3 (normalization and frozen equality).** Spec §1.3/§1.5: typed
  normalization with termination, type preservation, replay stability, and
  provenance sufficient to distinguish new families from references;
  frozen univalent equality as an equivalence and congruence respected by
  weakening, substitution, and normalization. This is also where general
  C6 (arbitrary-dimension face-indexed motives, boundary-aware dependent
  coe/hcom) is completed to the depth the historical and schema-3 classes
  require — and no further.
- **E-4 (naturality generators — the C2 theorem).** Spec §1.4: elementary
  generators (typed adjacent exchanges, weakening, typed instance
  substitution, cubical face maps) and the decomposition theorem: every
  legal typed context morphism relevant to the quotient is generated.
  Per-class induction; a class that resists stays a **named per-class
  gap** — no smoothing (F-E1).
- **E-5 (the demand projection — C(W) and F1's instrument).** Spec §1.2's
  scheme constructors restricted to demand schemes: the historical
  instance grammar, the equality/orbit quotient, and the decidable
  `D(B)`-membership judgment with certificates — the exact D-4 triple.
  Agent D's serialized pre-instance seeds are declared input stock, never
  relabelled as `C(W)`. Graders: (i) the coarse ladder including the
  stage-3 demand-before-jurisdiction wrinkle must re-derive as the focus
  projection of independent `C(W)` — losing the wrinkle is failure;
  (ii) **negative control:** the all-empty caller timeline must *fail*
  independent generation (the DEMAND-COMPLETE relativity witness must
  become impossible for the independent generator); (iii) run the
  exhaustive `(S15,S14)` instance check — Guard-Rail §8 Action 1 at true
  instance granularity. **This is the first execution at which Guard-Rail
  F1 is a live, executable falsifier.** A demanded-but-underdetermined
  instance kills Theorem 12, restores focus gating at 16, and rescues the
  halt by the old mechanism: report verbatim, immediately, and stop.
- **E-6 (marginality and internality).** Spec §1.7: predecessor-closure
  marginality invariant under frozen equality and presentation; guarded
  weakening/erasure inverse laws with provenance blocking silent
  reclassification of self-imports.
- **E-7 (the classifier).** Spec §2/§3: define `SLNF(W)` independently of
  the historical answer; implement `classify_W`; **search for faithfulness
  counterexamples before attempting `realize_W`**. Prove inverse laws
  class by class (Foundation, Former, Map, Axiomatic, Modal, HIT/V2,
  Synthesis, Unknown). The pre-registered fallback is explicit: if
  realization fails anywhere, drop surjectivity *by declaration* and
  deliver the one-way injection `Marg2(W) → SLNF(W)` — the upper bound the
  halt needs. A failed realization is evidence against exactness, never
  permission to prune `SLNF(W)` after the fact.
- **E-8 (the candidate bridge).** Spec §4: the total join from all 213
  archived schema-3 rows to typed telescopes and extracted `Marg2(W)`
  classes, per the nine bridge conditions, closing C8 and domain-wide C-3.
  Rows with any named gap stay unpromoted. The ρ-vs-Bar comparison remains
  a separate downstream replay this brief never runs.

## Falsifiers

- **F-E1 (per-class incompleteness).** A generator basis unprovable for a
  class → named gap, partial delivery; the class's rows stay unpromoted.
- **F-E2 (historical regression).** Grammar cannot reproduce `7/8/10/18`,
  `2/2/5/10`, or the ladder-with-wrinkle → fix the grammar, never the
  history; if the grammar is believed right and the mismatch stands,
  stop and report — that is a record-level event for the user.
- **F1 (Guard-Rail, live at E-5).** Report-verbatim-and-stop, as above.
- **F5 (Guard-Rail).** `C(W)` non-finite for some window → report;
  locality survives, the framing weakens; do not truncate to force
  finiteness.
- **F-E3 (spec §7 battery).** Any of: two inequivalent marginal families
  sharing a form; an intended schema with no form; an unrealized form
  under claimed surjectivity; a legal substitution outside the basis;
  classification unstable under weakening/presentation/equality; a
  boundary needing an unbound term or uncharged clause. Retain the
  witness; revise toward the one-way classifier; never delete the code
  point.
- **F-E4 (count-retrofit).** Any constructor or quotient decision traced
  to a recorded count or the bar → the certificate is invalid. Counts are
  graders, exclusively.

## Acceptance

Spec §6 verbatim, plus: create-new artifacts
(`docs/schema2_v1.json` + `docs/SCHEMA2_RESULT.md` per phase batch or
combined), full replay and mutation batteries, all archival digests
byte-stable, Agda `--safe` green. The result document states explicitly
what every prior result has: no Step-16 acceptance verdict, no closing
inequality, no global-halt conclusion — those belong to a separately
preregistered successor (EGP-v3 or the Phase-5b fork), which consumes this
grammar and is adjudicated by the user.

## What lands if this brief completes

The C2/C1/D-4/C6/C8 obstruction set is retired; F-T1 becomes dischargeable
by Agent A's final rerun; F1 has been executed at true granularity, one way
or the other; semantic `O(16)` emptiness and the d=4 demand-orphan
diagnostic lose their conditionality; and the halt/continuation question
reduces to the preregistered reselection fork — a decision, at last, with
nothing undefined inside it.
