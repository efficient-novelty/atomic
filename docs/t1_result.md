# t1_result.md — T1: The Genesis Squeeze Verification

**Date:** 2026-07-05. **Target:** T1 of `note_canonical_course.md` §7 — engine verification of the book's halt claim (`ch_genesis_mathematics`, "Why the Sequence Halts: The Univalent Horizon"): *no Step-16 candidate clears the bar in the current lane.* **Code:** `crates/pen-eval/src/halting.rs` (verdict layer), `crates/pen-search/src/halting_probe.rs` (drivers), `crates/pen-search/examples/genesis_halting.rs` (artifact). **Status of this document:** results record. It reports what the engine said, separates what is established from what is pending, and states the two available readings without adjudicating between them. The adjudication is registered as open.

---

## 1. Summary

The external half of the halt claim **failed its adversarial engine probe**. Seven hand-constructed, formula-maximizing candidates pass the lane's entire per-candidate gate stack — canonical identification, strict admissibility, type check, connectivity — and clear Bar₁₆ = 354333/39040 ≈ 9.077 by factors of 1.2× to 4.2×. The admissibility layer admits them as `admitted_and_focus_aligned` with reason `open_band_structural`: at step 16, for the first time in the sequence, no structural debt imposes a focus family, and the lane's generic "open band" is exposed. The internal half of the claim (identification and trivial-derivability leave the internal class at ν = 0) **passed** everywhere it was tested.

Whether this trips falsifier (a) of the canonical-course note as registered, or instead exposes an unimplemented layer of univalent identification in the evaluator, is the open adjudication of §6.

## 2. How the result was reached (execution history, kept for the audit trail)

Three driver designs failed before the probe that produced the result; each failure taught something material.

1. **Flat enumeration** (`enumerate_telescopes` per κ): aborted at a 3 GiB allocation. Lesson: the step-16 surface cannot be materialized.
2. **Engine bootstrap driver** (`search_bootstrap_from_prefix_*`): returned instantly with no step-16 record — it clamps at `LIVE_BOOTSTRAP_MAX_STEP = 15`. Lesson: the production lane has *never looked at* step 16; verifying the halt through a driver that cannot reach it would have been circular. The probe's failure classification (`ProbeFailure`, "not evidence for the squeeze") caught this.
3. **Engine per-step machinery, unclamped** (`probe_next_step_unclamped`): the engine's own catalog path also aborted at 3 GiB. Lesson: the step-16 discovery surface is out of engineering range for materializing paths.

**Surface geometry** (exact counting, no materialization): κ band 2..=4, max_expr_nodes 6, focus **None**, quota/bucket 2; raw per-position clause widths 910,182 (κ=2), 1,207,872 (κ=3), 1,559,142 (κ=4); raw telescope products 8.3×10¹¹, 1.8×10¹⁸, 5.9×10²⁴. For contrast, the focused late-Genesis catalogs were a few clauses wide per position. **Exhaustive step-16 verification is therefore infeasible without streaming engineering** — a standing limitation, disclosed. The adversarial probe (chosen strategy) attacks the claim with the formula-maximizing shapes instead: decisive if anything clears; non-exhaustive if nothing does.

## 3. The adversarial probe result

Gate stack per candidate, in lane order: canonical identification against the fifteen sealed entries → `assess_strict_admissibility` (Guarded, step 16) → `check_telescope` → `passes_connectivity` → `structural_nu` → ρ vs Bar₁₆. All candidates constructed within the lane's own bounds (κ band, 6 expression nodes).

| candidate | κ | ν (G/C/H) | ρ | admissibility | type | conn. | clears | **survives** |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| weave_max_r_kappa2 | 2 | 40 (0/40/0) | 20 | admitted (open_band_structural) | ok | ok | yes | **YES** |
| weave_max_r_kappa3 | 3 | 87 (0/87/0) | 29 | admitted (open_band_structural) | ok | ok | yes | **YES** |
| weave_max_r_kappa4 | 4 | 152 (0/152/0) | 38 | admitted (open_band_structural) | ok | ok | yes | **YES** |
| axiomatic_inheritance_kappa3 | 3 | 108 (1/107/0) | 36 | admitted (open_band_structural) | ok | ok | yes | **YES** |
| axiomatic_inheritance_kappa4 | 4 | 110 (1/109/0) | 55/2 | admitted (open_band_structural) | ok | ok | yes | **YES** |
| hit_path_d3 | 2 | 14 (4/0/10) | 7 | admitted (open_band_structural) | ok | ok | no | no |
| hit_path_d4 | 2 | 21 (4/0/17) | 21/2 | admitted (open_band_structural) | ok | ok | yes | **YES** |
| hit_path_d5 | 2 | 30 (4/0/26) | 15 | admitted (open_band_structural) | ok | ok | yes | **YES** |
| control_reproposal_step13 | 7 | — (identified) | — | rejected (outside_exact_kappa_band) | ok | fail | no | no |
| control_modal_rebuild_step10 | 4 | — (identified) | — | admitted (open_band_structural) | ok | ok | no | no |
| control_window_rereference | 2 | 0 | 0 | rejected (trivially_derivable) | ok | ok | no | no |
| control_bare_temporal_pair | 2 | 2 (2/0/0) | 1 | admitted (open_band_structural) | ok | ok | no | no |

**Verification of the numbers.** All by the published class formulas, checked by hand: Map ν_C = 2κ + r² (κ=2, r=6: 4+36 = 40 ✓; κ=4, r=12: 8+144 = 152 ✓); Axiomatic ν_C = ν(L_max) + κ + (r−1) with ν(L₁₅) = 103 (κ=3, r=2: 103+3+1 = 107, +ν_G 1 = 108 ✓); Hit ν_G = 1+3, ν_H = 1+d² (d=4: 4+17 = 21 ✓). Clearing thresholds: ν ≥ 19 (κ=2), 28 (κ=3), 37 (κ=4).

**Controls behaved.** Verbatim re-proposals were caught by canonical identification (ν ≡ 0, never formula-scored); the pure re-reference was rejected as trivially derivable; the κ=7 re-proposal fell outside the exact κ band; the bare temporal pair was admitted but scored ν = 2, far under the bar. The probe's machinery is sound; the survivors are not artifacts of a broken harness.

## 4. What is established, what is pending

**Established.** (i) The per-candidate gate stack of the current lane admits and clears the seven candidates above at step 16. (ii) The reason is structural: no post-15 debt ⇒ no focus family ⇒ the open band. Focus gating — not the bar — is what kept such shapes out of steps 4–15; Genesis 1–15 is unaffected by this finding. (iii) The internal case of the squeeze holds under the engine's identification (canonical presentation + trivial derivability). (iv) The halt claim is *not* carried by arithmetic: the evaluator's own P5/P-axiomatic formulas exceed the bar comfortably on the open band.

**Pending.** (i) The **minimality screen** — the probe does not run the engine's semantic-minimality witness. Analysis: the κ=2 weave's only amputation is a κ=1 telescope outside the exact band, so minimality likely does not remove it; unconfirmed in-engine. (ii) **Full-run acceptance** — the letter of falsifier (a) speaks of the lane sealing a sixteenth extension; the full run is OOM-blocked (§2). At per-candidate level every acceptance precondition tested is met.

## 5. The falsifier, as registered

`note_canonical_course.md` §8(a): *"T1 failing — an admissible sixteenth Genesis extension that clears Bar₁₆ — kills the squeeze reading and with it the unification's anchor (and contradicts ch_genesis_math's halt section: a book-level crisis, not just this note's)."*

The per-candidate condition of that sentence is now met, sevenfold, subject to §4's two pending items.

## 6. Two readings — adjudication open

**Reading A (falsifier as registered).** The engine, asked the registered question in the registered lane, admits and clears sixteenth extensions. Falsifier (a) trips; the squeeze reading dies; the halt section of ch_genesis_math is contradicted by its own engine; the consequences cascade per the note's own words. On this reading the honest response is the same as the H16 burn: record it, update the book, no retuning.

**Reading B (the identification gap).** The surviving weaves are Π/Σ-types over already-sealed entries. Semantically, univalence arguably makes such constructions *internal* — function types over existing structure are "already there," ν = 0 — which is precisely the book's internal case. But the engine's implementation of internality is only canonical-presentation-deep (Var-renaming, ordering); it cannot see that a weave is a definable construction over the library, so the P5 r² formula scores it as new schemas. On this reading the probe has not refuted the halt claim; it has shown the claim is **unanchored**: the evaluator lacks the layer of univalent identification the claim's proof sketch assumes. This is not a new discovery about the evaluator — it is exactly the V1 "direct-counter ground truth" gap pre-registered in `EVALUATOR_DERIVATION.md` §6 as the evaluator's central unclosed exposure.

**Discipline note, binding on both readings.** Reading B is only available if it is *earned*: the internality gate or direct counter must be derived from the definitions (D1–D3, the depth-2 window) and validated against Genesis 1–15 — where lib-referencing constructions scored ν > 0 and *must continue to* — without consulting this table as a target. Adopted any other way, Reading B is retuning wearing a theory costume, and the same rules that burned H16 forbid it. Until that work exists, the accurate status of the halt claim is: **engine verification attempted; per-candidate falsifier condition met; claim either falsified (A) or unanchored pending an honestly-derived identification layer (B).**

## 7. Actions

1. Commit the probe artifact as-is: `cargo run -p pen-search --release --example genesis_halting -- --out docs/halting_verification.json`.
2. Owner's adjudication of §6, recorded in `note_canonical_course.md` (and, if Reading A, in ch_genesis_math's halt section and ch16's register note per the falsifier's own scope).
3. Optional confirmation available on request: wire the engine's minimality witness into the probe to close pending item §4(i).
4. If Reading B is pursued: the V1 direct counter / internality gate becomes the program's next engine obligation, specified blind to this table, with Genesis 1–15 conservativity as its regression gate.

One structural observation belongs in whatever record survives: the sequence's fifteen steps were protected from the open band by their own unpaid debts — each step's focus family was its guard rail. Step 16 is the first step with nothing owed, and the first thing the open band did was clear the bar. Whether that is a bug in the engine's notion of novelty or a refutation of the halt is exactly what §6 leaves to the owner.

---

## 8. Addendum (2026-07-06): the Guard-Rail Theorem, the executed check, and the adjudication criterion

The closing observation of §7 has since been formalized (`book/debt_guard_theorem.pdf`): Theorem 7 (obligation locality — discharge persists, O(n+1) is window-generated, the selection dynamics is a guarded recursion with directive debt as its guard), Theorem 12 (debt exhaustion — because Step 15 is the free, law-like completion, O(16) = ∅ and the open band is forced), Corollary 16 (the halt section's pricing mechanism never governed — unconditional under either reading of §6), and Theorem 17 (the relocated halt criterion). This addendum executes that note's §8 action list against this record. (Provenance note: this file was found zeroed on disk on 2026-07-06 — a Windows write corruption committed as `dbe9dd1` — and was restored from `7f12187` before this addendum was appended; if `dbe9dd1` was meant to carry intentional edits to this file, they predate the restore and are lost.)

**Action 1 — executed.** The O(16)-emptiness check is implemented (`crates/pen-eval/src/debt_guard.rs`, artifact `docs/o16_emptiness.json`) and passed on 2026-07-06: the directive-debt timeline shows exactly one standing demand per structural stage — former_eliminator (4), initial_hit (5), truncation_hit (6), higher_hit (7), sphere_lift (8), axiomatic_bundle (9), modal_shell (10), connection_shell (11), curvature_shell (12), operator_bundle (13), hilbert_functional (14), temporal_shell (15) — each guard created by the previous step's product and discharged by the step that answered it, all demand-intervals contiguous (Theorem 7's persistence face), and **O(16) = ∅, the first debt-free stage**. One wrinkle recorded: the former_eliminator demand already stands at stage 3, one stage before the lane's structural admissibility begins consulting the debt — demand precedes jurisdiction by one stage; harmless to the theorem. **Granularity disclosure:** the check verifies debt exhaustion at the granularity the engine implements — the focus-family generator, which the theorem note's A3 names as the demand schema's engine face. Instance-level enumeration of the (S₁₅, S₁₄) window's demands, with per-instance derivability verification, remains the finer refinement; it is also the natural instrument for hunting falsifier F1 (a demanded-but-underdetermined instance would refute J2/J3 as used and restore focus gating at 16, rescuing the halt by the old mechanism).

**Action 2 — executed.** Corollary 16 is recorded where the claim lives: the halt section of `ch_genesis_mathematics.tex` now carries a dated postscript stating, unconditionally, that the section's mechanism is superseded — throughout Genesis the guards excluded the formula-maximizers, pricing never did, and at the one stage where pricing must act alone the shipped valuation does not. If the halt is true, it is true for a different reason than the section gave.

**Action 3 — this entry.** Theorem 17 is hereby folded into this adjudication record as **the criterion the blind V1/L1/L2 work will be evaluated against**: the sequence halts at 15 iff sup ρ < Bar₁₆ = 354333/39040 on the open band. Sufficient: an extraction-guarded valuation — (V-int) ν = 0 for kernels derivable over B₁₅ up to univalent equivalence, plus (V-opq) ν ≤ ακ + β for opaque candidates on a debt-free field — giving sup ρ ≤ α + β/2 in the lane band, halting iff that bound is under 9.077. Necessary, by contraposition: any honestly-derived valuation sustaining superlinear open-band credit (r² on unbounded interface count, 1 + d² at fixed κ, telescopic inheritance on mere reference) continues the sequence at the smallest-margin honest survivor. **The L1 asymmetry, stated so the adjudication cannot quietly reduce to the weave case:** V-int at best removes the *definable* survivors — the Π/Σ-weaves and the axiomatic-inheritance bundles. The higher-cell candidates (hit_path_d4 at ρ = 21/2, hit_path_d5 at ρ = 15) are external to any univalent identification; for them the halt stands or falls with L1's validity domain alone — specifically, whether ν_H-credit at dimension d presupposes library filling capacity at d. A blind L1 derivation that grants dimension-credit without a capacity precondition decides Theorem 17(3) against the halt regardless of how internality treats the weaves.

**Action 4 — standing.** The replacement narrative ("runtime begins where debt ends") remains conditional on Theorem 17(2) and is quoted, everywhere it appears, with its condition.

**Effect on §4's pending items.** The relocation changes what is load-bearing. The full-run acceptance confirmation (§4 ii) is no longer the decisive question — Theorem 17 reduces the adjudication from an engine-competition fact to a valuation question, so the OOM-blocked step-16 search stays blocked without blocking anything. The minimality screen (§4 i) is likewise demoted to hygiene. What now decides the halt, exhaustively: the blind derivations of V-int/V-opq (V1) and of L1/L2's validity domains, produced by a context that has never seen this document's §3 table, evaluated against Theorem 17, with Genesis 1–15 conservativity as the regression gate.
