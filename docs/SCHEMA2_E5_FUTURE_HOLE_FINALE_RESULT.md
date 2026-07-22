# Schema-2 E-5 future-hole finale result

**Date:** 2026-07-21  
**Schema:** `schema2-e5-future-hole-finale-v1`  
**Status:** **V1 REJECTED BY POST-AUDIT; E-5 REMAINS OPEN**

> **Post-audit correction.** The JSON below is mechanically replay-valid,
> but its positive semantic conclusion is not admissible. The underlying A3
> generator explicitly leaves rule-inventory exhaustiveness and typed output
> construction false, while this assembly only partitions the 89 instances
> that generator happened to emit. In addition, all structural schemes were
> retrofitted with the same `x : Type |- x : Type` judgment; their scheme IDs
> did not bind a declared motive or constructor-specific filler predicate.
> Consequently the run does **not** execute F1 over full intended A3 and does
> **not** prove semantic O(16) empty. The artifact is retained as an immutable
> failed attempt and a versioned successor is required.

## Machine-reported v1 result (superseded)

The create-new run closes the two classes of undefined A3 output using the
adopted `future-hole-hypothesis-definition-v1`:

- all 17 unary actions replay through the registered transparent-former
  closure;
- all 13 historical structural holes replay as motive-typed open judgments
  and clause-4-prime specialization at their sealed fillers;
- F-FH1 passes on all 13 historical discharges;
- F-FH2 and F-FH3 pass: motives are declared before fillers or verdicts, and
  holes/fillings mint no kappa, nu, anchors, demand orbits, or credit;
- F-FH4 reports no named gap.

At Stage 16, the independently regenerated A3 inventory contains exactly 89
typed demand instances.  Its exhaustive D-membership partition is:

| Class | Instances | Disposition |
|---|---:|---|
| Unary transparent-former actions | 17 | derivable by hypothetical closure |
| Direct chronological actions | 64 | derivable; quotient regression is 64 to 8 families |
| Pointwise chronological actions | 8 | derivable; joined to those same 8 families |
| Higher open-box reductions | 0 | seed rejected: no typed path witness |
| Structural completion holes | 0 | no Stage-16 structural demand |

The union is exact, every one of the 89 instances has a replayable typed
derivation, and the underdetermined set is empty.  Therefore the executable
instance-level guard rail returns:

```text
F1_EXCLUDED_FULL_A3_SEMANTIC_O16_EMPTY
```

Those booleans were emitted by v1, but the post-audit findings above show
that they do not prove the full-granularity refinement of Theorem 12 and do
not lawfully authorize T-BF2. The bridge, bar-free law, and halt remain
closed.

## Historical regression and the Stage-3 wrinkle

For every historical window, the runner joins the freshly generated A3
completion scheme to its orbit, singleton instance, future-hole judgment,
and clause-4-prime discharge.  Projecting those semantic rows back to focus
families reproduces the complete coarse O-ladder.

Stage 3 remains explicitly demand-before-jurisdiction.  Its occurrence ID is
not equated with Stage 4's occurrence ID.  Instead, the certificate proves
the two `former_eliminator` schemes have the same open body, declared motive,
kernel types, transparent closure, weakening behavior, whole Step-4 filler,
and specialized result.  Both instantiation replays succeed.  Thus the
wrinkle is reproduced by a semantic transport theorem rather than erased by
identifier equality.

## Count- and verdict-blind discipline

The issuer accepts no desired count, bar, winner, or F1 verdict as an input.
It regenerates A3 from the sealed signature and records a forged caller
inventory noninterference test.  The coarse ladder is used only after the
semantic construction as F-FH1's regression corpus.

The certificate also retains the previously declared frozen-surface drift:
the archived naturality transport digests replay, while live replay of its
older grammar/J3 predecessor chain is false because bound source files have
subsequently changed.  This drift is explicit, no archived artifact is
reinterpreted, and the current A3 grammar is independently regenerated and
replayed for this certificate.

## Artifacts and verification

- Certificate: `docs/schema2_e5_future_hole_finale_v1.json`
- Future-hole definition and issuer:
  `crates/pen-eval/src/future_hole_hypothesis.rs`
- E-5 assembly and replay: `crates/pen-search/src/e5_future_hole_finale.rs`
- Driver: `crates/pen-search/examples/schema2_e5_future_hole_finale.rs`
- Result digest:
  `blake3:2b280056a7153749fb147ee3d9966bc96690086da7ef78f875e8cafe79fbe372`

Explicit disk replay returned:

```text
valid true
membership_count 89
underdetermined_count 0
historical_discharge_count 13
semantic_o16_empty true
f1_executed true
f1_excluded true
e5_complete true
t_bf2_authorized true
bridge_authorized false
outcome E5_COMPLETE_F1_EXCLUDED_SEMANTIC_O16_EMPTY
```

Focused tests cover the live 89-row partition, all historical discharge and
Stage-3 transport assertions, caller-inventory noninterference, and rejection
of redigested semantic-O16/credit forgeries.
