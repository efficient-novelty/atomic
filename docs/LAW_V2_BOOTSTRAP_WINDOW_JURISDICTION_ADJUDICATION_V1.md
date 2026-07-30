# Law V2 bootstrap-window jurisdiction adjudication V1

**Date:** 2026-07-29

**Status:** `DRAFT_PROPOSED_NOT_ADOPTED`

**Proposed jurisdiction rule:** `founding-prefix-no-selective-benchmark-v1`

## Authority boundary

This document decides whether values assigned to the registered bootstrap
could legislate a Selective-Law productivity threshold. It does not calculate
any \(\kappa\), \(\nu\), density, bar, or candidate ratio. It neither adopts a
productivity gate nor changes the currently adopted total-discharge Selective
Law.

The completed Profile A run remains `HaltedDebtFree`. The issued
[`LAW_V2_WINDOW_REGISTER_AUDIT_V1.md`](LAW_V2_WINDOW_REGISTER_AUDIT_V1.md)
remains `UndefinedAudit`; this document does not rewrite that artifact.

## Decision

The registered three-act bootstrap may receive descriptive, proof-bearing
semantic values, but those values have no Selective-Law benchmark
jurisdiction.

Acts 1--3 are arena-founding initial conditions. Under Law V2A they are
registered, checked, and sealed; they are not inferred, selected, ranked, or
claimed unique by the two laws. Guarded-law jurisdiction begins only when the
first live demand is exported.

Therefore the expression

\[
  \frac{\nu_3+\nu_2}{\kappa_3+\kappa_2}
\]

may, if its inputs are later lawfully audited, be reported only as a
descriptive `BootstrapTrailingDensity`. It is not a Selective-Law benchmark
for Act 4 and cannot pass, fail, rank, or choose Act 4.

If `founding-prefix-no-selective-benchmark-v1` is adopted, its Act-4
jurisdiction classification will be:

```text
NoSelectiveBenchmark
```

The currently adopted law already establishes the weaker authority fact that
no value benchmark legislates Act 4. The proposed typed classification above
is not issued until this rule is adopted. Its result would be independent of
the numerical values and denominator.

## Why the founding prefix cannot legislate

The governing formal appendix establishes three separate facts:

1. Law V2A defines Acts 1--3 as disclosed registered arena-founding input and
   transfers jurisdiction to the guarded law only at the first live demand.
2. At guarded stages, acceptance is total discharge with typed provenance;
   value, bars, thresholds, efficiency comparisons, and deterministic keys
   select nothing.
3. The two-step windowed bar is explicitly descriptive, is undefined on the
   bootstrap in its own jurisdictional sense, and has every selective
   interpretation excluded.

Auditing the founding acts does not change how they entered history. A
measurement theorem can describe registered initial conditions; it cannot
retroactively convert them into candidates that earned admission inside a
pre-existing arena.

The old bar-clearing formulation and historical window tables are testimony.
They cannot override the current register and jurisdiction contract.

## Current Act-4 rule

Act 4 was the first guarded response in Profile A. Its adopted acceptance rule
was total discharge of the live `G-Use` and `G-Compute` outputs. Numerical
value had no legislative authority.

Accordingly:

- a future defined Act-4 \((\kappa,\nu)\) pair is descriptive;
- a future defined bootstrap trailing density is descriptive;
- comparing those two quantities may be reported only as a diagnostic if an
  audit profile explicitly requests it; and
- no comparison can reclassify, invalidate, or reselect the completed H3/H4
  act.

## Future productivity profiles

A future Selective-Law productivity profile is a new, versioned principle. It
must specify its own jurisdiction and warm-up rule before any live values are
inspected.

Under the proposed default:

- only acts admitted within that productivity profile may contribute to its
  benchmark;
- founding registered acts never contribute;
- Act 4 has `NoSelectiveBenchmark`;
- with no separate one-act warm-up rule, Act 5 also lacks a complete
  width-two selective window; and
- the first possible full width-two benchmark would govern Act 6 using Acts 4
  and 5, provided both acts were inside that profile and carry same-manifest
  certified values.

A different initialization convention requires a separately named
adjudication. It may not be inferred from convenient bootstrap arithmetic.

## Typed reporting and precedence

Inside a future Selective-benchmark evaluator, benchmark evaluation must occur
in this order:

1. **Jurisdiction.** If no adopted benchmark has jurisdiction, report
   `NoSelectiveBenchmark`. All benchmark numerator, denominator, and ratio
   fields are `not_applicable`, not zero and not undefined.
2. **Audit completeness.** If jurisdiction exists but any required value,
   weakening, provenance, manifest identity, or audit certificate is missing,
   report `UndefinedAudit` or the applicable typed `Unknown`. Do not infer a
   denominator value.
3. **Denominator.** Only after every input is certified under the same audit
   and cost manifests may the denominator be summed. If it is zero, report
   `UndefinedRatio(ZeroDenominator)`.
4. **Candidate ratio.** A candidate with \(\kappa=0\) is a theorem readout,
   not a continuation candidate; its efficiency is `not_applicable`.
5. **Comparison.** Compare only when jurisdiction exists and both benchmark
   and candidate ratios are certified rationals.

Neither `NoSelectiveBenchmark` nor `UndefinedRatio(ZeroDenominator)` may be
coerced to zero, infinity, `ProvenClears`, or `ProvenFails`.

The evaluator's implementation vocabulary should keep at least:

```text
NoSelectiveBenchmark
UndefinedAudit
Unknown(reason)
UndefinedRatio(ZeroDenominator)
DefinedDiagnostic(rational)
DefinedSelectiveBenchmark(rational)
```

`DefinedDiagnostic` and `DefinedSelectiveBenchmark` are distinct types, not
flags on one untyped number.

This evaluator is a separate result axis from descriptive measurement:

```text
WindowAuditOutcome::UndefinedAudit
BenchmarkJurisdictionOutcome::NoSelectiveBenchmark
```

Jurisdiction-first evaluation may not short-circuit the semantic audit. The
window audit should continue to seek descriptive values even when a selective
benchmark lacks jurisdiction.

## Relationship to Window Audit V1

Window Audit V1 asked whether the four acts and the two descriptive fractions
could be issued in one semantic register. It correctly returned
`UndefinedAudit` because no act-local semantic values were available.

This jurisdiction decision answers a different question: even if those
descriptive values later become defined, the registered bootstrap does not
make their trailing density a Selective-Law benchmark. Therefore:

- Window Audit V1 remains unchanged and valid;
- a future Window Audit V2 may issue descriptive values and a
  `BootstrapTrailingDensity`;
- if this proposed jurisdiction rule is adopted, the separate Selective-Law
  field for Act 4 will be `NoSelectiveBenchmark`; and
- no \(B_4\) with legislative force may be invented.

## Adoption gate

Before a future productivity profile can use any two-step benchmark:

1. independently review and adopt a productivity principle;
2. freeze its jurisdiction and warm-up convention;
3. freeze the value register, formula, and strictness convention;
4. require same-manifest proof-bearing inputs; and
5. expose live acts only after those choices are digest-bound.

Until then `founding-prefix-no-selective-benchmark-v1` remains
`proposed_not_adopted`, and the current law continues to use total discharge
without a value gate.
