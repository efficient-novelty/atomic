# Law V2 Kernel-Cost V2 Public-Inventory Prototype Result

**Status:** `PROPOSED_PROTOTYPE_RESULT_NOT_ADOPTED`

**Date:** 2026-07-29

**Authority:** none; isolated generic prototype evidence only

## Result

The isolated `pen-semantic-audit` workspace now contains the next generic
prototype layer for the proposed `gf2-kernel-cost-core-v2` profile. V2
separates two questions that the V1 proposal had coupled:

\[
\begin{array}{ll}
\text{rewrite admissibility} &
  \text{whether a sealed equation may safely compute in Q0},\\
\text{kernel cost} &
  \text{whether that public equation was already derivable for free}.
\end{array}
\]

A separately sealed computation equation owned by a bodyless fresh head is
therefore a separate paid public clause unless the verified inventory proves
that the exact normalized equation was predecessor-public or proves a complete
duplicate presentation. Compiler generation, role metadata, and a
pre-existing demand do not make it free.

This result proposes and exercises that rule. It does not adopt or freeze the
profile, construct a Profile A adapter, or issue any Acts 1--4 value.

## Implemented generic capability surface

### Cost V2

The proposed `gf2-kernel-cost-core-v2` manifest contains exactly four
cost-free completion rules:

1. ordinary beta of a bodyful definition;
2. a prior-public transparent alias or exact equation replay;
3. a descriptor-forced projection; and
4. duplicate-presentation deletion.

`CertifiedFreshConstructorComputation` is absent. The V2 cost audit binds its
successful public path to a private verifier-minted public inventory. Raw
caller clause slices, caller availability labels, and caller-supplied
realized-closure sets do not confer public V2 authority.

The audit treats a successor-new bodyless head and its successor-new sealed
equation as independent generic-vector basis clauses when neither is
predecessor-public nor a proved duplicate. This is prototype evidence about
that fixed generic test vector only, not a value for any registered act.

The four-rule manifest is a declared grammar, not a claim that every public
rule path is already implemented. Inventory-backed ordinary beta currently
fails at `MissingOrdinaryBetaDerivation`, ambient-shaped declarations fail at
`MissingAmbientFirstExportInventory`, and descriptor/projection clauses fail
at `MissingDescriptorProjectionInventory`. Those outcomes prevent the current
public subset from silently charging a clause when applicability of a free
rule has not been proved.

### Replay-minted public inventory

`VerifiedPublicAuditInventoryV1` has private fields and no `Deserialize`
implementation. Its verifier replays:

- every cumulative predecessor boundary from the empty signature;
- the exact successor append;
- event, group, declaration, equation, projection, and demand censuses;
- source-to-normal declaration and judgment pairs;
- strict-prior exact demand `PortKey`s;
- verifier-derived availability and the canonical dependency DAG; and
- a typed origin-cutoff Q3 registry whose supplied entry set is positively
  checked empty.

The capability retains both source and normalized demand/equation judgments,
binds kernel-normalizer identity, and incorporates complete clause contents,
group membership, ports, availability, dependency data, projection census,
and counts into its coverage identity.

An equation's optional `PortKey` proves only that the exact key names one
strict-prior inventoried contract. It does not prove that the successor
equation realizes or discharges that schematic demand.

The forced-projection capability proves only exhaustive ledger census,
declaration typing, record-owner precedence, field-slot uniqueness, and
origin. It deliberately contains no reduction payload or rewrite theorem.

### Q0-bound finite replay

The carrier and quotient paths can be bound to the same restricted fresh
computation program. Every fixed quotient vertex is replayed through that Q0
program; an absent or mismatched program fails closed. Family specialization
resolves one quotient class and asks the kernel to replay the complete
dependent substitution, so distinct closed instances retain one originating
family identity.

`VerifiedTypedRewriteInventoryV1` composes the public inventory with the
restricted fresh program. It requires exact one-to-one correspondence between
successor-new sealed equations and reconstructed typed fresh rules, rejecting
omissions, extras, duplicates, wrong owners, wrong source identities, wrong
or non-prior port associations, and digest mismatches. An optional verified
`PortKey` is retained and digest-bound but is not used as rewrite authority or
as a proof of demand realization. The compiler also binds an exhaustive
forced-projection census. Because the current generic Q0 fragment has no
projection-reduction theorem, only an empty projection census can pass this
intermediate compiler.

This capability is an exact typed inventory, not a rewrite-admissibility
theorem.

## Adversarial repairs

An adversarial review of the first implementation found and repaired several
evidence-boundary errors:

- public V2 cost success could previously be reached from a raw deserializable
  clause vector, including an omitted vector;
- transparent-alias availability could be asserted by the caller;
- equation and declaration duplicate witnesses ignored relevant ownership,
  provenance, descriptor, demand, or dependency data;
- forced-projection classification relied on clause shape rather than a
  complete verified census;
- a caller-provided realized set could overstate reconstruction closure;
- a V2-only duplicate-equation witness had leaked into the V1 proof enum; and
- declaration presentation, full demand-family identity, and the Q2 witness
  census were not completely inventory-bound.

The successor implementation keeps the raw engine internal to tests, derives
public availability and predecessor replay from the verified inventory,
recomputes exact closure, derives declaration presentation, binds the complete
equation-to-`PortKey` map, uses V2-specific duplicate evidence, requires an
exact one-to-one Q2 witness census, and fails closed when beta, ambient,
projection, or rewrite authority is absent.

## Authority boundary

The inventory verifier proves internal completeness only relative to the
supplied generic ledger. It has no access to the issued H3/H4 artifact store
and cannot prove that a caller supplied the unique Profile A history or the
authoritative Q3 registry. Removing a fact consistently from an invented
ledger is outside the generic verifier's jurisdiction.

That is intentional isolation, not Profile A evidence. A future separately
locked `pen-semantic-audit-profile-a-adapter` must independently replay the
issued artifacts, pin the exact history and registry identities, and then
construct this generic capability. No such adapter is created here because
the generic semantic and cost profiles are not frozen.

## Exact stop condition

The cost layer first needs verifier-minted ordinary-beta reduction provenance,
a verified ambient first-export census, and exhaustive descriptor/projection
reduction authority to implement all four declared free rules. The current
public subset returns the corresponding typed `Unknown` outcomes instead of
issuing an incomplete cost certificate.

The next semantic gate is a real, inventory-bound
`VerifiedRewriteSystemV1` theorem. The existing restricted program
reconstructs and kernel-types the finite fresh rules and checks their local
shape, but those checks do not prove:

- substitution stability for the complete rewrite relation;
- termination and confluence for the combined beta/delta/projection/fresh
  system;
- all critical-pair and overlap obligations;
- conservativity over predecessor terms; or
- transcript-level agreement with an independently computing safe-Agda
  reference.

Accordingly, the public attempt stops at
`Unknown(MissingRewriteAdmissibilityTheorem)`. A nonempty forced-projection
census stops earlier because the corresponding reduction theorem is also
absent. Boolean certificate fields, structural tags, digest labels, and more
examples cannot replace these theorems.

A separate demand-specialization/compiler theorem is also required before a
schematic predecessor demand and a successor equation sharing a `PortKey` can
be treated as a verified realization. The inventory intentionally does not
infer that theorem from the key association.

The ordered carrier-completeness, cost-basis, structural weakening/restriction,
marginal, SR2, and Rust--Agda transcript gates are downstream of this stop and
receive no new authority from this prototype.

## Validation snapshot

The isolated semantic-audit workspace passed:

```text
cargo test --manifest-path crates/pen-semantic-audit/Cargo.toml \
  --all-targets --locked --no-fail-fast
```

Result: 70 tests passed, none failed, and the independently pinned live-Agda
unit test was ignored by default.

Formatting and all-target clippy with warnings denied passed. The ignored
live gate was then run through the explicit pinned example and returned:

```text
{"status":"valid","agda_reference_digest":"blake3:33f78fa03eac42a22950a209fb8669ccdeec325c799473b01606115bab3f4c32"}
```

This remains fixed-reference typechecking, not transcript-level Rust--Agda
semantic agreement.

The semantic and contextual nested-workspace isolation checkers passed, as
did both two-test checker suites. The contextual workspace separately passed
12 Rust tests, formatting, and all-target clippy. The root Law-V2 firewall
passed its isolated lawful-closure build and tests; the 15 Law-V2 Python
checker tests passed; and the quarantined oracle suite passed all 7 tests.

The registered Window Audit V1 replay remained:

```text
status: valid
outcome: UndefinedAudit
residual_gap_count: 20
profile_a_termination: HaltedDebtFree
```

The long legacy command

```text
cargo test --locked --workspace --all-targets --no-fail-fast
```

was run separately with a 15-minute timeout. It progressed through the slow
legacy executables and was actively computing in `pen_search` when the command
timed out after 904 seconds. It emitted no test failure or compiler error.
Under the test-operations rule, that runtime timeout is recorded separately
and is not treated as a semantic-audit theorem blocker.

`git diff --check` passed. The root manifests and lockfiles, the original
formal appendix, and the issued H3/H4, Profile Registry, and Window Audit V1
artifacts were unchanged from preserved commit `0919319`.

## Preserved results

This work does not modify or supersede:

- the issued Profile A H3 result;
- the issued Profile A H4 continuation and debt-free halt result;
- the Profile A registry;
- `LAW_V2_WINDOW_REGISTER_AUDIT_V1`; or
- the historical kernel-cost V1 adjudication and prototype result.

The Window Audit V1 outcome remains `UndefinedAudit`. No provisional
\(\kappa_i\), \(\nu_i\), ratio, bootstrap density, threshold, or bar result is
inserted.
