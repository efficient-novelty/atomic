# T-BI-NU1 / full-A3 compatibility audit

**Date:** 2026-07-22. **Status:** **NEGATIVE PREREQUISITE RESULT; NO NEW
RULE ADOPTED.** This audit compares the frozen full-history certificate, the
extraction-first T-BI-NU1 v2 result, and the currently adopted R1/R2/A3
accounting rules. It does not alter any certificate, bound, historical row, or
semantic rule.

## Artifacts compared

- `docs/phase5b_full_history_v1.json`,
  `blake3:0f9838e4b9f2f5dca8b2df3cbc5c759d832e0d01800cb486ce17f548e477f141`;
- `docs/t_bi_nu1_act_local_provenance_v2.json`,
  `blake3:4e144b60a71d9df092932de0e00e8f73365d508a6e00f1f2afa8f9ff94495e3c`;
- `docs/a3_rule_inventory_exhaustiveness_v2.json`,
  `blake3:bd3f33ed7a15b6f50c421d2e42ed7aaea710fbcab750d0a98a990a1dad3b30db`;
- adopted `formation-completion-package-family-rule-v1`,
  `derived-action-generator-membership-rule-v1`, the operational
  natural-family/instance law, and `future-hole-hypothesis-definition-v1`.

The T-BI-NU1 v2 computation issued all intrinsic packages before opening the
history comparator. Its intrinsic vector is

```text
2, 0, 1, 3, 1, 2, 1, 2, 3, 4, 4, 3, 4, 6, 6
```

while the archived certified vector is

```text
1, 1, 2, 5, 7, 8, 10, 17, 17, 19, 26, 34, 46, 62, 103.
```

F-AL1 therefore fires. The discrepancy is not repairable merely by attaching
the existing full-A3 base orbits as credit outputs.

## Exact full-A3 inventory

The table was computed from each historical window's independently enumerated
base-seed dispositions. “Eligible occurrence” means a promoted typed base
instance. “Base quotient orbit” is the cardinality of the distinct
`quotient_orbit_id` set after natural-family quotienting. Structural export is
the singleton structural-completion orbit when the raw debt predicate holds.

| Stage | Raw base seeds | Eligible base occurrences | Base quotient orbits | Exported base orbits | Exported structural orbits |
|---:|---:|---:|---:|---:|---:|
| 1 | 0 | 0 | 0 | 0 | 0 |
| 2 | 0 | 0 | 0 | 0 | 0 |
| 3 | 6 | 5 | 4 | 0 | 1 |
| 4 | 4 | 3 | 3 | 0 | 1 |
| 5 | 8 | 4 | 4 | 0 | 1 |
| 6 | 16 | 13 | 12 | 0 | 1 |
| 7 | 16 | 13 | 12 | 0 | 1 |
| 8 | 16 | 10 | 10 | 0 | 1 |
| 9 | 24 | 19 | 14 | 0 | 1 |
| 10 | 30 | 26 | 21 | 0 | 1 |
| 11 | 25 | 16 | 16 | 0 | 1 |
| 12 | 30 | 29 | 29 | 0 | 1 |
| 13 | 42 | 35 | 35 | 0 | 1 |
| 14 | 56 | 41 | 41 | 0 | 1 |
| 15 | 80 | 70 | 68 | 0 | 1 |
| 16 | 90 | 89 | 89 | 0 | 0 |

These export columns are not an interpretation imposed by this audit. In
`crates/pen-eval/src/a3_demand_grammar.rs:883`,
`make_scheme_and_instance` sets `independently_exported_demand` only for a
`StructuralCompletion` origin (`:909`). Its own comment says base occurrences
are uniform members (`:982`). The quotient sets
`independently_exported_demand_orbit` to the disjunction of its member flags
(`:1185`), so a quotient operation cannot silently promote a base occurrence.

This agrees with the adopted accounting text:

- uniform specializations are instances, not extra credit, unless separately
  exported as required outputs (`docs/SEMANTIC_NORMALIZATION_PROGRAM.md:155`);
- every independent credit needs a distinct local-role slot or distinct live
  required output (`:211`, `:231`–`:234`);
- a future hole is zero-kappa, zero-nu and mints no anchor or credit
  (`docs/future_hole_definition_adjudication.md:32`–`:34`).

There is consequently no adopted rule saying “one credit output for every
live base scheme orbit.” The current operational rule says the opposite: base
occurrences are non-exported unless a separate, versioned export decision is
made.

## The Stage-15 contradiction

The local-role codomain has four roles per kernel clause. The provenance
checker computes `local_capacity = 4 * kappa` and counts an instance again only
when it carries an independently exported demand-output reference
(`crates/pen-eval/src/semantic_provenance.rs:754`, `:790`–`:876`). Stage 15 has
`kappa = 8`, hence 32 local slots.

Three increasingly generous bounds make the contradiction explicit:

1. **Current export flags:** 32 local slots plus the one structural export give
   at most `32 + 1 = 33` available provenance positions.
2. **Counterfactual export of every base quotient orbit:** even if all 68 base
   orbits were declared exported, contrary to the current rule, the bound
   would be `32 + 68 + 1 = 101 < 103`.
3. **An exhibited matching relaxation:** `32 + 70 + 1 = 103` counts all 70
   eligible *occurrences*, before the two equal occurrences are collapsed to
   68 natural-family orbits. This inadmissible relaxation numerically reaches
   the archived number by violating the adopted family-versus-instance
   quotient and upgrading every base occurrence to an export. The equality is
   diagnostic; by itself it does not identify the archive's causal
   decomposition or prove this is the unique imaginable invalid relaxation.

This is an impossibility result for F-AL1 under the rules as currently
implemented. It is stronger than a missing code path. No correct implementation
of the present injection and export predicates can certify Stage 15 at 103.

The first capacity-level impossibility is exactly Stage 10. Using the v2
candidate kappa and the current A3 export flags gives:

| Stage | kappa | Exported output positions | Lawful capacity `4*kappa + exported` | Archived total | Capacity result |
|---:|---:|---:|---:|---:|:---|
| 10 | 4 | 1 | 17 | 19 | impossible |
| 11 | 5 | 1 | 21 | 26 | impossible |
| 12 | 6 | 1 | 25 | 34 | impossible |
| 13 | 7 | 1 | 29 | 46 | impossible |
| 14 | 9 | 1 | 37 | 62 | impossible |
| 15 | 8 | 1 | 33 | 103 | impossible |

Stages 1–9 are not refuted by this coarse capacity inequality alone. Stage 9
is the boundary case: `4*4+1 = 17`, exactly its archived total. The current
intrinsic extractor nevertheless finds only three marginal natural families
and no exported instance credit, so it does not reconstruct 17. Likewise,
capacity slack at earlier stages is not evidence that the missing families or
their anchor relations exist.

## Why the full-history v1 demand anchors are not A3 outputs

`phase5b_history_certification.rs` does not extract the units it certifies.
It first replays the structural score (`:725`, `:751`), then
`descriptor_inventory` expands the archived `nu_total` into exactly that many
mechanism descriptors (`:478`), and `issue_units` synthesizes one family ID per
mechanism ordinal (`:508`). When local slots run out it synthesizes orbit IDs
from a mechanism and a pair of window clauses, always using output position
zero. `demand_summary` itself is a hand-built unary/binary/open-box cross
product (`:421`), not the full-A3 typed eligibility and quotient computation.

At Stage 15 this procedure reports 24 local anchors and 79 demand anchors.
Full A3 exposes only one independently exported structural orbit at that
stage. The 79 identifiers therefore cannot be injected into a pre-existing
A3 required-output set. They are candidate/run-created coordinates. Setting
`historical_count_used_as_input = false` on each emitted unit (`:636`) does not
undo the fact that the loop cardinality came from `nu_total`.

The certificate also treats any imported successor digest as sufficient
provenance (`:762`, `:814`). That is a certificate join, not an act-local
reconstruction. Finally, it asserts the desired aggregate `sum_nu = 358`
inside issuance (`:908`). These choices are valid records of what v1 computed,
but F-AL2 forbids them as T-BI-NU1 authority.

## Per-stage reconstructability

### Stage 1

The raw clause extractor sees two marginal formation families and returns 2.
R1 gives a lawful route to 1: merge the carrier and its uniform completion
into one completed-package family, then issue no carrier exception. The
reusable package-shape witness is
`issue_stage1_r1_package_token`
(`crates/pen-schema/src/stage1_r1.rs:122`). However, that v1 token leaves the
exception undecided and the total unset (`:186`–`:188`), while the later
no-exception verdict imports the completed global E-4 v10 basis
(`crates/pen-search/src/completed_basis_membership.rs:41`, `:628`, `:685`).
T-BI-NU1 therefore needs a parameterized, prefix-local R1 exception proof;
the archived Stage-1 answer must remain only its comparator.

### Stage 2

The candidate has one typed family, identical to the Stage-1 completion in its
sealed predecessor closure. Marginality therefore returns zero. There is no
A3 base or structural export at Stage 2. Under the current representation and
marginality rule, the archived 1 cannot be ordinary marginal credit. It would
require a separately adopted constitutive-registration exception or a richer
typed declaration representation; neither may be smuggled into T-BI-NU1 as an
implementation fix.

### Stages 3–4

The current extractor returns 1 and 3, versus archived 2 and 5. These stages
do not violate the coarse `4*kappa + exported-output` capacity, so a future
semantic-family theorem could in principle enumerate additional genuine
roles. No such theorem currently exists. The full-history v1 extra rows are
synthetic `IntrinsicKernel` ordinals, not extracted family IDs, so they cannot
serve as that theorem.

### Stages 5–8

The historical HIT machinery provides useful candidate/prefix computations:
typed path bundles, ordinary typed realizers, normalization, and naturality.
Reusable entry points include:

- `build_hist_cert` / `build_hist_cert_v3`
  (`crates/pen-eval/src/tdc1_hist_cert.rs:1183`,
  `tdc1_hist_cert_v3.rs:841`);
- `issue_ordinary_typed_realizer`
  (`crates/pen-schema/src/ordinary.rs:261`);
- `issue_ordinary_schema_normalization`
  (`crates/pen-schema/src/e3_normalization.rs:2002`);
- `issue_ordinary_constructor_naturality`
  (`crates/pen-schema/src/e34_class_induction.rs:248`).

They are not yet sufficient provenance. HIST-CERT v3 explicitly leaves each
registered path family's credit verdict `Undefined` and records that no
demand-orbit/EGP token binds it to independently chargeable novelty
(`crates/pen-eval/src/tdc1_hist_cert_v3.rs:664`, `:696`–`:697`). Agent A v4
nevertheless forms its totals by adding the typed-and-marginal path subtotal
to ordinary-family rows (`crates/pen-search/src/agent_a_hist_cert_v4.rs:277`)
and declares F-T1 discharged from typed/marginal status (`:353`, `:362`). It
does not close the missing injection. Thus 7/8/10/17 are promising local
schema inventories, but are not presently reconstructed as T-BI-NU1
ordinary-charge totals. In particular, the 2/2/5/10 beta/Kan inventories must
be quotient-classified and individually anchored; dimension-squared labels do
not establish independence.

### Stages 9–15

The extraction-first current values are 3/4/4/3/4/6/6, versus archived
17/19/26/34/46/62/103. For Stages 10–15 the local-plus-current-export bound
already makes equality impossible. At Stage 9 the target fits the abstract
capacity but no existing extraction supplies the required families or export
positions. R1 concerns the Stage-1 formation package and R2 decides the
Step-8 operation/cell/coherence cases; neither rule licenses P5, P6, squared,
inheritance, or synthesis ordinals merely because the structural formula
contains them.

## Required implementation sequence

This is the shortest sound successor program. It is a theorem program, not a
request to tune totals.

1. **Generic R1 transform.** Parameterize the Stage-1 package checker by an
   arbitrary `(prefix, candidate)` and prove the carrier-exception verdict
   from the act and prefix only. Apply it before family cardinality is sealed.
2. **Unified semantic-family extraction.** Extend the clause extractor
   (`predecessor_closure` and `extract_candidate_families`,
   `crates/pen-eval/src/typed_families.rs:544`, `:597`) with the already typed
   ordinary and cubical schema issuers. Quotient all of them by the same
   normalization, weakening, univalence, and naturality relation. Do not use
   `structural_nu` or a recorded total to decide how many rows to emit.
3. **Exact prefix A3 inventory.** Replace the coarse package-orbit input with
   `generate_a3_window_for_exact_prefix_unbounded`
   (`crates/pen-eval/src/a3_demand_grammar.rs:1932`) and
   `prove_a3_window_inventory_for_exact_prefix_unbounded`
   (`a3_rule_inventory_exhaustiveness.rs:1303`). Construct required-output
   references only for orbits whose independently-exported flag is already
   true.
4. **Proof-bearing anchor relation.** For every marginal natural family, prove
   the exact `(clause, LocalRole)` relation or the exact live A3
   `(orbit, required-output)` answer. A generator clause index by itself is not
   a proof that an arbitrary semantic role belongs to that family. Enforce
   non-reuse globally within the act.
5. **Path quotient theorem.** Decide whether each beta/Kan coordinate is a
   distinct natural family or a uniform instance. Any additional instance
   credit must name its own already-exported A3 output. Replay R2's generated
   Step-8 cell action as an instance of its parent.
6. **Seal before comparison.** Hash the complete candidate/prefix package,
   then and only then open `phase5b_full_history_v1.json` for F-AL1. Publish
   the first divergence; never add an export or role because the comparator
   is short.

## Consequence

F-AL1 cannot pass under the currently adopted export and quotient rules. The
existing T-BI-NU1 v2 negative artifact is therefore the correct gate result,
and BI-0 must remain closed. A successor can improve semantic extraction and
may settle the non-capacity stages honestly, but matching all fifteen archived
totals requires a versioned semantic adjudication. The decisive choice is not
a coding detail: it would have to change what counts as an independently
exported demand output, what constitutes a natural family versus an instance,
or the status of the archived totals.
