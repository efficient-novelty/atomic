# ν-register construction execution result

**Date:** 2026-07-22. **Status:** **REPLAY-VALID NEGATIVE RESULT; BI-0,
BI-1, AND THE NON-ENACTED CONE REMAIN CLOSED.**

This is the create-new execution of `docs/nu_register_adjudication.md`.
It implements the T-BI-NU1 v4 intrinsic registry, T-SM1a, T-SM1b, their
fixed chronological join, and the first BI-0 semantic-register attempt under
F-AL1′. No sealed predecessor was edited and no incomplete extraction was
promoted to a semantic ledger.

## Create-new artifacts

- `docs/t_bi_nu1_semantic_provenance_v4.json` and
  `docs/T_BI_NU1_SEMANTIC_PROVENANCE_V4_RESULT.md`, digest
  `blake3:367196cae48f921b828ac4edecaa81e340202b78795917f122712f06bec2940d`;
- `docs/chronological_interface_slot_map_v4.json` and
  `docs/CHRONOLOGICAL_INTERFACE_SLOT_MAP_V4_RESULT.md`, digest
  `blake3:1b00718cc6ae78202ff736d855b4906e01362ad0a4916a85619fe64c34950d00`;
- `docs/bi0_semantic_register_v4.json` and
  `docs/BI0_SEMANTIC_REGISTER_V4_RESULT.md`, digest
  `blake3:017a9aa66c50123435817cd831b4d16e4b86125dba9a875c7c89c846da1fcc3a`.

All three artifacts replay from disk with `valid = true` and empty replay
error lists. Replay validity certifies the result, including a negative gate;
it does not turn that gate positive.

## T-BI-NU1 and F-AL1′

The v4 issuer implements the adjudication's six construction surfaces:
generic R1 evidence, unified ordinary/path-family extraction, exact-prefix A3
inventory, proof-bearing family-to-role anchoring, the path quotient, and a
seal-before-comparison audit. Direct labels are only search candidates: they
cannot issue `ProvedFamily` without a constructed, typed, natural family and
an exact prefix-local role relation.

The resulting classification is total as a registry declaration audit but
incomplete as a semantic extraction:

| Disposition | Count |
|---|---:|
| role declarations | 250 |
| proved families | 0 |
| theorem-backed impossibilities | 0 |
| named registry residuals | 250 |
| silent residue | 0 |

The per-stage residual distribution is
`[1, 1, 2, 5, 7, 6, 5, 12, 17, 11, 18, 26, 38, 53, 48]`.

The three registered boundary questions were not hidden:

- **Stage 1:** the term-local formation/completion construction replays, but
  the four carrier roles are not all decided by an exact C1 relation. The
  named blocker is `T_BI_NU1_R1_CARRIER_ROLE_SURFACE_INCOMPLETE_C1`; the
  exception theorem therefore does not issue.
- **Stage 2:** its sole typed family is proved predecessor-internal. With no
  independently adopted prefix-local constitutive theorem it is classified
  nonmarginal and mints no ordinary semantic credit. This does not repair the
  package's remaining registry residual.
- **Stage 9:** the exact boundary surface contains 17 map-role declarations;
  they remain explicit unresolved declarations rather than being inferred
  from their labels.

The Step-8 R2 check does replay the exact E-4 M1 generated-membership theorem
in a common occurrence namespace, so uniform generated instances are not
multiplied. Exact operational regression also passes. However, proof-bearing
transitive intrinsic isolation remains explicitly unproved under
`T_BI_NU1_TRANSITIVE_INTRINSIC_ISOLATION_UNPROVED`; direct source/runtime scans
are diagnostics only and cannot discharge that theorem.

Consequently extraction completeness and F-AL1′ are false. F-NR3 suppresses
the authoritative semantic vector, and F-NR4 therefore suppresses the
structural/semantic divergence table. The computed all-zero vector is emitted
only as a **non-authoritative extraction floor**; it is neither the semantic
history nor a claim that the acts have zero value.

## T-SM1a, T-SM1b, and the fixed positive gate

T-SM1a derives all **54/54** contextual-Formation cases. T-SM1b constructs
its 18-case surface from live A3 semantics, seals it before opening comparator
artifacts, and verifies that its instance-ID set is exactly the frozen v3
18-case sub-surface. It derives **17/18** cases.

The unique remaining case is:

- instance
  `blake3:4c9c4272b11cb6aec0c332cd6697401ccd3f95b8bfa2520b6b6dceb6986d3571`;
- blocker `T_SM1B_DEPENDENT_TARGET_CYCLE`;
- reason: `motive for parameter 2 depends on non-predecessors [1, 2]`;
- historical location: older Step 14 clause 3, newest Step 15 clause 6.

The exact chronological join therefore derives **71/72** sealed discharges
and **8/9** members of the former-gap set. Under F-NR5, this is an honest
negative result: partial promotion is forbidden.

## First BI-0 attempt

BI-0 was run create-new against both newly emitted prerequisites. It reports:

- T-BI-NU1/F-AL1′: **false**;
- F-SM1 fixed positive gate: **false**;
- BI-0: **false**;
- BI-1 invoked: **false**;
- non-enacted cone invoked: **false**.

No authoritative semantic ledger or archive-divergence table has been issued.
This is required by the adopted adjudication, not an implementation fallback.

## Required successors

The T-BI side needs three proof-bearing additions before it can be rerun:

1. a candidate-and-prefix-local bridge from `pen_core::Expr` normalization to
   the ordinary `Schema2` family calculus;
2. an exact family-to-local-role anchor relation theorem, capable of proving
   a family or proving its collision impossible;
3. a transitive intrinsic-isolation/capability theorem for the pre-seal call
   graph.

The chronological side needs a versioned, dependency-respecting
support-comprehension/context-pushout successor for the cyclic case, targeting
the dependent context
`(q1 : Type, q2 : Type, x : El(Eventually(F(q1,q2))))`, with `p1` mapped to
`F(q1,q2)` and `p2` mapped to `x`. It must then rerun the unchanged 72/72 and
9/9 positive gate.

Only after both sides pass may BI-0 reopen BI-1 and the cone.

## Verification

- `cargo check -p pen-search --lib --examples`: pass;
- act-local semantic-provenance tests: 7/7 pass;
- T-BI-NU1 v4 regression tests: 3/3 pass;
- T-SM1a tests: 3/3 pass;
- T-SM1b tests: 4/4 pass;
- chronological v4 tests: 2/2 pass;
- BI-0 semantic-register tests: 2/2 pass;
- all three emitted-directory replays: valid, with no replay errors;
- independent post-repair audit: no P0, P1, or blocking P2 findings.

A broad legacy suite was also sampled. Its failures were the already-known
live-source drift checks in older sealed-artifact modules, not failures in the
v4 focused surfaces above; the long legacy run was stopped once that baseline
pattern was established.
