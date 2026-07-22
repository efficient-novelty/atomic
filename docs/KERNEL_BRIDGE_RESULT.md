# KERNEL-BRIDGE result: restricted bridges proved, semantic bridge remains open

> **Successor execution (2026-07-19):** the adopted v3 element overlay has
> now retired the registered S1/S2/S3 C7 obstruction through replayed
> witnesses. See `KERNEL_BRIDGE_V3_ELEMENT_OVERLAY_RESULT.md` and the
> create-new schema-5 artifact
> `ip1_candidate_verdict_join_v5_element_overlay.json`. This document remains
> the byte- and theorem-bound report of the archival v2/schema-4 execution;
> its v2 obstruction is intentionally preserved rather than rewritten.

**Date:** 2026-07-19. **Execution status:** obstruction exit, as required by
`agent_c_kernel_bridge_plan.md`. The schema-4 certificate is reproducible,
joins every schema-3 quotient row, and promotes no row across an unproved
premise. It does **not** establish the intended-semantic candidate join or any
later acceptance conclusion.

## Immutable input and new artifact

Schema 3 remains the archival input:

- path: `docs/ip1_candidate_verdict_join_v3.json`;
- length: 168,692 bytes;
- SHA-256: `4913E818C080D67C81CE1770F047E22DB9EA6975BFE704FB01352F268FA2B729`;
- internal digest:
  `blake3:e4cb89fe83476fb8a0eee43bcc976053bcbbff0bd6db35607d7e8154a3ef858b`.

Schema 4 was created separately at
`docs/ip1_candidate_verdict_join_v4.json`:

- length: 535,972 bytes;
- SHA-256: `4094B2E65E36507157029760C71FF62BA241E82D066CF590F9184DE66BEDB5F4`;
- internal digest:
  `blake3:6a6c3ccfe4731080bfaa944b98f50afe2bde1eb29fdd679b8f62e3a42f72d1e2`;
- rows joined: 213 of 213, by stratum index, row index, and row digest;
- allowed schema-3 projection only: kappa, class, proxy, support, route, and
  count;
- bridge promotions: zero.

The schema-4 implementation compile-time binds the exact schema-3 byte
length, SHA-256, and internal digest. Its replay and mutation tests reject
archive, row, theorem-evidence, completeness-flag, and outer-digest changes.

## Obligation ledger

| Item | What was machine-checked | Result and exact remaining obstruction |
|---|---|---|
| C-1 substitution | Capture-safe simultaneous substitution over every current `Expr` constructor; identity; sequential-versus-composed equality for issued restricted substitutions; sort preservation and library-support preservation for sort-identical variable images | Restricted theorem proved. Arbitrary typed expression images require a dependent instance-typing judgment not supplied by the current kernel: `C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION`. |
| C-2 naturality basis | For one validated representative canonical presentation, all sort-preserving parameter permutations (within the implemented arity cap), identity, composition closure, and unused weakening by both parameter sorts | This is a finite raw renaming witness, not an enumeration of the schema-3 classes and not a classifier for intended schemas. Gap: `C2_DEPTH_TWO_SCHEMA_GRAMMAR_AND_GENERATOR_COMPLETENESS`. |
| C-3 fuel composition | Per clause, the allocated bound is `node_count_i * kappa` and observed use is checked below it; summing the allocations is definitionally the whole-telescope bound. All fifteen sealed historical telescopes replay | The proof is complete for those fifteen inputs, not for every schema-3 candidate without a total candidate-to-telescope/elaboration derivation. Gap: `C3_DOMAIN_WIDE_WHOLE_TELESCOPE_FUEL_COMPOSITION`. |
| C-4 schema-4 join | Exact archival binding, all 213 joins, fail-closed theorem statuses, and exhaustive mutations | Replays successfully. Both `full_candidate_extraction_join_proved` and `intended_semantic_candidate_join_proved` remain false. |
| C-5 P5 re-audit | The literal `App(Lib(14), Lib(13))` witness is replayed; its public lift token and adapter succeed; restricted variable-image substitution preserves library support | The literal witness is still outside the raw catalog. Record internality is unavailable, and arbitrary contextual-substitution inventory is incomplete. Whether a route opens under the missing complete theorem is deliberately `null`, not guessed false. Gap: `O-C5-ContextualSubstitutionClosure`. |
| C-6 V2 term layer | The adopted boundary and charging versions are pinned; declared contexts, reference-only terms, face/overlap checks, and replayable tokens exist. The registered Trunc endpoint diagram type-checks | Generic historical typed `coe`/`hcom`, face-indexed motive methods, typed realizers for all keys, and independence/exhaustiveness remain incomplete. Gap: `C6_V2_GENERAL_HISTORICAL_TERM_LEVEL_COMPLETION`. |
| C-7 base bindings | The S1, S2, and S3 registered inputs are replayed against the kernel rather than trusted as labels | All three hit the same exact obstruction: historical clause 1 has type `KernelTy::Type`, while the adopted diagram requires `KernelTy::El(owner)`. Identifier: `C-7-historical-point-clause-not-typed-as-owner-element-v1`. No witness is forged. |
| C-8 charging | Every registered diagram has replayable, reference-only provenance and zero boundary charge under `boundary-charge-zero-reference-only-v1`; a second adopted wrapper binds this evidence to the formula `1 + d^2` with `c(b) = 0`; non-reference content produces a hoisting error | The four registered-diagram formula/charge result is proved. Basis independence/exhaustiveness remains part of C-6. Transporting boundary provenance through the whole candidate quotient is not proved: `C8_CANDIDATE_BOUNDARY_PROVENANCE_JOIN`. |

The two adopted strings are used verbatim:

```text
pathcon-attachment-declared-bound-boundary-theory-axiom-v2
boundary-charge-zero-reference-only-v1
```

The C-7 result activates the reporting branch of falsifier F-B1. Following
the adopted rule, the checker was not loosened to make the historical point
clauses pass.

## Implemented surfaces

- `crates/pen-type/src/substitution.rs`: C-1 substitution and restricted
  support theorem.
- `crates/pen-type/src/fuel_composition.rs`: C-3 local-to-global fuel
  witnesses.
- `crates/pen-eval/src/naturality_basis.rs`: the deliberately restricted C-2
  renaming/weakening audit.
- `crates/pen-type/src/cubical/typed_boundary.rs`: adopted V2 boundary
  checking, reference-only provenance, and C-7 diagnostics.
- `crates/pen-search/src/candidate_join.rs`: schema-4 construction and replay.
- `agda/KernelBridge.agda`: intrinsically scoped substitution over the full
  current expression-constructor inventory, restricted variable-image sort
  cases, and finite fuel composition. It is not a dependent instance-typing
  theorem and explicitly contains no intended-semantic completeness or
  candidate-ranking theorem.

`pen-agda` exports `KernelBridge.agda` as a support module. The legacy
serialized clause artifacts remain stable because the newly observed total
fuel field is excluded from legacy serialization.

## Reproduction

The principal commands used were:

```powershell
cargo run -p pen-search --example ip1_candidate_join_v4 -- generate `
  docs\ip1_candidate_verdict_join_v3.json `
  docs\ip1_candidate_verdict_join_v4.json

cargo run -p pen-search --example ip1_candidate_join_v4 -- replay `
  docs\ip1_candidate_verdict_join_v3.json `
  docs\ip1_candidate_verdict_join_v4.json

cargo test -p pen-type --lib
cargo test -p pen-eval --lib
cargo test -p pen-search "candidate_join::tests::" --lib
cargo test -p pen-agda --lib
agda -i agda agda\KernelBridge.agda
```

The schema-4 replay reports:

```text
valid: true
exact_schema3_archive_bound: true
all_213_rows_joined: true
no_row_promoted_across_open_gap: true
full_candidate_extraction_join_proved: false
intended_semantic_candidate_join_proved: false
```

Current regression results:

```text
pen-type --lib:                         334 passed
pen-eval --lib:                         181 passed
pen-search candidate_join focused:       12 passed
pen-agda --lib:                           5 passed
Agda KernelBridge.agda:                 checked
schema-3 archival replay:               valid
schema-4 replay:                        valid
```

The repository-wide formatting check still reports unrelated pre-existing
formatting differences; `rustfmt --check` passes on the Agent C Rust files,
and `git diff --check` is clean.

## Recommended continuation

The long pole is now explicit rather than hidden in a Boolean. Define an
operational, typed grammar for *intended depth-two semantic schemas* and prove
that its naturality generators are complete. That task is specified in
`docs/step_15_completion_open_problem.md`.

C-7 has already rejected the adopted S1/S2/S3 base bindings relative to the
current kernel. Under F-B1, the interpretation must now be revised or
withdrawn by the user in a versioned successor; a richer dependent declaration
overlay may inform that successor, but must not silently upgrade the current
axiom. Only after a successor discharges C-6/C-7 can Agent A own the prescribed
create-new HIST-CERT rerun. No such rerun or downstream conclusion is claimed
here.
