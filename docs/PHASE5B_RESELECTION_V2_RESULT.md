# Phase-5b branch-(ii) reselection result

Date: 2026-07-21

## Outcome

The preregistered full sequential semantic reselection halts at Stage 8.
The burned run and its independent replay are valid.  The revised prefix is

```text
[1, 1, 2, 5, 7, 8, 10]
```

Stages 1--7 reenact their certified reference winners.  The complete guarded
Stage-8 cone contains exactly one admitted canonical candidate.  It is kernel
typed, is not identified with the prefix, and satisfies the provenance-capacity
check.  Its count-blind score derivation is

```text
structural total                 18
R2 generated-instance adjustment -1
semantic nu                      17
kappa                             5
rho                            17/5
Stage-8 bar                 357/104
```

The candidate therefore misses the bar by

```text
357/104 - 17/5 = 17/520.
```

The former testimonial score `18/5` would have cleared the same bar by
`87/520`; the completed R2 quotient removes precisely that margin.  No other
candidate exists in the complete guarded cone, so the clearing set is empty.

## Frozen-protocol consequences

The outcome semantics were frozen before the burn in
`docs/phase5b_reselection_program_v2.json`.  They require stopping only when no
candidate clears and forbid changing a rule after seeing the result.  Hence:

- the Stage-8 no-clearing result and the complete revised prefix are published;
- no Stage-8 winner is installed and Stages 9--15 are not reselected;
- no revised Bar16 exists;
- E-5/F1 is not authorized, because it requires a completed revised Step-15
  prefix;
- the bridge and final revised-bar certificate cannot soundly execute on this
  branch.

The machine-readable burned artifact is
`docs/phase5b_reselection_burn_v2.json`, with result digest
`blake3:6a265e5497341e3363efeed7bd224eac4120d5c2603ff4a581c200bb2341e6c1`.
Its preregistration digest is
`blake3:add0dd1566c522cdb8d2caf005b4d59e4ce20ee79b663886f04ac8fed07bc8ae`.

This is a certified Phase-5b divergence, not a proof of global halt at fifteen.
Any attempt to recover the fifteen-step sequence now requires an explicit,
versioned successor adjudication; it cannot be presented as execution of the
already adopted branch-(ii) protocol.
