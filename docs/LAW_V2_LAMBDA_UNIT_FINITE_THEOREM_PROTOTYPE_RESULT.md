# Law V2 Lambda/Unit Finite-Theorem Prototype Result

**Status:** `PROPOSED_PROTOTYPE_RESULT_NOT_ADOPTED`

**Date:** 2026-07-29

**Authority:** none; isolated generic prototype evidence only

## Result

The isolated `pen-semantic-audit` workspace now contains projection-free
successor proposals for:

```text
gf2-semantic-audit-lambda-unit-v1
gf2-kernel-cost-lambda-unit-v2
```

The broader semantic and cost proposals are unchanged. The successor profiles
admit only the finite dependent lambda/unit surface and remove
`DescriptorForcedProjection` from their respective Q0 and cost-free
manifests. A projection-bearing input returns
`OutsideFragment(DescriptorProjection)`; other record syntax remains
`OutsideFragment(UnsupportedTerm)`.

The generic prototype also implements verifier-minted ambient-export,
ordinary-beta, and pre-Q0 raw-carrier capabilities. It then reaches a new
definition-and-theorem blocker in the proposed finite substitution carrier.
The public finite-rewrite entry point remains deliberately fail-closed at:

```text
Unknown(MissingRewriteAdmissibilityTheorem)
```

No rewrite-system capability produced by the incomplete diagnostic machinery
is released as Q0 authority. No Q0 quotient, cost basis, weakening theorem,
marginal set, SR2 injection, live adapter, or Act value is issued.

## Implemented generic capability surface

### Projection-free successor manifests

The two successor manifest constructors and exact verifiers are versioned
separately from the broader proposals. Both remain
`GenericPrototypeOnly`, unfrozen, unadopted, and unable to access Profile A.

The supported term grammar is exactly:

```text
Sort(0)
Sort(1)
UnitType
Unit
Variable(index)
Global(id)
Pi(parameter_type, body_type)
Lambda(parameter_type, body)
Apply(function, argument)
```

The inventory compiler, normalizer boundary, carrier, ordinary-beta verifier,
cost entry point, and finite-rewrite boundary all apply the successor
fragment taxonomy recursively. In particular, a projection hidden in a type,
body, equation, demand, raw clause, or verified inventory object cannot be
treated as an empty projection census.

### Ambient public-export census

`VerifiedAmbientPrimitiveExportCensusV1` and its private class/member
capabilities replay complete normalized declaration bodies over both
boundaries. They distinguish:

- a first successor public-export class;
- a predecessor-public re-export class;
- an ambient primitive occurring only in a type or proper subterm;
- direct and transitive transparent aliases; and
- simultaneous equivalent successor exports.

The census is class-valued and representative-free. Simultaneous first
exports contribute one ambient class independently of declaration order,
serialization order, or digest ordering. A digest identifying an ambient
class is not cast to a public `ClauseIdV1`.

### Ordinary beta

`VerifiedOrdinaryBetaDerivationV1` is reconstructed from the verified
inventory rather than from a caller label. It binds:

- the bodyful definition and separately represented equation;
- the definition telescope and complete argument spine;
- the oldest-first sequential substitution;
- the unfolded body, substituted result, and result type;
- exact delta, beta, and congruence steps;
- declaration, equation, inventory, and kernel-protocol identities.

Every step is kernel-replayed. The capability is used only when a separately
represented public equation is claimed to be forced by the definition.
Implicit beta behavior with no separately sealed equation remains structural
and creates no second raw cost clause.

### Pre-Q0 raw carrier

`PreQ0RawCarrierCertificateV1` and
`VerifiedPreQ0SemanticSeedV1` have private fields and no `Deserialize`
implementation. A caller's `claimed_normalized` seed judgment is replaced by
the source judgment before the pre-Q0 identity is minted. The carrier then
exhausts rank-0, rank-1, and rank-2 syntactic schema construction without Q0
normalization or Q0-based deduplication.

Every enumerated application and equation-action tuple receives an
`Applicable` or typed `CertifiedInapplicable` disposition, or aborts the
complete construction with `OutsideFragment` or `Unknown`. A kernel
`ExpectedType`, `ExpectedFunction`, `ExpectedPair`, or `TypeMismatch` caused
by placing a family in an equation hole is a certified typed
inapplicability. Resource exhaustion, an unbound variable, or an unknown
global still aborts; those failures are not negative evidence.

The finite-rewrite boundary additionally verifies exact term-subject seed
coverage: one head seed for every inventoried declaration and one equation
seed for every inventoried equation, with matching identities, origins,
source identities, and source judgments. This is term-subject coverage only;
it does not manufacture semantic support, clause, or demand-anchor authority.

### Kernel-cost V2 integration

The public lambda/unit V2 cost entry point requires an exact
`VerifiedRewriteSystemV1` bound to the same semantic manifest, inventory,
successor signature, and normalizer protocol. It also consumes the ambient
census and exact ordinary-beta capabilities. Because the rewrite verifier
currently fails closed, no caller can obtain the capability needed to mint a
public lambda/unit cost certificate. The component integration below is
retained only as a private generic test diagnostic.

Ambient export classes occupy a disjoint
quotient-root namespace from ordinary public clauses. First exports,
predecessor re-exports, peer reconstructions, duplicate classes, negative
evidence, minimal bases, and root-signature uniqueness are recomputed from
the verified inventory.

The profile boundary rejects `Sigma` and `Pair` as
`OutsideFragment(UnsupportedTerm)` and rejects `First` and `Second` as
`OutsideFragment(DescriptorProjection)`, recursively across all raw and
verified terms. Thus excluded record syntax cannot mint a lambda/unit cost
certificate merely because the forced-projection ledger is empty.

## Non-authorizing finite-graph diagnostics

The prototype contains private, non-deserializable data types and diagnostic
algorithms for:

- typed whole-term nodes;
- candidate beta, public-delta, unit, and fresh-equation rules;
- one disposition for every enumerated node/position/rule tuple;
- kernel-checked source and target typing;
- finite graph acyclicity and decreasing topological ranks;
- unique reachable normal forms;
- reachability paths for the substitutions actually enumerated; and
- predecessor-edge and overlap diagnostics.

These routines are retained to expose the remaining obligations and support
adversarial tests. They are not a completed
`VerifiedAuditTermUniverseV1` or `VerifiedRewriteSystemV1` theorem.

## Exact new blocker set

### 1. The proposed substitution closure is not finite as stated

The manifest supplies embedding-lift and forced-newest-argument generators
and requires closure under composition. On the ordinary bodyless-head/fresh-
equation generic vector, only fourteen initial typed whole terms and three
contexts suffice to generate nontrivial context endomorphisms. Repeated
composition grows term images and exhausts the frozen 262,144-disposition
bound before the initial substitution census closes.

Truncating at that bound would make missing substitutions look
inapplicable, so the verifier does not truncate or mint authority. The next
adjudication must freeze a genuinely finite substitution carrier or a
canonical finite quotient and prove that its composition normal forms are
closed. Merely raising the resource bound is not such a proof.

### 2. Binder-local and subterm closure is incomplete

The diagnostic universe currently starts from whole inventory, demand, seed,
and raw-family judgments. It visits redex positions inside those terms, but
does not independently add every recursively typed subterm under its
binder-extended local context.

Consequently, a fresh rule in a term such as

```text
lambda y : UnitType . fresh y constructor
```

requires the local substitution `x |-> y`. Shifting an instance prepared only
for the outer context is not an exhaustive substitute for constructing that
binder-local instance. A completed theorem needs kernel-replayed subterm
typing, every binder-local context, and rule/substitution enumeration in
those contexts.

### 3. Predecessor graph reconstruction is not independent

Filtering the successor-generated universe to terms containing only
predecessor globals does not independently reconstruct the predecessor term
universe. Successor-only demands, equations, or raw derivations can contain
only old globals and thereby enlarge both sides of such a comparison.

The theorem must construct `G_pred` from predecessor-only inventory, semantic
seeds, raw carrier, contexts, substitutions, and admitted historical rewrite
rules, then compare both its node set and edge set with the exact successor
restriction.

The present generic inventory records predecessor equations as typed history,
but it does not prove their orientation or historical Q0 admissibility.
Until a private `VerifiedHistoricalRewriteInventoryV1`-style capability
provides that authority, a nonempty predecessor equation census also stops at
`Unknown(MissingRewriteAdmissibilityTheorem)`.

### 4. The explicit overlap report is incomplete

Unique reachable normal forms prove joinability for every outgoing edge that
is actually present in the enumerated graph. The diagnostic overlap report,
however, currently records only distinct rules firing at the exact same
position.

The requested report must bind every unordered pair of immediate edges from
each source and classify same-position, nested, and disjoint redex pairs,
together with an actual join/common-normal-form path or a certified
non-overlap disposition.

### 5. Full semantic seed authority is not derivable from inventory alone

Exact declaration/equation term-subject coverage does not derive:

- semantic `ClauseIdV1` identities from the cost wire;
- `DemandOrbitIdV1` or proof that an inventoried `DemandFamilyIdV1`/port
  association realizes a semantic demand anchor; or
- historical predecessor ambient presentations and support.

The downstream carrier/quotient theorem therefore still needs independently
verified public-clause and demand-anchor censuses, or a complete prior
`VerifiedSemanticSeedCensusV1`-style authority. Caller-supplied
`public_support`, `source_clause`, and `demand_anchor` fields cannot supply
that authority.

## Downstream gates

Until all five blockers above are resolved, no capability may authorize Q0.
Therefore Q1/Q2/Q3 quotient completion, the complete V2 cost-basis theorem,
weakening/restriction, exact marginals, SR2, and transcript-level Rust/safe-
Agda agreement remain downstream.

Independent transcript agreement must compute from common canonical input;
the existing pinned Agda typecheck is not semantic-output agreement.
Adoption and manifest freeze require a later human-reviewed packet. A
separately isolated Profile A adapter remains last and may be created only
after those generic gates are complete, adopted, and frozen.

## Validation snapshot

The isolated semantic-audit workspace passed:

```text
cargo test --locked --all-targets --no-fail-fast
```

Result: 107 tests passed, none failed, and the independently pinned live-Agda
test was ignored by default. The example target contained no tests.

The following checks also passed:

```text
cargo fmt --all -- --check
cargo clippy --locked --lib --tests -- -D warnings
cargo check --locked --lib
git diff --check
python scripts/check_semantic_audit_isolation.py --json
python -m unittest scripts.test_check_semantic_audit_isolation
python scripts/check_law_v2_firewall.py --isolation-build --json
```

The firewall reported `status: pass`, no violations, and a successful isolated
production-root build. The 15 firewall/window-audit checker tests passed. The
registered Window Audit replay remained:

```text
status: valid
outcome: UndefinedAudit
residual_gap_count: 20
profile_a_termination: HaltedDebtFree
```

These tests are prototype regression evidence only. They do not adopt either
successor manifest, prove Rust/Agda transcript agreement, or authorize a live
history calculation.

## Preserved results

This work does not modify or supersede:

- the issued Profile A H3 result;
- the issued H4 continuation and debt-free halt result;
- the Profile A registry;
- `LAW_V2_WINDOW_REGISTER_AUDIT_V1`;
- the historical V1 kernel-cost adjudication; or
- the broader V2 public-inventory prototype result.

The Window Audit V1 outcome remains `UndefinedAudit`. No provisional
\(\kappa_i\), \(\nu_i\), ratio, bootstrap density, threshold, or
Selective-Law result is inserted.
