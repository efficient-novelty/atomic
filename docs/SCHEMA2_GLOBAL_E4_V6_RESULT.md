# Schema2 global E-4 v6 assembly result

**Date:** 2026-07-21. **Status:** the sealed-head application witness is
resolved; an exact candidate-fresh-head `Unknown` survives; exhaustion remains
unproved.

The create-new artifact is `docs/schema2_global_e4_assembly_v6.json`, with
result digest
`blake3:2d58e39bbe6ffd4e63883181b5b08cdc836def50c7d10bb3178da261743d0aca`.
Definition replay succeeds.

## Resolved predecessor witness

The v6 assembly rechecks

```text
[Univ, Lam(App(Lib(14), Lib(15)))]
```

as an exact raw-catalog member. It now lands in the transparent-former
`Internal` branch: one clause is inherited, one clause carries complete
constructor evidence, the `app-stuck` derivation and every premise replay, the
typed result and provenance are preserved, and marginal `nu = 0`.

## Next exact fail-fast witness

The rerun then tests

```text
[Univ, Lam(App(Var(1), Lib(15)))].
```

It is an exact closed raw-catalog member. Clause 0's earned `Internal`
certificate is explicitly supplied, but the application function is that
candidate field itself. The ambient token therefore refuses the clause with

```text
candidate-fresh application head is excluded
```

This is the adopted F-A3 firewall, not a missing-premise accident. The
classifier retains F-G4 and
`INDUCTIVE_TELESCOPE_INTERNALITY_NOT_PROVED_FOR_CANDIDATE`.

## Lawful stopping point

F-A5 therefore continues the ladder honestly: the lower bound on surviving
`Unknown`s is one, `no_unknown_survives = false`, class exhaustion and global
E-4 completion are false, and the five memberships remain unauthorized.
Consequently E-2b/F-Q2, Agent A F-T1, E-5/F1, the classifier bridge, and the
fork were not executed.

The next rung must keep F-A3 intact and separately adjudicate whether a
candidate-field-headed application can earn `Internal`. Any positive rule
needs its own replayable evidence and another create-new global E-4 rerun.
