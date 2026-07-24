# Stage-4 semantic parsimony v3 cross-binding result

**Date:** 2026-07-22. **Outcome:** `STAGE4_V3_CROSS_BOUND_SEMANTIC_DIVERGENCE_ADJUDICATION_REQUIRED`.

The caller-supplied V2 claim replayed before cross-binding: **true**. Exact BI-0-to-blind-preseal prefix cross-binding: **true**. V2's pre-cross-binding cone field used as authority: **false**.

| Position | Stage | Candidate hash | Predecessor signature | BI-0 payload hash | Cross-bound |
|---:|---:|---|---|---|---|
| 0 | 1 | `blake3:61631d63a5877aad1b32b1e71aa6f0e555317b65732f327e3f38e32c222e29e7` | `blake3:fcfaddfd614022b3f5b0daabd7c06751e7dbac04aee5469c49043d2423c66e20` | `blake3:c05455b91ccb58c3ecb74e3454acfe2d023e4a766cd35c8b38ec4be9328e8893` | true |
| 1 | 2 | `blake3:a2dfff0fb8ce1073119da3893e1d195247c234af66b9a7de17c85d5cf718f555` | `blake3:be623a2f25ae686ac37301c38ba62bb96a67b5e42f10604e3d6df96f11032005` | `blake3:a1f557f3a264186f20be6dd1e655e3c88f6e6ff06c0d2fa50ca741a586f6ae35` | true |
| 2 | 3 | `blake3:934b5599bb0f28abf0be9652982caec5e0ff6d7d204ddaa4e66f23c37155457e` | `blake3:3e6728c35b5e2653432bd09ed0cb9b9f14537b5d32646cc8ba9c688404e6ed21` | `blake3:e7648e75443856b2f1ed7540c2d73deea5b12b6f6510b569c3db9c6a947f764d` | true |

Computed full prefix signature: `blake3:4bdef8bb889e05f6ff2df384795098086e583dd00f6b5585daaabd35242a81c3`. Blind opening copy: `blake3:4bdef8bb889e05f6ff2df384795098086e583dd00f6b5585daaabd35242a81c3`. Preseal common-prefix copy: `blake3:4bdef8bb889e05f6ff2df384795098086e583dd00f6b5585daaabd35242a81c3`. Cone-geometry copy: `blake3:4bdef8bb889e05f6ff2df384795098086e583dd00f6b5585daaabd35242a81c3`.

Authorized Stage-4 cone audit: **true**. Authorized branch execution: **false**. No branch selected or executed: **true**. Divergence outcome preserved: **true**. Divergence requires adjudication: **true**.

The caller-supplied V2 claim replayed in full, and the exact typed Stage-1-through-3 payload exported by BI-0 is entrywise identical to the blind audit's preseal opening and every common-prefix digest copy. V3 therefore authorizes the completed Stage-4 cone audit. No branch is selected or executed.

Next: Publish and adjudicate the preserved two-minimizer semantic divergence before any BI-1 branch execution, UC-1 scoring, bridge, or final certificate.

V2 claim digest: `blake3:fad8a4b1e688ccb3b8b2bb72e546c02d36197e92b8f69c52302873b10dbe2678`. V3 certificate digest: `blake3:de24d0acf67b53c4bd48510369b0ef5b3043f40232413d732130d14d0c74d69a`.
