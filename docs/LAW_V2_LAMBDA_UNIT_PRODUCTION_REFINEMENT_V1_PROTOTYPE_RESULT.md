# Law V2 lambda/unit production-refinement V1 prototype result

## Status

This is a generic, unfrozen, non-authorizing prototype result for:

```text
law-v2-lambda-unit-production-refinement-v1
```

It refines the existing proposed semantic profile:

```text
gf2-semantic-audit-lambda-unit-v3
```

It does not create V4, adopt or freeze V3, create a Profile A adapter, issue
live `kappa` or `nu`, or modify the protected `pen-kernel`.

## Result

The prototype now has a canonical proof-carrying production surface on both
sides of the Rust/Agda boundary:

```text
untrusted Rust certificate producer
  -> independent Rust certificate checker
  -> finite intrinsically scoped Agda mirror
  -> existing abstract lambda/unit calculus
```

The Rust checker, exact finite inventories, and safe-Agda structural theorems
are positive prototype evidence. They do not by themselves prove that the
Rust checker and Agda mirror denote the same judgments. The four
cross-language correspondence capabilities therefore remain opaque and
unminted, and the aggregate diagnostic stops at:

```text
MissingProductionCorrespondences
```

No digest, successful typecheck, caller Boolean, rule label, generated
expected answer, or fixed-root traversal can replace one of those
capabilities.

## Implemented production certificate protocol

`pen-kernel-synthesis/lambda-unit/v2` preserves V1 and adds an
authority-free serializable certificate wire with exactly eight synthesis
constructors:

```text
Sort
UnitType
Unit
VariableLookup
GlobalLookup
PiFormation
LambdaIntroduction
ApplicationElimination
```

The independent checker reconstructs the term, context, inferred type,
premise contexts, binder extensions, conversions, substituted result, and
rule tag. Only accepted codes produce opaque, non-deserializable
`VerifiedSynthesisCodeV2` capabilities.

Base conversion certificates contain:

- explicit beta and policy-authorized transparent-delta redexes;
- congruence paths under `Pi`, `Lambda`, and `Apply`;
- two typed traces to one byte-identical common normal form;
- a complete no-applicable-redex census;
- exact replay of every outer intermediate with the unchanged kernel; and
- a distinct `TypeFormation` or exact `HasType(expected_type)` endpoint
  judgment.

All replay outputs are checked back into the lambda/unit syntax boundary.
Public subjects and contexts admit only `Sort(0)` and `Sort(1)`.
Checker-produced term/type payloads may additionally contain `Sort(2)`.
The unchanged kernel may internally infer the higher universe needed to form
such a type; that internal universe is not serialized as new public syntax.

Transparent delta remains relative to an explicit declaration-slot policy.
The synthesis crate does not infer predecessor-public authority by scanning
bodyful declarations. The semantic theorem layer separately binds all and
only bodyful predecessor declarations from a verified public ledger,
requires the exact V1-to-V2 inventory compatibility proof, and transports
that binding through the exact V2 and V3 manifest identities before it can
clear the V3 delta-policy sub-frontier.

Exact binder-local kernel replay for nested congruence premises remains
explicitly unproved. Outer trace replay is not presented as that stronger
theorem.

## Implemented finite production mirror

The safe-Agda package adds:

```text
PTm global-count local-count
PCtx global-count local-count
```

`PCtx` is a snoc context stored oldest first. `Fin local-count` variables use
zero/newest de Bruijn semantics, and `Fin global-count` globals are finite
declaration-order slots.

Without postulates or unsafe checker options, the package proves:

- finite ordinal encode/decode and production-image term reification;
- the exact newest-first variable/oldest-first context lookup equation;
- context extension correspondence;
- renaming, weakening, simultaneous substitution, and binder lifting
  correspondence;
- structural term and synthesis-shape round trips;
- total, unique, injective tags `0..7`, with all larger tags rejected;
- an intrinsically scoped eight-constructor synthesis-code shape with
  tag-preserving erasure;
- typed common-normal-form conversion and substitution stability;
- exact decoded typing soundness for sort, unit type, unit, variable lookup,
  `Pi` formation, and lambda introduction;
- native conversion-aware application elimination, including preservation
  under substitution and unconditional application soundness from the two
  checked type-formation conversions;
- conditional finite-global soundness from an explicit slot-typing bridge;
  and
- separate existential-universe `TypeFormation` and arbitrary-expected-type
  `HasType` conversion judgments.

The separate inventory entry proves exact classification of all seven V3 Q0
tags, all three production family codes, all four abstract constructor
shapes, and the existing family-naturality identities. These are inventory
and abstract-structure theorems, not an assertion that Rust payloads have
already been decoded into them.

## Implemented Rust refinement foundations

The semantic-audit prototype now verifies:

- a declaration-order `GlobalId` slot table with exact coverage,
  bidirectional lookup, and strict-prior dependency retention;
- the public/inferred universe split `{0,1}` and `{0,1,2}`;
- the exact seven-rule V3 Q0 inventory and its representation,
  base-semantic, and runtime-public classification;
- the exact three-rule family inventory and four-to-three constructor-shape
  mapping;
- exact V2 synthesis protocol and eight-tag identity;
- pinned safe-Agda production and inventory packages; and
- exact V3 transport of the predecessor-public transparent-delta policy.

The theorem layer defines opaque, non-deserializable surfaces for:

```text
VerifiedFiniteContextCorrespondenceV1
VerifiedKernelBaseConversionCorrespondenceV1
VerifiedSynthesisCodeCorrespondenceV1
VerifiedV3InventoryCorrespondenceV1
VerifiedLambdaUnitProductionRefinementV1
```

There is deliberately no constructor or verifier that fabricates these
objects from their component digests. Even after every positive prerequisite
is replayed, the aggregate verifier returns the open correspondence frontier.

## Exact blocker set

### 1. Rust finite syntax and global-slot decoding

The Agda finite syntax is proved internally, and the Rust slot table is
verified internally, but no checked common wire yet proves total equality
between:

- Rust `Term` and `DependentContext` encodings;
- Agda `PTm` and `PCtx`;
- Rust `GlobalId` entries and Agda finite global slots; and
- Rust variable metadata (`index`, oldest-first ordinal, and shift distance)
  and the corresponding Agda equations.

This requires an exact canonical-input decoder and transcript theorem, not a
digest comparison.

### 2. Kernel base-conversion correspondence

The Rust checker validates concrete traces, and Agda proves preservation for
its abstract typed traces. The bridge still lacks:

- exact decoding of every serialized reduction step and replay judgment into
  the Agda relation;
- binder-local unchanged-kernel replays for nested congruence premises;
- a Rust/Agda binding for the existential formation universe used by
  `TypeFormation`; and
- a proof that policy-authorized Rust delta steps are exactly the abstract
  predecessor-public delta steps.

### 3. Exact synthesis-code correspondence

The eight tags and structural arities agree, but no common checked wire yet
relates every Rust payload to the intrinsically scoped Agda code. In
particular, exact conversion payloads, variable metadata, global identifiers,
premise judgments, and application result substitution still need a
cross-language decoder and soundness transcript.

Until that exists, structural Agda reification is not full checker
soundness, and no `VerifiedSynthesisCodeCorrespondenceV1` may be minted.

### 4. Exact V3 Q0 and family payload correspondence

The Rust and Agda inventories are each exact, but their semantic payload
relations are not yet identified. The remaining theorem must connect:

- de Bruijn, sequential-substitution, and telescope representation laws;
- beta, provenance-preserving delta, and unit semantic rules;
- every exact typed fresh nonrecursive constructor schema, including owner,
  constructor, left-linearity, non-recursion, coverage, and substitution
  stability; and
- every concrete family payload to `Seed`, `GenericPublicApplication`, or
  `GenericEquationAction`, transporting family naturality through that exact
  decoding.

### 5. Carrier-derived subject completeness is downstream

Generic certificate soundness is separate from proving that one native V3
carrier supplied every required root. The carrier does not yet exist, so its
authoritative root domain cannot yet be compared with a certificate-root
set. A caller-provided root slice, including an empty one, remains
non-authoritative.

## Lawful halt

Because the four generic correspondence capabilities are unminted, this turn
does not mint:

- `VerifiedLambdaUnitProductionRefinementV1`;
- `VerifiedLambdaUnitTypingMetatheoryV1`;
- a native V3 rank-0/1/2 carrier;
- a complete carrier-derived typing-subject census;
- a V3 rewrite system or family quotient;
- weakening, marginal, demand-provenance, or SR2 authority;
- transcript agreement, adoption, or freeze; or
- a Profile A adapter or any live semantic/cost value.

The existing V1/V2/V3 proposals and results, H3/H4 artifacts, Profile
Registry, Window Audit V1, and protected kernel remain preserved. The next
lawful implementation step is the common Rust/Agda certificate decoder and
transcript theorem. Native carrier construction remains blocked until the
combined production-refinement capability can actually be minted.

## Validation

The prototype was checked with isolated Rust formatting, tests, clippy with
warnings denied, direct safe-Agda checks for both entry modules, nested
workspace isolation tests, the Law V2 firewall, Window Audit replay, escape
hatch scans, and protected-file comparison against commit `786303e`.

These checks are development evidence only. They do not adopt the proposal or
substitute for the four missing correspondence proofs.
