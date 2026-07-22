# R-T2 future-hole confluence v2: dependent-context rerun

**Date:** 2026-07-22. **Status:** CREATE-NEW PASS — R-T2 REFUTED; R-T3 OPEN.

Authoritative artifact:
`docs/r_t2_future_hole_confluence_v2_dependent_context_v2.json`

Digest: `blake3:14e3cf8993eb6f708ff46e95be04b4cbd39e5e3317fb64030778e8bc32e3cf85`

## Verdict

The same R-T2 v2 issuer reran over the same four certified R-T1 classes under
`dependent-ambient-context-v1`. Every branch now forms all five successor
schemes: four unary registrations and one structural registration. On every
branch exactly one unary row has declared arity 3 and exactly one opaque
prior-clause reference. Every registration replays, is total under sequential
typed substitution, has zero marginal charge, uses no outcome filter, and has
no named gap.

This makes all six pairwise comparisons lawful for the first time. They are
well formed and invariant under reversing branch order, but none has a perfect
5/5 matching:

| Stage-4 candidates (digest prefixes) | Matching |
|---|---:|
| `20167267` ↔ `43a0ed70` | 3/5 |
| `20167267` ↔ `4b2211ec` | 2/5 |
| `20167267` ↔ `b4f821d9` | 3/5 |
| `43a0ed70` ↔ `4b2211ec` | 3/5 |
| `43a0ed70` ↔ `b4f821d9` | 2/5 |
| `4b2211ec` ↔ `b4f821d9` | 3/5 |

Therefore the complete Stage-5 scheme sets are inequivalent on the adopted
declared-arity frozen-family quotient. The certificate issues the law-level
outcome `law_level_confluence_refuted_rung_rt3_opened`. It selects no branch,
uses no enumeration or hash order as a selector, and keeps the bridge and halt
certificate closed.

## F-DC and hygiene audit

- Relative A3 seed/constructor coverage is complete on all four branches.
- All four branches have 5/5 exact scheme and instance coverage and no gaps.
- All arity-3 opaque declarations replay by sealed reference and use the
  referenced type whole.
- The six comparisons use declared parameter arity, not maximum free-variable
  reference, as equality scope.
- Exact sealed-reference digests remain in formation evidence but are removed
  from the branch-independent semantic comparison key. The first create-new
  diagnostic artifact, `r_t2_future_hole_confluence_v2_dependent_context.json`,
  exposed that digest leakage during post-run audit and is superseded by the
  authoritative `_v2` artifact above. No file was overwritten.
- After that correction, the arity-3 semantic keys form two honest classes:
  candidates `20167267`/`b4f821d9` and candidates
  `43a0ed70`/`4b2211ec`. The full scheme sets still fail every 5/5 comparison.

## Consequence

Under the adopted tie-resolution protocol, this is not a bridge-ready result.
R-T2 has spoken negatively, so R-T3 is the only lawful next rung. The user must
adjudicate constitutive choice versus explicit nondeterminism without citing
downstream scores, the desired fifteen-step history, enumeration order, or the
bar.
