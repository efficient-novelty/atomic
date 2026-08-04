# JG2b1b2 Complete-Through-Head Replay

Date: 2026-08-01

Status: JG2b1b2 and protocol-only JG2b2a are discharged. Full JG2 remains open;
JG2b2b indexed interfaces and generic substitution metatheory is active.

## Result

`crates/pen-generative-audit/src/complete_sealed_history.rs` implements the
first complete target-neutral generative-history authority, with the exact
theorem

```text
CompleteThroughHead(H, h)
```

for the finite history `H` owned by one consumed process-local JG2b1b1 writer
epoch and its producer-designated finalized head `h`. This is not global EOF,
latest-branch, actual-current, persistent-store, or cross-process authority.

The sole public verifier is:

```text
verify_complete_target_neutral_generative_history_through_head_v1(
    closed: ClosedGenerativeSealedLogV1,
    independent_kernel: &Kernel,
)
    -> Result<
        VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1,
        CompleteTargetNeutralGenerativeHistoryThroughHeadFailureV1,
    >
```

The verifier accepts no protocol/JG token, transcript, frame or stage slice,
event count, boundary, branch ID, head, finalized digest, EOF flag, caller
resource limit, or registered-history substitute. `closed` is consumed by
value into a crate-private owned replay-parts bundle; failure drops that bundle
without minting authority, and success cannot leave an intact closed artifact
beside the resulting token.

## Independent reconstruction

The verifier performs the following fail-closed sequence:

1. compare the independently supplied kernel protocol, normalizer, and sealed-
   log resource configuration to the protocol embedded in the owned state;
2. check the owned JG1/constructor binding, derive all V1 limits from that
   kernel, scan the complete owned frame vector, and reserve the exact fresh-
   stage output capacity fallibly;
3. independently verify empty genesis and recompute both producer genesis
   commitments;
4. enumerate every owned frame to derive its zero-based ordinal, never using a
   stored or caller count to choose a prefix;
5. freshly replay each exact extension, derive its sealing subject, reconstruct
   and verify free sealing, and reconstruct the exact JG2a stage;
6. compare every retained predecessor, successor, normalized extension,
   candidate, sealing subject, stage manifest, normalized/exact frame, and
   pre/post-head field;
7. recompute checked retained-extension and aggregate-material totals, the full
   terminal boundary, the normalized log head, and the rolling exact branch
   head;
8. recompute the producer-finalized-head commitment from only those derived
   facts and compare it to the consumed producer commitment;
9. derive JG2b1a afresh from exactly the full reconstructed JG2a vector, check
   every event correspondence, replay its terminal boundary, and require exact
   declaration-birth coverage; and
10. only then mint
    `VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1`.

The producer's `normalized_log_head_digest` and JG2b1a's
`normalized_history_digest` remain deliberately distinct commitment domains.
The rolling `exact_branch_head_digest` is likewise distinct from the
producer-finalized-head commitment.

## Resulting authority

The result is opaque, non-cloneable, non-formattable, non-serializable,
non-default-constructible, and not publicly canonically encodable. It retains
the freshly reconstructed JG2a stages and exact JG2b1a chain privately for
JG2b2 while discarding all unchecked producer frames.

Its complete-through-head commitment binds:

- the JG2b1b2 schema and resource-policy versions;
- JG2b1b0 protocol, JG1, constructor, scope-grammar, kernel, normalizer, and
  both kernel-configuration identities;
- the opaque process-local branch identity;
- normalized rolling, exact rolling, and producer-finalized heads;
- derived event, retained-extension, aggregate-material, terminal, and birth
  counts;
- the complete normalized terminal boundary; and
- JG2b1a normalized-history, normalized-chain, exact-evidence, and exhaustive
  declaration-birth commitments.

The frozen complete-through-head codec fixture is:

```text
blake3:9d50883ed51e02d9deb86b4aaf97432f8165f35474e9a06c55370456ba74d4e6
```

## Resource and adversarial coverage

The resource policy remains kernel-profile-relative: event count is bounded by
`max_depth`; individual and aggregate retained declarations by
`max_operations`; aggregate inspected predecessor/extension/successor material
by checked `max_operations * (2 * max_depth + 1)`; and the reconstructed-stage
vector by fallible exact reservation. Other allocator aborts yield no token.

The test suite covers:

- empty and multi-event complete histories using an independently supplied
  matching kernel;
- every retained digest and pre/post-head field;
- candidate mutation, gap/ordinal drift, reorder, duplicate, trailing frame,
  interior omission, prefix truncation, and branch substitution;
- terminal boundary, normalized head, exact head, totals, and finalized-head
  corruption;
- independent drift of `max_operations`, `max_depth`, and normalization fuel;
- complete JG2b1a terminal replay and declaration-birth coverage;
- compatible parallel branches that share normalized identity but retain
  distinct exact, finalized, and complete-through-head commitments;
- split and coalesced histories that reach the same terminal declarations but
  remain different complete histories; and
- compile-fail proofs for the consuming API and forbidden cloning, formatting,
  serialization, default minting, canonical encoding, borrowed replay, repeat
  replay, and private-frame access.

## Explicit non-theorems

JG2b1b2 does not certify:

- global EOF, globally latest history, or the actual current external branch;
- persistent identity, exclusive external storage ingress, crash recovery,
  signatures, or anti-equivocation;
- a unique segmentation of a terminal signature;
- retrospective promotion of a caller-selected transcript or prefix;
- a generic arbitrary-typed-substitution theorem, complete occurrence census,
  or universal naturality;
- birth-local predecessor closure or typed paid/prior-live-output support;
- a raw generative-capability carrier, constructor classification, quotient,
  weakening, marginal, provenance, `gamma`, bootstrap, debt, or selection; or
- absence of further events from resource exhaustion or process termination.

## Ordered continuation: JG2b2b

The JG2b2 entry audit established that finite substitution samples cannot prove
naturality under every admissible substitution. JG2b2a therefore freezes a
protocol requiring an executable structural occurrence grammar, indexed-
interface realizations, and a generic typed-substitution/naturality theorem
distinct from the later finite history census. JG2b2b must now supply those
calculi and the exact theorem-subject package. JG2b2c will then consume this
opaque complete-through-head authority—not a relative JG2b1a chain or caller
history—and derive the exact finite occurrence-by-constructor disposition
matrix, aggregate classification, and full concrete theorem-application
census. Boolean `natural` claims, substitution samples, caller occurrence
lists, and later/held-out evidence remain inadmissible.
JG2b3 and JG2c remain blocked until JG2b2 is discharged.
