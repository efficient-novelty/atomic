# HIST-CERT v1 result: historical arithmetic replays, semantic certification remains open

> **Successor result (HIST-CERT v3, 2026-07-19):** the adopted
> element-overlay boundary replay now discharges F-B2. Raw values remain
> `7/8/10/18`; the registered typed-and-marginal path is classified as
> `2/2/5/10`, while the ordinary `5/6/5/8` remainder and independent novelty
> credit remain unresolved. Every full `certified_total` is `null`, and F-T1
> remains open. See `docs/HIST_CERT_V3_RESULT.md`. The v1 body below is
> retained as an archival record.

**Execution date:** 2026-07-19  
**Artifact:** `docs/hist_cert_v1.json`  
**Schema:** `hist-cert-historical-hit-v1`  
**Result digest:**
`blake3:e840f99c519756bc6215fbf8db03a738dae7c9127043599c06eaafed25d77c27`

## Outcome

HIST-CERT v1 was executed fail-closed. The rule-driven operational
enumerator independently presents `7, 8, 10, 18` families for the four
historical HIT packages. It does **not** certify those numbers as semantic
novelty totals. `F-T1` therefore remains active.

This is an obstruction exit, not a contrary historical score. No certified
total different from the recorded total was produced. The machine instead
returns `certified_total = null` and names the missing evidence on every
affected family.

| historical package | operational families | path families typed and marginal in the registered fragment | undefined credit verdicts | certified total |
| --- | ---: | ---: | ---: | ---: |
| Step 5 / S1 | 7 | 2 | 7 | undefined |
| Step 6 / Trunc | 8 | 2 | 8 | undefined |
| Step 7 / S2 | 10 | 5 | 10 | undefined |
| Step 8 / S3 | 18 | 10 | 18 | undefined |

All four rows carry
`conditional_on_boundary_axiom_v1 = true`. The attachment rule remains
conditional and unadjudicated; HIST-CERT does not convert it into a theorem.

## What the machine now proves

### Exact predecessor reconstruction

Each package is audited against its own predecessor, not a later library.
The reconstructed signature digests are:

| context | digest |
| --- | --- |
| B4 | `blake3:11c74244bfb002c53bc4f3b9a3c7013e7e38460ad03ce63bf9bc41e535fc8458` |
| B5 | `blake3:0ee6911b2820caeb0e745b4e44fa0875b8bfbca8f66b6a4921694ebde1cf44fd` |
| B6 | `blake3:e8cba04def50e09c83b1e28a252bb6f2e327e6468027a3d9353e20d9ad19e5ed` |
| B7 | `blake3:301916fbd578c9e0d6375724b44201f0198d2d4bbd9493614009a603325a3adb` |

Every predecessor path token is reissued in the audited signature and
replayed before it enters the weakening inventory. It is also issued in its
original signature. The support-canonical projections agree after weakening,
and the artifact records both inverse checks: erasure after weakening returns
the original family, and weakening after erasure returns the same projected
family.

### Type-sensitive path-family tokens

`pen-type::cubical` now mints a historical projection that binds every
replayed path realization to its inferred type as well as its term and normal
form. A support-canonical projection treats the fresh declared owner as part
of the family normal form. Consequently, the three shallowly identical sphere
formation expressions are not collapsed merely because each prints as
`App(Univ, Var(1))`.

Equality is three-valued and restricted to the frozen historical path
fragment:

- equal families carry a replayed frozen-equality witness;
- dimensions, ordered path keys, inferred types, and equality-free fresh
  declaration supports are rigid separators;
- any identity syntax requiring a genuine univalent transport audit returns
  `undefined`.

The complete registered predecessor sweep decides every presented beta/Kan
family as marginal: `2, 2, 5, 10`. Token hashes are not used as semantic
equality; the support-canonical family projection is.

### Rule-driven operational inventory

The implementation does not fill a numerical gap from the recorded totals.
It walks each actual telescope and applies the frozen operational grammar:

- one formation family;
- one family for each pre-path point/unit clause;
- path introduction, recursor, and inductor;
- one parametric action when the formation is `Trunc(Var(_))`;
- beta plus the ordered dimension-by-dimension Kan basis;
- one forward family per post-path clause and one cell action per canonical
  operation position.

That grammar produces the recorded finite presentations exactly. The
recorded number is attached only afterward as a regression comparison.

## Why the totals are not certified

### Ordinary natural-family tokens are missing

Formation and point/unit clauses have shallow kernel elaborations. Path
introduction, recursor, and inductor have typed cubical representatives.
Neither form of evidence is an opaque token proving a complete natural family,
its naturality, and its full predecessor weakening/univalent-equality sweep.
The Trunc action and the post-path operation families lack even an ordinary
typed term in the current cubical fragment.

An individual typed eliminator application is not silently promoted to the
recursor or inductor natural family. A shallow lambda is not silently promoted
to multiplication, a unit law, or an action on a cell.

### Path-basis exhaustiveness remains unproved

The registered beta/Kan index set has exact, replayable representatives. The
machine still lacks the classifier theorem saying that every intended
depth-two path schema normalizes to exactly one member of that set. Exactness
of the registered grammar is therefore not exactness of the intended semantic
schema space.

### EGP provenance is not token-bound

Distinct path keys and distinct realization tokens are not distinct EGP
anchors. Several beta/Kan families originate at the same path clause and the
same coarse coherence role. Assigning that one slot repeatedly would trigger
the anchor-collision falsifier. HIST-CERT therefore assigns no fabricated
anchor and leaves each positive path-family credit verdict undefined.

A future anchor must bind the exact normalized family and its marginality
proof to either:

- a separately chargeable local semantic role whose validity is replayed; or
- a particular individual pre-existing demand output with a checked type
  match.

Focus-family labels are not accepted as such evidence.

### The Trunc boundary is a concrete mismatch

The frozen attachment rule provides an implicit base and a constant-boundary
loop. The intended Trunc squash has endpoint-dependent boundary data. The
current Trunc beta/Kan replay is therefore a conditional generic-loop
surrogate, not a typing of the squash constructor. This is recorded on the
affected per-family rows and cannot be repaired inside HIST-CERT because the
attachment theory is explicitly out of scope.

### The S3 post-path payload is only positional

The two post-path lambdas produce three slots under the canonical
operation/coherence pairing convention: operation forward face, coherence
rewrite, and operation-on-cell action. Their intended signatures are not
present in the shallow telescope or the cubical type grammar. The artifact
records all three as separate named obstructions rather than assuming their
legacy interpretation.

## Replay and mutation falsifiers

The create-new artifact replays by rebuilding every predecessor context,
token, equality comparison, inventory, verdict, and digest. The following
registered mutations all fail replay:

- verdict flip;
- fabricated local-anchor claim;
- equality-proof hash mutation;
- weakening/erasure inverse-law mutation;
- conditionality flip;
- typed-token hash mutation;
- family deletion;
- mutation followed by recomputation of an outer digest.

## Verification

```text
cargo test -p pen-type cubical --lib
cargo test -p pen-eval tdc1_hist_cert --lib
cargo run -p pen-eval --example hist_cert -- replay docs/hist_cert_v1.json
```

## Required next work

1. Resolve the attachment rule for each historical constructor, especially
   the endpoint-dependent Trunc squash.
2. Mint opaque ordinary-family tokens for formation, point/unit, recursor,
   inductor, Trunc action, and post-path operation families.
3. Prove the intended depth-two path classifier/exhaustiveness theorem.
4. Bind every marginal family to a distinct proof-carrying local role or
   individual demand output.
5. Re-run HIST-CERT create-new. Until all four rows have numeric certified
   totals, the operational counts remain a successful presentation replay,
   not a semantic re-certification.
