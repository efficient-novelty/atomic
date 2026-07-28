# Law V2 certificate boundary

Status: implemented Phase-2 trusted fragment; not the Phase-2 exit gate.

This document records exactly what `crates/pen-kernel` can check. It is a
claim boundary, not a roadmap completion certificate. In particular, the
kernel can replay typing, normalization, judgmental equality, and an exact
signature-extension fragment. It cannot establish the ambient cubical,
semantic, demand-completeness, or halting obligations required for a Law V2
acceptance decision.

The implementation is intentionally independent of the historical target
trace and semantic decoder. A successful handle from this crate means only
that the corresponding judgment passed the checks described below.

## 1. Implemented dependent core

The term language is anonymous. Local variables use zero-based de Bruijn
indices, with index zero denoting the innermost available binder. Globals are
identified by validated BLAKE3 digests; a global digest is an identifier, not
evidence.

The supported forms are exactly:

- universe sorts `Sort(level)`;
- local `Var(index)` and anonymous `Global(id)` references;
- dependent `Pi(parameter, body)` and `Sigma(parameter, body)` types;
- annotated `Lambda(parameter_type, body)`;
- `Apply(function, argument)`;
- annotated dependent `Pair(sigma_type, first, second)`;
- `First(pair)` and `Second(pair)` projections;
- `UnitType`; and
- `Unit`.

There are no implicit binders or names in the trusted syntax. Lambda and pair
annotations make every supported introduction form inferable by the small
checker.

The implemented typing behavior is:

- `Sort(level)` has type `Sort(level + 1)`, failing on level overflow;
- `Pi` and `Sigma` check both their parameter and dependent body as types and
  inhabit the maximum of the two inferred universe levels;
- an annotated lambda checks its parameter type, infers its body under one
  additional binder, and infers a dependent `Pi` type;
- application requires a normalized `Pi` type and substitutes the argument
  into its body type;
- an annotated pair requires a normalized `Sigma` type, checks its first
  component against the parameter type, and its second component against the
  substituted body type;
- projections require a normalized `Sigma` type;
- `UnitType` inhabits `Sort(0)` and `Unit` inhabits `UnitType`; and
- a term checks against an expected type only when the independently inferred
  and expected types have identical normal forms.

Substitution and shifting traverse every supported form and account for
crossed binders. Beta reduction therefore uses capture-avoiding de Bruijn
substitution rather than textual replacement.

## 2. Contexts, signatures, and judgments

### 2.1 Ordered dependent contexts

`DependentContext` is supplied as an ordered vector of types. Verification
proceeds strictly left to right. Entry `i` is checked as a type using only
entries before `i`, then normalized before it becomes available to later
entries. A forward or out-of-scope reference fails with `UnboundVariable`.

Successful verification returns a `VerifiedContext` whose fields are private.
It exposes its normalized entries, digest, and a normalized wire copy, but it
cannot be created by deserializing an assertion that a context is valid.

### 2.2 Closed normalized signatures

`UncheckedSignature` is a wire DTO containing ordered declarations. Each
declaration contains an anonymous global identifier, a type, and an optional
body.

Signature verification:

1. rejects duplicate global identifiers;
2. checks each declaration type as a closed type;
3. permits it to refer only to already verified globals;
4. normalizes the type;
5. when a body is present, checks it against that normalized type and
   normalizes it; and
6. computes a digest over the resulting normalized signature.

Because declarations are checked against the verified prefix, self-reference
and forward global reference are unavailable. Because the local context is
empty, free local variables are unavailable.

Successful verification returns a `VerifiedSignature` with private fields.
The returned normalized wire representation is data; it must be reverified
after deserialization before it regains trusted status.

### 2.3 Open judgments

The public judgment DTO supports exactly:

- `TypeFormation { context, term }`;
- `HasType { context, term, ty }`; and
- `DefinitionallyEqual { context, left, right, ty }`.

The kernel first verifies and normalizes the supplied context. It then
rechecks the judgment under a verified signature and returns the corresponding
normalized judgment. Definitional equality is typed equality of normal forms;
it is not presentation, categorical, cubical, or univalent equivalence.

## 3. Normalization and resource boundary

Normalization is deterministic over the implemented grammar. It performs:

- unfolding of verified global bodies;
- beta reduction of an application whose normalized head is an annotated
  lambda;
- first and second projection from an annotated pair; and
- recursive normalization of annotations and subterms.

No eta law, path computation, user rewrite system, or univalence principle is
present.

Every public kernel verification call uses fresh explicit limits:

- maximum kernel operations;
- maximum term-recursion depth; and
- normalization rewrite fuel.

All limits must be positive. Exhaustion returns
`KernelError::ResourceExhausted` with the exhausted resource kind. It does not
prove ill-typedness, inequivalence, absence of a response, blockage, or halt.
An orchestrator may translate exhaustion only to `Unknown(ResourceExhausted)`.
It must not turn a tighter budget into a negative mathematical conclusion.

Other malformed or unsupported inputs fail closed with a typed kernel or
certificate error. No partial `Verified*` capability is returned.

Certificate entry points preflight every untrusted judgment or signature
before recursive equality and canonical hashing, and global-table scans are
charged to the operation budget. Transport parsing and the allocation of an
already-deserialized DTO remain caller responsibilities; an artifact loader
must impose its own byte limit before invoking the kernel.

## 4. Canonical identities and scope binding

Proof-subject identities do not use JSON serialization. The kernel owns a
small canonical encoder with:

- fixed constructor tags;
- fixed-width little-endian integers;
- length-delimited byte strings, text, and sequences; and
- explicit option tags.

Canonical payloads are hashed with BLAKE3 under a length-delimited protocol
domain. Digest text must have the exact `blake3:` prefix followed by 64
lowercase hexadecimal characters. Domain separation ensures that equal bytes
used for different protocol subjects do not receive the same identity.

This hash discipline identifies a subject; it never proves the subject.
Certificate replay always rechecks the embedded witness.

Every certificate carries a versioned `CertificateBinding` covering:

- certificate schema and canonical-codec versions;
- claim kind;
- kernel and normalizer;
- law, grammar, and scheme calculus;
- blindness and bootstrap contracts;
- history;
- public boundary;
- derivation basis;
- active window; and
- candidate.

The verifier compares that deserialized binding field for field with a
locally constructed `TrustedScope`. It also requires the trusted public
boundary digest to equal the digest of the `VerifiedSignature` actually used
for replay. The kernel and normalizer fields are taken directly from
reproducible, domain-separated digests over the byte sequences explicitly
enumerated by `kernel_protocol_digest` and `normalizer_protocol_digest`.
The kernel digest covers `src/lib.rs`, `src/syntax.rs`, `src/checker.rs`,
`src/certificate.rs`, `src/digest.rs`, the crate and workspace
`Cargo.toml` files, workspace `Cargo.lock`, `rust-toolchain.toml`, and
`.cargo/config.toml`. The normalizer digest covers `src/lib.rs`,
`src/syntax.rs`, and `src/checker.rs` plus those same five
build/configuration files. This is exact source/resolved-input identity for
those enumerated bytes, not binary, compiler, operating-system, dependency
source, or environment attestation; callers cannot supply alternate values
for those two fields.
The law, grammar, scheme, history, and contract identities remain explicit
trusted caller inputs. A certificate cannot choose its own trusted scope
merely by embedding internally consistent hashes.

Judgment subject digests bind the verified signature digest and complete open
judgment. Sealing subject digests bind the verified base signature and exact
requested extension.

## 5. Implemented unchecked certificate DTOs

The wire certificate types are deliberately named `Unchecked*`. Deserializing
one produces data, not a proof capability. Unknown object fields are rejected.

### 5.1 Derivation fragment

`UncheckedDerivationCertificate` carries its complete binding, subject digest,
claimed open judgment, and claimed normalized judgment. Verification:

1. matches the trusted scope and `Derivation` claim kind;
2. matches a caller-supplied expected judgment;
3. recomputes the subject digest;
4. reruns context and kernel judgment verification; and
5. requires the claimed normalized judgment to equal the replayed result.

This is direct deterministic replay of the supported kernel judgment. It is
not yet a general serialized derivation-rule DAG for the ambient calculus.

### 5.2 Definitional-equivalence fragment

`UncheckedEquivalenceCertificate` has the same replay discipline but accepts
only a `DefinitionallyEqual` judgment and the
`DefinitionalEquivalence` claim kind.

Its successful handle is named `VerifiedDefinitionalEquivalence` to prevent
it from being cited as univalent or presentation equivalence.

### 5.3 Exact free-sealing fragment

`UncheckedFreeSealingCertificate` carries an exact requested signature
extension and a claimed normalized sealed signature. The verifier:

1. binds the claim to the trusted base boundary;
2. requires exact equality with the caller-supplied extension;
3. recomputes the base-plus-extension subject digest;
4. appends and rechecks all declarations; and
5. requires the claimed sealed signature to be exactly the normalized result.

This rejects an extra declaration in the claimed seal. It establishes only
an exact, well-typed syntactic extension of the small kernel signature.
`VerifiedFreeSealing` does not mutate or authorize mutation of a history.

### 5.4 Closed-specialization fragment

`UncheckedClosedSpecializationCertificate` binds an open judgment, an exact
oldest-to-newest assignment for its entire dependent context, and the claimed
closed normal judgment. The kernel checks each assignment sequentially
against the motive specialized by earlier assignments, performs its own
capture-avoiding substitution, and rechecks the resulting closed judgment.
The private result is `VerifiedClosedSpecialization`.

This proves one particular closed specialization. It does not prove the
total-specialization theorem required for a registered scheme, and it does
not yet support specialization into a nonempty support context.

## 6. Private verified capabilities

The successful capability types are:

- `VerifiedSignature`;
- `VerifiedContext`;
- `VerifiedDerivation`;
- `VerifiedDefinitionalEquivalence`;
- `VerifiedClosedSpecialization`; and
- `VerifiedFreeSealing`.

Their fields and constructors are private, and the types are not
deserializable. Public accessors expose normalized subjects and digests for
downstream checking, not a way to forge the capability. The derivation,
definitional-equivalence, and sealing handles are distinct types, so one
certificate class cannot be substituted for another.

These handles are local verification results. They are not persisted
law-level acceptance tokens and do not by themselves authorize candidate
selection, integration, branching, blockage, or halt.

`pen-law` additionally defines canonical, domain-separated subject
projections for raw histories and candidates. Their outer bindings are
excluded from their own subject hashes, avoiding self-reference. These
unchecked digest constructors have no internal resource budget; an artifact
loader must bound bytes, nodes, and recursion before invoking them, and their
output confers no authority. Each nested seal in a history records the
predecessor-history digest; a future history verifier must enforce that chain
and must reject a seal bound to the containing history. The supplied helper
defines the intended canonical event ID from that predecessor digest and the
exact sealing subject, but `EventId` remains freely constructible and no
history verifier currently recomputes it.
The remaining CL, authoritative law-level census, discharge, and halt DTOs
still have no canonical verifier projection or verified handle. The separate
`pen-demand::VerifiedRelativeCensus` handle proves only replay against its
explicit caller-supplied finite domain.

## 7. Adversarial coverage

The crate's tests currently exercise:

- canonical digest parsing and protocol-domain separation;
- rejection of unknown certificate fields;
- left-to-right dependent-context checking and forward-reference rejection;
- typed beta replay through capture-avoiding substitution;
- rejection of application at a non-function type;
- ordered, nonrecursive global declarations and duplicate-ID rejection;
- explicit resource exhaustion;
- replay of the witness rather than trust in a cached normal form;
- rejection of cross-history certificate replay;
- rejection of certificate-claim substitution; and
- rejection of an extra declaration in a claimed sealed signature.

These regression tests protect the implemented boundary. They are not
metatheoretic proofs of normalization, completeness, or consistency.

## 8. Explicitly unsupported claims

The following are outside this trusted fragment and must fail closed or remain
`Unknown`:

- the ambient CCHM-style cubical calculus;
- interval, face, path, composition, filling, Kan, Glue, or higher-inductive
  operations;
- univalence and transport between presentation-equivalent public
  signatures;
- finite sums or the full anonymous grammar expected by the final theory;
- natural-family equivalence and orbit quotienting;
- the relative-initial or universal property of free sealing;
- proof that an extension contains all and only law-forced equations;
- history integration and derivation-basis update;
- total specialization of registered future-hole schemes;
- any full `CLCertificate`, including constructive realization,
  equivalence invariance, depth-two local satisfiability, sealing canonicity,
  and demand-connectedness;
- semantic-family auditing and blindness/equivariance proofs;
- demand extraction, depth-two census, family normalization, derivability
  saturation, orbit reduction, completeness, or expiration;
- discharge certificates and finite response-cone completeness;
- candidate acceptance, uniqueness, confluence, or branch cardinality;
- `Blocked`, `Advanced`, or `Halted` engine outcomes; and
- a halt certificate, final empty-demand theorem, or target reconstruction.

Accordingly, this implementation is a strict subset of Phase 2 in
`docs/autonomous_genesis_plan.md`. It does not satisfy that phase's complete
exit gate, because full CL and ambient equivalence verification are absent.

## 9. Open source and theorem tensions

### 9.1 Recovered legacy appendix

The normative appendix says that it preserves the previous law statement in
`docs/app_a_two_laws_formal_axioms_old.tex`. That exact source is present and
is independently digest-bound by the partial legacy freeze manifest and its
replay checker. This provenance repair does not expand the kernel's trusted
certificate fragment or satisfy any theorem gate.

### 9.2 Stage-4 quotient cardinality

The normative appendix describes four genuinely distinct Stage-4 classes
under the full quotient, but later leaves the relevant Pi/Sigma act
equivalence open and says its resolution determines whether the cone has four
worlds or two. Until a versioned adjudication supplies the missing typed
equivalence or obstruction, neither post-quotient cardinality is an
executable theorem.

The kernel must therefore not contain an expected cone cardinality, and its
definitional-equivalence handle must not be used as evidence that the open
univalent equivalence exists or fails to exist.

## 10. Implemented relative census and remaining Phase-3 boundary

`pen-demand` now provides a deterministic finite closure/replay primitive. It
distinguishes caller-supplied families from caller-supplied instances,
derives structural support against an opaque two-global window, saturates an
explicit finite rule relation from explicit library seeds, and returns either
`CompleteRelative` or `Unknown`. Its certificate verifier recomputes that
calculation and returns a private `VerifiedRelativeCensus` handle.

This is not the Phase-3 exit gate. Family membership, instance membership,
the rule relation, and library seeds are relative inputs rather than
generation, specialization, naturality, or derivability-completeness
theorems. The certificate binds the verified signature and finite census
inputs, but it does not yet bind a verified history, derivation basis, law,
grammar, scheme calculus, or event-based active window, and it provides no
weakening or expiration proof.

The remaining authoritative layer must:

1. generate and prove exhaustive the depth-two open scheme domain over
   `DependentContext` and `OpenJudgment`;
2. prove total specialization and distinguish normalized natural families
   from their concrete instances;
3. bind extraction to a verified history, public signature, derivation basis,
   law, grammar, scheme calculus, and the two most recent sealed events;
4. implement the certified family normalizer and the judgmental, naturality,
   univalent, and family-instance orbit quotient;
5. independently certify extraction completeness, derivability completeness,
   and locality/weakening/expiration; and
6. return `Unknown` whenever any domain, proof replay, or required ambient
   equivalence is incomplete.

Frozen prefixes may be external test inputs, but expected next labels,
expected totals, and historical winners must not enter production
dependencies or certificate premises. Neither the relative primitive nor the
eventual authoritative layer may claim the final instance census, a unique
live orbit, an empty final live set, discharge, CL, response-cone
completeness, or halt until their later certificate gates close.
