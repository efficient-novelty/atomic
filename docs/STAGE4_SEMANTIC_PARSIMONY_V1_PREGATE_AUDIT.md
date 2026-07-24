# Stage-4 semantic parsimony v1 pre-gate audit

**Date:** 2026-07-22. **Status:** **REPLAYABLE MATHEMATICAL AUDIT;
NOT BI-0 AUTHORITY; NOT A CONE-CONTINUATION ARTIFACT.**

Certificate:
`docs/stage4_semantic_parsimony_v1.pregate-audit.json`, digest
`blake3:107dd9a38b8250fc8451f446c2379108d500152c7fc921fb0d2e8c9dc2035b19`.

## Result

The blind source-first audit retained all four strict Stage-4 candidates.  All
four have `kappa = 3`; their certified semantic-family values are:

| Candidate | Semantic `nu` | Status in semantic parsimony |
|---|---:|---|
| `blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407` | 3 | enacted; nonminimal |
| `blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308` | 2 | minimum |
| `blake3:4b2211ecae25f3afbb1187f3a4a1b644edfc6adbd3512b9ca27c64a7b731265b` | 2 | minimum |
| `blake3:b4f821d9bb28366d8ae37adc7c1cb3e28c8a0de60f05c81e9ad75ba4f8b0edd4` | 3 | nonminimal |

The minimum pair is `(kappa, nu) = (3, 2)`.  The two minimum roots are the
frozen R-T2 survivor pair, whose complete scheme sets are inequivalent (five
schemes on each side, three matched).  No branch was selected or executed.

Every root reissued its Stage-1-through-4 source-first B1/B2 package and the
real-receipt B3 isolation theorem.  The common Stage-1-through-3 semantic
prefix is `[1, 0, 1]`; every named role, quotient, A3, and silent residual is
zero.

## Why this is pre-gate only

The v1 Stage-4 opening token was self-issued from the exact Stage-1-through-3
payload.  Its `granting_gate_schema` field names BI-0, but the token contains
no BI-0 certificate digest or pass proof and was not issued by BI-0.  The
frozen BI-0 v5 certificate was read only after the semantic result was sealed;
it retains the correct passed/no-branch logical projection, but current
create-new reissuance reports deterministic drift after the source-first
hardening.

That ordering is sufficient for a verdict-blind theorem about the strict
candidate surface: R-T3 expressly permits the cone members as mathematical
objects.  It is not sufficient for the adopted operational gate.  The BI-0
prerequisites adjudication and the nu-register adjudication require BI-0 to
rerun and pass before BI-1 or cone continuation is reopened.  Postseal
testimony cannot issue authority retroactively.

Consequently this artifact must not:

- reopen BI-1 or authorize branch execution;
- move or select a branch index;
- feed UC-1 as certified cone authority; or
- unlock T-BF, the bridge, or the final certificate.

## Required successor

Issue a versioned source-first T-BI/F-AL1-prime certificate, rerun BI-0 against
that certificate and the replay-valid support-comprehension chronology, and
have the passing BI-0 issuer export a minimal self-digested Stage-4 opening
capability.  The authorized Stage-4 successor must consume and replay that
external capability before enumerating any candidate.  It may not mint the
capability itself.

