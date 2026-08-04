# Phase JG2b1a — Contiguous Generative Stage Chain

**Status:** DISCHARGED on 2026-08-01  
**Authority:** `VerifiedContiguousGenerativeStageChainV1`  
**Implementation:** `crates/pen-generative-audit`  
**Next phase:** JG2b2b — indexed interfaces and generic substitution metatheory

## 1. Jurisdiction

JG2b1a derives a canonical normalized history chain and declaration-birth
index from an ordered slice of opaque JG2a stage surfaces. It begins from the
internally kernel-verified empty signature, independently replays each
normalized extension, and admits no caller event identity, ordinal, expected
count, initial or terminal boundary, active window, or registered-history
token.

The authority is deliberately relative:

> Every declaration in the terminal public boundary has exactly one derived
> birth in the supplied contiguous normalized chain, and every derived birth
> appears in that terminal boundary.

It does not prove that the supplied chain is the complete externally intended
history or that its event segmentation is uniquely privileged.

## 2. Why the scope is relative

Opaque stage tokens can prove exact typed transitions and adjacency. They
cannot, by themselves, prove that the caller supplied the final event, that a
multi-declaration extension was not split into several events, or that several
events were not coalesced into one valid extension. Independent declarations
may also admit more than one valid ordering when all affected stage tokens are
rebuilt consistently.

Rejecting every multi-declaration extension would silently impose a one-
clause-per-act ontology that is absent from the two laws. Calling a contiguous
slice “the complete history” would instead turn caller segmentation into
authority. JG2b1a does neither.

Consequently:

- an empty slice validly certifies the empty-to-empty relative chain;
- a proper valid prefix validly certifies a shorter relative chain;
- split and coalesced presentations may both verify but receive different
  history-relative event chains; and
- this JG2b1a token alone remains relative; complete-through-head promotion
  requires the now-discharged JG2b1b authority.

## 3. Derived transition and event identities

Each stage yields two distinct identities:

1. a normalized transition identity derived from the frozen JG1/JG2a grammar,
   kernel protocol and configuration, verified predecessor, replay-derived
   normalized extension, and verified successor; and
2. a history-relative event identity derived from the preceding normalized
   history digest, the verifier-derived ordinal, and the transition identity.

This distinction prevents the same later transition from aliasing across
different split or coalesced prefixes.

Canonical normalized identities exclude the JG2a stage-manifest digest, raw
candidate digest, sealing subject, and unrelated `TrustedScope` law, history,
window, bootstrap, or blindness fields. Those fields can differ while proving
the same normalized public transition. They remain stage-level evidence, but
they do not mint distinct normalized births.

The authority retains exact evidence bindings separately from the normalized
chain identity. Exact token equality is evidence-sensitive; normalized
comparison uses the normalized transition/event/chain identities instead. A
digest identifies a subject; it is never used as a substitute for the opaque
stage or chain proof.

## 4. Replay and adjacency rules

For each supplied stage, in order, the verifier:

1. checks the exact JG1, constructor grammar, scope-grammar, kernel,
   normalizer, and kernel-configuration bindings;
2. requires the first predecessor to equal the internally verified empty
   boundary and every later predecessor to equal the preceding replayed
   successor, by digest and normalized declarations;
3. rejects an empty normalized extension;
4. replays the normalized extension with `Kernel::verify_extension`;
5. requires exact declaration-count arithmetic and complete successor
   equality;
6. derives the transition, ordinal, event identity, and pre/post normalized
   history digests;
7. inserts every newly introduced `GlobalId` into the declaration-birth map
   without overwrite; and
8. finally proves exact equality between the birth-map keys and terminal
   boundary identifiers.

An omitted interior stage, reordering, duplication, or branch splice using
the original stage tokens therefore fails adjacency. Unsupported replay or
resource exhaustion aborts the entire mint; it never yields a shorter
authoritative chain.

## 5. Resource contract

Resource policy V1 is explicitly relative to the bound kernel configuration.
Sequential stage count is bounded by the supplied kernel's `max_depth`, whose
hard ceiling is 256. Terminal and retained extension material are bounded by
`max_operations`; inspected predecessor/extension/successor material is
bounded by the checked formula
`max_operations * (2 * max_depth + 1)`. Counts use checked arithmetic and
output vectors use fallible reservation. The resource-policy version and
kernel-configuration digest enter canonical chain identity. Exceeding a bound
is an explicit resource failure, not proof that later events or declarations
are absent.

The chain output retains normalized transition material and the terminal
verified boundary rather than cloning every growing boundary snapshot. Any
birth boundary needed later must be reconstructed through checked prefix
replay.

## 6. Explicit non-authorities

JG2b1a issues no authority for:

- complete external history, an EOF, terminality, or unique event
  segmentation;
- an actual live branch, registered bootstrap, active window, or Law-V2A
  founding prefix;
- occurrence-to-constructor classification;
- a generic arbitrary-typed-substitution theorem, complete occurrence census,
  or universal naturality;
- predecessor capability closure, newly paid support, or prior live
  generative-output support;
- a raw capability member or complete `RawGCap` carrier;
- quotienting, weakening, marginality, strict enlargement, or provenance;
- `gamma`, bootstrap values, debt, selection, or a live Genesis result; or
- Rust/safe-Agda correspondence.

The implementation depends only on `pen-kernel`; it does not import
`pen-law::UncheckedHistory`, the registered `pen-law::VerifiedHistory`, the
semantic audit, or the oracle lane.

## 7. JG2b1b result

JG2b1b is now discharged through the opaque
`VerifiedCompleteTargetNeutralGenerativeHistoryThroughHeadV1`. JG2b1b0 froze
the prospective event/finalization protocol, JG2b1b1 implemented its owned
process-local producer, and JG2b1b2 consumes the resulting closed state and
independently reconstructs every JG2a stage plus this exact JG2b1a chain and
birth index.

The theorem is complete only through that producer-designated finalized head.
There is no alternative authoritative EOF route. A caller-supplied transcript,
EOF Boolean, event count, terminal digest, branch selector, or segmentation
vector remains data rather than authority. JG2b2a has since frozen the rule
that a finite history census cannot replace a generic naturality theorem.
JG2b2b must now provide the indexed-interface calculus and theorem package;
JG2b2c may then consume the complete-through-head history instead of a supplied
relative chain. Closure, typed support, and raw-carrier authority remain
blocked behind JG2b2 and JG2b3.
