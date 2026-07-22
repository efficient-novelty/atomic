# Schema2 E-5 demand-projection result

**Date:** 2026-07-21. **Status:** E-5 was executed fail-closed. The exact
J3 temporal subgrammar is clean, but full A3 extraction completeness remains
open. E-5 is therefore partial; Guard-Rail F1 is excluded only on the exact
J3 subgrammar and is still not executable over all intended `C(S15,S14)`.

The create-new artifact is
`docs/schema2_e5_demand_projection_v1.json` (230345 bytes), with result digest
`blake3:24b1f886d923d6fbeb793ca40b39f2414b86b8c3cbe1d6bbc0f9722051609f90`.
Definition replay succeeds.

## Exact J3 instance check

The issuer reads no focus-family labels and no score or bar. It takes the
eight exact Step-15 J3 schemes and the eight Step-14 clauses whose replayed
kernel type is `Type`, then performs all 64 typed substitutions.

For every instance it records:

- the exact generic J3 expression and Step-14 typed image;
- a replayed structural-substitution derivation;
- term-level elaboration of the required output over `B15`;
- frozen beta-normal/univalent equality witnessing that normalization
  commutes with the substitution; and
- a `D(B15)` derivation under the explicit typed-kernel-scheme
  specialization rule.

All 64 instances type-check, all normalization squares commute, and every
instance is derivable. Uniform specializations are not multiplied as new
families: the quotient has exactly eight natural-family orbits, one for each
J3 scheme, and all eight are discharged. Consequently no
demanded-but-underdetermined instance exists in this exact J3 subgrammar.

The all-empty caller timeline cannot suppress this generation: the 64
instances are reconstructed from the frozen `(S15,S14)` registrations alone.
This passes E-5's negative-control idea on the restricted surface.

## Why this is not full E-5

The exact theorem note's A3 postulates a finite `C(W)` but does not give its
constructors. Agent D's independent Stage-16 stock has 17 clause seeds and 90
pre-instance rule seeds: 17 unary, 72 chronological binary, and one higher
open-box seed. Those records explicitly are not typed demand instances. The
J3 audit above instead has 64 typed specializations and eight family orbits.
Neither inventory has been proved equal to all intended A3 demands.

Likewise, the inherited Phase-5b history's stage capacities (and the derived
Stage-16 capacity 5950) are arithmetic capacities only. They serialize no
instances and no per-instance `D(B)` proofs. E-5 therefore refuses to treat
the flags `d_membership_decidable` and `every_generated_orbit_derivable` in
those summaries as extraction evidence.

The missing theorem is recorded as:

```text
E5_A3_CW_EXTRACTION_AND_HISTORICAL_FOCUS_PROJECTION_NOT_PROVED
```

Specifically, a count-blind constructor grammar must still generate the
historical `C(W)` inventories, reproduce the complete focus ladder and the
Stage-3 demand-before-jurisdiction wrinkle as projections of those
inventories, prove the J3 subgrammar plus the other ordinary/binary/higher
constructors exhaustive, and establish window locality and expiration.

## F1 and downstream disposition

| Question | Result |
| --- | --- |
| Exact J3 instances checked | 64/64 |
| Exact J3 natural-family orbits | 8/8 derivable |
| Restricted F1 | excluded |
| Full A3 F1 | neither triggered nor excluded; not yet executable |
| Semantic `O(16)=empty` | not issued |
| Theorem 12 at instance granularity | not proved or refuted |
| E-5 complete | no |
| Bridge authorized | no |
| Halt claim | none |

The global E-4 v10 and DEMAND-COMPLETE v1 live replays are valid. The older
grammar-completion artifact retains its already-declared frozen-surface
status: live replay fails because the sealed M1 ancestor binds the earlier
`normalize.rs`; E-5 records the exact drift rather than rewriting that
ancestor.

The bar-free proposal remains a proposal. T-BF1, T-BF2, and T-BF3 have not
been emitted, and this partial E-5 result does not satisfy its adoption block.

## Verification

```text
cargo test -p pen-search e5_demand_projection::tests -- --nocapture
cargo run -q -p pen-search --example schema2_e5_demand_projection -- replay docs/schema2_e5_demand_projection_v1.json
```

The focused mutation suite passes: redigested attempts to promote full F1
exclusion, Theorem 12, semantic emptiness, bridge authorization, or a halt
claim are rejected.
