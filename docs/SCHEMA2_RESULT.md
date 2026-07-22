# SCHEMA2 v1 phase-batch result

**Executed:** 2026-07-19  
**Disposition:** scoped E-1 operational fragment proved; global C1 remains open;
E-2 preflight stopped with two mandatory adjudications; E-3 through E-8 were
not executed.

## Artifact

- Certificate: `docs/schema2_v1.json`
- Schema: `schema2-phase-batch-v1`
- Bytes: `27265`
- SHA-256: `13352267D41415D49F263583B68A81F1CE30A8021372AF02552DA64A9EF003D4`
- Result digest:
  `blake3:25aac1ac064d808875785887d379cbc6cb5102bf73090220ac65e873ed4a38eb`
- Outcome: `partial_e1_scoped_fragment_e2_adjudication_required`

The artifact was emitted with create-new semantics. A second emit refused to
overwrite it, and strict replay returned `valid: true` with no errors.

## E-1: what was proved

The new `pen-schema` crate implements the current E-1 typed-expression
fragment:

- contexts whose entries are type parameters, opaque elements, rigid library
  references, interval variables, or cofibration assumptions;
- left-to-right dependent context formation and lookup;
- genuine typed non-variable images, including `point(a) : Trunc(A)`;
- identity, composition, weakening, legal adjacent exchange, and typing
  preservation;
- replayable finite support bounds for substitution;
- stable `(step, symbol)` library keys, including a collision regression that
  prevents a library dependency from being redirected through a binder ID;
- a read-only join to the four registered TRUNC endpoint maps: identity, swap,
  collapse-to-x, and collapse-to-y.

`agda/Schema2.agda` is checked with `--safe --without-K`. It has
entry-kind-indexed context payloads and an inductive legal-substitution type;
it proves the abstract identity/composition/preservation laws, legal exchange
inverse, the non-variable point instance, and the four TRUNC maps. It contains
no postulate.

This is deliberately a **scoped** theorem. The certificate records all of the
following separately:

```text
scoped_e1_fragment_proved                         = true
global_c1_retired                                = false
agda_to_rust_issuer_soundness_proved              = false
all_pen_core_expr_to_typed_e1_elaboration_proved = false
```

Consequently, full
`C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION` is not retired. Its two named
successor obligations are:

1. `C1_PEN_CORE_EXPR_TO_SCHEMA2_TYPED_EXPRESSION_BRIDGE`;
2. `E1_AGDA_RUST_OPERATIONAL_CORRESPONDENCE_NOT_FORMALIZED`.

The finite Rust tests cover every constructor in the current E-1 syntax and
the important mutations, but they are not a generated property theorem over
all future `Schema2` syntax. The Agda algebra is also broader than the Rust
issuer and does not yet prove issuer soundness.

## E-2 preflight: historical ordinary inventory

The audit reconstructed the HIST-CERT v3 ordinary rows from the archived row
keys, then compared the derived projection with the archive. Counts were
outputs of this reconstruction and were never used to choose a constructor.

| Step | Raw operational rows | Registered path subtotal | Ordinary unresolved rows |
| ---: | ---: | ---: | ---: |
| 5 | 7 | 2 | 5 |
| 6 | 8 | 2 | 6 |
| 7 | 10 | 5 | 5 |
| 8 | 18 | 10 | 8 |

The ordinary constructor inventory retained by the certificate is:

- fresh formation;
- point or unit introduction;
- **path-constructor introduction**;
- recursor;
- inductor;
- the single polymorphic Trunc parametric action at Step 6;
- Step-8 post-path operation, coherence, and cell action.

`PathConstructorIntro` is important: HIST-CERT v3 treats it as an ordinary
row, although the prose list in the Agent E plan omitted it. It was retained
as pending and was not silently absorbed into the registered path subtotal.
No ordinary typed-realizer or Agent A handoff token was issued.

## Mandatory adjudication 1: the Stage-1 Universe package

The Stage-1 diagnostic replayed exactly:

```text
clause 0 = Univ
clause 1 = App(Univ, Var(1))
EGP-v1 clause-local marginal families = 2
sealed family unit                    = 1
```

The cause is not safely described as merely "double-counting App". EGP-v1
created two distinct clause-local canonical families over the empty closure.
The legacy reading excludes arena formation and counts liveness, while burned
EGP-v2 instead credits `Univ` and covers `App(Univ,m)` by a registered
completion. The latter is diagnostic data, not an adopted theorem.

No adopted frozen source decides which package-level quotient is intended.
The certificate therefore records
`ADJUDICATION_REQUIRED_STAGE1_UNIVERSE_PACKAGE_BOUNDARY`, with no selected
option:

- **A:** only `App(Univ,m)` is the exported natural family; `Univ` is arena or
  formation infrastructure;
- **B:** `Univ` and `App(Univ,m)` form one typed formation/completion package;
- **C:** they are two distinct natural families.

An adopted resolution must state the canonical family, a typed coverage or
exclusion direction, naturality, and predecessor disposition. The historical
value `1` is not sufficient evidence for A or B.

## Mandatory adjudication 2: Step-8 post-path semantics

The archive names three rows positionally but does not define their typed
schema rules. The certificate records
`ADJUDICATION_REQUIRED_STEP8_POST_PATH_TYPED_SEMANTICS` and requires:

1. the typed natural-family signature of clause 3's `mu` operation;
2. the orientation and typed equality content of clause 4's unit/coherence
   row;
3. the typed clause-3 by clause-2 cell-action constructor;
4. a decision whether cell action is an independently exported inequivalent
   family or a definitional/naturality instance.

The hand reading in `HSPACE_ENUMERATION.md` and the archived cardinality do
not decide this quotient. No post-path constructor or token was minted.

## Strict-order disposition

The phase ledger is:

| Phase | Status |
| --- | --- |
| E-1 | `scoped_operational_fragment_complete_global_c1_open` |
| E-2 | `partial_adjudication_required` (preflight only while global E-1 remains open) |
| E-3--E-8 | `not_started_blocked_by_e1_global_gap_and_e2_adjudication_strict_order` |

Thus C2, general C6, D-4, domain-wide C3, and C8 remain open. F1 and F5 are
not executable, triggered, or excluded. Semantic `O(16)` remains undecided.
No schema-3 row was promoted.

The read-only later-phase audit also identified two specification-level
successor issues which were not executed here:

- E-5 still lacks an operational demand-scheme `D(B)` derivability decision;
- E-8's 213 archived rows are aggregate buckets, not individual telescopes.
  A sound successor needs a parent-row refinement/universal theorem rather
  than choosing one representative telescope per row.

## Verification

The following passed on the frozen artifact and source tree:

```text
cargo test -p pen-schema                         14 passed, 0 failed
cargo clippy -p pen-schema --all-targets --no-deps -- -D warnings
cargo fmt -p pen-schema -- --check
agda -i agda agda/Schema2.agda
cargo run -p pen-schema --example schema2 -- replay docs/schema2_v1.json
```

Certificate tests reject unknown or duplicate fields; scalar and nonempty
array mutations even after redigesting; source-pin changes; endpoint-map
changes; deleted `PathConstructorIntro`; adjudication selection; inserted
ordinary tokens; and promotion of any later phase.

## Conclusion boundary

This result contains no Step-16 acceptance verdict, no closing inequality,
and no global-halt or global-continuation conclusion. It did not run EGP-v3,
did not use a bar value in a constructor decision, and did not alter the
evaluator, token rules, or any archival result. Any continuation must use a
new successor artifact rather than overwrite `schema2_v1.json`.
