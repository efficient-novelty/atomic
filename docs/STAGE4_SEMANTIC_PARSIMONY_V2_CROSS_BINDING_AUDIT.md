# Stage-4 v2 cross-binding audit

**Date:** 2026-07-22. **Status:** implementation audit; no new semantic
adjudication.

## Finding

`stage4_semantic_parsimony_v2` is a valid theorem/pre-cross-binding audit, but
it is not the final cone-authority certificate. It establishes two facts in
the right order:

1. a caller-supplied BI-0 v6 opening capability replays before Stage-4 work;
2. the verdict-blind V1 semantic cone computation then completes without
   selecting or executing a branch.

The v2 certificate contains both the BI-0 capability and the V1 preseal, but
it does not prove that they carry the *same typed prefix payload*. In
particular, v2 does not join each BI-0 Stage-1-through-3 telescope to the
corresponding `blind_audit.preseal.opening_token` telescope and then propagate
that equality through every preseal common-prefix copy. Its
`authorized_stage4_cone_audit` field must therefore be read as a
pre-cross-binding theorem claim, not as final cone authority.

## Required successor

`stage4_semantic_parsimony_v3` consumes the archived v2 certificate only as a
claim and first replays v2 in full. It then proves, entry by entry, equality of
the BI-0 and blind-preseal:

- stages;
- telescope payloads and their candidate hashes;
- predecessor signature digests, independently recomputed on both sides;
- full three-entry prefix signature digest;
- opening-token steps, candidate hashes, and signature-digest copies;
- top-level preseal steps, candidate hashes, and signature-digest copies;
- the cone-geometry common-prefix copy;
- every semantic root as the exact Stage-4 extension of that cross-bound
  three-stage prefix, using its matching live geometry telescope and its
  already-proved candidate/prefix/package bindings; and
- every selection root as the exact projection of one such proved semantic
  root.

Only the v3 certificate may promote the already-computed blind result to
`authorized_stage4_cone_audit = true`. V3 does not use v2's same-named field as
a premise, cannot issue BI-0 or v2, and preserves the certified no-branch and
semantic-divergence outcomes exactly.

No v3 create-new artifact is issued by this audit document. The example's
default inspection reads `docs/stage4_semantic_parsimony_v2.json`; an explicit
`create-new DIRECTORY` reads the caller-supplied
`DIRECTORY/stage4_semantic_parsimony_v2.json` and writes v3 only there.
