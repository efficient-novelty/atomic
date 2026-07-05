# RUNTIME_CALCULUS.md — Stage-0 Design (Freeze Candidate)

**Date:** 2026-07-05. **Status:** design for freeze. No crossing has been computed under these semantics; no numerical output of the rules below has been consulted in writing them. The H16/ε/band predictions are stated in §6 as *blind outcomes to be checked after freeze*, not as design inputs.

**Purpose.** Extend the clause calculus from one-shot library selection (Genesis) to repeated application at runtime. Three capabilities: (RC-1) repeated-application deduplication, (RC-2) throughput/engagement units, (RC-3) runtime obligation-band export. This single layer closes or grades: L-Λ1's n* (blind), L-Λ2's Premises A/B, the w_eff(z) profile (Stage 3), the stalling-state search (Stage 4), C-COURSE T2.

**Design discipline.** Every rule below is pinned to an existing, published principle of the framework. Nothing is a free dial. Where a choice existed, the pin explains why it is forced; if a reviewer finds a genuinely free choice below, that is a defect in this document and must be resolved before freeze — not after a run.

---

## 1. RC-1: Repeated-application deduplication

**Principle pins.**
- P-ν (ch04 postscript, definitional): ν_H(x) = |L(H ∪ {x})| − |L(H)|. Dedup is not an extension — it is the definition of ν, which Genesis never had to exercise under repetition.
- P-kernel (ch04 postscript): "A clause may be costly over one history and free over a later one if the later history has already learned how to derive it. PEN charges only the irreducible burden at the moment of actualization."
- P-internal (ch_genesis_math, the squeeze's internal case): anything reachable from accepted structure by guarded flow is, by univalence, already there — ν = 0 up to univalent equivalence. **This is the transport rule. The dedup semantics is the Step-16 squeeze's internal case, applied per-level instead of once.**

**Definitions.**
- A *stratum* is a schema (telescope) S applied repeatedly: application at level k produces library entries tagged (S, k), with history H_k containing all entries of levels ≤ k plus the ambient library. (Pin: Rule 1, cumulative growth.)
- The *level-shift substitution* σ_k maps level-(k−1) stratum entries to their level-k counterparts: σ_k(Lib(e)) = Lib(shift_k(e)) for e ∈ stratum(S), identity elsewhere.

**The frozen rule (R1 — transport-image discount).** When evaluating application k of stratum S, a clause c of the instantiated telescope contributes 0 to ν_G and ν_H if its expression is the σ_k-image of a clause already accepted at a previous level of S, up to the engine's canonical presentation (Var-renaming and canonical ordering). Pin: such a clause is reachable from level k−1 by guarded flow along the level structure (the temporal shell transports it), hence ν = 0 by P-internal. Clauses that are *not* σ-images — genuinely new cross-references to entries outside the stratum, or new combinations first expressible at level k — are scored by the standard evaluator (six-principle class formulas, unchanged). Pin: P5/P6 of the evaluator derivation; no special-casing.

**Consequence (mechanism, not target).** ν_k declines with k because the σ-image fraction of the telescope grows as the stratum's internal pattern saturates, while the genuinely-new remainder is drawn from a cross-combination space the previous levels progressively exhaust (standard dedup against H_{k−1}). The decline *profile* is a combinatorial output of the library — nowhere in this document is any profile assumed.

**What R1 forbids.** No per-level weights, no decay constants, no tunable discount factors. The only inputs are: the telescope, the level map σ, canonical presentation, and the existing evaluator. If a run produces an unwanted crossing, none of these admit adjustment.

## 2. RC-2: Throughput and engagement (Premise B in engine form)

**Principle pins.**
- P-gauge (EVALUATOR_DERIVATION): selection is invariant under global ν rescaling — only ratios are physical.
- P-widening (ch04 Rule 2): the horizon widens only when nothing local suffices.

**Definitions.**
- Per cadence step k, the stratum's *serviced demand* D_serv(k) is the κ-weight of obligations discharged by accepted applications at step k; the *total demand* D_tot(k) is the ledger's exported debt at step k (Δ_k in ledger units).
- The *engagement fraction* e_k = D_serv(k)/D_tot(k) ∈ [0, 1]. Pin (P-gauge): capacity enters ONLY as this ratio; no absolute clause-per-second quantity is ever defined, because none would be gauge-invariant.
- *Saturation* (Premise B, now formal): the step at which e_k < 1 first holds with the widening move active — the stratum can no longer service total demand, and shares of demand become fractions of standing debt. The trigger is the first such step. Pin: P-widening — the widening activates exactly when locals fail, not before.

**Note.** This replaces v1/v2's ill-posed "capacity in absolute clause units" (the C2 no-go). The debt reading's crossing is now: first k with e_k < 1.

## 3. RC-3: Runtime obligation-band export (Premise A in engine form)

**Principle pins.**
- P-window (Appendix D): obligations factor through the two-layer chronological window — each obligation's referent lives in the live layer (k−1: *attachment*) or the sealed layer (k−2: *echo*).
- P-admissibility (pen-type): attachments are non-deferrable — a candidate failing depth-1 attachment does not seal (the bare-S³ precedent: admissibility gates beyond ρ, with exact κ-band export via `requires_sphere_lift_package`).

**The frozen rule (R3 — band typing and export).** Type every obligation of an application by the layer of its referent: depth-1 (attachment, referent at k−1) or depth-2 (echo, referent at k−2), read off the chronological factorization — no other typing input is admissible. At a step with e_k < 1: depth-1 obligations must be fully serviced for any application to seal (P-admissibility); the unserviced remainder is exported as a typed band on the interface. The engine reports the band's depth composition. Prediction (§6, blind): the export is the depth-2 band.

## 4. Conservativity and regression requirements (Stage-1 gates)

1. **Genesis conservativity:** with no repeated applications, RC-1/2/3 change nothing — the strict 15-step lane must remain hash-identical (accepted blake3:e919c841…, canonical blake3:6f4b65c2…).
2. **No-go regression:** with R1 disabled, the constant-stratum crossing must reproduce n = 3 (lambda_trigger_v2 tests stay green).
3. **Mechanics check (no crossing reads):** with R1 enabled on the v2 hierarchical variants, ν_k must be non-increasing beyond the first level, and strictly decreasing at some level (else R1 is inert and the implementation is wrong). This check inspects the ν profile only — the crossing index must not be computed or logged in Stage 1.

## 5. Staging and freeze protocol

- **Stage 0 (this document):** semantics design, principle-pinned. Freeze by commit. Reviewer pass invited: find a free dial.
- **Stage 1 (machine agent):** implement RC-1/2/3 (suggested: `crates/pen-eval/src/runtime_dedup.rs`, extending — not modifying — the existing evaluator surface). Gates of §4 as tests. Crossing computation code EXCLUDED from this stage (enforce by not implementing the reporting path).
- **Stage 2 (freeze → blind run):** freeze commit; then a single run computes, for the frozen v2 stratum variants (`bound_interface_*`, unchanged from their existing freeze): the ν_k profile, the crossing index (first k with e_k < 1), and the exported band composition. Output to `docs/runtime_calculus_run.json`. Report whatever comes out.
- **Stage 3:** discharge dynamics on the standing band (the w_eff(z) profile) — separate design doc; not part of this freeze.
- **Stage 4:** stalling-state search (C-FATE-2's judge) — after Stage 2.

## 6. Blind outcomes to be checked (stated once, then not consulted)

Registered predictions this calculus will grade — the implementation must not branch on any of them:

- **B1 (H16):** the crossing index for the structure stratum is 16. A frozen-semantics crossing elsewhere is a REAL miss (unlike the v1 invalid run): it burns H16 per its original registration, no retuning, and the Λ chain's trigger reverts to observation-selected with the closed form refiled as retrodiction.
- **B2 (ε):** the standing export share at the crossing is Δ_{k−2}/Δ_k (= 377/987 at k = 16).
- **B3 (band):** the export is the depth-2 band, exclusively or dominantly.
- **B4 (C-COURSE):** the stratum's Ω-course resembles the Genesis Ω-course (same grammar, same menu); Bar₁₆^(s) in stratum units lands near φ·Ω₁₅^(s).

Failure semantics are those already pre-registered in the notes (`pen/book/note_lambda_trigger_lemma.md`, `note_lambda_coefficient_lemma.md`, `note_canonical_course.md`).

## 7. Relation to prior no-gos

The v2 no-gos established that no *frozen constant* schema can probe H16 under the old semantics (constant-ν crossing always 3; spec-κ never reaches the band). RC-1 removes the constancy (ν_k declines by definition-faithful dedup); RC-2 removes the ill-posed absolute capacity (engagement ratio). The no-gos remain as regression tests, not as obstacles: they were facts about the missing layer, and this document is the missing layer.

## 8. Stage-0 reviewer resolutions (2026-07-05, pre-freeze)

The invited reviewer pass (§5, Stage 0) found three genuinely free choices. Per §0's discipline they are resolved here, before freeze. R-F1 and R-F2 supersede the corresponding phrasing in §1; R-F3 is a named open pin that gates the Stage-2 freeze.

**R-F1 (resolved) — σ-image clauses contribute 0 to all of ν.** R1's phrasing "contributes 0 to ν_G and ν_H" left the ν_C treatment of σ-image clauses ambiguous. Resolution: a σ-image clause contributes 0 to ν_G, ν_C, and ν_H. Pin: P-internal delivers ν(Y | B) = 0 as a whole — by univalence the transported clause *is* accepted structure, and P-ν's marginal |L(H ∪ {x})| − |L(H)| is component-blind; no principle supports charging a connective residue on a clause that is already there. Operationally: the level-k evaluation scores the *residual telescope* (the non-σ-image clauses, order preserved) with the unchanged six-principle evaluator against H_{k−1}. κ is untouched by R1: it remains the specification κ of the full instantiation (R1 is a rule about ν only; the D3 charged-κ question is out of scope for this layer).

**R-F2 (resolved) — the instantiation map is pinned.** §1 defined σ_k but not the level-k instantiation itself, and the frozen v2 variants contain no stratum-entry Lib references for σ_k to act on (they are Var-connected internally). Resolution: instantiate(S, 1) = S as frozen; for k ≥ 2, every stratum-internal Var reference of S — Var(d) in clause j with d ≤ j, which is the engine's existing clause-connectivity convention, not a new one — rebinds to Lib of the level-(k−1) sealed stratum entry. σ_k is the induced level shift (stratum-entry Lib references shift up one level; ambient references unchanged), and instantiate(S, k) = σ_k(instantiate(S, k−1)) for k ≥ 3 by construction. Pin: Rule 1 (cumulative growth) — once level k−1 seals, the schema's self-reference *is* a library reference to realized history, and the temporal shell transports the pattern one level (P-internal's guarded flow). Consequence, stated for auditability and not as a target: clauses with only ambient references become σ-images from level 2; clauses consuming the stratum's own product are first expressible over a *sealed* prior level at level 2 (genuinely new cross-references), and become σ-images from level 3.

**R-F1 corollary, noted for the auditor (2026-07-05, post-Stage-1 review):** scoring the residual as a standalone telescope means the residual can change *class* relative to the full instantiation. Hand-derivation of the Stage-1 profiles: `bound_interface_persistent` gives ν = [12, 24, 0, 0, …] — at level 2 the PathCon is discounted as a σ-image, the residual reclassifies Hit → Map, and the |lib_refs|² term produces a transient level-2 bump before total burnout at level 3. This is the unchanged evaluator doing what R-F1 says (no new dial is proposed to suppress the bump), but the bump and the burnout are both material facts for R-F3 below, and the Stage-1 gate-3 wording ("non-increasing beyond the first level") should be read as *from level 2 onward*, which is what the implementation tests. A mechanics artifact recording the actual profiles (ν numbers only — no engagement fractions, no crossings) should be committed for the audit trail.

**R-F3 (OPEN — gates the Stage-2 freeze) — the application schedule per cadence step.** RC-2's D_serv(k) ("κ-weight of obligations discharged by accepted applications at step k") is not computable from the stated inputs: the number and domain of applications attempted per cadence step is unpinned. Candidate pins, recorded without adoption: (i) one application per cadence step (level = step) — literal, but D_serv is then bounded by specification κ and the C2-shaped early crossing reappears, contradicting §7; (ii) within each cadence step the stratum applies across all σ-distinct admissible instantiations over prior sealed entries until nothing local suffices — pinned by P-widening and by §1's cross-combination language, but the instantiation domain then needs its own freeze. Stage 1 therefore implements RC-2 and RC-3 as definitional machinery only (engagement ratio, obligation typing, band export — synthetic tests, no schedule), and the Stage-2 freeze commit must first resolve R-F3 in this document. No crossing under either candidate pin has been computed.

## 9. PROPOSED resolution of R-F3 (2026-07-05, for reviewer pass — not yet adopted)

**The pin: the schedule is not a new rule — it is the Selective Law itself, run at token scale.** Within a cadence step, the stratum's applications are chosen by the standard selection loop over the *instantiation field*: admissibility first, bar-clearing, smallest sufficient overshoot, constructive primeness — the same five rules, unchanged. The cadence step *ends at local exhaustion*: when nothing in the local field clears, the step closes and the widening move is the only remaining motion (Rule 2, verbatim). D_serv(k) is then the κ-weight discharged by the applications accepted during step k — computable with no new quantity defined.

**The instantiation field (the domain question R-F3(ii) raised).** The field at any moment is everything the grammar can bind: instantiations of the stratum schema over *tuples* of sealed entries (own prior levels, other strata, ambient library), R1-deduped (σ-images and history-derivable combinations score 0 and hence cannot clear any positive bar), admissibility-gated, and anti-bundled (a package of independently sufficient applications is not one candidate — Rule 4). No arity cap and no domain whitelist is imposed: caps would be dials. The field is generated by the grammar; dedup and the bar do the limiting.

**Why this pin is forced rather than chosen.** (a) Candidate (i) of R-F3 (one application per step) reproduces the C2 no-go and contradicts §7 — eliminated. (b) Any *fixed multiplicity* or *fixed arity* schedule is a dial with no principle behind it — forbidden by §0. (c) What remains is to let the calculus that already governs selection govern it here: PEN is scale-free by construction (ch27 runs it at thought scale; the Genesis lanes run it at library scale). The only residual policy item is bounded search, which is existing disclosed lane practice (the Genesis relaxed lanes), not a new freedom.

**Consequence for the Stage-1 burnout finding.** Under single-chain instantiation the toy strata burn out at level 3 (profiles [12, 24, 0, …], [10, 15, 0, …]): chain-repetition alone cannot sustain a course, exactly as the §8 corollary records. Under this §9 pin, sustained decline must come — if it comes at all — from the *growing tuple field* over accumulating sealed products fighting R1 dedup. Whether that contest produces a course, and where it stalls, is precisely what the blind run measures. If the field also exhausts early, the crossing lands early, and B1 fails as a real miss: the exposure stands.

**Freeze checklist for adopting this section:** (1) reviewer pass on this §9 — find a dial (the declared candidates: the lane-search budget, which must be disclosed as policy with a sensitivity check across budgets; and the cadence-step boundary, pinned here to local exhaustion — challenge it); (2) Stage-1 addendum implementing the field generator + step loop behind the same no-crossing exclusion; (3) mechanics artifact (ν profiles under the field schedule, numbers only); (4) freeze commit adopting §9; (5) Stage-2 blind run.
