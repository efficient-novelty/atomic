# T-BI-NU1 act-local provenance v2 result

**Date:** 2026-07-22. **Outcome:** `F_AL1_v2_exact_regression_failed_BI0_stays_closed`.

The intrinsic issuer ran before the archive comparator. It issued 15/15 authoritative packages, minted 0 candidate demand-output positions, and covered 13/13 structural filler classes authoritatively. The conditional-at-most checker ran at every stage: **true**. T-BI-NU1 is proved: **false**. BI-0 authorization from this side: **false**.

| Stage | kappa | Extracted families | Marginal families | Credited families | Intrinsic nu | Gaps |
|---:|---:|---:|---:|---:|---:|---:|
| 1 | 2 | 2 | 2 | 2 | 2 | 0 |
| 2 | 1 | 1 | 0 | 0 | 0 | 0 |
| 3 | 1 | 1 | 1 | 1 | 1 | 0 |
| 4 | 3 | 3 | 3 | 3 | 3 | 0 |
| 5 | 3 | 3 | 1 | 1 | 1 | 0 |
| 6 | 3 | 3 | 2 | 2 | 2 | 0 |
| 7 | 3 | 3 | 1 | 1 | 1 | 0 |
| 8 | 5 | 4 | 2 | 2 | 2 | 0 |
| 9 | 4 | 4 | 3 | 3 | 3 | 0 |
| 10 | 4 | 4 | 4 | 4 | 4 | 0 |
| 11 | 5 | 5 | 4 | 4 | 4 | 0 |
| 12 | 6 | 5 | 3 | 3 | 3 | 0 |
| 13 | 7 | 7 | 4 | 4 | 4 | 0 |
| 14 | 9 | 9 | 6 | 6 | 6 | 0 |
| 15 | 8 | 8 | 6 | 6 | 6 | 0 |

Intrinsic vector: `[Some(2), Some(0), Some(1), Some(3), Some(1), Some(2), Some(1), Some(2), Some(3), Some(4), Some(4), Some(3), Some(4), Some(6), Some(6)]`.

Archived vector (post-issuance comparator only): `[1, 1, 2, 5, 7, 8, 10, 17, 17, 19, 26, 34, 46, 62, 103]`.

F-AL1 passed: **false**. First exact divergence: Stage 1, `certified_semantic_total`: intrinsic `2`, archive `1`.

Permitted conclusion: The v2 computation is a replayable negative prerequisite result. Any named intrinsic gap or F-AL1 divergence keeps T-BI-NU1 unproved and BI-0 closed; the archive was not used to repair issuance.

Next: Resolve the first named intrinsic theorem gap or exact F-AL1 divergence in a versioned successor; do not force the sealed vector and do not run BI.

Certificate digest: `blake3:4e144b60a71d9df092932de0e00e8f73365d508a6e00f1f2afa8f9ff94495e3c`.
