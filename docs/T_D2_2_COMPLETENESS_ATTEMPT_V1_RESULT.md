# T-D2-2 completeness attempt v1 — lawful dependency stop

**Date:** 2026-07-23. **Status:** `StoppedOnTd21FiniteDomainDependency`. **Theorem disposition:** `NotAttemptedDependencyUnsatisfied`.

## Result

Dependency stop, not a completeness verdict: T-D2-1 has not produced an operative independent finite quotient. The literal kernel-accepted reading is obstructed by the closed family E_n = Susp^n(Univ), while the intended fixed depth-two fragment remains operationally undefined. The future least-fixed-point algorithm is registered but not executed.

No surjectivity or incompleteness verdict was issued. No domain-membership decision was run, and no generator, A3 inventory, or historical regression row entered a membership definition.

## T-D2-1 dependency binding

- source: `crates/pen-search/src/t_d2_1_domain_build_v1.rs` (present: `true`; digest: `sha256:b00bf929e44d762b75a1d6fbf883fc822f6567c9c062d883519a2cb726afe3b0`)
- certificate: `docs/t_d2_1_domain_build_v1.json` (present: `true`; digest: `sha256:79a7fea2d14801b7a83539eb0e4db38e25b7327c505fb689e84a4a25da2437d0`)
- report: `docs/T_D2_1_DOMAIN_BUILD_V1_RESULT.md` (present: `true`; digest: `sha256:5b455d509cedab1cbb0383cfe8b554dc39a9d6219a68cd765c393a4c2447a656`)
- typed certificate parse: `true`
- exact v1 schema: `true`
- typed result digest: `blake3:0811bf4686f42ead69f1d642ab47682933eeecaaa34d788da7a8e27b49214790`
- public T-D2-1 replay: `true`
- typed certificate authenticated: `true`
- source matches typed source binding: `true`
- report matches deterministic rendering: `true`
- complete dependency binding trusted: `true`
- declared schema (generic diagnostic): `t-d2-1-independent-depth-two-domain-build-v1`
- declared status (generic diagnostic): `stopped_named_gaps`
- declared T-D2-2 prerequisite (generic diagnostic): `Some(false)`
- disposition: `PresentNamedGap`
- finite-domain contract satisfied: `false`

The typed exact-v1 T-D2-1 certificate passed public replay and is bound as a named-gap result: the fixed depth-two fragment is not operationally defined and the literal kernel-accepted quotient is not finite.

Authenticated typed T-D2-1 gap IDs:

- `T_D2_1_FIXED_DEPTH_TWO_FRAGMENT_NOT_OPERATIONALLY_DEFINED`
- `TD21_NFSCH2_MEMBERSHIP_DECISION_NOT_TOTAL`
- `TD21_TOTAL_TYPED_NORMALIZATION_NOT_PROVED`
- `TD21_GENERAL_UNIVALENT_FAMILY_QUOTIENT_NOT_DECIDABLE`
- `T_D2_1_LITERAL_KERNEL_ACCEPTED_QUOTIENT_INFINITE`
- `TD21_CORPUS_REGRESSION_BLOCKED_UNTIL_DOMAIN_EXISTS`

Generic JSON gap diagnostics (non-authoritative):

- `TD21_CORPUS_REGRESSION_BLOCKED_UNTIL_DOMAIN_EXISTS`
- `TD21_GENERAL_UNIVALENT_FAMILY_QUOTIENT_NOT_DECIDABLE`
- `TD21_NFSCH2_MEMBERSHIP_DECISION_NOT_TOTAL`
- `TD21_TOTAL_TYPED_NORMALIZATION_NOT_PROVED`
- `T_D2_1_FIXED_DEPTH_TWO_FRAGMENT_NOT_OPERATIONALLY_DEFINED`
- `T_D2_1_LITERAL_KERNEL_ACCEPTED_QUOTIENT_INFINITE`

## Why literal kernel acceptance is not finite

Under the literal all-kernel-accepted reading, n -> E_n is an injection into closed, typed, beta-normal, zero-parameter family presentations. Frozen equality is syntactic equality of beta normal forms, so E_m and E_n remain distinct when m != n. Therefore the quotient is not finitely enumerable. The finite prefix is replay testimony for the generic source-level induction, not an enumeration bound.

| n | equation | accepted | ambient | normal fixed point | parameters | expression digest |
|---:|---|:---:|---:|:---:|---:|---|
| 0 | `E_0 = Univ` | true | 0 | true | 0 | `sha256:3e3c506aedaf940beeea91c14e0925e5f10ac5747d5d5db416174bc2d34e0e21` |
| 1 | `E_1 = Susp(E_0)` | true | 0 | true | 0 | `sha256:fcfd91b2b318896e83d6ef1a7816fb45d9d3fea42337ca3b0ee58ccacfd028cd` |
| 2 | `E_2 = Susp(E_1)` | true | 0 | true | 0 | `sha256:b12be861d51626be673bdea2b6502cae02ab7a7292a07a7e498b38cc9c6ecb06` |
| 3 | `E_3 = Susp(E_2)` | true | 0 | true | 0 | `sha256:9b3719212760b0ae2509f11eecf2dabb0b67a7ab6961a62124c01330d40b3c52` |
| 4 | `E_4 = Susp(E_3)` | true | 0 | true | 0 | `sha256:f2ea768463d910312aa11ebf3341ea8f3bea9399a5c5ca2364b001a69e2ea1eb` |
| 5 | `E_5 = Susp(E_4)` | true | 0 | true | 0 | `sha256:0b9fa8796cf11b96949a4f71c08b99287a5710c5b181715327f67b7a827bb14d` |
| 6 | `E_6 = Susp(E_5)` | true | 0 | true | 0 | `sha256:f012a85e0138fc350751a7a298406d186cb252905d22488933126e387794502c` |

The finite table is a replay regression for the generic induction, never an enumeration bound. `u32` constructor payloads do not bound recursive AST height, and the ambient-parameter cap does not bound expression nesting or telescope length. The previously adopted problem statement explicitly forbids replacing semantic Schema2 depth with raw AST node depth.

## Registered future theorem procedure

1. **seal-independent-carrier:** Replay T-D2-1 and seal the carrier, quotient, history digest, and depth tags before loading any generator reachability relation.
2. **initialize-zero-premise-reachability:** Mark only independently present carrier nodes concluded by verified zero-premise rules.
3. **saturate-frozen-rule-relation:** Iterate frozen rules to a least fixed point; each rule may mark an existing carrier node reachable but may not manufacture domain membership.
4. **recheck-typed-normal-equality:** Re-elaborate and normalize every conclusion, then compare through the T-D2-1 structural equality witness rather than a hash alone.
5. **check-both-inclusions:** Check D subset Reach for surjectivity and Reach subset D for rule soundness/closure; never silently discard an out-of-domain conclusion.
6. **publish-proof-or-refutation:** Emit derivations for every carrier class or publish the least unreached structural representative with its failed-rule frontier.
7. **recompute-depth-one-restriction:** Filter by the independently defined depth-one tag, rerun saturation, and only afterward compare the sealed depth-one campaign as regression.

This least-fixed-point procedure is registered but was not executed. It first seals an independent finite carrier, then permits generator rules only to mark existing carrier nodes reachable. It checks both `D ⊆ Reach` and `Reach ⊆ D`; missing nodes and out-of-domain conclusions are published, not suppressed.

## Gate and charge

T-D2-2 remains unattempted until T-D2-1 exports a generator-independent, decidable, finitely enumerable semantic carrier. This artifact changes no downstream gate. T-D2-3, BC3, M3, and M4 receive no authorization from this artifact. Charge: κ = 0, ν = 0, anchors = 0.

## Source bindings

| path | role | bytes | SHA-256 |
|---|---|---:|---|
| `docs/depth_two_domain_adjudication.md` | adopted T-D2 law and boundedness stop | 6825 | `sha256:fffd48c0eb5f89b6728842aa6f9de5aa43013ab867278d6b8ec6b2737ed92151` |
| `docs/step_15_completion_open_problem.md` | prior Schema2 grammar requirement and raw-AST warning | 15507 | `sha256:fe3b2eb8f17f12563596d7a937d39d2618625503cf4ba1af8bba33e224309c82` |
| `crates/pen-core/src/expr.rs` | recursive expression carrier and structural equality | 10207 | `sha256:f4da000f35a343fe6ad749c36e12d56330c8787efc73bb9c01a0289f65ea2531` |
| `crates/pen-core/src/telescope.rs` | unbounded clause-vector representation | 19443 | `sha256:be2d6e9dc0c111279c5535c9b933ea0a157837e2fb5af59ece6b8cba5adfafa0` |
| `crates/pen-type/src/elaborate.rs` | frozen kernel acceptance and Susp closure | 102010 | `sha256:6b713bb5286648a1c7a22c55a2446af8eca58223343ce2cd716c49748e447719` |
| `crates/pen-type/src/normalize.rs` | frozen normalizer and Susp preservation | 17962 | `sha256:7d3398b60708bf1ac132609e0bf84d6a26383f9dcc90f11f99d5803ed9da73f2` |
| `crates/pen-type/src/equality.rs` | frozen beta-normal syntactic equality | 4757 | `sha256:3aa5fd8d4e9667c1534f39fc0c76ef956e6372301b767f2442c918c6b85c3401` |
| `crates/pen-eval/src/typed_families.rs` | adopted family presentation quotient | 53286 | `sha256:8ac9d5f76c8e965aee6ef43244a4df1b8ec6bd264e14a3eac52d8d56f2319c67` |
| `crates/pen-search/src/t_d2_2_completeness_attempt_v1.rs` | this theorem-attempt issuer and replay | 62282 | `sha256:3e3d3bcb2e527bf57163b67c1c640e649db02685a91727a3c59c4d5d4bb3de8f` |
| `crates/pen-search/src/t_d2_1_domain_build_v1.rs` | T-D2-1 domain-build implementation | 67179 | `sha256:b00bf929e44d762b75a1d6fbf883fc822f6567c9c062d883519a2cb726afe3b0` |
| `docs/t_d2_1_domain_build_v1.json` | T-D2-1 create-new certificate | 35882 | `sha256:79a7fea2d14801b7a83539eb0e4db38e25b7327c505fb689e84a4a25da2437d0` |
| `docs/T_D2_1_DOMAIN_BUILD_V1_RESULT.md` | T-D2-1 deterministic result report | 5752 | `sha256:5b455d509cedab1cbb0383cfe8b554dc39a9d6219a68cd765c393a4c2447a656` |

Certificate digest: `sha256:ff8bc3d979edb1557e841457ff5a47ab336059fc3fe9c10ffa22b26f65e854bc`
