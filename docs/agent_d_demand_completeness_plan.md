# Agent D — DEMAND-COMPLETE: extractor vs intended demand schemas

> **Successor execution note (DEMAND-COMPLETE v1, 2026-07-19):** execution
> follows the D-4 gap branch. The coarse ladder replays exactly, including
> the stage-3 demand-before-jurisdiction wrinkle, but the current extractor
> mirrors the caller/focus timeline and cannot be identified with an
> independently generated `C(W)`. The missing historical instance grammar,
> orbit quotient, and `D`-membership decision are recorded as
> `D4_A3_CW_INSTANCE_GRAMMAR_ORBIT_QUOTIENT_D_MEMBERSHIP_UNDEFINED`. F1 and
> F5 are not triggered; semantic `O(16)` emptiness and the `d = 4`
> demand-orphan conclusion remain conditional. See
> `docs/DEMAND_COMPLETENESS_RESULT.md`. The original brief below is archival.

**Standing:** construction task with one live theoretical hazard (see
falsifier F1 below — the most consequential possible outcome of any of the
four agent tracks). Admissible context: the Guard-Rail theorem note
(A1–A4, A3's demand schema C(W), Theorem 12, falsifier F1),
`docs/o16_emptiness.json` and `pen-eval/src/debt_guard.rs`,
`docs/TDC1_CUBICAL_RESULT.md` (provenance diagnostic). Forbidden: the bar
value; any Step-16 candidate scoring.

## Mission

The TDC v3 provenance diagnostic — step 8 carried **one** extracted live
demanded output, the d = 4 candidate carries **zero** — is currently
conditional because no theorem identifies the kernel demand-orbit
extractor with the *intended* depth-two demand schemas (Guard-Rail A3).
Close that gap in either direction: prove the identification, or exhibit
and name the difference. Success upgrades (a) the O(16)=∅ check from
focus-generator granularity toward instance granularity, and (b) the
1-vs-0 live-output finding from hint to obstruction — or retires it.

## Owned modules

`pen-eval::{debt_guard, demand-orbit extractor}` and a new intended-schema
spec module. No changes to `pen-type` (Agents A/C) or scoring paths.

## Work items

1. **D-1 (spec).** Formalize the intended demand schema: for a window
   W = (S_n, S_{n−1}), the finite set C(W) of depth-two coherence demands
   per Guard-Rail A3 and the Appendix-D machinery, as a checkable
   generator — independent of the extractor's implementation.
2. **D-2 (identification or gap).** For every historical window (stages
   4–15), compute C(W) from D-1 and the extractor's output, and compare.
   The sealed O-ladder is the regression signature: the one-demand-per-
   stage ladder must reappear, **including the stage-3 wrinkle** (demand
   preceding jurisdiction by one stage). A clean reproduction that loses
   the wrinkle is a red flag, not a success.
3. **D-3 (if identified).** Re-emit the O(16) emptiness check at instance
   granularity over C(S₁₅, S₁₄) (this is Guard-Rail §8 Action 1 at its
   intended fineness, and F1's proper instrument). Re-emit the provenance
   diagnostic with the obstruction flag earned: the d = 4 attachment is
   certified demand-orphaned, or it is not.
4. **D-4 (if a gap).** Name the missing or excess schema family exactly;
   the provenance diagnostic stays conditional; the artifact records the
   gap and what would close it. No shrinking of C(W) to match the
   extractor.
5. **D-5 (artifact).** `docs/demand_completeness_v1.json` (create-new) +
   `docs/DEMAND_COMPLETENESS_RESULT.md`, replay + mutation battery
   (mutating any demand instance, ladder entry, or completeness flag must
   invalidate).

## Falsifiers / exit conditions

- **Extractor ≠ intended on the historical ladder:** fix the extractor,
  never the ladder record.
- **F1 (Guard-Rail) hazard — handle with maximum care:** if the
  instance-granular check finds a demanded-but-underdetermined instance in
  the (S₁₅, S₁₄) window, Theorem 12 is refuted as used, O(16) ≠ ∅, and
  focus gating is restored at Step 16 — the halt would be rescued by the
  *old* mechanism and the entire post-Guard-Rail program restructures.
  Report verbatim, immediately, with the instance exhibited; suppress
  nothing; adjudication is the user's.
- Nonterminating or non-finite C(W) for any window → Guard-Rail F5
  territory; report; locality (Theorem 7) survives, the framing weakens.

## Done

Identification proved (with D-3 artifacts) or gap named (with D-4 record);
historical ladder reproduced wrinkle-and-all; F1 status stated explicitly
either way; artifact replays.
