# JG2b1b1 Process-Local Linear Append Producer

Date: 2026-08-01

Status: JG2b1b1, its JG2b1b2 consuming replay, and protocol-only JG2b2a are
discharged. JG2b1b is complete; full JG2 remains open at active JG2b2b.

## Result

`crates/pen-generative-audit/src/linear_append_log.rs` implements the first
prospective owner of a target-neutral generative history. It is the adapter
from the kernel-only JG2b1b0 lifecycle protocol to the existing JG1/JG2a
generative authorities.

The public lifecycle is:

```text
begin_generative_sealed_log_v1(protocol, JG1, constructors, kernel)
    -> OpenGenerativeSealedLogV1

append_verified_generative_stage_v1(
    &mut open,
    trusted_scope,
    exact_candidate,
    verified_free_sealing,
)
    -> GenerativeSealedAppendCommitmentV1

close_generative_sealed_log_v1(open)
    -> ClosedGenerativeSealedLogV1
```

`OpenGenerativeSealedLogV1` and `ClosedGenerativeSealedLogV1` are opaque,
non-cloneable, and non-serializable. There is no public constructor from an
event slice, count, digest, EOF flag, terminal boundary, or prior chain token,
and there is no transition from closed state back to open state. The closed
artifact, retained frames, and process-local branch ID do not implement
`Debug`, preventing recursive formatting from disclosing retained transcript
evidence or branch identity internals. The branch ID exposes only opaque
equality: its allocation number has no public getter, canonical encoder, hash,
or ordering surface; exact commitment encoding remains private to the
producer module.

## Producer authority

`begin_generative_sealed_log_v1`:

- requires the JG2b1b0 protocol token to bind the same kernel, normalizer, and
  kernel resource configuration;
- requires the constructor grammar to bind the supplied JG1 grammar;
- independently verifies the empty kernel boundary; and
- allocates a checked atomic branch identity internally.

That identity is unique only among successfully allocated branches in this
process. It is deliberately not a persistent, cross-process, signed, or
globally latest identity. Parallel branches with identical normalized events
share normalized log-head identity but have distinct exact branch and finalized
commitments.

Every successful call to `append_verified_generative_stage_v1` is exactly one
historical event. The caller supplies no ordinal, event ID, delimiter, parent,
head, or stage token. The writer:

1. derives the zero-based ordinal from its private frame count;
2. applies all checked event/material limits;
3. calls `verify_generative_capability_stage_surface_v1` with its own current
   boundary, thereby reconstructing JG2a internally;
4. commits the normalized predecessor, extension, and successor;
5. separately commits the branch identity and exact scope/candidate/sealing/
   stage evidence; and
6. mutates the private boundary, heads, counters, and frame store only after
   all checks and fallible reservation succeed.

A failed append leaves the writer count, boundary, both heads, resource
counters, and retained frame sequence unchanged. Resource failure never closes
the writer and never returns a shorter prefix token.

## Commitment separation

The producer maintains two rolling commitments.

The normalized-log path binds:

- the JG2b1b0 protocol, JG1, constructor/scope grammar, kernel, normalizer, and
  both resource-configuration identities;
- the internally derived ordinal and previous normalized head; and
- the complete normalized predecessor, extension, and successor boundaries.

It excludes process-local branch identity, arbitrary trusted-scope fields, raw
candidate presentation, and exact sealing provenance. Thus independently
opened branches with the same normalized transitions compare equal here.
The closed-state getter is named `normalized_log_head_digest`; it is distinct
from JG2b1a's independently reconstructed `normalized_history_digest`.

The exact-evidence path additionally binds:

- the process-local branch identity;
- the previous exact append-state head;
- the full trusted scope binding and digest;
- the exact candidate wire and candidate digest;
- the verified sealing scope, subject, and complete sealed successor; and
- the exact JG2a stage-manifest and sealing-subject digests.

`exact_branch_head_digest` is only a rolling append-state head. It exists while
the writer is open and is explicitly not finalization evidence.

`close_generative_sealed_log_v1` is infallible after construction and consumes
the sole writer. It mints the distinct
`producer_finalized_head_commitment_digest`, binding the branch identity,
derived event count, both terminal append heads, complete terminal boundary,
retained extension count, aggregate material, and all protocol/JG/kernel
identities. Closing is an out-of-band control transition, not another
generative event. An empty close is a valid finalized empty producer epoch.

The closed token is producer-designated finalization evidence, but not yet an
independently replayed complete-history token.

## Resource contract

The implementation discharges the JG2b1b0 V1 resource rule:

- event count is bounded by kernel `max_depth`;
- retained extension declarations are bounded by `max_operations`;
- each exact JG2a verification enforces the kernel's own operation, depth, and
  normalization-fuel limits;
- aggregate predecessor/extension/successor material is bounded by checked
  `max_operations * (2 * max_depth + 1)` arithmetic; and
- retained-frame vector growth uses `try_reserve` before state mutation.

Other transient Rust allocations may abort the process on allocator failure;
such an abort returns no append receipt, closed state, or finalized-head
authority.

All V1 counters are proved to fit `u64` before the infallible consuming close:
event count is `u16`-bounded, retained material is `u32`-bounded, and the
aggregate product is below `u32::MAX * (2 * u16::MAX + 1)`.

## Adversarial coverage

The combined generative workspace now has 43 passing unit tests. JG2b1b1
fixtures establish that:

- identical parallel branches share normalized identity but receive distinct
  exact branch and producer-finalized identities;
- consuming an empty writer creates a distinct out-of-band finalized-head
  commitment without inventing event zero;
- two adjacent appends derive ordinals and boundaries without caller framing;
- a test-only two-event shadow replay reconstructs fresh sealing and JG2a
  stages from retained scope/candidate evidence, then matches every retained
  stage, sealing, frame, pre/post-head, terminal, count, resource, and
  producer-finalized-head fact without minting a replay token;
- empty candidates, stale predecessor evidence, and malformed scopes fail
  atomically;
- normalization-equivalent raw candidates and irrelevant scope drift affect
  exact evidence but not normalized log-head identity;
- split and coalesced appends are different committed histories;
- depth and retained-extension limits fail without mutation or close;
- protocol/kernel-configuration drift fails at begin; and
- exact field order and domain separation are pinned by a fixed branch-7 codec
  vector:

```text
exact frame:
  blake3:07cca3ec7494d48a8266728fd038ad035c355b7d51c465fae559ad36712391b0
rolling exact append head:
  blake3:d0dd23ffae1db8e04b4957b12d6efc44159a109b2a80a1129a7b61fc012c1897
producer-finalized head:
  blake3:0716930c303c0cd4f6808ebca88081f35688e1b727c2306c9f90a1fa8548b155
```

Any change to the exact frame fields, order, tags, or digest domains invalidates
that fixture.

## Explicit non-theorems

JG2b1b1 does not prove that the closed frame sequence has been independently
replayed in full. It does not mint:

- EOF or globally latest history;
- `VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1`;
- JG2b1a complete-range adjacency or declaration-birth coverage;
- universal substitution/naturality;
- typed generative support or a raw capability carrier;
- quotient, marginal, provenance, `gamma`, bootstrap, or selection authority;
  or
- persistence, crash recovery, signatures, external sole ingress, or
  cross-process anti-equivocation.

The retained frames are crate-private and contain the exact trusted scope and
raw candidate plus all commitment identities needed for fresh replay. No
public transcript accessor or retrospective promotion path is exposed.

## Discharged continuation: JG2b1b2

JG2b1b2 now consumes `ClosedGenerativeSealedLogV1` by value together with an
independently supplied identity-matching kernel. It does not accept a
transcript, frame slice, EOF flag, count, boundary, or finalized-head digest as
a substitute. It:

1. reinitializes the kernel-verified empty boundary;
2. reconstructs a fresh free-sealing token and JG2a stage from each retained
   scope/candidate record in exact ordinal order;
3. rejects every gap, duplicate, reorder, prefix, suffix, branch substitution,
   or trailing frame;
4. compares every retained field and recomputes every normalized and exact
   frame/head commitment and the producer-finalized head;
5. constructs the exact temporary JG2a stage vector and derives JG2b1a afresh;
6. compares the terminal boundary, normalized chain, exact evidence binding,
   and exhaustive declaration-birth index; and
7. only then mints
   `VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1`.

The completed replay is recorded in
`docs/260801_jg2b1b2_complete_through_head_replay.md`. JG2b2a then froze the
substitution/naturality entry protocol; JG2b2b indexed interfaces and generic
metatheory is now active.
