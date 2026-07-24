# BI-4 v2 — Cone assembly: preregistration

**Date:** 2026-07-23. **Status:** brief frozen before any BI-4 v2
engine output exists. Successor to the obsolete BI-4 issuer for this
lineage: the v2 issuer consumes the BI-2 v2 aggregate
(`blake3:cfa5bbd2…1fc03ff`, `ready_for_bi4_assembly = true`) directly.
**The only licensed change from the original BI-4 preregistration is
input lineage.** The comparison semantics, granularity ladder, and
zone taxonomy of the BI-1 plan are retained verbatim; any weakening,
redefinition, or reordering of them in the issuer is invalid on its
face (F-B4-1).

**Epistemic state at freeze (disclosed, not steering).** Known: all
four branches hold local-G4 halts (89/89, O(16) empty, F1 excluded,
E-5 complete); the correspondence diagnostic shows byte-identical
winners at stages 8–15; the Stage-4 semantic ν split is 2, 2, 3, 3.
Current data therefore *suggests* G2 and cone-G4 pass while G3 may
refute through its cumulative clauses, since the Stage-4 offset
propagates through Σν and every subsequent diagnostic bar. This
expectation is recorded as bias disclosure, not prediction; the
registered semantics below are what speak.

## Inputs and lineage

- The four BI-2 v2 branch certificates, by digest, each replayed;
  lineage verified back through the BI-1b sweep
  (`blake3:79272aa3…`), BI-0 v6, and the Stage-4 v3 cross-binding.
- The obsolete BI-4 v1 issuer is retired as testimony, not deleted;
  the known BI-1b post-seal replay drift remains frozen testimony,
  disclosed in the assembly artifact, never repaired there.
- The correspondence diagnostic ledger, opened read-only only after
  all comparison verdicts are computed from the certificates
  themselves (it corroborates; it never substitutes).

## Comparisons (registered semantics, reported fine-grained)

- **G2 — obligation profile.** Per-stage live-demand structure across
  the four branches, branch point to halt: same demands, total
  discharge, O-ladder shape. Verdict: pass/refute per the original
  definition.
- **G3 — numeric ledger.** The original registered content, reported
  as three sub-measurements and one composite: **G3a** per-stage
  (κ, semantic ν) equality at stages 5–15; **G3b** Σν equality;
  **G3c** diagnostic bar-trajectory equality (the bar retained as a
  descriptive column throughout — Corollary-14 split, gating
  nothing). **The composite G3 verdict issues per the original
  wording:** it passes only if the ledger vectors are equal as the
  BI-1 plan stated. Sub-reporting is transparency, not a new
  standard; a composite refutation with sub-passes is published
  exactly so.
- **Cone-G4 — halt.** All four branches debt-free at 15 with F1
  excluded, verified from the four certificates plus assembly
  consistency (identical demand at the halt boundary, no branch
  index leaking into any G4 premise).

## Zone assignment and refinement labels

The original taxonomy governs: **Z-CONE** (G2 and G3 pass),
**Z-ISO** (G3 pass, G2 diverge), **Z-SPLIT** (different halt or
different ledger), **Z-TREE**, **Z-STOP**. If the outcome is a G3
refutation whose entire content is the Stage-4 offset propagating
through cumulative quantities — with G3a passing at every stage —
the assignment per the registered letter is Z-SPLIT, and the artifact
additionally records the refinement label **Z-SPLIT-4** ("ledger
difference confined to the Stage-4 offset"). The label is a recorded
description of *where* the difference lives; it does not soften the
composite verdict and confers no promotion that Z-SPLIT would deny.

## Branch-index disposition (F-R3-B1 discharge rules)

- Cone-G4 pass → the halt claim, O(16) = ∅, and F1 exclusion rise to
  **cone-level**: theorems of the laws, not of the choice.
- G2 pass → obligation-structure claims (O-ladder shape, demand
  chain) rise to cone-level.
- G3 sub-verdicts promote exactly what they prove: a G3a pass makes
  per-stage ledger claims (5–15) cone-level; a G3b/G3c refutation
  keeps every *cumulative* quantity (Σν, bar trajectories, WB-1
  tables) **branch-indexed permanently**, recorded as descriptive
  interface physics that differs lawfully across the cone.
- Whatever remains indexed is listed explicitly in the cone report;
  silence is not disposition.

## Falsifiers

- **F-B4-1.** Any change to comparison semantics, granularity
  definitions, or zone taxonomy beyond input lineage → invalid; the
  registered letter governs, including composite G3.
- **F-B4-2.** Any issuer behavior conditioned on the disclosed
  expectation (e.g., ordering comparisons to stop early at a desired
  verdict) → invalid; all comparisons compute and publish.
- **F-B4-3.** UC-1 scored, cited, or partially assessed in this
  artifact → void; UC-1 has its own artifact, still gated on the
  exported typed cross-package transport maps that the invariance
  theorem honestly declined to claim.
- **F-B4-4.** Any claim promoted to cone-level without its exact
  verdict premise, or any branch index silently dropped → invalid.
- **F-B4-5.** Any drift repaired, concealed, or re-derived inside
  assembly → invalid; drifts are disclosed testimony.

## Deliverables

`docs/BI4_CONE_REPORT_V2.md` + certificate with mutation falsifiers:
per-comparison verdicts (G2, G3a/b/c + composite, cone-G4), zone
assignment with any refinement label, the full branch-index
disposition table, and the drift disclosures. No bridge, no final
certificate, no UC-1 content.

## Relation to the standing program

After BI-4 v2: UC-1's scoring artifact becomes buildable once the
transport maps export (the one remaining F-UC4 gate — a small,
self-contained successor to the invariance theorem); the successor
selective-law consolidation and the bridge documents take their
quantifiers from the cone report's disposition table. WB-1 remains
parked, non-gating, its tables now candidates for per-branch
recomputation as descriptive diagnostics only.
