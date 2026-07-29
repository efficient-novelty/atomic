# Law V2 H4 continuation result

**Date:** 2026-07-29

**Status:** `HALTED`

**Profile:** `law-v2-owner-specific-inductive-continuation-v1`

**Continuation semantic digest:** `blake3:00d97ce3446442d91b8572557c16dd0576f7281260b5c000e64f41323323c7e2`

**Result digest:** `blake3:20f8940f305a71802728af3c9206b7f0d0c126c4528459e9274f878a7dd324b8`

## Result

The accepted direct eliminator was freely sealed as the fourth act. The exact
seal preserves the old boundary and adds only:

1. the bodyless eliminator head `r`; and
2. its compiler-generated equation `r(P,m,unit) ↦ m`, whose public-alias
   presentation is `r(P,m,g3) ↦ m`.

No additional generator, equation, closed-former group, contextual rule, or
legacy structure was added.

After advancing the active width-two window to Acts 3 and 4, the prospective
Step-5 census re-extracted the complete supported inventory:

| Demand | Disposition |
|---|---|
| `G-Use` | `Derived` by the sealed eliminator |
| `G-Compute` | `Derived` by the sealed computation equation |

Extraction, derivability, and expiration are all certified complete. Both
ports are derived, the live-orbit count is zero, and the cumulative weakening
replay proves that the formerly live H3 demands remain discharged.

The frozen demand-connectedness rule therefore excludes every positive-cost
continuation. The sequence halts after four sealed acts; no fifth structure is
generated.

## Scope

This halt is relative to the frozen owner-specific calculus containing only
`G-Use` and `G-Compute`. It is not a claim that every possible two-law
calculus halts at four, nor a retrospective proof about the archived
fifteen-act trace.

Continuing beyond this point would require a new constitutive
structure-producing rule—such as independently justified contextual
internalization, action, comparison, mate, or horn generation—specified,
versioned, and frozen before its result is inspected. Adding such a rule to
the completed H3/H4 profile would invalidate the experiment.

## Evidence

The machine-readable result is
[`law_v2_h4_continuation_v1.json`](law_v2_h4_continuation_v1.json). It binds:

- the accepted H3 result and all stable H3 semantic component digests;
- the unchanged H3 GSC semantic manifest;
- the pre-run
  [continuation adjudication](LAW_V2_H4_CONTINUATION_ADJUDICATION_V1.md);
- the exact free-seal event, new boundary, active anchor, and empty Q3
  registry;
- the H4-bound active-inventory digest;
- both typed `Derived` decisions;
- extraction, derivability, expiration, and cumulative-weakening evidence;
  and
- the positive-cost exclusion and halt certificate.

The halt-certificate digest is
`blake3:f0a0e3af6de0f25e8a1bba1bc080a25341b72e5e99c8b083942ff0dd1fb5af72`.

## Reproduction

Replay the issued result with:

```text
cargo run --quiet --locked -p pen-engine --example law_v2_h4_continuation -- --replay docs/law_v2_h4_continuation_v1.json
```

The replay returns `{"status":"valid","valid":true}`.

The implementation also passes portable `pen-gf2` and `pen-engine` tests,
Clippy with warnings denied, the pinned live continuation test, the Law-V2
oracle firewall, and replay of the original issued H3 artifact under the
additive verifier successor.
