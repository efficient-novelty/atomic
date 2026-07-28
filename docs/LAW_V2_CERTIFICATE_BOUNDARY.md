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
`src/certificate.rs`, `src/dependency_graph.rs`, `src/digest.rs`, the
self-contained `crates/pen-kernel/Cargo.toml`, the reviewed
`crates/pen-kernel/production-dependency-graph.lock`, `rust-toolchain.toml`,
and `.cargo/config.toml`. The normalizer digest covers `src/lib.rs`,
`src/syntax.rs`, `src/checker.rs`, and `src/dependency_graph.rs` plus those
same build/configuration and dependency-graph inputs.

The minimized dependency graph is not copied blindly from the aggregate
lockfile. The kernel parser selects only normal and build dependencies from
its own manifest, resolves their complete transitive closure in the active
`Cargo.lock`, and canonicalizes every root, package name/version/source,
registry checksum, and dependency edge. That canonical form must byte-match
the reviewed snapshot before either protocol digest can be constructed.
Dev-only dependencies are not roots. Registry packages without checksums and
source-less transitive production packages fail closed. The ordinary and
physical-isolation workspaces may therefore have different membership and
unrelated lock records while producing the same kernel identity when—and only
when—the kernel dependency closure is identical.

This is exact source/protocol and resolved dependency-graph identity for the
enumerated inputs, not binary, compiler, operating-system, installed package
bytes, or environment attestation; callers cannot supply alternate values for
those two fields.
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
- a universal total-specialization theorem for the full registered
  future-hole scheme calculus;
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
exit gate, because full CL and authoritative finite-fragment equivalence
verification are absent.

## 9. Open source and theorem tensions

### 9.1 Recovered legacy appendix

The normative appendix says that it preserves the previous law statement in
`docs/app_a_two_laws_formal_axioms_old.tex`. That exact source is present and
is independently digest-bound by the partial legacy freeze manifest and its
replay checker. This provenance repair does not expand the kernel's trusted
certificate fragment or satisfy any theorem gate.

### 9.2 Stage-4 quotient cardinality

The repaired normative appendix distinguishes four archived normalized
representative slots from the classes of the full adopted quotient. Once
individual representative certificates and the order-axis obstruction replay,
the conditional bound is `2 <= |Q4| <= 4`. The present repository has not
issued those Law V2 certificates. The Pi/Sigma former-axis equivalence remains
open, so an executable run unable to settle the required quotient must return
`Unknown(UnknownQuotient)` rather than an exact cardinality.

The kernel must therefore not contain an expected cone cardinality, and its
definitional-equivalence handle must not be used as evidence that the open
univalent equivalence exists or fails to exist.

`pen-law` now exposes versioned `UncheckedQuotientStatus`,
`UncheckedFiniteFragmentOutcome`, and `UncheckedBootstrapStatus` wire records.
They distinguish unresolved from claimed-complete quotients, proof/refutation
claims from fragment escape and resource exhaustion, and registered Law V2A
from claimed-derived Law V2B. Constructing or deserializing them grants no
verified capability; `pen-engine` maps every current variant to `Unknown`.

## 10. Implemented relative census and remaining Phase-3 boundary

`pen-demand` now provides a deterministic finite closure/replay primitive. It
distinguishes caller-supplied families from caller-supplied instances,
derives structural support against an opaque two-global window, saturates an
explicit finite rule relation from explicit library seeds, and returns either
`CompleteRelative` or `Unknown`. Its certificate verifier recomputes that
calculation and returns a private `VerifiedRelativeCensus` handle.

`pen-demand` also implements three native intrinsic scheme constructors:
type formation, typed term use, and definitional computation closure.
Registration rechecks the dependent context and open judgment. Each complete,
ordered closed assignment is replayed through the kernel's resource-bounded
specializer and yields a private `VerifiedClosedSpecialization` capability.
Malformed work is `Unknown(Unsupported)` and budget failure is
`Unknown(ResourceExhausted)`; neither is refutation evidence.

This remains short of the Phase-3 exit gate. The intrinsic slice proves
particular native registrations and particular complete assignments, not the
universal total-specialization theorem or exhaustive generation for the full
GF2 scheme calculus. Family membership, instance membership, the relative
rule relation, and library seeds are still caller inputs. The census
certificate binds the verified signature and finite inputs, but it does not
yet bind a verified history, derivation basis, law, grammar, complete scheme
calculus, or event-based active window, and it provides no weakening or
expiration proof.

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

## 11. GF2, checker readiness, and registered bootstrap boundaries

`pen-gf2` now validates a versioned, canonically ordered manifest containing
the required finite feature families and hard resource limits. Its
`LawDecision` distinguishes `Proven`, `Refuted`, `OutsideFragment`, and
`ResourceExhausted`. The native adapter can mint a positive handle only by
replaying a supported `pen-kernel` judgment. It has no native refutation
constructor, and checker rejection is therefore never promoted to
`Refuted`. Listed but unsupported sums, path, cubical, and other artifacts
fail closed.

The resulting `VerifiedFiniteFragment` capability proves structural manifest
validation and native backend binding. It does not prove that an inventory is
semantically sound, exhaustive, or closed.

`pen-gf2-agda` separately probes one exact external checker environment:
Agda 2.8.0, `cubical-0.9` commit
`b150186d2544e7efeddd31e5d14a8b9ecbb100f7`, its reviewed canonical source
tree, exact library flags, and a fixed postulate-free smoke module under
explicit safe/cubical options. Before invoking either tool, it requires the
observed Agda and Git
bytes to match independently supplied trusted digest pins. The evidence binds
both configured and observed executable identities, the reviewed Cubical
canonical source manifest, the independently reviewed Agda primitive-runtime
manifest supplied by trusted configuration for that exact distribution,
source, checkout, command, output, and manifest digest. Text members must be
UTF-8; CRLF is canonicalized to LF and a bare carriage return is rejected.
Primitive `.agdai` members are bound and copied as raw bytes. Those canonical
members are materialized in a private scratch snapshot; Agda checks only that
snapshot with Cubical interfaces ignored and the snapshot selected as its data
directory. The repository fixes the reviewed Cubical canonical tree digest.
The exported `AGDA_REFERENCE_PRIMITIVE_TREE_DIGEST` constant records only the
local integration fixture; it is a reproducibility aid and not a production
authenticity anchor. Git runs with system/global configuration suppressed,
paging and filesystem monitors disabled, and hooks disabled. It is used only
for fixed, non-content `rev-parse` probes; no `status` or attribute/filter
path is executed. Canonical source manifests before and after the check, not
Git cleanliness output, bind the checked source state.
Every child runs in a process group or Windows Job that is terminated even
after nominal leader success, and scratch cleanup must succeed before the
private readiness capability is returned. Self-pinning the executable bytes
being inspected proves reproducibility only, not tool authenticity. The probe
accepts no caller-supplied source and verifies no GF2 theorem.

Finally, `pen-law` embeds a strict anonymous Law-V2A three-act registration.
The verifier checks the exact universe/type/witness source, every normalized
one-declaration extension, predecessor and boundary chain, kernel and
normalizer identity, and the final artifact digest. This proves only that the
disclosed initial condition is exact and well typed in the native fragment.
It proves neither free-sealing initiality nor Law-V2B derivation, leastness, or
uniqueness.
