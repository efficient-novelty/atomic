# Schema2 global E-4 successor assembly result

**Date:** 2026-07-20. **Status:** successor rerun executed; exact F-I2/F-G4
survivor found; class exhaustion not proved.

The create-new artifact is `docs/schema2_global_e4_assembly_v2.json`, with
result digest
`blake3:de80a123d2542782b8559ae7d52beccf152317a10e83199747cbdddd1b882e95`.
Definition replay succeeds.

## Assembly result

The rerun replayed both the failed global E-4 predecessor and the adopted
Internal-branch successor. It then rechecked the exact witness against the
frozen raw catalog and the successor classifier.

The witness remains an exact raw-catalog member. Its `Univ` clause earns
Internal derivability, but `Lam(Var(1))` depends on candidate-local field 0.
No complete Internal certificate issues, and the classifier retains the
candidate under
`F_G4_TYPED_CANDIDATE_OUTSIDE_EVERY_ADOPTED_SCHEMA2_CLASS`.

The successor assembly records one Unknown survivor as a lower bound and
terminates fail-fast. One survivor is sufficient to make each of these false:

- `no_unknown_survives`;
- class exhaustion;
- global E-4 completeness; and
- authorization of the five pending membership verdicts.

## Downstream stopping point

The five membership verdicts, E-2b regression, F-Q2, Agent A's F-T1
discharge, E-5/Guard-Rail F1, the classifier/bridge, and the fork were not
executed. The artifact explicitly records each output as withheld.

The next step requires an explicit versioned semantic decision: retain the
exact `Lam(Var(1))` term and reopen its grammar classification, or adopt
`Lam(Var(2))` as the corrected identity candidate. The frozen term must not
be reinterpreted in place merely to obtain exhaustion.

