# T-BF1 prefix re-derivation: F-BF1 fires at Stage 4

**Date:** 2026-07-21  
**Law tested:** \`demand-parsimony-law-v1\`  
**Artifact:** \`docs/t_bf1_prefix_v1.json\`  
**Schema:** \`t-bf1-prefix-v1\`  
**Result digest:** \`blake3:f14445c4e66ffad3f1f18dea6476983d98c252381ddf30f9623a259103428757\`  
**SHA-256:** \`E142E87292C6B22830A5F13B165BCC92FEBCE2A8CE10308C3CECF91858E834B3\`  
**Length:** \`10479\` bytes

## Outcome

T-BF1 is **not proved**. Its preregistered falsifier F-BF1 fires at
Stage 4.

The selector was permitted to inspect only the ordered pair

\`\`\`text
(kappa, certified_nu)
\`\`\`

over the complete canonical strict-discharge cone. Stage 4 has four
distinct typed strict total dischargers, and every one has the same
minimum pair:

\`\`\`text
(kappa, certified_nu) = (3, 5).
\`\`\`

Accordingly the run stops at Stage 4. It does not select a winner and it
does not reach Stages 5--7. The proposal's adoption block remains locked.

## Prefix replay

| Stage | Jurisdiction | Standing demand | Engine focus | Complete enumerated | Strict admitted | Result |
| ---: | --- | --- | --- | ---: | ---: | --- |
| 1 | constitutive | none | none | 288 | 1 | registered |
| 2 | constitutive | none | none | 33 | 1 | registered |
| 3 | constitutive, pre-structural | \`former_eliminator\` | none | 56 | 1 | registered; wrinkle preserved |
| 4 | demand parsimony | \`former_eliminator\` | \`FormerEliminator\` | 4 | 4 | F-BF1: four equal minimizers |
| 5--7 | not reached | -- | -- | -- | -- | forbidden after F-BF1 |

Stage 3 therefore retains the exact recorded wrinkle: the
\`former_eliminator\` demand is already present, but the pre-structural
lane has no focus jurisdiction yet. The implementation does not move the
demand forward to Stage 4 or erase it from Stage 3.

## The Stage-4 tie

| Candidate hash | Canonical key | kappa | certified nu | bit-kappa (audit only) |
| --- | --- | ---: | ---: | ---: |
| \`blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407\` | \`0e9aa132\` | 3 | 5 | 49 |
| \`blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308\` | \`4ede818c\` | 3 | 5 | 49 |
| \`blake3:b4f821d9bb28366d8ae37adc7c1cb3e28c8a0de60f05c81e9ad75ba4f8b0edd4\` | \`b516ff09\` | 3 | 5 | 50 |
| \`blake3:4b2211ecae25f3afbb1187f3a4a1b644edfc6adbd3512b9ca27c64a7b731265b\` | \`d860242f\` | 3 | 5 | 50 |

The current engine re-enumerated this Stage-4 cone directly from the
constitutively registered Steps 1--3, without consuming the v2 history
certificate chain. The four hashes, canonical keys, kappa values, and
structural nu values exactly match the frozen certificate.

The diagnostic bar, bit-kappa, canonical key order, candidate hash order,
and certified reference winner were unavailable to the selector. In
particular, the visible bit-kappa split cannot be imported as an
unregistered third comparison key. Doing so would silently change the
proposed law and violate F-BF1.

## Replay boundary

The frozen v2 program and burn digests validate, their linkage validates,
and the v3 artifact imports exactly the same Steps 1--7 prefix. The old
v2 *live definition* replay does not currently validate transitively:
later grammar-completion / membership sources have drifted since that
burn. This is serialized as
\`v2_live_definition_replay_valid = false\`, with the exact error text.

That drift is not hidden. The new T-BF1 issuer independently recomputes
the decisive current Stage-4 cone and nu values, and separately binds the
immutable v2/v3 records. Both routes yield the same four-way tie.

## Consequence

The bar-free proposal cannot be adopted as written. The lawful next step
is a separate, verdict-blind adjudication that either:

1. proves some of the four candidates equivalent under an already adopted
   semantic equivalence and reruns canonical quotienting; or
2. produces an independently certified nu refinement that distinguishes
   them; or
3. explicitly amends the parsimony order in a versioned successor law.

No desired winner, bar, bit-cost, or hash-order rule may be introduced
retroactively. Until such an adjudication exists, T-BF1, T-BF3 enactment
equivalence, and the bar-free adoption block cannot all be true as stated.

## Commands

\`\`\`powershell
cargo test -q -p pen-search t_bf1_honors_stage4_f_bf1_instead_of_importing_a_hidden_tie_break -- --nocapture
cargo run -q -p pen-search --example t_bf1_prefix -- create-new docs/t_bf1_prefix_v1.json
cargo run -q -p pen-search --example t_bf1_prefix -- replay docs/t_bf1_prefix_v1.json
\`\`\`

The targeted test passed. Create-new and the independent public replay
both returned \`valid: true\`,
\`outcome: stopped_f_bf1_parsimony_tie\`, \`stopped_stage: 4\`, and
\`minimizer_count_at_stop: 4\`.
