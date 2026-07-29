# pen-kernel-synthesis

`pen-kernel-synthesis` is an isolated, proof-carrying synthesis successor for
the exact `pen-kernel` lambda/unit fragment:

```text
Sort, UnitType, Unit, Var, Global, Pi, Lambda, Apply
```

It depends on the unchanged `pen-kernel`. It does not expose or duplicate a
trusted kernel inference API. Instead, its syntax-directed mirror constructs a
candidate type, replays type formation and `HasType` with the unchanged
kernel, and batch-replays the complete derivation before returning an opaque,
non-deserializable capability.

This crate is deliberately a nested Cargo workspace. Adding it to the
repository root workspace would change protected root evidence inputs. Build
it with:

```text
cargo test --manifest-path crates/pen-kernel-synthesis/Cargo.toml --locked
cargo clippy --manifest-path crates/pen-kernel-synthesis/Cargo.toml --locked --all-targets -- -D warnings
```

The crate has no Profile A adapter and grants no live Law V2 authority.
