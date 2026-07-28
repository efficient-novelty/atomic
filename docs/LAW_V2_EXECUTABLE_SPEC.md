# Law V2 executable specification

Status: the PR-1 production boundary and a conservative PR-2
certificate/kernel slice are implemented. A finite relative closure/replay
primitive is implemented in `pen-demand` and re-exported by `pen-law`; it is
not integrated into `pen-engine`, and the Phase-3 exit gate remains unmet.
The full cubical, Constitutive-Law, authoritative census, response,
semantic-audit, and halt gates remain open.

This document restates the executable contract in
`docs/app_a_two_laws_formal_axioms.tex` and
`docs/autonomous_genesis_plan.md`. It is a specification, not evidence that
the lawful engine, its certificates, or the Genesis result already exist.
When an implementation cannot construct required evidence, it must fail
closed.

The exact claim boundary of the implemented verifier slice is recorded in
`docs/LAW_V2_CERTIFICATE_BOUNDARY.md`; the finite-relative census boundary is
recorded in `docs/LAW_V2_RELATIVE_CENSUS_BOUNDARY.md`.

## 1. Target and scope

The first software target is B2, trace-blind search over a disclosed fixed
anonymous basis:

- a trusted normalization, type-checking, derivation-checking, and
  equivalence-checking kernel;
- a finite anonymous grammar;
- a stage-generic depth-two demand-scheme calculus;
- the Constitutive and Selective Laws;
- a demand-window width of two;
- free-sealing and equivalence rules; and
- resource budgets that may return `Unknown`, but may not justify a false
  result.

B3, in which the engine also synthesizes fresh primitive schemas, is a later
research phase. The initial lawful claim must not imply B3.

Until bootstrap uniqueness is proved, stages 1 through 3 are a registered
constitutive bootstrap. The honest initial claim is therefore that the
two-law solver governs the guarded cone from the first exported demand
onward. Beginning from a genuinely empty public context is a later theorem
gate.

## 2. Mandatory register discipline

Every numerical value assigned to an extension must carry an explicit
versioned register:

```rust
enum ValueRegister {
    SemanticFamilyV2,
    LegacyStructuralV1,
}

struct RegisteredValue<T> {
    register: ValueRegister,
    value: T,
}
```

Normatively, a `SemanticFamilyV2` value must be act-local, transportable, and
proof-bearing before it can carry law-level authority. The current
`SemanticFamilyValue` implementation establishes only a type-and-tag register
firewall: its public numeric constructor does not certify a semantic audit.
Until a verified audit wrapper exists, such a value is not proof-bearing
evidence and cannot justify acceptance, selection, or halt.

`LegacyStructuralV1` records the enacted engine, bars, overshoot, historical
scores, and quantities derived from them. It is testimony and diagnostics
only. It must not:

- satisfy a demand;
- make a candidate acceptable;
- rank acceptable candidates;
- break a tie;
- justify a halt; or
- be presented as a semantic-family count.

Syntax identifiers, hashes, byte counts, stage labels, and artifact metadata
must be distinguished from extension values. They confer no law-level
authority.

## 3. Proof-relevant state

A lawful history is a sequence of sealed extensions together with only the
state needed to check the laws:

```rust
struct History {
    stages: Vec<SealedExtension>,
    public_boundary: NormalizedSignature,
    derivation_basis: SchemaBasis,
    last_two: Window,
    history_digest: Digest,
}
```

A candidate is more than a bare telescope:

```rust
struct Candidate {
    boundary: NormalizedSignature,
    realization: RealizationTerm,
    induced_obligations: ObligationComplex,
    boundary_inclusion: BoundaryMap,
}
```

Presentation multiplicity is not plurality. Candidate identity is computed
after:

1. normalized-presentation equivalence;
2. natural-family transport;
3. univalent equivalence over the existing boundary; and
4. confluence analysis.

Serialization order and hashes may schedule or identify work, but may not
decide which lawful act occurs.

## 4. Constitutive admissibility

A candidate may enter the acceptable cone only with a recheckable
Constitutive-Law certificate containing:

- witness-bearing typing and realization;
- invariance under the adopted equivalence;
- discharge of every primitive induced obligation within depth two;
- canonicity under sealing; and
- a demand-connected-core proof excluding simultaneous unowed structure.

The verifier, not the generator, is trusted. A missing, malformed,
out-of-scope, or unreplayable component rejects the certificate.

Free sealing happens only after acceptance. It constructs the least
realization of the selected API, quotiented by old judgmental equalities and
equations forced by that API. It adds no other generator or equation, and it
is not a preference rule.

## 5. Demand census

At each prospective guarded act, the live obligation profile is:

```text
O(n + 1) = C(S_n, S_(n - 1)) \ D(B_n)
```

where the only visible memory parameter is the width-two active window.

Each census must independently certify:

- extraction completeness: every normalized demand generated by the active
  window is represented;
- derivability completeness: every extracted family or instance is decided
  as derived or live by typed evidence;
- locality and expiration: every live demand belongs to the active window,
  and older demands remain discharged under weakening; and
- orbit correctness: equivalent instances are collapsed without confusing
  a natural family with its specializations.

Demand motives are registered before fillers exist. Registrations use
dependent contexts, sequential typed substitution, and a total-specialization
theorem. Outcome-dependent narrowing or filtering is forbidden.

The demand generator must be stage-generic. A table indexed by step,
reference telescope, expected family, or accepted trace is not a compliant
scheme calculus.

## 6. Response-cone completeness

Search starts from the required typed outputs of live obligation orbits. A
response-bound certificate must establish a finite graph
`Resp2(history, obligations)` containing a representative of every
constitutively admissible, demand-connected total discharger.

Pruning is lawful only with proof of one of:

- a typing contradiction;
- disconnected support;
- an impossible remaining obligation;
- duplicate normal form; or
- a removable unowed component.

Operational caps may stop the search with `Unknown(ResourceExhausted)`.
Enlarging a cap must not reveal a legal response omitted by a certificate
previously presented as complete.

## 7. Guarded selection

At a guarded stage, a candidate is acceptable exactly when:

1. its Constitutive-Law certificate verifies;
2. typed discharge proofs cover every required output of every live orbit;
   and
3. its semantic-family audit verifies.

No bar, score, efficiency ratio, overshoot, parsimony order, expected future,
hash, or presentation key participates in acceptance.

After the full equivalence quotient:

- zero classes with complete enumeration yields `Blocked` and an unpaid
  demand certificate;
- zero classes with incomplete enumeration yields `Unknown`;
- one class is freely sealed; and
- several genuinely distinct classes create one branch per class, with none
  privileged.

## 8. Semantic-family audit

For each candidate, the audit computes normalized family bases before and
after integration and removes the image of typed weakening:

```text
Marg2(H; x) = Sch2(I(H, x)) \ image(wk_x)
nu_H(x) = |Marg2(H; x)|
```

Each counted family must inject into either:

- an irreducible candidate kernel clause paired with one of
  `KernelHead`, `AdjointMate`, `SupportAction`, or `Coherence`; or
- a required output of a pre-existing live obligation orbit.

String labels, package membership, cardinal slots, archived totals, and
accepted futures are not anchors. The audit must commute with renaming,
substitution, weakening, and adopted equivalence. If these proofs are absent,
the candidate has no lawful semantic value.

## 9. Outcomes and autonomous halt

The lawful lane distinguishes:

```rust
enum RunOutcome {
    Advanced(Cone),
    Halted(HaltCertificate),
    Blocked(UnpaidDemandCertificate),
    Unknown(ResourceExhausted),
}
```

`Blocked`, `Halted`, and `Unknown` are not interchangeable.

A branch may halt only after a prospective census proves its live orbit set
empty. A cone halts only when every branch supplies a compatible
`HaltCertificate`. The certificate must bind:

- history and active-window digests;
- extraction, derivability, and expiration completeness;
- the full decided instance census;
- a live-orbit count of zero;
- execution and exclusion of the demanded-but-underdetermined F1 falsifier;
  and
- the semantic provenance bound.

The observed target is a prospective stage-16 census with all expected
instances decided, not `until_step = 15`, search failure, or inability to
clear a bar. Resource exhaustion can only produce `Unknown`.

## 10. Required evidence

Each stage must emit a proof bundle containing at least:

```text
history
window
extraction certificate
derivability certificate
expiration certificate
live orbits
response-bound certificate
enumeration manifest
candidate classes
Constitutive-Law certificates
discharge certificates
equivalence certificates
semantic-family audit
seal certificate
```

A final run additionally requires the cone, confluence, halt, blindness, and
build-provenance certificates. The blindness manifest must record the kernel,
grammar, scheme-calculus, law, source, and binary digests; the production
dependency graph; absence of oracle dependencies; and absence of
target-length configuration.

## 11. Dependency rule

The law-level lane must have no dependency on:

- target telescopes, labels, hashes, ledgers, or step-count expectations;
- the legacy structural evaluator or bar implementation;
- post-run decoders;
- accepted target artifacts; or
- future-viability routines.

Oracle and diagnostic crates may consume frozen run artifacts. Information
must not flow back into demand extraction, candidate generation,
certification, acceptance, branch creation, or halt.

## 12. Phase gates and non-claims

The implementation phases in `docs/autonomous_genesis_plan.md` are
sequential. A later claim is unavailable until its earlier certificate gates
close. The current production closure contains:

- an independently implemented dependent-core fragment with ordered contexts,
  universes, dependent functions and pairs, units, bounded normalization, and
  judgmental equality;
- canonical, domain-separated certificate bindings;
- replayable derivation, judgmental-equivalence, and exact-extension
  certificates whose successful handles are not deserializable; and
- replay of one particular closed dependent-context specialization, without a
  total-specialization claim;
- a deterministic finite closure and private verified replay handle complete
  only relative to caller-supplied families, instances, rules, seeds, verified
  signature, and opaque two-global window; and
- unchecked storage schemas for the later CL, demand, discharge, semantic
  reference, and halt layers.

This slice is not the full ambient CCHM-style calculus. In particular, it
does not implement path types, finite sums, univalence, Kan composition, or
the universal property of free sealing. Consequently it does not claim:

- that an authoritative or normative complete demand census or response bound
  exists;
- that the current engine accepts by total discharge;
- that the current cone was generated autonomously;
- that global halt at fifteen is executable under Law V2;
- that the bootstrap has been derived; or
- that B3 grammar synthesis exists.

## 13. Recorded blockers

### 13.1 Recovered historical appendix

The normative TeX says the prior bar-law formulation is retained as
`app_a_two_laws_formal_axioms_old.tex`. The exact historical source has now
been recovered byte-for-byte from the sibling `book` repository and is bound
in `configs/legacy_bar_v1.freeze.json` by SHA-256
`75aa7e45edd8bb2e19d53acfa535f1eea3e0fdb88ea8956e64f5010bf6fbc443`.
This closes the missing-source provenance item; the other partial-freeze
requirements remain open.

### 13.2 Stage-4 quotient tension

The normative TeX calls the Stage-4 output four genuinely distinct classes
under the full quotient, but later identifies the
`B4^Pi ~= B4^Sigma` act equivalence as open and says its resolution determines
whether the cone contains four worlds or two. These statements cannot both
serve as a closed executable theorem.

Until a versioned adjudication resolves the tension:

- four Stage-4 candidate presentations may be retained as an unselected
  pre-quotient or provisionally certified cone;
- no production certificate may claim that all four survive the full
  act-equivalence quotient;
- the lawful cone cardinality must be reported as unresolved between the
  documented alternatives; and
- tests must not encode four post-quotient worlds as a proved invariant.

### 13.3 Final-shell realization

A target-shaped temporal telescope is not by itself a realization of the
filtered DCT specification. The decoder may apply the `DCT` label only after a
kernel-checked filtered-model certificate passes. Otherwise the result must be
described as a syntactic temporal-cohesive shell with an open realization
obligation.
