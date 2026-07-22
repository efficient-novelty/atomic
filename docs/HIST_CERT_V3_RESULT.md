# HIST-CERT v3 result: boundary replay closes F-B2, ordinary certification remains open

**Execution date:** 2026-07-19  
**Artifact:** `docs/hist_cert_v3.json`  
**Schema:** `hist-cert-historical-hit-v3`  
**Artifact length:** 178,331 bytes  
**SHA-256:** `90AA3B75513F5E0F845635F0B15349D8544E6804D7C89433CF3FA8F58D792136`  
**Git blob:** `bfff08c420be85f1ebd283fb5b249a336c7dcee0`  
**Result digest:**
`blake3:1984ef55c1687749c0eff8f7109aaca316755251a36d78686a7299c8572877f3`

## Outcome

HIST-CERT v3 consumes the adopted
`pathcon-attachment-declared-bound-boundary-theory-axiom-v3-element-overlay`
and replays the four historical boundary rows without importing the bar or a
Step-16 candidate. The raw historical values remain `7/8/10/18`. The
registered path has the scoped typed-and-marginal classification
`2/2/5/10`, leaving the arithmetical ordinary remainder `5/6/5/8`.

This discharges **F-B2**: the registered boundary path now has a source-bound,
typed, replayable certificate. The classification is not yet novelty credit:
the public-key bijection alone supplies neither EGP demand-orbit provenance
nor a proof of independent charge. It therefore does **not** discharge
**F-T1**. Every full `certified_total` remains `null`; no historical total is
contradicted, but none is yet fully certified by this run.

| historical step | raw total | typed + marginal registered path | ordinary unresolved | full `certified_total` |
| --- | ---: | ---: | ---: | --- |
| 5 | 7 | 2 | 5 | `null` |
| 6 | 8 | 2 | 6 | `null` |
| 7 | 10 | 5 | 5 | `null` |
| 8 | 18 | 10 | 8 | `null` |

## What v3 newly checks

The v3 replay joins the exact historical B4/B5/B6/B7 prefix to the registered
boundary evidence. Where an element overlay is present, the opaque v3 bundle
replays the role, overlay, base, and typed-token chain. It also checks the
registered endpoint-Trunc bundle and exact public `PathSchemaKey` membership
in each typed bundle. Realization hashes are deliberately retained as an
unkeyed whole-bundle inventory: the public correspondence and realization
arrays are not row-aligned at dimensions two and three, so replay makes no
per-row positional claim about them.

The independently rebuilt predecessor inventory is `[0,2,4,9]`, and the
complete current-versus-predecessor comparison inventory is `[0,4,20,90]`.
Every non-vacuous comparison is separated by fresh owner support under its
equality-free sealed prefix. These are audit inventories, not certified
novelty totals.

The element overlay is used only at historical steps 5, 7, and 8. The Trunc
realizer is deliberately overlay-free. Thus `2/2/5/10` is a scoped
classification of the registered path; it is not an assertion that those
families carry independent novelty credit or that the remaining ordinary
families are independent, exhaustive, or marginal.

## Why F-T1 remains open

The ordinary remainder still lacks a machine-checkable certificate carrying
all of the information needed by the Selective-Law audit: a typed
natural-family witness, weakening/erasure inverse evidence, a predecessor
sweep, injective demand-orbit provenance, and a proof that uniform
specializations have not been multiplied without independent export.

HIST-CERT v3 therefore cannot manufacture the missing ordinary contribution
from the raw trace or turn a key bijection into EGP provenance. The only sound
result is to retain the raw observations, classify the registered path, and
leave the full totals partial.

The retained proof gaps are: arbitrary C1, exhaustiveness and independence of
the proposed basis, EGP demand-orbit/independent-charge provenance, general
C6, candidate-level C8, and the general schema-classification theorem. The
registered boundary replay closes none of those broader obligations.

## Falsifier disposition

| falsifier | v3 disposition |
| --- | --- |
| F-O1 | discharged for the registered element-overlay rows at steps 5, 7, and 8 |
| F-O2 | unchanged and already discharged for the registered S1/S2/S3 term-typing rows; Trunc is handled by its separate endpoint bundle |
| F-O3 | unchanged; the earlier conservativity result remains archival evidence |
| F-B2 | discharged by the exact-prefix boundary replay and typed realization join |
| F-T1 | open for the ordinary `5/6/5/8` remainder |

## Replay and mutation verification

| check | recorded result |
| --- | --- |
| `pen-type` full library | 355 passed; 0 failed |
| `pen-eval` full library | 191 passed; 0 failed |
| `pen-eval` HIST-CERT v3 focused suite | 6 passed; 0 failed |
| mutation/replay coverage | exhaustive scalar and nonempty-vector mutations, strict unknown/duplicate JSON, omitted-comparison and credit-promotion mutations all rejected |
| create-new generation | passed; the resolved target was absent and `create_new` emitted one new artifact |
| public JSON replay | `valid: true`; both regression vectors true; full totals partial; F-T1 false |
| archival bindings | exact HIST-CERT v1 and TRUNC-ER v1 bytes pinned and both public definition replays passed |
| independent semantic review | passed with no P1/P2 findings |

Reproduction:

```powershell
cargo test -p pen-type --lib
cargo test -p pen-eval --lib
cargo run -p pen-eval --example hist_cert_v3 -- emit docs\hist_cert_v3.json
cargo run -p pen-eval --example hist_cert_v3 -- replay docs\hist_cert_v3.json
```

## Honest conclusion

HIST-CERT v3 closes the boundary-to-typed-realizer exposure F-B2 and fixes the
registered path classification at `2/2/5/10`; its novelty credit remains
undefined. It leaves F-T1 and the ordinary `5/6/5/8` remainder open, so all
full historical `certified_total` fields are `null`. Consequently this result
establishes neither a later candidate nor a later acceptance, Step 16, or the
global halt at fifteen.
