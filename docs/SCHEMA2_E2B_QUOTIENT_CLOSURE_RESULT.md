# Schema2 E-2b quotient-closure result

**Date:** 2026-07-21  
**Status:** E-2b complete; F-Q2 triggered by one certified Step-8 divergence  
**Artifact:** `docs/schema2_e2b_quotient_closure_v1.json`

## Outcome

The create-new E-2b successor consumed the completed E-4 membership artifact,
HIST-CERT v3, the frozen Stage-1 diagnostic, the monotone M1 cell-action
verdict, and the adopted R1/R2 and P1 texts. It then minted and replayed typed,
normalized, substitution-natural evidence for every unresolved ordinary row.

```text
ordinary rows expected/covered       24 / 24
ordinary-family tokens issued        23
generated-instance tokens issued      1
pending ordinary rows                  0
Stage-1 quotient output                1
operational row-coverage totals        7 / 8 / 10 / 18
independent quotient-family totals     7 / 8 / 10 / 17
F-Q2                                   TRIGGERED at Step 8
```

E-2b itself succeeded. F-Q2 is an output falsifier, so finding a certified
divergence is a valid terminal result of the phase rather than a replay or
construction failure.

## What was minted

For each of the 24 historical ordinary rows, the issuer reconstructs the
corresponding typed `OrdinarySchema`, then replays:

1. the E-2 ordinary typed-realizer token;
2. the E-3 frozen normalization token; and
3. the class-indexed constructor-naturality token under a genuine dependent
   substitution.

The quotient layer then issues exactly one of two mutually exclusive objects:

- an ordinary-family token, including its completed-basis provenance and a
  comparison against every family already issued in the run; or
- a generated-instance token naming its parent family and the proven M1
  sub-basis.

All formation rows are treated as the canonical completed-package family, not
as a second carrier family. All structural rows mint one family. The Step-8
left-unit mints one family because the completed basis issued `Independent`.
The Step-8 cell action mints no family because M1 had already issued
`Generated` from the parent operation and registered 3-cell.

No historical count, score, or acceptance bar occurs in any of those
decisions. The comparators are read only after all row resolutions have been
sealed.

## F-Q2 result

Stage 1 matches exactly:

```text
completed package family              1
separate carrier family               0
quotient output                        1
sealed comparator                      1
```

Steps 5–7 also match exactly. Step 8 separates two quantities that were equal
in the predecessor operational inventory:

```text
registered typed-and-marginal paths   10
ordinary operational rows              8
operational coverage total            18

independent ordinary-family tokens      7
generated ordinary instances            1
quotient-family total                  17
sealed comparator                      18
delta                                  -1
```

The exact certified cause is:

```text
step8_cell_action_is_a_generated_instance_of_the_parent_operation_and_does_not_mint_an_independent_ordinary_family
```

The left-unit is included in the seven independent ordinary families. Its
`Independent` verdict is therefore preserved and used. The divergence is not
caused by losing the coherence row; it is caused by refusing to multiply the
Generated cell-action instance into a second family, exactly as the v3 M1
result required.

Per adopted F-Q2, the artifact records the divergence verbatim, leaves R1/R2
unchanged, leaves the sealed history unchanged, and registers the Phase-5b
fork route without executing it.

## Successor boundary

The artifact issues an Agent A handoff containing:

- all 23 ordinary-family token hashes;
- the one generated-instance token hash;
- all 19 registered path-family join hashes;
- the four count outputs; and
- the F-Q2 derivation and divergence status.

Agent A, F-T1, E-5/F1, the bridge, the Phase-5b fork, and any halt or
continuation conclusion are explicitly unexecuted here.

## Certificate identifiers

```text
schema         schema2-e2b-quotient-closure-v1
result digest  blake3:9f27c97eac4ea96b329ccf6988a82c4ccb5eda5dd77fbafd28fca24ae898a0b4
file SHA-256   6bfbae477192370004f2d98664dccdbca9142da96f8ab4a146509c31f700407b
bytes          248709
```

## Verification

```powershell
cargo test -p pen-search e2b_quotient_closure -- --nocapture
cargo run -q -p pen-search --example schema2_e2b_quotient_closure -- replay docs/schema2_e2b_quotient_closure_v1.json
```

The focused suite contains three positive/mutation tests. It rejects both a
forged independent credit for the Generated cell action and suppression of
the F-Q2 result.
