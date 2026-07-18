# Semantic Normalization Program

**Date registered:** 2026-07-18.
**Target:** discharge the extraction, anchoring, historical-reselection, and
exhaustion obligations of
`docs/CERTIFIED_HALT_15.md` §6 and `certified_halt_verification.json`
(`semantic_boundary.outstanding_obligations`), so that exactly one of the
following preregistered outcomes is certified:

- **(A) Anchored embedding.** A real typed elaboration and reduction kernel
  constructs normalized natural-family and demand-orbit inventories, plus a
  family-valid injective EGP classifier (no full-code realizer or saturation
  law is required); `anchored_provenance_embedding_supplied`,
  `shipped_raw_telescope_quotient_complete`, and
  `original_global_halt_proven` all flip to `true` only after J2/J3 and window
  locality, conservative internality, full Step-16 exhaustion, and all fifteen
  candidate cones, orders, winners, and scores have also been re-audited; the
  Genesis halt at Step 15 is then proved, not conditional.
- **(B) Independence.** A certified counterexample shows the shipped
  structural projection and the proof-carrying EGP projection cannot be
  reconciled. A counterexample may be a well-typed marginal family with no
  valid local-role or live-demand anchor, two distinct families forced into
  one provenance tag, a demand orbit omitted by J2/J3, a failure of window
  locality, or an admitted Step-16 flow outside the internal/EGP split. This
  is a publishable independence result, and the selection calculus inherits
  a *declared* choice of projection.

Both outcomes are wins. What is not a win is a third state in which the
obligation stays open but the halt claim hardens by repetition.

**Book-side stakes** (from the companion volume's appendix chain): the
anchored embedding is one load-bearing theorem, but it is no longer presented
as the sole missing fact. Orbit completeness/window locality, internality,
historical reselection, and Step-16 domain exhaustion remain separate gates.

---

## 0. Ground rules (read before Phase 1)

1. **Preregistration.** This document is committed before implementation
   begins. The expected outcomes in §7 are frozen now. The first full
   historical reselection run (Phase 5b) is a burned run: its numbers count,
   whatever they are.
2. **Frozen surfaces.** All work is within the certified caps:
   `2 <= kappa <= 4`, depth `d <= 1`, `max_expr_nodes = 6`, raw leaves
   `r <= 2` (only L14, L15). The expression alphabet is the frozen one
   (`Lam` + 4 modal + 2 temporal unary constructors, plus the binary/base
   formers). Here expression depth is distinct from the two-stage historical
   support window used by the demand-orbit audit. Nothing in this program
   mutates the shipped structural evaluator, frozen records, or bar law. The
   semantic run is a separate full reselection whose scores and winners are
   not assumed to match those records. Conservativity in the sense of the
   dual certificate (`structural_evaluator_left_unchanged`,
   `history_identical_before_and_after_audit`) must hold at every commit; the
   latter is an input-record checksum, not a claim that semantic winners agree.
3. **Compat discipline.** The kernel and token rules get their own hashes
   (`elaborator_hash`, `token_rules_hash`), added to the resume/replay
   compat gate alongside `ast_schema_hash`, `type_rules_hash`,
   `evaluator_hash`, `search_semantics_hash`. Any change to kernel or
   token rules invalidates downstream certificates by construction.
4. **No public token constructors.** This stays a compile-time invariant:
   token types live in the kernel module with `pub(crate)`-at-most
   constructors behind a sealed trait; CI greps the public API surface and
   fails on any public constructor. The certificate records the module
   boundary hash.
5. **Every phase has a grader.** Listed per phase; the summary matrix is
   §8. A phase without a passing grader is not done.

---

## 1. Phase 1 — The bounded elaboration/reduction kernel

*Obligation addressed:* "connect trusted typed-eliminator and naturality
evidence to a real elaboration/reduction kernel."

**Scope discipline (the main risk).** This is NOT a general dependent type
checker. It is a bidirectional elaborator for the finite fragment reachable
within the frozen caps, over the sealed library signature `∂H_15` (library
snapshot from `checkpoints/steps/step-15.json`). Six nodes, depth one, four
clauses: every elaboration problem is small and finite. Resist every
temptation to generalize.

**Deliverables.**

- `crates/pen-type/src/elaborate.rs`:
  - judgments `Γ ⊢ e ⇒ A` / `Γ ⊢ e ⇐ A` over the sealed signature; library
    entries enter as opaque constants with their exported formation,
    eliminator, and computation clauses only;
  - weak-head reduction using exactly the exported computation clauses;
    fresh heads with no exported eliminator (the temporal pair in
    particular) are *stuck by construction* — stuckness detection is a
    kernel verdict, not an AST pattern;
  - fuel-bounded evaluation with a certified static fuel bound
    (measure: node count x clause count within caps); the certificate
    records `max_fuel_observed` and the static bound, and the grader
    checks `observed <= bound`.
- Token issuance as *derivations*:
  - `TypedEliminatorToken`: constructible only from an elaboration
    derivation exhibiting a real `Formation` clause, an oriented beta/Kan
    basis, and an eliminator typed against its motive;
  - `NaturalityToken`: derivation that the naturality square closes up to
    the frozen judgmental equality;
  - `TypedLiftToken` (P5 route): unique reachability-dominant direct
    import plus a typed lift derivation;
  - each token embeds the BLAKE3 of its derivation; replay re-derives.

**Graders.**
- Golden tests: the fifteen sealed steps' exported clause sets elaborate
  and reduce to their frozen normal forms (this is the kernel's
  conservativity regression — Genesis 1-15 as fixed test data, per the
  open-problems chapter's instruction to freeze them as regression, not
  retune them as training data).
- Mutation tests: perturb a beta rule, an import, a motive — every token
  derivation must fail closed.
- `cargo test -p pen-type elaborate --lib` green; fuel certificate emitted.

---

## 2. Phase 2 — Typed semantic families and canonical representatives

*Obligation addressed:* extract the objects that the strengthened law counts,
rather than attaching semantic names to raw AST nodes.

**Operational definitions.** A schema family is admitted only when all five
checks below carry kernel derivations in its certificate:

1. **Typed normalization:** elaborate the clause and reduce it to a stable
   typed normal form under the sealed signature.
2. **Weakening:** quotient only by checked weakening/renaming maps; record the
   maps so equality is replayable rather than string-based.
3. **Univalent equality:** identify normal forms only through the frozen
   univalent-equality decision procedure and retain its equality witness.
4. **Natural family versus instance:** a natural family carries its parameter
   telescope and naturality derivation. Uniform specializations are members of
   that one family, not additional credit, unless a specialization is exported
   as a separately identified required output of a live demand orbit.
5. **Marginality:** test the normalized natural family against the typed
   predecessor closure. Derivable/inherited families are internal and receive
   zero marginal credit; only non-derivable families enter the provenance
   classifier.

**Deliverables.**

- A kernel-backed extractor producing canonical `NaturalFamilyId` values,
  checked instance membership, and explicit marginal/internal dispositions
  for every admitted candidate.
- Canonicality under telescope presentation, weakening, and the frozen
  univalent equality: equivalent presentations produce the same family ID;
  non-equivalent families are never merged merely because they share a shape
  label.
- Family validity as a dependent kernel judgment. It must be impossible for a
  caller to manufacture a counted family by supplying an arbitrary string,
  token name, or support-local bucket.
- Total extraction over the *enumerated candidate domain*. This is not a
  claim that nine support-local forms are semantically saturated, and it does
  not require constructing an inhabitant for every finite code.

**Graders.** Kernel derivations replay; presentation/weakening/univalence
property tests agree on streamed samples from all three kappa strata; mutation
tests reject an altered type, naturality square, equality witness, predecessor
closure, or family ID.

---

## 3. Phase 3 — Demand orbits, EGP anchors, and the one-way classifier

*Obligation addressed:* refine debt from focus-family labels to individual
semantic demand orbits and prove that every credited marginal family has valid
extraction-guarded provenance.

**Demand-orbit extraction.** For every historical depth-two support window,
including `(S14,S15)` for Step 16, extract a finite inventory of normalized
`DemandOrbitId` values. Each orbit records its generating window, natural
family, finite set of required output positions, and its live, answered,
derivable, or expired disposition. The certificate must prove:

- **J2 (extraction completeness):** no normalized demand orbit generated by
  the active window is omitted;
- **J3 (disposition completeness):** every extracted orbit and every required
  output position has a checked disposition;
- **window locality/expiration:** older live debt either persists through a
  checked transport into the active window or carries a checked discharge or
  expiry. An empty `(S14,S15)` vector is not debt freedom by itself.

**EGP classifier.** For each counted marginal natural family `f`, compute one
canonical tag in

```text
ProvTag = (KernelClause × LocalRole)
        ⊎ Σ (o : LiveDemandOrbit), RequiredOutput(o),

LocalRole = KernelHead | AdjointMate | SupportAction | Coherence.
```

The certificate contains both a family-valid injection `pi` and dependent
evidence `Anchors(f, pi(f))`. `Anchors` proves that the exact family is either
the named local semantic role of the named kernel clause or answers the named
required output of a pre-existing live orbit. Injectivity without this
dependent evidence is not a semantic proof. The demand branch additionally
checks the orbit's active window, liveness, required output position, and
non-reuse.

This is a one-way upper-bound classifier. No `realize`, surjectivity,
`classify/realize` inverse, or saturation theorem is required. The old nine
support-local codes may remain useful coarse diagnostics, but they are neither
the target quotient nor evidence that `Anchors` holds.

**Amplification reconciliation.** P5, P6, `d^2`, `r^2`, and combinatorial
synthesis are audited family by family. Independent credit must occupy a
distinct valid local-role slot or answer a distinct live required output.
Uniform specializations remain one natural family unless they are independently
exported demand-orbit outputs. Labels such as “P6” or “synthesis” do not mint
credit.

**Graders.** J2/J3 and locality certificates replay for every window; duplicate
orbit/output use, stale-window anchors, malformed family evidence, and tag
collisions fail closed; direct kernel comparison confirms canonical family IDs
and every `Anchors` judgment.

---

## 4. Phase 4 — Internality, the blind bound, and Step-16 exhaustion

*Obligations addressed:* prove conservative internality for guarded
Step-15-generated flows, derive the finite local-role bound without semantic
case storytelling, and cover the whole admitted Step-16 cone.

**Conservative internality.** For a normalized flow generated solely by the
Step-15 interface, construct checked weakening and erasure maps and prove the
two inverse laws on the guarded subspace:

```text
erase (weaken f) = f
weaken (erase g) = g.
```

The equalities are typed equalities modulo the same normalization and
univalent equality used in Phase 2. They imply that the guarded extension adds
no marginal natural family, hence `AtMost Marginal 0`. A syntactic reference to
Step 15 is not an internality proof; the guard, maps, and both inverse
derivations must be serialized and replayed.

**Blind local-role bound.** Once J2/J3 and locality prove that the Step-16 live
demand summand is empty, the EGP injection lands in
`KernelClause × LocalRole`. With `|KernelClause| = kappa` and the four fixed
roles, the machine-exported theorem is

```text
AtMost Marginal (4 * kappa).
```

The proof counts the codomain of the valid injection; it does not enumerate
P5/P6 meanings or assume a nine-class ontology. `AtMost (9 * kappa)` may be
exported only as a weakening of this same four-role certificate, not as an
independent fallback with five anonymous roles. The arithmetic theorem already
present in `agda/ProvenanceBound.agda` is only the conditional boundary; this
phase must supply its typed `ValidAnchor` and debt-free inputs.

**Step-16 domain exhaustion.** The surface cannot be materialized (3 GiB
aborts; raw products up to 5.9e24 — `t1_result.md` §2). Extend the exact
counting automaton with the finite kernel features needed to reconstruct the
canonical family disposition and certificate reference for each state. Its
partition is:

```text
internal-by-weakening/erasure
| EGP-certified marginal
| invalid-or-unclassified.
```

Prove the last part has cardinality zero in every kappa stratum, and that the
first two counts sum to the frozen total catalog width. On a materializable
sub-cap, exhaustively compare concrete typed extraction, family IDs,
internality, provenance tags, and anchor validity with the automaton. The
historical bar used in the closing inequality is the *recomputed* Bar16 from
Phase 5b. The legacy value `354333/39040` may be used only if the full semantic
reselection reproduces it.

**Graders.** Weakening/erasure inverse proofs replay; Agda checks the injective
`AtMost` export; partition sums and the zero-unclassified proof replay;
small-cap bisimulation is exhaustive; mutations of a map, anchor, count,
completeness flag, or bar provenance fail closed.

---

## 5. Phase 5 — Falsifiers and history

### 5a. The four registered falsifiers (adversarial regression)

Each of `hit_no_formation_d1`, `temporal_polymorphic_kappa2`,
`axiomatic_single_l15_kappa3`, `axiomatic_inheritance_kappa3` receives a full
typed-family disposition. When a derivation fails, the certificate names the
required token (H-form eliminator, naturality, P5 lift), replays the attempt,
and records the exact failure point (no formation clause; stuck fresh head; no
reachability-dominant import). A failed token cannot silently become “bounded
opaque credit”: every remaining marginal family still needs a valid EGP anchor
or else remains in the invalid/unclassified part and blocks exhaustion.

This is the internality and provenance gate closing *with receipts* — an AST
pattern, a P5/P6 label, or an arbitrary evidence name cannot manufacture a
semantic theorem. A falsifier is below the final bar only after its certified
score is compared with the recomputed historical bar.

**Grader.** `cargo test -p pen-search
t1_adversarial_probe_records_only_raw_minimal_clearing_survivors --lib`
extended: no unguarded semantic credit; per-falsifier family inventory,
failure trace, anchor disposition, and exact score serialized.

### 5b. The historical reselection (burned run; preregistered decision gate)

Do not merely rescore the recorded winners. Starting from the initial state,
run a full sequential semantic reselection:

1. Enumerate the complete admissible candidate cone at the next stage and
   commit its digest.
2. For every candidate, extract typed natural families and demand orbits,
   replay internality or valid EGP anchors, and compute its exact revised
   score. P5, P6, `d^2`, `r^2`, and synthesis contributions survive only to
   the extent that their individual families occupy independent anchors or
   answer live required outputs.
3. Recompute the exact order, tie-breaks, and winner; update the typed history
   and demand ledger with that winner; then derive the next bar from this
   revised prefix.
4. Repeat through Stage 15 and publish every cone, order, winner, score,
   demand-ledger transition, and Bar value.

The legacy vector `1,1,2,5,7,8,10,18,17,19,26,34,46,62,103` is a regression
input, not a semantic result. Earlier amplification may indeed survive because
those stages answer live debt, but the orbit/output correspondence and
non-reuse proof must be exhibited; it is never inferred from the old score.

**Decision gate.** Only after full reselection compare the revised trace with
the sealed structural trace. If all cones, orders, winners, scores, and bars
agree, record a semantic reenactment. If any differ, publish the first
divergence and the complete revised prefix. Do not reuse the old Bar16 or an
exhaustion proof over the old `H15`. The lawful continuations are then (i)
declare the shipped structural audit as a constitutive scoring bridge, with a
separately justified semantic halt gate, or (ii) adopt the semantic audit and
continue re-deriving its history. That choice is Two-Law bridge content and
belongs in the book's ledger; this program makes the fork explicit and
certified.

**Grader.** Exact-arithmetic, full-cone reselection certificate; independent
replay reconstructs every order and ledger transition; the comparison branch
and first divergence, if any, are data fields rather than prose.

---

## 6. Phase 6 — Certificate, flags, and book-side closure

- Extend the generated certificate with `elaborator_hash`, normalization,
  weakening, equality, orbit, and token-rule hashes; canonical family and
  demand-orbit inventories; J2/J3/locality evidence; dependent `Anchors`
  evidence; injection and internality certificates; automaton partition and
  bisimulation digests; falsifier traces; the full historical reselection; and
  the final Step-16 inequality with explicit bar provenance.
- Flip `anchored_provenance_embedding_supplied` only after the typed,
  family-valid injection and every dependent anchor replay. Flip
  `shipped_raw_telescope_quotient_complete` only after invalid/unclassified is
  proved empty over the whole admitted Step-16 domain. Flip
  `original_global_halt_proven` only when orbit debt is discharged, the revised
  history supplies the referenced Bar16, and the exhaustive Step-16 maximum is
  below it. Outcome (B) emits a checked counterexample and leaves the relevant
  flags false.
- Regenerate via the frozen reproduction path:
  `cargo run -p pen-search --release --example genesis_certified_halt --
  --out docs/certified_halt_verification.json`, plus the Agda checks
  (`CountingLemmas.agda`, `ProvenanceBound.agda`, `P5RecordBoundary.agda`,
  `CertifiedHalt.agda`, and `StepWitness.agda`). Agda checks the injective
  `AtMost` theorem and inverse-law consequences; it does not manufacture the
  Rust kernel's semantic witnesses.
- Book-side hooks (companion volume): the CSC2 appendix's verdict clause
  (ii) updates from "fixedness fails" to "fixedness holds" (outcome A) or
  to a certified independence statement (outcome B); the WP2
  admissibility-switch instance at Step 16 closes accordingly; the
  open-problem ledger's WP5/§11 entries update. One paragraph each; the
  appendix structure anticipates both branches.

---

## 7. Preregistered completion criteria (frozen with this document)

| # | Completion criterion | Required result |
|---|----------------------|-----------------|
| C1 | Four falsifiers | no family receives credit from a failed token or label; exact dispositions are certified |
| C2 | Typed family extraction | total and canonical on every enumerated cone; every counted family carries validity evidence |
| C3 | Demand ledger | J2, J3, and window locality/expiration replay for every historical window |
| C4 | EGP | every marginal family has a dependent valid anchor and the classifier is injective |
| C5 | Internality and bound | inverse laws prove the guarded internal part marginally empty; debt-free EGP exports `AtMost (4*kappa)` |
| C6 | Step-16 domain | internal and EGP parts cover the full admitted cone; invalid/unclassified count is zero |
| C7 | Historical audit | all fifteen complete cones, revised scores, orders, winners, ledgers, and bars are reselected |
| C8 | Closing inequality | the exhaustive Step-16 bound is below the Bar16 derived from that same revised history |
| C9 | Kernel discipline | fuel bound holds and every proof/rule hash participates in replay compatibility |

No equality with the legacy scores is a completion criterion, and no divergence
may be repaired silently. A divergence is itself a certified result routed
through the Phase 5b decision gate.

**Current status.** This document is an implementation program, not evidence
that these criteria hold. The present `semantic_provenance_audit.json` retains
`null` revised scores, unknown semantic O(16), and false J2/J3 and internality
flags. The existing Agda arithmetic proves only what follows *if* typed valid
anchors, debt freedom, and inverse-law witnesses are supplied.

## 8. Grader matrix

| Phase | Artifact | Grader |
|---|---|---|
| 1 | kernel + tokens | golden 15-step regression; mutation tests; fuel certificate |
| 2 | typed family extractor | derivation replay; canonical-key and marginality tests; mutation rejection |
| 3 | orbit extractor + EGP | J2/J3/locality replay; dependent-anchor validation; injectivity and non-reuse |
| 4 | inverse laws + bound + exhaustion | Agda `AtMost`; zero-unclassified partition; exhaustive sub-cap bisimulation |
| 5a | falsifier traces | no unguarded credit; localized failures and dispositions |
| 5b | full reselection | exact complete cones, orders, winners, ledgers, bars, and decision branch |
| 6 | final certificate | schema validation; compatibility hashes; full replay; Agda green |

## 9. Sequencing and effort shape

Strict order 1 → 2 → 3 → 5a → 4 → 5b → 6 (5a before 4: the falsifiers are a
cheap, decisive signal for kernel/token/anchor adequacy; 5b last because its
first full run is burned). The long pole is Phase 1; its scope fence (§1, caps,
sealed signature only) is the schedule. Phase 3's orbit extraction and Phase
4's automaton enrichment are independent engineering risks. De-risk both on a
materializable sub-cap before lifting them to the exact counting automaton.

## 10. Relation to the standing no-gos (scope fence, again)

Nothing here derives clock calibration (runtime no-go stands), the public-event
extractor (Section 11 program), phase-with-descent (WP6's merged target), or any
quantitative constant. This program does one thing: it attempts to turn the
conditional EGP boundary into typed, replayable family/orbit certificates and
an exhaustive revised Genesis trace — or produces a checked obstruction. Only
the successful completion branch settles that the intended sequence halts at
Step 15 because of what its candidates mean, rather than because of what the
legacy evaluator happens to count.
