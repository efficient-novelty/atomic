# Law V2 window-register audit V1

**Date:** 2026-07-29

**Status:** `ISSUED_INCOMPLETE`

**Outcome:** `UndefinedAudit`

**Machine-readable audit:**
[`law_v2_window_register_audit_v1.json`](law_v2_window_register_audit_v1.json)

## Decision

No lawful \((\kappa_i,\nu_i)\) pair can currently be issued for any of
registered Acts 1–3 or the direct-eliminator Act 4. Consequently,

\[
B^{\mathrm{sem}}_4
  = \frac{\nu_3+\nu_2}{\kappa_3+\kappa_2}
\qquad\text{and}\qquad
\rho^{\mathrm{sem}}_4
  = \frac{\nu_4}{\kappa_4}
\]

are both undefined. The direct act is therefore **unrankable**: it is neither
`ProvenClears` nor `ProvenFails`.

This is `UndefinedAudit`, rather than `Unknown`. No completed operational audit
returned an outside-fragment or resource result. The proof-bearing semantic
audit required to pose the numerical question has not yet been constructed.

The fraction orientation above follows the canonical semantic-register
definitions in
[`app_a_two_laws_formal_axioms.tex`](app_a_two_laws_formal_axioms.tex):
semantic families are value and first-irreducible public clauses are cost.
If both ratios later become defined, `ProvenClears` means only the descriptive
strict comparison \(\rho^{\mathrm{sem}}_4>B^{\mathrm{sem}}_4\).
That convention has no selective authority and does not pre-adopt the
strictness or formula to be adjudicated for any future productivity gate.

## Register discipline

The audit consumed only the issued Profile A evidence:

- the unchanged three-act registered bootstrap and exact H3 history;
- the frozen H3 semantic digest
  `blake3:d61458ebd47036861e48af9ef458df1b2b3dc890194069958ef2f14d4afdd11e`;
- the issued H3 direct-response result
  `blake3:f43acaf6f0b0b9e51dc9a55829eedbc1fb2f7cf1090260ac2d244d1616a27d03`;
- the frozen H4 continuation digest
  `blake3:00d97ce3446442d91b8572557c16dd0576f7281260b5c000e64f41323323c7e2`;
  and
- the issued H4 halt result
  `blake3:20f8940f305a71802728af3c9206b7f0d0c126c4528459e9274f878a7dd324b8`.

It consumed no legacy structural values, archived semantic totals, archived
candidate catalog, or enacted future outcome. Visible declaration counts were
not promoted to \(\kappa\), and the two GSC demand families were not promoted
to \(\nu\).

## What the current evidence proves

| Act | Exact bound surface | \(\kappa_i\) | \(\nu_i\) |
|---:|---|---:|---:|
| 1 | registered event, checked boundary extension, one normalized declaration | undefined | undefined |
| 2 | registered event, checked boundary extension, one normalized declaration | undefined | undefined |
| 3 | registered event, checked boundary extension, one normalized declaration; typed GSC birth support | undefined | undefined |
| 4 | exact free seal of one bodyless eliminator head and one generated equation; complete response quotient and debt census | undefined | undefined |

These bindings are genuine evidence, but they answer different questions from
the semantic register:

- A checked declaration is not yet a certificate that the clause is charged
  once at first irreducibility rather than being an alias or forced
  definitional completion.
- `G-Use` and `G-Compute` are typed demand families. Their presence and
  discharge do not enumerate
  \(\operatorname{Marg}_2(H;x)\).
- The H3 Q0/Q2/Q3 quotient identifies response-candidate presentations. It is
  not the natural semantic-family-versus-instance quotient.
- H3 term weakening and H4 cumulative demand weakening preserve derivations.
  They are not the required typed map
  \(\mathrm{wk}_x:\mathsf{Sch}^{\mathrm{GF2}}_2(H)\to
  \mathsf{Sch}^{\mathrm{GF2}}_2(I(H,x))\).
- The composite provenance digests bind manifests, sources, tools, and
  results. They are not SR2 injections from every marginal family to a
  kernel-role or pre-existing-demand anchor.

## Per-act residual audit

For every act, the machine artifact binds the exact event, source, binding,
predecessor boundary, successor boundary, extension, observed family, and
available quotient digests. It then records five open residuals:

1. first-irreducible kernel-clause basis;
2. complete pre/post marginal semantic-family basis;
3. typed semantic-family weakening from the predecessor;
4. equivalence-invariant SR2 provenance injection; and
5. complete family-versus-instance quotient.

Thus `zero_residual_audit_gaps` is false for all four acts. Assigning a number
despite any one of these gaps would violate the semantic-register definitions
and/or SR1–SR3.

## Root blocker

The formal appendix requires a finite
\(\mathsf{RawSch}^{\mathrm{GF2}}_2(H)\) carrier and a terminating typed
normalizer with type preservation, soundness, and completeness for the adopted
equivalence. The same appendix explicitly lists that carrier, checker, and
completeness certificate as not implemented.

That missing theorem program blocks all four audits at once:

1. implement and certify the finite raw-schema carrier, normalizer, and
   natural-family quotient;
2. construct each act's kernel first-irreducibility basis and typed family
   weakening map; and
3. construct the act-local SR2 injections and close every residual.

Only then may the four pairs and the two fractions be issued. No productivity
experiment is authorized while this result remains `UndefinedAudit`.

## Profile A remains classified independently

This failed numerical precondition does not weaken the completed structural
run. The H4 census still proves extraction, derivability, and expiration
complete; both extracted ports are `Derived`; no live orbit remains. Profile A
therefore remains `HaltedDebtFree`.

## Replay

Validate all input bindings, residual dispositions, and the fail-closed
calculation with:

```text
python scripts/check_law_v2_window_register_audit_v1.py
```

The expected result is:

```json
{"complete_acts": 0, "outcome": "UndefinedAudit", "profile_a_termination": "HaltedDebtFree", "residual_gap_count": 20, "status": "valid"}
```
