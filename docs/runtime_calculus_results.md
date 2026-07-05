# runtime_calculus_results.md — Stage-2 Blind Run: Grading and Honesty-Ledger Updates

**Date:** 2026-07-05. **Run:** `docs/runtime_calculus_run.json` (freeze/eval commit `3c32bdb`, single post-freeze execution, 24 cadence steps, exhaustive field rounds). **Semantics:** `RUNTIME_CALCULUS.md` §§1–3 with §8 resolutions, §9 schedule, §9.1 pins. **Status of this document:** grading per the pre-registered failure semantics, plus paste-ready honesty-ledger inserts for the three book notes. Nothing here adjusts, reinterprets, or proposes recovery of any missed registration.

---

## 1. The run, in four lines

- All three frozen variants (`bound_interface_minimal/weighted/persistent`, κ = 3/5/6): **crossing index = 2**. e₁ = 1; e_k = 0 for every k ≥ 2 — production is a single step-1 burst (1/9/11 sealings), after which R1 dedup leaves the exhaustive field permanently empty while the ledger's demand renews every step.
- Band at the crossing: demand splits 1 attachment / 0 echo (k ≤ 2 truncated window); serviced 0; **nothing seals; unsealed-full-export; exported share 1/1** — all depth-1.
- Stratum sealing ledgers squeezed pincer-style at sealing indices 2 / 10 / 12 (persistent: last clearance at ρ = 12 against Bar = 1157/100; the next faced ≈ 12.28 — unpayable).
- **Validity:** this is a valid frozen-semantics run, not a v1-style invalid one — candidates live (ν = 7/10/12 at seal 1), every Stage-1 gate green, no dial available for adjustment (§0, §1: "If a run produces an unwanted crossing, none of these admit adjustment").

## 2. Grading against the §6 registrations

| Registration | Predicted | Computed | Verdict |
| --- | --- | --- | --- |
| B1 (H16) | crossing = 16 | crossing = 2 (×3 variants) | **REAL miss — H16 burns per its original registration** |
| B2 (ε) | export share Δ_{k−2}/Δ_k = 377/987 at k = 16 | share 1/1 at k = 2; echo share 0 | miss |
| B3 (band) | depth-2, exclusively or dominantly | depth-1, unsealed full export | miss (opposite composition) |
| B4 (C-COURSE) | ~15 clearances, Bar₁₆^(s) ≈ φ·Ω₁₅^(s) | 1 / 9 / 11 clearances, squeeze at sealing 2 / 10 / 12 | miss; C-COURSE falsifier (b) trips |

Two findings the registrations did not anticipate, recorded as facts (not mitigation): **(F1, two clocks)** the cadence ledger (e_k steps) and the stratum's sealing ledger (clearances) are distinct clocks in the frozen calculus; the Genesis-style bar-failure pincer *did* operate, on the sealing ledger, at indices the registrations never named. **(F2, no carryover)** under R1, a fixed-arity schema's production is spent in one burst — the σ-image discount leaves nothing for later cadence steps, while Δ_k renews every step. Any fixed-arity stratum is therefore a one-burst producer under these semantics.

---

## 3. Paste-ready insert — `note_lambda_trigger_lemma.md`

Append to the **Engine outcome** block (after the 2026-07-04 entries):

> **Engine outcome 2026-07-05 (frozen runtime calculus — REAL miss; H16 burnt).** The missing layer named on 2026-07-04 was designed (RUNTIME_CALCULUS.md, principle-pinned; reviewer dials resolved pre-freeze in its §8/§9.1), frozen by commit `3c32bdb`, and run blind exactly once (`docs/runtime_calculus_run.json`). Result: the engagement crossing (first k with e_k < 1) is **k = 2** for all three frozen bound-interface variants — not 16. Unlike the v1 run this one is *valid*: candidates were live (ν = 7/10/12), the field was searched exhaustively (no budget, no order dependence), and every conservativity/no-go gate stayed green. Per the pre-registered semantics of RUNTIME_CALCULUS.md §6 B1, this burns H16: **no retuning; the trigger reverts to observation-selected; the closed form of §6 refiles as retrodiction.** What §5 established remains exactly what it was — a uniqueness argument over the observation "late acceleration exists" — and no internal derivation now backs it. The §8 path-1 program ("bar-crossing computation") is closed with a miss in its first frozen form. Two facts from the run, recorded without interpretation: the calculus separates the cadence clock from the stratum's sealing clock (the registrations conflated them; the Genesis-style squeeze appeared on the sealing ledger at indices 2/10/12), and under the transport-image discount a fixed-arity schema produces in a single burst with zero step-to-step carryover. The observational falsifiers of this note (w(z) comb frequency; trigger window; Ω_Λ/Ω_m drift) are unchanged and now carry the entire load.

And in the **Honesty ledger**, amend the "Conjectured" line:

> - **Conjectured:** ~~that n\* = 16 has an internal derivation (§8)~~ — attempted under frozen runtime-calculus semantics 2026-07-05; **missed (crossing = 2); H16 burnt per registration**. n\* = 16 stands only as the unique observation-compatible integer of §5. ε = 1/φ² (L-Λ2 note; see its ledger for the engine outcome).

## 4. Paste-ready insert — `note_lambda_coefficient_lemma.md`

Append to §7 (**Honesty ledger**):

> - **Engine outcome (2026-07-05):** the runtime calculus (Premises A/B in engine form, RUNTIME_CALCULUS.md §§2–3) was frozen and run blind once (`docs/runtime_calculus_run.json`). The registered engine checks **missed**: the crossing landed at k = 2, where the standing export is 1/1 and entirely depth-1 (nothing seals) — not the depth-2 share, and not 377/987. Two precisions the adjudication requires: (i) at k = 2 the two-layer window is truncated (Δ_{k−2} does not yet exist), so the depth-2 reading had no arena at the computed crossing; (ii) the producer had zero serviced capacity at the crossing, so depth-1-priority servicing never operated. Consequently **falsifier (b) of this note is NOT tripped** — no formal result that admissible sealing fails to force depth-1 priority was produced — but the engine-bridge attempt is on record as a miss, and with H16 burnt (trigger note) the joint closed form is a retrodiction. ε = 1/φ² retains exactly its pre-registered status: a bridge argued from the d = 2 partition plus Premises A/B, adjudicated by the sky (§6), now with one failed engine-grounding attempt disclosed. Premises A and B remain open formalization targets; they may not be re-attempted against a calculus tuned toward this note's numbers.

## 5. Paste-ready insert — `note_canonical_course.md`

Append to §8 (**Falsifiers and corrections**):

> **Falsifier (b) tripped (2026-07-05).** T2 was executed: the runtime calculus was frozen (RUNTIME_CALCULUS.md, commit `3c32bdb`) and the stratum course computed blind (`docs/runtime_calculus_run.json`). The computed courses have **1, 9, and 11 clearances** — not 15 — with squeezes at sealing indices 2, 10, 12 and terminal bars ≈ ρ_max at each variant's arity ceiling (persistent: eleventh clearance at ρ = 12 vs bar 1157/100; twelfth demand ≈ 12.28, unpayable). Per this section's own registration, **C-COURSE is killed for the computed course and H16 returns to standalone status** (and is burnt there; see the trigger note). P3 falls with B2/B3. What the run *did* exhibit, for the record: the squeeze mechanism itself — fifteen was wrong, but the pincer (internal candidates deduped to ν = 0; external demands priced beyond the achievable ρ-menu) closed each course exactly as ch_genesis_math describes, on the stratum's own sealing ledger. The unification's mechanism survives its index prediction's death; C-COURSE as stated (grammar ⇒ *fifteen* clearances, squeeze at *sixteen*) does not. **T1 is untouched by this result** — the Genesis squeeze verification (`halting.rs`, still a stub) remains runnable and is now the program's only live engine anchor for the squeeze index. The register-table row "clearances: 15 (conjectured, H16)" should be marked falsified-as-computed; the ledger/register split of §2 gains an unanticipated refinement: cadence steps and clearances are *different clocks* (the registrations conflated them), and Δ/Ω/Bar arithmetic ran on both without contradiction.

---

## 6. What stands and what fell (program bookkeeping, no new claims)

**Fell with this run:** H16's internal-derivation claim (burnt, B1); the depth-2 export reading at the crossing (B2/B3, as registered in §6); C-COURSE as stated (falsifier b); P3 of the canonical-course note.

**Stands, per its own registrations, untouched by this run:** the L-Λ1 reduction and §5 uniqueness argument (observational, now load-bearing alone); the d = 2 partition theorem and Appendix D; Lemma A's cadence with its observational falsifier; ε's observational adjudication (§6 of the coefficient note); the coefficient note's falsifier (b), untripped; T1; the v2 no-gos, the Genesis conservativity gates, and the runtime calculus itself as a frozen, tested engine layer (Stages 3–4 of its §5 remain declared program items, defined independently of these outcomes).

**Discipline note.** The freeze held end to end: every dial was resolved or dissolved before commitment, the run happened once, and the numbers above were first computed by that run. This document records the miss; it does not propose a successor fit, and none of the three notes' inserts permit one.
