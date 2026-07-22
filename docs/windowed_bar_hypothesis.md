# WB-1: The Two-Step Memory Bar (overdetermination hypothesis)

**Registered hypothesis, frozen for engine testing.**

**Date:** 2026-07-21. **Status:** frozen before any WB engine output
exists. Naming (`WB-1`) is provisional. This document registers three
separable claims about the windowed bar, their discriminating
experiments, and burn semantics, before any test runs. Nothing here is
law: the windowed bar is and remains a *descriptive* quantity under the
Corollary-14 price/guard split, and per the standing verdict-blindness
rule its agreement with the certified world is not now and can never
become a ground for adopting it.

**Firewall.** This document contains certified scores, winners, and
clear/fail outcomes. It is an admissible input to replay-harness
construction (W-T1, W-T3) and to the comparison protocol, but a
**forbidden input** to any blind derivation, any valuation law, and any
selection code path. The standing bar-value ban is unchanged and extends
to Bar̂ₙ: no windowed value may appear in any derivation, admissibility
gate, or law-level selection. WB-1's tests are replay-only computations
over sealed artifacts.

---

## 0. One-paragraph statement

The certified Genesis sequence appears to be overdetermined: it is
forced by the adopted debt law (reselection v3 — every guarded stage has
exactly one discharger), and it *also* clears, at every step where the
quantity is defined, a bar with no golden ratio and no legislation in it
— the trailing two-step novelty density Bar̂ₙ = (νₙ₋₁+νₙ₋₂)/(κₙ₋₁+κₙ₋₂).
At step 16 this windowed bar stands at 165/17 ≈ 9.706 against the
certified debt-free envelope ρ ≤ 4: insurmountable by a factor of ~2.4.
WB-1 hypothesizes that this is not coincidence but a single deep
parameter appearing twice: **the theory's memory depth is 2**. The
demand computation C(Sₙ, Sₙ₋₁) reads the last two stages; the
quantitative debt Δₙ = Fₙ is two-step memory written as a recursion
(Fₙ = Fₙ₋₁ + Fₙ₋₂); the golden ratio of the legacy bar is not an
imported constant but the fixed point of a width-2 window. If WB-1
holds, the generative and selective laws are two projections of one
width-2 structure, and the halt at 16 is one event seen from two sides:
O(16) = ∅ (demand side) and trailing-density unattainability (value
side).

## 1. Fixed premises (not under test)

- **P1.** The certified v3 semantic vector ν = [1, 1, 2, 5, 7, 8, 10,
  17, 17, 19, 26, 34, 46, 62, 103] with κ = [2, 1, 1, 3, 3, 3, 3, 5, 4,
  4, 5, 6, 7, 9, 8] (imported v2 prefix + v3 guarded census,
  `docs/phase5b_reselection_burn_v3.json`). Σν = 358, Σκ = 54.
- **P2.** Option B two-register gating as adopted: debt gates guarded
  stages; certified value gates the open band; the bar (any bar) gates
  nothing.
- **P3.** The EGP envelope: certified ν ≤ 4κ on debt-free fields, hence
  ρ ≤ 4 on the open band.
- **P4.** Theorem 12 / O(16) = ∅ as engine-verified; the bar-free
  proposal's status (locked pending T-BF1..T-BF3) is unchanged by
  anything in this document.
- **P5 (definition).** Bar̂ₙ = (νₙ₋₁ + νₙ₋₂)/(κₙ₋₁ + κₙ₋₂), defined for
  n ≥ 3. "Clears" means ρₙ = νₙ/κₙ ≥ Bar̂ₙ. The Φ-dressed variant
  Bar̂ₙ·Φₙ (Φₙ = Fₙ/Fₙ₋₁) is carried as a diagnostic only.

## 2. Hypothesis clauses (separable; each burns independently)

- **WB-1a (shadow claim).** Every certified winner clears Bar̂ₙ for all
  n in 3..15. Verified 2026-07-21 by exact-rational chat-level
  computation on P1; not yet a replay artifact. WB-1a asserts the
  computation survives create-new replay with mutation falsifiers.
- **WB-1b (co-determination claim — the crux).** The windowed bar is
  not merely a trace but an independent selector: at each stage with a
  nontrivial raw candidate surface, selection by Bar̂-clearing alone
  (min-overshoot above Bar̂ₙ, no demand law consulted) picks the
  certified winner uniquely. Note the guarded cones of v3 are
  singletons, so WB-1b is *vacuous on the admitted cones* and must be
  tested on the raw enumerated surface, where the debt law demonstrably
  did exclusion work (the sealed Step-8 bare-S³ exclusion is the
  canonical instance). If the window cannot replicate that exclusion,
  WB-1b is false and the window is shadow only.
- **WB-1c (common-root claim).** The overdetermination, whether shadow
  or co-determination, has a single source: the width-2 window of the
  generative law. Consequences claimed: (i) Fibonacci debt is the
  unique quantitative shadow of a width-2 demand computation; (ii) φ in
  the legacy bar is the window's fixed point, not physics imported from
  outside; (iii) the Φ-dressed windowed bar fails at steps 4–14 (chat
  computation, same date) because applying the golden ratio *to a
  quantity that already has two-step memory* counts the window twice —
  the legacy cumulative bar needed Φ only because Ω had smoothed the
  window away. WB-1c is a derivation target (paper-level theorem), not
  an engine claim.
- **WB-1d (halt-equivalence claim).** On a width-2 demand system,
  debt-freedom and trailing-density unattainability are the same halt:
  O(n+1) = ∅ ⟺ sup ρ on the debt-free surface < Bar̂ₙ₊₁. This is the
  strong form of T-BF2. At n+1 = 16 both sides are already separately
  certified facts (O(16) = ∅; 4 < 165/17); WB-1d claims the
  implication, not the instance.

## 3. Computed reference table (comparison target for W-T1)

Computed 2026-07-21 by exact-rational iteration on P1. Any replay
divergence burns WB-1a.

| n | Bar̂ₙ | ρₙ | clears | margin | Bar̂ₙ·Φₙ | clears (Φ) |
| ---: | ---: | ---: | :---: | ---: | ---: | :---: |
| 3 | 2/3 | 2 | yes | 4/3 | 4/3 | yes |
| 4 | 3/2 | 5/3 | yes | **1/6** | 9/4 | no |
| 5 | 7/4 | 7/3 | yes | 7/12 | 35/12 | no |
| 6 | 2 | 8/3 | yes | 2/3 | 16/5 | no |
| 7 | 5/2 | 10/3 | yes | 5/6 | 65/16 | no |
| 8 | 3 | 17/5 | yes | 2/5 | 63/13 | no |
| 9 | 27/8 | 17/4 | yes | 7/8 | 153/28 | no |
| 10 | 34/9 | 19/4 | yes | 35/36 | 55/9 | no |
| 11 | 9/2 | 26/5 | yes | 7/10 | 801/110 | no |
| 12 | 5 | 17/3 | yes | 2/3 | 720/89 | no |
| 13 | 60/11 | 46/7 | yes | 86/77 | 1165/132 | no |
| 14 | 80/13 | 62/9 | yes | 86/117 | 2320/233 | no |
| 15 | 27/4 | 103/8 | yes | 49/8 | 4119/377 | yes |
| 16 | **165/17** | ≤ 4 (envelope) | **no** | −97/17 | — | no |

Replication diagnostics: the two tight margins are **1/6 at step 4**
and **2/5 at step 8** — any ledger-model discrepancy shows there first.
Recorded without interpretation: the tightest margin of the ascent
falls at the stage that carries the four-way parsimony tie. Jurisdiction
note: Bar̂ is undefined at n ≤ 2; the bootstrap prefix falls outside the
window exactly as it falls outside the demand chain (the bar-free
proposal's constitutive clause) — same boundary, third appearance.

## 4. Engine tasks (replay-only; independent of E-5 and R-T2)

- **W-T1 (shadow replay).** Create-new artifact recomputing §3 from the
  sealed v3 burn by exact rationals. Mutation falsifiers: perturbing any
  ν, κ, or clear/fail flag must invalidate. No new semantics.
- **W-T2 (selectivity probe).** For each stage with a nontrivial raw
  enumerated cone, apply Bar̂-selection alone (admit = clears Bar̂ₙ;
  select = min overshoot) and record: winner match / winner divergence /
  non-unique / empty. The candidate surfaces must be the sealed
  enumeration cones, reconstructed without reference to winners
  (contamination voids the probe, F-WB3). The Step-8 bare-S³ instance is
  the pre-registered decisive case.
- **W-T3 (foreclosure instance).** Certificate-level statement that
  Bar̂₁₆ = 165/17 exceeds the certified open-band envelope ρ ≤ 4, with
  both inputs bound by digest to their sealed sources. This is an
  instance record for WB-1d, not a proof of it.
- **W-T4 (deferred, paper-level).** The common-root derivation: width-2
  demand window ⇒ Fibonacci quantitative debt ⇒ φ as growth fixed
  point ⇒ the legacy bar as the cumulative dress of Bar̂. Registered as
  a target; no engine work; not gating anything.

## 5. Pre-registered outcome grading

- **Z1 (overdetermination real).** W-T1 replays and W-T2 reports winner
  match, unique, at every nontrivial stage including Step 8 → the
  window is a co-determinant; WB-1b stands; the common-root theorem
  (W-T4) is promoted to the principal open derivation of the bar-free
  program.
- **Z2 (shadow only).** W-T1 replays but W-T2 shows the window fails to
  exclude some candidate the debt law excluded, or selects non-uniquely
  → WB-1b burns; WB-1a stands as a descriptive theorem ("the ascent
  never decelerated"); WB-1c/WB-1d remain live as derivation targets
  with weakened support.
- **Z3 (winner divergence).** W-T2 selects a *different* winner at some
  stage → WB-1b burns hard: the window is not even extensionally
  faithful where it has choices; recorded verbatim.
- **Z4 (replay failure).** W-T1 diverges from §3 → WB-1a false; the
  entire hypothesis burns; the table above is retained as the burn
  record.

## 6. Falsifiers

- **F-WB1.** W-T1 replay divergence from §3 → WB-1a burned. The
  hypothesis revises only through a versioned successor; the certified
  history never does.
- **F-WB2 (standing rule, restated).** Any recorded ground for adopting
  any law, rule, or gate that cites the windowed bar's agreement with
  the certified world → invalid on its face (F-BF4/F-S8-2 pattern).
  WB-1's own confirmation, in any zone, is not an adoption ground.
- **F-WB3.** Any W-T2 candidate surface constructed with knowledge of
  winners, or filtered by any adopted law before Bar̂-selection → probe
  void; no verdict.
- **F-WB4.** Bar̂ appearing in any law-level code path, gate, or
  derivation → invalid on its face; the descriptive status is not a
  default but a rule.
- **F-WB5 (no retuning).** If WB-1 burns in any zone, the window width
  is not a free parameter: no width-3, no weighted-window, no smoothing
  variant may be registered as a successor to rescue the claim. A
  successor must derive its window from the demand structure or not
  exist.

## 7. Relation to the standing program

WB-1 gates nothing and is gated by nothing: E-5, R-T2, T-BF1..3, and
the bridge proceed unchanged. If Z1 or Z2 lands, the natural successor
is a strengthened T-BF2 stating WB-1d on the certified domain. The
bar-free adoption block remains locked on its own prerequisites
regardless of WB-1's fate.
