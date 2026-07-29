# pen-semantic-audit

This is the isolated generic implementation lane for the broader
`gf2-semantic-audit-core-v1` / `gf2-kernel-cost-core-v2` proposals and their
projection-free successors:

```text
gf2-semantic-audit-lambda-unit-v1
gf2-semantic-audit-lambda-unit-v2
gf2-semantic-audit-lambda-unit-v3
gf2-kernel-cost-lambda-unit-v2
```

The broader proposals and historical V1 cost prototype remain present for
regression and comparison.

It is deliberately a nested Cargo workspace. Adding it to the repository root
workspace would change the root `Cargo.toml` and `Cargo.lock`, both of which
are bound into the issued H3/H4 evidence. Build it with:

```text
cargo test --manifest-path crates/pen-semantic-audit/Cargo.toml --locked
```

The crate has no live Profile A adapter and cannot issue Acts 1--4 values.
The semantic and V2 cost profiles remain `proposed_not_adopted` until their
generic theorem, transcript-level Rust/safe-Agda, review, and freeze gates are
complete.

## Prototype closeout

The current crate is a generic, isolated prototype only. Its semantic-audit
and kernel-cost manifests are unfrozen, have no live Profile A authority, and
must not be used to calculate or issue any Act value.

Cost V2 no longer treats a separately sealed computation equation for a
bodyless fresh head as free merely because a demand or compiler determines its
shape. Relative to a verifier-minted generic public inventory, the head and
equation are separate paid clauses unless exact predecessor-public replay or a
complete duplicate proof applies.

The V1 lambda/unit lane retains its verifier-minted ambient first-export and
ordinary-beta capabilities and syntactic pre-Q0 rank-0/1/2 carrier. Its public
finite-rewrite entry point still returns
`Unknown(MissingRewriteAdmissibilityTheorem)`.

The V2 semantic successor removes the false requirement that a finite set of
substitution generators be closed under arbitrary composition. It enumerates
only derivation-local construction witnesses and assigns arbitrary
substitution algebra, typing, reduction stability, and family naturality to a
generic theorem package. It also adds verifier-minted public-clause authority,
an empty demand-anchor base, a V2 seed census for compatible empty-demand
inventories, binder-local typed occurrences, and an empty historical rewrite
base. None of these partial capabilities enters authoritative Q0
normalization.

The additive V3 semantic successor corrects the authority ordering without
changing V1 or V2. Rank-zero seed and family identity is demand-neutral:
verifier-derived event/declaration structural support participates in identity,
while exact equation `PortKey` associations remain separately bound metadata
with no orbit, realization, marginal, SR2, or novelty authority. Nonempty
demand provenance is therefore deferred until after the family quotient,
weakening, and marginal identification.

V3 also adds the isolated `pen-kernel-synthesis` workspace. Its
syntax-directed sort/unit/variable/global/`Pi`/lambda/application derivations
are replayed through the unchanged kernel before an opaque synthesis
capability is returned. An additive fixed safe-Agda package proves synthesis
soundness/completeness for its conversion-free abstract calculus, dependent
simultaneous substitution, binder lifting, and typed beta/closed-delta/fresh
schema stability. `VerifiedLambdaUnitTypingFoundationV1` can be minted from
that package.

The stronger `VerifiedLambdaUnitTypingMetatheoryV1` remains unminted until four
exact correspondences are proved:

- abstract variables and contexts to oldest-first finite de Bruijn contexts;
- the declarative calculus to kernel conversion and normalization;
- Agda terms/derivations to the exact Rust synthesis codes; and
- abstract typed reductions/families to the exact Q0 and family inventories.

Synthesis-backed binder-local traversal is positive for fixed supplied roots,
including a variable-headed application. Each occurrence retains the full
opaque synthesis capability and binds it to an independent unchanged-kernel
replay. That does not prove a complete occurrence census: only the future
production metatheory and native carrier can supply and close the
authoritative finite root domain. Consequently V3 does not yet mint its native
rank-0/1/2 carrier, complete synthesis-backed occurrence census, base rewrite
system, Q0/family quotient, weakening/marginals, demand realizations, SR2,
transcript agreement, adoption, freeze, or Profile A adapter.

The inventory capability proves exact replay and internal census closure only
relative to its supplied generic event ledger. It deliberately does not prove
that the ledger is the issued Profile A history. That later authority binding
belongs in the separately isolated Profile A adapter and may occur only after
the generic profiles are frozen.

The principal V2 open gates are:

- the dependent typing substitution lemma and typed Q0 stability (the pinned
  Agda package currently proves raw substitution algebra and scope, not
  dependent typing);
- demand-orbit equivalence and kernel-replayed typed demand realizations for a
  nonempty predecessor demand inventory;
- a native V2 rank-inductive carrier reconstructed from V2 seed identities,
  rather than a relative direct-witness census over the V1 carrier;
- kernel-supported synthesis of binder-local variable/function types needed
  for a complete typed-occurrence census;
- an issued historical rewrite theorem for every nonempty predecessor;
- edge-local rule matching, complete reduction graphs, exhaustive
  same/nested/disjoint overlap joins, and weakening-image conservativity;
- structural typed family weakening and its restriction retraction;
- complete carrier, quotient, cost-basis, marginal, and SR2 theorem
  capabilities composed from the verified inventory;
- transcript-level Rust/safe-Agda agreement for the complete generic vector
  suite; and
- a later adapter that binds the generic inventory wire to the independently
  replayed issued Profile A history and authoritative Q3 registry.

The issued Profile A H3 and H4 results and
`LAW_V2_WINDOW_REGISTER_AUDIT_V1` are unchanged. In particular, this
prototype does not replace that audit's `UndefinedAudit` outcome.

See the historical
[V1 prototype result](../../docs/LAW_V2_SEMANTIC_AUDIT_CORE_V1_PROTOTYPE_RESULT.md)
and the successor
[V2 inventory prototype result](../../docs/LAW_V2_KERNEL_COST_V2_PUBLIC_INVENTORY_PROTOTYPE_RESULT.md)
and
[lambda/unit finite-theorem prototype result](../../docs/LAW_V2_LAMBDA_UNIT_FINITE_THEOREM_PROTOTYPE_RESULT.md)
and the
[lambda/unit V2 adjudication](../../docs/LAW_V2_SEMANTIC_AUDIT_LAMBDA_UNIT_ADJUDICATION_V2.md)
and
[lambda/unit V2 prototype result](../../docs/LAW_V2_SEMANTIC_AUDIT_LAMBDA_UNIT_V2_PROTOTYPE_RESULT.md)
and the
[lambda/unit V3 adjudication](../../docs/LAW_V2_SEMANTIC_AUDIT_LAMBDA_UNIT_ADJUDICATION_V3.md)
and
[lambda/unit V3 prototype result](../../docs/LAW_V2_SEMANTIC_AUDIT_LAMBDA_UNIT_V3_PROTOTYPE_RESULT.md)
for their respective authority boundaries and stop conditions.
