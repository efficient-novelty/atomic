# TDC-1 result: regression gate failure, no numerical conclusion

**Date:** 2026-07-18. **Controlling registration:**
`docs/ip1_tdc1_plan.md`. **Phase-1 freeze:** commit `fc3b026`.

## Result

The historical regression gate failed. Therefore **F-T1 is true** and the
TDC-1 machinery does not issue a valid Step-16 semantic verdict. The frozen
comparison records **Z4** only as the pre-registered diagnostic classification
of this insufficient fragment: novelty is undefined, so `nu`, `rho`, and the
bar comparison are all `null`. It is not a numerical result for Step 16.

F-T2 and F-T3 are false. The fragment found neither an anchor collision nor a
basis count above the frozen `1 + d^2` ceiling. This run consequently neither
refutes the `d = 4` content hypothesis nor proves the global halt at fifteen.
Verdict validity is future-safe: a defined score alone cannot pass without an
exact partition, complete typed realization and weakening, complete injective
anchors, historical regression, and every registered mutation rejection.

| registered field | frozen value |
| --- | --- |
| zone | `Z4` (diagnostic; verdict void under F-T1) |
| certified `nu` / `rho` | undefined / undefined |
| `Bar16` / clearing threshold | `354333/39040` / `19` |
| clears bar | undefined |
| historical regression | failed |
| Step-16 verdict valid | false |
| F-T1 / F-T2 / F-T3 | true / false / false |

## What was certified

The registered surface `[App(Univ, Lib(15)), PathCon(4)]`, with `kappa = 2`,
elaborates against the sealed `B15` signature. Two independent enumerators
agree on the exact symbolic L1 partition:

| beta | diagonal Kan | ordered off-diagonal Kan | total |
| ---: | ---: | ---: | ---: |
| 1 | 4 | 12 | 17 |

The one formation family makes `18` a **conditional raw index-level ceiling**.
It is not a certified novelty score. None of the 17 path sites has a typed
cubical realization, semantic weakening is incomplete, and no marginal site
has an EGP anchor. The audit also records local anchor capacity `8` and zero
live Stage-16 demand orbits; neither fact upgrades an unrealized index to
semantic credit.

Historical HIT regression used the same machinery and did not copy fixture
scores into the output:

| step | class | recorded path `nu` | raw L1 sites | certified path `nu` | recorded total `nu` rederived? |
| ---: | --- | ---: | ---: | --- | --- |
| 5 | S1 | 2 | 2 | undefined | no |
| 6 | Trunc | 2 | 2 | undefined | no |
| 7 | S2 | 5 | 5 | undefined | no |
| 8 | S3 | 10 | 10 | undefined | no |

All five registered structural mutations were rejected: dimension drift,
duplicate site, missing site, over-ceiling site, and anchor collision. Focused
adversarial replay tests additionally reject changes to a site disposition, a
historical verdict, or the typed-realization completeness bit. Exact comparison
replay also rejects a changed verdict-validity field or digest.

## Frozen artifacts and replay

- `docs/tdc1_typed_d4_certificate.json`: 25,979 bytes; SHA-256
  `8F7BBF16BBF36BD0076A924AE49A70D2EBAD827434692F444792464A58C4152B`;
  internal digest
  `blake3:c6fd0e40f08e24c6088691ce3d8ac352ca41a5ce4dfcfd8fa2d97d9ccc44bd2f`.
- `docs/tdc1_typed_d4_comparison.json`: 633 bytes; SHA-256
  `4CD78F379B3756DB25E86419756DCA1FA1176071BE45760432881656279151B4`;
  internal digest
  `blake3:af10d7b3807db8c008d84ec223e5a11814d2481380d3f7b2b2c01e76e3f19894`.

The comparison embeds the Phase-1 certificate digest exactly. Execution and
verification commands were:

```powershell
cargo run -p pen-search --example tdc1_typed_d4 -- certificate docs/tdc1_typed_d4_certificate.json
cargo run -p pen-search --example tdc1_typed_d4 -- replay docs/tdc1_typed_d4_certificate.json
cargo run -p pen-search --example tdc1_typed_d4 -- compare docs/tdc1_typed_d4_certificate.json docs/tdc1_typed_d4_comparison.json
cargo run -p pen-search --example tdc1_typed_d4 -- replay-comparison docs/tdc1_typed_d4_certificate.json docs/tdc1_typed_d4_comparison.json
cargo test -p pen-type tdc1
cargo test -p pen-eval tdc1
```

The certificate was committed before the comparison command was run. Replay
passed byte-for-byte with the certificate digest above.

## Exact next scope increment

Per the frozen certificate, the next implementation must proceed in this
order before reading any Step-16 score:

1. `add interval variables and a cofibration lattice to pen-type`
2. `add typed coe/hcom terms with oriented computation and union-staging replay`
3. `add a motive-typed PathCon eliminator and constructor beta rule`
4. `mint opaque per-site realization tokens only from those derivations`
5. `extend typed univalent equality/weakening to the new cubical family terms`
6. `rerun historical HIT regression before reading the Step-16 score`

No history, score, bar, or zone boundary was adjusted in response to the
negative gate outcome.
