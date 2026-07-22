# Global E-4 v10 result: F-A5 finds no surviving Unknown

**Date:** 2026-07-21. **Status:** create-new v10 emitted and replayed; wrapped
class exhaustion is proved and global E-4 is complete.

The create-new artifact is
`docs/schema2_global_e4_assembly_v10.json`, with result digest
`blake3:118d015f6cabe967052891d4d4dab9660b9837a1f4e3a588fe7f2108bb4d5d2b`.
It byte-checks and replays both the sealed v9 predecessor and the generic
motive-parametric coherence certificate.

## What v10 proves

- The exact v9 survivor is now `Internal` at marginal `nu = 0`.
- The ambient wrapper, declaration independence, surjective projection, and
  closed-fiber agreement remain unchanged.
- The finite six-node motive grammar retains its complete, unfiltered
  syntactic upper bound of 914,612. There is no motive filtering and no silent
  truncation.
- An exhaustive match over all 17 predecessor contextual-error variants gives
  every case an explicit disposition: named typed exclusion, a previously
  certified zero-credit closure rule, an adopted charged class, the generic
  substitution theorem, or a replay invariant.
- The six-rule substitution induction is total, and registered probes are not
  evidence.

Thus every admissible wrapped candidate is either classified or excluded by a
named typed obstruction. F-A5 finds zero surviving `Unknown` candidates.
Wrapped-domain class exhaustion and global E-4 are therefore proved relative
to the adopted, source-bound classifier and finite wrapper domain.

## Authorized next action

The five pending membership verdicts against the completed basis are now
authorized. They have not been executed by v10. E-2b/F-Q2, Agent A's
certified totals, E-5/F1, the classifier bridge, and the fork likewise remain
deferred in the artifact. The adopted sequence must begin with those five
membership verdicts; v10 does not itself assert the final halt.

## Verification

```text
cargo test -p pen-search global_e4_assembly_v9 -- --nocapture
cargo test -p pen-search global_e4_assembly_v10 -- --nocapture
cargo run -q -p pen-search --example schema2_global_e4_assembly_v10 -- replay docs/schema2_global_e4_assembly_v10.json
```

All focused tests and both theorem/v10 artifact replays pass. Existing
unrelated `pen-search` warnings remain unchanged.
