# XF-0 Census Result — Zero-Worth / Degrees-of-Freedom

**Campaign:** XF-1 (`docs/xf1_external_falsifier_campaign.md`). **Date:**
2026-07-24. **Rule:** `docs/xf0_correspondence_rule_v1.md` /
`blake3:fd20618c86d5e5c532bc84f5fcbf0218e970473fbb322cc543964179e595814a`
(frozen before any row was filled; digest recorded in `docs/XF_LEDGER.md`
event 001). **Blind column:** `docs/xf0_physics_column_v1.json` (binds the
rule digest; filled by 48 isolated agent contexts — 15 analysts, 30
adversarial verifiers, adjudicators on dispute — none of which received the
certified register). **Join:** mechanical and replayable,
`cargo run -p pen-search --example xf0_census_v1 -- census docs`, output
`docs/xf0_census_v1.json`. The recorded order — Column S sealed in MS-1
before the campaign; rule frozen; column filled blind; join last — is
enforced by digest chain in the joiner.

## Headline (frozen tally partition, verbatim)

- **Blind single-target rows: 3/11 Match** (stages 3, 7, 11).
- **Stage 4 (blind dual-target, branch-indexed):** D(4) = 2 — **matches
  the cone-minimum value class (2); does not match the enacted branch
  (3).** The blind count landed inside the certified branch-value set.
- **Calibration rows: 3/3** consistent with the brief-frozen anchors
  (8 → 1, 9 → 0, 12 → 0) — and derived by the discipline, not merely
  quoted: the stage-9 analyst enumerated 14 defining data of the Hopf
  bundle and classed every one D/C/T/P; stage 12 produced 8 data, all
  zero-classes; stage 8 reproduced exactly the anchor's carrier-D /
  operation-N / coherence-P split.

**This is not a systematic match.** Under the frozen outcome semantics
(rule §7) the census is published verbatim as a mismatch pattern with
row-level localization, below.

## The verbatim table

| n | Stratum | Blind D(n) | Certified ν_sem | Verdict | Blind class-N data |
|---|---|---|---|---|---|
| 1 | Universe U₀ | 2 | 1 | Mismatch (+1) | universe carrier; elementhood/decoding |
| 2 | Unit type **1** | 1 | 0 | Mismatch (+1) | the unit former itself |
| 3 | Witness ⋆ : **1** | 1 | 1 | **Match** | the witness/constructor ⋆ |
| 4 | Π/Σ shell | 2 | branch-indexed {3 enacted, 2 cone-min} | **BranchIndexed: cone-min Match**, enacted Mismatch | Π-formation; Σ-formation |
| 5 | Circle S¹ | 1 | 6 | Mismatch (−5) | the loop constructor |
| 6 | Propositional truncation | 1 | 3 | Mismatch (−2) | the truncation former |
| 7 | Sphere S² | 1 | 1 | **Match** | the 2-cell generator surf |
| 8 | H-space S³ | 1 | 1 (calibration) | Calibration ✓ | the multiplication μ |
| 9 | Hopf fibration | 0 | 0 (calibration) | Calibration ✓ | — (14 data, all D/C/T/P) |
| 10 | Cohesion/modal shell | 3 | 4 | Mismatch (−1) | ♭; ♯; shape Π∞ |
| 11 | Connection shell | 1 | 1 | **Match** | the connection ∇ itself |
| 12 | Curvature shell | 0 | 0 (calibration) | Calibration ✓ | — (8 data, all C/P/D) |
| 13 | Metric/operator bundle | 1 | 2 | Mismatch (−1) | the metric g |
| 14 | Hilbert-functional shell | 1 | 3 | Mismatch (−2) | the inner product |
| 15 | Temporal-cohesive shell | 3 | 6 | Mismatch (−3) | Next; guarded fixpoint; Eventually |

Full enumerations (95 classified data across the fifteen rows), citations
per datum (HoTT Book, Martin-Löf, Kobayashi–Nomizu, Steenrod, Reed–Simon,
Schreiber, et al.), verifier reports, and adjudication records are
published verbatim in `docs/xf0_physics_column_v1.json` and
`docs/xf0_census_v1.json`. Adjudication was required on rows 1, 9, 15;
every other row survived both audits undisputed.

## Localization (what the mismatch pattern says, row by row)

1. **Sign structure is clean.** Every mismatch from stage 5 upward is an
   *undercount* (blind D < certified worth); the only overcounts (+1 each)
   are the two bootstrap rows 1–2. The crossover sits exactly at the
   bootstrap/geometric-ascent boundary.
2. **The sparse tail agrees exactly.** On every stratum with certified
   worth ≤ 1 except rows 1–2 (stages 3, 7, 8, 9, 11, 12), the blind
   independent-structure count reproduces the register exactly — including
   both certified zeros (9, 12), where blind analysts enumerated 14 and 8
   defining data respectively and independently classed every one as
   derived/determined/propositional/topological. The register's "pure
   discharge mints nothing" reading is corroborated by blind counting.
3. **Rich strata compress.** Everywhere certified worth is ≥ 2 (stages 4,
   5, 6, 10, 13, 14, 15), blind D is strictly smaller — except stage 4,
   where it equals the cone minimum. The frozen unit of counting (one
   minted idea per independent structure, granularity fixed by the
   anchors) merges what the act-local semantic-family audit distinguishes.
   The extreme case is the Circle: one loop constructor against certified
   worth 6.
4. **Stage 4 lands inside the cone.** The standard Π/Σ presentation counts
   its two formers as the only unimplied structures (introductions and
   eliminations classed as determined once the formers are counted) —
   exactly the non-enacted branches' value class 2, not the enacted 3.
5. **Bootstrap overcounts.** Rows 1–2 disagree in the opposite direction:
   blind counting refuses to see the Universe as one idea (carrier +
   elementhood/decoding both unimplied at an empty ladder) and refuses to
   see Unit as zero (the former itself unimplied). The audit's worth for
   the founding acts is not recoverable from standard-presentation
   independence counting under this rule.

## Consequence for the campaign (honest, per rule §7)

- XF-0 does **not** deliver the systematic match that would have
  calibrated later items. No later campaign item may lean on a claimed
  register ↔ degrees-of-freedom correspondence. The partial structure —
  exact agreement on the sparse tail including both certified zeros,
  systematic compression on rich strata, boundary overcounts at the
  bootstrap, and the stage-4 cone-minimum coincidence — is recorded as
  diagnostic localization, nothing more.
- Under the frozen rule, no row may now be re-filled, re-classified, or
  re-cited; this rule version is spent. Any successor census requires
  `xf0_correspondence_rule_v2.md` with a *different, principled* unit of
  counting preregistered before a fresh blind fill — and must disclose
  that v2 is designed with knowledge of this pattern, so its blindness is
  weaker than v1's: v2 can localize further, but only a rule frozen before
  this result (this one) could have claimed first contact.
- Suppression check (F-XF4): all fifteen rows, all verdicts, and both
  failed directions (undercount and overcount) are published above and in
  the JSON artifacts verbatim.
