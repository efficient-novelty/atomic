# Agent A — HIST-CERT: certify the historical HIT totals (F-T1 discharge)

> **Successor execution note (HIST-CERT v3, 2026-07-19):** Agent A's
> create-new v3 run consumes the adopted element-overlay boundary theory. It
> preserves raw `7/8/10/18` and classifies the registered typed-and-marginal
> path as `2/2/5/10`, discharging F-B2. The ordinary `5/6/5/8` remainder is
> unresolved, independent novelty credit is not established, every full
> `certified_total` is `null`, and F-T1 remains open. See
> `docs/HIST_CERT_V3_RESULT.md`. The original brief below is archival.

**Standing:** construction task, not a blind derivation. Admissible context:
`docs/ip1_tdc1_plan.md`, `docs/TDC1_CUBICAL_RESULT.md`,
`docs/SH1_EXPERIMENT_RESULT.md`, the T4 frozen law, `pen-type::cubical` as
built in TDC v3. Forbidden in all code paths and derivations: the bar value
(Bar₁₆ or any Bar_n), any Step-16 candidate scoring.

## Mission

Discharge F-T1: take the historical HIT-class steps (5/S¹, 6/Trunc, 7/S²,
8/S³) from "presented basis matches recorded counts" to **certified ν**,
reproducing the recorded step totals `7, 8, 10, 18`. This is the first
semantic re-audit of any Genesis step. The machinery that succeeds here is
the machinery whose Step-16 verdict will later count.

## Scope

**In:**
- Univalent/definitional equality and weakening recognition for the
  historical path fragment (d ≤ 3), each step audited against its own
  predecessor library (B₄ for step 5, … B₇ for step 8) — not against B₁₅.
- Certification of the non-path family components of those steps
  (formation, eliminator, computation clauses) needed to reach the full
  recorded totals, not only the path-basis counts (2/2/5/10).
- EGP anchor assignment with distinctness proofs for every certified
  family.
- Certificate emission: per-step, per-family verdicts
  (marginal / weakening / undefined), with replay and mutation falsifiers.

**Out:**
- Any d = 4 or Step-16 computation. Emit no Step-16 numerics of any kind.
- Any change to the attachment boundary theory: consume
  `...constant-boundary-theory-axiom-v1` as-is and record the
  conditionality flag in the artifact (Agent B and the user's adjudication
  own that rule).
- Core `pen-type::{infer,normalize,equality,substitution}` rewrites
  (Agent C owns those). Extend `pen-type::cubical` and `pen-eval`
  tdc1 modules only.

## Work items

1. **H-1.** Per-step predecessor-library reconstruction: the sealed B₄…B₇
   contexts, replayed from the reference trace with digests recorded.
2. **H-2.** Equality/weakening layer for the historical fragment: decide,
   for each presented family, membership in the predecessor weakening image
   up to the fragment's definitional/univalent equality. Named obligations
   for any equality question the fragment cannot decide — an undecided
   family stays `undefined`, never defaults.
3. **H-3.** Non-path components: enumerate, normalize, classify to close
   the gap from path-basis counts to recorded totals.
4. **H-4.** Certificate `docs/hist_cert_v1.json` (create-new) +
   `docs/HIST_CERT_RESULT.md`. Mutation battery: flipping any verdict,
   anchor, equality proof, or conditionality flag must invalidate replay.

## Falsifiers / exit conditions

- **Certified total ≠ recorded total for any step:** fix the machinery,
  never the history. If after honest effort the machinery is believed
  correct and the mismatch stands, STOP and report verbatim — a historical
  mis-scoring is a book-level event and is the user's adjudication, not a
  bug to absorb.
- **Anchor collision** (two families, same anchor, both marginal):
  thinness failure; certificate invalid; report.
- **Boundary-rule dependence:** if any step's certification depends on the
  unadopted v1 boundary rule, the artifact must carry
  `conditional_on_boundary_axiom_v1 = true` for that step. Do not launder
  conditionality.

## Done

Every one of the four steps is either certified at its recorded total or
carries a named per-family obstruction; artifact replays; zero Step-16
content anywhere in the diff.
