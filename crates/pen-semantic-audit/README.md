# pen-semantic-audit

This is the isolated generic implementation lane for the proposed
`gf2-semantic-audit-core-v1` calculus and the successor
`gf2-kernel-cost-core-v2` cost proposal. The historical V1 cost prototype
remains present for regression and comparison.

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
complete duplicate proof applies. Rewrite admissibility remains independent:
the equation cannot enter authoritative Q0 normalization until a real,
input-bound rewrite-system theorem proves substitution stability, termination,
confluence, nonoverlap, and conservativity.

The inventory capability proves exact replay and internal census closure only
relative to its supplied generic event ledger. It deliberately does not prove
that the ledger is the issued Profile A history. That later authority binding
belongs in the separately isolated Profile A adapter and may occur only after
the generic profiles are frozen.

The principal open gates are:

- verifier-minted ordinary-beta provenance, ambient first-export coverage, and
  descriptor/projection reduction authority for the complete V2 cost grammar;
- a theorem-producing `VerifiedRewriteSystemV1`, bound one-to-one to the
  verified sealed-equation and projection inventory;
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
for their respective authority boundaries and stop conditions.
