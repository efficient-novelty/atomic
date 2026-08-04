# Phase JG1 — Pre-Exposure Generative-Capability Grammar

**Status:** COMPLETE on 2026-08-01  
**Authority:** `VerifiedPreExposureGenerativeCapabilityGrammarV1`  
**Implementation:** `crates/pen-generative-audit`  
**Next phase:** JG2b2b — executable occurrence/indexed-interface calculus and
generic substitution/naturality metatheory

## 1. Jurisdiction

JG1 freezes the target-neutral grammar used by the independent generative-
capacity audit. It does not modify the semantic-family carrier, the frozen
four-role semantic SR2 grammar, or the exact negative SR2 result.

The verifier accepts only a typed manifest. It has no history, registered
Genesis prefix, future candidate, held-out label, diagnostic channel, or
oracle input.

## 2. Frozen capability roles

The vocabulary is the following closed seven-member enum, in canonical order:

1. `Formation`;
2. `Abstraction`;
3. `Aggregation`;
4. `Transport`;
5. `Comparison`;
6. `Compiler`; and
7. `DischargeTransformer`.

There is no string-valued or extensible `Other` role. Every later capability
must receive exactly one canonical role; JG1 does not yet certify any
capability member or role assignment.

### 2.1 Scope clarification recorded during JG2

The initial phase wording called JG1 a “capability grammar,” but the V1 token
freezes roles, obligations, and receipt-source forms—not an executable finite
constructor/interface calculus. A `VerifiedSignature` cannot infer
`Compiler` or `DischargeTransformer` from those role names. Treating the role
enum itself as a constructor calculus would permit caller role tagging and
would not establish finite carrier completeness.

JG1 V1 remains frozen and valid within its stated authority. The missing
constructor premise is supplied by the separately versioned, JG1-bound JG2a
authority rather than silently changing the pinned JG1 transcript.

## 3. Frozen admission obligations

Every later member of the native GCap carrier must be proved:

1. stage-generic;
2. natural under every admissible substitution in its declared context;
3. represented by a finite public typed interface;
4. checkable from the present sealed grammar;
5. absent from the predecessor capability closure;
6. present after sealing;
7. supported by a newly paid clause or a previously live generative output;
   and
8. independent of later candidates and held-out evidence.

These are admission obligations, not self-attested flags. JG2 must derive
members from verified public structure; caller-supplied capability lists are
outside the authority boundary.

## 4. Frozen receipt-source grammar

JG7 must eventually construct an injection into exactly

```text
(NewPaidClause × GenerativeCapabilityRole)
  ⊔ PriorLiveGenerativeRequirementOutput.
```

The prior-output summand is not multiplied by roles. Consequently the future
counting shape is

```text
|GCapMarg| ≤ 7 |K| + |GenReq|.
```

Only a separate exact cost theorem can replace `|K|` by `kappa`. JG1 freezes
this source grammar but issues no receipt and proves no injection.

## 5. Canonical authority

The implementation pins:

- schema version `1`;
- exact enum membership and order;
- exact canonical discriminants and byte transcript;
- the pinned domain-separated digest
  `blake3:eee311f5f8c6f73d41e8b2d3dca0bc4bcc9c2e736c63a730073cc75acc1e0192`;
  and
- an opaque verified token that callers cannot construct directly.

Mutation tests reject a changed version, reordered role, missing admission
obligation, and malformed receipt-source product. Separate tests establish the
unique/exhaustive role and obligation inventories, the unroled prior-output
summand, and deterministic bytes/digest. The isolated test command is also
part of the Law-V2 firewall workflow.

## 6. Explicit non-authorities

JG1 issues no:

- native GCap member or carrier completeness theorem;
- capability quotient or equivalence;
- predecessor weakening or conservativity map;
- exact marginal capability set;
- strict-enlargement proof;
- provenance injection;
- `gamma` value or inequality involving `nu`;
- bootstrap `(kappa,gamma)` pair;
- debt, candidate, profile, or selection verdict; or
- Rust/Agda correspondence claim.

Therefore S+ and S↑ remain without live authority. S0 is logically independent
of `gamma`, although the common six-profile experiment remains frozen until
the shared JG substrate and experiment manifest are complete.

## 7. JG2 status and remaining entry condition

JG2a, JG2b1a, JG2b1b0, JG2b1b1, JG2b1b2, and protocol-only JG2b2a are
discharged in their `docs/260801_jg2*.md` phase records. Together they freeze
the constructor/stage vocabulary, relative chain and birth indices,
target-neutral sealed-log lifecycle, process-local consuming producer,
complete-through-producer-finalized-head authority, and the repaired
substitution/naturality entry protocol. None mints an occurrence,
substitution, or naturality fact.

Full JG2 remains open at active JG2b2b. It must freeze the executable
structural occurrence and indexed-interface grammars plus the generic theorem
over arbitrary typed substitutions. JG2b2c must then derive the exact finite
occurrence-by-constructor disposition matrix and concrete theorem-application
census; JG2b3 must derive closure and typed support; and JG2c must exhaust the
finite raw carrier, which may validly be empty. Every remaining sub-gate must
fail closed whenever any JG1 admission obligation cannot be certified. They
must not consume semantic novelty, historical Genesis identities, caller-
supplied capability inventories, caller obligation flags, or later-run
outcomes.

The four generic `(nu,gamma)` quadrant fixtures remain pre-registered
falsifiers. They cannot run until the independent semantic and generative
carrier/quotient/marginal/provenance chains can issue both coordinates.
