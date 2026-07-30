# pen-production-wire

`pen-production-wire` is the isolated, authority-free byte grammar for the
Law V2 lambda/unit production-refinement bridge. It has no dependency on the
kernel, synthesis, semantic-audit, oracle, or runtime crates.

The authoritative serialization is implemented manually. Serde and JSON are
not part of this crate's protocol.

## Canonical envelope

Every V1 bundle has:

1. the fixed 16-byte ASCII magic `PEN-PROD-WIRE-V1`;
2. little-endian `u16` schema version `1`;
3. little-endian `u16` section count `11`;
4. eleven frames in exact tag order.

Each frame is a one-byte tag, a little-endian `u64` payload byte length, and
the payload. The required tags are:

| Tag | Section |
| ---: | --- |
| 1 | V3 correspondence-manifest surface |
| 2 | production signature binding |
| 3 | global-slot table |
| 4 | production contexts |
| 5 | conversion certificates |
| 6 | nested conversion-typing supplements |
| 7 | synthesis certificates |
| 8 | exact V3 Q0 inventory |
| 9 | fresh-rule schemas |
| 10 | exact family inventory |
| 11 | family payloads |

Sequences and byte strings carry a little-endian `u64` count. Options use
only tags `0` and `1`. Integers are little-endian. Every enum uses a one-byte
tag. A decoder accepts only full consumption, exact section order, structural
well-scoping, and byte-identical canonical re-encoding.

## Trust boundary

Successful decoding proves only that bytes belong to this closed canonical
grammar and satisfy its fail-closed structural invariants. It does not prove
typing, conversion, semantic correspondence, transcript agreement, or
production authority. Those remain responsibilities of independent Rust
replay, safe Agda checking, and the private semantic minting factory.
