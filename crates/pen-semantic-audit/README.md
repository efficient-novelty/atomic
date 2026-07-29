# pen-semantic-audit

This is the isolated generic implementation lane for the broader
`gf2-semantic-audit-core-v1` / `gf2-kernel-cost-core-v2` proposals and their
projection-free successors:

```text
gf2-semantic-audit-lambda-unit-v1
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

The lambda/unit successor now has verifier-minted ambient first-export and
ordinary-beta capabilities and a syntactic pre-Q0 rank-0/1/2 carrier.
Rewrite admissibility remains independent. The public finite-rewrite entry
point validates its bindings, exact seed term subjects, and fragment boundary,
then deliberately returns
`Unknown(MissingRewriteAdmissibilityTheorem)`. No diagnostic graph can enter
authoritative Q0 normalization. The public lambda/unit cost entry point
requires the matching rewrite-system capability, so it cannot mint a cost
certificate while this gate remains open.

The inventory capability proves exact replay and internal census closure only
relative to its supplied generic event ledger. It deliberately does not prove
that the ledger is the issued Profile A history. That later authority binding
belongs in the separately isolated Profile A adapter and may occur only after
the generic profiles are frozen.

The principal open gates are:

- a genuinely finite or canonically quotiented substitution carrier: the
  current embedding/forced-newest generators have unbounded syntactic closure
  under composition even on a small generic vector;
- recursively typed subterm and binder-local rule-instance closure;
- an independently reconstructed predecessor universe and historical rewrite
  authority, plus an exhaustive same/nested/disjoint edge-pair report;
- complete semantic seed authority for public-clause and demand-anchor
  identities, rather than caller-supplied support metadata;
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
for their respective authority boundaries and stop conditions.
