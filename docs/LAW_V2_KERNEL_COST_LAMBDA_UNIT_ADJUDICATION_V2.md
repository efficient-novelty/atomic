# Law V2 kernel-cost lambda/unit adjudication V2

**Date:** 2026-07-29

**Status:** `DRAFT_PROPOSED_NOT_ADOPTED`

**Proposed cost profile:** `gf2-kernel-cost-lambda-unit-v2`

**Broader predecessor proposal:** `gf2-kernel-cost-core-v2`

**Companion audit profile:** `gf2-semantic-audit-lambda-unit-v1`

## Authority boundary

This document proposes a projection-free successor profile for the first
finite kernel-cost theorem. It does not edit, narrow, supersede, adopt, or
freeze
[`gf2-kernel-cost-core-v2`](LAW_V2_KERNEL_COST_ADJUDICATION_V2.md).
That broader proposal remains unchanged and may continue independently.
The historical V1 proposal and every issued result also remain unchanged.

This proposal has no adopted manifest or digest. It consumes no Profile A
history, H3/H4 artifact, registered act, archived semantic vector, Stage-4
candidate, desired ratio, or desired continuation result. It authorizes no
Acts 1--4 calculation and issues no \(\kappa\), \(\nu\), ratio, threshold,
bootstrap density, or Selective-Law result.

The issued H3/H4 results, the Profile A registry, and
[`LAW_V2_WINDOW_REGISTER_AUDIT_V1.md`](LAW_V2_WINDOW_REGISTER_AUDIT_V1.md)
remain unchanged. In particular, this proposal does not alter the registered
`UndefinedAudit` outcome.

## Decision

The first complete V2 cost-basis theorem should be proved for the same
independently meaningful projection-free surface as the companion
[`lambda/unit semantic-audit proposal`](LAW_V2_SEMANTIC_AUDIT_LAMBDA_UNIT_ADJUDICATION_V1.md):

> finite dependent lambda calculus with universe levels 0 and 1, unit,
> transparent definitions and aliases, opaque operations, and fresh
> nonrecursive public equations.

Kernel cost remains the cardinality of a unique, exact,
predecessor-relative basis of first-irreducible public clauses. It is not a
count of declarations, syntax nodes, generated files, demand entries, or
visible equations.

A separately sealed public equation owned by a bodyless fresh head is a
separate paid public clause unless the exact normalized equation is
predecessor-public or a complete Q2 witness proves that it is a duplicate
presentation. Compiler generation, a prior demand, role metadata, provenance,
or rewrite admissibility does not make that behavioral commitment free.

## Exact supported surface

The term surface contains exactly:

```text
Sort(0)
Sort(1)
UnitType
Unit
Variable(index)
Global(id)
Pi(parameter_type, body_type)
Lambda(parameter_type, body)
Apply(function, argument)
```

The public cost surface may contain:

- verified transparent public definitions and aliases;
- verified opaque bodyless public operation heads;
- separately sealed public equations;
- fresh equations only when the companion audit verifies them as typed,
  left-linear, nonrecursive, nonoverlapping, and conservative over
  predecessor-supported terms;
- finite declaration groups;
- equation-free operation-role metadata with no record field, descriptor, or
  projection payload;
- exact conservative public extensions;
- predecessor-visible demand contracts, which may type or identify an
  obligation but do not themselves discharge it; and
- a positively verified-empty origin-cutoff Q3 registry.

The profile contains no `Sigma`, record, record constructor, record
descriptor, record field, projection term, projection clause, or
projection-reduction witness. A metadata label cannot move any such object
inside the fragment.

## Exact public API and raw clauses

Let \(\mathrm{ExactAPI}(H,x)\) contain every:

- source-level public export paired with its normalized declaration;
- separately sealed public equation paired with its normalized judgment; and
- equation-free public-interface or operation-role descriptor admitted by
  the closed lambda/unit metadata grammar.

Source identity, source syntax, normalized syntax, and the checked
source-to-normal derivation remain bound as one raw clause. Source and
normalized presentations are not charged separately.

The raw clause grammar is exactly:

```text
PublicDeclaration {
    head,
    source_identity,
    source_type,
    source_body,
    normalized_type,
    normalized_body,
    source_to_normal_derivation,
    public_group,
    equation_free_lambda_unit_metadata,
}

PublicEquation {
    equation_id,
    source_identity,
    source_left,
    source_right,
    source_type,
    normalized_left,
    normalized_right,
    normalized_type,
    source_to_normal_derivation,
    owner_head,
}
```

There is no projection-clause constructor in this grammar. A bodyful public
definition is one declaration clause; its implicit ordinary beta behavior is
not a second raw clause. A bodyless head and every separately sealed public
equation owned by it are separate raw clauses before classification.

Every in-fragment clause receives exactly one disposition:

```text
FirstIrreducible
TransparentAlias
ForcedDefinitionalCompletion
DuplicatePresentation
OutsideFragment(reason)
Unknown(reason)
```

`OutsideFragment` and `Unknown` abort the complete cost audit. Neither is a
zero-cost classification.

## Projection boundary

The lambda/unit free-completion manifest contains no
descriptor-generated projection rule. The verified inventory supplied to
this profile must have an exhaustively empty forced-projection census.

Any of the following returns exactly:

```text
OutsideFragment(DescriptorProjection)
```

- a nonempty forced-projection inventory;
- a raw forced-projection public clause;
- a record or descriptor that introduces a field or projection;
- a projection term or projection-reduction witness; or
- a claimed descriptor-forced-projection reconstruction.

This disposition aborts the complete cost audit. It is not `Unknown`, a
certified absence, a free clause, or permission to omit the object from the
public census.

## Lambda/unit free-completion manifest

The complete cost-free rule set is exactly:

```rust
enum FreeCompletionRuleLambdaUnitV2 {
    OrdinaryBetaOfBodyfulDefinition,
    PriorPublicTransparentAlias,
    DuplicatePresentationDeletion,
}
```

The rules mean:

1. **Ordinary beta of a bodyful definition.** Implicit typed beta behavior
   follows structurally from the verified transparent body. If that behavior
   is also represented by a separate public equation, it is
   `ForcedDefinitionalCompletion` only when the exact inventory-bound
   delta/beta derivation reconstructs it.
2. **Prior-public transparent alias or replay.** A declaration whose exact
   normalized target is proved predecessor-public or dependency-prior is a
   `TransparentAlias`. A sealed equation receives this disposition only when
   its exact normalized equation is predecessor-public.
3. **Duplicate-presentation deletion.** A clause is
   `DuplicatePresentation` only when an exhaustive Q2 witness proves the same
   normalized public content and public-supported provenance is already
   represented.

There is no `CertifiedFreshConstructorComputation` rule. There is also no
projection rule in this manifest. Arbitrary eta, unregistered theorem search,
later history, demand prediction, compiler convention, candidate-local
metadata, and post-hoc provenance are not free-completion rules.

## Verified public inventory

Cost classification consumes only a private verifier-minted public-inventory
capability. Its unchecked wire is replayed against:

- the exact predecessor history and public boundary;
- the exact successor boundary and conservative append;
- every public group, declaration, and equation;
- every source identity and source-to-normal judgment;
- declaration and equation origins;
- predecessor-visible demand contracts with exact family/output `PortKey`s;
- the canonical dependency DAG and verifier-derived availability;
- the exhaustive empty forced-projection census; and
- the positively verified-empty origin-cutoff Q3 registry.

Private verified fields and replayed contents, rather than caller labels,
caller availability, digest claims, or `complete = true` flags, carry
authority. Inventory closure is only relative to the supplied generic ledger;
this proposal does not claim that a ledger is an issued history.

## Ambient first-export census

`Sort(0)`, `Sort(1)`, `UnitType`, and `Unit` are ambiently formable but are
not automatically public or cost-free. A primitive is publicly exported only
when the complete normalized body of a verified public declaration is that
primitive. Occurrence in a type, proper subterm, verifier implementation, or
unexported intermediate reduction is not a public export.

Before classifying such a declaration, a private
`VerifiedAmbientPrimitiveExportClassV1`-style capability must:

1. replay all predecessor public declaration bodies;
2. replay all successor-new public declaration bodies;
3. retain source-to-normal provenance;
4. form complete Q0/Q2 export classes; and
5. distinguish a predecessor re-export from the first successor export
   class.

Simultaneous equivalent first exports form one Q2 class. No serialized order,
hash, or caller `first_export` flag selects a representative. A missing or
incomplete census returns
`Unknown(MissingAmbientFirstExportInventory)` and does not infer a paid
clause.

## Ordinary beta reconstruction

When a separately represented public equation is claimed to be the ordinary
behavior of a bodyful definition, a private
`VerifiedOrdinaryBetaDerivationV1`-style capability must reconstruct from the
verified inventory:

- the definition body and binder telescope;
- the complete typed argument spine;
- the left-to-right sequential substitution;
- the substituted body and result type; and
- the exact kernel-replayed delta/beta/congruence reduction path.

A caller tag asserting that beta occurred confers no authority. If the exact
derivation is unavailable, the in-fragment case returns
`Unknown(MissingOrdinaryBetaDerivation)`. Implicit beta with no separately
sealed public equation remains structural behavior and creates no additional
raw cost clause.

## Bodyless fresh equations

For each separately sealed public equation owned by a bodyless fresh head,
the verifier applies this total decision:

1. if the exact normalized equation is already public at the predecessor
   boundary, classify the new presentation as `TransparentAlias`;
2. otherwise, if a complete Q2 witness proves that it duplicates an already
   represented normalized public equation, classify it
   `DuplicatePresentation`;
3. otherwise, after complete inventory coverage and exhaustion of all three
   lambda/unit free rules, classify it `FirstIrreducible`; and
4. if any inventory, availability, duplicate, or negative decision is
   incomplete, return `Unknown`.

The bodyless head and its equation remain independent public commitments.
Their types and associated demand contracts may be required to type-check the
equation, but they do not reconstruct its behavior.

## Exact basis theorem

After authorized Q0 normalization and complete Q2 presentation quotienting,
\(S\) is an admissible lambda/unit basis exactly when

\[
  \mathrm{Free}^{\mathcal K_{\lambda U,2}}
    (\mathrm{Pub}(H),\mathrm{LiveSpec}(H);S)
  \simeq
  \mathrm{ExactAPI}(H,x),
\]

and replay proves:

1. **Exact reconstruction:** completion reconstructs every and only admitted
   public declaration, descriptor, and equation.
2. **Complete disposition:** every raw clause receives one exhaustive
   lambda/unit disposition.
3. **Non-reconstruction:** each basis member remains unavailable from the
   predecessor and the complete candidate basis with that member removed.
4. **Independence:** removing any basis member prevents exact
   reconstruction.
5. **Uniqueness:** every admissible basis agrees up to complete Q2
   presentation equivalence.
6. **Order invariance:** equivalent independent declaration orders produce
   the same quotient basis.

Then, and only then,

\[
  \mathsf K^{\mathcal K_{\lambda U,2}}_H(x)=S,
  \qquad
  \kappa^{\mathcal K_{\lambda U,2}}_H(x)=|S|.
\]

Multiple inequivalent bases, incomplete inventory or disposition, missing
negative evidence, or failed uniqueness returns `Unknown` and leaves the
cost undefined. No traversal, source order, serialization order, hash, or
greedy strongly connected component choice may select a basis.

## Rewrite admissibility remains separate

Cost-free completion and rewrite admissibility answer different questions:

\[
\begin{array}{ll}
\text{Rewrite admissibility:}
  & \text{May a sealed equation safely compute?}\\[0.3em]
\text{Kernel cost:}
  & \text{Was that public equation already derivable for free?}
\end{array}
\]

The cost verifier may consume Q0 normal forms only from the finite,
inventory-bound rewrite theorem required by the companion lambda/unit audit.
That theorem must exhaustively establish typing, substitution stability,
termination, confluence, overlaps, and predecessor conservativity over the
manifest-generated finite universe.

Rewrite authority does not establish a zero-cost completion. A typed,
admissible equation for a bodyless fresh head may still be
`FirstIrreducible`. Conversely, missing rewrite authority leaves Q0 and the
complete cost audit undefined even when a clause's proposed cost disposition
would otherwise be clear.

## Generic pre-adoption vectors

Before any live input could be considered, independent Rust and safe-Agda
computations must agree on at least:

1. first public export of `UnitType`;
2. later re-export of `UnitType`;
3. a primitive occurring only in a declaration type or proper subterm;
4. a transparent alias to an earlier public export;
5. two simultaneous Q2-equivalent first exports under two independent
   declaration orders;
6. a bodyful definition whose beta follows by ordinary reduction;
7. a separately represented equation exactly reconstructed by that ordinary
   beta path;
8. a bodyless fresh head plus a separately sealed fresh equation, neither
   predecessor-public nor duplicate;
9. an exact bodyless-head equation replay that is predecessor-public;
10. a duplicate transparent declaration or equation presentation;
11. two mutually dependent irreducible clauses;
12. a deliberately nonunique irreducible basis; and
13. every projection-bearing adversarial input listed by the projection
    boundary.

Vector 8 classifies both the bodyless head and its separately sealed equation
`FirstIrreducible` after complete negative evidence. Vector 9 classifies the
equation replay `TransparentAlias`. Vector 12 returns
`Unknown(NonUniqueBasis)`. Every vector in item 13 returns
`OutsideFragment(DescriptorProjection)`.

The transcript comparison must bind canonical input and independently
computed inventory, ambient-census, ordinary-beta, Q0, Q2, clause
disposition, reconstruction, negative-evidence, basis, and order-invariance
outputs. Typechecking a fixed reference module or embedding a Rust-produced
answer is not transcript agreement.

## Adoption and freeze boundary

This document neither adopts nor freezes
`gf2-kernel-cost-lambda-unit-v2`. A later adoption packet must expose the
exact manifest, candidate digests, theorem surface, independently computed
transcripts, supported surface, outside-fragment surface, and falsifiers, and
must confirm that no Profile A data was accessed. Human review and explicit
adoption must precede any manifest freeze.

Only after the semantic and cost theorems, transcript agreement, explicit
adoption, and freeze may any separately isolated live-history adapter be
proposed. This document creates no adapter, imports no live history, and
issues no value.

## Later record extension

Records and projections belong in a separately versioned proposal, for
example:

```text
gf2-semantic-audit-record-extension-v1
gf2-kernel-cost-record-extension-v1
```

That later cost profile must define the exact record/descriptor grammar,
exhaustive projection inventory, typed projection reconstruction and
reduction theorems, critical-pair obligations, cost dispositions, generic and
adversarial transcript vectors, adoption packet, and freeze boundary. It may
extend or transport lambda/unit results only through an explicit reviewed
theorem; no authority is inherited by a name or version relationship.

## Preserved results

This successor proposal changes no existing proposal or issued artifact. It
does not calculate a live act, assume any \(\kappa_i\) or \(\nu_i\), authorize
a productivity benchmark, or predict whether a future frozen audit will be
defined.
