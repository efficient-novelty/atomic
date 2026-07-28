# Blindness contract

Status: the dependency/source firewall, conservative verifier slice,
structural GF2 contract, exact checker-readiness probe, and anonymous
registered-bootstrap replay are implemented for the Law V2 closure. The
relative census and native intrinsic scheme slice remain unintegrated with
`pen-engine`, and the Phase-3 exit gate is unmet. Later theorem and
perturbation controls remain open.

This contract defines what information the Law V2 production lane may
observe. It applies to source dependencies, generated code, build scripts,
configuration, runtime inputs, caches, checkpoints, search heuristics, and
certificate verification.

## 1. Blindness levels

### B1: name-blind

Syntax is anonymous, but stage-specific policies, target-shaped families,
expected costs, or a configured endpoint may remain. B1 is useful legacy
testimony but is not the Law V2 target.

### B2: trace-blind, fixed basis

The engine receives a fixed anonymous grammar and stage-generic scheme
calculus, but no target trace, semantic labels, expected ledger, endpoint,
stage-indexed family policy, bar-selected winner, or expected-future rule.
B2 is the first executable target.

### B3: basis-generative

The engine synthesizes fresh primitive schemas when typed demand cannot be
discharged from the existing grammar. B3 is a later phase and must not be
implied by a B2 result.

Every future run claiming B2 or B3 authority, and every certificate supporting
that run claim, must explicitly bind its blindness level and manifest. The
current kernel/law bindings carry a blindness-contract digest rather than an
explicit level, while the relative-census certificate and engine startup
configuration carry neither. Those current artifacts therefore cannot by
themselves support a B2/B3 run claim; explicit level/manifest binding remains
an open schema gate.

## 2. Permitted production inputs

A B2 lawful run may observe only:

1. a trusted kernel and its versioned rules;
2. a finite anonymous grammar of typed constructors and equations;
3. a stage-generic depth-two demand-scheme calculus;
4. the Constitutive and Selective Laws;
5. window width two;
6. free-sealing and equivalence rules;
7. a versioned finite Genesis Fragment of depth two (GF2) delimiting every
   executable completeness claim; and
8. resource budgets.

The run manifest must bind these inputs by digest. Changing any one creates a
different run.

## 3. Forbidden production inputs

The lawful lane must not observe, directly or indirectly:

- Genesis names or equivalent semantic labels, including `circle`, `Hopf`,
  `cohesion`, `curvature`, `Hilbert`, and `DCT`;
- the target length fifteen or any configured equivalent;
- `Telescope::reference`, `all_reference_telescopes`, or copied target
  telescopes;
- expected candidate or canonical hashes;
- expected semantic or structural `(nu, kappa)` vectors;
- target checkpoints, frozen accepted histories, or survivor lists;
- named progression predicates such as
  `requires_temporal_shell_package`;
- branches on a semantic stage index;
- bar clearance, overshoot, `rho`, efficiency, or semantic novelty as
  admissibility, continuation, selection, or halt authority;
- deterministic hash or presentation-order tie-breaking;
- next-step viability against the expected future;
- baseline parity as acceptance evidence; or
- heuristics trained, tuned, or amended using the accepted target trace.

Renaming a forbidden input, embedding it in generated code, reading it through
a cache, or reconstructing it from a fixture remains a violation.

## 4. Information-flow boundary

The intended dependency direction is:

```text
lawful run artifacts ---> diagnostics
lawful run artifacts ---> oracle decoder

pen-oracle -X-> kernel/demand/synthesis/audit/law/engine
diagnostics -X-> kernel/demand/synthesis/audit/law/engine
```

`pen-oracle` owns target telescopes, human labels, expected hashes, expected
ledgers, and expected step counts. It may be used by tests, development
oracles, and post-run decoders. It is never a production dependency of the
lawful binary or any crate on the lawful binary's transitive dependency path.

`pen-gf2-agda` may inspect only its fixed postulate-free smoke source, the
explicit checker paths and independently trusted executable-digest pins
supplied for a readiness probe, the pinned Cubical checkout, and the
Agda-2.8.0 primitive runtime directory reported by the pinned executable.
Text members of both imported trees must be UTF-8; CRLF is canonicalized to LF
and any remaining bare carriage return is rejected. Primitive `.agdai` members
are retained as raw bytes. These canonical members are copied into a private
scratch snapshot, and only that snapshot is exposed to the smoke checker. The
Cubical manifest must match the repository-reviewed canonical tree digest. The
Agda primitive-runtime manifest must match an independently reviewed digest
supplied by trusted configuration for the exact executable distribution. The
local reference primitive digest and a self-derived executable digest are
reproducibility evidence, not authenticity anchors. The crate exposes no
arbitrary-source checker API and receives no history, target candidate, label,
cardinality, score, or future event. Its readiness capability is not theorem
evidence.

Diagnostics may inspect already sealed artifacts. They may not return a
selector, pruning hint, demand, certificate premise, or halt decision to the
lawful lane.

## 5. Scheduling versus legislation

Deterministic ordering is permitted for:

- work queues;
- reproducible serialization;
- cache keys;
- deduplication after equality is established; and
- stable diagnostics.

It is forbidden for:

- selecting one of several acceptable equivalence classes;
- declaring uniqueness;
- choosing a representative before the required quotient;
- deciding whether a candidate totally discharges an obligation; or
- turning incomplete enumeration into `Blocked` or `Halted`.

Scheduling metadata must not appear among the premises of a law-level
certificate.

## 6. Stage identifiers

A stage number may appear in logs and artifact paths after an event occurs.
It must not be an input to demand generation, admissibility, response
synthesis, semantic-family auditing, or acceptance.

Compliance requires replacing event identifiers with random opaque values and
obtaining the same result up to identifier renaming.

Law V2A's registered three-act bootstrap is a disclosed initial condition, not
a hidden stage-index policy. Its run manifest must say that the three acts
were supplied and must not claim empty-context derivation or uniqueness.
Law V2B is a distinct future claim requiring a least-arena derivation and
uniqueness proof from the empty public context.

The current embedded registration contains only three anonymous
kernel-normalized one-declaration acts and their provenance chain. Its strict
wire schema rejects downstream annotations. Replaying it proves registration
integrity and typing, not autonomous discovery.

## 7. Semantic names and decoding

The production run operates on opaque operator and act identifiers. Human
names are attached only after:

1. the run and all certificates are frozen;
2. the blindness manifest verifies;
3. the decoder is invoked out of process or outside the lawful dependency
   closure; and
4. any semantic realization required by the label has independently passed.

In particular, a temporal shell must not be called `DCT` merely because its
syntax resembles the target.

## 8. Resource semantics

Resource budgets are allowed to affect completion time and the result
`Unknown(ResourceExhausted)`. An input outside GF2 yields
`Unknown(OutsideFragment)`. Neither result may affect or be restated as the
mathematical cone.

Across worker counts, schedules, memory limits, and operational caps, a run
must yield either:

- the same certified result up to adopted equivalence; or
- `Unknown(ResourceExhausted)` or `Unknown(OutsideFragment)`.

A different winner, a smaller cone, `Blocked`, or `Halted` under tighter
resources is a blindness/completeness failure.

## 9. Equivariance

For every admissible renaming or presentation automorphism `pi`:

```text
BlindGenesis(pi(G)) ~= pi(BlindGenesis(G))
```

Required perturbations include:

- anonymous operator-ID permutation with the inverse map withheld;
- grammar declaration reordering;
- clause-catalog reordering;
- worker and enumeration-order changes;
- canonical hash replacement preserving equality;
- conservative syntax aliases; and
- alternative supported presentations.

An invariant hash alone is not proof of equivariance. The result must include
typed transport or equivalence evidence.

## 10. Blindness manifest

Every claimed B2 run must record:

```text
blindness level
source commit and dirty-state declaration
binary digest
production dependency graph
kernel digest
grammar digest
scheme-calculus digest
GF2 fragment digest
law digest
window width
bootstrap contract
resource budgets
absence of oracle dependencies
absence of target-length configuration
absence of diagnostics-to-law dependencies
perturbation-suite results
```

A missing field fails closed.

## 11. Required enforcement

The oracle-firewall phase is not complete until automated checks prove:

- the lawful binary's transitive dependency graph excludes `pen-oracle`;
- production sources do not include target fixtures or target-bearing
  generated files;
- deleting the oracle crate and all reference-telescope files still permits
  the lawful binary to build and execute its non-oracle tests;
- target labels do not occur in the lawful dependency closure;
- no lawful configuration contains a semantic target length or selector; and
- post-run decoder tests remain separate from production acceptance tests.

Text searches are useful defense in depth but are not sufficient by
themselves. The dependency and delete-oracle tests are authoritative.

The delete-oracle gate is evaluated against the isolated Law V2 production
closure, not the historical workspace. The legacy crates retain source-bound
reference fixtures so their frozen certificates remain replayable; the
quarantined `pen-oracle` copy makes those fixtures explicit for tests and
post-run decoding. Physically relocating the legacy definitions requires a
versioned testimony migration because existing certificates bind the exact
`pen-core` source bytes. That compatibility migration is not part of PR 1 and
must not be described as complete.

The historical `docs/app_a_two_laws_formal_axioms_old.tex` has been recovered
byte-for-byte and digest-bound in the partial legacy freeze manifest. Its
recovery closes that provenance item without widening the Law V2 production
closure.

## 12. Known blockers

- Four normalized Stage-4 representative slots are required before
  quotienting, but the repository has not yet certified those representatives
  under Law V2. Even after certification, the full quotient has only the
  conditional bound `2 <= count <= 4` until the Pi/Sigma former-axis
  equivalence is constructed or obstructed; an undecidable quotient returns
  `Unknown(UnknownQuotient)`.
- The current repository has not implemented GF2 or its closure/completeness
  verifier. An outside-fragment input must remain `Unknown`; lack of a second
  result is not uniqueness evidence.
- Complete physical relocation of legacy target fixtures is coupled to
  versioning the historical source-bound certificates; the current PR-1
  guarantee is the independently built, oracle-free Law V2 closure.

## 13. Violation policy

Discovery of a forbidden information flow invalidates the affected lawful
artifact. The remedy is:

1. record the violation and affected artifacts;
2. remove the flow through a versioned change;
3. rebuild from clean inputs;
4. rerun the blindness and certificate suites; and
5. issue new artifacts rather than silently amending old ones.
