# DEMAND-COMPLETE v1 result: coarse ladder replays, intended-schema identification remains open

**Execution date:** 2026-07-19  
**Artifact:** `docs/demand_completeness_v1.json`  
**Schema:** `demand-completeness-v1`  
**Artifact length:** `411603` bytes  
**SHA-256:** `76DC502E010A7A4B2D28905A36C3BD58BFD69923648BDC3FC13A79129D05870D`  
**Git blob:** `5bbaf4bd451ad9467a67642e854b9e1251d7ae7f`  
**Result digest:** `blake3:864ddda453e2107434866652128078797bc56c167d42cab715c4486268f78ac1`

## Outcome

The DEMAND-COMPLETE comparison ends in the preregistered **D-4 gap** branch,
not the D-3 identification branch. The coarse directive-debt ladder replays
exactly, including the stage-3 wrinkle. The current demand-orbit extractor
also replays its own typed grounding, answer, and locality checks.

That regression does not identify the extractor with Guard-Rail A3's intended
finite demand set `C(W)`. The current extractor obtains its live packages from
the caller/focus timeline and explicitly mirrors that timeline by
construction. The written source asserts that `C(W)` is finite and describes
generic unary and binary demands, but it does not operationally define the
historical instance grammar, the equality/orbit quotient, or membership in
the derivability closure `D(B)`. There is therefore no independently generated
intended inventory against which the extractor can be proved complete.

The named obstruction is:

```text
D4_A3_CW_INSTANCE_GRAMMAR_ORBIT_QUOTIENT_D_MEMBERSHIP_UNDEFINED
```

## Coarse regression signature

The replay preserves the historical caller demand and the focus seen by the
admissibility lane:

| stage | caller/package demand | focus consulted by lane | status |
| ---: | --- | --- | --- |
| 1 | none | none | pre-structural |
| 2 | none | none | pre-structural |
| 3 | `former_eliminator` | none | pre-structural wrinkle |
| 4 | `former_eliminator` | `FormerEliminator` | structural |
| 5 | `initial_hit` | `InitialHit` | structural |
| 6 | `truncation_hit` | `TruncationHit` | structural |
| 7 | `higher_hit` | `HigherHit` | structural |
| 8 | `sphere_lift` | `SphereLift` | structural |
| 9 | `axiomatic_bundle` | `AxiomaticBundle` | structural |
| 10 | `modal_shell` | `ModalShell` | structural |
| 11 | `connection_shell` | `ConnectionShell` | structural |
| 12 | `curvature_shell` | `CurvatureShell` | structural |
| 13 | `operator_bundle` | `OperatorBundle` | structural |
| 14 | `hilbert_functional` | `HilbertFunctional` | structural |
| 15 | `temporal_shell` | `TemporalShell` | structural |
| 16 | none | none | coarse open band |

Stage 3 is essential evidence: the `former_eliminator` demand exists one
stage before structural admissibility begins consulting directive debt. A
replay that started the demand only at stage 4 would not preserve the sealed
history.

## Why the identification cannot yet be stated

Guard-Rail A3 postulates a schema assigning each two-step window a finite set
of coherence demands, including clauses and future-directed clause schemes.
It calls the engine focus-family generator the implementation of that schema.
This fixes the intended relationship in prose, but does not provide an
independent executable definition of either side of the relationship.

Theorem 12 then divides `C(S15,S14)` into mutual demands, unary
future-directed demands sourced in `S14`, and future-directed demands sourced
in `S15`, including binary temporal/interface demands. Its proof argues that
these generic instances lie in `D(B15)` using totality, law-likeness, and
initiality. It does not enumerate their historical substitutions, specify
when two substitutions are one orbit, or give a decision procedure and
certificate for scheme-instance membership in `D(B15)`.

Agent D can independently extract each historical window's clause vocabulary
and form a finite over-approximation of potential unary, cross-window binary,
and higher open-box rule seeds. Each serialized seed is explicitly marked
pre-instance: exported-public eligibility, univalent eligibility, typed-hole
instantiation, and `D(B)` membership are not inferred. These seeds therefore
provide input for a future grammar, but are not relabelled as `C(W)`.

The current Rust extractor is correspondingly narrower:

- it reads live package names from `directive_debt_timeline`;
- it creates one live orbit and one output position per package;
- it grounds a structural live package in typed content from the newest
  window entry and checks historical answers with a characteristic-former
  predicate;
- it proves totality and disposition completeness for the inventory produced
  by that deterministic function.

Those checks are useful and falsifiable, but their J2/J3 certificates range
over the extractor's own image. They do not prove that the image equals all
intended A3 instances. Reproducing a caller-owned timeline cannot serve as an
independent completeness proof for that same timeline.

The certificate makes that distinction executable. It calls the public
extractor with an explicit stages-1--16 timeline whose package sets are all
empty. The call succeeds, returns sixteen empty orbit inventories with an
empty locality ledger, and still marks J2, J3, and locality kernel-verified at
every stage. This is not a claim that the all-empty timeline is semantically
valid; it is a witness that the current evidence is relative to the supplied
timeline. The witness has a different derivation hash from the canonical
coarse extraction.

## Falsifier disposition

| falsifier | disposition |
| --- | --- |
| F1 | **not triggered, but not executable at intended-instance granularity**: no demanded-but-underdetermined `(S15,S14)` instance was exhibited, while the missing grammar and `D` decision prevent exhaustive search for one |
| F5 | **not triggered**: no non-finite or nonterminating `C(W)` was exhibited; A3's asserted finiteness is neither refuted nor independently machine-verified |

These are deliberately not negative proofs. In particular, “F1 not
triggered” does not mean that every intended instance was found derivable.

## Consequences for O(16) and the d = 4 diagnostic

The coarse engine result still reproduces an empty stage-16 package vector,
and the mirrored extractor can conditionally report an empty live-orbit
vector with checked historical discharges. Full semantic instance emptiness
for `O(16)` remains conditional on the missing identification theorem.

For the same reason, TDC's comparison between one extracted live demanded
output at Step 8 and zero for the registered `d = 4` attachment remains a
conditional provenance diagnostic. The `d = 4` attachment has not been
proved demand-orphaned with respect to all intended A3 demand instances.

## Replay and mutation verification

| check | recorded result |
| --- | --- |
| DEMAND-COMPLETE unit tests | `6 passed; 0 failed` after formatting |
| full `pen-eval` library regression | `197 passed; 0 failed` |
| coarse ladder and stage-3 wrinkle | exact ordered stages 1--16; stage 3 demand-before-jurisdiction preserved |
| extractor-relativity witness | all-empty caller timeline accepted with 16 empty inventories and kernel-marked J2/J3/locality |
| instance/orbit/gap mutation tests | exhaustive scalar mutation plus vector delete/duplicate/reorder and typed insertions all rejected after re-digest |
| create-new generation | passed; a second emit refused to overwrite the artifact |
| public JSON replay | valid; D-4 gap outcome reconstructed exactly |
| archival conservativity replays | O16 definition test passed; TDC v3 public replay passed; semantic-provenance regeneration was byte-identical (`71763` bytes) |

Reproduction:

```powershell
cargo test -p pen-eval demand_completeness --lib --no-fail-fast
cargo run -p pen-eval --example demand_completeness -- replay docs/demand_completeness_v1.json
cargo run -p pen-search --example tdc1_cubical_regression -- replay docs/tdc1_cubical_regression_v3.json
```

## Honest conclusion

The historical regression is clean and the operational gap in `C(W)` is now
named. This is not an extractor/intended-schema identification theorem. F1
and F5 are not triggered, but F1 is not yet executable at the required
instance granularity. Semantic `O(16)` emptiness and the `d = 4`
demand-orphan conclusion remain conditional. No Step-16 acceptance verdict,
closing inequality, or global halt at fifteen follows from this result.
