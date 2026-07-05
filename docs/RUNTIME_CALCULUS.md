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
