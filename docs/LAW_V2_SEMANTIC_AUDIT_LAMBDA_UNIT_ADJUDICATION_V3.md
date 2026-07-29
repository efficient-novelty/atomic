# Law V2 semantic-audit lambda/unit adjudication V3

**Date:** 2026-07-29

**Status:** `DRAFT_PROPOSED_NOT_ADOPTED`

**Proposed audit profile:** `gf2-semantic-audit-lambda-unit-v3`

**Preserved predecessor proposals:**
`gf2-semantic-audit-lambda-unit-v1` and
`gf2-semantic-audit-lambda-unit-v2`

## Authority boundary

This document proposes a semantic successor that corrects the ordering of
family identity and demand provenance. It does not edit, reinterpret,
supersede, adopt, or freeze V1 or V2. It changes no issued H3/H4 artifact,
Profile Registry entry, Window Audit V1 result, or registered
`UndefinedAudit` outcome.

V3 is a generic, unfrozen proposal. Verification of its exact manifest wire
grants no rewrite, quotient, cost, demand-realization, SR2, or Profile A
authority. No Profile A history, archived semantic vector, desired
continuation, or Stage-4 candidate may be consumed while developing it.

This document issues no live \(\kappa\), \(\nu\), marginal-family, ratio,
threshold, or Selective-Law result.

## Decision

V2 correctly separated finite construction substitutions from the generic
metatheory of arbitrary typed substitutions. V3 preserves that correction.

V2 nevertheless coupled a rank-zero seed to a demand anchor. Because demand
orbit and realization authority was unavailable, a nonempty equation port
prevented the semantic seed census, native carrier, and rewrite construction
from proceeding.

That coupling is not part of semantic-family identity. A demand realization
proves why a previously constructed family may receive credit for an old
demand. It does not create the family or distinguish two otherwise identical
families.

The binding V3 clauses are:

> Semantic seed and raw-family identity contains structural support only:
> public events and public declarations.

> Demand orbit, output, specialization, and realization data is assigned
> downstream, after the family quotient, weakening, and marginal-family
> identification, as evidence consumed by SR2.

Consequently a missing demand orbit or realization remains a genuine blocker
for final novelty and SR2, but it does not block seeds, the native carrier,
typed occurrences, the base rewrite theorem, or Q0.

## Preserved V2 surface

V3 preserves the projection-free lambda/unit grammar, rank bound, direct
construction-substitution inventory, generic substitution theorem
obligations, edge-local fresh-rule matching, binder-local occurrence
protocol, independent historical subjects, weakening-image conservativity,
overlap census, Q0 rule inventory, Q2 inventory, and positively empty Q3
rule of V2.

In particular, V3 still has:

```text
DirectConstructionWitnessesOnly

ContextEmbedding
ForcedNewestArgument
OneHoleInstantiation
```

It has no global substitution-composition carrier and no fixed-point closure
of inventoried substitutions.

Records, descriptors, fields, projections, and projection reductions remain
outside the fragment.

## Demand-neutral structural support

V3 splits the former public-support role into two capabilities.

Structural support is:

```rust
pub struct VerifiedStructuralSupportV1 {
    events: BTreeSet<EventIdV1>,
    declarations: BTreeSet<GlobalId>,
}
```

It is verifier-derived from the complete public-clause census. It participates
in semantic seed identity, raw-family identity, union, weakening, and
old-support tests. The type cannot represent a demand output.

Demand provenance is downstream evidence of the form:

```rust
pub struct VerifiedDemandProvenanceV1 {
    family_class: FamilyClassIdV3,
    orbit: DemandOrbitIdV2,
    output: DemandOutputIdV1,
    realization: VerifiedDemandRealizationV2,
}
```

This second capability may be minted only after a family class, a complete
demand-orbit census, and an exact typed realization exist. It is consumed by
SR2 and does not alter the family-class ID.

## Demand-neutral V3 seed base

The V3 seed base is reconstructed from:

- the verified V1 public inventory;
- the narrow V1-inventory/V2-profile compatibility capability;
- the V3 semantic manifest;
- the verifier-derived public-clause census;
- exact source identities and judgments;
- event/declaration structural support;
- constructor-derived local roles; and
- unchanged-kernel replay.

No demand-anchor census, orbit census, output census, or realization
certificate is an input.

Each seed has the authority-bearing shape:

```rust
pub struct VerifiedPreQ0SemanticSeedV3 {
    id: SeedIdV3,
    subject: PublicSemanticSeedSubjectV3,
    source_identity: Digest,
    origin_event: EventIdV1,
    source_judgment: GenericJudgmentV1,
    source_clause: ClauseIdV1,
    structural_support: VerifiedStructuralSupportV1,
    local_role: LocalRoleV1,
    kernel_replay_digest: Digest,
}
```

The V1 inventory and V1 public-clause identities bind the complete verified
wire. That complete wire includes equation-port metadata. Therefore neither
the V1 inventory digest nor the V1 clause ID may enter `SeedIdV3` or the V3
seed semantic digest. `source_clause` remains private verifier-derived lineage
and is bound by the enclosing census proof.

`SeedIdV3` and the semantic seed digest derive only from:

- the exact V3 manifest digest;
- the public semantic subject;
- source identity;
- origin event;
- source judgment;
- structural support;
- local role; and
- unchanged-kernel replay.

This makes the semantic identity invariant under a change to otherwise
non-authoritative PortKey metadata.

## Bound equation-port metadata

An inventoried PortKey remains useful evidence that a successor equation was
associated with an exact strict-prior demand contract. V3 preserves that fact
in a separate, verifier-derived metadata ledger.

The metadata record binds:

- the verified inventory digest;
- the equation ID;
- the verifier-derived source-clause ID; and
- the exact PortKey.

It grants no demand orbit, family class, typed specialization, discharge,
marginal, SR2, or novelty authority. It is absent from every V3 seed ID and
seed semantic digest. The enclosing seed-census digest binds the inventory
compatibility proof, public-clause census, seed-to-clause lineage, and complete
PortKey metadata ledger, preventing lineage substitution without contaminating
family identity.

If demand data is ever detected inside structural support, the verifier
returns exactly:

```text
Unknown(DemandAuthorityEnteredStructuralIdentity)
```

## V3 manifest

The exact manifest profile is:

```text
gf2-semantic-audit-lambda-unit-v3
```

It is schema version 3, generic-only, unfrozen, and has no live Profile A
access. In addition to the preserved V2 surface it binds:

```text
DemandNeutralStructuralSupport
BoundNonAuthoritative
PostQuotientWeakeningMarginalSr2Only
EmptyBaseOrExactIssuedPriorRewriteSystem
```

Its closed authority order is:

```text
public-clause census
-> demand-neutral semantic-seed census
-> lambda/unit typing-and-synthesis metatheory
-> native rank-inductive carrier
-> synthesis-backed typed-occurrence census
-> rewrite system
-> family quotient
-> weakening and restriction
-> marginal family set
-> demand-orbit census
-> demand-realization census
-> SR2 provenance assignment
```

Changing, omitting, duplicating, or reordering any manifest entry returns:

```text
Unknown(MalformedManifest)
```

## Shared typing and synthesis authority

Dependent substitution typing and synthesis-dependent occurrence typing are
one theorem/API boundary. V3 requires an isolated synthesis successor that
depends on, but does not modify, the issued `pen-kernel`.

Every synthesized judgment must carry a syntax-directed derivation and be
replayed as an exact `HasType` or `TypeFormation` judgment through the
unchanged kernel. A corresponding safe-Agda calculus must prove synthesis
soundness and completeness for the exact profile, dependent simultaneous
substitution, typed reduction stability, and family-constructor naturality.

Until that combined capability exists, the exact outcome is:

```text
Unknown(MissingLambdaUnitTypingMetatheory)
```

If a complete occurrence census has not yet been reconstructed from that
capability, the exact outcome is:

```text
Unknown(MissingSynthesisBackedTypedOccurrenceCensus)
```

## Native V3 carrier

The V3 carrier must be rebuilt from V3 seeds. It may not reinterpret a V1
family ID or promote the V2 relative direct-witness census.

Rank zero inserts every verified V3 seed exactly once. Rank one and rank two
exhaust every ordered source tuple and registered context/one-hole witness,
reconstructing the exact direct construction substitutions while processing
that tuple. Every tuple receives exactly one of:

```text
Applicable
CertifiedInapplicable
OutsideFragment
Unknown
```

Positive-rank identity binds the manifest, rank, constructor, ordered V3
source-family IDs, context-witness IDs, exact tuple-local substitutions, and
output judgment. No V1 family ID enters an authoritative V3 identity.

Until complete rank-by-rank coverage exists, the exact outcome is:

```text
Unknown(MissingNativeRankInductiveCarrierV3)
```

## Base and inductive historical authority

The verified equation-free predecessor is sufficient historical authority for
the first V3 rewrite theorem. Nonempty history is not a prerequisite for that
base case.

Historical authority is either:

1. the positively verified empty historical rewrite system; or
2. a capability minted from a previously issued V3 rewrite theorem whose
   successor boundary exactly equals the new predecessor boundary.

Stored historical equation syntax is never rewrite authority. Before an exact
prior V3 theorem exists, a nonempty predecessor returns:

```text
Unknown(MissingIssuedPriorRewriteSystemV3)
```

The first rewrite theorem must be constructed over the empty base, entered
into a private append-only theorem registry, and then used to verify a generic
second extension before the nonempty induction path is claimed.

## Rewrite, quotient, and downstream provenance

The native carrier and synthesis-backed occurrence census feed edge-local
beta, public delta, unit, and fresh-rule matching. Rewrite authority requires
complete edges, type preservation, an explicit decreasing rank, one reachable
normal form per node, exhaustive same/nested/disjoint overlap pairs with join
paths, and empty-base weakening-image conservativity.

Missing stages return exactly:

```text
Unknown(MissingRewriteSystemV3)
Unknown(MissingFamilyQuotientV3)
Unknown(MissingWeakeningMarginalAuthority)
```

Only after the rewrite theorem, Q0/Q1/Q2 family classes, weakening, and exact
marginal-family identification may demand authority be built. Missing
downstream stages return:

```text
Unknown(MissingDemandOrbitCensusV2)
Unknown(MissingDemandRealizationCensusV2)
Unknown(MissingSr2ProvenanceAssignment)
```

These outcomes do not invalidate or block the already verified seed base,
native carrier, occurrences, rewrite theorem, or quotient.

## Rust--Agda agreement and adoption

Targeted Rust and safe-Agda checks are development evidence. They are not
transcript agreement until both implementations independently consume the same
canonical inputs and compute the complete V3 outputs.

No V3 profile may be adopted or frozen, and no Profile A adapter may be
created, until typing/synthesis, native carrier, typed occurrences, rewrite
system, Q0/Q1/Q2/Q3, cost basis, weakening/retraction, exact marginals, demand
orbits and realizations, SR2, and full transcript agreement are complete and
independently reviewed.

## Preserved results

The V1 and V2 semantic proposals and prototype results remain unchanged. So do
all broader proposals, cost adjudications, issued H3/H4 results, the Profile
Registry, and Window Audit V1.

This V3 proposal does not assume a value for \(\kappa_4\) or \(\nu_4\), create
a bootstrap density, authorize a Selective-Law benchmark, or predict whether
a future frozen audit will be defined.
