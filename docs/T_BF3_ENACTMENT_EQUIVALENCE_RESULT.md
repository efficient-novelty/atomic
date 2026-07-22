# T-BF3 enactment-equivalence audit

Date: 2026-07-21

Status: **negative create-new result; T-BF3 is not proved**

## Outcome

The threshold-free law in 'docs/bar_free_law_proposal.md' does not enact a
sequence through the frozen certified domain. Its first comparative stage,
Stage 4, has four canonically distinct typed total dischargers, and all four
have the same proposed parsimony key:

    (least clause kappa, then least certified semantic nu) = (3, 5).

Consequently F-BF1 fires. The run halts at Stage 4, issues no winner, and does
not inspect Stages 5--15 as sequential selections. The T-BF3 theorem flag is
false and the bar-free proposal remains non-adoptable.

The immutable result is:

- 'docs/t_bf3_enactment_equivalence_v1.json'
- digest:
  'blake3:06e8727179d3f46b8ac829309f61d80f0bc04eb481b7b980baa5951d1728bd63'

## What the selector was allowed to read

For a candidate to enter the comparison, it first had to occur in the frozen,
complete, canonically deduplicated strict-admissibility cone and carry kernel
typing plus certified score provenance. The selector then read exactly:

1. clause kappa;
2. certified semantic nu.

It did not read the old winner, diagnostic bar, overshoot, bit cost, canonical
key, exact acceptance rank, or candidate input order. The recorded certified
winner is compared only after a unique minimum exists. A unit test reverses
the candidate order and obtains the same four-way tie.

## Bootstrap and the Stage-3 wrinkle

Stages 1--3 replay as the three unique constitutive registrations from the
frozen v2 evidence. The artifact preserves the wrinkle explicitly:

- Stages 1 and 2 export no required package;
- Stage 3 is still registered in the constitutive prefix, while its window
  exports 'former_eliminator'.

No comparative parsimony claim is made for these three registrations.

## The Stage-4 counterexample

The Stage-4 frozen cone has:

    required package       former_eliminator
    enumerated              4
    strict-admitted         4
    canonically deduped     4
    typed scored members    4

Every member has kappa = 3 and certified nu = 5:

| Candidate hash | kappa | certified nu |
| --- | ---: | ---: |
| 'blake3:2016726758f30ee3f1dc73b5e89388f2c5d0f6059cd2c3a82aaa01f2b89a3407' | 3 | 5 |
| 'blake3:43a0ed7077700a3c4a917d9ac9e327f5b91d3d3c9d87048ce60b58f86b493308' | 3 | 5 |
| 'blake3:4b2211ecae25f3afbb1187f3a4a1b644edfc6adbd3512b9ca27c64a7b731265b' | 3 | 5 |
| 'blake3:b4f821d9bb28366d8ae37adc7c1cb3e28c8a0de60f05c81e9ad75ba4f8b0edd4' | 3 | 5 |

The predecessor's exact rank selected the first hash, but importing that rank
would add a selector absent from 'demand-parsimony-law-v1'. It would therefore
evade, rather than discharge, F-BF1.

This is not F-BF2: there is no bar-free winner that differs from the certified
winner. There is no winner at all because the stated order is not total on the
admissible cone.

## Frozen-surface hygiene

The run validates the internal digests of both v2 and v3 program/burn pairs and
all cross-artifact links:

- v2 burn to v2 program and history;
- v3 program to the imported v2 program and burn;
- v3 burn to the v3 program and imported v2 burn;
- v3 burn to its frozen issuer source hash.

The certificate is explicitly relative to that immutable issued surface. It
does **not** claim that the current development tree still replays every
source-bound upstream prerequisite: later grammar and normalization work has
advanced that live surface. This distinction is encoded in the JSON rather
than hidden as a warning.

## Consequences

1. T-BF3 is false for the proposal as written.
2. T-BF1 must encounter the same Stage-4 F-BF1 stop if it uses this exact
   candidate domain and parsimony order.
3. The adoption block in 'docs/bar_free_law_proposal.md' remains locked,
   independently of the E-5 successor and T-BF2.
4. The result does not run T-BF2, adopt the law, run the bridge, or make any
   claim about semantic O(16).

There are only two lawful continuations:

- prove, without consulting the desired winner, an independent equivalence
  theorem that places the four Stage-4 cone members in one selection class; or
- version the proposal and adjudicate an additional comparative selector,
  with its own tie falsifier, before rerunning T-BF1 and T-BF3 create-new.

## Replay

Implementation:

- 'crates/pen-search/src/t_bf3_enactment_equivalence.rs'
- 'crates/pen-search/examples/t_bf3_enactment_equivalence.rs'

Commands executed:

    cargo test -p pen-search t_bf3_enactment_equivalence::tests -- --skip full_certificate_honors_f_bf1_at_stage4
    cargo run -p pen-search --example t_bf3_enactment_equivalence -- create-new docs\t_bf3_enactment_equivalence_v1.json
    cargo run -p pen-search --example t_bf3_enactment_equivalence -- replay docs\t_bf3_enactment_equivalence_v1.json

The focused selector tests passed 2/2. Create-new replay and the subsequent
standalone replay both returned valid=true, theorem_issued=false,
first_unresolved_stage=4, minimizer_count=4, F-BF1=true, and no replay errors.
