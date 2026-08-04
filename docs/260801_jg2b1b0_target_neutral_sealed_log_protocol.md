# JG2b1b0 Target-Neutral Sealed-Log Protocol

Date: 2026-08-01

Status: JG2b1b0, JG2b1b1, JG2b1b2, and protocol-only JG2b2a are discharged.
JG2b1b is complete; full JG2 remains open at active JG2b2b.

## Result

`crates/pen-sealed-history` freezes the target-neutral lifecycle and authority
rules that any JG2b1b producer must obey. Its sole opaque token is
`VerifiedTargetNeutralSealedLogProtocolV1`.

The token is a protocol-grammar authority only. It contains no event, writer,
branch head, transcript, finalization receipt, EOF fact, or verified history,
and it cannot establish indexed-interface or naturality authority.

## Why a retrospective completeness verifier is impossible

Let a proposed verifier receive a finite transcript `T` and any metadata that
the caller can recompute from `T`: an EOF Boolean, event count, terminal
digest, hash-chain root, or terminal boundary. The verifier receives exactly
the same inputs in both of these worlds:

1. the producer stopped after `T`; and
2. the producer continued with an event `e`, so the actual log is `T . e`.

No pure function of the supplied inputs can distinguish those worlds. Wrapping
the recomputed fields in an opaque Rust type after the fact does not create the
missing designation authority. This is the extension-indistinguishability
obstruction.

The existing repository authorities do not supply that missing fact:

- `pen-kernel::VerifiedFreeSealing` proves one exact extension and explicitly
  does not authorize history mutation;
- a `TrustedScope` copies its caller-provided `history_digest` as binding data;
- `pen-law::UncheckedHistory` is unchecked wire data;
- `pen-law::VerifiedHistory` is fixed to the registered three-act Law-V2A
  founding prefix; and
- JG2b1a deliberately certifies adjacency and births only relative to its
  supplied slice.

Therefore JG2b1b must be prospective: an owned producer must authoritatively
frame every event and later consume its writer state to designate a finalized
head.

## Correct completeness semantics

The generic theorem target is

```text
CompleteThroughHead(H, h)
```

where `h` is an opaque producer-designated finalized head for exactly the
finite history `H`. It is not "globally latest history" and is not a claim that
no other branch or later epoch exists.

This agrees with the formal theory: `H_n` is defined as a finite sequence of
sealed extensions. A truncation is a different finite history, not a false
representation of the same history. Binding a chosen `H` to an authoritative
head prevents caller-selected omission within that producer epoch; it does not
turn the mathematical history parameter into metaphysical global EOF.

Literal current-world or persistent-store finality is a stronger integration
claim. It additionally needs exclusive external ingress, atomic persistence,
locking or signatures, crash recovery, and anti-equivocation. That claim is
outside JG2b1b0 and the in-memory target-neutral theorem.

## Frozen V1 rules

The V1 manifest closes ten pairwise-tagged rules:

1. require a kernel-verified empty boundary as genesis;
2. derive exactly one event delimiter from each successful append commit;
3. maintain a strict single-parent commitment chain;
4. make one private non-cloneable writer the sole append ingress;
5. mint a distinct producer identity for every opened, continued, or forked
   writer epoch;
6. forbid retrospective promotion of caller transcripts or recomputed roots;
7. bind event/material/replay limits to the exact kernel profile, use checked
   arithmetic and fallible retained-output reservation, and never interpret a
   reported resource failure or allocator abort as finalization or EOF;
8. finalize only by consuming the corresponding open writer;
9. mint the finalized head only from that consumed owned state; and
10. certify completeness through the finalized head only after independent
    full replay and terminal-boundary comparison.

The canonical 15-byte rules transcript is:

```text
c1 01 00 01 00 10 11 12 13 14 15 16 17 18 19
```

Its frozen domain-separated digest is:

```text
blake3:b1ab522d4c388abb26b8ff0384344bfd350904d32a78da44d2b410a3360f44bf
```

The verified protocol token additionally binds the kernel protocol,
normalizer protocol, and the complete kernel resource configuration.

## Dependency and authority boundary

`pen-sealed-history` is a standalone workspace whose only production
dependency is `pen-kernel`. It has no dependency on `pen-law`, `pen-store`,
`pen-engine`, semantic audit, a registered profile, diagnostics, or an oracle.
Its lockfile is seeded from the reviewed repository lock so the binary's actual
kernel transitive closure matches the graph committed by the kernel protocol
digest.

The token is cloneable because it describes an immutable, freely remintable
grammar. Conforming open-writer, branch-head, and consuming-finalization
authority must not be cloneable or deserializable merely because this grammar
token is.

## Ordered continuation

JG2b1b was discharged through two implementation gates:

1. **JG2b1b1 - prospective linear producer - DISCHARGED.** A same-process,
   non-cloneable writer internally reconstructs one JG2a stage per successful
   append, derives its ordinal and producer framing, and atomically advances
   separate normalized log-head and exact-evidence commitments. Infallible
   consuming close mints a distinct producer-finalized head, but not a
   complete-history token.
2. **JG2b1b2 - independent snapshot completeness - DISCHARGED.** The consuming
   verifier accepts the closed owned state by value with an independently
   supplied identity-matching kernel, freshly reconstructs every JG2a stage
   and the exact JG2b1a chain, verifies the complete ordinal range, retained
   fields, heads, resource totals, terminal boundary, birth coverage, and
   producer-finalized commitment, then mints
   `VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1`.

JG2b2a has since frozen the substitution/naturality entry protocol, and JG2b2b
is now active. A later JG2b1b3 is required only if a
live or persistent external branch must be certified as the actual current
branch rather than as the finite branch owned by the in-memory producer.
