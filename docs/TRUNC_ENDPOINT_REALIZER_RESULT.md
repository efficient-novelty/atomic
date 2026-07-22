# TRUNC-ER result: registered endpoint realizer closed at d = 1

> **HIST-CERT v3 successor (2026-07-19):** the two-key B5 handoff here was
> consumed with the exact B4/B6/B7 boundary prefix. F-B2 is discharged, and
> the registered typed-and-marginal path is classified as `2/2/5/10`. The
> ordinary `5/6/5/8` remainder and independent novelty credit remain
> unresolved; every full `certified_total` is `null`, and F-T1 remains open.
> See `docs/HIST_CERT_V3_RESULT.md`. This TRUNC-ER result remains archival.

**Date:** 2026-07-19. **Execution status:** success at the registered Trunc
scope. The obstruction
`C6_TRUNC_DECLARED_ENDPOINT_PATHCON_REALIZER` is retired by new replayed
tokens in full H15 and exact historical B5. General C6 remains open.

## Create-new artifact

The certificate is `docs/trunc_endpoint_realizer_v1.json`:

- schema: `trunc-endpoint-realizer-v1`;
- length: 21,372 bytes;
- SHA-256:
  `22C14233391B251379F3E99D6A3F199ECC3AD5F214291CF6CEA4451164D47967`;
- Git blob: `ea41ae48902f19ee4f318f1b5f4321b17d35daf3`;
- internal digest:
  `blake3:62b9e72a0047dd578f6881dfbeff9ff3b87cd19118e3f84ea917fdef604a84a1`.

Its replay is strict: duplicate recognized fields, unknown top-level fields,
and unknown or duplicate nested fields fail. The JSON contains projections,
not deserialized kernel tokens; replay reconstructs every opaque token from
the sealed sources and compares the exact certificate.

## Computation rule implemented

The constant-boundary `PathConstructor` and its token domain were not
changed. A sibling d = 1 constructor now carries two typed ordinary-context
points and computes by face:

```text
squash(x,y,0) = x
squash(x,y,1) = y
```

Its eliminator method has the PathP shape

```text
PathP (i -> P(squash(x,y,i))) (method x) (method y).
```

This is checked as a sequent under an explicit formal schema context, not as
a closed inhabitant. The opaque, source-bound premise ledger declares the
motive `P`, endpoint evaluations `e0 : P(x)` and `e1 : P(y)`, and the
supplied PathP method `q` with recorded equations `q(0)=e0` and `q(1)=e1`.
Ordinary context-free inference rejects every endpoint premise reference,
boundary parameter, and endpoint constructor. Only the endpoint-aware
checker can use them, and only against the exact replayed ledger.

The machine checks all of the following before evidence is issued:

- method restriction at 0 and 1 gives the supplied endpoint evaluations;
- eliminators on `x` and `y` compute by endpoint instantiation;
- an eliminator on the constructor computes to the supplied `q`;
- an eliminator on any other typed neutral remains stuck at `P(z)` while
  retaining the exact `q` provenance;
- dependent `coe` along the exact `squash(x,y,i)` family consumes `e0` at
  `P(x)` and has target `P(y)`;
- forged endpoint paths and missing, mutated, or cross-source premise
  contexts fail.

The two endpoint terms are not accepted from display labels. The issuer
derives them from the replayed Trunc typed-boundary token and requires the
exact context lookups at indices 1 and 2 in
`A : Type; x,y : Trunc(A)`.

## Exact two-key basis

Both source scopes replay the same exact correspondence:

| declared-boundary key | typed path key |
| --- | --- |
| `Beta` | `Beta` |
| `PrincipalTransport(0)` | `Kan(0,0)` |

No endpoint image is exported as an additional key. Thus the registered
count is exactly `1 + d² = 2`, with zero reference-only boundary charge.

| scope | signature digest | basis result |
| --- | --- | ---: |
| full H15 | `blake3:d51ffb32c2e6e18f45ee15a8c1af3edbf40759fda1696032ae6e97439c48f10a` | 2 / 2 |
| exact B5 | `blake3:0ee6911b2820caeb0e745b4e44fa0875b8bfbca8f66b6a4921694ebde1cf44fd` | 2 / 2 |

Wrong predecessors and altered Step-6 telescopes fail before token issue.

## Restricted parameter instantiation

The bundle replays C-1 only in its proved scope:

```text
sort-identical variable images; arbitrary typed instance images remain an explicit gap
```

For the fixed source and target context, the owner has one same-sort target
and each endpoint has two. The checker exhausts all four possible endpoint
maps using one canonical C-1 image inventory per instance:

| restricted instance | endpoint image |
| --- | --- |
| identity | `(x,y) -> (x,y)` |
| swap | `(x,y) -> (y,x)` |
| collapse to x | `(x,y) -> (x,x)` |
| collapse to y | `(x,y) -> (y,y)` |

Each instance reconstructs and checks its endpoint constructor, formal
premise ledger, PathP method, eliminator computations, and `coe`. The two
contractions use one coherent endpoint-evaluation reference. No arbitrary
typed image is used. The inherited gap remains named in the certificate:

```text
C1_ARBITRARY_TYPED_INSTANCE_SORT_PRESERVATION
```

## Agent A handoff

The successor API
`issue_historical_prefix_v3_c6_typed_bundle_token` accepts the caller's
reconstructed predecessor and current telescope. It wraps the archival
constant bridges and the new Trunc bridge uniformly:

| package | prefix | evidence path | realized keys |
| --- | --- | --- | ---: |
| S1 / Step 5 | B4 | archival constant bridge | 2 |
| Trunc / Step 6 | B5 | endpoint-dependent successor | 2 |
| S2 / Step 7 | B6 | archival constant bridge | 5 |
| S3 / Step 8 | B7 | archival constant bridge | 10 |

The resulting handoff vector is exactly `2/2/5/10`. The old
`issue_historical_prefix_v3_c6_bridge_token` remains unchanged and still
reports the old Trunc gap. This is intentional: schema 5 is an archival
snapshot, while the typed-bundle API is its explicit successor.

## Conservativity gate

The new certificate compile-time binds the exact bytes of all six archival
artifacts by pinned byte length and SHA-256, and records the internal-digest
field contained in those pinned bytes. `replay_trunc_endpoint_realizer_json`
does not execute the archives' domain-specific definition replayers. As a
separate external F-TR2 gate for this execution, the six public replay
commands below were run; all succeeded and printed the following results:

| archive | externally observed definition-replay result (this execution) |
| --- | --- |
| HIST-CERT v1 | `blake3:e840f99c519756bc6215fbf8db03a738dae7c9127043599c06eaafed25d77c27` |
| BOUNDARY-AUDIT v1 | `blake3:76c389766a5c54582d1e2ffe9942db5c53cd3764137cfc62254b9857e1f1c435` |
| TDC cubical regression v3 | `blake3:075791e9024a8c36a3f0dab593fa1460fb8288d3e9fc3345aa76c48cff978e56` |
| IP-1 schema 3 | `blake3:e4cb89fe83476fb8a0eee43bcc976053bcbbff0bd6db35607d7e8154a3ef858b` |
| IP-1 schema 4 | `blake3:6a6c3ccfe4731080bfaa944b98f50afe2bde1eb29fdd679b8f62e3a42f72d1e2` |
| IP-1 schema 5 | `blake3:ee5a0883ef0a81450035d472c855a81d259d6058498983eddc05707b8c3b637b` |

Accordingly F-TR2 passed for this execution through two distinct checks:
certificate-bound byte identity and separately executed public definition
replay. A future replay of only `trunc_endpoint_realizer_v1.json`
re-establishes the former; it must rerun the listed commands to re-establish
the latter.

The S1/S2/S3 child bridge hashes also remain exactly the schema-5 values.
No old enum variant gained a field, and no incumbent cubical version or hash
domain was changed.

## Reproduction

```powershell
cargo test -p pen-type --lib
cargo test -p pen-eval --lib

cargo run -p pen-eval --example trunc_endpoint_realizer -- replay `
  docs\trunc_endpoint_realizer_v1.json

cargo run -p pen-eval --example hist_cert -- replay docs\hist_cert_v1.json
cargo run -p pen-eval --features boundary-audit --example boundary_audit -- replay docs\boundary_audit_v1.json
cargo run -p pen-search --example tdc1_cubical_regression -- replay docs\tdc1_cubical_regression_v3.json
cargo run -p pen-search --example ip1_candidate_join -- replay docs\ip1_candidate_verdict_join_v3.json
cargo run -p pen-search --example ip1_candidate_join_v4 -- replay docs\ip1_candidate_verdict_join_v3.json docs\ip1_candidate_verdict_join_v4.json
cargo run -p pen-search --example ip1_candidate_join_v5 -- replay docs\ip1_candidate_verdict_join_v3.json docs\ip1_candidate_verdict_join_v4.json docs\ip1_candidate_verdict_join_v5_element_overlay.json
```

## Honest conclusion

TRUNC-ER closes the registered d = 1 computation layer and completes the
four-package `2/2/5/10` handoff required by Agent A. The judgement is under
the explicit formal eliminator-method premises above; it does not claim a
closed term without those inputs. It also does not prove arbitrary-dimension
face-indexed motives, basis independence or intended exhaustiveness,
candidate-level C8, intended-schema classification, or any Step-16/global-
halt result. The next prescribed action is Agent A's create-new HIST-CERT v3
rerun under sharpened F-B2.
