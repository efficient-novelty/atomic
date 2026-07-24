# T-D2-3 candidate/provenance key-schema result

**Date (syntax identifier):** 2026-07-23. **Status:** `schema_frozen_named_integration_gap`. **Certificate:** `blake3:9628bc38f5f20d4d7dfa065039d70ee6667b1559ee1af0259144da19b12079a7`.

The key schema is frozen before any candidate-level join is run. No join artifact or desired verdict is an input. M-3 v1 remains authoritative and M-4 remains unauthorized.

## Frozen key schema

Schema digest: `blake3:e3c859c3bf0589b23c106d21bbdd4658ec54294d279968557656cf3c2e716d63`. Ordered field surface: **14 [register: `artifact_metadata`; meaning: ordered schema-field surface size]**. Candidate digest, telescope clause path, and the typed coordinate bundle are mandatory. Every field rule is outcome-independent and selector/value/bar-independent.

| Field | Source authority | Exact derivation |
|---|---|---|
| `schema_digest` | versioned T-D2-3 schema definition | digest of this ordered field-rule vector with its digest slot cleared |
| `candidate_digest` | frozen kernel candidate_hash | BLAKE3 of the serialized telescope content |
| `telescope_clause_path` | sealed Telescope clause order | candidate-root plus exact zero-based clause occurrence |
| `clause_content_digest` | sealed ClauseRec content | BLAKE3 of the exact declared role and raw expression |
| `signature_digest` | frozen SealedSignature | exact signature digest used for candidate elaboration |
| `visible_library` | frozen elaborator invocation | exact visible signature prefix bound supplied to elaboration |
| `ambient_and_free_scope` | frozen elaborator | minimal ambient parameters and clause-local free-scope length |
| `candidate_elaboration_derivation` | frozen elaborator | derivation hash returned by successful whole-candidate elaboration |
| `prior_kernel_role_prefix` | frozen elaborator | digest of kernel-derived roles strictly preceding the clause |
| `kernel_role_and_type` | frozen elaborator | kernel-derived role plus digest and constructor of the clause classifier |
| `typed_normal_form` | frozen normalizer through elaboration | digest and constructor of the clause normal form |
| `canonical_presentation` | adopted natural-family quotient | digest of the canonical normal form, parameter sorts, and checked renaming |
| `typed_coordinate_digest` | T-D2-3 typed bundle | digest of every preceding typed coordinate |
| `key_digest` | T-D2-3 key issuer | digest of the complete candidate-and-clause key with this slot cleared |

## Sealed-content regression

Candidates: **15 [register: `artifact_metadata`; meaning: sealed historical candidate regression surface size]**. Clauses: **64 [register: `artifact_metadata`; meaning: sealed historical clause regression surface size]**. Issued keys: **64 [register: `artifact_metadata`; meaning: issued historical key regression surface size]**. Every sealed candidate is typed, every clause has one key, key digests are unique within each candidate, and the schema digest is constant across the regression.

| Entry | Candidate digest | Clause keys | Replay |
|---|---|---|---|
| 1 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:61631d63a5877aad1b32b1e71aa6f0e555317b65732f327e3f38e32c222e29e7` | 2 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 2 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:a2dfff0fb8ce1073119da3893e1d195247c234af66b9a7de17c85d5cf718f555` | 1 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 3 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:934b5599bb0f28abf0be9652982caec5e0ff6d7d204ddaa4e66f23c37155457e` | 1 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 4 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407` | 3 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 5 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:043c7990d42d91f972544a3cbcae33ca65e48eaab82d605a15cb505ed0ad26d0` | 3 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 6 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:a2ff7bc69a678e8c257824465993f8aadf71c05676562887da12dce3ae6cfe1d` | 3 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 7 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:30f190b67aab5179a6de8dccee6c29c699b80c12bef676bf2e9f9377595a7906` | 3 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 8 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:e01de7b95b0dda56a1062add33387a37709e23d3d6ea6ce4efaf27adacb868c0` | 5 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 9 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:f57e3a5aa44003adb5f8054013e549e4f3065757d88922313478e8e445825491` | 4 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 10 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:93289041755cf4c4e0396029273822630028999d945359b10bb88e2376b9788e` | 4 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 11 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:03cc71839428ceab088e386e31e6c72b7ca4a1e12c9309557efc76826dcb88cd` | 5 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 12 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:0d06e3b14bfd7c1bd16d9f66039e016f84a6162ce85584f7c641753b833b1dcb` | 6 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 13 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:b09b3f832bef16953747c213e6b8a548f594339dc7db9148e7ae60a6c53a4cf1` | 7 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 14 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:1ff4820f2272c022a5fece1032aa9647e41167a3dd8f62765e84e22f5d90b19c` | 9 [register: `artifact_metadata`; meaning: issued key surface size] | true |
| 15 [register: `syntax_identifier`; meaning: sealed historical entry identifier; syntax identifier only] | `blake3:e919c8419bbafde89e3e99ff25f348f3c8b679ed22685e84d3cdec634ebf90d4` | 8 [register: `artifact_metadata`; meaning: issued key surface size] | true |

## Static integration audit

The candidate kernel exposes the telescope digest; typed-family and provenance modules address occurrences by candidate identity plus clause coordinate. The new schema supplies those coordinates and their kernel derivations. The aggregate key and fold were audited as source contracts only and were not executed.

Remaining gap: `TD23_AGGREGATE_DP_DOES_NOT_EXPORT_COMPLETE_CANDIDATE_CONTENT_STREAM` — The current aggregate dynamic program maps expression classes into aggregate rows and does not export a complete stream of candidate Telescope payloads. The frozen T-D2-3 key schema can issue one replayable key per clause for any explicit typed candidate, but it cannot materialize keys for candidate payloads the aggregate API does not expose.

## Zero charge and gate

Minted kappa: **0 [register: `artifact_metadata`; meaning: kappa minted by key description]**; semantic-family nu: **0 [register: `artifact_metadata`; meaning: semantic-family nu minted by key description]**; anchors: **0 [register: `artifact_metadata`; meaning: anchors minted by key description]**; demand orbits: **0 [register: `artifact_metadata`; meaning: demand orbits minted by key description]**. No selector, value, bar, or enumeration-order selector was read. M-3 successor issued: **false**; M-4 authorized: **false**.

## Permitted conclusion

The versioned candidate/provenance key schema is frozen outcome-blind. For every explicit candidate accepted by the frozen kernel, it deterministically issues one key per telescope clause, carrying candidate digest, exact clause path, and kernel-derived typed coordinates. The sealed historical regression is complete. The current aggregate API does not export the complete candidate-content stream needed to materialize this schema over its aggregate surface, so no candidate-level join or full aggregate inventory is claimed.

## Required successor

Add a versioned candidate-content stream or a lossless candidate-preserving refinement to the aggregate generator, feed each explicit candidate through the already-frozen T-D2-3 key issuer, and only afterward run a separately versioned candidate-level join. The key schema may not be revised in response to that join.
