# Global E-4 v9 result: the wrapper closes v8, F-A5 finds a new semantic gap

Date: 2026-07-21  
Status: create-new v9 emitted and replayed; global E-4 remains incomplete.

The adopted `ambient-telescope-candidate-wrapper-v1` was implemented and the
global E-4 scan was rerun over the wrapped domain. The create-new artifact is
`docs/schema2_global_e4_assembly_v9.json`, with result digest
`blake3:772103f0fad2c96d1f4bea479a5642283de5772e8bd4e1089469d66a62c3e9ac`.

## What v9 proves

- Motives are explicit candidate data and are hashed, enumerated, and supplied
  before classification. The classifier performs no motive inference,
  verdict filtering, selection, or repair.
- The six-node motive grammar is finite by an exact structural recurrence.
  Its unfiltered syntactic upper bound is 914,612 motives; B15 declaration
  replay can only reduce that set. No silent truncation is used.
- The forgetful projection is replayably surjective on the live and closed
  controls, and every tested wrapper projects back into the frozen raw
  catalog. The closed empty-ambient fiber agrees with the frozen classifier.
- The prior v8 artifact is byte-stable and replays unchanged.
- The v8 live-context candidate with the independently declared motive
  `Type -> Type` is now **Internal at marginal nu = 0**. Thus the
  `RAW_TELESCOPE_AMBIENT_MOTIVE_DECLARATION_MISSING` gap is genuinely closed.

## What F-A5 found

The verdict-blind scan orders motives by exact node count and fixed constructor
order. It visited 179 syntactic motives, of which 147 were admissible wrapped
candidates. Before the first Unknown it obtained 145 named typed exclusions,
one Internal candidate, and no residual `Classified` candidate.

The first surviving wrapped Unknown has the same exact frozen clauses as the
v8 live-context control and the four-node motive

```text
Type -> Element(Univ)
```

This declaration is B15-formable. Its domain agrees with the `Lib(15)`
argument, so the live ambient use is motive-typed and it is **not** a typed
exclusion. The contextual token fails later, at instantiation coherence:

```text
no registered closed probes inhabit ambient motive for parameter 1
```

The named gap is
`WRAPPED_CONTEXTUAL_INSTANTIATION_COHERENCE_NOT_PROVED`. Because this is an
independently enumerated member of the adopted wrapped domain, F-A5 requires
the scan to stop here.

## Consequence

Global exhaustion is not proved, global E-4 is not complete, and the five
pending membership verdicts are not authorized. E-2b, F-Q2, Agent A's
certified totals, E-5/F1, the bridge, and the fork remain deliberately
unexecuted in the artifact.

The next legitimate rung is a versioned, motive-parametric contextual
instantiation theorem (or equivalent generic closed-specialization evidence)
covering every admissible B15 motive. Restricting the motive grammar merely to
motives with already registered probes would violate declaration independence
unless such a restriction is separately adjudicated on non-verdict grounds.

## Verification

```text
cargo test -p pen-schema internal_classifier_branch_v8 -- --nocapture
cargo test -p pen-search ambient_wrapper_domain -- --nocapture
cargo test -p pen-search global_e4_assembly_v9 -- --nocapture
cargo run -p pen-search --example schema2_global_e4_assembly_v9 -- replay docs/schema2_global_e4_assembly_v9.json
```

All focused tests and the emitted artifact replay pass. Existing unrelated
workspace warnings remain unchanged.
