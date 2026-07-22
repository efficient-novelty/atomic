# Schema2 G-8 total-classifier result

**Date:** 2026-07-20. **Status:** total disjoint-sum classifier complete;
global E-4 assembly not run.

The create-new artifact is `docs/schema2_g8_total_classifier_v1.json`, with
result digest
`blake3:d19d97137f6ebb8fcda38117603189beda62d70eebe5692ecb4acff8e7f308da`.
Definition replay succeeds.

## The theorem implemented

`classify_raw_candidate` is total on arbitrary telescopes. It first checks the
frozen surface (`2 <= kappa <= 4`, imports among L14/L15 with support at most
two, at most six nodes per expression, path dimension at most one, no Trunc,
and no linear exponentials), then invokes the frozen elaborator against
`partial H_15`, and finally performs an exhaustive match on `TelescopeClass`.
Every input returns exactly one of:

1. a named surface or typed-elaboration exclusion;
2. a typed adopted class (Foundation, Former, Map, Axiomatic, Modal, HIT/V2,
   or Synthesis); or
3. the named typed obstruction
   `F_G4_TYPED_CANDIDATE_OUTSIDE_EVERY_ADOPTED_SCHEMA2_CLASS`.

The implementation exhaustively matches all 20 `Expr` heads and all nine
kernel telescope classes. `Hit` and `Suspension` both map to HIT/V2. The
`Unknown` branch is retained as the F-G4 obstruction; it is never dropped,
averaged, or silently treated as classified.

## Interpretation

This proves classifier *totality as a disjoint sum*. It does not prove that a
future global enumeration encounters no typed-Unknown candidate. If global E-4
assembly encounters one, F-G4 requires it to remain open under its recorded
obstruction; class exhaustion cannot be claimed over it.

G-8 therefore authorizes the next prescribed action, global E-4 assembly, but
does not execute it. No pending membership verdict, E-2b count, stage score,
or halt/continuation conclusion is emitted here.
