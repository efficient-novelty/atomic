# XF-0b2 — Provenance finding (v2): the sealed stage-14 telescope is a hand-authored literal that predates the search

**Date:** 2026-07-24. **Status:** supersedes `docs/xf0b2_provenance_finding_v1.md`
(retained unedited, superseded banner applied, because the adversarial verifier
audited that text). **Origin:** raised by independent armed analyst B; verified
and extended by the coordinator; **corrected and completed by the adversarial
verifier**, which settled the causal direction v1 left open and refuted or
qualified four of v1's sub-claims. Every chronological claim below was
re-verified independently by the coordinator against `git log`.

**Bearing:** decisive against the export. The naming verdict *survives* it, but
is not untouched by it — v1 overstated that, and the correction is recorded in §4.

---

## 1. The core finding (confirmed)

The subject clause's exact shape, `Pi(Lam(Var 1), Sigma(Var 1, Var 2))`, is a
**hard-coded literal** in three places, and the per-position enumeration gate
`supports_hilbert_functional_clause_at_position`
(`crates/pen-search/src/enumerate.rs`) admits **exactly one expression at
position 3** — every leaf a concrete literal. The gate is on the live
enumeration path. Confirmed by the verifier against the sources.

**Stronger than v1 reported.** The verifier found that the gate is not merely a
filter: `late_clause_options` (`enumerate.rs`) intercepts `clause_kappa == 9`
with `max_expr_nodes >= 7` and **bypasses raw enumeration entirely**, returning
the nine-position hard-coded table `hilbert_functional_reference_clause`.
`max_expr_nodes_for_mode` yields exactly 7 for a HilbertFunctional focus, so the
arm fires at stage 14. The whole nine-clause telescope is **emitted verbatim
from a literal table**, independently of the `require_hilbert_functional_clauses`
flag.

## 2. The causal direction — settled, and both of v1's readings were wrong

v1 left open two readings: a post-hoc replay recognizer, or an a priori gate.
**Neither is correct.** Verified independently:

| Artifact | First appears | Commit |
|---|---|---|
| `Telescope::reference(14)`, clause 3 included | **2026-03-13** | `5dd8447` "First rust implementation step" |
| `matches_hilbert_functional_shell` (`pen-core/library.rs`) | 2026-03-14 | `698935a` |
| `supports_hilbert_functional_clause_at_position` (`pen-search/enumerate.rs`) | 2026-03-15 | `64b1c21` |

`git log -L 335,363:crates/pen-core/src/telescope.rs` returns **exactly one
commit**. The stage-14 telescope is byte-identical in that first commit and has
**never been modified since**.

So the sealed literal **predates every piece of stage-14 search machinery in
this repository by one to two days.** Reading 1 (post-hoc recognizer written
after a search found the winner) is refuted: no run in this repository ever
found it. Reading 2 (a priori gate constraining a run) is refuted: the gate
postdates the literal, so it constrained nothing.

**The true reading:** `Telescope::reference` is a **hand-transcribed design
target**, and the search's stage-14 arms were written afterwards to reproduce it.

Two independent confirmations, both re-verified:

1. **The certified register is built by reading the literals.**
   `crates/pen-search/src/t_bi_nu1_regression_v6.rs`, `fn reference_entries`, is
   `(1..=15).map(|stage| (stage, Telescope::reference(stage)))`. The v6
   provenance JSON — the C-1 extraction source of the whole XF-0b concordance —
   is produced from that, not from a search run.
2. **The target sequence was pre-declared.**
   `skills/pen-atomic/references/02-target-sequence.md`, line 47, carries
   `| 14 | Hilbert | 377 | 9 | 62 | 6.89 |` — step 14, "Hilbert", κ = 9 — and
   that file lands in commit `4adcf28` **"Initial commit"**, the repository's
   first content commit, hours before the Rust literal.

**What is NOT settled, stated as a limit:** whether a prior engine (the Haskell
`RunAbInitio.hs` referenced by the same brief) ever searched for and found the
clause-3 expression. That would need that repository's history. What *is*
settled is that **no search in this repository did.**

## 3. Corrections to v1 (from the verifier's audit)

| v1 claim | Verdict | Correction |
|---|---|---|
| The position-3 gate is a syntactic singleton | **confirmed** | — |
| The gate is on the live enumeration path | **confirmed** | — |
| Stage 14 raises the flag by construction | **overstated** | The cited `demand_completeness.rs` `14 => Some("hilbert_functional")` is an *expectation table* for checking a certificate; it raises nothing. The real raiser is `StructuralDebt::requires_hilbert_functional_package` (`pen-type/obligations.rs`), which fires on library **content**, not a stage index. The conclusion holds in Guarded mode (the default); the justification cited was the wrong one. |
| Not specific to this family | **confirmed, and understated** | Positions 0–7 are equally literal, including both credited siblings the concordance *named*. `operator_bundle` and `temporal_shell` have identical architecture. |
| The export is defeated | **confirmed** | On stronger grounds than v1 had. |
| **"Early families are looser; pinning tightens with stage depth"** | **REFUTED** | `supports_initial_hit_clause_at_position` is position 0 = exactly `App(Univ, Var 1)`, position 1 = exactly `Var 1`, position ≥2 = exactly `PathCon(1)` — three literal singletons. The truncation and higher-HIT gates likewise. **The pinning is uniform from the earliest structural families onward.** |
| The four listed sites are all hard-coded literals | **one misdescribed** | `supports_hilbert_functional_clause` (position-agnostic) uses `Expr::Var(_)` **wildcards**, not literals. Analyst B said so; v1's summary did not. |
| Surface-independence | **omitted qualification** | The position-3 singleton holds on `LateFamilySurface::None` and `RealisticShadow`. On `DesktopClaimShadow` and `DemoBreadthShadow` the gate offers three alternatives each at position 3. Internally consistent with v1's conclusion (the require-flag is raised only in Guarded mode, where the surface is `None`), but the qualification belongs in the record. |
| ADDENDUM C verifies the decisive gate | **tests a different function** | The perturbation table reads `has_hilbert`, computed by `matches_hilbert_functional_shell` in `pen-core/library.rs` — the public capability flag — not the `enumerate.rs` per-position gate v1 calls decisive. The two carry the same literals under `LateFamilySurface::None`, so the demonstration is sound as far as it goes, but it does not exercise the gate it cites. |

## 4. Bearing on the naming verdict — v1 was too strong

v1 said the finding "does not touch the naming verdict … none of those grounds
is disturbed." **Corrected:** every one of the eight R-1/R-2 contexts uses
anti-poverty rebuttals of the form *"the record could have written Y at coarse 0
and declined to; therefore writing Z is significant content."* That inference
presupposes something was choosing. Once the record is known to be a
hand-transcribed target table authored before the search machinery existed,
those rebuttals lose evidential force.

They are **auxiliary everywhere and load-bearing nowhere** — every surviving
obstruction is of the form "defining datum D of the target has no counterpart in
the family's typed content", and under the translation standard a datum the
source does not carry cannot be mapped regardless of *why* it is missing. So:

> **The verdict survives. The grounds are disturbed.** The record must say the
> former, not "nothing is disturbed".

And one consequence v1 did not draw, which the verifier requires be reported:
once the clause is known to be hand-authored, **"authoring artifact" becomes a
live and arguably the best explanation of the shape**, converging with analyst
B's deflationary reading.

## 5. Consequences registered

1. **The export is withdrawn**, not deferred. Per F-R5 a withdrawn export is a
   success of the discipline. The ground is stronger than the naming question:
   the object's provenance does not support the claim the registration makes.
2. **`docs/df4feb52_specification_v1.md` is not written.** Its purpose is to
   specify a candidate novel object; writing it would assert the provenance this
   finding defeats. This note stands in its place.
3. **The methods question is much wider than v1 framed it.** Not "to what extent
   is the *late* stratum a transcription" but: **the whole certified fifteen-stage
   register is read from a hand-authored reference table, and every
   structural-family gate from stage 3 onward is a per-position literal.** This
   bears on XF-0b's headline (31 of 32 families named) more heavily than v1's
   consequence 3 allowed, and on any future v2 census.
4. **XF-0 is untouched.** It was blind and never read the register; its 3/11
   mismatch stands exactly as published.
