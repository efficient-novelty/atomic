# Naturality/orbit transport result

**Date:** 2026-07-21. **Status:** **PARTIAL SUCCESS; R-T1 COMPLETE.**
The create-new artifact is `docs/naturality_orbit_transport_v1.json`.
Its result digest is
`blake3:a7665275fd102d81c12f520d62b26059c363cc503076bde5994be74ab38a3388`.
Independent definition replay is valid.

## What the shared build proved

The Stage-16 A3 metadata inventory is partitioned exactly, with no dropped or
duplicated occurrence:

- 17 unary occurrences;
- 64 direct chronological occurrences; and
- 8 pointwise chronological occurrences.

All 72 chronological occurrences are in bijection with constructed,
kernel-typed transport records. The direct regression reproduces exactly 64
typed instances in the eight frozen J3 natural families. The additional eight
pointwise instances are typed substitutions and join those same eight family
IDs. They do not mint eight new families or independent demand credit.

The orbit exporter was also corrected so that independent-export provenance
is derived from member occurrences instead of being set unconditionally.

## Stage-4 R-T1 result

The module replays the live T-BF1 Stage-4 parsimony result and joins the exact
four minimizers at `(kappa, certified nu) = (3, 5)`. It then compares all six
pairs using complete typed packages: clause roles, kernel types, parameter
sorts, canonical normal forms, and the frozen equality procedure.

The quotient has **four semantic classes**, not one:

1. `Pi` package with application order `(Var(2), Var(3))` — candidate
   `blake3:201672...a3407`;
2. `Pi` package with application order `(Var(3), Var(2))` — candidate
   `blake3:43a0ed...93308`;
3. `Sigma` package with application order `(Var(3), Var(2))` — candidate
   `blake3:4b2211...1265b`; and
4. `Sigma` package with application order `(Var(2), Var(3))` — candidate
   `blake3:b4f821...0edd4`.

`Pi` and `Sigma` do not become equal under the frozen beta-normal equality,
and the two application orders carry different exact parameter-sort vectors.
The quotient is computed from pairwise judgments rather than digest equality.
A reversal test proves that package and comparison enumeration order does not
change the quotient. No hash or presentation order selects a candidate.

Therefore R-T1 is complete and negative: it does not dissolve the tie, and
the adopted protocol advances to R-T2.

## Remaining A3 boundary

The build does **not** define either of the following outputs:

- the action/operator demanded by each of the 17 unary seeds; or
- an intrinsically typed future-hole term for the 13 historical structural
  completion occurrences.

The existing naturality tokens transport an already chosen term. They do not
choose an action. Returning the source clause would be reflexivity, not a
unary action proof. Likewise, a structural snapshot determines a coarse
constructor predicate but supplies no carrier, endpoints, motive, or term for
an object-level hole. Importing the later historical answer would be
retrospective.

Accordingly, full A3 output-grammar completeness is false, F1 is not
executable, and semantic `O(16)` is not decided by this artifact.

## Frozen-surface hygiene

The imported grammar and restricted J3 archives both pass their internal
digest checks. Their current live source replays do not pass, and that drift
is serialized rather than hidden:

- grammar replay reaches the known `e4_generator_basis_source` mismatch
  (`82053/fb19...` archived versus `82321/149d...` live); and
- the restricted J3 replay stops in its upstream v3/v2/history predecessor
  chain.

The result is therefore explicitly relative to the byte-stable frozen wrapped
surface, consistent with the adopted hygiene rule. The live drift is not used
to mint a stronger conclusion.

## Verification

- transport unit tests: 4 passed, including reversal invariance and forged
  promotion rejection;
- A3 grammar tests: 7 passed;
- create-new emission: valid;
- public JSON replay: valid;
- artifact size: 417274 bytes;
- artifact SHA-256:
  `43CB2FF92212B8B01145348CA4EB374FF8BCFB719710AA88D08AFAB66D57566C`.

No bridge, bar-free adoption, or halt certificate is authorized by this
result.
