# Stage-4 semantic parsimony v2 result

**Date:** 2026-07-22. **Outcome:** `STAGE4_V2_AUTHORIZED_SEMANTIC_DIVERGENCE_ADJUDICATION_REQUIRED`. **BI-0 gate replayed before cone audit:** **true**. **Authorized branch execution:** **false**.

| Candidate | kappa | semantic nu | Result |
|---|---:|---:|---|
| `blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407` | 3 | 3 | enacted; nonminimal |
| `blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308` | 3 | 2 | minimum |
| `blake3:4b2211ecae25f3afbb1187f3a4a1b644edfc6adbd3512b9ca27c64a7b731265b` | 3 | 2 | minimum |
| `blake3:b4f821d9bb28366d8ae37adc7c1cb3e28c8a0de60f05c81e9ad75ba4f8b0edd4` | 3 | 3 | nonminimal |

Minimum: `(3, 2)`; minimizers: **2**. Enacted root `blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407` has semantic nu **3** and is minimal: **false**. Frozen R-T2 survivor pair inequivalent: **true**. No branch selected or executed: **true**.

A passing current BI-0 v6 capability was replayed before any Stage-4 work, so the four-root semantic parsimony computation is an authorized cone audit. It selects and executes no branch.

Next: Adjudicate the certified semantic divergence (two nu-minimal, R-T2-inequivalent roots; enacted root nonminimal) before any BI-1 branch execution, UC-1 scoring, bridge, or final certificate.

Certificate digest: `blake3:fad8a4b1e688ccb3b8b2bb72e546c02d36197e92b8f69c52302873b10dbe2678`.
