# Law V2 lambda/unit semantic-audit V2 prototype result

## Status

This is a generic, unfrozen prototype result. It neither adopts nor freezes a
Law V2 semantic profile, creates a Profile A adapter, issues a live
\(\kappa\) or \(\nu\), nor changes the companion
`gf2-kernel-cost-lambda-unit-v2` rule.

The work preserves commit `1a120b0` and all earlier adjudications, prototype
results, H3/H4 artifacts, the Profile Registry, and Window Audit V1.

## Result

The false finite-substitution-monoid requirement has been removed from the
semantic successor. The prototype now separates:

1. finite, derivation-local substitutions consumed by rank-at-most-two
   constructors; and
2. generic laws for arbitrary well-typed simultaneous substitutions.

No V2 type has a global `Composite` construction-substitution variant, and no
V2 verifier computes a fixed-point closure of compatible substitutions.

The corrected chain advances through several positive authorities and then
halts before `VerifiedRewriteSystemV2` at a new, explicit theorem/API blocker
set.

## Positive authorities implemented

### Semantic manifest successor

`gf2-semantic-audit-lambda-unit-v2` is a distinct schema/digest with:

- direct construction substitutions only;
- the full generic substitution theorem obligation list;
- edge-local fresh matching;
- binder-local typed occurrences;
- independent historical subjects;
- weakening-image conservativity;
- exhaustive immediate-edge-pair overlaps;
- the existing projection-free lambda/unit surface; and
- a positively empty origin-cutoff Q3 rule.

The V1 proposal remains unchanged.

### Narrow V1-inventory/V2 compatibility

`VerifiedPublicInventoryCompatibilityV2` resolves the public-inventory
successor binding without casting the V1 capability. It verifies the exact
lambda/unit V1/V2 profile pair, the inventory-to-V1 binding, recursive
lambda/unit syntax, the projection-free boundary, universes, context bound,
Q0/eta/Q3 agreement, coverage, history, exact kernel boundaries, extension,
and normalizer protocol.

Its closed invariant list deliberately excludes V1 substitution-carrier
rules and all V2 construction-substitution, generic-substitution,
occurrence, matching, overlap, and conservativity claims.

### Public-clause authority

`VerifiedPublicClauseCensusV1` derives exactly one canonical clause identity
for every verified public declaration and equation. Each identity binds:

- inventory and coverage identity;
- source identity;
- origin event;
- normalized public subject;
- group membership;
- equation ownership where applicable; and
- the complete verifier-derived dependency support.

Clause identity is prior to kernel-cost classification. No caller clause ID
or completeness Boolean is accepted.

### Demand and semantic-seed authority

`VerifiedDemandAnchorCensusV1` has a positively verified empty base. A
nonempty predecessor demand does not mint this capability until demand-orbit
equivalence and typed realization are proved. A matching `DemandPortKeyV1`
alone has no effect.

For a V2-compatible inventory whose demand-anchor census is available,
`VerifiedSemanticSeedCensusV2` reconstructs every declaration/equation seed
and kernel-replays it. It derives:

- source identity;
- origin and event support;
- source clause;
- public support;
- demand anchor;
- local role; and
- exact source judgment.

The prototype test mints the complete census for an empty-demand inventory.
A nonempty port fixture returns the demand blockers instead.

### Direct construction substitutions

`VerifiedConstructionSubstitutionCensusV2` reconstructs, per carrier tuple:

- both registered context-amalgamation legs;
- the forced-newest argument substitution for each applicable application;
  and
- left and right one-hole filler substitutions for each applicable equation
  action.

Every image vector is kernel-checked and bound to its source tuple, contexts,
families, result, and kernel protocol. The census also checks that every
positive-rank V1 raw family is the unique result of one applicable tuple.

This is a relative witness census over the existing V1 pre-Q0 carrier. It is
not yet a native V2 rank-inductive carrier theorem.

### Generic raw substitution algebra

The pinned safe-Agda package proves, for the raw lambda/unit term grammar:

- identity, composition, and associativity;
- binder lifting and lifting/composition commutation;
- capture avoidance by construction;
- beta, closed public-delta, unit, and fresh-schema substitution stability;
- reflexive-transitive reduction transport; and
- both registered family-constructor naturality laws.

Rust checks arbitrary dependent simultaneous-substitution instances on
demand and implements identity, composition, lifting, and weakening without
enumeration. `VerifiedRawSubstitutionAlgebraV1` can be minted from the fixed
package. The stronger `VerifiedSubstitutionMetatheoryV1` cannot.

### Binder-local occurrence census

`VerifiedTypedOccurrenceCensusV1` recursively visits terms, equation sides,
and judgment types. It records distinct paths for functions, arguments,
binder parameters, binder bodies, equation sides, and judgment types.

Under `Pi` and `Lambda`, the body is checked in the kernel-normalized extended
context \(\Gamma,A\). Each record binds the exact local context, source term,
checked local judgment, and input/output kernel replay. No shifted
outer-context approximation is used.

### Historical empty base

`VerifiedHistoricalRewriteSystemV1` now has a positive equation-free base. It
binds the complete zero-equation census, exact predecessor history and
boundary, and the base Q0 inventory. A nonempty predecessor returns
`Unknown(MissingHistoricalRewriteAuthority)` unless a previously issued
rewrite theorem for that exact boundary is supplied.

## New blocker set

### B1 — dependent substitution typing

The safe-Agda package currently proves scope preservation, not the dependent
typing substitution lemma for sorts, variables, globals, `Pi`, `Lambda`,
application, unit type, and unit.

Consequently typed Q0 reduction stability cannot yet be derived. The full
gate returns:

```text
Unknown(MissingSubstitutionMetatheory)
```

Scope preservation and finite tests are not promoted to typing authority.

### B2 — nonempty demand orbits and realizations

The public inventory proves exact predecessor demand contracts and strict
prior port association, but it does not prove the typed-reindexing quotient
that forms demand orbits or a specialization from a semantic family to an
exact demand output.

The exact dispositions are:

```text
Unknown(MissingDemandOrbitAuthority)
Unknown(MissingDemandRealizationAuthority)
```

The empty demand base remains positive.

### B3 — native V2 rank-inductive carrier

The direct witness census is bound to a V1 pre-Q0 carrier whose internal
family identities still derive from V1 seed wires. Although the V2
semantic-seed census now exists for the empty-demand case, no theorem yet
reconstructs the complete rank-0/1/2 carrier natively from those V2 seed
identities or proves an exact correspondence to every V1 carrier identity.

The legacy upgrade gate therefore returns:

```text
Unknown(MissingRankInductiveCarrierTheorem)
```

The relative direct-witness census is not misreported as full carrier
authority.

### B4 — synthesis-dependent occurrence types

`pen-kernel` publicly checks a caller-supplied type but does not expose the
inferred type returned by variable lookup or a general synthesis operation.
Global-headed applications and bidirectional positions are covered; a
variable in function position requires unavailable inference.

Such a root returns:

```text
Unknown(MissingTypedOccurrenceCensus)
```

No hand-shifted context entry is substituted for the missing kernel result.

### B5 — nonempty historical predecessors

Stored historical equation syntax does not establish its orientation,
admissibility, or rewrite-system theorem. The empty base is complete; an
inductive nonempty predecessor requires the previously issued theorem for its
exact boundary and currently returns:

```text
Unknown(MissingHistoricalRewriteAuthority)
```

## Lawful halt

Because B1–B5 occur before the required rewrite-system chain closes, the
prototype does not mint:

- a native complete V2 carrier;
- exhaustive edge-local fresh matches;
- a complete finite reduction graph;
- overlap joins;
- weakening-image conservativity;
- `VerifiedRewriteSystemV2`;
- Q0 normalization authority; or
- any downstream Q1/Q2/Q3, cost-basis, marginal, SR2, transcript, adoption,
  freeze, or Profile A result.

Implementing those downstream layers while any prerequisite capability is
missing would weaken the authority boundary. The computation therefore halts
at this blocker set.
