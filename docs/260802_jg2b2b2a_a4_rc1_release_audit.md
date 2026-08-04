# JG2b2b2a-A4-O Release-Candidate-1 Audit and Repair Frontier

Date: 2026-08-02

Status: **A4-O RC1 REJECTED; A4-R2a AND A4-R2b FROZEN; A4-R2c ACTIVE.** This audit
withdraws the RC1 freeze claim in
`docs/260802_jg2b2b2a_a4_ordinary_profile_closure.md`. The candidate made the
ordinary semantic boundary substantially sharper, but it did not satisfy the
P0 requirement that no encoding, source-census, Level-I/Level-II, failure, or
resource-accounting choice remain for implementation time. JG2b2b2b therefore
remains blocked. No token or factual authority was minted and no live history
was inspected.

## 1. What independently verified

The RC1 fixtures are internally reproducible:

- the embedded static candidate is exactly 1,209 bytes;
- its domain-separated digest is
  `blake3:c6ad7291770439b2d1a3055dca862f62680a507f28ce803761777fc823361300`;
- `Digest` canonical text is exactly 71 ASCII bytes;
- the synthetic manifest is exactly
  `1 + 2 + 1209 + 9*(8+71) = 1923` bytes; and
- its domain-separated digest is
  `blake3:8198531645e4e04840d11794a0c3d50f9f393707b9ccc4e8832223be605df113`.

The candidate also correctly separates a history-free definition, a
nonrecursive manifest identity, a future opaque verified-definition token,
and a still-later law-verified profile. Its A1 field-17 and manifest formulas
have no direct self-inclusion. These are retained design results, not evidence
that RC1 closed the profile.

## 2. Why RC1 cannot be frozen

### 2.1 The bytes bind labels, not the executable definitions

RC1 field 6 contains eight component version pairs and field 16 contains 44
rule ordinals. It contains no by-value typed A1/A2/A3 definition tree and no
profile-verifier source identity. A change to a decoder premise, carrier
layout, quotient field, evidence record, or action reconstruction could retain
all RC1 bytes. The claim that reminting detects every semantic V1 mutation is
therefore false.

The repair must bind both:

1. a by-value closed schema transcript for every new V1 record/sum and its
   exact field order; and
2. the reproducible protocol identity of the Rust implementation that
   interprets those schemas, while still retaining full parent evidence.

A digest remains identity, not proof. The future opaque token must own the
full manifest, parents, and independently reconstructed schema values.

### 2.2 The Level-I/Level-II relation omitted the checking signature

A2-O requires every Level-II value to retain one exact normalized signature.
RC1 quantified over `(kappa,H,o,c,r,E)` but did not select `Sigma`, and it
listed carrier/raw/built/action Level-I values while defining only the
raw-code-to-envelope Level-II case.

The accepted ordinary repair separates provenance from evaluation:

```text
b             = BirthEventOrdinal(o)
S^hist_b(H,o) = the independently replayed full normalized successor
                immediately after birth event b
Sigma(H,o)    = reverify the projection of S^hist_b that erases every body
                born before b and retains every declaration born at b.
```

`S^hist_b` contains every older declaration and the complete extension born at
`b`, and no later birth. It must agree in full canonical bytes, digest, and
declaration count with both the replayed event successor and
`H.reconstructed_stages[b].successor_boundary`. The current kernel unfolds a
global whose declaration retains a body, so `S^hist_b` is provenance only.
Every carrier, decoder, builder, normalizer, and action check for `(H,o)` uses
the separately verified opaque `Sigma(H,o)`; all IDs/types and current-birth
bodies agree with history, while strictly older bodies are absent.

The closed Level-II sum must separately cover checked carrier values, checked
raw decoding, checked built records, checked candidate envelopes, and checked
action images. A raw or carrier Level-I proposal is not itself authority.

### 2.3 The current/older source census was undefined and circular

`OccurrenceId14`, `ExportId`, and `PublicFieldPath` were names rather than
record definitions. Worse, an “older public interface” census cannot be an
input to the classifier that is supposed eventually to derive the first
complete interface census.

The accepted V1 repair derives both source classes only from opaque complete
history:

- `U_b` is the complete JG2b2b0 traversal of declarations born in event `b`;
- for every declaration occurrence, the strict birth-prefix signature is the
  independently verified prefix of the full historical birth successor ending immediately before
  that declaration's terminal ordinal, including earlier declarations in the
  same birth extension; and
- `Pub_<b(H)` contains exactly one opaque `DeclarationField` export for every
  declaration whose verified birth-event ordinal is strictly less than `b`.

An older source exposes only `Term::Global(global_id)` and the normalized
declared public type retained by the verified birth history and replayed under
`Sigma(H,o)`. It exposes no declaration body and does not rely on a prior
indexed-interface classification. The declared type is metadata/support
ownership, not a second raw syntax source.

The repair must now freeze all six rooted records at `0xea`--`0xef`, their six
object domains, the local-context digest domain, the two `SourceRefV1` tags,
the inherited JG2b2b0 path/node tags, scalar widths, and full source-entry bytes
before an ordinal order exists.

### 2.4 Dynamic codecs were only domain reservations

Roots `0xfb`--`0xff`, `0xe8`, and `0xe9` did not determine layouts for:

- current and older source entries;
- all seven raw, decoded, and built records;
- A1 fields 1--16 and exact rejection/evidence receipts;
- quotient keys and first-difference witnesses;
- outcomes, partitions, complete coverage, and the three dispositions; or
- action syntax, endpoint projections, traces, and images.

The repair must give a closed type-expression/schema table and exact
type-directed codec for each. “Displayed order” is sufficient only after the
display defines every nested field and scalar width. An endpoint in an action
trace must be a nonrecursive payload/key projection, never another action
image; the opaque action-image token must own the full base candidate and full
trace even when canonical bytes contain checked identities/commitments.

### 2.5 Failure and resource transcripts were not closed

RC1 encoded the seven false-family tables, top-level abort tags, pipeline and
parent tags, and a kernel-error table. It omitted the subordinate
`ProfileBoundary`, arithmetic, resource, codec-object, and path schemas and
their payload widths. It also used six generic budget categories for fourteen
resource counters.

The repair rules are:

- `BudgetExhausted` carries the exact `ResourceCounterV1` tag directly plus
  `used:u64` and `limit:u64`;
- duplicate current/older census identities have one dedicated abort receipt
  with source class and the two checked `u32` ordinals;
- immediately after successful decode and before build, the ordinary anchor
  stage reads the one fixed raw `OriginPathV1` leaf: `OlderPublic` and a
  `Current(q)` with `q != o` are the two disjoint anchor-false cases;
- failure after a valid `Current(o)` principal leaf to replay exactly that one
  lineage through normalization is an invariant abort, not a second empty or
  multiple-origin false case;
- kernel call roles are a closed enum; logical kernel errors may be false only
  at explicitly listed proposal-check roles and are abort at every remint,
  history, normalization, deterministic reconstruction, quotient, coverage,
  and action-rebuild role; and
- a checked-false receipt has an exact record containing the complete raw
  code, constructor/raw ordinal, closed stage/locus/reason, optional expected
  and actual checked judgments, optional kernel error, exact relevant input
  bytes, and deterministic resource deltas.

The current kernel exposes no exact usage receipt across multiple public
calls. A4 must retain its fixed units, but JG2b2b2b cannot begin until A4
specifies a metered aggregate API and remint consequences exactly.

### 2.6 Search/material accounting was implementation-dependent

RC1 invoked a codec AST and “material nodes” without defining either count.
The repair must freeze:

1. a count-only traversal over the same structural generator, with one event
   for the initial empty code, every encoded scalar/tag/vector-length/source
   decision attempted (including a rejected duplicate source), and every
   completed raw code; no semantic decoder may run during that traversal;
2. exact equality between count-only and emitting traversal counts;
3. a recursive material-size function over scalars, sums/options,
   records/tuples, vectors/sets, terms, judgments, and retained references,
   with repeated occurrences charged repeatedly; and
4. one exact mapping from every retained object and byte encoding to the
   fourteen counters.

Pure finite control work need not acquire an invented “CPU step” counter.
Decoder/build evidence contributes material nodes; logical checking consumes
the exact kernel meter; canonical output consumes the exact byte counter.
Quotient comparisons are bounded by admitted key bytes and class/member
counts. This distinction must replace RC1's claim that every informal step was
charged somewhere.

### 2.7 Two smaller authority defects

`P` was used both for the manifest digest and the opaque profile token. The
repair must use distinct types and names, for example
`ProfileManifestIdV1 M` and
`VerifiedOrdinaryDependentEnvelopeProfileDefinitionV1 K`. A1 field 2 is a
verified parent reference owned through `K`; its canonical projection contains
`M`, never a digest promoted to evidence.

A1 also requires a selector-totality subject. The exact subject has now been
added alongside the four stability subjects:

```text
Eval^kappa_c(H,o,r)=Match(E) ->
  exists! q in U_(Birth(o)).
    origin_(nf(E))(root_c)={q} and q=o and sel_(ord,c)(nf(E))=q.
```

Its proof remains assigned to JG2b2b3c.

## 3. Ordered repair gates

The active A4-O repair is split only for auditability; none of these subgates
mints authority:

1. **A4-R0 -- independent RC1 release audit -- DISCHARGED 2026-08-02.** This
   record verifies the candidate fixtures and rejects the freeze claim.
2. **A4-R1 -- signature and noncircular source-census types -- FROZEN
   2026-08-02.** Independent audit accepted `Sigma(H,o)`, the full current
   occurrence/older export records, paths, replay equations, codecs, and source order.
3. **A4-R2a -- acyclic schema/path/ownership foundation -- FROZEN
   2026-08-02.** Independent audit passed after four representation leaks were
   repaired.
4. **A4-R2b--R2d -- concrete typed schema table -- ACTIVE AT R2c.** R2b has
   frozen every Level-I/raw/decoded/built and early-Level-II record. Close the
   remaining envelope/evidence, quotient/outcome/coverage/action records and
   sums under the frozen R2a/R2b boundary.
5. **A4-R3 -- failure/resource machine -- BLOCKED BY R2d.** Close all reason
   payloads and widths, call-role mapping, count-only traversal, material-size
   recurrence, meter API, and counter mapping.
6. **A4-R2e -- complete registry/source integration join -- BLOCKED BY R3.**
   Assemble root `0xe0`, measure the isolated reference source, and audit every
   codec and ownership edge.
7. **A4-R4 -- regenerated fixtures and mutations -- BLOCKED BY R2e.** Rebuild
   the full static and synthetic-manifest bytes independently and require
   field-, tag-, order-, width-, root-, domain-, resource-, and ownership-
   sensitive rejection fixtures.
8. **JG2b2b2b -- executable ordinary Rust representation -- BLOCKED BY A4-R4.**

The cubical A2-C lane remains open in parallel. JG2b2b3a--c, the factual
JG2b2c census, and every GCap/selective phase remain blocked. RC1's fixture
digests may be retained as rejected regression inputs but may never identify a
verified profile.
