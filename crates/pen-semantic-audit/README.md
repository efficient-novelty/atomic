# pen-semantic-audit

This is the isolated generic implementation lane for the proposed
`gf2-semantic-audit-core-v1` and `gf2-kernel-cost-core-v1` calculi.

It is deliberately a nested Cargo workspace. Adding it to the repository root
workspace would change the root `Cargo.toml` and `Cargo.lock`, both of which
are bound into the issued H3/H4 evidence. Build it with:

```text
cargo test --manifest-path crates/pen-semantic-audit/Cargo.toml --locked
```

The crate has no live Profile A adapter and cannot issue Acts 1--4 values.
The semantic and cost profiles remain `proposed_not_adopted` until their
generic Rust/safe-Agda theorem and reference-vector gates are complete and
independently reviewed.

## Prototype closeout

The current crate is a generic, isolated prototype only. Its semantic-audit
and kernel-cost manifests are unfrozen, have no live Profile A authority, and
must not be used to calculate or issue any Act value.

Fresh constructor computation is fail-closed. The prototype's structural
reconstruction tags are not the exact free-completion theorem required to
classify a fresh public equation as `ForcedDefinitionalCompletion`, so the
generic cost vector returns `Unknown(MissingFreeCompletionTheorem)`. That
result must remain in force until the theorem is implemented, bound to
verified inputs, and independently reviewed.

The principal open gates are:

- a complete verified predecessor/public inventory and boundary binding;
- a pre-existing typed computation-demand contract that cannot encode the
  candidate equation post hoc;
- the exact fresh-equation free-completion theorem;
- structural typed family weakening and its restriction retraction;
- a digest-bound verified-empty origin-cutoff Q3 registry; and
- executable Rust/safe-Agda agreement for the complete generic vector suite.

The issued Profile A H3 and H4 results and
`LAW_V2_WINDOW_REGISTER_AUDIT_V1` are unchanged. In particular, this
prototype does not replace that audit's `UndefinedAudit` outcome.

See the
[prototype result](../../docs/LAW_V2_SEMANTIC_AUDIT_CORE_V1_PROTOTYPE_RESULT.md)
for the validation snapshot and exact stop condition.
