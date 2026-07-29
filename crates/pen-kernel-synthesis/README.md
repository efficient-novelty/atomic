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

## Protocol V2

`pen-kernel-synthesis/lambda-unit/v2` adds an authority-free, serializable
eight-rule synthesis code and an independent proof-producing verifier. Its
base-Q0 conversion evidence contains explicit beta, transparent-delta, and
Pi/Lambda/Apply congruence traces, two reductions to one common normal form,
a complete no-redex census, and unchanged-kernel replay of every outer trace
intermediate.

Transparent delta is relative to an ordered `(signature slot, GlobalId)`
policy wire. The crate verifies that the table names exact bodyful
declarations but does **not** claim that they are predecessor-public. That
historical binding belongs to a later semantic-audit capability.

Public certificate syntax admits `Sort 0` and `Sort 1`. Exact checker-produced
types may additionally contain `Sort 2`; caller-supplied `Sort 2` code is
rejected.

Conversion endpoints use an explicit replay judgment. Term-level conversions
replay `HasType` with an exact expected type. Synthesis conversions between
types replay `TypeFormation`, so a certificate containing `Sort 2` does not
serialize the higher universe that the unchanged kernel uses internally to
establish formation. Certificate syntax remains capped at `{0,1,2}`. A later
safe-Agda bridge must still prove the existential kernel-internal
formation-universe correspondence; V2 does not mint that theorem.

Every recorded unchanged-kernel replay output is checked before it enters an
opaque capability and checked again during aggregate replay. Its normalized
context must remain public lambda/unit syntax capped at `{0,1}`, and every
term/type payload must remain lambda/unit checker-output syntax capped at
`{0,1,2}`. Unrestricted signature bodies therefore cannot smuggle higher
universes or out-of-fragment syntax through global normalization.

V2 replays every outer conversion-trace intermediate. Nested congruence
premises are reconstructed exactly, but exact binder-local kernel replay for
those nested premises remains an explicit frontier exposed by
`NESTED_CONGRUENCE_REPLAY_FRONTIER_V2`; V2 does not claim that stronger
conversion-typing correspondence.
