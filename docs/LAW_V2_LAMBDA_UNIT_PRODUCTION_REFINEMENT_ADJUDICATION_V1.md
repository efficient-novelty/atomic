# Law V2 Lambda/Unit Production Refinement Adjudication V1

Status: additive generic adjudication; not adopted; not frozen

Profile under refinement:

```text
gf2-semantic-audit-lambda-unit-v3
```

## Decision

This adjudication fixes the production-code correspondence problem inside the
existing V3 semantic successor. It does **not** create a V4 manifest, profile,
carrier, rewrite system, or authority stage.

The refinement lane is generic-only. It may inspect only verifier-produced
generic inputs. It may not read Profile A, archived continuation values,
desired Stage-4 outputs, live `kappa`, live `nu`, or any adopted semantic or
cost vector. Successful inventory checking is not adoption, freezing,
transcript agreement, or permission to construct a Profile A adapter.

V1, V2, and the V3 manifest remain unchanged.

## Production syntax

The production correspondence target uses a separate, finite code grammar.
`PTm` is not identified with the existing abstract Agda `Tm` merely because
the constructor names resemble one another.

```text
PTm ::=
    PSort(level)
  | PVar(index)
  | PGlobal(slot)
  | PUnitType
  | PUnit
  | PPi(parameter, body)
  | PLambda(parameter_type, body)
  | PApply(function, argument)
```

`Sigma`, pairs, projections, recursive eliminators, metavariables, implicit
arguments, eta rules, and unregistered syntax are not `PTm`.

```text
PCtx ::= [A0, A1, ..., An-1]
PSig ::= [D0, D1, ..., Dm-1]
Di   ::= (type : PTm, body : Option<PTm>)
```

`PCtx` is stored oldest first. Appending a binder extends the context. A
de Bruijn code `PVar(0)` denotes the newest entry. More generally,
`PVar(i)` selects position `len(PCtx) - 1 - i`, and its stored type is shifted
by exactly `i + 1` binders before it is returned. An underflow, overflow, or
out-of-range index is not a failed proof; it is outside the production
judgment grammar.

## Declaration-order global slots

Production globals are declaration-order slots, not hashes:

```text
PGlobal(0), PGlobal(1), ..., PGlobal(m - 1).
```

For a verified Rust signature with declarations
`[d0, d1, ..., dm-1]`, the bridge derives the unique table

```text
slot i <-> declarations[i].id.
```

The table must bind the exact ordered verified-signature digest, every slot,
every `GlobalId`, every normalized declaration type, and every optional
normalized body. It must prove:

1. all verified declaration identifiers are distinct;
2. the table has exactly `m` entries;
3. every Rust global occurring in the translated subject has exactly one
   slot;
4. every production slot translates back to exactly one verified Rust global;
5. a declaration at slot `i` refers only to slots strictly smaller than `i`;
6. no caller ordering, digest ordering, or map iteration order changes a slot.

Missing, duplicate, extra, out-of-range, or forward global references return a
typed failure. A bare `GlobalId` digest is never declaration-order evidence.

## Universe boundary

The public lambda/unit subject grammar admits exactly:

```text
PublicUniverse = {0, 1}.
```

The closed range of inferred universe codes needed to type those subjects is:

```text
InferredUniverse = {0, 1, 2}.
```

In particular:

```text
PSort(0) => PSort(1)
PSort(1) => PSort(2).
```

`PSort(2)` may occur only as a checker-produced inferred type or as an
explicitly bound expected result of such inference. It is not a public input
sort and may not be used to synthesize `PSort(3)`. No certificate may contain
a universe outside `{0,1,2}`, and no public subject may contain a sort outside
`{0,1}`.

This distinction is mandatory. Treating the manifest's public
`universe_levels = [0,1]` as if checker outputs were also limited to `[0,1]`
would incorrectly reject `PSort(1)`. Allowing `PSort(2)` as new public syntax
would silently enlarge V3.

## Exact Q0 production classification

The V3 Q0 inventory has exactly seven tags in the following order and with the
following classes:

| V3 Q0 tag | Production class |
|---|---|
| `DeBruijn` | representation |
| `SequentialSubstitution` | representation |
| `Beta` | base-semantic |
| `ProvenancePreservingDelta` | base-semantic |
| `Unit` | base-semantic |
| `TelescopeFlattening` | representation |
| `FreshNonrecursiveConstructorComputation` | runtime-public |

The classes have different authority:

- **representation** steps certify the unique production encoding of binders,
  simultaneous substitutions, and telescopes;
- **base-semantic** rules are intrinsic lambda/unit computation, unit
  canonicality, and policy-authorized predecessor-public delta;
- **runtime-public** rules require an exact inventoried fresh-equation
  capability in addition to typing.

`DescriptorForcedProjection` is not in V3. A bridge containing it, omitting
one of the seven tags, duplicating a tag, changing the order, or assigning a
different class fails closed.

This classification is an inventory theorem only. It does not establish
termination, confluence, complete matching, Q0 authority, or equality between
the Rust and Agda step relations.

## Conversion certificate grammar

V2 base conversion is witnessed by a typed common-normal-form certificate.
There is no unchecked boolean conversion oracle. Its step grammar is
deliberately smaller than the Q0 inventory: it contains beta,
policy-authorized predecessor-public delta, and congruence only.

```text
PPosition ::=
    root
  | pi-parameter(PPosition)
  | pi-body(PPosition)
  | lambda-parameter(PPosition)
  | lambda-body(PPosition)
  | apply-function(PPosition)
  | apply-argument(PPosition)

PBaseConversionRedex ::=
    beta(argument, binder_substitution)
  | predecessor-public-delta(
        slot,
        predecessor-public-membership,
        policy-authorization)

PConversionStep ::= at(PPosition, PBaseConversionRedex)

PTrace ::= [] | PConversionStep :: PTrace

PConversionCertificate ::= common-normal-form(
    context,
    replay_mode,
    left,
    right,
    common,
    left_trace,
    right_trace)

PConversionReplayMode ::=
    type-formation
  | has-type(expected_type)
```

The checker must replay every trace endpoint, binder-local context, typed
substitution, predecessor-public lookup, policy authorization, and
intermediate typing judgment. Both traces must end at the byte-identical
canonical `PTm` named by `common`. Reflexivity is represented by empty traces,
and symmetry may swap the two traces of the same checked certificate.

Synthesis application compares checker-produced *types*, so both of its
conversion certificates use `type-formation`. This is significant for the
universe boundary. A public term can synthesize a type containing `Sort(2)`;
forming that type may require the unchanged kernel to infer `Sort(3)`
internally. The certificate does not serialize that internal formation
universe, and `Sort(3)` is not admitted to `PTm`. A term-level conversion uses
`has-type(expected_type)` and must bind the exact expected type. The replay
mode is verifier-checked data, not a caller claim that bypasses typing.

Transitivity is not inferred from two unrelated common-normal-form
certificates. It requires a new checked certificate whose adjacent traces
share and replay the same exact normal endpoint. A more general transitivity
theorem remains in the production-refinement frontier until deterministic
normalization and conversion correspondence are proved.

De Bruijn normalization, sequential substitution, and telescope flattening
are representation-bridge obligations, not steps in this base conversion
trace. Unit is an inventoried base-semantic canonicality rule but has no
reduction constructor in the current base trace. Fresh nonrecursive
constructor computation is a runtime rewrite-schema obligation and likewise
does not enter base conversion. Neither representation evidence nor a fresh
runtime schema becomes a semantic-family edge merely by being inventoried.

## Synthesis certificate grammar

The exact syntax-directed synthesis grammar has eight rules:

```text
PSynthesisCertificate ::=
    sort(level)
  | unit-type
  | unit
  | variable-lookup(index, selected_entry, shift_witness)
  | global-lookup(slot, declaration_witness)
  | pi-formation(
        parameter_certificate,
        parameter_formation_replay,
        body_certificate,
        result_universe)
  | lambda-introduction(
        parameter_certificate,
        parameter_formation_replay,
        body_certificate)
  | application-elimination(
        function_certificate,
        function_type_conversion,
        argument_certificate,
        argument_check_conversion,
        result_substitution_witness)
```

Every node binds:

- the exact verified signature and `PCtx`;
- its subject `PTm`;
- its inferred `PTm`;
- its rule tag and ordered premise certificates;
- every conversion certificate used by that rule;
- every unchanged-kernel input/output replay; and
- the kernel and synthesis protocol digests.

The variable rule must perform the exact newest-first de Bruijn selection and
`i + 1` shift described above. The global rule must use the declaration-order
slot table. Application must convert the synthesized function type to an
actual `PPi`, check the argument against its parameter, and substitute the
argument into the dependent result. A mirror-produced inferred type has no
authority until the unchanged kernel accepts the corresponding judgment.

No fallback expected type, caller-provided annotation, search rule, or
unrecorded normalization may fill a missing synthesis premise.

## Exact family-constructor mapping

The production family code inventory contains exactly three codes in this
order:

```text
Seed
GenericPublicApplication
GenericEquationAction
```

Every current family-constructor shape maps as follows:

| Family constructor shape | Production family code |
|---|---|
| `PublicHeadSeed` | `Seed` |
| `PublicEquationSeed` | `Seed` |
| `GenericPublicApplication` | `GenericPublicApplication` |
| `GenericEquationAction` | `GenericEquationAction` |

The mapping erases only the constructor payload for classification. It does
not reuse a V1 seed ID, V1 family ID, V2 tuple ID, or caller tag as a V3 family
identity. A native V3 family retains its V3 seed, source-family,
context-witness, direct-substitution, structural-support, source-judgment, and
kernel-replay bindings independently of this three-code classifier.

An exact bridge must verify both directions:

1. every constructor shape has exactly one production code; and
2. each of the three production codes is represented by the manifest's exact
   ordered derivation-rule inventory.

Any extra, missing, duplicate, reordered, or differently mapped code fails
closed.

## Inventory bridge capability

The additive Rust lane may mint a private
`VerifiedProductionInventoryBridgeV1` only from an already verified exact V3
manifest. The capability binds:

- the semantic-manifest candidate digest;
- the exact seven-tag Q0 sequence and its classifications;
- the exact three-code family sequence;
- the four-shape-to-three-code family mapping;
- separate no-extra/no-missing evidence for Q0 and family inventories; and
- a canonical bridge digest.

Private fields and the absence of deserialization are required. Equality of
expected and observed ordered sequence digests, together with equal counts,
is the no-extra/no-missing witness. It is not sufficient to check only set
membership or only the number of entries.

This capability is a production-code inventory bridge. It is deliberately too
weak to mint `VerifiedLambdaUnitTypingMetatheoryV1`,
`VerifiedRankInductiveCarrierV3`, a complete occurrence census,
`VerifiedRewriteSystemV3`, or Q0.

## Generic checker soundness is not subject completeness

Two theorems must remain separate.

**Generic checker soundness** says:

```text
if the production certificate checker accepts
  (verified signature, PCtx, PTm, certificate),
then the unchanged kernel accepts the translated judgment
and the corresponding abstract typing/reduction judgment is derivable.
```

This theorem is quantified over a supplied subject. It does not say which
subjects must be supplied.

**Carrier-derived subject completeness** says:

```text
the root inventory is derived from every raw family in the native V3 carrier,
every required judgment component and binder-local subterm occurs exactly
once, and every such root has an accepted production certificate.
```

This theorem requires the still-unminted native V3 carrier and an exhaustive,
carrier-derived root inventory. Testing selected roots, including every
current fixture, proves neither subject completeness nor a complete typed
occurrence census.

Consequently generic soundness may be developed before the carrier, while
complete-census authority must continue to return:

```text
Unknown(MissingSynthesisBackedTypedOccurrenceCensus)
```

until the full metatheory, native carrier, and carrier-derived exhaustive root
inventory all exist.

## Safe-Agda boundary

Safe Agda may presently prove:

- total classification of an internal seven-tag Q0 code inventory;
- the three-way classification of the existing abstract `Family`
  constructors;
- preservation of the abstract family code by family substitution; and
- classification of the existing abstract beta, public-delta, and
  fresh-equation step constructors.

Those proofs do not yet establish:

- a total erasure between Rust `Term`/`DependentContext` and `PTm`/`PCtx`;
- the declaration-order `GlobalId`-to-slot bijection;
- equality of Rust and Agda universe checking or conversion;
- correspondence of all seven production Q0 rules to an Agda relation;
- equality of Rust family payload codes and the abstract Agda family codes;
- certificate-checker soundness for kernel conversion; or
- carrier-derived subject completeness.

These are explicit production-correspondence obligations, not axioms. No
postulate, unsafe pragma, reflection escape hatch, or constructor with a
theorem-like name may stand in for them.

## Current lawful stop

The inventory bridge can be positive while the combined production
metatheory remains:

```text
Unknown(MissingLambdaUnitTypingMetatheory)
```

The remaining production-refinement frontier is:

1. `PTm`/`PCtx` erasure and declaration-slot correspondence;
2. kernel conversion and certificate-checker correspondence;
3. Rust synthesis derivation-code correspondence;
4. exact Q0 semantic-relation correspondence;
5. exact runtime family-payload correspondence; and
6. carrier-derived subject completeness.

No native carrier, rewrite theorem, family quotient, demand authority, live
semantic value, live cost value, adoption, freeze, or Profile A adapter is
issued by this adjudication.
