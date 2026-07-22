# Schema2 global E-4 v4 assembly result

**Date:** 2026-07-20. **Status:** guarded identity resolved; a new exact
structural Unknown found; class exhaustion remains unproved.

The create-new artifact is `docs/schema2_global_e4_assembly_v4.json`, with
result digest
`blake3:2ab9a81e4b80430fe44f69b0ad17ccb2a362826928d1aae7105cc7f015214e6a`.
Definition replay succeeds.

## Resolved predecessor witness

The v4 assembly replays the two clause-level weakening/erasure tokens and
rechecks

```text
[Univ, Lam(Var(3))]
```

as an exact raw-catalog member. It now lands in the guarded `Internal` branch
with zero marginal credit. The v3 guarded-inverse obstruction is therefore
retired.

## Next exact fail-fast witness

The rerun then tests the exact closed candidate

```text
[Univ, Lam(Univ)].
```

It elaborates, but it is outside the current direct, inductive-projection, and
guarded-identity branches. The classifier returns the named gap

```text
INDUCTIVE_TELESCOPE_INTERNALITY_NOT_PROVED_FOR_CANDIDATE
```

and retains F-G4. This is a structural-lambda closure gap: the constant
universe body is structural, but lambda introduction over that already
Internal body does not yet produce replayable candidate-level evidence.

## Lawful stopping point

Because an exact Unknown survives, class exhaustion and global E-4
completeness remain false. The five membership verdicts, E-2b/F-Q2, F-T1,
E-5/F1, classifier bridge, and fork remain unexecuted.

The next proof obligation is a versioned structural-closure `Internal` rule
for lambda introduction over an already Internal structural body, beginning
with the constant `Univ` body. It must replay at constructor level and award
no new marginal credit. Global E-4 must then rerun create-new again.
