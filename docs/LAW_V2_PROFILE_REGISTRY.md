# Law V2 profile registry

**Registry:** `law-v2-profile-registry-v1`

**Status:** `ACTIVE`

The machine authority is
[`crates/pen-law/assets/law_v2_profile_registry_v1.json`](../crates/pen-law/assets/law_v2_profile_registry_v1.json).
`pen-law` parses it with a closed schema and pins every issued Profile A
semantic and result digest. The normal Law-V2 package tests therefore reject
semantic-manifest drift or a reclassification of the completed run.

## Profile A — frozen negative control

| Field | Frozen value |
|---|---|
| Primary profile | `gsc-inductive-completion-core-v1` |
| Continuation profile | `law-v2-owner-specific-inductive-continuation-v1` |
| Constitutive demand | owner-specific `G-Use` and `G-Compute` completion |
| Selective gate | total discharge |
| Result | unique direct eliminator, then complete debt-free halt |
| Final termination | `HaltedDebtFree` |

The H3 semantic digest remains
`blake3:d61458ebd47036861e48af9ef458df1b2b3dc890194069958ef2f14d4afdd11e`.
The H4 continuation semantic digest remains
`blake3:00d97ce3446442d91b8572557c16dd0576f7281260b5c000e64f41323323c7e2`.
Their issued result digests remain, respectively,
`blake3:f43acaf6f0b0b9e51dc9a55829eedbc1fb2f7cf1090260ac2d244d1616a27d03`
and
`blake3:20f8940f305a71802728af3c9206b7f0d0c126c4528459e9274f878a7dd324b8`.

`HaltedDebtFree` is a successful, profile-relative terminal classification.
It is not failure, lack of interest, or evidence for changing the frozen
profile.

## Reserved successors

`gsc-contextual-internalization-v1` and
`selective-productive-discharge-v1` are registered only as
`proposed_not_adopted`. They have no semantic digest, result, artifact, or
termination authority. A later adoption must be independently reviewed and
digest-frozen before a live run.

Every entry has an empty `privileged_candidate_fixtures` list. In particular,
no successor may import the Profile A direct eliminator as a privileged search
fixture. It may arise only through the successor's generic grammar and proofs.

## Typed final status

The public `pen-law::StructuralRunTermination` enum is:

```text
HaltedDebtFree
BlockedNoDischarger
BlockedProductivity
OutsideFragment
ResourceExhausted
Unknown
```

These alternatives separate a certified empty obligation profile, failure to
find a total discharger, failure of an adopted productivity gate, fragment
limits, resource limits, and missing proof.
